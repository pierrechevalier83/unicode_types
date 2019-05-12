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
    pub const BOX_DRAWINGS_LIGHT_HORIZONTAL: char = '─';
    /// \u{2501}: '━'
    pub const BOX_DRAWINGS_HEAVY_HORIZONTAL: char = '━';
    /// \u{2502}: '│'
    pub const BOX_DRAWINGS_LIGHT_VERTICAL: char = '│';
    /// \u{2503}: '┃'
    pub const BOX_DRAWINGS_HEAVY_VERTICAL: char = '┃';
    /// \u{2504}: '┄'
    pub const BOX_DRAWINGS_LIGHT_TRIPLE_DASH_HORIZONTAL: char = '┄';
    /// \u{2505}: '┅'
    pub const BOX_DRAWINGS_HEAVY_TRIPLE_DASH_HORIZONTAL: char = '┅';
    /// \u{2506}: '┆'
    pub const BOX_DRAWINGS_LIGHT_TRIPLE_DASH_VERTICAL: char = '┆';
    /// \u{2507}: '┇'
    pub const BOX_DRAWINGS_HEAVY_TRIPLE_DASH_VERTICAL: char = '┇';
    /// \u{2508}: '┈'
    pub const BOX_DRAWINGS_LIGHT_QUADRUPLE_DASH_HORIZONTAL: char = '┈';
    /// \u{2509}: '┉'
    pub const BOX_DRAWINGS_HEAVY_QUADRUPLE_DASH_HORIZONTAL: char = '┉';
    /// \u{250a}: '┊'
    pub const BOX_DRAWINGS_LIGHT_QUADRUPLE_DASH_VERTICAL: char = '┊';
    /// \u{250b}: '┋'
    pub const BOX_DRAWINGS_HEAVY_QUADRUPLE_DASH_VERTICAL: char = '┋';
    /// \u{250c}: '┌'
    pub const BOX_DRAWINGS_LIGHT_DOWN_AND_RIGHT: char = '┌';
    /// \u{250d}: '┍'
    pub const BOX_DRAWINGS_DOWN_LIGHT_AND_RIGHT_HEAVY: char = '┍';
    /// \u{250e}: '┎'
    pub const BOX_DRAWINGS_DOWN_HEAVY_AND_RIGHT_LIGHT: char = '┎';
    /// \u{250f}: '┏'
    pub const BOX_DRAWINGS_HEAVY_DOWN_AND_RIGHT: char = '┏';
    /// \u{2510}: '┐'
    pub const BOX_DRAWINGS_LIGHT_DOWN_AND_LEFT: char = '┐';
    /// \u{2511}: '┑'
    pub const BOX_DRAWINGS_DOWN_LIGHT_AND_LEFT_HEAVY: char = '┑';
    /// \u{2512}: '┒'
    pub const BOX_DRAWINGS_DOWN_HEAVY_AND_LEFT_LIGHT: char = '┒';
    /// \u{2513}: '┓'
    pub const BOX_DRAWINGS_HEAVY_DOWN_AND_LEFT: char = '┓';
    /// \u{2514}: '└'
    pub const BOX_DRAWINGS_LIGHT_UP_AND_RIGHT: char = '└';
    /// \u{2515}: '┕'
    pub const BOX_DRAWINGS_UP_LIGHT_AND_RIGHT_HEAVY: char = '┕';
    /// \u{2516}: '┖'
    pub const BOX_DRAWINGS_UP_HEAVY_AND_RIGHT_LIGHT: char = '┖';
    /// \u{2517}: '┗'
    pub const BOX_DRAWINGS_HEAVY_UP_AND_RIGHT: char = '┗';
    /// \u{2518}: '┘'
    pub const BOX_DRAWINGS_LIGHT_UP_AND_LEFT: char = '┘';
    /// \u{2519}: '┙'
    pub const BOX_DRAWINGS_UP_LIGHT_AND_LEFT_HEAVY: char = '┙';
    /// \u{251a}: '┚'
    pub const BOX_DRAWINGS_UP_HEAVY_AND_LEFT_LIGHT: char = '┚';
    /// \u{251b}: '┛'
    pub const BOX_DRAWINGS_HEAVY_UP_AND_LEFT: char = '┛';
    /// \u{251c}: '├'
    pub const BOX_DRAWINGS_LIGHT_VERTICAL_AND_RIGHT: char = '├';
    /// \u{251d}: '┝'
    pub const BOX_DRAWINGS_VERTICAL_LIGHT_AND_RIGHT_HEAVY: char = '┝';
    /// \u{251e}: '┞'
    pub const BOX_DRAWINGS_UP_HEAVY_AND_RIGHT_DOWN_LIGHT: char = '┞';
    /// \u{251f}: '┟'
    pub const BOX_DRAWINGS_DOWN_HEAVY_AND_RIGHT_UP_LIGHT: char = '┟';
    /// \u{2520}: '┠'
    pub const BOX_DRAWINGS_VERTICAL_HEAVY_AND_RIGHT_LIGHT: char = '┠';
    /// \u{2521}: '┡'
    pub const BOX_DRAWINGS_DOWN_LIGHT_AND_RIGHT_UP_HEAVY: char = '┡';
    /// \u{2522}: '┢'
    pub const BOX_DRAWINGS_UP_LIGHT_AND_RIGHT_DOWN_HEAVY: char = '┢';
    /// \u{2523}: '┣'
    pub const BOX_DRAWINGS_HEAVY_VERTICAL_AND_RIGHT: char = '┣';
    /// \u{2524}: '┤'
    pub const BOX_DRAWINGS_LIGHT_VERTICAL_AND_LEFT: char = '┤';
    /// \u{2525}: '┥'
    pub const BOX_DRAWINGS_VERTICAL_LIGHT_AND_LEFT_HEAVY: char = '┥';
    /// \u{2526}: '┦'
    pub const BOX_DRAWINGS_UP_HEAVY_AND_LEFT_DOWN_LIGHT: char = '┦';
    /// \u{2527}: '┧'
    pub const BOX_DRAWINGS_DOWN_HEAVY_AND_LEFT_UP_LIGHT: char = '┧';
    /// \u{2528}: '┨'
    pub const BOX_DRAWINGS_VERTICAL_HEAVY_AND_LEFT_LIGHT: char = '┨';
    /// \u{2529}: '┩'
    pub const BOX_DRAWINGS_DOWN_LIGHT_AND_LEFT_UP_HEAVY: char = '┩';
    /// \u{252a}: '┪'
    pub const BOX_DRAWINGS_UP_LIGHT_AND_LEFT_DOWN_HEAVY: char = '┪';
    /// \u{252b}: '┫'
    pub const BOX_DRAWINGS_HEAVY_VERTICAL_AND_LEFT: char = '┫';
    /// \u{252c}: '┬'
    pub const BOX_DRAWINGS_LIGHT_DOWN_AND_HORIZONTAL: char = '┬';
    /// \u{252d}: '┭'
    pub const BOX_DRAWINGS_LEFT_HEAVY_AND_RIGHT_DOWN_LIGHT: char = '┭';
    /// \u{252e}: '┮'
    pub const BOX_DRAWINGS_RIGHT_HEAVY_AND_LEFT_DOWN_LIGHT: char = '┮';
    /// \u{252f}: '┯'
    pub const BOX_DRAWINGS_DOWN_LIGHT_AND_HORIZONTAL_HEAVY: char = '┯';
    /// \u{2530}: '┰'
    pub const BOX_DRAWINGS_DOWN_HEAVY_AND_HORIZONTAL_LIGHT: char = '┰';
    /// \u{2531}: '┱'
    pub const BOX_DRAWINGS_RIGHT_LIGHT_AND_LEFT_DOWN_HEAVY: char = '┱';
    /// \u{2532}: '┲'
    pub const BOX_DRAWINGS_LEFT_LIGHT_AND_RIGHT_DOWN_HEAVY: char = '┲';
    /// \u{2533}: '┳'
    pub const BOX_DRAWINGS_HEAVY_DOWN_AND_HORIZONTAL: char = '┳';
    /// \u{2534}: '┴'
    pub const BOX_DRAWINGS_LIGHT_UP_AND_HORIZONTAL: char = '┴';
    /// \u{2535}: '┵'
    pub const BOX_DRAWINGS_LEFT_HEAVY_AND_RIGHT_UP_LIGHT: char = '┵';
    /// \u{2536}: '┶'
    pub const BOX_DRAWINGS_RIGHT_HEAVY_AND_LEFT_UP_LIGHT: char = '┶';
    /// \u{2537}: '┷'
    pub const BOX_DRAWINGS_UP_LIGHT_AND_HORIZONTAL_HEAVY: char = '┷';
    /// \u{2538}: '┸'
    pub const BOX_DRAWINGS_UP_HEAVY_AND_HORIZONTAL_LIGHT: char = '┸';
    /// \u{2539}: '┹'
    pub const BOX_DRAWINGS_RIGHT_LIGHT_AND_LEFT_UP_HEAVY: char = '┹';
    /// \u{253a}: '┺'
    pub const BOX_DRAWINGS_LEFT_LIGHT_AND_RIGHT_UP_HEAVY: char = '┺';
    /// \u{253b}: '┻'
    pub const BOX_DRAWINGS_HEAVY_UP_AND_HORIZONTAL: char = '┻';
    /// \u{253c}: '┼'
    pub const BOX_DRAWINGS_LIGHT_VERTICAL_AND_HORIZONTAL: char = '┼';
    /// \u{253d}: '┽'
    pub const BOX_DRAWINGS_LEFT_HEAVY_AND_RIGHT_VERTICAL_LIGHT: char = '┽';
    /// \u{253e}: '┾'
    pub const BOX_DRAWINGS_RIGHT_HEAVY_AND_LEFT_VERTICAL_LIGHT: char = '┾';
    /// \u{253f}: '┿'
    pub const BOX_DRAWINGS_VERTICAL_LIGHT_AND_HORIZONTAL_HEAVY: char = '┿';
    /// \u{2540}: '╀'
    pub const BOX_DRAWINGS_UP_HEAVY_AND_DOWN_HORIZONTAL_LIGHT: char = '╀';
    /// \u{2541}: '╁'
    pub const BOX_DRAWINGS_DOWN_HEAVY_AND_UP_HORIZONTAL_LIGHT: char = '╁';
    /// \u{2542}: '╂'
    pub const BOX_DRAWINGS_VERTICAL_HEAVY_AND_HORIZONTAL_LIGHT: char = '╂';
    /// \u{2543}: '╃'
    pub const BOX_DRAWINGS_LEFT_UP_HEAVY_AND_RIGHT_DOWN_LIGHT: char = '╃';
    /// \u{2544}: '╄'
    pub const BOX_DRAWINGS_RIGHT_UP_HEAVY_AND_LEFT_DOWN_LIGHT: char = '╄';
    /// \u{2545}: '╅'
    pub const BOX_DRAWINGS_LEFT_DOWN_HEAVY_AND_RIGHT_UP_LIGHT: char = '╅';
    /// \u{2546}: '╆'
    pub const BOX_DRAWINGS_RIGHT_DOWN_HEAVY_AND_LEFT_UP_LIGHT: char = '╆';
    /// \u{2547}: '╇'
    pub const BOX_DRAWINGS_DOWN_LIGHT_AND_UP_HORIZONTAL_HEAVY: char = '╇';
    /// \u{2548}: '╈'
    pub const BOX_DRAWINGS_UP_LIGHT_AND_DOWN_HORIZONTAL_HEAVY: char = '╈';
    /// \u{2549}: '╉'
    pub const BOX_DRAWINGS_RIGHT_LIGHT_AND_LEFT_VERTICAL_HEAVY: char = '╉';
    /// \u{254a}: '╊'
    pub const BOX_DRAWINGS_LEFT_LIGHT_AND_RIGHT_VERTICAL_HEAVY: char = '╊';
    /// \u{254b}: '╋'
    pub const BOX_DRAWINGS_HEAVY_VERTICAL_AND_HORIZONTAL: char = '╋';
    /// \u{254c}: '╌'
    pub const BOX_DRAWINGS_LIGHT_DOUBLE_DASH_HORIZONTAL: char = '╌';
    /// \u{254d}: '╍'
    pub const BOX_DRAWINGS_HEAVY_DOUBLE_DASH_HORIZONTAL: char = '╍';
    /// \u{254e}: '╎'
    pub const BOX_DRAWINGS_LIGHT_DOUBLE_DASH_VERTICAL: char = '╎';
    /// \u{254f}: '╏'
    pub const BOX_DRAWINGS_HEAVY_DOUBLE_DASH_VERTICAL: char = '╏';
    /// \u{2550}: '═'
    pub const BOX_DRAWINGS_DOUBLE_HORIZONTAL: char = '═';
    /// \u{2551}: '║'
    pub const BOX_DRAWINGS_DOUBLE_VERTICAL: char = '║';
    /// \u{2552}: '╒'
    pub const BOX_DRAWINGS_DOWN_SINGLE_AND_RIGHT_DOUBLE: char = '╒';
    /// \u{2553}: '╓'
    pub const BOX_DRAWINGS_DOWN_DOUBLE_AND_RIGHT_SINGLE: char = '╓';
    /// \u{2554}: '╔'
    pub const BOX_DRAWINGS_DOUBLE_DOWN_AND_RIGHT: char = '╔';
    /// \u{2555}: '╕'
    pub const BOX_DRAWINGS_DOWN_SINGLE_AND_LEFT_DOUBLE: char = '╕';
    /// \u{2556}: '╖'
    pub const BOX_DRAWINGS_DOWN_DOUBLE_AND_LEFT_SINGLE: char = '╖';
    /// \u{2557}: '╗'
    pub const BOX_DRAWINGS_DOUBLE_DOWN_AND_LEFT: char = '╗';
    /// \u{2558}: '╘'
    pub const BOX_DRAWINGS_UP_SINGLE_AND_RIGHT_DOUBLE: char = '╘';
    /// \u{2559}: '╙'
    pub const BOX_DRAWINGS_UP_DOUBLE_AND_RIGHT_SINGLE: char = '╙';
    /// \u{255a}: '╚'
    pub const BOX_DRAWINGS_DOUBLE_UP_AND_RIGHT: char = '╚';
    /// \u{255b}: '╛'
    pub const BOX_DRAWINGS_UP_SINGLE_AND_LEFT_DOUBLE: char = '╛';
    /// \u{255c}: '╜'
    pub const BOX_DRAWINGS_UP_DOUBLE_AND_LEFT_SINGLE: char = '╜';
    /// \u{255d}: '╝'
    pub const BOX_DRAWINGS_DOUBLE_UP_AND_LEFT: char = '╝';
    /// \u{255e}: '╞'
    pub const BOX_DRAWINGS_VERTICAL_SINGLE_AND_RIGHT_DOUBLE: char = '╞';
    /// \u{255f}: '╟'
    pub const BOX_DRAWINGS_VERTICAL_DOUBLE_AND_RIGHT_SINGLE: char = '╟';
    /// \u{2560}: '╠'
    pub const BOX_DRAWINGS_DOUBLE_VERTICAL_AND_RIGHT: char = '╠';
    /// \u{2561}: '╡'
    pub const BOX_DRAWINGS_VERTICAL_SINGLE_AND_LEFT_DOUBLE: char = '╡';
    /// \u{2562}: '╢'
    pub const BOX_DRAWINGS_VERTICAL_DOUBLE_AND_LEFT_SINGLE: char = '╢';
    /// \u{2563}: '╣'
    pub const BOX_DRAWINGS_DOUBLE_VERTICAL_AND_LEFT: char = '╣';
    /// \u{2564}: '╤'
    pub const BOX_DRAWINGS_DOWN_SINGLE_AND_HORIZONTAL_DOUBLE: char = '╤';
    /// \u{2565}: '╥'
    pub const BOX_DRAWINGS_DOWN_DOUBLE_AND_HORIZONTAL_SINGLE: char = '╥';
    /// \u{2566}: '╦'
    pub const BOX_DRAWINGS_DOUBLE_DOWN_AND_HORIZONTAL: char = '╦';
    /// \u{2567}: '╧'
    pub const BOX_DRAWINGS_UP_SINGLE_AND_HORIZONTAL_DOUBLE: char = '╧';
    /// \u{2568}: '╨'
    pub const BOX_DRAWINGS_UP_DOUBLE_AND_HORIZONTAL_SINGLE: char = '╨';
    /// \u{2569}: '╩'
    pub const BOX_DRAWINGS_DOUBLE_UP_AND_HORIZONTAL: char = '╩';
    /// \u{256a}: '╪'
    pub const BOX_DRAWINGS_VERTICAL_SINGLE_AND_HORIZONTAL_DOUBLE: char = '╪';
    /// \u{256b}: '╫'
    pub const BOX_DRAWINGS_VERTICAL_DOUBLE_AND_HORIZONTAL_SINGLE: char = '╫';
    /// \u{256c}: '╬'
    pub const BOX_DRAWINGS_DOUBLE_VERTICAL_AND_HORIZONTAL: char = '╬';
    /// \u{256d}: '╭'
    pub const BOX_DRAWINGS_LIGHT_ARC_DOWN_AND_RIGHT: char = '╭';
    /// \u{256e}: '╮'
    pub const BOX_DRAWINGS_LIGHT_ARC_DOWN_AND_LEFT: char = '╮';
    /// \u{256f}: '╯'
    pub const BOX_DRAWINGS_LIGHT_ARC_UP_AND_LEFT: char = '╯';
    /// \u{2570}: '╰'
    pub const BOX_DRAWINGS_LIGHT_ARC_UP_AND_RIGHT: char = '╰';
    /// \u{2571}: '╱'
    pub const BOX_DRAWINGS_LIGHT_DIAGONAL_UPPER_RIGHT_TO_LOWER_LEFT: char = '╱';
    /// \u{2572}: '╲'
    pub const BOX_DRAWINGS_LIGHT_DIAGONAL_UPPER_LEFT_TO_LOWER_RIGHT: char = '╲';
    /// \u{2573}: '╳'
    pub const BOX_DRAWINGS_LIGHT_DIAGONAL_CROSS: char = '╳';
    /// \u{2574}: '╴'
    pub const BOX_DRAWINGS_LIGHT_LEFT: char = '╴';
    /// \u{2575}: '╵'
    pub const BOX_DRAWINGS_LIGHT_UP: char = '╵';
    /// \u{2576}: '╶'
    pub const BOX_DRAWINGS_LIGHT_RIGHT: char = '╶';
    /// \u{2577}: '╷'
    pub const BOX_DRAWINGS_LIGHT_DOWN: char = '╷';
    /// \u{2578}: '╸'
    pub const BOX_DRAWINGS_HEAVY_LEFT: char = '╸';
    /// \u{2579}: '╹'
    pub const BOX_DRAWINGS_HEAVY_UP: char = '╹';
    /// \u{257a}: '╺'
    pub const BOX_DRAWINGS_HEAVY_RIGHT: char = '╺';
    /// \u{257b}: '╻'
    pub const BOX_DRAWINGS_HEAVY_DOWN: char = '╻';
    /// \u{257c}: '╼'
    pub const BOX_DRAWINGS_LIGHT_LEFT_AND_HEAVY_RIGHT: char = '╼';
    /// \u{257d}: '╽'
    pub const BOX_DRAWINGS_LIGHT_UP_AND_HEAVY_DOWN: char = '╽';
    /// \u{257e}: '╾'
    pub const BOX_DRAWINGS_HEAVY_LEFT_AND_LIGHT_RIGHT: char = '╾';
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
    BoxDrawingsLightHorizontal,
    /// \u{2501}: '━'
    BoxDrawingsHeavyHorizontal,
    /// \u{2502}: '│'
    BoxDrawingsLightVertical,
    /// \u{2503}: '┃'
    BoxDrawingsHeavyVertical,
    /// \u{2504}: '┄'
    BoxDrawingsLightTripleDashHorizontal,
    /// \u{2505}: '┅'
    BoxDrawingsHeavyTripleDashHorizontal,
    /// \u{2506}: '┆'
    BoxDrawingsLightTripleDashVertical,
    /// \u{2507}: '┇'
    BoxDrawingsHeavyTripleDashVertical,
    /// \u{2508}: '┈'
    BoxDrawingsLightQuadrupleDashHorizontal,
    /// \u{2509}: '┉'
    BoxDrawingsHeavyQuadrupleDashHorizontal,
    /// \u{250a}: '┊'
    BoxDrawingsLightQuadrupleDashVertical,
    /// \u{250b}: '┋'
    BoxDrawingsHeavyQuadrupleDashVertical,
    /// \u{250c}: '┌'
    BoxDrawingsLightDownAndRight,
    /// \u{250d}: '┍'
    BoxDrawingsDownLightAndRightHeavy,
    /// \u{250e}: '┎'
    BoxDrawingsDownHeavyAndRightLight,
    /// \u{250f}: '┏'
    BoxDrawingsHeavyDownAndRight,
    /// \u{2510}: '┐'
    BoxDrawingsLightDownAndLeft,
    /// \u{2511}: '┑'
    BoxDrawingsDownLightAndLeftHeavy,
    /// \u{2512}: '┒'
    BoxDrawingsDownHeavyAndLeftLight,
    /// \u{2513}: '┓'
    BoxDrawingsHeavyDownAndLeft,
    /// \u{2514}: '└'
    BoxDrawingsLightUpAndRight,
    /// \u{2515}: '┕'
    BoxDrawingsUpLightAndRightHeavy,
    /// \u{2516}: '┖'
    BoxDrawingsUpHeavyAndRightLight,
    /// \u{2517}: '┗'
    BoxDrawingsHeavyUpAndRight,
    /// \u{2518}: '┘'
    BoxDrawingsLightUpAndLeft,
    /// \u{2519}: '┙'
    BoxDrawingsUpLightAndLeftHeavy,
    /// \u{251a}: '┚'
    BoxDrawingsUpHeavyAndLeftLight,
    /// \u{251b}: '┛'
    BoxDrawingsHeavyUpAndLeft,
    /// \u{251c}: '├'
    BoxDrawingsLightVerticalAndRight,
    /// \u{251d}: '┝'
    BoxDrawingsVerticalLightAndRightHeavy,
    /// \u{251e}: '┞'
    BoxDrawingsUpHeavyAndRightDownLight,
    /// \u{251f}: '┟'
    BoxDrawingsDownHeavyAndRightUpLight,
    /// \u{2520}: '┠'
    BoxDrawingsVerticalHeavyAndRightLight,
    /// \u{2521}: '┡'
    BoxDrawingsDownLightAndRightUpHeavy,
    /// \u{2522}: '┢'
    BoxDrawingsUpLightAndRightDownHeavy,
    /// \u{2523}: '┣'
    BoxDrawingsHeavyVerticalAndRight,
    /// \u{2524}: '┤'
    BoxDrawingsLightVerticalAndLeft,
    /// \u{2525}: '┥'
    BoxDrawingsVerticalLightAndLeftHeavy,
    /// \u{2526}: '┦'
    BoxDrawingsUpHeavyAndLeftDownLight,
    /// \u{2527}: '┧'
    BoxDrawingsDownHeavyAndLeftUpLight,
    /// \u{2528}: '┨'
    BoxDrawingsVerticalHeavyAndLeftLight,
    /// \u{2529}: '┩'
    BoxDrawingsDownLightAndLeftUpHeavy,
    /// \u{252a}: '┪'
    BoxDrawingsUpLightAndLeftDownHeavy,
    /// \u{252b}: '┫'
    BoxDrawingsHeavyVerticalAndLeft,
    /// \u{252c}: '┬'
    BoxDrawingsLightDownAndHorizontal,
    /// \u{252d}: '┭'
    BoxDrawingsLeftHeavyAndRightDownLight,
    /// \u{252e}: '┮'
    BoxDrawingsRightHeavyAndLeftDownLight,
    /// \u{252f}: '┯'
    BoxDrawingsDownLightAndHorizontalHeavy,
    /// \u{2530}: '┰'
    BoxDrawingsDownHeavyAndHorizontalLight,
    /// \u{2531}: '┱'
    BoxDrawingsRightLightAndLeftDownHeavy,
    /// \u{2532}: '┲'
    BoxDrawingsLeftLightAndRightDownHeavy,
    /// \u{2533}: '┳'
    BoxDrawingsHeavyDownAndHorizontal,
    /// \u{2534}: '┴'
    BoxDrawingsLightUpAndHorizontal,
    /// \u{2535}: '┵'
    BoxDrawingsLeftHeavyAndRightUpLight,
    /// \u{2536}: '┶'
    BoxDrawingsRightHeavyAndLeftUpLight,
    /// \u{2537}: '┷'
    BoxDrawingsUpLightAndHorizontalHeavy,
    /// \u{2538}: '┸'
    BoxDrawingsUpHeavyAndHorizontalLight,
    /// \u{2539}: '┹'
    BoxDrawingsRightLightAndLeftUpHeavy,
    /// \u{253a}: '┺'
    BoxDrawingsLeftLightAndRightUpHeavy,
    /// \u{253b}: '┻'
    BoxDrawingsHeavyUpAndHorizontal,
    /// \u{253c}: '┼'
    BoxDrawingsLightVerticalAndHorizontal,
    /// \u{253d}: '┽'
    BoxDrawingsLeftHeavyAndRightVerticalLight,
    /// \u{253e}: '┾'
    BoxDrawingsRightHeavyAndLeftVerticalLight,
    /// \u{253f}: '┿'
    BoxDrawingsVerticalLightAndHorizontalHeavy,
    /// \u{2540}: '╀'
    BoxDrawingsUpHeavyAndDownHorizontalLight,
    /// \u{2541}: '╁'
    BoxDrawingsDownHeavyAndUpHorizontalLight,
    /// \u{2542}: '╂'
    BoxDrawingsVerticalHeavyAndHorizontalLight,
    /// \u{2543}: '╃'
    BoxDrawingsLeftUpHeavyAndRightDownLight,
    /// \u{2544}: '╄'
    BoxDrawingsRightUpHeavyAndLeftDownLight,
    /// \u{2545}: '╅'
    BoxDrawingsLeftDownHeavyAndRightUpLight,
    /// \u{2546}: '╆'
    BoxDrawingsRightDownHeavyAndLeftUpLight,
    /// \u{2547}: '╇'
    BoxDrawingsDownLightAndUpHorizontalHeavy,
    /// \u{2548}: '╈'
    BoxDrawingsUpLightAndDownHorizontalHeavy,
    /// \u{2549}: '╉'
    BoxDrawingsRightLightAndLeftVerticalHeavy,
    /// \u{254a}: '╊'
    BoxDrawingsLeftLightAndRightVerticalHeavy,
    /// \u{254b}: '╋'
    BoxDrawingsHeavyVerticalAndHorizontal,
    /// \u{254c}: '╌'
    BoxDrawingsLightDoubleDashHorizontal,
    /// \u{254d}: '╍'
    BoxDrawingsHeavyDoubleDashHorizontal,
    /// \u{254e}: '╎'
    BoxDrawingsLightDoubleDashVertical,
    /// \u{254f}: '╏'
    BoxDrawingsHeavyDoubleDashVertical,
    /// \u{2550}: '═'
    BoxDrawingsDoubleHorizontal,
    /// \u{2551}: '║'
    BoxDrawingsDoubleVertical,
    /// \u{2552}: '╒'
    BoxDrawingsDownSingleAndRightDouble,
    /// \u{2553}: '╓'
    BoxDrawingsDownDoubleAndRightSingle,
    /// \u{2554}: '╔'
    BoxDrawingsDoubleDownAndRight,
    /// \u{2555}: '╕'
    BoxDrawingsDownSingleAndLeftDouble,
    /// \u{2556}: '╖'
    BoxDrawingsDownDoubleAndLeftSingle,
    /// \u{2557}: '╗'
    BoxDrawingsDoubleDownAndLeft,
    /// \u{2558}: '╘'
    BoxDrawingsUpSingleAndRightDouble,
    /// \u{2559}: '╙'
    BoxDrawingsUpDoubleAndRightSingle,
    /// \u{255a}: '╚'
    BoxDrawingsDoubleUpAndRight,
    /// \u{255b}: '╛'
    BoxDrawingsUpSingleAndLeftDouble,
    /// \u{255c}: '╜'
    BoxDrawingsUpDoubleAndLeftSingle,
    /// \u{255d}: '╝'
    BoxDrawingsDoubleUpAndLeft,
    /// \u{255e}: '╞'
    BoxDrawingsVerticalSingleAndRightDouble,
    /// \u{255f}: '╟'
    BoxDrawingsVerticalDoubleAndRightSingle,
    /// \u{2560}: '╠'
    BoxDrawingsDoubleVerticalAndRight,
    /// \u{2561}: '╡'
    BoxDrawingsVerticalSingleAndLeftDouble,
    /// \u{2562}: '╢'
    BoxDrawingsVerticalDoubleAndLeftSingle,
    /// \u{2563}: '╣'
    BoxDrawingsDoubleVerticalAndLeft,
    /// \u{2564}: '╤'
    BoxDrawingsDownSingleAndHorizontalDouble,
    /// \u{2565}: '╥'
    BoxDrawingsDownDoubleAndHorizontalSingle,
    /// \u{2566}: '╦'
    BoxDrawingsDoubleDownAndHorizontal,
    /// \u{2567}: '╧'
    BoxDrawingsUpSingleAndHorizontalDouble,
    /// \u{2568}: '╨'
    BoxDrawingsUpDoubleAndHorizontalSingle,
    /// \u{2569}: '╩'
    BoxDrawingsDoubleUpAndHorizontal,
    /// \u{256a}: '╪'
    BoxDrawingsVerticalSingleAndHorizontalDouble,
    /// \u{256b}: '╫'
    BoxDrawingsVerticalDoubleAndHorizontalSingle,
    /// \u{256c}: '╬'
    BoxDrawingsDoubleVerticalAndHorizontal,
    /// \u{256d}: '╭'
    BoxDrawingsLightArcDownAndRight,
    /// \u{256e}: '╮'
    BoxDrawingsLightArcDownAndLeft,
    /// \u{256f}: '╯'
    BoxDrawingsLightArcUpAndLeft,
    /// \u{2570}: '╰'
    BoxDrawingsLightArcUpAndRight,
    /// \u{2571}: '╱'
    BoxDrawingsLightDiagonalUpperRightToLowerLeft,
    /// \u{2572}: '╲'
    BoxDrawingsLightDiagonalUpperLeftToLowerRight,
    /// \u{2573}: '╳'
    BoxDrawingsLightDiagonalCross,
    /// \u{2574}: '╴'
    BoxDrawingsLightLeft,
    /// \u{2575}: '╵'
    BoxDrawingsLightUp,
    /// \u{2576}: '╶'
    BoxDrawingsLightRight,
    /// \u{2577}: '╷'
    BoxDrawingsLightDown,
    /// \u{2578}: '╸'
    BoxDrawingsHeavyLeft,
    /// \u{2579}: '╹'
    BoxDrawingsHeavyUp,
    /// \u{257a}: '╺'
    BoxDrawingsHeavyRight,
    /// \u{257b}: '╻'
    BoxDrawingsHeavyDown,
    /// \u{257c}: '╼'
    BoxDrawingsLightLeftAndHeavyRight,
    /// \u{257d}: '╽'
    BoxDrawingsLightUpAndHeavyDown,
    /// \u{257e}: '╾'
    BoxDrawingsHeavyLeftAndLightRight,
}

impl Into<char> for BoxDrawing {
    fn into(self) -> char {
        use constants::*;
        match self {
            BoxDrawing::BoxDrawingsLightHorizontal => BOX_DRAWINGS_LIGHT_HORIZONTAL,
            BoxDrawing::BoxDrawingsHeavyHorizontal => BOX_DRAWINGS_HEAVY_HORIZONTAL,
            BoxDrawing::BoxDrawingsLightVertical => BOX_DRAWINGS_LIGHT_VERTICAL,
            BoxDrawing::BoxDrawingsHeavyVertical => BOX_DRAWINGS_HEAVY_VERTICAL,
            BoxDrawing::BoxDrawingsLightTripleDashHorizontal => BOX_DRAWINGS_LIGHT_TRIPLE_DASH_HORIZONTAL,
            BoxDrawing::BoxDrawingsHeavyTripleDashHorizontal => BOX_DRAWINGS_HEAVY_TRIPLE_DASH_HORIZONTAL,
            BoxDrawing::BoxDrawingsLightTripleDashVertical => BOX_DRAWINGS_LIGHT_TRIPLE_DASH_VERTICAL,
            BoxDrawing::BoxDrawingsHeavyTripleDashVertical => BOX_DRAWINGS_HEAVY_TRIPLE_DASH_VERTICAL,
            BoxDrawing::BoxDrawingsLightQuadrupleDashHorizontal => BOX_DRAWINGS_LIGHT_QUADRUPLE_DASH_HORIZONTAL,
            BoxDrawing::BoxDrawingsHeavyQuadrupleDashHorizontal => BOX_DRAWINGS_HEAVY_QUADRUPLE_DASH_HORIZONTAL,
            BoxDrawing::BoxDrawingsLightQuadrupleDashVertical => BOX_DRAWINGS_LIGHT_QUADRUPLE_DASH_VERTICAL,
            BoxDrawing::BoxDrawingsHeavyQuadrupleDashVertical => BOX_DRAWINGS_HEAVY_QUADRUPLE_DASH_VERTICAL,
            BoxDrawing::BoxDrawingsLightDownAndRight => BOX_DRAWINGS_LIGHT_DOWN_AND_RIGHT,
            BoxDrawing::BoxDrawingsDownLightAndRightHeavy => BOX_DRAWINGS_DOWN_LIGHT_AND_RIGHT_HEAVY,
            BoxDrawing::BoxDrawingsDownHeavyAndRightLight => BOX_DRAWINGS_DOWN_HEAVY_AND_RIGHT_LIGHT,
            BoxDrawing::BoxDrawingsHeavyDownAndRight => BOX_DRAWINGS_HEAVY_DOWN_AND_RIGHT,
            BoxDrawing::BoxDrawingsLightDownAndLeft => BOX_DRAWINGS_LIGHT_DOWN_AND_LEFT,
            BoxDrawing::BoxDrawingsDownLightAndLeftHeavy => BOX_DRAWINGS_DOWN_LIGHT_AND_LEFT_HEAVY,
            BoxDrawing::BoxDrawingsDownHeavyAndLeftLight => BOX_DRAWINGS_DOWN_HEAVY_AND_LEFT_LIGHT,
            BoxDrawing::BoxDrawingsHeavyDownAndLeft => BOX_DRAWINGS_HEAVY_DOWN_AND_LEFT,
            BoxDrawing::BoxDrawingsLightUpAndRight => BOX_DRAWINGS_LIGHT_UP_AND_RIGHT,
            BoxDrawing::BoxDrawingsUpLightAndRightHeavy => BOX_DRAWINGS_UP_LIGHT_AND_RIGHT_HEAVY,
            BoxDrawing::BoxDrawingsUpHeavyAndRightLight => BOX_DRAWINGS_UP_HEAVY_AND_RIGHT_LIGHT,
            BoxDrawing::BoxDrawingsHeavyUpAndRight => BOX_DRAWINGS_HEAVY_UP_AND_RIGHT,
            BoxDrawing::BoxDrawingsLightUpAndLeft => BOX_DRAWINGS_LIGHT_UP_AND_LEFT,
            BoxDrawing::BoxDrawingsUpLightAndLeftHeavy => BOX_DRAWINGS_UP_LIGHT_AND_LEFT_HEAVY,
            BoxDrawing::BoxDrawingsUpHeavyAndLeftLight => BOX_DRAWINGS_UP_HEAVY_AND_LEFT_LIGHT,
            BoxDrawing::BoxDrawingsHeavyUpAndLeft => BOX_DRAWINGS_HEAVY_UP_AND_LEFT,
            BoxDrawing::BoxDrawingsLightVerticalAndRight => BOX_DRAWINGS_LIGHT_VERTICAL_AND_RIGHT,
            BoxDrawing::BoxDrawingsVerticalLightAndRightHeavy => BOX_DRAWINGS_VERTICAL_LIGHT_AND_RIGHT_HEAVY,
            BoxDrawing::BoxDrawingsUpHeavyAndRightDownLight => BOX_DRAWINGS_UP_HEAVY_AND_RIGHT_DOWN_LIGHT,
            BoxDrawing::BoxDrawingsDownHeavyAndRightUpLight => BOX_DRAWINGS_DOWN_HEAVY_AND_RIGHT_UP_LIGHT,
            BoxDrawing::BoxDrawingsVerticalHeavyAndRightLight => BOX_DRAWINGS_VERTICAL_HEAVY_AND_RIGHT_LIGHT,
            BoxDrawing::BoxDrawingsDownLightAndRightUpHeavy => BOX_DRAWINGS_DOWN_LIGHT_AND_RIGHT_UP_HEAVY,
            BoxDrawing::BoxDrawingsUpLightAndRightDownHeavy => BOX_DRAWINGS_UP_LIGHT_AND_RIGHT_DOWN_HEAVY,
            BoxDrawing::BoxDrawingsHeavyVerticalAndRight => BOX_DRAWINGS_HEAVY_VERTICAL_AND_RIGHT,
            BoxDrawing::BoxDrawingsLightVerticalAndLeft => BOX_DRAWINGS_LIGHT_VERTICAL_AND_LEFT,
            BoxDrawing::BoxDrawingsVerticalLightAndLeftHeavy => BOX_DRAWINGS_VERTICAL_LIGHT_AND_LEFT_HEAVY,
            BoxDrawing::BoxDrawingsUpHeavyAndLeftDownLight => BOX_DRAWINGS_UP_HEAVY_AND_LEFT_DOWN_LIGHT,
            BoxDrawing::BoxDrawingsDownHeavyAndLeftUpLight => BOX_DRAWINGS_DOWN_HEAVY_AND_LEFT_UP_LIGHT,
            BoxDrawing::BoxDrawingsVerticalHeavyAndLeftLight => BOX_DRAWINGS_VERTICAL_HEAVY_AND_LEFT_LIGHT,
            BoxDrawing::BoxDrawingsDownLightAndLeftUpHeavy => BOX_DRAWINGS_DOWN_LIGHT_AND_LEFT_UP_HEAVY,
            BoxDrawing::BoxDrawingsUpLightAndLeftDownHeavy => BOX_DRAWINGS_UP_LIGHT_AND_LEFT_DOWN_HEAVY,
            BoxDrawing::BoxDrawingsHeavyVerticalAndLeft => BOX_DRAWINGS_HEAVY_VERTICAL_AND_LEFT,
            BoxDrawing::BoxDrawingsLightDownAndHorizontal => BOX_DRAWINGS_LIGHT_DOWN_AND_HORIZONTAL,
            BoxDrawing::BoxDrawingsLeftHeavyAndRightDownLight => BOX_DRAWINGS_LEFT_HEAVY_AND_RIGHT_DOWN_LIGHT,
            BoxDrawing::BoxDrawingsRightHeavyAndLeftDownLight => BOX_DRAWINGS_RIGHT_HEAVY_AND_LEFT_DOWN_LIGHT,
            BoxDrawing::BoxDrawingsDownLightAndHorizontalHeavy => BOX_DRAWINGS_DOWN_LIGHT_AND_HORIZONTAL_HEAVY,
            BoxDrawing::BoxDrawingsDownHeavyAndHorizontalLight => BOX_DRAWINGS_DOWN_HEAVY_AND_HORIZONTAL_LIGHT,
            BoxDrawing::BoxDrawingsRightLightAndLeftDownHeavy => BOX_DRAWINGS_RIGHT_LIGHT_AND_LEFT_DOWN_HEAVY,
            BoxDrawing::BoxDrawingsLeftLightAndRightDownHeavy => BOX_DRAWINGS_LEFT_LIGHT_AND_RIGHT_DOWN_HEAVY,
            BoxDrawing::BoxDrawingsHeavyDownAndHorizontal => BOX_DRAWINGS_HEAVY_DOWN_AND_HORIZONTAL,
            BoxDrawing::BoxDrawingsLightUpAndHorizontal => BOX_DRAWINGS_LIGHT_UP_AND_HORIZONTAL,
            BoxDrawing::BoxDrawingsLeftHeavyAndRightUpLight => BOX_DRAWINGS_LEFT_HEAVY_AND_RIGHT_UP_LIGHT,
            BoxDrawing::BoxDrawingsRightHeavyAndLeftUpLight => BOX_DRAWINGS_RIGHT_HEAVY_AND_LEFT_UP_LIGHT,
            BoxDrawing::BoxDrawingsUpLightAndHorizontalHeavy => BOX_DRAWINGS_UP_LIGHT_AND_HORIZONTAL_HEAVY,
            BoxDrawing::BoxDrawingsUpHeavyAndHorizontalLight => BOX_DRAWINGS_UP_HEAVY_AND_HORIZONTAL_LIGHT,
            BoxDrawing::BoxDrawingsRightLightAndLeftUpHeavy => BOX_DRAWINGS_RIGHT_LIGHT_AND_LEFT_UP_HEAVY,
            BoxDrawing::BoxDrawingsLeftLightAndRightUpHeavy => BOX_DRAWINGS_LEFT_LIGHT_AND_RIGHT_UP_HEAVY,
            BoxDrawing::BoxDrawingsHeavyUpAndHorizontal => BOX_DRAWINGS_HEAVY_UP_AND_HORIZONTAL,
            BoxDrawing::BoxDrawingsLightVerticalAndHorizontal => BOX_DRAWINGS_LIGHT_VERTICAL_AND_HORIZONTAL,
            BoxDrawing::BoxDrawingsLeftHeavyAndRightVerticalLight => BOX_DRAWINGS_LEFT_HEAVY_AND_RIGHT_VERTICAL_LIGHT,
            BoxDrawing::BoxDrawingsRightHeavyAndLeftVerticalLight => BOX_DRAWINGS_RIGHT_HEAVY_AND_LEFT_VERTICAL_LIGHT,
            BoxDrawing::BoxDrawingsVerticalLightAndHorizontalHeavy => BOX_DRAWINGS_VERTICAL_LIGHT_AND_HORIZONTAL_HEAVY,
            BoxDrawing::BoxDrawingsUpHeavyAndDownHorizontalLight => BOX_DRAWINGS_UP_HEAVY_AND_DOWN_HORIZONTAL_LIGHT,
            BoxDrawing::BoxDrawingsDownHeavyAndUpHorizontalLight => BOX_DRAWINGS_DOWN_HEAVY_AND_UP_HORIZONTAL_LIGHT,
            BoxDrawing::BoxDrawingsVerticalHeavyAndHorizontalLight => BOX_DRAWINGS_VERTICAL_HEAVY_AND_HORIZONTAL_LIGHT,
            BoxDrawing::BoxDrawingsLeftUpHeavyAndRightDownLight => BOX_DRAWINGS_LEFT_UP_HEAVY_AND_RIGHT_DOWN_LIGHT,
            BoxDrawing::BoxDrawingsRightUpHeavyAndLeftDownLight => BOX_DRAWINGS_RIGHT_UP_HEAVY_AND_LEFT_DOWN_LIGHT,
            BoxDrawing::BoxDrawingsLeftDownHeavyAndRightUpLight => BOX_DRAWINGS_LEFT_DOWN_HEAVY_AND_RIGHT_UP_LIGHT,
            BoxDrawing::BoxDrawingsRightDownHeavyAndLeftUpLight => BOX_DRAWINGS_RIGHT_DOWN_HEAVY_AND_LEFT_UP_LIGHT,
            BoxDrawing::BoxDrawingsDownLightAndUpHorizontalHeavy => BOX_DRAWINGS_DOWN_LIGHT_AND_UP_HORIZONTAL_HEAVY,
            BoxDrawing::BoxDrawingsUpLightAndDownHorizontalHeavy => BOX_DRAWINGS_UP_LIGHT_AND_DOWN_HORIZONTAL_HEAVY,
            BoxDrawing::BoxDrawingsRightLightAndLeftVerticalHeavy => BOX_DRAWINGS_RIGHT_LIGHT_AND_LEFT_VERTICAL_HEAVY,
            BoxDrawing::BoxDrawingsLeftLightAndRightVerticalHeavy => BOX_DRAWINGS_LEFT_LIGHT_AND_RIGHT_VERTICAL_HEAVY,
            BoxDrawing::BoxDrawingsHeavyVerticalAndHorizontal => BOX_DRAWINGS_HEAVY_VERTICAL_AND_HORIZONTAL,
            BoxDrawing::BoxDrawingsLightDoubleDashHorizontal => BOX_DRAWINGS_LIGHT_DOUBLE_DASH_HORIZONTAL,
            BoxDrawing::BoxDrawingsHeavyDoubleDashHorizontal => BOX_DRAWINGS_HEAVY_DOUBLE_DASH_HORIZONTAL,
            BoxDrawing::BoxDrawingsLightDoubleDashVertical => BOX_DRAWINGS_LIGHT_DOUBLE_DASH_VERTICAL,
            BoxDrawing::BoxDrawingsHeavyDoubleDashVertical => BOX_DRAWINGS_HEAVY_DOUBLE_DASH_VERTICAL,
            BoxDrawing::BoxDrawingsDoubleHorizontal => BOX_DRAWINGS_DOUBLE_HORIZONTAL,
            BoxDrawing::BoxDrawingsDoubleVertical => BOX_DRAWINGS_DOUBLE_VERTICAL,
            BoxDrawing::BoxDrawingsDownSingleAndRightDouble => BOX_DRAWINGS_DOWN_SINGLE_AND_RIGHT_DOUBLE,
            BoxDrawing::BoxDrawingsDownDoubleAndRightSingle => BOX_DRAWINGS_DOWN_DOUBLE_AND_RIGHT_SINGLE,
            BoxDrawing::BoxDrawingsDoubleDownAndRight => BOX_DRAWINGS_DOUBLE_DOWN_AND_RIGHT,
            BoxDrawing::BoxDrawingsDownSingleAndLeftDouble => BOX_DRAWINGS_DOWN_SINGLE_AND_LEFT_DOUBLE,
            BoxDrawing::BoxDrawingsDownDoubleAndLeftSingle => BOX_DRAWINGS_DOWN_DOUBLE_AND_LEFT_SINGLE,
            BoxDrawing::BoxDrawingsDoubleDownAndLeft => BOX_DRAWINGS_DOUBLE_DOWN_AND_LEFT,
            BoxDrawing::BoxDrawingsUpSingleAndRightDouble => BOX_DRAWINGS_UP_SINGLE_AND_RIGHT_DOUBLE,
            BoxDrawing::BoxDrawingsUpDoubleAndRightSingle => BOX_DRAWINGS_UP_DOUBLE_AND_RIGHT_SINGLE,
            BoxDrawing::BoxDrawingsDoubleUpAndRight => BOX_DRAWINGS_DOUBLE_UP_AND_RIGHT,
            BoxDrawing::BoxDrawingsUpSingleAndLeftDouble => BOX_DRAWINGS_UP_SINGLE_AND_LEFT_DOUBLE,
            BoxDrawing::BoxDrawingsUpDoubleAndLeftSingle => BOX_DRAWINGS_UP_DOUBLE_AND_LEFT_SINGLE,
            BoxDrawing::BoxDrawingsDoubleUpAndLeft => BOX_DRAWINGS_DOUBLE_UP_AND_LEFT,
            BoxDrawing::BoxDrawingsVerticalSingleAndRightDouble => BOX_DRAWINGS_VERTICAL_SINGLE_AND_RIGHT_DOUBLE,
            BoxDrawing::BoxDrawingsVerticalDoubleAndRightSingle => BOX_DRAWINGS_VERTICAL_DOUBLE_AND_RIGHT_SINGLE,
            BoxDrawing::BoxDrawingsDoubleVerticalAndRight => BOX_DRAWINGS_DOUBLE_VERTICAL_AND_RIGHT,
            BoxDrawing::BoxDrawingsVerticalSingleAndLeftDouble => BOX_DRAWINGS_VERTICAL_SINGLE_AND_LEFT_DOUBLE,
            BoxDrawing::BoxDrawingsVerticalDoubleAndLeftSingle => BOX_DRAWINGS_VERTICAL_DOUBLE_AND_LEFT_SINGLE,
            BoxDrawing::BoxDrawingsDoubleVerticalAndLeft => BOX_DRAWINGS_DOUBLE_VERTICAL_AND_LEFT,
            BoxDrawing::BoxDrawingsDownSingleAndHorizontalDouble => BOX_DRAWINGS_DOWN_SINGLE_AND_HORIZONTAL_DOUBLE,
            BoxDrawing::BoxDrawingsDownDoubleAndHorizontalSingle => BOX_DRAWINGS_DOWN_DOUBLE_AND_HORIZONTAL_SINGLE,
            BoxDrawing::BoxDrawingsDoubleDownAndHorizontal => BOX_DRAWINGS_DOUBLE_DOWN_AND_HORIZONTAL,
            BoxDrawing::BoxDrawingsUpSingleAndHorizontalDouble => BOX_DRAWINGS_UP_SINGLE_AND_HORIZONTAL_DOUBLE,
            BoxDrawing::BoxDrawingsUpDoubleAndHorizontalSingle => BOX_DRAWINGS_UP_DOUBLE_AND_HORIZONTAL_SINGLE,
            BoxDrawing::BoxDrawingsDoubleUpAndHorizontal => BOX_DRAWINGS_DOUBLE_UP_AND_HORIZONTAL,
            BoxDrawing::BoxDrawingsVerticalSingleAndHorizontalDouble => BOX_DRAWINGS_VERTICAL_SINGLE_AND_HORIZONTAL_DOUBLE,
            BoxDrawing::BoxDrawingsVerticalDoubleAndHorizontalSingle => BOX_DRAWINGS_VERTICAL_DOUBLE_AND_HORIZONTAL_SINGLE,
            BoxDrawing::BoxDrawingsDoubleVerticalAndHorizontal => BOX_DRAWINGS_DOUBLE_VERTICAL_AND_HORIZONTAL,
            BoxDrawing::BoxDrawingsLightArcDownAndRight => BOX_DRAWINGS_LIGHT_ARC_DOWN_AND_RIGHT,
            BoxDrawing::BoxDrawingsLightArcDownAndLeft => BOX_DRAWINGS_LIGHT_ARC_DOWN_AND_LEFT,
            BoxDrawing::BoxDrawingsLightArcUpAndLeft => BOX_DRAWINGS_LIGHT_ARC_UP_AND_LEFT,
            BoxDrawing::BoxDrawingsLightArcUpAndRight => BOX_DRAWINGS_LIGHT_ARC_UP_AND_RIGHT,
            BoxDrawing::BoxDrawingsLightDiagonalUpperRightToLowerLeft => BOX_DRAWINGS_LIGHT_DIAGONAL_UPPER_RIGHT_TO_LOWER_LEFT,
            BoxDrawing::BoxDrawingsLightDiagonalUpperLeftToLowerRight => BOX_DRAWINGS_LIGHT_DIAGONAL_UPPER_LEFT_TO_LOWER_RIGHT,
            BoxDrawing::BoxDrawingsLightDiagonalCross => BOX_DRAWINGS_LIGHT_DIAGONAL_CROSS,
            BoxDrawing::BoxDrawingsLightLeft => BOX_DRAWINGS_LIGHT_LEFT,
            BoxDrawing::BoxDrawingsLightUp => BOX_DRAWINGS_LIGHT_UP,
            BoxDrawing::BoxDrawingsLightRight => BOX_DRAWINGS_LIGHT_RIGHT,
            BoxDrawing::BoxDrawingsLightDown => BOX_DRAWINGS_LIGHT_DOWN,
            BoxDrawing::BoxDrawingsHeavyLeft => BOX_DRAWINGS_HEAVY_LEFT,
            BoxDrawing::BoxDrawingsHeavyUp => BOX_DRAWINGS_HEAVY_UP,
            BoxDrawing::BoxDrawingsHeavyRight => BOX_DRAWINGS_HEAVY_RIGHT,
            BoxDrawing::BoxDrawingsHeavyDown => BOX_DRAWINGS_HEAVY_DOWN,
            BoxDrawing::BoxDrawingsLightLeftAndHeavyRight => BOX_DRAWINGS_LIGHT_LEFT_AND_HEAVY_RIGHT,
            BoxDrawing::BoxDrawingsLightUpAndHeavyDown => BOX_DRAWINGS_LIGHT_UP_AND_HEAVY_DOWN,
            BoxDrawing::BoxDrawingsHeavyLeftAndLightRight => BOX_DRAWINGS_HEAVY_LEFT_AND_LIGHT_RIGHT,
        }
    }
}

impl std::convert::TryFrom<char> for BoxDrawing {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            BOX_DRAWINGS_LIGHT_HORIZONTAL => Ok(BoxDrawing::BoxDrawingsLightHorizontal),
            BOX_DRAWINGS_HEAVY_HORIZONTAL => Ok(BoxDrawing::BoxDrawingsHeavyHorizontal),
            BOX_DRAWINGS_LIGHT_VERTICAL => Ok(BoxDrawing::BoxDrawingsLightVertical),
            BOX_DRAWINGS_HEAVY_VERTICAL => Ok(BoxDrawing::BoxDrawingsHeavyVertical),
            BOX_DRAWINGS_LIGHT_TRIPLE_DASH_HORIZONTAL => Ok(BoxDrawing::BoxDrawingsLightTripleDashHorizontal),
            BOX_DRAWINGS_HEAVY_TRIPLE_DASH_HORIZONTAL => Ok(BoxDrawing::BoxDrawingsHeavyTripleDashHorizontal),
            BOX_DRAWINGS_LIGHT_TRIPLE_DASH_VERTICAL => Ok(BoxDrawing::BoxDrawingsLightTripleDashVertical),
            BOX_DRAWINGS_HEAVY_TRIPLE_DASH_VERTICAL => Ok(BoxDrawing::BoxDrawingsHeavyTripleDashVertical),
            BOX_DRAWINGS_LIGHT_QUADRUPLE_DASH_HORIZONTAL => Ok(BoxDrawing::BoxDrawingsLightQuadrupleDashHorizontal),
            BOX_DRAWINGS_HEAVY_QUADRUPLE_DASH_HORIZONTAL => Ok(BoxDrawing::BoxDrawingsHeavyQuadrupleDashHorizontal),
            BOX_DRAWINGS_LIGHT_QUADRUPLE_DASH_VERTICAL => Ok(BoxDrawing::BoxDrawingsLightQuadrupleDashVertical),
            BOX_DRAWINGS_HEAVY_QUADRUPLE_DASH_VERTICAL => Ok(BoxDrawing::BoxDrawingsHeavyQuadrupleDashVertical),
            BOX_DRAWINGS_LIGHT_DOWN_AND_RIGHT => Ok(BoxDrawing::BoxDrawingsLightDownAndRight),
            BOX_DRAWINGS_DOWN_LIGHT_AND_RIGHT_HEAVY => Ok(BoxDrawing::BoxDrawingsDownLightAndRightHeavy),
            BOX_DRAWINGS_DOWN_HEAVY_AND_RIGHT_LIGHT => Ok(BoxDrawing::BoxDrawingsDownHeavyAndRightLight),
            BOX_DRAWINGS_HEAVY_DOWN_AND_RIGHT => Ok(BoxDrawing::BoxDrawingsHeavyDownAndRight),
            BOX_DRAWINGS_LIGHT_DOWN_AND_LEFT => Ok(BoxDrawing::BoxDrawingsLightDownAndLeft),
            BOX_DRAWINGS_DOWN_LIGHT_AND_LEFT_HEAVY => Ok(BoxDrawing::BoxDrawingsDownLightAndLeftHeavy),
            BOX_DRAWINGS_DOWN_HEAVY_AND_LEFT_LIGHT => Ok(BoxDrawing::BoxDrawingsDownHeavyAndLeftLight),
            BOX_DRAWINGS_HEAVY_DOWN_AND_LEFT => Ok(BoxDrawing::BoxDrawingsHeavyDownAndLeft),
            BOX_DRAWINGS_LIGHT_UP_AND_RIGHT => Ok(BoxDrawing::BoxDrawingsLightUpAndRight),
            BOX_DRAWINGS_UP_LIGHT_AND_RIGHT_HEAVY => Ok(BoxDrawing::BoxDrawingsUpLightAndRightHeavy),
            BOX_DRAWINGS_UP_HEAVY_AND_RIGHT_LIGHT => Ok(BoxDrawing::BoxDrawingsUpHeavyAndRightLight),
            BOX_DRAWINGS_HEAVY_UP_AND_RIGHT => Ok(BoxDrawing::BoxDrawingsHeavyUpAndRight),
            BOX_DRAWINGS_LIGHT_UP_AND_LEFT => Ok(BoxDrawing::BoxDrawingsLightUpAndLeft),
            BOX_DRAWINGS_UP_LIGHT_AND_LEFT_HEAVY => Ok(BoxDrawing::BoxDrawingsUpLightAndLeftHeavy),
            BOX_DRAWINGS_UP_HEAVY_AND_LEFT_LIGHT => Ok(BoxDrawing::BoxDrawingsUpHeavyAndLeftLight),
            BOX_DRAWINGS_HEAVY_UP_AND_LEFT => Ok(BoxDrawing::BoxDrawingsHeavyUpAndLeft),
            BOX_DRAWINGS_LIGHT_VERTICAL_AND_RIGHT => Ok(BoxDrawing::BoxDrawingsLightVerticalAndRight),
            BOX_DRAWINGS_VERTICAL_LIGHT_AND_RIGHT_HEAVY => Ok(BoxDrawing::BoxDrawingsVerticalLightAndRightHeavy),
            BOX_DRAWINGS_UP_HEAVY_AND_RIGHT_DOWN_LIGHT => Ok(BoxDrawing::BoxDrawingsUpHeavyAndRightDownLight),
            BOX_DRAWINGS_DOWN_HEAVY_AND_RIGHT_UP_LIGHT => Ok(BoxDrawing::BoxDrawingsDownHeavyAndRightUpLight),
            BOX_DRAWINGS_VERTICAL_HEAVY_AND_RIGHT_LIGHT => Ok(BoxDrawing::BoxDrawingsVerticalHeavyAndRightLight),
            BOX_DRAWINGS_DOWN_LIGHT_AND_RIGHT_UP_HEAVY => Ok(BoxDrawing::BoxDrawingsDownLightAndRightUpHeavy),
            BOX_DRAWINGS_UP_LIGHT_AND_RIGHT_DOWN_HEAVY => Ok(BoxDrawing::BoxDrawingsUpLightAndRightDownHeavy),
            BOX_DRAWINGS_HEAVY_VERTICAL_AND_RIGHT => Ok(BoxDrawing::BoxDrawingsHeavyVerticalAndRight),
            BOX_DRAWINGS_LIGHT_VERTICAL_AND_LEFT => Ok(BoxDrawing::BoxDrawingsLightVerticalAndLeft),
            BOX_DRAWINGS_VERTICAL_LIGHT_AND_LEFT_HEAVY => Ok(BoxDrawing::BoxDrawingsVerticalLightAndLeftHeavy),
            BOX_DRAWINGS_UP_HEAVY_AND_LEFT_DOWN_LIGHT => Ok(BoxDrawing::BoxDrawingsUpHeavyAndLeftDownLight),
            BOX_DRAWINGS_DOWN_HEAVY_AND_LEFT_UP_LIGHT => Ok(BoxDrawing::BoxDrawingsDownHeavyAndLeftUpLight),
            BOX_DRAWINGS_VERTICAL_HEAVY_AND_LEFT_LIGHT => Ok(BoxDrawing::BoxDrawingsVerticalHeavyAndLeftLight),
            BOX_DRAWINGS_DOWN_LIGHT_AND_LEFT_UP_HEAVY => Ok(BoxDrawing::BoxDrawingsDownLightAndLeftUpHeavy),
            BOX_DRAWINGS_UP_LIGHT_AND_LEFT_DOWN_HEAVY => Ok(BoxDrawing::BoxDrawingsUpLightAndLeftDownHeavy),
            BOX_DRAWINGS_HEAVY_VERTICAL_AND_LEFT => Ok(BoxDrawing::BoxDrawingsHeavyVerticalAndLeft),
            BOX_DRAWINGS_LIGHT_DOWN_AND_HORIZONTAL => Ok(BoxDrawing::BoxDrawingsLightDownAndHorizontal),
            BOX_DRAWINGS_LEFT_HEAVY_AND_RIGHT_DOWN_LIGHT => Ok(BoxDrawing::BoxDrawingsLeftHeavyAndRightDownLight),
            BOX_DRAWINGS_RIGHT_HEAVY_AND_LEFT_DOWN_LIGHT => Ok(BoxDrawing::BoxDrawingsRightHeavyAndLeftDownLight),
            BOX_DRAWINGS_DOWN_LIGHT_AND_HORIZONTAL_HEAVY => Ok(BoxDrawing::BoxDrawingsDownLightAndHorizontalHeavy),
            BOX_DRAWINGS_DOWN_HEAVY_AND_HORIZONTAL_LIGHT => Ok(BoxDrawing::BoxDrawingsDownHeavyAndHorizontalLight),
            BOX_DRAWINGS_RIGHT_LIGHT_AND_LEFT_DOWN_HEAVY => Ok(BoxDrawing::BoxDrawingsRightLightAndLeftDownHeavy),
            BOX_DRAWINGS_LEFT_LIGHT_AND_RIGHT_DOWN_HEAVY => Ok(BoxDrawing::BoxDrawingsLeftLightAndRightDownHeavy),
            BOX_DRAWINGS_HEAVY_DOWN_AND_HORIZONTAL => Ok(BoxDrawing::BoxDrawingsHeavyDownAndHorizontal),
            BOX_DRAWINGS_LIGHT_UP_AND_HORIZONTAL => Ok(BoxDrawing::BoxDrawingsLightUpAndHorizontal),
            BOX_DRAWINGS_LEFT_HEAVY_AND_RIGHT_UP_LIGHT => Ok(BoxDrawing::BoxDrawingsLeftHeavyAndRightUpLight),
            BOX_DRAWINGS_RIGHT_HEAVY_AND_LEFT_UP_LIGHT => Ok(BoxDrawing::BoxDrawingsRightHeavyAndLeftUpLight),
            BOX_DRAWINGS_UP_LIGHT_AND_HORIZONTAL_HEAVY => Ok(BoxDrawing::BoxDrawingsUpLightAndHorizontalHeavy),
            BOX_DRAWINGS_UP_HEAVY_AND_HORIZONTAL_LIGHT => Ok(BoxDrawing::BoxDrawingsUpHeavyAndHorizontalLight),
            BOX_DRAWINGS_RIGHT_LIGHT_AND_LEFT_UP_HEAVY => Ok(BoxDrawing::BoxDrawingsRightLightAndLeftUpHeavy),
            BOX_DRAWINGS_LEFT_LIGHT_AND_RIGHT_UP_HEAVY => Ok(BoxDrawing::BoxDrawingsLeftLightAndRightUpHeavy),
            BOX_DRAWINGS_HEAVY_UP_AND_HORIZONTAL => Ok(BoxDrawing::BoxDrawingsHeavyUpAndHorizontal),
            BOX_DRAWINGS_LIGHT_VERTICAL_AND_HORIZONTAL => Ok(BoxDrawing::BoxDrawingsLightVerticalAndHorizontal),
            BOX_DRAWINGS_LEFT_HEAVY_AND_RIGHT_VERTICAL_LIGHT => Ok(BoxDrawing::BoxDrawingsLeftHeavyAndRightVerticalLight),
            BOX_DRAWINGS_RIGHT_HEAVY_AND_LEFT_VERTICAL_LIGHT => Ok(BoxDrawing::BoxDrawingsRightHeavyAndLeftVerticalLight),
            BOX_DRAWINGS_VERTICAL_LIGHT_AND_HORIZONTAL_HEAVY => Ok(BoxDrawing::BoxDrawingsVerticalLightAndHorizontalHeavy),
            BOX_DRAWINGS_UP_HEAVY_AND_DOWN_HORIZONTAL_LIGHT => Ok(BoxDrawing::BoxDrawingsUpHeavyAndDownHorizontalLight),
            BOX_DRAWINGS_DOWN_HEAVY_AND_UP_HORIZONTAL_LIGHT => Ok(BoxDrawing::BoxDrawingsDownHeavyAndUpHorizontalLight),
            BOX_DRAWINGS_VERTICAL_HEAVY_AND_HORIZONTAL_LIGHT => Ok(BoxDrawing::BoxDrawingsVerticalHeavyAndHorizontalLight),
            BOX_DRAWINGS_LEFT_UP_HEAVY_AND_RIGHT_DOWN_LIGHT => Ok(BoxDrawing::BoxDrawingsLeftUpHeavyAndRightDownLight),
            BOX_DRAWINGS_RIGHT_UP_HEAVY_AND_LEFT_DOWN_LIGHT => Ok(BoxDrawing::BoxDrawingsRightUpHeavyAndLeftDownLight),
            BOX_DRAWINGS_LEFT_DOWN_HEAVY_AND_RIGHT_UP_LIGHT => Ok(BoxDrawing::BoxDrawingsLeftDownHeavyAndRightUpLight),
            BOX_DRAWINGS_RIGHT_DOWN_HEAVY_AND_LEFT_UP_LIGHT => Ok(BoxDrawing::BoxDrawingsRightDownHeavyAndLeftUpLight),
            BOX_DRAWINGS_DOWN_LIGHT_AND_UP_HORIZONTAL_HEAVY => Ok(BoxDrawing::BoxDrawingsDownLightAndUpHorizontalHeavy),
            BOX_DRAWINGS_UP_LIGHT_AND_DOWN_HORIZONTAL_HEAVY => Ok(BoxDrawing::BoxDrawingsUpLightAndDownHorizontalHeavy),
            BOX_DRAWINGS_RIGHT_LIGHT_AND_LEFT_VERTICAL_HEAVY => Ok(BoxDrawing::BoxDrawingsRightLightAndLeftVerticalHeavy),
            BOX_DRAWINGS_LEFT_LIGHT_AND_RIGHT_VERTICAL_HEAVY => Ok(BoxDrawing::BoxDrawingsLeftLightAndRightVerticalHeavy),
            BOX_DRAWINGS_HEAVY_VERTICAL_AND_HORIZONTAL => Ok(BoxDrawing::BoxDrawingsHeavyVerticalAndHorizontal),
            BOX_DRAWINGS_LIGHT_DOUBLE_DASH_HORIZONTAL => Ok(BoxDrawing::BoxDrawingsLightDoubleDashHorizontal),
            BOX_DRAWINGS_HEAVY_DOUBLE_DASH_HORIZONTAL => Ok(BoxDrawing::BoxDrawingsHeavyDoubleDashHorizontal),
            BOX_DRAWINGS_LIGHT_DOUBLE_DASH_VERTICAL => Ok(BoxDrawing::BoxDrawingsLightDoubleDashVertical),
            BOX_DRAWINGS_HEAVY_DOUBLE_DASH_VERTICAL => Ok(BoxDrawing::BoxDrawingsHeavyDoubleDashVertical),
            BOX_DRAWINGS_DOUBLE_HORIZONTAL => Ok(BoxDrawing::BoxDrawingsDoubleHorizontal),
            BOX_DRAWINGS_DOUBLE_VERTICAL => Ok(BoxDrawing::BoxDrawingsDoubleVertical),
            BOX_DRAWINGS_DOWN_SINGLE_AND_RIGHT_DOUBLE => Ok(BoxDrawing::BoxDrawingsDownSingleAndRightDouble),
            BOX_DRAWINGS_DOWN_DOUBLE_AND_RIGHT_SINGLE => Ok(BoxDrawing::BoxDrawingsDownDoubleAndRightSingle),
            BOX_DRAWINGS_DOUBLE_DOWN_AND_RIGHT => Ok(BoxDrawing::BoxDrawingsDoubleDownAndRight),
            BOX_DRAWINGS_DOWN_SINGLE_AND_LEFT_DOUBLE => Ok(BoxDrawing::BoxDrawingsDownSingleAndLeftDouble),
            BOX_DRAWINGS_DOWN_DOUBLE_AND_LEFT_SINGLE => Ok(BoxDrawing::BoxDrawingsDownDoubleAndLeftSingle),
            BOX_DRAWINGS_DOUBLE_DOWN_AND_LEFT => Ok(BoxDrawing::BoxDrawingsDoubleDownAndLeft),
            BOX_DRAWINGS_UP_SINGLE_AND_RIGHT_DOUBLE => Ok(BoxDrawing::BoxDrawingsUpSingleAndRightDouble),
            BOX_DRAWINGS_UP_DOUBLE_AND_RIGHT_SINGLE => Ok(BoxDrawing::BoxDrawingsUpDoubleAndRightSingle),
            BOX_DRAWINGS_DOUBLE_UP_AND_RIGHT => Ok(BoxDrawing::BoxDrawingsDoubleUpAndRight),
            BOX_DRAWINGS_UP_SINGLE_AND_LEFT_DOUBLE => Ok(BoxDrawing::BoxDrawingsUpSingleAndLeftDouble),
            BOX_DRAWINGS_UP_DOUBLE_AND_LEFT_SINGLE => Ok(BoxDrawing::BoxDrawingsUpDoubleAndLeftSingle),
            BOX_DRAWINGS_DOUBLE_UP_AND_LEFT => Ok(BoxDrawing::BoxDrawingsDoubleUpAndLeft),
            BOX_DRAWINGS_VERTICAL_SINGLE_AND_RIGHT_DOUBLE => Ok(BoxDrawing::BoxDrawingsVerticalSingleAndRightDouble),
            BOX_DRAWINGS_VERTICAL_DOUBLE_AND_RIGHT_SINGLE => Ok(BoxDrawing::BoxDrawingsVerticalDoubleAndRightSingle),
            BOX_DRAWINGS_DOUBLE_VERTICAL_AND_RIGHT => Ok(BoxDrawing::BoxDrawingsDoubleVerticalAndRight),
            BOX_DRAWINGS_VERTICAL_SINGLE_AND_LEFT_DOUBLE => Ok(BoxDrawing::BoxDrawingsVerticalSingleAndLeftDouble),
            BOX_DRAWINGS_VERTICAL_DOUBLE_AND_LEFT_SINGLE => Ok(BoxDrawing::BoxDrawingsVerticalDoubleAndLeftSingle),
            BOX_DRAWINGS_DOUBLE_VERTICAL_AND_LEFT => Ok(BoxDrawing::BoxDrawingsDoubleVerticalAndLeft),
            BOX_DRAWINGS_DOWN_SINGLE_AND_HORIZONTAL_DOUBLE => Ok(BoxDrawing::BoxDrawingsDownSingleAndHorizontalDouble),
            BOX_DRAWINGS_DOWN_DOUBLE_AND_HORIZONTAL_SINGLE => Ok(BoxDrawing::BoxDrawingsDownDoubleAndHorizontalSingle),
            BOX_DRAWINGS_DOUBLE_DOWN_AND_HORIZONTAL => Ok(BoxDrawing::BoxDrawingsDoubleDownAndHorizontal),
            BOX_DRAWINGS_UP_SINGLE_AND_HORIZONTAL_DOUBLE => Ok(BoxDrawing::BoxDrawingsUpSingleAndHorizontalDouble),
            BOX_DRAWINGS_UP_DOUBLE_AND_HORIZONTAL_SINGLE => Ok(BoxDrawing::BoxDrawingsUpDoubleAndHorizontalSingle),
            BOX_DRAWINGS_DOUBLE_UP_AND_HORIZONTAL => Ok(BoxDrawing::BoxDrawingsDoubleUpAndHorizontal),
            BOX_DRAWINGS_VERTICAL_SINGLE_AND_HORIZONTAL_DOUBLE => Ok(BoxDrawing::BoxDrawingsVerticalSingleAndHorizontalDouble),
            BOX_DRAWINGS_VERTICAL_DOUBLE_AND_HORIZONTAL_SINGLE => Ok(BoxDrawing::BoxDrawingsVerticalDoubleAndHorizontalSingle),
            BOX_DRAWINGS_DOUBLE_VERTICAL_AND_HORIZONTAL => Ok(BoxDrawing::BoxDrawingsDoubleVerticalAndHorizontal),
            BOX_DRAWINGS_LIGHT_ARC_DOWN_AND_RIGHT => Ok(BoxDrawing::BoxDrawingsLightArcDownAndRight),
            BOX_DRAWINGS_LIGHT_ARC_DOWN_AND_LEFT => Ok(BoxDrawing::BoxDrawingsLightArcDownAndLeft),
            BOX_DRAWINGS_LIGHT_ARC_UP_AND_LEFT => Ok(BoxDrawing::BoxDrawingsLightArcUpAndLeft),
            BOX_DRAWINGS_LIGHT_ARC_UP_AND_RIGHT => Ok(BoxDrawing::BoxDrawingsLightArcUpAndRight),
            BOX_DRAWINGS_LIGHT_DIAGONAL_UPPER_RIGHT_TO_LOWER_LEFT => Ok(BoxDrawing::BoxDrawingsLightDiagonalUpperRightToLowerLeft),
            BOX_DRAWINGS_LIGHT_DIAGONAL_UPPER_LEFT_TO_LOWER_RIGHT => Ok(BoxDrawing::BoxDrawingsLightDiagonalUpperLeftToLowerRight),
            BOX_DRAWINGS_LIGHT_DIAGONAL_CROSS => Ok(BoxDrawing::BoxDrawingsLightDiagonalCross),
            BOX_DRAWINGS_LIGHT_LEFT => Ok(BoxDrawing::BoxDrawingsLightLeft),
            BOX_DRAWINGS_LIGHT_UP => Ok(BoxDrawing::BoxDrawingsLightUp),
            BOX_DRAWINGS_LIGHT_RIGHT => Ok(BoxDrawing::BoxDrawingsLightRight),
            BOX_DRAWINGS_LIGHT_DOWN => Ok(BoxDrawing::BoxDrawingsLightDown),
            BOX_DRAWINGS_HEAVY_LEFT => Ok(BoxDrawing::BoxDrawingsHeavyLeft),
            BOX_DRAWINGS_HEAVY_UP => Ok(BoxDrawing::BoxDrawingsHeavyUp),
            BOX_DRAWINGS_HEAVY_RIGHT => Ok(BoxDrawing::BoxDrawingsHeavyRight),
            BOX_DRAWINGS_HEAVY_DOWN => Ok(BoxDrawing::BoxDrawingsHeavyDown),
            BOX_DRAWINGS_LIGHT_LEFT_AND_HEAVY_RIGHT => Ok(BoxDrawing::BoxDrawingsLightLeftAndHeavyRight),
            BOX_DRAWINGS_LIGHT_UP_AND_HEAVY_DOWN => Ok(BoxDrawing::BoxDrawingsLightUpAndHeavyDown),
            BOX_DRAWINGS_HEAVY_LEFT_AND_LIGHT_RIGHT => Ok(BoxDrawing::BoxDrawingsHeavyLeftAndLightRight),
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
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        BoxDrawing::BoxDrawingsLightHorizontal
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            BoxDrawing::BoxDrawingsLightHorizontal => "box drawings light horizontal",
            BoxDrawing::BoxDrawingsHeavyHorizontal => "box drawings heavy horizontal",
            BoxDrawing::BoxDrawingsLightVertical => "box drawings light vertical",
            BoxDrawing::BoxDrawingsHeavyVertical => "box drawings heavy vertical",
            BoxDrawing::BoxDrawingsLightTripleDashHorizontal => "box drawings light triple dash horizontal",
            BoxDrawing::BoxDrawingsHeavyTripleDashHorizontal => "box drawings heavy triple dash horizontal",
            BoxDrawing::BoxDrawingsLightTripleDashVertical => "box drawings light triple dash vertical",
            BoxDrawing::BoxDrawingsHeavyTripleDashVertical => "box drawings heavy triple dash vertical",
            BoxDrawing::BoxDrawingsLightQuadrupleDashHorizontal => "box drawings light quadruple dash horizontal",
            BoxDrawing::BoxDrawingsHeavyQuadrupleDashHorizontal => "box drawings heavy quadruple dash horizontal",
            BoxDrawing::BoxDrawingsLightQuadrupleDashVertical => "box drawings light quadruple dash vertical",
            BoxDrawing::BoxDrawingsHeavyQuadrupleDashVertical => "box drawings heavy quadruple dash vertical",
            BoxDrawing::BoxDrawingsLightDownAndRight => "box drawings light down and right",
            BoxDrawing::BoxDrawingsDownLightAndRightHeavy => "box drawings down light and right heavy",
            BoxDrawing::BoxDrawingsDownHeavyAndRightLight => "box drawings down heavy and right light",
            BoxDrawing::BoxDrawingsHeavyDownAndRight => "box drawings heavy down and right",
            BoxDrawing::BoxDrawingsLightDownAndLeft => "box drawings light down and left",
            BoxDrawing::BoxDrawingsDownLightAndLeftHeavy => "box drawings down light and left heavy",
            BoxDrawing::BoxDrawingsDownHeavyAndLeftLight => "box drawings down heavy and left light",
            BoxDrawing::BoxDrawingsHeavyDownAndLeft => "box drawings heavy down and left",
            BoxDrawing::BoxDrawingsLightUpAndRight => "box drawings light up and right",
            BoxDrawing::BoxDrawingsUpLightAndRightHeavy => "box drawings up light and right heavy",
            BoxDrawing::BoxDrawingsUpHeavyAndRightLight => "box drawings up heavy and right light",
            BoxDrawing::BoxDrawingsHeavyUpAndRight => "box drawings heavy up and right",
            BoxDrawing::BoxDrawingsLightUpAndLeft => "box drawings light up and left",
            BoxDrawing::BoxDrawingsUpLightAndLeftHeavy => "box drawings up light and left heavy",
            BoxDrawing::BoxDrawingsUpHeavyAndLeftLight => "box drawings up heavy and left light",
            BoxDrawing::BoxDrawingsHeavyUpAndLeft => "box drawings heavy up and left",
            BoxDrawing::BoxDrawingsLightVerticalAndRight => "box drawings light vertical and right",
            BoxDrawing::BoxDrawingsVerticalLightAndRightHeavy => "box drawings vertical light and right heavy",
            BoxDrawing::BoxDrawingsUpHeavyAndRightDownLight => "box drawings up heavy and right down light",
            BoxDrawing::BoxDrawingsDownHeavyAndRightUpLight => "box drawings down heavy and right up light",
            BoxDrawing::BoxDrawingsVerticalHeavyAndRightLight => "box drawings vertical heavy and right light",
            BoxDrawing::BoxDrawingsDownLightAndRightUpHeavy => "box drawings down light and right up heavy",
            BoxDrawing::BoxDrawingsUpLightAndRightDownHeavy => "box drawings up light and right down heavy",
            BoxDrawing::BoxDrawingsHeavyVerticalAndRight => "box drawings heavy vertical and right",
            BoxDrawing::BoxDrawingsLightVerticalAndLeft => "box drawings light vertical and left",
            BoxDrawing::BoxDrawingsVerticalLightAndLeftHeavy => "box drawings vertical light and left heavy",
            BoxDrawing::BoxDrawingsUpHeavyAndLeftDownLight => "box drawings up heavy and left down light",
            BoxDrawing::BoxDrawingsDownHeavyAndLeftUpLight => "box drawings down heavy and left up light",
            BoxDrawing::BoxDrawingsVerticalHeavyAndLeftLight => "box drawings vertical heavy and left light",
            BoxDrawing::BoxDrawingsDownLightAndLeftUpHeavy => "box drawings down light and left up heavy",
            BoxDrawing::BoxDrawingsUpLightAndLeftDownHeavy => "box drawings up light and left down heavy",
            BoxDrawing::BoxDrawingsHeavyVerticalAndLeft => "box drawings heavy vertical and left",
            BoxDrawing::BoxDrawingsLightDownAndHorizontal => "box drawings light down and horizontal",
            BoxDrawing::BoxDrawingsLeftHeavyAndRightDownLight => "box drawings left heavy and right down light",
            BoxDrawing::BoxDrawingsRightHeavyAndLeftDownLight => "box drawings right heavy and left down light",
            BoxDrawing::BoxDrawingsDownLightAndHorizontalHeavy => "box drawings down light and horizontal heavy",
            BoxDrawing::BoxDrawingsDownHeavyAndHorizontalLight => "box drawings down heavy and horizontal light",
            BoxDrawing::BoxDrawingsRightLightAndLeftDownHeavy => "box drawings right light and left down heavy",
            BoxDrawing::BoxDrawingsLeftLightAndRightDownHeavy => "box drawings left light and right down heavy",
            BoxDrawing::BoxDrawingsHeavyDownAndHorizontal => "box drawings heavy down and horizontal",
            BoxDrawing::BoxDrawingsLightUpAndHorizontal => "box drawings light up and horizontal",
            BoxDrawing::BoxDrawingsLeftHeavyAndRightUpLight => "box drawings left heavy and right up light",
            BoxDrawing::BoxDrawingsRightHeavyAndLeftUpLight => "box drawings right heavy and left up light",
            BoxDrawing::BoxDrawingsUpLightAndHorizontalHeavy => "box drawings up light and horizontal heavy",
            BoxDrawing::BoxDrawingsUpHeavyAndHorizontalLight => "box drawings up heavy and horizontal light",
            BoxDrawing::BoxDrawingsRightLightAndLeftUpHeavy => "box drawings right light and left up heavy",
            BoxDrawing::BoxDrawingsLeftLightAndRightUpHeavy => "box drawings left light and right up heavy",
            BoxDrawing::BoxDrawingsHeavyUpAndHorizontal => "box drawings heavy up and horizontal",
            BoxDrawing::BoxDrawingsLightVerticalAndHorizontal => "box drawings light vertical and horizontal",
            BoxDrawing::BoxDrawingsLeftHeavyAndRightVerticalLight => "box drawings left heavy and right vertical light",
            BoxDrawing::BoxDrawingsRightHeavyAndLeftVerticalLight => "box drawings right heavy and left vertical light",
            BoxDrawing::BoxDrawingsVerticalLightAndHorizontalHeavy => "box drawings vertical light and horizontal heavy",
            BoxDrawing::BoxDrawingsUpHeavyAndDownHorizontalLight => "box drawings up heavy and down horizontal light",
            BoxDrawing::BoxDrawingsDownHeavyAndUpHorizontalLight => "box drawings down heavy and up horizontal light",
            BoxDrawing::BoxDrawingsVerticalHeavyAndHorizontalLight => "box drawings vertical heavy and horizontal light",
            BoxDrawing::BoxDrawingsLeftUpHeavyAndRightDownLight => "box drawings left up heavy and right down light",
            BoxDrawing::BoxDrawingsRightUpHeavyAndLeftDownLight => "box drawings right up heavy and left down light",
            BoxDrawing::BoxDrawingsLeftDownHeavyAndRightUpLight => "box drawings left down heavy and right up light",
            BoxDrawing::BoxDrawingsRightDownHeavyAndLeftUpLight => "box drawings right down heavy and left up light",
            BoxDrawing::BoxDrawingsDownLightAndUpHorizontalHeavy => "box drawings down light and up horizontal heavy",
            BoxDrawing::BoxDrawingsUpLightAndDownHorizontalHeavy => "box drawings up light and down horizontal heavy",
            BoxDrawing::BoxDrawingsRightLightAndLeftVerticalHeavy => "box drawings right light and left vertical heavy",
            BoxDrawing::BoxDrawingsLeftLightAndRightVerticalHeavy => "box drawings left light and right vertical heavy",
            BoxDrawing::BoxDrawingsHeavyVerticalAndHorizontal => "box drawings heavy vertical and horizontal",
            BoxDrawing::BoxDrawingsLightDoubleDashHorizontal => "box drawings light double dash horizontal",
            BoxDrawing::BoxDrawingsHeavyDoubleDashHorizontal => "box drawings heavy double dash horizontal",
            BoxDrawing::BoxDrawingsLightDoubleDashVertical => "box drawings light double dash vertical",
            BoxDrawing::BoxDrawingsHeavyDoubleDashVertical => "box drawings heavy double dash vertical",
            BoxDrawing::BoxDrawingsDoubleHorizontal => "box drawings double horizontal",
            BoxDrawing::BoxDrawingsDoubleVertical => "box drawings double vertical",
            BoxDrawing::BoxDrawingsDownSingleAndRightDouble => "box drawings down single and right double",
            BoxDrawing::BoxDrawingsDownDoubleAndRightSingle => "box drawings down double and right single",
            BoxDrawing::BoxDrawingsDoubleDownAndRight => "box drawings double down and right",
            BoxDrawing::BoxDrawingsDownSingleAndLeftDouble => "box drawings down single and left double",
            BoxDrawing::BoxDrawingsDownDoubleAndLeftSingle => "box drawings down double and left single",
            BoxDrawing::BoxDrawingsDoubleDownAndLeft => "box drawings double down and left",
            BoxDrawing::BoxDrawingsUpSingleAndRightDouble => "box drawings up single and right double",
            BoxDrawing::BoxDrawingsUpDoubleAndRightSingle => "box drawings up double and right single",
            BoxDrawing::BoxDrawingsDoubleUpAndRight => "box drawings double up and right",
            BoxDrawing::BoxDrawingsUpSingleAndLeftDouble => "box drawings up single and left double",
            BoxDrawing::BoxDrawingsUpDoubleAndLeftSingle => "box drawings up double and left single",
            BoxDrawing::BoxDrawingsDoubleUpAndLeft => "box drawings double up and left",
            BoxDrawing::BoxDrawingsVerticalSingleAndRightDouble => "box drawings vertical single and right double",
            BoxDrawing::BoxDrawingsVerticalDoubleAndRightSingle => "box drawings vertical double and right single",
            BoxDrawing::BoxDrawingsDoubleVerticalAndRight => "box drawings double vertical and right",
            BoxDrawing::BoxDrawingsVerticalSingleAndLeftDouble => "box drawings vertical single and left double",
            BoxDrawing::BoxDrawingsVerticalDoubleAndLeftSingle => "box drawings vertical double and left single",
            BoxDrawing::BoxDrawingsDoubleVerticalAndLeft => "box drawings double vertical and left",
            BoxDrawing::BoxDrawingsDownSingleAndHorizontalDouble => "box drawings down single and horizontal double",
            BoxDrawing::BoxDrawingsDownDoubleAndHorizontalSingle => "box drawings down double and horizontal single",
            BoxDrawing::BoxDrawingsDoubleDownAndHorizontal => "box drawings double down and horizontal",
            BoxDrawing::BoxDrawingsUpSingleAndHorizontalDouble => "box drawings up single and horizontal double",
            BoxDrawing::BoxDrawingsUpDoubleAndHorizontalSingle => "box drawings up double and horizontal single",
            BoxDrawing::BoxDrawingsDoubleUpAndHorizontal => "box drawings double up and horizontal",
            BoxDrawing::BoxDrawingsVerticalSingleAndHorizontalDouble => "box drawings vertical single and horizontal double",
            BoxDrawing::BoxDrawingsVerticalDoubleAndHorizontalSingle => "box drawings vertical double and horizontal single",
            BoxDrawing::BoxDrawingsDoubleVerticalAndHorizontal => "box drawings double vertical and horizontal",
            BoxDrawing::BoxDrawingsLightArcDownAndRight => "box drawings light arc down and right",
            BoxDrawing::BoxDrawingsLightArcDownAndLeft => "box drawings light arc down and left",
            BoxDrawing::BoxDrawingsLightArcUpAndLeft => "box drawings light arc up and left",
            BoxDrawing::BoxDrawingsLightArcUpAndRight => "box drawings light arc up and right",
            BoxDrawing::BoxDrawingsLightDiagonalUpperRightToLowerLeft => "box drawings light diagonal upper right to lower left",
            BoxDrawing::BoxDrawingsLightDiagonalUpperLeftToLowerRight => "box drawings light diagonal upper left to lower right",
            BoxDrawing::BoxDrawingsLightDiagonalCross => "box drawings light diagonal cross",
            BoxDrawing::BoxDrawingsLightLeft => "box drawings light left",
            BoxDrawing::BoxDrawingsLightUp => "box drawings light up",
            BoxDrawing::BoxDrawingsLightRight => "box drawings light right",
            BoxDrawing::BoxDrawingsLightDown => "box drawings light down",
            BoxDrawing::BoxDrawingsHeavyLeft => "box drawings heavy left",
            BoxDrawing::BoxDrawingsHeavyUp => "box drawings heavy up",
            BoxDrawing::BoxDrawingsHeavyRight => "box drawings heavy right",
            BoxDrawing::BoxDrawingsHeavyDown => "box drawings heavy down",
            BoxDrawing::BoxDrawingsLightLeftAndHeavyRight => "box drawings light left and heavy right",
            BoxDrawing::BoxDrawingsLightUpAndHeavyDown => "box drawings light up and heavy down",
            BoxDrawing::BoxDrawingsHeavyLeftAndLightRight => "box drawings heavy left and light right",
        }
    }
}
