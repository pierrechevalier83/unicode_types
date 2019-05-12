
/// An enum to represent all characters in the CountingRodNumerals block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum CountingRodNumerals {
    /// \u{1d360}: '𝍠'
    CountingRodUnitDigitOne,
    /// \u{1d361}: '𝍡'
    CountingRodUnitDigitTwo,
    /// \u{1d362}: '𝍢'
    CountingRodUnitDigitThree,
    /// \u{1d363}: '𝍣'
    CountingRodUnitDigitFour,
    /// \u{1d364}: '𝍤'
    CountingRodUnitDigitFive,
    /// \u{1d365}: '𝍥'
    CountingRodUnitDigitSix,
    /// \u{1d366}: '𝍦'
    CountingRodUnitDigitSeven,
    /// \u{1d367}: '𝍧'
    CountingRodUnitDigitEight,
    /// \u{1d368}: '𝍨'
    CountingRodUnitDigitNine,
    /// \u{1d369}: '𝍩'
    CountingRodTensDigitOne,
    /// \u{1d36a}: '𝍪'
    CountingRodTensDigitTwo,
    /// \u{1d36b}: '𝍫'
    CountingRodTensDigitThree,
    /// \u{1d36c}: '𝍬'
    CountingRodTensDigitFour,
    /// \u{1d36d}: '𝍭'
    CountingRodTensDigitFive,
    /// \u{1d36e}: '𝍮'
    CountingRodTensDigitSix,
    /// \u{1d36f}: '𝍯'
    CountingRodTensDigitSeven,
    /// \u{1d370}: '𝍰'
    CountingRodTensDigitEight,
    /// \u{1d371}: '𝍱'
    CountingRodTensDigitNine,
    /// \u{1d372}: '𝍲'
    IdeographicTallyMarkOne,
    /// \u{1d373}: '𝍳'
    IdeographicTallyMarkTwo,
    /// \u{1d374}: '𝍴'
    IdeographicTallyMarkThree,
    /// \u{1d375}: '𝍵'
    IdeographicTallyMarkFour,
    /// \u{1d376}: '𝍶'
    IdeographicTallyMarkFive,
    /// \u{1d377}: '𝍷'
    TallyMarkOne,
    /// \u{1d378}: '𝍸'
    TallyMarkFive,
}

impl Into<char> for CountingRodNumerals {
    fn into(self) -> char {
        match self {
            CountingRodNumerals::CountingRodUnitDigitOne => '𝍠',
            CountingRodNumerals::CountingRodUnitDigitTwo => '𝍡',
            CountingRodNumerals::CountingRodUnitDigitThree => '𝍢',
            CountingRodNumerals::CountingRodUnitDigitFour => '𝍣',
            CountingRodNumerals::CountingRodUnitDigitFive => '𝍤',
            CountingRodNumerals::CountingRodUnitDigitSix => '𝍥',
            CountingRodNumerals::CountingRodUnitDigitSeven => '𝍦',
            CountingRodNumerals::CountingRodUnitDigitEight => '𝍧',
            CountingRodNumerals::CountingRodUnitDigitNine => '𝍨',
            CountingRodNumerals::CountingRodTensDigitOne => '𝍩',
            CountingRodNumerals::CountingRodTensDigitTwo => '𝍪',
            CountingRodNumerals::CountingRodTensDigitThree => '𝍫',
            CountingRodNumerals::CountingRodTensDigitFour => '𝍬',
            CountingRodNumerals::CountingRodTensDigitFive => '𝍭',
            CountingRodNumerals::CountingRodTensDigitSix => '𝍮',
            CountingRodNumerals::CountingRodTensDigitSeven => '𝍯',
            CountingRodNumerals::CountingRodTensDigitEight => '𝍰',
            CountingRodNumerals::CountingRodTensDigitNine => '𝍱',
            CountingRodNumerals::IdeographicTallyMarkOne => '𝍲',
            CountingRodNumerals::IdeographicTallyMarkTwo => '𝍳',
            CountingRodNumerals::IdeographicTallyMarkThree => '𝍴',
            CountingRodNumerals::IdeographicTallyMarkFour => '𝍵',
            CountingRodNumerals::IdeographicTallyMarkFive => '𝍶',
            CountingRodNumerals::TallyMarkOne => '𝍷',
            CountingRodNumerals::TallyMarkFive => '𝍸',
        }
    }
}

impl std::convert::TryFrom<char> for CountingRodNumerals {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𝍠' => Ok(CountingRodNumerals::CountingRodUnitDigitOne),
            '𝍡' => Ok(CountingRodNumerals::CountingRodUnitDigitTwo),
            '𝍢' => Ok(CountingRodNumerals::CountingRodUnitDigitThree),
            '𝍣' => Ok(CountingRodNumerals::CountingRodUnitDigitFour),
            '𝍤' => Ok(CountingRodNumerals::CountingRodUnitDigitFive),
            '𝍥' => Ok(CountingRodNumerals::CountingRodUnitDigitSix),
            '𝍦' => Ok(CountingRodNumerals::CountingRodUnitDigitSeven),
            '𝍧' => Ok(CountingRodNumerals::CountingRodUnitDigitEight),
            '𝍨' => Ok(CountingRodNumerals::CountingRodUnitDigitNine),
            '𝍩' => Ok(CountingRodNumerals::CountingRodTensDigitOne),
            '𝍪' => Ok(CountingRodNumerals::CountingRodTensDigitTwo),
            '𝍫' => Ok(CountingRodNumerals::CountingRodTensDigitThree),
            '𝍬' => Ok(CountingRodNumerals::CountingRodTensDigitFour),
            '𝍭' => Ok(CountingRodNumerals::CountingRodTensDigitFive),
            '𝍮' => Ok(CountingRodNumerals::CountingRodTensDigitSix),
            '𝍯' => Ok(CountingRodNumerals::CountingRodTensDigitSeven),
            '𝍰' => Ok(CountingRodNumerals::CountingRodTensDigitEight),
            '𝍱' => Ok(CountingRodNumerals::CountingRodTensDigitNine),
            '𝍲' => Ok(CountingRodNumerals::IdeographicTallyMarkOne),
            '𝍳' => Ok(CountingRodNumerals::IdeographicTallyMarkTwo),
            '𝍴' => Ok(CountingRodNumerals::IdeographicTallyMarkThree),
            '𝍵' => Ok(CountingRodNumerals::IdeographicTallyMarkFour),
            '𝍶' => Ok(CountingRodNumerals::IdeographicTallyMarkFive),
            '𝍷' => Ok(CountingRodNumerals::TallyMarkOne),
            '𝍸' => Ok(CountingRodNumerals::TallyMarkFive),
            _ => Err(()),
        }
    }
}

impl Into<u32> for CountingRodNumerals {
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

impl std::convert::TryFrom<u32> for CountingRodNumerals {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for CountingRodNumerals {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl CountingRodNumerals {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        CountingRodNumerals::CountingRodUnitDigitOne
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("CountingRodNumerals{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
