use crate::error::DecodeError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AddressingType {
    XIndexedZeroIndirect,
    ZeroPage,
    Immediate,
    Absolute,
    ZeroIndirectIndexed,
    XIndexedZero,
    XIndexedAbsolute,
    YIndexedAbsolute,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Instruction {
    AdcXIndexedZeroIndirect = 0x61,
    AdcZeroPage = 0x65,
    AdcImmediate = 0x69,
    AdcAbsolute = 0x6D,
    AdcZeroIndirectIndexed = 0x71,
    AdcXIndexedZero = 0x75,
    AdcYIndexedAbsolute = 0x79,
    AdcXIndexedAbsolute = 0x7D,
    AndXIndexedZeroIndirect = 0x21,
    AndZeroPage = 0x25,
    AndImmediate = 0x29,
    AndAbsolute = 0x2D,
    AndZeroIndirectIndexed = 0x31,
    AndXIndexedZero = 0x35,
    AndYIndexedAbsolute = 0x39,
    AndXIndexedAbsolute = 0x3D,
    JMP = 0x4C,
    NOP = 0xEA,
}

impl Into<u8> for Instruction {
    fn into(self) -> u8 {
        self as u8
    }
}

impl TryFrom<u8> for Instruction {
    type Error = DecodeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x61 => Ok(Instruction::AdcXIndexedZeroIndirect),
            0x65 => Ok(Instruction::AdcZeroPage),
            0x69 => Ok(Instruction::AdcImmediate),
            0x6D => Ok(Instruction::AdcAbsolute),
            0x71 => Ok(Instruction::AdcZeroIndirectIndexed),
            0x75 => Ok(Instruction::AdcXIndexedZero),
            0x79 => Ok(Instruction::AdcYIndexedAbsolute),
            0x7D => Ok(Instruction::AdcXIndexedAbsolute),
            0x21 => Ok(Instruction::AndXIndexedZeroIndirect),
            0x25 => Ok(Instruction::AndZeroPage),
            0x29 => Ok(Instruction::AndImmediate),
            0x2D => Ok(Instruction::AndAbsolute),
            0x31 => Ok(Instruction::AndZeroIndirectIndexed),
            0x35 => Ok(Instruction::AndXIndexedZero),
            0x39 => Ok(Instruction::AndYIndexedAbsolute),
            0x3D => Ok(Instruction::AndXIndexedAbsolute),
            0x4C => Ok(Instruction::JMP),
            0xEA => Ok(Instruction::NOP),
            _ => Err(DecodeError::UnknownOpcodeError(format!("{:#X}", value))),
        }
    }
}
