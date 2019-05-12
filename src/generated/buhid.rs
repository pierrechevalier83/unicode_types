
/// An enum to represent all characters in the Buhid block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Buhid {
    /// \u{1740}: 'ᝀ'
    LetterA,
    /// \u{1741}: 'ᝁ'
    LetterI,
    /// \u{1742}: 'ᝂ'
    LetterU,
    /// \u{1743}: 'ᝃ'
    LetterKa,
    /// \u{1744}: 'ᝄ'
    LetterGa,
    /// \u{1745}: 'ᝅ'
    LetterNga,
    /// \u{1746}: 'ᝆ'
    LetterTa,
    /// \u{1747}: 'ᝇ'
    LetterDa,
    /// \u{1748}: 'ᝈ'
    LetterNa,
    /// \u{1749}: 'ᝉ'
    LetterPa,
    /// \u{174a}: 'ᝊ'
    LetterBa,
    /// \u{174b}: 'ᝋ'
    LetterMa,
    /// \u{174c}: 'ᝌ'
    LetterYa,
    /// \u{174d}: 'ᝍ'
    LetterRa,
    /// \u{174e}: 'ᝎ'
    LetterLa,
    /// \u{174f}: 'ᝏ'
    LetterWa,
    /// \u{1750}: 'ᝐ'
    LetterSa,
    /// \u{1751}: 'ᝑ'
    LetterHa,
    /// \u{1752}: 'ᝒ'
    VowelSignI,
    /// \u{1753}: 'ᝓ'
    VowelSignU,
}

impl Into<char> for Buhid {
    fn into(self) -> char {
        match self {
            Buhid::LetterA => 'ᝀ',
            Buhid::LetterI => 'ᝁ',
            Buhid::LetterU => 'ᝂ',
            Buhid::LetterKa => 'ᝃ',
            Buhid::LetterGa => 'ᝄ',
            Buhid::LetterNga => 'ᝅ',
            Buhid::LetterTa => 'ᝆ',
            Buhid::LetterDa => 'ᝇ',
            Buhid::LetterNa => 'ᝈ',
            Buhid::LetterPa => 'ᝉ',
            Buhid::LetterBa => 'ᝊ',
            Buhid::LetterMa => 'ᝋ',
            Buhid::LetterYa => 'ᝌ',
            Buhid::LetterRa => 'ᝍ',
            Buhid::LetterLa => 'ᝎ',
            Buhid::LetterWa => 'ᝏ',
            Buhid::LetterSa => 'ᝐ',
            Buhid::LetterHa => 'ᝑ',
            Buhid::VowelSignI => 'ᝒ',
            Buhid::VowelSignU => 'ᝓ',
        }
    }
}

impl std::convert::TryFrom<char> for Buhid {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ᝀ' => Ok(Buhid::LetterA),
            'ᝁ' => Ok(Buhid::LetterI),
            'ᝂ' => Ok(Buhid::LetterU),
            'ᝃ' => Ok(Buhid::LetterKa),
            'ᝄ' => Ok(Buhid::LetterGa),
            'ᝅ' => Ok(Buhid::LetterNga),
            'ᝆ' => Ok(Buhid::LetterTa),
            'ᝇ' => Ok(Buhid::LetterDa),
            'ᝈ' => Ok(Buhid::LetterNa),
            'ᝉ' => Ok(Buhid::LetterPa),
            'ᝊ' => Ok(Buhid::LetterBa),
            'ᝋ' => Ok(Buhid::LetterMa),
            'ᝌ' => Ok(Buhid::LetterYa),
            'ᝍ' => Ok(Buhid::LetterRa),
            'ᝎ' => Ok(Buhid::LetterLa),
            'ᝏ' => Ok(Buhid::LetterWa),
            'ᝐ' => Ok(Buhid::LetterSa),
            'ᝑ' => Ok(Buhid::LetterHa),
            'ᝒ' => Ok(Buhid::VowelSignI),
            'ᝓ' => Ok(Buhid::VowelSignU),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Buhid {
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

impl std::convert::TryFrom<u32> for Buhid {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Buhid {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Buhid {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Buhid::LetterA
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Buhid{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
