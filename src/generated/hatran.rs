/// A number of constants to give a name to all characters in this block.
mod constants {
    /// \u{108e0}: '𐣠'
    pub const LETTER_ALEPH: char = '𐣠';
    /// \u{108e1}: '𐣡'
    pub const LETTER_BETH: char = '𐣡';
    /// \u{108e2}: '𐣢'
    pub const LETTER_GIMEL: char = '𐣢';
    /// \u{108e3}: '𐣣'
    pub const LETTER_DALETH_DASH_RESH: char = '𐣣';
    /// \u{108e4}: '𐣤'
    pub const LETTER_HE: char = '𐣤';
    /// \u{108e5}: '𐣥'
    pub const LETTER_WAW: char = '𐣥';
    /// \u{108e6}: '𐣦'
    pub const LETTER_ZAYN: char = '𐣦';
    /// \u{108e7}: '𐣧'
    pub const LETTER_HETH: char = '𐣧';
    /// \u{108e8}: '𐣨'
    pub const LETTER_TETH: char = '𐣨';
    /// \u{108e9}: '𐣩'
    pub const LETTER_YODH: char = '𐣩';
    /// \u{108ea}: '𐣪'
    pub const LETTER_KAPH: char = '𐣪';
    /// \u{108eb}: '𐣫'
    pub const LETTER_LAMEDH: char = '𐣫';
    /// \u{108ec}: '𐣬'
    pub const LETTER_MEM: char = '𐣬';
    /// \u{108ed}: '𐣭'
    pub const LETTER_NUN: char = '𐣭';
    /// \u{108ee}: '𐣮'
    pub const LETTER_SAMEKH: char = '𐣮';
    /// \u{108ef}: '𐣯'
    pub const LETTER_AYN: char = '𐣯';
    /// \u{108f0}: '𐣰'
    pub const LETTER_PE: char = '𐣰';
    /// \u{108f1}: '𐣱'
    pub const LETTER_SADHE: char = '𐣱';
    /// \u{108f2}: '𐣲'
    pub const LETTER_QOPH: char = '𐣲';
    /// \u{108f4}: '𐣴'
    pub const LETTER_SHIN: char = '𐣴';
    /// \u{108f5}: '𐣵'
    pub const LETTER_TAW: char = '𐣵';
    /// \u{108fb}: '𐣻'
    pub const NUMBER_ONE: char = '𐣻';
    /// \u{108fc}: '𐣼'
    pub const NUMBER_FIVE: char = '𐣼';
    /// \u{108fd}: '𐣽'
    pub const NUMBER_TEN: char = '𐣽';
    /// \u{108fe}: '𐣾'
    pub const NUMBER_TWENTY: char = '𐣾';
}

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
        use constants::*;
        match self {
            Hatran::LetterAleph => LETTER_ALEPH,
            Hatran::LetterBeth => LETTER_BETH,
            Hatran::LetterGimel => LETTER_GIMEL,
            Hatran::LetterDalethDashResh => LETTER_DALETH_DASH_RESH,
            Hatran::LetterHe => LETTER_HE,
            Hatran::LetterWaw => LETTER_WAW,
            Hatran::LetterZayn => LETTER_ZAYN,
            Hatran::LetterHeth => LETTER_HETH,
            Hatran::LetterTeth => LETTER_TETH,
            Hatran::LetterYodh => LETTER_YODH,
            Hatran::LetterKaph => LETTER_KAPH,
            Hatran::LetterLamedh => LETTER_LAMEDH,
            Hatran::LetterMem => LETTER_MEM,
            Hatran::LetterNun => LETTER_NUN,
            Hatran::LetterSamekh => LETTER_SAMEKH,
            Hatran::LetterAyn => LETTER_AYN,
            Hatran::LetterPe => LETTER_PE,
            Hatran::LetterSadhe => LETTER_SADHE,
            Hatran::LetterQoph => LETTER_QOPH,
            Hatran::LetterShin => LETTER_SHIN,
            Hatran::LetterTaw => LETTER_TAW,
            Hatran::NumberOne => NUMBER_ONE,
            Hatran::NumberFive => NUMBER_FIVE,
            Hatran::NumberTen => NUMBER_TEN,
            Hatran::NumberTwenty => NUMBER_TWENTY,
        }
    }
}

impl std::convert::TryFrom<char> for Hatran {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_ALEPH => Ok(Hatran::LetterAleph),
            LETTER_BETH => Ok(Hatran::LetterBeth),
            LETTER_GIMEL => Ok(Hatran::LetterGimel),
            LETTER_DALETH_DASH_RESH => Ok(Hatran::LetterDalethDashResh),
            LETTER_HE => Ok(Hatran::LetterHe),
            LETTER_WAW => Ok(Hatran::LetterWaw),
            LETTER_ZAYN => Ok(Hatran::LetterZayn),
            LETTER_HETH => Ok(Hatran::LetterHeth),
            LETTER_TETH => Ok(Hatran::LetterTeth),
            LETTER_YODH => Ok(Hatran::LetterYodh),
            LETTER_KAPH => Ok(Hatran::LetterKaph),
            LETTER_LAMEDH => Ok(Hatran::LetterLamedh),
            LETTER_MEM => Ok(Hatran::LetterMem),
            LETTER_NUN => Ok(Hatran::LetterNun),
            LETTER_SAMEKH => Ok(Hatran::LetterSamekh),
            LETTER_AYN => Ok(Hatran::LetterAyn),
            LETTER_PE => Ok(Hatran::LetterPe),
            LETTER_SADHE => Ok(Hatran::LetterSadhe),
            LETTER_QOPH => Ok(Hatran::LetterQoph),
            LETTER_SHIN => Ok(Hatran::LetterShin),
            LETTER_TAW => Ok(Hatran::LetterTaw),
            NUMBER_ONE => Ok(Hatran::NumberOne),
            NUMBER_FIVE => Ok(Hatran::NumberFive),
            NUMBER_TEN => Ok(Hatran::NumberTen),
            NUMBER_TWENTY => Ok(Hatran::NumberTwenty),
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
