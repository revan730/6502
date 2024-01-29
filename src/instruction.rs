use crate::error::DecodeError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Instruction {
    AdcImmediate = 0x69,
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
            0x69 => Ok(Instruction::AdcImmediate),
            0x4C => Ok(Instruction::JMP),
            0xEA => Ok(Instruction::NOP),
            _ => Err(DecodeError::UnknownOpcodeError(value)),
        }
    }
}