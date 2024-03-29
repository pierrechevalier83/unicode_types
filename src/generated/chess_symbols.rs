
/// An enum to represent all characters in the ChessSymbols block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ChessSymbols {
    /// \u{1fa00}: '🨀'
    NeutralChessKing,
    /// \u{1fa01}: '🨁'
    NeutralChessQueen,
    /// \u{1fa02}: '🨂'
    NeutralChessRook,
    /// \u{1fa03}: '🨃'
    NeutralChessBishop,
    /// \u{1fa04}: '🨄'
    NeutralChessKnight,
    /// \u{1fa05}: '🨅'
    NeutralChessPawn,
    /// \u{1fa06}: '🨆'
    WhiteChessKnightRotatedFortyDashFiveDegrees,
    /// \u{1fa07}: '🨇'
    BlackChessKnightRotatedFortyDashFiveDegrees,
    /// \u{1fa08}: '🨈'
    NeutralChessKnightRotatedFortyDashFiveDegrees,
    /// \u{1fa09}: '🨉'
    WhiteChessKingRotatedNinetyDegrees,
    /// \u{1fa0a}: '🨊'
    WhiteChessQueenRotatedNinetyDegrees,
    /// \u{1fa0b}: '🨋'
    WhiteChessRookRotatedNinetyDegrees,
    /// \u{1fa0c}: '🨌'
    WhiteChessBishopRotatedNinetyDegrees,
    /// \u{1fa0d}: '🨍'
    WhiteChessKnightRotatedNinetyDegrees,
    /// \u{1fa0e}: '🨎'
    WhiteChessPawnRotatedNinetyDegrees,
    /// \u{1fa0f}: '🨏'
    BlackChessKingRotatedNinetyDegrees,
    /// \u{1fa10}: '🨐'
    BlackChessQueenRotatedNinetyDegrees,
    /// \u{1fa11}: '🨑'
    BlackChessRookRotatedNinetyDegrees,
    /// \u{1fa12}: '🨒'
    BlackChessBishopRotatedNinetyDegrees,
    /// \u{1fa13}: '🨓'
    BlackChessKnightRotatedNinetyDegrees,
    /// \u{1fa14}: '🨔'
    BlackChessPawnRotatedNinetyDegrees,
    /// \u{1fa15}: '🨕'
    NeutralChessKingRotatedNinetyDegrees,
    /// \u{1fa16}: '🨖'
    NeutralChessQueenRotatedNinetyDegrees,
    /// \u{1fa17}: '🨗'
    NeutralChessRookRotatedNinetyDegrees,
    /// \u{1fa18}: '🨘'
    NeutralChessBishopRotatedNinetyDegrees,
    /// \u{1fa19}: '🨙'
    NeutralChessKnightRotatedNinetyDegrees,
    /// \u{1fa1a}: '🨚'
    NeutralChessPawnRotatedNinetyDegrees,
    /// \u{1fa1b}: '🨛'
    WhiteChessKnightRotatedOneHundredThirtyDashFiveDegrees,
    /// \u{1fa1c}: '🨜'
    BlackChessKnightRotatedOneHundredThirtyDashFiveDegrees,
    /// \u{1fa1d}: '🨝'
    NeutralChessKnightRotatedOneHundredThirtyDashFiveDegrees,
    /// \u{1fa1e}: '🨞'
    WhiteChessTurnedKing,
    /// \u{1fa1f}: '🨟'
    WhiteChessTurnedQueen,
    /// \u{1fa20}: '🨠'
    WhiteChessTurnedRook,
    /// \u{1fa21}: '🨡'
    WhiteChessTurnedBishop,
    /// \u{1fa22}: '🨢'
    WhiteChessTurnedKnight,
    /// \u{1fa23}: '🨣'
    WhiteChessTurnedPawn,
    /// \u{1fa24}: '🨤'
    BlackChessTurnedKing,
    /// \u{1fa25}: '🨥'
    BlackChessTurnedQueen,
    /// \u{1fa26}: '🨦'
    BlackChessTurnedRook,
    /// \u{1fa27}: '🨧'
    BlackChessTurnedBishop,
    /// \u{1fa28}: '🨨'
    BlackChessTurnedKnight,
    /// \u{1fa29}: '🨩'
    BlackChessTurnedPawn,
    /// \u{1fa2a}: '🨪'
    NeutralChessTurnedKing,
    /// \u{1fa2b}: '🨫'
    NeutralChessTurnedQueen,
    /// \u{1fa2c}: '🨬'
    NeutralChessTurnedRook,
    /// \u{1fa2d}: '🨭'
    NeutralChessTurnedBishop,
    /// \u{1fa2e}: '🨮'
    NeutralChessTurnedKnight,
    /// \u{1fa2f}: '🨯'
    NeutralChessTurnedPawn,
    /// \u{1fa30}: '🨰'
    WhiteChessKnightRotatedTwoHundredTwentyDashFiveDegrees,
    /// \u{1fa31}: '🨱'
    BlackChessKnightRotatedTwoHundredTwentyDashFiveDegrees,
    /// \u{1fa32}: '🨲'
    NeutralChessKnightRotatedTwoHundredTwentyDashFiveDegrees,
    /// \u{1fa33}: '🨳'
    WhiteChessKingRotatedTwoHundredSeventyDegrees,
    /// \u{1fa34}: '🨴'
    WhiteChessQueenRotatedTwoHundredSeventyDegrees,
    /// \u{1fa35}: '🨵'
    WhiteChessRookRotatedTwoHundredSeventyDegrees,
    /// \u{1fa36}: '🨶'
    WhiteChessBishopRotatedTwoHundredSeventyDegrees,
    /// \u{1fa37}: '🨷'
    WhiteChessKnightRotatedTwoHundredSeventyDegrees,
    /// \u{1fa38}: '🨸'
    WhiteChessPawnRotatedTwoHundredSeventyDegrees,
    /// \u{1fa39}: '🨹'
    BlackChessKingRotatedTwoHundredSeventyDegrees,
    /// \u{1fa3a}: '🨺'
    BlackChessQueenRotatedTwoHundredSeventyDegrees,
    /// \u{1fa3b}: '🨻'
    BlackChessRookRotatedTwoHundredSeventyDegrees,
    /// \u{1fa3c}: '🨼'
    BlackChessBishopRotatedTwoHundredSeventyDegrees,
    /// \u{1fa3d}: '🨽'
    BlackChessKnightRotatedTwoHundredSeventyDegrees,
    /// \u{1fa3e}: '🨾'
    BlackChessPawnRotatedTwoHundredSeventyDegrees,
    /// \u{1fa3f}: '🨿'
    NeutralChessKingRotatedTwoHundredSeventyDegrees,
    /// \u{1fa40}: '🩀'
    NeutralChessQueenRotatedTwoHundredSeventyDegrees,
    /// \u{1fa41}: '🩁'
    NeutralChessRookRotatedTwoHundredSeventyDegrees,
    /// \u{1fa42}: '🩂'
    NeutralChessBishopRotatedTwoHundredSeventyDegrees,
    /// \u{1fa43}: '🩃'
    NeutralChessKnightRotatedTwoHundredSeventyDegrees,
    /// \u{1fa44}: '🩄'
    NeutralChessPawnRotatedTwoHundredSeventyDegrees,
    /// \u{1fa45}: '🩅'
    WhiteChessKnightRotatedThreeHundredFifteenDegrees,
    /// \u{1fa46}: '🩆'
    BlackChessKnightRotatedThreeHundredFifteenDegrees,
    /// \u{1fa47}: '🩇'
    NeutralChessKnightRotatedThreeHundredFifteenDegrees,
    /// \u{1fa48}: '🩈'
    WhiteChessEquihopper,
    /// \u{1fa49}: '🩉'
    BlackChessEquihopper,
    /// \u{1fa4a}: '🩊'
    NeutralChessEquihopper,
    /// \u{1fa4b}: '🩋'
    WhiteChessEquihopperRotatedNinetyDegrees,
    /// \u{1fa4c}: '🩌'
    BlackChessEquihopperRotatedNinetyDegrees,
    /// \u{1fa4d}: '🩍'
    NeutralChessEquihopperRotatedNinetyDegrees,
    /// \u{1fa4e}: '🩎'
    WhiteChessKnightDashQueen,
    /// \u{1fa4f}: '🩏'
    WhiteChessKnightDashRook,
    /// \u{1fa50}: '🩐'
    WhiteChessKnightDashBishop,
    /// \u{1fa51}: '🩑'
    BlackChessKnightDashQueen,
    /// \u{1fa52}: '🩒'
    BlackChessKnightDashRook,
    /// \u{1fa53}: '🩓'
    BlackChessKnightDashBishop,
    /// \u{1fa60}: '🩠'
    XiangqiRedGeneral,
    /// \u{1fa61}: '🩡'
    XiangqiRedMandarin,
    /// \u{1fa62}: '🩢'
    XiangqiRedElephant,
    /// \u{1fa63}: '🩣'
    XiangqiRedHorse,
    /// \u{1fa64}: '🩤'
    XiangqiRedChariot,
    /// \u{1fa65}: '🩥'
    XiangqiRedCannon,
    /// \u{1fa66}: '🩦'
    XiangqiRedSoldier,
    /// \u{1fa67}: '🩧'
    XiangqiBlackGeneral,
    /// \u{1fa68}: '🩨'
    XiangqiBlackMandarin,
    /// \u{1fa69}: '🩩'
    XiangqiBlackElephant,
    /// \u{1fa6a}: '🩪'
    XiangqiBlackHorse,
    /// \u{1fa6b}: '🩫'
    XiangqiBlackChariot,
    /// \u{1fa6c}: '🩬'
    XiangqiBlackCannon,
    /// \u{1fa6d}: '🩭'
    XiangqiBlackSoldier,
}

impl Into<char> for ChessSymbols {
    fn into(self) -> char {
        match self {
            ChessSymbols::NeutralChessKing => '🨀',
            ChessSymbols::NeutralChessQueen => '🨁',
            ChessSymbols::NeutralChessRook => '🨂',
            ChessSymbols::NeutralChessBishop => '🨃',
            ChessSymbols::NeutralChessKnight => '🨄',
            ChessSymbols::NeutralChessPawn => '🨅',
            ChessSymbols::WhiteChessKnightRotatedFortyDashFiveDegrees => '🨆',
            ChessSymbols::BlackChessKnightRotatedFortyDashFiveDegrees => '🨇',
            ChessSymbols::NeutralChessKnightRotatedFortyDashFiveDegrees => '🨈',
            ChessSymbols::WhiteChessKingRotatedNinetyDegrees => '🨉',
            ChessSymbols::WhiteChessQueenRotatedNinetyDegrees => '🨊',
            ChessSymbols::WhiteChessRookRotatedNinetyDegrees => '🨋',
            ChessSymbols::WhiteChessBishopRotatedNinetyDegrees => '🨌',
            ChessSymbols::WhiteChessKnightRotatedNinetyDegrees => '🨍',
            ChessSymbols::WhiteChessPawnRotatedNinetyDegrees => '🨎',
            ChessSymbols::BlackChessKingRotatedNinetyDegrees => '🨏',
            ChessSymbols::BlackChessQueenRotatedNinetyDegrees => '🨐',
            ChessSymbols::BlackChessRookRotatedNinetyDegrees => '🨑',
            ChessSymbols::BlackChessBishopRotatedNinetyDegrees => '🨒',
            ChessSymbols::BlackChessKnightRotatedNinetyDegrees => '🨓',
            ChessSymbols::BlackChessPawnRotatedNinetyDegrees => '🨔',
            ChessSymbols::NeutralChessKingRotatedNinetyDegrees => '🨕',
            ChessSymbols::NeutralChessQueenRotatedNinetyDegrees => '🨖',
            ChessSymbols::NeutralChessRookRotatedNinetyDegrees => '🨗',
            ChessSymbols::NeutralChessBishopRotatedNinetyDegrees => '🨘',
            ChessSymbols::NeutralChessKnightRotatedNinetyDegrees => '🨙',
            ChessSymbols::NeutralChessPawnRotatedNinetyDegrees => '🨚',
            ChessSymbols::WhiteChessKnightRotatedOneHundredThirtyDashFiveDegrees => '🨛',
            ChessSymbols::BlackChessKnightRotatedOneHundredThirtyDashFiveDegrees => '🨜',
            ChessSymbols::NeutralChessKnightRotatedOneHundredThirtyDashFiveDegrees => '🨝',
            ChessSymbols::WhiteChessTurnedKing => '🨞',
            ChessSymbols::WhiteChessTurnedQueen => '🨟',
            ChessSymbols::WhiteChessTurnedRook => '🨠',
            ChessSymbols::WhiteChessTurnedBishop => '🨡',
            ChessSymbols::WhiteChessTurnedKnight => '🨢',
            ChessSymbols::WhiteChessTurnedPawn => '🨣',
            ChessSymbols::BlackChessTurnedKing => '🨤',
            ChessSymbols::BlackChessTurnedQueen => '🨥',
            ChessSymbols::BlackChessTurnedRook => '🨦',
            ChessSymbols::BlackChessTurnedBishop => '🨧',
            ChessSymbols::BlackChessTurnedKnight => '🨨',
            ChessSymbols::BlackChessTurnedPawn => '🨩',
            ChessSymbols::NeutralChessTurnedKing => '🨪',
            ChessSymbols::NeutralChessTurnedQueen => '🨫',
            ChessSymbols::NeutralChessTurnedRook => '🨬',
            ChessSymbols::NeutralChessTurnedBishop => '🨭',
            ChessSymbols::NeutralChessTurnedKnight => '🨮',
            ChessSymbols::NeutralChessTurnedPawn => '🨯',
            ChessSymbols::WhiteChessKnightRotatedTwoHundredTwentyDashFiveDegrees => '🨰',
            ChessSymbols::BlackChessKnightRotatedTwoHundredTwentyDashFiveDegrees => '🨱',
            ChessSymbols::NeutralChessKnightRotatedTwoHundredTwentyDashFiveDegrees => '🨲',
            ChessSymbols::WhiteChessKingRotatedTwoHundredSeventyDegrees => '🨳',
            ChessSymbols::WhiteChessQueenRotatedTwoHundredSeventyDegrees => '🨴',
            ChessSymbols::WhiteChessRookRotatedTwoHundredSeventyDegrees => '🨵',
            ChessSymbols::WhiteChessBishopRotatedTwoHundredSeventyDegrees => '🨶',
            ChessSymbols::WhiteChessKnightRotatedTwoHundredSeventyDegrees => '🨷',
            ChessSymbols::WhiteChessPawnRotatedTwoHundredSeventyDegrees => '🨸',
            ChessSymbols::BlackChessKingRotatedTwoHundredSeventyDegrees => '🨹',
            ChessSymbols::BlackChessQueenRotatedTwoHundredSeventyDegrees => '🨺',
            ChessSymbols::BlackChessRookRotatedTwoHundredSeventyDegrees => '🨻',
            ChessSymbols::BlackChessBishopRotatedTwoHundredSeventyDegrees => '🨼',
            ChessSymbols::BlackChessKnightRotatedTwoHundredSeventyDegrees => '🨽',
            ChessSymbols::BlackChessPawnRotatedTwoHundredSeventyDegrees => '🨾',
            ChessSymbols::NeutralChessKingRotatedTwoHundredSeventyDegrees => '🨿',
            ChessSymbols::NeutralChessQueenRotatedTwoHundredSeventyDegrees => '🩀',
            ChessSymbols::NeutralChessRookRotatedTwoHundredSeventyDegrees => '🩁',
            ChessSymbols::NeutralChessBishopRotatedTwoHundredSeventyDegrees => '🩂',
            ChessSymbols::NeutralChessKnightRotatedTwoHundredSeventyDegrees => '🩃',
            ChessSymbols::NeutralChessPawnRotatedTwoHundredSeventyDegrees => '🩄',
            ChessSymbols::WhiteChessKnightRotatedThreeHundredFifteenDegrees => '🩅',
            ChessSymbols::BlackChessKnightRotatedThreeHundredFifteenDegrees => '🩆',
            ChessSymbols::NeutralChessKnightRotatedThreeHundredFifteenDegrees => '🩇',
            ChessSymbols::WhiteChessEquihopper => '🩈',
            ChessSymbols::BlackChessEquihopper => '🩉',
            ChessSymbols::NeutralChessEquihopper => '🩊',
            ChessSymbols::WhiteChessEquihopperRotatedNinetyDegrees => '🩋',
            ChessSymbols::BlackChessEquihopperRotatedNinetyDegrees => '🩌',
            ChessSymbols::NeutralChessEquihopperRotatedNinetyDegrees => '🩍',
            ChessSymbols::WhiteChessKnightDashQueen => '🩎',
            ChessSymbols::WhiteChessKnightDashRook => '🩏',
            ChessSymbols::WhiteChessKnightDashBishop => '🩐',
            ChessSymbols::BlackChessKnightDashQueen => '🩑',
            ChessSymbols::BlackChessKnightDashRook => '🩒',
            ChessSymbols::BlackChessKnightDashBishop => '🩓',
            ChessSymbols::XiangqiRedGeneral => '🩠',
            ChessSymbols::XiangqiRedMandarin => '🩡',
            ChessSymbols::XiangqiRedElephant => '🩢',
            ChessSymbols::XiangqiRedHorse => '🩣',
            ChessSymbols::XiangqiRedChariot => '🩤',
            ChessSymbols::XiangqiRedCannon => '🩥',
            ChessSymbols::XiangqiRedSoldier => '🩦',
            ChessSymbols::XiangqiBlackGeneral => '🩧',
            ChessSymbols::XiangqiBlackMandarin => '🩨',
            ChessSymbols::XiangqiBlackElephant => '🩩',
            ChessSymbols::XiangqiBlackHorse => '🩪',
            ChessSymbols::XiangqiBlackChariot => '🩫',
            ChessSymbols::XiangqiBlackCannon => '🩬',
            ChessSymbols::XiangqiBlackSoldier => '🩭',
        }
    }
}

impl std::convert::TryFrom<char> for ChessSymbols {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '🨀' => Ok(ChessSymbols::NeutralChessKing),
            '🨁' => Ok(ChessSymbols::NeutralChessQueen),
            '🨂' => Ok(ChessSymbols::NeutralChessRook),
            '🨃' => Ok(ChessSymbols::NeutralChessBishop),
            '🨄' => Ok(ChessSymbols::NeutralChessKnight),
            '🨅' => Ok(ChessSymbols::NeutralChessPawn),
            '🨆' => Ok(ChessSymbols::WhiteChessKnightRotatedFortyDashFiveDegrees),
            '🨇' => Ok(ChessSymbols::BlackChessKnightRotatedFortyDashFiveDegrees),
            '🨈' => Ok(ChessSymbols::NeutralChessKnightRotatedFortyDashFiveDegrees),
            '🨉' => Ok(ChessSymbols::WhiteChessKingRotatedNinetyDegrees),
            '🨊' => Ok(ChessSymbols::WhiteChessQueenRotatedNinetyDegrees),
            '🨋' => Ok(ChessSymbols::WhiteChessRookRotatedNinetyDegrees),
            '🨌' => Ok(ChessSymbols::WhiteChessBishopRotatedNinetyDegrees),
            '🨍' => Ok(ChessSymbols::WhiteChessKnightRotatedNinetyDegrees),
            '🨎' => Ok(ChessSymbols::WhiteChessPawnRotatedNinetyDegrees),
            '🨏' => Ok(ChessSymbols::BlackChessKingRotatedNinetyDegrees),
            '🨐' => Ok(ChessSymbols::BlackChessQueenRotatedNinetyDegrees),
            '🨑' => Ok(ChessSymbols::BlackChessRookRotatedNinetyDegrees),
            '🨒' => Ok(ChessSymbols::BlackChessBishopRotatedNinetyDegrees),
            '🨓' => Ok(ChessSymbols::BlackChessKnightRotatedNinetyDegrees),
            '🨔' => Ok(ChessSymbols::BlackChessPawnRotatedNinetyDegrees),
            '🨕' => Ok(ChessSymbols::NeutralChessKingRotatedNinetyDegrees),
            '🨖' => Ok(ChessSymbols::NeutralChessQueenRotatedNinetyDegrees),
            '🨗' => Ok(ChessSymbols::NeutralChessRookRotatedNinetyDegrees),
            '🨘' => Ok(ChessSymbols::NeutralChessBishopRotatedNinetyDegrees),
            '🨙' => Ok(ChessSymbols::NeutralChessKnightRotatedNinetyDegrees),
            '🨚' => Ok(ChessSymbols::NeutralChessPawnRotatedNinetyDegrees),
            '🨛' => Ok(ChessSymbols::WhiteChessKnightRotatedOneHundredThirtyDashFiveDegrees),
            '🨜' => Ok(ChessSymbols::BlackChessKnightRotatedOneHundredThirtyDashFiveDegrees),
            '🨝' => Ok(ChessSymbols::NeutralChessKnightRotatedOneHundredThirtyDashFiveDegrees),
            '🨞' => Ok(ChessSymbols::WhiteChessTurnedKing),
            '🨟' => Ok(ChessSymbols::WhiteChessTurnedQueen),
            '🨠' => Ok(ChessSymbols::WhiteChessTurnedRook),
            '🨡' => Ok(ChessSymbols::WhiteChessTurnedBishop),
            '🨢' => Ok(ChessSymbols::WhiteChessTurnedKnight),
            '🨣' => Ok(ChessSymbols::WhiteChessTurnedPawn),
            '🨤' => Ok(ChessSymbols::BlackChessTurnedKing),
            '🨥' => Ok(ChessSymbols::BlackChessTurnedQueen),
            '🨦' => Ok(ChessSymbols::BlackChessTurnedRook),
            '🨧' => Ok(ChessSymbols::BlackChessTurnedBishop),
            '🨨' => Ok(ChessSymbols::BlackChessTurnedKnight),
            '🨩' => Ok(ChessSymbols::BlackChessTurnedPawn),
            '🨪' => Ok(ChessSymbols::NeutralChessTurnedKing),
            '🨫' => Ok(ChessSymbols::NeutralChessTurnedQueen),
            '🨬' => Ok(ChessSymbols::NeutralChessTurnedRook),
            '🨭' => Ok(ChessSymbols::NeutralChessTurnedBishop),
            '🨮' => Ok(ChessSymbols::NeutralChessTurnedKnight),
            '🨯' => Ok(ChessSymbols::NeutralChessTurnedPawn),
            '🨰' => Ok(ChessSymbols::WhiteChessKnightRotatedTwoHundredTwentyDashFiveDegrees),
            '🨱' => Ok(ChessSymbols::BlackChessKnightRotatedTwoHundredTwentyDashFiveDegrees),
            '🨲' => Ok(ChessSymbols::NeutralChessKnightRotatedTwoHundredTwentyDashFiveDegrees),
            '🨳' => Ok(ChessSymbols::WhiteChessKingRotatedTwoHundredSeventyDegrees),
            '🨴' => Ok(ChessSymbols::WhiteChessQueenRotatedTwoHundredSeventyDegrees),
            '🨵' => Ok(ChessSymbols::WhiteChessRookRotatedTwoHundredSeventyDegrees),
            '🨶' => Ok(ChessSymbols::WhiteChessBishopRotatedTwoHundredSeventyDegrees),
            '🨷' => Ok(ChessSymbols::WhiteChessKnightRotatedTwoHundredSeventyDegrees),
            '🨸' => Ok(ChessSymbols::WhiteChessPawnRotatedTwoHundredSeventyDegrees),
            '🨹' => Ok(ChessSymbols::BlackChessKingRotatedTwoHundredSeventyDegrees),
            '🨺' => Ok(ChessSymbols::BlackChessQueenRotatedTwoHundredSeventyDegrees),
            '🨻' => Ok(ChessSymbols::BlackChessRookRotatedTwoHundredSeventyDegrees),
            '🨼' => Ok(ChessSymbols::BlackChessBishopRotatedTwoHundredSeventyDegrees),
            '🨽' => Ok(ChessSymbols::BlackChessKnightRotatedTwoHundredSeventyDegrees),
            '🨾' => Ok(ChessSymbols::BlackChessPawnRotatedTwoHundredSeventyDegrees),
            '🨿' => Ok(ChessSymbols::NeutralChessKingRotatedTwoHundredSeventyDegrees),
            '🩀' => Ok(ChessSymbols::NeutralChessQueenRotatedTwoHundredSeventyDegrees),
            '🩁' => Ok(ChessSymbols::NeutralChessRookRotatedTwoHundredSeventyDegrees),
            '🩂' => Ok(ChessSymbols::NeutralChessBishopRotatedTwoHundredSeventyDegrees),
            '🩃' => Ok(ChessSymbols::NeutralChessKnightRotatedTwoHundredSeventyDegrees),
            '🩄' => Ok(ChessSymbols::NeutralChessPawnRotatedTwoHundredSeventyDegrees),
            '🩅' => Ok(ChessSymbols::WhiteChessKnightRotatedThreeHundredFifteenDegrees),
            '🩆' => Ok(ChessSymbols::BlackChessKnightRotatedThreeHundredFifteenDegrees),
            '🩇' => Ok(ChessSymbols::NeutralChessKnightRotatedThreeHundredFifteenDegrees),
            '🩈' => Ok(ChessSymbols::WhiteChessEquihopper),
            '🩉' => Ok(ChessSymbols::BlackChessEquihopper),
            '🩊' => Ok(ChessSymbols::NeutralChessEquihopper),
            '🩋' => Ok(ChessSymbols::WhiteChessEquihopperRotatedNinetyDegrees),
            '🩌' => Ok(ChessSymbols::BlackChessEquihopperRotatedNinetyDegrees),
            '🩍' => Ok(ChessSymbols::NeutralChessEquihopperRotatedNinetyDegrees),
            '🩎' => Ok(ChessSymbols::WhiteChessKnightDashQueen),
            '🩏' => Ok(ChessSymbols::WhiteChessKnightDashRook),
            '🩐' => Ok(ChessSymbols::WhiteChessKnightDashBishop),
            '🩑' => Ok(ChessSymbols::BlackChessKnightDashQueen),
            '🩒' => Ok(ChessSymbols::BlackChessKnightDashRook),
            '🩓' => Ok(ChessSymbols::BlackChessKnightDashBishop),
            '🩠' => Ok(ChessSymbols::XiangqiRedGeneral),
            '🩡' => Ok(ChessSymbols::XiangqiRedMandarin),
            '🩢' => Ok(ChessSymbols::XiangqiRedElephant),
            '🩣' => Ok(ChessSymbols::XiangqiRedHorse),
            '🩤' => Ok(ChessSymbols::XiangqiRedChariot),
            '🩥' => Ok(ChessSymbols::XiangqiRedCannon),
            '🩦' => Ok(ChessSymbols::XiangqiRedSoldier),
            '🩧' => Ok(ChessSymbols::XiangqiBlackGeneral),
            '🩨' => Ok(ChessSymbols::XiangqiBlackMandarin),
            '🩩' => Ok(ChessSymbols::XiangqiBlackElephant),
            '🩪' => Ok(ChessSymbols::XiangqiBlackHorse),
            '🩫' => Ok(ChessSymbols::XiangqiBlackChariot),
            '🩬' => Ok(ChessSymbols::XiangqiBlackCannon),
            '🩭' => Ok(ChessSymbols::XiangqiBlackSoldier),
            _ => Err(()),
        }
    }
}

impl Into<u32> for ChessSymbols {
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

impl std::convert::TryFrom<u32> for ChessSymbols {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for ChessSymbols {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl ChessSymbols {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        ChessSymbols::NeutralChessKing
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("ChessSymbols{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
