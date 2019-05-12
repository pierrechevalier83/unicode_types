
/// An enum to represent all characters in the Tagbanwa block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Tagbanwa {
    /// \u{1760}: 'ᝠ'
    LetterA,
    /// \u{1761}: 'ᝡ'
    LetterI,
    /// \u{1762}: 'ᝢ'
    LetterU,
    /// \u{1763}: 'ᝣ'
    LetterKa,
    /// \u{1764}: 'ᝤ'
    LetterGa,
    /// \u{1765}: 'ᝥ'
    LetterNga,
    /// \u{1766}: 'ᝦ'
    LetterTa,
    /// \u{1767}: 'ᝧ'
    LetterDa,
    /// \u{1768}: 'ᝨ'
    LetterNa,
    /// \u{1769}: 'ᝩ'
    LetterPa,
    /// \u{176a}: 'ᝪ'
    LetterBa,
    /// \u{176b}: 'ᝫ'
    LetterMa,
    /// \u{176c}: 'ᝬ'
    LetterYa,
    /// \u{176e}: 'ᝮ'
    LetterLa,
    /// \u{176f}: 'ᝯ'
    LetterWa,
    /// \u{1770}: 'ᝰ'
    LetterSa,
    /// \u{1772}: 'ᝲ'
    VowelSignI,
    /// \u{1773}: 'ᝳ'
    VowelSignU,
}

impl Into<char> for Tagbanwa {
    fn into(self) -> char {
        match self {
            Tagbanwa::LetterA => 'ᝠ',
            Tagbanwa::LetterI => 'ᝡ',
            Tagbanwa::LetterU => 'ᝢ',
            Tagbanwa::LetterKa => 'ᝣ',
            Tagbanwa::LetterGa => 'ᝤ',
            Tagbanwa::LetterNga => 'ᝥ',
            Tagbanwa::LetterTa => 'ᝦ',
            Tagbanwa::LetterDa => 'ᝧ',
            Tagbanwa::LetterNa => 'ᝨ',
            Tagbanwa::LetterPa => 'ᝩ',
            Tagbanwa::LetterBa => 'ᝪ',
            Tagbanwa::LetterMa => 'ᝫ',
            Tagbanwa::LetterYa => 'ᝬ',
            Tagbanwa::LetterLa => 'ᝮ',
            Tagbanwa::LetterWa => 'ᝯ',
            Tagbanwa::LetterSa => 'ᝰ',
            Tagbanwa::VowelSignI => 'ᝲ',
            Tagbanwa::VowelSignU => 'ᝳ',
        }
    }
}

impl std::convert::TryFrom<char> for Tagbanwa {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ᝠ' => Ok(Tagbanwa::LetterA),
            'ᝡ' => Ok(Tagbanwa::LetterI),
            'ᝢ' => Ok(Tagbanwa::LetterU),
            'ᝣ' => Ok(Tagbanwa::LetterKa),
            'ᝤ' => Ok(Tagbanwa::LetterGa),
            'ᝥ' => Ok(Tagbanwa::LetterNga),
            'ᝦ' => Ok(Tagbanwa::LetterTa),
            'ᝧ' => Ok(Tagbanwa::LetterDa),
            'ᝨ' => Ok(Tagbanwa::LetterNa),
            'ᝩ' => Ok(Tagbanwa::LetterPa),
            'ᝪ' => Ok(Tagbanwa::LetterBa),
            'ᝫ' => Ok(Tagbanwa::LetterMa),
            'ᝬ' => Ok(Tagbanwa::LetterYa),
            'ᝮ' => Ok(Tagbanwa::LetterLa),
            'ᝯ' => Ok(Tagbanwa::LetterWa),
            'ᝰ' => Ok(Tagbanwa::LetterSa),
            'ᝲ' => Ok(Tagbanwa::VowelSignI),
            'ᝳ' => Ok(Tagbanwa::VowelSignU),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Tagbanwa {
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

impl std::convert::TryFrom<u32> for Tagbanwa {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Tagbanwa {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Tagbanwa {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Tagbanwa::LetterA
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Tagbanwa::LetterA => "tagbanwa letter a",
            Tagbanwa::LetterI => "tagbanwa letter i",
            Tagbanwa::LetterU => "tagbanwa letter u",
            Tagbanwa::LetterKa => "tagbanwa letter ka",
            Tagbanwa::LetterGa => "tagbanwa letter ga",
            Tagbanwa::LetterNga => "tagbanwa letter nga",
            Tagbanwa::LetterTa => "tagbanwa letter ta",
            Tagbanwa::LetterDa => "tagbanwa letter da",
            Tagbanwa::LetterNa => "tagbanwa letter na",
            Tagbanwa::LetterPa => "tagbanwa letter pa",
            Tagbanwa::LetterBa => "tagbanwa letter ba",
            Tagbanwa::LetterMa => "tagbanwa letter ma",
            Tagbanwa::LetterYa => "tagbanwa letter ya",
            Tagbanwa::LetterLa => "tagbanwa letter la",
            Tagbanwa::LetterWa => "tagbanwa letter wa",
            Tagbanwa::LetterSa => "tagbanwa letter sa",
            Tagbanwa::VowelSignI => "tagbanwa vowel sign i",
            Tagbanwa::VowelSignU => "tagbanwa vowel sign u",
        }
    }
}
