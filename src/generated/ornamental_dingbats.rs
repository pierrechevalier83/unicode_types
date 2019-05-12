/// A number of constants to give a name to all characters in this block.
mod constants {
    /// \u{1f650}: '🙐'
    pub const NORTH_WEST_POINTING_LEAF: char = '🙐';
    /// \u{1f651}: '🙑'
    pub const SOUTH_WEST_POINTING_LEAF: char = '🙑';
    /// \u{1f652}: '🙒'
    pub const NORTH_EAST_POINTING_LEAF: char = '🙒';
    /// \u{1f653}: '🙓'
    pub const SOUTH_EAST_POINTING_LEAF: char = '🙓';
    /// \u{1f654}: '🙔'
    pub const TURNED_NORTH_WEST_POINTING_LEAF: char = '🙔';
    /// \u{1f655}: '🙕'
    pub const TURNED_SOUTH_WEST_POINTING_LEAF: char = '🙕';
    /// \u{1f656}: '🙖'
    pub const TURNED_NORTH_EAST_POINTING_LEAF: char = '🙖';
    /// \u{1f657}: '🙗'
    pub const TURNED_SOUTH_EAST_POINTING_LEAF: char = '🙗';
    /// \u{1f658}: '🙘'
    pub const NORTH_WEST_POINTING_VINE_LEAF: char = '🙘';
    /// \u{1f659}: '🙙'
    pub const SOUTH_WEST_POINTING_VINE_LEAF: char = '🙙';
    /// \u{1f65a}: '🙚'
    pub const NORTH_EAST_POINTING_VINE_LEAF: char = '🙚';
    /// \u{1f65b}: '🙛'
    pub const SOUTH_EAST_POINTING_VINE_LEAF: char = '🙛';
    /// \u{1f65c}: '🙜'
    pub const HEAVY_NORTH_WEST_POINTING_VINE_LEAF: char = '🙜';
    /// \u{1f65d}: '🙝'
    pub const HEAVY_SOUTH_WEST_POINTING_VINE_LEAF: char = '🙝';
    /// \u{1f65e}: '🙞'
    pub const HEAVY_NORTH_EAST_POINTING_VINE_LEAF: char = '🙞';
    /// \u{1f65f}: '🙟'
    pub const HEAVY_SOUTH_EAST_POINTING_VINE_LEAF: char = '🙟';
    /// \u{1f660}: '🙠'
    pub const NORTH_WEST_POINTING_BUD: char = '🙠';
    /// \u{1f661}: '🙡'
    pub const SOUTH_WEST_POINTING_BUD: char = '🙡';
    /// \u{1f662}: '🙢'
    pub const NORTH_EAST_POINTING_BUD: char = '🙢';
    /// \u{1f663}: '🙣'
    pub const SOUTH_EAST_POINTING_BUD: char = '🙣';
    /// \u{1f664}: '🙤'
    pub const HEAVY_NORTH_WEST_POINTING_BUD: char = '🙤';
    /// \u{1f665}: '🙥'
    pub const HEAVY_SOUTH_WEST_POINTING_BUD: char = '🙥';
    /// \u{1f666}: '🙦'
    pub const HEAVY_NORTH_EAST_POINTING_BUD: char = '🙦';
    /// \u{1f667}: '🙧'
    pub const HEAVY_SOUTH_EAST_POINTING_BUD: char = '🙧';
    /// \u{1f668}: '🙨'
    pub const HOLLOW_QUILT_SQUARE_ORNAMENT: char = '🙨';
    /// \u{1f669}: '🙩'
    pub const HOLLOW_QUILT_SQUARE_ORNAMENT_IN_BLACK_SQUARE: char = '🙩';
    /// \u{1f66a}: '🙪'
    pub const SOLID_QUILT_SQUARE_ORNAMENT: char = '🙪';
    /// \u{1f66b}: '🙫'
    pub const SOLID_QUILT_SQUARE_ORNAMENT_IN_BLACK_SQUARE: char = '🙫';
    /// \u{1f66c}: '🙬'
    pub const LEFTWARDS_ROCKET: char = '🙬';
    /// \u{1f66d}: '🙭'
    pub const UPWARDS_ROCKET: char = '🙭';
    /// \u{1f66e}: '🙮'
    pub const RIGHTWARDS_ROCKET: char = '🙮';
    /// \u{1f66f}: '🙯'
    pub const DOWNWARDS_ROCKET: char = '🙯';
    /// \u{1f670}: '🙰'
    pub const SCRIPT_LIGATURE_ET_ORNAMENT: char = '🙰';
    /// \u{1f671}: '🙱'
    pub const HEAVY_SCRIPT_LIGATURE_ET_ORNAMENT: char = '🙱';
    /// \u{1f672}: '🙲'
    pub const LIGATURE_OPEN_ET_ORNAMENT: char = '🙲';
    /// \u{1f673}: '🙳'
    pub const HEAVY_LIGATURE_OPEN_ET_ORNAMENT: char = '🙳';
    /// \u{1f674}: '🙴'
    pub const HEAVY_AMPERSAND_ORNAMENT: char = '🙴';
    /// \u{1f675}: '🙵'
    pub const SWASH_AMPERSAND_ORNAMENT: char = '🙵';
    /// \u{1f676}: '🙶'
    pub const SANS_DASH_SERIF_HEAVY_DOUBLE_TURNED_COMMA_QUOTATION_MARK_ORNAMENT: char = '🙶';
    /// \u{1f677}: '🙷'
    pub const SANS_DASH_SERIF_HEAVY_DOUBLE_COMMA_QUOTATION_MARK_ORNAMENT: char = '🙷';
    /// \u{1f678}: '🙸'
    pub const SANS_DASH_SERIF_HEAVY_LOW_DOUBLE_COMMA_QUOTATION_MARK_ORNAMENT: char = '🙸';
    /// \u{1f679}: '🙹'
    pub const HEAVY_INTERROBANG_ORNAMENT: char = '🙹';
    /// \u{1f67a}: '🙺'
    pub const SANS_DASH_SERIF_INTERROBANG_ORNAMENT: char = '🙺';
    /// \u{1f67b}: '🙻'
    pub const HEAVY_SANS_DASH_SERIF_INTERROBANG_ORNAMENT: char = '🙻';
    /// \u{1f67c}: '🙼'
    pub const VERY_HEAVY_SOLIDUS: char = '🙼';
    /// \u{1f67d}: '🙽'
    pub const VERY_HEAVY_REVERSE_SOLIDUS: char = '🙽';
    /// \u{1f67e}: '🙾'
    pub const CHECKER_BOARD: char = '🙾';
}

/// An enum to represent all characters in the OrnamentalDingbats block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum OrnamentalDingbats {
    /// \u{1f650}: '🙐'
    NorthWestPointingLeaf,
    /// \u{1f651}: '🙑'
    SouthWestPointingLeaf,
    /// \u{1f652}: '🙒'
    NorthEastPointingLeaf,
    /// \u{1f653}: '🙓'
    SouthEastPointingLeaf,
    /// \u{1f654}: '🙔'
    TurnedNorthWestPointingLeaf,
    /// \u{1f655}: '🙕'
    TurnedSouthWestPointingLeaf,
    /// \u{1f656}: '🙖'
    TurnedNorthEastPointingLeaf,
    /// \u{1f657}: '🙗'
    TurnedSouthEastPointingLeaf,
    /// \u{1f658}: '🙘'
    NorthWestPointingVineLeaf,
    /// \u{1f659}: '🙙'
    SouthWestPointingVineLeaf,
    /// \u{1f65a}: '🙚'
    NorthEastPointingVineLeaf,
    /// \u{1f65b}: '🙛'
    SouthEastPointingVineLeaf,
    /// \u{1f65c}: '🙜'
    HeavyNorthWestPointingVineLeaf,
    /// \u{1f65d}: '🙝'
    HeavySouthWestPointingVineLeaf,
    /// \u{1f65e}: '🙞'
    HeavyNorthEastPointingVineLeaf,
    /// \u{1f65f}: '🙟'
    HeavySouthEastPointingVineLeaf,
    /// \u{1f660}: '🙠'
    NorthWestPointingBud,
    /// \u{1f661}: '🙡'
    SouthWestPointingBud,
    /// \u{1f662}: '🙢'
    NorthEastPointingBud,
    /// \u{1f663}: '🙣'
    SouthEastPointingBud,
    /// \u{1f664}: '🙤'
    HeavyNorthWestPointingBud,
    /// \u{1f665}: '🙥'
    HeavySouthWestPointingBud,
    /// \u{1f666}: '🙦'
    HeavyNorthEastPointingBud,
    /// \u{1f667}: '🙧'
    HeavySouthEastPointingBud,
    /// \u{1f668}: '🙨'
    HollowQuiltSquareOrnament,
    /// \u{1f669}: '🙩'
    HollowQuiltSquareOrnamentInBlackSquare,
    /// \u{1f66a}: '🙪'
    SolidQuiltSquareOrnament,
    /// \u{1f66b}: '🙫'
    SolidQuiltSquareOrnamentInBlackSquare,
    /// \u{1f66c}: '🙬'
    LeftwardsRocket,
    /// \u{1f66d}: '🙭'
    UpwardsRocket,
    /// \u{1f66e}: '🙮'
    RightwardsRocket,
    /// \u{1f66f}: '🙯'
    DownwardsRocket,
    /// \u{1f670}: '🙰'
    ScriptLigatureEtOrnament,
    /// \u{1f671}: '🙱'
    HeavyScriptLigatureEtOrnament,
    /// \u{1f672}: '🙲'
    LigatureOpenEtOrnament,
    /// \u{1f673}: '🙳'
    HeavyLigatureOpenEtOrnament,
    /// \u{1f674}: '🙴'
    HeavyAmpersandOrnament,
    /// \u{1f675}: '🙵'
    SwashAmpersandOrnament,
    /// \u{1f676}: '🙶'
    SansDashSerifHeavyDoubleTurnedCommaQuotationMarkOrnament,
    /// \u{1f677}: '🙷'
    SansDashSerifHeavyDoubleCommaQuotationMarkOrnament,
    /// \u{1f678}: '🙸'
    SansDashSerifHeavyLowDoubleCommaQuotationMarkOrnament,
    /// \u{1f679}: '🙹'
    HeavyInterrobangOrnament,
    /// \u{1f67a}: '🙺'
    SansDashSerifInterrobangOrnament,
    /// \u{1f67b}: '🙻'
    HeavySansDashSerifInterrobangOrnament,
    /// \u{1f67c}: '🙼'
    VeryHeavySolidus,
    /// \u{1f67d}: '🙽'
    VeryHeavyReverseSolidus,
    /// \u{1f67e}: '🙾'
    CheckerBoard,
}

impl Into<char> for OrnamentalDingbats {
    fn into(self) -> char {
        use constants::*;
        match self {
            OrnamentalDingbats::NorthWestPointingLeaf => NORTH_WEST_POINTING_LEAF,
            OrnamentalDingbats::SouthWestPointingLeaf => SOUTH_WEST_POINTING_LEAF,
            OrnamentalDingbats::NorthEastPointingLeaf => NORTH_EAST_POINTING_LEAF,
            OrnamentalDingbats::SouthEastPointingLeaf => SOUTH_EAST_POINTING_LEAF,
            OrnamentalDingbats::TurnedNorthWestPointingLeaf => TURNED_NORTH_WEST_POINTING_LEAF,
            OrnamentalDingbats::TurnedSouthWestPointingLeaf => TURNED_SOUTH_WEST_POINTING_LEAF,
            OrnamentalDingbats::TurnedNorthEastPointingLeaf => TURNED_NORTH_EAST_POINTING_LEAF,
            OrnamentalDingbats::TurnedSouthEastPointingLeaf => TURNED_SOUTH_EAST_POINTING_LEAF,
            OrnamentalDingbats::NorthWestPointingVineLeaf => NORTH_WEST_POINTING_VINE_LEAF,
            OrnamentalDingbats::SouthWestPointingVineLeaf => SOUTH_WEST_POINTING_VINE_LEAF,
            OrnamentalDingbats::NorthEastPointingVineLeaf => NORTH_EAST_POINTING_VINE_LEAF,
            OrnamentalDingbats::SouthEastPointingVineLeaf => SOUTH_EAST_POINTING_VINE_LEAF,
            OrnamentalDingbats::HeavyNorthWestPointingVineLeaf => HEAVY_NORTH_WEST_POINTING_VINE_LEAF,
            OrnamentalDingbats::HeavySouthWestPointingVineLeaf => HEAVY_SOUTH_WEST_POINTING_VINE_LEAF,
            OrnamentalDingbats::HeavyNorthEastPointingVineLeaf => HEAVY_NORTH_EAST_POINTING_VINE_LEAF,
            OrnamentalDingbats::HeavySouthEastPointingVineLeaf => HEAVY_SOUTH_EAST_POINTING_VINE_LEAF,
            OrnamentalDingbats::NorthWestPointingBud => NORTH_WEST_POINTING_BUD,
            OrnamentalDingbats::SouthWestPointingBud => SOUTH_WEST_POINTING_BUD,
            OrnamentalDingbats::NorthEastPointingBud => NORTH_EAST_POINTING_BUD,
            OrnamentalDingbats::SouthEastPointingBud => SOUTH_EAST_POINTING_BUD,
            OrnamentalDingbats::HeavyNorthWestPointingBud => HEAVY_NORTH_WEST_POINTING_BUD,
            OrnamentalDingbats::HeavySouthWestPointingBud => HEAVY_SOUTH_WEST_POINTING_BUD,
            OrnamentalDingbats::HeavyNorthEastPointingBud => HEAVY_NORTH_EAST_POINTING_BUD,
            OrnamentalDingbats::HeavySouthEastPointingBud => HEAVY_SOUTH_EAST_POINTING_BUD,
            OrnamentalDingbats::HollowQuiltSquareOrnament => HOLLOW_QUILT_SQUARE_ORNAMENT,
            OrnamentalDingbats::HollowQuiltSquareOrnamentInBlackSquare => HOLLOW_QUILT_SQUARE_ORNAMENT_IN_BLACK_SQUARE,
            OrnamentalDingbats::SolidQuiltSquareOrnament => SOLID_QUILT_SQUARE_ORNAMENT,
            OrnamentalDingbats::SolidQuiltSquareOrnamentInBlackSquare => SOLID_QUILT_SQUARE_ORNAMENT_IN_BLACK_SQUARE,
            OrnamentalDingbats::LeftwardsRocket => LEFTWARDS_ROCKET,
            OrnamentalDingbats::UpwardsRocket => UPWARDS_ROCKET,
            OrnamentalDingbats::RightwardsRocket => RIGHTWARDS_ROCKET,
            OrnamentalDingbats::DownwardsRocket => DOWNWARDS_ROCKET,
            OrnamentalDingbats::ScriptLigatureEtOrnament => SCRIPT_LIGATURE_ET_ORNAMENT,
            OrnamentalDingbats::HeavyScriptLigatureEtOrnament => HEAVY_SCRIPT_LIGATURE_ET_ORNAMENT,
            OrnamentalDingbats::LigatureOpenEtOrnament => LIGATURE_OPEN_ET_ORNAMENT,
            OrnamentalDingbats::HeavyLigatureOpenEtOrnament => HEAVY_LIGATURE_OPEN_ET_ORNAMENT,
            OrnamentalDingbats::HeavyAmpersandOrnament => HEAVY_AMPERSAND_ORNAMENT,
            OrnamentalDingbats::SwashAmpersandOrnament => SWASH_AMPERSAND_ORNAMENT,
            OrnamentalDingbats::SansDashSerifHeavyDoubleTurnedCommaQuotationMarkOrnament => SANS_DASH_SERIF_HEAVY_DOUBLE_TURNED_COMMA_QUOTATION_MARK_ORNAMENT,
            OrnamentalDingbats::SansDashSerifHeavyDoubleCommaQuotationMarkOrnament => SANS_DASH_SERIF_HEAVY_DOUBLE_COMMA_QUOTATION_MARK_ORNAMENT,
            OrnamentalDingbats::SansDashSerifHeavyLowDoubleCommaQuotationMarkOrnament => SANS_DASH_SERIF_HEAVY_LOW_DOUBLE_COMMA_QUOTATION_MARK_ORNAMENT,
            OrnamentalDingbats::HeavyInterrobangOrnament => HEAVY_INTERROBANG_ORNAMENT,
            OrnamentalDingbats::SansDashSerifInterrobangOrnament => SANS_DASH_SERIF_INTERROBANG_ORNAMENT,
            OrnamentalDingbats::HeavySansDashSerifInterrobangOrnament => HEAVY_SANS_DASH_SERIF_INTERROBANG_ORNAMENT,
            OrnamentalDingbats::VeryHeavySolidus => VERY_HEAVY_SOLIDUS,
            OrnamentalDingbats::VeryHeavyReverseSolidus => VERY_HEAVY_REVERSE_SOLIDUS,
            OrnamentalDingbats::CheckerBoard => CHECKER_BOARD,
        }
    }
}

impl std::convert::TryFrom<char> for OrnamentalDingbats {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            NORTH_WEST_POINTING_LEAF => Ok(OrnamentalDingbats::NorthWestPointingLeaf),
            SOUTH_WEST_POINTING_LEAF => Ok(OrnamentalDingbats::SouthWestPointingLeaf),
            NORTH_EAST_POINTING_LEAF => Ok(OrnamentalDingbats::NorthEastPointingLeaf),
            SOUTH_EAST_POINTING_LEAF => Ok(OrnamentalDingbats::SouthEastPointingLeaf),
            TURNED_NORTH_WEST_POINTING_LEAF => Ok(OrnamentalDingbats::TurnedNorthWestPointingLeaf),
            TURNED_SOUTH_WEST_POINTING_LEAF => Ok(OrnamentalDingbats::TurnedSouthWestPointingLeaf),
            TURNED_NORTH_EAST_POINTING_LEAF => Ok(OrnamentalDingbats::TurnedNorthEastPointingLeaf),
            TURNED_SOUTH_EAST_POINTING_LEAF => Ok(OrnamentalDingbats::TurnedSouthEastPointingLeaf),
            NORTH_WEST_POINTING_VINE_LEAF => Ok(OrnamentalDingbats::NorthWestPointingVineLeaf),
            SOUTH_WEST_POINTING_VINE_LEAF => Ok(OrnamentalDingbats::SouthWestPointingVineLeaf),
            NORTH_EAST_POINTING_VINE_LEAF => Ok(OrnamentalDingbats::NorthEastPointingVineLeaf),
            SOUTH_EAST_POINTING_VINE_LEAF => Ok(OrnamentalDingbats::SouthEastPointingVineLeaf),
            HEAVY_NORTH_WEST_POINTING_VINE_LEAF => Ok(OrnamentalDingbats::HeavyNorthWestPointingVineLeaf),
            HEAVY_SOUTH_WEST_POINTING_VINE_LEAF => Ok(OrnamentalDingbats::HeavySouthWestPointingVineLeaf),
            HEAVY_NORTH_EAST_POINTING_VINE_LEAF => Ok(OrnamentalDingbats::HeavyNorthEastPointingVineLeaf),
            HEAVY_SOUTH_EAST_POINTING_VINE_LEAF => Ok(OrnamentalDingbats::HeavySouthEastPointingVineLeaf),
            NORTH_WEST_POINTING_BUD => Ok(OrnamentalDingbats::NorthWestPointingBud),
            SOUTH_WEST_POINTING_BUD => Ok(OrnamentalDingbats::SouthWestPointingBud),
            NORTH_EAST_POINTING_BUD => Ok(OrnamentalDingbats::NorthEastPointingBud),
            SOUTH_EAST_POINTING_BUD => Ok(OrnamentalDingbats::SouthEastPointingBud),
            HEAVY_NORTH_WEST_POINTING_BUD => Ok(OrnamentalDingbats::HeavyNorthWestPointingBud),
            HEAVY_SOUTH_WEST_POINTING_BUD => Ok(OrnamentalDingbats::HeavySouthWestPointingBud),
            HEAVY_NORTH_EAST_POINTING_BUD => Ok(OrnamentalDingbats::HeavyNorthEastPointingBud),
            HEAVY_SOUTH_EAST_POINTING_BUD => Ok(OrnamentalDingbats::HeavySouthEastPointingBud),
            HOLLOW_QUILT_SQUARE_ORNAMENT => Ok(OrnamentalDingbats::HollowQuiltSquareOrnament),
            HOLLOW_QUILT_SQUARE_ORNAMENT_IN_BLACK_SQUARE => Ok(OrnamentalDingbats::HollowQuiltSquareOrnamentInBlackSquare),
            SOLID_QUILT_SQUARE_ORNAMENT => Ok(OrnamentalDingbats::SolidQuiltSquareOrnament),
            SOLID_QUILT_SQUARE_ORNAMENT_IN_BLACK_SQUARE => Ok(OrnamentalDingbats::SolidQuiltSquareOrnamentInBlackSquare),
            LEFTWARDS_ROCKET => Ok(OrnamentalDingbats::LeftwardsRocket),
            UPWARDS_ROCKET => Ok(OrnamentalDingbats::UpwardsRocket),
            RIGHTWARDS_ROCKET => Ok(OrnamentalDingbats::RightwardsRocket),
            DOWNWARDS_ROCKET => Ok(OrnamentalDingbats::DownwardsRocket),
            SCRIPT_LIGATURE_ET_ORNAMENT => Ok(OrnamentalDingbats::ScriptLigatureEtOrnament),
            HEAVY_SCRIPT_LIGATURE_ET_ORNAMENT => Ok(OrnamentalDingbats::HeavyScriptLigatureEtOrnament),
            LIGATURE_OPEN_ET_ORNAMENT => Ok(OrnamentalDingbats::LigatureOpenEtOrnament),
            HEAVY_LIGATURE_OPEN_ET_ORNAMENT => Ok(OrnamentalDingbats::HeavyLigatureOpenEtOrnament),
            HEAVY_AMPERSAND_ORNAMENT => Ok(OrnamentalDingbats::HeavyAmpersandOrnament),
            SWASH_AMPERSAND_ORNAMENT => Ok(OrnamentalDingbats::SwashAmpersandOrnament),
            SANS_DASH_SERIF_HEAVY_DOUBLE_TURNED_COMMA_QUOTATION_MARK_ORNAMENT => Ok(OrnamentalDingbats::SansDashSerifHeavyDoubleTurnedCommaQuotationMarkOrnament),
            SANS_DASH_SERIF_HEAVY_DOUBLE_COMMA_QUOTATION_MARK_ORNAMENT => Ok(OrnamentalDingbats::SansDashSerifHeavyDoubleCommaQuotationMarkOrnament),
            SANS_DASH_SERIF_HEAVY_LOW_DOUBLE_COMMA_QUOTATION_MARK_ORNAMENT => Ok(OrnamentalDingbats::SansDashSerifHeavyLowDoubleCommaQuotationMarkOrnament),
            HEAVY_INTERROBANG_ORNAMENT => Ok(OrnamentalDingbats::HeavyInterrobangOrnament),
            SANS_DASH_SERIF_INTERROBANG_ORNAMENT => Ok(OrnamentalDingbats::SansDashSerifInterrobangOrnament),
            HEAVY_SANS_DASH_SERIF_INTERROBANG_ORNAMENT => Ok(OrnamentalDingbats::HeavySansDashSerifInterrobangOrnament),
            VERY_HEAVY_SOLIDUS => Ok(OrnamentalDingbats::VeryHeavySolidus),
            VERY_HEAVY_REVERSE_SOLIDUS => Ok(OrnamentalDingbats::VeryHeavyReverseSolidus),
            CHECKER_BOARD => Ok(OrnamentalDingbats::CheckerBoard),
            _ => Err(()),
        }
    }
}

impl Into<u32> for OrnamentalDingbats {
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

impl std::convert::TryFrom<u32> for OrnamentalDingbats {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for OrnamentalDingbats {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl OrnamentalDingbats {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        OrnamentalDingbats::NorthWestPointingLeaf
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            OrnamentalDingbats::NorthWestPointingLeaf => "north west pointing leaf",
            OrnamentalDingbats::SouthWestPointingLeaf => "south west pointing leaf",
            OrnamentalDingbats::NorthEastPointingLeaf => "north east pointing leaf",
            OrnamentalDingbats::SouthEastPointingLeaf => "south east pointing leaf",
            OrnamentalDingbats::TurnedNorthWestPointingLeaf => "turned north west pointing leaf",
            OrnamentalDingbats::TurnedSouthWestPointingLeaf => "turned south west pointing leaf",
            OrnamentalDingbats::TurnedNorthEastPointingLeaf => "turned north east pointing leaf",
            OrnamentalDingbats::TurnedSouthEastPointingLeaf => "turned south east pointing leaf",
            OrnamentalDingbats::NorthWestPointingVineLeaf => "north west pointing vine leaf",
            OrnamentalDingbats::SouthWestPointingVineLeaf => "south west pointing vine leaf",
            OrnamentalDingbats::NorthEastPointingVineLeaf => "north east pointing vine leaf",
            OrnamentalDingbats::SouthEastPointingVineLeaf => "south east pointing vine leaf",
            OrnamentalDingbats::HeavyNorthWestPointingVineLeaf => "heavy north west pointing vine leaf",
            OrnamentalDingbats::HeavySouthWestPointingVineLeaf => "heavy south west pointing vine leaf",
            OrnamentalDingbats::HeavyNorthEastPointingVineLeaf => "heavy north east pointing vine leaf",
            OrnamentalDingbats::HeavySouthEastPointingVineLeaf => "heavy south east pointing vine leaf",
            OrnamentalDingbats::NorthWestPointingBud => "north west pointing bud",
            OrnamentalDingbats::SouthWestPointingBud => "south west pointing bud",
            OrnamentalDingbats::NorthEastPointingBud => "north east pointing bud",
            OrnamentalDingbats::SouthEastPointingBud => "south east pointing bud",
            OrnamentalDingbats::HeavyNorthWestPointingBud => "heavy north west pointing bud",
            OrnamentalDingbats::HeavySouthWestPointingBud => "heavy south west pointing bud",
            OrnamentalDingbats::HeavyNorthEastPointingBud => "heavy north east pointing bud",
            OrnamentalDingbats::HeavySouthEastPointingBud => "heavy south east pointing bud",
            OrnamentalDingbats::HollowQuiltSquareOrnament => "hollow quilt square ornament",
            OrnamentalDingbats::HollowQuiltSquareOrnamentInBlackSquare => "hollow quilt square ornament in black square",
            OrnamentalDingbats::SolidQuiltSquareOrnament => "solid quilt square ornament",
            OrnamentalDingbats::SolidQuiltSquareOrnamentInBlackSquare => "solid quilt square ornament in black square",
            OrnamentalDingbats::LeftwardsRocket => "leftwards rocket",
            OrnamentalDingbats::UpwardsRocket => "upwards rocket",
            OrnamentalDingbats::RightwardsRocket => "rightwards rocket",
            OrnamentalDingbats::DownwardsRocket => "downwards rocket",
            OrnamentalDingbats::ScriptLigatureEtOrnament => "script ligature et ornament",
            OrnamentalDingbats::HeavyScriptLigatureEtOrnament => "heavy script ligature et ornament",
            OrnamentalDingbats::LigatureOpenEtOrnament => "ligature open et ornament",
            OrnamentalDingbats::HeavyLigatureOpenEtOrnament => "heavy ligature open et ornament",
            OrnamentalDingbats::HeavyAmpersandOrnament => "heavy ampersand ornament",
            OrnamentalDingbats::SwashAmpersandOrnament => "swash ampersand ornament",
            OrnamentalDingbats::SansDashSerifHeavyDoubleTurnedCommaQuotationMarkOrnament => "sans-serif heavy double turned comma quotation mark ornament",
            OrnamentalDingbats::SansDashSerifHeavyDoubleCommaQuotationMarkOrnament => "sans-serif heavy double comma quotation mark ornament",
            OrnamentalDingbats::SansDashSerifHeavyLowDoubleCommaQuotationMarkOrnament => "sans-serif heavy low double comma quotation mark ornament",
            OrnamentalDingbats::HeavyInterrobangOrnament => "heavy interrobang ornament",
            OrnamentalDingbats::SansDashSerifInterrobangOrnament => "sans-serif interrobang ornament",
            OrnamentalDingbats::HeavySansDashSerifInterrobangOrnament => "heavy sans-serif interrobang ornament",
            OrnamentalDingbats::VeryHeavySolidus => "very heavy solidus",
            OrnamentalDingbats::VeryHeavyReverseSolidus => "very heavy reverse solidus",
            OrnamentalDingbats::CheckerBoard => "checker board",
        }
    }
}
