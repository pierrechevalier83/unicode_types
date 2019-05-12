/// \u{1fa70} → \u{1faff}\
///\
/// 🩰 🩱 🩲 🩳 🩸 🩹 🩺 🪀 🪁 🪂 🪐 🪑 🪒 🪓 🪔 🪕\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{1fa70}: '🩰'
    pub const BALLET_SHOES: char = '🩰';
    /// \u{1fa71}: '🩱'
    pub const ONE_DASH_PIECE_SWIMSUIT: char = '🩱';
    /// \u{1fa72}: '🩲'
    pub const BRIEFS: char = '🩲';
    /// \u{1fa73}: '🩳'
    pub const SHORTS: char = '🩳';
    /// \u{1fa78}: '🩸'
    pub const DROP_OF_BLOOD: char = '🩸';
    /// \u{1fa79}: '🩹'
    pub const ADHESIVE_BANDAGE: char = '🩹';
    /// \u{1fa7a}: '🩺'
    pub const STETHOSCOPE: char = '🩺';
    /// \u{1fa80}: '🪀'
    pub const YO_DASH_YO: char = '🪀';
    /// \u{1fa81}: '🪁'
    pub const KITE: char = '🪁';
    /// \u{1fa82}: '🪂'
    pub const PARACHUTE: char = '🪂';
    /// \u{1fa90}: '🪐'
    pub const RINGED_PLANET: char = '🪐';
    /// \u{1fa91}: '🪑'
    pub const CHAIR: char = '🪑';
    /// \u{1fa92}: '🪒'
    pub const RAZOR: char = '🪒';
    /// \u{1fa93}: '🪓'
    pub const AXE: char = '🪓';
    /// \u{1fa94}: '🪔'
    pub const DIYA_LAMP: char = '🪔';
    /// \u{1fa95}: '🪕'
    pub const BANJO: char = '🪕';
}

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
        use constants::*;
        match self {
            SymbolsandPictographsExtendedA::BalletShoes => BALLET_SHOES,
            SymbolsandPictographsExtendedA::OneDashPieceSwimsuit => ONE_DASH_PIECE_SWIMSUIT,
            SymbolsandPictographsExtendedA::Briefs => BRIEFS,
            SymbolsandPictographsExtendedA::Shorts => SHORTS,
            SymbolsandPictographsExtendedA::DropOfBlood => DROP_OF_BLOOD,
            SymbolsandPictographsExtendedA::AdhesiveBandage => ADHESIVE_BANDAGE,
            SymbolsandPictographsExtendedA::Stethoscope => STETHOSCOPE,
            SymbolsandPictographsExtendedA::YoDashYo => YO_DASH_YO,
            SymbolsandPictographsExtendedA::Kite => KITE,
            SymbolsandPictographsExtendedA::Parachute => PARACHUTE,
            SymbolsandPictographsExtendedA::RingedPlanet => RINGED_PLANET,
            SymbolsandPictographsExtendedA::Chair => CHAIR,
            SymbolsandPictographsExtendedA::Razor => RAZOR,
            SymbolsandPictographsExtendedA::Axe => AXE,
            SymbolsandPictographsExtendedA::DiyaLamp => DIYA_LAMP,
            SymbolsandPictographsExtendedA::Banjo => BANJO,
        }
    }
}

impl std::convert::TryFrom<char> for SymbolsandPictographsExtendedA {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            BALLET_SHOES => Ok(SymbolsandPictographsExtendedA::BalletShoes),
            ONE_DASH_PIECE_SWIMSUIT => Ok(SymbolsandPictographsExtendedA::OneDashPieceSwimsuit),
            BRIEFS => Ok(SymbolsandPictographsExtendedA::Briefs),
            SHORTS => Ok(SymbolsandPictographsExtendedA::Shorts),
            DROP_OF_BLOOD => Ok(SymbolsandPictographsExtendedA::DropOfBlood),
            ADHESIVE_BANDAGE => Ok(SymbolsandPictographsExtendedA::AdhesiveBandage),
            STETHOSCOPE => Ok(SymbolsandPictographsExtendedA::Stethoscope),
            YO_DASH_YO => Ok(SymbolsandPictographsExtendedA::YoDashYo),
            KITE => Ok(SymbolsandPictographsExtendedA::Kite),
            PARACHUTE => Ok(SymbolsandPictographsExtendedA::Parachute),
            RINGED_PLANET => Ok(SymbolsandPictographsExtendedA::RingedPlanet),
            CHAIR => Ok(SymbolsandPictographsExtendedA::Chair),
            RAZOR => Ok(SymbolsandPictographsExtendedA::Razor),
            AXE => Ok(SymbolsandPictographsExtendedA::Axe),
            DIYA_LAMP => Ok(SymbolsandPictographsExtendedA::DiyaLamp),
            BANJO => Ok(SymbolsandPictographsExtendedA::Banjo),
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

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            SymbolsandPictographsExtendedA::BalletShoes => "ballet shoes",
            SymbolsandPictographsExtendedA::OneDashPieceSwimsuit => "one-piece swimsuit",
            SymbolsandPictographsExtendedA::Briefs => "briefs",
            SymbolsandPictographsExtendedA::Shorts => "shorts",
            SymbolsandPictographsExtendedA::DropOfBlood => "drop of blood",
            SymbolsandPictographsExtendedA::AdhesiveBandage => "adhesive bandage",
            SymbolsandPictographsExtendedA::Stethoscope => "stethoscope",
            SymbolsandPictographsExtendedA::YoDashYo => "yo-yo",
            SymbolsandPictographsExtendedA::Kite => "kite",
            SymbolsandPictographsExtendedA::Parachute => "parachute",
            SymbolsandPictographsExtendedA::RingedPlanet => "ringed planet",
            SymbolsandPictographsExtendedA::Chair => "chair",
            SymbolsandPictographsExtendedA::Razor => "razor",
            SymbolsandPictographsExtendedA::Axe => "axe",
            SymbolsandPictographsExtendedA::DiyaLamp => "diya lamp",
            SymbolsandPictographsExtendedA::Banjo => "banjo",
        }
    }
}
