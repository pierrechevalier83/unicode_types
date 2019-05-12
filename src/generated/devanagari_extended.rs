/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{a8e0}: '꣠'
    pub const COMBINING_DEVANAGARI_DIGIT_ZERO: char = '꣠';
    /// \u{a8e1}: '꣡'
    pub const COMBINING_DEVANAGARI_DIGIT_ONE: char = '꣡';
    /// \u{a8e2}: '꣢'
    pub const COMBINING_DEVANAGARI_DIGIT_TWO: char = '꣢';
    /// \u{a8e3}: '꣣'
    pub const COMBINING_DEVANAGARI_DIGIT_THREE: char = '꣣';
    /// \u{a8e4}: '꣤'
    pub const COMBINING_DEVANAGARI_DIGIT_FOUR: char = '꣤';
    /// \u{a8e5}: '꣥'
    pub const COMBINING_DEVANAGARI_DIGIT_FIVE: char = '꣥';
    /// \u{a8e6}: '꣦'
    pub const COMBINING_DEVANAGARI_DIGIT_SIX: char = '꣦';
    /// \u{a8e7}: '꣧'
    pub const COMBINING_DEVANAGARI_DIGIT_SEVEN: char = '꣧';
    /// \u{a8e8}: '꣨'
    pub const COMBINING_DEVANAGARI_DIGIT_EIGHT: char = '꣨';
    /// \u{a8e9}: '꣩'
    pub const COMBINING_DEVANAGARI_DIGIT_NINE: char = '꣩';
    /// \u{a8ea}: '꣪'
    pub const COMBINING_DEVANAGARI_LETTER_A: char = '꣪';
    /// \u{a8eb}: '꣫'
    pub const COMBINING_DEVANAGARI_LETTER_U: char = '꣫';
    /// \u{a8ec}: '꣬'
    pub const COMBINING_DEVANAGARI_LETTER_KA: char = '꣬';
    /// \u{a8ed}: '꣭'
    pub const COMBINING_DEVANAGARI_LETTER_NA: char = '꣭';
    /// \u{a8ee}: '꣮'
    pub const COMBINING_DEVANAGARI_LETTER_PA: char = '꣮';
    /// \u{a8ef}: '꣯'
    pub const COMBINING_DEVANAGARI_LETTER_RA: char = '꣯';
    /// \u{a8f0}: '꣰'
    pub const COMBINING_DEVANAGARI_LETTER_VI: char = '꣰';
    /// \u{a8f1}: '꣱'
    pub const COMBINING_DEVANAGARI_SIGN_AVAGRAHA: char = '꣱';
    /// \u{a8f2}: 'ꣲ'
    pub const DEVANAGARI_SIGN_SPACING_CANDRABINDU: char = 'ꣲ';
    /// \u{a8f3}: 'ꣳ'
    pub const DEVANAGARI_SIGN_CANDRABINDU_VIRAMA: char = 'ꣳ';
    /// \u{a8f4}: 'ꣴ'
    pub const DEVANAGARI_SIGN_DOUBLE_CANDRABINDU_VIRAMA: char = 'ꣴ';
    /// \u{a8f5}: 'ꣵ'
    pub const DEVANAGARI_SIGN_CANDRABINDU_TWO: char = 'ꣵ';
    /// \u{a8f6}: 'ꣶ'
    pub const DEVANAGARI_SIGN_CANDRABINDU_THREE: char = 'ꣶ';
    /// \u{a8f7}: 'ꣷ'
    pub const DEVANAGARI_SIGN_CANDRABINDU_AVAGRAHA: char = 'ꣷ';
    /// \u{a8f8}: '꣸'
    pub const DEVANAGARI_SIGN_PUSHPIKA: char = '꣸';
    /// \u{a8f9}: '꣹'
    pub const DEVANAGARI_GAP_FILLER: char = '꣹';
    /// \u{a8fa}: '꣺'
    pub const DEVANAGARI_CARET: char = '꣺';
    /// \u{a8fb}: 'ꣻ'
    pub const DEVANAGARI_HEADSTROKE: char = 'ꣻ';
    /// \u{a8fc}: '꣼'
    pub const DEVANAGARI_SIGN_SIDDHAM: char = '꣼';
    /// \u{a8fd}: 'ꣽ'
    pub const DEVANAGARI_JAIN_OM: char = 'ꣽ';
    /// \u{a8fe}: 'ꣾ'
    pub const DEVANAGARI_LETTER_AY: char = 'ꣾ';
}

/// An enum to represent all characters in the DevanagariExtended block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum DevanagariExtended {
    /// \u{a8e0}: '꣠'
    CombiningDevanagariDigitZero,
    /// \u{a8e1}: '꣡'
    CombiningDevanagariDigitOne,
    /// \u{a8e2}: '꣢'
    CombiningDevanagariDigitTwo,
    /// \u{a8e3}: '꣣'
    CombiningDevanagariDigitThree,
    /// \u{a8e4}: '꣤'
    CombiningDevanagariDigitFour,
    /// \u{a8e5}: '꣥'
    CombiningDevanagariDigitFive,
    /// \u{a8e6}: '꣦'
    CombiningDevanagariDigitSix,
    /// \u{a8e7}: '꣧'
    CombiningDevanagariDigitSeven,
    /// \u{a8e8}: '꣨'
    CombiningDevanagariDigitEight,
    /// \u{a8e9}: '꣩'
    CombiningDevanagariDigitNine,
    /// \u{a8ea}: '꣪'
    CombiningDevanagariLetterA,
    /// \u{a8eb}: '꣫'
    CombiningDevanagariLetterU,
    /// \u{a8ec}: '꣬'
    CombiningDevanagariLetterKa,
    /// \u{a8ed}: '꣭'
    CombiningDevanagariLetterNa,
    /// \u{a8ee}: '꣮'
    CombiningDevanagariLetterPa,
    /// \u{a8ef}: '꣯'
    CombiningDevanagariLetterRa,
    /// \u{a8f0}: '꣰'
    CombiningDevanagariLetterVi,
    /// \u{a8f1}: '꣱'
    CombiningDevanagariSignAvagraha,
    /// \u{a8f2}: 'ꣲ'
    DevanagariSignSpacingCandrabindu,
    /// \u{a8f3}: 'ꣳ'
    DevanagariSignCandrabinduVirama,
    /// \u{a8f4}: 'ꣴ'
    DevanagariSignDoubleCandrabinduVirama,
    /// \u{a8f5}: 'ꣵ'
    DevanagariSignCandrabinduTwo,
    /// \u{a8f6}: 'ꣶ'
    DevanagariSignCandrabinduThree,
    /// \u{a8f7}: 'ꣷ'
    DevanagariSignCandrabinduAvagraha,
    /// \u{a8f8}: '꣸'
    DevanagariSignPushpika,
    /// \u{a8f9}: '꣹'
    DevanagariGapFiller,
    /// \u{a8fa}: '꣺'
    DevanagariCaret,
    /// \u{a8fb}: 'ꣻ'
    DevanagariHeadstroke,
    /// \u{a8fc}: '꣼'
    DevanagariSignSiddham,
    /// \u{a8fd}: 'ꣽ'
    DevanagariJainOm,
    /// \u{a8fe}: 'ꣾ'
    DevanagariLetterAy,
}

impl Into<char> for DevanagariExtended {
    fn into(self) -> char {
        use constants::*;
        match self {
            DevanagariExtended::CombiningDevanagariDigitZero => COMBINING_DEVANAGARI_DIGIT_ZERO,
            DevanagariExtended::CombiningDevanagariDigitOne => COMBINING_DEVANAGARI_DIGIT_ONE,
            DevanagariExtended::CombiningDevanagariDigitTwo => COMBINING_DEVANAGARI_DIGIT_TWO,
            DevanagariExtended::CombiningDevanagariDigitThree => COMBINING_DEVANAGARI_DIGIT_THREE,
            DevanagariExtended::CombiningDevanagariDigitFour => COMBINING_DEVANAGARI_DIGIT_FOUR,
            DevanagariExtended::CombiningDevanagariDigitFive => COMBINING_DEVANAGARI_DIGIT_FIVE,
            DevanagariExtended::CombiningDevanagariDigitSix => COMBINING_DEVANAGARI_DIGIT_SIX,
            DevanagariExtended::CombiningDevanagariDigitSeven => COMBINING_DEVANAGARI_DIGIT_SEVEN,
            DevanagariExtended::CombiningDevanagariDigitEight => COMBINING_DEVANAGARI_DIGIT_EIGHT,
            DevanagariExtended::CombiningDevanagariDigitNine => COMBINING_DEVANAGARI_DIGIT_NINE,
            DevanagariExtended::CombiningDevanagariLetterA => COMBINING_DEVANAGARI_LETTER_A,
            DevanagariExtended::CombiningDevanagariLetterU => COMBINING_DEVANAGARI_LETTER_U,
            DevanagariExtended::CombiningDevanagariLetterKa => COMBINING_DEVANAGARI_LETTER_KA,
            DevanagariExtended::CombiningDevanagariLetterNa => COMBINING_DEVANAGARI_LETTER_NA,
            DevanagariExtended::CombiningDevanagariLetterPa => COMBINING_DEVANAGARI_LETTER_PA,
            DevanagariExtended::CombiningDevanagariLetterRa => COMBINING_DEVANAGARI_LETTER_RA,
            DevanagariExtended::CombiningDevanagariLetterVi => COMBINING_DEVANAGARI_LETTER_VI,
            DevanagariExtended::CombiningDevanagariSignAvagraha => COMBINING_DEVANAGARI_SIGN_AVAGRAHA,
            DevanagariExtended::DevanagariSignSpacingCandrabindu => DEVANAGARI_SIGN_SPACING_CANDRABINDU,
            DevanagariExtended::DevanagariSignCandrabinduVirama => DEVANAGARI_SIGN_CANDRABINDU_VIRAMA,
            DevanagariExtended::DevanagariSignDoubleCandrabinduVirama => DEVANAGARI_SIGN_DOUBLE_CANDRABINDU_VIRAMA,
            DevanagariExtended::DevanagariSignCandrabinduTwo => DEVANAGARI_SIGN_CANDRABINDU_TWO,
            DevanagariExtended::DevanagariSignCandrabinduThree => DEVANAGARI_SIGN_CANDRABINDU_THREE,
            DevanagariExtended::DevanagariSignCandrabinduAvagraha => DEVANAGARI_SIGN_CANDRABINDU_AVAGRAHA,
            DevanagariExtended::DevanagariSignPushpika => DEVANAGARI_SIGN_PUSHPIKA,
            DevanagariExtended::DevanagariGapFiller => DEVANAGARI_GAP_FILLER,
            DevanagariExtended::DevanagariCaret => DEVANAGARI_CARET,
            DevanagariExtended::DevanagariHeadstroke => DEVANAGARI_HEADSTROKE,
            DevanagariExtended::DevanagariSignSiddham => DEVANAGARI_SIGN_SIDDHAM,
            DevanagariExtended::DevanagariJainOm => DEVANAGARI_JAIN_OM,
            DevanagariExtended::DevanagariLetterAy => DEVANAGARI_LETTER_AY,
        }
    }
}

impl std::convert::TryFrom<char> for DevanagariExtended {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            COMBINING_DEVANAGARI_DIGIT_ZERO => Ok(DevanagariExtended::CombiningDevanagariDigitZero),
            COMBINING_DEVANAGARI_DIGIT_ONE => Ok(DevanagariExtended::CombiningDevanagariDigitOne),
            COMBINING_DEVANAGARI_DIGIT_TWO => Ok(DevanagariExtended::CombiningDevanagariDigitTwo),
            COMBINING_DEVANAGARI_DIGIT_THREE => Ok(DevanagariExtended::CombiningDevanagariDigitThree),
            COMBINING_DEVANAGARI_DIGIT_FOUR => Ok(DevanagariExtended::CombiningDevanagariDigitFour),
            COMBINING_DEVANAGARI_DIGIT_FIVE => Ok(DevanagariExtended::CombiningDevanagariDigitFive),
            COMBINING_DEVANAGARI_DIGIT_SIX => Ok(DevanagariExtended::CombiningDevanagariDigitSix),
            COMBINING_DEVANAGARI_DIGIT_SEVEN => Ok(DevanagariExtended::CombiningDevanagariDigitSeven),
            COMBINING_DEVANAGARI_DIGIT_EIGHT => Ok(DevanagariExtended::CombiningDevanagariDigitEight),
            COMBINING_DEVANAGARI_DIGIT_NINE => Ok(DevanagariExtended::CombiningDevanagariDigitNine),
            COMBINING_DEVANAGARI_LETTER_A => Ok(DevanagariExtended::CombiningDevanagariLetterA),
            COMBINING_DEVANAGARI_LETTER_U => Ok(DevanagariExtended::CombiningDevanagariLetterU),
            COMBINING_DEVANAGARI_LETTER_KA => Ok(DevanagariExtended::CombiningDevanagariLetterKa),
            COMBINING_DEVANAGARI_LETTER_NA => Ok(DevanagariExtended::CombiningDevanagariLetterNa),
            COMBINING_DEVANAGARI_LETTER_PA => Ok(DevanagariExtended::CombiningDevanagariLetterPa),
            COMBINING_DEVANAGARI_LETTER_RA => Ok(DevanagariExtended::CombiningDevanagariLetterRa),
            COMBINING_DEVANAGARI_LETTER_VI => Ok(DevanagariExtended::CombiningDevanagariLetterVi),
            COMBINING_DEVANAGARI_SIGN_AVAGRAHA => Ok(DevanagariExtended::CombiningDevanagariSignAvagraha),
            DEVANAGARI_SIGN_SPACING_CANDRABINDU => Ok(DevanagariExtended::DevanagariSignSpacingCandrabindu),
            DEVANAGARI_SIGN_CANDRABINDU_VIRAMA => Ok(DevanagariExtended::DevanagariSignCandrabinduVirama),
            DEVANAGARI_SIGN_DOUBLE_CANDRABINDU_VIRAMA => Ok(DevanagariExtended::DevanagariSignDoubleCandrabinduVirama),
            DEVANAGARI_SIGN_CANDRABINDU_TWO => Ok(DevanagariExtended::DevanagariSignCandrabinduTwo),
            DEVANAGARI_SIGN_CANDRABINDU_THREE => Ok(DevanagariExtended::DevanagariSignCandrabinduThree),
            DEVANAGARI_SIGN_CANDRABINDU_AVAGRAHA => Ok(DevanagariExtended::DevanagariSignCandrabinduAvagraha),
            DEVANAGARI_SIGN_PUSHPIKA => Ok(DevanagariExtended::DevanagariSignPushpika),
            DEVANAGARI_GAP_FILLER => Ok(DevanagariExtended::DevanagariGapFiller),
            DEVANAGARI_CARET => Ok(DevanagariExtended::DevanagariCaret),
            DEVANAGARI_HEADSTROKE => Ok(DevanagariExtended::DevanagariHeadstroke),
            DEVANAGARI_SIGN_SIDDHAM => Ok(DevanagariExtended::DevanagariSignSiddham),
            DEVANAGARI_JAIN_OM => Ok(DevanagariExtended::DevanagariJainOm),
            DEVANAGARI_LETTER_AY => Ok(DevanagariExtended::DevanagariLetterAy),
            _ => Err(()),
        }
    }
}

impl Into<u32> for DevanagariExtended {
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

impl std::convert::TryFrom<u32> for DevanagariExtended {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for DevanagariExtended {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl DevanagariExtended {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        DevanagariExtended::CombiningDevanagariDigitZero
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            DevanagariExtended::CombiningDevanagariDigitZero => "combining devanagari digit zero",
            DevanagariExtended::CombiningDevanagariDigitOne => "combining devanagari digit one",
            DevanagariExtended::CombiningDevanagariDigitTwo => "combining devanagari digit two",
            DevanagariExtended::CombiningDevanagariDigitThree => "combining devanagari digit three",
            DevanagariExtended::CombiningDevanagariDigitFour => "combining devanagari digit four",
            DevanagariExtended::CombiningDevanagariDigitFive => "combining devanagari digit five",
            DevanagariExtended::CombiningDevanagariDigitSix => "combining devanagari digit six",
            DevanagariExtended::CombiningDevanagariDigitSeven => "combining devanagari digit seven",
            DevanagariExtended::CombiningDevanagariDigitEight => "combining devanagari digit eight",
            DevanagariExtended::CombiningDevanagariDigitNine => "combining devanagari digit nine",
            DevanagariExtended::CombiningDevanagariLetterA => "combining devanagari letter a",
            DevanagariExtended::CombiningDevanagariLetterU => "combining devanagari letter u",
            DevanagariExtended::CombiningDevanagariLetterKa => "combining devanagari letter ka",
            DevanagariExtended::CombiningDevanagariLetterNa => "combining devanagari letter na",
            DevanagariExtended::CombiningDevanagariLetterPa => "combining devanagari letter pa",
            DevanagariExtended::CombiningDevanagariLetterRa => "combining devanagari letter ra",
            DevanagariExtended::CombiningDevanagariLetterVi => "combining devanagari letter vi",
            DevanagariExtended::CombiningDevanagariSignAvagraha => "combining devanagari sign avagraha",
            DevanagariExtended::DevanagariSignSpacingCandrabindu => "devanagari sign spacing candrabindu",
            DevanagariExtended::DevanagariSignCandrabinduVirama => "devanagari sign candrabindu virama",
            DevanagariExtended::DevanagariSignDoubleCandrabinduVirama => "devanagari sign double candrabindu virama",
            DevanagariExtended::DevanagariSignCandrabinduTwo => "devanagari sign candrabindu two",
            DevanagariExtended::DevanagariSignCandrabinduThree => "devanagari sign candrabindu three",
            DevanagariExtended::DevanagariSignCandrabinduAvagraha => "devanagari sign candrabindu avagraha",
            DevanagariExtended::DevanagariSignPushpika => "devanagari sign pushpika",
            DevanagariExtended::DevanagariGapFiller => "devanagari gap filler",
            DevanagariExtended::DevanagariCaret => "devanagari caret",
            DevanagariExtended::DevanagariHeadstroke => "devanagari headstroke",
            DevanagariExtended::DevanagariSignSiddham => "devanagari sign siddham",
            DevanagariExtended::DevanagariJainOm => "devanagari jain om",
            DevanagariExtended::DevanagariLetterAy => "devanagari letter ay",
        }
    }
}
