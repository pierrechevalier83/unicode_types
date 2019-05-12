
/// An enum to represent all characters in the SinhalaArchaicNumbers block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SinhalaArchaicNumbers {
    /// \u{111e1}: '𑇡'
    SinhalaArchaicDigitOne,
    /// \u{111e2}: '𑇢'
    SinhalaArchaicDigitTwo,
    /// \u{111e3}: '𑇣'
    SinhalaArchaicDigitThree,
    /// \u{111e4}: '𑇤'
    SinhalaArchaicDigitFour,
    /// \u{111e5}: '𑇥'
    SinhalaArchaicDigitFive,
    /// \u{111e6}: '𑇦'
    SinhalaArchaicDigitSix,
    /// \u{111e7}: '𑇧'
    SinhalaArchaicDigitSeven,
    /// \u{111e8}: '𑇨'
    SinhalaArchaicDigitEight,
    /// \u{111e9}: '𑇩'
    SinhalaArchaicDigitNine,
    /// \u{111ea}: '𑇪'
    SinhalaArchaicNumberTen,
    /// \u{111eb}: '𑇫'
    SinhalaArchaicNumberTwenty,
    /// \u{111ec}: '𑇬'
    SinhalaArchaicNumberThirty,
    /// \u{111ed}: '𑇭'
    SinhalaArchaicNumberForty,
    /// \u{111ee}: '𑇮'
    SinhalaArchaicNumberFifty,
    /// \u{111ef}: '𑇯'
    SinhalaArchaicNumberSixty,
    /// \u{111f0}: '𑇰'
    SinhalaArchaicNumberSeventy,
    /// \u{111f1}: '𑇱'
    SinhalaArchaicNumberEighty,
    /// \u{111f2}: '𑇲'
    SinhalaArchaicNumberNinety,
    /// \u{111f3}: '𑇳'
    SinhalaArchaicNumberOneHundred,
    /// \u{111f4}: '𑇴'
    SinhalaArchaicNumberOneThousand,
}

impl Into<char> for SinhalaArchaicNumbers {
    fn into(self) -> char {
        match self {
            SinhalaArchaicNumbers::SinhalaArchaicDigitOne => '𑇡',
            SinhalaArchaicNumbers::SinhalaArchaicDigitTwo => '𑇢',
            SinhalaArchaicNumbers::SinhalaArchaicDigitThree => '𑇣',
            SinhalaArchaicNumbers::SinhalaArchaicDigitFour => '𑇤',
            SinhalaArchaicNumbers::SinhalaArchaicDigitFive => '𑇥',
            SinhalaArchaicNumbers::SinhalaArchaicDigitSix => '𑇦',
            SinhalaArchaicNumbers::SinhalaArchaicDigitSeven => '𑇧',
            SinhalaArchaicNumbers::SinhalaArchaicDigitEight => '𑇨',
            SinhalaArchaicNumbers::SinhalaArchaicDigitNine => '𑇩',
            SinhalaArchaicNumbers::SinhalaArchaicNumberTen => '𑇪',
            SinhalaArchaicNumbers::SinhalaArchaicNumberTwenty => '𑇫',
            SinhalaArchaicNumbers::SinhalaArchaicNumberThirty => '𑇬',
            SinhalaArchaicNumbers::SinhalaArchaicNumberForty => '𑇭',
            SinhalaArchaicNumbers::SinhalaArchaicNumberFifty => '𑇮',
            SinhalaArchaicNumbers::SinhalaArchaicNumberSixty => '𑇯',
            SinhalaArchaicNumbers::SinhalaArchaicNumberSeventy => '𑇰',
            SinhalaArchaicNumbers::SinhalaArchaicNumberEighty => '𑇱',
            SinhalaArchaicNumbers::SinhalaArchaicNumberNinety => '𑇲',
            SinhalaArchaicNumbers::SinhalaArchaicNumberOneHundred => '𑇳',
            SinhalaArchaicNumbers::SinhalaArchaicNumberOneThousand => '𑇴',
        }
    }
}

impl std::convert::TryFrom<char> for SinhalaArchaicNumbers {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𑇡' => Ok(SinhalaArchaicNumbers::SinhalaArchaicDigitOne),
            '𑇢' => Ok(SinhalaArchaicNumbers::SinhalaArchaicDigitTwo),
            '𑇣' => Ok(SinhalaArchaicNumbers::SinhalaArchaicDigitThree),
            '𑇤' => Ok(SinhalaArchaicNumbers::SinhalaArchaicDigitFour),
            '𑇥' => Ok(SinhalaArchaicNumbers::SinhalaArchaicDigitFive),
            '𑇦' => Ok(SinhalaArchaicNumbers::SinhalaArchaicDigitSix),
            '𑇧' => Ok(SinhalaArchaicNumbers::SinhalaArchaicDigitSeven),
            '𑇨' => Ok(SinhalaArchaicNumbers::SinhalaArchaicDigitEight),
            '𑇩' => Ok(SinhalaArchaicNumbers::SinhalaArchaicDigitNine),
            '𑇪' => Ok(SinhalaArchaicNumbers::SinhalaArchaicNumberTen),
            '𑇫' => Ok(SinhalaArchaicNumbers::SinhalaArchaicNumberTwenty),
            '𑇬' => Ok(SinhalaArchaicNumbers::SinhalaArchaicNumberThirty),
            '𑇭' => Ok(SinhalaArchaicNumbers::SinhalaArchaicNumberForty),
            '𑇮' => Ok(SinhalaArchaicNumbers::SinhalaArchaicNumberFifty),
            '𑇯' => Ok(SinhalaArchaicNumbers::SinhalaArchaicNumberSixty),
            '𑇰' => Ok(SinhalaArchaicNumbers::SinhalaArchaicNumberSeventy),
            '𑇱' => Ok(SinhalaArchaicNumbers::SinhalaArchaicNumberEighty),
            '𑇲' => Ok(SinhalaArchaicNumbers::SinhalaArchaicNumberNinety),
            '𑇳' => Ok(SinhalaArchaicNumbers::SinhalaArchaicNumberOneHundred),
            '𑇴' => Ok(SinhalaArchaicNumbers::SinhalaArchaicNumberOneThousand),
            _ => Err(()),
        }
    }
}

impl Into<u32> for SinhalaArchaicNumbers {
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

impl std::convert::TryFrom<u32> for SinhalaArchaicNumbers {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for SinhalaArchaicNumbers {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl SinhalaArchaicNumbers {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        SinhalaArchaicNumbers::SinhalaArchaicDigitOne
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("SinhalaArchaicNumbers{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
