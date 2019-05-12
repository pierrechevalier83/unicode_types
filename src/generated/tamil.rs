/// \u{b80} → \u{bff}\
///\
/// ஂ ஃ அ ஆ இ ஈ உ ஊ எ ஏ ஐ ஒ ஓ ஔ க ங
/// ச ஜ ஞ ட ண த ந ன ப ம ய ர ற ல ள ழ
/// வ ஶ ஷ ஸ ஹ ா ி ீ ு ூ ெ ே ை ொ ோ ௌ
/// ் ௐ ௗ ௦ ௧ ௨ ௩ ௪ ௫ ௬ ௭ ௮ ௯ ௰ ௱ ௲
/// ௳ ௴ ௵ ௶ ௷ ௸ ௹ ௺
pub mod constants {
    /// \u{b82}: 'ஂ'
    pub const TAMIL_SIGN_ANUSVARA: char = 'ஂ';
    /// \u{b83}: 'ஃ'
    pub const TAMIL_SIGN_VISARGA: char = 'ஃ';
    /// \u{b85}: 'அ'
    pub const TAMIL_LETTER_A: char = 'அ';
    /// \u{b86}: 'ஆ'
    pub const TAMIL_LETTER_AA: char = 'ஆ';
    /// \u{b87}: 'இ'
    pub const TAMIL_LETTER_I: char = 'இ';
    /// \u{b88}: 'ஈ'
    pub const TAMIL_LETTER_II: char = 'ஈ';
    /// \u{b89}: 'உ'
    pub const TAMIL_LETTER_U: char = 'உ';
    /// \u{b8a}: 'ஊ'
    pub const TAMIL_LETTER_UU: char = 'ஊ';
    /// \u{b8e}: 'எ'
    pub const TAMIL_LETTER_E: char = 'எ';
    /// \u{b8f}: 'ஏ'
    pub const TAMIL_LETTER_EE: char = 'ஏ';
    /// \u{b90}: 'ஐ'
    pub const TAMIL_LETTER_AI: char = 'ஐ';
    /// \u{b92}: 'ஒ'
    pub const TAMIL_LETTER_O: char = 'ஒ';
    /// \u{b93}: 'ஓ'
    pub const TAMIL_LETTER_OO: char = 'ஓ';
    /// \u{b94}: 'ஔ'
    pub const TAMIL_LETTER_AU: char = 'ஔ';
    /// \u{b95}: 'க'
    pub const TAMIL_LETTER_KA: char = 'க';
    /// \u{b99}: 'ங'
    pub const TAMIL_LETTER_NGA: char = 'ங';
    /// \u{b9a}: 'ச'
    pub const TAMIL_LETTER_CA: char = 'ச';
    /// \u{b9c}: 'ஜ'
    pub const TAMIL_LETTER_JA: char = 'ஜ';
    /// \u{b9e}: 'ஞ'
    pub const TAMIL_LETTER_NYA: char = 'ஞ';
    /// \u{b9f}: 'ட'
    pub const TAMIL_LETTER_TTA: char = 'ட';
    /// \u{ba3}: 'ண'
    pub const TAMIL_LETTER_NNA: char = 'ண';
    /// \u{ba4}: 'த'
    pub const TAMIL_LETTER_TA: char = 'த';
    /// \u{ba8}: 'ந'
    pub const TAMIL_LETTER_NA: char = 'ந';
    /// \u{ba9}: 'ன'
    pub const TAMIL_LETTER_NNNA: char = 'ன';
    /// \u{baa}: 'ப'
    pub const TAMIL_LETTER_PA: char = 'ப';
    /// \u{bae}: 'ம'
    pub const TAMIL_LETTER_MA: char = 'ம';
    /// \u{baf}: 'ய'
    pub const TAMIL_LETTER_YA: char = 'ய';
    /// \u{bb0}: 'ர'
    pub const TAMIL_LETTER_RA: char = 'ர';
    /// \u{bb1}: 'ற'
    pub const TAMIL_LETTER_RRA: char = 'ற';
    /// \u{bb2}: 'ல'
    pub const TAMIL_LETTER_LA: char = 'ல';
    /// \u{bb3}: 'ள'
    pub const TAMIL_LETTER_LLA: char = 'ள';
    /// \u{bb4}: 'ழ'
    pub const TAMIL_LETTER_LLLA: char = 'ழ';
    /// \u{bb5}: 'வ'
    pub const TAMIL_LETTER_VA: char = 'வ';
    /// \u{bb6}: 'ஶ'
    pub const TAMIL_LETTER_SHA: char = 'ஶ';
    /// \u{bb7}: 'ஷ'
    pub const TAMIL_LETTER_SSA: char = 'ஷ';
    /// \u{bb8}: 'ஸ'
    pub const TAMIL_LETTER_SA: char = 'ஸ';
    /// \u{bb9}: 'ஹ'
    pub const TAMIL_LETTER_HA: char = 'ஹ';
    /// \u{bbe}: 'ா'
    pub const TAMIL_VOWEL_SIGN_AA: char = 'ா';
    /// \u{bbf}: 'ி'
    pub const TAMIL_VOWEL_SIGN_I: char = 'ி';
    /// \u{bc0}: 'ீ'
    pub const TAMIL_VOWEL_SIGN_II: char = 'ீ';
    /// \u{bc1}: 'ு'
    pub const TAMIL_VOWEL_SIGN_U: char = 'ு';
    /// \u{bc2}: 'ூ'
    pub const TAMIL_VOWEL_SIGN_UU: char = 'ூ';
    /// \u{bc6}: 'ெ'
    pub const TAMIL_VOWEL_SIGN_E: char = 'ெ';
    /// \u{bc7}: 'ே'
    pub const TAMIL_VOWEL_SIGN_EE: char = 'ே';
    /// \u{bc8}: 'ை'
    pub const TAMIL_VOWEL_SIGN_AI: char = 'ை';
    /// \u{bca}: 'ொ'
    pub const TAMIL_VOWEL_SIGN_O: char = 'ொ';
    /// \u{bcb}: 'ோ'
    pub const TAMIL_VOWEL_SIGN_OO: char = 'ோ';
    /// \u{bcc}: 'ௌ'
    pub const TAMIL_VOWEL_SIGN_AU: char = 'ௌ';
    /// \u{bcd}: '்'
    pub const TAMIL_SIGN_VIRAMA: char = '்';
    /// \u{bd0}: 'ௐ'
    pub const TAMIL_OM: char = 'ௐ';
    /// \u{bd7}: 'ௗ'
    pub const TAMIL_AU_LENGTH_MARK: char = 'ௗ';
    /// \u{be6}: '௦'
    pub const TAMIL_DIGIT_ZERO: char = '௦';
    /// \u{be7}: '௧'
    pub const TAMIL_DIGIT_ONE: char = '௧';
    /// \u{be8}: '௨'
    pub const TAMIL_DIGIT_TWO: char = '௨';
    /// \u{be9}: '௩'
    pub const TAMIL_DIGIT_THREE: char = '௩';
    /// \u{bea}: '௪'
    pub const TAMIL_DIGIT_FOUR: char = '௪';
    /// \u{beb}: '௫'
    pub const TAMIL_DIGIT_FIVE: char = '௫';
    /// \u{bec}: '௬'
    pub const TAMIL_DIGIT_SIX: char = '௬';
    /// \u{bed}: '௭'
    pub const TAMIL_DIGIT_SEVEN: char = '௭';
    /// \u{bee}: '௮'
    pub const TAMIL_DIGIT_EIGHT: char = '௮';
    /// \u{bef}: '௯'
    pub const TAMIL_DIGIT_NINE: char = '௯';
    /// \u{bf0}: '௰'
    pub const TAMIL_NUMBER_TEN: char = '௰';
    /// \u{bf1}: '௱'
    pub const TAMIL_NUMBER_ONE_HUNDRED: char = '௱';
    /// \u{bf2}: '௲'
    pub const TAMIL_NUMBER_ONE_THOUSAND: char = '௲';
    /// \u{bf3}: '௳'
    pub const TAMIL_DAY_SIGN: char = '௳';
    /// \u{bf4}: '௴'
    pub const TAMIL_MONTH_SIGN: char = '௴';
    /// \u{bf5}: '௵'
    pub const TAMIL_YEAR_SIGN: char = '௵';
    /// \u{bf6}: '௶'
    pub const TAMIL_DEBIT_SIGN: char = '௶';
    /// \u{bf7}: '௷'
    pub const TAMIL_CREDIT_SIGN: char = '௷';
    /// \u{bf8}: '௸'
    pub const TAMIL_AS_ABOVE_SIGN: char = '௸';
    /// \u{bf9}: '௹'
    pub const TAMIL_RUPEE_SIGN: char = '௹';
    /// \u{bfa}: '௺'
    pub const TAMIL_NUMBER_SIGN: char = '௺';
}

/// \u{b80} → \u{bff}\
///\
/// ஂ ஃ அ ஆ இ ஈ உ ஊ எ ஏ ஐ ஒ ஓ ஔ க ங
/// ச ஜ ஞ ட ண த ந ன ப ம ய ர ற ல ள ழ
/// வ ஶ ஷ ஸ ஹ ா ி ீ ு ூ ெ ே ை ொ ோ ௌ
/// ் ௐ ௗ ௦ ௧ ௨ ௩ ௪ ௫ ௬ ௭ ௮ ௯ ௰ ௱ ௲
/// ௳ ௴ ௵ ௶ ௷ ௸ ௹ ௺
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Tamil {
    /// \u{b82}: 'ஂ'
    TamilSignAnusvara,
    /// \u{b83}: 'ஃ'
    TamilSignVisarga,
    /// \u{b85}: 'அ'
    TamilLetterA,
    /// \u{b86}: 'ஆ'
    TamilLetterAa,
    /// \u{b87}: 'இ'
    TamilLetterI,
    /// \u{b88}: 'ஈ'
    TamilLetterIi,
    /// \u{b89}: 'உ'
    TamilLetterU,
    /// \u{b8a}: 'ஊ'
    TamilLetterUu,
    /// \u{b8e}: 'எ'
    TamilLetterE,
    /// \u{b8f}: 'ஏ'
    TamilLetterEe,
    /// \u{b90}: 'ஐ'
    TamilLetterAi,
    /// \u{b92}: 'ஒ'
    TamilLetterO,
    /// \u{b93}: 'ஓ'
    TamilLetterOo,
    /// \u{b94}: 'ஔ'
    TamilLetterAu,
    /// \u{b95}: 'க'
    TamilLetterKa,
    /// \u{b99}: 'ங'
    TamilLetterNga,
    /// \u{b9a}: 'ச'
    TamilLetterCa,
    /// \u{b9c}: 'ஜ'
    TamilLetterJa,
    /// \u{b9e}: 'ஞ'
    TamilLetterNya,
    /// \u{b9f}: 'ட'
    TamilLetterTta,
    /// \u{ba3}: 'ண'
    TamilLetterNna,
    /// \u{ba4}: 'த'
    TamilLetterTa,
    /// \u{ba8}: 'ந'
    TamilLetterNa,
    /// \u{ba9}: 'ன'
    TamilLetterNnna,
    /// \u{baa}: 'ப'
    TamilLetterPa,
    /// \u{bae}: 'ம'
    TamilLetterMa,
    /// \u{baf}: 'ய'
    TamilLetterYa,
    /// \u{bb0}: 'ர'
    TamilLetterRa,
    /// \u{bb1}: 'ற'
    TamilLetterRra,
    /// \u{bb2}: 'ல'
    TamilLetterLa,
    /// \u{bb3}: 'ள'
    TamilLetterLla,
    /// \u{bb4}: 'ழ'
    TamilLetterLlla,
    /// \u{bb5}: 'வ'
    TamilLetterVa,
    /// \u{bb6}: 'ஶ'
    TamilLetterSha,
    /// \u{bb7}: 'ஷ'
    TamilLetterSsa,
    /// \u{bb8}: 'ஸ'
    TamilLetterSa,
    /// \u{bb9}: 'ஹ'
    TamilLetterHa,
    /// \u{bbe}: 'ா'
    TamilVowelSignAa,
    /// \u{bbf}: 'ி'
    TamilVowelSignI,
    /// \u{bc0}: 'ீ'
    TamilVowelSignIi,
    /// \u{bc1}: 'ு'
    TamilVowelSignU,
    /// \u{bc2}: 'ூ'
    TamilVowelSignUu,
    /// \u{bc6}: 'ெ'
    TamilVowelSignE,
    /// \u{bc7}: 'ே'
    TamilVowelSignEe,
    /// \u{bc8}: 'ை'
    TamilVowelSignAi,
    /// \u{bca}: 'ொ'
    TamilVowelSignO,
    /// \u{bcb}: 'ோ'
    TamilVowelSignOo,
    /// \u{bcc}: 'ௌ'
    TamilVowelSignAu,
    /// \u{bcd}: '்'
    TamilSignVirama,
    /// \u{bd0}: 'ௐ'
    TamilOm,
    /// \u{bd7}: 'ௗ'
    TamilAuLengthMark,
    /// \u{be6}: '௦'
    TamilDigitZero,
    /// \u{be7}: '௧'
    TamilDigitOne,
    /// \u{be8}: '௨'
    TamilDigitTwo,
    /// \u{be9}: '௩'
    TamilDigitThree,
    /// \u{bea}: '௪'
    TamilDigitFour,
    /// \u{beb}: '௫'
    TamilDigitFive,
    /// \u{bec}: '௬'
    TamilDigitSix,
    /// \u{bed}: '௭'
    TamilDigitSeven,
    /// \u{bee}: '௮'
    TamilDigitEight,
    /// \u{bef}: '௯'
    TamilDigitNine,
    /// \u{bf0}: '௰'
    TamilNumberTen,
    /// \u{bf1}: '௱'
    TamilNumberOneHundred,
    /// \u{bf2}: '௲'
    TamilNumberOneThousand,
    /// \u{bf3}: '௳'
    TamilDaySign,
    /// \u{bf4}: '௴'
    TamilMonthSign,
    /// \u{bf5}: '௵'
    TamilYearSign,
    /// \u{bf6}: '௶'
    TamilDebitSign,
    /// \u{bf7}: '௷'
    TamilCreditSign,
    /// \u{bf8}: '௸'
    TamilAsAboveSign,
    /// \u{bf9}: '௹'
    TamilRupeeSign,
    /// \u{bfa}: '௺'
    TamilNumberSign,
}

impl Into<char> for Tamil {
    fn into(self) -> char {
        use constants::*;
        match self {
            Tamil::TamilSignAnusvara => TAMIL_SIGN_ANUSVARA,
            Tamil::TamilSignVisarga => TAMIL_SIGN_VISARGA,
            Tamil::TamilLetterA => TAMIL_LETTER_A,
            Tamil::TamilLetterAa => TAMIL_LETTER_AA,
            Tamil::TamilLetterI => TAMIL_LETTER_I,
            Tamil::TamilLetterIi => TAMIL_LETTER_II,
            Tamil::TamilLetterU => TAMIL_LETTER_U,
            Tamil::TamilLetterUu => TAMIL_LETTER_UU,
            Tamil::TamilLetterE => TAMIL_LETTER_E,
            Tamil::TamilLetterEe => TAMIL_LETTER_EE,
            Tamil::TamilLetterAi => TAMIL_LETTER_AI,
            Tamil::TamilLetterO => TAMIL_LETTER_O,
            Tamil::TamilLetterOo => TAMIL_LETTER_OO,
            Tamil::TamilLetterAu => TAMIL_LETTER_AU,
            Tamil::TamilLetterKa => TAMIL_LETTER_KA,
            Tamil::TamilLetterNga => TAMIL_LETTER_NGA,
            Tamil::TamilLetterCa => TAMIL_LETTER_CA,
            Tamil::TamilLetterJa => TAMIL_LETTER_JA,
            Tamil::TamilLetterNya => TAMIL_LETTER_NYA,
            Tamil::TamilLetterTta => TAMIL_LETTER_TTA,
            Tamil::TamilLetterNna => TAMIL_LETTER_NNA,
            Tamil::TamilLetterTa => TAMIL_LETTER_TA,
            Tamil::TamilLetterNa => TAMIL_LETTER_NA,
            Tamil::TamilLetterNnna => TAMIL_LETTER_NNNA,
            Tamil::TamilLetterPa => TAMIL_LETTER_PA,
            Tamil::TamilLetterMa => TAMIL_LETTER_MA,
            Tamil::TamilLetterYa => TAMIL_LETTER_YA,
            Tamil::TamilLetterRa => TAMIL_LETTER_RA,
            Tamil::TamilLetterRra => TAMIL_LETTER_RRA,
            Tamil::TamilLetterLa => TAMIL_LETTER_LA,
            Tamil::TamilLetterLla => TAMIL_LETTER_LLA,
            Tamil::TamilLetterLlla => TAMIL_LETTER_LLLA,
            Tamil::TamilLetterVa => TAMIL_LETTER_VA,
            Tamil::TamilLetterSha => TAMIL_LETTER_SHA,
            Tamil::TamilLetterSsa => TAMIL_LETTER_SSA,
            Tamil::TamilLetterSa => TAMIL_LETTER_SA,
            Tamil::TamilLetterHa => TAMIL_LETTER_HA,
            Tamil::TamilVowelSignAa => TAMIL_VOWEL_SIGN_AA,
            Tamil::TamilVowelSignI => TAMIL_VOWEL_SIGN_I,
            Tamil::TamilVowelSignIi => TAMIL_VOWEL_SIGN_II,
            Tamil::TamilVowelSignU => TAMIL_VOWEL_SIGN_U,
            Tamil::TamilVowelSignUu => TAMIL_VOWEL_SIGN_UU,
            Tamil::TamilVowelSignE => TAMIL_VOWEL_SIGN_E,
            Tamil::TamilVowelSignEe => TAMIL_VOWEL_SIGN_EE,
            Tamil::TamilVowelSignAi => TAMIL_VOWEL_SIGN_AI,
            Tamil::TamilVowelSignO => TAMIL_VOWEL_SIGN_O,
            Tamil::TamilVowelSignOo => TAMIL_VOWEL_SIGN_OO,
            Tamil::TamilVowelSignAu => TAMIL_VOWEL_SIGN_AU,
            Tamil::TamilSignVirama => TAMIL_SIGN_VIRAMA,
            Tamil::TamilOm => TAMIL_OM,
            Tamil::TamilAuLengthMark => TAMIL_AU_LENGTH_MARK,
            Tamil::TamilDigitZero => TAMIL_DIGIT_ZERO,
            Tamil::TamilDigitOne => TAMIL_DIGIT_ONE,
            Tamil::TamilDigitTwo => TAMIL_DIGIT_TWO,
            Tamil::TamilDigitThree => TAMIL_DIGIT_THREE,
            Tamil::TamilDigitFour => TAMIL_DIGIT_FOUR,
            Tamil::TamilDigitFive => TAMIL_DIGIT_FIVE,
            Tamil::TamilDigitSix => TAMIL_DIGIT_SIX,
            Tamil::TamilDigitSeven => TAMIL_DIGIT_SEVEN,
            Tamil::TamilDigitEight => TAMIL_DIGIT_EIGHT,
            Tamil::TamilDigitNine => TAMIL_DIGIT_NINE,
            Tamil::TamilNumberTen => TAMIL_NUMBER_TEN,
            Tamil::TamilNumberOneHundred => TAMIL_NUMBER_ONE_HUNDRED,
            Tamil::TamilNumberOneThousand => TAMIL_NUMBER_ONE_THOUSAND,
            Tamil::TamilDaySign => TAMIL_DAY_SIGN,
            Tamil::TamilMonthSign => TAMIL_MONTH_SIGN,
            Tamil::TamilYearSign => TAMIL_YEAR_SIGN,
            Tamil::TamilDebitSign => TAMIL_DEBIT_SIGN,
            Tamil::TamilCreditSign => TAMIL_CREDIT_SIGN,
            Tamil::TamilAsAboveSign => TAMIL_AS_ABOVE_SIGN,
            Tamil::TamilRupeeSign => TAMIL_RUPEE_SIGN,
            Tamil::TamilNumberSign => TAMIL_NUMBER_SIGN,
        }
    }
}

impl std::convert::TryFrom<char> for Tamil {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            TAMIL_SIGN_ANUSVARA => Ok(Tamil::TamilSignAnusvara),
            TAMIL_SIGN_VISARGA => Ok(Tamil::TamilSignVisarga),
            TAMIL_LETTER_A => Ok(Tamil::TamilLetterA),
            TAMIL_LETTER_AA => Ok(Tamil::TamilLetterAa),
            TAMIL_LETTER_I => Ok(Tamil::TamilLetterI),
            TAMIL_LETTER_II => Ok(Tamil::TamilLetterIi),
            TAMIL_LETTER_U => Ok(Tamil::TamilLetterU),
            TAMIL_LETTER_UU => Ok(Tamil::TamilLetterUu),
            TAMIL_LETTER_E => Ok(Tamil::TamilLetterE),
            TAMIL_LETTER_EE => Ok(Tamil::TamilLetterEe),
            TAMIL_LETTER_AI => Ok(Tamil::TamilLetterAi),
            TAMIL_LETTER_O => Ok(Tamil::TamilLetterO),
            TAMIL_LETTER_OO => Ok(Tamil::TamilLetterOo),
            TAMIL_LETTER_AU => Ok(Tamil::TamilLetterAu),
            TAMIL_LETTER_KA => Ok(Tamil::TamilLetterKa),
            TAMIL_LETTER_NGA => Ok(Tamil::TamilLetterNga),
            TAMIL_LETTER_CA => Ok(Tamil::TamilLetterCa),
            TAMIL_LETTER_JA => Ok(Tamil::TamilLetterJa),
            TAMIL_LETTER_NYA => Ok(Tamil::TamilLetterNya),
            TAMIL_LETTER_TTA => Ok(Tamil::TamilLetterTta),
            TAMIL_LETTER_NNA => Ok(Tamil::TamilLetterNna),
            TAMIL_LETTER_TA => Ok(Tamil::TamilLetterTa),
            TAMIL_LETTER_NA => Ok(Tamil::TamilLetterNa),
            TAMIL_LETTER_NNNA => Ok(Tamil::TamilLetterNnna),
            TAMIL_LETTER_PA => Ok(Tamil::TamilLetterPa),
            TAMIL_LETTER_MA => Ok(Tamil::TamilLetterMa),
            TAMIL_LETTER_YA => Ok(Tamil::TamilLetterYa),
            TAMIL_LETTER_RA => Ok(Tamil::TamilLetterRa),
            TAMIL_LETTER_RRA => Ok(Tamil::TamilLetterRra),
            TAMIL_LETTER_LA => Ok(Tamil::TamilLetterLa),
            TAMIL_LETTER_LLA => Ok(Tamil::TamilLetterLla),
            TAMIL_LETTER_LLLA => Ok(Tamil::TamilLetterLlla),
            TAMIL_LETTER_VA => Ok(Tamil::TamilLetterVa),
            TAMIL_LETTER_SHA => Ok(Tamil::TamilLetterSha),
            TAMIL_LETTER_SSA => Ok(Tamil::TamilLetterSsa),
            TAMIL_LETTER_SA => Ok(Tamil::TamilLetterSa),
            TAMIL_LETTER_HA => Ok(Tamil::TamilLetterHa),
            TAMIL_VOWEL_SIGN_AA => Ok(Tamil::TamilVowelSignAa),
            TAMIL_VOWEL_SIGN_I => Ok(Tamil::TamilVowelSignI),
            TAMIL_VOWEL_SIGN_II => Ok(Tamil::TamilVowelSignIi),
            TAMIL_VOWEL_SIGN_U => Ok(Tamil::TamilVowelSignU),
            TAMIL_VOWEL_SIGN_UU => Ok(Tamil::TamilVowelSignUu),
            TAMIL_VOWEL_SIGN_E => Ok(Tamil::TamilVowelSignE),
            TAMIL_VOWEL_SIGN_EE => Ok(Tamil::TamilVowelSignEe),
            TAMIL_VOWEL_SIGN_AI => Ok(Tamil::TamilVowelSignAi),
            TAMIL_VOWEL_SIGN_O => Ok(Tamil::TamilVowelSignO),
            TAMIL_VOWEL_SIGN_OO => Ok(Tamil::TamilVowelSignOo),
            TAMIL_VOWEL_SIGN_AU => Ok(Tamil::TamilVowelSignAu),
            TAMIL_SIGN_VIRAMA => Ok(Tamil::TamilSignVirama),
            TAMIL_OM => Ok(Tamil::TamilOm),
            TAMIL_AU_LENGTH_MARK => Ok(Tamil::TamilAuLengthMark),
            TAMIL_DIGIT_ZERO => Ok(Tamil::TamilDigitZero),
            TAMIL_DIGIT_ONE => Ok(Tamil::TamilDigitOne),
            TAMIL_DIGIT_TWO => Ok(Tamil::TamilDigitTwo),
            TAMIL_DIGIT_THREE => Ok(Tamil::TamilDigitThree),
            TAMIL_DIGIT_FOUR => Ok(Tamil::TamilDigitFour),
            TAMIL_DIGIT_FIVE => Ok(Tamil::TamilDigitFive),
            TAMIL_DIGIT_SIX => Ok(Tamil::TamilDigitSix),
            TAMIL_DIGIT_SEVEN => Ok(Tamil::TamilDigitSeven),
            TAMIL_DIGIT_EIGHT => Ok(Tamil::TamilDigitEight),
            TAMIL_DIGIT_NINE => Ok(Tamil::TamilDigitNine),
            TAMIL_NUMBER_TEN => Ok(Tamil::TamilNumberTen),
            TAMIL_NUMBER_ONE_HUNDRED => Ok(Tamil::TamilNumberOneHundred),
            TAMIL_NUMBER_ONE_THOUSAND => Ok(Tamil::TamilNumberOneThousand),
            TAMIL_DAY_SIGN => Ok(Tamil::TamilDaySign),
            TAMIL_MONTH_SIGN => Ok(Tamil::TamilMonthSign),
            TAMIL_YEAR_SIGN => Ok(Tamil::TamilYearSign),
            TAMIL_DEBIT_SIGN => Ok(Tamil::TamilDebitSign),
            TAMIL_CREDIT_SIGN => Ok(Tamil::TamilCreditSign),
            TAMIL_AS_ABOVE_SIGN => Ok(Tamil::TamilAsAboveSign),
            TAMIL_RUPEE_SIGN => Ok(Tamil::TamilRupeeSign),
            TAMIL_NUMBER_SIGN => Ok(Tamil::TamilNumberSign),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Tamil {
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

impl std::convert::TryFrom<u32> for Tamil {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Tamil {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Tamil {
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        Tamil::TamilSignAnusvara
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Tamil::TamilSignAnusvara => "tamil sign anusvara",
            Tamil::TamilSignVisarga => "tamil sign visarga",
            Tamil::TamilLetterA => "tamil letter a",
            Tamil::TamilLetterAa => "tamil letter aa",
            Tamil::TamilLetterI => "tamil letter i",
            Tamil::TamilLetterIi => "tamil letter ii",
            Tamil::TamilLetterU => "tamil letter u",
            Tamil::TamilLetterUu => "tamil letter uu",
            Tamil::TamilLetterE => "tamil letter e",
            Tamil::TamilLetterEe => "tamil letter ee",
            Tamil::TamilLetterAi => "tamil letter ai",
            Tamil::TamilLetterO => "tamil letter o",
            Tamil::TamilLetterOo => "tamil letter oo",
            Tamil::TamilLetterAu => "tamil letter au",
            Tamil::TamilLetterKa => "tamil letter ka",
            Tamil::TamilLetterNga => "tamil letter nga",
            Tamil::TamilLetterCa => "tamil letter ca",
            Tamil::TamilLetterJa => "tamil letter ja",
            Tamil::TamilLetterNya => "tamil letter nya",
            Tamil::TamilLetterTta => "tamil letter tta",
            Tamil::TamilLetterNna => "tamil letter nna",
            Tamil::TamilLetterTa => "tamil letter ta",
            Tamil::TamilLetterNa => "tamil letter na",
            Tamil::TamilLetterNnna => "tamil letter nnna",
            Tamil::TamilLetterPa => "tamil letter pa",
            Tamil::TamilLetterMa => "tamil letter ma",
            Tamil::TamilLetterYa => "tamil letter ya",
            Tamil::TamilLetterRa => "tamil letter ra",
            Tamil::TamilLetterRra => "tamil letter rra",
            Tamil::TamilLetterLa => "tamil letter la",
            Tamil::TamilLetterLla => "tamil letter lla",
            Tamil::TamilLetterLlla => "tamil letter llla",
            Tamil::TamilLetterVa => "tamil letter va",
            Tamil::TamilLetterSha => "tamil letter sha",
            Tamil::TamilLetterSsa => "tamil letter ssa",
            Tamil::TamilLetterSa => "tamil letter sa",
            Tamil::TamilLetterHa => "tamil letter ha",
            Tamil::TamilVowelSignAa => "tamil vowel sign aa",
            Tamil::TamilVowelSignI => "tamil vowel sign i",
            Tamil::TamilVowelSignIi => "tamil vowel sign ii",
            Tamil::TamilVowelSignU => "tamil vowel sign u",
            Tamil::TamilVowelSignUu => "tamil vowel sign uu",
            Tamil::TamilVowelSignE => "tamil vowel sign e",
            Tamil::TamilVowelSignEe => "tamil vowel sign ee",
            Tamil::TamilVowelSignAi => "tamil vowel sign ai",
            Tamil::TamilVowelSignO => "tamil vowel sign o",
            Tamil::TamilVowelSignOo => "tamil vowel sign oo",
            Tamil::TamilVowelSignAu => "tamil vowel sign au",
            Tamil::TamilSignVirama => "tamil sign virama",
            Tamil::TamilOm => "tamil om",
            Tamil::TamilAuLengthMark => "tamil au length mark",
            Tamil::TamilDigitZero => "tamil digit zero",
            Tamil::TamilDigitOne => "tamil digit one",
            Tamil::TamilDigitTwo => "tamil digit two",
            Tamil::TamilDigitThree => "tamil digit three",
            Tamil::TamilDigitFour => "tamil digit four",
            Tamil::TamilDigitFive => "tamil digit five",
            Tamil::TamilDigitSix => "tamil digit six",
            Tamil::TamilDigitSeven => "tamil digit seven",
            Tamil::TamilDigitEight => "tamil digit eight",
            Tamil::TamilDigitNine => "tamil digit nine",
            Tamil::TamilNumberTen => "tamil number ten",
            Tamil::TamilNumberOneHundred => "tamil number one hundred",
            Tamil::TamilNumberOneThousand => "tamil number one thousand",
            Tamil::TamilDaySign => "tamil day sign",
            Tamil::TamilMonthSign => "tamil month sign",
            Tamil::TamilYearSign => "tamil year sign",
            Tamil::TamilDebitSign => "tamil debit sign",
            Tamil::TamilCreditSign => "tamil credit sign",
            Tamil::TamilAsAboveSign => "tamil as above sign",
            Tamil::TamilRupeeSign => "tamil rupee sign",
            Tamil::TamilNumberSign => "tamil number sign",
        }
    }
}
