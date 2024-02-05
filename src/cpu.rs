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

enum AslOperand {
    A,
    Value(u8),
}

enum DecOperand {
    X,
    Y,
    Value(u8),
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
            address_space: mem_bus, // TODO: Init memory bus here
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
        let opcode = Instruction::try_from(value).expect("Failed to decode opcode");
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
                self.asl(AslOperand::Value(arg0), address);
                self.pc += 3;
            }
            Instruction::AslZeroPage => {
                let FetchOperandResult(arg0, address) =
                    self.fetch_operand(instr, AddressingType::ZeroPage);
                self.asl(AslOperand::Value(arg0), address);
                self.pc += 2;
            }
            Instruction::AslAccumulator => {
                self.asl(AslOperand::A, None);
                self.pc += 1;
            }
            Instruction::AslXIndexedZero => {
                let FetchOperandResult(arg0, address) =
                    self.fetch_operand(instr, AddressingType::XIndexedZero);
                self.asl(AslOperand::Value(arg0), address);
                self.pc += 2;
            }
            Instruction::AslXIndexedAbsolute => {
                let FetchOperandResult(arg0, address) =
                    self.fetch_operand(instr, AddressingType::XIndexedAbsolute);
                self.asl(AslOperand::Value(arg0), address);
                self.pc += 3;
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
                self.dec(DecOperand::Value(arg0), address);
                self.pc += 3;
            }
            Instruction::DecZeroPage => {
                let FetchOperandResult(arg0, address) =
                    self.fetch_operand(instr, AddressingType::ZeroPage);
                self.dec(DecOperand::Value(arg0), address);
                self.pc += 2;
            }
            Instruction::DecXIndexedZero => {
                let FetchOperandResult(arg0, address) =
                    self.fetch_operand(instr, AddressingType::XIndexedZero);
                self.dec(DecOperand::Value(arg0), address);
                self.pc += 2;
            }
            Instruction::DecXIndexedAbsolute => {
                let FetchOperandResult(arg0, address) =
                    self.fetch_operand(instr, AddressingType::XIndexedAbsolute);
                self.dec(DecOperand::Value(arg0), address);
                self.pc += 3;
            }
            // DEX
            Instruction::Dex => {
                self.dec(DecOperand::X, None);
                self.pc += 1;
            }
            // DEY
            Instruction::Dey => {
                self.dec(DecOperand::Y, None);
                self.pc += 1;
            }
            Instruction::Nop => {
                self.pc += 1;
            }
            Instruction::Jmp => {
                let addr: u16 =
                    TryInto::try_into(instr.arg).expect("JMP execute error: expected address");
                println!("jump addr {addr:#X}");

                self.pc = addr;
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

    fn asl(&mut self, operand: AslOperand, operand_address: Option<u16>) {
        let operand_value: u8 = match operand {
            AslOperand::A => self.a,
            AslOperand::Value(v) => v,
        };

        let result = operand_value.wrapping_shl(1);
        println!("operand_value {operand_value:#X} result {result:#X}");

        self.p
            .write_flag(FlagPosition::Carry, (operand_value & 0b1000_0000) >> 7 == 1);
        self.p
            .write_flag(FlagPosition::Negative, (result & 0b1000_0000) >> 7 == 1);
        self.p.write_flag(FlagPosition::Zero, result == 0);

        match operand {
            AslOperand::A => self.a = result,
            AslOperand::Value(_) => self.address_space.write_byte(
                operand_address.expect("ASL: expected address") as usize,
                result,
            ),
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

    fn dec(&mut self, operand: DecOperand, operand_address: Option<u16>) {
        let operand_value: u8 = match operand {
            DecOperand::X => self.x,
            DecOperand::Y => self.y,
            DecOperand::Value(v) => v,
        };

        let result = u8::wrapping_sub(operand_value, 1);

        self.p.write_flag(FlagPosition::Zero, result == 0);
        self.p
            .write_flag(FlagPosition::Negative, (result & 0b1000_0000) >> 7 == 1);

        match operand {
            DecOperand::X => self.x = result,
            DecOperand::Y => self.y = result,
            DecOperand::Value(_) => self.address_space.write_byte(
                operand_address.expect("DEC: expected address") as usize,
                result,
            ),
        }
    }
}
