
/// An enum to represent all characters in the EthiopicSupplement block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum EthiopicSupplement {
    /// \u{1380}: 'ᎀ'
    EthiopicSyllableSebatbeitMwa,
    /// \u{1381}: 'ᎁ'
    EthiopicSyllableMwi,
    /// \u{1382}: 'ᎂ'
    EthiopicSyllableMwee,
    /// \u{1383}: 'ᎃ'
    EthiopicSyllableMwe,
    /// \u{1384}: 'ᎄ'
    EthiopicSyllableSebatbeitBwa,
    /// \u{1385}: 'ᎅ'
    EthiopicSyllableBwi,
    /// \u{1386}: 'ᎆ'
    EthiopicSyllableBwee,
    /// \u{1387}: 'ᎇ'
    EthiopicSyllableBwe,
    /// \u{1388}: 'ᎈ'
    EthiopicSyllableSebatbeitFwa,
    /// \u{1389}: 'ᎉ'
    EthiopicSyllableFwi,
    /// \u{138a}: 'ᎊ'
    EthiopicSyllableFwee,
    /// \u{138b}: 'ᎋ'
    EthiopicSyllableFwe,
    /// \u{138c}: 'ᎌ'
    EthiopicSyllableSebatbeitPwa,
    /// \u{138d}: 'ᎍ'
    EthiopicSyllablePwi,
    /// \u{138e}: 'ᎎ'
    EthiopicSyllablePwee,
    /// \u{138f}: 'ᎏ'
    EthiopicSyllablePwe,
    /// \u{1390}: '᎐'
    EthiopicTonalMarkYizet,
    /// \u{1391}: '᎑'
    EthiopicTonalMarkDeret,
    /// \u{1392}: '᎒'
    EthiopicTonalMarkRikrik,
    /// \u{1393}: '᎓'
    EthiopicTonalMarkShortRikrik,
    /// \u{1394}: '᎔'
    EthiopicTonalMarkDifat,
    /// \u{1395}: '᎕'
    EthiopicTonalMarkKenat,
    /// \u{1396}: '᎖'
    EthiopicTonalMarkChiret,
    /// \u{1397}: '᎗'
    EthiopicTonalMarkHidet,
    /// \u{1398}: '᎘'
    EthiopicTonalMarkDeretDashHidet,
    /// \u{1399}: '᎙'
    EthiopicTonalMarkKurt,
}

impl Into<char> for EthiopicSupplement {
    fn into(self) -> char {
        match self {
            EthiopicSupplement::EthiopicSyllableSebatbeitMwa => 'ᎀ',
            EthiopicSupplement::EthiopicSyllableMwi => 'ᎁ',
            EthiopicSupplement::EthiopicSyllableMwee => 'ᎂ',
            EthiopicSupplement::EthiopicSyllableMwe => 'ᎃ',
            EthiopicSupplement::EthiopicSyllableSebatbeitBwa => 'ᎄ',
            EthiopicSupplement::EthiopicSyllableBwi => 'ᎅ',
            EthiopicSupplement::EthiopicSyllableBwee => 'ᎆ',
            EthiopicSupplement::EthiopicSyllableBwe => 'ᎇ',
            EthiopicSupplement::EthiopicSyllableSebatbeitFwa => 'ᎈ',
            EthiopicSupplement::EthiopicSyllableFwi => 'ᎉ',
            EthiopicSupplement::EthiopicSyllableFwee => 'ᎊ',
            EthiopicSupplement::EthiopicSyllableFwe => 'ᎋ',
            EthiopicSupplement::EthiopicSyllableSebatbeitPwa => 'ᎌ',
            EthiopicSupplement::EthiopicSyllablePwi => 'ᎍ',
            EthiopicSupplement::EthiopicSyllablePwee => 'ᎎ',
            EthiopicSupplement::EthiopicSyllablePwe => 'ᎏ',
            EthiopicSupplement::EthiopicTonalMarkYizet => '᎐',
            EthiopicSupplement::EthiopicTonalMarkDeret => '᎑',
            EthiopicSupplement::EthiopicTonalMarkRikrik => '᎒',
            EthiopicSupplement::EthiopicTonalMarkShortRikrik => '᎓',
            EthiopicSupplement::EthiopicTonalMarkDifat => '᎔',
            EthiopicSupplement::EthiopicTonalMarkKenat => '᎕',
            EthiopicSupplement::EthiopicTonalMarkChiret => '᎖',
            EthiopicSupplement::EthiopicTonalMarkHidet => '᎗',
            EthiopicSupplement::EthiopicTonalMarkDeretDashHidet => '᎘',
            EthiopicSupplement::EthiopicTonalMarkKurt => '᎙',
        }
    }
}

impl std::convert::TryFrom<char> for EthiopicSupplement {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ᎀ' => Ok(EthiopicSupplement::EthiopicSyllableSebatbeitMwa),
            'ᎁ' => Ok(EthiopicSupplement::EthiopicSyllableMwi),
            'ᎂ' => Ok(EthiopicSupplement::EthiopicSyllableMwee),
            'ᎃ' => Ok(EthiopicSupplement::EthiopicSyllableMwe),
            'ᎄ' => Ok(EthiopicSupplement::EthiopicSyllableSebatbeitBwa),
            'ᎅ' => Ok(EthiopicSupplement::EthiopicSyllableBwi),
            'ᎆ' => Ok(EthiopicSupplement::EthiopicSyllableBwee),
            'ᎇ' => Ok(EthiopicSupplement::EthiopicSyllableBwe),
            'ᎈ' => Ok(EthiopicSupplement::EthiopicSyllableSebatbeitFwa),
            'ᎉ' => Ok(EthiopicSupplement::EthiopicSyllableFwi),
            'ᎊ' => Ok(EthiopicSupplement::EthiopicSyllableFwee),
            'ᎋ' => Ok(EthiopicSupplement::EthiopicSyllableFwe),
            'ᎌ' => Ok(EthiopicSupplement::EthiopicSyllableSebatbeitPwa),
            'ᎍ' => Ok(EthiopicSupplement::EthiopicSyllablePwi),
            'ᎎ' => Ok(EthiopicSupplement::EthiopicSyllablePwee),
            'ᎏ' => Ok(EthiopicSupplement::EthiopicSyllablePwe),
            '᎐' => Ok(EthiopicSupplement::EthiopicTonalMarkYizet),
            '᎑' => Ok(EthiopicSupplement::EthiopicTonalMarkDeret),
            '᎒' => Ok(EthiopicSupplement::EthiopicTonalMarkRikrik),
            '᎓' => Ok(EthiopicSupplement::EthiopicTonalMarkShortRikrik),
            '᎔' => Ok(EthiopicSupplement::EthiopicTonalMarkDifat),
            '᎕' => Ok(EthiopicSupplement::EthiopicTonalMarkKenat),
            '᎖' => Ok(EthiopicSupplement::EthiopicTonalMarkChiret),
            '᎗' => Ok(EthiopicSupplement::EthiopicTonalMarkHidet),
            '᎘' => Ok(EthiopicSupplement::EthiopicTonalMarkDeretDashHidet),
            '᎙' => Ok(EthiopicSupplement::EthiopicTonalMarkKurt),
            _ => Err(()),
        }
    }
}

impl Into<u32> for EthiopicSupplement {
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

impl std::convert::TryFrom<u32> for EthiopicSupplement {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for EthiopicSupplement {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl EthiopicSupplement {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        EthiopicSupplement::EthiopicSyllableSebatbeitMwa
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            EthiopicSupplement::EthiopicSyllableSebatbeitMwa => "ethiopic syllable sebatbeit mwa",
            EthiopicSupplement::EthiopicSyllableMwi => "ethiopic syllable mwi",
            EthiopicSupplement::EthiopicSyllableMwee => "ethiopic syllable mwee",
            EthiopicSupplement::EthiopicSyllableMwe => "ethiopic syllable mwe",
            EthiopicSupplement::EthiopicSyllableSebatbeitBwa => "ethiopic syllable sebatbeit bwa",
            EthiopicSupplement::EthiopicSyllableBwi => "ethiopic syllable bwi",
            EthiopicSupplement::EthiopicSyllableBwee => "ethiopic syllable bwee",
            EthiopicSupplement::EthiopicSyllableBwe => "ethiopic syllable bwe",
            EthiopicSupplement::EthiopicSyllableSebatbeitFwa => "ethiopic syllable sebatbeit fwa",
            EthiopicSupplement::EthiopicSyllableFwi => "ethiopic syllable fwi",
            EthiopicSupplement::EthiopicSyllableFwee => "ethiopic syllable fwee",
            EthiopicSupplement::EthiopicSyllableFwe => "ethiopic syllable fwe",
            EthiopicSupplement::EthiopicSyllableSebatbeitPwa => "ethiopic syllable sebatbeit pwa",
            EthiopicSupplement::EthiopicSyllablePwi => "ethiopic syllable pwi",
            EthiopicSupplement::EthiopicSyllablePwee => "ethiopic syllable pwee",
            EthiopicSupplement::EthiopicSyllablePwe => "ethiopic syllable pwe",
            EthiopicSupplement::EthiopicTonalMarkYizet => "ethiopic tonal mark yizet",
            EthiopicSupplement::EthiopicTonalMarkDeret => "ethiopic tonal mark deret",
            EthiopicSupplement::EthiopicTonalMarkRikrik => "ethiopic tonal mark rikrik",
            EthiopicSupplement::EthiopicTonalMarkShortRikrik => "ethiopic tonal mark short rikrik",
            EthiopicSupplement::EthiopicTonalMarkDifat => "ethiopic tonal mark difat",
            EthiopicSupplement::EthiopicTonalMarkKenat => "ethiopic tonal mark kenat",
            EthiopicSupplement::EthiopicTonalMarkChiret => "ethiopic tonal mark chiret",
            EthiopicSupplement::EthiopicTonalMarkHidet => "ethiopic tonal mark hidet",
            EthiopicSupplement::EthiopicTonalMarkDeretDashHidet => "ethiopic tonal mark deret-hidet",
            EthiopicSupplement::EthiopicTonalMarkKurt => "ethiopic tonal mark kurt",
        }
    }
}
