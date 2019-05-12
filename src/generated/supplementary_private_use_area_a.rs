
/// An enum to represent all characters in the SupplementaryPrivateUseAreaA block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SupplementaryPrivateUseAreaA {
    /// \u{f0000}: '󰀀'
    Plane15PrivateUseFirst,
    /// \u{ffffd}: '󿿽'
    Plane15PrivateUseLast,
}

impl Into<char> for SupplementaryPrivateUseAreaA {
    fn into(self) -> char {
        match self {
            SupplementaryPrivateUseAreaA::Plane15PrivateUseFirst => '󰀀',
            SupplementaryPrivateUseAreaA::Plane15PrivateUseLast => '󿿽',
        }
    }
}

impl std::convert::TryFrom<char> for SupplementaryPrivateUseAreaA {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '󰀀' => Ok(SupplementaryPrivateUseAreaA::Plane15PrivateUseFirst),
            '󿿽' => Ok(SupplementaryPrivateUseAreaA::Plane15PrivateUseLast),
            _ => Err(()),
        }
    }
}

impl Into<u32> for SupplementaryPrivateUseAreaA {
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

impl std::convert::TryFrom<u32> for SupplementaryPrivateUseAreaA {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for SupplementaryPrivateUseAreaA {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl SupplementaryPrivateUseAreaA {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        SupplementaryPrivateUseAreaA::Plane15PrivateUseFirst
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            SupplementaryPrivateUseAreaA::Plane15PrivateUseFirst => "plane 15 private use first",
            SupplementaryPrivateUseAreaA::Plane15PrivateUseLast => "plane 15 private use last",
        }
    }
}
