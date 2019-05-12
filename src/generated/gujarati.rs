/// \u{a80} → \u{aff}
///
/// ઁ ં ઃ અ આ ઇ ઈ ઉ ઊ ઋ ઌ ઍ એ ઐ ઑ ઓ\
/// ઔ ક ખ ગ ઘ ઙ ચ છ જ ઝ ઞ ટ ઠ ડ ઢ ણ\
/// ત થ દ ધ ન પ ફ બ ભ મ ય ર લ ળ વ શ\
/// ષ સ હ ઼ ઽ ા િ ી ુ ૂ ૃ ૄ ૅ ે ૈ ૉ\
/// ો ૌ ્ ૐ ૠ ૡ ૢ ૣ ૦ ૧ ૨ ૩ ૪ ૫ ૬ ૭\
/// ૮ ૯ ૰ ૱ ૹ ૺ ૻ ૼ ૽ ૾\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{a81}: 'ઁ'
    pub const SIGN_CANDRABINDU: char = 'ઁ';
    /// \u{a82}: 'ં'
    pub const SIGN_ANUSVARA: char = 'ં';
    /// \u{a83}: 'ઃ'
    pub const SIGN_VISARGA: char = 'ઃ';
    /// \u{a85}: 'અ'
    pub const LETTER_A: char = 'અ';
    /// \u{a86}: 'આ'
    pub const LETTER_AA: char = 'આ';
    /// \u{a87}: 'ઇ'
    pub const LETTER_I: char = 'ઇ';
    /// \u{a88}: 'ઈ'
    pub const LETTER_II: char = 'ઈ';
    /// \u{a89}: 'ઉ'
    pub const LETTER_U: char = 'ઉ';
    /// \u{a8a}: 'ઊ'
    pub const LETTER_UU: char = 'ઊ';
    /// \u{a8b}: 'ઋ'
    pub const LETTER_VOCALIC_R: char = 'ઋ';
    /// \u{a8c}: 'ઌ'
    pub const LETTER_VOCALIC_L: char = 'ઌ';
    /// \u{a8d}: 'ઍ'
    pub const VOWEL_CANDRA_E: char = 'ઍ';
    /// \u{a8f}: 'એ'
    pub const LETTER_E: char = 'એ';
    /// \u{a90}: 'ઐ'
    pub const LETTER_AI: char = 'ઐ';
    /// \u{a91}: 'ઑ'
    pub const VOWEL_CANDRA_O: char = 'ઑ';
    /// \u{a93}: 'ઓ'
    pub const LETTER_O: char = 'ઓ';
    /// \u{a94}: 'ઔ'
    pub const LETTER_AU: char = 'ઔ';
    /// \u{a95}: 'ક'
    pub const LETTER_KA: char = 'ક';
    /// \u{a96}: 'ખ'
    pub const LETTER_KHA: char = 'ખ';
    /// \u{a97}: 'ગ'
    pub const LETTER_GA: char = 'ગ';
    /// \u{a98}: 'ઘ'
    pub const LETTER_GHA: char = 'ઘ';
    /// \u{a99}: 'ઙ'
    pub const LETTER_NGA: char = 'ઙ';
    /// \u{a9a}: 'ચ'
    pub const LETTER_CA: char = 'ચ';
    /// \u{a9b}: 'છ'
    pub const LETTER_CHA: char = 'છ';
    /// \u{a9c}: 'જ'
    pub const LETTER_JA: char = 'જ';
    /// \u{a9d}: 'ઝ'
    pub const LETTER_JHA: char = 'ઝ';
    /// \u{a9e}: 'ઞ'
    pub const LETTER_NYA: char = 'ઞ';
    /// \u{a9f}: 'ટ'
    pub const LETTER_TTA: char = 'ટ';
    /// \u{aa0}: 'ઠ'
    pub const LETTER_TTHA: char = 'ઠ';
    /// \u{aa1}: 'ડ'
    pub const LETTER_DDA: char = 'ડ';
    /// \u{aa2}: 'ઢ'
    pub const LETTER_DDHA: char = 'ઢ';
    /// \u{aa3}: 'ણ'
    pub const LETTER_NNA: char = 'ણ';
    /// \u{aa4}: 'ત'
    pub const LETTER_TA: char = 'ત';
    /// \u{aa5}: 'થ'
    pub const LETTER_THA: char = 'થ';
    /// \u{aa6}: 'દ'
    pub const LETTER_DA: char = 'દ';
    /// \u{aa7}: 'ધ'
    pub const LETTER_DHA: char = 'ધ';
    /// \u{aa8}: 'ન'
    pub const LETTER_NA: char = 'ન';
    /// \u{aaa}: 'પ'
    pub const LETTER_PA: char = 'પ';
    /// \u{aab}: 'ફ'
    pub const LETTER_PHA: char = 'ફ';
    /// \u{aac}: 'બ'
    pub const LETTER_BA: char = 'બ';
    /// \u{aad}: 'ભ'
    pub const LETTER_BHA: char = 'ભ';
    /// \u{aae}: 'મ'
    pub const LETTER_MA: char = 'મ';
    /// \u{aaf}: 'ય'
    pub const LETTER_YA: char = 'ય';
    /// \u{ab0}: 'ર'
    pub const LETTER_RA: char = 'ર';
    /// \u{ab2}: 'લ'
    pub const LETTER_LA: char = 'લ';
    /// \u{ab3}: 'ળ'
    pub const LETTER_LLA: char = 'ળ';
    /// \u{ab5}: 'વ'
    pub const LETTER_VA: char = 'વ';
    /// \u{ab6}: 'શ'
    pub const LETTER_SHA: char = 'શ';
    /// \u{ab7}: 'ષ'
    pub const LETTER_SSA: char = 'ષ';
    /// \u{ab8}: 'સ'
    pub const LETTER_SA: char = 'સ';
    /// \u{ab9}: 'હ'
    pub const LETTER_HA: char = 'હ';
    /// \u{abc}: '઼'
    pub const SIGN_NUKTA: char = '઼';
    /// \u{abd}: 'ઽ'
    pub const SIGN_AVAGRAHA: char = 'ઽ';
    /// \u{abe}: 'ા'
    pub const VOWEL_SIGN_AA: char = 'ા';
    /// \u{abf}: 'િ'
    pub const VOWEL_SIGN_I: char = 'િ';
    /// \u{ac0}: 'ી'
    pub const VOWEL_SIGN_II: char = 'ી';
    /// \u{ac1}: 'ુ'
    pub const VOWEL_SIGN_U: char = 'ુ';
    /// \u{ac2}: 'ૂ'
    pub const VOWEL_SIGN_UU: char = 'ૂ';
    /// \u{ac3}: 'ૃ'
    pub const VOWEL_SIGN_VOCALIC_R: char = 'ૃ';
    /// \u{ac4}: 'ૄ'
    pub const VOWEL_SIGN_VOCALIC_RR: char = 'ૄ';
    /// \u{ac5}: 'ૅ'
    pub const VOWEL_SIGN_CANDRA_E: char = 'ૅ';
    /// \u{ac7}: 'ે'
    pub const VOWEL_SIGN_E: char = 'ે';
    /// \u{ac8}: 'ૈ'
    pub const VOWEL_SIGN_AI: char = 'ૈ';
    /// \u{ac9}: 'ૉ'
    pub const VOWEL_SIGN_CANDRA_O: char = 'ૉ';
    /// \u{acb}: 'ો'
    pub const VOWEL_SIGN_O: char = 'ો';
    /// \u{acc}: 'ૌ'
    pub const VOWEL_SIGN_AU: char = 'ૌ';
    /// \u{acd}: '્'
    pub const SIGN_VIRAMA: char = '્';
    /// \u{ad0}: 'ૐ'
    pub const OM: char = 'ૐ';
    /// \u{ae0}: 'ૠ'
    pub const LETTER_VOCALIC_RR: char = 'ૠ';
    /// \u{ae1}: 'ૡ'
    pub const LETTER_VOCALIC_LL: char = 'ૡ';
    /// \u{ae2}: 'ૢ'
    pub const VOWEL_SIGN_VOCALIC_L: char = 'ૢ';
    /// \u{ae3}: 'ૣ'
    pub const VOWEL_SIGN_VOCALIC_LL: char = 'ૣ';
    /// \u{ae6}: '૦'
    pub const DIGIT_ZERO: char = '૦';
    /// \u{ae7}: '૧'
    pub const DIGIT_ONE: char = '૧';
    /// \u{ae8}: '૨'
    pub const DIGIT_TWO: char = '૨';
    /// \u{ae9}: '૩'
    pub const DIGIT_THREE: char = '૩';
    /// \u{aea}: '૪'
    pub const DIGIT_FOUR: char = '૪';
    /// \u{aeb}: '૫'
    pub const DIGIT_FIVE: char = '૫';
    /// \u{aec}: '૬'
    pub const DIGIT_SIX: char = '૬';
    /// \u{aed}: '૭'
    pub const DIGIT_SEVEN: char = '૭';
    /// \u{aee}: '૮'
    pub const DIGIT_EIGHT: char = '૮';
    /// \u{aef}: '૯'
    pub const DIGIT_NINE: char = '૯';
    /// \u{af0}: '૰'
    pub const ABBREVIATION_SIGN: char = '૰';
    /// \u{af1}: '૱'
    pub const RUPEE_SIGN: char = '૱';
    /// \u{af9}: 'ૹ'
    pub const LETTER_ZHA: char = 'ૹ';
    /// \u{afa}: 'ૺ'
    pub const SIGN_SUKUN: char = 'ૺ';
    /// \u{afb}: 'ૻ'
    pub const SIGN_SHADDA: char = 'ૻ';
    /// \u{afc}: 'ૼ'
    pub const SIGN_MADDAH: char = 'ૼ';
    /// \u{afd}: '૽'
    pub const SIGN_THREE_DASH_DOT_NUKTA_ABOVE: char = '૽';
    /// \u{afe}: '૾'
    pub const SIGN_CIRCLE_NUKTA_ABOVE: char = '૾';
}

/// An enum to represent all characters in the Gujarati block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Gujarati {
    /// \u{a81}: 'ઁ'
    SignCandrabindu,
    /// \u{a82}: 'ં'
    SignAnusvara,
    /// \u{a83}: 'ઃ'
    SignVisarga,
    /// \u{a85}: 'અ'
    LetterA,
    /// \u{a86}: 'આ'
    LetterAa,
    /// \u{a87}: 'ઇ'
    LetterI,
    /// \u{a88}: 'ઈ'
    LetterIi,
    /// \u{a89}: 'ઉ'
    LetterU,
    /// \u{a8a}: 'ઊ'
    LetterUu,
    /// \u{a8b}: 'ઋ'
    LetterVocalicR,
    /// \u{a8c}: 'ઌ'
    LetterVocalicL,
    /// \u{a8d}: 'ઍ'
    VowelCandraE,
    /// \u{a8f}: 'એ'
    LetterE,
    /// \u{a90}: 'ઐ'
    LetterAi,
    /// \u{a91}: 'ઑ'
    VowelCandraO,
    /// \u{a93}: 'ઓ'
    LetterO,
    /// \u{a94}: 'ઔ'
    LetterAu,
    /// \u{a95}: 'ક'
    LetterKa,
    /// \u{a96}: 'ખ'
    LetterKha,
    /// \u{a97}: 'ગ'
    LetterGa,
    /// \u{a98}: 'ઘ'
    LetterGha,
    /// \u{a99}: 'ઙ'
    LetterNga,
    /// \u{a9a}: 'ચ'
    LetterCa,
    /// \u{a9b}: 'છ'
    LetterCha,
    /// \u{a9c}: 'જ'
    LetterJa,
    /// \u{a9d}: 'ઝ'
    LetterJha,
    /// \u{a9e}: 'ઞ'
    LetterNya,
    /// \u{a9f}: 'ટ'
    LetterTta,
    /// \u{aa0}: 'ઠ'
    LetterTtha,
    /// \u{aa1}: 'ડ'
    LetterDda,
    /// \u{aa2}: 'ઢ'
    LetterDdha,
    /// \u{aa3}: 'ણ'
    LetterNna,
    /// \u{aa4}: 'ત'
    LetterTa,
    /// \u{aa5}: 'થ'
    LetterTha,
    /// \u{aa6}: 'દ'
    LetterDa,
    /// \u{aa7}: 'ધ'
    LetterDha,
    /// \u{aa8}: 'ન'
    LetterNa,
    /// \u{aaa}: 'પ'
    LetterPa,
    /// \u{aab}: 'ફ'
    LetterPha,
    /// \u{aac}: 'બ'
    LetterBa,
    /// \u{aad}: 'ભ'
    LetterBha,
    /// \u{aae}: 'મ'
    LetterMa,
    /// \u{aaf}: 'ય'
    LetterYa,
    /// \u{ab0}: 'ર'
    LetterRa,
    /// \u{ab2}: 'લ'
    LetterLa,
    /// \u{ab3}: 'ળ'
    LetterLla,
    /// \u{ab5}: 'વ'
    LetterVa,
    /// \u{ab6}: 'શ'
    LetterSha,
    /// \u{ab7}: 'ષ'
    LetterSsa,
    /// \u{ab8}: 'સ'
    LetterSa,
    /// \u{ab9}: 'હ'
    LetterHa,
    /// \u{abc}: '઼'
    SignNukta,
    /// \u{abd}: 'ઽ'
    SignAvagraha,
    /// \u{abe}: 'ા'
    VowelSignAa,
    /// \u{abf}: 'િ'
    VowelSignI,
    /// \u{ac0}: 'ી'
    VowelSignIi,
    /// \u{ac1}: 'ુ'
    VowelSignU,
    /// \u{ac2}: 'ૂ'
    VowelSignUu,
    /// \u{ac3}: 'ૃ'
    VowelSignVocalicR,
    /// \u{ac4}: 'ૄ'
    VowelSignVocalicRr,
    /// \u{ac5}: 'ૅ'
    VowelSignCandraE,
    /// \u{ac7}: 'ે'
    VowelSignE,
    /// \u{ac8}: 'ૈ'
    VowelSignAi,
    /// \u{ac9}: 'ૉ'
    VowelSignCandraO,
    /// \u{acb}: 'ો'
    VowelSignO,
    /// \u{acc}: 'ૌ'
    VowelSignAu,
    /// \u{acd}: '્'
    SignVirama,
    /// \u{ad0}: 'ૐ'
    Om,
    /// \u{ae0}: 'ૠ'
    LetterVocalicRr,
    /// \u{ae1}: 'ૡ'
    LetterVocalicLl,
    /// \u{ae2}: 'ૢ'
    VowelSignVocalicL,
    /// \u{ae3}: 'ૣ'
    VowelSignVocalicLl,
    /// \u{ae6}: '૦'
    DigitZero,
    /// \u{ae7}: '૧'
    DigitOne,
    /// \u{ae8}: '૨'
    DigitTwo,
    /// \u{ae9}: '૩'
    DigitThree,
    /// \u{aea}: '૪'
    DigitFour,
    /// \u{aeb}: '૫'
    DigitFive,
    /// \u{aec}: '૬'
    DigitSix,
    /// \u{aed}: '૭'
    DigitSeven,
    /// \u{aee}: '૮'
    DigitEight,
    /// \u{aef}: '૯'
    DigitNine,
    /// \u{af0}: '૰'
    AbbreviationSign,
    /// \u{af1}: '૱'
    RupeeSign,
    /// \u{af9}: 'ૹ'
    LetterZha,
    /// \u{afa}: 'ૺ'
    SignSukun,
    /// \u{afb}: 'ૻ'
    SignShadda,
    /// \u{afc}: 'ૼ'
    SignMaddah,
    /// \u{afd}: '૽'
    SignThreeDashDotNuktaAbove,
    /// \u{afe}: '૾'
    SignCircleNuktaAbove,
}

impl Into<char> for Gujarati {
    fn into(self) -> char {
        use constants::*;
        match self {
            Gujarati::SignCandrabindu => SIGN_CANDRABINDU,
            Gujarati::SignAnusvara => SIGN_ANUSVARA,
            Gujarati::SignVisarga => SIGN_VISARGA,
            Gujarati::LetterA => LETTER_A,
            Gujarati::LetterAa => LETTER_AA,
            Gujarati::LetterI => LETTER_I,
            Gujarati::LetterIi => LETTER_II,
            Gujarati::LetterU => LETTER_U,
            Gujarati::LetterUu => LETTER_UU,
            Gujarati::LetterVocalicR => LETTER_VOCALIC_R,
            Gujarati::LetterVocalicL => LETTER_VOCALIC_L,
            Gujarati::VowelCandraE => VOWEL_CANDRA_E,
            Gujarati::LetterE => LETTER_E,
            Gujarati::LetterAi => LETTER_AI,
            Gujarati::VowelCandraO => VOWEL_CANDRA_O,
            Gujarati::LetterO => LETTER_O,
            Gujarati::LetterAu => LETTER_AU,
            Gujarati::LetterKa => LETTER_KA,
            Gujarati::LetterKha => LETTER_KHA,
            Gujarati::LetterGa => LETTER_GA,
            Gujarati::LetterGha => LETTER_GHA,
            Gujarati::LetterNga => LETTER_NGA,
            Gujarati::LetterCa => LETTER_CA,
            Gujarati::LetterCha => LETTER_CHA,
            Gujarati::LetterJa => LETTER_JA,
            Gujarati::LetterJha => LETTER_JHA,
            Gujarati::LetterNya => LETTER_NYA,
            Gujarati::LetterTta => LETTER_TTA,
            Gujarati::LetterTtha => LETTER_TTHA,
            Gujarati::LetterDda => LETTER_DDA,
            Gujarati::LetterDdha => LETTER_DDHA,
            Gujarati::LetterNna => LETTER_NNA,
            Gujarati::LetterTa => LETTER_TA,
            Gujarati::LetterTha => LETTER_THA,
            Gujarati::LetterDa => LETTER_DA,
            Gujarati::LetterDha => LETTER_DHA,
            Gujarati::LetterNa => LETTER_NA,
            Gujarati::LetterPa => LETTER_PA,
            Gujarati::LetterPha => LETTER_PHA,
            Gujarati::LetterBa => LETTER_BA,
            Gujarati::LetterBha => LETTER_BHA,
            Gujarati::LetterMa => LETTER_MA,
            Gujarati::LetterYa => LETTER_YA,
            Gujarati::LetterRa => LETTER_RA,
            Gujarati::LetterLa => LETTER_LA,
            Gujarati::LetterLla => LETTER_LLA,
            Gujarati::LetterVa => LETTER_VA,
            Gujarati::LetterSha => LETTER_SHA,
            Gujarati::LetterSsa => LETTER_SSA,
            Gujarati::LetterSa => LETTER_SA,
            Gujarati::LetterHa => LETTER_HA,
            Gujarati::SignNukta => SIGN_NUKTA,
            Gujarati::SignAvagraha => SIGN_AVAGRAHA,
            Gujarati::VowelSignAa => VOWEL_SIGN_AA,
            Gujarati::VowelSignI => VOWEL_SIGN_I,
            Gujarati::VowelSignIi => VOWEL_SIGN_II,
            Gujarati::VowelSignU => VOWEL_SIGN_U,
            Gujarati::VowelSignUu => VOWEL_SIGN_UU,
            Gujarati::VowelSignVocalicR => VOWEL_SIGN_VOCALIC_R,
            Gujarati::VowelSignVocalicRr => VOWEL_SIGN_VOCALIC_RR,
            Gujarati::VowelSignCandraE => VOWEL_SIGN_CANDRA_E,
            Gujarati::VowelSignE => VOWEL_SIGN_E,
            Gujarati::VowelSignAi => VOWEL_SIGN_AI,
            Gujarati::VowelSignCandraO => VOWEL_SIGN_CANDRA_O,
            Gujarati::VowelSignO => VOWEL_SIGN_O,
            Gujarati::VowelSignAu => VOWEL_SIGN_AU,
            Gujarati::SignVirama => SIGN_VIRAMA,
            Gujarati::Om => OM,
            Gujarati::LetterVocalicRr => LETTER_VOCALIC_RR,
            Gujarati::LetterVocalicLl => LETTER_VOCALIC_LL,
            Gujarati::VowelSignVocalicL => VOWEL_SIGN_VOCALIC_L,
            Gujarati::VowelSignVocalicLl => VOWEL_SIGN_VOCALIC_LL,
            Gujarati::DigitZero => DIGIT_ZERO,
            Gujarati::DigitOne => DIGIT_ONE,
            Gujarati::DigitTwo => DIGIT_TWO,
            Gujarati::DigitThree => DIGIT_THREE,
            Gujarati::DigitFour => DIGIT_FOUR,
            Gujarati::DigitFive => DIGIT_FIVE,
            Gujarati::DigitSix => DIGIT_SIX,
            Gujarati::DigitSeven => DIGIT_SEVEN,
            Gujarati::DigitEight => DIGIT_EIGHT,
            Gujarati::DigitNine => DIGIT_NINE,
            Gujarati::AbbreviationSign => ABBREVIATION_SIGN,
            Gujarati::RupeeSign => RUPEE_SIGN,
            Gujarati::LetterZha => LETTER_ZHA,
            Gujarati::SignSukun => SIGN_SUKUN,
            Gujarati::SignShadda => SIGN_SHADDA,
            Gujarati::SignMaddah => SIGN_MADDAH,
            Gujarati::SignThreeDashDotNuktaAbove => SIGN_THREE_DASH_DOT_NUKTA_ABOVE,
            Gujarati::SignCircleNuktaAbove => SIGN_CIRCLE_NUKTA_ABOVE,
        }
    }
}

impl std::convert::TryFrom<char> for Gujarati {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            SIGN_CANDRABINDU => Ok(Gujarati::SignCandrabindu),
            SIGN_ANUSVARA => Ok(Gujarati::SignAnusvara),
            SIGN_VISARGA => Ok(Gujarati::SignVisarga),
            LETTER_A => Ok(Gujarati::LetterA),
            LETTER_AA => Ok(Gujarati::LetterAa),
            LETTER_I => Ok(Gujarati::LetterI),
            LETTER_II => Ok(Gujarati::LetterIi),
            LETTER_U => Ok(Gujarati::LetterU),
            LETTER_UU => Ok(Gujarati::LetterUu),
            LETTER_VOCALIC_R => Ok(Gujarati::LetterVocalicR),
            LETTER_VOCALIC_L => Ok(Gujarati::LetterVocalicL),
            VOWEL_CANDRA_E => Ok(Gujarati::VowelCandraE),
            LETTER_E => Ok(Gujarati::LetterE),
            LETTER_AI => Ok(Gujarati::LetterAi),
            VOWEL_CANDRA_O => Ok(Gujarati::VowelCandraO),
            LETTER_O => Ok(Gujarati::LetterO),
            LETTER_AU => Ok(Gujarati::LetterAu),
            LETTER_KA => Ok(Gujarati::LetterKa),
            LETTER_KHA => Ok(Gujarati::LetterKha),
            LETTER_GA => Ok(Gujarati::LetterGa),
            LETTER_GHA => Ok(Gujarati::LetterGha),
            LETTER_NGA => Ok(Gujarati::LetterNga),
            LETTER_CA => Ok(Gujarati::LetterCa),
            LETTER_CHA => Ok(Gujarati::LetterCha),
            LETTER_JA => Ok(Gujarati::LetterJa),
            LETTER_JHA => Ok(Gujarati::LetterJha),
            LETTER_NYA => Ok(Gujarati::LetterNya),
            LETTER_TTA => Ok(Gujarati::LetterTta),
            LETTER_TTHA => Ok(Gujarati::LetterTtha),
            LETTER_DDA => Ok(Gujarati::LetterDda),
            LETTER_DDHA => Ok(Gujarati::LetterDdha),
            LETTER_NNA => Ok(Gujarati::LetterNna),
            LETTER_TA => Ok(Gujarati::LetterTa),
            LETTER_THA => Ok(Gujarati::LetterTha),
            LETTER_DA => Ok(Gujarati::LetterDa),
            LETTER_DHA => Ok(Gujarati::LetterDha),
            LETTER_NA => Ok(Gujarati::LetterNa),
            LETTER_PA => Ok(Gujarati::LetterPa),
            LETTER_PHA => Ok(Gujarati::LetterPha),
            LETTER_BA => Ok(Gujarati::LetterBa),
            LETTER_BHA => Ok(Gujarati::LetterBha),
            LETTER_MA => Ok(Gujarati::LetterMa),
            LETTER_YA => Ok(Gujarati::LetterYa),
            LETTER_RA => Ok(Gujarati::LetterRa),
            LETTER_LA => Ok(Gujarati::LetterLa),
            LETTER_LLA => Ok(Gujarati::LetterLla),
            LETTER_VA => Ok(Gujarati::LetterVa),
            LETTER_SHA => Ok(Gujarati::LetterSha),
            LETTER_SSA => Ok(Gujarati::LetterSsa),
            LETTER_SA => Ok(Gujarati::LetterSa),
            LETTER_HA => Ok(Gujarati::LetterHa),
            SIGN_NUKTA => Ok(Gujarati::SignNukta),
            SIGN_AVAGRAHA => Ok(Gujarati::SignAvagraha),
            VOWEL_SIGN_AA => Ok(Gujarati::VowelSignAa),
            VOWEL_SIGN_I => Ok(Gujarati::VowelSignI),
            VOWEL_SIGN_II => Ok(Gujarati::VowelSignIi),
            VOWEL_SIGN_U => Ok(Gujarati::VowelSignU),
            VOWEL_SIGN_UU => Ok(Gujarati::VowelSignUu),
            VOWEL_SIGN_VOCALIC_R => Ok(Gujarati::VowelSignVocalicR),
            VOWEL_SIGN_VOCALIC_RR => Ok(Gujarati::VowelSignVocalicRr),
            VOWEL_SIGN_CANDRA_E => Ok(Gujarati::VowelSignCandraE),
            VOWEL_SIGN_E => Ok(Gujarati::VowelSignE),
            VOWEL_SIGN_AI => Ok(Gujarati::VowelSignAi),
            VOWEL_SIGN_CANDRA_O => Ok(Gujarati::VowelSignCandraO),
            VOWEL_SIGN_O => Ok(Gujarati::VowelSignO),
            VOWEL_SIGN_AU => Ok(Gujarati::VowelSignAu),
            SIGN_VIRAMA => Ok(Gujarati::SignVirama),
            OM => Ok(Gujarati::Om),
            LETTER_VOCALIC_RR => Ok(Gujarati::LetterVocalicRr),
            LETTER_VOCALIC_LL => Ok(Gujarati::LetterVocalicLl),
            VOWEL_SIGN_VOCALIC_L => Ok(Gujarati::VowelSignVocalicL),
            VOWEL_SIGN_VOCALIC_LL => Ok(Gujarati::VowelSignVocalicLl),
            DIGIT_ZERO => Ok(Gujarati::DigitZero),
            DIGIT_ONE => Ok(Gujarati::DigitOne),
            DIGIT_TWO => Ok(Gujarati::DigitTwo),
            DIGIT_THREE => Ok(Gujarati::DigitThree),
            DIGIT_FOUR => Ok(Gujarati::DigitFour),
            DIGIT_FIVE => Ok(Gujarati::DigitFive),
            DIGIT_SIX => Ok(Gujarati::DigitSix),
            DIGIT_SEVEN => Ok(Gujarati::DigitSeven),
            DIGIT_EIGHT => Ok(Gujarati::DigitEight),
            DIGIT_NINE => Ok(Gujarati::DigitNine),
            ABBREVIATION_SIGN => Ok(Gujarati::AbbreviationSign),
            RUPEE_SIGN => Ok(Gujarati::RupeeSign),
            LETTER_ZHA => Ok(Gujarati::LetterZha),
            SIGN_SUKUN => Ok(Gujarati::SignSukun),
            SIGN_SHADDA => Ok(Gujarati::SignShadda),
            SIGN_MADDAH => Ok(Gujarati::SignMaddah),
            SIGN_THREE_DASH_DOT_NUKTA_ABOVE => Ok(Gujarati::SignThreeDashDotNuktaAbove),
            SIGN_CIRCLE_NUKTA_ABOVE => Ok(Gujarati::SignCircleNuktaAbove),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Gujarati {
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

impl std::convert::TryFrom<u32> for Gujarati {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Gujarati {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Gujarati {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Gujarati::SignCandrabindu
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Gujarati::SignCandrabindu => "gujarati sign candrabindu",
            Gujarati::SignAnusvara => "gujarati sign anusvara",
            Gujarati::SignVisarga => "gujarati sign visarga",
            Gujarati::LetterA => "gujarati letter a",
            Gujarati::LetterAa => "gujarati letter aa",
            Gujarati::LetterI => "gujarati letter i",
            Gujarati::LetterIi => "gujarati letter ii",
            Gujarati::LetterU => "gujarati letter u",
            Gujarati::LetterUu => "gujarati letter uu",
            Gujarati::LetterVocalicR => "gujarati letter vocalic r",
            Gujarati::LetterVocalicL => "gujarati letter vocalic l",
            Gujarati::VowelCandraE => "gujarati vowel candra e",
            Gujarati::LetterE => "gujarati letter e",
            Gujarati::LetterAi => "gujarati letter ai",
            Gujarati::VowelCandraO => "gujarati vowel candra o",
            Gujarati::LetterO => "gujarati letter o",
            Gujarati::LetterAu => "gujarati letter au",
            Gujarati::LetterKa => "gujarati letter ka",
            Gujarati::LetterKha => "gujarati letter kha",
            Gujarati::LetterGa => "gujarati letter ga",
            Gujarati::LetterGha => "gujarati letter gha",
            Gujarati::LetterNga => "gujarati letter nga",
            Gujarati::LetterCa => "gujarati letter ca",
            Gujarati::LetterCha => "gujarati letter cha",
            Gujarati::LetterJa => "gujarati letter ja",
            Gujarati::LetterJha => "gujarati letter jha",
            Gujarati::LetterNya => "gujarati letter nya",
            Gujarati::LetterTta => "gujarati letter tta",
            Gujarati::LetterTtha => "gujarati letter ttha",
            Gujarati::LetterDda => "gujarati letter dda",
            Gujarati::LetterDdha => "gujarati letter ddha",
            Gujarati::LetterNna => "gujarati letter nna",
            Gujarati::LetterTa => "gujarati letter ta",
            Gujarati::LetterTha => "gujarati letter tha",
            Gujarati::LetterDa => "gujarati letter da",
            Gujarati::LetterDha => "gujarati letter dha",
            Gujarati::LetterNa => "gujarati letter na",
            Gujarati::LetterPa => "gujarati letter pa",
            Gujarati::LetterPha => "gujarati letter pha",
            Gujarati::LetterBa => "gujarati letter ba",
            Gujarati::LetterBha => "gujarati letter bha",
            Gujarati::LetterMa => "gujarati letter ma",
            Gujarati::LetterYa => "gujarati letter ya",
            Gujarati::LetterRa => "gujarati letter ra",
            Gujarati::LetterLa => "gujarati letter la",
            Gujarati::LetterLla => "gujarati letter lla",
            Gujarati::LetterVa => "gujarati letter va",
            Gujarati::LetterSha => "gujarati letter sha",
            Gujarati::LetterSsa => "gujarati letter ssa",
            Gujarati::LetterSa => "gujarati letter sa",
            Gujarati::LetterHa => "gujarati letter ha",
            Gujarati::SignNukta => "gujarati sign nukta",
            Gujarati::SignAvagraha => "gujarati sign avagraha",
            Gujarati::VowelSignAa => "gujarati vowel sign aa",
            Gujarati::VowelSignI => "gujarati vowel sign i",
            Gujarati::VowelSignIi => "gujarati vowel sign ii",
            Gujarati::VowelSignU => "gujarati vowel sign u",
            Gujarati::VowelSignUu => "gujarati vowel sign uu",
            Gujarati::VowelSignVocalicR => "gujarati vowel sign vocalic r",
            Gujarati::VowelSignVocalicRr => "gujarati vowel sign vocalic rr",
            Gujarati::VowelSignCandraE => "gujarati vowel sign candra e",
            Gujarati::VowelSignE => "gujarati vowel sign e",
            Gujarati::VowelSignAi => "gujarati vowel sign ai",
            Gujarati::VowelSignCandraO => "gujarati vowel sign candra o",
            Gujarati::VowelSignO => "gujarati vowel sign o",
            Gujarati::VowelSignAu => "gujarati vowel sign au",
            Gujarati::SignVirama => "gujarati sign virama",
            Gujarati::Om => "gujarati om",
            Gujarati::LetterVocalicRr => "gujarati letter vocalic rr",
            Gujarati::LetterVocalicLl => "gujarati letter vocalic ll",
            Gujarati::VowelSignVocalicL => "gujarati vowel sign vocalic l",
            Gujarati::VowelSignVocalicLl => "gujarati vowel sign vocalic ll",
            Gujarati::DigitZero => "gujarati digit zero",
            Gujarati::DigitOne => "gujarati digit one",
            Gujarati::DigitTwo => "gujarati digit two",
            Gujarati::DigitThree => "gujarati digit three",
            Gujarati::DigitFour => "gujarati digit four",
            Gujarati::DigitFive => "gujarati digit five",
            Gujarati::DigitSix => "gujarati digit six",
            Gujarati::DigitSeven => "gujarati digit seven",
            Gujarati::DigitEight => "gujarati digit eight",
            Gujarati::DigitNine => "gujarati digit nine",
            Gujarati::AbbreviationSign => "gujarati abbreviation sign",
            Gujarati::RupeeSign => "gujarati rupee sign",
            Gujarati::LetterZha => "gujarati letter zha",
            Gujarati::SignSukun => "gujarati sign sukun",
            Gujarati::SignShadda => "gujarati sign shadda",
            Gujarati::SignMaddah => "gujarati sign maddah",
            Gujarati::SignThreeDashDotNuktaAbove => "gujarati sign three-dot nukta above",
            Gujarati::SignCircleNuktaAbove => "gujarati sign circle nukta above",
        }
    }
}
