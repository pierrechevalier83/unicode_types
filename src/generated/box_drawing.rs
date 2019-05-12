
/// An enum to represent all characters in the BoxDrawing block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum BoxDrawing {
    /// \u{2500}: '─'
    LightHorizontal,
    /// \u{2501}: '━'
    HeavyHorizontal,
    /// \u{2502}: '│'
    LightVertical,
    /// \u{2503}: '┃'
    HeavyVertical,
    /// \u{2504}: '┄'
    LightTripleDashHorizontal,
    /// \u{2505}: '┅'
    HeavyTripleDashHorizontal,
    /// \u{2506}: '┆'
    LightTripleDashVertical,
    /// \u{2507}: '┇'
    HeavyTripleDashVertical,
    /// \u{2508}: '┈'
    LightQuadrupleDashHorizontal,
    /// \u{2509}: '┉'
    HeavyQuadrupleDashHorizontal,
    /// \u{250a}: '┊'
    LightQuadrupleDashVertical,
    /// \u{250b}: '┋'
    HeavyQuadrupleDashVertical,
    /// \u{250c}: '┌'
    LightDownAndRight,
    /// \u{250d}: '┍'
    DownLightAndRightHeavy,
    /// \u{250e}: '┎'
    DownHeavyAndRightLight,
    /// \u{250f}: '┏'
    HeavyDownAndRight,
    /// \u{2510}: '┐'
    LightDownAndLeft,
    /// \u{2511}: '┑'
    DownLightAndLeftHeavy,
    /// \u{2512}: '┒'
    DownHeavyAndLeftLight,
    /// \u{2513}: '┓'
    HeavyDownAndLeft,
    /// \u{2514}: '└'
    LightUpAndRight,
    /// \u{2515}: '┕'
    UpLightAndRightHeavy,
    /// \u{2516}: '┖'
    UpHeavyAndRightLight,
    /// \u{2517}: '┗'
    HeavyUpAndRight,
    /// \u{2518}: '┘'
    LightUpAndLeft,
    /// \u{2519}: '┙'
    UpLightAndLeftHeavy,
    /// \u{251a}: '┚'
    UpHeavyAndLeftLight,
    /// \u{251b}: '┛'
    HeavyUpAndLeft,
    /// \u{251c}: '├'
    LightVerticalAndRight,
    /// \u{251d}: '┝'
    VerticalLightAndRightHeavy,
    /// \u{251e}: '┞'
    UpHeavyAndRightDownLight,
    /// \u{251f}: '┟'
    DownHeavyAndRightUpLight,
    /// \u{2520}: '┠'
    VerticalHeavyAndRightLight,
    /// \u{2521}: '┡'
    DownLightAndRightUpHeavy,
    /// \u{2522}: '┢'
    UpLightAndRightDownHeavy,
    /// \u{2523}: '┣'
    HeavyVerticalAndRight,
    /// \u{2524}: '┤'
    LightVerticalAndLeft,
    /// \u{2525}: '┥'
    VerticalLightAndLeftHeavy,
    /// \u{2526}: '┦'
    UpHeavyAndLeftDownLight,
    /// \u{2527}: '┧'
    DownHeavyAndLeftUpLight,
    /// \u{2528}: '┨'
    VerticalHeavyAndLeftLight,
    /// \u{2529}: '┩'
    DownLightAndLeftUpHeavy,
    /// \u{252a}: '┪'
    UpLightAndLeftDownHeavy,
    /// \u{252b}: '┫'
    HeavyVerticalAndLeft,
    /// \u{252c}: '┬'
    LightDownAndHorizontal,
    /// \u{252d}: '┭'
    LeftHeavyAndRightDownLight,
    /// \u{252e}: '┮'
    RightHeavyAndLeftDownLight,
    /// \u{252f}: '┯'
    DownLightAndHorizontalHeavy,
    /// \u{2530}: '┰'
    DownHeavyAndHorizontalLight,
    /// \u{2531}: '┱'
    RightLightAndLeftDownHeavy,
    /// \u{2532}: '┲'
    LeftLightAndRightDownHeavy,
    /// \u{2533}: '┳'
    HeavyDownAndHorizontal,
    /// \u{2534}: '┴'
    LightUpAndHorizontal,
    /// \u{2535}: '┵'
    LeftHeavyAndRightUpLight,
    /// \u{2536}: '┶'
    RightHeavyAndLeftUpLight,
    /// \u{2537}: '┷'
    UpLightAndHorizontalHeavy,
    /// \u{2538}: '┸'
    UpHeavyAndHorizontalLight,
    /// \u{2539}: '┹'
    RightLightAndLeftUpHeavy,
    /// \u{253a}: '┺'
    LeftLightAndRightUpHeavy,
    /// \u{253b}: '┻'
    HeavyUpAndHorizontal,
    /// \u{253c}: '┼'
    LightVerticalAndHorizontal,
    /// \u{253d}: '┽'
    LeftHeavyAndRightVerticalLight,
    /// \u{253e}: '┾'
    RightHeavyAndLeftVerticalLight,
    /// \u{253f}: '┿'
    VerticalLightAndHorizontalHeavy,
    /// \u{2540}: '╀'
    UpHeavyAndDownHorizontalLight,
    /// \u{2541}: '╁'
    DownHeavyAndUpHorizontalLight,
    /// \u{2542}: '╂'
    VerticalHeavyAndHorizontalLight,
    /// \u{2543}: '╃'
    LeftUpHeavyAndRightDownLight,
    /// \u{2544}: '╄'
    RightUpHeavyAndLeftDownLight,
    /// \u{2545}: '╅'
    LeftDownHeavyAndRightUpLight,
    /// \u{2546}: '╆'
    RightDownHeavyAndLeftUpLight,
    /// \u{2547}: '╇'
    DownLightAndUpHorizontalHeavy,
    /// \u{2548}: '╈'
    UpLightAndDownHorizontalHeavy,
    /// \u{2549}: '╉'
    RightLightAndLeftVerticalHeavy,
    /// \u{254a}: '╊'
    LeftLightAndRightVerticalHeavy,
    /// \u{254b}: '╋'
    HeavyVerticalAndHorizontal,
    /// \u{254c}: '╌'
    LightDoubleDashHorizontal,
    /// \u{254d}: '╍'
    HeavyDoubleDashHorizontal,
    /// \u{254e}: '╎'
    LightDoubleDashVertical,
    /// \u{254f}: '╏'
    HeavyDoubleDashVertical,
    /// \u{2550}: '═'
    DoubleHorizontal,
    /// \u{2551}: '║'
    DoubleVertical,
    /// \u{2552}: '╒'
    DownSingleAndRightDouble,
    /// \u{2553}: '╓'
    DownDoubleAndRightSingle,
    /// \u{2554}: '╔'
    DoubleDownAndRight,
    /// \u{2555}: '╕'
    DownSingleAndLeftDouble,
    /// \u{2556}: '╖'
    DownDoubleAndLeftSingle,
    /// \u{2557}: '╗'
    DoubleDownAndLeft,
    /// \u{2558}: '╘'
    UpSingleAndRightDouble,
    /// \u{2559}: '╙'
    UpDoubleAndRightSingle,
    /// \u{255a}: '╚'
    DoubleUpAndRight,
    /// \u{255b}: '╛'
    UpSingleAndLeftDouble,
    /// \u{255c}: '╜'
    UpDoubleAndLeftSingle,
    /// \u{255d}: '╝'
    DoubleUpAndLeft,
    /// \u{255e}: '╞'
    VerticalSingleAndRightDouble,
    /// \u{255f}: '╟'
    VerticalDoubleAndRightSingle,
    /// \u{2560}: '╠'
    DoubleVerticalAndRight,
    /// \u{2561}: '╡'
    VerticalSingleAndLeftDouble,
    /// \u{2562}: '╢'
    VerticalDoubleAndLeftSingle,
    /// \u{2563}: '╣'
    DoubleVerticalAndLeft,
    /// \u{2564}: '╤'
    DownSingleAndHorizontalDouble,
    /// \u{2565}: '╥'
    DownDoubleAndHorizontalSingle,
    /// \u{2566}: '╦'
    DoubleDownAndHorizontal,
    /// \u{2567}: '╧'
    UpSingleAndHorizontalDouble,
    /// \u{2568}: '╨'
    UpDoubleAndHorizontalSingle,
    /// \u{2569}: '╩'
    DoubleUpAndHorizontal,
    /// \u{256a}: '╪'
    VerticalSingleAndHorizontalDouble,
    /// \u{256b}: '╫'
    VerticalDoubleAndHorizontalSingle,
    /// \u{256c}: '╬'
    DoubleVerticalAndHorizontal,
    /// \u{256d}: '╭'
    LightArcDownAndRight,
    /// \u{256e}: '╮'
    LightArcDownAndLeft,
    /// \u{256f}: '╯'
    LightArcUpAndLeft,
    /// \u{2570}: '╰'
    LightArcUpAndRight,
    /// \u{2571}: '╱'
    LightDiagonalUpperRightToLowerLeft,
    /// \u{2572}: '╲'
    LightDiagonalUpperLeftToLowerRight,
    /// \u{2573}: '╳'
    LightDiagonalCross,
    /// \u{2574}: '╴'
    LightLeft,
    /// \u{2575}: '╵'
    LightUp,
    /// \u{2576}: '╶'
    LightRight,
    /// \u{2577}: '╷'
    LightDown,
    /// \u{2578}: '╸'
    HeavyLeft,
    /// \u{2579}: '╹'
    HeavyUp,
    /// \u{257a}: '╺'
    HeavyRight,
    /// \u{257b}: '╻'
    HeavyDown,
    /// \u{257c}: '╼'
    LightLeftAndHeavyRight,
    /// \u{257d}: '╽'
    LightUpAndHeavyDown,
    /// \u{257e}: '╾'
    HeavyLeftAndLightRight,
}

impl Into<char> for BoxDrawing {
    fn into(self) -> char {
        match self {
            BoxDrawing::LightHorizontal => '─',
            BoxDrawing::HeavyHorizontal => '━',
            BoxDrawing::LightVertical => '│',
            BoxDrawing::HeavyVertical => '┃',
            BoxDrawing::LightTripleDashHorizontal => '┄',
            BoxDrawing::HeavyTripleDashHorizontal => '┅',
            BoxDrawing::LightTripleDashVertical => '┆',
            BoxDrawing::HeavyTripleDashVertical => '┇',
            BoxDrawing::LightQuadrupleDashHorizontal => '┈',
            BoxDrawing::HeavyQuadrupleDashHorizontal => '┉',
            BoxDrawing::LightQuadrupleDashVertical => '┊',
            BoxDrawing::HeavyQuadrupleDashVertical => '┋',
            BoxDrawing::LightDownAndRight => '┌',
            BoxDrawing::DownLightAndRightHeavy => '┍',
            BoxDrawing::DownHeavyAndRightLight => '┎',
            BoxDrawing::HeavyDownAndRight => '┏',
            BoxDrawing::LightDownAndLeft => '┐',
            BoxDrawing::DownLightAndLeftHeavy => '┑',
            BoxDrawing::DownHeavyAndLeftLight => '┒',
            BoxDrawing::HeavyDownAndLeft => '┓',
            BoxDrawing::LightUpAndRight => '└',
            BoxDrawing::UpLightAndRightHeavy => '┕',
            BoxDrawing::UpHeavyAndRightLight => '┖',
            BoxDrawing::HeavyUpAndRight => '┗',
            BoxDrawing::LightUpAndLeft => '┘',
            BoxDrawing::UpLightAndLeftHeavy => '┙',
            BoxDrawing::UpHeavyAndLeftLight => '┚',
            BoxDrawing::HeavyUpAndLeft => '┛',
            BoxDrawing::LightVerticalAndRight => '├',
            BoxDrawing::VerticalLightAndRightHeavy => '┝',
            BoxDrawing::UpHeavyAndRightDownLight => '┞',
            BoxDrawing::DownHeavyAndRightUpLight => '┟',
            BoxDrawing::VerticalHeavyAndRightLight => '┠',
            BoxDrawing::DownLightAndRightUpHeavy => '┡',
            BoxDrawing::UpLightAndRightDownHeavy => '┢',
            BoxDrawing::HeavyVerticalAndRight => '┣',
            BoxDrawing::LightVerticalAndLeft => '┤',
            BoxDrawing::VerticalLightAndLeftHeavy => '┥',
            BoxDrawing::UpHeavyAndLeftDownLight => '┦',
            BoxDrawing::DownHeavyAndLeftUpLight => '┧',
            BoxDrawing::VerticalHeavyAndLeftLight => '┨',
            BoxDrawing::DownLightAndLeftUpHeavy => '┩',
            BoxDrawing::UpLightAndLeftDownHeavy => '┪',
            BoxDrawing::HeavyVerticalAndLeft => '┫',
            BoxDrawing::LightDownAndHorizontal => '┬',
            BoxDrawing::LeftHeavyAndRightDownLight => '┭',
            BoxDrawing::RightHeavyAndLeftDownLight => '┮',
            BoxDrawing::DownLightAndHorizontalHeavy => '┯',
            BoxDrawing::DownHeavyAndHorizontalLight => '┰',
            BoxDrawing::RightLightAndLeftDownHeavy => '┱',
            BoxDrawing::LeftLightAndRightDownHeavy => '┲',
            BoxDrawing::HeavyDownAndHorizontal => '┳',
            BoxDrawing::LightUpAndHorizontal => '┴',
            BoxDrawing::LeftHeavyAndRightUpLight => '┵',
            BoxDrawing::RightHeavyAndLeftUpLight => '┶',
            BoxDrawing::UpLightAndHorizontalHeavy => '┷',
            BoxDrawing::UpHeavyAndHorizontalLight => '┸',
            BoxDrawing::RightLightAndLeftUpHeavy => '┹',
            BoxDrawing::LeftLightAndRightUpHeavy => '┺',
            BoxDrawing::HeavyUpAndHorizontal => '┻',
            BoxDrawing::LightVerticalAndHorizontal => '┼',
            BoxDrawing::LeftHeavyAndRightVerticalLight => '┽',
            BoxDrawing::RightHeavyAndLeftVerticalLight => '┾',
            BoxDrawing::VerticalLightAndHorizontalHeavy => '┿',
            BoxDrawing::UpHeavyAndDownHorizontalLight => '╀',
            BoxDrawing::DownHeavyAndUpHorizontalLight => '╁',
            BoxDrawing::VerticalHeavyAndHorizontalLight => '╂',
            BoxDrawing::LeftUpHeavyAndRightDownLight => '╃',
            BoxDrawing::RightUpHeavyAndLeftDownLight => '╄',
            BoxDrawing::LeftDownHeavyAndRightUpLight => '╅',
            BoxDrawing::RightDownHeavyAndLeftUpLight => '╆',
            BoxDrawing::DownLightAndUpHorizontalHeavy => '╇',
            BoxDrawing::UpLightAndDownHorizontalHeavy => '╈',
            BoxDrawing::RightLightAndLeftVerticalHeavy => '╉',
            BoxDrawing::LeftLightAndRightVerticalHeavy => '╊',
            BoxDrawing::HeavyVerticalAndHorizontal => '╋',
            BoxDrawing::LightDoubleDashHorizontal => '╌',
            BoxDrawing::HeavyDoubleDashHorizontal => '╍',
            BoxDrawing::LightDoubleDashVertical => '╎',
            BoxDrawing::HeavyDoubleDashVertical => '╏',
            BoxDrawing::DoubleHorizontal => '═',
            BoxDrawing::DoubleVertical => '║',
            BoxDrawing::DownSingleAndRightDouble => '╒',
            BoxDrawing::DownDoubleAndRightSingle => '╓',
            BoxDrawing::DoubleDownAndRight => '╔',
            BoxDrawing::DownSingleAndLeftDouble => '╕',
            BoxDrawing::DownDoubleAndLeftSingle => '╖',
            BoxDrawing::DoubleDownAndLeft => '╗',
            BoxDrawing::UpSingleAndRightDouble => '╘',
            BoxDrawing::UpDoubleAndRightSingle => '╙',
            BoxDrawing::DoubleUpAndRight => '╚',
            BoxDrawing::UpSingleAndLeftDouble => '╛',
            BoxDrawing::UpDoubleAndLeftSingle => '╜',
            BoxDrawing::DoubleUpAndLeft => '╝',
            BoxDrawing::VerticalSingleAndRightDouble => '╞',
            BoxDrawing::VerticalDoubleAndRightSingle => '╟',
            BoxDrawing::DoubleVerticalAndRight => '╠',
            BoxDrawing::VerticalSingleAndLeftDouble => '╡',
            BoxDrawing::VerticalDoubleAndLeftSingle => '╢',
            BoxDrawing::DoubleVerticalAndLeft => '╣',
            BoxDrawing::DownSingleAndHorizontalDouble => '╤',
            BoxDrawing::DownDoubleAndHorizontalSingle => '╥',
            BoxDrawing::DoubleDownAndHorizontal => '╦',
            BoxDrawing::UpSingleAndHorizontalDouble => '╧',
            BoxDrawing::UpDoubleAndHorizontalSingle => '╨',
            BoxDrawing::DoubleUpAndHorizontal => '╩',
            BoxDrawing::VerticalSingleAndHorizontalDouble => '╪',
            BoxDrawing::VerticalDoubleAndHorizontalSingle => '╫',
            BoxDrawing::DoubleVerticalAndHorizontal => '╬',
            BoxDrawing::LightArcDownAndRight => '╭',
            BoxDrawing::LightArcDownAndLeft => '╮',
            BoxDrawing::LightArcUpAndLeft => '╯',
            BoxDrawing::LightArcUpAndRight => '╰',
            BoxDrawing::LightDiagonalUpperRightToLowerLeft => '╱',
            BoxDrawing::LightDiagonalUpperLeftToLowerRight => '╲',
            BoxDrawing::LightDiagonalCross => '╳',
            BoxDrawing::LightLeft => '╴',
            BoxDrawing::LightUp => '╵',
            BoxDrawing::LightRight => '╶',
            BoxDrawing::LightDown => '╷',
            BoxDrawing::HeavyLeft => '╸',
            BoxDrawing::HeavyUp => '╹',
            BoxDrawing::HeavyRight => '╺',
            BoxDrawing::HeavyDown => '╻',
            BoxDrawing::LightLeftAndHeavyRight => '╼',
            BoxDrawing::LightUpAndHeavyDown => '╽',
            BoxDrawing::HeavyLeftAndLightRight => '╾',
        }
    }
}

impl std::convert::TryFrom<char> for BoxDrawing {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '─' => Ok(BoxDrawing::LightHorizontal),
            '━' => Ok(BoxDrawing::HeavyHorizontal),
            '│' => Ok(BoxDrawing::LightVertical),
            '┃' => Ok(BoxDrawing::HeavyVertical),
            '┄' => Ok(BoxDrawing::LightTripleDashHorizontal),
            '┅' => Ok(BoxDrawing::HeavyTripleDashHorizontal),
            '┆' => Ok(BoxDrawing::LightTripleDashVertical),
            '┇' => Ok(BoxDrawing::HeavyTripleDashVertical),
            '┈' => Ok(BoxDrawing::LightQuadrupleDashHorizontal),
            '┉' => Ok(BoxDrawing::HeavyQuadrupleDashHorizontal),
            '┊' => Ok(BoxDrawing::LightQuadrupleDashVertical),
            '┋' => Ok(BoxDrawing::HeavyQuadrupleDashVertical),
            '┌' => Ok(BoxDrawing::LightDownAndRight),
            '┍' => Ok(BoxDrawing::DownLightAndRightHeavy),
            '┎' => Ok(BoxDrawing::DownHeavyAndRightLight),
            '┏' => Ok(BoxDrawing::HeavyDownAndRight),
            '┐' => Ok(BoxDrawing::LightDownAndLeft),
            '┑' => Ok(BoxDrawing::DownLightAndLeftHeavy),
            '┒' => Ok(BoxDrawing::DownHeavyAndLeftLight),
            '┓' => Ok(BoxDrawing::HeavyDownAndLeft),
            '└' => Ok(BoxDrawing::LightUpAndRight),
            '┕' => Ok(BoxDrawing::UpLightAndRightHeavy),
            '┖' => Ok(BoxDrawing::UpHeavyAndRightLight),
            '┗' => Ok(BoxDrawing::HeavyUpAndRight),
            '┘' => Ok(BoxDrawing::LightUpAndLeft),
            '┙' => Ok(BoxDrawing::UpLightAndLeftHeavy),
            '┚' => Ok(BoxDrawing::UpHeavyAndLeftLight),
            '┛' => Ok(BoxDrawing::HeavyUpAndLeft),
            '├' => Ok(BoxDrawing::LightVerticalAndRight),
            '┝' => Ok(BoxDrawing::VerticalLightAndRightHeavy),
            '┞' => Ok(BoxDrawing::UpHeavyAndRightDownLight),
            '┟' => Ok(BoxDrawing::DownHeavyAndRightUpLight),
            '┠' => Ok(BoxDrawing::VerticalHeavyAndRightLight),
            '┡' => Ok(BoxDrawing::DownLightAndRightUpHeavy),
            '┢' => Ok(BoxDrawing::UpLightAndRightDownHeavy),
            '┣' => Ok(BoxDrawing::HeavyVerticalAndRight),
            '┤' => Ok(BoxDrawing::LightVerticalAndLeft),
            '┥' => Ok(BoxDrawing::VerticalLightAndLeftHeavy),
            '┦' => Ok(BoxDrawing::UpHeavyAndLeftDownLight),
            '┧' => Ok(BoxDrawing::DownHeavyAndLeftUpLight),
            '┨' => Ok(BoxDrawing::VerticalHeavyAndLeftLight),
            '┩' => Ok(BoxDrawing::DownLightAndLeftUpHeavy),
            '┪' => Ok(BoxDrawing::UpLightAndLeftDownHeavy),
            '┫' => Ok(BoxDrawing::HeavyVerticalAndLeft),
            '┬' => Ok(BoxDrawing::LightDownAndHorizontal),
            '┭' => Ok(BoxDrawing::LeftHeavyAndRightDownLight),
            '┮' => Ok(BoxDrawing::RightHeavyAndLeftDownLight),
            '┯' => Ok(BoxDrawing::DownLightAndHorizontalHeavy),
            '┰' => Ok(BoxDrawing::DownHeavyAndHorizontalLight),
            '┱' => Ok(BoxDrawing::RightLightAndLeftDownHeavy),
            '┲' => Ok(BoxDrawing::LeftLightAndRightDownHeavy),
            '┳' => Ok(BoxDrawing::HeavyDownAndHorizontal),
            '┴' => Ok(BoxDrawing::LightUpAndHorizontal),
            '┵' => Ok(BoxDrawing::LeftHeavyAndRightUpLight),
            '┶' => Ok(BoxDrawing::RightHeavyAndLeftUpLight),
            '┷' => Ok(BoxDrawing::UpLightAndHorizontalHeavy),
            '┸' => Ok(BoxDrawing::UpHeavyAndHorizontalLight),
            '┹' => Ok(BoxDrawing::RightLightAndLeftUpHeavy),
            '┺' => Ok(BoxDrawing::LeftLightAndRightUpHeavy),
            '┻' => Ok(BoxDrawing::HeavyUpAndHorizontal),
            '┼' => Ok(BoxDrawing::LightVerticalAndHorizontal),
            '┽' => Ok(BoxDrawing::LeftHeavyAndRightVerticalLight),
            '┾' => Ok(BoxDrawing::RightHeavyAndLeftVerticalLight),
            '┿' => Ok(BoxDrawing::VerticalLightAndHorizontalHeavy),
            '╀' => Ok(BoxDrawing::UpHeavyAndDownHorizontalLight),
            '╁' => Ok(BoxDrawing::DownHeavyAndUpHorizontalLight),
            '╂' => Ok(BoxDrawing::VerticalHeavyAndHorizontalLight),
            '╃' => Ok(BoxDrawing::LeftUpHeavyAndRightDownLight),
            '╄' => Ok(BoxDrawing::RightUpHeavyAndLeftDownLight),
            '╅' => Ok(BoxDrawing::LeftDownHeavyAndRightUpLight),
            '╆' => Ok(BoxDrawing::RightDownHeavyAndLeftUpLight),
            '╇' => Ok(BoxDrawing::DownLightAndUpHorizontalHeavy),
            '╈' => Ok(BoxDrawing::UpLightAndDownHorizontalHeavy),
            '╉' => Ok(BoxDrawing::RightLightAndLeftVerticalHeavy),
            '╊' => Ok(BoxDrawing::LeftLightAndRightVerticalHeavy),
            '╋' => Ok(BoxDrawing::HeavyVerticalAndHorizontal),
            '╌' => Ok(BoxDrawing::LightDoubleDashHorizontal),
            '╍' => Ok(BoxDrawing::HeavyDoubleDashHorizontal),
            '╎' => Ok(BoxDrawing::LightDoubleDashVertical),
            '╏' => Ok(BoxDrawing::HeavyDoubleDashVertical),
            '═' => Ok(BoxDrawing::DoubleHorizontal),
            '║' => Ok(BoxDrawing::DoubleVertical),
            '╒' => Ok(BoxDrawing::DownSingleAndRightDouble),
            '╓' => Ok(BoxDrawing::DownDoubleAndRightSingle),
            '╔' => Ok(BoxDrawing::DoubleDownAndRight),
            '╕' => Ok(BoxDrawing::DownSingleAndLeftDouble),
            '╖' => Ok(BoxDrawing::DownDoubleAndLeftSingle),
            '╗' => Ok(BoxDrawing::DoubleDownAndLeft),
            '╘' => Ok(BoxDrawing::UpSingleAndRightDouble),
            '╙' => Ok(BoxDrawing::UpDoubleAndRightSingle),
            '╚' => Ok(BoxDrawing::DoubleUpAndRight),
            '╛' => Ok(BoxDrawing::UpSingleAndLeftDouble),
            '╜' => Ok(BoxDrawing::UpDoubleAndLeftSingle),
            '╝' => Ok(BoxDrawing::DoubleUpAndLeft),
            '╞' => Ok(BoxDrawing::VerticalSingleAndRightDouble),
            '╟' => Ok(BoxDrawing::VerticalDoubleAndRightSingle),
            '╠' => Ok(BoxDrawing::DoubleVerticalAndRight),
            '╡' => Ok(BoxDrawing::VerticalSingleAndLeftDouble),
            '╢' => Ok(BoxDrawing::VerticalDoubleAndLeftSingle),
            '╣' => Ok(BoxDrawing::DoubleVerticalAndLeft),
            '╤' => Ok(BoxDrawing::DownSingleAndHorizontalDouble),
            '╥' => Ok(BoxDrawing::DownDoubleAndHorizontalSingle),
            '╦' => Ok(BoxDrawing::DoubleDownAndHorizontal),
            '╧' => Ok(BoxDrawing::UpSingleAndHorizontalDouble),
            '╨' => Ok(BoxDrawing::UpDoubleAndHorizontalSingle),
            '╩' => Ok(BoxDrawing::DoubleUpAndHorizontal),
            '╪' => Ok(BoxDrawing::VerticalSingleAndHorizontalDouble),
            '╫' => Ok(BoxDrawing::VerticalDoubleAndHorizontalSingle),
            '╬' => Ok(BoxDrawing::DoubleVerticalAndHorizontal),
            '╭' => Ok(BoxDrawing::LightArcDownAndRight),
            '╮' => Ok(BoxDrawing::LightArcDownAndLeft),
            '╯' => Ok(BoxDrawing::LightArcUpAndLeft),
            '╰' => Ok(BoxDrawing::LightArcUpAndRight),
            '╱' => Ok(BoxDrawing::LightDiagonalUpperRightToLowerLeft),
            '╲' => Ok(BoxDrawing::LightDiagonalUpperLeftToLowerRight),
            '╳' => Ok(BoxDrawing::LightDiagonalCross),
            '╴' => Ok(BoxDrawing::LightLeft),
            '╵' => Ok(BoxDrawing::LightUp),
            '╶' => Ok(BoxDrawing::LightRight),
            '╷' => Ok(BoxDrawing::LightDown),
            '╸' => Ok(BoxDrawing::HeavyLeft),
            '╹' => Ok(BoxDrawing::HeavyUp),
            '╺' => Ok(BoxDrawing::HeavyRight),
            '╻' => Ok(BoxDrawing::HeavyDown),
            '╼' => Ok(BoxDrawing::LightLeftAndHeavyRight),
            '╽' => Ok(BoxDrawing::LightUpAndHeavyDown),
            '╾' => Ok(BoxDrawing::HeavyLeftAndLightRight),
            _ => Err(()),
        }
    }
}

impl Into<u32> for BoxDrawing {
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

impl std::convert::TryFrom<u32> for BoxDrawing {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for BoxDrawing {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl BoxDrawing {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        BoxDrawing::LightHorizontal
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("BoxDrawing{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
