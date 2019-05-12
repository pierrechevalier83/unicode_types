
/// An enum to represent all characters in the OldSouthArabian block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum OldSouthArabian {
    /// \u{10a60}: '𐩠'
    LetterHe,
    /// \u{10a61}: '𐩡'
    LetterLamedh,
    /// \u{10a62}: '𐩢'
    LetterHeth,
    /// \u{10a63}: '𐩣'
    LetterMem,
    /// \u{10a64}: '𐩤'
    LetterQoph,
    /// \u{10a65}: '𐩥'
    LetterWaw,
    /// \u{10a66}: '𐩦'
    LetterShin,
    /// \u{10a67}: '𐩧'
    LetterResh,
    /// \u{10a68}: '𐩨'
    LetterBeth,
    /// \u{10a69}: '𐩩'
    LetterTaw,
    /// \u{10a6a}: '𐩪'
    LetterSat,
    /// \u{10a6b}: '𐩫'
    LetterKaph,
    /// \u{10a6c}: '𐩬'
    LetterNun,
    /// \u{10a6d}: '𐩭'
    LetterKheth,
    /// \u{10a6e}: '𐩮'
    LetterSadhe,
    /// \u{10a6f}: '𐩯'
    LetterSamekh,
    /// \u{10a70}: '𐩰'
    LetterFe,
    /// \u{10a71}: '𐩱'
    LetterAlef,
    /// \u{10a72}: '𐩲'
    LetterAyn,
    /// \u{10a73}: '𐩳'
    LetterDhadhe,
    /// \u{10a74}: '𐩴'
    LetterGimel,
    /// \u{10a75}: '𐩵'
    LetterDaleth,
    /// \u{10a76}: '𐩶'
    LetterGhayn,
    /// \u{10a77}: '𐩷'
    LetterTeth,
    /// \u{10a78}: '𐩸'
    LetterZayn,
    /// \u{10a79}: '𐩹'
    LetterDhaleth,
    /// \u{10a7a}: '𐩺'
    LetterYodh,
    /// \u{10a7b}: '𐩻'
    LetterThaw,
    /// \u{10a7c}: '𐩼'
    LetterTheth,
    /// \u{10a7d}: '𐩽'
    NumberOne,
    /// \u{10a7e}: '𐩾'
    NumberFifty,
}

impl Into<char> for OldSouthArabian {
    fn into(self) -> char {
        match self {
            OldSouthArabian::LetterHe => '𐩠',
            OldSouthArabian::LetterLamedh => '𐩡',
            OldSouthArabian::LetterHeth => '𐩢',
            OldSouthArabian::LetterMem => '𐩣',
            OldSouthArabian::LetterQoph => '𐩤',
            OldSouthArabian::LetterWaw => '𐩥',
            OldSouthArabian::LetterShin => '𐩦',
            OldSouthArabian::LetterResh => '𐩧',
            OldSouthArabian::LetterBeth => '𐩨',
            OldSouthArabian::LetterTaw => '𐩩',
            OldSouthArabian::LetterSat => '𐩪',
            OldSouthArabian::LetterKaph => '𐩫',
            OldSouthArabian::LetterNun => '𐩬',
            OldSouthArabian::LetterKheth => '𐩭',
            OldSouthArabian::LetterSadhe => '𐩮',
            OldSouthArabian::LetterSamekh => '𐩯',
            OldSouthArabian::LetterFe => '𐩰',
            OldSouthArabian::LetterAlef => '𐩱',
            OldSouthArabian::LetterAyn => '𐩲',
            OldSouthArabian::LetterDhadhe => '𐩳',
            OldSouthArabian::LetterGimel => '𐩴',
            OldSouthArabian::LetterDaleth => '𐩵',
            OldSouthArabian::LetterGhayn => '𐩶',
            OldSouthArabian::LetterTeth => '𐩷',
            OldSouthArabian::LetterZayn => '𐩸',
            OldSouthArabian::LetterDhaleth => '𐩹',
            OldSouthArabian::LetterYodh => '𐩺',
            OldSouthArabian::LetterThaw => '𐩻',
            OldSouthArabian::LetterTheth => '𐩼',
            OldSouthArabian::NumberOne => '𐩽',
            OldSouthArabian::NumberFifty => '𐩾',
        }
    }
}

impl std::convert::TryFrom<char> for OldSouthArabian {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𐩠' => Ok(OldSouthArabian::LetterHe),
            '𐩡' => Ok(OldSouthArabian::LetterLamedh),
            '𐩢' => Ok(OldSouthArabian::LetterHeth),
            '𐩣' => Ok(OldSouthArabian::LetterMem),
            '𐩤' => Ok(OldSouthArabian::LetterQoph),
            '𐩥' => Ok(OldSouthArabian::LetterWaw),
            '𐩦' => Ok(OldSouthArabian::LetterShin),
            '𐩧' => Ok(OldSouthArabian::LetterResh),
            '𐩨' => Ok(OldSouthArabian::LetterBeth),
            '𐩩' => Ok(OldSouthArabian::LetterTaw),
            '𐩪' => Ok(OldSouthArabian::LetterSat),
            '𐩫' => Ok(OldSouthArabian::LetterKaph),
            '𐩬' => Ok(OldSouthArabian::LetterNun),
            '𐩭' => Ok(OldSouthArabian::LetterKheth),
            '𐩮' => Ok(OldSouthArabian::LetterSadhe),
            '𐩯' => Ok(OldSouthArabian::LetterSamekh),
            '𐩰' => Ok(OldSouthArabian::LetterFe),
            '𐩱' => Ok(OldSouthArabian::LetterAlef),
            '𐩲' => Ok(OldSouthArabian::LetterAyn),
            '𐩳' => Ok(OldSouthArabian::LetterDhadhe),
            '𐩴' => Ok(OldSouthArabian::LetterGimel),
            '𐩵' => Ok(OldSouthArabian::LetterDaleth),
            '𐩶' => Ok(OldSouthArabian::LetterGhayn),
            '𐩷' => Ok(OldSouthArabian::LetterTeth),
            '𐩸' => Ok(OldSouthArabian::LetterZayn),
            '𐩹' => Ok(OldSouthArabian::LetterDhaleth),
            '𐩺' => Ok(OldSouthArabian::LetterYodh),
            '𐩻' => Ok(OldSouthArabian::LetterThaw),
            '𐩼' => Ok(OldSouthArabian::LetterTheth),
            '𐩽' => Ok(OldSouthArabian::NumberOne),
            '𐩾' => Ok(OldSouthArabian::NumberFifty),
            _ => Err(()),
        }
    }
}

impl Into<u32> for OldSouthArabian {
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

impl std::convert::TryFrom<u32> for OldSouthArabian {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for OldSouthArabian {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl OldSouthArabian {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        OldSouthArabian::LetterHe
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("OldSouthArabian{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
