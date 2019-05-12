
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
        match self {
            DevanagariExtended::CombiningDevanagariDigitZero => '꣠',
            DevanagariExtended::CombiningDevanagariDigitOne => '꣡',
            DevanagariExtended::CombiningDevanagariDigitTwo => '꣢',
            DevanagariExtended::CombiningDevanagariDigitThree => '꣣',
            DevanagariExtended::CombiningDevanagariDigitFour => '꣤',
            DevanagariExtended::CombiningDevanagariDigitFive => '꣥',
            DevanagariExtended::CombiningDevanagariDigitSix => '꣦',
            DevanagariExtended::CombiningDevanagariDigitSeven => '꣧',
            DevanagariExtended::CombiningDevanagariDigitEight => '꣨',
            DevanagariExtended::CombiningDevanagariDigitNine => '꣩',
            DevanagariExtended::CombiningDevanagariLetterA => '꣪',
            DevanagariExtended::CombiningDevanagariLetterU => '꣫',
            DevanagariExtended::CombiningDevanagariLetterKa => '꣬',
            DevanagariExtended::CombiningDevanagariLetterNa => '꣭',
            DevanagariExtended::CombiningDevanagariLetterPa => '꣮',
            DevanagariExtended::CombiningDevanagariLetterRa => '꣯',
            DevanagariExtended::CombiningDevanagariLetterVi => '꣰',
            DevanagariExtended::CombiningDevanagariSignAvagraha => '꣱',
            DevanagariExtended::DevanagariSignSpacingCandrabindu => 'ꣲ',
            DevanagariExtended::DevanagariSignCandrabinduVirama => 'ꣳ',
            DevanagariExtended::DevanagariSignDoubleCandrabinduVirama => 'ꣴ',
            DevanagariExtended::DevanagariSignCandrabinduTwo => 'ꣵ',
            DevanagariExtended::DevanagariSignCandrabinduThree => 'ꣶ',
            DevanagariExtended::DevanagariSignCandrabinduAvagraha => 'ꣷ',
            DevanagariExtended::DevanagariSignPushpika => '꣸',
            DevanagariExtended::DevanagariGapFiller => '꣹',
            DevanagariExtended::DevanagariCaret => '꣺',
            DevanagariExtended::DevanagariHeadstroke => 'ꣻ',
            DevanagariExtended::DevanagariSignSiddham => '꣼',
            DevanagariExtended::DevanagariJainOm => 'ꣽ',
            DevanagariExtended::DevanagariLetterAy => 'ꣾ',
        }
    }
}

impl std::convert::TryFrom<char> for DevanagariExtended {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '꣠' => Ok(DevanagariExtended::CombiningDevanagariDigitZero),
            '꣡' => Ok(DevanagariExtended::CombiningDevanagariDigitOne),
            '꣢' => Ok(DevanagariExtended::CombiningDevanagariDigitTwo),
            '꣣' => Ok(DevanagariExtended::CombiningDevanagariDigitThree),
            '꣤' => Ok(DevanagariExtended::CombiningDevanagariDigitFour),
            '꣥' => Ok(DevanagariExtended::CombiningDevanagariDigitFive),
            '꣦' => Ok(DevanagariExtended::CombiningDevanagariDigitSix),
            '꣧' => Ok(DevanagariExtended::CombiningDevanagariDigitSeven),
            '꣨' => Ok(DevanagariExtended::CombiningDevanagariDigitEight),
            '꣩' => Ok(DevanagariExtended::CombiningDevanagariDigitNine),
            '꣪' => Ok(DevanagariExtended::CombiningDevanagariLetterA),
            '꣫' => Ok(DevanagariExtended::CombiningDevanagariLetterU),
            '꣬' => Ok(DevanagariExtended::CombiningDevanagariLetterKa),
            '꣭' => Ok(DevanagariExtended::CombiningDevanagariLetterNa),
            '꣮' => Ok(DevanagariExtended::CombiningDevanagariLetterPa),
            '꣯' => Ok(DevanagariExtended::CombiningDevanagariLetterRa),
            '꣰' => Ok(DevanagariExtended::CombiningDevanagariLetterVi),
            '꣱' => Ok(DevanagariExtended::CombiningDevanagariSignAvagraha),
            'ꣲ' => Ok(DevanagariExtended::DevanagariSignSpacingCandrabindu),
            'ꣳ' => Ok(DevanagariExtended::DevanagariSignCandrabinduVirama),
            'ꣴ' => Ok(DevanagariExtended::DevanagariSignDoubleCandrabinduVirama),
            'ꣵ' => Ok(DevanagariExtended::DevanagariSignCandrabinduTwo),
            'ꣶ' => Ok(DevanagariExtended::DevanagariSignCandrabinduThree),
            'ꣷ' => Ok(DevanagariExtended::DevanagariSignCandrabinduAvagraha),
            '꣸' => Ok(DevanagariExtended::DevanagariSignPushpika),
            '꣹' => Ok(DevanagariExtended::DevanagariGapFiller),
            '꣺' => Ok(DevanagariExtended::DevanagariCaret),
            'ꣻ' => Ok(DevanagariExtended::DevanagariHeadstroke),
            '꣼' => Ok(DevanagariExtended::DevanagariSignSiddham),
            'ꣽ' => Ok(DevanagariExtended::DevanagariJainOm),
            'ꣾ' => Ok(DevanagariExtended::DevanagariLetterAy),
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

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("DevanagariExtended{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
