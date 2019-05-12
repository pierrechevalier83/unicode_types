
/// An enum to represent all characters in the Tagalog block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Tagalog {
    /// \u{1700}: 'ᜀ'
    LetterA,
    /// \u{1701}: 'ᜁ'
    LetterI,
    /// \u{1702}: 'ᜂ'
    LetterU,
    /// \u{1703}: 'ᜃ'
    LetterKa,
    /// \u{1704}: 'ᜄ'
    LetterGa,
    /// \u{1705}: 'ᜅ'
    LetterNga,
    /// \u{1706}: 'ᜆ'
    LetterTa,
    /// \u{1707}: 'ᜇ'
    LetterDa,
    /// \u{1708}: 'ᜈ'
    LetterNa,
    /// \u{1709}: 'ᜉ'
    LetterPa,
    /// \u{170a}: 'ᜊ'
    LetterBa,
    /// \u{170b}: 'ᜋ'
    LetterMa,
    /// \u{170c}: 'ᜌ'
    LetterYa,
    /// \u{170e}: 'ᜎ'
    LetterLa,
    /// \u{170f}: 'ᜏ'
    LetterWa,
    /// \u{1710}: 'ᜐ'
    LetterSa,
    /// \u{1711}: 'ᜑ'
    LetterHa,
    /// \u{1712}: 'ᜒ'
    VowelSignI,
    /// \u{1713}: 'ᜓ'
    VowelSignU,
    /// \u{1714}: '᜔'
    SignVirama,
}

impl Into<char> for Tagalog {
    fn into(self) -> char {
        match self {
            Tagalog::LetterA => 'ᜀ',
            Tagalog::LetterI => 'ᜁ',
            Tagalog::LetterU => 'ᜂ',
            Tagalog::LetterKa => 'ᜃ',
            Tagalog::LetterGa => 'ᜄ',
            Tagalog::LetterNga => 'ᜅ',
            Tagalog::LetterTa => 'ᜆ',
            Tagalog::LetterDa => 'ᜇ',
            Tagalog::LetterNa => 'ᜈ',
            Tagalog::LetterPa => 'ᜉ',
            Tagalog::LetterBa => 'ᜊ',
            Tagalog::LetterMa => 'ᜋ',
            Tagalog::LetterYa => 'ᜌ',
            Tagalog::LetterLa => 'ᜎ',
            Tagalog::LetterWa => 'ᜏ',
            Tagalog::LetterSa => 'ᜐ',
            Tagalog::LetterHa => 'ᜑ',
            Tagalog::VowelSignI => 'ᜒ',
            Tagalog::VowelSignU => 'ᜓ',
            Tagalog::SignVirama => '᜔',
        }
    }
}

impl std::convert::TryFrom<char> for Tagalog {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ᜀ' => Ok(Tagalog::LetterA),
            'ᜁ' => Ok(Tagalog::LetterI),
            'ᜂ' => Ok(Tagalog::LetterU),
            'ᜃ' => Ok(Tagalog::LetterKa),
            'ᜄ' => Ok(Tagalog::LetterGa),
            'ᜅ' => Ok(Tagalog::LetterNga),
            'ᜆ' => Ok(Tagalog::LetterTa),
            'ᜇ' => Ok(Tagalog::LetterDa),
            'ᜈ' => Ok(Tagalog::LetterNa),
            'ᜉ' => Ok(Tagalog::LetterPa),
            'ᜊ' => Ok(Tagalog::LetterBa),
            'ᜋ' => Ok(Tagalog::LetterMa),
            'ᜌ' => Ok(Tagalog::LetterYa),
            'ᜎ' => Ok(Tagalog::LetterLa),
            'ᜏ' => Ok(Tagalog::LetterWa),
            'ᜐ' => Ok(Tagalog::LetterSa),
            'ᜑ' => Ok(Tagalog::LetterHa),
            'ᜒ' => Ok(Tagalog::VowelSignI),
            'ᜓ' => Ok(Tagalog::VowelSignU),
            '᜔' => Ok(Tagalog::SignVirama),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Tagalog {
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

impl std::convert::TryFrom<u32> for Tagalog {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Tagalog {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Tagalog {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Tagalog::LetterA
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Tagalog::LetterA => "tagalog letter a",
            Tagalog::LetterI => "tagalog letter i",
            Tagalog::LetterU => "tagalog letter u",
            Tagalog::LetterKa => "tagalog letter ka",
            Tagalog::LetterGa => "tagalog letter ga",
            Tagalog::LetterNga => "tagalog letter nga",
            Tagalog::LetterTa => "tagalog letter ta",
            Tagalog::LetterDa => "tagalog letter da",
            Tagalog::LetterNa => "tagalog letter na",
            Tagalog::LetterPa => "tagalog letter pa",
            Tagalog::LetterBa => "tagalog letter ba",
            Tagalog::LetterMa => "tagalog letter ma",
            Tagalog::LetterYa => "tagalog letter ya",
            Tagalog::LetterLa => "tagalog letter la",
            Tagalog::LetterWa => "tagalog letter wa",
            Tagalog::LetterSa => "tagalog letter sa",
            Tagalog::LetterHa => "tagalog letter ha",
            Tagalog::VowelSignI => "tagalog vowel sign i",
            Tagalog::VowelSignU => "tagalog vowel sign u",
            Tagalog::SignVirama => "tagalog sign virama",
        }
    }
}
