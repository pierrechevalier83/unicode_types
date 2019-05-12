/// \u{b00} → \u{b7f}
///
/// ଁ ଂ ଃ ଅ ଆ ଇ ଈ ଉ ଊ ଋ ଌ ଏ ଐ ଓ ଔ କ\
/// ଖ ଗ ଘ ଙ ଚ ଛ ଜ ଝ ଞ ଟ ଠ ଡ ଢ ଣ ତ ଥ\
/// ଦ ଧ ନ ପ ଫ ବ ଭ ମ ଯ ର ଲ ଳ ଵ ଶ ଷ ସ\
/// ହ ଼ ଽ ା ି ୀ ୁ ୂ ୃ ୄ େ ୈ ୋ ୌ ୍ ୖ\
/// ୗ ଡ଼ ଢ଼ ୟ ୠ ୡ ୢ ୣ ୦ ୧ ୨ ୩ ୪ ୫ ୬ ୭\
/// ୮ ୯ ୰ ୱ ୲ ୳ ୴ ୵ ୶ ୷\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{b01}: 'ଁ'
    pub const SIGN_CANDRABINDU: char = 'ଁ';
    /// \u{b02}: 'ଂ'
    pub const SIGN_ANUSVARA: char = 'ଂ';
    /// \u{b03}: 'ଃ'
    pub const SIGN_VISARGA: char = 'ଃ';
    /// \u{b05}: 'ଅ'
    pub const LETTER_A: char = 'ଅ';
    /// \u{b06}: 'ଆ'
    pub const LETTER_AA: char = 'ଆ';
    /// \u{b07}: 'ଇ'
    pub const LETTER_I: char = 'ଇ';
    /// \u{b08}: 'ଈ'
    pub const LETTER_II: char = 'ଈ';
    /// \u{b09}: 'ଉ'
    pub const LETTER_U: char = 'ଉ';
    /// \u{b0a}: 'ଊ'
    pub const LETTER_UU: char = 'ଊ';
    /// \u{b0b}: 'ଋ'
    pub const LETTER_VOCALIC_R: char = 'ଋ';
    /// \u{b0c}: 'ଌ'
    pub const LETTER_VOCALIC_L: char = 'ଌ';
    /// \u{b0f}: 'ଏ'
    pub const LETTER_E: char = 'ଏ';
    /// \u{b10}: 'ଐ'
    pub const LETTER_AI: char = 'ଐ';
    /// \u{b13}: 'ଓ'
    pub const LETTER_O: char = 'ଓ';
    /// \u{b14}: 'ଔ'
    pub const LETTER_AU: char = 'ଔ';
    /// \u{b15}: 'କ'
    pub const LETTER_KA: char = 'କ';
    /// \u{b16}: 'ଖ'
    pub const LETTER_KHA: char = 'ଖ';
    /// \u{b17}: 'ଗ'
    pub const LETTER_GA: char = 'ଗ';
    /// \u{b18}: 'ଘ'
    pub const LETTER_GHA: char = 'ଘ';
    /// \u{b19}: 'ଙ'
    pub const LETTER_NGA: char = 'ଙ';
    /// \u{b1a}: 'ଚ'
    pub const LETTER_CA: char = 'ଚ';
    /// \u{b1b}: 'ଛ'
    pub const LETTER_CHA: char = 'ଛ';
    /// \u{b1c}: 'ଜ'
    pub const LETTER_JA: char = 'ଜ';
    /// \u{b1d}: 'ଝ'
    pub const LETTER_JHA: char = 'ଝ';
    /// \u{b1e}: 'ଞ'
    pub const LETTER_NYA: char = 'ଞ';
    /// \u{b1f}: 'ଟ'
    pub const LETTER_TTA: char = 'ଟ';
    /// \u{b20}: 'ଠ'
    pub const LETTER_TTHA: char = 'ଠ';
    /// \u{b21}: 'ଡ'
    pub const LETTER_DDA: char = 'ଡ';
    /// \u{b22}: 'ଢ'
    pub const LETTER_DDHA: char = 'ଢ';
    /// \u{b23}: 'ଣ'
    pub const LETTER_NNA: char = 'ଣ';
    /// \u{b24}: 'ତ'
    pub const LETTER_TA: char = 'ତ';
    /// \u{b25}: 'ଥ'
    pub const LETTER_THA: char = 'ଥ';
    /// \u{b26}: 'ଦ'
    pub const LETTER_DA: char = 'ଦ';
    /// \u{b27}: 'ଧ'
    pub const LETTER_DHA: char = 'ଧ';
    /// \u{b28}: 'ନ'
    pub const LETTER_NA: char = 'ନ';
    /// \u{b2a}: 'ପ'
    pub const LETTER_PA: char = 'ପ';
    /// \u{b2b}: 'ଫ'
    pub const LETTER_PHA: char = 'ଫ';
    /// \u{b2c}: 'ବ'
    pub const LETTER_BA: char = 'ବ';
    /// \u{b2d}: 'ଭ'
    pub const LETTER_BHA: char = 'ଭ';
    /// \u{b2e}: 'ମ'
    pub const LETTER_MA: char = 'ମ';
    /// \u{b2f}: 'ଯ'
    pub const LETTER_YA: char = 'ଯ';
    /// \u{b30}: 'ର'
    pub const LETTER_RA: char = 'ର';
    /// \u{b32}: 'ଲ'
    pub const LETTER_LA: char = 'ଲ';
    /// \u{b33}: 'ଳ'
    pub const LETTER_LLA: char = 'ଳ';
    /// \u{b35}: 'ଵ'
    pub const LETTER_VA: char = 'ଵ';
    /// \u{b36}: 'ଶ'
    pub const LETTER_SHA: char = 'ଶ';
    /// \u{b37}: 'ଷ'
    pub const LETTER_SSA: char = 'ଷ';
    /// \u{b38}: 'ସ'
    pub const LETTER_SA: char = 'ସ';
    /// \u{b39}: 'ହ'
    pub const LETTER_HA: char = 'ହ';
    /// \u{b3c}: '଼'
    pub const SIGN_NUKTA: char = '଼';
    /// \u{b3d}: 'ଽ'
    pub const SIGN_AVAGRAHA: char = 'ଽ';
    /// \u{b3e}: 'ା'
    pub const VOWEL_SIGN_AA: char = 'ା';
    /// \u{b3f}: 'ି'
    pub const VOWEL_SIGN_I: char = 'ି';
    /// \u{b40}: 'ୀ'
    pub const VOWEL_SIGN_II: char = 'ୀ';
    /// \u{b41}: 'ୁ'
    pub const VOWEL_SIGN_U: char = 'ୁ';
    /// \u{b42}: 'ୂ'
    pub const VOWEL_SIGN_UU: char = 'ୂ';
    /// \u{b43}: 'ୃ'
    pub const VOWEL_SIGN_VOCALIC_R: char = 'ୃ';
    /// \u{b44}: 'ୄ'
    pub const VOWEL_SIGN_VOCALIC_RR: char = 'ୄ';
    /// \u{b47}: 'େ'
    pub const VOWEL_SIGN_E: char = 'େ';
    /// \u{b48}: 'ୈ'
    pub const VOWEL_SIGN_AI: char = 'ୈ';
    /// \u{b4b}: 'ୋ'
    pub const VOWEL_SIGN_O: char = 'ୋ';
    /// \u{b4c}: 'ୌ'
    pub const VOWEL_SIGN_AU: char = 'ୌ';
    /// \u{b4d}: '୍'
    pub const SIGN_VIRAMA: char = '୍';
    /// \u{b56}: 'ୖ'
    pub const AI_LENGTH_MARK: char = 'ୖ';
    /// \u{b57}: 'ୗ'
    pub const AU_LENGTH_MARK: char = 'ୗ';
    /// \u{b5c}: 'ଡ଼'
    pub const LETTER_RRA: char = 'ଡ଼';
    /// \u{b5d}: 'ଢ଼'
    pub const LETTER_RHA: char = 'ଢ଼';
    /// \u{b5f}: 'ୟ'
    pub const LETTER_YYA: char = 'ୟ';
    /// \u{b60}: 'ୠ'
    pub const LETTER_VOCALIC_RR: char = 'ୠ';
    /// \u{b61}: 'ୡ'
    pub const LETTER_VOCALIC_LL: char = 'ୡ';
    /// \u{b62}: 'ୢ'
    pub const VOWEL_SIGN_VOCALIC_L: char = 'ୢ';
    /// \u{b63}: 'ୣ'
    pub const VOWEL_SIGN_VOCALIC_LL: char = 'ୣ';
    /// \u{b66}: '୦'
    pub const DIGIT_ZERO: char = '୦';
    /// \u{b67}: '୧'
    pub const DIGIT_ONE: char = '୧';
    /// \u{b68}: '୨'
    pub const DIGIT_TWO: char = '୨';
    /// \u{b69}: '୩'
    pub const DIGIT_THREE: char = '୩';
    /// \u{b6a}: '୪'
    pub const DIGIT_FOUR: char = '୪';
    /// \u{b6b}: '୫'
    pub const DIGIT_FIVE: char = '୫';
    /// \u{b6c}: '୬'
    pub const DIGIT_SIX: char = '୬';
    /// \u{b6d}: '୭'
    pub const DIGIT_SEVEN: char = '୭';
    /// \u{b6e}: '୮'
    pub const DIGIT_EIGHT: char = '୮';
    /// \u{b6f}: '୯'
    pub const DIGIT_NINE: char = '୯';
    /// \u{b70}: '୰'
    pub const ISSHAR: char = '୰';
    /// \u{b71}: 'ୱ'
    pub const LETTER_WA: char = 'ୱ';
    /// \u{b72}: '୲'
    pub const FRACTION_ONE_QUARTER: char = '୲';
    /// \u{b73}: '୳'
    pub const FRACTION_ONE_HALF: char = '୳';
    /// \u{b74}: '୴'
    pub const FRACTION_THREE_QUARTERS: char = '୴';
    /// \u{b75}: '୵'
    pub const FRACTION_ONE_SIXTEENTH: char = '୵';
    /// \u{b76}: '୶'
    pub const FRACTION_ONE_EIGHTH: char = '୶';
    /// \u{b77}: '୷'
    pub const FRACTION_THREE_SIXTEENTHS: char = '୷';
}

/// An enum to represent all characters in the Oriya block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Oriya {
    /// \u{b01}: 'ଁ'
    SignCandrabindu,
    /// \u{b02}: 'ଂ'
    SignAnusvara,
    /// \u{b03}: 'ଃ'
    SignVisarga,
    /// \u{b05}: 'ଅ'
    LetterA,
    /// \u{b06}: 'ଆ'
    LetterAa,
    /// \u{b07}: 'ଇ'
    LetterI,
    /// \u{b08}: 'ଈ'
    LetterIi,
    /// \u{b09}: 'ଉ'
    LetterU,
    /// \u{b0a}: 'ଊ'
    LetterUu,
    /// \u{b0b}: 'ଋ'
    LetterVocalicR,
    /// \u{b0c}: 'ଌ'
    LetterVocalicL,
    /// \u{b0f}: 'ଏ'
    LetterE,
    /// \u{b10}: 'ଐ'
    LetterAi,
    /// \u{b13}: 'ଓ'
    LetterO,
    /// \u{b14}: 'ଔ'
    LetterAu,
    /// \u{b15}: 'କ'
    LetterKa,
    /// \u{b16}: 'ଖ'
    LetterKha,
    /// \u{b17}: 'ଗ'
    LetterGa,
    /// \u{b18}: 'ଘ'
    LetterGha,
    /// \u{b19}: 'ଙ'
    LetterNga,
    /// \u{b1a}: 'ଚ'
    LetterCa,
    /// \u{b1b}: 'ଛ'
    LetterCha,
    /// \u{b1c}: 'ଜ'
    LetterJa,
    /// \u{b1d}: 'ଝ'
    LetterJha,
    /// \u{b1e}: 'ଞ'
    LetterNya,
    /// \u{b1f}: 'ଟ'
    LetterTta,
    /// \u{b20}: 'ଠ'
    LetterTtha,
    /// \u{b21}: 'ଡ'
    LetterDda,
    /// \u{b22}: 'ଢ'
    LetterDdha,
    /// \u{b23}: 'ଣ'
    LetterNna,
    /// \u{b24}: 'ତ'
    LetterTa,
    /// \u{b25}: 'ଥ'
    LetterTha,
    /// \u{b26}: 'ଦ'
    LetterDa,
    /// \u{b27}: 'ଧ'
    LetterDha,
    /// \u{b28}: 'ନ'
    LetterNa,
    /// \u{b2a}: 'ପ'
    LetterPa,
    /// \u{b2b}: 'ଫ'
    LetterPha,
    /// \u{b2c}: 'ବ'
    LetterBa,
    /// \u{b2d}: 'ଭ'
    LetterBha,
    /// \u{b2e}: 'ମ'
    LetterMa,
    /// \u{b2f}: 'ଯ'
    LetterYa,
    /// \u{b30}: 'ର'
    LetterRa,
    /// \u{b32}: 'ଲ'
    LetterLa,
    /// \u{b33}: 'ଳ'
    LetterLla,
    /// \u{b35}: 'ଵ'
    LetterVa,
    /// \u{b36}: 'ଶ'
    LetterSha,
    /// \u{b37}: 'ଷ'
    LetterSsa,
    /// \u{b38}: 'ସ'
    LetterSa,
    /// \u{b39}: 'ହ'
    LetterHa,
    /// \u{b3c}: '଼'
    SignNukta,
    /// \u{b3d}: 'ଽ'
    SignAvagraha,
    /// \u{b3e}: 'ା'
    VowelSignAa,
    /// \u{b3f}: 'ି'
    VowelSignI,
    /// \u{b40}: 'ୀ'
    VowelSignIi,
    /// \u{b41}: 'ୁ'
    VowelSignU,
    /// \u{b42}: 'ୂ'
    VowelSignUu,
    /// \u{b43}: 'ୃ'
    VowelSignVocalicR,
    /// \u{b44}: 'ୄ'
    VowelSignVocalicRr,
    /// \u{b47}: 'େ'
    VowelSignE,
    /// \u{b48}: 'ୈ'
    VowelSignAi,
    /// \u{b4b}: 'ୋ'
    VowelSignO,
    /// \u{b4c}: 'ୌ'
    VowelSignAu,
    /// \u{b4d}: '୍'
    SignVirama,
    /// \u{b56}: 'ୖ'
    AiLengthMark,
    /// \u{b57}: 'ୗ'
    AuLengthMark,
    /// \u{b5c}: 'ଡ଼'
    LetterRra,
    /// \u{b5d}: 'ଢ଼'
    LetterRha,
    /// \u{b5f}: 'ୟ'
    LetterYya,
    /// \u{b60}: 'ୠ'
    LetterVocalicRr,
    /// \u{b61}: 'ୡ'
    LetterVocalicLl,
    /// \u{b62}: 'ୢ'
    VowelSignVocalicL,
    /// \u{b63}: 'ୣ'
    VowelSignVocalicLl,
    /// \u{b66}: '୦'
    DigitZero,
    /// \u{b67}: '୧'
    DigitOne,
    /// \u{b68}: '୨'
    DigitTwo,
    /// \u{b69}: '୩'
    DigitThree,
    /// \u{b6a}: '୪'
    DigitFour,
    /// \u{b6b}: '୫'
    DigitFive,
    /// \u{b6c}: '୬'
    DigitSix,
    /// \u{b6d}: '୭'
    DigitSeven,
    /// \u{b6e}: '୮'
    DigitEight,
    /// \u{b6f}: '୯'
    DigitNine,
    /// \u{b70}: '୰'
    Isshar,
    /// \u{b71}: 'ୱ'
    LetterWa,
    /// \u{b72}: '୲'
    FractionOneQuarter,
    /// \u{b73}: '୳'
    FractionOneHalf,
    /// \u{b74}: '୴'
    FractionThreeQuarters,
    /// \u{b75}: '୵'
    FractionOneSixteenth,
    /// \u{b76}: '୶'
    FractionOneEighth,
    /// \u{b77}: '୷'
    FractionThreeSixteenths,
}

impl Into<char> for Oriya {
    fn into(self) -> char {
        use constants::*;
        match self {
            Oriya::SignCandrabindu => SIGN_CANDRABINDU,
            Oriya::SignAnusvara => SIGN_ANUSVARA,
            Oriya::SignVisarga => SIGN_VISARGA,
            Oriya::LetterA => LETTER_A,
            Oriya::LetterAa => LETTER_AA,
            Oriya::LetterI => LETTER_I,
            Oriya::LetterIi => LETTER_II,
            Oriya::LetterU => LETTER_U,
            Oriya::LetterUu => LETTER_UU,
            Oriya::LetterVocalicR => LETTER_VOCALIC_R,
            Oriya::LetterVocalicL => LETTER_VOCALIC_L,
            Oriya::LetterE => LETTER_E,
            Oriya::LetterAi => LETTER_AI,
            Oriya::LetterO => LETTER_O,
            Oriya::LetterAu => LETTER_AU,
            Oriya::LetterKa => LETTER_KA,
            Oriya::LetterKha => LETTER_KHA,
            Oriya::LetterGa => LETTER_GA,
            Oriya::LetterGha => LETTER_GHA,
            Oriya::LetterNga => LETTER_NGA,
            Oriya::LetterCa => LETTER_CA,
            Oriya::LetterCha => LETTER_CHA,
            Oriya::LetterJa => LETTER_JA,
            Oriya::LetterJha => LETTER_JHA,
            Oriya::LetterNya => LETTER_NYA,
            Oriya::LetterTta => LETTER_TTA,
            Oriya::LetterTtha => LETTER_TTHA,
            Oriya::LetterDda => LETTER_DDA,
            Oriya::LetterDdha => LETTER_DDHA,
            Oriya::LetterNna => LETTER_NNA,
            Oriya::LetterTa => LETTER_TA,
            Oriya::LetterTha => LETTER_THA,
            Oriya::LetterDa => LETTER_DA,
            Oriya::LetterDha => LETTER_DHA,
            Oriya::LetterNa => LETTER_NA,
            Oriya::LetterPa => LETTER_PA,
            Oriya::LetterPha => LETTER_PHA,
            Oriya::LetterBa => LETTER_BA,
            Oriya::LetterBha => LETTER_BHA,
            Oriya::LetterMa => LETTER_MA,
            Oriya::LetterYa => LETTER_YA,
            Oriya::LetterRa => LETTER_RA,
            Oriya::LetterLa => LETTER_LA,
            Oriya::LetterLla => LETTER_LLA,
            Oriya::LetterVa => LETTER_VA,
            Oriya::LetterSha => LETTER_SHA,
            Oriya::LetterSsa => LETTER_SSA,
            Oriya::LetterSa => LETTER_SA,
            Oriya::LetterHa => LETTER_HA,
            Oriya::SignNukta => SIGN_NUKTA,
            Oriya::SignAvagraha => SIGN_AVAGRAHA,
            Oriya::VowelSignAa => VOWEL_SIGN_AA,
            Oriya::VowelSignI => VOWEL_SIGN_I,
            Oriya::VowelSignIi => VOWEL_SIGN_II,
            Oriya::VowelSignU => VOWEL_SIGN_U,
            Oriya::VowelSignUu => VOWEL_SIGN_UU,
            Oriya::VowelSignVocalicR => VOWEL_SIGN_VOCALIC_R,
            Oriya::VowelSignVocalicRr => VOWEL_SIGN_VOCALIC_RR,
            Oriya::VowelSignE => VOWEL_SIGN_E,
            Oriya::VowelSignAi => VOWEL_SIGN_AI,
            Oriya::VowelSignO => VOWEL_SIGN_O,
            Oriya::VowelSignAu => VOWEL_SIGN_AU,
            Oriya::SignVirama => SIGN_VIRAMA,
            Oriya::AiLengthMark => AI_LENGTH_MARK,
            Oriya::AuLengthMark => AU_LENGTH_MARK,
            Oriya::LetterRra => LETTER_RRA,
            Oriya::LetterRha => LETTER_RHA,
            Oriya::LetterYya => LETTER_YYA,
            Oriya::LetterVocalicRr => LETTER_VOCALIC_RR,
            Oriya::LetterVocalicLl => LETTER_VOCALIC_LL,
            Oriya::VowelSignVocalicL => VOWEL_SIGN_VOCALIC_L,
            Oriya::VowelSignVocalicLl => VOWEL_SIGN_VOCALIC_LL,
            Oriya::DigitZero => DIGIT_ZERO,
            Oriya::DigitOne => DIGIT_ONE,
            Oriya::DigitTwo => DIGIT_TWO,
            Oriya::DigitThree => DIGIT_THREE,
            Oriya::DigitFour => DIGIT_FOUR,
            Oriya::DigitFive => DIGIT_FIVE,
            Oriya::DigitSix => DIGIT_SIX,
            Oriya::DigitSeven => DIGIT_SEVEN,
            Oriya::DigitEight => DIGIT_EIGHT,
            Oriya::DigitNine => DIGIT_NINE,
            Oriya::Isshar => ISSHAR,
            Oriya::LetterWa => LETTER_WA,
            Oriya::FractionOneQuarter => FRACTION_ONE_QUARTER,
            Oriya::FractionOneHalf => FRACTION_ONE_HALF,
            Oriya::FractionThreeQuarters => FRACTION_THREE_QUARTERS,
            Oriya::FractionOneSixteenth => FRACTION_ONE_SIXTEENTH,
            Oriya::FractionOneEighth => FRACTION_ONE_EIGHTH,
            Oriya::FractionThreeSixteenths => FRACTION_THREE_SIXTEENTHS,
        }
    }
}

impl std::convert::TryFrom<char> for Oriya {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            SIGN_CANDRABINDU => Ok(Oriya::SignCandrabindu),
            SIGN_ANUSVARA => Ok(Oriya::SignAnusvara),
            SIGN_VISARGA => Ok(Oriya::SignVisarga),
            LETTER_A => Ok(Oriya::LetterA),
            LETTER_AA => Ok(Oriya::LetterAa),
            LETTER_I => Ok(Oriya::LetterI),
            LETTER_II => Ok(Oriya::LetterIi),
            LETTER_U => Ok(Oriya::LetterU),
            LETTER_UU => Ok(Oriya::LetterUu),
            LETTER_VOCALIC_R => Ok(Oriya::LetterVocalicR),
            LETTER_VOCALIC_L => Ok(Oriya::LetterVocalicL),
            LETTER_E => Ok(Oriya::LetterE),
            LETTER_AI => Ok(Oriya::LetterAi),
            LETTER_O => Ok(Oriya::LetterO),
            LETTER_AU => Ok(Oriya::LetterAu),
            LETTER_KA => Ok(Oriya::LetterKa),
            LETTER_KHA => Ok(Oriya::LetterKha),
            LETTER_GA => Ok(Oriya::LetterGa),
            LETTER_GHA => Ok(Oriya::LetterGha),
            LETTER_NGA => Ok(Oriya::LetterNga),
            LETTER_CA => Ok(Oriya::LetterCa),
            LETTER_CHA => Ok(Oriya::LetterCha),
            LETTER_JA => Ok(Oriya::LetterJa),
            LETTER_JHA => Ok(Oriya::LetterJha),
            LETTER_NYA => Ok(Oriya::LetterNya),
            LETTER_TTA => Ok(Oriya::LetterTta),
            LETTER_TTHA => Ok(Oriya::LetterTtha),
            LETTER_DDA => Ok(Oriya::LetterDda),
            LETTER_DDHA => Ok(Oriya::LetterDdha),
            LETTER_NNA => Ok(Oriya::LetterNna),
            LETTER_TA => Ok(Oriya::LetterTa),
            LETTER_THA => Ok(Oriya::LetterTha),
            LETTER_DA => Ok(Oriya::LetterDa),
            LETTER_DHA => Ok(Oriya::LetterDha),
            LETTER_NA => Ok(Oriya::LetterNa),
            LETTER_PA => Ok(Oriya::LetterPa),
            LETTER_PHA => Ok(Oriya::LetterPha),
            LETTER_BA => Ok(Oriya::LetterBa),
            LETTER_BHA => Ok(Oriya::LetterBha),
            LETTER_MA => Ok(Oriya::LetterMa),
            LETTER_YA => Ok(Oriya::LetterYa),
            LETTER_RA => Ok(Oriya::LetterRa),
            LETTER_LA => Ok(Oriya::LetterLa),
            LETTER_LLA => Ok(Oriya::LetterLla),
            LETTER_VA => Ok(Oriya::LetterVa),
            LETTER_SHA => Ok(Oriya::LetterSha),
            LETTER_SSA => Ok(Oriya::LetterSsa),
            LETTER_SA => Ok(Oriya::LetterSa),
            LETTER_HA => Ok(Oriya::LetterHa),
            SIGN_NUKTA => Ok(Oriya::SignNukta),
            SIGN_AVAGRAHA => Ok(Oriya::SignAvagraha),
            VOWEL_SIGN_AA => Ok(Oriya::VowelSignAa),
            VOWEL_SIGN_I => Ok(Oriya::VowelSignI),
            VOWEL_SIGN_II => Ok(Oriya::VowelSignIi),
            VOWEL_SIGN_U => Ok(Oriya::VowelSignU),
            VOWEL_SIGN_UU => Ok(Oriya::VowelSignUu),
            VOWEL_SIGN_VOCALIC_R => Ok(Oriya::VowelSignVocalicR),
            VOWEL_SIGN_VOCALIC_RR => Ok(Oriya::VowelSignVocalicRr),
            VOWEL_SIGN_E => Ok(Oriya::VowelSignE),
            VOWEL_SIGN_AI => Ok(Oriya::VowelSignAi),
            VOWEL_SIGN_O => Ok(Oriya::VowelSignO),
            VOWEL_SIGN_AU => Ok(Oriya::VowelSignAu),
            SIGN_VIRAMA => Ok(Oriya::SignVirama),
            AI_LENGTH_MARK => Ok(Oriya::AiLengthMark),
            AU_LENGTH_MARK => Ok(Oriya::AuLengthMark),
            LETTER_RRA => Ok(Oriya::LetterRra),
            LETTER_RHA => Ok(Oriya::LetterRha),
            LETTER_YYA => Ok(Oriya::LetterYya),
            LETTER_VOCALIC_RR => Ok(Oriya::LetterVocalicRr),
            LETTER_VOCALIC_LL => Ok(Oriya::LetterVocalicLl),
            VOWEL_SIGN_VOCALIC_L => Ok(Oriya::VowelSignVocalicL),
            VOWEL_SIGN_VOCALIC_LL => Ok(Oriya::VowelSignVocalicLl),
            DIGIT_ZERO => Ok(Oriya::DigitZero),
            DIGIT_ONE => Ok(Oriya::DigitOne),
            DIGIT_TWO => Ok(Oriya::DigitTwo),
            DIGIT_THREE => Ok(Oriya::DigitThree),
            DIGIT_FOUR => Ok(Oriya::DigitFour),
            DIGIT_FIVE => Ok(Oriya::DigitFive),
            DIGIT_SIX => Ok(Oriya::DigitSix),
            DIGIT_SEVEN => Ok(Oriya::DigitSeven),
            DIGIT_EIGHT => Ok(Oriya::DigitEight),
            DIGIT_NINE => Ok(Oriya::DigitNine),
            ISSHAR => Ok(Oriya::Isshar),
            LETTER_WA => Ok(Oriya::LetterWa),
            FRACTION_ONE_QUARTER => Ok(Oriya::FractionOneQuarter),
            FRACTION_ONE_HALF => Ok(Oriya::FractionOneHalf),
            FRACTION_THREE_QUARTERS => Ok(Oriya::FractionThreeQuarters),
            FRACTION_ONE_SIXTEENTH => Ok(Oriya::FractionOneSixteenth),
            FRACTION_ONE_EIGHTH => Ok(Oriya::FractionOneEighth),
            FRACTION_THREE_SIXTEENTHS => Ok(Oriya::FractionThreeSixteenths),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Oriya {
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

impl std::convert::TryFrom<u32> for Oriya {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Oriya {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Oriya {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Oriya::SignCandrabindu
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Oriya::SignCandrabindu => "oriya sign candrabindu",
            Oriya::SignAnusvara => "oriya sign anusvara",
            Oriya::SignVisarga => "oriya sign visarga",
            Oriya::LetterA => "oriya letter a",
            Oriya::LetterAa => "oriya letter aa",
            Oriya::LetterI => "oriya letter i",
            Oriya::LetterIi => "oriya letter ii",
            Oriya::LetterU => "oriya letter u",
            Oriya::LetterUu => "oriya letter uu",
            Oriya::LetterVocalicR => "oriya letter vocalic r",
            Oriya::LetterVocalicL => "oriya letter vocalic l",
            Oriya::LetterE => "oriya letter e",
            Oriya::LetterAi => "oriya letter ai",
            Oriya::LetterO => "oriya letter o",
            Oriya::LetterAu => "oriya letter au",
            Oriya::LetterKa => "oriya letter ka",
            Oriya::LetterKha => "oriya letter kha",
            Oriya::LetterGa => "oriya letter ga",
            Oriya::LetterGha => "oriya letter gha",
            Oriya::LetterNga => "oriya letter nga",
            Oriya::LetterCa => "oriya letter ca",
            Oriya::LetterCha => "oriya letter cha",
            Oriya::LetterJa => "oriya letter ja",
            Oriya::LetterJha => "oriya letter jha",
            Oriya::LetterNya => "oriya letter nya",
            Oriya::LetterTta => "oriya letter tta",
            Oriya::LetterTtha => "oriya letter ttha",
            Oriya::LetterDda => "oriya letter dda",
            Oriya::LetterDdha => "oriya letter ddha",
            Oriya::LetterNna => "oriya letter nna",
            Oriya::LetterTa => "oriya letter ta",
            Oriya::LetterTha => "oriya letter tha",
            Oriya::LetterDa => "oriya letter da",
            Oriya::LetterDha => "oriya letter dha",
            Oriya::LetterNa => "oriya letter na",
            Oriya::LetterPa => "oriya letter pa",
            Oriya::LetterPha => "oriya letter pha",
            Oriya::LetterBa => "oriya letter ba",
            Oriya::LetterBha => "oriya letter bha",
            Oriya::LetterMa => "oriya letter ma",
            Oriya::LetterYa => "oriya letter ya",
            Oriya::LetterRa => "oriya letter ra",
            Oriya::LetterLa => "oriya letter la",
            Oriya::LetterLla => "oriya letter lla",
            Oriya::LetterVa => "oriya letter va",
            Oriya::LetterSha => "oriya letter sha",
            Oriya::LetterSsa => "oriya letter ssa",
            Oriya::LetterSa => "oriya letter sa",
            Oriya::LetterHa => "oriya letter ha",
            Oriya::SignNukta => "oriya sign nukta",
            Oriya::SignAvagraha => "oriya sign avagraha",
            Oriya::VowelSignAa => "oriya vowel sign aa",
            Oriya::VowelSignI => "oriya vowel sign i",
            Oriya::VowelSignIi => "oriya vowel sign ii",
            Oriya::VowelSignU => "oriya vowel sign u",
            Oriya::VowelSignUu => "oriya vowel sign uu",
            Oriya::VowelSignVocalicR => "oriya vowel sign vocalic r",
            Oriya::VowelSignVocalicRr => "oriya vowel sign vocalic rr",
            Oriya::VowelSignE => "oriya vowel sign e",
            Oriya::VowelSignAi => "oriya vowel sign ai",
            Oriya::VowelSignO => "oriya vowel sign o",
            Oriya::VowelSignAu => "oriya vowel sign au",
            Oriya::SignVirama => "oriya sign virama",
            Oriya::AiLengthMark => "oriya ai length mark",
            Oriya::AuLengthMark => "oriya au length mark",
            Oriya::LetterRra => "oriya letter rra",
            Oriya::LetterRha => "oriya letter rha",
            Oriya::LetterYya => "oriya letter yya",
            Oriya::LetterVocalicRr => "oriya letter vocalic rr",
            Oriya::LetterVocalicLl => "oriya letter vocalic ll",
            Oriya::VowelSignVocalicL => "oriya vowel sign vocalic l",
            Oriya::VowelSignVocalicLl => "oriya vowel sign vocalic ll",
            Oriya::DigitZero => "oriya digit zero",
            Oriya::DigitOne => "oriya digit one",
            Oriya::DigitTwo => "oriya digit two",
            Oriya::DigitThree => "oriya digit three",
            Oriya::DigitFour => "oriya digit four",
            Oriya::DigitFive => "oriya digit five",
            Oriya::DigitSix => "oriya digit six",
            Oriya::DigitSeven => "oriya digit seven",
            Oriya::DigitEight => "oriya digit eight",
            Oriya::DigitNine => "oriya digit nine",
            Oriya::Isshar => "oriya isshar",
            Oriya::LetterWa => "oriya letter wa",
            Oriya::FractionOneQuarter => "oriya fraction one quarter",
            Oriya::FractionOneHalf => "oriya fraction one half",
            Oriya::FractionThreeQuarters => "oriya fraction three quarters",
            Oriya::FractionOneSixteenth => "oriya fraction one sixteenth",
            Oriya::FractionOneEighth => "oriya fraction one eighth",
            Oriya::FractionThreeSixteenths => "oriya fraction three sixteenths",
        }
    }
}
