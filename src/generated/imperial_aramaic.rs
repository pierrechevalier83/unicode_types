
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
        match self {
            ImperialAramaic::LetterAleph => '𐡀',
            ImperialAramaic::LetterBeth => '𐡁',
            ImperialAramaic::LetterGimel => '𐡂',
            ImperialAramaic::LetterDaleth => '𐡃',
            ImperialAramaic::LetterHe => '𐡄',
            ImperialAramaic::LetterWaw => '𐡅',
            ImperialAramaic::LetterZayin => '𐡆',
            ImperialAramaic::LetterHeth => '𐡇',
            ImperialAramaic::LetterTeth => '𐡈',
            ImperialAramaic::LetterYodh => '𐡉',
            ImperialAramaic::LetterKaph => '𐡊',
            ImperialAramaic::LetterLamedh => '𐡋',
            ImperialAramaic::LetterMem => '𐡌',
            ImperialAramaic::LetterNun => '𐡍',
            ImperialAramaic::LetterSamekh => '𐡎',
            ImperialAramaic::LetterAyin => '𐡏',
            ImperialAramaic::LetterPe => '𐡐',
            ImperialAramaic::LetterSadhe => '𐡑',
            ImperialAramaic::LetterQoph => '𐡒',
            ImperialAramaic::LetterResh => '𐡓',
            ImperialAramaic::LetterShin => '𐡔',
            ImperialAramaic::LetterTaw => '𐡕',
            ImperialAramaic::SectionSign => '𐡗',
            ImperialAramaic::NumberOne => '𐡘',
            ImperialAramaic::NumberTwo => '𐡙',
            ImperialAramaic::NumberThree => '𐡚',
            ImperialAramaic::NumberTen => '𐡛',
            ImperialAramaic::NumberTwenty => '𐡜',
            ImperialAramaic::NumberOneHundred => '𐡝',
            ImperialAramaic::NumberOneThousand => '𐡞',
        }
    }
}

impl std::convert::TryFrom<char> for ImperialAramaic {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𐡀' => Ok(ImperialAramaic::LetterAleph),
            '𐡁' => Ok(ImperialAramaic::LetterBeth),
            '𐡂' => Ok(ImperialAramaic::LetterGimel),
            '𐡃' => Ok(ImperialAramaic::LetterDaleth),
            '𐡄' => Ok(ImperialAramaic::LetterHe),
            '𐡅' => Ok(ImperialAramaic::LetterWaw),
            '𐡆' => Ok(ImperialAramaic::LetterZayin),
            '𐡇' => Ok(ImperialAramaic::LetterHeth),
            '𐡈' => Ok(ImperialAramaic::LetterTeth),
            '𐡉' => Ok(ImperialAramaic::LetterYodh),
            '𐡊' => Ok(ImperialAramaic::LetterKaph),
            '𐡋' => Ok(ImperialAramaic::LetterLamedh),
            '𐡌' => Ok(ImperialAramaic::LetterMem),
            '𐡍' => Ok(ImperialAramaic::LetterNun),
            '𐡎' => Ok(ImperialAramaic::LetterSamekh),
            '𐡏' => Ok(ImperialAramaic::LetterAyin),
            '𐡐' => Ok(ImperialAramaic::LetterPe),
            '𐡑' => Ok(ImperialAramaic::LetterSadhe),
            '𐡒' => Ok(ImperialAramaic::LetterQoph),
            '𐡓' => Ok(ImperialAramaic::LetterResh),
            '𐡔' => Ok(ImperialAramaic::LetterShin),
            '𐡕' => Ok(ImperialAramaic::LetterTaw),
            '𐡗' => Ok(ImperialAramaic::SectionSign),
            '𐡘' => Ok(ImperialAramaic::NumberOne),
            '𐡙' => Ok(ImperialAramaic::NumberTwo),
            '𐡚' => Ok(ImperialAramaic::NumberThree),
            '𐡛' => Ok(ImperialAramaic::NumberTen),
            '𐡜' => Ok(ImperialAramaic::NumberTwenty),
            '𐡝' => Ok(ImperialAramaic::NumberOneHundred),
            '𐡞' => Ok(ImperialAramaic::NumberOneThousand),
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
