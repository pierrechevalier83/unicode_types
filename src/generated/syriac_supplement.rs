/// \u{860} → \u{86f}\
///\
/// ࡠ ࡡ ࡢ ࡣ ࡤ ࡥ ࡦ ࡧ ࡨ ࡩ ࡪ\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{860}: 'ࡠ'
    pub const SYRIAC_LETTER_MALAYALAM_NGA: char = 'ࡠ';
    /// \u{861}: 'ࡡ'
    pub const SYRIAC_LETTER_MALAYALAM_JA: char = 'ࡡ';
    /// \u{862}: 'ࡢ'
    pub const SYRIAC_LETTER_MALAYALAM_NYA: char = 'ࡢ';
    /// \u{863}: 'ࡣ'
    pub const SYRIAC_LETTER_MALAYALAM_TTA: char = 'ࡣ';
    /// \u{864}: 'ࡤ'
    pub const SYRIAC_LETTER_MALAYALAM_NNA: char = 'ࡤ';
    /// \u{865}: 'ࡥ'
    pub const SYRIAC_LETTER_MALAYALAM_NNNA: char = 'ࡥ';
    /// \u{866}: 'ࡦ'
    pub const SYRIAC_LETTER_MALAYALAM_BHA: char = 'ࡦ';
    /// \u{867}: 'ࡧ'
    pub const SYRIAC_LETTER_MALAYALAM_RA: char = 'ࡧ';
    /// \u{868}: 'ࡨ'
    pub const SYRIAC_LETTER_MALAYALAM_LLA: char = 'ࡨ';
    /// \u{869}: 'ࡩ'
    pub const SYRIAC_LETTER_MALAYALAM_LLLA: char = 'ࡩ';
    /// \u{86a}: 'ࡪ'
    pub const SYRIAC_LETTER_MALAYALAM_SSA: char = 'ࡪ';
}

/// An enum to represent all characters in the SyriacSupplement block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SyriacSupplement {
    /// \u{860}: 'ࡠ'
    SyriacLetterMalayalamNga,
    /// \u{861}: 'ࡡ'
    SyriacLetterMalayalamJa,
    /// \u{862}: 'ࡢ'
    SyriacLetterMalayalamNya,
    /// \u{863}: 'ࡣ'
    SyriacLetterMalayalamTta,
    /// \u{864}: 'ࡤ'
    SyriacLetterMalayalamNna,
    /// \u{865}: 'ࡥ'
    SyriacLetterMalayalamNnna,
    /// \u{866}: 'ࡦ'
    SyriacLetterMalayalamBha,
    /// \u{867}: 'ࡧ'
    SyriacLetterMalayalamRa,
    /// \u{868}: 'ࡨ'
    SyriacLetterMalayalamLla,
    /// \u{869}: 'ࡩ'
    SyriacLetterMalayalamLlla,
    /// \u{86a}: 'ࡪ'
    SyriacLetterMalayalamSsa,
}

impl Into<char> for SyriacSupplement {
    fn into(self) -> char {
        use constants::*;
        match self {
            SyriacSupplement::SyriacLetterMalayalamNga => SYRIAC_LETTER_MALAYALAM_NGA,
            SyriacSupplement::SyriacLetterMalayalamJa => SYRIAC_LETTER_MALAYALAM_JA,
            SyriacSupplement::SyriacLetterMalayalamNya => SYRIAC_LETTER_MALAYALAM_NYA,
            SyriacSupplement::SyriacLetterMalayalamTta => SYRIAC_LETTER_MALAYALAM_TTA,
            SyriacSupplement::SyriacLetterMalayalamNna => SYRIAC_LETTER_MALAYALAM_NNA,
            SyriacSupplement::SyriacLetterMalayalamNnna => SYRIAC_LETTER_MALAYALAM_NNNA,
            SyriacSupplement::SyriacLetterMalayalamBha => SYRIAC_LETTER_MALAYALAM_BHA,
            SyriacSupplement::SyriacLetterMalayalamRa => SYRIAC_LETTER_MALAYALAM_RA,
            SyriacSupplement::SyriacLetterMalayalamLla => SYRIAC_LETTER_MALAYALAM_LLA,
            SyriacSupplement::SyriacLetterMalayalamLlla => SYRIAC_LETTER_MALAYALAM_LLLA,
            SyriacSupplement::SyriacLetterMalayalamSsa => SYRIAC_LETTER_MALAYALAM_SSA,
        }
    }
}

impl std::convert::TryFrom<char> for SyriacSupplement {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            SYRIAC_LETTER_MALAYALAM_NGA => Ok(SyriacSupplement::SyriacLetterMalayalamNga),
            SYRIAC_LETTER_MALAYALAM_JA => Ok(SyriacSupplement::SyriacLetterMalayalamJa),
            SYRIAC_LETTER_MALAYALAM_NYA => Ok(SyriacSupplement::SyriacLetterMalayalamNya),
            SYRIAC_LETTER_MALAYALAM_TTA => Ok(SyriacSupplement::SyriacLetterMalayalamTta),
            SYRIAC_LETTER_MALAYALAM_NNA => Ok(SyriacSupplement::SyriacLetterMalayalamNna),
            SYRIAC_LETTER_MALAYALAM_NNNA => Ok(SyriacSupplement::SyriacLetterMalayalamNnna),
            SYRIAC_LETTER_MALAYALAM_BHA => Ok(SyriacSupplement::SyriacLetterMalayalamBha),
            SYRIAC_LETTER_MALAYALAM_RA => Ok(SyriacSupplement::SyriacLetterMalayalamRa),
            SYRIAC_LETTER_MALAYALAM_LLA => Ok(SyriacSupplement::SyriacLetterMalayalamLla),
            SYRIAC_LETTER_MALAYALAM_LLLA => Ok(SyriacSupplement::SyriacLetterMalayalamLlla),
            SYRIAC_LETTER_MALAYALAM_SSA => Ok(SyriacSupplement::SyriacLetterMalayalamSsa),
            _ => Err(()),
        }
    }
}

impl Into<u32> for SyriacSupplement {
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

impl std::convert::TryFrom<u32> for SyriacSupplement {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for SyriacSupplement {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl SyriacSupplement {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        SyriacSupplement::SyriacLetterMalayalamNga
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            SyriacSupplement::SyriacLetterMalayalamNga => "syriac letter malayalam nga",
            SyriacSupplement::SyriacLetterMalayalamJa => "syriac letter malayalam ja",
            SyriacSupplement::SyriacLetterMalayalamNya => "syriac letter malayalam nya",
            SyriacSupplement::SyriacLetterMalayalamTta => "syriac letter malayalam tta",
            SyriacSupplement::SyriacLetterMalayalamNna => "syriac letter malayalam nna",
            SyriacSupplement::SyriacLetterMalayalamNnna => "syriac letter malayalam nnna",
            SyriacSupplement::SyriacLetterMalayalamBha => "syriac letter malayalam bha",
            SyriacSupplement::SyriacLetterMalayalamRa => "syriac letter malayalam ra",
            SyriacSupplement::SyriacLetterMalayalamLla => "syriac letter malayalam lla",
            SyriacSupplement::SyriacLetterMalayalamLlla => "syriac letter malayalam llla",
            SyriacSupplement::SyriacLetterMalayalamSsa => "syriac letter malayalam ssa",
        }
    }
}
