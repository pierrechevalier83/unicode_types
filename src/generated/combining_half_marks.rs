
/// An enum to represent all characters in the CombiningHalfMarks block.
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
        match self {
            CombiningHalfMarks::CombiningLigatureLeftHalf => '︠',
            CombiningHalfMarks::CombiningLigatureRightHalf => '︡',
            CombiningHalfMarks::CombiningDoubleTildeLeftHalf => '︢',
            CombiningHalfMarks::CombiningDoubleTildeRightHalf => '︣',
            CombiningHalfMarks::CombiningMacronLeftHalf => '︤',
            CombiningHalfMarks::CombiningMacronRightHalf => '︥',
            CombiningHalfMarks::CombiningConjoiningMacron => '︦',
            CombiningHalfMarks::CombiningLigatureLeftHalfBelow => '︧',
            CombiningHalfMarks::CombiningLigatureRightHalfBelow => '︨',
            CombiningHalfMarks::CombiningTildeLeftHalfBelow => '︩',
            CombiningHalfMarks::CombiningTildeRightHalfBelow => '︪',
            CombiningHalfMarks::CombiningMacronLeftHalfBelow => '︫',
            CombiningHalfMarks::CombiningMacronRightHalfBelow => '︬',
            CombiningHalfMarks::CombiningConjoiningMacronBelow => '︭',
            CombiningHalfMarks::CombiningCyrillicTitloLeftHalf => '︮',
        }
    }
}

impl std::convert::TryFrom<char> for CombiningHalfMarks {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '︠' => Ok(CombiningHalfMarks::CombiningLigatureLeftHalf),
            '︡' => Ok(CombiningHalfMarks::CombiningLigatureRightHalf),
            '︢' => Ok(CombiningHalfMarks::CombiningDoubleTildeLeftHalf),
            '︣' => Ok(CombiningHalfMarks::CombiningDoubleTildeRightHalf),
            '︤' => Ok(CombiningHalfMarks::CombiningMacronLeftHalf),
            '︥' => Ok(CombiningHalfMarks::CombiningMacronRightHalf),
            '︦' => Ok(CombiningHalfMarks::CombiningConjoiningMacron),
            '︧' => Ok(CombiningHalfMarks::CombiningLigatureLeftHalfBelow),
            '︨' => Ok(CombiningHalfMarks::CombiningLigatureRightHalfBelow),
            '︩' => Ok(CombiningHalfMarks::CombiningTildeLeftHalfBelow),
            '︪' => Ok(CombiningHalfMarks::CombiningTildeRightHalfBelow),
            '︫' => Ok(CombiningHalfMarks::CombiningMacronLeftHalfBelow),
            '︬' => Ok(CombiningHalfMarks::CombiningMacronRightHalfBelow),
            '︭' => Ok(CombiningHalfMarks::CombiningConjoiningMacronBelow),
            '︮' => Ok(CombiningHalfMarks::CombiningCyrillicTitloLeftHalf),
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
    /// The character with the lowest index in this unicode block
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
