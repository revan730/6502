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
    pub argument_decoders: Vec<ArgumentDecoder>,
}

lazy_static! {
    pub static ref OPCODE_DECODERS: HashMap<Instruction, OpcodeDecoder<'static>> = {
        let mut m = HashMap::new();
        m.insert(
            Instruction::AdcZeroPage,
            OpcodeDecoder {
                name: "ADC n",
                instruction: Instruction::AdcZeroPage,
                argument_decoders: vec![ArgumentDecoder {
                    index: 0,
                    kind: ArgumentType::Byte,
                }],
            },
        );

        m.insert(
            Instruction::AdcImmediate,
            OpcodeDecoder {
                name: "ADC #n",
                instruction: Instruction::AdcImmediate,
                argument_decoders: vec![ArgumentDecoder {
                    index: 0,
                    kind: ArgumentType::Byte,
                }],
            },
        );

        m.insert(
            Instruction::AdcAbsolute,
            OpcodeDecoder {
                name: "ADC nn",
                instruction: Instruction::AdcAbsolute,
                argument_decoders: vec![ArgumentDecoder {
                    index: 0,
                    kind: ArgumentType::Addr,
                }],
            },
        );

        m.insert(
            Instruction::JMP,
            OpcodeDecoder {
                name: "JMP nn",
                instruction: Instruction::JMP,
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
                argument_decoders: vec![],
            },
        );

        m
    };
}
