/// \u{10880} → \u{108af}
///
/// 𐢀 𐢁 𐢂 𐢃 𐢄 𐢅 𐢆 𐢇 𐢈 𐢉 𐢊 𐢋 𐢌 𐢍 𐢎 𐢏\
/// 𐢐 𐢑 𐢒 𐢓 𐢔 𐢕 𐢖 𐢗 𐢘 𐢙 𐢚 𐢛 𐢜 𐢝 𐢞 𐢧\
/// 𐢨 𐢩 𐢪 𐢫 𐢬 𐢭 𐢮\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{10880}: '𐢀'
    pub const LETTER_FINAL_ALEPH: char = '𐢀';
    /// \u{10881}: '𐢁'
    pub const LETTER_ALEPH: char = '𐢁';
    /// \u{10882}: '𐢂'
    pub const LETTER_FINAL_BETH: char = '𐢂';
    /// \u{10883}: '𐢃'
    pub const LETTER_BETH: char = '𐢃';
    /// \u{10884}: '𐢄'
    pub const LETTER_GIMEL: char = '𐢄';
    /// \u{10885}: '𐢅'
    pub const LETTER_DALETH: char = '𐢅';
    /// \u{10886}: '𐢆'
    pub const LETTER_FINAL_HE: char = '𐢆';
    /// \u{10887}: '𐢇'
    pub const LETTER_HE: char = '𐢇';
    /// \u{10888}: '𐢈'
    pub const LETTER_WAW: char = '𐢈';
    /// \u{10889}: '𐢉'
    pub const LETTER_ZAYIN: char = '𐢉';
    /// \u{1088a}: '𐢊'
    pub const LETTER_HETH: char = '𐢊';
    /// \u{1088b}: '𐢋'
    pub const LETTER_TETH: char = '𐢋';
    /// \u{1088c}: '𐢌'
    pub const LETTER_FINAL_YODH: char = '𐢌';
    /// \u{1088d}: '𐢍'
    pub const LETTER_YODH: char = '𐢍';
    /// \u{1088e}: '𐢎'
    pub const LETTER_FINAL_KAPH: char = '𐢎';
    /// \u{1088f}: '𐢏'
    pub const LETTER_KAPH: char = '𐢏';
    /// \u{10890}: '𐢐'
    pub const LETTER_FINAL_LAMEDH: char = '𐢐';
    /// \u{10891}: '𐢑'
    pub const LETTER_LAMEDH: char = '𐢑';
    /// \u{10892}: '𐢒'
    pub const LETTER_FINAL_MEM: char = '𐢒';
    /// \u{10893}: '𐢓'
    pub const LETTER_MEM: char = '𐢓';
    /// \u{10894}: '𐢔'
    pub const LETTER_FINAL_NUN: char = '𐢔';
    /// \u{10895}: '𐢕'
    pub const LETTER_NUN: char = '𐢕';
    /// \u{10896}: '𐢖'
    pub const LETTER_SAMEKH: char = '𐢖';
    /// \u{10897}: '𐢗'
    pub const LETTER_AYIN: char = '𐢗';
    /// \u{10898}: '𐢘'
    pub const LETTER_PE: char = '𐢘';
    /// \u{10899}: '𐢙'
    pub const LETTER_SADHE: char = '𐢙';
    /// \u{1089a}: '𐢚'
    pub const LETTER_QOPH: char = '𐢚';
    /// \u{1089b}: '𐢛'
    pub const LETTER_RESH: char = '𐢛';
    /// \u{1089c}: '𐢜'
    pub const LETTER_FINAL_SHIN: char = '𐢜';
    /// \u{1089d}: '𐢝'
    pub const LETTER_SHIN: char = '𐢝';
    /// \u{1089e}: '𐢞'
    pub const LETTER_TAW: char = '𐢞';
    /// \u{108a7}: '𐢧'
    pub const NUMBER_ONE: char = '𐢧';
    /// \u{108a8}: '𐢨'
    pub const NUMBER_TWO: char = '𐢨';
    /// \u{108a9}: '𐢩'
    pub const NUMBER_THREE: char = '𐢩';
    /// \u{108aa}: '𐢪'
    pub const NUMBER_FOUR: char = '𐢪';
    /// \u{108ab}: '𐢫'
    pub const CRUCIFORM_NUMBER_FOUR: char = '𐢫';
    /// \u{108ac}: '𐢬'
    pub const NUMBER_FIVE: char = '𐢬';
    /// \u{108ad}: '𐢭'
    pub const NUMBER_TEN: char = '𐢭';
    /// \u{108ae}: '𐢮'
    pub const NUMBER_TWENTY: char = '𐢮';
}

/// An enum to represent all characters in the Nabataean block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Nabataean {
    /// \u{10880}: '𐢀'
    LetterFinalAleph,
    /// \u{10881}: '𐢁'
    LetterAleph,
    /// \u{10882}: '𐢂'
    LetterFinalBeth,
    /// \u{10883}: '𐢃'
    LetterBeth,
    /// \u{10884}: '𐢄'
    LetterGimel,
    /// \u{10885}: '𐢅'
    LetterDaleth,
    /// \u{10886}: '𐢆'
    LetterFinalHe,
    /// \u{10887}: '𐢇'
    LetterHe,
    /// \u{10888}: '𐢈'
    LetterWaw,
    /// \u{10889}: '𐢉'
    LetterZayin,
    /// \u{1088a}: '𐢊'
    LetterHeth,
    /// \u{1088b}: '𐢋'
    LetterTeth,
    /// \u{1088c}: '𐢌'
    LetterFinalYodh,
    /// \u{1088d}: '𐢍'
    LetterYodh,
    /// \u{1088e}: '𐢎'
    LetterFinalKaph,
    /// \u{1088f}: '𐢏'
    LetterKaph,
    /// \u{10890}: '𐢐'
    LetterFinalLamedh,
    /// \u{10891}: '𐢑'
    LetterLamedh,
    /// \u{10892}: '𐢒'
    LetterFinalMem,
    /// \u{10893}: '𐢓'
    LetterMem,
    /// \u{10894}: '𐢔'
    LetterFinalNun,
    /// \u{10895}: '𐢕'
    LetterNun,
    /// \u{10896}: '𐢖'
    LetterSamekh,
    /// \u{10897}: '𐢗'
    LetterAyin,
    /// \u{10898}: '𐢘'
    LetterPe,
    /// \u{10899}: '𐢙'
    LetterSadhe,
    /// \u{1089a}: '𐢚'
    LetterQoph,
    /// \u{1089b}: '𐢛'
    LetterResh,
    /// \u{1089c}: '𐢜'
    LetterFinalShin,
    /// \u{1089d}: '𐢝'
    LetterShin,
    /// \u{1089e}: '𐢞'
    LetterTaw,
    /// \u{108a7}: '𐢧'
    NumberOne,
    /// \u{108a8}: '𐢨'
    NumberTwo,
    /// \u{108a9}: '𐢩'
    NumberThree,
    /// \u{108aa}: '𐢪'
    NumberFour,
    /// \u{108ab}: '𐢫'
    CruciformNumberFour,
    /// \u{108ac}: '𐢬'
    NumberFive,
    /// \u{108ad}: '𐢭'
    NumberTen,
    /// \u{108ae}: '𐢮'
    NumberTwenty,
}

impl Into<char> for Nabataean {
    fn into(self) -> char {
        use constants::*;
        match self {
            Nabataean::LetterFinalAleph => LETTER_FINAL_ALEPH,
            Nabataean::LetterAleph => LETTER_ALEPH,
            Nabataean::LetterFinalBeth => LETTER_FINAL_BETH,
            Nabataean::LetterBeth => LETTER_BETH,
            Nabataean::LetterGimel => LETTER_GIMEL,
            Nabataean::LetterDaleth => LETTER_DALETH,
            Nabataean::LetterFinalHe => LETTER_FINAL_HE,
            Nabataean::LetterHe => LETTER_HE,
            Nabataean::LetterWaw => LETTER_WAW,
            Nabataean::LetterZayin => LETTER_ZAYIN,
            Nabataean::LetterHeth => LETTER_HETH,
            Nabataean::LetterTeth => LETTER_TETH,
            Nabataean::LetterFinalYodh => LETTER_FINAL_YODH,
            Nabataean::LetterYodh => LETTER_YODH,
            Nabataean::LetterFinalKaph => LETTER_FINAL_KAPH,
            Nabataean::LetterKaph => LETTER_KAPH,
            Nabataean::LetterFinalLamedh => LETTER_FINAL_LAMEDH,
            Nabataean::LetterLamedh => LETTER_LAMEDH,
            Nabataean::LetterFinalMem => LETTER_FINAL_MEM,
            Nabataean::LetterMem => LETTER_MEM,
            Nabataean::LetterFinalNun => LETTER_FINAL_NUN,
            Nabataean::LetterNun => LETTER_NUN,
            Nabataean::LetterSamekh => LETTER_SAMEKH,
            Nabataean::LetterAyin => LETTER_AYIN,
            Nabataean::LetterPe => LETTER_PE,
            Nabataean::LetterSadhe => LETTER_SADHE,
            Nabataean::LetterQoph => LETTER_QOPH,
            Nabataean::LetterResh => LETTER_RESH,
            Nabataean::LetterFinalShin => LETTER_FINAL_SHIN,
            Nabataean::LetterShin => LETTER_SHIN,
            Nabataean::LetterTaw => LETTER_TAW,
            Nabataean::NumberOne => NUMBER_ONE,
            Nabataean::NumberTwo => NUMBER_TWO,
            Nabataean::NumberThree => NUMBER_THREE,
            Nabataean::NumberFour => NUMBER_FOUR,
            Nabataean::CruciformNumberFour => CRUCIFORM_NUMBER_FOUR,
            Nabataean::NumberFive => NUMBER_FIVE,
            Nabataean::NumberTen => NUMBER_TEN,
            Nabataean::NumberTwenty => NUMBER_TWENTY,
        }
    }
}

impl std::convert::TryFrom<char> for Nabataean {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_FINAL_ALEPH => Ok(Nabataean::LetterFinalAleph),
            LETTER_ALEPH => Ok(Nabataean::LetterAleph),
            LETTER_FINAL_BETH => Ok(Nabataean::LetterFinalBeth),
            LETTER_BETH => Ok(Nabataean::LetterBeth),
            LETTER_GIMEL => Ok(Nabataean::LetterGimel),
            LETTER_DALETH => Ok(Nabataean::LetterDaleth),
            LETTER_FINAL_HE => Ok(Nabataean::LetterFinalHe),
            LETTER_HE => Ok(Nabataean::LetterHe),
            LETTER_WAW => Ok(Nabataean::LetterWaw),
            LETTER_ZAYIN => Ok(Nabataean::LetterZayin),
            LETTER_HETH => Ok(Nabataean::LetterHeth),
            LETTER_TETH => Ok(Nabataean::LetterTeth),
            LETTER_FINAL_YODH => Ok(Nabataean::LetterFinalYodh),
            LETTER_YODH => Ok(Nabataean::LetterYodh),
            LETTER_FINAL_KAPH => Ok(Nabataean::LetterFinalKaph),
            LETTER_KAPH => Ok(Nabataean::LetterKaph),
            LETTER_FINAL_LAMEDH => Ok(Nabataean::LetterFinalLamedh),
            LETTER_LAMEDH => Ok(Nabataean::LetterLamedh),
            LETTER_FINAL_MEM => Ok(Nabataean::LetterFinalMem),
            LETTER_MEM => Ok(Nabataean::LetterMem),
            LETTER_FINAL_NUN => Ok(Nabataean::LetterFinalNun),
            LETTER_NUN => Ok(Nabataean::LetterNun),
            LETTER_SAMEKH => Ok(Nabataean::LetterSamekh),
            LETTER_AYIN => Ok(Nabataean::LetterAyin),
            LETTER_PE => Ok(Nabataean::LetterPe),
            LETTER_SADHE => Ok(Nabataean::LetterSadhe),
            LETTER_QOPH => Ok(Nabataean::LetterQoph),
            LETTER_RESH => Ok(Nabataean::LetterResh),
            LETTER_FINAL_SHIN => Ok(Nabataean::LetterFinalShin),
            LETTER_SHIN => Ok(Nabataean::LetterShin),
            LETTER_TAW => Ok(Nabataean::LetterTaw),
            NUMBER_ONE => Ok(Nabataean::NumberOne),
            NUMBER_TWO => Ok(Nabataean::NumberTwo),
            NUMBER_THREE => Ok(Nabataean::NumberThree),
            NUMBER_FOUR => Ok(Nabataean::NumberFour),
            CRUCIFORM_NUMBER_FOUR => Ok(Nabataean::CruciformNumberFour),
            NUMBER_FIVE => Ok(Nabataean::NumberFive),
            NUMBER_TEN => Ok(Nabataean::NumberTen),
            NUMBER_TWENTY => Ok(Nabataean::NumberTwenty),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Nabataean {
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

impl std::convert::TryFrom<u32> for Nabataean {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Nabataean {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Nabataean {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Nabataean::LetterFinalAleph
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Nabataean::LetterFinalAleph => "nabataean letter final aleph",
            Nabataean::LetterAleph => "nabataean letter aleph",
            Nabataean::LetterFinalBeth => "nabataean letter final beth",
            Nabataean::LetterBeth => "nabataean letter beth",
            Nabataean::LetterGimel => "nabataean letter gimel",
            Nabataean::LetterDaleth => "nabataean letter daleth",
            Nabataean::LetterFinalHe => "nabataean letter final he",
            Nabataean::LetterHe => "nabataean letter he",
            Nabataean::LetterWaw => "nabataean letter waw",
            Nabataean::LetterZayin => "nabataean letter zayin",
            Nabataean::LetterHeth => "nabataean letter heth",
            Nabataean::LetterTeth => "nabataean letter teth",
            Nabataean::LetterFinalYodh => "nabataean letter final yodh",
            Nabataean::LetterYodh => "nabataean letter yodh",
            Nabataean::LetterFinalKaph => "nabataean letter final kaph",
            Nabataean::LetterKaph => "nabataean letter kaph",
            Nabataean::LetterFinalLamedh => "nabataean letter final lamedh",
            Nabataean::LetterLamedh => "nabataean letter lamedh",
            Nabataean::LetterFinalMem => "nabataean letter final mem",
            Nabataean::LetterMem => "nabataean letter mem",
            Nabataean::LetterFinalNun => "nabataean letter final nun",
            Nabataean::LetterNun => "nabataean letter nun",
            Nabataean::LetterSamekh => "nabataean letter samekh",
            Nabataean::LetterAyin => "nabataean letter ayin",
            Nabataean::LetterPe => "nabataean letter pe",
            Nabataean::LetterSadhe => "nabataean letter sadhe",
            Nabataean::LetterQoph => "nabataean letter qoph",
            Nabataean::LetterResh => "nabataean letter resh",
            Nabataean::LetterFinalShin => "nabataean letter final shin",
            Nabataean::LetterShin => "nabataean letter shin",
            Nabataean::LetterTaw => "nabataean letter taw",
            Nabataean::NumberOne => "nabataean number one",
            Nabataean::NumberTwo => "nabataean number two",
            Nabataean::NumberThree => "nabataean number three",
            Nabataean::NumberFour => "nabataean number four",
            Nabataean::CruciformNumberFour => "nabataean cruciform number four",
            Nabataean::NumberFive => "nabataean number five",
            Nabataean::NumberTen => "nabataean number ten",
            Nabataean::NumberTwenty => "nabataean number twenty",
        }
    }
}
