
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
        match self {
            CombiningDiacriticalMarks::CombiningGraveAccent => '̀',
            CombiningDiacriticalMarks::CombiningAcuteAccent => '́',
            CombiningDiacriticalMarks::CombiningCircumflexAccent => '̂',
            CombiningDiacriticalMarks::CombiningTilde => '̃',
            CombiningDiacriticalMarks::CombiningMacron => '̄',
            CombiningDiacriticalMarks::CombiningOverline => '̅',
            CombiningDiacriticalMarks::CombiningBreve => '̆',
            CombiningDiacriticalMarks::CombiningDotAbove => '̇',
            CombiningDiacriticalMarks::CombiningDiaeresis => '̈',
            CombiningDiacriticalMarks::CombiningHookAbove => '̉',
            CombiningDiacriticalMarks::CombiningRingAbove => '̊',
            CombiningDiacriticalMarks::CombiningDoubleAcuteAccent => '̋',
            CombiningDiacriticalMarks::CombiningCaron => '̌',
            CombiningDiacriticalMarks::CombiningVerticalLineAbove => '̍',
            CombiningDiacriticalMarks::CombiningDoubleVerticalLineAbove => '̎',
            CombiningDiacriticalMarks::CombiningDoubleGraveAccent => '̏',
            CombiningDiacriticalMarks::CombiningCandrabindu => '̐',
            CombiningDiacriticalMarks::CombiningInvertedBreve => '̑',
            CombiningDiacriticalMarks::CombiningTurnedCommaAbove => '̒',
            CombiningDiacriticalMarks::CombiningCommaAbove => '̓',
            CombiningDiacriticalMarks::CombiningReversedCommaAbove => '̔',
            CombiningDiacriticalMarks::CombiningCommaAboveRight => '̕',
            CombiningDiacriticalMarks::CombiningGraveAccentBelow => '̖',
            CombiningDiacriticalMarks::CombiningAcuteAccentBelow => '̗',
            CombiningDiacriticalMarks::CombiningLeftTackBelow => '̘',
            CombiningDiacriticalMarks::CombiningRightTackBelow => '̙',
            CombiningDiacriticalMarks::CombiningLeftAngleAbove => '̚',
            CombiningDiacriticalMarks::CombiningHorn => '̛',
            CombiningDiacriticalMarks::CombiningLeftHalfRingBelow => '̜',
            CombiningDiacriticalMarks::CombiningUpTackBelow => '̝',
            CombiningDiacriticalMarks::CombiningDownTackBelow => '̞',
            CombiningDiacriticalMarks::CombiningPlusSignBelow => '̟',
            CombiningDiacriticalMarks::CombiningMinusSignBelow => '̠',
            CombiningDiacriticalMarks::CombiningPalatalizedHookBelow => '̡',
            CombiningDiacriticalMarks::CombiningRetroflexHookBelow => '̢',
            CombiningDiacriticalMarks::CombiningDotBelow => '̣',
            CombiningDiacriticalMarks::CombiningDiaeresisBelow => '̤',
            CombiningDiacriticalMarks::CombiningRingBelow => '̥',
            CombiningDiacriticalMarks::CombiningCommaBelow => '̦',
            CombiningDiacriticalMarks::CombiningCedilla => '̧',
            CombiningDiacriticalMarks::CombiningOgonek => '̨',
            CombiningDiacriticalMarks::CombiningVerticalLineBelow => '̩',
            CombiningDiacriticalMarks::CombiningBridgeBelow => '̪',
            CombiningDiacriticalMarks::CombiningInvertedDoubleArchBelow => '̫',
            CombiningDiacriticalMarks::CombiningCaronBelow => '̬',
            CombiningDiacriticalMarks::CombiningCircumflexAccentBelow => '̭',
            CombiningDiacriticalMarks::CombiningBreveBelow => '̮',
            CombiningDiacriticalMarks::CombiningInvertedBreveBelow => '̯',
            CombiningDiacriticalMarks::CombiningTildeBelow => '̰',
            CombiningDiacriticalMarks::CombiningMacronBelow => '̱',
            CombiningDiacriticalMarks::CombiningLowLine => '̲',
            CombiningDiacriticalMarks::CombiningDoubleLowLine => '̳',
            CombiningDiacriticalMarks::CombiningTildeOverlay => '̴',
            CombiningDiacriticalMarks::CombiningShortStrokeOverlay => '̵',
            CombiningDiacriticalMarks::CombiningLongStrokeOverlay => '̶',
            CombiningDiacriticalMarks::CombiningShortSolidusOverlay => '̷',
            CombiningDiacriticalMarks::CombiningLongSolidusOverlay => '̸',
            CombiningDiacriticalMarks::CombiningRightHalfRingBelow => '̹',
            CombiningDiacriticalMarks::CombiningInvertedBridgeBelow => '̺',
            CombiningDiacriticalMarks::CombiningSquareBelow => '̻',
            CombiningDiacriticalMarks::CombiningSeagullBelow => '̼',
            CombiningDiacriticalMarks::CombiningXAbove => '̽',
            CombiningDiacriticalMarks::CombiningVerticalTilde => '̾',
            CombiningDiacriticalMarks::CombiningDoubleOverline => '̿',
            CombiningDiacriticalMarks::CombiningGraveToneMark => '̀',
            CombiningDiacriticalMarks::CombiningAcuteToneMark => '́',
            CombiningDiacriticalMarks::CombiningGreekPerispomeni => '͂',
            CombiningDiacriticalMarks::CombiningGreekKoronis => '̓',
            CombiningDiacriticalMarks::CombiningGreekDialytikaTonos => '̈́',
            CombiningDiacriticalMarks::CombiningGreekYpogegrammeni => 'ͅ',
            CombiningDiacriticalMarks::CombiningBridgeAbove => '͆',
            CombiningDiacriticalMarks::CombiningEqualsSignBelow => '͇',
            CombiningDiacriticalMarks::CombiningDoubleVerticalLineBelow => '͈',
            CombiningDiacriticalMarks::CombiningLeftAngleBelow => '͉',
            CombiningDiacriticalMarks::CombiningNotTildeAbove => '͊',
            CombiningDiacriticalMarks::CombiningHomotheticAbove => '͋',
            CombiningDiacriticalMarks::CombiningAlmostEqualToAbove => '͌',
            CombiningDiacriticalMarks::CombiningLeftRightArrowBelow => '͍',
            CombiningDiacriticalMarks::CombiningUpwardsArrowBelow => '͎',
            CombiningDiacriticalMarks::CombiningGraphemeJoiner => '͏',
            CombiningDiacriticalMarks::CombiningRightArrowheadAbove => '͐',
            CombiningDiacriticalMarks::CombiningLeftHalfRingAbove => '͑',
            CombiningDiacriticalMarks::CombiningFermata => '͒',
            CombiningDiacriticalMarks::CombiningXBelow => '͓',
            CombiningDiacriticalMarks::CombiningLeftArrowheadBelow => '͔',
            CombiningDiacriticalMarks::CombiningRightArrowheadBelow => '͕',
            CombiningDiacriticalMarks::CombiningRightArrowheadAndUpArrowheadBelow => '͖',
            CombiningDiacriticalMarks::CombiningRightHalfRingAbove => '͗',
            CombiningDiacriticalMarks::CombiningDotAboveRight => '͘',
            CombiningDiacriticalMarks::CombiningAsteriskBelow => '͙',
            CombiningDiacriticalMarks::CombiningDoubleRingBelow => '͚',
            CombiningDiacriticalMarks::CombiningZigzagAbove => '͛',
            CombiningDiacriticalMarks::CombiningDoubleBreveBelow => '͜',
            CombiningDiacriticalMarks::CombiningDoubleBreve => '͝',
            CombiningDiacriticalMarks::CombiningDoubleMacron => '͞',
            CombiningDiacriticalMarks::CombiningDoubleMacronBelow => '͟',
            CombiningDiacriticalMarks::CombiningDoubleTilde => '͠',
            CombiningDiacriticalMarks::CombiningDoubleInvertedBreve => '͡',
            CombiningDiacriticalMarks::CombiningDoubleRightwardsArrowBelow => '͢',
            CombiningDiacriticalMarks::CombiningLatinSmallLetterA => 'ͣ',
            CombiningDiacriticalMarks::CombiningLatinSmallLetterE => 'ͤ',
            CombiningDiacriticalMarks::CombiningLatinSmallLetterI => 'ͥ',
            CombiningDiacriticalMarks::CombiningLatinSmallLetterO => 'ͦ',
            CombiningDiacriticalMarks::CombiningLatinSmallLetterU => 'ͧ',
            CombiningDiacriticalMarks::CombiningLatinSmallLetterC => 'ͨ',
            CombiningDiacriticalMarks::CombiningLatinSmallLetterD => 'ͩ',
            CombiningDiacriticalMarks::CombiningLatinSmallLetterH => 'ͪ',
            CombiningDiacriticalMarks::CombiningLatinSmallLetterM => 'ͫ',
            CombiningDiacriticalMarks::CombiningLatinSmallLetterR => 'ͬ',
            CombiningDiacriticalMarks::CombiningLatinSmallLetterT => 'ͭ',
            CombiningDiacriticalMarks::CombiningLatinSmallLetterV => 'ͮ',
        }
    }
}

impl std::convert::TryFrom<char> for CombiningDiacriticalMarks {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '̀' => Ok(CombiningDiacriticalMarks::CombiningGraveAccent),
            '́' => Ok(CombiningDiacriticalMarks::CombiningAcuteAccent),
            '̂' => Ok(CombiningDiacriticalMarks::CombiningCircumflexAccent),
            '̃' => Ok(CombiningDiacriticalMarks::CombiningTilde),
            '̄' => Ok(CombiningDiacriticalMarks::CombiningMacron),
            '̅' => Ok(CombiningDiacriticalMarks::CombiningOverline),
            '̆' => Ok(CombiningDiacriticalMarks::CombiningBreve),
            '̇' => Ok(CombiningDiacriticalMarks::CombiningDotAbove),
            '̈' => Ok(CombiningDiacriticalMarks::CombiningDiaeresis),
            '̉' => Ok(CombiningDiacriticalMarks::CombiningHookAbove),
            '̊' => Ok(CombiningDiacriticalMarks::CombiningRingAbove),
            '̋' => Ok(CombiningDiacriticalMarks::CombiningDoubleAcuteAccent),
            '̌' => Ok(CombiningDiacriticalMarks::CombiningCaron),
            '̍' => Ok(CombiningDiacriticalMarks::CombiningVerticalLineAbove),
            '̎' => Ok(CombiningDiacriticalMarks::CombiningDoubleVerticalLineAbove),
            '̏' => Ok(CombiningDiacriticalMarks::CombiningDoubleGraveAccent),
            '̐' => Ok(CombiningDiacriticalMarks::CombiningCandrabindu),
            '̑' => Ok(CombiningDiacriticalMarks::CombiningInvertedBreve),
            '̒' => Ok(CombiningDiacriticalMarks::CombiningTurnedCommaAbove),
            '̓' => Ok(CombiningDiacriticalMarks::CombiningCommaAbove),
            '̔' => Ok(CombiningDiacriticalMarks::CombiningReversedCommaAbove),
            '̕' => Ok(CombiningDiacriticalMarks::CombiningCommaAboveRight),
            '̖' => Ok(CombiningDiacriticalMarks::CombiningGraveAccentBelow),
            '̗' => Ok(CombiningDiacriticalMarks::CombiningAcuteAccentBelow),
            '̘' => Ok(CombiningDiacriticalMarks::CombiningLeftTackBelow),
            '̙' => Ok(CombiningDiacriticalMarks::CombiningRightTackBelow),
            '̚' => Ok(CombiningDiacriticalMarks::CombiningLeftAngleAbove),
            '̛' => Ok(CombiningDiacriticalMarks::CombiningHorn),
            '̜' => Ok(CombiningDiacriticalMarks::CombiningLeftHalfRingBelow),
            '̝' => Ok(CombiningDiacriticalMarks::CombiningUpTackBelow),
            '̞' => Ok(CombiningDiacriticalMarks::CombiningDownTackBelow),
            '̟' => Ok(CombiningDiacriticalMarks::CombiningPlusSignBelow),
            '̠' => Ok(CombiningDiacriticalMarks::CombiningMinusSignBelow),
            '̡' => Ok(CombiningDiacriticalMarks::CombiningPalatalizedHookBelow),
            '̢' => Ok(CombiningDiacriticalMarks::CombiningRetroflexHookBelow),
            '̣' => Ok(CombiningDiacriticalMarks::CombiningDotBelow),
            '̤' => Ok(CombiningDiacriticalMarks::CombiningDiaeresisBelow),
            '̥' => Ok(CombiningDiacriticalMarks::CombiningRingBelow),
            '̦' => Ok(CombiningDiacriticalMarks::CombiningCommaBelow),
            '̧' => Ok(CombiningDiacriticalMarks::CombiningCedilla),
            '̨' => Ok(CombiningDiacriticalMarks::CombiningOgonek),
            '̩' => Ok(CombiningDiacriticalMarks::CombiningVerticalLineBelow),
            '̪' => Ok(CombiningDiacriticalMarks::CombiningBridgeBelow),
            '̫' => Ok(CombiningDiacriticalMarks::CombiningInvertedDoubleArchBelow),
            '̬' => Ok(CombiningDiacriticalMarks::CombiningCaronBelow),
            '̭' => Ok(CombiningDiacriticalMarks::CombiningCircumflexAccentBelow),
            '̮' => Ok(CombiningDiacriticalMarks::CombiningBreveBelow),
            '̯' => Ok(CombiningDiacriticalMarks::CombiningInvertedBreveBelow),
            '̰' => Ok(CombiningDiacriticalMarks::CombiningTildeBelow),
            '̱' => Ok(CombiningDiacriticalMarks::CombiningMacronBelow),
            '̲' => Ok(CombiningDiacriticalMarks::CombiningLowLine),
            '̳' => Ok(CombiningDiacriticalMarks::CombiningDoubleLowLine),
            '̴' => Ok(CombiningDiacriticalMarks::CombiningTildeOverlay),
            '̵' => Ok(CombiningDiacriticalMarks::CombiningShortStrokeOverlay),
            '̶' => Ok(CombiningDiacriticalMarks::CombiningLongStrokeOverlay),
            '̷' => Ok(CombiningDiacriticalMarks::CombiningShortSolidusOverlay),
            '̸' => Ok(CombiningDiacriticalMarks::CombiningLongSolidusOverlay),
            '̹' => Ok(CombiningDiacriticalMarks::CombiningRightHalfRingBelow),
            '̺' => Ok(CombiningDiacriticalMarks::CombiningInvertedBridgeBelow),
            '̻' => Ok(CombiningDiacriticalMarks::CombiningSquareBelow),
            '̼' => Ok(CombiningDiacriticalMarks::CombiningSeagullBelow),
            '̽' => Ok(CombiningDiacriticalMarks::CombiningXAbove),
            '̾' => Ok(CombiningDiacriticalMarks::CombiningVerticalTilde),
            '̿' => Ok(CombiningDiacriticalMarks::CombiningDoubleOverline),
            '̀' => Ok(CombiningDiacriticalMarks::CombiningGraveToneMark),
            '́' => Ok(CombiningDiacriticalMarks::CombiningAcuteToneMark),
            '͂' => Ok(CombiningDiacriticalMarks::CombiningGreekPerispomeni),
            '̓' => Ok(CombiningDiacriticalMarks::CombiningGreekKoronis),
            '̈́' => Ok(CombiningDiacriticalMarks::CombiningGreekDialytikaTonos),
            'ͅ' => Ok(CombiningDiacriticalMarks::CombiningGreekYpogegrammeni),
            '͆' => Ok(CombiningDiacriticalMarks::CombiningBridgeAbove),
            '͇' => Ok(CombiningDiacriticalMarks::CombiningEqualsSignBelow),
            '͈' => Ok(CombiningDiacriticalMarks::CombiningDoubleVerticalLineBelow),
            '͉' => Ok(CombiningDiacriticalMarks::CombiningLeftAngleBelow),
            '͊' => Ok(CombiningDiacriticalMarks::CombiningNotTildeAbove),
            '͋' => Ok(CombiningDiacriticalMarks::CombiningHomotheticAbove),
            '͌' => Ok(CombiningDiacriticalMarks::CombiningAlmostEqualToAbove),
            '͍' => Ok(CombiningDiacriticalMarks::CombiningLeftRightArrowBelow),
            '͎' => Ok(CombiningDiacriticalMarks::CombiningUpwardsArrowBelow),
            '͏' => Ok(CombiningDiacriticalMarks::CombiningGraphemeJoiner),
            '͐' => Ok(CombiningDiacriticalMarks::CombiningRightArrowheadAbove),
            '͑' => Ok(CombiningDiacriticalMarks::CombiningLeftHalfRingAbove),
            '͒' => Ok(CombiningDiacriticalMarks::CombiningFermata),
            '͓' => Ok(CombiningDiacriticalMarks::CombiningXBelow),
            '͔' => Ok(CombiningDiacriticalMarks::CombiningLeftArrowheadBelow),
            '͕' => Ok(CombiningDiacriticalMarks::CombiningRightArrowheadBelow),
            '͖' => Ok(CombiningDiacriticalMarks::CombiningRightArrowheadAndUpArrowheadBelow),
            '͗' => Ok(CombiningDiacriticalMarks::CombiningRightHalfRingAbove),
            '͘' => Ok(CombiningDiacriticalMarks::CombiningDotAboveRight),
            '͙' => Ok(CombiningDiacriticalMarks::CombiningAsteriskBelow),
            '͚' => Ok(CombiningDiacriticalMarks::CombiningDoubleRingBelow),
            '͛' => Ok(CombiningDiacriticalMarks::CombiningZigzagAbove),
            '͜' => Ok(CombiningDiacriticalMarks::CombiningDoubleBreveBelow),
            '͝' => Ok(CombiningDiacriticalMarks::CombiningDoubleBreve),
            '͞' => Ok(CombiningDiacriticalMarks::CombiningDoubleMacron),
            '͟' => Ok(CombiningDiacriticalMarks::CombiningDoubleMacronBelow),
            '͠' => Ok(CombiningDiacriticalMarks::CombiningDoubleTilde),
            '͡' => Ok(CombiningDiacriticalMarks::CombiningDoubleInvertedBreve),
            '͢' => Ok(CombiningDiacriticalMarks::CombiningDoubleRightwardsArrowBelow),
            'ͣ' => Ok(CombiningDiacriticalMarks::CombiningLatinSmallLetterA),
            'ͤ' => Ok(CombiningDiacriticalMarks::CombiningLatinSmallLetterE),
            'ͥ' => Ok(CombiningDiacriticalMarks::CombiningLatinSmallLetterI),
            'ͦ' => Ok(CombiningDiacriticalMarks::CombiningLatinSmallLetterO),
            'ͧ' => Ok(CombiningDiacriticalMarks::CombiningLatinSmallLetterU),
            'ͨ' => Ok(CombiningDiacriticalMarks::CombiningLatinSmallLetterC),
            'ͩ' => Ok(CombiningDiacriticalMarks::CombiningLatinSmallLetterD),
            'ͪ' => Ok(CombiningDiacriticalMarks::CombiningLatinSmallLetterH),
            'ͫ' => Ok(CombiningDiacriticalMarks::CombiningLatinSmallLetterM),
            'ͬ' => Ok(CombiningDiacriticalMarks::CombiningLatinSmallLetterR),
            'ͭ' => Ok(CombiningDiacriticalMarks::CombiningLatinSmallLetterT),
            'ͮ' => Ok(CombiningDiacriticalMarks::CombiningLatinSmallLetterV),
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

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("CombiningDiacriticalMarks{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
