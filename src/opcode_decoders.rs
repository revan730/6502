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
            Instruction::AdcXIndexedZeroIndirect,
            OpcodeDecoder {
                name: "ADC (n,X)",
                instruction: Instruction::AdcZeroPage,
                argument_decoders: vec![ArgumentDecoder {
                    index: 0,
                    kind: ArgumentType::Byte,
                }],
            },
        );

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
            Instruction::AdcZeroIndirectIndexed,
            OpcodeDecoder {
                name: "ADC (n),Y",
                instruction: Instruction::AdcZeroIndirectIndexed,
                argument_decoders: vec![ArgumentDecoder {
                    index: 0,
                    kind: ArgumentType::Byte,
                }],
            },
        );

        m.insert(
            Instruction::AdcXIndexedZero,
            OpcodeDecoder {
                name: "ADC n,X",
                instruction: Instruction::AdcXIndexedZero,
                argument_decoders: vec![ArgumentDecoder {
                    index: 0,
                    kind: ArgumentType::Byte,
                }],
            },
        );

        m.insert(
            Instruction::AdcYIndexedAbsolute,
            OpcodeDecoder {
                name: "ADC nn,Y",
                instruction: Instruction::AdcYIndexedAbsolute,
                argument_decoders: vec![ArgumentDecoder {
                    index: 0,
                    kind: ArgumentType::Addr,
                }],
            },
        );

        m.insert(
            Instruction::AdcXIndexedAbsolute,
            OpcodeDecoder {
                name: "ADC nn,X",
                instruction: Instruction::AdcXIndexedAbsolute,
                argument_decoders: vec![ArgumentDecoder {
                    index: 0,
                    kind: ArgumentType::Addr,
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
            Instruction::AndXIndexedZeroIndirect,
            OpcodeDecoder {
                name: "AND (n,X)",
                instruction: Instruction::AndXIndexedZeroIndirect,
                argument_decoders: vec![ArgumentDecoder {
                    index: 0,
                    kind: ArgumentType::Byte,
                }],
            },
        );

        m.insert(
            Instruction::AndZeroPage,
            OpcodeDecoder {
                name: "AND n",
                instruction: Instruction::AndZeroPage,
                argument_decoders: vec![ArgumentDecoder {
                    index: 0,
                    kind: ArgumentType::Byte,
                }],
            },
        );

        m.insert(
            Instruction::AndImmediate,
            OpcodeDecoder {
                name: "AND #n",
                instruction: Instruction::AndImmediate,
                argument_decoders: vec![ArgumentDecoder {
                    index: 0,
                    kind: ArgumentType::Byte,
                }],
            },
        );

        m.insert(
            Instruction::AndAbsolute,
            OpcodeDecoder {
                name: "AND nn",
                instruction: Instruction::AndAbsolute,
                argument_decoders: vec![ArgumentDecoder {
                    index: 0,
                    kind: ArgumentType::Addr,
                }],
            },
        );

        m.insert(
            Instruction::AndZeroIndirectIndexed,
            OpcodeDecoder {
                name: "AND (n),Y",
                instruction: Instruction::AndZeroIndirectIndexed,
                argument_decoders: vec![ArgumentDecoder {
                    index: 0,
                    kind: ArgumentType::Byte,
                }],
            },
        );

        m.insert(
            Instruction::AndXIndexedZero,
            OpcodeDecoder {
                name: "AND n,X",
                instruction: Instruction::AndXIndexedZero,
                argument_decoders: vec![ArgumentDecoder {
                    index: 0,
                    kind: ArgumentType::Byte,
                }],
            },
        );

        m.insert(
            Instruction::AndYIndexedAbsolute,
            OpcodeDecoder {
                name: "AND nn,Y",
                instruction: Instruction::AndYIndexedAbsolute,
                argument_decoders: vec![ArgumentDecoder {
                    index: 0,
                    kind: ArgumentType::Addr,
                }],
            },
        );

        m.insert(
            Instruction::AdcXIndexedAbsolute,
            OpcodeDecoder {
                name: "ADC nn,X",
                instruction: Instruction::AdcXIndexedAbsolute,
                argument_decoders: vec![ArgumentDecoder {
                    index: 0,
                    kind: ArgumentType::Addr,
                }],
            },
        );

        m.insert(
            Instruction::AndYIndexedAbsolute,
            OpcodeDecoder {
                name: "AND nn,Y",
                instruction: Instruction::AndYIndexedAbsolute,
                argument_decoders: vec![ArgumentDecoder {
                    index: 0,
                    kind: ArgumentType::Addr,
                }],
            },
        );

        m.insert(
            Instruction::AndXIndexedAbsolute,
            OpcodeDecoder {
                name: "AND nn,X",
                instruction: Instruction::AndXIndexedAbsolute,
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
