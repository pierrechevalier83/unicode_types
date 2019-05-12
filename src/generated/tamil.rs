/// \u{b80} → \u{bff}
///
/// ஂ ஃ அ ஆ இ ஈ உ ஊ எ ஏ ஐ ஒ ஓ ஔ க ங\
/// ச ஜ ஞ ட ண த ந ன ப ம ய ர ற ல ள ழ\
/// வ ஶ ஷ ஸ ஹ ா ி ீ ு ூ ெ ே ை ொ ோ ௌ\
/// ் ௐ ௗ ௦ ௧ ௨ ௩ ௪ ௫ ௬ ௭ ௮ ௯ ௰ ௱ ௲\
/// ௳ ௴ ௵ ௶ ௷ ௸ ௹ ௺\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{b82}: 'ஂ'
    pub const SIGN_ANUSVARA: char = 'ஂ';
    /// \u{b83}: 'ஃ'
    pub const SIGN_VISARGA: char = 'ஃ';
    /// \u{b85}: 'அ'
    pub const LETTER_A: char = 'அ';
    /// \u{b86}: 'ஆ'
    pub const LETTER_AA: char = 'ஆ';
    /// \u{b87}: 'இ'
    pub const LETTER_I: char = 'இ';
    /// \u{b88}: 'ஈ'
    pub const LETTER_II: char = 'ஈ';
    /// \u{b89}: 'உ'
    pub const LETTER_U: char = 'உ';
    /// \u{b8a}: 'ஊ'
    pub const LETTER_UU: char = 'ஊ';
    /// \u{b8e}: 'எ'
    pub const LETTER_E: char = 'எ';
    /// \u{b8f}: 'ஏ'
    pub const LETTER_EE: char = 'ஏ';
    /// \u{b90}: 'ஐ'
    pub const LETTER_AI: char = 'ஐ';
    /// \u{b92}: 'ஒ'
    pub const LETTER_O: char = 'ஒ';
    /// \u{b93}: 'ஓ'
    pub const LETTER_OO: char = 'ஓ';
    /// \u{b94}: 'ஔ'
    pub const LETTER_AU: char = 'ஔ';
    /// \u{b95}: 'க'
    pub const LETTER_KA: char = 'க';
    /// \u{b99}: 'ங'
    pub const LETTER_NGA: char = 'ங';
    /// \u{b9a}: 'ச'
    pub const LETTER_CA: char = 'ச';
    /// \u{b9c}: 'ஜ'
    pub const LETTER_JA: char = 'ஜ';
    /// \u{b9e}: 'ஞ'
    pub const LETTER_NYA: char = 'ஞ';
    /// \u{b9f}: 'ட'
    pub const LETTER_TTA: char = 'ட';
    /// \u{ba3}: 'ண'
    pub const LETTER_NNA: char = 'ண';
    /// \u{ba4}: 'த'
    pub const LETTER_TA: char = 'த';
    /// \u{ba8}: 'ந'
    pub const LETTER_NA: char = 'ந';
    /// \u{ba9}: 'ன'
    pub const LETTER_NNNA: char = 'ன';
    /// \u{baa}: 'ப'
    pub const LETTER_PA: char = 'ப';
    /// \u{bae}: 'ம'
    pub const LETTER_MA: char = 'ம';
    /// \u{baf}: 'ய'
    pub const LETTER_YA: char = 'ய';
    /// \u{bb0}: 'ர'
    pub const LETTER_RA: char = 'ர';
    /// \u{bb1}: 'ற'
    pub const LETTER_RRA: char = 'ற';
    /// \u{bb2}: 'ல'
    pub const LETTER_LA: char = 'ல';
    /// \u{bb3}: 'ள'
    pub const LETTER_LLA: char = 'ள';
    /// \u{bb4}: 'ழ'
    pub const LETTER_LLLA: char = 'ழ';
    /// \u{bb5}: 'வ'
    pub const LETTER_VA: char = 'வ';
    /// \u{bb6}: 'ஶ'
    pub const LETTER_SHA: char = 'ஶ';
    /// \u{bb7}: 'ஷ'
    pub const LETTER_SSA: char = 'ஷ';
    /// \u{bb8}: 'ஸ'
    pub const LETTER_SA: char = 'ஸ';
    /// \u{bb9}: 'ஹ'
    pub const LETTER_HA: char = 'ஹ';
    /// \u{bbe}: 'ா'
    pub const VOWEL_SIGN_AA: char = 'ா';
    /// \u{bbf}: 'ி'
    pub const VOWEL_SIGN_I: char = 'ி';
    /// \u{bc0}: 'ீ'
    pub const VOWEL_SIGN_II: char = 'ீ';
    /// \u{bc1}: 'ு'
    pub const VOWEL_SIGN_U: char = 'ு';
    /// \u{bc2}: 'ூ'
    pub const VOWEL_SIGN_UU: char = 'ூ';
    /// \u{bc6}: 'ெ'
    pub const VOWEL_SIGN_E: char = 'ெ';
    /// \u{bc7}: 'ே'
    pub const VOWEL_SIGN_EE: char = 'ே';
    /// \u{bc8}: 'ை'
    pub const VOWEL_SIGN_AI: char = 'ை';
    /// \u{bca}: 'ொ'
    pub const VOWEL_SIGN_O: char = 'ொ';
    /// \u{bcb}: 'ோ'
    pub const VOWEL_SIGN_OO: char = 'ோ';
    /// \u{bcc}: 'ௌ'
    pub const VOWEL_SIGN_AU: char = 'ௌ';
    /// \u{bcd}: '்'
    pub const SIGN_VIRAMA: char = '்';
    /// \u{bd0}: 'ௐ'
    pub const OM: char = 'ௐ';
    /// \u{bd7}: 'ௗ'
    pub const AU_LENGTH_MARK: char = 'ௗ';
    /// \u{be6}: '௦'
    pub const DIGIT_ZERO: char = '௦';
    /// \u{be7}: '௧'
    pub const DIGIT_ONE: char = '௧';
    /// \u{be8}: '௨'
    pub const DIGIT_TWO: char = '௨';
    /// \u{be9}: '௩'
    pub const DIGIT_THREE: char = '௩';
    /// \u{bea}: '௪'
    pub const DIGIT_FOUR: char = '௪';
    /// \u{beb}: '௫'
    pub const DIGIT_FIVE: char = '௫';
    /// \u{bec}: '௬'
    pub const DIGIT_SIX: char = '௬';
    /// \u{bed}: '௭'
    pub const DIGIT_SEVEN: char = '௭';
    /// \u{bee}: '௮'
    pub const DIGIT_EIGHT: char = '௮';
    /// \u{bef}: '௯'
    pub const DIGIT_NINE: char = '௯';
    /// \u{bf0}: '௰'
    pub const NUMBER_TEN: char = '௰';
    /// \u{bf1}: '௱'
    pub const NUMBER_ONE_HUNDRED: char = '௱';
    /// \u{bf2}: '௲'
    pub const NUMBER_ONE_THOUSAND: char = '௲';
    /// \u{bf3}: '௳'
    pub const DAY_SIGN: char = '௳';
    /// \u{bf4}: '௴'
    pub const MONTH_SIGN: char = '௴';
    /// \u{bf5}: '௵'
    pub const YEAR_SIGN: char = '௵';
    /// \u{bf6}: '௶'
    pub const DEBIT_SIGN: char = '௶';
    /// \u{bf7}: '௷'
    pub const CREDIT_SIGN: char = '௷';
    /// \u{bf8}: '௸'
    pub const AS_ABOVE_SIGN: char = '௸';
    /// \u{bf9}: '௹'
    pub const RUPEE_SIGN: char = '௹';
    /// \u{bfa}: '௺'
    pub const NUMBER_SIGN: char = '௺';
}

/// An enum to represent all characters in the Tamil block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Tamil {
    /// \u{b82}: 'ஂ'
    SignAnusvara,
    /// \u{b83}: 'ஃ'
    SignVisarga,
    /// \u{b85}: 'அ'
    LetterA,
    /// \u{b86}: 'ஆ'
    LetterAa,
    /// \u{b87}: 'இ'
    LetterI,
    /// \u{b88}: 'ஈ'
    LetterIi,
    /// \u{b89}: 'உ'
    LetterU,
    /// \u{b8a}: 'ஊ'
    LetterUu,
    /// \u{b8e}: 'எ'
    LetterE,
    /// \u{b8f}: 'ஏ'
    LetterEe,
    /// \u{b90}: 'ஐ'
    LetterAi,
    /// \u{b92}: 'ஒ'
    LetterO,
    /// \u{b93}: 'ஓ'
    LetterOo,
    /// \u{b94}: 'ஔ'
    LetterAu,
    /// \u{b95}: 'க'
    LetterKa,
    /// \u{b99}: 'ங'
    LetterNga,
    /// \u{b9a}: 'ச'
    LetterCa,
    /// \u{b9c}: 'ஜ'
    LetterJa,
    /// \u{b9e}: 'ஞ'
    LetterNya,
    /// \u{b9f}: 'ட'
    LetterTta,
    /// \u{ba3}: 'ண'
    LetterNna,
    /// \u{ba4}: 'த'
    LetterTa,
    /// \u{ba8}: 'ந'
    LetterNa,
    /// \u{ba9}: 'ன'
    LetterNnna,
    /// \u{baa}: 'ப'
    LetterPa,
    /// \u{bae}: 'ம'
    LetterMa,
    /// \u{baf}: 'ய'
    LetterYa,
    /// \u{bb0}: 'ர'
    LetterRa,
    /// \u{bb1}: 'ற'
    LetterRra,
    /// \u{bb2}: 'ல'
    LetterLa,
    /// \u{bb3}: 'ள'
    LetterLla,
    /// \u{bb4}: 'ழ'
    LetterLlla,
    /// \u{bb5}: 'வ'
    LetterVa,
    /// \u{bb6}: 'ஶ'
    LetterSha,
    /// \u{bb7}: 'ஷ'
    LetterSsa,
    /// \u{bb8}: 'ஸ'
    LetterSa,
    /// \u{bb9}: 'ஹ'
    LetterHa,
    /// \u{bbe}: 'ா'
    VowelSignAa,
    /// \u{bbf}: 'ி'
    VowelSignI,
    /// \u{bc0}: 'ீ'
    VowelSignIi,
    /// \u{bc1}: 'ு'
    VowelSignU,
    /// \u{bc2}: 'ூ'
    VowelSignUu,
    /// \u{bc6}: 'ெ'
    VowelSignE,
    /// \u{bc7}: 'ே'
    VowelSignEe,
    /// \u{bc8}: 'ை'
    VowelSignAi,
    /// \u{bca}: 'ொ'
    VowelSignO,
    /// \u{bcb}: 'ோ'
    VowelSignOo,
    /// \u{bcc}: 'ௌ'
    VowelSignAu,
    /// \u{bcd}: '்'
    SignVirama,
    /// \u{bd0}: 'ௐ'
    Om,
    /// \u{bd7}: 'ௗ'
    AuLengthMark,
    /// \u{be6}: '௦'
    DigitZero,
    /// \u{be7}: '௧'
    DigitOne,
    /// \u{be8}: '௨'
    DigitTwo,
    /// \u{be9}: '௩'
    DigitThree,
    /// \u{bea}: '௪'
    DigitFour,
    /// \u{beb}: '௫'
    DigitFive,
    /// \u{bec}: '௬'
    DigitSix,
    /// \u{bed}: '௭'
    DigitSeven,
    /// \u{bee}: '௮'
    DigitEight,
    /// \u{bef}: '௯'
    DigitNine,
    /// \u{bf0}: '௰'
    NumberTen,
    /// \u{bf1}: '௱'
    NumberOneHundred,
    /// \u{bf2}: '௲'
    NumberOneThousand,
    /// \u{bf3}: '௳'
    DaySign,
    /// \u{bf4}: '௴'
    MonthSign,
    /// \u{bf5}: '௵'
    YearSign,
    /// \u{bf6}: '௶'
    DebitSign,
    /// \u{bf7}: '௷'
    CreditSign,
    /// \u{bf8}: '௸'
    AsAboveSign,
    /// \u{bf9}: '௹'
    RupeeSign,
    /// \u{bfa}: '௺'
    NumberSign,
}

impl Into<char> for Tamil {
    fn into(self) -> char {
        use constants::*;
        match self {
            Tamil::SignAnusvara => SIGN_ANUSVARA,
            Tamil::SignVisarga => SIGN_VISARGA,
            Tamil::LetterA => LETTER_A,
            Tamil::LetterAa => LETTER_AA,
            Tamil::LetterI => LETTER_I,
            Tamil::LetterIi => LETTER_II,
            Tamil::LetterU => LETTER_U,
            Tamil::LetterUu => LETTER_UU,
            Tamil::LetterE => LETTER_E,
            Tamil::LetterEe => LETTER_EE,
            Tamil::LetterAi => LETTER_AI,
            Tamil::LetterO => LETTER_O,
            Tamil::LetterOo => LETTER_OO,
            Tamil::LetterAu => LETTER_AU,
            Tamil::LetterKa => LETTER_KA,
            Tamil::LetterNga => LETTER_NGA,
            Tamil::LetterCa => LETTER_CA,
            Tamil::LetterJa => LETTER_JA,
            Tamil::LetterNya => LETTER_NYA,
            Tamil::LetterTta => LETTER_TTA,
            Tamil::LetterNna => LETTER_NNA,
            Tamil::LetterTa => LETTER_TA,
            Tamil::LetterNa => LETTER_NA,
            Tamil::LetterNnna => LETTER_NNNA,
            Tamil::LetterPa => LETTER_PA,
            Tamil::LetterMa => LETTER_MA,
            Tamil::LetterYa => LETTER_YA,
            Tamil::LetterRa => LETTER_RA,
            Tamil::LetterRra => LETTER_RRA,
            Tamil::LetterLa => LETTER_LA,
            Tamil::LetterLla => LETTER_LLA,
            Tamil::LetterLlla => LETTER_LLLA,
            Tamil::LetterVa => LETTER_VA,
            Tamil::LetterSha => LETTER_SHA,
            Tamil::LetterSsa => LETTER_SSA,
            Tamil::LetterSa => LETTER_SA,
            Tamil::LetterHa => LETTER_HA,
            Tamil::VowelSignAa => VOWEL_SIGN_AA,
            Tamil::VowelSignI => VOWEL_SIGN_I,
            Tamil::VowelSignIi => VOWEL_SIGN_II,
            Tamil::VowelSignU => VOWEL_SIGN_U,
            Tamil::VowelSignUu => VOWEL_SIGN_UU,
            Tamil::VowelSignE => VOWEL_SIGN_E,
            Tamil::VowelSignEe => VOWEL_SIGN_EE,
            Tamil::VowelSignAi => VOWEL_SIGN_AI,
            Tamil::VowelSignO => VOWEL_SIGN_O,
            Tamil::VowelSignOo => VOWEL_SIGN_OO,
            Tamil::VowelSignAu => VOWEL_SIGN_AU,
            Tamil::SignVirama => SIGN_VIRAMA,
            Tamil::Om => OM,
            Tamil::AuLengthMark => AU_LENGTH_MARK,
            Tamil::DigitZero => DIGIT_ZERO,
            Tamil::DigitOne => DIGIT_ONE,
            Tamil::DigitTwo => DIGIT_TWO,
            Tamil::DigitThree => DIGIT_THREE,
            Tamil::DigitFour => DIGIT_FOUR,
            Tamil::DigitFive => DIGIT_FIVE,
            Tamil::DigitSix => DIGIT_SIX,
            Tamil::DigitSeven => DIGIT_SEVEN,
            Tamil::DigitEight => DIGIT_EIGHT,
            Tamil::DigitNine => DIGIT_NINE,
            Tamil::NumberTen => NUMBER_TEN,
            Tamil::NumberOneHundred => NUMBER_ONE_HUNDRED,
            Tamil::NumberOneThousand => NUMBER_ONE_THOUSAND,
            Tamil::DaySign => DAY_SIGN,
            Tamil::MonthSign => MONTH_SIGN,
            Tamil::YearSign => YEAR_SIGN,
            Tamil::DebitSign => DEBIT_SIGN,
            Tamil::CreditSign => CREDIT_SIGN,
            Tamil::AsAboveSign => AS_ABOVE_SIGN,
            Tamil::RupeeSign => RUPEE_SIGN,
            Tamil::NumberSign => NUMBER_SIGN,
        }
    }
}

impl std::convert::TryFrom<char> for Tamil {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            SIGN_ANUSVARA => Ok(Tamil::SignAnusvara),
            SIGN_VISARGA => Ok(Tamil::SignVisarga),
            LETTER_A => Ok(Tamil::LetterA),
            LETTER_AA => Ok(Tamil::LetterAa),
            LETTER_I => Ok(Tamil::LetterI),
            LETTER_II => Ok(Tamil::LetterIi),
            LETTER_U => Ok(Tamil::LetterU),
            LETTER_UU => Ok(Tamil::LetterUu),
            LETTER_E => Ok(Tamil::LetterE),
            LETTER_EE => Ok(Tamil::LetterEe),
            LETTER_AI => Ok(Tamil::LetterAi),
            LETTER_O => Ok(Tamil::LetterO),
            LETTER_OO => Ok(Tamil::LetterOo),
            LETTER_AU => Ok(Tamil::LetterAu),
            LETTER_KA => Ok(Tamil::LetterKa),
            LETTER_NGA => Ok(Tamil::LetterNga),
            LETTER_CA => Ok(Tamil::LetterCa),
            LETTER_JA => Ok(Tamil::LetterJa),
            LETTER_NYA => Ok(Tamil::LetterNya),
            LETTER_TTA => Ok(Tamil::LetterTta),
            LETTER_NNA => Ok(Tamil::LetterNna),
            LETTER_TA => Ok(Tamil::LetterTa),
            LETTER_NA => Ok(Tamil::LetterNa),
            LETTER_NNNA => Ok(Tamil::LetterNnna),
            LETTER_PA => Ok(Tamil::LetterPa),
            LETTER_MA => Ok(Tamil::LetterMa),
            LETTER_YA => Ok(Tamil::LetterYa),
            LETTER_RA => Ok(Tamil::LetterRa),
            LETTER_RRA => Ok(Tamil::LetterRra),
            LETTER_LA => Ok(Tamil::LetterLa),
            LETTER_LLA => Ok(Tamil::LetterLla),
            LETTER_LLLA => Ok(Tamil::LetterLlla),
            LETTER_VA => Ok(Tamil::LetterVa),
            LETTER_SHA => Ok(Tamil::LetterSha),
            LETTER_SSA => Ok(Tamil::LetterSsa),
            LETTER_SA => Ok(Tamil::LetterSa),
            LETTER_HA => Ok(Tamil::LetterHa),
            VOWEL_SIGN_AA => Ok(Tamil::VowelSignAa),
            VOWEL_SIGN_I => Ok(Tamil::VowelSignI),
            VOWEL_SIGN_II => Ok(Tamil::VowelSignIi),
            VOWEL_SIGN_U => Ok(Tamil::VowelSignU),
            VOWEL_SIGN_UU => Ok(Tamil::VowelSignUu),
            VOWEL_SIGN_E => Ok(Tamil::VowelSignE),
            VOWEL_SIGN_EE => Ok(Tamil::VowelSignEe),
            VOWEL_SIGN_AI => Ok(Tamil::VowelSignAi),
            VOWEL_SIGN_O => Ok(Tamil::VowelSignO),
            VOWEL_SIGN_OO => Ok(Tamil::VowelSignOo),
            VOWEL_SIGN_AU => Ok(Tamil::VowelSignAu),
            SIGN_VIRAMA => Ok(Tamil::SignVirama),
            OM => Ok(Tamil::Om),
            AU_LENGTH_MARK => Ok(Tamil::AuLengthMark),
            DIGIT_ZERO => Ok(Tamil::DigitZero),
            DIGIT_ONE => Ok(Tamil::DigitOne),
            DIGIT_TWO => Ok(Tamil::DigitTwo),
            DIGIT_THREE => Ok(Tamil::DigitThree),
            DIGIT_FOUR => Ok(Tamil::DigitFour),
            DIGIT_FIVE => Ok(Tamil::DigitFive),
            DIGIT_SIX => Ok(Tamil::DigitSix),
            DIGIT_SEVEN => Ok(Tamil::DigitSeven),
            DIGIT_EIGHT => Ok(Tamil::DigitEight),
            DIGIT_NINE => Ok(Tamil::DigitNine),
            NUMBER_TEN => Ok(Tamil::NumberTen),
            NUMBER_ONE_HUNDRED => Ok(Tamil::NumberOneHundred),
            NUMBER_ONE_THOUSAND => Ok(Tamil::NumberOneThousand),
            DAY_SIGN => Ok(Tamil::DaySign),
            MONTH_SIGN => Ok(Tamil::MonthSign),
            YEAR_SIGN => Ok(Tamil::YearSign),
            DEBIT_SIGN => Ok(Tamil::DebitSign),
            CREDIT_SIGN => Ok(Tamil::CreditSign),
            AS_ABOVE_SIGN => Ok(Tamil::AsAboveSign),
            RUPEE_SIGN => Ok(Tamil::RupeeSign),
            NUMBER_SIGN => Ok(Tamil::NumberSign),
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
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Tamil::SignAnusvara
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Tamil::SignAnusvara => "tamil sign anusvara",
            Tamil::SignVisarga => "tamil sign visarga",
            Tamil::LetterA => "tamil letter a",
            Tamil::LetterAa => "tamil letter aa",
            Tamil::LetterI => "tamil letter i",
            Tamil::LetterIi => "tamil letter ii",
            Tamil::LetterU => "tamil letter u",
            Tamil::LetterUu => "tamil letter uu",
            Tamil::LetterE => "tamil letter e",
            Tamil::LetterEe => "tamil letter ee",
            Tamil::LetterAi => "tamil letter ai",
            Tamil::LetterO => "tamil letter o",
            Tamil::LetterOo => "tamil letter oo",
            Tamil::LetterAu => "tamil letter au",
            Tamil::LetterKa => "tamil letter ka",
            Tamil::LetterNga => "tamil letter nga",
            Tamil::LetterCa => "tamil letter ca",
            Tamil::LetterJa => "tamil letter ja",
            Tamil::LetterNya => "tamil letter nya",
            Tamil::LetterTta => "tamil letter tta",
            Tamil::LetterNna => "tamil letter nna",
            Tamil::LetterTa => "tamil letter ta",
            Tamil::LetterNa => "tamil letter na",
            Tamil::LetterNnna => "tamil letter nnna",
            Tamil::LetterPa => "tamil letter pa",
            Tamil::LetterMa => "tamil letter ma",
            Tamil::LetterYa => "tamil letter ya",
            Tamil::LetterRa => "tamil letter ra",
            Tamil::LetterRra => "tamil letter rra",
            Tamil::LetterLa => "tamil letter la",
            Tamil::LetterLla => "tamil letter lla",
            Tamil::LetterLlla => "tamil letter llla",
            Tamil::LetterVa => "tamil letter va",
            Tamil::LetterSha => "tamil letter sha",
            Tamil::LetterSsa => "tamil letter ssa",
            Tamil::LetterSa => "tamil letter sa",
            Tamil::LetterHa => "tamil letter ha",
            Tamil::VowelSignAa => "tamil vowel sign aa",
            Tamil::VowelSignI => "tamil vowel sign i",
            Tamil::VowelSignIi => "tamil vowel sign ii",
            Tamil::VowelSignU => "tamil vowel sign u",
            Tamil::VowelSignUu => "tamil vowel sign uu",
            Tamil::VowelSignE => "tamil vowel sign e",
            Tamil::VowelSignEe => "tamil vowel sign ee",
            Tamil::VowelSignAi => "tamil vowel sign ai",
            Tamil::VowelSignO => "tamil vowel sign o",
            Tamil::VowelSignOo => "tamil vowel sign oo",
            Tamil::VowelSignAu => "tamil vowel sign au",
            Tamil::SignVirama => "tamil sign virama",
            Tamil::Om => "tamil om",
            Tamil::AuLengthMark => "tamil au length mark",
            Tamil::DigitZero => "tamil digit zero",
            Tamil::DigitOne => "tamil digit one",
            Tamil::DigitTwo => "tamil digit two",
            Tamil::DigitThree => "tamil digit three",
            Tamil::DigitFour => "tamil digit four",
            Tamil::DigitFive => "tamil digit five",
            Tamil::DigitSix => "tamil digit six",
            Tamil::DigitSeven => "tamil digit seven",
            Tamil::DigitEight => "tamil digit eight",
            Tamil::DigitNine => "tamil digit nine",
            Tamil::NumberTen => "tamil number ten",
            Tamil::NumberOneHundred => "tamil number one hundred",
            Tamil::NumberOneThousand => "tamil number one thousand",
            Tamil::DaySign => "tamil day sign",
            Tamil::MonthSign => "tamil month sign",
            Tamil::YearSign => "tamil year sign",
            Tamil::DebitSign => "tamil debit sign",
            Tamil::CreditSign => "tamil credit sign",
            Tamil::AsAboveSign => "tamil as above sign",
            Tamil::RupeeSign => "tamil rupee sign",
            Tamil::NumberSign => "tamil number sign",
        }
    }
}
