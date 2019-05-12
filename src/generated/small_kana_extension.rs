
/// An enum to represent all characters in the SmallKanaExtension block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SmallKanaExtension {
    /// \u{1b150}: '𛅐'
    HiraganaLetterSmallWi,
    /// \u{1b151}: '𛅑'
    HiraganaLetterSmallWe,
    /// \u{1b152}: '𛅒'
    HiraganaLetterSmallWo,
    /// \u{1b164}: '𛅤'
    KatakanaLetterSmallWi,
    /// \u{1b165}: '𛅥'
    KatakanaLetterSmallWe,
    /// \u{1b166}: '𛅦'
    KatakanaLetterSmallWo,
    /// \u{1b167}: '𛅧'
    KatakanaLetterSmallN,
}

impl Into<char> for SmallKanaExtension {
    fn into(self) -> char {
        match self {
            SmallKanaExtension::HiraganaLetterSmallWi => '𛅐',
            SmallKanaExtension::HiraganaLetterSmallWe => '𛅑',
            SmallKanaExtension::HiraganaLetterSmallWo => '𛅒',
            SmallKanaExtension::KatakanaLetterSmallWi => '𛅤',
            SmallKanaExtension::KatakanaLetterSmallWe => '𛅥',
            SmallKanaExtension::KatakanaLetterSmallWo => '𛅦',
            SmallKanaExtension::KatakanaLetterSmallN => '𛅧',
        }
    }
}

impl std::convert::TryFrom<char> for SmallKanaExtension {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𛅐' => Ok(SmallKanaExtension::HiraganaLetterSmallWi),
            '𛅑' => Ok(SmallKanaExtension::HiraganaLetterSmallWe),
            '𛅒' => Ok(SmallKanaExtension::HiraganaLetterSmallWo),
            '𛅤' => Ok(SmallKanaExtension::KatakanaLetterSmallWi),
            '𛅥' => Ok(SmallKanaExtension::KatakanaLetterSmallWe),
            '𛅦' => Ok(SmallKanaExtension::KatakanaLetterSmallWo),
            '𛅧' => Ok(SmallKanaExtension::KatakanaLetterSmallN),
            _ => Err(()),
        }
    }
}

impl Into<u32> for SmallKanaExtension {
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

impl std::convert::TryFrom<u32> for SmallKanaExtension {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for SmallKanaExtension {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl SmallKanaExtension {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        SmallKanaExtension::HiraganaLetterSmallWi
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            SmallKanaExtension::HiraganaLetterSmallWi => "hiragana letter small wi",
            SmallKanaExtension::HiraganaLetterSmallWe => "hiragana letter small we",
            SmallKanaExtension::HiraganaLetterSmallWo => "hiragana letter small wo",
            SmallKanaExtension::KatakanaLetterSmallWi => "katakana letter small wi",
            SmallKanaExtension::KatakanaLetterSmallWe => "katakana letter small we",
            SmallKanaExtension::KatakanaLetterSmallWo => "katakana letter small wo",
            SmallKanaExtension::KatakanaLetterSmallN => "katakana letter small n",
        }
    }
}
