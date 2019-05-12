/// \u{2b00} → \u{2bff}
///
/// ⬀ ⬁ ⬂ ⬃ ⬄ ⬅ ⬆ ⬇ ⬈ ⬉ ⬊ ⬋ ⬌ ⬍ ⬎ ⬏\
/// ⬐ ⬑ ⬒ ⬓ ⬔ ⬕ ⬖ ⬗ ⬘ ⬙ ⬚ ⬛ ⬜ ⬝ ⬞ ⬟\
/// ⬠ ⬡ ⬢ ⬣ ⬤ ⬥ ⬦ ⬧ ⬨ ⬩ ⬪ ⬫ ⬬ ⬭ ⬮ ⬯\
/// ⬰ ⬱ ⬲ ⬳ ⬴ ⬵ ⬶ ⬷ ⬸ ⬹ ⬺ ⬻ ⬼ ⬽ ⬾ ⬿\
/// ⭀ ⭁ ⭂ ⭃ ⭄ ⭅ ⭆ ⭇ ⭈ ⭉ ⭊ ⭋ ⭌ ⭍ ⭎ ⭏\
/// ⭐ ⭑ ⭒ ⭓ ⭔ ⭕ ⭖ ⭗ ⭘ ⭙ ⭚ ⭛ ⭜ ⭝ ⭞ ⭟\
/// ⭠ ⭡ ⭢ ⭣ ⭤ ⭥ ⭦ ⭧ ⭨ ⭩ ⭪ ⭫ ⭬ ⭭ ⭮ ⭯\
/// ⭰ ⭱ ⭲ ⭳ ⭶ ⭷ ⭸ ⭹ ⭺ ⭻ ⭼ ⭽ ⭾ ⭿ ⮀ ⮁\
/// ⮂ ⮃ ⮄ ⮅ ⮆ ⮇ ⮈ ⮉ ⮊ ⮋ ⮌ ⮍ ⮎ ⮏ ⮐ ⮑\
/// ⮒ ⮓ ⮔ ⮕ ⮘ ⮙ ⮚ ⮛ ⮜ ⮝ ⮞ ⮟ ⮠ ⮡ ⮢ ⮣\
/// ⮤ ⮥ ⮦ ⮧ ⮨ ⮩ ⮪ ⮫ ⮬ ⮭ ⮮ ⮯ ⮰ ⮱ ⮲ ⮳\
/// ⮴ ⮵ ⮶ ⮷ ⮸ ⮹ ⮺ ⮻ ⮼ ⮽ ⮾ ⮿ ⯀ ⯁ ⯂ ⯃\
/// ⯄ ⯅ ⯆ ⯇ ⯈ ⯉ ⯊ ⯋ ⯌ ⯍ ⯎ ⯏ ⯐ ⯑ ⯒ ⯓\
/// ⯔ ⯕ ⯖ ⯗ ⯘ ⯙ ⯚ ⯛ ⯜ ⯝ ⯞ ⯟ ⯠ ⯡ ⯢ ⯣\
/// ⯤ ⯥ ⯦ ⯧ ⯨ ⯩ ⯪ ⯫ ⯬ ⯭ ⯮ ⯯ ⯰ ⯱ ⯲ ⯳\
/// ⯴ ⯵ ⯶ ⯷ ⯸ ⯹ ⯺ ⯻ ⯼ ⯽ ⯾\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{2b00}: '⬀'
    pub const NORTH_EAST_WHITE_ARROW: char = '⬀';
    /// \u{2b01}: '⬁'
    pub const NORTH_WEST_WHITE_ARROW: char = '⬁';
    /// \u{2b02}: '⬂'
    pub const SOUTH_EAST_WHITE_ARROW: char = '⬂';
    /// \u{2b03}: '⬃'
    pub const SOUTH_WEST_WHITE_ARROW: char = '⬃';
    /// \u{2b04}: '⬄'
    pub const LEFT_RIGHT_WHITE_ARROW: char = '⬄';
    /// \u{2b05}: '⬅'
    pub const LEFTWARDS_BLACK_ARROW: char = '⬅';
    /// \u{2b06}: '⬆'
    pub const UPWARDS_BLACK_ARROW: char = '⬆';
    /// \u{2b07}: '⬇'
    pub const DOWNWARDS_BLACK_ARROW: char = '⬇';
    /// \u{2b08}: '⬈'
    pub const NORTH_EAST_BLACK_ARROW: char = '⬈';
    /// \u{2b09}: '⬉'
    pub const NORTH_WEST_BLACK_ARROW: char = '⬉';
    /// \u{2b0a}: '⬊'
    pub const SOUTH_EAST_BLACK_ARROW: char = '⬊';
    /// \u{2b0b}: '⬋'
    pub const SOUTH_WEST_BLACK_ARROW: char = '⬋';
    /// \u{2b0c}: '⬌'
    pub const LEFT_RIGHT_BLACK_ARROW: char = '⬌';
    /// \u{2b0d}: '⬍'
    pub const UP_DOWN_BLACK_ARROW: char = '⬍';
    /// \u{2b0e}: '⬎'
    pub const RIGHTWARDS_ARROW_WITH_TIP_DOWNWARDS: char = '⬎';
    /// \u{2b0f}: '⬏'
    pub const RIGHTWARDS_ARROW_WITH_TIP_UPWARDS: char = '⬏';
    /// \u{2b10}: '⬐'
    pub const LEFTWARDS_ARROW_WITH_TIP_DOWNWARDS: char = '⬐';
    /// \u{2b11}: '⬑'
    pub const LEFTWARDS_ARROW_WITH_TIP_UPWARDS: char = '⬑';
    /// \u{2b12}: '⬒'
    pub const SQUARE_WITH_TOP_HALF_BLACK: char = '⬒';
    /// \u{2b13}: '⬓'
    pub const SQUARE_WITH_BOTTOM_HALF_BLACK: char = '⬓';
    /// \u{2b14}: '⬔'
    pub const SQUARE_WITH_UPPER_RIGHT_DIAGONAL_HALF_BLACK: char = '⬔';
    /// \u{2b15}: '⬕'
    pub const SQUARE_WITH_LOWER_LEFT_DIAGONAL_HALF_BLACK: char = '⬕';
    /// \u{2b16}: '⬖'
    pub const DIAMOND_WITH_LEFT_HALF_BLACK: char = '⬖';
    /// \u{2b17}: '⬗'
    pub const DIAMOND_WITH_RIGHT_HALF_BLACK: char = '⬗';
    /// \u{2b18}: '⬘'
    pub const DIAMOND_WITH_TOP_HALF_BLACK: char = '⬘';
    /// \u{2b19}: '⬙'
    pub const DIAMOND_WITH_BOTTOM_HALF_BLACK: char = '⬙';
    /// \u{2b1a}: '⬚'
    pub const DOTTED_SQUARE: char = '⬚';
    /// \u{2b1b}: '⬛'
    pub const BLACK_LARGE_SQUARE: char = '⬛';
    /// \u{2b1c}: '⬜'
    pub const WHITE_LARGE_SQUARE: char = '⬜';
    /// \u{2b1d}: '⬝'
    pub const BLACK_VERY_SMALL_SQUARE: char = '⬝';
    /// \u{2b1e}: '⬞'
    pub const WHITE_VERY_SMALL_SQUARE: char = '⬞';
    /// \u{2b1f}: '⬟'
    pub const BLACK_PENTAGON: char = '⬟';
    /// \u{2b20}: '⬠'
    pub const WHITE_PENTAGON: char = '⬠';
    /// \u{2b21}: '⬡'
    pub const WHITE_HEXAGON: char = '⬡';
    /// \u{2b22}: '⬢'
    pub const BLACK_HEXAGON: char = '⬢';
    /// \u{2b23}: '⬣'
    pub const HORIZONTAL_BLACK_HEXAGON: char = '⬣';
    /// \u{2b24}: '⬤'
    pub const BLACK_LARGE_CIRCLE: char = '⬤';
    /// \u{2b25}: '⬥'
    pub const BLACK_MEDIUM_DIAMOND: char = '⬥';
    /// \u{2b26}: '⬦'
    pub const WHITE_MEDIUM_DIAMOND: char = '⬦';
    /// \u{2b27}: '⬧'
    pub const BLACK_MEDIUM_LOZENGE: char = '⬧';
    /// \u{2b28}: '⬨'
    pub const WHITE_MEDIUM_LOZENGE: char = '⬨';
    /// \u{2b29}: '⬩'
    pub const BLACK_SMALL_DIAMOND: char = '⬩';
    /// \u{2b2a}: '⬪'
    pub const BLACK_SMALL_LOZENGE: char = '⬪';
    /// \u{2b2b}: '⬫'
    pub const WHITE_SMALL_LOZENGE: char = '⬫';
    /// \u{2b2c}: '⬬'
    pub const BLACK_HORIZONTAL_ELLIPSE: char = '⬬';
    /// \u{2b2d}: '⬭'
    pub const WHITE_HORIZONTAL_ELLIPSE: char = '⬭';
    /// \u{2b2e}: '⬮'
    pub const BLACK_VERTICAL_ELLIPSE: char = '⬮';
    /// \u{2b2f}: '⬯'
    pub const WHITE_VERTICAL_ELLIPSE: char = '⬯';
    /// \u{2b30}: '⬰'
    pub const LEFT_ARROW_WITH_SMALL_CIRCLE: char = '⬰';
    /// \u{2b31}: '⬱'
    pub const THREE_LEFTWARDS_ARROWS: char = '⬱';
    /// \u{2b32}: '⬲'
    pub const LEFT_ARROW_WITH_CIRCLED_PLUS: char = '⬲';
    /// \u{2b33}: '⬳'
    pub const LONG_LEFTWARDS_SQUIGGLE_ARROW: char = '⬳';
    /// \u{2b34}: '⬴'
    pub const LEFTWARDS_TWO_DASH_HEADED_ARROW_WITH_VERTICAL_STROKE: char = '⬴';
    /// \u{2b35}: '⬵'
    pub const LEFTWARDS_TWO_DASH_HEADED_ARROW_WITH_DOUBLE_VERTICAL_STROKE: char = '⬵';
    /// \u{2b36}: '⬶'
    pub const LEFTWARDS_TWO_DASH_HEADED_ARROW_FROM_BAR: char = '⬶';
    /// \u{2b37}: '⬷'
    pub const LEFTWARDS_TWO_DASH_HEADED_TRIPLE_DASH_ARROW: char = '⬷';
    /// \u{2b38}: '⬸'
    pub const LEFTWARDS_ARROW_WITH_DOTTED_STEM: char = '⬸';
    /// \u{2b39}: '⬹'
    pub const LEFTWARDS_ARROW_WITH_TAIL_WITH_VERTICAL_STROKE: char = '⬹';
    /// \u{2b3a}: '⬺'
    pub const LEFTWARDS_ARROW_WITH_TAIL_WITH_DOUBLE_VERTICAL_STROKE: char = '⬺';
    /// \u{2b3b}: '⬻'
    pub const LEFTWARDS_TWO_DASH_HEADED_ARROW_WITH_TAIL: char = '⬻';
    /// \u{2b3c}: '⬼'
    pub const LEFTWARDS_TWO_DASH_HEADED_ARROW_WITH_TAIL_WITH_VERTICAL_STROKE: char = '⬼';
    /// \u{2b3d}: '⬽'
    pub const LEFTWARDS_TWO_DASH_HEADED_ARROW_WITH_TAIL_WITH_DOUBLE_VERTICAL_STROKE: char = '⬽';
    /// \u{2b3e}: '⬾'
    pub const LEFTWARDS_ARROW_THROUGH_X: char = '⬾';
    /// \u{2b3f}: '⬿'
    pub const WAVE_ARROW_POINTING_DIRECTLY_LEFT: char = '⬿';
    /// \u{2b40}: '⭀'
    pub const EQUALS_SIGN_ABOVE_LEFTWARDS_ARROW: char = '⭀';
    /// \u{2b41}: '⭁'
    pub const REVERSE_TILDE_OPERATOR_ABOVE_LEFTWARDS_ARROW: char = '⭁';
    /// \u{2b42}: '⭂'
    pub const LEFTWARDS_ARROW_ABOVE_REVERSE_ALMOST_EQUAL_TO: char = '⭂';
    /// \u{2b43}: '⭃'
    pub const RIGHTWARDS_ARROW_THROUGH_GREATER_DASH_THAN: char = '⭃';
    /// \u{2b44}: '⭄'
    pub const RIGHTWARDS_ARROW_THROUGH_SUPERSET: char = '⭄';
    /// \u{2b45}: '⭅'
    pub const LEFTWARDS_QUADRUPLE_ARROW: char = '⭅';
    /// \u{2b46}: '⭆'
    pub const RIGHTWARDS_QUADRUPLE_ARROW: char = '⭆';
    /// \u{2b47}: '⭇'
    pub const REVERSE_TILDE_OPERATOR_ABOVE_RIGHTWARDS_ARROW: char = '⭇';
    /// \u{2b48}: '⭈'
    pub const RIGHTWARDS_ARROW_ABOVE_REVERSE_ALMOST_EQUAL_TO: char = '⭈';
    /// \u{2b49}: '⭉'
    pub const TILDE_OPERATOR_ABOVE_LEFTWARDS_ARROW: char = '⭉';
    /// \u{2b4a}: '⭊'
    pub const LEFTWARDS_ARROW_ABOVE_ALMOST_EQUAL_TO: char = '⭊';
    /// \u{2b4b}: '⭋'
    pub const LEFTWARDS_ARROW_ABOVE_REVERSE_TILDE_OPERATOR: char = '⭋';
    /// \u{2b4c}: '⭌'
    pub const RIGHTWARDS_ARROW_ABOVE_REVERSE_TILDE_OPERATOR: char = '⭌';
    /// \u{2b4d}: '⭍'
    pub const DOWNWARDS_TRIANGLE_DASH_HEADED_ZIGZAG_ARROW: char = '⭍';
    /// \u{2b4e}: '⭎'
    pub const SHORT_SLANTED_NORTH_ARROW: char = '⭎';
    /// \u{2b4f}: '⭏'
    pub const SHORT_BACKSLANTED_SOUTH_ARROW: char = '⭏';
    /// \u{2b50}: '⭐'
    pub const WHITE_MEDIUM_STAR: char = '⭐';
    /// \u{2b51}: '⭑'
    pub const BLACK_SMALL_STAR: char = '⭑';
    /// \u{2b52}: '⭒'
    pub const WHITE_SMALL_STAR: char = '⭒';
    /// \u{2b53}: '⭓'
    pub const BLACK_RIGHT_DASH_POINTING_PENTAGON: char = '⭓';
    /// \u{2b54}: '⭔'
    pub const WHITE_RIGHT_DASH_POINTING_PENTAGON: char = '⭔';
    /// \u{2b55}: '⭕'
    pub const HEAVY_LARGE_CIRCLE: char = '⭕';
    /// \u{2b56}: '⭖'
    pub const HEAVY_OVAL_WITH_OVAL_INSIDE: char = '⭖';
    /// \u{2b57}: '⭗'
    pub const HEAVY_CIRCLE_WITH_CIRCLE_INSIDE: char = '⭗';
    /// \u{2b58}: '⭘'
    pub const HEAVY_CIRCLE: char = '⭘';
    /// \u{2b59}: '⭙'
    pub const HEAVY_CIRCLED_SALTIRE: char = '⭙';
    /// \u{2b5a}: '⭚'
    pub const SLANTED_NORTH_ARROW_WITH_HOOKED_HEAD: char = '⭚';
    /// \u{2b5b}: '⭛'
    pub const BACKSLANTED_SOUTH_ARROW_WITH_HOOKED_TAIL: char = '⭛';
    /// \u{2b5c}: '⭜'
    pub const SLANTED_NORTH_ARROW_WITH_HORIZONTAL_TAIL: char = '⭜';
    /// \u{2b5d}: '⭝'
    pub const BACKSLANTED_SOUTH_ARROW_WITH_HORIZONTAL_TAIL: char = '⭝';
    /// \u{2b5e}: '⭞'
    pub const BENT_ARROW_POINTING_DOWNWARDS_THEN_NORTH_EAST: char = '⭞';
    /// \u{2b5f}: '⭟'
    pub const SHORT_BENT_ARROW_POINTING_DOWNWARDS_THEN_NORTH_EAST: char = '⭟';
    /// \u{2b60}: '⭠'
    pub const LEFTWARDS_TRIANGLE_DASH_HEADED_ARROW: char = '⭠';
    /// \u{2b61}: '⭡'
    pub const UPWARDS_TRIANGLE_DASH_HEADED_ARROW: char = '⭡';
    /// \u{2b62}: '⭢'
    pub const RIGHTWARDS_TRIANGLE_DASH_HEADED_ARROW: char = '⭢';
    /// \u{2b63}: '⭣'
    pub const DOWNWARDS_TRIANGLE_DASH_HEADED_ARROW: char = '⭣';
    /// \u{2b64}: '⭤'
    pub const LEFT_RIGHT_TRIANGLE_DASH_HEADED_ARROW: char = '⭤';
    /// \u{2b65}: '⭥'
    pub const UP_DOWN_TRIANGLE_DASH_HEADED_ARROW: char = '⭥';
    /// \u{2b66}: '⭦'
    pub const NORTH_WEST_TRIANGLE_DASH_HEADED_ARROW: char = '⭦';
    /// \u{2b67}: '⭧'
    pub const NORTH_EAST_TRIANGLE_DASH_HEADED_ARROW: char = '⭧';
    /// \u{2b68}: '⭨'
    pub const SOUTH_EAST_TRIANGLE_DASH_HEADED_ARROW: char = '⭨';
    /// \u{2b69}: '⭩'
    pub const SOUTH_WEST_TRIANGLE_DASH_HEADED_ARROW: char = '⭩';
    /// \u{2b6a}: '⭪'
    pub const LEFTWARDS_TRIANGLE_DASH_HEADED_DASHED_ARROW: char = '⭪';
    /// \u{2b6b}: '⭫'
    pub const UPWARDS_TRIANGLE_DASH_HEADED_DASHED_ARROW: char = '⭫';
    /// \u{2b6c}: '⭬'
    pub const RIGHTWARDS_TRIANGLE_DASH_HEADED_DASHED_ARROW: char = '⭬';
    /// \u{2b6d}: '⭭'
    pub const DOWNWARDS_TRIANGLE_DASH_HEADED_DASHED_ARROW: char = '⭭';
    /// \u{2b6e}: '⭮'
    pub const CLOCKWISE_TRIANGLE_DASH_HEADED_OPEN_CIRCLE_ARROW: char = '⭮';
    /// \u{2b6f}: '⭯'
    pub const ANTICLOCKWISE_TRIANGLE_DASH_HEADED_OPEN_CIRCLE_ARROW: char = '⭯';
    /// \u{2b70}: '⭰'
    pub const LEFTWARDS_TRIANGLE_DASH_HEADED_ARROW_TO_BAR: char = '⭰';
    /// \u{2b71}: '⭱'
    pub const UPWARDS_TRIANGLE_DASH_HEADED_ARROW_TO_BAR: char = '⭱';
    /// \u{2b72}: '⭲'
    pub const RIGHTWARDS_TRIANGLE_DASH_HEADED_ARROW_TO_BAR: char = '⭲';
    /// \u{2b73}: '⭳'
    pub const DOWNWARDS_TRIANGLE_DASH_HEADED_ARROW_TO_BAR: char = '⭳';
    /// \u{2b76}: '⭶'
    pub const NORTH_WEST_TRIANGLE_DASH_HEADED_ARROW_TO_BAR: char = '⭶';
    /// \u{2b77}: '⭷'
    pub const NORTH_EAST_TRIANGLE_DASH_HEADED_ARROW_TO_BAR: char = '⭷';
    /// \u{2b78}: '⭸'
    pub const SOUTH_EAST_TRIANGLE_DASH_HEADED_ARROW_TO_BAR: char = '⭸';
    /// \u{2b79}: '⭹'
    pub const SOUTH_WEST_TRIANGLE_DASH_HEADED_ARROW_TO_BAR: char = '⭹';
    /// \u{2b7a}: '⭺'
    pub const LEFTWARDS_TRIANGLE_DASH_HEADED_ARROW_WITH_DOUBLE_HORIZONTAL_STROKE: char = '⭺';
    /// \u{2b7b}: '⭻'
    pub const UPWARDS_TRIANGLE_DASH_HEADED_ARROW_WITH_DOUBLE_HORIZONTAL_STROKE: char = '⭻';
    /// \u{2b7c}: '⭼'
    pub const RIGHTWARDS_TRIANGLE_DASH_HEADED_ARROW_WITH_DOUBLE_HORIZONTAL_STROKE: char = '⭼';
    /// \u{2b7d}: '⭽'
    pub const DOWNWARDS_TRIANGLE_DASH_HEADED_ARROW_WITH_DOUBLE_HORIZONTAL_STROKE: char = '⭽';
    /// \u{2b7e}: '⭾'
    pub const HORIZONTAL_TAB_KEY: char = '⭾';
    /// \u{2b7f}: '⭿'
    pub const VERTICAL_TAB_KEY: char = '⭿';
    /// \u{2b80}: '⮀'
    pub const LEFTWARDS_TRIANGLE_DASH_HEADED_ARROW_OVER_RIGHTWARDS_TRIANGLE_DASH_HEADED_ARROW: char = '⮀';
    /// \u{2b81}: '⮁'
    pub const UPWARDS_TRIANGLE_DASH_HEADED_ARROW_LEFTWARDS_OF_DOWNWARDS_TRIANGLE_DASH_HEADED_ARROW: char = '⮁';
    /// \u{2b82}: '⮂'
    pub const RIGHTWARDS_TRIANGLE_DASH_HEADED_ARROW_OVER_LEFTWARDS_TRIANGLE_DASH_HEADED_ARROW: char = '⮂';
    /// \u{2b83}: '⮃'
    pub const DOWNWARDS_TRIANGLE_DASH_HEADED_ARROW_LEFTWARDS_OF_UPWARDS_TRIANGLE_DASH_HEADED_ARROW: char = '⮃';
    /// \u{2b84}: '⮄'
    pub const LEFTWARDS_TRIANGLE_DASH_HEADED_PAIRED_ARROWS: char = '⮄';
    /// \u{2b85}: '⮅'
    pub const UPWARDS_TRIANGLE_DASH_HEADED_PAIRED_ARROWS: char = '⮅';
    /// \u{2b86}: '⮆'
    pub const RIGHTWARDS_TRIANGLE_DASH_HEADED_PAIRED_ARROWS: char = '⮆';
    /// \u{2b87}: '⮇'
    pub const DOWNWARDS_TRIANGLE_DASH_HEADED_PAIRED_ARROWS: char = '⮇';
    /// \u{2b88}: '⮈'
    pub const LEFTWARDS_BLACK_CIRCLED_WHITE_ARROW: char = '⮈';
    /// \u{2b89}: '⮉'
    pub const UPWARDS_BLACK_CIRCLED_WHITE_ARROW: char = '⮉';
    /// \u{2b8a}: '⮊'
    pub const RIGHTWARDS_BLACK_CIRCLED_WHITE_ARROW: char = '⮊';
    /// \u{2b8b}: '⮋'
    pub const DOWNWARDS_BLACK_CIRCLED_WHITE_ARROW: char = '⮋';
    /// \u{2b8c}: '⮌'
    pub const ANTICLOCKWISE_TRIANGLE_DASH_HEADED_RIGHT_U_DASH_SHAPED_ARROW: char = '⮌';
    /// \u{2b8d}: '⮍'
    pub const ANTICLOCKWISE_TRIANGLE_DASH_HEADED_BOTTOM_U_DASH_SHAPED_ARROW: char = '⮍';
    /// \u{2b8e}: '⮎'
    pub const ANTICLOCKWISE_TRIANGLE_DASH_HEADED_LEFT_U_DASH_SHAPED_ARROW: char = '⮎';
    /// \u{2b8f}: '⮏'
    pub const ANTICLOCKWISE_TRIANGLE_DASH_HEADED_TOP_U_DASH_SHAPED_ARROW: char = '⮏';
    /// \u{2b90}: '⮐'
    pub const RETURN_LEFT: char = '⮐';
    /// \u{2b91}: '⮑'
    pub const RETURN_RIGHT: char = '⮑';
    /// \u{2b92}: '⮒'
    pub const NEWLINE_LEFT: char = '⮒';
    /// \u{2b93}: '⮓'
    pub const NEWLINE_RIGHT: char = '⮓';
    /// \u{2b94}: '⮔'
    pub const FOUR_CORNER_ARROWS_CIRCLING_ANTICLOCKWISE: char = '⮔';
    /// \u{2b95}: '⮕'
    pub const RIGHTWARDS_BLACK_ARROW: char = '⮕';
    /// \u{2b98}: '⮘'
    pub const THREE_DASH_D_TOP_DASH_LIGHTED_LEFTWARDS_EQUILATERAL_ARROWHEAD: char = '⮘';
    /// \u{2b99}: '⮙'
    pub const THREE_DASH_D_RIGHT_DASH_LIGHTED_UPWARDS_EQUILATERAL_ARROWHEAD: char = '⮙';
    /// \u{2b9a}: '⮚'
    pub const THREE_DASH_D_TOP_DASH_LIGHTED_RIGHTWARDS_EQUILATERAL_ARROWHEAD: char = '⮚';
    /// \u{2b9b}: '⮛'
    pub const THREE_DASH_D_LEFT_DASH_LIGHTED_DOWNWARDS_EQUILATERAL_ARROWHEAD: char = '⮛';
    /// \u{2b9c}: '⮜'
    pub const BLACK_LEFTWARDS_EQUILATERAL_ARROWHEAD: char = '⮜';
    /// \u{2b9d}: '⮝'
    pub const BLACK_UPWARDS_EQUILATERAL_ARROWHEAD: char = '⮝';
    /// \u{2b9e}: '⮞'
    pub const BLACK_RIGHTWARDS_EQUILATERAL_ARROWHEAD: char = '⮞';
    /// \u{2b9f}: '⮟'
    pub const BLACK_DOWNWARDS_EQUILATERAL_ARROWHEAD: char = '⮟';
    /// \u{2ba0}: '⮠'
    pub const DOWNWARDS_TRIANGLE_DASH_HEADED_ARROW_WITH_LONG_TIP_LEFTWARDS: char = '⮠';
    /// \u{2ba1}: '⮡'
    pub const DOWNWARDS_TRIANGLE_DASH_HEADED_ARROW_WITH_LONG_TIP_RIGHTWARDS: char = '⮡';
    /// \u{2ba2}: '⮢'
    pub const UPWARDS_TRIANGLE_DASH_HEADED_ARROW_WITH_LONG_TIP_LEFTWARDS: char = '⮢';
    /// \u{2ba3}: '⮣'
    pub const UPWARDS_TRIANGLE_DASH_HEADED_ARROW_WITH_LONG_TIP_RIGHTWARDS: char = '⮣';
    /// \u{2ba4}: '⮤'
    pub const LEFTWARDS_TRIANGLE_DASH_HEADED_ARROW_WITH_LONG_TIP_UPWARDS: char = '⮤';
    /// \u{2ba5}: '⮥'
    pub const RIGHTWARDS_TRIANGLE_DASH_HEADED_ARROW_WITH_LONG_TIP_UPWARDS: char = '⮥';
    /// \u{2ba6}: '⮦'
    pub const LEFTWARDS_TRIANGLE_DASH_HEADED_ARROW_WITH_LONG_TIP_DOWNWARDS: char = '⮦';
    /// \u{2ba7}: '⮧'
    pub const RIGHTWARDS_TRIANGLE_DASH_HEADED_ARROW_WITH_LONG_TIP_DOWNWARDS: char = '⮧';
    /// \u{2ba8}: '⮨'
    pub const BLACK_CURVED_DOWNWARDS_AND_LEFTWARDS_ARROW: char = '⮨';
    /// \u{2ba9}: '⮩'
    pub const BLACK_CURVED_DOWNWARDS_AND_RIGHTWARDS_ARROW: char = '⮩';
    /// \u{2baa}: '⮪'
    pub const BLACK_CURVED_UPWARDS_AND_LEFTWARDS_ARROW: char = '⮪';
    /// \u{2bab}: '⮫'
    pub const BLACK_CURVED_UPWARDS_AND_RIGHTWARDS_ARROW: char = '⮫';
    /// \u{2bac}: '⮬'
    pub const BLACK_CURVED_LEFTWARDS_AND_UPWARDS_ARROW: char = '⮬';
    /// \u{2bad}: '⮭'
    pub const BLACK_CURVED_RIGHTWARDS_AND_UPWARDS_ARROW: char = '⮭';
    /// \u{2bae}: '⮮'
    pub const BLACK_CURVED_LEFTWARDS_AND_DOWNWARDS_ARROW: char = '⮮';
    /// \u{2baf}: '⮯'
    pub const BLACK_CURVED_RIGHTWARDS_AND_DOWNWARDS_ARROW: char = '⮯';
    /// \u{2bb0}: '⮰'
    pub const RIBBON_ARROW_DOWN_LEFT: char = '⮰';
    /// \u{2bb1}: '⮱'
    pub const RIBBON_ARROW_DOWN_RIGHT: char = '⮱';
    /// \u{2bb2}: '⮲'
    pub const RIBBON_ARROW_UP_LEFT: char = '⮲';
    /// \u{2bb3}: '⮳'
    pub const RIBBON_ARROW_UP_RIGHT: char = '⮳';
    /// \u{2bb4}: '⮴'
    pub const RIBBON_ARROW_LEFT_UP: char = '⮴';
    /// \u{2bb5}: '⮵'
    pub const RIBBON_ARROW_RIGHT_UP: char = '⮵';
    /// \u{2bb6}: '⮶'
    pub const RIBBON_ARROW_LEFT_DOWN: char = '⮶';
    /// \u{2bb7}: '⮷'
    pub const RIBBON_ARROW_RIGHT_DOWN: char = '⮷';
    /// \u{2bb8}: '⮸'
    pub const UPWARDS_WHITE_ARROW_FROM_BAR_WITH_HORIZONTAL_BAR: char = '⮸';
    /// \u{2bb9}: '⮹'
    pub const UP_ARROWHEAD_IN_A_RECTANGLE_BOX: char = '⮹';
    /// \u{2bba}: '⮺'
    pub const OVERLAPPING_WHITE_SQUARES: char = '⮺';
    /// \u{2bbb}: '⮻'
    pub const OVERLAPPING_WHITE_AND_BLACK_SQUARES: char = '⮻';
    /// \u{2bbc}: '⮼'
    pub const OVERLAPPING_BLACK_SQUARES: char = '⮼';
    /// \u{2bbd}: '⮽'
    pub const BALLOT_BOX_WITH_LIGHT_X: char = '⮽';
    /// \u{2bbe}: '⮾'
    pub const CIRCLED_X: char = '⮾';
    /// \u{2bbf}: '⮿'
    pub const CIRCLED_BOLD_X: char = '⮿';
    /// \u{2bc0}: '⯀'
    pub const BLACK_SQUARE_CENTRED: char = '⯀';
    /// \u{2bc1}: '⯁'
    pub const BLACK_DIAMOND_CENTRED: char = '⯁';
    /// \u{2bc2}: '⯂'
    pub const TURNED_BLACK_PENTAGON: char = '⯂';
    /// \u{2bc3}: '⯃'
    pub const HORIZONTAL_BLACK_OCTAGON: char = '⯃';
    /// \u{2bc4}: '⯄'
    pub const BLACK_OCTAGON: char = '⯄';
    /// \u{2bc5}: '⯅'
    pub const BLACK_MEDIUM_UP_DASH_POINTING_TRIANGLE_CENTRED: char = '⯅';
    /// \u{2bc6}: '⯆'
    pub const BLACK_MEDIUM_DOWN_DASH_POINTING_TRIANGLE_CENTRED: char = '⯆';
    /// \u{2bc7}: '⯇'
    pub const BLACK_MEDIUM_LEFT_DASH_POINTING_TRIANGLE_CENTRED: char = '⯇';
    /// \u{2bc8}: '⯈'
    pub const BLACK_MEDIUM_RIGHT_DASH_POINTING_TRIANGLE_CENTRED: char = '⯈';
    /// \u{2bc9}: '⯉'
    pub const NEPTUNE_FORM_TWO: char = '⯉';
    /// \u{2bca}: '⯊'
    pub const TOP_HALF_BLACK_CIRCLE: char = '⯊';
    /// \u{2bcb}: '⯋'
    pub const BOTTOM_HALF_BLACK_CIRCLE: char = '⯋';
    /// \u{2bcc}: '⯌'
    pub const LIGHT_FOUR_POINTED_BLACK_CUSP: char = '⯌';
    /// \u{2bcd}: '⯍'
    pub const ROTATED_LIGHT_FOUR_POINTED_BLACK_CUSP: char = '⯍';
    /// \u{2bce}: '⯎'
    pub const WHITE_FOUR_POINTED_CUSP: char = '⯎';
    /// \u{2bcf}: '⯏'
    pub const ROTATED_WHITE_FOUR_POINTED_CUSP: char = '⯏';
    /// \u{2bd0}: '⯐'
    pub const SQUARE_POSITION_INDICATOR: char = '⯐';
    /// \u{2bd1}: '⯑'
    pub const UNCERTAINTY_SIGN: char = '⯑';
    /// \u{2bd2}: '⯒'
    pub const GROUP_MARK: char = '⯒';
    /// \u{2bd3}: '⯓'
    pub const PLUTO_FORM_TWO: char = '⯓';
    /// \u{2bd4}: '⯔'
    pub const PLUTO_FORM_THREE: char = '⯔';
    /// \u{2bd5}: '⯕'
    pub const PLUTO_FORM_FOUR: char = '⯕';
    /// \u{2bd6}: '⯖'
    pub const PLUTO_FORM_FIVE: char = '⯖';
    /// \u{2bd7}: '⯗'
    pub const TRANSPLUTO: char = '⯗';
    /// \u{2bd8}: '⯘'
    pub const PROSERPINA: char = '⯘';
    /// \u{2bd9}: '⯙'
    pub const ASTRAEA: char = '⯙';
    /// \u{2bda}: '⯚'
    pub const HYGIEA: char = '⯚';
    /// \u{2bdb}: '⯛'
    pub const PHOLUS: char = '⯛';
    /// \u{2bdc}: '⯜'
    pub const NESSUS: char = '⯜';
    /// \u{2bdd}: '⯝'
    pub const WHITE_MOON_SELENA: char = '⯝';
    /// \u{2bde}: '⯞'
    pub const BLACK_DIAMOND_ON_CROSS: char = '⯞';
    /// \u{2bdf}: '⯟'
    pub const TRUE_LIGHT_MOON_ARTA: char = '⯟';
    /// \u{2be0}: '⯠'
    pub const CUPIDO: char = '⯠';
    /// \u{2be1}: '⯡'
    pub const HADES: char = '⯡';
    /// \u{2be2}: '⯢'
    pub const ZEUS: char = '⯢';
    /// \u{2be3}: '⯣'
    pub const KRONOS: char = '⯣';
    /// \u{2be4}: '⯤'
    pub const APOLLON: char = '⯤';
    /// \u{2be5}: '⯥'
    pub const ADMETOS: char = '⯥';
    /// \u{2be6}: '⯦'
    pub const VULCANUS: char = '⯦';
    /// \u{2be7}: '⯧'
    pub const POSEIDON: char = '⯧';
    /// \u{2be8}: '⯨'
    pub const LEFT_HALF_BLACK_STAR: char = '⯨';
    /// \u{2be9}: '⯩'
    pub const RIGHT_HALF_BLACK_STAR: char = '⯩';
    /// \u{2bea}: '⯪'
    pub const STAR_WITH_LEFT_HALF_BLACK: char = '⯪';
    /// \u{2beb}: '⯫'
    pub const STAR_WITH_RIGHT_HALF_BLACK: char = '⯫';
    /// \u{2bec}: '⯬'
    pub const LEFTWARDS_TWO_DASH_HEADED_ARROW_WITH_TRIANGLE_ARROWHEADS: char = '⯬';
    /// \u{2bed}: '⯭'
    pub const UPWARDS_TWO_DASH_HEADED_ARROW_WITH_TRIANGLE_ARROWHEADS: char = '⯭';
    /// \u{2bee}: '⯮'
    pub const RIGHTWARDS_TWO_DASH_HEADED_ARROW_WITH_TRIANGLE_ARROWHEADS: char = '⯮';
    /// \u{2bef}: '⯯'
    pub const DOWNWARDS_TWO_DASH_HEADED_ARROW_WITH_TRIANGLE_ARROWHEADS: char = '⯯';
    /// \u{2bf0}: '⯰'
    pub const ERIS_FORM_ONE: char = '⯰';
    /// \u{2bf1}: '⯱'
    pub const ERIS_FORM_TWO: char = '⯱';
    /// \u{2bf2}: '⯲'
    pub const SEDNA: char = '⯲';
    /// \u{2bf3}: '⯳'
    pub const RUSSIAN_ASTROLOGICAL_SYMBOL_VIGINTILE: char = '⯳';
    /// \u{2bf4}: '⯴'
    pub const RUSSIAN_ASTROLOGICAL_SYMBOL_NOVILE: char = '⯴';
    /// \u{2bf5}: '⯵'
    pub const RUSSIAN_ASTROLOGICAL_SYMBOL_QUINTILE: char = '⯵';
    /// \u{2bf6}: '⯶'
    pub const RUSSIAN_ASTROLOGICAL_SYMBOL_BINOVILE: char = '⯶';
    /// \u{2bf7}: '⯷'
    pub const RUSSIAN_ASTROLOGICAL_SYMBOL_SENTAGON: char = '⯷';
    /// \u{2bf8}: '⯸'
    pub const RUSSIAN_ASTROLOGICAL_SYMBOL_TREDECILE: char = '⯸';
    /// \u{2bf9}: '⯹'
    pub const EQUALS_SIGN_WITH_INFINITY_BELOW: char = '⯹';
    /// \u{2bfa}: '⯺'
    pub const UNITED_SYMBOL: char = '⯺';
    /// \u{2bfb}: '⯻'
    pub const SEPARATED_SYMBOL: char = '⯻';
    /// \u{2bfc}: '⯼'
    pub const DOUBLED_SYMBOL: char = '⯼';
    /// \u{2bfd}: '⯽'
    pub const PASSED_SYMBOL: char = '⯽';
    /// \u{2bfe}: '⯾'
    pub const REVERSED_RIGHT_ANGLE: char = '⯾';
}

/// An enum to represent all characters in the MiscellaneousSymbolsandArrows block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum MiscellaneousSymbolsandArrows {
    /// \u{2b00}: '⬀'
    NorthEastWhiteArrow,
    /// \u{2b01}: '⬁'
    NorthWestWhiteArrow,
    /// \u{2b02}: '⬂'
    SouthEastWhiteArrow,
    /// \u{2b03}: '⬃'
    SouthWestWhiteArrow,
    /// \u{2b04}: '⬄'
    LeftRightWhiteArrow,
    /// \u{2b05}: '⬅'
    LeftwardsBlackArrow,
    /// \u{2b06}: '⬆'
    UpwardsBlackArrow,
    /// \u{2b07}: '⬇'
    DownwardsBlackArrow,
    /// \u{2b08}: '⬈'
    NorthEastBlackArrow,
    /// \u{2b09}: '⬉'
    NorthWestBlackArrow,
    /// \u{2b0a}: '⬊'
    SouthEastBlackArrow,
    /// \u{2b0b}: '⬋'
    SouthWestBlackArrow,
    /// \u{2b0c}: '⬌'
    LeftRightBlackArrow,
    /// \u{2b0d}: '⬍'
    UpDownBlackArrow,
    /// \u{2b0e}: '⬎'
    RightwardsArrowWithTipDownwards,
    /// \u{2b0f}: '⬏'
    RightwardsArrowWithTipUpwards,
    /// \u{2b10}: '⬐'
    LeftwardsArrowWithTipDownwards,
    /// \u{2b11}: '⬑'
    LeftwardsArrowWithTipUpwards,
    /// \u{2b12}: '⬒'
    SquareWithTopHalfBlack,
    /// \u{2b13}: '⬓'
    SquareWithBottomHalfBlack,
    /// \u{2b14}: '⬔'
    SquareWithUpperRightDiagonalHalfBlack,
    /// \u{2b15}: '⬕'
    SquareWithLowerLeftDiagonalHalfBlack,
    /// \u{2b16}: '⬖'
    DiamondWithLeftHalfBlack,
    /// \u{2b17}: '⬗'
    DiamondWithRightHalfBlack,
    /// \u{2b18}: '⬘'
    DiamondWithTopHalfBlack,
    /// \u{2b19}: '⬙'
    DiamondWithBottomHalfBlack,
    /// \u{2b1a}: '⬚'
    DottedSquare,
    /// \u{2b1b}: '⬛'
    BlackLargeSquare,
    /// \u{2b1c}: '⬜'
    WhiteLargeSquare,
    /// \u{2b1d}: '⬝'
    BlackVerySmallSquare,
    /// \u{2b1e}: '⬞'
    WhiteVerySmallSquare,
    /// \u{2b1f}: '⬟'
    BlackPentagon,
    /// \u{2b20}: '⬠'
    WhitePentagon,
    /// \u{2b21}: '⬡'
    WhiteHexagon,
    /// \u{2b22}: '⬢'
    BlackHexagon,
    /// \u{2b23}: '⬣'
    HorizontalBlackHexagon,
    /// \u{2b24}: '⬤'
    BlackLargeCircle,
    /// \u{2b25}: '⬥'
    BlackMediumDiamond,
    /// \u{2b26}: '⬦'
    WhiteMediumDiamond,
    /// \u{2b27}: '⬧'
    BlackMediumLozenge,
    /// \u{2b28}: '⬨'
    WhiteMediumLozenge,
    /// \u{2b29}: '⬩'
    BlackSmallDiamond,
    /// \u{2b2a}: '⬪'
    BlackSmallLozenge,
    /// \u{2b2b}: '⬫'
    WhiteSmallLozenge,
    /// \u{2b2c}: '⬬'
    BlackHorizontalEllipse,
    /// \u{2b2d}: '⬭'
    WhiteHorizontalEllipse,
    /// \u{2b2e}: '⬮'
    BlackVerticalEllipse,
    /// \u{2b2f}: '⬯'
    WhiteVerticalEllipse,
    /// \u{2b30}: '⬰'
    LeftArrowWithSmallCircle,
    /// \u{2b31}: '⬱'
    ThreeLeftwardsArrows,
    /// \u{2b32}: '⬲'
    LeftArrowWithCircledPlus,
    /// \u{2b33}: '⬳'
    LongLeftwardsSquiggleArrow,
    /// \u{2b34}: '⬴'
    LeftwardsTwoDashHeadedArrowWithVerticalStroke,
    /// \u{2b35}: '⬵'
    LeftwardsTwoDashHeadedArrowWithDoubleVerticalStroke,
    /// \u{2b36}: '⬶'
    LeftwardsTwoDashHeadedArrowFromBar,
    /// \u{2b37}: '⬷'
    LeftwardsTwoDashHeadedTripleDashArrow,
    /// \u{2b38}: '⬸'
    LeftwardsArrowWithDottedStem,
    /// \u{2b39}: '⬹'
    LeftwardsArrowWithTailWithVerticalStroke,
    /// \u{2b3a}: '⬺'
    LeftwardsArrowWithTailWithDoubleVerticalStroke,
    /// \u{2b3b}: '⬻'
    LeftwardsTwoDashHeadedArrowWithTail,
    /// \u{2b3c}: '⬼'
    LeftwardsTwoDashHeadedArrowWithTailWithVerticalStroke,
    /// \u{2b3d}: '⬽'
    LeftwardsTwoDashHeadedArrowWithTailWithDoubleVerticalStroke,
    /// \u{2b3e}: '⬾'
    LeftwardsArrowThroughX,
    /// \u{2b3f}: '⬿'
    WaveArrowPointingDirectlyLeft,
    /// \u{2b40}: '⭀'
    EqualsSignAboveLeftwardsArrow,
    /// \u{2b41}: '⭁'
    ReverseTildeOperatorAboveLeftwardsArrow,
    /// \u{2b42}: '⭂'
    LeftwardsArrowAboveReverseAlmostEqualTo,
    /// \u{2b43}: '⭃'
    RightwardsArrowThroughGreaterDashThan,
    /// \u{2b44}: '⭄'
    RightwardsArrowThroughSuperset,
    /// \u{2b45}: '⭅'
    LeftwardsQuadrupleArrow,
    /// \u{2b46}: '⭆'
    RightwardsQuadrupleArrow,
    /// \u{2b47}: '⭇'
    ReverseTildeOperatorAboveRightwardsArrow,
    /// \u{2b48}: '⭈'
    RightwardsArrowAboveReverseAlmostEqualTo,
    /// \u{2b49}: '⭉'
    TildeOperatorAboveLeftwardsArrow,
    /// \u{2b4a}: '⭊'
    LeftwardsArrowAboveAlmostEqualTo,
    /// \u{2b4b}: '⭋'
    LeftwardsArrowAboveReverseTildeOperator,
    /// \u{2b4c}: '⭌'
    RightwardsArrowAboveReverseTildeOperator,
    /// \u{2b4d}: '⭍'
    DownwardsTriangleDashHeadedZigzagArrow,
    /// \u{2b4e}: '⭎'
    ShortSlantedNorthArrow,
    /// \u{2b4f}: '⭏'
    ShortBackslantedSouthArrow,
    /// \u{2b50}: '⭐'
    WhiteMediumStar,
    /// \u{2b51}: '⭑'
    BlackSmallStar,
    /// \u{2b52}: '⭒'
    WhiteSmallStar,
    /// \u{2b53}: '⭓'
    BlackRightDashPointingPentagon,
    /// \u{2b54}: '⭔'
    WhiteRightDashPointingPentagon,
    /// \u{2b55}: '⭕'
    HeavyLargeCircle,
    /// \u{2b56}: '⭖'
    HeavyOvalWithOvalInside,
    /// \u{2b57}: '⭗'
    HeavyCircleWithCircleInside,
    /// \u{2b58}: '⭘'
    HeavyCircle,
    /// \u{2b59}: '⭙'
    HeavyCircledSaltire,
    /// \u{2b5a}: '⭚'
    SlantedNorthArrowWithHookedHead,
    /// \u{2b5b}: '⭛'
    BackslantedSouthArrowWithHookedTail,
    /// \u{2b5c}: '⭜'
    SlantedNorthArrowWithHorizontalTail,
    /// \u{2b5d}: '⭝'
    BackslantedSouthArrowWithHorizontalTail,
    /// \u{2b5e}: '⭞'
    BentArrowPointingDownwardsThenNorthEast,
    /// \u{2b5f}: '⭟'
    ShortBentArrowPointingDownwardsThenNorthEast,
    /// \u{2b60}: '⭠'
    LeftwardsTriangleDashHeadedArrow,
    /// \u{2b61}: '⭡'
    UpwardsTriangleDashHeadedArrow,
    /// \u{2b62}: '⭢'
    RightwardsTriangleDashHeadedArrow,
    /// \u{2b63}: '⭣'
    DownwardsTriangleDashHeadedArrow,
    /// \u{2b64}: '⭤'
    LeftRightTriangleDashHeadedArrow,
    /// \u{2b65}: '⭥'
    UpDownTriangleDashHeadedArrow,
    /// \u{2b66}: '⭦'
    NorthWestTriangleDashHeadedArrow,
    /// \u{2b67}: '⭧'
    NorthEastTriangleDashHeadedArrow,
    /// \u{2b68}: '⭨'
    SouthEastTriangleDashHeadedArrow,
    /// \u{2b69}: '⭩'
    SouthWestTriangleDashHeadedArrow,
    /// \u{2b6a}: '⭪'
    LeftwardsTriangleDashHeadedDashedArrow,
    /// \u{2b6b}: '⭫'
    UpwardsTriangleDashHeadedDashedArrow,
    /// \u{2b6c}: '⭬'
    RightwardsTriangleDashHeadedDashedArrow,
    /// \u{2b6d}: '⭭'
    DownwardsTriangleDashHeadedDashedArrow,
    /// \u{2b6e}: '⭮'
    ClockwiseTriangleDashHeadedOpenCircleArrow,
    /// \u{2b6f}: '⭯'
    AnticlockwiseTriangleDashHeadedOpenCircleArrow,
    /// \u{2b70}: '⭰'
    LeftwardsTriangleDashHeadedArrowToBar,
    /// \u{2b71}: '⭱'
    UpwardsTriangleDashHeadedArrowToBar,
    /// \u{2b72}: '⭲'
    RightwardsTriangleDashHeadedArrowToBar,
    /// \u{2b73}: '⭳'
    DownwardsTriangleDashHeadedArrowToBar,
    /// \u{2b76}: '⭶'
    NorthWestTriangleDashHeadedArrowToBar,
    /// \u{2b77}: '⭷'
    NorthEastTriangleDashHeadedArrowToBar,
    /// \u{2b78}: '⭸'
    SouthEastTriangleDashHeadedArrowToBar,
    /// \u{2b79}: '⭹'
    SouthWestTriangleDashHeadedArrowToBar,
    /// \u{2b7a}: '⭺'
    LeftwardsTriangleDashHeadedArrowWithDoubleHorizontalStroke,
    /// \u{2b7b}: '⭻'
    UpwardsTriangleDashHeadedArrowWithDoubleHorizontalStroke,
    /// \u{2b7c}: '⭼'
    RightwardsTriangleDashHeadedArrowWithDoubleHorizontalStroke,
    /// \u{2b7d}: '⭽'
    DownwardsTriangleDashHeadedArrowWithDoubleHorizontalStroke,
    /// \u{2b7e}: '⭾'
    HorizontalTabKey,
    /// \u{2b7f}: '⭿'
    VerticalTabKey,
    /// \u{2b80}: '⮀'
    LeftwardsTriangleDashHeadedArrowOverRightwardsTriangleDashHeadedArrow,
    /// \u{2b81}: '⮁'
    UpwardsTriangleDashHeadedArrowLeftwardsOfDownwardsTriangleDashHeadedArrow,
    /// \u{2b82}: '⮂'
    RightwardsTriangleDashHeadedArrowOverLeftwardsTriangleDashHeadedArrow,
    /// \u{2b83}: '⮃'
    DownwardsTriangleDashHeadedArrowLeftwardsOfUpwardsTriangleDashHeadedArrow,
    /// \u{2b84}: '⮄'
    LeftwardsTriangleDashHeadedPairedArrows,
    /// \u{2b85}: '⮅'
    UpwardsTriangleDashHeadedPairedArrows,
    /// \u{2b86}: '⮆'
    RightwardsTriangleDashHeadedPairedArrows,
    /// \u{2b87}: '⮇'
    DownwardsTriangleDashHeadedPairedArrows,
    /// \u{2b88}: '⮈'
    LeftwardsBlackCircledWhiteArrow,
    /// \u{2b89}: '⮉'
    UpwardsBlackCircledWhiteArrow,
    /// \u{2b8a}: '⮊'
    RightwardsBlackCircledWhiteArrow,
    /// \u{2b8b}: '⮋'
    DownwardsBlackCircledWhiteArrow,
    /// \u{2b8c}: '⮌'
    AnticlockwiseTriangleDashHeadedRightUDashShapedArrow,
    /// \u{2b8d}: '⮍'
    AnticlockwiseTriangleDashHeadedBottomUDashShapedArrow,
    /// \u{2b8e}: '⮎'
    AnticlockwiseTriangleDashHeadedLeftUDashShapedArrow,
    /// \u{2b8f}: '⮏'
    AnticlockwiseTriangleDashHeadedTopUDashShapedArrow,
    /// \u{2b90}: '⮐'
    ReturnLeft,
    /// \u{2b91}: '⮑'
    ReturnRight,
    /// \u{2b92}: '⮒'
    NewlineLeft,
    /// \u{2b93}: '⮓'
    NewlineRight,
    /// \u{2b94}: '⮔'
    FourCornerArrowsCirclingAnticlockwise,
    /// \u{2b95}: '⮕'
    RightwardsBlackArrow,
    /// \u{2b98}: '⮘'
    ThreeDashDTopDashLightedLeftwardsEquilateralArrowhead,
    /// \u{2b99}: '⮙'
    ThreeDashDRightDashLightedUpwardsEquilateralArrowhead,
    /// \u{2b9a}: '⮚'
    ThreeDashDTopDashLightedRightwardsEquilateralArrowhead,
    /// \u{2b9b}: '⮛'
    ThreeDashDLeftDashLightedDownwardsEquilateralArrowhead,
    /// \u{2b9c}: '⮜'
    BlackLeftwardsEquilateralArrowhead,
    /// \u{2b9d}: '⮝'
    BlackUpwardsEquilateralArrowhead,
    /// \u{2b9e}: '⮞'
    BlackRightwardsEquilateralArrowhead,
    /// \u{2b9f}: '⮟'
    BlackDownwardsEquilateralArrowhead,
    /// \u{2ba0}: '⮠'
    DownwardsTriangleDashHeadedArrowWithLongTipLeftwards,
    /// \u{2ba1}: '⮡'
    DownwardsTriangleDashHeadedArrowWithLongTipRightwards,
    /// \u{2ba2}: '⮢'
    UpwardsTriangleDashHeadedArrowWithLongTipLeftwards,
    /// \u{2ba3}: '⮣'
    UpwardsTriangleDashHeadedArrowWithLongTipRightwards,
    /// \u{2ba4}: '⮤'
    LeftwardsTriangleDashHeadedArrowWithLongTipUpwards,
    /// \u{2ba5}: '⮥'
    RightwardsTriangleDashHeadedArrowWithLongTipUpwards,
    /// \u{2ba6}: '⮦'
    LeftwardsTriangleDashHeadedArrowWithLongTipDownwards,
    /// \u{2ba7}: '⮧'
    RightwardsTriangleDashHeadedArrowWithLongTipDownwards,
    /// \u{2ba8}: '⮨'
    BlackCurvedDownwardsAndLeftwardsArrow,
    /// \u{2ba9}: '⮩'
    BlackCurvedDownwardsAndRightwardsArrow,
    /// \u{2baa}: '⮪'
    BlackCurvedUpwardsAndLeftwardsArrow,
    /// \u{2bab}: '⮫'
    BlackCurvedUpwardsAndRightwardsArrow,
    /// \u{2bac}: '⮬'
    BlackCurvedLeftwardsAndUpwardsArrow,
    /// \u{2bad}: '⮭'
    BlackCurvedRightwardsAndUpwardsArrow,
    /// \u{2bae}: '⮮'
    BlackCurvedLeftwardsAndDownwardsArrow,
    /// \u{2baf}: '⮯'
    BlackCurvedRightwardsAndDownwardsArrow,
    /// \u{2bb0}: '⮰'
    RibbonArrowDownLeft,
    /// \u{2bb1}: '⮱'
    RibbonArrowDownRight,
    /// \u{2bb2}: '⮲'
    RibbonArrowUpLeft,
    /// \u{2bb3}: '⮳'
    RibbonArrowUpRight,
    /// \u{2bb4}: '⮴'
    RibbonArrowLeftUp,
    /// \u{2bb5}: '⮵'
    RibbonArrowRightUp,
    /// \u{2bb6}: '⮶'
    RibbonArrowLeftDown,
    /// \u{2bb7}: '⮷'
    RibbonArrowRightDown,
    /// \u{2bb8}: '⮸'
    UpwardsWhiteArrowFromBarWithHorizontalBar,
    /// \u{2bb9}: '⮹'
    UpArrowheadInARectangleBox,
    /// \u{2bba}: '⮺'
    OverlappingWhiteSquares,
    /// \u{2bbb}: '⮻'
    OverlappingWhiteAndBlackSquares,
    /// \u{2bbc}: '⮼'
    OverlappingBlackSquares,
    /// \u{2bbd}: '⮽'
    BallotBoxWithLightX,
    /// \u{2bbe}: '⮾'
    CircledX,
    /// \u{2bbf}: '⮿'
    CircledBoldX,
    /// \u{2bc0}: '⯀'
    BlackSquareCentred,
    /// \u{2bc1}: '⯁'
    BlackDiamondCentred,
    /// \u{2bc2}: '⯂'
    TurnedBlackPentagon,
    /// \u{2bc3}: '⯃'
    HorizontalBlackOctagon,
    /// \u{2bc4}: '⯄'
    BlackOctagon,
    /// \u{2bc5}: '⯅'
    BlackMediumUpDashPointingTriangleCentred,
    /// \u{2bc6}: '⯆'
    BlackMediumDownDashPointingTriangleCentred,
    /// \u{2bc7}: '⯇'
    BlackMediumLeftDashPointingTriangleCentred,
    /// \u{2bc8}: '⯈'
    BlackMediumRightDashPointingTriangleCentred,
    /// \u{2bc9}: '⯉'
    NeptuneFormTwo,
    /// \u{2bca}: '⯊'
    TopHalfBlackCircle,
    /// \u{2bcb}: '⯋'
    BottomHalfBlackCircle,
    /// \u{2bcc}: '⯌'
    LightFourPointedBlackCusp,
    /// \u{2bcd}: '⯍'
    RotatedLightFourPointedBlackCusp,
    /// \u{2bce}: '⯎'
    WhiteFourPointedCusp,
    /// \u{2bcf}: '⯏'
    RotatedWhiteFourPointedCusp,
    /// \u{2bd0}: '⯐'
    SquarePositionIndicator,
    /// \u{2bd1}: '⯑'
    UncertaintySign,
    /// \u{2bd2}: '⯒'
    GroupMark,
    /// \u{2bd3}: '⯓'
    PlutoFormTwo,
    /// \u{2bd4}: '⯔'
    PlutoFormThree,
    /// \u{2bd5}: '⯕'
    PlutoFormFour,
    /// \u{2bd6}: '⯖'
    PlutoFormFive,
    /// \u{2bd7}: '⯗'
    Transpluto,
    /// \u{2bd8}: '⯘'
    Proserpina,
    /// \u{2bd9}: '⯙'
    Astraea,
    /// \u{2bda}: '⯚'
    Hygiea,
    /// \u{2bdb}: '⯛'
    Pholus,
    /// \u{2bdc}: '⯜'
    Nessus,
    /// \u{2bdd}: '⯝'
    WhiteMoonSelena,
    /// \u{2bde}: '⯞'
    BlackDiamondOnCross,
    /// \u{2bdf}: '⯟'
    TrueLightMoonArta,
    /// \u{2be0}: '⯠'
    Cupido,
    /// \u{2be1}: '⯡'
    Hades,
    /// \u{2be2}: '⯢'
    Zeus,
    /// \u{2be3}: '⯣'
    Kronos,
    /// \u{2be4}: '⯤'
    Apollon,
    /// \u{2be5}: '⯥'
    Admetos,
    /// \u{2be6}: '⯦'
    Vulcanus,
    /// \u{2be7}: '⯧'
    Poseidon,
    /// \u{2be8}: '⯨'
    LeftHalfBlackStar,
    /// \u{2be9}: '⯩'
    RightHalfBlackStar,
    /// \u{2bea}: '⯪'
    StarWithLeftHalfBlack,
    /// \u{2beb}: '⯫'
    StarWithRightHalfBlack,
    /// \u{2bec}: '⯬'
    LeftwardsTwoDashHeadedArrowWithTriangleArrowheads,
    /// \u{2bed}: '⯭'
    UpwardsTwoDashHeadedArrowWithTriangleArrowheads,
    /// \u{2bee}: '⯮'
    RightwardsTwoDashHeadedArrowWithTriangleArrowheads,
    /// \u{2bef}: '⯯'
    DownwardsTwoDashHeadedArrowWithTriangleArrowheads,
    /// \u{2bf0}: '⯰'
    ErisFormOne,
    /// \u{2bf1}: '⯱'
    ErisFormTwo,
    /// \u{2bf2}: '⯲'
    Sedna,
    /// \u{2bf3}: '⯳'
    RussianAstrologicalSymbolVigintile,
    /// \u{2bf4}: '⯴'
    RussianAstrologicalSymbolNovile,
    /// \u{2bf5}: '⯵'
    RussianAstrologicalSymbolQuintile,
    /// \u{2bf6}: '⯶'
    RussianAstrologicalSymbolBinovile,
    /// \u{2bf7}: '⯷'
    RussianAstrologicalSymbolSentagon,
    /// \u{2bf8}: '⯸'
    RussianAstrologicalSymbolTredecile,
    /// \u{2bf9}: '⯹'
    EqualsSignWithInfinityBelow,
    /// \u{2bfa}: '⯺'
    UnitedSymbol,
    /// \u{2bfb}: '⯻'
    SeparatedSymbol,
    /// \u{2bfc}: '⯼'
    DoubledSymbol,
    /// \u{2bfd}: '⯽'
    PassedSymbol,
    /// \u{2bfe}: '⯾'
    ReversedRightAngle,
}

impl Into<char> for MiscellaneousSymbolsandArrows {
    fn into(self) -> char {
        use constants::*;
        match self {
            MiscellaneousSymbolsandArrows::NorthEastWhiteArrow => NORTH_EAST_WHITE_ARROW,
            MiscellaneousSymbolsandArrows::NorthWestWhiteArrow => NORTH_WEST_WHITE_ARROW,
            MiscellaneousSymbolsandArrows::SouthEastWhiteArrow => SOUTH_EAST_WHITE_ARROW,
            MiscellaneousSymbolsandArrows::SouthWestWhiteArrow => SOUTH_WEST_WHITE_ARROW,
            MiscellaneousSymbolsandArrows::LeftRightWhiteArrow => LEFT_RIGHT_WHITE_ARROW,
            MiscellaneousSymbolsandArrows::LeftwardsBlackArrow => LEFTWARDS_BLACK_ARROW,
            MiscellaneousSymbolsandArrows::UpwardsBlackArrow => UPWARDS_BLACK_ARROW,
            MiscellaneousSymbolsandArrows::DownwardsBlackArrow => DOWNWARDS_BLACK_ARROW,
            MiscellaneousSymbolsandArrows::NorthEastBlackArrow => NORTH_EAST_BLACK_ARROW,
            MiscellaneousSymbolsandArrows::NorthWestBlackArrow => NORTH_WEST_BLACK_ARROW,
            MiscellaneousSymbolsandArrows::SouthEastBlackArrow => SOUTH_EAST_BLACK_ARROW,
            MiscellaneousSymbolsandArrows::SouthWestBlackArrow => SOUTH_WEST_BLACK_ARROW,
            MiscellaneousSymbolsandArrows::LeftRightBlackArrow => LEFT_RIGHT_BLACK_ARROW,
            MiscellaneousSymbolsandArrows::UpDownBlackArrow => UP_DOWN_BLACK_ARROW,
            MiscellaneousSymbolsandArrows::RightwardsArrowWithTipDownwards => RIGHTWARDS_ARROW_WITH_TIP_DOWNWARDS,
            MiscellaneousSymbolsandArrows::RightwardsArrowWithTipUpwards => RIGHTWARDS_ARROW_WITH_TIP_UPWARDS,
            MiscellaneousSymbolsandArrows::LeftwardsArrowWithTipDownwards => LEFTWARDS_ARROW_WITH_TIP_DOWNWARDS,
            MiscellaneousSymbolsandArrows::LeftwardsArrowWithTipUpwards => LEFTWARDS_ARROW_WITH_TIP_UPWARDS,
            MiscellaneousSymbolsandArrows::SquareWithTopHalfBlack => SQUARE_WITH_TOP_HALF_BLACK,
            MiscellaneousSymbolsandArrows::SquareWithBottomHalfBlack => SQUARE_WITH_BOTTOM_HALF_BLACK,
            MiscellaneousSymbolsandArrows::SquareWithUpperRightDiagonalHalfBlack => SQUARE_WITH_UPPER_RIGHT_DIAGONAL_HALF_BLACK,
            MiscellaneousSymbolsandArrows::SquareWithLowerLeftDiagonalHalfBlack => SQUARE_WITH_LOWER_LEFT_DIAGONAL_HALF_BLACK,
            MiscellaneousSymbolsandArrows::DiamondWithLeftHalfBlack => DIAMOND_WITH_LEFT_HALF_BLACK,
            MiscellaneousSymbolsandArrows::DiamondWithRightHalfBlack => DIAMOND_WITH_RIGHT_HALF_BLACK,
            MiscellaneousSymbolsandArrows::DiamondWithTopHalfBlack => DIAMOND_WITH_TOP_HALF_BLACK,
            MiscellaneousSymbolsandArrows::DiamondWithBottomHalfBlack => DIAMOND_WITH_BOTTOM_HALF_BLACK,
            MiscellaneousSymbolsandArrows::DottedSquare => DOTTED_SQUARE,
            MiscellaneousSymbolsandArrows::BlackLargeSquare => BLACK_LARGE_SQUARE,
            MiscellaneousSymbolsandArrows::WhiteLargeSquare => WHITE_LARGE_SQUARE,
            MiscellaneousSymbolsandArrows::BlackVerySmallSquare => BLACK_VERY_SMALL_SQUARE,
            MiscellaneousSymbolsandArrows::WhiteVerySmallSquare => WHITE_VERY_SMALL_SQUARE,
            MiscellaneousSymbolsandArrows::BlackPentagon => BLACK_PENTAGON,
            MiscellaneousSymbolsandArrows::WhitePentagon => WHITE_PENTAGON,
            MiscellaneousSymbolsandArrows::WhiteHexagon => WHITE_HEXAGON,
            MiscellaneousSymbolsandArrows::BlackHexagon => BLACK_HEXAGON,
            MiscellaneousSymbolsandArrows::HorizontalBlackHexagon => HORIZONTAL_BLACK_HEXAGON,
            MiscellaneousSymbolsandArrows::BlackLargeCircle => BLACK_LARGE_CIRCLE,
            MiscellaneousSymbolsandArrows::BlackMediumDiamond => BLACK_MEDIUM_DIAMOND,
            MiscellaneousSymbolsandArrows::WhiteMediumDiamond => WHITE_MEDIUM_DIAMOND,
            MiscellaneousSymbolsandArrows::BlackMediumLozenge => BLACK_MEDIUM_LOZENGE,
            MiscellaneousSymbolsandArrows::WhiteMediumLozenge => WHITE_MEDIUM_LOZENGE,
            MiscellaneousSymbolsandArrows::BlackSmallDiamond => BLACK_SMALL_DIAMOND,
            MiscellaneousSymbolsandArrows::BlackSmallLozenge => BLACK_SMALL_LOZENGE,
            MiscellaneousSymbolsandArrows::WhiteSmallLozenge => WHITE_SMALL_LOZENGE,
            MiscellaneousSymbolsandArrows::BlackHorizontalEllipse => BLACK_HORIZONTAL_ELLIPSE,
            MiscellaneousSymbolsandArrows::WhiteHorizontalEllipse => WHITE_HORIZONTAL_ELLIPSE,
            MiscellaneousSymbolsandArrows::BlackVerticalEllipse => BLACK_VERTICAL_ELLIPSE,
            MiscellaneousSymbolsandArrows::WhiteVerticalEllipse => WHITE_VERTICAL_ELLIPSE,
            MiscellaneousSymbolsandArrows::LeftArrowWithSmallCircle => LEFT_ARROW_WITH_SMALL_CIRCLE,
            MiscellaneousSymbolsandArrows::ThreeLeftwardsArrows => THREE_LEFTWARDS_ARROWS,
            MiscellaneousSymbolsandArrows::LeftArrowWithCircledPlus => LEFT_ARROW_WITH_CIRCLED_PLUS,
            MiscellaneousSymbolsandArrows::LongLeftwardsSquiggleArrow => LONG_LEFTWARDS_SQUIGGLE_ARROW,
            MiscellaneousSymbolsandArrows::LeftwardsTwoDashHeadedArrowWithVerticalStroke => LEFTWARDS_TWO_DASH_HEADED_ARROW_WITH_VERTICAL_STROKE,
            MiscellaneousSymbolsandArrows::LeftwardsTwoDashHeadedArrowWithDoubleVerticalStroke => LEFTWARDS_TWO_DASH_HEADED_ARROW_WITH_DOUBLE_VERTICAL_STROKE,
            MiscellaneousSymbolsandArrows::LeftwardsTwoDashHeadedArrowFromBar => LEFTWARDS_TWO_DASH_HEADED_ARROW_FROM_BAR,
            MiscellaneousSymbolsandArrows::LeftwardsTwoDashHeadedTripleDashArrow => LEFTWARDS_TWO_DASH_HEADED_TRIPLE_DASH_ARROW,
            MiscellaneousSymbolsandArrows::LeftwardsArrowWithDottedStem => LEFTWARDS_ARROW_WITH_DOTTED_STEM,
            MiscellaneousSymbolsandArrows::LeftwardsArrowWithTailWithVerticalStroke => LEFTWARDS_ARROW_WITH_TAIL_WITH_VERTICAL_STROKE,
            MiscellaneousSymbolsandArrows::LeftwardsArrowWithTailWithDoubleVerticalStroke => LEFTWARDS_ARROW_WITH_TAIL_WITH_DOUBLE_VERTICAL_STROKE,
            MiscellaneousSymbolsandArrows::LeftwardsTwoDashHeadedArrowWithTail => LEFTWARDS_TWO_DASH_HEADED_ARROW_WITH_TAIL,
            MiscellaneousSymbolsandArrows::LeftwardsTwoDashHeadedArrowWithTailWithVerticalStroke => LEFTWARDS_TWO_DASH_HEADED_ARROW_WITH_TAIL_WITH_VERTICAL_STROKE,
            MiscellaneousSymbolsandArrows::LeftwardsTwoDashHeadedArrowWithTailWithDoubleVerticalStroke => LEFTWARDS_TWO_DASH_HEADED_ARROW_WITH_TAIL_WITH_DOUBLE_VERTICAL_STROKE,
            MiscellaneousSymbolsandArrows::LeftwardsArrowThroughX => LEFTWARDS_ARROW_THROUGH_X,
            MiscellaneousSymbolsandArrows::WaveArrowPointingDirectlyLeft => WAVE_ARROW_POINTING_DIRECTLY_LEFT,
            MiscellaneousSymbolsandArrows::EqualsSignAboveLeftwardsArrow => EQUALS_SIGN_ABOVE_LEFTWARDS_ARROW,
            MiscellaneousSymbolsandArrows::ReverseTildeOperatorAboveLeftwardsArrow => REVERSE_TILDE_OPERATOR_ABOVE_LEFTWARDS_ARROW,
            MiscellaneousSymbolsandArrows::LeftwardsArrowAboveReverseAlmostEqualTo => LEFTWARDS_ARROW_ABOVE_REVERSE_ALMOST_EQUAL_TO,
            MiscellaneousSymbolsandArrows::RightwardsArrowThroughGreaterDashThan => RIGHTWARDS_ARROW_THROUGH_GREATER_DASH_THAN,
            MiscellaneousSymbolsandArrows::RightwardsArrowThroughSuperset => RIGHTWARDS_ARROW_THROUGH_SUPERSET,
            MiscellaneousSymbolsandArrows::LeftwardsQuadrupleArrow => LEFTWARDS_QUADRUPLE_ARROW,
            MiscellaneousSymbolsandArrows::RightwardsQuadrupleArrow => RIGHTWARDS_QUADRUPLE_ARROW,
            MiscellaneousSymbolsandArrows::ReverseTildeOperatorAboveRightwardsArrow => REVERSE_TILDE_OPERATOR_ABOVE_RIGHTWARDS_ARROW,
            MiscellaneousSymbolsandArrows::RightwardsArrowAboveReverseAlmostEqualTo => RIGHTWARDS_ARROW_ABOVE_REVERSE_ALMOST_EQUAL_TO,
            MiscellaneousSymbolsandArrows::TildeOperatorAboveLeftwardsArrow => TILDE_OPERATOR_ABOVE_LEFTWARDS_ARROW,
            MiscellaneousSymbolsandArrows::LeftwardsArrowAboveAlmostEqualTo => LEFTWARDS_ARROW_ABOVE_ALMOST_EQUAL_TO,
            MiscellaneousSymbolsandArrows::LeftwardsArrowAboveReverseTildeOperator => LEFTWARDS_ARROW_ABOVE_REVERSE_TILDE_OPERATOR,
            MiscellaneousSymbolsandArrows::RightwardsArrowAboveReverseTildeOperator => RIGHTWARDS_ARROW_ABOVE_REVERSE_TILDE_OPERATOR,
            MiscellaneousSymbolsandArrows::DownwardsTriangleDashHeadedZigzagArrow => DOWNWARDS_TRIANGLE_DASH_HEADED_ZIGZAG_ARROW,
            MiscellaneousSymbolsandArrows::ShortSlantedNorthArrow => SHORT_SLANTED_NORTH_ARROW,
            MiscellaneousSymbolsandArrows::ShortBackslantedSouthArrow => SHORT_BACKSLANTED_SOUTH_ARROW,
            MiscellaneousSymbolsandArrows::WhiteMediumStar => WHITE_MEDIUM_STAR,
            MiscellaneousSymbolsandArrows::BlackSmallStar => BLACK_SMALL_STAR,
            MiscellaneousSymbolsandArrows::WhiteSmallStar => WHITE_SMALL_STAR,
            MiscellaneousSymbolsandArrows::BlackRightDashPointingPentagon => BLACK_RIGHT_DASH_POINTING_PENTAGON,
            MiscellaneousSymbolsandArrows::WhiteRightDashPointingPentagon => WHITE_RIGHT_DASH_POINTING_PENTAGON,
            MiscellaneousSymbolsandArrows::HeavyLargeCircle => HEAVY_LARGE_CIRCLE,
            MiscellaneousSymbolsandArrows::HeavyOvalWithOvalInside => HEAVY_OVAL_WITH_OVAL_INSIDE,
            MiscellaneousSymbolsandArrows::HeavyCircleWithCircleInside => HEAVY_CIRCLE_WITH_CIRCLE_INSIDE,
            MiscellaneousSymbolsandArrows::HeavyCircle => HEAVY_CIRCLE,
            MiscellaneousSymbolsandArrows::HeavyCircledSaltire => HEAVY_CIRCLED_SALTIRE,
            MiscellaneousSymbolsandArrows::SlantedNorthArrowWithHookedHead => SLANTED_NORTH_ARROW_WITH_HOOKED_HEAD,
            MiscellaneousSymbolsandArrows::BackslantedSouthArrowWithHookedTail => BACKSLANTED_SOUTH_ARROW_WITH_HOOKED_TAIL,
            MiscellaneousSymbolsandArrows::SlantedNorthArrowWithHorizontalTail => SLANTED_NORTH_ARROW_WITH_HORIZONTAL_TAIL,
            MiscellaneousSymbolsandArrows::BackslantedSouthArrowWithHorizontalTail => BACKSLANTED_SOUTH_ARROW_WITH_HORIZONTAL_TAIL,
            MiscellaneousSymbolsandArrows::BentArrowPointingDownwardsThenNorthEast => BENT_ARROW_POINTING_DOWNWARDS_THEN_NORTH_EAST,
            MiscellaneousSymbolsandArrows::ShortBentArrowPointingDownwardsThenNorthEast => SHORT_BENT_ARROW_POINTING_DOWNWARDS_THEN_NORTH_EAST,
            MiscellaneousSymbolsandArrows::LeftwardsTriangleDashHeadedArrow => LEFTWARDS_TRIANGLE_DASH_HEADED_ARROW,
            MiscellaneousSymbolsandArrows::UpwardsTriangleDashHeadedArrow => UPWARDS_TRIANGLE_DASH_HEADED_ARROW,
            MiscellaneousSymbolsandArrows::RightwardsTriangleDashHeadedArrow => RIGHTWARDS_TRIANGLE_DASH_HEADED_ARROW,
            MiscellaneousSymbolsandArrows::DownwardsTriangleDashHeadedArrow => DOWNWARDS_TRIANGLE_DASH_HEADED_ARROW,
            MiscellaneousSymbolsandArrows::LeftRightTriangleDashHeadedArrow => LEFT_RIGHT_TRIANGLE_DASH_HEADED_ARROW,
            MiscellaneousSymbolsandArrows::UpDownTriangleDashHeadedArrow => UP_DOWN_TRIANGLE_DASH_HEADED_ARROW,
            MiscellaneousSymbolsandArrows::NorthWestTriangleDashHeadedArrow => NORTH_WEST_TRIANGLE_DASH_HEADED_ARROW,
            MiscellaneousSymbolsandArrows::NorthEastTriangleDashHeadedArrow => NORTH_EAST_TRIANGLE_DASH_HEADED_ARROW,
            MiscellaneousSymbolsandArrows::SouthEastTriangleDashHeadedArrow => SOUTH_EAST_TRIANGLE_DASH_HEADED_ARROW,
            MiscellaneousSymbolsandArrows::SouthWestTriangleDashHeadedArrow => SOUTH_WEST_TRIANGLE_DASH_HEADED_ARROW,
            MiscellaneousSymbolsandArrows::LeftwardsTriangleDashHeadedDashedArrow => LEFTWARDS_TRIANGLE_DASH_HEADED_DASHED_ARROW,
            MiscellaneousSymbolsandArrows::UpwardsTriangleDashHeadedDashedArrow => UPWARDS_TRIANGLE_DASH_HEADED_DASHED_ARROW,
            MiscellaneousSymbolsandArrows::RightwardsTriangleDashHeadedDashedArrow => RIGHTWARDS_TRIANGLE_DASH_HEADED_DASHED_ARROW,
            MiscellaneousSymbolsandArrows::DownwardsTriangleDashHeadedDashedArrow => DOWNWARDS_TRIANGLE_DASH_HEADED_DASHED_ARROW,
            MiscellaneousSymbolsandArrows::ClockwiseTriangleDashHeadedOpenCircleArrow => CLOCKWISE_TRIANGLE_DASH_HEADED_OPEN_CIRCLE_ARROW,
            MiscellaneousSymbolsandArrows::AnticlockwiseTriangleDashHeadedOpenCircleArrow => ANTICLOCKWISE_TRIANGLE_DASH_HEADED_OPEN_CIRCLE_ARROW,
            MiscellaneousSymbolsandArrows::LeftwardsTriangleDashHeadedArrowToBar => LEFTWARDS_TRIANGLE_DASH_HEADED_ARROW_TO_BAR,
            MiscellaneousSymbolsandArrows::UpwardsTriangleDashHeadedArrowToBar => UPWARDS_TRIANGLE_DASH_HEADED_ARROW_TO_BAR,
            MiscellaneousSymbolsandArrows::RightwardsTriangleDashHeadedArrowToBar => RIGHTWARDS_TRIANGLE_DASH_HEADED_ARROW_TO_BAR,
            MiscellaneousSymbolsandArrows::DownwardsTriangleDashHeadedArrowToBar => DOWNWARDS_TRIANGLE_DASH_HEADED_ARROW_TO_BAR,
            MiscellaneousSymbolsandArrows::NorthWestTriangleDashHeadedArrowToBar => NORTH_WEST_TRIANGLE_DASH_HEADED_ARROW_TO_BAR,
            MiscellaneousSymbolsandArrows::NorthEastTriangleDashHeadedArrowToBar => NORTH_EAST_TRIANGLE_DASH_HEADED_ARROW_TO_BAR,
            MiscellaneousSymbolsandArrows::SouthEastTriangleDashHeadedArrowToBar => SOUTH_EAST_TRIANGLE_DASH_HEADED_ARROW_TO_BAR,
            MiscellaneousSymbolsandArrows::SouthWestTriangleDashHeadedArrowToBar => SOUTH_WEST_TRIANGLE_DASH_HEADED_ARROW_TO_BAR,
            MiscellaneousSymbolsandArrows::LeftwardsTriangleDashHeadedArrowWithDoubleHorizontalStroke => LEFTWARDS_TRIANGLE_DASH_HEADED_ARROW_WITH_DOUBLE_HORIZONTAL_STROKE,
            MiscellaneousSymbolsandArrows::UpwardsTriangleDashHeadedArrowWithDoubleHorizontalStroke => UPWARDS_TRIANGLE_DASH_HEADED_ARROW_WITH_DOUBLE_HORIZONTAL_STROKE,
            MiscellaneousSymbolsandArrows::RightwardsTriangleDashHeadedArrowWithDoubleHorizontalStroke => RIGHTWARDS_TRIANGLE_DASH_HEADED_ARROW_WITH_DOUBLE_HORIZONTAL_STROKE,
            MiscellaneousSymbolsandArrows::DownwardsTriangleDashHeadedArrowWithDoubleHorizontalStroke => DOWNWARDS_TRIANGLE_DASH_HEADED_ARROW_WITH_DOUBLE_HORIZONTAL_STROKE,
            MiscellaneousSymbolsandArrows::HorizontalTabKey => HORIZONTAL_TAB_KEY,
            MiscellaneousSymbolsandArrows::VerticalTabKey => VERTICAL_TAB_KEY,
            MiscellaneousSymbolsandArrows::LeftwardsTriangleDashHeadedArrowOverRightwardsTriangleDashHeadedArrow => LEFTWARDS_TRIANGLE_DASH_HEADED_ARROW_OVER_RIGHTWARDS_TRIANGLE_DASH_HEADED_ARROW,
            MiscellaneousSymbolsandArrows::UpwardsTriangleDashHeadedArrowLeftwardsOfDownwardsTriangleDashHeadedArrow => UPWARDS_TRIANGLE_DASH_HEADED_ARROW_LEFTWARDS_OF_DOWNWARDS_TRIANGLE_DASH_HEADED_ARROW,
            MiscellaneousSymbolsandArrows::RightwardsTriangleDashHeadedArrowOverLeftwardsTriangleDashHeadedArrow => RIGHTWARDS_TRIANGLE_DASH_HEADED_ARROW_OVER_LEFTWARDS_TRIANGLE_DASH_HEADED_ARROW,
            MiscellaneousSymbolsandArrows::DownwardsTriangleDashHeadedArrowLeftwardsOfUpwardsTriangleDashHeadedArrow => DOWNWARDS_TRIANGLE_DASH_HEADED_ARROW_LEFTWARDS_OF_UPWARDS_TRIANGLE_DASH_HEADED_ARROW,
            MiscellaneousSymbolsandArrows::LeftwardsTriangleDashHeadedPairedArrows => LEFTWARDS_TRIANGLE_DASH_HEADED_PAIRED_ARROWS,
            MiscellaneousSymbolsandArrows::UpwardsTriangleDashHeadedPairedArrows => UPWARDS_TRIANGLE_DASH_HEADED_PAIRED_ARROWS,
            MiscellaneousSymbolsandArrows::RightwardsTriangleDashHeadedPairedArrows => RIGHTWARDS_TRIANGLE_DASH_HEADED_PAIRED_ARROWS,
            MiscellaneousSymbolsandArrows::DownwardsTriangleDashHeadedPairedArrows => DOWNWARDS_TRIANGLE_DASH_HEADED_PAIRED_ARROWS,
            MiscellaneousSymbolsandArrows::LeftwardsBlackCircledWhiteArrow => LEFTWARDS_BLACK_CIRCLED_WHITE_ARROW,
            MiscellaneousSymbolsandArrows::UpwardsBlackCircledWhiteArrow => UPWARDS_BLACK_CIRCLED_WHITE_ARROW,
            MiscellaneousSymbolsandArrows::RightwardsBlackCircledWhiteArrow => RIGHTWARDS_BLACK_CIRCLED_WHITE_ARROW,
            MiscellaneousSymbolsandArrows::DownwardsBlackCircledWhiteArrow => DOWNWARDS_BLACK_CIRCLED_WHITE_ARROW,
            MiscellaneousSymbolsandArrows::AnticlockwiseTriangleDashHeadedRightUDashShapedArrow => ANTICLOCKWISE_TRIANGLE_DASH_HEADED_RIGHT_U_DASH_SHAPED_ARROW,
            MiscellaneousSymbolsandArrows::AnticlockwiseTriangleDashHeadedBottomUDashShapedArrow => ANTICLOCKWISE_TRIANGLE_DASH_HEADED_BOTTOM_U_DASH_SHAPED_ARROW,
            MiscellaneousSymbolsandArrows::AnticlockwiseTriangleDashHeadedLeftUDashShapedArrow => ANTICLOCKWISE_TRIANGLE_DASH_HEADED_LEFT_U_DASH_SHAPED_ARROW,
            MiscellaneousSymbolsandArrows::AnticlockwiseTriangleDashHeadedTopUDashShapedArrow => ANTICLOCKWISE_TRIANGLE_DASH_HEADED_TOP_U_DASH_SHAPED_ARROW,
            MiscellaneousSymbolsandArrows::ReturnLeft => RETURN_LEFT,
            MiscellaneousSymbolsandArrows::ReturnRight => RETURN_RIGHT,
            MiscellaneousSymbolsandArrows::NewlineLeft => NEWLINE_LEFT,
            MiscellaneousSymbolsandArrows::NewlineRight => NEWLINE_RIGHT,
            MiscellaneousSymbolsandArrows::FourCornerArrowsCirclingAnticlockwise => FOUR_CORNER_ARROWS_CIRCLING_ANTICLOCKWISE,
            MiscellaneousSymbolsandArrows::RightwardsBlackArrow => RIGHTWARDS_BLACK_ARROW,
            MiscellaneousSymbolsandArrows::ThreeDashDTopDashLightedLeftwardsEquilateralArrowhead => THREE_DASH_D_TOP_DASH_LIGHTED_LEFTWARDS_EQUILATERAL_ARROWHEAD,
            MiscellaneousSymbolsandArrows::ThreeDashDRightDashLightedUpwardsEquilateralArrowhead => THREE_DASH_D_RIGHT_DASH_LIGHTED_UPWARDS_EQUILATERAL_ARROWHEAD,
            MiscellaneousSymbolsandArrows::ThreeDashDTopDashLightedRightwardsEquilateralArrowhead => THREE_DASH_D_TOP_DASH_LIGHTED_RIGHTWARDS_EQUILATERAL_ARROWHEAD,
            MiscellaneousSymbolsandArrows::ThreeDashDLeftDashLightedDownwardsEquilateralArrowhead => THREE_DASH_D_LEFT_DASH_LIGHTED_DOWNWARDS_EQUILATERAL_ARROWHEAD,
            MiscellaneousSymbolsandArrows::BlackLeftwardsEquilateralArrowhead => BLACK_LEFTWARDS_EQUILATERAL_ARROWHEAD,
            MiscellaneousSymbolsandArrows::BlackUpwardsEquilateralArrowhead => BLACK_UPWARDS_EQUILATERAL_ARROWHEAD,
            MiscellaneousSymbolsandArrows::BlackRightwardsEquilateralArrowhead => BLACK_RIGHTWARDS_EQUILATERAL_ARROWHEAD,
            MiscellaneousSymbolsandArrows::BlackDownwardsEquilateralArrowhead => BLACK_DOWNWARDS_EQUILATERAL_ARROWHEAD,
            MiscellaneousSymbolsandArrows::DownwardsTriangleDashHeadedArrowWithLongTipLeftwards => DOWNWARDS_TRIANGLE_DASH_HEADED_ARROW_WITH_LONG_TIP_LEFTWARDS,
            MiscellaneousSymbolsandArrows::DownwardsTriangleDashHeadedArrowWithLongTipRightwards => DOWNWARDS_TRIANGLE_DASH_HEADED_ARROW_WITH_LONG_TIP_RIGHTWARDS,
            MiscellaneousSymbolsandArrows::UpwardsTriangleDashHeadedArrowWithLongTipLeftwards => UPWARDS_TRIANGLE_DASH_HEADED_ARROW_WITH_LONG_TIP_LEFTWARDS,
            MiscellaneousSymbolsandArrows::UpwardsTriangleDashHeadedArrowWithLongTipRightwards => UPWARDS_TRIANGLE_DASH_HEADED_ARROW_WITH_LONG_TIP_RIGHTWARDS,
            MiscellaneousSymbolsandArrows::LeftwardsTriangleDashHeadedArrowWithLongTipUpwards => LEFTWARDS_TRIANGLE_DASH_HEADED_ARROW_WITH_LONG_TIP_UPWARDS,
            MiscellaneousSymbolsandArrows::RightwardsTriangleDashHeadedArrowWithLongTipUpwards => RIGHTWARDS_TRIANGLE_DASH_HEADED_ARROW_WITH_LONG_TIP_UPWARDS,
            MiscellaneousSymbolsandArrows::LeftwardsTriangleDashHeadedArrowWithLongTipDownwards => LEFTWARDS_TRIANGLE_DASH_HEADED_ARROW_WITH_LONG_TIP_DOWNWARDS,
            MiscellaneousSymbolsandArrows::RightwardsTriangleDashHeadedArrowWithLongTipDownwards => RIGHTWARDS_TRIANGLE_DASH_HEADED_ARROW_WITH_LONG_TIP_DOWNWARDS,
            MiscellaneousSymbolsandArrows::BlackCurvedDownwardsAndLeftwardsArrow => BLACK_CURVED_DOWNWARDS_AND_LEFTWARDS_ARROW,
            MiscellaneousSymbolsandArrows::BlackCurvedDownwardsAndRightwardsArrow => BLACK_CURVED_DOWNWARDS_AND_RIGHTWARDS_ARROW,
            MiscellaneousSymbolsandArrows::BlackCurvedUpwardsAndLeftwardsArrow => BLACK_CURVED_UPWARDS_AND_LEFTWARDS_ARROW,
            MiscellaneousSymbolsandArrows::BlackCurvedUpwardsAndRightwardsArrow => BLACK_CURVED_UPWARDS_AND_RIGHTWARDS_ARROW,
            MiscellaneousSymbolsandArrows::BlackCurvedLeftwardsAndUpwardsArrow => BLACK_CURVED_LEFTWARDS_AND_UPWARDS_ARROW,
            MiscellaneousSymbolsandArrows::BlackCurvedRightwardsAndUpwardsArrow => BLACK_CURVED_RIGHTWARDS_AND_UPWARDS_ARROW,
            MiscellaneousSymbolsandArrows::BlackCurvedLeftwardsAndDownwardsArrow => BLACK_CURVED_LEFTWARDS_AND_DOWNWARDS_ARROW,
            MiscellaneousSymbolsandArrows::BlackCurvedRightwardsAndDownwardsArrow => BLACK_CURVED_RIGHTWARDS_AND_DOWNWARDS_ARROW,
            MiscellaneousSymbolsandArrows::RibbonArrowDownLeft => RIBBON_ARROW_DOWN_LEFT,
            MiscellaneousSymbolsandArrows::RibbonArrowDownRight => RIBBON_ARROW_DOWN_RIGHT,
            MiscellaneousSymbolsandArrows::RibbonArrowUpLeft => RIBBON_ARROW_UP_LEFT,
            MiscellaneousSymbolsandArrows::RibbonArrowUpRight => RIBBON_ARROW_UP_RIGHT,
            MiscellaneousSymbolsandArrows::RibbonArrowLeftUp => RIBBON_ARROW_LEFT_UP,
            MiscellaneousSymbolsandArrows::RibbonArrowRightUp => RIBBON_ARROW_RIGHT_UP,
            MiscellaneousSymbolsandArrows::RibbonArrowLeftDown => RIBBON_ARROW_LEFT_DOWN,
            MiscellaneousSymbolsandArrows::RibbonArrowRightDown => RIBBON_ARROW_RIGHT_DOWN,
            MiscellaneousSymbolsandArrows::UpwardsWhiteArrowFromBarWithHorizontalBar => UPWARDS_WHITE_ARROW_FROM_BAR_WITH_HORIZONTAL_BAR,
            MiscellaneousSymbolsandArrows::UpArrowheadInARectangleBox => UP_ARROWHEAD_IN_A_RECTANGLE_BOX,
            MiscellaneousSymbolsandArrows::OverlappingWhiteSquares => OVERLAPPING_WHITE_SQUARES,
            MiscellaneousSymbolsandArrows::OverlappingWhiteAndBlackSquares => OVERLAPPING_WHITE_AND_BLACK_SQUARES,
            MiscellaneousSymbolsandArrows::OverlappingBlackSquares => OVERLAPPING_BLACK_SQUARES,
            MiscellaneousSymbolsandArrows::BallotBoxWithLightX => BALLOT_BOX_WITH_LIGHT_X,
            MiscellaneousSymbolsandArrows::CircledX => CIRCLED_X,
            MiscellaneousSymbolsandArrows::CircledBoldX => CIRCLED_BOLD_X,
            MiscellaneousSymbolsandArrows::BlackSquareCentred => BLACK_SQUARE_CENTRED,
            MiscellaneousSymbolsandArrows::BlackDiamondCentred => BLACK_DIAMOND_CENTRED,
            MiscellaneousSymbolsandArrows::TurnedBlackPentagon => TURNED_BLACK_PENTAGON,
            MiscellaneousSymbolsandArrows::HorizontalBlackOctagon => HORIZONTAL_BLACK_OCTAGON,
            MiscellaneousSymbolsandArrows::BlackOctagon => BLACK_OCTAGON,
            MiscellaneousSymbolsandArrows::BlackMediumUpDashPointingTriangleCentred => BLACK_MEDIUM_UP_DASH_POINTING_TRIANGLE_CENTRED,
            MiscellaneousSymbolsandArrows::BlackMediumDownDashPointingTriangleCentred => BLACK_MEDIUM_DOWN_DASH_POINTING_TRIANGLE_CENTRED,
            MiscellaneousSymbolsandArrows::BlackMediumLeftDashPointingTriangleCentred => BLACK_MEDIUM_LEFT_DASH_POINTING_TRIANGLE_CENTRED,
            MiscellaneousSymbolsandArrows::BlackMediumRightDashPointingTriangleCentred => BLACK_MEDIUM_RIGHT_DASH_POINTING_TRIANGLE_CENTRED,
            MiscellaneousSymbolsandArrows::NeptuneFormTwo => NEPTUNE_FORM_TWO,
            MiscellaneousSymbolsandArrows::TopHalfBlackCircle => TOP_HALF_BLACK_CIRCLE,
            MiscellaneousSymbolsandArrows::BottomHalfBlackCircle => BOTTOM_HALF_BLACK_CIRCLE,
            MiscellaneousSymbolsandArrows::LightFourPointedBlackCusp => LIGHT_FOUR_POINTED_BLACK_CUSP,
            MiscellaneousSymbolsandArrows::RotatedLightFourPointedBlackCusp => ROTATED_LIGHT_FOUR_POINTED_BLACK_CUSP,
            MiscellaneousSymbolsandArrows::WhiteFourPointedCusp => WHITE_FOUR_POINTED_CUSP,
            MiscellaneousSymbolsandArrows::RotatedWhiteFourPointedCusp => ROTATED_WHITE_FOUR_POINTED_CUSP,
            MiscellaneousSymbolsandArrows::SquarePositionIndicator => SQUARE_POSITION_INDICATOR,
            MiscellaneousSymbolsandArrows::UncertaintySign => UNCERTAINTY_SIGN,
            MiscellaneousSymbolsandArrows::GroupMark => GROUP_MARK,
            MiscellaneousSymbolsandArrows::PlutoFormTwo => PLUTO_FORM_TWO,
            MiscellaneousSymbolsandArrows::PlutoFormThree => PLUTO_FORM_THREE,
            MiscellaneousSymbolsandArrows::PlutoFormFour => PLUTO_FORM_FOUR,
            MiscellaneousSymbolsandArrows::PlutoFormFive => PLUTO_FORM_FIVE,
            MiscellaneousSymbolsandArrows::Transpluto => TRANSPLUTO,
            MiscellaneousSymbolsandArrows::Proserpina => PROSERPINA,
            MiscellaneousSymbolsandArrows::Astraea => ASTRAEA,
            MiscellaneousSymbolsandArrows::Hygiea => HYGIEA,
            MiscellaneousSymbolsandArrows::Pholus => PHOLUS,
            MiscellaneousSymbolsandArrows::Nessus => NESSUS,
            MiscellaneousSymbolsandArrows::WhiteMoonSelena => WHITE_MOON_SELENA,
            MiscellaneousSymbolsandArrows::BlackDiamondOnCross => BLACK_DIAMOND_ON_CROSS,
            MiscellaneousSymbolsandArrows::TrueLightMoonArta => TRUE_LIGHT_MOON_ARTA,
            MiscellaneousSymbolsandArrows::Cupido => CUPIDO,
            MiscellaneousSymbolsandArrows::Hades => HADES,
            MiscellaneousSymbolsandArrows::Zeus => ZEUS,
            MiscellaneousSymbolsandArrows::Kronos => KRONOS,
            MiscellaneousSymbolsandArrows::Apollon => APOLLON,
            MiscellaneousSymbolsandArrows::Admetos => ADMETOS,
            MiscellaneousSymbolsandArrows::Vulcanus => VULCANUS,
            MiscellaneousSymbolsandArrows::Poseidon => POSEIDON,
            MiscellaneousSymbolsandArrows::LeftHalfBlackStar => LEFT_HALF_BLACK_STAR,
            MiscellaneousSymbolsandArrows::RightHalfBlackStar => RIGHT_HALF_BLACK_STAR,
            MiscellaneousSymbolsandArrows::StarWithLeftHalfBlack => STAR_WITH_LEFT_HALF_BLACK,
            MiscellaneousSymbolsandArrows::StarWithRightHalfBlack => STAR_WITH_RIGHT_HALF_BLACK,
            MiscellaneousSymbolsandArrows::LeftwardsTwoDashHeadedArrowWithTriangleArrowheads => LEFTWARDS_TWO_DASH_HEADED_ARROW_WITH_TRIANGLE_ARROWHEADS,
            MiscellaneousSymbolsandArrows::UpwardsTwoDashHeadedArrowWithTriangleArrowheads => UPWARDS_TWO_DASH_HEADED_ARROW_WITH_TRIANGLE_ARROWHEADS,
            MiscellaneousSymbolsandArrows::RightwardsTwoDashHeadedArrowWithTriangleArrowheads => RIGHTWARDS_TWO_DASH_HEADED_ARROW_WITH_TRIANGLE_ARROWHEADS,
            MiscellaneousSymbolsandArrows::DownwardsTwoDashHeadedArrowWithTriangleArrowheads => DOWNWARDS_TWO_DASH_HEADED_ARROW_WITH_TRIANGLE_ARROWHEADS,
            MiscellaneousSymbolsandArrows::ErisFormOne => ERIS_FORM_ONE,
            MiscellaneousSymbolsandArrows::ErisFormTwo => ERIS_FORM_TWO,
            MiscellaneousSymbolsandArrows::Sedna => SEDNA,
            MiscellaneousSymbolsandArrows::RussianAstrologicalSymbolVigintile => RUSSIAN_ASTROLOGICAL_SYMBOL_VIGINTILE,
            MiscellaneousSymbolsandArrows::RussianAstrologicalSymbolNovile => RUSSIAN_ASTROLOGICAL_SYMBOL_NOVILE,
            MiscellaneousSymbolsandArrows::RussianAstrologicalSymbolQuintile => RUSSIAN_ASTROLOGICAL_SYMBOL_QUINTILE,
            MiscellaneousSymbolsandArrows::RussianAstrologicalSymbolBinovile => RUSSIAN_ASTROLOGICAL_SYMBOL_BINOVILE,
            MiscellaneousSymbolsandArrows::RussianAstrologicalSymbolSentagon => RUSSIAN_ASTROLOGICAL_SYMBOL_SENTAGON,
            MiscellaneousSymbolsandArrows::RussianAstrologicalSymbolTredecile => RUSSIAN_ASTROLOGICAL_SYMBOL_TREDECILE,
            MiscellaneousSymbolsandArrows::EqualsSignWithInfinityBelow => EQUALS_SIGN_WITH_INFINITY_BELOW,
            MiscellaneousSymbolsandArrows::UnitedSymbol => UNITED_SYMBOL,
            MiscellaneousSymbolsandArrows::SeparatedSymbol => SEPARATED_SYMBOL,
            MiscellaneousSymbolsandArrows::DoubledSymbol => DOUBLED_SYMBOL,
            MiscellaneousSymbolsandArrows::PassedSymbol => PASSED_SYMBOL,
            MiscellaneousSymbolsandArrows::ReversedRightAngle => REVERSED_RIGHT_ANGLE,
        }
    }
}

impl std::convert::TryFrom<char> for MiscellaneousSymbolsandArrows {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            NORTH_EAST_WHITE_ARROW => Ok(MiscellaneousSymbolsandArrows::NorthEastWhiteArrow),
            NORTH_WEST_WHITE_ARROW => Ok(MiscellaneousSymbolsandArrows::NorthWestWhiteArrow),
            SOUTH_EAST_WHITE_ARROW => Ok(MiscellaneousSymbolsandArrows::SouthEastWhiteArrow),
            SOUTH_WEST_WHITE_ARROW => Ok(MiscellaneousSymbolsandArrows::SouthWestWhiteArrow),
            LEFT_RIGHT_WHITE_ARROW => Ok(MiscellaneousSymbolsandArrows::LeftRightWhiteArrow),
            LEFTWARDS_BLACK_ARROW => Ok(MiscellaneousSymbolsandArrows::LeftwardsBlackArrow),
            UPWARDS_BLACK_ARROW => Ok(MiscellaneousSymbolsandArrows::UpwardsBlackArrow),
            DOWNWARDS_BLACK_ARROW => Ok(MiscellaneousSymbolsandArrows::DownwardsBlackArrow),
            NORTH_EAST_BLACK_ARROW => Ok(MiscellaneousSymbolsandArrows::NorthEastBlackArrow),
            NORTH_WEST_BLACK_ARROW => Ok(MiscellaneousSymbolsandArrows::NorthWestBlackArrow),
            SOUTH_EAST_BLACK_ARROW => Ok(MiscellaneousSymbolsandArrows::SouthEastBlackArrow),
            SOUTH_WEST_BLACK_ARROW => Ok(MiscellaneousSymbolsandArrows::SouthWestBlackArrow),
            LEFT_RIGHT_BLACK_ARROW => Ok(MiscellaneousSymbolsandArrows::LeftRightBlackArrow),
            UP_DOWN_BLACK_ARROW => Ok(MiscellaneousSymbolsandArrows::UpDownBlackArrow),
            RIGHTWARDS_ARROW_WITH_TIP_DOWNWARDS => Ok(MiscellaneousSymbolsandArrows::RightwardsArrowWithTipDownwards),
            RIGHTWARDS_ARROW_WITH_TIP_UPWARDS => Ok(MiscellaneousSymbolsandArrows::RightwardsArrowWithTipUpwards),
            LEFTWARDS_ARROW_WITH_TIP_DOWNWARDS => Ok(MiscellaneousSymbolsandArrows::LeftwardsArrowWithTipDownwards),
            LEFTWARDS_ARROW_WITH_TIP_UPWARDS => Ok(MiscellaneousSymbolsandArrows::LeftwardsArrowWithTipUpwards),
            SQUARE_WITH_TOP_HALF_BLACK => Ok(MiscellaneousSymbolsandArrows::SquareWithTopHalfBlack),
            SQUARE_WITH_BOTTOM_HALF_BLACK => Ok(MiscellaneousSymbolsandArrows::SquareWithBottomHalfBlack),
            SQUARE_WITH_UPPER_RIGHT_DIAGONAL_HALF_BLACK => Ok(MiscellaneousSymbolsandArrows::SquareWithUpperRightDiagonalHalfBlack),
            SQUARE_WITH_LOWER_LEFT_DIAGONAL_HALF_BLACK => Ok(MiscellaneousSymbolsandArrows::SquareWithLowerLeftDiagonalHalfBlack),
            DIAMOND_WITH_LEFT_HALF_BLACK => Ok(MiscellaneousSymbolsandArrows::DiamondWithLeftHalfBlack),
            DIAMOND_WITH_RIGHT_HALF_BLACK => Ok(MiscellaneousSymbolsandArrows::DiamondWithRightHalfBlack),
            DIAMOND_WITH_TOP_HALF_BLACK => Ok(MiscellaneousSymbolsandArrows::DiamondWithTopHalfBlack),
            DIAMOND_WITH_BOTTOM_HALF_BLACK => Ok(MiscellaneousSymbolsandArrows::DiamondWithBottomHalfBlack),
            DOTTED_SQUARE => Ok(MiscellaneousSymbolsandArrows::DottedSquare),
            BLACK_LARGE_SQUARE => Ok(MiscellaneousSymbolsandArrows::BlackLargeSquare),
            WHITE_LARGE_SQUARE => Ok(MiscellaneousSymbolsandArrows::WhiteLargeSquare),
            BLACK_VERY_SMALL_SQUARE => Ok(MiscellaneousSymbolsandArrows::BlackVerySmallSquare),
            WHITE_VERY_SMALL_SQUARE => Ok(MiscellaneousSymbolsandArrows::WhiteVerySmallSquare),
            BLACK_PENTAGON => Ok(MiscellaneousSymbolsandArrows::BlackPentagon),
            WHITE_PENTAGON => Ok(MiscellaneousSymbolsandArrows::WhitePentagon),
            WHITE_HEXAGON => Ok(MiscellaneousSymbolsandArrows::WhiteHexagon),
            BLACK_HEXAGON => Ok(MiscellaneousSymbolsandArrows::BlackHexagon),
            HORIZONTAL_BLACK_HEXAGON => Ok(MiscellaneousSymbolsandArrows::HorizontalBlackHexagon),
            BLACK_LARGE_CIRCLE => Ok(MiscellaneousSymbolsandArrows::BlackLargeCircle),
            BLACK_MEDIUM_DIAMOND => Ok(MiscellaneousSymbolsandArrows::BlackMediumDiamond),
            WHITE_MEDIUM_DIAMOND => Ok(MiscellaneousSymbolsandArrows::WhiteMediumDiamond),
            BLACK_MEDIUM_LOZENGE => Ok(MiscellaneousSymbolsandArrows::BlackMediumLozenge),
            WHITE_MEDIUM_LOZENGE => Ok(MiscellaneousSymbolsandArrows::WhiteMediumLozenge),
            BLACK_SMALL_DIAMOND => Ok(MiscellaneousSymbolsandArrows::BlackSmallDiamond),
            BLACK_SMALL_LOZENGE => Ok(MiscellaneousSymbolsandArrows::BlackSmallLozenge),
            WHITE_SMALL_LOZENGE => Ok(MiscellaneousSymbolsandArrows::WhiteSmallLozenge),
            BLACK_HORIZONTAL_ELLIPSE => Ok(MiscellaneousSymbolsandArrows::BlackHorizontalEllipse),
            WHITE_HORIZONTAL_ELLIPSE => Ok(MiscellaneousSymbolsandArrows::WhiteHorizontalEllipse),
            BLACK_VERTICAL_ELLIPSE => Ok(MiscellaneousSymbolsandArrows::BlackVerticalEllipse),
            WHITE_VERTICAL_ELLIPSE => Ok(MiscellaneousSymbolsandArrows::WhiteVerticalEllipse),
            LEFT_ARROW_WITH_SMALL_CIRCLE => Ok(MiscellaneousSymbolsandArrows::LeftArrowWithSmallCircle),
            THREE_LEFTWARDS_ARROWS => Ok(MiscellaneousSymbolsandArrows::ThreeLeftwardsArrows),
            LEFT_ARROW_WITH_CIRCLED_PLUS => Ok(MiscellaneousSymbolsandArrows::LeftArrowWithCircledPlus),
            LONG_LEFTWARDS_SQUIGGLE_ARROW => Ok(MiscellaneousSymbolsandArrows::LongLeftwardsSquiggleArrow),
            LEFTWARDS_TWO_DASH_HEADED_ARROW_WITH_VERTICAL_STROKE => Ok(MiscellaneousSymbolsandArrows::LeftwardsTwoDashHeadedArrowWithVerticalStroke),
            LEFTWARDS_TWO_DASH_HEADED_ARROW_WITH_DOUBLE_VERTICAL_STROKE => Ok(MiscellaneousSymbolsandArrows::LeftwardsTwoDashHeadedArrowWithDoubleVerticalStroke),
            LEFTWARDS_TWO_DASH_HEADED_ARROW_FROM_BAR => Ok(MiscellaneousSymbolsandArrows::LeftwardsTwoDashHeadedArrowFromBar),
            LEFTWARDS_TWO_DASH_HEADED_TRIPLE_DASH_ARROW => Ok(MiscellaneousSymbolsandArrows::LeftwardsTwoDashHeadedTripleDashArrow),
            LEFTWARDS_ARROW_WITH_DOTTED_STEM => Ok(MiscellaneousSymbolsandArrows::LeftwardsArrowWithDottedStem),
            LEFTWARDS_ARROW_WITH_TAIL_WITH_VERTICAL_STROKE => Ok(MiscellaneousSymbolsandArrows::LeftwardsArrowWithTailWithVerticalStroke),
            LEFTWARDS_ARROW_WITH_TAIL_WITH_DOUBLE_VERTICAL_STROKE => Ok(MiscellaneousSymbolsandArrows::LeftwardsArrowWithTailWithDoubleVerticalStroke),
            LEFTWARDS_TWO_DASH_HEADED_ARROW_WITH_TAIL => Ok(MiscellaneousSymbolsandArrows::LeftwardsTwoDashHeadedArrowWithTail),
            LEFTWARDS_TWO_DASH_HEADED_ARROW_WITH_TAIL_WITH_VERTICAL_STROKE => Ok(MiscellaneousSymbolsandArrows::LeftwardsTwoDashHeadedArrowWithTailWithVerticalStroke),
            LEFTWARDS_TWO_DASH_HEADED_ARROW_WITH_TAIL_WITH_DOUBLE_VERTICAL_STROKE => Ok(MiscellaneousSymbolsandArrows::LeftwardsTwoDashHeadedArrowWithTailWithDoubleVerticalStroke),
            LEFTWARDS_ARROW_THROUGH_X => Ok(MiscellaneousSymbolsandArrows::LeftwardsArrowThroughX),
            WAVE_ARROW_POINTING_DIRECTLY_LEFT => Ok(MiscellaneousSymbolsandArrows::WaveArrowPointingDirectlyLeft),
            EQUALS_SIGN_ABOVE_LEFTWARDS_ARROW => Ok(MiscellaneousSymbolsandArrows::EqualsSignAboveLeftwardsArrow),
            REVERSE_TILDE_OPERATOR_ABOVE_LEFTWARDS_ARROW => Ok(MiscellaneousSymbolsandArrows::ReverseTildeOperatorAboveLeftwardsArrow),
            LEFTWARDS_ARROW_ABOVE_REVERSE_ALMOST_EQUAL_TO => Ok(MiscellaneousSymbolsandArrows::LeftwardsArrowAboveReverseAlmostEqualTo),
            RIGHTWARDS_ARROW_THROUGH_GREATER_DASH_THAN => Ok(MiscellaneousSymbolsandArrows::RightwardsArrowThroughGreaterDashThan),
            RIGHTWARDS_ARROW_THROUGH_SUPERSET => Ok(MiscellaneousSymbolsandArrows::RightwardsArrowThroughSuperset),
            LEFTWARDS_QUADRUPLE_ARROW => Ok(MiscellaneousSymbolsandArrows::LeftwardsQuadrupleArrow),
            RIGHTWARDS_QUADRUPLE_ARROW => Ok(MiscellaneousSymbolsandArrows::RightwardsQuadrupleArrow),
            REVERSE_TILDE_OPERATOR_ABOVE_RIGHTWARDS_ARROW => Ok(MiscellaneousSymbolsandArrows::ReverseTildeOperatorAboveRightwardsArrow),
            RIGHTWARDS_ARROW_ABOVE_REVERSE_ALMOST_EQUAL_TO => Ok(MiscellaneousSymbolsandArrows::RightwardsArrowAboveReverseAlmostEqualTo),
            TILDE_OPERATOR_ABOVE_LEFTWARDS_ARROW => Ok(MiscellaneousSymbolsandArrows::TildeOperatorAboveLeftwardsArrow),
            LEFTWARDS_ARROW_ABOVE_ALMOST_EQUAL_TO => Ok(MiscellaneousSymbolsandArrows::LeftwardsArrowAboveAlmostEqualTo),
            LEFTWARDS_ARROW_ABOVE_REVERSE_TILDE_OPERATOR => Ok(MiscellaneousSymbolsandArrows::LeftwardsArrowAboveReverseTildeOperator),
            RIGHTWARDS_ARROW_ABOVE_REVERSE_TILDE_OPERATOR => Ok(MiscellaneousSymbolsandArrows::RightwardsArrowAboveReverseTildeOperator),
            DOWNWARDS_TRIANGLE_DASH_HEADED_ZIGZAG_ARROW => Ok(MiscellaneousSymbolsandArrows::DownwardsTriangleDashHeadedZigzagArrow),
            SHORT_SLANTED_NORTH_ARROW => Ok(MiscellaneousSymbolsandArrows::ShortSlantedNorthArrow),
            SHORT_BACKSLANTED_SOUTH_ARROW => Ok(MiscellaneousSymbolsandArrows::ShortBackslantedSouthArrow),
            WHITE_MEDIUM_STAR => Ok(MiscellaneousSymbolsandArrows::WhiteMediumStar),
            BLACK_SMALL_STAR => Ok(MiscellaneousSymbolsandArrows::BlackSmallStar),
            WHITE_SMALL_STAR => Ok(MiscellaneousSymbolsandArrows::WhiteSmallStar),
            BLACK_RIGHT_DASH_POINTING_PENTAGON => Ok(MiscellaneousSymbolsandArrows::BlackRightDashPointingPentagon),
            WHITE_RIGHT_DASH_POINTING_PENTAGON => Ok(MiscellaneousSymbolsandArrows::WhiteRightDashPointingPentagon),
            HEAVY_LARGE_CIRCLE => Ok(MiscellaneousSymbolsandArrows::HeavyLargeCircle),
            HEAVY_OVAL_WITH_OVAL_INSIDE => Ok(MiscellaneousSymbolsandArrows::HeavyOvalWithOvalInside),
            HEAVY_CIRCLE_WITH_CIRCLE_INSIDE => Ok(MiscellaneousSymbolsandArrows::HeavyCircleWithCircleInside),
            HEAVY_CIRCLE => Ok(MiscellaneousSymbolsandArrows::HeavyCircle),
            HEAVY_CIRCLED_SALTIRE => Ok(MiscellaneousSymbolsandArrows::HeavyCircledSaltire),
            SLANTED_NORTH_ARROW_WITH_HOOKED_HEAD => Ok(MiscellaneousSymbolsandArrows::SlantedNorthArrowWithHookedHead),
            BACKSLANTED_SOUTH_ARROW_WITH_HOOKED_TAIL => Ok(MiscellaneousSymbolsandArrows::BackslantedSouthArrowWithHookedTail),
            SLANTED_NORTH_ARROW_WITH_HORIZONTAL_TAIL => Ok(MiscellaneousSymbolsandArrows::SlantedNorthArrowWithHorizontalTail),
            BACKSLANTED_SOUTH_ARROW_WITH_HORIZONTAL_TAIL => Ok(MiscellaneousSymbolsandArrows::BackslantedSouthArrowWithHorizontalTail),
            BENT_ARROW_POINTING_DOWNWARDS_THEN_NORTH_EAST => Ok(MiscellaneousSymbolsandArrows::BentArrowPointingDownwardsThenNorthEast),
            SHORT_BENT_ARROW_POINTING_DOWNWARDS_THEN_NORTH_EAST => Ok(MiscellaneousSymbolsandArrows::ShortBentArrowPointingDownwardsThenNorthEast),
            LEFTWARDS_TRIANGLE_DASH_HEADED_ARROW => Ok(MiscellaneousSymbolsandArrows::LeftwardsTriangleDashHeadedArrow),
            UPWARDS_TRIANGLE_DASH_HEADED_ARROW => Ok(MiscellaneousSymbolsandArrows::UpwardsTriangleDashHeadedArrow),
            RIGHTWARDS_TRIANGLE_DASH_HEADED_ARROW => Ok(MiscellaneousSymbolsandArrows::RightwardsTriangleDashHeadedArrow),
            DOWNWARDS_TRIANGLE_DASH_HEADED_ARROW => Ok(MiscellaneousSymbolsandArrows::DownwardsTriangleDashHeadedArrow),
            LEFT_RIGHT_TRIANGLE_DASH_HEADED_ARROW => Ok(MiscellaneousSymbolsandArrows::LeftRightTriangleDashHeadedArrow),
            UP_DOWN_TRIANGLE_DASH_HEADED_ARROW => Ok(MiscellaneousSymbolsandArrows::UpDownTriangleDashHeadedArrow),
            NORTH_WEST_TRIANGLE_DASH_HEADED_ARROW => Ok(MiscellaneousSymbolsandArrows::NorthWestTriangleDashHeadedArrow),
            NORTH_EAST_TRIANGLE_DASH_HEADED_ARROW => Ok(MiscellaneousSymbolsandArrows::NorthEastTriangleDashHeadedArrow),
            SOUTH_EAST_TRIANGLE_DASH_HEADED_ARROW => Ok(MiscellaneousSymbolsandArrows::SouthEastTriangleDashHeadedArrow),
            SOUTH_WEST_TRIANGLE_DASH_HEADED_ARROW => Ok(MiscellaneousSymbolsandArrows::SouthWestTriangleDashHeadedArrow),
            LEFTWARDS_TRIANGLE_DASH_HEADED_DASHED_ARROW => Ok(MiscellaneousSymbolsandArrows::LeftwardsTriangleDashHeadedDashedArrow),
            UPWARDS_TRIANGLE_DASH_HEADED_DASHED_ARROW => Ok(MiscellaneousSymbolsandArrows::UpwardsTriangleDashHeadedDashedArrow),
            RIGHTWARDS_TRIANGLE_DASH_HEADED_DASHED_ARROW => Ok(MiscellaneousSymbolsandArrows::RightwardsTriangleDashHeadedDashedArrow),
            DOWNWARDS_TRIANGLE_DASH_HEADED_DASHED_ARROW => Ok(MiscellaneousSymbolsandArrows::DownwardsTriangleDashHeadedDashedArrow),
            CLOCKWISE_TRIANGLE_DASH_HEADED_OPEN_CIRCLE_ARROW => Ok(MiscellaneousSymbolsandArrows::ClockwiseTriangleDashHeadedOpenCircleArrow),
            ANTICLOCKWISE_TRIANGLE_DASH_HEADED_OPEN_CIRCLE_ARROW => Ok(MiscellaneousSymbolsandArrows::AnticlockwiseTriangleDashHeadedOpenCircleArrow),
            LEFTWARDS_TRIANGLE_DASH_HEADED_ARROW_TO_BAR => Ok(MiscellaneousSymbolsandArrows::LeftwardsTriangleDashHeadedArrowToBar),
            UPWARDS_TRIANGLE_DASH_HEADED_ARROW_TO_BAR => Ok(MiscellaneousSymbolsandArrows::UpwardsTriangleDashHeadedArrowToBar),
            RIGHTWARDS_TRIANGLE_DASH_HEADED_ARROW_TO_BAR => Ok(MiscellaneousSymbolsandArrows::RightwardsTriangleDashHeadedArrowToBar),
            DOWNWARDS_TRIANGLE_DASH_HEADED_ARROW_TO_BAR => Ok(MiscellaneousSymbolsandArrows::DownwardsTriangleDashHeadedArrowToBar),
            NORTH_WEST_TRIANGLE_DASH_HEADED_ARROW_TO_BAR => Ok(MiscellaneousSymbolsandArrows::NorthWestTriangleDashHeadedArrowToBar),
            NORTH_EAST_TRIANGLE_DASH_HEADED_ARROW_TO_BAR => Ok(MiscellaneousSymbolsandArrows::NorthEastTriangleDashHeadedArrowToBar),
            SOUTH_EAST_TRIANGLE_DASH_HEADED_ARROW_TO_BAR => Ok(MiscellaneousSymbolsandArrows::SouthEastTriangleDashHeadedArrowToBar),
            SOUTH_WEST_TRIANGLE_DASH_HEADED_ARROW_TO_BAR => Ok(MiscellaneousSymbolsandArrows::SouthWestTriangleDashHeadedArrowToBar),
            LEFTWARDS_TRIANGLE_DASH_HEADED_ARROW_WITH_DOUBLE_HORIZONTAL_STROKE => Ok(MiscellaneousSymbolsandArrows::LeftwardsTriangleDashHeadedArrowWithDoubleHorizontalStroke),
            UPWARDS_TRIANGLE_DASH_HEADED_ARROW_WITH_DOUBLE_HORIZONTAL_STROKE => Ok(MiscellaneousSymbolsandArrows::UpwardsTriangleDashHeadedArrowWithDoubleHorizontalStroke),
            RIGHTWARDS_TRIANGLE_DASH_HEADED_ARROW_WITH_DOUBLE_HORIZONTAL_STROKE => Ok(MiscellaneousSymbolsandArrows::RightwardsTriangleDashHeadedArrowWithDoubleHorizontalStroke),
            DOWNWARDS_TRIANGLE_DASH_HEADED_ARROW_WITH_DOUBLE_HORIZONTAL_STROKE => Ok(MiscellaneousSymbolsandArrows::DownwardsTriangleDashHeadedArrowWithDoubleHorizontalStroke),
            HORIZONTAL_TAB_KEY => Ok(MiscellaneousSymbolsandArrows::HorizontalTabKey),
            VERTICAL_TAB_KEY => Ok(MiscellaneousSymbolsandArrows::VerticalTabKey),
            LEFTWARDS_TRIANGLE_DASH_HEADED_ARROW_OVER_RIGHTWARDS_TRIANGLE_DASH_HEADED_ARROW => Ok(MiscellaneousSymbolsandArrows::LeftwardsTriangleDashHeadedArrowOverRightwardsTriangleDashHeadedArrow),
            UPWARDS_TRIANGLE_DASH_HEADED_ARROW_LEFTWARDS_OF_DOWNWARDS_TRIANGLE_DASH_HEADED_ARROW => Ok(MiscellaneousSymbolsandArrows::UpwardsTriangleDashHeadedArrowLeftwardsOfDownwardsTriangleDashHeadedArrow),
            RIGHTWARDS_TRIANGLE_DASH_HEADED_ARROW_OVER_LEFTWARDS_TRIANGLE_DASH_HEADED_ARROW => Ok(MiscellaneousSymbolsandArrows::RightwardsTriangleDashHeadedArrowOverLeftwardsTriangleDashHeadedArrow),
            DOWNWARDS_TRIANGLE_DASH_HEADED_ARROW_LEFTWARDS_OF_UPWARDS_TRIANGLE_DASH_HEADED_ARROW => Ok(MiscellaneousSymbolsandArrows::DownwardsTriangleDashHeadedArrowLeftwardsOfUpwardsTriangleDashHeadedArrow),
            LEFTWARDS_TRIANGLE_DASH_HEADED_PAIRED_ARROWS => Ok(MiscellaneousSymbolsandArrows::LeftwardsTriangleDashHeadedPairedArrows),
            UPWARDS_TRIANGLE_DASH_HEADED_PAIRED_ARROWS => Ok(MiscellaneousSymbolsandArrows::UpwardsTriangleDashHeadedPairedArrows),
            RIGHTWARDS_TRIANGLE_DASH_HEADED_PAIRED_ARROWS => Ok(MiscellaneousSymbolsandArrows::RightwardsTriangleDashHeadedPairedArrows),
            DOWNWARDS_TRIANGLE_DASH_HEADED_PAIRED_ARROWS => Ok(MiscellaneousSymbolsandArrows::DownwardsTriangleDashHeadedPairedArrows),
            LEFTWARDS_BLACK_CIRCLED_WHITE_ARROW => Ok(MiscellaneousSymbolsandArrows::LeftwardsBlackCircledWhiteArrow),
            UPWARDS_BLACK_CIRCLED_WHITE_ARROW => Ok(MiscellaneousSymbolsandArrows::UpwardsBlackCircledWhiteArrow),
            RIGHTWARDS_BLACK_CIRCLED_WHITE_ARROW => Ok(MiscellaneousSymbolsandArrows::RightwardsBlackCircledWhiteArrow),
            DOWNWARDS_BLACK_CIRCLED_WHITE_ARROW => Ok(MiscellaneousSymbolsandArrows::DownwardsBlackCircledWhiteArrow),
            ANTICLOCKWISE_TRIANGLE_DASH_HEADED_RIGHT_U_DASH_SHAPED_ARROW => Ok(MiscellaneousSymbolsandArrows::AnticlockwiseTriangleDashHeadedRightUDashShapedArrow),
            ANTICLOCKWISE_TRIANGLE_DASH_HEADED_BOTTOM_U_DASH_SHAPED_ARROW => Ok(MiscellaneousSymbolsandArrows::AnticlockwiseTriangleDashHeadedBottomUDashShapedArrow),
            ANTICLOCKWISE_TRIANGLE_DASH_HEADED_LEFT_U_DASH_SHAPED_ARROW => Ok(MiscellaneousSymbolsandArrows::AnticlockwiseTriangleDashHeadedLeftUDashShapedArrow),
            ANTICLOCKWISE_TRIANGLE_DASH_HEADED_TOP_U_DASH_SHAPED_ARROW => Ok(MiscellaneousSymbolsandArrows::AnticlockwiseTriangleDashHeadedTopUDashShapedArrow),
            RETURN_LEFT => Ok(MiscellaneousSymbolsandArrows::ReturnLeft),
            RETURN_RIGHT => Ok(MiscellaneousSymbolsandArrows::ReturnRight),
            NEWLINE_LEFT => Ok(MiscellaneousSymbolsandArrows::NewlineLeft),
            NEWLINE_RIGHT => Ok(MiscellaneousSymbolsandArrows::NewlineRight),
            FOUR_CORNER_ARROWS_CIRCLING_ANTICLOCKWISE => Ok(MiscellaneousSymbolsandArrows::FourCornerArrowsCirclingAnticlockwise),
            RIGHTWARDS_BLACK_ARROW => Ok(MiscellaneousSymbolsandArrows::RightwardsBlackArrow),
            THREE_DASH_D_TOP_DASH_LIGHTED_LEFTWARDS_EQUILATERAL_ARROWHEAD => Ok(MiscellaneousSymbolsandArrows::ThreeDashDTopDashLightedLeftwardsEquilateralArrowhead),
            THREE_DASH_D_RIGHT_DASH_LIGHTED_UPWARDS_EQUILATERAL_ARROWHEAD => Ok(MiscellaneousSymbolsandArrows::ThreeDashDRightDashLightedUpwardsEquilateralArrowhead),
            THREE_DASH_D_TOP_DASH_LIGHTED_RIGHTWARDS_EQUILATERAL_ARROWHEAD => Ok(MiscellaneousSymbolsandArrows::ThreeDashDTopDashLightedRightwardsEquilateralArrowhead),
            THREE_DASH_D_LEFT_DASH_LIGHTED_DOWNWARDS_EQUILATERAL_ARROWHEAD => Ok(MiscellaneousSymbolsandArrows::ThreeDashDLeftDashLightedDownwardsEquilateralArrowhead),
            BLACK_LEFTWARDS_EQUILATERAL_ARROWHEAD => Ok(MiscellaneousSymbolsandArrows::BlackLeftwardsEquilateralArrowhead),
            BLACK_UPWARDS_EQUILATERAL_ARROWHEAD => Ok(MiscellaneousSymbolsandArrows::BlackUpwardsEquilateralArrowhead),
            BLACK_RIGHTWARDS_EQUILATERAL_ARROWHEAD => Ok(MiscellaneousSymbolsandArrows::BlackRightwardsEquilateralArrowhead),
            BLACK_DOWNWARDS_EQUILATERAL_ARROWHEAD => Ok(MiscellaneousSymbolsandArrows::BlackDownwardsEquilateralArrowhead),
            DOWNWARDS_TRIANGLE_DASH_HEADED_ARROW_WITH_LONG_TIP_LEFTWARDS => Ok(MiscellaneousSymbolsandArrows::DownwardsTriangleDashHeadedArrowWithLongTipLeftwards),
            DOWNWARDS_TRIANGLE_DASH_HEADED_ARROW_WITH_LONG_TIP_RIGHTWARDS => Ok(MiscellaneousSymbolsandArrows::DownwardsTriangleDashHeadedArrowWithLongTipRightwards),
            UPWARDS_TRIANGLE_DASH_HEADED_ARROW_WITH_LONG_TIP_LEFTWARDS => Ok(MiscellaneousSymbolsandArrows::UpwardsTriangleDashHeadedArrowWithLongTipLeftwards),
            UPWARDS_TRIANGLE_DASH_HEADED_ARROW_WITH_LONG_TIP_RIGHTWARDS => Ok(MiscellaneousSymbolsandArrows::UpwardsTriangleDashHeadedArrowWithLongTipRightwards),
            LEFTWARDS_TRIANGLE_DASH_HEADED_ARROW_WITH_LONG_TIP_UPWARDS => Ok(MiscellaneousSymbolsandArrows::LeftwardsTriangleDashHeadedArrowWithLongTipUpwards),
            RIGHTWARDS_TRIANGLE_DASH_HEADED_ARROW_WITH_LONG_TIP_UPWARDS => Ok(MiscellaneousSymbolsandArrows::RightwardsTriangleDashHeadedArrowWithLongTipUpwards),
            LEFTWARDS_TRIANGLE_DASH_HEADED_ARROW_WITH_LONG_TIP_DOWNWARDS => Ok(MiscellaneousSymbolsandArrows::LeftwardsTriangleDashHeadedArrowWithLongTipDownwards),
            RIGHTWARDS_TRIANGLE_DASH_HEADED_ARROW_WITH_LONG_TIP_DOWNWARDS => Ok(MiscellaneousSymbolsandArrows::RightwardsTriangleDashHeadedArrowWithLongTipDownwards),
            BLACK_CURVED_DOWNWARDS_AND_LEFTWARDS_ARROW => Ok(MiscellaneousSymbolsandArrows::BlackCurvedDownwardsAndLeftwardsArrow),
            BLACK_CURVED_DOWNWARDS_AND_RIGHTWARDS_ARROW => Ok(MiscellaneousSymbolsandArrows::BlackCurvedDownwardsAndRightwardsArrow),
            BLACK_CURVED_UPWARDS_AND_LEFTWARDS_ARROW => Ok(MiscellaneousSymbolsandArrows::BlackCurvedUpwardsAndLeftwardsArrow),
            BLACK_CURVED_UPWARDS_AND_RIGHTWARDS_ARROW => Ok(MiscellaneousSymbolsandArrows::BlackCurvedUpwardsAndRightwardsArrow),
            BLACK_CURVED_LEFTWARDS_AND_UPWARDS_ARROW => Ok(MiscellaneousSymbolsandArrows::BlackCurvedLeftwardsAndUpwardsArrow),
            BLACK_CURVED_RIGHTWARDS_AND_UPWARDS_ARROW => Ok(MiscellaneousSymbolsandArrows::BlackCurvedRightwardsAndUpwardsArrow),
            BLACK_CURVED_LEFTWARDS_AND_DOWNWARDS_ARROW => Ok(MiscellaneousSymbolsandArrows::BlackCurvedLeftwardsAndDownwardsArrow),
            BLACK_CURVED_RIGHTWARDS_AND_DOWNWARDS_ARROW => Ok(MiscellaneousSymbolsandArrows::BlackCurvedRightwardsAndDownwardsArrow),
            RIBBON_ARROW_DOWN_LEFT => Ok(MiscellaneousSymbolsandArrows::RibbonArrowDownLeft),
            RIBBON_ARROW_DOWN_RIGHT => Ok(MiscellaneousSymbolsandArrows::RibbonArrowDownRight),
            RIBBON_ARROW_UP_LEFT => Ok(MiscellaneousSymbolsandArrows::RibbonArrowUpLeft),
            RIBBON_ARROW_UP_RIGHT => Ok(MiscellaneousSymbolsandArrows::RibbonArrowUpRight),
            RIBBON_ARROW_LEFT_UP => Ok(MiscellaneousSymbolsandArrows::RibbonArrowLeftUp),
            RIBBON_ARROW_RIGHT_UP => Ok(MiscellaneousSymbolsandArrows::RibbonArrowRightUp),
            RIBBON_ARROW_LEFT_DOWN => Ok(MiscellaneousSymbolsandArrows::RibbonArrowLeftDown),
            RIBBON_ARROW_RIGHT_DOWN => Ok(MiscellaneousSymbolsandArrows::RibbonArrowRightDown),
            UPWARDS_WHITE_ARROW_FROM_BAR_WITH_HORIZONTAL_BAR => Ok(MiscellaneousSymbolsandArrows::UpwardsWhiteArrowFromBarWithHorizontalBar),
            UP_ARROWHEAD_IN_A_RECTANGLE_BOX => Ok(MiscellaneousSymbolsandArrows::UpArrowheadInARectangleBox),
            OVERLAPPING_WHITE_SQUARES => Ok(MiscellaneousSymbolsandArrows::OverlappingWhiteSquares),
            OVERLAPPING_WHITE_AND_BLACK_SQUARES => Ok(MiscellaneousSymbolsandArrows::OverlappingWhiteAndBlackSquares),
            OVERLAPPING_BLACK_SQUARES => Ok(MiscellaneousSymbolsandArrows::OverlappingBlackSquares),
            BALLOT_BOX_WITH_LIGHT_X => Ok(MiscellaneousSymbolsandArrows::BallotBoxWithLightX),
            CIRCLED_X => Ok(MiscellaneousSymbolsandArrows::CircledX),
            CIRCLED_BOLD_X => Ok(MiscellaneousSymbolsandArrows::CircledBoldX),
            BLACK_SQUARE_CENTRED => Ok(MiscellaneousSymbolsandArrows::BlackSquareCentred),
            BLACK_DIAMOND_CENTRED => Ok(MiscellaneousSymbolsandArrows::BlackDiamondCentred),
            TURNED_BLACK_PENTAGON => Ok(MiscellaneousSymbolsandArrows::TurnedBlackPentagon),
            HORIZONTAL_BLACK_OCTAGON => Ok(MiscellaneousSymbolsandArrows::HorizontalBlackOctagon),
            BLACK_OCTAGON => Ok(MiscellaneousSymbolsandArrows::BlackOctagon),
            BLACK_MEDIUM_UP_DASH_POINTING_TRIANGLE_CENTRED => Ok(MiscellaneousSymbolsandArrows::BlackMediumUpDashPointingTriangleCentred),
            BLACK_MEDIUM_DOWN_DASH_POINTING_TRIANGLE_CENTRED => Ok(MiscellaneousSymbolsandArrows::BlackMediumDownDashPointingTriangleCentred),
            BLACK_MEDIUM_LEFT_DASH_POINTING_TRIANGLE_CENTRED => Ok(MiscellaneousSymbolsandArrows::BlackMediumLeftDashPointingTriangleCentred),
            BLACK_MEDIUM_RIGHT_DASH_POINTING_TRIANGLE_CENTRED => Ok(MiscellaneousSymbolsandArrows::BlackMediumRightDashPointingTriangleCentred),
            NEPTUNE_FORM_TWO => Ok(MiscellaneousSymbolsandArrows::NeptuneFormTwo),
            TOP_HALF_BLACK_CIRCLE => Ok(MiscellaneousSymbolsandArrows::TopHalfBlackCircle),
            BOTTOM_HALF_BLACK_CIRCLE => Ok(MiscellaneousSymbolsandArrows::BottomHalfBlackCircle),
            LIGHT_FOUR_POINTED_BLACK_CUSP => Ok(MiscellaneousSymbolsandArrows::LightFourPointedBlackCusp),
            ROTATED_LIGHT_FOUR_POINTED_BLACK_CUSP => Ok(MiscellaneousSymbolsandArrows::RotatedLightFourPointedBlackCusp),
            WHITE_FOUR_POINTED_CUSP => Ok(MiscellaneousSymbolsandArrows::WhiteFourPointedCusp),
            ROTATED_WHITE_FOUR_POINTED_CUSP => Ok(MiscellaneousSymbolsandArrows::RotatedWhiteFourPointedCusp),
            SQUARE_POSITION_INDICATOR => Ok(MiscellaneousSymbolsandArrows::SquarePositionIndicator),
            UNCERTAINTY_SIGN => Ok(MiscellaneousSymbolsandArrows::UncertaintySign),
            GROUP_MARK => Ok(MiscellaneousSymbolsandArrows::GroupMark),
            PLUTO_FORM_TWO => Ok(MiscellaneousSymbolsandArrows::PlutoFormTwo),
            PLUTO_FORM_THREE => Ok(MiscellaneousSymbolsandArrows::PlutoFormThree),
            PLUTO_FORM_FOUR => Ok(MiscellaneousSymbolsandArrows::PlutoFormFour),
            PLUTO_FORM_FIVE => Ok(MiscellaneousSymbolsandArrows::PlutoFormFive),
            TRANSPLUTO => Ok(MiscellaneousSymbolsandArrows::Transpluto),
            PROSERPINA => Ok(MiscellaneousSymbolsandArrows::Proserpina),
            ASTRAEA => Ok(MiscellaneousSymbolsandArrows::Astraea),
            HYGIEA => Ok(MiscellaneousSymbolsandArrows::Hygiea),
            PHOLUS => Ok(MiscellaneousSymbolsandArrows::Pholus),
            NESSUS => Ok(MiscellaneousSymbolsandArrows::Nessus),
            WHITE_MOON_SELENA => Ok(MiscellaneousSymbolsandArrows::WhiteMoonSelena),
            BLACK_DIAMOND_ON_CROSS => Ok(MiscellaneousSymbolsandArrows::BlackDiamondOnCross),
            TRUE_LIGHT_MOON_ARTA => Ok(MiscellaneousSymbolsandArrows::TrueLightMoonArta),
            CUPIDO => Ok(MiscellaneousSymbolsandArrows::Cupido),
            HADES => Ok(MiscellaneousSymbolsandArrows::Hades),
            ZEUS => Ok(MiscellaneousSymbolsandArrows::Zeus),
            KRONOS => Ok(MiscellaneousSymbolsandArrows::Kronos),
            APOLLON => Ok(MiscellaneousSymbolsandArrows::Apollon),
            ADMETOS => Ok(MiscellaneousSymbolsandArrows::Admetos),
            VULCANUS => Ok(MiscellaneousSymbolsandArrows::Vulcanus),
            POSEIDON => Ok(MiscellaneousSymbolsandArrows::Poseidon),
            LEFT_HALF_BLACK_STAR => Ok(MiscellaneousSymbolsandArrows::LeftHalfBlackStar),
            RIGHT_HALF_BLACK_STAR => Ok(MiscellaneousSymbolsandArrows::RightHalfBlackStar),
            STAR_WITH_LEFT_HALF_BLACK => Ok(MiscellaneousSymbolsandArrows::StarWithLeftHalfBlack),
            STAR_WITH_RIGHT_HALF_BLACK => Ok(MiscellaneousSymbolsandArrows::StarWithRightHalfBlack),
            LEFTWARDS_TWO_DASH_HEADED_ARROW_WITH_TRIANGLE_ARROWHEADS => Ok(MiscellaneousSymbolsandArrows::LeftwardsTwoDashHeadedArrowWithTriangleArrowheads),
            UPWARDS_TWO_DASH_HEADED_ARROW_WITH_TRIANGLE_ARROWHEADS => Ok(MiscellaneousSymbolsandArrows::UpwardsTwoDashHeadedArrowWithTriangleArrowheads),
            RIGHTWARDS_TWO_DASH_HEADED_ARROW_WITH_TRIANGLE_ARROWHEADS => Ok(MiscellaneousSymbolsandArrows::RightwardsTwoDashHeadedArrowWithTriangleArrowheads),
            DOWNWARDS_TWO_DASH_HEADED_ARROW_WITH_TRIANGLE_ARROWHEADS => Ok(MiscellaneousSymbolsandArrows::DownwardsTwoDashHeadedArrowWithTriangleArrowheads),
            ERIS_FORM_ONE => Ok(MiscellaneousSymbolsandArrows::ErisFormOne),
            ERIS_FORM_TWO => Ok(MiscellaneousSymbolsandArrows::ErisFormTwo),
            SEDNA => Ok(MiscellaneousSymbolsandArrows::Sedna),
            RUSSIAN_ASTROLOGICAL_SYMBOL_VIGINTILE => Ok(MiscellaneousSymbolsandArrows::RussianAstrologicalSymbolVigintile),
            RUSSIAN_ASTROLOGICAL_SYMBOL_NOVILE => Ok(MiscellaneousSymbolsandArrows::RussianAstrologicalSymbolNovile),
            RUSSIAN_ASTROLOGICAL_SYMBOL_QUINTILE => Ok(MiscellaneousSymbolsandArrows::RussianAstrologicalSymbolQuintile),
            RUSSIAN_ASTROLOGICAL_SYMBOL_BINOVILE => Ok(MiscellaneousSymbolsandArrows::RussianAstrologicalSymbolBinovile),
            RUSSIAN_ASTROLOGICAL_SYMBOL_SENTAGON => Ok(MiscellaneousSymbolsandArrows::RussianAstrologicalSymbolSentagon),
            RUSSIAN_ASTROLOGICAL_SYMBOL_TREDECILE => Ok(MiscellaneousSymbolsandArrows::RussianAstrologicalSymbolTredecile),
            EQUALS_SIGN_WITH_INFINITY_BELOW => Ok(MiscellaneousSymbolsandArrows::EqualsSignWithInfinityBelow),
            UNITED_SYMBOL => Ok(MiscellaneousSymbolsandArrows::UnitedSymbol),
            SEPARATED_SYMBOL => Ok(MiscellaneousSymbolsandArrows::SeparatedSymbol),
            DOUBLED_SYMBOL => Ok(MiscellaneousSymbolsandArrows::DoubledSymbol),
            PASSED_SYMBOL => Ok(MiscellaneousSymbolsandArrows::PassedSymbol),
            REVERSED_RIGHT_ANGLE => Ok(MiscellaneousSymbolsandArrows::ReversedRightAngle),
            _ => Err(()),
        }
    }
}

impl Into<u32> for MiscellaneousSymbolsandArrows {
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

impl std::convert::TryFrom<u32> for MiscellaneousSymbolsandArrows {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for MiscellaneousSymbolsandArrows {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl MiscellaneousSymbolsandArrows {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        MiscellaneousSymbolsandArrows::NorthEastWhiteArrow
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            MiscellaneousSymbolsandArrows::NorthEastWhiteArrow => "north east white arrow",
            MiscellaneousSymbolsandArrows::NorthWestWhiteArrow => "north west white arrow",
            MiscellaneousSymbolsandArrows::SouthEastWhiteArrow => "south east white arrow",
            MiscellaneousSymbolsandArrows::SouthWestWhiteArrow => "south west white arrow",
            MiscellaneousSymbolsandArrows::LeftRightWhiteArrow => "left right white arrow",
            MiscellaneousSymbolsandArrows::LeftwardsBlackArrow => "leftwards black arrow",
            MiscellaneousSymbolsandArrows::UpwardsBlackArrow => "upwards black arrow",
            MiscellaneousSymbolsandArrows::DownwardsBlackArrow => "downwards black arrow",
            MiscellaneousSymbolsandArrows::NorthEastBlackArrow => "north east black arrow",
            MiscellaneousSymbolsandArrows::NorthWestBlackArrow => "north west black arrow",
            MiscellaneousSymbolsandArrows::SouthEastBlackArrow => "south east black arrow",
            MiscellaneousSymbolsandArrows::SouthWestBlackArrow => "south west black arrow",
            MiscellaneousSymbolsandArrows::LeftRightBlackArrow => "left right black arrow",
            MiscellaneousSymbolsandArrows::UpDownBlackArrow => "up down black arrow",
            MiscellaneousSymbolsandArrows::RightwardsArrowWithTipDownwards => "rightwards arrow with tip downwards",
            MiscellaneousSymbolsandArrows::RightwardsArrowWithTipUpwards => "rightwards arrow with tip upwards",
            MiscellaneousSymbolsandArrows::LeftwardsArrowWithTipDownwards => "leftwards arrow with tip downwards",
            MiscellaneousSymbolsandArrows::LeftwardsArrowWithTipUpwards => "leftwards arrow with tip upwards",
            MiscellaneousSymbolsandArrows::SquareWithTopHalfBlack => "square with top half black",
            MiscellaneousSymbolsandArrows::SquareWithBottomHalfBlack => "square with bottom half black",
            MiscellaneousSymbolsandArrows::SquareWithUpperRightDiagonalHalfBlack => "square with upper right diagonal half black",
            MiscellaneousSymbolsandArrows::SquareWithLowerLeftDiagonalHalfBlack => "square with lower left diagonal half black",
            MiscellaneousSymbolsandArrows::DiamondWithLeftHalfBlack => "diamond with left half black",
            MiscellaneousSymbolsandArrows::DiamondWithRightHalfBlack => "diamond with right half black",
            MiscellaneousSymbolsandArrows::DiamondWithTopHalfBlack => "diamond with top half black",
            MiscellaneousSymbolsandArrows::DiamondWithBottomHalfBlack => "diamond with bottom half black",
            MiscellaneousSymbolsandArrows::DottedSquare => "dotted square",
            MiscellaneousSymbolsandArrows::BlackLargeSquare => "black large square",
            MiscellaneousSymbolsandArrows::WhiteLargeSquare => "white large square",
            MiscellaneousSymbolsandArrows::BlackVerySmallSquare => "black very small square",
            MiscellaneousSymbolsandArrows::WhiteVerySmallSquare => "white very small square",
            MiscellaneousSymbolsandArrows::BlackPentagon => "black pentagon",
            MiscellaneousSymbolsandArrows::WhitePentagon => "white pentagon",
            MiscellaneousSymbolsandArrows::WhiteHexagon => "white hexagon",
            MiscellaneousSymbolsandArrows::BlackHexagon => "black hexagon",
            MiscellaneousSymbolsandArrows::HorizontalBlackHexagon => "horizontal black hexagon",
            MiscellaneousSymbolsandArrows::BlackLargeCircle => "black large circle",
            MiscellaneousSymbolsandArrows::BlackMediumDiamond => "black medium diamond",
            MiscellaneousSymbolsandArrows::WhiteMediumDiamond => "white medium diamond",
            MiscellaneousSymbolsandArrows::BlackMediumLozenge => "black medium lozenge",
            MiscellaneousSymbolsandArrows::WhiteMediumLozenge => "white medium lozenge",
            MiscellaneousSymbolsandArrows::BlackSmallDiamond => "black small diamond",
            MiscellaneousSymbolsandArrows::BlackSmallLozenge => "black small lozenge",
            MiscellaneousSymbolsandArrows::WhiteSmallLozenge => "white small lozenge",
            MiscellaneousSymbolsandArrows::BlackHorizontalEllipse => "black horizontal ellipse",
            MiscellaneousSymbolsandArrows::WhiteHorizontalEllipse => "white horizontal ellipse",
            MiscellaneousSymbolsandArrows::BlackVerticalEllipse => "black vertical ellipse",
            MiscellaneousSymbolsandArrows::WhiteVerticalEllipse => "white vertical ellipse",
            MiscellaneousSymbolsandArrows::LeftArrowWithSmallCircle => "left arrow with small circle",
            MiscellaneousSymbolsandArrows::ThreeLeftwardsArrows => "three leftwards arrows",
            MiscellaneousSymbolsandArrows::LeftArrowWithCircledPlus => "left arrow with circled plus",
            MiscellaneousSymbolsandArrows::LongLeftwardsSquiggleArrow => "long leftwards squiggle arrow",
            MiscellaneousSymbolsandArrows::LeftwardsTwoDashHeadedArrowWithVerticalStroke => "leftwards two-headed arrow with vertical stroke",
            MiscellaneousSymbolsandArrows::LeftwardsTwoDashHeadedArrowWithDoubleVerticalStroke => "leftwards two-headed arrow with double vertical stroke",
            MiscellaneousSymbolsandArrows::LeftwardsTwoDashHeadedArrowFromBar => "leftwards two-headed arrow from bar",
            MiscellaneousSymbolsandArrows::LeftwardsTwoDashHeadedTripleDashArrow => "leftwards two-headed triple dash arrow",
            MiscellaneousSymbolsandArrows::LeftwardsArrowWithDottedStem => "leftwards arrow with dotted stem",
            MiscellaneousSymbolsandArrows::LeftwardsArrowWithTailWithVerticalStroke => "leftwards arrow with tail with vertical stroke",
            MiscellaneousSymbolsandArrows::LeftwardsArrowWithTailWithDoubleVerticalStroke => "leftwards arrow with tail with double vertical stroke",
            MiscellaneousSymbolsandArrows::LeftwardsTwoDashHeadedArrowWithTail => "leftwards two-headed arrow with tail",
            MiscellaneousSymbolsandArrows::LeftwardsTwoDashHeadedArrowWithTailWithVerticalStroke => "leftwards two-headed arrow with tail with vertical stroke",
            MiscellaneousSymbolsandArrows::LeftwardsTwoDashHeadedArrowWithTailWithDoubleVerticalStroke => "leftwards two-headed arrow with tail with double vertical stroke",
            MiscellaneousSymbolsandArrows::LeftwardsArrowThroughX => "leftwards arrow through x",
            MiscellaneousSymbolsandArrows::WaveArrowPointingDirectlyLeft => "wave arrow pointing directly left",
            MiscellaneousSymbolsandArrows::EqualsSignAboveLeftwardsArrow => "equals sign above leftwards arrow",
            MiscellaneousSymbolsandArrows::ReverseTildeOperatorAboveLeftwardsArrow => "reverse tilde operator above leftwards arrow",
            MiscellaneousSymbolsandArrows::LeftwardsArrowAboveReverseAlmostEqualTo => "leftwards arrow above reverse almost equal to",
            MiscellaneousSymbolsandArrows::RightwardsArrowThroughGreaterDashThan => "rightwards arrow through greater-than",
            MiscellaneousSymbolsandArrows::RightwardsArrowThroughSuperset => "rightwards arrow through superset",
            MiscellaneousSymbolsandArrows::LeftwardsQuadrupleArrow => "leftwards quadruple arrow",
            MiscellaneousSymbolsandArrows::RightwardsQuadrupleArrow => "rightwards quadruple arrow",
            MiscellaneousSymbolsandArrows::ReverseTildeOperatorAboveRightwardsArrow => "reverse tilde operator above rightwards arrow",
            MiscellaneousSymbolsandArrows::RightwardsArrowAboveReverseAlmostEqualTo => "rightwards arrow above reverse almost equal to",
            MiscellaneousSymbolsandArrows::TildeOperatorAboveLeftwardsArrow => "tilde operator above leftwards arrow",
            MiscellaneousSymbolsandArrows::LeftwardsArrowAboveAlmostEqualTo => "leftwards arrow above almost equal to",
            MiscellaneousSymbolsandArrows::LeftwardsArrowAboveReverseTildeOperator => "leftwards arrow above reverse tilde operator",
            MiscellaneousSymbolsandArrows::RightwardsArrowAboveReverseTildeOperator => "rightwards arrow above reverse tilde operator",
            MiscellaneousSymbolsandArrows::DownwardsTriangleDashHeadedZigzagArrow => "downwards triangle-headed zigzag arrow",
            MiscellaneousSymbolsandArrows::ShortSlantedNorthArrow => "short slanted north arrow",
            MiscellaneousSymbolsandArrows::ShortBackslantedSouthArrow => "short backslanted south arrow",
            MiscellaneousSymbolsandArrows::WhiteMediumStar => "white medium star",
            MiscellaneousSymbolsandArrows::BlackSmallStar => "black small star",
            MiscellaneousSymbolsandArrows::WhiteSmallStar => "white small star",
            MiscellaneousSymbolsandArrows::BlackRightDashPointingPentagon => "black right-pointing pentagon",
            MiscellaneousSymbolsandArrows::WhiteRightDashPointingPentagon => "white right-pointing pentagon",
            MiscellaneousSymbolsandArrows::HeavyLargeCircle => "heavy large circle",
            MiscellaneousSymbolsandArrows::HeavyOvalWithOvalInside => "heavy oval with oval inside",
            MiscellaneousSymbolsandArrows::HeavyCircleWithCircleInside => "heavy circle with circle inside",
            MiscellaneousSymbolsandArrows::HeavyCircle => "heavy circle",
            MiscellaneousSymbolsandArrows::HeavyCircledSaltire => "heavy circled saltire",
            MiscellaneousSymbolsandArrows::SlantedNorthArrowWithHookedHead => "slanted north arrow with hooked head",
            MiscellaneousSymbolsandArrows::BackslantedSouthArrowWithHookedTail => "backslanted south arrow with hooked tail",
            MiscellaneousSymbolsandArrows::SlantedNorthArrowWithHorizontalTail => "slanted north arrow with horizontal tail",
            MiscellaneousSymbolsandArrows::BackslantedSouthArrowWithHorizontalTail => "backslanted south arrow with horizontal tail",
            MiscellaneousSymbolsandArrows::BentArrowPointingDownwardsThenNorthEast => "bent arrow pointing downwards then north east",
            MiscellaneousSymbolsandArrows::ShortBentArrowPointingDownwardsThenNorthEast => "short bent arrow pointing downwards then north east",
            MiscellaneousSymbolsandArrows::LeftwardsTriangleDashHeadedArrow => "leftwards triangle-headed arrow",
            MiscellaneousSymbolsandArrows::UpwardsTriangleDashHeadedArrow => "upwards triangle-headed arrow",
            MiscellaneousSymbolsandArrows::RightwardsTriangleDashHeadedArrow => "rightwards triangle-headed arrow",
            MiscellaneousSymbolsandArrows::DownwardsTriangleDashHeadedArrow => "downwards triangle-headed arrow",
            MiscellaneousSymbolsandArrows::LeftRightTriangleDashHeadedArrow => "left right triangle-headed arrow",
            MiscellaneousSymbolsandArrows::UpDownTriangleDashHeadedArrow => "up down triangle-headed arrow",
            MiscellaneousSymbolsandArrows::NorthWestTriangleDashHeadedArrow => "north west triangle-headed arrow",
            MiscellaneousSymbolsandArrows::NorthEastTriangleDashHeadedArrow => "north east triangle-headed arrow",
            MiscellaneousSymbolsandArrows::SouthEastTriangleDashHeadedArrow => "south east triangle-headed arrow",
            MiscellaneousSymbolsandArrows::SouthWestTriangleDashHeadedArrow => "south west triangle-headed arrow",
            MiscellaneousSymbolsandArrows::LeftwardsTriangleDashHeadedDashedArrow => "leftwards triangle-headed dashed arrow",
            MiscellaneousSymbolsandArrows::UpwardsTriangleDashHeadedDashedArrow => "upwards triangle-headed dashed arrow",
            MiscellaneousSymbolsandArrows::RightwardsTriangleDashHeadedDashedArrow => "rightwards triangle-headed dashed arrow",
            MiscellaneousSymbolsandArrows::DownwardsTriangleDashHeadedDashedArrow => "downwards triangle-headed dashed arrow",
            MiscellaneousSymbolsandArrows::ClockwiseTriangleDashHeadedOpenCircleArrow => "clockwise triangle-headed open circle arrow",
            MiscellaneousSymbolsandArrows::AnticlockwiseTriangleDashHeadedOpenCircleArrow => "anticlockwise triangle-headed open circle arrow",
            MiscellaneousSymbolsandArrows::LeftwardsTriangleDashHeadedArrowToBar => "leftwards triangle-headed arrow to bar",
            MiscellaneousSymbolsandArrows::UpwardsTriangleDashHeadedArrowToBar => "upwards triangle-headed arrow to bar",
            MiscellaneousSymbolsandArrows::RightwardsTriangleDashHeadedArrowToBar => "rightwards triangle-headed arrow to bar",
            MiscellaneousSymbolsandArrows::DownwardsTriangleDashHeadedArrowToBar => "downwards triangle-headed arrow to bar",
            MiscellaneousSymbolsandArrows::NorthWestTriangleDashHeadedArrowToBar => "north west triangle-headed arrow to bar",
            MiscellaneousSymbolsandArrows::NorthEastTriangleDashHeadedArrowToBar => "north east triangle-headed arrow to bar",
            MiscellaneousSymbolsandArrows::SouthEastTriangleDashHeadedArrowToBar => "south east triangle-headed arrow to bar",
            MiscellaneousSymbolsandArrows::SouthWestTriangleDashHeadedArrowToBar => "south west triangle-headed arrow to bar",
            MiscellaneousSymbolsandArrows::LeftwardsTriangleDashHeadedArrowWithDoubleHorizontalStroke => "leftwards triangle-headed arrow with double horizontal stroke",
            MiscellaneousSymbolsandArrows::UpwardsTriangleDashHeadedArrowWithDoubleHorizontalStroke => "upwards triangle-headed arrow with double horizontal stroke",
            MiscellaneousSymbolsandArrows::RightwardsTriangleDashHeadedArrowWithDoubleHorizontalStroke => "rightwards triangle-headed arrow with double horizontal stroke",
            MiscellaneousSymbolsandArrows::DownwardsTriangleDashHeadedArrowWithDoubleHorizontalStroke => "downwards triangle-headed arrow with double horizontal stroke",
            MiscellaneousSymbolsandArrows::HorizontalTabKey => "horizontal tab key",
            MiscellaneousSymbolsandArrows::VerticalTabKey => "vertical tab key",
            MiscellaneousSymbolsandArrows::LeftwardsTriangleDashHeadedArrowOverRightwardsTriangleDashHeadedArrow => "leftwards triangle-headed arrow over rightwards triangle-headed arrow",
            MiscellaneousSymbolsandArrows::UpwardsTriangleDashHeadedArrowLeftwardsOfDownwardsTriangleDashHeadedArrow => "upwards triangle-headed arrow leftwards of downwards triangle-headed arrow",
            MiscellaneousSymbolsandArrows::RightwardsTriangleDashHeadedArrowOverLeftwardsTriangleDashHeadedArrow => "rightwards triangle-headed arrow over leftwards triangle-headed arrow",
            MiscellaneousSymbolsandArrows::DownwardsTriangleDashHeadedArrowLeftwardsOfUpwardsTriangleDashHeadedArrow => "downwards triangle-headed arrow leftwards of upwards triangle-headed arrow",
            MiscellaneousSymbolsandArrows::LeftwardsTriangleDashHeadedPairedArrows => "leftwards triangle-headed paired arrows",
            MiscellaneousSymbolsandArrows::UpwardsTriangleDashHeadedPairedArrows => "upwards triangle-headed paired arrows",
            MiscellaneousSymbolsandArrows::RightwardsTriangleDashHeadedPairedArrows => "rightwards triangle-headed paired arrows",
            MiscellaneousSymbolsandArrows::DownwardsTriangleDashHeadedPairedArrows => "downwards triangle-headed paired arrows",
            MiscellaneousSymbolsandArrows::LeftwardsBlackCircledWhiteArrow => "leftwards black circled white arrow",
            MiscellaneousSymbolsandArrows::UpwardsBlackCircledWhiteArrow => "upwards black circled white arrow",
            MiscellaneousSymbolsandArrows::RightwardsBlackCircledWhiteArrow => "rightwards black circled white arrow",
            MiscellaneousSymbolsandArrows::DownwardsBlackCircledWhiteArrow => "downwards black circled white arrow",
            MiscellaneousSymbolsandArrows::AnticlockwiseTriangleDashHeadedRightUDashShapedArrow => "anticlockwise triangle-headed right u-shaped arrow",
            MiscellaneousSymbolsandArrows::AnticlockwiseTriangleDashHeadedBottomUDashShapedArrow => "anticlockwise triangle-headed bottom u-shaped arrow",
            MiscellaneousSymbolsandArrows::AnticlockwiseTriangleDashHeadedLeftUDashShapedArrow => "anticlockwise triangle-headed left u-shaped arrow",
            MiscellaneousSymbolsandArrows::AnticlockwiseTriangleDashHeadedTopUDashShapedArrow => "anticlockwise triangle-headed top u-shaped arrow",
            MiscellaneousSymbolsandArrows::ReturnLeft => "return left",
            MiscellaneousSymbolsandArrows::ReturnRight => "return right",
            MiscellaneousSymbolsandArrows::NewlineLeft => "newline left",
            MiscellaneousSymbolsandArrows::NewlineRight => "newline right",
            MiscellaneousSymbolsandArrows::FourCornerArrowsCirclingAnticlockwise => "four corner arrows circling anticlockwise",
            MiscellaneousSymbolsandArrows::RightwardsBlackArrow => "rightwards black arrow",
            MiscellaneousSymbolsandArrows::ThreeDashDTopDashLightedLeftwardsEquilateralArrowhead => "three-d top-lighted leftwards equilateral arrowhead",
            MiscellaneousSymbolsandArrows::ThreeDashDRightDashLightedUpwardsEquilateralArrowhead => "three-d right-lighted upwards equilateral arrowhead",
            MiscellaneousSymbolsandArrows::ThreeDashDTopDashLightedRightwardsEquilateralArrowhead => "three-d top-lighted rightwards equilateral arrowhead",
            MiscellaneousSymbolsandArrows::ThreeDashDLeftDashLightedDownwardsEquilateralArrowhead => "three-d left-lighted downwards equilateral arrowhead",
            MiscellaneousSymbolsandArrows::BlackLeftwardsEquilateralArrowhead => "black leftwards equilateral arrowhead",
            MiscellaneousSymbolsandArrows::BlackUpwardsEquilateralArrowhead => "black upwards equilateral arrowhead",
            MiscellaneousSymbolsandArrows::BlackRightwardsEquilateralArrowhead => "black rightwards equilateral arrowhead",
            MiscellaneousSymbolsandArrows::BlackDownwardsEquilateralArrowhead => "black downwards equilateral arrowhead",
            MiscellaneousSymbolsandArrows::DownwardsTriangleDashHeadedArrowWithLongTipLeftwards => "downwards triangle-headed arrow with long tip leftwards",
            MiscellaneousSymbolsandArrows::DownwardsTriangleDashHeadedArrowWithLongTipRightwards => "downwards triangle-headed arrow with long tip rightwards",
            MiscellaneousSymbolsandArrows::UpwardsTriangleDashHeadedArrowWithLongTipLeftwards => "upwards triangle-headed arrow with long tip leftwards",
            MiscellaneousSymbolsandArrows::UpwardsTriangleDashHeadedArrowWithLongTipRightwards => "upwards triangle-headed arrow with long tip rightwards",
            MiscellaneousSymbolsandArrows::LeftwardsTriangleDashHeadedArrowWithLongTipUpwards => "leftwards triangle-headed arrow with long tip upwards",
            MiscellaneousSymbolsandArrows::RightwardsTriangleDashHeadedArrowWithLongTipUpwards => "rightwards triangle-headed arrow with long tip upwards",
            MiscellaneousSymbolsandArrows::LeftwardsTriangleDashHeadedArrowWithLongTipDownwards => "leftwards triangle-headed arrow with long tip downwards",
            MiscellaneousSymbolsandArrows::RightwardsTriangleDashHeadedArrowWithLongTipDownwards => "rightwards triangle-headed arrow with long tip downwards",
            MiscellaneousSymbolsandArrows::BlackCurvedDownwardsAndLeftwardsArrow => "black curved downwards and leftwards arrow",
            MiscellaneousSymbolsandArrows::BlackCurvedDownwardsAndRightwardsArrow => "black curved downwards and rightwards arrow",
            MiscellaneousSymbolsandArrows::BlackCurvedUpwardsAndLeftwardsArrow => "black curved upwards and leftwards arrow",
            MiscellaneousSymbolsandArrows::BlackCurvedUpwardsAndRightwardsArrow => "black curved upwards and rightwards arrow",
            MiscellaneousSymbolsandArrows::BlackCurvedLeftwardsAndUpwardsArrow => "black curved leftwards and upwards arrow",
            MiscellaneousSymbolsandArrows::BlackCurvedRightwardsAndUpwardsArrow => "black curved rightwards and upwards arrow",
            MiscellaneousSymbolsandArrows::BlackCurvedLeftwardsAndDownwardsArrow => "black curved leftwards and downwards arrow",
            MiscellaneousSymbolsandArrows::BlackCurvedRightwardsAndDownwardsArrow => "black curved rightwards and downwards arrow",
            MiscellaneousSymbolsandArrows::RibbonArrowDownLeft => "ribbon arrow down left",
            MiscellaneousSymbolsandArrows::RibbonArrowDownRight => "ribbon arrow down right",
            MiscellaneousSymbolsandArrows::RibbonArrowUpLeft => "ribbon arrow up left",
            MiscellaneousSymbolsandArrows::RibbonArrowUpRight => "ribbon arrow up right",
            MiscellaneousSymbolsandArrows::RibbonArrowLeftUp => "ribbon arrow left up",
            MiscellaneousSymbolsandArrows::RibbonArrowRightUp => "ribbon arrow right up",
            MiscellaneousSymbolsandArrows::RibbonArrowLeftDown => "ribbon arrow left down",
            MiscellaneousSymbolsandArrows::RibbonArrowRightDown => "ribbon arrow right down",
            MiscellaneousSymbolsandArrows::UpwardsWhiteArrowFromBarWithHorizontalBar => "upwards white arrow from bar with horizontal bar",
            MiscellaneousSymbolsandArrows::UpArrowheadInARectangleBox => "up arrowhead in a rectangle box",
            MiscellaneousSymbolsandArrows::OverlappingWhiteSquares => "overlapping white squares",
            MiscellaneousSymbolsandArrows::OverlappingWhiteAndBlackSquares => "overlapping white and black squares",
            MiscellaneousSymbolsandArrows::OverlappingBlackSquares => "overlapping black squares",
            MiscellaneousSymbolsandArrows::BallotBoxWithLightX => "ballot box with light x",
            MiscellaneousSymbolsandArrows::CircledX => "circled x",
            MiscellaneousSymbolsandArrows::CircledBoldX => "circled bold x",
            MiscellaneousSymbolsandArrows::BlackSquareCentred => "black square centred",
            MiscellaneousSymbolsandArrows::BlackDiamondCentred => "black diamond centred",
            MiscellaneousSymbolsandArrows::TurnedBlackPentagon => "turned black pentagon",
            MiscellaneousSymbolsandArrows::HorizontalBlackOctagon => "horizontal black octagon",
            MiscellaneousSymbolsandArrows::BlackOctagon => "black octagon",
            MiscellaneousSymbolsandArrows::BlackMediumUpDashPointingTriangleCentred => "black medium up-pointing triangle centred",
            MiscellaneousSymbolsandArrows::BlackMediumDownDashPointingTriangleCentred => "black medium down-pointing triangle centred",
            MiscellaneousSymbolsandArrows::BlackMediumLeftDashPointingTriangleCentred => "black medium left-pointing triangle centred",
            MiscellaneousSymbolsandArrows::BlackMediumRightDashPointingTriangleCentred => "black medium right-pointing triangle centred",
            MiscellaneousSymbolsandArrows::NeptuneFormTwo => "neptune form two",
            MiscellaneousSymbolsandArrows::TopHalfBlackCircle => "top half black circle",
            MiscellaneousSymbolsandArrows::BottomHalfBlackCircle => "bottom half black circle",
            MiscellaneousSymbolsandArrows::LightFourPointedBlackCusp => "light four pointed black cusp",
            MiscellaneousSymbolsandArrows::RotatedLightFourPointedBlackCusp => "rotated light four pointed black cusp",
            MiscellaneousSymbolsandArrows::WhiteFourPointedCusp => "white four pointed cusp",
            MiscellaneousSymbolsandArrows::RotatedWhiteFourPointedCusp => "rotated white four pointed cusp",
            MiscellaneousSymbolsandArrows::SquarePositionIndicator => "square position indicator",
            MiscellaneousSymbolsandArrows::UncertaintySign => "uncertainty sign",
            MiscellaneousSymbolsandArrows::GroupMark => "group mark",
            MiscellaneousSymbolsandArrows::PlutoFormTwo => "pluto form two",
            MiscellaneousSymbolsandArrows::PlutoFormThree => "pluto form three",
            MiscellaneousSymbolsandArrows::PlutoFormFour => "pluto form four",
            MiscellaneousSymbolsandArrows::PlutoFormFive => "pluto form five",
            MiscellaneousSymbolsandArrows::Transpluto => "transpluto",
            MiscellaneousSymbolsandArrows::Proserpina => "proserpina",
            MiscellaneousSymbolsandArrows::Astraea => "astraea",
            MiscellaneousSymbolsandArrows::Hygiea => "hygiea",
            MiscellaneousSymbolsandArrows::Pholus => "pholus",
            MiscellaneousSymbolsandArrows::Nessus => "nessus",
            MiscellaneousSymbolsandArrows::WhiteMoonSelena => "white moon selena",
            MiscellaneousSymbolsandArrows::BlackDiamondOnCross => "black diamond on cross",
            MiscellaneousSymbolsandArrows::TrueLightMoonArta => "true light moon arta",
            MiscellaneousSymbolsandArrows::Cupido => "cupido",
            MiscellaneousSymbolsandArrows::Hades => "hades",
            MiscellaneousSymbolsandArrows::Zeus => "zeus",
            MiscellaneousSymbolsandArrows::Kronos => "kronos",
            MiscellaneousSymbolsandArrows::Apollon => "apollon",
            MiscellaneousSymbolsandArrows::Admetos => "admetos",
            MiscellaneousSymbolsandArrows::Vulcanus => "vulcanus",
            MiscellaneousSymbolsandArrows::Poseidon => "poseidon",
            MiscellaneousSymbolsandArrows::LeftHalfBlackStar => "left half black star",
            MiscellaneousSymbolsandArrows::RightHalfBlackStar => "right half black star",
            MiscellaneousSymbolsandArrows::StarWithLeftHalfBlack => "star with left half black",
            MiscellaneousSymbolsandArrows::StarWithRightHalfBlack => "star with right half black",
            MiscellaneousSymbolsandArrows::LeftwardsTwoDashHeadedArrowWithTriangleArrowheads => "leftwards two-headed arrow with triangle arrowheads",
            MiscellaneousSymbolsandArrows::UpwardsTwoDashHeadedArrowWithTriangleArrowheads => "upwards two-headed arrow with triangle arrowheads",
            MiscellaneousSymbolsandArrows::RightwardsTwoDashHeadedArrowWithTriangleArrowheads => "rightwards two-headed arrow with triangle arrowheads",
            MiscellaneousSymbolsandArrows::DownwardsTwoDashHeadedArrowWithTriangleArrowheads => "downwards two-headed arrow with triangle arrowheads",
            MiscellaneousSymbolsandArrows::ErisFormOne => "eris form one",
            MiscellaneousSymbolsandArrows::ErisFormTwo => "eris form two",
            MiscellaneousSymbolsandArrows::Sedna => "sedna",
            MiscellaneousSymbolsandArrows::RussianAstrologicalSymbolVigintile => "russian astrological symbol vigintile",
            MiscellaneousSymbolsandArrows::RussianAstrologicalSymbolNovile => "russian astrological symbol novile",
            MiscellaneousSymbolsandArrows::RussianAstrologicalSymbolQuintile => "russian astrological symbol quintile",
            MiscellaneousSymbolsandArrows::RussianAstrologicalSymbolBinovile => "russian astrological symbol binovile",
            MiscellaneousSymbolsandArrows::RussianAstrologicalSymbolSentagon => "russian astrological symbol sentagon",
            MiscellaneousSymbolsandArrows::RussianAstrologicalSymbolTredecile => "russian astrological symbol tredecile",
            MiscellaneousSymbolsandArrows::EqualsSignWithInfinityBelow => "equals sign with infinity below",
            MiscellaneousSymbolsandArrows::UnitedSymbol => "united symbol",
            MiscellaneousSymbolsandArrows::SeparatedSymbol => "separated symbol",
            MiscellaneousSymbolsandArrows::DoubledSymbol => "doubled symbol",
            MiscellaneousSymbolsandArrows::PassedSymbol => "passed symbol",
            MiscellaneousSymbolsandArrows::ReversedRightAngle => "reversed right angle",
        }
    }
}
