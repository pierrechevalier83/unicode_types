/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{d00}: 'ഀ'
    pub const SIGN_COMBINING_ANUSVARA_ABOVE: char = 'ഀ';
    /// \u{d01}: 'ഁ'
    pub const SIGN_CANDRABINDU: char = 'ഁ';
    /// \u{d02}: 'ം'
    pub const SIGN_ANUSVARA: char = 'ം';
    /// \u{d03}: 'ഃ'
    pub const SIGN_VISARGA: char = 'ഃ';
    /// \u{d05}: 'അ'
    pub const LETTER_A: char = 'അ';
    /// \u{d06}: 'ആ'
    pub const LETTER_AA: char = 'ആ';
    /// \u{d07}: 'ഇ'
    pub const LETTER_I: char = 'ഇ';
    /// \u{d08}: 'ഈ'
    pub const LETTER_II: char = 'ഈ';
    /// \u{d09}: 'ഉ'
    pub const LETTER_U: char = 'ഉ';
    /// \u{d0a}: 'ഊ'
    pub const LETTER_UU: char = 'ഊ';
    /// \u{d0b}: 'ഋ'
    pub const LETTER_VOCALIC_R: char = 'ഋ';
    /// \u{d0c}: 'ഌ'
    pub const LETTER_VOCALIC_L: char = 'ഌ';
    /// \u{d0e}: 'എ'
    pub const LETTER_E: char = 'എ';
    /// \u{d0f}: 'ഏ'
    pub const LETTER_EE: char = 'ഏ';
    /// \u{d10}: 'ഐ'
    pub const LETTER_AI: char = 'ഐ';
    /// \u{d12}: 'ഒ'
    pub const LETTER_O: char = 'ഒ';
    /// \u{d13}: 'ഓ'
    pub const LETTER_OO: char = 'ഓ';
    /// \u{d14}: 'ഔ'
    pub const LETTER_AU: char = 'ഔ';
    /// \u{d15}: 'ക'
    pub const LETTER_KA: char = 'ക';
    /// \u{d16}: 'ഖ'
    pub const LETTER_KHA: char = 'ഖ';
    /// \u{d17}: 'ഗ'
    pub const LETTER_GA: char = 'ഗ';
    /// \u{d18}: 'ഘ'
    pub const LETTER_GHA: char = 'ഘ';
    /// \u{d19}: 'ങ'
    pub const LETTER_NGA: char = 'ങ';
    /// \u{d1a}: 'ച'
    pub const LETTER_CA: char = 'ച';
    /// \u{d1b}: 'ഛ'
    pub const LETTER_CHA: char = 'ഛ';
    /// \u{d1c}: 'ജ'
    pub const LETTER_JA: char = 'ജ';
    /// \u{d1d}: 'ഝ'
    pub const LETTER_JHA: char = 'ഝ';
    /// \u{d1e}: 'ഞ'
    pub const LETTER_NYA: char = 'ഞ';
    /// \u{d1f}: 'ട'
    pub const LETTER_TTA: char = 'ട';
    /// \u{d20}: 'ഠ'
    pub const LETTER_TTHA: char = 'ഠ';
    /// \u{d21}: 'ഡ'
    pub const LETTER_DDA: char = 'ഡ';
    /// \u{d22}: 'ഢ'
    pub const LETTER_DDHA: char = 'ഢ';
    /// \u{d23}: 'ണ'
    pub const LETTER_NNA: char = 'ണ';
    /// \u{d24}: 'ത'
    pub const LETTER_TA: char = 'ത';
    /// \u{d25}: 'ഥ'
    pub const LETTER_THA: char = 'ഥ';
    /// \u{d26}: 'ദ'
    pub const LETTER_DA: char = 'ദ';
    /// \u{d27}: 'ധ'
    pub const LETTER_DHA: char = 'ധ';
    /// \u{d28}: 'ന'
    pub const LETTER_NA: char = 'ന';
    /// \u{d29}: 'ഩ'
    pub const LETTER_NNNA: char = 'ഩ';
    /// \u{d2a}: 'പ'
    pub const LETTER_PA: char = 'പ';
    /// \u{d2b}: 'ഫ'
    pub const LETTER_PHA: char = 'ഫ';
    /// \u{d2c}: 'ബ'
    pub const LETTER_BA: char = 'ബ';
    /// \u{d2d}: 'ഭ'
    pub const LETTER_BHA: char = 'ഭ';
    /// \u{d2e}: 'മ'
    pub const LETTER_MA: char = 'മ';
    /// \u{d2f}: 'യ'
    pub const LETTER_YA: char = 'യ';
    /// \u{d30}: 'ര'
    pub const LETTER_RA: char = 'ര';
    /// \u{d31}: 'റ'
    pub const LETTER_RRA: char = 'റ';
    /// \u{d32}: 'ല'
    pub const LETTER_LA: char = 'ല';
    /// \u{d33}: 'ള'
    pub const LETTER_LLA: char = 'ള';
    /// \u{d34}: 'ഴ'
    pub const LETTER_LLLA: char = 'ഴ';
    /// \u{d35}: 'വ'
    pub const LETTER_VA: char = 'വ';
    /// \u{d36}: 'ശ'
    pub const LETTER_SHA: char = 'ശ';
    /// \u{d37}: 'ഷ'
    pub const LETTER_SSA: char = 'ഷ';
    /// \u{d38}: 'സ'
    pub const LETTER_SA: char = 'സ';
    /// \u{d39}: 'ഹ'
    pub const LETTER_HA: char = 'ഹ';
    /// \u{d3a}: 'ഺ'
    pub const LETTER_TTTA: char = 'ഺ';
    /// \u{d3b}: '഻'
    pub const SIGN_VERTICAL_BAR_VIRAMA: char = '഻';
    /// \u{d3c}: '഼'
    pub const SIGN_CIRCULAR_VIRAMA: char = '഼';
    /// \u{d3d}: 'ഽ'
    pub const SIGN_AVAGRAHA: char = 'ഽ';
    /// \u{d3e}: 'ാ'
    pub const VOWEL_SIGN_AA: char = 'ാ';
    /// \u{d3f}: 'ി'
    pub const VOWEL_SIGN_I: char = 'ി';
    /// \u{d40}: 'ീ'
    pub const VOWEL_SIGN_II: char = 'ീ';
    /// \u{d41}: 'ു'
    pub const VOWEL_SIGN_U: char = 'ു';
    /// \u{d42}: 'ൂ'
    pub const VOWEL_SIGN_UU: char = 'ൂ';
    /// \u{d43}: 'ൃ'
    pub const VOWEL_SIGN_VOCALIC_R: char = 'ൃ';
    /// \u{d44}: 'ൄ'
    pub const VOWEL_SIGN_VOCALIC_RR: char = 'ൄ';
    /// \u{d46}: 'െ'
    pub const VOWEL_SIGN_E: char = 'െ';
    /// \u{d47}: 'േ'
    pub const VOWEL_SIGN_EE: char = 'േ';
    /// \u{d48}: 'ൈ'
    pub const VOWEL_SIGN_AI: char = 'ൈ';
    /// \u{d4a}: 'ൊ'
    pub const VOWEL_SIGN_O: char = 'ൊ';
    /// \u{d4b}: 'ോ'
    pub const VOWEL_SIGN_OO: char = 'ോ';
    /// \u{d4c}: 'ൌ'
    pub const VOWEL_SIGN_AU: char = 'ൌ';
    /// \u{d4d}: '്'
    pub const SIGN_VIRAMA: char = '്';
    /// \u{d4e}: 'ൎ'
    pub const LETTER_DOT_REPH: char = 'ൎ';
    /// \u{d4f}: '൏'
    pub const SIGN_PARA: char = '൏';
    /// \u{d54}: 'ൔ'
    pub const LETTER_CHILLU_M: char = 'ൔ';
    /// \u{d55}: 'ൕ'
    pub const LETTER_CHILLU_Y: char = 'ൕ';
    /// \u{d56}: 'ൖ'
    pub const LETTER_CHILLU_LLL: char = 'ൖ';
    /// \u{d57}: 'ൗ'
    pub const AU_LENGTH_MARK: char = 'ൗ';
    /// \u{d58}: '൘'
    pub const FRACTION_ONE_ONE_DASH_HUNDRED_DASH_AND_DASH_SIXTIETH: char = '൘';
    /// \u{d59}: '൙'
    pub const FRACTION_ONE_FORTIETH: char = '൙';
    /// \u{d5a}: '൚'
    pub const FRACTION_THREE_EIGHTIETHS: char = '൚';
    /// \u{d5b}: '൛'
    pub const FRACTION_ONE_TWENTIETH: char = '൛';
    /// \u{d5c}: '൜'
    pub const FRACTION_ONE_TENTH: char = '൜';
    /// \u{d5d}: '൝'
    pub const FRACTION_THREE_TWENTIETHS: char = '൝';
    /// \u{d5e}: '൞'
    pub const FRACTION_ONE_FIFTH: char = '൞';
    /// \u{d5f}: 'ൟ'
    pub const LETTER_ARCHAIC_II: char = 'ൟ';
    /// \u{d60}: 'ൠ'
    pub const LETTER_VOCALIC_RR: char = 'ൠ';
    /// \u{d61}: 'ൡ'
    pub const LETTER_VOCALIC_LL: char = 'ൡ';
    /// \u{d62}: 'ൢ'
    pub const VOWEL_SIGN_VOCALIC_L: char = 'ൢ';
    /// \u{d63}: 'ൣ'
    pub const VOWEL_SIGN_VOCALIC_LL: char = 'ൣ';
    /// \u{d66}: '൦'
    pub const DIGIT_ZERO: char = '൦';
    /// \u{d67}: '൧'
    pub const DIGIT_ONE: char = '൧';
    /// \u{d68}: '൨'
    pub const DIGIT_TWO: char = '൨';
    /// \u{d69}: '൩'
    pub const DIGIT_THREE: char = '൩';
    /// \u{d6a}: '൪'
    pub const DIGIT_FOUR: char = '൪';
    /// \u{d6b}: '൫'
    pub const DIGIT_FIVE: char = '൫';
    /// \u{d6c}: '൬'
    pub const DIGIT_SIX: char = '൬';
    /// \u{d6d}: '൭'
    pub const DIGIT_SEVEN: char = '൭';
    /// \u{d6e}: '൮'
    pub const DIGIT_EIGHT: char = '൮';
    /// \u{d6f}: '൯'
    pub const DIGIT_NINE: char = '൯';
    /// \u{d70}: '൰'
    pub const NUMBER_TEN: char = '൰';
    /// \u{d71}: '൱'
    pub const NUMBER_ONE_HUNDRED: char = '൱';
    /// \u{d72}: '൲'
    pub const NUMBER_ONE_THOUSAND: char = '൲';
    /// \u{d73}: '൳'
    pub const FRACTION_ONE_QUARTER: char = '൳';
    /// \u{d74}: '൴'
    pub const FRACTION_ONE_HALF: char = '൴';
    /// \u{d75}: '൵'
    pub const FRACTION_THREE_QUARTERS: char = '൵';
    /// \u{d76}: '൶'
    pub const FRACTION_ONE_SIXTEENTH: char = '൶';
    /// \u{d77}: '൷'
    pub const FRACTION_ONE_EIGHTH: char = '൷';
    /// \u{d78}: '൸'
    pub const FRACTION_THREE_SIXTEENTHS: char = '൸';
    /// \u{d79}: '൹'
    pub const DATE_MARK: char = '൹';
    /// \u{d7a}: 'ൺ'
    pub const LETTER_CHILLU_NN: char = 'ൺ';
    /// \u{d7b}: 'ൻ'
    pub const LETTER_CHILLU_N: char = 'ൻ';
    /// \u{d7c}: 'ർ'
    pub const LETTER_CHILLU_RR: char = 'ർ';
    /// \u{d7d}: 'ൽ'
    pub const LETTER_CHILLU_L: char = 'ൽ';
    /// \u{d7e}: 'ൾ'
    pub const LETTER_CHILLU_LL: char = 'ൾ';
}

/// An enum to represent all characters in the Malayalam block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Malayalam {
    /// \u{d00}: 'ഀ'
    SignCombiningAnusvaraAbove,
    /// \u{d01}: 'ഁ'
    SignCandrabindu,
    /// \u{d02}: 'ം'
    SignAnusvara,
    /// \u{d03}: 'ഃ'
    SignVisarga,
    /// \u{d05}: 'അ'
    LetterA,
    /// \u{d06}: 'ആ'
    LetterAa,
    /// \u{d07}: 'ഇ'
    LetterI,
    /// \u{d08}: 'ഈ'
    LetterIi,
    /// \u{d09}: 'ഉ'
    LetterU,
    /// \u{d0a}: 'ഊ'
    LetterUu,
    /// \u{d0b}: 'ഋ'
    LetterVocalicR,
    /// \u{d0c}: 'ഌ'
    LetterVocalicL,
    /// \u{d0e}: 'എ'
    LetterE,
    /// \u{d0f}: 'ഏ'
    LetterEe,
    /// \u{d10}: 'ഐ'
    LetterAi,
    /// \u{d12}: 'ഒ'
    LetterO,
    /// \u{d13}: 'ഓ'
    LetterOo,
    /// \u{d14}: 'ഔ'
    LetterAu,
    /// \u{d15}: 'ക'
    LetterKa,
    /// \u{d16}: 'ഖ'
    LetterKha,
    /// \u{d17}: 'ഗ'
    LetterGa,
    /// \u{d18}: 'ഘ'
    LetterGha,
    /// \u{d19}: 'ങ'
    LetterNga,
    /// \u{d1a}: 'ച'
    LetterCa,
    /// \u{d1b}: 'ഛ'
    LetterCha,
    /// \u{d1c}: 'ജ'
    LetterJa,
    /// \u{d1d}: 'ഝ'
    LetterJha,
    /// \u{d1e}: 'ഞ'
    LetterNya,
    /// \u{d1f}: 'ട'
    LetterTta,
    /// \u{d20}: 'ഠ'
    LetterTtha,
    /// \u{d21}: 'ഡ'
    LetterDda,
    /// \u{d22}: 'ഢ'
    LetterDdha,
    /// \u{d23}: 'ണ'
    LetterNna,
    /// \u{d24}: 'ത'
    LetterTa,
    /// \u{d25}: 'ഥ'
    LetterTha,
    /// \u{d26}: 'ദ'
    LetterDa,
    /// \u{d27}: 'ധ'
    LetterDha,
    /// \u{d28}: 'ന'
    LetterNa,
    /// \u{d29}: 'ഩ'
    LetterNnna,
    /// \u{d2a}: 'പ'
    LetterPa,
    /// \u{d2b}: 'ഫ'
    LetterPha,
    /// \u{d2c}: 'ബ'
    LetterBa,
    /// \u{d2d}: 'ഭ'
    LetterBha,
    /// \u{d2e}: 'മ'
    LetterMa,
    /// \u{d2f}: 'യ'
    LetterYa,
    /// \u{d30}: 'ര'
    LetterRa,
    /// \u{d31}: 'റ'
    LetterRra,
    /// \u{d32}: 'ല'
    LetterLa,
    /// \u{d33}: 'ള'
    LetterLla,
    /// \u{d34}: 'ഴ'
    LetterLlla,
    /// \u{d35}: 'വ'
    LetterVa,
    /// \u{d36}: 'ശ'
    LetterSha,
    /// \u{d37}: 'ഷ'
    LetterSsa,
    /// \u{d38}: 'സ'
    LetterSa,
    /// \u{d39}: 'ഹ'
    LetterHa,
    /// \u{d3a}: 'ഺ'
    LetterTtta,
    /// \u{d3b}: '഻'
    SignVerticalBarVirama,
    /// \u{d3c}: '഼'
    SignCircularVirama,
    /// \u{d3d}: 'ഽ'
    SignAvagraha,
    /// \u{d3e}: 'ാ'
    VowelSignAa,
    /// \u{d3f}: 'ി'
    VowelSignI,
    /// \u{d40}: 'ീ'
    VowelSignIi,
    /// \u{d41}: 'ു'
    VowelSignU,
    /// \u{d42}: 'ൂ'
    VowelSignUu,
    /// \u{d43}: 'ൃ'
    VowelSignVocalicR,
    /// \u{d44}: 'ൄ'
    VowelSignVocalicRr,
    /// \u{d46}: 'െ'
    VowelSignE,
    /// \u{d47}: 'േ'
    VowelSignEe,
    /// \u{d48}: 'ൈ'
    VowelSignAi,
    /// \u{d4a}: 'ൊ'
    VowelSignO,
    /// \u{d4b}: 'ോ'
    VowelSignOo,
    /// \u{d4c}: 'ൌ'
    VowelSignAu,
    /// \u{d4d}: '്'
    SignVirama,
    /// \u{d4e}: 'ൎ'
    LetterDotReph,
    /// \u{d4f}: '൏'
    SignPara,
    /// \u{d54}: 'ൔ'
    LetterChilluM,
    /// \u{d55}: 'ൕ'
    LetterChilluY,
    /// \u{d56}: 'ൖ'
    LetterChilluLll,
    /// \u{d57}: 'ൗ'
    AuLengthMark,
    /// \u{d58}: '൘'
    FractionOneOneDashHundredDashAndDashSixtieth,
    /// \u{d59}: '൙'
    FractionOneFortieth,
    /// \u{d5a}: '൚'
    FractionThreeEightieths,
    /// \u{d5b}: '൛'
    FractionOneTwentieth,
    /// \u{d5c}: '൜'
    FractionOneTenth,
    /// \u{d5d}: '൝'
    FractionThreeTwentieths,
    /// \u{d5e}: '൞'
    FractionOneFifth,
    /// \u{d5f}: 'ൟ'
    LetterArchaicIi,
    /// \u{d60}: 'ൠ'
    LetterVocalicRr,
    /// \u{d61}: 'ൡ'
    LetterVocalicLl,
    /// \u{d62}: 'ൢ'
    VowelSignVocalicL,
    /// \u{d63}: 'ൣ'
    VowelSignVocalicLl,
    /// \u{d66}: '൦'
    DigitZero,
    /// \u{d67}: '൧'
    DigitOne,
    /// \u{d68}: '൨'
    DigitTwo,
    /// \u{d69}: '൩'
    DigitThree,
    /// \u{d6a}: '൪'
    DigitFour,
    /// \u{d6b}: '൫'
    DigitFive,
    /// \u{d6c}: '൬'
    DigitSix,
    /// \u{d6d}: '൭'
    DigitSeven,
    /// \u{d6e}: '൮'
    DigitEight,
    /// \u{d6f}: '൯'
    DigitNine,
    /// \u{d70}: '൰'
    NumberTen,
    /// \u{d71}: '൱'
    NumberOneHundred,
    /// \u{d72}: '൲'
    NumberOneThousand,
    /// \u{d73}: '൳'
    FractionOneQuarter,
    /// \u{d74}: '൴'
    FractionOneHalf,
    /// \u{d75}: '൵'
    FractionThreeQuarters,
    /// \u{d76}: '൶'
    FractionOneSixteenth,
    /// \u{d77}: '൷'
    FractionOneEighth,
    /// \u{d78}: '൸'
    FractionThreeSixteenths,
    /// \u{d79}: '൹'
    DateMark,
    /// \u{d7a}: 'ൺ'
    LetterChilluNn,
    /// \u{d7b}: 'ൻ'
    LetterChilluN,
    /// \u{d7c}: 'ർ'
    LetterChilluRr,
    /// \u{d7d}: 'ൽ'
    LetterChilluL,
    /// \u{d7e}: 'ൾ'
    LetterChilluLl,
}

impl Into<char> for Malayalam {
    fn into(self) -> char {
        use constants::*;
        match self {
            Malayalam::SignCombiningAnusvaraAbove => SIGN_COMBINING_ANUSVARA_ABOVE,
            Malayalam::SignCandrabindu => SIGN_CANDRABINDU,
            Malayalam::SignAnusvara => SIGN_ANUSVARA,
            Malayalam::SignVisarga => SIGN_VISARGA,
            Malayalam::LetterA => LETTER_A,
            Malayalam::LetterAa => LETTER_AA,
            Malayalam::LetterI => LETTER_I,
            Malayalam::LetterIi => LETTER_II,
            Malayalam::LetterU => LETTER_U,
            Malayalam::LetterUu => LETTER_UU,
            Malayalam::LetterVocalicR => LETTER_VOCALIC_R,
            Malayalam::LetterVocalicL => LETTER_VOCALIC_L,
            Malayalam::LetterE => LETTER_E,
            Malayalam::LetterEe => LETTER_EE,
            Malayalam::LetterAi => LETTER_AI,
            Malayalam::LetterO => LETTER_O,
            Malayalam::LetterOo => LETTER_OO,
            Malayalam::LetterAu => LETTER_AU,
            Malayalam::LetterKa => LETTER_KA,
            Malayalam::LetterKha => LETTER_KHA,
            Malayalam::LetterGa => LETTER_GA,
            Malayalam::LetterGha => LETTER_GHA,
            Malayalam::LetterNga => LETTER_NGA,
            Malayalam::LetterCa => LETTER_CA,
            Malayalam::LetterCha => LETTER_CHA,
            Malayalam::LetterJa => LETTER_JA,
            Malayalam::LetterJha => LETTER_JHA,
            Malayalam::LetterNya => LETTER_NYA,
            Malayalam::LetterTta => LETTER_TTA,
            Malayalam::LetterTtha => LETTER_TTHA,
            Malayalam::LetterDda => LETTER_DDA,
            Malayalam::LetterDdha => LETTER_DDHA,
            Malayalam::LetterNna => LETTER_NNA,
            Malayalam::LetterTa => LETTER_TA,
            Malayalam::LetterTha => LETTER_THA,
            Malayalam::LetterDa => LETTER_DA,
            Malayalam::LetterDha => LETTER_DHA,
            Malayalam::LetterNa => LETTER_NA,
            Malayalam::LetterNnna => LETTER_NNNA,
            Malayalam::LetterPa => LETTER_PA,
            Malayalam::LetterPha => LETTER_PHA,
            Malayalam::LetterBa => LETTER_BA,
            Malayalam::LetterBha => LETTER_BHA,
            Malayalam::LetterMa => LETTER_MA,
            Malayalam::LetterYa => LETTER_YA,
            Malayalam::LetterRa => LETTER_RA,
            Malayalam::LetterRra => LETTER_RRA,
            Malayalam::LetterLa => LETTER_LA,
            Malayalam::LetterLla => LETTER_LLA,
            Malayalam::LetterLlla => LETTER_LLLA,
            Malayalam::LetterVa => LETTER_VA,
            Malayalam::LetterSha => LETTER_SHA,
            Malayalam::LetterSsa => LETTER_SSA,
            Malayalam::LetterSa => LETTER_SA,
            Malayalam::LetterHa => LETTER_HA,
            Malayalam::LetterTtta => LETTER_TTTA,
            Malayalam::SignVerticalBarVirama => SIGN_VERTICAL_BAR_VIRAMA,
            Malayalam::SignCircularVirama => SIGN_CIRCULAR_VIRAMA,
            Malayalam::SignAvagraha => SIGN_AVAGRAHA,
            Malayalam::VowelSignAa => VOWEL_SIGN_AA,
            Malayalam::VowelSignI => VOWEL_SIGN_I,
            Malayalam::VowelSignIi => VOWEL_SIGN_II,
            Malayalam::VowelSignU => VOWEL_SIGN_U,
            Malayalam::VowelSignUu => VOWEL_SIGN_UU,
            Malayalam::VowelSignVocalicR => VOWEL_SIGN_VOCALIC_R,
            Malayalam::VowelSignVocalicRr => VOWEL_SIGN_VOCALIC_RR,
            Malayalam::VowelSignE => VOWEL_SIGN_E,
            Malayalam::VowelSignEe => VOWEL_SIGN_EE,
            Malayalam::VowelSignAi => VOWEL_SIGN_AI,
            Malayalam::VowelSignO => VOWEL_SIGN_O,
            Malayalam::VowelSignOo => VOWEL_SIGN_OO,
            Malayalam::VowelSignAu => VOWEL_SIGN_AU,
            Malayalam::SignVirama => SIGN_VIRAMA,
            Malayalam::LetterDotReph => LETTER_DOT_REPH,
            Malayalam::SignPara => SIGN_PARA,
            Malayalam::LetterChilluM => LETTER_CHILLU_M,
            Malayalam::LetterChilluY => LETTER_CHILLU_Y,
            Malayalam::LetterChilluLll => LETTER_CHILLU_LLL,
            Malayalam::AuLengthMark => AU_LENGTH_MARK,
            Malayalam::FractionOneOneDashHundredDashAndDashSixtieth => FRACTION_ONE_ONE_DASH_HUNDRED_DASH_AND_DASH_SIXTIETH,
            Malayalam::FractionOneFortieth => FRACTION_ONE_FORTIETH,
            Malayalam::FractionThreeEightieths => FRACTION_THREE_EIGHTIETHS,
            Malayalam::FractionOneTwentieth => FRACTION_ONE_TWENTIETH,
            Malayalam::FractionOneTenth => FRACTION_ONE_TENTH,
            Malayalam::FractionThreeTwentieths => FRACTION_THREE_TWENTIETHS,
            Malayalam::FractionOneFifth => FRACTION_ONE_FIFTH,
            Malayalam::LetterArchaicIi => LETTER_ARCHAIC_II,
            Malayalam::LetterVocalicRr => LETTER_VOCALIC_RR,
            Malayalam::LetterVocalicLl => LETTER_VOCALIC_LL,
            Malayalam::VowelSignVocalicL => VOWEL_SIGN_VOCALIC_L,
            Malayalam::VowelSignVocalicLl => VOWEL_SIGN_VOCALIC_LL,
            Malayalam::DigitZero => DIGIT_ZERO,
            Malayalam::DigitOne => DIGIT_ONE,
            Malayalam::DigitTwo => DIGIT_TWO,
            Malayalam::DigitThree => DIGIT_THREE,
            Malayalam::DigitFour => DIGIT_FOUR,
            Malayalam::DigitFive => DIGIT_FIVE,
            Malayalam::DigitSix => DIGIT_SIX,
            Malayalam::DigitSeven => DIGIT_SEVEN,
            Malayalam::DigitEight => DIGIT_EIGHT,
            Malayalam::DigitNine => DIGIT_NINE,
            Malayalam::NumberTen => NUMBER_TEN,
            Malayalam::NumberOneHundred => NUMBER_ONE_HUNDRED,
            Malayalam::NumberOneThousand => NUMBER_ONE_THOUSAND,
            Malayalam::FractionOneQuarter => FRACTION_ONE_QUARTER,
            Malayalam::FractionOneHalf => FRACTION_ONE_HALF,
            Malayalam::FractionThreeQuarters => FRACTION_THREE_QUARTERS,
            Malayalam::FractionOneSixteenth => FRACTION_ONE_SIXTEENTH,
            Malayalam::FractionOneEighth => FRACTION_ONE_EIGHTH,
            Malayalam::FractionThreeSixteenths => FRACTION_THREE_SIXTEENTHS,
            Malayalam::DateMark => DATE_MARK,
            Malayalam::LetterChilluNn => LETTER_CHILLU_NN,
            Malayalam::LetterChilluN => LETTER_CHILLU_N,
            Malayalam::LetterChilluRr => LETTER_CHILLU_RR,
            Malayalam::LetterChilluL => LETTER_CHILLU_L,
            Malayalam::LetterChilluLl => LETTER_CHILLU_LL,
        }
    }
}

impl std::convert::TryFrom<char> for Malayalam {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            SIGN_COMBINING_ANUSVARA_ABOVE => Ok(Malayalam::SignCombiningAnusvaraAbove),
            SIGN_CANDRABINDU => Ok(Malayalam::SignCandrabindu),
            SIGN_ANUSVARA => Ok(Malayalam::SignAnusvara),
            SIGN_VISARGA => Ok(Malayalam::SignVisarga),
            LETTER_A => Ok(Malayalam::LetterA),
            LETTER_AA => Ok(Malayalam::LetterAa),
            LETTER_I => Ok(Malayalam::LetterI),
            LETTER_II => Ok(Malayalam::LetterIi),
            LETTER_U => Ok(Malayalam::LetterU),
            LETTER_UU => Ok(Malayalam::LetterUu),
            LETTER_VOCALIC_R => Ok(Malayalam::LetterVocalicR),
            LETTER_VOCALIC_L => Ok(Malayalam::LetterVocalicL),
            LETTER_E => Ok(Malayalam::LetterE),
            LETTER_EE => Ok(Malayalam::LetterEe),
            LETTER_AI => Ok(Malayalam::LetterAi),
            LETTER_O => Ok(Malayalam::LetterO),
            LETTER_OO => Ok(Malayalam::LetterOo),
            LETTER_AU => Ok(Malayalam::LetterAu),
            LETTER_KA => Ok(Malayalam::LetterKa),
            LETTER_KHA => Ok(Malayalam::LetterKha),
            LETTER_GA => Ok(Malayalam::LetterGa),
            LETTER_GHA => Ok(Malayalam::LetterGha),
            LETTER_NGA => Ok(Malayalam::LetterNga),
            LETTER_CA => Ok(Malayalam::LetterCa),
            LETTER_CHA => Ok(Malayalam::LetterCha),
            LETTER_JA => Ok(Malayalam::LetterJa),
            LETTER_JHA => Ok(Malayalam::LetterJha),
            LETTER_NYA => Ok(Malayalam::LetterNya),
            LETTER_TTA => Ok(Malayalam::LetterTta),
            LETTER_TTHA => Ok(Malayalam::LetterTtha),
            LETTER_DDA => Ok(Malayalam::LetterDda),
            LETTER_DDHA => Ok(Malayalam::LetterDdha),
            LETTER_NNA => Ok(Malayalam::LetterNna),
            LETTER_TA => Ok(Malayalam::LetterTa),
            LETTER_THA => Ok(Malayalam::LetterTha),
            LETTER_DA => Ok(Malayalam::LetterDa),
            LETTER_DHA => Ok(Malayalam::LetterDha),
            LETTER_NA => Ok(Malayalam::LetterNa),
            LETTER_NNNA => Ok(Malayalam::LetterNnna),
            LETTER_PA => Ok(Malayalam::LetterPa),
            LETTER_PHA => Ok(Malayalam::LetterPha),
            LETTER_BA => Ok(Malayalam::LetterBa),
            LETTER_BHA => Ok(Malayalam::LetterBha),
            LETTER_MA => Ok(Malayalam::LetterMa),
            LETTER_YA => Ok(Malayalam::LetterYa),
            LETTER_RA => Ok(Malayalam::LetterRa),
            LETTER_RRA => Ok(Malayalam::LetterRra),
            LETTER_LA => Ok(Malayalam::LetterLa),
            LETTER_LLA => Ok(Malayalam::LetterLla),
            LETTER_LLLA => Ok(Malayalam::LetterLlla),
            LETTER_VA => Ok(Malayalam::LetterVa),
            LETTER_SHA => Ok(Malayalam::LetterSha),
            LETTER_SSA => Ok(Malayalam::LetterSsa),
            LETTER_SA => Ok(Malayalam::LetterSa),
            LETTER_HA => Ok(Malayalam::LetterHa),
            LETTER_TTTA => Ok(Malayalam::LetterTtta),
            SIGN_VERTICAL_BAR_VIRAMA => Ok(Malayalam::SignVerticalBarVirama),
            SIGN_CIRCULAR_VIRAMA => Ok(Malayalam::SignCircularVirama),
            SIGN_AVAGRAHA => Ok(Malayalam::SignAvagraha),
            VOWEL_SIGN_AA => Ok(Malayalam::VowelSignAa),
            VOWEL_SIGN_I => Ok(Malayalam::VowelSignI),
            VOWEL_SIGN_II => Ok(Malayalam::VowelSignIi),
            VOWEL_SIGN_U => Ok(Malayalam::VowelSignU),
            VOWEL_SIGN_UU => Ok(Malayalam::VowelSignUu),
            VOWEL_SIGN_VOCALIC_R => Ok(Malayalam::VowelSignVocalicR),
            VOWEL_SIGN_VOCALIC_RR => Ok(Malayalam::VowelSignVocalicRr),
            VOWEL_SIGN_E => Ok(Malayalam::VowelSignE),
            VOWEL_SIGN_EE => Ok(Malayalam::VowelSignEe),
            VOWEL_SIGN_AI => Ok(Malayalam::VowelSignAi),
            VOWEL_SIGN_O => Ok(Malayalam::VowelSignO),
            VOWEL_SIGN_OO => Ok(Malayalam::VowelSignOo),
            VOWEL_SIGN_AU => Ok(Malayalam::VowelSignAu),
            SIGN_VIRAMA => Ok(Malayalam::SignVirama),
            LETTER_DOT_REPH => Ok(Malayalam::LetterDotReph),
            SIGN_PARA => Ok(Malayalam::SignPara),
            LETTER_CHILLU_M => Ok(Malayalam::LetterChilluM),
            LETTER_CHILLU_Y => Ok(Malayalam::LetterChilluY),
            LETTER_CHILLU_LLL => Ok(Malayalam::LetterChilluLll),
            AU_LENGTH_MARK => Ok(Malayalam::AuLengthMark),
            FRACTION_ONE_ONE_DASH_HUNDRED_DASH_AND_DASH_SIXTIETH => Ok(Malayalam::FractionOneOneDashHundredDashAndDashSixtieth),
            FRACTION_ONE_FORTIETH => Ok(Malayalam::FractionOneFortieth),
            FRACTION_THREE_EIGHTIETHS => Ok(Malayalam::FractionThreeEightieths),
            FRACTION_ONE_TWENTIETH => Ok(Malayalam::FractionOneTwentieth),
            FRACTION_ONE_TENTH => Ok(Malayalam::FractionOneTenth),
            FRACTION_THREE_TWENTIETHS => Ok(Malayalam::FractionThreeTwentieths),
            FRACTION_ONE_FIFTH => Ok(Malayalam::FractionOneFifth),
            LETTER_ARCHAIC_II => Ok(Malayalam::LetterArchaicIi),
            LETTER_VOCALIC_RR => Ok(Malayalam::LetterVocalicRr),
            LETTER_VOCALIC_LL => Ok(Malayalam::LetterVocalicLl),
            VOWEL_SIGN_VOCALIC_L => Ok(Malayalam::VowelSignVocalicL),
            VOWEL_SIGN_VOCALIC_LL => Ok(Malayalam::VowelSignVocalicLl),
            DIGIT_ZERO => Ok(Malayalam::DigitZero),
            DIGIT_ONE => Ok(Malayalam::DigitOne),
            DIGIT_TWO => Ok(Malayalam::DigitTwo),
            DIGIT_THREE => Ok(Malayalam::DigitThree),
            DIGIT_FOUR => Ok(Malayalam::DigitFour),
            DIGIT_FIVE => Ok(Malayalam::DigitFive),
            DIGIT_SIX => Ok(Malayalam::DigitSix),
            DIGIT_SEVEN => Ok(Malayalam::DigitSeven),
            DIGIT_EIGHT => Ok(Malayalam::DigitEight),
            DIGIT_NINE => Ok(Malayalam::DigitNine),
            NUMBER_TEN => Ok(Malayalam::NumberTen),
            NUMBER_ONE_HUNDRED => Ok(Malayalam::NumberOneHundred),
            NUMBER_ONE_THOUSAND => Ok(Malayalam::NumberOneThousand),
            FRACTION_ONE_QUARTER => Ok(Malayalam::FractionOneQuarter),
            FRACTION_ONE_HALF => Ok(Malayalam::FractionOneHalf),
            FRACTION_THREE_QUARTERS => Ok(Malayalam::FractionThreeQuarters),
            FRACTION_ONE_SIXTEENTH => Ok(Malayalam::FractionOneSixteenth),
            FRACTION_ONE_EIGHTH => Ok(Malayalam::FractionOneEighth),
            FRACTION_THREE_SIXTEENTHS => Ok(Malayalam::FractionThreeSixteenths),
            DATE_MARK => Ok(Malayalam::DateMark),
            LETTER_CHILLU_NN => Ok(Malayalam::LetterChilluNn),
            LETTER_CHILLU_N => Ok(Malayalam::LetterChilluN),
            LETTER_CHILLU_RR => Ok(Malayalam::LetterChilluRr),
            LETTER_CHILLU_L => Ok(Malayalam::LetterChilluL),
            LETTER_CHILLU_LL => Ok(Malayalam::LetterChilluLl),
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
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Malayalam::SignCombiningAnusvaraAbove
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Malayalam::SignCombiningAnusvaraAbove => "malayalam sign combining anusvara above",
            Malayalam::SignCandrabindu => "malayalam sign candrabindu",
            Malayalam::SignAnusvara => "malayalam sign anusvara",
            Malayalam::SignVisarga => "malayalam sign visarga",
            Malayalam::LetterA => "malayalam letter a",
            Malayalam::LetterAa => "malayalam letter aa",
            Malayalam::LetterI => "malayalam letter i",
            Malayalam::LetterIi => "malayalam letter ii",
            Malayalam::LetterU => "malayalam letter u",
            Malayalam::LetterUu => "malayalam letter uu",
            Malayalam::LetterVocalicR => "malayalam letter vocalic r",
            Malayalam::LetterVocalicL => "malayalam letter vocalic l",
            Malayalam::LetterE => "malayalam letter e",
            Malayalam::LetterEe => "malayalam letter ee",
            Malayalam::LetterAi => "malayalam letter ai",
            Malayalam::LetterO => "malayalam letter o",
            Malayalam::LetterOo => "malayalam letter oo",
            Malayalam::LetterAu => "malayalam letter au",
            Malayalam::LetterKa => "malayalam letter ka",
            Malayalam::LetterKha => "malayalam letter kha",
            Malayalam::LetterGa => "malayalam letter ga",
            Malayalam::LetterGha => "malayalam letter gha",
            Malayalam::LetterNga => "malayalam letter nga",
            Malayalam::LetterCa => "malayalam letter ca",
            Malayalam::LetterCha => "malayalam letter cha",
            Malayalam::LetterJa => "malayalam letter ja",
            Malayalam::LetterJha => "malayalam letter jha",
            Malayalam::LetterNya => "malayalam letter nya",
            Malayalam::LetterTta => "malayalam letter tta",
            Malayalam::LetterTtha => "malayalam letter ttha",
            Malayalam::LetterDda => "malayalam letter dda",
            Malayalam::LetterDdha => "malayalam letter ddha",
            Malayalam::LetterNna => "malayalam letter nna",
            Malayalam::LetterTa => "malayalam letter ta",
            Malayalam::LetterTha => "malayalam letter tha",
            Malayalam::LetterDa => "malayalam letter da",
            Malayalam::LetterDha => "malayalam letter dha",
            Malayalam::LetterNa => "malayalam letter na",
            Malayalam::LetterNnna => "malayalam letter nnna",
            Malayalam::LetterPa => "malayalam letter pa",
            Malayalam::LetterPha => "malayalam letter pha",
            Malayalam::LetterBa => "malayalam letter ba",
            Malayalam::LetterBha => "malayalam letter bha",
            Malayalam::LetterMa => "malayalam letter ma",
            Malayalam::LetterYa => "malayalam letter ya",
            Malayalam::LetterRa => "malayalam letter ra",
            Malayalam::LetterRra => "malayalam letter rra",
            Malayalam::LetterLa => "malayalam letter la",
            Malayalam::LetterLla => "malayalam letter lla",
            Malayalam::LetterLlla => "malayalam letter llla",
            Malayalam::LetterVa => "malayalam letter va",
            Malayalam::LetterSha => "malayalam letter sha",
            Malayalam::LetterSsa => "malayalam letter ssa",
            Malayalam::LetterSa => "malayalam letter sa",
            Malayalam::LetterHa => "malayalam letter ha",
            Malayalam::LetterTtta => "malayalam letter ttta",
            Malayalam::SignVerticalBarVirama => "malayalam sign vertical bar virama",
            Malayalam::SignCircularVirama => "malayalam sign circular virama",
            Malayalam::SignAvagraha => "malayalam sign avagraha",
            Malayalam::VowelSignAa => "malayalam vowel sign aa",
            Malayalam::VowelSignI => "malayalam vowel sign i",
            Malayalam::VowelSignIi => "malayalam vowel sign ii",
            Malayalam::VowelSignU => "malayalam vowel sign u",
            Malayalam::VowelSignUu => "malayalam vowel sign uu",
            Malayalam::VowelSignVocalicR => "malayalam vowel sign vocalic r",
            Malayalam::VowelSignVocalicRr => "malayalam vowel sign vocalic rr",
            Malayalam::VowelSignE => "malayalam vowel sign e",
            Malayalam::VowelSignEe => "malayalam vowel sign ee",
            Malayalam::VowelSignAi => "malayalam vowel sign ai",
            Malayalam::VowelSignO => "malayalam vowel sign o",
            Malayalam::VowelSignOo => "malayalam vowel sign oo",
            Malayalam::VowelSignAu => "malayalam vowel sign au",
            Malayalam::SignVirama => "malayalam sign virama",
            Malayalam::LetterDotReph => "malayalam letter dot reph",
            Malayalam::SignPara => "malayalam sign para",
            Malayalam::LetterChilluM => "malayalam letter chillu m",
            Malayalam::LetterChilluY => "malayalam letter chillu y",
            Malayalam::LetterChilluLll => "malayalam letter chillu lll",
            Malayalam::AuLengthMark => "malayalam au length mark",
            Malayalam::FractionOneOneDashHundredDashAndDashSixtieth => "malayalam fraction one one-hundred-and-sixtieth",
            Malayalam::FractionOneFortieth => "malayalam fraction one fortieth",
            Malayalam::FractionThreeEightieths => "malayalam fraction three eightieths",
            Malayalam::FractionOneTwentieth => "malayalam fraction one twentieth",
            Malayalam::FractionOneTenth => "malayalam fraction one tenth",
            Malayalam::FractionThreeTwentieths => "malayalam fraction three twentieths",
            Malayalam::FractionOneFifth => "malayalam fraction one fifth",
            Malayalam::LetterArchaicIi => "malayalam letter archaic ii",
            Malayalam::LetterVocalicRr => "malayalam letter vocalic rr",
            Malayalam::LetterVocalicLl => "malayalam letter vocalic ll",
            Malayalam::VowelSignVocalicL => "malayalam vowel sign vocalic l",
            Malayalam::VowelSignVocalicLl => "malayalam vowel sign vocalic ll",
            Malayalam::DigitZero => "malayalam digit zero",
            Malayalam::DigitOne => "malayalam digit one",
            Malayalam::DigitTwo => "malayalam digit two",
            Malayalam::DigitThree => "malayalam digit three",
            Malayalam::DigitFour => "malayalam digit four",
            Malayalam::DigitFive => "malayalam digit five",
            Malayalam::DigitSix => "malayalam digit six",
            Malayalam::DigitSeven => "malayalam digit seven",
            Malayalam::DigitEight => "malayalam digit eight",
            Malayalam::DigitNine => "malayalam digit nine",
            Malayalam::NumberTen => "malayalam number ten",
            Malayalam::NumberOneHundred => "malayalam number one hundred",
            Malayalam::NumberOneThousand => "malayalam number one thousand",
            Malayalam::FractionOneQuarter => "malayalam fraction one quarter",
            Malayalam::FractionOneHalf => "malayalam fraction one half",
            Malayalam::FractionThreeQuarters => "malayalam fraction three quarters",
            Malayalam::FractionOneSixteenth => "malayalam fraction one sixteenth",
            Malayalam::FractionOneEighth => "malayalam fraction one eighth",
            Malayalam::FractionThreeSixteenths => "malayalam fraction three sixteenths",
            Malayalam::DateMark => "malayalam date mark",
            Malayalam::LetterChilluNn => "malayalam letter chillu nn",
            Malayalam::LetterChilluN => "malayalam letter chillu n",
            Malayalam::LetterChilluRr => "malayalam letter chillu rr",
            Malayalam::LetterChilluL => "malayalam letter chillu l",
            Malayalam::LetterChilluLl => "malayalam letter chillu ll",
        }
    }
}
