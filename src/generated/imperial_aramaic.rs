/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{10840}: '𐡀'
    pub const LETTER_ALEPH: char = '𐡀';
    /// \u{10841}: '𐡁'
    pub const LETTER_BETH: char = '𐡁';
    /// \u{10842}: '𐡂'
    pub const LETTER_GIMEL: char = '𐡂';
    /// \u{10843}: '𐡃'
    pub const LETTER_DALETH: char = '𐡃';
    /// \u{10844}: '𐡄'
    pub const LETTER_HE: char = '𐡄';
    /// \u{10845}: '𐡅'
    pub const LETTER_WAW: char = '𐡅';
    /// \u{10846}: '𐡆'
    pub const LETTER_ZAYIN: char = '𐡆';
    /// \u{10847}: '𐡇'
    pub const LETTER_HETH: char = '𐡇';
    /// \u{10848}: '𐡈'
    pub const LETTER_TETH: char = '𐡈';
    /// \u{10849}: '𐡉'
    pub const LETTER_YODH: char = '𐡉';
    /// \u{1084a}: '𐡊'
    pub const LETTER_KAPH: char = '𐡊';
    /// \u{1084b}: '𐡋'
    pub const LETTER_LAMEDH: char = '𐡋';
    /// \u{1084c}: '𐡌'
    pub const LETTER_MEM: char = '𐡌';
    /// \u{1084d}: '𐡍'
    pub const LETTER_NUN: char = '𐡍';
    /// \u{1084e}: '𐡎'
    pub const LETTER_SAMEKH: char = '𐡎';
    /// \u{1084f}: '𐡏'
    pub const LETTER_AYIN: char = '𐡏';
    /// \u{10850}: '𐡐'
    pub const LETTER_PE: char = '𐡐';
    /// \u{10851}: '𐡑'
    pub const LETTER_SADHE: char = '𐡑';
    /// \u{10852}: '𐡒'
    pub const LETTER_QOPH: char = '𐡒';
    /// \u{10853}: '𐡓'
    pub const LETTER_RESH: char = '𐡓';
    /// \u{10854}: '𐡔'
    pub const LETTER_SHIN: char = '𐡔';
    /// \u{10855}: '𐡕'
    pub const LETTER_TAW: char = '𐡕';
    /// \u{10857}: '𐡗'
    pub const SECTION_SIGN: char = '𐡗';
    /// \u{10858}: '𐡘'
    pub const NUMBER_ONE: char = '𐡘';
    /// \u{10859}: '𐡙'
    pub const NUMBER_TWO: char = '𐡙';
    /// \u{1085a}: '𐡚'
    pub const NUMBER_THREE: char = '𐡚';
    /// \u{1085b}: '𐡛'
    pub const NUMBER_TEN: char = '𐡛';
    /// \u{1085c}: '𐡜'
    pub const NUMBER_TWENTY: char = '𐡜';
    /// \u{1085d}: '𐡝'
    pub const NUMBER_ONE_HUNDRED: char = '𐡝';
    /// \u{1085e}: '𐡞'
    pub const NUMBER_ONE_THOUSAND: char = '𐡞';
}

/// An enum to represent all characters in the ImperialAramaic block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ImperialAramaic {
    /// \u{10840}: '𐡀'
    LetterAleph,
    /// \u{10841}: '𐡁'
    LetterBeth,
    /// \u{10842}: '𐡂'
    LetterGimel,
    /// \u{10843}: '𐡃'
    LetterDaleth,
    /// \u{10844}: '𐡄'
    LetterHe,
    /// \u{10845}: '𐡅'
    LetterWaw,
    /// \u{10846}: '𐡆'
    LetterZayin,
    /// \u{10847}: '𐡇'
    LetterHeth,
    /// \u{10848}: '𐡈'
    LetterTeth,
    /// \u{10849}: '𐡉'
    LetterYodh,
    /// \u{1084a}: '𐡊'
    LetterKaph,
    /// \u{1084b}: '𐡋'
    LetterLamedh,
    /// \u{1084c}: '𐡌'
    LetterMem,
    /// \u{1084d}: '𐡍'
    LetterNun,
    /// \u{1084e}: '𐡎'
    LetterSamekh,
    /// \u{1084f}: '𐡏'
    LetterAyin,
    /// \u{10850}: '𐡐'
    LetterPe,
    /// \u{10851}: '𐡑'
    LetterSadhe,
    /// \u{10852}: '𐡒'
    LetterQoph,
    /// \u{10853}: '𐡓'
    LetterResh,
    /// \u{10854}: '𐡔'
    LetterShin,
    /// \u{10855}: '𐡕'
    LetterTaw,
    /// \u{10857}: '𐡗'
    SectionSign,
    /// \u{10858}: '𐡘'
    NumberOne,
    /// \u{10859}: '𐡙'
    NumberTwo,
    /// \u{1085a}: '𐡚'
    NumberThree,
    /// \u{1085b}: '𐡛'
    NumberTen,
    /// \u{1085c}: '𐡜'
    NumberTwenty,
    /// \u{1085d}: '𐡝'
    NumberOneHundred,
    /// \u{1085e}: '𐡞'
    NumberOneThousand,
}

impl Into<char> for ImperialAramaic {
    fn into(self) -> char {
        use constants::*;
        match self {
            ImperialAramaic::LetterAleph => LETTER_ALEPH,
            ImperialAramaic::LetterBeth => LETTER_BETH,
            ImperialAramaic::LetterGimel => LETTER_GIMEL,
            ImperialAramaic::LetterDaleth => LETTER_DALETH,
            ImperialAramaic::LetterHe => LETTER_HE,
            ImperialAramaic::LetterWaw => LETTER_WAW,
            ImperialAramaic::LetterZayin => LETTER_ZAYIN,
            ImperialAramaic::LetterHeth => LETTER_HETH,
            ImperialAramaic::LetterTeth => LETTER_TETH,
            ImperialAramaic::LetterYodh => LETTER_YODH,
            ImperialAramaic::LetterKaph => LETTER_KAPH,
            ImperialAramaic::LetterLamedh => LETTER_LAMEDH,
            ImperialAramaic::LetterMem => LETTER_MEM,
            ImperialAramaic::LetterNun => LETTER_NUN,
            ImperialAramaic::LetterSamekh => LETTER_SAMEKH,
            ImperialAramaic::LetterAyin => LETTER_AYIN,
            ImperialAramaic::LetterPe => LETTER_PE,
            ImperialAramaic::LetterSadhe => LETTER_SADHE,
            ImperialAramaic::LetterQoph => LETTER_QOPH,
            ImperialAramaic::LetterResh => LETTER_RESH,
            ImperialAramaic::LetterShin => LETTER_SHIN,
            ImperialAramaic::LetterTaw => LETTER_TAW,
            ImperialAramaic::SectionSign => SECTION_SIGN,
            ImperialAramaic::NumberOne => NUMBER_ONE,
            ImperialAramaic::NumberTwo => NUMBER_TWO,
            ImperialAramaic::NumberThree => NUMBER_THREE,
            ImperialAramaic::NumberTen => NUMBER_TEN,
            ImperialAramaic::NumberTwenty => NUMBER_TWENTY,
            ImperialAramaic::NumberOneHundred => NUMBER_ONE_HUNDRED,
            ImperialAramaic::NumberOneThousand => NUMBER_ONE_THOUSAND,
        }
    }
}

impl std::convert::TryFrom<char> for ImperialAramaic {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_ALEPH => Ok(ImperialAramaic::LetterAleph),
            LETTER_BETH => Ok(ImperialAramaic::LetterBeth),
            LETTER_GIMEL => Ok(ImperialAramaic::LetterGimel),
            LETTER_DALETH => Ok(ImperialAramaic::LetterDaleth),
            LETTER_HE => Ok(ImperialAramaic::LetterHe),
            LETTER_WAW => Ok(ImperialAramaic::LetterWaw),
            LETTER_ZAYIN => Ok(ImperialAramaic::LetterZayin),
            LETTER_HETH => Ok(ImperialAramaic::LetterHeth),
            LETTER_TETH => Ok(ImperialAramaic::LetterTeth),
            LETTER_YODH => Ok(ImperialAramaic::LetterYodh),
            LETTER_KAPH => Ok(ImperialAramaic::LetterKaph),
            LETTER_LAMEDH => Ok(ImperialAramaic::LetterLamedh),
            LETTER_MEM => Ok(ImperialAramaic::LetterMem),
            LETTER_NUN => Ok(ImperialAramaic::LetterNun),
            LETTER_SAMEKH => Ok(ImperialAramaic::LetterSamekh),
            LETTER_AYIN => Ok(ImperialAramaic::LetterAyin),
            LETTER_PE => Ok(ImperialAramaic::LetterPe),
            LETTER_SADHE => Ok(ImperialAramaic::LetterSadhe),
            LETTER_QOPH => Ok(ImperialAramaic::LetterQoph),
            LETTER_RESH => Ok(ImperialAramaic::LetterResh),
            LETTER_SHIN => Ok(ImperialAramaic::LetterShin),
            LETTER_TAW => Ok(ImperialAramaic::LetterTaw),
            SECTION_SIGN => Ok(ImperialAramaic::SectionSign),
            NUMBER_ONE => Ok(ImperialAramaic::NumberOne),
            NUMBER_TWO => Ok(ImperialAramaic::NumberTwo),
            NUMBER_THREE => Ok(ImperialAramaic::NumberThree),
            NUMBER_TEN => Ok(ImperialAramaic::NumberTen),
            NUMBER_TWENTY => Ok(ImperialAramaic::NumberTwenty),
            NUMBER_ONE_HUNDRED => Ok(ImperialAramaic::NumberOneHundred),
            NUMBER_ONE_THOUSAND => Ok(ImperialAramaic::NumberOneThousand),
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
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        ImperialAramaic::LetterAleph
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            ImperialAramaic::LetterAleph => "imperial aramaic letter aleph",
            ImperialAramaic::LetterBeth => "imperial aramaic letter beth",
            ImperialAramaic::LetterGimel => "imperial aramaic letter gimel",
            ImperialAramaic::LetterDaleth => "imperial aramaic letter daleth",
            ImperialAramaic::LetterHe => "imperial aramaic letter he",
            ImperialAramaic::LetterWaw => "imperial aramaic letter waw",
            ImperialAramaic::LetterZayin => "imperial aramaic letter zayin",
            ImperialAramaic::LetterHeth => "imperial aramaic letter heth",
            ImperialAramaic::LetterTeth => "imperial aramaic letter teth",
            ImperialAramaic::LetterYodh => "imperial aramaic letter yodh",
            ImperialAramaic::LetterKaph => "imperial aramaic letter kaph",
            ImperialAramaic::LetterLamedh => "imperial aramaic letter lamedh",
            ImperialAramaic::LetterMem => "imperial aramaic letter mem",
            ImperialAramaic::LetterNun => "imperial aramaic letter nun",
            ImperialAramaic::LetterSamekh => "imperial aramaic letter samekh",
            ImperialAramaic::LetterAyin => "imperial aramaic letter ayin",
            ImperialAramaic::LetterPe => "imperial aramaic letter pe",
            ImperialAramaic::LetterSadhe => "imperial aramaic letter sadhe",
            ImperialAramaic::LetterQoph => "imperial aramaic letter qoph",
            ImperialAramaic::LetterResh => "imperial aramaic letter resh",
            ImperialAramaic::LetterShin => "imperial aramaic letter shin",
            ImperialAramaic::LetterTaw => "imperial aramaic letter taw",
            ImperialAramaic::SectionSign => "imperial aramaic section sign",
            ImperialAramaic::NumberOne => "imperial aramaic number one",
            ImperialAramaic::NumberTwo => "imperial aramaic number two",
            ImperialAramaic::NumberThree => "imperial aramaic number three",
            ImperialAramaic::NumberTen => "imperial aramaic number ten",
            ImperialAramaic::NumberTwenty => "imperial aramaic number twenty",
            ImperialAramaic::NumberOneHundred => "imperial aramaic number one hundred",
            ImperialAramaic::NumberOneThousand => "imperial aramaic number one thousand",
        }
    }
}
