/// \u{10fe0} → \u{10fff}\
///\
/// 𐿠 𐿡 𐿢 𐿣 𐿤 𐿥 𐿦 𐿧 𐿨 𐿩 𐿪 𐿫 𐿬 𐿭 𐿮 𐿯
/// 𐿰 𐿱 𐿲 𐿳 𐿴 𐿵 𐿶
pub mod constants {
    /// \u{10fe0}: '𐿠'
    pub const ELYMAIC_LETTER_ALEPH: char = '𐿠';
    /// \u{10fe1}: '𐿡'
    pub const ELYMAIC_LETTER_BETH: char = '𐿡';
    /// \u{10fe2}: '𐿢'
    pub const ELYMAIC_LETTER_GIMEL: char = '𐿢';
    /// \u{10fe3}: '𐿣'
    pub const ELYMAIC_LETTER_DALETH: char = '𐿣';
    /// \u{10fe4}: '𐿤'
    pub const ELYMAIC_LETTER_HE: char = '𐿤';
    /// \u{10fe5}: '𐿥'
    pub const ELYMAIC_LETTER_WAW: char = '𐿥';
    /// \u{10fe6}: '𐿦'
    pub const ELYMAIC_LETTER_ZAYIN: char = '𐿦';
    /// \u{10fe7}: '𐿧'
    pub const ELYMAIC_LETTER_HETH: char = '𐿧';
    /// \u{10fe8}: '𐿨'
    pub const ELYMAIC_LETTER_TETH: char = '𐿨';
    /// \u{10fe9}: '𐿩'
    pub const ELYMAIC_LETTER_YODH: char = '𐿩';
    /// \u{10fea}: '𐿪'
    pub const ELYMAIC_LETTER_KAPH: char = '𐿪';
    /// \u{10feb}: '𐿫'
    pub const ELYMAIC_LETTER_LAMEDH: char = '𐿫';
    /// \u{10fec}: '𐿬'
    pub const ELYMAIC_LETTER_MEM: char = '𐿬';
    /// \u{10fed}: '𐿭'
    pub const ELYMAIC_LETTER_NUN: char = '𐿭';
    /// \u{10fee}: '𐿮'
    pub const ELYMAIC_LETTER_SAMEKH: char = '𐿮';
    /// \u{10fef}: '𐿯'
    pub const ELYMAIC_LETTER_AYIN: char = '𐿯';
    /// \u{10ff0}: '𐿰'
    pub const ELYMAIC_LETTER_PE: char = '𐿰';
    /// \u{10ff1}: '𐿱'
    pub const ELYMAIC_LETTER_SADHE: char = '𐿱';
    /// \u{10ff2}: '𐿲'
    pub const ELYMAIC_LETTER_QOPH: char = '𐿲';
    /// \u{10ff3}: '𐿳'
    pub const ELYMAIC_LETTER_RESH: char = '𐿳';
    /// \u{10ff4}: '𐿴'
    pub const ELYMAIC_LETTER_SHIN: char = '𐿴';
    /// \u{10ff5}: '𐿵'
    pub const ELYMAIC_LETTER_TAW: char = '𐿵';
    /// \u{10ff6}: '𐿶'
    pub const ELYMAIC_LIGATURE_ZAYIN_DASH_YODH: char = '𐿶';
}

/// \u{10fe0} → \u{10fff}\
///\
/// 𐿠 𐿡 𐿢 𐿣 𐿤 𐿥 𐿦 𐿧 𐿨 𐿩 𐿪 𐿫 𐿬 𐿭 𐿮 𐿯
/// 𐿰 𐿱 𐿲 𐿳 𐿴 𐿵 𐿶
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Elymaic {
    /// \u{10fe0}: '𐿠'
    ElymaicLetterAleph,
    /// \u{10fe1}: '𐿡'
    ElymaicLetterBeth,
    /// \u{10fe2}: '𐿢'
    ElymaicLetterGimel,
    /// \u{10fe3}: '𐿣'
    ElymaicLetterDaleth,
    /// \u{10fe4}: '𐿤'
    ElymaicLetterHe,
    /// \u{10fe5}: '𐿥'
    ElymaicLetterWaw,
    /// \u{10fe6}: '𐿦'
    ElymaicLetterZayin,
    /// \u{10fe7}: '𐿧'
    ElymaicLetterHeth,
    /// \u{10fe8}: '𐿨'
    ElymaicLetterTeth,
    /// \u{10fe9}: '𐿩'
    ElymaicLetterYodh,
    /// \u{10fea}: '𐿪'
    ElymaicLetterKaph,
    /// \u{10feb}: '𐿫'
    ElymaicLetterLamedh,
    /// \u{10fec}: '𐿬'
    ElymaicLetterMem,
    /// \u{10fed}: '𐿭'
    ElymaicLetterNun,
    /// \u{10fee}: '𐿮'
    ElymaicLetterSamekh,
    /// \u{10fef}: '𐿯'
    ElymaicLetterAyin,
    /// \u{10ff0}: '𐿰'
    ElymaicLetterPe,
    /// \u{10ff1}: '𐿱'
    ElymaicLetterSadhe,
    /// \u{10ff2}: '𐿲'
    ElymaicLetterQoph,
    /// \u{10ff3}: '𐿳'
    ElymaicLetterResh,
    /// \u{10ff4}: '𐿴'
    ElymaicLetterShin,
    /// \u{10ff5}: '𐿵'
    ElymaicLetterTaw,
    /// \u{10ff6}: '𐿶'
    ElymaicLigatureZayinDashYodh,
}

impl Into<char> for Elymaic {
    fn into(self) -> char {
        use constants::*;
        match self {
            Elymaic::ElymaicLetterAleph => ELYMAIC_LETTER_ALEPH,
            Elymaic::ElymaicLetterBeth => ELYMAIC_LETTER_BETH,
            Elymaic::ElymaicLetterGimel => ELYMAIC_LETTER_GIMEL,
            Elymaic::ElymaicLetterDaleth => ELYMAIC_LETTER_DALETH,
            Elymaic::ElymaicLetterHe => ELYMAIC_LETTER_HE,
            Elymaic::ElymaicLetterWaw => ELYMAIC_LETTER_WAW,
            Elymaic::ElymaicLetterZayin => ELYMAIC_LETTER_ZAYIN,
            Elymaic::ElymaicLetterHeth => ELYMAIC_LETTER_HETH,
            Elymaic::ElymaicLetterTeth => ELYMAIC_LETTER_TETH,
            Elymaic::ElymaicLetterYodh => ELYMAIC_LETTER_YODH,
            Elymaic::ElymaicLetterKaph => ELYMAIC_LETTER_KAPH,
            Elymaic::ElymaicLetterLamedh => ELYMAIC_LETTER_LAMEDH,
            Elymaic::ElymaicLetterMem => ELYMAIC_LETTER_MEM,
            Elymaic::ElymaicLetterNun => ELYMAIC_LETTER_NUN,
            Elymaic::ElymaicLetterSamekh => ELYMAIC_LETTER_SAMEKH,
            Elymaic::ElymaicLetterAyin => ELYMAIC_LETTER_AYIN,
            Elymaic::ElymaicLetterPe => ELYMAIC_LETTER_PE,
            Elymaic::ElymaicLetterSadhe => ELYMAIC_LETTER_SADHE,
            Elymaic::ElymaicLetterQoph => ELYMAIC_LETTER_QOPH,
            Elymaic::ElymaicLetterResh => ELYMAIC_LETTER_RESH,
            Elymaic::ElymaicLetterShin => ELYMAIC_LETTER_SHIN,
            Elymaic::ElymaicLetterTaw => ELYMAIC_LETTER_TAW,
            Elymaic::ElymaicLigatureZayinDashYodh => ELYMAIC_LIGATURE_ZAYIN_DASH_YODH,
        }
    }
}

impl std::convert::TryFrom<char> for Elymaic {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            ELYMAIC_LETTER_ALEPH => Ok(Elymaic::ElymaicLetterAleph),
            ELYMAIC_LETTER_BETH => Ok(Elymaic::ElymaicLetterBeth),
            ELYMAIC_LETTER_GIMEL => Ok(Elymaic::ElymaicLetterGimel),
            ELYMAIC_LETTER_DALETH => Ok(Elymaic::ElymaicLetterDaleth),
            ELYMAIC_LETTER_HE => Ok(Elymaic::ElymaicLetterHe),
            ELYMAIC_LETTER_WAW => Ok(Elymaic::ElymaicLetterWaw),
            ELYMAIC_LETTER_ZAYIN => Ok(Elymaic::ElymaicLetterZayin),
            ELYMAIC_LETTER_HETH => Ok(Elymaic::ElymaicLetterHeth),
            ELYMAIC_LETTER_TETH => Ok(Elymaic::ElymaicLetterTeth),
            ELYMAIC_LETTER_YODH => Ok(Elymaic::ElymaicLetterYodh),
            ELYMAIC_LETTER_KAPH => Ok(Elymaic::ElymaicLetterKaph),
            ELYMAIC_LETTER_LAMEDH => Ok(Elymaic::ElymaicLetterLamedh),
            ELYMAIC_LETTER_MEM => Ok(Elymaic::ElymaicLetterMem),
            ELYMAIC_LETTER_NUN => Ok(Elymaic::ElymaicLetterNun),
            ELYMAIC_LETTER_SAMEKH => Ok(Elymaic::ElymaicLetterSamekh),
            ELYMAIC_LETTER_AYIN => Ok(Elymaic::ElymaicLetterAyin),
            ELYMAIC_LETTER_PE => Ok(Elymaic::ElymaicLetterPe),
            ELYMAIC_LETTER_SADHE => Ok(Elymaic::ElymaicLetterSadhe),
            ELYMAIC_LETTER_QOPH => Ok(Elymaic::ElymaicLetterQoph),
            ELYMAIC_LETTER_RESH => Ok(Elymaic::ElymaicLetterResh),
            ELYMAIC_LETTER_SHIN => Ok(Elymaic::ElymaicLetterShin),
            ELYMAIC_LETTER_TAW => Ok(Elymaic::ElymaicLetterTaw),
            ELYMAIC_LIGATURE_ZAYIN_DASH_YODH => Ok(Elymaic::ElymaicLigatureZayinDashYodh),
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
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        Elymaic::ElymaicLetterAleph
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Elymaic::ElymaicLetterAleph => "elymaic letter aleph",
            Elymaic::ElymaicLetterBeth => "elymaic letter beth",
            Elymaic::ElymaicLetterGimel => "elymaic letter gimel",
            Elymaic::ElymaicLetterDaleth => "elymaic letter daleth",
            Elymaic::ElymaicLetterHe => "elymaic letter he",
            Elymaic::ElymaicLetterWaw => "elymaic letter waw",
            Elymaic::ElymaicLetterZayin => "elymaic letter zayin",
            Elymaic::ElymaicLetterHeth => "elymaic letter heth",
            Elymaic::ElymaicLetterTeth => "elymaic letter teth",
            Elymaic::ElymaicLetterYodh => "elymaic letter yodh",
            Elymaic::ElymaicLetterKaph => "elymaic letter kaph",
            Elymaic::ElymaicLetterLamedh => "elymaic letter lamedh",
            Elymaic::ElymaicLetterMem => "elymaic letter mem",
            Elymaic::ElymaicLetterNun => "elymaic letter nun",
            Elymaic::ElymaicLetterSamekh => "elymaic letter samekh",
            Elymaic::ElymaicLetterAyin => "elymaic letter ayin",
            Elymaic::ElymaicLetterPe => "elymaic letter pe",
            Elymaic::ElymaicLetterSadhe => "elymaic letter sadhe",
            Elymaic::ElymaicLetterQoph => "elymaic letter qoph",
            Elymaic::ElymaicLetterResh => "elymaic letter resh",
            Elymaic::ElymaicLetterShin => "elymaic letter shin",
            Elymaic::ElymaicLetterTaw => "elymaic letter taw",
            Elymaic::ElymaicLigatureZayinDashYodh => "elymaic ligature zayin-yodh",
        }
    }
}
