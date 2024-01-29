use crate::instruction::Instruction;
use std::collections::HashMap;

#[derive(Debug)]
pub enum ArgumentType {
    Reg,
    Byte,
    Addr,
}

#[derive(Debug)]
pub struct ArgumentDecoder {
    pub index: u8, // Index of the argument after opcode itself
    // ex. for AND nn first arg will have index 0
    pub kind: ArgumentType,
}

#[derive(Debug)]
pub struct OpcodeDecoder<'a> {
    pub name: &'a str,
    pub instruction: Instruction,
    pub opcode: u16,
    pub argument_decoders: Vec<ArgumentDecoder>,
}

lazy_static! {
    pub static ref OPCODE_DECODERS: HashMap<Instruction, OpcodeDecoder<'static>> = {
        let mut m = HashMap::new();
        m.insert(
            Instruction::AdcImmediate,
            OpcodeDecoder {
                name: "ADC #n",
                instruction: Instruction::AdcImmediate,
                opcode: 0x69,
                argument_decoders: vec![ArgumentDecoder {
                    index: 0,
                    kind: ArgumentType::Byte,
                }],
            },
        );

        m.insert(
            Instruction::JMP,
            OpcodeDecoder {
                name: "JMP nn",
                instruction: Instruction::JMP,
                opcode: 0x4C,
                argument_decoders: vec![ArgumentDecoder {
                    index: 0,
                    kind: ArgumentType::Addr,
                }],
            },
        );

        m.insert(
            Instruction::NOP,
            OpcodeDecoder {
                name: "NOP",
                instruction: Instruction::NOP,
                opcode: 0xEA,
                argument_decoders: vec![],
            },
        );

        m
    };
}
