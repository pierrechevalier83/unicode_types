/// \u{111e0} → \u{111ff}\
///\
/// 𑇡 𑇢 𑇣 𑇤 𑇥 𑇦 𑇧 𑇨 𑇩 𑇪 𑇫 𑇬 𑇭 𑇮 𑇯 𑇰
/// 𑇱 𑇲 𑇳 𑇴
pub mod constants {
    /// \u{111e1}: '𑇡'
    pub const SINHALA_ARCHAIC_DIGIT_ONE: char = '𑇡';
    /// \u{111e2}: '𑇢'
    pub const SINHALA_ARCHAIC_DIGIT_TWO: char = '𑇢';
    /// \u{111e3}: '𑇣'
    pub const SINHALA_ARCHAIC_DIGIT_THREE: char = '𑇣';
    /// \u{111e4}: '𑇤'
    pub const SINHALA_ARCHAIC_DIGIT_FOUR: char = '𑇤';
    /// \u{111e5}: '𑇥'
    pub const SINHALA_ARCHAIC_DIGIT_FIVE: char = '𑇥';
    /// \u{111e6}: '𑇦'
    pub const SINHALA_ARCHAIC_DIGIT_SIX: char = '𑇦';
    /// \u{111e7}: '𑇧'
    pub const SINHALA_ARCHAIC_DIGIT_SEVEN: char = '𑇧';
    /// \u{111e8}: '𑇨'
    pub const SINHALA_ARCHAIC_DIGIT_EIGHT: char = '𑇨';
    /// \u{111e9}: '𑇩'
    pub const SINHALA_ARCHAIC_DIGIT_NINE: char = '𑇩';
    /// \u{111ea}: '𑇪'
    pub const SINHALA_ARCHAIC_NUMBER_TEN: char = '𑇪';
    /// \u{111eb}: '𑇫'
    pub const SINHALA_ARCHAIC_NUMBER_TWENTY: char = '𑇫';
    /// \u{111ec}: '𑇬'
    pub const SINHALA_ARCHAIC_NUMBER_THIRTY: char = '𑇬';
    /// \u{111ed}: '𑇭'
    pub const SINHALA_ARCHAIC_NUMBER_FORTY: char = '𑇭';
    /// \u{111ee}: '𑇮'
    pub const SINHALA_ARCHAIC_NUMBER_FIFTY: char = '𑇮';
    /// \u{111ef}: '𑇯'
    pub const SINHALA_ARCHAIC_NUMBER_SIXTY: char = '𑇯';
    /// \u{111f0}: '𑇰'
    pub const SINHALA_ARCHAIC_NUMBER_SEVENTY: char = '𑇰';
    /// \u{111f1}: '𑇱'
    pub const SINHALA_ARCHAIC_NUMBER_EIGHTY: char = '𑇱';
    /// \u{111f2}: '𑇲'
    pub const SINHALA_ARCHAIC_NUMBER_NINETY: char = '𑇲';
    /// \u{111f3}: '𑇳'
    pub const SINHALA_ARCHAIC_NUMBER_ONE_HUNDRED: char = '𑇳';
    /// \u{111f4}: '𑇴'
    pub const SINHALA_ARCHAIC_NUMBER_ONE_THOUSAND: char = '𑇴';
}

/// \u{111e0} → \u{111ff}\
///\
/// 𑇡 𑇢 𑇣 𑇤 𑇥 𑇦 𑇧 𑇨 𑇩 𑇪 𑇫 𑇬 𑇭 𑇮 𑇯 𑇰
/// 𑇱 𑇲 𑇳 𑇴
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
        use constants::*;
        match self {
            SinhalaArchaicNumbers::SinhalaArchaicDigitOne => SINHALA_ARCHAIC_DIGIT_ONE,
            SinhalaArchaicNumbers::SinhalaArchaicDigitTwo => SINHALA_ARCHAIC_DIGIT_TWO,
            SinhalaArchaicNumbers::SinhalaArchaicDigitThree => SINHALA_ARCHAIC_DIGIT_THREE,
            SinhalaArchaicNumbers::SinhalaArchaicDigitFour => SINHALA_ARCHAIC_DIGIT_FOUR,
            SinhalaArchaicNumbers::SinhalaArchaicDigitFive => SINHALA_ARCHAIC_DIGIT_FIVE,
            SinhalaArchaicNumbers::SinhalaArchaicDigitSix => SINHALA_ARCHAIC_DIGIT_SIX,
            SinhalaArchaicNumbers::SinhalaArchaicDigitSeven => SINHALA_ARCHAIC_DIGIT_SEVEN,
            SinhalaArchaicNumbers::SinhalaArchaicDigitEight => SINHALA_ARCHAIC_DIGIT_EIGHT,
            SinhalaArchaicNumbers::SinhalaArchaicDigitNine => SINHALA_ARCHAIC_DIGIT_NINE,
            SinhalaArchaicNumbers::SinhalaArchaicNumberTen => SINHALA_ARCHAIC_NUMBER_TEN,
            SinhalaArchaicNumbers::SinhalaArchaicNumberTwenty => SINHALA_ARCHAIC_NUMBER_TWENTY,
            SinhalaArchaicNumbers::SinhalaArchaicNumberThirty => SINHALA_ARCHAIC_NUMBER_THIRTY,
            SinhalaArchaicNumbers::SinhalaArchaicNumberForty => SINHALA_ARCHAIC_NUMBER_FORTY,
            SinhalaArchaicNumbers::SinhalaArchaicNumberFifty => SINHALA_ARCHAIC_NUMBER_FIFTY,
            SinhalaArchaicNumbers::SinhalaArchaicNumberSixty => SINHALA_ARCHAIC_NUMBER_SIXTY,
            SinhalaArchaicNumbers::SinhalaArchaicNumberSeventy => SINHALA_ARCHAIC_NUMBER_SEVENTY,
            SinhalaArchaicNumbers::SinhalaArchaicNumberEighty => SINHALA_ARCHAIC_NUMBER_EIGHTY,
            SinhalaArchaicNumbers::SinhalaArchaicNumberNinety => SINHALA_ARCHAIC_NUMBER_NINETY,
            SinhalaArchaicNumbers::SinhalaArchaicNumberOneHundred => SINHALA_ARCHAIC_NUMBER_ONE_HUNDRED,
            SinhalaArchaicNumbers::SinhalaArchaicNumberOneThousand => SINHALA_ARCHAIC_NUMBER_ONE_THOUSAND,
        }
    }
}

impl std::convert::TryFrom<char> for SinhalaArchaicNumbers {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            SINHALA_ARCHAIC_DIGIT_ONE => Ok(SinhalaArchaicNumbers::SinhalaArchaicDigitOne),
            SINHALA_ARCHAIC_DIGIT_TWO => Ok(SinhalaArchaicNumbers::SinhalaArchaicDigitTwo),
            SINHALA_ARCHAIC_DIGIT_THREE => Ok(SinhalaArchaicNumbers::SinhalaArchaicDigitThree),
            SINHALA_ARCHAIC_DIGIT_FOUR => Ok(SinhalaArchaicNumbers::SinhalaArchaicDigitFour),
            SINHALA_ARCHAIC_DIGIT_FIVE => Ok(SinhalaArchaicNumbers::SinhalaArchaicDigitFive),
            SINHALA_ARCHAIC_DIGIT_SIX => Ok(SinhalaArchaicNumbers::SinhalaArchaicDigitSix),
            SINHALA_ARCHAIC_DIGIT_SEVEN => Ok(SinhalaArchaicNumbers::SinhalaArchaicDigitSeven),
            SINHALA_ARCHAIC_DIGIT_EIGHT => Ok(SinhalaArchaicNumbers::SinhalaArchaicDigitEight),
            SINHALA_ARCHAIC_DIGIT_NINE => Ok(SinhalaArchaicNumbers::SinhalaArchaicDigitNine),
            SINHALA_ARCHAIC_NUMBER_TEN => Ok(SinhalaArchaicNumbers::SinhalaArchaicNumberTen),
            SINHALA_ARCHAIC_NUMBER_TWENTY => Ok(SinhalaArchaicNumbers::SinhalaArchaicNumberTwenty),
            SINHALA_ARCHAIC_NUMBER_THIRTY => Ok(SinhalaArchaicNumbers::SinhalaArchaicNumberThirty),
            SINHALA_ARCHAIC_NUMBER_FORTY => Ok(SinhalaArchaicNumbers::SinhalaArchaicNumberForty),
            SINHALA_ARCHAIC_NUMBER_FIFTY => Ok(SinhalaArchaicNumbers::SinhalaArchaicNumberFifty),
            SINHALA_ARCHAIC_NUMBER_SIXTY => Ok(SinhalaArchaicNumbers::SinhalaArchaicNumberSixty),
            SINHALA_ARCHAIC_NUMBER_SEVENTY => Ok(SinhalaArchaicNumbers::SinhalaArchaicNumberSeventy),
            SINHALA_ARCHAIC_NUMBER_EIGHTY => Ok(SinhalaArchaicNumbers::SinhalaArchaicNumberEighty),
            SINHALA_ARCHAIC_NUMBER_NINETY => Ok(SinhalaArchaicNumbers::SinhalaArchaicNumberNinety),
            SINHALA_ARCHAIC_NUMBER_ONE_HUNDRED => Ok(SinhalaArchaicNumbers::SinhalaArchaicNumberOneHundred),
            SINHALA_ARCHAIC_NUMBER_ONE_THOUSAND => Ok(SinhalaArchaicNumbers::SinhalaArchaicNumberOneThousand),
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
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        SinhalaArchaicNumbers::SinhalaArchaicDigitOne
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            SinhalaArchaicNumbers::SinhalaArchaicDigitOne => "sinhala archaic digit one",
            SinhalaArchaicNumbers::SinhalaArchaicDigitTwo => "sinhala archaic digit two",
            SinhalaArchaicNumbers::SinhalaArchaicDigitThree => "sinhala archaic digit three",
            SinhalaArchaicNumbers::SinhalaArchaicDigitFour => "sinhala archaic digit four",
            SinhalaArchaicNumbers::SinhalaArchaicDigitFive => "sinhala archaic digit five",
            SinhalaArchaicNumbers::SinhalaArchaicDigitSix => "sinhala archaic digit six",
            SinhalaArchaicNumbers::SinhalaArchaicDigitSeven => "sinhala archaic digit seven",
            SinhalaArchaicNumbers::SinhalaArchaicDigitEight => "sinhala archaic digit eight",
            SinhalaArchaicNumbers::SinhalaArchaicDigitNine => "sinhala archaic digit nine",
            SinhalaArchaicNumbers::SinhalaArchaicNumberTen => "sinhala archaic number ten",
            SinhalaArchaicNumbers::SinhalaArchaicNumberTwenty => "sinhala archaic number twenty",
            SinhalaArchaicNumbers::SinhalaArchaicNumberThirty => "sinhala archaic number thirty",
            SinhalaArchaicNumbers::SinhalaArchaicNumberForty => "sinhala archaic number forty",
            SinhalaArchaicNumbers::SinhalaArchaicNumberFifty => "sinhala archaic number fifty",
            SinhalaArchaicNumbers::SinhalaArchaicNumberSixty => "sinhala archaic number sixty",
            SinhalaArchaicNumbers::SinhalaArchaicNumberSeventy => "sinhala archaic number seventy",
            SinhalaArchaicNumbers::SinhalaArchaicNumberEighty => "sinhala archaic number eighty",
            SinhalaArchaicNumbers::SinhalaArchaicNumberNinety => "sinhala archaic number ninety",
            SinhalaArchaicNumbers::SinhalaArchaicNumberOneHundred => "sinhala archaic number one hundred",
            SinhalaArchaicNumbers::SinhalaArchaicNumberOneThousand => "sinhala archaic number one thousand",
        }
    }
}
