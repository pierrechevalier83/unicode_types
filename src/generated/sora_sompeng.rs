
/// An enum to represent all characters in the SoraSompeng block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SoraSompeng {
    /// \u{110d0}: '𑃐'
    LetterSah,
    /// \u{110d1}: '𑃑'
    LetterTah,
    /// \u{110d2}: '𑃒'
    LetterBah,
    /// \u{110d3}: '𑃓'
    LetterCah,
    /// \u{110d4}: '𑃔'
    LetterDah,
    /// \u{110d5}: '𑃕'
    LetterGah,
    /// \u{110d6}: '𑃖'
    LetterMah,
    /// \u{110d7}: '𑃗'
    LetterNgah,
    /// \u{110d8}: '𑃘'
    LetterLah,
    /// \u{110d9}: '𑃙'
    LetterNah,
    /// \u{110da}: '𑃚'
    LetterVah,
    /// \u{110db}: '𑃛'
    LetterPah,
    /// \u{110dc}: '𑃜'
    LetterYah,
    /// \u{110dd}: '𑃝'
    LetterRah,
    /// \u{110de}: '𑃞'
    LetterHah,
    /// \u{110df}: '𑃟'
    LetterKah,
    /// \u{110e0}: '𑃠'
    LetterJah,
    /// \u{110e1}: '𑃡'
    LetterNyah,
    /// \u{110e2}: '𑃢'
    LetterAh,
    /// \u{110e3}: '𑃣'
    LetterEeh,
    /// \u{110e4}: '𑃤'
    LetterIh,
    /// \u{110e5}: '𑃥'
    LetterUh,
    /// \u{110e6}: '𑃦'
    LetterOh,
    /// \u{110e7}: '𑃧'
    LetterEh,
    /// \u{110e8}: '𑃨'
    LetterMae,
    /// \u{110f0}: '𑃰'
    DigitZero,
    /// \u{110f1}: '𑃱'
    DigitOne,
    /// \u{110f2}: '𑃲'
    DigitTwo,
    /// \u{110f3}: '𑃳'
    DigitThree,
    /// \u{110f4}: '𑃴'
    DigitFour,
    /// \u{110f5}: '𑃵'
    DigitFive,
    /// \u{110f6}: '𑃶'
    DigitSix,
    /// \u{110f7}: '𑃷'
    DigitSeven,
    /// \u{110f8}: '𑃸'
    DigitEight,
    /// \u{110f9}: '𑃹'
    DigitNine,
}

impl Into<char> for SoraSompeng {
    fn into(self) -> char {
        match self {
            SoraSompeng::LetterSah => '𑃐',
            SoraSompeng::LetterTah => '𑃑',
            SoraSompeng::LetterBah => '𑃒',
            SoraSompeng::LetterCah => '𑃓',
            SoraSompeng::LetterDah => '𑃔',
            SoraSompeng::LetterGah => '𑃕',
            SoraSompeng::LetterMah => '𑃖',
            SoraSompeng::LetterNgah => '𑃗',
            SoraSompeng::LetterLah => '𑃘',
            SoraSompeng::LetterNah => '𑃙',
            SoraSompeng::LetterVah => '𑃚',
            SoraSompeng::LetterPah => '𑃛',
            SoraSompeng::LetterYah => '𑃜',
            SoraSompeng::LetterRah => '𑃝',
            SoraSompeng::LetterHah => '𑃞',
            SoraSompeng::LetterKah => '𑃟',
            SoraSompeng::LetterJah => '𑃠',
            SoraSompeng::LetterNyah => '𑃡',
            SoraSompeng::LetterAh => '𑃢',
            SoraSompeng::LetterEeh => '𑃣',
            SoraSompeng::LetterIh => '𑃤',
            SoraSompeng::LetterUh => '𑃥',
            SoraSompeng::LetterOh => '𑃦',
            SoraSompeng::LetterEh => '𑃧',
            SoraSompeng::LetterMae => '𑃨',
            SoraSompeng::DigitZero => '𑃰',
            SoraSompeng::DigitOne => '𑃱',
            SoraSompeng::DigitTwo => '𑃲',
            SoraSompeng::DigitThree => '𑃳',
            SoraSompeng::DigitFour => '𑃴',
            SoraSompeng::DigitFive => '𑃵',
            SoraSompeng::DigitSix => '𑃶',
            SoraSompeng::DigitSeven => '𑃷',
            SoraSompeng::DigitEight => '𑃸',
            SoraSompeng::DigitNine => '𑃹',
        }
    }
}

impl std::convert::TryFrom<char> for SoraSompeng {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𑃐' => Ok(SoraSompeng::LetterSah),
            '𑃑' => Ok(SoraSompeng::LetterTah),
            '𑃒' => Ok(SoraSompeng::LetterBah),
            '𑃓' => Ok(SoraSompeng::LetterCah),
            '𑃔' => Ok(SoraSompeng::LetterDah),
            '𑃕' => Ok(SoraSompeng::LetterGah),
            '𑃖' => Ok(SoraSompeng::LetterMah),
            '𑃗' => Ok(SoraSompeng::LetterNgah),
            '𑃘' => Ok(SoraSompeng::LetterLah),
            '𑃙' => Ok(SoraSompeng::LetterNah),
            '𑃚' => Ok(SoraSompeng::LetterVah),
            '𑃛' => Ok(SoraSompeng::LetterPah),
            '𑃜' => Ok(SoraSompeng::LetterYah),
            '𑃝' => Ok(SoraSompeng::LetterRah),
            '𑃞' => Ok(SoraSompeng::LetterHah),
            '𑃟' => Ok(SoraSompeng::LetterKah),
            '𑃠' => Ok(SoraSompeng::LetterJah),
            '𑃡' => Ok(SoraSompeng::LetterNyah),
            '𑃢' => Ok(SoraSompeng::LetterAh),
            '𑃣' => Ok(SoraSompeng::LetterEeh),
            '𑃤' => Ok(SoraSompeng::LetterIh),
            '𑃥' => Ok(SoraSompeng::LetterUh),
            '𑃦' => Ok(SoraSompeng::LetterOh),
            '𑃧' => Ok(SoraSompeng::LetterEh),
            '𑃨' => Ok(SoraSompeng::LetterMae),
            '𑃰' => Ok(SoraSompeng::DigitZero),
            '𑃱' => Ok(SoraSompeng::DigitOne),
            '𑃲' => Ok(SoraSompeng::DigitTwo),
            '𑃳' => Ok(SoraSompeng::DigitThree),
            '𑃴' => Ok(SoraSompeng::DigitFour),
            '𑃵' => Ok(SoraSompeng::DigitFive),
            '𑃶' => Ok(SoraSompeng::DigitSix),
            '𑃷' => Ok(SoraSompeng::DigitSeven),
            '𑃸' => Ok(SoraSompeng::DigitEight),
            '𑃹' => Ok(SoraSompeng::DigitNine),
            _ => Err(()),
        }
    }
}

impl Into<u32> for SoraSompeng {
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

impl std::convert::TryFrom<u32> for SoraSompeng {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for SoraSompeng {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl SoraSompeng {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        SoraSompeng::LetterSah
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            SoraSompeng::LetterSah => "sora sompeng letter sah",
            SoraSompeng::LetterTah => "sora sompeng letter tah",
            SoraSompeng::LetterBah => "sora sompeng letter bah",
            SoraSompeng::LetterCah => "sora sompeng letter cah",
            SoraSompeng::LetterDah => "sora sompeng letter dah",
            SoraSompeng::LetterGah => "sora sompeng letter gah",
            SoraSompeng::LetterMah => "sora sompeng letter mah",
            SoraSompeng::LetterNgah => "sora sompeng letter ngah",
            SoraSompeng::LetterLah => "sora sompeng letter lah",
            SoraSompeng::LetterNah => "sora sompeng letter nah",
            SoraSompeng::LetterVah => "sora sompeng letter vah",
            SoraSompeng::LetterPah => "sora sompeng letter pah",
            SoraSompeng::LetterYah => "sora sompeng letter yah",
            SoraSompeng::LetterRah => "sora sompeng letter rah",
            SoraSompeng::LetterHah => "sora sompeng letter hah",
            SoraSompeng::LetterKah => "sora sompeng letter kah",
            SoraSompeng::LetterJah => "sora sompeng letter jah",
            SoraSompeng::LetterNyah => "sora sompeng letter nyah",
            SoraSompeng::LetterAh => "sora sompeng letter ah",
            SoraSompeng::LetterEeh => "sora sompeng letter eeh",
            SoraSompeng::LetterIh => "sora sompeng letter ih",
            SoraSompeng::LetterUh => "sora sompeng letter uh",
            SoraSompeng::LetterOh => "sora sompeng letter oh",
            SoraSompeng::LetterEh => "sora sompeng letter eh",
            SoraSompeng::LetterMae => "sora sompeng letter mae",
            SoraSompeng::DigitZero => "sora sompeng digit zero",
            SoraSompeng::DigitOne => "sora sompeng digit one",
            SoraSompeng::DigitTwo => "sora sompeng digit two",
            SoraSompeng::DigitThree => "sora sompeng digit three",
            SoraSompeng::DigitFour => "sora sompeng digit four",
            SoraSompeng::DigitFive => "sora sompeng digit five",
            SoraSompeng::DigitSix => "sora sompeng digit six",
            SoraSompeng::DigitSeven => "sora sompeng digit seven",
            SoraSompeng::DigitEight => "sora sompeng digit eight",
            SoraSompeng::DigitNine => "sora sompeng digit nine",
        }
    }
}
