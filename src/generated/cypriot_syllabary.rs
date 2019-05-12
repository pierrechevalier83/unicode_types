
/// An enum to represent all characters in the CypriotSyllabary block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum CypriotSyllabary {
    /// \u{10800}: '𐠀'
    CypriotSyllableA,
    /// \u{10801}: '𐠁'
    CypriotSyllableE,
    /// \u{10802}: '𐠂'
    CypriotSyllableI,
    /// \u{10803}: '𐠃'
    CypriotSyllableO,
    /// \u{10804}: '𐠄'
    CypriotSyllableU,
    /// \u{10805}: '𐠅'
    CypriotSyllableJa,
    /// \u{10808}: '𐠈'
    CypriotSyllableJo,
    /// \u{1080a}: '𐠊'
    CypriotSyllableKa,
    /// \u{1080b}: '𐠋'
    CypriotSyllableKe,
    /// \u{1080c}: '𐠌'
    CypriotSyllableKi,
    /// \u{1080d}: '𐠍'
    CypriotSyllableKo,
    /// \u{1080e}: '𐠎'
    CypriotSyllableKu,
    /// \u{1080f}: '𐠏'
    CypriotSyllableLa,
    /// \u{10810}: '𐠐'
    CypriotSyllableLe,
    /// \u{10811}: '𐠑'
    CypriotSyllableLi,
    /// \u{10812}: '𐠒'
    CypriotSyllableLo,
    /// \u{10813}: '𐠓'
    CypriotSyllableLu,
    /// \u{10814}: '𐠔'
    CypriotSyllableMa,
    /// \u{10815}: '𐠕'
    CypriotSyllableMe,
    /// \u{10816}: '𐠖'
    CypriotSyllableMi,
    /// \u{10817}: '𐠗'
    CypriotSyllableMo,
    /// \u{10818}: '𐠘'
    CypriotSyllableMu,
    /// \u{10819}: '𐠙'
    CypriotSyllableNa,
    /// \u{1081a}: '𐠚'
    CypriotSyllableNe,
    /// \u{1081b}: '𐠛'
    CypriotSyllableNi,
    /// \u{1081c}: '𐠜'
    CypriotSyllableNo,
    /// \u{1081d}: '𐠝'
    CypriotSyllableNu,
    /// \u{1081e}: '𐠞'
    CypriotSyllablePa,
    /// \u{1081f}: '𐠟'
    CypriotSyllablePe,
    /// \u{10820}: '𐠠'
    CypriotSyllablePi,
    /// \u{10821}: '𐠡'
    CypriotSyllablePo,
    /// \u{10822}: '𐠢'
    CypriotSyllablePu,
    /// \u{10823}: '𐠣'
    CypriotSyllableRa,
    /// \u{10824}: '𐠤'
    CypriotSyllableRe,
    /// \u{10825}: '𐠥'
    CypriotSyllableRi,
    /// \u{10826}: '𐠦'
    CypriotSyllableRo,
    /// \u{10827}: '𐠧'
    CypriotSyllableRu,
    /// \u{10828}: '𐠨'
    CypriotSyllableSa,
    /// \u{10829}: '𐠩'
    CypriotSyllableSe,
    /// \u{1082a}: '𐠪'
    CypriotSyllableSi,
    /// \u{1082b}: '𐠫'
    CypriotSyllableSo,
    /// \u{1082c}: '𐠬'
    CypriotSyllableSu,
    /// \u{1082d}: '𐠭'
    CypriotSyllableTa,
    /// \u{1082e}: '𐠮'
    CypriotSyllableTe,
    /// \u{1082f}: '𐠯'
    CypriotSyllableTi,
    /// \u{10830}: '𐠰'
    CypriotSyllableTo,
    /// \u{10831}: '𐠱'
    CypriotSyllableTu,
    /// \u{10832}: '𐠲'
    CypriotSyllableWa,
    /// \u{10833}: '𐠳'
    CypriotSyllableWe,
    /// \u{10834}: '𐠴'
    CypriotSyllableWi,
    /// \u{10835}: '𐠵'
    CypriotSyllableWo,
    /// \u{10837}: '𐠷'
    CypriotSyllableXa,
    /// \u{10838}: '𐠸'
    CypriotSyllableXe,
    /// \u{1083c}: '𐠼'
    CypriotSyllableZa,
}

impl Into<char> for CypriotSyllabary {
    fn into(self) -> char {
        match self {
            CypriotSyllabary::CypriotSyllableA => '𐠀',
            CypriotSyllabary::CypriotSyllableE => '𐠁',
            CypriotSyllabary::CypriotSyllableI => '𐠂',
            CypriotSyllabary::CypriotSyllableO => '𐠃',
            CypriotSyllabary::CypriotSyllableU => '𐠄',
            CypriotSyllabary::CypriotSyllableJa => '𐠅',
            CypriotSyllabary::CypriotSyllableJo => '𐠈',
            CypriotSyllabary::CypriotSyllableKa => '𐠊',
            CypriotSyllabary::CypriotSyllableKe => '𐠋',
            CypriotSyllabary::CypriotSyllableKi => '𐠌',
            CypriotSyllabary::CypriotSyllableKo => '𐠍',
            CypriotSyllabary::CypriotSyllableKu => '𐠎',
            CypriotSyllabary::CypriotSyllableLa => '𐠏',
            CypriotSyllabary::CypriotSyllableLe => '𐠐',
            CypriotSyllabary::CypriotSyllableLi => '𐠑',
            CypriotSyllabary::CypriotSyllableLo => '𐠒',
            CypriotSyllabary::CypriotSyllableLu => '𐠓',
            CypriotSyllabary::CypriotSyllableMa => '𐠔',
            CypriotSyllabary::CypriotSyllableMe => '𐠕',
            CypriotSyllabary::CypriotSyllableMi => '𐠖',
            CypriotSyllabary::CypriotSyllableMo => '𐠗',
            CypriotSyllabary::CypriotSyllableMu => '𐠘',
            CypriotSyllabary::CypriotSyllableNa => '𐠙',
            CypriotSyllabary::CypriotSyllableNe => '𐠚',
            CypriotSyllabary::CypriotSyllableNi => '𐠛',
            CypriotSyllabary::CypriotSyllableNo => '𐠜',
            CypriotSyllabary::CypriotSyllableNu => '𐠝',
            CypriotSyllabary::CypriotSyllablePa => '𐠞',
            CypriotSyllabary::CypriotSyllablePe => '𐠟',
            CypriotSyllabary::CypriotSyllablePi => '𐠠',
            CypriotSyllabary::CypriotSyllablePo => '𐠡',
            CypriotSyllabary::CypriotSyllablePu => '𐠢',
            CypriotSyllabary::CypriotSyllableRa => '𐠣',
            CypriotSyllabary::CypriotSyllableRe => '𐠤',
            CypriotSyllabary::CypriotSyllableRi => '𐠥',
            CypriotSyllabary::CypriotSyllableRo => '𐠦',
            CypriotSyllabary::CypriotSyllableRu => '𐠧',
            CypriotSyllabary::CypriotSyllableSa => '𐠨',
            CypriotSyllabary::CypriotSyllableSe => '𐠩',
            CypriotSyllabary::CypriotSyllableSi => '𐠪',
            CypriotSyllabary::CypriotSyllableSo => '𐠫',
            CypriotSyllabary::CypriotSyllableSu => '𐠬',
            CypriotSyllabary::CypriotSyllableTa => '𐠭',
            CypriotSyllabary::CypriotSyllableTe => '𐠮',
            CypriotSyllabary::CypriotSyllableTi => '𐠯',
            CypriotSyllabary::CypriotSyllableTo => '𐠰',
            CypriotSyllabary::CypriotSyllableTu => '𐠱',
            CypriotSyllabary::CypriotSyllableWa => '𐠲',
            CypriotSyllabary::CypriotSyllableWe => '𐠳',
            CypriotSyllabary::CypriotSyllableWi => '𐠴',
            CypriotSyllabary::CypriotSyllableWo => '𐠵',
            CypriotSyllabary::CypriotSyllableXa => '𐠷',
            CypriotSyllabary::CypriotSyllableXe => '𐠸',
            CypriotSyllabary::CypriotSyllableZa => '𐠼',
        }
    }
}

impl std::convert::TryFrom<char> for CypriotSyllabary {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𐠀' => Ok(CypriotSyllabary::CypriotSyllableA),
            '𐠁' => Ok(CypriotSyllabary::CypriotSyllableE),
            '𐠂' => Ok(CypriotSyllabary::CypriotSyllableI),
            '𐠃' => Ok(CypriotSyllabary::CypriotSyllableO),
            '𐠄' => Ok(CypriotSyllabary::CypriotSyllableU),
            '𐠅' => Ok(CypriotSyllabary::CypriotSyllableJa),
            '𐠈' => Ok(CypriotSyllabary::CypriotSyllableJo),
            '𐠊' => Ok(CypriotSyllabary::CypriotSyllableKa),
            '𐠋' => Ok(CypriotSyllabary::CypriotSyllableKe),
            '𐠌' => Ok(CypriotSyllabary::CypriotSyllableKi),
            '𐠍' => Ok(CypriotSyllabary::CypriotSyllableKo),
            '𐠎' => Ok(CypriotSyllabary::CypriotSyllableKu),
            '𐠏' => Ok(CypriotSyllabary::CypriotSyllableLa),
            '𐠐' => Ok(CypriotSyllabary::CypriotSyllableLe),
            '𐠑' => Ok(CypriotSyllabary::CypriotSyllableLi),
            '𐠒' => Ok(CypriotSyllabary::CypriotSyllableLo),
            '𐠓' => Ok(CypriotSyllabary::CypriotSyllableLu),
            '𐠔' => Ok(CypriotSyllabary::CypriotSyllableMa),
            '𐠕' => Ok(CypriotSyllabary::CypriotSyllableMe),
            '𐠖' => Ok(CypriotSyllabary::CypriotSyllableMi),
            '𐠗' => Ok(CypriotSyllabary::CypriotSyllableMo),
            '𐠘' => Ok(CypriotSyllabary::CypriotSyllableMu),
            '𐠙' => Ok(CypriotSyllabary::CypriotSyllableNa),
            '𐠚' => Ok(CypriotSyllabary::CypriotSyllableNe),
            '𐠛' => Ok(CypriotSyllabary::CypriotSyllableNi),
            '𐠜' => Ok(CypriotSyllabary::CypriotSyllableNo),
            '𐠝' => Ok(CypriotSyllabary::CypriotSyllableNu),
            '𐠞' => Ok(CypriotSyllabary::CypriotSyllablePa),
            '𐠟' => Ok(CypriotSyllabary::CypriotSyllablePe),
            '𐠠' => Ok(CypriotSyllabary::CypriotSyllablePi),
            '𐠡' => Ok(CypriotSyllabary::CypriotSyllablePo),
            '𐠢' => Ok(CypriotSyllabary::CypriotSyllablePu),
            '𐠣' => Ok(CypriotSyllabary::CypriotSyllableRa),
            '𐠤' => Ok(CypriotSyllabary::CypriotSyllableRe),
            '𐠥' => Ok(CypriotSyllabary::CypriotSyllableRi),
            '𐠦' => Ok(CypriotSyllabary::CypriotSyllableRo),
            '𐠧' => Ok(CypriotSyllabary::CypriotSyllableRu),
            '𐠨' => Ok(CypriotSyllabary::CypriotSyllableSa),
            '𐠩' => Ok(CypriotSyllabary::CypriotSyllableSe),
            '𐠪' => Ok(CypriotSyllabary::CypriotSyllableSi),
            '𐠫' => Ok(CypriotSyllabary::CypriotSyllableSo),
            '𐠬' => Ok(CypriotSyllabary::CypriotSyllableSu),
            '𐠭' => Ok(CypriotSyllabary::CypriotSyllableTa),
            '𐠮' => Ok(CypriotSyllabary::CypriotSyllableTe),
            '𐠯' => Ok(CypriotSyllabary::CypriotSyllableTi),
            '𐠰' => Ok(CypriotSyllabary::CypriotSyllableTo),
            '𐠱' => Ok(CypriotSyllabary::CypriotSyllableTu),
            '𐠲' => Ok(CypriotSyllabary::CypriotSyllableWa),
            '𐠳' => Ok(CypriotSyllabary::CypriotSyllableWe),
            '𐠴' => Ok(CypriotSyllabary::CypriotSyllableWi),
            '𐠵' => Ok(CypriotSyllabary::CypriotSyllableWo),
            '𐠷' => Ok(CypriotSyllabary::CypriotSyllableXa),
            '𐠸' => Ok(CypriotSyllabary::CypriotSyllableXe),
            '𐠼' => Ok(CypriotSyllabary::CypriotSyllableZa),
            _ => Err(()),
        }
    }
}

impl Into<u32> for CypriotSyllabary {
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

impl std::convert::TryFrom<u32> for CypriotSyllabary {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for CypriotSyllabary {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl CypriotSyllabary {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        CypriotSyllabary::CypriotSyllableA
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            CypriotSyllabary::CypriotSyllableA => "cypriot syllable a",
            CypriotSyllabary::CypriotSyllableE => "cypriot syllable e",
            CypriotSyllabary::CypriotSyllableI => "cypriot syllable i",
            CypriotSyllabary::CypriotSyllableO => "cypriot syllable o",
            CypriotSyllabary::CypriotSyllableU => "cypriot syllable u",
            CypriotSyllabary::CypriotSyllableJa => "cypriot syllable ja",
            CypriotSyllabary::CypriotSyllableJo => "cypriot syllable jo",
            CypriotSyllabary::CypriotSyllableKa => "cypriot syllable ka",
            CypriotSyllabary::CypriotSyllableKe => "cypriot syllable ke",
            CypriotSyllabary::CypriotSyllableKi => "cypriot syllable ki",
            CypriotSyllabary::CypriotSyllableKo => "cypriot syllable ko",
            CypriotSyllabary::CypriotSyllableKu => "cypriot syllable ku",
            CypriotSyllabary::CypriotSyllableLa => "cypriot syllable la",
            CypriotSyllabary::CypriotSyllableLe => "cypriot syllable le",
            CypriotSyllabary::CypriotSyllableLi => "cypriot syllable li",
            CypriotSyllabary::CypriotSyllableLo => "cypriot syllable lo",
            CypriotSyllabary::CypriotSyllableLu => "cypriot syllable lu",
            CypriotSyllabary::CypriotSyllableMa => "cypriot syllable ma",
            CypriotSyllabary::CypriotSyllableMe => "cypriot syllable me",
            CypriotSyllabary::CypriotSyllableMi => "cypriot syllable mi",
            CypriotSyllabary::CypriotSyllableMo => "cypriot syllable mo",
            CypriotSyllabary::CypriotSyllableMu => "cypriot syllable mu",
            CypriotSyllabary::CypriotSyllableNa => "cypriot syllable na",
            CypriotSyllabary::CypriotSyllableNe => "cypriot syllable ne",
            CypriotSyllabary::CypriotSyllableNi => "cypriot syllable ni",
            CypriotSyllabary::CypriotSyllableNo => "cypriot syllable no",
            CypriotSyllabary::CypriotSyllableNu => "cypriot syllable nu",
            CypriotSyllabary::CypriotSyllablePa => "cypriot syllable pa",
            CypriotSyllabary::CypriotSyllablePe => "cypriot syllable pe",
            CypriotSyllabary::CypriotSyllablePi => "cypriot syllable pi",
            CypriotSyllabary::CypriotSyllablePo => "cypriot syllable po",
            CypriotSyllabary::CypriotSyllablePu => "cypriot syllable pu",
            CypriotSyllabary::CypriotSyllableRa => "cypriot syllable ra",
            CypriotSyllabary::CypriotSyllableRe => "cypriot syllable re",
            CypriotSyllabary::CypriotSyllableRi => "cypriot syllable ri",
            CypriotSyllabary::CypriotSyllableRo => "cypriot syllable ro",
            CypriotSyllabary::CypriotSyllableRu => "cypriot syllable ru",
            CypriotSyllabary::CypriotSyllableSa => "cypriot syllable sa",
            CypriotSyllabary::CypriotSyllableSe => "cypriot syllable se",
            CypriotSyllabary::CypriotSyllableSi => "cypriot syllable si",
            CypriotSyllabary::CypriotSyllableSo => "cypriot syllable so",
            CypriotSyllabary::CypriotSyllableSu => "cypriot syllable su",
            CypriotSyllabary::CypriotSyllableTa => "cypriot syllable ta",
            CypriotSyllabary::CypriotSyllableTe => "cypriot syllable te",
            CypriotSyllabary::CypriotSyllableTi => "cypriot syllable ti",
            CypriotSyllabary::CypriotSyllableTo => "cypriot syllable to",
            CypriotSyllabary::CypriotSyllableTu => "cypriot syllable tu",
            CypriotSyllabary::CypriotSyllableWa => "cypriot syllable wa",
            CypriotSyllabary::CypriotSyllableWe => "cypriot syllable we",
            CypriotSyllabary::CypriotSyllableWi => "cypriot syllable wi",
            CypriotSyllabary::CypriotSyllableWo => "cypriot syllable wo",
            CypriotSyllabary::CypriotSyllableXa => "cypriot syllable xa",
            CypriotSyllabary::CypriotSyllableXe => "cypriot syllable xe",
            CypriotSyllabary::CypriotSyllableZa => "cypriot syllable za",
        }
    }
}
