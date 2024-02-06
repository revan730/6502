use std::fmt;

use crate::{
    error::DecodeError,
    flags_register::{FlagPosition, FlagsRegister},
    instruction::{AddressingType, Instruction},
    memory_bus::{MemoryBus, MEM_SPACE_END},
    opcode_decoders::{ArgumentType, INSTRUCTIONS_ADDRESSING},
};

pub struct Cpu {
    pub address_space: MemoryBus, // TODO: replace with memory bus implementation
    pub a: u8,                    // Accumulator register
    pub x: u8,                    // X index register
    pub y: u8,                    // Y index register
    pub pc: u16,                  // Program counter
    pub s: u8,                    // Stack pointer
    pub p: FlagsRegister,         // Flags register
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Registers:").unwrap();

        writeln!(f, "A: {:#X}", self.a).unwrap();
        writeln!(f, "X: {:#X}", self.x).unwrap();
        writeln!(f, "Y: {:#X}", self.y).unwrap();
        writeln!(f, "PC: {:#X}", self.pc).unwrap();
        writeln!(f, "S: {:#X} P: {:#X}", self.s, Into::<u8>::into(&self.p))
    }
}

#[derive(Debug)]
enum Argument {
    Void,
    Byte(u8),
    Addr(u16),
}

enum AslLsrOperand {
    A,
    Value(u8),
}

enum IncDecOperand {
    X,
    Y,
    Value(u8),
}

enum LdOperand {
    A,
    X,
    Y,
}

impl TryInto<u8> for Argument {
    type Error = DecodeError;

    fn try_into(self) -> Result<u8, Self::Error> {
        match self {
            Argument::Byte(byte) => Ok(byte),
            _ => Err(DecodeError::ByteExpectedArgument),
        }
    }
}

impl TryInto<u16> for Argument {
    type Error = DecodeError;

    fn try_into(self) -> Result<u16, Self::Error> {
        match self {
            Argument::Addr(addr) => Ok(addr),
            _ => Err(DecodeError::AddrExpectedArgument),
        }
    }
}

#[derive(Debug)]
struct DecodedInstruction {
    pub int: Instruction,
    pub arg: Argument,
}

fn dword_from_nibbles(low_byte: u8, high_byte: u8) -> u16 {
    u16::from(high_byte) << 8 | u16::from(low_byte)
}

struct FetchOperandResult(u8, Option<u16>);

impl Cpu {
    pub fn new(mem_bus: MemoryBus) -> Cpu {
        Cpu {
            address_space: mem_bus,
            a: 1,
            x: 0,
            y: 0,
            pc: 0x200, // TODO: Probably should point to reset vector
            s: 0,
            p: FlagsRegister::default(),
        }
    }

    pub fn step(&mut self) {
        let opcode = self.fetch(self.pc);
        let instruction = self.decode(opcode);

        self.execute(instruction);
    }

    fn fetch(&self, address: u16) -> u8 {
        const SPACE_END: u16 = MEM_SPACE_END as u16;
        match address {
            0..=SPACE_END => self.address_space.read_byte(address as usize),
            _ => panic!("PC address out of bounds"),
        }
    }

    fn fetch_dword(&self, address: u16) -> u16 {
        let low_byte = self.fetch(address);
        let high_byte = self.fetch(address + 1);

        dword_from_nibbles(low_byte, high_byte)
    }

    fn decode(&self, value: u8) -> DecodedInstruction {
        let opcode = Instruction::try_from(value)
            .unwrap_or_else(|_| panic!("Failed to decode opcode {value:#X}"));
        let argument_kind = INSTRUCTIONS_ADDRESSING
            .get(&opcode)
            .unwrap_or_else(|| panic!("Unimplemented opcode {opcode:?}"));

        let arg: Argument = match *argument_kind {
            ArgumentType::Addr => {
                let low_byte = self.fetch(self.pc + 1);
                let high_byte = self.fetch(self.pc + 2);

                Argument::Addr(dword_from_nibbles(low_byte, high_byte))
                // TODO: Make args vec of Instruction ?
            }
            ArgumentType::Byte => Argument::Byte(self.fetch(self.pc + 1)),
            ArgumentType::Void => Argument::Void,
        };

        DecodedInstruction { int: opcode, arg }
    }

    fn fetch_operand(
        &self,
        instr: DecodedInstruction,
        addressing_type: AddressingType,
    ) -> FetchOperandResult {
        match addressing_type {
            AddressingType::XIndexedZeroIndirect => {
                let arg0: u8 = TryInto::<u8>::try_into(instr.arg)
                    .expect("x indexed zero indirect operand fetch error: expected byte");

                let x_indexed_ptr = u8::wrapping_add(self.x, arg0) as u16;

                let address = self.fetch_dword(x_indexed_ptr);

                FetchOperandResult(self.fetch(address), Some(address))
            }
            AddressingType::ZeroPage => {
                let arg0: u8 = TryInto::try_into(instr.arg)
                    .expect("zero page operand fetch error: expected zero page addr byte");

                FetchOperandResult(self.fetch(arg0 as u16), Some(arg0 as u16))
            }
            AddressingType::Immediate => FetchOperandResult(
                TryInto::try_into(instr.arg)
                    .expect("immediate operand fetch error: expected immediate byte"),
                None,
            ),
            AddressingType::Absolute => {
                let address: u16 = TryInto::try_into(instr.arg)
                    .expect("absolute operand fetch error: expected address");

                FetchOperandResult(self.fetch(address), Some(address))
            }
            AddressingType::ZeroIndirectIndexed => {
                let arg0: u8 = TryInto::try_into(instr.arg)
                    .expect("Zero indirect indexed operand fetch error: expected byte");

                let low_byte = self.fetch(arg0 as u16);
                let high_byte = self.fetch(arg0 as u16 + 1);
                let address = dword_from_nibbles(low_byte, high_byte);

                FetchOperandResult(self.fetch(self.y as u16 + address), Some(address))
            }
            AddressingType::XIndexedZero => {
                let arg0: u8 = TryInto::try_into(instr.arg)
                    .expect("X indexed zero page operand fetch error: expected byte");

                let x_indexed_ptr = u8::wrapping_add(self.x, arg0) as u16;

                FetchOperandResult(self.fetch(x_indexed_ptr), Some(x_indexed_ptr))
            }
            AddressingType::YIndexedZero => {
                let arg0: u8 = TryInto::try_into(instr.arg)
                    .expect("Y indexed zero page operand fetch error: expected byte");

                let y_indexed_ptr = u8::wrapping_add(self.y, arg0) as u16;

                FetchOperandResult(self.fetch(y_indexed_ptr), Some(y_indexed_ptr))
            }
            AddressingType::XIndexedAbsolute => {
                let address: u16 = TryInto::try_into(instr.arg)
                    .expect("X indexed absolute operand fetch error: expected address");

                let address_x_indexed = address + self.x as u16;

                FetchOperandResult(self.fetch(address_x_indexed), Some(address_x_indexed))
            }
            AddressingType::YIndexedAbsolute => {
                let address: u16 = TryInto::try_into(instr.arg)
                    .expect("Y indexed absolute operand fetch error: expected address");

                let address_y_indexed = address + self.y as u16;

                FetchOperandResult(self.fetch(address_y_indexed), Some(address_y_indexed))
            }
        }
    }

    fn execute(&mut self, instr: DecodedInstruction) {
        println!("Executing opcode {:#X}", instr.int as u8);
        match instr.int {
            Instruction::AdcXIndexedZeroIndirect => {
                let FetchOperandResult(operand, _) =
                    self.fetch_operand(instr, AddressingType::XIndexedZeroIndirect);
                self.adc(operand);
                self.pc += 2;
            }
            Instruction::AdcZeroPage => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::ZeroPage);
                self.adc(arg0);
                self.pc += 2;
            }
            Instruction::AdcImmediate => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::Immediate);

                self.adc(arg0);
                self.pc += 2;
            }
            Instruction::AdcAbsolute => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::Absolute);
                self.adc(arg0);
                self.pc += 3;
            }
            Instruction::AdcZeroIndirectIndexed => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::ZeroIndirectIndexed);
                self.adc(arg0);
                self.pc += 2;
            }
            Instruction::AdcXIndexedZero => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::XIndexedZero);
                self.adc(arg0);
                self.pc += 2;
            }
            Instruction::AdcYIndexedAbsolute => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::YIndexedAbsolute);
                self.adc(arg0);
                self.pc += 3;
            }
            Instruction::AdcXIndexedAbsolute => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::XIndexedAbsolute);
                self.adc(arg0);
                self.pc += 3;
            }
            // AND
            Instruction::AndXIndexedZeroIndirect => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::XIndexedZeroIndirect);
                self.and(arg0);
                self.pc += 2;
            }
            Instruction::AndZeroPage => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::ZeroPage);
                self.and(arg0);
                self.pc += 2;
            }
            Instruction::AndImmediate => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::Immediate);
                self.and(arg0);
                self.pc += 2;
            }
            Instruction::AndAbsolute => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::Absolute);
                self.and(arg0);
                self.pc += 3;
            }
            Instruction::AndZeroIndirectIndexed => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::ZeroIndirectIndexed);
                self.and(arg0);
                self.pc += 2;
            }
            Instruction::AndXIndexedZero => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::XIndexedZero);
                self.and(arg0);
                self.pc += 2;
            }
            Instruction::AndYIndexedAbsolute => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::YIndexedAbsolute);
                self.and(arg0);
                self.pc += 3;
            }
            Instruction::AndXIndexedAbsolute => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::XIndexedAbsolute);
                self.and(arg0);
                self.pc += 3;
            }
            // ASL
            Instruction::AslAbsolute => {
                let FetchOperandResult(arg0, address) =
                    self.fetch_operand(instr, AddressingType::Absolute);
                self.asl(AslLsrOperand::Value(arg0), address);
                self.pc += 3;
            }
            Instruction::AslZeroPage => {
                let FetchOperandResult(arg0, address) =
                    self.fetch_operand(instr, AddressingType::ZeroPage);
                self.asl(AslLsrOperand::Value(arg0), address);
                self.pc += 2;
            }
            Instruction::AslAccumulator => {
                self.asl(AslLsrOperand::A, None);
                self.pc += 1;
            }
            Instruction::AslXIndexedZero => {
                let FetchOperandResult(arg0, address) =
                    self.fetch_operand(instr, AddressingType::XIndexedZero);
                self.asl(AslLsrOperand::Value(arg0), address);
                self.pc += 2;
            }
            Instruction::AslXIndexedAbsolute => {
                let FetchOperandResult(arg0, address) =
                    self.fetch_operand(instr, AddressingType::XIndexedAbsolute);
                self.asl(AslLsrOperand::Value(arg0), address);
                self.pc += 3;
            }
            // Branch
            Instruction::Bcc => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::Immediate);

                self.pc += 2;
                self.branch(arg0 as i8, FlagPosition::Carry, false);
            }
            Instruction::Bcs => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::Immediate);

                self.pc += 2;
                self.branch(arg0 as i8, FlagPosition::Carry, true);
            }
            Instruction::Beq => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::Immediate);

                self.pc += 2;
                self.branch(arg0 as i8, FlagPosition::Zero, true);
            }
            Instruction::Bne => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::Immediate);

                self.pc += 2;
                self.branch(arg0 as i8, FlagPosition::Zero, false);
            }
            Instruction::Bmi => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::Immediate);

                self.pc += 2;
                self.branch(arg0 as i8, FlagPosition::Negative, true);
            }
            Instruction::Bpl => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::Immediate);

                self.pc += 2;
                self.branch(arg0 as i8, FlagPosition::Negative, false);
            }
            Instruction::Bvc => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::Immediate);

                self.pc += 2;
                self.branch(arg0 as i8, FlagPosition::Overflow, false);
            }
            Instruction::Bvs => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::Immediate);

                self.pc += 2;
                self.branch(arg0 as i8, FlagPosition::Overflow, true);
            }
            // BIT
            Instruction::BitZeroPage => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::ZeroPage);

                self.bit(arg0);
                self.pc += 2;
            }
            Instruction::BitAbsolute => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::Absolute);

                self.bit(arg0);
                self.pc += 3;
            }
            // Software interrupt
            Instruction::Brk => {
                self.brk();
            }
            // Flag reset
            Instruction::Clc => {
                self.clear_flag(FlagPosition::Carry);
                self.pc += 1;
            }
            Instruction::Cld => {
                self.clear_flag(FlagPosition::DecimalMode);
                self.pc += 1;
            }
            Instruction::Cli => {
                self.clear_flag(FlagPosition::IrqDisable);
                self.pc += 1;
            }
            Instruction::Clv => {
                self.clear_flag(FlagPosition::Overflow);
                self.pc += 1;
            }
            // CMP
            Instruction::CmpXIndexedZeroIndirect => {
                let FetchOperandResult(operand, _) =
                    self.fetch_operand(instr, AddressingType::XIndexedZeroIndirect);
                self.cmp(self.a, operand);
                self.pc += 2;
            }
            Instruction::CmpZeroPage => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::ZeroPage);
                self.cmp(self.a, arg0);
                self.pc += 2;
            }
            Instruction::CmpImmediate => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::Immediate);

                self.cmp(self.a, arg0);
                self.pc += 2;
            }
            Instruction::CmpAbsolute => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::Absolute);
                self.cmp(self.a, arg0);
                self.pc += 3;
            }
            Instruction::CmpZeroIndirectIndexed => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::ZeroIndirectIndexed);
                self.cmp(self.a, arg0);
                self.pc += 2;
            }
            Instruction::CmpXIndexedZero => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::XIndexedZero);
                self.cmp(self.a, arg0);
                self.pc += 2;
            }
            Instruction::CmpYIndexedAbsolute => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::YIndexedAbsolute);
                self.cmp(self.a, arg0);
                self.pc += 3;
            }
            Instruction::CmpXIndexedAbsolute => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::XIndexedAbsolute);
                self.cmp(self.a, arg0);
                self.pc += 3;
            }
            // CPX
            Instruction::CpxZeroPage => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::ZeroPage);
                self.cmp(self.x, arg0);
                self.pc += 2;
            }
            Instruction::CpxImmediate => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::Immediate);

                self.cmp(self.x, arg0);
                self.pc += 2;
            }
            Instruction::CpxAbsolute => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::Absolute);
                self.cmp(self.x, arg0);
                self.pc += 3;
            }
            // CPY
            Instruction::CpyZeroPage => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::ZeroPage);
                self.cmp(self.y, arg0);
                self.pc += 2;
            }
            Instruction::CpyImmediate => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::Immediate);

                self.cmp(self.y, arg0);
                self.pc += 2;
            }
            Instruction::CpyAbsolute => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::Absolute);
                self.cmp(self.y, arg0);
                self.pc += 3;
            }
            // DEC
            Instruction::DecAbsolute => {
                let FetchOperandResult(arg0, address) =
                    self.fetch_operand(instr, AddressingType::Absolute);
                self.inc_dec(false, IncDecOperand::Value(arg0), address);
                self.pc += 3;
            }
            Instruction::DecZeroPage => {
                let FetchOperandResult(arg0, address) =
                    self.fetch_operand(instr, AddressingType::ZeroPage);
                self.inc_dec(false, IncDecOperand::Value(arg0), address);
                self.pc += 2;
            }
            Instruction::DecXIndexedZero => {
                let FetchOperandResult(arg0, address) =
                    self.fetch_operand(instr, AddressingType::XIndexedZero);
                self.inc_dec(false, IncDecOperand::Value(arg0), address);
                self.pc += 2;
            }
            Instruction::DecXIndexedAbsolute => {
                let FetchOperandResult(arg0, address) =
                    self.fetch_operand(instr, AddressingType::XIndexedAbsolute);
                self.inc_dec(false, IncDecOperand::Value(arg0), address);
                self.pc += 3;
            }
            // DEX
            Instruction::Dex => {
                self.inc_dec(false, IncDecOperand::X, None);
                self.pc += 1;
            }
            // DEY
            Instruction::Dey => {
                self.inc_dec(false, IncDecOperand::Y, None);
                self.pc += 1;
            }
            // EOR
            Instruction::EorXIndexedZeroIndirect => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::XIndexedZeroIndirect);
                self.eor(arg0);
                self.pc += 2;
            }
            Instruction::EorZeroPage => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::ZeroPage);
                self.eor(arg0);
                self.pc += 2;
            }
            Instruction::EorImmediate => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::Immediate);
                self.eor(arg0);
                self.pc += 2;
            }
            Instruction::EorAbsolute => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::Absolute);
                self.eor(arg0);
                self.pc += 3;
            }
            Instruction::EorZeroIndirectIndexed => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::ZeroIndirectIndexed);
                self.eor(arg0);
                self.pc += 2;
            }
            Instruction::EorXIndexedZero => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::XIndexedZero);
                self.eor(arg0);
                self.pc += 2;
            }
            Instruction::EorYIndexedAbsolute => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::YIndexedAbsolute);
                self.eor(arg0);
                self.pc += 3;
            }
            Instruction::EorXIndexedAbsolute => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::XIndexedAbsolute);
                self.eor(arg0);
                self.pc += 3;
            }
            // INC
            Instruction::IncAbsolute => {
                let FetchOperandResult(arg0, address) =
                    self.fetch_operand(instr, AddressingType::Absolute);
                self.inc_dec(true, IncDecOperand::Value(arg0), address);
                self.pc += 3;
            }
            Instruction::IncZeroPage => {
                let FetchOperandResult(arg0, address) =
                    self.fetch_operand(instr, AddressingType::ZeroPage);
                self.inc_dec(true, IncDecOperand::Value(arg0), address);
                self.pc += 2;
            }
            Instruction::IncXIndexedZero => {
                let FetchOperandResult(arg0, address) =
                    self.fetch_operand(instr, AddressingType::XIndexedZero);
                self.inc_dec(true, IncDecOperand::Value(arg0), address);
                self.pc += 2;
            }
            Instruction::IncXIndexedAbsolute => {
                let FetchOperandResult(arg0, address) =
                    self.fetch_operand(instr, AddressingType::XIndexedAbsolute);
                self.inc_dec(true, IncDecOperand::Value(arg0), address);
                self.pc += 3;
            }
            // INX
            Instruction::Inx => {
                self.inc_dec(true, IncDecOperand::X, None);
                self.pc += 1;
            }
            // INY
            Instruction::Iny => {
                self.inc_dec(true, IncDecOperand::Y, None);
                self.pc += 1;
            }
            Instruction::Nop => {
                self.pc += 1;
            }
            Instruction::Jmp => {
                let addr: u16 =
                    TryInto::try_into(instr.arg).expect("JMP nnnn execute error: expected address");
                println!("jump addr {addr:#X}");

                self.pc = addr;
            }
            Instruction::JmpIndirect => {
                let indirect_addr: u16 = TryInto::try_into(instr.arg)
                    .expect("JMP (nnnn) execute error: expected address");
                println!("jump addr {indirect_addr:#X}");

                let addr = self.fetch_dword(indirect_addr);

                self.pc = addr;
            }
            Instruction::Jsr => {
                let addr: u16 =
                    TryInto::try_into(instr.arg).expect("JSR execute error: expected address");
                println!("jump addr {addr:#X}");

                self.jsr(addr);
            }
            // LDA
            Instruction::LdaXIndexedZeroIndirect => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::XIndexedZeroIndirect);
                self.ld(LdOperand::A, arg0);
                self.pc += 2;
            }
            Instruction::LdaZeroPage => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::ZeroPage);
                self.ld(LdOperand::A, arg0);
                self.pc += 2;
            }
            Instruction::LdaImmediate => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::Immediate);
                self.ld(LdOperand::A, arg0);
                self.pc += 2;
            }
            Instruction::LdaAbsolute => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::Absolute);
                self.ld(LdOperand::A, arg0);
                self.pc += 3;
            }
            Instruction::LdaZeroIndirectIndexed => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::ZeroIndirectIndexed);
                self.ld(LdOperand::A, arg0);
                self.pc += 2;
            }
            Instruction::LdaXIndexedZero => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::XIndexedZero);
                self.ld(LdOperand::A, arg0);
                self.pc += 2;
            }
            Instruction::LdaYIndexedAbsolute => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::YIndexedAbsolute);
                self.ld(LdOperand::A, arg0);
                self.pc += 3;
            }
            Instruction::LdaXIndexedAbsolute => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::XIndexedAbsolute);
                self.ld(LdOperand::A, arg0);
                self.pc += 3;
            }
            // LDX
            Instruction::LdxZeroPage => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::ZeroPage);
                self.ld(LdOperand::X, arg0);
                self.pc += 2;
            }
            Instruction::LdxImmediate => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::Immediate);
                self.ld(LdOperand::X, arg0);
                self.pc += 2;
            }
            Instruction::LdxAbsolute => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::Absolute);
                self.ld(LdOperand::X, arg0);
                self.pc += 3;
            }
            Instruction::LdxYIndexedAbsolute => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::YIndexedAbsolute);
                self.ld(LdOperand::X, arg0);
                self.pc += 3;
            }
            Instruction::LdxYIndexedZero => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::YIndexedZero);
                self.ld(LdOperand::X, arg0);
                self.pc += 2;
            }
            // LDY
            Instruction::LdyZeroPage => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::ZeroPage);
                self.ld(LdOperand::Y, arg0);
                self.pc += 2;
            }
            Instruction::LdyImmediate => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::Immediate);
                self.ld(LdOperand::Y, arg0);
                self.pc += 2;
            }
            Instruction::LdyAbsolute => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::Absolute);
                self.ld(LdOperand::Y, arg0);
                self.pc += 3;
            }
            Instruction::LdyXIndexedAbsolute => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::XIndexedAbsolute);
                self.ld(LdOperand::Y, arg0);
                self.pc += 3;
            }
            Instruction::LdyXIndexedZero => {
                let FetchOperandResult(arg0, _) =
                    self.fetch_operand(instr, AddressingType::XIndexedZero);
                self.ld(LdOperand::Y, arg0);
                self.pc += 2;
            }
            // LSR
            Instruction::LsrAbsolute => {
                let FetchOperandResult(arg0, address) =
                    self.fetch_operand(instr, AddressingType::Absolute);
                self.lsr(AslLsrOperand::Value(arg0), address);

                self.pc += 3;
            }
            Instruction::LsrZeroPage => {
                let FetchOperandResult(arg0, address) =
                    self.fetch_operand(instr, AddressingType::ZeroPage);
                self.lsr(AslLsrOperand::Value(arg0), address);
                self.pc += 2;
            }
            Instruction::LsrAccumulator => {
                self.lsr(AslLsrOperand::A, None);
                self.pc += 1;
            }
            Instruction::LsrXIndexedAbsolute => {
                let FetchOperandResult(arg0, address) =
                    self.fetch_operand(instr, AddressingType::XIndexedAbsolute);
                self.lsr(AslLsrOperand::Value(arg0), address);
                self.pc += 3;
            }
            Instruction::LsrXIndexedZero => {
                let FetchOperandResult(arg0, address) =
                    self.fetch_operand(instr, AddressingType::XIndexedZero);
                self.lsr(AslLsrOperand::Value(arg0), address);
                self.pc += 2;
            }
            _ => panic!("Unknown instruction {:?}", instr.int),
        }
    }

    fn adc(&mut self, operand: u8) {
        let carry = self.p.read_flag(FlagPosition::Carry);
        let result = u16::from(self.a) + u16::from(operand) + u16::from(carry);

        self.p.write_flag(FlagPosition::Carry, result > 255);
        self.p.write_flag(FlagPosition::Zero, result == 0);

        let overflow: bool = i8::checked_add(self.a as i8, operand as i8)
            .and_then(|x| i8::checked_add(x, carry as i8))
            .map_or(true, |_| false);

        self.p.write_flag(FlagPosition::Overflow, overflow);
        self.p
            .write_flag(FlagPosition::Negative, (result & 0b1000_0000) >> 7 == 1);

        self.a = result as u8;
    }

    fn and(&mut self, operand: u8) {
        let result = self.a & operand;

        self.p.write_flag(FlagPosition::Zero, result == 0);
        self.p
            .write_flag(FlagPosition::Negative, (result & 0b1000_0000) >> 7 == 1);

        self.a = result;
    }

    fn asl(&mut self, operand: AslLsrOperand, operand_address: Option<u16>) {
        let operand_value: u8 = match operand {
            AslLsrOperand::A => self.a,
            AslLsrOperand::Value(v) => v,
        };

        let result = operand_value.wrapping_shl(1);

        self.p
            .write_flag(FlagPosition::Carry, (operand_value & 0b1000_0000) >> 7 == 1);
        self.p
            .write_flag(FlagPosition::Negative, (result & 0b1000_0000) >> 7 == 1);
        self.p.write_flag(FlagPosition::Zero, result == 0);

        match operand {
            AslLsrOperand::A => self.a = result,
            AslLsrOperand::Value(_) => self.address_space.write_byte(
                operand_address.expect("ASL: expected address") as usize,
                result,
            ),
        }
    }

    fn branch(&mut self, offset: i8, flag: FlagPosition, set: bool) {
        // PC is already on next command after branch here

        if self.p.read_flag(flag) == set as u8 {
            self.pc = self.pc.wrapping_add(offset as i16 as u16);
        }
    }

    fn bit(&mut self, operand: u8) {
        let result = self.a & operand;

        self.p.write_flag(FlagPosition::Zero, result == 0);
        self.p
            .write_flag(FlagPosition::Overflow, (operand & 0b0100_0000) >> 6 == 1);
        self.p
            .write_flag(FlagPosition::Negative, (operand & 0b1000_0000) >> 7 == 1);
    }

    fn brk(&mut self) {
        self.p.write_flag(FlagPosition::IrqDisable, true);

        let high_byte = (self.pc & 0xFF00) >> 8;
        let low_byte = self.pc & 0x00FF;

        self.address_space
            .write_byte(self.s as usize, high_byte as u8);
        self.s = self.s.wrapping_sub(1);

        self.address_space
            .write_byte(self.s as usize, low_byte as u8);
        self.s = self.s.wrapping_sub(1);

        self.address_space
            .write_byte(self.s as usize, Into::<u8>::into(&self.p));
        self.s = self.s.wrapping_sub(1);

        let irq_vec_high_byte = self.address_space.read_byte(0xFFFF);
        let irq_vec_low_byte = self.address_space.read_byte(0xFFFE);

        self.pc = dword_from_nibbles(irq_vec_low_byte, irq_vec_high_byte);
    }

    fn clear_flag(&mut self, flag: FlagPosition) {
        match flag {
            FlagPosition::Carry
            | FlagPosition::DecimalMode
            | FlagPosition::IrqDisable
            | FlagPosition::Overflow => self.p.write_flag(flag, false),
            _ => panic!("Unsupported clear flag instruction for flag {}", flag as u8),
        }
    }

    fn cmp(&mut self, register: u8, operand: u8) {
        let result = u8::saturating_sub(register, operand);

        self.p.write_flag(FlagPosition::Zero, result == 0);
        self.p
            .write_flag(FlagPosition::Negative, (result & 0b1000_0000) >> 7 == 1);
        self.p.write_flag(FlagPosition::Carry, register >= operand);
    }

    fn inc_dec(&mut self, inc: bool, operand: IncDecOperand, operand_address: Option<u16>) {
        let operand_value: u8 = match operand {
            IncDecOperand::X => self.x,
            IncDecOperand::Y => self.y,
            IncDecOperand::Value(v) => v,
        };

        let result = if inc {
            u8::wrapping_add(operand_value, 1)
        } else {
            u8::wrapping_sub(operand_value, 1)
        };

        self.p.write_flag(FlagPosition::Zero, result == 0);
        self.p
            .write_flag(FlagPosition::Negative, (result & 0b1000_0000) >> 7 == 1);

        match operand {
            IncDecOperand::X => self.x = result,
            IncDecOperand::Y => self.y = result,
            IncDecOperand::Value(_) => self.address_space.write_byte(
                operand_address.expect("INC/DEC: expected address") as usize,
                result,
            ),
        }
    }

    fn eor(&mut self, operand: u8) {
        let result = self.a ^ operand;

        self.p.write_flag(FlagPosition::Zero, result == 0);
        self.p
            .write_flag(FlagPosition::Negative, (result & 0b1000_0000) >> 7 == 1);

        self.a = result;
    }

    fn jsr(&mut self, address: u16) {
        self.pc += 2;

        let high_byte = (self.pc & 0xFF00) >> 8;
        let low_byte = self.pc & 0x00FF;

        self.address_space
            .write_byte(self.s as usize, high_byte as u8);
        self.s = self.s.wrapping_sub(1);

        self.address_space
            .write_byte(self.s as usize, low_byte as u8);
        self.s = self.s.wrapping_sub(1);

        self.pc = address;
    }

    fn ld(&mut self, register: LdOperand, operand: u8) {
        match register {
            LdOperand::A => {
                self.a = operand;
            }
            LdOperand::X => {
                self.x = operand;
            }
            LdOperand::Y => {
                self.y = operand;
            }
        }

        self.p.write_flag(FlagPosition::Zero, operand == 0);
        self.p
            .write_flag(FlagPosition::Negative, (operand & 0b1000_0000) >> 7 == 1);
    }

    fn lsr(&mut self, operand: AslLsrOperand, operand_address: Option<u16>) {
        let operand_value: u8 = match operand {
            AslLsrOperand::A => self.a,
            AslLsrOperand::Value(v) => v,
        };

        let result = operand_value >> 1;

        self.p
            .write_flag(FlagPosition::Carry, (operand_value & 0b0000_0001) == 1);
        self.p.write_flag(FlagPosition::Negative, false);
        self.p.write_flag(FlagPosition::Zero, result == 0);

        match operand {
            AslLsrOperand::A => self.a = result,
            AslLsrOperand::Value(_) => self.address_space.write_byte(
                operand_address.expect("LSR: expected address") as usize,
                result,
            ),
        }
    }
}
