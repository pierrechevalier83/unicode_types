/// \u{110d0} → \u{110ff}\
///\
/// 𑃐 𑃑 𑃒 𑃓 𑃔 𑃕 𑃖 𑃗 𑃘 𑃙 𑃚 𑃛 𑃜 𑃝 𑃞 𑃟\
/// 𑃠 𑃡 𑃢 𑃣 𑃤 𑃥 𑃦 𑃧 𑃨 𑃰 𑃱 𑃲 𑃳 𑃴 𑃵 𑃶\
/// 𑃷 𑃸 𑃹\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{110d0}: '𑃐'
    pub const LETTER_SAH: char = '𑃐';
    /// \u{110d1}: '𑃑'
    pub const LETTER_TAH: char = '𑃑';
    /// \u{110d2}: '𑃒'
    pub const LETTER_BAH: char = '𑃒';
    /// \u{110d3}: '𑃓'
    pub const LETTER_CAH: char = '𑃓';
    /// \u{110d4}: '𑃔'
    pub const LETTER_DAH: char = '𑃔';
    /// \u{110d5}: '𑃕'
    pub const LETTER_GAH: char = '𑃕';
    /// \u{110d6}: '𑃖'
    pub const LETTER_MAH: char = '𑃖';
    /// \u{110d7}: '𑃗'
    pub const LETTER_NGAH: char = '𑃗';
    /// \u{110d8}: '𑃘'
    pub const LETTER_LAH: char = '𑃘';
    /// \u{110d9}: '𑃙'
    pub const LETTER_NAH: char = '𑃙';
    /// \u{110da}: '𑃚'
    pub const LETTER_VAH: char = '𑃚';
    /// \u{110db}: '𑃛'
    pub const LETTER_PAH: char = '𑃛';
    /// \u{110dc}: '𑃜'
    pub const LETTER_YAH: char = '𑃜';
    /// \u{110dd}: '𑃝'
    pub const LETTER_RAH: char = '𑃝';
    /// \u{110de}: '𑃞'
    pub const LETTER_HAH: char = '𑃞';
    /// \u{110df}: '𑃟'
    pub const LETTER_KAH: char = '𑃟';
    /// \u{110e0}: '𑃠'
    pub const LETTER_JAH: char = '𑃠';
    /// \u{110e1}: '𑃡'
    pub const LETTER_NYAH: char = '𑃡';
    /// \u{110e2}: '𑃢'
    pub const LETTER_AH: char = '𑃢';
    /// \u{110e3}: '𑃣'
    pub const LETTER_EEH: char = '𑃣';
    /// \u{110e4}: '𑃤'
    pub const LETTER_IH: char = '𑃤';
    /// \u{110e5}: '𑃥'
    pub const LETTER_UH: char = '𑃥';
    /// \u{110e6}: '𑃦'
    pub const LETTER_OH: char = '𑃦';
    /// \u{110e7}: '𑃧'
    pub const LETTER_EH: char = '𑃧';
    /// \u{110e8}: '𑃨'
    pub const LETTER_MAE: char = '𑃨';
    /// \u{110f0}: '𑃰'
    pub const DIGIT_ZERO: char = '𑃰';
    /// \u{110f1}: '𑃱'
    pub const DIGIT_ONE: char = '𑃱';
    /// \u{110f2}: '𑃲'
    pub const DIGIT_TWO: char = '𑃲';
    /// \u{110f3}: '𑃳'
    pub const DIGIT_THREE: char = '𑃳';
    /// \u{110f4}: '𑃴'
    pub const DIGIT_FOUR: char = '𑃴';
    /// \u{110f5}: '𑃵'
    pub const DIGIT_FIVE: char = '𑃵';
    /// \u{110f6}: '𑃶'
    pub const DIGIT_SIX: char = '𑃶';
    /// \u{110f7}: '𑃷'
    pub const DIGIT_SEVEN: char = '𑃷';
    /// \u{110f8}: '𑃸'
    pub const DIGIT_EIGHT: char = '𑃸';
    /// \u{110f9}: '𑃹'
    pub const DIGIT_NINE: char = '𑃹';
}

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
        use constants::*;
        match self {
            SoraSompeng::LetterSah => LETTER_SAH,
            SoraSompeng::LetterTah => LETTER_TAH,
            SoraSompeng::LetterBah => LETTER_BAH,
            SoraSompeng::LetterCah => LETTER_CAH,
            SoraSompeng::LetterDah => LETTER_DAH,
            SoraSompeng::LetterGah => LETTER_GAH,
            SoraSompeng::LetterMah => LETTER_MAH,
            SoraSompeng::LetterNgah => LETTER_NGAH,
            SoraSompeng::LetterLah => LETTER_LAH,
            SoraSompeng::LetterNah => LETTER_NAH,
            SoraSompeng::LetterVah => LETTER_VAH,
            SoraSompeng::LetterPah => LETTER_PAH,
            SoraSompeng::LetterYah => LETTER_YAH,
            SoraSompeng::LetterRah => LETTER_RAH,
            SoraSompeng::LetterHah => LETTER_HAH,
            SoraSompeng::LetterKah => LETTER_KAH,
            SoraSompeng::LetterJah => LETTER_JAH,
            SoraSompeng::LetterNyah => LETTER_NYAH,
            SoraSompeng::LetterAh => LETTER_AH,
            SoraSompeng::LetterEeh => LETTER_EEH,
            SoraSompeng::LetterIh => LETTER_IH,
            SoraSompeng::LetterUh => LETTER_UH,
            SoraSompeng::LetterOh => LETTER_OH,
            SoraSompeng::LetterEh => LETTER_EH,
            SoraSompeng::LetterMae => LETTER_MAE,
            SoraSompeng::DigitZero => DIGIT_ZERO,
            SoraSompeng::DigitOne => DIGIT_ONE,
            SoraSompeng::DigitTwo => DIGIT_TWO,
            SoraSompeng::DigitThree => DIGIT_THREE,
            SoraSompeng::DigitFour => DIGIT_FOUR,
            SoraSompeng::DigitFive => DIGIT_FIVE,
            SoraSompeng::DigitSix => DIGIT_SIX,
            SoraSompeng::DigitSeven => DIGIT_SEVEN,
            SoraSompeng::DigitEight => DIGIT_EIGHT,
            SoraSompeng::DigitNine => DIGIT_NINE,
        }
    }
}

impl std::convert::TryFrom<char> for SoraSompeng {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_SAH => Ok(SoraSompeng::LetterSah),
            LETTER_TAH => Ok(SoraSompeng::LetterTah),
            LETTER_BAH => Ok(SoraSompeng::LetterBah),
            LETTER_CAH => Ok(SoraSompeng::LetterCah),
            LETTER_DAH => Ok(SoraSompeng::LetterDah),
            LETTER_GAH => Ok(SoraSompeng::LetterGah),
            LETTER_MAH => Ok(SoraSompeng::LetterMah),
            LETTER_NGAH => Ok(SoraSompeng::LetterNgah),
            LETTER_LAH => Ok(SoraSompeng::LetterLah),
            LETTER_NAH => Ok(SoraSompeng::LetterNah),
            LETTER_VAH => Ok(SoraSompeng::LetterVah),
            LETTER_PAH => Ok(SoraSompeng::LetterPah),
            LETTER_YAH => Ok(SoraSompeng::LetterYah),
            LETTER_RAH => Ok(SoraSompeng::LetterRah),
            LETTER_HAH => Ok(SoraSompeng::LetterHah),
            LETTER_KAH => Ok(SoraSompeng::LetterKah),
            LETTER_JAH => Ok(SoraSompeng::LetterJah),
            LETTER_NYAH => Ok(SoraSompeng::LetterNyah),
            LETTER_AH => Ok(SoraSompeng::LetterAh),
            LETTER_EEH => Ok(SoraSompeng::LetterEeh),
            LETTER_IH => Ok(SoraSompeng::LetterIh),
            LETTER_UH => Ok(SoraSompeng::LetterUh),
            LETTER_OH => Ok(SoraSompeng::LetterOh),
            LETTER_EH => Ok(SoraSompeng::LetterEh),
            LETTER_MAE => Ok(SoraSompeng::LetterMae),
            DIGIT_ZERO => Ok(SoraSompeng::DigitZero),
            DIGIT_ONE => Ok(SoraSompeng::DigitOne),
            DIGIT_TWO => Ok(SoraSompeng::DigitTwo),
            DIGIT_THREE => Ok(SoraSompeng::DigitThree),
            DIGIT_FOUR => Ok(SoraSompeng::DigitFour),
            DIGIT_FIVE => Ok(SoraSompeng::DigitFive),
            DIGIT_SIX => Ok(SoraSompeng::DigitSix),
            DIGIT_SEVEN => Ok(SoraSompeng::DigitSeven),
            DIGIT_EIGHT => Ok(SoraSompeng::DigitEight),
            DIGIT_NINE => Ok(SoraSompeng::DigitNine),
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
