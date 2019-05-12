/// \u{10a80} → \u{10a9f}\
///\
/// 𐪀 𐪁 𐪂 𐪃 𐪄 𐪅 𐪆 𐪇 𐪈 𐪉 𐪊 𐪋 𐪌 𐪍 𐪎 𐪏
/// 𐪐 𐪑 𐪒 𐪓 𐪔 𐪕 𐪖 𐪗 𐪘 𐪙 𐪚 𐪛 𐪜 𐪝 𐪞
pub mod constants {
    /// \u{10a80}: '𐪀'
    pub const OLD_NORTH_ARABIAN_LETTER_HEH: char = '𐪀';
    /// \u{10a81}: '𐪁'
    pub const OLD_NORTH_ARABIAN_LETTER_LAM: char = '𐪁';
    /// \u{10a82}: '𐪂'
    pub const OLD_NORTH_ARABIAN_LETTER_HAH: char = '𐪂';
    /// \u{10a83}: '𐪃'
    pub const OLD_NORTH_ARABIAN_LETTER_MEEM: char = '𐪃';
    /// \u{10a84}: '𐪄'
    pub const OLD_NORTH_ARABIAN_LETTER_QAF: char = '𐪄';
    /// \u{10a85}: '𐪅'
    pub const OLD_NORTH_ARABIAN_LETTER_WAW: char = '𐪅';
    /// \u{10a86}: '𐪆'
    pub const OLD_NORTH_ARABIAN_LETTER_ES_DASH_2: char = '𐪆';
    /// \u{10a87}: '𐪇'
    pub const OLD_NORTH_ARABIAN_LETTER_REH: char = '𐪇';
    /// \u{10a88}: '𐪈'
    pub const OLD_NORTH_ARABIAN_LETTER_BEH: char = '𐪈';
    /// \u{10a89}: '𐪉'
    pub const OLD_NORTH_ARABIAN_LETTER_TEH: char = '𐪉';
    /// \u{10a8a}: '𐪊'
    pub const OLD_NORTH_ARABIAN_LETTER_ES_DASH_1: char = '𐪊';
    /// \u{10a8b}: '𐪋'
    pub const OLD_NORTH_ARABIAN_LETTER_KAF: char = '𐪋';
    /// \u{10a8c}: '𐪌'
    pub const OLD_NORTH_ARABIAN_LETTER_NOON: char = '𐪌';
    /// \u{10a8d}: '𐪍'
    pub const OLD_NORTH_ARABIAN_LETTER_KHAH: char = '𐪍';
    /// \u{10a8e}: '𐪎'
    pub const OLD_NORTH_ARABIAN_LETTER_SAD: char = '𐪎';
    /// \u{10a8f}: '𐪏'
    pub const OLD_NORTH_ARABIAN_LETTER_ES_DASH_3: char = '𐪏';
    /// \u{10a90}: '𐪐'
    pub const OLD_NORTH_ARABIAN_LETTER_FEH: char = '𐪐';
    /// \u{10a91}: '𐪑'
    pub const OLD_NORTH_ARABIAN_LETTER_ALEF: char = '𐪑';
    /// \u{10a92}: '𐪒'
    pub const OLD_NORTH_ARABIAN_LETTER_AIN: char = '𐪒';
    /// \u{10a93}: '𐪓'
    pub const OLD_NORTH_ARABIAN_LETTER_DAD: char = '𐪓';
    /// \u{10a94}: '𐪔'
    pub const OLD_NORTH_ARABIAN_LETTER_GEEM: char = '𐪔';
    /// \u{10a95}: '𐪕'
    pub const OLD_NORTH_ARABIAN_LETTER_DAL: char = '𐪕';
    /// \u{10a96}: '𐪖'
    pub const OLD_NORTH_ARABIAN_LETTER_GHAIN: char = '𐪖';
    /// \u{10a97}: '𐪗'
    pub const OLD_NORTH_ARABIAN_LETTER_TAH: char = '𐪗';
    /// \u{10a98}: '𐪘'
    pub const OLD_NORTH_ARABIAN_LETTER_ZAIN: char = '𐪘';
    /// \u{10a99}: '𐪙'
    pub const OLD_NORTH_ARABIAN_LETTER_THAL: char = '𐪙';
    /// \u{10a9a}: '𐪚'
    pub const OLD_NORTH_ARABIAN_LETTER_YEH: char = '𐪚';
    /// \u{10a9b}: '𐪛'
    pub const OLD_NORTH_ARABIAN_LETTER_THEH: char = '𐪛';
    /// \u{10a9c}: '𐪜'
    pub const OLD_NORTH_ARABIAN_LETTER_ZAH: char = '𐪜';
    /// \u{10a9d}: '𐪝'
    pub const OLD_NORTH_ARABIAN_NUMBER_ONE: char = '𐪝';
    /// \u{10a9e}: '𐪞'
    pub const OLD_NORTH_ARABIAN_NUMBER_TEN: char = '𐪞';
}

/// \u{10a80} → \u{10a9f}\
///\
/// 𐪀 𐪁 𐪂 𐪃 𐪄 𐪅 𐪆 𐪇 𐪈 𐪉 𐪊 𐪋 𐪌 𐪍 𐪎 𐪏
/// 𐪐 𐪑 𐪒 𐪓 𐪔 𐪕 𐪖 𐪗 𐪘 𐪙 𐪚 𐪛 𐪜 𐪝 𐪞
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum OldNorthArabian {
    /// \u{10a80}: '𐪀'
    OldNorthArabianLetterHeh,
    /// \u{10a81}: '𐪁'
    OldNorthArabianLetterLam,
    /// \u{10a82}: '𐪂'
    OldNorthArabianLetterHah,
    /// \u{10a83}: '𐪃'
    OldNorthArabianLetterMeem,
    /// \u{10a84}: '𐪄'
    OldNorthArabianLetterQaf,
    /// \u{10a85}: '𐪅'
    OldNorthArabianLetterWaw,
    /// \u{10a86}: '𐪆'
    OldNorthArabianLetterEsDash2,
    /// \u{10a87}: '𐪇'
    OldNorthArabianLetterReh,
    /// \u{10a88}: '𐪈'
    OldNorthArabianLetterBeh,
    /// \u{10a89}: '𐪉'
    OldNorthArabianLetterTeh,
    /// \u{10a8a}: '𐪊'
    OldNorthArabianLetterEsDash1,
    /// \u{10a8b}: '𐪋'
    OldNorthArabianLetterKaf,
    /// \u{10a8c}: '𐪌'
    OldNorthArabianLetterNoon,
    /// \u{10a8d}: '𐪍'
    OldNorthArabianLetterKhah,
    /// \u{10a8e}: '𐪎'
    OldNorthArabianLetterSad,
    /// \u{10a8f}: '𐪏'
    OldNorthArabianLetterEsDash3,
    /// \u{10a90}: '𐪐'
    OldNorthArabianLetterFeh,
    /// \u{10a91}: '𐪑'
    OldNorthArabianLetterAlef,
    /// \u{10a92}: '𐪒'
    OldNorthArabianLetterAin,
    /// \u{10a93}: '𐪓'
    OldNorthArabianLetterDad,
    /// \u{10a94}: '𐪔'
    OldNorthArabianLetterGeem,
    /// \u{10a95}: '𐪕'
    OldNorthArabianLetterDal,
    /// \u{10a96}: '𐪖'
    OldNorthArabianLetterGhain,
    /// \u{10a97}: '𐪗'
    OldNorthArabianLetterTah,
    /// \u{10a98}: '𐪘'
    OldNorthArabianLetterZain,
    /// \u{10a99}: '𐪙'
    OldNorthArabianLetterThal,
    /// \u{10a9a}: '𐪚'
    OldNorthArabianLetterYeh,
    /// \u{10a9b}: '𐪛'
    OldNorthArabianLetterTheh,
    /// \u{10a9c}: '𐪜'
    OldNorthArabianLetterZah,
    /// \u{10a9d}: '𐪝'
    OldNorthArabianNumberOne,
    /// \u{10a9e}: '𐪞'
    OldNorthArabianNumberTen,
}

impl Into<char> for OldNorthArabian {
    fn into(self) -> char {
        use constants::*;
        match self {
            OldNorthArabian::OldNorthArabianLetterHeh => OLD_NORTH_ARABIAN_LETTER_HEH,
            OldNorthArabian::OldNorthArabianLetterLam => OLD_NORTH_ARABIAN_LETTER_LAM,
            OldNorthArabian::OldNorthArabianLetterHah => OLD_NORTH_ARABIAN_LETTER_HAH,
            OldNorthArabian::OldNorthArabianLetterMeem => OLD_NORTH_ARABIAN_LETTER_MEEM,
            OldNorthArabian::OldNorthArabianLetterQaf => OLD_NORTH_ARABIAN_LETTER_QAF,
            OldNorthArabian::OldNorthArabianLetterWaw => OLD_NORTH_ARABIAN_LETTER_WAW,
            OldNorthArabian::OldNorthArabianLetterEsDash2 => OLD_NORTH_ARABIAN_LETTER_ES_DASH_2,
            OldNorthArabian::OldNorthArabianLetterReh => OLD_NORTH_ARABIAN_LETTER_REH,
            OldNorthArabian::OldNorthArabianLetterBeh => OLD_NORTH_ARABIAN_LETTER_BEH,
            OldNorthArabian::OldNorthArabianLetterTeh => OLD_NORTH_ARABIAN_LETTER_TEH,
            OldNorthArabian::OldNorthArabianLetterEsDash1 => OLD_NORTH_ARABIAN_LETTER_ES_DASH_1,
            OldNorthArabian::OldNorthArabianLetterKaf => OLD_NORTH_ARABIAN_LETTER_KAF,
            OldNorthArabian::OldNorthArabianLetterNoon => OLD_NORTH_ARABIAN_LETTER_NOON,
            OldNorthArabian::OldNorthArabianLetterKhah => OLD_NORTH_ARABIAN_LETTER_KHAH,
            OldNorthArabian::OldNorthArabianLetterSad => OLD_NORTH_ARABIAN_LETTER_SAD,
            OldNorthArabian::OldNorthArabianLetterEsDash3 => OLD_NORTH_ARABIAN_LETTER_ES_DASH_3,
            OldNorthArabian::OldNorthArabianLetterFeh => OLD_NORTH_ARABIAN_LETTER_FEH,
            OldNorthArabian::OldNorthArabianLetterAlef => OLD_NORTH_ARABIAN_LETTER_ALEF,
            OldNorthArabian::OldNorthArabianLetterAin => OLD_NORTH_ARABIAN_LETTER_AIN,
            OldNorthArabian::OldNorthArabianLetterDad => OLD_NORTH_ARABIAN_LETTER_DAD,
            OldNorthArabian::OldNorthArabianLetterGeem => OLD_NORTH_ARABIAN_LETTER_GEEM,
            OldNorthArabian::OldNorthArabianLetterDal => OLD_NORTH_ARABIAN_LETTER_DAL,
            OldNorthArabian::OldNorthArabianLetterGhain => OLD_NORTH_ARABIAN_LETTER_GHAIN,
            OldNorthArabian::OldNorthArabianLetterTah => OLD_NORTH_ARABIAN_LETTER_TAH,
            OldNorthArabian::OldNorthArabianLetterZain => OLD_NORTH_ARABIAN_LETTER_ZAIN,
            OldNorthArabian::OldNorthArabianLetterThal => OLD_NORTH_ARABIAN_LETTER_THAL,
            OldNorthArabian::OldNorthArabianLetterYeh => OLD_NORTH_ARABIAN_LETTER_YEH,
            OldNorthArabian::OldNorthArabianLetterTheh => OLD_NORTH_ARABIAN_LETTER_THEH,
            OldNorthArabian::OldNorthArabianLetterZah => OLD_NORTH_ARABIAN_LETTER_ZAH,
            OldNorthArabian::OldNorthArabianNumberOne => OLD_NORTH_ARABIAN_NUMBER_ONE,
            OldNorthArabian::OldNorthArabianNumberTen => OLD_NORTH_ARABIAN_NUMBER_TEN,
        }
    }
}

impl std::convert::TryFrom<char> for OldNorthArabian {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            OLD_NORTH_ARABIAN_LETTER_HEH => Ok(OldNorthArabian::OldNorthArabianLetterHeh),
            OLD_NORTH_ARABIAN_LETTER_LAM => Ok(OldNorthArabian::OldNorthArabianLetterLam),
            OLD_NORTH_ARABIAN_LETTER_HAH => Ok(OldNorthArabian::OldNorthArabianLetterHah),
            OLD_NORTH_ARABIAN_LETTER_MEEM => Ok(OldNorthArabian::OldNorthArabianLetterMeem),
            OLD_NORTH_ARABIAN_LETTER_QAF => Ok(OldNorthArabian::OldNorthArabianLetterQaf),
            OLD_NORTH_ARABIAN_LETTER_WAW => Ok(OldNorthArabian::OldNorthArabianLetterWaw),
            OLD_NORTH_ARABIAN_LETTER_ES_DASH_2 => Ok(OldNorthArabian::OldNorthArabianLetterEsDash2),
            OLD_NORTH_ARABIAN_LETTER_REH => Ok(OldNorthArabian::OldNorthArabianLetterReh),
            OLD_NORTH_ARABIAN_LETTER_BEH => Ok(OldNorthArabian::OldNorthArabianLetterBeh),
            OLD_NORTH_ARABIAN_LETTER_TEH => Ok(OldNorthArabian::OldNorthArabianLetterTeh),
            OLD_NORTH_ARABIAN_LETTER_ES_DASH_1 => Ok(OldNorthArabian::OldNorthArabianLetterEsDash1),
            OLD_NORTH_ARABIAN_LETTER_KAF => Ok(OldNorthArabian::OldNorthArabianLetterKaf),
            OLD_NORTH_ARABIAN_LETTER_NOON => Ok(OldNorthArabian::OldNorthArabianLetterNoon),
            OLD_NORTH_ARABIAN_LETTER_KHAH => Ok(OldNorthArabian::OldNorthArabianLetterKhah),
            OLD_NORTH_ARABIAN_LETTER_SAD => Ok(OldNorthArabian::OldNorthArabianLetterSad),
            OLD_NORTH_ARABIAN_LETTER_ES_DASH_3 => Ok(OldNorthArabian::OldNorthArabianLetterEsDash3),
            OLD_NORTH_ARABIAN_LETTER_FEH => Ok(OldNorthArabian::OldNorthArabianLetterFeh),
            OLD_NORTH_ARABIAN_LETTER_ALEF => Ok(OldNorthArabian::OldNorthArabianLetterAlef),
            OLD_NORTH_ARABIAN_LETTER_AIN => Ok(OldNorthArabian::OldNorthArabianLetterAin),
            OLD_NORTH_ARABIAN_LETTER_DAD => Ok(OldNorthArabian::OldNorthArabianLetterDad),
            OLD_NORTH_ARABIAN_LETTER_GEEM => Ok(OldNorthArabian::OldNorthArabianLetterGeem),
            OLD_NORTH_ARABIAN_LETTER_DAL => Ok(OldNorthArabian::OldNorthArabianLetterDal),
            OLD_NORTH_ARABIAN_LETTER_GHAIN => Ok(OldNorthArabian::OldNorthArabianLetterGhain),
            OLD_NORTH_ARABIAN_LETTER_TAH => Ok(OldNorthArabian::OldNorthArabianLetterTah),
            OLD_NORTH_ARABIAN_LETTER_ZAIN => Ok(OldNorthArabian::OldNorthArabianLetterZain),
            OLD_NORTH_ARABIAN_LETTER_THAL => Ok(OldNorthArabian::OldNorthArabianLetterThal),
            OLD_NORTH_ARABIAN_LETTER_YEH => Ok(OldNorthArabian::OldNorthArabianLetterYeh),
            OLD_NORTH_ARABIAN_LETTER_THEH => Ok(OldNorthArabian::OldNorthArabianLetterTheh),
            OLD_NORTH_ARABIAN_LETTER_ZAH => Ok(OldNorthArabian::OldNorthArabianLetterZah),
            OLD_NORTH_ARABIAN_NUMBER_ONE => Ok(OldNorthArabian::OldNorthArabianNumberOne),
            OLD_NORTH_ARABIAN_NUMBER_TEN => Ok(OldNorthArabian::OldNorthArabianNumberTen),
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
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        OldNorthArabian::OldNorthArabianLetterHeh
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            OldNorthArabian::OldNorthArabianLetterHeh => "old north arabian letter heh",
            OldNorthArabian::OldNorthArabianLetterLam => "old north arabian letter lam",
            OldNorthArabian::OldNorthArabianLetterHah => "old north arabian letter hah",
            OldNorthArabian::OldNorthArabianLetterMeem => "old north arabian letter meem",
            OldNorthArabian::OldNorthArabianLetterQaf => "old north arabian letter qaf",
            OldNorthArabian::OldNorthArabianLetterWaw => "old north arabian letter waw",
            OldNorthArabian::OldNorthArabianLetterEsDash2 => "old north arabian letter es-2",
            OldNorthArabian::OldNorthArabianLetterReh => "old north arabian letter reh",
            OldNorthArabian::OldNorthArabianLetterBeh => "old north arabian letter beh",
            OldNorthArabian::OldNorthArabianLetterTeh => "old north arabian letter teh",
            OldNorthArabian::OldNorthArabianLetterEsDash1 => "old north arabian letter es-1",
            OldNorthArabian::OldNorthArabianLetterKaf => "old north arabian letter kaf",
            OldNorthArabian::OldNorthArabianLetterNoon => "old north arabian letter noon",
            OldNorthArabian::OldNorthArabianLetterKhah => "old north arabian letter khah",
            OldNorthArabian::OldNorthArabianLetterSad => "old north arabian letter sad",
            OldNorthArabian::OldNorthArabianLetterEsDash3 => "old north arabian letter es-3",
            OldNorthArabian::OldNorthArabianLetterFeh => "old north arabian letter feh",
            OldNorthArabian::OldNorthArabianLetterAlef => "old north arabian letter alef",
            OldNorthArabian::OldNorthArabianLetterAin => "old north arabian letter ain",
            OldNorthArabian::OldNorthArabianLetterDad => "old north arabian letter dad",
            OldNorthArabian::OldNorthArabianLetterGeem => "old north arabian letter geem",
            OldNorthArabian::OldNorthArabianLetterDal => "old north arabian letter dal",
            OldNorthArabian::OldNorthArabianLetterGhain => "old north arabian letter ghain",
            OldNorthArabian::OldNorthArabianLetterTah => "old north arabian letter tah",
            OldNorthArabian::OldNorthArabianLetterZain => "old north arabian letter zain",
            OldNorthArabian::OldNorthArabianLetterThal => "old north arabian letter thal",
            OldNorthArabian::OldNorthArabianLetterYeh => "old north arabian letter yeh",
            OldNorthArabian::OldNorthArabianLetterTheh => "old north arabian letter theh",
            OldNorthArabian::OldNorthArabianLetterZah => "old north arabian letter zah",
            OldNorthArabian::OldNorthArabianNumberOne => "old north arabian number one",
            OldNorthArabian::OldNorthArabianNumberTen => "old north arabian number ten",
        }
    }
}
