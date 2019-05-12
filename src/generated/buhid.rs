/// \u{1740} → \u{175f}
///
/// ᝀ ᝁ ᝂ ᝃ ᝄ ᝅ ᝆ ᝇ ᝈ ᝉ ᝊ ᝋ ᝌ ᝍ ᝎ ᝏ\
/// ᝐ ᝑ ᝒ ᝓ\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{1740}: 'ᝀ'
    pub const LETTER_A: char = 'ᝀ';
    /// \u{1741}: 'ᝁ'
    pub const LETTER_I: char = 'ᝁ';
    /// \u{1742}: 'ᝂ'
    pub const LETTER_U: char = 'ᝂ';
    /// \u{1743}: 'ᝃ'
    pub const LETTER_KA: char = 'ᝃ';
    /// \u{1744}: 'ᝄ'
    pub const LETTER_GA: char = 'ᝄ';
    /// \u{1745}: 'ᝅ'
    pub const LETTER_NGA: char = 'ᝅ';
    /// \u{1746}: 'ᝆ'
    pub const LETTER_TA: char = 'ᝆ';
    /// \u{1747}: 'ᝇ'
    pub const LETTER_DA: char = 'ᝇ';
    /// \u{1748}: 'ᝈ'
    pub const LETTER_NA: char = 'ᝈ';
    /// \u{1749}: 'ᝉ'
    pub const LETTER_PA: char = 'ᝉ';
    /// \u{174a}: 'ᝊ'
    pub const LETTER_BA: char = 'ᝊ';
    /// \u{174b}: 'ᝋ'
    pub const LETTER_MA: char = 'ᝋ';
    /// \u{174c}: 'ᝌ'
    pub const LETTER_YA: char = 'ᝌ';
    /// \u{174d}: 'ᝍ'
    pub const LETTER_RA: char = 'ᝍ';
    /// \u{174e}: 'ᝎ'
    pub const LETTER_LA: char = 'ᝎ';
    /// \u{174f}: 'ᝏ'
    pub const LETTER_WA: char = 'ᝏ';
    /// \u{1750}: 'ᝐ'
    pub const LETTER_SA: char = 'ᝐ';
    /// \u{1751}: 'ᝑ'
    pub const LETTER_HA: char = 'ᝑ';
    /// \u{1752}: 'ᝒ'
    pub const VOWEL_SIGN_I: char = 'ᝒ';
    /// \u{1753}: 'ᝓ'
    pub const VOWEL_SIGN_U: char = 'ᝓ';
}

/// An enum to represent all characters in the Buhid block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Buhid {
    /// \u{1740}: 'ᝀ'
    LetterA,
    /// \u{1741}: 'ᝁ'
    LetterI,
    /// \u{1742}: 'ᝂ'
    LetterU,
    /// \u{1743}: 'ᝃ'
    LetterKa,
    /// \u{1744}: 'ᝄ'
    LetterGa,
    /// \u{1745}: 'ᝅ'
    LetterNga,
    /// \u{1746}: 'ᝆ'
    LetterTa,
    /// \u{1747}: 'ᝇ'
    LetterDa,
    /// \u{1748}: 'ᝈ'
    LetterNa,
    /// \u{1749}: 'ᝉ'
    LetterPa,
    /// \u{174a}: 'ᝊ'
    LetterBa,
    /// \u{174b}: 'ᝋ'
    LetterMa,
    /// \u{174c}: 'ᝌ'
    LetterYa,
    /// \u{174d}: 'ᝍ'
    LetterRa,
    /// \u{174e}: 'ᝎ'
    LetterLa,
    /// \u{174f}: 'ᝏ'
    LetterWa,
    /// \u{1750}: 'ᝐ'
    LetterSa,
    /// \u{1751}: 'ᝑ'
    LetterHa,
    /// \u{1752}: 'ᝒ'
    VowelSignI,
    /// \u{1753}: 'ᝓ'
    VowelSignU,
}

impl Into<char> for Buhid {
    fn into(self) -> char {
        use constants::*;
        match self {
            Buhid::LetterA => LETTER_A,
            Buhid::LetterI => LETTER_I,
            Buhid::LetterU => LETTER_U,
            Buhid::LetterKa => LETTER_KA,
            Buhid::LetterGa => LETTER_GA,
            Buhid::LetterNga => LETTER_NGA,
            Buhid::LetterTa => LETTER_TA,
            Buhid::LetterDa => LETTER_DA,
            Buhid::LetterNa => LETTER_NA,
            Buhid::LetterPa => LETTER_PA,
            Buhid::LetterBa => LETTER_BA,
            Buhid::LetterMa => LETTER_MA,
            Buhid::LetterYa => LETTER_YA,
            Buhid::LetterRa => LETTER_RA,
            Buhid::LetterLa => LETTER_LA,
            Buhid::LetterWa => LETTER_WA,
            Buhid::LetterSa => LETTER_SA,
            Buhid::LetterHa => LETTER_HA,
            Buhid::VowelSignI => VOWEL_SIGN_I,
            Buhid::VowelSignU => VOWEL_SIGN_U,
        }
    }
}

impl std::convert::TryFrom<char> for Buhid {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_A => Ok(Buhid::LetterA),
            LETTER_I => Ok(Buhid::LetterI),
            LETTER_U => Ok(Buhid::LetterU),
            LETTER_KA => Ok(Buhid::LetterKa),
            LETTER_GA => Ok(Buhid::LetterGa),
            LETTER_NGA => Ok(Buhid::LetterNga),
            LETTER_TA => Ok(Buhid::LetterTa),
            LETTER_DA => Ok(Buhid::LetterDa),
            LETTER_NA => Ok(Buhid::LetterNa),
            LETTER_PA => Ok(Buhid::LetterPa),
            LETTER_BA => Ok(Buhid::LetterBa),
            LETTER_MA => Ok(Buhid::LetterMa),
            LETTER_YA => Ok(Buhid::LetterYa),
            LETTER_RA => Ok(Buhid::LetterRa),
            LETTER_LA => Ok(Buhid::LetterLa),
            LETTER_WA => Ok(Buhid::LetterWa),
            LETTER_SA => Ok(Buhid::LetterSa),
            LETTER_HA => Ok(Buhid::LetterHa),
            VOWEL_SIGN_I => Ok(Buhid::VowelSignI),
            VOWEL_SIGN_U => Ok(Buhid::VowelSignU),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Buhid {
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

impl std::convert::TryFrom<u32> for Buhid {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Buhid {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Buhid {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Buhid::LetterA
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Buhid::LetterA => "buhid letter a",
            Buhid::LetterI => "buhid letter i",
            Buhid::LetterU => "buhid letter u",
            Buhid::LetterKa => "buhid letter ka",
            Buhid::LetterGa => "buhid letter ga",
            Buhid::LetterNga => "buhid letter nga",
            Buhid::LetterTa => "buhid letter ta",
            Buhid::LetterDa => "buhid letter da",
            Buhid::LetterNa => "buhid letter na",
            Buhid::LetterPa => "buhid letter pa",
            Buhid::LetterBa => "buhid letter ba",
            Buhid::LetterMa => "buhid letter ma",
            Buhid::LetterYa => "buhid letter ya",
            Buhid::LetterRa => "buhid letter ra",
            Buhid::LetterLa => "buhid letter la",
            Buhid::LetterWa => "buhid letter wa",
            Buhid::LetterSa => "buhid letter sa",
            Buhid::LetterHa => "buhid letter ha",
            Buhid::VowelSignI => "buhid vowel sign i",
            Buhid::VowelSignU => "buhid vowel sign u",
        }
    }
}
