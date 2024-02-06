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

        m.insert(Instruction::AslAbsolute, ArgumentType::Addr);
        m.insert(Instruction::AslZeroPage, ArgumentType::Byte);
        m.insert(Instruction::AslAccumulator, ArgumentType::Void);
        m.insert(Instruction::AslXIndexedZero, ArgumentType::Byte);
        m.insert(Instruction::AslXIndexedAbsolute, ArgumentType::Addr);

        m.insert(Instruction::Bcc, ArgumentType::Byte);
        m.insert(Instruction::Bcs, ArgumentType::Byte);
        m.insert(Instruction::Beq, ArgumentType::Byte);
        m.insert(Instruction::Bne, ArgumentType::Byte);
        m.insert(Instruction::Bmi, ArgumentType::Byte);
        m.insert(Instruction::Bpl, ArgumentType::Byte);
        m.insert(Instruction::Bvc, ArgumentType::Byte);
        m.insert(Instruction::Bvs, ArgumentType::Byte);

        m.insert(Instruction::BitZeroPage, ArgumentType::Byte);
        m.insert(Instruction::BitAbsolute, ArgumentType::Addr);

        m.insert(Instruction::Brk, ArgumentType::Void);

        m.insert(Instruction::Clc, ArgumentType::Void);
        m.insert(Instruction::Cld, ArgumentType::Void);
        m.insert(Instruction::Cli, ArgumentType::Void);
        m.insert(Instruction::Clv, ArgumentType::Void);

        m.insert(Instruction::CmpXIndexedZeroIndirect, ArgumentType::Byte);
        m.insert(Instruction::CmpZeroPage, ArgumentType::Byte);
        m.insert(Instruction::CmpImmediate, ArgumentType::Byte);
        m.insert(Instruction::CmpZeroIndirectIndexed, ArgumentType::Byte);
        m.insert(Instruction::CmpXIndexedZero, ArgumentType::Byte);
        m.insert(Instruction::CmpYIndexedAbsolute, ArgumentType::Addr);
        m.insert(Instruction::CmpXIndexedAbsolute, ArgumentType::Addr);
        m.insert(Instruction::CmpAbsolute, ArgumentType::Addr);

        m.insert(Instruction::CpxZeroPage, ArgumentType::Byte);
        m.insert(Instruction::CpxImmediate, ArgumentType::Byte);
        m.insert(Instruction::CpxAbsolute, ArgumentType::Addr);

        m.insert(Instruction::CpyZeroPage, ArgumentType::Byte);
        m.insert(Instruction::CpyImmediate, ArgumentType::Byte);
        m.insert(Instruction::CpyAbsolute, ArgumentType::Addr);

        m.insert(Instruction::DecAbsolute, ArgumentType::Addr);
        m.insert(Instruction::DecZeroPage, ArgumentType::Byte);
        m.insert(Instruction::DecXIndexedZero, ArgumentType::Byte);
        m.insert(Instruction::DecXIndexedAbsolute, ArgumentType::Addr);

        m.insert(Instruction::Dex, ArgumentType::Void);
        m.insert(Instruction::Dey, ArgumentType::Void);

        m.insert(Instruction::EorXIndexedZeroIndirect, ArgumentType::Byte);
        m.insert(Instruction::EorZeroPage, ArgumentType::Byte);
        m.insert(Instruction::EorImmediate, ArgumentType::Byte);
        m.insert(Instruction::EorAbsolute, ArgumentType::Addr);
        m.insert(Instruction::EorZeroIndirectIndexed, ArgumentType::Byte);
        m.insert(Instruction::EorXIndexedZero, ArgumentType::Byte);
        m.insert(Instruction::EorXIndexedAbsolute, ArgumentType::Addr);
        m.insert(Instruction::EorYIndexedAbsolute, ArgumentType::Addr);

        m.insert(Instruction::IncAbsolute, ArgumentType::Addr);
        m.insert(Instruction::IncZeroPage, ArgumentType::Byte);
        m.insert(Instruction::IncXIndexedZero, ArgumentType::Byte);
        m.insert(Instruction::IncXIndexedAbsolute, ArgumentType::Addr);

        m.insert(Instruction::Inx, ArgumentType::Void);
        m.insert(Instruction::Iny, ArgumentType::Void);

        m.insert(Instruction::Jmp, ArgumentType::Addr);
        m.insert(Instruction::JmpIndirect, ArgumentType::Addr);

        m.insert(Instruction::Jsr, ArgumentType::Addr);

        m.insert(Instruction::Nop, ArgumentType::Void);

        m.insert(Instruction::LdaXIndexedZeroIndirect, ArgumentType::Byte);
        m.insert(Instruction::LdaZeroPage, ArgumentType::Byte);
        m.insert(Instruction::LdaImmediate, ArgumentType::Byte);
        m.insert(Instruction::LdaAbsolute, ArgumentType::Addr);
        m.insert(Instruction::LdaZeroIndirectIndexed, ArgumentType::Byte);
        m.insert(Instruction::LdaXIndexedZero, ArgumentType::Byte);
        m.insert(Instruction::LdaXIndexedAbsolute, ArgumentType::Addr);
        m.insert(Instruction::LdaYIndexedAbsolute, ArgumentType::Addr);

        m
    };
}
