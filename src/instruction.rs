use num_enum::{IntoPrimitive, TryFromPrimitive};

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

#[derive(IntoPrimitive, TryFromPrimitive, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
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

    Bcc = 0x90,
    Bcs = 0xB0,
    Beq = 0xF0,
    Bne = 0xD0,
    Bmi = 0x30,
    Bpl = 0x10,
    Bvc = 0x50,
    Bvs = 0x70,

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

    EorXIndexedZeroIndirect = 0x41,
    EorZeroPage = 0x45,
    EorImmediate = 0x49,
    EorAbsolute = 0x4D,
    EorZeroIndirectIndexed = 0x51,
    EorXIndexedZero = 0x55,
    EorYIndexedAbsolute = 0x59,
    EorXIndexedAbsolute = 0x5D,

    IncZeroPage = 0xE6,
    IncAbsolute = 0xEE,
    IncXIndexedZero = 0xF6,
    IncXIndexedAbsolute = 0xFE,

    Inx = 0xE8,
    Iny = 0xC8,

    Jmp = 0x4C,

    Nop = 0xEA,
}
