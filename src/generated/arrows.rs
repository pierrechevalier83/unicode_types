
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
        match self {
            Arrows::LeftwardsArrow => '←',
            Arrows::UpwardsArrow => '↑',
            Arrows::RightwardsArrow => '→',
            Arrows::DownwardsArrow => '↓',
            Arrows::LeftRightArrow => '↔',
            Arrows::UpDownArrow => '↕',
            Arrows::NorthWestArrow => '↖',
            Arrows::NorthEastArrow => '↗',
            Arrows::SouthEastArrow => '↘',
            Arrows::SouthWestArrow => '↙',
            Arrows::LeftwardsArrowWithStroke => '↚',
            Arrows::RightwardsArrowWithStroke => '↛',
            Arrows::LeftwardsWaveArrow => '↜',
            Arrows::RightwardsWaveArrow => '↝',
            Arrows::LeftwardsTwoHeadedArrow => '↞',
            Arrows::UpwardsTwoHeadedArrow => '↟',
            Arrows::RightwardsTwoHeadedArrow => '↠',
            Arrows::DownwardsTwoHeadedArrow => '↡',
            Arrows::LeftwardsArrowWithTail => '↢',
            Arrows::RightwardsArrowWithTail => '↣',
            Arrows::LeftwardsArrowFromBar => '↤',
            Arrows::UpwardsArrowFromBar => '↥',
            Arrows::RightwardsArrowFromBar => '↦',
            Arrows::DownwardsArrowFromBar => '↧',
            Arrows::UpDownArrowWithBase => '↨',
            Arrows::LeftwardsArrowWithHook => '↩',
            Arrows::RightwardsArrowWithHook => '↪',
            Arrows::LeftwardsArrowWithLoop => '↫',
            Arrows::RightwardsArrowWithLoop => '↬',
            Arrows::LeftRightWaveArrow => '↭',
            Arrows::LeftRightArrowWithStroke => '↮',
            Arrows::DownwardsZigzagArrow => '↯',
            Arrows::UpwardsArrowWithTipLeftwards => '↰',
            Arrows::UpwardsArrowWithTipRightwards => '↱',
            Arrows::DownwardsArrowWithTipLeftwards => '↲',
            Arrows::DownwardsArrowWithTipRightwards => '↳',
            Arrows::RightwardsArrowWithCornerDownwards => '↴',
            Arrows::DownwardsArrowWithCornerLeftwards => '↵',
            Arrows::AnticlockwiseTopSemicircleArrow => '↶',
            Arrows::ClockwiseTopSemicircleArrow => '↷',
            Arrows::NorthWestArrowToLongBar => '↸',
            Arrows::LeftwardsArrowToBarOverRightwardsArrowToBar => '↹',
            Arrows::AnticlockwiseOpenCircleArrow => '↺',
            Arrows::ClockwiseOpenCircleArrow => '↻',
            Arrows::LeftwardsHarpoonWithBarbUpwards => '↼',
            Arrows::LeftwardsHarpoonWithBarbDownwards => '↽',
            Arrows::UpwardsHarpoonWithBarbRightwards => '↾',
            Arrows::UpwardsHarpoonWithBarbLeftwards => '↿',
            Arrows::RightwardsHarpoonWithBarbUpwards => '⇀',
            Arrows::RightwardsHarpoonWithBarbDownwards => '⇁',
            Arrows::DownwardsHarpoonWithBarbRightwards => '⇂',
            Arrows::DownwardsHarpoonWithBarbLeftwards => '⇃',
            Arrows::RightwardsArrowOverLeftwardsArrow => '⇄',
            Arrows::UpwardsArrowLeftwardsOfDownwardsArrow => '⇅',
            Arrows::LeftwardsArrowOverRightwardsArrow => '⇆',
            Arrows::LeftwardsPaired => '⇇',
            Arrows::UpwardsPaired => '⇈',
            Arrows::RightwardsPaired => '⇉',
            Arrows::DownwardsPaired => '⇊',
            Arrows::LeftwardsHarpoonOverRightwardsHarpoon => '⇋',
            Arrows::RightwardsHarpoonOverLeftwardsHarpoon => '⇌',
            Arrows::LeftwardsDoubleArrowWithStroke => '⇍',
            Arrows::LeftRightDoubleArrowWithStroke => '⇎',
            Arrows::RightwardsDoubleArrowWithStroke => '⇏',
            Arrows::LeftwardsDoubleArrow => '⇐',
            Arrows::UpwardsDoubleArrow => '⇑',
            Arrows::RightwardsDoubleArrow => '⇒',
            Arrows::DownwardsDoubleArrow => '⇓',
            Arrows::LeftRightDoubleArrow => '⇔',
            Arrows::UpDownDoubleArrow => '⇕',
            Arrows::NorthWestDoubleArrow => '⇖',
            Arrows::NorthEastDoubleArrow => '⇗',
            Arrows::SouthEastDoubleArrow => '⇘',
            Arrows::SouthWestDoubleArrow => '⇙',
            Arrows::LeftwardsTripleArrow => '⇚',
            Arrows::RightwardsTripleArrow => '⇛',
            Arrows::LeftwardsSquiggleArrow => '⇜',
            Arrows::RightwardsSquiggleArrow => '⇝',
            Arrows::UpwardsArrowWithDoubleStroke => '⇞',
            Arrows::DownwardsArrowWithDoubleStroke => '⇟',
            Arrows::LeftwardsDashedArrow => '⇠',
            Arrows::UpwardsDashedArrow => '⇡',
            Arrows::RightwardsDashedArrow => '⇢',
            Arrows::DownwardsDashedArrow => '⇣',
            Arrows::LeftwardsArrowToBar => '⇤',
            Arrows::RightwardsArrowToBar => '⇥',
            Arrows::LeftwardsWhiteArrow => '⇦',
            Arrows::UpwardsWhiteArrow => '⇧',
            Arrows::RightwardsWhiteArrow => '⇨',
            Arrows::DownwardsWhiteArrow => '⇩',
            Arrows::UpwardsWhiteArrowFromBar => '⇪',
            Arrows::UpwardsWhiteArrowOnPedestal => '⇫',
            Arrows::UpwardsWhiteArrowOnPedestalWithHorizontalBar => '⇬',
            Arrows::UpwardsWhiteArrowOnPedestalWithVerticalBar => '⇭',
            Arrows::UpwardsWhiteDoubleArrow => '⇮',
            Arrows::UpwardsWhiteDoubleArrowOnPedestal => '⇯',
            Arrows::RightwardsWhiteArrowFromWall => '⇰',
            Arrows::NorthWestArrowToCorner => '⇱',
            Arrows::SouthEastArrowToCorner => '⇲',
            Arrows::UpDownWhiteArrow => '⇳',
            Arrows::RightArrowWithSmallCircle => '⇴',
            Arrows::DownwardsArrowLeftwardsOfUpwardsArrow => '⇵',
            Arrows::ThreeRightwards => '⇶',
            Arrows::LeftwardsArrowWithVerticalStroke => '⇷',
            Arrows::RightwardsArrowWithVerticalStroke => '⇸',
            Arrows::LeftRightArrowWithVerticalStroke => '⇹',
            Arrows::LeftwardsArrowWithDoubleVerticalStroke => '⇺',
            Arrows::RightwardsArrowWithDoubleVerticalStroke => '⇻',
            Arrows::LeftRightArrowWithDoubleVerticalStroke => '⇼',
            Arrows::LeftwardsOpenDashHeadedArrow => '⇽',
            Arrows::RightwardsOpenDashHeadedArrow => '⇾',
        }
    }
}

impl std::convert::TryFrom<char> for Arrows {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '←' => Ok(Arrows::LeftwardsArrow),
            '↑' => Ok(Arrows::UpwardsArrow),
            '→' => Ok(Arrows::RightwardsArrow),
            '↓' => Ok(Arrows::DownwardsArrow),
            '↔' => Ok(Arrows::LeftRightArrow),
            '↕' => Ok(Arrows::UpDownArrow),
            '↖' => Ok(Arrows::NorthWestArrow),
            '↗' => Ok(Arrows::NorthEastArrow),
            '↘' => Ok(Arrows::SouthEastArrow),
            '↙' => Ok(Arrows::SouthWestArrow),
            '↚' => Ok(Arrows::LeftwardsArrowWithStroke),
            '↛' => Ok(Arrows::RightwardsArrowWithStroke),
            '↜' => Ok(Arrows::LeftwardsWaveArrow),
            '↝' => Ok(Arrows::RightwardsWaveArrow),
            '↞' => Ok(Arrows::LeftwardsTwoHeadedArrow),
            '↟' => Ok(Arrows::UpwardsTwoHeadedArrow),
            '↠' => Ok(Arrows::RightwardsTwoHeadedArrow),
            '↡' => Ok(Arrows::DownwardsTwoHeadedArrow),
            '↢' => Ok(Arrows::LeftwardsArrowWithTail),
            '↣' => Ok(Arrows::RightwardsArrowWithTail),
            '↤' => Ok(Arrows::LeftwardsArrowFromBar),
            '↥' => Ok(Arrows::UpwardsArrowFromBar),
            '↦' => Ok(Arrows::RightwardsArrowFromBar),
            '↧' => Ok(Arrows::DownwardsArrowFromBar),
            '↨' => Ok(Arrows::UpDownArrowWithBase),
            '↩' => Ok(Arrows::LeftwardsArrowWithHook),
            '↪' => Ok(Arrows::RightwardsArrowWithHook),
            '↫' => Ok(Arrows::LeftwardsArrowWithLoop),
            '↬' => Ok(Arrows::RightwardsArrowWithLoop),
            '↭' => Ok(Arrows::LeftRightWaveArrow),
            '↮' => Ok(Arrows::LeftRightArrowWithStroke),
            '↯' => Ok(Arrows::DownwardsZigzagArrow),
            '↰' => Ok(Arrows::UpwardsArrowWithTipLeftwards),
            '↱' => Ok(Arrows::UpwardsArrowWithTipRightwards),
            '↲' => Ok(Arrows::DownwardsArrowWithTipLeftwards),
            '↳' => Ok(Arrows::DownwardsArrowWithTipRightwards),
            '↴' => Ok(Arrows::RightwardsArrowWithCornerDownwards),
            '↵' => Ok(Arrows::DownwardsArrowWithCornerLeftwards),
            '↶' => Ok(Arrows::AnticlockwiseTopSemicircleArrow),
            '↷' => Ok(Arrows::ClockwiseTopSemicircleArrow),
            '↸' => Ok(Arrows::NorthWestArrowToLongBar),
            '↹' => Ok(Arrows::LeftwardsArrowToBarOverRightwardsArrowToBar),
            '↺' => Ok(Arrows::AnticlockwiseOpenCircleArrow),
            '↻' => Ok(Arrows::ClockwiseOpenCircleArrow),
            '↼' => Ok(Arrows::LeftwardsHarpoonWithBarbUpwards),
            '↽' => Ok(Arrows::LeftwardsHarpoonWithBarbDownwards),
            '↾' => Ok(Arrows::UpwardsHarpoonWithBarbRightwards),
            '↿' => Ok(Arrows::UpwardsHarpoonWithBarbLeftwards),
            '⇀' => Ok(Arrows::RightwardsHarpoonWithBarbUpwards),
            '⇁' => Ok(Arrows::RightwardsHarpoonWithBarbDownwards),
            '⇂' => Ok(Arrows::DownwardsHarpoonWithBarbRightwards),
            '⇃' => Ok(Arrows::DownwardsHarpoonWithBarbLeftwards),
            '⇄' => Ok(Arrows::RightwardsArrowOverLeftwardsArrow),
            '⇅' => Ok(Arrows::UpwardsArrowLeftwardsOfDownwardsArrow),
            '⇆' => Ok(Arrows::LeftwardsArrowOverRightwardsArrow),
            '⇇' => Ok(Arrows::LeftwardsPaired),
            '⇈' => Ok(Arrows::UpwardsPaired),
            '⇉' => Ok(Arrows::RightwardsPaired),
            '⇊' => Ok(Arrows::DownwardsPaired),
            '⇋' => Ok(Arrows::LeftwardsHarpoonOverRightwardsHarpoon),
            '⇌' => Ok(Arrows::RightwardsHarpoonOverLeftwardsHarpoon),
            '⇍' => Ok(Arrows::LeftwardsDoubleArrowWithStroke),
            '⇎' => Ok(Arrows::LeftRightDoubleArrowWithStroke),
            '⇏' => Ok(Arrows::RightwardsDoubleArrowWithStroke),
            '⇐' => Ok(Arrows::LeftwardsDoubleArrow),
            '⇑' => Ok(Arrows::UpwardsDoubleArrow),
            '⇒' => Ok(Arrows::RightwardsDoubleArrow),
            '⇓' => Ok(Arrows::DownwardsDoubleArrow),
            '⇔' => Ok(Arrows::LeftRightDoubleArrow),
            '⇕' => Ok(Arrows::UpDownDoubleArrow),
            '⇖' => Ok(Arrows::NorthWestDoubleArrow),
            '⇗' => Ok(Arrows::NorthEastDoubleArrow),
            '⇘' => Ok(Arrows::SouthEastDoubleArrow),
            '⇙' => Ok(Arrows::SouthWestDoubleArrow),
            '⇚' => Ok(Arrows::LeftwardsTripleArrow),
            '⇛' => Ok(Arrows::RightwardsTripleArrow),
            '⇜' => Ok(Arrows::LeftwardsSquiggleArrow),
            '⇝' => Ok(Arrows::RightwardsSquiggleArrow),
            '⇞' => Ok(Arrows::UpwardsArrowWithDoubleStroke),
            '⇟' => Ok(Arrows::DownwardsArrowWithDoubleStroke),
            '⇠' => Ok(Arrows::LeftwardsDashedArrow),
            '⇡' => Ok(Arrows::UpwardsDashedArrow),
            '⇢' => Ok(Arrows::RightwardsDashedArrow),
            '⇣' => Ok(Arrows::DownwardsDashedArrow),
            '⇤' => Ok(Arrows::LeftwardsArrowToBar),
            '⇥' => Ok(Arrows::RightwardsArrowToBar),
            '⇦' => Ok(Arrows::LeftwardsWhiteArrow),
            '⇧' => Ok(Arrows::UpwardsWhiteArrow),
            '⇨' => Ok(Arrows::RightwardsWhiteArrow),
            '⇩' => Ok(Arrows::DownwardsWhiteArrow),
            '⇪' => Ok(Arrows::UpwardsWhiteArrowFromBar),
            '⇫' => Ok(Arrows::UpwardsWhiteArrowOnPedestal),
            '⇬' => Ok(Arrows::UpwardsWhiteArrowOnPedestalWithHorizontalBar),
            '⇭' => Ok(Arrows::UpwardsWhiteArrowOnPedestalWithVerticalBar),
            '⇮' => Ok(Arrows::UpwardsWhiteDoubleArrow),
            '⇯' => Ok(Arrows::UpwardsWhiteDoubleArrowOnPedestal),
            '⇰' => Ok(Arrows::RightwardsWhiteArrowFromWall),
            '⇱' => Ok(Arrows::NorthWestArrowToCorner),
            '⇲' => Ok(Arrows::SouthEastArrowToCorner),
            '⇳' => Ok(Arrows::UpDownWhiteArrow),
            '⇴' => Ok(Arrows::RightArrowWithSmallCircle),
            '⇵' => Ok(Arrows::DownwardsArrowLeftwardsOfUpwardsArrow),
            '⇶' => Ok(Arrows::ThreeRightwards),
            '⇷' => Ok(Arrows::LeftwardsArrowWithVerticalStroke),
            '⇸' => Ok(Arrows::RightwardsArrowWithVerticalStroke),
            '⇹' => Ok(Arrows::LeftRightArrowWithVerticalStroke),
            '⇺' => Ok(Arrows::LeftwardsArrowWithDoubleVerticalStroke),
            '⇻' => Ok(Arrows::RightwardsArrowWithDoubleVerticalStroke),
            '⇼' => Ok(Arrows::LeftRightArrowWithDoubleVerticalStroke),
            '⇽' => Ok(Arrows::LeftwardsOpenDashHeadedArrow),
            '⇾' => Ok(Arrows::RightwardsOpenDashHeadedArrow),
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

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Arrows{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
