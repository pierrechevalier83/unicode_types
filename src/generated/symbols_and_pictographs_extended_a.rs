
/// An enum to represent all characters in the SymbolsandPictographsExtendedA block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SymbolsandPictographsExtendedA {
    /// \u{1fa70}: '🩰'
    BalletShoes,
    /// \u{1fa71}: '🩱'
    OneDashPieceSwimsuit,
    /// \u{1fa72}: '🩲'
    Briefs,
    /// \u{1fa73}: '🩳'
    Shorts,
    /// \u{1fa78}: '🩸'
    DropOfBlood,
    /// \u{1fa79}: '🩹'
    AdhesiveBandage,
    /// \u{1fa7a}: '🩺'
    Stethoscope,
    /// \u{1fa80}: '🪀'
    YoDashYo,
    /// \u{1fa81}: '🪁'
    Kite,
    /// \u{1fa82}: '🪂'
    Parachute,
    /// \u{1fa90}: '🪐'
    RingedPlanet,
    /// \u{1fa91}: '🪑'
    Chair,
    /// \u{1fa92}: '🪒'
    Razor,
    /// \u{1fa93}: '🪓'
    Axe,
    /// \u{1fa94}: '🪔'
    DiyaLamp,
    /// \u{1fa95}: '🪕'
    Banjo,
}

impl Into<char> for SymbolsandPictographsExtendedA {
    fn into(self) -> char {
        match self {
            SymbolsandPictographsExtendedA::BalletShoes => '🩰',
            SymbolsandPictographsExtendedA::OneDashPieceSwimsuit => '🩱',
            SymbolsandPictographsExtendedA::Briefs => '🩲',
            SymbolsandPictographsExtendedA::Shorts => '🩳',
            SymbolsandPictographsExtendedA::DropOfBlood => '🩸',
            SymbolsandPictographsExtendedA::AdhesiveBandage => '🩹',
            SymbolsandPictographsExtendedA::Stethoscope => '🩺',
            SymbolsandPictographsExtendedA::YoDashYo => '🪀',
            SymbolsandPictographsExtendedA::Kite => '🪁',
            SymbolsandPictographsExtendedA::Parachute => '🪂',
            SymbolsandPictographsExtendedA::RingedPlanet => '🪐',
            SymbolsandPictographsExtendedA::Chair => '🪑',
            SymbolsandPictographsExtendedA::Razor => '🪒',
            SymbolsandPictographsExtendedA::Axe => '🪓',
            SymbolsandPictographsExtendedA::DiyaLamp => '🪔',
            SymbolsandPictographsExtendedA::Banjo => '🪕',
        }
    }
}

impl std::convert::TryFrom<char> for SymbolsandPictographsExtendedA {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '🩰' => Ok(SymbolsandPictographsExtendedA::BalletShoes),
            '🩱' => Ok(SymbolsandPictographsExtendedA::OneDashPieceSwimsuit),
            '🩲' => Ok(SymbolsandPictographsExtendedA::Briefs),
            '🩳' => Ok(SymbolsandPictographsExtendedA::Shorts),
            '🩸' => Ok(SymbolsandPictographsExtendedA::DropOfBlood),
            '🩹' => Ok(SymbolsandPictographsExtendedA::AdhesiveBandage),
            '🩺' => Ok(SymbolsandPictographsExtendedA::Stethoscope),
            '🪀' => Ok(SymbolsandPictographsExtendedA::YoDashYo),
            '🪁' => Ok(SymbolsandPictographsExtendedA::Kite),
            '🪂' => Ok(SymbolsandPictographsExtendedA::Parachute),
            '🪐' => Ok(SymbolsandPictographsExtendedA::RingedPlanet),
            '🪑' => Ok(SymbolsandPictographsExtendedA::Chair),
            '🪒' => Ok(SymbolsandPictographsExtendedA::Razor),
            '🪓' => Ok(SymbolsandPictographsExtendedA::Axe),
            '🪔' => Ok(SymbolsandPictographsExtendedA::DiyaLamp),
            '🪕' => Ok(SymbolsandPictographsExtendedA::Banjo),
            _ => Err(()),
        }
    }
}

impl Into<u32> for SymbolsandPictographsExtendedA {
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

impl std::convert::TryFrom<u32> for SymbolsandPictographsExtendedA {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for SymbolsandPictographsExtendedA {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl SymbolsandPictographsExtendedA {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        SymbolsandPictographsExtendedA::BalletShoes
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("SymbolsandPictographsExtendedA{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
