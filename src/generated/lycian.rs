
/// An enum to represent all characters in the Lycian block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Lycian {
    /// \u{10280}: '𐊀'
    LetterA,
    /// \u{10281}: '𐊁'
    LetterE,
    /// \u{10282}: '𐊂'
    LetterB,
    /// \u{10283}: '𐊃'
    LetterBh,
    /// \u{10284}: '𐊄'
    LetterG,
    /// \u{10285}: '𐊅'
    LetterD,
    /// \u{10286}: '𐊆'
    LetterI,
    /// \u{10287}: '𐊇'
    LetterW,
    /// \u{10288}: '𐊈'
    LetterZ,
    /// \u{10289}: '𐊉'
    LetterTh,
    /// \u{1028a}: '𐊊'
    LetterJ,
    /// \u{1028b}: '𐊋'
    LetterK,
    /// \u{1028c}: '𐊌'
    LetterQ,
    /// \u{1028d}: '𐊍'
    LetterL,
    /// \u{1028e}: '𐊎'
    LetterM,
    /// \u{1028f}: '𐊏'
    LetterN,
    /// \u{10290}: '𐊐'
    LetterMm,
    /// \u{10291}: '𐊑'
    LetterNn,
    /// \u{10292}: '𐊒'
    LetterU,
    /// \u{10293}: '𐊓'
    LetterP,
    /// \u{10294}: '𐊔'
    LetterKk,
    /// \u{10295}: '𐊕'
    LetterR,
    /// \u{10296}: '𐊖'
    LetterS,
    /// \u{10297}: '𐊗'
    LetterT,
    /// \u{10298}: '𐊘'
    LetterTt,
    /// \u{10299}: '𐊙'
    LetterAn,
    /// \u{1029a}: '𐊚'
    LetterEn,
    /// \u{1029b}: '𐊛'
    LetterH,
    /// \u{1029c}: '𐊜'
    LetterX,
}

impl Into<char> for Lycian {
    fn into(self) -> char {
        match self {
            Lycian::LetterA => '𐊀',
            Lycian::LetterE => '𐊁',
            Lycian::LetterB => '𐊂',
            Lycian::LetterBh => '𐊃',
            Lycian::LetterG => '𐊄',
            Lycian::LetterD => '𐊅',
            Lycian::LetterI => '𐊆',
            Lycian::LetterW => '𐊇',
            Lycian::LetterZ => '𐊈',
            Lycian::LetterTh => '𐊉',
            Lycian::LetterJ => '𐊊',
            Lycian::LetterK => '𐊋',
            Lycian::LetterQ => '𐊌',
            Lycian::LetterL => '𐊍',
            Lycian::LetterM => '𐊎',
            Lycian::LetterN => '𐊏',
            Lycian::LetterMm => '𐊐',
            Lycian::LetterNn => '𐊑',
            Lycian::LetterU => '𐊒',
            Lycian::LetterP => '𐊓',
            Lycian::LetterKk => '𐊔',
            Lycian::LetterR => '𐊕',
            Lycian::LetterS => '𐊖',
            Lycian::LetterT => '𐊗',
            Lycian::LetterTt => '𐊘',
            Lycian::LetterAn => '𐊙',
            Lycian::LetterEn => '𐊚',
            Lycian::LetterH => '𐊛',
            Lycian::LetterX => '𐊜',
        }
    }
}

impl std::convert::TryFrom<char> for Lycian {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𐊀' => Ok(Lycian::LetterA),
            '𐊁' => Ok(Lycian::LetterE),
            '𐊂' => Ok(Lycian::LetterB),
            '𐊃' => Ok(Lycian::LetterBh),
            '𐊄' => Ok(Lycian::LetterG),
            '𐊅' => Ok(Lycian::LetterD),
            '𐊆' => Ok(Lycian::LetterI),
            '𐊇' => Ok(Lycian::LetterW),
            '𐊈' => Ok(Lycian::LetterZ),
            '𐊉' => Ok(Lycian::LetterTh),
            '𐊊' => Ok(Lycian::LetterJ),
            '𐊋' => Ok(Lycian::LetterK),
            '𐊌' => Ok(Lycian::LetterQ),
            '𐊍' => Ok(Lycian::LetterL),
            '𐊎' => Ok(Lycian::LetterM),
            '𐊏' => Ok(Lycian::LetterN),
            '𐊐' => Ok(Lycian::LetterMm),
            '𐊑' => Ok(Lycian::LetterNn),
            '𐊒' => Ok(Lycian::LetterU),
            '𐊓' => Ok(Lycian::LetterP),
            '𐊔' => Ok(Lycian::LetterKk),
            '𐊕' => Ok(Lycian::LetterR),
            '𐊖' => Ok(Lycian::LetterS),
            '𐊗' => Ok(Lycian::LetterT),
            '𐊘' => Ok(Lycian::LetterTt),
            '𐊙' => Ok(Lycian::LetterAn),
            '𐊚' => Ok(Lycian::LetterEn),
            '𐊛' => Ok(Lycian::LetterH),
            '𐊜' => Ok(Lycian::LetterX),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Lycian {
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

impl std::convert::TryFrom<u32> for Lycian {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Lycian {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Lycian {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Lycian::LetterA
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Lycian{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
