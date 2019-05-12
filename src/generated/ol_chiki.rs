
/// An enum to represent all characters in the OlChiki block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum OlChiki {
    /// \u{1c50}: '᱐'
    DigitZero,
    /// \u{1c51}: '᱑'
    DigitOne,
    /// \u{1c52}: '᱒'
    DigitTwo,
    /// \u{1c53}: '᱓'
    DigitThree,
    /// \u{1c54}: '᱔'
    DigitFour,
    /// \u{1c55}: '᱕'
    DigitFive,
    /// \u{1c56}: '᱖'
    DigitSix,
    /// \u{1c57}: '᱗'
    DigitSeven,
    /// \u{1c58}: '᱘'
    DigitEight,
    /// \u{1c59}: '᱙'
    DigitNine,
    /// \u{1c5a}: 'ᱚ'
    LetterLa,
    /// \u{1c5b}: 'ᱛ'
    LetterAt,
    /// \u{1c5c}: 'ᱜ'
    LetterAg,
    /// \u{1c5d}: 'ᱝ'
    LetterAng,
    /// \u{1c5e}: 'ᱞ'
    LetterAl,
    /// \u{1c5f}: 'ᱟ'
    LetterLaa,
    /// \u{1c60}: 'ᱠ'
    LetterAak,
    /// \u{1c61}: 'ᱡ'
    LetterAaj,
    /// \u{1c62}: 'ᱢ'
    LetterAam,
    /// \u{1c63}: 'ᱣ'
    LetterAaw,
    /// \u{1c64}: 'ᱤ'
    LetterLi,
    /// \u{1c65}: 'ᱥ'
    LetterIs,
    /// \u{1c66}: 'ᱦ'
    LetterIh,
    /// \u{1c67}: 'ᱧ'
    LetterIny,
    /// \u{1c68}: 'ᱨ'
    LetterIr,
    /// \u{1c69}: 'ᱩ'
    LetterLu,
    /// \u{1c6a}: 'ᱪ'
    LetterUc,
    /// \u{1c6b}: 'ᱫ'
    LetterUd,
    /// \u{1c6c}: 'ᱬ'
    LetterUnn,
    /// \u{1c6d}: 'ᱭ'
    LetterUy,
    /// \u{1c6e}: 'ᱮ'
    LetterLe,
    /// \u{1c6f}: 'ᱯ'
    LetterEp,
    /// \u{1c70}: 'ᱰ'
    LetterEdd,
    /// \u{1c71}: 'ᱱ'
    LetterEn,
    /// \u{1c72}: 'ᱲ'
    LetterErr,
    /// \u{1c73}: 'ᱳ'
    LetterLo,
    /// \u{1c74}: 'ᱴ'
    LetterOtt,
    /// \u{1c75}: 'ᱵ'
    LetterOb,
    /// \u{1c76}: 'ᱶ'
    LetterOv,
    /// \u{1c77}: 'ᱷ'
    LetterOh,
    /// \u{1c78}: 'ᱸ'
    MuTtuddag,
    /// \u{1c79}: 'ᱹ'
    GaahlaaTtuddaag,
    /// \u{1c7a}: 'ᱺ'
    MuDashGaahlaaTtuddaag,
    /// \u{1c7b}: 'ᱻ'
    Relaa,
    /// \u{1c7c}: 'ᱼ'
    Phaarkaa,
    /// \u{1c7d}: 'ᱽ'
    Ahad,
    /// \u{1c7e}: '᱾'
    PunctuationMucaad,
}

impl Into<char> for OlChiki {
    fn into(self) -> char {
        match self {
            OlChiki::DigitZero => '᱐',
            OlChiki::DigitOne => '᱑',
            OlChiki::DigitTwo => '᱒',
            OlChiki::DigitThree => '᱓',
            OlChiki::DigitFour => '᱔',
            OlChiki::DigitFive => '᱕',
            OlChiki::DigitSix => '᱖',
            OlChiki::DigitSeven => '᱗',
            OlChiki::DigitEight => '᱘',
            OlChiki::DigitNine => '᱙',
            OlChiki::LetterLa => 'ᱚ',
            OlChiki::LetterAt => 'ᱛ',
            OlChiki::LetterAg => 'ᱜ',
            OlChiki::LetterAng => 'ᱝ',
            OlChiki::LetterAl => 'ᱞ',
            OlChiki::LetterLaa => 'ᱟ',
            OlChiki::LetterAak => 'ᱠ',
            OlChiki::LetterAaj => 'ᱡ',
            OlChiki::LetterAam => 'ᱢ',
            OlChiki::LetterAaw => 'ᱣ',
            OlChiki::LetterLi => 'ᱤ',
            OlChiki::LetterIs => 'ᱥ',
            OlChiki::LetterIh => 'ᱦ',
            OlChiki::LetterIny => 'ᱧ',
            OlChiki::LetterIr => 'ᱨ',
            OlChiki::LetterLu => 'ᱩ',
            OlChiki::LetterUc => 'ᱪ',
            OlChiki::LetterUd => 'ᱫ',
            OlChiki::LetterUnn => 'ᱬ',
            OlChiki::LetterUy => 'ᱭ',
            OlChiki::LetterLe => 'ᱮ',
            OlChiki::LetterEp => 'ᱯ',
            OlChiki::LetterEdd => 'ᱰ',
            OlChiki::LetterEn => 'ᱱ',
            OlChiki::LetterErr => 'ᱲ',
            OlChiki::LetterLo => 'ᱳ',
            OlChiki::LetterOtt => 'ᱴ',
            OlChiki::LetterOb => 'ᱵ',
            OlChiki::LetterOv => 'ᱶ',
            OlChiki::LetterOh => 'ᱷ',
            OlChiki::MuTtuddag => 'ᱸ',
            OlChiki::GaahlaaTtuddaag => 'ᱹ',
            OlChiki::MuDashGaahlaaTtuddaag => 'ᱺ',
            OlChiki::Relaa => 'ᱻ',
            OlChiki::Phaarkaa => 'ᱼ',
            OlChiki::Ahad => 'ᱽ',
            OlChiki::PunctuationMucaad => '᱾',
        }
    }
}

impl std::convert::TryFrom<char> for OlChiki {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '᱐' => Ok(OlChiki::DigitZero),
            '᱑' => Ok(OlChiki::DigitOne),
            '᱒' => Ok(OlChiki::DigitTwo),
            '᱓' => Ok(OlChiki::DigitThree),
            '᱔' => Ok(OlChiki::DigitFour),
            '᱕' => Ok(OlChiki::DigitFive),
            '᱖' => Ok(OlChiki::DigitSix),
            '᱗' => Ok(OlChiki::DigitSeven),
            '᱘' => Ok(OlChiki::DigitEight),
            '᱙' => Ok(OlChiki::DigitNine),
            'ᱚ' => Ok(OlChiki::LetterLa),
            'ᱛ' => Ok(OlChiki::LetterAt),
            'ᱜ' => Ok(OlChiki::LetterAg),
            'ᱝ' => Ok(OlChiki::LetterAng),
            'ᱞ' => Ok(OlChiki::LetterAl),
            'ᱟ' => Ok(OlChiki::LetterLaa),
            'ᱠ' => Ok(OlChiki::LetterAak),
            'ᱡ' => Ok(OlChiki::LetterAaj),
            'ᱢ' => Ok(OlChiki::LetterAam),
            'ᱣ' => Ok(OlChiki::LetterAaw),
            'ᱤ' => Ok(OlChiki::LetterLi),
            'ᱥ' => Ok(OlChiki::LetterIs),
            'ᱦ' => Ok(OlChiki::LetterIh),
            'ᱧ' => Ok(OlChiki::LetterIny),
            'ᱨ' => Ok(OlChiki::LetterIr),
            'ᱩ' => Ok(OlChiki::LetterLu),
            'ᱪ' => Ok(OlChiki::LetterUc),
            'ᱫ' => Ok(OlChiki::LetterUd),
            'ᱬ' => Ok(OlChiki::LetterUnn),
            'ᱭ' => Ok(OlChiki::LetterUy),
            'ᱮ' => Ok(OlChiki::LetterLe),
            'ᱯ' => Ok(OlChiki::LetterEp),
            'ᱰ' => Ok(OlChiki::LetterEdd),
            'ᱱ' => Ok(OlChiki::LetterEn),
            'ᱲ' => Ok(OlChiki::LetterErr),
            'ᱳ' => Ok(OlChiki::LetterLo),
            'ᱴ' => Ok(OlChiki::LetterOtt),
            'ᱵ' => Ok(OlChiki::LetterOb),
            'ᱶ' => Ok(OlChiki::LetterOv),
            'ᱷ' => Ok(OlChiki::LetterOh),
            'ᱸ' => Ok(OlChiki::MuTtuddag),
            'ᱹ' => Ok(OlChiki::GaahlaaTtuddaag),
            'ᱺ' => Ok(OlChiki::MuDashGaahlaaTtuddaag),
            'ᱻ' => Ok(OlChiki::Relaa),
            'ᱼ' => Ok(OlChiki::Phaarkaa),
            'ᱽ' => Ok(OlChiki::Ahad),
            '᱾' => Ok(OlChiki::PunctuationMucaad),
            _ => Err(()),
        }
    }
}

impl Into<u32> for OlChiki {
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

impl std::convert::TryFrom<u32> for OlChiki {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for OlChiki {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl OlChiki {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        OlChiki::DigitZero
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("OlChiki{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
