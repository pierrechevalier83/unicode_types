/// A number of constants to give a name to all characters in this block.
mod constants {
    /// \u{11150}: '𑅐'
    pub const LETTER_A: char = '𑅐';
    /// \u{11151}: '𑅑'
    pub const LETTER_I: char = '𑅑';
    /// \u{11152}: '𑅒'
    pub const LETTER_U: char = '𑅒';
    /// \u{11153}: '𑅓'
    pub const LETTER_E: char = '𑅓';
    /// \u{11154}: '𑅔'
    pub const LETTER_O: char = '𑅔';
    /// \u{11155}: '𑅕'
    pub const LETTER_KA: char = '𑅕';
    /// \u{11156}: '𑅖'
    pub const LETTER_KHA: char = '𑅖';
    /// \u{11157}: '𑅗'
    pub const LETTER_GA: char = '𑅗';
    /// \u{11158}: '𑅘'
    pub const LETTER_GHA: char = '𑅘';
    /// \u{11159}: '𑅙'
    pub const LETTER_CA: char = '𑅙';
    /// \u{1115a}: '𑅚'
    pub const LETTER_CHA: char = '𑅚';
    /// \u{1115b}: '𑅛'
    pub const LETTER_JA: char = '𑅛';
    /// \u{1115c}: '𑅜'
    pub const LETTER_JHA: char = '𑅜';
    /// \u{1115d}: '𑅝'
    pub const LETTER_NYA: char = '𑅝';
    /// \u{1115e}: '𑅞'
    pub const LETTER_TTA: char = '𑅞';
    /// \u{1115f}: '𑅟'
    pub const LETTER_TTHA: char = '𑅟';
    /// \u{11160}: '𑅠'
    pub const LETTER_DDA: char = '𑅠';
    /// \u{11161}: '𑅡'
    pub const LETTER_DDHA: char = '𑅡';
    /// \u{11162}: '𑅢'
    pub const LETTER_NNA: char = '𑅢';
    /// \u{11163}: '𑅣'
    pub const LETTER_TA: char = '𑅣';
    /// \u{11164}: '𑅤'
    pub const LETTER_THA: char = '𑅤';
    /// \u{11165}: '𑅥'
    pub const LETTER_DA: char = '𑅥';
    /// \u{11166}: '𑅦'
    pub const LETTER_DHA: char = '𑅦';
    /// \u{11167}: '𑅧'
    pub const LETTER_NA: char = '𑅧';
    /// \u{11168}: '𑅨'
    pub const LETTER_PA: char = '𑅨';
    /// \u{11169}: '𑅩'
    pub const LETTER_PHA: char = '𑅩';
    /// \u{1116a}: '𑅪'
    pub const LETTER_BA: char = '𑅪';
    /// \u{1116b}: '𑅫'
    pub const LETTER_BHA: char = '𑅫';
    /// \u{1116c}: '𑅬'
    pub const LETTER_MA: char = '𑅬';
    /// \u{1116d}: '𑅭'
    pub const LETTER_RA: char = '𑅭';
    /// \u{1116e}: '𑅮'
    pub const LETTER_LA: char = '𑅮';
    /// \u{1116f}: '𑅯'
    pub const LETTER_VA: char = '𑅯';
    /// \u{11170}: '𑅰'
    pub const LETTER_SA: char = '𑅰';
    /// \u{11171}: '𑅱'
    pub const LETTER_HA: char = '𑅱';
    /// \u{11172}: '𑅲'
    pub const LETTER_RRA: char = '𑅲';
    /// \u{11173}: '𑅳'
    pub const SIGN_NUKTA: char = '𑅳';
    /// \u{11174}: '𑅴'
    pub const ABBREVIATION_SIGN: char = '𑅴';
    /// \u{11175}: '𑅵'
    pub const SECTION_MARK: char = '𑅵';
    /// \u{11176}: '𑅶'
    pub const LIGATURE_SHRI: char = '𑅶';
}

/// An enum to represent all characters in the Mahajani block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Mahajani {
    /// \u{11150}: '𑅐'
    LetterA,
    /// \u{11151}: '𑅑'
    LetterI,
    /// \u{11152}: '𑅒'
    LetterU,
    /// \u{11153}: '𑅓'
    LetterE,
    /// \u{11154}: '𑅔'
    LetterO,
    /// \u{11155}: '𑅕'
    LetterKa,
    /// \u{11156}: '𑅖'
    LetterKha,
    /// \u{11157}: '𑅗'
    LetterGa,
    /// \u{11158}: '𑅘'
    LetterGha,
    /// \u{11159}: '𑅙'
    LetterCa,
    /// \u{1115a}: '𑅚'
    LetterCha,
    /// \u{1115b}: '𑅛'
    LetterJa,
    /// \u{1115c}: '𑅜'
    LetterJha,
    /// \u{1115d}: '𑅝'
    LetterNya,
    /// \u{1115e}: '𑅞'
    LetterTta,
    /// \u{1115f}: '𑅟'
    LetterTtha,
    /// \u{11160}: '𑅠'
    LetterDda,
    /// \u{11161}: '𑅡'
    LetterDdha,
    /// \u{11162}: '𑅢'
    LetterNna,
    /// \u{11163}: '𑅣'
    LetterTa,
    /// \u{11164}: '𑅤'
    LetterTha,
    /// \u{11165}: '𑅥'
    LetterDa,
    /// \u{11166}: '𑅦'
    LetterDha,
    /// \u{11167}: '𑅧'
    LetterNa,
    /// \u{11168}: '𑅨'
    LetterPa,
    /// \u{11169}: '𑅩'
    LetterPha,
    /// \u{1116a}: '𑅪'
    LetterBa,
    /// \u{1116b}: '𑅫'
    LetterBha,
    /// \u{1116c}: '𑅬'
    LetterMa,
    /// \u{1116d}: '𑅭'
    LetterRa,
    /// \u{1116e}: '𑅮'
    LetterLa,
    /// \u{1116f}: '𑅯'
    LetterVa,
    /// \u{11170}: '𑅰'
    LetterSa,
    /// \u{11171}: '𑅱'
    LetterHa,
    /// \u{11172}: '𑅲'
    LetterRra,
    /// \u{11173}: '𑅳'
    SignNukta,
    /// \u{11174}: '𑅴'
    AbbreviationSign,
    /// \u{11175}: '𑅵'
    SectionMark,
    /// \u{11176}: '𑅶'
    LigatureShri,
}

impl Into<char> for Mahajani {
    fn into(self) -> char {
        use constants::*;
        match self {
            Mahajani::LetterA => LETTER_A,
            Mahajani::LetterI => LETTER_I,
            Mahajani::LetterU => LETTER_U,
            Mahajani::LetterE => LETTER_E,
            Mahajani::LetterO => LETTER_O,
            Mahajani::LetterKa => LETTER_KA,
            Mahajani::LetterKha => LETTER_KHA,
            Mahajani::LetterGa => LETTER_GA,
            Mahajani::LetterGha => LETTER_GHA,
            Mahajani::LetterCa => LETTER_CA,
            Mahajani::LetterCha => LETTER_CHA,
            Mahajani::LetterJa => LETTER_JA,
            Mahajani::LetterJha => LETTER_JHA,
            Mahajani::LetterNya => LETTER_NYA,
            Mahajani::LetterTta => LETTER_TTA,
            Mahajani::LetterTtha => LETTER_TTHA,
            Mahajani::LetterDda => LETTER_DDA,
            Mahajani::LetterDdha => LETTER_DDHA,
            Mahajani::LetterNna => LETTER_NNA,
            Mahajani::LetterTa => LETTER_TA,
            Mahajani::LetterTha => LETTER_THA,
            Mahajani::LetterDa => LETTER_DA,
            Mahajani::LetterDha => LETTER_DHA,
            Mahajani::LetterNa => LETTER_NA,
            Mahajani::LetterPa => LETTER_PA,
            Mahajani::LetterPha => LETTER_PHA,
            Mahajani::LetterBa => LETTER_BA,
            Mahajani::LetterBha => LETTER_BHA,
            Mahajani::LetterMa => LETTER_MA,
            Mahajani::LetterRa => LETTER_RA,
            Mahajani::LetterLa => LETTER_LA,
            Mahajani::LetterVa => LETTER_VA,
            Mahajani::LetterSa => LETTER_SA,
            Mahajani::LetterHa => LETTER_HA,
            Mahajani::LetterRra => LETTER_RRA,
            Mahajani::SignNukta => SIGN_NUKTA,
            Mahajani::AbbreviationSign => ABBREVIATION_SIGN,
            Mahajani::SectionMark => SECTION_MARK,
            Mahajani::LigatureShri => LIGATURE_SHRI,
        }
    }
}

impl std::convert::TryFrom<char> for Mahajani {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_A => Ok(Mahajani::LetterA),
            LETTER_I => Ok(Mahajani::LetterI),
            LETTER_U => Ok(Mahajani::LetterU),
            LETTER_E => Ok(Mahajani::LetterE),
            LETTER_O => Ok(Mahajani::LetterO),
            LETTER_KA => Ok(Mahajani::LetterKa),
            LETTER_KHA => Ok(Mahajani::LetterKha),
            LETTER_GA => Ok(Mahajani::LetterGa),
            LETTER_GHA => Ok(Mahajani::LetterGha),
            LETTER_CA => Ok(Mahajani::LetterCa),
            LETTER_CHA => Ok(Mahajani::LetterCha),
            LETTER_JA => Ok(Mahajani::LetterJa),
            LETTER_JHA => Ok(Mahajani::LetterJha),
            LETTER_NYA => Ok(Mahajani::LetterNya),
            LETTER_TTA => Ok(Mahajani::LetterTta),
            LETTER_TTHA => Ok(Mahajani::LetterTtha),
            LETTER_DDA => Ok(Mahajani::LetterDda),
            LETTER_DDHA => Ok(Mahajani::LetterDdha),
            LETTER_NNA => Ok(Mahajani::LetterNna),
            LETTER_TA => Ok(Mahajani::LetterTa),
            LETTER_THA => Ok(Mahajani::LetterTha),
            LETTER_DA => Ok(Mahajani::LetterDa),
            LETTER_DHA => Ok(Mahajani::LetterDha),
            LETTER_NA => Ok(Mahajani::LetterNa),
            LETTER_PA => Ok(Mahajani::LetterPa),
            LETTER_PHA => Ok(Mahajani::LetterPha),
            LETTER_BA => Ok(Mahajani::LetterBa),
            LETTER_BHA => Ok(Mahajani::LetterBha),
            LETTER_MA => Ok(Mahajani::LetterMa),
            LETTER_RA => Ok(Mahajani::LetterRa),
            LETTER_LA => Ok(Mahajani::LetterLa),
            LETTER_VA => Ok(Mahajani::LetterVa),
            LETTER_SA => Ok(Mahajani::LetterSa),
            LETTER_HA => Ok(Mahajani::LetterHa),
            LETTER_RRA => Ok(Mahajani::LetterRra),
            SIGN_NUKTA => Ok(Mahajani::SignNukta),
            ABBREVIATION_SIGN => Ok(Mahajani::AbbreviationSign),
            SECTION_MARK => Ok(Mahajani::SectionMark),
            LIGATURE_SHRI => Ok(Mahajani::LigatureShri),
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
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Mahajani::LetterA
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Mahajani::LetterA => "mahajani letter a",
            Mahajani::LetterI => "mahajani letter i",
            Mahajani::LetterU => "mahajani letter u",
            Mahajani::LetterE => "mahajani letter e",
            Mahajani::LetterO => "mahajani letter o",
            Mahajani::LetterKa => "mahajani letter ka",
            Mahajani::LetterKha => "mahajani letter kha",
            Mahajani::LetterGa => "mahajani letter ga",
            Mahajani::LetterGha => "mahajani letter gha",
            Mahajani::LetterCa => "mahajani letter ca",
            Mahajani::LetterCha => "mahajani letter cha",
            Mahajani::LetterJa => "mahajani letter ja",
            Mahajani::LetterJha => "mahajani letter jha",
            Mahajani::LetterNya => "mahajani letter nya",
            Mahajani::LetterTta => "mahajani letter tta",
            Mahajani::LetterTtha => "mahajani letter ttha",
            Mahajani::LetterDda => "mahajani letter dda",
            Mahajani::LetterDdha => "mahajani letter ddha",
            Mahajani::LetterNna => "mahajani letter nna",
            Mahajani::LetterTa => "mahajani letter ta",
            Mahajani::LetterTha => "mahajani letter tha",
            Mahajani::LetterDa => "mahajani letter da",
            Mahajani::LetterDha => "mahajani letter dha",
            Mahajani::LetterNa => "mahajani letter na",
            Mahajani::LetterPa => "mahajani letter pa",
            Mahajani::LetterPha => "mahajani letter pha",
            Mahajani::LetterBa => "mahajani letter ba",
            Mahajani::LetterBha => "mahajani letter bha",
            Mahajani::LetterMa => "mahajani letter ma",
            Mahajani::LetterRa => "mahajani letter ra",
            Mahajani::LetterLa => "mahajani letter la",
            Mahajani::LetterVa => "mahajani letter va",
            Mahajani::LetterSa => "mahajani letter sa",
            Mahajani::LetterHa => "mahajani letter ha",
            Mahajani::LetterRra => "mahajani letter rra",
            Mahajani::SignNukta => "mahajani sign nukta",
            Mahajani::AbbreviationSign => "mahajani abbreviation sign",
            Mahajani::SectionMark => "mahajani section mark",
            Mahajani::LigatureShri => "mahajani ligature shri",
        }
    }
}
