/// \u{10330} → \u{1034f}\
///\
/// 𐌰 𐌱 𐌲 𐌳 𐌴 𐌵 𐌶 𐌷 𐌸 𐌹 𐌺 𐌻 𐌼 𐌽 𐌾 𐌿
/// 𐍀 𐍁 𐍂 𐍃 𐍄 𐍅 𐍆 𐍇 𐍈 𐍉 𐍊
pub mod constants {
    /// \u{10330}: '𐌰'
    pub const GOTHIC_LETTER_AHSA: char = '𐌰';
    /// \u{10331}: '𐌱'
    pub const GOTHIC_LETTER_BAIRKAN: char = '𐌱';
    /// \u{10332}: '𐌲'
    pub const GOTHIC_LETTER_GIBA: char = '𐌲';
    /// \u{10333}: '𐌳'
    pub const GOTHIC_LETTER_DAGS: char = '𐌳';
    /// \u{10334}: '𐌴'
    pub const GOTHIC_LETTER_AIHVUS: char = '𐌴';
    /// \u{10335}: '𐌵'
    pub const GOTHIC_LETTER_QAIRTHRA: char = '𐌵';
    /// \u{10336}: '𐌶'
    pub const GOTHIC_LETTER_IUJA: char = '𐌶';
    /// \u{10337}: '𐌷'
    pub const GOTHIC_LETTER_HAGL: char = '𐌷';
    /// \u{10338}: '𐌸'
    pub const GOTHIC_LETTER_THIUTH: char = '𐌸';
    /// \u{10339}: '𐌹'
    pub const GOTHIC_LETTER_EIS: char = '𐌹';
    /// \u{1033a}: '𐌺'
    pub const GOTHIC_LETTER_KUSMA: char = '𐌺';
    /// \u{1033b}: '𐌻'
    pub const GOTHIC_LETTER_LAGUS: char = '𐌻';
    /// \u{1033c}: '𐌼'
    pub const GOTHIC_LETTER_MANNA: char = '𐌼';
    /// \u{1033d}: '𐌽'
    pub const GOTHIC_LETTER_NAUTHS: char = '𐌽';
    /// \u{1033e}: '𐌾'
    pub const GOTHIC_LETTER_JER: char = '𐌾';
    /// \u{1033f}: '𐌿'
    pub const GOTHIC_LETTER_URUS: char = '𐌿';
    /// \u{10340}: '𐍀'
    pub const GOTHIC_LETTER_PAIRTHRA: char = '𐍀';
    /// \u{10341}: '𐍁'
    pub const GOTHIC_LETTER_NINETY: char = '𐍁';
    /// \u{10342}: '𐍂'
    pub const GOTHIC_LETTER_RAIDA: char = '𐍂';
    /// \u{10343}: '𐍃'
    pub const GOTHIC_LETTER_SAUIL: char = '𐍃';
    /// \u{10344}: '𐍄'
    pub const GOTHIC_LETTER_TEIWS: char = '𐍄';
    /// \u{10345}: '𐍅'
    pub const GOTHIC_LETTER_WINJA: char = '𐍅';
    /// \u{10346}: '𐍆'
    pub const GOTHIC_LETTER_FAIHU: char = '𐍆';
    /// \u{10347}: '𐍇'
    pub const GOTHIC_LETTER_IGGWS: char = '𐍇';
    /// \u{10348}: '𐍈'
    pub const GOTHIC_LETTER_HWAIR: char = '𐍈';
    /// \u{10349}: '𐍉'
    pub const GOTHIC_LETTER_OTHAL: char = '𐍉';
    /// \u{1034a}: '𐍊'
    pub const GOTHIC_LETTER_NINE_HUNDRED: char = '𐍊';
}

/// \u{10330} → \u{1034f}\
///\
/// 𐌰 𐌱 𐌲 𐌳 𐌴 𐌵 𐌶 𐌷 𐌸 𐌹 𐌺 𐌻 𐌼 𐌽 𐌾 𐌿
/// 𐍀 𐍁 𐍂 𐍃 𐍄 𐍅 𐍆 𐍇 𐍈 𐍉 𐍊
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Gothic {
    /// \u{10330}: '𐌰'
    GothicLetterAhsa,
    /// \u{10331}: '𐌱'
    GothicLetterBairkan,
    /// \u{10332}: '𐌲'
    GothicLetterGiba,
    /// \u{10333}: '𐌳'
    GothicLetterDags,
    /// \u{10334}: '𐌴'
    GothicLetterAihvus,
    /// \u{10335}: '𐌵'
    GothicLetterQairthra,
    /// \u{10336}: '𐌶'
    GothicLetterIuja,
    /// \u{10337}: '𐌷'
    GothicLetterHagl,
    /// \u{10338}: '𐌸'
    GothicLetterThiuth,
    /// \u{10339}: '𐌹'
    GothicLetterEis,
    /// \u{1033a}: '𐌺'
    GothicLetterKusma,
    /// \u{1033b}: '𐌻'
    GothicLetterLagus,
    /// \u{1033c}: '𐌼'
    GothicLetterManna,
    /// \u{1033d}: '𐌽'
    GothicLetterNauths,
    /// \u{1033e}: '𐌾'
    GothicLetterJer,
    /// \u{1033f}: '𐌿'
    GothicLetterUrus,
    /// \u{10340}: '𐍀'
    GothicLetterPairthra,
    /// \u{10341}: '𐍁'
    GothicLetterNinety,
    /// \u{10342}: '𐍂'
    GothicLetterRaida,
    /// \u{10343}: '𐍃'
    GothicLetterSauil,
    /// \u{10344}: '𐍄'
    GothicLetterTeiws,
    /// \u{10345}: '𐍅'
    GothicLetterWinja,
    /// \u{10346}: '𐍆'
    GothicLetterFaihu,
    /// \u{10347}: '𐍇'
    GothicLetterIggws,
    /// \u{10348}: '𐍈'
    GothicLetterHwair,
    /// \u{10349}: '𐍉'
    GothicLetterOthal,
    /// \u{1034a}: '𐍊'
    GothicLetterNineHundred,
}

impl Into<char> for Gothic {
    fn into(self) -> char {
        use constants::*;
        match self {
            Gothic::GothicLetterAhsa => GOTHIC_LETTER_AHSA,
            Gothic::GothicLetterBairkan => GOTHIC_LETTER_BAIRKAN,
            Gothic::GothicLetterGiba => GOTHIC_LETTER_GIBA,
            Gothic::GothicLetterDags => GOTHIC_LETTER_DAGS,
            Gothic::GothicLetterAihvus => GOTHIC_LETTER_AIHVUS,
            Gothic::GothicLetterQairthra => GOTHIC_LETTER_QAIRTHRA,
            Gothic::GothicLetterIuja => GOTHIC_LETTER_IUJA,
            Gothic::GothicLetterHagl => GOTHIC_LETTER_HAGL,
            Gothic::GothicLetterThiuth => GOTHIC_LETTER_THIUTH,
            Gothic::GothicLetterEis => GOTHIC_LETTER_EIS,
            Gothic::GothicLetterKusma => GOTHIC_LETTER_KUSMA,
            Gothic::GothicLetterLagus => GOTHIC_LETTER_LAGUS,
            Gothic::GothicLetterManna => GOTHIC_LETTER_MANNA,
            Gothic::GothicLetterNauths => GOTHIC_LETTER_NAUTHS,
            Gothic::GothicLetterJer => GOTHIC_LETTER_JER,
            Gothic::GothicLetterUrus => GOTHIC_LETTER_URUS,
            Gothic::GothicLetterPairthra => GOTHIC_LETTER_PAIRTHRA,
            Gothic::GothicLetterNinety => GOTHIC_LETTER_NINETY,
            Gothic::GothicLetterRaida => GOTHIC_LETTER_RAIDA,
            Gothic::GothicLetterSauil => GOTHIC_LETTER_SAUIL,
            Gothic::GothicLetterTeiws => GOTHIC_LETTER_TEIWS,
            Gothic::GothicLetterWinja => GOTHIC_LETTER_WINJA,
            Gothic::GothicLetterFaihu => GOTHIC_LETTER_FAIHU,
            Gothic::GothicLetterIggws => GOTHIC_LETTER_IGGWS,
            Gothic::GothicLetterHwair => GOTHIC_LETTER_HWAIR,
            Gothic::GothicLetterOthal => GOTHIC_LETTER_OTHAL,
            Gothic::GothicLetterNineHundred => GOTHIC_LETTER_NINE_HUNDRED,
        }
    }
}

impl std::convert::TryFrom<char> for Gothic {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            GOTHIC_LETTER_AHSA => Ok(Gothic::GothicLetterAhsa),
            GOTHIC_LETTER_BAIRKAN => Ok(Gothic::GothicLetterBairkan),
            GOTHIC_LETTER_GIBA => Ok(Gothic::GothicLetterGiba),
            GOTHIC_LETTER_DAGS => Ok(Gothic::GothicLetterDags),
            GOTHIC_LETTER_AIHVUS => Ok(Gothic::GothicLetterAihvus),
            GOTHIC_LETTER_QAIRTHRA => Ok(Gothic::GothicLetterQairthra),
            GOTHIC_LETTER_IUJA => Ok(Gothic::GothicLetterIuja),
            GOTHIC_LETTER_HAGL => Ok(Gothic::GothicLetterHagl),
            GOTHIC_LETTER_THIUTH => Ok(Gothic::GothicLetterThiuth),
            GOTHIC_LETTER_EIS => Ok(Gothic::GothicLetterEis),
            GOTHIC_LETTER_KUSMA => Ok(Gothic::GothicLetterKusma),
            GOTHIC_LETTER_LAGUS => Ok(Gothic::GothicLetterLagus),
            GOTHIC_LETTER_MANNA => Ok(Gothic::GothicLetterManna),
            GOTHIC_LETTER_NAUTHS => Ok(Gothic::GothicLetterNauths),
            GOTHIC_LETTER_JER => Ok(Gothic::GothicLetterJer),
            GOTHIC_LETTER_URUS => Ok(Gothic::GothicLetterUrus),
            GOTHIC_LETTER_PAIRTHRA => Ok(Gothic::GothicLetterPairthra),
            GOTHIC_LETTER_NINETY => Ok(Gothic::GothicLetterNinety),
            GOTHIC_LETTER_RAIDA => Ok(Gothic::GothicLetterRaida),
            GOTHIC_LETTER_SAUIL => Ok(Gothic::GothicLetterSauil),
            GOTHIC_LETTER_TEIWS => Ok(Gothic::GothicLetterTeiws),
            GOTHIC_LETTER_WINJA => Ok(Gothic::GothicLetterWinja),
            GOTHIC_LETTER_FAIHU => Ok(Gothic::GothicLetterFaihu),
            GOTHIC_LETTER_IGGWS => Ok(Gothic::GothicLetterIggws),
            GOTHIC_LETTER_HWAIR => Ok(Gothic::GothicLetterHwair),
            GOTHIC_LETTER_OTHAL => Ok(Gothic::GothicLetterOthal),
            GOTHIC_LETTER_NINE_HUNDRED => Ok(Gothic::GothicLetterNineHundred),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Gothic {
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

impl std::convert::TryFrom<u32> for Gothic {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Gothic {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Gothic {
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        Gothic::GothicLetterAhsa
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Gothic::GothicLetterAhsa => "gothic letter ahsa",
            Gothic::GothicLetterBairkan => "gothic letter bairkan",
            Gothic::GothicLetterGiba => "gothic letter giba",
            Gothic::GothicLetterDags => "gothic letter dags",
            Gothic::GothicLetterAihvus => "gothic letter aihvus",
            Gothic::GothicLetterQairthra => "gothic letter qairthra",
            Gothic::GothicLetterIuja => "gothic letter iuja",
            Gothic::GothicLetterHagl => "gothic letter hagl",
            Gothic::GothicLetterThiuth => "gothic letter thiuth",
            Gothic::GothicLetterEis => "gothic letter eis",
            Gothic::GothicLetterKusma => "gothic letter kusma",
            Gothic::GothicLetterLagus => "gothic letter lagus",
            Gothic::GothicLetterManna => "gothic letter manna",
            Gothic::GothicLetterNauths => "gothic letter nauths",
            Gothic::GothicLetterJer => "gothic letter jer",
            Gothic::GothicLetterUrus => "gothic letter urus",
            Gothic::GothicLetterPairthra => "gothic letter pairthra",
            Gothic::GothicLetterNinety => "gothic letter ninety",
            Gothic::GothicLetterRaida => "gothic letter raida",
            Gothic::GothicLetterSauil => "gothic letter sauil",
            Gothic::GothicLetterTeiws => "gothic letter teiws",
            Gothic::GothicLetterWinja => "gothic letter winja",
            Gothic::GothicLetterFaihu => "gothic letter faihu",
            Gothic::GothicLetterIggws => "gothic letter iggws",
            Gothic::GothicLetterHwair => "gothic letter hwair",
            Gothic::GothicLetterOthal => "gothic letter othal",
            Gothic::GothicLetterNineHundred => "gothic letter nine hundred",
        }
    }
}
