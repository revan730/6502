use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AddressingType {
    XIndexedZeroIndirect,
    ZeroPage,
    Immediate,
    Absolute,
    ZeroIndirectIndexed,
    XIndexedZero,
    YIndexedZero,
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

    Brk = 0x00,

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
    JmpIndirect = 0x6C,

    Jsr = 0x20,

    Nop = 0xEA,

    LdaXIndexedZeroIndirect = 0xA1,
    LdaZeroPage = 0xA5,
    LdaImmediate = 0xA9,
    LdaAbsolute = 0xAD,
    LdaZeroIndirectIndexed = 0xB1,
    LdaXIndexedZero = 0xB5,
    LdaYIndexedAbsolute = 0xB9,
    LdaXIndexedAbsolute = 0xBD,

    LdxZeroPage = 0xA6,
    LdxImmediate = 0xA2,
    LdxAbsolute = 0xAE,
    LdxYIndexedAbsolute = 0xBE,
    LdxYIndexedZero = 0xB6,

    LdyZeroPage = 0xA4,
    LdyImmediate = 0xA0,
    LdyAbsolute = 0xAC,
    LdyXIndexedAbsolute = 0xBC,
    LdyXIndexedZero = 0xB4,

    LsrAbsolute = 0x4E,
    LsrZeroPage = 0x46,
    LsrAccumulator = 0x4A,
    LsrXIndexedZero = 0x56,
    LsrXIndexedAbsolute = 0x5E,

    OraXIndexedZeroIndirect = 0x01,
    OraZeroPage = 0x05,
    OraImmediate = 0x09,
    OraAbsolute = 0x0D,
    OraZeroIndirectIndexed = 0x11,
    OraXIndexedZero = 0x15,
    OraYIndexedAbsolute = 0x19,
    OraXIndexedAbsolute = 0x1D,

    Pha = 0x48,
    Php = 0x08,
    Pla = 0x68,
    Plp = 0x28,

    RolAbsolute = 0x2E,
    RolZeroPage = 0x26,
    RolAccumulator = 0x2A,
    RolXIndexedZero = 0x36,
    RolXIndexedAbsolute = 0x3E,

    RorAbsolute = 0x6E,
    RorZeroPage = 0x66,
    RorAccumulator = 0x6A,
    RorXIndexedZero = 0x76,
    RorXIndexedAbsolute = 0x7E,

    Rti = 0x40,

    Rts = 0x60,
}
