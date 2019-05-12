
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
        match self {
            Mahajani::LetterA => '𑅐',
            Mahajani::LetterI => '𑅑',
            Mahajani::LetterU => '𑅒',
            Mahajani::LetterE => '𑅓',
            Mahajani::LetterO => '𑅔',
            Mahajani::LetterKa => '𑅕',
            Mahajani::LetterKha => '𑅖',
            Mahajani::LetterGa => '𑅗',
            Mahajani::LetterGha => '𑅘',
            Mahajani::LetterCa => '𑅙',
            Mahajani::LetterCha => '𑅚',
            Mahajani::LetterJa => '𑅛',
            Mahajani::LetterJha => '𑅜',
            Mahajani::LetterNya => '𑅝',
            Mahajani::LetterTta => '𑅞',
            Mahajani::LetterTtha => '𑅟',
            Mahajani::LetterDda => '𑅠',
            Mahajani::LetterDdha => '𑅡',
            Mahajani::LetterNna => '𑅢',
            Mahajani::LetterTa => '𑅣',
            Mahajani::LetterTha => '𑅤',
            Mahajani::LetterDa => '𑅥',
            Mahajani::LetterDha => '𑅦',
            Mahajani::LetterNa => '𑅧',
            Mahajani::LetterPa => '𑅨',
            Mahajani::LetterPha => '𑅩',
            Mahajani::LetterBa => '𑅪',
            Mahajani::LetterBha => '𑅫',
            Mahajani::LetterMa => '𑅬',
            Mahajani::LetterRa => '𑅭',
            Mahajani::LetterLa => '𑅮',
            Mahajani::LetterVa => '𑅯',
            Mahajani::LetterSa => '𑅰',
            Mahajani::LetterHa => '𑅱',
            Mahajani::LetterRra => '𑅲',
            Mahajani::SignNukta => '𑅳',
            Mahajani::AbbreviationSign => '𑅴',
            Mahajani::SectionMark => '𑅵',
            Mahajani::LigatureShri => '𑅶',
        }
    }
}

impl std::convert::TryFrom<char> for Mahajani {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𑅐' => Ok(Mahajani::LetterA),
            '𑅑' => Ok(Mahajani::LetterI),
            '𑅒' => Ok(Mahajani::LetterU),
            '𑅓' => Ok(Mahajani::LetterE),
            '𑅔' => Ok(Mahajani::LetterO),
            '𑅕' => Ok(Mahajani::LetterKa),
            '𑅖' => Ok(Mahajani::LetterKha),
            '𑅗' => Ok(Mahajani::LetterGa),
            '𑅘' => Ok(Mahajani::LetterGha),
            '𑅙' => Ok(Mahajani::LetterCa),
            '𑅚' => Ok(Mahajani::LetterCha),
            '𑅛' => Ok(Mahajani::LetterJa),
            '𑅜' => Ok(Mahajani::LetterJha),
            '𑅝' => Ok(Mahajani::LetterNya),
            '𑅞' => Ok(Mahajani::LetterTta),
            '𑅟' => Ok(Mahajani::LetterTtha),
            '𑅠' => Ok(Mahajani::LetterDda),
            '𑅡' => Ok(Mahajani::LetterDdha),
            '𑅢' => Ok(Mahajani::LetterNna),
            '𑅣' => Ok(Mahajani::LetterTa),
            '𑅤' => Ok(Mahajani::LetterTha),
            '𑅥' => Ok(Mahajani::LetterDa),
            '𑅦' => Ok(Mahajani::LetterDha),
            '𑅧' => Ok(Mahajani::LetterNa),
            '𑅨' => Ok(Mahajani::LetterPa),
            '𑅩' => Ok(Mahajani::LetterPha),
            '𑅪' => Ok(Mahajani::LetterBa),
            '𑅫' => Ok(Mahajani::LetterBha),
            '𑅬' => Ok(Mahajani::LetterMa),
            '𑅭' => Ok(Mahajani::LetterRa),
            '𑅮' => Ok(Mahajani::LetterLa),
            '𑅯' => Ok(Mahajani::LetterVa),
            '𑅰' => Ok(Mahajani::LetterSa),
            '𑅱' => Ok(Mahajani::LetterHa),
            '𑅲' => Ok(Mahajani::LetterRra),
            '𑅳' => Ok(Mahajani::SignNukta),
            '𑅴' => Ok(Mahajani::AbbreviationSign),
            '𑅵' => Ok(Mahajani::SectionMark),
            '𑅶' => Ok(Mahajani::LigatureShri),
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

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Mahajani{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
