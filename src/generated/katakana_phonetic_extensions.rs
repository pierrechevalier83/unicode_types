
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
        match self {
            KatakanaPhoneticExtensions::KatakanaLetterSmallKu => 'ㇰ',
            KatakanaPhoneticExtensions::KatakanaLetterSmallSi => 'ㇱ',
            KatakanaPhoneticExtensions::KatakanaLetterSmallSu => 'ㇲ',
            KatakanaPhoneticExtensions::KatakanaLetterSmallTo => 'ㇳ',
            KatakanaPhoneticExtensions::KatakanaLetterSmallNu => 'ㇴ',
            KatakanaPhoneticExtensions::KatakanaLetterSmallHa => 'ㇵ',
            KatakanaPhoneticExtensions::KatakanaLetterSmallHi => 'ㇶ',
            KatakanaPhoneticExtensions::KatakanaLetterSmallHu => 'ㇷ',
            KatakanaPhoneticExtensions::KatakanaLetterSmallHe => 'ㇸ',
            KatakanaPhoneticExtensions::KatakanaLetterSmallHo => 'ㇹ',
            KatakanaPhoneticExtensions::KatakanaLetterSmallMu => 'ㇺ',
            KatakanaPhoneticExtensions::KatakanaLetterSmallRa => 'ㇻ',
            KatakanaPhoneticExtensions::KatakanaLetterSmallRi => 'ㇼ',
            KatakanaPhoneticExtensions::KatakanaLetterSmallRu => 'ㇽ',
            KatakanaPhoneticExtensions::KatakanaLetterSmallRe => 'ㇾ',
        }
    }
}

impl std::convert::TryFrom<char> for KatakanaPhoneticExtensions {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ㇰ' => Ok(KatakanaPhoneticExtensions::KatakanaLetterSmallKu),
            'ㇱ' => Ok(KatakanaPhoneticExtensions::KatakanaLetterSmallSi),
            'ㇲ' => Ok(KatakanaPhoneticExtensions::KatakanaLetterSmallSu),
            'ㇳ' => Ok(KatakanaPhoneticExtensions::KatakanaLetterSmallTo),
            'ㇴ' => Ok(KatakanaPhoneticExtensions::KatakanaLetterSmallNu),
            'ㇵ' => Ok(KatakanaPhoneticExtensions::KatakanaLetterSmallHa),
            'ㇶ' => Ok(KatakanaPhoneticExtensions::KatakanaLetterSmallHi),
            'ㇷ' => Ok(KatakanaPhoneticExtensions::KatakanaLetterSmallHu),
            'ㇸ' => Ok(KatakanaPhoneticExtensions::KatakanaLetterSmallHe),
            'ㇹ' => Ok(KatakanaPhoneticExtensions::KatakanaLetterSmallHo),
            'ㇺ' => Ok(KatakanaPhoneticExtensions::KatakanaLetterSmallMu),
            'ㇻ' => Ok(KatakanaPhoneticExtensions::KatakanaLetterSmallRa),
            'ㇼ' => Ok(KatakanaPhoneticExtensions::KatakanaLetterSmallRi),
            'ㇽ' => Ok(KatakanaPhoneticExtensions::KatakanaLetterSmallRu),
            'ㇾ' => Ok(KatakanaPhoneticExtensions::KatakanaLetterSmallRe),
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

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("KatakanaPhoneticExtensions{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
