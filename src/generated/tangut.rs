/// \u{17000} → \u{187ff}\
///\
/// 𗀀 𘟷
pub mod constants {
    /// \u{17000}: '𗀀'
    pub const TANGUT_IDEOGRAPH_FIRST: char = '𗀀';
    /// \u{187f7}: '𘟷'
    pub const TANGUT_IDEOGRAPH_LAST: char = '𘟷';
}

/// \u{17000} → \u{187ff}\
///\
/// 𗀀 𘟷
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Tangut {
    /// \u{17000}: '𗀀'
    TangutIdeographFirst,
    /// \u{187f7}: '𘟷'
    TangutIdeographLast,
}

impl Into<char> for Tangut {
    fn into(self) -> char {
        use constants::*;
        match self {
            Tangut::TangutIdeographFirst => TANGUT_IDEOGRAPH_FIRST,
            Tangut::TangutIdeographLast => TANGUT_IDEOGRAPH_LAST,
        }
    }
}

impl std::convert::TryFrom<char> for Tangut {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            TANGUT_IDEOGRAPH_FIRST => Ok(Tangut::TangutIdeographFirst),
            TANGUT_IDEOGRAPH_LAST => Ok(Tangut::TangutIdeographLast),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Tangut {
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

impl std::convert::TryFrom<u32> for Tangut {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Tangut {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Tangut {
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        Tangut::TangutIdeographFirst
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Tangut::TangutIdeographFirst => "tangut ideograph first",
            Tangut::TangutIdeographLast => "tangut ideograph last",
        }
    }
}
