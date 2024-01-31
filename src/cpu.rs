use std::fmt;

use crate::{
    instruction::Instruction,
    memory_bus::{MemoryBus, MEM_SPACE_END, ZERO_PAGE_END, ZERO_PAGE_SIZE},
    opcode_decoders::OPCODE_DECODERS,
};

pub struct Cpu {
    pub address_space: MemoryBus, // TODO: replace with memory bus implementation
    pub a: u8,                    // Accumulator register
    pub x: u8,                    // X index register
    pub y: u8,                    // Y index register
    pub pc: u16,                  // Program counter
    pub s: u8,                    // Stack pointer
    pub p: u8,                    // Flags register
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Registers:").unwrap();

        writeln!(f, "A: {:#X}", self.a).unwrap();
        writeln!(f, "X: {:#X}", self.x).unwrap();
        writeln!(f, "Y: {:#X}", self.y).unwrap();
        writeln!(f, "PC: {:#X}", self.pc).unwrap();
        writeln!(f, "S: {:#X} P: {:#X}", self.s, self.p)
    }
}

#[derive(Debug)]
struct DecodedInstruction {
    pub int: Instruction,
    pub args: Vec<u8>,
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
            p: 0,
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
        let decoder = OPCODE_DECODERS
            .get(&opcode)
            .expect(format!("Unimplemented opcode {:?}", opcode).as_str());

        let mut args = Vec::with_capacity(decoder.argument_decoders.len());
        for arg in decoder.argument_decoders.iter() {
            match arg.kind {
                crate::opcode_decoders::ArgumentType::Addr => {
                    let low_byte = self.fetch(self.pc + arg.index as u16 + 1);
                    let high_byte = self.fetch(self.pc + arg.index as u16 + 2);

                    args.push(low_byte);
                    args.push(high_byte);
                    // TODO: Make args vec of Instruction ?
                }
                _ => {
                    let arg_byte = self.fetch(self.pc + arg.index as u16 + 1);
                    args.push(arg_byte);
                }
            }
        }

        DecodedInstruction {
            int: decoder.instruction,
            args,
        }
    }

    fn execute(&mut self, instr: DecodedInstruction) {
        match instr.int {
            Instruction::AdcXIndexedZeroIndirect => {
                let arg0 = instr
                    .args
                    .get(0)
                    .expect("execute ADC (n,X) error: expected byte");

                let x_indexed_ptr = u8::wrapping_add(self.x, *arg0) as u16;

                let address = self.fetch_dword(x_indexed_ptr);

                let operand = self.fetch(address);

                self.adc(operand);
                self.pc += 2;
            }
            Instruction::AdcZeroPage => {
                let arg0 = instr
                    .args
                    .get(0)
                    .expect("execute ADC n error: expected zero page addr byte");

                let arg0 = self.fetch(*arg0 as u16);

                self.adc(arg0);
                self.pc += 2;
            }
            Instruction::AdcImmediate => {
                let arg0 = instr
                    .args
                    .get(0)
                    .expect("execute ADC #n error: expected immediate byte");

                self.adc(*arg0);
                self.pc += 2;
            }
            Instruction::AdcAbsolute => {
                let low_byte = instr
                    .args
                    .get(0)
                    .expect("execute ADC nn error: expected address low byte");

                let high_byte = instr
                    .args
                    .get(1)
                    .expect("execute ADC nn error: expected address high byte");

                let arg0 = self.fetch(dword_from_nibbles(*low_byte, *high_byte));
                self.adc(arg0);
                self.pc += 3;
            }
            Instruction::AdcZeroIndirectIndexed => {
                let arg0 = instr
                    .args
                    .get(0)
                    .expect("execute ADC (n),Y error: expected byte");

                let low_byte = self.fetch(*arg0 as u16);
                let high_byte = self.fetch(*arg0 as u16 + 1);
                let address = dword_from_nibbles(low_byte, high_byte);

                let operand = self.fetch(self.y as u16 + address);

                self.adc(operand);
                self.pc += 2;
            }
            Instruction::AdcXIndexedZero => {
                let arg0 = instr
                    .args
                    .get(0)
                    .expect("execute ADC n,X error: expected byte");

                let x_indexed_ptr = u8::wrapping_add(self.x, *arg0) as u16;

                let operand = self.fetch(x_indexed_ptr);

                self.adc(operand);
                self.pc += 2;
            }
            Instruction::AdcYIndexedAbsolute => {
                let low_byte = instr
                    .args
                    .get(0)
                    .expect("execute ADC nn,Y error: expected address low byte");

                let high_byte = instr
                    .args
                    .get(1)
                    .expect("execute ADC nn,Y error: expected address high byte");

                let address = (dword_from_nibbles(*low_byte, *high_byte)) + self.y as u16;

                let operand = self.fetch(address);

                self.adc(operand);
                self.pc += 3;
            }
            Instruction::AdcXIndexedAbsolute => {
                let low_byte = instr
                    .args
                    .get(0)
                    .expect("execute ADC nn,X error: expected address low byte");

                let high_byte = instr
                    .args
                    .get(1)
                    .expect("execute ADC nn,X error: expected address high byte");

                let address = (dword_from_nibbles(*low_byte, *high_byte)) + self.x as u16;

                let operand = self.fetch(address);

                self.adc(operand);
                self.pc += 3;
            }
            Instruction::NOP => {
                self.pc += 1;
            }
            Instruction::JMP => {
                let low_byte = instr
                    .args
                    .get(0)
                    .expect("execute JMP nn error: expected address low byte");
                let high_byte = instr
                    .args
                    .get(1)
                    .expect("execute JMP nn error: expected address high byte");

                let addr = dword_from_nibbles(*low_byte, *high_byte);
                println!("jump addr {:#X}", addr);

                self.pc = addr;
            }
            _ => panic!("Unknown instruction {:?}", instr.int),
        }
    }

    fn adc(&mut self, operand: u8) {
        let carry = self.p & 0x1;
        let result = u16::from(self.a) + u16::from(operand) + u16::from(carry);

        // carry flag
        if result > 255 {
            self.p |= 1;
        } else {
            self.p &= !1;
        }

        // zero flag
        if result == 0 {
            self.p |= 1 << 1;
        } else {
            self.p &= !(1 << 1);
        }

        let overflow: bool = i8::checked_add(self.a as i8, operand as i8)
            .and_then(|x| i8::checked_add(x, carry as i8))
            .map_or(true, |_| false);

        if overflow {
            self.p |= 1 << 6;
        } else {
            self.p &= !(1 << 6);
        }

        // negative flag
        if (result & 0b10000000) >> 7 == 1 {
            self.p |= 1 << 7;
        } else {
            self.p &= !(1 << 7);
        }

        self.a = result as u8;
    }
}
