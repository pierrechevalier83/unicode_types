/// A number of constants to give a name to all characters in this block.
mod constants {
    /// \u{1c80}: 'ᲀ'
    pub const CYRILLIC_SMALL_LETTER_ROUNDED_VE: char = 'ᲀ';
    /// \u{1c81}: 'ᲁ'
    pub const CYRILLIC_SMALL_LETTER_LONG_DASH_LEGGED_DE: char = 'ᲁ';
    /// \u{1c82}: 'ᲂ'
    pub const CYRILLIC_SMALL_LETTER_NARROW_O: char = 'ᲂ';
    /// \u{1c83}: 'ᲃ'
    pub const CYRILLIC_SMALL_LETTER_WIDE_ES: char = 'ᲃ';
    /// \u{1c84}: 'ᲄ'
    pub const CYRILLIC_SMALL_LETTER_TALL_TE: char = 'ᲄ';
    /// \u{1c85}: 'ᲅ'
    pub const CYRILLIC_SMALL_LETTER_THREE_DASH_LEGGED_TE: char = 'ᲅ';
    /// \u{1c86}: 'ᲆ'
    pub const CYRILLIC_SMALL_LETTER_TALL_HARD_SIGN: char = 'ᲆ';
    /// \u{1c87}: 'ᲇ'
    pub const CYRILLIC_SMALL_LETTER_TALL_YAT: char = 'ᲇ';
    /// \u{1c88}: 'ᲈ'
    pub const CYRILLIC_SMALL_LETTER_UNBLENDED_UK: char = 'ᲈ';
}

/// An enum to represent all characters in the CyrillicExtendedC block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum CyrillicExtendedC {
    /// \u{1c80}: 'ᲀ'
    CyrillicSmallLetterRoundedVe,
    /// \u{1c81}: 'ᲁ'
    CyrillicSmallLetterLongDashLeggedDe,
    /// \u{1c82}: 'ᲂ'
    CyrillicSmallLetterNarrowO,
    /// \u{1c83}: 'ᲃ'
    CyrillicSmallLetterWideEs,
    /// \u{1c84}: 'ᲄ'
    CyrillicSmallLetterTallTe,
    /// \u{1c85}: 'ᲅ'
    CyrillicSmallLetterThreeDashLeggedTe,
    /// \u{1c86}: 'ᲆ'
    CyrillicSmallLetterTallHardSign,
    /// \u{1c87}: 'ᲇ'
    CyrillicSmallLetterTallYat,
    /// \u{1c88}: 'ᲈ'
    CyrillicSmallLetterUnblendedUk,
}

impl Into<char> for CyrillicExtendedC {
    fn into(self) -> char {
        use constants::*;
        match self {
            CyrillicExtendedC::CyrillicSmallLetterRoundedVe => CYRILLIC_SMALL_LETTER_ROUNDED_VE,
            CyrillicExtendedC::CyrillicSmallLetterLongDashLeggedDe => CYRILLIC_SMALL_LETTER_LONG_DASH_LEGGED_DE,
            CyrillicExtendedC::CyrillicSmallLetterNarrowO => CYRILLIC_SMALL_LETTER_NARROW_O,
            CyrillicExtendedC::CyrillicSmallLetterWideEs => CYRILLIC_SMALL_LETTER_WIDE_ES,
            CyrillicExtendedC::CyrillicSmallLetterTallTe => CYRILLIC_SMALL_LETTER_TALL_TE,
            CyrillicExtendedC::CyrillicSmallLetterThreeDashLeggedTe => CYRILLIC_SMALL_LETTER_THREE_DASH_LEGGED_TE,
            CyrillicExtendedC::CyrillicSmallLetterTallHardSign => CYRILLIC_SMALL_LETTER_TALL_HARD_SIGN,
            CyrillicExtendedC::CyrillicSmallLetterTallYat => CYRILLIC_SMALL_LETTER_TALL_YAT,
            CyrillicExtendedC::CyrillicSmallLetterUnblendedUk => CYRILLIC_SMALL_LETTER_UNBLENDED_UK,
        }
    }
}

impl std::convert::TryFrom<char> for CyrillicExtendedC {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            CYRILLIC_SMALL_LETTER_ROUNDED_VE => Ok(CyrillicExtendedC::CyrillicSmallLetterRoundedVe),
            CYRILLIC_SMALL_LETTER_LONG_DASH_LEGGED_DE => Ok(CyrillicExtendedC::CyrillicSmallLetterLongDashLeggedDe),
            CYRILLIC_SMALL_LETTER_NARROW_O => Ok(CyrillicExtendedC::CyrillicSmallLetterNarrowO),
            CYRILLIC_SMALL_LETTER_WIDE_ES => Ok(CyrillicExtendedC::CyrillicSmallLetterWideEs),
            CYRILLIC_SMALL_LETTER_TALL_TE => Ok(CyrillicExtendedC::CyrillicSmallLetterTallTe),
            CYRILLIC_SMALL_LETTER_THREE_DASH_LEGGED_TE => Ok(CyrillicExtendedC::CyrillicSmallLetterThreeDashLeggedTe),
            CYRILLIC_SMALL_LETTER_TALL_HARD_SIGN => Ok(CyrillicExtendedC::CyrillicSmallLetterTallHardSign),
            CYRILLIC_SMALL_LETTER_TALL_YAT => Ok(CyrillicExtendedC::CyrillicSmallLetterTallYat),
            CYRILLIC_SMALL_LETTER_UNBLENDED_UK => Ok(CyrillicExtendedC::CyrillicSmallLetterUnblendedUk),
            _ => Err(()),
        }
    }
}

impl Into<u32> for CyrillicExtendedC {
    fn into(self) -> u32 {
        let c: char = self.into();
        let hex = c
            .escape_unicode()
            .to_string()
            .replace("\\u{", "")
            .replace("}", "");
        u32::from_str_radix(&hex, 16).unwrap()
    }
}

impl std::convert::TryFrom<u32> for CyrillicExtendedC {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for CyrillicExtendedC {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl CyrillicExtendedC {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        CyrillicExtendedC::CyrillicSmallLetterRoundedVe
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            CyrillicExtendedC::CyrillicSmallLetterRoundedVe => "cyrillic small letter rounded ve",
            CyrillicExtendedC::CyrillicSmallLetterLongDashLeggedDe => "cyrillic small letter long-legged de",
            CyrillicExtendedC::CyrillicSmallLetterNarrowO => "cyrillic small letter narrow o",
            CyrillicExtendedC::CyrillicSmallLetterWideEs => "cyrillic small letter wide es",
            CyrillicExtendedC::CyrillicSmallLetterTallTe => "cyrillic small letter tall te",
            CyrillicExtendedC::CyrillicSmallLetterThreeDashLeggedTe => "cyrillic small letter three-legged te",
            CyrillicExtendedC::CyrillicSmallLetterTallHardSign => "cyrillic small letter tall hard sign",
            CyrillicExtendedC::CyrillicSmallLetterTallYat => "cyrillic small letter tall yat",
            CyrillicExtendedC::CyrillicSmallLetterUnblendedUk => "cyrillic small letter unblended uk",
        }
    }
}
