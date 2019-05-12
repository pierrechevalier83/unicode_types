
/// An enum to represent all characters in the InscriptionalPahlavi block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum InscriptionalPahlavi {
    /// \u{10b60}: '𐭠'
    LetterAleph,
    /// \u{10b61}: '𐭡'
    LetterBeth,
    /// \u{10b62}: '𐭢'
    LetterGimel,
    /// \u{10b63}: '𐭣'
    LetterDaleth,
    /// \u{10b64}: '𐭤'
    LetterHe,
    /// \u{10b65}: '𐭥'
    LetterWawDashAyinDashResh,
    /// \u{10b66}: '𐭦'
    LetterZayin,
    /// \u{10b67}: '𐭧'
    LetterHeth,
    /// \u{10b68}: '𐭨'
    LetterTeth,
    /// \u{10b69}: '𐭩'
    LetterYodh,
    /// \u{10b6a}: '𐭪'
    LetterKaph,
    /// \u{10b6b}: '𐭫'
    LetterLamedh,
    /// \u{10b6c}: '𐭬'
    LetterMemDashQoph,
    /// \u{10b6d}: '𐭭'
    LetterNun,
    /// \u{10b6e}: '𐭮'
    LetterSamekh,
    /// \u{10b6f}: '𐭯'
    LetterPe,
    /// \u{10b70}: '𐭰'
    LetterSadhe,
    /// \u{10b71}: '𐭱'
    LetterShin,
    /// \u{10b72}: '𐭲'
    LetterTaw,
    /// \u{10b78}: '𐭸'
    NumberOne,
    /// \u{10b79}: '𐭹'
    NumberTwo,
    /// \u{10b7a}: '𐭺'
    NumberThree,
    /// \u{10b7b}: '𐭻'
    NumberFour,
    /// \u{10b7c}: '𐭼'
    NumberTen,
    /// \u{10b7d}: '𐭽'
    NumberTwenty,
    /// \u{10b7e}: '𐭾'
    NumberOneHundred,
}

impl Into<char> for InscriptionalPahlavi {
    fn into(self) -> char {
        match self {
            InscriptionalPahlavi::LetterAleph => '𐭠',
            InscriptionalPahlavi::LetterBeth => '𐭡',
            InscriptionalPahlavi::LetterGimel => '𐭢',
            InscriptionalPahlavi::LetterDaleth => '𐭣',
            InscriptionalPahlavi::LetterHe => '𐭤',
            InscriptionalPahlavi::LetterWawDashAyinDashResh => '𐭥',
            InscriptionalPahlavi::LetterZayin => '𐭦',
            InscriptionalPahlavi::LetterHeth => '𐭧',
            InscriptionalPahlavi::LetterTeth => '𐭨',
            InscriptionalPahlavi::LetterYodh => '𐭩',
            InscriptionalPahlavi::LetterKaph => '𐭪',
            InscriptionalPahlavi::LetterLamedh => '𐭫',
            InscriptionalPahlavi::LetterMemDashQoph => '𐭬',
            InscriptionalPahlavi::LetterNun => '𐭭',
            InscriptionalPahlavi::LetterSamekh => '𐭮',
            InscriptionalPahlavi::LetterPe => '𐭯',
            InscriptionalPahlavi::LetterSadhe => '𐭰',
            InscriptionalPahlavi::LetterShin => '𐭱',
            InscriptionalPahlavi::LetterTaw => '𐭲',
            InscriptionalPahlavi::NumberOne => '𐭸',
            InscriptionalPahlavi::NumberTwo => '𐭹',
            InscriptionalPahlavi::NumberThree => '𐭺',
            InscriptionalPahlavi::NumberFour => '𐭻',
            InscriptionalPahlavi::NumberTen => '𐭼',
            InscriptionalPahlavi::NumberTwenty => '𐭽',
            InscriptionalPahlavi::NumberOneHundred => '𐭾',
        }
    }
}

impl std::convert::TryFrom<char> for InscriptionalPahlavi {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𐭠' => Ok(InscriptionalPahlavi::LetterAleph),
            '𐭡' => Ok(InscriptionalPahlavi::LetterBeth),
            '𐭢' => Ok(InscriptionalPahlavi::LetterGimel),
            '𐭣' => Ok(InscriptionalPahlavi::LetterDaleth),
            '𐭤' => Ok(InscriptionalPahlavi::LetterHe),
            '𐭥' => Ok(InscriptionalPahlavi::LetterWawDashAyinDashResh),
            '𐭦' => Ok(InscriptionalPahlavi::LetterZayin),
            '𐭧' => Ok(InscriptionalPahlavi::LetterHeth),
            '𐭨' => Ok(InscriptionalPahlavi::LetterTeth),
            '𐭩' => Ok(InscriptionalPahlavi::LetterYodh),
            '𐭪' => Ok(InscriptionalPahlavi::LetterKaph),
            '𐭫' => Ok(InscriptionalPahlavi::LetterLamedh),
            '𐭬' => Ok(InscriptionalPahlavi::LetterMemDashQoph),
            '𐭭' => Ok(InscriptionalPahlavi::LetterNun),
            '𐭮' => Ok(InscriptionalPahlavi::LetterSamekh),
            '𐭯' => Ok(InscriptionalPahlavi::LetterPe),
            '𐭰' => Ok(InscriptionalPahlavi::LetterSadhe),
            '𐭱' => Ok(InscriptionalPahlavi::LetterShin),
            '𐭲' => Ok(InscriptionalPahlavi::LetterTaw),
            '𐭸' => Ok(InscriptionalPahlavi::NumberOne),
            '𐭹' => Ok(InscriptionalPahlavi::NumberTwo),
            '𐭺' => Ok(InscriptionalPahlavi::NumberThree),
            '𐭻' => Ok(InscriptionalPahlavi::NumberFour),
            '𐭼' => Ok(InscriptionalPahlavi::NumberTen),
            '𐭽' => Ok(InscriptionalPahlavi::NumberTwenty),
            '𐭾' => Ok(InscriptionalPahlavi::NumberOneHundred),
            _ => Err(()),
        }
    }
}

impl Into<u32> for InscriptionalPahlavi {
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

impl std::convert::TryFrom<u32> for InscriptionalPahlavi {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for InscriptionalPahlavi {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl InscriptionalPahlavi {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        InscriptionalPahlavi::LetterAleph
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            InscriptionalPahlavi::LetterAleph => "inscriptional pahlavi letter aleph",
            InscriptionalPahlavi::LetterBeth => "inscriptional pahlavi letter beth",
            InscriptionalPahlavi::LetterGimel => "inscriptional pahlavi letter gimel",
            InscriptionalPahlavi::LetterDaleth => "inscriptional pahlavi letter daleth",
            InscriptionalPahlavi::LetterHe => "inscriptional pahlavi letter he",
            InscriptionalPahlavi::LetterWawDashAyinDashResh => "inscriptional pahlavi letter waw-ayin-resh",
            InscriptionalPahlavi::LetterZayin => "inscriptional pahlavi letter zayin",
            InscriptionalPahlavi::LetterHeth => "inscriptional pahlavi letter heth",
            InscriptionalPahlavi::LetterTeth => "inscriptional pahlavi letter teth",
            InscriptionalPahlavi::LetterYodh => "inscriptional pahlavi letter yodh",
            InscriptionalPahlavi::LetterKaph => "inscriptional pahlavi letter kaph",
            InscriptionalPahlavi::LetterLamedh => "inscriptional pahlavi letter lamedh",
            InscriptionalPahlavi::LetterMemDashQoph => "inscriptional pahlavi letter mem-qoph",
            InscriptionalPahlavi::LetterNun => "inscriptional pahlavi letter nun",
            InscriptionalPahlavi::LetterSamekh => "inscriptional pahlavi letter samekh",
            InscriptionalPahlavi::LetterPe => "inscriptional pahlavi letter pe",
            InscriptionalPahlavi::LetterSadhe => "inscriptional pahlavi letter sadhe",
            InscriptionalPahlavi::LetterShin => "inscriptional pahlavi letter shin",
            InscriptionalPahlavi::LetterTaw => "inscriptional pahlavi letter taw",
            InscriptionalPahlavi::NumberOne => "inscriptional pahlavi number one",
            InscriptionalPahlavi::NumberTwo => "inscriptional pahlavi number two",
            InscriptionalPahlavi::NumberThree => "inscriptional pahlavi number three",
            InscriptionalPahlavi::NumberFour => "inscriptional pahlavi number four",
            InscriptionalPahlavi::NumberTen => "inscriptional pahlavi number ten",
            InscriptionalPahlavi::NumberTwenty => "inscriptional pahlavi number twenty",
            InscriptionalPahlavi::NumberOneHundred => "inscriptional pahlavi number one hundred",
        }
    }
}
