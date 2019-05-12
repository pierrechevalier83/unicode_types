
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
        match self {
            CyrillicExtendedC::CyrillicSmallLetterRoundedVe => 'ᲀ',
            CyrillicExtendedC::CyrillicSmallLetterLongDashLeggedDe => 'ᲁ',
            CyrillicExtendedC::CyrillicSmallLetterNarrowO => 'ᲂ',
            CyrillicExtendedC::CyrillicSmallLetterWideEs => 'ᲃ',
            CyrillicExtendedC::CyrillicSmallLetterTallTe => 'ᲄ',
            CyrillicExtendedC::CyrillicSmallLetterThreeDashLeggedTe => 'ᲅ',
            CyrillicExtendedC::CyrillicSmallLetterTallHardSign => 'ᲆ',
            CyrillicExtendedC::CyrillicSmallLetterTallYat => 'ᲇ',
            CyrillicExtendedC::CyrillicSmallLetterUnblendedUk => 'ᲈ',
        }
    }
}

impl std::convert::TryFrom<char> for CyrillicExtendedC {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ᲀ' => Ok(CyrillicExtendedC::CyrillicSmallLetterRoundedVe),
            'ᲁ' => Ok(CyrillicExtendedC::CyrillicSmallLetterLongDashLeggedDe),
            'ᲂ' => Ok(CyrillicExtendedC::CyrillicSmallLetterNarrowO),
            'ᲃ' => Ok(CyrillicExtendedC::CyrillicSmallLetterWideEs),
            'ᲄ' => Ok(CyrillicExtendedC::CyrillicSmallLetterTallTe),
            'ᲅ' => Ok(CyrillicExtendedC::CyrillicSmallLetterThreeDashLeggedTe),
            'ᲆ' => Ok(CyrillicExtendedC::CyrillicSmallLetterTallHardSign),
            'ᲇ' => Ok(CyrillicExtendedC::CyrillicSmallLetterTallYat),
            'ᲈ' => Ok(CyrillicExtendedC::CyrillicSmallLetterUnblendedUk),
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
