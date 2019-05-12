/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{1680}: ' '
    pub const SPACE_MARK: char = ' ';
    /// \u{1681}: 'ᚁ'
    pub const LETTER_BEITH: char = 'ᚁ';
    /// \u{1682}: 'ᚂ'
    pub const LETTER_LUIS: char = 'ᚂ';
    /// \u{1683}: 'ᚃ'
    pub const LETTER_FEARN: char = 'ᚃ';
    /// \u{1684}: 'ᚄ'
    pub const LETTER_SAIL: char = 'ᚄ';
    /// \u{1685}: 'ᚅ'
    pub const LETTER_NION: char = 'ᚅ';
    /// \u{1686}: 'ᚆ'
    pub const LETTER_UATH: char = 'ᚆ';
    /// \u{1687}: 'ᚇ'
    pub const LETTER_DAIR: char = 'ᚇ';
    /// \u{1688}: 'ᚈ'
    pub const LETTER_TINNE: char = 'ᚈ';
    /// \u{1689}: 'ᚉ'
    pub const LETTER_COLL: char = 'ᚉ';
    /// \u{168a}: 'ᚊ'
    pub const LETTER_CEIRT: char = 'ᚊ';
    /// \u{168b}: 'ᚋ'
    pub const LETTER_MUIN: char = 'ᚋ';
    /// \u{168c}: 'ᚌ'
    pub const LETTER_GORT: char = 'ᚌ';
    /// \u{168d}: 'ᚍ'
    pub const LETTER_NGEADAL: char = 'ᚍ';
    /// \u{168e}: 'ᚎ'
    pub const LETTER_STRAIF: char = 'ᚎ';
    /// \u{168f}: 'ᚏ'
    pub const LETTER_RUIS: char = 'ᚏ';
    /// \u{1690}: 'ᚐ'
    pub const LETTER_AILM: char = 'ᚐ';
    /// \u{1691}: 'ᚑ'
    pub const LETTER_ONN: char = 'ᚑ';
    /// \u{1692}: 'ᚒ'
    pub const LETTER_UR: char = 'ᚒ';
    /// \u{1693}: 'ᚓ'
    pub const LETTER_EADHADH: char = 'ᚓ';
    /// \u{1694}: 'ᚔ'
    pub const LETTER_IODHADH: char = 'ᚔ';
    /// \u{1695}: 'ᚕ'
    pub const LETTER_EABHADH: char = 'ᚕ';
    /// \u{1696}: 'ᚖ'
    pub const LETTER_OR: char = 'ᚖ';
    /// \u{1697}: 'ᚗ'
    pub const LETTER_UILLEANN: char = 'ᚗ';
    /// \u{1698}: 'ᚘ'
    pub const LETTER_IFIN: char = 'ᚘ';
    /// \u{1699}: 'ᚙ'
    pub const LETTER_EAMHANCHOLL: char = 'ᚙ';
    /// \u{169a}: 'ᚚ'
    pub const LETTER_PEITH: char = 'ᚚ';
    /// \u{169b}: '᚛'
    pub const FEATHER_MARK: char = '᚛';
    /// \u{169c}: '᚜'
    pub const REVERSED_FEATHER_MARK: char = '᚜';
}

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
        use constants::*;
        match self {
            Ogham::SpaceMark => SPACE_MARK,
            Ogham::LetterBeith => LETTER_BEITH,
            Ogham::LetterLuis => LETTER_LUIS,
            Ogham::LetterFearn => LETTER_FEARN,
            Ogham::LetterSail => LETTER_SAIL,
            Ogham::LetterNion => LETTER_NION,
            Ogham::LetterUath => LETTER_UATH,
            Ogham::LetterDair => LETTER_DAIR,
            Ogham::LetterTinne => LETTER_TINNE,
            Ogham::LetterColl => LETTER_COLL,
            Ogham::LetterCeirt => LETTER_CEIRT,
            Ogham::LetterMuin => LETTER_MUIN,
            Ogham::LetterGort => LETTER_GORT,
            Ogham::LetterNgeadal => LETTER_NGEADAL,
            Ogham::LetterStraif => LETTER_STRAIF,
            Ogham::LetterRuis => LETTER_RUIS,
            Ogham::LetterAilm => LETTER_AILM,
            Ogham::LetterOnn => LETTER_ONN,
            Ogham::LetterUr => LETTER_UR,
            Ogham::LetterEadhadh => LETTER_EADHADH,
            Ogham::LetterIodhadh => LETTER_IODHADH,
            Ogham::LetterEabhadh => LETTER_EABHADH,
            Ogham::LetterOr => LETTER_OR,
            Ogham::LetterUilleann => LETTER_UILLEANN,
            Ogham::LetterIfin => LETTER_IFIN,
            Ogham::LetterEamhancholl => LETTER_EAMHANCHOLL,
            Ogham::LetterPeith => LETTER_PEITH,
            Ogham::FeatherMark => FEATHER_MARK,
            Ogham::ReversedFeatherMark => REVERSED_FEATHER_MARK,
        }
    }
}

impl std::convert::TryFrom<char> for Ogham {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            SPACE_MARK => Ok(Ogham::SpaceMark),
            LETTER_BEITH => Ok(Ogham::LetterBeith),
            LETTER_LUIS => Ok(Ogham::LetterLuis),
            LETTER_FEARN => Ok(Ogham::LetterFearn),
            LETTER_SAIL => Ok(Ogham::LetterSail),
            LETTER_NION => Ok(Ogham::LetterNion),
            LETTER_UATH => Ok(Ogham::LetterUath),
            LETTER_DAIR => Ok(Ogham::LetterDair),
            LETTER_TINNE => Ok(Ogham::LetterTinne),
            LETTER_COLL => Ok(Ogham::LetterColl),
            LETTER_CEIRT => Ok(Ogham::LetterCeirt),
            LETTER_MUIN => Ok(Ogham::LetterMuin),
            LETTER_GORT => Ok(Ogham::LetterGort),
            LETTER_NGEADAL => Ok(Ogham::LetterNgeadal),
            LETTER_STRAIF => Ok(Ogham::LetterStraif),
            LETTER_RUIS => Ok(Ogham::LetterRuis),
            LETTER_AILM => Ok(Ogham::LetterAilm),
            LETTER_ONN => Ok(Ogham::LetterOnn),
            LETTER_UR => Ok(Ogham::LetterUr),
            LETTER_EADHADH => Ok(Ogham::LetterEadhadh),
            LETTER_IODHADH => Ok(Ogham::LetterIodhadh),
            LETTER_EABHADH => Ok(Ogham::LetterEabhadh),
            LETTER_OR => Ok(Ogham::LetterOr),
            LETTER_UILLEANN => Ok(Ogham::LetterUilleann),
            LETTER_IFIN => Ok(Ogham::LetterIfin),
            LETTER_EAMHANCHOLL => Ok(Ogham::LetterEamhancholl),
            LETTER_PEITH => Ok(Ogham::LetterPeith),
            FEATHER_MARK => Ok(Ogham::FeatherMark),
            REVERSED_FEATHER_MARK => Ok(Ogham::ReversedFeatherMark),
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
