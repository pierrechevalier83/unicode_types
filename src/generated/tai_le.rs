
/// An enum to represent all characters in the TaiLe block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum TaiLe {
    /// \u{1950}: 'ᥐ'
    LetterKa,
    /// \u{1951}: 'ᥑ'
    LetterXa,
    /// \u{1952}: 'ᥒ'
    LetterNga,
    /// \u{1953}: 'ᥓ'
    LetterTsa,
    /// \u{1954}: 'ᥔ'
    LetterSa,
    /// \u{1955}: 'ᥕ'
    LetterYa,
    /// \u{1956}: 'ᥖ'
    LetterTa,
    /// \u{1957}: 'ᥗ'
    LetterTha,
    /// \u{1958}: 'ᥘ'
    LetterLa,
    /// \u{1959}: 'ᥙ'
    LetterPa,
    /// \u{195a}: 'ᥚ'
    LetterPha,
    /// \u{195b}: 'ᥛ'
    LetterMa,
    /// \u{195c}: 'ᥜ'
    LetterFa,
    /// \u{195d}: 'ᥝ'
    LetterVa,
    /// \u{195e}: 'ᥞ'
    LetterHa,
    /// \u{195f}: 'ᥟ'
    LetterQa,
    /// \u{1960}: 'ᥠ'
    LetterKha,
    /// \u{1961}: 'ᥡ'
    LetterTsha,
    /// \u{1962}: 'ᥢ'
    LetterNa,
    /// \u{1963}: 'ᥣ'
    LetterA,
    /// \u{1964}: 'ᥤ'
    LetterI,
    /// \u{1965}: 'ᥥ'
    LetterEe,
    /// \u{1966}: 'ᥦ'
    LetterEh,
    /// \u{1967}: 'ᥧ'
    LetterU,
    /// \u{1968}: 'ᥨ'
    LetterOo,
    /// \u{1969}: 'ᥩ'
    LetterO,
    /// \u{196a}: 'ᥪ'
    LetterUe,
    /// \u{196b}: 'ᥫ'
    LetterE,
    /// \u{196c}: 'ᥬ'
    LetterAue,
    /// \u{196d}: 'ᥭ'
    LetterAi,
    /// \u{1970}: 'ᥰ'
    LetterToneDash2,
    /// \u{1971}: 'ᥱ'
    LetterToneDash3,
    /// \u{1972}: 'ᥲ'
    LetterToneDash4,
    /// \u{1973}: 'ᥳ'
    LetterToneDash5,
    /// \u{1974}: 'ᥴ'
    LetterToneDash6,
}

impl Into<char> for TaiLe {
    fn into(self) -> char {
        match self {
            TaiLe::LetterKa => 'ᥐ',
            TaiLe::LetterXa => 'ᥑ',
            TaiLe::LetterNga => 'ᥒ',
            TaiLe::LetterTsa => 'ᥓ',
            TaiLe::LetterSa => 'ᥔ',
            TaiLe::LetterYa => 'ᥕ',
            TaiLe::LetterTa => 'ᥖ',
            TaiLe::LetterTha => 'ᥗ',
            TaiLe::LetterLa => 'ᥘ',
            TaiLe::LetterPa => 'ᥙ',
            TaiLe::LetterPha => 'ᥚ',
            TaiLe::LetterMa => 'ᥛ',
            TaiLe::LetterFa => 'ᥜ',
            TaiLe::LetterVa => 'ᥝ',
            TaiLe::LetterHa => 'ᥞ',
            TaiLe::LetterQa => 'ᥟ',
            TaiLe::LetterKha => 'ᥠ',
            TaiLe::LetterTsha => 'ᥡ',
            TaiLe::LetterNa => 'ᥢ',
            TaiLe::LetterA => 'ᥣ',
            TaiLe::LetterI => 'ᥤ',
            TaiLe::LetterEe => 'ᥥ',
            TaiLe::LetterEh => 'ᥦ',
            TaiLe::LetterU => 'ᥧ',
            TaiLe::LetterOo => 'ᥨ',
            TaiLe::LetterO => 'ᥩ',
            TaiLe::LetterUe => 'ᥪ',
            TaiLe::LetterE => 'ᥫ',
            TaiLe::LetterAue => 'ᥬ',
            TaiLe::LetterAi => 'ᥭ',
            TaiLe::LetterToneDash2 => 'ᥰ',
            TaiLe::LetterToneDash3 => 'ᥱ',
            TaiLe::LetterToneDash4 => 'ᥲ',
            TaiLe::LetterToneDash5 => 'ᥳ',
            TaiLe::LetterToneDash6 => 'ᥴ',
        }
    }
}

impl std::convert::TryFrom<char> for TaiLe {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ᥐ' => Ok(TaiLe::LetterKa),
            'ᥑ' => Ok(TaiLe::LetterXa),
            'ᥒ' => Ok(TaiLe::LetterNga),
            'ᥓ' => Ok(TaiLe::LetterTsa),
            'ᥔ' => Ok(TaiLe::LetterSa),
            'ᥕ' => Ok(TaiLe::LetterYa),
            'ᥖ' => Ok(TaiLe::LetterTa),
            'ᥗ' => Ok(TaiLe::LetterTha),
            'ᥘ' => Ok(TaiLe::LetterLa),
            'ᥙ' => Ok(TaiLe::LetterPa),
            'ᥚ' => Ok(TaiLe::LetterPha),
            'ᥛ' => Ok(TaiLe::LetterMa),
            'ᥜ' => Ok(TaiLe::LetterFa),
            'ᥝ' => Ok(TaiLe::LetterVa),
            'ᥞ' => Ok(TaiLe::LetterHa),
            'ᥟ' => Ok(TaiLe::LetterQa),
            'ᥠ' => Ok(TaiLe::LetterKha),
            'ᥡ' => Ok(TaiLe::LetterTsha),
            'ᥢ' => Ok(TaiLe::LetterNa),
            'ᥣ' => Ok(TaiLe::LetterA),
            'ᥤ' => Ok(TaiLe::LetterI),
            'ᥥ' => Ok(TaiLe::LetterEe),
            'ᥦ' => Ok(TaiLe::LetterEh),
            'ᥧ' => Ok(TaiLe::LetterU),
            'ᥨ' => Ok(TaiLe::LetterOo),
            'ᥩ' => Ok(TaiLe::LetterO),
            'ᥪ' => Ok(TaiLe::LetterUe),
            'ᥫ' => Ok(TaiLe::LetterE),
            'ᥬ' => Ok(TaiLe::LetterAue),
            'ᥭ' => Ok(TaiLe::LetterAi),
            'ᥰ' => Ok(TaiLe::LetterToneDash2),
            'ᥱ' => Ok(TaiLe::LetterToneDash3),
            'ᥲ' => Ok(TaiLe::LetterToneDash4),
            'ᥳ' => Ok(TaiLe::LetterToneDash5),
            'ᥴ' => Ok(TaiLe::LetterToneDash6),
            _ => Err(()),
        }
    }
}

impl Into<u32> for TaiLe {
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

impl std::convert::TryFrom<u32> for TaiLe {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for TaiLe {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl TaiLe {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        TaiLe::LetterKa
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("TaiLe{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
