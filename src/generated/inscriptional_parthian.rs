/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{10b40}: '𐭀'
    pub const LETTER_ALEPH: char = '𐭀';
    /// \u{10b41}: '𐭁'
    pub const LETTER_BETH: char = '𐭁';
    /// \u{10b42}: '𐭂'
    pub const LETTER_GIMEL: char = '𐭂';
    /// \u{10b43}: '𐭃'
    pub const LETTER_DALETH: char = '𐭃';
    /// \u{10b44}: '𐭄'
    pub const LETTER_HE: char = '𐭄';
    /// \u{10b45}: '𐭅'
    pub const LETTER_WAW: char = '𐭅';
    /// \u{10b46}: '𐭆'
    pub const LETTER_ZAYIN: char = '𐭆';
    /// \u{10b47}: '𐭇'
    pub const LETTER_HETH: char = '𐭇';
    /// \u{10b48}: '𐭈'
    pub const LETTER_TETH: char = '𐭈';
    /// \u{10b49}: '𐭉'
    pub const LETTER_YODH: char = '𐭉';
    /// \u{10b4a}: '𐭊'
    pub const LETTER_KAPH: char = '𐭊';
    /// \u{10b4b}: '𐭋'
    pub const LETTER_LAMEDH: char = '𐭋';
    /// \u{10b4c}: '𐭌'
    pub const LETTER_MEM: char = '𐭌';
    /// \u{10b4d}: '𐭍'
    pub const LETTER_NUN: char = '𐭍';
    /// \u{10b4e}: '𐭎'
    pub const LETTER_SAMEKH: char = '𐭎';
    /// \u{10b4f}: '𐭏'
    pub const LETTER_AYIN: char = '𐭏';
    /// \u{10b50}: '𐭐'
    pub const LETTER_PE: char = '𐭐';
    /// \u{10b51}: '𐭑'
    pub const LETTER_SADHE: char = '𐭑';
    /// \u{10b52}: '𐭒'
    pub const LETTER_QOPH: char = '𐭒';
    /// \u{10b53}: '𐭓'
    pub const LETTER_RESH: char = '𐭓';
    /// \u{10b54}: '𐭔'
    pub const LETTER_SHIN: char = '𐭔';
    /// \u{10b55}: '𐭕'
    pub const LETTER_TAW: char = '𐭕';
    /// \u{10b58}: '𐭘'
    pub const NUMBER_ONE: char = '𐭘';
    /// \u{10b59}: '𐭙'
    pub const NUMBER_TWO: char = '𐭙';
    /// \u{10b5a}: '𐭚'
    pub const NUMBER_THREE: char = '𐭚';
    /// \u{10b5b}: '𐭛'
    pub const NUMBER_FOUR: char = '𐭛';
    /// \u{10b5c}: '𐭜'
    pub const NUMBER_TEN: char = '𐭜';
    /// \u{10b5d}: '𐭝'
    pub const NUMBER_TWENTY: char = '𐭝';
    /// \u{10b5e}: '𐭞'
    pub const NUMBER_ONE_HUNDRED: char = '𐭞';
}

/// An enum to represent all characters in the InscriptionalParthian block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum InscriptionalParthian {
    /// \u{10b40}: '𐭀'
    LetterAleph,
    /// \u{10b41}: '𐭁'
    LetterBeth,
    /// \u{10b42}: '𐭂'
    LetterGimel,
    /// \u{10b43}: '𐭃'
    LetterDaleth,
    /// \u{10b44}: '𐭄'
    LetterHe,
    /// \u{10b45}: '𐭅'
    LetterWaw,
    /// \u{10b46}: '𐭆'
    LetterZayin,
    /// \u{10b47}: '𐭇'
    LetterHeth,
    /// \u{10b48}: '𐭈'
    LetterTeth,
    /// \u{10b49}: '𐭉'
    LetterYodh,
    /// \u{10b4a}: '𐭊'
    LetterKaph,
    /// \u{10b4b}: '𐭋'
    LetterLamedh,
    /// \u{10b4c}: '𐭌'
    LetterMem,
    /// \u{10b4d}: '𐭍'
    LetterNun,
    /// \u{10b4e}: '𐭎'
    LetterSamekh,
    /// \u{10b4f}: '𐭏'
    LetterAyin,
    /// \u{10b50}: '𐭐'
    LetterPe,
    /// \u{10b51}: '𐭑'
    LetterSadhe,
    /// \u{10b52}: '𐭒'
    LetterQoph,
    /// \u{10b53}: '𐭓'
    LetterResh,
    /// \u{10b54}: '𐭔'
    LetterShin,
    /// \u{10b55}: '𐭕'
    LetterTaw,
    /// \u{10b58}: '𐭘'
    NumberOne,
    /// \u{10b59}: '𐭙'
    NumberTwo,
    /// \u{10b5a}: '𐭚'
    NumberThree,
    /// \u{10b5b}: '𐭛'
    NumberFour,
    /// \u{10b5c}: '𐭜'
    NumberTen,
    /// \u{10b5d}: '𐭝'
    NumberTwenty,
    /// \u{10b5e}: '𐭞'
    NumberOneHundred,
}

impl Into<char> for InscriptionalParthian {
    fn into(self) -> char {
        use constants::*;
        match self {
            InscriptionalParthian::LetterAleph => LETTER_ALEPH,
            InscriptionalParthian::LetterBeth => LETTER_BETH,
            InscriptionalParthian::LetterGimel => LETTER_GIMEL,
            InscriptionalParthian::LetterDaleth => LETTER_DALETH,
            InscriptionalParthian::LetterHe => LETTER_HE,
            InscriptionalParthian::LetterWaw => LETTER_WAW,
            InscriptionalParthian::LetterZayin => LETTER_ZAYIN,
            InscriptionalParthian::LetterHeth => LETTER_HETH,
            InscriptionalParthian::LetterTeth => LETTER_TETH,
            InscriptionalParthian::LetterYodh => LETTER_YODH,
            InscriptionalParthian::LetterKaph => LETTER_KAPH,
            InscriptionalParthian::LetterLamedh => LETTER_LAMEDH,
            InscriptionalParthian::LetterMem => LETTER_MEM,
            InscriptionalParthian::LetterNun => LETTER_NUN,
            InscriptionalParthian::LetterSamekh => LETTER_SAMEKH,
            InscriptionalParthian::LetterAyin => LETTER_AYIN,
            InscriptionalParthian::LetterPe => LETTER_PE,
            InscriptionalParthian::LetterSadhe => LETTER_SADHE,
            InscriptionalParthian::LetterQoph => LETTER_QOPH,
            InscriptionalParthian::LetterResh => LETTER_RESH,
            InscriptionalParthian::LetterShin => LETTER_SHIN,
            InscriptionalParthian::LetterTaw => LETTER_TAW,
            InscriptionalParthian::NumberOne => NUMBER_ONE,
            InscriptionalParthian::NumberTwo => NUMBER_TWO,
            InscriptionalParthian::NumberThree => NUMBER_THREE,
            InscriptionalParthian::NumberFour => NUMBER_FOUR,
            InscriptionalParthian::NumberTen => NUMBER_TEN,
            InscriptionalParthian::NumberTwenty => NUMBER_TWENTY,
            InscriptionalParthian::NumberOneHundred => NUMBER_ONE_HUNDRED,
        }
    }
}

impl std::convert::TryFrom<char> for InscriptionalParthian {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_ALEPH => Ok(InscriptionalParthian::LetterAleph),
            LETTER_BETH => Ok(InscriptionalParthian::LetterBeth),
            LETTER_GIMEL => Ok(InscriptionalParthian::LetterGimel),
            LETTER_DALETH => Ok(InscriptionalParthian::LetterDaleth),
            LETTER_HE => Ok(InscriptionalParthian::LetterHe),
            LETTER_WAW => Ok(InscriptionalParthian::LetterWaw),
            LETTER_ZAYIN => Ok(InscriptionalParthian::LetterZayin),
            LETTER_HETH => Ok(InscriptionalParthian::LetterHeth),
            LETTER_TETH => Ok(InscriptionalParthian::LetterTeth),
            LETTER_YODH => Ok(InscriptionalParthian::LetterYodh),
            LETTER_KAPH => Ok(InscriptionalParthian::LetterKaph),
            LETTER_LAMEDH => Ok(InscriptionalParthian::LetterLamedh),
            LETTER_MEM => Ok(InscriptionalParthian::LetterMem),
            LETTER_NUN => Ok(InscriptionalParthian::LetterNun),
            LETTER_SAMEKH => Ok(InscriptionalParthian::LetterSamekh),
            LETTER_AYIN => Ok(InscriptionalParthian::LetterAyin),
            LETTER_PE => Ok(InscriptionalParthian::LetterPe),
            LETTER_SADHE => Ok(InscriptionalParthian::LetterSadhe),
            LETTER_QOPH => Ok(InscriptionalParthian::LetterQoph),
            LETTER_RESH => Ok(InscriptionalParthian::LetterResh),
            LETTER_SHIN => Ok(InscriptionalParthian::LetterShin),
            LETTER_TAW => Ok(InscriptionalParthian::LetterTaw),
            NUMBER_ONE => Ok(InscriptionalParthian::NumberOne),
            NUMBER_TWO => Ok(InscriptionalParthian::NumberTwo),
            NUMBER_THREE => Ok(InscriptionalParthian::NumberThree),
            NUMBER_FOUR => Ok(InscriptionalParthian::NumberFour),
            NUMBER_TEN => Ok(InscriptionalParthian::NumberTen),
            NUMBER_TWENTY => Ok(InscriptionalParthian::NumberTwenty),
            NUMBER_ONE_HUNDRED => Ok(InscriptionalParthian::NumberOneHundred),
            _ => Err(()),
        }
    }
}

impl Into<u32> for InscriptionalParthian {
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

impl std::convert::TryFrom<u32> for InscriptionalParthian {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for InscriptionalParthian {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl InscriptionalParthian {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        InscriptionalParthian::LetterAleph
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            InscriptionalParthian::LetterAleph => "inscriptional parthian letter aleph",
            InscriptionalParthian::LetterBeth => "inscriptional parthian letter beth",
            InscriptionalParthian::LetterGimel => "inscriptional parthian letter gimel",
            InscriptionalParthian::LetterDaleth => "inscriptional parthian letter daleth",
            InscriptionalParthian::LetterHe => "inscriptional parthian letter he",
            InscriptionalParthian::LetterWaw => "inscriptional parthian letter waw",
            InscriptionalParthian::LetterZayin => "inscriptional parthian letter zayin",
            InscriptionalParthian::LetterHeth => "inscriptional parthian letter heth",
            InscriptionalParthian::LetterTeth => "inscriptional parthian letter teth",
            InscriptionalParthian::LetterYodh => "inscriptional parthian letter yodh",
            InscriptionalParthian::LetterKaph => "inscriptional parthian letter kaph",
            InscriptionalParthian::LetterLamedh => "inscriptional parthian letter lamedh",
            InscriptionalParthian::LetterMem => "inscriptional parthian letter mem",
            InscriptionalParthian::LetterNun => "inscriptional parthian letter nun",
            InscriptionalParthian::LetterSamekh => "inscriptional parthian letter samekh",
            InscriptionalParthian::LetterAyin => "inscriptional parthian letter ayin",
            InscriptionalParthian::LetterPe => "inscriptional parthian letter pe",
            InscriptionalParthian::LetterSadhe => "inscriptional parthian letter sadhe",
            InscriptionalParthian::LetterQoph => "inscriptional parthian letter qoph",
            InscriptionalParthian::LetterResh => "inscriptional parthian letter resh",
            InscriptionalParthian::LetterShin => "inscriptional parthian letter shin",
            InscriptionalParthian::LetterTaw => "inscriptional parthian letter taw",
            InscriptionalParthian::NumberOne => "inscriptional parthian number one",
            InscriptionalParthian::NumberTwo => "inscriptional parthian number two",
            InscriptionalParthian::NumberThree => "inscriptional parthian number three",
            InscriptionalParthian::NumberFour => "inscriptional parthian number four",
            InscriptionalParthian::NumberTen => "inscriptional parthian number ten",
            InscriptionalParthian::NumberTwenty => "inscriptional parthian number twenty",
            InscriptionalParthian::NumberOneHundred => "inscriptional parthian number one hundred",
        }
    }
}
