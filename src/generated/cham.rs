/// \u{aa00} → \u{aa5f}\
///\
/// ꨀ ꨁ ꨂ ꨃ ꨄ ꨅ ꨆ ꨇ ꨈ ꨉ ꨊ ꨋ ꨌ ꨍ ꨎ ꨏ\
/// ꨐ ꨑ ꨒ ꨓ ꨔ ꨕ ꨖ ꨗ ꨘ ꨙ ꨚ ꨛ ꨜ ꨝ ꨞ ꨟ\
/// ꨠ ꨡ ꨢ ꨣ ꨤ ꨥ ꨦ ꨧ ꨨ ꨩ ꨪ ꨫ ꨬ ꨭ ꨮ ꨯ\
/// ꨰ ꨱ ꨲ ꨳ ꨴ ꨵ ꨶ ꩀ ꩁ ꩂ ꩃ ꩄ ꩅ ꩆ ꩇ ꩈ\
/// ꩉ ꩊ ꩋ ꩌ ꩍ ꩐ ꩑ ꩒ ꩓ ꩔ ꩕ ꩖ ꩗ ꩘ ꩙ ꩜\
/// ꩝ ꩞\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{aa00}: 'ꨀ'
    pub const LETTER_A: char = 'ꨀ';
    /// \u{aa01}: 'ꨁ'
    pub const LETTER_I: char = 'ꨁ';
    /// \u{aa02}: 'ꨂ'
    pub const LETTER_U: char = 'ꨂ';
    /// \u{aa03}: 'ꨃ'
    pub const LETTER_E: char = 'ꨃ';
    /// \u{aa04}: 'ꨄ'
    pub const LETTER_AI: char = 'ꨄ';
    /// \u{aa05}: 'ꨅ'
    pub const LETTER_O: char = 'ꨅ';
    /// \u{aa06}: 'ꨆ'
    pub const LETTER_KA: char = 'ꨆ';
    /// \u{aa07}: 'ꨇ'
    pub const LETTER_KHA: char = 'ꨇ';
    /// \u{aa08}: 'ꨈ'
    pub const LETTER_GA: char = 'ꨈ';
    /// \u{aa09}: 'ꨉ'
    pub const LETTER_GHA: char = 'ꨉ';
    /// \u{aa0a}: 'ꨊ'
    pub const LETTER_NGUE: char = 'ꨊ';
    /// \u{aa0b}: 'ꨋ'
    pub const LETTER_NGA: char = 'ꨋ';
    /// \u{aa0c}: 'ꨌ'
    pub const LETTER_CHA: char = 'ꨌ';
    /// \u{aa0d}: 'ꨍ'
    pub const LETTER_CHHA: char = 'ꨍ';
    /// \u{aa0e}: 'ꨎ'
    pub const LETTER_JA: char = 'ꨎ';
    /// \u{aa0f}: 'ꨏ'
    pub const LETTER_JHA: char = 'ꨏ';
    /// \u{aa10}: 'ꨐ'
    pub const LETTER_NHUE: char = 'ꨐ';
    /// \u{aa11}: 'ꨑ'
    pub const LETTER_NHA: char = 'ꨑ';
    /// \u{aa12}: 'ꨒ'
    pub const LETTER_NHJA: char = 'ꨒ';
    /// \u{aa13}: 'ꨓ'
    pub const LETTER_TA: char = 'ꨓ';
    /// \u{aa14}: 'ꨔ'
    pub const LETTER_THA: char = 'ꨔ';
    /// \u{aa15}: 'ꨕ'
    pub const LETTER_DA: char = 'ꨕ';
    /// \u{aa16}: 'ꨖ'
    pub const LETTER_DHA: char = 'ꨖ';
    /// \u{aa17}: 'ꨗ'
    pub const LETTER_NUE: char = 'ꨗ';
    /// \u{aa18}: 'ꨘ'
    pub const LETTER_NA: char = 'ꨘ';
    /// \u{aa19}: 'ꨙ'
    pub const LETTER_DDA: char = 'ꨙ';
    /// \u{aa1a}: 'ꨚ'
    pub const LETTER_PA: char = 'ꨚ';
    /// \u{aa1b}: 'ꨛ'
    pub const LETTER_PPA: char = 'ꨛ';
    /// \u{aa1c}: 'ꨜ'
    pub const LETTER_PHA: char = 'ꨜ';
    /// \u{aa1d}: 'ꨝ'
    pub const LETTER_BA: char = 'ꨝ';
    /// \u{aa1e}: 'ꨞ'
    pub const LETTER_BHA: char = 'ꨞ';
    /// \u{aa1f}: 'ꨟ'
    pub const LETTER_MUE: char = 'ꨟ';
    /// \u{aa20}: 'ꨠ'
    pub const LETTER_MA: char = 'ꨠ';
    /// \u{aa21}: 'ꨡ'
    pub const LETTER_BBA: char = 'ꨡ';
    /// \u{aa22}: 'ꨢ'
    pub const LETTER_YA: char = 'ꨢ';
    /// \u{aa23}: 'ꨣ'
    pub const LETTER_RA: char = 'ꨣ';
    /// \u{aa24}: 'ꨤ'
    pub const LETTER_LA: char = 'ꨤ';
    /// \u{aa25}: 'ꨥ'
    pub const LETTER_VA: char = 'ꨥ';
    /// \u{aa26}: 'ꨦ'
    pub const LETTER_SSA: char = 'ꨦ';
    /// \u{aa27}: 'ꨧ'
    pub const LETTER_SA: char = 'ꨧ';
    /// \u{aa28}: 'ꨨ'
    pub const LETTER_HA: char = 'ꨨ';
    /// \u{aa29}: 'ꨩ'
    pub const VOWEL_SIGN_AA: char = 'ꨩ';
    /// \u{aa2a}: 'ꨪ'
    pub const VOWEL_SIGN_I: char = 'ꨪ';
    /// \u{aa2b}: 'ꨫ'
    pub const VOWEL_SIGN_II: char = 'ꨫ';
    /// \u{aa2c}: 'ꨬ'
    pub const VOWEL_SIGN_EI: char = 'ꨬ';
    /// \u{aa2d}: 'ꨭ'
    pub const VOWEL_SIGN_U: char = 'ꨭ';
    /// \u{aa2e}: 'ꨮ'
    pub const VOWEL_SIGN_OE: char = 'ꨮ';
    /// \u{aa2f}: 'ꨯ'
    pub const VOWEL_SIGN_O: char = 'ꨯ';
    /// \u{aa30}: 'ꨰ'
    pub const VOWEL_SIGN_AI: char = 'ꨰ';
    /// \u{aa31}: 'ꨱ'
    pub const VOWEL_SIGN_AU: char = 'ꨱ';
    /// \u{aa32}: 'ꨲ'
    pub const VOWEL_SIGN_UE: char = 'ꨲ';
    /// \u{aa33}: 'ꨳ'
    pub const CONSONANT_SIGN_YA: char = 'ꨳ';
    /// \u{aa34}: 'ꨴ'
    pub const CONSONANT_SIGN_RA: char = 'ꨴ';
    /// \u{aa35}: 'ꨵ'
    pub const CONSONANT_SIGN_LA: char = 'ꨵ';
    /// \u{aa36}: 'ꨶ'
    pub const CONSONANT_SIGN_WA: char = 'ꨶ';
    /// \u{aa40}: 'ꩀ'
    pub const LETTER_FINAL_K: char = 'ꩀ';
    /// \u{aa41}: 'ꩁ'
    pub const LETTER_FINAL_G: char = 'ꩁ';
    /// \u{aa42}: 'ꩂ'
    pub const LETTER_FINAL_NG: char = 'ꩂ';
    /// \u{aa43}: 'ꩃ'
    pub const CONSONANT_SIGN_FINAL_NG: char = 'ꩃ';
    /// \u{aa44}: 'ꩄ'
    pub const LETTER_FINAL_CH: char = 'ꩄ';
    /// \u{aa45}: 'ꩅ'
    pub const LETTER_FINAL_T: char = 'ꩅ';
    /// \u{aa46}: 'ꩆ'
    pub const LETTER_FINAL_N: char = 'ꩆ';
    /// \u{aa47}: 'ꩇ'
    pub const LETTER_FINAL_P: char = 'ꩇ';
    /// \u{aa48}: 'ꩈ'
    pub const LETTER_FINAL_Y: char = 'ꩈ';
    /// \u{aa49}: 'ꩉ'
    pub const LETTER_FINAL_R: char = 'ꩉ';
    /// \u{aa4a}: 'ꩊ'
    pub const LETTER_FINAL_L: char = 'ꩊ';
    /// \u{aa4b}: 'ꩋ'
    pub const LETTER_FINAL_SS: char = 'ꩋ';
    /// \u{aa4c}: 'ꩌ'
    pub const CONSONANT_SIGN_FINAL_M: char = 'ꩌ';
    /// \u{aa4d}: 'ꩍ'
    pub const CONSONANT_SIGN_FINAL_H: char = 'ꩍ';
    /// \u{aa50}: '꩐'
    pub const DIGIT_ZERO: char = '꩐';
    /// \u{aa51}: '꩑'
    pub const DIGIT_ONE: char = '꩑';
    /// \u{aa52}: '꩒'
    pub const DIGIT_TWO: char = '꩒';
    /// \u{aa53}: '꩓'
    pub const DIGIT_THREE: char = '꩓';
    /// \u{aa54}: '꩔'
    pub const DIGIT_FOUR: char = '꩔';
    /// \u{aa55}: '꩕'
    pub const DIGIT_FIVE: char = '꩕';
    /// \u{aa56}: '꩖'
    pub const DIGIT_SIX: char = '꩖';
    /// \u{aa57}: '꩗'
    pub const DIGIT_SEVEN: char = '꩗';
    /// \u{aa58}: '꩘'
    pub const DIGIT_EIGHT: char = '꩘';
    /// \u{aa59}: '꩙'
    pub const DIGIT_NINE: char = '꩙';
    /// \u{aa5c}: '꩜'
    pub const PUNCTUATION_SPIRAL: char = '꩜';
    /// \u{aa5d}: '꩝'
    pub const PUNCTUATION_DANDA: char = '꩝';
    /// \u{aa5e}: '꩞'
    pub const PUNCTUATION_DOUBLE_DANDA: char = '꩞';
}

/// An enum to represent all characters in the Cham block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Cham {
    /// \u{aa00}: 'ꨀ'
    LetterA,
    /// \u{aa01}: 'ꨁ'
    LetterI,
    /// \u{aa02}: 'ꨂ'
    LetterU,
    /// \u{aa03}: 'ꨃ'
    LetterE,
    /// \u{aa04}: 'ꨄ'
    LetterAi,
    /// \u{aa05}: 'ꨅ'
    LetterO,
    /// \u{aa06}: 'ꨆ'
    LetterKa,
    /// \u{aa07}: 'ꨇ'
    LetterKha,
    /// \u{aa08}: 'ꨈ'
    LetterGa,
    /// \u{aa09}: 'ꨉ'
    LetterGha,
    /// \u{aa0a}: 'ꨊ'
    LetterNgue,
    /// \u{aa0b}: 'ꨋ'
    LetterNga,
    /// \u{aa0c}: 'ꨌ'
    LetterCha,
    /// \u{aa0d}: 'ꨍ'
    LetterChha,
    /// \u{aa0e}: 'ꨎ'
    LetterJa,
    /// \u{aa0f}: 'ꨏ'
    LetterJha,
    /// \u{aa10}: 'ꨐ'
    LetterNhue,
    /// \u{aa11}: 'ꨑ'
    LetterNha,
    /// \u{aa12}: 'ꨒ'
    LetterNhja,
    /// \u{aa13}: 'ꨓ'
    LetterTa,
    /// \u{aa14}: 'ꨔ'
    LetterTha,
    /// \u{aa15}: 'ꨕ'
    LetterDa,
    /// \u{aa16}: 'ꨖ'
    LetterDha,
    /// \u{aa17}: 'ꨗ'
    LetterNue,
    /// \u{aa18}: 'ꨘ'
    LetterNa,
    /// \u{aa19}: 'ꨙ'
    LetterDda,
    /// \u{aa1a}: 'ꨚ'
    LetterPa,
    /// \u{aa1b}: 'ꨛ'
    LetterPpa,
    /// \u{aa1c}: 'ꨜ'
    LetterPha,
    /// \u{aa1d}: 'ꨝ'
    LetterBa,
    /// \u{aa1e}: 'ꨞ'
    LetterBha,
    /// \u{aa1f}: 'ꨟ'
    LetterMue,
    /// \u{aa20}: 'ꨠ'
    LetterMa,
    /// \u{aa21}: 'ꨡ'
    LetterBba,
    /// \u{aa22}: 'ꨢ'
    LetterYa,
    /// \u{aa23}: 'ꨣ'
    LetterRa,
    /// \u{aa24}: 'ꨤ'
    LetterLa,
    /// \u{aa25}: 'ꨥ'
    LetterVa,
    /// \u{aa26}: 'ꨦ'
    LetterSsa,
    /// \u{aa27}: 'ꨧ'
    LetterSa,
    /// \u{aa28}: 'ꨨ'
    LetterHa,
    /// \u{aa29}: 'ꨩ'
    VowelSignAa,
    /// \u{aa2a}: 'ꨪ'
    VowelSignI,
    /// \u{aa2b}: 'ꨫ'
    VowelSignIi,
    /// \u{aa2c}: 'ꨬ'
    VowelSignEi,
    /// \u{aa2d}: 'ꨭ'
    VowelSignU,
    /// \u{aa2e}: 'ꨮ'
    VowelSignOe,
    /// \u{aa2f}: 'ꨯ'
    VowelSignO,
    /// \u{aa30}: 'ꨰ'
    VowelSignAi,
    /// \u{aa31}: 'ꨱ'
    VowelSignAu,
    /// \u{aa32}: 'ꨲ'
    VowelSignUe,
    /// \u{aa33}: 'ꨳ'
    ConsonantSignYa,
    /// \u{aa34}: 'ꨴ'
    ConsonantSignRa,
    /// \u{aa35}: 'ꨵ'
    ConsonantSignLa,
    /// \u{aa36}: 'ꨶ'
    ConsonantSignWa,
    /// \u{aa40}: 'ꩀ'
    LetterFinalK,
    /// \u{aa41}: 'ꩁ'
    LetterFinalG,
    /// \u{aa42}: 'ꩂ'
    LetterFinalNg,
    /// \u{aa43}: 'ꩃ'
    ConsonantSignFinalNg,
    /// \u{aa44}: 'ꩄ'
    LetterFinalCh,
    /// \u{aa45}: 'ꩅ'
    LetterFinalT,
    /// \u{aa46}: 'ꩆ'
    LetterFinalN,
    /// \u{aa47}: 'ꩇ'
    LetterFinalP,
    /// \u{aa48}: 'ꩈ'
    LetterFinalY,
    /// \u{aa49}: 'ꩉ'
    LetterFinalR,
    /// \u{aa4a}: 'ꩊ'
    LetterFinalL,
    /// \u{aa4b}: 'ꩋ'
    LetterFinalSs,
    /// \u{aa4c}: 'ꩌ'
    ConsonantSignFinalM,
    /// \u{aa4d}: 'ꩍ'
    ConsonantSignFinalH,
    /// \u{aa50}: '꩐'
    DigitZero,
    /// \u{aa51}: '꩑'
    DigitOne,
    /// \u{aa52}: '꩒'
    DigitTwo,
    /// \u{aa53}: '꩓'
    DigitThree,
    /// \u{aa54}: '꩔'
    DigitFour,
    /// \u{aa55}: '꩕'
    DigitFive,
    /// \u{aa56}: '꩖'
    DigitSix,
    /// \u{aa57}: '꩗'
    DigitSeven,
    /// \u{aa58}: '꩘'
    DigitEight,
    /// \u{aa59}: '꩙'
    DigitNine,
    /// \u{aa5c}: '꩜'
    PunctuationSpiral,
    /// \u{aa5d}: '꩝'
    PunctuationDanda,
    /// \u{aa5e}: '꩞'
    PunctuationDoubleDanda,
}

impl Into<char> for Cham {
    fn into(self) -> char {
        use constants::*;
        match self {
            Cham::LetterA => LETTER_A,
            Cham::LetterI => LETTER_I,
            Cham::LetterU => LETTER_U,
            Cham::LetterE => LETTER_E,
            Cham::LetterAi => LETTER_AI,
            Cham::LetterO => LETTER_O,
            Cham::LetterKa => LETTER_KA,
            Cham::LetterKha => LETTER_KHA,
            Cham::LetterGa => LETTER_GA,
            Cham::LetterGha => LETTER_GHA,
            Cham::LetterNgue => LETTER_NGUE,
            Cham::LetterNga => LETTER_NGA,
            Cham::LetterCha => LETTER_CHA,
            Cham::LetterChha => LETTER_CHHA,
            Cham::LetterJa => LETTER_JA,
            Cham::LetterJha => LETTER_JHA,
            Cham::LetterNhue => LETTER_NHUE,
            Cham::LetterNha => LETTER_NHA,
            Cham::LetterNhja => LETTER_NHJA,
            Cham::LetterTa => LETTER_TA,
            Cham::LetterTha => LETTER_THA,
            Cham::LetterDa => LETTER_DA,
            Cham::LetterDha => LETTER_DHA,
            Cham::LetterNue => LETTER_NUE,
            Cham::LetterNa => LETTER_NA,
            Cham::LetterDda => LETTER_DDA,
            Cham::LetterPa => LETTER_PA,
            Cham::LetterPpa => LETTER_PPA,
            Cham::LetterPha => LETTER_PHA,
            Cham::LetterBa => LETTER_BA,
            Cham::LetterBha => LETTER_BHA,
            Cham::LetterMue => LETTER_MUE,
            Cham::LetterMa => LETTER_MA,
            Cham::LetterBba => LETTER_BBA,
            Cham::LetterYa => LETTER_YA,
            Cham::LetterRa => LETTER_RA,
            Cham::LetterLa => LETTER_LA,
            Cham::LetterVa => LETTER_VA,
            Cham::LetterSsa => LETTER_SSA,
            Cham::LetterSa => LETTER_SA,
            Cham::LetterHa => LETTER_HA,
            Cham::VowelSignAa => VOWEL_SIGN_AA,
            Cham::VowelSignI => VOWEL_SIGN_I,
            Cham::VowelSignIi => VOWEL_SIGN_II,
            Cham::VowelSignEi => VOWEL_SIGN_EI,
            Cham::VowelSignU => VOWEL_SIGN_U,
            Cham::VowelSignOe => VOWEL_SIGN_OE,
            Cham::VowelSignO => VOWEL_SIGN_O,
            Cham::VowelSignAi => VOWEL_SIGN_AI,
            Cham::VowelSignAu => VOWEL_SIGN_AU,
            Cham::VowelSignUe => VOWEL_SIGN_UE,
            Cham::ConsonantSignYa => CONSONANT_SIGN_YA,
            Cham::ConsonantSignRa => CONSONANT_SIGN_RA,
            Cham::ConsonantSignLa => CONSONANT_SIGN_LA,
            Cham::ConsonantSignWa => CONSONANT_SIGN_WA,
            Cham::LetterFinalK => LETTER_FINAL_K,
            Cham::LetterFinalG => LETTER_FINAL_G,
            Cham::LetterFinalNg => LETTER_FINAL_NG,
            Cham::ConsonantSignFinalNg => CONSONANT_SIGN_FINAL_NG,
            Cham::LetterFinalCh => LETTER_FINAL_CH,
            Cham::LetterFinalT => LETTER_FINAL_T,
            Cham::LetterFinalN => LETTER_FINAL_N,
            Cham::LetterFinalP => LETTER_FINAL_P,
            Cham::LetterFinalY => LETTER_FINAL_Y,
            Cham::LetterFinalR => LETTER_FINAL_R,
            Cham::LetterFinalL => LETTER_FINAL_L,
            Cham::LetterFinalSs => LETTER_FINAL_SS,
            Cham::ConsonantSignFinalM => CONSONANT_SIGN_FINAL_M,
            Cham::ConsonantSignFinalH => CONSONANT_SIGN_FINAL_H,
            Cham::DigitZero => DIGIT_ZERO,
            Cham::DigitOne => DIGIT_ONE,
            Cham::DigitTwo => DIGIT_TWO,
            Cham::DigitThree => DIGIT_THREE,
            Cham::DigitFour => DIGIT_FOUR,
            Cham::DigitFive => DIGIT_FIVE,
            Cham::DigitSix => DIGIT_SIX,
            Cham::DigitSeven => DIGIT_SEVEN,
            Cham::DigitEight => DIGIT_EIGHT,
            Cham::DigitNine => DIGIT_NINE,
            Cham::PunctuationSpiral => PUNCTUATION_SPIRAL,
            Cham::PunctuationDanda => PUNCTUATION_DANDA,
            Cham::PunctuationDoubleDanda => PUNCTUATION_DOUBLE_DANDA,
        }
    }
}

impl std::convert::TryFrom<char> for Cham {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_A => Ok(Cham::LetterA),
            LETTER_I => Ok(Cham::LetterI),
            LETTER_U => Ok(Cham::LetterU),
            LETTER_E => Ok(Cham::LetterE),
            LETTER_AI => Ok(Cham::LetterAi),
            LETTER_O => Ok(Cham::LetterO),
            LETTER_KA => Ok(Cham::LetterKa),
            LETTER_KHA => Ok(Cham::LetterKha),
            LETTER_GA => Ok(Cham::LetterGa),
            LETTER_GHA => Ok(Cham::LetterGha),
            LETTER_NGUE => Ok(Cham::LetterNgue),
            LETTER_NGA => Ok(Cham::LetterNga),
            LETTER_CHA => Ok(Cham::LetterCha),
            LETTER_CHHA => Ok(Cham::LetterChha),
            LETTER_JA => Ok(Cham::LetterJa),
            LETTER_JHA => Ok(Cham::LetterJha),
            LETTER_NHUE => Ok(Cham::LetterNhue),
            LETTER_NHA => Ok(Cham::LetterNha),
            LETTER_NHJA => Ok(Cham::LetterNhja),
            LETTER_TA => Ok(Cham::LetterTa),
            LETTER_THA => Ok(Cham::LetterTha),
            LETTER_DA => Ok(Cham::LetterDa),
            LETTER_DHA => Ok(Cham::LetterDha),
            LETTER_NUE => Ok(Cham::LetterNue),
            LETTER_NA => Ok(Cham::LetterNa),
            LETTER_DDA => Ok(Cham::LetterDda),
            LETTER_PA => Ok(Cham::LetterPa),
            LETTER_PPA => Ok(Cham::LetterPpa),
            LETTER_PHA => Ok(Cham::LetterPha),
            LETTER_BA => Ok(Cham::LetterBa),
            LETTER_BHA => Ok(Cham::LetterBha),
            LETTER_MUE => Ok(Cham::LetterMue),
            LETTER_MA => Ok(Cham::LetterMa),
            LETTER_BBA => Ok(Cham::LetterBba),
            LETTER_YA => Ok(Cham::LetterYa),
            LETTER_RA => Ok(Cham::LetterRa),
            LETTER_LA => Ok(Cham::LetterLa),
            LETTER_VA => Ok(Cham::LetterVa),
            LETTER_SSA => Ok(Cham::LetterSsa),
            LETTER_SA => Ok(Cham::LetterSa),
            LETTER_HA => Ok(Cham::LetterHa),
            VOWEL_SIGN_AA => Ok(Cham::VowelSignAa),
            VOWEL_SIGN_I => Ok(Cham::VowelSignI),
            VOWEL_SIGN_II => Ok(Cham::VowelSignIi),
            VOWEL_SIGN_EI => Ok(Cham::VowelSignEi),
            VOWEL_SIGN_U => Ok(Cham::VowelSignU),
            VOWEL_SIGN_OE => Ok(Cham::VowelSignOe),
            VOWEL_SIGN_O => Ok(Cham::VowelSignO),
            VOWEL_SIGN_AI => Ok(Cham::VowelSignAi),
            VOWEL_SIGN_AU => Ok(Cham::VowelSignAu),
            VOWEL_SIGN_UE => Ok(Cham::VowelSignUe),
            CONSONANT_SIGN_YA => Ok(Cham::ConsonantSignYa),
            CONSONANT_SIGN_RA => Ok(Cham::ConsonantSignRa),
            CONSONANT_SIGN_LA => Ok(Cham::ConsonantSignLa),
            CONSONANT_SIGN_WA => Ok(Cham::ConsonantSignWa),
            LETTER_FINAL_K => Ok(Cham::LetterFinalK),
            LETTER_FINAL_G => Ok(Cham::LetterFinalG),
            LETTER_FINAL_NG => Ok(Cham::LetterFinalNg),
            CONSONANT_SIGN_FINAL_NG => Ok(Cham::ConsonantSignFinalNg),
            LETTER_FINAL_CH => Ok(Cham::LetterFinalCh),
            LETTER_FINAL_T => Ok(Cham::LetterFinalT),
            LETTER_FINAL_N => Ok(Cham::LetterFinalN),
            LETTER_FINAL_P => Ok(Cham::LetterFinalP),
            LETTER_FINAL_Y => Ok(Cham::LetterFinalY),
            LETTER_FINAL_R => Ok(Cham::LetterFinalR),
            LETTER_FINAL_L => Ok(Cham::LetterFinalL),
            LETTER_FINAL_SS => Ok(Cham::LetterFinalSs),
            CONSONANT_SIGN_FINAL_M => Ok(Cham::ConsonantSignFinalM),
            CONSONANT_SIGN_FINAL_H => Ok(Cham::ConsonantSignFinalH),
            DIGIT_ZERO => Ok(Cham::DigitZero),
            DIGIT_ONE => Ok(Cham::DigitOne),
            DIGIT_TWO => Ok(Cham::DigitTwo),
            DIGIT_THREE => Ok(Cham::DigitThree),
            DIGIT_FOUR => Ok(Cham::DigitFour),
            DIGIT_FIVE => Ok(Cham::DigitFive),
            DIGIT_SIX => Ok(Cham::DigitSix),
            DIGIT_SEVEN => Ok(Cham::DigitSeven),
            DIGIT_EIGHT => Ok(Cham::DigitEight),
            DIGIT_NINE => Ok(Cham::DigitNine),
            PUNCTUATION_SPIRAL => Ok(Cham::PunctuationSpiral),
            PUNCTUATION_DANDA => Ok(Cham::PunctuationDanda),
            PUNCTUATION_DOUBLE_DANDA => Ok(Cham::PunctuationDoubleDanda),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Cham {
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

impl std::convert::TryFrom<u32> for Cham {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Cham {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Cham {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Cham::LetterA
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Cham::LetterA => "cham letter a",
            Cham::LetterI => "cham letter i",
            Cham::LetterU => "cham letter u",
            Cham::LetterE => "cham letter e",
            Cham::LetterAi => "cham letter ai",
            Cham::LetterO => "cham letter o",
            Cham::LetterKa => "cham letter ka",
            Cham::LetterKha => "cham letter kha",
            Cham::LetterGa => "cham letter ga",
            Cham::LetterGha => "cham letter gha",
            Cham::LetterNgue => "cham letter ngue",
            Cham::LetterNga => "cham letter nga",
            Cham::LetterCha => "cham letter cha",
            Cham::LetterChha => "cham letter chha",
            Cham::LetterJa => "cham letter ja",
            Cham::LetterJha => "cham letter jha",
            Cham::LetterNhue => "cham letter nhue",
            Cham::LetterNha => "cham letter nha",
            Cham::LetterNhja => "cham letter nhja",
            Cham::LetterTa => "cham letter ta",
            Cham::LetterTha => "cham letter tha",
            Cham::LetterDa => "cham letter da",
            Cham::LetterDha => "cham letter dha",
            Cham::LetterNue => "cham letter nue",
            Cham::LetterNa => "cham letter na",
            Cham::LetterDda => "cham letter dda",
            Cham::LetterPa => "cham letter pa",
            Cham::LetterPpa => "cham letter ppa",
            Cham::LetterPha => "cham letter pha",
            Cham::LetterBa => "cham letter ba",
            Cham::LetterBha => "cham letter bha",
            Cham::LetterMue => "cham letter mue",
            Cham::LetterMa => "cham letter ma",
            Cham::LetterBba => "cham letter bba",
            Cham::LetterYa => "cham letter ya",
            Cham::LetterRa => "cham letter ra",
            Cham::LetterLa => "cham letter la",
            Cham::LetterVa => "cham letter va",
            Cham::LetterSsa => "cham letter ssa",
            Cham::LetterSa => "cham letter sa",
            Cham::LetterHa => "cham letter ha",
            Cham::VowelSignAa => "cham vowel sign aa",
            Cham::VowelSignI => "cham vowel sign i",
            Cham::VowelSignIi => "cham vowel sign ii",
            Cham::VowelSignEi => "cham vowel sign ei",
            Cham::VowelSignU => "cham vowel sign u",
            Cham::VowelSignOe => "cham vowel sign oe",
            Cham::VowelSignO => "cham vowel sign o",
            Cham::VowelSignAi => "cham vowel sign ai",
            Cham::VowelSignAu => "cham vowel sign au",
            Cham::VowelSignUe => "cham vowel sign ue",
            Cham::ConsonantSignYa => "cham consonant sign ya",
            Cham::ConsonantSignRa => "cham consonant sign ra",
            Cham::ConsonantSignLa => "cham consonant sign la",
            Cham::ConsonantSignWa => "cham consonant sign wa",
            Cham::LetterFinalK => "cham letter final k",
            Cham::LetterFinalG => "cham letter final g",
            Cham::LetterFinalNg => "cham letter final ng",
            Cham::ConsonantSignFinalNg => "cham consonant sign final ng",
            Cham::LetterFinalCh => "cham letter final ch",
            Cham::LetterFinalT => "cham letter final t",
            Cham::LetterFinalN => "cham letter final n",
            Cham::LetterFinalP => "cham letter final p",
            Cham::LetterFinalY => "cham letter final y",
            Cham::LetterFinalR => "cham letter final r",
            Cham::LetterFinalL => "cham letter final l",
            Cham::LetterFinalSs => "cham letter final ss",
            Cham::ConsonantSignFinalM => "cham consonant sign final m",
            Cham::ConsonantSignFinalH => "cham consonant sign final h",
            Cham::DigitZero => "cham digit zero",
            Cham::DigitOne => "cham digit one",
            Cham::DigitTwo => "cham digit two",
            Cham::DigitThree => "cham digit three",
            Cham::DigitFour => "cham digit four",
            Cham::DigitFive => "cham digit five",
            Cham::DigitSix => "cham digit six",
            Cham::DigitSeven => "cham digit seven",
            Cham::DigitEight => "cham digit eight",
            Cham::DigitNine => "cham digit nine",
            Cham::PunctuationSpiral => "cham punctuation spiral",
            Cham::PunctuationDanda => "cham punctuation danda",
            Cham::PunctuationDoubleDanda => "cham punctuation double danda",
        }
    }
}
