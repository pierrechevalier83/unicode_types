
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
        match self {
            MongolianSupplement::MongolianBirgaWithOrnament => '𑙠',
            MongolianSupplement::MongolianRotatedBirga => '𑙡',
            MongolianSupplement::MongolianDoubleBirgaWithOrnament => '𑙢',
            MongolianSupplement::MongolianTripleBirgaWithOrnament => '𑙣',
            MongolianSupplement::MongolianBirgaWithDoubleOrnament => '𑙤',
            MongolianSupplement::MongolianRotatedBirgaWithOrnament => '𑙥',
            MongolianSupplement::MongolianRotatedBirgaWithDoubleOrnament => '𑙦',
            MongolianSupplement::MongolianInvertedBirga => '𑙧',
            MongolianSupplement::MongolianInvertedBirgaWithDoubleOrnament => '𑙨',
            MongolianSupplement::MongolianSwirlBirga => '𑙩',
            MongolianSupplement::MongolianSwirlBirgaWithOrnament => '𑙪',
            MongolianSupplement::MongolianSwirlBirgaWithDoubleOrnament => '𑙫',
            MongolianSupplement::MongolianTurnedSwirlBirgaWithDoubleOrnament => '𑙬',
        }
    }
}

impl std::convert::TryFrom<char> for MongolianSupplement {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𑙠' => Ok(MongolianSupplement::MongolianBirgaWithOrnament),
            '𑙡' => Ok(MongolianSupplement::MongolianRotatedBirga),
            '𑙢' => Ok(MongolianSupplement::MongolianDoubleBirgaWithOrnament),
            '𑙣' => Ok(MongolianSupplement::MongolianTripleBirgaWithOrnament),
            '𑙤' => Ok(MongolianSupplement::MongolianBirgaWithDoubleOrnament),
            '𑙥' => Ok(MongolianSupplement::MongolianRotatedBirgaWithOrnament),
            '𑙦' => Ok(MongolianSupplement::MongolianRotatedBirgaWithDoubleOrnament),
            '𑙧' => Ok(MongolianSupplement::MongolianInvertedBirga),
            '𑙨' => Ok(MongolianSupplement::MongolianInvertedBirgaWithDoubleOrnament),
            '𑙩' => Ok(MongolianSupplement::MongolianSwirlBirga),
            '𑙪' => Ok(MongolianSupplement::MongolianSwirlBirgaWithOrnament),
            '𑙫' => Ok(MongolianSupplement::MongolianSwirlBirgaWithDoubleOrnament),
            '𑙬' => Ok(MongolianSupplement::MongolianTurnedSwirlBirgaWithDoubleOrnament),
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

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("MongolianSupplement{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
