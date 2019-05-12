
/// An enum to represent all characters in the OpticalCharacterRecognition block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum OpticalCharacterRecognition {
    /// \u{2440}: '⑀'
    OcrHook,
    /// \u{2441}: '⑁'
    OcrChair,
    /// \u{2442}: '⑂'
    OcrFork,
    /// \u{2443}: '⑃'
    OcrInvertedFork,
    /// \u{2444}: '⑄'
    OcrBeltBuckle,
    /// \u{2445}: '⑅'
    OcrBowTie,
    /// \u{2446}: '⑆'
    OcrBranchBankIdentification,
    /// \u{2447}: '⑇'
    OcrAmountOfCheck,
    /// \u{2448}: '⑈'
    OcrDash,
    /// \u{2449}: '⑉'
    OcrCustomerAccountNumber,
    /// \u{244a}: '⑊'
    OcrDoubleBackslash,
}

impl Into<char> for OpticalCharacterRecognition {
    fn into(self) -> char {
        match self {
            OpticalCharacterRecognition::OcrHook => '⑀',
            OpticalCharacterRecognition::OcrChair => '⑁',
            OpticalCharacterRecognition::OcrFork => '⑂',
            OpticalCharacterRecognition::OcrInvertedFork => '⑃',
            OpticalCharacterRecognition::OcrBeltBuckle => '⑄',
            OpticalCharacterRecognition::OcrBowTie => '⑅',
            OpticalCharacterRecognition::OcrBranchBankIdentification => '⑆',
            OpticalCharacterRecognition::OcrAmountOfCheck => '⑇',
            OpticalCharacterRecognition::OcrDash => '⑈',
            OpticalCharacterRecognition::OcrCustomerAccountNumber => '⑉',
            OpticalCharacterRecognition::OcrDoubleBackslash => '⑊',
        }
    }
}

impl std::convert::TryFrom<char> for OpticalCharacterRecognition {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '⑀' => Ok(OpticalCharacterRecognition::OcrHook),
            '⑁' => Ok(OpticalCharacterRecognition::OcrChair),
            '⑂' => Ok(OpticalCharacterRecognition::OcrFork),
            '⑃' => Ok(OpticalCharacterRecognition::OcrInvertedFork),
            '⑄' => Ok(OpticalCharacterRecognition::OcrBeltBuckle),
            '⑅' => Ok(OpticalCharacterRecognition::OcrBowTie),
            '⑆' => Ok(OpticalCharacterRecognition::OcrBranchBankIdentification),
            '⑇' => Ok(OpticalCharacterRecognition::OcrAmountOfCheck),
            '⑈' => Ok(OpticalCharacterRecognition::OcrDash),
            '⑉' => Ok(OpticalCharacterRecognition::OcrCustomerAccountNumber),
            '⑊' => Ok(OpticalCharacterRecognition::OcrDoubleBackslash),
            _ => Err(()),
        }
    }
}

impl Into<u32> for OpticalCharacterRecognition {
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

impl std::convert::TryFrom<u32> for OpticalCharacterRecognition {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for OpticalCharacterRecognition {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl OpticalCharacterRecognition {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        OpticalCharacterRecognition::OcrHook
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            OpticalCharacterRecognition::OcrHook => "ocr hook",
            OpticalCharacterRecognition::OcrChair => "ocr chair",
            OpticalCharacterRecognition::OcrFork => "ocr fork",
            OpticalCharacterRecognition::OcrInvertedFork => "ocr inverted fork",
            OpticalCharacterRecognition::OcrBeltBuckle => "ocr belt buckle",
            OpticalCharacterRecognition::OcrBowTie => "ocr bow tie",
            OpticalCharacterRecognition::OcrBranchBankIdentification => "ocr branch bank identification",
            OpticalCharacterRecognition::OcrAmountOfCheck => "ocr amount of check",
            OpticalCharacterRecognition::OcrDash => "ocr dash",
            OpticalCharacterRecognition::OcrCustomerAccountNumber => "ocr customer account number",
            OpticalCharacterRecognition::OcrDoubleBackslash => "ocr double backslash",
        }
    }
}
