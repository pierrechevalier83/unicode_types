/// \u{10b80} → \u{10baf}\
///\
/// 𐮀 𐮁 𐮂 𐮃 𐮄 𐮅 𐮆 𐮇 𐮈 𐮉 𐮊 𐮋 𐮌 𐮍 𐮎 𐮏
/// 𐮐 𐮑 𐮙 𐮚 𐮛 𐮜 𐮩 𐮪 𐮫 𐮬 𐮭 𐮮
pub mod constants {
    /// \u{10b80}: '𐮀'
    pub const PSALTER_PAHLAVI_LETTER_ALEPH: char = '𐮀';
    /// \u{10b81}: '𐮁'
    pub const PSALTER_PAHLAVI_LETTER_BETH: char = '𐮁';
    /// \u{10b82}: '𐮂'
    pub const PSALTER_PAHLAVI_LETTER_GIMEL: char = '𐮂';
    /// \u{10b83}: '𐮃'
    pub const PSALTER_PAHLAVI_LETTER_DALETH: char = '𐮃';
    /// \u{10b84}: '𐮄'
    pub const PSALTER_PAHLAVI_LETTER_HE: char = '𐮄';
    /// \u{10b85}: '𐮅'
    pub const PSALTER_PAHLAVI_LETTER_WAW_DASH_AYIN_DASH_RESH: char = '𐮅';
    /// \u{10b86}: '𐮆'
    pub const PSALTER_PAHLAVI_LETTER_ZAYIN: char = '𐮆';
    /// \u{10b87}: '𐮇'
    pub const PSALTER_PAHLAVI_LETTER_HETH: char = '𐮇';
    /// \u{10b88}: '𐮈'
    pub const PSALTER_PAHLAVI_LETTER_YODH: char = '𐮈';
    /// \u{10b89}: '𐮉'
    pub const PSALTER_PAHLAVI_LETTER_KAPH: char = '𐮉';
    /// \u{10b8a}: '𐮊'
    pub const PSALTER_PAHLAVI_LETTER_LAMEDH: char = '𐮊';
    /// \u{10b8b}: '𐮋'
    pub const PSALTER_PAHLAVI_LETTER_MEM_DASH_QOPH: char = '𐮋';
    /// \u{10b8c}: '𐮌'
    pub const PSALTER_PAHLAVI_LETTER_NUN: char = '𐮌';
    /// \u{10b8d}: '𐮍'
    pub const PSALTER_PAHLAVI_LETTER_SAMEKH: char = '𐮍';
    /// \u{10b8e}: '𐮎'
    pub const PSALTER_PAHLAVI_LETTER_PE: char = '𐮎';
    /// \u{10b8f}: '𐮏'
    pub const PSALTER_PAHLAVI_LETTER_SADHE: char = '𐮏';
    /// \u{10b90}: '𐮐'
    pub const PSALTER_PAHLAVI_LETTER_SHIN: char = '𐮐';
    /// \u{10b91}: '𐮑'
    pub const PSALTER_PAHLAVI_LETTER_TAW: char = '𐮑';
    /// \u{10b99}: '𐮙'
    pub const PSALTER_PAHLAVI_SECTION_MARK: char = '𐮙';
    /// \u{10b9a}: '𐮚'
    pub const PSALTER_PAHLAVI_TURNED_SECTION_MARK: char = '𐮚';
    /// \u{10b9b}: '𐮛'
    pub const PSALTER_PAHLAVI_FOUR_DOTS_WITH_CROSS: char = '𐮛';
    /// \u{10b9c}: '𐮜'
    pub const PSALTER_PAHLAVI_FOUR_DOTS_WITH_DOT: char = '𐮜';
    /// \u{10ba9}: '𐮩'
    pub const PSALTER_PAHLAVI_NUMBER_ONE: char = '𐮩';
    /// \u{10baa}: '𐮪'
    pub const PSALTER_PAHLAVI_NUMBER_TWO: char = '𐮪';
    /// \u{10bab}: '𐮫'
    pub const PSALTER_PAHLAVI_NUMBER_THREE: char = '𐮫';
    /// \u{10bac}: '𐮬'
    pub const PSALTER_PAHLAVI_NUMBER_FOUR: char = '𐮬';
    /// \u{10bad}: '𐮭'
    pub const PSALTER_PAHLAVI_NUMBER_TEN: char = '𐮭';
    /// \u{10bae}: '𐮮'
    pub const PSALTER_PAHLAVI_NUMBER_TWENTY: char = '𐮮';
}

/// \u{10b80} → \u{10baf}\
///\
/// 𐮀 𐮁 𐮂 𐮃 𐮄 𐮅 𐮆 𐮇 𐮈 𐮉 𐮊 𐮋 𐮌 𐮍 𐮎 𐮏
/// 𐮐 𐮑 𐮙 𐮚 𐮛 𐮜 𐮩 𐮪 𐮫 𐮬 𐮭 𐮮
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum PsalterPahlavi {
    /// \u{10b80}: '𐮀'
    PsalterPahlaviLetterAleph,
    /// \u{10b81}: '𐮁'
    PsalterPahlaviLetterBeth,
    /// \u{10b82}: '𐮂'
    PsalterPahlaviLetterGimel,
    /// \u{10b83}: '𐮃'
    PsalterPahlaviLetterDaleth,
    /// \u{10b84}: '𐮄'
    PsalterPahlaviLetterHe,
    /// \u{10b85}: '𐮅'
    PsalterPahlaviLetterWawDashAyinDashResh,
    /// \u{10b86}: '𐮆'
    PsalterPahlaviLetterZayin,
    /// \u{10b87}: '𐮇'
    PsalterPahlaviLetterHeth,
    /// \u{10b88}: '𐮈'
    PsalterPahlaviLetterYodh,
    /// \u{10b89}: '𐮉'
    PsalterPahlaviLetterKaph,
    /// \u{10b8a}: '𐮊'
    PsalterPahlaviLetterLamedh,
    /// \u{10b8b}: '𐮋'
    PsalterPahlaviLetterMemDashQoph,
    /// \u{10b8c}: '𐮌'
    PsalterPahlaviLetterNun,
    /// \u{10b8d}: '𐮍'
    PsalterPahlaviLetterSamekh,
    /// \u{10b8e}: '𐮎'
    PsalterPahlaviLetterPe,
    /// \u{10b8f}: '𐮏'
    PsalterPahlaviLetterSadhe,
    /// \u{10b90}: '𐮐'
    PsalterPahlaviLetterShin,
    /// \u{10b91}: '𐮑'
    PsalterPahlaviLetterTaw,
    /// \u{10b99}: '𐮙'
    PsalterPahlaviSectionMark,
    /// \u{10b9a}: '𐮚'
    PsalterPahlaviTurnedSectionMark,
    /// \u{10b9b}: '𐮛'
    PsalterPahlaviFourDotsWithCross,
    /// \u{10b9c}: '𐮜'
    PsalterPahlaviFourDotsWithDot,
    /// \u{10ba9}: '𐮩'
    PsalterPahlaviNumberOne,
    /// \u{10baa}: '𐮪'
    PsalterPahlaviNumberTwo,
    /// \u{10bab}: '𐮫'
    PsalterPahlaviNumberThree,
    /// \u{10bac}: '𐮬'
    PsalterPahlaviNumberFour,
    /// \u{10bad}: '𐮭'
    PsalterPahlaviNumberTen,
    /// \u{10bae}: '𐮮'
    PsalterPahlaviNumberTwenty,
}

impl Into<char> for PsalterPahlavi {
    fn into(self) -> char {
        use constants::*;
        match self {
            PsalterPahlavi::PsalterPahlaviLetterAleph => PSALTER_PAHLAVI_LETTER_ALEPH,
            PsalterPahlavi::PsalterPahlaviLetterBeth => PSALTER_PAHLAVI_LETTER_BETH,
            PsalterPahlavi::PsalterPahlaviLetterGimel => PSALTER_PAHLAVI_LETTER_GIMEL,
            PsalterPahlavi::PsalterPahlaviLetterDaleth => PSALTER_PAHLAVI_LETTER_DALETH,
            PsalterPahlavi::PsalterPahlaviLetterHe => PSALTER_PAHLAVI_LETTER_HE,
            PsalterPahlavi::PsalterPahlaviLetterWawDashAyinDashResh => PSALTER_PAHLAVI_LETTER_WAW_DASH_AYIN_DASH_RESH,
            PsalterPahlavi::PsalterPahlaviLetterZayin => PSALTER_PAHLAVI_LETTER_ZAYIN,
            PsalterPahlavi::PsalterPahlaviLetterHeth => PSALTER_PAHLAVI_LETTER_HETH,
            PsalterPahlavi::PsalterPahlaviLetterYodh => PSALTER_PAHLAVI_LETTER_YODH,
            PsalterPahlavi::PsalterPahlaviLetterKaph => PSALTER_PAHLAVI_LETTER_KAPH,
            PsalterPahlavi::PsalterPahlaviLetterLamedh => PSALTER_PAHLAVI_LETTER_LAMEDH,
            PsalterPahlavi::PsalterPahlaviLetterMemDashQoph => PSALTER_PAHLAVI_LETTER_MEM_DASH_QOPH,
            PsalterPahlavi::PsalterPahlaviLetterNun => PSALTER_PAHLAVI_LETTER_NUN,
            PsalterPahlavi::PsalterPahlaviLetterSamekh => PSALTER_PAHLAVI_LETTER_SAMEKH,
            PsalterPahlavi::PsalterPahlaviLetterPe => PSALTER_PAHLAVI_LETTER_PE,
            PsalterPahlavi::PsalterPahlaviLetterSadhe => PSALTER_PAHLAVI_LETTER_SADHE,
            PsalterPahlavi::PsalterPahlaviLetterShin => PSALTER_PAHLAVI_LETTER_SHIN,
            PsalterPahlavi::PsalterPahlaviLetterTaw => PSALTER_PAHLAVI_LETTER_TAW,
            PsalterPahlavi::PsalterPahlaviSectionMark => PSALTER_PAHLAVI_SECTION_MARK,
            PsalterPahlavi::PsalterPahlaviTurnedSectionMark => PSALTER_PAHLAVI_TURNED_SECTION_MARK,
            PsalterPahlavi::PsalterPahlaviFourDotsWithCross => PSALTER_PAHLAVI_FOUR_DOTS_WITH_CROSS,
            PsalterPahlavi::PsalterPahlaviFourDotsWithDot => PSALTER_PAHLAVI_FOUR_DOTS_WITH_DOT,
            PsalterPahlavi::PsalterPahlaviNumberOne => PSALTER_PAHLAVI_NUMBER_ONE,
            PsalterPahlavi::PsalterPahlaviNumberTwo => PSALTER_PAHLAVI_NUMBER_TWO,
            PsalterPahlavi::PsalterPahlaviNumberThree => PSALTER_PAHLAVI_NUMBER_THREE,
            PsalterPahlavi::PsalterPahlaviNumberFour => PSALTER_PAHLAVI_NUMBER_FOUR,
            PsalterPahlavi::PsalterPahlaviNumberTen => PSALTER_PAHLAVI_NUMBER_TEN,
            PsalterPahlavi::PsalterPahlaviNumberTwenty => PSALTER_PAHLAVI_NUMBER_TWENTY,
        }
    }
}

impl std::convert::TryFrom<char> for PsalterPahlavi {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            PSALTER_PAHLAVI_LETTER_ALEPH => Ok(PsalterPahlavi::PsalterPahlaviLetterAleph),
            PSALTER_PAHLAVI_LETTER_BETH => Ok(PsalterPahlavi::PsalterPahlaviLetterBeth),
            PSALTER_PAHLAVI_LETTER_GIMEL => Ok(PsalterPahlavi::PsalterPahlaviLetterGimel),
            PSALTER_PAHLAVI_LETTER_DALETH => Ok(PsalterPahlavi::PsalterPahlaviLetterDaleth),
            PSALTER_PAHLAVI_LETTER_HE => Ok(PsalterPahlavi::PsalterPahlaviLetterHe),
            PSALTER_PAHLAVI_LETTER_WAW_DASH_AYIN_DASH_RESH => Ok(PsalterPahlavi::PsalterPahlaviLetterWawDashAyinDashResh),
            PSALTER_PAHLAVI_LETTER_ZAYIN => Ok(PsalterPahlavi::PsalterPahlaviLetterZayin),
            PSALTER_PAHLAVI_LETTER_HETH => Ok(PsalterPahlavi::PsalterPahlaviLetterHeth),
            PSALTER_PAHLAVI_LETTER_YODH => Ok(PsalterPahlavi::PsalterPahlaviLetterYodh),
            PSALTER_PAHLAVI_LETTER_KAPH => Ok(PsalterPahlavi::PsalterPahlaviLetterKaph),
            PSALTER_PAHLAVI_LETTER_LAMEDH => Ok(PsalterPahlavi::PsalterPahlaviLetterLamedh),
            PSALTER_PAHLAVI_LETTER_MEM_DASH_QOPH => Ok(PsalterPahlavi::PsalterPahlaviLetterMemDashQoph),
            PSALTER_PAHLAVI_LETTER_NUN => Ok(PsalterPahlavi::PsalterPahlaviLetterNun),
            PSALTER_PAHLAVI_LETTER_SAMEKH => Ok(PsalterPahlavi::PsalterPahlaviLetterSamekh),
            PSALTER_PAHLAVI_LETTER_PE => Ok(PsalterPahlavi::PsalterPahlaviLetterPe),
            PSALTER_PAHLAVI_LETTER_SADHE => Ok(PsalterPahlavi::PsalterPahlaviLetterSadhe),
            PSALTER_PAHLAVI_LETTER_SHIN => Ok(PsalterPahlavi::PsalterPahlaviLetterShin),
            PSALTER_PAHLAVI_LETTER_TAW => Ok(PsalterPahlavi::PsalterPahlaviLetterTaw),
            PSALTER_PAHLAVI_SECTION_MARK => Ok(PsalterPahlavi::PsalterPahlaviSectionMark),
            PSALTER_PAHLAVI_TURNED_SECTION_MARK => Ok(PsalterPahlavi::PsalterPahlaviTurnedSectionMark),
            PSALTER_PAHLAVI_FOUR_DOTS_WITH_CROSS => Ok(PsalterPahlavi::PsalterPahlaviFourDotsWithCross),
            PSALTER_PAHLAVI_FOUR_DOTS_WITH_DOT => Ok(PsalterPahlavi::PsalterPahlaviFourDotsWithDot),
            PSALTER_PAHLAVI_NUMBER_ONE => Ok(PsalterPahlavi::PsalterPahlaviNumberOne),
            PSALTER_PAHLAVI_NUMBER_TWO => Ok(PsalterPahlavi::PsalterPahlaviNumberTwo),
            PSALTER_PAHLAVI_NUMBER_THREE => Ok(PsalterPahlavi::PsalterPahlaviNumberThree),
            PSALTER_PAHLAVI_NUMBER_FOUR => Ok(PsalterPahlavi::PsalterPahlaviNumberFour),
            PSALTER_PAHLAVI_NUMBER_TEN => Ok(PsalterPahlavi::PsalterPahlaviNumberTen),
            PSALTER_PAHLAVI_NUMBER_TWENTY => Ok(PsalterPahlavi::PsalterPahlaviNumberTwenty),
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
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        PsalterPahlavi::PsalterPahlaviLetterAleph
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            PsalterPahlavi::PsalterPahlaviLetterAleph => "psalter pahlavi letter aleph",
            PsalterPahlavi::PsalterPahlaviLetterBeth => "psalter pahlavi letter beth",
            PsalterPahlavi::PsalterPahlaviLetterGimel => "psalter pahlavi letter gimel",
            PsalterPahlavi::PsalterPahlaviLetterDaleth => "psalter pahlavi letter daleth",
            PsalterPahlavi::PsalterPahlaviLetterHe => "psalter pahlavi letter he",
            PsalterPahlavi::PsalterPahlaviLetterWawDashAyinDashResh => "psalter pahlavi letter waw-ayin-resh",
            PsalterPahlavi::PsalterPahlaviLetterZayin => "psalter pahlavi letter zayin",
            PsalterPahlavi::PsalterPahlaviLetterHeth => "psalter pahlavi letter heth",
            PsalterPahlavi::PsalterPahlaviLetterYodh => "psalter pahlavi letter yodh",
            PsalterPahlavi::PsalterPahlaviLetterKaph => "psalter pahlavi letter kaph",
            PsalterPahlavi::PsalterPahlaviLetterLamedh => "psalter pahlavi letter lamedh",
            PsalterPahlavi::PsalterPahlaviLetterMemDashQoph => "psalter pahlavi letter mem-qoph",
            PsalterPahlavi::PsalterPahlaviLetterNun => "psalter pahlavi letter nun",
            PsalterPahlavi::PsalterPahlaviLetterSamekh => "psalter pahlavi letter samekh",
            PsalterPahlavi::PsalterPahlaviLetterPe => "psalter pahlavi letter pe",
            PsalterPahlavi::PsalterPahlaviLetterSadhe => "psalter pahlavi letter sadhe",
            PsalterPahlavi::PsalterPahlaviLetterShin => "psalter pahlavi letter shin",
            PsalterPahlavi::PsalterPahlaviLetterTaw => "psalter pahlavi letter taw",
            PsalterPahlavi::PsalterPahlaviSectionMark => "psalter pahlavi section mark",
            PsalterPahlavi::PsalterPahlaviTurnedSectionMark => "psalter pahlavi turned section mark",
            PsalterPahlavi::PsalterPahlaviFourDotsWithCross => "psalter pahlavi four dots with cross",
            PsalterPahlavi::PsalterPahlaviFourDotsWithDot => "psalter pahlavi four dots with dot",
            PsalterPahlavi::PsalterPahlaviNumberOne => "psalter pahlavi number one",
            PsalterPahlavi::PsalterPahlaviNumberTwo => "psalter pahlavi number two",
            PsalterPahlavi::PsalterPahlaviNumberThree => "psalter pahlavi number three",
            PsalterPahlavi::PsalterPahlaviNumberFour => "psalter pahlavi number four",
            PsalterPahlavi::PsalterPahlaviNumberTen => "psalter pahlavi number ten",
            PsalterPahlavi::PsalterPahlaviNumberTwenty => "psalter pahlavi number twenty",
        }
    }
}
