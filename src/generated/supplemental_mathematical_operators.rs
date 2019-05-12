/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{2a00}: '⨀'
    pub const N_DASH_ARY_CIRCLED_DOT_OPERATOR: char = '⨀';
    /// \u{2a01}: '⨁'
    pub const N_DASH_ARY_CIRCLED_PLUS_OPERATOR: char = '⨁';
    /// \u{2a02}: '⨂'
    pub const N_DASH_ARY_CIRCLED_TIMES_OPERATOR: char = '⨂';
    /// \u{2a03}: '⨃'
    pub const N_DASH_ARY_UNION_OPERATOR_WITH_DOT: char = '⨃';
    /// \u{2a04}: '⨄'
    pub const N_DASH_ARY_UNION_OPERATOR_WITH_PLUS: char = '⨄';
    /// \u{2a05}: '⨅'
    pub const N_DASH_ARY_SQUARE_INTERSECTION_OPERATOR: char = '⨅';
    /// \u{2a06}: '⨆'
    pub const N_DASH_ARY_SQUARE_UNION_OPERATOR: char = '⨆';
    /// \u{2a07}: '⨇'
    pub const TWO_LOGICAL_AND_OPERATOR: char = '⨇';
    /// \u{2a08}: '⨈'
    pub const TWO_LOGICAL_OR_OPERATOR: char = '⨈';
    /// \u{2a09}: '⨉'
    pub const N_DASH_ARY_TIMES_OPERATOR: char = '⨉';
    /// \u{2a0a}: '⨊'
    pub const MODULO_TWO_SUM: char = '⨊';
    /// \u{2a0b}: '⨋'
    pub const SUMMATION_WITH_INTEGRAL: char = '⨋';
    /// \u{2a0c}: '⨌'
    pub const QUADRUPLE_INTEGRAL_OPERATOR: char = '⨌';
    /// \u{2a0d}: '⨍'
    pub const FINITE_PART_INTEGRAL: char = '⨍';
    /// \u{2a0e}: '⨎'
    pub const INTEGRAL_WITH_DOUBLE_STROKE: char = '⨎';
    /// \u{2a0f}: '⨏'
    pub const INTEGRAL_AVERAGE_WITH_SLASH: char = '⨏';
    /// \u{2a10}: '⨐'
    pub const CIRCULATION_FUNCTION: char = '⨐';
    /// \u{2a11}: '⨑'
    pub const ANTICLOCKWISE_INTEGRATION: char = '⨑';
    /// \u{2a12}: '⨒'
    pub const LINE_INTEGRATION_WITH_RECTANGULAR_PATH_AROUND_POLE: char = '⨒';
    /// \u{2a13}: '⨓'
    pub const LINE_INTEGRATION_WITH_SEMICIRCULAR_PATH_AROUND_POLE: char = '⨓';
    /// \u{2a14}: '⨔'
    pub const LINE_INTEGRATION_NOT_INCLUDING_THE_POLE: char = '⨔';
    /// \u{2a15}: '⨕'
    pub const INTEGRAL_AROUND_A_POINT_OPERATOR: char = '⨕';
    /// \u{2a16}: '⨖'
    pub const QUATERNION_INTEGRAL_OPERATOR: char = '⨖';
    /// \u{2a17}: '⨗'
    pub const INTEGRAL_WITH_LEFTWARDS_ARROW_WITH_HOOK: char = '⨗';
    /// \u{2a18}: '⨘'
    pub const INTEGRAL_WITH_TIMES_SIGN: char = '⨘';
    /// \u{2a19}: '⨙'
    pub const INTEGRAL_WITH_INTERSECTION: char = '⨙';
    /// \u{2a1a}: '⨚'
    pub const INTEGRAL_WITH_UNION: char = '⨚';
    /// \u{2a1b}: '⨛'
    pub const INTEGRAL_WITH_OVERBAR: char = '⨛';
    /// \u{2a1c}: '⨜'
    pub const INTEGRAL_WITH_UNDERBAR: char = '⨜';
    /// \u{2a1d}: '⨝'
    pub const JOIN: char = '⨝';
    /// \u{2a1e}: '⨞'
    pub const LARGE_LEFT_TRIANGLE_OPERATOR: char = '⨞';
    /// \u{2a1f}: '⨟'
    pub const Z_NOTATION_SCHEMA_COMPOSITION: char = '⨟';
    /// \u{2a20}: '⨠'
    pub const Z_NOTATION_SCHEMA_PIPING: char = '⨠';
    /// \u{2a21}: '⨡'
    pub const Z_NOTATION_SCHEMA_PROJECTION: char = '⨡';
    /// \u{2a22}: '⨢'
    pub const PLUS_SIGN_WITH_SMALL_CIRCLE_ABOVE: char = '⨢';
    /// \u{2a23}: '⨣'
    pub const PLUS_SIGN_WITH_CIRCUMFLEX_ACCENT_ABOVE: char = '⨣';
    /// \u{2a24}: '⨤'
    pub const PLUS_SIGN_WITH_TILDE_ABOVE: char = '⨤';
    /// \u{2a25}: '⨥'
    pub const PLUS_SIGN_WITH_DOT_BELOW: char = '⨥';
    /// \u{2a26}: '⨦'
    pub const PLUS_SIGN_WITH_TILDE_BELOW: char = '⨦';
    /// \u{2a27}: '⨧'
    pub const PLUS_SIGN_WITH_SUBSCRIPT_TWO: char = '⨧';
    /// \u{2a28}: '⨨'
    pub const PLUS_SIGN_WITH_BLACK_TRIANGLE: char = '⨨';
    /// \u{2a29}: '⨩'
    pub const MINUS_SIGN_WITH_COMMA_ABOVE: char = '⨩';
    /// \u{2a2a}: '⨪'
    pub const MINUS_SIGN_WITH_DOT_BELOW: char = '⨪';
    /// \u{2a2b}: '⨫'
    pub const MINUS_SIGN_WITH_FALLING_DOTS: char = '⨫';
    /// \u{2a2c}: '⨬'
    pub const MINUS_SIGN_WITH_RISING_DOTS: char = '⨬';
    /// \u{2a2d}: '⨭'
    pub const PLUS_SIGN_IN_LEFT_HALF_CIRCLE: char = '⨭';
    /// \u{2a2e}: '⨮'
    pub const PLUS_SIGN_IN_RIGHT_HALF_CIRCLE: char = '⨮';
    /// \u{2a2f}: '⨯'
    pub const VECTOR_OR_CROSS_PRODUCT: char = '⨯';
    /// \u{2a30}: '⨰'
    pub const MULTIPLICATION_SIGN_WITH_DOT_ABOVE: char = '⨰';
    /// \u{2a31}: '⨱'
    pub const MULTIPLICATION_SIGN_WITH_UNDERBAR: char = '⨱';
    /// \u{2a32}: '⨲'
    pub const SEMIDIRECT_PRODUCT_WITH_BOTTOM_CLOSED: char = '⨲';
    /// \u{2a33}: '⨳'
    pub const SMASH_PRODUCT: char = '⨳';
    /// \u{2a34}: '⨴'
    pub const MULTIPLICATION_SIGN_IN_LEFT_HALF_CIRCLE: char = '⨴';
    /// \u{2a35}: '⨵'
    pub const MULTIPLICATION_SIGN_IN_RIGHT_HALF_CIRCLE: char = '⨵';
    /// \u{2a36}: '⨶'
    pub const CIRCLED_MULTIPLICATION_SIGN_WITH_CIRCUMFLEX_ACCENT: char = '⨶';
    /// \u{2a37}: '⨷'
    pub const MULTIPLICATION_SIGN_IN_DOUBLE_CIRCLE: char = '⨷';
    /// \u{2a38}: '⨸'
    pub const CIRCLED_DIVISION_SIGN: char = '⨸';
    /// \u{2a39}: '⨹'
    pub const PLUS_SIGN_IN_TRIANGLE: char = '⨹';
    /// \u{2a3a}: '⨺'
    pub const MINUS_SIGN_IN_TRIANGLE: char = '⨺';
    /// \u{2a3b}: '⨻'
    pub const MULTIPLICATION_SIGN_IN_TRIANGLE: char = '⨻';
    /// \u{2a3c}: '⨼'
    pub const INTERIOR_PRODUCT: char = '⨼';
    /// \u{2a3d}: '⨽'
    pub const RIGHTHAND_INTERIOR_PRODUCT: char = '⨽';
    /// \u{2a3e}: '⨾'
    pub const Z_NOTATION_RELATIONAL_COMPOSITION: char = '⨾';
    /// \u{2a3f}: '⨿'
    pub const AMALGAMATION_OR_COPRODUCT: char = '⨿';
    /// \u{2a40}: '⩀'
    pub const INTERSECTION_WITH_DOT: char = '⩀';
    /// \u{2a41}: '⩁'
    pub const UNION_WITH_MINUS_SIGN: char = '⩁';
    /// \u{2a42}: '⩂'
    pub const UNION_WITH_OVERBAR: char = '⩂';
    /// \u{2a43}: '⩃'
    pub const INTERSECTION_WITH_OVERBAR: char = '⩃';
    /// \u{2a44}: '⩄'
    pub const INTERSECTION_WITH_LOGICAL_AND: char = '⩄';
    /// \u{2a45}: '⩅'
    pub const UNION_WITH_LOGICAL_OR: char = '⩅';
    /// \u{2a46}: '⩆'
    pub const UNION_ABOVE_INTERSECTION: char = '⩆';
    /// \u{2a47}: '⩇'
    pub const INTERSECTION_ABOVE_UNION: char = '⩇';
    /// \u{2a48}: '⩈'
    pub const UNION_ABOVE_BAR_ABOVE_INTERSECTION: char = '⩈';
    /// \u{2a49}: '⩉'
    pub const INTERSECTION_ABOVE_BAR_ABOVE_UNION: char = '⩉';
    /// \u{2a4a}: '⩊'
    pub const UNION_BESIDE_AND_JOINED_WITH_UNION: char = '⩊';
    /// \u{2a4b}: '⩋'
    pub const INTERSECTION_BESIDE_AND_JOINED_WITH_INTERSECTION: char = '⩋';
    /// \u{2a4c}: '⩌'
    pub const CLOSED_UNION_WITH_SERIFS: char = '⩌';
    /// \u{2a4d}: '⩍'
    pub const CLOSED_INTERSECTION_WITH_SERIFS: char = '⩍';
    /// \u{2a4e}: '⩎'
    pub const DOUBLE_SQUARE_INTERSECTION: char = '⩎';
    /// \u{2a4f}: '⩏'
    pub const DOUBLE_SQUARE_UNION: char = '⩏';
    /// \u{2a50}: '⩐'
    pub const CLOSED_UNION_WITH_SERIFS_AND_SMASH_PRODUCT: char = '⩐';
    /// \u{2a51}: '⩑'
    pub const LOGICAL_AND_WITH_DOT_ABOVE: char = '⩑';
    /// \u{2a52}: '⩒'
    pub const LOGICAL_OR_WITH_DOT_ABOVE: char = '⩒';
    /// \u{2a53}: '⩓'
    pub const DOUBLE_LOGICAL_AND: char = '⩓';
    /// \u{2a54}: '⩔'
    pub const DOUBLE_LOGICAL_OR: char = '⩔';
    /// \u{2a55}: '⩕'
    pub const TWO_INTERSECTING_LOGICAL_AND: char = '⩕';
    /// \u{2a56}: '⩖'
    pub const TWO_INTERSECTING_LOGICAL_OR: char = '⩖';
    /// \u{2a57}: '⩗'
    pub const SLOPING_LARGE_OR: char = '⩗';
    /// \u{2a58}: '⩘'
    pub const SLOPING_LARGE_AND: char = '⩘';
    /// \u{2a59}: '⩙'
    pub const LOGICAL_OR_OVERLAPPING_LOGICAL_AND: char = '⩙';
    /// \u{2a5a}: '⩚'
    pub const LOGICAL_AND_WITH_MIDDLE_STEM: char = '⩚';
    /// \u{2a5b}: '⩛'
    pub const LOGICAL_OR_WITH_MIDDLE_STEM: char = '⩛';
    /// \u{2a5c}: '⩜'
    pub const LOGICAL_AND_WITH_HORIZONTAL_DASH: char = '⩜';
    /// \u{2a5d}: '⩝'
    pub const LOGICAL_OR_WITH_HORIZONTAL_DASH: char = '⩝';
    /// \u{2a5e}: '⩞'
    pub const LOGICAL_AND_WITH_DOUBLE_OVERBAR: char = '⩞';
    /// \u{2a5f}: '⩟'
    pub const LOGICAL_AND_WITH_UNDERBAR: char = '⩟';
    /// \u{2a60}: '⩠'
    pub const LOGICAL_AND_WITH_DOUBLE_UNDERBAR: char = '⩠';
    /// \u{2a61}: '⩡'
    pub const SMALL_VEE_WITH_UNDERBAR: char = '⩡';
    /// \u{2a62}: '⩢'
    pub const LOGICAL_OR_WITH_DOUBLE_OVERBAR: char = '⩢';
    /// \u{2a63}: '⩣'
    pub const LOGICAL_OR_WITH_DOUBLE_UNDERBAR: char = '⩣';
    /// \u{2a64}: '⩤'
    pub const Z_NOTATION_DOMAIN_ANTIRESTRICTION: char = '⩤';
    /// \u{2a65}: '⩥'
    pub const Z_NOTATION_RANGE_ANTIRESTRICTION: char = '⩥';
    /// \u{2a66}: '⩦'
    pub const EQUALS_SIGN_WITH_DOT_BELOW: char = '⩦';
    /// \u{2a67}: '⩧'
    pub const IDENTICAL_WITH_DOT_ABOVE: char = '⩧';
    /// \u{2a68}: '⩨'
    pub const TRIPLE_HORIZONTAL_BAR_WITH_DOUBLE_VERTICAL_STROKE: char = '⩨';
    /// \u{2a69}: '⩩'
    pub const TRIPLE_HORIZONTAL_BAR_WITH_TRIPLE_VERTICAL_STROKE: char = '⩩';
    /// \u{2a6a}: '⩪'
    pub const TILDE_OPERATOR_WITH_DOT_ABOVE: char = '⩪';
    /// \u{2a6b}: '⩫'
    pub const TILDE_OPERATOR_WITH_RISING_DOTS: char = '⩫';
    /// \u{2a6c}: '⩬'
    pub const SIMILAR_MINUS_SIMILAR: char = '⩬';
    /// \u{2a6d}: '⩭'
    pub const CONGRUENT_WITH_DOT_ABOVE: char = '⩭';
    /// \u{2a6e}: '⩮'
    pub const EQUALS_WITH_ASTERISK: char = '⩮';
    /// \u{2a6f}: '⩯'
    pub const ALMOST_EQUAL_TO_WITH_CIRCUMFLEX_ACCENT: char = '⩯';
    /// \u{2a70}: '⩰'
    pub const APPROXIMATELY_EQUAL_OR_EQUAL_TO: char = '⩰';
    /// \u{2a71}: '⩱'
    pub const EQUALS_SIGN_ABOVE_PLUS_SIGN: char = '⩱';
    /// \u{2a72}: '⩲'
    pub const PLUS_SIGN_ABOVE_EQUALS_SIGN: char = '⩲';
    /// \u{2a73}: '⩳'
    pub const EQUALS_SIGN_ABOVE_TILDE_OPERATOR: char = '⩳';
    /// \u{2a74}: '⩴'
    pub const DOUBLE_COLON_EQUAL: char = '⩴';
    /// \u{2a75}: '⩵'
    pub const TWO_CONSECUTIVE_EQUALS_SIGNS: char = '⩵';
    /// \u{2a76}: '⩶'
    pub const THREE_CONSECUTIVE_EQUALS_SIGNS: char = '⩶';
    /// \u{2a77}: '⩷'
    pub const EQUALS_SIGN_WITH_TWO_DOTS_ABOVE_AND_TWO_DOTS_BELOW: char = '⩷';
    /// \u{2a78}: '⩸'
    pub const EQUIVALENT_WITH_FOUR_DOTS_ABOVE: char = '⩸';
    /// \u{2a79}: '⩹'
    pub const LESS_DASH_THAN_WITH_CIRCLE_INSIDE: char = '⩹';
    /// \u{2a7a}: '⩺'
    pub const GREATER_DASH_THAN_WITH_CIRCLE_INSIDE: char = '⩺';
    /// \u{2a7b}: '⩻'
    pub const LESS_DASH_THAN_WITH_QUESTION_MARK_ABOVE: char = '⩻';
    /// \u{2a7c}: '⩼'
    pub const GREATER_DASH_THAN_WITH_QUESTION_MARK_ABOVE: char = '⩼';
    /// \u{2a7d}: '⩽'
    pub const LESS_DASH_THAN_OR_SLANTED_EQUAL_TO: char = '⩽';
    /// \u{2a7e}: '⩾'
    pub const GREATER_DASH_THAN_OR_SLANTED_EQUAL_TO: char = '⩾';
    /// \u{2a7f}: '⩿'
    pub const LESS_DASH_THAN_OR_SLANTED_EQUAL_TO_WITH_DOT_INSIDE: char = '⩿';
    /// \u{2a80}: '⪀'
    pub const GREATER_DASH_THAN_OR_SLANTED_EQUAL_TO_WITH_DOT_INSIDE: char = '⪀';
    /// \u{2a81}: '⪁'
    pub const LESS_DASH_THAN_OR_SLANTED_EQUAL_TO_WITH_DOT_ABOVE: char = '⪁';
    /// \u{2a82}: '⪂'
    pub const GREATER_DASH_THAN_OR_SLANTED_EQUAL_TO_WITH_DOT_ABOVE: char = '⪂';
    /// \u{2a83}: '⪃'
    pub const LESS_DASH_THAN_OR_SLANTED_EQUAL_TO_WITH_DOT_ABOVE_RIGHT: char = '⪃';
    /// \u{2a84}: '⪄'
    pub const GREATER_DASH_THAN_OR_SLANTED_EQUAL_TO_WITH_DOT_ABOVE_LEFT: char = '⪄';
    /// \u{2a85}: '⪅'
    pub const LESS_DASH_THAN_OR_APPROXIMATE: char = '⪅';
    /// \u{2a86}: '⪆'
    pub const GREATER_DASH_THAN_OR_APPROXIMATE: char = '⪆';
    /// \u{2a87}: '⪇'
    pub const LESS_DASH_THAN_AND_SINGLE_DASH_LINE_NOT_EQUAL_TO: char = '⪇';
    /// \u{2a88}: '⪈'
    pub const GREATER_DASH_THAN_AND_SINGLE_DASH_LINE_NOT_EQUAL_TO: char = '⪈';
    /// \u{2a89}: '⪉'
    pub const LESS_DASH_THAN_AND_NOT_APPROXIMATE: char = '⪉';
    /// \u{2a8a}: '⪊'
    pub const GREATER_DASH_THAN_AND_NOT_APPROXIMATE: char = '⪊';
    /// \u{2a8b}: '⪋'
    pub const LESS_DASH_THAN_ABOVE_DOUBLE_DASH_LINE_EQUAL_ABOVE_GREATER_DASH_THAN: char = '⪋';
    /// \u{2a8c}: '⪌'
    pub const GREATER_DASH_THAN_ABOVE_DOUBLE_DASH_LINE_EQUAL_ABOVE_LESS_DASH_THAN: char = '⪌';
    /// \u{2a8d}: '⪍'
    pub const LESS_DASH_THAN_ABOVE_SIMILAR_OR_EQUAL: char = '⪍';
    /// \u{2a8e}: '⪎'
    pub const GREATER_DASH_THAN_ABOVE_SIMILAR_OR_EQUAL: char = '⪎';
    /// \u{2a8f}: '⪏'
    pub const LESS_DASH_THAN_ABOVE_SIMILAR_ABOVE_GREATER_DASH_THAN: char = '⪏';
    /// \u{2a90}: '⪐'
    pub const GREATER_DASH_THAN_ABOVE_SIMILAR_ABOVE_LESS_DASH_THAN: char = '⪐';
    /// \u{2a91}: '⪑'
    pub const LESS_DASH_THAN_ABOVE_GREATER_DASH_THAN_ABOVE_DOUBLE_DASH_LINE_EQUAL: char = '⪑';
    /// \u{2a92}: '⪒'
    pub const GREATER_DASH_THAN_ABOVE_LESS_DASH_THAN_ABOVE_DOUBLE_DASH_LINE_EQUAL: char = '⪒';
    /// \u{2a93}: '⪓'
    pub const LESS_DASH_THAN_ABOVE_SLANTED_EQUAL_ABOVE_GREATER_DASH_THAN_ABOVE_SLANTED_EQUAL: char = '⪓';
    /// \u{2a94}: '⪔'
    pub const GREATER_DASH_THAN_ABOVE_SLANTED_EQUAL_ABOVE_LESS_DASH_THAN_ABOVE_SLANTED_EQUAL: char = '⪔';
    /// \u{2a95}: '⪕'
    pub const SLANTED_EQUAL_TO_OR_LESS_DASH_THAN: char = '⪕';
    /// \u{2a96}: '⪖'
    pub const SLANTED_EQUAL_TO_OR_GREATER_DASH_THAN: char = '⪖';
    /// \u{2a97}: '⪗'
    pub const SLANTED_EQUAL_TO_OR_LESS_DASH_THAN_WITH_DOT_INSIDE: char = '⪗';
    /// \u{2a98}: '⪘'
    pub const SLANTED_EQUAL_TO_OR_GREATER_DASH_THAN_WITH_DOT_INSIDE: char = '⪘';
    /// \u{2a99}: '⪙'
    pub const DOUBLE_DASH_LINE_EQUAL_TO_OR_LESS_DASH_THAN: char = '⪙';
    /// \u{2a9a}: '⪚'
    pub const DOUBLE_DASH_LINE_EQUAL_TO_OR_GREATER_DASH_THAN: char = '⪚';
    /// \u{2a9b}: '⪛'
    pub const DOUBLE_DASH_LINE_SLANTED_EQUAL_TO_OR_LESS_DASH_THAN: char = '⪛';
    /// \u{2a9c}: '⪜'
    pub const DOUBLE_DASH_LINE_SLANTED_EQUAL_TO_OR_GREATER_DASH_THAN: char = '⪜';
    /// \u{2a9d}: '⪝'
    pub const SIMILAR_OR_LESS_DASH_THAN: char = '⪝';
    /// \u{2a9e}: '⪞'
    pub const SIMILAR_OR_GREATER_DASH_THAN: char = '⪞';
    /// \u{2a9f}: '⪟'
    pub const SIMILAR_ABOVE_LESS_DASH_THAN_ABOVE_EQUALS_SIGN: char = '⪟';
    /// \u{2aa0}: '⪠'
    pub const SIMILAR_ABOVE_GREATER_DASH_THAN_ABOVE_EQUALS_SIGN: char = '⪠';
    /// \u{2aa1}: '⪡'
    pub const DOUBLE_NESTED_LESS_DASH_THAN: char = '⪡';
    /// \u{2aa2}: '⪢'
    pub const DOUBLE_NESTED_GREATER_DASH_THAN: char = '⪢';
    /// \u{2aa3}: '⪣'
    pub const DOUBLE_NESTED_LESS_DASH_THAN_WITH_UNDERBAR: char = '⪣';
    /// \u{2aa4}: '⪤'
    pub const GREATER_DASH_THAN_OVERLAPPING_LESS_DASH_THAN: char = '⪤';
    /// \u{2aa5}: '⪥'
    pub const GREATER_DASH_THAN_BESIDE_LESS_DASH_THAN: char = '⪥';
    /// \u{2aa6}: '⪦'
    pub const LESS_DASH_THAN_CLOSED_BY_CURVE: char = '⪦';
    /// \u{2aa7}: '⪧'
    pub const GREATER_DASH_THAN_CLOSED_BY_CURVE: char = '⪧';
    /// \u{2aa8}: '⪨'
    pub const LESS_DASH_THAN_CLOSED_BY_CURVE_ABOVE_SLANTED_EQUAL: char = '⪨';
    /// \u{2aa9}: '⪩'
    pub const GREATER_DASH_THAN_CLOSED_BY_CURVE_ABOVE_SLANTED_EQUAL: char = '⪩';
    /// \u{2aaa}: '⪪'
    pub const SMALLER_THAN: char = '⪪';
    /// \u{2aab}: '⪫'
    pub const LARGER_THAN: char = '⪫';
    /// \u{2aac}: '⪬'
    pub const SMALLER_THAN_OR_EQUAL_TO: char = '⪬';
    /// \u{2aad}: '⪭'
    pub const LARGER_THAN_OR_EQUAL_TO: char = '⪭';
    /// \u{2aae}: '⪮'
    pub const EQUALS_SIGN_WITH_BUMPY_ABOVE: char = '⪮';
    /// \u{2aaf}: '⪯'
    pub const PRECEDES_ABOVE_SINGLE_DASH_LINE_EQUALS_SIGN: char = '⪯';
    /// \u{2ab0}: '⪰'
    pub const SUCCEEDS_ABOVE_SINGLE_DASH_LINE_EQUALS_SIGN: char = '⪰';
    /// \u{2ab1}: '⪱'
    pub const PRECEDES_ABOVE_SINGLE_DASH_LINE_NOT_EQUAL_TO: char = '⪱';
    /// \u{2ab2}: '⪲'
    pub const SUCCEEDS_ABOVE_SINGLE_DASH_LINE_NOT_EQUAL_TO: char = '⪲';
    /// \u{2ab3}: '⪳'
    pub const PRECEDES_ABOVE_EQUALS_SIGN: char = '⪳';
    /// \u{2ab4}: '⪴'
    pub const SUCCEEDS_ABOVE_EQUALS_SIGN: char = '⪴';
    /// \u{2ab5}: '⪵'
    pub const PRECEDES_ABOVE_NOT_EQUAL_TO: char = '⪵';
    /// \u{2ab6}: '⪶'
    pub const SUCCEEDS_ABOVE_NOT_EQUAL_TO: char = '⪶';
    /// \u{2ab7}: '⪷'
    pub const PRECEDES_ABOVE_ALMOST_EQUAL_TO: char = '⪷';
    /// \u{2ab8}: '⪸'
    pub const SUCCEEDS_ABOVE_ALMOST_EQUAL_TO: char = '⪸';
    /// \u{2ab9}: '⪹'
    pub const PRECEDES_ABOVE_NOT_ALMOST_EQUAL_TO: char = '⪹';
    /// \u{2aba}: '⪺'
    pub const SUCCEEDS_ABOVE_NOT_ALMOST_EQUAL_TO: char = '⪺';
    /// \u{2abb}: '⪻'
    pub const DOUBLE_PRECEDES: char = '⪻';
    /// \u{2abc}: '⪼'
    pub const DOUBLE_SUCCEEDS: char = '⪼';
    /// \u{2abd}: '⪽'
    pub const SUBSET_WITH_DOT: char = '⪽';
    /// \u{2abe}: '⪾'
    pub const SUPERSET_WITH_DOT: char = '⪾';
    /// \u{2abf}: '⪿'
    pub const SUBSET_WITH_PLUS_SIGN_BELOW: char = '⪿';
    /// \u{2ac0}: '⫀'
    pub const SUPERSET_WITH_PLUS_SIGN_BELOW: char = '⫀';
    /// \u{2ac1}: '⫁'
    pub const SUBSET_WITH_MULTIPLICATION_SIGN_BELOW: char = '⫁';
    /// \u{2ac2}: '⫂'
    pub const SUPERSET_WITH_MULTIPLICATION_SIGN_BELOW: char = '⫂';
    /// \u{2ac3}: '⫃'
    pub const SUBSET_OF_OR_EQUAL_TO_WITH_DOT_ABOVE: char = '⫃';
    /// \u{2ac4}: '⫄'
    pub const SUPERSET_OF_OR_EQUAL_TO_WITH_DOT_ABOVE: char = '⫄';
    /// \u{2ac5}: '⫅'
    pub const SUBSET_OF_ABOVE_EQUALS_SIGN: char = '⫅';
    /// \u{2ac6}: '⫆'
    pub const SUPERSET_OF_ABOVE_EQUALS_SIGN: char = '⫆';
    /// \u{2ac7}: '⫇'
    pub const SUBSET_OF_ABOVE_TILDE_OPERATOR: char = '⫇';
    /// \u{2ac8}: '⫈'
    pub const SUPERSET_OF_ABOVE_TILDE_OPERATOR: char = '⫈';
    /// \u{2ac9}: '⫉'
    pub const SUBSET_OF_ABOVE_ALMOST_EQUAL_TO: char = '⫉';
    /// \u{2aca}: '⫊'
    pub const SUPERSET_OF_ABOVE_ALMOST_EQUAL_TO: char = '⫊';
    /// \u{2acb}: '⫋'
    pub const SUBSET_OF_ABOVE_NOT_EQUAL_TO: char = '⫋';
    /// \u{2acc}: '⫌'
    pub const SUPERSET_OF_ABOVE_NOT_EQUAL_TO: char = '⫌';
    /// \u{2acd}: '⫍'
    pub const SQUARE_LEFT_OPEN_BOX_OPERATOR: char = '⫍';
    /// \u{2ace}: '⫎'
    pub const SQUARE_RIGHT_OPEN_BOX_OPERATOR: char = '⫎';
    /// \u{2acf}: '⫏'
    pub const CLOSED_SUBSET: char = '⫏';
    /// \u{2ad0}: '⫐'
    pub const CLOSED_SUPERSET: char = '⫐';
    /// \u{2ad1}: '⫑'
    pub const CLOSED_SUBSET_OR_EQUAL_TO: char = '⫑';
    /// \u{2ad2}: '⫒'
    pub const CLOSED_SUPERSET_OR_EQUAL_TO: char = '⫒';
    /// \u{2ad3}: '⫓'
    pub const SUBSET_ABOVE_SUPERSET: char = '⫓';
    /// \u{2ad4}: '⫔'
    pub const SUPERSET_ABOVE_SUBSET: char = '⫔';
    /// \u{2ad5}: '⫕'
    pub const SUBSET_ABOVE_SUBSET: char = '⫕';
    /// \u{2ad6}: '⫖'
    pub const SUPERSET_ABOVE_SUPERSET: char = '⫖';
    /// \u{2ad7}: '⫗'
    pub const SUPERSET_BESIDE_SUBSET: char = '⫗';
    /// \u{2ad8}: '⫘'
    pub const SUPERSET_BESIDE_AND_JOINED_BY_DASH_WITH_SUBSET: char = '⫘';
    /// \u{2ad9}: '⫙'
    pub const ELEMENT_OF_OPENING_DOWNWARDS: char = '⫙';
    /// \u{2ada}: '⫚'
    pub const PITCHFORK_WITH_TEE_TOP: char = '⫚';
    /// \u{2adb}: '⫛'
    pub const TRANSVERSAL_INTERSECTION: char = '⫛';
    /// \u{2adc}: '⫝̸'
    pub const FORKING: char = '⫝̸';
    /// \u{2add}: '⫝'
    pub const NONFORKING: char = '⫝';
    /// \u{2ade}: '⫞'
    pub const SHORT_LEFT_TACK: char = '⫞';
    /// \u{2adf}: '⫟'
    pub const SHORT_DOWN_TACK: char = '⫟';
    /// \u{2ae0}: '⫠'
    pub const SHORT_UP_TACK: char = '⫠';
    /// \u{2ae1}: '⫡'
    pub const PERPENDICULAR_WITH_S: char = '⫡';
    /// \u{2ae2}: '⫢'
    pub const VERTICAL_BAR_TRIPLE_RIGHT_TURNSTILE: char = '⫢';
    /// \u{2ae3}: '⫣'
    pub const DOUBLE_VERTICAL_BAR_LEFT_TURNSTILE: char = '⫣';
    /// \u{2ae4}: '⫤'
    pub const VERTICAL_BAR_DOUBLE_LEFT_TURNSTILE: char = '⫤';
    /// \u{2ae5}: '⫥'
    pub const DOUBLE_VERTICAL_BAR_DOUBLE_LEFT_TURNSTILE: char = '⫥';
    /// \u{2ae6}: '⫦'
    pub const LONG_DASH_FROM_LEFT_MEMBER_OF_DOUBLE_VERTICAL: char = '⫦';
    /// \u{2ae7}: '⫧'
    pub const SHORT_DOWN_TACK_WITH_OVERBAR: char = '⫧';
    /// \u{2ae8}: '⫨'
    pub const SHORT_UP_TACK_WITH_UNDERBAR: char = '⫨';
    /// \u{2ae9}: '⫩'
    pub const SHORT_UP_TACK_ABOVE_SHORT_DOWN_TACK: char = '⫩';
    /// \u{2aea}: '⫪'
    pub const DOUBLE_DOWN_TACK: char = '⫪';
    /// \u{2aeb}: '⫫'
    pub const DOUBLE_UP_TACK: char = '⫫';
    /// \u{2aec}: '⫬'
    pub const DOUBLE_STROKE_NOT_SIGN: char = '⫬';
    /// \u{2aed}: '⫭'
    pub const REVERSED_DOUBLE_STROKE_NOT_SIGN: char = '⫭';
    /// \u{2aee}: '⫮'
    pub const DOES_NOT_DIVIDE_WITH_REVERSED_NEGATION_SLASH: char = '⫮';
    /// \u{2aef}: '⫯'
    pub const VERTICAL_LINE_WITH_CIRCLE_ABOVE: char = '⫯';
    /// \u{2af0}: '⫰'
    pub const VERTICAL_LINE_WITH_CIRCLE_BELOW: char = '⫰';
    /// \u{2af1}: '⫱'
    pub const DOWN_TACK_WITH_CIRCLE_BELOW: char = '⫱';
    /// \u{2af2}: '⫲'
    pub const PARALLEL_WITH_HORIZONTAL_STROKE: char = '⫲';
    /// \u{2af3}: '⫳'
    pub const PARALLEL_WITH_TILDE_OPERATOR: char = '⫳';
    /// \u{2af4}: '⫴'
    pub const TRIPLE_VERTICAL_BAR_BINARY_RELATION: char = '⫴';
    /// \u{2af5}: '⫵'
    pub const TRIPLE_VERTICAL_BAR_WITH_HORIZONTAL_STROKE: char = '⫵';
    /// \u{2af6}: '⫶'
    pub const TRIPLE_COLON_OPERATOR: char = '⫶';
    /// \u{2af7}: '⫷'
    pub const TRIPLE_NESTED_LESS_DASH_THAN: char = '⫷';
    /// \u{2af8}: '⫸'
    pub const TRIPLE_NESTED_GREATER_DASH_THAN: char = '⫸';
    /// \u{2af9}: '⫹'
    pub const DOUBLE_DASH_LINE_SLANTED_LESS_DASH_THAN_OR_EQUAL_TO: char = '⫹';
    /// \u{2afa}: '⫺'
    pub const DOUBLE_DASH_LINE_SLANTED_GREATER_DASH_THAN_OR_EQUAL_TO: char = '⫺';
    /// \u{2afb}: '⫻'
    pub const TRIPLE_SOLIDUS_BINARY_RELATION: char = '⫻';
    /// \u{2afc}: '⫼'
    pub const LARGE_TRIPLE_VERTICAL_BAR_OPERATOR: char = '⫼';
    /// \u{2afd}: '⫽'
    pub const DOUBLE_SOLIDUS_OPERATOR: char = '⫽';
    /// \u{2afe}: '⫾'
    pub const WHITE_VERTICAL_BAR: char = '⫾';
}

/// An enum to represent all characters in the SupplementalMathematicalOperators block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SupplementalMathematicalOperators {
    /// \u{2a00}: '⨀'
    NDashAryCircledDotOperator,
    /// \u{2a01}: '⨁'
    NDashAryCircledPlusOperator,
    /// \u{2a02}: '⨂'
    NDashAryCircledTimesOperator,
    /// \u{2a03}: '⨃'
    NDashAryUnionOperatorWithDot,
    /// \u{2a04}: '⨄'
    NDashAryUnionOperatorWithPlus,
    /// \u{2a05}: '⨅'
    NDashArySquareIntersectionOperator,
    /// \u{2a06}: '⨆'
    NDashArySquareUnionOperator,
    /// \u{2a07}: '⨇'
    TwoLogicalAndOperator,
    /// \u{2a08}: '⨈'
    TwoLogicalOrOperator,
    /// \u{2a09}: '⨉'
    NDashAryTimesOperator,
    /// \u{2a0a}: '⨊'
    ModuloTwoSum,
    /// \u{2a0b}: '⨋'
    SummationWithIntegral,
    /// \u{2a0c}: '⨌'
    QuadrupleIntegralOperator,
    /// \u{2a0d}: '⨍'
    FinitePartIntegral,
    /// \u{2a0e}: '⨎'
    IntegralWithDoubleStroke,
    /// \u{2a0f}: '⨏'
    IntegralAverageWithSlash,
    /// \u{2a10}: '⨐'
    CirculationFunction,
    /// \u{2a11}: '⨑'
    AnticlockwiseIntegration,
    /// \u{2a12}: '⨒'
    LineIntegrationWithRectangularPathAroundPole,
    /// \u{2a13}: '⨓'
    LineIntegrationWithSemicircularPathAroundPole,
    /// \u{2a14}: '⨔'
    LineIntegrationNotIncludingThePole,
    /// \u{2a15}: '⨕'
    IntegralAroundAPointOperator,
    /// \u{2a16}: '⨖'
    QuaternionIntegralOperator,
    /// \u{2a17}: '⨗'
    IntegralWithLeftwardsArrowWithHook,
    /// \u{2a18}: '⨘'
    IntegralWithTimesSign,
    /// \u{2a19}: '⨙'
    IntegralWithIntersection,
    /// \u{2a1a}: '⨚'
    IntegralWithUnion,
    /// \u{2a1b}: '⨛'
    IntegralWithOverbar,
    /// \u{2a1c}: '⨜'
    IntegralWithUnderbar,
    /// \u{2a1d}: '⨝'
    Join,
    /// \u{2a1e}: '⨞'
    LargeLeftTriangleOperator,
    /// \u{2a1f}: '⨟'
    ZNotationSchemaComposition,
    /// \u{2a20}: '⨠'
    ZNotationSchemaPiping,
    /// \u{2a21}: '⨡'
    ZNotationSchemaProjection,
    /// \u{2a22}: '⨢'
    PlusSignWithSmallCircleAbove,
    /// \u{2a23}: '⨣'
    PlusSignWithCircumflexAccentAbove,
    /// \u{2a24}: '⨤'
    PlusSignWithTildeAbove,
    /// \u{2a25}: '⨥'
    PlusSignWithDotBelow,
    /// \u{2a26}: '⨦'
    PlusSignWithTildeBelow,
    /// \u{2a27}: '⨧'
    PlusSignWithSubscriptTwo,
    /// \u{2a28}: '⨨'
    PlusSignWithBlackTriangle,
    /// \u{2a29}: '⨩'
    MinusSignWithCommaAbove,
    /// \u{2a2a}: '⨪'
    MinusSignWithDotBelow,
    /// \u{2a2b}: '⨫'
    MinusSignWithFallingDots,
    /// \u{2a2c}: '⨬'
    MinusSignWithRisingDots,
    /// \u{2a2d}: '⨭'
    PlusSignInLeftHalfCircle,
    /// \u{2a2e}: '⨮'
    PlusSignInRightHalfCircle,
    /// \u{2a2f}: '⨯'
    VectorOrCrossProduct,
    /// \u{2a30}: '⨰'
    MultiplicationSignWithDotAbove,
    /// \u{2a31}: '⨱'
    MultiplicationSignWithUnderbar,
    /// \u{2a32}: '⨲'
    SemidirectProductWithBottomClosed,
    /// \u{2a33}: '⨳'
    SmashProduct,
    /// \u{2a34}: '⨴'
    MultiplicationSignInLeftHalfCircle,
    /// \u{2a35}: '⨵'
    MultiplicationSignInRightHalfCircle,
    /// \u{2a36}: '⨶'
    CircledMultiplicationSignWithCircumflexAccent,
    /// \u{2a37}: '⨷'
    MultiplicationSignInDoubleCircle,
    /// \u{2a38}: '⨸'
    CircledDivisionSign,
    /// \u{2a39}: '⨹'
    PlusSignInTriangle,
    /// \u{2a3a}: '⨺'
    MinusSignInTriangle,
    /// \u{2a3b}: '⨻'
    MultiplicationSignInTriangle,
    /// \u{2a3c}: '⨼'
    InteriorProduct,
    /// \u{2a3d}: '⨽'
    RighthandInteriorProduct,
    /// \u{2a3e}: '⨾'
    ZNotationRelationalComposition,
    /// \u{2a3f}: '⨿'
    AmalgamationOrCoproduct,
    /// \u{2a40}: '⩀'
    IntersectionWithDot,
    /// \u{2a41}: '⩁'
    UnionWithMinusSign,
    /// \u{2a42}: '⩂'
    UnionWithOverbar,
    /// \u{2a43}: '⩃'
    IntersectionWithOverbar,
    /// \u{2a44}: '⩄'
    IntersectionWithLogicalAnd,
    /// \u{2a45}: '⩅'
    UnionWithLogicalOr,
    /// \u{2a46}: '⩆'
    UnionAboveIntersection,
    /// \u{2a47}: '⩇'
    IntersectionAboveUnion,
    /// \u{2a48}: '⩈'
    UnionAboveBarAboveIntersection,
    /// \u{2a49}: '⩉'
    IntersectionAboveBarAboveUnion,
    /// \u{2a4a}: '⩊'
    UnionBesideAndJoinedWithUnion,
    /// \u{2a4b}: '⩋'
    IntersectionBesideAndJoinedWithIntersection,
    /// \u{2a4c}: '⩌'
    ClosedUnionWithSerifs,
    /// \u{2a4d}: '⩍'
    ClosedIntersectionWithSerifs,
    /// \u{2a4e}: '⩎'
    DoubleSquareIntersection,
    /// \u{2a4f}: '⩏'
    DoubleSquareUnion,
    /// \u{2a50}: '⩐'
    ClosedUnionWithSerifsAndSmashProduct,
    /// \u{2a51}: '⩑'
    LogicalAndWithDotAbove,
    /// \u{2a52}: '⩒'
    LogicalOrWithDotAbove,
    /// \u{2a53}: '⩓'
    DoubleLogicalAnd,
    /// \u{2a54}: '⩔'
    DoubleLogicalOr,
    /// \u{2a55}: '⩕'
    TwoIntersectingLogicalAnd,
    /// \u{2a56}: '⩖'
    TwoIntersectingLogicalOr,
    /// \u{2a57}: '⩗'
    SlopingLargeOr,
    /// \u{2a58}: '⩘'
    SlopingLargeAnd,
    /// \u{2a59}: '⩙'
    LogicalOrOverlappingLogicalAnd,
    /// \u{2a5a}: '⩚'
    LogicalAndWithMiddleStem,
    /// \u{2a5b}: '⩛'
    LogicalOrWithMiddleStem,
    /// \u{2a5c}: '⩜'
    LogicalAndWithHorizontalDash,
    /// \u{2a5d}: '⩝'
    LogicalOrWithHorizontalDash,
    /// \u{2a5e}: '⩞'
    LogicalAndWithDoubleOverbar,
    /// \u{2a5f}: '⩟'
    LogicalAndWithUnderbar,
    /// \u{2a60}: '⩠'
    LogicalAndWithDoubleUnderbar,
    /// \u{2a61}: '⩡'
    SmallVeeWithUnderbar,
    /// \u{2a62}: '⩢'
    LogicalOrWithDoubleOverbar,
    /// \u{2a63}: '⩣'
    LogicalOrWithDoubleUnderbar,
    /// \u{2a64}: '⩤'
    ZNotationDomainAntirestriction,
    /// \u{2a65}: '⩥'
    ZNotationRangeAntirestriction,
    /// \u{2a66}: '⩦'
    EqualsSignWithDotBelow,
    /// \u{2a67}: '⩧'
    IdenticalWithDotAbove,
    /// \u{2a68}: '⩨'
    TripleHorizontalBarWithDoubleVerticalStroke,
    /// \u{2a69}: '⩩'
    TripleHorizontalBarWithTripleVerticalStroke,
    /// \u{2a6a}: '⩪'
    TildeOperatorWithDotAbove,
    /// \u{2a6b}: '⩫'
    TildeOperatorWithRisingDots,
    /// \u{2a6c}: '⩬'
    SimilarMinusSimilar,
    /// \u{2a6d}: '⩭'
    CongruentWithDotAbove,
    /// \u{2a6e}: '⩮'
    EqualsWithAsterisk,
    /// \u{2a6f}: '⩯'
    AlmostEqualToWithCircumflexAccent,
    /// \u{2a70}: '⩰'
    ApproximatelyEqualOrEqualTo,
    /// \u{2a71}: '⩱'
    EqualsSignAbovePlusSign,
    /// \u{2a72}: '⩲'
    PlusSignAboveEqualsSign,
    /// \u{2a73}: '⩳'
    EqualsSignAboveTildeOperator,
    /// \u{2a74}: '⩴'
    DoubleColonEqual,
    /// \u{2a75}: '⩵'
    TwoConsecutiveEqualsSigns,
    /// \u{2a76}: '⩶'
    ThreeConsecutiveEqualsSigns,
    /// \u{2a77}: '⩷'
    EqualsSignWithTwoDotsAboveAndTwoDotsBelow,
    /// \u{2a78}: '⩸'
    EquivalentWithFourDotsAbove,
    /// \u{2a79}: '⩹'
    LessDashThanWithCircleInside,
    /// \u{2a7a}: '⩺'
    GreaterDashThanWithCircleInside,
    /// \u{2a7b}: '⩻'
    LessDashThanWithQuestionMarkAbove,
    /// \u{2a7c}: '⩼'
    GreaterDashThanWithQuestionMarkAbove,
    /// \u{2a7d}: '⩽'
    LessDashThanOrSlantedEqualTo,
    /// \u{2a7e}: '⩾'
    GreaterDashThanOrSlantedEqualTo,
    /// \u{2a7f}: '⩿'
    LessDashThanOrSlantedEqualToWithDotInside,
    /// \u{2a80}: '⪀'
    GreaterDashThanOrSlantedEqualToWithDotInside,
    /// \u{2a81}: '⪁'
    LessDashThanOrSlantedEqualToWithDotAbove,
    /// \u{2a82}: '⪂'
    GreaterDashThanOrSlantedEqualToWithDotAbove,
    /// \u{2a83}: '⪃'
    LessDashThanOrSlantedEqualToWithDotAboveRight,
    /// \u{2a84}: '⪄'
    GreaterDashThanOrSlantedEqualToWithDotAboveLeft,
    /// \u{2a85}: '⪅'
    LessDashThanOrApproximate,
    /// \u{2a86}: '⪆'
    GreaterDashThanOrApproximate,
    /// \u{2a87}: '⪇'
    LessDashThanAndSingleDashLineNotEqualTo,
    /// \u{2a88}: '⪈'
    GreaterDashThanAndSingleDashLineNotEqualTo,
    /// \u{2a89}: '⪉'
    LessDashThanAndNotApproximate,
    /// \u{2a8a}: '⪊'
    GreaterDashThanAndNotApproximate,
    /// \u{2a8b}: '⪋'
    LessDashThanAboveDoubleDashLineEqualAboveGreaterDashThan,
    /// \u{2a8c}: '⪌'
    GreaterDashThanAboveDoubleDashLineEqualAboveLessDashThan,
    /// \u{2a8d}: '⪍'
    LessDashThanAboveSimilarOrEqual,
    /// \u{2a8e}: '⪎'
    GreaterDashThanAboveSimilarOrEqual,
    /// \u{2a8f}: '⪏'
    LessDashThanAboveSimilarAboveGreaterDashThan,
    /// \u{2a90}: '⪐'
    GreaterDashThanAboveSimilarAboveLessDashThan,
    /// \u{2a91}: '⪑'
    LessDashThanAboveGreaterDashThanAboveDoubleDashLineEqual,
    /// \u{2a92}: '⪒'
    GreaterDashThanAboveLessDashThanAboveDoubleDashLineEqual,
    /// \u{2a93}: '⪓'
    LessDashThanAboveSlantedEqualAboveGreaterDashThanAboveSlantedEqual,
    /// \u{2a94}: '⪔'
    GreaterDashThanAboveSlantedEqualAboveLessDashThanAboveSlantedEqual,
    /// \u{2a95}: '⪕'
    SlantedEqualToOrLessDashThan,
    /// \u{2a96}: '⪖'
    SlantedEqualToOrGreaterDashThan,
    /// \u{2a97}: '⪗'
    SlantedEqualToOrLessDashThanWithDotInside,
    /// \u{2a98}: '⪘'
    SlantedEqualToOrGreaterDashThanWithDotInside,
    /// \u{2a99}: '⪙'
    DoubleDashLineEqualToOrLessDashThan,
    /// \u{2a9a}: '⪚'
    DoubleDashLineEqualToOrGreaterDashThan,
    /// \u{2a9b}: '⪛'
    DoubleDashLineSlantedEqualToOrLessDashThan,
    /// \u{2a9c}: '⪜'
    DoubleDashLineSlantedEqualToOrGreaterDashThan,
    /// \u{2a9d}: '⪝'
    SimilarOrLessDashThan,
    /// \u{2a9e}: '⪞'
    SimilarOrGreaterDashThan,
    /// \u{2a9f}: '⪟'
    SimilarAboveLessDashThanAboveEqualsSign,
    /// \u{2aa0}: '⪠'
    SimilarAboveGreaterDashThanAboveEqualsSign,
    /// \u{2aa1}: '⪡'
    DoubleNestedLessDashThan,
    /// \u{2aa2}: '⪢'
    DoubleNestedGreaterDashThan,
    /// \u{2aa3}: '⪣'
    DoubleNestedLessDashThanWithUnderbar,
    /// \u{2aa4}: '⪤'
    GreaterDashThanOverlappingLessDashThan,
    /// \u{2aa5}: '⪥'
    GreaterDashThanBesideLessDashThan,
    /// \u{2aa6}: '⪦'
    LessDashThanClosedByCurve,
    /// \u{2aa7}: '⪧'
    GreaterDashThanClosedByCurve,
    /// \u{2aa8}: '⪨'
    LessDashThanClosedByCurveAboveSlantedEqual,
    /// \u{2aa9}: '⪩'
    GreaterDashThanClosedByCurveAboveSlantedEqual,
    /// \u{2aaa}: '⪪'
    SmallerThan,
    /// \u{2aab}: '⪫'
    LargerThan,
    /// \u{2aac}: '⪬'
    SmallerThanOrEqualTo,
    /// \u{2aad}: '⪭'
    LargerThanOrEqualTo,
    /// \u{2aae}: '⪮'
    EqualsSignWithBumpyAbove,
    /// \u{2aaf}: '⪯'
    PrecedesAboveSingleDashLineEqualsSign,
    /// \u{2ab0}: '⪰'
    SucceedsAboveSingleDashLineEqualsSign,
    /// \u{2ab1}: '⪱'
    PrecedesAboveSingleDashLineNotEqualTo,
    /// \u{2ab2}: '⪲'
    SucceedsAboveSingleDashLineNotEqualTo,
    /// \u{2ab3}: '⪳'
    PrecedesAboveEqualsSign,
    /// \u{2ab4}: '⪴'
    SucceedsAboveEqualsSign,
    /// \u{2ab5}: '⪵'
    PrecedesAboveNotEqualTo,
    /// \u{2ab6}: '⪶'
    SucceedsAboveNotEqualTo,
    /// \u{2ab7}: '⪷'
    PrecedesAboveAlmostEqualTo,
    /// \u{2ab8}: '⪸'
    SucceedsAboveAlmostEqualTo,
    /// \u{2ab9}: '⪹'
    PrecedesAboveNotAlmostEqualTo,
    /// \u{2aba}: '⪺'
    SucceedsAboveNotAlmostEqualTo,
    /// \u{2abb}: '⪻'
    DoublePrecedes,
    /// \u{2abc}: '⪼'
    DoubleSucceeds,
    /// \u{2abd}: '⪽'
    SubsetWithDot,
    /// \u{2abe}: '⪾'
    SupersetWithDot,
    /// \u{2abf}: '⪿'
    SubsetWithPlusSignBelow,
    /// \u{2ac0}: '⫀'
    SupersetWithPlusSignBelow,
    /// \u{2ac1}: '⫁'
    SubsetWithMultiplicationSignBelow,
    /// \u{2ac2}: '⫂'
    SupersetWithMultiplicationSignBelow,
    /// \u{2ac3}: '⫃'
    SubsetOfOrEqualToWithDotAbove,
    /// \u{2ac4}: '⫄'
    SupersetOfOrEqualToWithDotAbove,
    /// \u{2ac5}: '⫅'
    SubsetOfAboveEqualsSign,
    /// \u{2ac6}: '⫆'
    SupersetOfAboveEqualsSign,
    /// \u{2ac7}: '⫇'
    SubsetOfAboveTildeOperator,
    /// \u{2ac8}: '⫈'
    SupersetOfAboveTildeOperator,
    /// \u{2ac9}: '⫉'
    SubsetOfAboveAlmostEqualTo,
    /// \u{2aca}: '⫊'
    SupersetOfAboveAlmostEqualTo,
    /// \u{2acb}: '⫋'
    SubsetOfAboveNotEqualTo,
    /// \u{2acc}: '⫌'
    SupersetOfAboveNotEqualTo,
    /// \u{2acd}: '⫍'
    SquareLeftOpenBoxOperator,
    /// \u{2ace}: '⫎'
    SquareRightOpenBoxOperator,
    /// \u{2acf}: '⫏'
    ClosedSubset,
    /// \u{2ad0}: '⫐'
    ClosedSuperset,
    /// \u{2ad1}: '⫑'
    ClosedSubsetOrEqualTo,
    /// \u{2ad2}: '⫒'
    ClosedSupersetOrEqualTo,
    /// \u{2ad3}: '⫓'
    SubsetAboveSuperset,
    /// \u{2ad4}: '⫔'
    SupersetAboveSubset,
    /// \u{2ad5}: '⫕'
    SubsetAboveSubset,
    /// \u{2ad6}: '⫖'
    SupersetAboveSuperset,
    /// \u{2ad7}: '⫗'
    SupersetBesideSubset,
    /// \u{2ad8}: '⫘'
    SupersetBesideAndJoinedByDashWithSubset,
    /// \u{2ad9}: '⫙'
    ElementOfOpeningDownwards,
    /// \u{2ada}: '⫚'
    PitchforkWithTeeTop,
    /// \u{2adb}: '⫛'
    TransversalIntersection,
    /// \u{2adc}: '⫝̸'
    Forking,
    /// \u{2add}: '⫝'
    Nonforking,
    /// \u{2ade}: '⫞'
    ShortLeftTack,
    /// \u{2adf}: '⫟'
    ShortDownTack,
    /// \u{2ae0}: '⫠'
    ShortUpTack,
    /// \u{2ae1}: '⫡'
    PerpendicularWithS,
    /// \u{2ae2}: '⫢'
    VerticalBarTripleRightTurnstile,
    /// \u{2ae3}: '⫣'
    DoubleVerticalBarLeftTurnstile,
    /// \u{2ae4}: '⫤'
    VerticalBarDoubleLeftTurnstile,
    /// \u{2ae5}: '⫥'
    DoubleVerticalBarDoubleLeftTurnstile,
    /// \u{2ae6}: '⫦'
    LongDashFromLeftMemberOfDoubleVertical,
    /// \u{2ae7}: '⫧'
    ShortDownTackWithOverbar,
    /// \u{2ae8}: '⫨'
    ShortUpTackWithUnderbar,
    /// \u{2ae9}: '⫩'
    ShortUpTackAboveShortDownTack,
    /// \u{2aea}: '⫪'
    DoubleDownTack,
    /// \u{2aeb}: '⫫'
    DoubleUpTack,
    /// \u{2aec}: '⫬'
    DoubleStrokeNotSign,
    /// \u{2aed}: '⫭'
    ReversedDoubleStrokeNotSign,
    /// \u{2aee}: '⫮'
    DoesNotDivideWithReversedNegationSlash,
    /// \u{2aef}: '⫯'
    VerticalLineWithCircleAbove,
    /// \u{2af0}: '⫰'
    VerticalLineWithCircleBelow,
    /// \u{2af1}: '⫱'
    DownTackWithCircleBelow,
    /// \u{2af2}: '⫲'
    ParallelWithHorizontalStroke,
    /// \u{2af3}: '⫳'
    ParallelWithTildeOperator,
    /// \u{2af4}: '⫴'
    TripleVerticalBarBinaryRelation,
    /// \u{2af5}: '⫵'
    TripleVerticalBarWithHorizontalStroke,
    /// \u{2af6}: '⫶'
    TripleColonOperator,
    /// \u{2af7}: '⫷'
    TripleNestedLessDashThan,
    /// \u{2af8}: '⫸'
    TripleNestedGreaterDashThan,
    /// \u{2af9}: '⫹'
    DoubleDashLineSlantedLessDashThanOrEqualTo,
    /// \u{2afa}: '⫺'
    DoubleDashLineSlantedGreaterDashThanOrEqualTo,
    /// \u{2afb}: '⫻'
    TripleSolidusBinaryRelation,
    /// \u{2afc}: '⫼'
    LargeTripleVerticalBarOperator,
    /// \u{2afd}: '⫽'
    DoubleSolidusOperator,
    /// \u{2afe}: '⫾'
    WhiteVerticalBar,
}

impl Into<char> for SupplementalMathematicalOperators {
    fn into(self) -> char {
        use constants::*;
        match self {
            SupplementalMathematicalOperators::NDashAryCircledDotOperator => N_DASH_ARY_CIRCLED_DOT_OPERATOR,
            SupplementalMathematicalOperators::NDashAryCircledPlusOperator => N_DASH_ARY_CIRCLED_PLUS_OPERATOR,
            SupplementalMathematicalOperators::NDashAryCircledTimesOperator => N_DASH_ARY_CIRCLED_TIMES_OPERATOR,
            SupplementalMathematicalOperators::NDashAryUnionOperatorWithDot => N_DASH_ARY_UNION_OPERATOR_WITH_DOT,
            SupplementalMathematicalOperators::NDashAryUnionOperatorWithPlus => N_DASH_ARY_UNION_OPERATOR_WITH_PLUS,
            SupplementalMathematicalOperators::NDashArySquareIntersectionOperator => N_DASH_ARY_SQUARE_INTERSECTION_OPERATOR,
            SupplementalMathematicalOperators::NDashArySquareUnionOperator => N_DASH_ARY_SQUARE_UNION_OPERATOR,
            SupplementalMathematicalOperators::TwoLogicalAndOperator => TWO_LOGICAL_AND_OPERATOR,
            SupplementalMathematicalOperators::TwoLogicalOrOperator => TWO_LOGICAL_OR_OPERATOR,
            SupplementalMathematicalOperators::NDashAryTimesOperator => N_DASH_ARY_TIMES_OPERATOR,
            SupplementalMathematicalOperators::ModuloTwoSum => MODULO_TWO_SUM,
            SupplementalMathematicalOperators::SummationWithIntegral => SUMMATION_WITH_INTEGRAL,
            SupplementalMathematicalOperators::QuadrupleIntegralOperator => QUADRUPLE_INTEGRAL_OPERATOR,
            SupplementalMathematicalOperators::FinitePartIntegral => FINITE_PART_INTEGRAL,
            SupplementalMathematicalOperators::IntegralWithDoubleStroke => INTEGRAL_WITH_DOUBLE_STROKE,
            SupplementalMathematicalOperators::IntegralAverageWithSlash => INTEGRAL_AVERAGE_WITH_SLASH,
            SupplementalMathematicalOperators::CirculationFunction => CIRCULATION_FUNCTION,
            SupplementalMathematicalOperators::AnticlockwiseIntegration => ANTICLOCKWISE_INTEGRATION,
            SupplementalMathematicalOperators::LineIntegrationWithRectangularPathAroundPole => LINE_INTEGRATION_WITH_RECTANGULAR_PATH_AROUND_POLE,
            SupplementalMathematicalOperators::LineIntegrationWithSemicircularPathAroundPole => LINE_INTEGRATION_WITH_SEMICIRCULAR_PATH_AROUND_POLE,
            SupplementalMathematicalOperators::LineIntegrationNotIncludingThePole => LINE_INTEGRATION_NOT_INCLUDING_THE_POLE,
            SupplementalMathematicalOperators::IntegralAroundAPointOperator => INTEGRAL_AROUND_A_POINT_OPERATOR,
            SupplementalMathematicalOperators::QuaternionIntegralOperator => QUATERNION_INTEGRAL_OPERATOR,
            SupplementalMathematicalOperators::IntegralWithLeftwardsArrowWithHook => INTEGRAL_WITH_LEFTWARDS_ARROW_WITH_HOOK,
            SupplementalMathematicalOperators::IntegralWithTimesSign => INTEGRAL_WITH_TIMES_SIGN,
            SupplementalMathematicalOperators::IntegralWithIntersection => INTEGRAL_WITH_INTERSECTION,
            SupplementalMathematicalOperators::IntegralWithUnion => INTEGRAL_WITH_UNION,
            SupplementalMathematicalOperators::IntegralWithOverbar => INTEGRAL_WITH_OVERBAR,
            SupplementalMathematicalOperators::IntegralWithUnderbar => INTEGRAL_WITH_UNDERBAR,
            SupplementalMathematicalOperators::Join => JOIN,
            SupplementalMathematicalOperators::LargeLeftTriangleOperator => LARGE_LEFT_TRIANGLE_OPERATOR,
            SupplementalMathematicalOperators::ZNotationSchemaComposition => Z_NOTATION_SCHEMA_COMPOSITION,
            SupplementalMathematicalOperators::ZNotationSchemaPiping => Z_NOTATION_SCHEMA_PIPING,
            SupplementalMathematicalOperators::ZNotationSchemaProjection => Z_NOTATION_SCHEMA_PROJECTION,
            SupplementalMathematicalOperators::PlusSignWithSmallCircleAbove => PLUS_SIGN_WITH_SMALL_CIRCLE_ABOVE,
            SupplementalMathematicalOperators::PlusSignWithCircumflexAccentAbove => PLUS_SIGN_WITH_CIRCUMFLEX_ACCENT_ABOVE,
            SupplementalMathematicalOperators::PlusSignWithTildeAbove => PLUS_SIGN_WITH_TILDE_ABOVE,
            SupplementalMathematicalOperators::PlusSignWithDotBelow => PLUS_SIGN_WITH_DOT_BELOW,
            SupplementalMathematicalOperators::PlusSignWithTildeBelow => PLUS_SIGN_WITH_TILDE_BELOW,
            SupplementalMathematicalOperators::PlusSignWithSubscriptTwo => PLUS_SIGN_WITH_SUBSCRIPT_TWO,
            SupplementalMathematicalOperators::PlusSignWithBlackTriangle => PLUS_SIGN_WITH_BLACK_TRIANGLE,
            SupplementalMathematicalOperators::MinusSignWithCommaAbove => MINUS_SIGN_WITH_COMMA_ABOVE,
            SupplementalMathematicalOperators::MinusSignWithDotBelow => MINUS_SIGN_WITH_DOT_BELOW,
            SupplementalMathematicalOperators::MinusSignWithFallingDots => MINUS_SIGN_WITH_FALLING_DOTS,
            SupplementalMathematicalOperators::MinusSignWithRisingDots => MINUS_SIGN_WITH_RISING_DOTS,
            SupplementalMathematicalOperators::PlusSignInLeftHalfCircle => PLUS_SIGN_IN_LEFT_HALF_CIRCLE,
            SupplementalMathematicalOperators::PlusSignInRightHalfCircle => PLUS_SIGN_IN_RIGHT_HALF_CIRCLE,
            SupplementalMathematicalOperators::VectorOrCrossProduct => VECTOR_OR_CROSS_PRODUCT,
            SupplementalMathematicalOperators::MultiplicationSignWithDotAbove => MULTIPLICATION_SIGN_WITH_DOT_ABOVE,
            SupplementalMathematicalOperators::MultiplicationSignWithUnderbar => MULTIPLICATION_SIGN_WITH_UNDERBAR,
            SupplementalMathematicalOperators::SemidirectProductWithBottomClosed => SEMIDIRECT_PRODUCT_WITH_BOTTOM_CLOSED,
            SupplementalMathematicalOperators::SmashProduct => SMASH_PRODUCT,
            SupplementalMathematicalOperators::MultiplicationSignInLeftHalfCircle => MULTIPLICATION_SIGN_IN_LEFT_HALF_CIRCLE,
            SupplementalMathematicalOperators::MultiplicationSignInRightHalfCircle => MULTIPLICATION_SIGN_IN_RIGHT_HALF_CIRCLE,
            SupplementalMathematicalOperators::CircledMultiplicationSignWithCircumflexAccent => CIRCLED_MULTIPLICATION_SIGN_WITH_CIRCUMFLEX_ACCENT,
            SupplementalMathematicalOperators::MultiplicationSignInDoubleCircle => MULTIPLICATION_SIGN_IN_DOUBLE_CIRCLE,
            SupplementalMathematicalOperators::CircledDivisionSign => CIRCLED_DIVISION_SIGN,
            SupplementalMathematicalOperators::PlusSignInTriangle => PLUS_SIGN_IN_TRIANGLE,
            SupplementalMathematicalOperators::MinusSignInTriangle => MINUS_SIGN_IN_TRIANGLE,
            SupplementalMathematicalOperators::MultiplicationSignInTriangle => MULTIPLICATION_SIGN_IN_TRIANGLE,
            SupplementalMathematicalOperators::InteriorProduct => INTERIOR_PRODUCT,
            SupplementalMathematicalOperators::RighthandInteriorProduct => RIGHTHAND_INTERIOR_PRODUCT,
            SupplementalMathematicalOperators::ZNotationRelationalComposition => Z_NOTATION_RELATIONAL_COMPOSITION,
            SupplementalMathematicalOperators::AmalgamationOrCoproduct => AMALGAMATION_OR_COPRODUCT,
            SupplementalMathematicalOperators::IntersectionWithDot => INTERSECTION_WITH_DOT,
            SupplementalMathematicalOperators::UnionWithMinusSign => UNION_WITH_MINUS_SIGN,
            SupplementalMathematicalOperators::UnionWithOverbar => UNION_WITH_OVERBAR,
            SupplementalMathematicalOperators::IntersectionWithOverbar => INTERSECTION_WITH_OVERBAR,
            SupplementalMathematicalOperators::IntersectionWithLogicalAnd => INTERSECTION_WITH_LOGICAL_AND,
            SupplementalMathematicalOperators::UnionWithLogicalOr => UNION_WITH_LOGICAL_OR,
            SupplementalMathematicalOperators::UnionAboveIntersection => UNION_ABOVE_INTERSECTION,
            SupplementalMathematicalOperators::IntersectionAboveUnion => INTERSECTION_ABOVE_UNION,
            SupplementalMathematicalOperators::UnionAboveBarAboveIntersection => UNION_ABOVE_BAR_ABOVE_INTERSECTION,
            SupplementalMathematicalOperators::IntersectionAboveBarAboveUnion => INTERSECTION_ABOVE_BAR_ABOVE_UNION,
            SupplementalMathematicalOperators::UnionBesideAndJoinedWithUnion => UNION_BESIDE_AND_JOINED_WITH_UNION,
            SupplementalMathematicalOperators::IntersectionBesideAndJoinedWithIntersection => INTERSECTION_BESIDE_AND_JOINED_WITH_INTERSECTION,
            SupplementalMathematicalOperators::ClosedUnionWithSerifs => CLOSED_UNION_WITH_SERIFS,
            SupplementalMathematicalOperators::ClosedIntersectionWithSerifs => CLOSED_INTERSECTION_WITH_SERIFS,
            SupplementalMathematicalOperators::DoubleSquareIntersection => DOUBLE_SQUARE_INTERSECTION,
            SupplementalMathematicalOperators::DoubleSquareUnion => DOUBLE_SQUARE_UNION,
            SupplementalMathematicalOperators::ClosedUnionWithSerifsAndSmashProduct => CLOSED_UNION_WITH_SERIFS_AND_SMASH_PRODUCT,
            SupplementalMathematicalOperators::LogicalAndWithDotAbove => LOGICAL_AND_WITH_DOT_ABOVE,
            SupplementalMathematicalOperators::LogicalOrWithDotAbove => LOGICAL_OR_WITH_DOT_ABOVE,
            SupplementalMathematicalOperators::DoubleLogicalAnd => DOUBLE_LOGICAL_AND,
            SupplementalMathematicalOperators::DoubleLogicalOr => DOUBLE_LOGICAL_OR,
            SupplementalMathematicalOperators::TwoIntersectingLogicalAnd => TWO_INTERSECTING_LOGICAL_AND,
            SupplementalMathematicalOperators::TwoIntersectingLogicalOr => TWO_INTERSECTING_LOGICAL_OR,
            SupplementalMathematicalOperators::SlopingLargeOr => SLOPING_LARGE_OR,
            SupplementalMathematicalOperators::SlopingLargeAnd => SLOPING_LARGE_AND,
            SupplementalMathematicalOperators::LogicalOrOverlappingLogicalAnd => LOGICAL_OR_OVERLAPPING_LOGICAL_AND,
            SupplementalMathematicalOperators::LogicalAndWithMiddleStem => LOGICAL_AND_WITH_MIDDLE_STEM,
            SupplementalMathematicalOperators::LogicalOrWithMiddleStem => LOGICAL_OR_WITH_MIDDLE_STEM,
            SupplementalMathematicalOperators::LogicalAndWithHorizontalDash => LOGICAL_AND_WITH_HORIZONTAL_DASH,
            SupplementalMathematicalOperators::LogicalOrWithHorizontalDash => LOGICAL_OR_WITH_HORIZONTAL_DASH,
            SupplementalMathematicalOperators::LogicalAndWithDoubleOverbar => LOGICAL_AND_WITH_DOUBLE_OVERBAR,
            SupplementalMathematicalOperators::LogicalAndWithUnderbar => LOGICAL_AND_WITH_UNDERBAR,
            SupplementalMathematicalOperators::LogicalAndWithDoubleUnderbar => LOGICAL_AND_WITH_DOUBLE_UNDERBAR,
            SupplementalMathematicalOperators::SmallVeeWithUnderbar => SMALL_VEE_WITH_UNDERBAR,
            SupplementalMathematicalOperators::LogicalOrWithDoubleOverbar => LOGICAL_OR_WITH_DOUBLE_OVERBAR,
            SupplementalMathematicalOperators::LogicalOrWithDoubleUnderbar => LOGICAL_OR_WITH_DOUBLE_UNDERBAR,
            SupplementalMathematicalOperators::ZNotationDomainAntirestriction => Z_NOTATION_DOMAIN_ANTIRESTRICTION,
            SupplementalMathematicalOperators::ZNotationRangeAntirestriction => Z_NOTATION_RANGE_ANTIRESTRICTION,
            SupplementalMathematicalOperators::EqualsSignWithDotBelow => EQUALS_SIGN_WITH_DOT_BELOW,
            SupplementalMathematicalOperators::IdenticalWithDotAbove => IDENTICAL_WITH_DOT_ABOVE,
            SupplementalMathematicalOperators::TripleHorizontalBarWithDoubleVerticalStroke => TRIPLE_HORIZONTAL_BAR_WITH_DOUBLE_VERTICAL_STROKE,
            SupplementalMathematicalOperators::TripleHorizontalBarWithTripleVerticalStroke => TRIPLE_HORIZONTAL_BAR_WITH_TRIPLE_VERTICAL_STROKE,
            SupplementalMathematicalOperators::TildeOperatorWithDotAbove => TILDE_OPERATOR_WITH_DOT_ABOVE,
            SupplementalMathematicalOperators::TildeOperatorWithRisingDots => TILDE_OPERATOR_WITH_RISING_DOTS,
            SupplementalMathematicalOperators::SimilarMinusSimilar => SIMILAR_MINUS_SIMILAR,
            SupplementalMathematicalOperators::CongruentWithDotAbove => CONGRUENT_WITH_DOT_ABOVE,
            SupplementalMathematicalOperators::EqualsWithAsterisk => EQUALS_WITH_ASTERISK,
            SupplementalMathematicalOperators::AlmostEqualToWithCircumflexAccent => ALMOST_EQUAL_TO_WITH_CIRCUMFLEX_ACCENT,
            SupplementalMathematicalOperators::ApproximatelyEqualOrEqualTo => APPROXIMATELY_EQUAL_OR_EQUAL_TO,
            SupplementalMathematicalOperators::EqualsSignAbovePlusSign => EQUALS_SIGN_ABOVE_PLUS_SIGN,
            SupplementalMathematicalOperators::PlusSignAboveEqualsSign => PLUS_SIGN_ABOVE_EQUALS_SIGN,
            SupplementalMathematicalOperators::EqualsSignAboveTildeOperator => EQUALS_SIGN_ABOVE_TILDE_OPERATOR,
            SupplementalMathematicalOperators::DoubleColonEqual => DOUBLE_COLON_EQUAL,
            SupplementalMathematicalOperators::TwoConsecutiveEqualsSigns => TWO_CONSECUTIVE_EQUALS_SIGNS,
            SupplementalMathematicalOperators::ThreeConsecutiveEqualsSigns => THREE_CONSECUTIVE_EQUALS_SIGNS,
            SupplementalMathematicalOperators::EqualsSignWithTwoDotsAboveAndTwoDotsBelow => EQUALS_SIGN_WITH_TWO_DOTS_ABOVE_AND_TWO_DOTS_BELOW,
            SupplementalMathematicalOperators::EquivalentWithFourDotsAbove => EQUIVALENT_WITH_FOUR_DOTS_ABOVE,
            SupplementalMathematicalOperators::LessDashThanWithCircleInside => LESS_DASH_THAN_WITH_CIRCLE_INSIDE,
            SupplementalMathematicalOperators::GreaterDashThanWithCircleInside => GREATER_DASH_THAN_WITH_CIRCLE_INSIDE,
            SupplementalMathematicalOperators::LessDashThanWithQuestionMarkAbove => LESS_DASH_THAN_WITH_QUESTION_MARK_ABOVE,
            SupplementalMathematicalOperators::GreaterDashThanWithQuestionMarkAbove => GREATER_DASH_THAN_WITH_QUESTION_MARK_ABOVE,
            SupplementalMathematicalOperators::LessDashThanOrSlantedEqualTo => LESS_DASH_THAN_OR_SLANTED_EQUAL_TO,
            SupplementalMathematicalOperators::GreaterDashThanOrSlantedEqualTo => GREATER_DASH_THAN_OR_SLANTED_EQUAL_TO,
            SupplementalMathematicalOperators::LessDashThanOrSlantedEqualToWithDotInside => LESS_DASH_THAN_OR_SLANTED_EQUAL_TO_WITH_DOT_INSIDE,
            SupplementalMathematicalOperators::GreaterDashThanOrSlantedEqualToWithDotInside => GREATER_DASH_THAN_OR_SLANTED_EQUAL_TO_WITH_DOT_INSIDE,
            SupplementalMathematicalOperators::LessDashThanOrSlantedEqualToWithDotAbove => LESS_DASH_THAN_OR_SLANTED_EQUAL_TO_WITH_DOT_ABOVE,
            SupplementalMathematicalOperators::GreaterDashThanOrSlantedEqualToWithDotAbove => GREATER_DASH_THAN_OR_SLANTED_EQUAL_TO_WITH_DOT_ABOVE,
            SupplementalMathematicalOperators::LessDashThanOrSlantedEqualToWithDotAboveRight => LESS_DASH_THAN_OR_SLANTED_EQUAL_TO_WITH_DOT_ABOVE_RIGHT,
            SupplementalMathematicalOperators::GreaterDashThanOrSlantedEqualToWithDotAboveLeft => GREATER_DASH_THAN_OR_SLANTED_EQUAL_TO_WITH_DOT_ABOVE_LEFT,
            SupplementalMathematicalOperators::LessDashThanOrApproximate => LESS_DASH_THAN_OR_APPROXIMATE,
            SupplementalMathematicalOperators::GreaterDashThanOrApproximate => GREATER_DASH_THAN_OR_APPROXIMATE,
            SupplementalMathematicalOperators::LessDashThanAndSingleDashLineNotEqualTo => LESS_DASH_THAN_AND_SINGLE_DASH_LINE_NOT_EQUAL_TO,
            SupplementalMathematicalOperators::GreaterDashThanAndSingleDashLineNotEqualTo => GREATER_DASH_THAN_AND_SINGLE_DASH_LINE_NOT_EQUAL_TO,
            SupplementalMathematicalOperators::LessDashThanAndNotApproximate => LESS_DASH_THAN_AND_NOT_APPROXIMATE,
            SupplementalMathematicalOperators::GreaterDashThanAndNotApproximate => GREATER_DASH_THAN_AND_NOT_APPROXIMATE,
            SupplementalMathematicalOperators::LessDashThanAboveDoubleDashLineEqualAboveGreaterDashThan => LESS_DASH_THAN_ABOVE_DOUBLE_DASH_LINE_EQUAL_ABOVE_GREATER_DASH_THAN,
            SupplementalMathematicalOperators::GreaterDashThanAboveDoubleDashLineEqualAboveLessDashThan => GREATER_DASH_THAN_ABOVE_DOUBLE_DASH_LINE_EQUAL_ABOVE_LESS_DASH_THAN,
            SupplementalMathematicalOperators::LessDashThanAboveSimilarOrEqual => LESS_DASH_THAN_ABOVE_SIMILAR_OR_EQUAL,
            SupplementalMathematicalOperators::GreaterDashThanAboveSimilarOrEqual => GREATER_DASH_THAN_ABOVE_SIMILAR_OR_EQUAL,
            SupplementalMathematicalOperators::LessDashThanAboveSimilarAboveGreaterDashThan => LESS_DASH_THAN_ABOVE_SIMILAR_ABOVE_GREATER_DASH_THAN,
            SupplementalMathematicalOperators::GreaterDashThanAboveSimilarAboveLessDashThan => GREATER_DASH_THAN_ABOVE_SIMILAR_ABOVE_LESS_DASH_THAN,
            SupplementalMathematicalOperators::LessDashThanAboveGreaterDashThanAboveDoubleDashLineEqual => LESS_DASH_THAN_ABOVE_GREATER_DASH_THAN_ABOVE_DOUBLE_DASH_LINE_EQUAL,
            SupplementalMathematicalOperators::GreaterDashThanAboveLessDashThanAboveDoubleDashLineEqual => GREATER_DASH_THAN_ABOVE_LESS_DASH_THAN_ABOVE_DOUBLE_DASH_LINE_EQUAL,
            SupplementalMathematicalOperators::LessDashThanAboveSlantedEqualAboveGreaterDashThanAboveSlantedEqual => LESS_DASH_THAN_ABOVE_SLANTED_EQUAL_ABOVE_GREATER_DASH_THAN_ABOVE_SLANTED_EQUAL,
            SupplementalMathematicalOperators::GreaterDashThanAboveSlantedEqualAboveLessDashThanAboveSlantedEqual => GREATER_DASH_THAN_ABOVE_SLANTED_EQUAL_ABOVE_LESS_DASH_THAN_ABOVE_SLANTED_EQUAL,
            SupplementalMathematicalOperators::SlantedEqualToOrLessDashThan => SLANTED_EQUAL_TO_OR_LESS_DASH_THAN,
            SupplementalMathematicalOperators::SlantedEqualToOrGreaterDashThan => SLANTED_EQUAL_TO_OR_GREATER_DASH_THAN,
            SupplementalMathematicalOperators::SlantedEqualToOrLessDashThanWithDotInside => SLANTED_EQUAL_TO_OR_LESS_DASH_THAN_WITH_DOT_INSIDE,
            SupplementalMathematicalOperators::SlantedEqualToOrGreaterDashThanWithDotInside => SLANTED_EQUAL_TO_OR_GREATER_DASH_THAN_WITH_DOT_INSIDE,
            SupplementalMathematicalOperators::DoubleDashLineEqualToOrLessDashThan => DOUBLE_DASH_LINE_EQUAL_TO_OR_LESS_DASH_THAN,
            SupplementalMathematicalOperators::DoubleDashLineEqualToOrGreaterDashThan => DOUBLE_DASH_LINE_EQUAL_TO_OR_GREATER_DASH_THAN,
            SupplementalMathematicalOperators::DoubleDashLineSlantedEqualToOrLessDashThan => DOUBLE_DASH_LINE_SLANTED_EQUAL_TO_OR_LESS_DASH_THAN,
            SupplementalMathematicalOperators::DoubleDashLineSlantedEqualToOrGreaterDashThan => DOUBLE_DASH_LINE_SLANTED_EQUAL_TO_OR_GREATER_DASH_THAN,
            SupplementalMathematicalOperators::SimilarOrLessDashThan => SIMILAR_OR_LESS_DASH_THAN,
            SupplementalMathematicalOperators::SimilarOrGreaterDashThan => SIMILAR_OR_GREATER_DASH_THAN,
            SupplementalMathematicalOperators::SimilarAboveLessDashThanAboveEqualsSign => SIMILAR_ABOVE_LESS_DASH_THAN_ABOVE_EQUALS_SIGN,
            SupplementalMathematicalOperators::SimilarAboveGreaterDashThanAboveEqualsSign => SIMILAR_ABOVE_GREATER_DASH_THAN_ABOVE_EQUALS_SIGN,
            SupplementalMathematicalOperators::DoubleNestedLessDashThan => DOUBLE_NESTED_LESS_DASH_THAN,
            SupplementalMathematicalOperators::DoubleNestedGreaterDashThan => DOUBLE_NESTED_GREATER_DASH_THAN,
            SupplementalMathematicalOperators::DoubleNestedLessDashThanWithUnderbar => DOUBLE_NESTED_LESS_DASH_THAN_WITH_UNDERBAR,
            SupplementalMathematicalOperators::GreaterDashThanOverlappingLessDashThan => GREATER_DASH_THAN_OVERLAPPING_LESS_DASH_THAN,
            SupplementalMathematicalOperators::GreaterDashThanBesideLessDashThan => GREATER_DASH_THAN_BESIDE_LESS_DASH_THAN,
            SupplementalMathematicalOperators::LessDashThanClosedByCurve => LESS_DASH_THAN_CLOSED_BY_CURVE,
            SupplementalMathematicalOperators::GreaterDashThanClosedByCurve => GREATER_DASH_THAN_CLOSED_BY_CURVE,
            SupplementalMathematicalOperators::LessDashThanClosedByCurveAboveSlantedEqual => LESS_DASH_THAN_CLOSED_BY_CURVE_ABOVE_SLANTED_EQUAL,
            SupplementalMathematicalOperators::GreaterDashThanClosedByCurveAboveSlantedEqual => GREATER_DASH_THAN_CLOSED_BY_CURVE_ABOVE_SLANTED_EQUAL,
            SupplementalMathematicalOperators::SmallerThan => SMALLER_THAN,
            SupplementalMathematicalOperators::LargerThan => LARGER_THAN,
            SupplementalMathematicalOperators::SmallerThanOrEqualTo => SMALLER_THAN_OR_EQUAL_TO,
            SupplementalMathematicalOperators::LargerThanOrEqualTo => LARGER_THAN_OR_EQUAL_TO,
            SupplementalMathematicalOperators::EqualsSignWithBumpyAbove => EQUALS_SIGN_WITH_BUMPY_ABOVE,
            SupplementalMathematicalOperators::PrecedesAboveSingleDashLineEqualsSign => PRECEDES_ABOVE_SINGLE_DASH_LINE_EQUALS_SIGN,
            SupplementalMathematicalOperators::SucceedsAboveSingleDashLineEqualsSign => SUCCEEDS_ABOVE_SINGLE_DASH_LINE_EQUALS_SIGN,
            SupplementalMathematicalOperators::PrecedesAboveSingleDashLineNotEqualTo => PRECEDES_ABOVE_SINGLE_DASH_LINE_NOT_EQUAL_TO,
            SupplementalMathematicalOperators::SucceedsAboveSingleDashLineNotEqualTo => SUCCEEDS_ABOVE_SINGLE_DASH_LINE_NOT_EQUAL_TO,
            SupplementalMathematicalOperators::PrecedesAboveEqualsSign => PRECEDES_ABOVE_EQUALS_SIGN,
            SupplementalMathematicalOperators::SucceedsAboveEqualsSign => SUCCEEDS_ABOVE_EQUALS_SIGN,
            SupplementalMathematicalOperators::PrecedesAboveNotEqualTo => PRECEDES_ABOVE_NOT_EQUAL_TO,
            SupplementalMathematicalOperators::SucceedsAboveNotEqualTo => SUCCEEDS_ABOVE_NOT_EQUAL_TO,
            SupplementalMathematicalOperators::PrecedesAboveAlmostEqualTo => PRECEDES_ABOVE_ALMOST_EQUAL_TO,
            SupplementalMathematicalOperators::SucceedsAboveAlmostEqualTo => SUCCEEDS_ABOVE_ALMOST_EQUAL_TO,
            SupplementalMathematicalOperators::PrecedesAboveNotAlmostEqualTo => PRECEDES_ABOVE_NOT_ALMOST_EQUAL_TO,
            SupplementalMathematicalOperators::SucceedsAboveNotAlmostEqualTo => SUCCEEDS_ABOVE_NOT_ALMOST_EQUAL_TO,
            SupplementalMathematicalOperators::DoublePrecedes => DOUBLE_PRECEDES,
            SupplementalMathematicalOperators::DoubleSucceeds => DOUBLE_SUCCEEDS,
            SupplementalMathematicalOperators::SubsetWithDot => SUBSET_WITH_DOT,
            SupplementalMathematicalOperators::SupersetWithDot => SUPERSET_WITH_DOT,
            SupplementalMathematicalOperators::SubsetWithPlusSignBelow => SUBSET_WITH_PLUS_SIGN_BELOW,
            SupplementalMathematicalOperators::SupersetWithPlusSignBelow => SUPERSET_WITH_PLUS_SIGN_BELOW,
            SupplementalMathematicalOperators::SubsetWithMultiplicationSignBelow => SUBSET_WITH_MULTIPLICATION_SIGN_BELOW,
            SupplementalMathematicalOperators::SupersetWithMultiplicationSignBelow => SUPERSET_WITH_MULTIPLICATION_SIGN_BELOW,
            SupplementalMathematicalOperators::SubsetOfOrEqualToWithDotAbove => SUBSET_OF_OR_EQUAL_TO_WITH_DOT_ABOVE,
            SupplementalMathematicalOperators::SupersetOfOrEqualToWithDotAbove => SUPERSET_OF_OR_EQUAL_TO_WITH_DOT_ABOVE,
            SupplementalMathematicalOperators::SubsetOfAboveEqualsSign => SUBSET_OF_ABOVE_EQUALS_SIGN,
            SupplementalMathematicalOperators::SupersetOfAboveEqualsSign => SUPERSET_OF_ABOVE_EQUALS_SIGN,
            SupplementalMathematicalOperators::SubsetOfAboveTildeOperator => SUBSET_OF_ABOVE_TILDE_OPERATOR,
            SupplementalMathematicalOperators::SupersetOfAboveTildeOperator => SUPERSET_OF_ABOVE_TILDE_OPERATOR,
            SupplementalMathematicalOperators::SubsetOfAboveAlmostEqualTo => SUBSET_OF_ABOVE_ALMOST_EQUAL_TO,
            SupplementalMathematicalOperators::SupersetOfAboveAlmostEqualTo => SUPERSET_OF_ABOVE_ALMOST_EQUAL_TO,
            SupplementalMathematicalOperators::SubsetOfAboveNotEqualTo => SUBSET_OF_ABOVE_NOT_EQUAL_TO,
            SupplementalMathematicalOperators::SupersetOfAboveNotEqualTo => SUPERSET_OF_ABOVE_NOT_EQUAL_TO,
            SupplementalMathematicalOperators::SquareLeftOpenBoxOperator => SQUARE_LEFT_OPEN_BOX_OPERATOR,
            SupplementalMathematicalOperators::SquareRightOpenBoxOperator => SQUARE_RIGHT_OPEN_BOX_OPERATOR,
            SupplementalMathematicalOperators::ClosedSubset => CLOSED_SUBSET,
            SupplementalMathematicalOperators::ClosedSuperset => CLOSED_SUPERSET,
            SupplementalMathematicalOperators::ClosedSubsetOrEqualTo => CLOSED_SUBSET_OR_EQUAL_TO,
            SupplementalMathematicalOperators::ClosedSupersetOrEqualTo => CLOSED_SUPERSET_OR_EQUAL_TO,
            SupplementalMathematicalOperators::SubsetAboveSuperset => SUBSET_ABOVE_SUPERSET,
            SupplementalMathematicalOperators::SupersetAboveSubset => SUPERSET_ABOVE_SUBSET,
            SupplementalMathematicalOperators::SubsetAboveSubset => SUBSET_ABOVE_SUBSET,
            SupplementalMathematicalOperators::SupersetAboveSuperset => SUPERSET_ABOVE_SUPERSET,
            SupplementalMathematicalOperators::SupersetBesideSubset => SUPERSET_BESIDE_SUBSET,
            SupplementalMathematicalOperators::SupersetBesideAndJoinedByDashWithSubset => SUPERSET_BESIDE_AND_JOINED_BY_DASH_WITH_SUBSET,
            SupplementalMathematicalOperators::ElementOfOpeningDownwards => ELEMENT_OF_OPENING_DOWNWARDS,
            SupplementalMathematicalOperators::PitchforkWithTeeTop => PITCHFORK_WITH_TEE_TOP,
            SupplementalMathematicalOperators::TransversalIntersection => TRANSVERSAL_INTERSECTION,
            SupplementalMathematicalOperators::Forking => FORKING,
            SupplementalMathematicalOperators::Nonforking => NONFORKING,
            SupplementalMathematicalOperators::ShortLeftTack => SHORT_LEFT_TACK,
            SupplementalMathematicalOperators::ShortDownTack => SHORT_DOWN_TACK,
            SupplementalMathematicalOperators::ShortUpTack => SHORT_UP_TACK,
            SupplementalMathematicalOperators::PerpendicularWithS => PERPENDICULAR_WITH_S,
            SupplementalMathematicalOperators::VerticalBarTripleRightTurnstile => VERTICAL_BAR_TRIPLE_RIGHT_TURNSTILE,
            SupplementalMathematicalOperators::DoubleVerticalBarLeftTurnstile => DOUBLE_VERTICAL_BAR_LEFT_TURNSTILE,
            SupplementalMathematicalOperators::VerticalBarDoubleLeftTurnstile => VERTICAL_BAR_DOUBLE_LEFT_TURNSTILE,
            SupplementalMathematicalOperators::DoubleVerticalBarDoubleLeftTurnstile => DOUBLE_VERTICAL_BAR_DOUBLE_LEFT_TURNSTILE,
            SupplementalMathematicalOperators::LongDashFromLeftMemberOfDoubleVertical => LONG_DASH_FROM_LEFT_MEMBER_OF_DOUBLE_VERTICAL,
            SupplementalMathematicalOperators::ShortDownTackWithOverbar => SHORT_DOWN_TACK_WITH_OVERBAR,
            SupplementalMathematicalOperators::ShortUpTackWithUnderbar => SHORT_UP_TACK_WITH_UNDERBAR,
            SupplementalMathematicalOperators::ShortUpTackAboveShortDownTack => SHORT_UP_TACK_ABOVE_SHORT_DOWN_TACK,
            SupplementalMathematicalOperators::DoubleDownTack => DOUBLE_DOWN_TACK,
            SupplementalMathematicalOperators::DoubleUpTack => DOUBLE_UP_TACK,
            SupplementalMathematicalOperators::DoubleStrokeNotSign => DOUBLE_STROKE_NOT_SIGN,
            SupplementalMathematicalOperators::ReversedDoubleStrokeNotSign => REVERSED_DOUBLE_STROKE_NOT_SIGN,
            SupplementalMathematicalOperators::DoesNotDivideWithReversedNegationSlash => DOES_NOT_DIVIDE_WITH_REVERSED_NEGATION_SLASH,
            SupplementalMathematicalOperators::VerticalLineWithCircleAbove => VERTICAL_LINE_WITH_CIRCLE_ABOVE,
            SupplementalMathematicalOperators::VerticalLineWithCircleBelow => VERTICAL_LINE_WITH_CIRCLE_BELOW,
            SupplementalMathematicalOperators::DownTackWithCircleBelow => DOWN_TACK_WITH_CIRCLE_BELOW,
            SupplementalMathematicalOperators::ParallelWithHorizontalStroke => PARALLEL_WITH_HORIZONTAL_STROKE,
            SupplementalMathematicalOperators::ParallelWithTildeOperator => PARALLEL_WITH_TILDE_OPERATOR,
            SupplementalMathematicalOperators::TripleVerticalBarBinaryRelation => TRIPLE_VERTICAL_BAR_BINARY_RELATION,
            SupplementalMathematicalOperators::TripleVerticalBarWithHorizontalStroke => TRIPLE_VERTICAL_BAR_WITH_HORIZONTAL_STROKE,
            SupplementalMathematicalOperators::TripleColonOperator => TRIPLE_COLON_OPERATOR,
            SupplementalMathematicalOperators::TripleNestedLessDashThan => TRIPLE_NESTED_LESS_DASH_THAN,
            SupplementalMathematicalOperators::TripleNestedGreaterDashThan => TRIPLE_NESTED_GREATER_DASH_THAN,
            SupplementalMathematicalOperators::DoubleDashLineSlantedLessDashThanOrEqualTo => DOUBLE_DASH_LINE_SLANTED_LESS_DASH_THAN_OR_EQUAL_TO,
            SupplementalMathematicalOperators::DoubleDashLineSlantedGreaterDashThanOrEqualTo => DOUBLE_DASH_LINE_SLANTED_GREATER_DASH_THAN_OR_EQUAL_TO,
            SupplementalMathematicalOperators::TripleSolidusBinaryRelation => TRIPLE_SOLIDUS_BINARY_RELATION,
            SupplementalMathematicalOperators::LargeTripleVerticalBarOperator => LARGE_TRIPLE_VERTICAL_BAR_OPERATOR,
            SupplementalMathematicalOperators::DoubleSolidusOperator => DOUBLE_SOLIDUS_OPERATOR,
            SupplementalMathematicalOperators::WhiteVerticalBar => WHITE_VERTICAL_BAR,
        }
    }
}

impl std::convert::TryFrom<char> for SupplementalMathematicalOperators {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            N_DASH_ARY_CIRCLED_DOT_OPERATOR => Ok(SupplementalMathematicalOperators::NDashAryCircledDotOperator),
            N_DASH_ARY_CIRCLED_PLUS_OPERATOR => Ok(SupplementalMathematicalOperators::NDashAryCircledPlusOperator),
            N_DASH_ARY_CIRCLED_TIMES_OPERATOR => Ok(SupplementalMathematicalOperators::NDashAryCircledTimesOperator),
            N_DASH_ARY_UNION_OPERATOR_WITH_DOT => Ok(SupplementalMathematicalOperators::NDashAryUnionOperatorWithDot),
            N_DASH_ARY_UNION_OPERATOR_WITH_PLUS => Ok(SupplementalMathematicalOperators::NDashAryUnionOperatorWithPlus),
            N_DASH_ARY_SQUARE_INTERSECTION_OPERATOR => Ok(SupplementalMathematicalOperators::NDashArySquareIntersectionOperator),
            N_DASH_ARY_SQUARE_UNION_OPERATOR => Ok(SupplementalMathematicalOperators::NDashArySquareUnionOperator),
            TWO_LOGICAL_AND_OPERATOR => Ok(SupplementalMathematicalOperators::TwoLogicalAndOperator),
            TWO_LOGICAL_OR_OPERATOR => Ok(SupplementalMathematicalOperators::TwoLogicalOrOperator),
            N_DASH_ARY_TIMES_OPERATOR => Ok(SupplementalMathematicalOperators::NDashAryTimesOperator),
            MODULO_TWO_SUM => Ok(SupplementalMathematicalOperators::ModuloTwoSum),
            SUMMATION_WITH_INTEGRAL => Ok(SupplementalMathematicalOperators::SummationWithIntegral),
            QUADRUPLE_INTEGRAL_OPERATOR => Ok(SupplementalMathematicalOperators::QuadrupleIntegralOperator),
            FINITE_PART_INTEGRAL => Ok(SupplementalMathematicalOperators::FinitePartIntegral),
            INTEGRAL_WITH_DOUBLE_STROKE => Ok(SupplementalMathematicalOperators::IntegralWithDoubleStroke),
            INTEGRAL_AVERAGE_WITH_SLASH => Ok(SupplementalMathematicalOperators::IntegralAverageWithSlash),
            CIRCULATION_FUNCTION => Ok(SupplementalMathematicalOperators::CirculationFunction),
            ANTICLOCKWISE_INTEGRATION => Ok(SupplementalMathematicalOperators::AnticlockwiseIntegration),
            LINE_INTEGRATION_WITH_RECTANGULAR_PATH_AROUND_POLE => Ok(SupplementalMathematicalOperators::LineIntegrationWithRectangularPathAroundPole),
            LINE_INTEGRATION_WITH_SEMICIRCULAR_PATH_AROUND_POLE => Ok(SupplementalMathematicalOperators::LineIntegrationWithSemicircularPathAroundPole),
            LINE_INTEGRATION_NOT_INCLUDING_THE_POLE => Ok(SupplementalMathematicalOperators::LineIntegrationNotIncludingThePole),
            INTEGRAL_AROUND_A_POINT_OPERATOR => Ok(SupplementalMathematicalOperators::IntegralAroundAPointOperator),
            QUATERNION_INTEGRAL_OPERATOR => Ok(SupplementalMathematicalOperators::QuaternionIntegralOperator),
            INTEGRAL_WITH_LEFTWARDS_ARROW_WITH_HOOK => Ok(SupplementalMathematicalOperators::IntegralWithLeftwardsArrowWithHook),
            INTEGRAL_WITH_TIMES_SIGN => Ok(SupplementalMathematicalOperators::IntegralWithTimesSign),
            INTEGRAL_WITH_INTERSECTION => Ok(SupplementalMathematicalOperators::IntegralWithIntersection),
            INTEGRAL_WITH_UNION => Ok(SupplementalMathematicalOperators::IntegralWithUnion),
            INTEGRAL_WITH_OVERBAR => Ok(SupplementalMathematicalOperators::IntegralWithOverbar),
            INTEGRAL_WITH_UNDERBAR => Ok(SupplementalMathematicalOperators::IntegralWithUnderbar),
            JOIN => Ok(SupplementalMathematicalOperators::Join),
            LARGE_LEFT_TRIANGLE_OPERATOR => Ok(SupplementalMathematicalOperators::LargeLeftTriangleOperator),
            Z_NOTATION_SCHEMA_COMPOSITION => Ok(SupplementalMathematicalOperators::ZNotationSchemaComposition),
            Z_NOTATION_SCHEMA_PIPING => Ok(SupplementalMathematicalOperators::ZNotationSchemaPiping),
            Z_NOTATION_SCHEMA_PROJECTION => Ok(SupplementalMathematicalOperators::ZNotationSchemaProjection),
            PLUS_SIGN_WITH_SMALL_CIRCLE_ABOVE => Ok(SupplementalMathematicalOperators::PlusSignWithSmallCircleAbove),
            PLUS_SIGN_WITH_CIRCUMFLEX_ACCENT_ABOVE => Ok(SupplementalMathematicalOperators::PlusSignWithCircumflexAccentAbove),
            PLUS_SIGN_WITH_TILDE_ABOVE => Ok(SupplementalMathematicalOperators::PlusSignWithTildeAbove),
            PLUS_SIGN_WITH_DOT_BELOW => Ok(SupplementalMathematicalOperators::PlusSignWithDotBelow),
            PLUS_SIGN_WITH_TILDE_BELOW => Ok(SupplementalMathematicalOperators::PlusSignWithTildeBelow),
            PLUS_SIGN_WITH_SUBSCRIPT_TWO => Ok(SupplementalMathematicalOperators::PlusSignWithSubscriptTwo),
            PLUS_SIGN_WITH_BLACK_TRIANGLE => Ok(SupplementalMathematicalOperators::PlusSignWithBlackTriangle),
            MINUS_SIGN_WITH_COMMA_ABOVE => Ok(SupplementalMathematicalOperators::MinusSignWithCommaAbove),
            MINUS_SIGN_WITH_DOT_BELOW => Ok(SupplementalMathematicalOperators::MinusSignWithDotBelow),
            MINUS_SIGN_WITH_FALLING_DOTS => Ok(SupplementalMathematicalOperators::MinusSignWithFallingDots),
            MINUS_SIGN_WITH_RISING_DOTS => Ok(SupplementalMathematicalOperators::MinusSignWithRisingDots),
            PLUS_SIGN_IN_LEFT_HALF_CIRCLE => Ok(SupplementalMathematicalOperators::PlusSignInLeftHalfCircle),
            PLUS_SIGN_IN_RIGHT_HALF_CIRCLE => Ok(SupplementalMathematicalOperators::PlusSignInRightHalfCircle),
            VECTOR_OR_CROSS_PRODUCT => Ok(SupplementalMathematicalOperators::VectorOrCrossProduct),
            MULTIPLICATION_SIGN_WITH_DOT_ABOVE => Ok(SupplementalMathematicalOperators::MultiplicationSignWithDotAbove),
            MULTIPLICATION_SIGN_WITH_UNDERBAR => Ok(SupplementalMathematicalOperators::MultiplicationSignWithUnderbar),
            SEMIDIRECT_PRODUCT_WITH_BOTTOM_CLOSED => Ok(SupplementalMathematicalOperators::SemidirectProductWithBottomClosed),
            SMASH_PRODUCT => Ok(SupplementalMathematicalOperators::SmashProduct),
            MULTIPLICATION_SIGN_IN_LEFT_HALF_CIRCLE => Ok(SupplementalMathematicalOperators::MultiplicationSignInLeftHalfCircle),
            MULTIPLICATION_SIGN_IN_RIGHT_HALF_CIRCLE => Ok(SupplementalMathematicalOperators::MultiplicationSignInRightHalfCircle),
            CIRCLED_MULTIPLICATION_SIGN_WITH_CIRCUMFLEX_ACCENT => Ok(SupplementalMathematicalOperators::CircledMultiplicationSignWithCircumflexAccent),
            MULTIPLICATION_SIGN_IN_DOUBLE_CIRCLE => Ok(SupplementalMathematicalOperators::MultiplicationSignInDoubleCircle),
            CIRCLED_DIVISION_SIGN => Ok(SupplementalMathematicalOperators::CircledDivisionSign),
            PLUS_SIGN_IN_TRIANGLE => Ok(SupplementalMathematicalOperators::PlusSignInTriangle),
            MINUS_SIGN_IN_TRIANGLE => Ok(SupplementalMathematicalOperators::MinusSignInTriangle),
            MULTIPLICATION_SIGN_IN_TRIANGLE => Ok(SupplementalMathematicalOperators::MultiplicationSignInTriangle),
            INTERIOR_PRODUCT => Ok(SupplementalMathematicalOperators::InteriorProduct),
            RIGHTHAND_INTERIOR_PRODUCT => Ok(SupplementalMathematicalOperators::RighthandInteriorProduct),
            Z_NOTATION_RELATIONAL_COMPOSITION => Ok(SupplementalMathematicalOperators::ZNotationRelationalComposition),
            AMALGAMATION_OR_COPRODUCT => Ok(SupplementalMathematicalOperators::AmalgamationOrCoproduct),
            INTERSECTION_WITH_DOT => Ok(SupplementalMathematicalOperators::IntersectionWithDot),
            UNION_WITH_MINUS_SIGN => Ok(SupplementalMathematicalOperators::UnionWithMinusSign),
            UNION_WITH_OVERBAR => Ok(SupplementalMathematicalOperators::UnionWithOverbar),
            INTERSECTION_WITH_OVERBAR => Ok(SupplementalMathematicalOperators::IntersectionWithOverbar),
            INTERSECTION_WITH_LOGICAL_AND => Ok(SupplementalMathematicalOperators::IntersectionWithLogicalAnd),
            UNION_WITH_LOGICAL_OR => Ok(SupplementalMathematicalOperators::UnionWithLogicalOr),
            UNION_ABOVE_INTERSECTION => Ok(SupplementalMathematicalOperators::UnionAboveIntersection),
            INTERSECTION_ABOVE_UNION => Ok(SupplementalMathematicalOperators::IntersectionAboveUnion),
            UNION_ABOVE_BAR_ABOVE_INTERSECTION => Ok(SupplementalMathematicalOperators::UnionAboveBarAboveIntersection),
            INTERSECTION_ABOVE_BAR_ABOVE_UNION => Ok(SupplementalMathematicalOperators::IntersectionAboveBarAboveUnion),
            UNION_BESIDE_AND_JOINED_WITH_UNION => Ok(SupplementalMathematicalOperators::UnionBesideAndJoinedWithUnion),
            INTERSECTION_BESIDE_AND_JOINED_WITH_INTERSECTION => Ok(SupplementalMathematicalOperators::IntersectionBesideAndJoinedWithIntersection),
            CLOSED_UNION_WITH_SERIFS => Ok(SupplementalMathematicalOperators::ClosedUnionWithSerifs),
            CLOSED_INTERSECTION_WITH_SERIFS => Ok(SupplementalMathematicalOperators::ClosedIntersectionWithSerifs),
            DOUBLE_SQUARE_INTERSECTION => Ok(SupplementalMathematicalOperators::DoubleSquareIntersection),
            DOUBLE_SQUARE_UNION => Ok(SupplementalMathematicalOperators::DoubleSquareUnion),
            CLOSED_UNION_WITH_SERIFS_AND_SMASH_PRODUCT => Ok(SupplementalMathematicalOperators::ClosedUnionWithSerifsAndSmashProduct),
            LOGICAL_AND_WITH_DOT_ABOVE => Ok(SupplementalMathematicalOperators::LogicalAndWithDotAbove),
            LOGICAL_OR_WITH_DOT_ABOVE => Ok(SupplementalMathematicalOperators::LogicalOrWithDotAbove),
            DOUBLE_LOGICAL_AND => Ok(SupplementalMathematicalOperators::DoubleLogicalAnd),
            DOUBLE_LOGICAL_OR => Ok(SupplementalMathematicalOperators::DoubleLogicalOr),
            TWO_INTERSECTING_LOGICAL_AND => Ok(SupplementalMathematicalOperators::TwoIntersectingLogicalAnd),
            TWO_INTERSECTING_LOGICAL_OR => Ok(SupplementalMathematicalOperators::TwoIntersectingLogicalOr),
            SLOPING_LARGE_OR => Ok(SupplementalMathematicalOperators::SlopingLargeOr),
            SLOPING_LARGE_AND => Ok(SupplementalMathematicalOperators::SlopingLargeAnd),
            LOGICAL_OR_OVERLAPPING_LOGICAL_AND => Ok(SupplementalMathematicalOperators::LogicalOrOverlappingLogicalAnd),
            LOGICAL_AND_WITH_MIDDLE_STEM => Ok(SupplementalMathematicalOperators::LogicalAndWithMiddleStem),
            LOGICAL_OR_WITH_MIDDLE_STEM => Ok(SupplementalMathematicalOperators::LogicalOrWithMiddleStem),
            LOGICAL_AND_WITH_HORIZONTAL_DASH => Ok(SupplementalMathematicalOperators::LogicalAndWithHorizontalDash),
            LOGICAL_OR_WITH_HORIZONTAL_DASH => Ok(SupplementalMathematicalOperators::LogicalOrWithHorizontalDash),
            LOGICAL_AND_WITH_DOUBLE_OVERBAR => Ok(SupplementalMathematicalOperators::LogicalAndWithDoubleOverbar),
            LOGICAL_AND_WITH_UNDERBAR => Ok(SupplementalMathematicalOperators::LogicalAndWithUnderbar),
            LOGICAL_AND_WITH_DOUBLE_UNDERBAR => Ok(SupplementalMathematicalOperators::LogicalAndWithDoubleUnderbar),
            SMALL_VEE_WITH_UNDERBAR => Ok(SupplementalMathematicalOperators::SmallVeeWithUnderbar),
            LOGICAL_OR_WITH_DOUBLE_OVERBAR => Ok(SupplementalMathematicalOperators::LogicalOrWithDoubleOverbar),
            LOGICAL_OR_WITH_DOUBLE_UNDERBAR => Ok(SupplementalMathematicalOperators::LogicalOrWithDoubleUnderbar),
            Z_NOTATION_DOMAIN_ANTIRESTRICTION => Ok(SupplementalMathematicalOperators::ZNotationDomainAntirestriction),
            Z_NOTATION_RANGE_ANTIRESTRICTION => Ok(SupplementalMathematicalOperators::ZNotationRangeAntirestriction),
            EQUALS_SIGN_WITH_DOT_BELOW => Ok(SupplementalMathematicalOperators::EqualsSignWithDotBelow),
            IDENTICAL_WITH_DOT_ABOVE => Ok(SupplementalMathematicalOperators::IdenticalWithDotAbove),
            TRIPLE_HORIZONTAL_BAR_WITH_DOUBLE_VERTICAL_STROKE => Ok(SupplementalMathematicalOperators::TripleHorizontalBarWithDoubleVerticalStroke),
            TRIPLE_HORIZONTAL_BAR_WITH_TRIPLE_VERTICAL_STROKE => Ok(SupplementalMathematicalOperators::TripleHorizontalBarWithTripleVerticalStroke),
            TILDE_OPERATOR_WITH_DOT_ABOVE => Ok(SupplementalMathematicalOperators::TildeOperatorWithDotAbove),
            TILDE_OPERATOR_WITH_RISING_DOTS => Ok(SupplementalMathematicalOperators::TildeOperatorWithRisingDots),
            SIMILAR_MINUS_SIMILAR => Ok(SupplementalMathematicalOperators::SimilarMinusSimilar),
            CONGRUENT_WITH_DOT_ABOVE => Ok(SupplementalMathematicalOperators::CongruentWithDotAbove),
            EQUALS_WITH_ASTERISK => Ok(SupplementalMathematicalOperators::EqualsWithAsterisk),
            ALMOST_EQUAL_TO_WITH_CIRCUMFLEX_ACCENT => Ok(SupplementalMathematicalOperators::AlmostEqualToWithCircumflexAccent),
            APPROXIMATELY_EQUAL_OR_EQUAL_TO => Ok(SupplementalMathematicalOperators::ApproximatelyEqualOrEqualTo),
            EQUALS_SIGN_ABOVE_PLUS_SIGN => Ok(SupplementalMathematicalOperators::EqualsSignAbovePlusSign),
            PLUS_SIGN_ABOVE_EQUALS_SIGN => Ok(SupplementalMathematicalOperators::PlusSignAboveEqualsSign),
            EQUALS_SIGN_ABOVE_TILDE_OPERATOR => Ok(SupplementalMathematicalOperators::EqualsSignAboveTildeOperator),
            DOUBLE_COLON_EQUAL => Ok(SupplementalMathematicalOperators::DoubleColonEqual),
            TWO_CONSECUTIVE_EQUALS_SIGNS => Ok(SupplementalMathematicalOperators::TwoConsecutiveEqualsSigns),
            THREE_CONSECUTIVE_EQUALS_SIGNS => Ok(SupplementalMathematicalOperators::ThreeConsecutiveEqualsSigns),
            EQUALS_SIGN_WITH_TWO_DOTS_ABOVE_AND_TWO_DOTS_BELOW => Ok(SupplementalMathematicalOperators::EqualsSignWithTwoDotsAboveAndTwoDotsBelow),
            EQUIVALENT_WITH_FOUR_DOTS_ABOVE => Ok(SupplementalMathematicalOperators::EquivalentWithFourDotsAbove),
            LESS_DASH_THAN_WITH_CIRCLE_INSIDE => Ok(SupplementalMathematicalOperators::LessDashThanWithCircleInside),
            GREATER_DASH_THAN_WITH_CIRCLE_INSIDE => Ok(SupplementalMathematicalOperators::GreaterDashThanWithCircleInside),
            LESS_DASH_THAN_WITH_QUESTION_MARK_ABOVE => Ok(SupplementalMathematicalOperators::LessDashThanWithQuestionMarkAbove),
            GREATER_DASH_THAN_WITH_QUESTION_MARK_ABOVE => Ok(SupplementalMathematicalOperators::GreaterDashThanWithQuestionMarkAbove),
            LESS_DASH_THAN_OR_SLANTED_EQUAL_TO => Ok(SupplementalMathematicalOperators::LessDashThanOrSlantedEqualTo),
            GREATER_DASH_THAN_OR_SLANTED_EQUAL_TO => Ok(SupplementalMathematicalOperators::GreaterDashThanOrSlantedEqualTo),
            LESS_DASH_THAN_OR_SLANTED_EQUAL_TO_WITH_DOT_INSIDE => Ok(SupplementalMathematicalOperators::LessDashThanOrSlantedEqualToWithDotInside),
            GREATER_DASH_THAN_OR_SLANTED_EQUAL_TO_WITH_DOT_INSIDE => Ok(SupplementalMathematicalOperators::GreaterDashThanOrSlantedEqualToWithDotInside),
            LESS_DASH_THAN_OR_SLANTED_EQUAL_TO_WITH_DOT_ABOVE => Ok(SupplementalMathematicalOperators::LessDashThanOrSlantedEqualToWithDotAbove),
            GREATER_DASH_THAN_OR_SLANTED_EQUAL_TO_WITH_DOT_ABOVE => Ok(SupplementalMathematicalOperators::GreaterDashThanOrSlantedEqualToWithDotAbove),
            LESS_DASH_THAN_OR_SLANTED_EQUAL_TO_WITH_DOT_ABOVE_RIGHT => Ok(SupplementalMathematicalOperators::LessDashThanOrSlantedEqualToWithDotAboveRight),
            GREATER_DASH_THAN_OR_SLANTED_EQUAL_TO_WITH_DOT_ABOVE_LEFT => Ok(SupplementalMathematicalOperators::GreaterDashThanOrSlantedEqualToWithDotAboveLeft),
            LESS_DASH_THAN_OR_APPROXIMATE => Ok(SupplementalMathematicalOperators::LessDashThanOrApproximate),
            GREATER_DASH_THAN_OR_APPROXIMATE => Ok(SupplementalMathematicalOperators::GreaterDashThanOrApproximate),
            LESS_DASH_THAN_AND_SINGLE_DASH_LINE_NOT_EQUAL_TO => Ok(SupplementalMathematicalOperators::LessDashThanAndSingleDashLineNotEqualTo),
            GREATER_DASH_THAN_AND_SINGLE_DASH_LINE_NOT_EQUAL_TO => Ok(SupplementalMathematicalOperators::GreaterDashThanAndSingleDashLineNotEqualTo),
            LESS_DASH_THAN_AND_NOT_APPROXIMATE => Ok(SupplementalMathematicalOperators::LessDashThanAndNotApproximate),
            GREATER_DASH_THAN_AND_NOT_APPROXIMATE => Ok(SupplementalMathematicalOperators::GreaterDashThanAndNotApproximate),
            LESS_DASH_THAN_ABOVE_DOUBLE_DASH_LINE_EQUAL_ABOVE_GREATER_DASH_THAN => Ok(SupplementalMathematicalOperators::LessDashThanAboveDoubleDashLineEqualAboveGreaterDashThan),
            GREATER_DASH_THAN_ABOVE_DOUBLE_DASH_LINE_EQUAL_ABOVE_LESS_DASH_THAN => Ok(SupplementalMathematicalOperators::GreaterDashThanAboveDoubleDashLineEqualAboveLessDashThan),
            LESS_DASH_THAN_ABOVE_SIMILAR_OR_EQUAL => Ok(SupplementalMathematicalOperators::LessDashThanAboveSimilarOrEqual),
            GREATER_DASH_THAN_ABOVE_SIMILAR_OR_EQUAL => Ok(SupplementalMathematicalOperators::GreaterDashThanAboveSimilarOrEqual),
            LESS_DASH_THAN_ABOVE_SIMILAR_ABOVE_GREATER_DASH_THAN => Ok(SupplementalMathematicalOperators::LessDashThanAboveSimilarAboveGreaterDashThan),
            GREATER_DASH_THAN_ABOVE_SIMILAR_ABOVE_LESS_DASH_THAN => Ok(SupplementalMathematicalOperators::GreaterDashThanAboveSimilarAboveLessDashThan),
            LESS_DASH_THAN_ABOVE_GREATER_DASH_THAN_ABOVE_DOUBLE_DASH_LINE_EQUAL => Ok(SupplementalMathematicalOperators::LessDashThanAboveGreaterDashThanAboveDoubleDashLineEqual),
            GREATER_DASH_THAN_ABOVE_LESS_DASH_THAN_ABOVE_DOUBLE_DASH_LINE_EQUAL => Ok(SupplementalMathematicalOperators::GreaterDashThanAboveLessDashThanAboveDoubleDashLineEqual),
            LESS_DASH_THAN_ABOVE_SLANTED_EQUAL_ABOVE_GREATER_DASH_THAN_ABOVE_SLANTED_EQUAL => Ok(SupplementalMathematicalOperators::LessDashThanAboveSlantedEqualAboveGreaterDashThanAboveSlantedEqual),
            GREATER_DASH_THAN_ABOVE_SLANTED_EQUAL_ABOVE_LESS_DASH_THAN_ABOVE_SLANTED_EQUAL => Ok(SupplementalMathematicalOperators::GreaterDashThanAboveSlantedEqualAboveLessDashThanAboveSlantedEqual),
            SLANTED_EQUAL_TO_OR_LESS_DASH_THAN => Ok(SupplementalMathematicalOperators::SlantedEqualToOrLessDashThan),
            SLANTED_EQUAL_TO_OR_GREATER_DASH_THAN => Ok(SupplementalMathematicalOperators::SlantedEqualToOrGreaterDashThan),
            SLANTED_EQUAL_TO_OR_LESS_DASH_THAN_WITH_DOT_INSIDE => Ok(SupplementalMathematicalOperators::SlantedEqualToOrLessDashThanWithDotInside),
            SLANTED_EQUAL_TO_OR_GREATER_DASH_THAN_WITH_DOT_INSIDE => Ok(SupplementalMathematicalOperators::SlantedEqualToOrGreaterDashThanWithDotInside),
            DOUBLE_DASH_LINE_EQUAL_TO_OR_LESS_DASH_THAN => Ok(SupplementalMathematicalOperators::DoubleDashLineEqualToOrLessDashThan),
            DOUBLE_DASH_LINE_EQUAL_TO_OR_GREATER_DASH_THAN => Ok(SupplementalMathematicalOperators::DoubleDashLineEqualToOrGreaterDashThan),
            DOUBLE_DASH_LINE_SLANTED_EQUAL_TO_OR_LESS_DASH_THAN => Ok(SupplementalMathematicalOperators::DoubleDashLineSlantedEqualToOrLessDashThan),
            DOUBLE_DASH_LINE_SLANTED_EQUAL_TO_OR_GREATER_DASH_THAN => Ok(SupplementalMathematicalOperators::DoubleDashLineSlantedEqualToOrGreaterDashThan),
            SIMILAR_OR_LESS_DASH_THAN => Ok(SupplementalMathematicalOperators::SimilarOrLessDashThan),
            SIMILAR_OR_GREATER_DASH_THAN => Ok(SupplementalMathematicalOperators::SimilarOrGreaterDashThan),
            SIMILAR_ABOVE_LESS_DASH_THAN_ABOVE_EQUALS_SIGN => Ok(SupplementalMathematicalOperators::SimilarAboveLessDashThanAboveEqualsSign),
            SIMILAR_ABOVE_GREATER_DASH_THAN_ABOVE_EQUALS_SIGN => Ok(SupplementalMathematicalOperators::SimilarAboveGreaterDashThanAboveEqualsSign),
            DOUBLE_NESTED_LESS_DASH_THAN => Ok(SupplementalMathematicalOperators::DoubleNestedLessDashThan),
            DOUBLE_NESTED_GREATER_DASH_THAN => Ok(SupplementalMathematicalOperators::DoubleNestedGreaterDashThan),
            DOUBLE_NESTED_LESS_DASH_THAN_WITH_UNDERBAR => Ok(SupplementalMathematicalOperators::DoubleNestedLessDashThanWithUnderbar),
            GREATER_DASH_THAN_OVERLAPPING_LESS_DASH_THAN => Ok(SupplementalMathematicalOperators::GreaterDashThanOverlappingLessDashThan),
            GREATER_DASH_THAN_BESIDE_LESS_DASH_THAN => Ok(SupplementalMathematicalOperators::GreaterDashThanBesideLessDashThan),
            LESS_DASH_THAN_CLOSED_BY_CURVE => Ok(SupplementalMathematicalOperators::LessDashThanClosedByCurve),
            GREATER_DASH_THAN_CLOSED_BY_CURVE => Ok(SupplementalMathematicalOperators::GreaterDashThanClosedByCurve),
            LESS_DASH_THAN_CLOSED_BY_CURVE_ABOVE_SLANTED_EQUAL => Ok(SupplementalMathematicalOperators::LessDashThanClosedByCurveAboveSlantedEqual),
            GREATER_DASH_THAN_CLOSED_BY_CURVE_ABOVE_SLANTED_EQUAL => Ok(SupplementalMathematicalOperators::GreaterDashThanClosedByCurveAboveSlantedEqual),
            SMALLER_THAN => Ok(SupplementalMathematicalOperators::SmallerThan),
            LARGER_THAN => Ok(SupplementalMathematicalOperators::LargerThan),
            SMALLER_THAN_OR_EQUAL_TO => Ok(SupplementalMathematicalOperators::SmallerThanOrEqualTo),
            LARGER_THAN_OR_EQUAL_TO => Ok(SupplementalMathematicalOperators::LargerThanOrEqualTo),
            EQUALS_SIGN_WITH_BUMPY_ABOVE => Ok(SupplementalMathematicalOperators::EqualsSignWithBumpyAbove),
            PRECEDES_ABOVE_SINGLE_DASH_LINE_EQUALS_SIGN => Ok(SupplementalMathematicalOperators::PrecedesAboveSingleDashLineEqualsSign),
            SUCCEEDS_ABOVE_SINGLE_DASH_LINE_EQUALS_SIGN => Ok(SupplementalMathematicalOperators::SucceedsAboveSingleDashLineEqualsSign),
            PRECEDES_ABOVE_SINGLE_DASH_LINE_NOT_EQUAL_TO => Ok(SupplementalMathematicalOperators::PrecedesAboveSingleDashLineNotEqualTo),
            SUCCEEDS_ABOVE_SINGLE_DASH_LINE_NOT_EQUAL_TO => Ok(SupplementalMathematicalOperators::SucceedsAboveSingleDashLineNotEqualTo),
            PRECEDES_ABOVE_EQUALS_SIGN => Ok(SupplementalMathematicalOperators::PrecedesAboveEqualsSign),
            SUCCEEDS_ABOVE_EQUALS_SIGN => Ok(SupplementalMathematicalOperators::SucceedsAboveEqualsSign),
            PRECEDES_ABOVE_NOT_EQUAL_TO => Ok(SupplementalMathematicalOperators::PrecedesAboveNotEqualTo),
            SUCCEEDS_ABOVE_NOT_EQUAL_TO => Ok(SupplementalMathematicalOperators::SucceedsAboveNotEqualTo),
            PRECEDES_ABOVE_ALMOST_EQUAL_TO => Ok(SupplementalMathematicalOperators::PrecedesAboveAlmostEqualTo),
            SUCCEEDS_ABOVE_ALMOST_EQUAL_TO => Ok(SupplementalMathematicalOperators::SucceedsAboveAlmostEqualTo),
            PRECEDES_ABOVE_NOT_ALMOST_EQUAL_TO => Ok(SupplementalMathematicalOperators::PrecedesAboveNotAlmostEqualTo),
            SUCCEEDS_ABOVE_NOT_ALMOST_EQUAL_TO => Ok(SupplementalMathematicalOperators::SucceedsAboveNotAlmostEqualTo),
            DOUBLE_PRECEDES => Ok(SupplementalMathematicalOperators::DoublePrecedes),
            DOUBLE_SUCCEEDS => Ok(SupplementalMathematicalOperators::DoubleSucceeds),
            SUBSET_WITH_DOT => Ok(SupplementalMathematicalOperators::SubsetWithDot),
            SUPERSET_WITH_DOT => Ok(SupplementalMathematicalOperators::SupersetWithDot),
            SUBSET_WITH_PLUS_SIGN_BELOW => Ok(SupplementalMathematicalOperators::SubsetWithPlusSignBelow),
            SUPERSET_WITH_PLUS_SIGN_BELOW => Ok(SupplementalMathematicalOperators::SupersetWithPlusSignBelow),
            SUBSET_WITH_MULTIPLICATION_SIGN_BELOW => Ok(SupplementalMathematicalOperators::SubsetWithMultiplicationSignBelow),
            SUPERSET_WITH_MULTIPLICATION_SIGN_BELOW => Ok(SupplementalMathematicalOperators::SupersetWithMultiplicationSignBelow),
            SUBSET_OF_OR_EQUAL_TO_WITH_DOT_ABOVE => Ok(SupplementalMathematicalOperators::SubsetOfOrEqualToWithDotAbove),
            SUPERSET_OF_OR_EQUAL_TO_WITH_DOT_ABOVE => Ok(SupplementalMathematicalOperators::SupersetOfOrEqualToWithDotAbove),
            SUBSET_OF_ABOVE_EQUALS_SIGN => Ok(SupplementalMathematicalOperators::SubsetOfAboveEqualsSign),
            SUPERSET_OF_ABOVE_EQUALS_SIGN => Ok(SupplementalMathematicalOperators::SupersetOfAboveEqualsSign),
            SUBSET_OF_ABOVE_TILDE_OPERATOR => Ok(SupplementalMathematicalOperators::SubsetOfAboveTildeOperator),
            SUPERSET_OF_ABOVE_TILDE_OPERATOR => Ok(SupplementalMathematicalOperators::SupersetOfAboveTildeOperator),
            SUBSET_OF_ABOVE_ALMOST_EQUAL_TO => Ok(SupplementalMathematicalOperators::SubsetOfAboveAlmostEqualTo),
            SUPERSET_OF_ABOVE_ALMOST_EQUAL_TO => Ok(SupplementalMathematicalOperators::SupersetOfAboveAlmostEqualTo),
            SUBSET_OF_ABOVE_NOT_EQUAL_TO => Ok(SupplementalMathematicalOperators::SubsetOfAboveNotEqualTo),
            SUPERSET_OF_ABOVE_NOT_EQUAL_TO => Ok(SupplementalMathematicalOperators::SupersetOfAboveNotEqualTo),
            SQUARE_LEFT_OPEN_BOX_OPERATOR => Ok(SupplementalMathematicalOperators::SquareLeftOpenBoxOperator),
            SQUARE_RIGHT_OPEN_BOX_OPERATOR => Ok(SupplementalMathematicalOperators::SquareRightOpenBoxOperator),
            CLOSED_SUBSET => Ok(SupplementalMathematicalOperators::ClosedSubset),
            CLOSED_SUPERSET => Ok(SupplementalMathematicalOperators::ClosedSuperset),
            CLOSED_SUBSET_OR_EQUAL_TO => Ok(SupplementalMathematicalOperators::ClosedSubsetOrEqualTo),
            CLOSED_SUPERSET_OR_EQUAL_TO => Ok(SupplementalMathematicalOperators::ClosedSupersetOrEqualTo),
            SUBSET_ABOVE_SUPERSET => Ok(SupplementalMathematicalOperators::SubsetAboveSuperset),
            SUPERSET_ABOVE_SUBSET => Ok(SupplementalMathematicalOperators::SupersetAboveSubset),
            SUBSET_ABOVE_SUBSET => Ok(SupplementalMathematicalOperators::SubsetAboveSubset),
            SUPERSET_ABOVE_SUPERSET => Ok(SupplementalMathematicalOperators::SupersetAboveSuperset),
            SUPERSET_BESIDE_SUBSET => Ok(SupplementalMathematicalOperators::SupersetBesideSubset),
            SUPERSET_BESIDE_AND_JOINED_BY_DASH_WITH_SUBSET => Ok(SupplementalMathematicalOperators::SupersetBesideAndJoinedByDashWithSubset),
            ELEMENT_OF_OPENING_DOWNWARDS => Ok(SupplementalMathematicalOperators::ElementOfOpeningDownwards),
            PITCHFORK_WITH_TEE_TOP => Ok(SupplementalMathematicalOperators::PitchforkWithTeeTop),
            TRANSVERSAL_INTERSECTION => Ok(SupplementalMathematicalOperators::TransversalIntersection),
            FORKING => Ok(SupplementalMathematicalOperators::Forking),
            NONFORKING => Ok(SupplementalMathematicalOperators::Nonforking),
            SHORT_LEFT_TACK => Ok(SupplementalMathematicalOperators::ShortLeftTack),
            SHORT_DOWN_TACK => Ok(SupplementalMathematicalOperators::ShortDownTack),
            SHORT_UP_TACK => Ok(SupplementalMathematicalOperators::ShortUpTack),
            PERPENDICULAR_WITH_S => Ok(SupplementalMathematicalOperators::PerpendicularWithS),
            VERTICAL_BAR_TRIPLE_RIGHT_TURNSTILE => Ok(SupplementalMathematicalOperators::VerticalBarTripleRightTurnstile),
            DOUBLE_VERTICAL_BAR_LEFT_TURNSTILE => Ok(SupplementalMathematicalOperators::DoubleVerticalBarLeftTurnstile),
            VERTICAL_BAR_DOUBLE_LEFT_TURNSTILE => Ok(SupplementalMathematicalOperators::VerticalBarDoubleLeftTurnstile),
            DOUBLE_VERTICAL_BAR_DOUBLE_LEFT_TURNSTILE => Ok(SupplementalMathematicalOperators::DoubleVerticalBarDoubleLeftTurnstile),
            LONG_DASH_FROM_LEFT_MEMBER_OF_DOUBLE_VERTICAL => Ok(SupplementalMathematicalOperators::LongDashFromLeftMemberOfDoubleVertical),
            SHORT_DOWN_TACK_WITH_OVERBAR => Ok(SupplementalMathematicalOperators::ShortDownTackWithOverbar),
            SHORT_UP_TACK_WITH_UNDERBAR => Ok(SupplementalMathematicalOperators::ShortUpTackWithUnderbar),
            SHORT_UP_TACK_ABOVE_SHORT_DOWN_TACK => Ok(SupplementalMathematicalOperators::ShortUpTackAboveShortDownTack),
            DOUBLE_DOWN_TACK => Ok(SupplementalMathematicalOperators::DoubleDownTack),
            DOUBLE_UP_TACK => Ok(SupplementalMathematicalOperators::DoubleUpTack),
            DOUBLE_STROKE_NOT_SIGN => Ok(SupplementalMathematicalOperators::DoubleStrokeNotSign),
            REVERSED_DOUBLE_STROKE_NOT_SIGN => Ok(SupplementalMathematicalOperators::ReversedDoubleStrokeNotSign),
            DOES_NOT_DIVIDE_WITH_REVERSED_NEGATION_SLASH => Ok(SupplementalMathematicalOperators::DoesNotDivideWithReversedNegationSlash),
            VERTICAL_LINE_WITH_CIRCLE_ABOVE => Ok(SupplementalMathematicalOperators::VerticalLineWithCircleAbove),
            VERTICAL_LINE_WITH_CIRCLE_BELOW => Ok(SupplementalMathematicalOperators::VerticalLineWithCircleBelow),
            DOWN_TACK_WITH_CIRCLE_BELOW => Ok(SupplementalMathematicalOperators::DownTackWithCircleBelow),
            PARALLEL_WITH_HORIZONTAL_STROKE => Ok(SupplementalMathematicalOperators::ParallelWithHorizontalStroke),
            PARALLEL_WITH_TILDE_OPERATOR => Ok(SupplementalMathematicalOperators::ParallelWithTildeOperator),
            TRIPLE_VERTICAL_BAR_BINARY_RELATION => Ok(SupplementalMathematicalOperators::TripleVerticalBarBinaryRelation),
            TRIPLE_VERTICAL_BAR_WITH_HORIZONTAL_STROKE => Ok(SupplementalMathematicalOperators::TripleVerticalBarWithHorizontalStroke),
            TRIPLE_COLON_OPERATOR => Ok(SupplementalMathematicalOperators::TripleColonOperator),
            TRIPLE_NESTED_LESS_DASH_THAN => Ok(SupplementalMathematicalOperators::TripleNestedLessDashThan),
            TRIPLE_NESTED_GREATER_DASH_THAN => Ok(SupplementalMathematicalOperators::TripleNestedGreaterDashThan),
            DOUBLE_DASH_LINE_SLANTED_LESS_DASH_THAN_OR_EQUAL_TO => Ok(SupplementalMathematicalOperators::DoubleDashLineSlantedLessDashThanOrEqualTo),
            DOUBLE_DASH_LINE_SLANTED_GREATER_DASH_THAN_OR_EQUAL_TO => Ok(SupplementalMathematicalOperators::DoubleDashLineSlantedGreaterDashThanOrEqualTo),
            TRIPLE_SOLIDUS_BINARY_RELATION => Ok(SupplementalMathematicalOperators::TripleSolidusBinaryRelation),
            LARGE_TRIPLE_VERTICAL_BAR_OPERATOR => Ok(SupplementalMathematicalOperators::LargeTripleVerticalBarOperator),
            DOUBLE_SOLIDUS_OPERATOR => Ok(SupplementalMathematicalOperators::DoubleSolidusOperator),
            WHITE_VERTICAL_BAR => Ok(SupplementalMathematicalOperators::WhiteVerticalBar),
            _ => Err(()),
        }
    }
}

impl Into<u32> for SupplementalMathematicalOperators {
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

impl std::convert::TryFrom<u32> for SupplementalMathematicalOperators {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for SupplementalMathematicalOperators {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl SupplementalMathematicalOperators {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        SupplementalMathematicalOperators::NDashAryCircledDotOperator
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            SupplementalMathematicalOperators::NDashAryCircledDotOperator => "n-ary circled dot operator",
            SupplementalMathematicalOperators::NDashAryCircledPlusOperator => "n-ary circled plus operator",
            SupplementalMathematicalOperators::NDashAryCircledTimesOperator => "n-ary circled times operator",
            SupplementalMathematicalOperators::NDashAryUnionOperatorWithDot => "n-ary union operator with dot",
            SupplementalMathematicalOperators::NDashAryUnionOperatorWithPlus => "n-ary union operator with plus",
            SupplementalMathematicalOperators::NDashArySquareIntersectionOperator => "n-ary square intersection operator",
            SupplementalMathematicalOperators::NDashArySquareUnionOperator => "n-ary square union operator",
            SupplementalMathematicalOperators::TwoLogicalAndOperator => "two logical and operator",
            SupplementalMathematicalOperators::TwoLogicalOrOperator => "two logical or operator",
            SupplementalMathematicalOperators::NDashAryTimesOperator => "n-ary times operator",
            SupplementalMathematicalOperators::ModuloTwoSum => "modulo two sum",
            SupplementalMathematicalOperators::SummationWithIntegral => "summation with integral",
            SupplementalMathematicalOperators::QuadrupleIntegralOperator => "quadruple integral operator",
            SupplementalMathematicalOperators::FinitePartIntegral => "finite part integral",
            SupplementalMathematicalOperators::IntegralWithDoubleStroke => "integral with double stroke",
            SupplementalMathematicalOperators::IntegralAverageWithSlash => "integral average with slash",
            SupplementalMathematicalOperators::CirculationFunction => "circulation function",
            SupplementalMathematicalOperators::AnticlockwiseIntegration => "anticlockwise integration",
            SupplementalMathematicalOperators::LineIntegrationWithRectangularPathAroundPole => "line integration with rectangular path around pole",
            SupplementalMathematicalOperators::LineIntegrationWithSemicircularPathAroundPole => "line integration with semicircular path around pole",
            SupplementalMathematicalOperators::LineIntegrationNotIncludingThePole => "line integration not including the pole",
            SupplementalMathematicalOperators::IntegralAroundAPointOperator => "integral around a point operator",
            SupplementalMathematicalOperators::QuaternionIntegralOperator => "quaternion integral operator",
            SupplementalMathematicalOperators::IntegralWithLeftwardsArrowWithHook => "integral with leftwards arrow with hook",
            SupplementalMathematicalOperators::IntegralWithTimesSign => "integral with times sign",
            SupplementalMathematicalOperators::IntegralWithIntersection => "integral with intersection",
            SupplementalMathematicalOperators::IntegralWithUnion => "integral with union",
            SupplementalMathematicalOperators::IntegralWithOverbar => "integral with overbar",
            SupplementalMathematicalOperators::IntegralWithUnderbar => "integral with underbar",
            SupplementalMathematicalOperators::Join => "join",
            SupplementalMathematicalOperators::LargeLeftTriangleOperator => "large left triangle operator",
            SupplementalMathematicalOperators::ZNotationSchemaComposition => "z notation schema composition",
            SupplementalMathematicalOperators::ZNotationSchemaPiping => "z notation schema piping",
            SupplementalMathematicalOperators::ZNotationSchemaProjection => "z notation schema projection",
            SupplementalMathematicalOperators::PlusSignWithSmallCircleAbove => "plus sign with small circle above",
            SupplementalMathematicalOperators::PlusSignWithCircumflexAccentAbove => "plus sign with circumflex accent above",
            SupplementalMathematicalOperators::PlusSignWithTildeAbove => "plus sign with tilde above",
            SupplementalMathematicalOperators::PlusSignWithDotBelow => "plus sign with dot below",
            SupplementalMathematicalOperators::PlusSignWithTildeBelow => "plus sign with tilde below",
            SupplementalMathematicalOperators::PlusSignWithSubscriptTwo => "plus sign with subscript two",
            SupplementalMathematicalOperators::PlusSignWithBlackTriangle => "plus sign with black triangle",
            SupplementalMathematicalOperators::MinusSignWithCommaAbove => "minus sign with comma above",
            SupplementalMathematicalOperators::MinusSignWithDotBelow => "minus sign with dot below",
            SupplementalMathematicalOperators::MinusSignWithFallingDots => "minus sign with falling dots",
            SupplementalMathematicalOperators::MinusSignWithRisingDots => "minus sign with rising dots",
            SupplementalMathematicalOperators::PlusSignInLeftHalfCircle => "plus sign in left half circle",
            SupplementalMathematicalOperators::PlusSignInRightHalfCircle => "plus sign in right half circle",
            SupplementalMathematicalOperators::VectorOrCrossProduct => "vector or cross product",
            SupplementalMathematicalOperators::MultiplicationSignWithDotAbove => "multiplication sign with dot above",
            SupplementalMathematicalOperators::MultiplicationSignWithUnderbar => "multiplication sign with underbar",
            SupplementalMathematicalOperators::SemidirectProductWithBottomClosed => "semidirect product with bottom closed",
            SupplementalMathematicalOperators::SmashProduct => "smash product",
            SupplementalMathematicalOperators::MultiplicationSignInLeftHalfCircle => "multiplication sign in left half circle",
            SupplementalMathematicalOperators::MultiplicationSignInRightHalfCircle => "multiplication sign in right half circle",
            SupplementalMathematicalOperators::CircledMultiplicationSignWithCircumflexAccent => "circled multiplication sign with circumflex accent",
            SupplementalMathematicalOperators::MultiplicationSignInDoubleCircle => "multiplication sign in double circle",
            SupplementalMathematicalOperators::CircledDivisionSign => "circled division sign",
            SupplementalMathematicalOperators::PlusSignInTriangle => "plus sign in triangle",
            SupplementalMathematicalOperators::MinusSignInTriangle => "minus sign in triangle",
            SupplementalMathematicalOperators::MultiplicationSignInTriangle => "multiplication sign in triangle",
            SupplementalMathematicalOperators::InteriorProduct => "interior product",
            SupplementalMathematicalOperators::RighthandInteriorProduct => "righthand interior product",
            SupplementalMathematicalOperators::ZNotationRelationalComposition => "z notation relational composition",
            SupplementalMathematicalOperators::AmalgamationOrCoproduct => "amalgamation or coproduct",
            SupplementalMathematicalOperators::IntersectionWithDot => "intersection with dot",
            SupplementalMathematicalOperators::UnionWithMinusSign => "union with minus sign",
            SupplementalMathematicalOperators::UnionWithOverbar => "union with overbar",
            SupplementalMathematicalOperators::IntersectionWithOverbar => "intersection with overbar",
            SupplementalMathematicalOperators::IntersectionWithLogicalAnd => "intersection with logical and",
            SupplementalMathematicalOperators::UnionWithLogicalOr => "union with logical or",
            SupplementalMathematicalOperators::UnionAboveIntersection => "union above intersection",
            SupplementalMathematicalOperators::IntersectionAboveUnion => "intersection above union",
            SupplementalMathematicalOperators::UnionAboveBarAboveIntersection => "union above bar above intersection",
            SupplementalMathematicalOperators::IntersectionAboveBarAboveUnion => "intersection above bar above union",
            SupplementalMathematicalOperators::UnionBesideAndJoinedWithUnion => "union beside and joined with union",
            SupplementalMathematicalOperators::IntersectionBesideAndJoinedWithIntersection => "intersection beside and joined with intersection",
            SupplementalMathematicalOperators::ClosedUnionWithSerifs => "closed union with serifs",
            SupplementalMathematicalOperators::ClosedIntersectionWithSerifs => "closed intersection with serifs",
            SupplementalMathematicalOperators::DoubleSquareIntersection => "double square intersection",
            SupplementalMathematicalOperators::DoubleSquareUnion => "double square union",
            SupplementalMathematicalOperators::ClosedUnionWithSerifsAndSmashProduct => "closed union with serifs and smash product",
            SupplementalMathematicalOperators::LogicalAndWithDotAbove => "logical and with dot above",
            SupplementalMathematicalOperators::LogicalOrWithDotAbove => "logical or with dot above",
            SupplementalMathematicalOperators::DoubleLogicalAnd => "double logical and",
            SupplementalMathematicalOperators::DoubleLogicalOr => "double logical or",
            SupplementalMathematicalOperators::TwoIntersectingLogicalAnd => "two intersecting logical and",
            SupplementalMathematicalOperators::TwoIntersectingLogicalOr => "two intersecting logical or",
            SupplementalMathematicalOperators::SlopingLargeOr => "sloping large or",
            SupplementalMathematicalOperators::SlopingLargeAnd => "sloping large and",
            SupplementalMathematicalOperators::LogicalOrOverlappingLogicalAnd => "logical or overlapping logical and",
            SupplementalMathematicalOperators::LogicalAndWithMiddleStem => "logical and with middle stem",
            SupplementalMathematicalOperators::LogicalOrWithMiddleStem => "logical or with middle stem",
            SupplementalMathematicalOperators::LogicalAndWithHorizontalDash => "logical and with horizontal dash",
            SupplementalMathematicalOperators::LogicalOrWithHorizontalDash => "logical or with horizontal dash",
            SupplementalMathematicalOperators::LogicalAndWithDoubleOverbar => "logical and with double overbar",
            SupplementalMathematicalOperators::LogicalAndWithUnderbar => "logical and with underbar",
            SupplementalMathematicalOperators::LogicalAndWithDoubleUnderbar => "logical and with double underbar",
            SupplementalMathematicalOperators::SmallVeeWithUnderbar => "small vee with underbar",
            SupplementalMathematicalOperators::LogicalOrWithDoubleOverbar => "logical or with double overbar",
            SupplementalMathematicalOperators::LogicalOrWithDoubleUnderbar => "logical or with double underbar",
            SupplementalMathematicalOperators::ZNotationDomainAntirestriction => "z notation domain antirestriction",
            SupplementalMathematicalOperators::ZNotationRangeAntirestriction => "z notation range antirestriction",
            SupplementalMathematicalOperators::EqualsSignWithDotBelow => "equals sign with dot below",
            SupplementalMathematicalOperators::IdenticalWithDotAbove => "identical with dot above",
            SupplementalMathematicalOperators::TripleHorizontalBarWithDoubleVerticalStroke => "triple horizontal bar with double vertical stroke",
            SupplementalMathematicalOperators::TripleHorizontalBarWithTripleVerticalStroke => "triple horizontal bar with triple vertical stroke",
            SupplementalMathematicalOperators::TildeOperatorWithDotAbove => "tilde operator with dot above",
            SupplementalMathematicalOperators::TildeOperatorWithRisingDots => "tilde operator with rising dots",
            SupplementalMathematicalOperators::SimilarMinusSimilar => "similar minus similar",
            SupplementalMathematicalOperators::CongruentWithDotAbove => "congruent with dot above",
            SupplementalMathematicalOperators::EqualsWithAsterisk => "equals with asterisk",
            SupplementalMathematicalOperators::AlmostEqualToWithCircumflexAccent => "almost equal to with circumflex accent",
            SupplementalMathematicalOperators::ApproximatelyEqualOrEqualTo => "approximately equal or equal to",
            SupplementalMathematicalOperators::EqualsSignAbovePlusSign => "equals sign above plus sign",
            SupplementalMathematicalOperators::PlusSignAboveEqualsSign => "plus sign above equals sign",
            SupplementalMathematicalOperators::EqualsSignAboveTildeOperator => "equals sign above tilde operator",
            SupplementalMathematicalOperators::DoubleColonEqual => "double colon equal",
            SupplementalMathematicalOperators::TwoConsecutiveEqualsSigns => "two consecutive equals signs",
            SupplementalMathematicalOperators::ThreeConsecutiveEqualsSigns => "three consecutive equals signs",
            SupplementalMathematicalOperators::EqualsSignWithTwoDotsAboveAndTwoDotsBelow => "equals sign with two dots above and two dots below",
            SupplementalMathematicalOperators::EquivalentWithFourDotsAbove => "equivalent with four dots above",
            SupplementalMathematicalOperators::LessDashThanWithCircleInside => "less-than with circle inside",
            SupplementalMathematicalOperators::GreaterDashThanWithCircleInside => "greater-than with circle inside",
            SupplementalMathematicalOperators::LessDashThanWithQuestionMarkAbove => "less-than with question mark above",
            SupplementalMathematicalOperators::GreaterDashThanWithQuestionMarkAbove => "greater-than with question mark above",
            SupplementalMathematicalOperators::LessDashThanOrSlantedEqualTo => "less-than or slanted equal to",
            SupplementalMathematicalOperators::GreaterDashThanOrSlantedEqualTo => "greater-than or slanted equal to",
            SupplementalMathematicalOperators::LessDashThanOrSlantedEqualToWithDotInside => "less-than or slanted equal to with dot inside",
            SupplementalMathematicalOperators::GreaterDashThanOrSlantedEqualToWithDotInside => "greater-than or slanted equal to with dot inside",
            SupplementalMathematicalOperators::LessDashThanOrSlantedEqualToWithDotAbove => "less-than or slanted equal to with dot above",
            SupplementalMathematicalOperators::GreaterDashThanOrSlantedEqualToWithDotAbove => "greater-than or slanted equal to with dot above",
            SupplementalMathematicalOperators::LessDashThanOrSlantedEqualToWithDotAboveRight => "less-than or slanted equal to with dot above right",
            SupplementalMathematicalOperators::GreaterDashThanOrSlantedEqualToWithDotAboveLeft => "greater-than or slanted equal to with dot above left",
            SupplementalMathematicalOperators::LessDashThanOrApproximate => "less-than or approximate",
            SupplementalMathematicalOperators::GreaterDashThanOrApproximate => "greater-than or approximate",
            SupplementalMathematicalOperators::LessDashThanAndSingleDashLineNotEqualTo => "less-than and single-line not equal to",
            SupplementalMathematicalOperators::GreaterDashThanAndSingleDashLineNotEqualTo => "greater-than and single-line not equal to",
            SupplementalMathematicalOperators::LessDashThanAndNotApproximate => "less-than and not approximate",
            SupplementalMathematicalOperators::GreaterDashThanAndNotApproximate => "greater-than and not approximate",
            SupplementalMathematicalOperators::LessDashThanAboveDoubleDashLineEqualAboveGreaterDashThan => "less-than above double-line equal above greater-than",
            SupplementalMathematicalOperators::GreaterDashThanAboveDoubleDashLineEqualAboveLessDashThan => "greater-than above double-line equal above less-than",
            SupplementalMathematicalOperators::LessDashThanAboveSimilarOrEqual => "less-than above similar or equal",
            SupplementalMathematicalOperators::GreaterDashThanAboveSimilarOrEqual => "greater-than above similar or equal",
            SupplementalMathematicalOperators::LessDashThanAboveSimilarAboveGreaterDashThan => "less-than above similar above greater-than",
            SupplementalMathematicalOperators::GreaterDashThanAboveSimilarAboveLessDashThan => "greater-than above similar above less-than",
            SupplementalMathematicalOperators::LessDashThanAboveGreaterDashThanAboveDoubleDashLineEqual => "less-than above greater-than above double-line equal",
            SupplementalMathematicalOperators::GreaterDashThanAboveLessDashThanAboveDoubleDashLineEqual => "greater-than above less-than above double-line equal",
            SupplementalMathematicalOperators::LessDashThanAboveSlantedEqualAboveGreaterDashThanAboveSlantedEqual => "less-than above slanted equal above greater-than above slanted equal",
            SupplementalMathematicalOperators::GreaterDashThanAboveSlantedEqualAboveLessDashThanAboveSlantedEqual => "greater-than above slanted equal above less-than above slanted equal",
            SupplementalMathematicalOperators::SlantedEqualToOrLessDashThan => "slanted equal to or less-than",
            SupplementalMathematicalOperators::SlantedEqualToOrGreaterDashThan => "slanted equal to or greater-than",
            SupplementalMathematicalOperators::SlantedEqualToOrLessDashThanWithDotInside => "slanted equal to or less-than with dot inside",
            SupplementalMathematicalOperators::SlantedEqualToOrGreaterDashThanWithDotInside => "slanted equal to or greater-than with dot inside",
            SupplementalMathematicalOperators::DoubleDashLineEqualToOrLessDashThan => "double-line equal to or less-than",
            SupplementalMathematicalOperators::DoubleDashLineEqualToOrGreaterDashThan => "double-line equal to or greater-than",
            SupplementalMathematicalOperators::DoubleDashLineSlantedEqualToOrLessDashThan => "double-line slanted equal to or less-than",
            SupplementalMathematicalOperators::DoubleDashLineSlantedEqualToOrGreaterDashThan => "double-line slanted equal to or greater-than",
            SupplementalMathematicalOperators::SimilarOrLessDashThan => "similar or less-than",
            SupplementalMathematicalOperators::SimilarOrGreaterDashThan => "similar or greater-than",
            SupplementalMathematicalOperators::SimilarAboveLessDashThanAboveEqualsSign => "similar above less-than above equals sign",
            SupplementalMathematicalOperators::SimilarAboveGreaterDashThanAboveEqualsSign => "similar above greater-than above equals sign",
            SupplementalMathematicalOperators::DoubleNestedLessDashThan => "double nested less-than",
            SupplementalMathematicalOperators::DoubleNestedGreaterDashThan => "double nested greater-than",
            SupplementalMathematicalOperators::DoubleNestedLessDashThanWithUnderbar => "double nested less-than with underbar",
            SupplementalMathematicalOperators::GreaterDashThanOverlappingLessDashThan => "greater-than overlapping less-than",
            SupplementalMathematicalOperators::GreaterDashThanBesideLessDashThan => "greater-than beside less-than",
            SupplementalMathematicalOperators::LessDashThanClosedByCurve => "less-than closed by curve",
            SupplementalMathematicalOperators::GreaterDashThanClosedByCurve => "greater-than closed by curve",
            SupplementalMathematicalOperators::LessDashThanClosedByCurveAboveSlantedEqual => "less-than closed by curve above slanted equal",
            SupplementalMathematicalOperators::GreaterDashThanClosedByCurveAboveSlantedEqual => "greater-than closed by curve above slanted equal",
            SupplementalMathematicalOperators::SmallerThan => "smaller than",
            SupplementalMathematicalOperators::LargerThan => "larger than",
            SupplementalMathematicalOperators::SmallerThanOrEqualTo => "smaller than or equal to",
            SupplementalMathematicalOperators::LargerThanOrEqualTo => "larger than or equal to",
            SupplementalMathematicalOperators::EqualsSignWithBumpyAbove => "equals sign with bumpy above",
            SupplementalMathematicalOperators::PrecedesAboveSingleDashLineEqualsSign => "precedes above single-line equals sign",
            SupplementalMathematicalOperators::SucceedsAboveSingleDashLineEqualsSign => "succeeds above single-line equals sign",
            SupplementalMathematicalOperators::PrecedesAboveSingleDashLineNotEqualTo => "precedes above single-line not equal to",
            SupplementalMathematicalOperators::SucceedsAboveSingleDashLineNotEqualTo => "succeeds above single-line not equal to",
            SupplementalMathematicalOperators::PrecedesAboveEqualsSign => "precedes above equals sign",
            SupplementalMathematicalOperators::SucceedsAboveEqualsSign => "succeeds above equals sign",
            SupplementalMathematicalOperators::PrecedesAboveNotEqualTo => "precedes above not equal to",
            SupplementalMathematicalOperators::SucceedsAboveNotEqualTo => "succeeds above not equal to",
            SupplementalMathematicalOperators::PrecedesAboveAlmostEqualTo => "precedes above almost equal to",
            SupplementalMathematicalOperators::SucceedsAboveAlmostEqualTo => "succeeds above almost equal to",
            SupplementalMathematicalOperators::PrecedesAboveNotAlmostEqualTo => "precedes above not almost equal to",
            SupplementalMathematicalOperators::SucceedsAboveNotAlmostEqualTo => "succeeds above not almost equal to",
            SupplementalMathematicalOperators::DoublePrecedes => "double precedes",
            SupplementalMathematicalOperators::DoubleSucceeds => "double succeeds",
            SupplementalMathematicalOperators::SubsetWithDot => "subset with dot",
            SupplementalMathematicalOperators::SupersetWithDot => "superset with dot",
            SupplementalMathematicalOperators::SubsetWithPlusSignBelow => "subset with plus sign below",
            SupplementalMathematicalOperators::SupersetWithPlusSignBelow => "superset with plus sign below",
            SupplementalMathematicalOperators::SubsetWithMultiplicationSignBelow => "subset with multiplication sign below",
            SupplementalMathematicalOperators::SupersetWithMultiplicationSignBelow => "superset with multiplication sign below",
            SupplementalMathematicalOperators::SubsetOfOrEqualToWithDotAbove => "subset of or equal to with dot above",
            SupplementalMathematicalOperators::SupersetOfOrEqualToWithDotAbove => "superset of or equal to with dot above",
            SupplementalMathematicalOperators::SubsetOfAboveEqualsSign => "subset of above equals sign",
            SupplementalMathematicalOperators::SupersetOfAboveEqualsSign => "superset of above equals sign",
            SupplementalMathematicalOperators::SubsetOfAboveTildeOperator => "subset of above tilde operator",
            SupplementalMathematicalOperators::SupersetOfAboveTildeOperator => "superset of above tilde operator",
            SupplementalMathematicalOperators::SubsetOfAboveAlmostEqualTo => "subset of above almost equal to",
            SupplementalMathematicalOperators::SupersetOfAboveAlmostEqualTo => "superset of above almost equal to",
            SupplementalMathematicalOperators::SubsetOfAboveNotEqualTo => "subset of above not equal to",
            SupplementalMathematicalOperators::SupersetOfAboveNotEqualTo => "superset of above not equal to",
            SupplementalMathematicalOperators::SquareLeftOpenBoxOperator => "square left open box operator",
            SupplementalMathematicalOperators::SquareRightOpenBoxOperator => "square right open box operator",
            SupplementalMathematicalOperators::ClosedSubset => "closed subset",
            SupplementalMathematicalOperators::ClosedSuperset => "closed superset",
            SupplementalMathematicalOperators::ClosedSubsetOrEqualTo => "closed subset or equal to",
            SupplementalMathematicalOperators::ClosedSupersetOrEqualTo => "closed superset or equal to",
            SupplementalMathematicalOperators::SubsetAboveSuperset => "subset above superset",
            SupplementalMathematicalOperators::SupersetAboveSubset => "superset above subset",
            SupplementalMathematicalOperators::SubsetAboveSubset => "subset above subset",
            SupplementalMathematicalOperators::SupersetAboveSuperset => "superset above superset",
            SupplementalMathematicalOperators::SupersetBesideSubset => "superset beside subset",
            SupplementalMathematicalOperators::SupersetBesideAndJoinedByDashWithSubset => "superset beside and joined by dash with subset",
            SupplementalMathematicalOperators::ElementOfOpeningDownwards => "element of opening downwards",
            SupplementalMathematicalOperators::PitchforkWithTeeTop => "pitchfork with tee top",
            SupplementalMathematicalOperators::TransversalIntersection => "transversal intersection",
            SupplementalMathematicalOperators::Forking => "forking",
            SupplementalMathematicalOperators::Nonforking => "nonforking",
            SupplementalMathematicalOperators::ShortLeftTack => "short left tack",
            SupplementalMathematicalOperators::ShortDownTack => "short down tack",
            SupplementalMathematicalOperators::ShortUpTack => "short up tack",
            SupplementalMathematicalOperators::PerpendicularWithS => "perpendicular with s",
            SupplementalMathematicalOperators::VerticalBarTripleRightTurnstile => "vertical bar triple right turnstile",
            SupplementalMathematicalOperators::DoubleVerticalBarLeftTurnstile => "double vertical bar left turnstile",
            SupplementalMathematicalOperators::VerticalBarDoubleLeftTurnstile => "vertical bar double left turnstile",
            SupplementalMathematicalOperators::DoubleVerticalBarDoubleLeftTurnstile => "double vertical bar double left turnstile",
            SupplementalMathematicalOperators::LongDashFromLeftMemberOfDoubleVertical => "long dash from left member of double vertical",
            SupplementalMathematicalOperators::ShortDownTackWithOverbar => "short down tack with overbar",
            SupplementalMathematicalOperators::ShortUpTackWithUnderbar => "short up tack with underbar",
            SupplementalMathematicalOperators::ShortUpTackAboveShortDownTack => "short up tack above short down tack",
            SupplementalMathematicalOperators::DoubleDownTack => "double down tack",
            SupplementalMathematicalOperators::DoubleUpTack => "double up tack",
            SupplementalMathematicalOperators::DoubleStrokeNotSign => "double stroke not sign",
            SupplementalMathematicalOperators::ReversedDoubleStrokeNotSign => "reversed double stroke not sign",
            SupplementalMathematicalOperators::DoesNotDivideWithReversedNegationSlash => "does not divide with reversed negation slash",
            SupplementalMathematicalOperators::VerticalLineWithCircleAbove => "vertical line with circle above",
            SupplementalMathematicalOperators::VerticalLineWithCircleBelow => "vertical line with circle below",
            SupplementalMathematicalOperators::DownTackWithCircleBelow => "down tack with circle below",
            SupplementalMathematicalOperators::ParallelWithHorizontalStroke => "parallel with horizontal stroke",
            SupplementalMathematicalOperators::ParallelWithTildeOperator => "parallel with tilde operator",
            SupplementalMathematicalOperators::TripleVerticalBarBinaryRelation => "triple vertical bar binary relation",
            SupplementalMathematicalOperators::TripleVerticalBarWithHorizontalStroke => "triple vertical bar with horizontal stroke",
            SupplementalMathematicalOperators::TripleColonOperator => "triple colon operator",
            SupplementalMathematicalOperators::TripleNestedLessDashThan => "triple nested less-than",
            SupplementalMathematicalOperators::TripleNestedGreaterDashThan => "triple nested greater-than",
            SupplementalMathematicalOperators::DoubleDashLineSlantedLessDashThanOrEqualTo => "double-line slanted less-than or equal to",
            SupplementalMathematicalOperators::DoubleDashLineSlantedGreaterDashThanOrEqualTo => "double-line slanted greater-than or equal to",
            SupplementalMathematicalOperators::TripleSolidusBinaryRelation => "triple solidus binary relation",
            SupplementalMathematicalOperators::LargeTripleVerticalBarOperator => "large triple vertical bar operator",
            SupplementalMathematicalOperators::DoubleSolidusOperator => "double solidus operator",
            SupplementalMathematicalOperators::WhiteVerticalBar => "white vertical bar",
        }
    }
}
