
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
        match self {
            InscriptionalParthian::LetterAleph => '𐭀',
            InscriptionalParthian::LetterBeth => '𐭁',
            InscriptionalParthian::LetterGimel => '𐭂',
            InscriptionalParthian::LetterDaleth => '𐭃',
            InscriptionalParthian::LetterHe => '𐭄',
            InscriptionalParthian::LetterWaw => '𐭅',
            InscriptionalParthian::LetterZayin => '𐭆',
            InscriptionalParthian::LetterHeth => '𐭇',
            InscriptionalParthian::LetterTeth => '𐭈',
            InscriptionalParthian::LetterYodh => '𐭉',
            InscriptionalParthian::LetterKaph => '𐭊',
            InscriptionalParthian::LetterLamedh => '𐭋',
            InscriptionalParthian::LetterMem => '𐭌',
            InscriptionalParthian::LetterNun => '𐭍',
            InscriptionalParthian::LetterSamekh => '𐭎',
            InscriptionalParthian::LetterAyin => '𐭏',
            InscriptionalParthian::LetterPe => '𐭐',
            InscriptionalParthian::LetterSadhe => '𐭑',
            InscriptionalParthian::LetterQoph => '𐭒',
            InscriptionalParthian::LetterResh => '𐭓',
            InscriptionalParthian::LetterShin => '𐭔',
            InscriptionalParthian::LetterTaw => '𐭕',
            InscriptionalParthian::NumberOne => '𐭘',
            InscriptionalParthian::NumberTwo => '𐭙',
            InscriptionalParthian::NumberThree => '𐭚',
            InscriptionalParthian::NumberFour => '𐭛',
            InscriptionalParthian::NumberTen => '𐭜',
            InscriptionalParthian::NumberTwenty => '𐭝',
            InscriptionalParthian::NumberOneHundred => '𐭞',
        }
    }
}

impl std::convert::TryFrom<char> for InscriptionalParthian {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𐭀' => Ok(InscriptionalParthian::LetterAleph),
            '𐭁' => Ok(InscriptionalParthian::LetterBeth),
            '𐭂' => Ok(InscriptionalParthian::LetterGimel),
            '𐭃' => Ok(InscriptionalParthian::LetterDaleth),
            '𐭄' => Ok(InscriptionalParthian::LetterHe),
            '𐭅' => Ok(InscriptionalParthian::LetterWaw),
            '𐭆' => Ok(InscriptionalParthian::LetterZayin),
            '𐭇' => Ok(InscriptionalParthian::LetterHeth),
            '𐭈' => Ok(InscriptionalParthian::LetterTeth),
            '𐭉' => Ok(InscriptionalParthian::LetterYodh),
            '𐭊' => Ok(InscriptionalParthian::LetterKaph),
            '𐭋' => Ok(InscriptionalParthian::LetterLamedh),
            '𐭌' => Ok(InscriptionalParthian::LetterMem),
            '𐭍' => Ok(InscriptionalParthian::LetterNun),
            '𐭎' => Ok(InscriptionalParthian::LetterSamekh),
            '𐭏' => Ok(InscriptionalParthian::LetterAyin),
            '𐭐' => Ok(InscriptionalParthian::LetterPe),
            '𐭑' => Ok(InscriptionalParthian::LetterSadhe),
            '𐭒' => Ok(InscriptionalParthian::LetterQoph),
            '𐭓' => Ok(InscriptionalParthian::LetterResh),
            '𐭔' => Ok(InscriptionalParthian::LetterShin),
            '𐭕' => Ok(InscriptionalParthian::LetterTaw),
            '𐭘' => Ok(InscriptionalParthian::NumberOne),
            '𐭙' => Ok(InscriptionalParthian::NumberTwo),
            '𐭚' => Ok(InscriptionalParthian::NumberThree),
            '𐭛' => Ok(InscriptionalParthian::NumberFour),
            '𐭜' => Ok(InscriptionalParthian::NumberTen),
            '𐭝' => Ok(InscriptionalParthian::NumberTwenty),
            '𐭞' => Ok(InscriptionalParthian::NumberOneHundred),
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
