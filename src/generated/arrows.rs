/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{2190}: '←'
    pub const LEFTWARDS_ARROW: char = '←';
    /// \u{2191}: '↑'
    pub const UPWARDS_ARROW: char = '↑';
    /// \u{2192}: '→'
    pub const RIGHTWARDS_ARROW: char = '→';
    /// \u{2193}: '↓'
    pub const DOWNWARDS_ARROW: char = '↓';
    /// \u{2194}: '↔'
    pub const LEFT_RIGHT_ARROW: char = '↔';
    /// \u{2195}: '↕'
    pub const UP_DOWN_ARROW: char = '↕';
    /// \u{2196}: '↖'
    pub const NORTH_WEST_ARROW: char = '↖';
    /// \u{2197}: '↗'
    pub const NORTH_EAST_ARROW: char = '↗';
    /// \u{2198}: '↘'
    pub const SOUTH_EAST_ARROW: char = '↘';
    /// \u{2199}: '↙'
    pub const SOUTH_WEST_ARROW: char = '↙';
    /// \u{219a}: '↚'
    pub const LEFTWARDS_ARROW_WITH_STROKE: char = '↚';
    /// \u{219b}: '↛'
    pub const RIGHTWARDS_ARROW_WITH_STROKE: char = '↛';
    /// \u{219c}: '↜'
    pub const LEFTWARDS_WAVE_ARROW: char = '↜';
    /// \u{219d}: '↝'
    pub const RIGHTWARDS_WAVE_ARROW: char = '↝';
    /// \u{219e}: '↞'
    pub const LEFTWARDS_TWO_HEADED_ARROW: char = '↞';
    /// \u{219f}: '↟'
    pub const UPWARDS_TWO_HEADED_ARROW: char = '↟';
    /// \u{21a0}: '↠'
    pub const RIGHTWARDS_TWO_HEADED_ARROW: char = '↠';
    /// \u{21a1}: '↡'
    pub const DOWNWARDS_TWO_HEADED_ARROW: char = '↡';
    /// \u{21a2}: '↢'
    pub const LEFTWARDS_ARROW_WITH_TAIL: char = '↢';
    /// \u{21a3}: '↣'
    pub const RIGHTWARDS_ARROW_WITH_TAIL: char = '↣';
    /// \u{21a4}: '↤'
    pub const LEFTWARDS_ARROW_FROM_BAR: char = '↤';
    /// \u{21a5}: '↥'
    pub const UPWARDS_ARROW_FROM_BAR: char = '↥';
    /// \u{21a6}: '↦'
    pub const RIGHTWARDS_ARROW_FROM_BAR: char = '↦';
    /// \u{21a7}: '↧'
    pub const DOWNWARDS_ARROW_FROM_BAR: char = '↧';
    /// \u{21a8}: '↨'
    pub const UP_DOWN_ARROW_WITH_BASE: char = '↨';
    /// \u{21a9}: '↩'
    pub const LEFTWARDS_ARROW_WITH_HOOK: char = '↩';
    /// \u{21aa}: '↪'
    pub const RIGHTWARDS_ARROW_WITH_HOOK: char = '↪';
    /// \u{21ab}: '↫'
    pub const LEFTWARDS_ARROW_WITH_LOOP: char = '↫';
    /// \u{21ac}: '↬'
    pub const RIGHTWARDS_ARROW_WITH_LOOP: char = '↬';
    /// \u{21ad}: '↭'
    pub const LEFT_RIGHT_WAVE_ARROW: char = '↭';
    /// \u{21ae}: '↮'
    pub const LEFT_RIGHT_ARROW_WITH_STROKE: char = '↮';
    /// \u{21af}: '↯'
    pub const DOWNWARDS_ZIGZAG_ARROW: char = '↯';
    /// \u{21b0}: '↰'
    pub const UPWARDS_ARROW_WITH_TIP_LEFTWARDS: char = '↰';
    /// \u{21b1}: '↱'
    pub const UPWARDS_ARROW_WITH_TIP_RIGHTWARDS: char = '↱';
    /// \u{21b2}: '↲'
    pub const DOWNWARDS_ARROW_WITH_TIP_LEFTWARDS: char = '↲';
    /// \u{21b3}: '↳'
    pub const DOWNWARDS_ARROW_WITH_TIP_RIGHTWARDS: char = '↳';
    /// \u{21b4}: '↴'
    pub const RIGHTWARDS_ARROW_WITH_CORNER_DOWNWARDS: char = '↴';
    /// \u{21b5}: '↵'
    pub const DOWNWARDS_ARROW_WITH_CORNER_LEFTWARDS: char = '↵';
    /// \u{21b6}: '↶'
    pub const ANTICLOCKWISE_TOP_SEMICIRCLE_ARROW: char = '↶';
    /// \u{21b7}: '↷'
    pub const CLOCKWISE_TOP_SEMICIRCLE_ARROW: char = '↷';
    /// \u{21b8}: '↸'
    pub const NORTH_WEST_ARROW_TO_LONG_BAR: char = '↸';
    /// \u{21b9}: '↹'
    pub const LEFTWARDS_ARROW_TO_BAR_OVER_RIGHTWARDS_ARROW_TO_BAR: char = '↹';
    /// \u{21ba}: '↺'
    pub const ANTICLOCKWISE_OPEN_CIRCLE_ARROW: char = '↺';
    /// \u{21bb}: '↻'
    pub const CLOCKWISE_OPEN_CIRCLE_ARROW: char = '↻';
    /// \u{21bc}: '↼'
    pub const LEFTWARDS_HARPOON_WITH_BARB_UPWARDS: char = '↼';
    /// \u{21bd}: '↽'
    pub const LEFTWARDS_HARPOON_WITH_BARB_DOWNWARDS: char = '↽';
    /// \u{21be}: '↾'
    pub const UPWARDS_HARPOON_WITH_BARB_RIGHTWARDS: char = '↾';
    /// \u{21bf}: '↿'
    pub const UPWARDS_HARPOON_WITH_BARB_LEFTWARDS: char = '↿';
    /// \u{21c0}: '⇀'
    pub const RIGHTWARDS_HARPOON_WITH_BARB_UPWARDS: char = '⇀';
    /// \u{21c1}: '⇁'
    pub const RIGHTWARDS_HARPOON_WITH_BARB_DOWNWARDS: char = '⇁';
    /// \u{21c2}: '⇂'
    pub const DOWNWARDS_HARPOON_WITH_BARB_RIGHTWARDS: char = '⇂';
    /// \u{21c3}: '⇃'
    pub const DOWNWARDS_HARPOON_WITH_BARB_LEFTWARDS: char = '⇃';
    /// \u{21c4}: '⇄'
    pub const RIGHTWARDS_ARROW_OVER_LEFTWARDS_ARROW: char = '⇄';
    /// \u{21c5}: '⇅'
    pub const UPWARDS_ARROW_LEFTWARDS_OF_DOWNWARDS_ARROW: char = '⇅';
    /// \u{21c6}: '⇆'
    pub const LEFTWARDS_ARROW_OVER_RIGHTWARDS_ARROW: char = '⇆';
    /// \u{21c7}: '⇇'
    pub const LEFTWARDS_PAIRED_ARROWS: char = '⇇';
    /// \u{21c8}: '⇈'
    pub const UPWARDS_PAIRED_ARROWS: char = '⇈';
    /// \u{21c9}: '⇉'
    pub const RIGHTWARDS_PAIRED_ARROWS: char = '⇉';
    /// \u{21ca}: '⇊'
    pub const DOWNWARDS_PAIRED_ARROWS: char = '⇊';
    /// \u{21cb}: '⇋'
    pub const LEFTWARDS_HARPOON_OVER_RIGHTWARDS_HARPOON: char = '⇋';
    /// \u{21cc}: '⇌'
    pub const RIGHTWARDS_HARPOON_OVER_LEFTWARDS_HARPOON: char = '⇌';
    /// \u{21cd}: '⇍'
    pub const LEFTWARDS_DOUBLE_ARROW_WITH_STROKE: char = '⇍';
    /// \u{21ce}: '⇎'
    pub const LEFT_RIGHT_DOUBLE_ARROW_WITH_STROKE: char = '⇎';
    /// \u{21cf}: '⇏'
    pub const RIGHTWARDS_DOUBLE_ARROW_WITH_STROKE: char = '⇏';
    /// \u{21d0}: '⇐'
    pub const LEFTWARDS_DOUBLE_ARROW: char = '⇐';
    /// \u{21d1}: '⇑'
    pub const UPWARDS_DOUBLE_ARROW: char = '⇑';
    /// \u{21d2}: '⇒'
    pub const RIGHTWARDS_DOUBLE_ARROW: char = '⇒';
    /// \u{21d3}: '⇓'
    pub const DOWNWARDS_DOUBLE_ARROW: char = '⇓';
    /// \u{21d4}: '⇔'
    pub const LEFT_RIGHT_DOUBLE_ARROW: char = '⇔';
    /// \u{21d5}: '⇕'
    pub const UP_DOWN_DOUBLE_ARROW: char = '⇕';
    /// \u{21d6}: '⇖'
    pub const NORTH_WEST_DOUBLE_ARROW: char = '⇖';
    /// \u{21d7}: '⇗'
    pub const NORTH_EAST_DOUBLE_ARROW: char = '⇗';
    /// \u{21d8}: '⇘'
    pub const SOUTH_EAST_DOUBLE_ARROW: char = '⇘';
    /// \u{21d9}: '⇙'
    pub const SOUTH_WEST_DOUBLE_ARROW: char = '⇙';
    /// \u{21da}: '⇚'
    pub const LEFTWARDS_TRIPLE_ARROW: char = '⇚';
    /// \u{21db}: '⇛'
    pub const RIGHTWARDS_TRIPLE_ARROW: char = '⇛';
    /// \u{21dc}: '⇜'
    pub const LEFTWARDS_SQUIGGLE_ARROW: char = '⇜';
    /// \u{21dd}: '⇝'
    pub const RIGHTWARDS_SQUIGGLE_ARROW: char = '⇝';
    /// \u{21de}: '⇞'
    pub const UPWARDS_ARROW_WITH_DOUBLE_STROKE: char = '⇞';
    /// \u{21df}: '⇟'
    pub const DOWNWARDS_ARROW_WITH_DOUBLE_STROKE: char = '⇟';
    /// \u{21e0}: '⇠'
    pub const LEFTWARDS_DASHED_ARROW: char = '⇠';
    /// \u{21e1}: '⇡'
    pub const UPWARDS_DASHED_ARROW: char = '⇡';
    /// \u{21e2}: '⇢'
    pub const RIGHTWARDS_DASHED_ARROW: char = '⇢';
    /// \u{21e3}: '⇣'
    pub const DOWNWARDS_DASHED_ARROW: char = '⇣';
    /// \u{21e4}: '⇤'
    pub const LEFTWARDS_ARROW_TO_BAR: char = '⇤';
    /// \u{21e5}: '⇥'
    pub const RIGHTWARDS_ARROW_TO_BAR: char = '⇥';
    /// \u{21e6}: '⇦'
    pub const LEFTWARDS_WHITE_ARROW: char = '⇦';
    /// \u{21e7}: '⇧'
    pub const UPWARDS_WHITE_ARROW: char = '⇧';
    /// \u{21e8}: '⇨'
    pub const RIGHTWARDS_WHITE_ARROW: char = '⇨';
    /// \u{21e9}: '⇩'
    pub const DOWNWARDS_WHITE_ARROW: char = '⇩';
    /// \u{21ea}: '⇪'
    pub const UPWARDS_WHITE_ARROW_FROM_BAR: char = '⇪';
    /// \u{21eb}: '⇫'
    pub const UPWARDS_WHITE_ARROW_ON_PEDESTAL: char = '⇫';
    /// \u{21ec}: '⇬'
    pub const UPWARDS_WHITE_ARROW_ON_PEDESTAL_WITH_HORIZONTAL_BAR: char = '⇬';
    /// \u{21ed}: '⇭'
    pub const UPWARDS_WHITE_ARROW_ON_PEDESTAL_WITH_VERTICAL_BAR: char = '⇭';
    /// \u{21ee}: '⇮'
    pub const UPWARDS_WHITE_DOUBLE_ARROW: char = '⇮';
    /// \u{21ef}: '⇯'
    pub const UPWARDS_WHITE_DOUBLE_ARROW_ON_PEDESTAL: char = '⇯';
    /// \u{21f0}: '⇰'
    pub const RIGHTWARDS_WHITE_ARROW_FROM_WALL: char = '⇰';
    /// \u{21f1}: '⇱'
    pub const NORTH_WEST_ARROW_TO_CORNER: char = '⇱';
    /// \u{21f2}: '⇲'
    pub const SOUTH_EAST_ARROW_TO_CORNER: char = '⇲';
    /// \u{21f3}: '⇳'
    pub const UP_DOWN_WHITE_ARROW: char = '⇳';
    /// \u{21f4}: '⇴'
    pub const RIGHT_ARROW_WITH_SMALL_CIRCLE: char = '⇴';
    /// \u{21f5}: '⇵'
    pub const DOWNWARDS_ARROW_LEFTWARDS_OF_UPWARDS_ARROW: char = '⇵';
    /// \u{21f6}: '⇶'
    pub const THREE_RIGHTWARDS_ARROWS: char = '⇶';
    /// \u{21f7}: '⇷'
    pub const LEFTWARDS_ARROW_WITH_VERTICAL_STROKE: char = '⇷';
    /// \u{21f8}: '⇸'
    pub const RIGHTWARDS_ARROW_WITH_VERTICAL_STROKE: char = '⇸';
    /// \u{21f9}: '⇹'
    pub const LEFT_RIGHT_ARROW_WITH_VERTICAL_STROKE: char = '⇹';
    /// \u{21fa}: '⇺'
    pub const LEFTWARDS_ARROW_WITH_DOUBLE_VERTICAL_STROKE: char = '⇺';
    /// \u{21fb}: '⇻'
    pub const RIGHTWARDS_ARROW_WITH_DOUBLE_VERTICAL_STROKE: char = '⇻';
    /// \u{21fc}: '⇼'
    pub const LEFT_RIGHT_ARROW_WITH_DOUBLE_VERTICAL_STROKE: char = '⇼';
    /// \u{21fd}: '⇽'
    pub const LEFTWARDS_OPEN_DASH_HEADED_ARROW: char = '⇽';
    /// \u{21fe}: '⇾'
    pub const RIGHTWARDS_OPEN_DASH_HEADED_ARROW: char = '⇾';
}

/// An enum to represent all characters in the Arrows block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Arrows {
    /// \u{2190}: '←'
    LeftwardsArrow,
    /// \u{2191}: '↑'
    UpwardsArrow,
    /// \u{2192}: '→'
    RightwardsArrow,
    /// \u{2193}: '↓'
    DownwardsArrow,
    /// \u{2194}: '↔'
    LeftRightArrow,
    /// \u{2195}: '↕'
    UpDownArrow,
    /// \u{2196}: '↖'
    NorthWestArrow,
    /// \u{2197}: '↗'
    NorthEastArrow,
    /// \u{2198}: '↘'
    SouthEastArrow,
    /// \u{2199}: '↙'
    SouthWestArrow,
    /// \u{219a}: '↚'
    LeftwardsArrowWithStroke,
    /// \u{219b}: '↛'
    RightwardsArrowWithStroke,
    /// \u{219c}: '↜'
    LeftwardsWaveArrow,
    /// \u{219d}: '↝'
    RightwardsWaveArrow,
    /// \u{219e}: '↞'
    LeftwardsTwoHeadedArrow,
    /// \u{219f}: '↟'
    UpwardsTwoHeadedArrow,
    /// \u{21a0}: '↠'
    RightwardsTwoHeadedArrow,
    /// \u{21a1}: '↡'
    DownwardsTwoHeadedArrow,
    /// \u{21a2}: '↢'
    LeftwardsArrowWithTail,
    /// \u{21a3}: '↣'
    RightwardsArrowWithTail,
    /// \u{21a4}: '↤'
    LeftwardsArrowFromBar,
    /// \u{21a5}: '↥'
    UpwardsArrowFromBar,
    /// \u{21a6}: '↦'
    RightwardsArrowFromBar,
    /// \u{21a7}: '↧'
    DownwardsArrowFromBar,
    /// \u{21a8}: '↨'
    UpDownArrowWithBase,
    /// \u{21a9}: '↩'
    LeftwardsArrowWithHook,
    /// \u{21aa}: '↪'
    RightwardsArrowWithHook,
    /// \u{21ab}: '↫'
    LeftwardsArrowWithLoop,
    /// \u{21ac}: '↬'
    RightwardsArrowWithLoop,
    /// \u{21ad}: '↭'
    LeftRightWaveArrow,
    /// \u{21ae}: '↮'
    LeftRightArrowWithStroke,
    /// \u{21af}: '↯'
    DownwardsZigzagArrow,
    /// \u{21b0}: '↰'
    UpwardsArrowWithTipLeftwards,
    /// \u{21b1}: '↱'
    UpwardsArrowWithTipRightwards,
    /// \u{21b2}: '↲'
    DownwardsArrowWithTipLeftwards,
    /// \u{21b3}: '↳'
    DownwardsArrowWithTipRightwards,
    /// \u{21b4}: '↴'
    RightwardsArrowWithCornerDownwards,
    /// \u{21b5}: '↵'
    DownwardsArrowWithCornerLeftwards,
    /// \u{21b6}: '↶'
    AnticlockwiseTopSemicircleArrow,
    /// \u{21b7}: '↷'
    ClockwiseTopSemicircleArrow,
    /// \u{21b8}: '↸'
    NorthWestArrowToLongBar,
    /// \u{21b9}: '↹'
    LeftwardsArrowToBarOverRightwardsArrowToBar,
    /// \u{21ba}: '↺'
    AnticlockwiseOpenCircleArrow,
    /// \u{21bb}: '↻'
    ClockwiseOpenCircleArrow,
    /// \u{21bc}: '↼'
    LeftwardsHarpoonWithBarbUpwards,
    /// \u{21bd}: '↽'
    LeftwardsHarpoonWithBarbDownwards,
    /// \u{21be}: '↾'
    UpwardsHarpoonWithBarbRightwards,
    /// \u{21bf}: '↿'
    UpwardsHarpoonWithBarbLeftwards,
    /// \u{21c0}: '⇀'
    RightwardsHarpoonWithBarbUpwards,
    /// \u{21c1}: '⇁'
    RightwardsHarpoonWithBarbDownwards,
    /// \u{21c2}: '⇂'
    DownwardsHarpoonWithBarbRightwards,
    /// \u{21c3}: '⇃'
    DownwardsHarpoonWithBarbLeftwards,
    /// \u{21c4}: '⇄'
    RightwardsArrowOverLeftwardsArrow,
    /// \u{21c5}: '⇅'
    UpwardsArrowLeftwardsOfDownwardsArrow,
    /// \u{21c6}: '⇆'
    LeftwardsArrowOverRightwardsArrow,
    /// \u{21c7}: '⇇'
    LeftwardsPaired,
    /// \u{21c8}: '⇈'
    UpwardsPaired,
    /// \u{21c9}: '⇉'
    RightwardsPaired,
    /// \u{21ca}: '⇊'
    DownwardsPaired,
    /// \u{21cb}: '⇋'
    LeftwardsHarpoonOverRightwardsHarpoon,
    /// \u{21cc}: '⇌'
    RightwardsHarpoonOverLeftwardsHarpoon,
    /// \u{21cd}: '⇍'
    LeftwardsDoubleArrowWithStroke,
    /// \u{21ce}: '⇎'
    LeftRightDoubleArrowWithStroke,
    /// \u{21cf}: '⇏'
    RightwardsDoubleArrowWithStroke,
    /// \u{21d0}: '⇐'
    LeftwardsDoubleArrow,
    /// \u{21d1}: '⇑'
    UpwardsDoubleArrow,
    /// \u{21d2}: '⇒'
    RightwardsDoubleArrow,
    /// \u{21d3}: '⇓'
    DownwardsDoubleArrow,
    /// \u{21d4}: '⇔'
    LeftRightDoubleArrow,
    /// \u{21d5}: '⇕'
    UpDownDoubleArrow,
    /// \u{21d6}: '⇖'
    NorthWestDoubleArrow,
    /// \u{21d7}: '⇗'
    NorthEastDoubleArrow,
    /// \u{21d8}: '⇘'
    SouthEastDoubleArrow,
    /// \u{21d9}: '⇙'
    SouthWestDoubleArrow,
    /// \u{21da}: '⇚'
    LeftwardsTripleArrow,
    /// \u{21db}: '⇛'
    RightwardsTripleArrow,
    /// \u{21dc}: '⇜'
    LeftwardsSquiggleArrow,
    /// \u{21dd}: '⇝'
    RightwardsSquiggleArrow,
    /// \u{21de}: '⇞'
    UpwardsArrowWithDoubleStroke,
    /// \u{21df}: '⇟'
    DownwardsArrowWithDoubleStroke,
    /// \u{21e0}: '⇠'
    LeftwardsDashedArrow,
    /// \u{21e1}: '⇡'
    UpwardsDashedArrow,
    /// \u{21e2}: '⇢'
    RightwardsDashedArrow,
    /// \u{21e3}: '⇣'
    DownwardsDashedArrow,
    /// \u{21e4}: '⇤'
    LeftwardsArrowToBar,
    /// \u{21e5}: '⇥'
    RightwardsArrowToBar,
    /// \u{21e6}: '⇦'
    LeftwardsWhiteArrow,
    /// \u{21e7}: '⇧'
    UpwardsWhiteArrow,
    /// \u{21e8}: '⇨'
    RightwardsWhiteArrow,
    /// \u{21e9}: '⇩'
    DownwardsWhiteArrow,
    /// \u{21ea}: '⇪'
    UpwardsWhiteArrowFromBar,
    /// \u{21eb}: '⇫'
    UpwardsWhiteArrowOnPedestal,
    /// \u{21ec}: '⇬'
    UpwardsWhiteArrowOnPedestalWithHorizontalBar,
    /// \u{21ed}: '⇭'
    UpwardsWhiteArrowOnPedestalWithVerticalBar,
    /// \u{21ee}: '⇮'
    UpwardsWhiteDoubleArrow,
    /// \u{21ef}: '⇯'
    UpwardsWhiteDoubleArrowOnPedestal,
    /// \u{21f0}: '⇰'
    RightwardsWhiteArrowFromWall,
    /// \u{21f1}: '⇱'
    NorthWestArrowToCorner,
    /// \u{21f2}: '⇲'
    SouthEastArrowToCorner,
    /// \u{21f3}: '⇳'
    UpDownWhiteArrow,
    /// \u{21f4}: '⇴'
    RightArrowWithSmallCircle,
    /// \u{21f5}: '⇵'
    DownwardsArrowLeftwardsOfUpwardsArrow,
    /// \u{21f6}: '⇶'
    ThreeRightwards,
    /// \u{21f7}: '⇷'
    LeftwardsArrowWithVerticalStroke,
    /// \u{21f8}: '⇸'
    RightwardsArrowWithVerticalStroke,
    /// \u{21f9}: '⇹'
    LeftRightArrowWithVerticalStroke,
    /// \u{21fa}: '⇺'
    LeftwardsArrowWithDoubleVerticalStroke,
    /// \u{21fb}: '⇻'
    RightwardsArrowWithDoubleVerticalStroke,
    /// \u{21fc}: '⇼'
    LeftRightArrowWithDoubleVerticalStroke,
    /// \u{21fd}: '⇽'
    LeftwardsOpenDashHeadedArrow,
    /// \u{21fe}: '⇾'
    RightwardsOpenDashHeadedArrow,
}

impl Into<char> for Arrows {
    fn into(self) -> char {
        use constants::*;
        match self {
            Arrows::LeftwardsArrow => LEFTWARDS_ARROW,
            Arrows::UpwardsArrow => UPWARDS_ARROW,
            Arrows::RightwardsArrow => RIGHTWARDS_ARROW,
            Arrows::DownwardsArrow => DOWNWARDS_ARROW,
            Arrows::LeftRightArrow => LEFT_RIGHT_ARROW,
            Arrows::UpDownArrow => UP_DOWN_ARROW,
            Arrows::NorthWestArrow => NORTH_WEST_ARROW,
            Arrows::NorthEastArrow => NORTH_EAST_ARROW,
            Arrows::SouthEastArrow => SOUTH_EAST_ARROW,
            Arrows::SouthWestArrow => SOUTH_WEST_ARROW,
            Arrows::LeftwardsArrowWithStroke => LEFTWARDS_ARROW_WITH_STROKE,
            Arrows::RightwardsArrowWithStroke => RIGHTWARDS_ARROW_WITH_STROKE,
            Arrows::LeftwardsWaveArrow => LEFTWARDS_WAVE_ARROW,
            Arrows::RightwardsWaveArrow => RIGHTWARDS_WAVE_ARROW,
            Arrows::LeftwardsTwoHeadedArrow => LEFTWARDS_TWO_HEADED_ARROW,
            Arrows::UpwardsTwoHeadedArrow => UPWARDS_TWO_HEADED_ARROW,
            Arrows::RightwardsTwoHeadedArrow => RIGHTWARDS_TWO_HEADED_ARROW,
            Arrows::DownwardsTwoHeadedArrow => DOWNWARDS_TWO_HEADED_ARROW,
            Arrows::LeftwardsArrowWithTail => LEFTWARDS_ARROW_WITH_TAIL,
            Arrows::RightwardsArrowWithTail => RIGHTWARDS_ARROW_WITH_TAIL,
            Arrows::LeftwardsArrowFromBar => LEFTWARDS_ARROW_FROM_BAR,
            Arrows::UpwardsArrowFromBar => UPWARDS_ARROW_FROM_BAR,
            Arrows::RightwardsArrowFromBar => RIGHTWARDS_ARROW_FROM_BAR,
            Arrows::DownwardsArrowFromBar => DOWNWARDS_ARROW_FROM_BAR,
            Arrows::UpDownArrowWithBase => UP_DOWN_ARROW_WITH_BASE,
            Arrows::LeftwardsArrowWithHook => LEFTWARDS_ARROW_WITH_HOOK,
            Arrows::RightwardsArrowWithHook => RIGHTWARDS_ARROW_WITH_HOOK,
            Arrows::LeftwardsArrowWithLoop => LEFTWARDS_ARROW_WITH_LOOP,
            Arrows::RightwardsArrowWithLoop => RIGHTWARDS_ARROW_WITH_LOOP,
            Arrows::LeftRightWaveArrow => LEFT_RIGHT_WAVE_ARROW,
            Arrows::LeftRightArrowWithStroke => LEFT_RIGHT_ARROW_WITH_STROKE,
            Arrows::DownwardsZigzagArrow => DOWNWARDS_ZIGZAG_ARROW,
            Arrows::UpwardsArrowWithTipLeftwards => UPWARDS_ARROW_WITH_TIP_LEFTWARDS,
            Arrows::UpwardsArrowWithTipRightwards => UPWARDS_ARROW_WITH_TIP_RIGHTWARDS,
            Arrows::DownwardsArrowWithTipLeftwards => DOWNWARDS_ARROW_WITH_TIP_LEFTWARDS,
            Arrows::DownwardsArrowWithTipRightwards => DOWNWARDS_ARROW_WITH_TIP_RIGHTWARDS,
            Arrows::RightwardsArrowWithCornerDownwards => RIGHTWARDS_ARROW_WITH_CORNER_DOWNWARDS,
            Arrows::DownwardsArrowWithCornerLeftwards => DOWNWARDS_ARROW_WITH_CORNER_LEFTWARDS,
            Arrows::AnticlockwiseTopSemicircleArrow => ANTICLOCKWISE_TOP_SEMICIRCLE_ARROW,
            Arrows::ClockwiseTopSemicircleArrow => CLOCKWISE_TOP_SEMICIRCLE_ARROW,
            Arrows::NorthWestArrowToLongBar => NORTH_WEST_ARROW_TO_LONG_BAR,
            Arrows::LeftwardsArrowToBarOverRightwardsArrowToBar => LEFTWARDS_ARROW_TO_BAR_OVER_RIGHTWARDS_ARROW_TO_BAR,
            Arrows::AnticlockwiseOpenCircleArrow => ANTICLOCKWISE_OPEN_CIRCLE_ARROW,
            Arrows::ClockwiseOpenCircleArrow => CLOCKWISE_OPEN_CIRCLE_ARROW,
            Arrows::LeftwardsHarpoonWithBarbUpwards => LEFTWARDS_HARPOON_WITH_BARB_UPWARDS,
            Arrows::LeftwardsHarpoonWithBarbDownwards => LEFTWARDS_HARPOON_WITH_BARB_DOWNWARDS,
            Arrows::UpwardsHarpoonWithBarbRightwards => UPWARDS_HARPOON_WITH_BARB_RIGHTWARDS,
            Arrows::UpwardsHarpoonWithBarbLeftwards => UPWARDS_HARPOON_WITH_BARB_LEFTWARDS,
            Arrows::RightwardsHarpoonWithBarbUpwards => RIGHTWARDS_HARPOON_WITH_BARB_UPWARDS,
            Arrows::RightwardsHarpoonWithBarbDownwards => RIGHTWARDS_HARPOON_WITH_BARB_DOWNWARDS,
            Arrows::DownwardsHarpoonWithBarbRightwards => DOWNWARDS_HARPOON_WITH_BARB_RIGHTWARDS,
            Arrows::DownwardsHarpoonWithBarbLeftwards => DOWNWARDS_HARPOON_WITH_BARB_LEFTWARDS,
            Arrows::RightwardsArrowOverLeftwardsArrow => RIGHTWARDS_ARROW_OVER_LEFTWARDS_ARROW,
            Arrows::UpwardsArrowLeftwardsOfDownwardsArrow => UPWARDS_ARROW_LEFTWARDS_OF_DOWNWARDS_ARROW,
            Arrows::LeftwardsArrowOverRightwardsArrow => LEFTWARDS_ARROW_OVER_RIGHTWARDS_ARROW,
            Arrows::LeftwardsPaired => LEFTWARDS_PAIRED_ARROWS,
            Arrows::UpwardsPaired => UPWARDS_PAIRED_ARROWS,
            Arrows::RightwardsPaired => RIGHTWARDS_PAIRED_ARROWS,
            Arrows::DownwardsPaired => DOWNWARDS_PAIRED_ARROWS,
            Arrows::LeftwardsHarpoonOverRightwardsHarpoon => LEFTWARDS_HARPOON_OVER_RIGHTWARDS_HARPOON,
            Arrows::RightwardsHarpoonOverLeftwardsHarpoon => RIGHTWARDS_HARPOON_OVER_LEFTWARDS_HARPOON,
            Arrows::LeftwardsDoubleArrowWithStroke => LEFTWARDS_DOUBLE_ARROW_WITH_STROKE,
            Arrows::LeftRightDoubleArrowWithStroke => LEFT_RIGHT_DOUBLE_ARROW_WITH_STROKE,
            Arrows::RightwardsDoubleArrowWithStroke => RIGHTWARDS_DOUBLE_ARROW_WITH_STROKE,
            Arrows::LeftwardsDoubleArrow => LEFTWARDS_DOUBLE_ARROW,
            Arrows::UpwardsDoubleArrow => UPWARDS_DOUBLE_ARROW,
            Arrows::RightwardsDoubleArrow => RIGHTWARDS_DOUBLE_ARROW,
            Arrows::DownwardsDoubleArrow => DOWNWARDS_DOUBLE_ARROW,
            Arrows::LeftRightDoubleArrow => LEFT_RIGHT_DOUBLE_ARROW,
            Arrows::UpDownDoubleArrow => UP_DOWN_DOUBLE_ARROW,
            Arrows::NorthWestDoubleArrow => NORTH_WEST_DOUBLE_ARROW,
            Arrows::NorthEastDoubleArrow => NORTH_EAST_DOUBLE_ARROW,
            Arrows::SouthEastDoubleArrow => SOUTH_EAST_DOUBLE_ARROW,
            Arrows::SouthWestDoubleArrow => SOUTH_WEST_DOUBLE_ARROW,
            Arrows::LeftwardsTripleArrow => LEFTWARDS_TRIPLE_ARROW,
            Arrows::RightwardsTripleArrow => RIGHTWARDS_TRIPLE_ARROW,
            Arrows::LeftwardsSquiggleArrow => LEFTWARDS_SQUIGGLE_ARROW,
            Arrows::RightwardsSquiggleArrow => RIGHTWARDS_SQUIGGLE_ARROW,
            Arrows::UpwardsArrowWithDoubleStroke => UPWARDS_ARROW_WITH_DOUBLE_STROKE,
            Arrows::DownwardsArrowWithDoubleStroke => DOWNWARDS_ARROW_WITH_DOUBLE_STROKE,
            Arrows::LeftwardsDashedArrow => LEFTWARDS_DASHED_ARROW,
            Arrows::UpwardsDashedArrow => UPWARDS_DASHED_ARROW,
            Arrows::RightwardsDashedArrow => RIGHTWARDS_DASHED_ARROW,
            Arrows::DownwardsDashedArrow => DOWNWARDS_DASHED_ARROW,
            Arrows::LeftwardsArrowToBar => LEFTWARDS_ARROW_TO_BAR,
            Arrows::RightwardsArrowToBar => RIGHTWARDS_ARROW_TO_BAR,
            Arrows::LeftwardsWhiteArrow => LEFTWARDS_WHITE_ARROW,
            Arrows::UpwardsWhiteArrow => UPWARDS_WHITE_ARROW,
            Arrows::RightwardsWhiteArrow => RIGHTWARDS_WHITE_ARROW,
            Arrows::DownwardsWhiteArrow => DOWNWARDS_WHITE_ARROW,
            Arrows::UpwardsWhiteArrowFromBar => UPWARDS_WHITE_ARROW_FROM_BAR,
            Arrows::UpwardsWhiteArrowOnPedestal => UPWARDS_WHITE_ARROW_ON_PEDESTAL,
            Arrows::UpwardsWhiteArrowOnPedestalWithHorizontalBar => UPWARDS_WHITE_ARROW_ON_PEDESTAL_WITH_HORIZONTAL_BAR,
            Arrows::UpwardsWhiteArrowOnPedestalWithVerticalBar => UPWARDS_WHITE_ARROW_ON_PEDESTAL_WITH_VERTICAL_BAR,
            Arrows::UpwardsWhiteDoubleArrow => UPWARDS_WHITE_DOUBLE_ARROW,
            Arrows::UpwardsWhiteDoubleArrowOnPedestal => UPWARDS_WHITE_DOUBLE_ARROW_ON_PEDESTAL,
            Arrows::RightwardsWhiteArrowFromWall => RIGHTWARDS_WHITE_ARROW_FROM_WALL,
            Arrows::NorthWestArrowToCorner => NORTH_WEST_ARROW_TO_CORNER,
            Arrows::SouthEastArrowToCorner => SOUTH_EAST_ARROW_TO_CORNER,
            Arrows::UpDownWhiteArrow => UP_DOWN_WHITE_ARROW,
            Arrows::RightArrowWithSmallCircle => RIGHT_ARROW_WITH_SMALL_CIRCLE,
            Arrows::DownwardsArrowLeftwardsOfUpwardsArrow => DOWNWARDS_ARROW_LEFTWARDS_OF_UPWARDS_ARROW,
            Arrows::ThreeRightwards => THREE_RIGHTWARDS_ARROWS,
            Arrows::LeftwardsArrowWithVerticalStroke => LEFTWARDS_ARROW_WITH_VERTICAL_STROKE,
            Arrows::RightwardsArrowWithVerticalStroke => RIGHTWARDS_ARROW_WITH_VERTICAL_STROKE,
            Arrows::LeftRightArrowWithVerticalStroke => LEFT_RIGHT_ARROW_WITH_VERTICAL_STROKE,
            Arrows::LeftwardsArrowWithDoubleVerticalStroke => LEFTWARDS_ARROW_WITH_DOUBLE_VERTICAL_STROKE,
            Arrows::RightwardsArrowWithDoubleVerticalStroke => RIGHTWARDS_ARROW_WITH_DOUBLE_VERTICAL_STROKE,
            Arrows::LeftRightArrowWithDoubleVerticalStroke => LEFT_RIGHT_ARROW_WITH_DOUBLE_VERTICAL_STROKE,
            Arrows::LeftwardsOpenDashHeadedArrow => LEFTWARDS_OPEN_DASH_HEADED_ARROW,
            Arrows::RightwardsOpenDashHeadedArrow => RIGHTWARDS_OPEN_DASH_HEADED_ARROW,
        }
    }
}

impl std::convert::TryFrom<char> for Arrows {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LEFTWARDS_ARROW => Ok(Arrows::LeftwardsArrow),
            UPWARDS_ARROW => Ok(Arrows::UpwardsArrow),
            RIGHTWARDS_ARROW => Ok(Arrows::RightwardsArrow),
            DOWNWARDS_ARROW => Ok(Arrows::DownwardsArrow),
            LEFT_RIGHT_ARROW => Ok(Arrows::LeftRightArrow),
            UP_DOWN_ARROW => Ok(Arrows::UpDownArrow),
            NORTH_WEST_ARROW => Ok(Arrows::NorthWestArrow),
            NORTH_EAST_ARROW => Ok(Arrows::NorthEastArrow),
            SOUTH_EAST_ARROW => Ok(Arrows::SouthEastArrow),
            SOUTH_WEST_ARROW => Ok(Arrows::SouthWestArrow),
            LEFTWARDS_ARROW_WITH_STROKE => Ok(Arrows::LeftwardsArrowWithStroke),
            RIGHTWARDS_ARROW_WITH_STROKE => Ok(Arrows::RightwardsArrowWithStroke),
            LEFTWARDS_WAVE_ARROW => Ok(Arrows::LeftwardsWaveArrow),
            RIGHTWARDS_WAVE_ARROW => Ok(Arrows::RightwardsWaveArrow),
            LEFTWARDS_TWO_HEADED_ARROW => Ok(Arrows::LeftwardsTwoHeadedArrow),
            UPWARDS_TWO_HEADED_ARROW => Ok(Arrows::UpwardsTwoHeadedArrow),
            RIGHTWARDS_TWO_HEADED_ARROW => Ok(Arrows::RightwardsTwoHeadedArrow),
            DOWNWARDS_TWO_HEADED_ARROW => Ok(Arrows::DownwardsTwoHeadedArrow),
            LEFTWARDS_ARROW_WITH_TAIL => Ok(Arrows::LeftwardsArrowWithTail),
            RIGHTWARDS_ARROW_WITH_TAIL => Ok(Arrows::RightwardsArrowWithTail),
            LEFTWARDS_ARROW_FROM_BAR => Ok(Arrows::LeftwardsArrowFromBar),
            UPWARDS_ARROW_FROM_BAR => Ok(Arrows::UpwardsArrowFromBar),
            RIGHTWARDS_ARROW_FROM_BAR => Ok(Arrows::RightwardsArrowFromBar),
            DOWNWARDS_ARROW_FROM_BAR => Ok(Arrows::DownwardsArrowFromBar),
            UP_DOWN_ARROW_WITH_BASE => Ok(Arrows::UpDownArrowWithBase),
            LEFTWARDS_ARROW_WITH_HOOK => Ok(Arrows::LeftwardsArrowWithHook),
            RIGHTWARDS_ARROW_WITH_HOOK => Ok(Arrows::RightwardsArrowWithHook),
            LEFTWARDS_ARROW_WITH_LOOP => Ok(Arrows::LeftwardsArrowWithLoop),
            RIGHTWARDS_ARROW_WITH_LOOP => Ok(Arrows::RightwardsArrowWithLoop),
            LEFT_RIGHT_WAVE_ARROW => Ok(Arrows::LeftRightWaveArrow),
            LEFT_RIGHT_ARROW_WITH_STROKE => Ok(Arrows::LeftRightArrowWithStroke),
            DOWNWARDS_ZIGZAG_ARROW => Ok(Arrows::DownwardsZigzagArrow),
            UPWARDS_ARROW_WITH_TIP_LEFTWARDS => Ok(Arrows::UpwardsArrowWithTipLeftwards),
            UPWARDS_ARROW_WITH_TIP_RIGHTWARDS => Ok(Arrows::UpwardsArrowWithTipRightwards),
            DOWNWARDS_ARROW_WITH_TIP_LEFTWARDS => Ok(Arrows::DownwardsArrowWithTipLeftwards),
            DOWNWARDS_ARROW_WITH_TIP_RIGHTWARDS => Ok(Arrows::DownwardsArrowWithTipRightwards),
            RIGHTWARDS_ARROW_WITH_CORNER_DOWNWARDS => Ok(Arrows::RightwardsArrowWithCornerDownwards),
            DOWNWARDS_ARROW_WITH_CORNER_LEFTWARDS => Ok(Arrows::DownwardsArrowWithCornerLeftwards),
            ANTICLOCKWISE_TOP_SEMICIRCLE_ARROW => Ok(Arrows::AnticlockwiseTopSemicircleArrow),
            CLOCKWISE_TOP_SEMICIRCLE_ARROW => Ok(Arrows::ClockwiseTopSemicircleArrow),
            NORTH_WEST_ARROW_TO_LONG_BAR => Ok(Arrows::NorthWestArrowToLongBar),
            LEFTWARDS_ARROW_TO_BAR_OVER_RIGHTWARDS_ARROW_TO_BAR => Ok(Arrows::LeftwardsArrowToBarOverRightwardsArrowToBar),
            ANTICLOCKWISE_OPEN_CIRCLE_ARROW => Ok(Arrows::AnticlockwiseOpenCircleArrow),
            CLOCKWISE_OPEN_CIRCLE_ARROW => Ok(Arrows::ClockwiseOpenCircleArrow),
            LEFTWARDS_HARPOON_WITH_BARB_UPWARDS => Ok(Arrows::LeftwardsHarpoonWithBarbUpwards),
            LEFTWARDS_HARPOON_WITH_BARB_DOWNWARDS => Ok(Arrows::LeftwardsHarpoonWithBarbDownwards),
            UPWARDS_HARPOON_WITH_BARB_RIGHTWARDS => Ok(Arrows::UpwardsHarpoonWithBarbRightwards),
            UPWARDS_HARPOON_WITH_BARB_LEFTWARDS => Ok(Arrows::UpwardsHarpoonWithBarbLeftwards),
            RIGHTWARDS_HARPOON_WITH_BARB_UPWARDS => Ok(Arrows::RightwardsHarpoonWithBarbUpwards),
            RIGHTWARDS_HARPOON_WITH_BARB_DOWNWARDS => Ok(Arrows::RightwardsHarpoonWithBarbDownwards),
            DOWNWARDS_HARPOON_WITH_BARB_RIGHTWARDS => Ok(Arrows::DownwardsHarpoonWithBarbRightwards),
            DOWNWARDS_HARPOON_WITH_BARB_LEFTWARDS => Ok(Arrows::DownwardsHarpoonWithBarbLeftwards),
            RIGHTWARDS_ARROW_OVER_LEFTWARDS_ARROW => Ok(Arrows::RightwardsArrowOverLeftwardsArrow),
            UPWARDS_ARROW_LEFTWARDS_OF_DOWNWARDS_ARROW => Ok(Arrows::UpwardsArrowLeftwardsOfDownwardsArrow),
            LEFTWARDS_ARROW_OVER_RIGHTWARDS_ARROW => Ok(Arrows::LeftwardsArrowOverRightwardsArrow),
            LEFTWARDS_PAIRED_ARROWS => Ok(Arrows::LeftwardsPaired),
            UPWARDS_PAIRED_ARROWS => Ok(Arrows::UpwardsPaired),
            RIGHTWARDS_PAIRED_ARROWS => Ok(Arrows::RightwardsPaired),
            DOWNWARDS_PAIRED_ARROWS => Ok(Arrows::DownwardsPaired),
            LEFTWARDS_HARPOON_OVER_RIGHTWARDS_HARPOON => Ok(Arrows::LeftwardsHarpoonOverRightwardsHarpoon),
            RIGHTWARDS_HARPOON_OVER_LEFTWARDS_HARPOON => Ok(Arrows::RightwardsHarpoonOverLeftwardsHarpoon),
            LEFTWARDS_DOUBLE_ARROW_WITH_STROKE => Ok(Arrows::LeftwardsDoubleArrowWithStroke),
            LEFT_RIGHT_DOUBLE_ARROW_WITH_STROKE => Ok(Arrows::LeftRightDoubleArrowWithStroke),
            RIGHTWARDS_DOUBLE_ARROW_WITH_STROKE => Ok(Arrows::RightwardsDoubleArrowWithStroke),
            LEFTWARDS_DOUBLE_ARROW => Ok(Arrows::LeftwardsDoubleArrow),
            UPWARDS_DOUBLE_ARROW => Ok(Arrows::UpwardsDoubleArrow),
            RIGHTWARDS_DOUBLE_ARROW => Ok(Arrows::RightwardsDoubleArrow),
            DOWNWARDS_DOUBLE_ARROW => Ok(Arrows::DownwardsDoubleArrow),
            LEFT_RIGHT_DOUBLE_ARROW => Ok(Arrows::LeftRightDoubleArrow),
            UP_DOWN_DOUBLE_ARROW => Ok(Arrows::UpDownDoubleArrow),
            NORTH_WEST_DOUBLE_ARROW => Ok(Arrows::NorthWestDoubleArrow),
            NORTH_EAST_DOUBLE_ARROW => Ok(Arrows::NorthEastDoubleArrow),
            SOUTH_EAST_DOUBLE_ARROW => Ok(Arrows::SouthEastDoubleArrow),
            SOUTH_WEST_DOUBLE_ARROW => Ok(Arrows::SouthWestDoubleArrow),
            LEFTWARDS_TRIPLE_ARROW => Ok(Arrows::LeftwardsTripleArrow),
            RIGHTWARDS_TRIPLE_ARROW => Ok(Arrows::RightwardsTripleArrow),
            LEFTWARDS_SQUIGGLE_ARROW => Ok(Arrows::LeftwardsSquiggleArrow),
            RIGHTWARDS_SQUIGGLE_ARROW => Ok(Arrows::RightwardsSquiggleArrow),
            UPWARDS_ARROW_WITH_DOUBLE_STROKE => Ok(Arrows::UpwardsArrowWithDoubleStroke),
            DOWNWARDS_ARROW_WITH_DOUBLE_STROKE => Ok(Arrows::DownwardsArrowWithDoubleStroke),
            LEFTWARDS_DASHED_ARROW => Ok(Arrows::LeftwardsDashedArrow),
            UPWARDS_DASHED_ARROW => Ok(Arrows::UpwardsDashedArrow),
            RIGHTWARDS_DASHED_ARROW => Ok(Arrows::RightwardsDashedArrow),
            DOWNWARDS_DASHED_ARROW => Ok(Arrows::DownwardsDashedArrow),
            LEFTWARDS_ARROW_TO_BAR => Ok(Arrows::LeftwardsArrowToBar),
            RIGHTWARDS_ARROW_TO_BAR => Ok(Arrows::RightwardsArrowToBar),
            LEFTWARDS_WHITE_ARROW => Ok(Arrows::LeftwardsWhiteArrow),
            UPWARDS_WHITE_ARROW => Ok(Arrows::UpwardsWhiteArrow),
            RIGHTWARDS_WHITE_ARROW => Ok(Arrows::RightwardsWhiteArrow),
            DOWNWARDS_WHITE_ARROW => Ok(Arrows::DownwardsWhiteArrow),
            UPWARDS_WHITE_ARROW_FROM_BAR => Ok(Arrows::UpwardsWhiteArrowFromBar),
            UPWARDS_WHITE_ARROW_ON_PEDESTAL => Ok(Arrows::UpwardsWhiteArrowOnPedestal),
            UPWARDS_WHITE_ARROW_ON_PEDESTAL_WITH_HORIZONTAL_BAR => Ok(Arrows::UpwardsWhiteArrowOnPedestalWithHorizontalBar),
            UPWARDS_WHITE_ARROW_ON_PEDESTAL_WITH_VERTICAL_BAR => Ok(Arrows::UpwardsWhiteArrowOnPedestalWithVerticalBar),
            UPWARDS_WHITE_DOUBLE_ARROW => Ok(Arrows::UpwardsWhiteDoubleArrow),
            UPWARDS_WHITE_DOUBLE_ARROW_ON_PEDESTAL => Ok(Arrows::UpwardsWhiteDoubleArrowOnPedestal),
            RIGHTWARDS_WHITE_ARROW_FROM_WALL => Ok(Arrows::RightwardsWhiteArrowFromWall),
            NORTH_WEST_ARROW_TO_CORNER => Ok(Arrows::NorthWestArrowToCorner),
            SOUTH_EAST_ARROW_TO_CORNER => Ok(Arrows::SouthEastArrowToCorner),
            UP_DOWN_WHITE_ARROW => Ok(Arrows::UpDownWhiteArrow),
            RIGHT_ARROW_WITH_SMALL_CIRCLE => Ok(Arrows::RightArrowWithSmallCircle),
            DOWNWARDS_ARROW_LEFTWARDS_OF_UPWARDS_ARROW => Ok(Arrows::DownwardsArrowLeftwardsOfUpwardsArrow),
            THREE_RIGHTWARDS_ARROWS => Ok(Arrows::ThreeRightwards),
            LEFTWARDS_ARROW_WITH_VERTICAL_STROKE => Ok(Arrows::LeftwardsArrowWithVerticalStroke),
            RIGHTWARDS_ARROW_WITH_VERTICAL_STROKE => Ok(Arrows::RightwardsArrowWithVerticalStroke),
            LEFT_RIGHT_ARROW_WITH_VERTICAL_STROKE => Ok(Arrows::LeftRightArrowWithVerticalStroke),
            LEFTWARDS_ARROW_WITH_DOUBLE_VERTICAL_STROKE => Ok(Arrows::LeftwardsArrowWithDoubleVerticalStroke),
            RIGHTWARDS_ARROW_WITH_DOUBLE_VERTICAL_STROKE => Ok(Arrows::RightwardsArrowWithDoubleVerticalStroke),
            LEFT_RIGHT_ARROW_WITH_DOUBLE_VERTICAL_STROKE => Ok(Arrows::LeftRightArrowWithDoubleVerticalStroke),
            LEFTWARDS_OPEN_DASH_HEADED_ARROW => Ok(Arrows::LeftwardsOpenDashHeadedArrow),
            RIGHTWARDS_OPEN_DASH_HEADED_ARROW => Ok(Arrows::RightwardsOpenDashHeadedArrow),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Arrows {
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

impl std::convert::TryFrom<u32> for Arrows {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Arrows {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Arrows {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Arrows::LeftwardsArrow
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Arrows::LeftwardsArrow => "leftwards arrow",
            Arrows::UpwardsArrow => "upwards arrow",
            Arrows::RightwardsArrow => "rightwards arrow",
            Arrows::DownwardsArrow => "downwards arrow",
            Arrows::LeftRightArrow => "left right arrow",
            Arrows::UpDownArrow => "up down arrow",
            Arrows::NorthWestArrow => "north west arrow",
            Arrows::NorthEastArrow => "north east arrow",
            Arrows::SouthEastArrow => "south east arrow",
            Arrows::SouthWestArrow => "south west arrow",
            Arrows::LeftwardsArrowWithStroke => "leftwards arrow with stroke",
            Arrows::RightwardsArrowWithStroke => "rightwards arrow with stroke",
            Arrows::LeftwardsWaveArrow => "leftwards wave arrow",
            Arrows::RightwardsWaveArrow => "rightwards wave arrow",
            Arrows::LeftwardsTwoHeadedArrow => "leftwards two headed arrow",
            Arrows::UpwardsTwoHeadedArrow => "upwards two headed arrow",
            Arrows::RightwardsTwoHeadedArrow => "rightwards two headed arrow",
            Arrows::DownwardsTwoHeadedArrow => "downwards two headed arrow",
            Arrows::LeftwardsArrowWithTail => "leftwards arrow with tail",
            Arrows::RightwardsArrowWithTail => "rightwards arrow with tail",
            Arrows::LeftwardsArrowFromBar => "leftwards arrow from bar",
            Arrows::UpwardsArrowFromBar => "upwards arrow from bar",
            Arrows::RightwardsArrowFromBar => "rightwards arrow from bar",
            Arrows::DownwardsArrowFromBar => "downwards arrow from bar",
            Arrows::UpDownArrowWithBase => "up down arrow with base",
            Arrows::LeftwardsArrowWithHook => "leftwards arrow with hook",
            Arrows::RightwardsArrowWithHook => "rightwards arrow with hook",
            Arrows::LeftwardsArrowWithLoop => "leftwards arrow with loop",
            Arrows::RightwardsArrowWithLoop => "rightwards arrow with loop",
            Arrows::LeftRightWaveArrow => "left right wave arrow",
            Arrows::LeftRightArrowWithStroke => "left right arrow with stroke",
            Arrows::DownwardsZigzagArrow => "downwards zigzag arrow",
            Arrows::UpwardsArrowWithTipLeftwards => "upwards arrow with tip leftwards",
            Arrows::UpwardsArrowWithTipRightwards => "upwards arrow with tip rightwards",
            Arrows::DownwardsArrowWithTipLeftwards => "downwards arrow with tip leftwards",
            Arrows::DownwardsArrowWithTipRightwards => "downwards arrow with tip rightwards",
            Arrows::RightwardsArrowWithCornerDownwards => "rightwards arrow with corner downwards",
            Arrows::DownwardsArrowWithCornerLeftwards => "downwards arrow with corner leftwards",
            Arrows::AnticlockwiseTopSemicircleArrow => "anticlockwise top semicircle arrow",
            Arrows::ClockwiseTopSemicircleArrow => "clockwise top semicircle arrow",
            Arrows::NorthWestArrowToLongBar => "north west arrow to long bar",
            Arrows::LeftwardsArrowToBarOverRightwardsArrowToBar => "leftwards arrow to bar over rightwards arrow to bar",
            Arrows::AnticlockwiseOpenCircleArrow => "anticlockwise open circle arrow",
            Arrows::ClockwiseOpenCircleArrow => "clockwise open circle arrow",
            Arrows::LeftwardsHarpoonWithBarbUpwards => "leftwards harpoon with barb upwards",
            Arrows::LeftwardsHarpoonWithBarbDownwards => "leftwards harpoon with barb downwards",
            Arrows::UpwardsHarpoonWithBarbRightwards => "upwards harpoon with barb rightwards",
            Arrows::UpwardsHarpoonWithBarbLeftwards => "upwards harpoon with barb leftwards",
            Arrows::RightwardsHarpoonWithBarbUpwards => "rightwards harpoon with barb upwards",
            Arrows::RightwardsHarpoonWithBarbDownwards => "rightwards harpoon with barb downwards",
            Arrows::DownwardsHarpoonWithBarbRightwards => "downwards harpoon with barb rightwards",
            Arrows::DownwardsHarpoonWithBarbLeftwards => "downwards harpoon with barb leftwards",
            Arrows::RightwardsArrowOverLeftwardsArrow => "rightwards arrow over leftwards arrow",
            Arrows::UpwardsArrowLeftwardsOfDownwardsArrow => "upwards arrow leftwards of downwards arrow",
            Arrows::LeftwardsArrowOverRightwardsArrow => "leftwards arrow over rightwards arrow",
            Arrows::LeftwardsPaired => "leftwards paired arrows",
            Arrows::UpwardsPaired => "upwards paired arrows",
            Arrows::RightwardsPaired => "rightwards paired arrows",
            Arrows::DownwardsPaired => "downwards paired arrows",
            Arrows::LeftwardsHarpoonOverRightwardsHarpoon => "leftwards harpoon over rightwards harpoon",
            Arrows::RightwardsHarpoonOverLeftwardsHarpoon => "rightwards harpoon over leftwards harpoon",
            Arrows::LeftwardsDoubleArrowWithStroke => "leftwards double arrow with stroke",
            Arrows::LeftRightDoubleArrowWithStroke => "left right double arrow with stroke",
            Arrows::RightwardsDoubleArrowWithStroke => "rightwards double arrow with stroke",
            Arrows::LeftwardsDoubleArrow => "leftwards double arrow",
            Arrows::UpwardsDoubleArrow => "upwards double arrow",
            Arrows::RightwardsDoubleArrow => "rightwards double arrow",
            Arrows::DownwardsDoubleArrow => "downwards double arrow",
            Arrows::LeftRightDoubleArrow => "left right double arrow",
            Arrows::UpDownDoubleArrow => "up down double arrow",
            Arrows::NorthWestDoubleArrow => "north west double arrow",
            Arrows::NorthEastDoubleArrow => "north east double arrow",
            Arrows::SouthEastDoubleArrow => "south east double arrow",
            Arrows::SouthWestDoubleArrow => "south west double arrow",
            Arrows::LeftwardsTripleArrow => "leftwards triple arrow",
            Arrows::RightwardsTripleArrow => "rightwards triple arrow",
            Arrows::LeftwardsSquiggleArrow => "leftwards squiggle arrow",
            Arrows::RightwardsSquiggleArrow => "rightwards squiggle arrow",
            Arrows::UpwardsArrowWithDoubleStroke => "upwards arrow with double stroke",
            Arrows::DownwardsArrowWithDoubleStroke => "downwards arrow with double stroke",
            Arrows::LeftwardsDashedArrow => "leftwards dashed arrow",
            Arrows::UpwardsDashedArrow => "upwards dashed arrow",
            Arrows::RightwardsDashedArrow => "rightwards dashed arrow",
            Arrows::DownwardsDashedArrow => "downwards dashed arrow",
            Arrows::LeftwardsArrowToBar => "leftwards arrow to bar",
            Arrows::RightwardsArrowToBar => "rightwards arrow to bar",
            Arrows::LeftwardsWhiteArrow => "leftwards white arrow",
            Arrows::UpwardsWhiteArrow => "upwards white arrow",
            Arrows::RightwardsWhiteArrow => "rightwards white arrow",
            Arrows::DownwardsWhiteArrow => "downwards white arrow",
            Arrows::UpwardsWhiteArrowFromBar => "upwards white arrow from bar",
            Arrows::UpwardsWhiteArrowOnPedestal => "upwards white arrow on pedestal",
            Arrows::UpwardsWhiteArrowOnPedestalWithHorizontalBar => "upwards white arrow on pedestal with horizontal bar",
            Arrows::UpwardsWhiteArrowOnPedestalWithVerticalBar => "upwards white arrow on pedestal with vertical bar",
            Arrows::UpwardsWhiteDoubleArrow => "upwards white double arrow",
            Arrows::UpwardsWhiteDoubleArrowOnPedestal => "upwards white double arrow on pedestal",
            Arrows::RightwardsWhiteArrowFromWall => "rightwards white arrow from wall",
            Arrows::NorthWestArrowToCorner => "north west arrow to corner",
            Arrows::SouthEastArrowToCorner => "south east arrow to corner",
            Arrows::UpDownWhiteArrow => "up down white arrow",
            Arrows::RightArrowWithSmallCircle => "right arrow with small circle",
            Arrows::DownwardsArrowLeftwardsOfUpwardsArrow => "downwards arrow leftwards of upwards arrow",
            Arrows::ThreeRightwards => "three rightwards arrows",
            Arrows::LeftwardsArrowWithVerticalStroke => "leftwards arrow with vertical stroke",
            Arrows::RightwardsArrowWithVerticalStroke => "rightwards arrow with vertical stroke",
            Arrows::LeftRightArrowWithVerticalStroke => "left right arrow with vertical stroke",
            Arrows::LeftwardsArrowWithDoubleVerticalStroke => "leftwards arrow with double vertical stroke",
            Arrows::RightwardsArrowWithDoubleVerticalStroke => "rightwards arrow with double vertical stroke",
            Arrows::LeftRightArrowWithDoubleVerticalStroke => "left right arrow with double vertical stroke",
            Arrows::LeftwardsOpenDashHeadedArrow => "leftwards open-headed arrow",
            Arrows::RightwardsOpenDashHeadedArrow => "rightwards open-headed arrow",
        }
    }
}
