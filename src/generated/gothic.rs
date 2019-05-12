
/// An enum to represent all characters in the Gothic block.
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
        match self {
            Gothic::LetterAhsa => '𐌰',
            Gothic::LetterBairkan => '𐌱',
            Gothic::LetterGiba => '𐌲',
            Gothic::LetterDags => '𐌳',
            Gothic::LetterAihvus => '𐌴',
            Gothic::LetterQairthra => '𐌵',
            Gothic::LetterIuja => '𐌶',
            Gothic::LetterHagl => '𐌷',
            Gothic::LetterThiuth => '𐌸',
            Gothic::LetterEis => '𐌹',
            Gothic::LetterKusma => '𐌺',
            Gothic::LetterLagus => '𐌻',
            Gothic::LetterManna => '𐌼',
            Gothic::LetterNauths => '𐌽',
            Gothic::LetterJer => '𐌾',
            Gothic::LetterUrus => '𐌿',
            Gothic::LetterPairthra => '𐍀',
            Gothic::LetterNinety => '𐍁',
            Gothic::LetterRaida => '𐍂',
            Gothic::LetterSauil => '𐍃',
            Gothic::LetterTeiws => '𐍄',
            Gothic::LetterWinja => '𐍅',
            Gothic::LetterFaihu => '𐍆',
            Gothic::LetterIggws => '𐍇',
            Gothic::LetterHwair => '𐍈',
            Gothic::LetterOthal => '𐍉',
            Gothic::LetterNineHundred => '𐍊',
        }
    }
}

impl std::convert::TryFrom<char> for Gothic {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𐌰' => Ok(Gothic::LetterAhsa),
            '𐌱' => Ok(Gothic::LetterBairkan),
            '𐌲' => Ok(Gothic::LetterGiba),
            '𐌳' => Ok(Gothic::LetterDags),
            '𐌴' => Ok(Gothic::LetterAihvus),
            '𐌵' => Ok(Gothic::LetterQairthra),
            '𐌶' => Ok(Gothic::LetterIuja),
            '𐌷' => Ok(Gothic::LetterHagl),
            '𐌸' => Ok(Gothic::LetterThiuth),
            '𐌹' => Ok(Gothic::LetterEis),
            '𐌺' => Ok(Gothic::LetterKusma),
            '𐌻' => Ok(Gothic::LetterLagus),
            '𐌼' => Ok(Gothic::LetterManna),
            '𐌽' => Ok(Gothic::LetterNauths),
            '𐌾' => Ok(Gothic::LetterJer),
            '𐌿' => Ok(Gothic::LetterUrus),
            '𐍀' => Ok(Gothic::LetterPairthra),
            '𐍁' => Ok(Gothic::LetterNinety),
            '𐍂' => Ok(Gothic::LetterRaida),
            '𐍃' => Ok(Gothic::LetterSauil),
            '𐍄' => Ok(Gothic::LetterTeiws),
            '𐍅' => Ok(Gothic::LetterWinja),
            '𐍆' => Ok(Gothic::LetterFaihu),
            '𐍇' => Ok(Gothic::LetterIggws),
            '𐍈' => Ok(Gothic::LetterHwair),
            '𐍉' => Ok(Gothic::LetterOthal),
            '𐍊' => Ok(Gothic::LetterNineHundred),
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

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Gothic{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
