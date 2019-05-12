/// \u{10b60} → \u{10b7f}\
///\
/// 𐭠 𐭡 𐭢 𐭣 𐭤 𐭥 𐭦 𐭧 𐭨 𐭩 𐭪 𐭫 𐭬 𐭭 𐭮 𐭯
/// 𐭰 𐭱 𐭲 𐭸 𐭹 𐭺 𐭻 𐭼 𐭽 𐭾
pub mod constants {
    /// \u{10b60}: '𐭠'
    pub const INSCRIPTIONAL_PAHLAVI_LETTER_ALEPH: char = '𐭠';
    /// \u{10b61}: '𐭡'
    pub const INSCRIPTIONAL_PAHLAVI_LETTER_BETH: char = '𐭡';
    /// \u{10b62}: '𐭢'
    pub const INSCRIPTIONAL_PAHLAVI_LETTER_GIMEL: char = '𐭢';
    /// \u{10b63}: '𐭣'
    pub const INSCRIPTIONAL_PAHLAVI_LETTER_DALETH: char = '𐭣';
    /// \u{10b64}: '𐭤'
    pub const INSCRIPTIONAL_PAHLAVI_LETTER_HE: char = '𐭤';
    /// \u{10b65}: '𐭥'
    pub const INSCRIPTIONAL_PAHLAVI_LETTER_WAW_DASH_AYIN_DASH_RESH: char = '𐭥';
    /// \u{10b66}: '𐭦'
    pub const INSCRIPTIONAL_PAHLAVI_LETTER_ZAYIN: char = '𐭦';
    /// \u{10b67}: '𐭧'
    pub const INSCRIPTIONAL_PAHLAVI_LETTER_HETH: char = '𐭧';
    /// \u{10b68}: '𐭨'
    pub const INSCRIPTIONAL_PAHLAVI_LETTER_TETH: char = '𐭨';
    /// \u{10b69}: '𐭩'
    pub const INSCRIPTIONAL_PAHLAVI_LETTER_YODH: char = '𐭩';
    /// \u{10b6a}: '𐭪'
    pub const INSCRIPTIONAL_PAHLAVI_LETTER_KAPH: char = '𐭪';
    /// \u{10b6b}: '𐭫'
    pub const INSCRIPTIONAL_PAHLAVI_LETTER_LAMEDH: char = '𐭫';
    /// \u{10b6c}: '𐭬'
    pub const INSCRIPTIONAL_PAHLAVI_LETTER_MEM_DASH_QOPH: char = '𐭬';
    /// \u{10b6d}: '𐭭'
    pub const INSCRIPTIONAL_PAHLAVI_LETTER_NUN: char = '𐭭';
    /// \u{10b6e}: '𐭮'
    pub const INSCRIPTIONAL_PAHLAVI_LETTER_SAMEKH: char = '𐭮';
    /// \u{10b6f}: '𐭯'
    pub const INSCRIPTIONAL_PAHLAVI_LETTER_PE: char = '𐭯';
    /// \u{10b70}: '𐭰'
    pub const INSCRIPTIONAL_PAHLAVI_LETTER_SADHE: char = '𐭰';
    /// \u{10b71}: '𐭱'
    pub const INSCRIPTIONAL_PAHLAVI_LETTER_SHIN: char = '𐭱';
    /// \u{10b72}: '𐭲'
    pub const INSCRIPTIONAL_PAHLAVI_LETTER_TAW: char = '𐭲';
    /// \u{10b78}: '𐭸'
    pub const INSCRIPTIONAL_PAHLAVI_NUMBER_ONE: char = '𐭸';
    /// \u{10b79}: '𐭹'
    pub const INSCRIPTIONAL_PAHLAVI_NUMBER_TWO: char = '𐭹';
    /// \u{10b7a}: '𐭺'
    pub const INSCRIPTIONAL_PAHLAVI_NUMBER_THREE: char = '𐭺';
    /// \u{10b7b}: '𐭻'
    pub const INSCRIPTIONAL_PAHLAVI_NUMBER_FOUR: char = '𐭻';
    /// \u{10b7c}: '𐭼'
    pub const INSCRIPTIONAL_PAHLAVI_NUMBER_TEN: char = '𐭼';
    /// \u{10b7d}: '𐭽'
    pub const INSCRIPTIONAL_PAHLAVI_NUMBER_TWENTY: char = '𐭽';
    /// \u{10b7e}: '𐭾'
    pub const INSCRIPTIONAL_PAHLAVI_NUMBER_ONE_HUNDRED: char = '𐭾';
}

/// \u{10b60} → \u{10b7f}\
///\
/// 𐭠 𐭡 𐭢 𐭣 𐭤 𐭥 𐭦 𐭧 𐭨 𐭩 𐭪 𐭫 𐭬 𐭭 𐭮 𐭯
/// 𐭰 𐭱 𐭲 𐭸 𐭹 𐭺 𐭻 𐭼 𐭽 𐭾
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum InscriptionalPahlavi {
    /// \u{10b60}: '𐭠'
    InscriptionalPahlaviLetterAleph,
    /// \u{10b61}: '𐭡'
    InscriptionalPahlaviLetterBeth,
    /// \u{10b62}: '𐭢'
    InscriptionalPahlaviLetterGimel,
    /// \u{10b63}: '𐭣'
    InscriptionalPahlaviLetterDaleth,
    /// \u{10b64}: '𐭤'
    InscriptionalPahlaviLetterHe,
    /// \u{10b65}: '𐭥'
    InscriptionalPahlaviLetterWawDashAyinDashResh,
    /// \u{10b66}: '𐭦'
    InscriptionalPahlaviLetterZayin,
    /// \u{10b67}: '𐭧'
    InscriptionalPahlaviLetterHeth,
    /// \u{10b68}: '𐭨'
    InscriptionalPahlaviLetterTeth,
    /// \u{10b69}: '𐭩'
    InscriptionalPahlaviLetterYodh,
    /// \u{10b6a}: '𐭪'
    InscriptionalPahlaviLetterKaph,
    /// \u{10b6b}: '𐭫'
    InscriptionalPahlaviLetterLamedh,
    /// \u{10b6c}: '𐭬'
    InscriptionalPahlaviLetterMemDashQoph,
    /// \u{10b6d}: '𐭭'
    InscriptionalPahlaviLetterNun,
    /// \u{10b6e}: '𐭮'
    InscriptionalPahlaviLetterSamekh,
    /// \u{10b6f}: '𐭯'
    InscriptionalPahlaviLetterPe,
    /// \u{10b70}: '𐭰'
    InscriptionalPahlaviLetterSadhe,
    /// \u{10b71}: '𐭱'
    InscriptionalPahlaviLetterShin,
    /// \u{10b72}: '𐭲'
    InscriptionalPahlaviLetterTaw,
    /// \u{10b78}: '𐭸'
    InscriptionalPahlaviNumberOne,
    /// \u{10b79}: '𐭹'
    InscriptionalPahlaviNumberTwo,
    /// \u{10b7a}: '𐭺'
    InscriptionalPahlaviNumberThree,
    /// \u{10b7b}: '𐭻'
    InscriptionalPahlaviNumberFour,
    /// \u{10b7c}: '𐭼'
    InscriptionalPahlaviNumberTen,
    /// \u{10b7d}: '𐭽'
    InscriptionalPahlaviNumberTwenty,
    /// \u{10b7e}: '𐭾'
    InscriptionalPahlaviNumberOneHundred,
}

impl Into<char> for InscriptionalPahlavi {
    fn into(self) -> char {
        use constants::*;
        match self {
            InscriptionalPahlavi::InscriptionalPahlaviLetterAleph => INSCRIPTIONAL_PAHLAVI_LETTER_ALEPH,
            InscriptionalPahlavi::InscriptionalPahlaviLetterBeth => INSCRIPTIONAL_PAHLAVI_LETTER_BETH,
            InscriptionalPahlavi::InscriptionalPahlaviLetterGimel => INSCRIPTIONAL_PAHLAVI_LETTER_GIMEL,
            InscriptionalPahlavi::InscriptionalPahlaviLetterDaleth => INSCRIPTIONAL_PAHLAVI_LETTER_DALETH,
            InscriptionalPahlavi::InscriptionalPahlaviLetterHe => INSCRIPTIONAL_PAHLAVI_LETTER_HE,
            InscriptionalPahlavi::InscriptionalPahlaviLetterWawDashAyinDashResh => INSCRIPTIONAL_PAHLAVI_LETTER_WAW_DASH_AYIN_DASH_RESH,
            InscriptionalPahlavi::InscriptionalPahlaviLetterZayin => INSCRIPTIONAL_PAHLAVI_LETTER_ZAYIN,
            InscriptionalPahlavi::InscriptionalPahlaviLetterHeth => INSCRIPTIONAL_PAHLAVI_LETTER_HETH,
            InscriptionalPahlavi::InscriptionalPahlaviLetterTeth => INSCRIPTIONAL_PAHLAVI_LETTER_TETH,
            InscriptionalPahlavi::InscriptionalPahlaviLetterYodh => INSCRIPTIONAL_PAHLAVI_LETTER_YODH,
            InscriptionalPahlavi::InscriptionalPahlaviLetterKaph => INSCRIPTIONAL_PAHLAVI_LETTER_KAPH,
            InscriptionalPahlavi::InscriptionalPahlaviLetterLamedh => INSCRIPTIONAL_PAHLAVI_LETTER_LAMEDH,
            InscriptionalPahlavi::InscriptionalPahlaviLetterMemDashQoph => INSCRIPTIONAL_PAHLAVI_LETTER_MEM_DASH_QOPH,
            InscriptionalPahlavi::InscriptionalPahlaviLetterNun => INSCRIPTIONAL_PAHLAVI_LETTER_NUN,
            InscriptionalPahlavi::InscriptionalPahlaviLetterSamekh => INSCRIPTIONAL_PAHLAVI_LETTER_SAMEKH,
            InscriptionalPahlavi::InscriptionalPahlaviLetterPe => INSCRIPTIONAL_PAHLAVI_LETTER_PE,
            InscriptionalPahlavi::InscriptionalPahlaviLetterSadhe => INSCRIPTIONAL_PAHLAVI_LETTER_SADHE,
            InscriptionalPahlavi::InscriptionalPahlaviLetterShin => INSCRIPTIONAL_PAHLAVI_LETTER_SHIN,
            InscriptionalPahlavi::InscriptionalPahlaviLetterTaw => INSCRIPTIONAL_PAHLAVI_LETTER_TAW,
            InscriptionalPahlavi::InscriptionalPahlaviNumberOne => INSCRIPTIONAL_PAHLAVI_NUMBER_ONE,
            InscriptionalPahlavi::InscriptionalPahlaviNumberTwo => INSCRIPTIONAL_PAHLAVI_NUMBER_TWO,
            InscriptionalPahlavi::InscriptionalPahlaviNumberThree => INSCRIPTIONAL_PAHLAVI_NUMBER_THREE,
            InscriptionalPahlavi::InscriptionalPahlaviNumberFour => INSCRIPTIONAL_PAHLAVI_NUMBER_FOUR,
            InscriptionalPahlavi::InscriptionalPahlaviNumberTen => INSCRIPTIONAL_PAHLAVI_NUMBER_TEN,
            InscriptionalPahlavi::InscriptionalPahlaviNumberTwenty => INSCRIPTIONAL_PAHLAVI_NUMBER_TWENTY,
            InscriptionalPahlavi::InscriptionalPahlaviNumberOneHundred => INSCRIPTIONAL_PAHLAVI_NUMBER_ONE_HUNDRED,
        }
    }
}

impl std::convert::TryFrom<char> for InscriptionalPahlavi {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            INSCRIPTIONAL_PAHLAVI_LETTER_ALEPH => Ok(InscriptionalPahlavi::InscriptionalPahlaviLetterAleph),
            INSCRIPTIONAL_PAHLAVI_LETTER_BETH => Ok(InscriptionalPahlavi::InscriptionalPahlaviLetterBeth),
            INSCRIPTIONAL_PAHLAVI_LETTER_GIMEL => Ok(InscriptionalPahlavi::InscriptionalPahlaviLetterGimel),
            INSCRIPTIONAL_PAHLAVI_LETTER_DALETH => Ok(InscriptionalPahlavi::InscriptionalPahlaviLetterDaleth),
            INSCRIPTIONAL_PAHLAVI_LETTER_HE => Ok(InscriptionalPahlavi::InscriptionalPahlaviLetterHe),
            INSCRIPTIONAL_PAHLAVI_LETTER_WAW_DASH_AYIN_DASH_RESH => Ok(InscriptionalPahlavi::InscriptionalPahlaviLetterWawDashAyinDashResh),
            INSCRIPTIONAL_PAHLAVI_LETTER_ZAYIN => Ok(InscriptionalPahlavi::InscriptionalPahlaviLetterZayin),
            INSCRIPTIONAL_PAHLAVI_LETTER_HETH => Ok(InscriptionalPahlavi::InscriptionalPahlaviLetterHeth),
            INSCRIPTIONAL_PAHLAVI_LETTER_TETH => Ok(InscriptionalPahlavi::InscriptionalPahlaviLetterTeth),
            INSCRIPTIONAL_PAHLAVI_LETTER_YODH => Ok(InscriptionalPahlavi::InscriptionalPahlaviLetterYodh),
            INSCRIPTIONAL_PAHLAVI_LETTER_KAPH => Ok(InscriptionalPahlavi::InscriptionalPahlaviLetterKaph),
            INSCRIPTIONAL_PAHLAVI_LETTER_LAMEDH => Ok(InscriptionalPahlavi::InscriptionalPahlaviLetterLamedh),
            INSCRIPTIONAL_PAHLAVI_LETTER_MEM_DASH_QOPH => Ok(InscriptionalPahlavi::InscriptionalPahlaviLetterMemDashQoph),
            INSCRIPTIONAL_PAHLAVI_LETTER_NUN => Ok(InscriptionalPahlavi::InscriptionalPahlaviLetterNun),
            INSCRIPTIONAL_PAHLAVI_LETTER_SAMEKH => Ok(InscriptionalPahlavi::InscriptionalPahlaviLetterSamekh),
            INSCRIPTIONAL_PAHLAVI_LETTER_PE => Ok(InscriptionalPahlavi::InscriptionalPahlaviLetterPe),
            INSCRIPTIONAL_PAHLAVI_LETTER_SADHE => Ok(InscriptionalPahlavi::InscriptionalPahlaviLetterSadhe),
            INSCRIPTIONAL_PAHLAVI_LETTER_SHIN => Ok(InscriptionalPahlavi::InscriptionalPahlaviLetterShin),
            INSCRIPTIONAL_PAHLAVI_LETTER_TAW => Ok(InscriptionalPahlavi::InscriptionalPahlaviLetterTaw),
            INSCRIPTIONAL_PAHLAVI_NUMBER_ONE => Ok(InscriptionalPahlavi::InscriptionalPahlaviNumberOne),
            INSCRIPTIONAL_PAHLAVI_NUMBER_TWO => Ok(InscriptionalPahlavi::InscriptionalPahlaviNumberTwo),
            INSCRIPTIONAL_PAHLAVI_NUMBER_THREE => Ok(InscriptionalPahlavi::InscriptionalPahlaviNumberThree),
            INSCRIPTIONAL_PAHLAVI_NUMBER_FOUR => Ok(InscriptionalPahlavi::InscriptionalPahlaviNumberFour),
            INSCRIPTIONAL_PAHLAVI_NUMBER_TEN => Ok(InscriptionalPahlavi::InscriptionalPahlaviNumberTen),
            INSCRIPTIONAL_PAHLAVI_NUMBER_TWENTY => Ok(InscriptionalPahlavi::InscriptionalPahlaviNumberTwenty),
            INSCRIPTIONAL_PAHLAVI_NUMBER_ONE_HUNDRED => Ok(InscriptionalPahlavi::InscriptionalPahlaviNumberOneHundred),
            _ => Err(()),
        }
    }
}

impl Into<u32> for InscriptionalPahlavi {
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

impl std::convert::TryFrom<u32> for InscriptionalPahlavi {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for InscriptionalPahlavi {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl InscriptionalPahlavi {
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        InscriptionalPahlavi::InscriptionalPahlaviLetterAleph
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            InscriptionalPahlavi::InscriptionalPahlaviLetterAleph => "inscriptional pahlavi letter aleph",
            InscriptionalPahlavi::InscriptionalPahlaviLetterBeth => "inscriptional pahlavi letter beth",
            InscriptionalPahlavi::InscriptionalPahlaviLetterGimel => "inscriptional pahlavi letter gimel",
            InscriptionalPahlavi::InscriptionalPahlaviLetterDaleth => "inscriptional pahlavi letter daleth",
            InscriptionalPahlavi::InscriptionalPahlaviLetterHe => "inscriptional pahlavi letter he",
            InscriptionalPahlavi::InscriptionalPahlaviLetterWawDashAyinDashResh => "inscriptional pahlavi letter waw-ayin-resh",
            InscriptionalPahlavi::InscriptionalPahlaviLetterZayin => "inscriptional pahlavi letter zayin",
            InscriptionalPahlavi::InscriptionalPahlaviLetterHeth => "inscriptional pahlavi letter heth",
            InscriptionalPahlavi::InscriptionalPahlaviLetterTeth => "inscriptional pahlavi letter teth",
            InscriptionalPahlavi::InscriptionalPahlaviLetterYodh => "inscriptional pahlavi letter yodh",
            InscriptionalPahlavi::InscriptionalPahlaviLetterKaph => "inscriptional pahlavi letter kaph",
            InscriptionalPahlavi::InscriptionalPahlaviLetterLamedh => "inscriptional pahlavi letter lamedh",
            InscriptionalPahlavi::InscriptionalPahlaviLetterMemDashQoph => "inscriptional pahlavi letter mem-qoph",
            InscriptionalPahlavi::InscriptionalPahlaviLetterNun => "inscriptional pahlavi letter nun",
            InscriptionalPahlavi::InscriptionalPahlaviLetterSamekh => "inscriptional pahlavi letter samekh",
            InscriptionalPahlavi::InscriptionalPahlaviLetterPe => "inscriptional pahlavi letter pe",
            InscriptionalPahlavi::InscriptionalPahlaviLetterSadhe => "inscriptional pahlavi letter sadhe",
            InscriptionalPahlavi::InscriptionalPahlaviLetterShin => "inscriptional pahlavi letter shin",
            InscriptionalPahlavi::InscriptionalPahlaviLetterTaw => "inscriptional pahlavi letter taw",
            InscriptionalPahlavi::InscriptionalPahlaviNumberOne => "inscriptional pahlavi number one",
            InscriptionalPahlavi::InscriptionalPahlaviNumberTwo => "inscriptional pahlavi number two",
            InscriptionalPahlavi::InscriptionalPahlaviNumberThree => "inscriptional pahlavi number three",
            InscriptionalPahlavi::InscriptionalPahlaviNumberFour => "inscriptional pahlavi number four",
            InscriptionalPahlavi::InscriptionalPahlaviNumberTen => "inscriptional pahlavi number ten",
            InscriptionalPahlavi::InscriptionalPahlaviNumberTwenty => "inscriptional pahlavi number twenty",
            InscriptionalPahlavi::InscriptionalPahlaviNumberOneHundred => "inscriptional pahlavi number one hundred",
        }
    }
}
