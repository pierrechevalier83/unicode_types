/// A number of constants to give a name to all characters in this block.
mod constants {
    /// \u{10b60}: '𐭠'
    pub const LETTER_ALEPH: char = '𐭠';
    /// \u{10b61}: '𐭡'
    pub const LETTER_BETH: char = '𐭡';
    /// \u{10b62}: '𐭢'
    pub const LETTER_GIMEL: char = '𐭢';
    /// \u{10b63}: '𐭣'
    pub const LETTER_DALETH: char = '𐭣';
    /// \u{10b64}: '𐭤'
    pub const LETTER_HE: char = '𐭤';
    /// \u{10b65}: '𐭥'
    pub const LETTER_WAW_DASH_AYIN_DASH_RESH: char = '𐭥';
    /// \u{10b66}: '𐭦'
    pub const LETTER_ZAYIN: char = '𐭦';
    /// \u{10b67}: '𐭧'
    pub const LETTER_HETH: char = '𐭧';
    /// \u{10b68}: '𐭨'
    pub const LETTER_TETH: char = '𐭨';
    /// \u{10b69}: '𐭩'
    pub const LETTER_YODH: char = '𐭩';
    /// \u{10b6a}: '𐭪'
    pub const LETTER_KAPH: char = '𐭪';
    /// \u{10b6b}: '𐭫'
    pub const LETTER_LAMEDH: char = '𐭫';
    /// \u{10b6c}: '𐭬'
    pub const LETTER_MEM_DASH_QOPH: char = '𐭬';
    /// \u{10b6d}: '𐭭'
    pub const LETTER_NUN: char = '𐭭';
    /// \u{10b6e}: '𐭮'
    pub const LETTER_SAMEKH: char = '𐭮';
    /// \u{10b6f}: '𐭯'
    pub const LETTER_PE: char = '𐭯';
    /// \u{10b70}: '𐭰'
    pub const LETTER_SADHE: char = '𐭰';
    /// \u{10b71}: '𐭱'
    pub const LETTER_SHIN: char = '𐭱';
    /// \u{10b72}: '𐭲'
    pub const LETTER_TAW: char = '𐭲';
    /// \u{10b78}: '𐭸'
    pub const NUMBER_ONE: char = '𐭸';
    /// \u{10b79}: '𐭹'
    pub const NUMBER_TWO: char = '𐭹';
    /// \u{10b7a}: '𐭺'
    pub const NUMBER_THREE: char = '𐭺';
    /// \u{10b7b}: '𐭻'
    pub const NUMBER_FOUR: char = '𐭻';
    /// \u{10b7c}: '𐭼'
    pub const NUMBER_TEN: char = '𐭼';
    /// \u{10b7d}: '𐭽'
    pub const NUMBER_TWENTY: char = '𐭽';
    /// \u{10b7e}: '𐭾'
    pub const NUMBER_ONE_HUNDRED: char = '𐭾';
}

/// An enum to represent all characters in the InscriptionalPahlavi block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum InscriptionalPahlavi {
    /// \u{10b60}: '𐭠'
    LetterAleph,
    /// \u{10b61}: '𐭡'
    LetterBeth,
    /// \u{10b62}: '𐭢'
    LetterGimel,
    /// \u{10b63}: '𐭣'
    LetterDaleth,
    /// \u{10b64}: '𐭤'
    LetterHe,
    /// \u{10b65}: '𐭥'
    LetterWawDashAyinDashResh,
    /// \u{10b66}: '𐭦'
    LetterZayin,
    /// \u{10b67}: '𐭧'
    LetterHeth,
    /// \u{10b68}: '𐭨'
    LetterTeth,
    /// \u{10b69}: '𐭩'
    LetterYodh,
    /// \u{10b6a}: '𐭪'
    LetterKaph,
    /// \u{10b6b}: '𐭫'
    LetterLamedh,
    /// \u{10b6c}: '𐭬'
    LetterMemDashQoph,
    /// \u{10b6d}: '𐭭'
    LetterNun,
    /// \u{10b6e}: '𐭮'
    LetterSamekh,
    /// \u{10b6f}: '𐭯'
    LetterPe,
    /// \u{10b70}: '𐭰'
    LetterSadhe,
    /// \u{10b71}: '𐭱'
    LetterShin,
    /// \u{10b72}: '𐭲'
    LetterTaw,
    /// \u{10b78}: '𐭸'
    NumberOne,
    /// \u{10b79}: '𐭹'
    NumberTwo,
    /// \u{10b7a}: '𐭺'
    NumberThree,
    /// \u{10b7b}: '𐭻'
    NumberFour,
    /// \u{10b7c}: '𐭼'
    NumberTen,
    /// \u{10b7d}: '𐭽'
    NumberTwenty,
    /// \u{10b7e}: '𐭾'
    NumberOneHundred,
}

impl Into<char> for InscriptionalPahlavi {
    fn into(self) -> char {
        use constants::*;
        match self {
            InscriptionalPahlavi::LetterAleph => LETTER_ALEPH,
            InscriptionalPahlavi::LetterBeth => LETTER_BETH,
            InscriptionalPahlavi::LetterGimel => LETTER_GIMEL,
            InscriptionalPahlavi::LetterDaleth => LETTER_DALETH,
            InscriptionalPahlavi::LetterHe => LETTER_HE,
            InscriptionalPahlavi::LetterWawDashAyinDashResh => LETTER_WAW_DASH_AYIN_DASH_RESH,
            InscriptionalPahlavi::LetterZayin => LETTER_ZAYIN,
            InscriptionalPahlavi::LetterHeth => LETTER_HETH,
            InscriptionalPahlavi::LetterTeth => LETTER_TETH,
            InscriptionalPahlavi::LetterYodh => LETTER_YODH,
            InscriptionalPahlavi::LetterKaph => LETTER_KAPH,
            InscriptionalPahlavi::LetterLamedh => LETTER_LAMEDH,
            InscriptionalPahlavi::LetterMemDashQoph => LETTER_MEM_DASH_QOPH,
            InscriptionalPahlavi::LetterNun => LETTER_NUN,
            InscriptionalPahlavi::LetterSamekh => LETTER_SAMEKH,
            InscriptionalPahlavi::LetterPe => LETTER_PE,
            InscriptionalPahlavi::LetterSadhe => LETTER_SADHE,
            InscriptionalPahlavi::LetterShin => LETTER_SHIN,
            InscriptionalPahlavi::LetterTaw => LETTER_TAW,
            InscriptionalPahlavi::NumberOne => NUMBER_ONE,
            InscriptionalPahlavi::NumberTwo => NUMBER_TWO,
            InscriptionalPahlavi::NumberThree => NUMBER_THREE,
            InscriptionalPahlavi::NumberFour => NUMBER_FOUR,
            InscriptionalPahlavi::NumberTen => NUMBER_TEN,
            InscriptionalPahlavi::NumberTwenty => NUMBER_TWENTY,
            InscriptionalPahlavi::NumberOneHundred => NUMBER_ONE_HUNDRED,
        }
    }
}

impl std::convert::TryFrom<char> for InscriptionalPahlavi {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_ALEPH => Ok(InscriptionalPahlavi::LetterAleph),
            LETTER_BETH => Ok(InscriptionalPahlavi::LetterBeth),
            LETTER_GIMEL => Ok(InscriptionalPahlavi::LetterGimel),
            LETTER_DALETH => Ok(InscriptionalPahlavi::LetterDaleth),
            LETTER_HE => Ok(InscriptionalPahlavi::LetterHe),
            LETTER_WAW_DASH_AYIN_DASH_RESH => Ok(InscriptionalPahlavi::LetterWawDashAyinDashResh),
            LETTER_ZAYIN => Ok(InscriptionalPahlavi::LetterZayin),
            LETTER_HETH => Ok(InscriptionalPahlavi::LetterHeth),
            LETTER_TETH => Ok(InscriptionalPahlavi::LetterTeth),
            LETTER_YODH => Ok(InscriptionalPahlavi::LetterYodh),
            LETTER_KAPH => Ok(InscriptionalPahlavi::LetterKaph),
            LETTER_LAMEDH => Ok(InscriptionalPahlavi::LetterLamedh),
            LETTER_MEM_DASH_QOPH => Ok(InscriptionalPahlavi::LetterMemDashQoph),
            LETTER_NUN => Ok(InscriptionalPahlavi::LetterNun),
            LETTER_SAMEKH => Ok(InscriptionalPahlavi::LetterSamekh),
            LETTER_PE => Ok(InscriptionalPahlavi::LetterPe),
            LETTER_SADHE => Ok(InscriptionalPahlavi::LetterSadhe),
            LETTER_SHIN => Ok(InscriptionalPahlavi::LetterShin),
            LETTER_TAW => Ok(InscriptionalPahlavi::LetterTaw),
            NUMBER_ONE => Ok(InscriptionalPahlavi::NumberOne),
            NUMBER_TWO => Ok(InscriptionalPahlavi::NumberTwo),
            NUMBER_THREE => Ok(InscriptionalPahlavi::NumberThree),
            NUMBER_FOUR => Ok(InscriptionalPahlavi::NumberFour),
            NUMBER_TEN => Ok(InscriptionalPahlavi::NumberTen),
            NUMBER_TWENTY => Ok(InscriptionalPahlavi::NumberTwenty),
            NUMBER_ONE_HUNDRED => Ok(InscriptionalPahlavi::NumberOneHundred),
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
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        InscriptionalPahlavi::LetterAleph
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            InscriptionalPahlavi::LetterAleph => "inscriptional pahlavi letter aleph",
            InscriptionalPahlavi::LetterBeth => "inscriptional pahlavi letter beth",
            InscriptionalPahlavi::LetterGimel => "inscriptional pahlavi letter gimel",
            InscriptionalPahlavi::LetterDaleth => "inscriptional pahlavi letter daleth",
            InscriptionalPahlavi::LetterHe => "inscriptional pahlavi letter he",
            InscriptionalPahlavi::LetterWawDashAyinDashResh => "inscriptional pahlavi letter waw-ayin-resh",
            InscriptionalPahlavi::LetterZayin => "inscriptional pahlavi letter zayin",
            InscriptionalPahlavi::LetterHeth => "inscriptional pahlavi letter heth",
            InscriptionalPahlavi::LetterTeth => "inscriptional pahlavi letter teth",
            InscriptionalPahlavi::LetterYodh => "inscriptional pahlavi letter yodh",
            InscriptionalPahlavi::LetterKaph => "inscriptional pahlavi letter kaph",
            InscriptionalPahlavi::LetterLamedh => "inscriptional pahlavi letter lamedh",
            InscriptionalPahlavi::LetterMemDashQoph => "inscriptional pahlavi letter mem-qoph",
            InscriptionalPahlavi::LetterNun => "inscriptional pahlavi letter nun",
            InscriptionalPahlavi::LetterSamekh => "inscriptional pahlavi letter samekh",
            InscriptionalPahlavi::LetterPe => "inscriptional pahlavi letter pe",
            InscriptionalPahlavi::LetterSadhe => "inscriptional pahlavi letter sadhe",
            InscriptionalPahlavi::LetterShin => "inscriptional pahlavi letter shin",
            InscriptionalPahlavi::LetterTaw => "inscriptional pahlavi letter taw",
            InscriptionalPahlavi::NumberOne => "inscriptional pahlavi number one",
            InscriptionalPahlavi::NumberTwo => "inscriptional pahlavi number two",
            InscriptionalPahlavi::NumberThree => "inscriptional pahlavi number three",
            InscriptionalPahlavi::NumberFour => "inscriptional pahlavi number four",
            InscriptionalPahlavi::NumberTen => "inscriptional pahlavi number ten",
            InscriptionalPahlavi::NumberTwenty => "inscriptional pahlavi number twenty",
            InscriptionalPahlavi::NumberOneHundred => "inscriptional pahlavi number one hundred",
        }
    }
}
