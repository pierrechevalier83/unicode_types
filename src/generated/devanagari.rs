/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{900}: 'ऀ'
    pub const SIGN_INVERTED_CANDRABINDU: char = 'ऀ';
    /// \u{901}: 'ँ'
    pub const SIGN_CANDRABINDU: char = 'ँ';
    /// \u{902}: 'ं'
    pub const SIGN_ANUSVARA: char = 'ं';
    /// \u{903}: 'ः'
    pub const SIGN_VISARGA: char = 'ः';
    /// \u{904}: 'ऄ'
    pub const LETTER_SHORT_A: char = 'ऄ';
    /// \u{905}: 'अ'
    pub const LETTER_A: char = 'अ';
    /// \u{906}: 'आ'
    pub const LETTER_AA: char = 'आ';
    /// \u{907}: 'इ'
    pub const LETTER_I: char = 'इ';
    /// \u{908}: 'ई'
    pub const LETTER_II: char = 'ई';
    /// \u{909}: 'उ'
    pub const LETTER_U: char = 'उ';
    /// \u{90a}: 'ऊ'
    pub const LETTER_UU: char = 'ऊ';
    /// \u{90b}: 'ऋ'
    pub const LETTER_VOCALIC_R: char = 'ऋ';
    /// \u{90c}: 'ऌ'
    pub const LETTER_VOCALIC_L: char = 'ऌ';
    /// \u{90d}: 'ऍ'
    pub const LETTER_CANDRA_E: char = 'ऍ';
    /// \u{90e}: 'ऎ'
    pub const LETTER_SHORT_E: char = 'ऎ';
    /// \u{90f}: 'ए'
    pub const LETTER_E: char = 'ए';
    /// \u{910}: 'ऐ'
    pub const LETTER_AI: char = 'ऐ';
    /// \u{911}: 'ऑ'
    pub const LETTER_CANDRA_O: char = 'ऑ';
    /// \u{912}: 'ऒ'
    pub const LETTER_SHORT_O: char = 'ऒ';
    /// \u{913}: 'ओ'
    pub const LETTER_O: char = 'ओ';
    /// \u{914}: 'औ'
    pub const LETTER_AU: char = 'औ';
    /// \u{915}: 'क'
    pub const LETTER_KA: char = 'क';
    /// \u{916}: 'ख'
    pub const LETTER_KHA: char = 'ख';
    /// \u{917}: 'ग'
    pub const LETTER_GA: char = 'ग';
    /// \u{918}: 'घ'
    pub const LETTER_GHA: char = 'घ';
    /// \u{919}: 'ङ'
    pub const LETTER_NGA: char = 'ङ';
    /// \u{91a}: 'च'
    pub const LETTER_CA: char = 'च';
    /// \u{91b}: 'छ'
    pub const LETTER_CHA: char = 'छ';
    /// \u{91c}: 'ज'
    pub const LETTER_JA: char = 'ज';
    /// \u{91d}: 'झ'
    pub const LETTER_JHA: char = 'झ';
    /// \u{91e}: 'ञ'
    pub const LETTER_NYA: char = 'ञ';
    /// \u{91f}: 'ट'
    pub const LETTER_TTA: char = 'ट';
    /// \u{920}: 'ठ'
    pub const LETTER_TTHA: char = 'ठ';
    /// \u{921}: 'ड'
    pub const LETTER_DDA: char = 'ड';
    /// \u{922}: 'ढ'
    pub const LETTER_DDHA: char = 'ढ';
    /// \u{923}: 'ण'
    pub const LETTER_NNA: char = 'ण';
    /// \u{924}: 'त'
    pub const LETTER_TA: char = 'त';
    /// \u{925}: 'थ'
    pub const LETTER_THA: char = 'थ';
    /// \u{926}: 'द'
    pub const LETTER_DA: char = 'द';
    /// \u{927}: 'ध'
    pub const LETTER_DHA: char = 'ध';
    /// \u{928}: 'न'
    pub const LETTER_NA: char = 'न';
    /// \u{929}: 'ऩ'
    pub const LETTER_NNNA: char = 'ऩ';
    /// \u{92a}: 'प'
    pub const LETTER_PA: char = 'प';
    /// \u{92b}: 'फ'
    pub const LETTER_PHA: char = 'फ';
    /// \u{92c}: 'ब'
    pub const LETTER_BA: char = 'ब';
    /// \u{92d}: 'भ'
    pub const LETTER_BHA: char = 'भ';
    /// \u{92e}: 'म'
    pub const LETTER_MA: char = 'म';
    /// \u{92f}: 'य'
    pub const LETTER_YA: char = 'य';
    /// \u{930}: 'र'
    pub const LETTER_RA: char = 'र';
    /// \u{931}: 'ऱ'
    pub const LETTER_RRA: char = 'ऱ';
    /// \u{932}: 'ल'
    pub const LETTER_LA: char = 'ल';
    /// \u{933}: 'ळ'
    pub const LETTER_LLA: char = 'ळ';
    /// \u{934}: 'ऴ'
    pub const LETTER_LLLA: char = 'ऴ';
    /// \u{935}: 'व'
    pub const LETTER_VA: char = 'व';
    /// \u{936}: 'श'
    pub const LETTER_SHA: char = 'श';
    /// \u{937}: 'ष'
    pub const LETTER_SSA: char = 'ष';
    /// \u{938}: 'स'
    pub const LETTER_SA: char = 'स';
    /// \u{939}: 'ह'
    pub const LETTER_HA: char = 'ह';
    /// \u{93a}: 'ऺ'
    pub const VOWEL_SIGN_OE: char = 'ऺ';
    /// \u{93b}: 'ऻ'
    pub const VOWEL_SIGN_OOE: char = 'ऻ';
    /// \u{93c}: '़'
    pub const SIGN_NUKTA: char = '़';
    /// \u{93d}: 'ऽ'
    pub const SIGN_AVAGRAHA: char = 'ऽ';
    /// \u{93e}: 'ा'
    pub const VOWEL_SIGN_AA: char = 'ा';
    /// \u{93f}: 'ि'
    pub const VOWEL_SIGN_I: char = 'ि';
    /// \u{940}: 'ी'
    pub const VOWEL_SIGN_II: char = 'ी';
    /// \u{941}: 'ु'
    pub const VOWEL_SIGN_U: char = 'ु';
    /// \u{942}: 'ू'
    pub const VOWEL_SIGN_UU: char = 'ू';
    /// \u{943}: 'ृ'
    pub const VOWEL_SIGN_VOCALIC_R: char = 'ृ';
    /// \u{944}: 'ॄ'
    pub const VOWEL_SIGN_VOCALIC_RR: char = 'ॄ';
    /// \u{945}: 'ॅ'
    pub const VOWEL_SIGN_CANDRA_E: char = 'ॅ';
    /// \u{946}: 'ॆ'
    pub const VOWEL_SIGN_SHORT_E: char = 'ॆ';
    /// \u{947}: 'े'
    pub const VOWEL_SIGN_E: char = 'े';
    /// \u{948}: 'ै'
    pub const VOWEL_SIGN_AI: char = 'ै';
    /// \u{949}: 'ॉ'
    pub const VOWEL_SIGN_CANDRA_O: char = 'ॉ';
    /// \u{94a}: 'ॊ'
    pub const VOWEL_SIGN_SHORT_O: char = 'ॊ';
    /// \u{94b}: 'ो'
    pub const VOWEL_SIGN_O: char = 'ो';
    /// \u{94c}: 'ौ'
    pub const VOWEL_SIGN_AU: char = 'ौ';
    /// \u{94d}: '्'
    pub const SIGN_VIRAMA: char = '्';
    /// \u{94e}: 'ॎ'
    pub const VOWEL_SIGN_PRISHTHAMATRA_E: char = 'ॎ';
    /// \u{94f}: 'ॏ'
    pub const VOWEL_SIGN_AW: char = 'ॏ';
    /// \u{950}: 'ॐ'
    pub const OM: char = 'ॐ';
    /// \u{951}: '॑'
    pub const STRESS_SIGN_UDATTA: char = '॑';
    /// \u{952}: '॒'
    pub const STRESS_SIGN_ANUDATTA: char = '॒';
    /// \u{953}: '॓'
    pub const GRAVE_ACCENT: char = '॓';
    /// \u{954}: '॔'
    pub const ACUTE_ACCENT: char = '॔';
    /// \u{955}: 'ॕ'
    pub const VOWEL_SIGN_CANDRA_LONG_E: char = 'ॕ';
    /// \u{956}: 'ॖ'
    pub const VOWEL_SIGN_UE: char = 'ॖ';
    /// \u{957}: 'ॗ'
    pub const VOWEL_SIGN_UUE: char = 'ॗ';
    /// \u{958}: 'क़'
    pub const LETTER_QA: char = 'क़';
    /// \u{959}: 'ख़'
    pub const LETTER_KHHA: char = 'ख़';
    /// \u{95a}: 'ग़'
    pub const LETTER_GHHA: char = 'ग़';
    /// \u{95b}: 'ज़'
    pub const LETTER_ZA: char = 'ज़';
    /// \u{95c}: 'ड़'
    pub const LETTER_DDDHA: char = 'ड़';
    /// \u{95d}: 'ढ़'
    pub const LETTER_RHA: char = 'ढ़';
    /// \u{95e}: 'फ़'
    pub const LETTER_FA: char = 'फ़';
    /// \u{95f}: 'य़'
    pub const LETTER_YYA: char = 'य़';
    /// \u{960}: 'ॠ'
    pub const LETTER_VOCALIC_RR: char = 'ॠ';
    /// \u{961}: 'ॡ'
    pub const LETTER_VOCALIC_LL: char = 'ॡ';
    /// \u{962}: 'ॢ'
    pub const VOWEL_SIGN_VOCALIC_L: char = 'ॢ';
    /// \u{963}: 'ॣ'
    pub const VOWEL_SIGN_VOCALIC_LL: char = 'ॣ';
    /// \u{964}: '।'
    pub const DANDA: char = '।';
    /// \u{965}: '॥'
    pub const DOUBLE_DANDA: char = '॥';
    /// \u{966}: '०'
    pub const DIGIT_ZERO: char = '०';
    /// \u{967}: '१'
    pub const DIGIT_ONE: char = '१';
    /// \u{968}: '२'
    pub const DIGIT_TWO: char = '२';
    /// \u{969}: '३'
    pub const DIGIT_THREE: char = '३';
    /// \u{96a}: '४'
    pub const DIGIT_FOUR: char = '४';
    /// \u{96b}: '५'
    pub const DIGIT_FIVE: char = '५';
    /// \u{96c}: '६'
    pub const DIGIT_SIX: char = '६';
    /// \u{96d}: '७'
    pub const DIGIT_SEVEN: char = '७';
    /// \u{96e}: '८'
    pub const DIGIT_EIGHT: char = '८';
    /// \u{96f}: '९'
    pub const DIGIT_NINE: char = '९';
    /// \u{970}: '॰'
    pub const ABBREVIATION_SIGN: char = '॰';
    /// \u{971}: 'ॱ'
    pub const SIGN_HIGH_SPACING_DOT: char = 'ॱ';
    /// \u{972}: 'ॲ'
    pub const LETTER_CANDRA_A: char = 'ॲ';
    /// \u{973}: 'ॳ'
    pub const LETTER_OE: char = 'ॳ';
    /// \u{974}: 'ॴ'
    pub const LETTER_OOE: char = 'ॴ';
    /// \u{975}: 'ॵ'
    pub const LETTER_AW: char = 'ॵ';
    /// \u{976}: 'ॶ'
    pub const LETTER_UE: char = 'ॶ';
    /// \u{977}: 'ॷ'
    pub const LETTER_UUE: char = 'ॷ';
    /// \u{978}: 'ॸ'
    pub const LETTER_MARWARI_DDA: char = 'ॸ';
    /// \u{979}: 'ॹ'
    pub const LETTER_ZHA: char = 'ॹ';
    /// \u{97a}: 'ॺ'
    pub const LETTER_HEAVY_YA: char = 'ॺ';
    /// \u{97b}: 'ॻ'
    pub const LETTER_GGA: char = 'ॻ';
    /// \u{97c}: 'ॼ'
    pub const LETTER_JJA: char = 'ॼ';
    /// \u{97d}: 'ॽ'
    pub const LETTER_GLOTTAL_STOP: char = 'ॽ';
    /// \u{97e}: 'ॾ'
    pub const LETTER_DDDA: char = 'ॾ';
}

/// An enum to represent all characters in the Devanagari block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Devanagari {
    /// \u{900}: 'ऀ'
    SignInvertedCandrabindu,
    /// \u{901}: 'ँ'
    SignCandrabindu,
    /// \u{902}: 'ं'
    SignAnusvara,
    /// \u{903}: 'ः'
    SignVisarga,
    /// \u{904}: 'ऄ'
    LetterShortA,
    /// \u{905}: 'अ'
    LetterA,
    /// \u{906}: 'आ'
    LetterAa,
    /// \u{907}: 'इ'
    LetterI,
    /// \u{908}: 'ई'
    LetterIi,
    /// \u{909}: 'उ'
    LetterU,
    /// \u{90a}: 'ऊ'
    LetterUu,
    /// \u{90b}: 'ऋ'
    LetterVocalicR,
    /// \u{90c}: 'ऌ'
    LetterVocalicL,
    /// \u{90d}: 'ऍ'
    LetterCandraE,
    /// \u{90e}: 'ऎ'
    LetterShortE,
    /// \u{90f}: 'ए'
    LetterE,
    /// \u{910}: 'ऐ'
    LetterAi,
    /// \u{911}: 'ऑ'
    LetterCandraO,
    /// \u{912}: 'ऒ'
    LetterShortO,
    /// \u{913}: 'ओ'
    LetterO,
    /// \u{914}: 'औ'
    LetterAu,
    /// \u{915}: 'क'
    LetterKa,
    /// \u{916}: 'ख'
    LetterKha,
    /// \u{917}: 'ग'
    LetterGa,
    /// \u{918}: 'घ'
    LetterGha,
    /// \u{919}: 'ङ'
    LetterNga,
    /// \u{91a}: 'च'
    LetterCa,
    /// \u{91b}: 'छ'
    LetterCha,
    /// \u{91c}: 'ज'
    LetterJa,
    /// \u{91d}: 'झ'
    LetterJha,
    /// \u{91e}: 'ञ'
    LetterNya,
    /// \u{91f}: 'ट'
    LetterTta,
    /// \u{920}: 'ठ'
    LetterTtha,
    /// \u{921}: 'ड'
    LetterDda,
    /// \u{922}: 'ढ'
    LetterDdha,
    /// \u{923}: 'ण'
    LetterNna,
    /// \u{924}: 'त'
    LetterTa,
    /// \u{925}: 'थ'
    LetterTha,
    /// \u{926}: 'द'
    LetterDa,
    /// \u{927}: 'ध'
    LetterDha,
    /// \u{928}: 'न'
    LetterNa,
    /// \u{929}: 'ऩ'
    LetterNnna,
    /// \u{92a}: 'प'
    LetterPa,
    /// \u{92b}: 'फ'
    LetterPha,
    /// \u{92c}: 'ब'
    LetterBa,
    /// \u{92d}: 'भ'
    LetterBha,
    /// \u{92e}: 'म'
    LetterMa,
    /// \u{92f}: 'य'
    LetterYa,
    /// \u{930}: 'र'
    LetterRa,
    /// \u{931}: 'ऱ'
    LetterRra,
    /// \u{932}: 'ल'
    LetterLa,
    /// \u{933}: 'ळ'
    LetterLla,
    /// \u{934}: 'ऴ'
    LetterLlla,
    /// \u{935}: 'व'
    LetterVa,
    /// \u{936}: 'श'
    LetterSha,
    /// \u{937}: 'ष'
    LetterSsa,
    /// \u{938}: 'स'
    LetterSa,
    /// \u{939}: 'ह'
    LetterHa,
    /// \u{93a}: 'ऺ'
    VowelSignOe,
    /// \u{93b}: 'ऻ'
    VowelSignOoe,
    /// \u{93c}: '़'
    SignNukta,
    /// \u{93d}: 'ऽ'
    SignAvagraha,
    /// \u{93e}: 'ा'
    VowelSignAa,
    /// \u{93f}: 'ि'
    VowelSignI,
    /// \u{940}: 'ी'
    VowelSignIi,
    /// \u{941}: 'ु'
    VowelSignU,
    /// \u{942}: 'ू'
    VowelSignUu,
    /// \u{943}: 'ृ'
    VowelSignVocalicR,
    /// \u{944}: 'ॄ'
    VowelSignVocalicRr,
    /// \u{945}: 'ॅ'
    VowelSignCandraE,
    /// \u{946}: 'ॆ'
    VowelSignShortE,
    /// \u{947}: 'े'
    VowelSignE,
    /// \u{948}: 'ै'
    VowelSignAi,
    /// \u{949}: 'ॉ'
    VowelSignCandraO,
    /// \u{94a}: 'ॊ'
    VowelSignShortO,
    /// \u{94b}: 'ो'
    VowelSignO,
    /// \u{94c}: 'ौ'
    VowelSignAu,
    /// \u{94d}: '्'
    SignVirama,
    /// \u{94e}: 'ॎ'
    VowelSignPrishthamatraE,
    /// \u{94f}: 'ॏ'
    VowelSignAw,
    /// \u{950}: 'ॐ'
    Om,
    /// \u{951}: '॑'
    StressSignUdatta,
    /// \u{952}: '॒'
    StressSignAnudatta,
    /// \u{953}: '॓'
    GraveAccent,
    /// \u{954}: '॔'
    AcuteAccent,
    /// \u{955}: 'ॕ'
    VowelSignCandraLongE,
    /// \u{956}: 'ॖ'
    VowelSignUe,
    /// \u{957}: 'ॗ'
    VowelSignUue,
    /// \u{958}: 'क़'
    LetterQa,
    /// \u{959}: 'ख़'
    LetterKhha,
    /// \u{95a}: 'ग़'
    LetterGhha,
    /// \u{95b}: 'ज़'
    LetterZa,
    /// \u{95c}: 'ड़'
    LetterDddha,
    /// \u{95d}: 'ढ़'
    LetterRha,
    /// \u{95e}: 'फ़'
    LetterFa,
    /// \u{95f}: 'य़'
    LetterYya,
    /// \u{960}: 'ॠ'
    LetterVocalicRr,
    /// \u{961}: 'ॡ'
    LetterVocalicLl,
    /// \u{962}: 'ॢ'
    VowelSignVocalicL,
    /// \u{963}: 'ॣ'
    VowelSignVocalicLl,
    /// \u{964}: '।'
    Danda,
    /// \u{965}: '॥'
    DoubleDanda,
    /// \u{966}: '०'
    DigitZero,
    /// \u{967}: '१'
    DigitOne,
    /// \u{968}: '२'
    DigitTwo,
    /// \u{969}: '३'
    DigitThree,
    /// \u{96a}: '४'
    DigitFour,
    /// \u{96b}: '५'
    DigitFive,
    /// \u{96c}: '६'
    DigitSix,
    /// \u{96d}: '७'
    DigitSeven,
    /// \u{96e}: '८'
    DigitEight,
    /// \u{96f}: '९'
    DigitNine,
    /// \u{970}: '॰'
    AbbreviationSign,
    /// \u{971}: 'ॱ'
    SignHighSpacingDot,
    /// \u{972}: 'ॲ'
    LetterCandraA,
    /// \u{973}: 'ॳ'
    LetterOe,
    /// \u{974}: 'ॴ'
    LetterOoe,
    /// \u{975}: 'ॵ'
    LetterAw,
    /// \u{976}: 'ॶ'
    LetterUe,
    /// \u{977}: 'ॷ'
    LetterUue,
    /// \u{978}: 'ॸ'
    LetterMarwariDda,
    /// \u{979}: 'ॹ'
    LetterZha,
    /// \u{97a}: 'ॺ'
    LetterHeavyYa,
    /// \u{97b}: 'ॻ'
    LetterGga,
    /// \u{97c}: 'ॼ'
    LetterJja,
    /// \u{97d}: 'ॽ'
    LetterGlottalStop,
    /// \u{97e}: 'ॾ'
    LetterDdda,
}

impl Into<char> for Devanagari {
    fn into(self) -> char {
        use constants::*;
        match self {
            Devanagari::SignInvertedCandrabindu => SIGN_INVERTED_CANDRABINDU,
            Devanagari::SignCandrabindu => SIGN_CANDRABINDU,
            Devanagari::SignAnusvara => SIGN_ANUSVARA,
            Devanagari::SignVisarga => SIGN_VISARGA,
            Devanagari::LetterShortA => LETTER_SHORT_A,
            Devanagari::LetterA => LETTER_A,
            Devanagari::LetterAa => LETTER_AA,
            Devanagari::LetterI => LETTER_I,
            Devanagari::LetterIi => LETTER_II,
            Devanagari::LetterU => LETTER_U,
            Devanagari::LetterUu => LETTER_UU,
            Devanagari::LetterVocalicR => LETTER_VOCALIC_R,
            Devanagari::LetterVocalicL => LETTER_VOCALIC_L,
            Devanagari::LetterCandraE => LETTER_CANDRA_E,
            Devanagari::LetterShortE => LETTER_SHORT_E,
            Devanagari::LetterE => LETTER_E,
            Devanagari::LetterAi => LETTER_AI,
            Devanagari::LetterCandraO => LETTER_CANDRA_O,
            Devanagari::LetterShortO => LETTER_SHORT_O,
            Devanagari::LetterO => LETTER_O,
            Devanagari::LetterAu => LETTER_AU,
            Devanagari::LetterKa => LETTER_KA,
            Devanagari::LetterKha => LETTER_KHA,
            Devanagari::LetterGa => LETTER_GA,
            Devanagari::LetterGha => LETTER_GHA,
            Devanagari::LetterNga => LETTER_NGA,
            Devanagari::LetterCa => LETTER_CA,
            Devanagari::LetterCha => LETTER_CHA,
            Devanagari::LetterJa => LETTER_JA,
            Devanagari::LetterJha => LETTER_JHA,
            Devanagari::LetterNya => LETTER_NYA,
            Devanagari::LetterTta => LETTER_TTA,
            Devanagari::LetterTtha => LETTER_TTHA,
            Devanagari::LetterDda => LETTER_DDA,
            Devanagari::LetterDdha => LETTER_DDHA,
            Devanagari::LetterNna => LETTER_NNA,
            Devanagari::LetterTa => LETTER_TA,
            Devanagari::LetterTha => LETTER_THA,
            Devanagari::LetterDa => LETTER_DA,
            Devanagari::LetterDha => LETTER_DHA,
            Devanagari::LetterNa => LETTER_NA,
            Devanagari::LetterNnna => LETTER_NNNA,
            Devanagari::LetterPa => LETTER_PA,
            Devanagari::LetterPha => LETTER_PHA,
            Devanagari::LetterBa => LETTER_BA,
            Devanagari::LetterBha => LETTER_BHA,
            Devanagari::LetterMa => LETTER_MA,
            Devanagari::LetterYa => LETTER_YA,
            Devanagari::LetterRa => LETTER_RA,
            Devanagari::LetterRra => LETTER_RRA,
            Devanagari::LetterLa => LETTER_LA,
            Devanagari::LetterLla => LETTER_LLA,
            Devanagari::LetterLlla => LETTER_LLLA,
            Devanagari::LetterVa => LETTER_VA,
            Devanagari::LetterSha => LETTER_SHA,
            Devanagari::LetterSsa => LETTER_SSA,
            Devanagari::LetterSa => LETTER_SA,
            Devanagari::LetterHa => LETTER_HA,
            Devanagari::VowelSignOe => VOWEL_SIGN_OE,
            Devanagari::VowelSignOoe => VOWEL_SIGN_OOE,
            Devanagari::SignNukta => SIGN_NUKTA,
            Devanagari::SignAvagraha => SIGN_AVAGRAHA,
            Devanagari::VowelSignAa => VOWEL_SIGN_AA,
            Devanagari::VowelSignI => VOWEL_SIGN_I,
            Devanagari::VowelSignIi => VOWEL_SIGN_II,
            Devanagari::VowelSignU => VOWEL_SIGN_U,
            Devanagari::VowelSignUu => VOWEL_SIGN_UU,
            Devanagari::VowelSignVocalicR => VOWEL_SIGN_VOCALIC_R,
            Devanagari::VowelSignVocalicRr => VOWEL_SIGN_VOCALIC_RR,
            Devanagari::VowelSignCandraE => VOWEL_SIGN_CANDRA_E,
            Devanagari::VowelSignShortE => VOWEL_SIGN_SHORT_E,
            Devanagari::VowelSignE => VOWEL_SIGN_E,
            Devanagari::VowelSignAi => VOWEL_SIGN_AI,
            Devanagari::VowelSignCandraO => VOWEL_SIGN_CANDRA_O,
            Devanagari::VowelSignShortO => VOWEL_SIGN_SHORT_O,
            Devanagari::VowelSignO => VOWEL_SIGN_O,
            Devanagari::VowelSignAu => VOWEL_SIGN_AU,
            Devanagari::SignVirama => SIGN_VIRAMA,
            Devanagari::VowelSignPrishthamatraE => VOWEL_SIGN_PRISHTHAMATRA_E,
            Devanagari::VowelSignAw => VOWEL_SIGN_AW,
            Devanagari::Om => OM,
            Devanagari::StressSignUdatta => STRESS_SIGN_UDATTA,
            Devanagari::StressSignAnudatta => STRESS_SIGN_ANUDATTA,
            Devanagari::GraveAccent => GRAVE_ACCENT,
            Devanagari::AcuteAccent => ACUTE_ACCENT,
            Devanagari::VowelSignCandraLongE => VOWEL_SIGN_CANDRA_LONG_E,
            Devanagari::VowelSignUe => VOWEL_SIGN_UE,
            Devanagari::VowelSignUue => VOWEL_SIGN_UUE,
            Devanagari::LetterQa => LETTER_QA,
            Devanagari::LetterKhha => LETTER_KHHA,
            Devanagari::LetterGhha => LETTER_GHHA,
            Devanagari::LetterZa => LETTER_ZA,
            Devanagari::LetterDddha => LETTER_DDDHA,
            Devanagari::LetterRha => LETTER_RHA,
            Devanagari::LetterFa => LETTER_FA,
            Devanagari::LetterYya => LETTER_YYA,
            Devanagari::LetterVocalicRr => LETTER_VOCALIC_RR,
            Devanagari::LetterVocalicLl => LETTER_VOCALIC_LL,
            Devanagari::VowelSignVocalicL => VOWEL_SIGN_VOCALIC_L,
            Devanagari::VowelSignVocalicLl => VOWEL_SIGN_VOCALIC_LL,
            Devanagari::Danda => DANDA,
            Devanagari::DoubleDanda => DOUBLE_DANDA,
            Devanagari::DigitZero => DIGIT_ZERO,
            Devanagari::DigitOne => DIGIT_ONE,
            Devanagari::DigitTwo => DIGIT_TWO,
            Devanagari::DigitThree => DIGIT_THREE,
            Devanagari::DigitFour => DIGIT_FOUR,
            Devanagari::DigitFive => DIGIT_FIVE,
            Devanagari::DigitSix => DIGIT_SIX,
            Devanagari::DigitSeven => DIGIT_SEVEN,
            Devanagari::DigitEight => DIGIT_EIGHT,
            Devanagari::DigitNine => DIGIT_NINE,
            Devanagari::AbbreviationSign => ABBREVIATION_SIGN,
            Devanagari::SignHighSpacingDot => SIGN_HIGH_SPACING_DOT,
            Devanagari::LetterCandraA => LETTER_CANDRA_A,
            Devanagari::LetterOe => LETTER_OE,
            Devanagari::LetterOoe => LETTER_OOE,
            Devanagari::LetterAw => LETTER_AW,
            Devanagari::LetterUe => LETTER_UE,
            Devanagari::LetterUue => LETTER_UUE,
            Devanagari::LetterMarwariDda => LETTER_MARWARI_DDA,
            Devanagari::LetterZha => LETTER_ZHA,
            Devanagari::LetterHeavyYa => LETTER_HEAVY_YA,
            Devanagari::LetterGga => LETTER_GGA,
            Devanagari::LetterJja => LETTER_JJA,
            Devanagari::LetterGlottalStop => LETTER_GLOTTAL_STOP,
            Devanagari::LetterDdda => LETTER_DDDA,
        }
    }
}

impl std::convert::TryFrom<char> for Devanagari {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            SIGN_INVERTED_CANDRABINDU => Ok(Devanagari::SignInvertedCandrabindu),
            SIGN_CANDRABINDU => Ok(Devanagari::SignCandrabindu),
            SIGN_ANUSVARA => Ok(Devanagari::SignAnusvara),
            SIGN_VISARGA => Ok(Devanagari::SignVisarga),
            LETTER_SHORT_A => Ok(Devanagari::LetterShortA),
            LETTER_A => Ok(Devanagari::LetterA),
            LETTER_AA => Ok(Devanagari::LetterAa),
            LETTER_I => Ok(Devanagari::LetterI),
            LETTER_II => Ok(Devanagari::LetterIi),
            LETTER_U => Ok(Devanagari::LetterU),
            LETTER_UU => Ok(Devanagari::LetterUu),
            LETTER_VOCALIC_R => Ok(Devanagari::LetterVocalicR),
            LETTER_VOCALIC_L => Ok(Devanagari::LetterVocalicL),
            LETTER_CANDRA_E => Ok(Devanagari::LetterCandraE),
            LETTER_SHORT_E => Ok(Devanagari::LetterShortE),
            LETTER_E => Ok(Devanagari::LetterE),
            LETTER_AI => Ok(Devanagari::LetterAi),
            LETTER_CANDRA_O => Ok(Devanagari::LetterCandraO),
            LETTER_SHORT_O => Ok(Devanagari::LetterShortO),
            LETTER_O => Ok(Devanagari::LetterO),
            LETTER_AU => Ok(Devanagari::LetterAu),
            LETTER_KA => Ok(Devanagari::LetterKa),
            LETTER_KHA => Ok(Devanagari::LetterKha),
            LETTER_GA => Ok(Devanagari::LetterGa),
            LETTER_GHA => Ok(Devanagari::LetterGha),
            LETTER_NGA => Ok(Devanagari::LetterNga),
            LETTER_CA => Ok(Devanagari::LetterCa),
            LETTER_CHA => Ok(Devanagari::LetterCha),
            LETTER_JA => Ok(Devanagari::LetterJa),
            LETTER_JHA => Ok(Devanagari::LetterJha),
            LETTER_NYA => Ok(Devanagari::LetterNya),
            LETTER_TTA => Ok(Devanagari::LetterTta),
            LETTER_TTHA => Ok(Devanagari::LetterTtha),
            LETTER_DDA => Ok(Devanagari::LetterDda),
            LETTER_DDHA => Ok(Devanagari::LetterDdha),
            LETTER_NNA => Ok(Devanagari::LetterNna),
            LETTER_TA => Ok(Devanagari::LetterTa),
            LETTER_THA => Ok(Devanagari::LetterTha),
            LETTER_DA => Ok(Devanagari::LetterDa),
            LETTER_DHA => Ok(Devanagari::LetterDha),
            LETTER_NA => Ok(Devanagari::LetterNa),
            LETTER_NNNA => Ok(Devanagari::LetterNnna),
            LETTER_PA => Ok(Devanagari::LetterPa),
            LETTER_PHA => Ok(Devanagari::LetterPha),
            LETTER_BA => Ok(Devanagari::LetterBa),
            LETTER_BHA => Ok(Devanagari::LetterBha),
            LETTER_MA => Ok(Devanagari::LetterMa),
            LETTER_YA => Ok(Devanagari::LetterYa),
            LETTER_RA => Ok(Devanagari::LetterRa),
            LETTER_RRA => Ok(Devanagari::LetterRra),
            LETTER_LA => Ok(Devanagari::LetterLa),
            LETTER_LLA => Ok(Devanagari::LetterLla),
            LETTER_LLLA => Ok(Devanagari::LetterLlla),
            LETTER_VA => Ok(Devanagari::LetterVa),
            LETTER_SHA => Ok(Devanagari::LetterSha),
            LETTER_SSA => Ok(Devanagari::LetterSsa),
            LETTER_SA => Ok(Devanagari::LetterSa),
            LETTER_HA => Ok(Devanagari::LetterHa),
            VOWEL_SIGN_OE => Ok(Devanagari::VowelSignOe),
            VOWEL_SIGN_OOE => Ok(Devanagari::VowelSignOoe),
            SIGN_NUKTA => Ok(Devanagari::SignNukta),
            SIGN_AVAGRAHA => Ok(Devanagari::SignAvagraha),
            VOWEL_SIGN_AA => Ok(Devanagari::VowelSignAa),
            VOWEL_SIGN_I => Ok(Devanagari::VowelSignI),
            VOWEL_SIGN_II => Ok(Devanagari::VowelSignIi),
            VOWEL_SIGN_U => Ok(Devanagari::VowelSignU),
            VOWEL_SIGN_UU => Ok(Devanagari::VowelSignUu),
            VOWEL_SIGN_VOCALIC_R => Ok(Devanagari::VowelSignVocalicR),
            VOWEL_SIGN_VOCALIC_RR => Ok(Devanagari::VowelSignVocalicRr),
            VOWEL_SIGN_CANDRA_E => Ok(Devanagari::VowelSignCandraE),
            VOWEL_SIGN_SHORT_E => Ok(Devanagari::VowelSignShortE),
            VOWEL_SIGN_E => Ok(Devanagari::VowelSignE),
            VOWEL_SIGN_AI => Ok(Devanagari::VowelSignAi),
            VOWEL_SIGN_CANDRA_O => Ok(Devanagari::VowelSignCandraO),
            VOWEL_SIGN_SHORT_O => Ok(Devanagari::VowelSignShortO),
            VOWEL_SIGN_O => Ok(Devanagari::VowelSignO),
            VOWEL_SIGN_AU => Ok(Devanagari::VowelSignAu),
            SIGN_VIRAMA => Ok(Devanagari::SignVirama),
            VOWEL_SIGN_PRISHTHAMATRA_E => Ok(Devanagari::VowelSignPrishthamatraE),
            VOWEL_SIGN_AW => Ok(Devanagari::VowelSignAw),
            OM => Ok(Devanagari::Om),
            STRESS_SIGN_UDATTA => Ok(Devanagari::StressSignUdatta),
            STRESS_SIGN_ANUDATTA => Ok(Devanagari::StressSignAnudatta),
            GRAVE_ACCENT => Ok(Devanagari::GraveAccent),
            ACUTE_ACCENT => Ok(Devanagari::AcuteAccent),
            VOWEL_SIGN_CANDRA_LONG_E => Ok(Devanagari::VowelSignCandraLongE),
            VOWEL_SIGN_UE => Ok(Devanagari::VowelSignUe),
            VOWEL_SIGN_UUE => Ok(Devanagari::VowelSignUue),
            LETTER_QA => Ok(Devanagari::LetterQa),
            LETTER_KHHA => Ok(Devanagari::LetterKhha),
            LETTER_GHHA => Ok(Devanagari::LetterGhha),
            LETTER_ZA => Ok(Devanagari::LetterZa),
            LETTER_DDDHA => Ok(Devanagari::LetterDddha),
            LETTER_RHA => Ok(Devanagari::LetterRha),
            LETTER_FA => Ok(Devanagari::LetterFa),
            LETTER_YYA => Ok(Devanagari::LetterYya),
            LETTER_VOCALIC_RR => Ok(Devanagari::LetterVocalicRr),
            LETTER_VOCALIC_LL => Ok(Devanagari::LetterVocalicLl),
            VOWEL_SIGN_VOCALIC_L => Ok(Devanagari::VowelSignVocalicL),
            VOWEL_SIGN_VOCALIC_LL => Ok(Devanagari::VowelSignVocalicLl),
            DANDA => Ok(Devanagari::Danda),
            DOUBLE_DANDA => Ok(Devanagari::DoubleDanda),
            DIGIT_ZERO => Ok(Devanagari::DigitZero),
            DIGIT_ONE => Ok(Devanagari::DigitOne),
            DIGIT_TWO => Ok(Devanagari::DigitTwo),
            DIGIT_THREE => Ok(Devanagari::DigitThree),
            DIGIT_FOUR => Ok(Devanagari::DigitFour),
            DIGIT_FIVE => Ok(Devanagari::DigitFive),
            DIGIT_SIX => Ok(Devanagari::DigitSix),
            DIGIT_SEVEN => Ok(Devanagari::DigitSeven),
            DIGIT_EIGHT => Ok(Devanagari::DigitEight),
            DIGIT_NINE => Ok(Devanagari::DigitNine),
            ABBREVIATION_SIGN => Ok(Devanagari::AbbreviationSign),
            SIGN_HIGH_SPACING_DOT => Ok(Devanagari::SignHighSpacingDot),
            LETTER_CANDRA_A => Ok(Devanagari::LetterCandraA),
            LETTER_OE => Ok(Devanagari::LetterOe),
            LETTER_OOE => Ok(Devanagari::LetterOoe),
            LETTER_AW => Ok(Devanagari::LetterAw),
            LETTER_UE => Ok(Devanagari::LetterUe),
            LETTER_UUE => Ok(Devanagari::LetterUue),
            LETTER_MARWARI_DDA => Ok(Devanagari::LetterMarwariDda),
            LETTER_ZHA => Ok(Devanagari::LetterZha),
            LETTER_HEAVY_YA => Ok(Devanagari::LetterHeavyYa),
            LETTER_GGA => Ok(Devanagari::LetterGga),
            LETTER_JJA => Ok(Devanagari::LetterJja),
            LETTER_GLOTTAL_STOP => Ok(Devanagari::LetterGlottalStop),
            LETTER_DDDA => Ok(Devanagari::LetterDdda),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Devanagari {
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

impl std::convert::TryFrom<u32> for Devanagari {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Devanagari {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Devanagari {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Devanagari::SignInvertedCandrabindu
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Devanagari::SignInvertedCandrabindu => "devanagari sign inverted candrabindu",
            Devanagari::SignCandrabindu => "devanagari sign candrabindu",
            Devanagari::SignAnusvara => "devanagari sign anusvara",
            Devanagari::SignVisarga => "devanagari sign visarga",
            Devanagari::LetterShortA => "devanagari letter short a",
            Devanagari::LetterA => "devanagari letter a",
            Devanagari::LetterAa => "devanagari letter aa",
            Devanagari::LetterI => "devanagari letter i",
            Devanagari::LetterIi => "devanagari letter ii",
            Devanagari::LetterU => "devanagari letter u",
            Devanagari::LetterUu => "devanagari letter uu",
            Devanagari::LetterVocalicR => "devanagari letter vocalic r",
            Devanagari::LetterVocalicL => "devanagari letter vocalic l",
            Devanagari::LetterCandraE => "devanagari letter candra e",
            Devanagari::LetterShortE => "devanagari letter short e",
            Devanagari::LetterE => "devanagari letter e",
            Devanagari::LetterAi => "devanagari letter ai",
            Devanagari::LetterCandraO => "devanagari letter candra o",
            Devanagari::LetterShortO => "devanagari letter short o",
            Devanagari::LetterO => "devanagari letter o",
            Devanagari::LetterAu => "devanagari letter au",
            Devanagari::LetterKa => "devanagari letter ka",
            Devanagari::LetterKha => "devanagari letter kha",
            Devanagari::LetterGa => "devanagari letter ga",
            Devanagari::LetterGha => "devanagari letter gha",
            Devanagari::LetterNga => "devanagari letter nga",
            Devanagari::LetterCa => "devanagari letter ca",
            Devanagari::LetterCha => "devanagari letter cha",
            Devanagari::LetterJa => "devanagari letter ja",
            Devanagari::LetterJha => "devanagari letter jha",
            Devanagari::LetterNya => "devanagari letter nya",
            Devanagari::LetterTta => "devanagari letter tta",
            Devanagari::LetterTtha => "devanagari letter ttha",
            Devanagari::LetterDda => "devanagari letter dda",
            Devanagari::LetterDdha => "devanagari letter ddha",
            Devanagari::LetterNna => "devanagari letter nna",
            Devanagari::LetterTa => "devanagari letter ta",
            Devanagari::LetterTha => "devanagari letter tha",
            Devanagari::LetterDa => "devanagari letter da",
            Devanagari::LetterDha => "devanagari letter dha",
            Devanagari::LetterNa => "devanagari letter na",
            Devanagari::LetterNnna => "devanagari letter nnna",
            Devanagari::LetterPa => "devanagari letter pa",
            Devanagari::LetterPha => "devanagari letter pha",
            Devanagari::LetterBa => "devanagari letter ba",
            Devanagari::LetterBha => "devanagari letter bha",
            Devanagari::LetterMa => "devanagari letter ma",
            Devanagari::LetterYa => "devanagari letter ya",
            Devanagari::LetterRa => "devanagari letter ra",
            Devanagari::LetterRra => "devanagari letter rra",
            Devanagari::LetterLa => "devanagari letter la",
            Devanagari::LetterLla => "devanagari letter lla",
            Devanagari::LetterLlla => "devanagari letter llla",
            Devanagari::LetterVa => "devanagari letter va",
            Devanagari::LetterSha => "devanagari letter sha",
            Devanagari::LetterSsa => "devanagari letter ssa",
            Devanagari::LetterSa => "devanagari letter sa",
            Devanagari::LetterHa => "devanagari letter ha",
            Devanagari::VowelSignOe => "devanagari vowel sign oe",
            Devanagari::VowelSignOoe => "devanagari vowel sign ooe",
            Devanagari::SignNukta => "devanagari sign nukta",
            Devanagari::SignAvagraha => "devanagari sign avagraha",
            Devanagari::VowelSignAa => "devanagari vowel sign aa",
            Devanagari::VowelSignI => "devanagari vowel sign i",
            Devanagari::VowelSignIi => "devanagari vowel sign ii",
            Devanagari::VowelSignU => "devanagari vowel sign u",
            Devanagari::VowelSignUu => "devanagari vowel sign uu",
            Devanagari::VowelSignVocalicR => "devanagari vowel sign vocalic r",
            Devanagari::VowelSignVocalicRr => "devanagari vowel sign vocalic rr",
            Devanagari::VowelSignCandraE => "devanagari vowel sign candra e",
            Devanagari::VowelSignShortE => "devanagari vowel sign short e",
            Devanagari::VowelSignE => "devanagari vowel sign e",
            Devanagari::VowelSignAi => "devanagari vowel sign ai",
            Devanagari::VowelSignCandraO => "devanagari vowel sign candra o",
            Devanagari::VowelSignShortO => "devanagari vowel sign short o",
            Devanagari::VowelSignO => "devanagari vowel sign o",
            Devanagari::VowelSignAu => "devanagari vowel sign au",
            Devanagari::SignVirama => "devanagari sign virama",
            Devanagari::VowelSignPrishthamatraE => "devanagari vowel sign prishthamatra e",
            Devanagari::VowelSignAw => "devanagari vowel sign aw",
            Devanagari::Om => "devanagari om",
            Devanagari::StressSignUdatta => "devanagari stress sign udatta",
            Devanagari::StressSignAnudatta => "devanagari stress sign anudatta",
            Devanagari::GraveAccent => "devanagari grave accent",
            Devanagari::AcuteAccent => "devanagari acute accent",
            Devanagari::VowelSignCandraLongE => "devanagari vowel sign candra long e",
            Devanagari::VowelSignUe => "devanagari vowel sign ue",
            Devanagari::VowelSignUue => "devanagari vowel sign uue",
            Devanagari::LetterQa => "devanagari letter qa",
            Devanagari::LetterKhha => "devanagari letter khha",
            Devanagari::LetterGhha => "devanagari letter ghha",
            Devanagari::LetterZa => "devanagari letter za",
            Devanagari::LetterDddha => "devanagari letter dddha",
            Devanagari::LetterRha => "devanagari letter rha",
            Devanagari::LetterFa => "devanagari letter fa",
            Devanagari::LetterYya => "devanagari letter yya",
            Devanagari::LetterVocalicRr => "devanagari letter vocalic rr",
            Devanagari::LetterVocalicLl => "devanagari letter vocalic ll",
            Devanagari::VowelSignVocalicL => "devanagari vowel sign vocalic l",
            Devanagari::VowelSignVocalicLl => "devanagari vowel sign vocalic ll",
            Devanagari::Danda => "devanagari danda",
            Devanagari::DoubleDanda => "devanagari double danda",
            Devanagari::DigitZero => "devanagari digit zero",
            Devanagari::DigitOne => "devanagari digit one",
            Devanagari::DigitTwo => "devanagari digit two",
            Devanagari::DigitThree => "devanagari digit three",
            Devanagari::DigitFour => "devanagari digit four",
            Devanagari::DigitFive => "devanagari digit five",
            Devanagari::DigitSix => "devanagari digit six",
            Devanagari::DigitSeven => "devanagari digit seven",
            Devanagari::DigitEight => "devanagari digit eight",
            Devanagari::DigitNine => "devanagari digit nine",
            Devanagari::AbbreviationSign => "devanagari abbreviation sign",
            Devanagari::SignHighSpacingDot => "devanagari sign high spacing dot",
            Devanagari::LetterCandraA => "devanagari letter candra a",
            Devanagari::LetterOe => "devanagari letter oe",
            Devanagari::LetterOoe => "devanagari letter ooe",
            Devanagari::LetterAw => "devanagari letter aw",
            Devanagari::LetterUe => "devanagari letter ue",
            Devanagari::LetterUue => "devanagari letter uue",
            Devanagari::LetterMarwariDda => "devanagari letter marwari dda",
            Devanagari::LetterZha => "devanagari letter zha",
            Devanagari::LetterHeavyYa => "devanagari letter heavy ya",
            Devanagari::LetterGga => "devanagari letter gga",
            Devanagari::LetterJja => "devanagari letter jja",
            Devanagari::LetterGlottalStop => "devanagari letter glottal stop",
            Devanagari::LetterDdda => "devanagari letter ddda",
        }
    }
}
