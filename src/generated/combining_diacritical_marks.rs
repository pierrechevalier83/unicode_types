/// \u{300} → \u{36f}\
///\
/// ̀ ́ ̂ ̃ ̄ ̅ ̆ ̇ ̈ ̉ ̊ ̋ ̌ ̍ ̎ ̏\
/// ̐ ̑ ̒ ̓ ̔ ̕ ̖ ̗ ̘ ̙ ̚ ̛ ̜ ̝ ̞ ̟\
/// ̠ ̡ ̢ ̣ ̤ ̥ ̦ ̧ ̨ ̩ ̪ ̫ ̬ ̭ ̮ ̯\
/// ̰ ̱ ̲ ̳ ̴ ̵ ̶ ̷ ̸ ̹ ̺ ̻ ̼ ̽ ̾ ̿\
/// ̀ ́ ͂ ̓ ̈́ ͅ ͆ ͇ ͈ ͉ ͊ ͋ ͌ ͍ ͎ ͏\
/// ͐ ͑ ͒ ͓ ͔ ͕ ͖ ͗ ͘ ͙ ͚ ͛ ͜ ͝ ͞ ͟\
/// ͠ ͡ ͢ ͣ ͤ ͥ ͦ ͧ ͨ ͩ ͪ ͫ ͬ ͭ ͮ\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{300}: '̀'
    pub const COMBINING_GRAVE_ACCENT: char = '̀';
    /// \u{301}: '́'
    pub const COMBINING_ACUTE_ACCENT: char = '́';
    /// \u{302}: '̂'
    pub const COMBINING_CIRCUMFLEX_ACCENT: char = '̂';
    /// \u{303}: '̃'
    pub const COMBINING_TILDE: char = '̃';
    /// \u{304}: '̄'
    pub const COMBINING_MACRON: char = '̄';
    /// \u{305}: '̅'
    pub const COMBINING_OVERLINE: char = '̅';
    /// \u{306}: '̆'
    pub const COMBINING_BREVE: char = '̆';
    /// \u{307}: '̇'
    pub const COMBINING_DOT_ABOVE: char = '̇';
    /// \u{308}: '̈'
    pub const COMBINING_DIAERESIS: char = '̈';
    /// \u{309}: '̉'
    pub const COMBINING_HOOK_ABOVE: char = '̉';
    /// \u{30a}: '̊'
    pub const COMBINING_RING_ABOVE: char = '̊';
    /// \u{30b}: '̋'
    pub const COMBINING_DOUBLE_ACUTE_ACCENT: char = '̋';
    /// \u{30c}: '̌'
    pub const COMBINING_CARON: char = '̌';
    /// \u{30d}: '̍'
    pub const COMBINING_VERTICAL_LINE_ABOVE: char = '̍';
    /// \u{30e}: '̎'
    pub const COMBINING_DOUBLE_VERTICAL_LINE_ABOVE: char = '̎';
    /// \u{30f}: '̏'
    pub const COMBINING_DOUBLE_GRAVE_ACCENT: char = '̏';
    /// \u{310}: '̐'
    pub const COMBINING_CANDRABINDU: char = '̐';
    /// \u{311}: '̑'
    pub const COMBINING_INVERTED_BREVE: char = '̑';
    /// \u{312}: '̒'
    pub const COMBINING_TURNED_COMMA_ABOVE: char = '̒';
    /// \u{313}: '̓'
    pub const COMBINING_COMMA_ABOVE: char = '̓';
    /// \u{314}: '̔'
    pub const COMBINING_REVERSED_COMMA_ABOVE: char = '̔';
    /// \u{315}: '̕'
    pub const COMBINING_COMMA_ABOVE_RIGHT: char = '̕';
    /// \u{316}: '̖'
    pub const COMBINING_GRAVE_ACCENT_BELOW: char = '̖';
    /// \u{317}: '̗'
    pub const COMBINING_ACUTE_ACCENT_BELOW: char = '̗';
    /// \u{318}: '̘'
    pub const COMBINING_LEFT_TACK_BELOW: char = '̘';
    /// \u{319}: '̙'
    pub const COMBINING_RIGHT_TACK_BELOW: char = '̙';
    /// \u{31a}: '̚'
    pub const COMBINING_LEFT_ANGLE_ABOVE: char = '̚';
    /// \u{31b}: '̛'
    pub const COMBINING_HORN: char = '̛';
    /// \u{31c}: '̜'
    pub const COMBINING_LEFT_HALF_RING_BELOW: char = '̜';
    /// \u{31d}: '̝'
    pub const COMBINING_UP_TACK_BELOW: char = '̝';
    /// \u{31e}: '̞'
    pub const COMBINING_DOWN_TACK_BELOW: char = '̞';
    /// \u{31f}: '̟'
    pub const COMBINING_PLUS_SIGN_BELOW: char = '̟';
    /// \u{320}: '̠'
    pub const COMBINING_MINUS_SIGN_BELOW: char = '̠';
    /// \u{321}: '̡'
    pub const COMBINING_PALATALIZED_HOOK_BELOW: char = '̡';
    /// \u{322}: '̢'
    pub const COMBINING_RETROFLEX_HOOK_BELOW: char = '̢';
    /// \u{323}: '̣'
    pub const COMBINING_DOT_BELOW: char = '̣';
    /// \u{324}: '̤'
    pub const COMBINING_DIAERESIS_BELOW: char = '̤';
    /// \u{325}: '̥'
    pub const COMBINING_RING_BELOW: char = '̥';
    /// \u{326}: '̦'
    pub const COMBINING_COMMA_BELOW: char = '̦';
    /// \u{327}: '̧'
    pub const COMBINING_CEDILLA: char = '̧';
    /// \u{328}: '̨'
    pub const COMBINING_OGONEK: char = '̨';
    /// \u{329}: '̩'
    pub const COMBINING_VERTICAL_LINE_BELOW: char = '̩';
    /// \u{32a}: '̪'
    pub const COMBINING_BRIDGE_BELOW: char = '̪';
    /// \u{32b}: '̫'
    pub const COMBINING_INVERTED_DOUBLE_ARCH_BELOW: char = '̫';
    /// \u{32c}: '̬'
    pub const COMBINING_CARON_BELOW: char = '̬';
    /// \u{32d}: '̭'
    pub const COMBINING_CIRCUMFLEX_ACCENT_BELOW: char = '̭';
    /// \u{32e}: '̮'
    pub const COMBINING_BREVE_BELOW: char = '̮';
    /// \u{32f}: '̯'
    pub const COMBINING_INVERTED_BREVE_BELOW: char = '̯';
    /// \u{330}: '̰'
    pub const COMBINING_TILDE_BELOW: char = '̰';
    /// \u{331}: '̱'
    pub const COMBINING_MACRON_BELOW: char = '̱';
    /// \u{332}: '̲'
    pub const COMBINING_LOW_LINE: char = '̲';
    /// \u{333}: '̳'
    pub const COMBINING_DOUBLE_LOW_LINE: char = '̳';
    /// \u{334}: '̴'
    pub const COMBINING_TILDE_OVERLAY: char = '̴';
    /// \u{335}: '̵'
    pub const COMBINING_SHORT_STROKE_OVERLAY: char = '̵';
    /// \u{336}: '̶'
    pub const COMBINING_LONG_STROKE_OVERLAY: char = '̶';
    /// \u{337}: '̷'
    pub const COMBINING_SHORT_SOLIDUS_OVERLAY: char = '̷';
    /// \u{338}: '̸'
    pub const COMBINING_LONG_SOLIDUS_OVERLAY: char = '̸';
    /// \u{339}: '̹'
    pub const COMBINING_RIGHT_HALF_RING_BELOW: char = '̹';
    /// \u{33a}: '̺'
    pub const COMBINING_INVERTED_BRIDGE_BELOW: char = '̺';
    /// \u{33b}: '̻'
    pub const COMBINING_SQUARE_BELOW: char = '̻';
    /// \u{33c}: '̼'
    pub const COMBINING_SEAGULL_BELOW: char = '̼';
    /// \u{33d}: '̽'
    pub const COMBINING_X_ABOVE: char = '̽';
    /// \u{33e}: '̾'
    pub const COMBINING_VERTICAL_TILDE: char = '̾';
    /// \u{33f}: '̿'
    pub const COMBINING_DOUBLE_OVERLINE: char = '̿';
    /// \u{340}: '̀'
    pub const COMBINING_GRAVE_TONE_MARK: char = '̀';
    /// \u{341}: '́'
    pub const COMBINING_ACUTE_TONE_MARK: char = '́';
    /// \u{342}: '͂'
    pub const COMBINING_GREEK_PERISPOMENI: char = '͂';
    /// \u{343}: '̓'
    pub const COMBINING_GREEK_KORONIS: char = '̓';
    /// \u{344}: '̈́'
    pub const COMBINING_GREEK_DIALYTIKA_TONOS: char = '̈́';
    /// \u{345}: 'ͅ'
    pub const COMBINING_GREEK_YPOGEGRAMMENI: char = 'ͅ';
    /// \u{346}: '͆'
    pub const COMBINING_BRIDGE_ABOVE: char = '͆';
    /// \u{347}: '͇'
    pub const COMBINING_EQUALS_SIGN_BELOW: char = '͇';
    /// \u{348}: '͈'
    pub const COMBINING_DOUBLE_VERTICAL_LINE_BELOW: char = '͈';
    /// \u{349}: '͉'
    pub const COMBINING_LEFT_ANGLE_BELOW: char = '͉';
    /// \u{34a}: '͊'
    pub const COMBINING_NOT_TILDE_ABOVE: char = '͊';
    /// \u{34b}: '͋'
    pub const COMBINING_HOMOTHETIC_ABOVE: char = '͋';
    /// \u{34c}: '͌'
    pub const COMBINING_ALMOST_EQUAL_TO_ABOVE: char = '͌';
    /// \u{34d}: '͍'
    pub const COMBINING_LEFT_RIGHT_ARROW_BELOW: char = '͍';
    /// \u{34e}: '͎'
    pub const COMBINING_UPWARDS_ARROW_BELOW: char = '͎';
    /// \u{34f}: '͏'
    pub const COMBINING_GRAPHEME_JOINER: char = '͏';
    /// \u{350}: '͐'
    pub const COMBINING_RIGHT_ARROWHEAD_ABOVE: char = '͐';
    /// \u{351}: '͑'
    pub const COMBINING_LEFT_HALF_RING_ABOVE: char = '͑';
    /// \u{352}: '͒'
    pub const COMBINING_FERMATA: char = '͒';
    /// \u{353}: '͓'
    pub const COMBINING_X_BELOW: char = '͓';
    /// \u{354}: '͔'
    pub const COMBINING_LEFT_ARROWHEAD_BELOW: char = '͔';
    /// \u{355}: '͕'
    pub const COMBINING_RIGHT_ARROWHEAD_BELOW: char = '͕';
    /// \u{356}: '͖'
    pub const COMBINING_RIGHT_ARROWHEAD_AND_UP_ARROWHEAD_BELOW: char = '͖';
    /// \u{357}: '͗'
    pub const COMBINING_RIGHT_HALF_RING_ABOVE: char = '͗';
    /// \u{358}: '͘'
    pub const COMBINING_DOT_ABOVE_RIGHT: char = '͘';
    /// \u{359}: '͙'
    pub const COMBINING_ASTERISK_BELOW: char = '͙';
    /// \u{35a}: '͚'
    pub const COMBINING_DOUBLE_RING_BELOW: char = '͚';
    /// \u{35b}: '͛'
    pub const COMBINING_ZIGZAG_ABOVE: char = '͛';
    /// \u{35c}: '͜'
    pub const COMBINING_DOUBLE_BREVE_BELOW: char = '͜';
    /// \u{35d}: '͝'
    pub const COMBINING_DOUBLE_BREVE: char = '͝';
    /// \u{35e}: '͞'
    pub const COMBINING_DOUBLE_MACRON: char = '͞';
    /// \u{35f}: '͟'
    pub const COMBINING_DOUBLE_MACRON_BELOW: char = '͟';
    /// \u{360}: '͠'
    pub const COMBINING_DOUBLE_TILDE: char = '͠';
    /// \u{361}: '͡'
    pub const COMBINING_DOUBLE_INVERTED_BREVE: char = '͡';
    /// \u{362}: '͢'
    pub const COMBINING_DOUBLE_RIGHTWARDS_ARROW_BELOW: char = '͢';
    /// \u{363}: 'ͣ'
    pub const COMBINING_LATIN_SMALL_LETTER_A: char = 'ͣ';
    /// \u{364}: 'ͤ'
    pub const COMBINING_LATIN_SMALL_LETTER_E: char = 'ͤ';
    /// \u{365}: 'ͥ'
    pub const COMBINING_LATIN_SMALL_LETTER_I: char = 'ͥ';
    /// \u{366}: 'ͦ'
    pub const COMBINING_LATIN_SMALL_LETTER_O: char = 'ͦ';
    /// \u{367}: 'ͧ'
    pub const COMBINING_LATIN_SMALL_LETTER_U: char = 'ͧ';
    /// \u{368}: 'ͨ'
    pub const COMBINING_LATIN_SMALL_LETTER_C: char = 'ͨ';
    /// \u{369}: 'ͩ'
    pub const COMBINING_LATIN_SMALL_LETTER_D: char = 'ͩ';
    /// \u{36a}: 'ͪ'
    pub const COMBINING_LATIN_SMALL_LETTER_H: char = 'ͪ';
    /// \u{36b}: 'ͫ'
    pub const COMBINING_LATIN_SMALL_LETTER_M: char = 'ͫ';
    /// \u{36c}: 'ͬ'
    pub const COMBINING_LATIN_SMALL_LETTER_R: char = 'ͬ';
    /// \u{36d}: 'ͭ'
    pub const COMBINING_LATIN_SMALL_LETTER_T: char = 'ͭ';
    /// \u{36e}: 'ͮ'
    pub const COMBINING_LATIN_SMALL_LETTER_V: char = 'ͮ';
}

/// An enum to represent all characters in the CombiningDiacriticalMarks block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum CombiningDiacriticalMarks {
    /// \u{300}: '̀'
    CombiningGraveAccent,
    /// \u{301}: '́'
    CombiningAcuteAccent,
    /// \u{302}: '̂'
    CombiningCircumflexAccent,
    /// \u{303}: '̃'
    CombiningTilde,
    /// \u{304}: '̄'
    CombiningMacron,
    /// \u{305}: '̅'
    CombiningOverline,
    /// \u{306}: '̆'
    CombiningBreve,
    /// \u{307}: '̇'
    CombiningDotAbove,
    /// \u{308}: '̈'
    CombiningDiaeresis,
    /// \u{309}: '̉'
    CombiningHookAbove,
    /// \u{30a}: '̊'
    CombiningRingAbove,
    /// \u{30b}: '̋'
    CombiningDoubleAcuteAccent,
    /// \u{30c}: '̌'
    CombiningCaron,
    /// \u{30d}: '̍'
    CombiningVerticalLineAbove,
    /// \u{30e}: '̎'
    CombiningDoubleVerticalLineAbove,
    /// \u{30f}: '̏'
    CombiningDoubleGraveAccent,
    /// \u{310}: '̐'
    CombiningCandrabindu,
    /// \u{311}: '̑'
    CombiningInvertedBreve,
    /// \u{312}: '̒'
    CombiningTurnedCommaAbove,
    /// \u{313}: '̓'
    CombiningCommaAbove,
    /// \u{314}: '̔'
    CombiningReversedCommaAbove,
    /// \u{315}: '̕'
    CombiningCommaAboveRight,
    /// \u{316}: '̖'
    CombiningGraveAccentBelow,
    /// \u{317}: '̗'
    CombiningAcuteAccentBelow,
    /// \u{318}: '̘'
    CombiningLeftTackBelow,
    /// \u{319}: '̙'
    CombiningRightTackBelow,
    /// \u{31a}: '̚'
    CombiningLeftAngleAbove,
    /// \u{31b}: '̛'
    CombiningHorn,
    /// \u{31c}: '̜'
    CombiningLeftHalfRingBelow,
    /// \u{31d}: '̝'
    CombiningUpTackBelow,
    /// \u{31e}: '̞'
    CombiningDownTackBelow,
    /// \u{31f}: '̟'
    CombiningPlusSignBelow,
    /// \u{320}: '̠'
    CombiningMinusSignBelow,
    /// \u{321}: '̡'
    CombiningPalatalizedHookBelow,
    /// \u{322}: '̢'
    CombiningRetroflexHookBelow,
    /// \u{323}: '̣'
    CombiningDotBelow,
    /// \u{324}: '̤'
    CombiningDiaeresisBelow,
    /// \u{325}: '̥'
    CombiningRingBelow,
    /// \u{326}: '̦'
    CombiningCommaBelow,
    /// \u{327}: '̧'
    CombiningCedilla,
    /// \u{328}: '̨'
    CombiningOgonek,
    /// \u{329}: '̩'
    CombiningVerticalLineBelow,
    /// \u{32a}: '̪'
    CombiningBridgeBelow,
    /// \u{32b}: '̫'
    CombiningInvertedDoubleArchBelow,
    /// \u{32c}: '̬'
    CombiningCaronBelow,
    /// \u{32d}: '̭'
    CombiningCircumflexAccentBelow,
    /// \u{32e}: '̮'
    CombiningBreveBelow,
    /// \u{32f}: '̯'
    CombiningInvertedBreveBelow,
    /// \u{330}: '̰'
    CombiningTildeBelow,
    /// \u{331}: '̱'
    CombiningMacronBelow,
    /// \u{332}: '̲'
    CombiningLowLine,
    /// \u{333}: '̳'
    CombiningDoubleLowLine,
    /// \u{334}: '̴'
    CombiningTildeOverlay,
    /// \u{335}: '̵'
    CombiningShortStrokeOverlay,
    /// \u{336}: '̶'
    CombiningLongStrokeOverlay,
    /// \u{337}: '̷'
    CombiningShortSolidusOverlay,
    /// \u{338}: '̸'
    CombiningLongSolidusOverlay,
    /// \u{339}: '̹'
    CombiningRightHalfRingBelow,
    /// \u{33a}: '̺'
    CombiningInvertedBridgeBelow,
    /// \u{33b}: '̻'
    CombiningSquareBelow,
    /// \u{33c}: '̼'
    CombiningSeagullBelow,
    /// \u{33d}: '̽'
    CombiningXAbove,
    /// \u{33e}: '̾'
    CombiningVerticalTilde,
    /// \u{33f}: '̿'
    CombiningDoubleOverline,
    /// \u{340}: '̀'
    CombiningGraveToneMark,
    /// \u{341}: '́'
    CombiningAcuteToneMark,
    /// \u{342}: '͂'
    CombiningGreekPerispomeni,
    /// \u{343}: '̓'
    CombiningGreekKoronis,
    /// \u{344}: '̈́'
    CombiningGreekDialytikaTonos,
    /// \u{345}: 'ͅ'
    CombiningGreekYpogegrammeni,
    /// \u{346}: '͆'
    CombiningBridgeAbove,
    /// \u{347}: '͇'
    CombiningEqualsSignBelow,
    /// \u{348}: '͈'
    CombiningDoubleVerticalLineBelow,
    /// \u{349}: '͉'
    CombiningLeftAngleBelow,
    /// \u{34a}: '͊'
    CombiningNotTildeAbove,
    /// \u{34b}: '͋'
    CombiningHomotheticAbove,
    /// \u{34c}: '͌'
    CombiningAlmostEqualToAbove,
    /// \u{34d}: '͍'
    CombiningLeftRightArrowBelow,
    /// \u{34e}: '͎'
    CombiningUpwardsArrowBelow,
    /// \u{34f}: '͏'
    CombiningGraphemeJoiner,
    /// \u{350}: '͐'
    CombiningRightArrowheadAbove,
    /// \u{351}: '͑'
    CombiningLeftHalfRingAbove,
    /// \u{352}: '͒'
    CombiningFermata,
    /// \u{353}: '͓'
    CombiningXBelow,
    /// \u{354}: '͔'
    CombiningLeftArrowheadBelow,
    /// \u{355}: '͕'
    CombiningRightArrowheadBelow,
    /// \u{356}: '͖'
    CombiningRightArrowheadAndUpArrowheadBelow,
    /// \u{357}: '͗'
    CombiningRightHalfRingAbove,
    /// \u{358}: '͘'
    CombiningDotAboveRight,
    /// \u{359}: '͙'
    CombiningAsteriskBelow,
    /// \u{35a}: '͚'
    CombiningDoubleRingBelow,
    /// \u{35b}: '͛'
    CombiningZigzagAbove,
    /// \u{35c}: '͜'
    CombiningDoubleBreveBelow,
    /// \u{35d}: '͝'
    CombiningDoubleBreve,
    /// \u{35e}: '͞'
    CombiningDoubleMacron,
    /// \u{35f}: '͟'
    CombiningDoubleMacronBelow,
    /// \u{360}: '͠'
    CombiningDoubleTilde,
    /// \u{361}: '͡'
    CombiningDoubleInvertedBreve,
    /// \u{362}: '͢'
    CombiningDoubleRightwardsArrowBelow,
    /// \u{363}: 'ͣ'
    CombiningLatinSmallLetterA,
    /// \u{364}: 'ͤ'
    CombiningLatinSmallLetterE,
    /// \u{365}: 'ͥ'
    CombiningLatinSmallLetterI,
    /// \u{366}: 'ͦ'
    CombiningLatinSmallLetterO,
    /// \u{367}: 'ͧ'
    CombiningLatinSmallLetterU,
    /// \u{368}: 'ͨ'
    CombiningLatinSmallLetterC,
    /// \u{369}: 'ͩ'
    CombiningLatinSmallLetterD,
    /// \u{36a}: 'ͪ'
    CombiningLatinSmallLetterH,
    /// \u{36b}: 'ͫ'
    CombiningLatinSmallLetterM,
    /// \u{36c}: 'ͬ'
    CombiningLatinSmallLetterR,
    /// \u{36d}: 'ͭ'
    CombiningLatinSmallLetterT,
    /// \u{36e}: 'ͮ'
    CombiningLatinSmallLetterV,
}

impl Into<char> for CombiningDiacriticalMarks {
    fn into(self) -> char {
        use constants::*;
        match self {
            CombiningDiacriticalMarks::CombiningGraveAccent => COMBINING_GRAVE_ACCENT,
            CombiningDiacriticalMarks::CombiningAcuteAccent => COMBINING_ACUTE_ACCENT,
            CombiningDiacriticalMarks::CombiningCircumflexAccent => COMBINING_CIRCUMFLEX_ACCENT,
            CombiningDiacriticalMarks::CombiningTilde => COMBINING_TILDE,
            CombiningDiacriticalMarks::CombiningMacron => COMBINING_MACRON,
            CombiningDiacriticalMarks::CombiningOverline => COMBINING_OVERLINE,
            CombiningDiacriticalMarks::CombiningBreve => COMBINING_BREVE,
            CombiningDiacriticalMarks::CombiningDotAbove => COMBINING_DOT_ABOVE,
            CombiningDiacriticalMarks::CombiningDiaeresis => COMBINING_DIAERESIS,
            CombiningDiacriticalMarks::CombiningHookAbove => COMBINING_HOOK_ABOVE,
            CombiningDiacriticalMarks::CombiningRingAbove => COMBINING_RING_ABOVE,
            CombiningDiacriticalMarks::CombiningDoubleAcuteAccent => COMBINING_DOUBLE_ACUTE_ACCENT,
            CombiningDiacriticalMarks::CombiningCaron => COMBINING_CARON,
            CombiningDiacriticalMarks::CombiningVerticalLineAbove => COMBINING_VERTICAL_LINE_ABOVE,
            CombiningDiacriticalMarks::CombiningDoubleVerticalLineAbove => COMBINING_DOUBLE_VERTICAL_LINE_ABOVE,
            CombiningDiacriticalMarks::CombiningDoubleGraveAccent => COMBINING_DOUBLE_GRAVE_ACCENT,
            CombiningDiacriticalMarks::CombiningCandrabindu => COMBINING_CANDRABINDU,
            CombiningDiacriticalMarks::CombiningInvertedBreve => COMBINING_INVERTED_BREVE,
            CombiningDiacriticalMarks::CombiningTurnedCommaAbove => COMBINING_TURNED_COMMA_ABOVE,
            CombiningDiacriticalMarks::CombiningCommaAbove => COMBINING_COMMA_ABOVE,
            CombiningDiacriticalMarks::CombiningReversedCommaAbove => COMBINING_REVERSED_COMMA_ABOVE,
            CombiningDiacriticalMarks::CombiningCommaAboveRight => COMBINING_COMMA_ABOVE_RIGHT,
            CombiningDiacriticalMarks::CombiningGraveAccentBelow => COMBINING_GRAVE_ACCENT_BELOW,
            CombiningDiacriticalMarks::CombiningAcuteAccentBelow => COMBINING_ACUTE_ACCENT_BELOW,
            CombiningDiacriticalMarks::CombiningLeftTackBelow => COMBINING_LEFT_TACK_BELOW,
            CombiningDiacriticalMarks::CombiningRightTackBelow => COMBINING_RIGHT_TACK_BELOW,
            CombiningDiacriticalMarks::CombiningLeftAngleAbove => COMBINING_LEFT_ANGLE_ABOVE,
            CombiningDiacriticalMarks::CombiningHorn => COMBINING_HORN,
            CombiningDiacriticalMarks::CombiningLeftHalfRingBelow => COMBINING_LEFT_HALF_RING_BELOW,
            CombiningDiacriticalMarks::CombiningUpTackBelow => COMBINING_UP_TACK_BELOW,
            CombiningDiacriticalMarks::CombiningDownTackBelow => COMBINING_DOWN_TACK_BELOW,
            CombiningDiacriticalMarks::CombiningPlusSignBelow => COMBINING_PLUS_SIGN_BELOW,
            CombiningDiacriticalMarks::CombiningMinusSignBelow => COMBINING_MINUS_SIGN_BELOW,
            CombiningDiacriticalMarks::CombiningPalatalizedHookBelow => COMBINING_PALATALIZED_HOOK_BELOW,
            CombiningDiacriticalMarks::CombiningRetroflexHookBelow => COMBINING_RETROFLEX_HOOK_BELOW,
            CombiningDiacriticalMarks::CombiningDotBelow => COMBINING_DOT_BELOW,
            CombiningDiacriticalMarks::CombiningDiaeresisBelow => COMBINING_DIAERESIS_BELOW,
            CombiningDiacriticalMarks::CombiningRingBelow => COMBINING_RING_BELOW,
            CombiningDiacriticalMarks::CombiningCommaBelow => COMBINING_COMMA_BELOW,
            CombiningDiacriticalMarks::CombiningCedilla => COMBINING_CEDILLA,
            CombiningDiacriticalMarks::CombiningOgonek => COMBINING_OGONEK,
            CombiningDiacriticalMarks::CombiningVerticalLineBelow => COMBINING_VERTICAL_LINE_BELOW,
            CombiningDiacriticalMarks::CombiningBridgeBelow => COMBINING_BRIDGE_BELOW,
            CombiningDiacriticalMarks::CombiningInvertedDoubleArchBelow => COMBINING_INVERTED_DOUBLE_ARCH_BELOW,
            CombiningDiacriticalMarks::CombiningCaronBelow => COMBINING_CARON_BELOW,
            CombiningDiacriticalMarks::CombiningCircumflexAccentBelow => COMBINING_CIRCUMFLEX_ACCENT_BELOW,
            CombiningDiacriticalMarks::CombiningBreveBelow => COMBINING_BREVE_BELOW,
            CombiningDiacriticalMarks::CombiningInvertedBreveBelow => COMBINING_INVERTED_BREVE_BELOW,
            CombiningDiacriticalMarks::CombiningTildeBelow => COMBINING_TILDE_BELOW,
            CombiningDiacriticalMarks::CombiningMacronBelow => COMBINING_MACRON_BELOW,
            CombiningDiacriticalMarks::CombiningLowLine => COMBINING_LOW_LINE,
            CombiningDiacriticalMarks::CombiningDoubleLowLine => COMBINING_DOUBLE_LOW_LINE,
            CombiningDiacriticalMarks::CombiningTildeOverlay => COMBINING_TILDE_OVERLAY,
            CombiningDiacriticalMarks::CombiningShortStrokeOverlay => COMBINING_SHORT_STROKE_OVERLAY,
            CombiningDiacriticalMarks::CombiningLongStrokeOverlay => COMBINING_LONG_STROKE_OVERLAY,
            CombiningDiacriticalMarks::CombiningShortSolidusOverlay => COMBINING_SHORT_SOLIDUS_OVERLAY,
            CombiningDiacriticalMarks::CombiningLongSolidusOverlay => COMBINING_LONG_SOLIDUS_OVERLAY,
            CombiningDiacriticalMarks::CombiningRightHalfRingBelow => COMBINING_RIGHT_HALF_RING_BELOW,
            CombiningDiacriticalMarks::CombiningInvertedBridgeBelow => COMBINING_INVERTED_BRIDGE_BELOW,
            CombiningDiacriticalMarks::CombiningSquareBelow => COMBINING_SQUARE_BELOW,
            CombiningDiacriticalMarks::CombiningSeagullBelow => COMBINING_SEAGULL_BELOW,
            CombiningDiacriticalMarks::CombiningXAbove => COMBINING_X_ABOVE,
            CombiningDiacriticalMarks::CombiningVerticalTilde => COMBINING_VERTICAL_TILDE,
            CombiningDiacriticalMarks::CombiningDoubleOverline => COMBINING_DOUBLE_OVERLINE,
            CombiningDiacriticalMarks::CombiningGraveToneMark => COMBINING_GRAVE_TONE_MARK,
            CombiningDiacriticalMarks::CombiningAcuteToneMark => COMBINING_ACUTE_TONE_MARK,
            CombiningDiacriticalMarks::CombiningGreekPerispomeni => COMBINING_GREEK_PERISPOMENI,
            CombiningDiacriticalMarks::CombiningGreekKoronis => COMBINING_GREEK_KORONIS,
            CombiningDiacriticalMarks::CombiningGreekDialytikaTonos => COMBINING_GREEK_DIALYTIKA_TONOS,
            CombiningDiacriticalMarks::CombiningGreekYpogegrammeni => COMBINING_GREEK_YPOGEGRAMMENI,
            CombiningDiacriticalMarks::CombiningBridgeAbove => COMBINING_BRIDGE_ABOVE,
            CombiningDiacriticalMarks::CombiningEqualsSignBelow => COMBINING_EQUALS_SIGN_BELOW,
            CombiningDiacriticalMarks::CombiningDoubleVerticalLineBelow => COMBINING_DOUBLE_VERTICAL_LINE_BELOW,
            CombiningDiacriticalMarks::CombiningLeftAngleBelow => COMBINING_LEFT_ANGLE_BELOW,
            CombiningDiacriticalMarks::CombiningNotTildeAbove => COMBINING_NOT_TILDE_ABOVE,
            CombiningDiacriticalMarks::CombiningHomotheticAbove => COMBINING_HOMOTHETIC_ABOVE,
            CombiningDiacriticalMarks::CombiningAlmostEqualToAbove => COMBINING_ALMOST_EQUAL_TO_ABOVE,
            CombiningDiacriticalMarks::CombiningLeftRightArrowBelow => COMBINING_LEFT_RIGHT_ARROW_BELOW,
            CombiningDiacriticalMarks::CombiningUpwardsArrowBelow => COMBINING_UPWARDS_ARROW_BELOW,
            CombiningDiacriticalMarks::CombiningGraphemeJoiner => COMBINING_GRAPHEME_JOINER,
            CombiningDiacriticalMarks::CombiningRightArrowheadAbove => COMBINING_RIGHT_ARROWHEAD_ABOVE,
            CombiningDiacriticalMarks::CombiningLeftHalfRingAbove => COMBINING_LEFT_HALF_RING_ABOVE,
            CombiningDiacriticalMarks::CombiningFermata => COMBINING_FERMATA,
            CombiningDiacriticalMarks::CombiningXBelow => COMBINING_X_BELOW,
            CombiningDiacriticalMarks::CombiningLeftArrowheadBelow => COMBINING_LEFT_ARROWHEAD_BELOW,
            CombiningDiacriticalMarks::CombiningRightArrowheadBelow => COMBINING_RIGHT_ARROWHEAD_BELOW,
            CombiningDiacriticalMarks::CombiningRightArrowheadAndUpArrowheadBelow => COMBINING_RIGHT_ARROWHEAD_AND_UP_ARROWHEAD_BELOW,
            CombiningDiacriticalMarks::CombiningRightHalfRingAbove => COMBINING_RIGHT_HALF_RING_ABOVE,
            CombiningDiacriticalMarks::CombiningDotAboveRight => COMBINING_DOT_ABOVE_RIGHT,
            CombiningDiacriticalMarks::CombiningAsteriskBelow => COMBINING_ASTERISK_BELOW,
            CombiningDiacriticalMarks::CombiningDoubleRingBelow => COMBINING_DOUBLE_RING_BELOW,
            CombiningDiacriticalMarks::CombiningZigzagAbove => COMBINING_ZIGZAG_ABOVE,
            CombiningDiacriticalMarks::CombiningDoubleBreveBelow => COMBINING_DOUBLE_BREVE_BELOW,
            CombiningDiacriticalMarks::CombiningDoubleBreve => COMBINING_DOUBLE_BREVE,
            CombiningDiacriticalMarks::CombiningDoubleMacron => COMBINING_DOUBLE_MACRON,
            CombiningDiacriticalMarks::CombiningDoubleMacronBelow => COMBINING_DOUBLE_MACRON_BELOW,
            CombiningDiacriticalMarks::CombiningDoubleTilde => COMBINING_DOUBLE_TILDE,
            CombiningDiacriticalMarks::CombiningDoubleInvertedBreve => COMBINING_DOUBLE_INVERTED_BREVE,
            CombiningDiacriticalMarks::CombiningDoubleRightwardsArrowBelow => COMBINING_DOUBLE_RIGHTWARDS_ARROW_BELOW,
            CombiningDiacriticalMarks::CombiningLatinSmallLetterA => COMBINING_LATIN_SMALL_LETTER_A,
            CombiningDiacriticalMarks::CombiningLatinSmallLetterE => COMBINING_LATIN_SMALL_LETTER_E,
            CombiningDiacriticalMarks::CombiningLatinSmallLetterI => COMBINING_LATIN_SMALL_LETTER_I,
            CombiningDiacriticalMarks::CombiningLatinSmallLetterO => COMBINING_LATIN_SMALL_LETTER_O,
            CombiningDiacriticalMarks::CombiningLatinSmallLetterU => COMBINING_LATIN_SMALL_LETTER_U,
            CombiningDiacriticalMarks::CombiningLatinSmallLetterC => COMBINING_LATIN_SMALL_LETTER_C,
            CombiningDiacriticalMarks::CombiningLatinSmallLetterD => COMBINING_LATIN_SMALL_LETTER_D,
            CombiningDiacriticalMarks::CombiningLatinSmallLetterH => COMBINING_LATIN_SMALL_LETTER_H,
            CombiningDiacriticalMarks::CombiningLatinSmallLetterM => COMBINING_LATIN_SMALL_LETTER_M,
            CombiningDiacriticalMarks::CombiningLatinSmallLetterR => COMBINING_LATIN_SMALL_LETTER_R,
            CombiningDiacriticalMarks::CombiningLatinSmallLetterT => COMBINING_LATIN_SMALL_LETTER_T,
            CombiningDiacriticalMarks::CombiningLatinSmallLetterV => COMBINING_LATIN_SMALL_LETTER_V,
        }
    }
}

impl std::convert::TryFrom<char> for CombiningDiacriticalMarks {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            COMBINING_GRAVE_ACCENT => Ok(CombiningDiacriticalMarks::CombiningGraveAccent),
            COMBINING_ACUTE_ACCENT => Ok(CombiningDiacriticalMarks::CombiningAcuteAccent),
            COMBINING_CIRCUMFLEX_ACCENT => Ok(CombiningDiacriticalMarks::CombiningCircumflexAccent),
            COMBINING_TILDE => Ok(CombiningDiacriticalMarks::CombiningTilde),
            COMBINING_MACRON => Ok(CombiningDiacriticalMarks::CombiningMacron),
            COMBINING_OVERLINE => Ok(CombiningDiacriticalMarks::CombiningOverline),
            COMBINING_BREVE => Ok(CombiningDiacriticalMarks::CombiningBreve),
            COMBINING_DOT_ABOVE => Ok(CombiningDiacriticalMarks::CombiningDotAbove),
            COMBINING_DIAERESIS => Ok(CombiningDiacriticalMarks::CombiningDiaeresis),
            COMBINING_HOOK_ABOVE => Ok(CombiningDiacriticalMarks::CombiningHookAbove),
            COMBINING_RING_ABOVE => Ok(CombiningDiacriticalMarks::CombiningRingAbove),
            COMBINING_DOUBLE_ACUTE_ACCENT => Ok(CombiningDiacriticalMarks::CombiningDoubleAcuteAccent),
            COMBINING_CARON => Ok(CombiningDiacriticalMarks::CombiningCaron),
            COMBINING_VERTICAL_LINE_ABOVE => Ok(CombiningDiacriticalMarks::CombiningVerticalLineAbove),
            COMBINING_DOUBLE_VERTICAL_LINE_ABOVE => Ok(CombiningDiacriticalMarks::CombiningDoubleVerticalLineAbove),
            COMBINING_DOUBLE_GRAVE_ACCENT => Ok(CombiningDiacriticalMarks::CombiningDoubleGraveAccent),
            COMBINING_CANDRABINDU => Ok(CombiningDiacriticalMarks::CombiningCandrabindu),
            COMBINING_INVERTED_BREVE => Ok(CombiningDiacriticalMarks::CombiningInvertedBreve),
            COMBINING_TURNED_COMMA_ABOVE => Ok(CombiningDiacriticalMarks::CombiningTurnedCommaAbove),
            COMBINING_COMMA_ABOVE => Ok(CombiningDiacriticalMarks::CombiningCommaAbove),
            COMBINING_REVERSED_COMMA_ABOVE => Ok(CombiningDiacriticalMarks::CombiningReversedCommaAbove),
            COMBINING_COMMA_ABOVE_RIGHT => Ok(CombiningDiacriticalMarks::CombiningCommaAboveRight),
            COMBINING_GRAVE_ACCENT_BELOW => Ok(CombiningDiacriticalMarks::CombiningGraveAccentBelow),
            COMBINING_ACUTE_ACCENT_BELOW => Ok(CombiningDiacriticalMarks::CombiningAcuteAccentBelow),
            COMBINING_LEFT_TACK_BELOW => Ok(CombiningDiacriticalMarks::CombiningLeftTackBelow),
            COMBINING_RIGHT_TACK_BELOW => Ok(CombiningDiacriticalMarks::CombiningRightTackBelow),
            COMBINING_LEFT_ANGLE_ABOVE => Ok(CombiningDiacriticalMarks::CombiningLeftAngleAbove),
            COMBINING_HORN => Ok(CombiningDiacriticalMarks::CombiningHorn),
            COMBINING_LEFT_HALF_RING_BELOW => Ok(CombiningDiacriticalMarks::CombiningLeftHalfRingBelow),
            COMBINING_UP_TACK_BELOW => Ok(CombiningDiacriticalMarks::CombiningUpTackBelow),
            COMBINING_DOWN_TACK_BELOW => Ok(CombiningDiacriticalMarks::CombiningDownTackBelow),
            COMBINING_PLUS_SIGN_BELOW => Ok(CombiningDiacriticalMarks::CombiningPlusSignBelow),
            COMBINING_MINUS_SIGN_BELOW => Ok(CombiningDiacriticalMarks::CombiningMinusSignBelow),
            COMBINING_PALATALIZED_HOOK_BELOW => Ok(CombiningDiacriticalMarks::CombiningPalatalizedHookBelow),
            COMBINING_RETROFLEX_HOOK_BELOW => Ok(CombiningDiacriticalMarks::CombiningRetroflexHookBelow),
            COMBINING_DOT_BELOW => Ok(CombiningDiacriticalMarks::CombiningDotBelow),
            COMBINING_DIAERESIS_BELOW => Ok(CombiningDiacriticalMarks::CombiningDiaeresisBelow),
            COMBINING_RING_BELOW => Ok(CombiningDiacriticalMarks::CombiningRingBelow),
            COMBINING_COMMA_BELOW => Ok(CombiningDiacriticalMarks::CombiningCommaBelow),
            COMBINING_CEDILLA => Ok(CombiningDiacriticalMarks::CombiningCedilla),
            COMBINING_OGONEK => Ok(CombiningDiacriticalMarks::CombiningOgonek),
            COMBINING_VERTICAL_LINE_BELOW => Ok(CombiningDiacriticalMarks::CombiningVerticalLineBelow),
            COMBINING_BRIDGE_BELOW => Ok(CombiningDiacriticalMarks::CombiningBridgeBelow),
            COMBINING_INVERTED_DOUBLE_ARCH_BELOW => Ok(CombiningDiacriticalMarks::CombiningInvertedDoubleArchBelow),
            COMBINING_CARON_BELOW => Ok(CombiningDiacriticalMarks::CombiningCaronBelow),
            COMBINING_CIRCUMFLEX_ACCENT_BELOW => Ok(CombiningDiacriticalMarks::CombiningCircumflexAccentBelow),
            COMBINING_BREVE_BELOW => Ok(CombiningDiacriticalMarks::CombiningBreveBelow),
            COMBINING_INVERTED_BREVE_BELOW => Ok(CombiningDiacriticalMarks::CombiningInvertedBreveBelow),
            COMBINING_TILDE_BELOW => Ok(CombiningDiacriticalMarks::CombiningTildeBelow),
            COMBINING_MACRON_BELOW => Ok(CombiningDiacriticalMarks::CombiningMacronBelow),
            COMBINING_LOW_LINE => Ok(CombiningDiacriticalMarks::CombiningLowLine),
            COMBINING_DOUBLE_LOW_LINE => Ok(CombiningDiacriticalMarks::CombiningDoubleLowLine),
            COMBINING_TILDE_OVERLAY => Ok(CombiningDiacriticalMarks::CombiningTildeOverlay),
            COMBINING_SHORT_STROKE_OVERLAY => Ok(CombiningDiacriticalMarks::CombiningShortStrokeOverlay),
            COMBINING_LONG_STROKE_OVERLAY => Ok(CombiningDiacriticalMarks::CombiningLongStrokeOverlay),
            COMBINING_SHORT_SOLIDUS_OVERLAY => Ok(CombiningDiacriticalMarks::CombiningShortSolidusOverlay),
            COMBINING_LONG_SOLIDUS_OVERLAY => Ok(CombiningDiacriticalMarks::CombiningLongSolidusOverlay),
            COMBINING_RIGHT_HALF_RING_BELOW => Ok(CombiningDiacriticalMarks::CombiningRightHalfRingBelow),
            COMBINING_INVERTED_BRIDGE_BELOW => Ok(CombiningDiacriticalMarks::CombiningInvertedBridgeBelow),
            COMBINING_SQUARE_BELOW => Ok(CombiningDiacriticalMarks::CombiningSquareBelow),
            COMBINING_SEAGULL_BELOW => Ok(CombiningDiacriticalMarks::CombiningSeagullBelow),
            COMBINING_X_ABOVE => Ok(CombiningDiacriticalMarks::CombiningXAbove),
            COMBINING_VERTICAL_TILDE => Ok(CombiningDiacriticalMarks::CombiningVerticalTilde),
            COMBINING_DOUBLE_OVERLINE => Ok(CombiningDiacriticalMarks::CombiningDoubleOverline),
            COMBINING_GRAVE_TONE_MARK => Ok(CombiningDiacriticalMarks::CombiningGraveToneMark),
            COMBINING_ACUTE_TONE_MARK => Ok(CombiningDiacriticalMarks::CombiningAcuteToneMark),
            COMBINING_GREEK_PERISPOMENI => Ok(CombiningDiacriticalMarks::CombiningGreekPerispomeni),
            COMBINING_GREEK_KORONIS => Ok(CombiningDiacriticalMarks::CombiningGreekKoronis),
            COMBINING_GREEK_DIALYTIKA_TONOS => Ok(CombiningDiacriticalMarks::CombiningGreekDialytikaTonos),
            COMBINING_GREEK_YPOGEGRAMMENI => Ok(CombiningDiacriticalMarks::CombiningGreekYpogegrammeni),
            COMBINING_BRIDGE_ABOVE => Ok(CombiningDiacriticalMarks::CombiningBridgeAbove),
            COMBINING_EQUALS_SIGN_BELOW => Ok(CombiningDiacriticalMarks::CombiningEqualsSignBelow),
            COMBINING_DOUBLE_VERTICAL_LINE_BELOW => Ok(CombiningDiacriticalMarks::CombiningDoubleVerticalLineBelow),
            COMBINING_LEFT_ANGLE_BELOW => Ok(CombiningDiacriticalMarks::CombiningLeftAngleBelow),
            COMBINING_NOT_TILDE_ABOVE => Ok(CombiningDiacriticalMarks::CombiningNotTildeAbove),
            COMBINING_HOMOTHETIC_ABOVE => Ok(CombiningDiacriticalMarks::CombiningHomotheticAbove),
            COMBINING_ALMOST_EQUAL_TO_ABOVE => Ok(CombiningDiacriticalMarks::CombiningAlmostEqualToAbove),
            COMBINING_LEFT_RIGHT_ARROW_BELOW => Ok(CombiningDiacriticalMarks::CombiningLeftRightArrowBelow),
            COMBINING_UPWARDS_ARROW_BELOW => Ok(CombiningDiacriticalMarks::CombiningUpwardsArrowBelow),
            COMBINING_GRAPHEME_JOINER => Ok(CombiningDiacriticalMarks::CombiningGraphemeJoiner),
            COMBINING_RIGHT_ARROWHEAD_ABOVE => Ok(CombiningDiacriticalMarks::CombiningRightArrowheadAbove),
            COMBINING_LEFT_HALF_RING_ABOVE => Ok(CombiningDiacriticalMarks::CombiningLeftHalfRingAbove),
            COMBINING_FERMATA => Ok(CombiningDiacriticalMarks::CombiningFermata),
            COMBINING_X_BELOW => Ok(CombiningDiacriticalMarks::CombiningXBelow),
            COMBINING_LEFT_ARROWHEAD_BELOW => Ok(CombiningDiacriticalMarks::CombiningLeftArrowheadBelow),
            COMBINING_RIGHT_ARROWHEAD_BELOW => Ok(CombiningDiacriticalMarks::CombiningRightArrowheadBelow),
            COMBINING_RIGHT_ARROWHEAD_AND_UP_ARROWHEAD_BELOW => Ok(CombiningDiacriticalMarks::CombiningRightArrowheadAndUpArrowheadBelow),
            COMBINING_RIGHT_HALF_RING_ABOVE => Ok(CombiningDiacriticalMarks::CombiningRightHalfRingAbove),
            COMBINING_DOT_ABOVE_RIGHT => Ok(CombiningDiacriticalMarks::CombiningDotAboveRight),
            COMBINING_ASTERISK_BELOW => Ok(CombiningDiacriticalMarks::CombiningAsteriskBelow),
            COMBINING_DOUBLE_RING_BELOW => Ok(CombiningDiacriticalMarks::CombiningDoubleRingBelow),
            COMBINING_ZIGZAG_ABOVE => Ok(CombiningDiacriticalMarks::CombiningZigzagAbove),
            COMBINING_DOUBLE_BREVE_BELOW => Ok(CombiningDiacriticalMarks::CombiningDoubleBreveBelow),
            COMBINING_DOUBLE_BREVE => Ok(CombiningDiacriticalMarks::CombiningDoubleBreve),
            COMBINING_DOUBLE_MACRON => Ok(CombiningDiacriticalMarks::CombiningDoubleMacron),
            COMBINING_DOUBLE_MACRON_BELOW => Ok(CombiningDiacriticalMarks::CombiningDoubleMacronBelow),
            COMBINING_DOUBLE_TILDE => Ok(CombiningDiacriticalMarks::CombiningDoubleTilde),
            COMBINING_DOUBLE_INVERTED_BREVE => Ok(CombiningDiacriticalMarks::CombiningDoubleInvertedBreve),
            COMBINING_DOUBLE_RIGHTWARDS_ARROW_BELOW => Ok(CombiningDiacriticalMarks::CombiningDoubleRightwardsArrowBelow),
            COMBINING_LATIN_SMALL_LETTER_A => Ok(CombiningDiacriticalMarks::CombiningLatinSmallLetterA),
            COMBINING_LATIN_SMALL_LETTER_E => Ok(CombiningDiacriticalMarks::CombiningLatinSmallLetterE),
            COMBINING_LATIN_SMALL_LETTER_I => Ok(CombiningDiacriticalMarks::CombiningLatinSmallLetterI),
            COMBINING_LATIN_SMALL_LETTER_O => Ok(CombiningDiacriticalMarks::CombiningLatinSmallLetterO),
            COMBINING_LATIN_SMALL_LETTER_U => Ok(CombiningDiacriticalMarks::CombiningLatinSmallLetterU),
            COMBINING_LATIN_SMALL_LETTER_C => Ok(CombiningDiacriticalMarks::CombiningLatinSmallLetterC),
            COMBINING_LATIN_SMALL_LETTER_D => Ok(CombiningDiacriticalMarks::CombiningLatinSmallLetterD),
            COMBINING_LATIN_SMALL_LETTER_H => Ok(CombiningDiacriticalMarks::CombiningLatinSmallLetterH),
            COMBINING_LATIN_SMALL_LETTER_M => Ok(CombiningDiacriticalMarks::CombiningLatinSmallLetterM),
            COMBINING_LATIN_SMALL_LETTER_R => Ok(CombiningDiacriticalMarks::CombiningLatinSmallLetterR),
            COMBINING_LATIN_SMALL_LETTER_T => Ok(CombiningDiacriticalMarks::CombiningLatinSmallLetterT),
            COMBINING_LATIN_SMALL_LETTER_V => Ok(CombiningDiacriticalMarks::CombiningLatinSmallLetterV),
            _ => Err(()),
        }
    }
}

impl Into<u32> for CombiningDiacriticalMarks {
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

impl std::convert::TryFrom<u32> for CombiningDiacriticalMarks {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for CombiningDiacriticalMarks {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl CombiningDiacriticalMarks {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        CombiningDiacriticalMarks::CombiningGraveAccent
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            CombiningDiacriticalMarks::CombiningGraveAccent => "combining grave accent",
            CombiningDiacriticalMarks::CombiningAcuteAccent => "combining acute accent",
            CombiningDiacriticalMarks::CombiningCircumflexAccent => "combining circumflex accent",
            CombiningDiacriticalMarks::CombiningTilde => "combining tilde",
            CombiningDiacriticalMarks::CombiningMacron => "combining macron",
            CombiningDiacriticalMarks::CombiningOverline => "combining overline",
            CombiningDiacriticalMarks::CombiningBreve => "combining breve",
            CombiningDiacriticalMarks::CombiningDotAbove => "combining dot above",
            CombiningDiacriticalMarks::CombiningDiaeresis => "combining diaeresis",
            CombiningDiacriticalMarks::CombiningHookAbove => "combining hook above",
            CombiningDiacriticalMarks::CombiningRingAbove => "combining ring above",
            CombiningDiacriticalMarks::CombiningDoubleAcuteAccent => "combining double acute accent",
            CombiningDiacriticalMarks::CombiningCaron => "combining caron",
            CombiningDiacriticalMarks::CombiningVerticalLineAbove => "combining vertical line above",
            CombiningDiacriticalMarks::CombiningDoubleVerticalLineAbove => "combining double vertical line above",
            CombiningDiacriticalMarks::CombiningDoubleGraveAccent => "combining double grave accent",
            CombiningDiacriticalMarks::CombiningCandrabindu => "combining candrabindu",
            CombiningDiacriticalMarks::CombiningInvertedBreve => "combining inverted breve",
            CombiningDiacriticalMarks::CombiningTurnedCommaAbove => "combining turned comma above",
            CombiningDiacriticalMarks::CombiningCommaAbove => "combining comma above",
            CombiningDiacriticalMarks::CombiningReversedCommaAbove => "combining reversed comma above",
            CombiningDiacriticalMarks::CombiningCommaAboveRight => "combining comma above right",
            CombiningDiacriticalMarks::CombiningGraveAccentBelow => "combining grave accent below",
            CombiningDiacriticalMarks::CombiningAcuteAccentBelow => "combining acute accent below",
            CombiningDiacriticalMarks::CombiningLeftTackBelow => "combining left tack below",
            CombiningDiacriticalMarks::CombiningRightTackBelow => "combining right tack below",
            CombiningDiacriticalMarks::CombiningLeftAngleAbove => "combining left angle above",
            CombiningDiacriticalMarks::CombiningHorn => "combining horn",
            CombiningDiacriticalMarks::CombiningLeftHalfRingBelow => "combining left half ring below",
            CombiningDiacriticalMarks::CombiningUpTackBelow => "combining up tack below",
            CombiningDiacriticalMarks::CombiningDownTackBelow => "combining down tack below",
            CombiningDiacriticalMarks::CombiningPlusSignBelow => "combining plus sign below",
            CombiningDiacriticalMarks::CombiningMinusSignBelow => "combining minus sign below",
            CombiningDiacriticalMarks::CombiningPalatalizedHookBelow => "combining palatalized hook below",
            CombiningDiacriticalMarks::CombiningRetroflexHookBelow => "combining retroflex hook below",
            CombiningDiacriticalMarks::CombiningDotBelow => "combining dot below",
            CombiningDiacriticalMarks::CombiningDiaeresisBelow => "combining diaeresis below",
            CombiningDiacriticalMarks::CombiningRingBelow => "combining ring below",
            CombiningDiacriticalMarks::CombiningCommaBelow => "combining comma below",
            CombiningDiacriticalMarks::CombiningCedilla => "combining cedilla",
            CombiningDiacriticalMarks::CombiningOgonek => "combining ogonek",
            CombiningDiacriticalMarks::CombiningVerticalLineBelow => "combining vertical line below",
            CombiningDiacriticalMarks::CombiningBridgeBelow => "combining bridge below",
            CombiningDiacriticalMarks::CombiningInvertedDoubleArchBelow => "combining inverted double arch below",
            CombiningDiacriticalMarks::CombiningCaronBelow => "combining caron below",
            CombiningDiacriticalMarks::CombiningCircumflexAccentBelow => "combining circumflex accent below",
            CombiningDiacriticalMarks::CombiningBreveBelow => "combining breve below",
            CombiningDiacriticalMarks::CombiningInvertedBreveBelow => "combining inverted breve below",
            CombiningDiacriticalMarks::CombiningTildeBelow => "combining tilde below",
            CombiningDiacriticalMarks::CombiningMacronBelow => "combining macron below",
            CombiningDiacriticalMarks::CombiningLowLine => "combining low line",
            CombiningDiacriticalMarks::CombiningDoubleLowLine => "combining double low line",
            CombiningDiacriticalMarks::CombiningTildeOverlay => "combining tilde overlay",
            CombiningDiacriticalMarks::CombiningShortStrokeOverlay => "combining short stroke overlay",
            CombiningDiacriticalMarks::CombiningLongStrokeOverlay => "combining long stroke overlay",
            CombiningDiacriticalMarks::CombiningShortSolidusOverlay => "combining short solidus overlay",
            CombiningDiacriticalMarks::CombiningLongSolidusOverlay => "combining long solidus overlay",
            CombiningDiacriticalMarks::CombiningRightHalfRingBelow => "combining right half ring below",
            CombiningDiacriticalMarks::CombiningInvertedBridgeBelow => "combining inverted bridge below",
            CombiningDiacriticalMarks::CombiningSquareBelow => "combining square below",
            CombiningDiacriticalMarks::CombiningSeagullBelow => "combining seagull below",
            CombiningDiacriticalMarks::CombiningXAbove => "combining x above",
            CombiningDiacriticalMarks::CombiningVerticalTilde => "combining vertical tilde",
            CombiningDiacriticalMarks::CombiningDoubleOverline => "combining double overline",
            CombiningDiacriticalMarks::CombiningGraveToneMark => "combining grave tone mark",
            CombiningDiacriticalMarks::CombiningAcuteToneMark => "combining acute tone mark",
            CombiningDiacriticalMarks::CombiningGreekPerispomeni => "combining greek perispomeni",
            CombiningDiacriticalMarks::CombiningGreekKoronis => "combining greek koronis",
            CombiningDiacriticalMarks::CombiningGreekDialytikaTonos => "combining greek dialytika tonos",
            CombiningDiacriticalMarks::CombiningGreekYpogegrammeni => "combining greek ypogegrammeni",
            CombiningDiacriticalMarks::CombiningBridgeAbove => "combining bridge above",
            CombiningDiacriticalMarks::CombiningEqualsSignBelow => "combining equals sign below",
            CombiningDiacriticalMarks::CombiningDoubleVerticalLineBelow => "combining double vertical line below",
            CombiningDiacriticalMarks::CombiningLeftAngleBelow => "combining left angle below",
            CombiningDiacriticalMarks::CombiningNotTildeAbove => "combining not tilde above",
            CombiningDiacriticalMarks::CombiningHomotheticAbove => "combining homothetic above",
            CombiningDiacriticalMarks::CombiningAlmostEqualToAbove => "combining almost equal to above",
            CombiningDiacriticalMarks::CombiningLeftRightArrowBelow => "combining left right arrow below",
            CombiningDiacriticalMarks::CombiningUpwardsArrowBelow => "combining upwards arrow below",
            CombiningDiacriticalMarks::CombiningGraphemeJoiner => "combining grapheme joiner",
            CombiningDiacriticalMarks::CombiningRightArrowheadAbove => "combining right arrowhead above",
            CombiningDiacriticalMarks::CombiningLeftHalfRingAbove => "combining left half ring above",
            CombiningDiacriticalMarks::CombiningFermata => "combining fermata",
            CombiningDiacriticalMarks::CombiningXBelow => "combining x below",
            CombiningDiacriticalMarks::CombiningLeftArrowheadBelow => "combining left arrowhead below",
            CombiningDiacriticalMarks::CombiningRightArrowheadBelow => "combining right arrowhead below",
            CombiningDiacriticalMarks::CombiningRightArrowheadAndUpArrowheadBelow => "combining right arrowhead and up arrowhead below",
            CombiningDiacriticalMarks::CombiningRightHalfRingAbove => "combining right half ring above",
            CombiningDiacriticalMarks::CombiningDotAboveRight => "combining dot above right",
            CombiningDiacriticalMarks::CombiningAsteriskBelow => "combining asterisk below",
            CombiningDiacriticalMarks::CombiningDoubleRingBelow => "combining double ring below",
            CombiningDiacriticalMarks::CombiningZigzagAbove => "combining zigzag above",
            CombiningDiacriticalMarks::CombiningDoubleBreveBelow => "combining double breve below",
            CombiningDiacriticalMarks::CombiningDoubleBreve => "combining double breve",
            CombiningDiacriticalMarks::CombiningDoubleMacron => "combining double macron",
            CombiningDiacriticalMarks::CombiningDoubleMacronBelow => "combining double macron below",
            CombiningDiacriticalMarks::CombiningDoubleTilde => "combining double tilde",
            CombiningDiacriticalMarks::CombiningDoubleInvertedBreve => "combining double inverted breve",
            CombiningDiacriticalMarks::CombiningDoubleRightwardsArrowBelow => "combining double rightwards arrow below",
            CombiningDiacriticalMarks::CombiningLatinSmallLetterA => "combining latin small letter a",
            CombiningDiacriticalMarks::CombiningLatinSmallLetterE => "combining latin small letter e",
            CombiningDiacriticalMarks::CombiningLatinSmallLetterI => "combining latin small letter i",
            CombiningDiacriticalMarks::CombiningLatinSmallLetterO => "combining latin small letter o",
            CombiningDiacriticalMarks::CombiningLatinSmallLetterU => "combining latin small letter u",
            CombiningDiacriticalMarks::CombiningLatinSmallLetterC => "combining latin small letter c",
            CombiningDiacriticalMarks::CombiningLatinSmallLetterD => "combining latin small letter d",
            CombiningDiacriticalMarks::CombiningLatinSmallLetterH => "combining latin small letter h",
            CombiningDiacriticalMarks::CombiningLatinSmallLetterM => "combining latin small letter m",
            CombiningDiacriticalMarks::CombiningLatinSmallLetterR => "combining latin small letter r",
            CombiningDiacriticalMarks::CombiningLatinSmallLetterT => "combining latin small letter t",
            CombiningDiacriticalMarks::CombiningLatinSmallLetterV => "combining latin small letter v",
        }
    }
}
