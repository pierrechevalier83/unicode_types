/// \u{1680} → \u{169f}\
///\
///   ᚁ ᚂ ᚃ ᚄ ᚅ ᚆ ᚇ ᚈ ᚉ ᚊ ᚋ ᚌ ᚍ ᚎ ᚏ
/// ᚐ ᚑ ᚒ ᚓ ᚔ ᚕ ᚖ ᚗ ᚘ ᚙ ᚚ ᚛ ᚜
pub mod constants {
    /// \u{1680}: ' '
    pub const OGHAM_SPACE_MARK: char = ' ';
    /// \u{1681}: 'ᚁ'
    pub const OGHAM_LETTER_BEITH: char = 'ᚁ';
    /// \u{1682}: 'ᚂ'
    pub const OGHAM_LETTER_LUIS: char = 'ᚂ';
    /// \u{1683}: 'ᚃ'
    pub const OGHAM_LETTER_FEARN: char = 'ᚃ';
    /// \u{1684}: 'ᚄ'
    pub const OGHAM_LETTER_SAIL: char = 'ᚄ';
    /// \u{1685}: 'ᚅ'
    pub const OGHAM_LETTER_NION: char = 'ᚅ';
    /// \u{1686}: 'ᚆ'
    pub const OGHAM_LETTER_UATH: char = 'ᚆ';
    /// \u{1687}: 'ᚇ'
    pub const OGHAM_LETTER_DAIR: char = 'ᚇ';
    /// \u{1688}: 'ᚈ'
    pub const OGHAM_LETTER_TINNE: char = 'ᚈ';
    /// \u{1689}: 'ᚉ'
    pub const OGHAM_LETTER_COLL: char = 'ᚉ';
    /// \u{168a}: 'ᚊ'
    pub const OGHAM_LETTER_CEIRT: char = 'ᚊ';
    /// \u{168b}: 'ᚋ'
    pub const OGHAM_LETTER_MUIN: char = 'ᚋ';
    /// \u{168c}: 'ᚌ'
    pub const OGHAM_LETTER_GORT: char = 'ᚌ';
    /// \u{168d}: 'ᚍ'
    pub const OGHAM_LETTER_NGEADAL: char = 'ᚍ';
    /// \u{168e}: 'ᚎ'
    pub const OGHAM_LETTER_STRAIF: char = 'ᚎ';
    /// \u{168f}: 'ᚏ'
    pub const OGHAM_LETTER_RUIS: char = 'ᚏ';
    /// \u{1690}: 'ᚐ'
    pub const OGHAM_LETTER_AILM: char = 'ᚐ';
    /// \u{1691}: 'ᚑ'
    pub const OGHAM_LETTER_ONN: char = 'ᚑ';
    /// \u{1692}: 'ᚒ'
    pub const OGHAM_LETTER_UR: char = 'ᚒ';
    /// \u{1693}: 'ᚓ'
    pub const OGHAM_LETTER_EADHADH: char = 'ᚓ';
    /// \u{1694}: 'ᚔ'
    pub const OGHAM_LETTER_IODHADH: char = 'ᚔ';
    /// \u{1695}: 'ᚕ'
    pub const OGHAM_LETTER_EABHADH: char = 'ᚕ';
    /// \u{1696}: 'ᚖ'
    pub const OGHAM_LETTER_OR: char = 'ᚖ';
    /// \u{1697}: 'ᚗ'
    pub const OGHAM_LETTER_UILLEANN: char = 'ᚗ';
    /// \u{1698}: 'ᚘ'
    pub const OGHAM_LETTER_IFIN: char = 'ᚘ';
    /// \u{1699}: 'ᚙ'
    pub const OGHAM_LETTER_EAMHANCHOLL: char = 'ᚙ';
    /// \u{169a}: 'ᚚ'
    pub const OGHAM_LETTER_PEITH: char = 'ᚚ';
    /// \u{169b}: '᚛'
    pub const OGHAM_FEATHER_MARK: char = '᚛';
    /// \u{169c}: '᚜'
    pub const OGHAM_REVERSED_FEATHER_MARK: char = '᚜';
}

/// \u{1680} → \u{169f}\
///\
///   ᚁ ᚂ ᚃ ᚄ ᚅ ᚆ ᚇ ᚈ ᚉ ᚊ ᚋ ᚌ ᚍ ᚎ ᚏ
/// ᚐ ᚑ ᚒ ᚓ ᚔ ᚕ ᚖ ᚗ ᚘ ᚙ ᚚ ᚛ ᚜
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Ogham {
    /// \u{1680}: ' '
    OghamSpaceMark,
    /// \u{1681}: 'ᚁ'
    OghamLetterBeith,
    /// \u{1682}: 'ᚂ'
    OghamLetterLuis,
    /// \u{1683}: 'ᚃ'
    OghamLetterFearn,
    /// \u{1684}: 'ᚄ'
    OghamLetterSail,
    /// \u{1685}: 'ᚅ'
    OghamLetterNion,
    /// \u{1686}: 'ᚆ'
    OghamLetterUath,
    /// \u{1687}: 'ᚇ'
    OghamLetterDair,
    /// \u{1688}: 'ᚈ'
    OghamLetterTinne,
    /// \u{1689}: 'ᚉ'
    OghamLetterColl,
    /// \u{168a}: 'ᚊ'
    OghamLetterCeirt,
    /// \u{168b}: 'ᚋ'
    OghamLetterMuin,
    /// \u{168c}: 'ᚌ'
    OghamLetterGort,
    /// \u{168d}: 'ᚍ'
    OghamLetterNgeadal,
    /// \u{168e}: 'ᚎ'
    OghamLetterStraif,
    /// \u{168f}: 'ᚏ'
    OghamLetterRuis,
    /// \u{1690}: 'ᚐ'
    OghamLetterAilm,
    /// \u{1691}: 'ᚑ'
    OghamLetterOnn,
    /// \u{1692}: 'ᚒ'
    OghamLetterUr,
    /// \u{1693}: 'ᚓ'
    OghamLetterEadhadh,
    /// \u{1694}: 'ᚔ'
    OghamLetterIodhadh,
    /// \u{1695}: 'ᚕ'
    OghamLetterEabhadh,
    /// \u{1696}: 'ᚖ'
    OghamLetterOr,
    /// \u{1697}: 'ᚗ'
    OghamLetterUilleann,
    /// \u{1698}: 'ᚘ'
    OghamLetterIfin,
    /// \u{1699}: 'ᚙ'
    OghamLetterEamhancholl,
    /// \u{169a}: 'ᚚ'
    OghamLetterPeith,
    /// \u{169b}: '᚛'
    OghamFeatherMark,
    /// \u{169c}: '᚜'
    OghamReversedFeatherMark,
}

impl Into<char> for Ogham {
    fn into(self) -> char {
        use constants::*;
        match self {
            Ogham::OghamSpaceMark => OGHAM_SPACE_MARK,
            Ogham::OghamLetterBeith => OGHAM_LETTER_BEITH,
            Ogham::OghamLetterLuis => OGHAM_LETTER_LUIS,
            Ogham::OghamLetterFearn => OGHAM_LETTER_FEARN,
            Ogham::OghamLetterSail => OGHAM_LETTER_SAIL,
            Ogham::OghamLetterNion => OGHAM_LETTER_NION,
            Ogham::OghamLetterUath => OGHAM_LETTER_UATH,
            Ogham::OghamLetterDair => OGHAM_LETTER_DAIR,
            Ogham::OghamLetterTinne => OGHAM_LETTER_TINNE,
            Ogham::OghamLetterColl => OGHAM_LETTER_COLL,
            Ogham::OghamLetterCeirt => OGHAM_LETTER_CEIRT,
            Ogham::OghamLetterMuin => OGHAM_LETTER_MUIN,
            Ogham::OghamLetterGort => OGHAM_LETTER_GORT,
            Ogham::OghamLetterNgeadal => OGHAM_LETTER_NGEADAL,
            Ogham::OghamLetterStraif => OGHAM_LETTER_STRAIF,
            Ogham::OghamLetterRuis => OGHAM_LETTER_RUIS,
            Ogham::OghamLetterAilm => OGHAM_LETTER_AILM,
            Ogham::OghamLetterOnn => OGHAM_LETTER_ONN,
            Ogham::OghamLetterUr => OGHAM_LETTER_UR,
            Ogham::OghamLetterEadhadh => OGHAM_LETTER_EADHADH,
            Ogham::OghamLetterIodhadh => OGHAM_LETTER_IODHADH,
            Ogham::OghamLetterEabhadh => OGHAM_LETTER_EABHADH,
            Ogham::OghamLetterOr => OGHAM_LETTER_OR,
            Ogham::OghamLetterUilleann => OGHAM_LETTER_UILLEANN,
            Ogham::OghamLetterIfin => OGHAM_LETTER_IFIN,
            Ogham::OghamLetterEamhancholl => OGHAM_LETTER_EAMHANCHOLL,
            Ogham::OghamLetterPeith => OGHAM_LETTER_PEITH,
            Ogham::OghamFeatherMark => OGHAM_FEATHER_MARK,
            Ogham::OghamReversedFeatherMark => OGHAM_REVERSED_FEATHER_MARK,
        }
    }
}

impl std::convert::TryFrom<char> for Ogham {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            OGHAM_SPACE_MARK => Ok(Ogham::OghamSpaceMark),
            OGHAM_LETTER_BEITH => Ok(Ogham::OghamLetterBeith),
            OGHAM_LETTER_LUIS => Ok(Ogham::OghamLetterLuis),
            OGHAM_LETTER_FEARN => Ok(Ogham::OghamLetterFearn),
            OGHAM_LETTER_SAIL => Ok(Ogham::OghamLetterSail),
            OGHAM_LETTER_NION => Ok(Ogham::OghamLetterNion),
            OGHAM_LETTER_UATH => Ok(Ogham::OghamLetterUath),
            OGHAM_LETTER_DAIR => Ok(Ogham::OghamLetterDair),
            OGHAM_LETTER_TINNE => Ok(Ogham::OghamLetterTinne),
            OGHAM_LETTER_COLL => Ok(Ogham::OghamLetterColl),
            OGHAM_LETTER_CEIRT => Ok(Ogham::OghamLetterCeirt),
            OGHAM_LETTER_MUIN => Ok(Ogham::OghamLetterMuin),
            OGHAM_LETTER_GORT => Ok(Ogham::OghamLetterGort),
            OGHAM_LETTER_NGEADAL => Ok(Ogham::OghamLetterNgeadal),
            OGHAM_LETTER_STRAIF => Ok(Ogham::OghamLetterStraif),
            OGHAM_LETTER_RUIS => Ok(Ogham::OghamLetterRuis),
            OGHAM_LETTER_AILM => Ok(Ogham::OghamLetterAilm),
            OGHAM_LETTER_ONN => Ok(Ogham::OghamLetterOnn),
            OGHAM_LETTER_UR => Ok(Ogham::OghamLetterUr),
            OGHAM_LETTER_EADHADH => Ok(Ogham::OghamLetterEadhadh),
            OGHAM_LETTER_IODHADH => Ok(Ogham::OghamLetterIodhadh),
            OGHAM_LETTER_EABHADH => Ok(Ogham::OghamLetterEabhadh),
            OGHAM_LETTER_OR => Ok(Ogham::OghamLetterOr),
            OGHAM_LETTER_UILLEANN => Ok(Ogham::OghamLetterUilleann),
            OGHAM_LETTER_IFIN => Ok(Ogham::OghamLetterIfin),
            OGHAM_LETTER_EAMHANCHOLL => Ok(Ogham::OghamLetterEamhancholl),
            OGHAM_LETTER_PEITH => Ok(Ogham::OghamLetterPeith),
            OGHAM_FEATHER_MARK => Ok(Ogham::OghamFeatherMark),
            OGHAM_REVERSED_FEATHER_MARK => Ok(Ogham::OghamReversedFeatherMark),
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
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        Ogham::OghamSpaceMark
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Ogham::OghamSpaceMark => "ogham space mark",
            Ogham::OghamLetterBeith => "ogham letter beith",
            Ogham::OghamLetterLuis => "ogham letter luis",
            Ogham::OghamLetterFearn => "ogham letter fearn",
            Ogham::OghamLetterSail => "ogham letter sail",
            Ogham::OghamLetterNion => "ogham letter nion",
            Ogham::OghamLetterUath => "ogham letter uath",
            Ogham::OghamLetterDair => "ogham letter dair",
            Ogham::OghamLetterTinne => "ogham letter tinne",
            Ogham::OghamLetterColl => "ogham letter coll",
            Ogham::OghamLetterCeirt => "ogham letter ceirt",
            Ogham::OghamLetterMuin => "ogham letter muin",
            Ogham::OghamLetterGort => "ogham letter gort",
            Ogham::OghamLetterNgeadal => "ogham letter ngeadal",
            Ogham::OghamLetterStraif => "ogham letter straif",
            Ogham::OghamLetterRuis => "ogham letter ruis",
            Ogham::OghamLetterAilm => "ogham letter ailm",
            Ogham::OghamLetterOnn => "ogham letter onn",
            Ogham::OghamLetterUr => "ogham letter ur",
            Ogham::OghamLetterEadhadh => "ogham letter eadhadh",
            Ogham::OghamLetterIodhadh => "ogham letter iodhadh",
            Ogham::OghamLetterEabhadh => "ogham letter eabhadh",
            Ogham::OghamLetterOr => "ogham letter or",
            Ogham::OghamLetterUilleann => "ogham letter uilleann",
            Ogham::OghamLetterIfin => "ogham letter ifin",
            Ogham::OghamLetterEamhancholl => "ogham letter eamhancholl",
            Ogham::OghamLetterPeith => "ogham letter peith",
            Ogham::OghamFeatherMark => "ogham feather mark",
            Ogham::OghamReversedFeatherMark => "ogham reversed feather mark",
        }
    }
}
