
/// An enum to represent all characters in the Elymaic block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Elymaic {
    /// \u{10fe0}: '𐿠'
    LetterAleph,
    /// \u{10fe1}: '𐿡'
    LetterBeth,
    /// \u{10fe2}: '𐿢'
    LetterGimel,
    /// \u{10fe3}: '𐿣'
    LetterDaleth,
    /// \u{10fe4}: '𐿤'
    LetterHe,
    /// \u{10fe5}: '𐿥'
    LetterWaw,
    /// \u{10fe6}: '𐿦'
    LetterZayin,
    /// \u{10fe7}: '𐿧'
    LetterHeth,
    /// \u{10fe8}: '𐿨'
    LetterTeth,
    /// \u{10fe9}: '𐿩'
    LetterYodh,
    /// \u{10fea}: '𐿪'
    LetterKaph,
    /// \u{10feb}: '𐿫'
    LetterLamedh,
    /// \u{10fec}: '𐿬'
    LetterMem,
    /// \u{10fed}: '𐿭'
    LetterNun,
    /// \u{10fee}: '𐿮'
    LetterSamekh,
    /// \u{10fef}: '𐿯'
    LetterAyin,
    /// \u{10ff0}: '𐿰'
    LetterPe,
    /// \u{10ff1}: '𐿱'
    LetterSadhe,
    /// \u{10ff2}: '𐿲'
    LetterQoph,
    /// \u{10ff3}: '𐿳'
    LetterResh,
    /// \u{10ff4}: '𐿴'
    LetterShin,
    /// \u{10ff5}: '𐿵'
    LetterTaw,
    /// \u{10ff6}: '𐿶'
    LigatureZayinDashYodh,
}

impl Into<char> for Elymaic {
    fn into(self) -> char {
        match self {
            Elymaic::LetterAleph => '𐿠',
            Elymaic::LetterBeth => '𐿡',
            Elymaic::LetterGimel => '𐿢',
            Elymaic::LetterDaleth => '𐿣',
            Elymaic::LetterHe => '𐿤',
            Elymaic::LetterWaw => '𐿥',
            Elymaic::LetterZayin => '𐿦',
            Elymaic::LetterHeth => '𐿧',
            Elymaic::LetterTeth => '𐿨',
            Elymaic::LetterYodh => '𐿩',
            Elymaic::LetterKaph => '𐿪',
            Elymaic::LetterLamedh => '𐿫',
            Elymaic::LetterMem => '𐿬',
            Elymaic::LetterNun => '𐿭',
            Elymaic::LetterSamekh => '𐿮',
            Elymaic::LetterAyin => '𐿯',
            Elymaic::LetterPe => '𐿰',
            Elymaic::LetterSadhe => '𐿱',
            Elymaic::LetterQoph => '𐿲',
            Elymaic::LetterResh => '𐿳',
            Elymaic::LetterShin => '𐿴',
            Elymaic::LetterTaw => '𐿵',
            Elymaic::LigatureZayinDashYodh => '𐿶',
        }
    }
}

impl std::convert::TryFrom<char> for Elymaic {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𐿠' => Ok(Elymaic::LetterAleph),
            '𐿡' => Ok(Elymaic::LetterBeth),
            '𐿢' => Ok(Elymaic::LetterGimel),
            '𐿣' => Ok(Elymaic::LetterDaleth),
            '𐿤' => Ok(Elymaic::LetterHe),
            '𐿥' => Ok(Elymaic::LetterWaw),
            '𐿦' => Ok(Elymaic::LetterZayin),
            '𐿧' => Ok(Elymaic::LetterHeth),
            '𐿨' => Ok(Elymaic::LetterTeth),
            '𐿩' => Ok(Elymaic::LetterYodh),
            '𐿪' => Ok(Elymaic::LetterKaph),
            '𐿫' => Ok(Elymaic::LetterLamedh),
            '𐿬' => Ok(Elymaic::LetterMem),
            '𐿭' => Ok(Elymaic::LetterNun),
            '𐿮' => Ok(Elymaic::LetterSamekh),
            '𐿯' => Ok(Elymaic::LetterAyin),
            '𐿰' => Ok(Elymaic::LetterPe),
            '𐿱' => Ok(Elymaic::LetterSadhe),
            '𐿲' => Ok(Elymaic::LetterQoph),
            '𐿳' => Ok(Elymaic::LetterResh),
            '𐿴' => Ok(Elymaic::LetterShin),
            '𐿵' => Ok(Elymaic::LetterTaw),
            '𐿶' => Ok(Elymaic::LigatureZayinDashYodh),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Elymaic {
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

impl std::convert::TryFrom<u32> for Elymaic {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Elymaic {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Elymaic {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Elymaic::LetterAleph
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Elymaic{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
