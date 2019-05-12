/// \u{10330} → \u{1034f}\
///\
/// 𐌰 𐌱 𐌲 𐌳 𐌴 𐌵 𐌶 𐌷 𐌸 𐌹 𐌺 𐌻 𐌼 𐌽 𐌾 𐌿
/// 𐍀 𐍁 𐍂 𐍃 𐍄 𐍅 𐍆 𐍇 𐍈 𐍉 𐍊
pub mod constants {
    /// \u{10330}: '𐌰'
    pub const LETTER_AHSA: char = '𐌰';
    /// \u{10331}: '𐌱'
    pub const LETTER_BAIRKAN: char = '𐌱';
    /// \u{10332}: '𐌲'
    pub const LETTER_GIBA: char = '𐌲';
    /// \u{10333}: '𐌳'
    pub const LETTER_DAGS: char = '𐌳';
    /// \u{10334}: '𐌴'
    pub const LETTER_AIHVUS: char = '𐌴';
    /// \u{10335}: '𐌵'
    pub const LETTER_QAIRTHRA: char = '𐌵';
    /// \u{10336}: '𐌶'
    pub const LETTER_IUJA: char = '𐌶';
    /// \u{10337}: '𐌷'
    pub const LETTER_HAGL: char = '𐌷';
    /// \u{10338}: '𐌸'
    pub const LETTER_THIUTH: char = '𐌸';
    /// \u{10339}: '𐌹'
    pub const LETTER_EIS: char = '𐌹';
    /// \u{1033a}: '𐌺'
    pub const LETTER_KUSMA: char = '𐌺';
    /// \u{1033b}: '𐌻'
    pub const LETTER_LAGUS: char = '𐌻';
    /// \u{1033c}: '𐌼'
    pub const LETTER_MANNA: char = '𐌼';
    /// \u{1033d}: '𐌽'
    pub const LETTER_NAUTHS: char = '𐌽';
    /// \u{1033e}: '𐌾'
    pub const LETTER_JER: char = '𐌾';
    /// \u{1033f}: '𐌿'
    pub const LETTER_URUS: char = '𐌿';
    /// \u{10340}: '𐍀'
    pub const LETTER_PAIRTHRA: char = '𐍀';
    /// \u{10341}: '𐍁'
    pub const LETTER_NINETY: char = '𐍁';
    /// \u{10342}: '𐍂'
    pub const LETTER_RAIDA: char = '𐍂';
    /// \u{10343}: '𐍃'
    pub const LETTER_SAUIL: char = '𐍃';
    /// \u{10344}: '𐍄'
    pub const LETTER_TEIWS: char = '𐍄';
    /// \u{10345}: '𐍅'
    pub const LETTER_WINJA: char = '𐍅';
    /// \u{10346}: '𐍆'
    pub const LETTER_FAIHU: char = '𐍆';
    /// \u{10347}: '𐍇'
    pub const LETTER_IGGWS: char = '𐍇';
    /// \u{10348}: '𐍈'
    pub const LETTER_HWAIR: char = '𐍈';
    /// \u{10349}: '𐍉'
    pub const LETTER_OTHAL: char = '𐍉';
    /// \u{1034a}: '𐍊'
    pub const LETTER_NINE_HUNDRED: char = '𐍊';
}

/// \u{10330} → \u{1034f}\
///\
/// 𐌰 𐌱 𐌲 𐌳 𐌴 𐌵 𐌶 𐌷 𐌸 𐌹 𐌺 𐌻 𐌼 𐌽 𐌾 𐌿
/// 𐍀 𐍁 𐍂 𐍃 𐍄 𐍅 𐍆 𐍇 𐍈 𐍉 𐍊
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Gothic {
    /// \u{10330}: '𐌰'
    LetterAhsa,
    /// \u{10331}: '𐌱'
    LetterBairkan,
    /// \u{10332}: '𐌲'
    LetterGiba,
    /// \u{10333}: '𐌳'
    LetterDags,
    /// \u{10334}: '𐌴'
    LetterAihvus,
    /// \u{10335}: '𐌵'
    LetterQairthra,
    /// \u{10336}: '𐌶'
    LetterIuja,
    /// \u{10337}: '𐌷'
    LetterHagl,
    /// \u{10338}: '𐌸'
    LetterThiuth,
    /// \u{10339}: '𐌹'
    LetterEis,
    /// \u{1033a}: '𐌺'
    LetterKusma,
    /// \u{1033b}: '𐌻'
    LetterLagus,
    /// \u{1033c}: '𐌼'
    LetterManna,
    /// \u{1033d}: '𐌽'
    LetterNauths,
    /// \u{1033e}: '𐌾'
    LetterJer,
    /// \u{1033f}: '𐌿'
    LetterUrus,
    /// \u{10340}: '𐍀'
    LetterPairthra,
    /// \u{10341}: '𐍁'
    LetterNinety,
    /// \u{10342}: '𐍂'
    LetterRaida,
    /// \u{10343}: '𐍃'
    LetterSauil,
    /// \u{10344}: '𐍄'
    LetterTeiws,
    /// \u{10345}: '𐍅'
    LetterWinja,
    /// \u{10346}: '𐍆'
    LetterFaihu,
    /// \u{10347}: '𐍇'
    LetterIggws,
    /// \u{10348}: '𐍈'
    LetterHwair,
    /// \u{10349}: '𐍉'
    LetterOthal,
    /// \u{1034a}: '𐍊'
    LetterNineHundred,
}

impl Into<char> for Gothic {
    fn into(self) -> char {
        use constants::*;
        match self {
            Gothic::LetterAhsa => LETTER_AHSA,
            Gothic::LetterBairkan => LETTER_BAIRKAN,
            Gothic::LetterGiba => LETTER_GIBA,
            Gothic::LetterDags => LETTER_DAGS,
            Gothic::LetterAihvus => LETTER_AIHVUS,
            Gothic::LetterQairthra => LETTER_QAIRTHRA,
            Gothic::LetterIuja => LETTER_IUJA,
            Gothic::LetterHagl => LETTER_HAGL,
            Gothic::LetterThiuth => LETTER_THIUTH,
            Gothic::LetterEis => LETTER_EIS,
            Gothic::LetterKusma => LETTER_KUSMA,
            Gothic::LetterLagus => LETTER_LAGUS,
            Gothic::LetterManna => LETTER_MANNA,
            Gothic::LetterNauths => LETTER_NAUTHS,
            Gothic::LetterJer => LETTER_JER,
            Gothic::LetterUrus => LETTER_URUS,
            Gothic::LetterPairthra => LETTER_PAIRTHRA,
            Gothic::LetterNinety => LETTER_NINETY,
            Gothic::LetterRaida => LETTER_RAIDA,
            Gothic::LetterSauil => LETTER_SAUIL,
            Gothic::LetterTeiws => LETTER_TEIWS,
            Gothic::LetterWinja => LETTER_WINJA,
            Gothic::LetterFaihu => LETTER_FAIHU,
            Gothic::LetterIggws => LETTER_IGGWS,
            Gothic::LetterHwair => LETTER_HWAIR,
            Gothic::LetterOthal => LETTER_OTHAL,
            Gothic::LetterNineHundred => LETTER_NINE_HUNDRED,
        }
    }
}

impl std::convert::TryFrom<char> for Gothic {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_AHSA => Ok(Gothic::LetterAhsa),
            LETTER_BAIRKAN => Ok(Gothic::LetterBairkan),
            LETTER_GIBA => Ok(Gothic::LetterGiba),
            LETTER_DAGS => Ok(Gothic::LetterDags),
            LETTER_AIHVUS => Ok(Gothic::LetterAihvus),
            LETTER_QAIRTHRA => Ok(Gothic::LetterQairthra),
            LETTER_IUJA => Ok(Gothic::LetterIuja),
            LETTER_HAGL => Ok(Gothic::LetterHagl),
            LETTER_THIUTH => Ok(Gothic::LetterThiuth),
            LETTER_EIS => Ok(Gothic::LetterEis),
            LETTER_KUSMA => Ok(Gothic::LetterKusma),
            LETTER_LAGUS => Ok(Gothic::LetterLagus),
            LETTER_MANNA => Ok(Gothic::LetterManna),
            LETTER_NAUTHS => Ok(Gothic::LetterNauths),
            LETTER_JER => Ok(Gothic::LetterJer),
            LETTER_URUS => Ok(Gothic::LetterUrus),
            LETTER_PAIRTHRA => Ok(Gothic::LetterPairthra),
            LETTER_NINETY => Ok(Gothic::LetterNinety),
            LETTER_RAIDA => Ok(Gothic::LetterRaida),
            LETTER_SAUIL => Ok(Gothic::LetterSauil),
            LETTER_TEIWS => Ok(Gothic::LetterTeiws),
            LETTER_WINJA => Ok(Gothic::LetterWinja),
            LETTER_FAIHU => Ok(Gothic::LetterFaihu),
            LETTER_IGGWS => Ok(Gothic::LetterIggws),
            LETTER_HWAIR => Ok(Gothic::LetterHwair),
            LETTER_OTHAL => Ok(Gothic::LetterOthal),
            LETTER_NINE_HUNDRED => Ok(Gothic::LetterNineHundred),
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
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Gothic::LetterAhsa
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Gothic::LetterAhsa => "gothic letter ahsa",
            Gothic::LetterBairkan => "gothic letter bairkan",
            Gothic::LetterGiba => "gothic letter giba",
            Gothic::LetterDags => "gothic letter dags",
            Gothic::LetterAihvus => "gothic letter aihvus",
            Gothic::LetterQairthra => "gothic letter qairthra",
            Gothic::LetterIuja => "gothic letter iuja",
            Gothic::LetterHagl => "gothic letter hagl",
            Gothic::LetterThiuth => "gothic letter thiuth",
            Gothic::LetterEis => "gothic letter eis",
            Gothic::LetterKusma => "gothic letter kusma",
            Gothic::LetterLagus => "gothic letter lagus",
            Gothic::LetterManna => "gothic letter manna",
            Gothic::LetterNauths => "gothic letter nauths",
            Gothic::LetterJer => "gothic letter jer",
            Gothic::LetterUrus => "gothic letter urus",
            Gothic::LetterPairthra => "gothic letter pairthra",
            Gothic::LetterNinety => "gothic letter ninety",
            Gothic::LetterRaida => "gothic letter raida",
            Gothic::LetterSauil => "gothic letter sauil",
            Gothic::LetterTeiws => "gothic letter teiws",
            Gothic::LetterWinja => "gothic letter winja",
            Gothic::LetterFaihu => "gothic letter faihu",
            Gothic::LetterIggws => "gothic letter iggws",
            Gothic::LetterHwair => "gothic letter hwair",
            Gothic::LetterOthal => "gothic letter othal",
            Gothic::LetterNineHundred => "gothic letter nine hundred",
        }
    }
}
