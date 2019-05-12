/// \u{c00} → \u{c7f}\
///\
/// ఀ ఁ ం ః ఄ అ ఆ ఇ ఈ ఉ ఊ ఋ ఌ ఎ ఏ ఐ
/// ఒ ఓ ఔ క ఖ గ ఘ ఙ చ ఛ జ ఝ ఞ ట ఠ డ
/// ఢ ణ త థ ద ధ న ప ఫ బ భ మ య ర ఱ ల
/// ళ ఴ వ శ ష స హ ఽ ా ి ీ ు ూ ృ ౄ ె
/// ే ై ొ ో ౌ ్ ౕ ౖ ౘ ౙ ౚ ౠ ౡ ౢ ౣ ౦
/// ౧ ౨ ౩ ౪ ౫ ౬ ౭ ౮ ౯ ౷ ౸ ౹ ౺ ౻ ౼ ౽
/// ౾
pub mod constants {
    /// \u{c00}: 'ఀ'
    pub const TELUGU_SIGN_COMBINING_CANDRABINDU_ABOVE: char = 'ఀ';
    /// \u{c01}: 'ఁ'
    pub const TELUGU_SIGN_CANDRABINDU: char = 'ఁ';
    /// \u{c02}: 'ం'
    pub const TELUGU_SIGN_ANUSVARA: char = 'ం';
    /// \u{c03}: 'ః'
    pub const TELUGU_SIGN_VISARGA: char = 'ః';
    /// \u{c04}: 'ఄ'
    pub const TELUGU_SIGN_COMBINING_ANUSVARA_ABOVE: char = 'ఄ';
    /// \u{c05}: 'అ'
    pub const TELUGU_LETTER_A: char = 'అ';
    /// \u{c06}: 'ఆ'
    pub const TELUGU_LETTER_AA: char = 'ఆ';
    /// \u{c07}: 'ఇ'
    pub const TELUGU_LETTER_I: char = 'ఇ';
    /// \u{c08}: 'ఈ'
    pub const TELUGU_LETTER_II: char = 'ఈ';
    /// \u{c09}: 'ఉ'
    pub const TELUGU_LETTER_U: char = 'ఉ';
    /// \u{c0a}: 'ఊ'
    pub const TELUGU_LETTER_UU: char = 'ఊ';
    /// \u{c0b}: 'ఋ'
    pub const TELUGU_LETTER_VOCALIC_R: char = 'ఋ';
    /// \u{c0c}: 'ఌ'
    pub const TELUGU_LETTER_VOCALIC_L: char = 'ఌ';
    /// \u{c0e}: 'ఎ'
    pub const TELUGU_LETTER_E: char = 'ఎ';
    /// \u{c0f}: 'ఏ'
    pub const TELUGU_LETTER_EE: char = 'ఏ';
    /// \u{c10}: 'ఐ'
    pub const TELUGU_LETTER_AI: char = 'ఐ';
    /// \u{c12}: 'ఒ'
    pub const TELUGU_LETTER_O: char = 'ఒ';
    /// \u{c13}: 'ఓ'
    pub const TELUGU_LETTER_OO: char = 'ఓ';
    /// \u{c14}: 'ఔ'
    pub const TELUGU_LETTER_AU: char = 'ఔ';
    /// \u{c15}: 'క'
    pub const TELUGU_LETTER_KA: char = 'క';
    /// \u{c16}: 'ఖ'
    pub const TELUGU_LETTER_KHA: char = 'ఖ';
    /// \u{c17}: 'గ'
    pub const TELUGU_LETTER_GA: char = 'గ';
    /// \u{c18}: 'ఘ'
    pub const TELUGU_LETTER_GHA: char = 'ఘ';
    /// \u{c19}: 'ఙ'
    pub const TELUGU_LETTER_NGA: char = 'ఙ';
    /// \u{c1a}: 'చ'
    pub const TELUGU_LETTER_CA: char = 'చ';
    /// \u{c1b}: 'ఛ'
    pub const TELUGU_LETTER_CHA: char = 'ఛ';
    /// \u{c1c}: 'జ'
    pub const TELUGU_LETTER_JA: char = 'జ';
    /// \u{c1d}: 'ఝ'
    pub const TELUGU_LETTER_JHA: char = 'ఝ';
    /// \u{c1e}: 'ఞ'
    pub const TELUGU_LETTER_NYA: char = 'ఞ';
    /// \u{c1f}: 'ట'
    pub const TELUGU_LETTER_TTA: char = 'ట';
    /// \u{c20}: 'ఠ'
    pub const TELUGU_LETTER_TTHA: char = 'ఠ';
    /// \u{c21}: 'డ'
    pub const TELUGU_LETTER_DDA: char = 'డ';
    /// \u{c22}: 'ఢ'
    pub const TELUGU_LETTER_DDHA: char = 'ఢ';
    /// \u{c23}: 'ణ'
    pub const TELUGU_LETTER_NNA: char = 'ణ';
    /// \u{c24}: 'త'
    pub const TELUGU_LETTER_TA: char = 'త';
    /// \u{c25}: 'థ'
    pub const TELUGU_LETTER_THA: char = 'థ';
    /// \u{c26}: 'ద'
    pub const TELUGU_LETTER_DA: char = 'ద';
    /// \u{c27}: 'ధ'
    pub const TELUGU_LETTER_DHA: char = 'ధ';
    /// \u{c28}: 'న'
    pub const TELUGU_LETTER_NA: char = 'న';
    /// \u{c2a}: 'ప'
    pub const TELUGU_LETTER_PA: char = 'ప';
    /// \u{c2b}: 'ఫ'
    pub const TELUGU_LETTER_PHA: char = 'ఫ';
    /// \u{c2c}: 'బ'
    pub const TELUGU_LETTER_BA: char = 'బ';
    /// \u{c2d}: 'భ'
    pub const TELUGU_LETTER_BHA: char = 'భ';
    /// \u{c2e}: 'మ'
    pub const TELUGU_LETTER_MA: char = 'మ';
    /// \u{c2f}: 'య'
    pub const TELUGU_LETTER_YA: char = 'య';
    /// \u{c30}: 'ర'
    pub const TELUGU_LETTER_RA: char = 'ర';
    /// \u{c31}: 'ఱ'
    pub const TELUGU_LETTER_RRA: char = 'ఱ';
    /// \u{c32}: 'ల'
    pub const TELUGU_LETTER_LA: char = 'ల';
    /// \u{c33}: 'ళ'
    pub const TELUGU_LETTER_LLA: char = 'ళ';
    /// \u{c34}: 'ఴ'
    pub const TELUGU_LETTER_LLLA: char = 'ఴ';
    /// \u{c35}: 'వ'
    pub const TELUGU_LETTER_VA: char = 'వ';
    /// \u{c36}: 'శ'
    pub const TELUGU_LETTER_SHA: char = 'శ';
    /// \u{c37}: 'ష'
    pub const TELUGU_LETTER_SSA: char = 'ష';
    /// \u{c38}: 'స'
    pub const TELUGU_LETTER_SA: char = 'స';
    /// \u{c39}: 'హ'
    pub const TELUGU_LETTER_HA: char = 'హ';
    /// \u{c3d}: 'ఽ'
    pub const TELUGU_SIGN_AVAGRAHA: char = 'ఽ';
    /// \u{c3e}: 'ా'
    pub const TELUGU_VOWEL_SIGN_AA: char = 'ా';
    /// \u{c3f}: 'ి'
    pub const TELUGU_VOWEL_SIGN_I: char = 'ి';
    /// \u{c40}: 'ీ'
    pub const TELUGU_VOWEL_SIGN_II: char = 'ీ';
    /// \u{c41}: 'ు'
    pub const TELUGU_VOWEL_SIGN_U: char = 'ు';
    /// \u{c42}: 'ూ'
    pub const TELUGU_VOWEL_SIGN_UU: char = 'ూ';
    /// \u{c43}: 'ృ'
    pub const TELUGU_VOWEL_SIGN_VOCALIC_R: char = 'ృ';
    /// \u{c44}: 'ౄ'
    pub const TELUGU_VOWEL_SIGN_VOCALIC_RR: char = 'ౄ';
    /// \u{c46}: 'ె'
    pub const TELUGU_VOWEL_SIGN_E: char = 'ె';
    /// \u{c47}: 'ే'
    pub const TELUGU_VOWEL_SIGN_EE: char = 'ే';
    /// \u{c48}: 'ై'
    pub const TELUGU_VOWEL_SIGN_AI: char = 'ై';
    /// \u{c4a}: 'ొ'
    pub const TELUGU_VOWEL_SIGN_O: char = 'ొ';
    /// \u{c4b}: 'ో'
    pub const TELUGU_VOWEL_SIGN_OO: char = 'ో';
    /// \u{c4c}: 'ౌ'
    pub const TELUGU_VOWEL_SIGN_AU: char = 'ౌ';
    /// \u{c4d}: '్'
    pub const TELUGU_SIGN_VIRAMA: char = '్';
    /// \u{c55}: 'ౕ'
    pub const TELUGU_LENGTH_MARK: char = 'ౕ';
    /// \u{c56}: 'ౖ'
    pub const TELUGU_AI_LENGTH_MARK: char = 'ౖ';
    /// \u{c58}: 'ౘ'
    pub const TELUGU_LETTER_TSA: char = 'ౘ';
    /// \u{c59}: 'ౙ'
    pub const TELUGU_LETTER_DZA: char = 'ౙ';
    /// \u{c5a}: 'ౚ'
    pub const TELUGU_LETTER_RRRA: char = 'ౚ';
    /// \u{c60}: 'ౠ'
    pub const TELUGU_LETTER_VOCALIC_RR: char = 'ౠ';
    /// \u{c61}: 'ౡ'
    pub const TELUGU_LETTER_VOCALIC_LL: char = 'ౡ';
    /// \u{c62}: 'ౢ'
    pub const TELUGU_VOWEL_SIGN_VOCALIC_L: char = 'ౢ';
    /// \u{c63}: 'ౣ'
    pub const TELUGU_VOWEL_SIGN_VOCALIC_LL: char = 'ౣ';
    /// \u{c66}: '౦'
    pub const TELUGU_DIGIT_ZERO: char = '౦';
    /// \u{c67}: '౧'
    pub const TELUGU_DIGIT_ONE: char = '౧';
    /// \u{c68}: '౨'
    pub const TELUGU_DIGIT_TWO: char = '౨';
    /// \u{c69}: '౩'
    pub const TELUGU_DIGIT_THREE: char = '౩';
    /// \u{c6a}: '౪'
    pub const TELUGU_DIGIT_FOUR: char = '౪';
    /// \u{c6b}: '౫'
    pub const TELUGU_DIGIT_FIVE: char = '౫';
    /// \u{c6c}: '౬'
    pub const TELUGU_DIGIT_SIX: char = '౬';
    /// \u{c6d}: '౭'
    pub const TELUGU_DIGIT_SEVEN: char = '౭';
    /// \u{c6e}: '౮'
    pub const TELUGU_DIGIT_EIGHT: char = '౮';
    /// \u{c6f}: '౯'
    pub const TELUGU_DIGIT_NINE: char = '౯';
    /// \u{c77}: '౷'
    pub const TELUGU_SIGN_SIDDHAM: char = '౷';
    /// \u{c78}: '౸'
    pub const TELUGU_FRACTION_DIGIT_ZERO_FOR_ODD_POWERS_OF_FOUR: char = '౸';
    /// \u{c79}: '౹'
    pub const TELUGU_FRACTION_DIGIT_ONE_FOR_ODD_POWERS_OF_FOUR: char = '౹';
    /// \u{c7a}: '౺'
    pub const TELUGU_FRACTION_DIGIT_TWO_FOR_ODD_POWERS_OF_FOUR: char = '౺';
    /// \u{c7b}: '౻'
    pub const TELUGU_FRACTION_DIGIT_THREE_FOR_ODD_POWERS_OF_FOUR: char = '౻';
    /// \u{c7c}: '౼'
    pub const TELUGU_FRACTION_DIGIT_ONE_FOR_EVEN_POWERS_OF_FOUR: char = '౼';
    /// \u{c7d}: '౽'
    pub const TELUGU_FRACTION_DIGIT_TWO_FOR_EVEN_POWERS_OF_FOUR: char = '౽';
    /// \u{c7e}: '౾'
    pub const TELUGU_FRACTION_DIGIT_THREE_FOR_EVEN_POWERS_OF_FOUR: char = '౾';
}

/// \u{c00} → \u{c7f}\
///\
/// ఀ ఁ ం ః ఄ అ ఆ ఇ ఈ ఉ ఊ ఋ ఌ ఎ ఏ ఐ
/// ఒ ఓ ఔ క ఖ గ ఘ ఙ చ ఛ జ ఝ ఞ ట ఠ డ
/// ఢ ణ త థ ద ధ న ప ఫ బ భ మ య ర ఱ ల
/// ళ ఴ వ శ ష స హ ఽ ా ి ీ ు ూ ృ ౄ ె
/// ే ై ొ ో ౌ ్ ౕ ౖ ౘ ౙ ౚ ౠ ౡ ౢ ౣ ౦
/// ౧ ౨ ౩ ౪ ౫ ౬ ౭ ౮ ౯ ౷ ౸ ౹ ౺ ౻ ౼ ౽
/// ౾
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Telugu {
    /// \u{c00}: 'ఀ'
    TeluguSignCombiningCandrabinduAbove,
    /// \u{c01}: 'ఁ'
    TeluguSignCandrabindu,
    /// \u{c02}: 'ం'
    TeluguSignAnusvara,
    /// \u{c03}: 'ః'
    TeluguSignVisarga,
    /// \u{c04}: 'ఄ'
    TeluguSignCombiningAnusvaraAbove,
    /// \u{c05}: 'అ'
    TeluguLetterA,
    /// \u{c06}: 'ఆ'
    TeluguLetterAa,
    /// \u{c07}: 'ఇ'
    TeluguLetterI,
    /// \u{c08}: 'ఈ'
    TeluguLetterIi,
    /// \u{c09}: 'ఉ'
    TeluguLetterU,
    /// \u{c0a}: 'ఊ'
    TeluguLetterUu,
    /// \u{c0b}: 'ఋ'
    TeluguLetterVocalicR,
    /// \u{c0c}: 'ఌ'
    TeluguLetterVocalicL,
    /// \u{c0e}: 'ఎ'
    TeluguLetterE,
    /// \u{c0f}: 'ఏ'
    TeluguLetterEe,
    /// \u{c10}: 'ఐ'
    TeluguLetterAi,
    /// \u{c12}: 'ఒ'
    TeluguLetterO,
    /// \u{c13}: 'ఓ'
    TeluguLetterOo,
    /// \u{c14}: 'ఔ'
    TeluguLetterAu,
    /// \u{c15}: 'క'
    TeluguLetterKa,
    /// \u{c16}: 'ఖ'
    TeluguLetterKha,
    /// \u{c17}: 'గ'
    TeluguLetterGa,
    /// \u{c18}: 'ఘ'
    TeluguLetterGha,
    /// \u{c19}: 'ఙ'
    TeluguLetterNga,
    /// \u{c1a}: 'చ'
    TeluguLetterCa,
    /// \u{c1b}: 'ఛ'
    TeluguLetterCha,
    /// \u{c1c}: 'జ'
    TeluguLetterJa,
    /// \u{c1d}: 'ఝ'
    TeluguLetterJha,
    /// \u{c1e}: 'ఞ'
    TeluguLetterNya,
    /// \u{c1f}: 'ట'
    TeluguLetterTta,
    /// \u{c20}: 'ఠ'
    TeluguLetterTtha,
    /// \u{c21}: 'డ'
    TeluguLetterDda,
    /// \u{c22}: 'ఢ'
    TeluguLetterDdha,
    /// \u{c23}: 'ణ'
    TeluguLetterNna,
    /// \u{c24}: 'త'
    TeluguLetterTa,
    /// \u{c25}: 'థ'
    TeluguLetterTha,
    /// \u{c26}: 'ద'
    TeluguLetterDa,
    /// \u{c27}: 'ధ'
    TeluguLetterDha,
    /// \u{c28}: 'న'
    TeluguLetterNa,
    /// \u{c2a}: 'ప'
    TeluguLetterPa,
    /// \u{c2b}: 'ఫ'
    TeluguLetterPha,
    /// \u{c2c}: 'బ'
    TeluguLetterBa,
    /// \u{c2d}: 'భ'
    TeluguLetterBha,
    /// \u{c2e}: 'మ'
    TeluguLetterMa,
    /// \u{c2f}: 'య'
    TeluguLetterYa,
    /// \u{c30}: 'ర'
    TeluguLetterRa,
    /// \u{c31}: 'ఱ'
    TeluguLetterRra,
    /// \u{c32}: 'ల'
    TeluguLetterLa,
    /// \u{c33}: 'ళ'
    TeluguLetterLla,
    /// \u{c34}: 'ఴ'
    TeluguLetterLlla,
    /// \u{c35}: 'వ'
    TeluguLetterVa,
    /// \u{c36}: 'శ'
    TeluguLetterSha,
    /// \u{c37}: 'ష'
    TeluguLetterSsa,
    /// \u{c38}: 'స'
    TeluguLetterSa,
    /// \u{c39}: 'హ'
    TeluguLetterHa,
    /// \u{c3d}: 'ఽ'
    TeluguSignAvagraha,
    /// \u{c3e}: 'ా'
    TeluguVowelSignAa,
    /// \u{c3f}: 'ి'
    TeluguVowelSignI,
    /// \u{c40}: 'ీ'
    TeluguVowelSignIi,
    /// \u{c41}: 'ు'
    TeluguVowelSignU,
    /// \u{c42}: 'ూ'
    TeluguVowelSignUu,
    /// \u{c43}: 'ృ'
    TeluguVowelSignVocalicR,
    /// \u{c44}: 'ౄ'
    TeluguVowelSignVocalicRr,
    /// \u{c46}: 'ె'
    TeluguVowelSignE,
    /// \u{c47}: 'ే'
    TeluguVowelSignEe,
    /// \u{c48}: 'ై'
    TeluguVowelSignAi,
    /// \u{c4a}: 'ొ'
    TeluguVowelSignO,
    /// \u{c4b}: 'ో'
    TeluguVowelSignOo,
    /// \u{c4c}: 'ౌ'
    TeluguVowelSignAu,
    /// \u{c4d}: '్'
    TeluguSignVirama,
    /// \u{c55}: 'ౕ'
    TeluguLengthMark,
    /// \u{c56}: 'ౖ'
    TeluguAiLengthMark,
    /// \u{c58}: 'ౘ'
    TeluguLetterTsa,
    /// \u{c59}: 'ౙ'
    TeluguLetterDza,
    /// \u{c5a}: 'ౚ'
    TeluguLetterRrra,
    /// \u{c60}: 'ౠ'
    TeluguLetterVocalicRr,
    /// \u{c61}: 'ౡ'
    TeluguLetterVocalicLl,
    /// \u{c62}: 'ౢ'
    TeluguVowelSignVocalicL,
    /// \u{c63}: 'ౣ'
    TeluguVowelSignVocalicLl,
    /// \u{c66}: '౦'
    TeluguDigitZero,
    /// \u{c67}: '౧'
    TeluguDigitOne,
    /// \u{c68}: '౨'
    TeluguDigitTwo,
    /// \u{c69}: '౩'
    TeluguDigitThree,
    /// \u{c6a}: '౪'
    TeluguDigitFour,
    /// \u{c6b}: '౫'
    TeluguDigitFive,
    /// \u{c6c}: '౬'
    TeluguDigitSix,
    /// \u{c6d}: '౭'
    TeluguDigitSeven,
    /// \u{c6e}: '౮'
    TeluguDigitEight,
    /// \u{c6f}: '౯'
    TeluguDigitNine,
    /// \u{c77}: '౷'
    TeluguSignSiddham,
    /// \u{c78}: '౸'
    TeluguFractionDigitZeroForOddPowersOfFour,
    /// \u{c79}: '౹'
    TeluguFractionDigitOneForOddPowersOfFour,
    /// \u{c7a}: '౺'
    TeluguFractionDigitTwoForOddPowersOfFour,
    /// \u{c7b}: '౻'
    TeluguFractionDigitThreeForOddPowersOfFour,
    /// \u{c7c}: '౼'
    TeluguFractionDigitOneForEvenPowersOfFour,
    /// \u{c7d}: '౽'
    TeluguFractionDigitTwoForEvenPowersOfFour,
    /// \u{c7e}: '౾'
    TeluguFractionDigitThreeForEvenPowersOfFour,
}

impl Into<char> for Telugu {
    fn into(self) -> char {
        use constants::*;
        match self {
            Telugu::TeluguSignCombiningCandrabinduAbove => TELUGU_SIGN_COMBINING_CANDRABINDU_ABOVE,
            Telugu::TeluguSignCandrabindu => TELUGU_SIGN_CANDRABINDU,
            Telugu::TeluguSignAnusvara => TELUGU_SIGN_ANUSVARA,
            Telugu::TeluguSignVisarga => TELUGU_SIGN_VISARGA,
            Telugu::TeluguSignCombiningAnusvaraAbove => TELUGU_SIGN_COMBINING_ANUSVARA_ABOVE,
            Telugu::TeluguLetterA => TELUGU_LETTER_A,
            Telugu::TeluguLetterAa => TELUGU_LETTER_AA,
            Telugu::TeluguLetterI => TELUGU_LETTER_I,
            Telugu::TeluguLetterIi => TELUGU_LETTER_II,
            Telugu::TeluguLetterU => TELUGU_LETTER_U,
            Telugu::TeluguLetterUu => TELUGU_LETTER_UU,
            Telugu::TeluguLetterVocalicR => TELUGU_LETTER_VOCALIC_R,
            Telugu::TeluguLetterVocalicL => TELUGU_LETTER_VOCALIC_L,
            Telugu::TeluguLetterE => TELUGU_LETTER_E,
            Telugu::TeluguLetterEe => TELUGU_LETTER_EE,
            Telugu::TeluguLetterAi => TELUGU_LETTER_AI,
            Telugu::TeluguLetterO => TELUGU_LETTER_O,
            Telugu::TeluguLetterOo => TELUGU_LETTER_OO,
            Telugu::TeluguLetterAu => TELUGU_LETTER_AU,
            Telugu::TeluguLetterKa => TELUGU_LETTER_KA,
            Telugu::TeluguLetterKha => TELUGU_LETTER_KHA,
            Telugu::TeluguLetterGa => TELUGU_LETTER_GA,
            Telugu::TeluguLetterGha => TELUGU_LETTER_GHA,
            Telugu::TeluguLetterNga => TELUGU_LETTER_NGA,
            Telugu::TeluguLetterCa => TELUGU_LETTER_CA,
            Telugu::TeluguLetterCha => TELUGU_LETTER_CHA,
            Telugu::TeluguLetterJa => TELUGU_LETTER_JA,
            Telugu::TeluguLetterJha => TELUGU_LETTER_JHA,
            Telugu::TeluguLetterNya => TELUGU_LETTER_NYA,
            Telugu::TeluguLetterTta => TELUGU_LETTER_TTA,
            Telugu::TeluguLetterTtha => TELUGU_LETTER_TTHA,
            Telugu::TeluguLetterDda => TELUGU_LETTER_DDA,
            Telugu::TeluguLetterDdha => TELUGU_LETTER_DDHA,
            Telugu::TeluguLetterNna => TELUGU_LETTER_NNA,
            Telugu::TeluguLetterTa => TELUGU_LETTER_TA,
            Telugu::TeluguLetterTha => TELUGU_LETTER_THA,
            Telugu::TeluguLetterDa => TELUGU_LETTER_DA,
            Telugu::TeluguLetterDha => TELUGU_LETTER_DHA,
            Telugu::TeluguLetterNa => TELUGU_LETTER_NA,
            Telugu::TeluguLetterPa => TELUGU_LETTER_PA,
            Telugu::TeluguLetterPha => TELUGU_LETTER_PHA,
            Telugu::TeluguLetterBa => TELUGU_LETTER_BA,
            Telugu::TeluguLetterBha => TELUGU_LETTER_BHA,
            Telugu::TeluguLetterMa => TELUGU_LETTER_MA,
            Telugu::TeluguLetterYa => TELUGU_LETTER_YA,
            Telugu::TeluguLetterRa => TELUGU_LETTER_RA,
            Telugu::TeluguLetterRra => TELUGU_LETTER_RRA,
            Telugu::TeluguLetterLa => TELUGU_LETTER_LA,
            Telugu::TeluguLetterLla => TELUGU_LETTER_LLA,
            Telugu::TeluguLetterLlla => TELUGU_LETTER_LLLA,
            Telugu::TeluguLetterVa => TELUGU_LETTER_VA,
            Telugu::TeluguLetterSha => TELUGU_LETTER_SHA,
            Telugu::TeluguLetterSsa => TELUGU_LETTER_SSA,
            Telugu::TeluguLetterSa => TELUGU_LETTER_SA,
            Telugu::TeluguLetterHa => TELUGU_LETTER_HA,
            Telugu::TeluguSignAvagraha => TELUGU_SIGN_AVAGRAHA,
            Telugu::TeluguVowelSignAa => TELUGU_VOWEL_SIGN_AA,
            Telugu::TeluguVowelSignI => TELUGU_VOWEL_SIGN_I,
            Telugu::TeluguVowelSignIi => TELUGU_VOWEL_SIGN_II,
            Telugu::TeluguVowelSignU => TELUGU_VOWEL_SIGN_U,
            Telugu::TeluguVowelSignUu => TELUGU_VOWEL_SIGN_UU,
            Telugu::TeluguVowelSignVocalicR => TELUGU_VOWEL_SIGN_VOCALIC_R,
            Telugu::TeluguVowelSignVocalicRr => TELUGU_VOWEL_SIGN_VOCALIC_RR,
            Telugu::TeluguVowelSignE => TELUGU_VOWEL_SIGN_E,
            Telugu::TeluguVowelSignEe => TELUGU_VOWEL_SIGN_EE,
            Telugu::TeluguVowelSignAi => TELUGU_VOWEL_SIGN_AI,
            Telugu::TeluguVowelSignO => TELUGU_VOWEL_SIGN_O,
            Telugu::TeluguVowelSignOo => TELUGU_VOWEL_SIGN_OO,
            Telugu::TeluguVowelSignAu => TELUGU_VOWEL_SIGN_AU,
            Telugu::TeluguSignVirama => TELUGU_SIGN_VIRAMA,
            Telugu::TeluguLengthMark => TELUGU_LENGTH_MARK,
            Telugu::TeluguAiLengthMark => TELUGU_AI_LENGTH_MARK,
            Telugu::TeluguLetterTsa => TELUGU_LETTER_TSA,
            Telugu::TeluguLetterDza => TELUGU_LETTER_DZA,
            Telugu::TeluguLetterRrra => TELUGU_LETTER_RRRA,
            Telugu::TeluguLetterVocalicRr => TELUGU_LETTER_VOCALIC_RR,
            Telugu::TeluguLetterVocalicLl => TELUGU_LETTER_VOCALIC_LL,
            Telugu::TeluguVowelSignVocalicL => TELUGU_VOWEL_SIGN_VOCALIC_L,
            Telugu::TeluguVowelSignVocalicLl => TELUGU_VOWEL_SIGN_VOCALIC_LL,
            Telugu::TeluguDigitZero => TELUGU_DIGIT_ZERO,
            Telugu::TeluguDigitOne => TELUGU_DIGIT_ONE,
            Telugu::TeluguDigitTwo => TELUGU_DIGIT_TWO,
            Telugu::TeluguDigitThree => TELUGU_DIGIT_THREE,
            Telugu::TeluguDigitFour => TELUGU_DIGIT_FOUR,
            Telugu::TeluguDigitFive => TELUGU_DIGIT_FIVE,
            Telugu::TeluguDigitSix => TELUGU_DIGIT_SIX,
            Telugu::TeluguDigitSeven => TELUGU_DIGIT_SEVEN,
            Telugu::TeluguDigitEight => TELUGU_DIGIT_EIGHT,
            Telugu::TeluguDigitNine => TELUGU_DIGIT_NINE,
            Telugu::TeluguSignSiddham => TELUGU_SIGN_SIDDHAM,
            Telugu::TeluguFractionDigitZeroForOddPowersOfFour => TELUGU_FRACTION_DIGIT_ZERO_FOR_ODD_POWERS_OF_FOUR,
            Telugu::TeluguFractionDigitOneForOddPowersOfFour => TELUGU_FRACTION_DIGIT_ONE_FOR_ODD_POWERS_OF_FOUR,
            Telugu::TeluguFractionDigitTwoForOddPowersOfFour => TELUGU_FRACTION_DIGIT_TWO_FOR_ODD_POWERS_OF_FOUR,
            Telugu::TeluguFractionDigitThreeForOddPowersOfFour => TELUGU_FRACTION_DIGIT_THREE_FOR_ODD_POWERS_OF_FOUR,
            Telugu::TeluguFractionDigitOneForEvenPowersOfFour => TELUGU_FRACTION_DIGIT_ONE_FOR_EVEN_POWERS_OF_FOUR,
            Telugu::TeluguFractionDigitTwoForEvenPowersOfFour => TELUGU_FRACTION_DIGIT_TWO_FOR_EVEN_POWERS_OF_FOUR,
            Telugu::TeluguFractionDigitThreeForEvenPowersOfFour => TELUGU_FRACTION_DIGIT_THREE_FOR_EVEN_POWERS_OF_FOUR,
        }
    }
}

impl std::convert::TryFrom<char> for Telugu {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            TELUGU_SIGN_COMBINING_CANDRABINDU_ABOVE => Ok(Telugu::TeluguSignCombiningCandrabinduAbove),
            TELUGU_SIGN_CANDRABINDU => Ok(Telugu::TeluguSignCandrabindu),
            TELUGU_SIGN_ANUSVARA => Ok(Telugu::TeluguSignAnusvara),
            TELUGU_SIGN_VISARGA => Ok(Telugu::TeluguSignVisarga),
            TELUGU_SIGN_COMBINING_ANUSVARA_ABOVE => Ok(Telugu::TeluguSignCombiningAnusvaraAbove),
            TELUGU_LETTER_A => Ok(Telugu::TeluguLetterA),
            TELUGU_LETTER_AA => Ok(Telugu::TeluguLetterAa),
            TELUGU_LETTER_I => Ok(Telugu::TeluguLetterI),
            TELUGU_LETTER_II => Ok(Telugu::TeluguLetterIi),
            TELUGU_LETTER_U => Ok(Telugu::TeluguLetterU),
            TELUGU_LETTER_UU => Ok(Telugu::TeluguLetterUu),
            TELUGU_LETTER_VOCALIC_R => Ok(Telugu::TeluguLetterVocalicR),
            TELUGU_LETTER_VOCALIC_L => Ok(Telugu::TeluguLetterVocalicL),
            TELUGU_LETTER_E => Ok(Telugu::TeluguLetterE),
            TELUGU_LETTER_EE => Ok(Telugu::TeluguLetterEe),
            TELUGU_LETTER_AI => Ok(Telugu::TeluguLetterAi),
            TELUGU_LETTER_O => Ok(Telugu::TeluguLetterO),
            TELUGU_LETTER_OO => Ok(Telugu::TeluguLetterOo),
            TELUGU_LETTER_AU => Ok(Telugu::TeluguLetterAu),
            TELUGU_LETTER_KA => Ok(Telugu::TeluguLetterKa),
            TELUGU_LETTER_KHA => Ok(Telugu::TeluguLetterKha),
            TELUGU_LETTER_GA => Ok(Telugu::TeluguLetterGa),
            TELUGU_LETTER_GHA => Ok(Telugu::TeluguLetterGha),
            TELUGU_LETTER_NGA => Ok(Telugu::TeluguLetterNga),
            TELUGU_LETTER_CA => Ok(Telugu::TeluguLetterCa),
            TELUGU_LETTER_CHA => Ok(Telugu::TeluguLetterCha),
            TELUGU_LETTER_JA => Ok(Telugu::TeluguLetterJa),
            TELUGU_LETTER_JHA => Ok(Telugu::TeluguLetterJha),
            TELUGU_LETTER_NYA => Ok(Telugu::TeluguLetterNya),
            TELUGU_LETTER_TTA => Ok(Telugu::TeluguLetterTta),
            TELUGU_LETTER_TTHA => Ok(Telugu::TeluguLetterTtha),
            TELUGU_LETTER_DDA => Ok(Telugu::TeluguLetterDda),
            TELUGU_LETTER_DDHA => Ok(Telugu::TeluguLetterDdha),
            TELUGU_LETTER_NNA => Ok(Telugu::TeluguLetterNna),
            TELUGU_LETTER_TA => Ok(Telugu::TeluguLetterTa),
            TELUGU_LETTER_THA => Ok(Telugu::TeluguLetterTha),
            TELUGU_LETTER_DA => Ok(Telugu::TeluguLetterDa),
            TELUGU_LETTER_DHA => Ok(Telugu::TeluguLetterDha),
            TELUGU_LETTER_NA => Ok(Telugu::TeluguLetterNa),
            TELUGU_LETTER_PA => Ok(Telugu::TeluguLetterPa),
            TELUGU_LETTER_PHA => Ok(Telugu::TeluguLetterPha),
            TELUGU_LETTER_BA => Ok(Telugu::TeluguLetterBa),
            TELUGU_LETTER_BHA => Ok(Telugu::TeluguLetterBha),
            TELUGU_LETTER_MA => Ok(Telugu::TeluguLetterMa),
            TELUGU_LETTER_YA => Ok(Telugu::TeluguLetterYa),
            TELUGU_LETTER_RA => Ok(Telugu::TeluguLetterRa),
            TELUGU_LETTER_RRA => Ok(Telugu::TeluguLetterRra),
            TELUGU_LETTER_LA => Ok(Telugu::TeluguLetterLa),
            TELUGU_LETTER_LLA => Ok(Telugu::TeluguLetterLla),
            TELUGU_LETTER_LLLA => Ok(Telugu::TeluguLetterLlla),
            TELUGU_LETTER_VA => Ok(Telugu::TeluguLetterVa),
            TELUGU_LETTER_SHA => Ok(Telugu::TeluguLetterSha),
            TELUGU_LETTER_SSA => Ok(Telugu::TeluguLetterSsa),
            TELUGU_LETTER_SA => Ok(Telugu::TeluguLetterSa),
            TELUGU_LETTER_HA => Ok(Telugu::TeluguLetterHa),
            TELUGU_SIGN_AVAGRAHA => Ok(Telugu::TeluguSignAvagraha),
            TELUGU_VOWEL_SIGN_AA => Ok(Telugu::TeluguVowelSignAa),
            TELUGU_VOWEL_SIGN_I => Ok(Telugu::TeluguVowelSignI),
            TELUGU_VOWEL_SIGN_II => Ok(Telugu::TeluguVowelSignIi),
            TELUGU_VOWEL_SIGN_U => Ok(Telugu::TeluguVowelSignU),
            TELUGU_VOWEL_SIGN_UU => Ok(Telugu::TeluguVowelSignUu),
            TELUGU_VOWEL_SIGN_VOCALIC_R => Ok(Telugu::TeluguVowelSignVocalicR),
            TELUGU_VOWEL_SIGN_VOCALIC_RR => Ok(Telugu::TeluguVowelSignVocalicRr),
            TELUGU_VOWEL_SIGN_E => Ok(Telugu::TeluguVowelSignE),
            TELUGU_VOWEL_SIGN_EE => Ok(Telugu::TeluguVowelSignEe),
            TELUGU_VOWEL_SIGN_AI => Ok(Telugu::TeluguVowelSignAi),
            TELUGU_VOWEL_SIGN_O => Ok(Telugu::TeluguVowelSignO),
            TELUGU_VOWEL_SIGN_OO => Ok(Telugu::TeluguVowelSignOo),
            TELUGU_VOWEL_SIGN_AU => Ok(Telugu::TeluguVowelSignAu),
            TELUGU_SIGN_VIRAMA => Ok(Telugu::TeluguSignVirama),
            TELUGU_LENGTH_MARK => Ok(Telugu::TeluguLengthMark),
            TELUGU_AI_LENGTH_MARK => Ok(Telugu::TeluguAiLengthMark),
            TELUGU_LETTER_TSA => Ok(Telugu::TeluguLetterTsa),
            TELUGU_LETTER_DZA => Ok(Telugu::TeluguLetterDza),
            TELUGU_LETTER_RRRA => Ok(Telugu::TeluguLetterRrra),
            TELUGU_LETTER_VOCALIC_RR => Ok(Telugu::TeluguLetterVocalicRr),
            TELUGU_LETTER_VOCALIC_LL => Ok(Telugu::TeluguLetterVocalicLl),
            TELUGU_VOWEL_SIGN_VOCALIC_L => Ok(Telugu::TeluguVowelSignVocalicL),
            TELUGU_VOWEL_SIGN_VOCALIC_LL => Ok(Telugu::TeluguVowelSignVocalicLl),
            TELUGU_DIGIT_ZERO => Ok(Telugu::TeluguDigitZero),
            TELUGU_DIGIT_ONE => Ok(Telugu::TeluguDigitOne),
            TELUGU_DIGIT_TWO => Ok(Telugu::TeluguDigitTwo),
            TELUGU_DIGIT_THREE => Ok(Telugu::TeluguDigitThree),
            TELUGU_DIGIT_FOUR => Ok(Telugu::TeluguDigitFour),
            TELUGU_DIGIT_FIVE => Ok(Telugu::TeluguDigitFive),
            TELUGU_DIGIT_SIX => Ok(Telugu::TeluguDigitSix),
            TELUGU_DIGIT_SEVEN => Ok(Telugu::TeluguDigitSeven),
            TELUGU_DIGIT_EIGHT => Ok(Telugu::TeluguDigitEight),
            TELUGU_DIGIT_NINE => Ok(Telugu::TeluguDigitNine),
            TELUGU_SIGN_SIDDHAM => Ok(Telugu::TeluguSignSiddham),
            TELUGU_FRACTION_DIGIT_ZERO_FOR_ODD_POWERS_OF_FOUR => Ok(Telugu::TeluguFractionDigitZeroForOddPowersOfFour),
            TELUGU_FRACTION_DIGIT_ONE_FOR_ODD_POWERS_OF_FOUR => Ok(Telugu::TeluguFractionDigitOneForOddPowersOfFour),
            TELUGU_FRACTION_DIGIT_TWO_FOR_ODD_POWERS_OF_FOUR => Ok(Telugu::TeluguFractionDigitTwoForOddPowersOfFour),
            TELUGU_FRACTION_DIGIT_THREE_FOR_ODD_POWERS_OF_FOUR => Ok(Telugu::TeluguFractionDigitThreeForOddPowersOfFour),
            TELUGU_FRACTION_DIGIT_ONE_FOR_EVEN_POWERS_OF_FOUR => Ok(Telugu::TeluguFractionDigitOneForEvenPowersOfFour),
            TELUGU_FRACTION_DIGIT_TWO_FOR_EVEN_POWERS_OF_FOUR => Ok(Telugu::TeluguFractionDigitTwoForEvenPowersOfFour),
            TELUGU_FRACTION_DIGIT_THREE_FOR_EVEN_POWERS_OF_FOUR => Ok(Telugu::TeluguFractionDigitThreeForEvenPowersOfFour),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Telugu {
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

impl std::convert::TryFrom<u32> for Telugu {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Telugu {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Telugu {
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        Telugu::TeluguSignCombiningCandrabinduAbove
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Telugu::TeluguSignCombiningCandrabinduAbove => "telugu sign combining candrabindu above",
            Telugu::TeluguSignCandrabindu => "telugu sign candrabindu",
            Telugu::TeluguSignAnusvara => "telugu sign anusvara",
            Telugu::TeluguSignVisarga => "telugu sign visarga",
            Telugu::TeluguSignCombiningAnusvaraAbove => "telugu sign combining anusvara above",
            Telugu::TeluguLetterA => "telugu letter a",
            Telugu::TeluguLetterAa => "telugu letter aa",
            Telugu::TeluguLetterI => "telugu letter i",
            Telugu::TeluguLetterIi => "telugu letter ii",
            Telugu::TeluguLetterU => "telugu letter u",
            Telugu::TeluguLetterUu => "telugu letter uu",
            Telugu::TeluguLetterVocalicR => "telugu letter vocalic r",
            Telugu::TeluguLetterVocalicL => "telugu letter vocalic l",
            Telugu::TeluguLetterE => "telugu letter e",
            Telugu::TeluguLetterEe => "telugu letter ee",
            Telugu::TeluguLetterAi => "telugu letter ai",
            Telugu::TeluguLetterO => "telugu letter o",
            Telugu::TeluguLetterOo => "telugu letter oo",
            Telugu::TeluguLetterAu => "telugu letter au",
            Telugu::TeluguLetterKa => "telugu letter ka",
            Telugu::TeluguLetterKha => "telugu letter kha",
            Telugu::TeluguLetterGa => "telugu letter ga",
            Telugu::TeluguLetterGha => "telugu letter gha",
            Telugu::TeluguLetterNga => "telugu letter nga",
            Telugu::TeluguLetterCa => "telugu letter ca",
            Telugu::TeluguLetterCha => "telugu letter cha",
            Telugu::TeluguLetterJa => "telugu letter ja",
            Telugu::TeluguLetterJha => "telugu letter jha",
            Telugu::TeluguLetterNya => "telugu letter nya",
            Telugu::TeluguLetterTta => "telugu letter tta",
            Telugu::TeluguLetterTtha => "telugu letter ttha",
            Telugu::TeluguLetterDda => "telugu letter dda",
            Telugu::TeluguLetterDdha => "telugu letter ddha",
            Telugu::TeluguLetterNna => "telugu letter nna",
            Telugu::TeluguLetterTa => "telugu letter ta",
            Telugu::TeluguLetterTha => "telugu letter tha",
            Telugu::TeluguLetterDa => "telugu letter da",
            Telugu::TeluguLetterDha => "telugu letter dha",
            Telugu::TeluguLetterNa => "telugu letter na",
            Telugu::TeluguLetterPa => "telugu letter pa",
            Telugu::TeluguLetterPha => "telugu letter pha",
            Telugu::TeluguLetterBa => "telugu letter ba",
            Telugu::TeluguLetterBha => "telugu letter bha",
            Telugu::TeluguLetterMa => "telugu letter ma",
            Telugu::TeluguLetterYa => "telugu letter ya",
            Telugu::TeluguLetterRa => "telugu letter ra",
            Telugu::TeluguLetterRra => "telugu letter rra",
            Telugu::TeluguLetterLa => "telugu letter la",
            Telugu::TeluguLetterLla => "telugu letter lla",
            Telugu::TeluguLetterLlla => "telugu letter llla",
            Telugu::TeluguLetterVa => "telugu letter va",
            Telugu::TeluguLetterSha => "telugu letter sha",
            Telugu::TeluguLetterSsa => "telugu letter ssa",
            Telugu::TeluguLetterSa => "telugu letter sa",
            Telugu::TeluguLetterHa => "telugu letter ha",
            Telugu::TeluguSignAvagraha => "telugu sign avagraha",
            Telugu::TeluguVowelSignAa => "telugu vowel sign aa",
            Telugu::TeluguVowelSignI => "telugu vowel sign i",
            Telugu::TeluguVowelSignIi => "telugu vowel sign ii",
            Telugu::TeluguVowelSignU => "telugu vowel sign u",
            Telugu::TeluguVowelSignUu => "telugu vowel sign uu",
            Telugu::TeluguVowelSignVocalicR => "telugu vowel sign vocalic r",
            Telugu::TeluguVowelSignVocalicRr => "telugu vowel sign vocalic rr",
            Telugu::TeluguVowelSignE => "telugu vowel sign e",
            Telugu::TeluguVowelSignEe => "telugu vowel sign ee",
            Telugu::TeluguVowelSignAi => "telugu vowel sign ai",
            Telugu::TeluguVowelSignO => "telugu vowel sign o",
            Telugu::TeluguVowelSignOo => "telugu vowel sign oo",
            Telugu::TeluguVowelSignAu => "telugu vowel sign au",
            Telugu::TeluguSignVirama => "telugu sign virama",
            Telugu::TeluguLengthMark => "telugu length mark",
            Telugu::TeluguAiLengthMark => "telugu ai length mark",
            Telugu::TeluguLetterTsa => "telugu letter tsa",
            Telugu::TeluguLetterDza => "telugu letter dza",
            Telugu::TeluguLetterRrra => "telugu letter rrra",
            Telugu::TeluguLetterVocalicRr => "telugu letter vocalic rr",
            Telugu::TeluguLetterVocalicLl => "telugu letter vocalic ll",
            Telugu::TeluguVowelSignVocalicL => "telugu vowel sign vocalic l",
            Telugu::TeluguVowelSignVocalicLl => "telugu vowel sign vocalic ll",
            Telugu::TeluguDigitZero => "telugu digit zero",
            Telugu::TeluguDigitOne => "telugu digit one",
            Telugu::TeluguDigitTwo => "telugu digit two",
            Telugu::TeluguDigitThree => "telugu digit three",
            Telugu::TeluguDigitFour => "telugu digit four",
            Telugu::TeluguDigitFive => "telugu digit five",
            Telugu::TeluguDigitSix => "telugu digit six",
            Telugu::TeluguDigitSeven => "telugu digit seven",
            Telugu::TeluguDigitEight => "telugu digit eight",
            Telugu::TeluguDigitNine => "telugu digit nine",
            Telugu::TeluguSignSiddham => "telugu sign siddham",
            Telugu::TeluguFractionDigitZeroForOddPowersOfFour => "telugu fraction digit zero for odd powers of four",
            Telugu::TeluguFractionDigitOneForOddPowersOfFour => "telugu fraction digit one for odd powers of four",
            Telugu::TeluguFractionDigitTwoForOddPowersOfFour => "telugu fraction digit two for odd powers of four",
            Telugu::TeluguFractionDigitThreeForOddPowersOfFour => "telugu fraction digit three for odd powers of four",
            Telugu::TeluguFractionDigitOneForEvenPowersOfFour => "telugu fraction digit one for even powers of four",
            Telugu::TeluguFractionDigitTwoForEvenPowersOfFour => "telugu fraction digit two for even powers of four",
            Telugu::TeluguFractionDigitThreeForEvenPowersOfFour => "telugu fraction digit three for even powers of four",
        }
    }
}
