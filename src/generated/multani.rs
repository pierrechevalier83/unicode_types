
/// An enum to represent all characters in the Multani block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Multani {
    /// \u{11280}: '𑊀'
    LetterA,
    /// \u{11281}: '𑊁'
    LetterI,
    /// \u{11282}: '𑊂'
    LetterU,
    /// \u{11283}: '𑊃'
    LetterE,
    /// \u{11284}: '𑊄'
    LetterKa,
    /// \u{11285}: '𑊅'
    LetterKha,
    /// \u{11286}: '𑊆'
    LetterGa,
    /// \u{11288}: '𑊈'
    LetterGha,
    /// \u{1128a}: '𑊊'
    LetterCa,
    /// \u{1128b}: '𑊋'
    LetterCha,
    /// \u{1128c}: '𑊌'
    LetterJa,
    /// \u{1128d}: '𑊍'
    LetterJja,
    /// \u{1128f}: '𑊏'
    LetterNya,
    /// \u{11290}: '𑊐'
    LetterTta,
    /// \u{11291}: '𑊑'
    LetterTtha,
    /// \u{11292}: '𑊒'
    LetterDda,
    /// \u{11293}: '𑊓'
    LetterDdda,
    /// \u{11294}: '𑊔'
    LetterDdha,
    /// \u{11295}: '𑊕'
    LetterNna,
    /// \u{11296}: '𑊖'
    LetterTa,
    /// \u{11297}: '𑊗'
    LetterTha,
    /// \u{11298}: '𑊘'
    LetterDa,
    /// \u{11299}: '𑊙'
    LetterDha,
    /// \u{1129a}: '𑊚'
    LetterNa,
    /// \u{1129b}: '𑊛'
    LetterPa,
    /// \u{1129c}: '𑊜'
    LetterPha,
    /// \u{1129d}: '𑊝'
    LetterBa,
    /// \u{1129f}: '𑊟'
    LetterBha,
    /// \u{112a0}: '𑊠'
    LetterMa,
    /// \u{112a1}: '𑊡'
    LetterYa,
    /// \u{112a2}: '𑊢'
    LetterRa,
    /// \u{112a3}: '𑊣'
    LetterLa,
    /// \u{112a4}: '𑊤'
    LetterVa,
    /// \u{112a5}: '𑊥'
    LetterSa,
    /// \u{112a6}: '𑊦'
    LetterHa,
    /// \u{112a7}: '𑊧'
    LetterRra,
    /// \u{112a8}: '𑊨'
    LetterRha,
    /// \u{112a9}: '𑊩'
    SectionMark,
}

impl Into<char> for Multani {
    fn into(self) -> char {
        match self {
            Multani::LetterA => '𑊀',
            Multani::LetterI => '𑊁',
            Multani::LetterU => '𑊂',
            Multani::LetterE => '𑊃',
            Multani::LetterKa => '𑊄',
            Multani::LetterKha => '𑊅',
            Multani::LetterGa => '𑊆',
            Multani::LetterGha => '𑊈',
            Multani::LetterCa => '𑊊',
            Multani::LetterCha => '𑊋',
            Multani::LetterJa => '𑊌',
            Multani::LetterJja => '𑊍',
            Multani::LetterNya => '𑊏',
            Multani::LetterTta => '𑊐',
            Multani::LetterTtha => '𑊑',
            Multani::LetterDda => '𑊒',
            Multani::LetterDdda => '𑊓',
            Multani::LetterDdha => '𑊔',
            Multani::LetterNna => '𑊕',
            Multani::LetterTa => '𑊖',
            Multani::LetterTha => '𑊗',
            Multani::LetterDa => '𑊘',
            Multani::LetterDha => '𑊙',
            Multani::LetterNa => '𑊚',
            Multani::LetterPa => '𑊛',
            Multani::LetterPha => '𑊜',
            Multani::LetterBa => '𑊝',
            Multani::LetterBha => '𑊟',
            Multani::LetterMa => '𑊠',
            Multani::LetterYa => '𑊡',
            Multani::LetterRa => '𑊢',
            Multani::LetterLa => '𑊣',
            Multani::LetterVa => '𑊤',
            Multani::LetterSa => '𑊥',
            Multani::LetterHa => '𑊦',
            Multani::LetterRra => '𑊧',
            Multani::LetterRha => '𑊨',
            Multani::SectionMark => '𑊩',
        }
    }
}

impl std::convert::TryFrom<char> for Multani {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𑊀' => Ok(Multani::LetterA),
            '𑊁' => Ok(Multani::LetterI),
            '𑊂' => Ok(Multani::LetterU),
            '𑊃' => Ok(Multani::LetterE),
            '𑊄' => Ok(Multani::LetterKa),
            '𑊅' => Ok(Multani::LetterKha),
            '𑊆' => Ok(Multani::LetterGa),
            '𑊈' => Ok(Multani::LetterGha),
            '𑊊' => Ok(Multani::LetterCa),
            '𑊋' => Ok(Multani::LetterCha),
            '𑊌' => Ok(Multani::LetterJa),
            '𑊍' => Ok(Multani::LetterJja),
            '𑊏' => Ok(Multani::LetterNya),
            '𑊐' => Ok(Multani::LetterTta),
            '𑊑' => Ok(Multani::LetterTtha),
            '𑊒' => Ok(Multani::LetterDda),
            '𑊓' => Ok(Multani::LetterDdda),
            '𑊔' => Ok(Multani::LetterDdha),
            '𑊕' => Ok(Multani::LetterNna),
            '𑊖' => Ok(Multani::LetterTa),
            '𑊗' => Ok(Multani::LetterTha),
            '𑊘' => Ok(Multani::LetterDa),
            '𑊙' => Ok(Multani::LetterDha),
            '𑊚' => Ok(Multani::LetterNa),
            '𑊛' => Ok(Multani::LetterPa),
            '𑊜' => Ok(Multani::LetterPha),
            '𑊝' => Ok(Multani::LetterBa),
            '𑊟' => Ok(Multani::LetterBha),
            '𑊠' => Ok(Multani::LetterMa),
            '𑊡' => Ok(Multani::LetterYa),
            '𑊢' => Ok(Multani::LetterRa),
            '𑊣' => Ok(Multani::LetterLa),
            '𑊤' => Ok(Multani::LetterVa),
            '𑊥' => Ok(Multani::LetterSa),
            '𑊦' => Ok(Multani::LetterHa),
            '𑊧' => Ok(Multani::LetterRra),
            '𑊨' => Ok(Multani::LetterRha),
            '𑊩' => Ok(Multani::SectionMark),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Multani {
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

impl std::convert::TryFrom<u32> for Multani {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Multani {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Multani {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Multani::LetterA
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Multani::LetterA => "multani letter a",
            Multani::LetterI => "multani letter i",
            Multani::LetterU => "multani letter u",
            Multani::LetterE => "multani letter e",
            Multani::LetterKa => "multani letter ka",
            Multani::LetterKha => "multani letter kha",
            Multani::LetterGa => "multani letter ga",
            Multani::LetterGha => "multani letter gha",
            Multani::LetterCa => "multani letter ca",
            Multani::LetterCha => "multani letter cha",
            Multani::LetterJa => "multani letter ja",
            Multani::LetterJja => "multani letter jja",
            Multani::LetterNya => "multani letter nya",
            Multani::LetterTta => "multani letter tta",
            Multani::LetterTtha => "multani letter ttha",
            Multani::LetterDda => "multani letter dda",
            Multani::LetterDdda => "multani letter ddda",
            Multani::LetterDdha => "multani letter ddha",
            Multani::LetterNna => "multani letter nna",
            Multani::LetterTa => "multani letter ta",
            Multani::LetterTha => "multani letter tha",
            Multani::LetterDa => "multani letter da",
            Multani::LetterDha => "multani letter dha",
            Multani::LetterNa => "multani letter na",
            Multani::LetterPa => "multani letter pa",
            Multani::LetterPha => "multani letter pha",
            Multani::LetterBa => "multani letter ba",
            Multani::LetterBha => "multani letter bha",
            Multani::LetterMa => "multani letter ma",
            Multani::LetterYa => "multani letter ya",
            Multani::LetterRa => "multani letter ra",
            Multani::LetterLa => "multani letter la",
            Multani::LetterVa => "multani letter va",
            Multani::LetterSa => "multani letter sa",
            Multani::LetterHa => "multani letter ha",
            Multani::LetterRra => "multani letter rra",
            Multani::LetterRha => "multani letter rha",
            Multani::SectionMark => "multani section mark",
        }
    }
}
