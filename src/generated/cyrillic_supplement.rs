
/// An enum to represent all characters in the CyrillicSupplement block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum CyrillicSupplement {
    /// \u{500}: 'Ԁ'
    CyrillicCapitalLetterKomiDe,
    /// \u{501}: 'ԁ'
    CyrillicSmallLetterKomiDe,
    /// \u{502}: 'Ԃ'
    CyrillicCapitalLetterKomiDje,
    /// \u{503}: 'ԃ'
    CyrillicSmallLetterKomiDje,
    /// \u{504}: 'Ԅ'
    CyrillicCapitalLetterKomiZje,
    /// \u{505}: 'ԅ'
    CyrillicSmallLetterKomiZje,
    /// \u{506}: 'Ԇ'
    CyrillicCapitalLetterKomiDzje,
    /// \u{507}: 'ԇ'
    CyrillicSmallLetterKomiDzje,
    /// \u{508}: 'Ԉ'
    CyrillicCapitalLetterKomiLje,
    /// \u{509}: 'ԉ'
    CyrillicSmallLetterKomiLje,
    /// \u{50a}: 'Ԋ'
    CyrillicCapitalLetterKomiNje,
    /// \u{50b}: 'ԋ'
    CyrillicSmallLetterKomiNje,
    /// \u{50c}: 'Ԍ'
    CyrillicCapitalLetterKomiSje,
    /// \u{50d}: 'ԍ'
    CyrillicSmallLetterKomiSje,
    /// \u{50e}: 'Ԏ'
    CyrillicCapitalLetterKomiTje,
    /// \u{50f}: 'ԏ'
    CyrillicSmallLetterKomiTje,
    /// \u{510}: 'Ԑ'
    CyrillicCapitalLetterReversedZe,
    /// \u{511}: 'ԑ'
    CyrillicSmallLetterReversedZe,
    /// \u{512}: 'Ԓ'
    CyrillicCapitalLetterElWithHook,
    /// \u{513}: 'ԓ'
    CyrillicSmallLetterElWithHook,
    /// \u{514}: 'Ԕ'
    CyrillicCapitalLetterLha,
    /// \u{515}: 'ԕ'
    CyrillicSmallLetterLha,
    /// \u{516}: 'Ԗ'
    CyrillicCapitalLetterRha,
    /// \u{517}: 'ԗ'
    CyrillicSmallLetterRha,
    /// \u{518}: 'Ԙ'
    CyrillicCapitalLetterYae,
    /// \u{519}: 'ԙ'
    CyrillicSmallLetterYae,
    /// \u{51a}: 'Ԛ'
    CyrillicCapitalLetterQa,
    /// \u{51b}: 'ԛ'
    CyrillicSmallLetterQa,
    /// \u{51c}: 'Ԝ'
    CyrillicCapitalLetterWe,
    /// \u{51d}: 'ԝ'
    CyrillicSmallLetterWe,
    /// \u{51e}: 'Ԟ'
    CyrillicCapitalLetterAleutKa,
    /// \u{51f}: 'ԟ'
    CyrillicSmallLetterAleutKa,
    /// \u{520}: 'Ԡ'
    CyrillicCapitalLetterElWithMiddleHook,
    /// \u{521}: 'ԡ'
    CyrillicSmallLetterElWithMiddleHook,
    /// \u{522}: 'Ԣ'
    CyrillicCapitalLetterEnWithMiddleHook,
    /// \u{523}: 'ԣ'
    CyrillicSmallLetterEnWithMiddleHook,
    /// \u{524}: 'Ԥ'
    CyrillicCapitalLetterPeWithDescender,
    /// \u{525}: 'ԥ'
    CyrillicSmallLetterPeWithDescender,
    /// \u{526}: 'Ԧ'
    CyrillicCapitalLetterShhaWithDescender,
    /// \u{527}: 'ԧ'
    CyrillicSmallLetterShhaWithDescender,
    /// \u{528}: 'Ԩ'
    CyrillicCapitalLetterEnWithLeftHook,
    /// \u{529}: 'ԩ'
    CyrillicSmallLetterEnWithLeftHook,
    /// \u{52a}: 'Ԫ'
    CyrillicCapitalLetterDzzhe,
    /// \u{52b}: 'ԫ'
    CyrillicSmallLetterDzzhe,
    /// \u{52c}: 'Ԭ'
    CyrillicCapitalLetterDche,
    /// \u{52d}: 'ԭ'
    CyrillicSmallLetterDche,
    /// \u{52e}: 'Ԯ'
    CyrillicCapitalLetterElWithDescender,
}

impl Into<char> for CyrillicSupplement {
    fn into(self) -> char {
        match self {
            CyrillicSupplement::CyrillicCapitalLetterKomiDe => 'Ԁ',
            CyrillicSupplement::CyrillicSmallLetterKomiDe => 'ԁ',
            CyrillicSupplement::CyrillicCapitalLetterKomiDje => 'Ԃ',
            CyrillicSupplement::CyrillicSmallLetterKomiDje => 'ԃ',
            CyrillicSupplement::CyrillicCapitalLetterKomiZje => 'Ԅ',
            CyrillicSupplement::CyrillicSmallLetterKomiZje => 'ԅ',
            CyrillicSupplement::CyrillicCapitalLetterKomiDzje => 'Ԇ',
            CyrillicSupplement::CyrillicSmallLetterKomiDzje => 'ԇ',
            CyrillicSupplement::CyrillicCapitalLetterKomiLje => 'Ԉ',
            CyrillicSupplement::CyrillicSmallLetterKomiLje => 'ԉ',
            CyrillicSupplement::CyrillicCapitalLetterKomiNje => 'Ԋ',
            CyrillicSupplement::CyrillicSmallLetterKomiNje => 'ԋ',
            CyrillicSupplement::CyrillicCapitalLetterKomiSje => 'Ԍ',
            CyrillicSupplement::CyrillicSmallLetterKomiSje => 'ԍ',
            CyrillicSupplement::CyrillicCapitalLetterKomiTje => 'Ԏ',
            CyrillicSupplement::CyrillicSmallLetterKomiTje => 'ԏ',
            CyrillicSupplement::CyrillicCapitalLetterReversedZe => 'Ԑ',
            CyrillicSupplement::CyrillicSmallLetterReversedZe => 'ԑ',
            CyrillicSupplement::CyrillicCapitalLetterElWithHook => 'Ԓ',
            CyrillicSupplement::CyrillicSmallLetterElWithHook => 'ԓ',
            CyrillicSupplement::CyrillicCapitalLetterLha => 'Ԕ',
            CyrillicSupplement::CyrillicSmallLetterLha => 'ԕ',
            CyrillicSupplement::CyrillicCapitalLetterRha => 'Ԗ',
            CyrillicSupplement::CyrillicSmallLetterRha => 'ԗ',
            CyrillicSupplement::CyrillicCapitalLetterYae => 'Ԙ',
            CyrillicSupplement::CyrillicSmallLetterYae => 'ԙ',
            CyrillicSupplement::CyrillicCapitalLetterQa => 'Ԛ',
            CyrillicSupplement::CyrillicSmallLetterQa => 'ԛ',
            CyrillicSupplement::CyrillicCapitalLetterWe => 'Ԝ',
            CyrillicSupplement::CyrillicSmallLetterWe => 'ԝ',
            CyrillicSupplement::CyrillicCapitalLetterAleutKa => 'Ԟ',
            CyrillicSupplement::CyrillicSmallLetterAleutKa => 'ԟ',
            CyrillicSupplement::CyrillicCapitalLetterElWithMiddleHook => 'Ԡ',
            CyrillicSupplement::CyrillicSmallLetterElWithMiddleHook => 'ԡ',
            CyrillicSupplement::CyrillicCapitalLetterEnWithMiddleHook => 'Ԣ',
            CyrillicSupplement::CyrillicSmallLetterEnWithMiddleHook => 'ԣ',
            CyrillicSupplement::CyrillicCapitalLetterPeWithDescender => 'Ԥ',
            CyrillicSupplement::CyrillicSmallLetterPeWithDescender => 'ԥ',
            CyrillicSupplement::CyrillicCapitalLetterShhaWithDescender => 'Ԧ',
            CyrillicSupplement::CyrillicSmallLetterShhaWithDescender => 'ԧ',
            CyrillicSupplement::CyrillicCapitalLetterEnWithLeftHook => 'Ԩ',
            CyrillicSupplement::CyrillicSmallLetterEnWithLeftHook => 'ԩ',
            CyrillicSupplement::CyrillicCapitalLetterDzzhe => 'Ԫ',
            CyrillicSupplement::CyrillicSmallLetterDzzhe => 'ԫ',
            CyrillicSupplement::CyrillicCapitalLetterDche => 'Ԭ',
            CyrillicSupplement::CyrillicSmallLetterDche => 'ԭ',
            CyrillicSupplement::CyrillicCapitalLetterElWithDescender => 'Ԯ',
        }
    }
}

impl std::convert::TryFrom<char> for CyrillicSupplement {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'Ԁ' => Ok(CyrillicSupplement::CyrillicCapitalLetterKomiDe),
            'ԁ' => Ok(CyrillicSupplement::CyrillicSmallLetterKomiDe),
            'Ԃ' => Ok(CyrillicSupplement::CyrillicCapitalLetterKomiDje),
            'ԃ' => Ok(CyrillicSupplement::CyrillicSmallLetterKomiDje),
            'Ԅ' => Ok(CyrillicSupplement::CyrillicCapitalLetterKomiZje),
            'ԅ' => Ok(CyrillicSupplement::CyrillicSmallLetterKomiZje),
            'Ԇ' => Ok(CyrillicSupplement::CyrillicCapitalLetterKomiDzje),
            'ԇ' => Ok(CyrillicSupplement::CyrillicSmallLetterKomiDzje),
            'Ԉ' => Ok(CyrillicSupplement::CyrillicCapitalLetterKomiLje),
            'ԉ' => Ok(CyrillicSupplement::CyrillicSmallLetterKomiLje),
            'Ԋ' => Ok(CyrillicSupplement::CyrillicCapitalLetterKomiNje),
            'ԋ' => Ok(CyrillicSupplement::CyrillicSmallLetterKomiNje),
            'Ԍ' => Ok(CyrillicSupplement::CyrillicCapitalLetterKomiSje),
            'ԍ' => Ok(CyrillicSupplement::CyrillicSmallLetterKomiSje),
            'Ԏ' => Ok(CyrillicSupplement::CyrillicCapitalLetterKomiTje),
            'ԏ' => Ok(CyrillicSupplement::CyrillicSmallLetterKomiTje),
            'Ԑ' => Ok(CyrillicSupplement::CyrillicCapitalLetterReversedZe),
            'ԑ' => Ok(CyrillicSupplement::CyrillicSmallLetterReversedZe),
            'Ԓ' => Ok(CyrillicSupplement::CyrillicCapitalLetterElWithHook),
            'ԓ' => Ok(CyrillicSupplement::CyrillicSmallLetterElWithHook),
            'Ԕ' => Ok(CyrillicSupplement::CyrillicCapitalLetterLha),
            'ԕ' => Ok(CyrillicSupplement::CyrillicSmallLetterLha),
            'Ԗ' => Ok(CyrillicSupplement::CyrillicCapitalLetterRha),
            'ԗ' => Ok(CyrillicSupplement::CyrillicSmallLetterRha),
            'Ԙ' => Ok(CyrillicSupplement::CyrillicCapitalLetterYae),
            'ԙ' => Ok(CyrillicSupplement::CyrillicSmallLetterYae),
            'Ԛ' => Ok(CyrillicSupplement::CyrillicCapitalLetterQa),
            'ԛ' => Ok(CyrillicSupplement::CyrillicSmallLetterQa),
            'Ԝ' => Ok(CyrillicSupplement::CyrillicCapitalLetterWe),
            'ԝ' => Ok(CyrillicSupplement::CyrillicSmallLetterWe),
            'Ԟ' => Ok(CyrillicSupplement::CyrillicCapitalLetterAleutKa),
            'ԟ' => Ok(CyrillicSupplement::CyrillicSmallLetterAleutKa),
            'Ԡ' => Ok(CyrillicSupplement::CyrillicCapitalLetterElWithMiddleHook),
            'ԡ' => Ok(CyrillicSupplement::CyrillicSmallLetterElWithMiddleHook),
            'Ԣ' => Ok(CyrillicSupplement::CyrillicCapitalLetterEnWithMiddleHook),
            'ԣ' => Ok(CyrillicSupplement::CyrillicSmallLetterEnWithMiddleHook),
            'Ԥ' => Ok(CyrillicSupplement::CyrillicCapitalLetterPeWithDescender),
            'ԥ' => Ok(CyrillicSupplement::CyrillicSmallLetterPeWithDescender),
            'Ԧ' => Ok(CyrillicSupplement::CyrillicCapitalLetterShhaWithDescender),
            'ԧ' => Ok(CyrillicSupplement::CyrillicSmallLetterShhaWithDescender),
            'Ԩ' => Ok(CyrillicSupplement::CyrillicCapitalLetterEnWithLeftHook),
            'ԩ' => Ok(CyrillicSupplement::CyrillicSmallLetterEnWithLeftHook),
            'Ԫ' => Ok(CyrillicSupplement::CyrillicCapitalLetterDzzhe),
            'ԫ' => Ok(CyrillicSupplement::CyrillicSmallLetterDzzhe),
            'Ԭ' => Ok(CyrillicSupplement::CyrillicCapitalLetterDche),
            'ԭ' => Ok(CyrillicSupplement::CyrillicSmallLetterDche),
            'Ԯ' => Ok(CyrillicSupplement::CyrillicCapitalLetterElWithDescender),
            _ => Err(()),
        }
    }
}

impl Into<u32> for CyrillicSupplement {
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

impl std::convert::TryFrom<u32> for CyrillicSupplement {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for CyrillicSupplement {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl CyrillicSupplement {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        CyrillicSupplement::CyrillicCapitalLetterKomiDe
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            CyrillicSupplement::CyrillicCapitalLetterKomiDe => "cyrillic capital letter komi de",
            CyrillicSupplement::CyrillicSmallLetterKomiDe => "cyrillic small letter komi de",
            CyrillicSupplement::CyrillicCapitalLetterKomiDje => "cyrillic capital letter komi dje",
            CyrillicSupplement::CyrillicSmallLetterKomiDje => "cyrillic small letter komi dje",
            CyrillicSupplement::CyrillicCapitalLetterKomiZje => "cyrillic capital letter komi zje",
            CyrillicSupplement::CyrillicSmallLetterKomiZje => "cyrillic small letter komi zje",
            CyrillicSupplement::CyrillicCapitalLetterKomiDzje => "cyrillic capital letter komi dzje",
            CyrillicSupplement::CyrillicSmallLetterKomiDzje => "cyrillic small letter komi dzje",
            CyrillicSupplement::CyrillicCapitalLetterKomiLje => "cyrillic capital letter komi lje",
            CyrillicSupplement::CyrillicSmallLetterKomiLje => "cyrillic small letter komi lje",
            CyrillicSupplement::CyrillicCapitalLetterKomiNje => "cyrillic capital letter komi nje",
            CyrillicSupplement::CyrillicSmallLetterKomiNje => "cyrillic small letter komi nje",
            CyrillicSupplement::CyrillicCapitalLetterKomiSje => "cyrillic capital letter komi sje",
            CyrillicSupplement::CyrillicSmallLetterKomiSje => "cyrillic small letter komi sje",
            CyrillicSupplement::CyrillicCapitalLetterKomiTje => "cyrillic capital letter komi tje",
            CyrillicSupplement::CyrillicSmallLetterKomiTje => "cyrillic small letter komi tje",
            CyrillicSupplement::CyrillicCapitalLetterReversedZe => "cyrillic capital letter reversed ze",
            CyrillicSupplement::CyrillicSmallLetterReversedZe => "cyrillic small letter reversed ze",
            CyrillicSupplement::CyrillicCapitalLetterElWithHook => "cyrillic capital letter el with hook",
            CyrillicSupplement::CyrillicSmallLetterElWithHook => "cyrillic small letter el with hook",
            CyrillicSupplement::CyrillicCapitalLetterLha => "cyrillic capital letter lha",
            CyrillicSupplement::CyrillicSmallLetterLha => "cyrillic small letter lha",
            CyrillicSupplement::CyrillicCapitalLetterRha => "cyrillic capital letter rha",
            CyrillicSupplement::CyrillicSmallLetterRha => "cyrillic small letter rha",
            CyrillicSupplement::CyrillicCapitalLetterYae => "cyrillic capital letter yae",
            CyrillicSupplement::CyrillicSmallLetterYae => "cyrillic small letter yae",
            CyrillicSupplement::CyrillicCapitalLetterQa => "cyrillic capital letter qa",
            CyrillicSupplement::CyrillicSmallLetterQa => "cyrillic small letter qa",
            CyrillicSupplement::CyrillicCapitalLetterWe => "cyrillic capital letter we",
            CyrillicSupplement::CyrillicSmallLetterWe => "cyrillic small letter we",
            CyrillicSupplement::CyrillicCapitalLetterAleutKa => "cyrillic capital letter aleut ka",
            CyrillicSupplement::CyrillicSmallLetterAleutKa => "cyrillic small letter aleut ka",
            CyrillicSupplement::CyrillicCapitalLetterElWithMiddleHook => "cyrillic capital letter el with middle hook",
            CyrillicSupplement::CyrillicSmallLetterElWithMiddleHook => "cyrillic small letter el with middle hook",
            CyrillicSupplement::CyrillicCapitalLetterEnWithMiddleHook => "cyrillic capital letter en with middle hook",
            CyrillicSupplement::CyrillicSmallLetterEnWithMiddleHook => "cyrillic small letter en with middle hook",
            CyrillicSupplement::CyrillicCapitalLetterPeWithDescender => "cyrillic capital letter pe with descender",
            CyrillicSupplement::CyrillicSmallLetterPeWithDescender => "cyrillic small letter pe with descender",
            CyrillicSupplement::CyrillicCapitalLetterShhaWithDescender => "cyrillic capital letter shha with descender",
            CyrillicSupplement::CyrillicSmallLetterShhaWithDescender => "cyrillic small letter shha with descender",
            CyrillicSupplement::CyrillicCapitalLetterEnWithLeftHook => "cyrillic capital letter en with left hook",
            CyrillicSupplement::CyrillicSmallLetterEnWithLeftHook => "cyrillic small letter en with left hook",
            CyrillicSupplement::CyrillicCapitalLetterDzzhe => "cyrillic capital letter dzzhe",
            CyrillicSupplement::CyrillicSmallLetterDzzhe => "cyrillic small letter dzzhe",
            CyrillicSupplement::CyrillicCapitalLetterDche => "cyrillic capital letter dche",
            CyrillicSupplement::CyrillicSmallLetterDche => "cyrillic small letter dche",
            CyrillicSupplement::CyrillicCapitalLetterElWithDescender => "cyrillic capital letter el with descender",
        }
    }
}
