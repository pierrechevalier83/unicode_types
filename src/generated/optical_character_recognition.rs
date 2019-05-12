/// \u{2440} → \u{245f}\
///\
/// ⑀ ⑁ ⑂ ⑃ ⑄ ⑅ ⑆ ⑇ ⑈ ⑉ ⑊
pub mod constants {
    /// \u{2440}: '⑀'
    pub const OCR_HOOK: char = '⑀';
    /// \u{2441}: '⑁'
    pub const OCR_CHAIR: char = '⑁';
    /// \u{2442}: '⑂'
    pub const OCR_FORK: char = '⑂';
    /// \u{2443}: '⑃'
    pub const OCR_INVERTED_FORK: char = '⑃';
    /// \u{2444}: '⑄'
    pub const OCR_BELT_BUCKLE: char = '⑄';
    /// \u{2445}: '⑅'
    pub const OCR_BOW_TIE: char = '⑅';
    /// \u{2446}: '⑆'
    pub const OCR_BRANCH_BANK_IDENTIFICATION: char = '⑆';
    /// \u{2447}: '⑇'
    pub const OCR_AMOUNT_OF_CHECK: char = '⑇';
    /// \u{2448}: '⑈'
    pub const OCR_DASH: char = '⑈';
    /// \u{2449}: '⑉'
    pub const OCR_CUSTOMER_ACCOUNT_NUMBER: char = '⑉';
    /// \u{244a}: '⑊'
    pub const OCR_DOUBLE_BACKSLASH: char = '⑊';
}

/// \u{2440} → \u{245f}\
///\
/// ⑀ ⑁ ⑂ ⑃ ⑄ ⑅ ⑆ ⑇ ⑈ ⑉ ⑊
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
        use constants::*;
        match self {
            OpticalCharacterRecognition::OcrHook => OCR_HOOK,
            OpticalCharacterRecognition::OcrChair => OCR_CHAIR,
            OpticalCharacterRecognition::OcrFork => OCR_FORK,
            OpticalCharacterRecognition::OcrInvertedFork => OCR_INVERTED_FORK,
            OpticalCharacterRecognition::OcrBeltBuckle => OCR_BELT_BUCKLE,
            OpticalCharacterRecognition::OcrBowTie => OCR_BOW_TIE,
            OpticalCharacterRecognition::OcrBranchBankIdentification => OCR_BRANCH_BANK_IDENTIFICATION,
            OpticalCharacterRecognition::OcrAmountOfCheck => OCR_AMOUNT_OF_CHECK,
            OpticalCharacterRecognition::OcrDash => OCR_DASH,
            OpticalCharacterRecognition::OcrCustomerAccountNumber => OCR_CUSTOMER_ACCOUNT_NUMBER,
            OpticalCharacterRecognition::OcrDoubleBackslash => OCR_DOUBLE_BACKSLASH,
        }
    }
}

impl std::convert::TryFrom<char> for OpticalCharacterRecognition {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            OCR_HOOK => Ok(OpticalCharacterRecognition::OcrHook),
            OCR_CHAIR => Ok(OpticalCharacterRecognition::OcrChair),
            OCR_FORK => Ok(OpticalCharacterRecognition::OcrFork),
            OCR_INVERTED_FORK => Ok(OpticalCharacterRecognition::OcrInvertedFork),
            OCR_BELT_BUCKLE => Ok(OpticalCharacterRecognition::OcrBeltBuckle),
            OCR_BOW_TIE => Ok(OpticalCharacterRecognition::OcrBowTie),
            OCR_BRANCH_BANK_IDENTIFICATION => Ok(OpticalCharacterRecognition::OcrBranchBankIdentification),
            OCR_AMOUNT_OF_CHECK => Ok(OpticalCharacterRecognition::OcrAmountOfCheck),
            OCR_DASH => Ok(OpticalCharacterRecognition::OcrDash),
            OCR_CUSTOMER_ACCOUNT_NUMBER => Ok(OpticalCharacterRecognition::OcrCustomerAccountNumber),
            OCR_DOUBLE_BACKSLASH => Ok(OpticalCharacterRecognition::OcrDoubleBackslash),
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
    /// The character with the lowest index this unicode block
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
