pub struct FlagsRegister(u8);

pub enum FlagPosition {
    NEGATIVE = 7,
    OVERFLOW = 6,
    ZERO = 1,
    CARRY = 0,
}

impl Into<u8> for FlagPosition {
    fn into(self) -> u8 {
        self as u8
    }
}

impl Into<u8> for &FlagsRegister {
    fn into(self) -> u8 {
        self.0
    }
}

impl FlagsRegister {
    pub fn new() -> FlagsRegister {
        FlagsRegister(0)
    }

    pub fn write_flag(&mut self, flag: FlagPosition, set: bool) {
        if set {
            self.0 |= 1 << Into::<u8>::into(flag);
        } else {
            self.0 &= !(1 << Into::<u8>::into(flag));
        }
    }

    pub fn read_flag(&self, flag: FlagPosition) -> u8 {
        self.0 & 1 << Into::<u8>::into(flag)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_flag() {
        let mut flags = FlagsRegister(0);

        flags.write_flag(FlagPosition::NEGATIVE, true);
        assert_eq!(flags.0, 0b10000000);
        flags.write_flag(FlagPosition::NEGATIVE, false);
        assert_eq!(flags.0, 0);

        flags.write_flag(FlagPosition::OVERFLOW, true);
        assert_eq!(flags.0, 0b01000000);
        flags.write_flag(FlagPosition::OVERFLOW, false);
        assert_eq!(flags.0, 0);

        flags.write_flag(FlagPosition::ZERO, true);
        assert_eq!(flags.0, 0b00000010);
        flags.write_flag(FlagPosition::ZERO, false);
        assert_eq!(flags.0, 0);

        flags.write_flag(FlagPosition::CARRY, true);
        assert_eq!(flags.0, 0b00000001);
        flags.write_flag(FlagPosition::CARRY, false);
        assert_eq!(flags.0, 0);
    }

    #[test]
    fn read_flag() {
        let mut flags = FlagsRegister(0);

        flags.write_flag(FlagPosition::NEGATIVE, true);
        assert_eq!(flags.read_flag(FlagPosition::NEGATIVE), 0b10000000);
        flags.write_flag(FlagPosition::NEGATIVE, false);
        assert_eq!(flags.read_flag(FlagPosition::NEGATIVE), 0);

        flags.write_flag(FlagPosition::OVERFLOW, true);
        assert_eq!(flags.read_flag(FlagPosition::OVERFLOW), 0b01000000);
        flags.write_flag(FlagPosition::OVERFLOW, false);
        assert_eq!(flags.read_flag(FlagPosition::OVERFLOW), 0);

        flags.write_flag(FlagPosition::ZERO, true);
        assert_eq!(flags.read_flag(FlagPosition::ZERO), 0b00000010);
        flags.write_flag(FlagPosition::ZERO, false);
        assert_eq!(flags.read_flag(FlagPosition::ZERO), 0);

        flags.write_flag(FlagPosition::CARRY, true);
        assert_eq!(flags.read_flag(FlagPosition::CARRY), 0b00000001);
        flags.write_flag(FlagPosition::CARRY, false);
        assert_eq!(flags.read_flag(FlagPosition::CARRY), 0);
    }

    #[test]
    fn flags_into_u8() {
        let flags = FlagsRegister(0b10000001);
        assert_eq!(Into::<u8>::into(&flags), 0b10000001);
    }
}
