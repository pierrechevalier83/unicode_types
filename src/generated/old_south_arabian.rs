/// \u{10a60} → \u{10a7f}\
///\
/// 𐩠 𐩡 𐩢 𐩣 𐩤 𐩥 𐩦 𐩧 𐩨 𐩩 𐩪 𐩫 𐩬 𐩭 𐩮 𐩯\
/// 𐩰 𐩱 𐩲 𐩳 𐩴 𐩵 𐩶 𐩷 𐩸 𐩹 𐩺 𐩻 𐩼 𐩽 𐩾\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{10a60}: '𐩠'
    pub const LETTER_HE: char = '𐩠';
    /// \u{10a61}: '𐩡'
    pub const LETTER_LAMEDH: char = '𐩡';
    /// \u{10a62}: '𐩢'
    pub const LETTER_HETH: char = '𐩢';
    /// \u{10a63}: '𐩣'
    pub const LETTER_MEM: char = '𐩣';
    /// \u{10a64}: '𐩤'
    pub const LETTER_QOPH: char = '𐩤';
    /// \u{10a65}: '𐩥'
    pub const LETTER_WAW: char = '𐩥';
    /// \u{10a66}: '𐩦'
    pub const LETTER_SHIN: char = '𐩦';
    /// \u{10a67}: '𐩧'
    pub const LETTER_RESH: char = '𐩧';
    /// \u{10a68}: '𐩨'
    pub const LETTER_BETH: char = '𐩨';
    /// \u{10a69}: '𐩩'
    pub const LETTER_TAW: char = '𐩩';
    /// \u{10a6a}: '𐩪'
    pub const LETTER_SAT: char = '𐩪';
    /// \u{10a6b}: '𐩫'
    pub const LETTER_KAPH: char = '𐩫';
    /// \u{10a6c}: '𐩬'
    pub const LETTER_NUN: char = '𐩬';
    /// \u{10a6d}: '𐩭'
    pub const LETTER_KHETH: char = '𐩭';
    /// \u{10a6e}: '𐩮'
    pub const LETTER_SADHE: char = '𐩮';
    /// \u{10a6f}: '𐩯'
    pub const LETTER_SAMEKH: char = '𐩯';
    /// \u{10a70}: '𐩰'
    pub const LETTER_FE: char = '𐩰';
    /// \u{10a71}: '𐩱'
    pub const LETTER_ALEF: char = '𐩱';
    /// \u{10a72}: '𐩲'
    pub const LETTER_AYN: char = '𐩲';
    /// \u{10a73}: '𐩳'
    pub const LETTER_DHADHE: char = '𐩳';
    /// \u{10a74}: '𐩴'
    pub const LETTER_GIMEL: char = '𐩴';
    /// \u{10a75}: '𐩵'
    pub const LETTER_DALETH: char = '𐩵';
    /// \u{10a76}: '𐩶'
    pub const LETTER_GHAYN: char = '𐩶';
    /// \u{10a77}: '𐩷'
    pub const LETTER_TETH: char = '𐩷';
    /// \u{10a78}: '𐩸'
    pub const LETTER_ZAYN: char = '𐩸';
    /// \u{10a79}: '𐩹'
    pub const LETTER_DHALETH: char = '𐩹';
    /// \u{10a7a}: '𐩺'
    pub const LETTER_YODH: char = '𐩺';
    /// \u{10a7b}: '𐩻'
    pub const LETTER_THAW: char = '𐩻';
    /// \u{10a7c}: '𐩼'
    pub const LETTER_THETH: char = '𐩼';
    /// \u{10a7d}: '𐩽'
    pub const NUMBER_ONE: char = '𐩽';
    /// \u{10a7e}: '𐩾'
    pub const NUMBER_FIFTY: char = '𐩾';
}

/// An enum to represent all characters in the OldSouthArabian block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum OldSouthArabian {
    /// \u{10a60}: '𐩠'
    LetterHe,
    /// \u{10a61}: '𐩡'
    LetterLamedh,
    /// \u{10a62}: '𐩢'
    LetterHeth,
    /// \u{10a63}: '𐩣'
    LetterMem,
    /// \u{10a64}: '𐩤'
    LetterQoph,
    /// \u{10a65}: '𐩥'
    LetterWaw,
    /// \u{10a66}: '𐩦'
    LetterShin,
    /// \u{10a67}: '𐩧'
    LetterResh,
    /// \u{10a68}: '𐩨'
    LetterBeth,
    /// \u{10a69}: '𐩩'
    LetterTaw,
    /// \u{10a6a}: '𐩪'
    LetterSat,
    /// \u{10a6b}: '𐩫'
    LetterKaph,
    /// \u{10a6c}: '𐩬'
    LetterNun,
    /// \u{10a6d}: '𐩭'
    LetterKheth,
    /// \u{10a6e}: '𐩮'
    LetterSadhe,
    /// \u{10a6f}: '𐩯'
    LetterSamekh,
    /// \u{10a70}: '𐩰'
    LetterFe,
    /// \u{10a71}: '𐩱'
    LetterAlef,
    /// \u{10a72}: '𐩲'
    LetterAyn,
    /// \u{10a73}: '𐩳'
    LetterDhadhe,
    /// \u{10a74}: '𐩴'
    LetterGimel,
    /// \u{10a75}: '𐩵'
    LetterDaleth,
    /// \u{10a76}: '𐩶'
    LetterGhayn,
    /// \u{10a77}: '𐩷'
    LetterTeth,
    /// \u{10a78}: '𐩸'
    LetterZayn,
    /// \u{10a79}: '𐩹'
    LetterDhaleth,
    /// \u{10a7a}: '𐩺'
    LetterYodh,
    /// \u{10a7b}: '𐩻'
    LetterThaw,
    /// \u{10a7c}: '𐩼'
    LetterTheth,
    /// \u{10a7d}: '𐩽'
    NumberOne,
    /// \u{10a7e}: '𐩾'
    NumberFifty,
}

impl Into<char> for OldSouthArabian {
    fn into(self) -> char {
        use constants::*;
        match self {
            OldSouthArabian::LetterHe => LETTER_HE,
            OldSouthArabian::LetterLamedh => LETTER_LAMEDH,
            OldSouthArabian::LetterHeth => LETTER_HETH,
            OldSouthArabian::LetterMem => LETTER_MEM,
            OldSouthArabian::LetterQoph => LETTER_QOPH,
            OldSouthArabian::LetterWaw => LETTER_WAW,
            OldSouthArabian::LetterShin => LETTER_SHIN,
            OldSouthArabian::LetterResh => LETTER_RESH,
            OldSouthArabian::LetterBeth => LETTER_BETH,
            OldSouthArabian::LetterTaw => LETTER_TAW,
            OldSouthArabian::LetterSat => LETTER_SAT,
            OldSouthArabian::LetterKaph => LETTER_KAPH,
            OldSouthArabian::LetterNun => LETTER_NUN,
            OldSouthArabian::LetterKheth => LETTER_KHETH,
            OldSouthArabian::LetterSadhe => LETTER_SADHE,
            OldSouthArabian::LetterSamekh => LETTER_SAMEKH,
            OldSouthArabian::LetterFe => LETTER_FE,
            OldSouthArabian::LetterAlef => LETTER_ALEF,
            OldSouthArabian::LetterAyn => LETTER_AYN,
            OldSouthArabian::LetterDhadhe => LETTER_DHADHE,
            OldSouthArabian::LetterGimel => LETTER_GIMEL,
            OldSouthArabian::LetterDaleth => LETTER_DALETH,
            OldSouthArabian::LetterGhayn => LETTER_GHAYN,
            OldSouthArabian::LetterTeth => LETTER_TETH,
            OldSouthArabian::LetterZayn => LETTER_ZAYN,
            OldSouthArabian::LetterDhaleth => LETTER_DHALETH,
            OldSouthArabian::LetterYodh => LETTER_YODH,
            OldSouthArabian::LetterThaw => LETTER_THAW,
            OldSouthArabian::LetterTheth => LETTER_THETH,
            OldSouthArabian::NumberOne => NUMBER_ONE,
            OldSouthArabian::NumberFifty => NUMBER_FIFTY,
        }
    }
}

impl std::convert::TryFrom<char> for OldSouthArabian {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_HE => Ok(OldSouthArabian::LetterHe),
            LETTER_LAMEDH => Ok(OldSouthArabian::LetterLamedh),
            LETTER_HETH => Ok(OldSouthArabian::LetterHeth),
            LETTER_MEM => Ok(OldSouthArabian::LetterMem),
            LETTER_QOPH => Ok(OldSouthArabian::LetterQoph),
            LETTER_WAW => Ok(OldSouthArabian::LetterWaw),
            LETTER_SHIN => Ok(OldSouthArabian::LetterShin),
            LETTER_RESH => Ok(OldSouthArabian::LetterResh),
            LETTER_BETH => Ok(OldSouthArabian::LetterBeth),
            LETTER_TAW => Ok(OldSouthArabian::LetterTaw),
            LETTER_SAT => Ok(OldSouthArabian::LetterSat),
            LETTER_KAPH => Ok(OldSouthArabian::LetterKaph),
            LETTER_NUN => Ok(OldSouthArabian::LetterNun),
            LETTER_KHETH => Ok(OldSouthArabian::LetterKheth),
            LETTER_SADHE => Ok(OldSouthArabian::LetterSadhe),
            LETTER_SAMEKH => Ok(OldSouthArabian::LetterSamekh),
            LETTER_FE => Ok(OldSouthArabian::LetterFe),
            LETTER_ALEF => Ok(OldSouthArabian::LetterAlef),
            LETTER_AYN => Ok(OldSouthArabian::LetterAyn),
            LETTER_DHADHE => Ok(OldSouthArabian::LetterDhadhe),
            LETTER_GIMEL => Ok(OldSouthArabian::LetterGimel),
            LETTER_DALETH => Ok(OldSouthArabian::LetterDaleth),
            LETTER_GHAYN => Ok(OldSouthArabian::LetterGhayn),
            LETTER_TETH => Ok(OldSouthArabian::LetterTeth),
            LETTER_ZAYN => Ok(OldSouthArabian::LetterZayn),
            LETTER_DHALETH => Ok(OldSouthArabian::LetterDhaleth),
            LETTER_YODH => Ok(OldSouthArabian::LetterYodh),
            LETTER_THAW => Ok(OldSouthArabian::LetterThaw),
            LETTER_THETH => Ok(OldSouthArabian::LetterTheth),
            NUMBER_ONE => Ok(OldSouthArabian::NumberOne),
            NUMBER_FIFTY => Ok(OldSouthArabian::NumberFifty),
            _ => Err(()),
        }
    }
}

impl Into<u32> for OldSouthArabian {
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

impl std::convert::TryFrom<u32> for OldSouthArabian {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for OldSouthArabian {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl OldSouthArabian {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        OldSouthArabian::LetterHe
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            OldSouthArabian::LetterHe => "old south arabian letter he",
            OldSouthArabian::LetterLamedh => "old south arabian letter lamedh",
            OldSouthArabian::LetterHeth => "old south arabian letter heth",
            OldSouthArabian::LetterMem => "old south arabian letter mem",
            OldSouthArabian::LetterQoph => "old south arabian letter qoph",
            OldSouthArabian::LetterWaw => "old south arabian letter waw",
            OldSouthArabian::LetterShin => "old south arabian letter shin",
            OldSouthArabian::LetterResh => "old south arabian letter resh",
            OldSouthArabian::LetterBeth => "old south arabian letter beth",
            OldSouthArabian::LetterTaw => "old south arabian letter taw",
            OldSouthArabian::LetterSat => "old south arabian letter sat",
            OldSouthArabian::LetterKaph => "old south arabian letter kaph",
            OldSouthArabian::LetterNun => "old south arabian letter nun",
            OldSouthArabian::LetterKheth => "old south arabian letter kheth",
            OldSouthArabian::LetterSadhe => "old south arabian letter sadhe",
            OldSouthArabian::LetterSamekh => "old south arabian letter samekh",
            OldSouthArabian::LetterFe => "old south arabian letter fe",
            OldSouthArabian::LetterAlef => "old south arabian letter alef",
            OldSouthArabian::LetterAyn => "old south arabian letter ayn",
            OldSouthArabian::LetterDhadhe => "old south arabian letter dhadhe",
            OldSouthArabian::LetterGimel => "old south arabian letter gimel",
            OldSouthArabian::LetterDaleth => "old south arabian letter daleth",
            OldSouthArabian::LetterGhayn => "old south arabian letter ghayn",
            OldSouthArabian::LetterTeth => "old south arabian letter teth",
            OldSouthArabian::LetterZayn => "old south arabian letter zayn",
            OldSouthArabian::LetterDhaleth => "old south arabian letter dhaleth",
            OldSouthArabian::LetterYodh => "old south arabian letter yodh",
            OldSouthArabian::LetterThaw => "old south arabian letter thaw",
            OldSouthArabian::LetterTheth => "old south arabian letter theth",
            OldSouthArabian::NumberOne => "old south arabian number one",
            OldSouthArabian::NumberFifty => "old south arabian number fifty",
        }
    }
}
