
/// An enum to represent all characters in the OldNorthArabian block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum OldNorthArabian {
    /// \u{10a80}: '𐪀'
    LetterHeh,
    /// \u{10a81}: '𐪁'
    LetterLam,
    /// \u{10a82}: '𐪂'
    LetterHah,
    /// \u{10a83}: '𐪃'
    LetterMeem,
    /// \u{10a84}: '𐪄'
    LetterQaf,
    /// \u{10a85}: '𐪅'
    LetterWaw,
    /// \u{10a86}: '𐪆'
    LetterEsDash2,
    /// \u{10a87}: '𐪇'
    LetterReh,
    /// \u{10a88}: '𐪈'
    LetterBeh,
    /// \u{10a89}: '𐪉'
    LetterTeh,
    /// \u{10a8a}: '𐪊'
    LetterEsDash1,
    /// \u{10a8b}: '𐪋'
    LetterKaf,
    /// \u{10a8c}: '𐪌'
    LetterNoon,
    /// \u{10a8d}: '𐪍'
    LetterKhah,
    /// \u{10a8e}: '𐪎'
    LetterSad,
    /// \u{10a8f}: '𐪏'
    LetterEsDash3,
    /// \u{10a90}: '𐪐'
    LetterFeh,
    /// \u{10a91}: '𐪑'
    LetterAlef,
    /// \u{10a92}: '𐪒'
    LetterAin,
    /// \u{10a93}: '𐪓'
    LetterDad,
    /// \u{10a94}: '𐪔'
    LetterGeem,
    /// \u{10a95}: '𐪕'
    LetterDal,
    /// \u{10a96}: '𐪖'
    LetterGhain,
    /// \u{10a97}: '𐪗'
    LetterTah,
    /// \u{10a98}: '𐪘'
    LetterZain,
    /// \u{10a99}: '𐪙'
    LetterThal,
    /// \u{10a9a}: '𐪚'
    LetterYeh,
    /// \u{10a9b}: '𐪛'
    LetterTheh,
    /// \u{10a9c}: '𐪜'
    LetterZah,
    /// \u{10a9d}: '𐪝'
    NumberOne,
    /// \u{10a9e}: '𐪞'
    NumberTen,
}

impl Into<char> for OldNorthArabian {
    fn into(self) -> char {
        match self {
            OldNorthArabian::LetterHeh => '𐪀',
            OldNorthArabian::LetterLam => '𐪁',
            OldNorthArabian::LetterHah => '𐪂',
            OldNorthArabian::LetterMeem => '𐪃',
            OldNorthArabian::LetterQaf => '𐪄',
            OldNorthArabian::LetterWaw => '𐪅',
            OldNorthArabian::LetterEsDash2 => '𐪆',
            OldNorthArabian::LetterReh => '𐪇',
            OldNorthArabian::LetterBeh => '𐪈',
            OldNorthArabian::LetterTeh => '𐪉',
            OldNorthArabian::LetterEsDash1 => '𐪊',
            OldNorthArabian::LetterKaf => '𐪋',
            OldNorthArabian::LetterNoon => '𐪌',
            OldNorthArabian::LetterKhah => '𐪍',
            OldNorthArabian::LetterSad => '𐪎',
            OldNorthArabian::LetterEsDash3 => '𐪏',
            OldNorthArabian::LetterFeh => '𐪐',
            OldNorthArabian::LetterAlef => '𐪑',
            OldNorthArabian::LetterAin => '𐪒',
            OldNorthArabian::LetterDad => '𐪓',
            OldNorthArabian::LetterGeem => '𐪔',
            OldNorthArabian::LetterDal => '𐪕',
            OldNorthArabian::LetterGhain => '𐪖',
            OldNorthArabian::LetterTah => '𐪗',
            OldNorthArabian::LetterZain => '𐪘',
            OldNorthArabian::LetterThal => '𐪙',
            OldNorthArabian::LetterYeh => '𐪚',
            OldNorthArabian::LetterTheh => '𐪛',
            OldNorthArabian::LetterZah => '𐪜',
            OldNorthArabian::NumberOne => '𐪝',
            OldNorthArabian::NumberTen => '𐪞',
        }
    }
}

impl std::convert::TryFrom<char> for OldNorthArabian {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𐪀' => Ok(OldNorthArabian::LetterHeh),
            '𐪁' => Ok(OldNorthArabian::LetterLam),
            '𐪂' => Ok(OldNorthArabian::LetterHah),
            '𐪃' => Ok(OldNorthArabian::LetterMeem),
            '𐪄' => Ok(OldNorthArabian::LetterQaf),
            '𐪅' => Ok(OldNorthArabian::LetterWaw),
            '𐪆' => Ok(OldNorthArabian::LetterEsDash2),
            '𐪇' => Ok(OldNorthArabian::LetterReh),
            '𐪈' => Ok(OldNorthArabian::LetterBeh),
            '𐪉' => Ok(OldNorthArabian::LetterTeh),
            '𐪊' => Ok(OldNorthArabian::LetterEsDash1),
            '𐪋' => Ok(OldNorthArabian::LetterKaf),
            '𐪌' => Ok(OldNorthArabian::LetterNoon),
            '𐪍' => Ok(OldNorthArabian::LetterKhah),
            '𐪎' => Ok(OldNorthArabian::LetterSad),
            '𐪏' => Ok(OldNorthArabian::LetterEsDash3),
            '𐪐' => Ok(OldNorthArabian::LetterFeh),
            '𐪑' => Ok(OldNorthArabian::LetterAlef),
            '𐪒' => Ok(OldNorthArabian::LetterAin),
            '𐪓' => Ok(OldNorthArabian::LetterDad),
            '𐪔' => Ok(OldNorthArabian::LetterGeem),
            '𐪕' => Ok(OldNorthArabian::LetterDal),
            '𐪖' => Ok(OldNorthArabian::LetterGhain),
            '𐪗' => Ok(OldNorthArabian::LetterTah),
            '𐪘' => Ok(OldNorthArabian::LetterZain),
            '𐪙' => Ok(OldNorthArabian::LetterThal),
            '𐪚' => Ok(OldNorthArabian::LetterYeh),
            '𐪛' => Ok(OldNorthArabian::LetterTheh),
            '𐪜' => Ok(OldNorthArabian::LetterZah),
            '𐪝' => Ok(OldNorthArabian::NumberOne),
            '𐪞' => Ok(OldNorthArabian::NumberTen),
            _ => Err(()),
        }
    }
}

impl Into<u32> for OldNorthArabian {
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

impl std::convert::TryFrom<u32> for OldNorthArabian {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for OldNorthArabian {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl OldNorthArabian {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        OldNorthArabian::LetterHeh
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("OldNorthArabian{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
