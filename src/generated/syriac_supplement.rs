
/// An enum to represent all characters in the SyriacSupplement block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SyriacSupplement {
    /// \u{860}: 'ࡠ'
    SyriacLetterMalayalamNga,
    /// \u{861}: 'ࡡ'
    SyriacLetterMalayalamJa,
    /// \u{862}: 'ࡢ'
    SyriacLetterMalayalamNya,
    /// \u{863}: 'ࡣ'
    SyriacLetterMalayalamTta,
    /// \u{864}: 'ࡤ'
    SyriacLetterMalayalamNna,
    /// \u{865}: 'ࡥ'
    SyriacLetterMalayalamNnna,
    /// \u{866}: 'ࡦ'
    SyriacLetterMalayalamBha,
    /// \u{867}: 'ࡧ'
    SyriacLetterMalayalamRa,
    /// \u{868}: 'ࡨ'
    SyriacLetterMalayalamLla,
    /// \u{869}: 'ࡩ'
    SyriacLetterMalayalamLlla,
    /// \u{86a}: 'ࡪ'
    SyriacLetterMalayalamSsa,
}

impl Into<char> for SyriacSupplement {
    fn into(self) -> char {
        match self {
            SyriacSupplement::SyriacLetterMalayalamNga => 'ࡠ',
            SyriacSupplement::SyriacLetterMalayalamJa => 'ࡡ',
            SyriacSupplement::SyriacLetterMalayalamNya => 'ࡢ',
            SyriacSupplement::SyriacLetterMalayalamTta => 'ࡣ',
            SyriacSupplement::SyriacLetterMalayalamNna => 'ࡤ',
            SyriacSupplement::SyriacLetterMalayalamNnna => 'ࡥ',
            SyriacSupplement::SyriacLetterMalayalamBha => 'ࡦ',
            SyriacSupplement::SyriacLetterMalayalamRa => 'ࡧ',
            SyriacSupplement::SyriacLetterMalayalamLla => 'ࡨ',
            SyriacSupplement::SyriacLetterMalayalamLlla => 'ࡩ',
            SyriacSupplement::SyriacLetterMalayalamSsa => 'ࡪ',
        }
    }
}

impl std::convert::TryFrom<char> for SyriacSupplement {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ࡠ' => Ok(SyriacSupplement::SyriacLetterMalayalamNga),
            'ࡡ' => Ok(SyriacSupplement::SyriacLetterMalayalamJa),
            'ࡢ' => Ok(SyriacSupplement::SyriacLetterMalayalamNya),
            'ࡣ' => Ok(SyriacSupplement::SyriacLetterMalayalamTta),
            'ࡤ' => Ok(SyriacSupplement::SyriacLetterMalayalamNna),
            'ࡥ' => Ok(SyriacSupplement::SyriacLetterMalayalamNnna),
            'ࡦ' => Ok(SyriacSupplement::SyriacLetterMalayalamBha),
            'ࡧ' => Ok(SyriacSupplement::SyriacLetterMalayalamRa),
            'ࡨ' => Ok(SyriacSupplement::SyriacLetterMalayalamLla),
            'ࡩ' => Ok(SyriacSupplement::SyriacLetterMalayalamLlla),
            'ࡪ' => Ok(SyriacSupplement::SyriacLetterMalayalamSsa),
            _ => Err(()),
        }
    }
}

impl Into<u32> for SyriacSupplement {
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

impl std::convert::TryFrom<u32> for SyriacSupplement {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for SyriacSupplement {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl SyriacSupplement {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        SyriacSupplement::SyriacLetterMalayalamNga
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            SyriacSupplement::SyriacLetterMalayalamNga => "syriac letter malayalam nga",
            SyriacSupplement::SyriacLetterMalayalamJa => "syriac letter malayalam ja",
            SyriacSupplement::SyriacLetterMalayalamNya => "syriac letter malayalam nya",
            SyriacSupplement::SyriacLetterMalayalamTta => "syriac letter malayalam tta",
            SyriacSupplement::SyriacLetterMalayalamNna => "syriac letter malayalam nna",
            SyriacSupplement::SyriacLetterMalayalamNnna => "syriac letter malayalam nnna",
            SyriacSupplement::SyriacLetterMalayalamBha => "syriac letter malayalam bha",
            SyriacSupplement::SyriacLetterMalayalamRa => "syriac letter malayalam ra",
            SyriacSupplement::SyriacLetterMalayalamLla => "syriac letter malayalam lla",
            SyriacSupplement::SyriacLetterMalayalamLlla => "syriac letter malayalam llla",
            SyriacSupplement::SyriacLetterMalayalamSsa => "syriac letter malayalam ssa",
        }
    }
}
