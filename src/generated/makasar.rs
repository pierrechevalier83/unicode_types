/// \u{11ee0} → \u{11eff}\
///\
/// 𑻠 𑻡 𑻢 𑻣 𑻤 𑻥 𑻦 𑻧 𑻨 𑻩 𑻪 𑻫 𑻬 𑻭 𑻮 𑻯
/// 𑻰 𑻱 𑻲 𑻳 𑻴 𑻵 𑻶 𑻷 𑻸
pub mod constants {
    /// \u{11ee0}: '𑻠'
    pub const MAKASAR_LETTER_KA: char = '𑻠';
    /// \u{11ee1}: '𑻡'
    pub const MAKASAR_LETTER_GA: char = '𑻡';
    /// \u{11ee2}: '𑻢'
    pub const MAKASAR_LETTER_NGA: char = '𑻢';
    /// \u{11ee3}: '𑻣'
    pub const MAKASAR_LETTER_PA: char = '𑻣';
    /// \u{11ee4}: '𑻤'
    pub const MAKASAR_LETTER_BA: char = '𑻤';
    /// \u{11ee5}: '𑻥'
    pub const MAKASAR_LETTER_MA: char = '𑻥';
    /// \u{11ee6}: '𑻦'
    pub const MAKASAR_LETTER_TA: char = '𑻦';
    /// \u{11ee7}: '𑻧'
    pub const MAKASAR_LETTER_DA: char = '𑻧';
    /// \u{11ee8}: '𑻨'
    pub const MAKASAR_LETTER_NA: char = '𑻨';
    /// \u{11ee9}: '𑻩'
    pub const MAKASAR_LETTER_CA: char = '𑻩';
    /// \u{11eea}: '𑻪'
    pub const MAKASAR_LETTER_JA: char = '𑻪';
    /// \u{11eeb}: '𑻫'
    pub const MAKASAR_LETTER_NYA: char = '𑻫';
    /// \u{11eec}: '𑻬'
    pub const MAKASAR_LETTER_YA: char = '𑻬';
    /// \u{11eed}: '𑻭'
    pub const MAKASAR_LETTER_RA: char = '𑻭';
    /// \u{11eee}: '𑻮'
    pub const MAKASAR_LETTER_LA: char = '𑻮';
    /// \u{11eef}: '𑻯'
    pub const MAKASAR_LETTER_VA: char = '𑻯';
    /// \u{11ef0}: '𑻰'
    pub const MAKASAR_LETTER_SA: char = '𑻰';
    /// \u{11ef1}: '𑻱'
    pub const MAKASAR_LETTER_A: char = '𑻱';
    /// \u{11ef2}: '𑻲'
    pub const MAKASAR_ANGKA: char = '𑻲';
    /// \u{11ef3}: '𑻳'
    pub const MAKASAR_VOWEL_SIGN_I: char = '𑻳';
    /// \u{11ef4}: '𑻴'
    pub const MAKASAR_VOWEL_SIGN_U: char = '𑻴';
    /// \u{11ef5}: '𑻵'
    pub const MAKASAR_VOWEL_SIGN_E: char = '𑻵';
    /// \u{11ef6}: '𑻶'
    pub const MAKASAR_VOWEL_SIGN_O: char = '𑻶';
    /// \u{11ef7}: '𑻷'
    pub const MAKASAR_PASSIMBANG: char = '𑻷';
    /// \u{11ef8}: '𑻸'
    pub const MAKASAR_END_OF_SECTION: char = '𑻸';
}

/// \u{11ee0} → \u{11eff}\
///\
/// 𑻠 𑻡 𑻢 𑻣 𑻤 𑻥 𑻦 𑻧 𑻨 𑻩 𑻪 𑻫 𑻬 𑻭 𑻮 𑻯
/// 𑻰 𑻱 𑻲 𑻳 𑻴 𑻵 𑻶 𑻷 𑻸
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Makasar {
    /// \u{11ee0}: '𑻠'
    MakasarLetterKa,
    /// \u{11ee1}: '𑻡'
    MakasarLetterGa,
    /// \u{11ee2}: '𑻢'
    MakasarLetterNga,
    /// \u{11ee3}: '𑻣'
    MakasarLetterPa,
    /// \u{11ee4}: '𑻤'
    MakasarLetterBa,
    /// \u{11ee5}: '𑻥'
    MakasarLetterMa,
    /// \u{11ee6}: '𑻦'
    MakasarLetterTa,
    /// \u{11ee7}: '𑻧'
    MakasarLetterDa,
    /// \u{11ee8}: '𑻨'
    MakasarLetterNa,
    /// \u{11ee9}: '𑻩'
    MakasarLetterCa,
    /// \u{11eea}: '𑻪'
    MakasarLetterJa,
    /// \u{11eeb}: '𑻫'
    MakasarLetterNya,
    /// \u{11eec}: '𑻬'
    MakasarLetterYa,
    /// \u{11eed}: '𑻭'
    MakasarLetterRa,
    /// \u{11eee}: '𑻮'
    MakasarLetterLa,
    /// \u{11eef}: '𑻯'
    MakasarLetterVa,
    /// \u{11ef0}: '𑻰'
    MakasarLetterSa,
    /// \u{11ef1}: '𑻱'
    MakasarLetterA,
    /// \u{11ef2}: '𑻲'
    MakasarAngka,
    /// \u{11ef3}: '𑻳'
    MakasarVowelSignI,
    /// \u{11ef4}: '𑻴'
    MakasarVowelSignU,
    /// \u{11ef5}: '𑻵'
    MakasarVowelSignE,
    /// \u{11ef6}: '𑻶'
    MakasarVowelSignO,
    /// \u{11ef7}: '𑻷'
    MakasarPassimbang,
    /// \u{11ef8}: '𑻸'
    MakasarEndOfSection,
}

impl Into<char> for Makasar {
    fn into(self) -> char {
        use constants::*;
        match self {
            Makasar::MakasarLetterKa => MAKASAR_LETTER_KA,
            Makasar::MakasarLetterGa => MAKASAR_LETTER_GA,
            Makasar::MakasarLetterNga => MAKASAR_LETTER_NGA,
            Makasar::MakasarLetterPa => MAKASAR_LETTER_PA,
            Makasar::MakasarLetterBa => MAKASAR_LETTER_BA,
            Makasar::MakasarLetterMa => MAKASAR_LETTER_MA,
            Makasar::MakasarLetterTa => MAKASAR_LETTER_TA,
            Makasar::MakasarLetterDa => MAKASAR_LETTER_DA,
            Makasar::MakasarLetterNa => MAKASAR_LETTER_NA,
            Makasar::MakasarLetterCa => MAKASAR_LETTER_CA,
            Makasar::MakasarLetterJa => MAKASAR_LETTER_JA,
            Makasar::MakasarLetterNya => MAKASAR_LETTER_NYA,
            Makasar::MakasarLetterYa => MAKASAR_LETTER_YA,
            Makasar::MakasarLetterRa => MAKASAR_LETTER_RA,
            Makasar::MakasarLetterLa => MAKASAR_LETTER_LA,
            Makasar::MakasarLetterVa => MAKASAR_LETTER_VA,
            Makasar::MakasarLetterSa => MAKASAR_LETTER_SA,
            Makasar::MakasarLetterA => MAKASAR_LETTER_A,
            Makasar::MakasarAngka => MAKASAR_ANGKA,
            Makasar::MakasarVowelSignI => MAKASAR_VOWEL_SIGN_I,
            Makasar::MakasarVowelSignU => MAKASAR_VOWEL_SIGN_U,
            Makasar::MakasarVowelSignE => MAKASAR_VOWEL_SIGN_E,
            Makasar::MakasarVowelSignO => MAKASAR_VOWEL_SIGN_O,
            Makasar::MakasarPassimbang => MAKASAR_PASSIMBANG,
            Makasar::MakasarEndOfSection => MAKASAR_END_OF_SECTION,
        }
    }
}

impl std::convert::TryFrom<char> for Makasar {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            MAKASAR_LETTER_KA => Ok(Makasar::MakasarLetterKa),
            MAKASAR_LETTER_GA => Ok(Makasar::MakasarLetterGa),
            MAKASAR_LETTER_NGA => Ok(Makasar::MakasarLetterNga),
            MAKASAR_LETTER_PA => Ok(Makasar::MakasarLetterPa),
            MAKASAR_LETTER_BA => Ok(Makasar::MakasarLetterBa),
            MAKASAR_LETTER_MA => Ok(Makasar::MakasarLetterMa),
            MAKASAR_LETTER_TA => Ok(Makasar::MakasarLetterTa),
            MAKASAR_LETTER_DA => Ok(Makasar::MakasarLetterDa),
            MAKASAR_LETTER_NA => Ok(Makasar::MakasarLetterNa),
            MAKASAR_LETTER_CA => Ok(Makasar::MakasarLetterCa),
            MAKASAR_LETTER_JA => Ok(Makasar::MakasarLetterJa),
            MAKASAR_LETTER_NYA => Ok(Makasar::MakasarLetterNya),
            MAKASAR_LETTER_YA => Ok(Makasar::MakasarLetterYa),
            MAKASAR_LETTER_RA => Ok(Makasar::MakasarLetterRa),
            MAKASAR_LETTER_LA => Ok(Makasar::MakasarLetterLa),
            MAKASAR_LETTER_VA => Ok(Makasar::MakasarLetterVa),
            MAKASAR_LETTER_SA => Ok(Makasar::MakasarLetterSa),
            MAKASAR_LETTER_A => Ok(Makasar::MakasarLetterA),
            MAKASAR_ANGKA => Ok(Makasar::MakasarAngka),
            MAKASAR_VOWEL_SIGN_I => Ok(Makasar::MakasarVowelSignI),
            MAKASAR_VOWEL_SIGN_U => Ok(Makasar::MakasarVowelSignU),
            MAKASAR_VOWEL_SIGN_E => Ok(Makasar::MakasarVowelSignE),
            MAKASAR_VOWEL_SIGN_O => Ok(Makasar::MakasarVowelSignO),
            MAKASAR_PASSIMBANG => Ok(Makasar::MakasarPassimbang),
            MAKASAR_END_OF_SECTION => Ok(Makasar::MakasarEndOfSection),
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
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        Makasar::MakasarLetterKa
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Makasar::MakasarLetterKa => "makasar letter ka",
            Makasar::MakasarLetterGa => "makasar letter ga",
            Makasar::MakasarLetterNga => "makasar letter nga",
            Makasar::MakasarLetterPa => "makasar letter pa",
            Makasar::MakasarLetterBa => "makasar letter ba",
            Makasar::MakasarLetterMa => "makasar letter ma",
            Makasar::MakasarLetterTa => "makasar letter ta",
            Makasar::MakasarLetterDa => "makasar letter da",
            Makasar::MakasarLetterNa => "makasar letter na",
            Makasar::MakasarLetterCa => "makasar letter ca",
            Makasar::MakasarLetterJa => "makasar letter ja",
            Makasar::MakasarLetterNya => "makasar letter nya",
            Makasar::MakasarLetterYa => "makasar letter ya",
            Makasar::MakasarLetterRa => "makasar letter ra",
            Makasar::MakasarLetterLa => "makasar letter la",
            Makasar::MakasarLetterVa => "makasar letter va",
            Makasar::MakasarLetterSa => "makasar letter sa",
            Makasar::MakasarLetterA => "makasar letter a",
            Makasar::MakasarAngka => "makasar angka",
            Makasar::MakasarVowelSignI => "makasar vowel sign i",
            Makasar::MakasarVowelSignU => "makasar vowel sign u",
            Makasar::MakasarVowelSignE => "makasar vowel sign e",
            Makasar::MakasarVowelSignO => "makasar vowel sign o",
            Makasar::MakasarPassimbang => "makasar passimbang",
            Makasar::MakasarEndOfSection => "makasar end of section",
        }
    }
}
