/// \u{31f0} → \u{31ff}\
///\
/// ㇰ ㇱ ㇲ ㇳ ㇴ ㇵ ㇶ ㇷ ㇸ ㇹ ㇺ ㇻ ㇼ ㇽ ㇾ\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{31f0}: 'ㇰ'
    pub const KATAKANA_LETTER_SMALL_KU: char = 'ㇰ';
    /// \u{31f1}: 'ㇱ'
    pub const KATAKANA_LETTER_SMALL_SI: char = 'ㇱ';
    /// \u{31f2}: 'ㇲ'
    pub const KATAKANA_LETTER_SMALL_SU: char = 'ㇲ';
    /// \u{31f3}: 'ㇳ'
    pub const KATAKANA_LETTER_SMALL_TO: char = 'ㇳ';
    /// \u{31f4}: 'ㇴ'
    pub const KATAKANA_LETTER_SMALL_NU: char = 'ㇴ';
    /// \u{31f5}: 'ㇵ'
    pub const KATAKANA_LETTER_SMALL_HA: char = 'ㇵ';
    /// \u{31f6}: 'ㇶ'
    pub const KATAKANA_LETTER_SMALL_HI: char = 'ㇶ';
    /// \u{31f7}: 'ㇷ'
    pub const KATAKANA_LETTER_SMALL_HU: char = 'ㇷ';
    /// \u{31f8}: 'ㇸ'
    pub const KATAKANA_LETTER_SMALL_HE: char = 'ㇸ';
    /// \u{31f9}: 'ㇹ'
    pub const KATAKANA_LETTER_SMALL_HO: char = 'ㇹ';
    /// \u{31fa}: 'ㇺ'
    pub const KATAKANA_LETTER_SMALL_MU: char = 'ㇺ';
    /// \u{31fb}: 'ㇻ'
    pub const KATAKANA_LETTER_SMALL_RA: char = 'ㇻ';
    /// \u{31fc}: 'ㇼ'
    pub const KATAKANA_LETTER_SMALL_RI: char = 'ㇼ';
    /// \u{31fd}: 'ㇽ'
    pub const KATAKANA_LETTER_SMALL_RU: char = 'ㇽ';
    /// \u{31fe}: 'ㇾ'
    pub const KATAKANA_LETTER_SMALL_RE: char = 'ㇾ';
}

/// An enum to represent all characters in the KatakanaPhoneticExtensions block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum KatakanaPhoneticExtensions {
    /// \u{31f0}: 'ㇰ'
    KatakanaLetterSmallKu,
    /// \u{31f1}: 'ㇱ'
    KatakanaLetterSmallSi,
    /// \u{31f2}: 'ㇲ'
    KatakanaLetterSmallSu,
    /// \u{31f3}: 'ㇳ'
    KatakanaLetterSmallTo,
    /// \u{31f4}: 'ㇴ'
    KatakanaLetterSmallNu,
    /// \u{31f5}: 'ㇵ'
    KatakanaLetterSmallHa,
    /// \u{31f6}: 'ㇶ'
    KatakanaLetterSmallHi,
    /// \u{31f7}: 'ㇷ'
    KatakanaLetterSmallHu,
    /// \u{31f8}: 'ㇸ'
    KatakanaLetterSmallHe,
    /// \u{31f9}: 'ㇹ'
    KatakanaLetterSmallHo,
    /// \u{31fa}: 'ㇺ'
    KatakanaLetterSmallMu,
    /// \u{31fb}: 'ㇻ'
    KatakanaLetterSmallRa,
    /// \u{31fc}: 'ㇼ'
    KatakanaLetterSmallRi,
    /// \u{31fd}: 'ㇽ'
    KatakanaLetterSmallRu,
    /// \u{31fe}: 'ㇾ'
    KatakanaLetterSmallRe,
}

impl Into<char> for KatakanaPhoneticExtensions {
    fn into(self) -> char {
        use constants::*;
        match self {
            KatakanaPhoneticExtensions::KatakanaLetterSmallKu => KATAKANA_LETTER_SMALL_KU,
            KatakanaPhoneticExtensions::KatakanaLetterSmallSi => KATAKANA_LETTER_SMALL_SI,
            KatakanaPhoneticExtensions::KatakanaLetterSmallSu => KATAKANA_LETTER_SMALL_SU,
            KatakanaPhoneticExtensions::KatakanaLetterSmallTo => KATAKANA_LETTER_SMALL_TO,
            KatakanaPhoneticExtensions::KatakanaLetterSmallNu => KATAKANA_LETTER_SMALL_NU,
            KatakanaPhoneticExtensions::KatakanaLetterSmallHa => KATAKANA_LETTER_SMALL_HA,
            KatakanaPhoneticExtensions::KatakanaLetterSmallHi => KATAKANA_LETTER_SMALL_HI,
            KatakanaPhoneticExtensions::KatakanaLetterSmallHu => KATAKANA_LETTER_SMALL_HU,
            KatakanaPhoneticExtensions::KatakanaLetterSmallHe => KATAKANA_LETTER_SMALL_HE,
            KatakanaPhoneticExtensions::KatakanaLetterSmallHo => KATAKANA_LETTER_SMALL_HO,
            KatakanaPhoneticExtensions::KatakanaLetterSmallMu => KATAKANA_LETTER_SMALL_MU,
            KatakanaPhoneticExtensions::KatakanaLetterSmallRa => KATAKANA_LETTER_SMALL_RA,
            KatakanaPhoneticExtensions::KatakanaLetterSmallRi => KATAKANA_LETTER_SMALL_RI,
            KatakanaPhoneticExtensions::KatakanaLetterSmallRu => KATAKANA_LETTER_SMALL_RU,
            KatakanaPhoneticExtensions::KatakanaLetterSmallRe => KATAKANA_LETTER_SMALL_RE,
        }
    }
}

impl std::convert::TryFrom<char> for KatakanaPhoneticExtensions {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            KATAKANA_LETTER_SMALL_KU => Ok(KatakanaPhoneticExtensions::KatakanaLetterSmallKu),
            KATAKANA_LETTER_SMALL_SI => Ok(KatakanaPhoneticExtensions::KatakanaLetterSmallSi),
            KATAKANA_LETTER_SMALL_SU => Ok(KatakanaPhoneticExtensions::KatakanaLetterSmallSu),
            KATAKANA_LETTER_SMALL_TO => Ok(KatakanaPhoneticExtensions::KatakanaLetterSmallTo),
            KATAKANA_LETTER_SMALL_NU => Ok(KatakanaPhoneticExtensions::KatakanaLetterSmallNu),
            KATAKANA_LETTER_SMALL_HA => Ok(KatakanaPhoneticExtensions::KatakanaLetterSmallHa),
            KATAKANA_LETTER_SMALL_HI => Ok(KatakanaPhoneticExtensions::KatakanaLetterSmallHi),
            KATAKANA_LETTER_SMALL_HU => Ok(KatakanaPhoneticExtensions::KatakanaLetterSmallHu),
            KATAKANA_LETTER_SMALL_HE => Ok(KatakanaPhoneticExtensions::KatakanaLetterSmallHe),
            KATAKANA_LETTER_SMALL_HO => Ok(KatakanaPhoneticExtensions::KatakanaLetterSmallHo),
            KATAKANA_LETTER_SMALL_MU => Ok(KatakanaPhoneticExtensions::KatakanaLetterSmallMu),
            KATAKANA_LETTER_SMALL_RA => Ok(KatakanaPhoneticExtensions::KatakanaLetterSmallRa),
            KATAKANA_LETTER_SMALL_RI => Ok(KatakanaPhoneticExtensions::KatakanaLetterSmallRi),
            KATAKANA_LETTER_SMALL_RU => Ok(KatakanaPhoneticExtensions::KatakanaLetterSmallRu),
            KATAKANA_LETTER_SMALL_RE => Ok(KatakanaPhoneticExtensions::KatakanaLetterSmallRe),
            _ => Err(()),
        }
    }
}

impl Into<u32> for KatakanaPhoneticExtensions {
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

impl std::convert::TryFrom<u32> for KatakanaPhoneticExtensions {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for KatakanaPhoneticExtensions {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl KatakanaPhoneticExtensions {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        KatakanaPhoneticExtensions::KatakanaLetterSmallKu
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            KatakanaPhoneticExtensions::KatakanaLetterSmallKu => "katakana letter small ku",
            KatakanaPhoneticExtensions::KatakanaLetterSmallSi => "katakana letter small si",
            KatakanaPhoneticExtensions::KatakanaLetterSmallSu => "katakana letter small su",
            KatakanaPhoneticExtensions::KatakanaLetterSmallTo => "katakana letter small to",
            KatakanaPhoneticExtensions::KatakanaLetterSmallNu => "katakana letter small nu",
            KatakanaPhoneticExtensions::KatakanaLetterSmallHa => "katakana letter small ha",
            KatakanaPhoneticExtensions::KatakanaLetterSmallHi => "katakana letter small hi",
            KatakanaPhoneticExtensions::KatakanaLetterSmallHu => "katakana letter small hu",
            KatakanaPhoneticExtensions::KatakanaLetterSmallHe => "katakana letter small he",
            KatakanaPhoneticExtensions::KatakanaLetterSmallHo => "katakana letter small ho",
            KatakanaPhoneticExtensions::KatakanaLetterSmallMu => "katakana letter small mu",
            KatakanaPhoneticExtensions::KatakanaLetterSmallRa => "katakana letter small ra",
            KatakanaPhoneticExtensions::KatakanaLetterSmallRi => "katakana letter small ri",
            KatakanaPhoneticExtensions::KatakanaLetterSmallRu => "katakana letter small ru",
            KatakanaPhoneticExtensions::KatakanaLetterSmallRe => "katakana letter small re",
        }
    }
}
