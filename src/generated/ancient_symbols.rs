/// \u{10190} → \u{101cf}\
///\
/// 𐆐 𐆑 𐆒 𐆓 𐆔 𐆕 𐆖 𐆗 𐆘 𐆙 𐆚 𐆛 𐆠
pub mod constants {
    /// \u{10190}: '𐆐'
    pub const ROMAN_SEXTANS_SIGN: char = '𐆐';
    /// \u{10191}: '𐆑'
    pub const ROMAN_UNCIA_SIGN: char = '𐆑';
    /// \u{10192}: '𐆒'
    pub const ROMAN_SEMUNCIA_SIGN: char = '𐆒';
    /// \u{10193}: '𐆓'
    pub const ROMAN_SEXTULA_SIGN: char = '𐆓';
    /// \u{10194}: '𐆔'
    pub const ROMAN_DIMIDIA_SEXTULA_SIGN: char = '𐆔';
    /// \u{10195}: '𐆕'
    pub const ROMAN_SILIQUA_SIGN: char = '𐆕';
    /// \u{10196}: '𐆖'
    pub const ROMAN_DENARIUS_SIGN: char = '𐆖';
    /// \u{10197}: '𐆗'
    pub const ROMAN_QUINARIUS_SIGN: char = '𐆗';
    /// \u{10198}: '𐆘'
    pub const ROMAN_SESTERTIUS_SIGN: char = '𐆘';
    /// \u{10199}: '𐆙'
    pub const ROMAN_DUPONDIUS_SIGN: char = '𐆙';
    /// \u{1019a}: '𐆚'
    pub const ROMAN_AS_SIGN: char = '𐆚';
    /// \u{1019b}: '𐆛'
    pub const ROMAN_CENTURIAL_SIGN: char = '𐆛';
    /// \u{101a0}: '𐆠'
    pub const GREEK_SYMBOL_TAU_RHO: char = '𐆠';
}

/// \u{10190} → \u{101cf}\
///\
/// 𐆐 𐆑 𐆒 𐆓 𐆔 𐆕 𐆖 𐆗 𐆘 𐆙 𐆚 𐆛 𐆠
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum AncientSymbols {
    /// \u{10190}: '𐆐'
    RomanSextansSign,
    /// \u{10191}: '𐆑'
    RomanUnciaSign,
    /// \u{10192}: '𐆒'
    RomanSemunciaSign,
    /// \u{10193}: '𐆓'
    RomanSextulaSign,
    /// \u{10194}: '𐆔'
    RomanDimidiaSextulaSign,
    /// \u{10195}: '𐆕'
    RomanSiliquaSign,
    /// \u{10196}: '𐆖'
    RomanDenariusSign,
    /// \u{10197}: '𐆗'
    RomanQuinariusSign,
    /// \u{10198}: '𐆘'
    RomanSestertiusSign,
    /// \u{10199}: '𐆙'
    RomanDupondiusSign,
    /// \u{1019a}: '𐆚'
    RomanAsSign,
    /// \u{1019b}: '𐆛'
    RomanCenturialSign,
    /// \u{101a0}: '𐆠'
    GreekSymbolTauRho,
}

impl Into<char> for AncientSymbols {
    fn into(self) -> char {
        use constants::*;
        match self {
            AncientSymbols::RomanSextansSign => ROMAN_SEXTANS_SIGN,
            AncientSymbols::RomanUnciaSign => ROMAN_UNCIA_SIGN,
            AncientSymbols::RomanSemunciaSign => ROMAN_SEMUNCIA_SIGN,
            AncientSymbols::RomanSextulaSign => ROMAN_SEXTULA_SIGN,
            AncientSymbols::RomanDimidiaSextulaSign => ROMAN_DIMIDIA_SEXTULA_SIGN,
            AncientSymbols::RomanSiliquaSign => ROMAN_SILIQUA_SIGN,
            AncientSymbols::RomanDenariusSign => ROMAN_DENARIUS_SIGN,
            AncientSymbols::RomanQuinariusSign => ROMAN_QUINARIUS_SIGN,
            AncientSymbols::RomanSestertiusSign => ROMAN_SESTERTIUS_SIGN,
            AncientSymbols::RomanDupondiusSign => ROMAN_DUPONDIUS_SIGN,
            AncientSymbols::RomanAsSign => ROMAN_AS_SIGN,
            AncientSymbols::RomanCenturialSign => ROMAN_CENTURIAL_SIGN,
            AncientSymbols::GreekSymbolTauRho => GREEK_SYMBOL_TAU_RHO,
        }
    }
}

impl std::convert::TryFrom<char> for AncientSymbols {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            ROMAN_SEXTANS_SIGN => Ok(AncientSymbols::RomanSextansSign),
            ROMAN_UNCIA_SIGN => Ok(AncientSymbols::RomanUnciaSign),
            ROMAN_SEMUNCIA_SIGN => Ok(AncientSymbols::RomanSemunciaSign),
            ROMAN_SEXTULA_SIGN => Ok(AncientSymbols::RomanSextulaSign),
            ROMAN_DIMIDIA_SEXTULA_SIGN => Ok(AncientSymbols::RomanDimidiaSextulaSign),
            ROMAN_SILIQUA_SIGN => Ok(AncientSymbols::RomanSiliquaSign),
            ROMAN_DENARIUS_SIGN => Ok(AncientSymbols::RomanDenariusSign),
            ROMAN_QUINARIUS_SIGN => Ok(AncientSymbols::RomanQuinariusSign),
            ROMAN_SESTERTIUS_SIGN => Ok(AncientSymbols::RomanSestertiusSign),
            ROMAN_DUPONDIUS_SIGN => Ok(AncientSymbols::RomanDupondiusSign),
            ROMAN_AS_SIGN => Ok(AncientSymbols::RomanAsSign),
            ROMAN_CENTURIAL_SIGN => Ok(AncientSymbols::RomanCenturialSign),
            GREEK_SYMBOL_TAU_RHO => Ok(AncientSymbols::GreekSymbolTauRho),
            _ => Err(()),
        }
    }
}

impl Into<u32> for AncientSymbols {
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

impl std::convert::TryFrom<u32> for AncientSymbols {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for AncientSymbols {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl AncientSymbols {
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        AncientSymbols::RomanSextansSign
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            AncientSymbols::RomanSextansSign => "roman sextans sign",
            AncientSymbols::RomanUnciaSign => "roman uncia sign",
            AncientSymbols::RomanSemunciaSign => "roman semuncia sign",
            AncientSymbols::RomanSextulaSign => "roman sextula sign",
            AncientSymbols::RomanDimidiaSextulaSign => "roman dimidia sextula sign",
            AncientSymbols::RomanSiliquaSign => "roman siliqua sign",
            AncientSymbols::RomanDenariusSign => "roman denarius sign",
            AncientSymbols::RomanQuinariusSign => "roman quinarius sign",
            AncientSymbols::RomanSestertiusSign => "roman sestertius sign",
            AncientSymbols::RomanDupondiusSign => "roman dupondius sign",
            AncientSymbols::RomanAsSign => "roman as sign",
            AncientSymbols::RomanCenturialSign => "roman centurial sign",
            AncientSymbols::GreekSymbolTauRho => "greek symbol tau rho",
        }
    }
}
