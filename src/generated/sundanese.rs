/// \u{1b80} → \u{1bbf}
///
/// ᮀ ᮁ ᮂ ᮃ ᮄ ᮅ ᮆ ᮇ ᮈ ᮉ ᮊ ᮋ ᮌ ᮍ ᮎ ᮏ\
/// ᮐ ᮑ ᮒ ᮓ ᮔ ᮕ ᮖ ᮗ ᮘ ᮙ ᮚ ᮛ ᮜ ᮝ ᮞ ᮟ\
/// ᮠ ᮡ ᮢ ᮣ ᮤ ᮥ ᮦ ᮧ ᮨ ᮩ ᮪ ᮫ ᮬ ᮭ ᮮ ᮯ\
/// ᮰ ᮱ ᮲ ᮳ ᮴ ᮵ ᮶ ᮷ ᮸ ᮹ ᮺ ᮻ ᮼ ᮽ ᮾ\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{1b80}: 'ᮀ'
    pub const SIGN_PANYECEK: char = 'ᮀ';
    /// \u{1b81}: 'ᮁ'
    pub const SIGN_PANGLAYAR: char = 'ᮁ';
    /// \u{1b82}: 'ᮂ'
    pub const SIGN_PANGWISAD: char = 'ᮂ';
    /// \u{1b83}: 'ᮃ'
    pub const LETTER_A: char = 'ᮃ';
    /// \u{1b84}: 'ᮄ'
    pub const LETTER_I: char = 'ᮄ';
    /// \u{1b85}: 'ᮅ'
    pub const LETTER_U: char = 'ᮅ';
    /// \u{1b86}: 'ᮆ'
    pub const LETTER_AE: char = 'ᮆ';
    /// \u{1b87}: 'ᮇ'
    pub const LETTER_O: char = 'ᮇ';
    /// \u{1b88}: 'ᮈ'
    pub const LETTER_E: char = 'ᮈ';
    /// \u{1b89}: 'ᮉ'
    pub const LETTER_EU: char = 'ᮉ';
    /// \u{1b8a}: 'ᮊ'
    pub const LETTER_KA: char = 'ᮊ';
    /// \u{1b8b}: 'ᮋ'
    pub const LETTER_QA: char = 'ᮋ';
    /// \u{1b8c}: 'ᮌ'
    pub const LETTER_GA: char = 'ᮌ';
    /// \u{1b8d}: 'ᮍ'
    pub const LETTER_NGA: char = 'ᮍ';
    /// \u{1b8e}: 'ᮎ'
    pub const LETTER_CA: char = 'ᮎ';
    /// \u{1b8f}: 'ᮏ'
    pub const LETTER_JA: char = 'ᮏ';
    /// \u{1b90}: 'ᮐ'
    pub const LETTER_ZA: char = 'ᮐ';
    /// \u{1b91}: 'ᮑ'
    pub const LETTER_NYA: char = 'ᮑ';
    /// \u{1b92}: 'ᮒ'
    pub const LETTER_TA: char = 'ᮒ';
    /// \u{1b93}: 'ᮓ'
    pub const LETTER_DA: char = 'ᮓ';
    /// \u{1b94}: 'ᮔ'
    pub const LETTER_NA: char = 'ᮔ';
    /// \u{1b95}: 'ᮕ'
    pub const LETTER_PA: char = 'ᮕ';
    /// \u{1b96}: 'ᮖ'
    pub const LETTER_FA: char = 'ᮖ';
    /// \u{1b97}: 'ᮗ'
    pub const LETTER_VA: char = 'ᮗ';
    /// \u{1b98}: 'ᮘ'
    pub const LETTER_BA: char = 'ᮘ';
    /// \u{1b99}: 'ᮙ'
    pub const LETTER_MA: char = 'ᮙ';
    /// \u{1b9a}: 'ᮚ'
    pub const LETTER_YA: char = 'ᮚ';
    /// \u{1b9b}: 'ᮛ'
    pub const LETTER_RA: char = 'ᮛ';
    /// \u{1b9c}: 'ᮜ'
    pub const LETTER_LA: char = 'ᮜ';
    /// \u{1b9d}: 'ᮝ'
    pub const LETTER_WA: char = 'ᮝ';
    /// \u{1b9e}: 'ᮞ'
    pub const LETTER_SA: char = 'ᮞ';
    /// \u{1b9f}: 'ᮟ'
    pub const LETTER_XA: char = 'ᮟ';
    /// \u{1ba0}: 'ᮠ'
    pub const LETTER_HA: char = 'ᮠ';
    /// \u{1ba1}: 'ᮡ'
    pub const CONSONANT_SIGN_PAMINGKAL: char = 'ᮡ';
    /// \u{1ba2}: 'ᮢ'
    pub const CONSONANT_SIGN_PANYAKRA: char = 'ᮢ';
    /// \u{1ba3}: 'ᮣ'
    pub const CONSONANT_SIGN_PANYIKU: char = 'ᮣ';
    /// \u{1ba4}: 'ᮤ'
    pub const VOWEL_SIGN_PANGHULU: char = 'ᮤ';
    /// \u{1ba5}: 'ᮥ'
    pub const VOWEL_SIGN_PANYUKU: char = 'ᮥ';
    /// \u{1ba6}: 'ᮦ'
    pub const VOWEL_SIGN_PANAELAENG: char = 'ᮦ';
    /// \u{1ba7}: 'ᮧ'
    pub const VOWEL_SIGN_PANOLONG: char = 'ᮧ';
    /// \u{1ba8}: 'ᮨ'
    pub const VOWEL_SIGN_PAMEPET: char = 'ᮨ';
    /// \u{1ba9}: 'ᮩ'
    pub const VOWEL_SIGN_PANEULEUNG: char = 'ᮩ';
    /// \u{1baa}: '᮪'
    pub const SIGN_PAMAAEH: char = '᮪';
    /// \u{1bab}: '᮫'
    pub const SIGN_VIRAMA: char = '᮫';
    /// \u{1bac}: 'ᮬ'
    pub const CONSONANT_SIGN_PASANGAN_MA: char = 'ᮬ';
    /// \u{1bad}: 'ᮭ'
    pub const CONSONANT_SIGN_PASANGAN_WA: char = 'ᮭ';
    /// \u{1bae}: 'ᮮ'
    pub const LETTER_KHA: char = 'ᮮ';
    /// \u{1baf}: 'ᮯ'
    pub const LETTER_SYA: char = 'ᮯ';
    /// \u{1bb0}: '᮰'
    pub const DIGIT_ZERO: char = '᮰';
    /// \u{1bb1}: '᮱'
    pub const DIGIT_ONE: char = '᮱';
    /// \u{1bb2}: '᮲'
    pub const DIGIT_TWO: char = '᮲';
    /// \u{1bb3}: '᮳'
    pub const DIGIT_THREE: char = '᮳';
    /// \u{1bb4}: '᮴'
    pub const DIGIT_FOUR: char = '᮴';
    /// \u{1bb5}: '᮵'
    pub const DIGIT_FIVE: char = '᮵';
    /// \u{1bb6}: '᮶'
    pub const DIGIT_SIX: char = '᮶';
    /// \u{1bb7}: '᮷'
    pub const DIGIT_SEVEN: char = '᮷';
    /// \u{1bb8}: '᮸'
    pub const DIGIT_EIGHT: char = '᮸';
    /// \u{1bb9}: '᮹'
    pub const DIGIT_NINE: char = '᮹';
    /// \u{1bba}: 'ᮺ'
    pub const AVAGRAHA: char = 'ᮺ';
    /// \u{1bbb}: 'ᮻ'
    pub const LETTER_REU: char = 'ᮻ';
    /// \u{1bbc}: 'ᮼ'
    pub const LETTER_LEU: char = 'ᮼ';
    /// \u{1bbd}: 'ᮽ'
    pub const LETTER_BHA: char = 'ᮽ';
    /// \u{1bbe}: 'ᮾ'
    pub const LETTER_FINAL_K: char = 'ᮾ';
}

/// An enum to represent all characters in the Sundanese block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Sundanese {
    /// \u{1b80}: 'ᮀ'
    SignPanyecek,
    /// \u{1b81}: 'ᮁ'
    SignPanglayar,
    /// \u{1b82}: 'ᮂ'
    SignPangwisad,
    /// \u{1b83}: 'ᮃ'
    LetterA,
    /// \u{1b84}: 'ᮄ'
    LetterI,
    /// \u{1b85}: 'ᮅ'
    LetterU,
    /// \u{1b86}: 'ᮆ'
    LetterAe,
    /// \u{1b87}: 'ᮇ'
    LetterO,
    /// \u{1b88}: 'ᮈ'
    LetterE,
    /// \u{1b89}: 'ᮉ'
    LetterEu,
    /// \u{1b8a}: 'ᮊ'
    LetterKa,
    /// \u{1b8b}: 'ᮋ'
    LetterQa,
    /// \u{1b8c}: 'ᮌ'
    LetterGa,
    /// \u{1b8d}: 'ᮍ'
    LetterNga,
    /// \u{1b8e}: 'ᮎ'
    LetterCa,
    /// \u{1b8f}: 'ᮏ'
    LetterJa,
    /// \u{1b90}: 'ᮐ'
    LetterZa,
    /// \u{1b91}: 'ᮑ'
    LetterNya,
    /// \u{1b92}: 'ᮒ'
    LetterTa,
    /// \u{1b93}: 'ᮓ'
    LetterDa,
    /// \u{1b94}: 'ᮔ'
    LetterNa,
    /// \u{1b95}: 'ᮕ'
    LetterPa,
    /// \u{1b96}: 'ᮖ'
    LetterFa,
    /// \u{1b97}: 'ᮗ'
    LetterVa,
    /// \u{1b98}: 'ᮘ'
    LetterBa,
    /// \u{1b99}: 'ᮙ'
    LetterMa,
    /// \u{1b9a}: 'ᮚ'
    LetterYa,
    /// \u{1b9b}: 'ᮛ'
    LetterRa,
    /// \u{1b9c}: 'ᮜ'
    LetterLa,
    /// \u{1b9d}: 'ᮝ'
    LetterWa,
    /// \u{1b9e}: 'ᮞ'
    LetterSa,
    /// \u{1b9f}: 'ᮟ'
    LetterXa,
    /// \u{1ba0}: 'ᮠ'
    LetterHa,
    /// \u{1ba1}: 'ᮡ'
    ConsonantSignPamingkal,
    /// \u{1ba2}: 'ᮢ'
    ConsonantSignPanyakra,
    /// \u{1ba3}: 'ᮣ'
    ConsonantSignPanyiku,
    /// \u{1ba4}: 'ᮤ'
    VowelSignPanghulu,
    /// \u{1ba5}: 'ᮥ'
    VowelSignPanyuku,
    /// \u{1ba6}: 'ᮦ'
    VowelSignPanaelaeng,
    /// \u{1ba7}: 'ᮧ'
    VowelSignPanolong,
    /// \u{1ba8}: 'ᮨ'
    VowelSignPamepet,
    /// \u{1ba9}: 'ᮩ'
    VowelSignPaneuleung,
    /// \u{1baa}: '᮪'
    SignPamaaeh,
    /// \u{1bab}: '᮫'
    SignVirama,
    /// \u{1bac}: 'ᮬ'
    ConsonantSignPasanganMa,
    /// \u{1bad}: 'ᮭ'
    ConsonantSignPasanganWa,
    /// \u{1bae}: 'ᮮ'
    LetterKha,
    /// \u{1baf}: 'ᮯ'
    LetterSya,
    /// \u{1bb0}: '᮰'
    DigitZero,
    /// \u{1bb1}: '᮱'
    DigitOne,
    /// \u{1bb2}: '᮲'
    DigitTwo,
    /// \u{1bb3}: '᮳'
    DigitThree,
    /// \u{1bb4}: '᮴'
    DigitFour,
    /// \u{1bb5}: '᮵'
    DigitFive,
    /// \u{1bb6}: '᮶'
    DigitSix,
    /// \u{1bb7}: '᮷'
    DigitSeven,
    /// \u{1bb8}: '᮸'
    DigitEight,
    /// \u{1bb9}: '᮹'
    DigitNine,
    /// \u{1bba}: 'ᮺ'
    Avagraha,
    /// \u{1bbb}: 'ᮻ'
    LetterReu,
    /// \u{1bbc}: 'ᮼ'
    LetterLeu,
    /// \u{1bbd}: 'ᮽ'
    LetterBha,
    /// \u{1bbe}: 'ᮾ'
    LetterFinalK,
}

impl Into<char> for Sundanese {
    fn into(self) -> char {
        use constants::*;
        match self {
            Sundanese::SignPanyecek => SIGN_PANYECEK,
            Sundanese::SignPanglayar => SIGN_PANGLAYAR,
            Sundanese::SignPangwisad => SIGN_PANGWISAD,
            Sundanese::LetterA => LETTER_A,
            Sundanese::LetterI => LETTER_I,
            Sundanese::LetterU => LETTER_U,
            Sundanese::LetterAe => LETTER_AE,
            Sundanese::LetterO => LETTER_O,
            Sundanese::LetterE => LETTER_E,
            Sundanese::LetterEu => LETTER_EU,
            Sundanese::LetterKa => LETTER_KA,
            Sundanese::LetterQa => LETTER_QA,
            Sundanese::LetterGa => LETTER_GA,
            Sundanese::LetterNga => LETTER_NGA,
            Sundanese::LetterCa => LETTER_CA,
            Sundanese::LetterJa => LETTER_JA,
            Sundanese::LetterZa => LETTER_ZA,
            Sundanese::LetterNya => LETTER_NYA,
            Sundanese::LetterTa => LETTER_TA,
            Sundanese::LetterDa => LETTER_DA,
            Sundanese::LetterNa => LETTER_NA,
            Sundanese::LetterPa => LETTER_PA,
            Sundanese::LetterFa => LETTER_FA,
            Sundanese::LetterVa => LETTER_VA,
            Sundanese::LetterBa => LETTER_BA,
            Sundanese::LetterMa => LETTER_MA,
            Sundanese::LetterYa => LETTER_YA,
            Sundanese::LetterRa => LETTER_RA,
            Sundanese::LetterLa => LETTER_LA,
            Sundanese::LetterWa => LETTER_WA,
            Sundanese::LetterSa => LETTER_SA,
            Sundanese::LetterXa => LETTER_XA,
            Sundanese::LetterHa => LETTER_HA,
            Sundanese::ConsonantSignPamingkal => CONSONANT_SIGN_PAMINGKAL,
            Sundanese::ConsonantSignPanyakra => CONSONANT_SIGN_PANYAKRA,
            Sundanese::ConsonantSignPanyiku => CONSONANT_SIGN_PANYIKU,
            Sundanese::VowelSignPanghulu => VOWEL_SIGN_PANGHULU,
            Sundanese::VowelSignPanyuku => VOWEL_SIGN_PANYUKU,
            Sundanese::VowelSignPanaelaeng => VOWEL_SIGN_PANAELAENG,
            Sundanese::VowelSignPanolong => VOWEL_SIGN_PANOLONG,
            Sundanese::VowelSignPamepet => VOWEL_SIGN_PAMEPET,
            Sundanese::VowelSignPaneuleung => VOWEL_SIGN_PANEULEUNG,
            Sundanese::SignPamaaeh => SIGN_PAMAAEH,
            Sundanese::SignVirama => SIGN_VIRAMA,
            Sundanese::ConsonantSignPasanganMa => CONSONANT_SIGN_PASANGAN_MA,
            Sundanese::ConsonantSignPasanganWa => CONSONANT_SIGN_PASANGAN_WA,
            Sundanese::LetterKha => LETTER_KHA,
            Sundanese::LetterSya => LETTER_SYA,
            Sundanese::DigitZero => DIGIT_ZERO,
            Sundanese::DigitOne => DIGIT_ONE,
            Sundanese::DigitTwo => DIGIT_TWO,
            Sundanese::DigitThree => DIGIT_THREE,
            Sundanese::DigitFour => DIGIT_FOUR,
            Sundanese::DigitFive => DIGIT_FIVE,
            Sundanese::DigitSix => DIGIT_SIX,
            Sundanese::DigitSeven => DIGIT_SEVEN,
            Sundanese::DigitEight => DIGIT_EIGHT,
            Sundanese::DigitNine => DIGIT_NINE,
            Sundanese::Avagraha => AVAGRAHA,
            Sundanese::LetterReu => LETTER_REU,
            Sundanese::LetterLeu => LETTER_LEU,
            Sundanese::LetterBha => LETTER_BHA,
            Sundanese::LetterFinalK => LETTER_FINAL_K,
        }
    }
}

impl std::convert::TryFrom<char> for Sundanese {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            SIGN_PANYECEK => Ok(Sundanese::SignPanyecek),
            SIGN_PANGLAYAR => Ok(Sundanese::SignPanglayar),
            SIGN_PANGWISAD => Ok(Sundanese::SignPangwisad),
            LETTER_A => Ok(Sundanese::LetterA),
            LETTER_I => Ok(Sundanese::LetterI),
            LETTER_U => Ok(Sundanese::LetterU),
            LETTER_AE => Ok(Sundanese::LetterAe),
            LETTER_O => Ok(Sundanese::LetterO),
            LETTER_E => Ok(Sundanese::LetterE),
            LETTER_EU => Ok(Sundanese::LetterEu),
            LETTER_KA => Ok(Sundanese::LetterKa),
            LETTER_QA => Ok(Sundanese::LetterQa),
            LETTER_GA => Ok(Sundanese::LetterGa),
            LETTER_NGA => Ok(Sundanese::LetterNga),
            LETTER_CA => Ok(Sundanese::LetterCa),
            LETTER_JA => Ok(Sundanese::LetterJa),
            LETTER_ZA => Ok(Sundanese::LetterZa),
            LETTER_NYA => Ok(Sundanese::LetterNya),
            LETTER_TA => Ok(Sundanese::LetterTa),
            LETTER_DA => Ok(Sundanese::LetterDa),
            LETTER_NA => Ok(Sundanese::LetterNa),
            LETTER_PA => Ok(Sundanese::LetterPa),
            LETTER_FA => Ok(Sundanese::LetterFa),
            LETTER_VA => Ok(Sundanese::LetterVa),
            LETTER_BA => Ok(Sundanese::LetterBa),
            LETTER_MA => Ok(Sundanese::LetterMa),
            LETTER_YA => Ok(Sundanese::LetterYa),
            LETTER_RA => Ok(Sundanese::LetterRa),
            LETTER_LA => Ok(Sundanese::LetterLa),
            LETTER_WA => Ok(Sundanese::LetterWa),
            LETTER_SA => Ok(Sundanese::LetterSa),
            LETTER_XA => Ok(Sundanese::LetterXa),
            LETTER_HA => Ok(Sundanese::LetterHa),
            CONSONANT_SIGN_PAMINGKAL => Ok(Sundanese::ConsonantSignPamingkal),
            CONSONANT_SIGN_PANYAKRA => Ok(Sundanese::ConsonantSignPanyakra),
            CONSONANT_SIGN_PANYIKU => Ok(Sundanese::ConsonantSignPanyiku),
            VOWEL_SIGN_PANGHULU => Ok(Sundanese::VowelSignPanghulu),
            VOWEL_SIGN_PANYUKU => Ok(Sundanese::VowelSignPanyuku),
            VOWEL_SIGN_PANAELAENG => Ok(Sundanese::VowelSignPanaelaeng),
            VOWEL_SIGN_PANOLONG => Ok(Sundanese::VowelSignPanolong),
            VOWEL_SIGN_PAMEPET => Ok(Sundanese::VowelSignPamepet),
            VOWEL_SIGN_PANEULEUNG => Ok(Sundanese::VowelSignPaneuleung),
            SIGN_PAMAAEH => Ok(Sundanese::SignPamaaeh),
            SIGN_VIRAMA => Ok(Sundanese::SignVirama),
            CONSONANT_SIGN_PASANGAN_MA => Ok(Sundanese::ConsonantSignPasanganMa),
            CONSONANT_SIGN_PASANGAN_WA => Ok(Sundanese::ConsonantSignPasanganWa),
            LETTER_KHA => Ok(Sundanese::LetterKha),
            LETTER_SYA => Ok(Sundanese::LetterSya),
            DIGIT_ZERO => Ok(Sundanese::DigitZero),
            DIGIT_ONE => Ok(Sundanese::DigitOne),
            DIGIT_TWO => Ok(Sundanese::DigitTwo),
            DIGIT_THREE => Ok(Sundanese::DigitThree),
            DIGIT_FOUR => Ok(Sundanese::DigitFour),
            DIGIT_FIVE => Ok(Sundanese::DigitFive),
            DIGIT_SIX => Ok(Sundanese::DigitSix),
            DIGIT_SEVEN => Ok(Sundanese::DigitSeven),
            DIGIT_EIGHT => Ok(Sundanese::DigitEight),
            DIGIT_NINE => Ok(Sundanese::DigitNine),
            AVAGRAHA => Ok(Sundanese::Avagraha),
            LETTER_REU => Ok(Sundanese::LetterReu),
            LETTER_LEU => Ok(Sundanese::LetterLeu),
            LETTER_BHA => Ok(Sundanese::LetterBha),
            LETTER_FINAL_K => Ok(Sundanese::LetterFinalK),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Sundanese {
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

impl std::convert::TryFrom<u32> for Sundanese {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Sundanese {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Sundanese {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Sundanese::SignPanyecek
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Sundanese::SignPanyecek => "sundanese sign panyecek",
            Sundanese::SignPanglayar => "sundanese sign panglayar",
            Sundanese::SignPangwisad => "sundanese sign pangwisad",
            Sundanese::LetterA => "sundanese letter a",
            Sundanese::LetterI => "sundanese letter i",
            Sundanese::LetterU => "sundanese letter u",
            Sundanese::LetterAe => "sundanese letter ae",
            Sundanese::LetterO => "sundanese letter o",
            Sundanese::LetterE => "sundanese letter e",
            Sundanese::LetterEu => "sundanese letter eu",
            Sundanese::LetterKa => "sundanese letter ka",
            Sundanese::LetterQa => "sundanese letter qa",
            Sundanese::LetterGa => "sundanese letter ga",
            Sundanese::LetterNga => "sundanese letter nga",
            Sundanese::LetterCa => "sundanese letter ca",
            Sundanese::LetterJa => "sundanese letter ja",
            Sundanese::LetterZa => "sundanese letter za",
            Sundanese::LetterNya => "sundanese letter nya",
            Sundanese::LetterTa => "sundanese letter ta",
            Sundanese::LetterDa => "sundanese letter da",
            Sundanese::LetterNa => "sundanese letter na",
            Sundanese::LetterPa => "sundanese letter pa",
            Sundanese::LetterFa => "sundanese letter fa",
            Sundanese::LetterVa => "sundanese letter va",
            Sundanese::LetterBa => "sundanese letter ba",
            Sundanese::LetterMa => "sundanese letter ma",
            Sundanese::LetterYa => "sundanese letter ya",
            Sundanese::LetterRa => "sundanese letter ra",
            Sundanese::LetterLa => "sundanese letter la",
            Sundanese::LetterWa => "sundanese letter wa",
            Sundanese::LetterSa => "sundanese letter sa",
            Sundanese::LetterXa => "sundanese letter xa",
            Sundanese::LetterHa => "sundanese letter ha",
            Sundanese::ConsonantSignPamingkal => "sundanese consonant sign pamingkal",
            Sundanese::ConsonantSignPanyakra => "sundanese consonant sign panyakra",
            Sundanese::ConsonantSignPanyiku => "sundanese consonant sign panyiku",
            Sundanese::VowelSignPanghulu => "sundanese vowel sign panghulu",
            Sundanese::VowelSignPanyuku => "sundanese vowel sign panyuku",
            Sundanese::VowelSignPanaelaeng => "sundanese vowel sign panaelaeng",
            Sundanese::VowelSignPanolong => "sundanese vowel sign panolong",
            Sundanese::VowelSignPamepet => "sundanese vowel sign pamepet",
            Sundanese::VowelSignPaneuleung => "sundanese vowel sign paneuleung",
            Sundanese::SignPamaaeh => "sundanese sign pamaaeh",
            Sundanese::SignVirama => "sundanese sign virama",
            Sundanese::ConsonantSignPasanganMa => "sundanese consonant sign pasangan ma",
            Sundanese::ConsonantSignPasanganWa => "sundanese consonant sign pasangan wa",
            Sundanese::LetterKha => "sundanese letter kha",
            Sundanese::LetterSya => "sundanese letter sya",
            Sundanese::DigitZero => "sundanese digit zero",
            Sundanese::DigitOne => "sundanese digit one",
            Sundanese::DigitTwo => "sundanese digit two",
            Sundanese::DigitThree => "sundanese digit three",
            Sundanese::DigitFour => "sundanese digit four",
            Sundanese::DigitFive => "sundanese digit five",
            Sundanese::DigitSix => "sundanese digit six",
            Sundanese::DigitSeven => "sundanese digit seven",
            Sundanese::DigitEight => "sundanese digit eight",
            Sundanese::DigitNine => "sundanese digit nine",
            Sundanese::Avagraha => "sundanese avagraha",
            Sundanese::LetterReu => "sundanese letter reu",
            Sundanese::LetterLeu => "sundanese letter leu",
            Sundanese::LetterBha => "sundanese letter bha",
            Sundanese::LetterFinalK => "sundanese letter final k",
        }
    }
}
