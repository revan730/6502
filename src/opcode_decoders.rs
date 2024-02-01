use crate::instruction::Instruction;
use std::collections::HashMap;

#[derive(Debug)]
pub enum ArgumentType {
    Void, // Opcode without arguments
    Byte, // Opcode with single argument
    Addr, // Opcode with two address (two bytes) argument
}

lazy_static! {
    pub static ref INSTRUCTIONS_ADDRESSING: HashMap<Instruction, ArgumentType> = {
        let mut m = HashMap::new();
        m.insert(Instruction::AdcXIndexedZeroIndirect, ArgumentType::Byte);
        m.insert(Instruction::AdcZeroPage, ArgumentType::Byte);
        m.insert(Instruction::AdcImmediate, ArgumentType::Byte);
        m.insert(Instruction::AdcZeroIndirectIndexed, ArgumentType::Byte);
        m.insert(Instruction::AdcXIndexedZero, ArgumentType::Byte);
        m.insert(Instruction::AdcYIndexedAbsolute, ArgumentType::Addr);
        m.insert(Instruction::AdcXIndexedAbsolute, ArgumentType::Addr);
        m.insert(Instruction::AdcAbsolute, ArgumentType::Addr);

        m.insert(Instruction::AndXIndexedZeroIndirect, ArgumentType::Byte);
        m.insert(Instruction::AndZeroPage, ArgumentType::Byte);
        m.insert(Instruction::AndImmediate, ArgumentType::Byte);
        m.insert(Instruction::AndAbsolute, ArgumentType::Addr);
        m.insert(Instruction::AndZeroIndirectIndexed, ArgumentType::Byte);
        m.insert(Instruction::AndXIndexedZero, ArgumentType::Byte);
        m.insert(Instruction::AndXIndexedAbsolute, ArgumentType::Addr);
        m.insert(Instruction::AndYIndexedAbsolute, ArgumentType::Addr);

        m.insert(Instruction::Jmp, ArgumentType::Addr);

        m.insert(Instruction::Nop, ArgumentType::Void);

        m
    };
}
