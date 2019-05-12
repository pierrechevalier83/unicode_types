
/// An enum to represent all characters in the SupplementaryPrivateUseAreaB block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SupplementaryPrivateUseAreaB {
    /// \u{100000}: '􀀀'
    Plane16PrivateUseFirst,
    /// \u{10fffd}: '􏿽'
    Plane16PrivateUseLast,
}

impl Into<char> for SupplementaryPrivateUseAreaB {
    fn into(self) -> char {
        match self {
            SupplementaryPrivateUseAreaB::Plane16PrivateUseFirst => '􀀀',
            SupplementaryPrivateUseAreaB::Plane16PrivateUseLast => '􏿽',
        }
    }
}

impl std::convert::TryFrom<char> for SupplementaryPrivateUseAreaB {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '􀀀' => Ok(SupplementaryPrivateUseAreaB::Plane16PrivateUseFirst),
            '􏿽' => Ok(SupplementaryPrivateUseAreaB::Plane16PrivateUseLast),
            _ => Err(()),
        }
    }
}

impl Into<u32> for SupplementaryPrivateUseAreaB {
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

impl std::convert::TryFrom<u32> for SupplementaryPrivateUseAreaB {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for SupplementaryPrivateUseAreaB {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl SupplementaryPrivateUseAreaB {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        SupplementaryPrivateUseAreaB::Plane16PrivateUseFirst
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            SupplementaryPrivateUseAreaB::Plane16PrivateUseFirst => "plane 16 private use first",
            SupplementaryPrivateUseAreaB::Plane16PrivateUseLast => "plane 16 private use last",
        }
    }
}
