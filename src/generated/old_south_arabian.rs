/// \u{10a60} → \u{10a7f}\
///\
/// 𐩠 𐩡 𐩢 𐩣 𐩤 𐩥 𐩦 𐩧 𐩨 𐩩 𐩪 𐩫 𐩬 𐩭 𐩮 𐩯
/// 𐩰 𐩱 𐩲 𐩳 𐩴 𐩵 𐩶 𐩷 𐩸 𐩹 𐩺 𐩻 𐩼 𐩽 𐩾
pub mod constants {
    /// \u{10a60}: '𐩠'
    pub const OLD_SOUTH_ARABIAN_LETTER_HE: char = '𐩠';
    /// \u{10a61}: '𐩡'
    pub const OLD_SOUTH_ARABIAN_LETTER_LAMEDH: char = '𐩡';
    /// \u{10a62}: '𐩢'
    pub const OLD_SOUTH_ARABIAN_LETTER_HETH: char = '𐩢';
    /// \u{10a63}: '𐩣'
    pub const OLD_SOUTH_ARABIAN_LETTER_MEM: char = '𐩣';
    /// \u{10a64}: '𐩤'
    pub const OLD_SOUTH_ARABIAN_LETTER_QOPH: char = '𐩤';
    /// \u{10a65}: '𐩥'
    pub const OLD_SOUTH_ARABIAN_LETTER_WAW: char = '𐩥';
    /// \u{10a66}: '𐩦'
    pub const OLD_SOUTH_ARABIAN_LETTER_SHIN: char = '𐩦';
    /// \u{10a67}: '𐩧'
    pub const OLD_SOUTH_ARABIAN_LETTER_RESH: char = '𐩧';
    /// \u{10a68}: '𐩨'
    pub const OLD_SOUTH_ARABIAN_LETTER_BETH: char = '𐩨';
    /// \u{10a69}: '𐩩'
    pub const OLD_SOUTH_ARABIAN_LETTER_TAW: char = '𐩩';
    /// \u{10a6a}: '𐩪'
    pub const OLD_SOUTH_ARABIAN_LETTER_SAT: char = '𐩪';
    /// \u{10a6b}: '𐩫'
    pub const OLD_SOUTH_ARABIAN_LETTER_KAPH: char = '𐩫';
    /// \u{10a6c}: '𐩬'
    pub const OLD_SOUTH_ARABIAN_LETTER_NUN: char = '𐩬';
    /// \u{10a6d}: '𐩭'
    pub const OLD_SOUTH_ARABIAN_LETTER_KHETH: char = '𐩭';
    /// \u{10a6e}: '𐩮'
    pub const OLD_SOUTH_ARABIAN_LETTER_SADHE: char = '𐩮';
    /// \u{10a6f}: '𐩯'
    pub const OLD_SOUTH_ARABIAN_LETTER_SAMEKH: char = '𐩯';
    /// \u{10a70}: '𐩰'
    pub const OLD_SOUTH_ARABIAN_LETTER_FE: char = '𐩰';
    /// \u{10a71}: '𐩱'
    pub const OLD_SOUTH_ARABIAN_LETTER_ALEF: char = '𐩱';
    /// \u{10a72}: '𐩲'
    pub const OLD_SOUTH_ARABIAN_LETTER_AYN: char = '𐩲';
    /// \u{10a73}: '𐩳'
    pub const OLD_SOUTH_ARABIAN_LETTER_DHADHE: char = '𐩳';
    /// \u{10a74}: '𐩴'
    pub const OLD_SOUTH_ARABIAN_LETTER_GIMEL: char = '𐩴';
    /// \u{10a75}: '𐩵'
    pub const OLD_SOUTH_ARABIAN_LETTER_DALETH: char = '𐩵';
    /// \u{10a76}: '𐩶'
    pub const OLD_SOUTH_ARABIAN_LETTER_GHAYN: char = '𐩶';
    /// \u{10a77}: '𐩷'
    pub const OLD_SOUTH_ARABIAN_LETTER_TETH: char = '𐩷';
    /// \u{10a78}: '𐩸'
    pub const OLD_SOUTH_ARABIAN_LETTER_ZAYN: char = '𐩸';
    /// \u{10a79}: '𐩹'
    pub const OLD_SOUTH_ARABIAN_LETTER_DHALETH: char = '𐩹';
    /// \u{10a7a}: '𐩺'
    pub const OLD_SOUTH_ARABIAN_LETTER_YODH: char = '𐩺';
    /// \u{10a7b}: '𐩻'
    pub const OLD_SOUTH_ARABIAN_LETTER_THAW: char = '𐩻';
    /// \u{10a7c}: '𐩼'
    pub const OLD_SOUTH_ARABIAN_LETTER_THETH: char = '𐩼';
    /// \u{10a7d}: '𐩽'
    pub const OLD_SOUTH_ARABIAN_NUMBER_ONE: char = '𐩽';
    /// \u{10a7e}: '𐩾'
    pub const OLD_SOUTH_ARABIAN_NUMBER_FIFTY: char = '𐩾';
}

/// \u{10a60} → \u{10a7f}\
///\
/// 𐩠 𐩡 𐩢 𐩣 𐩤 𐩥 𐩦 𐩧 𐩨 𐩩 𐩪 𐩫 𐩬 𐩭 𐩮 𐩯
/// 𐩰 𐩱 𐩲 𐩳 𐩴 𐩵 𐩶 𐩷 𐩸 𐩹 𐩺 𐩻 𐩼 𐩽 𐩾
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum OldSouthArabian {
    /// \u{10a60}: '𐩠'
    OldSouthArabianLetterHe,
    /// \u{10a61}: '𐩡'
    OldSouthArabianLetterLamedh,
    /// \u{10a62}: '𐩢'
    OldSouthArabianLetterHeth,
    /// \u{10a63}: '𐩣'
    OldSouthArabianLetterMem,
    /// \u{10a64}: '𐩤'
    OldSouthArabianLetterQoph,
    /// \u{10a65}: '𐩥'
    OldSouthArabianLetterWaw,
    /// \u{10a66}: '𐩦'
    OldSouthArabianLetterShin,
    /// \u{10a67}: '𐩧'
    OldSouthArabianLetterResh,
    /// \u{10a68}: '𐩨'
    OldSouthArabianLetterBeth,
    /// \u{10a69}: '𐩩'
    OldSouthArabianLetterTaw,
    /// \u{10a6a}: '𐩪'
    OldSouthArabianLetterSat,
    /// \u{10a6b}: '𐩫'
    OldSouthArabianLetterKaph,
    /// \u{10a6c}: '𐩬'
    OldSouthArabianLetterNun,
    /// \u{10a6d}: '𐩭'
    OldSouthArabianLetterKheth,
    /// \u{10a6e}: '𐩮'
    OldSouthArabianLetterSadhe,
    /// \u{10a6f}: '𐩯'
    OldSouthArabianLetterSamekh,
    /// \u{10a70}: '𐩰'
    OldSouthArabianLetterFe,
    /// \u{10a71}: '𐩱'
    OldSouthArabianLetterAlef,
    /// \u{10a72}: '𐩲'
    OldSouthArabianLetterAyn,
    /// \u{10a73}: '𐩳'
    OldSouthArabianLetterDhadhe,
    /// \u{10a74}: '𐩴'
    OldSouthArabianLetterGimel,
    /// \u{10a75}: '𐩵'
    OldSouthArabianLetterDaleth,
    /// \u{10a76}: '𐩶'
    OldSouthArabianLetterGhayn,
    /// \u{10a77}: '𐩷'
    OldSouthArabianLetterTeth,
    /// \u{10a78}: '𐩸'
    OldSouthArabianLetterZayn,
    /// \u{10a79}: '𐩹'
    OldSouthArabianLetterDhaleth,
    /// \u{10a7a}: '𐩺'
    OldSouthArabianLetterYodh,
    /// \u{10a7b}: '𐩻'
    OldSouthArabianLetterThaw,
    /// \u{10a7c}: '𐩼'
    OldSouthArabianLetterTheth,
    /// \u{10a7d}: '𐩽'
    OldSouthArabianNumberOne,
    /// \u{10a7e}: '𐩾'
    OldSouthArabianNumberFifty,
}

impl Into<char> for OldSouthArabian {
    fn into(self) -> char {
        use constants::*;
        match self {
            OldSouthArabian::OldSouthArabianLetterHe => OLD_SOUTH_ARABIAN_LETTER_HE,
            OldSouthArabian::OldSouthArabianLetterLamedh => OLD_SOUTH_ARABIAN_LETTER_LAMEDH,
            OldSouthArabian::OldSouthArabianLetterHeth => OLD_SOUTH_ARABIAN_LETTER_HETH,
            OldSouthArabian::OldSouthArabianLetterMem => OLD_SOUTH_ARABIAN_LETTER_MEM,
            OldSouthArabian::OldSouthArabianLetterQoph => OLD_SOUTH_ARABIAN_LETTER_QOPH,
            OldSouthArabian::OldSouthArabianLetterWaw => OLD_SOUTH_ARABIAN_LETTER_WAW,
            OldSouthArabian::OldSouthArabianLetterShin => OLD_SOUTH_ARABIAN_LETTER_SHIN,
            OldSouthArabian::OldSouthArabianLetterResh => OLD_SOUTH_ARABIAN_LETTER_RESH,
            OldSouthArabian::OldSouthArabianLetterBeth => OLD_SOUTH_ARABIAN_LETTER_BETH,
            OldSouthArabian::OldSouthArabianLetterTaw => OLD_SOUTH_ARABIAN_LETTER_TAW,
            OldSouthArabian::OldSouthArabianLetterSat => OLD_SOUTH_ARABIAN_LETTER_SAT,
            OldSouthArabian::OldSouthArabianLetterKaph => OLD_SOUTH_ARABIAN_LETTER_KAPH,
            OldSouthArabian::OldSouthArabianLetterNun => OLD_SOUTH_ARABIAN_LETTER_NUN,
            OldSouthArabian::OldSouthArabianLetterKheth => OLD_SOUTH_ARABIAN_LETTER_KHETH,
            OldSouthArabian::OldSouthArabianLetterSadhe => OLD_SOUTH_ARABIAN_LETTER_SADHE,
            OldSouthArabian::OldSouthArabianLetterSamekh => OLD_SOUTH_ARABIAN_LETTER_SAMEKH,
            OldSouthArabian::OldSouthArabianLetterFe => OLD_SOUTH_ARABIAN_LETTER_FE,
            OldSouthArabian::OldSouthArabianLetterAlef => OLD_SOUTH_ARABIAN_LETTER_ALEF,
            OldSouthArabian::OldSouthArabianLetterAyn => OLD_SOUTH_ARABIAN_LETTER_AYN,
            OldSouthArabian::OldSouthArabianLetterDhadhe => OLD_SOUTH_ARABIAN_LETTER_DHADHE,
            OldSouthArabian::OldSouthArabianLetterGimel => OLD_SOUTH_ARABIAN_LETTER_GIMEL,
            OldSouthArabian::OldSouthArabianLetterDaleth => OLD_SOUTH_ARABIAN_LETTER_DALETH,
            OldSouthArabian::OldSouthArabianLetterGhayn => OLD_SOUTH_ARABIAN_LETTER_GHAYN,
            OldSouthArabian::OldSouthArabianLetterTeth => OLD_SOUTH_ARABIAN_LETTER_TETH,
            OldSouthArabian::OldSouthArabianLetterZayn => OLD_SOUTH_ARABIAN_LETTER_ZAYN,
            OldSouthArabian::OldSouthArabianLetterDhaleth => OLD_SOUTH_ARABIAN_LETTER_DHALETH,
            OldSouthArabian::OldSouthArabianLetterYodh => OLD_SOUTH_ARABIAN_LETTER_YODH,
            OldSouthArabian::OldSouthArabianLetterThaw => OLD_SOUTH_ARABIAN_LETTER_THAW,
            OldSouthArabian::OldSouthArabianLetterTheth => OLD_SOUTH_ARABIAN_LETTER_THETH,
            OldSouthArabian::OldSouthArabianNumberOne => OLD_SOUTH_ARABIAN_NUMBER_ONE,
            OldSouthArabian::OldSouthArabianNumberFifty => OLD_SOUTH_ARABIAN_NUMBER_FIFTY,
        }
    }
}

impl std::convert::TryFrom<char> for OldSouthArabian {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            OLD_SOUTH_ARABIAN_LETTER_HE => Ok(OldSouthArabian::OldSouthArabianLetterHe),
            OLD_SOUTH_ARABIAN_LETTER_LAMEDH => Ok(OldSouthArabian::OldSouthArabianLetterLamedh),
            OLD_SOUTH_ARABIAN_LETTER_HETH => Ok(OldSouthArabian::OldSouthArabianLetterHeth),
            OLD_SOUTH_ARABIAN_LETTER_MEM => Ok(OldSouthArabian::OldSouthArabianLetterMem),
            OLD_SOUTH_ARABIAN_LETTER_QOPH => Ok(OldSouthArabian::OldSouthArabianLetterQoph),
            OLD_SOUTH_ARABIAN_LETTER_WAW => Ok(OldSouthArabian::OldSouthArabianLetterWaw),
            OLD_SOUTH_ARABIAN_LETTER_SHIN => Ok(OldSouthArabian::OldSouthArabianLetterShin),
            OLD_SOUTH_ARABIAN_LETTER_RESH => Ok(OldSouthArabian::OldSouthArabianLetterResh),
            OLD_SOUTH_ARABIAN_LETTER_BETH => Ok(OldSouthArabian::OldSouthArabianLetterBeth),
            OLD_SOUTH_ARABIAN_LETTER_TAW => Ok(OldSouthArabian::OldSouthArabianLetterTaw),
            OLD_SOUTH_ARABIAN_LETTER_SAT => Ok(OldSouthArabian::OldSouthArabianLetterSat),
            OLD_SOUTH_ARABIAN_LETTER_KAPH => Ok(OldSouthArabian::OldSouthArabianLetterKaph),
            OLD_SOUTH_ARABIAN_LETTER_NUN => Ok(OldSouthArabian::OldSouthArabianLetterNun),
            OLD_SOUTH_ARABIAN_LETTER_KHETH => Ok(OldSouthArabian::OldSouthArabianLetterKheth),
            OLD_SOUTH_ARABIAN_LETTER_SADHE => Ok(OldSouthArabian::OldSouthArabianLetterSadhe),
            OLD_SOUTH_ARABIAN_LETTER_SAMEKH => Ok(OldSouthArabian::OldSouthArabianLetterSamekh),
            OLD_SOUTH_ARABIAN_LETTER_FE => Ok(OldSouthArabian::OldSouthArabianLetterFe),
            OLD_SOUTH_ARABIAN_LETTER_ALEF => Ok(OldSouthArabian::OldSouthArabianLetterAlef),
            OLD_SOUTH_ARABIAN_LETTER_AYN => Ok(OldSouthArabian::OldSouthArabianLetterAyn),
            OLD_SOUTH_ARABIAN_LETTER_DHADHE => Ok(OldSouthArabian::OldSouthArabianLetterDhadhe),
            OLD_SOUTH_ARABIAN_LETTER_GIMEL => Ok(OldSouthArabian::OldSouthArabianLetterGimel),
            OLD_SOUTH_ARABIAN_LETTER_DALETH => Ok(OldSouthArabian::OldSouthArabianLetterDaleth),
            OLD_SOUTH_ARABIAN_LETTER_GHAYN => Ok(OldSouthArabian::OldSouthArabianLetterGhayn),
            OLD_SOUTH_ARABIAN_LETTER_TETH => Ok(OldSouthArabian::OldSouthArabianLetterTeth),
            OLD_SOUTH_ARABIAN_LETTER_ZAYN => Ok(OldSouthArabian::OldSouthArabianLetterZayn),
            OLD_SOUTH_ARABIAN_LETTER_DHALETH => Ok(OldSouthArabian::OldSouthArabianLetterDhaleth),
            OLD_SOUTH_ARABIAN_LETTER_YODH => Ok(OldSouthArabian::OldSouthArabianLetterYodh),
            OLD_SOUTH_ARABIAN_LETTER_THAW => Ok(OldSouthArabian::OldSouthArabianLetterThaw),
            OLD_SOUTH_ARABIAN_LETTER_THETH => Ok(OldSouthArabian::OldSouthArabianLetterTheth),
            OLD_SOUTH_ARABIAN_NUMBER_ONE => Ok(OldSouthArabian::OldSouthArabianNumberOne),
            OLD_SOUTH_ARABIAN_NUMBER_FIFTY => Ok(OldSouthArabian::OldSouthArabianNumberFifty),
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
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        OldSouthArabian::OldSouthArabianLetterHe
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            OldSouthArabian::OldSouthArabianLetterHe => "old south arabian letter he",
            OldSouthArabian::OldSouthArabianLetterLamedh => "old south arabian letter lamedh",
            OldSouthArabian::OldSouthArabianLetterHeth => "old south arabian letter heth",
            OldSouthArabian::OldSouthArabianLetterMem => "old south arabian letter mem",
            OldSouthArabian::OldSouthArabianLetterQoph => "old south arabian letter qoph",
            OldSouthArabian::OldSouthArabianLetterWaw => "old south arabian letter waw",
            OldSouthArabian::OldSouthArabianLetterShin => "old south arabian letter shin",
            OldSouthArabian::OldSouthArabianLetterResh => "old south arabian letter resh",
            OldSouthArabian::OldSouthArabianLetterBeth => "old south arabian letter beth",
            OldSouthArabian::OldSouthArabianLetterTaw => "old south arabian letter taw",
            OldSouthArabian::OldSouthArabianLetterSat => "old south arabian letter sat",
            OldSouthArabian::OldSouthArabianLetterKaph => "old south arabian letter kaph",
            OldSouthArabian::OldSouthArabianLetterNun => "old south arabian letter nun",
            OldSouthArabian::OldSouthArabianLetterKheth => "old south arabian letter kheth",
            OldSouthArabian::OldSouthArabianLetterSadhe => "old south arabian letter sadhe",
            OldSouthArabian::OldSouthArabianLetterSamekh => "old south arabian letter samekh",
            OldSouthArabian::OldSouthArabianLetterFe => "old south arabian letter fe",
            OldSouthArabian::OldSouthArabianLetterAlef => "old south arabian letter alef",
            OldSouthArabian::OldSouthArabianLetterAyn => "old south arabian letter ayn",
            OldSouthArabian::OldSouthArabianLetterDhadhe => "old south arabian letter dhadhe",
            OldSouthArabian::OldSouthArabianLetterGimel => "old south arabian letter gimel",
            OldSouthArabian::OldSouthArabianLetterDaleth => "old south arabian letter daleth",
            OldSouthArabian::OldSouthArabianLetterGhayn => "old south arabian letter ghayn",
            OldSouthArabian::OldSouthArabianLetterTeth => "old south arabian letter teth",
            OldSouthArabian::OldSouthArabianLetterZayn => "old south arabian letter zayn",
            OldSouthArabian::OldSouthArabianLetterDhaleth => "old south arabian letter dhaleth",
            OldSouthArabian::OldSouthArabianLetterYodh => "old south arabian letter yodh",
            OldSouthArabian::OldSouthArabianLetterThaw => "old south arabian letter thaw",
            OldSouthArabian::OldSouthArabianLetterTheth => "old south arabian letter theth",
            OldSouthArabian::OldSouthArabianNumberOne => "old south arabian number one",
            OldSouthArabian::OldSouthArabianNumberFifty => "old south arabian number fifty",
        }
    }
}
