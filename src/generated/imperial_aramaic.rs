/// \u{10840} → \u{1085f}\
///\
/// 𐡀 𐡁 𐡂 𐡃 𐡄 𐡅 𐡆 𐡇 𐡈 𐡉 𐡊 𐡋 𐡌 𐡍 𐡎 𐡏
/// 𐡐 𐡑 𐡒 𐡓 𐡔 𐡕 𐡗 𐡘 𐡙 𐡚 𐡛 𐡜 𐡝 𐡞
pub mod constants {
    /// \u{10840}: '𐡀'
    pub const IMPERIAL_ARAMAIC_LETTER_ALEPH: char = '𐡀';
    /// \u{10841}: '𐡁'
    pub const IMPERIAL_ARAMAIC_LETTER_BETH: char = '𐡁';
    /// \u{10842}: '𐡂'
    pub const IMPERIAL_ARAMAIC_LETTER_GIMEL: char = '𐡂';
    /// \u{10843}: '𐡃'
    pub const IMPERIAL_ARAMAIC_LETTER_DALETH: char = '𐡃';
    /// \u{10844}: '𐡄'
    pub const IMPERIAL_ARAMAIC_LETTER_HE: char = '𐡄';
    /// \u{10845}: '𐡅'
    pub const IMPERIAL_ARAMAIC_LETTER_WAW: char = '𐡅';
    /// \u{10846}: '𐡆'
    pub const IMPERIAL_ARAMAIC_LETTER_ZAYIN: char = '𐡆';
    /// \u{10847}: '𐡇'
    pub const IMPERIAL_ARAMAIC_LETTER_HETH: char = '𐡇';
    /// \u{10848}: '𐡈'
    pub const IMPERIAL_ARAMAIC_LETTER_TETH: char = '𐡈';
    /// \u{10849}: '𐡉'
    pub const IMPERIAL_ARAMAIC_LETTER_YODH: char = '𐡉';
    /// \u{1084a}: '𐡊'
    pub const IMPERIAL_ARAMAIC_LETTER_KAPH: char = '𐡊';
    /// \u{1084b}: '𐡋'
    pub const IMPERIAL_ARAMAIC_LETTER_LAMEDH: char = '𐡋';
    /// \u{1084c}: '𐡌'
    pub const IMPERIAL_ARAMAIC_LETTER_MEM: char = '𐡌';
    /// \u{1084d}: '𐡍'
    pub const IMPERIAL_ARAMAIC_LETTER_NUN: char = '𐡍';
    /// \u{1084e}: '𐡎'
    pub const IMPERIAL_ARAMAIC_LETTER_SAMEKH: char = '𐡎';
    /// \u{1084f}: '𐡏'
    pub const IMPERIAL_ARAMAIC_LETTER_AYIN: char = '𐡏';
    /// \u{10850}: '𐡐'
    pub const IMPERIAL_ARAMAIC_LETTER_PE: char = '𐡐';
    /// \u{10851}: '𐡑'
    pub const IMPERIAL_ARAMAIC_LETTER_SADHE: char = '𐡑';
    /// \u{10852}: '𐡒'
    pub const IMPERIAL_ARAMAIC_LETTER_QOPH: char = '𐡒';
    /// \u{10853}: '𐡓'
    pub const IMPERIAL_ARAMAIC_LETTER_RESH: char = '𐡓';
    /// \u{10854}: '𐡔'
    pub const IMPERIAL_ARAMAIC_LETTER_SHIN: char = '𐡔';
    /// \u{10855}: '𐡕'
    pub const IMPERIAL_ARAMAIC_LETTER_TAW: char = '𐡕';
    /// \u{10857}: '𐡗'
    pub const IMPERIAL_ARAMAIC_SECTION_SIGN: char = '𐡗';
    /// \u{10858}: '𐡘'
    pub const IMPERIAL_ARAMAIC_NUMBER_ONE: char = '𐡘';
    /// \u{10859}: '𐡙'
    pub const IMPERIAL_ARAMAIC_NUMBER_TWO: char = '𐡙';
    /// \u{1085a}: '𐡚'
    pub const IMPERIAL_ARAMAIC_NUMBER_THREE: char = '𐡚';
    /// \u{1085b}: '𐡛'
    pub const IMPERIAL_ARAMAIC_NUMBER_TEN: char = '𐡛';
    /// \u{1085c}: '𐡜'
    pub const IMPERIAL_ARAMAIC_NUMBER_TWENTY: char = '𐡜';
    /// \u{1085d}: '𐡝'
    pub const IMPERIAL_ARAMAIC_NUMBER_ONE_HUNDRED: char = '𐡝';
    /// \u{1085e}: '𐡞'
    pub const IMPERIAL_ARAMAIC_NUMBER_ONE_THOUSAND: char = '𐡞';
}

/// \u{10840} → \u{1085f}\
///\
/// 𐡀 𐡁 𐡂 𐡃 𐡄 𐡅 𐡆 𐡇 𐡈 𐡉 𐡊 𐡋 𐡌 𐡍 𐡎 𐡏
/// 𐡐 𐡑 𐡒 𐡓 𐡔 𐡕 𐡗 𐡘 𐡙 𐡚 𐡛 𐡜 𐡝 𐡞
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ImperialAramaic {
    /// \u{10840}: '𐡀'
    ImperialAramaicLetterAleph,
    /// \u{10841}: '𐡁'
    ImperialAramaicLetterBeth,
    /// \u{10842}: '𐡂'
    ImperialAramaicLetterGimel,
    /// \u{10843}: '𐡃'
    ImperialAramaicLetterDaleth,
    /// \u{10844}: '𐡄'
    ImperialAramaicLetterHe,
    /// \u{10845}: '𐡅'
    ImperialAramaicLetterWaw,
    /// \u{10846}: '𐡆'
    ImperialAramaicLetterZayin,
    /// \u{10847}: '𐡇'
    ImperialAramaicLetterHeth,
    /// \u{10848}: '𐡈'
    ImperialAramaicLetterTeth,
    /// \u{10849}: '𐡉'
    ImperialAramaicLetterYodh,
    /// \u{1084a}: '𐡊'
    ImperialAramaicLetterKaph,
    /// \u{1084b}: '𐡋'
    ImperialAramaicLetterLamedh,
    /// \u{1084c}: '𐡌'
    ImperialAramaicLetterMem,
    /// \u{1084d}: '𐡍'
    ImperialAramaicLetterNun,
    /// \u{1084e}: '𐡎'
    ImperialAramaicLetterSamekh,
    /// \u{1084f}: '𐡏'
    ImperialAramaicLetterAyin,
    /// \u{10850}: '𐡐'
    ImperialAramaicLetterPe,
    /// \u{10851}: '𐡑'
    ImperialAramaicLetterSadhe,
    /// \u{10852}: '𐡒'
    ImperialAramaicLetterQoph,
    /// \u{10853}: '𐡓'
    ImperialAramaicLetterResh,
    /// \u{10854}: '𐡔'
    ImperialAramaicLetterShin,
    /// \u{10855}: '𐡕'
    ImperialAramaicLetterTaw,
    /// \u{10857}: '𐡗'
    ImperialAramaicSectionSign,
    /// \u{10858}: '𐡘'
    ImperialAramaicNumberOne,
    /// \u{10859}: '𐡙'
    ImperialAramaicNumberTwo,
    /// \u{1085a}: '𐡚'
    ImperialAramaicNumberThree,
    /// \u{1085b}: '𐡛'
    ImperialAramaicNumberTen,
    /// \u{1085c}: '𐡜'
    ImperialAramaicNumberTwenty,
    /// \u{1085d}: '𐡝'
    ImperialAramaicNumberOneHundred,
    /// \u{1085e}: '𐡞'
    ImperialAramaicNumberOneThousand,
}

impl Into<char> for ImperialAramaic {
    fn into(self) -> char {
        use constants::*;
        match self {
            ImperialAramaic::ImperialAramaicLetterAleph => IMPERIAL_ARAMAIC_LETTER_ALEPH,
            ImperialAramaic::ImperialAramaicLetterBeth => IMPERIAL_ARAMAIC_LETTER_BETH,
            ImperialAramaic::ImperialAramaicLetterGimel => IMPERIAL_ARAMAIC_LETTER_GIMEL,
            ImperialAramaic::ImperialAramaicLetterDaleth => IMPERIAL_ARAMAIC_LETTER_DALETH,
            ImperialAramaic::ImperialAramaicLetterHe => IMPERIAL_ARAMAIC_LETTER_HE,
            ImperialAramaic::ImperialAramaicLetterWaw => IMPERIAL_ARAMAIC_LETTER_WAW,
            ImperialAramaic::ImperialAramaicLetterZayin => IMPERIAL_ARAMAIC_LETTER_ZAYIN,
            ImperialAramaic::ImperialAramaicLetterHeth => IMPERIAL_ARAMAIC_LETTER_HETH,
            ImperialAramaic::ImperialAramaicLetterTeth => IMPERIAL_ARAMAIC_LETTER_TETH,
            ImperialAramaic::ImperialAramaicLetterYodh => IMPERIAL_ARAMAIC_LETTER_YODH,
            ImperialAramaic::ImperialAramaicLetterKaph => IMPERIAL_ARAMAIC_LETTER_KAPH,
            ImperialAramaic::ImperialAramaicLetterLamedh => IMPERIAL_ARAMAIC_LETTER_LAMEDH,
            ImperialAramaic::ImperialAramaicLetterMem => IMPERIAL_ARAMAIC_LETTER_MEM,
            ImperialAramaic::ImperialAramaicLetterNun => IMPERIAL_ARAMAIC_LETTER_NUN,
            ImperialAramaic::ImperialAramaicLetterSamekh => IMPERIAL_ARAMAIC_LETTER_SAMEKH,
            ImperialAramaic::ImperialAramaicLetterAyin => IMPERIAL_ARAMAIC_LETTER_AYIN,
            ImperialAramaic::ImperialAramaicLetterPe => IMPERIAL_ARAMAIC_LETTER_PE,
            ImperialAramaic::ImperialAramaicLetterSadhe => IMPERIAL_ARAMAIC_LETTER_SADHE,
            ImperialAramaic::ImperialAramaicLetterQoph => IMPERIAL_ARAMAIC_LETTER_QOPH,
            ImperialAramaic::ImperialAramaicLetterResh => IMPERIAL_ARAMAIC_LETTER_RESH,
            ImperialAramaic::ImperialAramaicLetterShin => IMPERIAL_ARAMAIC_LETTER_SHIN,
            ImperialAramaic::ImperialAramaicLetterTaw => IMPERIAL_ARAMAIC_LETTER_TAW,
            ImperialAramaic::ImperialAramaicSectionSign => IMPERIAL_ARAMAIC_SECTION_SIGN,
            ImperialAramaic::ImperialAramaicNumberOne => IMPERIAL_ARAMAIC_NUMBER_ONE,
            ImperialAramaic::ImperialAramaicNumberTwo => IMPERIAL_ARAMAIC_NUMBER_TWO,
            ImperialAramaic::ImperialAramaicNumberThree => IMPERIAL_ARAMAIC_NUMBER_THREE,
            ImperialAramaic::ImperialAramaicNumberTen => IMPERIAL_ARAMAIC_NUMBER_TEN,
            ImperialAramaic::ImperialAramaicNumberTwenty => IMPERIAL_ARAMAIC_NUMBER_TWENTY,
            ImperialAramaic::ImperialAramaicNumberOneHundred => IMPERIAL_ARAMAIC_NUMBER_ONE_HUNDRED,
            ImperialAramaic::ImperialAramaicNumberOneThousand => IMPERIAL_ARAMAIC_NUMBER_ONE_THOUSAND,
        }
    }
}

impl std::convert::TryFrom<char> for ImperialAramaic {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            IMPERIAL_ARAMAIC_LETTER_ALEPH => Ok(ImperialAramaic::ImperialAramaicLetterAleph),
            IMPERIAL_ARAMAIC_LETTER_BETH => Ok(ImperialAramaic::ImperialAramaicLetterBeth),
            IMPERIAL_ARAMAIC_LETTER_GIMEL => Ok(ImperialAramaic::ImperialAramaicLetterGimel),
            IMPERIAL_ARAMAIC_LETTER_DALETH => Ok(ImperialAramaic::ImperialAramaicLetterDaleth),
            IMPERIAL_ARAMAIC_LETTER_HE => Ok(ImperialAramaic::ImperialAramaicLetterHe),
            IMPERIAL_ARAMAIC_LETTER_WAW => Ok(ImperialAramaic::ImperialAramaicLetterWaw),
            IMPERIAL_ARAMAIC_LETTER_ZAYIN => Ok(ImperialAramaic::ImperialAramaicLetterZayin),
            IMPERIAL_ARAMAIC_LETTER_HETH => Ok(ImperialAramaic::ImperialAramaicLetterHeth),
            IMPERIAL_ARAMAIC_LETTER_TETH => Ok(ImperialAramaic::ImperialAramaicLetterTeth),
            IMPERIAL_ARAMAIC_LETTER_YODH => Ok(ImperialAramaic::ImperialAramaicLetterYodh),
            IMPERIAL_ARAMAIC_LETTER_KAPH => Ok(ImperialAramaic::ImperialAramaicLetterKaph),
            IMPERIAL_ARAMAIC_LETTER_LAMEDH => Ok(ImperialAramaic::ImperialAramaicLetterLamedh),
            IMPERIAL_ARAMAIC_LETTER_MEM => Ok(ImperialAramaic::ImperialAramaicLetterMem),
            IMPERIAL_ARAMAIC_LETTER_NUN => Ok(ImperialAramaic::ImperialAramaicLetterNun),
            IMPERIAL_ARAMAIC_LETTER_SAMEKH => Ok(ImperialAramaic::ImperialAramaicLetterSamekh),
            IMPERIAL_ARAMAIC_LETTER_AYIN => Ok(ImperialAramaic::ImperialAramaicLetterAyin),
            IMPERIAL_ARAMAIC_LETTER_PE => Ok(ImperialAramaic::ImperialAramaicLetterPe),
            IMPERIAL_ARAMAIC_LETTER_SADHE => Ok(ImperialAramaic::ImperialAramaicLetterSadhe),
            IMPERIAL_ARAMAIC_LETTER_QOPH => Ok(ImperialAramaic::ImperialAramaicLetterQoph),
            IMPERIAL_ARAMAIC_LETTER_RESH => Ok(ImperialAramaic::ImperialAramaicLetterResh),
            IMPERIAL_ARAMAIC_LETTER_SHIN => Ok(ImperialAramaic::ImperialAramaicLetterShin),
            IMPERIAL_ARAMAIC_LETTER_TAW => Ok(ImperialAramaic::ImperialAramaicLetterTaw),
            IMPERIAL_ARAMAIC_SECTION_SIGN => Ok(ImperialAramaic::ImperialAramaicSectionSign),
            IMPERIAL_ARAMAIC_NUMBER_ONE => Ok(ImperialAramaic::ImperialAramaicNumberOne),
            IMPERIAL_ARAMAIC_NUMBER_TWO => Ok(ImperialAramaic::ImperialAramaicNumberTwo),
            IMPERIAL_ARAMAIC_NUMBER_THREE => Ok(ImperialAramaic::ImperialAramaicNumberThree),
            IMPERIAL_ARAMAIC_NUMBER_TEN => Ok(ImperialAramaic::ImperialAramaicNumberTen),
            IMPERIAL_ARAMAIC_NUMBER_TWENTY => Ok(ImperialAramaic::ImperialAramaicNumberTwenty),
            IMPERIAL_ARAMAIC_NUMBER_ONE_HUNDRED => Ok(ImperialAramaic::ImperialAramaicNumberOneHundred),
            IMPERIAL_ARAMAIC_NUMBER_ONE_THOUSAND => Ok(ImperialAramaic::ImperialAramaicNumberOneThousand),
            _ => Err(()),
        }
    }
}

impl Into<u32> for ImperialAramaic {
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

impl std::convert::TryFrom<u32> for ImperialAramaic {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for ImperialAramaic {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl ImperialAramaic {
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        ImperialAramaic::ImperialAramaicLetterAleph
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            ImperialAramaic::ImperialAramaicLetterAleph => "imperial aramaic letter aleph",
            ImperialAramaic::ImperialAramaicLetterBeth => "imperial aramaic letter beth",
            ImperialAramaic::ImperialAramaicLetterGimel => "imperial aramaic letter gimel",
            ImperialAramaic::ImperialAramaicLetterDaleth => "imperial aramaic letter daleth",
            ImperialAramaic::ImperialAramaicLetterHe => "imperial aramaic letter he",
            ImperialAramaic::ImperialAramaicLetterWaw => "imperial aramaic letter waw",
            ImperialAramaic::ImperialAramaicLetterZayin => "imperial aramaic letter zayin",
            ImperialAramaic::ImperialAramaicLetterHeth => "imperial aramaic letter heth",
            ImperialAramaic::ImperialAramaicLetterTeth => "imperial aramaic letter teth",
            ImperialAramaic::ImperialAramaicLetterYodh => "imperial aramaic letter yodh",
            ImperialAramaic::ImperialAramaicLetterKaph => "imperial aramaic letter kaph",
            ImperialAramaic::ImperialAramaicLetterLamedh => "imperial aramaic letter lamedh",
            ImperialAramaic::ImperialAramaicLetterMem => "imperial aramaic letter mem",
            ImperialAramaic::ImperialAramaicLetterNun => "imperial aramaic letter nun",
            ImperialAramaic::ImperialAramaicLetterSamekh => "imperial aramaic letter samekh",
            ImperialAramaic::ImperialAramaicLetterAyin => "imperial aramaic letter ayin",
            ImperialAramaic::ImperialAramaicLetterPe => "imperial aramaic letter pe",
            ImperialAramaic::ImperialAramaicLetterSadhe => "imperial aramaic letter sadhe",
            ImperialAramaic::ImperialAramaicLetterQoph => "imperial aramaic letter qoph",
            ImperialAramaic::ImperialAramaicLetterResh => "imperial aramaic letter resh",
            ImperialAramaic::ImperialAramaicLetterShin => "imperial aramaic letter shin",
            ImperialAramaic::ImperialAramaicLetterTaw => "imperial aramaic letter taw",
            ImperialAramaic::ImperialAramaicSectionSign => "imperial aramaic section sign",
            ImperialAramaic::ImperialAramaicNumberOne => "imperial aramaic number one",
            ImperialAramaic::ImperialAramaicNumberTwo => "imperial aramaic number two",
            ImperialAramaic::ImperialAramaicNumberThree => "imperial aramaic number three",
            ImperialAramaic::ImperialAramaicNumberTen => "imperial aramaic number ten",
            ImperialAramaic::ImperialAramaicNumberTwenty => "imperial aramaic number twenty",
            ImperialAramaic::ImperialAramaicNumberOneHundred => "imperial aramaic number one hundred",
            ImperialAramaic::ImperialAramaicNumberOneThousand => "imperial aramaic number one thousand",
        }
    }
}
