/// \u{10b40} → \u{10b5f}\
///\
/// 𐭀 𐭁 𐭂 𐭃 𐭄 𐭅 𐭆 𐭇 𐭈 𐭉 𐭊 𐭋 𐭌 𐭍 𐭎 𐭏
/// 𐭐 𐭑 𐭒 𐭓 𐭔 𐭕 𐭘 𐭙 𐭚 𐭛 𐭜 𐭝 𐭞
pub mod constants {
    /// \u{10b40}: '𐭀'
    pub const INSCRIPTIONAL_PARTHIAN_LETTER_ALEPH: char = '𐭀';
    /// \u{10b41}: '𐭁'
    pub const INSCRIPTIONAL_PARTHIAN_LETTER_BETH: char = '𐭁';
    /// \u{10b42}: '𐭂'
    pub const INSCRIPTIONAL_PARTHIAN_LETTER_GIMEL: char = '𐭂';
    /// \u{10b43}: '𐭃'
    pub const INSCRIPTIONAL_PARTHIAN_LETTER_DALETH: char = '𐭃';
    /// \u{10b44}: '𐭄'
    pub const INSCRIPTIONAL_PARTHIAN_LETTER_HE: char = '𐭄';
    /// \u{10b45}: '𐭅'
    pub const INSCRIPTIONAL_PARTHIAN_LETTER_WAW: char = '𐭅';
    /// \u{10b46}: '𐭆'
    pub const INSCRIPTIONAL_PARTHIAN_LETTER_ZAYIN: char = '𐭆';
    /// \u{10b47}: '𐭇'
    pub const INSCRIPTIONAL_PARTHIAN_LETTER_HETH: char = '𐭇';
    /// \u{10b48}: '𐭈'
    pub const INSCRIPTIONAL_PARTHIAN_LETTER_TETH: char = '𐭈';
    /// \u{10b49}: '𐭉'
    pub const INSCRIPTIONAL_PARTHIAN_LETTER_YODH: char = '𐭉';
    /// \u{10b4a}: '𐭊'
    pub const INSCRIPTIONAL_PARTHIAN_LETTER_KAPH: char = '𐭊';
    /// \u{10b4b}: '𐭋'
    pub const INSCRIPTIONAL_PARTHIAN_LETTER_LAMEDH: char = '𐭋';
    /// \u{10b4c}: '𐭌'
    pub const INSCRIPTIONAL_PARTHIAN_LETTER_MEM: char = '𐭌';
    /// \u{10b4d}: '𐭍'
    pub const INSCRIPTIONAL_PARTHIAN_LETTER_NUN: char = '𐭍';
    /// \u{10b4e}: '𐭎'
    pub const INSCRIPTIONAL_PARTHIAN_LETTER_SAMEKH: char = '𐭎';
    /// \u{10b4f}: '𐭏'
    pub const INSCRIPTIONAL_PARTHIAN_LETTER_AYIN: char = '𐭏';
    /// \u{10b50}: '𐭐'
    pub const INSCRIPTIONAL_PARTHIAN_LETTER_PE: char = '𐭐';
    /// \u{10b51}: '𐭑'
    pub const INSCRIPTIONAL_PARTHIAN_LETTER_SADHE: char = '𐭑';
    /// \u{10b52}: '𐭒'
    pub const INSCRIPTIONAL_PARTHIAN_LETTER_QOPH: char = '𐭒';
    /// \u{10b53}: '𐭓'
    pub const INSCRIPTIONAL_PARTHIAN_LETTER_RESH: char = '𐭓';
    /// \u{10b54}: '𐭔'
    pub const INSCRIPTIONAL_PARTHIAN_LETTER_SHIN: char = '𐭔';
    /// \u{10b55}: '𐭕'
    pub const INSCRIPTIONAL_PARTHIAN_LETTER_TAW: char = '𐭕';
    /// \u{10b58}: '𐭘'
    pub const INSCRIPTIONAL_PARTHIAN_NUMBER_ONE: char = '𐭘';
    /// \u{10b59}: '𐭙'
    pub const INSCRIPTIONAL_PARTHIAN_NUMBER_TWO: char = '𐭙';
    /// \u{10b5a}: '𐭚'
    pub const INSCRIPTIONAL_PARTHIAN_NUMBER_THREE: char = '𐭚';
    /// \u{10b5b}: '𐭛'
    pub const INSCRIPTIONAL_PARTHIAN_NUMBER_FOUR: char = '𐭛';
    /// \u{10b5c}: '𐭜'
    pub const INSCRIPTIONAL_PARTHIAN_NUMBER_TEN: char = '𐭜';
    /// \u{10b5d}: '𐭝'
    pub const INSCRIPTIONAL_PARTHIAN_NUMBER_TWENTY: char = '𐭝';
    /// \u{10b5e}: '𐭞'
    pub const INSCRIPTIONAL_PARTHIAN_NUMBER_ONE_HUNDRED: char = '𐭞';
}

/// \u{10b40} → \u{10b5f}\
///\
/// 𐭀 𐭁 𐭂 𐭃 𐭄 𐭅 𐭆 𐭇 𐭈 𐭉 𐭊 𐭋 𐭌 𐭍 𐭎 𐭏
/// 𐭐 𐭑 𐭒 𐭓 𐭔 𐭕 𐭘 𐭙 𐭚 𐭛 𐭜 𐭝 𐭞
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum InscriptionalParthian {
    /// \u{10b40}: '𐭀'
    InscriptionalParthianLetterAleph,
    /// \u{10b41}: '𐭁'
    InscriptionalParthianLetterBeth,
    /// \u{10b42}: '𐭂'
    InscriptionalParthianLetterGimel,
    /// \u{10b43}: '𐭃'
    InscriptionalParthianLetterDaleth,
    /// \u{10b44}: '𐭄'
    InscriptionalParthianLetterHe,
    /// \u{10b45}: '𐭅'
    InscriptionalParthianLetterWaw,
    /// \u{10b46}: '𐭆'
    InscriptionalParthianLetterZayin,
    /// \u{10b47}: '𐭇'
    InscriptionalParthianLetterHeth,
    /// \u{10b48}: '𐭈'
    InscriptionalParthianLetterTeth,
    /// \u{10b49}: '𐭉'
    InscriptionalParthianLetterYodh,
    /// \u{10b4a}: '𐭊'
    InscriptionalParthianLetterKaph,
    /// \u{10b4b}: '𐭋'
    InscriptionalParthianLetterLamedh,
    /// \u{10b4c}: '𐭌'
    InscriptionalParthianLetterMem,
    /// \u{10b4d}: '𐭍'
    InscriptionalParthianLetterNun,
    /// \u{10b4e}: '𐭎'
    InscriptionalParthianLetterSamekh,
    /// \u{10b4f}: '𐭏'
    InscriptionalParthianLetterAyin,
    /// \u{10b50}: '𐭐'
    InscriptionalParthianLetterPe,
    /// \u{10b51}: '𐭑'
    InscriptionalParthianLetterSadhe,
    /// \u{10b52}: '𐭒'
    InscriptionalParthianLetterQoph,
    /// \u{10b53}: '𐭓'
    InscriptionalParthianLetterResh,
    /// \u{10b54}: '𐭔'
    InscriptionalParthianLetterShin,
    /// \u{10b55}: '𐭕'
    InscriptionalParthianLetterTaw,
    /// \u{10b58}: '𐭘'
    InscriptionalParthianNumberOne,
    /// \u{10b59}: '𐭙'
    InscriptionalParthianNumberTwo,
    /// \u{10b5a}: '𐭚'
    InscriptionalParthianNumberThree,
    /// \u{10b5b}: '𐭛'
    InscriptionalParthianNumberFour,
    /// \u{10b5c}: '𐭜'
    InscriptionalParthianNumberTen,
    /// \u{10b5d}: '𐭝'
    InscriptionalParthianNumberTwenty,
    /// \u{10b5e}: '𐭞'
    InscriptionalParthianNumberOneHundred,
}

impl Into<char> for InscriptionalParthian {
    fn into(self) -> char {
        use constants::*;
        match self {
            InscriptionalParthian::InscriptionalParthianLetterAleph => INSCRIPTIONAL_PARTHIAN_LETTER_ALEPH,
            InscriptionalParthian::InscriptionalParthianLetterBeth => INSCRIPTIONAL_PARTHIAN_LETTER_BETH,
            InscriptionalParthian::InscriptionalParthianLetterGimel => INSCRIPTIONAL_PARTHIAN_LETTER_GIMEL,
            InscriptionalParthian::InscriptionalParthianLetterDaleth => INSCRIPTIONAL_PARTHIAN_LETTER_DALETH,
            InscriptionalParthian::InscriptionalParthianLetterHe => INSCRIPTIONAL_PARTHIAN_LETTER_HE,
            InscriptionalParthian::InscriptionalParthianLetterWaw => INSCRIPTIONAL_PARTHIAN_LETTER_WAW,
            InscriptionalParthian::InscriptionalParthianLetterZayin => INSCRIPTIONAL_PARTHIAN_LETTER_ZAYIN,
            InscriptionalParthian::InscriptionalParthianLetterHeth => INSCRIPTIONAL_PARTHIAN_LETTER_HETH,
            InscriptionalParthian::InscriptionalParthianLetterTeth => INSCRIPTIONAL_PARTHIAN_LETTER_TETH,
            InscriptionalParthian::InscriptionalParthianLetterYodh => INSCRIPTIONAL_PARTHIAN_LETTER_YODH,
            InscriptionalParthian::InscriptionalParthianLetterKaph => INSCRIPTIONAL_PARTHIAN_LETTER_KAPH,
            InscriptionalParthian::InscriptionalParthianLetterLamedh => INSCRIPTIONAL_PARTHIAN_LETTER_LAMEDH,
            InscriptionalParthian::InscriptionalParthianLetterMem => INSCRIPTIONAL_PARTHIAN_LETTER_MEM,
            InscriptionalParthian::InscriptionalParthianLetterNun => INSCRIPTIONAL_PARTHIAN_LETTER_NUN,
            InscriptionalParthian::InscriptionalParthianLetterSamekh => INSCRIPTIONAL_PARTHIAN_LETTER_SAMEKH,
            InscriptionalParthian::InscriptionalParthianLetterAyin => INSCRIPTIONAL_PARTHIAN_LETTER_AYIN,
            InscriptionalParthian::InscriptionalParthianLetterPe => INSCRIPTIONAL_PARTHIAN_LETTER_PE,
            InscriptionalParthian::InscriptionalParthianLetterSadhe => INSCRIPTIONAL_PARTHIAN_LETTER_SADHE,
            InscriptionalParthian::InscriptionalParthianLetterQoph => INSCRIPTIONAL_PARTHIAN_LETTER_QOPH,
            InscriptionalParthian::InscriptionalParthianLetterResh => INSCRIPTIONAL_PARTHIAN_LETTER_RESH,
            InscriptionalParthian::InscriptionalParthianLetterShin => INSCRIPTIONAL_PARTHIAN_LETTER_SHIN,
            InscriptionalParthian::InscriptionalParthianLetterTaw => INSCRIPTIONAL_PARTHIAN_LETTER_TAW,
            InscriptionalParthian::InscriptionalParthianNumberOne => INSCRIPTIONAL_PARTHIAN_NUMBER_ONE,
            InscriptionalParthian::InscriptionalParthianNumberTwo => INSCRIPTIONAL_PARTHIAN_NUMBER_TWO,
            InscriptionalParthian::InscriptionalParthianNumberThree => INSCRIPTIONAL_PARTHIAN_NUMBER_THREE,
            InscriptionalParthian::InscriptionalParthianNumberFour => INSCRIPTIONAL_PARTHIAN_NUMBER_FOUR,
            InscriptionalParthian::InscriptionalParthianNumberTen => INSCRIPTIONAL_PARTHIAN_NUMBER_TEN,
            InscriptionalParthian::InscriptionalParthianNumberTwenty => INSCRIPTIONAL_PARTHIAN_NUMBER_TWENTY,
            InscriptionalParthian::InscriptionalParthianNumberOneHundred => INSCRIPTIONAL_PARTHIAN_NUMBER_ONE_HUNDRED,
        }
    }
}

impl std::convert::TryFrom<char> for InscriptionalParthian {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            INSCRIPTIONAL_PARTHIAN_LETTER_ALEPH => Ok(InscriptionalParthian::InscriptionalParthianLetterAleph),
            INSCRIPTIONAL_PARTHIAN_LETTER_BETH => Ok(InscriptionalParthian::InscriptionalParthianLetterBeth),
            INSCRIPTIONAL_PARTHIAN_LETTER_GIMEL => Ok(InscriptionalParthian::InscriptionalParthianLetterGimel),
            INSCRIPTIONAL_PARTHIAN_LETTER_DALETH => Ok(InscriptionalParthian::InscriptionalParthianLetterDaleth),
            INSCRIPTIONAL_PARTHIAN_LETTER_HE => Ok(InscriptionalParthian::InscriptionalParthianLetterHe),
            INSCRIPTIONAL_PARTHIAN_LETTER_WAW => Ok(InscriptionalParthian::InscriptionalParthianLetterWaw),
            INSCRIPTIONAL_PARTHIAN_LETTER_ZAYIN => Ok(InscriptionalParthian::InscriptionalParthianLetterZayin),
            INSCRIPTIONAL_PARTHIAN_LETTER_HETH => Ok(InscriptionalParthian::InscriptionalParthianLetterHeth),
            INSCRIPTIONAL_PARTHIAN_LETTER_TETH => Ok(InscriptionalParthian::InscriptionalParthianLetterTeth),
            INSCRIPTIONAL_PARTHIAN_LETTER_YODH => Ok(InscriptionalParthian::InscriptionalParthianLetterYodh),
            INSCRIPTIONAL_PARTHIAN_LETTER_KAPH => Ok(InscriptionalParthian::InscriptionalParthianLetterKaph),
            INSCRIPTIONAL_PARTHIAN_LETTER_LAMEDH => Ok(InscriptionalParthian::InscriptionalParthianLetterLamedh),
            INSCRIPTIONAL_PARTHIAN_LETTER_MEM => Ok(InscriptionalParthian::InscriptionalParthianLetterMem),
            INSCRIPTIONAL_PARTHIAN_LETTER_NUN => Ok(InscriptionalParthian::InscriptionalParthianLetterNun),
            INSCRIPTIONAL_PARTHIAN_LETTER_SAMEKH => Ok(InscriptionalParthian::InscriptionalParthianLetterSamekh),
            INSCRIPTIONAL_PARTHIAN_LETTER_AYIN => Ok(InscriptionalParthian::InscriptionalParthianLetterAyin),
            INSCRIPTIONAL_PARTHIAN_LETTER_PE => Ok(InscriptionalParthian::InscriptionalParthianLetterPe),
            INSCRIPTIONAL_PARTHIAN_LETTER_SADHE => Ok(InscriptionalParthian::InscriptionalParthianLetterSadhe),
            INSCRIPTIONAL_PARTHIAN_LETTER_QOPH => Ok(InscriptionalParthian::InscriptionalParthianLetterQoph),
            INSCRIPTIONAL_PARTHIAN_LETTER_RESH => Ok(InscriptionalParthian::InscriptionalParthianLetterResh),
            INSCRIPTIONAL_PARTHIAN_LETTER_SHIN => Ok(InscriptionalParthian::InscriptionalParthianLetterShin),
            INSCRIPTIONAL_PARTHIAN_LETTER_TAW => Ok(InscriptionalParthian::InscriptionalParthianLetterTaw),
            INSCRIPTIONAL_PARTHIAN_NUMBER_ONE => Ok(InscriptionalParthian::InscriptionalParthianNumberOne),
            INSCRIPTIONAL_PARTHIAN_NUMBER_TWO => Ok(InscriptionalParthian::InscriptionalParthianNumberTwo),
            INSCRIPTIONAL_PARTHIAN_NUMBER_THREE => Ok(InscriptionalParthian::InscriptionalParthianNumberThree),
            INSCRIPTIONAL_PARTHIAN_NUMBER_FOUR => Ok(InscriptionalParthian::InscriptionalParthianNumberFour),
            INSCRIPTIONAL_PARTHIAN_NUMBER_TEN => Ok(InscriptionalParthian::InscriptionalParthianNumberTen),
            INSCRIPTIONAL_PARTHIAN_NUMBER_TWENTY => Ok(InscriptionalParthian::InscriptionalParthianNumberTwenty),
            INSCRIPTIONAL_PARTHIAN_NUMBER_ONE_HUNDRED => Ok(InscriptionalParthian::InscriptionalParthianNumberOneHundred),
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
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        InscriptionalParthian::InscriptionalParthianLetterAleph
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            InscriptionalParthian::InscriptionalParthianLetterAleph => "inscriptional parthian letter aleph",
            InscriptionalParthian::InscriptionalParthianLetterBeth => "inscriptional parthian letter beth",
            InscriptionalParthian::InscriptionalParthianLetterGimel => "inscriptional parthian letter gimel",
            InscriptionalParthian::InscriptionalParthianLetterDaleth => "inscriptional parthian letter daleth",
            InscriptionalParthian::InscriptionalParthianLetterHe => "inscriptional parthian letter he",
            InscriptionalParthian::InscriptionalParthianLetterWaw => "inscriptional parthian letter waw",
            InscriptionalParthian::InscriptionalParthianLetterZayin => "inscriptional parthian letter zayin",
            InscriptionalParthian::InscriptionalParthianLetterHeth => "inscriptional parthian letter heth",
            InscriptionalParthian::InscriptionalParthianLetterTeth => "inscriptional parthian letter teth",
            InscriptionalParthian::InscriptionalParthianLetterYodh => "inscriptional parthian letter yodh",
            InscriptionalParthian::InscriptionalParthianLetterKaph => "inscriptional parthian letter kaph",
            InscriptionalParthian::InscriptionalParthianLetterLamedh => "inscriptional parthian letter lamedh",
            InscriptionalParthian::InscriptionalParthianLetterMem => "inscriptional parthian letter mem",
            InscriptionalParthian::InscriptionalParthianLetterNun => "inscriptional parthian letter nun",
            InscriptionalParthian::InscriptionalParthianLetterSamekh => "inscriptional parthian letter samekh",
            InscriptionalParthian::InscriptionalParthianLetterAyin => "inscriptional parthian letter ayin",
            InscriptionalParthian::InscriptionalParthianLetterPe => "inscriptional parthian letter pe",
            InscriptionalParthian::InscriptionalParthianLetterSadhe => "inscriptional parthian letter sadhe",
            InscriptionalParthian::InscriptionalParthianLetterQoph => "inscriptional parthian letter qoph",
            InscriptionalParthian::InscriptionalParthianLetterResh => "inscriptional parthian letter resh",
            InscriptionalParthian::InscriptionalParthianLetterShin => "inscriptional parthian letter shin",
            InscriptionalParthian::InscriptionalParthianLetterTaw => "inscriptional parthian letter taw",
            InscriptionalParthian::InscriptionalParthianNumberOne => "inscriptional parthian number one",
            InscriptionalParthian::InscriptionalParthianNumberTwo => "inscriptional parthian number two",
            InscriptionalParthian::InscriptionalParthianNumberThree => "inscriptional parthian number three",
            InscriptionalParthian::InscriptionalParthianNumberFour => "inscriptional parthian number four",
            InscriptionalParthian::InscriptionalParthianNumberTen => "inscriptional parthian number ten",
            InscriptionalParthian::InscriptionalParthianNumberTwenty => "inscriptional parthian number twenty",
            InscriptionalParthian::InscriptionalParthianNumberOneHundred => "inscriptional parthian number one hundred",
        }
    }
}
