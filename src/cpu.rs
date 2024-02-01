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

    fn fetch_operand(&self, instr: DecodedInstruction, addressing_type: AddressingType) -> u8 {
        match addressing_type {
            AddressingType::XIndexedZeroIndirect => {
                let arg0: u8 = TryInto::<u8>::try_into(instr.arg)
                    .expect("x indexed zero indirect operand fetch error: expected byte");

                let x_indexed_ptr = u8::wrapping_add(self.x, arg0) as u16;

                let address = self.fetch_dword(x_indexed_ptr);

                self.fetch(address)
            }
            AddressingType::ZeroPage => {
                let arg0: u8 = TryInto::try_into(instr.arg)
                    .expect("zero page operand fetch error: expected zero page addr byte");

                self.fetch(arg0 as u16)
            }
            AddressingType::Immediate => TryInto::try_into(instr.arg)
                .expect("immediate operand fetch error: expected immediate byte"),
            AddressingType::Absolute => {
                let address: u16 = TryInto::try_into(instr.arg)
                    .expect("absolute operand fetch error: expected address");

                self.fetch(address)
            }
            AddressingType::ZeroIndirectIndexed => {
                let arg0: u8 = TryInto::try_into(instr.arg)
                    .expect("Zero indirect indexed operand fetch error: expected byte");

                let low_byte = self.fetch(arg0 as u16);
                let high_byte = self.fetch(arg0 as u16 + 1);
                let address = dword_from_nibbles(low_byte, high_byte);

                self.fetch(self.y as u16 + address)
            }
            AddressingType::XIndexedZero => {
                let arg0: u8 = TryInto::try_into(instr.arg)
                    .expect("X indexed zero page operand fetch error: expected byte");

                let x_indexed_ptr = u8::wrapping_add(self.x, arg0) as u16;

                self.fetch(x_indexed_ptr)
            }
            AddressingType::XIndexedAbsolute => {
                let address: u16 = TryInto::try_into(instr.arg)
                    .expect("X indexed absolute operand fetch error: expected address");

                let address_x_indexed = address + self.x as u16;

                self.fetch(address_x_indexed)
            }
            AddressingType::YIndexedAbsolute => {
                let address: u16 = TryInto::try_into(instr.arg)
                    .expect("Y indexed absolute operand fetch error: expected address");

                let address_y_indexed = address + self.y as u16;

                self.fetch(address_y_indexed)
            }
        }
    }

    fn execute(&mut self, instr: DecodedInstruction) {
        match instr.int {
            Instruction::AdcXIndexedZeroIndirect => {
                let operand = self.fetch_operand(instr, AddressingType::XIndexedZeroIndirect);
                self.adc(operand);
                self.pc += 2;
            }
            Instruction::AdcZeroPage => {
                let arg0 = self.fetch_operand(instr, AddressingType::ZeroPage);
                self.adc(arg0);
                self.pc += 2;
            }
            Instruction::AdcImmediate => {
                let arg0 = self.fetch_operand(instr, AddressingType::Immediate);

                self.adc(arg0);
                self.pc += 2;
            }
            Instruction::AdcAbsolute => {
                let arg0 = self.fetch_operand(instr, AddressingType::Absolute);
                self.adc(arg0);
                self.pc += 3;
            }
            Instruction::AdcZeroIndirectIndexed => {
                let arg0 = self.fetch_operand(instr, AddressingType::ZeroIndirectIndexed);
                self.adc(arg0);
                self.pc += 2;
            }
            Instruction::AdcXIndexedZero => {
                let arg0 = self.fetch_operand(instr, AddressingType::XIndexedZero);
                self.adc(arg0);
                self.pc += 2;
            }
            Instruction::AdcYIndexedAbsolute => {
                let arg0 = self.fetch_operand(instr, AddressingType::YIndexedAbsolute);
                self.adc(arg0);
                self.pc += 3;
            }
            Instruction::AdcXIndexedAbsolute => {
                let arg0 = self.fetch_operand(instr, AddressingType::XIndexedAbsolute);
                self.adc(arg0);
                self.pc += 3;
            }
            // AND
            Instruction::AndXIndexedZeroIndirect => {
                let operand = self.fetch_operand(instr, AddressingType::XIndexedZeroIndirect);
                self.and(operand);
                self.pc += 2;
            }
            Instruction::AndZeroPage => {
                let arg0 = self.fetch_operand(instr, AddressingType::ZeroPage);
                self.and(arg0);
                self.pc += 2;
            }
            Instruction::AndImmediate => {
                let arg0 = self.fetch_operand(instr, AddressingType::Immediate);
                self.and(arg0);
                self.pc += 2;
            }
            Instruction::AndAbsolute => {
                let arg0 = self.fetch_operand(instr, AddressingType::Absolute);
                self.and(arg0);
                self.pc += 3;
            }
            Instruction::AndZeroIndirectIndexed => {
                let arg0 = self.fetch_operand(instr, AddressingType::ZeroIndirectIndexed);
                self.and(arg0);
                self.pc += 2;
            }
            Instruction::AndXIndexedZero => {
                let arg0 = self.fetch_operand(instr, AddressingType::XIndexedZero);
                self.and(arg0);
                self.pc += 2;
            }
            Instruction::AndYIndexedAbsolute => {
                let arg0 = self.fetch_operand(instr, AddressingType::YIndexedAbsolute);
                self.and(arg0);
                self.pc += 3;
            }
            Instruction::AndXIndexedAbsolute => {
                let arg0 = self.fetch_operand(instr, AddressingType::XIndexedAbsolute);
                self.and(arg0);
                self.pc += 3;
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
}
