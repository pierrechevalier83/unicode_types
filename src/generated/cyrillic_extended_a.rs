
/// An enum to represent all characters in the CyrillicExtendedA block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum CyrillicExtendedA {
    /// \u{2de0}: 'ⷠ'
    CombiningCyrillicLetterBe,
    /// \u{2de1}: 'ⷡ'
    CombiningCyrillicLetterVe,
    /// \u{2de2}: 'ⷢ'
    CombiningCyrillicLetterGhe,
    /// \u{2de3}: 'ⷣ'
    CombiningCyrillicLetterDe,
    /// \u{2de4}: 'ⷤ'
    CombiningCyrillicLetterZhe,
    /// \u{2de5}: 'ⷥ'
    CombiningCyrillicLetterZe,
    /// \u{2de6}: 'ⷦ'
    CombiningCyrillicLetterKa,
    /// \u{2de7}: 'ⷧ'
    CombiningCyrillicLetterEl,
    /// \u{2de8}: 'ⷨ'
    CombiningCyrillicLetterEm,
    /// \u{2de9}: 'ⷩ'
    CombiningCyrillicLetterEn,
    /// \u{2dea}: 'ⷪ'
    CombiningCyrillicLetterO,
    /// \u{2deb}: 'ⷫ'
    CombiningCyrillicLetterPe,
    /// \u{2dec}: 'ⷬ'
    CombiningCyrillicLetterEr,
    /// \u{2ded}: 'ⷭ'
    CombiningCyrillicLetterEs,
    /// \u{2dee}: 'ⷮ'
    CombiningCyrillicLetterTe,
    /// \u{2def}: 'ⷯ'
    CombiningCyrillicLetterHa,
    /// \u{2df0}: 'ⷰ'
    CombiningCyrillicLetterTse,
    /// \u{2df1}: 'ⷱ'
    CombiningCyrillicLetterChe,
    /// \u{2df2}: 'ⷲ'
    CombiningCyrillicLetterSha,
    /// \u{2df3}: 'ⷳ'
    CombiningCyrillicLetterShcha,
    /// \u{2df4}: 'ⷴ'
    CombiningCyrillicLetterFita,
    /// \u{2df5}: 'ⷵ'
    CombiningCyrillicLetterEsDashTe,
    /// \u{2df6}: 'ⷶ'
    CombiningCyrillicLetterA,
    /// \u{2df7}: 'ⷷ'
    CombiningCyrillicLetterIe,
    /// \u{2df8}: 'ⷸ'
    CombiningCyrillicLetterDjerv,
    /// \u{2df9}: 'ⷹ'
    CombiningCyrillicLetterMonographUk,
    /// \u{2dfa}: 'ⷺ'
    CombiningCyrillicLetterYat,
    /// \u{2dfb}: 'ⷻ'
    CombiningCyrillicLetterYu,
    /// \u{2dfc}: 'ⷼ'
    CombiningCyrillicLetterIotifiedA,
    /// \u{2dfd}: 'ⷽ'
    CombiningCyrillicLetterLittleYus,
    /// \u{2dfe}: 'ⷾ'
    CombiningCyrillicLetterBigYus,
}

impl Into<char> for CyrillicExtendedA {
    fn into(self) -> char {
        match self {
            CyrillicExtendedA::CombiningCyrillicLetterBe => 'ⷠ',
            CyrillicExtendedA::CombiningCyrillicLetterVe => 'ⷡ',
            CyrillicExtendedA::CombiningCyrillicLetterGhe => 'ⷢ',
            CyrillicExtendedA::CombiningCyrillicLetterDe => 'ⷣ',
            CyrillicExtendedA::CombiningCyrillicLetterZhe => 'ⷤ',
            CyrillicExtendedA::CombiningCyrillicLetterZe => 'ⷥ',
            CyrillicExtendedA::CombiningCyrillicLetterKa => 'ⷦ',
            CyrillicExtendedA::CombiningCyrillicLetterEl => 'ⷧ',
            CyrillicExtendedA::CombiningCyrillicLetterEm => 'ⷨ',
            CyrillicExtendedA::CombiningCyrillicLetterEn => 'ⷩ',
            CyrillicExtendedA::CombiningCyrillicLetterO => 'ⷪ',
            CyrillicExtendedA::CombiningCyrillicLetterPe => 'ⷫ',
            CyrillicExtendedA::CombiningCyrillicLetterEr => 'ⷬ',
            CyrillicExtendedA::CombiningCyrillicLetterEs => 'ⷭ',
            CyrillicExtendedA::CombiningCyrillicLetterTe => 'ⷮ',
            CyrillicExtendedA::CombiningCyrillicLetterHa => 'ⷯ',
            CyrillicExtendedA::CombiningCyrillicLetterTse => 'ⷰ',
            CyrillicExtendedA::CombiningCyrillicLetterChe => 'ⷱ',
            CyrillicExtendedA::CombiningCyrillicLetterSha => 'ⷲ',
            CyrillicExtendedA::CombiningCyrillicLetterShcha => 'ⷳ',
            CyrillicExtendedA::CombiningCyrillicLetterFita => 'ⷴ',
            CyrillicExtendedA::CombiningCyrillicLetterEsDashTe => 'ⷵ',
            CyrillicExtendedA::CombiningCyrillicLetterA => 'ⷶ',
            CyrillicExtendedA::CombiningCyrillicLetterIe => 'ⷷ',
            CyrillicExtendedA::CombiningCyrillicLetterDjerv => 'ⷸ',
            CyrillicExtendedA::CombiningCyrillicLetterMonographUk => 'ⷹ',
            CyrillicExtendedA::CombiningCyrillicLetterYat => 'ⷺ',
            CyrillicExtendedA::CombiningCyrillicLetterYu => 'ⷻ',
            CyrillicExtendedA::CombiningCyrillicLetterIotifiedA => 'ⷼ',
            CyrillicExtendedA::CombiningCyrillicLetterLittleYus => 'ⷽ',
            CyrillicExtendedA::CombiningCyrillicLetterBigYus => 'ⷾ',
        }
    }
}

impl std::convert::TryFrom<char> for CyrillicExtendedA {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ⷠ' => Ok(CyrillicExtendedA::CombiningCyrillicLetterBe),
            'ⷡ' => Ok(CyrillicExtendedA::CombiningCyrillicLetterVe),
            'ⷢ' => Ok(CyrillicExtendedA::CombiningCyrillicLetterGhe),
            'ⷣ' => Ok(CyrillicExtendedA::CombiningCyrillicLetterDe),
            'ⷤ' => Ok(CyrillicExtendedA::CombiningCyrillicLetterZhe),
            'ⷥ' => Ok(CyrillicExtendedA::CombiningCyrillicLetterZe),
            'ⷦ' => Ok(CyrillicExtendedA::CombiningCyrillicLetterKa),
            'ⷧ' => Ok(CyrillicExtendedA::CombiningCyrillicLetterEl),
            'ⷨ' => Ok(CyrillicExtendedA::CombiningCyrillicLetterEm),
            'ⷩ' => Ok(CyrillicExtendedA::CombiningCyrillicLetterEn),
            'ⷪ' => Ok(CyrillicExtendedA::CombiningCyrillicLetterO),
            'ⷫ' => Ok(CyrillicExtendedA::CombiningCyrillicLetterPe),
            'ⷬ' => Ok(CyrillicExtendedA::CombiningCyrillicLetterEr),
            'ⷭ' => Ok(CyrillicExtendedA::CombiningCyrillicLetterEs),
            'ⷮ' => Ok(CyrillicExtendedA::CombiningCyrillicLetterTe),
            'ⷯ' => Ok(CyrillicExtendedA::CombiningCyrillicLetterHa),
            'ⷰ' => Ok(CyrillicExtendedA::CombiningCyrillicLetterTse),
            'ⷱ' => Ok(CyrillicExtendedA::CombiningCyrillicLetterChe),
            'ⷲ' => Ok(CyrillicExtendedA::CombiningCyrillicLetterSha),
            'ⷳ' => Ok(CyrillicExtendedA::CombiningCyrillicLetterShcha),
            'ⷴ' => Ok(CyrillicExtendedA::CombiningCyrillicLetterFita),
            'ⷵ' => Ok(CyrillicExtendedA::CombiningCyrillicLetterEsDashTe),
            'ⷶ' => Ok(CyrillicExtendedA::CombiningCyrillicLetterA),
            'ⷷ' => Ok(CyrillicExtendedA::CombiningCyrillicLetterIe),
            'ⷸ' => Ok(CyrillicExtendedA::CombiningCyrillicLetterDjerv),
            'ⷹ' => Ok(CyrillicExtendedA::CombiningCyrillicLetterMonographUk),
            'ⷺ' => Ok(CyrillicExtendedA::CombiningCyrillicLetterYat),
            'ⷻ' => Ok(CyrillicExtendedA::CombiningCyrillicLetterYu),
            'ⷼ' => Ok(CyrillicExtendedA::CombiningCyrillicLetterIotifiedA),
            'ⷽ' => Ok(CyrillicExtendedA::CombiningCyrillicLetterLittleYus),
            'ⷾ' => Ok(CyrillicExtendedA::CombiningCyrillicLetterBigYus),
            _ => Err(()),
        }
    }
}

impl Into<u32> for CyrillicExtendedA {
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

impl std::convert::TryFrom<u32> for CyrillicExtendedA {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for CyrillicExtendedA {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl CyrillicExtendedA {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        CyrillicExtendedA::CombiningCyrillicLetterBe
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("CyrillicExtendedA{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
