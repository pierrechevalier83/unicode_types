/// \u{f0000} → \u{fffff}
///
/// 󰀀 󿿽\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{f0000}: '󰀀'
    pub const PLANE_15_PRIVATE_USE_FIRST: char = '󰀀';
    /// \u{ffffd}: '󿿽'
    pub const PLANE_15_PRIVATE_USE_LAST: char = '󿿽';
}

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
        use constants::*;
        match self {
            SupplementaryPrivateUseAreaA::Plane15PrivateUseFirst => PLANE_15_PRIVATE_USE_FIRST,
            SupplementaryPrivateUseAreaA::Plane15PrivateUseLast => PLANE_15_PRIVATE_USE_LAST,
        }
    }
}

impl std::convert::TryFrom<char> for SupplementaryPrivateUseAreaA {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            PLANE_15_PRIVATE_USE_FIRST => Ok(SupplementaryPrivateUseAreaA::Plane15PrivateUseFirst),
            PLANE_15_PRIVATE_USE_LAST => Ok(SupplementaryPrivateUseAreaA::Plane15PrivateUseLast),
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
