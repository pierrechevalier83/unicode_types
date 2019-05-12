
/// An enum to represent all characters in the CommonIndicNumberForms block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum CommonIndicNumberForms {
    /// \u{a830}: '꠰'
    NorthIndicFractionOneQuarter,
    /// \u{a831}: '꠱'
    NorthIndicFractionOneHalf,
    /// \u{a832}: '꠲'
    NorthIndicFractionThreeQuarters,
    /// \u{a833}: '꠳'
    NorthIndicFractionOneSixteenth,
    /// \u{a834}: '꠴'
    NorthIndicFractionOneEighth,
    /// \u{a835}: '꠵'
    NorthIndicFractionThreeSixteenths,
    /// \u{a836}: '꠶'
    NorthIndicQuarterMark,
    /// \u{a837}: '꠷'
    NorthIndicPlaceholderMark,
    /// \u{a838}: '꠸'
    NorthIndicRupeeMark,
    /// \u{a839}: '꠹'
    NorthIndicQuantityMark,
}

impl Into<char> for CommonIndicNumberForms {
    fn into(self) -> char {
        match self {
            CommonIndicNumberForms::NorthIndicFractionOneQuarter => '꠰',
            CommonIndicNumberForms::NorthIndicFractionOneHalf => '꠱',
            CommonIndicNumberForms::NorthIndicFractionThreeQuarters => '꠲',
            CommonIndicNumberForms::NorthIndicFractionOneSixteenth => '꠳',
            CommonIndicNumberForms::NorthIndicFractionOneEighth => '꠴',
            CommonIndicNumberForms::NorthIndicFractionThreeSixteenths => '꠵',
            CommonIndicNumberForms::NorthIndicQuarterMark => '꠶',
            CommonIndicNumberForms::NorthIndicPlaceholderMark => '꠷',
            CommonIndicNumberForms::NorthIndicRupeeMark => '꠸',
            CommonIndicNumberForms::NorthIndicQuantityMark => '꠹',
        }
    }
}

impl std::convert::TryFrom<char> for CommonIndicNumberForms {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '꠰' => Ok(CommonIndicNumberForms::NorthIndicFractionOneQuarter),
            '꠱' => Ok(CommonIndicNumberForms::NorthIndicFractionOneHalf),
            '꠲' => Ok(CommonIndicNumberForms::NorthIndicFractionThreeQuarters),
            '꠳' => Ok(CommonIndicNumberForms::NorthIndicFractionOneSixteenth),
            '꠴' => Ok(CommonIndicNumberForms::NorthIndicFractionOneEighth),
            '꠵' => Ok(CommonIndicNumberForms::NorthIndicFractionThreeSixteenths),
            '꠶' => Ok(CommonIndicNumberForms::NorthIndicQuarterMark),
            '꠷' => Ok(CommonIndicNumberForms::NorthIndicPlaceholderMark),
            '꠸' => Ok(CommonIndicNumberForms::NorthIndicRupeeMark),
            '꠹' => Ok(CommonIndicNumberForms::NorthIndicQuantityMark),
            _ => Err(()),
        }
    }
}

impl Into<u32> for CommonIndicNumberForms {
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

impl std::convert::TryFrom<u32> for CommonIndicNumberForms {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for CommonIndicNumberForms {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl CommonIndicNumberForms {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        CommonIndicNumberForms::NorthIndicFractionOneQuarter
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("CommonIndicNumberForms{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
