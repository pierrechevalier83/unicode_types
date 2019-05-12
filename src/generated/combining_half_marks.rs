/// \u{fe20} → \u{fe2f}\
///\
/// ︠ ︡ ︢ ︣ ︤ ︥ ︦ ︧ ︨ ︩ ︪ ︫ ︬ ︭ ︮
pub mod constants {
    /// \u{fe20}: '︠'
    pub const COMBINING_LIGATURE_LEFT_HALF: char = '︠';
    /// \u{fe21}: '︡'
    pub const COMBINING_LIGATURE_RIGHT_HALF: char = '︡';
    /// \u{fe22}: '︢'
    pub const COMBINING_DOUBLE_TILDE_LEFT_HALF: char = '︢';
    /// \u{fe23}: '︣'
    pub const COMBINING_DOUBLE_TILDE_RIGHT_HALF: char = '︣';
    /// \u{fe24}: '︤'
    pub const COMBINING_MACRON_LEFT_HALF: char = '︤';
    /// \u{fe25}: '︥'
    pub const COMBINING_MACRON_RIGHT_HALF: char = '︥';
    /// \u{fe26}: '︦'
    pub const COMBINING_CONJOINING_MACRON: char = '︦';
    /// \u{fe27}: '︧'
    pub const COMBINING_LIGATURE_LEFT_HALF_BELOW: char = '︧';
    /// \u{fe28}: '︨'
    pub const COMBINING_LIGATURE_RIGHT_HALF_BELOW: char = '︨';
    /// \u{fe29}: '︩'
    pub const COMBINING_TILDE_LEFT_HALF_BELOW: char = '︩';
    /// \u{fe2a}: '︪'
    pub const COMBINING_TILDE_RIGHT_HALF_BELOW: char = '︪';
    /// \u{fe2b}: '︫'
    pub const COMBINING_MACRON_LEFT_HALF_BELOW: char = '︫';
    /// \u{fe2c}: '︬'
    pub const COMBINING_MACRON_RIGHT_HALF_BELOW: char = '︬';
    /// \u{fe2d}: '︭'
    pub const COMBINING_CONJOINING_MACRON_BELOW: char = '︭';
    /// \u{fe2e}: '︮'
    pub const COMBINING_CYRILLIC_TITLO_LEFT_HALF: char = '︮';
}

/// \u{fe20} → \u{fe2f}\
///\
/// ︠ ︡ ︢ ︣ ︤ ︥ ︦ ︧ ︨ ︩ ︪ ︫ ︬ ︭ ︮
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum CombiningHalfMarks {
    /// \u{fe20}: '︠'
    CombiningLigatureLeftHalf,
    /// \u{fe21}: '︡'
    CombiningLigatureRightHalf,
    /// \u{fe22}: '︢'
    CombiningDoubleTildeLeftHalf,
    /// \u{fe23}: '︣'
    CombiningDoubleTildeRightHalf,
    /// \u{fe24}: '︤'
    CombiningMacronLeftHalf,
    /// \u{fe25}: '︥'
    CombiningMacronRightHalf,
    /// \u{fe26}: '︦'
    CombiningConjoiningMacron,
    /// \u{fe27}: '︧'
    CombiningLigatureLeftHalfBelow,
    /// \u{fe28}: '︨'
    CombiningLigatureRightHalfBelow,
    /// \u{fe29}: '︩'
    CombiningTildeLeftHalfBelow,
    /// \u{fe2a}: '︪'
    CombiningTildeRightHalfBelow,
    /// \u{fe2b}: '︫'
    CombiningMacronLeftHalfBelow,
    /// \u{fe2c}: '︬'
    CombiningMacronRightHalfBelow,
    /// \u{fe2d}: '︭'
    CombiningConjoiningMacronBelow,
    /// \u{fe2e}: '︮'
    CombiningCyrillicTitloLeftHalf,
}

impl Into<char> for CombiningHalfMarks {
    fn into(self) -> char {
        use constants::*;
        match self {
            CombiningHalfMarks::CombiningLigatureLeftHalf => COMBINING_LIGATURE_LEFT_HALF,
            CombiningHalfMarks::CombiningLigatureRightHalf => COMBINING_LIGATURE_RIGHT_HALF,
            CombiningHalfMarks::CombiningDoubleTildeLeftHalf => COMBINING_DOUBLE_TILDE_LEFT_HALF,
            CombiningHalfMarks::CombiningDoubleTildeRightHalf => COMBINING_DOUBLE_TILDE_RIGHT_HALF,
            CombiningHalfMarks::CombiningMacronLeftHalf => COMBINING_MACRON_LEFT_HALF,
            CombiningHalfMarks::CombiningMacronRightHalf => COMBINING_MACRON_RIGHT_HALF,
            CombiningHalfMarks::CombiningConjoiningMacron => COMBINING_CONJOINING_MACRON,
            CombiningHalfMarks::CombiningLigatureLeftHalfBelow => COMBINING_LIGATURE_LEFT_HALF_BELOW,
            CombiningHalfMarks::CombiningLigatureRightHalfBelow => COMBINING_LIGATURE_RIGHT_HALF_BELOW,
            CombiningHalfMarks::CombiningTildeLeftHalfBelow => COMBINING_TILDE_LEFT_HALF_BELOW,
            CombiningHalfMarks::CombiningTildeRightHalfBelow => COMBINING_TILDE_RIGHT_HALF_BELOW,
            CombiningHalfMarks::CombiningMacronLeftHalfBelow => COMBINING_MACRON_LEFT_HALF_BELOW,
            CombiningHalfMarks::CombiningMacronRightHalfBelow => COMBINING_MACRON_RIGHT_HALF_BELOW,
            CombiningHalfMarks::CombiningConjoiningMacronBelow => COMBINING_CONJOINING_MACRON_BELOW,
            CombiningHalfMarks::CombiningCyrillicTitloLeftHalf => COMBINING_CYRILLIC_TITLO_LEFT_HALF,
        }
    }
}

impl std::convert::TryFrom<char> for CombiningHalfMarks {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            COMBINING_LIGATURE_LEFT_HALF => Ok(CombiningHalfMarks::CombiningLigatureLeftHalf),
            COMBINING_LIGATURE_RIGHT_HALF => Ok(CombiningHalfMarks::CombiningLigatureRightHalf),
            COMBINING_DOUBLE_TILDE_LEFT_HALF => Ok(CombiningHalfMarks::CombiningDoubleTildeLeftHalf),
            COMBINING_DOUBLE_TILDE_RIGHT_HALF => Ok(CombiningHalfMarks::CombiningDoubleTildeRightHalf),
            COMBINING_MACRON_LEFT_HALF => Ok(CombiningHalfMarks::CombiningMacronLeftHalf),
            COMBINING_MACRON_RIGHT_HALF => Ok(CombiningHalfMarks::CombiningMacronRightHalf),
            COMBINING_CONJOINING_MACRON => Ok(CombiningHalfMarks::CombiningConjoiningMacron),
            COMBINING_LIGATURE_LEFT_HALF_BELOW => Ok(CombiningHalfMarks::CombiningLigatureLeftHalfBelow),
            COMBINING_LIGATURE_RIGHT_HALF_BELOW => Ok(CombiningHalfMarks::CombiningLigatureRightHalfBelow),
            COMBINING_TILDE_LEFT_HALF_BELOW => Ok(CombiningHalfMarks::CombiningTildeLeftHalfBelow),
            COMBINING_TILDE_RIGHT_HALF_BELOW => Ok(CombiningHalfMarks::CombiningTildeRightHalfBelow),
            COMBINING_MACRON_LEFT_HALF_BELOW => Ok(CombiningHalfMarks::CombiningMacronLeftHalfBelow),
            COMBINING_MACRON_RIGHT_HALF_BELOW => Ok(CombiningHalfMarks::CombiningMacronRightHalfBelow),
            COMBINING_CONJOINING_MACRON_BELOW => Ok(CombiningHalfMarks::CombiningConjoiningMacronBelow),
            COMBINING_CYRILLIC_TITLO_LEFT_HALF => Ok(CombiningHalfMarks::CombiningCyrillicTitloLeftHalf),
            _ => Err(()),
        }
    }
}

impl Into<u32> for CombiningHalfMarks {
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

impl std::convert::TryFrom<u32> for CombiningHalfMarks {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for CombiningHalfMarks {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl CombiningHalfMarks {
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        CombiningHalfMarks::CombiningLigatureLeftHalf
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            CombiningHalfMarks::CombiningLigatureLeftHalf => "combining ligature left half",
            CombiningHalfMarks::CombiningLigatureRightHalf => "combining ligature right half",
            CombiningHalfMarks::CombiningDoubleTildeLeftHalf => "combining double tilde left half",
            CombiningHalfMarks::CombiningDoubleTildeRightHalf => "combining double tilde right half",
            CombiningHalfMarks::CombiningMacronLeftHalf => "combining macron left half",
            CombiningHalfMarks::CombiningMacronRightHalf => "combining macron right half",
            CombiningHalfMarks::CombiningConjoiningMacron => "combining conjoining macron",
            CombiningHalfMarks::CombiningLigatureLeftHalfBelow => "combining ligature left half below",
            CombiningHalfMarks::CombiningLigatureRightHalfBelow => "combining ligature right half below",
            CombiningHalfMarks::CombiningTildeLeftHalfBelow => "combining tilde left half below",
            CombiningHalfMarks::CombiningTildeRightHalfBelow => "combining tilde right half below",
            CombiningHalfMarks::CombiningMacronLeftHalfBelow => "combining macron left half below",
            CombiningHalfMarks::CombiningMacronRightHalfBelow => "combining macron right half below",
            CombiningHalfMarks::CombiningConjoiningMacronBelow => "combining conjoining macron below",
            CombiningHalfMarks::CombiningCyrillicTitloLeftHalf => "combining cyrillic titlo left half",
        }
    }
}
