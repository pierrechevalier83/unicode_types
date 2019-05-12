
/// An enum to represent all characters in the Lydian block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Lydian {
    /// \u{10920}: '𐤠'
    LetterA,
    /// \u{10921}: '𐤡'
    LetterB,
    /// \u{10922}: '𐤢'
    LetterG,
    /// \u{10923}: '𐤣'
    LetterD,
    /// \u{10924}: '𐤤'
    LetterE,
    /// \u{10925}: '𐤥'
    LetterV,
    /// \u{10926}: '𐤦'
    LetterI,
    /// \u{10927}: '𐤧'
    LetterY,
    /// \u{10928}: '𐤨'
    LetterK,
    /// \u{10929}: '𐤩'
    LetterL,
    /// \u{1092a}: '𐤪'
    LetterM,
    /// \u{1092b}: '𐤫'
    LetterN,
    /// \u{1092c}: '𐤬'
    LetterO,
    /// \u{1092d}: '𐤭'
    LetterR,
    /// \u{1092e}: '𐤮'
    LetterSs,
    /// \u{1092f}: '𐤯'
    LetterT,
    /// \u{10930}: '𐤰'
    LetterU,
    /// \u{10931}: '𐤱'
    LetterF,
    /// \u{10932}: '𐤲'
    LetterQ,
    /// \u{10933}: '𐤳'
    LetterS,
    /// \u{10934}: '𐤴'
    LetterTt,
    /// \u{10935}: '𐤵'
    LetterAn,
    /// \u{10936}: '𐤶'
    LetterEn,
    /// \u{10937}: '𐤷'
    LetterLy,
    /// \u{10938}: '𐤸'
    LetterNn,
    /// \u{10939}: '𐤹'
    LetterC,
}

impl Into<char> for Lydian {
    fn into(self) -> char {
        match self {
            Lydian::LetterA => '𐤠',
            Lydian::LetterB => '𐤡',
            Lydian::LetterG => '𐤢',
            Lydian::LetterD => '𐤣',
            Lydian::LetterE => '𐤤',
            Lydian::LetterV => '𐤥',
            Lydian::LetterI => '𐤦',
            Lydian::LetterY => '𐤧',
            Lydian::LetterK => '𐤨',
            Lydian::LetterL => '𐤩',
            Lydian::LetterM => '𐤪',
            Lydian::LetterN => '𐤫',
            Lydian::LetterO => '𐤬',
            Lydian::LetterR => '𐤭',
            Lydian::LetterSs => '𐤮',
            Lydian::LetterT => '𐤯',
            Lydian::LetterU => '𐤰',
            Lydian::LetterF => '𐤱',
            Lydian::LetterQ => '𐤲',
            Lydian::LetterS => '𐤳',
            Lydian::LetterTt => '𐤴',
            Lydian::LetterAn => '𐤵',
            Lydian::LetterEn => '𐤶',
            Lydian::LetterLy => '𐤷',
            Lydian::LetterNn => '𐤸',
            Lydian::LetterC => '𐤹',
        }
    }
}

impl std::convert::TryFrom<char> for Lydian {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𐤠' => Ok(Lydian::LetterA),
            '𐤡' => Ok(Lydian::LetterB),
            '𐤢' => Ok(Lydian::LetterG),
            '𐤣' => Ok(Lydian::LetterD),
            '𐤤' => Ok(Lydian::LetterE),
            '𐤥' => Ok(Lydian::LetterV),
            '𐤦' => Ok(Lydian::LetterI),
            '𐤧' => Ok(Lydian::LetterY),
            '𐤨' => Ok(Lydian::LetterK),
            '𐤩' => Ok(Lydian::LetterL),
            '𐤪' => Ok(Lydian::LetterM),
            '𐤫' => Ok(Lydian::LetterN),
            '𐤬' => Ok(Lydian::LetterO),
            '𐤭' => Ok(Lydian::LetterR),
            '𐤮' => Ok(Lydian::LetterSs),
            '𐤯' => Ok(Lydian::LetterT),
            '𐤰' => Ok(Lydian::LetterU),
            '𐤱' => Ok(Lydian::LetterF),
            '𐤲' => Ok(Lydian::LetterQ),
            '𐤳' => Ok(Lydian::LetterS),
            '𐤴' => Ok(Lydian::LetterTt),
            '𐤵' => Ok(Lydian::LetterAn),
            '𐤶' => Ok(Lydian::LetterEn),
            '𐤷' => Ok(Lydian::LetterLy),
            '𐤸' => Ok(Lydian::LetterNn),
            '𐤹' => Ok(Lydian::LetterC),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Lydian {
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

impl std::convert::TryFrom<u32> for Lydian {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Lydian {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Lydian {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Lydian::LetterA
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Lydian::LetterA => "lydian letter a",
            Lydian::LetterB => "lydian letter b",
            Lydian::LetterG => "lydian letter g",
            Lydian::LetterD => "lydian letter d",
            Lydian::LetterE => "lydian letter e",
            Lydian::LetterV => "lydian letter v",
            Lydian::LetterI => "lydian letter i",
            Lydian::LetterY => "lydian letter y",
            Lydian::LetterK => "lydian letter k",
            Lydian::LetterL => "lydian letter l",
            Lydian::LetterM => "lydian letter m",
            Lydian::LetterN => "lydian letter n",
            Lydian::LetterO => "lydian letter o",
            Lydian::LetterR => "lydian letter r",
            Lydian::LetterSs => "lydian letter ss",
            Lydian::LetterT => "lydian letter t",
            Lydian::LetterU => "lydian letter u",
            Lydian::LetterF => "lydian letter f",
            Lydian::LetterQ => "lydian letter q",
            Lydian::LetterS => "lydian letter s",
            Lydian::LetterTt => "lydian letter tt",
            Lydian::LetterAn => "lydian letter an",
            Lydian::LetterEn => "lydian letter en",
            Lydian::LetterLy => "lydian letter ly",
            Lydian::LetterNn => "lydian letter nn",
            Lydian::LetterC => "lydian letter c",
        }
    }
}
