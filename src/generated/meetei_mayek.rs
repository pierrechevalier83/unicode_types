/// \u{abc0} → \u{abff}\
///\
/// ꯀ ꯁ ꯂ ꯃ ꯄ ꯅ ꯆ ꯇ ꯈ ꯉ ꯊ ꯋ ꯌ ꯍ ꯎ ꯏ\
/// ꯐ ꯑ ꯒ ꯓ ꯔ ꯕ ꯖ ꯗ ꯘ ꯙ ꯚ ꯛ ꯜ ꯝ ꯞ ꯟ\
/// ꯠ ꯡ ꯢ ꯣ ꯤ ꯥ ꯦ ꯧ ꯨ ꯩ ꯪ ꯫ ꯬ ꯭ ꯰ ꯱\
/// ꯲ ꯳ ꯴ ꯵ ꯶ ꯷ ꯸ ꯹\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{abc0}: 'ꯀ'
    pub const LETTER_KOK: char = 'ꯀ';
    /// \u{abc1}: 'ꯁ'
    pub const LETTER_SAM: char = 'ꯁ';
    /// \u{abc2}: 'ꯂ'
    pub const LETTER_LAI: char = 'ꯂ';
    /// \u{abc3}: 'ꯃ'
    pub const LETTER_MIT: char = 'ꯃ';
    /// \u{abc4}: 'ꯄ'
    pub const LETTER_PA: char = 'ꯄ';
    /// \u{abc5}: 'ꯅ'
    pub const LETTER_NA: char = 'ꯅ';
    /// \u{abc6}: 'ꯆ'
    pub const LETTER_CHIL: char = 'ꯆ';
    /// \u{abc7}: 'ꯇ'
    pub const LETTER_TIL: char = 'ꯇ';
    /// \u{abc8}: 'ꯈ'
    pub const LETTER_KHOU: char = 'ꯈ';
    /// \u{abc9}: 'ꯉ'
    pub const LETTER_NGOU: char = 'ꯉ';
    /// \u{abca}: 'ꯊ'
    pub const LETTER_THOU: char = 'ꯊ';
    /// \u{abcb}: 'ꯋ'
    pub const LETTER_WAI: char = 'ꯋ';
    /// \u{abcc}: 'ꯌ'
    pub const LETTER_YANG: char = 'ꯌ';
    /// \u{abcd}: 'ꯍ'
    pub const LETTER_HUK: char = 'ꯍ';
    /// \u{abce}: 'ꯎ'
    pub const LETTER_UN: char = 'ꯎ';
    /// \u{abcf}: 'ꯏ'
    pub const LETTER_I: char = 'ꯏ';
    /// \u{abd0}: 'ꯐ'
    pub const LETTER_PHAM: char = 'ꯐ';
    /// \u{abd1}: 'ꯑ'
    pub const LETTER_ATIYA: char = 'ꯑ';
    /// \u{abd2}: 'ꯒ'
    pub const LETTER_GOK: char = 'ꯒ';
    /// \u{abd3}: 'ꯓ'
    pub const LETTER_JHAM: char = 'ꯓ';
    /// \u{abd4}: 'ꯔ'
    pub const LETTER_RAI: char = 'ꯔ';
    /// \u{abd5}: 'ꯕ'
    pub const LETTER_BA: char = 'ꯕ';
    /// \u{abd6}: 'ꯖ'
    pub const LETTER_JIL: char = 'ꯖ';
    /// \u{abd7}: 'ꯗ'
    pub const LETTER_DIL: char = 'ꯗ';
    /// \u{abd8}: 'ꯘ'
    pub const LETTER_GHOU: char = 'ꯘ';
    /// \u{abd9}: 'ꯙ'
    pub const LETTER_DHOU: char = 'ꯙ';
    /// \u{abda}: 'ꯚ'
    pub const LETTER_BHAM: char = 'ꯚ';
    /// \u{abdb}: 'ꯛ'
    pub const LETTER_KOK_LONSUM: char = 'ꯛ';
    /// \u{abdc}: 'ꯜ'
    pub const LETTER_LAI_LONSUM: char = 'ꯜ';
    /// \u{abdd}: 'ꯝ'
    pub const LETTER_MIT_LONSUM: char = 'ꯝ';
    /// \u{abde}: 'ꯞ'
    pub const LETTER_PA_LONSUM: char = 'ꯞ';
    /// \u{abdf}: 'ꯟ'
    pub const LETTER_NA_LONSUM: char = 'ꯟ';
    /// \u{abe0}: 'ꯠ'
    pub const LETTER_TIL_LONSUM: char = 'ꯠ';
    /// \u{abe1}: 'ꯡ'
    pub const LETTER_NGOU_LONSUM: char = 'ꯡ';
    /// \u{abe2}: 'ꯢ'
    pub const LETTER_I_LONSUM: char = 'ꯢ';
    /// \u{abe3}: 'ꯣ'
    pub const VOWEL_SIGN_ONAP: char = 'ꯣ';
    /// \u{abe4}: 'ꯤ'
    pub const VOWEL_SIGN_INAP: char = 'ꯤ';
    /// \u{abe5}: 'ꯥ'
    pub const VOWEL_SIGN_ANAP: char = 'ꯥ';
    /// \u{abe6}: 'ꯦ'
    pub const VOWEL_SIGN_YENAP: char = 'ꯦ';
    /// \u{abe7}: 'ꯧ'
    pub const VOWEL_SIGN_SOUNAP: char = 'ꯧ';
    /// \u{abe8}: 'ꯨ'
    pub const VOWEL_SIGN_UNAP: char = 'ꯨ';
    /// \u{abe9}: 'ꯩ'
    pub const VOWEL_SIGN_CHEINAP: char = 'ꯩ';
    /// \u{abea}: 'ꯪ'
    pub const VOWEL_SIGN_NUNG: char = 'ꯪ';
    /// \u{abeb}: '꯫'
    pub const CHEIKHEI: char = '꯫';
    /// \u{abec}: '꯬'
    pub const LUM_IYEK: char = '꯬';
    /// \u{abed}: '꯭'
    pub const APUN_IYEK: char = '꯭';
    /// \u{abf0}: '꯰'
    pub const DIGIT_ZERO: char = '꯰';
    /// \u{abf1}: '꯱'
    pub const DIGIT_ONE: char = '꯱';
    /// \u{abf2}: '꯲'
    pub const DIGIT_TWO: char = '꯲';
    /// \u{abf3}: '꯳'
    pub const DIGIT_THREE: char = '꯳';
    /// \u{abf4}: '꯴'
    pub const DIGIT_FOUR: char = '꯴';
    /// \u{abf5}: '꯵'
    pub const DIGIT_FIVE: char = '꯵';
    /// \u{abf6}: '꯶'
    pub const DIGIT_SIX: char = '꯶';
    /// \u{abf7}: '꯷'
    pub const DIGIT_SEVEN: char = '꯷';
    /// \u{abf8}: '꯸'
    pub const DIGIT_EIGHT: char = '꯸';
    /// \u{abf9}: '꯹'
    pub const DIGIT_NINE: char = '꯹';
}

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
        use constants::*;
        match self {
            MeeteiMayek::LetterKok => LETTER_KOK,
            MeeteiMayek::LetterSam => LETTER_SAM,
            MeeteiMayek::LetterLai => LETTER_LAI,
            MeeteiMayek::LetterMit => LETTER_MIT,
            MeeteiMayek::LetterPa => LETTER_PA,
            MeeteiMayek::LetterNa => LETTER_NA,
            MeeteiMayek::LetterChil => LETTER_CHIL,
            MeeteiMayek::LetterTil => LETTER_TIL,
            MeeteiMayek::LetterKhou => LETTER_KHOU,
            MeeteiMayek::LetterNgou => LETTER_NGOU,
            MeeteiMayek::LetterThou => LETTER_THOU,
            MeeteiMayek::LetterWai => LETTER_WAI,
            MeeteiMayek::LetterYang => LETTER_YANG,
            MeeteiMayek::LetterHuk => LETTER_HUK,
            MeeteiMayek::LetterUn => LETTER_UN,
            MeeteiMayek::LetterI => LETTER_I,
            MeeteiMayek::LetterPham => LETTER_PHAM,
            MeeteiMayek::LetterAtiya => LETTER_ATIYA,
            MeeteiMayek::LetterGok => LETTER_GOK,
            MeeteiMayek::LetterJham => LETTER_JHAM,
            MeeteiMayek::LetterRai => LETTER_RAI,
            MeeteiMayek::LetterBa => LETTER_BA,
            MeeteiMayek::LetterJil => LETTER_JIL,
            MeeteiMayek::LetterDil => LETTER_DIL,
            MeeteiMayek::LetterGhou => LETTER_GHOU,
            MeeteiMayek::LetterDhou => LETTER_DHOU,
            MeeteiMayek::LetterBham => LETTER_BHAM,
            MeeteiMayek::LetterKokLonsum => LETTER_KOK_LONSUM,
            MeeteiMayek::LetterLaiLonsum => LETTER_LAI_LONSUM,
            MeeteiMayek::LetterMitLonsum => LETTER_MIT_LONSUM,
            MeeteiMayek::LetterPaLonsum => LETTER_PA_LONSUM,
            MeeteiMayek::LetterNaLonsum => LETTER_NA_LONSUM,
            MeeteiMayek::LetterTilLonsum => LETTER_TIL_LONSUM,
            MeeteiMayek::LetterNgouLonsum => LETTER_NGOU_LONSUM,
            MeeteiMayek::LetterILonsum => LETTER_I_LONSUM,
            MeeteiMayek::VowelSignOnap => VOWEL_SIGN_ONAP,
            MeeteiMayek::VowelSignInap => VOWEL_SIGN_INAP,
            MeeteiMayek::VowelSignAnap => VOWEL_SIGN_ANAP,
            MeeteiMayek::VowelSignYenap => VOWEL_SIGN_YENAP,
            MeeteiMayek::VowelSignSounap => VOWEL_SIGN_SOUNAP,
            MeeteiMayek::VowelSignUnap => VOWEL_SIGN_UNAP,
            MeeteiMayek::VowelSignCheinap => VOWEL_SIGN_CHEINAP,
            MeeteiMayek::VowelSignNung => VOWEL_SIGN_NUNG,
            MeeteiMayek::Cheikhei => CHEIKHEI,
            MeeteiMayek::LumIyek => LUM_IYEK,
            MeeteiMayek::ApunIyek => APUN_IYEK,
            MeeteiMayek::DigitZero => DIGIT_ZERO,
            MeeteiMayek::DigitOne => DIGIT_ONE,
            MeeteiMayek::DigitTwo => DIGIT_TWO,
            MeeteiMayek::DigitThree => DIGIT_THREE,
            MeeteiMayek::DigitFour => DIGIT_FOUR,
            MeeteiMayek::DigitFive => DIGIT_FIVE,
            MeeteiMayek::DigitSix => DIGIT_SIX,
            MeeteiMayek::DigitSeven => DIGIT_SEVEN,
            MeeteiMayek::DigitEight => DIGIT_EIGHT,
            MeeteiMayek::DigitNine => DIGIT_NINE,
        }
    }
}

impl std::convert::TryFrom<char> for MeeteiMayek {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_KOK => Ok(MeeteiMayek::LetterKok),
            LETTER_SAM => Ok(MeeteiMayek::LetterSam),
            LETTER_LAI => Ok(MeeteiMayek::LetterLai),
            LETTER_MIT => Ok(MeeteiMayek::LetterMit),
            LETTER_PA => Ok(MeeteiMayek::LetterPa),
            LETTER_NA => Ok(MeeteiMayek::LetterNa),
            LETTER_CHIL => Ok(MeeteiMayek::LetterChil),
            LETTER_TIL => Ok(MeeteiMayek::LetterTil),
            LETTER_KHOU => Ok(MeeteiMayek::LetterKhou),
            LETTER_NGOU => Ok(MeeteiMayek::LetterNgou),
            LETTER_THOU => Ok(MeeteiMayek::LetterThou),
            LETTER_WAI => Ok(MeeteiMayek::LetterWai),
            LETTER_YANG => Ok(MeeteiMayek::LetterYang),
            LETTER_HUK => Ok(MeeteiMayek::LetterHuk),
            LETTER_UN => Ok(MeeteiMayek::LetterUn),
            LETTER_I => Ok(MeeteiMayek::LetterI),
            LETTER_PHAM => Ok(MeeteiMayek::LetterPham),
            LETTER_ATIYA => Ok(MeeteiMayek::LetterAtiya),
            LETTER_GOK => Ok(MeeteiMayek::LetterGok),
            LETTER_JHAM => Ok(MeeteiMayek::LetterJham),
            LETTER_RAI => Ok(MeeteiMayek::LetterRai),
            LETTER_BA => Ok(MeeteiMayek::LetterBa),
            LETTER_JIL => Ok(MeeteiMayek::LetterJil),
            LETTER_DIL => Ok(MeeteiMayek::LetterDil),
            LETTER_GHOU => Ok(MeeteiMayek::LetterGhou),
            LETTER_DHOU => Ok(MeeteiMayek::LetterDhou),
            LETTER_BHAM => Ok(MeeteiMayek::LetterBham),
            LETTER_KOK_LONSUM => Ok(MeeteiMayek::LetterKokLonsum),
            LETTER_LAI_LONSUM => Ok(MeeteiMayek::LetterLaiLonsum),
            LETTER_MIT_LONSUM => Ok(MeeteiMayek::LetterMitLonsum),
            LETTER_PA_LONSUM => Ok(MeeteiMayek::LetterPaLonsum),
            LETTER_NA_LONSUM => Ok(MeeteiMayek::LetterNaLonsum),
            LETTER_TIL_LONSUM => Ok(MeeteiMayek::LetterTilLonsum),
            LETTER_NGOU_LONSUM => Ok(MeeteiMayek::LetterNgouLonsum),
            LETTER_I_LONSUM => Ok(MeeteiMayek::LetterILonsum),
            VOWEL_SIGN_ONAP => Ok(MeeteiMayek::VowelSignOnap),
            VOWEL_SIGN_INAP => Ok(MeeteiMayek::VowelSignInap),
            VOWEL_SIGN_ANAP => Ok(MeeteiMayek::VowelSignAnap),
            VOWEL_SIGN_YENAP => Ok(MeeteiMayek::VowelSignYenap),
            VOWEL_SIGN_SOUNAP => Ok(MeeteiMayek::VowelSignSounap),
            VOWEL_SIGN_UNAP => Ok(MeeteiMayek::VowelSignUnap),
            VOWEL_SIGN_CHEINAP => Ok(MeeteiMayek::VowelSignCheinap),
            VOWEL_SIGN_NUNG => Ok(MeeteiMayek::VowelSignNung),
            CHEIKHEI => Ok(MeeteiMayek::Cheikhei),
            LUM_IYEK => Ok(MeeteiMayek::LumIyek),
            APUN_IYEK => Ok(MeeteiMayek::ApunIyek),
            DIGIT_ZERO => Ok(MeeteiMayek::DigitZero),
            DIGIT_ONE => Ok(MeeteiMayek::DigitOne),
            DIGIT_TWO => Ok(MeeteiMayek::DigitTwo),
            DIGIT_THREE => Ok(MeeteiMayek::DigitThree),
            DIGIT_FOUR => Ok(MeeteiMayek::DigitFour),
            DIGIT_FIVE => Ok(MeeteiMayek::DigitFive),
            DIGIT_SIX => Ok(MeeteiMayek::DigitSix),
            DIGIT_SEVEN => Ok(MeeteiMayek::DigitSeven),
            DIGIT_EIGHT => Ok(MeeteiMayek::DigitEight),
            DIGIT_NINE => Ok(MeeteiMayek::DigitNine),
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
