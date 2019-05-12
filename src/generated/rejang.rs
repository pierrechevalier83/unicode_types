/// \u{a930} → \u{a95f}\
///\
/// ꤰ ꤱ ꤲ ꤳ ꤴ ꤵ ꤶ ꤷ ꤸ ꤹ ꤺ ꤻ ꤼ ꤽ ꤾ ꤿ
/// ꥀ ꥁ ꥂ ꥃ ꥄ ꥅ ꥆ ꥇ ꥈ ꥉ ꥊ ꥋ ꥌ ꥍ ꥎ ꥏ
/// ꥐ ꥑ ꥒ ꥓
pub mod constants {
    /// \u{a930}: 'ꤰ'
    pub const REJANG_LETTER_KA: char = 'ꤰ';
    /// \u{a931}: 'ꤱ'
    pub const REJANG_LETTER_GA: char = 'ꤱ';
    /// \u{a932}: 'ꤲ'
    pub const REJANG_LETTER_NGA: char = 'ꤲ';
    /// \u{a933}: 'ꤳ'
    pub const REJANG_LETTER_TA: char = 'ꤳ';
    /// \u{a934}: 'ꤴ'
    pub const REJANG_LETTER_DA: char = 'ꤴ';
    /// \u{a935}: 'ꤵ'
    pub const REJANG_LETTER_NA: char = 'ꤵ';
    /// \u{a936}: 'ꤶ'
    pub const REJANG_LETTER_PA: char = 'ꤶ';
    /// \u{a937}: 'ꤷ'
    pub const REJANG_LETTER_BA: char = 'ꤷ';
    /// \u{a938}: 'ꤸ'
    pub const REJANG_LETTER_MA: char = 'ꤸ';
    /// \u{a939}: 'ꤹ'
    pub const REJANG_LETTER_CA: char = 'ꤹ';
    /// \u{a93a}: 'ꤺ'
    pub const REJANG_LETTER_JA: char = 'ꤺ';
    /// \u{a93b}: 'ꤻ'
    pub const REJANG_LETTER_NYA: char = 'ꤻ';
    /// \u{a93c}: 'ꤼ'
    pub const REJANG_LETTER_SA: char = 'ꤼ';
    /// \u{a93d}: 'ꤽ'
    pub const REJANG_LETTER_RA: char = 'ꤽ';
    /// \u{a93e}: 'ꤾ'
    pub const REJANG_LETTER_LA: char = 'ꤾ';
    /// \u{a93f}: 'ꤿ'
    pub const REJANG_LETTER_YA: char = 'ꤿ';
    /// \u{a940}: 'ꥀ'
    pub const REJANG_LETTER_WA: char = 'ꥀ';
    /// \u{a941}: 'ꥁ'
    pub const REJANG_LETTER_HA: char = 'ꥁ';
    /// \u{a942}: 'ꥂ'
    pub const REJANG_LETTER_MBA: char = 'ꥂ';
    /// \u{a943}: 'ꥃ'
    pub const REJANG_LETTER_NGGA: char = 'ꥃ';
    /// \u{a944}: 'ꥄ'
    pub const REJANG_LETTER_NDA: char = 'ꥄ';
    /// \u{a945}: 'ꥅ'
    pub const REJANG_LETTER_NYJA: char = 'ꥅ';
    /// \u{a946}: 'ꥆ'
    pub const REJANG_LETTER_A: char = 'ꥆ';
    /// \u{a947}: 'ꥇ'
    pub const REJANG_VOWEL_SIGN_I: char = 'ꥇ';
    /// \u{a948}: 'ꥈ'
    pub const REJANG_VOWEL_SIGN_U: char = 'ꥈ';
    /// \u{a949}: 'ꥉ'
    pub const REJANG_VOWEL_SIGN_E: char = 'ꥉ';
    /// \u{a94a}: 'ꥊ'
    pub const REJANG_VOWEL_SIGN_AI: char = 'ꥊ';
    /// \u{a94b}: 'ꥋ'
    pub const REJANG_VOWEL_SIGN_O: char = 'ꥋ';
    /// \u{a94c}: 'ꥌ'
    pub const REJANG_VOWEL_SIGN_AU: char = 'ꥌ';
    /// \u{a94d}: 'ꥍ'
    pub const REJANG_VOWEL_SIGN_EU: char = 'ꥍ';
    /// \u{a94e}: 'ꥎ'
    pub const REJANG_VOWEL_SIGN_EA: char = 'ꥎ';
    /// \u{a94f}: 'ꥏ'
    pub const REJANG_CONSONANT_SIGN_NG: char = 'ꥏ';
    /// \u{a950}: 'ꥐ'
    pub const REJANG_CONSONANT_SIGN_N: char = 'ꥐ';
    /// \u{a951}: 'ꥑ'
    pub const REJANG_CONSONANT_SIGN_R: char = 'ꥑ';
    /// \u{a952}: 'ꥒ'
    pub const REJANG_CONSONANT_SIGN_H: char = 'ꥒ';
    /// \u{a953}: '꥓'
    pub const REJANG_VIRAMA: char = '꥓';
}

/// \u{a930} → \u{a95f}\
///\
/// ꤰ ꤱ ꤲ ꤳ ꤴ ꤵ ꤶ ꤷ ꤸ ꤹ ꤺ ꤻ ꤼ ꤽ ꤾ ꤿ
/// ꥀ ꥁ ꥂ ꥃ ꥄ ꥅ ꥆ ꥇ ꥈ ꥉ ꥊ ꥋ ꥌ ꥍ ꥎ ꥏ
/// ꥐ ꥑ ꥒ ꥓
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Rejang {
    /// \u{a930}: 'ꤰ'
    RejangLetterKa,
    /// \u{a931}: 'ꤱ'
    RejangLetterGa,
    /// \u{a932}: 'ꤲ'
    RejangLetterNga,
    /// \u{a933}: 'ꤳ'
    RejangLetterTa,
    /// \u{a934}: 'ꤴ'
    RejangLetterDa,
    /// \u{a935}: 'ꤵ'
    RejangLetterNa,
    /// \u{a936}: 'ꤶ'
    RejangLetterPa,
    /// \u{a937}: 'ꤷ'
    RejangLetterBa,
    /// \u{a938}: 'ꤸ'
    RejangLetterMa,
    /// \u{a939}: 'ꤹ'
    RejangLetterCa,
    /// \u{a93a}: 'ꤺ'
    RejangLetterJa,
    /// \u{a93b}: 'ꤻ'
    RejangLetterNya,
    /// \u{a93c}: 'ꤼ'
    RejangLetterSa,
    /// \u{a93d}: 'ꤽ'
    RejangLetterRa,
    /// \u{a93e}: 'ꤾ'
    RejangLetterLa,
    /// \u{a93f}: 'ꤿ'
    RejangLetterYa,
    /// \u{a940}: 'ꥀ'
    RejangLetterWa,
    /// \u{a941}: 'ꥁ'
    RejangLetterHa,
    /// \u{a942}: 'ꥂ'
    RejangLetterMba,
    /// \u{a943}: 'ꥃ'
    RejangLetterNgga,
    /// \u{a944}: 'ꥄ'
    RejangLetterNda,
    /// \u{a945}: 'ꥅ'
    RejangLetterNyja,
    /// \u{a946}: 'ꥆ'
    RejangLetterA,
    /// \u{a947}: 'ꥇ'
    RejangVowelSignI,
    /// \u{a948}: 'ꥈ'
    RejangVowelSignU,
    /// \u{a949}: 'ꥉ'
    RejangVowelSignE,
    /// \u{a94a}: 'ꥊ'
    RejangVowelSignAi,
    /// \u{a94b}: 'ꥋ'
    RejangVowelSignO,
    /// \u{a94c}: 'ꥌ'
    RejangVowelSignAu,
    /// \u{a94d}: 'ꥍ'
    RejangVowelSignEu,
    /// \u{a94e}: 'ꥎ'
    RejangVowelSignEa,
    /// \u{a94f}: 'ꥏ'
    RejangConsonantSignNg,
    /// \u{a950}: 'ꥐ'
    RejangConsonantSignN,
    /// \u{a951}: 'ꥑ'
    RejangConsonantSignR,
    /// \u{a952}: 'ꥒ'
    RejangConsonantSignH,
    /// \u{a953}: '꥓'
    RejangVirama,
}

impl Into<char> for Rejang {
    fn into(self) -> char {
        use constants::*;
        match self {
            Rejang::RejangLetterKa => REJANG_LETTER_KA,
            Rejang::RejangLetterGa => REJANG_LETTER_GA,
            Rejang::RejangLetterNga => REJANG_LETTER_NGA,
            Rejang::RejangLetterTa => REJANG_LETTER_TA,
            Rejang::RejangLetterDa => REJANG_LETTER_DA,
            Rejang::RejangLetterNa => REJANG_LETTER_NA,
            Rejang::RejangLetterPa => REJANG_LETTER_PA,
            Rejang::RejangLetterBa => REJANG_LETTER_BA,
            Rejang::RejangLetterMa => REJANG_LETTER_MA,
            Rejang::RejangLetterCa => REJANG_LETTER_CA,
            Rejang::RejangLetterJa => REJANG_LETTER_JA,
            Rejang::RejangLetterNya => REJANG_LETTER_NYA,
            Rejang::RejangLetterSa => REJANG_LETTER_SA,
            Rejang::RejangLetterRa => REJANG_LETTER_RA,
            Rejang::RejangLetterLa => REJANG_LETTER_LA,
            Rejang::RejangLetterYa => REJANG_LETTER_YA,
            Rejang::RejangLetterWa => REJANG_LETTER_WA,
            Rejang::RejangLetterHa => REJANG_LETTER_HA,
            Rejang::RejangLetterMba => REJANG_LETTER_MBA,
            Rejang::RejangLetterNgga => REJANG_LETTER_NGGA,
            Rejang::RejangLetterNda => REJANG_LETTER_NDA,
            Rejang::RejangLetterNyja => REJANG_LETTER_NYJA,
            Rejang::RejangLetterA => REJANG_LETTER_A,
            Rejang::RejangVowelSignI => REJANG_VOWEL_SIGN_I,
            Rejang::RejangVowelSignU => REJANG_VOWEL_SIGN_U,
            Rejang::RejangVowelSignE => REJANG_VOWEL_SIGN_E,
            Rejang::RejangVowelSignAi => REJANG_VOWEL_SIGN_AI,
            Rejang::RejangVowelSignO => REJANG_VOWEL_SIGN_O,
            Rejang::RejangVowelSignAu => REJANG_VOWEL_SIGN_AU,
            Rejang::RejangVowelSignEu => REJANG_VOWEL_SIGN_EU,
            Rejang::RejangVowelSignEa => REJANG_VOWEL_SIGN_EA,
            Rejang::RejangConsonantSignNg => REJANG_CONSONANT_SIGN_NG,
            Rejang::RejangConsonantSignN => REJANG_CONSONANT_SIGN_N,
            Rejang::RejangConsonantSignR => REJANG_CONSONANT_SIGN_R,
            Rejang::RejangConsonantSignH => REJANG_CONSONANT_SIGN_H,
            Rejang::RejangVirama => REJANG_VIRAMA,
        }
    }
}

impl std::convert::TryFrom<char> for Rejang {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            REJANG_LETTER_KA => Ok(Rejang::RejangLetterKa),
            REJANG_LETTER_GA => Ok(Rejang::RejangLetterGa),
            REJANG_LETTER_NGA => Ok(Rejang::RejangLetterNga),
            REJANG_LETTER_TA => Ok(Rejang::RejangLetterTa),
            REJANG_LETTER_DA => Ok(Rejang::RejangLetterDa),
            REJANG_LETTER_NA => Ok(Rejang::RejangLetterNa),
            REJANG_LETTER_PA => Ok(Rejang::RejangLetterPa),
            REJANG_LETTER_BA => Ok(Rejang::RejangLetterBa),
            REJANG_LETTER_MA => Ok(Rejang::RejangLetterMa),
            REJANG_LETTER_CA => Ok(Rejang::RejangLetterCa),
            REJANG_LETTER_JA => Ok(Rejang::RejangLetterJa),
            REJANG_LETTER_NYA => Ok(Rejang::RejangLetterNya),
            REJANG_LETTER_SA => Ok(Rejang::RejangLetterSa),
            REJANG_LETTER_RA => Ok(Rejang::RejangLetterRa),
            REJANG_LETTER_LA => Ok(Rejang::RejangLetterLa),
            REJANG_LETTER_YA => Ok(Rejang::RejangLetterYa),
            REJANG_LETTER_WA => Ok(Rejang::RejangLetterWa),
            REJANG_LETTER_HA => Ok(Rejang::RejangLetterHa),
            REJANG_LETTER_MBA => Ok(Rejang::RejangLetterMba),
            REJANG_LETTER_NGGA => Ok(Rejang::RejangLetterNgga),
            REJANG_LETTER_NDA => Ok(Rejang::RejangLetterNda),
            REJANG_LETTER_NYJA => Ok(Rejang::RejangLetterNyja),
            REJANG_LETTER_A => Ok(Rejang::RejangLetterA),
            REJANG_VOWEL_SIGN_I => Ok(Rejang::RejangVowelSignI),
            REJANG_VOWEL_SIGN_U => Ok(Rejang::RejangVowelSignU),
            REJANG_VOWEL_SIGN_E => Ok(Rejang::RejangVowelSignE),
            REJANG_VOWEL_SIGN_AI => Ok(Rejang::RejangVowelSignAi),
            REJANG_VOWEL_SIGN_O => Ok(Rejang::RejangVowelSignO),
            REJANG_VOWEL_SIGN_AU => Ok(Rejang::RejangVowelSignAu),
            REJANG_VOWEL_SIGN_EU => Ok(Rejang::RejangVowelSignEu),
            REJANG_VOWEL_SIGN_EA => Ok(Rejang::RejangVowelSignEa),
            REJANG_CONSONANT_SIGN_NG => Ok(Rejang::RejangConsonantSignNg),
            REJANG_CONSONANT_SIGN_N => Ok(Rejang::RejangConsonantSignN),
            REJANG_CONSONANT_SIGN_R => Ok(Rejang::RejangConsonantSignR),
            REJANG_CONSONANT_SIGN_H => Ok(Rejang::RejangConsonantSignH),
            REJANG_VIRAMA => Ok(Rejang::RejangVirama),
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
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        Rejang::RejangLetterKa
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Rejang::RejangLetterKa => "rejang letter ka",
            Rejang::RejangLetterGa => "rejang letter ga",
            Rejang::RejangLetterNga => "rejang letter nga",
            Rejang::RejangLetterTa => "rejang letter ta",
            Rejang::RejangLetterDa => "rejang letter da",
            Rejang::RejangLetterNa => "rejang letter na",
            Rejang::RejangLetterPa => "rejang letter pa",
            Rejang::RejangLetterBa => "rejang letter ba",
            Rejang::RejangLetterMa => "rejang letter ma",
            Rejang::RejangLetterCa => "rejang letter ca",
            Rejang::RejangLetterJa => "rejang letter ja",
            Rejang::RejangLetterNya => "rejang letter nya",
            Rejang::RejangLetterSa => "rejang letter sa",
            Rejang::RejangLetterRa => "rejang letter ra",
            Rejang::RejangLetterLa => "rejang letter la",
            Rejang::RejangLetterYa => "rejang letter ya",
            Rejang::RejangLetterWa => "rejang letter wa",
            Rejang::RejangLetterHa => "rejang letter ha",
            Rejang::RejangLetterMba => "rejang letter mba",
            Rejang::RejangLetterNgga => "rejang letter ngga",
            Rejang::RejangLetterNda => "rejang letter nda",
            Rejang::RejangLetterNyja => "rejang letter nyja",
            Rejang::RejangLetterA => "rejang letter a",
            Rejang::RejangVowelSignI => "rejang vowel sign i",
            Rejang::RejangVowelSignU => "rejang vowel sign u",
            Rejang::RejangVowelSignE => "rejang vowel sign e",
            Rejang::RejangVowelSignAi => "rejang vowel sign ai",
            Rejang::RejangVowelSignO => "rejang vowel sign o",
            Rejang::RejangVowelSignAu => "rejang vowel sign au",
            Rejang::RejangVowelSignEu => "rejang vowel sign eu",
            Rejang::RejangVowelSignEa => "rejang vowel sign ea",
            Rejang::RejangConsonantSignNg => "rejang consonant sign ng",
            Rejang::RejangConsonantSignN => "rejang consonant sign n",
            Rejang::RejangConsonantSignR => "rejang consonant sign r",
            Rejang::RejangConsonantSignH => "rejang consonant sign h",
            Rejang::RejangVirama => "rejang virama",
        }
    }
}
