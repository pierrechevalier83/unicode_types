/// \u{2500} → \u{257f}\
///\
/// ─ ━ │ ┃ ┄ ┅ ┆ ┇ ┈ ┉ ┊ ┋ ┌ ┍ ┎ ┏
/// ┐ ┑ ┒ ┓ └ ┕ ┖ ┗ ┘ ┙ ┚ ┛ ├ ┝ ┞ ┟
/// ┠ ┡ ┢ ┣ ┤ ┥ ┦ ┧ ┨ ┩ ┪ ┫ ┬ ┭ ┮ ┯
/// ┰ ┱ ┲ ┳ ┴ ┵ ┶ ┷ ┸ ┹ ┺ ┻ ┼ ┽ ┾ ┿
/// ╀ ╁ ╂ ╃ ╄ ╅ ╆ ╇ ╈ ╉ ╊ ╋ ╌ ╍ ╎ ╏
/// ═ ║ ╒ ╓ ╔ ╕ ╖ ╗ ╘ ╙ ╚ ╛ ╜ ╝ ╞ ╟
/// ╠ ╡ ╢ ╣ ╤ ╥ ╦ ╧ ╨ ╩ ╪ ╫ ╬ ╭ ╮ ╯
/// ╰ ╱ ╲ ╳ ╴ ╵ ╶ ╷ ╸ ╹ ╺ ╻ ╼ ╽ ╾
pub mod constants {
    /// \u{2500}: '─'
    pub const LIGHT_HORIZONTAL: char = '─';
    /// \u{2501}: '━'
    pub const HEAVY_HORIZONTAL: char = '━';
    /// \u{2502}: '│'
    pub const LIGHT_VERTICAL: char = '│';
    /// \u{2503}: '┃'
    pub const HEAVY_VERTICAL: char = '┃';
    /// \u{2504}: '┄'
    pub const LIGHT_TRIPLE_DASH_HORIZONTAL: char = '┄';
    /// \u{2505}: '┅'
    pub const HEAVY_TRIPLE_DASH_HORIZONTAL: char = '┅';
    /// \u{2506}: '┆'
    pub const LIGHT_TRIPLE_DASH_VERTICAL: char = '┆';
    /// \u{2507}: '┇'
    pub const HEAVY_TRIPLE_DASH_VERTICAL: char = '┇';
    /// \u{2508}: '┈'
    pub const LIGHT_QUADRUPLE_DASH_HORIZONTAL: char = '┈';
    /// \u{2509}: '┉'
    pub const HEAVY_QUADRUPLE_DASH_HORIZONTAL: char = '┉';
    /// \u{250a}: '┊'
    pub const LIGHT_QUADRUPLE_DASH_VERTICAL: char = '┊';
    /// \u{250b}: '┋'
    pub const HEAVY_QUADRUPLE_DASH_VERTICAL: char = '┋';
    /// \u{250c}: '┌'
    pub const LIGHT_DOWN_AND_RIGHT: char = '┌';
    /// \u{250d}: '┍'
    pub const DOWN_LIGHT_AND_RIGHT_HEAVY: char = '┍';
    /// \u{250e}: '┎'
    pub const DOWN_HEAVY_AND_RIGHT_LIGHT: char = '┎';
    /// \u{250f}: '┏'
    pub const HEAVY_DOWN_AND_RIGHT: char = '┏';
    /// \u{2510}: '┐'
    pub const LIGHT_DOWN_AND_LEFT: char = '┐';
    /// \u{2511}: '┑'
    pub const DOWN_LIGHT_AND_LEFT_HEAVY: char = '┑';
    /// \u{2512}: '┒'
    pub const DOWN_HEAVY_AND_LEFT_LIGHT: char = '┒';
    /// \u{2513}: '┓'
    pub const HEAVY_DOWN_AND_LEFT: char = '┓';
    /// \u{2514}: '└'
    pub const LIGHT_UP_AND_RIGHT: char = '└';
    /// \u{2515}: '┕'
    pub const UP_LIGHT_AND_RIGHT_HEAVY: char = '┕';
    /// \u{2516}: '┖'
    pub const UP_HEAVY_AND_RIGHT_LIGHT: char = '┖';
    /// \u{2517}: '┗'
    pub const HEAVY_UP_AND_RIGHT: char = '┗';
    /// \u{2518}: '┘'
    pub const LIGHT_UP_AND_LEFT: char = '┘';
    /// \u{2519}: '┙'
    pub const UP_LIGHT_AND_LEFT_HEAVY: char = '┙';
    /// \u{251a}: '┚'
    pub const UP_HEAVY_AND_LEFT_LIGHT: char = '┚';
    /// \u{251b}: '┛'
    pub const HEAVY_UP_AND_LEFT: char = '┛';
    /// \u{251c}: '├'
    pub const LIGHT_VERTICAL_AND_RIGHT: char = '├';
    /// \u{251d}: '┝'
    pub const VERTICAL_LIGHT_AND_RIGHT_HEAVY: char = '┝';
    /// \u{251e}: '┞'
    pub const UP_HEAVY_AND_RIGHT_DOWN_LIGHT: char = '┞';
    /// \u{251f}: '┟'
    pub const DOWN_HEAVY_AND_RIGHT_UP_LIGHT: char = '┟';
    /// \u{2520}: '┠'
    pub const VERTICAL_HEAVY_AND_RIGHT_LIGHT: char = '┠';
    /// \u{2521}: '┡'
    pub const DOWN_LIGHT_AND_RIGHT_UP_HEAVY: char = '┡';
    /// \u{2522}: '┢'
    pub const UP_LIGHT_AND_RIGHT_DOWN_HEAVY: char = '┢';
    /// \u{2523}: '┣'
    pub const HEAVY_VERTICAL_AND_RIGHT: char = '┣';
    /// \u{2524}: '┤'
    pub const LIGHT_VERTICAL_AND_LEFT: char = '┤';
    /// \u{2525}: '┥'
    pub const VERTICAL_LIGHT_AND_LEFT_HEAVY: char = '┥';
    /// \u{2526}: '┦'
    pub const UP_HEAVY_AND_LEFT_DOWN_LIGHT: char = '┦';
    /// \u{2527}: '┧'
    pub const DOWN_HEAVY_AND_LEFT_UP_LIGHT: char = '┧';
    /// \u{2528}: '┨'
    pub const VERTICAL_HEAVY_AND_LEFT_LIGHT: char = '┨';
    /// \u{2529}: '┩'
    pub const DOWN_LIGHT_AND_LEFT_UP_HEAVY: char = '┩';
    /// \u{252a}: '┪'
    pub const UP_LIGHT_AND_LEFT_DOWN_HEAVY: char = '┪';
    /// \u{252b}: '┫'
    pub const HEAVY_VERTICAL_AND_LEFT: char = '┫';
    /// \u{252c}: '┬'
    pub const LIGHT_DOWN_AND_HORIZONTAL: char = '┬';
    /// \u{252d}: '┭'
    pub const LEFT_HEAVY_AND_RIGHT_DOWN_LIGHT: char = '┭';
    /// \u{252e}: '┮'
    pub const RIGHT_HEAVY_AND_LEFT_DOWN_LIGHT: char = '┮';
    /// \u{252f}: '┯'
    pub const DOWN_LIGHT_AND_HORIZONTAL_HEAVY: char = '┯';
    /// \u{2530}: '┰'
    pub const DOWN_HEAVY_AND_HORIZONTAL_LIGHT: char = '┰';
    /// \u{2531}: '┱'
    pub const RIGHT_LIGHT_AND_LEFT_DOWN_HEAVY: char = '┱';
    /// \u{2532}: '┲'
    pub const LEFT_LIGHT_AND_RIGHT_DOWN_HEAVY: char = '┲';
    /// \u{2533}: '┳'
    pub const HEAVY_DOWN_AND_HORIZONTAL: char = '┳';
    /// \u{2534}: '┴'
    pub const LIGHT_UP_AND_HORIZONTAL: char = '┴';
    /// \u{2535}: '┵'
    pub const LEFT_HEAVY_AND_RIGHT_UP_LIGHT: char = '┵';
    /// \u{2536}: '┶'
    pub const RIGHT_HEAVY_AND_LEFT_UP_LIGHT: char = '┶';
    /// \u{2537}: '┷'
    pub const UP_LIGHT_AND_HORIZONTAL_HEAVY: char = '┷';
    /// \u{2538}: '┸'
    pub const UP_HEAVY_AND_HORIZONTAL_LIGHT: char = '┸';
    /// \u{2539}: '┹'
    pub const RIGHT_LIGHT_AND_LEFT_UP_HEAVY: char = '┹';
    /// \u{253a}: '┺'
    pub const LEFT_LIGHT_AND_RIGHT_UP_HEAVY: char = '┺';
    /// \u{253b}: '┻'
    pub const HEAVY_UP_AND_HORIZONTAL: char = '┻';
    /// \u{253c}: '┼'
    pub const LIGHT_VERTICAL_AND_HORIZONTAL: char = '┼';
    /// \u{253d}: '┽'
    pub const LEFT_HEAVY_AND_RIGHT_VERTICAL_LIGHT: char = '┽';
    /// \u{253e}: '┾'
    pub const RIGHT_HEAVY_AND_LEFT_VERTICAL_LIGHT: char = '┾';
    /// \u{253f}: '┿'
    pub const VERTICAL_LIGHT_AND_HORIZONTAL_HEAVY: char = '┿';
    /// \u{2540}: '╀'
    pub const UP_HEAVY_AND_DOWN_HORIZONTAL_LIGHT: char = '╀';
    /// \u{2541}: '╁'
    pub const DOWN_HEAVY_AND_UP_HORIZONTAL_LIGHT: char = '╁';
    /// \u{2542}: '╂'
    pub const VERTICAL_HEAVY_AND_HORIZONTAL_LIGHT: char = '╂';
    /// \u{2543}: '╃'
    pub const LEFT_UP_HEAVY_AND_RIGHT_DOWN_LIGHT: char = '╃';
    /// \u{2544}: '╄'
    pub const RIGHT_UP_HEAVY_AND_LEFT_DOWN_LIGHT: char = '╄';
    /// \u{2545}: '╅'
    pub const LEFT_DOWN_HEAVY_AND_RIGHT_UP_LIGHT: char = '╅';
    /// \u{2546}: '╆'
    pub const RIGHT_DOWN_HEAVY_AND_LEFT_UP_LIGHT: char = '╆';
    /// \u{2547}: '╇'
    pub const DOWN_LIGHT_AND_UP_HORIZONTAL_HEAVY: char = '╇';
    /// \u{2548}: '╈'
    pub const UP_LIGHT_AND_DOWN_HORIZONTAL_HEAVY: char = '╈';
    /// \u{2549}: '╉'
    pub const RIGHT_LIGHT_AND_LEFT_VERTICAL_HEAVY: char = '╉';
    /// \u{254a}: '╊'
    pub const LEFT_LIGHT_AND_RIGHT_VERTICAL_HEAVY: char = '╊';
    /// \u{254b}: '╋'
    pub const HEAVY_VERTICAL_AND_HORIZONTAL: char = '╋';
    /// \u{254c}: '╌'
    pub const LIGHT_DOUBLE_DASH_HORIZONTAL: char = '╌';
    /// \u{254d}: '╍'
    pub const HEAVY_DOUBLE_DASH_HORIZONTAL: char = '╍';
    /// \u{254e}: '╎'
    pub const LIGHT_DOUBLE_DASH_VERTICAL: char = '╎';
    /// \u{254f}: '╏'
    pub const HEAVY_DOUBLE_DASH_VERTICAL: char = '╏';
    /// \u{2550}: '═'
    pub const DOUBLE_HORIZONTAL: char = '═';
    /// \u{2551}: '║'
    pub const DOUBLE_VERTICAL: char = '║';
    /// \u{2552}: '╒'
    pub const DOWN_SINGLE_AND_RIGHT_DOUBLE: char = '╒';
    /// \u{2553}: '╓'
    pub const DOWN_DOUBLE_AND_RIGHT_SINGLE: char = '╓';
    /// \u{2554}: '╔'
    pub const DOUBLE_DOWN_AND_RIGHT: char = '╔';
    /// \u{2555}: '╕'
    pub const DOWN_SINGLE_AND_LEFT_DOUBLE: char = '╕';
    /// \u{2556}: '╖'
    pub const DOWN_DOUBLE_AND_LEFT_SINGLE: char = '╖';
    /// \u{2557}: '╗'
    pub const DOUBLE_DOWN_AND_LEFT: char = '╗';
    /// \u{2558}: '╘'
    pub const UP_SINGLE_AND_RIGHT_DOUBLE: char = '╘';
    /// \u{2559}: '╙'
    pub const UP_DOUBLE_AND_RIGHT_SINGLE: char = '╙';
    /// \u{255a}: '╚'
    pub const DOUBLE_UP_AND_RIGHT: char = '╚';
    /// \u{255b}: '╛'
    pub const UP_SINGLE_AND_LEFT_DOUBLE: char = '╛';
    /// \u{255c}: '╜'
    pub const UP_DOUBLE_AND_LEFT_SINGLE: char = '╜';
    /// \u{255d}: '╝'
    pub const DOUBLE_UP_AND_LEFT: char = '╝';
    /// \u{255e}: '╞'
    pub const VERTICAL_SINGLE_AND_RIGHT_DOUBLE: char = '╞';
    /// \u{255f}: '╟'
    pub const VERTICAL_DOUBLE_AND_RIGHT_SINGLE: char = '╟';
    /// \u{2560}: '╠'
    pub const DOUBLE_VERTICAL_AND_RIGHT: char = '╠';
    /// \u{2561}: '╡'
    pub const VERTICAL_SINGLE_AND_LEFT_DOUBLE: char = '╡';
    /// \u{2562}: '╢'
    pub const VERTICAL_DOUBLE_AND_LEFT_SINGLE: char = '╢';
    /// \u{2563}: '╣'
    pub const DOUBLE_VERTICAL_AND_LEFT: char = '╣';
    /// \u{2564}: '╤'
    pub const DOWN_SINGLE_AND_HORIZONTAL_DOUBLE: char = '╤';
    /// \u{2565}: '╥'
    pub const DOWN_DOUBLE_AND_HORIZONTAL_SINGLE: char = '╥';
    /// \u{2566}: '╦'
    pub const DOUBLE_DOWN_AND_HORIZONTAL: char = '╦';
    /// \u{2567}: '╧'
    pub const UP_SINGLE_AND_HORIZONTAL_DOUBLE: char = '╧';
    /// \u{2568}: '╨'
    pub const UP_DOUBLE_AND_HORIZONTAL_SINGLE: char = '╨';
    /// \u{2569}: '╩'
    pub const DOUBLE_UP_AND_HORIZONTAL: char = '╩';
    /// \u{256a}: '╪'
    pub const VERTICAL_SINGLE_AND_HORIZONTAL_DOUBLE: char = '╪';
    /// \u{256b}: '╫'
    pub const VERTICAL_DOUBLE_AND_HORIZONTAL_SINGLE: char = '╫';
    /// \u{256c}: '╬'
    pub const DOUBLE_VERTICAL_AND_HORIZONTAL: char = '╬';
    /// \u{256d}: '╭'
    pub const LIGHT_ARC_DOWN_AND_RIGHT: char = '╭';
    /// \u{256e}: '╮'
    pub const LIGHT_ARC_DOWN_AND_LEFT: char = '╮';
    /// \u{256f}: '╯'
    pub const LIGHT_ARC_UP_AND_LEFT: char = '╯';
    /// \u{2570}: '╰'
    pub const LIGHT_ARC_UP_AND_RIGHT: char = '╰';
    /// \u{2571}: '╱'
    pub const LIGHT_DIAGONAL_UPPER_RIGHT_TO_LOWER_LEFT: char = '╱';
    /// \u{2572}: '╲'
    pub const LIGHT_DIAGONAL_UPPER_LEFT_TO_LOWER_RIGHT: char = '╲';
    /// \u{2573}: '╳'
    pub const LIGHT_DIAGONAL_CROSS: char = '╳';
    /// \u{2574}: '╴'
    pub const LIGHT_LEFT: char = '╴';
    /// \u{2575}: '╵'
    pub const LIGHT_UP: char = '╵';
    /// \u{2576}: '╶'
    pub const LIGHT_RIGHT: char = '╶';
    /// \u{2577}: '╷'
    pub const LIGHT_DOWN: char = '╷';
    /// \u{2578}: '╸'
    pub const HEAVY_LEFT: char = '╸';
    /// \u{2579}: '╹'
    pub const HEAVY_UP: char = '╹';
    /// \u{257a}: '╺'
    pub const HEAVY_RIGHT: char = '╺';
    /// \u{257b}: '╻'
    pub const HEAVY_DOWN: char = '╻';
    /// \u{257c}: '╼'
    pub const LIGHT_LEFT_AND_HEAVY_RIGHT: char = '╼';
    /// \u{257d}: '╽'
    pub const LIGHT_UP_AND_HEAVY_DOWN: char = '╽';
    /// \u{257e}: '╾'
    pub const HEAVY_LEFT_AND_LIGHT_RIGHT: char = '╾';
}

/// \u{2500} → \u{257f}\
///\
/// ─ ━ │ ┃ ┄ ┅ ┆ ┇ ┈ ┉ ┊ ┋ ┌ ┍ ┎ ┏
/// ┐ ┑ ┒ ┓ └ ┕ ┖ ┗ ┘ ┙ ┚ ┛ ├ ┝ ┞ ┟
/// ┠ ┡ ┢ ┣ ┤ ┥ ┦ ┧ ┨ ┩ ┪ ┫ ┬ ┭ ┮ ┯
/// ┰ ┱ ┲ ┳ ┴ ┵ ┶ ┷ ┸ ┹ ┺ ┻ ┼ ┽ ┾ ┿
/// ╀ ╁ ╂ ╃ ╄ ╅ ╆ ╇ ╈ ╉ ╊ ╋ ╌ ╍ ╎ ╏
/// ═ ║ ╒ ╓ ╔ ╕ ╖ ╗ ╘ ╙ ╚ ╛ ╜ ╝ ╞ ╟
/// ╠ ╡ ╢ ╣ ╤ ╥ ╦ ╧ ╨ ╩ ╪ ╫ ╬ ╭ ╮ ╯
/// ╰ ╱ ╲ ╳ ╴ ╵ ╶ ╷ ╸ ╹ ╺ ╻ ╼ ╽ ╾
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
        use constants::*;
        match self {
            BoxDrawing::LightHorizontal => LIGHT_HORIZONTAL,
            BoxDrawing::HeavyHorizontal => HEAVY_HORIZONTAL,
            BoxDrawing::LightVertical => LIGHT_VERTICAL,
            BoxDrawing::HeavyVertical => HEAVY_VERTICAL,
            BoxDrawing::LightTripleDashHorizontal => LIGHT_TRIPLE_DASH_HORIZONTAL,
            BoxDrawing::HeavyTripleDashHorizontal => HEAVY_TRIPLE_DASH_HORIZONTAL,
            BoxDrawing::LightTripleDashVertical => LIGHT_TRIPLE_DASH_VERTICAL,
            BoxDrawing::HeavyTripleDashVertical => HEAVY_TRIPLE_DASH_VERTICAL,
            BoxDrawing::LightQuadrupleDashHorizontal => LIGHT_QUADRUPLE_DASH_HORIZONTAL,
            BoxDrawing::HeavyQuadrupleDashHorizontal => HEAVY_QUADRUPLE_DASH_HORIZONTAL,
            BoxDrawing::LightQuadrupleDashVertical => LIGHT_QUADRUPLE_DASH_VERTICAL,
            BoxDrawing::HeavyQuadrupleDashVertical => HEAVY_QUADRUPLE_DASH_VERTICAL,
            BoxDrawing::LightDownAndRight => LIGHT_DOWN_AND_RIGHT,
            BoxDrawing::DownLightAndRightHeavy => DOWN_LIGHT_AND_RIGHT_HEAVY,
            BoxDrawing::DownHeavyAndRightLight => DOWN_HEAVY_AND_RIGHT_LIGHT,
            BoxDrawing::HeavyDownAndRight => HEAVY_DOWN_AND_RIGHT,
            BoxDrawing::LightDownAndLeft => LIGHT_DOWN_AND_LEFT,
            BoxDrawing::DownLightAndLeftHeavy => DOWN_LIGHT_AND_LEFT_HEAVY,
            BoxDrawing::DownHeavyAndLeftLight => DOWN_HEAVY_AND_LEFT_LIGHT,
            BoxDrawing::HeavyDownAndLeft => HEAVY_DOWN_AND_LEFT,
            BoxDrawing::LightUpAndRight => LIGHT_UP_AND_RIGHT,
            BoxDrawing::UpLightAndRightHeavy => UP_LIGHT_AND_RIGHT_HEAVY,
            BoxDrawing::UpHeavyAndRightLight => UP_HEAVY_AND_RIGHT_LIGHT,
            BoxDrawing::HeavyUpAndRight => HEAVY_UP_AND_RIGHT,
            BoxDrawing::LightUpAndLeft => LIGHT_UP_AND_LEFT,
            BoxDrawing::UpLightAndLeftHeavy => UP_LIGHT_AND_LEFT_HEAVY,
            BoxDrawing::UpHeavyAndLeftLight => UP_HEAVY_AND_LEFT_LIGHT,
            BoxDrawing::HeavyUpAndLeft => HEAVY_UP_AND_LEFT,
            BoxDrawing::LightVerticalAndRight => LIGHT_VERTICAL_AND_RIGHT,
            BoxDrawing::VerticalLightAndRightHeavy => VERTICAL_LIGHT_AND_RIGHT_HEAVY,
            BoxDrawing::UpHeavyAndRightDownLight => UP_HEAVY_AND_RIGHT_DOWN_LIGHT,
            BoxDrawing::DownHeavyAndRightUpLight => DOWN_HEAVY_AND_RIGHT_UP_LIGHT,
            BoxDrawing::VerticalHeavyAndRightLight => VERTICAL_HEAVY_AND_RIGHT_LIGHT,
            BoxDrawing::DownLightAndRightUpHeavy => DOWN_LIGHT_AND_RIGHT_UP_HEAVY,
            BoxDrawing::UpLightAndRightDownHeavy => UP_LIGHT_AND_RIGHT_DOWN_HEAVY,
            BoxDrawing::HeavyVerticalAndRight => HEAVY_VERTICAL_AND_RIGHT,
            BoxDrawing::LightVerticalAndLeft => LIGHT_VERTICAL_AND_LEFT,
            BoxDrawing::VerticalLightAndLeftHeavy => VERTICAL_LIGHT_AND_LEFT_HEAVY,
            BoxDrawing::UpHeavyAndLeftDownLight => UP_HEAVY_AND_LEFT_DOWN_LIGHT,
            BoxDrawing::DownHeavyAndLeftUpLight => DOWN_HEAVY_AND_LEFT_UP_LIGHT,
            BoxDrawing::VerticalHeavyAndLeftLight => VERTICAL_HEAVY_AND_LEFT_LIGHT,
            BoxDrawing::DownLightAndLeftUpHeavy => DOWN_LIGHT_AND_LEFT_UP_HEAVY,
            BoxDrawing::UpLightAndLeftDownHeavy => UP_LIGHT_AND_LEFT_DOWN_HEAVY,
            BoxDrawing::HeavyVerticalAndLeft => HEAVY_VERTICAL_AND_LEFT,
            BoxDrawing::LightDownAndHorizontal => LIGHT_DOWN_AND_HORIZONTAL,
            BoxDrawing::LeftHeavyAndRightDownLight => LEFT_HEAVY_AND_RIGHT_DOWN_LIGHT,
            BoxDrawing::RightHeavyAndLeftDownLight => RIGHT_HEAVY_AND_LEFT_DOWN_LIGHT,
            BoxDrawing::DownLightAndHorizontalHeavy => DOWN_LIGHT_AND_HORIZONTAL_HEAVY,
            BoxDrawing::DownHeavyAndHorizontalLight => DOWN_HEAVY_AND_HORIZONTAL_LIGHT,
            BoxDrawing::RightLightAndLeftDownHeavy => RIGHT_LIGHT_AND_LEFT_DOWN_HEAVY,
            BoxDrawing::LeftLightAndRightDownHeavy => LEFT_LIGHT_AND_RIGHT_DOWN_HEAVY,
            BoxDrawing::HeavyDownAndHorizontal => HEAVY_DOWN_AND_HORIZONTAL,
            BoxDrawing::LightUpAndHorizontal => LIGHT_UP_AND_HORIZONTAL,
            BoxDrawing::LeftHeavyAndRightUpLight => LEFT_HEAVY_AND_RIGHT_UP_LIGHT,
            BoxDrawing::RightHeavyAndLeftUpLight => RIGHT_HEAVY_AND_LEFT_UP_LIGHT,
            BoxDrawing::UpLightAndHorizontalHeavy => UP_LIGHT_AND_HORIZONTAL_HEAVY,
            BoxDrawing::UpHeavyAndHorizontalLight => UP_HEAVY_AND_HORIZONTAL_LIGHT,
            BoxDrawing::RightLightAndLeftUpHeavy => RIGHT_LIGHT_AND_LEFT_UP_HEAVY,
            BoxDrawing::LeftLightAndRightUpHeavy => LEFT_LIGHT_AND_RIGHT_UP_HEAVY,
            BoxDrawing::HeavyUpAndHorizontal => HEAVY_UP_AND_HORIZONTAL,
            BoxDrawing::LightVerticalAndHorizontal => LIGHT_VERTICAL_AND_HORIZONTAL,
            BoxDrawing::LeftHeavyAndRightVerticalLight => LEFT_HEAVY_AND_RIGHT_VERTICAL_LIGHT,
            BoxDrawing::RightHeavyAndLeftVerticalLight => RIGHT_HEAVY_AND_LEFT_VERTICAL_LIGHT,
            BoxDrawing::VerticalLightAndHorizontalHeavy => VERTICAL_LIGHT_AND_HORIZONTAL_HEAVY,
            BoxDrawing::UpHeavyAndDownHorizontalLight => UP_HEAVY_AND_DOWN_HORIZONTAL_LIGHT,
            BoxDrawing::DownHeavyAndUpHorizontalLight => DOWN_HEAVY_AND_UP_HORIZONTAL_LIGHT,
            BoxDrawing::VerticalHeavyAndHorizontalLight => VERTICAL_HEAVY_AND_HORIZONTAL_LIGHT,
            BoxDrawing::LeftUpHeavyAndRightDownLight => LEFT_UP_HEAVY_AND_RIGHT_DOWN_LIGHT,
            BoxDrawing::RightUpHeavyAndLeftDownLight => RIGHT_UP_HEAVY_AND_LEFT_DOWN_LIGHT,
            BoxDrawing::LeftDownHeavyAndRightUpLight => LEFT_DOWN_HEAVY_AND_RIGHT_UP_LIGHT,
            BoxDrawing::RightDownHeavyAndLeftUpLight => RIGHT_DOWN_HEAVY_AND_LEFT_UP_LIGHT,
            BoxDrawing::DownLightAndUpHorizontalHeavy => DOWN_LIGHT_AND_UP_HORIZONTAL_HEAVY,
            BoxDrawing::UpLightAndDownHorizontalHeavy => UP_LIGHT_AND_DOWN_HORIZONTAL_HEAVY,
            BoxDrawing::RightLightAndLeftVerticalHeavy => RIGHT_LIGHT_AND_LEFT_VERTICAL_HEAVY,
            BoxDrawing::LeftLightAndRightVerticalHeavy => LEFT_LIGHT_AND_RIGHT_VERTICAL_HEAVY,
            BoxDrawing::HeavyVerticalAndHorizontal => HEAVY_VERTICAL_AND_HORIZONTAL,
            BoxDrawing::LightDoubleDashHorizontal => LIGHT_DOUBLE_DASH_HORIZONTAL,
            BoxDrawing::HeavyDoubleDashHorizontal => HEAVY_DOUBLE_DASH_HORIZONTAL,
            BoxDrawing::LightDoubleDashVertical => LIGHT_DOUBLE_DASH_VERTICAL,
            BoxDrawing::HeavyDoubleDashVertical => HEAVY_DOUBLE_DASH_VERTICAL,
            BoxDrawing::DoubleHorizontal => DOUBLE_HORIZONTAL,
            BoxDrawing::DoubleVertical => DOUBLE_VERTICAL,
            BoxDrawing::DownSingleAndRightDouble => DOWN_SINGLE_AND_RIGHT_DOUBLE,
            BoxDrawing::DownDoubleAndRightSingle => DOWN_DOUBLE_AND_RIGHT_SINGLE,
            BoxDrawing::DoubleDownAndRight => DOUBLE_DOWN_AND_RIGHT,
            BoxDrawing::DownSingleAndLeftDouble => DOWN_SINGLE_AND_LEFT_DOUBLE,
            BoxDrawing::DownDoubleAndLeftSingle => DOWN_DOUBLE_AND_LEFT_SINGLE,
            BoxDrawing::DoubleDownAndLeft => DOUBLE_DOWN_AND_LEFT,
            BoxDrawing::UpSingleAndRightDouble => UP_SINGLE_AND_RIGHT_DOUBLE,
            BoxDrawing::UpDoubleAndRightSingle => UP_DOUBLE_AND_RIGHT_SINGLE,
            BoxDrawing::DoubleUpAndRight => DOUBLE_UP_AND_RIGHT,
            BoxDrawing::UpSingleAndLeftDouble => UP_SINGLE_AND_LEFT_DOUBLE,
            BoxDrawing::UpDoubleAndLeftSingle => UP_DOUBLE_AND_LEFT_SINGLE,
            BoxDrawing::DoubleUpAndLeft => DOUBLE_UP_AND_LEFT,
            BoxDrawing::VerticalSingleAndRightDouble => VERTICAL_SINGLE_AND_RIGHT_DOUBLE,
            BoxDrawing::VerticalDoubleAndRightSingle => VERTICAL_DOUBLE_AND_RIGHT_SINGLE,
            BoxDrawing::DoubleVerticalAndRight => DOUBLE_VERTICAL_AND_RIGHT,
            BoxDrawing::VerticalSingleAndLeftDouble => VERTICAL_SINGLE_AND_LEFT_DOUBLE,
            BoxDrawing::VerticalDoubleAndLeftSingle => VERTICAL_DOUBLE_AND_LEFT_SINGLE,
            BoxDrawing::DoubleVerticalAndLeft => DOUBLE_VERTICAL_AND_LEFT,
            BoxDrawing::DownSingleAndHorizontalDouble => DOWN_SINGLE_AND_HORIZONTAL_DOUBLE,
            BoxDrawing::DownDoubleAndHorizontalSingle => DOWN_DOUBLE_AND_HORIZONTAL_SINGLE,
            BoxDrawing::DoubleDownAndHorizontal => DOUBLE_DOWN_AND_HORIZONTAL,
            BoxDrawing::UpSingleAndHorizontalDouble => UP_SINGLE_AND_HORIZONTAL_DOUBLE,
            BoxDrawing::UpDoubleAndHorizontalSingle => UP_DOUBLE_AND_HORIZONTAL_SINGLE,
            BoxDrawing::DoubleUpAndHorizontal => DOUBLE_UP_AND_HORIZONTAL,
            BoxDrawing::VerticalSingleAndHorizontalDouble => VERTICAL_SINGLE_AND_HORIZONTAL_DOUBLE,
            BoxDrawing::VerticalDoubleAndHorizontalSingle => VERTICAL_DOUBLE_AND_HORIZONTAL_SINGLE,
            BoxDrawing::DoubleVerticalAndHorizontal => DOUBLE_VERTICAL_AND_HORIZONTAL,
            BoxDrawing::LightArcDownAndRight => LIGHT_ARC_DOWN_AND_RIGHT,
            BoxDrawing::LightArcDownAndLeft => LIGHT_ARC_DOWN_AND_LEFT,
            BoxDrawing::LightArcUpAndLeft => LIGHT_ARC_UP_AND_LEFT,
            BoxDrawing::LightArcUpAndRight => LIGHT_ARC_UP_AND_RIGHT,
            BoxDrawing::LightDiagonalUpperRightToLowerLeft => LIGHT_DIAGONAL_UPPER_RIGHT_TO_LOWER_LEFT,
            BoxDrawing::LightDiagonalUpperLeftToLowerRight => LIGHT_DIAGONAL_UPPER_LEFT_TO_LOWER_RIGHT,
            BoxDrawing::LightDiagonalCross => LIGHT_DIAGONAL_CROSS,
            BoxDrawing::LightLeft => LIGHT_LEFT,
            BoxDrawing::LightUp => LIGHT_UP,
            BoxDrawing::LightRight => LIGHT_RIGHT,
            BoxDrawing::LightDown => LIGHT_DOWN,
            BoxDrawing::HeavyLeft => HEAVY_LEFT,
            BoxDrawing::HeavyUp => HEAVY_UP,
            BoxDrawing::HeavyRight => HEAVY_RIGHT,
            BoxDrawing::HeavyDown => HEAVY_DOWN,
            BoxDrawing::LightLeftAndHeavyRight => LIGHT_LEFT_AND_HEAVY_RIGHT,
            BoxDrawing::LightUpAndHeavyDown => LIGHT_UP_AND_HEAVY_DOWN,
            BoxDrawing::HeavyLeftAndLightRight => HEAVY_LEFT_AND_LIGHT_RIGHT,
        }
    }
}

impl std::convert::TryFrom<char> for BoxDrawing {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LIGHT_HORIZONTAL => Ok(BoxDrawing::LightHorizontal),
            HEAVY_HORIZONTAL => Ok(BoxDrawing::HeavyHorizontal),
            LIGHT_VERTICAL => Ok(BoxDrawing::LightVertical),
            HEAVY_VERTICAL => Ok(BoxDrawing::HeavyVertical),
            LIGHT_TRIPLE_DASH_HORIZONTAL => Ok(BoxDrawing::LightTripleDashHorizontal),
            HEAVY_TRIPLE_DASH_HORIZONTAL => Ok(BoxDrawing::HeavyTripleDashHorizontal),
            LIGHT_TRIPLE_DASH_VERTICAL => Ok(BoxDrawing::LightTripleDashVertical),
            HEAVY_TRIPLE_DASH_VERTICAL => Ok(BoxDrawing::HeavyTripleDashVertical),
            LIGHT_QUADRUPLE_DASH_HORIZONTAL => Ok(BoxDrawing::LightQuadrupleDashHorizontal),
            HEAVY_QUADRUPLE_DASH_HORIZONTAL => Ok(BoxDrawing::HeavyQuadrupleDashHorizontal),
            LIGHT_QUADRUPLE_DASH_VERTICAL => Ok(BoxDrawing::LightQuadrupleDashVertical),
            HEAVY_QUADRUPLE_DASH_VERTICAL => Ok(BoxDrawing::HeavyQuadrupleDashVertical),
            LIGHT_DOWN_AND_RIGHT => Ok(BoxDrawing::LightDownAndRight),
            DOWN_LIGHT_AND_RIGHT_HEAVY => Ok(BoxDrawing::DownLightAndRightHeavy),
            DOWN_HEAVY_AND_RIGHT_LIGHT => Ok(BoxDrawing::DownHeavyAndRightLight),
            HEAVY_DOWN_AND_RIGHT => Ok(BoxDrawing::HeavyDownAndRight),
            LIGHT_DOWN_AND_LEFT => Ok(BoxDrawing::LightDownAndLeft),
            DOWN_LIGHT_AND_LEFT_HEAVY => Ok(BoxDrawing::DownLightAndLeftHeavy),
            DOWN_HEAVY_AND_LEFT_LIGHT => Ok(BoxDrawing::DownHeavyAndLeftLight),
            HEAVY_DOWN_AND_LEFT => Ok(BoxDrawing::HeavyDownAndLeft),
            LIGHT_UP_AND_RIGHT => Ok(BoxDrawing::LightUpAndRight),
            UP_LIGHT_AND_RIGHT_HEAVY => Ok(BoxDrawing::UpLightAndRightHeavy),
            UP_HEAVY_AND_RIGHT_LIGHT => Ok(BoxDrawing::UpHeavyAndRightLight),
            HEAVY_UP_AND_RIGHT => Ok(BoxDrawing::HeavyUpAndRight),
            LIGHT_UP_AND_LEFT => Ok(BoxDrawing::LightUpAndLeft),
            UP_LIGHT_AND_LEFT_HEAVY => Ok(BoxDrawing::UpLightAndLeftHeavy),
            UP_HEAVY_AND_LEFT_LIGHT => Ok(BoxDrawing::UpHeavyAndLeftLight),
            HEAVY_UP_AND_LEFT => Ok(BoxDrawing::HeavyUpAndLeft),
            LIGHT_VERTICAL_AND_RIGHT => Ok(BoxDrawing::LightVerticalAndRight),
            VERTICAL_LIGHT_AND_RIGHT_HEAVY => Ok(BoxDrawing::VerticalLightAndRightHeavy),
            UP_HEAVY_AND_RIGHT_DOWN_LIGHT => Ok(BoxDrawing::UpHeavyAndRightDownLight),
            DOWN_HEAVY_AND_RIGHT_UP_LIGHT => Ok(BoxDrawing::DownHeavyAndRightUpLight),
            VERTICAL_HEAVY_AND_RIGHT_LIGHT => Ok(BoxDrawing::VerticalHeavyAndRightLight),
            DOWN_LIGHT_AND_RIGHT_UP_HEAVY => Ok(BoxDrawing::DownLightAndRightUpHeavy),
            UP_LIGHT_AND_RIGHT_DOWN_HEAVY => Ok(BoxDrawing::UpLightAndRightDownHeavy),
            HEAVY_VERTICAL_AND_RIGHT => Ok(BoxDrawing::HeavyVerticalAndRight),
            LIGHT_VERTICAL_AND_LEFT => Ok(BoxDrawing::LightVerticalAndLeft),
            VERTICAL_LIGHT_AND_LEFT_HEAVY => Ok(BoxDrawing::VerticalLightAndLeftHeavy),
            UP_HEAVY_AND_LEFT_DOWN_LIGHT => Ok(BoxDrawing::UpHeavyAndLeftDownLight),
            DOWN_HEAVY_AND_LEFT_UP_LIGHT => Ok(BoxDrawing::DownHeavyAndLeftUpLight),
            VERTICAL_HEAVY_AND_LEFT_LIGHT => Ok(BoxDrawing::VerticalHeavyAndLeftLight),
            DOWN_LIGHT_AND_LEFT_UP_HEAVY => Ok(BoxDrawing::DownLightAndLeftUpHeavy),
            UP_LIGHT_AND_LEFT_DOWN_HEAVY => Ok(BoxDrawing::UpLightAndLeftDownHeavy),
            HEAVY_VERTICAL_AND_LEFT => Ok(BoxDrawing::HeavyVerticalAndLeft),
            LIGHT_DOWN_AND_HORIZONTAL => Ok(BoxDrawing::LightDownAndHorizontal),
            LEFT_HEAVY_AND_RIGHT_DOWN_LIGHT => Ok(BoxDrawing::LeftHeavyAndRightDownLight),
            RIGHT_HEAVY_AND_LEFT_DOWN_LIGHT => Ok(BoxDrawing::RightHeavyAndLeftDownLight),
            DOWN_LIGHT_AND_HORIZONTAL_HEAVY => Ok(BoxDrawing::DownLightAndHorizontalHeavy),
            DOWN_HEAVY_AND_HORIZONTAL_LIGHT => Ok(BoxDrawing::DownHeavyAndHorizontalLight),
            RIGHT_LIGHT_AND_LEFT_DOWN_HEAVY => Ok(BoxDrawing::RightLightAndLeftDownHeavy),
            LEFT_LIGHT_AND_RIGHT_DOWN_HEAVY => Ok(BoxDrawing::LeftLightAndRightDownHeavy),
            HEAVY_DOWN_AND_HORIZONTAL => Ok(BoxDrawing::HeavyDownAndHorizontal),
            LIGHT_UP_AND_HORIZONTAL => Ok(BoxDrawing::LightUpAndHorizontal),
            LEFT_HEAVY_AND_RIGHT_UP_LIGHT => Ok(BoxDrawing::LeftHeavyAndRightUpLight),
            RIGHT_HEAVY_AND_LEFT_UP_LIGHT => Ok(BoxDrawing::RightHeavyAndLeftUpLight),
            UP_LIGHT_AND_HORIZONTAL_HEAVY => Ok(BoxDrawing::UpLightAndHorizontalHeavy),
            UP_HEAVY_AND_HORIZONTAL_LIGHT => Ok(BoxDrawing::UpHeavyAndHorizontalLight),
            RIGHT_LIGHT_AND_LEFT_UP_HEAVY => Ok(BoxDrawing::RightLightAndLeftUpHeavy),
            LEFT_LIGHT_AND_RIGHT_UP_HEAVY => Ok(BoxDrawing::LeftLightAndRightUpHeavy),
            HEAVY_UP_AND_HORIZONTAL => Ok(BoxDrawing::HeavyUpAndHorizontal),
            LIGHT_VERTICAL_AND_HORIZONTAL => Ok(BoxDrawing::LightVerticalAndHorizontal),
            LEFT_HEAVY_AND_RIGHT_VERTICAL_LIGHT => Ok(BoxDrawing::LeftHeavyAndRightVerticalLight),
            RIGHT_HEAVY_AND_LEFT_VERTICAL_LIGHT => Ok(BoxDrawing::RightHeavyAndLeftVerticalLight),
            VERTICAL_LIGHT_AND_HORIZONTAL_HEAVY => Ok(BoxDrawing::VerticalLightAndHorizontalHeavy),
            UP_HEAVY_AND_DOWN_HORIZONTAL_LIGHT => Ok(BoxDrawing::UpHeavyAndDownHorizontalLight),
            DOWN_HEAVY_AND_UP_HORIZONTAL_LIGHT => Ok(BoxDrawing::DownHeavyAndUpHorizontalLight),
            VERTICAL_HEAVY_AND_HORIZONTAL_LIGHT => Ok(BoxDrawing::VerticalHeavyAndHorizontalLight),
            LEFT_UP_HEAVY_AND_RIGHT_DOWN_LIGHT => Ok(BoxDrawing::LeftUpHeavyAndRightDownLight),
            RIGHT_UP_HEAVY_AND_LEFT_DOWN_LIGHT => Ok(BoxDrawing::RightUpHeavyAndLeftDownLight),
            LEFT_DOWN_HEAVY_AND_RIGHT_UP_LIGHT => Ok(BoxDrawing::LeftDownHeavyAndRightUpLight),
            RIGHT_DOWN_HEAVY_AND_LEFT_UP_LIGHT => Ok(BoxDrawing::RightDownHeavyAndLeftUpLight),
            DOWN_LIGHT_AND_UP_HORIZONTAL_HEAVY => Ok(BoxDrawing::DownLightAndUpHorizontalHeavy),
            UP_LIGHT_AND_DOWN_HORIZONTAL_HEAVY => Ok(BoxDrawing::UpLightAndDownHorizontalHeavy),
            RIGHT_LIGHT_AND_LEFT_VERTICAL_HEAVY => Ok(BoxDrawing::RightLightAndLeftVerticalHeavy),
            LEFT_LIGHT_AND_RIGHT_VERTICAL_HEAVY => Ok(BoxDrawing::LeftLightAndRightVerticalHeavy),
            HEAVY_VERTICAL_AND_HORIZONTAL => Ok(BoxDrawing::HeavyVerticalAndHorizontal),
            LIGHT_DOUBLE_DASH_HORIZONTAL => Ok(BoxDrawing::LightDoubleDashHorizontal),
            HEAVY_DOUBLE_DASH_HORIZONTAL => Ok(BoxDrawing::HeavyDoubleDashHorizontal),
            LIGHT_DOUBLE_DASH_VERTICAL => Ok(BoxDrawing::LightDoubleDashVertical),
            HEAVY_DOUBLE_DASH_VERTICAL => Ok(BoxDrawing::HeavyDoubleDashVertical),
            DOUBLE_HORIZONTAL => Ok(BoxDrawing::DoubleHorizontal),
            DOUBLE_VERTICAL => Ok(BoxDrawing::DoubleVertical),
            DOWN_SINGLE_AND_RIGHT_DOUBLE => Ok(BoxDrawing::DownSingleAndRightDouble),
            DOWN_DOUBLE_AND_RIGHT_SINGLE => Ok(BoxDrawing::DownDoubleAndRightSingle),
            DOUBLE_DOWN_AND_RIGHT => Ok(BoxDrawing::DoubleDownAndRight),
            DOWN_SINGLE_AND_LEFT_DOUBLE => Ok(BoxDrawing::DownSingleAndLeftDouble),
            DOWN_DOUBLE_AND_LEFT_SINGLE => Ok(BoxDrawing::DownDoubleAndLeftSingle),
            DOUBLE_DOWN_AND_LEFT => Ok(BoxDrawing::DoubleDownAndLeft),
            UP_SINGLE_AND_RIGHT_DOUBLE => Ok(BoxDrawing::UpSingleAndRightDouble),
            UP_DOUBLE_AND_RIGHT_SINGLE => Ok(BoxDrawing::UpDoubleAndRightSingle),
            DOUBLE_UP_AND_RIGHT => Ok(BoxDrawing::DoubleUpAndRight),
            UP_SINGLE_AND_LEFT_DOUBLE => Ok(BoxDrawing::UpSingleAndLeftDouble),
            UP_DOUBLE_AND_LEFT_SINGLE => Ok(BoxDrawing::UpDoubleAndLeftSingle),
            DOUBLE_UP_AND_LEFT => Ok(BoxDrawing::DoubleUpAndLeft),
            VERTICAL_SINGLE_AND_RIGHT_DOUBLE => Ok(BoxDrawing::VerticalSingleAndRightDouble),
            VERTICAL_DOUBLE_AND_RIGHT_SINGLE => Ok(BoxDrawing::VerticalDoubleAndRightSingle),
            DOUBLE_VERTICAL_AND_RIGHT => Ok(BoxDrawing::DoubleVerticalAndRight),
            VERTICAL_SINGLE_AND_LEFT_DOUBLE => Ok(BoxDrawing::VerticalSingleAndLeftDouble),
            VERTICAL_DOUBLE_AND_LEFT_SINGLE => Ok(BoxDrawing::VerticalDoubleAndLeftSingle),
            DOUBLE_VERTICAL_AND_LEFT => Ok(BoxDrawing::DoubleVerticalAndLeft),
            DOWN_SINGLE_AND_HORIZONTAL_DOUBLE => Ok(BoxDrawing::DownSingleAndHorizontalDouble),
            DOWN_DOUBLE_AND_HORIZONTAL_SINGLE => Ok(BoxDrawing::DownDoubleAndHorizontalSingle),
            DOUBLE_DOWN_AND_HORIZONTAL => Ok(BoxDrawing::DoubleDownAndHorizontal),
            UP_SINGLE_AND_HORIZONTAL_DOUBLE => Ok(BoxDrawing::UpSingleAndHorizontalDouble),
            UP_DOUBLE_AND_HORIZONTAL_SINGLE => Ok(BoxDrawing::UpDoubleAndHorizontalSingle),
            DOUBLE_UP_AND_HORIZONTAL => Ok(BoxDrawing::DoubleUpAndHorizontal),
            VERTICAL_SINGLE_AND_HORIZONTAL_DOUBLE => Ok(BoxDrawing::VerticalSingleAndHorizontalDouble),
            VERTICAL_DOUBLE_AND_HORIZONTAL_SINGLE => Ok(BoxDrawing::VerticalDoubleAndHorizontalSingle),
            DOUBLE_VERTICAL_AND_HORIZONTAL => Ok(BoxDrawing::DoubleVerticalAndHorizontal),
            LIGHT_ARC_DOWN_AND_RIGHT => Ok(BoxDrawing::LightArcDownAndRight),
            LIGHT_ARC_DOWN_AND_LEFT => Ok(BoxDrawing::LightArcDownAndLeft),
            LIGHT_ARC_UP_AND_LEFT => Ok(BoxDrawing::LightArcUpAndLeft),
            LIGHT_ARC_UP_AND_RIGHT => Ok(BoxDrawing::LightArcUpAndRight),
            LIGHT_DIAGONAL_UPPER_RIGHT_TO_LOWER_LEFT => Ok(BoxDrawing::LightDiagonalUpperRightToLowerLeft),
            LIGHT_DIAGONAL_UPPER_LEFT_TO_LOWER_RIGHT => Ok(BoxDrawing::LightDiagonalUpperLeftToLowerRight),
            LIGHT_DIAGONAL_CROSS => Ok(BoxDrawing::LightDiagonalCross),
            LIGHT_LEFT => Ok(BoxDrawing::LightLeft),
            LIGHT_UP => Ok(BoxDrawing::LightUp),
            LIGHT_RIGHT => Ok(BoxDrawing::LightRight),
            LIGHT_DOWN => Ok(BoxDrawing::LightDown),
            HEAVY_LEFT => Ok(BoxDrawing::HeavyLeft),
            HEAVY_UP => Ok(BoxDrawing::HeavyUp),
            HEAVY_RIGHT => Ok(BoxDrawing::HeavyRight),
            HEAVY_DOWN => Ok(BoxDrawing::HeavyDown),
            LIGHT_LEFT_AND_HEAVY_RIGHT => Ok(BoxDrawing::LightLeftAndHeavyRight),
            LIGHT_UP_AND_HEAVY_DOWN => Ok(BoxDrawing::LightUpAndHeavyDown),
            HEAVY_LEFT_AND_LIGHT_RIGHT => Ok(BoxDrawing::HeavyLeftAndLightRight),
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

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            BoxDrawing::LightHorizontal => "box drawing light horizontal",
            BoxDrawing::HeavyHorizontal => "box drawing heavy horizontal",
            BoxDrawing::LightVertical => "box drawing light vertical",
            BoxDrawing::HeavyVertical => "box drawing heavy vertical",
            BoxDrawing::LightTripleDashHorizontal => "box drawing light triple dash horizontal",
            BoxDrawing::HeavyTripleDashHorizontal => "box drawing heavy triple dash horizontal",
            BoxDrawing::LightTripleDashVertical => "box drawing light triple dash vertical",
            BoxDrawing::HeavyTripleDashVertical => "box drawing heavy triple dash vertical",
            BoxDrawing::LightQuadrupleDashHorizontal => "box drawing light quadruple dash horizontal",
            BoxDrawing::HeavyQuadrupleDashHorizontal => "box drawing heavy quadruple dash horizontal",
            BoxDrawing::LightQuadrupleDashVertical => "box drawing light quadruple dash vertical",
            BoxDrawing::HeavyQuadrupleDashVertical => "box drawing heavy quadruple dash vertical",
            BoxDrawing::LightDownAndRight => "box drawing light down and right",
            BoxDrawing::DownLightAndRightHeavy => "box drawing down light and right heavy",
            BoxDrawing::DownHeavyAndRightLight => "box drawing down heavy and right light",
            BoxDrawing::HeavyDownAndRight => "box drawing heavy down and right",
            BoxDrawing::LightDownAndLeft => "box drawing light down and left",
            BoxDrawing::DownLightAndLeftHeavy => "box drawing down light and left heavy",
            BoxDrawing::DownHeavyAndLeftLight => "box drawing down heavy and left light",
            BoxDrawing::HeavyDownAndLeft => "box drawing heavy down and left",
            BoxDrawing::LightUpAndRight => "box drawing light up and right",
            BoxDrawing::UpLightAndRightHeavy => "box drawing up light and right heavy",
            BoxDrawing::UpHeavyAndRightLight => "box drawing up heavy and right light",
            BoxDrawing::HeavyUpAndRight => "box drawing heavy up and right",
            BoxDrawing::LightUpAndLeft => "box drawing light up and left",
            BoxDrawing::UpLightAndLeftHeavy => "box drawing up light and left heavy",
            BoxDrawing::UpHeavyAndLeftLight => "box drawing up heavy and left light",
            BoxDrawing::HeavyUpAndLeft => "box drawing heavy up and left",
            BoxDrawing::LightVerticalAndRight => "box drawing light vertical and right",
            BoxDrawing::VerticalLightAndRightHeavy => "box drawing vertical light and right heavy",
            BoxDrawing::UpHeavyAndRightDownLight => "box drawing up heavy and right down light",
            BoxDrawing::DownHeavyAndRightUpLight => "box drawing down heavy and right up light",
            BoxDrawing::VerticalHeavyAndRightLight => "box drawing vertical heavy and right light",
            BoxDrawing::DownLightAndRightUpHeavy => "box drawing down light and right up heavy",
            BoxDrawing::UpLightAndRightDownHeavy => "box drawing up light and right down heavy",
            BoxDrawing::HeavyVerticalAndRight => "box drawing heavy vertical and right",
            BoxDrawing::LightVerticalAndLeft => "box drawing light vertical and left",
            BoxDrawing::VerticalLightAndLeftHeavy => "box drawing vertical light and left heavy",
            BoxDrawing::UpHeavyAndLeftDownLight => "box drawing up heavy and left down light",
            BoxDrawing::DownHeavyAndLeftUpLight => "box drawing down heavy and left up light",
            BoxDrawing::VerticalHeavyAndLeftLight => "box drawing vertical heavy and left light",
            BoxDrawing::DownLightAndLeftUpHeavy => "box drawing down light and left up heavy",
            BoxDrawing::UpLightAndLeftDownHeavy => "box drawing up light and left down heavy",
            BoxDrawing::HeavyVerticalAndLeft => "box drawing heavy vertical and left",
            BoxDrawing::LightDownAndHorizontal => "box drawing light down and horizontal",
            BoxDrawing::LeftHeavyAndRightDownLight => "box drawing left heavy and right down light",
            BoxDrawing::RightHeavyAndLeftDownLight => "box drawing right heavy and left down light",
            BoxDrawing::DownLightAndHorizontalHeavy => "box drawing down light and horizontal heavy",
            BoxDrawing::DownHeavyAndHorizontalLight => "box drawing down heavy and horizontal light",
            BoxDrawing::RightLightAndLeftDownHeavy => "box drawing right light and left down heavy",
            BoxDrawing::LeftLightAndRightDownHeavy => "box drawing left light and right down heavy",
            BoxDrawing::HeavyDownAndHorizontal => "box drawing heavy down and horizontal",
            BoxDrawing::LightUpAndHorizontal => "box drawing light up and horizontal",
            BoxDrawing::LeftHeavyAndRightUpLight => "box drawing left heavy and right up light",
            BoxDrawing::RightHeavyAndLeftUpLight => "box drawing right heavy and left up light",
            BoxDrawing::UpLightAndHorizontalHeavy => "box drawing up light and horizontal heavy",
            BoxDrawing::UpHeavyAndHorizontalLight => "box drawing up heavy and horizontal light",
            BoxDrawing::RightLightAndLeftUpHeavy => "box drawing right light and left up heavy",
            BoxDrawing::LeftLightAndRightUpHeavy => "box drawing left light and right up heavy",
            BoxDrawing::HeavyUpAndHorizontal => "box drawing heavy up and horizontal",
            BoxDrawing::LightVerticalAndHorizontal => "box drawing light vertical and horizontal",
            BoxDrawing::LeftHeavyAndRightVerticalLight => "box drawing left heavy and right vertical light",
            BoxDrawing::RightHeavyAndLeftVerticalLight => "box drawing right heavy and left vertical light",
            BoxDrawing::VerticalLightAndHorizontalHeavy => "box drawing vertical light and horizontal heavy",
            BoxDrawing::UpHeavyAndDownHorizontalLight => "box drawing up heavy and down horizontal light",
            BoxDrawing::DownHeavyAndUpHorizontalLight => "box drawing down heavy and up horizontal light",
            BoxDrawing::VerticalHeavyAndHorizontalLight => "box drawing vertical heavy and horizontal light",
            BoxDrawing::LeftUpHeavyAndRightDownLight => "box drawing left up heavy and right down light",
            BoxDrawing::RightUpHeavyAndLeftDownLight => "box drawing right up heavy and left down light",
            BoxDrawing::LeftDownHeavyAndRightUpLight => "box drawing left down heavy and right up light",
            BoxDrawing::RightDownHeavyAndLeftUpLight => "box drawing right down heavy and left up light",
            BoxDrawing::DownLightAndUpHorizontalHeavy => "box drawing down light and up horizontal heavy",
            BoxDrawing::UpLightAndDownHorizontalHeavy => "box drawing up light and down horizontal heavy",
            BoxDrawing::RightLightAndLeftVerticalHeavy => "box drawing right light and left vertical heavy",
            BoxDrawing::LeftLightAndRightVerticalHeavy => "box drawing left light and right vertical heavy",
            BoxDrawing::HeavyVerticalAndHorizontal => "box drawing heavy vertical and horizontal",
            BoxDrawing::LightDoubleDashHorizontal => "box drawing light double dash horizontal",
            BoxDrawing::HeavyDoubleDashHorizontal => "box drawing heavy double dash horizontal",
            BoxDrawing::LightDoubleDashVertical => "box drawing light double dash vertical",
            BoxDrawing::HeavyDoubleDashVertical => "box drawing heavy double dash vertical",
            BoxDrawing::DoubleHorizontal => "box drawing double horizontal",
            BoxDrawing::DoubleVertical => "box drawing double vertical",
            BoxDrawing::DownSingleAndRightDouble => "box drawing down single and right double",
            BoxDrawing::DownDoubleAndRightSingle => "box drawing down double and right single",
            BoxDrawing::DoubleDownAndRight => "box drawing double down and right",
            BoxDrawing::DownSingleAndLeftDouble => "box drawing down single and left double",
            BoxDrawing::DownDoubleAndLeftSingle => "box drawing down double and left single",
            BoxDrawing::DoubleDownAndLeft => "box drawing double down and left",
            BoxDrawing::UpSingleAndRightDouble => "box drawing up single and right double",
            BoxDrawing::UpDoubleAndRightSingle => "box drawing up double and right single",
            BoxDrawing::DoubleUpAndRight => "box drawing double up and right",
            BoxDrawing::UpSingleAndLeftDouble => "box drawing up single and left double",
            BoxDrawing::UpDoubleAndLeftSingle => "box drawing up double and left single",
            BoxDrawing::DoubleUpAndLeft => "box drawing double up and left",
            BoxDrawing::VerticalSingleAndRightDouble => "box drawing vertical single and right double",
            BoxDrawing::VerticalDoubleAndRightSingle => "box drawing vertical double and right single",
            BoxDrawing::DoubleVerticalAndRight => "box drawing double vertical and right",
            BoxDrawing::VerticalSingleAndLeftDouble => "box drawing vertical single and left double",
            BoxDrawing::VerticalDoubleAndLeftSingle => "box drawing vertical double and left single",
            BoxDrawing::DoubleVerticalAndLeft => "box drawing double vertical and left",
            BoxDrawing::DownSingleAndHorizontalDouble => "box drawing down single and horizontal double",
            BoxDrawing::DownDoubleAndHorizontalSingle => "box drawing down double and horizontal single",
            BoxDrawing::DoubleDownAndHorizontal => "box drawing double down and horizontal",
            BoxDrawing::UpSingleAndHorizontalDouble => "box drawing up single and horizontal double",
            BoxDrawing::UpDoubleAndHorizontalSingle => "box drawing up double and horizontal single",
            BoxDrawing::DoubleUpAndHorizontal => "box drawing double up and horizontal",
            BoxDrawing::VerticalSingleAndHorizontalDouble => "box drawing vertical single and horizontal double",
            BoxDrawing::VerticalDoubleAndHorizontalSingle => "box drawing vertical double and horizontal single",
            BoxDrawing::DoubleVerticalAndHorizontal => "box drawing double vertical and horizontal",
            BoxDrawing::LightArcDownAndRight => "box drawing light arc down and right",
            BoxDrawing::LightArcDownAndLeft => "box drawing light arc down and left",
            BoxDrawing::LightArcUpAndLeft => "box drawing light arc up and left",
            BoxDrawing::LightArcUpAndRight => "box drawing light arc up and right",
            BoxDrawing::LightDiagonalUpperRightToLowerLeft => "box drawing light diagonal upper right to lower left",
            BoxDrawing::LightDiagonalUpperLeftToLowerRight => "box drawing light diagonal upper left to lower right",
            BoxDrawing::LightDiagonalCross => "box drawing light diagonal cross",
            BoxDrawing::LightLeft => "box drawing light left",
            BoxDrawing::LightUp => "box drawing light up",
            BoxDrawing::LightRight => "box drawing light right",
            BoxDrawing::LightDown => "box drawing light down",
            BoxDrawing::HeavyLeft => "box drawing heavy left",
            BoxDrawing::HeavyUp => "box drawing heavy up",
            BoxDrawing::HeavyRight => "box drawing heavy right",
            BoxDrawing::HeavyDown => "box drawing heavy down",
            BoxDrawing::LightLeftAndHeavyRight => "box drawing light left and heavy right",
            BoxDrawing::LightUpAndHeavyDown => "box drawing light up and heavy down",
            BoxDrawing::HeavyLeftAndLightRight => "box drawing heavy left and light right",
        }
    }
}
