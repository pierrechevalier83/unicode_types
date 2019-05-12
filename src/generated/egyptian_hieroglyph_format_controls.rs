/// \u{13430} → \u{1343f}\
///\
/// 𓐰 𓐱 𓐲 𓐳 𓐴 𓐵 𓐶 𓐷 𓐸
pub mod constants {
    /// \u{13430}: '𓐰'
    pub const EGYPTIAN_HIEROGLYPH_VERTICAL_JOINER: char = '𓐰';
    /// \u{13431}: '𓐱'
    pub const EGYPTIAN_HIEROGLYPH_HORIZONTAL_JOINER: char = '𓐱';
    /// \u{13432}: '𓐲'
    pub const EGYPTIAN_HIEROGLYPH_INSERT_AT_TOP_START: char = '𓐲';
    /// \u{13433}: '𓐳'
    pub const EGYPTIAN_HIEROGLYPH_INSERT_AT_BOTTOM_START: char = '𓐳';
    /// \u{13434}: '𓐴'
    pub const EGYPTIAN_HIEROGLYPH_INSERT_AT_TOP_END: char = '𓐴';
    /// \u{13435}: '𓐵'
    pub const EGYPTIAN_HIEROGLYPH_INSERT_AT_BOTTOM_END: char = '𓐵';
    /// \u{13436}: '𓐶'
    pub const EGYPTIAN_HIEROGLYPH_OVERLAY_MIDDLE: char = '𓐶';
    /// \u{13437}: '𓐷'
    pub const EGYPTIAN_HIEROGLYPH_BEGIN_SEGMENT: char = '𓐷';
    /// \u{13438}: '𓐸'
    pub const EGYPTIAN_HIEROGLYPH_END_SEGMENT: char = '𓐸';
}

/// \u{13430} → \u{1343f}\
///\
/// 𓐰 𓐱 𓐲 𓐳 𓐴 𓐵 𓐶 𓐷 𓐸
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum EgyptianHieroglyphFormatControls {
    /// \u{13430}: '𓐰'
    EgyptianHieroglyphVerticalJoiner,
    /// \u{13431}: '𓐱'
    EgyptianHieroglyphHorizontalJoiner,
    /// \u{13432}: '𓐲'
    EgyptianHieroglyphInsertAtTopStart,
    /// \u{13433}: '𓐳'
    EgyptianHieroglyphInsertAtBottomStart,
    /// \u{13434}: '𓐴'
    EgyptianHieroglyphInsertAtTopEnd,
    /// \u{13435}: '𓐵'
    EgyptianHieroglyphInsertAtBottomEnd,
    /// \u{13436}: '𓐶'
    EgyptianHieroglyphOverlayMiddle,
    /// \u{13437}: '𓐷'
    EgyptianHieroglyphBeginSegment,
    /// \u{13438}: '𓐸'
    EgyptianHieroglyphEndSegment,
}

impl Into<char> for EgyptianHieroglyphFormatControls {
    fn into(self) -> char {
        use constants::*;
        match self {
            EgyptianHieroglyphFormatControls::EgyptianHieroglyphVerticalJoiner => EGYPTIAN_HIEROGLYPH_VERTICAL_JOINER,
            EgyptianHieroglyphFormatControls::EgyptianHieroglyphHorizontalJoiner => EGYPTIAN_HIEROGLYPH_HORIZONTAL_JOINER,
            EgyptianHieroglyphFormatControls::EgyptianHieroglyphInsertAtTopStart => EGYPTIAN_HIEROGLYPH_INSERT_AT_TOP_START,
            EgyptianHieroglyphFormatControls::EgyptianHieroglyphInsertAtBottomStart => EGYPTIAN_HIEROGLYPH_INSERT_AT_BOTTOM_START,
            EgyptianHieroglyphFormatControls::EgyptianHieroglyphInsertAtTopEnd => EGYPTIAN_HIEROGLYPH_INSERT_AT_TOP_END,
            EgyptianHieroglyphFormatControls::EgyptianHieroglyphInsertAtBottomEnd => EGYPTIAN_HIEROGLYPH_INSERT_AT_BOTTOM_END,
            EgyptianHieroglyphFormatControls::EgyptianHieroglyphOverlayMiddle => EGYPTIAN_HIEROGLYPH_OVERLAY_MIDDLE,
            EgyptianHieroglyphFormatControls::EgyptianHieroglyphBeginSegment => EGYPTIAN_HIEROGLYPH_BEGIN_SEGMENT,
            EgyptianHieroglyphFormatControls::EgyptianHieroglyphEndSegment => EGYPTIAN_HIEROGLYPH_END_SEGMENT,
        }
    }
}

impl std::convert::TryFrom<char> for EgyptianHieroglyphFormatControls {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            EGYPTIAN_HIEROGLYPH_VERTICAL_JOINER => Ok(EgyptianHieroglyphFormatControls::EgyptianHieroglyphVerticalJoiner),
            EGYPTIAN_HIEROGLYPH_HORIZONTAL_JOINER => Ok(EgyptianHieroglyphFormatControls::EgyptianHieroglyphHorizontalJoiner),
            EGYPTIAN_HIEROGLYPH_INSERT_AT_TOP_START => Ok(EgyptianHieroglyphFormatControls::EgyptianHieroglyphInsertAtTopStart),
            EGYPTIAN_HIEROGLYPH_INSERT_AT_BOTTOM_START => Ok(EgyptianHieroglyphFormatControls::EgyptianHieroglyphInsertAtBottomStart),
            EGYPTIAN_HIEROGLYPH_INSERT_AT_TOP_END => Ok(EgyptianHieroglyphFormatControls::EgyptianHieroglyphInsertAtTopEnd),
            EGYPTIAN_HIEROGLYPH_INSERT_AT_BOTTOM_END => Ok(EgyptianHieroglyphFormatControls::EgyptianHieroglyphInsertAtBottomEnd),
            EGYPTIAN_HIEROGLYPH_OVERLAY_MIDDLE => Ok(EgyptianHieroglyphFormatControls::EgyptianHieroglyphOverlayMiddle),
            EGYPTIAN_HIEROGLYPH_BEGIN_SEGMENT => Ok(EgyptianHieroglyphFormatControls::EgyptianHieroglyphBeginSegment),
            EGYPTIAN_HIEROGLYPH_END_SEGMENT => Ok(EgyptianHieroglyphFormatControls::EgyptianHieroglyphEndSegment),
            _ => Err(()),
        }
    }
}

impl Into<u32> for EgyptianHieroglyphFormatControls {
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

impl std::convert::TryFrom<u32> for EgyptianHieroglyphFormatControls {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for EgyptianHieroglyphFormatControls {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl EgyptianHieroglyphFormatControls {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        EgyptianHieroglyphFormatControls::EgyptianHieroglyphVerticalJoiner
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            EgyptianHieroglyphFormatControls::EgyptianHieroglyphVerticalJoiner => "egyptian hieroglyph vertical joiner",
            EgyptianHieroglyphFormatControls::EgyptianHieroglyphHorizontalJoiner => "egyptian hieroglyph horizontal joiner",
            EgyptianHieroglyphFormatControls::EgyptianHieroglyphInsertAtTopStart => "egyptian hieroglyph insert at top start",
            EgyptianHieroglyphFormatControls::EgyptianHieroglyphInsertAtBottomStart => "egyptian hieroglyph insert at bottom start",
            EgyptianHieroglyphFormatControls::EgyptianHieroglyphInsertAtTopEnd => "egyptian hieroglyph insert at top end",
            EgyptianHieroglyphFormatControls::EgyptianHieroglyphInsertAtBottomEnd => "egyptian hieroglyph insert at bottom end",
            EgyptianHieroglyphFormatControls::EgyptianHieroglyphOverlayMiddle => "egyptian hieroglyph overlay middle",
            EgyptianHieroglyphFormatControls::EgyptianHieroglyphBeginSegment => "egyptian hieroglyph begin segment",
            EgyptianHieroglyphFormatControls::EgyptianHieroglyphEndSegment => "egyptian hieroglyph end segment",
        }
    }
}
