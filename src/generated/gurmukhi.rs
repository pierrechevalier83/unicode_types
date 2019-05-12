/// A number of constants to give a name to all characters in this block.
mod constants {
    /// \u{a01}: 'ਁ'
    pub const SIGN_ADAK_BINDI: char = 'ਁ';
    /// \u{a02}: 'ਂ'
    pub const SIGN_BINDI: char = 'ਂ';
    /// \u{a03}: 'ਃ'
    pub const SIGN_VISARGA: char = 'ਃ';
    /// \u{a05}: 'ਅ'
    pub const LETTER_A: char = 'ਅ';
    /// \u{a06}: 'ਆ'
    pub const LETTER_AA: char = 'ਆ';
    /// \u{a07}: 'ਇ'
    pub const LETTER_I: char = 'ਇ';
    /// \u{a08}: 'ਈ'
    pub const LETTER_II: char = 'ਈ';
    /// \u{a09}: 'ਉ'
    pub const LETTER_U: char = 'ਉ';
    /// \u{a0a}: 'ਊ'
    pub const LETTER_UU: char = 'ਊ';
    /// \u{a0f}: 'ਏ'
    pub const LETTER_EE: char = 'ਏ';
    /// \u{a10}: 'ਐ'
    pub const LETTER_AI: char = 'ਐ';
    /// \u{a13}: 'ਓ'
    pub const LETTER_OO: char = 'ਓ';
    /// \u{a14}: 'ਔ'
    pub const LETTER_AU: char = 'ਔ';
    /// \u{a15}: 'ਕ'
    pub const LETTER_KA: char = 'ਕ';
    /// \u{a16}: 'ਖ'
    pub const LETTER_KHA: char = 'ਖ';
    /// \u{a17}: 'ਗ'
    pub const LETTER_GA: char = 'ਗ';
    /// \u{a18}: 'ਘ'
    pub const LETTER_GHA: char = 'ਘ';
    /// \u{a19}: 'ਙ'
    pub const LETTER_NGA: char = 'ਙ';
    /// \u{a1a}: 'ਚ'
    pub const LETTER_CA: char = 'ਚ';
    /// \u{a1b}: 'ਛ'
    pub const LETTER_CHA: char = 'ਛ';
    /// \u{a1c}: 'ਜ'
    pub const LETTER_JA: char = 'ਜ';
    /// \u{a1d}: 'ਝ'
    pub const LETTER_JHA: char = 'ਝ';
    /// \u{a1e}: 'ਞ'
    pub const LETTER_NYA: char = 'ਞ';
    /// \u{a1f}: 'ਟ'
    pub const LETTER_TTA: char = 'ਟ';
    /// \u{a20}: 'ਠ'
    pub const LETTER_TTHA: char = 'ਠ';
    /// \u{a21}: 'ਡ'
    pub const LETTER_DDA: char = 'ਡ';
    /// \u{a22}: 'ਢ'
    pub const LETTER_DDHA: char = 'ਢ';
    /// \u{a23}: 'ਣ'
    pub const LETTER_NNA: char = 'ਣ';
    /// \u{a24}: 'ਤ'
    pub const LETTER_TA: char = 'ਤ';
    /// \u{a25}: 'ਥ'
    pub const LETTER_THA: char = 'ਥ';
    /// \u{a26}: 'ਦ'
    pub const LETTER_DA: char = 'ਦ';
    /// \u{a27}: 'ਧ'
    pub const LETTER_DHA: char = 'ਧ';
    /// \u{a28}: 'ਨ'
    pub const LETTER_NA: char = 'ਨ';
    /// \u{a2a}: 'ਪ'
    pub const LETTER_PA: char = 'ਪ';
    /// \u{a2b}: 'ਫ'
    pub const LETTER_PHA: char = 'ਫ';
    /// \u{a2c}: 'ਬ'
    pub const LETTER_BA: char = 'ਬ';
    /// \u{a2d}: 'ਭ'
    pub const LETTER_BHA: char = 'ਭ';
    /// \u{a2e}: 'ਮ'
    pub const LETTER_MA: char = 'ਮ';
    /// \u{a2f}: 'ਯ'
    pub const LETTER_YA: char = 'ਯ';
    /// \u{a30}: 'ਰ'
    pub const LETTER_RA: char = 'ਰ';
    /// \u{a32}: 'ਲ'
    pub const LETTER_LA: char = 'ਲ';
    /// \u{a33}: 'ਲ਼'
    pub const LETTER_LLA: char = 'ਲ਼';
    /// \u{a35}: 'ਵ'
    pub const LETTER_VA: char = 'ਵ';
    /// \u{a36}: 'ਸ਼'
    pub const LETTER_SHA: char = 'ਸ਼';
    /// \u{a38}: 'ਸ'
    pub const LETTER_SA: char = 'ਸ';
    /// \u{a39}: 'ਹ'
    pub const LETTER_HA: char = 'ਹ';
    /// \u{a3c}: '਼'
    pub const SIGN_NUKTA: char = '਼';
    /// \u{a3e}: 'ਾ'
    pub const VOWEL_SIGN_AA: char = 'ਾ';
    /// \u{a3f}: 'ਿ'
    pub const VOWEL_SIGN_I: char = 'ਿ';
    /// \u{a40}: 'ੀ'
    pub const VOWEL_SIGN_II: char = 'ੀ';
    /// \u{a41}: 'ੁ'
    pub const VOWEL_SIGN_U: char = 'ੁ';
    /// \u{a42}: 'ੂ'
    pub const VOWEL_SIGN_UU: char = 'ੂ';
    /// \u{a47}: 'ੇ'
    pub const VOWEL_SIGN_EE: char = 'ੇ';
    /// \u{a48}: 'ੈ'
    pub const VOWEL_SIGN_AI: char = 'ੈ';
    /// \u{a4b}: 'ੋ'
    pub const VOWEL_SIGN_OO: char = 'ੋ';
    /// \u{a4c}: 'ੌ'
    pub const VOWEL_SIGN_AU: char = 'ੌ';
    /// \u{a4d}: '੍'
    pub const SIGN_VIRAMA: char = '੍';
    /// \u{a51}: 'ੑ'
    pub const SIGN_UDAAT: char = 'ੑ';
    /// \u{a59}: 'ਖ਼'
    pub const LETTER_KHHA: char = 'ਖ਼';
    /// \u{a5a}: 'ਗ਼'
    pub const LETTER_GHHA: char = 'ਗ਼';
    /// \u{a5b}: 'ਜ਼'
    pub const LETTER_ZA: char = 'ਜ਼';
    /// \u{a5c}: 'ੜ'
    pub const LETTER_RRA: char = 'ੜ';
    /// \u{a5e}: 'ਫ਼'
    pub const LETTER_FA: char = 'ਫ਼';
    /// \u{a66}: '੦'
    pub const DIGIT_ZERO: char = '੦';
    /// \u{a67}: '੧'
    pub const DIGIT_ONE: char = '੧';
    /// \u{a68}: '੨'
    pub const DIGIT_TWO: char = '੨';
    /// \u{a69}: '੩'
    pub const DIGIT_THREE: char = '੩';
    /// \u{a6a}: '੪'
    pub const DIGIT_FOUR: char = '੪';
    /// \u{a6b}: '੫'
    pub const DIGIT_FIVE: char = '੫';
    /// \u{a6c}: '੬'
    pub const DIGIT_SIX: char = '੬';
    /// \u{a6d}: '੭'
    pub const DIGIT_SEVEN: char = '੭';
    /// \u{a6e}: '੮'
    pub const DIGIT_EIGHT: char = '੮';
    /// \u{a6f}: '੯'
    pub const DIGIT_NINE: char = '੯';
    /// \u{a70}: 'ੰ'
    pub const TIPPI: char = 'ੰ';
    /// \u{a71}: 'ੱ'
    pub const ADDAK: char = 'ੱ';
    /// \u{a72}: 'ੲ'
    pub const IRI: char = 'ੲ';
    /// \u{a73}: 'ੳ'
    pub const URA: char = 'ੳ';
    /// \u{a74}: 'ੴ'
    pub const EK_ONKAR: char = 'ੴ';
    /// \u{a75}: 'ੵ'
    pub const SIGN_YAKASH: char = 'ੵ';
    /// \u{a76}: '੶'
    pub const ABBREVIATION_SIGN: char = '੶';
}

/// An enum to represent all characters in the Gurmukhi block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Gurmukhi {
    /// \u{a01}: 'ਁ'
    SignAdakBindi,
    /// \u{a02}: 'ਂ'
    SignBindi,
    /// \u{a03}: 'ਃ'
    SignVisarga,
    /// \u{a05}: 'ਅ'
    LetterA,
    /// \u{a06}: 'ਆ'
    LetterAa,
    /// \u{a07}: 'ਇ'
    LetterI,
    /// \u{a08}: 'ਈ'
    LetterIi,
    /// \u{a09}: 'ਉ'
    LetterU,
    /// \u{a0a}: 'ਊ'
    LetterUu,
    /// \u{a0f}: 'ਏ'
    LetterEe,
    /// \u{a10}: 'ਐ'
    LetterAi,
    /// \u{a13}: 'ਓ'
    LetterOo,
    /// \u{a14}: 'ਔ'
    LetterAu,
    /// \u{a15}: 'ਕ'
    LetterKa,
    /// \u{a16}: 'ਖ'
    LetterKha,
    /// \u{a17}: 'ਗ'
    LetterGa,
    /// \u{a18}: 'ਘ'
    LetterGha,
    /// \u{a19}: 'ਙ'
    LetterNga,
    /// \u{a1a}: 'ਚ'
    LetterCa,
    /// \u{a1b}: 'ਛ'
    LetterCha,
    /// \u{a1c}: 'ਜ'
    LetterJa,
    /// \u{a1d}: 'ਝ'
    LetterJha,
    /// \u{a1e}: 'ਞ'
    LetterNya,
    /// \u{a1f}: 'ਟ'
    LetterTta,
    /// \u{a20}: 'ਠ'
    LetterTtha,
    /// \u{a21}: 'ਡ'
    LetterDda,
    /// \u{a22}: 'ਢ'
    LetterDdha,
    /// \u{a23}: 'ਣ'
    LetterNna,
    /// \u{a24}: 'ਤ'
    LetterTa,
    /// \u{a25}: 'ਥ'
    LetterTha,
    /// \u{a26}: 'ਦ'
    LetterDa,
    /// \u{a27}: 'ਧ'
    LetterDha,
    /// \u{a28}: 'ਨ'
    LetterNa,
    /// \u{a2a}: 'ਪ'
    LetterPa,
    /// \u{a2b}: 'ਫ'
    LetterPha,
    /// \u{a2c}: 'ਬ'
    LetterBa,
    /// \u{a2d}: 'ਭ'
    LetterBha,
    /// \u{a2e}: 'ਮ'
    LetterMa,
    /// \u{a2f}: 'ਯ'
    LetterYa,
    /// \u{a30}: 'ਰ'
    LetterRa,
    /// \u{a32}: 'ਲ'
    LetterLa,
    /// \u{a33}: 'ਲ਼'
    LetterLla,
    /// \u{a35}: 'ਵ'
    LetterVa,
    /// \u{a36}: 'ਸ਼'
    LetterSha,
    /// \u{a38}: 'ਸ'
    LetterSa,
    /// \u{a39}: 'ਹ'
    LetterHa,
    /// \u{a3c}: '਼'
    SignNukta,
    /// \u{a3e}: 'ਾ'
    VowelSignAa,
    /// \u{a3f}: 'ਿ'
    VowelSignI,
    /// \u{a40}: 'ੀ'
    VowelSignIi,
    /// \u{a41}: 'ੁ'
    VowelSignU,
    /// \u{a42}: 'ੂ'
    VowelSignUu,
    /// \u{a47}: 'ੇ'
    VowelSignEe,
    /// \u{a48}: 'ੈ'
    VowelSignAi,
    /// \u{a4b}: 'ੋ'
    VowelSignOo,
    /// \u{a4c}: 'ੌ'
    VowelSignAu,
    /// \u{a4d}: '੍'
    SignVirama,
    /// \u{a51}: 'ੑ'
    SignUdaat,
    /// \u{a59}: 'ਖ਼'
    LetterKhha,
    /// \u{a5a}: 'ਗ਼'
    LetterGhha,
    /// \u{a5b}: 'ਜ਼'
    LetterZa,
    /// \u{a5c}: 'ੜ'
    LetterRra,
    /// \u{a5e}: 'ਫ਼'
    LetterFa,
    /// \u{a66}: '੦'
    DigitZero,
    /// \u{a67}: '੧'
    DigitOne,
    /// \u{a68}: '੨'
    DigitTwo,
    /// \u{a69}: '੩'
    DigitThree,
    /// \u{a6a}: '੪'
    DigitFour,
    /// \u{a6b}: '੫'
    DigitFive,
    /// \u{a6c}: '੬'
    DigitSix,
    /// \u{a6d}: '੭'
    DigitSeven,
    /// \u{a6e}: '੮'
    DigitEight,
    /// \u{a6f}: '੯'
    DigitNine,
    /// \u{a70}: 'ੰ'
    Tippi,
    /// \u{a71}: 'ੱ'
    Addak,
    /// \u{a72}: 'ੲ'
    Iri,
    /// \u{a73}: 'ੳ'
    Ura,
    /// \u{a74}: 'ੴ'
    EkOnkar,
    /// \u{a75}: 'ੵ'
    SignYakash,
    /// \u{a76}: '੶'
    AbbreviationSign,
}

impl Into<char> for Gurmukhi {
    fn into(self) -> char {
        use constants::*;
        match self {
            Gurmukhi::SignAdakBindi => SIGN_ADAK_BINDI,
            Gurmukhi::SignBindi => SIGN_BINDI,
            Gurmukhi::SignVisarga => SIGN_VISARGA,
            Gurmukhi::LetterA => LETTER_A,
            Gurmukhi::LetterAa => LETTER_AA,
            Gurmukhi::LetterI => LETTER_I,
            Gurmukhi::LetterIi => LETTER_II,
            Gurmukhi::LetterU => LETTER_U,
            Gurmukhi::LetterUu => LETTER_UU,
            Gurmukhi::LetterEe => LETTER_EE,
            Gurmukhi::LetterAi => LETTER_AI,
            Gurmukhi::LetterOo => LETTER_OO,
            Gurmukhi::LetterAu => LETTER_AU,
            Gurmukhi::LetterKa => LETTER_KA,
            Gurmukhi::LetterKha => LETTER_KHA,
            Gurmukhi::LetterGa => LETTER_GA,
            Gurmukhi::LetterGha => LETTER_GHA,
            Gurmukhi::LetterNga => LETTER_NGA,
            Gurmukhi::LetterCa => LETTER_CA,
            Gurmukhi::LetterCha => LETTER_CHA,
            Gurmukhi::LetterJa => LETTER_JA,
            Gurmukhi::LetterJha => LETTER_JHA,
            Gurmukhi::LetterNya => LETTER_NYA,
            Gurmukhi::LetterTta => LETTER_TTA,
            Gurmukhi::LetterTtha => LETTER_TTHA,
            Gurmukhi::LetterDda => LETTER_DDA,
            Gurmukhi::LetterDdha => LETTER_DDHA,
            Gurmukhi::LetterNna => LETTER_NNA,
            Gurmukhi::LetterTa => LETTER_TA,
            Gurmukhi::LetterTha => LETTER_THA,
            Gurmukhi::LetterDa => LETTER_DA,
            Gurmukhi::LetterDha => LETTER_DHA,
            Gurmukhi::LetterNa => LETTER_NA,
            Gurmukhi::LetterPa => LETTER_PA,
            Gurmukhi::LetterPha => LETTER_PHA,
            Gurmukhi::LetterBa => LETTER_BA,
            Gurmukhi::LetterBha => LETTER_BHA,
            Gurmukhi::LetterMa => LETTER_MA,
            Gurmukhi::LetterYa => LETTER_YA,
            Gurmukhi::LetterRa => LETTER_RA,
            Gurmukhi::LetterLa => LETTER_LA,
            Gurmukhi::LetterLla => LETTER_LLA,
            Gurmukhi::LetterVa => LETTER_VA,
            Gurmukhi::LetterSha => LETTER_SHA,
            Gurmukhi::LetterSa => LETTER_SA,
            Gurmukhi::LetterHa => LETTER_HA,
            Gurmukhi::SignNukta => SIGN_NUKTA,
            Gurmukhi::VowelSignAa => VOWEL_SIGN_AA,
            Gurmukhi::VowelSignI => VOWEL_SIGN_I,
            Gurmukhi::VowelSignIi => VOWEL_SIGN_II,
            Gurmukhi::VowelSignU => VOWEL_SIGN_U,
            Gurmukhi::VowelSignUu => VOWEL_SIGN_UU,
            Gurmukhi::VowelSignEe => VOWEL_SIGN_EE,
            Gurmukhi::VowelSignAi => VOWEL_SIGN_AI,
            Gurmukhi::VowelSignOo => VOWEL_SIGN_OO,
            Gurmukhi::VowelSignAu => VOWEL_SIGN_AU,
            Gurmukhi::SignVirama => SIGN_VIRAMA,
            Gurmukhi::SignUdaat => SIGN_UDAAT,
            Gurmukhi::LetterKhha => LETTER_KHHA,
            Gurmukhi::LetterGhha => LETTER_GHHA,
            Gurmukhi::LetterZa => LETTER_ZA,
            Gurmukhi::LetterRra => LETTER_RRA,
            Gurmukhi::LetterFa => LETTER_FA,
            Gurmukhi::DigitZero => DIGIT_ZERO,
            Gurmukhi::DigitOne => DIGIT_ONE,
            Gurmukhi::DigitTwo => DIGIT_TWO,
            Gurmukhi::DigitThree => DIGIT_THREE,
            Gurmukhi::DigitFour => DIGIT_FOUR,
            Gurmukhi::DigitFive => DIGIT_FIVE,
            Gurmukhi::DigitSix => DIGIT_SIX,
            Gurmukhi::DigitSeven => DIGIT_SEVEN,
            Gurmukhi::DigitEight => DIGIT_EIGHT,
            Gurmukhi::DigitNine => DIGIT_NINE,
            Gurmukhi::Tippi => TIPPI,
            Gurmukhi::Addak => ADDAK,
            Gurmukhi::Iri => IRI,
            Gurmukhi::Ura => URA,
            Gurmukhi::EkOnkar => EK_ONKAR,
            Gurmukhi::SignYakash => SIGN_YAKASH,
            Gurmukhi::AbbreviationSign => ABBREVIATION_SIGN,
        }
    }
}

impl std::convert::TryFrom<char> for Gurmukhi {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            SIGN_ADAK_BINDI => Ok(Gurmukhi::SignAdakBindi),
            SIGN_BINDI => Ok(Gurmukhi::SignBindi),
            SIGN_VISARGA => Ok(Gurmukhi::SignVisarga),
            LETTER_A => Ok(Gurmukhi::LetterA),
            LETTER_AA => Ok(Gurmukhi::LetterAa),
            LETTER_I => Ok(Gurmukhi::LetterI),
            LETTER_II => Ok(Gurmukhi::LetterIi),
            LETTER_U => Ok(Gurmukhi::LetterU),
            LETTER_UU => Ok(Gurmukhi::LetterUu),
            LETTER_EE => Ok(Gurmukhi::LetterEe),
            LETTER_AI => Ok(Gurmukhi::LetterAi),
            LETTER_OO => Ok(Gurmukhi::LetterOo),
            LETTER_AU => Ok(Gurmukhi::LetterAu),
            LETTER_KA => Ok(Gurmukhi::LetterKa),
            LETTER_KHA => Ok(Gurmukhi::LetterKha),
            LETTER_GA => Ok(Gurmukhi::LetterGa),
            LETTER_GHA => Ok(Gurmukhi::LetterGha),
            LETTER_NGA => Ok(Gurmukhi::LetterNga),
            LETTER_CA => Ok(Gurmukhi::LetterCa),
            LETTER_CHA => Ok(Gurmukhi::LetterCha),
            LETTER_JA => Ok(Gurmukhi::LetterJa),
            LETTER_JHA => Ok(Gurmukhi::LetterJha),
            LETTER_NYA => Ok(Gurmukhi::LetterNya),
            LETTER_TTA => Ok(Gurmukhi::LetterTta),
            LETTER_TTHA => Ok(Gurmukhi::LetterTtha),
            LETTER_DDA => Ok(Gurmukhi::LetterDda),
            LETTER_DDHA => Ok(Gurmukhi::LetterDdha),
            LETTER_NNA => Ok(Gurmukhi::LetterNna),
            LETTER_TA => Ok(Gurmukhi::LetterTa),
            LETTER_THA => Ok(Gurmukhi::LetterTha),
            LETTER_DA => Ok(Gurmukhi::LetterDa),
            LETTER_DHA => Ok(Gurmukhi::LetterDha),
            LETTER_NA => Ok(Gurmukhi::LetterNa),
            LETTER_PA => Ok(Gurmukhi::LetterPa),
            LETTER_PHA => Ok(Gurmukhi::LetterPha),
            LETTER_BA => Ok(Gurmukhi::LetterBa),
            LETTER_BHA => Ok(Gurmukhi::LetterBha),
            LETTER_MA => Ok(Gurmukhi::LetterMa),
            LETTER_YA => Ok(Gurmukhi::LetterYa),
            LETTER_RA => Ok(Gurmukhi::LetterRa),
            LETTER_LA => Ok(Gurmukhi::LetterLa),
            LETTER_LLA => Ok(Gurmukhi::LetterLla),
            LETTER_VA => Ok(Gurmukhi::LetterVa),
            LETTER_SHA => Ok(Gurmukhi::LetterSha),
            LETTER_SA => Ok(Gurmukhi::LetterSa),
            LETTER_HA => Ok(Gurmukhi::LetterHa),
            SIGN_NUKTA => Ok(Gurmukhi::SignNukta),
            VOWEL_SIGN_AA => Ok(Gurmukhi::VowelSignAa),
            VOWEL_SIGN_I => Ok(Gurmukhi::VowelSignI),
            VOWEL_SIGN_II => Ok(Gurmukhi::VowelSignIi),
            VOWEL_SIGN_U => Ok(Gurmukhi::VowelSignU),
            VOWEL_SIGN_UU => Ok(Gurmukhi::VowelSignUu),
            VOWEL_SIGN_EE => Ok(Gurmukhi::VowelSignEe),
            VOWEL_SIGN_AI => Ok(Gurmukhi::VowelSignAi),
            VOWEL_SIGN_OO => Ok(Gurmukhi::VowelSignOo),
            VOWEL_SIGN_AU => Ok(Gurmukhi::VowelSignAu),
            SIGN_VIRAMA => Ok(Gurmukhi::SignVirama),
            SIGN_UDAAT => Ok(Gurmukhi::SignUdaat),
            LETTER_KHHA => Ok(Gurmukhi::LetterKhha),
            LETTER_GHHA => Ok(Gurmukhi::LetterGhha),
            LETTER_ZA => Ok(Gurmukhi::LetterZa),
            LETTER_RRA => Ok(Gurmukhi::LetterRra),
            LETTER_FA => Ok(Gurmukhi::LetterFa),
            DIGIT_ZERO => Ok(Gurmukhi::DigitZero),
            DIGIT_ONE => Ok(Gurmukhi::DigitOne),
            DIGIT_TWO => Ok(Gurmukhi::DigitTwo),
            DIGIT_THREE => Ok(Gurmukhi::DigitThree),
            DIGIT_FOUR => Ok(Gurmukhi::DigitFour),
            DIGIT_FIVE => Ok(Gurmukhi::DigitFive),
            DIGIT_SIX => Ok(Gurmukhi::DigitSix),
            DIGIT_SEVEN => Ok(Gurmukhi::DigitSeven),
            DIGIT_EIGHT => Ok(Gurmukhi::DigitEight),
            DIGIT_NINE => Ok(Gurmukhi::DigitNine),
            TIPPI => Ok(Gurmukhi::Tippi),
            ADDAK => Ok(Gurmukhi::Addak),
            IRI => Ok(Gurmukhi::Iri),
            URA => Ok(Gurmukhi::Ura),
            EK_ONKAR => Ok(Gurmukhi::EkOnkar),
            SIGN_YAKASH => Ok(Gurmukhi::SignYakash),
            ABBREVIATION_SIGN => Ok(Gurmukhi::AbbreviationSign),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Gurmukhi {
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

impl std::convert::TryFrom<u32> for Gurmukhi {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Gurmukhi {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Gurmukhi {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Gurmukhi::SignAdakBindi
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Gurmukhi::SignAdakBindi => "gurmukhi sign adak bindi",
            Gurmukhi::SignBindi => "gurmukhi sign bindi",
            Gurmukhi::SignVisarga => "gurmukhi sign visarga",
            Gurmukhi::LetterA => "gurmukhi letter a",
            Gurmukhi::LetterAa => "gurmukhi letter aa",
            Gurmukhi::LetterI => "gurmukhi letter i",
            Gurmukhi::LetterIi => "gurmukhi letter ii",
            Gurmukhi::LetterU => "gurmukhi letter u",
            Gurmukhi::LetterUu => "gurmukhi letter uu",
            Gurmukhi::LetterEe => "gurmukhi letter ee",
            Gurmukhi::LetterAi => "gurmukhi letter ai",
            Gurmukhi::LetterOo => "gurmukhi letter oo",
            Gurmukhi::LetterAu => "gurmukhi letter au",
            Gurmukhi::LetterKa => "gurmukhi letter ka",
            Gurmukhi::LetterKha => "gurmukhi letter kha",
            Gurmukhi::LetterGa => "gurmukhi letter ga",
            Gurmukhi::LetterGha => "gurmukhi letter gha",
            Gurmukhi::LetterNga => "gurmukhi letter nga",
            Gurmukhi::LetterCa => "gurmukhi letter ca",
            Gurmukhi::LetterCha => "gurmukhi letter cha",
            Gurmukhi::LetterJa => "gurmukhi letter ja",
            Gurmukhi::LetterJha => "gurmukhi letter jha",
            Gurmukhi::LetterNya => "gurmukhi letter nya",
            Gurmukhi::LetterTta => "gurmukhi letter tta",
            Gurmukhi::LetterTtha => "gurmukhi letter ttha",
            Gurmukhi::LetterDda => "gurmukhi letter dda",
            Gurmukhi::LetterDdha => "gurmukhi letter ddha",
            Gurmukhi::LetterNna => "gurmukhi letter nna",
            Gurmukhi::LetterTa => "gurmukhi letter ta",
            Gurmukhi::LetterTha => "gurmukhi letter tha",
            Gurmukhi::LetterDa => "gurmukhi letter da",
            Gurmukhi::LetterDha => "gurmukhi letter dha",
            Gurmukhi::LetterNa => "gurmukhi letter na",
            Gurmukhi::LetterPa => "gurmukhi letter pa",
            Gurmukhi::LetterPha => "gurmukhi letter pha",
            Gurmukhi::LetterBa => "gurmukhi letter ba",
            Gurmukhi::LetterBha => "gurmukhi letter bha",
            Gurmukhi::LetterMa => "gurmukhi letter ma",
            Gurmukhi::LetterYa => "gurmukhi letter ya",
            Gurmukhi::LetterRa => "gurmukhi letter ra",
            Gurmukhi::LetterLa => "gurmukhi letter la",
            Gurmukhi::LetterLla => "gurmukhi letter lla",
            Gurmukhi::LetterVa => "gurmukhi letter va",
            Gurmukhi::LetterSha => "gurmukhi letter sha",
            Gurmukhi::LetterSa => "gurmukhi letter sa",
            Gurmukhi::LetterHa => "gurmukhi letter ha",
            Gurmukhi::SignNukta => "gurmukhi sign nukta",
            Gurmukhi::VowelSignAa => "gurmukhi vowel sign aa",
            Gurmukhi::VowelSignI => "gurmukhi vowel sign i",
            Gurmukhi::VowelSignIi => "gurmukhi vowel sign ii",
            Gurmukhi::VowelSignU => "gurmukhi vowel sign u",
            Gurmukhi::VowelSignUu => "gurmukhi vowel sign uu",
            Gurmukhi::VowelSignEe => "gurmukhi vowel sign ee",
            Gurmukhi::VowelSignAi => "gurmukhi vowel sign ai",
            Gurmukhi::VowelSignOo => "gurmukhi vowel sign oo",
            Gurmukhi::VowelSignAu => "gurmukhi vowel sign au",
            Gurmukhi::SignVirama => "gurmukhi sign virama",
            Gurmukhi::SignUdaat => "gurmukhi sign udaat",
            Gurmukhi::LetterKhha => "gurmukhi letter khha",
            Gurmukhi::LetterGhha => "gurmukhi letter ghha",
            Gurmukhi::LetterZa => "gurmukhi letter za",
            Gurmukhi::LetterRra => "gurmukhi letter rra",
            Gurmukhi::LetterFa => "gurmukhi letter fa",
            Gurmukhi::DigitZero => "gurmukhi digit zero",
            Gurmukhi::DigitOne => "gurmukhi digit one",
            Gurmukhi::DigitTwo => "gurmukhi digit two",
            Gurmukhi::DigitThree => "gurmukhi digit three",
            Gurmukhi::DigitFour => "gurmukhi digit four",
            Gurmukhi::DigitFive => "gurmukhi digit five",
            Gurmukhi::DigitSix => "gurmukhi digit six",
            Gurmukhi::DigitSeven => "gurmukhi digit seven",
            Gurmukhi::DigitEight => "gurmukhi digit eight",
            Gurmukhi::DigitNine => "gurmukhi digit nine",
            Gurmukhi::Tippi => "gurmukhi tippi",
            Gurmukhi::Addak => "gurmukhi addak",
            Gurmukhi::Iri => "gurmukhi iri",
            Gurmukhi::Ura => "gurmukhi ura",
            Gurmukhi::EkOnkar => "gurmukhi ek onkar",
            Gurmukhi::SignYakash => "gurmukhi sign yakash",
            Gurmukhi::AbbreviationSign => "gurmukhi abbreviation sign",
        }
    }
}
