/// \u{a930} → \u{a95f}\
///\
/// ꤰ ꤱ ꤲ ꤳ ꤴ ꤵ ꤶ ꤷ ꤸ ꤹ ꤺ ꤻ ꤼ ꤽ ꤾ ꤿ\
/// ꥀ ꥁ ꥂ ꥃ ꥄ ꥅ ꥆ ꥇ ꥈ ꥉ ꥊ ꥋ ꥌ ꥍ ꥎ ꥏ\
/// ꥐ ꥑ ꥒ ꥓\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{a930}: 'ꤰ'
    pub const LETTER_KA: char = 'ꤰ';
    /// \u{a931}: 'ꤱ'
    pub const LETTER_GA: char = 'ꤱ';
    /// \u{a932}: 'ꤲ'
    pub const LETTER_NGA: char = 'ꤲ';
    /// \u{a933}: 'ꤳ'
    pub const LETTER_TA: char = 'ꤳ';
    /// \u{a934}: 'ꤴ'
    pub const LETTER_DA: char = 'ꤴ';
    /// \u{a935}: 'ꤵ'
    pub const LETTER_NA: char = 'ꤵ';
    /// \u{a936}: 'ꤶ'
    pub const LETTER_PA: char = 'ꤶ';
    /// \u{a937}: 'ꤷ'
    pub const LETTER_BA: char = 'ꤷ';
    /// \u{a938}: 'ꤸ'
    pub const LETTER_MA: char = 'ꤸ';
    /// \u{a939}: 'ꤹ'
    pub const LETTER_CA: char = 'ꤹ';
    /// \u{a93a}: 'ꤺ'
    pub const LETTER_JA: char = 'ꤺ';
    /// \u{a93b}: 'ꤻ'
    pub const LETTER_NYA: char = 'ꤻ';
    /// \u{a93c}: 'ꤼ'
    pub const LETTER_SA: char = 'ꤼ';
    /// \u{a93d}: 'ꤽ'
    pub const LETTER_RA: char = 'ꤽ';
    /// \u{a93e}: 'ꤾ'
    pub const LETTER_LA: char = 'ꤾ';
    /// \u{a93f}: 'ꤿ'
    pub const LETTER_YA: char = 'ꤿ';
    /// \u{a940}: 'ꥀ'
    pub const LETTER_WA: char = 'ꥀ';
    /// \u{a941}: 'ꥁ'
    pub const LETTER_HA: char = 'ꥁ';
    /// \u{a942}: 'ꥂ'
    pub const LETTER_MBA: char = 'ꥂ';
    /// \u{a943}: 'ꥃ'
    pub const LETTER_NGGA: char = 'ꥃ';
    /// \u{a944}: 'ꥄ'
    pub const LETTER_NDA: char = 'ꥄ';
    /// \u{a945}: 'ꥅ'
    pub const LETTER_NYJA: char = 'ꥅ';
    /// \u{a946}: 'ꥆ'
    pub const LETTER_A: char = 'ꥆ';
    /// \u{a947}: 'ꥇ'
    pub const VOWEL_SIGN_I: char = 'ꥇ';
    /// \u{a948}: 'ꥈ'
    pub const VOWEL_SIGN_U: char = 'ꥈ';
    /// \u{a949}: 'ꥉ'
    pub const VOWEL_SIGN_E: char = 'ꥉ';
    /// \u{a94a}: 'ꥊ'
    pub const VOWEL_SIGN_AI: char = 'ꥊ';
    /// \u{a94b}: 'ꥋ'
    pub const VOWEL_SIGN_O: char = 'ꥋ';
    /// \u{a94c}: 'ꥌ'
    pub const VOWEL_SIGN_AU: char = 'ꥌ';
    /// \u{a94d}: 'ꥍ'
    pub const VOWEL_SIGN_EU: char = 'ꥍ';
    /// \u{a94e}: 'ꥎ'
    pub const VOWEL_SIGN_EA: char = 'ꥎ';
    /// \u{a94f}: 'ꥏ'
    pub const CONSONANT_SIGN_NG: char = 'ꥏ';
    /// \u{a950}: 'ꥐ'
    pub const CONSONANT_SIGN_N: char = 'ꥐ';
    /// \u{a951}: 'ꥑ'
    pub const CONSONANT_SIGN_R: char = 'ꥑ';
    /// \u{a952}: 'ꥒ'
    pub const CONSONANT_SIGN_H: char = 'ꥒ';
    /// \u{a953}: '꥓'
    pub const VIRAMA: char = '꥓';
}

/// An enum to represent all characters in the Rejang block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Rejang {
    /// \u{a930}: 'ꤰ'
    LetterKa,
    /// \u{a931}: 'ꤱ'
    LetterGa,
    /// \u{a932}: 'ꤲ'
    LetterNga,
    /// \u{a933}: 'ꤳ'
    LetterTa,
    /// \u{a934}: 'ꤴ'
    LetterDa,
    /// \u{a935}: 'ꤵ'
    LetterNa,
    /// \u{a936}: 'ꤶ'
    LetterPa,
    /// \u{a937}: 'ꤷ'
    LetterBa,
    /// \u{a938}: 'ꤸ'
    LetterMa,
    /// \u{a939}: 'ꤹ'
    LetterCa,
    /// \u{a93a}: 'ꤺ'
    LetterJa,
    /// \u{a93b}: 'ꤻ'
    LetterNya,
    /// \u{a93c}: 'ꤼ'
    LetterSa,
    /// \u{a93d}: 'ꤽ'
    LetterRa,
    /// \u{a93e}: 'ꤾ'
    LetterLa,
    /// \u{a93f}: 'ꤿ'
    LetterYa,
    /// \u{a940}: 'ꥀ'
    LetterWa,
    /// \u{a941}: 'ꥁ'
    LetterHa,
    /// \u{a942}: 'ꥂ'
    LetterMba,
    /// \u{a943}: 'ꥃ'
    LetterNgga,
    /// \u{a944}: 'ꥄ'
    LetterNda,
    /// \u{a945}: 'ꥅ'
    LetterNyja,
    /// \u{a946}: 'ꥆ'
    LetterA,
    /// \u{a947}: 'ꥇ'
    VowelSignI,
    /// \u{a948}: 'ꥈ'
    VowelSignU,
    /// \u{a949}: 'ꥉ'
    VowelSignE,
    /// \u{a94a}: 'ꥊ'
    VowelSignAi,
    /// \u{a94b}: 'ꥋ'
    VowelSignO,
    /// \u{a94c}: 'ꥌ'
    VowelSignAu,
    /// \u{a94d}: 'ꥍ'
    VowelSignEu,
    /// \u{a94e}: 'ꥎ'
    VowelSignEa,
    /// \u{a94f}: 'ꥏ'
    ConsonantSignNg,
    /// \u{a950}: 'ꥐ'
    ConsonantSignN,
    /// \u{a951}: 'ꥑ'
    ConsonantSignR,
    /// \u{a952}: 'ꥒ'
    ConsonantSignH,
    /// \u{a953}: '꥓'
    Virama,
}

impl Into<char> for Rejang {
    fn into(self) -> char {
        use constants::*;
        match self {
            Rejang::LetterKa => LETTER_KA,
            Rejang::LetterGa => LETTER_GA,
            Rejang::LetterNga => LETTER_NGA,
            Rejang::LetterTa => LETTER_TA,
            Rejang::LetterDa => LETTER_DA,
            Rejang::LetterNa => LETTER_NA,
            Rejang::LetterPa => LETTER_PA,
            Rejang::LetterBa => LETTER_BA,
            Rejang::LetterMa => LETTER_MA,
            Rejang::LetterCa => LETTER_CA,
            Rejang::LetterJa => LETTER_JA,
            Rejang::LetterNya => LETTER_NYA,
            Rejang::LetterSa => LETTER_SA,
            Rejang::LetterRa => LETTER_RA,
            Rejang::LetterLa => LETTER_LA,
            Rejang::LetterYa => LETTER_YA,
            Rejang::LetterWa => LETTER_WA,
            Rejang::LetterHa => LETTER_HA,
            Rejang::LetterMba => LETTER_MBA,
            Rejang::LetterNgga => LETTER_NGGA,
            Rejang::LetterNda => LETTER_NDA,
            Rejang::LetterNyja => LETTER_NYJA,
            Rejang::LetterA => LETTER_A,
            Rejang::VowelSignI => VOWEL_SIGN_I,
            Rejang::VowelSignU => VOWEL_SIGN_U,
            Rejang::VowelSignE => VOWEL_SIGN_E,
            Rejang::VowelSignAi => VOWEL_SIGN_AI,
            Rejang::VowelSignO => VOWEL_SIGN_O,
            Rejang::VowelSignAu => VOWEL_SIGN_AU,
            Rejang::VowelSignEu => VOWEL_SIGN_EU,
            Rejang::VowelSignEa => VOWEL_SIGN_EA,
            Rejang::ConsonantSignNg => CONSONANT_SIGN_NG,
            Rejang::ConsonantSignN => CONSONANT_SIGN_N,
            Rejang::ConsonantSignR => CONSONANT_SIGN_R,
            Rejang::ConsonantSignH => CONSONANT_SIGN_H,
            Rejang::Virama => VIRAMA,
        }
    }
}

impl std::convert::TryFrom<char> for Rejang {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_KA => Ok(Rejang::LetterKa),
            LETTER_GA => Ok(Rejang::LetterGa),
            LETTER_NGA => Ok(Rejang::LetterNga),
            LETTER_TA => Ok(Rejang::LetterTa),
            LETTER_DA => Ok(Rejang::LetterDa),
            LETTER_NA => Ok(Rejang::LetterNa),
            LETTER_PA => Ok(Rejang::LetterPa),
            LETTER_BA => Ok(Rejang::LetterBa),
            LETTER_MA => Ok(Rejang::LetterMa),
            LETTER_CA => Ok(Rejang::LetterCa),
            LETTER_JA => Ok(Rejang::LetterJa),
            LETTER_NYA => Ok(Rejang::LetterNya),
            LETTER_SA => Ok(Rejang::LetterSa),
            LETTER_RA => Ok(Rejang::LetterRa),
            LETTER_LA => Ok(Rejang::LetterLa),
            LETTER_YA => Ok(Rejang::LetterYa),
            LETTER_WA => Ok(Rejang::LetterWa),
            LETTER_HA => Ok(Rejang::LetterHa),
            LETTER_MBA => Ok(Rejang::LetterMba),
            LETTER_NGGA => Ok(Rejang::LetterNgga),
            LETTER_NDA => Ok(Rejang::LetterNda),
            LETTER_NYJA => Ok(Rejang::LetterNyja),
            LETTER_A => Ok(Rejang::LetterA),
            VOWEL_SIGN_I => Ok(Rejang::VowelSignI),
            VOWEL_SIGN_U => Ok(Rejang::VowelSignU),
            VOWEL_SIGN_E => Ok(Rejang::VowelSignE),
            VOWEL_SIGN_AI => Ok(Rejang::VowelSignAi),
            VOWEL_SIGN_O => Ok(Rejang::VowelSignO),
            VOWEL_SIGN_AU => Ok(Rejang::VowelSignAu),
            VOWEL_SIGN_EU => Ok(Rejang::VowelSignEu),
            VOWEL_SIGN_EA => Ok(Rejang::VowelSignEa),
            CONSONANT_SIGN_NG => Ok(Rejang::ConsonantSignNg),
            CONSONANT_SIGN_N => Ok(Rejang::ConsonantSignN),
            CONSONANT_SIGN_R => Ok(Rejang::ConsonantSignR),
            CONSONANT_SIGN_H => Ok(Rejang::ConsonantSignH),
            VIRAMA => Ok(Rejang::Virama),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Rejang {
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

impl std::convert::TryFrom<u32> for Rejang {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Rejang {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Rejang {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Rejang::LetterKa
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Rejang::LetterKa => "rejang letter ka",
            Rejang::LetterGa => "rejang letter ga",
            Rejang::LetterNga => "rejang letter nga",
            Rejang::LetterTa => "rejang letter ta",
            Rejang::LetterDa => "rejang letter da",
            Rejang::LetterNa => "rejang letter na",
            Rejang::LetterPa => "rejang letter pa",
            Rejang::LetterBa => "rejang letter ba",
            Rejang::LetterMa => "rejang letter ma",
            Rejang::LetterCa => "rejang letter ca",
            Rejang::LetterJa => "rejang letter ja",
            Rejang::LetterNya => "rejang letter nya",
            Rejang::LetterSa => "rejang letter sa",
            Rejang::LetterRa => "rejang letter ra",
            Rejang::LetterLa => "rejang letter la",
            Rejang::LetterYa => "rejang letter ya",
            Rejang::LetterWa => "rejang letter wa",
            Rejang::LetterHa => "rejang letter ha",
            Rejang::LetterMba => "rejang letter mba",
            Rejang::LetterNgga => "rejang letter ngga",
            Rejang::LetterNda => "rejang letter nda",
            Rejang::LetterNyja => "rejang letter nyja",
            Rejang::LetterA => "rejang letter a",
            Rejang::VowelSignI => "rejang vowel sign i",
            Rejang::VowelSignU => "rejang vowel sign u",
            Rejang::VowelSignE => "rejang vowel sign e",
            Rejang::VowelSignAi => "rejang vowel sign ai",
            Rejang::VowelSignO => "rejang vowel sign o",
            Rejang::VowelSignAu => "rejang vowel sign au",
            Rejang::VowelSignEu => "rejang vowel sign eu",
            Rejang::VowelSignEa => "rejang vowel sign ea",
            Rejang::ConsonantSignNg => "rejang consonant sign ng",
            Rejang::ConsonantSignN => "rejang consonant sign n",
            Rejang::ConsonantSignR => "rejang consonant sign r",
            Rejang::ConsonantSignH => "rejang consonant sign h",
            Rejang::Virama => "rejang virama",
        }
    }
}
