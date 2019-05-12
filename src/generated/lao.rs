/// \u{e80} → \u{eff}
///
/// ກ ຂ ຄ ຆ ງ ຈ ຉ ຊ ຌ ຍ ຎ ຏ ຐ ຑ ຒ ຓ\
/// ດ ຕ ຖ ທ ຘ ນ ບ ປ ຜ ຝ ພ ຟ ຠ ມ ຢ ຣ\
/// ລ ວ ຨ ຩ ສ ຫ ຬ ອ ຮ ຯ ະ ັ າ ຳ ິ ີ\
/// ຶ ື ຸ ູ ຺ ົ ຼ ຽ ເ ແ ໂ ໃ ໄ ໆ ່ ້\
/// ໊ ໋ ໌ ໍ ໐ ໑ ໒ ໓ ໔ ໕ ໖ ໗ ໘ ໙ ໜ ໝ\
/// ໞ ໟ\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{e81}: 'ກ'
    pub const LETTER_KO: char = 'ກ';
    /// \u{e82}: 'ຂ'
    pub const LETTER_KHO_SUNG: char = 'ຂ';
    /// \u{e84}: 'ຄ'
    pub const LETTER_KHO_TAM: char = 'ຄ';
    /// \u{e86}: 'ຆ'
    pub const LETTER_PALI_GHA: char = 'ຆ';
    /// \u{e87}: 'ງ'
    pub const LETTER_NGO: char = 'ງ';
    /// \u{e88}: 'ຈ'
    pub const LETTER_CO: char = 'ຈ';
    /// \u{e89}: 'ຉ'
    pub const LETTER_PALI_CHA: char = 'ຉ';
    /// \u{e8a}: 'ຊ'
    pub const LETTER_SO_TAM: char = 'ຊ';
    /// \u{e8c}: 'ຌ'
    pub const LETTER_PALI_JHA: char = 'ຌ';
    /// \u{e8d}: 'ຍ'
    pub const LETTER_NYO: char = 'ຍ';
    /// \u{e8e}: 'ຎ'
    pub const LETTER_PALI_NYA: char = 'ຎ';
    /// \u{e8f}: 'ຏ'
    pub const LETTER_PALI_TTA: char = 'ຏ';
    /// \u{e90}: 'ຐ'
    pub const LETTER_PALI_TTHA: char = 'ຐ';
    /// \u{e91}: 'ຑ'
    pub const LETTER_PALI_DDA: char = 'ຑ';
    /// \u{e92}: 'ຒ'
    pub const LETTER_PALI_DDHA: char = 'ຒ';
    /// \u{e93}: 'ຓ'
    pub const LETTER_PALI_NNA: char = 'ຓ';
    /// \u{e94}: 'ດ'
    pub const LETTER_DO: char = 'ດ';
    /// \u{e95}: 'ຕ'
    pub const LETTER_TO: char = 'ຕ';
    /// \u{e96}: 'ຖ'
    pub const LETTER_THO_SUNG: char = 'ຖ';
    /// \u{e97}: 'ທ'
    pub const LETTER_THO_TAM: char = 'ທ';
    /// \u{e98}: 'ຘ'
    pub const LETTER_PALI_DHA: char = 'ຘ';
    /// \u{e99}: 'ນ'
    pub const LETTER_NO: char = 'ນ';
    /// \u{e9a}: 'ບ'
    pub const LETTER_BO: char = 'ບ';
    /// \u{e9b}: 'ປ'
    pub const LETTER_PO: char = 'ປ';
    /// \u{e9c}: 'ຜ'
    pub const LETTER_PHO_SUNG: char = 'ຜ';
    /// \u{e9d}: 'ຝ'
    pub const LETTER_FO_TAM: char = 'ຝ';
    /// \u{e9e}: 'ພ'
    pub const LETTER_PHO_TAM: char = 'ພ';
    /// \u{e9f}: 'ຟ'
    pub const LETTER_FO_SUNG: char = 'ຟ';
    /// \u{ea0}: 'ຠ'
    pub const LETTER_PALI_BHA: char = 'ຠ';
    /// \u{ea1}: 'ມ'
    pub const LETTER_MO: char = 'ມ';
    /// \u{ea2}: 'ຢ'
    pub const LETTER_YO: char = 'ຢ';
    /// \u{ea3}: 'ຣ'
    pub const LETTER_LO_LING: char = 'ຣ';
    /// \u{ea5}: 'ລ'
    pub const LETTER_LO_LOOT: char = 'ລ';
    /// \u{ea7}: 'ວ'
    pub const LETTER_WO: char = 'ວ';
    /// \u{ea8}: 'ຨ'
    pub const LETTER_SANSKRIT_SHA: char = 'ຨ';
    /// \u{ea9}: 'ຩ'
    pub const LETTER_SANSKRIT_SSA: char = 'ຩ';
    /// \u{eaa}: 'ສ'
    pub const LETTER_SO_SUNG: char = 'ສ';
    /// \u{eab}: 'ຫ'
    pub const LETTER_HO_SUNG: char = 'ຫ';
    /// \u{eac}: 'ຬ'
    pub const LETTER_PALI_LLA: char = 'ຬ';
    /// \u{ead}: 'ອ'
    pub const LETTER_O: char = 'ອ';
    /// \u{eae}: 'ຮ'
    pub const LETTER_HO_TAM: char = 'ຮ';
    /// \u{eaf}: 'ຯ'
    pub const ELLIPSIS: char = 'ຯ';
    /// \u{eb0}: 'ະ'
    pub const VOWEL_SIGN_A: char = 'ະ';
    /// \u{eb1}: 'ັ'
    pub const VOWEL_SIGN_MAI_KAN: char = 'ັ';
    /// \u{eb2}: 'າ'
    pub const VOWEL_SIGN_AA: char = 'າ';
    /// \u{eb3}: 'ຳ'
    pub const VOWEL_SIGN_AM: char = 'ຳ';
    /// \u{eb4}: 'ິ'
    pub const VOWEL_SIGN_I: char = 'ິ';
    /// \u{eb5}: 'ີ'
    pub const VOWEL_SIGN_II: char = 'ີ';
    /// \u{eb6}: 'ຶ'
    pub const VOWEL_SIGN_Y: char = 'ຶ';
    /// \u{eb7}: 'ື'
    pub const VOWEL_SIGN_YY: char = 'ື';
    /// \u{eb8}: 'ຸ'
    pub const VOWEL_SIGN_U: char = 'ຸ';
    /// \u{eb9}: 'ູ'
    pub const VOWEL_SIGN_UU: char = 'ູ';
    /// \u{eba}: '຺'
    pub const SIGN_PALI_VIRAMA: char = '຺';
    /// \u{ebb}: 'ົ'
    pub const VOWEL_SIGN_MAI_KON: char = 'ົ';
    /// \u{ebc}: 'ຼ'
    pub const SEMIVOWEL_SIGN_LO: char = 'ຼ';
    /// \u{ebd}: 'ຽ'
    pub const SEMIVOWEL_SIGN_NYO: char = 'ຽ';
    /// \u{ec0}: 'ເ'
    pub const VOWEL_SIGN_E: char = 'ເ';
    /// \u{ec1}: 'ແ'
    pub const VOWEL_SIGN_EI: char = 'ແ';
    /// \u{ec2}: 'ໂ'
    pub const VOWEL_SIGN_O: char = 'ໂ';
    /// \u{ec3}: 'ໃ'
    pub const VOWEL_SIGN_AY: char = 'ໃ';
    /// \u{ec4}: 'ໄ'
    pub const VOWEL_SIGN_AI: char = 'ໄ';
    /// \u{ec6}: 'ໆ'
    pub const KO_LA: char = 'ໆ';
    /// \u{ec8}: '່'
    pub const TONE_MAI_EK: char = '່';
    /// \u{ec9}: '້'
    pub const TONE_MAI_THO: char = '້';
    /// \u{eca}: '໊'
    pub const TONE_MAI_TI: char = '໊';
    /// \u{ecb}: '໋'
    pub const TONE_MAI_CATAWA: char = '໋';
    /// \u{ecc}: '໌'
    pub const CANCELLATION_MARK: char = '໌';
    /// \u{ecd}: 'ໍ'
    pub const NIGGAHITA: char = 'ໍ';
    /// \u{ed0}: '໐'
    pub const DIGIT_ZERO: char = '໐';
    /// \u{ed1}: '໑'
    pub const DIGIT_ONE: char = '໑';
    /// \u{ed2}: '໒'
    pub const DIGIT_TWO: char = '໒';
    /// \u{ed3}: '໓'
    pub const DIGIT_THREE: char = '໓';
    /// \u{ed4}: '໔'
    pub const DIGIT_FOUR: char = '໔';
    /// \u{ed5}: '໕'
    pub const DIGIT_FIVE: char = '໕';
    /// \u{ed6}: '໖'
    pub const DIGIT_SIX: char = '໖';
    /// \u{ed7}: '໗'
    pub const DIGIT_SEVEN: char = '໗';
    /// \u{ed8}: '໘'
    pub const DIGIT_EIGHT: char = '໘';
    /// \u{ed9}: '໙'
    pub const DIGIT_NINE: char = '໙';
    /// \u{edc}: 'ໜ'
    pub const HO_NO: char = 'ໜ';
    /// \u{edd}: 'ໝ'
    pub const HO_MO: char = 'ໝ';
    /// \u{ede}: 'ໞ'
    pub const LETTER_KHMU_GO: char = 'ໞ';
    /// \u{edf}: 'ໟ'
    pub const LETTER_KHMU_NYO: char = 'ໟ';
}

/// An enum to represent all characters in the Lao block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Lao {
    /// \u{e81}: 'ກ'
    LetterKo,
    /// \u{e82}: 'ຂ'
    LetterKhoSung,
    /// \u{e84}: 'ຄ'
    LetterKhoTam,
    /// \u{e86}: 'ຆ'
    LetterPaliGha,
    /// \u{e87}: 'ງ'
    LetterNgo,
    /// \u{e88}: 'ຈ'
    LetterCo,
    /// \u{e89}: 'ຉ'
    LetterPaliCha,
    /// \u{e8a}: 'ຊ'
    LetterSoTam,
    /// \u{e8c}: 'ຌ'
    LetterPaliJha,
    /// \u{e8d}: 'ຍ'
    LetterNyo,
    /// \u{e8e}: 'ຎ'
    LetterPaliNya,
    /// \u{e8f}: 'ຏ'
    LetterPaliTta,
    /// \u{e90}: 'ຐ'
    LetterPaliTtha,
    /// \u{e91}: 'ຑ'
    LetterPaliDda,
    /// \u{e92}: 'ຒ'
    LetterPaliDdha,
    /// \u{e93}: 'ຓ'
    LetterPaliNna,
    /// \u{e94}: 'ດ'
    LetterDo,
    /// \u{e95}: 'ຕ'
    LetterTo,
    /// \u{e96}: 'ຖ'
    LetterThoSung,
    /// \u{e97}: 'ທ'
    LetterThoTam,
    /// \u{e98}: 'ຘ'
    LetterPaliDha,
    /// \u{e99}: 'ນ'
    LetterNo,
    /// \u{e9a}: 'ບ'
    LetterBo,
    /// \u{e9b}: 'ປ'
    LetterPo,
    /// \u{e9c}: 'ຜ'
    LetterPhoSung,
    /// \u{e9d}: 'ຝ'
    LetterFoTam,
    /// \u{e9e}: 'ພ'
    LetterPhoTam,
    /// \u{e9f}: 'ຟ'
    LetterFoSung,
    /// \u{ea0}: 'ຠ'
    LetterPaliBha,
    /// \u{ea1}: 'ມ'
    LetterMo,
    /// \u{ea2}: 'ຢ'
    LetterYo,
    /// \u{ea3}: 'ຣ'
    LetterLoLing,
    /// \u{ea5}: 'ລ'
    LetterLoLoot,
    /// \u{ea7}: 'ວ'
    LetterWo,
    /// \u{ea8}: 'ຨ'
    LetterSanskritSha,
    /// \u{ea9}: 'ຩ'
    LetterSanskritSsa,
    /// \u{eaa}: 'ສ'
    LetterSoSung,
    /// \u{eab}: 'ຫ'
    LetterHoSung,
    /// \u{eac}: 'ຬ'
    LetterPaliLla,
    /// \u{ead}: 'ອ'
    LetterO,
    /// \u{eae}: 'ຮ'
    LetterHoTam,
    /// \u{eaf}: 'ຯ'
    Ellipsis,
    /// \u{eb0}: 'ະ'
    VowelSignA,
    /// \u{eb1}: 'ັ'
    VowelSignMaiKan,
    /// \u{eb2}: 'າ'
    VowelSignAa,
    /// \u{eb3}: 'ຳ'
    VowelSignAm,
    /// \u{eb4}: 'ິ'
    VowelSignI,
    /// \u{eb5}: 'ີ'
    VowelSignIi,
    /// \u{eb6}: 'ຶ'
    VowelSignY,
    /// \u{eb7}: 'ື'
    VowelSignYy,
    /// \u{eb8}: 'ຸ'
    VowelSignU,
    /// \u{eb9}: 'ູ'
    VowelSignUu,
    /// \u{eba}: '຺'
    SignPaliVirama,
    /// \u{ebb}: 'ົ'
    VowelSignMaiKon,
    /// \u{ebc}: 'ຼ'
    SemivowelSignLo,
    /// \u{ebd}: 'ຽ'
    SemivowelSignNyo,
    /// \u{ec0}: 'ເ'
    VowelSignE,
    /// \u{ec1}: 'ແ'
    VowelSignEi,
    /// \u{ec2}: 'ໂ'
    VowelSignO,
    /// \u{ec3}: 'ໃ'
    VowelSignAy,
    /// \u{ec4}: 'ໄ'
    VowelSignAi,
    /// \u{ec6}: 'ໆ'
    KoLa,
    /// \u{ec8}: '່'
    ToneMaiEk,
    /// \u{ec9}: '້'
    ToneMaiTho,
    /// \u{eca}: '໊'
    ToneMaiTi,
    /// \u{ecb}: '໋'
    ToneMaiCatawa,
    /// \u{ecc}: '໌'
    CancellationMark,
    /// \u{ecd}: 'ໍ'
    Niggahita,
    /// \u{ed0}: '໐'
    DigitZero,
    /// \u{ed1}: '໑'
    DigitOne,
    /// \u{ed2}: '໒'
    DigitTwo,
    /// \u{ed3}: '໓'
    DigitThree,
    /// \u{ed4}: '໔'
    DigitFour,
    /// \u{ed5}: '໕'
    DigitFive,
    /// \u{ed6}: '໖'
    DigitSix,
    /// \u{ed7}: '໗'
    DigitSeven,
    /// \u{ed8}: '໘'
    DigitEight,
    /// \u{ed9}: '໙'
    DigitNine,
    /// \u{edc}: 'ໜ'
    HoNo,
    /// \u{edd}: 'ໝ'
    HoMo,
    /// \u{ede}: 'ໞ'
    LetterKhmuGo,
    /// \u{edf}: 'ໟ'
    LetterKhmuNyo,
}

impl Into<char> for Lao {
    fn into(self) -> char {
        use constants::*;
        match self {
            Lao::LetterKo => LETTER_KO,
            Lao::LetterKhoSung => LETTER_KHO_SUNG,
            Lao::LetterKhoTam => LETTER_KHO_TAM,
            Lao::LetterPaliGha => LETTER_PALI_GHA,
            Lao::LetterNgo => LETTER_NGO,
            Lao::LetterCo => LETTER_CO,
            Lao::LetterPaliCha => LETTER_PALI_CHA,
            Lao::LetterSoTam => LETTER_SO_TAM,
            Lao::LetterPaliJha => LETTER_PALI_JHA,
            Lao::LetterNyo => LETTER_NYO,
            Lao::LetterPaliNya => LETTER_PALI_NYA,
            Lao::LetterPaliTta => LETTER_PALI_TTA,
            Lao::LetterPaliTtha => LETTER_PALI_TTHA,
            Lao::LetterPaliDda => LETTER_PALI_DDA,
            Lao::LetterPaliDdha => LETTER_PALI_DDHA,
            Lao::LetterPaliNna => LETTER_PALI_NNA,
            Lao::LetterDo => LETTER_DO,
            Lao::LetterTo => LETTER_TO,
            Lao::LetterThoSung => LETTER_THO_SUNG,
            Lao::LetterThoTam => LETTER_THO_TAM,
            Lao::LetterPaliDha => LETTER_PALI_DHA,
            Lao::LetterNo => LETTER_NO,
            Lao::LetterBo => LETTER_BO,
            Lao::LetterPo => LETTER_PO,
            Lao::LetterPhoSung => LETTER_PHO_SUNG,
            Lao::LetterFoTam => LETTER_FO_TAM,
            Lao::LetterPhoTam => LETTER_PHO_TAM,
            Lao::LetterFoSung => LETTER_FO_SUNG,
            Lao::LetterPaliBha => LETTER_PALI_BHA,
            Lao::LetterMo => LETTER_MO,
            Lao::LetterYo => LETTER_YO,
            Lao::LetterLoLing => LETTER_LO_LING,
            Lao::LetterLoLoot => LETTER_LO_LOOT,
            Lao::LetterWo => LETTER_WO,
            Lao::LetterSanskritSha => LETTER_SANSKRIT_SHA,
            Lao::LetterSanskritSsa => LETTER_SANSKRIT_SSA,
            Lao::LetterSoSung => LETTER_SO_SUNG,
            Lao::LetterHoSung => LETTER_HO_SUNG,
            Lao::LetterPaliLla => LETTER_PALI_LLA,
            Lao::LetterO => LETTER_O,
            Lao::LetterHoTam => LETTER_HO_TAM,
            Lao::Ellipsis => ELLIPSIS,
            Lao::VowelSignA => VOWEL_SIGN_A,
            Lao::VowelSignMaiKan => VOWEL_SIGN_MAI_KAN,
            Lao::VowelSignAa => VOWEL_SIGN_AA,
            Lao::VowelSignAm => VOWEL_SIGN_AM,
            Lao::VowelSignI => VOWEL_SIGN_I,
            Lao::VowelSignIi => VOWEL_SIGN_II,
            Lao::VowelSignY => VOWEL_SIGN_Y,
            Lao::VowelSignYy => VOWEL_SIGN_YY,
            Lao::VowelSignU => VOWEL_SIGN_U,
            Lao::VowelSignUu => VOWEL_SIGN_UU,
            Lao::SignPaliVirama => SIGN_PALI_VIRAMA,
            Lao::VowelSignMaiKon => VOWEL_SIGN_MAI_KON,
            Lao::SemivowelSignLo => SEMIVOWEL_SIGN_LO,
            Lao::SemivowelSignNyo => SEMIVOWEL_SIGN_NYO,
            Lao::VowelSignE => VOWEL_SIGN_E,
            Lao::VowelSignEi => VOWEL_SIGN_EI,
            Lao::VowelSignO => VOWEL_SIGN_O,
            Lao::VowelSignAy => VOWEL_SIGN_AY,
            Lao::VowelSignAi => VOWEL_SIGN_AI,
            Lao::KoLa => KO_LA,
            Lao::ToneMaiEk => TONE_MAI_EK,
            Lao::ToneMaiTho => TONE_MAI_THO,
            Lao::ToneMaiTi => TONE_MAI_TI,
            Lao::ToneMaiCatawa => TONE_MAI_CATAWA,
            Lao::CancellationMark => CANCELLATION_MARK,
            Lao::Niggahita => NIGGAHITA,
            Lao::DigitZero => DIGIT_ZERO,
            Lao::DigitOne => DIGIT_ONE,
            Lao::DigitTwo => DIGIT_TWO,
            Lao::DigitThree => DIGIT_THREE,
            Lao::DigitFour => DIGIT_FOUR,
            Lao::DigitFive => DIGIT_FIVE,
            Lao::DigitSix => DIGIT_SIX,
            Lao::DigitSeven => DIGIT_SEVEN,
            Lao::DigitEight => DIGIT_EIGHT,
            Lao::DigitNine => DIGIT_NINE,
            Lao::HoNo => HO_NO,
            Lao::HoMo => HO_MO,
            Lao::LetterKhmuGo => LETTER_KHMU_GO,
            Lao::LetterKhmuNyo => LETTER_KHMU_NYO,
        }
    }
}

impl std::convert::TryFrom<char> for Lao {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_KO => Ok(Lao::LetterKo),
            LETTER_KHO_SUNG => Ok(Lao::LetterKhoSung),
            LETTER_KHO_TAM => Ok(Lao::LetterKhoTam),
            LETTER_PALI_GHA => Ok(Lao::LetterPaliGha),
            LETTER_NGO => Ok(Lao::LetterNgo),
            LETTER_CO => Ok(Lao::LetterCo),
            LETTER_PALI_CHA => Ok(Lao::LetterPaliCha),
            LETTER_SO_TAM => Ok(Lao::LetterSoTam),
            LETTER_PALI_JHA => Ok(Lao::LetterPaliJha),
            LETTER_NYO => Ok(Lao::LetterNyo),
            LETTER_PALI_NYA => Ok(Lao::LetterPaliNya),
            LETTER_PALI_TTA => Ok(Lao::LetterPaliTta),
            LETTER_PALI_TTHA => Ok(Lao::LetterPaliTtha),
            LETTER_PALI_DDA => Ok(Lao::LetterPaliDda),
            LETTER_PALI_DDHA => Ok(Lao::LetterPaliDdha),
            LETTER_PALI_NNA => Ok(Lao::LetterPaliNna),
            LETTER_DO => Ok(Lao::LetterDo),
            LETTER_TO => Ok(Lao::LetterTo),
            LETTER_THO_SUNG => Ok(Lao::LetterThoSung),
            LETTER_THO_TAM => Ok(Lao::LetterThoTam),
            LETTER_PALI_DHA => Ok(Lao::LetterPaliDha),
            LETTER_NO => Ok(Lao::LetterNo),
            LETTER_BO => Ok(Lao::LetterBo),
            LETTER_PO => Ok(Lao::LetterPo),
            LETTER_PHO_SUNG => Ok(Lao::LetterPhoSung),
            LETTER_FO_TAM => Ok(Lao::LetterFoTam),
            LETTER_PHO_TAM => Ok(Lao::LetterPhoTam),
            LETTER_FO_SUNG => Ok(Lao::LetterFoSung),
            LETTER_PALI_BHA => Ok(Lao::LetterPaliBha),
            LETTER_MO => Ok(Lao::LetterMo),
            LETTER_YO => Ok(Lao::LetterYo),
            LETTER_LO_LING => Ok(Lao::LetterLoLing),
            LETTER_LO_LOOT => Ok(Lao::LetterLoLoot),
            LETTER_WO => Ok(Lao::LetterWo),
            LETTER_SANSKRIT_SHA => Ok(Lao::LetterSanskritSha),
            LETTER_SANSKRIT_SSA => Ok(Lao::LetterSanskritSsa),
            LETTER_SO_SUNG => Ok(Lao::LetterSoSung),
            LETTER_HO_SUNG => Ok(Lao::LetterHoSung),
            LETTER_PALI_LLA => Ok(Lao::LetterPaliLla),
            LETTER_O => Ok(Lao::LetterO),
            LETTER_HO_TAM => Ok(Lao::LetterHoTam),
            ELLIPSIS => Ok(Lao::Ellipsis),
            VOWEL_SIGN_A => Ok(Lao::VowelSignA),
            VOWEL_SIGN_MAI_KAN => Ok(Lao::VowelSignMaiKan),
            VOWEL_SIGN_AA => Ok(Lao::VowelSignAa),
            VOWEL_SIGN_AM => Ok(Lao::VowelSignAm),
            VOWEL_SIGN_I => Ok(Lao::VowelSignI),
            VOWEL_SIGN_II => Ok(Lao::VowelSignIi),
            VOWEL_SIGN_Y => Ok(Lao::VowelSignY),
            VOWEL_SIGN_YY => Ok(Lao::VowelSignYy),
            VOWEL_SIGN_U => Ok(Lao::VowelSignU),
            VOWEL_SIGN_UU => Ok(Lao::VowelSignUu),
            SIGN_PALI_VIRAMA => Ok(Lao::SignPaliVirama),
            VOWEL_SIGN_MAI_KON => Ok(Lao::VowelSignMaiKon),
            SEMIVOWEL_SIGN_LO => Ok(Lao::SemivowelSignLo),
            SEMIVOWEL_SIGN_NYO => Ok(Lao::SemivowelSignNyo),
            VOWEL_SIGN_E => Ok(Lao::VowelSignE),
            VOWEL_SIGN_EI => Ok(Lao::VowelSignEi),
            VOWEL_SIGN_O => Ok(Lao::VowelSignO),
            VOWEL_SIGN_AY => Ok(Lao::VowelSignAy),
            VOWEL_SIGN_AI => Ok(Lao::VowelSignAi),
            KO_LA => Ok(Lao::KoLa),
            TONE_MAI_EK => Ok(Lao::ToneMaiEk),
            TONE_MAI_THO => Ok(Lao::ToneMaiTho),
            TONE_MAI_TI => Ok(Lao::ToneMaiTi),
            TONE_MAI_CATAWA => Ok(Lao::ToneMaiCatawa),
            CANCELLATION_MARK => Ok(Lao::CancellationMark),
            NIGGAHITA => Ok(Lao::Niggahita),
            DIGIT_ZERO => Ok(Lao::DigitZero),
            DIGIT_ONE => Ok(Lao::DigitOne),
            DIGIT_TWO => Ok(Lao::DigitTwo),
            DIGIT_THREE => Ok(Lao::DigitThree),
            DIGIT_FOUR => Ok(Lao::DigitFour),
            DIGIT_FIVE => Ok(Lao::DigitFive),
            DIGIT_SIX => Ok(Lao::DigitSix),
            DIGIT_SEVEN => Ok(Lao::DigitSeven),
            DIGIT_EIGHT => Ok(Lao::DigitEight),
            DIGIT_NINE => Ok(Lao::DigitNine),
            HO_NO => Ok(Lao::HoNo),
            HO_MO => Ok(Lao::HoMo),
            LETTER_KHMU_GO => Ok(Lao::LetterKhmuGo),
            LETTER_KHMU_NYO => Ok(Lao::LetterKhmuNyo),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Lao {
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

impl std::convert::TryFrom<u32> for Lao {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Lao {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Lao {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Lao::LetterKo
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Lao::LetterKo => "lao letter ko",
            Lao::LetterKhoSung => "lao letter kho sung",
            Lao::LetterKhoTam => "lao letter kho tam",
            Lao::LetterPaliGha => "lao letter pali gha",
            Lao::LetterNgo => "lao letter ngo",
            Lao::LetterCo => "lao letter co",
            Lao::LetterPaliCha => "lao letter pali cha",
            Lao::LetterSoTam => "lao letter so tam",
            Lao::LetterPaliJha => "lao letter pali jha",
            Lao::LetterNyo => "lao letter nyo",
            Lao::LetterPaliNya => "lao letter pali nya",
            Lao::LetterPaliTta => "lao letter pali tta",
            Lao::LetterPaliTtha => "lao letter pali ttha",
            Lao::LetterPaliDda => "lao letter pali dda",
            Lao::LetterPaliDdha => "lao letter pali ddha",
            Lao::LetterPaliNna => "lao letter pali nna",
            Lao::LetterDo => "lao letter do",
            Lao::LetterTo => "lao letter to",
            Lao::LetterThoSung => "lao letter tho sung",
            Lao::LetterThoTam => "lao letter tho tam",
            Lao::LetterPaliDha => "lao letter pali dha",
            Lao::LetterNo => "lao letter no",
            Lao::LetterBo => "lao letter bo",
            Lao::LetterPo => "lao letter po",
            Lao::LetterPhoSung => "lao letter pho sung",
            Lao::LetterFoTam => "lao letter fo tam",
            Lao::LetterPhoTam => "lao letter pho tam",
            Lao::LetterFoSung => "lao letter fo sung",
            Lao::LetterPaliBha => "lao letter pali bha",
            Lao::LetterMo => "lao letter mo",
            Lao::LetterYo => "lao letter yo",
            Lao::LetterLoLing => "lao letter lo ling",
            Lao::LetterLoLoot => "lao letter lo loot",
            Lao::LetterWo => "lao letter wo",
            Lao::LetterSanskritSha => "lao letter sanskrit sha",
            Lao::LetterSanskritSsa => "lao letter sanskrit ssa",
            Lao::LetterSoSung => "lao letter so sung",
            Lao::LetterHoSung => "lao letter ho sung",
            Lao::LetterPaliLla => "lao letter pali lla",
            Lao::LetterO => "lao letter o",
            Lao::LetterHoTam => "lao letter ho tam",
            Lao::Ellipsis => "lao ellipsis",
            Lao::VowelSignA => "lao vowel sign a",
            Lao::VowelSignMaiKan => "lao vowel sign mai kan",
            Lao::VowelSignAa => "lao vowel sign aa",
            Lao::VowelSignAm => "lao vowel sign am",
            Lao::VowelSignI => "lao vowel sign i",
            Lao::VowelSignIi => "lao vowel sign ii",
            Lao::VowelSignY => "lao vowel sign y",
            Lao::VowelSignYy => "lao vowel sign yy",
            Lao::VowelSignU => "lao vowel sign u",
            Lao::VowelSignUu => "lao vowel sign uu",
            Lao::SignPaliVirama => "lao sign pali virama",
            Lao::VowelSignMaiKon => "lao vowel sign mai kon",
            Lao::SemivowelSignLo => "lao semivowel sign lo",
            Lao::SemivowelSignNyo => "lao semivowel sign nyo",
            Lao::VowelSignE => "lao vowel sign e",
            Lao::VowelSignEi => "lao vowel sign ei",
            Lao::VowelSignO => "lao vowel sign o",
            Lao::VowelSignAy => "lao vowel sign ay",
            Lao::VowelSignAi => "lao vowel sign ai",
            Lao::KoLa => "lao ko la",
            Lao::ToneMaiEk => "lao tone mai ek",
            Lao::ToneMaiTho => "lao tone mai tho",
            Lao::ToneMaiTi => "lao tone mai ti",
            Lao::ToneMaiCatawa => "lao tone mai catawa",
            Lao::CancellationMark => "lao cancellation mark",
            Lao::Niggahita => "lao niggahita",
            Lao::DigitZero => "lao digit zero",
            Lao::DigitOne => "lao digit one",
            Lao::DigitTwo => "lao digit two",
            Lao::DigitThree => "lao digit three",
            Lao::DigitFour => "lao digit four",
            Lao::DigitFive => "lao digit five",
            Lao::DigitSix => "lao digit six",
            Lao::DigitSeven => "lao digit seven",
            Lao::DigitEight => "lao digit eight",
            Lao::DigitNine => "lao digit nine",
            Lao::HoNo => "lao ho no",
            Lao::HoMo => "lao ho mo",
            Lao::LetterKhmuGo => "lao letter khmu go",
            Lao::LetterKhmuNyo => "lao letter khmu nyo",
        }
    }
}
