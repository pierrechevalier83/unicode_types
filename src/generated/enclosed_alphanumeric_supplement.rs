
/// An enum to represent all characters in the EnclosedAlphanumericSupplement block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum EnclosedAlphanumericSupplement {
    /// \u{1f100}: '🄀'
    DigitZeroFullStop,
    /// \u{1f101}: '🄁'
    DigitZeroComma,
    /// \u{1f102}: '🄂'
    DigitOneComma,
    /// \u{1f103}: '🄃'
    DigitTwoComma,
    /// \u{1f104}: '🄄'
    DigitThreeComma,
    /// \u{1f105}: '🄅'
    DigitFourComma,
    /// \u{1f106}: '🄆'
    DigitFiveComma,
    /// \u{1f107}: '🄇'
    DigitSixComma,
    /// \u{1f108}: '🄈'
    DigitSevenComma,
    /// \u{1f109}: '🄉'
    DigitEightComma,
    /// \u{1f10a}: '🄊'
    DigitNineComma,
    /// \u{1f10b}: '🄋'
    DingbatCircledSansDashSerifDigitZero,
    /// \u{1f10c}: '🄌'
    DingbatNegativeCircledSansDashSerifDigitZero,
    /// \u{1f110}: '🄐'
    ParenthesizedLatinCapitalLetterA,
    /// \u{1f111}: '🄑'
    ParenthesizedLatinCapitalLetterB,
    /// \u{1f112}: '🄒'
    ParenthesizedLatinCapitalLetterC,
    /// \u{1f113}: '🄓'
    ParenthesizedLatinCapitalLetterD,
    /// \u{1f114}: '🄔'
    ParenthesizedLatinCapitalLetterE,
    /// \u{1f115}: '🄕'
    ParenthesizedLatinCapitalLetterF,
    /// \u{1f116}: '🄖'
    ParenthesizedLatinCapitalLetterG,
    /// \u{1f117}: '🄗'
    ParenthesizedLatinCapitalLetterH,
    /// \u{1f118}: '🄘'
    ParenthesizedLatinCapitalLetterI,
    /// \u{1f119}: '🄙'
    ParenthesizedLatinCapitalLetterJ,
    /// \u{1f11a}: '🄚'
    ParenthesizedLatinCapitalLetterK,
    /// \u{1f11b}: '🄛'
    ParenthesizedLatinCapitalLetterL,
    /// \u{1f11c}: '🄜'
    ParenthesizedLatinCapitalLetterM,
    /// \u{1f11d}: '🄝'
    ParenthesizedLatinCapitalLetterN,
    /// \u{1f11e}: '🄞'
    ParenthesizedLatinCapitalLetterO,
    /// \u{1f11f}: '🄟'
    ParenthesizedLatinCapitalLetterP,
    /// \u{1f120}: '🄠'
    ParenthesizedLatinCapitalLetterQ,
    /// \u{1f121}: '🄡'
    ParenthesizedLatinCapitalLetterR,
    /// \u{1f122}: '🄢'
    ParenthesizedLatinCapitalLetterS,
    /// \u{1f123}: '🄣'
    ParenthesizedLatinCapitalLetterT,
    /// \u{1f124}: '🄤'
    ParenthesizedLatinCapitalLetterU,
    /// \u{1f125}: '🄥'
    ParenthesizedLatinCapitalLetterV,
    /// \u{1f126}: '🄦'
    ParenthesizedLatinCapitalLetterW,
    /// \u{1f127}: '🄧'
    ParenthesizedLatinCapitalLetterX,
    /// \u{1f128}: '🄨'
    ParenthesizedLatinCapitalLetterY,
    /// \u{1f129}: '🄩'
    ParenthesizedLatinCapitalLetterZ,
    /// \u{1f12a}: '🄪'
    TortoiseShellBracketedLatinCapitalLetterS,
    /// \u{1f12b}: '🄫'
    CircledItalicLatinCapitalLetterC,
    /// \u{1f12c}: '🄬'
    CircledItalicLatinCapitalLetterR,
    /// \u{1f12d}: '🄭'
    CircledCd,
    /// \u{1f12e}: '🄮'
    CircledWz,
    /// \u{1f12f}: '🄯'
    CopyleftSymbol,
    /// \u{1f130}: '🄰'
    SquaredLatinCapitalLetterA,
    /// \u{1f131}: '🄱'
    SquaredLatinCapitalLetterB,
    /// \u{1f132}: '🄲'
    SquaredLatinCapitalLetterC,
    /// \u{1f133}: '🄳'
    SquaredLatinCapitalLetterD,
    /// \u{1f134}: '🄴'
    SquaredLatinCapitalLetterE,
    /// \u{1f135}: '🄵'
    SquaredLatinCapitalLetterF,
    /// \u{1f136}: '🄶'
    SquaredLatinCapitalLetterG,
    /// \u{1f137}: '🄷'
    SquaredLatinCapitalLetterH,
    /// \u{1f138}: '🄸'
    SquaredLatinCapitalLetterI,
    /// \u{1f139}: '🄹'
    SquaredLatinCapitalLetterJ,
    /// \u{1f13a}: '🄺'
    SquaredLatinCapitalLetterK,
    /// \u{1f13b}: '🄻'
    SquaredLatinCapitalLetterL,
    /// \u{1f13c}: '🄼'
    SquaredLatinCapitalLetterM,
    /// \u{1f13d}: '🄽'
    SquaredLatinCapitalLetterN,
    /// \u{1f13e}: '🄾'
    SquaredLatinCapitalLetterO,
    /// \u{1f13f}: '🄿'
    SquaredLatinCapitalLetterP,
    /// \u{1f140}: '🅀'
    SquaredLatinCapitalLetterQ,
    /// \u{1f141}: '🅁'
    SquaredLatinCapitalLetterR,
    /// \u{1f142}: '🅂'
    SquaredLatinCapitalLetterS,
    /// \u{1f143}: '🅃'
    SquaredLatinCapitalLetterT,
    /// \u{1f144}: '🅄'
    SquaredLatinCapitalLetterU,
    /// \u{1f145}: '🅅'
    SquaredLatinCapitalLetterV,
    /// \u{1f146}: '🅆'
    SquaredLatinCapitalLetterW,
    /// \u{1f147}: '🅇'
    SquaredLatinCapitalLetterX,
    /// \u{1f148}: '🅈'
    SquaredLatinCapitalLetterY,
    /// \u{1f149}: '🅉'
    SquaredLatinCapitalLetterZ,
    /// \u{1f14a}: '🅊'
    SquaredHv,
    /// \u{1f14b}: '🅋'
    SquaredMv,
    /// \u{1f14c}: '🅌'
    SquaredSd,
    /// \u{1f14d}: '🅍'
    SquaredSs,
    /// \u{1f14e}: '🅎'
    SquaredPpv,
    /// \u{1f14f}: '🅏'
    SquaredWc,
    /// \u{1f150}: '🅐'
    NegativeCircledLatinCapitalLetterA,
    /// \u{1f151}: '🅑'
    NegativeCircledLatinCapitalLetterB,
    /// \u{1f152}: '🅒'
    NegativeCircledLatinCapitalLetterC,
    /// \u{1f153}: '🅓'
    NegativeCircledLatinCapitalLetterD,
    /// \u{1f154}: '🅔'
    NegativeCircledLatinCapitalLetterE,
    /// \u{1f155}: '🅕'
    NegativeCircledLatinCapitalLetterF,
    /// \u{1f156}: '🅖'
    NegativeCircledLatinCapitalLetterG,
    /// \u{1f157}: '🅗'
    NegativeCircledLatinCapitalLetterH,
    /// \u{1f158}: '🅘'
    NegativeCircledLatinCapitalLetterI,
    /// \u{1f159}: '🅙'
    NegativeCircledLatinCapitalLetterJ,
    /// \u{1f15a}: '🅚'
    NegativeCircledLatinCapitalLetterK,
    /// \u{1f15b}: '🅛'
    NegativeCircledLatinCapitalLetterL,
    /// \u{1f15c}: '🅜'
    NegativeCircledLatinCapitalLetterM,
    /// \u{1f15d}: '🅝'
    NegativeCircledLatinCapitalLetterN,
    /// \u{1f15e}: '🅞'
    NegativeCircledLatinCapitalLetterO,
    /// \u{1f15f}: '🅟'
    NegativeCircledLatinCapitalLetterP,
    /// \u{1f160}: '🅠'
    NegativeCircledLatinCapitalLetterQ,
    /// \u{1f161}: '🅡'
    NegativeCircledLatinCapitalLetterR,
    /// \u{1f162}: '🅢'
    NegativeCircledLatinCapitalLetterS,
    /// \u{1f163}: '🅣'
    NegativeCircledLatinCapitalLetterT,
    /// \u{1f164}: '🅤'
    NegativeCircledLatinCapitalLetterU,
    /// \u{1f165}: '🅥'
    NegativeCircledLatinCapitalLetterV,
    /// \u{1f166}: '🅦'
    NegativeCircledLatinCapitalLetterW,
    /// \u{1f167}: '🅧'
    NegativeCircledLatinCapitalLetterX,
    /// \u{1f168}: '🅨'
    NegativeCircledLatinCapitalLetterY,
    /// \u{1f169}: '🅩'
    NegativeCircledLatinCapitalLetterZ,
    /// \u{1f16a}: '🅪'
    RaisedMcSign,
    /// \u{1f16b}: '🅫'
    RaisedMdSign,
    /// \u{1f16c}: '🅬'
    RaisedMrSign,
    /// \u{1f170}: '🅰'
    NegativeSquaredLatinCapitalLetterA,
    /// \u{1f171}: '🅱'
    NegativeSquaredLatinCapitalLetterB,
    /// \u{1f172}: '🅲'
    NegativeSquaredLatinCapitalLetterC,
    /// \u{1f173}: '🅳'
    NegativeSquaredLatinCapitalLetterD,
    /// \u{1f174}: '🅴'
    NegativeSquaredLatinCapitalLetterE,
    /// \u{1f175}: '🅵'
    NegativeSquaredLatinCapitalLetterF,
    /// \u{1f176}: '🅶'
    NegativeSquaredLatinCapitalLetterG,
    /// \u{1f177}: '🅷'
    NegativeSquaredLatinCapitalLetterH,
    /// \u{1f178}: '🅸'
    NegativeSquaredLatinCapitalLetterI,
    /// \u{1f179}: '🅹'
    NegativeSquaredLatinCapitalLetterJ,
    /// \u{1f17a}: '🅺'
    NegativeSquaredLatinCapitalLetterK,
    /// \u{1f17b}: '🅻'
    NegativeSquaredLatinCapitalLetterL,
    /// \u{1f17c}: '🅼'
    NegativeSquaredLatinCapitalLetterM,
    /// \u{1f17d}: '🅽'
    NegativeSquaredLatinCapitalLetterN,
    /// \u{1f17e}: '🅾'
    NegativeSquaredLatinCapitalLetterO,
    /// \u{1f17f}: '🅿'
    NegativeSquaredLatinCapitalLetterP,
    /// \u{1f180}: '🆀'
    NegativeSquaredLatinCapitalLetterQ,
    /// \u{1f181}: '🆁'
    NegativeSquaredLatinCapitalLetterR,
    /// \u{1f182}: '🆂'
    NegativeSquaredLatinCapitalLetterS,
    /// \u{1f183}: '🆃'
    NegativeSquaredLatinCapitalLetterT,
    /// \u{1f184}: '🆄'
    NegativeSquaredLatinCapitalLetterU,
    /// \u{1f185}: '🆅'
    NegativeSquaredLatinCapitalLetterV,
    /// \u{1f186}: '🆆'
    NegativeSquaredLatinCapitalLetterW,
    /// \u{1f187}: '🆇'
    NegativeSquaredLatinCapitalLetterX,
    /// \u{1f188}: '🆈'
    NegativeSquaredLatinCapitalLetterY,
    /// \u{1f189}: '🆉'
    NegativeSquaredLatinCapitalLetterZ,
    /// \u{1f18a}: '🆊'
    CrossedNegativeSquaredLatinCapitalLetterP,
    /// \u{1f18b}: '🆋'
    NegativeSquaredIc,
    /// \u{1f18c}: '🆌'
    NegativeSquaredPa,
    /// \u{1f18d}: '🆍'
    NegativeSquaredSa,
    /// \u{1f18e}: '🆎'
    NegativeSquaredAb,
    /// \u{1f18f}: '🆏'
    NegativeSquaredWc,
    /// \u{1f190}: '🆐'
    SquareDj,
    /// \u{1f191}: '🆑'
    SquaredCl,
    /// \u{1f192}: '🆒'
    SquaredCool,
    /// \u{1f193}: '🆓'
    SquaredFree,
    /// \u{1f194}: '🆔'
    SquaredId,
    /// \u{1f195}: '🆕'
    SquaredNew,
    /// \u{1f196}: '🆖'
    SquaredNg,
    /// \u{1f197}: '🆗'
    SquaredOk,
    /// \u{1f198}: '🆘'
    SquaredSos,
    /// \u{1f199}: '🆙'
    SquaredUpWithExclamationMark,
    /// \u{1f19a}: '🆚'
    SquaredVs,
    /// \u{1f19b}: '🆛'
    SquaredThreeD,
    /// \u{1f19c}: '🆜'
    SquaredSecondScreen,
    /// \u{1f19d}: '🆝'
    SquaredTwoK,
    /// \u{1f19e}: '🆞'
    SquaredFourK,
    /// \u{1f19f}: '🆟'
    SquaredEightK,
    /// \u{1f1a0}: '🆠'
    SquaredFivePointOne,
    /// \u{1f1a1}: '🆡'
    SquaredSevenPointOne,
    /// \u{1f1a2}: '🆢'
    SquaredTwentyDashTwoPointTwo,
    /// \u{1f1a3}: '🆣'
    SquaredSixtyP,
    /// \u{1f1a4}: '🆤'
    SquaredOneHundredTwentyP,
    /// \u{1f1a5}: '🆥'
    SquaredLatinSmallLetterD,
    /// \u{1f1a6}: '🆦'
    SquaredHc,
    /// \u{1f1a7}: '🆧'
    SquaredHdr,
    /// \u{1f1a8}: '🆨'
    SquaredHiDashRes,
    /// \u{1f1a9}: '🆩'
    SquaredLossless,
    /// \u{1f1aa}: '🆪'
    SquaredShv,
    /// \u{1f1ab}: '🆫'
    SquaredUhd,
    /// \u{1f1ac}: '🆬'
    SquaredVod,
    /// \u{1f1e6}: '🇦'
    RegionalIndicatorSymbolLetterA,
    /// \u{1f1e7}: '🇧'
    RegionalIndicatorSymbolLetterB,
    /// \u{1f1e8}: '🇨'
    RegionalIndicatorSymbolLetterC,
    /// \u{1f1e9}: '🇩'
    RegionalIndicatorSymbolLetterD,
    /// \u{1f1ea}: '🇪'
    RegionalIndicatorSymbolLetterE,
    /// \u{1f1eb}: '🇫'
    RegionalIndicatorSymbolLetterF,
    /// \u{1f1ec}: '🇬'
    RegionalIndicatorSymbolLetterG,
    /// \u{1f1ed}: '🇭'
    RegionalIndicatorSymbolLetterH,
    /// \u{1f1ee}: '🇮'
    RegionalIndicatorSymbolLetterI,
    /// \u{1f1ef}: '🇯'
    RegionalIndicatorSymbolLetterJ,
    /// \u{1f1f0}: '🇰'
    RegionalIndicatorSymbolLetterK,
    /// \u{1f1f1}: '🇱'
    RegionalIndicatorSymbolLetterL,
    /// \u{1f1f2}: '🇲'
    RegionalIndicatorSymbolLetterM,
    /// \u{1f1f3}: '🇳'
    RegionalIndicatorSymbolLetterN,
    /// \u{1f1f4}: '🇴'
    RegionalIndicatorSymbolLetterO,
    /// \u{1f1f5}: '🇵'
    RegionalIndicatorSymbolLetterP,
    /// \u{1f1f6}: '🇶'
    RegionalIndicatorSymbolLetterQ,
    /// \u{1f1f7}: '🇷'
    RegionalIndicatorSymbolLetterR,
    /// \u{1f1f8}: '🇸'
    RegionalIndicatorSymbolLetterS,
    /// \u{1f1f9}: '🇹'
    RegionalIndicatorSymbolLetterT,
    /// \u{1f1fa}: '🇺'
    RegionalIndicatorSymbolLetterU,
    /// \u{1f1fb}: '🇻'
    RegionalIndicatorSymbolLetterV,
    /// \u{1f1fc}: '🇼'
    RegionalIndicatorSymbolLetterW,
    /// \u{1f1fd}: '🇽'
    RegionalIndicatorSymbolLetterX,
    /// \u{1f1fe}: '🇾'
    RegionalIndicatorSymbolLetterY,
}

impl Into<char> for EnclosedAlphanumericSupplement {
    fn into(self) -> char {
        match self {
            EnclosedAlphanumericSupplement::DigitZeroFullStop => '🄀',
            EnclosedAlphanumericSupplement::DigitZeroComma => '🄁',
            EnclosedAlphanumericSupplement::DigitOneComma => '🄂',
            EnclosedAlphanumericSupplement::DigitTwoComma => '🄃',
            EnclosedAlphanumericSupplement::DigitThreeComma => '🄄',
            EnclosedAlphanumericSupplement::DigitFourComma => '🄅',
            EnclosedAlphanumericSupplement::DigitFiveComma => '🄆',
            EnclosedAlphanumericSupplement::DigitSixComma => '🄇',
            EnclosedAlphanumericSupplement::DigitSevenComma => '🄈',
            EnclosedAlphanumericSupplement::DigitEightComma => '🄉',
            EnclosedAlphanumericSupplement::DigitNineComma => '🄊',
            EnclosedAlphanumericSupplement::DingbatCircledSansDashSerifDigitZero => '🄋',
            EnclosedAlphanumericSupplement::DingbatNegativeCircledSansDashSerifDigitZero => '🄌',
            EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterA => '🄐',
            EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterB => '🄑',
            EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterC => '🄒',
            EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterD => '🄓',
            EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterE => '🄔',
            EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterF => '🄕',
            EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterG => '🄖',
            EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterH => '🄗',
            EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterI => '🄘',
            EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterJ => '🄙',
            EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterK => '🄚',
            EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterL => '🄛',
            EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterM => '🄜',
            EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterN => '🄝',
            EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterO => '🄞',
            EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterP => '🄟',
            EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterQ => '🄠',
            EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterR => '🄡',
            EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterS => '🄢',
            EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterT => '🄣',
            EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterU => '🄤',
            EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterV => '🄥',
            EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterW => '🄦',
            EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterX => '🄧',
            EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterY => '🄨',
            EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterZ => '🄩',
            EnclosedAlphanumericSupplement::TortoiseShellBracketedLatinCapitalLetterS => '🄪',
            EnclosedAlphanumericSupplement::CircledItalicLatinCapitalLetterC => '🄫',
            EnclosedAlphanumericSupplement::CircledItalicLatinCapitalLetterR => '🄬',
            EnclosedAlphanumericSupplement::CircledCd => '🄭',
            EnclosedAlphanumericSupplement::CircledWz => '🄮',
            EnclosedAlphanumericSupplement::CopyleftSymbol => '🄯',
            EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterA => '🄰',
            EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterB => '🄱',
            EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterC => '🄲',
            EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterD => '🄳',
            EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterE => '🄴',
            EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterF => '🄵',
            EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterG => '🄶',
            EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterH => '🄷',
            EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterI => '🄸',
            EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterJ => '🄹',
            EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterK => '🄺',
            EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterL => '🄻',
            EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterM => '🄼',
            EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterN => '🄽',
            EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterO => '🄾',
            EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterP => '🄿',
            EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterQ => '🅀',
            EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterR => '🅁',
            EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterS => '🅂',
            EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterT => '🅃',
            EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterU => '🅄',
            EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterV => '🅅',
            EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterW => '🅆',
            EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterX => '🅇',
            EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterY => '🅈',
            EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterZ => '🅉',
            EnclosedAlphanumericSupplement::SquaredHv => '🅊',
            EnclosedAlphanumericSupplement::SquaredMv => '🅋',
            EnclosedAlphanumericSupplement::SquaredSd => '🅌',
            EnclosedAlphanumericSupplement::SquaredSs => '🅍',
            EnclosedAlphanumericSupplement::SquaredPpv => '🅎',
            EnclosedAlphanumericSupplement::SquaredWc => '🅏',
            EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterA => '🅐',
            EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterB => '🅑',
            EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterC => '🅒',
            EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterD => '🅓',
            EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterE => '🅔',
            EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterF => '🅕',
            EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterG => '🅖',
            EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterH => '🅗',
            EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterI => '🅘',
            EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterJ => '🅙',
            EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterK => '🅚',
            EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterL => '🅛',
            EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterM => '🅜',
            EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterN => '🅝',
            EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterO => '🅞',
            EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterP => '🅟',
            EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterQ => '🅠',
            EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterR => '🅡',
            EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterS => '🅢',
            EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterT => '🅣',
            EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterU => '🅤',
            EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterV => '🅥',
            EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterW => '🅦',
            EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterX => '🅧',
            EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterY => '🅨',
            EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterZ => '🅩',
            EnclosedAlphanumericSupplement::RaisedMcSign => '🅪',
            EnclosedAlphanumericSupplement::RaisedMdSign => '🅫',
            EnclosedAlphanumericSupplement::RaisedMrSign => '🅬',
            EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterA => '🅰',
            EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterB => '🅱',
            EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterC => '🅲',
            EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterD => '🅳',
            EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterE => '🅴',
            EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterF => '🅵',
            EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterG => '🅶',
            EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterH => '🅷',
            EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterI => '🅸',
            EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterJ => '🅹',
            EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterK => '🅺',
            EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterL => '🅻',
            EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterM => '🅼',
            EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterN => '🅽',
            EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterO => '🅾',
            EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterP => '🅿',
            EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterQ => '🆀',
            EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterR => '🆁',
            EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterS => '🆂',
            EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterT => '🆃',
            EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterU => '🆄',
            EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterV => '🆅',
            EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterW => '🆆',
            EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterX => '🆇',
            EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterY => '🆈',
            EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterZ => '🆉',
            EnclosedAlphanumericSupplement::CrossedNegativeSquaredLatinCapitalLetterP => '🆊',
            EnclosedAlphanumericSupplement::NegativeSquaredIc => '🆋',
            EnclosedAlphanumericSupplement::NegativeSquaredPa => '🆌',
            EnclosedAlphanumericSupplement::NegativeSquaredSa => '🆍',
            EnclosedAlphanumericSupplement::NegativeSquaredAb => '🆎',
            EnclosedAlphanumericSupplement::NegativeSquaredWc => '🆏',
            EnclosedAlphanumericSupplement::SquareDj => '🆐',
            EnclosedAlphanumericSupplement::SquaredCl => '🆑',
            EnclosedAlphanumericSupplement::SquaredCool => '🆒',
            EnclosedAlphanumericSupplement::SquaredFree => '🆓',
            EnclosedAlphanumericSupplement::SquaredId => '🆔',
            EnclosedAlphanumericSupplement::SquaredNew => '🆕',
            EnclosedAlphanumericSupplement::SquaredNg => '🆖',
            EnclosedAlphanumericSupplement::SquaredOk => '🆗',
            EnclosedAlphanumericSupplement::SquaredSos => '🆘',
            EnclosedAlphanumericSupplement::SquaredUpWithExclamationMark => '🆙',
            EnclosedAlphanumericSupplement::SquaredVs => '🆚',
            EnclosedAlphanumericSupplement::SquaredThreeD => '🆛',
            EnclosedAlphanumericSupplement::SquaredSecondScreen => '🆜',
            EnclosedAlphanumericSupplement::SquaredTwoK => '🆝',
            EnclosedAlphanumericSupplement::SquaredFourK => '🆞',
            EnclosedAlphanumericSupplement::SquaredEightK => '🆟',
            EnclosedAlphanumericSupplement::SquaredFivePointOne => '🆠',
            EnclosedAlphanumericSupplement::SquaredSevenPointOne => '🆡',
            EnclosedAlphanumericSupplement::SquaredTwentyDashTwoPointTwo => '🆢',
            EnclosedAlphanumericSupplement::SquaredSixtyP => '🆣',
            EnclosedAlphanumericSupplement::SquaredOneHundredTwentyP => '🆤',
            EnclosedAlphanumericSupplement::SquaredLatinSmallLetterD => '🆥',
            EnclosedAlphanumericSupplement::SquaredHc => '🆦',
            EnclosedAlphanumericSupplement::SquaredHdr => '🆧',
            EnclosedAlphanumericSupplement::SquaredHiDashRes => '🆨',
            EnclosedAlphanumericSupplement::SquaredLossless => '🆩',
            EnclosedAlphanumericSupplement::SquaredShv => '🆪',
            EnclosedAlphanumericSupplement::SquaredUhd => '🆫',
            EnclosedAlphanumericSupplement::SquaredVod => '🆬',
            EnclosedAlphanumericSupplement::RegionalIndicatorSymbolLetterA => '🇦',
            EnclosedAlphanumericSupplement::RegionalIndicatorSymbolLetterB => '🇧',
            EnclosedAlphanumericSupplement::RegionalIndicatorSymbolLetterC => '🇨',
            EnclosedAlphanumericSupplement::RegionalIndicatorSymbolLetterD => '🇩',
            EnclosedAlphanumericSupplement::RegionalIndicatorSymbolLetterE => '🇪',
            EnclosedAlphanumericSupplement::RegionalIndicatorSymbolLetterF => '🇫',
            EnclosedAlphanumericSupplement::RegionalIndicatorSymbolLetterG => '🇬',
            EnclosedAlphanumericSupplement::RegionalIndicatorSymbolLetterH => '🇭',
            EnclosedAlphanumericSupplement::RegionalIndicatorSymbolLetterI => '🇮',
            EnclosedAlphanumericSupplement::RegionalIndicatorSymbolLetterJ => '🇯',
            EnclosedAlphanumericSupplement::RegionalIndicatorSymbolLetterK => '🇰',
            EnclosedAlphanumericSupplement::RegionalIndicatorSymbolLetterL => '🇱',
            EnclosedAlphanumericSupplement::RegionalIndicatorSymbolLetterM => '🇲',
            EnclosedAlphanumericSupplement::RegionalIndicatorSymbolLetterN => '🇳',
            EnclosedAlphanumericSupplement::RegionalIndicatorSymbolLetterO => '🇴',
            EnclosedAlphanumericSupplement::RegionalIndicatorSymbolLetterP => '🇵',
            EnclosedAlphanumericSupplement::RegionalIndicatorSymbolLetterQ => '🇶',
            EnclosedAlphanumericSupplement::RegionalIndicatorSymbolLetterR => '🇷',
            EnclosedAlphanumericSupplement::RegionalIndicatorSymbolLetterS => '🇸',
            EnclosedAlphanumericSupplement::RegionalIndicatorSymbolLetterT => '🇹',
            EnclosedAlphanumericSupplement::RegionalIndicatorSymbolLetterU => '🇺',
            EnclosedAlphanumericSupplement::RegionalIndicatorSymbolLetterV => '🇻',
            EnclosedAlphanumericSupplement::RegionalIndicatorSymbolLetterW => '🇼',
            EnclosedAlphanumericSupplement::RegionalIndicatorSymbolLetterX => '🇽',
            EnclosedAlphanumericSupplement::RegionalIndicatorSymbolLetterY => '🇾',
        }
    }
}

impl std::convert::TryFrom<char> for EnclosedAlphanumericSupplement {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '🄀' => Ok(EnclosedAlphanumericSupplement::DigitZeroFullStop),
            '🄁' => Ok(EnclosedAlphanumericSupplement::DigitZeroComma),
            '🄂' => Ok(EnclosedAlphanumericSupplement::DigitOneComma),
            '🄃' => Ok(EnclosedAlphanumericSupplement::DigitTwoComma),
            '🄄' => Ok(EnclosedAlphanumericSupplement::DigitThreeComma),
            '🄅' => Ok(EnclosedAlphanumericSupplement::DigitFourComma),
            '🄆' => Ok(EnclosedAlphanumericSupplement::DigitFiveComma),
            '🄇' => Ok(EnclosedAlphanumericSupplement::DigitSixComma),
            '🄈' => Ok(EnclosedAlphanumericSupplement::DigitSevenComma),
            '🄉' => Ok(EnclosedAlphanumericSupplement::DigitEightComma),
            '🄊' => Ok(EnclosedAlphanumericSupplement::DigitNineComma),
            '🄋' => Ok(EnclosedAlphanumericSupplement::DingbatCircledSansDashSerifDigitZero),
            '🄌' => Ok(EnclosedAlphanumericSupplement::DingbatNegativeCircledSansDashSerifDigitZero),
            '🄐' => Ok(EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterA),
            '🄑' => Ok(EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterB),
            '🄒' => Ok(EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterC),
            '🄓' => Ok(EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterD),
            '🄔' => Ok(EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterE),
            '🄕' => Ok(EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterF),
            '🄖' => Ok(EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterG),
            '🄗' => Ok(EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterH),
            '🄘' => Ok(EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterI),
            '🄙' => Ok(EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterJ),
            '🄚' => Ok(EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterK),
            '🄛' => Ok(EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterL),
            '🄜' => Ok(EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterM),
            '🄝' => Ok(EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterN),
            '🄞' => Ok(EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterO),
            '🄟' => Ok(EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterP),
            '🄠' => Ok(EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterQ),
            '🄡' => Ok(EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterR),
            '🄢' => Ok(EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterS),
            '🄣' => Ok(EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterT),
            '🄤' => Ok(EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterU),
            '🄥' => Ok(EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterV),
            '🄦' => Ok(EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterW),
            '🄧' => Ok(EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterX),
            '🄨' => Ok(EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterY),
            '🄩' => Ok(EnclosedAlphanumericSupplement::ParenthesizedLatinCapitalLetterZ),
            '🄪' => Ok(EnclosedAlphanumericSupplement::TortoiseShellBracketedLatinCapitalLetterS),
            '🄫' => Ok(EnclosedAlphanumericSupplement::CircledItalicLatinCapitalLetterC),
            '🄬' => Ok(EnclosedAlphanumericSupplement::CircledItalicLatinCapitalLetterR),
            '🄭' => Ok(EnclosedAlphanumericSupplement::CircledCd),
            '🄮' => Ok(EnclosedAlphanumericSupplement::CircledWz),
            '🄯' => Ok(EnclosedAlphanumericSupplement::CopyleftSymbol),
            '🄰' => Ok(EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterA),
            '🄱' => Ok(EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterB),
            '🄲' => Ok(EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterC),
            '🄳' => Ok(EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterD),
            '🄴' => Ok(EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterE),
            '🄵' => Ok(EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterF),
            '🄶' => Ok(EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterG),
            '🄷' => Ok(EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterH),
            '🄸' => Ok(EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterI),
            '🄹' => Ok(EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterJ),
            '🄺' => Ok(EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterK),
            '🄻' => Ok(EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterL),
            '🄼' => Ok(EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterM),
            '🄽' => Ok(EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterN),
            '🄾' => Ok(EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterO),
            '🄿' => Ok(EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterP),
            '🅀' => Ok(EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterQ),
            '🅁' => Ok(EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterR),
            '🅂' => Ok(EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterS),
            '🅃' => Ok(EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterT),
            '🅄' => Ok(EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterU),
            '🅅' => Ok(EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterV),
            '🅆' => Ok(EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterW),
            '🅇' => Ok(EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterX),
            '🅈' => Ok(EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterY),
            '🅉' => Ok(EnclosedAlphanumericSupplement::SquaredLatinCapitalLetterZ),
            '🅊' => Ok(EnclosedAlphanumericSupplement::SquaredHv),
            '🅋' => Ok(EnclosedAlphanumericSupplement::SquaredMv),
            '🅌' => Ok(EnclosedAlphanumericSupplement::SquaredSd),
            '🅍' => Ok(EnclosedAlphanumericSupplement::SquaredSs),
            '🅎' => Ok(EnclosedAlphanumericSupplement::SquaredPpv),
            '🅏' => Ok(EnclosedAlphanumericSupplement::SquaredWc),
            '🅐' => Ok(EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterA),
            '🅑' => Ok(EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterB),
            '🅒' => Ok(EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterC),
            '🅓' => Ok(EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterD),
            '🅔' => Ok(EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterE),
            '🅕' => Ok(EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterF),
            '🅖' => Ok(EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterG),
            '🅗' => Ok(EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterH),
            '🅘' => Ok(EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterI),
            '🅙' => Ok(EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterJ),
            '🅚' => Ok(EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterK),
            '🅛' => Ok(EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterL),
            '🅜' => Ok(EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterM),
            '🅝' => Ok(EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterN),
            '🅞' => Ok(EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterO),
            '🅟' => Ok(EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterP),
            '🅠' => Ok(EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterQ),
            '🅡' => Ok(EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterR),
            '🅢' => Ok(EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterS),
            '🅣' => Ok(EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterT),
            '🅤' => Ok(EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterU),
            '🅥' => Ok(EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterV),
            '🅦' => Ok(EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterW),
            '🅧' => Ok(EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterX),
            '🅨' => Ok(EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterY),
            '🅩' => Ok(EnclosedAlphanumericSupplement::NegativeCircledLatinCapitalLetterZ),
            '🅪' => Ok(EnclosedAlphanumericSupplement::RaisedMcSign),
            '🅫' => Ok(EnclosedAlphanumericSupplement::RaisedMdSign),
            '🅬' => Ok(EnclosedAlphanumericSupplement::RaisedMrSign),
            '🅰' => Ok(EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterA),
            '🅱' => Ok(EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterB),
            '🅲' => Ok(EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterC),
            '🅳' => Ok(EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterD),
            '🅴' => Ok(EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterE),
            '🅵' => Ok(EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterF),
            '🅶' => Ok(EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterG),
            '🅷' => Ok(EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterH),
            '🅸' => Ok(EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterI),
            '🅹' => Ok(EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterJ),
            '🅺' => Ok(EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterK),
            '🅻' => Ok(EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterL),
            '🅼' => Ok(EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterM),
            '🅽' => Ok(EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterN),
            '🅾' => Ok(EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterO),
            '🅿' => Ok(EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterP),
            '🆀' => Ok(EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterQ),
            '🆁' => Ok(EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterR),
            '🆂' => Ok(EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterS),
            '🆃' => Ok(EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterT),
            '🆄' => Ok(EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterU),
            '🆅' => Ok(EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterV),
            '🆆' => Ok(EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterW),
            '🆇' => Ok(EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterX),
            '🆈' => Ok(EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterY),
            '🆉' => Ok(EnclosedAlphanumericSupplement::NegativeSquaredLatinCapitalLetterZ),
            '🆊' => Ok(EnclosedAlphanumericSupplement::CrossedNegativeSquaredLatinCapitalLetterP),
            '🆋' => Ok(EnclosedAlphanumericSupplement::NegativeSquaredIc),
            '🆌' => Ok(EnclosedAlphanumericSupplement::NegativeSquaredPa),
            '🆍' => Ok(EnclosedAlphanumericSupplement::NegativeSquaredSa),
            '🆎' => Ok(EnclosedAlphanumericSupplement::NegativeSquaredAb),
            '🆏' => Ok(EnclosedAlphanumericSupplement::NegativeSquaredWc),
            '🆐' => Ok(EnclosedAlphanumericSupplement::SquareDj),
            '🆑' => Ok(EnclosedAlphanumericSupplement::SquaredCl),
            '🆒' => Ok(EnclosedAlphanumericSupplement::SquaredCool),
            '🆓' => Ok(EnclosedAlphanumericSupplement::SquaredFree),
            '🆔' => Ok(EnclosedAlphanumericSupplement::SquaredId),
            '🆕' => Ok(EnclosedAlphanumericSupplement::SquaredNew),
            '🆖' => Ok(EnclosedAlphanumericSupplement::SquaredNg),
            '🆗' => Ok(EnclosedAlphanumericSupplement::SquaredOk),
            '🆘' => Ok(EnclosedAlphanumericSupplement::SquaredSos),
            '🆙' => Ok(EnclosedAlphanumericSupplement::SquaredUpWithExclamationMark),
            '🆚' => Ok(EnclosedAlphanumericSupplement::SquaredVs),
            '🆛' => Ok(EnclosedAlphanumericSupplement::SquaredThreeD),
            '🆜' => Ok(EnclosedAlphanumericSupplement::SquaredSecondScreen),
            '🆝' => Ok(EnclosedAlphanumericSupplement::SquaredTwoK),
            '🆞' => Ok(EnclosedAlphanumericSupplement::SquaredFourK),
            '🆟' => Ok(EnclosedAlphanumericSupplement::SquaredEightK),
            '🆠' => Ok(EnclosedAlphanumericSupplement::SquaredFivePointOne),
            '🆡' => Ok(EnclosedAlphanumericSupplement::SquaredSevenPointOne),
            '🆢' => Ok(EnclosedAlphanumericSupplement::SquaredTwentyDashTwoPointTwo),
            '🆣' => Ok(EnclosedAlphanumericSupplement::SquaredSixtyP),
            '🆤' => Ok(EnclosedAlphanumericSupplement::SquaredOneHundredTwentyP),
            '🆥' => Ok(EnclosedAlphanumericSupplement::SquaredLatinSmallLetterD),
            '🆦' => Ok(EnclosedAlphanumericSupplement::SquaredHc),
            '🆧' => Ok(EnclosedAlphanumericSupplement::SquaredHdr),
            '🆨' => Ok(EnclosedAlphanumericSupplement::SquaredHiDashRes),
            '🆩' => Ok(EnclosedAlphanumericSupplement::SquaredLossless),
            '🆪' => Ok(EnclosedAlphanumericSupplement::SquaredShv),
            '🆫' => Ok(EnclosedAlphanumericSupplement::SquaredUhd),
            '🆬' => Ok(EnclosedAlphanumericSupplement::SquaredVod),
            '🇦' => Ok(EnclosedAlphanumericSupplement::RegionalIndicatorSymbolLetterA),
            '🇧' => Ok(EnclosedAlphanumericSupplement::RegionalIndicatorSymbolLetterB),
            '🇨' => Ok(EnclosedAlphanumericSupplement::RegionalIndicatorSymbolLetterC),
            '🇩' => Ok(EnclosedAlphanumericSupplement::RegionalIndicatorSymbolLetterD),
            '🇪' => Ok(EnclosedAlphanumericSupplement::RegionalIndicatorSymbolLetterE),
            '🇫' => Ok(EnclosedAlphanumericSupplement::RegionalIndicatorSymbolLetterF),
            '🇬' => Ok(EnclosedAlphanumericSupplement::RegionalIndicatorSymbolLetterG),
            '🇭' => Ok(EnclosedAlphanumericSupplement::RegionalIndicatorSymbolLetterH),
            '🇮' => Ok(EnclosedAlphanumericSupplement::RegionalIndicatorSymbolLetterI),
            '🇯' => Ok(EnclosedAlphanumericSupplement::RegionalIndicatorSymbolLetterJ),
            '🇰' => Ok(EnclosedAlphanumericSupplement::RegionalIndicatorSymbolLetterK),
            '🇱' => Ok(EnclosedAlphanumericSupplement::RegionalIndicatorSymbolLetterL),
            '🇲' => Ok(EnclosedAlphanumericSupplement::RegionalIndicatorSymbolLetterM),
            '🇳' => Ok(EnclosedAlphanumericSupplement::RegionalIndicatorSymbolLetterN),
            '🇴' => Ok(EnclosedAlphanumericSupplement::RegionalIndicatorSymbolLetterO),
            '🇵' => Ok(EnclosedAlphanumericSupplement::RegionalIndicatorSymbolLetterP),
            '🇶' => Ok(EnclosedAlphanumericSupplement::RegionalIndicatorSymbolLetterQ),
            '🇷' => Ok(EnclosedAlphanumericSupplement::RegionalIndicatorSymbolLetterR),
            '🇸' => Ok(EnclosedAlphanumericSupplement::RegionalIndicatorSymbolLetterS),
            '🇹' => Ok(EnclosedAlphanumericSupplement::RegionalIndicatorSymbolLetterT),
            '🇺' => Ok(EnclosedAlphanumericSupplement::RegionalIndicatorSymbolLetterU),
            '🇻' => Ok(EnclosedAlphanumericSupplement::RegionalIndicatorSymbolLetterV),
            '🇼' => Ok(EnclosedAlphanumericSupplement::RegionalIndicatorSymbolLetterW),
            '🇽' => Ok(EnclosedAlphanumericSupplement::RegionalIndicatorSymbolLetterX),
            '🇾' => Ok(EnclosedAlphanumericSupplement::RegionalIndicatorSymbolLetterY),
            _ => Err(()),
        }
    }
}

impl Into<u32> for EnclosedAlphanumericSupplement {
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

impl std::convert::TryFrom<u32> for EnclosedAlphanumericSupplement {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for EnclosedAlphanumericSupplement {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl EnclosedAlphanumericSupplement {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        EnclosedAlphanumericSupplement::DigitZeroFullStop
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("EnclosedAlphanumericSupplement{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
