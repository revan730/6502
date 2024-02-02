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
    AslAbsolute = 0x0E,
    AslZeroPage = 0x06,
    AslAccumulator = 0x0A,
    AslXIndexedZero = 0x16,
    AslXIndexedAbsolute = 0x1E,
    Jmp = 0x4C,
    Nop = 0xEA,
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
            0x0E => Ok(Instruction::AslAbsolute),
            0x06 => Ok(Instruction::AslZeroPage),
            0x0A => Ok(Instruction::AslAccumulator),
            0x16 => Ok(Instruction::AslXIndexedZero),
            0x1E => Ok(Instruction::AslXIndexedAbsolute),
            0x4C => Ok(Instruction::Jmp),
            0xEA => Ok(Instruction::Nop),
            _ => Err(DecodeError::UnknownOpcode(format!("{value:#X}"))),
        }
    }
}
