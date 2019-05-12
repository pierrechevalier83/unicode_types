/// A number of constants to give a name to all characters in this block.
mod constants {
    /// \u{10b80}: '𐮀'
    pub const LETTER_ALEPH: char = '𐮀';
    /// \u{10b81}: '𐮁'
    pub const LETTER_BETH: char = '𐮁';
    /// \u{10b82}: '𐮂'
    pub const LETTER_GIMEL: char = '𐮂';
    /// \u{10b83}: '𐮃'
    pub const LETTER_DALETH: char = '𐮃';
    /// \u{10b84}: '𐮄'
    pub const LETTER_HE: char = '𐮄';
    /// \u{10b85}: '𐮅'
    pub const LETTER_WAW_DASH_AYIN_DASH_RESH: char = '𐮅';
    /// \u{10b86}: '𐮆'
    pub const LETTER_ZAYIN: char = '𐮆';
    /// \u{10b87}: '𐮇'
    pub const LETTER_HETH: char = '𐮇';
    /// \u{10b88}: '𐮈'
    pub const LETTER_YODH: char = '𐮈';
    /// \u{10b89}: '𐮉'
    pub const LETTER_KAPH: char = '𐮉';
    /// \u{10b8a}: '𐮊'
    pub const LETTER_LAMEDH: char = '𐮊';
    /// \u{10b8b}: '𐮋'
    pub const LETTER_MEM_DASH_QOPH: char = '𐮋';
    /// \u{10b8c}: '𐮌'
    pub const LETTER_NUN: char = '𐮌';
    /// \u{10b8d}: '𐮍'
    pub const LETTER_SAMEKH: char = '𐮍';
    /// \u{10b8e}: '𐮎'
    pub const LETTER_PE: char = '𐮎';
    /// \u{10b8f}: '𐮏'
    pub const LETTER_SADHE: char = '𐮏';
    /// \u{10b90}: '𐮐'
    pub const LETTER_SHIN: char = '𐮐';
    /// \u{10b91}: '𐮑'
    pub const LETTER_TAW: char = '𐮑';
    /// \u{10b99}: '𐮙'
    pub const SECTION_MARK: char = '𐮙';
    /// \u{10b9a}: '𐮚'
    pub const TURNED_SECTION_MARK: char = '𐮚';
    /// \u{10b9b}: '𐮛'
    pub const FOUR_DOTS_WITH_CROSS: char = '𐮛';
    /// \u{10b9c}: '𐮜'
    pub const FOUR_DOTS_WITH_DOT: char = '𐮜';
    /// \u{10ba9}: '𐮩'
    pub const NUMBER_ONE: char = '𐮩';
    /// \u{10baa}: '𐮪'
    pub const NUMBER_TWO: char = '𐮪';
    /// \u{10bab}: '𐮫'
    pub const NUMBER_THREE: char = '𐮫';
    /// \u{10bac}: '𐮬'
    pub const NUMBER_FOUR: char = '𐮬';
    /// \u{10bad}: '𐮭'
    pub const NUMBER_TEN: char = '𐮭';
    /// \u{10bae}: '𐮮'
    pub const NUMBER_TWENTY: char = '𐮮';
}

/// An enum to represent all characters in the PsalterPahlavi block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum PsalterPahlavi {
    /// \u{10b80}: '𐮀'
    LetterAleph,
    /// \u{10b81}: '𐮁'
    LetterBeth,
    /// \u{10b82}: '𐮂'
    LetterGimel,
    /// \u{10b83}: '𐮃'
    LetterDaleth,
    /// \u{10b84}: '𐮄'
    LetterHe,
    /// \u{10b85}: '𐮅'
    LetterWawDashAyinDashResh,
    /// \u{10b86}: '𐮆'
    LetterZayin,
    /// \u{10b87}: '𐮇'
    LetterHeth,
    /// \u{10b88}: '𐮈'
    LetterYodh,
    /// \u{10b89}: '𐮉'
    LetterKaph,
    /// \u{10b8a}: '𐮊'
    LetterLamedh,
    /// \u{10b8b}: '𐮋'
    LetterMemDashQoph,
    /// \u{10b8c}: '𐮌'
    LetterNun,
    /// \u{10b8d}: '𐮍'
    LetterSamekh,
    /// \u{10b8e}: '𐮎'
    LetterPe,
    /// \u{10b8f}: '𐮏'
    LetterSadhe,
    /// \u{10b90}: '𐮐'
    LetterShin,
    /// \u{10b91}: '𐮑'
    LetterTaw,
    /// \u{10b99}: '𐮙'
    SectionMark,
    /// \u{10b9a}: '𐮚'
    TurnedSectionMark,
    /// \u{10b9b}: '𐮛'
    FourDotsWithCross,
    /// \u{10b9c}: '𐮜'
    FourDotsWithDot,
    /// \u{10ba9}: '𐮩'
    NumberOne,
    /// \u{10baa}: '𐮪'
    NumberTwo,
    /// \u{10bab}: '𐮫'
    NumberThree,
    /// \u{10bac}: '𐮬'
    NumberFour,
    /// \u{10bad}: '𐮭'
    NumberTen,
    /// \u{10bae}: '𐮮'
    NumberTwenty,
}

impl Into<char> for PsalterPahlavi {
    fn into(self) -> char {
        use constants::*;
        match self {
            PsalterPahlavi::LetterAleph => LETTER_ALEPH,
            PsalterPahlavi::LetterBeth => LETTER_BETH,
            PsalterPahlavi::LetterGimel => LETTER_GIMEL,
            PsalterPahlavi::LetterDaleth => LETTER_DALETH,
            PsalterPahlavi::LetterHe => LETTER_HE,
            PsalterPahlavi::LetterWawDashAyinDashResh => LETTER_WAW_DASH_AYIN_DASH_RESH,
            PsalterPahlavi::LetterZayin => LETTER_ZAYIN,
            PsalterPahlavi::LetterHeth => LETTER_HETH,
            PsalterPahlavi::LetterYodh => LETTER_YODH,
            PsalterPahlavi::LetterKaph => LETTER_KAPH,
            PsalterPahlavi::LetterLamedh => LETTER_LAMEDH,
            PsalterPahlavi::LetterMemDashQoph => LETTER_MEM_DASH_QOPH,
            PsalterPahlavi::LetterNun => LETTER_NUN,
            PsalterPahlavi::LetterSamekh => LETTER_SAMEKH,
            PsalterPahlavi::LetterPe => LETTER_PE,
            PsalterPahlavi::LetterSadhe => LETTER_SADHE,
            PsalterPahlavi::LetterShin => LETTER_SHIN,
            PsalterPahlavi::LetterTaw => LETTER_TAW,
            PsalterPahlavi::SectionMark => SECTION_MARK,
            PsalterPahlavi::TurnedSectionMark => TURNED_SECTION_MARK,
            PsalterPahlavi::FourDotsWithCross => FOUR_DOTS_WITH_CROSS,
            PsalterPahlavi::FourDotsWithDot => FOUR_DOTS_WITH_DOT,
            PsalterPahlavi::NumberOne => NUMBER_ONE,
            PsalterPahlavi::NumberTwo => NUMBER_TWO,
            PsalterPahlavi::NumberThree => NUMBER_THREE,
            PsalterPahlavi::NumberFour => NUMBER_FOUR,
            PsalterPahlavi::NumberTen => NUMBER_TEN,
            PsalterPahlavi::NumberTwenty => NUMBER_TWENTY,
        }
    }
}

impl std::convert::TryFrom<char> for PsalterPahlavi {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_ALEPH => Ok(PsalterPahlavi::LetterAleph),
            LETTER_BETH => Ok(PsalterPahlavi::LetterBeth),
            LETTER_GIMEL => Ok(PsalterPahlavi::LetterGimel),
            LETTER_DALETH => Ok(PsalterPahlavi::LetterDaleth),
            LETTER_HE => Ok(PsalterPahlavi::LetterHe),
            LETTER_WAW_DASH_AYIN_DASH_RESH => Ok(PsalterPahlavi::LetterWawDashAyinDashResh),
            LETTER_ZAYIN => Ok(PsalterPahlavi::LetterZayin),
            LETTER_HETH => Ok(PsalterPahlavi::LetterHeth),
            LETTER_YODH => Ok(PsalterPahlavi::LetterYodh),
            LETTER_KAPH => Ok(PsalterPahlavi::LetterKaph),
            LETTER_LAMEDH => Ok(PsalterPahlavi::LetterLamedh),
            LETTER_MEM_DASH_QOPH => Ok(PsalterPahlavi::LetterMemDashQoph),
            LETTER_NUN => Ok(PsalterPahlavi::LetterNun),
            LETTER_SAMEKH => Ok(PsalterPahlavi::LetterSamekh),
            LETTER_PE => Ok(PsalterPahlavi::LetterPe),
            LETTER_SADHE => Ok(PsalterPahlavi::LetterSadhe),
            LETTER_SHIN => Ok(PsalterPahlavi::LetterShin),
            LETTER_TAW => Ok(PsalterPahlavi::LetterTaw),
            SECTION_MARK => Ok(PsalterPahlavi::SectionMark),
            TURNED_SECTION_MARK => Ok(PsalterPahlavi::TurnedSectionMark),
            FOUR_DOTS_WITH_CROSS => Ok(PsalterPahlavi::FourDotsWithCross),
            FOUR_DOTS_WITH_DOT => Ok(PsalterPahlavi::FourDotsWithDot),
            NUMBER_ONE => Ok(PsalterPahlavi::NumberOne),
            NUMBER_TWO => Ok(PsalterPahlavi::NumberTwo),
            NUMBER_THREE => Ok(PsalterPahlavi::NumberThree),
            NUMBER_FOUR => Ok(PsalterPahlavi::NumberFour),
            NUMBER_TEN => Ok(PsalterPahlavi::NumberTen),
            NUMBER_TWENTY => Ok(PsalterPahlavi::NumberTwenty),
            _ => Err(()),
        }
    }
}

impl Into<u32> for PsalterPahlavi {
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

impl std::convert::TryFrom<u32> for PsalterPahlavi {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for PsalterPahlavi {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl PsalterPahlavi {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        PsalterPahlavi::LetterAleph
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            PsalterPahlavi::LetterAleph => "psalter pahlavi letter aleph",
            PsalterPahlavi::LetterBeth => "psalter pahlavi letter beth",
            PsalterPahlavi::LetterGimel => "psalter pahlavi letter gimel",
            PsalterPahlavi::LetterDaleth => "psalter pahlavi letter daleth",
            PsalterPahlavi::LetterHe => "psalter pahlavi letter he",
            PsalterPahlavi::LetterWawDashAyinDashResh => "psalter pahlavi letter waw-ayin-resh",
            PsalterPahlavi::LetterZayin => "psalter pahlavi letter zayin",
            PsalterPahlavi::LetterHeth => "psalter pahlavi letter heth",
            PsalterPahlavi::LetterYodh => "psalter pahlavi letter yodh",
            PsalterPahlavi::LetterKaph => "psalter pahlavi letter kaph",
            PsalterPahlavi::LetterLamedh => "psalter pahlavi letter lamedh",
            PsalterPahlavi::LetterMemDashQoph => "psalter pahlavi letter mem-qoph",
            PsalterPahlavi::LetterNun => "psalter pahlavi letter nun",
            PsalterPahlavi::LetterSamekh => "psalter pahlavi letter samekh",
            PsalterPahlavi::LetterPe => "psalter pahlavi letter pe",
            PsalterPahlavi::LetterSadhe => "psalter pahlavi letter sadhe",
            PsalterPahlavi::LetterShin => "psalter pahlavi letter shin",
            PsalterPahlavi::LetterTaw => "psalter pahlavi letter taw",
            PsalterPahlavi::SectionMark => "psalter pahlavi section mark",
            PsalterPahlavi::TurnedSectionMark => "psalter pahlavi turned section mark",
            PsalterPahlavi::FourDotsWithCross => "psalter pahlavi four dots with cross",
            PsalterPahlavi::FourDotsWithDot => "psalter pahlavi four dots with dot",
            PsalterPahlavi::NumberOne => "psalter pahlavi number one",
            PsalterPahlavi::NumberTwo => "psalter pahlavi number two",
            PsalterPahlavi::NumberThree => "psalter pahlavi number three",
            PsalterPahlavi::NumberFour => "psalter pahlavi number four",
            PsalterPahlavi::NumberTen => "psalter pahlavi number ten",
            PsalterPahlavi::NumberTwenty => "psalter pahlavi number twenty",
        }
    }
}
