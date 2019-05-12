/// \u{110d0} → \u{110ff}\
///\
/// 𑃐 𑃑 𑃒 𑃓 𑃔 𑃕 𑃖 𑃗 𑃘 𑃙 𑃚 𑃛 𑃜 𑃝 𑃞 𑃟
/// 𑃠 𑃡 𑃢 𑃣 𑃤 𑃥 𑃦 𑃧 𑃨 𑃰 𑃱 𑃲 𑃳 𑃴 𑃵 𑃶
/// 𑃷 𑃸 𑃹
pub mod constants {
    /// \u{110d0}: '𑃐'
    pub const SORA_SOMPENG_LETTER_SAH: char = '𑃐';
    /// \u{110d1}: '𑃑'
    pub const SORA_SOMPENG_LETTER_TAH: char = '𑃑';
    /// \u{110d2}: '𑃒'
    pub const SORA_SOMPENG_LETTER_BAH: char = '𑃒';
    /// \u{110d3}: '𑃓'
    pub const SORA_SOMPENG_LETTER_CAH: char = '𑃓';
    /// \u{110d4}: '𑃔'
    pub const SORA_SOMPENG_LETTER_DAH: char = '𑃔';
    /// \u{110d5}: '𑃕'
    pub const SORA_SOMPENG_LETTER_GAH: char = '𑃕';
    /// \u{110d6}: '𑃖'
    pub const SORA_SOMPENG_LETTER_MAH: char = '𑃖';
    /// \u{110d7}: '𑃗'
    pub const SORA_SOMPENG_LETTER_NGAH: char = '𑃗';
    /// \u{110d8}: '𑃘'
    pub const SORA_SOMPENG_LETTER_LAH: char = '𑃘';
    /// \u{110d9}: '𑃙'
    pub const SORA_SOMPENG_LETTER_NAH: char = '𑃙';
    /// \u{110da}: '𑃚'
    pub const SORA_SOMPENG_LETTER_VAH: char = '𑃚';
    /// \u{110db}: '𑃛'
    pub const SORA_SOMPENG_LETTER_PAH: char = '𑃛';
    /// \u{110dc}: '𑃜'
    pub const SORA_SOMPENG_LETTER_YAH: char = '𑃜';
    /// \u{110dd}: '𑃝'
    pub const SORA_SOMPENG_LETTER_RAH: char = '𑃝';
    /// \u{110de}: '𑃞'
    pub const SORA_SOMPENG_LETTER_HAH: char = '𑃞';
    /// \u{110df}: '𑃟'
    pub const SORA_SOMPENG_LETTER_KAH: char = '𑃟';
    /// \u{110e0}: '𑃠'
    pub const SORA_SOMPENG_LETTER_JAH: char = '𑃠';
    /// \u{110e1}: '𑃡'
    pub const SORA_SOMPENG_LETTER_NYAH: char = '𑃡';
    /// \u{110e2}: '𑃢'
    pub const SORA_SOMPENG_LETTER_AH: char = '𑃢';
    /// \u{110e3}: '𑃣'
    pub const SORA_SOMPENG_LETTER_EEH: char = '𑃣';
    /// \u{110e4}: '𑃤'
    pub const SORA_SOMPENG_LETTER_IH: char = '𑃤';
    /// \u{110e5}: '𑃥'
    pub const SORA_SOMPENG_LETTER_UH: char = '𑃥';
    /// \u{110e6}: '𑃦'
    pub const SORA_SOMPENG_LETTER_OH: char = '𑃦';
    /// \u{110e7}: '𑃧'
    pub const SORA_SOMPENG_LETTER_EH: char = '𑃧';
    /// \u{110e8}: '𑃨'
    pub const SORA_SOMPENG_LETTER_MAE: char = '𑃨';
    /// \u{110f0}: '𑃰'
    pub const SORA_SOMPENG_DIGIT_ZERO: char = '𑃰';
    /// \u{110f1}: '𑃱'
    pub const SORA_SOMPENG_DIGIT_ONE: char = '𑃱';
    /// \u{110f2}: '𑃲'
    pub const SORA_SOMPENG_DIGIT_TWO: char = '𑃲';
    /// \u{110f3}: '𑃳'
    pub const SORA_SOMPENG_DIGIT_THREE: char = '𑃳';
    /// \u{110f4}: '𑃴'
    pub const SORA_SOMPENG_DIGIT_FOUR: char = '𑃴';
    /// \u{110f5}: '𑃵'
    pub const SORA_SOMPENG_DIGIT_FIVE: char = '𑃵';
    /// \u{110f6}: '𑃶'
    pub const SORA_SOMPENG_DIGIT_SIX: char = '𑃶';
    /// \u{110f7}: '𑃷'
    pub const SORA_SOMPENG_DIGIT_SEVEN: char = '𑃷';
    /// \u{110f8}: '𑃸'
    pub const SORA_SOMPENG_DIGIT_EIGHT: char = '𑃸';
    /// \u{110f9}: '𑃹'
    pub const SORA_SOMPENG_DIGIT_NINE: char = '𑃹';
}

/// \u{110d0} → \u{110ff}\
///\
/// 𑃐 𑃑 𑃒 𑃓 𑃔 𑃕 𑃖 𑃗 𑃘 𑃙 𑃚 𑃛 𑃜 𑃝 𑃞 𑃟
/// 𑃠 𑃡 𑃢 𑃣 𑃤 𑃥 𑃦 𑃧 𑃨 𑃰 𑃱 𑃲 𑃳 𑃴 𑃵 𑃶
/// 𑃷 𑃸 𑃹
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SoraSompeng {
    /// \u{110d0}: '𑃐'
    SoraSompengLetterSah,
    /// \u{110d1}: '𑃑'
    SoraSompengLetterTah,
    /// \u{110d2}: '𑃒'
    SoraSompengLetterBah,
    /// \u{110d3}: '𑃓'
    SoraSompengLetterCah,
    /// \u{110d4}: '𑃔'
    SoraSompengLetterDah,
    /// \u{110d5}: '𑃕'
    SoraSompengLetterGah,
    /// \u{110d6}: '𑃖'
    SoraSompengLetterMah,
    /// \u{110d7}: '𑃗'
    SoraSompengLetterNgah,
    /// \u{110d8}: '𑃘'
    SoraSompengLetterLah,
    /// \u{110d9}: '𑃙'
    SoraSompengLetterNah,
    /// \u{110da}: '𑃚'
    SoraSompengLetterVah,
    /// \u{110db}: '𑃛'
    SoraSompengLetterPah,
    /// \u{110dc}: '𑃜'
    SoraSompengLetterYah,
    /// \u{110dd}: '𑃝'
    SoraSompengLetterRah,
    /// \u{110de}: '𑃞'
    SoraSompengLetterHah,
    /// \u{110df}: '𑃟'
    SoraSompengLetterKah,
    /// \u{110e0}: '𑃠'
    SoraSompengLetterJah,
    /// \u{110e1}: '𑃡'
    SoraSompengLetterNyah,
    /// \u{110e2}: '𑃢'
    SoraSompengLetterAh,
    /// \u{110e3}: '𑃣'
    SoraSompengLetterEeh,
    /// \u{110e4}: '𑃤'
    SoraSompengLetterIh,
    /// \u{110e5}: '𑃥'
    SoraSompengLetterUh,
    /// \u{110e6}: '𑃦'
    SoraSompengLetterOh,
    /// \u{110e7}: '𑃧'
    SoraSompengLetterEh,
    /// \u{110e8}: '𑃨'
    SoraSompengLetterMae,
    /// \u{110f0}: '𑃰'
    SoraSompengDigitZero,
    /// \u{110f1}: '𑃱'
    SoraSompengDigitOne,
    /// \u{110f2}: '𑃲'
    SoraSompengDigitTwo,
    /// \u{110f3}: '𑃳'
    SoraSompengDigitThree,
    /// \u{110f4}: '𑃴'
    SoraSompengDigitFour,
    /// \u{110f5}: '𑃵'
    SoraSompengDigitFive,
    /// \u{110f6}: '𑃶'
    SoraSompengDigitSix,
    /// \u{110f7}: '𑃷'
    SoraSompengDigitSeven,
    /// \u{110f8}: '𑃸'
    SoraSompengDigitEight,
    /// \u{110f9}: '𑃹'
    SoraSompengDigitNine,
}

impl Into<char> for SoraSompeng {
    fn into(self) -> char {
        use constants::*;
        match self {
            SoraSompeng::SoraSompengLetterSah => SORA_SOMPENG_LETTER_SAH,
            SoraSompeng::SoraSompengLetterTah => SORA_SOMPENG_LETTER_TAH,
            SoraSompeng::SoraSompengLetterBah => SORA_SOMPENG_LETTER_BAH,
            SoraSompeng::SoraSompengLetterCah => SORA_SOMPENG_LETTER_CAH,
            SoraSompeng::SoraSompengLetterDah => SORA_SOMPENG_LETTER_DAH,
            SoraSompeng::SoraSompengLetterGah => SORA_SOMPENG_LETTER_GAH,
            SoraSompeng::SoraSompengLetterMah => SORA_SOMPENG_LETTER_MAH,
            SoraSompeng::SoraSompengLetterNgah => SORA_SOMPENG_LETTER_NGAH,
            SoraSompeng::SoraSompengLetterLah => SORA_SOMPENG_LETTER_LAH,
            SoraSompeng::SoraSompengLetterNah => SORA_SOMPENG_LETTER_NAH,
            SoraSompeng::SoraSompengLetterVah => SORA_SOMPENG_LETTER_VAH,
            SoraSompeng::SoraSompengLetterPah => SORA_SOMPENG_LETTER_PAH,
            SoraSompeng::SoraSompengLetterYah => SORA_SOMPENG_LETTER_YAH,
            SoraSompeng::SoraSompengLetterRah => SORA_SOMPENG_LETTER_RAH,
            SoraSompeng::SoraSompengLetterHah => SORA_SOMPENG_LETTER_HAH,
            SoraSompeng::SoraSompengLetterKah => SORA_SOMPENG_LETTER_KAH,
            SoraSompeng::SoraSompengLetterJah => SORA_SOMPENG_LETTER_JAH,
            SoraSompeng::SoraSompengLetterNyah => SORA_SOMPENG_LETTER_NYAH,
            SoraSompeng::SoraSompengLetterAh => SORA_SOMPENG_LETTER_AH,
            SoraSompeng::SoraSompengLetterEeh => SORA_SOMPENG_LETTER_EEH,
            SoraSompeng::SoraSompengLetterIh => SORA_SOMPENG_LETTER_IH,
            SoraSompeng::SoraSompengLetterUh => SORA_SOMPENG_LETTER_UH,
            SoraSompeng::SoraSompengLetterOh => SORA_SOMPENG_LETTER_OH,
            SoraSompeng::SoraSompengLetterEh => SORA_SOMPENG_LETTER_EH,
            SoraSompeng::SoraSompengLetterMae => SORA_SOMPENG_LETTER_MAE,
            SoraSompeng::SoraSompengDigitZero => SORA_SOMPENG_DIGIT_ZERO,
            SoraSompeng::SoraSompengDigitOne => SORA_SOMPENG_DIGIT_ONE,
            SoraSompeng::SoraSompengDigitTwo => SORA_SOMPENG_DIGIT_TWO,
            SoraSompeng::SoraSompengDigitThree => SORA_SOMPENG_DIGIT_THREE,
            SoraSompeng::SoraSompengDigitFour => SORA_SOMPENG_DIGIT_FOUR,
            SoraSompeng::SoraSompengDigitFive => SORA_SOMPENG_DIGIT_FIVE,
            SoraSompeng::SoraSompengDigitSix => SORA_SOMPENG_DIGIT_SIX,
            SoraSompeng::SoraSompengDigitSeven => SORA_SOMPENG_DIGIT_SEVEN,
            SoraSompeng::SoraSompengDigitEight => SORA_SOMPENG_DIGIT_EIGHT,
            SoraSompeng::SoraSompengDigitNine => SORA_SOMPENG_DIGIT_NINE,
        }
    }
}

impl std::convert::TryFrom<char> for SoraSompeng {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            SORA_SOMPENG_LETTER_SAH => Ok(SoraSompeng::SoraSompengLetterSah),
            SORA_SOMPENG_LETTER_TAH => Ok(SoraSompeng::SoraSompengLetterTah),
            SORA_SOMPENG_LETTER_BAH => Ok(SoraSompeng::SoraSompengLetterBah),
            SORA_SOMPENG_LETTER_CAH => Ok(SoraSompeng::SoraSompengLetterCah),
            SORA_SOMPENG_LETTER_DAH => Ok(SoraSompeng::SoraSompengLetterDah),
            SORA_SOMPENG_LETTER_GAH => Ok(SoraSompeng::SoraSompengLetterGah),
            SORA_SOMPENG_LETTER_MAH => Ok(SoraSompeng::SoraSompengLetterMah),
            SORA_SOMPENG_LETTER_NGAH => Ok(SoraSompeng::SoraSompengLetterNgah),
            SORA_SOMPENG_LETTER_LAH => Ok(SoraSompeng::SoraSompengLetterLah),
            SORA_SOMPENG_LETTER_NAH => Ok(SoraSompeng::SoraSompengLetterNah),
            SORA_SOMPENG_LETTER_VAH => Ok(SoraSompeng::SoraSompengLetterVah),
            SORA_SOMPENG_LETTER_PAH => Ok(SoraSompeng::SoraSompengLetterPah),
            SORA_SOMPENG_LETTER_YAH => Ok(SoraSompeng::SoraSompengLetterYah),
            SORA_SOMPENG_LETTER_RAH => Ok(SoraSompeng::SoraSompengLetterRah),
            SORA_SOMPENG_LETTER_HAH => Ok(SoraSompeng::SoraSompengLetterHah),
            SORA_SOMPENG_LETTER_KAH => Ok(SoraSompeng::SoraSompengLetterKah),
            SORA_SOMPENG_LETTER_JAH => Ok(SoraSompeng::SoraSompengLetterJah),
            SORA_SOMPENG_LETTER_NYAH => Ok(SoraSompeng::SoraSompengLetterNyah),
            SORA_SOMPENG_LETTER_AH => Ok(SoraSompeng::SoraSompengLetterAh),
            SORA_SOMPENG_LETTER_EEH => Ok(SoraSompeng::SoraSompengLetterEeh),
            SORA_SOMPENG_LETTER_IH => Ok(SoraSompeng::SoraSompengLetterIh),
            SORA_SOMPENG_LETTER_UH => Ok(SoraSompeng::SoraSompengLetterUh),
            SORA_SOMPENG_LETTER_OH => Ok(SoraSompeng::SoraSompengLetterOh),
            SORA_SOMPENG_LETTER_EH => Ok(SoraSompeng::SoraSompengLetterEh),
            SORA_SOMPENG_LETTER_MAE => Ok(SoraSompeng::SoraSompengLetterMae),
            SORA_SOMPENG_DIGIT_ZERO => Ok(SoraSompeng::SoraSompengDigitZero),
            SORA_SOMPENG_DIGIT_ONE => Ok(SoraSompeng::SoraSompengDigitOne),
            SORA_SOMPENG_DIGIT_TWO => Ok(SoraSompeng::SoraSompengDigitTwo),
            SORA_SOMPENG_DIGIT_THREE => Ok(SoraSompeng::SoraSompengDigitThree),
            SORA_SOMPENG_DIGIT_FOUR => Ok(SoraSompeng::SoraSompengDigitFour),
            SORA_SOMPENG_DIGIT_FIVE => Ok(SoraSompeng::SoraSompengDigitFive),
            SORA_SOMPENG_DIGIT_SIX => Ok(SoraSompeng::SoraSompengDigitSix),
            SORA_SOMPENG_DIGIT_SEVEN => Ok(SoraSompeng::SoraSompengDigitSeven),
            SORA_SOMPENG_DIGIT_EIGHT => Ok(SoraSompeng::SoraSompengDigitEight),
            SORA_SOMPENG_DIGIT_NINE => Ok(SoraSompeng::SoraSompengDigitNine),
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
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        SoraSompeng::SoraSompengLetterSah
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            SoraSompeng::SoraSompengLetterSah => "sora sompeng letter sah",
            SoraSompeng::SoraSompengLetterTah => "sora sompeng letter tah",
            SoraSompeng::SoraSompengLetterBah => "sora sompeng letter bah",
            SoraSompeng::SoraSompengLetterCah => "sora sompeng letter cah",
            SoraSompeng::SoraSompengLetterDah => "sora sompeng letter dah",
            SoraSompeng::SoraSompengLetterGah => "sora sompeng letter gah",
            SoraSompeng::SoraSompengLetterMah => "sora sompeng letter mah",
            SoraSompeng::SoraSompengLetterNgah => "sora sompeng letter ngah",
            SoraSompeng::SoraSompengLetterLah => "sora sompeng letter lah",
            SoraSompeng::SoraSompengLetterNah => "sora sompeng letter nah",
            SoraSompeng::SoraSompengLetterVah => "sora sompeng letter vah",
            SoraSompeng::SoraSompengLetterPah => "sora sompeng letter pah",
            SoraSompeng::SoraSompengLetterYah => "sora sompeng letter yah",
            SoraSompeng::SoraSompengLetterRah => "sora sompeng letter rah",
            SoraSompeng::SoraSompengLetterHah => "sora sompeng letter hah",
            SoraSompeng::SoraSompengLetterKah => "sora sompeng letter kah",
            SoraSompeng::SoraSompengLetterJah => "sora sompeng letter jah",
            SoraSompeng::SoraSompengLetterNyah => "sora sompeng letter nyah",
            SoraSompeng::SoraSompengLetterAh => "sora sompeng letter ah",
            SoraSompeng::SoraSompengLetterEeh => "sora sompeng letter eeh",
            SoraSompeng::SoraSompengLetterIh => "sora sompeng letter ih",
            SoraSompeng::SoraSompengLetterUh => "sora sompeng letter uh",
            SoraSompeng::SoraSompengLetterOh => "sora sompeng letter oh",
            SoraSompeng::SoraSompengLetterEh => "sora sompeng letter eh",
            SoraSompeng::SoraSompengLetterMae => "sora sompeng letter mae",
            SoraSompeng::SoraSompengDigitZero => "sora sompeng digit zero",
            SoraSompeng::SoraSompengDigitOne => "sora sompeng digit one",
            SoraSompeng::SoraSompengDigitTwo => "sora sompeng digit two",
            SoraSompeng::SoraSompengDigitThree => "sora sompeng digit three",
            SoraSompeng::SoraSompengDigitFour => "sora sompeng digit four",
            SoraSompeng::SoraSompengDigitFive => "sora sompeng digit five",
            SoraSompeng::SoraSompengDigitSix => "sora sompeng digit six",
            SoraSompeng::SoraSompengDigitSeven => "sora sompeng digit seven",
            SoraSompeng::SoraSompengDigitEight => "sora sompeng digit eight",
            SoraSompeng::SoraSompengDigitNine => "sora sompeng digit nine",
        }
    }
}
