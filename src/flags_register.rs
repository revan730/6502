pub struct FlagsRegister(u8);

pub enum FlagPosition {
    Negative = 7,
    Overflow = 6,
    Zero = 1,
    Carry = 0,
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

impl Default for FlagsRegister {
    fn default() -> Self {
        Self::new()
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

        flags.write_flag(FlagPosition::Negative, true);
        assert_eq!(flags.0, 0b10000000);
        flags.write_flag(FlagPosition::Negative, false);
        assert_eq!(flags.0, 0);

        flags.write_flag(FlagPosition::Overflow, true);
        assert_eq!(flags.0, 0b01000000);
        flags.write_flag(FlagPosition::Overflow, false);
        assert_eq!(flags.0, 0);

        flags.write_flag(FlagPosition::Zero, true);
        assert_eq!(flags.0, 0b00000010);
        flags.write_flag(FlagPosition::Zero, false);
        assert_eq!(flags.0, 0);

        flags.write_flag(FlagPosition::Carry, true);
        assert_eq!(flags.0, 0b00000001);
        flags.write_flag(FlagPosition::Carry, false);
        assert_eq!(flags.0, 0);
    }

    #[test]
    fn read_flag() {
        let mut flags = FlagsRegister(0);

        flags.write_flag(FlagPosition::Negative, true);
        assert_eq!(flags.read_flag(FlagPosition::Negative), 0b10000000);
        flags.write_flag(FlagPosition::Negative, false);
        assert_eq!(flags.read_flag(FlagPosition::Negative), 0);

        flags.write_flag(FlagPosition::Overflow, true);
        assert_eq!(flags.read_flag(FlagPosition::Overflow), 0b01000000);
        flags.write_flag(FlagPosition::Overflow, false);
        assert_eq!(flags.read_flag(FlagPosition::Overflow), 0);

        flags.write_flag(FlagPosition::Zero, true);
        assert_eq!(flags.read_flag(FlagPosition::Zero), 0b00000010);
        flags.write_flag(FlagPosition::Zero, false);
        assert_eq!(flags.read_flag(FlagPosition::Zero), 0);

        flags.write_flag(FlagPosition::Carry, true);
        assert_eq!(flags.read_flag(FlagPosition::Carry), 0b00000001);
        flags.write_flag(FlagPosition::Carry, false);
        assert_eq!(flags.read_flag(FlagPosition::Carry), 0);
    }

    #[test]
    fn flags_into_u8() {
        let flags = FlagsRegister(0b10000001);
        assert_eq!(Into::<u8>::into(&flags), 0b10000001);
    }
}
