
/// An enum to represent all characters in the EgyptianHieroglyphFormatControls block.
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
        match self {
            EgyptianHieroglyphFormatControls::EgyptianHieroglyphVerticalJoiner => '𓐰',
            EgyptianHieroglyphFormatControls::EgyptianHieroglyphHorizontalJoiner => '𓐱',
            EgyptianHieroglyphFormatControls::EgyptianHieroglyphInsertAtTopStart => '𓐲',
            EgyptianHieroglyphFormatControls::EgyptianHieroglyphInsertAtBottomStart => '𓐳',
            EgyptianHieroglyphFormatControls::EgyptianHieroglyphInsertAtTopEnd => '𓐴',
            EgyptianHieroglyphFormatControls::EgyptianHieroglyphInsertAtBottomEnd => '𓐵',
            EgyptianHieroglyphFormatControls::EgyptianHieroglyphOverlayMiddle => '𓐶',
            EgyptianHieroglyphFormatControls::EgyptianHieroglyphBeginSegment => '𓐷',
            EgyptianHieroglyphFormatControls::EgyptianHieroglyphEndSegment => '𓐸',
        }
    }
}

impl std::convert::TryFrom<char> for EgyptianHieroglyphFormatControls {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𓐰' => Ok(EgyptianHieroglyphFormatControls::EgyptianHieroglyphVerticalJoiner),
            '𓐱' => Ok(EgyptianHieroglyphFormatControls::EgyptianHieroglyphHorizontalJoiner),
            '𓐲' => Ok(EgyptianHieroglyphFormatControls::EgyptianHieroglyphInsertAtTopStart),
            '𓐳' => Ok(EgyptianHieroglyphFormatControls::EgyptianHieroglyphInsertAtBottomStart),
            '𓐴' => Ok(EgyptianHieroglyphFormatControls::EgyptianHieroglyphInsertAtTopEnd),
            '𓐵' => Ok(EgyptianHieroglyphFormatControls::EgyptianHieroglyphInsertAtBottomEnd),
            '𓐶' => Ok(EgyptianHieroglyphFormatControls::EgyptianHieroglyphOverlayMiddle),
            '𓐷' => Ok(EgyptianHieroglyphFormatControls::EgyptianHieroglyphBeginSegment),
            '𓐸' => Ok(EgyptianHieroglyphFormatControls::EgyptianHieroglyphEndSegment),
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
