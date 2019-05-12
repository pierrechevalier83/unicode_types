/// \u{a9e0} → \u{a9ff}\
///\
/// ꧠ ꧡ ꧢ ꧣ ꧤ ꧥ ꧦ ꧧ ꧨ ꧩ ꧪ ꧫ ꧬ ꧭ ꧮ ꧯ
/// ꧰ ꧱ ꧲ ꧳ ꧴ ꧵ ꧶ ꧷ ꧸ ꧹ ꧺ ꧻ ꧼ ꧽ ꧾ
pub mod constants {
    /// \u{a9e0}: 'ꧠ'
    pub const MYANMAR_LETTER_SHAN_GHA: char = 'ꧠ';
    /// \u{a9e1}: 'ꧡ'
    pub const MYANMAR_LETTER_SHAN_CHA: char = 'ꧡ';
    /// \u{a9e2}: 'ꧢ'
    pub const MYANMAR_LETTER_SHAN_JHA: char = 'ꧢ';
    /// \u{a9e3}: 'ꧣ'
    pub const MYANMAR_LETTER_SHAN_NNA: char = 'ꧣ';
    /// \u{a9e4}: 'ꧤ'
    pub const MYANMAR_LETTER_SHAN_BHA: char = 'ꧤ';
    /// \u{a9e5}: 'ꧥ'
    pub const MYANMAR_SIGN_SHAN_SAW: char = 'ꧥ';
    /// \u{a9e6}: 'ꧦ'
    pub const MYANMAR_MODIFIER_LETTER_SHAN_REDUPLICATION: char = 'ꧦ';
    /// \u{a9e7}: 'ꧧ'
    pub const MYANMAR_LETTER_TAI_LAING_NYA: char = 'ꧧ';
    /// \u{a9e8}: 'ꧨ'
    pub const MYANMAR_LETTER_TAI_LAING_FA: char = 'ꧨ';
    /// \u{a9e9}: 'ꧩ'
    pub const MYANMAR_LETTER_TAI_LAING_GA: char = 'ꧩ';
    /// \u{a9ea}: 'ꧪ'
    pub const MYANMAR_LETTER_TAI_LAING_GHA: char = 'ꧪ';
    /// \u{a9eb}: 'ꧫ'
    pub const MYANMAR_LETTER_TAI_LAING_JA: char = 'ꧫ';
    /// \u{a9ec}: 'ꧬ'
    pub const MYANMAR_LETTER_TAI_LAING_JHA: char = 'ꧬ';
    /// \u{a9ed}: 'ꧭ'
    pub const MYANMAR_LETTER_TAI_LAING_DDA: char = 'ꧭ';
    /// \u{a9ee}: 'ꧮ'
    pub const MYANMAR_LETTER_TAI_LAING_DDHA: char = 'ꧮ';
    /// \u{a9ef}: 'ꧯ'
    pub const MYANMAR_LETTER_TAI_LAING_NNA: char = 'ꧯ';
    /// \u{a9f0}: '꧰'
    pub const MYANMAR_TAI_LAING_DIGIT_ZERO: char = '꧰';
    /// \u{a9f1}: '꧱'
    pub const MYANMAR_TAI_LAING_DIGIT_ONE: char = '꧱';
    /// \u{a9f2}: '꧲'
    pub const MYANMAR_TAI_LAING_DIGIT_TWO: char = '꧲';
    /// \u{a9f3}: '꧳'
    pub const MYANMAR_TAI_LAING_DIGIT_THREE: char = '꧳';
    /// \u{a9f4}: '꧴'
    pub const MYANMAR_TAI_LAING_DIGIT_FOUR: char = '꧴';
    /// \u{a9f5}: '꧵'
    pub const MYANMAR_TAI_LAING_DIGIT_FIVE: char = '꧵';
    /// \u{a9f6}: '꧶'
    pub const MYANMAR_TAI_LAING_DIGIT_SIX: char = '꧶';
    /// \u{a9f7}: '꧷'
    pub const MYANMAR_TAI_LAING_DIGIT_SEVEN: char = '꧷';
    /// \u{a9f8}: '꧸'
    pub const MYANMAR_TAI_LAING_DIGIT_EIGHT: char = '꧸';
    /// \u{a9f9}: '꧹'
    pub const MYANMAR_TAI_LAING_DIGIT_NINE: char = '꧹';
    /// \u{a9fa}: 'ꧺ'
    pub const MYANMAR_LETTER_TAI_LAING_LLA: char = 'ꧺ';
    /// \u{a9fb}: 'ꧻ'
    pub const MYANMAR_LETTER_TAI_LAING_DA: char = 'ꧻ';
    /// \u{a9fc}: 'ꧼ'
    pub const MYANMAR_LETTER_TAI_LAING_DHA: char = 'ꧼ';
    /// \u{a9fd}: 'ꧽ'
    pub const MYANMAR_LETTER_TAI_LAING_BA: char = 'ꧽ';
    /// \u{a9fe}: 'ꧾ'
    pub const MYANMAR_LETTER_TAI_LAING_BHA: char = 'ꧾ';
}

/// \u{a9e0} → \u{a9ff}\
///\
/// ꧠ ꧡ ꧢ ꧣ ꧤ ꧥ ꧦ ꧧ ꧨ ꧩ ꧪ ꧫ ꧬ ꧭ ꧮ ꧯ
/// ꧰ ꧱ ꧲ ꧳ ꧴ ꧵ ꧶ ꧷ ꧸ ꧹ ꧺ ꧻ ꧼ ꧽ ꧾ
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum MyanmarExtendedB {
    /// \u{a9e0}: 'ꧠ'
    MyanmarLetterShanGha,
    /// \u{a9e1}: 'ꧡ'
    MyanmarLetterShanCha,
    /// \u{a9e2}: 'ꧢ'
    MyanmarLetterShanJha,
    /// \u{a9e3}: 'ꧣ'
    MyanmarLetterShanNna,
    /// \u{a9e4}: 'ꧤ'
    MyanmarLetterShanBha,
    /// \u{a9e5}: 'ꧥ'
    MyanmarSignShanSaw,
    /// \u{a9e6}: 'ꧦ'
    MyanmarModifierLetterShanReduplication,
    /// \u{a9e7}: 'ꧧ'
    MyanmarLetterTaiLaingNya,
    /// \u{a9e8}: 'ꧨ'
    MyanmarLetterTaiLaingFa,
    /// \u{a9e9}: 'ꧩ'
    MyanmarLetterTaiLaingGa,
    /// \u{a9ea}: 'ꧪ'
    MyanmarLetterTaiLaingGha,
    /// \u{a9eb}: 'ꧫ'
    MyanmarLetterTaiLaingJa,
    /// \u{a9ec}: 'ꧬ'
    MyanmarLetterTaiLaingJha,
    /// \u{a9ed}: 'ꧭ'
    MyanmarLetterTaiLaingDda,
    /// \u{a9ee}: 'ꧮ'
    MyanmarLetterTaiLaingDdha,
    /// \u{a9ef}: 'ꧯ'
    MyanmarLetterTaiLaingNna,
    /// \u{a9f0}: '꧰'
    MyanmarTaiLaingDigitZero,
    /// \u{a9f1}: '꧱'
    MyanmarTaiLaingDigitOne,
    /// \u{a9f2}: '꧲'
    MyanmarTaiLaingDigitTwo,
    /// \u{a9f3}: '꧳'
    MyanmarTaiLaingDigitThree,
    /// \u{a9f4}: '꧴'
    MyanmarTaiLaingDigitFour,
    /// \u{a9f5}: '꧵'
    MyanmarTaiLaingDigitFive,
    /// \u{a9f6}: '꧶'
    MyanmarTaiLaingDigitSix,
    /// \u{a9f7}: '꧷'
    MyanmarTaiLaingDigitSeven,
    /// \u{a9f8}: '꧸'
    MyanmarTaiLaingDigitEight,
    /// \u{a9f9}: '꧹'
    MyanmarTaiLaingDigitNine,
    /// \u{a9fa}: 'ꧺ'
    MyanmarLetterTaiLaingLla,
    /// \u{a9fb}: 'ꧻ'
    MyanmarLetterTaiLaingDa,
    /// \u{a9fc}: 'ꧼ'
    MyanmarLetterTaiLaingDha,
    /// \u{a9fd}: 'ꧽ'
    MyanmarLetterTaiLaingBa,
    /// \u{a9fe}: 'ꧾ'
    MyanmarLetterTaiLaingBha,
}

impl Into<char> for MyanmarExtendedB {
    fn into(self) -> char {
        use constants::*;
        match self {
            MyanmarExtendedB::MyanmarLetterShanGha => MYANMAR_LETTER_SHAN_GHA,
            MyanmarExtendedB::MyanmarLetterShanCha => MYANMAR_LETTER_SHAN_CHA,
            MyanmarExtendedB::MyanmarLetterShanJha => MYANMAR_LETTER_SHAN_JHA,
            MyanmarExtendedB::MyanmarLetterShanNna => MYANMAR_LETTER_SHAN_NNA,
            MyanmarExtendedB::MyanmarLetterShanBha => MYANMAR_LETTER_SHAN_BHA,
            MyanmarExtendedB::MyanmarSignShanSaw => MYANMAR_SIGN_SHAN_SAW,
            MyanmarExtendedB::MyanmarModifierLetterShanReduplication => MYANMAR_MODIFIER_LETTER_SHAN_REDUPLICATION,
            MyanmarExtendedB::MyanmarLetterTaiLaingNya => MYANMAR_LETTER_TAI_LAING_NYA,
            MyanmarExtendedB::MyanmarLetterTaiLaingFa => MYANMAR_LETTER_TAI_LAING_FA,
            MyanmarExtendedB::MyanmarLetterTaiLaingGa => MYANMAR_LETTER_TAI_LAING_GA,
            MyanmarExtendedB::MyanmarLetterTaiLaingGha => MYANMAR_LETTER_TAI_LAING_GHA,
            MyanmarExtendedB::MyanmarLetterTaiLaingJa => MYANMAR_LETTER_TAI_LAING_JA,
            MyanmarExtendedB::MyanmarLetterTaiLaingJha => MYANMAR_LETTER_TAI_LAING_JHA,
            MyanmarExtendedB::MyanmarLetterTaiLaingDda => MYANMAR_LETTER_TAI_LAING_DDA,
            MyanmarExtendedB::MyanmarLetterTaiLaingDdha => MYANMAR_LETTER_TAI_LAING_DDHA,
            MyanmarExtendedB::MyanmarLetterTaiLaingNna => MYANMAR_LETTER_TAI_LAING_NNA,
            MyanmarExtendedB::MyanmarTaiLaingDigitZero => MYANMAR_TAI_LAING_DIGIT_ZERO,
            MyanmarExtendedB::MyanmarTaiLaingDigitOne => MYANMAR_TAI_LAING_DIGIT_ONE,
            MyanmarExtendedB::MyanmarTaiLaingDigitTwo => MYANMAR_TAI_LAING_DIGIT_TWO,
            MyanmarExtendedB::MyanmarTaiLaingDigitThree => MYANMAR_TAI_LAING_DIGIT_THREE,
            MyanmarExtendedB::MyanmarTaiLaingDigitFour => MYANMAR_TAI_LAING_DIGIT_FOUR,
            MyanmarExtendedB::MyanmarTaiLaingDigitFive => MYANMAR_TAI_LAING_DIGIT_FIVE,
            MyanmarExtendedB::MyanmarTaiLaingDigitSix => MYANMAR_TAI_LAING_DIGIT_SIX,
            MyanmarExtendedB::MyanmarTaiLaingDigitSeven => MYANMAR_TAI_LAING_DIGIT_SEVEN,
            MyanmarExtendedB::MyanmarTaiLaingDigitEight => MYANMAR_TAI_LAING_DIGIT_EIGHT,
            MyanmarExtendedB::MyanmarTaiLaingDigitNine => MYANMAR_TAI_LAING_DIGIT_NINE,
            MyanmarExtendedB::MyanmarLetterTaiLaingLla => MYANMAR_LETTER_TAI_LAING_LLA,
            MyanmarExtendedB::MyanmarLetterTaiLaingDa => MYANMAR_LETTER_TAI_LAING_DA,
            MyanmarExtendedB::MyanmarLetterTaiLaingDha => MYANMAR_LETTER_TAI_LAING_DHA,
            MyanmarExtendedB::MyanmarLetterTaiLaingBa => MYANMAR_LETTER_TAI_LAING_BA,
            MyanmarExtendedB::MyanmarLetterTaiLaingBha => MYANMAR_LETTER_TAI_LAING_BHA,
        }
    }
}

impl std::convert::TryFrom<char> for MyanmarExtendedB {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            MYANMAR_LETTER_SHAN_GHA => Ok(MyanmarExtendedB::MyanmarLetterShanGha),
            MYANMAR_LETTER_SHAN_CHA => Ok(MyanmarExtendedB::MyanmarLetterShanCha),
            MYANMAR_LETTER_SHAN_JHA => Ok(MyanmarExtendedB::MyanmarLetterShanJha),
            MYANMAR_LETTER_SHAN_NNA => Ok(MyanmarExtendedB::MyanmarLetterShanNna),
            MYANMAR_LETTER_SHAN_BHA => Ok(MyanmarExtendedB::MyanmarLetterShanBha),
            MYANMAR_SIGN_SHAN_SAW => Ok(MyanmarExtendedB::MyanmarSignShanSaw),
            MYANMAR_MODIFIER_LETTER_SHAN_REDUPLICATION => Ok(MyanmarExtendedB::MyanmarModifierLetterShanReduplication),
            MYANMAR_LETTER_TAI_LAING_NYA => Ok(MyanmarExtendedB::MyanmarLetterTaiLaingNya),
            MYANMAR_LETTER_TAI_LAING_FA => Ok(MyanmarExtendedB::MyanmarLetterTaiLaingFa),
            MYANMAR_LETTER_TAI_LAING_GA => Ok(MyanmarExtendedB::MyanmarLetterTaiLaingGa),
            MYANMAR_LETTER_TAI_LAING_GHA => Ok(MyanmarExtendedB::MyanmarLetterTaiLaingGha),
            MYANMAR_LETTER_TAI_LAING_JA => Ok(MyanmarExtendedB::MyanmarLetterTaiLaingJa),
            MYANMAR_LETTER_TAI_LAING_JHA => Ok(MyanmarExtendedB::MyanmarLetterTaiLaingJha),
            MYANMAR_LETTER_TAI_LAING_DDA => Ok(MyanmarExtendedB::MyanmarLetterTaiLaingDda),
            MYANMAR_LETTER_TAI_LAING_DDHA => Ok(MyanmarExtendedB::MyanmarLetterTaiLaingDdha),
            MYANMAR_LETTER_TAI_LAING_NNA => Ok(MyanmarExtendedB::MyanmarLetterTaiLaingNna),
            MYANMAR_TAI_LAING_DIGIT_ZERO => Ok(MyanmarExtendedB::MyanmarTaiLaingDigitZero),
            MYANMAR_TAI_LAING_DIGIT_ONE => Ok(MyanmarExtendedB::MyanmarTaiLaingDigitOne),
            MYANMAR_TAI_LAING_DIGIT_TWO => Ok(MyanmarExtendedB::MyanmarTaiLaingDigitTwo),
            MYANMAR_TAI_LAING_DIGIT_THREE => Ok(MyanmarExtendedB::MyanmarTaiLaingDigitThree),
            MYANMAR_TAI_LAING_DIGIT_FOUR => Ok(MyanmarExtendedB::MyanmarTaiLaingDigitFour),
            MYANMAR_TAI_LAING_DIGIT_FIVE => Ok(MyanmarExtendedB::MyanmarTaiLaingDigitFive),
            MYANMAR_TAI_LAING_DIGIT_SIX => Ok(MyanmarExtendedB::MyanmarTaiLaingDigitSix),
            MYANMAR_TAI_LAING_DIGIT_SEVEN => Ok(MyanmarExtendedB::MyanmarTaiLaingDigitSeven),
            MYANMAR_TAI_LAING_DIGIT_EIGHT => Ok(MyanmarExtendedB::MyanmarTaiLaingDigitEight),
            MYANMAR_TAI_LAING_DIGIT_NINE => Ok(MyanmarExtendedB::MyanmarTaiLaingDigitNine),
            MYANMAR_LETTER_TAI_LAING_LLA => Ok(MyanmarExtendedB::MyanmarLetterTaiLaingLla),
            MYANMAR_LETTER_TAI_LAING_DA => Ok(MyanmarExtendedB::MyanmarLetterTaiLaingDa),
            MYANMAR_LETTER_TAI_LAING_DHA => Ok(MyanmarExtendedB::MyanmarLetterTaiLaingDha),
            MYANMAR_LETTER_TAI_LAING_BA => Ok(MyanmarExtendedB::MyanmarLetterTaiLaingBa),
            MYANMAR_LETTER_TAI_LAING_BHA => Ok(MyanmarExtendedB::MyanmarLetterTaiLaingBha),
            _ => Err(()),
        }
    }
}

impl Into<u32> for MyanmarExtendedB {
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

impl std::convert::TryFrom<u32> for MyanmarExtendedB {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for MyanmarExtendedB {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl MyanmarExtendedB {
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        MyanmarExtendedB::MyanmarLetterShanGha
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            MyanmarExtendedB::MyanmarLetterShanGha => "myanmar letter shan gha",
            MyanmarExtendedB::MyanmarLetterShanCha => "myanmar letter shan cha",
            MyanmarExtendedB::MyanmarLetterShanJha => "myanmar letter shan jha",
            MyanmarExtendedB::MyanmarLetterShanNna => "myanmar letter shan nna",
            MyanmarExtendedB::MyanmarLetterShanBha => "myanmar letter shan bha",
            MyanmarExtendedB::MyanmarSignShanSaw => "myanmar sign shan saw",
            MyanmarExtendedB::MyanmarModifierLetterShanReduplication => "myanmar modifier letter shan reduplication",
            MyanmarExtendedB::MyanmarLetterTaiLaingNya => "myanmar letter tai laing nya",
            MyanmarExtendedB::MyanmarLetterTaiLaingFa => "myanmar letter tai laing fa",
            MyanmarExtendedB::MyanmarLetterTaiLaingGa => "myanmar letter tai laing ga",
            MyanmarExtendedB::MyanmarLetterTaiLaingGha => "myanmar letter tai laing gha",
            MyanmarExtendedB::MyanmarLetterTaiLaingJa => "myanmar letter tai laing ja",
            MyanmarExtendedB::MyanmarLetterTaiLaingJha => "myanmar letter tai laing jha",
            MyanmarExtendedB::MyanmarLetterTaiLaingDda => "myanmar letter tai laing dda",
            MyanmarExtendedB::MyanmarLetterTaiLaingDdha => "myanmar letter tai laing ddha",
            MyanmarExtendedB::MyanmarLetterTaiLaingNna => "myanmar letter tai laing nna",
            MyanmarExtendedB::MyanmarTaiLaingDigitZero => "myanmar tai laing digit zero",
            MyanmarExtendedB::MyanmarTaiLaingDigitOne => "myanmar tai laing digit one",
            MyanmarExtendedB::MyanmarTaiLaingDigitTwo => "myanmar tai laing digit two",
            MyanmarExtendedB::MyanmarTaiLaingDigitThree => "myanmar tai laing digit three",
            MyanmarExtendedB::MyanmarTaiLaingDigitFour => "myanmar tai laing digit four",
            MyanmarExtendedB::MyanmarTaiLaingDigitFive => "myanmar tai laing digit five",
            MyanmarExtendedB::MyanmarTaiLaingDigitSix => "myanmar tai laing digit six",
            MyanmarExtendedB::MyanmarTaiLaingDigitSeven => "myanmar tai laing digit seven",
            MyanmarExtendedB::MyanmarTaiLaingDigitEight => "myanmar tai laing digit eight",
            MyanmarExtendedB::MyanmarTaiLaingDigitNine => "myanmar tai laing digit nine",
            MyanmarExtendedB::MyanmarLetterTaiLaingLla => "myanmar letter tai laing lla",
            MyanmarExtendedB::MyanmarLetterTaiLaingDa => "myanmar letter tai laing da",
            MyanmarExtendedB::MyanmarLetterTaiLaingDha => "myanmar letter tai laing dha",
            MyanmarExtendedB::MyanmarLetterTaiLaingBa => "myanmar letter tai laing ba",
            MyanmarExtendedB::MyanmarLetterTaiLaingBha => "myanmar letter tai laing bha",
        }
    }
}
