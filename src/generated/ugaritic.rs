
/// An enum to represent all characters in the Ugaritic block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Ugaritic {
    /// \u{10380}: '𐎀'
    LetterAlpa,
    /// \u{10381}: '𐎁'
    LetterBeta,
    /// \u{10382}: '𐎂'
    LetterGamla,
    /// \u{10383}: '𐎃'
    LetterKha,
    /// \u{10384}: '𐎄'
    LetterDelta,
    /// \u{10385}: '𐎅'
    LetterHo,
    /// \u{10386}: '𐎆'
    LetterWo,
    /// \u{10387}: '𐎇'
    LetterZeta,
    /// \u{10388}: '𐎈'
    LetterHota,
    /// \u{10389}: '𐎉'
    LetterTet,
    /// \u{1038a}: '𐎊'
    LetterYod,
    /// \u{1038b}: '𐎋'
    LetterKaf,
    /// \u{1038c}: '𐎌'
    LetterShin,
    /// \u{1038d}: '𐎍'
    LetterLamda,
    /// \u{1038e}: '𐎎'
    LetterMem,
    /// \u{1038f}: '𐎏'
    LetterDhal,
    /// \u{10390}: '𐎐'
    LetterNun,
    /// \u{10391}: '𐎑'
    LetterZu,
    /// \u{10392}: '𐎒'
    LetterSamka,
    /// \u{10393}: '𐎓'
    LetterAin,
    /// \u{10394}: '𐎔'
    LetterPu,
    /// \u{10395}: '𐎕'
    LetterSade,
    /// \u{10396}: '𐎖'
    LetterQopa,
    /// \u{10397}: '𐎗'
    LetterRasha,
    /// \u{10398}: '𐎘'
    LetterThanna,
    /// \u{10399}: '𐎙'
    LetterGhain,
    /// \u{1039a}: '𐎚'
    LetterTo,
    /// \u{1039b}: '𐎛'
    LetterI,
    /// \u{1039c}: '𐎜'
    LetterU,
    /// \u{1039d}: '𐎝'
    LetterSsu,
}

impl Into<char> for Ugaritic {
    fn into(self) -> char {
        match self {
            Ugaritic::LetterAlpa => '𐎀',
            Ugaritic::LetterBeta => '𐎁',
            Ugaritic::LetterGamla => '𐎂',
            Ugaritic::LetterKha => '𐎃',
            Ugaritic::LetterDelta => '𐎄',
            Ugaritic::LetterHo => '𐎅',
            Ugaritic::LetterWo => '𐎆',
            Ugaritic::LetterZeta => '𐎇',
            Ugaritic::LetterHota => '𐎈',
            Ugaritic::LetterTet => '𐎉',
            Ugaritic::LetterYod => '𐎊',
            Ugaritic::LetterKaf => '𐎋',
            Ugaritic::LetterShin => '𐎌',
            Ugaritic::LetterLamda => '𐎍',
            Ugaritic::LetterMem => '𐎎',
            Ugaritic::LetterDhal => '𐎏',
            Ugaritic::LetterNun => '𐎐',
            Ugaritic::LetterZu => '𐎑',
            Ugaritic::LetterSamka => '𐎒',
            Ugaritic::LetterAin => '𐎓',
            Ugaritic::LetterPu => '𐎔',
            Ugaritic::LetterSade => '𐎕',
            Ugaritic::LetterQopa => '𐎖',
            Ugaritic::LetterRasha => '𐎗',
            Ugaritic::LetterThanna => '𐎘',
            Ugaritic::LetterGhain => '𐎙',
            Ugaritic::LetterTo => '𐎚',
            Ugaritic::LetterI => '𐎛',
            Ugaritic::LetterU => '𐎜',
            Ugaritic::LetterSsu => '𐎝',
        }
    }
}

impl std::convert::TryFrom<char> for Ugaritic {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𐎀' => Ok(Ugaritic::LetterAlpa),
            '𐎁' => Ok(Ugaritic::LetterBeta),
            '𐎂' => Ok(Ugaritic::LetterGamla),
            '𐎃' => Ok(Ugaritic::LetterKha),
            '𐎄' => Ok(Ugaritic::LetterDelta),
            '𐎅' => Ok(Ugaritic::LetterHo),
            '𐎆' => Ok(Ugaritic::LetterWo),
            '𐎇' => Ok(Ugaritic::LetterZeta),
            '𐎈' => Ok(Ugaritic::LetterHota),
            '𐎉' => Ok(Ugaritic::LetterTet),
            '𐎊' => Ok(Ugaritic::LetterYod),
            '𐎋' => Ok(Ugaritic::LetterKaf),
            '𐎌' => Ok(Ugaritic::LetterShin),
            '𐎍' => Ok(Ugaritic::LetterLamda),
            '𐎎' => Ok(Ugaritic::LetterMem),
            '𐎏' => Ok(Ugaritic::LetterDhal),
            '𐎐' => Ok(Ugaritic::LetterNun),
            '𐎑' => Ok(Ugaritic::LetterZu),
            '𐎒' => Ok(Ugaritic::LetterSamka),
            '𐎓' => Ok(Ugaritic::LetterAin),
            '𐎔' => Ok(Ugaritic::LetterPu),
            '𐎕' => Ok(Ugaritic::LetterSade),
            '𐎖' => Ok(Ugaritic::LetterQopa),
            '𐎗' => Ok(Ugaritic::LetterRasha),
            '𐎘' => Ok(Ugaritic::LetterThanna),
            '𐎙' => Ok(Ugaritic::LetterGhain),
            '𐎚' => Ok(Ugaritic::LetterTo),
            '𐎛' => Ok(Ugaritic::LetterI),
            '𐎜' => Ok(Ugaritic::LetterU),
            '𐎝' => Ok(Ugaritic::LetterSsu),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Ugaritic {
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

impl std::convert::TryFrom<u32> for Ugaritic {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Ugaritic {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Ugaritic {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Ugaritic::LetterAlpa
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Ugaritic::LetterAlpa => "ugaritic letter alpa",
            Ugaritic::LetterBeta => "ugaritic letter beta",
            Ugaritic::LetterGamla => "ugaritic letter gamla",
            Ugaritic::LetterKha => "ugaritic letter kha",
            Ugaritic::LetterDelta => "ugaritic letter delta",
            Ugaritic::LetterHo => "ugaritic letter ho",
            Ugaritic::LetterWo => "ugaritic letter wo",
            Ugaritic::LetterZeta => "ugaritic letter zeta",
            Ugaritic::LetterHota => "ugaritic letter hota",
            Ugaritic::LetterTet => "ugaritic letter tet",
            Ugaritic::LetterYod => "ugaritic letter yod",
            Ugaritic::LetterKaf => "ugaritic letter kaf",
            Ugaritic::LetterShin => "ugaritic letter shin",
            Ugaritic::LetterLamda => "ugaritic letter lamda",
            Ugaritic::LetterMem => "ugaritic letter mem",
            Ugaritic::LetterDhal => "ugaritic letter dhal",
            Ugaritic::LetterNun => "ugaritic letter nun",
            Ugaritic::LetterZu => "ugaritic letter zu",
            Ugaritic::LetterSamka => "ugaritic letter samka",
            Ugaritic::LetterAin => "ugaritic letter ain",
            Ugaritic::LetterPu => "ugaritic letter pu",
            Ugaritic::LetterSade => "ugaritic letter sade",
            Ugaritic::LetterQopa => "ugaritic letter qopa",
            Ugaritic::LetterRasha => "ugaritic letter rasha",
            Ugaritic::LetterThanna => "ugaritic letter thanna",
            Ugaritic::LetterGhain => "ugaritic letter ghain",
            Ugaritic::LetterTo => "ugaritic letter to",
            Ugaritic::LetterI => "ugaritic letter i",
            Ugaritic::LetterU => "ugaritic letter u",
            Ugaritic::LetterSsu => "ugaritic letter ssu",
        }
    }
}
