/// \u{2700} → \u{27bf}
///
/// ✀ ✁ ✂ ✃ ✄ ✅ ✆ ✇ ✈ ✉ ✊ ✋ ✌ ✍ ✎ ✏\
/// ✐ ✑ ✒ ✓ ✔ ✕ ✖ ✗ ✘ ✙ ✚ ✛ ✜ ✝ ✞ ✟\
/// ✠ ✡ ✢ ✣ ✤ ✥ ✦ ✧ ✨ ✩ ✪ ✫ ✬ ✭ ✮ ✯\
/// ✰ ✱ ✲ ✳ ✴ ✵ ✶ ✷ ✸ ✹ ✺ ✻ ✼ ✽ ✾ ✿\
/// ❀ ❁ ❂ ❃ ❄ ❅ ❆ ❇ ❈ ❉ ❊ ❋ ❌ ❍ ❎ ❏\
/// ❐ ❑ ❒ ❓ ❔ ❕ ❖ ❗ ❘ ❙ ❚ ❛ ❜ ❝ ❞ ❟\
/// ❠ ❡ ❢ ❣ ❤ ❥ ❦ ❧ ❨ ❩ ❪ ❫ ❬ ❭ ❮ ❯\
/// ❰ ❱ ❲ ❳ ❴ ❵ ❶ ❷ ❸ ❹ ❺ ❻ ❼ ❽ ❾ ❿\
/// ➀ ➁ ➂ ➃ ➄ ➅ ➆ ➇ ➈ ➉ ➊ ➋ ➌ ➍ ➎ ➏\
/// ➐ ➑ ➒ ➓ ➔ ➕ ➖ ➗ ➘ ➙ ➚ ➛ ➜ ➝ ➞ ➟\
/// ➠ ➡ ➢ ➣ ➤ ➥ ➦ ➧ ➨ ➩ ➪ ➫ ➬ ➭ ➮ ➯\
/// ➰ ➱ ➲ ➳ ➴ ➵ ➶ ➷ ➸ ➹ ➺ ➻ ➼ ➽ ➾\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{2700}: '✀'
    pub const BLACK_SAFETY_SCISSORS: char = '✀';
    /// \u{2701}: '✁'
    pub const UPPER_BLADE_SCISSORS: char = '✁';
    /// \u{2702}: '✂'
    pub const BLACK_SCISSORS: char = '✂';
    /// \u{2703}: '✃'
    pub const LOWER_BLADE_SCISSORS: char = '✃';
    /// \u{2704}: '✄'
    pub const WHITE_SCISSORS: char = '✄';
    /// \u{2705}: '✅'
    pub const WHITE_HEAVY_CHECK_MARK: char = '✅';
    /// \u{2706}: '✆'
    pub const TELEPHONE_LOCATION_SIGN: char = '✆';
    /// \u{2707}: '✇'
    pub const TAPE_DRIVE: char = '✇';
    /// \u{2708}: '✈'
    pub const AIRPLANE: char = '✈';
    /// \u{2709}: '✉'
    pub const ENVELOPE: char = '✉';
    /// \u{270a}: '✊'
    pub const RAISED_FIST: char = '✊';
    /// \u{270b}: '✋'
    pub const RAISED_HAND: char = '✋';
    /// \u{270c}: '✌'
    pub const VICTORY_HAND: char = '✌';
    /// \u{270d}: '✍'
    pub const WRITING_HAND: char = '✍';
    /// \u{270e}: '✎'
    pub const LOWER_RIGHT_PENCIL: char = '✎';
    /// \u{270f}: '✏'
    pub const PENCIL: char = '✏';
    /// \u{2710}: '✐'
    pub const UPPER_RIGHT_PENCIL: char = '✐';
    /// \u{2711}: '✑'
    pub const WHITE_NIB: char = '✑';
    /// \u{2712}: '✒'
    pub const BLACK_NIB: char = '✒';
    /// \u{2713}: '✓'
    pub const CHECK_MARK: char = '✓';
    /// \u{2714}: '✔'
    pub const HEAVY_CHECK_MARK: char = '✔';
    /// \u{2715}: '✕'
    pub const MULTIPLICATION_X: char = '✕';
    /// \u{2716}: '✖'
    pub const HEAVY_MULTIPLICATION_X: char = '✖';
    /// \u{2717}: '✗'
    pub const BALLOT_X: char = '✗';
    /// \u{2718}: '✘'
    pub const HEAVY_BALLOT_X: char = '✘';
    /// \u{2719}: '✙'
    pub const OUTLINED_GREEK_CROSS: char = '✙';
    /// \u{271a}: '✚'
    pub const HEAVY_GREEK_CROSS: char = '✚';
    /// \u{271b}: '✛'
    pub const OPEN_CENTRE_CROSS: char = '✛';
    /// \u{271c}: '✜'
    pub const HEAVY_OPEN_CENTRE_CROSS: char = '✜';
    /// \u{271d}: '✝'
    pub const LATIN_CROSS: char = '✝';
    /// \u{271e}: '✞'
    pub const SHADOWED_WHITE_LATIN_CROSS: char = '✞';
    /// \u{271f}: '✟'
    pub const OUTLINED_LATIN_CROSS: char = '✟';
    /// \u{2720}: '✠'
    pub const MALTESE_CROSS: char = '✠';
    /// \u{2721}: '✡'
    pub const STAR_OF_DAVID: char = '✡';
    /// \u{2722}: '✢'
    pub const FOUR_TEARDROP_DASH_SPOKED_ASTERISK: char = '✢';
    /// \u{2723}: '✣'
    pub const FOUR_BALLOON_DASH_SPOKED_ASTERISK: char = '✣';
    /// \u{2724}: '✤'
    pub const HEAVY_FOUR_BALLOON_DASH_SPOKED_ASTERISK: char = '✤';
    /// \u{2725}: '✥'
    pub const FOUR_CLUB_DASH_SPOKED_ASTERISK: char = '✥';
    /// \u{2726}: '✦'
    pub const BLACK_FOUR_POINTED_STAR: char = '✦';
    /// \u{2727}: '✧'
    pub const WHITE_FOUR_POINTED_STAR: char = '✧';
    /// \u{2728}: '✨'
    pub const SPARKLES: char = '✨';
    /// \u{2729}: '✩'
    pub const STRESS_OUTLINED_WHITE_STAR: char = '✩';
    /// \u{272a}: '✪'
    pub const CIRCLED_WHITE_STAR: char = '✪';
    /// \u{272b}: '✫'
    pub const OPEN_CENTRE_BLACK_STAR: char = '✫';
    /// \u{272c}: '✬'
    pub const BLACK_CENTRE_WHITE_STAR: char = '✬';
    /// \u{272d}: '✭'
    pub const OUTLINED_BLACK_STAR: char = '✭';
    /// \u{272e}: '✮'
    pub const HEAVY_OUTLINED_BLACK_STAR: char = '✮';
    /// \u{272f}: '✯'
    pub const PINWHEEL_STAR: char = '✯';
    /// \u{2730}: '✰'
    pub const SHADOWED_WHITE_STAR: char = '✰';
    /// \u{2731}: '✱'
    pub const HEAVY_ASTERISK: char = '✱';
    /// \u{2732}: '✲'
    pub const OPEN_CENTRE_ASTERISK: char = '✲';
    /// \u{2733}: '✳'
    pub const EIGHT_SPOKED_ASTERISK: char = '✳';
    /// \u{2734}: '✴'
    pub const EIGHT_POINTED_BLACK_STAR: char = '✴';
    /// \u{2735}: '✵'
    pub const EIGHT_POINTED_PINWHEEL_STAR: char = '✵';
    /// \u{2736}: '✶'
    pub const SIX_POINTED_BLACK_STAR: char = '✶';
    /// \u{2737}: '✷'
    pub const EIGHT_POINTED_RECTILINEAR_BLACK_STAR: char = '✷';
    /// \u{2738}: '✸'
    pub const HEAVY_EIGHT_POINTED_RECTILINEAR_BLACK_STAR: char = '✸';
    /// \u{2739}: '✹'
    pub const TWELVE_POINTED_BLACK_STAR: char = '✹';
    /// \u{273a}: '✺'
    pub const SIXTEEN_POINTED_ASTERISK: char = '✺';
    /// \u{273b}: '✻'
    pub const TEARDROP_DASH_SPOKED_ASTERISK: char = '✻';
    /// \u{273c}: '✼'
    pub const OPEN_CENTRE_TEARDROP_DASH_SPOKED_ASTERISK: char = '✼';
    /// \u{273d}: '✽'
    pub const HEAVY_TEARDROP_DASH_SPOKED_ASTERISK: char = '✽';
    /// \u{273e}: '✾'
    pub const SIX_PETALLED_BLACK_AND_WHITE_FLORETTE: char = '✾';
    /// \u{273f}: '✿'
    pub const BLACK_FLORETTE: char = '✿';
    /// \u{2740}: '❀'
    pub const WHITE_FLORETTE: char = '❀';
    /// \u{2741}: '❁'
    pub const EIGHT_PETALLED_OUTLINED_BLACK_FLORETTE: char = '❁';
    /// \u{2742}: '❂'
    pub const CIRCLED_OPEN_CENTRE_EIGHT_POINTED_STAR: char = '❂';
    /// \u{2743}: '❃'
    pub const HEAVY_TEARDROP_DASH_SPOKED_PINWHEEL_ASTERISK: char = '❃';
    /// \u{2744}: '❄'
    pub const SNOWFLAKE: char = '❄';
    /// \u{2745}: '❅'
    pub const TIGHT_TRIFOLIATE_SNOWFLAKE: char = '❅';
    /// \u{2746}: '❆'
    pub const HEAVY_CHEVRON_SNOWFLAKE: char = '❆';
    /// \u{2747}: '❇'
    pub const SPARKLE: char = '❇';
    /// \u{2748}: '❈'
    pub const HEAVY_SPARKLE: char = '❈';
    /// \u{2749}: '❉'
    pub const BALLOON_DASH_SPOKED_ASTERISK: char = '❉';
    /// \u{274a}: '❊'
    pub const EIGHT_TEARDROP_DASH_SPOKED_PROPELLER_ASTERISK: char = '❊';
    /// \u{274b}: '❋'
    pub const HEAVY_EIGHT_TEARDROP_DASH_SPOKED_PROPELLER_ASTERISK: char = '❋';
    /// \u{274c}: '❌'
    pub const CROSS_MARK: char = '❌';
    /// \u{274d}: '❍'
    pub const SHADOWED_WHITE_CIRCLE: char = '❍';
    /// \u{274e}: '❎'
    pub const NEGATIVE_SQUARED_CROSS_MARK: char = '❎';
    /// \u{274f}: '❏'
    pub const LOWER_RIGHT_DROP_DASH_SHADOWED_WHITE_SQUARE: char = '❏';
    /// \u{2750}: '❐'
    pub const UPPER_RIGHT_DROP_DASH_SHADOWED_WHITE_SQUARE: char = '❐';
    /// \u{2751}: '❑'
    pub const LOWER_RIGHT_SHADOWED_WHITE_SQUARE: char = '❑';
    /// \u{2752}: '❒'
    pub const UPPER_RIGHT_SHADOWED_WHITE_SQUARE: char = '❒';
    /// \u{2753}: '❓'
    pub const BLACK_QUESTION_MARK_ORNAMENT: char = '❓';
    /// \u{2754}: '❔'
    pub const WHITE_QUESTION_MARK_ORNAMENT: char = '❔';
    /// \u{2755}: '❕'
    pub const WHITE_EXCLAMATION_MARK_ORNAMENT: char = '❕';
    /// \u{2756}: '❖'
    pub const BLACK_DIAMOND_MINUS_WHITE_X: char = '❖';
    /// \u{2757}: '❗'
    pub const HEAVY_EXCLAMATION_MARK_SYMBOL: char = '❗';
    /// \u{2758}: '❘'
    pub const LIGHT_VERTICAL_BAR: char = '❘';
    /// \u{2759}: '❙'
    pub const MEDIUM_VERTICAL_BAR: char = '❙';
    /// \u{275a}: '❚'
    pub const HEAVY_VERTICAL_BAR: char = '❚';
    /// \u{275b}: '❛'
    pub const HEAVY_SINGLE_TURNED_COMMA_QUOTATION_MARK_ORNAMENT: char = '❛';
    /// \u{275c}: '❜'
    pub const HEAVY_SINGLE_COMMA_QUOTATION_MARK_ORNAMENT: char = '❜';
    /// \u{275d}: '❝'
    pub const HEAVY_DOUBLE_TURNED_COMMA_QUOTATION_MARK_ORNAMENT: char = '❝';
    /// \u{275e}: '❞'
    pub const HEAVY_DOUBLE_COMMA_QUOTATION_MARK_ORNAMENT: char = '❞';
    /// \u{275f}: '❟'
    pub const HEAVY_LOW_SINGLE_COMMA_QUOTATION_MARK_ORNAMENT: char = '❟';
    /// \u{2760}: '❠'
    pub const HEAVY_LOW_DOUBLE_COMMA_QUOTATION_MARK_ORNAMENT: char = '❠';
    /// \u{2761}: '❡'
    pub const CURVED_STEM_PARAGRAPH_SIGN_ORNAMENT: char = '❡';
    /// \u{2762}: '❢'
    pub const HEAVY_EXCLAMATION_MARK_ORNAMENT: char = '❢';
    /// \u{2763}: '❣'
    pub const HEAVY_HEART_EXCLAMATION_MARK_ORNAMENT: char = '❣';
    /// \u{2764}: '❤'
    pub const HEAVY_BLACK_HEART: char = '❤';
    /// \u{2765}: '❥'
    pub const ROTATED_HEAVY_BLACK_HEART_BULLET: char = '❥';
    /// \u{2766}: '❦'
    pub const FLORAL_HEART: char = '❦';
    /// \u{2767}: '❧'
    pub const ROTATED_FLORAL_HEART_BULLET: char = '❧';
    /// \u{2768}: '❨'
    pub const MEDIUM_LEFT_PARENTHESIS_ORNAMENT: char = '❨';
    /// \u{2769}: '❩'
    pub const MEDIUM_RIGHT_PARENTHESIS_ORNAMENT: char = '❩';
    /// \u{276a}: '❪'
    pub const MEDIUM_FLATTENED_LEFT_PARENTHESIS_ORNAMENT: char = '❪';
    /// \u{276b}: '❫'
    pub const MEDIUM_FLATTENED_RIGHT_PARENTHESIS_ORNAMENT: char = '❫';
    /// \u{276c}: '❬'
    pub const MEDIUM_LEFT_DASH_POINTING_ANGLE_BRACKET_ORNAMENT: char = '❬';
    /// \u{276d}: '❭'
    pub const MEDIUM_RIGHT_DASH_POINTING_ANGLE_BRACKET_ORNAMENT: char = '❭';
    /// \u{276e}: '❮'
    pub const HEAVY_LEFT_DASH_POINTING_ANGLE_QUOTATION_MARK_ORNAMENT: char = '❮';
    /// \u{276f}: '❯'
    pub const HEAVY_RIGHT_DASH_POINTING_ANGLE_QUOTATION_MARK_ORNAMENT: char = '❯';
    /// \u{2770}: '❰'
    pub const HEAVY_LEFT_DASH_POINTING_ANGLE_BRACKET_ORNAMENT: char = '❰';
    /// \u{2771}: '❱'
    pub const HEAVY_RIGHT_DASH_POINTING_ANGLE_BRACKET_ORNAMENT: char = '❱';
    /// \u{2772}: '❲'
    pub const LIGHT_LEFT_TORTOISE_SHELL_BRACKET_ORNAMENT: char = '❲';
    /// \u{2773}: '❳'
    pub const LIGHT_RIGHT_TORTOISE_SHELL_BRACKET_ORNAMENT: char = '❳';
    /// \u{2774}: '❴'
    pub const MEDIUM_LEFT_CURLY_BRACKET_ORNAMENT: char = '❴';
    /// \u{2775}: '❵'
    pub const MEDIUM_RIGHT_CURLY_BRACKET_ORNAMENT: char = '❵';
    /// \u{2776}: '❶'
    pub const DINGBAT_NEGATIVE_CIRCLED_DIGIT_ONE: char = '❶';
    /// \u{2777}: '❷'
    pub const DINGBAT_NEGATIVE_CIRCLED_DIGIT_TWO: char = '❷';
    /// \u{2778}: '❸'
    pub const DINGBAT_NEGATIVE_CIRCLED_DIGIT_THREE: char = '❸';
    /// \u{2779}: '❹'
    pub const DINGBAT_NEGATIVE_CIRCLED_DIGIT_FOUR: char = '❹';
    /// \u{277a}: '❺'
    pub const DINGBAT_NEGATIVE_CIRCLED_DIGIT_FIVE: char = '❺';
    /// \u{277b}: '❻'
    pub const DINGBAT_NEGATIVE_CIRCLED_DIGIT_SIX: char = '❻';
    /// \u{277c}: '❼'
    pub const DINGBAT_NEGATIVE_CIRCLED_DIGIT_SEVEN: char = '❼';
    /// \u{277d}: '❽'
    pub const DINGBAT_NEGATIVE_CIRCLED_DIGIT_EIGHT: char = '❽';
    /// \u{277e}: '❾'
    pub const DINGBAT_NEGATIVE_CIRCLED_DIGIT_NINE: char = '❾';
    /// \u{277f}: '❿'
    pub const DINGBAT_NEGATIVE_CIRCLED_NUMBER_TEN: char = '❿';
    /// \u{2780}: '➀'
    pub const DINGBAT_CIRCLED_SANS_DASH_SERIF_DIGIT_ONE: char = '➀';
    /// \u{2781}: '➁'
    pub const DINGBAT_CIRCLED_SANS_DASH_SERIF_DIGIT_TWO: char = '➁';
    /// \u{2782}: '➂'
    pub const DINGBAT_CIRCLED_SANS_DASH_SERIF_DIGIT_THREE: char = '➂';
    /// \u{2783}: '➃'
    pub const DINGBAT_CIRCLED_SANS_DASH_SERIF_DIGIT_FOUR: char = '➃';
    /// \u{2784}: '➄'
    pub const DINGBAT_CIRCLED_SANS_DASH_SERIF_DIGIT_FIVE: char = '➄';
    /// \u{2785}: '➅'
    pub const DINGBAT_CIRCLED_SANS_DASH_SERIF_DIGIT_SIX: char = '➅';
    /// \u{2786}: '➆'
    pub const DINGBAT_CIRCLED_SANS_DASH_SERIF_DIGIT_SEVEN: char = '➆';
    /// \u{2787}: '➇'
    pub const DINGBAT_CIRCLED_SANS_DASH_SERIF_DIGIT_EIGHT: char = '➇';
    /// \u{2788}: '➈'
    pub const DINGBAT_CIRCLED_SANS_DASH_SERIF_DIGIT_NINE: char = '➈';
    /// \u{2789}: '➉'
    pub const DINGBAT_CIRCLED_SANS_DASH_SERIF_NUMBER_TEN: char = '➉';
    /// \u{278a}: '➊'
    pub const DINGBAT_NEGATIVE_CIRCLED_SANS_DASH_SERIF_DIGIT_ONE: char = '➊';
    /// \u{278b}: '➋'
    pub const DINGBAT_NEGATIVE_CIRCLED_SANS_DASH_SERIF_DIGIT_TWO: char = '➋';
    /// \u{278c}: '➌'
    pub const DINGBAT_NEGATIVE_CIRCLED_SANS_DASH_SERIF_DIGIT_THREE: char = '➌';
    /// \u{278d}: '➍'
    pub const DINGBAT_NEGATIVE_CIRCLED_SANS_DASH_SERIF_DIGIT_FOUR: char = '➍';
    /// \u{278e}: '➎'
    pub const DINGBAT_NEGATIVE_CIRCLED_SANS_DASH_SERIF_DIGIT_FIVE: char = '➎';
    /// \u{278f}: '➏'
    pub const DINGBAT_NEGATIVE_CIRCLED_SANS_DASH_SERIF_DIGIT_SIX: char = '➏';
    /// \u{2790}: '➐'
    pub const DINGBAT_NEGATIVE_CIRCLED_SANS_DASH_SERIF_DIGIT_SEVEN: char = '➐';
    /// \u{2791}: '➑'
    pub const DINGBAT_NEGATIVE_CIRCLED_SANS_DASH_SERIF_DIGIT_EIGHT: char = '➑';
    /// \u{2792}: '➒'
    pub const DINGBAT_NEGATIVE_CIRCLED_SANS_DASH_SERIF_DIGIT_NINE: char = '➒';
    /// \u{2793}: '➓'
    pub const DINGBAT_NEGATIVE_CIRCLED_SANS_DASH_SERIF_NUMBER_TEN: char = '➓';
    /// \u{2794}: '➔'
    pub const HEAVY_WIDE_DASH_HEADED_RIGHTWARDS_ARROW: char = '➔';
    /// \u{2795}: '➕'
    pub const HEAVY_PLUS_SIGN: char = '➕';
    /// \u{2796}: '➖'
    pub const HEAVY_MINUS_SIGN: char = '➖';
    /// \u{2797}: '➗'
    pub const HEAVY_DIVISION_SIGN: char = '➗';
    /// \u{2798}: '➘'
    pub const HEAVY_SOUTH_EAST_ARROW: char = '➘';
    /// \u{2799}: '➙'
    pub const HEAVY_RIGHTWARDS_ARROW: char = '➙';
    /// \u{279a}: '➚'
    pub const HEAVY_NORTH_EAST_ARROW: char = '➚';
    /// \u{279b}: '➛'
    pub const DRAFTING_POINT_RIGHTWARDS_ARROW: char = '➛';
    /// \u{279c}: '➜'
    pub const HEAVY_ROUND_DASH_TIPPED_RIGHTWARDS_ARROW: char = '➜';
    /// \u{279d}: '➝'
    pub const TRIANGLE_DASH_HEADED_RIGHTWARDS_ARROW: char = '➝';
    /// \u{279e}: '➞'
    pub const HEAVY_TRIANGLE_DASH_HEADED_RIGHTWARDS_ARROW: char = '➞';
    /// \u{279f}: '➟'
    pub const DASHED_TRIANGLE_DASH_HEADED_RIGHTWARDS_ARROW: char = '➟';
    /// \u{27a0}: '➠'
    pub const HEAVY_DASHED_TRIANGLE_DASH_HEADED_RIGHTWARDS_ARROW: char = '➠';
    /// \u{27a1}: '➡'
    pub const BLACK_RIGHTWARDS_ARROW: char = '➡';
    /// \u{27a2}: '➢'
    pub const THREE_DASH_D_TOP_DASH_LIGHTED_RIGHTWARDS_ARROWHEAD: char = '➢';
    /// \u{27a3}: '➣'
    pub const THREE_DASH_D_BOTTOM_DASH_LIGHTED_RIGHTWARDS_ARROWHEAD: char = '➣';
    /// \u{27a4}: '➤'
    pub const BLACK_RIGHTWARDS_ARROWHEAD: char = '➤';
    /// \u{27a5}: '➥'
    pub const HEAVY_BLACK_CURVED_DOWNWARDS_AND_RIGHTWARDS_ARROW: char = '➥';
    /// \u{27a6}: '➦'
    pub const HEAVY_BLACK_CURVED_UPWARDS_AND_RIGHTWARDS_ARROW: char = '➦';
    /// \u{27a7}: '➧'
    pub const SQUAT_BLACK_RIGHTWARDS_ARROW: char = '➧';
    /// \u{27a8}: '➨'
    pub const HEAVY_CONCAVE_DASH_POINTED_BLACK_RIGHTWARDS_ARROW: char = '➨';
    /// \u{27a9}: '➩'
    pub const RIGHT_DASH_SHADED_WHITE_RIGHTWARDS_ARROW: char = '➩';
    /// \u{27aa}: '➪'
    pub const LEFT_DASH_SHADED_WHITE_RIGHTWARDS_ARROW: char = '➪';
    /// \u{27ab}: '➫'
    pub const BACK_DASH_TILTED_SHADOWED_WHITE_RIGHTWARDS_ARROW: char = '➫';
    /// \u{27ac}: '➬'
    pub const FRONT_DASH_TILTED_SHADOWED_WHITE_RIGHTWARDS_ARROW: char = '➬';
    /// \u{27ad}: '➭'
    pub const HEAVY_LOWER_RIGHT_DASH_SHADOWED_WHITE_RIGHTWARDS_ARROW: char = '➭';
    /// \u{27ae}: '➮'
    pub const HEAVY_UPPER_RIGHT_DASH_SHADOWED_WHITE_RIGHTWARDS_ARROW: char = '➮';
    /// \u{27af}: '➯'
    pub const NOTCHED_LOWER_RIGHT_DASH_SHADOWED_WHITE_RIGHTWARDS_ARROW: char = '➯';
    /// \u{27b0}: '➰'
    pub const CURLY_LOOP: char = '➰';
    /// \u{27b1}: '➱'
    pub const NOTCHED_UPPER_RIGHT_DASH_SHADOWED_WHITE_RIGHTWARDS_ARROW: char = '➱';
    /// \u{27b2}: '➲'
    pub const CIRCLED_HEAVY_WHITE_RIGHTWARDS_ARROW: char = '➲';
    /// \u{27b3}: '➳'
    pub const WHITE_DASH_FEATHERED_RIGHTWARDS_ARROW: char = '➳';
    /// \u{27b4}: '➴'
    pub const BLACK_DASH_FEATHERED_SOUTH_EAST_ARROW: char = '➴';
    /// \u{27b5}: '➵'
    pub const BLACK_DASH_FEATHERED_RIGHTWARDS_ARROW: char = '➵';
    /// \u{27b6}: '➶'
    pub const BLACK_DASH_FEATHERED_NORTH_EAST_ARROW: char = '➶';
    /// \u{27b7}: '➷'
    pub const HEAVY_BLACK_DASH_FEATHERED_SOUTH_EAST_ARROW: char = '➷';
    /// \u{27b8}: '➸'
    pub const HEAVY_BLACK_DASH_FEATHERED_RIGHTWARDS_ARROW: char = '➸';
    /// \u{27b9}: '➹'
    pub const HEAVY_BLACK_DASH_FEATHERED_NORTH_EAST_ARROW: char = '➹';
    /// \u{27ba}: '➺'
    pub const TEARDROP_DASH_BARBED_RIGHTWARDS_ARROW: char = '➺';
    /// \u{27bb}: '➻'
    pub const HEAVY_TEARDROP_DASH_SHANKED_RIGHTWARDS_ARROW: char = '➻';
    /// \u{27bc}: '➼'
    pub const WEDGE_DASH_TAILED_RIGHTWARDS_ARROW: char = '➼';
    /// \u{27bd}: '➽'
    pub const HEAVY_WEDGE_DASH_TAILED_RIGHTWARDS_ARROW: char = '➽';
    /// \u{27be}: '➾'
    pub const OPEN_DASH_OUTLINED_RIGHTWARDS_ARROW: char = '➾';
}

/// An enum to represent all characters in the Dingbats block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Dingbats {
    /// \u{2700}: '✀'
    BlackSafetyScissors,
    /// \u{2701}: '✁'
    UpperBladeScissors,
    /// \u{2702}: '✂'
    BlackScissors,
    /// \u{2703}: '✃'
    LowerBladeScissors,
    /// \u{2704}: '✄'
    WhiteScissors,
    /// \u{2705}: '✅'
    WhiteHeavyCheckMark,
    /// \u{2706}: '✆'
    TelephoneLocationSign,
    /// \u{2707}: '✇'
    TapeDrive,
    /// \u{2708}: '✈'
    Airplane,
    /// \u{2709}: '✉'
    Envelope,
    /// \u{270a}: '✊'
    RaisedFist,
    /// \u{270b}: '✋'
    RaisedHand,
    /// \u{270c}: '✌'
    VictoryHand,
    /// \u{270d}: '✍'
    WritingHand,
    /// \u{270e}: '✎'
    LowerRightPencil,
    /// \u{270f}: '✏'
    Pencil,
    /// \u{2710}: '✐'
    UpperRightPencil,
    /// \u{2711}: '✑'
    WhiteNib,
    /// \u{2712}: '✒'
    BlackNib,
    /// \u{2713}: '✓'
    CheckMark,
    /// \u{2714}: '✔'
    HeavyCheckMark,
    /// \u{2715}: '✕'
    MultiplicationX,
    /// \u{2716}: '✖'
    HeavyMultiplicationX,
    /// \u{2717}: '✗'
    BallotX,
    /// \u{2718}: '✘'
    HeavyBallotX,
    /// \u{2719}: '✙'
    OutlinedGreekCross,
    /// \u{271a}: '✚'
    HeavyGreekCross,
    /// \u{271b}: '✛'
    OpenCentreCross,
    /// \u{271c}: '✜'
    HeavyOpenCentreCross,
    /// \u{271d}: '✝'
    LatinCross,
    /// \u{271e}: '✞'
    ShadowedWhiteLatinCross,
    /// \u{271f}: '✟'
    OutlinedLatinCross,
    /// \u{2720}: '✠'
    MalteseCross,
    /// \u{2721}: '✡'
    StarOfDavid,
    /// \u{2722}: '✢'
    FourTeardropDashSpokedAsterisk,
    /// \u{2723}: '✣'
    FourBalloonDashSpokedAsterisk,
    /// \u{2724}: '✤'
    HeavyFourBalloonDashSpokedAsterisk,
    /// \u{2725}: '✥'
    FourClubDashSpokedAsterisk,
    /// \u{2726}: '✦'
    BlackFourPointedStar,
    /// \u{2727}: '✧'
    WhiteFourPointedStar,
    /// \u{2728}: '✨'
    Sparkles,
    /// \u{2729}: '✩'
    StressOutlinedWhiteStar,
    /// \u{272a}: '✪'
    CircledWhiteStar,
    /// \u{272b}: '✫'
    OpenCentreBlackStar,
    /// \u{272c}: '✬'
    BlackCentreWhiteStar,
    /// \u{272d}: '✭'
    OutlinedBlackStar,
    /// \u{272e}: '✮'
    HeavyOutlinedBlackStar,
    /// \u{272f}: '✯'
    PinwheelStar,
    /// \u{2730}: '✰'
    ShadowedWhiteStar,
    /// \u{2731}: '✱'
    HeavyAsterisk,
    /// \u{2732}: '✲'
    OpenCentreAsterisk,
    /// \u{2733}: '✳'
    EightSpokedAsterisk,
    /// \u{2734}: '✴'
    EightPointedBlackStar,
    /// \u{2735}: '✵'
    EightPointedPinwheelStar,
    /// \u{2736}: '✶'
    SixPointedBlackStar,
    /// \u{2737}: '✷'
    EightPointedRectilinearBlackStar,
    /// \u{2738}: '✸'
    HeavyEightPointedRectilinearBlackStar,
    /// \u{2739}: '✹'
    TwelvePointedBlackStar,
    /// \u{273a}: '✺'
    SixteenPointedAsterisk,
    /// \u{273b}: '✻'
    TeardropDashSpokedAsterisk,
    /// \u{273c}: '✼'
    OpenCentreTeardropDashSpokedAsterisk,
    /// \u{273d}: '✽'
    HeavyTeardropDashSpokedAsterisk,
    /// \u{273e}: '✾'
    SixPetalledBlackAndWhiteFlorette,
    /// \u{273f}: '✿'
    BlackFlorette,
    /// \u{2740}: '❀'
    WhiteFlorette,
    /// \u{2741}: '❁'
    EightPetalledOutlinedBlackFlorette,
    /// \u{2742}: '❂'
    CircledOpenCentreEightPointedStar,
    /// \u{2743}: '❃'
    HeavyTeardropDashSpokedPinwheelAsterisk,
    /// \u{2744}: '❄'
    Snowflake,
    /// \u{2745}: '❅'
    TightTrifoliateSnowflake,
    /// \u{2746}: '❆'
    HeavyChevronSnowflake,
    /// \u{2747}: '❇'
    Sparkle,
    /// \u{2748}: '❈'
    HeavySparkle,
    /// \u{2749}: '❉'
    BalloonDashSpokedAsterisk,
    /// \u{274a}: '❊'
    EightTeardropDashSpokedPropellerAsterisk,
    /// \u{274b}: '❋'
    HeavyEightTeardropDashSpokedPropellerAsterisk,
    /// \u{274c}: '❌'
    CrossMark,
    /// \u{274d}: '❍'
    ShadowedWhiteCircle,
    /// \u{274e}: '❎'
    NegativeSquaredCrossMark,
    /// \u{274f}: '❏'
    LowerRightDropDashShadowedWhiteSquare,
    /// \u{2750}: '❐'
    UpperRightDropDashShadowedWhiteSquare,
    /// \u{2751}: '❑'
    LowerRightShadowedWhiteSquare,
    /// \u{2752}: '❒'
    UpperRightShadowedWhiteSquare,
    /// \u{2753}: '❓'
    BlackQuestionMarkOrnament,
    /// \u{2754}: '❔'
    WhiteQuestionMarkOrnament,
    /// \u{2755}: '❕'
    WhiteExclamationMarkOrnament,
    /// \u{2756}: '❖'
    BlackDiamondMinusWhiteX,
    /// \u{2757}: '❗'
    HeavyExclamationMarkSymbol,
    /// \u{2758}: '❘'
    LightVerticalBar,
    /// \u{2759}: '❙'
    MediumVerticalBar,
    /// \u{275a}: '❚'
    HeavyVerticalBar,
    /// \u{275b}: '❛'
    HeavySingleTurnedCommaQuotationMarkOrnament,
    /// \u{275c}: '❜'
    HeavySingleCommaQuotationMarkOrnament,
    /// \u{275d}: '❝'
    HeavyDoubleTurnedCommaQuotationMarkOrnament,
    /// \u{275e}: '❞'
    HeavyDoubleCommaQuotationMarkOrnament,
    /// \u{275f}: '❟'
    HeavyLowSingleCommaQuotationMarkOrnament,
    /// \u{2760}: '❠'
    HeavyLowDoubleCommaQuotationMarkOrnament,
    /// \u{2761}: '❡'
    CurvedStemParagraphSignOrnament,
    /// \u{2762}: '❢'
    HeavyExclamationMarkOrnament,
    /// \u{2763}: '❣'
    HeavyHeartExclamationMarkOrnament,
    /// \u{2764}: '❤'
    HeavyBlackHeart,
    /// \u{2765}: '❥'
    RotatedHeavyBlackHeartBullet,
    /// \u{2766}: '❦'
    FloralHeart,
    /// \u{2767}: '❧'
    RotatedFloralHeartBullet,
    /// \u{2768}: '❨'
    MediumLeftParenthesisOrnament,
    /// \u{2769}: '❩'
    MediumRightParenthesisOrnament,
    /// \u{276a}: '❪'
    MediumFlattenedLeftParenthesisOrnament,
    /// \u{276b}: '❫'
    MediumFlattenedRightParenthesisOrnament,
    /// \u{276c}: '❬'
    MediumLeftDashPointingAngleBracketOrnament,
    /// \u{276d}: '❭'
    MediumRightDashPointingAngleBracketOrnament,
    /// \u{276e}: '❮'
    HeavyLeftDashPointingAngleQuotationMarkOrnament,
    /// \u{276f}: '❯'
    HeavyRightDashPointingAngleQuotationMarkOrnament,
    /// \u{2770}: '❰'
    HeavyLeftDashPointingAngleBracketOrnament,
    /// \u{2771}: '❱'
    HeavyRightDashPointingAngleBracketOrnament,
    /// \u{2772}: '❲'
    LightLeftTortoiseShellBracketOrnament,
    /// \u{2773}: '❳'
    LightRightTortoiseShellBracketOrnament,
    /// \u{2774}: '❴'
    MediumLeftCurlyBracketOrnament,
    /// \u{2775}: '❵'
    MediumRightCurlyBracketOrnament,
    /// \u{2776}: '❶'
    DingbatNegativeCircledDigitOne,
    /// \u{2777}: '❷'
    DingbatNegativeCircledDigitTwo,
    /// \u{2778}: '❸'
    DingbatNegativeCircledDigitThree,
    /// \u{2779}: '❹'
    DingbatNegativeCircledDigitFour,
    /// \u{277a}: '❺'
    DingbatNegativeCircledDigitFive,
    /// \u{277b}: '❻'
    DingbatNegativeCircledDigitSix,
    /// \u{277c}: '❼'
    DingbatNegativeCircledDigitSeven,
    /// \u{277d}: '❽'
    DingbatNegativeCircledDigitEight,
    /// \u{277e}: '❾'
    DingbatNegativeCircledDigitNine,
    /// \u{277f}: '❿'
    DingbatNegativeCircledNumberTen,
    /// \u{2780}: '➀'
    DingbatCircledSansDashSerifDigitOne,
    /// \u{2781}: '➁'
    DingbatCircledSansDashSerifDigitTwo,
    /// \u{2782}: '➂'
    DingbatCircledSansDashSerifDigitThree,
    /// \u{2783}: '➃'
    DingbatCircledSansDashSerifDigitFour,
    /// \u{2784}: '➄'
    DingbatCircledSansDashSerifDigitFive,
    /// \u{2785}: '➅'
    DingbatCircledSansDashSerifDigitSix,
    /// \u{2786}: '➆'
    DingbatCircledSansDashSerifDigitSeven,
    /// \u{2787}: '➇'
    DingbatCircledSansDashSerifDigitEight,
    /// \u{2788}: '➈'
    DingbatCircledSansDashSerifDigitNine,
    /// \u{2789}: '➉'
    DingbatCircledSansDashSerifNumberTen,
    /// \u{278a}: '➊'
    DingbatNegativeCircledSansDashSerifDigitOne,
    /// \u{278b}: '➋'
    DingbatNegativeCircledSansDashSerifDigitTwo,
    /// \u{278c}: '➌'
    DingbatNegativeCircledSansDashSerifDigitThree,
    /// \u{278d}: '➍'
    DingbatNegativeCircledSansDashSerifDigitFour,
    /// \u{278e}: '➎'
    DingbatNegativeCircledSansDashSerifDigitFive,
    /// \u{278f}: '➏'
    DingbatNegativeCircledSansDashSerifDigitSix,
    /// \u{2790}: '➐'
    DingbatNegativeCircledSansDashSerifDigitSeven,
    /// \u{2791}: '➑'
    DingbatNegativeCircledSansDashSerifDigitEight,
    /// \u{2792}: '➒'
    DingbatNegativeCircledSansDashSerifDigitNine,
    /// \u{2793}: '➓'
    DingbatNegativeCircledSansDashSerifNumberTen,
    /// \u{2794}: '➔'
    HeavyWideDashHeadedRightwardsArrow,
    /// \u{2795}: '➕'
    HeavyPlusSign,
    /// \u{2796}: '➖'
    HeavyMinusSign,
    /// \u{2797}: '➗'
    HeavyDivisionSign,
    /// \u{2798}: '➘'
    HeavySouthEastArrow,
    /// \u{2799}: '➙'
    HeavyRightwardsArrow,
    /// \u{279a}: '➚'
    HeavyNorthEastArrow,
    /// \u{279b}: '➛'
    DraftingPointRightwardsArrow,
    /// \u{279c}: '➜'
    HeavyRoundDashTippedRightwardsArrow,
    /// \u{279d}: '➝'
    TriangleDashHeadedRightwardsArrow,
    /// \u{279e}: '➞'
    HeavyTriangleDashHeadedRightwardsArrow,
    /// \u{279f}: '➟'
    DashedTriangleDashHeadedRightwardsArrow,
    /// \u{27a0}: '➠'
    HeavyDashedTriangleDashHeadedRightwardsArrow,
    /// \u{27a1}: '➡'
    BlackRightwardsArrow,
    /// \u{27a2}: '➢'
    ThreeDashDTopDashLightedRightwardsArrowhead,
    /// \u{27a3}: '➣'
    ThreeDashDBottomDashLightedRightwardsArrowhead,
    /// \u{27a4}: '➤'
    BlackRightwardsArrowhead,
    /// \u{27a5}: '➥'
    HeavyBlackCurvedDownwardsAndRightwardsArrow,
    /// \u{27a6}: '➦'
    HeavyBlackCurvedUpwardsAndRightwardsArrow,
    /// \u{27a7}: '➧'
    SquatBlackRightwardsArrow,
    /// \u{27a8}: '➨'
    HeavyConcaveDashPointedBlackRightwardsArrow,
    /// \u{27a9}: '➩'
    RightDashShadedWhiteRightwardsArrow,
    /// \u{27aa}: '➪'
    LeftDashShadedWhiteRightwardsArrow,
    /// \u{27ab}: '➫'
    BackDashTiltedShadowedWhiteRightwardsArrow,
    /// \u{27ac}: '➬'
    FrontDashTiltedShadowedWhiteRightwardsArrow,
    /// \u{27ad}: '➭'
    HeavyLowerRightDashShadowedWhiteRightwardsArrow,
    /// \u{27ae}: '➮'
    HeavyUpperRightDashShadowedWhiteRightwardsArrow,
    /// \u{27af}: '➯'
    NotchedLowerRightDashShadowedWhiteRightwardsArrow,
    /// \u{27b0}: '➰'
    CurlyLoop,
    /// \u{27b1}: '➱'
    NotchedUpperRightDashShadowedWhiteRightwardsArrow,
    /// \u{27b2}: '➲'
    CircledHeavyWhiteRightwardsArrow,
    /// \u{27b3}: '➳'
    WhiteDashFeatheredRightwardsArrow,
    /// \u{27b4}: '➴'
    BlackDashFeatheredSouthEastArrow,
    /// \u{27b5}: '➵'
    BlackDashFeatheredRightwardsArrow,
    /// \u{27b6}: '➶'
    BlackDashFeatheredNorthEastArrow,
    /// \u{27b7}: '➷'
    HeavyBlackDashFeatheredSouthEastArrow,
    /// \u{27b8}: '➸'
    HeavyBlackDashFeatheredRightwardsArrow,
    /// \u{27b9}: '➹'
    HeavyBlackDashFeatheredNorthEastArrow,
    /// \u{27ba}: '➺'
    TeardropDashBarbedRightwardsArrow,
    /// \u{27bb}: '➻'
    HeavyTeardropDashShankedRightwardsArrow,
    /// \u{27bc}: '➼'
    WedgeDashTailedRightwardsArrow,
    /// \u{27bd}: '➽'
    HeavyWedgeDashTailedRightwardsArrow,
    /// \u{27be}: '➾'
    OpenDashOutlinedRightwardsArrow,
}

impl Into<char> for Dingbats {
    fn into(self) -> char {
        use constants::*;
        match self {
            Dingbats::BlackSafetyScissors => BLACK_SAFETY_SCISSORS,
            Dingbats::UpperBladeScissors => UPPER_BLADE_SCISSORS,
            Dingbats::BlackScissors => BLACK_SCISSORS,
            Dingbats::LowerBladeScissors => LOWER_BLADE_SCISSORS,
            Dingbats::WhiteScissors => WHITE_SCISSORS,
            Dingbats::WhiteHeavyCheckMark => WHITE_HEAVY_CHECK_MARK,
            Dingbats::TelephoneLocationSign => TELEPHONE_LOCATION_SIGN,
            Dingbats::TapeDrive => TAPE_DRIVE,
            Dingbats::Airplane => AIRPLANE,
            Dingbats::Envelope => ENVELOPE,
            Dingbats::RaisedFist => RAISED_FIST,
            Dingbats::RaisedHand => RAISED_HAND,
            Dingbats::VictoryHand => VICTORY_HAND,
            Dingbats::WritingHand => WRITING_HAND,
            Dingbats::LowerRightPencil => LOWER_RIGHT_PENCIL,
            Dingbats::Pencil => PENCIL,
            Dingbats::UpperRightPencil => UPPER_RIGHT_PENCIL,
            Dingbats::WhiteNib => WHITE_NIB,
            Dingbats::BlackNib => BLACK_NIB,
            Dingbats::CheckMark => CHECK_MARK,
            Dingbats::HeavyCheckMark => HEAVY_CHECK_MARK,
            Dingbats::MultiplicationX => MULTIPLICATION_X,
            Dingbats::HeavyMultiplicationX => HEAVY_MULTIPLICATION_X,
            Dingbats::BallotX => BALLOT_X,
            Dingbats::HeavyBallotX => HEAVY_BALLOT_X,
            Dingbats::OutlinedGreekCross => OUTLINED_GREEK_CROSS,
            Dingbats::HeavyGreekCross => HEAVY_GREEK_CROSS,
            Dingbats::OpenCentreCross => OPEN_CENTRE_CROSS,
            Dingbats::HeavyOpenCentreCross => HEAVY_OPEN_CENTRE_CROSS,
            Dingbats::LatinCross => LATIN_CROSS,
            Dingbats::ShadowedWhiteLatinCross => SHADOWED_WHITE_LATIN_CROSS,
            Dingbats::OutlinedLatinCross => OUTLINED_LATIN_CROSS,
            Dingbats::MalteseCross => MALTESE_CROSS,
            Dingbats::StarOfDavid => STAR_OF_DAVID,
            Dingbats::FourTeardropDashSpokedAsterisk => FOUR_TEARDROP_DASH_SPOKED_ASTERISK,
            Dingbats::FourBalloonDashSpokedAsterisk => FOUR_BALLOON_DASH_SPOKED_ASTERISK,
            Dingbats::HeavyFourBalloonDashSpokedAsterisk => HEAVY_FOUR_BALLOON_DASH_SPOKED_ASTERISK,
            Dingbats::FourClubDashSpokedAsterisk => FOUR_CLUB_DASH_SPOKED_ASTERISK,
            Dingbats::BlackFourPointedStar => BLACK_FOUR_POINTED_STAR,
            Dingbats::WhiteFourPointedStar => WHITE_FOUR_POINTED_STAR,
            Dingbats::Sparkles => SPARKLES,
            Dingbats::StressOutlinedWhiteStar => STRESS_OUTLINED_WHITE_STAR,
            Dingbats::CircledWhiteStar => CIRCLED_WHITE_STAR,
            Dingbats::OpenCentreBlackStar => OPEN_CENTRE_BLACK_STAR,
            Dingbats::BlackCentreWhiteStar => BLACK_CENTRE_WHITE_STAR,
            Dingbats::OutlinedBlackStar => OUTLINED_BLACK_STAR,
            Dingbats::HeavyOutlinedBlackStar => HEAVY_OUTLINED_BLACK_STAR,
            Dingbats::PinwheelStar => PINWHEEL_STAR,
            Dingbats::ShadowedWhiteStar => SHADOWED_WHITE_STAR,
            Dingbats::HeavyAsterisk => HEAVY_ASTERISK,
            Dingbats::OpenCentreAsterisk => OPEN_CENTRE_ASTERISK,
            Dingbats::EightSpokedAsterisk => EIGHT_SPOKED_ASTERISK,
            Dingbats::EightPointedBlackStar => EIGHT_POINTED_BLACK_STAR,
            Dingbats::EightPointedPinwheelStar => EIGHT_POINTED_PINWHEEL_STAR,
            Dingbats::SixPointedBlackStar => SIX_POINTED_BLACK_STAR,
            Dingbats::EightPointedRectilinearBlackStar => EIGHT_POINTED_RECTILINEAR_BLACK_STAR,
            Dingbats::HeavyEightPointedRectilinearBlackStar => HEAVY_EIGHT_POINTED_RECTILINEAR_BLACK_STAR,
            Dingbats::TwelvePointedBlackStar => TWELVE_POINTED_BLACK_STAR,
            Dingbats::SixteenPointedAsterisk => SIXTEEN_POINTED_ASTERISK,
            Dingbats::TeardropDashSpokedAsterisk => TEARDROP_DASH_SPOKED_ASTERISK,
            Dingbats::OpenCentreTeardropDashSpokedAsterisk => OPEN_CENTRE_TEARDROP_DASH_SPOKED_ASTERISK,
            Dingbats::HeavyTeardropDashSpokedAsterisk => HEAVY_TEARDROP_DASH_SPOKED_ASTERISK,
            Dingbats::SixPetalledBlackAndWhiteFlorette => SIX_PETALLED_BLACK_AND_WHITE_FLORETTE,
            Dingbats::BlackFlorette => BLACK_FLORETTE,
            Dingbats::WhiteFlorette => WHITE_FLORETTE,
            Dingbats::EightPetalledOutlinedBlackFlorette => EIGHT_PETALLED_OUTLINED_BLACK_FLORETTE,
            Dingbats::CircledOpenCentreEightPointedStar => CIRCLED_OPEN_CENTRE_EIGHT_POINTED_STAR,
            Dingbats::HeavyTeardropDashSpokedPinwheelAsterisk => HEAVY_TEARDROP_DASH_SPOKED_PINWHEEL_ASTERISK,
            Dingbats::Snowflake => SNOWFLAKE,
            Dingbats::TightTrifoliateSnowflake => TIGHT_TRIFOLIATE_SNOWFLAKE,
            Dingbats::HeavyChevronSnowflake => HEAVY_CHEVRON_SNOWFLAKE,
            Dingbats::Sparkle => SPARKLE,
            Dingbats::HeavySparkle => HEAVY_SPARKLE,
            Dingbats::BalloonDashSpokedAsterisk => BALLOON_DASH_SPOKED_ASTERISK,
            Dingbats::EightTeardropDashSpokedPropellerAsterisk => EIGHT_TEARDROP_DASH_SPOKED_PROPELLER_ASTERISK,
            Dingbats::HeavyEightTeardropDashSpokedPropellerAsterisk => HEAVY_EIGHT_TEARDROP_DASH_SPOKED_PROPELLER_ASTERISK,
            Dingbats::CrossMark => CROSS_MARK,
            Dingbats::ShadowedWhiteCircle => SHADOWED_WHITE_CIRCLE,
            Dingbats::NegativeSquaredCrossMark => NEGATIVE_SQUARED_CROSS_MARK,
            Dingbats::LowerRightDropDashShadowedWhiteSquare => LOWER_RIGHT_DROP_DASH_SHADOWED_WHITE_SQUARE,
            Dingbats::UpperRightDropDashShadowedWhiteSquare => UPPER_RIGHT_DROP_DASH_SHADOWED_WHITE_SQUARE,
            Dingbats::LowerRightShadowedWhiteSquare => LOWER_RIGHT_SHADOWED_WHITE_SQUARE,
            Dingbats::UpperRightShadowedWhiteSquare => UPPER_RIGHT_SHADOWED_WHITE_SQUARE,
            Dingbats::BlackQuestionMarkOrnament => BLACK_QUESTION_MARK_ORNAMENT,
            Dingbats::WhiteQuestionMarkOrnament => WHITE_QUESTION_MARK_ORNAMENT,
            Dingbats::WhiteExclamationMarkOrnament => WHITE_EXCLAMATION_MARK_ORNAMENT,
            Dingbats::BlackDiamondMinusWhiteX => BLACK_DIAMOND_MINUS_WHITE_X,
            Dingbats::HeavyExclamationMarkSymbol => HEAVY_EXCLAMATION_MARK_SYMBOL,
            Dingbats::LightVerticalBar => LIGHT_VERTICAL_BAR,
            Dingbats::MediumVerticalBar => MEDIUM_VERTICAL_BAR,
            Dingbats::HeavyVerticalBar => HEAVY_VERTICAL_BAR,
            Dingbats::HeavySingleTurnedCommaQuotationMarkOrnament => HEAVY_SINGLE_TURNED_COMMA_QUOTATION_MARK_ORNAMENT,
            Dingbats::HeavySingleCommaQuotationMarkOrnament => HEAVY_SINGLE_COMMA_QUOTATION_MARK_ORNAMENT,
            Dingbats::HeavyDoubleTurnedCommaQuotationMarkOrnament => HEAVY_DOUBLE_TURNED_COMMA_QUOTATION_MARK_ORNAMENT,
            Dingbats::HeavyDoubleCommaQuotationMarkOrnament => HEAVY_DOUBLE_COMMA_QUOTATION_MARK_ORNAMENT,
            Dingbats::HeavyLowSingleCommaQuotationMarkOrnament => HEAVY_LOW_SINGLE_COMMA_QUOTATION_MARK_ORNAMENT,
            Dingbats::HeavyLowDoubleCommaQuotationMarkOrnament => HEAVY_LOW_DOUBLE_COMMA_QUOTATION_MARK_ORNAMENT,
            Dingbats::CurvedStemParagraphSignOrnament => CURVED_STEM_PARAGRAPH_SIGN_ORNAMENT,
            Dingbats::HeavyExclamationMarkOrnament => HEAVY_EXCLAMATION_MARK_ORNAMENT,
            Dingbats::HeavyHeartExclamationMarkOrnament => HEAVY_HEART_EXCLAMATION_MARK_ORNAMENT,
            Dingbats::HeavyBlackHeart => HEAVY_BLACK_HEART,
            Dingbats::RotatedHeavyBlackHeartBullet => ROTATED_HEAVY_BLACK_HEART_BULLET,
            Dingbats::FloralHeart => FLORAL_HEART,
            Dingbats::RotatedFloralHeartBullet => ROTATED_FLORAL_HEART_BULLET,
            Dingbats::MediumLeftParenthesisOrnament => MEDIUM_LEFT_PARENTHESIS_ORNAMENT,
            Dingbats::MediumRightParenthesisOrnament => MEDIUM_RIGHT_PARENTHESIS_ORNAMENT,
            Dingbats::MediumFlattenedLeftParenthesisOrnament => MEDIUM_FLATTENED_LEFT_PARENTHESIS_ORNAMENT,
            Dingbats::MediumFlattenedRightParenthesisOrnament => MEDIUM_FLATTENED_RIGHT_PARENTHESIS_ORNAMENT,
            Dingbats::MediumLeftDashPointingAngleBracketOrnament => MEDIUM_LEFT_DASH_POINTING_ANGLE_BRACKET_ORNAMENT,
            Dingbats::MediumRightDashPointingAngleBracketOrnament => MEDIUM_RIGHT_DASH_POINTING_ANGLE_BRACKET_ORNAMENT,
            Dingbats::HeavyLeftDashPointingAngleQuotationMarkOrnament => HEAVY_LEFT_DASH_POINTING_ANGLE_QUOTATION_MARK_ORNAMENT,
            Dingbats::HeavyRightDashPointingAngleQuotationMarkOrnament => HEAVY_RIGHT_DASH_POINTING_ANGLE_QUOTATION_MARK_ORNAMENT,
            Dingbats::HeavyLeftDashPointingAngleBracketOrnament => HEAVY_LEFT_DASH_POINTING_ANGLE_BRACKET_ORNAMENT,
            Dingbats::HeavyRightDashPointingAngleBracketOrnament => HEAVY_RIGHT_DASH_POINTING_ANGLE_BRACKET_ORNAMENT,
            Dingbats::LightLeftTortoiseShellBracketOrnament => LIGHT_LEFT_TORTOISE_SHELL_BRACKET_ORNAMENT,
            Dingbats::LightRightTortoiseShellBracketOrnament => LIGHT_RIGHT_TORTOISE_SHELL_BRACKET_ORNAMENT,
            Dingbats::MediumLeftCurlyBracketOrnament => MEDIUM_LEFT_CURLY_BRACKET_ORNAMENT,
            Dingbats::MediumRightCurlyBracketOrnament => MEDIUM_RIGHT_CURLY_BRACKET_ORNAMENT,
            Dingbats::DingbatNegativeCircledDigitOne => DINGBAT_NEGATIVE_CIRCLED_DIGIT_ONE,
            Dingbats::DingbatNegativeCircledDigitTwo => DINGBAT_NEGATIVE_CIRCLED_DIGIT_TWO,
            Dingbats::DingbatNegativeCircledDigitThree => DINGBAT_NEGATIVE_CIRCLED_DIGIT_THREE,
            Dingbats::DingbatNegativeCircledDigitFour => DINGBAT_NEGATIVE_CIRCLED_DIGIT_FOUR,
            Dingbats::DingbatNegativeCircledDigitFive => DINGBAT_NEGATIVE_CIRCLED_DIGIT_FIVE,
            Dingbats::DingbatNegativeCircledDigitSix => DINGBAT_NEGATIVE_CIRCLED_DIGIT_SIX,
            Dingbats::DingbatNegativeCircledDigitSeven => DINGBAT_NEGATIVE_CIRCLED_DIGIT_SEVEN,
            Dingbats::DingbatNegativeCircledDigitEight => DINGBAT_NEGATIVE_CIRCLED_DIGIT_EIGHT,
            Dingbats::DingbatNegativeCircledDigitNine => DINGBAT_NEGATIVE_CIRCLED_DIGIT_NINE,
            Dingbats::DingbatNegativeCircledNumberTen => DINGBAT_NEGATIVE_CIRCLED_NUMBER_TEN,
            Dingbats::DingbatCircledSansDashSerifDigitOne => DINGBAT_CIRCLED_SANS_DASH_SERIF_DIGIT_ONE,
            Dingbats::DingbatCircledSansDashSerifDigitTwo => DINGBAT_CIRCLED_SANS_DASH_SERIF_DIGIT_TWO,
            Dingbats::DingbatCircledSansDashSerifDigitThree => DINGBAT_CIRCLED_SANS_DASH_SERIF_DIGIT_THREE,
            Dingbats::DingbatCircledSansDashSerifDigitFour => DINGBAT_CIRCLED_SANS_DASH_SERIF_DIGIT_FOUR,
            Dingbats::DingbatCircledSansDashSerifDigitFive => DINGBAT_CIRCLED_SANS_DASH_SERIF_DIGIT_FIVE,
            Dingbats::DingbatCircledSansDashSerifDigitSix => DINGBAT_CIRCLED_SANS_DASH_SERIF_DIGIT_SIX,
            Dingbats::DingbatCircledSansDashSerifDigitSeven => DINGBAT_CIRCLED_SANS_DASH_SERIF_DIGIT_SEVEN,
            Dingbats::DingbatCircledSansDashSerifDigitEight => DINGBAT_CIRCLED_SANS_DASH_SERIF_DIGIT_EIGHT,
            Dingbats::DingbatCircledSansDashSerifDigitNine => DINGBAT_CIRCLED_SANS_DASH_SERIF_DIGIT_NINE,
            Dingbats::DingbatCircledSansDashSerifNumberTen => DINGBAT_CIRCLED_SANS_DASH_SERIF_NUMBER_TEN,
            Dingbats::DingbatNegativeCircledSansDashSerifDigitOne => DINGBAT_NEGATIVE_CIRCLED_SANS_DASH_SERIF_DIGIT_ONE,
            Dingbats::DingbatNegativeCircledSansDashSerifDigitTwo => DINGBAT_NEGATIVE_CIRCLED_SANS_DASH_SERIF_DIGIT_TWO,
            Dingbats::DingbatNegativeCircledSansDashSerifDigitThree => DINGBAT_NEGATIVE_CIRCLED_SANS_DASH_SERIF_DIGIT_THREE,
            Dingbats::DingbatNegativeCircledSansDashSerifDigitFour => DINGBAT_NEGATIVE_CIRCLED_SANS_DASH_SERIF_DIGIT_FOUR,
            Dingbats::DingbatNegativeCircledSansDashSerifDigitFive => DINGBAT_NEGATIVE_CIRCLED_SANS_DASH_SERIF_DIGIT_FIVE,
            Dingbats::DingbatNegativeCircledSansDashSerifDigitSix => DINGBAT_NEGATIVE_CIRCLED_SANS_DASH_SERIF_DIGIT_SIX,
            Dingbats::DingbatNegativeCircledSansDashSerifDigitSeven => DINGBAT_NEGATIVE_CIRCLED_SANS_DASH_SERIF_DIGIT_SEVEN,
            Dingbats::DingbatNegativeCircledSansDashSerifDigitEight => DINGBAT_NEGATIVE_CIRCLED_SANS_DASH_SERIF_DIGIT_EIGHT,
            Dingbats::DingbatNegativeCircledSansDashSerifDigitNine => DINGBAT_NEGATIVE_CIRCLED_SANS_DASH_SERIF_DIGIT_NINE,
            Dingbats::DingbatNegativeCircledSansDashSerifNumberTen => DINGBAT_NEGATIVE_CIRCLED_SANS_DASH_SERIF_NUMBER_TEN,
            Dingbats::HeavyWideDashHeadedRightwardsArrow => HEAVY_WIDE_DASH_HEADED_RIGHTWARDS_ARROW,
            Dingbats::HeavyPlusSign => HEAVY_PLUS_SIGN,
            Dingbats::HeavyMinusSign => HEAVY_MINUS_SIGN,
            Dingbats::HeavyDivisionSign => HEAVY_DIVISION_SIGN,
            Dingbats::HeavySouthEastArrow => HEAVY_SOUTH_EAST_ARROW,
            Dingbats::HeavyRightwardsArrow => HEAVY_RIGHTWARDS_ARROW,
            Dingbats::HeavyNorthEastArrow => HEAVY_NORTH_EAST_ARROW,
            Dingbats::DraftingPointRightwardsArrow => DRAFTING_POINT_RIGHTWARDS_ARROW,
            Dingbats::HeavyRoundDashTippedRightwardsArrow => HEAVY_ROUND_DASH_TIPPED_RIGHTWARDS_ARROW,
            Dingbats::TriangleDashHeadedRightwardsArrow => TRIANGLE_DASH_HEADED_RIGHTWARDS_ARROW,
            Dingbats::HeavyTriangleDashHeadedRightwardsArrow => HEAVY_TRIANGLE_DASH_HEADED_RIGHTWARDS_ARROW,
            Dingbats::DashedTriangleDashHeadedRightwardsArrow => DASHED_TRIANGLE_DASH_HEADED_RIGHTWARDS_ARROW,
            Dingbats::HeavyDashedTriangleDashHeadedRightwardsArrow => HEAVY_DASHED_TRIANGLE_DASH_HEADED_RIGHTWARDS_ARROW,
            Dingbats::BlackRightwardsArrow => BLACK_RIGHTWARDS_ARROW,
            Dingbats::ThreeDashDTopDashLightedRightwardsArrowhead => THREE_DASH_D_TOP_DASH_LIGHTED_RIGHTWARDS_ARROWHEAD,
            Dingbats::ThreeDashDBottomDashLightedRightwardsArrowhead => THREE_DASH_D_BOTTOM_DASH_LIGHTED_RIGHTWARDS_ARROWHEAD,
            Dingbats::BlackRightwardsArrowhead => BLACK_RIGHTWARDS_ARROWHEAD,
            Dingbats::HeavyBlackCurvedDownwardsAndRightwardsArrow => HEAVY_BLACK_CURVED_DOWNWARDS_AND_RIGHTWARDS_ARROW,
            Dingbats::HeavyBlackCurvedUpwardsAndRightwardsArrow => HEAVY_BLACK_CURVED_UPWARDS_AND_RIGHTWARDS_ARROW,
            Dingbats::SquatBlackRightwardsArrow => SQUAT_BLACK_RIGHTWARDS_ARROW,
            Dingbats::HeavyConcaveDashPointedBlackRightwardsArrow => HEAVY_CONCAVE_DASH_POINTED_BLACK_RIGHTWARDS_ARROW,
            Dingbats::RightDashShadedWhiteRightwardsArrow => RIGHT_DASH_SHADED_WHITE_RIGHTWARDS_ARROW,
            Dingbats::LeftDashShadedWhiteRightwardsArrow => LEFT_DASH_SHADED_WHITE_RIGHTWARDS_ARROW,
            Dingbats::BackDashTiltedShadowedWhiteRightwardsArrow => BACK_DASH_TILTED_SHADOWED_WHITE_RIGHTWARDS_ARROW,
            Dingbats::FrontDashTiltedShadowedWhiteRightwardsArrow => FRONT_DASH_TILTED_SHADOWED_WHITE_RIGHTWARDS_ARROW,
            Dingbats::HeavyLowerRightDashShadowedWhiteRightwardsArrow => HEAVY_LOWER_RIGHT_DASH_SHADOWED_WHITE_RIGHTWARDS_ARROW,
            Dingbats::HeavyUpperRightDashShadowedWhiteRightwardsArrow => HEAVY_UPPER_RIGHT_DASH_SHADOWED_WHITE_RIGHTWARDS_ARROW,
            Dingbats::NotchedLowerRightDashShadowedWhiteRightwardsArrow => NOTCHED_LOWER_RIGHT_DASH_SHADOWED_WHITE_RIGHTWARDS_ARROW,
            Dingbats::CurlyLoop => CURLY_LOOP,
            Dingbats::NotchedUpperRightDashShadowedWhiteRightwardsArrow => NOTCHED_UPPER_RIGHT_DASH_SHADOWED_WHITE_RIGHTWARDS_ARROW,
            Dingbats::CircledHeavyWhiteRightwardsArrow => CIRCLED_HEAVY_WHITE_RIGHTWARDS_ARROW,
            Dingbats::WhiteDashFeatheredRightwardsArrow => WHITE_DASH_FEATHERED_RIGHTWARDS_ARROW,
            Dingbats::BlackDashFeatheredSouthEastArrow => BLACK_DASH_FEATHERED_SOUTH_EAST_ARROW,
            Dingbats::BlackDashFeatheredRightwardsArrow => BLACK_DASH_FEATHERED_RIGHTWARDS_ARROW,
            Dingbats::BlackDashFeatheredNorthEastArrow => BLACK_DASH_FEATHERED_NORTH_EAST_ARROW,
            Dingbats::HeavyBlackDashFeatheredSouthEastArrow => HEAVY_BLACK_DASH_FEATHERED_SOUTH_EAST_ARROW,
            Dingbats::HeavyBlackDashFeatheredRightwardsArrow => HEAVY_BLACK_DASH_FEATHERED_RIGHTWARDS_ARROW,
            Dingbats::HeavyBlackDashFeatheredNorthEastArrow => HEAVY_BLACK_DASH_FEATHERED_NORTH_EAST_ARROW,
            Dingbats::TeardropDashBarbedRightwardsArrow => TEARDROP_DASH_BARBED_RIGHTWARDS_ARROW,
            Dingbats::HeavyTeardropDashShankedRightwardsArrow => HEAVY_TEARDROP_DASH_SHANKED_RIGHTWARDS_ARROW,
            Dingbats::WedgeDashTailedRightwardsArrow => WEDGE_DASH_TAILED_RIGHTWARDS_ARROW,
            Dingbats::HeavyWedgeDashTailedRightwardsArrow => HEAVY_WEDGE_DASH_TAILED_RIGHTWARDS_ARROW,
            Dingbats::OpenDashOutlinedRightwardsArrow => OPEN_DASH_OUTLINED_RIGHTWARDS_ARROW,
        }
    }
}

impl std::convert::TryFrom<char> for Dingbats {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            BLACK_SAFETY_SCISSORS => Ok(Dingbats::BlackSafetyScissors),
            UPPER_BLADE_SCISSORS => Ok(Dingbats::UpperBladeScissors),
            BLACK_SCISSORS => Ok(Dingbats::BlackScissors),
            LOWER_BLADE_SCISSORS => Ok(Dingbats::LowerBladeScissors),
            WHITE_SCISSORS => Ok(Dingbats::WhiteScissors),
            WHITE_HEAVY_CHECK_MARK => Ok(Dingbats::WhiteHeavyCheckMark),
            TELEPHONE_LOCATION_SIGN => Ok(Dingbats::TelephoneLocationSign),
            TAPE_DRIVE => Ok(Dingbats::TapeDrive),
            AIRPLANE => Ok(Dingbats::Airplane),
            ENVELOPE => Ok(Dingbats::Envelope),
            RAISED_FIST => Ok(Dingbats::RaisedFist),
            RAISED_HAND => Ok(Dingbats::RaisedHand),
            VICTORY_HAND => Ok(Dingbats::VictoryHand),
            WRITING_HAND => Ok(Dingbats::WritingHand),
            LOWER_RIGHT_PENCIL => Ok(Dingbats::LowerRightPencil),
            PENCIL => Ok(Dingbats::Pencil),
            UPPER_RIGHT_PENCIL => Ok(Dingbats::UpperRightPencil),
            WHITE_NIB => Ok(Dingbats::WhiteNib),
            BLACK_NIB => Ok(Dingbats::BlackNib),
            CHECK_MARK => Ok(Dingbats::CheckMark),
            HEAVY_CHECK_MARK => Ok(Dingbats::HeavyCheckMark),
            MULTIPLICATION_X => Ok(Dingbats::MultiplicationX),
            HEAVY_MULTIPLICATION_X => Ok(Dingbats::HeavyMultiplicationX),
            BALLOT_X => Ok(Dingbats::BallotX),
            HEAVY_BALLOT_X => Ok(Dingbats::HeavyBallotX),
            OUTLINED_GREEK_CROSS => Ok(Dingbats::OutlinedGreekCross),
            HEAVY_GREEK_CROSS => Ok(Dingbats::HeavyGreekCross),
            OPEN_CENTRE_CROSS => Ok(Dingbats::OpenCentreCross),
            HEAVY_OPEN_CENTRE_CROSS => Ok(Dingbats::HeavyOpenCentreCross),
            LATIN_CROSS => Ok(Dingbats::LatinCross),
            SHADOWED_WHITE_LATIN_CROSS => Ok(Dingbats::ShadowedWhiteLatinCross),
            OUTLINED_LATIN_CROSS => Ok(Dingbats::OutlinedLatinCross),
            MALTESE_CROSS => Ok(Dingbats::MalteseCross),
            STAR_OF_DAVID => Ok(Dingbats::StarOfDavid),
            FOUR_TEARDROP_DASH_SPOKED_ASTERISK => Ok(Dingbats::FourTeardropDashSpokedAsterisk),
            FOUR_BALLOON_DASH_SPOKED_ASTERISK => Ok(Dingbats::FourBalloonDashSpokedAsterisk),
            HEAVY_FOUR_BALLOON_DASH_SPOKED_ASTERISK => Ok(Dingbats::HeavyFourBalloonDashSpokedAsterisk),
            FOUR_CLUB_DASH_SPOKED_ASTERISK => Ok(Dingbats::FourClubDashSpokedAsterisk),
            BLACK_FOUR_POINTED_STAR => Ok(Dingbats::BlackFourPointedStar),
            WHITE_FOUR_POINTED_STAR => Ok(Dingbats::WhiteFourPointedStar),
            SPARKLES => Ok(Dingbats::Sparkles),
            STRESS_OUTLINED_WHITE_STAR => Ok(Dingbats::StressOutlinedWhiteStar),
            CIRCLED_WHITE_STAR => Ok(Dingbats::CircledWhiteStar),
            OPEN_CENTRE_BLACK_STAR => Ok(Dingbats::OpenCentreBlackStar),
            BLACK_CENTRE_WHITE_STAR => Ok(Dingbats::BlackCentreWhiteStar),
            OUTLINED_BLACK_STAR => Ok(Dingbats::OutlinedBlackStar),
            HEAVY_OUTLINED_BLACK_STAR => Ok(Dingbats::HeavyOutlinedBlackStar),
            PINWHEEL_STAR => Ok(Dingbats::PinwheelStar),
            SHADOWED_WHITE_STAR => Ok(Dingbats::ShadowedWhiteStar),
            HEAVY_ASTERISK => Ok(Dingbats::HeavyAsterisk),
            OPEN_CENTRE_ASTERISK => Ok(Dingbats::OpenCentreAsterisk),
            EIGHT_SPOKED_ASTERISK => Ok(Dingbats::EightSpokedAsterisk),
            EIGHT_POINTED_BLACK_STAR => Ok(Dingbats::EightPointedBlackStar),
            EIGHT_POINTED_PINWHEEL_STAR => Ok(Dingbats::EightPointedPinwheelStar),
            SIX_POINTED_BLACK_STAR => Ok(Dingbats::SixPointedBlackStar),
            EIGHT_POINTED_RECTILINEAR_BLACK_STAR => Ok(Dingbats::EightPointedRectilinearBlackStar),
            HEAVY_EIGHT_POINTED_RECTILINEAR_BLACK_STAR => Ok(Dingbats::HeavyEightPointedRectilinearBlackStar),
            TWELVE_POINTED_BLACK_STAR => Ok(Dingbats::TwelvePointedBlackStar),
            SIXTEEN_POINTED_ASTERISK => Ok(Dingbats::SixteenPointedAsterisk),
            TEARDROP_DASH_SPOKED_ASTERISK => Ok(Dingbats::TeardropDashSpokedAsterisk),
            OPEN_CENTRE_TEARDROP_DASH_SPOKED_ASTERISK => Ok(Dingbats::OpenCentreTeardropDashSpokedAsterisk),
            HEAVY_TEARDROP_DASH_SPOKED_ASTERISK => Ok(Dingbats::HeavyTeardropDashSpokedAsterisk),
            SIX_PETALLED_BLACK_AND_WHITE_FLORETTE => Ok(Dingbats::SixPetalledBlackAndWhiteFlorette),
            BLACK_FLORETTE => Ok(Dingbats::BlackFlorette),
            WHITE_FLORETTE => Ok(Dingbats::WhiteFlorette),
            EIGHT_PETALLED_OUTLINED_BLACK_FLORETTE => Ok(Dingbats::EightPetalledOutlinedBlackFlorette),
            CIRCLED_OPEN_CENTRE_EIGHT_POINTED_STAR => Ok(Dingbats::CircledOpenCentreEightPointedStar),
            HEAVY_TEARDROP_DASH_SPOKED_PINWHEEL_ASTERISK => Ok(Dingbats::HeavyTeardropDashSpokedPinwheelAsterisk),
            SNOWFLAKE => Ok(Dingbats::Snowflake),
            TIGHT_TRIFOLIATE_SNOWFLAKE => Ok(Dingbats::TightTrifoliateSnowflake),
            HEAVY_CHEVRON_SNOWFLAKE => Ok(Dingbats::HeavyChevronSnowflake),
            SPARKLE => Ok(Dingbats::Sparkle),
            HEAVY_SPARKLE => Ok(Dingbats::HeavySparkle),
            BALLOON_DASH_SPOKED_ASTERISK => Ok(Dingbats::BalloonDashSpokedAsterisk),
            EIGHT_TEARDROP_DASH_SPOKED_PROPELLER_ASTERISK => Ok(Dingbats::EightTeardropDashSpokedPropellerAsterisk),
            HEAVY_EIGHT_TEARDROP_DASH_SPOKED_PROPELLER_ASTERISK => Ok(Dingbats::HeavyEightTeardropDashSpokedPropellerAsterisk),
            CROSS_MARK => Ok(Dingbats::CrossMark),
            SHADOWED_WHITE_CIRCLE => Ok(Dingbats::ShadowedWhiteCircle),
            NEGATIVE_SQUARED_CROSS_MARK => Ok(Dingbats::NegativeSquaredCrossMark),
            LOWER_RIGHT_DROP_DASH_SHADOWED_WHITE_SQUARE => Ok(Dingbats::LowerRightDropDashShadowedWhiteSquare),
            UPPER_RIGHT_DROP_DASH_SHADOWED_WHITE_SQUARE => Ok(Dingbats::UpperRightDropDashShadowedWhiteSquare),
            LOWER_RIGHT_SHADOWED_WHITE_SQUARE => Ok(Dingbats::LowerRightShadowedWhiteSquare),
            UPPER_RIGHT_SHADOWED_WHITE_SQUARE => Ok(Dingbats::UpperRightShadowedWhiteSquare),
            BLACK_QUESTION_MARK_ORNAMENT => Ok(Dingbats::BlackQuestionMarkOrnament),
            WHITE_QUESTION_MARK_ORNAMENT => Ok(Dingbats::WhiteQuestionMarkOrnament),
            WHITE_EXCLAMATION_MARK_ORNAMENT => Ok(Dingbats::WhiteExclamationMarkOrnament),
            BLACK_DIAMOND_MINUS_WHITE_X => Ok(Dingbats::BlackDiamondMinusWhiteX),
            HEAVY_EXCLAMATION_MARK_SYMBOL => Ok(Dingbats::HeavyExclamationMarkSymbol),
            LIGHT_VERTICAL_BAR => Ok(Dingbats::LightVerticalBar),
            MEDIUM_VERTICAL_BAR => Ok(Dingbats::MediumVerticalBar),
            HEAVY_VERTICAL_BAR => Ok(Dingbats::HeavyVerticalBar),
            HEAVY_SINGLE_TURNED_COMMA_QUOTATION_MARK_ORNAMENT => Ok(Dingbats::HeavySingleTurnedCommaQuotationMarkOrnament),
            HEAVY_SINGLE_COMMA_QUOTATION_MARK_ORNAMENT => Ok(Dingbats::HeavySingleCommaQuotationMarkOrnament),
            HEAVY_DOUBLE_TURNED_COMMA_QUOTATION_MARK_ORNAMENT => Ok(Dingbats::HeavyDoubleTurnedCommaQuotationMarkOrnament),
            HEAVY_DOUBLE_COMMA_QUOTATION_MARK_ORNAMENT => Ok(Dingbats::HeavyDoubleCommaQuotationMarkOrnament),
            HEAVY_LOW_SINGLE_COMMA_QUOTATION_MARK_ORNAMENT => Ok(Dingbats::HeavyLowSingleCommaQuotationMarkOrnament),
            HEAVY_LOW_DOUBLE_COMMA_QUOTATION_MARK_ORNAMENT => Ok(Dingbats::HeavyLowDoubleCommaQuotationMarkOrnament),
            CURVED_STEM_PARAGRAPH_SIGN_ORNAMENT => Ok(Dingbats::CurvedStemParagraphSignOrnament),
            HEAVY_EXCLAMATION_MARK_ORNAMENT => Ok(Dingbats::HeavyExclamationMarkOrnament),
            HEAVY_HEART_EXCLAMATION_MARK_ORNAMENT => Ok(Dingbats::HeavyHeartExclamationMarkOrnament),
            HEAVY_BLACK_HEART => Ok(Dingbats::HeavyBlackHeart),
            ROTATED_HEAVY_BLACK_HEART_BULLET => Ok(Dingbats::RotatedHeavyBlackHeartBullet),
            FLORAL_HEART => Ok(Dingbats::FloralHeart),
            ROTATED_FLORAL_HEART_BULLET => Ok(Dingbats::RotatedFloralHeartBullet),
            MEDIUM_LEFT_PARENTHESIS_ORNAMENT => Ok(Dingbats::MediumLeftParenthesisOrnament),
            MEDIUM_RIGHT_PARENTHESIS_ORNAMENT => Ok(Dingbats::MediumRightParenthesisOrnament),
            MEDIUM_FLATTENED_LEFT_PARENTHESIS_ORNAMENT => Ok(Dingbats::MediumFlattenedLeftParenthesisOrnament),
            MEDIUM_FLATTENED_RIGHT_PARENTHESIS_ORNAMENT => Ok(Dingbats::MediumFlattenedRightParenthesisOrnament),
            MEDIUM_LEFT_DASH_POINTING_ANGLE_BRACKET_ORNAMENT => Ok(Dingbats::MediumLeftDashPointingAngleBracketOrnament),
            MEDIUM_RIGHT_DASH_POINTING_ANGLE_BRACKET_ORNAMENT => Ok(Dingbats::MediumRightDashPointingAngleBracketOrnament),
            HEAVY_LEFT_DASH_POINTING_ANGLE_QUOTATION_MARK_ORNAMENT => Ok(Dingbats::HeavyLeftDashPointingAngleQuotationMarkOrnament),
            HEAVY_RIGHT_DASH_POINTING_ANGLE_QUOTATION_MARK_ORNAMENT => Ok(Dingbats::HeavyRightDashPointingAngleQuotationMarkOrnament),
            HEAVY_LEFT_DASH_POINTING_ANGLE_BRACKET_ORNAMENT => Ok(Dingbats::HeavyLeftDashPointingAngleBracketOrnament),
            HEAVY_RIGHT_DASH_POINTING_ANGLE_BRACKET_ORNAMENT => Ok(Dingbats::HeavyRightDashPointingAngleBracketOrnament),
            LIGHT_LEFT_TORTOISE_SHELL_BRACKET_ORNAMENT => Ok(Dingbats::LightLeftTortoiseShellBracketOrnament),
            LIGHT_RIGHT_TORTOISE_SHELL_BRACKET_ORNAMENT => Ok(Dingbats::LightRightTortoiseShellBracketOrnament),
            MEDIUM_LEFT_CURLY_BRACKET_ORNAMENT => Ok(Dingbats::MediumLeftCurlyBracketOrnament),
            MEDIUM_RIGHT_CURLY_BRACKET_ORNAMENT => Ok(Dingbats::MediumRightCurlyBracketOrnament),
            DINGBAT_NEGATIVE_CIRCLED_DIGIT_ONE => Ok(Dingbats::DingbatNegativeCircledDigitOne),
            DINGBAT_NEGATIVE_CIRCLED_DIGIT_TWO => Ok(Dingbats::DingbatNegativeCircledDigitTwo),
            DINGBAT_NEGATIVE_CIRCLED_DIGIT_THREE => Ok(Dingbats::DingbatNegativeCircledDigitThree),
            DINGBAT_NEGATIVE_CIRCLED_DIGIT_FOUR => Ok(Dingbats::DingbatNegativeCircledDigitFour),
            DINGBAT_NEGATIVE_CIRCLED_DIGIT_FIVE => Ok(Dingbats::DingbatNegativeCircledDigitFive),
            DINGBAT_NEGATIVE_CIRCLED_DIGIT_SIX => Ok(Dingbats::DingbatNegativeCircledDigitSix),
            DINGBAT_NEGATIVE_CIRCLED_DIGIT_SEVEN => Ok(Dingbats::DingbatNegativeCircledDigitSeven),
            DINGBAT_NEGATIVE_CIRCLED_DIGIT_EIGHT => Ok(Dingbats::DingbatNegativeCircledDigitEight),
            DINGBAT_NEGATIVE_CIRCLED_DIGIT_NINE => Ok(Dingbats::DingbatNegativeCircledDigitNine),
            DINGBAT_NEGATIVE_CIRCLED_NUMBER_TEN => Ok(Dingbats::DingbatNegativeCircledNumberTen),
            DINGBAT_CIRCLED_SANS_DASH_SERIF_DIGIT_ONE => Ok(Dingbats::DingbatCircledSansDashSerifDigitOne),
            DINGBAT_CIRCLED_SANS_DASH_SERIF_DIGIT_TWO => Ok(Dingbats::DingbatCircledSansDashSerifDigitTwo),
            DINGBAT_CIRCLED_SANS_DASH_SERIF_DIGIT_THREE => Ok(Dingbats::DingbatCircledSansDashSerifDigitThree),
            DINGBAT_CIRCLED_SANS_DASH_SERIF_DIGIT_FOUR => Ok(Dingbats::DingbatCircledSansDashSerifDigitFour),
            DINGBAT_CIRCLED_SANS_DASH_SERIF_DIGIT_FIVE => Ok(Dingbats::DingbatCircledSansDashSerifDigitFive),
            DINGBAT_CIRCLED_SANS_DASH_SERIF_DIGIT_SIX => Ok(Dingbats::DingbatCircledSansDashSerifDigitSix),
            DINGBAT_CIRCLED_SANS_DASH_SERIF_DIGIT_SEVEN => Ok(Dingbats::DingbatCircledSansDashSerifDigitSeven),
            DINGBAT_CIRCLED_SANS_DASH_SERIF_DIGIT_EIGHT => Ok(Dingbats::DingbatCircledSansDashSerifDigitEight),
            DINGBAT_CIRCLED_SANS_DASH_SERIF_DIGIT_NINE => Ok(Dingbats::DingbatCircledSansDashSerifDigitNine),
            DINGBAT_CIRCLED_SANS_DASH_SERIF_NUMBER_TEN => Ok(Dingbats::DingbatCircledSansDashSerifNumberTen),
            DINGBAT_NEGATIVE_CIRCLED_SANS_DASH_SERIF_DIGIT_ONE => Ok(Dingbats::DingbatNegativeCircledSansDashSerifDigitOne),
            DINGBAT_NEGATIVE_CIRCLED_SANS_DASH_SERIF_DIGIT_TWO => Ok(Dingbats::DingbatNegativeCircledSansDashSerifDigitTwo),
            DINGBAT_NEGATIVE_CIRCLED_SANS_DASH_SERIF_DIGIT_THREE => Ok(Dingbats::DingbatNegativeCircledSansDashSerifDigitThree),
            DINGBAT_NEGATIVE_CIRCLED_SANS_DASH_SERIF_DIGIT_FOUR => Ok(Dingbats::DingbatNegativeCircledSansDashSerifDigitFour),
            DINGBAT_NEGATIVE_CIRCLED_SANS_DASH_SERIF_DIGIT_FIVE => Ok(Dingbats::DingbatNegativeCircledSansDashSerifDigitFive),
            DINGBAT_NEGATIVE_CIRCLED_SANS_DASH_SERIF_DIGIT_SIX => Ok(Dingbats::DingbatNegativeCircledSansDashSerifDigitSix),
            DINGBAT_NEGATIVE_CIRCLED_SANS_DASH_SERIF_DIGIT_SEVEN => Ok(Dingbats::DingbatNegativeCircledSansDashSerifDigitSeven),
            DINGBAT_NEGATIVE_CIRCLED_SANS_DASH_SERIF_DIGIT_EIGHT => Ok(Dingbats::DingbatNegativeCircledSansDashSerifDigitEight),
            DINGBAT_NEGATIVE_CIRCLED_SANS_DASH_SERIF_DIGIT_NINE => Ok(Dingbats::DingbatNegativeCircledSansDashSerifDigitNine),
            DINGBAT_NEGATIVE_CIRCLED_SANS_DASH_SERIF_NUMBER_TEN => Ok(Dingbats::DingbatNegativeCircledSansDashSerifNumberTen),
            HEAVY_WIDE_DASH_HEADED_RIGHTWARDS_ARROW => Ok(Dingbats::HeavyWideDashHeadedRightwardsArrow),
            HEAVY_PLUS_SIGN => Ok(Dingbats::HeavyPlusSign),
            HEAVY_MINUS_SIGN => Ok(Dingbats::HeavyMinusSign),
            HEAVY_DIVISION_SIGN => Ok(Dingbats::HeavyDivisionSign),
            HEAVY_SOUTH_EAST_ARROW => Ok(Dingbats::HeavySouthEastArrow),
            HEAVY_RIGHTWARDS_ARROW => Ok(Dingbats::HeavyRightwardsArrow),
            HEAVY_NORTH_EAST_ARROW => Ok(Dingbats::HeavyNorthEastArrow),
            DRAFTING_POINT_RIGHTWARDS_ARROW => Ok(Dingbats::DraftingPointRightwardsArrow),
            HEAVY_ROUND_DASH_TIPPED_RIGHTWARDS_ARROW => Ok(Dingbats::HeavyRoundDashTippedRightwardsArrow),
            TRIANGLE_DASH_HEADED_RIGHTWARDS_ARROW => Ok(Dingbats::TriangleDashHeadedRightwardsArrow),
            HEAVY_TRIANGLE_DASH_HEADED_RIGHTWARDS_ARROW => Ok(Dingbats::HeavyTriangleDashHeadedRightwardsArrow),
            DASHED_TRIANGLE_DASH_HEADED_RIGHTWARDS_ARROW => Ok(Dingbats::DashedTriangleDashHeadedRightwardsArrow),
            HEAVY_DASHED_TRIANGLE_DASH_HEADED_RIGHTWARDS_ARROW => Ok(Dingbats::HeavyDashedTriangleDashHeadedRightwardsArrow),
            BLACK_RIGHTWARDS_ARROW => Ok(Dingbats::BlackRightwardsArrow),
            THREE_DASH_D_TOP_DASH_LIGHTED_RIGHTWARDS_ARROWHEAD => Ok(Dingbats::ThreeDashDTopDashLightedRightwardsArrowhead),
            THREE_DASH_D_BOTTOM_DASH_LIGHTED_RIGHTWARDS_ARROWHEAD => Ok(Dingbats::ThreeDashDBottomDashLightedRightwardsArrowhead),
            BLACK_RIGHTWARDS_ARROWHEAD => Ok(Dingbats::BlackRightwardsArrowhead),
            HEAVY_BLACK_CURVED_DOWNWARDS_AND_RIGHTWARDS_ARROW => Ok(Dingbats::HeavyBlackCurvedDownwardsAndRightwardsArrow),
            HEAVY_BLACK_CURVED_UPWARDS_AND_RIGHTWARDS_ARROW => Ok(Dingbats::HeavyBlackCurvedUpwardsAndRightwardsArrow),
            SQUAT_BLACK_RIGHTWARDS_ARROW => Ok(Dingbats::SquatBlackRightwardsArrow),
            HEAVY_CONCAVE_DASH_POINTED_BLACK_RIGHTWARDS_ARROW => Ok(Dingbats::HeavyConcaveDashPointedBlackRightwardsArrow),
            RIGHT_DASH_SHADED_WHITE_RIGHTWARDS_ARROW => Ok(Dingbats::RightDashShadedWhiteRightwardsArrow),
            LEFT_DASH_SHADED_WHITE_RIGHTWARDS_ARROW => Ok(Dingbats::LeftDashShadedWhiteRightwardsArrow),
            BACK_DASH_TILTED_SHADOWED_WHITE_RIGHTWARDS_ARROW => Ok(Dingbats::BackDashTiltedShadowedWhiteRightwardsArrow),
            FRONT_DASH_TILTED_SHADOWED_WHITE_RIGHTWARDS_ARROW => Ok(Dingbats::FrontDashTiltedShadowedWhiteRightwardsArrow),
            HEAVY_LOWER_RIGHT_DASH_SHADOWED_WHITE_RIGHTWARDS_ARROW => Ok(Dingbats::HeavyLowerRightDashShadowedWhiteRightwardsArrow),
            HEAVY_UPPER_RIGHT_DASH_SHADOWED_WHITE_RIGHTWARDS_ARROW => Ok(Dingbats::HeavyUpperRightDashShadowedWhiteRightwardsArrow),
            NOTCHED_LOWER_RIGHT_DASH_SHADOWED_WHITE_RIGHTWARDS_ARROW => Ok(Dingbats::NotchedLowerRightDashShadowedWhiteRightwardsArrow),
            CURLY_LOOP => Ok(Dingbats::CurlyLoop),
            NOTCHED_UPPER_RIGHT_DASH_SHADOWED_WHITE_RIGHTWARDS_ARROW => Ok(Dingbats::NotchedUpperRightDashShadowedWhiteRightwardsArrow),
            CIRCLED_HEAVY_WHITE_RIGHTWARDS_ARROW => Ok(Dingbats::CircledHeavyWhiteRightwardsArrow),
            WHITE_DASH_FEATHERED_RIGHTWARDS_ARROW => Ok(Dingbats::WhiteDashFeatheredRightwardsArrow),
            BLACK_DASH_FEATHERED_SOUTH_EAST_ARROW => Ok(Dingbats::BlackDashFeatheredSouthEastArrow),
            BLACK_DASH_FEATHERED_RIGHTWARDS_ARROW => Ok(Dingbats::BlackDashFeatheredRightwardsArrow),
            BLACK_DASH_FEATHERED_NORTH_EAST_ARROW => Ok(Dingbats::BlackDashFeatheredNorthEastArrow),
            HEAVY_BLACK_DASH_FEATHERED_SOUTH_EAST_ARROW => Ok(Dingbats::HeavyBlackDashFeatheredSouthEastArrow),
            HEAVY_BLACK_DASH_FEATHERED_RIGHTWARDS_ARROW => Ok(Dingbats::HeavyBlackDashFeatheredRightwardsArrow),
            HEAVY_BLACK_DASH_FEATHERED_NORTH_EAST_ARROW => Ok(Dingbats::HeavyBlackDashFeatheredNorthEastArrow),
            TEARDROP_DASH_BARBED_RIGHTWARDS_ARROW => Ok(Dingbats::TeardropDashBarbedRightwardsArrow),
            HEAVY_TEARDROP_DASH_SHANKED_RIGHTWARDS_ARROW => Ok(Dingbats::HeavyTeardropDashShankedRightwardsArrow),
            WEDGE_DASH_TAILED_RIGHTWARDS_ARROW => Ok(Dingbats::WedgeDashTailedRightwardsArrow),
            HEAVY_WEDGE_DASH_TAILED_RIGHTWARDS_ARROW => Ok(Dingbats::HeavyWedgeDashTailedRightwardsArrow),
            OPEN_DASH_OUTLINED_RIGHTWARDS_ARROW => Ok(Dingbats::OpenDashOutlinedRightwardsArrow),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Dingbats {
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

impl std::convert::TryFrom<u32> for Dingbats {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Dingbats {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Dingbats {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Dingbats::BlackSafetyScissors
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Dingbats::BlackSafetyScissors => "black safety scissors",
            Dingbats::UpperBladeScissors => "upper blade scissors",
            Dingbats::BlackScissors => "black scissors",
            Dingbats::LowerBladeScissors => "lower blade scissors",
            Dingbats::WhiteScissors => "white scissors",
            Dingbats::WhiteHeavyCheckMark => "white heavy check mark",
            Dingbats::TelephoneLocationSign => "telephone location sign",
            Dingbats::TapeDrive => "tape drive",
            Dingbats::Airplane => "airplane",
            Dingbats::Envelope => "envelope",
            Dingbats::RaisedFist => "raised fist",
            Dingbats::RaisedHand => "raised hand",
            Dingbats::VictoryHand => "victory hand",
            Dingbats::WritingHand => "writing hand",
            Dingbats::LowerRightPencil => "lower right pencil",
            Dingbats::Pencil => "pencil",
            Dingbats::UpperRightPencil => "upper right pencil",
            Dingbats::WhiteNib => "white nib",
            Dingbats::BlackNib => "black nib",
            Dingbats::CheckMark => "check mark",
            Dingbats::HeavyCheckMark => "heavy check mark",
            Dingbats::MultiplicationX => "multiplication x",
            Dingbats::HeavyMultiplicationX => "heavy multiplication x",
            Dingbats::BallotX => "ballot x",
            Dingbats::HeavyBallotX => "heavy ballot x",
            Dingbats::OutlinedGreekCross => "outlined greek cross",
            Dingbats::HeavyGreekCross => "heavy greek cross",
            Dingbats::OpenCentreCross => "open centre cross",
            Dingbats::HeavyOpenCentreCross => "heavy open centre cross",
            Dingbats::LatinCross => "latin cross",
            Dingbats::ShadowedWhiteLatinCross => "shadowed white latin cross",
            Dingbats::OutlinedLatinCross => "outlined latin cross",
            Dingbats::MalteseCross => "maltese cross",
            Dingbats::StarOfDavid => "star of david",
            Dingbats::FourTeardropDashSpokedAsterisk => "four teardrop-spoked asterisk",
            Dingbats::FourBalloonDashSpokedAsterisk => "four balloon-spoked asterisk",
            Dingbats::HeavyFourBalloonDashSpokedAsterisk => "heavy four balloon-spoked asterisk",
            Dingbats::FourClubDashSpokedAsterisk => "four club-spoked asterisk",
            Dingbats::BlackFourPointedStar => "black four pointed star",
            Dingbats::WhiteFourPointedStar => "white four pointed star",
            Dingbats::Sparkles => "sparkles",
            Dingbats::StressOutlinedWhiteStar => "stress outlined white star",
            Dingbats::CircledWhiteStar => "circled white star",
            Dingbats::OpenCentreBlackStar => "open centre black star",
            Dingbats::BlackCentreWhiteStar => "black centre white star",
            Dingbats::OutlinedBlackStar => "outlined black star",
            Dingbats::HeavyOutlinedBlackStar => "heavy outlined black star",
            Dingbats::PinwheelStar => "pinwheel star",
            Dingbats::ShadowedWhiteStar => "shadowed white star",
            Dingbats::HeavyAsterisk => "heavy asterisk",
            Dingbats::OpenCentreAsterisk => "open centre asterisk",
            Dingbats::EightSpokedAsterisk => "eight spoked asterisk",
            Dingbats::EightPointedBlackStar => "eight pointed black star",
            Dingbats::EightPointedPinwheelStar => "eight pointed pinwheel star",
            Dingbats::SixPointedBlackStar => "six pointed black star",
            Dingbats::EightPointedRectilinearBlackStar => "eight pointed rectilinear black star",
            Dingbats::HeavyEightPointedRectilinearBlackStar => "heavy eight pointed rectilinear black star",
            Dingbats::TwelvePointedBlackStar => "twelve pointed black star",
            Dingbats::SixteenPointedAsterisk => "sixteen pointed asterisk",
            Dingbats::TeardropDashSpokedAsterisk => "teardrop-spoked asterisk",
            Dingbats::OpenCentreTeardropDashSpokedAsterisk => "open centre teardrop-spoked asterisk",
            Dingbats::HeavyTeardropDashSpokedAsterisk => "heavy teardrop-spoked asterisk",
            Dingbats::SixPetalledBlackAndWhiteFlorette => "six petalled black and white florette",
            Dingbats::BlackFlorette => "black florette",
            Dingbats::WhiteFlorette => "white florette",
            Dingbats::EightPetalledOutlinedBlackFlorette => "eight petalled outlined black florette",
            Dingbats::CircledOpenCentreEightPointedStar => "circled open centre eight pointed star",
            Dingbats::HeavyTeardropDashSpokedPinwheelAsterisk => "heavy teardrop-spoked pinwheel asterisk",
            Dingbats::Snowflake => "snowflake",
            Dingbats::TightTrifoliateSnowflake => "tight trifoliate snowflake",
            Dingbats::HeavyChevronSnowflake => "heavy chevron snowflake",
            Dingbats::Sparkle => "sparkle",
            Dingbats::HeavySparkle => "heavy sparkle",
            Dingbats::BalloonDashSpokedAsterisk => "balloon-spoked asterisk",
            Dingbats::EightTeardropDashSpokedPropellerAsterisk => "eight teardrop-spoked propeller asterisk",
            Dingbats::HeavyEightTeardropDashSpokedPropellerAsterisk => "heavy eight teardrop-spoked propeller asterisk",
            Dingbats::CrossMark => "cross mark",
            Dingbats::ShadowedWhiteCircle => "shadowed white circle",
            Dingbats::NegativeSquaredCrossMark => "negative squared cross mark",
            Dingbats::LowerRightDropDashShadowedWhiteSquare => "lower right drop-shadowed white square",
            Dingbats::UpperRightDropDashShadowedWhiteSquare => "upper right drop-shadowed white square",
            Dingbats::LowerRightShadowedWhiteSquare => "lower right shadowed white square",
            Dingbats::UpperRightShadowedWhiteSquare => "upper right shadowed white square",
            Dingbats::BlackQuestionMarkOrnament => "black question mark ornament",
            Dingbats::WhiteQuestionMarkOrnament => "white question mark ornament",
            Dingbats::WhiteExclamationMarkOrnament => "white exclamation mark ornament",
            Dingbats::BlackDiamondMinusWhiteX => "black diamond minus white x",
            Dingbats::HeavyExclamationMarkSymbol => "heavy exclamation mark symbol",
            Dingbats::LightVerticalBar => "light vertical bar",
            Dingbats::MediumVerticalBar => "medium vertical bar",
            Dingbats::HeavyVerticalBar => "heavy vertical bar",
            Dingbats::HeavySingleTurnedCommaQuotationMarkOrnament => "heavy single turned comma quotation mark ornament",
            Dingbats::HeavySingleCommaQuotationMarkOrnament => "heavy single comma quotation mark ornament",
            Dingbats::HeavyDoubleTurnedCommaQuotationMarkOrnament => "heavy double turned comma quotation mark ornament",
            Dingbats::HeavyDoubleCommaQuotationMarkOrnament => "heavy double comma quotation mark ornament",
            Dingbats::HeavyLowSingleCommaQuotationMarkOrnament => "heavy low single comma quotation mark ornament",
            Dingbats::HeavyLowDoubleCommaQuotationMarkOrnament => "heavy low double comma quotation mark ornament",
            Dingbats::CurvedStemParagraphSignOrnament => "curved stem paragraph sign ornament",
            Dingbats::HeavyExclamationMarkOrnament => "heavy exclamation mark ornament",
            Dingbats::HeavyHeartExclamationMarkOrnament => "heavy heart exclamation mark ornament",
            Dingbats::HeavyBlackHeart => "heavy black heart",
            Dingbats::RotatedHeavyBlackHeartBullet => "rotated heavy black heart bullet",
            Dingbats::FloralHeart => "floral heart",
            Dingbats::RotatedFloralHeartBullet => "rotated floral heart bullet",
            Dingbats::MediumLeftParenthesisOrnament => "medium left parenthesis ornament",
            Dingbats::MediumRightParenthesisOrnament => "medium right parenthesis ornament",
            Dingbats::MediumFlattenedLeftParenthesisOrnament => "medium flattened left parenthesis ornament",
            Dingbats::MediumFlattenedRightParenthesisOrnament => "medium flattened right parenthesis ornament",
            Dingbats::MediumLeftDashPointingAngleBracketOrnament => "medium left-pointing angle bracket ornament",
            Dingbats::MediumRightDashPointingAngleBracketOrnament => "medium right-pointing angle bracket ornament",
            Dingbats::HeavyLeftDashPointingAngleQuotationMarkOrnament => "heavy left-pointing angle quotation mark ornament",
            Dingbats::HeavyRightDashPointingAngleQuotationMarkOrnament => "heavy right-pointing angle quotation mark ornament",
            Dingbats::HeavyLeftDashPointingAngleBracketOrnament => "heavy left-pointing angle bracket ornament",
            Dingbats::HeavyRightDashPointingAngleBracketOrnament => "heavy right-pointing angle bracket ornament",
            Dingbats::LightLeftTortoiseShellBracketOrnament => "light left tortoise shell bracket ornament",
            Dingbats::LightRightTortoiseShellBracketOrnament => "light right tortoise shell bracket ornament",
            Dingbats::MediumLeftCurlyBracketOrnament => "medium left curly bracket ornament",
            Dingbats::MediumRightCurlyBracketOrnament => "medium right curly bracket ornament",
            Dingbats::DingbatNegativeCircledDigitOne => "dingbat negative circled digit one",
            Dingbats::DingbatNegativeCircledDigitTwo => "dingbat negative circled digit two",
            Dingbats::DingbatNegativeCircledDigitThree => "dingbat negative circled digit three",
            Dingbats::DingbatNegativeCircledDigitFour => "dingbat negative circled digit four",
            Dingbats::DingbatNegativeCircledDigitFive => "dingbat negative circled digit five",
            Dingbats::DingbatNegativeCircledDigitSix => "dingbat negative circled digit six",
            Dingbats::DingbatNegativeCircledDigitSeven => "dingbat negative circled digit seven",
            Dingbats::DingbatNegativeCircledDigitEight => "dingbat negative circled digit eight",
            Dingbats::DingbatNegativeCircledDigitNine => "dingbat negative circled digit nine",
            Dingbats::DingbatNegativeCircledNumberTen => "dingbat negative circled number ten",
            Dingbats::DingbatCircledSansDashSerifDigitOne => "dingbat circled sans-serif digit one",
            Dingbats::DingbatCircledSansDashSerifDigitTwo => "dingbat circled sans-serif digit two",
            Dingbats::DingbatCircledSansDashSerifDigitThree => "dingbat circled sans-serif digit three",
            Dingbats::DingbatCircledSansDashSerifDigitFour => "dingbat circled sans-serif digit four",
            Dingbats::DingbatCircledSansDashSerifDigitFive => "dingbat circled sans-serif digit five",
            Dingbats::DingbatCircledSansDashSerifDigitSix => "dingbat circled sans-serif digit six",
            Dingbats::DingbatCircledSansDashSerifDigitSeven => "dingbat circled sans-serif digit seven",
            Dingbats::DingbatCircledSansDashSerifDigitEight => "dingbat circled sans-serif digit eight",
            Dingbats::DingbatCircledSansDashSerifDigitNine => "dingbat circled sans-serif digit nine",
            Dingbats::DingbatCircledSansDashSerifNumberTen => "dingbat circled sans-serif number ten",
            Dingbats::DingbatNegativeCircledSansDashSerifDigitOne => "dingbat negative circled sans-serif digit one",
            Dingbats::DingbatNegativeCircledSansDashSerifDigitTwo => "dingbat negative circled sans-serif digit two",
            Dingbats::DingbatNegativeCircledSansDashSerifDigitThree => "dingbat negative circled sans-serif digit three",
            Dingbats::DingbatNegativeCircledSansDashSerifDigitFour => "dingbat negative circled sans-serif digit four",
            Dingbats::DingbatNegativeCircledSansDashSerifDigitFive => "dingbat negative circled sans-serif digit five",
            Dingbats::DingbatNegativeCircledSansDashSerifDigitSix => "dingbat negative circled sans-serif digit six",
            Dingbats::DingbatNegativeCircledSansDashSerifDigitSeven => "dingbat negative circled sans-serif digit seven",
            Dingbats::DingbatNegativeCircledSansDashSerifDigitEight => "dingbat negative circled sans-serif digit eight",
            Dingbats::DingbatNegativeCircledSansDashSerifDigitNine => "dingbat negative circled sans-serif digit nine",
            Dingbats::DingbatNegativeCircledSansDashSerifNumberTen => "dingbat negative circled sans-serif number ten",
            Dingbats::HeavyWideDashHeadedRightwardsArrow => "heavy wide-headed rightwards arrow",
            Dingbats::HeavyPlusSign => "heavy plus sign",
            Dingbats::HeavyMinusSign => "heavy minus sign",
            Dingbats::HeavyDivisionSign => "heavy division sign",
            Dingbats::HeavySouthEastArrow => "heavy south east arrow",
            Dingbats::HeavyRightwardsArrow => "heavy rightwards arrow",
            Dingbats::HeavyNorthEastArrow => "heavy north east arrow",
            Dingbats::DraftingPointRightwardsArrow => "drafting point rightwards arrow",
            Dingbats::HeavyRoundDashTippedRightwardsArrow => "heavy round-tipped rightwards arrow",
            Dingbats::TriangleDashHeadedRightwardsArrow => "triangle-headed rightwards arrow",
            Dingbats::HeavyTriangleDashHeadedRightwardsArrow => "heavy triangle-headed rightwards arrow",
            Dingbats::DashedTriangleDashHeadedRightwardsArrow => "dashed triangle-headed rightwards arrow",
            Dingbats::HeavyDashedTriangleDashHeadedRightwardsArrow => "heavy dashed triangle-headed rightwards arrow",
            Dingbats::BlackRightwardsArrow => "black rightwards arrow",
            Dingbats::ThreeDashDTopDashLightedRightwardsArrowhead => "three-d top-lighted rightwards arrowhead",
            Dingbats::ThreeDashDBottomDashLightedRightwardsArrowhead => "three-d bottom-lighted rightwards arrowhead",
            Dingbats::BlackRightwardsArrowhead => "black rightwards arrowhead",
            Dingbats::HeavyBlackCurvedDownwardsAndRightwardsArrow => "heavy black curved downwards and rightwards arrow",
            Dingbats::HeavyBlackCurvedUpwardsAndRightwardsArrow => "heavy black curved upwards and rightwards arrow",
            Dingbats::SquatBlackRightwardsArrow => "squat black rightwards arrow",
            Dingbats::HeavyConcaveDashPointedBlackRightwardsArrow => "heavy concave-pointed black rightwards arrow",
            Dingbats::RightDashShadedWhiteRightwardsArrow => "right-shaded white rightwards arrow",
            Dingbats::LeftDashShadedWhiteRightwardsArrow => "left-shaded white rightwards arrow",
            Dingbats::BackDashTiltedShadowedWhiteRightwardsArrow => "back-tilted shadowed white rightwards arrow",
            Dingbats::FrontDashTiltedShadowedWhiteRightwardsArrow => "front-tilted shadowed white rightwards arrow",
            Dingbats::HeavyLowerRightDashShadowedWhiteRightwardsArrow => "heavy lower right-shadowed white rightwards arrow",
            Dingbats::HeavyUpperRightDashShadowedWhiteRightwardsArrow => "heavy upper right-shadowed white rightwards arrow",
            Dingbats::NotchedLowerRightDashShadowedWhiteRightwardsArrow => "notched lower right-shadowed white rightwards arrow",
            Dingbats::CurlyLoop => "curly loop",
            Dingbats::NotchedUpperRightDashShadowedWhiteRightwardsArrow => "notched upper right-shadowed white rightwards arrow",
            Dingbats::CircledHeavyWhiteRightwardsArrow => "circled heavy white rightwards arrow",
            Dingbats::WhiteDashFeatheredRightwardsArrow => "white-feathered rightwards arrow",
            Dingbats::BlackDashFeatheredSouthEastArrow => "black-feathered south east arrow",
            Dingbats::BlackDashFeatheredRightwardsArrow => "black-feathered rightwards arrow",
            Dingbats::BlackDashFeatheredNorthEastArrow => "black-feathered north east arrow",
            Dingbats::HeavyBlackDashFeatheredSouthEastArrow => "heavy black-feathered south east arrow",
            Dingbats::HeavyBlackDashFeatheredRightwardsArrow => "heavy black-feathered rightwards arrow",
            Dingbats::HeavyBlackDashFeatheredNorthEastArrow => "heavy black-feathered north east arrow",
            Dingbats::TeardropDashBarbedRightwardsArrow => "teardrop-barbed rightwards arrow",
            Dingbats::HeavyTeardropDashShankedRightwardsArrow => "heavy teardrop-shanked rightwards arrow",
            Dingbats::WedgeDashTailedRightwardsArrow => "wedge-tailed rightwards arrow",
            Dingbats::HeavyWedgeDashTailedRightwardsArrow => "heavy wedge-tailed rightwards arrow",
            Dingbats::OpenDashOutlinedRightwardsArrow => "open-outlined rightwards arrow",
        }
    }
}
