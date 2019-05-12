/// \u{c80} → \u{cff}
///
/// ಀ ಁ ಂ ಃ ಄ ಅ ಆ ಇ ಈ ಉ ಊ ಋ ಌ ಎ ಏ ಐ\
/// ಒ ಓ ಔ ಕ ಖ ಗ ಘ ಙ ಚ ಛ ಜ ಝ ಞ ಟ ಠ ಡ\
/// ಢ ಣ ತ ಥ ದ ಧ ನ ಪ ಫ ಬ ಭ ಮ ಯ ರ ಱ ಲ\
/// ಳ ವ ಶ ಷ ಸ ಹ ಼ ಽ ಾ ಿ ೀ ು ೂ ೃ ೄ ೆ\
/// ೇ ೈ ೊ ೋ ೌ ್ ೕ ೖ ೞ ೠ ೡ ೢ ೣ ೦ ೧ ೨\
/// ೩ ೪ ೫ ೬ ೭ ೮ ೯ ೱ ೲ\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{c80}: 'ಀ'
    pub const SIGN_SPACING_CANDRABINDU: char = 'ಀ';
    /// \u{c81}: 'ಁ'
    pub const SIGN_CANDRABINDU: char = 'ಁ';
    /// \u{c82}: 'ಂ'
    pub const SIGN_ANUSVARA: char = 'ಂ';
    /// \u{c83}: 'ಃ'
    pub const SIGN_VISARGA: char = 'ಃ';
    /// \u{c84}: '಄'
    pub const SIGN_SIDDHAM: char = '಄';
    /// \u{c85}: 'ಅ'
    pub const LETTER_A: char = 'ಅ';
    /// \u{c86}: 'ಆ'
    pub const LETTER_AA: char = 'ಆ';
    /// \u{c87}: 'ಇ'
    pub const LETTER_I: char = 'ಇ';
    /// \u{c88}: 'ಈ'
    pub const LETTER_II: char = 'ಈ';
    /// \u{c89}: 'ಉ'
    pub const LETTER_U: char = 'ಉ';
    /// \u{c8a}: 'ಊ'
    pub const LETTER_UU: char = 'ಊ';
    /// \u{c8b}: 'ಋ'
    pub const LETTER_VOCALIC_R: char = 'ಋ';
    /// \u{c8c}: 'ಌ'
    pub const LETTER_VOCALIC_L: char = 'ಌ';
    /// \u{c8e}: 'ಎ'
    pub const LETTER_E: char = 'ಎ';
    /// \u{c8f}: 'ಏ'
    pub const LETTER_EE: char = 'ಏ';
    /// \u{c90}: 'ಐ'
    pub const LETTER_AI: char = 'ಐ';
    /// \u{c92}: 'ಒ'
    pub const LETTER_O: char = 'ಒ';
    /// \u{c93}: 'ಓ'
    pub const LETTER_OO: char = 'ಓ';
    /// \u{c94}: 'ಔ'
    pub const LETTER_AU: char = 'ಔ';
    /// \u{c95}: 'ಕ'
    pub const LETTER_KA: char = 'ಕ';
    /// \u{c96}: 'ಖ'
    pub const LETTER_KHA: char = 'ಖ';
    /// \u{c97}: 'ಗ'
    pub const LETTER_GA: char = 'ಗ';
    /// \u{c98}: 'ಘ'
    pub const LETTER_GHA: char = 'ಘ';
    /// \u{c99}: 'ಙ'
    pub const LETTER_NGA: char = 'ಙ';
    /// \u{c9a}: 'ಚ'
    pub const LETTER_CA: char = 'ಚ';
    /// \u{c9b}: 'ಛ'
    pub const LETTER_CHA: char = 'ಛ';
    /// \u{c9c}: 'ಜ'
    pub const LETTER_JA: char = 'ಜ';
    /// \u{c9d}: 'ಝ'
    pub const LETTER_JHA: char = 'ಝ';
    /// \u{c9e}: 'ಞ'
    pub const LETTER_NYA: char = 'ಞ';
    /// \u{c9f}: 'ಟ'
    pub const LETTER_TTA: char = 'ಟ';
    /// \u{ca0}: 'ಠ'
    pub const LETTER_TTHA: char = 'ಠ';
    /// \u{ca1}: 'ಡ'
    pub const LETTER_DDA: char = 'ಡ';
    /// \u{ca2}: 'ಢ'
    pub const LETTER_DDHA: char = 'ಢ';
    /// \u{ca3}: 'ಣ'
    pub const LETTER_NNA: char = 'ಣ';
    /// \u{ca4}: 'ತ'
    pub const LETTER_TA: char = 'ತ';
    /// \u{ca5}: 'ಥ'
    pub const LETTER_THA: char = 'ಥ';
    /// \u{ca6}: 'ದ'
    pub const LETTER_DA: char = 'ದ';
    /// \u{ca7}: 'ಧ'
    pub const LETTER_DHA: char = 'ಧ';
    /// \u{ca8}: 'ನ'
    pub const LETTER_NA: char = 'ನ';
    /// \u{caa}: 'ಪ'
    pub const LETTER_PA: char = 'ಪ';
    /// \u{cab}: 'ಫ'
    pub const LETTER_PHA: char = 'ಫ';
    /// \u{cac}: 'ಬ'
    pub const LETTER_BA: char = 'ಬ';
    /// \u{cad}: 'ಭ'
    pub const LETTER_BHA: char = 'ಭ';
    /// \u{cae}: 'ಮ'
    pub const LETTER_MA: char = 'ಮ';
    /// \u{caf}: 'ಯ'
    pub const LETTER_YA: char = 'ಯ';
    /// \u{cb0}: 'ರ'
    pub const LETTER_RA: char = 'ರ';
    /// \u{cb1}: 'ಱ'
    pub const LETTER_RRA: char = 'ಱ';
    /// \u{cb2}: 'ಲ'
    pub const LETTER_LA: char = 'ಲ';
    /// \u{cb3}: 'ಳ'
    pub const LETTER_LLA: char = 'ಳ';
    /// \u{cb5}: 'ವ'
    pub const LETTER_VA: char = 'ವ';
    /// \u{cb6}: 'ಶ'
    pub const LETTER_SHA: char = 'ಶ';
    /// \u{cb7}: 'ಷ'
    pub const LETTER_SSA: char = 'ಷ';
    /// \u{cb8}: 'ಸ'
    pub const LETTER_SA: char = 'ಸ';
    /// \u{cb9}: 'ಹ'
    pub const LETTER_HA: char = 'ಹ';
    /// \u{cbc}: '಼'
    pub const SIGN_NUKTA: char = '಼';
    /// \u{cbd}: 'ಽ'
    pub const SIGN_AVAGRAHA: char = 'ಽ';
    /// \u{cbe}: 'ಾ'
    pub const VOWEL_SIGN_AA: char = 'ಾ';
    /// \u{cbf}: 'ಿ'
    pub const VOWEL_SIGN_I: char = 'ಿ';
    /// \u{cc0}: 'ೀ'
    pub const VOWEL_SIGN_II: char = 'ೀ';
    /// \u{cc1}: 'ು'
    pub const VOWEL_SIGN_U: char = 'ು';
    /// \u{cc2}: 'ೂ'
    pub const VOWEL_SIGN_UU: char = 'ೂ';
    /// \u{cc3}: 'ೃ'
    pub const VOWEL_SIGN_VOCALIC_R: char = 'ೃ';
    /// \u{cc4}: 'ೄ'
    pub const VOWEL_SIGN_VOCALIC_RR: char = 'ೄ';
    /// \u{cc6}: 'ೆ'
    pub const VOWEL_SIGN_E: char = 'ೆ';
    /// \u{cc7}: 'ೇ'
    pub const VOWEL_SIGN_EE: char = 'ೇ';
    /// \u{cc8}: 'ೈ'
    pub const VOWEL_SIGN_AI: char = 'ೈ';
    /// \u{cca}: 'ೊ'
    pub const VOWEL_SIGN_O: char = 'ೊ';
    /// \u{ccb}: 'ೋ'
    pub const VOWEL_SIGN_OO: char = 'ೋ';
    /// \u{ccc}: 'ೌ'
    pub const VOWEL_SIGN_AU: char = 'ೌ';
    /// \u{ccd}: '್'
    pub const SIGN_VIRAMA: char = '್';
    /// \u{cd5}: 'ೕ'
    pub const LENGTH_MARK: char = 'ೕ';
    /// \u{cd6}: 'ೖ'
    pub const AI_LENGTH_MARK: char = 'ೖ';
    /// \u{cde}: 'ೞ'
    pub const LETTER_FA: char = 'ೞ';
    /// \u{ce0}: 'ೠ'
    pub const LETTER_VOCALIC_RR: char = 'ೠ';
    /// \u{ce1}: 'ೡ'
    pub const LETTER_VOCALIC_LL: char = 'ೡ';
    /// \u{ce2}: 'ೢ'
    pub const VOWEL_SIGN_VOCALIC_L: char = 'ೢ';
    /// \u{ce3}: 'ೣ'
    pub const VOWEL_SIGN_VOCALIC_LL: char = 'ೣ';
    /// \u{ce6}: '೦'
    pub const DIGIT_ZERO: char = '೦';
    /// \u{ce7}: '೧'
    pub const DIGIT_ONE: char = '೧';
    /// \u{ce8}: '೨'
    pub const DIGIT_TWO: char = '೨';
    /// \u{ce9}: '೩'
    pub const DIGIT_THREE: char = '೩';
    /// \u{cea}: '೪'
    pub const DIGIT_FOUR: char = '೪';
    /// \u{ceb}: '೫'
    pub const DIGIT_FIVE: char = '೫';
    /// \u{cec}: '೬'
    pub const DIGIT_SIX: char = '೬';
    /// \u{ced}: '೭'
    pub const DIGIT_SEVEN: char = '೭';
    /// \u{cee}: '೮'
    pub const DIGIT_EIGHT: char = '೮';
    /// \u{cef}: '೯'
    pub const DIGIT_NINE: char = '೯';
    /// \u{cf1}: 'ೱ'
    pub const SIGN_JIHVAMULIYA: char = 'ೱ';
    /// \u{cf2}: 'ೲ'
    pub const SIGN_UPADHMANIYA: char = 'ೲ';
}

/// An enum to represent all characters in the Kannada block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Kannada {
    /// \u{c80}: 'ಀ'
    SignSpacingCandrabindu,
    /// \u{c81}: 'ಁ'
    SignCandrabindu,
    /// \u{c82}: 'ಂ'
    SignAnusvara,
    /// \u{c83}: 'ಃ'
    SignVisarga,
    /// \u{c84}: '಄'
    SignSiddham,
    /// \u{c85}: 'ಅ'
    LetterA,
    /// \u{c86}: 'ಆ'
    LetterAa,
    /// \u{c87}: 'ಇ'
    LetterI,
    /// \u{c88}: 'ಈ'
    LetterIi,
    /// \u{c89}: 'ಉ'
    LetterU,
    /// \u{c8a}: 'ಊ'
    LetterUu,
    /// \u{c8b}: 'ಋ'
    LetterVocalicR,
    /// \u{c8c}: 'ಌ'
    LetterVocalicL,
    /// \u{c8e}: 'ಎ'
    LetterE,
    /// \u{c8f}: 'ಏ'
    LetterEe,
    /// \u{c90}: 'ಐ'
    LetterAi,
    /// \u{c92}: 'ಒ'
    LetterO,
    /// \u{c93}: 'ಓ'
    LetterOo,
    /// \u{c94}: 'ಔ'
    LetterAu,
    /// \u{c95}: 'ಕ'
    LetterKa,
    /// \u{c96}: 'ಖ'
    LetterKha,
    /// \u{c97}: 'ಗ'
    LetterGa,
    /// \u{c98}: 'ಘ'
    LetterGha,
    /// \u{c99}: 'ಙ'
    LetterNga,
    /// \u{c9a}: 'ಚ'
    LetterCa,
    /// \u{c9b}: 'ಛ'
    LetterCha,
    /// \u{c9c}: 'ಜ'
    LetterJa,
    /// \u{c9d}: 'ಝ'
    LetterJha,
    /// \u{c9e}: 'ಞ'
    LetterNya,
    /// \u{c9f}: 'ಟ'
    LetterTta,
    /// \u{ca0}: 'ಠ'
    LetterTtha,
    /// \u{ca1}: 'ಡ'
    LetterDda,
    /// \u{ca2}: 'ಢ'
    LetterDdha,
    /// \u{ca3}: 'ಣ'
    LetterNna,
    /// \u{ca4}: 'ತ'
    LetterTa,
    /// \u{ca5}: 'ಥ'
    LetterTha,
    /// \u{ca6}: 'ದ'
    LetterDa,
    /// \u{ca7}: 'ಧ'
    LetterDha,
    /// \u{ca8}: 'ನ'
    LetterNa,
    /// \u{caa}: 'ಪ'
    LetterPa,
    /// \u{cab}: 'ಫ'
    LetterPha,
    /// \u{cac}: 'ಬ'
    LetterBa,
    /// \u{cad}: 'ಭ'
    LetterBha,
    /// \u{cae}: 'ಮ'
    LetterMa,
    /// \u{caf}: 'ಯ'
    LetterYa,
    /// \u{cb0}: 'ರ'
    LetterRa,
    /// \u{cb1}: 'ಱ'
    LetterRra,
    /// \u{cb2}: 'ಲ'
    LetterLa,
    /// \u{cb3}: 'ಳ'
    LetterLla,
    /// \u{cb5}: 'ವ'
    LetterVa,
    /// \u{cb6}: 'ಶ'
    LetterSha,
    /// \u{cb7}: 'ಷ'
    LetterSsa,
    /// \u{cb8}: 'ಸ'
    LetterSa,
    /// \u{cb9}: 'ಹ'
    LetterHa,
    /// \u{cbc}: '಼'
    SignNukta,
    /// \u{cbd}: 'ಽ'
    SignAvagraha,
    /// \u{cbe}: 'ಾ'
    VowelSignAa,
    /// \u{cbf}: 'ಿ'
    VowelSignI,
    /// \u{cc0}: 'ೀ'
    VowelSignIi,
    /// \u{cc1}: 'ು'
    VowelSignU,
    /// \u{cc2}: 'ೂ'
    VowelSignUu,
    /// \u{cc3}: 'ೃ'
    VowelSignVocalicR,
    /// \u{cc4}: 'ೄ'
    VowelSignVocalicRr,
    /// \u{cc6}: 'ೆ'
    VowelSignE,
    /// \u{cc7}: 'ೇ'
    VowelSignEe,
    /// \u{cc8}: 'ೈ'
    VowelSignAi,
    /// \u{cca}: 'ೊ'
    VowelSignO,
    /// \u{ccb}: 'ೋ'
    VowelSignOo,
    /// \u{ccc}: 'ೌ'
    VowelSignAu,
    /// \u{ccd}: '್'
    SignVirama,
    /// \u{cd5}: 'ೕ'
    LengthMark,
    /// \u{cd6}: 'ೖ'
    AiLengthMark,
    /// \u{cde}: 'ೞ'
    LetterFa,
    /// \u{ce0}: 'ೠ'
    LetterVocalicRr,
    /// \u{ce1}: 'ೡ'
    LetterVocalicLl,
    /// \u{ce2}: 'ೢ'
    VowelSignVocalicL,
    /// \u{ce3}: 'ೣ'
    VowelSignVocalicLl,
    /// \u{ce6}: '೦'
    DigitZero,
    /// \u{ce7}: '೧'
    DigitOne,
    /// \u{ce8}: '೨'
    DigitTwo,
    /// \u{ce9}: '೩'
    DigitThree,
    /// \u{cea}: '೪'
    DigitFour,
    /// \u{ceb}: '೫'
    DigitFive,
    /// \u{cec}: '೬'
    DigitSix,
    /// \u{ced}: '೭'
    DigitSeven,
    /// \u{cee}: '೮'
    DigitEight,
    /// \u{cef}: '೯'
    DigitNine,
    /// \u{cf1}: 'ೱ'
    SignJihvamuliya,
    /// \u{cf2}: 'ೲ'
    SignUpadhmaniya,
}

impl Into<char> for Kannada {
    fn into(self) -> char {
        use constants::*;
        match self {
            Kannada::SignSpacingCandrabindu => SIGN_SPACING_CANDRABINDU,
            Kannada::SignCandrabindu => SIGN_CANDRABINDU,
            Kannada::SignAnusvara => SIGN_ANUSVARA,
            Kannada::SignVisarga => SIGN_VISARGA,
            Kannada::SignSiddham => SIGN_SIDDHAM,
            Kannada::LetterA => LETTER_A,
            Kannada::LetterAa => LETTER_AA,
            Kannada::LetterI => LETTER_I,
            Kannada::LetterIi => LETTER_II,
            Kannada::LetterU => LETTER_U,
            Kannada::LetterUu => LETTER_UU,
            Kannada::LetterVocalicR => LETTER_VOCALIC_R,
            Kannada::LetterVocalicL => LETTER_VOCALIC_L,
            Kannada::LetterE => LETTER_E,
            Kannada::LetterEe => LETTER_EE,
            Kannada::LetterAi => LETTER_AI,
            Kannada::LetterO => LETTER_O,
            Kannada::LetterOo => LETTER_OO,
            Kannada::LetterAu => LETTER_AU,
            Kannada::LetterKa => LETTER_KA,
            Kannada::LetterKha => LETTER_KHA,
            Kannada::LetterGa => LETTER_GA,
            Kannada::LetterGha => LETTER_GHA,
            Kannada::LetterNga => LETTER_NGA,
            Kannada::LetterCa => LETTER_CA,
            Kannada::LetterCha => LETTER_CHA,
            Kannada::LetterJa => LETTER_JA,
            Kannada::LetterJha => LETTER_JHA,
            Kannada::LetterNya => LETTER_NYA,
            Kannada::LetterTta => LETTER_TTA,
            Kannada::LetterTtha => LETTER_TTHA,
            Kannada::LetterDda => LETTER_DDA,
            Kannada::LetterDdha => LETTER_DDHA,
            Kannada::LetterNna => LETTER_NNA,
            Kannada::LetterTa => LETTER_TA,
            Kannada::LetterTha => LETTER_THA,
            Kannada::LetterDa => LETTER_DA,
            Kannada::LetterDha => LETTER_DHA,
            Kannada::LetterNa => LETTER_NA,
            Kannada::LetterPa => LETTER_PA,
            Kannada::LetterPha => LETTER_PHA,
            Kannada::LetterBa => LETTER_BA,
            Kannada::LetterBha => LETTER_BHA,
            Kannada::LetterMa => LETTER_MA,
            Kannada::LetterYa => LETTER_YA,
            Kannada::LetterRa => LETTER_RA,
            Kannada::LetterRra => LETTER_RRA,
            Kannada::LetterLa => LETTER_LA,
            Kannada::LetterLla => LETTER_LLA,
            Kannada::LetterVa => LETTER_VA,
            Kannada::LetterSha => LETTER_SHA,
            Kannada::LetterSsa => LETTER_SSA,
            Kannada::LetterSa => LETTER_SA,
            Kannada::LetterHa => LETTER_HA,
            Kannada::SignNukta => SIGN_NUKTA,
            Kannada::SignAvagraha => SIGN_AVAGRAHA,
            Kannada::VowelSignAa => VOWEL_SIGN_AA,
            Kannada::VowelSignI => VOWEL_SIGN_I,
            Kannada::VowelSignIi => VOWEL_SIGN_II,
            Kannada::VowelSignU => VOWEL_SIGN_U,
            Kannada::VowelSignUu => VOWEL_SIGN_UU,
            Kannada::VowelSignVocalicR => VOWEL_SIGN_VOCALIC_R,
            Kannada::VowelSignVocalicRr => VOWEL_SIGN_VOCALIC_RR,
            Kannada::VowelSignE => VOWEL_SIGN_E,
            Kannada::VowelSignEe => VOWEL_SIGN_EE,
            Kannada::VowelSignAi => VOWEL_SIGN_AI,
            Kannada::VowelSignO => VOWEL_SIGN_O,
            Kannada::VowelSignOo => VOWEL_SIGN_OO,
            Kannada::VowelSignAu => VOWEL_SIGN_AU,
            Kannada::SignVirama => SIGN_VIRAMA,
            Kannada::LengthMark => LENGTH_MARK,
            Kannada::AiLengthMark => AI_LENGTH_MARK,
            Kannada::LetterFa => LETTER_FA,
            Kannada::LetterVocalicRr => LETTER_VOCALIC_RR,
            Kannada::LetterVocalicLl => LETTER_VOCALIC_LL,
            Kannada::VowelSignVocalicL => VOWEL_SIGN_VOCALIC_L,
            Kannada::VowelSignVocalicLl => VOWEL_SIGN_VOCALIC_LL,
            Kannada::DigitZero => DIGIT_ZERO,
            Kannada::DigitOne => DIGIT_ONE,
            Kannada::DigitTwo => DIGIT_TWO,
            Kannada::DigitThree => DIGIT_THREE,
            Kannada::DigitFour => DIGIT_FOUR,
            Kannada::DigitFive => DIGIT_FIVE,
            Kannada::DigitSix => DIGIT_SIX,
            Kannada::DigitSeven => DIGIT_SEVEN,
            Kannada::DigitEight => DIGIT_EIGHT,
            Kannada::DigitNine => DIGIT_NINE,
            Kannada::SignJihvamuliya => SIGN_JIHVAMULIYA,
            Kannada::SignUpadhmaniya => SIGN_UPADHMANIYA,
        }
    }
}

impl std::convert::TryFrom<char> for Kannada {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            SIGN_SPACING_CANDRABINDU => Ok(Kannada::SignSpacingCandrabindu),
            SIGN_CANDRABINDU => Ok(Kannada::SignCandrabindu),
            SIGN_ANUSVARA => Ok(Kannada::SignAnusvara),
            SIGN_VISARGA => Ok(Kannada::SignVisarga),
            SIGN_SIDDHAM => Ok(Kannada::SignSiddham),
            LETTER_A => Ok(Kannada::LetterA),
            LETTER_AA => Ok(Kannada::LetterAa),
            LETTER_I => Ok(Kannada::LetterI),
            LETTER_II => Ok(Kannada::LetterIi),
            LETTER_U => Ok(Kannada::LetterU),
            LETTER_UU => Ok(Kannada::LetterUu),
            LETTER_VOCALIC_R => Ok(Kannada::LetterVocalicR),
            LETTER_VOCALIC_L => Ok(Kannada::LetterVocalicL),
            LETTER_E => Ok(Kannada::LetterE),
            LETTER_EE => Ok(Kannada::LetterEe),
            LETTER_AI => Ok(Kannada::LetterAi),
            LETTER_O => Ok(Kannada::LetterO),
            LETTER_OO => Ok(Kannada::LetterOo),
            LETTER_AU => Ok(Kannada::LetterAu),
            LETTER_KA => Ok(Kannada::LetterKa),
            LETTER_KHA => Ok(Kannada::LetterKha),
            LETTER_GA => Ok(Kannada::LetterGa),
            LETTER_GHA => Ok(Kannada::LetterGha),
            LETTER_NGA => Ok(Kannada::LetterNga),
            LETTER_CA => Ok(Kannada::LetterCa),
            LETTER_CHA => Ok(Kannada::LetterCha),
            LETTER_JA => Ok(Kannada::LetterJa),
            LETTER_JHA => Ok(Kannada::LetterJha),
            LETTER_NYA => Ok(Kannada::LetterNya),
            LETTER_TTA => Ok(Kannada::LetterTta),
            LETTER_TTHA => Ok(Kannada::LetterTtha),
            LETTER_DDA => Ok(Kannada::LetterDda),
            LETTER_DDHA => Ok(Kannada::LetterDdha),
            LETTER_NNA => Ok(Kannada::LetterNna),
            LETTER_TA => Ok(Kannada::LetterTa),
            LETTER_THA => Ok(Kannada::LetterTha),
            LETTER_DA => Ok(Kannada::LetterDa),
            LETTER_DHA => Ok(Kannada::LetterDha),
            LETTER_NA => Ok(Kannada::LetterNa),
            LETTER_PA => Ok(Kannada::LetterPa),
            LETTER_PHA => Ok(Kannada::LetterPha),
            LETTER_BA => Ok(Kannada::LetterBa),
            LETTER_BHA => Ok(Kannada::LetterBha),
            LETTER_MA => Ok(Kannada::LetterMa),
            LETTER_YA => Ok(Kannada::LetterYa),
            LETTER_RA => Ok(Kannada::LetterRa),
            LETTER_RRA => Ok(Kannada::LetterRra),
            LETTER_LA => Ok(Kannada::LetterLa),
            LETTER_LLA => Ok(Kannada::LetterLla),
            LETTER_VA => Ok(Kannada::LetterVa),
            LETTER_SHA => Ok(Kannada::LetterSha),
            LETTER_SSA => Ok(Kannada::LetterSsa),
            LETTER_SA => Ok(Kannada::LetterSa),
            LETTER_HA => Ok(Kannada::LetterHa),
            SIGN_NUKTA => Ok(Kannada::SignNukta),
            SIGN_AVAGRAHA => Ok(Kannada::SignAvagraha),
            VOWEL_SIGN_AA => Ok(Kannada::VowelSignAa),
            VOWEL_SIGN_I => Ok(Kannada::VowelSignI),
            VOWEL_SIGN_II => Ok(Kannada::VowelSignIi),
            VOWEL_SIGN_U => Ok(Kannada::VowelSignU),
            VOWEL_SIGN_UU => Ok(Kannada::VowelSignUu),
            VOWEL_SIGN_VOCALIC_R => Ok(Kannada::VowelSignVocalicR),
            VOWEL_SIGN_VOCALIC_RR => Ok(Kannada::VowelSignVocalicRr),
            VOWEL_SIGN_E => Ok(Kannada::VowelSignE),
            VOWEL_SIGN_EE => Ok(Kannada::VowelSignEe),
            VOWEL_SIGN_AI => Ok(Kannada::VowelSignAi),
            VOWEL_SIGN_O => Ok(Kannada::VowelSignO),
            VOWEL_SIGN_OO => Ok(Kannada::VowelSignOo),
            VOWEL_SIGN_AU => Ok(Kannada::VowelSignAu),
            SIGN_VIRAMA => Ok(Kannada::SignVirama),
            LENGTH_MARK => Ok(Kannada::LengthMark),
            AI_LENGTH_MARK => Ok(Kannada::AiLengthMark),
            LETTER_FA => Ok(Kannada::LetterFa),
            LETTER_VOCALIC_RR => Ok(Kannada::LetterVocalicRr),
            LETTER_VOCALIC_LL => Ok(Kannada::LetterVocalicLl),
            VOWEL_SIGN_VOCALIC_L => Ok(Kannada::VowelSignVocalicL),
            VOWEL_SIGN_VOCALIC_LL => Ok(Kannada::VowelSignVocalicLl),
            DIGIT_ZERO => Ok(Kannada::DigitZero),
            DIGIT_ONE => Ok(Kannada::DigitOne),
            DIGIT_TWO => Ok(Kannada::DigitTwo),
            DIGIT_THREE => Ok(Kannada::DigitThree),
            DIGIT_FOUR => Ok(Kannada::DigitFour),
            DIGIT_FIVE => Ok(Kannada::DigitFive),
            DIGIT_SIX => Ok(Kannada::DigitSix),
            DIGIT_SEVEN => Ok(Kannada::DigitSeven),
            DIGIT_EIGHT => Ok(Kannada::DigitEight),
            DIGIT_NINE => Ok(Kannada::DigitNine),
            SIGN_JIHVAMULIYA => Ok(Kannada::SignJihvamuliya),
            SIGN_UPADHMANIYA => Ok(Kannada::SignUpadhmaniya),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Kannada {
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

impl std::convert::TryFrom<u32> for Kannada {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Kannada {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Kannada {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Kannada::SignSpacingCandrabindu
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Kannada::SignSpacingCandrabindu => "kannada sign spacing candrabindu",
            Kannada::SignCandrabindu => "kannada sign candrabindu",
            Kannada::SignAnusvara => "kannada sign anusvara",
            Kannada::SignVisarga => "kannada sign visarga",
            Kannada::SignSiddham => "kannada sign siddham",
            Kannada::LetterA => "kannada letter a",
            Kannada::LetterAa => "kannada letter aa",
            Kannada::LetterI => "kannada letter i",
            Kannada::LetterIi => "kannada letter ii",
            Kannada::LetterU => "kannada letter u",
            Kannada::LetterUu => "kannada letter uu",
            Kannada::LetterVocalicR => "kannada letter vocalic r",
            Kannada::LetterVocalicL => "kannada letter vocalic l",
            Kannada::LetterE => "kannada letter e",
            Kannada::LetterEe => "kannada letter ee",
            Kannada::LetterAi => "kannada letter ai",
            Kannada::LetterO => "kannada letter o",
            Kannada::LetterOo => "kannada letter oo",
            Kannada::LetterAu => "kannada letter au",
            Kannada::LetterKa => "kannada letter ka",
            Kannada::LetterKha => "kannada letter kha",
            Kannada::LetterGa => "kannada letter ga",
            Kannada::LetterGha => "kannada letter gha",
            Kannada::LetterNga => "kannada letter nga",
            Kannada::LetterCa => "kannada letter ca",
            Kannada::LetterCha => "kannada letter cha",
            Kannada::LetterJa => "kannada letter ja",
            Kannada::LetterJha => "kannada letter jha",
            Kannada::LetterNya => "kannada letter nya",
            Kannada::LetterTta => "kannada letter tta",
            Kannada::LetterTtha => "kannada letter ttha",
            Kannada::LetterDda => "kannada letter dda",
            Kannada::LetterDdha => "kannada letter ddha",
            Kannada::LetterNna => "kannada letter nna",
            Kannada::LetterTa => "kannada letter ta",
            Kannada::LetterTha => "kannada letter tha",
            Kannada::LetterDa => "kannada letter da",
            Kannada::LetterDha => "kannada letter dha",
            Kannada::LetterNa => "kannada letter na",
            Kannada::LetterPa => "kannada letter pa",
            Kannada::LetterPha => "kannada letter pha",
            Kannada::LetterBa => "kannada letter ba",
            Kannada::LetterBha => "kannada letter bha",
            Kannada::LetterMa => "kannada letter ma",
            Kannada::LetterYa => "kannada letter ya",
            Kannada::LetterRa => "kannada letter ra",
            Kannada::LetterRra => "kannada letter rra",
            Kannada::LetterLa => "kannada letter la",
            Kannada::LetterLla => "kannada letter lla",
            Kannada::LetterVa => "kannada letter va",
            Kannada::LetterSha => "kannada letter sha",
            Kannada::LetterSsa => "kannada letter ssa",
            Kannada::LetterSa => "kannada letter sa",
            Kannada::LetterHa => "kannada letter ha",
            Kannada::SignNukta => "kannada sign nukta",
            Kannada::SignAvagraha => "kannada sign avagraha",
            Kannada::VowelSignAa => "kannada vowel sign aa",
            Kannada::VowelSignI => "kannada vowel sign i",
            Kannada::VowelSignIi => "kannada vowel sign ii",
            Kannada::VowelSignU => "kannada vowel sign u",
            Kannada::VowelSignUu => "kannada vowel sign uu",
            Kannada::VowelSignVocalicR => "kannada vowel sign vocalic r",
            Kannada::VowelSignVocalicRr => "kannada vowel sign vocalic rr",
            Kannada::VowelSignE => "kannada vowel sign e",
            Kannada::VowelSignEe => "kannada vowel sign ee",
            Kannada::VowelSignAi => "kannada vowel sign ai",
            Kannada::VowelSignO => "kannada vowel sign o",
            Kannada::VowelSignOo => "kannada vowel sign oo",
            Kannada::VowelSignAu => "kannada vowel sign au",
            Kannada::SignVirama => "kannada sign virama",
            Kannada::LengthMark => "kannada length mark",
            Kannada::AiLengthMark => "kannada ai length mark",
            Kannada::LetterFa => "kannada letter fa",
            Kannada::LetterVocalicRr => "kannada letter vocalic rr",
            Kannada::LetterVocalicLl => "kannada letter vocalic ll",
            Kannada::VowelSignVocalicL => "kannada vowel sign vocalic l",
            Kannada::VowelSignVocalicLl => "kannada vowel sign vocalic ll",
            Kannada::DigitZero => "kannada digit zero",
            Kannada::DigitOne => "kannada digit one",
            Kannada::DigitTwo => "kannada digit two",
            Kannada::DigitThree => "kannada digit three",
            Kannada::DigitFour => "kannada digit four",
            Kannada::DigitFive => "kannada digit five",
            Kannada::DigitSix => "kannada digit six",
            Kannada::DigitSeven => "kannada digit seven",
            Kannada::DigitEight => "kannada digit eight",
            Kannada::DigitNine => "kannada digit nine",
            Kannada::SignJihvamuliya => "kannada sign jihvamuliya",
            Kannada::SignUpadhmaniya => "kannada sign upadhmaniya",
        }
    }
}
