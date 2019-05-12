/// \u{11ee0} → \u{11eff}\
///\
/// 𑻠 𑻡 𑻢 𑻣 𑻤 𑻥 𑻦 𑻧 𑻨 𑻩 𑻪 𑻫 𑻬 𑻭 𑻮 𑻯\
/// 𑻰 𑻱 𑻲 𑻳 𑻴 𑻵 𑻶 𑻷 𑻸\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{11ee0}: '𑻠'
    pub const LETTER_KA: char = '𑻠';
    /// \u{11ee1}: '𑻡'
    pub const LETTER_GA: char = '𑻡';
    /// \u{11ee2}: '𑻢'
    pub const LETTER_NGA: char = '𑻢';
    /// \u{11ee3}: '𑻣'
    pub const LETTER_PA: char = '𑻣';
    /// \u{11ee4}: '𑻤'
    pub const LETTER_BA: char = '𑻤';
    /// \u{11ee5}: '𑻥'
    pub const LETTER_MA: char = '𑻥';
    /// \u{11ee6}: '𑻦'
    pub const LETTER_TA: char = '𑻦';
    /// \u{11ee7}: '𑻧'
    pub const LETTER_DA: char = '𑻧';
    /// \u{11ee8}: '𑻨'
    pub const LETTER_NA: char = '𑻨';
    /// \u{11ee9}: '𑻩'
    pub const LETTER_CA: char = '𑻩';
    /// \u{11eea}: '𑻪'
    pub const LETTER_JA: char = '𑻪';
    /// \u{11eeb}: '𑻫'
    pub const LETTER_NYA: char = '𑻫';
    /// \u{11eec}: '𑻬'
    pub const LETTER_YA: char = '𑻬';
    /// \u{11eed}: '𑻭'
    pub const LETTER_RA: char = '𑻭';
    /// \u{11eee}: '𑻮'
    pub const LETTER_LA: char = '𑻮';
    /// \u{11eef}: '𑻯'
    pub const LETTER_VA: char = '𑻯';
    /// \u{11ef0}: '𑻰'
    pub const LETTER_SA: char = '𑻰';
    /// \u{11ef1}: '𑻱'
    pub const LETTER_A: char = '𑻱';
    /// \u{11ef2}: '𑻲'
    pub const ANGKA: char = '𑻲';
    /// \u{11ef3}: '𑻳'
    pub const VOWEL_SIGN_I: char = '𑻳';
    /// \u{11ef4}: '𑻴'
    pub const VOWEL_SIGN_U: char = '𑻴';
    /// \u{11ef5}: '𑻵'
    pub const VOWEL_SIGN_E: char = '𑻵';
    /// \u{11ef6}: '𑻶'
    pub const VOWEL_SIGN_O: char = '𑻶';
    /// \u{11ef7}: '𑻷'
    pub const PASSIMBANG: char = '𑻷';
    /// \u{11ef8}: '𑻸'
    pub const END_OF_SECTION: char = '𑻸';
}

/// An enum to represent all characters in the Makasar block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Makasar {
    /// \u{11ee0}: '𑻠'
    LetterKa,
    /// \u{11ee1}: '𑻡'
    LetterGa,
    /// \u{11ee2}: '𑻢'
    LetterNga,
    /// \u{11ee3}: '𑻣'
    LetterPa,
    /// \u{11ee4}: '𑻤'
    LetterBa,
    /// \u{11ee5}: '𑻥'
    LetterMa,
    /// \u{11ee6}: '𑻦'
    LetterTa,
    /// \u{11ee7}: '𑻧'
    LetterDa,
    /// \u{11ee8}: '𑻨'
    LetterNa,
    /// \u{11ee9}: '𑻩'
    LetterCa,
    /// \u{11eea}: '𑻪'
    LetterJa,
    /// \u{11eeb}: '𑻫'
    LetterNya,
    /// \u{11eec}: '𑻬'
    LetterYa,
    /// \u{11eed}: '𑻭'
    LetterRa,
    /// \u{11eee}: '𑻮'
    LetterLa,
    /// \u{11eef}: '𑻯'
    LetterVa,
    /// \u{11ef0}: '𑻰'
    LetterSa,
    /// \u{11ef1}: '𑻱'
    LetterA,
    /// \u{11ef2}: '𑻲'
    Angka,
    /// \u{11ef3}: '𑻳'
    VowelSignI,
    /// \u{11ef4}: '𑻴'
    VowelSignU,
    /// \u{11ef5}: '𑻵'
    VowelSignE,
    /// \u{11ef6}: '𑻶'
    VowelSignO,
    /// \u{11ef7}: '𑻷'
    Passimbang,
    /// \u{11ef8}: '𑻸'
    EndOfSection,
}

impl Into<char> for Makasar {
    fn into(self) -> char {
        use constants::*;
        match self {
            Makasar::LetterKa => LETTER_KA,
            Makasar::LetterGa => LETTER_GA,
            Makasar::LetterNga => LETTER_NGA,
            Makasar::LetterPa => LETTER_PA,
            Makasar::LetterBa => LETTER_BA,
            Makasar::LetterMa => LETTER_MA,
            Makasar::LetterTa => LETTER_TA,
            Makasar::LetterDa => LETTER_DA,
            Makasar::LetterNa => LETTER_NA,
            Makasar::LetterCa => LETTER_CA,
            Makasar::LetterJa => LETTER_JA,
            Makasar::LetterNya => LETTER_NYA,
            Makasar::LetterYa => LETTER_YA,
            Makasar::LetterRa => LETTER_RA,
            Makasar::LetterLa => LETTER_LA,
            Makasar::LetterVa => LETTER_VA,
            Makasar::LetterSa => LETTER_SA,
            Makasar::LetterA => LETTER_A,
            Makasar::Angka => ANGKA,
            Makasar::VowelSignI => VOWEL_SIGN_I,
            Makasar::VowelSignU => VOWEL_SIGN_U,
            Makasar::VowelSignE => VOWEL_SIGN_E,
            Makasar::VowelSignO => VOWEL_SIGN_O,
            Makasar::Passimbang => PASSIMBANG,
            Makasar::EndOfSection => END_OF_SECTION,
        }
    }
}

impl std::convert::TryFrom<char> for Makasar {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_KA => Ok(Makasar::LetterKa),
            LETTER_GA => Ok(Makasar::LetterGa),
            LETTER_NGA => Ok(Makasar::LetterNga),
            LETTER_PA => Ok(Makasar::LetterPa),
            LETTER_BA => Ok(Makasar::LetterBa),
            LETTER_MA => Ok(Makasar::LetterMa),
            LETTER_TA => Ok(Makasar::LetterTa),
            LETTER_DA => Ok(Makasar::LetterDa),
            LETTER_NA => Ok(Makasar::LetterNa),
            LETTER_CA => Ok(Makasar::LetterCa),
            LETTER_JA => Ok(Makasar::LetterJa),
            LETTER_NYA => Ok(Makasar::LetterNya),
            LETTER_YA => Ok(Makasar::LetterYa),
            LETTER_RA => Ok(Makasar::LetterRa),
            LETTER_LA => Ok(Makasar::LetterLa),
            LETTER_VA => Ok(Makasar::LetterVa),
            LETTER_SA => Ok(Makasar::LetterSa),
            LETTER_A => Ok(Makasar::LetterA),
            ANGKA => Ok(Makasar::Angka),
            VOWEL_SIGN_I => Ok(Makasar::VowelSignI),
            VOWEL_SIGN_U => Ok(Makasar::VowelSignU),
            VOWEL_SIGN_E => Ok(Makasar::VowelSignE),
            VOWEL_SIGN_O => Ok(Makasar::VowelSignO),
            PASSIMBANG => Ok(Makasar::Passimbang),
            END_OF_SECTION => Ok(Makasar::EndOfSection),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Makasar {
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

impl std::convert::TryFrom<u32> for Makasar {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Makasar {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Makasar {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Makasar::LetterKa
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Makasar::LetterKa => "makasar letter ka",
            Makasar::LetterGa => "makasar letter ga",
            Makasar::LetterNga => "makasar letter nga",
            Makasar::LetterPa => "makasar letter pa",
            Makasar::LetterBa => "makasar letter ba",
            Makasar::LetterMa => "makasar letter ma",
            Makasar::LetterTa => "makasar letter ta",
            Makasar::LetterDa => "makasar letter da",
            Makasar::LetterNa => "makasar letter na",
            Makasar::LetterCa => "makasar letter ca",
            Makasar::LetterJa => "makasar letter ja",
            Makasar::LetterNya => "makasar letter nya",
            Makasar::LetterYa => "makasar letter ya",
            Makasar::LetterRa => "makasar letter ra",
            Makasar::LetterLa => "makasar letter la",
            Makasar::LetterVa => "makasar letter va",
            Makasar::LetterSa => "makasar letter sa",
            Makasar::LetterA => "makasar letter a",
            Makasar::Angka => "makasar angka",
            Makasar::VowelSignI => "makasar vowel sign i",
            Makasar::VowelSignU => "makasar vowel sign u",
            Makasar::VowelSignE => "makasar vowel sign e",
            Makasar::VowelSignO => "makasar vowel sign o",
            Makasar::Passimbang => "makasar passimbang",
            Makasar::EndOfSection => "makasar end of section",
        }
    }
}
