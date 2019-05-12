
/// An enum to represent all characters in the MusicalSymbols block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum MusicalSymbols {
    /// \u{1d100}: '𝄀'
    MusicalSymbolSingleBarline,
    /// \u{1d101}: '𝄁'
    MusicalSymbolDoubleBarline,
    /// \u{1d102}: '𝄂'
    MusicalSymbolFinalBarline,
    /// \u{1d103}: '𝄃'
    MusicalSymbolReverseFinalBarline,
    /// \u{1d104}: '𝄄'
    MusicalSymbolDashedBarline,
    /// \u{1d105}: '𝄅'
    MusicalSymbolShortBarline,
    /// \u{1d106}: '𝄆'
    MusicalSymbolLeftRepeatSign,
    /// \u{1d107}: '𝄇'
    MusicalSymbolRightRepeatSign,
    /// \u{1d108}: '𝄈'
    MusicalSymbolRepeatDots,
    /// \u{1d109}: '𝄉'
    MusicalSymbolDalSegno,
    /// \u{1d10a}: '𝄊'
    MusicalSymbolDaCapo,
    /// \u{1d10b}: '𝄋'
    MusicalSymbolSegno,
    /// \u{1d10c}: '𝄌'
    MusicalSymbolCoda,
    /// \u{1d10d}: '𝄍'
    MusicalSymbolRepeatedFigureDash1,
    /// \u{1d10e}: '𝄎'
    MusicalSymbolRepeatedFigureDash2,
    /// \u{1d10f}: '𝄏'
    MusicalSymbolRepeatedFigureDash3,
    /// \u{1d110}: '𝄐'
    MusicalSymbolFermata,
    /// \u{1d111}: '𝄑'
    MusicalSymbolFermataBelow,
    /// \u{1d112}: '𝄒'
    MusicalSymbolBreathMark,
    /// \u{1d113}: '𝄓'
    MusicalSymbolCaesura,
    /// \u{1d114}: '𝄔'
    MusicalSymbolBrace,
    /// \u{1d115}: '𝄕'
    MusicalSymbolBracket,
    /// \u{1d116}: '𝄖'
    MusicalSymbolOneDashLineStaff,
    /// \u{1d117}: '𝄗'
    MusicalSymbolTwoDashLineStaff,
    /// \u{1d118}: '𝄘'
    MusicalSymbolThreeDashLineStaff,
    /// \u{1d119}: '𝄙'
    MusicalSymbolFourDashLineStaff,
    /// \u{1d11a}: '𝄚'
    MusicalSymbolFiveDashLineStaff,
    /// \u{1d11b}: '𝄛'
    MusicalSymbolSixDashLineStaff,
    /// \u{1d11c}: '𝄜'
    MusicalSymbolSixDashStringFretboard,
    /// \u{1d11d}: '𝄝'
    MusicalSymbolFourDashStringFretboard,
    /// \u{1d11e}: '𝄞'
    MusicalSymbolGClef,
    /// \u{1d11f}: '𝄟'
    MusicalSymbolGClefOttavaAlta,
    /// \u{1d120}: '𝄠'
    MusicalSymbolGClefOttavaBassa,
    /// \u{1d121}: '𝄡'
    MusicalSymbolCClef,
    /// \u{1d122}: '𝄢'
    MusicalSymbolFClef,
    /// \u{1d123}: '𝄣'
    MusicalSymbolFClefOttavaAlta,
    /// \u{1d124}: '𝄤'
    MusicalSymbolFClefOttavaBassa,
    /// \u{1d125}: '𝄥'
    MusicalSymbolDrumClefDash1,
    /// \u{1d126}: '𝄦'
    MusicalSymbolDrumClefDash2,
    /// \u{1d129}: '𝄩'
    MusicalSymbolMultipleMeasureRest,
    /// \u{1d12a}: '𝄪'
    MusicalSymbolDoubleSharp,
    /// \u{1d12b}: '𝄫'
    MusicalSymbolDoubleFlat,
    /// \u{1d12c}: '𝄬'
    MusicalSymbolFlatUp,
    /// \u{1d12d}: '𝄭'
    MusicalSymbolFlatDown,
    /// \u{1d12e}: '𝄮'
    MusicalSymbolNaturalUp,
    /// \u{1d12f}: '𝄯'
    MusicalSymbolNaturalDown,
    /// \u{1d130}: '𝄰'
    MusicalSymbolSharpUp,
    /// \u{1d131}: '𝄱'
    MusicalSymbolSharpDown,
    /// \u{1d132}: '𝄲'
    MusicalSymbolQuarterToneSharp,
    /// \u{1d133}: '𝄳'
    MusicalSymbolQuarterToneFlat,
    /// \u{1d134}: '𝄴'
    MusicalSymbolCommonTime,
    /// \u{1d135}: '𝄵'
    MusicalSymbolCutTime,
    /// \u{1d136}: '𝄶'
    MusicalSymbolOttavaAlta,
    /// \u{1d137}: '𝄷'
    MusicalSymbolOttavaBassa,
    /// \u{1d138}: '𝄸'
    MusicalSymbolQuindicesimaAlta,
    /// \u{1d139}: '𝄹'
    MusicalSymbolQuindicesimaBassa,
    /// \u{1d13a}: '𝄺'
    MusicalSymbolMultiRest,
    /// \u{1d13b}: '𝄻'
    MusicalSymbolWholeRest,
    /// \u{1d13c}: '𝄼'
    MusicalSymbolHalfRest,
    /// \u{1d13d}: '𝄽'
    MusicalSymbolQuarterRest,
    /// \u{1d13e}: '𝄾'
    MusicalSymbolEighthRest,
    /// \u{1d13f}: '𝄿'
    MusicalSymbolSixteenthRest,
    /// \u{1d140}: '𝅀'
    MusicalSymbolThirtyDashSecondRest,
    /// \u{1d141}: '𝅁'
    MusicalSymbolSixtyDashFourthRest,
    /// \u{1d142}: '𝅂'
    MusicalSymbolOneHundredTwentyDashEighthRest,
    /// \u{1d143}: '𝅃'
    MusicalSymbolXNotehead,
    /// \u{1d144}: '𝅄'
    MusicalSymbolPlusNotehead,
    /// \u{1d145}: '𝅅'
    MusicalSymbolCircleXNotehead,
    /// \u{1d146}: '𝅆'
    MusicalSymbolSquareNoteheadWhite,
    /// \u{1d147}: '𝅇'
    MusicalSymbolSquareNoteheadBlack,
    /// \u{1d148}: '𝅈'
    MusicalSymbolTriangleNoteheadUpWhite,
    /// \u{1d149}: '𝅉'
    MusicalSymbolTriangleNoteheadUpBlack,
    /// \u{1d14a}: '𝅊'
    MusicalSymbolTriangleNoteheadLeftWhite,
    /// \u{1d14b}: '𝅋'
    MusicalSymbolTriangleNoteheadLeftBlack,
    /// \u{1d14c}: '𝅌'
    MusicalSymbolTriangleNoteheadRightWhite,
    /// \u{1d14d}: '𝅍'
    MusicalSymbolTriangleNoteheadRightBlack,
    /// \u{1d14e}: '𝅎'
    MusicalSymbolTriangleNoteheadDownWhite,
    /// \u{1d14f}: '𝅏'
    MusicalSymbolTriangleNoteheadDownBlack,
    /// \u{1d150}: '𝅐'
    MusicalSymbolTriangleNoteheadUpRightWhite,
    /// \u{1d151}: '𝅑'
    MusicalSymbolTriangleNoteheadUpRightBlack,
    /// \u{1d152}: '𝅒'
    MusicalSymbolMoonNoteheadWhite,
    /// \u{1d153}: '𝅓'
    MusicalSymbolMoonNoteheadBlack,
    /// \u{1d154}: '𝅔'
    MusicalSymbolTriangleDashRoundNoteheadDownWhite,
    /// \u{1d155}: '𝅕'
    MusicalSymbolTriangleDashRoundNoteheadDownBlack,
    /// \u{1d156}: '𝅖'
    MusicalSymbolParenthesisNotehead,
    /// \u{1d157}: '𝅗'
    MusicalSymbolVoidNotehead,
    /// \u{1d158}: '𝅘'
    MusicalSymbolNoteheadBlack,
    /// \u{1d159}: '𝅙'
    MusicalSymbolNullNotehead,
    /// \u{1d15a}: '𝅚'
    MusicalSymbolClusterNoteheadWhite,
    /// \u{1d15b}: '𝅛'
    MusicalSymbolClusterNoteheadBlack,
    /// \u{1d15c}: '𝅜'
    MusicalSymbolBreve,
    /// \u{1d15d}: '𝅝'
    MusicalSymbolWholeNote,
    /// \u{1d15e}: '𝅗𝅥'
    MusicalSymbolHalfNote,
    /// \u{1d15f}: '𝅘𝅥'
    MusicalSymbolQuarterNote,
    /// \u{1d160}: '𝅘𝅥𝅮'
    MusicalSymbolEighthNote,
    /// \u{1d161}: '𝅘𝅥𝅯'
    MusicalSymbolSixteenthNote,
    /// \u{1d162}: '𝅘𝅥𝅰'
    MusicalSymbolThirtyDashSecondNote,
    /// \u{1d163}: '𝅘𝅥𝅱'
    MusicalSymbolSixtyDashFourthNote,
    /// \u{1d164}: '𝅘𝅥𝅲'
    MusicalSymbolOneHundredTwentyDashEighthNote,
    /// \u{1d165}: '𝅥'
    MusicalSymbolCombiningStem,
    /// \u{1d166}: '𝅦'
    MusicalSymbolCombiningSprechgesangStem,
    /// \u{1d167}: '𝅧'
    MusicalSymbolCombiningTremoloDash1,
    /// \u{1d168}: '𝅨'
    MusicalSymbolCombiningTremoloDash2,
    /// \u{1d169}: '𝅩'
    MusicalSymbolCombiningTremoloDash3,
    /// \u{1d16a}: '𝅪'
    MusicalSymbolFingeredTremoloDash1,
    /// \u{1d16b}: '𝅫'
    MusicalSymbolFingeredTremoloDash2,
    /// \u{1d16c}: '𝅬'
    MusicalSymbolFingeredTremoloDash3,
    /// \u{1d16d}: '𝅭'
    MusicalSymbolCombiningAugmentationDot,
    /// \u{1d16e}: '𝅮'
    MusicalSymbolCombiningFlagDash1,
    /// \u{1d16f}: '𝅯'
    MusicalSymbolCombiningFlagDash2,
    /// \u{1d170}: '𝅰'
    MusicalSymbolCombiningFlagDash3,
    /// \u{1d171}: '𝅱'
    MusicalSymbolCombiningFlagDash4,
    /// \u{1d172}: '𝅲'
    MusicalSymbolCombiningFlagDash5,
    /// \u{1d173}: '𝅳'
    MusicalSymbolBeginBeam,
    /// \u{1d174}: '𝅴'
    MusicalSymbolEndBeam,
    /// \u{1d175}: '𝅵'
    MusicalSymbolBeginTie,
    /// \u{1d176}: '𝅶'
    MusicalSymbolEndTie,
    /// \u{1d177}: '𝅷'
    MusicalSymbolBeginSlur,
    /// \u{1d178}: '𝅸'
    MusicalSymbolEndSlur,
    /// \u{1d179}: '𝅹'
    MusicalSymbolBeginPhrase,
    /// \u{1d17a}: '𝅺'
    MusicalSymbolEndPhrase,
    /// \u{1d17b}: '𝅻'
    MusicalSymbolCombiningAccent,
    /// \u{1d17c}: '𝅼'
    MusicalSymbolCombiningStaccato,
    /// \u{1d17d}: '𝅽'
    MusicalSymbolCombiningTenuto,
    /// \u{1d17e}: '𝅾'
    MusicalSymbolCombiningStaccatissimo,
    /// \u{1d17f}: '𝅿'
    MusicalSymbolCombiningMarcato,
    /// \u{1d180}: '𝆀'
    MusicalSymbolCombiningMarcatoDashStaccato,
    /// \u{1d181}: '𝆁'
    MusicalSymbolCombiningAccentDashStaccato,
    /// \u{1d182}: '𝆂'
    MusicalSymbolCombiningLoure,
    /// \u{1d183}: '𝆃'
    MusicalSymbolArpeggiatoUp,
    /// \u{1d184}: '𝆄'
    MusicalSymbolArpeggiatoDown,
    /// \u{1d185}: '𝆅'
    MusicalSymbolCombiningDoit,
    /// \u{1d186}: '𝆆'
    MusicalSymbolCombiningRip,
    /// \u{1d187}: '𝆇'
    MusicalSymbolCombiningFlip,
    /// \u{1d188}: '𝆈'
    MusicalSymbolCombiningSmear,
    /// \u{1d189}: '𝆉'
    MusicalSymbolCombiningBend,
    /// \u{1d18a}: '𝆊'
    MusicalSymbolCombiningDoubleTongue,
    /// \u{1d18b}: '𝆋'
    MusicalSymbolCombiningTripleTongue,
    /// \u{1d18c}: '𝆌'
    MusicalSymbolRinforzando,
    /// \u{1d18d}: '𝆍'
    MusicalSymbolSubito,
    /// \u{1d18e}: '𝆎'
    MusicalSymbolZ,
    /// \u{1d18f}: '𝆏'
    MusicalSymbolPiano,
    /// \u{1d190}: '𝆐'
    MusicalSymbolMezzo,
    /// \u{1d191}: '𝆑'
    MusicalSymbolForte,
    /// \u{1d192}: '𝆒'
    MusicalSymbolCrescendo,
    /// \u{1d193}: '𝆓'
    MusicalSymbolDecrescendo,
    /// \u{1d194}: '𝆔'
    MusicalSymbolGraceNoteSlash,
    /// \u{1d195}: '𝆕'
    MusicalSymbolGraceNoteNoSlash,
    /// \u{1d196}: '𝆖'
    MusicalSymbolTr,
    /// \u{1d197}: '𝆗'
    MusicalSymbolTurn,
    /// \u{1d198}: '𝆘'
    MusicalSymbolInvertedTurn,
    /// \u{1d199}: '𝆙'
    MusicalSymbolTurnSlash,
    /// \u{1d19a}: '𝆚'
    MusicalSymbolTurnUp,
    /// \u{1d19b}: '𝆛'
    MusicalSymbolOrnamentStrokeDash1,
    /// \u{1d19c}: '𝆜'
    MusicalSymbolOrnamentStrokeDash2,
    /// \u{1d19d}: '𝆝'
    MusicalSymbolOrnamentStrokeDash3,
    /// \u{1d19e}: '𝆞'
    MusicalSymbolOrnamentStrokeDash4,
    /// \u{1d19f}: '𝆟'
    MusicalSymbolOrnamentStrokeDash5,
    /// \u{1d1a0}: '𝆠'
    MusicalSymbolOrnamentStrokeDash6,
    /// \u{1d1a1}: '𝆡'
    MusicalSymbolOrnamentStrokeDash7,
    /// \u{1d1a2}: '𝆢'
    MusicalSymbolOrnamentStrokeDash8,
    /// \u{1d1a3}: '𝆣'
    MusicalSymbolOrnamentStrokeDash9,
    /// \u{1d1a4}: '𝆤'
    MusicalSymbolOrnamentStrokeDash10,
    /// \u{1d1a5}: '𝆥'
    MusicalSymbolOrnamentStrokeDash11,
    /// \u{1d1a6}: '𝆦'
    MusicalSymbolHauptstimme,
    /// \u{1d1a7}: '𝆧'
    MusicalSymbolNebenstimme,
    /// \u{1d1a8}: '𝆨'
    MusicalSymbolEndOfStimme,
    /// \u{1d1a9}: '𝆩'
    MusicalSymbolDegreeSlash,
    /// \u{1d1aa}: '𝆪'
    MusicalSymbolCombiningDownBow,
    /// \u{1d1ab}: '𝆫'
    MusicalSymbolCombiningUpBow,
    /// \u{1d1ac}: '𝆬'
    MusicalSymbolCombiningHarmonic,
    /// \u{1d1ad}: '𝆭'
    MusicalSymbolCombiningSnapPizzicato,
    /// \u{1d1ae}: '𝆮'
    MusicalSymbolPedalMark,
    /// \u{1d1af}: '𝆯'
    MusicalSymbolPedalUpMark,
    /// \u{1d1b0}: '𝆰'
    MusicalSymbolHalfPedalMark,
    /// \u{1d1b1}: '𝆱'
    MusicalSymbolGlissandoUp,
    /// \u{1d1b2}: '𝆲'
    MusicalSymbolGlissandoDown,
    /// \u{1d1b3}: '𝆳'
    MusicalSymbolWithFingernails,
    /// \u{1d1b4}: '𝆴'
    MusicalSymbolDamp,
    /// \u{1d1b5}: '𝆵'
    MusicalSymbolDampAll,
    /// \u{1d1b6}: '𝆶'
    MusicalSymbolMaxima,
    /// \u{1d1b7}: '𝆷'
    MusicalSymbolLonga,
    /// \u{1d1b8}: '𝆸'
    MusicalSymbolBrevis,
    /// \u{1d1b9}: '𝆹'
    MusicalSymbolSemibrevisWhite,
    /// \u{1d1ba}: '𝆺'
    MusicalSymbolSemibrevisBlack,
    /// \u{1d1bb}: '𝆹𝅥'
    MusicalSymbolMinima,
    /// \u{1d1bc}: '𝆺𝅥'
    MusicalSymbolMinimaBlack,
    /// \u{1d1bd}: '𝆹𝅥𝅮'
    MusicalSymbolSemiminimaWhite,
    /// \u{1d1be}: '𝆺𝅥𝅮'
    MusicalSymbolSemiminimaBlack,
    /// \u{1d1bf}: '𝆹𝅥𝅯'
    MusicalSymbolFusaWhite,
    /// \u{1d1c0}: '𝆺𝅥𝅯'
    MusicalSymbolFusaBlack,
    /// \u{1d1c1}: '𝇁'
    MusicalSymbolLongaPerfectaRest,
    /// \u{1d1c2}: '𝇂'
    MusicalSymbolLongaImperfectaRest,
    /// \u{1d1c3}: '𝇃'
    MusicalSymbolBrevisRest,
    /// \u{1d1c4}: '𝇄'
    MusicalSymbolSemibrevisRest,
    /// \u{1d1c5}: '𝇅'
    MusicalSymbolMinimaRest,
    /// \u{1d1c6}: '𝇆'
    MusicalSymbolSemiminimaRest,
    /// \u{1d1c7}: '𝇇'
    MusicalSymbolTempusPerfectumCumProlationePerfecta,
    /// \u{1d1c8}: '𝇈'
    MusicalSymbolTempusPerfectumCumProlationeImperfecta,
    /// \u{1d1c9}: '𝇉'
    MusicalSymbolTempusPerfectumCumProlationePerfectaDiminutionDash1,
    /// \u{1d1ca}: '𝇊'
    MusicalSymbolTempusImperfectumCumProlationePerfecta,
    /// \u{1d1cb}: '𝇋'
    MusicalSymbolTempusImperfectumCumProlationeImperfecta,
    /// \u{1d1cc}: '𝇌'
    MusicalSymbolTempusImperfectumCumProlationeImperfectaDiminutionDash1,
    /// \u{1d1cd}: '𝇍'
    MusicalSymbolTempusImperfectumCumProlationeImperfectaDiminutionDash2,
    /// \u{1d1ce}: '𝇎'
    MusicalSymbolTempusImperfectumCumProlationeImperfectaDiminutionDash3,
    /// \u{1d1cf}: '𝇏'
    MusicalSymbolCroix,
    /// \u{1d1d0}: '𝇐'
    MusicalSymbolGregorianCClef,
    /// \u{1d1d1}: '𝇑'
    MusicalSymbolGregorianFClef,
    /// \u{1d1d2}: '𝇒'
    MusicalSymbolSquareB,
    /// \u{1d1d3}: '𝇓'
    MusicalSymbolVirga,
    /// \u{1d1d4}: '𝇔'
    MusicalSymbolPodatus,
    /// \u{1d1d5}: '𝇕'
    MusicalSymbolClivis,
    /// \u{1d1d6}: '𝇖'
    MusicalSymbolScandicus,
    /// \u{1d1d7}: '𝇗'
    MusicalSymbolClimacus,
    /// \u{1d1d8}: '𝇘'
    MusicalSymbolTorculus,
    /// \u{1d1d9}: '𝇙'
    MusicalSymbolPorrectus,
    /// \u{1d1da}: '𝇚'
    MusicalSymbolPorrectusFlexus,
    /// \u{1d1db}: '𝇛'
    MusicalSymbolScandicusFlexus,
    /// \u{1d1dc}: '𝇜'
    MusicalSymbolTorculusResupinus,
    /// \u{1d1dd}: '𝇝'
    MusicalSymbolPesSubpunctis,
    /// \u{1d1de}: '𝇞'
    MusicalSymbolKievanCClef,
    /// \u{1d1df}: '𝇟'
    MusicalSymbolKievanEndOfPiece,
    /// \u{1d1e0}: '𝇠'
    MusicalSymbolKievanFinalNote,
    /// \u{1d1e1}: '𝇡'
    MusicalSymbolKievanRecitativeMark,
    /// \u{1d1e2}: '𝇢'
    MusicalSymbolKievanWholeNote,
    /// \u{1d1e3}: '𝇣'
    MusicalSymbolKievanHalfNote,
    /// \u{1d1e4}: '𝇤'
    MusicalSymbolKievanQuarterNoteStemDown,
    /// \u{1d1e5}: '𝇥'
    MusicalSymbolKievanQuarterNoteStemUp,
    /// \u{1d1e6}: '𝇦'
    MusicalSymbolKievanEighthNoteStemDown,
    /// \u{1d1e7}: '𝇧'
    MusicalSymbolKievanEighthNoteStemUp,
    /// \u{1d1e8}: '𝇨'
    MusicalSymbolKievanFlatSign,
}

impl Into<char> for MusicalSymbols {
    fn into(self) -> char {
        match self {
            MusicalSymbols::MusicalSymbolSingleBarline => '𝄀',
            MusicalSymbols::MusicalSymbolDoubleBarline => '𝄁',
            MusicalSymbols::MusicalSymbolFinalBarline => '𝄂',
            MusicalSymbols::MusicalSymbolReverseFinalBarline => '𝄃',
            MusicalSymbols::MusicalSymbolDashedBarline => '𝄄',
            MusicalSymbols::MusicalSymbolShortBarline => '𝄅',
            MusicalSymbols::MusicalSymbolLeftRepeatSign => '𝄆',
            MusicalSymbols::MusicalSymbolRightRepeatSign => '𝄇',
            MusicalSymbols::MusicalSymbolRepeatDots => '𝄈',
            MusicalSymbols::MusicalSymbolDalSegno => '𝄉',
            MusicalSymbols::MusicalSymbolDaCapo => '𝄊',
            MusicalSymbols::MusicalSymbolSegno => '𝄋',
            MusicalSymbols::MusicalSymbolCoda => '𝄌',
            MusicalSymbols::MusicalSymbolRepeatedFigureDash1 => '𝄍',
            MusicalSymbols::MusicalSymbolRepeatedFigureDash2 => '𝄎',
            MusicalSymbols::MusicalSymbolRepeatedFigureDash3 => '𝄏',
            MusicalSymbols::MusicalSymbolFermata => '𝄐',
            MusicalSymbols::MusicalSymbolFermataBelow => '𝄑',
            MusicalSymbols::MusicalSymbolBreathMark => '𝄒',
            MusicalSymbols::MusicalSymbolCaesura => '𝄓',
            MusicalSymbols::MusicalSymbolBrace => '𝄔',
            MusicalSymbols::MusicalSymbolBracket => '𝄕',
            MusicalSymbols::MusicalSymbolOneDashLineStaff => '𝄖',
            MusicalSymbols::MusicalSymbolTwoDashLineStaff => '𝄗',
            MusicalSymbols::MusicalSymbolThreeDashLineStaff => '𝄘',
            MusicalSymbols::MusicalSymbolFourDashLineStaff => '𝄙',
            MusicalSymbols::MusicalSymbolFiveDashLineStaff => '𝄚',
            MusicalSymbols::MusicalSymbolSixDashLineStaff => '𝄛',
            MusicalSymbols::MusicalSymbolSixDashStringFretboard => '𝄜',
            MusicalSymbols::MusicalSymbolFourDashStringFretboard => '𝄝',
            MusicalSymbols::MusicalSymbolGClef => '𝄞',
            MusicalSymbols::MusicalSymbolGClefOttavaAlta => '𝄟',
            MusicalSymbols::MusicalSymbolGClefOttavaBassa => '𝄠',
            MusicalSymbols::MusicalSymbolCClef => '𝄡',
            MusicalSymbols::MusicalSymbolFClef => '𝄢',
            MusicalSymbols::MusicalSymbolFClefOttavaAlta => '𝄣',
            MusicalSymbols::MusicalSymbolFClefOttavaBassa => '𝄤',
            MusicalSymbols::MusicalSymbolDrumClefDash1 => '𝄥',
            MusicalSymbols::MusicalSymbolDrumClefDash2 => '𝄦',
            MusicalSymbols::MusicalSymbolMultipleMeasureRest => '𝄩',
            MusicalSymbols::MusicalSymbolDoubleSharp => '𝄪',
            MusicalSymbols::MusicalSymbolDoubleFlat => '𝄫',
            MusicalSymbols::MusicalSymbolFlatUp => '𝄬',
            MusicalSymbols::MusicalSymbolFlatDown => '𝄭',
            MusicalSymbols::MusicalSymbolNaturalUp => '𝄮',
            MusicalSymbols::MusicalSymbolNaturalDown => '𝄯',
            MusicalSymbols::MusicalSymbolSharpUp => '𝄰',
            MusicalSymbols::MusicalSymbolSharpDown => '𝄱',
            MusicalSymbols::MusicalSymbolQuarterToneSharp => '𝄲',
            MusicalSymbols::MusicalSymbolQuarterToneFlat => '𝄳',
            MusicalSymbols::MusicalSymbolCommonTime => '𝄴',
            MusicalSymbols::MusicalSymbolCutTime => '𝄵',
            MusicalSymbols::MusicalSymbolOttavaAlta => '𝄶',
            MusicalSymbols::MusicalSymbolOttavaBassa => '𝄷',
            MusicalSymbols::MusicalSymbolQuindicesimaAlta => '𝄸',
            MusicalSymbols::MusicalSymbolQuindicesimaBassa => '𝄹',
            MusicalSymbols::MusicalSymbolMultiRest => '𝄺',
            MusicalSymbols::MusicalSymbolWholeRest => '𝄻',
            MusicalSymbols::MusicalSymbolHalfRest => '𝄼',
            MusicalSymbols::MusicalSymbolQuarterRest => '𝄽',
            MusicalSymbols::MusicalSymbolEighthRest => '𝄾',
            MusicalSymbols::MusicalSymbolSixteenthRest => '𝄿',
            MusicalSymbols::MusicalSymbolThirtyDashSecondRest => '𝅀',
            MusicalSymbols::MusicalSymbolSixtyDashFourthRest => '𝅁',
            MusicalSymbols::MusicalSymbolOneHundredTwentyDashEighthRest => '𝅂',
            MusicalSymbols::MusicalSymbolXNotehead => '𝅃',
            MusicalSymbols::MusicalSymbolPlusNotehead => '𝅄',
            MusicalSymbols::MusicalSymbolCircleXNotehead => '𝅅',
            MusicalSymbols::MusicalSymbolSquareNoteheadWhite => '𝅆',
            MusicalSymbols::MusicalSymbolSquareNoteheadBlack => '𝅇',
            MusicalSymbols::MusicalSymbolTriangleNoteheadUpWhite => '𝅈',
            MusicalSymbols::MusicalSymbolTriangleNoteheadUpBlack => '𝅉',
            MusicalSymbols::MusicalSymbolTriangleNoteheadLeftWhite => '𝅊',
            MusicalSymbols::MusicalSymbolTriangleNoteheadLeftBlack => '𝅋',
            MusicalSymbols::MusicalSymbolTriangleNoteheadRightWhite => '𝅌',
            MusicalSymbols::MusicalSymbolTriangleNoteheadRightBlack => '𝅍',
            MusicalSymbols::MusicalSymbolTriangleNoteheadDownWhite => '𝅎',
            MusicalSymbols::MusicalSymbolTriangleNoteheadDownBlack => '𝅏',
            MusicalSymbols::MusicalSymbolTriangleNoteheadUpRightWhite => '𝅐',
            MusicalSymbols::MusicalSymbolTriangleNoteheadUpRightBlack => '𝅑',
            MusicalSymbols::MusicalSymbolMoonNoteheadWhite => '𝅒',
            MusicalSymbols::MusicalSymbolMoonNoteheadBlack => '𝅓',
            MusicalSymbols::MusicalSymbolTriangleDashRoundNoteheadDownWhite => '𝅔',
            MusicalSymbols::MusicalSymbolTriangleDashRoundNoteheadDownBlack => '𝅕',
            MusicalSymbols::MusicalSymbolParenthesisNotehead => '𝅖',
            MusicalSymbols::MusicalSymbolVoidNotehead => '𝅗',
            MusicalSymbols::MusicalSymbolNoteheadBlack => '𝅘',
            MusicalSymbols::MusicalSymbolNullNotehead => '𝅙',
            MusicalSymbols::MusicalSymbolClusterNoteheadWhite => '𝅚',
            MusicalSymbols::MusicalSymbolClusterNoteheadBlack => '𝅛',
            MusicalSymbols::MusicalSymbolBreve => '𝅜',
            MusicalSymbols::MusicalSymbolWholeNote => '𝅝',
            MusicalSymbols::MusicalSymbolHalfNote => '𝅗𝅥',
            MusicalSymbols::MusicalSymbolQuarterNote => '𝅘𝅥',
            MusicalSymbols::MusicalSymbolEighthNote => '𝅘𝅥𝅮',
            MusicalSymbols::MusicalSymbolSixteenthNote => '𝅘𝅥𝅯',
            MusicalSymbols::MusicalSymbolThirtyDashSecondNote => '𝅘𝅥𝅰',
            MusicalSymbols::MusicalSymbolSixtyDashFourthNote => '𝅘𝅥𝅱',
            MusicalSymbols::MusicalSymbolOneHundredTwentyDashEighthNote => '𝅘𝅥𝅲',
            MusicalSymbols::MusicalSymbolCombiningStem => '𝅥',
            MusicalSymbols::MusicalSymbolCombiningSprechgesangStem => '𝅦',
            MusicalSymbols::MusicalSymbolCombiningTremoloDash1 => '𝅧',
            MusicalSymbols::MusicalSymbolCombiningTremoloDash2 => '𝅨',
            MusicalSymbols::MusicalSymbolCombiningTremoloDash3 => '𝅩',
            MusicalSymbols::MusicalSymbolFingeredTremoloDash1 => '𝅪',
            MusicalSymbols::MusicalSymbolFingeredTremoloDash2 => '𝅫',
            MusicalSymbols::MusicalSymbolFingeredTremoloDash3 => '𝅬',
            MusicalSymbols::MusicalSymbolCombiningAugmentationDot => '𝅭',
            MusicalSymbols::MusicalSymbolCombiningFlagDash1 => '𝅮',
            MusicalSymbols::MusicalSymbolCombiningFlagDash2 => '𝅯',
            MusicalSymbols::MusicalSymbolCombiningFlagDash3 => '𝅰',
            MusicalSymbols::MusicalSymbolCombiningFlagDash4 => '𝅱',
            MusicalSymbols::MusicalSymbolCombiningFlagDash5 => '𝅲',
            MusicalSymbols::MusicalSymbolBeginBeam => '𝅳',
            MusicalSymbols::MusicalSymbolEndBeam => '𝅴',
            MusicalSymbols::MusicalSymbolBeginTie => '𝅵',
            MusicalSymbols::MusicalSymbolEndTie => '𝅶',
            MusicalSymbols::MusicalSymbolBeginSlur => '𝅷',
            MusicalSymbols::MusicalSymbolEndSlur => '𝅸',
            MusicalSymbols::MusicalSymbolBeginPhrase => '𝅹',
            MusicalSymbols::MusicalSymbolEndPhrase => '𝅺',
            MusicalSymbols::MusicalSymbolCombiningAccent => '𝅻',
            MusicalSymbols::MusicalSymbolCombiningStaccato => '𝅼',
            MusicalSymbols::MusicalSymbolCombiningTenuto => '𝅽',
            MusicalSymbols::MusicalSymbolCombiningStaccatissimo => '𝅾',
            MusicalSymbols::MusicalSymbolCombiningMarcato => '𝅿',
            MusicalSymbols::MusicalSymbolCombiningMarcatoDashStaccato => '𝆀',
            MusicalSymbols::MusicalSymbolCombiningAccentDashStaccato => '𝆁',
            MusicalSymbols::MusicalSymbolCombiningLoure => '𝆂',
            MusicalSymbols::MusicalSymbolArpeggiatoUp => '𝆃',
            MusicalSymbols::MusicalSymbolArpeggiatoDown => '𝆄',
            MusicalSymbols::MusicalSymbolCombiningDoit => '𝆅',
            MusicalSymbols::MusicalSymbolCombiningRip => '𝆆',
            MusicalSymbols::MusicalSymbolCombiningFlip => '𝆇',
            MusicalSymbols::MusicalSymbolCombiningSmear => '𝆈',
            MusicalSymbols::MusicalSymbolCombiningBend => '𝆉',
            MusicalSymbols::MusicalSymbolCombiningDoubleTongue => '𝆊',
            MusicalSymbols::MusicalSymbolCombiningTripleTongue => '𝆋',
            MusicalSymbols::MusicalSymbolRinforzando => '𝆌',
            MusicalSymbols::MusicalSymbolSubito => '𝆍',
            MusicalSymbols::MusicalSymbolZ => '𝆎',
            MusicalSymbols::MusicalSymbolPiano => '𝆏',
            MusicalSymbols::MusicalSymbolMezzo => '𝆐',
            MusicalSymbols::MusicalSymbolForte => '𝆑',
            MusicalSymbols::MusicalSymbolCrescendo => '𝆒',
            MusicalSymbols::MusicalSymbolDecrescendo => '𝆓',
            MusicalSymbols::MusicalSymbolGraceNoteSlash => '𝆔',
            MusicalSymbols::MusicalSymbolGraceNoteNoSlash => '𝆕',
            MusicalSymbols::MusicalSymbolTr => '𝆖',
            MusicalSymbols::MusicalSymbolTurn => '𝆗',
            MusicalSymbols::MusicalSymbolInvertedTurn => '𝆘',
            MusicalSymbols::MusicalSymbolTurnSlash => '𝆙',
            MusicalSymbols::MusicalSymbolTurnUp => '𝆚',
            MusicalSymbols::MusicalSymbolOrnamentStrokeDash1 => '𝆛',
            MusicalSymbols::MusicalSymbolOrnamentStrokeDash2 => '𝆜',
            MusicalSymbols::MusicalSymbolOrnamentStrokeDash3 => '𝆝',
            MusicalSymbols::MusicalSymbolOrnamentStrokeDash4 => '𝆞',
            MusicalSymbols::MusicalSymbolOrnamentStrokeDash5 => '𝆟',
            MusicalSymbols::MusicalSymbolOrnamentStrokeDash6 => '𝆠',
            MusicalSymbols::MusicalSymbolOrnamentStrokeDash7 => '𝆡',
            MusicalSymbols::MusicalSymbolOrnamentStrokeDash8 => '𝆢',
            MusicalSymbols::MusicalSymbolOrnamentStrokeDash9 => '𝆣',
            MusicalSymbols::MusicalSymbolOrnamentStrokeDash10 => '𝆤',
            MusicalSymbols::MusicalSymbolOrnamentStrokeDash11 => '𝆥',
            MusicalSymbols::MusicalSymbolHauptstimme => '𝆦',
            MusicalSymbols::MusicalSymbolNebenstimme => '𝆧',
            MusicalSymbols::MusicalSymbolEndOfStimme => '𝆨',
            MusicalSymbols::MusicalSymbolDegreeSlash => '𝆩',
            MusicalSymbols::MusicalSymbolCombiningDownBow => '𝆪',
            MusicalSymbols::MusicalSymbolCombiningUpBow => '𝆫',
            MusicalSymbols::MusicalSymbolCombiningHarmonic => '𝆬',
            MusicalSymbols::MusicalSymbolCombiningSnapPizzicato => '𝆭',
            MusicalSymbols::MusicalSymbolPedalMark => '𝆮',
            MusicalSymbols::MusicalSymbolPedalUpMark => '𝆯',
            MusicalSymbols::MusicalSymbolHalfPedalMark => '𝆰',
            MusicalSymbols::MusicalSymbolGlissandoUp => '𝆱',
            MusicalSymbols::MusicalSymbolGlissandoDown => '𝆲',
            MusicalSymbols::MusicalSymbolWithFingernails => '𝆳',
            MusicalSymbols::MusicalSymbolDamp => '𝆴',
            MusicalSymbols::MusicalSymbolDampAll => '𝆵',
            MusicalSymbols::MusicalSymbolMaxima => '𝆶',
            MusicalSymbols::MusicalSymbolLonga => '𝆷',
            MusicalSymbols::MusicalSymbolBrevis => '𝆸',
            MusicalSymbols::MusicalSymbolSemibrevisWhite => '𝆹',
            MusicalSymbols::MusicalSymbolSemibrevisBlack => '𝆺',
            MusicalSymbols::MusicalSymbolMinima => '𝆹𝅥',
            MusicalSymbols::MusicalSymbolMinimaBlack => '𝆺𝅥',
            MusicalSymbols::MusicalSymbolSemiminimaWhite => '𝆹𝅥𝅮',
            MusicalSymbols::MusicalSymbolSemiminimaBlack => '𝆺𝅥𝅮',
            MusicalSymbols::MusicalSymbolFusaWhite => '𝆹𝅥𝅯',
            MusicalSymbols::MusicalSymbolFusaBlack => '𝆺𝅥𝅯',
            MusicalSymbols::MusicalSymbolLongaPerfectaRest => '𝇁',
            MusicalSymbols::MusicalSymbolLongaImperfectaRest => '𝇂',
            MusicalSymbols::MusicalSymbolBrevisRest => '𝇃',
            MusicalSymbols::MusicalSymbolSemibrevisRest => '𝇄',
            MusicalSymbols::MusicalSymbolMinimaRest => '𝇅',
            MusicalSymbols::MusicalSymbolSemiminimaRest => '𝇆',
            MusicalSymbols::MusicalSymbolTempusPerfectumCumProlationePerfecta => '𝇇',
            MusicalSymbols::MusicalSymbolTempusPerfectumCumProlationeImperfecta => '𝇈',
            MusicalSymbols::MusicalSymbolTempusPerfectumCumProlationePerfectaDiminutionDash1 => '𝇉',
            MusicalSymbols::MusicalSymbolTempusImperfectumCumProlationePerfecta => '𝇊',
            MusicalSymbols::MusicalSymbolTempusImperfectumCumProlationeImperfecta => '𝇋',
            MusicalSymbols::MusicalSymbolTempusImperfectumCumProlationeImperfectaDiminutionDash1 => '𝇌',
            MusicalSymbols::MusicalSymbolTempusImperfectumCumProlationeImperfectaDiminutionDash2 => '𝇍',
            MusicalSymbols::MusicalSymbolTempusImperfectumCumProlationeImperfectaDiminutionDash3 => '𝇎',
            MusicalSymbols::MusicalSymbolCroix => '𝇏',
            MusicalSymbols::MusicalSymbolGregorianCClef => '𝇐',
            MusicalSymbols::MusicalSymbolGregorianFClef => '𝇑',
            MusicalSymbols::MusicalSymbolSquareB => '𝇒',
            MusicalSymbols::MusicalSymbolVirga => '𝇓',
            MusicalSymbols::MusicalSymbolPodatus => '𝇔',
            MusicalSymbols::MusicalSymbolClivis => '𝇕',
            MusicalSymbols::MusicalSymbolScandicus => '𝇖',
            MusicalSymbols::MusicalSymbolClimacus => '𝇗',
            MusicalSymbols::MusicalSymbolTorculus => '𝇘',
            MusicalSymbols::MusicalSymbolPorrectus => '𝇙',
            MusicalSymbols::MusicalSymbolPorrectusFlexus => '𝇚',
            MusicalSymbols::MusicalSymbolScandicusFlexus => '𝇛',
            MusicalSymbols::MusicalSymbolTorculusResupinus => '𝇜',
            MusicalSymbols::MusicalSymbolPesSubpunctis => '𝇝',
            MusicalSymbols::MusicalSymbolKievanCClef => '𝇞',
            MusicalSymbols::MusicalSymbolKievanEndOfPiece => '𝇟',
            MusicalSymbols::MusicalSymbolKievanFinalNote => '𝇠',
            MusicalSymbols::MusicalSymbolKievanRecitativeMark => '𝇡',
            MusicalSymbols::MusicalSymbolKievanWholeNote => '𝇢',
            MusicalSymbols::MusicalSymbolKievanHalfNote => '𝇣',
            MusicalSymbols::MusicalSymbolKievanQuarterNoteStemDown => '𝇤',
            MusicalSymbols::MusicalSymbolKievanQuarterNoteStemUp => '𝇥',
            MusicalSymbols::MusicalSymbolKievanEighthNoteStemDown => '𝇦',
            MusicalSymbols::MusicalSymbolKievanEighthNoteStemUp => '𝇧',
            MusicalSymbols::MusicalSymbolKievanFlatSign => '𝇨',
        }
    }
}

impl std::convert::TryFrom<char> for MusicalSymbols {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𝄀' => Ok(MusicalSymbols::MusicalSymbolSingleBarline),
            '𝄁' => Ok(MusicalSymbols::MusicalSymbolDoubleBarline),
            '𝄂' => Ok(MusicalSymbols::MusicalSymbolFinalBarline),
            '𝄃' => Ok(MusicalSymbols::MusicalSymbolReverseFinalBarline),
            '𝄄' => Ok(MusicalSymbols::MusicalSymbolDashedBarline),
            '𝄅' => Ok(MusicalSymbols::MusicalSymbolShortBarline),
            '𝄆' => Ok(MusicalSymbols::MusicalSymbolLeftRepeatSign),
            '𝄇' => Ok(MusicalSymbols::MusicalSymbolRightRepeatSign),
            '𝄈' => Ok(MusicalSymbols::MusicalSymbolRepeatDots),
            '𝄉' => Ok(MusicalSymbols::MusicalSymbolDalSegno),
            '𝄊' => Ok(MusicalSymbols::MusicalSymbolDaCapo),
            '𝄋' => Ok(MusicalSymbols::MusicalSymbolSegno),
            '𝄌' => Ok(MusicalSymbols::MusicalSymbolCoda),
            '𝄍' => Ok(MusicalSymbols::MusicalSymbolRepeatedFigureDash1),
            '𝄎' => Ok(MusicalSymbols::MusicalSymbolRepeatedFigureDash2),
            '𝄏' => Ok(MusicalSymbols::MusicalSymbolRepeatedFigureDash3),
            '𝄐' => Ok(MusicalSymbols::MusicalSymbolFermata),
            '𝄑' => Ok(MusicalSymbols::MusicalSymbolFermataBelow),
            '𝄒' => Ok(MusicalSymbols::MusicalSymbolBreathMark),
            '𝄓' => Ok(MusicalSymbols::MusicalSymbolCaesura),
            '𝄔' => Ok(MusicalSymbols::MusicalSymbolBrace),
            '𝄕' => Ok(MusicalSymbols::MusicalSymbolBracket),
            '𝄖' => Ok(MusicalSymbols::MusicalSymbolOneDashLineStaff),
            '𝄗' => Ok(MusicalSymbols::MusicalSymbolTwoDashLineStaff),
            '𝄘' => Ok(MusicalSymbols::MusicalSymbolThreeDashLineStaff),
            '𝄙' => Ok(MusicalSymbols::MusicalSymbolFourDashLineStaff),
            '𝄚' => Ok(MusicalSymbols::MusicalSymbolFiveDashLineStaff),
            '𝄛' => Ok(MusicalSymbols::MusicalSymbolSixDashLineStaff),
            '𝄜' => Ok(MusicalSymbols::MusicalSymbolSixDashStringFretboard),
            '𝄝' => Ok(MusicalSymbols::MusicalSymbolFourDashStringFretboard),
            '𝄞' => Ok(MusicalSymbols::MusicalSymbolGClef),
            '𝄟' => Ok(MusicalSymbols::MusicalSymbolGClefOttavaAlta),
            '𝄠' => Ok(MusicalSymbols::MusicalSymbolGClefOttavaBassa),
            '𝄡' => Ok(MusicalSymbols::MusicalSymbolCClef),
            '𝄢' => Ok(MusicalSymbols::MusicalSymbolFClef),
            '𝄣' => Ok(MusicalSymbols::MusicalSymbolFClefOttavaAlta),
            '𝄤' => Ok(MusicalSymbols::MusicalSymbolFClefOttavaBassa),
            '𝄥' => Ok(MusicalSymbols::MusicalSymbolDrumClefDash1),
            '𝄦' => Ok(MusicalSymbols::MusicalSymbolDrumClefDash2),
            '𝄩' => Ok(MusicalSymbols::MusicalSymbolMultipleMeasureRest),
            '𝄪' => Ok(MusicalSymbols::MusicalSymbolDoubleSharp),
            '𝄫' => Ok(MusicalSymbols::MusicalSymbolDoubleFlat),
            '𝄬' => Ok(MusicalSymbols::MusicalSymbolFlatUp),
            '𝄭' => Ok(MusicalSymbols::MusicalSymbolFlatDown),
            '𝄮' => Ok(MusicalSymbols::MusicalSymbolNaturalUp),
            '𝄯' => Ok(MusicalSymbols::MusicalSymbolNaturalDown),
            '𝄰' => Ok(MusicalSymbols::MusicalSymbolSharpUp),
            '𝄱' => Ok(MusicalSymbols::MusicalSymbolSharpDown),
            '𝄲' => Ok(MusicalSymbols::MusicalSymbolQuarterToneSharp),
            '𝄳' => Ok(MusicalSymbols::MusicalSymbolQuarterToneFlat),
            '𝄴' => Ok(MusicalSymbols::MusicalSymbolCommonTime),
            '𝄵' => Ok(MusicalSymbols::MusicalSymbolCutTime),
            '𝄶' => Ok(MusicalSymbols::MusicalSymbolOttavaAlta),
            '𝄷' => Ok(MusicalSymbols::MusicalSymbolOttavaBassa),
            '𝄸' => Ok(MusicalSymbols::MusicalSymbolQuindicesimaAlta),
            '𝄹' => Ok(MusicalSymbols::MusicalSymbolQuindicesimaBassa),
            '𝄺' => Ok(MusicalSymbols::MusicalSymbolMultiRest),
            '𝄻' => Ok(MusicalSymbols::MusicalSymbolWholeRest),
            '𝄼' => Ok(MusicalSymbols::MusicalSymbolHalfRest),
            '𝄽' => Ok(MusicalSymbols::MusicalSymbolQuarterRest),
            '𝄾' => Ok(MusicalSymbols::MusicalSymbolEighthRest),
            '𝄿' => Ok(MusicalSymbols::MusicalSymbolSixteenthRest),
            '𝅀' => Ok(MusicalSymbols::MusicalSymbolThirtyDashSecondRest),
            '𝅁' => Ok(MusicalSymbols::MusicalSymbolSixtyDashFourthRest),
            '𝅂' => Ok(MusicalSymbols::MusicalSymbolOneHundredTwentyDashEighthRest),
            '𝅃' => Ok(MusicalSymbols::MusicalSymbolXNotehead),
            '𝅄' => Ok(MusicalSymbols::MusicalSymbolPlusNotehead),
            '𝅅' => Ok(MusicalSymbols::MusicalSymbolCircleXNotehead),
            '𝅆' => Ok(MusicalSymbols::MusicalSymbolSquareNoteheadWhite),
            '𝅇' => Ok(MusicalSymbols::MusicalSymbolSquareNoteheadBlack),
            '𝅈' => Ok(MusicalSymbols::MusicalSymbolTriangleNoteheadUpWhite),
            '𝅉' => Ok(MusicalSymbols::MusicalSymbolTriangleNoteheadUpBlack),
            '𝅊' => Ok(MusicalSymbols::MusicalSymbolTriangleNoteheadLeftWhite),
            '𝅋' => Ok(MusicalSymbols::MusicalSymbolTriangleNoteheadLeftBlack),
            '𝅌' => Ok(MusicalSymbols::MusicalSymbolTriangleNoteheadRightWhite),
            '𝅍' => Ok(MusicalSymbols::MusicalSymbolTriangleNoteheadRightBlack),
            '𝅎' => Ok(MusicalSymbols::MusicalSymbolTriangleNoteheadDownWhite),
            '𝅏' => Ok(MusicalSymbols::MusicalSymbolTriangleNoteheadDownBlack),
            '𝅐' => Ok(MusicalSymbols::MusicalSymbolTriangleNoteheadUpRightWhite),
            '𝅑' => Ok(MusicalSymbols::MusicalSymbolTriangleNoteheadUpRightBlack),
            '𝅒' => Ok(MusicalSymbols::MusicalSymbolMoonNoteheadWhite),
            '𝅓' => Ok(MusicalSymbols::MusicalSymbolMoonNoteheadBlack),
            '𝅔' => Ok(MusicalSymbols::MusicalSymbolTriangleDashRoundNoteheadDownWhite),
            '𝅕' => Ok(MusicalSymbols::MusicalSymbolTriangleDashRoundNoteheadDownBlack),
            '𝅖' => Ok(MusicalSymbols::MusicalSymbolParenthesisNotehead),
            '𝅗' => Ok(MusicalSymbols::MusicalSymbolVoidNotehead),
            '𝅘' => Ok(MusicalSymbols::MusicalSymbolNoteheadBlack),
            '𝅙' => Ok(MusicalSymbols::MusicalSymbolNullNotehead),
            '𝅚' => Ok(MusicalSymbols::MusicalSymbolClusterNoteheadWhite),
            '𝅛' => Ok(MusicalSymbols::MusicalSymbolClusterNoteheadBlack),
            '𝅜' => Ok(MusicalSymbols::MusicalSymbolBreve),
            '𝅝' => Ok(MusicalSymbols::MusicalSymbolWholeNote),
            '𝅗𝅥' => Ok(MusicalSymbols::MusicalSymbolHalfNote),
            '𝅘𝅥' => Ok(MusicalSymbols::MusicalSymbolQuarterNote),
            '𝅘𝅥𝅮' => Ok(MusicalSymbols::MusicalSymbolEighthNote),
            '𝅘𝅥𝅯' => Ok(MusicalSymbols::MusicalSymbolSixteenthNote),
            '𝅘𝅥𝅰' => Ok(MusicalSymbols::MusicalSymbolThirtyDashSecondNote),
            '𝅘𝅥𝅱' => Ok(MusicalSymbols::MusicalSymbolSixtyDashFourthNote),
            '𝅘𝅥𝅲' => Ok(MusicalSymbols::MusicalSymbolOneHundredTwentyDashEighthNote),
            '𝅥' => Ok(MusicalSymbols::MusicalSymbolCombiningStem),
            '𝅦' => Ok(MusicalSymbols::MusicalSymbolCombiningSprechgesangStem),
            '𝅧' => Ok(MusicalSymbols::MusicalSymbolCombiningTremoloDash1),
            '𝅨' => Ok(MusicalSymbols::MusicalSymbolCombiningTremoloDash2),
            '𝅩' => Ok(MusicalSymbols::MusicalSymbolCombiningTremoloDash3),
            '𝅪' => Ok(MusicalSymbols::MusicalSymbolFingeredTremoloDash1),
            '𝅫' => Ok(MusicalSymbols::MusicalSymbolFingeredTremoloDash2),
            '𝅬' => Ok(MusicalSymbols::MusicalSymbolFingeredTremoloDash3),
            '𝅭' => Ok(MusicalSymbols::MusicalSymbolCombiningAugmentationDot),
            '𝅮' => Ok(MusicalSymbols::MusicalSymbolCombiningFlagDash1),
            '𝅯' => Ok(MusicalSymbols::MusicalSymbolCombiningFlagDash2),
            '𝅰' => Ok(MusicalSymbols::MusicalSymbolCombiningFlagDash3),
            '𝅱' => Ok(MusicalSymbols::MusicalSymbolCombiningFlagDash4),
            '𝅲' => Ok(MusicalSymbols::MusicalSymbolCombiningFlagDash5),
            '𝅳' => Ok(MusicalSymbols::MusicalSymbolBeginBeam),
            '𝅴' => Ok(MusicalSymbols::MusicalSymbolEndBeam),
            '𝅵' => Ok(MusicalSymbols::MusicalSymbolBeginTie),
            '𝅶' => Ok(MusicalSymbols::MusicalSymbolEndTie),
            '𝅷' => Ok(MusicalSymbols::MusicalSymbolBeginSlur),
            '𝅸' => Ok(MusicalSymbols::MusicalSymbolEndSlur),
            '𝅹' => Ok(MusicalSymbols::MusicalSymbolBeginPhrase),
            '𝅺' => Ok(MusicalSymbols::MusicalSymbolEndPhrase),
            '𝅻' => Ok(MusicalSymbols::MusicalSymbolCombiningAccent),
            '𝅼' => Ok(MusicalSymbols::MusicalSymbolCombiningStaccato),
            '𝅽' => Ok(MusicalSymbols::MusicalSymbolCombiningTenuto),
            '𝅾' => Ok(MusicalSymbols::MusicalSymbolCombiningStaccatissimo),
            '𝅿' => Ok(MusicalSymbols::MusicalSymbolCombiningMarcato),
            '𝆀' => Ok(MusicalSymbols::MusicalSymbolCombiningMarcatoDashStaccato),
            '𝆁' => Ok(MusicalSymbols::MusicalSymbolCombiningAccentDashStaccato),
            '𝆂' => Ok(MusicalSymbols::MusicalSymbolCombiningLoure),
            '𝆃' => Ok(MusicalSymbols::MusicalSymbolArpeggiatoUp),
            '𝆄' => Ok(MusicalSymbols::MusicalSymbolArpeggiatoDown),
            '𝆅' => Ok(MusicalSymbols::MusicalSymbolCombiningDoit),
            '𝆆' => Ok(MusicalSymbols::MusicalSymbolCombiningRip),
            '𝆇' => Ok(MusicalSymbols::MusicalSymbolCombiningFlip),
            '𝆈' => Ok(MusicalSymbols::MusicalSymbolCombiningSmear),
            '𝆉' => Ok(MusicalSymbols::MusicalSymbolCombiningBend),
            '𝆊' => Ok(MusicalSymbols::MusicalSymbolCombiningDoubleTongue),
            '𝆋' => Ok(MusicalSymbols::MusicalSymbolCombiningTripleTongue),
            '𝆌' => Ok(MusicalSymbols::MusicalSymbolRinforzando),
            '𝆍' => Ok(MusicalSymbols::MusicalSymbolSubito),
            '𝆎' => Ok(MusicalSymbols::MusicalSymbolZ),
            '𝆏' => Ok(MusicalSymbols::MusicalSymbolPiano),
            '𝆐' => Ok(MusicalSymbols::MusicalSymbolMezzo),
            '𝆑' => Ok(MusicalSymbols::MusicalSymbolForte),
            '𝆒' => Ok(MusicalSymbols::MusicalSymbolCrescendo),
            '𝆓' => Ok(MusicalSymbols::MusicalSymbolDecrescendo),
            '𝆔' => Ok(MusicalSymbols::MusicalSymbolGraceNoteSlash),
            '𝆕' => Ok(MusicalSymbols::MusicalSymbolGraceNoteNoSlash),
            '𝆖' => Ok(MusicalSymbols::MusicalSymbolTr),
            '𝆗' => Ok(MusicalSymbols::MusicalSymbolTurn),
            '𝆘' => Ok(MusicalSymbols::MusicalSymbolInvertedTurn),
            '𝆙' => Ok(MusicalSymbols::MusicalSymbolTurnSlash),
            '𝆚' => Ok(MusicalSymbols::MusicalSymbolTurnUp),
            '𝆛' => Ok(MusicalSymbols::MusicalSymbolOrnamentStrokeDash1),
            '𝆜' => Ok(MusicalSymbols::MusicalSymbolOrnamentStrokeDash2),
            '𝆝' => Ok(MusicalSymbols::MusicalSymbolOrnamentStrokeDash3),
            '𝆞' => Ok(MusicalSymbols::MusicalSymbolOrnamentStrokeDash4),
            '𝆟' => Ok(MusicalSymbols::MusicalSymbolOrnamentStrokeDash5),
            '𝆠' => Ok(MusicalSymbols::MusicalSymbolOrnamentStrokeDash6),
            '𝆡' => Ok(MusicalSymbols::MusicalSymbolOrnamentStrokeDash7),
            '𝆢' => Ok(MusicalSymbols::MusicalSymbolOrnamentStrokeDash8),
            '𝆣' => Ok(MusicalSymbols::MusicalSymbolOrnamentStrokeDash9),
            '𝆤' => Ok(MusicalSymbols::MusicalSymbolOrnamentStrokeDash10),
            '𝆥' => Ok(MusicalSymbols::MusicalSymbolOrnamentStrokeDash11),
            '𝆦' => Ok(MusicalSymbols::MusicalSymbolHauptstimme),
            '𝆧' => Ok(MusicalSymbols::MusicalSymbolNebenstimme),
            '𝆨' => Ok(MusicalSymbols::MusicalSymbolEndOfStimme),
            '𝆩' => Ok(MusicalSymbols::MusicalSymbolDegreeSlash),
            '𝆪' => Ok(MusicalSymbols::MusicalSymbolCombiningDownBow),
            '𝆫' => Ok(MusicalSymbols::MusicalSymbolCombiningUpBow),
            '𝆬' => Ok(MusicalSymbols::MusicalSymbolCombiningHarmonic),
            '𝆭' => Ok(MusicalSymbols::MusicalSymbolCombiningSnapPizzicato),
            '𝆮' => Ok(MusicalSymbols::MusicalSymbolPedalMark),
            '𝆯' => Ok(MusicalSymbols::MusicalSymbolPedalUpMark),
            '𝆰' => Ok(MusicalSymbols::MusicalSymbolHalfPedalMark),
            '𝆱' => Ok(MusicalSymbols::MusicalSymbolGlissandoUp),
            '𝆲' => Ok(MusicalSymbols::MusicalSymbolGlissandoDown),
            '𝆳' => Ok(MusicalSymbols::MusicalSymbolWithFingernails),
            '𝆴' => Ok(MusicalSymbols::MusicalSymbolDamp),
            '𝆵' => Ok(MusicalSymbols::MusicalSymbolDampAll),
            '𝆶' => Ok(MusicalSymbols::MusicalSymbolMaxima),
            '𝆷' => Ok(MusicalSymbols::MusicalSymbolLonga),
            '𝆸' => Ok(MusicalSymbols::MusicalSymbolBrevis),
            '𝆹' => Ok(MusicalSymbols::MusicalSymbolSemibrevisWhite),
            '𝆺' => Ok(MusicalSymbols::MusicalSymbolSemibrevisBlack),
            '𝆹𝅥' => Ok(MusicalSymbols::MusicalSymbolMinima),
            '𝆺𝅥' => Ok(MusicalSymbols::MusicalSymbolMinimaBlack),
            '𝆹𝅥𝅮' => Ok(MusicalSymbols::MusicalSymbolSemiminimaWhite),
            '𝆺𝅥𝅮' => Ok(MusicalSymbols::MusicalSymbolSemiminimaBlack),
            '𝆹𝅥𝅯' => Ok(MusicalSymbols::MusicalSymbolFusaWhite),
            '𝆺𝅥𝅯' => Ok(MusicalSymbols::MusicalSymbolFusaBlack),
            '𝇁' => Ok(MusicalSymbols::MusicalSymbolLongaPerfectaRest),
            '𝇂' => Ok(MusicalSymbols::MusicalSymbolLongaImperfectaRest),
            '𝇃' => Ok(MusicalSymbols::MusicalSymbolBrevisRest),
            '𝇄' => Ok(MusicalSymbols::MusicalSymbolSemibrevisRest),
            '𝇅' => Ok(MusicalSymbols::MusicalSymbolMinimaRest),
            '𝇆' => Ok(MusicalSymbols::MusicalSymbolSemiminimaRest),
            '𝇇' => Ok(MusicalSymbols::MusicalSymbolTempusPerfectumCumProlationePerfecta),
            '𝇈' => Ok(MusicalSymbols::MusicalSymbolTempusPerfectumCumProlationeImperfecta),
            '𝇉' => Ok(MusicalSymbols::MusicalSymbolTempusPerfectumCumProlationePerfectaDiminutionDash1),
            '𝇊' => Ok(MusicalSymbols::MusicalSymbolTempusImperfectumCumProlationePerfecta),
            '𝇋' => Ok(MusicalSymbols::MusicalSymbolTempusImperfectumCumProlationeImperfecta),
            '𝇌' => Ok(MusicalSymbols::MusicalSymbolTempusImperfectumCumProlationeImperfectaDiminutionDash1),
            '𝇍' => Ok(MusicalSymbols::MusicalSymbolTempusImperfectumCumProlationeImperfectaDiminutionDash2),
            '𝇎' => Ok(MusicalSymbols::MusicalSymbolTempusImperfectumCumProlationeImperfectaDiminutionDash3),
            '𝇏' => Ok(MusicalSymbols::MusicalSymbolCroix),
            '𝇐' => Ok(MusicalSymbols::MusicalSymbolGregorianCClef),
            '𝇑' => Ok(MusicalSymbols::MusicalSymbolGregorianFClef),
            '𝇒' => Ok(MusicalSymbols::MusicalSymbolSquareB),
            '𝇓' => Ok(MusicalSymbols::MusicalSymbolVirga),
            '𝇔' => Ok(MusicalSymbols::MusicalSymbolPodatus),
            '𝇕' => Ok(MusicalSymbols::MusicalSymbolClivis),
            '𝇖' => Ok(MusicalSymbols::MusicalSymbolScandicus),
            '𝇗' => Ok(MusicalSymbols::MusicalSymbolClimacus),
            '𝇘' => Ok(MusicalSymbols::MusicalSymbolTorculus),
            '𝇙' => Ok(MusicalSymbols::MusicalSymbolPorrectus),
            '𝇚' => Ok(MusicalSymbols::MusicalSymbolPorrectusFlexus),
            '𝇛' => Ok(MusicalSymbols::MusicalSymbolScandicusFlexus),
            '𝇜' => Ok(MusicalSymbols::MusicalSymbolTorculusResupinus),
            '𝇝' => Ok(MusicalSymbols::MusicalSymbolPesSubpunctis),
            '𝇞' => Ok(MusicalSymbols::MusicalSymbolKievanCClef),
            '𝇟' => Ok(MusicalSymbols::MusicalSymbolKievanEndOfPiece),
            '𝇠' => Ok(MusicalSymbols::MusicalSymbolKievanFinalNote),
            '𝇡' => Ok(MusicalSymbols::MusicalSymbolKievanRecitativeMark),
            '𝇢' => Ok(MusicalSymbols::MusicalSymbolKievanWholeNote),
            '𝇣' => Ok(MusicalSymbols::MusicalSymbolKievanHalfNote),
            '𝇤' => Ok(MusicalSymbols::MusicalSymbolKievanQuarterNoteStemDown),
            '𝇥' => Ok(MusicalSymbols::MusicalSymbolKievanQuarterNoteStemUp),
            '𝇦' => Ok(MusicalSymbols::MusicalSymbolKievanEighthNoteStemDown),
            '𝇧' => Ok(MusicalSymbols::MusicalSymbolKievanEighthNoteStemUp),
            '𝇨' => Ok(MusicalSymbols::MusicalSymbolKievanFlatSign),
            _ => Err(()),
        }
    }
}

impl Into<u32> for MusicalSymbols {
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

impl std::convert::TryFrom<u32> for MusicalSymbols {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for MusicalSymbols {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl MusicalSymbols {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        MusicalSymbols::MusicalSymbolSingleBarline
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("MusicalSymbols{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
