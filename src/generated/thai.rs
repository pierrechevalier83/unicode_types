/// \u{e00} → \u{e7f}\
///\
/// ก ข ฃ ค ฅ ฆ ง จ ฉ ช ซ ฌ ญ ฎ ฏ ฐ
/// ฑ ฒ ณ ด ต ถ ท ธ น บ ป ผ ฝ พ ฟ ภ
/// ม ย ร ฤ ล ฦ ว ศ ษ ส ห ฬ อ ฮ ฯ ะ
/// ั า ำ ิ ี ึ ื ุ ู ฺ ฿ เ แ โ ใ ไ
/// ๅ ๆ ็ ่ ้ ๊ ๋ ์ ํ ๎ ๏ ๐ ๑ ๒ ๓ ๔
/// ๕ ๖ ๗ ๘ ๙ ๚ ๛
pub mod constants {
    /// \u{e01}: 'ก'
    pub const THAI_CHARACTER_KO_KAI: char = 'ก';
    /// \u{e02}: 'ข'
    pub const THAI_CHARACTER_KHO_KHAI: char = 'ข';
    /// \u{e03}: 'ฃ'
    pub const THAI_CHARACTER_KHO_KHUAT: char = 'ฃ';
    /// \u{e04}: 'ค'
    pub const THAI_CHARACTER_KHO_KHWAI: char = 'ค';
    /// \u{e05}: 'ฅ'
    pub const THAI_CHARACTER_KHO_KHON: char = 'ฅ';
    /// \u{e06}: 'ฆ'
    pub const THAI_CHARACTER_KHO_RAKHANG: char = 'ฆ';
    /// \u{e07}: 'ง'
    pub const THAI_CHARACTER_NGO_NGU: char = 'ง';
    /// \u{e08}: 'จ'
    pub const THAI_CHARACTER_CHO_CHAN: char = 'จ';
    /// \u{e09}: 'ฉ'
    pub const THAI_CHARACTER_CHO_CHING: char = 'ฉ';
    /// \u{e0a}: 'ช'
    pub const THAI_CHARACTER_CHO_CHANG: char = 'ช';
    /// \u{e0b}: 'ซ'
    pub const THAI_CHARACTER_SO_SO: char = 'ซ';
    /// \u{e0c}: 'ฌ'
    pub const THAI_CHARACTER_CHO_CHOE: char = 'ฌ';
    /// \u{e0d}: 'ญ'
    pub const THAI_CHARACTER_YO_YING: char = 'ญ';
    /// \u{e0e}: 'ฎ'
    pub const THAI_CHARACTER_DO_CHADA: char = 'ฎ';
    /// \u{e0f}: 'ฏ'
    pub const THAI_CHARACTER_TO_PATAK: char = 'ฏ';
    /// \u{e10}: 'ฐ'
    pub const THAI_CHARACTER_THO_THAN: char = 'ฐ';
    /// \u{e11}: 'ฑ'
    pub const THAI_CHARACTER_THO_NANGMONTHO: char = 'ฑ';
    /// \u{e12}: 'ฒ'
    pub const THAI_CHARACTER_THO_PHUTHAO: char = 'ฒ';
    /// \u{e13}: 'ณ'
    pub const THAI_CHARACTER_NO_NEN: char = 'ณ';
    /// \u{e14}: 'ด'
    pub const THAI_CHARACTER_DO_DEK: char = 'ด';
    /// \u{e15}: 'ต'
    pub const THAI_CHARACTER_TO_TAO: char = 'ต';
    /// \u{e16}: 'ถ'
    pub const THAI_CHARACTER_THO_THUNG: char = 'ถ';
    /// \u{e17}: 'ท'
    pub const THAI_CHARACTER_THO_THAHAN: char = 'ท';
    /// \u{e18}: 'ธ'
    pub const THAI_CHARACTER_THO_THONG: char = 'ธ';
    /// \u{e19}: 'น'
    pub const THAI_CHARACTER_NO_NU: char = 'น';
    /// \u{e1a}: 'บ'
    pub const THAI_CHARACTER_BO_BAIMAI: char = 'บ';
    /// \u{e1b}: 'ป'
    pub const THAI_CHARACTER_PO_PLA: char = 'ป';
    /// \u{e1c}: 'ผ'
    pub const THAI_CHARACTER_PHO_PHUNG: char = 'ผ';
    /// \u{e1d}: 'ฝ'
    pub const THAI_CHARACTER_FO_FA: char = 'ฝ';
    /// \u{e1e}: 'พ'
    pub const THAI_CHARACTER_PHO_PHAN: char = 'พ';
    /// \u{e1f}: 'ฟ'
    pub const THAI_CHARACTER_FO_FAN: char = 'ฟ';
    /// \u{e20}: 'ภ'
    pub const THAI_CHARACTER_PHO_SAMPHAO: char = 'ภ';
    /// \u{e21}: 'ม'
    pub const THAI_CHARACTER_MO_MA: char = 'ม';
    /// \u{e22}: 'ย'
    pub const THAI_CHARACTER_YO_YAK: char = 'ย';
    /// \u{e23}: 'ร'
    pub const THAI_CHARACTER_RO_RUA: char = 'ร';
    /// \u{e24}: 'ฤ'
    pub const THAI_CHARACTER_RU: char = 'ฤ';
    /// \u{e25}: 'ล'
    pub const THAI_CHARACTER_LO_LING: char = 'ล';
    /// \u{e26}: 'ฦ'
    pub const THAI_CHARACTER_LU: char = 'ฦ';
    /// \u{e27}: 'ว'
    pub const THAI_CHARACTER_WO_WAEN: char = 'ว';
    /// \u{e28}: 'ศ'
    pub const THAI_CHARACTER_SO_SALA: char = 'ศ';
    /// \u{e29}: 'ษ'
    pub const THAI_CHARACTER_SO_RUSI: char = 'ษ';
    /// \u{e2a}: 'ส'
    pub const THAI_CHARACTER_SO_SUA: char = 'ส';
    /// \u{e2b}: 'ห'
    pub const THAI_CHARACTER_HO_HIP: char = 'ห';
    /// \u{e2c}: 'ฬ'
    pub const THAI_CHARACTER_LO_CHULA: char = 'ฬ';
    /// \u{e2d}: 'อ'
    pub const THAI_CHARACTER_O_ANG: char = 'อ';
    /// \u{e2e}: 'ฮ'
    pub const THAI_CHARACTER_HO_NOKHUK: char = 'ฮ';
    /// \u{e2f}: 'ฯ'
    pub const THAI_CHARACTER_PAIYANNOI: char = 'ฯ';
    /// \u{e30}: 'ะ'
    pub const THAI_CHARACTER_SARA_A: char = 'ะ';
    /// \u{e31}: 'ั'
    pub const THAI_CHARACTER_MAI_HAN_DASH_AKAT: char = 'ั';
    /// \u{e32}: 'า'
    pub const THAI_CHARACTER_SARA_AA: char = 'า';
    /// \u{e33}: 'ำ'
    pub const THAI_CHARACTER_SARA_AM: char = 'ำ';
    /// \u{e34}: 'ิ'
    pub const THAI_CHARACTER_SARA_I: char = 'ิ';
    /// \u{e35}: 'ี'
    pub const THAI_CHARACTER_SARA_II: char = 'ี';
    /// \u{e36}: 'ึ'
    pub const THAI_CHARACTER_SARA_UE: char = 'ึ';
    /// \u{e37}: 'ื'
    pub const THAI_CHARACTER_SARA_UEE: char = 'ื';
    /// \u{e38}: 'ุ'
    pub const THAI_CHARACTER_SARA_U: char = 'ุ';
    /// \u{e39}: 'ู'
    pub const THAI_CHARACTER_SARA_UU: char = 'ู';
    /// \u{e3a}: 'ฺ'
    pub const THAI_CHARACTER_PHINTHU: char = 'ฺ';
    /// \u{e3f}: '฿'
    pub const THAI_CURRENCY_SYMBOL_BAHT: char = '฿';
    /// \u{e40}: 'เ'
    pub const THAI_CHARACTER_SARA_E: char = 'เ';
    /// \u{e41}: 'แ'
    pub const THAI_CHARACTER_SARA_AE: char = 'แ';
    /// \u{e42}: 'โ'
    pub const THAI_CHARACTER_SARA_O: char = 'โ';
    /// \u{e43}: 'ใ'
    pub const THAI_CHARACTER_SARA_AI_MAIMUAN: char = 'ใ';
    /// \u{e44}: 'ไ'
    pub const THAI_CHARACTER_SARA_AI_MAIMALAI: char = 'ไ';
    /// \u{e45}: 'ๅ'
    pub const THAI_CHARACTER_LAKKHANGYAO: char = 'ๅ';
    /// \u{e46}: 'ๆ'
    pub const THAI_CHARACTER_MAIYAMOK: char = 'ๆ';
    /// \u{e47}: '็'
    pub const THAI_CHARACTER_MAITAIKHU: char = '็';
    /// \u{e48}: '่'
    pub const THAI_CHARACTER_MAI_EK: char = '่';
    /// \u{e49}: '้'
    pub const THAI_CHARACTER_MAI_THO: char = '้';
    /// \u{e4a}: '๊'
    pub const THAI_CHARACTER_MAI_TRI: char = '๊';
    /// \u{e4b}: '๋'
    pub const THAI_CHARACTER_MAI_CHATTAWA: char = '๋';
    /// \u{e4c}: '์'
    pub const THAI_CHARACTER_THANTHAKHAT: char = '์';
    /// \u{e4d}: 'ํ'
    pub const THAI_CHARACTER_NIKHAHIT: char = 'ํ';
    /// \u{e4e}: '๎'
    pub const THAI_CHARACTER_YAMAKKAN: char = '๎';
    /// \u{e4f}: '๏'
    pub const THAI_CHARACTER_FONGMAN: char = '๏';
    /// \u{e50}: '๐'
    pub const THAI_DIGIT_ZERO: char = '๐';
    /// \u{e51}: '๑'
    pub const THAI_DIGIT_ONE: char = '๑';
    /// \u{e52}: '๒'
    pub const THAI_DIGIT_TWO: char = '๒';
    /// \u{e53}: '๓'
    pub const THAI_DIGIT_THREE: char = '๓';
    /// \u{e54}: '๔'
    pub const THAI_DIGIT_FOUR: char = '๔';
    /// \u{e55}: '๕'
    pub const THAI_DIGIT_FIVE: char = '๕';
    /// \u{e56}: '๖'
    pub const THAI_DIGIT_SIX: char = '๖';
    /// \u{e57}: '๗'
    pub const THAI_DIGIT_SEVEN: char = '๗';
    /// \u{e58}: '๘'
    pub const THAI_DIGIT_EIGHT: char = '๘';
    /// \u{e59}: '๙'
    pub const THAI_DIGIT_NINE: char = '๙';
    /// \u{e5a}: '๚'
    pub const THAI_CHARACTER_ANGKHANKHU: char = '๚';
    /// \u{e5b}: '๛'
    pub const THAI_CHARACTER_KHOMUT: char = '๛';
}

/// \u{e00} → \u{e7f}\
///\
/// ก ข ฃ ค ฅ ฆ ง จ ฉ ช ซ ฌ ญ ฎ ฏ ฐ
/// ฑ ฒ ณ ด ต ถ ท ธ น บ ป ผ ฝ พ ฟ ภ
/// ม ย ร ฤ ล ฦ ว ศ ษ ส ห ฬ อ ฮ ฯ ะ
/// ั า ำ ิ ี ึ ื ุ ู ฺ ฿ เ แ โ ใ ไ
/// ๅ ๆ ็ ่ ้ ๊ ๋ ์ ํ ๎ ๏ ๐ ๑ ๒ ๓ ๔
/// ๕ ๖ ๗ ๘ ๙ ๚ ๛
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Thai {
    /// \u{e01}: 'ก'
    ThaiCharacterKoKai,
    /// \u{e02}: 'ข'
    ThaiCharacterKhoKhai,
    /// \u{e03}: 'ฃ'
    ThaiCharacterKhoKhuat,
    /// \u{e04}: 'ค'
    ThaiCharacterKhoKhwai,
    /// \u{e05}: 'ฅ'
    ThaiCharacterKhoKhon,
    /// \u{e06}: 'ฆ'
    ThaiCharacterKhoRakhang,
    /// \u{e07}: 'ง'
    ThaiCharacterNgoNgu,
    /// \u{e08}: 'จ'
    ThaiCharacterChoChan,
    /// \u{e09}: 'ฉ'
    ThaiCharacterChoChing,
    /// \u{e0a}: 'ช'
    ThaiCharacterChoChang,
    /// \u{e0b}: 'ซ'
    ThaiCharacterSoSo,
    /// \u{e0c}: 'ฌ'
    ThaiCharacterChoChoe,
    /// \u{e0d}: 'ญ'
    ThaiCharacterYoYing,
    /// \u{e0e}: 'ฎ'
    ThaiCharacterDoChada,
    /// \u{e0f}: 'ฏ'
    ThaiCharacterToPatak,
    /// \u{e10}: 'ฐ'
    ThaiCharacterThoThan,
    /// \u{e11}: 'ฑ'
    ThaiCharacterThoNangmontho,
    /// \u{e12}: 'ฒ'
    ThaiCharacterThoPhuthao,
    /// \u{e13}: 'ณ'
    ThaiCharacterNoNen,
    /// \u{e14}: 'ด'
    ThaiCharacterDoDek,
    /// \u{e15}: 'ต'
    ThaiCharacterToTao,
    /// \u{e16}: 'ถ'
    ThaiCharacterThoThung,
    /// \u{e17}: 'ท'
    ThaiCharacterThoThahan,
    /// \u{e18}: 'ธ'
    ThaiCharacterThoThong,
    /// \u{e19}: 'น'
    ThaiCharacterNoNu,
    /// \u{e1a}: 'บ'
    ThaiCharacterBoBaimai,
    /// \u{e1b}: 'ป'
    ThaiCharacterPoPla,
    /// \u{e1c}: 'ผ'
    ThaiCharacterPhoPhung,
    /// \u{e1d}: 'ฝ'
    ThaiCharacterFoFa,
    /// \u{e1e}: 'พ'
    ThaiCharacterPhoPhan,
    /// \u{e1f}: 'ฟ'
    ThaiCharacterFoFan,
    /// \u{e20}: 'ภ'
    ThaiCharacterPhoSamphao,
    /// \u{e21}: 'ม'
    ThaiCharacterMoMa,
    /// \u{e22}: 'ย'
    ThaiCharacterYoYak,
    /// \u{e23}: 'ร'
    ThaiCharacterRoRua,
    /// \u{e24}: 'ฤ'
    ThaiCharacterRu,
    /// \u{e25}: 'ล'
    ThaiCharacterLoLing,
    /// \u{e26}: 'ฦ'
    ThaiCharacterLu,
    /// \u{e27}: 'ว'
    ThaiCharacterWoWaen,
    /// \u{e28}: 'ศ'
    ThaiCharacterSoSala,
    /// \u{e29}: 'ษ'
    ThaiCharacterSoRusi,
    /// \u{e2a}: 'ส'
    ThaiCharacterSoSua,
    /// \u{e2b}: 'ห'
    ThaiCharacterHoHip,
    /// \u{e2c}: 'ฬ'
    ThaiCharacterLoChula,
    /// \u{e2d}: 'อ'
    ThaiCharacterOAng,
    /// \u{e2e}: 'ฮ'
    ThaiCharacterHoNokhuk,
    /// \u{e2f}: 'ฯ'
    ThaiCharacterPaiyannoi,
    /// \u{e30}: 'ะ'
    ThaiCharacterSaraA,
    /// \u{e31}: 'ั'
    ThaiCharacterMaiHanDashAkat,
    /// \u{e32}: 'า'
    ThaiCharacterSaraAa,
    /// \u{e33}: 'ำ'
    ThaiCharacterSaraAm,
    /// \u{e34}: 'ิ'
    ThaiCharacterSaraI,
    /// \u{e35}: 'ี'
    ThaiCharacterSaraIi,
    /// \u{e36}: 'ึ'
    ThaiCharacterSaraUe,
    /// \u{e37}: 'ื'
    ThaiCharacterSaraUee,
    /// \u{e38}: 'ุ'
    ThaiCharacterSaraU,
    /// \u{e39}: 'ู'
    ThaiCharacterSaraUu,
    /// \u{e3a}: 'ฺ'
    ThaiCharacterPhinthu,
    /// \u{e3f}: '฿'
    ThaiCurrencySymbolBaht,
    /// \u{e40}: 'เ'
    ThaiCharacterSaraE,
    /// \u{e41}: 'แ'
    ThaiCharacterSaraAe,
    /// \u{e42}: 'โ'
    ThaiCharacterSaraO,
    /// \u{e43}: 'ใ'
    ThaiCharacterSaraAiMaimuan,
    /// \u{e44}: 'ไ'
    ThaiCharacterSaraAiMaimalai,
    /// \u{e45}: 'ๅ'
    ThaiCharacterLakkhangyao,
    /// \u{e46}: 'ๆ'
    ThaiCharacterMaiyamok,
    /// \u{e47}: '็'
    ThaiCharacterMaitaikhu,
    /// \u{e48}: '่'
    ThaiCharacterMaiEk,
    /// \u{e49}: '้'
    ThaiCharacterMaiTho,
    /// \u{e4a}: '๊'
    ThaiCharacterMaiTri,
    /// \u{e4b}: '๋'
    ThaiCharacterMaiChattawa,
    /// \u{e4c}: '์'
    ThaiCharacterThanthakhat,
    /// \u{e4d}: 'ํ'
    ThaiCharacterNikhahit,
    /// \u{e4e}: '๎'
    ThaiCharacterYamakkan,
    /// \u{e4f}: '๏'
    ThaiCharacterFongman,
    /// \u{e50}: '๐'
    ThaiDigitZero,
    /// \u{e51}: '๑'
    ThaiDigitOne,
    /// \u{e52}: '๒'
    ThaiDigitTwo,
    /// \u{e53}: '๓'
    ThaiDigitThree,
    /// \u{e54}: '๔'
    ThaiDigitFour,
    /// \u{e55}: '๕'
    ThaiDigitFive,
    /// \u{e56}: '๖'
    ThaiDigitSix,
    /// \u{e57}: '๗'
    ThaiDigitSeven,
    /// \u{e58}: '๘'
    ThaiDigitEight,
    /// \u{e59}: '๙'
    ThaiDigitNine,
    /// \u{e5a}: '๚'
    ThaiCharacterAngkhankhu,
    /// \u{e5b}: '๛'
    ThaiCharacterKhomut,
}

impl Into<char> for Thai {
    fn into(self) -> char {
        use constants::*;
        match self {
            Thai::ThaiCharacterKoKai => THAI_CHARACTER_KO_KAI,
            Thai::ThaiCharacterKhoKhai => THAI_CHARACTER_KHO_KHAI,
            Thai::ThaiCharacterKhoKhuat => THAI_CHARACTER_KHO_KHUAT,
            Thai::ThaiCharacterKhoKhwai => THAI_CHARACTER_KHO_KHWAI,
            Thai::ThaiCharacterKhoKhon => THAI_CHARACTER_KHO_KHON,
            Thai::ThaiCharacterKhoRakhang => THAI_CHARACTER_KHO_RAKHANG,
            Thai::ThaiCharacterNgoNgu => THAI_CHARACTER_NGO_NGU,
            Thai::ThaiCharacterChoChan => THAI_CHARACTER_CHO_CHAN,
            Thai::ThaiCharacterChoChing => THAI_CHARACTER_CHO_CHING,
            Thai::ThaiCharacterChoChang => THAI_CHARACTER_CHO_CHANG,
            Thai::ThaiCharacterSoSo => THAI_CHARACTER_SO_SO,
            Thai::ThaiCharacterChoChoe => THAI_CHARACTER_CHO_CHOE,
            Thai::ThaiCharacterYoYing => THAI_CHARACTER_YO_YING,
            Thai::ThaiCharacterDoChada => THAI_CHARACTER_DO_CHADA,
            Thai::ThaiCharacterToPatak => THAI_CHARACTER_TO_PATAK,
            Thai::ThaiCharacterThoThan => THAI_CHARACTER_THO_THAN,
            Thai::ThaiCharacterThoNangmontho => THAI_CHARACTER_THO_NANGMONTHO,
            Thai::ThaiCharacterThoPhuthao => THAI_CHARACTER_THO_PHUTHAO,
            Thai::ThaiCharacterNoNen => THAI_CHARACTER_NO_NEN,
            Thai::ThaiCharacterDoDek => THAI_CHARACTER_DO_DEK,
            Thai::ThaiCharacterToTao => THAI_CHARACTER_TO_TAO,
            Thai::ThaiCharacterThoThung => THAI_CHARACTER_THO_THUNG,
            Thai::ThaiCharacterThoThahan => THAI_CHARACTER_THO_THAHAN,
            Thai::ThaiCharacterThoThong => THAI_CHARACTER_THO_THONG,
            Thai::ThaiCharacterNoNu => THAI_CHARACTER_NO_NU,
            Thai::ThaiCharacterBoBaimai => THAI_CHARACTER_BO_BAIMAI,
            Thai::ThaiCharacterPoPla => THAI_CHARACTER_PO_PLA,
            Thai::ThaiCharacterPhoPhung => THAI_CHARACTER_PHO_PHUNG,
            Thai::ThaiCharacterFoFa => THAI_CHARACTER_FO_FA,
            Thai::ThaiCharacterPhoPhan => THAI_CHARACTER_PHO_PHAN,
            Thai::ThaiCharacterFoFan => THAI_CHARACTER_FO_FAN,
            Thai::ThaiCharacterPhoSamphao => THAI_CHARACTER_PHO_SAMPHAO,
            Thai::ThaiCharacterMoMa => THAI_CHARACTER_MO_MA,
            Thai::ThaiCharacterYoYak => THAI_CHARACTER_YO_YAK,
            Thai::ThaiCharacterRoRua => THAI_CHARACTER_RO_RUA,
            Thai::ThaiCharacterRu => THAI_CHARACTER_RU,
            Thai::ThaiCharacterLoLing => THAI_CHARACTER_LO_LING,
            Thai::ThaiCharacterLu => THAI_CHARACTER_LU,
            Thai::ThaiCharacterWoWaen => THAI_CHARACTER_WO_WAEN,
            Thai::ThaiCharacterSoSala => THAI_CHARACTER_SO_SALA,
            Thai::ThaiCharacterSoRusi => THAI_CHARACTER_SO_RUSI,
            Thai::ThaiCharacterSoSua => THAI_CHARACTER_SO_SUA,
            Thai::ThaiCharacterHoHip => THAI_CHARACTER_HO_HIP,
            Thai::ThaiCharacterLoChula => THAI_CHARACTER_LO_CHULA,
            Thai::ThaiCharacterOAng => THAI_CHARACTER_O_ANG,
            Thai::ThaiCharacterHoNokhuk => THAI_CHARACTER_HO_NOKHUK,
            Thai::ThaiCharacterPaiyannoi => THAI_CHARACTER_PAIYANNOI,
            Thai::ThaiCharacterSaraA => THAI_CHARACTER_SARA_A,
            Thai::ThaiCharacterMaiHanDashAkat => THAI_CHARACTER_MAI_HAN_DASH_AKAT,
            Thai::ThaiCharacterSaraAa => THAI_CHARACTER_SARA_AA,
            Thai::ThaiCharacterSaraAm => THAI_CHARACTER_SARA_AM,
            Thai::ThaiCharacterSaraI => THAI_CHARACTER_SARA_I,
            Thai::ThaiCharacterSaraIi => THAI_CHARACTER_SARA_II,
            Thai::ThaiCharacterSaraUe => THAI_CHARACTER_SARA_UE,
            Thai::ThaiCharacterSaraUee => THAI_CHARACTER_SARA_UEE,
            Thai::ThaiCharacterSaraU => THAI_CHARACTER_SARA_U,
            Thai::ThaiCharacterSaraUu => THAI_CHARACTER_SARA_UU,
            Thai::ThaiCharacterPhinthu => THAI_CHARACTER_PHINTHU,
            Thai::ThaiCurrencySymbolBaht => THAI_CURRENCY_SYMBOL_BAHT,
            Thai::ThaiCharacterSaraE => THAI_CHARACTER_SARA_E,
            Thai::ThaiCharacterSaraAe => THAI_CHARACTER_SARA_AE,
            Thai::ThaiCharacterSaraO => THAI_CHARACTER_SARA_O,
            Thai::ThaiCharacterSaraAiMaimuan => THAI_CHARACTER_SARA_AI_MAIMUAN,
            Thai::ThaiCharacterSaraAiMaimalai => THAI_CHARACTER_SARA_AI_MAIMALAI,
            Thai::ThaiCharacterLakkhangyao => THAI_CHARACTER_LAKKHANGYAO,
            Thai::ThaiCharacterMaiyamok => THAI_CHARACTER_MAIYAMOK,
            Thai::ThaiCharacterMaitaikhu => THAI_CHARACTER_MAITAIKHU,
            Thai::ThaiCharacterMaiEk => THAI_CHARACTER_MAI_EK,
            Thai::ThaiCharacterMaiTho => THAI_CHARACTER_MAI_THO,
            Thai::ThaiCharacterMaiTri => THAI_CHARACTER_MAI_TRI,
            Thai::ThaiCharacterMaiChattawa => THAI_CHARACTER_MAI_CHATTAWA,
            Thai::ThaiCharacterThanthakhat => THAI_CHARACTER_THANTHAKHAT,
            Thai::ThaiCharacterNikhahit => THAI_CHARACTER_NIKHAHIT,
            Thai::ThaiCharacterYamakkan => THAI_CHARACTER_YAMAKKAN,
            Thai::ThaiCharacterFongman => THAI_CHARACTER_FONGMAN,
            Thai::ThaiDigitZero => THAI_DIGIT_ZERO,
            Thai::ThaiDigitOne => THAI_DIGIT_ONE,
            Thai::ThaiDigitTwo => THAI_DIGIT_TWO,
            Thai::ThaiDigitThree => THAI_DIGIT_THREE,
            Thai::ThaiDigitFour => THAI_DIGIT_FOUR,
            Thai::ThaiDigitFive => THAI_DIGIT_FIVE,
            Thai::ThaiDigitSix => THAI_DIGIT_SIX,
            Thai::ThaiDigitSeven => THAI_DIGIT_SEVEN,
            Thai::ThaiDigitEight => THAI_DIGIT_EIGHT,
            Thai::ThaiDigitNine => THAI_DIGIT_NINE,
            Thai::ThaiCharacterAngkhankhu => THAI_CHARACTER_ANGKHANKHU,
            Thai::ThaiCharacterKhomut => THAI_CHARACTER_KHOMUT,
        }
    }
}

impl std::convert::TryFrom<char> for Thai {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            THAI_CHARACTER_KO_KAI => Ok(Thai::ThaiCharacterKoKai),
            THAI_CHARACTER_KHO_KHAI => Ok(Thai::ThaiCharacterKhoKhai),
            THAI_CHARACTER_KHO_KHUAT => Ok(Thai::ThaiCharacterKhoKhuat),
            THAI_CHARACTER_KHO_KHWAI => Ok(Thai::ThaiCharacterKhoKhwai),
            THAI_CHARACTER_KHO_KHON => Ok(Thai::ThaiCharacterKhoKhon),
            THAI_CHARACTER_KHO_RAKHANG => Ok(Thai::ThaiCharacterKhoRakhang),
            THAI_CHARACTER_NGO_NGU => Ok(Thai::ThaiCharacterNgoNgu),
            THAI_CHARACTER_CHO_CHAN => Ok(Thai::ThaiCharacterChoChan),
            THAI_CHARACTER_CHO_CHING => Ok(Thai::ThaiCharacterChoChing),
            THAI_CHARACTER_CHO_CHANG => Ok(Thai::ThaiCharacterChoChang),
            THAI_CHARACTER_SO_SO => Ok(Thai::ThaiCharacterSoSo),
            THAI_CHARACTER_CHO_CHOE => Ok(Thai::ThaiCharacterChoChoe),
            THAI_CHARACTER_YO_YING => Ok(Thai::ThaiCharacterYoYing),
            THAI_CHARACTER_DO_CHADA => Ok(Thai::ThaiCharacterDoChada),
            THAI_CHARACTER_TO_PATAK => Ok(Thai::ThaiCharacterToPatak),
            THAI_CHARACTER_THO_THAN => Ok(Thai::ThaiCharacterThoThan),
            THAI_CHARACTER_THO_NANGMONTHO => Ok(Thai::ThaiCharacterThoNangmontho),
            THAI_CHARACTER_THO_PHUTHAO => Ok(Thai::ThaiCharacterThoPhuthao),
            THAI_CHARACTER_NO_NEN => Ok(Thai::ThaiCharacterNoNen),
            THAI_CHARACTER_DO_DEK => Ok(Thai::ThaiCharacterDoDek),
            THAI_CHARACTER_TO_TAO => Ok(Thai::ThaiCharacterToTao),
            THAI_CHARACTER_THO_THUNG => Ok(Thai::ThaiCharacterThoThung),
            THAI_CHARACTER_THO_THAHAN => Ok(Thai::ThaiCharacterThoThahan),
            THAI_CHARACTER_THO_THONG => Ok(Thai::ThaiCharacterThoThong),
            THAI_CHARACTER_NO_NU => Ok(Thai::ThaiCharacterNoNu),
            THAI_CHARACTER_BO_BAIMAI => Ok(Thai::ThaiCharacterBoBaimai),
            THAI_CHARACTER_PO_PLA => Ok(Thai::ThaiCharacterPoPla),
            THAI_CHARACTER_PHO_PHUNG => Ok(Thai::ThaiCharacterPhoPhung),
            THAI_CHARACTER_FO_FA => Ok(Thai::ThaiCharacterFoFa),
            THAI_CHARACTER_PHO_PHAN => Ok(Thai::ThaiCharacterPhoPhan),
            THAI_CHARACTER_FO_FAN => Ok(Thai::ThaiCharacterFoFan),
            THAI_CHARACTER_PHO_SAMPHAO => Ok(Thai::ThaiCharacterPhoSamphao),
            THAI_CHARACTER_MO_MA => Ok(Thai::ThaiCharacterMoMa),
            THAI_CHARACTER_YO_YAK => Ok(Thai::ThaiCharacterYoYak),
            THAI_CHARACTER_RO_RUA => Ok(Thai::ThaiCharacterRoRua),
            THAI_CHARACTER_RU => Ok(Thai::ThaiCharacterRu),
            THAI_CHARACTER_LO_LING => Ok(Thai::ThaiCharacterLoLing),
            THAI_CHARACTER_LU => Ok(Thai::ThaiCharacterLu),
            THAI_CHARACTER_WO_WAEN => Ok(Thai::ThaiCharacterWoWaen),
            THAI_CHARACTER_SO_SALA => Ok(Thai::ThaiCharacterSoSala),
            THAI_CHARACTER_SO_RUSI => Ok(Thai::ThaiCharacterSoRusi),
            THAI_CHARACTER_SO_SUA => Ok(Thai::ThaiCharacterSoSua),
            THAI_CHARACTER_HO_HIP => Ok(Thai::ThaiCharacterHoHip),
            THAI_CHARACTER_LO_CHULA => Ok(Thai::ThaiCharacterLoChula),
            THAI_CHARACTER_O_ANG => Ok(Thai::ThaiCharacterOAng),
            THAI_CHARACTER_HO_NOKHUK => Ok(Thai::ThaiCharacterHoNokhuk),
            THAI_CHARACTER_PAIYANNOI => Ok(Thai::ThaiCharacterPaiyannoi),
            THAI_CHARACTER_SARA_A => Ok(Thai::ThaiCharacterSaraA),
            THAI_CHARACTER_MAI_HAN_DASH_AKAT => Ok(Thai::ThaiCharacterMaiHanDashAkat),
            THAI_CHARACTER_SARA_AA => Ok(Thai::ThaiCharacterSaraAa),
            THAI_CHARACTER_SARA_AM => Ok(Thai::ThaiCharacterSaraAm),
            THAI_CHARACTER_SARA_I => Ok(Thai::ThaiCharacterSaraI),
            THAI_CHARACTER_SARA_II => Ok(Thai::ThaiCharacterSaraIi),
            THAI_CHARACTER_SARA_UE => Ok(Thai::ThaiCharacterSaraUe),
            THAI_CHARACTER_SARA_UEE => Ok(Thai::ThaiCharacterSaraUee),
            THAI_CHARACTER_SARA_U => Ok(Thai::ThaiCharacterSaraU),
            THAI_CHARACTER_SARA_UU => Ok(Thai::ThaiCharacterSaraUu),
            THAI_CHARACTER_PHINTHU => Ok(Thai::ThaiCharacterPhinthu),
            THAI_CURRENCY_SYMBOL_BAHT => Ok(Thai::ThaiCurrencySymbolBaht),
            THAI_CHARACTER_SARA_E => Ok(Thai::ThaiCharacterSaraE),
            THAI_CHARACTER_SARA_AE => Ok(Thai::ThaiCharacterSaraAe),
            THAI_CHARACTER_SARA_O => Ok(Thai::ThaiCharacterSaraO),
            THAI_CHARACTER_SARA_AI_MAIMUAN => Ok(Thai::ThaiCharacterSaraAiMaimuan),
            THAI_CHARACTER_SARA_AI_MAIMALAI => Ok(Thai::ThaiCharacterSaraAiMaimalai),
            THAI_CHARACTER_LAKKHANGYAO => Ok(Thai::ThaiCharacterLakkhangyao),
            THAI_CHARACTER_MAIYAMOK => Ok(Thai::ThaiCharacterMaiyamok),
            THAI_CHARACTER_MAITAIKHU => Ok(Thai::ThaiCharacterMaitaikhu),
            THAI_CHARACTER_MAI_EK => Ok(Thai::ThaiCharacterMaiEk),
            THAI_CHARACTER_MAI_THO => Ok(Thai::ThaiCharacterMaiTho),
            THAI_CHARACTER_MAI_TRI => Ok(Thai::ThaiCharacterMaiTri),
            THAI_CHARACTER_MAI_CHATTAWA => Ok(Thai::ThaiCharacterMaiChattawa),
            THAI_CHARACTER_THANTHAKHAT => Ok(Thai::ThaiCharacterThanthakhat),
            THAI_CHARACTER_NIKHAHIT => Ok(Thai::ThaiCharacterNikhahit),
            THAI_CHARACTER_YAMAKKAN => Ok(Thai::ThaiCharacterYamakkan),
            THAI_CHARACTER_FONGMAN => Ok(Thai::ThaiCharacterFongman),
            THAI_DIGIT_ZERO => Ok(Thai::ThaiDigitZero),
            THAI_DIGIT_ONE => Ok(Thai::ThaiDigitOne),
            THAI_DIGIT_TWO => Ok(Thai::ThaiDigitTwo),
            THAI_DIGIT_THREE => Ok(Thai::ThaiDigitThree),
            THAI_DIGIT_FOUR => Ok(Thai::ThaiDigitFour),
            THAI_DIGIT_FIVE => Ok(Thai::ThaiDigitFive),
            THAI_DIGIT_SIX => Ok(Thai::ThaiDigitSix),
            THAI_DIGIT_SEVEN => Ok(Thai::ThaiDigitSeven),
            THAI_DIGIT_EIGHT => Ok(Thai::ThaiDigitEight),
            THAI_DIGIT_NINE => Ok(Thai::ThaiDigitNine),
            THAI_CHARACTER_ANGKHANKHU => Ok(Thai::ThaiCharacterAngkhankhu),
            THAI_CHARACTER_KHOMUT => Ok(Thai::ThaiCharacterKhomut),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Thai {
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

impl std::convert::TryFrom<u32> for Thai {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Thai {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Thai {
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        Thai::ThaiCharacterKoKai
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Thai::ThaiCharacterKoKai => "thai character ko kai",
            Thai::ThaiCharacterKhoKhai => "thai character kho khai",
            Thai::ThaiCharacterKhoKhuat => "thai character kho khuat",
            Thai::ThaiCharacterKhoKhwai => "thai character kho khwai",
            Thai::ThaiCharacterKhoKhon => "thai character kho khon",
            Thai::ThaiCharacterKhoRakhang => "thai character kho rakhang",
            Thai::ThaiCharacterNgoNgu => "thai character ngo ngu",
            Thai::ThaiCharacterChoChan => "thai character cho chan",
            Thai::ThaiCharacterChoChing => "thai character cho ching",
            Thai::ThaiCharacterChoChang => "thai character cho chang",
            Thai::ThaiCharacterSoSo => "thai character so so",
            Thai::ThaiCharacterChoChoe => "thai character cho choe",
            Thai::ThaiCharacterYoYing => "thai character yo ying",
            Thai::ThaiCharacterDoChada => "thai character do chada",
            Thai::ThaiCharacterToPatak => "thai character to patak",
            Thai::ThaiCharacterThoThan => "thai character tho than",
            Thai::ThaiCharacterThoNangmontho => "thai character tho nangmontho",
            Thai::ThaiCharacterThoPhuthao => "thai character tho phuthao",
            Thai::ThaiCharacterNoNen => "thai character no nen",
            Thai::ThaiCharacterDoDek => "thai character do dek",
            Thai::ThaiCharacterToTao => "thai character to tao",
            Thai::ThaiCharacterThoThung => "thai character tho thung",
            Thai::ThaiCharacterThoThahan => "thai character tho thahan",
            Thai::ThaiCharacterThoThong => "thai character tho thong",
            Thai::ThaiCharacterNoNu => "thai character no nu",
            Thai::ThaiCharacterBoBaimai => "thai character bo baimai",
            Thai::ThaiCharacterPoPla => "thai character po pla",
            Thai::ThaiCharacterPhoPhung => "thai character pho phung",
            Thai::ThaiCharacterFoFa => "thai character fo fa",
            Thai::ThaiCharacterPhoPhan => "thai character pho phan",
            Thai::ThaiCharacterFoFan => "thai character fo fan",
            Thai::ThaiCharacterPhoSamphao => "thai character pho samphao",
            Thai::ThaiCharacterMoMa => "thai character mo ma",
            Thai::ThaiCharacterYoYak => "thai character yo yak",
            Thai::ThaiCharacterRoRua => "thai character ro rua",
            Thai::ThaiCharacterRu => "thai character ru",
            Thai::ThaiCharacterLoLing => "thai character lo ling",
            Thai::ThaiCharacterLu => "thai character lu",
            Thai::ThaiCharacterWoWaen => "thai character wo waen",
            Thai::ThaiCharacterSoSala => "thai character so sala",
            Thai::ThaiCharacterSoRusi => "thai character so rusi",
            Thai::ThaiCharacterSoSua => "thai character so sua",
            Thai::ThaiCharacterHoHip => "thai character ho hip",
            Thai::ThaiCharacterLoChula => "thai character lo chula",
            Thai::ThaiCharacterOAng => "thai character o ang",
            Thai::ThaiCharacterHoNokhuk => "thai character ho nokhuk",
            Thai::ThaiCharacterPaiyannoi => "thai character paiyannoi",
            Thai::ThaiCharacterSaraA => "thai character sara a",
            Thai::ThaiCharacterMaiHanDashAkat => "thai character mai han-akat",
            Thai::ThaiCharacterSaraAa => "thai character sara aa",
            Thai::ThaiCharacterSaraAm => "thai character sara am",
            Thai::ThaiCharacterSaraI => "thai character sara i",
            Thai::ThaiCharacterSaraIi => "thai character sara ii",
            Thai::ThaiCharacterSaraUe => "thai character sara ue",
            Thai::ThaiCharacterSaraUee => "thai character sara uee",
            Thai::ThaiCharacterSaraU => "thai character sara u",
            Thai::ThaiCharacterSaraUu => "thai character sara uu",
            Thai::ThaiCharacterPhinthu => "thai character phinthu",
            Thai::ThaiCurrencySymbolBaht => "thai currency symbol baht",
            Thai::ThaiCharacterSaraE => "thai character sara e",
            Thai::ThaiCharacterSaraAe => "thai character sara ae",
            Thai::ThaiCharacterSaraO => "thai character sara o",
            Thai::ThaiCharacterSaraAiMaimuan => "thai character sara ai maimuan",
            Thai::ThaiCharacterSaraAiMaimalai => "thai character sara ai maimalai",
            Thai::ThaiCharacterLakkhangyao => "thai character lakkhangyao",
            Thai::ThaiCharacterMaiyamok => "thai character maiyamok",
            Thai::ThaiCharacterMaitaikhu => "thai character maitaikhu",
            Thai::ThaiCharacterMaiEk => "thai character mai ek",
            Thai::ThaiCharacterMaiTho => "thai character mai tho",
            Thai::ThaiCharacterMaiTri => "thai character mai tri",
            Thai::ThaiCharacterMaiChattawa => "thai character mai chattawa",
            Thai::ThaiCharacterThanthakhat => "thai character thanthakhat",
            Thai::ThaiCharacterNikhahit => "thai character nikhahit",
            Thai::ThaiCharacterYamakkan => "thai character yamakkan",
            Thai::ThaiCharacterFongman => "thai character fongman",
            Thai::ThaiDigitZero => "thai digit zero",
            Thai::ThaiDigitOne => "thai digit one",
            Thai::ThaiDigitTwo => "thai digit two",
            Thai::ThaiDigitThree => "thai digit three",
            Thai::ThaiDigitFour => "thai digit four",
            Thai::ThaiDigitFive => "thai digit five",
            Thai::ThaiDigitSix => "thai digit six",
            Thai::ThaiDigitSeven => "thai digit seven",
            Thai::ThaiDigitEight => "thai digit eight",
            Thai::ThaiDigitNine => "thai digit nine",
            Thai::ThaiCharacterAngkhankhu => "thai character angkhankhu",
            Thai::ThaiCharacterKhomut => "thai character khomut",
        }
    }
}
