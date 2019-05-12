
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

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            CyrillicExtendedA::CombiningCyrillicLetterBe => "combining cyrillic letter be",
            CyrillicExtendedA::CombiningCyrillicLetterVe => "combining cyrillic letter ve",
            CyrillicExtendedA::CombiningCyrillicLetterGhe => "combining cyrillic letter ghe",
            CyrillicExtendedA::CombiningCyrillicLetterDe => "combining cyrillic letter de",
            CyrillicExtendedA::CombiningCyrillicLetterZhe => "combining cyrillic letter zhe",
            CyrillicExtendedA::CombiningCyrillicLetterZe => "combining cyrillic letter ze",
            CyrillicExtendedA::CombiningCyrillicLetterKa => "combining cyrillic letter ka",
            CyrillicExtendedA::CombiningCyrillicLetterEl => "combining cyrillic letter el",
            CyrillicExtendedA::CombiningCyrillicLetterEm => "combining cyrillic letter em",
            CyrillicExtendedA::CombiningCyrillicLetterEn => "combining cyrillic letter en",
            CyrillicExtendedA::CombiningCyrillicLetterO => "combining cyrillic letter o",
            CyrillicExtendedA::CombiningCyrillicLetterPe => "combining cyrillic letter pe",
            CyrillicExtendedA::CombiningCyrillicLetterEr => "combining cyrillic letter er",
            CyrillicExtendedA::CombiningCyrillicLetterEs => "combining cyrillic letter es",
            CyrillicExtendedA::CombiningCyrillicLetterTe => "combining cyrillic letter te",
            CyrillicExtendedA::CombiningCyrillicLetterHa => "combining cyrillic letter ha",
            CyrillicExtendedA::CombiningCyrillicLetterTse => "combining cyrillic letter tse",
            CyrillicExtendedA::CombiningCyrillicLetterChe => "combining cyrillic letter che",
            CyrillicExtendedA::CombiningCyrillicLetterSha => "combining cyrillic letter sha",
            CyrillicExtendedA::CombiningCyrillicLetterShcha => "combining cyrillic letter shcha",
            CyrillicExtendedA::CombiningCyrillicLetterFita => "combining cyrillic letter fita",
            CyrillicExtendedA::CombiningCyrillicLetterEsDashTe => "combining cyrillic letter es-te",
            CyrillicExtendedA::CombiningCyrillicLetterA => "combining cyrillic letter a",
            CyrillicExtendedA::CombiningCyrillicLetterIe => "combining cyrillic letter ie",
            CyrillicExtendedA::CombiningCyrillicLetterDjerv => "combining cyrillic letter djerv",
            CyrillicExtendedA::CombiningCyrillicLetterMonographUk => "combining cyrillic letter monograph uk",
            CyrillicExtendedA::CombiningCyrillicLetterYat => "combining cyrillic letter yat",
            CyrillicExtendedA::CombiningCyrillicLetterYu => "combining cyrillic letter yu",
            CyrillicExtendedA::CombiningCyrillicLetterIotifiedA => "combining cyrillic letter iotified a",
            CyrillicExtendedA::CombiningCyrillicLetterLittleYus => "combining cyrillic letter little yus",
            CyrillicExtendedA::CombiningCyrillicLetterBigYus => "combining cyrillic letter big yus",
        }
    }
}
