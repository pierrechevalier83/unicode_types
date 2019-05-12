/// \u{108e0} → \u{108ff}\
///\
/// 𐣠 𐣡 𐣢 𐣣 𐣤 𐣥 𐣦 𐣧 𐣨 𐣩 𐣪 𐣫 𐣬 𐣭 𐣮 𐣯
/// 𐣰 𐣱 𐣲 𐣴 𐣵 𐣻 𐣼 𐣽 𐣾
pub mod constants {
    /// \u{108e0}: '𐣠'
    pub const HATRAN_LETTER_ALEPH: char = '𐣠';
    /// \u{108e1}: '𐣡'
    pub const HATRAN_LETTER_BETH: char = '𐣡';
    /// \u{108e2}: '𐣢'
    pub const HATRAN_LETTER_GIMEL: char = '𐣢';
    /// \u{108e3}: '𐣣'
    pub const HATRAN_LETTER_DALETH_DASH_RESH: char = '𐣣';
    /// \u{108e4}: '𐣤'
    pub const HATRAN_LETTER_HE: char = '𐣤';
    /// \u{108e5}: '𐣥'
    pub const HATRAN_LETTER_WAW: char = '𐣥';
    /// \u{108e6}: '𐣦'
    pub const HATRAN_LETTER_ZAYN: char = '𐣦';
    /// \u{108e7}: '𐣧'
    pub const HATRAN_LETTER_HETH: char = '𐣧';
    /// \u{108e8}: '𐣨'
    pub const HATRAN_LETTER_TETH: char = '𐣨';
    /// \u{108e9}: '𐣩'
    pub const HATRAN_LETTER_YODH: char = '𐣩';
    /// \u{108ea}: '𐣪'
    pub const HATRAN_LETTER_KAPH: char = '𐣪';
    /// \u{108eb}: '𐣫'
    pub const HATRAN_LETTER_LAMEDH: char = '𐣫';
    /// \u{108ec}: '𐣬'
    pub const HATRAN_LETTER_MEM: char = '𐣬';
    /// \u{108ed}: '𐣭'
    pub const HATRAN_LETTER_NUN: char = '𐣭';
    /// \u{108ee}: '𐣮'
    pub const HATRAN_LETTER_SAMEKH: char = '𐣮';
    /// \u{108ef}: '𐣯'
    pub const HATRAN_LETTER_AYN: char = '𐣯';
    /// \u{108f0}: '𐣰'
    pub const HATRAN_LETTER_PE: char = '𐣰';
    /// \u{108f1}: '𐣱'
    pub const HATRAN_LETTER_SADHE: char = '𐣱';
    /// \u{108f2}: '𐣲'
    pub const HATRAN_LETTER_QOPH: char = '𐣲';
    /// \u{108f4}: '𐣴'
    pub const HATRAN_LETTER_SHIN: char = '𐣴';
    /// \u{108f5}: '𐣵'
    pub const HATRAN_LETTER_TAW: char = '𐣵';
    /// \u{108fb}: '𐣻'
    pub const HATRAN_NUMBER_ONE: char = '𐣻';
    /// \u{108fc}: '𐣼'
    pub const HATRAN_NUMBER_FIVE: char = '𐣼';
    /// \u{108fd}: '𐣽'
    pub const HATRAN_NUMBER_TEN: char = '𐣽';
    /// \u{108fe}: '𐣾'
    pub const HATRAN_NUMBER_TWENTY: char = '𐣾';
}

/// \u{108e0} → \u{108ff}\
///\
/// 𐣠 𐣡 𐣢 𐣣 𐣤 𐣥 𐣦 𐣧 𐣨 𐣩 𐣪 𐣫 𐣬 𐣭 𐣮 𐣯
/// 𐣰 𐣱 𐣲 𐣴 𐣵 𐣻 𐣼 𐣽 𐣾
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Hatran {
    /// \u{108e0}: '𐣠'
    HatranLetterAleph,
    /// \u{108e1}: '𐣡'
    HatranLetterBeth,
    /// \u{108e2}: '𐣢'
    HatranLetterGimel,
    /// \u{108e3}: '𐣣'
    HatranLetterDalethDashResh,
    /// \u{108e4}: '𐣤'
    HatranLetterHe,
    /// \u{108e5}: '𐣥'
    HatranLetterWaw,
    /// \u{108e6}: '𐣦'
    HatranLetterZayn,
    /// \u{108e7}: '𐣧'
    HatranLetterHeth,
    /// \u{108e8}: '𐣨'
    HatranLetterTeth,
    /// \u{108e9}: '𐣩'
    HatranLetterYodh,
    /// \u{108ea}: '𐣪'
    HatranLetterKaph,
    /// \u{108eb}: '𐣫'
    HatranLetterLamedh,
    /// \u{108ec}: '𐣬'
    HatranLetterMem,
    /// \u{108ed}: '𐣭'
    HatranLetterNun,
    /// \u{108ee}: '𐣮'
    HatranLetterSamekh,
    /// \u{108ef}: '𐣯'
    HatranLetterAyn,
    /// \u{108f0}: '𐣰'
    HatranLetterPe,
    /// \u{108f1}: '𐣱'
    HatranLetterSadhe,
    /// \u{108f2}: '𐣲'
    HatranLetterQoph,
    /// \u{108f4}: '𐣴'
    HatranLetterShin,
    /// \u{108f5}: '𐣵'
    HatranLetterTaw,
    /// \u{108fb}: '𐣻'
    HatranNumberOne,
    /// \u{108fc}: '𐣼'
    HatranNumberFive,
    /// \u{108fd}: '𐣽'
    HatranNumberTen,
    /// \u{108fe}: '𐣾'
    HatranNumberTwenty,
}

impl Into<char> for Hatran {
    fn into(self) -> char {
        use constants::*;
        match self {
            Hatran::HatranLetterAleph => HATRAN_LETTER_ALEPH,
            Hatran::HatranLetterBeth => HATRAN_LETTER_BETH,
            Hatran::HatranLetterGimel => HATRAN_LETTER_GIMEL,
            Hatran::HatranLetterDalethDashResh => HATRAN_LETTER_DALETH_DASH_RESH,
            Hatran::HatranLetterHe => HATRAN_LETTER_HE,
            Hatran::HatranLetterWaw => HATRAN_LETTER_WAW,
            Hatran::HatranLetterZayn => HATRAN_LETTER_ZAYN,
            Hatran::HatranLetterHeth => HATRAN_LETTER_HETH,
            Hatran::HatranLetterTeth => HATRAN_LETTER_TETH,
            Hatran::HatranLetterYodh => HATRAN_LETTER_YODH,
            Hatran::HatranLetterKaph => HATRAN_LETTER_KAPH,
            Hatran::HatranLetterLamedh => HATRAN_LETTER_LAMEDH,
            Hatran::HatranLetterMem => HATRAN_LETTER_MEM,
            Hatran::HatranLetterNun => HATRAN_LETTER_NUN,
            Hatran::HatranLetterSamekh => HATRAN_LETTER_SAMEKH,
            Hatran::HatranLetterAyn => HATRAN_LETTER_AYN,
            Hatran::HatranLetterPe => HATRAN_LETTER_PE,
            Hatran::HatranLetterSadhe => HATRAN_LETTER_SADHE,
            Hatran::HatranLetterQoph => HATRAN_LETTER_QOPH,
            Hatran::HatranLetterShin => HATRAN_LETTER_SHIN,
            Hatran::HatranLetterTaw => HATRAN_LETTER_TAW,
            Hatran::HatranNumberOne => HATRAN_NUMBER_ONE,
            Hatran::HatranNumberFive => HATRAN_NUMBER_FIVE,
            Hatran::HatranNumberTen => HATRAN_NUMBER_TEN,
            Hatran::HatranNumberTwenty => HATRAN_NUMBER_TWENTY,
        }
    }
}

impl std::convert::TryFrom<char> for Hatran {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            HATRAN_LETTER_ALEPH => Ok(Hatran::HatranLetterAleph),
            HATRAN_LETTER_BETH => Ok(Hatran::HatranLetterBeth),
            HATRAN_LETTER_GIMEL => Ok(Hatran::HatranLetterGimel),
            HATRAN_LETTER_DALETH_DASH_RESH => Ok(Hatran::HatranLetterDalethDashResh),
            HATRAN_LETTER_HE => Ok(Hatran::HatranLetterHe),
            HATRAN_LETTER_WAW => Ok(Hatran::HatranLetterWaw),
            HATRAN_LETTER_ZAYN => Ok(Hatran::HatranLetterZayn),
            HATRAN_LETTER_HETH => Ok(Hatran::HatranLetterHeth),
            HATRAN_LETTER_TETH => Ok(Hatran::HatranLetterTeth),
            HATRAN_LETTER_YODH => Ok(Hatran::HatranLetterYodh),
            HATRAN_LETTER_KAPH => Ok(Hatran::HatranLetterKaph),
            HATRAN_LETTER_LAMEDH => Ok(Hatran::HatranLetterLamedh),
            HATRAN_LETTER_MEM => Ok(Hatran::HatranLetterMem),
            HATRAN_LETTER_NUN => Ok(Hatran::HatranLetterNun),
            HATRAN_LETTER_SAMEKH => Ok(Hatran::HatranLetterSamekh),
            HATRAN_LETTER_AYN => Ok(Hatran::HatranLetterAyn),
            HATRAN_LETTER_PE => Ok(Hatran::HatranLetterPe),
            HATRAN_LETTER_SADHE => Ok(Hatran::HatranLetterSadhe),
            HATRAN_LETTER_QOPH => Ok(Hatran::HatranLetterQoph),
            HATRAN_LETTER_SHIN => Ok(Hatran::HatranLetterShin),
            HATRAN_LETTER_TAW => Ok(Hatran::HatranLetterTaw),
            HATRAN_NUMBER_ONE => Ok(Hatran::HatranNumberOne),
            HATRAN_NUMBER_FIVE => Ok(Hatran::HatranNumberFive),
            HATRAN_NUMBER_TEN => Ok(Hatran::HatranNumberTen),
            HATRAN_NUMBER_TWENTY => Ok(Hatran::HatranNumberTwenty),
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
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        Hatran::HatranLetterAleph
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Hatran::HatranLetterAleph => "hatran letter aleph",
            Hatran::HatranLetterBeth => "hatran letter beth",
            Hatran::HatranLetterGimel => "hatran letter gimel",
            Hatran::HatranLetterDalethDashResh => "hatran letter daleth-resh",
            Hatran::HatranLetterHe => "hatran letter he",
            Hatran::HatranLetterWaw => "hatran letter waw",
            Hatran::HatranLetterZayn => "hatran letter zayn",
            Hatran::HatranLetterHeth => "hatran letter heth",
            Hatran::HatranLetterTeth => "hatran letter teth",
            Hatran::HatranLetterYodh => "hatran letter yodh",
            Hatran::HatranLetterKaph => "hatran letter kaph",
            Hatran::HatranLetterLamedh => "hatran letter lamedh",
            Hatran::HatranLetterMem => "hatran letter mem",
            Hatran::HatranLetterNun => "hatran letter nun",
            Hatran::HatranLetterSamekh => "hatran letter samekh",
            Hatran::HatranLetterAyn => "hatran letter ayn",
            Hatran::HatranLetterPe => "hatran letter pe",
            Hatran::HatranLetterSadhe => "hatran letter sadhe",
            Hatran::HatranLetterQoph => "hatran letter qoph",
            Hatran::HatranLetterShin => "hatran letter shin",
            Hatran::HatranLetterTaw => "hatran letter taw",
            Hatran::HatranNumberOne => "hatran number one",
            Hatran::HatranNumberFive => "hatran number five",
            Hatran::HatranNumberTen => "hatran number ten",
            Hatran::HatranNumberTwenty => "hatran number twenty",
        }
    }
}
