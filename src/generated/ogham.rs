
/// An enum to represent all characters in the Ogham block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Ogham {
    /// \u{1680}: ' '
    SpaceMark,
    /// \u{1681}: 'ᚁ'
    LetterBeith,
    /// \u{1682}: 'ᚂ'
    LetterLuis,
    /// \u{1683}: 'ᚃ'
    LetterFearn,
    /// \u{1684}: 'ᚄ'
    LetterSail,
    /// \u{1685}: 'ᚅ'
    LetterNion,
    /// \u{1686}: 'ᚆ'
    LetterUath,
    /// \u{1687}: 'ᚇ'
    LetterDair,
    /// \u{1688}: 'ᚈ'
    LetterTinne,
    /// \u{1689}: 'ᚉ'
    LetterColl,
    /// \u{168a}: 'ᚊ'
    LetterCeirt,
    /// \u{168b}: 'ᚋ'
    LetterMuin,
    /// \u{168c}: 'ᚌ'
    LetterGort,
    /// \u{168d}: 'ᚍ'
    LetterNgeadal,
    /// \u{168e}: 'ᚎ'
    LetterStraif,
    /// \u{168f}: 'ᚏ'
    LetterRuis,
    /// \u{1690}: 'ᚐ'
    LetterAilm,
    /// \u{1691}: 'ᚑ'
    LetterOnn,
    /// \u{1692}: 'ᚒ'
    LetterUr,
    /// \u{1693}: 'ᚓ'
    LetterEadhadh,
    /// \u{1694}: 'ᚔ'
    LetterIodhadh,
    /// \u{1695}: 'ᚕ'
    LetterEabhadh,
    /// \u{1696}: 'ᚖ'
    LetterOr,
    /// \u{1697}: 'ᚗ'
    LetterUilleann,
    /// \u{1698}: 'ᚘ'
    LetterIfin,
    /// \u{1699}: 'ᚙ'
    LetterEamhancholl,
    /// \u{169a}: 'ᚚ'
    LetterPeith,
    /// \u{169b}: '᚛'
    FeatherMark,
    /// \u{169c}: '᚜'
    ReversedFeatherMark,
}

impl Into<char> for Ogham {
    fn into(self) -> char {
        match self {
            Ogham::SpaceMark => ' ',
            Ogham::LetterBeith => 'ᚁ',
            Ogham::LetterLuis => 'ᚂ',
            Ogham::LetterFearn => 'ᚃ',
            Ogham::LetterSail => 'ᚄ',
            Ogham::LetterNion => 'ᚅ',
            Ogham::LetterUath => 'ᚆ',
            Ogham::LetterDair => 'ᚇ',
            Ogham::LetterTinne => 'ᚈ',
            Ogham::LetterColl => 'ᚉ',
            Ogham::LetterCeirt => 'ᚊ',
            Ogham::LetterMuin => 'ᚋ',
            Ogham::LetterGort => 'ᚌ',
            Ogham::LetterNgeadal => 'ᚍ',
            Ogham::LetterStraif => 'ᚎ',
            Ogham::LetterRuis => 'ᚏ',
            Ogham::LetterAilm => 'ᚐ',
            Ogham::LetterOnn => 'ᚑ',
            Ogham::LetterUr => 'ᚒ',
            Ogham::LetterEadhadh => 'ᚓ',
            Ogham::LetterIodhadh => 'ᚔ',
            Ogham::LetterEabhadh => 'ᚕ',
            Ogham::LetterOr => 'ᚖ',
            Ogham::LetterUilleann => 'ᚗ',
            Ogham::LetterIfin => 'ᚘ',
            Ogham::LetterEamhancholl => 'ᚙ',
            Ogham::LetterPeith => 'ᚚ',
            Ogham::FeatherMark => '᚛',
            Ogham::ReversedFeatherMark => '᚜',
        }
    }
}

impl std::convert::TryFrom<char> for Ogham {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            ' ' => Ok(Ogham::SpaceMark),
            'ᚁ' => Ok(Ogham::LetterBeith),
            'ᚂ' => Ok(Ogham::LetterLuis),
            'ᚃ' => Ok(Ogham::LetterFearn),
            'ᚄ' => Ok(Ogham::LetterSail),
            'ᚅ' => Ok(Ogham::LetterNion),
            'ᚆ' => Ok(Ogham::LetterUath),
            'ᚇ' => Ok(Ogham::LetterDair),
            'ᚈ' => Ok(Ogham::LetterTinne),
            'ᚉ' => Ok(Ogham::LetterColl),
            'ᚊ' => Ok(Ogham::LetterCeirt),
            'ᚋ' => Ok(Ogham::LetterMuin),
            'ᚌ' => Ok(Ogham::LetterGort),
            'ᚍ' => Ok(Ogham::LetterNgeadal),
            'ᚎ' => Ok(Ogham::LetterStraif),
            'ᚏ' => Ok(Ogham::LetterRuis),
            'ᚐ' => Ok(Ogham::LetterAilm),
            'ᚑ' => Ok(Ogham::LetterOnn),
            'ᚒ' => Ok(Ogham::LetterUr),
            'ᚓ' => Ok(Ogham::LetterEadhadh),
            'ᚔ' => Ok(Ogham::LetterIodhadh),
            'ᚕ' => Ok(Ogham::LetterEabhadh),
            'ᚖ' => Ok(Ogham::LetterOr),
            'ᚗ' => Ok(Ogham::LetterUilleann),
            'ᚘ' => Ok(Ogham::LetterIfin),
            'ᚙ' => Ok(Ogham::LetterEamhancholl),
            'ᚚ' => Ok(Ogham::LetterPeith),
            '᚛' => Ok(Ogham::FeatherMark),
            '᚜' => Ok(Ogham::ReversedFeatherMark),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Ogham {
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

impl std::convert::TryFrom<u32> for Ogham {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Ogham {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Ogham {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Ogham::SpaceMark
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Ogham::SpaceMark => "ogham space mark",
            Ogham::LetterBeith => "ogham letter beith",
            Ogham::LetterLuis => "ogham letter luis",
            Ogham::LetterFearn => "ogham letter fearn",
            Ogham::LetterSail => "ogham letter sail",
            Ogham::LetterNion => "ogham letter nion",
            Ogham::LetterUath => "ogham letter uath",
            Ogham::LetterDair => "ogham letter dair",
            Ogham::LetterTinne => "ogham letter tinne",
            Ogham::LetterColl => "ogham letter coll",
            Ogham::LetterCeirt => "ogham letter ceirt",
            Ogham::LetterMuin => "ogham letter muin",
            Ogham::LetterGort => "ogham letter gort",
            Ogham::LetterNgeadal => "ogham letter ngeadal",
            Ogham::LetterStraif => "ogham letter straif",
            Ogham::LetterRuis => "ogham letter ruis",
            Ogham::LetterAilm => "ogham letter ailm",
            Ogham::LetterOnn => "ogham letter onn",
            Ogham::LetterUr => "ogham letter ur",
            Ogham::LetterEadhadh => "ogham letter eadhadh",
            Ogham::LetterIodhadh => "ogham letter iodhadh",
            Ogham::LetterEabhadh => "ogham letter eabhadh",
            Ogham::LetterOr => "ogham letter or",
            Ogham::LetterUilleann => "ogham letter uilleann",
            Ogham::LetterIfin => "ogham letter ifin",
            Ogham::LetterEamhancholl => "ogham letter eamhancholl",
            Ogham::LetterPeith => "ogham letter peith",
            Ogham::FeatherMark => "ogham feather mark",
            Ogham::ReversedFeatherMark => "ogham reversed feather mark",
        }
    }
}
