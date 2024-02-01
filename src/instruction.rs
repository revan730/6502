use crate::error::DecodeError;

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
    AndImmediate = 0x29,
    AndAbsolute = 0x2D,
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
            0x29 => Ok(Instruction::AndImmediate),
            0x2D => Ok(Instruction::AndAbsolute),
            0x4C => Ok(Instruction::JMP),
            0xEA => Ok(Instruction::NOP),
            _ => Err(DecodeError::UnknownOpcodeError(format!("{:#X}", value))),
        }
    }
}
