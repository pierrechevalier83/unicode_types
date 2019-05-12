
/// An enum to represent all characters in the Thai block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Thai {
    /// \u{e01}: 'ก'
    CharacterKoKai,
    /// \u{e02}: 'ข'
    CharacterKhoKhai,
    /// \u{e03}: 'ฃ'
    CharacterKhoKhuat,
    /// \u{e04}: 'ค'
    CharacterKhoKhwai,
    /// \u{e05}: 'ฅ'
    CharacterKhoKhon,
    /// \u{e06}: 'ฆ'
    CharacterKhoRakhang,
    /// \u{e07}: 'ง'
    CharacterNgoNgu,
    /// \u{e08}: 'จ'
    CharacterChoChan,
    /// \u{e09}: 'ฉ'
    CharacterChoChing,
    /// \u{e0a}: 'ช'
    CharacterChoChang,
    /// \u{e0b}: 'ซ'
    CharacterSoSo,
    /// \u{e0c}: 'ฌ'
    CharacterChoChoe,
    /// \u{e0d}: 'ญ'
    CharacterYoYing,
    /// \u{e0e}: 'ฎ'
    CharacterDoChada,
    /// \u{e0f}: 'ฏ'
    CharacterToPatak,
    /// \u{e10}: 'ฐ'
    CharacterThoThan,
    /// \u{e11}: 'ฑ'
    CharacterThoNangmontho,
    /// \u{e12}: 'ฒ'
    CharacterThoPhuthao,
    /// \u{e13}: 'ณ'
    CharacterNoNen,
    /// \u{e14}: 'ด'
    CharacterDoDek,
    /// \u{e15}: 'ต'
    CharacterToTao,
    /// \u{e16}: 'ถ'
    CharacterThoThung,
    /// \u{e17}: 'ท'
    CharacterThoThahan,
    /// \u{e18}: 'ธ'
    CharacterThoThong,
    /// \u{e19}: 'น'
    CharacterNoNu,
    /// \u{e1a}: 'บ'
    CharacterBoBaimai,
    /// \u{e1b}: 'ป'
    CharacterPoPla,
    /// \u{e1c}: 'ผ'
    CharacterPhoPhung,
    /// \u{e1d}: 'ฝ'
    CharacterFoFa,
    /// \u{e1e}: 'พ'
    CharacterPhoPhan,
    /// \u{e1f}: 'ฟ'
    CharacterFoFan,
    /// \u{e20}: 'ภ'
    CharacterPhoSamphao,
    /// \u{e21}: 'ม'
    CharacterMoMa,
    /// \u{e22}: 'ย'
    CharacterYoYak,
    /// \u{e23}: 'ร'
    CharacterRoRua,
    /// \u{e24}: 'ฤ'
    CharacterRu,
    /// \u{e25}: 'ล'
    CharacterLoLing,
    /// \u{e26}: 'ฦ'
    CharacterLu,
    /// \u{e27}: 'ว'
    CharacterWoWaen,
    /// \u{e28}: 'ศ'
    CharacterSoSala,
    /// \u{e29}: 'ษ'
    CharacterSoRusi,
    /// \u{e2a}: 'ส'
    CharacterSoSua,
    /// \u{e2b}: 'ห'
    CharacterHoHip,
    /// \u{e2c}: 'ฬ'
    CharacterLoChula,
    /// \u{e2d}: 'อ'
    CharacterOAng,
    /// \u{e2e}: 'ฮ'
    CharacterHoNokhuk,
    /// \u{e2f}: 'ฯ'
    CharacterPaiyannoi,
    /// \u{e30}: 'ะ'
    CharacterSaraA,
    /// \u{e31}: 'ั'
    CharacterMaiHanDashAkat,
    /// \u{e32}: 'า'
    CharacterSaraAa,
    /// \u{e33}: 'ำ'
    CharacterSaraAm,
    /// \u{e34}: 'ิ'
    CharacterSaraI,
    /// \u{e35}: 'ี'
    CharacterSaraIi,
    /// \u{e36}: 'ึ'
    CharacterSaraUe,
    /// \u{e37}: 'ื'
    CharacterSaraUee,
    /// \u{e38}: 'ุ'
    CharacterSaraU,
    /// \u{e39}: 'ู'
    CharacterSaraUu,
    /// \u{e3a}: 'ฺ'
    CharacterPhinthu,
    /// \u{e3f}: '฿'
    CurrencySymbolBaht,
    /// \u{e40}: 'เ'
    CharacterSaraE,
    /// \u{e41}: 'แ'
    CharacterSaraAe,
    /// \u{e42}: 'โ'
    CharacterSaraO,
    /// \u{e43}: 'ใ'
    CharacterSaraAiMaimuan,
    /// \u{e44}: 'ไ'
    CharacterSaraAiMaimalai,
    /// \u{e45}: 'ๅ'
    CharacterLakkhangyao,
    /// \u{e46}: 'ๆ'
    CharacterMaiyamok,
    /// \u{e47}: '็'
    CharacterMaitaikhu,
    /// \u{e48}: '่'
    CharacterMaiEk,
    /// \u{e49}: '้'
    CharacterMaiTho,
    /// \u{e4a}: '๊'
    CharacterMaiTri,
    /// \u{e4b}: '๋'
    CharacterMaiChattawa,
    /// \u{e4c}: '์'
    CharacterThanthakhat,
    /// \u{e4d}: 'ํ'
    CharacterNikhahit,
    /// \u{e4e}: '๎'
    CharacterYamakkan,
    /// \u{e4f}: '๏'
    CharacterFongman,
    /// \u{e50}: '๐'
    DigitZero,
    /// \u{e51}: '๑'
    DigitOne,
    /// \u{e52}: '๒'
    DigitTwo,
    /// \u{e53}: '๓'
    DigitThree,
    /// \u{e54}: '๔'
    DigitFour,
    /// \u{e55}: '๕'
    DigitFive,
    /// \u{e56}: '๖'
    DigitSix,
    /// \u{e57}: '๗'
    DigitSeven,
    /// \u{e58}: '๘'
    DigitEight,
    /// \u{e59}: '๙'
    DigitNine,
    /// \u{e5a}: '๚'
    CharacterAngkhankhu,
    /// \u{e5b}: '๛'
    CharacterKhomut,
}

impl Into<char> for Thai {
    fn into(self) -> char {
        match self {
            Thai::CharacterKoKai => 'ก',
            Thai::CharacterKhoKhai => 'ข',
            Thai::CharacterKhoKhuat => 'ฃ',
            Thai::CharacterKhoKhwai => 'ค',
            Thai::CharacterKhoKhon => 'ฅ',
            Thai::CharacterKhoRakhang => 'ฆ',
            Thai::CharacterNgoNgu => 'ง',
            Thai::CharacterChoChan => 'จ',
            Thai::CharacterChoChing => 'ฉ',
            Thai::CharacterChoChang => 'ช',
            Thai::CharacterSoSo => 'ซ',
            Thai::CharacterChoChoe => 'ฌ',
            Thai::CharacterYoYing => 'ญ',
            Thai::CharacterDoChada => 'ฎ',
            Thai::CharacterToPatak => 'ฏ',
            Thai::CharacterThoThan => 'ฐ',
            Thai::CharacterThoNangmontho => 'ฑ',
            Thai::CharacterThoPhuthao => 'ฒ',
            Thai::CharacterNoNen => 'ณ',
            Thai::CharacterDoDek => 'ด',
            Thai::CharacterToTao => 'ต',
            Thai::CharacterThoThung => 'ถ',
            Thai::CharacterThoThahan => 'ท',
            Thai::CharacterThoThong => 'ธ',
            Thai::CharacterNoNu => 'น',
            Thai::CharacterBoBaimai => 'บ',
            Thai::CharacterPoPla => 'ป',
            Thai::CharacterPhoPhung => 'ผ',
            Thai::CharacterFoFa => 'ฝ',
            Thai::CharacterPhoPhan => 'พ',
            Thai::CharacterFoFan => 'ฟ',
            Thai::CharacterPhoSamphao => 'ภ',
            Thai::CharacterMoMa => 'ม',
            Thai::CharacterYoYak => 'ย',
            Thai::CharacterRoRua => 'ร',
            Thai::CharacterRu => 'ฤ',
            Thai::CharacterLoLing => 'ล',
            Thai::CharacterLu => 'ฦ',
            Thai::CharacterWoWaen => 'ว',
            Thai::CharacterSoSala => 'ศ',
            Thai::CharacterSoRusi => 'ษ',
            Thai::CharacterSoSua => 'ส',
            Thai::CharacterHoHip => 'ห',
            Thai::CharacterLoChula => 'ฬ',
            Thai::CharacterOAng => 'อ',
            Thai::CharacterHoNokhuk => 'ฮ',
            Thai::CharacterPaiyannoi => 'ฯ',
            Thai::CharacterSaraA => 'ะ',
            Thai::CharacterMaiHanDashAkat => 'ั',
            Thai::CharacterSaraAa => 'า',
            Thai::CharacterSaraAm => 'ำ',
            Thai::CharacterSaraI => 'ิ',
            Thai::CharacterSaraIi => 'ี',
            Thai::CharacterSaraUe => 'ึ',
            Thai::CharacterSaraUee => 'ื',
            Thai::CharacterSaraU => 'ุ',
            Thai::CharacterSaraUu => 'ู',
            Thai::CharacterPhinthu => 'ฺ',
            Thai::CurrencySymbolBaht => '฿',
            Thai::CharacterSaraE => 'เ',
            Thai::CharacterSaraAe => 'แ',
            Thai::CharacterSaraO => 'โ',
            Thai::CharacterSaraAiMaimuan => 'ใ',
            Thai::CharacterSaraAiMaimalai => 'ไ',
            Thai::CharacterLakkhangyao => 'ๅ',
            Thai::CharacterMaiyamok => 'ๆ',
            Thai::CharacterMaitaikhu => '็',
            Thai::CharacterMaiEk => '่',
            Thai::CharacterMaiTho => '้',
            Thai::CharacterMaiTri => '๊',
            Thai::CharacterMaiChattawa => '๋',
            Thai::CharacterThanthakhat => '์',
            Thai::CharacterNikhahit => 'ํ',
            Thai::CharacterYamakkan => '๎',
            Thai::CharacterFongman => '๏',
            Thai::DigitZero => '๐',
            Thai::DigitOne => '๑',
            Thai::DigitTwo => '๒',
            Thai::DigitThree => '๓',
            Thai::DigitFour => '๔',
            Thai::DigitFive => '๕',
            Thai::DigitSix => '๖',
            Thai::DigitSeven => '๗',
            Thai::DigitEight => '๘',
            Thai::DigitNine => '๙',
            Thai::CharacterAngkhankhu => '๚',
            Thai::CharacterKhomut => '๛',
        }
    }
}

impl std::convert::TryFrom<char> for Thai {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ก' => Ok(Thai::CharacterKoKai),
            'ข' => Ok(Thai::CharacterKhoKhai),
            'ฃ' => Ok(Thai::CharacterKhoKhuat),
            'ค' => Ok(Thai::CharacterKhoKhwai),
            'ฅ' => Ok(Thai::CharacterKhoKhon),
            'ฆ' => Ok(Thai::CharacterKhoRakhang),
            'ง' => Ok(Thai::CharacterNgoNgu),
            'จ' => Ok(Thai::CharacterChoChan),
            'ฉ' => Ok(Thai::CharacterChoChing),
            'ช' => Ok(Thai::CharacterChoChang),
            'ซ' => Ok(Thai::CharacterSoSo),
            'ฌ' => Ok(Thai::CharacterChoChoe),
            'ญ' => Ok(Thai::CharacterYoYing),
            'ฎ' => Ok(Thai::CharacterDoChada),
            'ฏ' => Ok(Thai::CharacterToPatak),
            'ฐ' => Ok(Thai::CharacterThoThan),
            'ฑ' => Ok(Thai::CharacterThoNangmontho),
            'ฒ' => Ok(Thai::CharacterThoPhuthao),
            'ณ' => Ok(Thai::CharacterNoNen),
            'ด' => Ok(Thai::CharacterDoDek),
            'ต' => Ok(Thai::CharacterToTao),
            'ถ' => Ok(Thai::CharacterThoThung),
            'ท' => Ok(Thai::CharacterThoThahan),
            'ธ' => Ok(Thai::CharacterThoThong),
            'น' => Ok(Thai::CharacterNoNu),
            'บ' => Ok(Thai::CharacterBoBaimai),
            'ป' => Ok(Thai::CharacterPoPla),
            'ผ' => Ok(Thai::CharacterPhoPhung),
            'ฝ' => Ok(Thai::CharacterFoFa),
            'พ' => Ok(Thai::CharacterPhoPhan),
            'ฟ' => Ok(Thai::CharacterFoFan),
            'ภ' => Ok(Thai::CharacterPhoSamphao),
            'ม' => Ok(Thai::CharacterMoMa),
            'ย' => Ok(Thai::CharacterYoYak),
            'ร' => Ok(Thai::CharacterRoRua),
            'ฤ' => Ok(Thai::CharacterRu),
            'ล' => Ok(Thai::CharacterLoLing),
            'ฦ' => Ok(Thai::CharacterLu),
            'ว' => Ok(Thai::CharacterWoWaen),
            'ศ' => Ok(Thai::CharacterSoSala),
            'ษ' => Ok(Thai::CharacterSoRusi),
            'ส' => Ok(Thai::CharacterSoSua),
            'ห' => Ok(Thai::CharacterHoHip),
            'ฬ' => Ok(Thai::CharacterLoChula),
            'อ' => Ok(Thai::CharacterOAng),
            'ฮ' => Ok(Thai::CharacterHoNokhuk),
            'ฯ' => Ok(Thai::CharacterPaiyannoi),
            'ะ' => Ok(Thai::CharacterSaraA),
            'ั' => Ok(Thai::CharacterMaiHanDashAkat),
            'า' => Ok(Thai::CharacterSaraAa),
            'ำ' => Ok(Thai::CharacterSaraAm),
            'ิ' => Ok(Thai::CharacterSaraI),
            'ี' => Ok(Thai::CharacterSaraIi),
            'ึ' => Ok(Thai::CharacterSaraUe),
            'ื' => Ok(Thai::CharacterSaraUee),
            'ุ' => Ok(Thai::CharacterSaraU),
            'ู' => Ok(Thai::CharacterSaraUu),
            'ฺ' => Ok(Thai::CharacterPhinthu),
            '฿' => Ok(Thai::CurrencySymbolBaht),
            'เ' => Ok(Thai::CharacterSaraE),
            'แ' => Ok(Thai::CharacterSaraAe),
            'โ' => Ok(Thai::CharacterSaraO),
            'ใ' => Ok(Thai::CharacterSaraAiMaimuan),
            'ไ' => Ok(Thai::CharacterSaraAiMaimalai),
            'ๅ' => Ok(Thai::CharacterLakkhangyao),
            'ๆ' => Ok(Thai::CharacterMaiyamok),
            '็' => Ok(Thai::CharacterMaitaikhu),
            '่' => Ok(Thai::CharacterMaiEk),
            '้' => Ok(Thai::CharacterMaiTho),
            '๊' => Ok(Thai::CharacterMaiTri),
            '๋' => Ok(Thai::CharacterMaiChattawa),
            '์' => Ok(Thai::CharacterThanthakhat),
            'ํ' => Ok(Thai::CharacterNikhahit),
            '๎' => Ok(Thai::CharacterYamakkan),
            '๏' => Ok(Thai::CharacterFongman),
            '๐' => Ok(Thai::DigitZero),
            '๑' => Ok(Thai::DigitOne),
            '๒' => Ok(Thai::DigitTwo),
            '๓' => Ok(Thai::DigitThree),
            '๔' => Ok(Thai::DigitFour),
            '๕' => Ok(Thai::DigitFive),
            '๖' => Ok(Thai::DigitSix),
            '๗' => Ok(Thai::DigitSeven),
            '๘' => Ok(Thai::DigitEight),
            '๙' => Ok(Thai::DigitNine),
            '๚' => Ok(Thai::CharacterAngkhankhu),
            '๛' => Ok(Thai::CharacterKhomut),
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
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Thai::CharacterKoKai
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Thai{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
