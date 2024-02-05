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

    BitZeroPage = 0x24,
    BitAbsolute = 0x2C,

    Clc = 0x18,
    Cld = 0xD8,
    Cli = 0x58,
    Clv = 0xB8,

    CmpXIndexedZeroIndirect = 0xC1,
    CmpZeroPage = 0xC5,
    CmpImmediate = 0xC9,
    CmpAbsolute = 0xCD,
    CmpZeroIndirectIndexed = 0xD1,
    CmpXIndexedZero = 0xD5,
    CmpYIndexedAbsolute = 0xD9,
    CmpXIndexedAbsolute = 0xDD,

    CpxZeroPage = 0xE4,
    CpxImmediate = 0xE0,
    CpxAbsolute = 0xEC,

    CpyZeroPage = 0xC4,
    CpyImmediate = 0xC0,
    CpyAbsolute = 0xCC,

    DecZeroPage = 0xC6,
    DecAbsolute = 0xCE,
    DecXIndexedZero = 0xD6,
    DecXIndexedAbsolute = 0xDE,

    Dex = 0xCA,
    Dey = 0x88,

    IncZeroPage = 0xE6,
    IncAbsolute = 0xEE,
    IncXIndexedZero = 0xF6,
    IncXIndexedAbsolute = 0xFE,

    Inx = 0xE8,
    Iny = 0xC8,

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

            0x24 => Ok(Instruction::BitZeroPage),
            0x2C => Ok(Instruction::BitAbsolute),

            0x18 => Ok(Instruction::Clc),
            0xD8 => Ok(Instruction::Cld),
            0x58 => Ok(Instruction::Cli),
            0xB8 => Ok(Instruction::Clv),

            0xC1 => Ok(Instruction::CmpXIndexedZeroIndirect),
            0xC5 => Ok(Instruction::CmpZeroPage),
            0xC9 => Ok(Instruction::CmpImmediate),
            0xCD => Ok(Instruction::CmpAbsolute),
            0xD1 => Ok(Instruction::CmpZeroIndirectIndexed),
            0xD5 => Ok(Instruction::CmpXIndexedZero),
            0xD9 => Ok(Instruction::CmpYIndexedAbsolute),
            0xDD => Ok(Instruction::CmpXIndexedAbsolute),

            0xE4 => Ok(Instruction::CpxZeroPage),
            0xE0 => Ok(Instruction::CpxImmediate),
            0xEC => Ok(Instruction::CpxAbsolute),

            0xC4 => Ok(Instruction::CpyZeroPage),
            0xC0 => Ok(Instruction::CpyImmediate),
            0xCC => Ok(Instruction::CpyAbsolute),

            0xCE => Ok(Instruction::DecAbsolute),
            0xC6 => Ok(Instruction::DecZeroPage),
            0xD6 => Ok(Instruction::DecXIndexedZero),
            0xDE => Ok(Instruction::DecXIndexedAbsolute),

            0xCA => Ok(Instruction::Dex),
            0x88 => Ok(Instruction::Dey),

            0xEE => Ok(Instruction::IncAbsolute),
            0xE6 => Ok(Instruction::IncZeroPage),
            0xF6 => Ok(Instruction::IncXIndexedZero),
            0xFE => Ok(Instruction::IncXIndexedAbsolute),

            0xE8 => Ok(Instruction::Inx),
            0xC8 => Ok(Instruction::Iny),

            0x4C => Ok(Instruction::Jmp),
            0xEA => Ok(Instruction::Nop),
            _ => Err(DecodeError::UnknownOpcode(format!("{value:#X}"))),
        }
    }
}
