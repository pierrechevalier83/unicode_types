/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{2de0}: 'ⷠ'
    pub const COMBINING_CYRILLIC_LETTER_BE: char = 'ⷠ';
    /// \u{2de1}: 'ⷡ'
    pub const COMBINING_CYRILLIC_LETTER_VE: char = 'ⷡ';
    /// \u{2de2}: 'ⷢ'
    pub const COMBINING_CYRILLIC_LETTER_GHE: char = 'ⷢ';
    /// \u{2de3}: 'ⷣ'
    pub const COMBINING_CYRILLIC_LETTER_DE: char = 'ⷣ';
    /// \u{2de4}: 'ⷤ'
    pub const COMBINING_CYRILLIC_LETTER_ZHE: char = 'ⷤ';
    /// \u{2de5}: 'ⷥ'
    pub const COMBINING_CYRILLIC_LETTER_ZE: char = 'ⷥ';
    /// \u{2de6}: 'ⷦ'
    pub const COMBINING_CYRILLIC_LETTER_KA: char = 'ⷦ';
    /// \u{2de7}: 'ⷧ'
    pub const COMBINING_CYRILLIC_LETTER_EL: char = 'ⷧ';
    /// \u{2de8}: 'ⷨ'
    pub const COMBINING_CYRILLIC_LETTER_EM: char = 'ⷨ';
    /// \u{2de9}: 'ⷩ'
    pub const COMBINING_CYRILLIC_LETTER_EN: char = 'ⷩ';
    /// \u{2dea}: 'ⷪ'
    pub const COMBINING_CYRILLIC_LETTER_O: char = 'ⷪ';
    /// \u{2deb}: 'ⷫ'
    pub const COMBINING_CYRILLIC_LETTER_PE: char = 'ⷫ';
    /// \u{2dec}: 'ⷬ'
    pub const COMBINING_CYRILLIC_LETTER_ER: char = 'ⷬ';
    /// \u{2ded}: 'ⷭ'
    pub const COMBINING_CYRILLIC_LETTER_ES: char = 'ⷭ';
    /// \u{2dee}: 'ⷮ'
    pub const COMBINING_CYRILLIC_LETTER_TE: char = 'ⷮ';
    /// \u{2def}: 'ⷯ'
    pub const COMBINING_CYRILLIC_LETTER_HA: char = 'ⷯ';
    /// \u{2df0}: 'ⷰ'
    pub const COMBINING_CYRILLIC_LETTER_TSE: char = 'ⷰ';
    /// \u{2df1}: 'ⷱ'
    pub const COMBINING_CYRILLIC_LETTER_CHE: char = 'ⷱ';
    /// \u{2df2}: 'ⷲ'
    pub const COMBINING_CYRILLIC_LETTER_SHA: char = 'ⷲ';
    /// \u{2df3}: 'ⷳ'
    pub const COMBINING_CYRILLIC_LETTER_SHCHA: char = 'ⷳ';
    /// \u{2df4}: 'ⷴ'
    pub const COMBINING_CYRILLIC_LETTER_FITA: char = 'ⷴ';
    /// \u{2df5}: 'ⷵ'
    pub const COMBINING_CYRILLIC_LETTER_ES_DASH_TE: char = 'ⷵ';
    /// \u{2df6}: 'ⷶ'
    pub const COMBINING_CYRILLIC_LETTER_A: char = 'ⷶ';
    /// \u{2df7}: 'ⷷ'
    pub const COMBINING_CYRILLIC_LETTER_IE: char = 'ⷷ';
    /// \u{2df8}: 'ⷸ'
    pub const COMBINING_CYRILLIC_LETTER_DJERV: char = 'ⷸ';
    /// \u{2df9}: 'ⷹ'
    pub const COMBINING_CYRILLIC_LETTER_MONOGRAPH_UK: char = 'ⷹ';
    /// \u{2dfa}: 'ⷺ'
    pub const COMBINING_CYRILLIC_LETTER_YAT: char = 'ⷺ';
    /// \u{2dfb}: 'ⷻ'
    pub const COMBINING_CYRILLIC_LETTER_YU: char = 'ⷻ';
    /// \u{2dfc}: 'ⷼ'
    pub const COMBINING_CYRILLIC_LETTER_IOTIFIED_A: char = 'ⷼ';
    /// \u{2dfd}: 'ⷽ'
    pub const COMBINING_CYRILLIC_LETTER_LITTLE_YUS: char = 'ⷽ';
    /// \u{2dfe}: 'ⷾ'
    pub const COMBINING_CYRILLIC_LETTER_BIG_YUS: char = 'ⷾ';
}

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
        use constants::*;
        match self {
            CyrillicExtendedA::CombiningCyrillicLetterBe => COMBINING_CYRILLIC_LETTER_BE,
            CyrillicExtendedA::CombiningCyrillicLetterVe => COMBINING_CYRILLIC_LETTER_VE,
            CyrillicExtendedA::CombiningCyrillicLetterGhe => COMBINING_CYRILLIC_LETTER_GHE,
            CyrillicExtendedA::CombiningCyrillicLetterDe => COMBINING_CYRILLIC_LETTER_DE,
            CyrillicExtendedA::CombiningCyrillicLetterZhe => COMBINING_CYRILLIC_LETTER_ZHE,
            CyrillicExtendedA::CombiningCyrillicLetterZe => COMBINING_CYRILLIC_LETTER_ZE,
            CyrillicExtendedA::CombiningCyrillicLetterKa => COMBINING_CYRILLIC_LETTER_KA,
            CyrillicExtendedA::CombiningCyrillicLetterEl => COMBINING_CYRILLIC_LETTER_EL,
            CyrillicExtendedA::CombiningCyrillicLetterEm => COMBINING_CYRILLIC_LETTER_EM,
            CyrillicExtendedA::CombiningCyrillicLetterEn => COMBINING_CYRILLIC_LETTER_EN,
            CyrillicExtendedA::CombiningCyrillicLetterO => COMBINING_CYRILLIC_LETTER_O,
            CyrillicExtendedA::CombiningCyrillicLetterPe => COMBINING_CYRILLIC_LETTER_PE,
            CyrillicExtendedA::CombiningCyrillicLetterEr => COMBINING_CYRILLIC_LETTER_ER,
            CyrillicExtendedA::CombiningCyrillicLetterEs => COMBINING_CYRILLIC_LETTER_ES,
            CyrillicExtendedA::CombiningCyrillicLetterTe => COMBINING_CYRILLIC_LETTER_TE,
            CyrillicExtendedA::CombiningCyrillicLetterHa => COMBINING_CYRILLIC_LETTER_HA,
            CyrillicExtendedA::CombiningCyrillicLetterTse => COMBINING_CYRILLIC_LETTER_TSE,
            CyrillicExtendedA::CombiningCyrillicLetterChe => COMBINING_CYRILLIC_LETTER_CHE,
            CyrillicExtendedA::CombiningCyrillicLetterSha => COMBINING_CYRILLIC_LETTER_SHA,
            CyrillicExtendedA::CombiningCyrillicLetterShcha => COMBINING_CYRILLIC_LETTER_SHCHA,
            CyrillicExtendedA::CombiningCyrillicLetterFita => COMBINING_CYRILLIC_LETTER_FITA,
            CyrillicExtendedA::CombiningCyrillicLetterEsDashTe => COMBINING_CYRILLIC_LETTER_ES_DASH_TE,
            CyrillicExtendedA::CombiningCyrillicLetterA => COMBINING_CYRILLIC_LETTER_A,
            CyrillicExtendedA::CombiningCyrillicLetterIe => COMBINING_CYRILLIC_LETTER_IE,
            CyrillicExtendedA::CombiningCyrillicLetterDjerv => COMBINING_CYRILLIC_LETTER_DJERV,
            CyrillicExtendedA::CombiningCyrillicLetterMonographUk => COMBINING_CYRILLIC_LETTER_MONOGRAPH_UK,
            CyrillicExtendedA::CombiningCyrillicLetterYat => COMBINING_CYRILLIC_LETTER_YAT,
            CyrillicExtendedA::CombiningCyrillicLetterYu => COMBINING_CYRILLIC_LETTER_YU,
            CyrillicExtendedA::CombiningCyrillicLetterIotifiedA => COMBINING_CYRILLIC_LETTER_IOTIFIED_A,
            CyrillicExtendedA::CombiningCyrillicLetterLittleYus => COMBINING_CYRILLIC_LETTER_LITTLE_YUS,
            CyrillicExtendedA::CombiningCyrillicLetterBigYus => COMBINING_CYRILLIC_LETTER_BIG_YUS,
        }
    }
}

impl std::convert::TryFrom<char> for CyrillicExtendedA {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            COMBINING_CYRILLIC_LETTER_BE => Ok(CyrillicExtendedA::CombiningCyrillicLetterBe),
            COMBINING_CYRILLIC_LETTER_VE => Ok(CyrillicExtendedA::CombiningCyrillicLetterVe),
            COMBINING_CYRILLIC_LETTER_GHE => Ok(CyrillicExtendedA::CombiningCyrillicLetterGhe),
            COMBINING_CYRILLIC_LETTER_DE => Ok(CyrillicExtendedA::CombiningCyrillicLetterDe),
            COMBINING_CYRILLIC_LETTER_ZHE => Ok(CyrillicExtendedA::CombiningCyrillicLetterZhe),
            COMBINING_CYRILLIC_LETTER_ZE => Ok(CyrillicExtendedA::CombiningCyrillicLetterZe),
            COMBINING_CYRILLIC_LETTER_KA => Ok(CyrillicExtendedA::CombiningCyrillicLetterKa),
            COMBINING_CYRILLIC_LETTER_EL => Ok(CyrillicExtendedA::CombiningCyrillicLetterEl),
            COMBINING_CYRILLIC_LETTER_EM => Ok(CyrillicExtendedA::CombiningCyrillicLetterEm),
            COMBINING_CYRILLIC_LETTER_EN => Ok(CyrillicExtendedA::CombiningCyrillicLetterEn),
            COMBINING_CYRILLIC_LETTER_O => Ok(CyrillicExtendedA::CombiningCyrillicLetterO),
            COMBINING_CYRILLIC_LETTER_PE => Ok(CyrillicExtendedA::CombiningCyrillicLetterPe),
            COMBINING_CYRILLIC_LETTER_ER => Ok(CyrillicExtendedA::CombiningCyrillicLetterEr),
            COMBINING_CYRILLIC_LETTER_ES => Ok(CyrillicExtendedA::CombiningCyrillicLetterEs),
            COMBINING_CYRILLIC_LETTER_TE => Ok(CyrillicExtendedA::CombiningCyrillicLetterTe),
            COMBINING_CYRILLIC_LETTER_HA => Ok(CyrillicExtendedA::CombiningCyrillicLetterHa),
            COMBINING_CYRILLIC_LETTER_TSE => Ok(CyrillicExtendedA::CombiningCyrillicLetterTse),
            COMBINING_CYRILLIC_LETTER_CHE => Ok(CyrillicExtendedA::CombiningCyrillicLetterChe),
            COMBINING_CYRILLIC_LETTER_SHA => Ok(CyrillicExtendedA::CombiningCyrillicLetterSha),
            COMBINING_CYRILLIC_LETTER_SHCHA => Ok(CyrillicExtendedA::CombiningCyrillicLetterShcha),
            COMBINING_CYRILLIC_LETTER_FITA => Ok(CyrillicExtendedA::CombiningCyrillicLetterFita),
            COMBINING_CYRILLIC_LETTER_ES_DASH_TE => Ok(CyrillicExtendedA::CombiningCyrillicLetterEsDashTe),
            COMBINING_CYRILLIC_LETTER_A => Ok(CyrillicExtendedA::CombiningCyrillicLetterA),
            COMBINING_CYRILLIC_LETTER_IE => Ok(CyrillicExtendedA::CombiningCyrillicLetterIe),
            COMBINING_CYRILLIC_LETTER_DJERV => Ok(CyrillicExtendedA::CombiningCyrillicLetterDjerv),
            COMBINING_CYRILLIC_LETTER_MONOGRAPH_UK => Ok(CyrillicExtendedA::CombiningCyrillicLetterMonographUk),
            COMBINING_CYRILLIC_LETTER_YAT => Ok(CyrillicExtendedA::CombiningCyrillicLetterYat),
            COMBINING_CYRILLIC_LETTER_YU => Ok(CyrillicExtendedA::CombiningCyrillicLetterYu),
            COMBINING_CYRILLIC_LETTER_IOTIFIED_A => Ok(CyrillicExtendedA::CombiningCyrillicLetterIotifiedA),
            COMBINING_CYRILLIC_LETTER_LITTLE_YUS => Ok(CyrillicExtendedA::CombiningCyrillicLetterLittleYus),
            COMBINING_CYRILLIC_LETTER_BIG_YUS => Ok(CyrillicExtendedA::CombiningCyrillicLetterBigYus),
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
