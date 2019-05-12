/// \u{11660} → \u{1167f}
///
/// 𑙠 𑙡 𑙢 𑙣 𑙤 𑙥 𑙦 𑙧 𑙨 𑙩 𑙪 𑙫 𑙬\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{11660}: '𑙠'
    pub const MONGOLIAN_BIRGA_WITH_ORNAMENT: char = '𑙠';
    /// \u{11661}: '𑙡'
    pub const MONGOLIAN_ROTATED_BIRGA: char = '𑙡';
    /// \u{11662}: '𑙢'
    pub const MONGOLIAN_DOUBLE_BIRGA_WITH_ORNAMENT: char = '𑙢';
    /// \u{11663}: '𑙣'
    pub const MONGOLIAN_TRIPLE_BIRGA_WITH_ORNAMENT: char = '𑙣';
    /// \u{11664}: '𑙤'
    pub const MONGOLIAN_BIRGA_WITH_DOUBLE_ORNAMENT: char = '𑙤';
    /// \u{11665}: '𑙥'
    pub const MONGOLIAN_ROTATED_BIRGA_WITH_ORNAMENT: char = '𑙥';
    /// \u{11666}: '𑙦'
    pub const MONGOLIAN_ROTATED_BIRGA_WITH_DOUBLE_ORNAMENT: char = '𑙦';
    /// \u{11667}: '𑙧'
    pub const MONGOLIAN_INVERTED_BIRGA: char = '𑙧';
    /// \u{11668}: '𑙨'
    pub const MONGOLIAN_INVERTED_BIRGA_WITH_DOUBLE_ORNAMENT: char = '𑙨';
    /// \u{11669}: '𑙩'
    pub const MONGOLIAN_SWIRL_BIRGA: char = '𑙩';
    /// \u{1166a}: '𑙪'
    pub const MONGOLIAN_SWIRL_BIRGA_WITH_ORNAMENT: char = '𑙪';
    /// \u{1166b}: '𑙫'
    pub const MONGOLIAN_SWIRL_BIRGA_WITH_DOUBLE_ORNAMENT: char = '𑙫';
    /// \u{1166c}: '𑙬'
    pub const MONGOLIAN_TURNED_SWIRL_BIRGA_WITH_DOUBLE_ORNAMENT: char = '𑙬';
}

/// An enum to represent all characters in the MongolianSupplement block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum MongolianSupplement {
    /// \u{11660}: '𑙠'
    MongolianBirgaWithOrnament,
    /// \u{11661}: '𑙡'
    MongolianRotatedBirga,
    /// \u{11662}: '𑙢'
    MongolianDoubleBirgaWithOrnament,
    /// \u{11663}: '𑙣'
    MongolianTripleBirgaWithOrnament,
    /// \u{11664}: '𑙤'
    MongolianBirgaWithDoubleOrnament,
    /// \u{11665}: '𑙥'
    MongolianRotatedBirgaWithOrnament,
    /// \u{11666}: '𑙦'
    MongolianRotatedBirgaWithDoubleOrnament,
    /// \u{11667}: '𑙧'
    MongolianInvertedBirga,
    /// \u{11668}: '𑙨'
    MongolianInvertedBirgaWithDoubleOrnament,
    /// \u{11669}: '𑙩'
    MongolianSwirlBirga,
    /// \u{1166a}: '𑙪'
    MongolianSwirlBirgaWithOrnament,
    /// \u{1166b}: '𑙫'
    MongolianSwirlBirgaWithDoubleOrnament,
    /// \u{1166c}: '𑙬'
    MongolianTurnedSwirlBirgaWithDoubleOrnament,
}

impl Into<char> for MongolianSupplement {
    fn into(self) -> char {
        use constants::*;
        match self {
            MongolianSupplement::MongolianBirgaWithOrnament => MONGOLIAN_BIRGA_WITH_ORNAMENT,
            MongolianSupplement::MongolianRotatedBirga => MONGOLIAN_ROTATED_BIRGA,
            MongolianSupplement::MongolianDoubleBirgaWithOrnament => MONGOLIAN_DOUBLE_BIRGA_WITH_ORNAMENT,
            MongolianSupplement::MongolianTripleBirgaWithOrnament => MONGOLIAN_TRIPLE_BIRGA_WITH_ORNAMENT,
            MongolianSupplement::MongolianBirgaWithDoubleOrnament => MONGOLIAN_BIRGA_WITH_DOUBLE_ORNAMENT,
            MongolianSupplement::MongolianRotatedBirgaWithOrnament => MONGOLIAN_ROTATED_BIRGA_WITH_ORNAMENT,
            MongolianSupplement::MongolianRotatedBirgaWithDoubleOrnament => MONGOLIAN_ROTATED_BIRGA_WITH_DOUBLE_ORNAMENT,
            MongolianSupplement::MongolianInvertedBirga => MONGOLIAN_INVERTED_BIRGA,
            MongolianSupplement::MongolianInvertedBirgaWithDoubleOrnament => MONGOLIAN_INVERTED_BIRGA_WITH_DOUBLE_ORNAMENT,
            MongolianSupplement::MongolianSwirlBirga => MONGOLIAN_SWIRL_BIRGA,
            MongolianSupplement::MongolianSwirlBirgaWithOrnament => MONGOLIAN_SWIRL_BIRGA_WITH_ORNAMENT,
            MongolianSupplement::MongolianSwirlBirgaWithDoubleOrnament => MONGOLIAN_SWIRL_BIRGA_WITH_DOUBLE_ORNAMENT,
            MongolianSupplement::MongolianTurnedSwirlBirgaWithDoubleOrnament => MONGOLIAN_TURNED_SWIRL_BIRGA_WITH_DOUBLE_ORNAMENT,
        }
    }
}

impl std::convert::TryFrom<char> for MongolianSupplement {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            MONGOLIAN_BIRGA_WITH_ORNAMENT => Ok(MongolianSupplement::MongolianBirgaWithOrnament),
            MONGOLIAN_ROTATED_BIRGA => Ok(MongolianSupplement::MongolianRotatedBirga),
            MONGOLIAN_DOUBLE_BIRGA_WITH_ORNAMENT => Ok(MongolianSupplement::MongolianDoubleBirgaWithOrnament),
            MONGOLIAN_TRIPLE_BIRGA_WITH_ORNAMENT => Ok(MongolianSupplement::MongolianTripleBirgaWithOrnament),
            MONGOLIAN_BIRGA_WITH_DOUBLE_ORNAMENT => Ok(MongolianSupplement::MongolianBirgaWithDoubleOrnament),
            MONGOLIAN_ROTATED_BIRGA_WITH_ORNAMENT => Ok(MongolianSupplement::MongolianRotatedBirgaWithOrnament),
            MONGOLIAN_ROTATED_BIRGA_WITH_DOUBLE_ORNAMENT => Ok(MongolianSupplement::MongolianRotatedBirgaWithDoubleOrnament),
            MONGOLIAN_INVERTED_BIRGA => Ok(MongolianSupplement::MongolianInvertedBirga),
            MONGOLIAN_INVERTED_BIRGA_WITH_DOUBLE_ORNAMENT => Ok(MongolianSupplement::MongolianInvertedBirgaWithDoubleOrnament),
            MONGOLIAN_SWIRL_BIRGA => Ok(MongolianSupplement::MongolianSwirlBirga),
            MONGOLIAN_SWIRL_BIRGA_WITH_ORNAMENT => Ok(MongolianSupplement::MongolianSwirlBirgaWithOrnament),
            MONGOLIAN_SWIRL_BIRGA_WITH_DOUBLE_ORNAMENT => Ok(MongolianSupplement::MongolianSwirlBirgaWithDoubleOrnament),
            MONGOLIAN_TURNED_SWIRL_BIRGA_WITH_DOUBLE_ORNAMENT => Ok(MongolianSupplement::MongolianTurnedSwirlBirgaWithDoubleOrnament),
            _ => Err(()),
        }
    }
}

impl Into<u32> for MongolianSupplement {
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

impl std::convert::TryFrom<u32> for MongolianSupplement {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for MongolianSupplement {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl MongolianSupplement {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        MongolianSupplement::MongolianBirgaWithOrnament
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            MongolianSupplement::MongolianBirgaWithOrnament => "mongolian birga with ornament",
            MongolianSupplement::MongolianRotatedBirga => "mongolian rotated birga",
            MongolianSupplement::MongolianDoubleBirgaWithOrnament => "mongolian double birga with ornament",
            MongolianSupplement::MongolianTripleBirgaWithOrnament => "mongolian triple birga with ornament",
            MongolianSupplement::MongolianBirgaWithDoubleOrnament => "mongolian birga with double ornament",
            MongolianSupplement::MongolianRotatedBirgaWithOrnament => "mongolian rotated birga with ornament",
            MongolianSupplement::MongolianRotatedBirgaWithDoubleOrnament => "mongolian rotated birga with double ornament",
            MongolianSupplement::MongolianInvertedBirga => "mongolian inverted birga",
            MongolianSupplement::MongolianInvertedBirgaWithDoubleOrnament => "mongolian inverted birga with double ornament",
            MongolianSupplement::MongolianSwirlBirga => "mongolian swirl birga",
            MongolianSupplement::MongolianSwirlBirgaWithOrnament => "mongolian swirl birga with ornament",
            MongolianSupplement::MongolianSwirlBirgaWithDoubleOrnament => "mongolian swirl birga with double ornament",
            MongolianSupplement::MongolianTurnedSwirlBirgaWithDoubleOrnament => "mongolian turned swirl birga with double ornament",
        }
    }
}
