/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{1c50}: '᱐'
    pub const DIGIT_ZERO: char = '᱐';
    /// \u{1c51}: '᱑'
    pub const DIGIT_ONE: char = '᱑';
    /// \u{1c52}: '᱒'
    pub const DIGIT_TWO: char = '᱒';
    /// \u{1c53}: '᱓'
    pub const DIGIT_THREE: char = '᱓';
    /// \u{1c54}: '᱔'
    pub const DIGIT_FOUR: char = '᱔';
    /// \u{1c55}: '᱕'
    pub const DIGIT_FIVE: char = '᱕';
    /// \u{1c56}: '᱖'
    pub const DIGIT_SIX: char = '᱖';
    /// \u{1c57}: '᱗'
    pub const DIGIT_SEVEN: char = '᱗';
    /// \u{1c58}: '᱘'
    pub const DIGIT_EIGHT: char = '᱘';
    /// \u{1c59}: '᱙'
    pub const DIGIT_NINE: char = '᱙';
    /// \u{1c5a}: 'ᱚ'
    pub const LETTER_LA: char = 'ᱚ';
    /// \u{1c5b}: 'ᱛ'
    pub const LETTER_AT: char = 'ᱛ';
    /// \u{1c5c}: 'ᱜ'
    pub const LETTER_AG: char = 'ᱜ';
    /// \u{1c5d}: 'ᱝ'
    pub const LETTER_ANG: char = 'ᱝ';
    /// \u{1c5e}: 'ᱞ'
    pub const LETTER_AL: char = 'ᱞ';
    /// \u{1c5f}: 'ᱟ'
    pub const LETTER_LAA: char = 'ᱟ';
    /// \u{1c60}: 'ᱠ'
    pub const LETTER_AAK: char = 'ᱠ';
    /// \u{1c61}: 'ᱡ'
    pub const LETTER_AAJ: char = 'ᱡ';
    /// \u{1c62}: 'ᱢ'
    pub const LETTER_AAM: char = 'ᱢ';
    /// \u{1c63}: 'ᱣ'
    pub const LETTER_AAW: char = 'ᱣ';
    /// \u{1c64}: 'ᱤ'
    pub const LETTER_LI: char = 'ᱤ';
    /// \u{1c65}: 'ᱥ'
    pub const LETTER_IS: char = 'ᱥ';
    /// \u{1c66}: 'ᱦ'
    pub const LETTER_IH: char = 'ᱦ';
    /// \u{1c67}: 'ᱧ'
    pub const LETTER_INY: char = 'ᱧ';
    /// \u{1c68}: 'ᱨ'
    pub const LETTER_IR: char = 'ᱨ';
    /// \u{1c69}: 'ᱩ'
    pub const LETTER_LU: char = 'ᱩ';
    /// \u{1c6a}: 'ᱪ'
    pub const LETTER_UC: char = 'ᱪ';
    /// \u{1c6b}: 'ᱫ'
    pub const LETTER_UD: char = 'ᱫ';
    /// \u{1c6c}: 'ᱬ'
    pub const LETTER_UNN: char = 'ᱬ';
    /// \u{1c6d}: 'ᱭ'
    pub const LETTER_UY: char = 'ᱭ';
    /// \u{1c6e}: 'ᱮ'
    pub const LETTER_LE: char = 'ᱮ';
    /// \u{1c6f}: 'ᱯ'
    pub const LETTER_EP: char = 'ᱯ';
    /// \u{1c70}: 'ᱰ'
    pub const LETTER_EDD: char = 'ᱰ';
    /// \u{1c71}: 'ᱱ'
    pub const LETTER_EN: char = 'ᱱ';
    /// \u{1c72}: 'ᱲ'
    pub const LETTER_ERR: char = 'ᱲ';
    /// \u{1c73}: 'ᱳ'
    pub const LETTER_LO: char = 'ᱳ';
    /// \u{1c74}: 'ᱴ'
    pub const LETTER_OTT: char = 'ᱴ';
    /// \u{1c75}: 'ᱵ'
    pub const LETTER_OB: char = 'ᱵ';
    /// \u{1c76}: 'ᱶ'
    pub const LETTER_OV: char = 'ᱶ';
    /// \u{1c77}: 'ᱷ'
    pub const LETTER_OH: char = 'ᱷ';
    /// \u{1c78}: 'ᱸ'
    pub const MU_TTUDDAG: char = 'ᱸ';
    /// \u{1c79}: 'ᱹ'
    pub const GAAHLAA_TTUDDAAG: char = 'ᱹ';
    /// \u{1c7a}: 'ᱺ'
    pub const MU_DASH_GAAHLAA_TTUDDAAG: char = 'ᱺ';
    /// \u{1c7b}: 'ᱻ'
    pub const RELAA: char = 'ᱻ';
    /// \u{1c7c}: 'ᱼ'
    pub const PHAARKAA: char = 'ᱼ';
    /// \u{1c7d}: 'ᱽ'
    pub const AHAD: char = 'ᱽ';
    /// \u{1c7e}: '᱾'
    pub const PUNCTUATION_MUCAAD: char = '᱾';
}

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
        use constants::*;
        match self {
            OlChiki::DigitZero => DIGIT_ZERO,
            OlChiki::DigitOne => DIGIT_ONE,
            OlChiki::DigitTwo => DIGIT_TWO,
            OlChiki::DigitThree => DIGIT_THREE,
            OlChiki::DigitFour => DIGIT_FOUR,
            OlChiki::DigitFive => DIGIT_FIVE,
            OlChiki::DigitSix => DIGIT_SIX,
            OlChiki::DigitSeven => DIGIT_SEVEN,
            OlChiki::DigitEight => DIGIT_EIGHT,
            OlChiki::DigitNine => DIGIT_NINE,
            OlChiki::LetterLa => LETTER_LA,
            OlChiki::LetterAt => LETTER_AT,
            OlChiki::LetterAg => LETTER_AG,
            OlChiki::LetterAng => LETTER_ANG,
            OlChiki::LetterAl => LETTER_AL,
            OlChiki::LetterLaa => LETTER_LAA,
            OlChiki::LetterAak => LETTER_AAK,
            OlChiki::LetterAaj => LETTER_AAJ,
            OlChiki::LetterAam => LETTER_AAM,
            OlChiki::LetterAaw => LETTER_AAW,
            OlChiki::LetterLi => LETTER_LI,
            OlChiki::LetterIs => LETTER_IS,
            OlChiki::LetterIh => LETTER_IH,
            OlChiki::LetterIny => LETTER_INY,
            OlChiki::LetterIr => LETTER_IR,
            OlChiki::LetterLu => LETTER_LU,
            OlChiki::LetterUc => LETTER_UC,
            OlChiki::LetterUd => LETTER_UD,
            OlChiki::LetterUnn => LETTER_UNN,
            OlChiki::LetterUy => LETTER_UY,
            OlChiki::LetterLe => LETTER_LE,
            OlChiki::LetterEp => LETTER_EP,
            OlChiki::LetterEdd => LETTER_EDD,
            OlChiki::LetterEn => LETTER_EN,
            OlChiki::LetterErr => LETTER_ERR,
            OlChiki::LetterLo => LETTER_LO,
            OlChiki::LetterOtt => LETTER_OTT,
            OlChiki::LetterOb => LETTER_OB,
            OlChiki::LetterOv => LETTER_OV,
            OlChiki::LetterOh => LETTER_OH,
            OlChiki::MuTtuddag => MU_TTUDDAG,
            OlChiki::GaahlaaTtuddaag => GAAHLAA_TTUDDAAG,
            OlChiki::MuDashGaahlaaTtuddaag => MU_DASH_GAAHLAA_TTUDDAAG,
            OlChiki::Relaa => RELAA,
            OlChiki::Phaarkaa => PHAARKAA,
            OlChiki::Ahad => AHAD,
            OlChiki::PunctuationMucaad => PUNCTUATION_MUCAAD,
        }
    }
}

impl std::convert::TryFrom<char> for OlChiki {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            DIGIT_ZERO => Ok(OlChiki::DigitZero),
            DIGIT_ONE => Ok(OlChiki::DigitOne),
            DIGIT_TWO => Ok(OlChiki::DigitTwo),
            DIGIT_THREE => Ok(OlChiki::DigitThree),
            DIGIT_FOUR => Ok(OlChiki::DigitFour),
            DIGIT_FIVE => Ok(OlChiki::DigitFive),
            DIGIT_SIX => Ok(OlChiki::DigitSix),
            DIGIT_SEVEN => Ok(OlChiki::DigitSeven),
            DIGIT_EIGHT => Ok(OlChiki::DigitEight),
            DIGIT_NINE => Ok(OlChiki::DigitNine),
            LETTER_LA => Ok(OlChiki::LetterLa),
            LETTER_AT => Ok(OlChiki::LetterAt),
            LETTER_AG => Ok(OlChiki::LetterAg),
            LETTER_ANG => Ok(OlChiki::LetterAng),
            LETTER_AL => Ok(OlChiki::LetterAl),
            LETTER_LAA => Ok(OlChiki::LetterLaa),
            LETTER_AAK => Ok(OlChiki::LetterAak),
            LETTER_AAJ => Ok(OlChiki::LetterAaj),
            LETTER_AAM => Ok(OlChiki::LetterAam),
            LETTER_AAW => Ok(OlChiki::LetterAaw),
            LETTER_LI => Ok(OlChiki::LetterLi),
            LETTER_IS => Ok(OlChiki::LetterIs),
            LETTER_IH => Ok(OlChiki::LetterIh),
            LETTER_INY => Ok(OlChiki::LetterIny),
            LETTER_IR => Ok(OlChiki::LetterIr),
            LETTER_LU => Ok(OlChiki::LetterLu),
            LETTER_UC => Ok(OlChiki::LetterUc),
            LETTER_UD => Ok(OlChiki::LetterUd),
            LETTER_UNN => Ok(OlChiki::LetterUnn),
            LETTER_UY => Ok(OlChiki::LetterUy),
            LETTER_LE => Ok(OlChiki::LetterLe),
            LETTER_EP => Ok(OlChiki::LetterEp),
            LETTER_EDD => Ok(OlChiki::LetterEdd),
            LETTER_EN => Ok(OlChiki::LetterEn),
            LETTER_ERR => Ok(OlChiki::LetterErr),
            LETTER_LO => Ok(OlChiki::LetterLo),
            LETTER_OTT => Ok(OlChiki::LetterOtt),
            LETTER_OB => Ok(OlChiki::LetterOb),
            LETTER_OV => Ok(OlChiki::LetterOv),
            LETTER_OH => Ok(OlChiki::LetterOh),
            MU_TTUDDAG => Ok(OlChiki::MuTtuddag),
            GAAHLAA_TTUDDAAG => Ok(OlChiki::GaahlaaTtuddaag),
            MU_DASH_GAAHLAA_TTUDDAAG => Ok(OlChiki::MuDashGaahlaaTtuddaag),
            RELAA => Ok(OlChiki::Relaa),
            PHAARKAA => Ok(OlChiki::Phaarkaa),
            AHAD => Ok(OlChiki::Ahad),
            PUNCTUATION_MUCAAD => Ok(OlChiki::PunctuationMucaad),
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

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            OlChiki::DigitZero => "ol chiki digit zero",
            OlChiki::DigitOne => "ol chiki digit one",
            OlChiki::DigitTwo => "ol chiki digit two",
            OlChiki::DigitThree => "ol chiki digit three",
            OlChiki::DigitFour => "ol chiki digit four",
            OlChiki::DigitFive => "ol chiki digit five",
            OlChiki::DigitSix => "ol chiki digit six",
            OlChiki::DigitSeven => "ol chiki digit seven",
            OlChiki::DigitEight => "ol chiki digit eight",
            OlChiki::DigitNine => "ol chiki digit nine",
            OlChiki::LetterLa => "ol chiki letter la",
            OlChiki::LetterAt => "ol chiki letter at",
            OlChiki::LetterAg => "ol chiki letter ag",
            OlChiki::LetterAng => "ol chiki letter ang",
            OlChiki::LetterAl => "ol chiki letter al",
            OlChiki::LetterLaa => "ol chiki letter laa",
            OlChiki::LetterAak => "ol chiki letter aak",
            OlChiki::LetterAaj => "ol chiki letter aaj",
            OlChiki::LetterAam => "ol chiki letter aam",
            OlChiki::LetterAaw => "ol chiki letter aaw",
            OlChiki::LetterLi => "ol chiki letter li",
            OlChiki::LetterIs => "ol chiki letter is",
            OlChiki::LetterIh => "ol chiki letter ih",
            OlChiki::LetterIny => "ol chiki letter iny",
            OlChiki::LetterIr => "ol chiki letter ir",
            OlChiki::LetterLu => "ol chiki letter lu",
            OlChiki::LetterUc => "ol chiki letter uc",
            OlChiki::LetterUd => "ol chiki letter ud",
            OlChiki::LetterUnn => "ol chiki letter unn",
            OlChiki::LetterUy => "ol chiki letter uy",
            OlChiki::LetterLe => "ol chiki letter le",
            OlChiki::LetterEp => "ol chiki letter ep",
            OlChiki::LetterEdd => "ol chiki letter edd",
            OlChiki::LetterEn => "ol chiki letter en",
            OlChiki::LetterErr => "ol chiki letter err",
            OlChiki::LetterLo => "ol chiki letter lo",
            OlChiki::LetterOtt => "ol chiki letter ott",
            OlChiki::LetterOb => "ol chiki letter ob",
            OlChiki::LetterOv => "ol chiki letter ov",
            OlChiki::LetterOh => "ol chiki letter oh",
            OlChiki::MuTtuddag => "ol chiki mu ttuddag",
            OlChiki::GaahlaaTtuddaag => "ol chiki gaahlaa ttuddaag",
            OlChiki::MuDashGaahlaaTtuddaag => "ol chiki mu-gaahlaa ttuddaag",
            OlChiki::Relaa => "ol chiki relaa",
            OlChiki::Phaarkaa => "ol chiki phaarkaa",
            OlChiki::Ahad => "ol chiki ahad",
            OlChiki::PunctuationMucaad => "ol chiki punctuation mucaad",
        }
    }
}
