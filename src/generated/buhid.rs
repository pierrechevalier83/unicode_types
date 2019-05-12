/// \u{1740} → \u{175f}\
///\
/// ᝀ ᝁ ᝂ ᝃ ᝄ ᝅ ᝆ ᝇ ᝈ ᝉ ᝊ ᝋ ᝌ ᝍ ᝎ ᝏ
/// ᝐ ᝑ ᝒ ᝓ
pub mod constants {
    /// \u{1740}: 'ᝀ'
    pub const BUHID_LETTER_A: char = 'ᝀ';
    /// \u{1741}: 'ᝁ'
    pub const BUHID_LETTER_I: char = 'ᝁ';
    /// \u{1742}: 'ᝂ'
    pub const BUHID_LETTER_U: char = 'ᝂ';
    /// \u{1743}: 'ᝃ'
    pub const BUHID_LETTER_KA: char = 'ᝃ';
    /// \u{1744}: 'ᝄ'
    pub const BUHID_LETTER_GA: char = 'ᝄ';
    /// \u{1745}: 'ᝅ'
    pub const BUHID_LETTER_NGA: char = 'ᝅ';
    /// \u{1746}: 'ᝆ'
    pub const BUHID_LETTER_TA: char = 'ᝆ';
    /// \u{1747}: 'ᝇ'
    pub const BUHID_LETTER_DA: char = 'ᝇ';
    /// \u{1748}: 'ᝈ'
    pub const BUHID_LETTER_NA: char = 'ᝈ';
    /// \u{1749}: 'ᝉ'
    pub const BUHID_LETTER_PA: char = 'ᝉ';
    /// \u{174a}: 'ᝊ'
    pub const BUHID_LETTER_BA: char = 'ᝊ';
    /// \u{174b}: 'ᝋ'
    pub const BUHID_LETTER_MA: char = 'ᝋ';
    /// \u{174c}: 'ᝌ'
    pub const BUHID_LETTER_YA: char = 'ᝌ';
    /// \u{174d}: 'ᝍ'
    pub const BUHID_LETTER_RA: char = 'ᝍ';
    /// \u{174e}: 'ᝎ'
    pub const BUHID_LETTER_LA: char = 'ᝎ';
    /// \u{174f}: 'ᝏ'
    pub const BUHID_LETTER_WA: char = 'ᝏ';
    /// \u{1750}: 'ᝐ'
    pub const BUHID_LETTER_SA: char = 'ᝐ';
    /// \u{1751}: 'ᝑ'
    pub const BUHID_LETTER_HA: char = 'ᝑ';
    /// \u{1752}: 'ᝒ'
    pub const BUHID_VOWEL_SIGN_I: char = 'ᝒ';
    /// \u{1753}: 'ᝓ'
    pub const BUHID_VOWEL_SIGN_U: char = 'ᝓ';
}

/// \u{1740} → \u{175f}\
///\
/// ᝀ ᝁ ᝂ ᝃ ᝄ ᝅ ᝆ ᝇ ᝈ ᝉ ᝊ ᝋ ᝌ ᝍ ᝎ ᝏ
/// ᝐ ᝑ ᝒ ᝓ
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Buhid {
    /// \u{1740}: 'ᝀ'
    BuhidLetterA,
    /// \u{1741}: 'ᝁ'
    BuhidLetterI,
    /// \u{1742}: 'ᝂ'
    BuhidLetterU,
    /// \u{1743}: 'ᝃ'
    BuhidLetterKa,
    /// \u{1744}: 'ᝄ'
    BuhidLetterGa,
    /// \u{1745}: 'ᝅ'
    BuhidLetterNga,
    /// \u{1746}: 'ᝆ'
    BuhidLetterTa,
    /// \u{1747}: 'ᝇ'
    BuhidLetterDa,
    /// \u{1748}: 'ᝈ'
    BuhidLetterNa,
    /// \u{1749}: 'ᝉ'
    BuhidLetterPa,
    /// \u{174a}: 'ᝊ'
    BuhidLetterBa,
    /// \u{174b}: 'ᝋ'
    BuhidLetterMa,
    /// \u{174c}: 'ᝌ'
    BuhidLetterYa,
    /// \u{174d}: 'ᝍ'
    BuhidLetterRa,
    /// \u{174e}: 'ᝎ'
    BuhidLetterLa,
    /// \u{174f}: 'ᝏ'
    BuhidLetterWa,
    /// \u{1750}: 'ᝐ'
    BuhidLetterSa,
    /// \u{1751}: 'ᝑ'
    BuhidLetterHa,
    /// \u{1752}: 'ᝒ'
    BuhidVowelSignI,
    /// \u{1753}: 'ᝓ'
    BuhidVowelSignU,
}

impl Into<char> for Buhid {
    fn into(self) -> char {
        use constants::*;
        match self {
            Buhid::BuhidLetterA => BUHID_LETTER_A,
            Buhid::BuhidLetterI => BUHID_LETTER_I,
            Buhid::BuhidLetterU => BUHID_LETTER_U,
            Buhid::BuhidLetterKa => BUHID_LETTER_KA,
            Buhid::BuhidLetterGa => BUHID_LETTER_GA,
            Buhid::BuhidLetterNga => BUHID_LETTER_NGA,
            Buhid::BuhidLetterTa => BUHID_LETTER_TA,
            Buhid::BuhidLetterDa => BUHID_LETTER_DA,
            Buhid::BuhidLetterNa => BUHID_LETTER_NA,
            Buhid::BuhidLetterPa => BUHID_LETTER_PA,
            Buhid::BuhidLetterBa => BUHID_LETTER_BA,
            Buhid::BuhidLetterMa => BUHID_LETTER_MA,
            Buhid::BuhidLetterYa => BUHID_LETTER_YA,
            Buhid::BuhidLetterRa => BUHID_LETTER_RA,
            Buhid::BuhidLetterLa => BUHID_LETTER_LA,
            Buhid::BuhidLetterWa => BUHID_LETTER_WA,
            Buhid::BuhidLetterSa => BUHID_LETTER_SA,
            Buhid::BuhidLetterHa => BUHID_LETTER_HA,
            Buhid::BuhidVowelSignI => BUHID_VOWEL_SIGN_I,
            Buhid::BuhidVowelSignU => BUHID_VOWEL_SIGN_U,
        }
    }
}

impl std::convert::TryFrom<char> for Buhid {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            BUHID_LETTER_A => Ok(Buhid::BuhidLetterA),
            BUHID_LETTER_I => Ok(Buhid::BuhidLetterI),
            BUHID_LETTER_U => Ok(Buhid::BuhidLetterU),
            BUHID_LETTER_KA => Ok(Buhid::BuhidLetterKa),
            BUHID_LETTER_GA => Ok(Buhid::BuhidLetterGa),
            BUHID_LETTER_NGA => Ok(Buhid::BuhidLetterNga),
            BUHID_LETTER_TA => Ok(Buhid::BuhidLetterTa),
            BUHID_LETTER_DA => Ok(Buhid::BuhidLetterDa),
            BUHID_LETTER_NA => Ok(Buhid::BuhidLetterNa),
            BUHID_LETTER_PA => Ok(Buhid::BuhidLetterPa),
            BUHID_LETTER_BA => Ok(Buhid::BuhidLetterBa),
            BUHID_LETTER_MA => Ok(Buhid::BuhidLetterMa),
            BUHID_LETTER_YA => Ok(Buhid::BuhidLetterYa),
            BUHID_LETTER_RA => Ok(Buhid::BuhidLetterRa),
            BUHID_LETTER_LA => Ok(Buhid::BuhidLetterLa),
            BUHID_LETTER_WA => Ok(Buhid::BuhidLetterWa),
            BUHID_LETTER_SA => Ok(Buhid::BuhidLetterSa),
            BUHID_LETTER_HA => Ok(Buhid::BuhidLetterHa),
            BUHID_VOWEL_SIGN_I => Ok(Buhid::BuhidVowelSignI),
            BUHID_VOWEL_SIGN_U => Ok(Buhid::BuhidVowelSignU),
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
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        Buhid::BuhidLetterA
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Buhid::BuhidLetterA => "buhid letter a",
            Buhid::BuhidLetterI => "buhid letter i",
            Buhid::BuhidLetterU => "buhid letter u",
            Buhid::BuhidLetterKa => "buhid letter ka",
            Buhid::BuhidLetterGa => "buhid letter ga",
            Buhid::BuhidLetterNga => "buhid letter nga",
            Buhid::BuhidLetterTa => "buhid letter ta",
            Buhid::BuhidLetterDa => "buhid letter da",
            Buhid::BuhidLetterNa => "buhid letter na",
            Buhid::BuhidLetterPa => "buhid letter pa",
            Buhid::BuhidLetterBa => "buhid letter ba",
            Buhid::BuhidLetterMa => "buhid letter ma",
            Buhid::BuhidLetterYa => "buhid letter ya",
            Buhid::BuhidLetterRa => "buhid letter ra",
            Buhid::BuhidLetterLa => "buhid letter la",
            Buhid::BuhidLetterWa => "buhid letter wa",
            Buhid::BuhidLetterSa => "buhid letter sa",
            Buhid::BuhidLetterHa => "buhid letter ha",
            Buhid::BuhidVowelSignI => "buhid vowel sign i",
            Buhid::BuhidVowelSignU => "buhid vowel sign u",
        }
    }
}
