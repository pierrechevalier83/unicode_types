/// \u{11150} → \u{1117f}\
///\
/// 𑅐 𑅑 𑅒 𑅓 𑅔 𑅕 𑅖 𑅗 𑅘 𑅙 𑅚 𑅛 𑅜 𑅝 𑅞 𑅟
/// 𑅠 𑅡 𑅢 𑅣 𑅤 𑅥 𑅦 𑅧 𑅨 𑅩 𑅪 𑅫 𑅬 𑅭 𑅮 𑅯
/// 𑅰 𑅱 𑅲 𑅳 𑅴 𑅵 𑅶
pub mod constants {
    /// \u{11150}: '𑅐'
    pub const MAHAJANI_LETTER_A: char = '𑅐';
    /// \u{11151}: '𑅑'
    pub const MAHAJANI_LETTER_I: char = '𑅑';
    /// \u{11152}: '𑅒'
    pub const MAHAJANI_LETTER_U: char = '𑅒';
    /// \u{11153}: '𑅓'
    pub const MAHAJANI_LETTER_E: char = '𑅓';
    /// \u{11154}: '𑅔'
    pub const MAHAJANI_LETTER_O: char = '𑅔';
    /// \u{11155}: '𑅕'
    pub const MAHAJANI_LETTER_KA: char = '𑅕';
    /// \u{11156}: '𑅖'
    pub const MAHAJANI_LETTER_KHA: char = '𑅖';
    /// \u{11157}: '𑅗'
    pub const MAHAJANI_LETTER_GA: char = '𑅗';
    /// \u{11158}: '𑅘'
    pub const MAHAJANI_LETTER_GHA: char = '𑅘';
    /// \u{11159}: '𑅙'
    pub const MAHAJANI_LETTER_CA: char = '𑅙';
    /// \u{1115a}: '𑅚'
    pub const MAHAJANI_LETTER_CHA: char = '𑅚';
    /// \u{1115b}: '𑅛'
    pub const MAHAJANI_LETTER_JA: char = '𑅛';
    /// \u{1115c}: '𑅜'
    pub const MAHAJANI_LETTER_JHA: char = '𑅜';
    /// \u{1115d}: '𑅝'
    pub const MAHAJANI_LETTER_NYA: char = '𑅝';
    /// \u{1115e}: '𑅞'
    pub const MAHAJANI_LETTER_TTA: char = '𑅞';
    /// \u{1115f}: '𑅟'
    pub const MAHAJANI_LETTER_TTHA: char = '𑅟';
    /// \u{11160}: '𑅠'
    pub const MAHAJANI_LETTER_DDA: char = '𑅠';
    /// \u{11161}: '𑅡'
    pub const MAHAJANI_LETTER_DDHA: char = '𑅡';
    /// \u{11162}: '𑅢'
    pub const MAHAJANI_LETTER_NNA: char = '𑅢';
    /// \u{11163}: '𑅣'
    pub const MAHAJANI_LETTER_TA: char = '𑅣';
    /// \u{11164}: '𑅤'
    pub const MAHAJANI_LETTER_THA: char = '𑅤';
    /// \u{11165}: '𑅥'
    pub const MAHAJANI_LETTER_DA: char = '𑅥';
    /// \u{11166}: '𑅦'
    pub const MAHAJANI_LETTER_DHA: char = '𑅦';
    /// \u{11167}: '𑅧'
    pub const MAHAJANI_LETTER_NA: char = '𑅧';
    /// \u{11168}: '𑅨'
    pub const MAHAJANI_LETTER_PA: char = '𑅨';
    /// \u{11169}: '𑅩'
    pub const MAHAJANI_LETTER_PHA: char = '𑅩';
    /// \u{1116a}: '𑅪'
    pub const MAHAJANI_LETTER_BA: char = '𑅪';
    /// \u{1116b}: '𑅫'
    pub const MAHAJANI_LETTER_BHA: char = '𑅫';
    /// \u{1116c}: '𑅬'
    pub const MAHAJANI_LETTER_MA: char = '𑅬';
    /// \u{1116d}: '𑅭'
    pub const MAHAJANI_LETTER_RA: char = '𑅭';
    /// \u{1116e}: '𑅮'
    pub const MAHAJANI_LETTER_LA: char = '𑅮';
    /// \u{1116f}: '𑅯'
    pub const MAHAJANI_LETTER_VA: char = '𑅯';
    /// \u{11170}: '𑅰'
    pub const MAHAJANI_LETTER_SA: char = '𑅰';
    /// \u{11171}: '𑅱'
    pub const MAHAJANI_LETTER_HA: char = '𑅱';
    /// \u{11172}: '𑅲'
    pub const MAHAJANI_LETTER_RRA: char = '𑅲';
    /// \u{11173}: '𑅳'
    pub const MAHAJANI_SIGN_NUKTA: char = '𑅳';
    /// \u{11174}: '𑅴'
    pub const MAHAJANI_ABBREVIATION_SIGN: char = '𑅴';
    /// \u{11175}: '𑅵'
    pub const MAHAJANI_SECTION_MARK: char = '𑅵';
    /// \u{11176}: '𑅶'
    pub const MAHAJANI_LIGATURE_SHRI: char = '𑅶';
}

/// \u{11150} → \u{1117f}\
///\
/// 𑅐 𑅑 𑅒 𑅓 𑅔 𑅕 𑅖 𑅗 𑅘 𑅙 𑅚 𑅛 𑅜 𑅝 𑅞 𑅟
/// 𑅠 𑅡 𑅢 𑅣 𑅤 𑅥 𑅦 𑅧 𑅨 𑅩 𑅪 𑅫 𑅬 𑅭 𑅮 𑅯
/// 𑅰 𑅱 𑅲 𑅳 𑅴 𑅵 𑅶
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Mahajani {
    /// \u{11150}: '𑅐'
    MahajaniLetterA,
    /// \u{11151}: '𑅑'
    MahajaniLetterI,
    /// \u{11152}: '𑅒'
    MahajaniLetterU,
    /// \u{11153}: '𑅓'
    MahajaniLetterE,
    /// \u{11154}: '𑅔'
    MahajaniLetterO,
    /// \u{11155}: '𑅕'
    MahajaniLetterKa,
    /// \u{11156}: '𑅖'
    MahajaniLetterKha,
    /// \u{11157}: '𑅗'
    MahajaniLetterGa,
    /// \u{11158}: '𑅘'
    MahajaniLetterGha,
    /// \u{11159}: '𑅙'
    MahajaniLetterCa,
    /// \u{1115a}: '𑅚'
    MahajaniLetterCha,
    /// \u{1115b}: '𑅛'
    MahajaniLetterJa,
    /// \u{1115c}: '𑅜'
    MahajaniLetterJha,
    /// \u{1115d}: '𑅝'
    MahajaniLetterNya,
    /// \u{1115e}: '𑅞'
    MahajaniLetterTta,
    /// \u{1115f}: '𑅟'
    MahajaniLetterTtha,
    /// \u{11160}: '𑅠'
    MahajaniLetterDda,
    /// \u{11161}: '𑅡'
    MahajaniLetterDdha,
    /// \u{11162}: '𑅢'
    MahajaniLetterNna,
    /// \u{11163}: '𑅣'
    MahajaniLetterTa,
    /// \u{11164}: '𑅤'
    MahajaniLetterTha,
    /// \u{11165}: '𑅥'
    MahajaniLetterDa,
    /// \u{11166}: '𑅦'
    MahajaniLetterDha,
    /// \u{11167}: '𑅧'
    MahajaniLetterNa,
    /// \u{11168}: '𑅨'
    MahajaniLetterPa,
    /// \u{11169}: '𑅩'
    MahajaniLetterPha,
    /// \u{1116a}: '𑅪'
    MahajaniLetterBa,
    /// \u{1116b}: '𑅫'
    MahajaniLetterBha,
    /// \u{1116c}: '𑅬'
    MahajaniLetterMa,
    /// \u{1116d}: '𑅭'
    MahajaniLetterRa,
    /// \u{1116e}: '𑅮'
    MahajaniLetterLa,
    /// \u{1116f}: '𑅯'
    MahajaniLetterVa,
    /// \u{11170}: '𑅰'
    MahajaniLetterSa,
    /// \u{11171}: '𑅱'
    MahajaniLetterHa,
    /// \u{11172}: '𑅲'
    MahajaniLetterRra,
    /// \u{11173}: '𑅳'
    MahajaniSignNukta,
    /// \u{11174}: '𑅴'
    MahajaniAbbreviationSign,
    /// \u{11175}: '𑅵'
    MahajaniSectionMark,
    /// \u{11176}: '𑅶'
    MahajaniLigatureShri,
}

impl Into<char> for Mahajani {
    fn into(self) -> char {
        use constants::*;
        match self {
            Mahajani::MahajaniLetterA => MAHAJANI_LETTER_A,
            Mahajani::MahajaniLetterI => MAHAJANI_LETTER_I,
            Mahajani::MahajaniLetterU => MAHAJANI_LETTER_U,
            Mahajani::MahajaniLetterE => MAHAJANI_LETTER_E,
            Mahajani::MahajaniLetterO => MAHAJANI_LETTER_O,
            Mahajani::MahajaniLetterKa => MAHAJANI_LETTER_KA,
            Mahajani::MahajaniLetterKha => MAHAJANI_LETTER_KHA,
            Mahajani::MahajaniLetterGa => MAHAJANI_LETTER_GA,
            Mahajani::MahajaniLetterGha => MAHAJANI_LETTER_GHA,
            Mahajani::MahajaniLetterCa => MAHAJANI_LETTER_CA,
            Mahajani::MahajaniLetterCha => MAHAJANI_LETTER_CHA,
            Mahajani::MahajaniLetterJa => MAHAJANI_LETTER_JA,
            Mahajani::MahajaniLetterJha => MAHAJANI_LETTER_JHA,
            Mahajani::MahajaniLetterNya => MAHAJANI_LETTER_NYA,
            Mahajani::MahajaniLetterTta => MAHAJANI_LETTER_TTA,
            Mahajani::MahajaniLetterTtha => MAHAJANI_LETTER_TTHA,
            Mahajani::MahajaniLetterDda => MAHAJANI_LETTER_DDA,
            Mahajani::MahajaniLetterDdha => MAHAJANI_LETTER_DDHA,
            Mahajani::MahajaniLetterNna => MAHAJANI_LETTER_NNA,
            Mahajani::MahajaniLetterTa => MAHAJANI_LETTER_TA,
            Mahajani::MahajaniLetterTha => MAHAJANI_LETTER_THA,
            Mahajani::MahajaniLetterDa => MAHAJANI_LETTER_DA,
            Mahajani::MahajaniLetterDha => MAHAJANI_LETTER_DHA,
            Mahajani::MahajaniLetterNa => MAHAJANI_LETTER_NA,
            Mahajani::MahajaniLetterPa => MAHAJANI_LETTER_PA,
            Mahajani::MahajaniLetterPha => MAHAJANI_LETTER_PHA,
            Mahajani::MahajaniLetterBa => MAHAJANI_LETTER_BA,
            Mahajani::MahajaniLetterBha => MAHAJANI_LETTER_BHA,
            Mahajani::MahajaniLetterMa => MAHAJANI_LETTER_MA,
            Mahajani::MahajaniLetterRa => MAHAJANI_LETTER_RA,
            Mahajani::MahajaniLetterLa => MAHAJANI_LETTER_LA,
            Mahajani::MahajaniLetterVa => MAHAJANI_LETTER_VA,
            Mahajani::MahajaniLetterSa => MAHAJANI_LETTER_SA,
            Mahajani::MahajaniLetterHa => MAHAJANI_LETTER_HA,
            Mahajani::MahajaniLetterRra => MAHAJANI_LETTER_RRA,
            Mahajani::MahajaniSignNukta => MAHAJANI_SIGN_NUKTA,
            Mahajani::MahajaniAbbreviationSign => MAHAJANI_ABBREVIATION_SIGN,
            Mahajani::MahajaniSectionMark => MAHAJANI_SECTION_MARK,
            Mahajani::MahajaniLigatureShri => MAHAJANI_LIGATURE_SHRI,
        }
    }
}

impl std::convert::TryFrom<char> for Mahajani {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            MAHAJANI_LETTER_A => Ok(Mahajani::MahajaniLetterA),
            MAHAJANI_LETTER_I => Ok(Mahajani::MahajaniLetterI),
            MAHAJANI_LETTER_U => Ok(Mahajani::MahajaniLetterU),
            MAHAJANI_LETTER_E => Ok(Mahajani::MahajaniLetterE),
            MAHAJANI_LETTER_O => Ok(Mahajani::MahajaniLetterO),
            MAHAJANI_LETTER_KA => Ok(Mahajani::MahajaniLetterKa),
            MAHAJANI_LETTER_KHA => Ok(Mahajani::MahajaniLetterKha),
            MAHAJANI_LETTER_GA => Ok(Mahajani::MahajaniLetterGa),
            MAHAJANI_LETTER_GHA => Ok(Mahajani::MahajaniLetterGha),
            MAHAJANI_LETTER_CA => Ok(Mahajani::MahajaniLetterCa),
            MAHAJANI_LETTER_CHA => Ok(Mahajani::MahajaniLetterCha),
            MAHAJANI_LETTER_JA => Ok(Mahajani::MahajaniLetterJa),
            MAHAJANI_LETTER_JHA => Ok(Mahajani::MahajaniLetterJha),
            MAHAJANI_LETTER_NYA => Ok(Mahajani::MahajaniLetterNya),
            MAHAJANI_LETTER_TTA => Ok(Mahajani::MahajaniLetterTta),
            MAHAJANI_LETTER_TTHA => Ok(Mahajani::MahajaniLetterTtha),
            MAHAJANI_LETTER_DDA => Ok(Mahajani::MahajaniLetterDda),
            MAHAJANI_LETTER_DDHA => Ok(Mahajani::MahajaniLetterDdha),
            MAHAJANI_LETTER_NNA => Ok(Mahajani::MahajaniLetterNna),
            MAHAJANI_LETTER_TA => Ok(Mahajani::MahajaniLetterTa),
            MAHAJANI_LETTER_THA => Ok(Mahajani::MahajaniLetterTha),
            MAHAJANI_LETTER_DA => Ok(Mahajani::MahajaniLetterDa),
            MAHAJANI_LETTER_DHA => Ok(Mahajani::MahajaniLetterDha),
            MAHAJANI_LETTER_NA => Ok(Mahajani::MahajaniLetterNa),
            MAHAJANI_LETTER_PA => Ok(Mahajani::MahajaniLetterPa),
            MAHAJANI_LETTER_PHA => Ok(Mahajani::MahajaniLetterPha),
            MAHAJANI_LETTER_BA => Ok(Mahajani::MahajaniLetterBa),
            MAHAJANI_LETTER_BHA => Ok(Mahajani::MahajaniLetterBha),
            MAHAJANI_LETTER_MA => Ok(Mahajani::MahajaniLetterMa),
            MAHAJANI_LETTER_RA => Ok(Mahajani::MahajaniLetterRa),
            MAHAJANI_LETTER_LA => Ok(Mahajani::MahajaniLetterLa),
            MAHAJANI_LETTER_VA => Ok(Mahajani::MahajaniLetterVa),
            MAHAJANI_LETTER_SA => Ok(Mahajani::MahajaniLetterSa),
            MAHAJANI_LETTER_HA => Ok(Mahajani::MahajaniLetterHa),
            MAHAJANI_LETTER_RRA => Ok(Mahajani::MahajaniLetterRra),
            MAHAJANI_SIGN_NUKTA => Ok(Mahajani::MahajaniSignNukta),
            MAHAJANI_ABBREVIATION_SIGN => Ok(Mahajani::MahajaniAbbreviationSign),
            MAHAJANI_SECTION_MARK => Ok(Mahajani::MahajaniSectionMark),
            MAHAJANI_LIGATURE_SHRI => Ok(Mahajani::MahajaniLigatureShri),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Mahajani {
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

impl std::convert::TryFrom<u32> for Mahajani {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Mahajani {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Mahajani {
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        Mahajani::MahajaniLetterA
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Mahajani::MahajaniLetterA => "mahajani letter a",
            Mahajani::MahajaniLetterI => "mahajani letter i",
            Mahajani::MahajaniLetterU => "mahajani letter u",
            Mahajani::MahajaniLetterE => "mahajani letter e",
            Mahajani::MahajaniLetterO => "mahajani letter o",
            Mahajani::MahajaniLetterKa => "mahajani letter ka",
            Mahajani::MahajaniLetterKha => "mahajani letter kha",
            Mahajani::MahajaniLetterGa => "mahajani letter ga",
            Mahajani::MahajaniLetterGha => "mahajani letter gha",
            Mahajani::MahajaniLetterCa => "mahajani letter ca",
            Mahajani::MahajaniLetterCha => "mahajani letter cha",
            Mahajani::MahajaniLetterJa => "mahajani letter ja",
            Mahajani::MahajaniLetterJha => "mahajani letter jha",
            Mahajani::MahajaniLetterNya => "mahajani letter nya",
            Mahajani::MahajaniLetterTta => "mahajani letter tta",
            Mahajani::MahajaniLetterTtha => "mahajani letter ttha",
            Mahajani::MahajaniLetterDda => "mahajani letter dda",
            Mahajani::MahajaniLetterDdha => "mahajani letter ddha",
            Mahajani::MahajaniLetterNna => "mahajani letter nna",
            Mahajani::MahajaniLetterTa => "mahajani letter ta",
            Mahajani::MahajaniLetterTha => "mahajani letter tha",
            Mahajani::MahajaniLetterDa => "mahajani letter da",
            Mahajani::MahajaniLetterDha => "mahajani letter dha",
            Mahajani::MahajaniLetterNa => "mahajani letter na",
            Mahajani::MahajaniLetterPa => "mahajani letter pa",
            Mahajani::MahajaniLetterPha => "mahajani letter pha",
            Mahajani::MahajaniLetterBa => "mahajani letter ba",
            Mahajani::MahajaniLetterBha => "mahajani letter bha",
            Mahajani::MahajaniLetterMa => "mahajani letter ma",
            Mahajani::MahajaniLetterRa => "mahajani letter ra",
            Mahajani::MahajaniLetterLa => "mahajani letter la",
            Mahajani::MahajaniLetterVa => "mahajani letter va",
            Mahajani::MahajaniLetterSa => "mahajani letter sa",
            Mahajani::MahajaniLetterHa => "mahajani letter ha",
            Mahajani::MahajaniLetterRra => "mahajani letter rra",
            Mahajani::MahajaniSignNukta => "mahajani sign nukta",
            Mahajani::MahajaniAbbreviationSign => "mahajani abbreviation sign",
            Mahajani::MahajaniSectionMark => "mahajani section mark",
            Mahajani::MahajaniLigatureShri => "mahajani ligature shri",
        }
    }
}
