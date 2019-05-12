
/// An enum to represent all characters in the Hatran block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Hatran {
    /// \u{108e0}: '𐣠'
    LetterAleph,
    /// \u{108e1}: '𐣡'
    LetterBeth,
    /// \u{108e2}: '𐣢'
    LetterGimel,
    /// \u{108e3}: '𐣣'
    LetterDalethDashResh,
    /// \u{108e4}: '𐣤'
    LetterHe,
    /// \u{108e5}: '𐣥'
    LetterWaw,
    /// \u{108e6}: '𐣦'
    LetterZayn,
    /// \u{108e7}: '𐣧'
    LetterHeth,
    /// \u{108e8}: '𐣨'
    LetterTeth,
    /// \u{108e9}: '𐣩'
    LetterYodh,
    /// \u{108ea}: '𐣪'
    LetterKaph,
    /// \u{108eb}: '𐣫'
    LetterLamedh,
    /// \u{108ec}: '𐣬'
    LetterMem,
    /// \u{108ed}: '𐣭'
    LetterNun,
    /// \u{108ee}: '𐣮'
    LetterSamekh,
    /// \u{108ef}: '𐣯'
    LetterAyn,
    /// \u{108f0}: '𐣰'
    LetterPe,
    /// \u{108f1}: '𐣱'
    LetterSadhe,
    /// \u{108f2}: '𐣲'
    LetterQoph,
    /// \u{108f4}: '𐣴'
    LetterShin,
    /// \u{108f5}: '𐣵'
    LetterTaw,
    /// \u{108fb}: '𐣻'
    NumberOne,
    /// \u{108fc}: '𐣼'
    NumberFive,
    /// \u{108fd}: '𐣽'
    NumberTen,
    /// \u{108fe}: '𐣾'
    NumberTwenty,
}

impl Into<char> for Hatran {
    fn into(self) -> char {
        match self {
            Hatran::LetterAleph => '𐣠',
            Hatran::LetterBeth => '𐣡',
            Hatran::LetterGimel => '𐣢',
            Hatran::LetterDalethDashResh => '𐣣',
            Hatran::LetterHe => '𐣤',
            Hatran::LetterWaw => '𐣥',
            Hatran::LetterZayn => '𐣦',
            Hatran::LetterHeth => '𐣧',
            Hatran::LetterTeth => '𐣨',
            Hatran::LetterYodh => '𐣩',
            Hatran::LetterKaph => '𐣪',
            Hatran::LetterLamedh => '𐣫',
            Hatran::LetterMem => '𐣬',
            Hatran::LetterNun => '𐣭',
            Hatran::LetterSamekh => '𐣮',
            Hatran::LetterAyn => '𐣯',
            Hatran::LetterPe => '𐣰',
            Hatran::LetterSadhe => '𐣱',
            Hatran::LetterQoph => '𐣲',
            Hatran::LetterShin => '𐣴',
            Hatran::LetterTaw => '𐣵',
            Hatran::NumberOne => '𐣻',
            Hatran::NumberFive => '𐣼',
            Hatran::NumberTen => '𐣽',
            Hatran::NumberTwenty => '𐣾',
        }
    }
}

impl std::convert::TryFrom<char> for Hatran {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𐣠' => Ok(Hatran::LetterAleph),
            '𐣡' => Ok(Hatran::LetterBeth),
            '𐣢' => Ok(Hatran::LetterGimel),
            '𐣣' => Ok(Hatran::LetterDalethDashResh),
            '𐣤' => Ok(Hatran::LetterHe),
            '𐣥' => Ok(Hatran::LetterWaw),
            '𐣦' => Ok(Hatran::LetterZayn),
            '𐣧' => Ok(Hatran::LetterHeth),
            '𐣨' => Ok(Hatran::LetterTeth),
            '𐣩' => Ok(Hatran::LetterYodh),
            '𐣪' => Ok(Hatran::LetterKaph),
            '𐣫' => Ok(Hatran::LetterLamedh),
            '𐣬' => Ok(Hatran::LetterMem),
            '𐣭' => Ok(Hatran::LetterNun),
            '𐣮' => Ok(Hatran::LetterSamekh),
            '𐣯' => Ok(Hatran::LetterAyn),
            '𐣰' => Ok(Hatran::LetterPe),
            '𐣱' => Ok(Hatran::LetterSadhe),
            '𐣲' => Ok(Hatran::LetterQoph),
            '𐣴' => Ok(Hatran::LetterShin),
            '𐣵' => Ok(Hatran::LetterTaw),
            '𐣻' => Ok(Hatran::NumberOne),
            '𐣼' => Ok(Hatran::NumberFive),
            '𐣽' => Ok(Hatran::NumberTen),
            '𐣾' => Ok(Hatran::NumberTwenty),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Hatran {
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

impl std::convert::TryFrom<u32> for Hatran {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Hatran {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Hatran {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Hatran::LetterAleph
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Hatran::LetterAleph => "hatran letter aleph",
            Hatran::LetterBeth => "hatran letter beth",
            Hatran::LetterGimel => "hatran letter gimel",
            Hatran::LetterDalethDashResh => "hatran letter daleth-resh",
            Hatran::LetterHe => "hatran letter he",
            Hatran::LetterWaw => "hatran letter waw",
            Hatran::LetterZayn => "hatran letter zayn",
            Hatran::LetterHeth => "hatran letter heth",
            Hatran::LetterTeth => "hatran letter teth",
            Hatran::LetterYodh => "hatran letter yodh",
            Hatran::LetterKaph => "hatran letter kaph",
            Hatran::LetterLamedh => "hatran letter lamedh",
            Hatran::LetterMem => "hatran letter mem",
            Hatran::LetterNun => "hatran letter nun",
            Hatran::LetterSamekh => "hatran letter samekh",
            Hatran::LetterAyn => "hatran letter ayn",
            Hatran::LetterPe => "hatran letter pe",
            Hatran::LetterSadhe => "hatran letter sadhe",
            Hatran::LetterQoph => "hatran letter qoph",
            Hatran::LetterShin => "hatran letter shin",
            Hatran::LetterTaw => "hatran letter taw",
            Hatran::NumberOne => "hatran number one",
            Hatran::NumberFive => "hatran number five",
            Hatran::NumberTen => "hatran number ten",
            Hatran::NumberTwenty => "hatran number twenty",
        }
    }
}
