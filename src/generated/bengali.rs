/// A number of constants to give a name to all characters in this block.
mod constants {
    /// \u{980}: 'ঀ'
    pub const ANJI: char = 'ঀ';
    /// \u{981}: 'ঁ'
    pub const SIGN_CANDRABINDU: char = 'ঁ';
    /// \u{982}: 'ং'
    pub const SIGN_ANUSVARA: char = 'ং';
    /// \u{983}: 'ঃ'
    pub const SIGN_VISARGA: char = 'ঃ';
    /// \u{985}: 'অ'
    pub const LETTER_A: char = 'অ';
    /// \u{986}: 'আ'
    pub const LETTER_AA: char = 'আ';
    /// \u{987}: 'ই'
    pub const LETTER_I: char = 'ই';
    /// \u{988}: 'ঈ'
    pub const LETTER_II: char = 'ঈ';
    /// \u{989}: 'উ'
    pub const LETTER_U: char = 'উ';
    /// \u{98a}: 'ঊ'
    pub const LETTER_UU: char = 'ঊ';
    /// \u{98b}: 'ঋ'
    pub const LETTER_VOCALIC_R: char = 'ঋ';
    /// \u{98c}: 'ঌ'
    pub const LETTER_VOCALIC_L: char = 'ঌ';
    /// \u{98f}: 'এ'
    pub const LETTER_E: char = 'এ';
    /// \u{990}: 'ঐ'
    pub const LETTER_AI: char = 'ঐ';
    /// \u{993}: 'ও'
    pub const LETTER_O: char = 'ও';
    /// \u{994}: 'ঔ'
    pub const LETTER_AU: char = 'ঔ';
    /// \u{995}: 'ক'
    pub const LETTER_KA: char = 'ক';
    /// \u{996}: 'খ'
    pub const LETTER_KHA: char = 'খ';
    /// \u{997}: 'গ'
    pub const LETTER_GA: char = 'গ';
    /// \u{998}: 'ঘ'
    pub const LETTER_GHA: char = 'ঘ';
    /// \u{999}: 'ঙ'
    pub const LETTER_NGA: char = 'ঙ';
    /// \u{99a}: 'চ'
    pub const LETTER_CA: char = 'চ';
    /// \u{99b}: 'ছ'
    pub const LETTER_CHA: char = 'ছ';
    /// \u{99c}: 'জ'
    pub const LETTER_JA: char = 'জ';
    /// \u{99d}: 'ঝ'
    pub const LETTER_JHA: char = 'ঝ';
    /// \u{99e}: 'ঞ'
    pub const LETTER_NYA: char = 'ঞ';
    /// \u{99f}: 'ট'
    pub const LETTER_TTA: char = 'ট';
    /// \u{9a0}: 'ঠ'
    pub const LETTER_TTHA: char = 'ঠ';
    /// \u{9a1}: 'ড'
    pub const LETTER_DDA: char = 'ড';
    /// \u{9a2}: 'ঢ'
    pub const LETTER_DDHA: char = 'ঢ';
    /// \u{9a3}: 'ণ'
    pub const LETTER_NNA: char = 'ণ';
    /// \u{9a4}: 'ত'
    pub const LETTER_TA: char = 'ত';
    /// \u{9a5}: 'থ'
    pub const LETTER_THA: char = 'থ';
    /// \u{9a6}: 'দ'
    pub const LETTER_DA: char = 'দ';
    /// \u{9a7}: 'ধ'
    pub const LETTER_DHA: char = 'ধ';
    /// \u{9a8}: 'ন'
    pub const LETTER_NA: char = 'ন';
    /// \u{9aa}: 'প'
    pub const LETTER_PA: char = 'প';
    /// \u{9ab}: 'ফ'
    pub const LETTER_PHA: char = 'ফ';
    /// \u{9ac}: 'ব'
    pub const LETTER_BA: char = 'ব';
    /// \u{9ad}: 'ভ'
    pub const LETTER_BHA: char = 'ভ';
    /// \u{9ae}: 'ম'
    pub const LETTER_MA: char = 'ম';
    /// \u{9af}: 'য'
    pub const LETTER_YA: char = 'য';
    /// \u{9b0}: 'র'
    pub const LETTER_RA: char = 'র';
    /// \u{9b2}: 'ল'
    pub const LETTER_LA: char = 'ল';
    /// \u{9b6}: 'শ'
    pub const LETTER_SHA: char = 'শ';
    /// \u{9b7}: 'ষ'
    pub const LETTER_SSA: char = 'ষ';
    /// \u{9b8}: 'স'
    pub const LETTER_SA: char = 'স';
    /// \u{9b9}: 'হ'
    pub const LETTER_HA: char = 'হ';
    /// \u{9bc}: '়'
    pub const SIGN_NUKTA: char = '়';
    /// \u{9bd}: 'ঽ'
    pub const SIGN_AVAGRAHA: char = 'ঽ';
    /// \u{9be}: 'া'
    pub const VOWEL_SIGN_AA: char = 'া';
    /// \u{9bf}: 'ি'
    pub const VOWEL_SIGN_I: char = 'ি';
    /// \u{9c0}: 'ী'
    pub const VOWEL_SIGN_II: char = 'ী';
    /// \u{9c1}: 'ু'
    pub const VOWEL_SIGN_U: char = 'ু';
    /// \u{9c2}: 'ূ'
    pub const VOWEL_SIGN_UU: char = 'ূ';
    /// \u{9c3}: 'ৃ'
    pub const VOWEL_SIGN_VOCALIC_R: char = 'ৃ';
    /// \u{9c4}: 'ৄ'
    pub const VOWEL_SIGN_VOCALIC_RR: char = 'ৄ';
    /// \u{9c7}: 'ে'
    pub const VOWEL_SIGN_E: char = 'ে';
    /// \u{9c8}: 'ৈ'
    pub const VOWEL_SIGN_AI: char = 'ৈ';
    /// \u{9cb}: 'ো'
    pub const VOWEL_SIGN_O: char = 'ো';
    /// \u{9cc}: 'ৌ'
    pub const VOWEL_SIGN_AU: char = 'ৌ';
    /// \u{9cd}: '্'
    pub const SIGN_VIRAMA: char = '্';
    /// \u{9ce}: 'ৎ'
    pub const LETTER_KHANDA_TA: char = 'ৎ';
    /// \u{9d7}: 'ৗ'
    pub const AU_LENGTH_MARK: char = 'ৗ';
    /// \u{9dc}: 'ড়'
    pub const LETTER_RRA: char = 'ড়';
    /// \u{9dd}: 'ঢ়'
    pub const LETTER_RHA: char = 'ঢ়';
    /// \u{9df}: 'য়'
    pub const LETTER_YYA: char = 'য়';
    /// \u{9e0}: 'ৠ'
    pub const LETTER_VOCALIC_RR: char = 'ৠ';
    /// \u{9e1}: 'ৡ'
    pub const LETTER_VOCALIC_LL: char = 'ৡ';
    /// \u{9e2}: 'ৢ'
    pub const VOWEL_SIGN_VOCALIC_L: char = 'ৢ';
    /// \u{9e3}: 'ৣ'
    pub const VOWEL_SIGN_VOCALIC_LL: char = 'ৣ';
    /// \u{9e6}: '০'
    pub const DIGIT_ZERO: char = '০';
    /// \u{9e7}: '১'
    pub const DIGIT_ONE: char = '১';
    /// \u{9e8}: '২'
    pub const DIGIT_TWO: char = '২';
    /// \u{9e9}: '৩'
    pub const DIGIT_THREE: char = '৩';
    /// \u{9ea}: '৪'
    pub const DIGIT_FOUR: char = '৪';
    /// \u{9eb}: '৫'
    pub const DIGIT_FIVE: char = '৫';
    /// \u{9ec}: '৬'
    pub const DIGIT_SIX: char = '৬';
    /// \u{9ed}: '৭'
    pub const DIGIT_SEVEN: char = '৭';
    /// \u{9ee}: '৮'
    pub const DIGIT_EIGHT: char = '৮';
    /// \u{9ef}: '৯'
    pub const DIGIT_NINE: char = '৯';
    /// \u{9f0}: 'ৰ'
    pub const LETTER_RA_WITH_MIDDLE_DIAGONAL: char = 'ৰ';
    /// \u{9f1}: 'ৱ'
    pub const LETTER_RA_WITH_LOWER_DIAGONAL: char = 'ৱ';
    /// \u{9f2}: '৲'
    pub const RUPEE_MARK: char = '৲';
    /// \u{9f3}: '৳'
    pub const RUPEE_SIGN: char = '৳';
    /// \u{9f4}: '৴'
    pub const CURRENCY_NUMERATOR_ONE: char = '৴';
    /// \u{9f5}: '৵'
    pub const CURRENCY_NUMERATOR_TWO: char = '৵';
    /// \u{9f6}: '৶'
    pub const CURRENCY_NUMERATOR_THREE: char = '৶';
    /// \u{9f7}: '৷'
    pub const CURRENCY_NUMERATOR_FOUR: char = '৷';
    /// \u{9f8}: '৸'
    pub const CURRENCY_NUMERATOR_ONE_LESS_THAN_THE_DENOMINATOR: char = '৸';
    /// \u{9f9}: '৹'
    pub const CURRENCY_DENOMINATOR_SIXTEEN: char = '৹';
    /// \u{9fa}: '৺'
    pub const ISSHAR: char = '৺';
    /// \u{9fb}: '৻'
    pub const GANDA_MARK: char = '৻';
    /// \u{9fc}: 'ৼ'
    pub const LETTER_VEDIC_ANUSVARA: char = 'ৼ';
    /// \u{9fd}: '৽'
    pub const ABBREVIATION_SIGN: char = '৽';
    /// \u{9fe}: '৾'
    pub const SANDHI_MARK: char = '৾';
}

/// An enum to represent all characters in the Bengali block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Bengali {
    /// \u{980}: 'ঀ'
    Anji,
    /// \u{981}: 'ঁ'
    SignCandrabindu,
    /// \u{982}: 'ং'
    SignAnusvara,
    /// \u{983}: 'ঃ'
    SignVisarga,
    /// \u{985}: 'অ'
    LetterA,
    /// \u{986}: 'আ'
    LetterAa,
    /// \u{987}: 'ই'
    LetterI,
    /// \u{988}: 'ঈ'
    LetterIi,
    /// \u{989}: 'উ'
    LetterU,
    /// \u{98a}: 'ঊ'
    LetterUu,
    /// \u{98b}: 'ঋ'
    LetterVocalicR,
    /// \u{98c}: 'ঌ'
    LetterVocalicL,
    /// \u{98f}: 'এ'
    LetterE,
    /// \u{990}: 'ঐ'
    LetterAi,
    /// \u{993}: 'ও'
    LetterO,
    /// \u{994}: 'ঔ'
    LetterAu,
    /// \u{995}: 'ক'
    LetterKa,
    /// \u{996}: 'খ'
    LetterKha,
    /// \u{997}: 'গ'
    LetterGa,
    /// \u{998}: 'ঘ'
    LetterGha,
    /// \u{999}: 'ঙ'
    LetterNga,
    /// \u{99a}: 'চ'
    LetterCa,
    /// \u{99b}: 'ছ'
    LetterCha,
    /// \u{99c}: 'জ'
    LetterJa,
    /// \u{99d}: 'ঝ'
    LetterJha,
    /// \u{99e}: 'ঞ'
    LetterNya,
    /// \u{99f}: 'ট'
    LetterTta,
    /// \u{9a0}: 'ঠ'
    LetterTtha,
    /// \u{9a1}: 'ড'
    LetterDda,
    /// \u{9a2}: 'ঢ'
    LetterDdha,
    /// \u{9a3}: 'ণ'
    LetterNna,
    /// \u{9a4}: 'ত'
    LetterTa,
    /// \u{9a5}: 'থ'
    LetterTha,
    /// \u{9a6}: 'দ'
    LetterDa,
    /// \u{9a7}: 'ধ'
    LetterDha,
    /// \u{9a8}: 'ন'
    LetterNa,
    /// \u{9aa}: 'প'
    LetterPa,
    /// \u{9ab}: 'ফ'
    LetterPha,
    /// \u{9ac}: 'ব'
    LetterBa,
    /// \u{9ad}: 'ভ'
    LetterBha,
    /// \u{9ae}: 'ম'
    LetterMa,
    /// \u{9af}: 'য'
    LetterYa,
    /// \u{9b0}: 'র'
    LetterRa,
    /// \u{9b2}: 'ল'
    LetterLa,
    /// \u{9b6}: 'শ'
    LetterSha,
    /// \u{9b7}: 'ষ'
    LetterSsa,
    /// \u{9b8}: 'স'
    LetterSa,
    /// \u{9b9}: 'হ'
    LetterHa,
    /// \u{9bc}: '়'
    SignNukta,
    /// \u{9bd}: 'ঽ'
    SignAvagraha,
    /// \u{9be}: 'া'
    VowelSignAa,
    /// \u{9bf}: 'ি'
    VowelSignI,
    /// \u{9c0}: 'ী'
    VowelSignIi,
    /// \u{9c1}: 'ু'
    VowelSignU,
    /// \u{9c2}: 'ূ'
    VowelSignUu,
    /// \u{9c3}: 'ৃ'
    VowelSignVocalicR,
    /// \u{9c4}: 'ৄ'
    VowelSignVocalicRr,
    /// \u{9c7}: 'ে'
    VowelSignE,
    /// \u{9c8}: 'ৈ'
    VowelSignAi,
    /// \u{9cb}: 'ো'
    VowelSignO,
    /// \u{9cc}: 'ৌ'
    VowelSignAu,
    /// \u{9cd}: '্'
    SignVirama,
    /// \u{9ce}: 'ৎ'
    LetterKhandaTa,
    /// \u{9d7}: 'ৗ'
    AuLengthMark,
    /// \u{9dc}: 'ড়'
    LetterRra,
    /// \u{9dd}: 'ঢ়'
    LetterRha,
    /// \u{9df}: 'য়'
    LetterYya,
    /// \u{9e0}: 'ৠ'
    LetterVocalicRr,
    /// \u{9e1}: 'ৡ'
    LetterVocalicLl,
    /// \u{9e2}: 'ৢ'
    VowelSignVocalicL,
    /// \u{9e3}: 'ৣ'
    VowelSignVocalicLl,
    /// \u{9e6}: '০'
    DigitZero,
    /// \u{9e7}: '১'
    DigitOne,
    /// \u{9e8}: '২'
    DigitTwo,
    /// \u{9e9}: '৩'
    DigitThree,
    /// \u{9ea}: '৪'
    DigitFour,
    /// \u{9eb}: '৫'
    DigitFive,
    /// \u{9ec}: '৬'
    DigitSix,
    /// \u{9ed}: '৭'
    DigitSeven,
    /// \u{9ee}: '৮'
    DigitEight,
    /// \u{9ef}: '৯'
    DigitNine,
    /// \u{9f0}: 'ৰ'
    LetterRaWithMiddleDiagonal,
    /// \u{9f1}: 'ৱ'
    LetterRaWithLowerDiagonal,
    /// \u{9f2}: '৲'
    RupeeMark,
    /// \u{9f3}: '৳'
    RupeeSign,
    /// \u{9f4}: '৴'
    CurrencyNumeratorOne,
    /// \u{9f5}: '৵'
    CurrencyNumeratorTwo,
    /// \u{9f6}: '৶'
    CurrencyNumeratorThree,
    /// \u{9f7}: '৷'
    CurrencyNumeratorFour,
    /// \u{9f8}: '৸'
    CurrencyNumeratorOneLessThanTheDenominator,
    /// \u{9f9}: '৹'
    CurrencyDenominatorSixteen,
    /// \u{9fa}: '৺'
    Isshar,
    /// \u{9fb}: '৻'
    GandaMark,
    /// \u{9fc}: 'ৼ'
    LetterVedicAnusvara,
    /// \u{9fd}: '৽'
    AbbreviationSign,
    /// \u{9fe}: '৾'
    SandhiMark,
}

impl Into<char> for Bengali {
    fn into(self) -> char {
        use constants::*;
        match self {
            Bengali::Anji => ANJI,
            Bengali::SignCandrabindu => SIGN_CANDRABINDU,
            Bengali::SignAnusvara => SIGN_ANUSVARA,
            Bengali::SignVisarga => SIGN_VISARGA,
            Bengali::LetterA => LETTER_A,
            Bengali::LetterAa => LETTER_AA,
            Bengali::LetterI => LETTER_I,
            Bengali::LetterIi => LETTER_II,
            Bengali::LetterU => LETTER_U,
            Bengali::LetterUu => LETTER_UU,
            Bengali::LetterVocalicR => LETTER_VOCALIC_R,
            Bengali::LetterVocalicL => LETTER_VOCALIC_L,
            Bengali::LetterE => LETTER_E,
            Bengali::LetterAi => LETTER_AI,
            Bengali::LetterO => LETTER_O,
            Bengali::LetterAu => LETTER_AU,
            Bengali::LetterKa => LETTER_KA,
            Bengali::LetterKha => LETTER_KHA,
            Bengali::LetterGa => LETTER_GA,
            Bengali::LetterGha => LETTER_GHA,
            Bengali::LetterNga => LETTER_NGA,
            Bengali::LetterCa => LETTER_CA,
            Bengali::LetterCha => LETTER_CHA,
            Bengali::LetterJa => LETTER_JA,
            Bengali::LetterJha => LETTER_JHA,
            Bengali::LetterNya => LETTER_NYA,
            Bengali::LetterTta => LETTER_TTA,
            Bengali::LetterTtha => LETTER_TTHA,
            Bengali::LetterDda => LETTER_DDA,
            Bengali::LetterDdha => LETTER_DDHA,
            Bengali::LetterNna => LETTER_NNA,
            Bengali::LetterTa => LETTER_TA,
            Bengali::LetterTha => LETTER_THA,
            Bengali::LetterDa => LETTER_DA,
            Bengali::LetterDha => LETTER_DHA,
            Bengali::LetterNa => LETTER_NA,
            Bengali::LetterPa => LETTER_PA,
            Bengali::LetterPha => LETTER_PHA,
            Bengali::LetterBa => LETTER_BA,
            Bengali::LetterBha => LETTER_BHA,
            Bengali::LetterMa => LETTER_MA,
            Bengali::LetterYa => LETTER_YA,
            Bengali::LetterRa => LETTER_RA,
            Bengali::LetterLa => LETTER_LA,
            Bengali::LetterSha => LETTER_SHA,
            Bengali::LetterSsa => LETTER_SSA,
            Bengali::LetterSa => LETTER_SA,
            Bengali::LetterHa => LETTER_HA,
            Bengali::SignNukta => SIGN_NUKTA,
            Bengali::SignAvagraha => SIGN_AVAGRAHA,
            Bengali::VowelSignAa => VOWEL_SIGN_AA,
            Bengali::VowelSignI => VOWEL_SIGN_I,
            Bengali::VowelSignIi => VOWEL_SIGN_II,
            Bengali::VowelSignU => VOWEL_SIGN_U,
            Bengali::VowelSignUu => VOWEL_SIGN_UU,
            Bengali::VowelSignVocalicR => VOWEL_SIGN_VOCALIC_R,
            Bengali::VowelSignVocalicRr => VOWEL_SIGN_VOCALIC_RR,
            Bengali::VowelSignE => VOWEL_SIGN_E,
            Bengali::VowelSignAi => VOWEL_SIGN_AI,
            Bengali::VowelSignO => VOWEL_SIGN_O,
            Bengali::VowelSignAu => VOWEL_SIGN_AU,
            Bengali::SignVirama => SIGN_VIRAMA,
            Bengali::LetterKhandaTa => LETTER_KHANDA_TA,
            Bengali::AuLengthMark => AU_LENGTH_MARK,
            Bengali::LetterRra => LETTER_RRA,
            Bengali::LetterRha => LETTER_RHA,
            Bengali::LetterYya => LETTER_YYA,
            Bengali::LetterVocalicRr => LETTER_VOCALIC_RR,
            Bengali::LetterVocalicLl => LETTER_VOCALIC_LL,
            Bengali::VowelSignVocalicL => VOWEL_SIGN_VOCALIC_L,
            Bengali::VowelSignVocalicLl => VOWEL_SIGN_VOCALIC_LL,
            Bengali::DigitZero => DIGIT_ZERO,
            Bengali::DigitOne => DIGIT_ONE,
            Bengali::DigitTwo => DIGIT_TWO,
            Bengali::DigitThree => DIGIT_THREE,
            Bengali::DigitFour => DIGIT_FOUR,
            Bengali::DigitFive => DIGIT_FIVE,
            Bengali::DigitSix => DIGIT_SIX,
            Bengali::DigitSeven => DIGIT_SEVEN,
            Bengali::DigitEight => DIGIT_EIGHT,
            Bengali::DigitNine => DIGIT_NINE,
            Bengali::LetterRaWithMiddleDiagonal => LETTER_RA_WITH_MIDDLE_DIAGONAL,
            Bengali::LetterRaWithLowerDiagonal => LETTER_RA_WITH_LOWER_DIAGONAL,
            Bengali::RupeeMark => RUPEE_MARK,
            Bengali::RupeeSign => RUPEE_SIGN,
            Bengali::CurrencyNumeratorOne => CURRENCY_NUMERATOR_ONE,
            Bengali::CurrencyNumeratorTwo => CURRENCY_NUMERATOR_TWO,
            Bengali::CurrencyNumeratorThree => CURRENCY_NUMERATOR_THREE,
            Bengali::CurrencyNumeratorFour => CURRENCY_NUMERATOR_FOUR,
            Bengali::CurrencyNumeratorOneLessThanTheDenominator => CURRENCY_NUMERATOR_ONE_LESS_THAN_THE_DENOMINATOR,
            Bengali::CurrencyDenominatorSixteen => CURRENCY_DENOMINATOR_SIXTEEN,
            Bengali::Isshar => ISSHAR,
            Bengali::GandaMark => GANDA_MARK,
            Bengali::LetterVedicAnusvara => LETTER_VEDIC_ANUSVARA,
            Bengali::AbbreviationSign => ABBREVIATION_SIGN,
            Bengali::SandhiMark => SANDHI_MARK,
        }
    }
}

impl std::convert::TryFrom<char> for Bengali {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            ANJI => Ok(Bengali::Anji),
            SIGN_CANDRABINDU => Ok(Bengali::SignCandrabindu),
            SIGN_ANUSVARA => Ok(Bengali::SignAnusvara),
            SIGN_VISARGA => Ok(Bengali::SignVisarga),
            LETTER_A => Ok(Bengali::LetterA),
            LETTER_AA => Ok(Bengali::LetterAa),
            LETTER_I => Ok(Bengali::LetterI),
            LETTER_II => Ok(Bengali::LetterIi),
            LETTER_U => Ok(Bengali::LetterU),
            LETTER_UU => Ok(Bengali::LetterUu),
            LETTER_VOCALIC_R => Ok(Bengali::LetterVocalicR),
            LETTER_VOCALIC_L => Ok(Bengali::LetterVocalicL),
            LETTER_E => Ok(Bengali::LetterE),
            LETTER_AI => Ok(Bengali::LetterAi),
            LETTER_O => Ok(Bengali::LetterO),
            LETTER_AU => Ok(Bengali::LetterAu),
            LETTER_KA => Ok(Bengali::LetterKa),
            LETTER_KHA => Ok(Bengali::LetterKha),
            LETTER_GA => Ok(Bengali::LetterGa),
            LETTER_GHA => Ok(Bengali::LetterGha),
            LETTER_NGA => Ok(Bengali::LetterNga),
            LETTER_CA => Ok(Bengali::LetterCa),
            LETTER_CHA => Ok(Bengali::LetterCha),
            LETTER_JA => Ok(Bengali::LetterJa),
            LETTER_JHA => Ok(Bengali::LetterJha),
            LETTER_NYA => Ok(Bengali::LetterNya),
            LETTER_TTA => Ok(Bengali::LetterTta),
            LETTER_TTHA => Ok(Bengali::LetterTtha),
            LETTER_DDA => Ok(Bengali::LetterDda),
            LETTER_DDHA => Ok(Bengali::LetterDdha),
            LETTER_NNA => Ok(Bengali::LetterNna),
            LETTER_TA => Ok(Bengali::LetterTa),
            LETTER_THA => Ok(Bengali::LetterTha),
            LETTER_DA => Ok(Bengali::LetterDa),
            LETTER_DHA => Ok(Bengali::LetterDha),
            LETTER_NA => Ok(Bengali::LetterNa),
            LETTER_PA => Ok(Bengali::LetterPa),
            LETTER_PHA => Ok(Bengali::LetterPha),
            LETTER_BA => Ok(Bengali::LetterBa),
            LETTER_BHA => Ok(Bengali::LetterBha),
            LETTER_MA => Ok(Bengali::LetterMa),
            LETTER_YA => Ok(Bengali::LetterYa),
            LETTER_RA => Ok(Bengali::LetterRa),
            LETTER_LA => Ok(Bengali::LetterLa),
            LETTER_SHA => Ok(Bengali::LetterSha),
            LETTER_SSA => Ok(Bengali::LetterSsa),
            LETTER_SA => Ok(Bengali::LetterSa),
            LETTER_HA => Ok(Bengali::LetterHa),
            SIGN_NUKTA => Ok(Bengali::SignNukta),
            SIGN_AVAGRAHA => Ok(Bengali::SignAvagraha),
            VOWEL_SIGN_AA => Ok(Bengali::VowelSignAa),
            VOWEL_SIGN_I => Ok(Bengali::VowelSignI),
            VOWEL_SIGN_II => Ok(Bengali::VowelSignIi),
            VOWEL_SIGN_U => Ok(Bengali::VowelSignU),
            VOWEL_SIGN_UU => Ok(Bengali::VowelSignUu),
            VOWEL_SIGN_VOCALIC_R => Ok(Bengali::VowelSignVocalicR),
            VOWEL_SIGN_VOCALIC_RR => Ok(Bengali::VowelSignVocalicRr),
            VOWEL_SIGN_E => Ok(Bengali::VowelSignE),
            VOWEL_SIGN_AI => Ok(Bengali::VowelSignAi),
            VOWEL_SIGN_O => Ok(Bengali::VowelSignO),
            VOWEL_SIGN_AU => Ok(Bengali::VowelSignAu),
            SIGN_VIRAMA => Ok(Bengali::SignVirama),
            LETTER_KHANDA_TA => Ok(Bengali::LetterKhandaTa),
            AU_LENGTH_MARK => Ok(Bengali::AuLengthMark),
            LETTER_RRA => Ok(Bengali::LetterRra),
            LETTER_RHA => Ok(Bengali::LetterRha),
            LETTER_YYA => Ok(Bengali::LetterYya),
            LETTER_VOCALIC_RR => Ok(Bengali::LetterVocalicRr),
            LETTER_VOCALIC_LL => Ok(Bengali::LetterVocalicLl),
            VOWEL_SIGN_VOCALIC_L => Ok(Bengali::VowelSignVocalicL),
            VOWEL_SIGN_VOCALIC_LL => Ok(Bengali::VowelSignVocalicLl),
            DIGIT_ZERO => Ok(Bengali::DigitZero),
            DIGIT_ONE => Ok(Bengali::DigitOne),
            DIGIT_TWO => Ok(Bengali::DigitTwo),
            DIGIT_THREE => Ok(Bengali::DigitThree),
            DIGIT_FOUR => Ok(Bengali::DigitFour),
            DIGIT_FIVE => Ok(Bengali::DigitFive),
            DIGIT_SIX => Ok(Bengali::DigitSix),
            DIGIT_SEVEN => Ok(Bengali::DigitSeven),
            DIGIT_EIGHT => Ok(Bengali::DigitEight),
            DIGIT_NINE => Ok(Bengali::DigitNine),
            LETTER_RA_WITH_MIDDLE_DIAGONAL => Ok(Bengali::LetterRaWithMiddleDiagonal),
            LETTER_RA_WITH_LOWER_DIAGONAL => Ok(Bengali::LetterRaWithLowerDiagonal),
            RUPEE_MARK => Ok(Bengali::RupeeMark),
            RUPEE_SIGN => Ok(Bengali::RupeeSign),
            CURRENCY_NUMERATOR_ONE => Ok(Bengali::CurrencyNumeratorOne),
            CURRENCY_NUMERATOR_TWO => Ok(Bengali::CurrencyNumeratorTwo),
            CURRENCY_NUMERATOR_THREE => Ok(Bengali::CurrencyNumeratorThree),
            CURRENCY_NUMERATOR_FOUR => Ok(Bengali::CurrencyNumeratorFour),
            CURRENCY_NUMERATOR_ONE_LESS_THAN_THE_DENOMINATOR => Ok(Bengali::CurrencyNumeratorOneLessThanTheDenominator),
            CURRENCY_DENOMINATOR_SIXTEEN => Ok(Bengali::CurrencyDenominatorSixteen),
            ISSHAR => Ok(Bengali::Isshar),
            GANDA_MARK => Ok(Bengali::GandaMark),
            LETTER_VEDIC_ANUSVARA => Ok(Bengali::LetterVedicAnusvara),
            ABBREVIATION_SIGN => Ok(Bengali::AbbreviationSign),
            SANDHI_MARK => Ok(Bengali::SandhiMark),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Bengali {
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

impl std::convert::TryFrom<u32> for Bengali {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Bengali {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Bengali {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Bengali::Anji
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Bengali::Anji => "bengali anji",
            Bengali::SignCandrabindu => "bengali sign candrabindu",
            Bengali::SignAnusvara => "bengali sign anusvara",
            Bengali::SignVisarga => "bengali sign visarga",
            Bengali::LetterA => "bengali letter a",
            Bengali::LetterAa => "bengali letter aa",
            Bengali::LetterI => "bengali letter i",
            Bengali::LetterIi => "bengali letter ii",
            Bengali::LetterU => "bengali letter u",
            Bengali::LetterUu => "bengali letter uu",
            Bengali::LetterVocalicR => "bengali letter vocalic r",
            Bengali::LetterVocalicL => "bengali letter vocalic l",
            Bengali::LetterE => "bengali letter e",
            Bengali::LetterAi => "bengali letter ai",
            Bengali::LetterO => "bengali letter o",
            Bengali::LetterAu => "bengali letter au",
            Bengali::LetterKa => "bengali letter ka",
            Bengali::LetterKha => "bengali letter kha",
            Bengali::LetterGa => "bengali letter ga",
            Bengali::LetterGha => "bengali letter gha",
            Bengali::LetterNga => "bengali letter nga",
            Bengali::LetterCa => "bengali letter ca",
            Bengali::LetterCha => "bengali letter cha",
            Bengali::LetterJa => "bengali letter ja",
            Bengali::LetterJha => "bengali letter jha",
            Bengali::LetterNya => "bengali letter nya",
            Bengali::LetterTta => "bengali letter tta",
            Bengali::LetterTtha => "bengali letter ttha",
            Bengali::LetterDda => "bengali letter dda",
            Bengali::LetterDdha => "bengali letter ddha",
            Bengali::LetterNna => "bengali letter nna",
            Bengali::LetterTa => "bengali letter ta",
            Bengali::LetterTha => "bengali letter tha",
            Bengali::LetterDa => "bengali letter da",
            Bengali::LetterDha => "bengali letter dha",
            Bengali::LetterNa => "bengali letter na",
            Bengali::LetterPa => "bengali letter pa",
            Bengali::LetterPha => "bengali letter pha",
            Bengali::LetterBa => "bengali letter ba",
            Bengali::LetterBha => "bengali letter bha",
            Bengali::LetterMa => "bengali letter ma",
            Bengali::LetterYa => "bengali letter ya",
            Bengali::LetterRa => "bengali letter ra",
            Bengali::LetterLa => "bengali letter la",
            Bengali::LetterSha => "bengali letter sha",
            Bengali::LetterSsa => "bengali letter ssa",
            Bengali::LetterSa => "bengali letter sa",
            Bengali::LetterHa => "bengali letter ha",
            Bengali::SignNukta => "bengali sign nukta",
            Bengali::SignAvagraha => "bengali sign avagraha",
            Bengali::VowelSignAa => "bengali vowel sign aa",
            Bengali::VowelSignI => "bengali vowel sign i",
            Bengali::VowelSignIi => "bengali vowel sign ii",
            Bengali::VowelSignU => "bengali vowel sign u",
            Bengali::VowelSignUu => "bengali vowel sign uu",
            Bengali::VowelSignVocalicR => "bengali vowel sign vocalic r",
            Bengali::VowelSignVocalicRr => "bengali vowel sign vocalic rr",
            Bengali::VowelSignE => "bengali vowel sign e",
            Bengali::VowelSignAi => "bengali vowel sign ai",
            Bengali::VowelSignO => "bengali vowel sign o",
            Bengali::VowelSignAu => "bengali vowel sign au",
            Bengali::SignVirama => "bengali sign virama",
            Bengali::LetterKhandaTa => "bengali letter khanda ta",
            Bengali::AuLengthMark => "bengali au length mark",
            Bengali::LetterRra => "bengali letter rra",
            Bengali::LetterRha => "bengali letter rha",
            Bengali::LetterYya => "bengali letter yya",
            Bengali::LetterVocalicRr => "bengali letter vocalic rr",
            Bengali::LetterVocalicLl => "bengali letter vocalic ll",
            Bengali::VowelSignVocalicL => "bengali vowel sign vocalic l",
            Bengali::VowelSignVocalicLl => "bengali vowel sign vocalic ll",
            Bengali::DigitZero => "bengali digit zero",
            Bengali::DigitOne => "bengali digit one",
            Bengali::DigitTwo => "bengali digit two",
            Bengali::DigitThree => "bengali digit three",
            Bengali::DigitFour => "bengali digit four",
            Bengali::DigitFive => "bengali digit five",
            Bengali::DigitSix => "bengali digit six",
            Bengali::DigitSeven => "bengali digit seven",
            Bengali::DigitEight => "bengali digit eight",
            Bengali::DigitNine => "bengali digit nine",
            Bengali::LetterRaWithMiddleDiagonal => "bengali letter ra with middle diagonal",
            Bengali::LetterRaWithLowerDiagonal => "bengali letter ra with lower diagonal",
            Bengali::RupeeMark => "bengali rupee mark",
            Bengali::RupeeSign => "bengali rupee sign",
            Bengali::CurrencyNumeratorOne => "bengali currency numerator one",
            Bengali::CurrencyNumeratorTwo => "bengali currency numerator two",
            Bengali::CurrencyNumeratorThree => "bengali currency numerator three",
            Bengali::CurrencyNumeratorFour => "bengali currency numerator four",
            Bengali::CurrencyNumeratorOneLessThanTheDenominator => "bengali currency numerator one less than the denominator",
            Bengali::CurrencyDenominatorSixteen => "bengali currency denominator sixteen",
            Bengali::Isshar => "bengali isshar",
            Bengali::GandaMark => "bengali ganda mark",
            Bengali::LetterVedicAnusvara => "bengali letter vedic anusvara",
            Bengali::AbbreviationSign => "bengali abbreviation sign",
            Bengali::SandhiMark => "bengali sandhi mark",
        }
    }
}
