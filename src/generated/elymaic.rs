/// \u{10fe0} → \u{10fff}\
///\
/// 𐿠 𐿡 𐿢 𐿣 𐿤 𐿥 𐿦 𐿧 𐿨 𐿩 𐿪 𐿫 𐿬 𐿭 𐿮 𐿯\
/// 𐿰 𐿱 𐿲 𐿳 𐿴 𐿵 𐿶\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{10fe0}: '𐿠'
    pub const LETTER_ALEPH: char = '𐿠';
    /// \u{10fe1}: '𐿡'
    pub const LETTER_BETH: char = '𐿡';
    /// \u{10fe2}: '𐿢'
    pub const LETTER_GIMEL: char = '𐿢';
    /// \u{10fe3}: '𐿣'
    pub const LETTER_DALETH: char = '𐿣';
    /// \u{10fe4}: '𐿤'
    pub const LETTER_HE: char = '𐿤';
    /// \u{10fe5}: '𐿥'
    pub const LETTER_WAW: char = '𐿥';
    /// \u{10fe6}: '𐿦'
    pub const LETTER_ZAYIN: char = '𐿦';
    /// \u{10fe7}: '𐿧'
    pub const LETTER_HETH: char = '𐿧';
    /// \u{10fe8}: '𐿨'
    pub const LETTER_TETH: char = '𐿨';
    /// \u{10fe9}: '𐿩'
    pub const LETTER_YODH: char = '𐿩';
    /// \u{10fea}: '𐿪'
    pub const LETTER_KAPH: char = '𐿪';
    /// \u{10feb}: '𐿫'
    pub const LETTER_LAMEDH: char = '𐿫';
    /// \u{10fec}: '𐿬'
    pub const LETTER_MEM: char = '𐿬';
    /// \u{10fed}: '𐿭'
    pub const LETTER_NUN: char = '𐿭';
    /// \u{10fee}: '𐿮'
    pub const LETTER_SAMEKH: char = '𐿮';
    /// \u{10fef}: '𐿯'
    pub const LETTER_AYIN: char = '𐿯';
    /// \u{10ff0}: '𐿰'
    pub const LETTER_PE: char = '𐿰';
    /// \u{10ff1}: '𐿱'
    pub const LETTER_SADHE: char = '𐿱';
    /// \u{10ff2}: '𐿲'
    pub const LETTER_QOPH: char = '𐿲';
    /// \u{10ff3}: '𐿳'
    pub const LETTER_RESH: char = '𐿳';
    /// \u{10ff4}: '𐿴'
    pub const LETTER_SHIN: char = '𐿴';
    /// \u{10ff5}: '𐿵'
    pub const LETTER_TAW: char = '𐿵';
    /// \u{10ff6}: '𐿶'
    pub const LIGATURE_ZAYIN_DASH_YODH: char = '𐿶';
}

/// An enum to represent all characters in the Elymaic block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Elymaic {
    /// \u{10fe0}: '𐿠'
    LetterAleph,
    /// \u{10fe1}: '𐿡'
    LetterBeth,
    /// \u{10fe2}: '𐿢'
    LetterGimel,
    /// \u{10fe3}: '𐿣'
    LetterDaleth,
    /// \u{10fe4}: '𐿤'
    LetterHe,
    /// \u{10fe5}: '𐿥'
    LetterWaw,
    /// \u{10fe6}: '𐿦'
    LetterZayin,
    /// \u{10fe7}: '𐿧'
    LetterHeth,
    /// \u{10fe8}: '𐿨'
    LetterTeth,
    /// \u{10fe9}: '𐿩'
    LetterYodh,
    /// \u{10fea}: '𐿪'
    LetterKaph,
    /// \u{10feb}: '𐿫'
    LetterLamedh,
    /// \u{10fec}: '𐿬'
    LetterMem,
    /// \u{10fed}: '𐿭'
    LetterNun,
    /// \u{10fee}: '𐿮'
    LetterSamekh,
    /// \u{10fef}: '𐿯'
    LetterAyin,
    /// \u{10ff0}: '𐿰'
    LetterPe,
    /// \u{10ff1}: '𐿱'
    LetterSadhe,
    /// \u{10ff2}: '𐿲'
    LetterQoph,
    /// \u{10ff3}: '𐿳'
    LetterResh,
    /// \u{10ff4}: '𐿴'
    LetterShin,
    /// \u{10ff5}: '𐿵'
    LetterTaw,
    /// \u{10ff6}: '𐿶'
    LigatureZayinDashYodh,
}

impl Into<char> for Elymaic {
    fn into(self) -> char {
        use constants::*;
        match self {
            Elymaic::LetterAleph => LETTER_ALEPH,
            Elymaic::LetterBeth => LETTER_BETH,
            Elymaic::LetterGimel => LETTER_GIMEL,
            Elymaic::LetterDaleth => LETTER_DALETH,
            Elymaic::LetterHe => LETTER_HE,
            Elymaic::LetterWaw => LETTER_WAW,
            Elymaic::LetterZayin => LETTER_ZAYIN,
            Elymaic::LetterHeth => LETTER_HETH,
            Elymaic::LetterTeth => LETTER_TETH,
            Elymaic::LetterYodh => LETTER_YODH,
            Elymaic::LetterKaph => LETTER_KAPH,
            Elymaic::LetterLamedh => LETTER_LAMEDH,
            Elymaic::LetterMem => LETTER_MEM,
            Elymaic::LetterNun => LETTER_NUN,
            Elymaic::LetterSamekh => LETTER_SAMEKH,
            Elymaic::LetterAyin => LETTER_AYIN,
            Elymaic::LetterPe => LETTER_PE,
            Elymaic::LetterSadhe => LETTER_SADHE,
            Elymaic::LetterQoph => LETTER_QOPH,
            Elymaic::LetterResh => LETTER_RESH,
            Elymaic::LetterShin => LETTER_SHIN,
            Elymaic::LetterTaw => LETTER_TAW,
            Elymaic::LigatureZayinDashYodh => LIGATURE_ZAYIN_DASH_YODH,
        }
    }
}

impl std::convert::TryFrom<char> for Elymaic {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_ALEPH => Ok(Elymaic::LetterAleph),
            LETTER_BETH => Ok(Elymaic::LetterBeth),
            LETTER_GIMEL => Ok(Elymaic::LetterGimel),
            LETTER_DALETH => Ok(Elymaic::LetterDaleth),
            LETTER_HE => Ok(Elymaic::LetterHe),
            LETTER_WAW => Ok(Elymaic::LetterWaw),
            LETTER_ZAYIN => Ok(Elymaic::LetterZayin),
            LETTER_HETH => Ok(Elymaic::LetterHeth),
            LETTER_TETH => Ok(Elymaic::LetterTeth),
            LETTER_YODH => Ok(Elymaic::LetterYodh),
            LETTER_KAPH => Ok(Elymaic::LetterKaph),
            LETTER_LAMEDH => Ok(Elymaic::LetterLamedh),
            LETTER_MEM => Ok(Elymaic::LetterMem),
            LETTER_NUN => Ok(Elymaic::LetterNun),
            LETTER_SAMEKH => Ok(Elymaic::LetterSamekh),
            LETTER_AYIN => Ok(Elymaic::LetterAyin),
            LETTER_PE => Ok(Elymaic::LetterPe),
            LETTER_SADHE => Ok(Elymaic::LetterSadhe),
            LETTER_QOPH => Ok(Elymaic::LetterQoph),
            LETTER_RESH => Ok(Elymaic::LetterResh),
            LETTER_SHIN => Ok(Elymaic::LetterShin),
            LETTER_TAW => Ok(Elymaic::LetterTaw),
            LIGATURE_ZAYIN_DASH_YODH => Ok(Elymaic::LigatureZayinDashYodh),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Elymaic {
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

impl std::convert::TryFrom<u32> for Elymaic {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Elymaic {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Elymaic {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Elymaic::LetterAleph
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Elymaic::LetterAleph => "elymaic letter aleph",
            Elymaic::LetterBeth => "elymaic letter beth",
            Elymaic::LetterGimel => "elymaic letter gimel",
            Elymaic::LetterDaleth => "elymaic letter daleth",
            Elymaic::LetterHe => "elymaic letter he",
            Elymaic::LetterWaw => "elymaic letter waw",
            Elymaic::LetterZayin => "elymaic letter zayin",
            Elymaic::LetterHeth => "elymaic letter heth",
            Elymaic::LetterTeth => "elymaic letter teth",
            Elymaic::LetterYodh => "elymaic letter yodh",
            Elymaic::LetterKaph => "elymaic letter kaph",
            Elymaic::LetterLamedh => "elymaic letter lamedh",
            Elymaic::LetterMem => "elymaic letter mem",
            Elymaic::LetterNun => "elymaic letter nun",
            Elymaic::LetterSamekh => "elymaic letter samekh",
            Elymaic::LetterAyin => "elymaic letter ayin",
            Elymaic::LetterPe => "elymaic letter pe",
            Elymaic::LetterSadhe => "elymaic letter sadhe",
            Elymaic::LetterQoph => "elymaic letter qoph",
            Elymaic::LetterResh => "elymaic letter resh",
            Elymaic::LetterShin => "elymaic letter shin",
            Elymaic::LetterTaw => "elymaic letter taw",
            Elymaic::LigatureZayinDashYodh => "elymaic ligature zayin-yodh",
        }
    }
}
