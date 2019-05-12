
/// An enum to represent all characters in the MyanmarExtendedB block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum MyanmarExtendedB {
    /// \u{a9e0}: 'ꧠ'
    MyanmarLetterShanGha,
    /// \u{a9e1}: 'ꧡ'
    MyanmarLetterShanCha,
    /// \u{a9e2}: 'ꧢ'
    MyanmarLetterShanJha,
    /// \u{a9e3}: 'ꧣ'
    MyanmarLetterShanNna,
    /// \u{a9e4}: 'ꧤ'
    MyanmarLetterShanBha,
    /// \u{a9e5}: 'ꧥ'
    MyanmarSignShanSaw,
    /// \u{a9e6}: 'ꧦ'
    MyanmarModifierLetterShanReduplication,
    /// \u{a9e7}: 'ꧧ'
    MyanmarLetterTaiLaingNya,
    /// \u{a9e8}: 'ꧨ'
    MyanmarLetterTaiLaingFa,
    /// \u{a9e9}: 'ꧩ'
    MyanmarLetterTaiLaingGa,
    /// \u{a9ea}: 'ꧪ'
    MyanmarLetterTaiLaingGha,
    /// \u{a9eb}: 'ꧫ'
    MyanmarLetterTaiLaingJa,
    /// \u{a9ec}: 'ꧬ'
    MyanmarLetterTaiLaingJha,
    /// \u{a9ed}: 'ꧭ'
    MyanmarLetterTaiLaingDda,
    /// \u{a9ee}: 'ꧮ'
    MyanmarLetterTaiLaingDdha,
    /// \u{a9ef}: 'ꧯ'
    MyanmarLetterTaiLaingNna,
    /// \u{a9f0}: '꧰'
    MyanmarTaiLaingDigitZero,
    /// \u{a9f1}: '꧱'
    MyanmarTaiLaingDigitOne,
    /// \u{a9f2}: '꧲'
    MyanmarTaiLaingDigitTwo,
    /// \u{a9f3}: '꧳'
    MyanmarTaiLaingDigitThree,
    /// \u{a9f4}: '꧴'
    MyanmarTaiLaingDigitFour,
    /// \u{a9f5}: '꧵'
    MyanmarTaiLaingDigitFive,
    /// \u{a9f6}: '꧶'
    MyanmarTaiLaingDigitSix,
    /// \u{a9f7}: '꧷'
    MyanmarTaiLaingDigitSeven,
    /// \u{a9f8}: '꧸'
    MyanmarTaiLaingDigitEight,
    /// \u{a9f9}: '꧹'
    MyanmarTaiLaingDigitNine,
    /// \u{a9fa}: 'ꧺ'
    MyanmarLetterTaiLaingLla,
    /// \u{a9fb}: 'ꧻ'
    MyanmarLetterTaiLaingDa,
    /// \u{a9fc}: 'ꧼ'
    MyanmarLetterTaiLaingDha,
    /// \u{a9fd}: 'ꧽ'
    MyanmarLetterTaiLaingBa,
    /// \u{a9fe}: 'ꧾ'
    MyanmarLetterTaiLaingBha,
}

impl Into<char> for MyanmarExtendedB {
    fn into(self) -> char {
        match self {
            MyanmarExtendedB::MyanmarLetterShanGha => 'ꧠ',
            MyanmarExtendedB::MyanmarLetterShanCha => 'ꧡ',
            MyanmarExtendedB::MyanmarLetterShanJha => 'ꧢ',
            MyanmarExtendedB::MyanmarLetterShanNna => 'ꧣ',
            MyanmarExtendedB::MyanmarLetterShanBha => 'ꧤ',
            MyanmarExtendedB::MyanmarSignShanSaw => 'ꧥ',
            MyanmarExtendedB::MyanmarModifierLetterShanReduplication => 'ꧦ',
            MyanmarExtendedB::MyanmarLetterTaiLaingNya => 'ꧧ',
            MyanmarExtendedB::MyanmarLetterTaiLaingFa => 'ꧨ',
            MyanmarExtendedB::MyanmarLetterTaiLaingGa => 'ꧩ',
            MyanmarExtendedB::MyanmarLetterTaiLaingGha => 'ꧪ',
            MyanmarExtendedB::MyanmarLetterTaiLaingJa => 'ꧫ',
            MyanmarExtendedB::MyanmarLetterTaiLaingJha => 'ꧬ',
            MyanmarExtendedB::MyanmarLetterTaiLaingDda => 'ꧭ',
            MyanmarExtendedB::MyanmarLetterTaiLaingDdha => 'ꧮ',
            MyanmarExtendedB::MyanmarLetterTaiLaingNna => 'ꧯ',
            MyanmarExtendedB::MyanmarTaiLaingDigitZero => '꧰',
            MyanmarExtendedB::MyanmarTaiLaingDigitOne => '꧱',
            MyanmarExtendedB::MyanmarTaiLaingDigitTwo => '꧲',
            MyanmarExtendedB::MyanmarTaiLaingDigitThree => '꧳',
            MyanmarExtendedB::MyanmarTaiLaingDigitFour => '꧴',
            MyanmarExtendedB::MyanmarTaiLaingDigitFive => '꧵',
            MyanmarExtendedB::MyanmarTaiLaingDigitSix => '꧶',
            MyanmarExtendedB::MyanmarTaiLaingDigitSeven => '꧷',
            MyanmarExtendedB::MyanmarTaiLaingDigitEight => '꧸',
            MyanmarExtendedB::MyanmarTaiLaingDigitNine => '꧹',
            MyanmarExtendedB::MyanmarLetterTaiLaingLla => 'ꧺ',
            MyanmarExtendedB::MyanmarLetterTaiLaingDa => 'ꧻ',
            MyanmarExtendedB::MyanmarLetterTaiLaingDha => 'ꧼ',
            MyanmarExtendedB::MyanmarLetterTaiLaingBa => 'ꧽ',
            MyanmarExtendedB::MyanmarLetterTaiLaingBha => 'ꧾ',
        }
    }
}

impl std::convert::TryFrom<char> for MyanmarExtendedB {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ꧠ' => Ok(MyanmarExtendedB::MyanmarLetterShanGha),
            'ꧡ' => Ok(MyanmarExtendedB::MyanmarLetterShanCha),
            'ꧢ' => Ok(MyanmarExtendedB::MyanmarLetterShanJha),
            'ꧣ' => Ok(MyanmarExtendedB::MyanmarLetterShanNna),
            'ꧤ' => Ok(MyanmarExtendedB::MyanmarLetterShanBha),
            'ꧥ' => Ok(MyanmarExtendedB::MyanmarSignShanSaw),
            'ꧦ' => Ok(MyanmarExtendedB::MyanmarModifierLetterShanReduplication),
            'ꧧ' => Ok(MyanmarExtendedB::MyanmarLetterTaiLaingNya),
            'ꧨ' => Ok(MyanmarExtendedB::MyanmarLetterTaiLaingFa),
            'ꧩ' => Ok(MyanmarExtendedB::MyanmarLetterTaiLaingGa),
            'ꧪ' => Ok(MyanmarExtendedB::MyanmarLetterTaiLaingGha),
            'ꧫ' => Ok(MyanmarExtendedB::MyanmarLetterTaiLaingJa),
            'ꧬ' => Ok(MyanmarExtendedB::MyanmarLetterTaiLaingJha),
            'ꧭ' => Ok(MyanmarExtendedB::MyanmarLetterTaiLaingDda),
            'ꧮ' => Ok(MyanmarExtendedB::MyanmarLetterTaiLaingDdha),
            'ꧯ' => Ok(MyanmarExtendedB::MyanmarLetterTaiLaingNna),
            '꧰' => Ok(MyanmarExtendedB::MyanmarTaiLaingDigitZero),
            '꧱' => Ok(MyanmarExtendedB::MyanmarTaiLaingDigitOne),
            '꧲' => Ok(MyanmarExtendedB::MyanmarTaiLaingDigitTwo),
            '꧳' => Ok(MyanmarExtendedB::MyanmarTaiLaingDigitThree),
            '꧴' => Ok(MyanmarExtendedB::MyanmarTaiLaingDigitFour),
            '꧵' => Ok(MyanmarExtendedB::MyanmarTaiLaingDigitFive),
            '꧶' => Ok(MyanmarExtendedB::MyanmarTaiLaingDigitSix),
            '꧷' => Ok(MyanmarExtendedB::MyanmarTaiLaingDigitSeven),
            '꧸' => Ok(MyanmarExtendedB::MyanmarTaiLaingDigitEight),
            '꧹' => Ok(MyanmarExtendedB::MyanmarTaiLaingDigitNine),
            'ꧺ' => Ok(MyanmarExtendedB::MyanmarLetterTaiLaingLla),
            'ꧻ' => Ok(MyanmarExtendedB::MyanmarLetterTaiLaingDa),
            'ꧼ' => Ok(MyanmarExtendedB::MyanmarLetterTaiLaingDha),
            'ꧽ' => Ok(MyanmarExtendedB::MyanmarLetterTaiLaingBa),
            'ꧾ' => Ok(MyanmarExtendedB::MyanmarLetterTaiLaingBha),
            _ => Err(()),
        }
    }
}

impl Into<u32> for MyanmarExtendedB {
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

impl std::convert::TryFrom<u32> for MyanmarExtendedB {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for MyanmarExtendedB {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl MyanmarExtendedB {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        MyanmarExtendedB::MyanmarLetterShanGha
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            MyanmarExtendedB::MyanmarLetterShanGha => "myanmar letter shan gha",
            MyanmarExtendedB::MyanmarLetterShanCha => "myanmar letter shan cha",
            MyanmarExtendedB::MyanmarLetterShanJha => "myanmar letter shan jha",
            MyanmarExtendedB::MyanmarLetterShanNna => "myanmar letter shan nna",
            MyanmarExtendedB::MyanmarLetterShanBha => "myanmar letter shan bha",
            MyanmarExtendedB::MyanmarSignShanSaw => "myanmar sign shan saw",
            MyanmarExtendedB::MyanmarModifierLetterShanReduplication => "myanmar modifier letter shan reduplication",
            MyanmarExtendedB::MyanmarLetterTaiLaingNya => "myanmar letter tai laing nya",
            MyanmarExtendedB::MyanmarLetterTaiLaingFa => "myanmar letter tai laing fa",
            MyanmarExtendedB::MyanmarLetterTaiLaingGa => "myanmar letter tai laing ga",
            MyanmarExtendedB::MyanmarLetterTaiLaingGha => "myanmar letter tai laing gha",
            MyanmarExtendedB::MyanmarLetterTaiLaingJa => "myanmar letter tai laing ja",
            MyanmarExtendedB::MyanmarLetterTaiLaingJha => "myanmar letter tai laing jha",
            MyanmarExtendedB::MyanmarLetterTaiLaingDda => "myanmar letter tai laing dda",
            MyanmarExtendedB::MyanmarLetterTaiLaingDdha => "myanmar letter tai laing ddha",
            MyanmarExtendedB::MyanmarLetterTaiLaingNna => "myanmar letter tai laing nna",
            MyanmarExtendedB::MyanmarTaiLaingDigitZero => "myanmar tai laing digit zero",
            MyanmarExtendedB::MyanmarTaiLaingDigitOne => "myanmar tai laing digit one",
            MyanmarExtendedB::MyanmarTaiLaingDigitTwo => "myanmar tai laing digit two",
            MyanmarExtendedB::MyanmarTaiLaingDigitThree => "myanmar tai laing digit three",
            MyanmarExtendedB::MyanmarTaiLaingDigitFour => "myanmar tai laing digit four",
            MyanmarExtendedB::MyanmarTaiLaingDigitFive => "myanmar tai laing digit five",
            MyanmarExtendedB::MyanmarTaiLaingDigitSix => "myanmar tai laing digit six",
            MyanmarExtendedB::MyanmarTaiLaingDigitSeven => "myanmar tai laing digit seven",
            MyanmarExtendedB::MyanmarTaiLaingDigitEight => "myanmar tai laing digit eight",
            MyanmarExtendedB::MyanmarTaiLaingDigitNine => "myanmar tai laing digit nine",
            MyanmarExtendedB::MyanmarLetterTaiLaingLla => "myanmar letter tai laing lla",
            MyanmarExtendedB::MyanmarLetterTaiLaingDa => "myanmar letter tai laing da",
            MyanmarExtendedB::MyanmarLetterTaiLaingDha => "myanmar letter tai laing dha",
            MyanmarExtendedB::MyanmarLetterTaiLaingBa => "myanmar letter tai laing ba",
            MyanmarExtendedB::MyanmarLetterTaiLaingBha => "myanmar letter tai laing bha",
        }
    }
}
