/// \u{d00} → \u{d7f}\
///\
/// ഀ ഁ ം ഃ അ ആ ഇ ഈ ഉ ഊ ഋ ഌ എ ഏ ഐ ഒ
/// ഓ ഔ ക ഖ ഗ ഘ ങ ച ഛ ജ ഝ ഞ ട ഠ ഡ ഢ
/// ണ ത ഥ ദ ധ ന ഩ പ ഫ ബ ഭ മ യ ര റ ല
/// ള ഴ വ ശ ഷ സ ഹ ഺ ഻ ഼ ഽ ാ ി ീ ു ൂ
/// ൃ ൄ െ േ ൈ ൊ ോ ൌ ് ൎ ൏ ൔ ൕ ൖ ൗ ൘
/// ൙ ൚ ൛ ൜ ൝ ൞ ൟ ൠ ൡ ൢ ൣ ൦ ൧ ൨ ൩ ൪
/// ൫ ൬ ൭ ൮ ൯ ൰ ൱ ൲ ൳ ൴ ൵ ൶ ൷ ൸ ൹ ൺ
/// ൻ ർ ൽ ൾ
pub mod constants {
    /// \u{d00}: 'ഀ'
    pub const MALAYALAM_SIGN_COMBINING_ANUSVARA_ABOVE: char = 'ഀ';
    /// \u{d01}: 'ഁ'
    pub const MALAYALAM_SIGN_CANDRABINDU: char = 'ഁ';
    /// \u{d02}: 'ം'
    pub const MALAYALAM_SIGN_ANUSVARA: char = 'ം';
    /// \u{d03}: 'ഃ'
    pub const MALAYALAM_SIGN_VISARGA: char = 'ഃ';
    /// \u{d05}: 'അ'
    pub const MALAYALAM_LETTER_A: char = 'അ';
    /// \u{d06}: 'ആ'
    pub const MALAYALAM_LETTER_AA: char = 'ആ';
    /// \u{d07}: 'ഇ'
    pub const MALAYALAM_LETTER_I: char = 'ഇ';
    /// \u{d08}: 'ഈ'
    pub const MALAYALAM_LETTER_II: char = 'ഈ';
    /// \u{d09}: 'ഉ'
    pub const MALAYALAM_LETTER_U: char = 'ഉ';
    /// \u{d0a}: 'ഊ'
    pub const MALAYALAM_LETTER_UU: char = 'ഊ';
    /// \u{d0b}: 'ഋ'
    pub const MALAYALAM_LETTER_VOCALIC_R: char = 'ഋ';
    /// \u{d0c}: 'ഌ'
    pub const MALAYALAM_LETTER_VOCALIC_L: char = 'ഌ';
    /// \u{d0e}: 'എ'
    pub const MALAYALAM_LETTER_E: char = 'എ';
    /// \u{d0f}: 'ഏ'
    pub const MALAYALAM_LETTER_EE: char = 'ഏ';
    /// \u{d10}: 'ഐ'
    pub const MALAYALAM_LETTER_AI: char = 'ഐ';
    /// \u{d12}: 'ഒ'
    pub const MALAYALAM_LETTER_O: char = 'ഒ';
    /// \u{d13}: 'ഓ'
    pub const MALAYALAM_LETTER_OO: char = 'ഓ';
    /// \u{d14}: 'ഔ'
    pub const MALAYALAM_LETTER_AU: char = 'ഔ';
    /// \u{d15}: 'ക'
    pub const MALAYALAM_LETTER_KA: char = 'ക';
    /// \u{d16}: 'ഖ'
    pub const MALAYALAM_LETTER_KHA: char = 'ഖ';
    /// \u{d17}: 'ഗ'
    pub const MALAYALAM_LETTER_GA: char = 'ഗ';
    /// \u{d18}: 'ഘ'
    pub const MALAYALAM_LETTER_GHA: char = 'ഘ';
    /// \u{d19}: 'ങ'
    pub const MALAYALAM_LETTER_NGA: char = 'ങ';
    /// \u{d1a}: 'ച'
    pub const MALAYALAM_LETTER_CA: char = 'ച';
    /// \u{d1b}: 'ഛ'
    pub const MALAYALAM_LETTER_CHA: char = 'ഛ';
    /// \u{d1c}: 'ജ'
    pub const MALAYALAM_LETTER_JA: char = 'ജ';
    /// \u{d1d}: 'ഝ'
    pub const MALAYALAM_LETTER_JHA: char = 'ഝ';
    /// \u{d1e}: 'ഞ'
    pub const MALAYALAM_LETTER_NYA: char = 'ഞ';
    /// \u{d1f}: 'ട'
    pub const MALAYALAM_LETTER_TTA: char = 'ട';
    /// \u{d20}: 'ഠ'
    pub const MALAYALAM_LETTER_TTHA: char = 'ഠ';
    /// \u{d21}: 'ഡ'
    pub const MALAYALAM_LETTER_DDA: char = 'ഡ';
    /// \u{d22}: 'ഢ'
    pub const MALAYALAM_LETTER_DDHA: char = 'ഢ';
    /// \u{d23}: 'ണ'
    pub const MALAYALAM_LETTER_NNA: char = 'ണ';
    /// \u{d24}: 'ത'
    pub const MALAYALAM_LETTER_TA: char = 'ത';
    /// \u{d25}: 'ഥ'
    pub const MALAYALAM_LETTER_THA: char = 'ഥ';
    /// \u{d26}: 'ദ'
    pub const MALAYALAM_LETTER_DA: char = 'ദ';
    /// \u{d27}: 'ധ'
    pub const MALAYALAM_LETTER_DHA: char = 'ധ';
    /// \u{d28}: 'ന'
    pub const MALAYALAM_LETTER_NA: char = 'ന';
    /// \u{d29}: 'ഩ'
    pub const MALAYALAM_LETTER_NNNA: char = 'ഩ';
    /// \u{d2a}: 'പ'
    pub const MALAYALAM_LETTER_PA: char = 'പ';
    /// \u{d2b}: 'ഫ'
    pub const MALAYALAM_LETTER_PHA: char = 'ഫ';
    /// \u{d2c}: 'ബ'
    pub const MALAYALAM_LETTER_BA: char = 'ബ';
    /// \u{d2d}: 'ഭ'
    pub const MALAYALAM_LETTER_BHA: char = 'ഭ';
    /// \u{d2e}: 'മ'
    pub const MALAYALAM_LETTER_MA: char = 'മ';
    /// \u{d2f}: 'യ'
    pub const MALAYALAM_LETTER_YA: char = 'യ';
    /// \u{d30}: 'ര'
    pub const MALAYALAM_LETTER_RA: char = 'ര';
    /// \u{d31}: 'റ'
    pub const MALAYALAM_LETTER_RRA: char = 'റ';
    /// \u{d32}: 'ല'
    pub const MALAYALAM_LETTER_LA: char = 'ല';
    /// \u{d33}: 'ള'
    pub const MALAYALAM_LETTER_LLA: char = 'ള';
    /// \u{d34}: 'ഴ'
    pub const MALAYALAM_LETTER_LLLA: char = 'ഴ';
    /// \u{d35}: 'വ'
    pub const MALAYALAM_LETTER_VA: char = 'വ';
    /// \u{d36}: 'ശ'
    pub const MALAYALAM_LETTER_SHA: char = 'ശ';
    /// \u{d37}: 'ഷ'
    pub const MALAYALAM_LETTER_SSA: char = 'ഷ';
    /// \u{d38}: 'സ'
    pub const MALAYALAM_LETTER_SA: char = 'സ';
    /// \u{d39}: 'ഹ'
    pub const MALAYALAM_LETTER_HA: char = 'ഹ';
    /// \u{d3a}: 'ഺ'
    pub const MALAYALAM_LETTER_TTTA: char = 'ഺ';
    /// \u{d3b}: '഻'
    pub const MALAYALAM_SIGN_VERTICAL_BAR_VIRAMA: char = '഻';
    /// \u{d3c}: '഼'
    pub const MALAYALAM_SIGN_CIRCULAR_VIRAMA: char = '഼';
    /// \u{d3d}: 'ഽ'
    pub const MALAYALAM_SIGN_AVAGRAHA: char = 'ഽ';
    /// \u{d3e}: 'ാ'
    pub const MALAYALAM_VOWEL_SIGN_AA: char = 'ാ';
    /// \u{d3f}: 'ി'
    pub const MALAYALAM_VOWEL_SIGN_I: char = 'ി';
    /// \u{d40}: 'ീ'
    pub const MALAYALAM_VOWEL_SIGN_II: char = 'ീ';
    /// \u{d41}: 'ു'
    pub const MALAYALAM_VOWEL_SIGN_U: char = 'ു';
    /// \u{d42}: 'ൂ'
    pub const MALAYALAM_VOWEL_SIGN_UU: char = 'ൂ';
    /// \u{d43}: 'ൃ'
    pub const MALAYALAM_VOWEL_SIGN_VOCALIC_R: char = 'ൃ';
    /// \u{d44}: 'ൄ'
    pub const MALAYALAM_VOWEL_SIGN_VOCALIC_RR: char = 'ൄ';
    /// \u{d46}: 'െ'
    pub const MALAYALAM_VOWEL_SIGN_E: char = 'െ';
    /// \u{d47}: 'േ'
    pub const MALAYALAM_VOWEL_SIGN_EE: char = 'േ';
    /// \u{d48}: 'ൈ'
    pub const MALAYALAM_VOWEL_SIGN_AI: char = 'ൈ';
    /// \u{d4a}: 'ൊ'
    pub const MALAYALAM_VOWEL_SIGN_O: char = 'ൊ';
    /// \u{d4b}: 'ോ'
    pub const MALAYALAM_VOWEL_SIGN_OO: char = 'ോ';
    /// \u{d4c}: 'ൌ'
    pub const MALAYALAM_VOWEL_SIGN_AU: char = 'ൌ';
    /// \u{d4d}: '്'
    pub const MALAYALAM_SIGN_VIRAMA: char = '്';
    /// \u{d4e}: 'ൎ'
    pub const MALAYALAM_LETTER_DOT_REPH: char = 'ൎ';
    /// \u{d4f}: '൏'
    pub const MALAYALAM_SIGN_PARA: char = '൏';
    /// \u{d54}: 'ൔ'
    pub const MALAYALAM_LETTER_CHILLU_M: char = 'ൔ';
    /// \u{d55}: 'ൕ'
    pub const MALAYALAM_LETTER_CHILLU_Y: char = 'ൕ';
    /// \u{d56}: 'ൖ'
    pub const MALAYALAM_LETTER_CHILLU_LLL: char = 'ൖ';
    /// \u{d57}: 'ൗ'
    pub const MALAYALAM_AU_LENGTH_MARK: char = 'ൗ';
    /// \u{d58}: '൘'
    pub const MALAYALAM_FRACTION_ONE_ONE_DASH_HUNDRED_DASH_AND_DASH_SIXTIETH: char = '൘';
    /// \u{d59}: '൙'
    pub const MALAYALAM_FRACTION_ONE_FORTIETH: char = '൙';
    /// \u{d5a}: '൚'
    pub const MALAYALAM_FRACTION_THREE_EIGHTIETHS: char = '൚';
    /// \u{d5b}: '൛'
    pub const MALAYALAM_FRACTION_ONE_TWENTIETH: char = '൛';
    /// \u{d5c}: '൜'
    pub const MALAYALAM_FRACTION_ONE_TENTH: char = '൜';
    /// \u{d5d}: '൝'
    pub const MALAYALAM_FRACTION_THREE_TWENTIETHS: char = '൝';
    /// \u{d5e}: '൞'
    pub const MALAYALAM_FRACTION_ONE_FIFTH: char = '൞';
    /// \u{d5f}: 'ൟ'
    pub const MALAYALAM_LETTER_ARCHAIC_II: char = 'ൟ';
    /// \u{d60}: 'ൠ'
    pub const MALAYALAM_LETTER_VOCALIC_RR: char = 'ൠ';
    /// \u{d61}: 'ൡ'
    pub const MALAYALAM_LETTER_VOCALIC_LL: char = 'ൡ';
    /// \u{d62}: 'ൢ'
    pub const MALAYALAM_VOWEL_SIGN_VOCALIC_L: char = 'ൢ';
    /// \u{d63}: 'ൣ'
    pub const MALAYALAM_VOWEL_SIGN_VOCALIC_LL: char = 'ൣ';
    /// \u{d66}: '൦'
    pub const MALAYALAM_DIGIT_ZERO: char = '൦';
    /// \u{d67}: '൧'
    pub const MALAYALAM_DIGIT_ONE: char = '൧';
    /// \u{d68}: '൨'
    pub const MALAYALAM_DIGIT_TWO: char = '൨';
    /// \u{d69}: '൩'
    pub const MALAYALAM_DIGIT_THREE: char = '൩';
    /// \u{d6a}: '൪'
    pub const MALAYALAM_DIGIT_FOUR: char = '൪';
    /// \u{d6b}: '൫'
    pub const MALAYALAM_DIGIT_FIVE: char = '൫';
    /// \u{d6c}: '൬'
    pub const MALAYALAM_DIGIT_SIX: char = '൬';
    /// \u{d6d}: '൭'
    pub const MALAYALAM_DIGIT_SEVEN: char = '൭';
    /// \u{d6e}: '൮'
    pub const MALAYALAM_DIGIT_EIGHT: char = '൮';
    /// \u{d6f}: '൯'
    pub const MALAYALAM_DIGIT_NINE: char = '൯';
    /// \u{d70}: '൰'
    pub const MALAYALAM_NUMBER_TEN: char = '൰';
    /// \u{d71}: '൱'
    pub const MALAYALAM_NUMBER_ONE_HUNDRED: char = '൱';
    /// \u{d72}: '൲'
    pub const MALAYALAM_NUMBER_ONE_THOUSAND: char = '൲';
    /// \u{d73}: '൳'
    pub const MALAYALAM_FRACTION_ONE_QUARTER: char = '൳';
    /// \u{d74}: '൴'
    pub const MALAYALAM_FRACTION_ONE_HALF: char = '൴';
    /// \u{d75}: '൵'
    pub const MALAYALAM_FRACTION_THREE_QUARTERS: char = '൵';
    /// \u{d76}: '൶'
    pub const MALAYALAM_FRACTION_ONE_SIXTEENTH: char = '൶';
    /// \u{d77}: '൷'
    pub const MALAYALAM_FRACTION_ONE_EIGHTH: char = '൷';
    /// \u{d78}: '൸'
    pub const MALAYALAM_FRACTION_THREE_SIXTEENTHS: char = '൸';
    /// \u{d79}: '൹'
    pub const MALAYALAM_DATE_MARK: char = '൹';
    /// \u{d7a}: 'ൺ'
    pub const MALAYALAM_LETTER_CHILLU_NN: char = 'ൺ';
    /// \u{d7b}: 'ൻ'
    pub const MALAYALAM_LETTER_CHILLU_N: char = 'ൻ';
    /// \u{d7c}: 'ർ'
    pub const MALAYALAM_LETTER_CHILLU_RR: char = 'ർ';
    /// \u{d7d}: 'ൽ'
    pub const MALAYALAM_LETTER_CHILLU_L: char = 'ൽ';
    /// \u{d7e}: 'ൾ'
    pub const MALAYALAM_LETTER_CHILLU_LL: char = 'ൾ';
}

/// \u{d00} → \u{d7f}\
///\
/// ഀ ഁ ം ഃ അ ആ ഇ ഈ ഉ ഊ ഋ ഌ എ ഏ ഐ ഒ
/// ഓ ഔ ക ഖ ഗ ഘ ങ ച ഛ ജ ഝ ഞ ട ഠ ഡ ഢ
/// ണ ത ഥ ദ ധ ന ഩ പ ഫ ബ ഭ മ യ ര റ ല
/// ള ഴ വ ശ ഷ സ ഹ ഺ ഻ ഼ ഽ ാ ി ീ ു ൂ
/// ൃ ൄ െ േ ൈ ൊ ോ ൌ ് ൎ ൏ ൔ ൕ ൖ ൗ ൘
/// ൙ ൚ ൛ ൜ ൝ ൞ ൟ ൠ ൡ ൢ ൣ ൦ ൧ ൨ ൩ ൪
/// ൫ ൬ ൭ ൮ ൯ ൰ ൱ ൲ ൳ ൴ ൵ ൶ ൷ ൸ ൹ ൺ
/// ൻ ർ ൽ ൾ
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Malayalam {
    /// \u{d00}: 'ഀ'
    MalayalamSignCombiningAnusvaraAbove,
    /// \u{d01}: 'ഁ'
    MalayalamSignCandrabindu,
    /// \u{d02}: 'ം'
    MalayalamSignAnusvara,
    /// \u{d03}: 'ഃ'
    MalayalamSignVisarga,
    /// \u{d05}: 'അ'
    MalayalamLetterA,
    /// \u{d06}: 'ആ'
    MalayalamLetterAa,
    /// \u{d07}: 'ഇ'
    MalayalamLetterI,
    /// \u{d08}: 'ഈ'
    MalayalamLetterIi,
    /// \u{d09}: 'ഉ'
    MalayalamLetterU,
    /// \u{d0a}: 'ഊ'
    MalayalamLetterUu,
    /// \u{d0b}: 'ഋ'
    MalayalamLetterVocalicR,
    /// \u{d0c}: 'ഌ'
    MalayalamLetterVocalicL,
    /// \u{d0e}: 'എ'
    MalayalamLetterE,
    /// \u{d0f}: 'ഏ'
    MalayalamLetterEe,
    /// \u{d10}: 'ഐ'
    MalayalamLetterAi,
    /// \u{d12}: 'ഒ'
    MalayalamLetterO,
    /// \u{d13}: 'ഓ'
    MalayalamLetterOo,
    /// \u{d14}: 'ഔ'
    MalayalamLetterAu,
    /// \u{d15}: 'ക'
    MalayalamLetterKa,
    /// \u{d16}: 'ഖ'
    MalayalamLetterKha,
    /// \u{d17}: 'ഗ'
    MalayalamLetterGa,
    /// \u{d18}: 'ഘ'
    MalayalamLetterGha,
    /// \u{d19}: 'ങ'
    MalayalamLetterNga,
    /// \u{d1a}: 'ച'
    MalayalamLetterCa,
    /// \u{d1b}: 'ഛ'
    MalayalamLetterCha,
    /// \u{d1c}: 'ജ'
    MalayalamLetterJa,
    /// \u{d1d}: 'ഝ'
    MalayalamLetterJha,
    /// \u{d1e}: 'ഞ'
    MalayalamLetterNya,
    /// \u{d1f}: 'ട'
    MalayalamLetterTta,
    /// \u{d20}: 'ഠ'
    MalayalamLetterTtha,
    /// \u{d21}: 'ഡ'
    MalayalamLetterDda,
    /// \u{d22}: 'ഢ'
    MalayalamLetterDdha,
    /// \u{d23}: 'ണ'
    MalayalamLetterNna,
    /// \u{d24}: 'ത'
    MalayalamLetterTa,
    /// \u{d25}: 'ഥ'
    MalayalamLetterTha,
    /// \u{d26}: 'ദ'
    MalayalamLetterDa,
    /// \u{d27}: 'ധ'
    MalayalamLetterDha,
    /// \u{d28}: 'ന'
    MalayalamLetterNa,
    /// \u{d29}: 'ഩ'
    MalayalamLetterNnna,
    /// \u{d2a}: 'പ'
    MalayalamLetterPa,
    /// \u{d2b}: 'ഫ'
    MalayalamLetterPha,
    /// \u{d2c}: 'ബ'
    MalayalamLetterBa,
    /// \u{d2d}: 'ഭ'
    MalayalamLetterBha,
    /// \u{d2e}: 'മ'
    MalayalamLetterMa,
    /// \u{d2f}: 'യ'
    MalayalamLetterYa,
    /// \u{d30}: 'ര'
    MalayalamLetterRa,
    /// \u{d31}: 'റ'
    MalayalamLetterRra,
    /// \u{d32}: 'ല'
    MalayalamLetterLa,
    /// \u{d33}: 'ള'
    MalayalamLetterLla,
    /// \u{d34}: 'ഴ'
    MalayalamLetterLlla,
    /// \u{d35}: 'വ'
    MalayalamLetterVa,
    /// \u{d36}: 'ശ'
    MalayalamLetterSha,
    /// \u{d37}: 'ഷ'
    MalayalamLetterSsa,
    /// \u{d38}: 'സ'
    MalayalamLetterSa,
    /// \u{d39}: 'ഹ'
    MalayalamLetterHa,
    /// \u{d3a}: 'ഺ'
    MalayalamLetterTtta,
    /// \u{d3b}: '഻'
    MalayalamSignVerticalBarVirama,
    /// \u{d3c}: '഼'
    MalayalamSignCircularVirama,
    /// \u{d3d}: 'ഽ'
    MalayalamSignAvagraha,
    /// \u{d3e}: 'ാ'
    MalayalamVowelSignAa,
    /// \u{d3f}: 'ി'
    MalayalamVowelSignI,
    /// \u{d40}: 'ീ'
    MalayalamVowelSignIi,
    /// \u{d41}: 'ു'
    MalayalamVowelSignU,
    /// \u{d42}: 'ൂ'
    MalayalamVowelSignUu,
    /// \u{d43}: 'ൃ'
    MalayalamVowelSignVocalicR,
    /// \u{d44}: 'ൄ'
    MalayalamVowelSignVocalicRr,
    /// \u{d46}: 'െ'
    MalayalamVowelSignE,
    /// \u{d47}: 'േ'
    MalayalamVowelSignEe,
    /// \u{d48}: 'ൈ'
    MalayalamVowelSignAi,
    /// \u{d4a}: 'ൊ'
    MalayalamVowelSignO,
    /// \u{d4b}: 'ോ'
    MalayalamVowelSignOo,
    /// \u{d4c}: 'ൌ'
    MalayalamVowelSignAu,
    /// \u{d4d}: '്'
    MalayalamSignVirama,
    /// \u{d4e}: 'ൎ'
    MalayalamLetterDotReph,
    /// \u{d4f}: '൏'
    MalayalamSignPara,
    /// \u{d54}: 'ൔ'
    MalayalamLetterChilluM,
    /// \u{d55}: 'ൕ'
    MalayalamLetterChilluY,
    /// \u{d56}: 'ൖ'
    MalayalamLetterChilluLll,
    /// \u{d57}: 'ൗ'
    MalayalamAuLengthMark,
    /// \u{d58}: '൘'
    MalayalamFractionOneOneDashHundredDashAndDashSixtieth,
    /// \u{d59}: '൙'
    MalayalamFractionOneFortieth,
    /// \u{d5a}: '൚'
    MalayalamFractionThreeEightieths,
    /// \u{d5b}: '൛'
    MalayalamFractionOneTwentieth,
    /// \u{d5c}: '൜'
    MalayalamFractionOneTenth,
    /// \u{d5d}: '൝'
    MalayalamFractionThreeTwentieths,
    /// \u{d5e}: '൞'
    MalayalamFractionOneFifth,
    /// \u{d5f}: 'ൟ'
    MalayalamLetterArchaicIi,
    /// \u{d60}: 'ൠ'
    MalayalamLetterVocalicRr,
    /// \u{d61}: 'ൡ'
    MalayalamLetterVocalicLl,
    /// \u{d62}: 'ൢ'
    MalayalamVowelSignVocalicL,
    /// \u{d63}: 'ൣ'
    MalayalamVowelSignVocalicLl,
    /// \u{d66}: '൦'
    MalayalamDigitZero,
    /// \u{d67}: '൧'
    MalayalamDigitOne,
    /// \u{d68}: '൨'
    MalayalamDigitTwo,
    /// \u{d69}: '൩'
    MalayalamDigitThree,
    /// \u{d6a}: '൪'
    MalayalamDigitFour,
    /// \u{d6b}: '൫'
    MalayalamDigitFive,
    /// \u{d6c}: '൬'
    MalayalamDigitSix,
    /// \u{d6d}: '൭'
    MalayalamDigitSeven,
    /// \u{d6e}: '൮'
    MalayalamDigitEight,
    /// \u{d6f}: '൯'
    MalayalamDigitNine,
    /// \u{d70}: '൰'
    MalayalamNumberTen,
    /// \u{d71}: '൱'
    MalayalamNumberOneHundred,
    /// \u{d72}: '൲'
    MalayalamNumberOneThousand,
    /// \u{d73}: '൳'
    MalayalamFractionOneQuarter,
    /// \u{d74}: '൴'
    MalayalamFractionOneHalf,
    /// \u{d75}: '൵'
    MalayalamFractionThreeQuarters,
    /// \u{d76}: '൶'
    MalayalamFractionOneSixteenth,
    /// \u{d77}: '൷'
    MalayalamFractionOneEighth,
    /// \u{d78}: '൸'
    MalayalamFractionThreeSixteenths,
    /// \u{d79}: '൹'
    MalayalamDateMark,
    /// \u{d7a}: 'ൺ'
    MalayalamLetterChilluNn,
    /// \u{d7b}: 'ൻ'
    MalayalamLetterChilluN,
    /// \u{d7c}: 'ർ'
    MalayalamLetterChilluRr,
    /// \u{d7d}: 'ൽ'
    MalayalamLetterChilluL,
    /// \u{d7e}: 'ൾ'
    MalayalamLetterChilluLl,
}

impl Into<char> for Malayalam {
    fn into(self) -> char {
        use constants::*;
        match self {
            Malayalam::MalayalamSignCombiningAnusvaraAbove => MALAYALAM_SIGN_COMBINING_ANUSVARA_ABOVE,
            Malayalam::MalayalamSignCandrabindu => MALAYALAM_SIGN_CANDRABINDU,
            Malayalam::MalayalamSignAnusvara => MALAYALAM_SIGN_ANUSVARA,
            Malayalam::MalayalamSignVisarga => MALAYALAM_SIGN_VISARGA,
            Malayalam::MalayalamLetterA => MALAYALAM_LETTER_A,
            Malayalam::MalayalamLetterAa => MALAYALAM_LETTER_AA,
            Malayalam::MalayalamLetterI => MALAYALAM_LETTER_I,
            Malayalam::MalayalamLetterIi => MALAYALAM_LETTER_II,
            Malayalam::MalayalamLetterU => MALAYALAM_LETTER_U,
            Malayalam::MalayalamLetterUu => MALAYALAM_LETTER_UU,
            Malayalam::MalayalamLetterVocalicR => MALAYALAM_LETTER_VOCALIC_R,
            Malayalam::MalayalamLetterVocalicL => MALAYALAM_LETTER_VOCALIC_L,
            Malayalam::MalayalamLetterE => MALAYALAM_LETTER_E,
            Malayalam::MalayalamLetterEe => MALAYALAM_LETTER_EE,
            Malayalam::MalayalamLetterAi => MALAYALAM_LETTER_AI,
            Malayalam::MalayalamLetterO => MALAYALAM_LETTER_O,
            Malayalam::MalayalamLetterOo => MALAYALAM_LETTER_OO,
            Malayalam::MalayalamLetterAu => MALAYALAM_LETTER_AU,
            Malayalam::MalayalamLetterKa => MALAYALAM_LETTER_KA,
            Malayalam::MalayalamLetterKha => MALAYALAM_LETTER_KHA,
            Malayalam::MalayalamLetterGa => MALAYALAM_LETTER_GA,
            Malayalam::MalayalamLetterGha => MALAYALAM_LETTER_GHA,
            Malayalam::MalayalamLetterNga => MALAYALAM_LETTER_NGA,
            Malayalam::MalayalamLetterCa => MALAYALAM_LETTER_CA,
            Malayalam::MalayalamLetterCha => MALAYALAM_LETTER_CHA,
            Malayalam::MalayalamLetterJa => MALAYALAM_LETTER_JA,
            Malayalam::MalayalamLetterJha => MALAYALAM_LETTER_JHA,
            Malayalam::MalayalamLetterNya => MALAYALAM_LETTER_NYA,
            Malayalam::MalayalamLetterTta => MALAYALAM_LETTER_TTA,
            Malayalam::MalayalamLetterTtha => MALAYALAM_LETTER_TTHA,
            Malayalam::MalayalamLetterDda => MALAYALAM_LETTER_DDA,
            Malayalam::MalayalamLetterDdha => MALAYALAM_LETTER_DDHA,
            Malayalam::MalayalamLetterNna => MALAYALAM_LETTER_NNA,
            Malayalam::MalayalamLetterTa => MALAYALAM_LETTER_TA,
            Malayalam::MalayalamLetterTha => MALAYALAM_LETTER_THA,
            Malayalam::MalayalamLetterDa => MALAYALAM_LETTER_DA,
            Malayalam::MalayalamLetterDha => MALAYALAM_LETTER_DHA,
            Malayalam::MalayalamLetterNa => MALAYALAM_LETTER_NA,
            Malayalam::MalayalamLetterNnna => MALAYALAM_LETTER_NNNA,
            Malayalam::MalayalamLetterPa => MALAYALAM_LETTER_PA,
            Malayalam::MalayalamLetterPha => MALAYALAM_LETTER_PHA,
            Malayalam::MalayalamLetterBa => MALAYALAM_LETTER_BA,
            Malayalam::MalayalamLetterBha => MALAYALAM_LETTER_BHA,
            Malayalam::MalayalamLetterMa => MALAYALAM_LETTER_MA,
            Malayalam::MalayalamLetterYa => MALAYALAM_LETTER_YA,
            Malayalam::MalayalamLetterRa => MALAYALAM_LETTER_RA,
            Malayalam::MalayalamLetterRra => MALAYALAM_LETTER_RRA,
            Malayalam::MalayalamLetterLa => MALAYALAM_LETTER_LA,
            Malayalam::MalayalamLetterLla => MALAYALAM_LETTER_LLA,
            Malayalam::MalayalamLetterLlla => MALAYALAM_LETTER_LLLA,
            Malayalam::MalayalamLetterVa => MALAYALAM_LETTER_VA,
            Malayalam::MalayalamLetterSha => MALAYALAM_LETTER_SHA,
            Malayalam::MalayalamLetterSsa => MALAYALAM_LETTER_SSA,
            Malayalam::MalayalamLetterSa => MALAYALAM_LETTER_SA,
            Malayalam::MalayalamLetterHa => MALAYALAM_LETTER_HA,
            Malayalam::MalayalamLetterTtta => MALAYALAM_LETTER_TTTA,
            Malayalam::MalayalamSignVerticalBarVirama => MALAYALAM_SIGN_VERTICAL_BAR_VIRAMA,
            Malayalam::MalayalamSignCircularVirama => MALAYALAM_SIGN_CIRCULAR_VIRAMA,
            Malayalam::MalayalamSignAvagraha => MALAYALAM_SIGN_AVAGRAHA,
            Malayalam::MalayalamVowelSignAa => MALAYALAM_VOWEL_SIGN_AA,
            Malayalam::MalayalamVowelSignI => MALAYALAM_VOWEL_SIGN_I,
            Malayalam::MalayalamVowelSignIi => MALAYALAM_VOWEL_SIGN_II,
            Malayalam::MalayalamVowelSignU => MALAYALAM_VOWEL_SIGN_U,
            Malayalam::MalayalamVowelSignUu => MALAYALAM_VOWEL_SIGN_UU,
            Malayalam::MalayalamVowelSignVocalicR => MALAYALAM_VOWEL_SIGN_VOCALIC_R,
            Malayalam::MalayalamVowelSignVocalicRr => MALAYALAM_VOWEL_SIGN_VOCALIC_RR,
            Malayalam::MalayalamVowelSignE => MALAYALAM_VOWEL_SIGN_E,
            Malayalam::MalayalamVowelSignEe => MALAYALAM_VOWEL_SIGN_EE,
            Malayalam::MalayalamVowelSignAi => MALAYALAM_VOWEL_SIGN_AI,
            Malayalam::MalayalamVowelSignO => MALAYALAM_VOWEL_SIGN_O,
            Malayalam::MalayalamVowelSignOo => MALAYALAM_VOWEL_SIGN_OO,
            Malayalam::MalayalamVowelSignAu => MALAYALAM_VOWEL_SIGN_AU,
            Malayalam::MalayalamSignVirama => MALAYALAM_SIGN_VIRAMA,
            Malayalam::MalayalamLetterDotReph => MALAYALAM_LETTER_DOT_REPH,
            Malayalam::MalayalamSignPara => MALAYALAM_SIGN_PARA,
            Malayalam::MalayalamLetterChilluM => MALAYALAM_LETTER_CHILLU_M,
            Malayalam::MalayalamLetterChilluY => MALAYALAM_LETTER_CHILLU_Y,
            Malayalam::MalayalamLetterChilluLll => MALAYALAM_LETTER_CHILLU_LLL,
            Malayalam::MalayalamAuLengthMark => MALAYALAM_AU_LENGTH_MARK,
            Malayalam::MalayalamFractionOneOneDashHundredDashAndDashSixtieth => MALAYALAM_FRACTION_ONE_ONE_DASH_HUNDRED_DASH_AND_DASH_SIXTIETH,
            Malayalam::MalayalamFractionOneFortieth => MALAYALAM_FRACTION_ONE_FORTIETH,
            Malayalam::MalayalamFractionThreeEightieths => MALAYALAM_FRACTION_THREE_EIGHTIETHS,
            Malayalam::MalayalamFractionOneTwentieth => MALAYALAM_FRACTION_ONE_TWENTIETH,
            Malayalam::MalayalamFractionOneTenth => MALAYALAM_FRACTION_ONE_TENTH,
            Malayalam::MalayalamFractionThreeTwentieths => MALAYALAM_FRACTION_THREE_TWENTIETHS,
            Malayalam::MalayalamFractionOneFifth => MALAYALAM_FRACTION_ONE_FIFTH,
            Malayalam::MalayalamLetterArchaicIi => MALAYALAM_LETTER_ARCHAIC_II,
            Malayalam::MalayalamLetterVocalicRr => MALAYALAM_LETTER_VOCALIC_RR,
            Malayalam::MalayalamLetterVocalicLl => MALAYALAM_LETTER_VOCALIC_LL,
            Malayalam::MalayalamVowelSignVocalicL => MALAYALAM_VOWEL_SIGN_VOCALIC_L,
            Malayalam::MalayalamVowelSignVocalicLl => MALAYALAM_VOWEL_SIGN_VOCALIC_LL,
            Malayalam::MalayalamDigitZero => MALAYALAM_DIGIT_ZERO,
            Malayalam::MalayalamDigitOne => MALAYALAM_DIGIT_ONE,
            Malayalam::MalayalamDigitTwo => MALAYALAM_DIGIT_TWO,
            Malayalam::MalayalamDigitThree => MALAYALAM_DIGIT_THREE,
            Malayalam::MalayalamDigitFour => MALAYALAM_DIGIT_FOUR,
            Malayalam::MalayalamDigitFive => MALAYALAM_DIGIT_FIVE,
            Malayalam::MalayalamDigitSix => MALAYALAM_DIGIT_SIX,
            Malayalam::MalayalamDigitSeven => MALAYALAM_DIGIT_SEVEN,
            Malayalam::MalayalamDigitEight => MALAYALAM_DIGIT_EIGHT,
            Malayalam::MalayalamDigitNine => MALAYALAM_DIGIT_NINE,
            Malayalam::MalayalamNumberTen => MALAYALAM_NUMBER_TEN,
            Malayalam::MalayalamNumberOneHundred => MALAYALAM_NUMBER_ONE_HUNDRED,
            Malayalam::MalayalamNumberOneThousand => MALAYALAM_NUMBER_ONE_THOUSAND,
            Malayalam::MalayalamFractionOneQuarter => MALAYALAM_FRACTION_ONE_QUARTER,
            Malayalam::MalayalamFractionOneHalf => MALAYALAM_FRACTION_ONE_HALF,
            Malayalam::MalayalamFractionThreeQuarters => MALAYALAM_FRACTION_THREE_QUARTERS,
            Malayalam::MalayalamFractionOneSixteenth => MALAYALAM_FRACTION_ONE_SIXTEENTH,
            Malayalam::MalayalamFractionOneEighth => MALAYALAM_FRACTION_ONE_EIGHTH,
            Malayalam::MalayalamFractionThreeSixteenths => MALAYALAM_FRACTION_THREE_SIXTEENTHS,
            Malayalam::MalayalamDateMark => MALAYALAM_DATE_MARK,
            Malayalam::MalayalamLetterChilluNn => MALAYALAM_LETTER_CHILLU_NN,
            Malayalam::MalayalamLetterChilluN => MALAYALAM_LETTER_CHILLU_N,
            Malayalam::MalayalamLetterChilluRr => MALAYALAM_LETTER_CHILLU_RR,
            Malayalam::MalayalamLetterChilluL => MALAYALAM_LETTER_CHILLU_L,
            Malayalam::MalayalamLetterChilluLl => MALAYALAM_LETTER_CHILLU_LL,
        }
    }
}

impl std::convert::TryFrom<char> for Malayalam {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            MALAYALAM_SIGN_COMBINING_ANUSVARA_ABOVE => Ok(Malayalam::MalayalamSignCombiningAnusvaraAbove),
            MALAYALAM_SIGN_CANDRABINDU => Ok(Malayalam::MalayalamSignCandrabindu),
            MALAYALAM_SIGN_ANUSVARA => Ok(Malayalam::MalayalamSignAnusvara),
            MALAYALAM_SIGN_VISARGA => Ok(Malayalam::MalayalamSignVisarga),
            MALAYALAM_LETTER_A => Ok(Malayalam::MalayalamLetterA),
            MALAYALAM_LETTER_AA => Ok(Malayalam::MalayalamLetterAa),
            MALAYALAM_LETTER_I => Ok(Malayalam::MalayalamLetterI),
            MALAYALAM_LETTER_II => Ok(Malayalam::MalayalamLetterIi),
            MALAYALAM_LETTER_U => Ok(Malayalam::MalayalamLetterU),
            MALAYALAM_LETTER_UU => Ok(Malayalam::MalayalamLetterUu),
            MALAYALAM_LETTER_VOCALIC_R => Ok(Malayalam::MalayalamLetterVocalicR),
            MALAYALAM_LETTER_VOCALIC_L => Ok(Malayalam::MalayalamLetterVocalicL),
            MALAYALAM_LETTER_E => Ok(Malayalam::MalayalamLetterE),
            MALAYALAM_LETTER_EE => Ok(Malayalam::MalayalamLetterEe),
            MALAYALAM_LETTER_AI => Ok(Malayalam::MalayalamLetterAi),
            MALAYALAM_LETTER_O => Ok(Malayalam::MalayalamLetterO),
            MALAYALAM_LETTER_OO => Ok(Malayalam::MalayalamLetterOo),
            MALAYALAM_LETTER_AU => Ok(Malayalam::MalayalamLetterAu),
            MALAYALAM_LETTER_KA => Ok(Malayalam::MalayalamLetterKa),
            MALAYALAM_LETTER_KHA => Ok(Malayalam::MalayalamLetterKha),
            MALAYALAM_LETTER_GA => Ok(Malayalam::MalayalamLetterGa),
            MALAYALAM_LETTER_GHA => Ok(Malayalam::MalayalamLetterGha),
            MALAYALAM_LETTER_NGA => Ok(Malayalam::MalayalamLetterNga),
            MALAYALAM_LETTER_CA => Ok(Malayalam::MalayalamLetterCa),
            MALAYALAM_LETTER_CHA => Ok(Malayalam::MalayalamLetterCha),
            MALAYALAM_LETTER_JA => Ok(Malayalam::MalayalamLetterJa),
            MALAYALAM_LETTER_JHA => Ok(Malayalam::MalayalamLetterJha),
            MALAYALAM_LETTER_NYA => Ok(Malayalam::MalayalamLetterNya),
            MALAYALAM_LETTER_TTA => Ok(Malayalam::MalayalamLetterTta),
            MALAYALAM_LETTER_TTHA => Ok(Malayalam::MalayalamLetterTtha),
            MALAYALAM_LETTER_DDA => Ok(Malayalam::MalayalamLetterDda),
            MALAYALAM_LETTER_DDHA => Ok(Malayalam::MalayalamLetterDdha),
            MALAYALAM_LETTER_NNA => Ok(Malayalam::MalayalamLetterNna),
            MALAYALAM_LETTER_TA => Ok(Malayalam::MalayalamLetterTa),
            MALAYALAM_LETTER_THA => Ok(Malayalam::MalayalamLetterTha),
            MALAYALAM_LETTER_DA => Ok(Malayalam::MalayalamLetterDa),
            MALAYALAM_LETTER_DHA => Ok(Malayalam::MalayalamLetterDha),
            MALAYALAM_LETTER_NA => Ok(Malayalam::MalayalamLetterNa),
            MALAYALAM_LETTER_NNNA => Ok(Malayalam::MalayalamLetterNnna),
            MALAYALAM_LETTER_PA => Ok(Malayalam::MalayalamLetterPa),
            MALAYALAM_LETTER_PHA => Ok(Malayalam::MalayalamLetterPha),
            MALAYALAM_LETTER_BA => Ok(Malayalam::MalayalamLetterBa),
            MALAYALAM_LETTER_BHA => Ok(Malayalam::MalayalamLetterBha),
            MALAYALAM_LETTER_MA => Ok(Malayalam::MalayalamLetterMa),
            MALAYALAM_LETTER_YA => Ok(Malayalam::MalayalamLetterYa),
            MALAYALAM_LETTER_RA => Ok(Malayalam::MalayalamLetterRa),
            MALAYALAM_LETTER_RRA => Ok(Malayalam::MalayalamLetterRra),
            MALAYALAM_LETTER_LA => Ok(Malayalam::MalayalamLetterLa),
            MALAYALAM_LETTER_LLA => Ok(Malayalam::MalayalamLetterLla),
            MALAYALAM_LETTER_LLLA => Ok(Malayalam::MalayalamLetterLlla),
            MALAYALAM_LETTER_VA => Ok(Malayalam::MalayalamLetterVa),
            MALAYALAM_LETTER_SHA => Ok(Malayalam::MalayalamLetterSha),
            MALAYALAM_LETTER_SSA => Ok(Malayalam::MalayalamLetterSsa),
            MALAYALAM_LETTER_SA => Ok(Malayalam::MalayalamLetterSa),
            MALAYALAM_LETTER_HA => Ok(Malayalam::MalayalamLetterHa),
            MALAYALAM_LETTER_TTTA => Ok(Malayalam::MalayalamLetterTtta),
            MALAYALAM_SIGN_VERTICAL_BAR_VIRAMA => Ok(Malayalam::MalayalamSignVerticalBarVirama),
            MALAYALAM_SIGN_CIRCULAR_VIRAMA => Ok(Malayalam::MalayalamSignCircularVirama),
            MALAYALAM_SIGN_AVAGRAHA => Ok(Malayalam::MalayalamSignAvagraha),
            MALAYALAM_VOWEL_SIGN_AA => Ok(Malayalam::MalayalamVowelSignAa),
            MALAYALAM_VOWEL_SIGN_I => Ok(Malayalam::MalayalamVowelSignI),
            MALAYALAM_VOWEL_SIGN_II => Ok(Malayalam::MalayalamVowelSignIi),
            MALAYALAM_VOWEL_SIGN_U => Ok(Malayalam::MalayalamVowelSignU),
            MALAYALAM_VOWEL_SIGN_UU => Ok(Malayalam::MalayalamVowelSignUu),
            MALAYALAM_VOWEL_SIGN_VOCALIC_R => Ok(Malayalam::MalayalamVowelSignVocalicR),
            MALAYALAM_VOWEL_SIGN_VOCALIC_RR => Ok(Malayalam::MalayalamVowelSignVocalicRr),
            MALAYALAM_VOWEL_SIGN_E => Ok(Malayalam::MalayalamVowelSignE),
            MALAYALAM_VOWEL_SIGN_EE => Ok(Malayalam::MalayalamVowelSignEe),
            MALAYALAM_VOWEL_SIGN_AI => Ok(Malayalam::MalayalamVowelSignAi),
            MALAYALAM_VOWEL_SIGN_O => Ok(Malayalam::MalayalamVowelSignO),
            MALAYALAM_VOWEL_SIGN_OO => Ok(Malayalam::MalayalamVowelSignOo),
            MALAYALAM_VOWEL_SIGN_AU => Ok(Malayalam::MalayalamVowelSignAu),
            MALAYALAM_SIGN_VIRAMA => Ok(Malayalam::MalayalamSignVirama),
            MALAYALAM_LETTER_DOT_REPH => Ok(Malayalam::MalayalamLetterDotReph),
            MALAYALAM_SIGN_PARA => Ok(Malayalam::MalayalamSignPara),
            MALAYALAM_LETTER_CHILLU_M => Ok(Malayalam::MalayalamLetterChilluM),
            MALAYALAM_LETTER_CHILLU_Y => Ok(Malayalam::MalayalamLetterChilluY),
            MALAYALAM_LETTER_CHILLU_LLL => Ok(Malayalam::MalayalamLetterChilluLll),
            MALAYALAM_AU_LENGTH_MARK => Ok(Malayalam::MalayalamAuLengthMark),
            MALAYALAM_FRACTION_ONE_ONE_DASH_HUNDRED_DASH_AND_DASH_SIXTIETH => Ok(Malayalam::MalayalamFractionOneOneDashHundredDashAndDashSixtieth),
            MALAYALAM_FRACTION_ONE_FORTIETH => Ok(Malayalam::MalayalamFractionOneFortieth),
            MALAYALAM_FRACTION_THREE_EIGHTIETHS => Ok(Malayalam::MalayalamFractionThreeEightieths),
            MALAYALAM_FRACTION_ONE_TWENTIETH => Ok(Malayalam::MalayalamFractionOneTwentieth),
            MALAYALAM_FRACTION_ONE_TENTH => Ok(Malayalam::MalayalamFractionOneTenth),
            MALAYALAM_FRACTION_THREE_TWENTIETHS => Ok(Malayalam::MalayalamFractionThreeTwentieths),
            MALAYALAM_FRACTION_ONE_FIFTH => Ok(Malayalam::MalayalamFractionOneFifth),
            MALAYALAM_LETTER_ARCHAIC_II => Ok(Malayalam::MalayalamLetterArchaicIi),
            MALAYALAM_LETTER_VOCALIC_RR => Ok(Malayalam::MalayalamLetterVocalicRr),
            MALAYALAM_LETTER_VOCALIC_LL => Ok(Malayalam::MalayalamLetterVocalicLl),
            MALAYALAM_VOWEL_SIGN_VOCALIC_L => Ok(Malayalam::MalayalamVowelSignVocalicL),
            MALAYALAM_VOWEL_SIGN_VOCALIC_LL => Ok(Malayalam::MalayalamVowelSignVocalicLl),
            MALAYALAM_DIGIT_ZERO => Ok(Malayalam::MalayalamDigitZero),
            MALAYALAM_DIGIT_ONE => Ok(Malayalam::MalayalamDigitOne),
            MALAYALAM_DIGIT_TWO => Ok(Malayalam::MalayalamDigitTwo),
            MALAYALAM_DIGIT_THREE => Ok(Malayalam::MalayalamDigitThree),
            MALAYALAM_DIGIT_FOUR => Ok(Malayalam::MalayalamDigitFour),
            MALAYALAM_DIGIT_FIVE => Ok(Malayalam::MalayalamDigitFive),
            MALAYALAM_DIGIT_SIX => Ok(Malayalam::MalayalamDigitSix),
            MALAYALAM_DIGIT_SEVEN => Ok(Malayalam::MalayalamDigitSeven),
            MALAYALAM_DIGIT_EIGHT => Ok(Malayalam::MalayalamDigitEight),
            MALAYALAM_DIGIT_NINE => Ok(Malayalam::MalayalamDigitNine),
            MALAYALAM_NUMBER_TEN => Ok(Malayalam::MalayalamNumberTen),
            MALAYALAM_NUMBER_ONE_HUNDRED => Ok(Malayalam::MalayalamNumberOneHundred),
            MALAYALAM_NUMBER_ONE_THOUSAND => Ok(Malayalam::MalayalamNumberOneThousand),
            MALAYALAM_FRACTION_ONE_QUARTER => Ok(Malayalam::MalayalamFractionOneQuarter),
            MALAYALAM_FRACTION_ONE_HALF => Ok(Malayalam::MalayalamFractionOneHalf),
            MALAYALAM_FRACTION_THREE_QUARTERS => Ok(Malayalam::MalayalamFractionThreeQuarters),
            MALAYALAM_FRACTION_ONE_SIXTEENTH => Ok(Malayalam::MalayalamFractionOneSixteenth),
            MALAYALAM_FRACTION_ONE_EIGHTH => Ok(Malayalam::MalayalamFractionOneEighth),
            MALAYALAM_FRACTION_THREE_SIXTEENTHS => Ok(Malayalam::MalayalamFractionThreeSixteenths),
            MALAYALAM_DATE_MARK => Ok(Malayalam::MalayalamDateMark),
            MALAYALAM_LETTER_CHILLU_NN => Ok(Malayalam::MalayalamLetterChilluNn),
            MALAYALAM_LETTER_CHILLU_N => Ok(Malayalam::MalayalamLetterChilluN),
            MALAYALAM_LETTER_CHILLU_RR => Ok(Malayalam::MalayalamLetterChilluRr),
            MALAYALAM_LETTER_CHILLU_L => Ok(Malayalam::MalayalamLetterChilluL),
            MALAYALAM_LETTER_CHILLU_LL => Ok(Malayalam::MalayalamLetterChilluLl),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Malayalam {
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

impl std::convert::TryFrom<u32> for Malayalam {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Malayalam {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Malayalam {
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        Malayalam::MalayalamSignCombiningAnusvaraAbove
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Malayalam::MalayalamSignCombiningAnusvaraAbove => "malayalam sign combining anusvara above",
            Malayalam::MalayalamSignCandrabindu => "malayalam sign candrabindu",
            Malayalam::MalayalamSignAnusvara => "malayalam sign anusvara",
            Malayalam::MalayalamSignVisarga => "malayalam sign visarga",
            Malayalam::MalayalamLetterA => "malayalam letter a",
            Malayalam::MalayalamLetterAa => "malayalam letter aa",
            Malayalam::MalayalamLetterI => "malayalam letter i",
            Malayalam::MalayalamLetterIi => "malayalam letter ii",
            Malayalam::MalayalamLetterU => "malayalam letter u",
            Malayalam::MalayalamLetterUu => "malayalam letter uu",
            Malayalam::MalayalamLetterVocalicR => "malayalam letter vocalic r",
            Malayalam::MalayalamLetterVocalicL => "malayalam letter vocalic l",
            Malayalam::MalayalamLetterE => "malayalam letter e",
            Malayalam::MalayalamLetterEe => "malayalam letter ee",
            Malayalam::MalayalamLetterAi => "malayalam letter ai",
            Malayalam::MalayalamLetterO => "malayalam letter o",
            Malayalam::MalayalamLetterOo => "malayalam letter oo",
            Malayalam::MalayalamLetterAu => "malayalam letter au",
            Malayalam::MalayalamLetterKa => "malayalam letter ka",
            Malayalam::MalayalamLetterKha => "malayalam letter kha",
            Malayalam::MalayalamLetterGa => "malayalam letter ga",
            Malayalam::MalayalamLetterGha => "malayalam letter gha",
            Malayalam::MalayalamLetterNga => "malayalam letter nga",
            Malayalam::MalayalamLetterCa => "malayalam letter ca",
            Malayalam::MalayalamLetterCha => "malayalam letter cha",
            Malayalam::MalayalamLetterJa => "malayalam letter ja",
            Malayalam::MalayalamLetterJha => "malayalam letter jha",
            Malayalam::MalayalamLetterNya => "malayalam letter nya",
            Malayalam::MalayalamLetterTta => "malayalam letter tta",
            Malayalam::MalayalamLetterTtha => "malayalam letter ttha",
            Malayalam::MalayalamLetterDda => "malayalam letter dda",
            Malayalam::MalayalamLetterDdha => "malayalam letter ddha",
            Malayalam::MalayalamLetterNna => "malayalam letter nna",
            Malayalam::MalayalamLetterTa => "malayalam letter ta",
            Malayalam::MalayalamLetterTha => "malayalam letter tha",
            Malayalam::MalayalamLetterDa => "malayalam letter da",
            Malayalam::MalayalamLetterDha => "malayalam letter dha",
            Malayalam::MalayalamLetterNa => "malayalam letter na",
            Malayalam::MalayalamLetterNnna => "malayalam letter nnna",
            Malayalam::MalayalamLetterPa => "malayalam letter pa",
            Malayalam::MalayalamLetterPha => "malayalam letter pha",
            Malayalam::MalayalamLetterBa => "malayalam letter ba",
            Malayalam::MalayalamLetterBha => "malayalam letter bha",
            Malayalam::MalayalamLetterMa => "malayalam letter ma",
            Malayalam::MalayalamLetterYa => "malayalam letter ya",
            Malayalam::MalayalamLetterRa => "malayalam letter ra",
            Malayalam::MalayalamLetterRra => "malayalam letter rra",
            Malayalam::MalayalamLetterLa => "malayalam letter la",
            Malayalam::MalayalamLetterLla => "malayalam letter lla",
            Malayalam::MalayalamLetterLlla => "malayalam letter llla",
            Malayalam::MalayalamLetterVa => "malayalam letter va",
            Malayalam::MalayalamLetterSha => "malayalam letter sha",
            Malayalam::MalayalamLetterSsa => "malayalam letter ssa",
            Malayalam::MalayalamLetterSa => "malayalam letter sa",
            Malayalam::MalayalamLetterHa => "malayalam letter ha",
            Malayalam::MalayalamLetterTtta => "malayalam letter ttta",
            Malayalam::MalayalamSignVerticalBarVirama => "malayalam sign vertical bar virama",
            Malayalam::MalayalamSignCircularVirama => "malayalam sign circular virama",
            Malayalam::MalayalamSignAvagraha => "malayalam sign avagraha",
            Malayalam::MalayalamVowelSignAa => "malayalam vowel sign aa",
            Malayalam::MalayalamVowelSignI => "malayalam vowel sign i",
            Malayalam::MalayalamVowelSignIi => "malayalam vowel sign ii",
            Malayalam::MalayalamVowelSignU => "malayalam vowel sign u",
            Malayalam::MalayalamVowelSignUu => "malayalam vowel sign uu",
            Malayalam::MalayalamVowelSignVocalicR => "malayalam vowel sign vocalic r",
            Malayalam::MalayalamVowelSignVocalicRr => "malayalam vowel sign vocalic rr",
            Malayalam::MalayalamVowelSignE => "malayalam vowel sign e",
            Malayalam::MalayalamVowelSignEe => "malayalam vowel sign ee",
            Malayalam::MalayalamVowelSignAi => "malayalam vowel sign ai",
            Malayalam::MalayalamVowelSignO => "malayalam vowel sign o",
            Malayalam::MalayalamVowelSignOo => "malayalam vowel sign oo",
            Malayalam::MalayalamVowelSignAu => "malayalam vowel sign au",
            Malayalam::MalayalamSignVirama => "malayalam sign virama",
            Malayalam::MalayalamLetterDotReph => "malayalam letter dot reph",
            Malayalam::MalayalamSignPara => "malayalam sign para",
            Malayalam::MalayalamLetterChilluM => "malayalam letter chillu m",
            Malayalam::MalayalamLetterChilluY => "malayalam letter chillu y",
            Malayalam::MalayalamLetterChilluLll => "malayalam letter chillu lll",
            Malayalam::MalayalamAuLengthMark => "malayalam au length mark",
            Malayalam::MalayalamFractionOneOneDashHundredDashAndDashSixtieth => "malayalam fraction one one-hundred-and-sixtieth",
            Malayalam::MalayalamFractionOneFortieth => "malayalam fraction one fortieth",
            Malayalam::MalayalamFractionThreeEightieths => "malayalam fraction three eightieths",
            Malayalam::MalayalamFractionOneTwentieth => "malayalam fraction one twentieth",
            Malayalam::MalayalamFractionOneTenth => "malayalam fraction one tenth",
            Malayalam::MalayalamFractionThreeTwentieths => "malayalam fraction three twentieths",
            Malayalam::MalayalamFractionOneFifth => "malayalam fraction one fifth",
            Malayalam::MalayalamLetterArchaicIi => "malayalam letter archaic ii",
            Malayalam::MalayalamLetterVocalicRr => "malayalam letter vocalic rr",
            Malayalam::MalayalamLetterVocalicLl => "malayalam letter vocalic ll",
            Malayalam::MalayalamVowelSignVocalicL => "malayalam vowel sign vocalic l",
            Malayalam::MalayalamVowelSignVocalicLl => "malayalam vowel sign vocalic ll",
            Malayalam::MalayalamDigitZero => "malayalam digit zero",
            Malayalam::MalayalamDigitOne => "malayalam digit one",
            Malayalam::MalayalamDigitTwo => "malayalam digit two",
            Malayalam::MalayalamDigitThree => "malayalam digit three",
            Malayalam::MalayalamDigitFour => "malayalam digit four",
            Malayalam::MalayalamDigitFive => "malayalam digit five",
            Malayalam::MalayalamDigitSix => "malayalam digit six",
            Malayalam::MalayalamDigitSeven => "malayalam digit seven",
            Malayalam::MalayalamDigitEight => "malayalam digit eight",
            Malayalam::MalayalamDigitNine => "malayalam digit nine",
            Malayalam::MalayalamNumberTen => "malayalam number ten",
            Malayalam::MalayalamNumberOneHundred => "malayalam number one hundred",
            Malayalam::MalayalamNumberOneThousand => "malayalam number one thousand",
            Malayalam::MalayalamFractionOneQuarter => "malayalam fraction one quarter",
            Malayalam::MalayalamFractionOneHalf => "malayalam fraction one half",
            Malayalam::MalayalamFractionThreeQuarters => "malayalam fraction three quarters",
            Malayalam::MalayalamFractionOneSixteenth => "malayalam fraction one sixteenth",
            Malayalam::MalayalamFractionOneEighth => "malayalam fraction one eighth",
            Malayalam::MalayalamFractionThreeSixteenths => "malayalam fraction three sixteenths",
            Malayalam::MalayalamDateMark => "malayalam date mark",
            Malayalam::MalayalamLetterChilluNn => "malayalam letter chillu nn",
            Malayalam::MalayalamLetterChilluN => "malayalam letter chillu n",
            Malayalam::MalayalamLetterChilluRr => "malayalam letter chillu rr",
            Malayalam::MalayalamLetterChilluL => "malayalam letter chillu l",
            Malayalam::MalayalamLetterChilluLl => "malayalam letter chillu ll",
        }
    }
}
