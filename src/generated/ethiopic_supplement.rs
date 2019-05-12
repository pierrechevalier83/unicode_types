/// \u{1380} → \u{139f}\
///\
/// ᎀ ᎁ ᎂ ᎃ ᎄ ᎅ ᎆ ᎇ ᎈ ᎉ ᎊ ᎋ ᎌ ᎍ ᎎ ᎏ
/// ᎐ ᎑ ᎒ ᎓ ᎔ ᎕ ᎖ ᎗ ᎘ ᎙
pub mod constants {
    /// \u{1380}: 'ᎀ'
    pub const ETHIOPIC_SYLLABLE_SEBATBEIT_MWA: char = 'ᎀ';
    /// \u{1381}: 'ᎁ'
    pub const ETHIOPIC_SYLLABLE_MWI: char = 'ᎁ';
    /// \u{1382}: 'ᎂ'
    pub const ETHIOPIC_SYLLABLE_MWEE: char = 'ᎂ';
    /// \u{1383}: 'ᎃ'
    pub const ETHIOPIC_SYLLABLE_MWE: char = 'ᎃ';
    /// \u{1384}: 'ᎄ'
    pub const ETHIOPIC_SYLLABLE_SEBATBEIT_BWA: char = 'ᎄ';
    /// \u{1385}: 'ᎅ'
    pub const ETHIOPIC_SYLLABLE_BWI: char = 'ᎅ';
    /// \u{1386}: 'ᎆ'
    pub const ETHIOPIC_SYLLABLE_BWEE: char = 'ᎆ';
    /// \u{1387}: 'ᎇ'
    pub const ETHIOPIC_SYLLABLE_BWE: char = 'ᎇ';
    /// \u{1388}: 'ᎈ'
    pub const ETHIOPIC_SYLLABLE_SEBATBEIT_FWA: char = 'ᎈ';
    /// \u{1389}: 'ᎉ'
    pub const ETHIOPIC_SYLLABLE_FWI: char = 'ᎉ';
    /// \u{138a}: 'ᎊ'
    pub const ETHIOPIC_SYLLABLE_FWEE: char = 'ᎊ';
    /// \u{138b}: 'ᎋ'
    pub const ETHIOPIC_SYLLABLE_FWE: char = 'ᎋ';
    /// \u{138c}: 'ᎌ'
    pub const ETHIOPIC_SYLLABLE_SEBATBEIT_PWA: char = 'ᎌ';
    /// \u{138d}: 'ᎍ'
    pub const ETHIOPIC_SYLLABLE_PWI: char = 'ᎍ';
    /// \u{138e}: 'ᎎ'
    pub const ETHIOPIC_SYLLABLE_PWEE: char = 'ᎎ';
    /// \u{138f}: 'ᎏ'
    pub const ETHIOPIC_SYLLABLE_PWE: char = 'ᎏ';
    /// \u{1390}: '᎐'
    pub const ETHIOPIC_TONAL_MARK_YIZET: char = '᎐';
    /// \u{1391}: '᎑'
    pub const ETHIOPIC_TONAL_MARK_DERET: char = '᎑';
    /// \u{1392}: '᎒'
    pub const ETHIOPIC_TONAL_MARK_RIKRIK: char = '᎒';
    /// \u{1393}: '᎓'
    pub const ETHIOPIC_TONAL_MARK_SHORT_RIKRIK: char = '᎓';
    /// \u{1394}: '᎔'
    pub const ETHIOPIC_TONAL_MARK_DIFAT: char = '᎔';
    /// \u{1395}: '᎕'
    pub const ETHIOPIC_TONAL_MARK_KENAT: char = '᎕';
    /// \u{1396}: '᎖'
    pub const ETHIOPIC_TONAL_MARK_CHIRET: char = '᎖';
    /// \u{1397}: '᎗'
    pub const ETHIOPIC_TONAL_MARK_HIDET: char = '᎗';
    /// \u{1398}: '᎘'
    pub const ETHIOPIC_TONAL_MARK_DERET_DASH_HIDET: char = '᎘';
    /// \u{1399}: '᎙'
    pub const ETHIOPIC_TONAL_MARK_KURT: char = '᎙';
}

/// \u{1380} → \u{139f}\
///\
/// ᎀ ᎁ ᎂ ᎃ ᎄ ᎅ ᎆ ᎇ ᎈ ᎉ ᎊ ᎋ ᎌ ᎍ ᎎ ᎏ
/// ᎐ ᎑ ᎒ ᎓ ᎔ ᎕ ᎖ ᎗ ᎘ ᎙
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
        use constants::*;
        match self {
            EthiopicSupplement::EthiopicSyllableSebatbeitMwa => ETHIOPIC_SYLLABLE_SEBATBEIT_MWA,
            EthiopicSupplement::EthiopicSyllableMwi => ETHIOPIC_SYLLABLE_MWI,
            EthiopicSupplement::EthiopicSyllableMwee => ETHIOPIC_SYLLABLE_MWEE,
            EthiopicSupplement::EthiopicSyllableMwe => ETHIOPIC_SYLLABLE_MWE,
            EthiopicSupplement::EthiopicSyllableSebatbeitBwa => ETHIOPIC_SYLLABLE_SEBATBEIT_BWA,
            EthiopicSupplement::EthiopicSyllableBwi => ETHIOPIC_SYLLABLE_BWI,
            EthiopicSupplement::EthiopicSyllableBwee => ETHIOPIC_SYLLABLE_BWEE,
            EthiopicSupplement::EthiopicSyllableBwe => ETHIOPIC_SYLLABLE_BWE,
            EthiopicSupplement::EthiopicSyllableSebatbeitFwa => ETHIOPIC_SYLLABLE_SEBATBEIT_FWA,
            EthiopicSupplement::EthiopicSyllableFwi => ETHIOPIC_SYLLABLE_FWI,
            EthiopicSupplement::EthiopicSyllableFwee => ETHIOPIC_SYLLABLE_FWEE,
            EthiopicSupplement::EthiopicSyllableFwe => ETHIOPIC_SYLLABLE_FWE,
            EthiopicSupplement::EthiopicSyllableSebatbeitPwa => ETHIOPIC_SYLLABLE_SEBATBEIT_PWA,
            EthiopicSupplement::EthiopicSyllablePwi => ETHIOPIC_SYLLABLE_PWI,
            EthiopicSupplement::EthiopicSyllablePwee => ETHIOPIC_SYLLABLE_PWEE,
            EthiopicSupplement::EthiopicSyllablePwe => ETHIOPIC_SYLLABLE_PWE,
            EthiopicSupplement::EthiopicTonalMarkYizet => ETHIOPIC_TONAL_MARK_YIZET,
            EthiopicSupplement::EthiopicTonalMarkDeret => ETHIOPIC_TONAL_MARK_DERET,
            EthiopicSupplement::EthiopicTonalMarkRikrik => ETHIOPIC_TONAL_MARK_RIKRIK,
            EthiopicSupplement::EthiopicTonalMarkShortRikrik => ETHIOPIC_TONAL_MARK_SHORT_RIKRIK,
            EthiopicSupplement::EthiopicTonalMarkDifat => ETHIOPIC_TONAL_MARK_DIFAT,
            EthiopicSupplement::EthiopicTonalMarkKenat => ETHIOPIC_TONAL_MARK_KENAT,
            EthiopicSupplement::EthiopicTonalMarkChiret => ETHIOPIC_TONAL_MARK_CHIRET,
            EthiopicSupplement::EthiopicTonalMarkHidet => ETHIOPIC_TONAL_MARK_HIDET,
            EthiopicSupplement::EthiopicTonalMarkDeretDashHidet => ETHIOPIC_TONAL_MARK_DERET_DASH_HIDET,
            EthiopicSupplement::EthiopicTonalMarkKurt => ETHIOPIC_TONAL_MARK_KURT,
        }
    }
}

impl std::convert::TryFrom<char> for EthiopicSupplement {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            ETHIOPIC_SYLLABLE_SEBATBEIT_MWA => Ok(EthiopicSupplement::EthiopicSyllableSebatbeitMwa),
            ETHIOPIC_SYLLABLE_MWI => Ok(EthiopicSupplement::EthiopicSyllableMwi),
            ETHIOPIC_SYLLABLE_MWEE => Ok(EthiopicSupplement::EthiopicSyllableMwee),
            ETHIOPIC_SYLLABLE_MWE => Ok(EthiopicSupplement::EthiopicSyllableMwe),
            ETHIOPIC_SYLLABLE_SEBATBEIT_BWA => Ok(EthiopicSupplement::EthiopicSyllableSebatbeitBwa),
            ETHIOPIC_SYLLABLE_BWI => Ok(EthiopicSupplement::EthiopicSyllableBwi),
            ETHIOPIC_SYLLABLE_BWEE => Ok(EthiopicSupplement::EthiopicSyllableBwee),
            ETHIOPIC_SYLLABLE_BWE => Ok(EthiopicSupplement::EthiopicSyllableBwe),
            ETHIOPIC_SYLLABLE_SEBATBEIT_FWA => Ok(EthiopicSupplement::EthiopicSyllableSebatbeitFwa),
            ETHIOPIC_SYLLABLE_FWI => Ok(EthiopicSupplement::EthiopicSyllableFwi),
            ETHIOPIC_SYLLABLE_FWEE => Ok(EthiopicSupplement::EthiopicSyllableFwee),
            ETHIOPIC_SYLLABLE_FWE => Ok(EthiopicSupplement::EthiopicSyllableFwe),
            ETHIOPIC_SYLLABLE_SEBATBEIT_PWA => Ok(EthiopicSupplement::EthiopicSyllableSebatbeitPwa),
            ETHIOPIC_SYLLABLE_PWI => Ok(EthiopicSupplement::EthiopicSyllablePwi),
            ETHIOPIC_SYLLABLE_PWEE => Ok(EthiopicSupplement::EthiopicSyllablePwee),
            ETHIOPIC_SYLLABLE_PWE => Ok(EthiopicSupplement::EthiopicSyllablePwe),
            ETHIOPIC_TONAL_MARK_YIZET => Ok(EthiopicSupplement::EthiopicTonalMarkYizet),
            ETHIOPIC_TONAL_MARK_DERET => Ok(EthiopicSupplement::EthiopicTonalMarkDeret),
            ETHIOPIC_TONAL_MARK_RIKRIK => Ok(EthiopicSupplement::EthiopicTonalMarkRikrik),
            ETHIOPIC_TONAL_MARK_SHORT_RIKRIK => Ok(EthiopicSupplement::EthiopicTonalMarkShortRikrik),
            ETHIOPIC_TONAL_MARK_DIFAT => Ok(EthiopicSupplement::EthiopicTonalMarkDifat),
            ETHIOPIC_TONAL_MARK_KENAT => Ok(EthiopicSupplement::EthiopicTonalMarkKenat),
            ETHIOPIC_TONAL_MARK_CHIRET => Ok(EthiopicSupplement::EthiopicTonalMarkChiret),
            ETHIOPIC_TONAL_MARK_HIDET => Ok(EthiopicSupplement::EthiopicTonalMarkHidet),
            ETHIOPIC_TONAL_MARK_DERET_DASH_HIDET => Ok(EthiopicSupplement::EthiopicTonalMarkDeretDashHidet),
            ETHIOPIC_TONAL_MARK_KURT => Ok(EthiopicSupplement::EthiopicTonalMarkKurt),
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
