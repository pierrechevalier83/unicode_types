
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
        match self {
            PsalterPahlavi::LetterAleph => '𐮀',
            PsalterPahlavi::LetterBeth => '𐮁',
            PsalterPahlavi::LetterGimel => '𐮂',
            PsalterPahlavi::LetterDaleth => '𐮃',
            PsalterPahlavi::LetterHe => '𐮄',
            PsalterPahlavi::LetterWawDashAyinDashResh => '𐮅',
            PsalterPahlavi::LetterZayin => '𐮆',
            PsalterPahlavi::LetterHeth => '𐮇',
            PsalterPahlavi::LetterYodh => '𐮈',
            PsalterPahlavi::LetterKaph => '𐮉',
            PsalterPahlavi::LetterLamedh => '𐮊',
            PsalterPahlavi::LetterMemDashQoph => '𐮋',
            PsalterPahlavi::LetterNun => '𐮌',
            PsalterPahlavi::LetterSamekh => '𐮍',
            PsalterPahlavi::LetterPe => '𐮎',
            PsalterPahlavi::LetterSadhe => '𐮏',
            PsalterPahlavi::LetterShin => '𐮐',
            PsalterPahlavi::LetterTaw => '𐮑',
            PsalterPahlavi::SectionMark => '𐮙',
            PsalterPahlavi::TurnedSectionMark => '𐮚',
            PsalterPahlavi::FourDotsWithCross => '𐮛',
            PsalterPahlavi::FourDotsWithDot => '𐮜',
            PsalterPahlavi::NumberOne => '𐮩',
            PsalterPahlavi::NumberTwo => '𐮪',
            PsalterPahlavi::NumberThree => '𐮫',
            PsalterPahlavi::NumberFour => '𐮬',
            PsalterPahlavi::NumberTen => '𐮭',
            PsalterPahlavi::NumberTwenty => '𐮮',
        }
    }
}

impl std::convert::TryFrom<char> for PsalterPahlavi {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𐮀' => Ok(PsalterPahlavi::LetterAleph),
            '𐮁' => Ok(PsalterPahlavi::LetterBeth),
            '𐮂' => Ok(PsalterPahlavi::LetterGimel),
            '𐮃' => Ok(PsalterPahlavi::LetterDaleth),
            '𐮄' => Ok(PsalterPahlavi::LetterHe),
            '𐮅' => Ok(PsalterPahlavi::LetterWawDashAyinDashResh),
            '𐮆' => Ok(PsalterPahlavi::LetterZayin),
            '𐮇' => Ok(PsalterPahlavi::LetterHeth),
            '𐮈' => Ok(PsalterPahlavi::LetterYodh),
            '𐮉' => Ok(PsalterPahlavi::LetterKaph),
            '𐮊' => Ok(PsalterPahlavi::LetterLamedh),
            '𐮋' => Ok(PsalterPahlavi::LetterMemDashQoph),
            '𐮌' => Ok(PsalterPahlavi::LetterNun),
            '𐮍' => Ok(PsalterPahlavi::LetterSamekh),
            '𐮎' => Ok(PsalterPahlavi::LetterPe),
            '𐮏' => Ok(PsalterPahlavi::LetterSadhe),
            '𐮐' => Ok(PsalterPahlavi::LetterShin),
            '𐮑' => Ok(PsalterPahlavi::LetterTaw),
            '𐮙' => Ok(PsalterPahlavi::SectionMark),
            '𐮚' => Ok(PsalterPahlavi::TurnedSectionMark),
            '𐮛' => Ok(PsalterPahlavi::FourDotsWithCross),
            '𐮜' => Ok(PsalterPahlavi::FourDotsWithDot),
            '𐮩' => Ok(PsalterPahlavi::NumberOne),
            '𐮪' => Ok(PsalterPahlavi::NumberTwo),
            '𐮫' => Ok(PsalterPahlavi::NumberThree),
            '𐮬' => Ok(PsalterPahlavi::NumberFour),
            '𐮭' => Ok(PsalterPahlavi::NumberTen),
            '𐮮' => Ok(PsalterPahlavi::NumberTwenty),
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
