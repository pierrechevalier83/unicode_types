
/// An enum to represent all characters in the Hanunoo block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Hanunoo {
    /// \u{1720}: 'ᜠ'
    LetterA,
    /// \u{1721}: 'ᜡ'
    LetterI,
    /// \u{1722}: 'ᜢ'
    LetterU,
    /// \u{1723}: 'ᜣ'
    LetterKa,
    /// \u{1724}: 'ᜤ'
    LetterGa,
    /// \u{1725}: 'ᜥ'
    LetterNga,
    /// \u{1726}: 'ᜦ'
    LetterTa,
    /// \u{1727}: 'ᜧ'
    LetterDa,
    /// \u{1728}: 'ᜨ'
    LetterNa,
    /// \u{1729}: 'ᜩ'
    LetterPa,
    /// \u{172a}: 'ᜪ'
    LetterBa,
    /// \u{172b}: 'ᜫ'
    LetterMa,
    /// \u{172c}: 'ᜬ'
    LetterYa,
    /// \u{172d}: 'ᜭ'
    LetterRa,
    /// \u{172e}: 'ᜮ'
    LetterLa,
    /// \u{172f}: 'ᜯ'
    LetterWa,
    /// \u{1730}: 'ᜰ'
    LetterSa,
    /// \u{1731}: 'ᜱ'
    LetterHa,
    /// \u{1732}: 'ᜲ'
    VowelSignI,
    /// \u{1733}: 'ᜳ'
    VowelSignU,
    /// \u{1734}: '᜴'
    SignPamudpod,
    /// \u{1735}: '᜵'
    PhilippineSinglePunctuation,
    /// \u{1736}: '᜶'
    PhilippineDoublePunctuation,
}

impl Into<char> for Hanunoo {
    fn into(self) -> char {
        match self {
            Hanunoo::LetterA => 'ᜠ',
            Hanunoo::LetterI => 'ᜡ',
            Hanunoo::LetterU => 'ᜢ',
            Hanunoo::LetterKa => 'ᜣ',
            Hanunoo::LetterGa => 'ᜤ',
            Hanunoo::LetterNga => 'ᜥ',
            Hanunoo::LetterTa => 'ᜦ',
            Hanunoo::LetterDa => 'ᜧ',
            Hanunoo::LetterNa => 'ᜨ',
            Hanunoo::LetterPa => 'ᜩ',
            Hanunoo::LetterBa => 'ᜪ',
            Hanunoo::LetterMa => 'ᜫ',
            Hanunoo::LetterYa => 'ᜬ',
            Hanunoo::LetterRa => 'ᜭ',
            Hanunoo::LetterLa => 'ᜮ',
            Hanunoo::LetterWa => 'ᜯ',
            Hanunoo::LetterSa => 'ᜰ',
            Hanunoo::LetterHa => 'ᜱ',
            Hanunoo::VowelSignI => 'ᜲ',
            Hanunoo::VowelSignU => 'ᜳ',
            Hanunoo::SignPamudpod => '᜴',
            Hanunoo::PhilippineSinglePunctuation => '᜵',
            Hanunoo::PhilippineDoublePunctuation => '᜶',
        }
    }
}

impl std::convert::TryFrom<char> for Hanunoo {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ᜠ' => Ok(Hanunoo::LetterA),
            'ᜡ' => Ok(Hanunoo::LetterI),
            'ᜢ' => Ok(Hanunoo::LetterU),
            'ᜣ' => Ok(Hanunoo::LetterKa),
            'ᜤ' => Ok(Hanunoo::LetterGa),
            'ᜥ' => Ok(Hanunoo::LetterNga),
            'ᜦ' => Ok(Hanunoo::LetterTa),
            'ᜧ' => Ok(Hanunoo::LetterDa),
            'ᜨ' => Ok(Hanunoo::LetterNa),
            'ᜩ' => Ok(Hanunoo::LetterPa),
            'ᜪ' => Ok(Hanunoo::LetterBa),
            'ᜫ' => Ok(Hanunoo::LetterMa),
            'ᜬ' => Ok(Hanunoo::LetterYa),
            'ᜭ' => Ok(Hanunoo::LetterRa),
            'ᜮ' => Ok(Hanunoo::LetterLa),
            'ᜯ' => Ok(Hanunoo::LetterWa),
            'ᜰ' => Ok(Hanunoo::LetterSa),
            'ᜱ' => Ok(Hanunoo::LetterHa),
            'ᜲ' => Ok(Hanunoo::VowelSignI),
            'ᜳ' => Ok(Hanunoo::VowelSignU),
            '᜴' => Ok(Hanunoo::SignPamudpod),
            '᜵' => Ok(Hanunoo::PhilippineSinglePunctuation),
            '᜶' => Ok(Hanunoo::PhilippineDoublePunctuation),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Hanunoo {
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

impl std::convert::TryFrom<u32> for Hanunoo {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Hanunoo {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Hanunoo {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Hanunoo::LetterA
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Hanunoo{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
