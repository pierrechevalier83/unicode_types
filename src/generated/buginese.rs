
/// An enum to represent all characters in the Buginese block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Buginese {
    /// \u{1a00}: 'ᨀ'
    LetterKa,
    /// \u{1a01}: 'ᨁ'
    LetterGa,
    /// \u{1a02}: 'ᨂ'
    LetterNga,
    /// \u{1a03}: 'ᨃ'
    LetterNgka,
    /// \u{1a04}: 'ᨄ'
    LetterPa,
    /// \u{1a05}: 'ᨅ'
    LetterBa,
    /// \u{1a06}: 'ᨆ'
    LetterMa,
    /// \u{1a07}: 'ᨇ'
    LetterMpa,
    /// \u{1a08}: 'ᨈ'
    LetterTa,
    /// \u{1a09}: 'ᨉ'
    LetterDa,
    /// \u{1a0a}: 'ᨊ'
    LetterNa,
    /// \u{1a0b}: 'ᨋ'
    LetterNra,
    /// \u{1a0c}: 'ᨌ'
    LetterCa,
    /// \u{1a0d}: 'ᨍ'
    LetterJa,
    /// \u{1a0e}: 'ᨎ'
    LetterNya,
    /// \u{1a0f}: 'ᨏ'
    LetterNyca,
    /// \u{1a10}: 'ᨐ'
    LetterYa,
    /// \u{1a11}: 'ᨑ'
    LetterRa,
    /// \u{1a12}: 'ᨒ'
    LetterLa,
    /// \u{1a13}: 'ᨓ'
    LetterVa,
    /// \u{1a14}: 'ᨔ'
    LetterSa,
    /// \u{1a15}: 'ᨕ'
    LetterA,
    /// \u{1a16}: 'ᨖ'
    LetterHa,
    /// \u{1a17}: 'ᨗ'
    VowelSignI,
    /// \u{1a18}: 'ᨘ'
    VowelSignU,
    /// \u{1a19}: 'ᨙ'
    VowelSignE,
    /// \u{1a1a}: 'ᨚ'
    VowelSignO,
    /// \u{1a1b}: 'ᨛ'
    VowelSignAe,
    /// \u{1a1e}: '᨞'
    Pallawa,
}

impl Into<char> for Buginese {
    fn into(self) -> char {
        match self {
            Buginese::LetterKa => 'ᨀ',
            Buginese::LetterGa => 'ᨁ',
            Buginese::LetterNga => 'ᨂ',
            Buginese::LetterNgka => 'ᨃ',
            Buginese::LetterPa => 'ᨄ',
            Buginese::LetterBa => 'ᨅ',
            Buginese::LetterMa => 'ᨆ',
            Buginese::LetterMpa => 'ᨇ',
            Buginese::LetterTa => 'ᨈ',
            Buginese::LetterDa => 'ᨉ',
            Buginese::LetterNa => 'ᨊ',
            Buginese::LetterNra => 'ᨋ',
            Buginese::LetterCa => 'ᨌ',
            Buginese::LetterJa => 'ᨍ',
            Buginese::LetterNya => 'ᨎ',
            Buginese::LetterNyca => 'ᨏ',
            Buginese::LetterYa => 'ᨐ',
            Buginese::LetterRa => 'ᨑ',
            Buginese::LetterLa => 'ᨒ',
            Buginese::LetterVa => 'ᨓ',
            Buginese::LetterSa => 'ᨔ',
            Buginese::LetterA => 'ᨕ',
            Buginese::LetterHa => 'ᨖ',
            Buginese::VowelSignI => 'ᨗ',
            Buginese::VowelSignU => 'ᨘ',
            Buginese::VowelSignE => 'ᨙ',
            Buginese::VowelSignO => 'ᨚ',
            Buginese::VowelSignAe => 'ᨛ',
            Buginese::Pallawa => '᨞',
        }
    }
}

impl std::convert::TryFrom<char> for Buginese {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ᨀ' => Ok(Buginese::LetterKa),
            'ᨁ' => Ok(Buginese::LetterGa),
            'ᨂ' => Ok(Buginese::LetterNga),
            'ᨃ' => Ok(Buginese::LetterNgka),
            'ᨄ' => Ok(Buginese::LetterPa),
            'ᨅ' => Ok(Buginese::LetterBa),
            'ᨆ' => Ok(Buginese::LetterMa),
            'ᨇ' => Ok(Buginese::LetterMpa),
            'ᨈ' => Ok(Buginese::LetterTa),
            'ᨉ' => Ok(Buginese::LetterDa),
            'ᨊ' => Ok(Buginese::LetterNa),
            'ᨋ' => Ok(Buginese::LetterNra),
            'ᨌ' => Ok(Buginese::LetterCa),
            'ᨍ' => Ok(Buginese::LetterJa),
            'ᨎ' => Ok(Buginese::LetterNya),
            'ᨏ' => Ok(Buginese::LetterNyca),
            'ᨐ' => Ok(Buginese::LetterYa),
            'ᨑ' => Ok(Buginese::LetterRa),
            'ᨒ' => Ok(Buginese::LetterLa),
            'ᨓ' => Ok(Buginese::LetterVa),
            'ᨔ' => Ok(Buginese::LetterSa),
            'ᨕ' => Ok(Buginese::LetterA),
            'ᨖ' => Ok(Buginese::LetterHa),
            'ᨗ' => Ok(Buginese::VowelSignI),
            'ᨘ' => Ok(Buginese::VowelSignU),
            'ᨙ' => Ok(Buginese::VowelSignE),
            'ᨚ' => Ok(Buginese::VowelSignO),
            'ᨛ' => Ok(Buginese::VowelSignAe),
            '᨞' => Ok(Buginese::Pallawa),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Buginese {
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

impl std::convert::TryFrom<u32> for Buginese {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Buginese {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Buginese {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Buginese::LetterKa
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Buginese::LetterKa => "buginese letter ka",
            Buginese::LetterGa => "buginese letter ga",
            Buginese::LetterNga => "buginese letter nga",
            Buginese::LetterNgka => "buginese letter ngka",
            Buginese::LetterPa => "buginese letter pa",
            Buginese::LetterBa => "buginese letter ba",
            Buginese::LetterMa => "buginese letter ma",
            Buginese::LetterMpa => "buginese letter mpa",
            Buginese::LetterTa => "buginese letter ta",
            Buginese::LetterDa => "buginese letter da",
            Buginese::LetterNa => "buginese letter na",
            Buginese::LetterNra => "buginese letter nra",
            Buginese::LetterCa => "buginese letter ca",
            Buginese::LetterJa => "buginese letter ja",
            Buginese::LetterNya => "buginese letter nya",
            Buginese::LetterNyca => "buginese letter nyca",
            Buginese::LetterYa => "buginese letter ya",
            Buginese::LetterRa => "buginese letter ra",
            Buginese::LetterLa => "buginese letter la",
            Buginese::LetterVa => "buginese letter va",
            Buginese::LetterSa => "buginese letter sa",
            Buginese::LetterA => "buginese letter a",
            Buginese::LetterHa => "buginese letter ha",
            Buginese::VowelSignI => "buginese vowel sign i",
            Buginese::VowelSignU => "buginese vowel sign u",
            Buginese::VowelSignE => "buginese vowel sign e",
            Buginese::VowelSignO => "buginese vowel sign o",
            Buginese::VowelSignAe => "buginese vowel sign ae",
            Buginese::Pallawa => "buginese pallawa",
        }
    }
}
