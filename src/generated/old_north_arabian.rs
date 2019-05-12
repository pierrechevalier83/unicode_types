/// \u{10a80} → \u{10a9f}\
///\
/// 𐪀 𐪁 𐪂 𐪃 𐪄 𐪅 𐪆 𐪇 𐪈 𐪉 𐪊 𐪋 𐪌 𐪍 𐪎 𐪏
/// 𐪐 𐪑 𐪒 𐪓 𐪔 𐪕 𐪖 𐪗 𐪘 𐪙 𐪚 𐪛 𐪜 𐪝 𐪞
pub mod constants {
    /// \u{10a80}: '𐪀'
    pub const LETTER_HEH: char = '𐪀';
    /// \u{10a81}: '𐪁'
    pub const LETTER_LAM: char = '𐪁';
    /// \u{10a82}: '𐪂'
    pub const LETTER_HAH: char = '𐪂';
    /// \u{10a83}: '𐪃'
    pub const LETTER_MEEM: char = '𐪃';
    /// \u{10a84}: '𐪄'
    pub const LETTER_QAF: char = '𐪄';
    /// \u{10a85}: '𐪅'
    pub const LETTER_WAW: char = '𐪅';
    /// \u{10a86}: '𐪆'
    pub const LETTER_ES_DASH_2: char = '𐪆';
    /// \u{10a87}: '𐪇'
    pub const LETTER_REH: char = '𐪇';
    /// \u{10a88}: '𐪈'
    pub const LETTER_BEH: char = '𐪈';
    /// \u{10a89}: '𐪉'
    pub const LETTER_TEH: char = '𐪉';
    /// \u{10a8a}: '𐪊'
    pub const LETTER_ES_DASH_1: char = '𐪊';
    /// \u{10a8b}: '𐪋'
    pub const LETTER_KAF: char = '𐪋';
    /// \u{10a8c}: '𐪌'
    pub const LETTER_NOON: char = '𐪌';
    /// \u{10a8d}: '𐪍'
    pub const LETTER_KHAH: char = '𐪍';
    /// \u{10a8e}: '𐪎'
    pub const LETTER_SAD: char = '𐪎';
    /// \u{10a8f}: '𐪏'
    pub const LETTER_ES_DASH_3: char = '𐪏';
    /// \u{10a90}: '𐪐'
    pub const LETTER_FEH: char = '𐪐';
    /// \u{10a91}: '𐪑'
    pub const LETTER_ALEF: char = '𐪑';
    /// \u{10a92}: '𐪒'
    pub const LETTER_AIN: char = '𐪒';
    /// \u{10a93}: '𐪓'
    pub const LETTER_DAD: char = '𐪓';
    /// \u{10a94}: '𐪔'
    pub const LETTER_GEEM: char = '𐪔';
    /// \u{10a95}: '𐪕'
    pub const LETTER_DAL: char = '𐪕';
    /// \u{10a96}: '𐪖'
    pub const LETTER_GHAIN: char = '𐪖';
    /// \u{10a97}: '𐪗'
    pub const LETTER_TAH: char = '𐪗';
    /// \u{10a98}: '𐪘'
    pub const LETTER_ZAIN: char = '𐪘';
    /// \u{10a99}: '𐪙'
    pub const LETTER_THAL: char = '𐪙';
    /// \u{10a9a}: '𐪚'
    pub const LETTER_YEH: char = '𐪚';
    /// \u{10a9b}: '𐪛'
    pub const LETTER_THEH: char = '𐪛';
    /// \u{10a9c}: '𐪜'
    pub const LETTER_ZAH: char = '𐪜';
    /// \u{10a9d}: '𐪝'
    pub const NUMBER_ONE: char = '𐪝';
    /// \u{10a9e}: '𐪞'
    pub const NUMBER_TEN: char = '𐪞';
}

/// \u{10a80} → \u{10a9f}\
///\
/// 𐪀 𐪁 𐪂 𐪃 𐪄 𐪅 𐪆 𐪇 𐪈 𐪉 𐪊 𐪋 𐪌 𐪍 𐪎 𐪏
/// 𐪐 𐪑 𐪒 𐪓 𐪔 𐪕 𐪖 𐪗 𐪘 𐪙 𐪚 𐪛 𐪜 𐪝 𐪞
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
        use constants::*;
        match self {
            OldNorthArabian::LetterHeh => LETTER_HEH,
            OldNorthArabian::LetterLam => LETTER_LAM,
            OldNorthArabian::LetterHah => LETTER_HAH,
            OldNorthArabian::LetterMeem => LETTER_MEEM,
            OldNorthArabian::LetterQaf => LETTER_QAF,
            OldNorthArabian::LetterWaw => LETTER_WAW,
            OldNorthArabian::LetterEsDash2 => LETTER_ES_DASH_2,
            OldNorthArabian::LetterReh => LETTER_REH,
            OldNorthArabian::LetterBeh => LETTER_BEH,
            OldNorthArabian::LetterTeh => LETTER_TEH,
            OldNorthArabian::LetterEsDash1 => LETTER_ES_DASH_1,
            OldNorthArabian::LetterKaf => LETTER_KAF,
            OldNorthArabian::LetterNoon => LETTER_NOON,
            OldNorthArabian::LetterKhah => LETTER_KHAH,
            OldNorthArabian::LetterSad => LETTER_SAD,
            OldNorthArabian::LetterEsDash3 => LETTER_ES_DASH_3,
            OldNorthArabian::LetterFeh => LETTER_FEH,
            OldNorthArabian::LetterAlef => LETTER_ALEF,
            OldNorthArabian::LetterAin => LETTER_AIN,
            OldNorthArabian::LetterDad => LETTER_DAD,
            OldNorthArabian::LetterGeem => LETTER_GEEM,
            OldNorthArabian::LetterDal => LETTER_DAL,
            OldNorthArabian::LetterGhain => LETTER_GHAIN,
            OldNorthArabian::LetterTah => LETTER_TAH,
            OldNorthArabian::LetterZain => LETTER_ZAIN,
            OldNorthArabian::LetterThal => LETTER_THAL,
            OldNorthArabian::LetterYeh => LETTER_YEH,
            OldNorthArabian::LetterTheh => LETTER_THEH,
            OldNorthArabian::LetterZah => LETTER_ZAH,
            OldNorthArabian::NumberOne => NUMBER_ONE,
            OldNorthArabian::NumberTen => NUMBER_TEN,
        }
    }
}

impl std::convert::TryFrom<char> for OldNorthArabian {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_HEH => Ok(OldNorthArabian::LetterHeh),
            LETTER_LAM => Ok(OldNorthArabian::LetterLam),
            LETTER_HAH => Ok(OldNorthArabian::LetterHah),
            LETTER_MEEM => Ok(OldNorthArabian::LetterMeem),
            LETTER_QAF => Ok(OldNorthArabian::LetterQaf),
            LETTER_WAW => Ok(OldNorthArabian::LetterWaw),
            LETTER_ES_DASH_2 => Ok(OldNorthArabian::LetterEsDash2),
            LETTER_REH => Ok(OldNorthArabian::LetterReh),
            LETTER_BEH => Ok(OldNorthArabian::LetterBeh),
            LETTER_TEH => Ok(OldNorthArabian::LetterTeh),
            LETTER_ES_DASH_1 => Ok(OldNorthArabian::LetterEsDash1),
            LETTER_KAF => Ok(OldNorthArabian::LetterKaf),
            LETTER_NOON => Ok(OldNorthArabian::LetterNoon),
            LETTER_KHAH => Ok(OldNorthArabian::LetterKhah),
            LETTER_SAD => Ok(OldNorthArabian::LetterSad),
            LETTER_ES_DASH_3 => Ok(OldNorthArabian::LetterEsDash3),
            LETTER_FEH => Ok(OldNorthArabian::LetterFeh),
            LETTER_ALEF => Ok(OldNorthArabian::LetterAlef),
            LETTER_AIN => Ok(OldNorthArabian::LetterAin),
            LETTER_DAD => Ok(OldNorthArabian::LetterDad),
            LETTER_GEEM => Ok(OldNorthArabian::LetterGeem),
            LETTER_DAL => Ok(OldNorthArabian::LetterDal),
            LETTER_GHAIN => Ok(OldNorthArabian::LetterGhain),
            LETTER_TAH => Ok(OldNorthArabian::LetterTah),
            LETTER_ZAIN => Ok(OldNorthArabian::LetterZain),
            LETTER_THAL => Ok(OldNorthArabian::LetterThal),
            LETTER_YEH => Ok(OldNorthArabian::LetterYeh),
            LETTER_THEH => Ok(OldNorthArabian::LetterTheh),
            LETTER_ZAH => Ok(OldNorthArabian::LetterZah),
            NUMBER_ONE => Ok(OldNorthArabian::NumberOne),
            NUMBER_TEN => Ok(OldNorthArabian::NumberTen),
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

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            OldNorthArabian::LetterHeh => "old north arabian letter heh",
            OldNorthArabian::LetterLam => "old north arabian letter lam",
            OldNorthArabian::LetterHah => "old north arabian letter hah",
            OldNorthArabian::LetterMeem => "old north arabian letter meem",
            OldNorthArabian::LetterQaf => "old north arabian letter qaf",
            OldNorthArabian::LetterWaw => "old north arabian letter waw",
            OldNorthArabian::LetterEsDash2 => "old north arabian letter es-2",
            OldNorthArabian::LetterReh => "old north arabian letter reh",
            OldNorthArabian::LetterBeh => "old north arabian letter beh",
            OldNorthArabian::LetterTeh => "old north arabian letter teh",
            OldNorthArabian::LetterEsDash1 => "old north arabian letter es-1",
            OldNorthArabian::LetterKaf => "old north arabian letter kaf",
            OldNorthArabian::LetterNoon => "old north arabian letter noon",
            OldNorthArabian::LetterKhah => "old north arabian letter khah",
            OldNorthArabian::LetterSad => "old north arabian letter sad",
            OldNorthArabian::LetterEsDash3 => "old north arabian letter es-3",
            OldNorthArabian::LetterFeh => "old north arabian letter feh",
            OldNorthArabian::LetterAlef => "old north arabian letter alef",
            OldNorthArabian::LetterAin => "old north arabian letter ain",
            OldNorthArabian::LetterDad => "old north arabian letter dad",
            OldNorthArabian::LetterGeem => "old north arabian letter geem",
            OldNorthArabian::LetterDal => "old north arabian letter dal",
            OldNorthArabian::LetterGhain => "old north arabian letter ghain",
            OldNorthArabian::LetterTah => "old north arabian letter tah",
            OldNorthArabian::LetterZain => "old north arabian letter zain",
            OldNorthArabian::LetterThal => "old north arabian letter thal",
            OldNorthArabian::LetterYeh => "old north arabian letter yeh",
            OldNorthArabian::LetterTheh => "old north arabian letter theh",
            OldNorthArabian::LetterZah => "old north arabian letter zah",
            OldNorthArabian::NumberOne => "old north arabian number one",
            OldNorthArabian::NumberTen => "old north arabian number ten",
        }
    }
}
