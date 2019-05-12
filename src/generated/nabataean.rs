/// \u{10880} → \u{108af}\
///\
/// 𐢀 𐢁 𐢂 𐢃 𐢄 𐢅 𐢆 𐢇 𐢈 𐢉 𐢊 𐢋 𐢌 𐢍 𐢎 𐢏
/// 𐢐 𐢑 𐢒 𐢓 𐢔 𐢕 𐢖 𐢗 𐢘 𐢙 𐢚 𐢛 𐢜 𐢝 𐢞 𐢧
/// 𐢨 𐢩 𐢪 𐢫 𐢬 𐢭 𐢮
pub mod constants {
    /// \u{10880}: '𐢀'
    pub const NABATAEAN_LETTER_FINAL_ALEPH: char = '𐢀';
    /// \u{10881}: '𐢁'
    pub const NABATAEAN_LETTER_ALEPH: char = '𐢁';
    /// \u{10882}: '𐢂'
    pub const NABATAEAN_LETTER_FINAL_BETH: char = '𐢂';
    /// \u{10883}: '𐢃'
    pub const NABATAEAN_LETTER_BETH: char = '𐢃';
    /// \u{10884}: '𐢄'
    pub const NABATAEAN_LETTER_GIMEL: char = '𐢄';
    /// \u{10885}: '𐢅'
    pub const NABATAEAN_LETTER_DALETH: char = '𐢅';
    /// \u{10886}: '𐢆'
    pub const NABATAEAN_LETTER_FINAL_HE: char = '𐢆';
    /// \u{10887}: '𐢇'
    pub const NABATAEAN_LETTER_HE: char = '𐢇';
    /// \u{10888}: '𐢈'
    pub const NABATAEAN_LETTER_WAW: char = '𐢈';
    /// \u{10889}: '𐢉'
    pub const NABATAEAN_LETTER_ZAYIN: char = '𐢉';
    /// \u{1088a}: '𐢊'
    pub const NABATAEAN_LETTER_HETH: char = '𐢊';
    /// \u{1088b}: '𐢋'
    pub const NABATAEAN_LETTER_TETH: char = '𐢋';
    /// \u{1088c}: '𐢌'
    pub const NABATAEAN_LETTER_FINAL_YODH: char = '𐢌';
    /// \u{1088d}: '𐢍'
    pub const NABATAEAN_LETTER_YODH: char = '𐢍';
    /// \u{1088e}: '𐢎'
    pub const NABATAEAN_LETTER_FINAL_KAPH: char = '𐢎';
    /// \u{1088f}: '𐢏'
    pub const NABATAEAN_LETTER_KAPH: char = '𐢏';
    /// \u{10890}: '𐢐'
    pub const NABATAEAN_LETTER_FINAL_LAMEDH: char = '𐢐';
    /// \u{10891}: '𐢑'
    pub const NABATAEAN_LETTER_LAMEDH: char = '𐢑';
    /// \u{10892}: '𐢒'
    pub const NABATAEAN_LETTER_FINAL_MEM: char = '𐢒';
    /// \u{10893}: '𐢓'
    pub const NABATAEAN_LETTER_MEM: char = '𐢓';
    /// \u{10894}: '𐢔'
    pub const NABATAEAN_LETTER_FINAL_NUN: char = '𐢔';
    /// \u{10895}: '𐢕'
    pub const NABATAEAN_LETTER_NUN: char = '𐢕';
    /// \u{10896}: '𐢖'
    pub const NABATAEAN_LETTER_SAMEKH: char = '𐢖';
    /// \u{10897}: '𐢗'
    pub const NABATAEAN_LETTER_AYIN: char = '𐢗';
    /// \u{10898}: '𐢘'
    pub const NABATAEAN_LETTER_PE: char = '𐢘';
    /// \u{10899}: '𐢙'
    pub const NABATAEAN_LETTER_SADHE: char = '𐢙';
    /// \u{1089a}: '𐢚'
    pub const NABATAEAN_LETTER_QOPH: char = '𐢚';
    /// \u{1089b}: '𐢛'
    pub const NABATAEAN_LETTER_RESH: char = '𐢛';
    /// \u{1089c}: '𐢜'
    pub const NABATAEAN_LETTER_FINAL_SHIN: char = '𐢜';
    /// \u{1089d}: '𐢝'
    pub const NABATAEAN_LETTER_SHIN: char = '𐢝';
    /// \u{1089e}: '𐢞'
    pub const NABATAEAN_LETTER_TAW: char = '𐢞';
    /// \u{108a7}: '𐢧'
    pub const NABATAEAN_NUMBER_ONE: char = '𐢧';
    /// \u{108a8}: '𐢨'
    pub const NABATAEAN_NUMBER_TWO: char = '𐢨';
    /// \u{108a9}: '𐢩'
    pub const NABATAEAN_NUMBER_THREE: char = '𐢩';
    /// \u{108aa}: '𐢪'
    pub const NABATAEAN_NUMBER_FOUR: char = '𐢪';
    /// \u{108ab}: '𐢫'
    pub const NABATAEAN_CRUCIFORM_NUMBER_FOUR: char = '𐢫';
    /// \u{108ac}: '𐢬'
    pub const NABATAEAN_NUMBER_FIVE: char = '𐢬';
    /// \u{108ad}: '𐢭'
    pub const NABATAEAN_NUMBER_TEN: char = '𐢭';
    /// \u{108ae}: '𐢮'
    pub const NABATAEAN_NUMBER_TWENTY: char = '𐢮';
}

/// \u{10880} → \u{108af}\
///\
/// 𐢀 𐢁 𐢂 𐢃 𐢄 𐢅 𐢆 𐢇 𐢈 𐢉 𐢊 𐢋 𐢌 𐢍 𐢎 𐢏
/// 𐢐 𐢑 𐢒 𐢓 𐢔 𐢕 𐢖 𐢗 𐢘 𐢙 𐢚 𐢛 𐢜 𐢝 𐢞 𐢧
/// 𐢨 𐢩 𐢪 𐢫 𐢬 𐢭 𐢮
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Nabataean {
    /// \u{10880}: '𐢀'
    NabataeanLetterFinalAleph,
    /// \u{10881}: '𐢁'
    NabataeanLetterAleph,
    /// \u{10882}: '𐢂'
    NabataeanLetterFinalBeth,
    /// \u{10883}: '𐢃'
    NabataeanLetterBeth,
    /// \u{10884}: '𐢄'
    NabataeanLetterGimel,
    /// \u{10885}: '𐢅'
    NabataeanLetterDaleth,
    /// \u{10886}: '𐢆'
    NabataeanLetterFinalHe,
    /// \u{10887}: '𐢇'
    NabataeanLetterHe,
    /// \u{10888}: '𐢈'
    NabataeanLetterWaw,
    /// \u{10889}: '𐢉'
    NabataeanLetterZayin,
    /// \u{1088a}: '𐢊'
    NabataeanLetterHeth,
    /// \u{1088b}: '𐢋'
    NabataeanLetterTeth,
    /// \u{1088c}: '𐢌'
    NabataeanLetterFinalYodh,
    /// \u{1088d}: '𐢍'
    NabataeanLetterYodh,
    /// \u{1088e}: '𐢎'
    NabataeanLetterFinalKaph,
    /// \u{1088f}: '𐢏'
    NabataeanLetterKaph,
    /// \u{10890}: '𐢐'
    NabataeanLetterFinalLamedh,
    /// \u{10891}: '𐢑'
    NabataeanLetterLamedh,
    /// \u{10892}: '𐢒'
    NabataeanLetterFinalMem,
    /// \u{10893}: '𐢓'
    NabataeanLetterMem,
    /// \u{10894}: '𐢔'
    NabataeanLetterFinalNun,
    /// \u{10895}: '𐢕'
    NabataeanLetterNun,
    /// \u{10896}: '𐢖'
    NabataeanLetterSamekh,
    /// \u{10897}: '𐢗'
    NabataeanLetterAyin,
    /// \u{10898}: '𐢘'
    NabataeanLetterPe,
    /// \u{10899}: '𐢙'
    NabataeanLetterSadhe,
    /// \u{1089a}: '𐢚'
    NabataeanLetterQoph,
    /// \u{1089b}: '𐢛'
    NabataeanLetterResh,
    /// \u{1089c}: '𐢜'
    NabataeanLetterFinalShin,
    /// \u{1089d}: '𐢝'
    NabataeanLetterShin,
    /// \u{1089e}: '𐢞'
    NabataeanLetterTaw,
    /// \u{108a7}: '𐢧'
    NabataeanNumberOne,
    /// \u{108a8}: '𐢨'
    NabataeanNumberTwo,
    /// \u{108a9}: '𐢩'
    NabataeanNumberThree,
    /// \u{108aa}: '𐢪'
    NabataeanNumberFour,
    /// \u{108ab}: '𐢫'
    NabataeanCruciformNumberFour,
    /// \u{108ac}: '𐢬'
    NabataeanNumberFive,
    /// \u{108ad}: '𐢭'
    NabataeanNumberTen,
    /// \u{108ae}: '𐢮'
    NabataeanNumberTwenty,
}

impl Into<char> for Nabataean {
    fn into(self) -> char {
        use constants::*;
        match self {
            Nabataean::NabataeanLetterFinalAleph => NABATAEAN_LETTER_FINAL_ALEPH,
            Nabataean::NabataeanLetterAleph => NABATAEAN_LETTER_ALEPH,
            Nabataean::NabataeanLetterFinalBeth => NABATAEAN_LETTER_FINAL_BETH,
            Nabataean::NabataeanLetterBeth => NABATAEAN_LETTER_BETH,
            Nabataean::NabataeanLetterGimel => NABATAEAN_LETTER_GIMEL,
            Nabataean::NabataeanLetterDaleth => NABATAEAN_LETTER_DALETH,
            Nabataean::NabataeanLetterFinalHe => NABATAEAN_LETTER_FINAL_HE,
            Nabataean::NabataeanLetterHe => NABATAEAN_LETTER_HE,
            Nabataean::NabataeanLetterWaw => NABATAEAN_LETTER_WAW,
            Nabataean::NabataeanLetterZayin => NABATAEAN_LETTER_ZAYIN,
            Nabataean::NabataeanLetterHeth => NABATAEAN_LETTER_HETH,
            Nabataean::NabataeanLetterTeth => NABATAEAN_LETTER_TETH,
            Nabataean::NabataeanLetterFinalYodh => NABATAEAN_LETTER_FINAL_YODH,
            Nabataean::NabataeanLetterYodh => NABATAEAN_LETTER_YODH,
            Nabataean::NabataeanLetterFinalKaph => NABATAEAN_LETTER_FINAL_KAPH,
            Nabataean::NabataeanLetterKaph => NABATAEAN_LETTER_KAPH,
            Nabataean::NabataeanLetterFinalLamedh => NABATAEAN_LETTER_FINAL_LAMEDH,
            Nabataean::NabataeanLetterLamedh => NABATAEAN_LETTER_LAMEDH,
            Nabataean::NabataeanLetterFinalMem => NABATAEAN_LETTER_FINAL_MEM,
            Nabataean::NabataeanLetterMem => NABATAEAN_LETTER_MEM,
            Nabataean::NabataeanLetterFinalNun => NABATAEAN_LETTER_FINAL_NUN,
            Nabataean::NabataeanLetterNun => NABATAEAN_LETTER_NUN,
            Nabataean::NabataeanLetterSamekh => NABATAEAN_LETTER_SAMEKH,
            Nabataean::NabataeanLetterAyin => NABATAEAN_LETTER_AYIN,
            Nabataean::NabataeanLetterPe => NABATAEAN_LETTER_PE,
            Nabataean::NabataeanLetterSadhe => NABATAEAN_LETTER_SADHE,
            Nabataean::NabataeanLetterQoph => NABATAEAN_LETTER_QOPH,
            Nabataean::NabataeanLetterResh => NABATAEAN_LETTER_RESH,
            Nabataean::NabataeanLetterFinalShin => NABATAEAN_LETTER_FINAL_SHIN,
            Nabataean::NabataeanLetterShin => NABATAEAN_LETTER_SHIN,
            Nabataean::NabataeanLetterTaw => NABATAEAN_LETTER_TAW,
            Nabataean::NabataeanNumberOne => NABATAEAN_NUMBER_ONE,
            Nabataean::NabataeanNumberTwo => NABATAEAN_NUMBER_TWO,
            Nabataean::NabataeanNumberThree => NABATAEAN_NUMBER_THREE,
            Nabataean::NabataeanNumberFour => NABATAEAN_NUMBER_FOUR,
            Nabataean::NabataeanCruciformNumberFour => NABATAEAN_CRUCIFORM_NUMBER_FOUR,
            Nabataean::NabataeanNumberFive => NABATAEAN_NUMBER_FIVE,
            Nabataean::NabataeanNumberTen => NABATAEAN_NUMBER_TEN,
            Nabataean::NabataeanNumberTwenty => NABATAEAN_NUMBER_TWENTY,
        }
    }
}

impl std::convert::TryFrom<char> for Nabataean {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            NABATAEAN_LETTER_FINAL_ALEPH => Ok(Nabataean::NabataeanLetterFinalAleph),
            NABATAEAN_LETTER_ALEPH => Ok(Nabataean::NabataeanLetterAleph),
            NABATAEAN_LETTER_FINAL_BETH => Ok(Nabataean::NabataeanLetterFinalBeth),
            NABATAEAN_LETTER_BETH => Ok(Nabataean::NabataeanLetterBeth),
            NABATAEAN_LETTER_GIMEL => Ok(Nabataean::NabataeanLetterGimel),
            NABATAEAN_LETTER_DALETH => Ok(Nabataean::NabataeanLetterDaleth),
            NABATAEAN_LETTER_FINAL_HE => Ok(Nabataean::NabataeanLetterFinalHe),
            NABATAEAN_LETTER_HE => Ok(Nabataean::NabataeanLetterHe),
            NABATAEAN_LETTER_WAW => Ok(Nabataean::NabataeanLetterWaw),
            NABATAEAN_LETTER_ZAYIN => Ok(Nabataean::NabataeanLetterZayin),
            NABATAEAN_LETTER_HETH => Ok(Nabataean::NabataeanLetterHeth),
            NABATAEAN_LETTER_TETH => Ok(Nabataean::NabataeanLetterTeth),
            NABATAEAN_LETTER_FINAL_YODH => Ok(Nabataean::NabataeanLetterFinalYodh),
            NABATAEAN_LETTER_YODH => Ok(Nabataean::NabataeanLetterYodh),
            NABATAEAN_LETTER_FINAL_KAPH => Ok(Nabataean::NabataeanLetterFinalKaph),
            NABATAEAN_LETTER_KAPH => Ok(Nabataean::NabataeanLetterKaph),
            NABATAEAN_LETTER_FINAL_LAMEDH => Ok(Nabataean::NabataeanLetterFinalLamedh),
            NABATAEAN_LETTER_LAMEDH => Ok(Nabataean::NabataeanLetterLamedh),
            NABATAEAN_LETTER_FINAL_MEM => Ok(Nabataean::NabataeanLetterFinalMem),
            NABATAEAN_LETTER_MEM => Ok(Nabataean::NabataeanLetterMem),
            NABATAEAN_LETTER_FINAL_NUN => Ok(Nabataean::NabataeanLetterFinalNun),
            NABATAEAN_LETTER_NUN => Ok(Nabataean::NabataeanLetterNun),
            NABATAEAN_LETTER_SAMEKH => Ok(Nabataean::NabataeanLetterSamekh),
            NABATAEAN_LETTER_AYIN => Ok(Nabataean::NabataeanLetterAyin),
            NABATAEAN_LETTER_PE => Ok(Nabataean::NabataeanLetterPe),
            NABATAEAN_LETTER_SADHE => Ok(Nabataean::NabataeanLetterSadhe),
            NABATAEAN_LETTER_QOPH => Ok(Nabataean::NabataeanLetterQoph),
            NABATAEAN_LETTER_RESH => Ok(Nabataean::NabataeanLetterResh),
            NABATAEAN_LETTER_FINAL_SHIN => Ok(Nabataean::NabataeanLetterFinalShin),
            NABATAEAN_LETTER_SHIN => Ok(Nabataean::NabataeanLetterShin),
            NABATAEAN_LETTER_TAW => Ok(Nabataean::NabataeanLetterTaw),
            NABATAEAN_NUMBER_ONE => Ok(Nabataean::NabataeanNumberOne),
            NABATAEAN_NUMBER_TWO => Ok(Nabataean::NabataeanNumberTwo),
            NABATAEAN_NUMBER_THREE => Ok(Nabataean::NabataeanNumberThree),
            NABATAEAN_NUMBER_FOUR => Ok(Nabataean::NabataeanNumberFour),
            NABATAEAN_CRUCIFORM_NUMBER_FOUR => Ok(Nabataean::NabataeanCruciformNumberFour),
            NABATAEAN_NUMBER_FIVE => Ok(Nabataean::NabataeanNumberFive),
            NABATAEAN_NUMBER_TEN => Ok(Nabataean::NabataeanNumberTen),
            NABATAEAN_NUMBER_TWENTY => Ok(Nabataean::NabataeanNumberTwenty),
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
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        Nabataean::NabataeanLetterFinalAleph
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Nabataean::NabataeanLetterFinalAleph => "nabataean letter final aleph",
            Nabataean::NabataeanLetterAleph => "nabataean letter aleph",
            Nabataean::NabataeanLetterFinalBeth => "nabataean letter final beth",
            Nabataean::NabataeanLetterBeth => "nabataean letter beth",
            Nabataean::NabataeanLetterGimel => "nabataean letter gimel",
            Nabataean::NabataeanLetterDaleth => "nabataean letter daleth",
            Nabataean::NabataeanLetterFinalHe => "nabataean letter final he",
            Nabataean::NabataeanLetterHe => "nabataean letter he",
            Nabataean::NabataeanLetterWaw => "nabataean letter waw",
            Nabataean::NabataeanLetterZayin => "nabataean letter zayin",
            Nabataean::NabataeanLetterHeth => "nabataean letter heth",
            Nabataean::NabataeanLetterTeth => "nabataean letter teth",
            Nabataean::NabataeanLetterFinalYodh => "nabataean letter final yodh",
            Nabataean::NabataeanLetterYodh => "nabataean letter yodh",
            Nabataean::NabataeanLetterFinalKaph => "nabataean letter final kaph",
            Nabataean::NabataeanLetterKaph => "nabataean letter kaph",
            Nabataean::NabataeanLetterFinalLamedh => "nabataean letter final lamedh",
            Nabataean::NabataeanLetterLamedh => "nabataean letter lamedh",
            Nabataean::NabataeanLetterFinalMem => "nabataean letter final mem",
            Nabataean::NabataeanLetterMem => "nabataean letter mem",
            Nabataean::NabataeanLetterFinalNun => "nabataean letter final nun",
            Nabataean::NabataeanLetterNun => "nabataean letter nun",
            Nabataean::NabataeanLetterSamekh => "nabataean letter samekh",
            Nabataean::NabataeanLetterAyin => "nabataean letter ayin",
            Nabataean::NabataeanLetterPe => "nabataean letter pe",
            Nabataean::NabataeanLetterSadhe => "nabataean letter sadhe",
            Nabataean::NabataeanLetterQoph => "nabataean letter qoph",
            Nabataean::NabataeanLetterResh => "nabataean letter resh",
            Nabataean::NabataeanLetterFinalShin => "nabataean letter final shin",
            Nabataean::NabataeanLetterShin => "nabataean letter shin",
            Nabataean::NabataeanLetterTaw => "nabataean letter taw",
            Nabataean::NabataeanNumberOne => "nabataean number one",
            Nabataean::NabataeanNumberTwo => "nabataean number two",
            Nabataean::NabataeanNumberThree => "nabataean number three",
            Nabataean::NabataeanNumberFour => "nabataean number four",
            Nabataean::NabataeanCruciformNumberFour => "nabataean cruciform number four",
            Nabataean::NabataeanNumberFive => "nabataean number five",
            Nabataean::NabataeanNumberTen => "nabataean number ten",
            Nabataean::NabataeanNumberTwenty => "nabataean number twenty",
        }
    }
}
