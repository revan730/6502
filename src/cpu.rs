use std::fmt;

use crate::{
    instruction::Instruction, memory_bus::MEM_SPACE_END, opcode_decoders::OPCODE_DECODERS,
};

pub struct Cpu {
    pub address_space: [u8; MEM_SPACE_END], // TODO: replace with memory bus implementation
    pub a: u8,                              // Accumulator register
    pub x: u8,                              // X index register
    pub y: u8,                              // Y index register
    pub pc: u16,                            // Program counter
    pub s: u8,                              // Stack pointer
    pub p: u8,                              // Flags register
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

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            address_space: [0; MEM_SPACE_END], // TODO: Init memory bus here
            a: 0,
            x: 0,
            y: 0,
            pc: 0, // TODO: Probably should point to reset vector
            s: 0,
            p: 0,
        }
    }

    pub fn step(&mut self) {
        self.address_space[0] = 0xEA;
        self.address_space[1] = 0x69;
        self.address_space[2] = 0xCA;
        self.address_space[3] = 0x4C;
        self.address_space[4] = 0x00;
        self.address_space[5] = 0x00;

        let opcode = self.fetch(self.pc);
        let instruction = self.decode(opcode);

        self.execute(instruction);
    }

    fn fetch(&self, address: u16) -> u8 {
        const SPACE_END: u16 = MEM_SPACE_END as u16;
        match address {
            0..=SPACE_END => self.address_space[address as usize],
            _ => panic!("PC address out of bounds"),
        }
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
            Instruction::AdcImmediate => {
                let arg0 = instr
                    .args
                    .get(0)
                    .expect("execute JMP nn error: expected immediate byte");
                let result = u16::from(self.a) + u16::from(*arg0);

                if result > 255 {
                    self.p |= 1;
                } else {
                    self.p &= !1;
                }

                self.a = result as u8;

                self.pc += 2;
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

                let addr = u16::from(*high_byte) << 8 | u16::from(*low_byte);
                println!("jump addr {:#X}", addr);

                self.pc = addr;
            }
            _ => panic!("Unknown instruction {:?}", instr.int),
        }
    }
}
