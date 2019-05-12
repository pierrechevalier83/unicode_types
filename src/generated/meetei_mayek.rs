
/// An enum to represent all characters in the MeeteiMayek block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum MeeteiMayek {
    /// \u{abc0}: 'ꯀ'
    LetterKok,
    /// \u{abc1}: 'ꯁ'
    LetterSam,
    /// \u{abc2}: 'ꯂ'
    LetterLai,
    /// \u{abc3}: 'ꯃ'
    LetterMit,
    /// \u{abc4}: 'ꯄ'
    LetterPa,
    /// \u{abc5}: 'ꯅ'
    LetterNa,
    /// \u{abc6}: 'ꯆ'
    LetterChil,
    /// \u{abc7}: 'ꯇ'
    LetterTil,
    /// \u{abc8}: 'ꯈ'
    LetterKhou,
    /// \u{abc9}: 'ꯉ'
    LetterNgou,
    /// \u{abca}: 'ꯊ'
    LetterThou,
    /// \u{abcb}: 'ꯋ'
    LetterWai,
    /// \u{abcc}: 'ꯌ'
    LetterYang,
    /// \u{abcd}: 'ꯍ'
    LetterHuk,
    /// \u{abce}: 'ꯎ'
    LetterUn,
    /// \u{abcf}: 'ꯏ'
    LetterI,
    /// \u{abd0}: 'ꯐ'
    LetterPham,
    /// \u{abd1}: 'ꯑ'
    LetterAtiya,
    /// \u{abd2}: 'ꯒ'
    LetterGok,
    /// \u{abd3}: 'ꯓ'
    LetterJham,
    /// \u{abd4}: 'ꯔ'
    LetterRai,
    /// \u{abd5}: 'ꯕ'
    LetterBa,
    /// \u{abd6}: 'ꯖ'
    LetterJil,
    /// \u{abd7}: 'ꯗ'
    LetterDil,
    /// \u{abd8}: 'ꯘ'
    LetterGhou,
    /// \u{abd9}: 'ꯙ'
    LetterDhou,
    /// \u{abda}: 'ꯚ'
    LetterBham,
    /// \u{abdb}: 'ꯛ'
    LetterKokLonsum,
    /// \u{abdc}: 'ꯜ'
    LetterLaiLonsum,
    /// \u{abdd}: 'ꯝ'
    LetterMitLonsum,
    /// \u{abde}: 'ꯞ'
    LetterPaLonsum,
    /// \u{abdf}: 'ꯟ'
    LetterNaLonsum,
    /// \u{abe0}: 'ꯠ'
    LetterTilLonsum,
    /// \u{abe1}: 'ꯡ'
    LetterNgouLonsum,
    /// \u{abe2}: 'ꯢ'
    LetterILonsum,
    /// \u{abe3}: 'ꯣ'
    VowelSignOnap,
    /// \u{abe4}: 'ꯤ'
    VowelSignInap,
    /// \u{abe5}: 'ꯥ'
    VowelSignAnap,
    /// \u{abe6}: 'ꯦ'
    VowelSignYenap,
    /// \u{abe7}: 'ꯧ'
    VowelSignSounap,
    /// \u{abe8}: 'ꯨ'
    VowelSignUnap,
    /// \u{abe9}: 'ꯩ'
    VowelSignCheinap,
    /// \u{abea}: 'ꯪ'
    VowelSignNung,
    /// \u{abeb}: '꯫'
    Cheikhei,
    /// \u{abec}: '꯬'
    LumIyek,
    /// \u{abed}: '꯭'
    ApunIyek,
    /// \u{abf0}: '꯰'
    DigitZero,
    /// \u{abf1}: '꯱'
    DigitOne,
    /// \u{abf2}: '꯲'
    DigitTwo,
    /// \u{abf3}: '꯳'
    DigitThree,
    /// \u{abf4}: '꯴'
    DigitFour,
    /// \u{abf5}: '꯵'
    DigitFive,
    /// \u{abf6}: '꯶'
    DigitSix,
    /// \u{abf7}: '꯷'
    DigitSeven,
    /// \u{abf8}: '꯸'
    DigitEight,
    /// \u{abf9}: '꯹'
    DigitNine,
}

impl Into<char> for MeeteiMayek {
    fn into(self) -> char {
        match self {
            MeeteiMayek::LetterKok => 'ꯀ',
            MeeteiMayek::LetterSam => 'ꯁ',
            MeeteiMayek::LetterLai => 'ꯂ',
            MeeteiMayek::LetterMit => 'ꯃ',
            MeeteiMayek::LetterPa => 'ꯄ',
            MeeteiMayek::LetterNa => 'ꯅ',
            MeeteiMayek::LetterChil => 'ꯆ',
            MeeteiMayek::LetterTil => 'ꯇ',
            MeeteiMayek::LetterKhou => 'ꯈ',
            MeeteiMayek::LetterNgou => 'ꯉ',
            MeeteiMayek::LetterThou => 'ꯊ',
            MeeteiMayek::LetterWai => 'ꯋ',
            MeeteiMayek::LetterYang => 'ꯌ',
            MeeteiMayek::LetterHuk => 'ꯍ',
            MeeteiMayek::LetterUn => 'ꯎ',
            MeeteiMayek::LetterI => 'ꯏ',
            MeeteiMayek::LetterPham => 'ꯐ',
            MeeteiMayek::LetterAtiya => 'ꯑ',
            MeeteiMayek::LetterGok => 'ꯒ',
            MeeteiMayek::LetterJham => 'ꯓ',
            MeeteiMayek::LetterRai => 'ꯔ',
            MeeteiMayek::LetterBa => 'ꯕ',
            MeeteiMayek::LetterJil => 'ꯖ',
            MeeteiMayek::LetterDil => 'ꯗ',
            MeeteiMayek::LetterGhou => 'ꯘ',
            MeeteiMayek::LetterDhou => 'ꯙ',
            MeeteiMayek::LetterBham => 'ꯚ',
            MeeteiMayek::LetterKokLonsum => 'ꯛ',
            MeeteiMayek::LetterLaiLonsum => 'ꯜ',
            MeeteiMayek::LetterMitLonsum => 'ꯝ',
            MeeteiMayek::LetterPaLonsum => 'ꯞ',
            MeeteiMayek::LetterNaLonsum => 'ꯟ',
            MeeteiMayek::LetterTilLonsum => 'ꯠ',
            MeeteiMayek::LetterNgouLonsum => 'ꯡ',
            MeeteiMayek::LetterILonsum => 'ꯢ',
            MeeteiMayek::VowelSignOnap => 'ꯣ',
            MeeteiMayek::VowelSignInap => 'ꯤ',
            MeeteiMayek::VowelSignAnap => 'ꯥ',
            MeeteiMayek::VowelSignYenap => 'ꯦ',
            MeeteiMayek::VowelSignSounap => 'ꯧ',
            MeeteiMayek::VowelSignUnap => 'ꯨ',
            MeeteiMayek::VowelSignCheinap => 'ꯩ',
            MeeteiMayek::VowelSignNung => 'ꯪ',
            MeeteiMayek::Cheikhei => '꯫',
            MeeteiMayek::LumIyek => '꯬',
            MeeteiMayek::ApunIyek => '꯭',
            MeeteiMayek::DigitZero => '꯰',
            MeeteiMayek::DigitOne => '꯱',
            MeeteiMayek::DigitTwo => '꯲',
            MeeteiMayek::DigitThree => '꯳',
            MeeteiMayek::DigitFour => '꯴',
            MeeteiMayek::DigitFive => '꯵',
            MeeteiMayek::DigitSix => '꯶',
            MeeteiMayek::DigitSeven => '꯷',
            MeeteiMayek::DigitEight => '꯸',
            MeeteiMayek::DigitNine => '꯹',
        }
    }
}

impl std::convert::TryFrom<char> for MeeteiMayek {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ꯀ' => Ok(MeeteiMayek::LetterKok),
            'ꯁ' => Ok(MeeteiMayek::LetterSam),
            'ꯂ' => Ok(MeeteiMayek::LetterLai),
            'ꯃ' => Ok(MeeteiMayek::LetterMit),
            'ꯄ' => Ok(MeeteiMayek::LetterPa),
            'ꯅ' => Ok(MeeteiMayek::LetterNa),
            'ꯆ' => Ok(MeeteiMayek::LetterChil),
            'ꯇ' => Ok(MeeteiMayek::LetterTil),
            'ꯈ' => Ok(MeeteiMayek::LetterKhou),
            'ꯉ' => Ok(MeeteiMayek::LetterNgou),
            'ꯊ' => Ok(MeeteiMayek::LetterThou),
            'ꯋ' => Ok(MeeteiMayek::LetterWai),
            'ꯌ' => Ok(MeeteiMayek::LetterYang),
            'ꯍ' => Ok(MeeteiMayek::LetterHuk),
            'ꯎ' => Ok(MeeteiMayek::LetterUn),
            'ꯏ' => Ok(MeeteiMayek::LetterI),
            'ꯐ' => Ok(MeeteiMayek::LetterPham),
            'ꯑ' => Ok(MeeteiMayek::LetterAtiya),
            'ꯒ' => Ok(MeeteiMayek::LetterGok),
            'ꯓ' => Ok(MeeteiMayek::LetterJham),
            'ꯔ' => Ok(MeeteiMayek::LetterRai),
            'ꯕ' => Ok(MeeteiMayek::LetterBa),
            'ꯖ' => Ok(MeeteiMayek::LetterJil),
            'ꯗ' => Ok(MeeteiMayek::LetterDil),
            'ꯘ' => Ok(MeeteiMayek::LetterGhou),
            'ꯙ' => Ok(MeeteiMayek::LetterDhou),
            'ꯚ' => Ok(MeeteiMayek::LetterBham),
            'ꯛ' => Ok(MeeteiMayek::LetterKokLonsum),
            'ꯜ' => Ok(MeeteiMayek::LetterLaiLonsum),
            'ꯝ' => Ok(MeeteiMayek::LetterMitLonsum),
            'ꯞ' => Ok(MeeteiMayek::LetterPaLonsum),
            'ꯟ' => Ok(MeeteiMayek::LetterNaLonsum),
            'ꯠ' => Ok(MeeteiMayek::LetterTilLonsum),
            'ꯡ' => Ok(MeeteiMayek::LetterNgouLonsum),
            'ꯢ' => Ok(MeeteiMayek::LetterILonsum),
            'ꯣ' => Ok(MeeteiMayek::VowelSignOnap),
            'ꯤ' => Ok(MeeteiMayek::VowelSignInap),
            'ꯥ' => Ok(MeeteiMayek::VowelSignAnap),
            'ꯦ' => Ok(MeeteiMayek::VowelSignYenap),
            'ꯧ' => Ok(MeeteiMayek::VowelSignSounap),
            'ꯨ' => Ok(MeeteiMayek::VowelSignUnap),
            'ꯩ' => Ok(MeeteiMayek::VowelSignCheinap),
            'ꯪ' => Ok(MeeteiMayek::VowelSignNung),
            '꯫' => Ok(MeeteiMayek::Cheikhei),
            '꯬' => Ok(MeeteiMayek::LumIyek),
            '꯭' => Ok(MeeteiMayek::ApunIyek),
            '꯰' => Ok(MeeteiMayek::DigitZero),
            '꯱' => Ok(MeeteiMayek::DigitOne),
            '꯲' => Ok(MeeteiMayek::DigitTwo),
            '꯳' => Ok(MeeteiMayek::DigitThree),
            '꯴' => Ok(MeeteiMayek::DigitFour),
            '꯵' => Ok(MeeteiMayek::DigitFive),
            '꯶' => Ok(MeeteiMayek::DigitSix),
            '꯷' => Ok(MeeteiMayek::DigitSeven),
            '꯸' => Ok(MeeteiMayek::DigitEight),
            '꯹' => Ok(MeeteiMayek::DigitNine),
            _ => Err(()),
        }
    }
}

impl Into<u32> for MeeteiMayek {
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

impl std::convert::TryFrom<u32> for MeeteiMayek {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for MeeteiMayek {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl MeeteiMayek {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        MeeteiMayek::LetterKok
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            MeeteiMayek::LetterKok => "meetei mayek letter kok",
            MeeteiMayek::LetterSam => "meetei mayek letter sam",
            MeeteiMayek::LetterLai => "meetei mayek letter lai",
            MeeteiMayek::LetterMit => "meetei mayek letter mit",
            MeeteiMayek::LetterPa => "meetei mayek letter pa",
            MeeteiMayek::LetterNa => "meetei mayek letter na",
            MeeteiMayek::LetterChil => "meetei mayek letter chil",
            MeeteiMayek::LetterTil => "meetei mayek letter til",
            MeeteiMayek::LetterKhou => "meetei mayek letter khou",
            MeeteiMayek::LetterNgou => "meetei mayek letter ngou",
            MeeteiMayek::LetterThou => "meetei mayek letter thou",
            MeeteiMayek::LetterWai => "meetei mayek letter wai",
            MeeteiMayek::LetterYang => "meetei mayek letter yang",
            MeeteiMayek::LetterHuk => "meetei mayek letter huk",
            MeeteiMayek::LetterUn => "meetei mayek letter un",
            MeeteiMayek::LetterI => "meetei mayek letter i",
            MeeteiMayek::LetterPham => "meetei mayek letter pham",
            MeeteiMayek::LetterAtiya => "meetei mayek letter atiya",
            MeeteiMayek::LetterGok => "meetei mayek letter gok",
            MeeteiMayek::LetterJham => "meetei mayek letter jham",
            MeeteiMayek::LetterRai => "meetei mayek letter rai",
            MeeteiMayek::LetterBa => "meetei mayek letter ba",
            MeeteiMayek::LetterJil => "meetei mayek letter jil",
            MeeteiMayek::LetterDil => "meetei mayek letter dil",
            MeeteiMayek::LetterGhou => "meetei mayek letter ghou",
            MeeteiMayek::LetterDhou => "meetei mayek letter dhou",
            MeeteiMayek::LetterBham => "meetei mayek letter bham",
            MeeteiMayek::LetterKokLonsum => "meetei mayek letter kok lonsum",
            MeeteiMayek::LetterLaiLonsum => "meetei mayek letter lai lonsum",
            MeeteiMayek::LetterMitLonsum => "meetei mayek letter mit lonsum",
            MeeteiMayek::LetterPaLonsum => "meetei mayek letter pa lonsum",
            MeeteiMayek::LetterNaLonsum => "meetei mayek letter na lonsum",
            MeeteiMayek::LetterTilLonsum => "meetei mayek letter til lonsum",
            MeeteiMayek::LetterNgouLonsum => "meetei mayek letter ngou lonsum",
            MeeteiMayek::LetterILonsum => "meetei mayek letter i lonsum",
            MeeteiMayek::VowelSignOnap => "meetei mayek vowel sign onap",
            MeeteiMayek::VowelSignInap => "meetei mayek vowel sign inap",
            MeeteiMayek::VowelSignAnap => "meetei mayek vowel sign anap",
            MeeteiMayek::VowelSignYenap => "meetei mayek vowel sign yenap",
            MeeteiMayek::VowelSignSounap => "meetei mayek vowel sign sounap",
            MeeteiMayek::VowelSignUnap => "meetei mayek vowel sign unap",
            MeeteiMayek::VowelSignCheinap => "meetei mayek vowel sign cheinap",
            MeeteiMayek::VowelSignNung => "meetei mayek vowel sign nung",
            MeeteiMayek::Cheikhei => "meetei mayek cheikhei",
            MeeteiMayek::LumIyek => "meetei mayek lum iyek",
            MeeteiMayek::ApunIyek => "meetei mayek apun iyek",
            MeeteiMayek::DigitZero => "meetei mayek digit zero",
            MeeteiMayek::DigitOne => "meetei mayek digit one",
            MeeteiMayek::DigitTwo => "meetei mayek digit two",
            MeeteiMayek::DigitThree => "meetei mayek digit three",
            MeeteiMayek::DigitFour => "meetei mayek digit four",
            MeeteiMayek::DigitFive => "meetei mayek digit five",
            MeeteiMayek::DigitSix => "meetei mayek digit six",
            MeeteiMayek::DigitSeven => "meetei mayek digit seven",
            MeeteiMayek::DigitEight => "meetei mayek digit eight",
            MeeteiMayek::DigitNine => "meetei mayek digit nine",
        }
    }
}
