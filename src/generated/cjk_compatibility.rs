
/// An enum to represent all characters in the CJKCompatibility block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum CJKCompatibility {
    /// \u{3300}: '㌀'
    SquareApaato,
    /// \u{3301}: '㌁'
    SquareAruhua,
    /// \u{3302}: '㌂'
    SquareAnpea,
    /// \u{3303}: '㌃'
    SquareAaru,
    /// \u{3304}: '㌄'
    SquareIningu,
    /// \u{3305}: '㌅'
    SquareInti,
    /// \u{3306}: '㌆'
    SquareUon,
    /// \u{3307}: '㌇'
    SquareEsukuudo,
    /// \u{3308}: '㌈'
    SquareEekaa,
    /// \u{3309}: '㌉'
    SquareOnsu,
    /// \u{330a}: '㌊'
    SquareOomu,
    /// \u{330b}: '㌋'
    SquareKairi,
    /// \u{330c}: '㌌'
    SquareKaratto,
    /// \u{330d}: '㌍'
    SquareKarorii,
    /// \u{330e}: '㌎'
    SquareGaron,
    /// \u{330f}: '㌏'
    SquareGanma,
    /// \u{3310}: '㌐'
    SquareGiga,
    /// \u{3311}: '㌑'
    SquareGinii,
    /// \u{3312}: '㌒'
    SquareKyurii,
    /// \u{3313}: '㌓'
    SquareGirudaa,
    /// \u{3314}: '㌔'
    SquareKiro,
    /// \u{3315}: '㌕'
    SquareKiroguramu,
    /// \u{3316}: '㌖'
    SquareKiromeetoru,
    /// \u{3317}: '㌗'
    SquareKirowatto,
    /// \u{3318}: '㌘'
    SquareGuramu,
    /// \u{3319}: '㌙'
    SquareGuramuton,
    /// \u{331a}: '㌚'
    SquareKuruzeiro,
    /// \u{331b}: '㌛'
    SquareKuroone,
    /// \u{331c}: '㌜'
    SquareKeesu,
    /// \u{331d}: '㌝'
    SquareKoruna,
    /// \u{331e}: '㌞'
    SquareKoopo,
    /// \u{331f}: '㌟'
    SquareSaikuru,
    /// \u{3320}: '㌠'
    SquareSantiimu,
    /// \u{3321}: '㌡'
    SquareSiringu,
    /// \u{3322}: '㌢'
    SquareSenti,
    /// \u{3323}: '㌣'
    SquareSento,
    /// \u{3324}: '㌤'
    SquareDaasu,
    /// \u{3325}: '㌥'
    SquareDesi,
    /// \u{3326}: '㌦'
    SquareDoru,
    /// \u{3327}: '㌧'
    SquareTon,
    /// \u{3328}: '㌨'
    SquareNano,
    /// \u{3329}: '㌩'
    SquareNotto,
    /// \u{332a}: '㌪'
    SquareHaitu,
    /// \u{332b}: '㌫'
    SquarePaasento,
    /// \u{332c}: '㌬'
    SquarePaatu,
    /// \u{332d}: '㌭'
    SquareBaareru,
    /// \u{332e}: '㌮'
    SquarePiasutoru,
    /// \u{332f}: '㌯'
    SquarePikuru,
    /// \u{3330}: '㌰'
    SquarePiko,
    /// \u{3331}: '㌱'
    SquareBiru,
    /// \u{3332}: '㌲'
    SquareHuaraddo,
    /// \u{3333}: '㌳'
    SquareHuiito,
    /// \u{3334}: '㌴'
    SquareBussyeru,
    /// \u{3335}: '㌵'
    SquareHuran,
    /// \u{3336}: '㌶'
    SquareHekutaaru,
    /// \u{3337}: '㌷'
    SquarePeso,
    /// \u{3338}: '㌸'
    SquarePenihi,
    /// \u{3339}: '㌹'
    SquareHerutu,
    /// \u{333a}: '㌺'
    SquarePensu,
    /// \u{333b}: '㌻'
    SquarePeezi,
    /// \u{333c}: '㌼'
    SquareBeeta,
    /// \u{333d}: '㌽'
    SquarePointo,
    /// \u{333e}: '㌾'
    SquareBoruto,
    /// \u{333f}: '㌿'
    SquareHon,
    /// \u{3340}: '㍀'
    SquarePondo,
    /// \u{3341}: '㍁'
    SquareHooru,
    /// \u{3342}: '㍂'
    SquareHoon,
    /// \u{3343}: '㍃'
    SquareMaikuro,
    /// \u{3344}: '㍄'
    SquareMairu,
    /// \u{3345}: '㍅'
    SquareMahha,
    /// \u{3346}: '㍆'
    SquareMaruku,
    /// \u{3347}: '㍇'
    SquareMansyon,
    /// \u{3348}: '㍈'
    SquareMikuron,
    /// \u{3349}: '㍉'
    SquareMiri,
    /// \u{334a}: '㍊'
    SquareMiribaaru,
    /// \u{334b}: '㍋'
    SquareMega,
    /// \u{334c}: '㍌'
    SquareMegaton,
    /// \u{334d}: '㍍'
    SquareMeetoru,
    /// \u{334e}: '㍎'
    SquareYaado,
    /// \u{334f}: '㍏'
    SquareYaaru,
    /// \u{3350}: '㍐'
    SquareYuan,
    /// \u{3351}: '㍑'
    SquareRittoru,
    /// \u{3352}: '㍒'
    SquareRira,
    /// \u{3353}: '㍓'
    SquareRupii,
    /// \u{3354}: '㍔'
    SquareRuuburu,
    /// \u{3355}: '㍕'
    SquareRemu,
    /// \u{3356}: '㍖'
    SquareRentogen,
    /// \u{3357}: '㍗'
    SquareWatto,
    /// \u{3358}: '㍘'
    IdeographicTelegraphSymbolForHourZero,
    /// \u{3359}: '㍙'
    IdeographicTelegraphSymbolForHourOne,
    /// \u{335a}: '㍚'
    IdeographicTelegraphSymbolForHourTwo,
    /// \u{335b}: '㍛'
    IdeographicTelegraphSymbolForHourThree,
    /// \u{335c}: '㍜'
    IdeographicTelegraphSymbolForHourFour,
    /// \u{335d}: '㍝'
    IdeographicTelegraphSymbolForHourFive,
    /// \u{335e}: '㍞'
    IdeographicTelegraphSymbolForHourSix,
    /// \u{335f}: '㍟'
    IdeographicTelegraphSymbolForHourSeven,
    /// \u{3360}: '㍠'
    IdeographicTelegraphSymbolForHourEight,
    /// \u{3361}: '㍡'
    IdeographicTelegraphSymbolForHourNine,
    /// \u{3362}: '㍢'
    IdeographicTelegraphSymbolForHourTen,
    /// \u{3363}: '㍣'
    IdeographicTelegraphSymbolForHourEleven,
    /// \u{3364}: '㍤'
    IdeographicTelegraphSymbolForHourTwelve,
    /// \u{3365}: '㍥'
    IdeographicTelegraphSymbolForHourThirteen,
    /// \u{3366}: '㍦'
    IdeographicTelegraphSymbolForHourFourteen,
    /// \u{3367}: '㍧'
    IdeographicTelegraphSymbolForHourFifteen,
    /// \u{3368}: '㍨'
    IdeographicTelegraphSymbolForHourSixteen,
    /// \u{3369}: '㍩'
    IdeographicTelegraphSymbolForHourSeventeen,
    /// \u{336a}: '㍪'
    IdeographicTelegraphSymbolForHourEighteen,
    /// \u{336b}: '㍫'
    IdeographicTelegraphSymbolForHourNineteen,
    /// \u{336c}: '㍬'
    IdeographicTelegraphSymbolForHourTwenty,
    /// \u{336d}: '㍭'
    IdeographicTelegraphSymbolForHourTwentyDashOne,
    /// \u{336e}: '㍮'
    IdeographicTelegraphSymbolForHourTwentyDashTwo,
    /// \u{336f}: '㍯'
    IdeographicTelegraphSymbolForHourTwentyDashThree,
    /// \u{3370}: '㍰'
    IdeographicTelegraphSymbolForHourTwentyDashFour,
    /// \u{3371}: '㍱'
    SquareHpa,
    /// \u{3372}: '㍲'
    SquareDa,
    /// \u{3373}: '㍳'
    SquareAu,
    /// \u{3374}: '㍴'
    SquareBar,
    /// \u{3375}: '㍵'
    SquareOv,
    /// \u{3376}: '㍶'
    SquarePc,
    /// \u{3377}: '㍷'
    SquareDm,
    /// \u{3378}: '㍸'
    SquareDmSquared,
    /// \u{3379}: '㍹'
    SquareDmCubed,
    /// \u{337a}: '㍺'
    SquareIu,
    /// \u{337b}: '㍻'
    SquareEraNameHeisei,
    /// \u{337c}: '㍼'
    SquareEraNameSyouwa,
    /// \u{337d}: '㍽'
    SquareEraNameTaisyou,
    /// \u{337e}: '㍾'
    SquareEraNameMeizi,
    /// \u{337f}: '㍿'
    SquareCorporation,
    /// \u{3380}: '㎀'
    SquarePaAmps,
    /// \u{3381}: '㎁'
    SquareNa,
    /// \u{3382}: '㎂'
    SquareMuA,
    /// \u{3383}: '㎃'
    SquareMa,
    /// \u{3384}: '㎄'
    SquareKa,
    /// \u{3385}: '㎅'
    SquareKb,
    /// \u{3386}: '㎆'
    SquareMb,
    /// \u{3387}: '㎇'
    SquareGb,
    /// \u{3388}: '㎈'
    SquareCal,
    /// \u{3389}: '㎉'
    SquareKcal,
    /// \u{338a}: '㎊'
    SquarePf,
    /// \u{338b}: '㎋'
    SquareNf,
    /// \u{338c}: '㎌'
    SquareMuF,
    /// \u{338d}: '㎍'
    SquareMuG,
    /// \u{338e}: '㎎'
    SquareMg,
    /// \u{338f}: '㎏'
    SquareKg,
    /// \u{3390}: '㎐'
    SquareHz,
    /// \u{3391}: '㎑'
    SquareKhz,
    /// \u{3392}: '㎒'
    SquareMhz,
    /// \u{3393}: '㎓'
    SquareGhz,
    /// \u{3394}: '㎔'
    SquareThz,
    /// \u{3395}: '㎕'
    SquareMuL,
    /// \u{3396}: '㎖'
    SquareMl,
    /// \u{3397}: '㎗'
    SquareDl,
    /// \u{3398}: '㎘'
    SquareKl,
    /// \u{3399}: '㎙'
    SquareFm,
    /// \u{339a}: '㎚'
    SquareNm,
    /// \u{339b}: '㎛'
    SquareMuM,
    /// \u{339c}: '㎜'
    SquareMm,
    /// \u{339d}: '㎝'
    SquareCm,
    /// \u{339e}: '㎞'
    SquareKm,
    /// \u{339f}: '㎟'
    SquareMmSquared,
    /// \u{33a0}: '㎠'
    SquareCmSquared,
    /// \u{33a1}: '㎡'
    SquareMSquared,
    /// \u{33a2}: '㎢'
    SquareKmSquared,
    /// \u{33a3}: '㎣'
    SquareMmCubed,
    /// \u{33a4}: '㎤'
    SquareCmCubed,
    /// \u{33a5}: '㎥'
    SquareMCubed,
    /// \u{33a6}: '㎦'
    SquareKmCubed,
    /// \u{33a7}: '㎧'
    SquareMOverS,
    /// \u{33a8}: '㎨'
    SquareMOverSSquared,
    /// \u{33a9}: '㎩'
    SquarePa,
    /// \u{33aa}: '㎪'
    SquareKpa,
    /// \u{33ab}: '㎫'
    SquareMpa,
    /// \u{33ac}: '㎬'
    SquareGpa,
    /// \u{33ad}: '㎭'
    SquareRad,
    /// \u{33ae}: '㎮'
    SquareRadOverS,
    /// \u{33af}: '㎯'
    SquareRadOverSSquared,
    /// \u{33b0}: '㎰'
    SquarePs,
    /// \u{33b1}: '㎱'
    SquareNs,
    /// \u{33b2}: '㎲'
    SquareMuS,
    /// \u{33b3}: '㎳'
    SquareMs,
    /// \u{33b4}: '㎴'
    SquarePv,
    /// \u{33b5}: '㎵'
    SquareNv,
    /// \u{33b6}: '㎶'
    SquareMuV,
    /// \u{33b7}: '㎷'
    SquareMv,
    /// \u{33b8}: '㎸'
    SquareKv,
    /// \u{33b9}: '㎹'
    SquareMvMega,
    /// \u{33ba}: '㎺'
    SquarePw,
    /// \u{33bb}: '㎻'
    SquareNw,
    /// \u{33bc}: '㎼'
    SquareMuW,
    /// \u{33bd}: '㎽'
    SquareMw,
    /// \u{33be}: '㎾'
    SquareKw,
    /// \u{33bf}: '㎿'
    SquareMwMega,
    /// \u{33c0}: '㏀'
    SquareKOhm,
    /// \u{33c1}: '㏁'
    SquareMOhm,
    /// \u{33c2}: '㏂'
    SquareAm,
    /// \u{33c3}: '㏃'
    SquareBq,
    /// \u{33c4}: '㏄'
    SquareCc,
    /// \u{33c5}: '㏅'
    SquareCd,
    /// \u{33c6}: '㏆'
    SquareCOverKg,
    /// \u{33c7}: '㏇'
    SquareCo,
    /// \u{33c8}: '㏈'
    SquareDb,
    /// \u{33c9}: '㏉'
    SquareGy,
    /// \u{33ca}: '㏊'
    SquareHa,
    /// \u{33cb}: '㏋'
    SquareHp,
    /// \u{33cc}: '㏌'
    SquareIn,
    /// \u{33cd}: '㏍'
    SquareKk,
    /// \u{33ce}: '㏎'
    SquareKmCapital,
    /// \u{33cf}: '㏏'
    SquareKt,
    /// \u{33d0}: '㏐'
    SquareLm,
    /// \u{33d1}: '㏑'
    SquareLn,
    /// \u{33d2}: '㏒'
    SquareLog,
    /// \u{33d3}: '㏓'
    SquareLx,
    /// \u{33d4}: '㏔'
    SquareMbSmall,
    /// \u{33d5}: '㏕'
    SquareMil,
    /// \u{33d6}: '㏖'
    SquareMol,
    /// \u{33d7}: '㏗'
    SquarePh,
    /// \u{33d8}: '㏘'
    SquarePm,
    /// \u{33d9}: '㏙'
    SquarePpm,
    /// \u{33da}: '㏚'
    SquarePr,
    /// \u{33db}: '㏛'
    SquareSr,
    /// \u{33dc}: '㏜'
    SquareSv,
    /// \u{33dd}: '㏝'
    SquareWb,
    /// \u{33de}: '㏞'
    SquareVOverM,
    /// \u{33df}: '㏟'
    SquareAOverM,
    /// \u{33e0}: '㏠'
    IdeographicTelegraphSymbolForDayOne,
    /// \u{33e1}: '㏡'
    IdeographicTelegraphSymbolForDayTwo,
    /// \u{33e2}: '㏢'
    IdeographicTelegraphSymbolForDayThree,
    /// \u{33e3}: '㏣'
    IdeographicTelegraphSymbolForDayFour,
    /// \u{33e4}: '㏤'
    IdeographicTelegraphSymbolForDayFive,
    /// \u{33e5}: '㏥'
    IdeographicTelegraphSymbolForDaySix,
    /// \u{33e6}: '㏦'
    IdeographicTelegraphSymbolForDaySeven,
    /// \u{33e7}: '㏧'
    IdeographicTelegraphSymbolForDayEight,
    /// \u{33e8}: '㏨'
    IdeographicTelegraphSymbolForDayNine,
    /// \u{33e9}: '㏩'
    IdeographicTelegraphSymbolForDayTen,
    /// \u{33ea}: '㏪'
    IdeographicTelegraphSymbolForDayEleven,
    /// \u{33eb}: '㏫'
    IdeographicTelegraphSymbolForDayTwelve,
    /// \u{33ec}: '㏬'
    IdeographicTelegraphSymbolForDayThirteen,
    /// \u{33ed}: '㏭'
    IdeographicTelegraphSymbolForDayFourteen,
    /// \u{33ee}: '㏮'
    IdeographicTelegraphSymbolForDayFifteen,
    /// \u{33ef}: '㏯'
    IdeographicTelegraphSymbolForDaySixteen,
    /// \u{33f0}: '㏰'
    IdeographicTelegraphSymbolForDaySeventeen,
    /// \u{33f1}: '㏱'
    IdeographicTelegraphSymbolForDayEighteen,
    /// \u{33f2}: '㏲'
    IdeographicTelegraphSymbolForDayNineteen,
    /// \u{33f3}: '㏳'
    IdeographicTelegraphSymbolForDayTwenty,
    /// \u{33f4}: '㏴'
    IdeographicTelegraphSymbolForDayTwentyDashOne,
    /// \u{33f5}: '㏵'
    IdeographicTelegraphSymbolForDayTwentyDashTwo,
    /// \u{33f6}: '㏶'
    IdeographicTelegraphSymbolForDayTwentyDashThree,
    /// \u{33f7}: '㏷'
    IdeographicTelegraphSymbolForDayTwentyDashFour,
    /// \u{33f8}: '㏸'
    IdeographicTelegraphSymbolForDayTwentyDashFive,
    /// \u{33f9}: '㏹'
    IdeographicTelegraphSymbolForDayTwentyDashSix,
    /// \u{33fa}: '㏺'
    IdeographicTelegraphSymbolForDayTwentyDashSeven,
    /// \u{33fb}: '㏻'
    IdeographicTelegraphSymbolForDayTwentyDashEight,
    /// \u{33fc}: '㏼'
    IdeographicTelegraphSymbolForDayTwentyDashNine,
    /// \u{33fd}: '㏽'
    IdeographicTelegraphSymbolForDayThirty,
    /// \u{33fe}: '㏾'
    IdeographicTelegraphSymbolForDayThirtyDashOne,
}

impl Into<char> for CJKCompatibility {
    fn into(self) -> char {
        match self {
            CJKCompatibility::SquareApaato => '㌀',
            CJKCompatibility::SquareAruhua => '㌁',
            CJKCompatibility::SquareAnpea => '㌂',
            CJKCompatibility::SquareAaru => '㌃',
            CJKCompatibility::SquareIningu => '㌄',
            CJKCompatibility::SquareInti => '㌅',
            CJKCompatibility::SquareUon => '㌆',
            CJKCompatibility::SquareEsukuudo => '㌇',
            CJKCompatibility::SquareEekaa => '㌈',
            CJKCompatibility::SquareOnsu => '㌉',
            CJKCompatibility::SquareOomu => '㌊',
            CJKCompatibility::SquareKairi => '㌋',
            CJKCompatibility::SquareKaratto => '㌌',
            CJKCompatibility::SquareKarorii => '㌍',
            CJKCompatibility::SquareGaron => '㌎',
            CJKCompatibility::SquareGanma => '㌏',
            CJKCompatibility::SquareGiga => '㌐',
            CJKCompatibility::SquareGinii => '㌑',
            CJKCompatibility::SquareKyurii => '㌒',
            CJKCompatibility::SquareGirudaa => '㌓',
            CJKCompatibility::SquareKiro => '㌔',
            CJKCompatibility::SquareKiroguramu => '㌕',
            CJKCompatibility::SquareKiromeetoru => '㌖',
            CJKCompatibility::SquareKirowatto => '㌗',
            CJKCompatibility::SquareGuramu => '㌘',
            CJKCompatibility::SquareGuramuton => '㌙',
            CJKCompatibility::SquareKuruzeiro => '㌚',
            CJKCompatibility::SquareKuroone => '㌛',
            CJKCompatibility::SquareKeesu => '㌜',
            CJKCompatibility::SquareKoruna => '㌝',
            CJKCompatibility::SquareKoopo => '㌞',
            CJKCompatibility::SquareSaikuru => '㌟',
            CJKCompatibility::SquareSantiimu => '㌠',
            CJKCompatibility::SquareSiringu => '㌡',
            CJKCompatibility::SquareSenti => '㌢',
            CJKCompatibility::SquareSento => '㌣',
            CJKCompatibility::SquareDaasu => '㌤',
            CJKCompatibility::SquareDesi => '㌥',
            CJKCompatibility::SquareDoru => '㌦',
            CJKCompatibility::SquareTon => '㌧',
            CJKCompatibility::SquareNano => '㌨',
            CJKCompatibility::SquareNotto => '㌩',
            CJKCompatibility::SquareHaitu => '㌪',
            CJKCompatibility::SquarePaasento => '㌫',
            CJKCompatibility::SquarePaatu => '㌬',
            CJKCompatibility::SquareBaareru => '㌭',
            CJKCompatibility::SquarePiasutoru => '㌮',
            CJKCompatibility::SquarePikuru => '㌯',
            CJKCompatibility::SquarePiko => '㌰',
            CJKCompatibility::SquareBiru => '㌱',
            CJKCompatibility::SquareHuaraddo => '㌲',
            CJKCompatibility::SquareHuiito => '㌳',
            CJKCompatibility::SquareBussyeru => '㌴',
            CJKCompatibility::SquareHuran => '㌵',
            CJKCompatibility::SquareHekutaaru => '㌶',
            CJKCompatibility::SquarePeso => '㌷',
            CJKCompatibility::SquarePenihi => '㌸',
            CJKCompatibility::SquareHerutu => '㌹',
            CJKCompatibility::SquarePensu => '㌺',
            CJKCompatibility::SquarePeezi => '㌻',
            CJKCompatibility::SquareBeeta => '㌼',
            CJKCompatibility::SquarePointo => '㌽',
            CJKCompatibility::SquareBoruto => '㌾',
            CJKCompatibility::SquareHon => '㌿',
            CJKCompatibility::SquarePondo => '㍀',
            CJKCompatibility::SquareHooru => '㍁',
            CJKCompatibility::SquareHoon => '㍂',
            CJKCompatibility::SquareMaikuro => '㍃',
            CJKCompatibility::SquareMairu => '㍄',
            CJKCompatibility::SquareMahha => '㍅',
            CJKCompatibility::SquareMaruku => '㍆',
            CJKCompatibility::SquareMansyon => '㍇',
            CJKCompatibility::SquareMikuron => '㍈',
            CJKCompatibility::SquareMiri => '㍉',
            CJKCompatibility::SquareMiribaaru => '㍊',
            CJKCompatibility::SquareMega => '㍋',
            CJKCompatibility::SquareMegaton => '㍌',
            CJKCompatibility::SquareMeetoru => '㍍',
            CJKCompatibility::SquareYaado => '㍎',
            CJKCompatibility::SquareYaaru => '㍏',
            CJKCompatibility::SquareYuan => '㍐',
            CJKCompatibility::SquareRittoru => '㍑',
            CJKCompatibility::SquareRira => '㍒',
            CJKCompatibility::SquareRupii => '㍓',
            CJKCompatibility::SquareRuuburu => '㍔',
            CJKCompatibility::SquareRemu => '㍕',
            CJKCompatibility::SquareRentogen => '㍖',
            CJKCompatibility::SquareWatto => '㍗',
            CJKCompatibility::IdeographicTelegraphSymbolForHourZero => '㍘',
            CJKCompatibility::IdeographicTelegraphSymbolForHourOne => '㍙',
            CJKCompatibility::IdeographicTelegraphSymbolForHourTwo => '㍚',
            CJKCompatibility::IdeographicTelegraphSymbolForHourThree => '㍛',
            CJKCompatibility::IdeographicTelegraphSymbolForHourFour => '㍜',
            CJKCompatibility::IdeographicTelegraphSymbolForHourFive => '㍝',
            CJKCompatibility::IdeographicTelegraphSymbolForHourSix => '㍞',
            CJKCompatibility::IdeographicTelegraphSymbolForHourSeven => '㍟',
            CJKCompatibility::IdeographicTelegraphSymbolForHourEight => '㍠',
            CJKCompatibility::IdeographicTelegraphSymbolForHourNine => '㍡',
            CJKCompatibility::IdeographicTelegraphSymbolForHourTen => '㍢',
            CJKCompatibility::IdeographicTelegraphSymbolForHourEleven => '㍣',
            CJKCompatibility::IdeographicTelegraphSymbolForHourTwelve => '㍤',
            CJKCompatibility::IdeographicTelegraphSymbolForHourThirteen => '㍥',
            CJKCompatibility::IdeographicTelegraphSymbolForHourFourteen => '㍦',
            CJKCompatibility::IdeographicTelegraphSymbolForHourFifteen => '㍧',
            CJKCompatibility::IdeographicTelegraphSymbolForHourSixteen => '㍨',
            CJKCompatibility::IdeographicTelegraphSymbolForHourSeventeen => '㍩',
            CJKCompatibility::IdeographicTelegraphSymbolForHourEighteen => '㍪',
            CJKCompatibility::IdeographicTelegraphSymbolForHourNineteen => '㍫',
            CJKCompatibility::IdeographicTelegraphSymbolForHourTwenty => '㍬',
            CJKCompatibility::IdeographicTelegraphSymbolForHourTwentyDashOne => '㍭',
            CJKCompatibility::IdeographicTelegraphSymbolForHourTwentyDashTwo => '㍮',
            CJKCompatibility::IdeographicTelegraphSymbolForHourTwentyDashThree => '㍯',
            CJKCompatibility::IdeographicTelegraphSymbolForHourTwentyDashFour => '㍰',
            CJKCompatibility::SquareHpa => '㍱',
            CJKCompatibility::SquareDa => '㍲',
            CJKCompatibility::SquareAu => '㍳',
            CJKCompatibility::SquareBar => '㍴',
            CJKCompatibility::SquareOv => '㍵',
            CJKCompatibility::SquarePc => '㍶',
            CJKCompatibility::SquareDm => '㍷',
            CJKCompatibility::SquareDmSquared => '㍸',
            CJKCompatibility::SquareDmCubed => '㍹',
            CJKCompatibility::SquareIu => '㍺',
            CJKCompatibility::SquareEraNameHeisei => '㍻',
            CJKCompatibility::SquareEraNameSyouwa => '㍼',
            CJKCompatibility::SquareEraNameTaisyou => '㍽',
            CJKCompatibility::SquareEraNameMeizi => '㍾',
            CJKCompatibility::SquareCorporation => '㍿',
            CJKCompatibility::SquarePaAmps => '㎀',
            CJKCompatibility::SquareNa => '㎁',
            CJKCompatibility::SquareMuA => '㎂',
            CJKCompatibility::SquareMa => '㎃',
            CJKCompatibility::SquareKa => '㎄',
            CJKCompatibility::SquareKb => '㎅',
            CJKCompatibility::SquareMb => '㎆',
            CJKCompatibility::SquareGb => '㎇',
            CJKCompatibility::SquareCal => '㎈',
            CJKCompatibility::SquareKcal => '㎉',
            CJKCompatibility::SquarePf => '㎊',
            CJKCompatibility::SquareNf => '㎋',
            CJKCompatibility::SquareMuF => '㎌',
            CJKCompatibility::SquareMuG => '㎍',
            CJKCompatibility::SquareMg => '㎎',
            CJKCompatibility::SquareKg => '㎏',
            CJKCompatibility::SquareHz => '㎐',
            CJKCompatibility::SquareKhz => '㎑',
            CJKCompatibility::SquareMhz => '㎒',
            CJKCompatibility::SquareGhz => '㎓',
            CJKCompatibility::SquareThz => '㎔',
            CJKCompatibility::SquareMuL => '㎕',
            CJKCompatibility::SquareMl => '㎖',
            CJKCompatibility::SquareDl => '㎗',
            CJKCompatibility::SquareKl => '㎘',
            CJKCompatibility::SquareFm => '㎙',
            CJKCompatibility::SquareNm => '㎚',
            CJKCompatibility::SquareMuM => '㎛',
            CJKCompatibility::SquareMm => '㎜',
            CJKCompatibility::SquareCm => '㎝',
            CJKCompatibility::SquareKm => '㎞',
            CJKCompatibility::SquareMmSquared => '㎟',
            CJKCompatibility::SquareCmSquared => '㎠',
            CJKCompatibility::SquareMSquared => '㎡',
            CJKCompatibility::SquareKmSquared => '㎢',
            CJKCompatibility::SquareMmCubed => '㎣',
            CJKCompatibility::SquareCmCubed => '㎤',
            CJKCompatibility::SquareMCubed => '㎥',
            CJKCompatibility::SquareKmCubed => '㎦',
            CJKCompatibility::SquareMOverS => '㎧',
            CJKCompatibility::SquareMOverSSquared => '㎨',
            CJKCompatibility::SquarePa => '㎩',
            CJKCompatibility::SquareKpa => '㎪',
            CJKCompatibility::SquareMpa => '㎫',
            CJKCompatibility::SquareGpa => '㎬',
            CJKCompatibility::SquareRad => '㎭',
            CJKCompatibility::SquareRadOverS => '㎮',
            CJKCompatibility::SquareRadOverSSquared => '㎯',
            CJKCompatibility::SquarePs => '㎰',
            CJKCompatibility::SquareNs => '㎱',
            CJKCompatibility::SquareMuS => '㎲',
            CJKCompatibility::SquareMs => '㎳',
            CJKCompatibility::SquarePv => '㎴',
            CJKCompatibility::SquareNv => '㎵',
            CJKCompatibility::SquareMuV => '㎶',
            CJKCompatibility::SquareMv => '㎷',
            CJKCompatibility::SquareKv => '㎸',
            CJKCompatibility::SquareMvMega => '㎹',
            CJKCompatibility::SquarePw => '㎺',
            CJKCompatibility::SquareNw => '㎻',
            CJKCompatibility::SquareMuW => '㎼',
            CJKCompatibility::SquareMw => '㎽',
            CJKCompatibility::SquareKw => '㎾',
            CJKCompatibility::SquareMwMega => '㎿',
            CJKCompatibility::SquareKOhm => '㏀',
            CJKCompatibility::SquareMOhm => '㏁',
            CJKCompatibility::SquareAm => '㏂',
            CJKCompatibility::SquareBq => '㏃',
            CJKCompatibility::SquareCc => '㏄',
            CJKCompatibility::SquareCd => '㏅',
            CJKCompatibility::SquareCOverKg => '㏆',
            CJKCompatibility::SquareCo => '㏇',
            CJKCompatibility::SquareDb => '㏈',
            CJKCompatibility::SquareGy => '㏉',
            CJKCompatibility::SquareHa => '㏊',
            CJKCompatibility::SquareHp => '㏋',
            CJKCompatibility::SquareIn => '㏌',
            CJKCompatibility::SquareKk => '㏍',
            CJKCompatibility::SquareKmCapital => '㏎',
            CJKCompatibility::SquareKt => '㏏',
            CJKCompatibility::SquareLm => '㏐',
            CJKCompatibility::SquareLn => '㏑',
            CJKCompatibility::SquareLog => '㏒',
            CJKCompatibility::SquareLx => '㏓',
            CJKCompatibility::SquareMbSmall => '㏔',
            CJKCompatibility::SquareMil => '㏕',
            CJKCompatibility::SquareMol => '㏖',
            CJKCompatibility::SquarePh => '㏗',
            CJKCompatibility::SquarePm => '㏘',
            CJKCompatibility::SquarePpm => '㏙',
            CJKCompatibility::SquarePr => '㏚',
            CJKCompatibility::SquareSr => '㏛',
            CJKCompatibility::SquareSv => '㏜',
            CJKCompatibility::SquareWb => '㏝',
            CJKCompatibility::SquareVOverM => '㏞',
            CJKCompatibility::SquareAOverM => '㏟',
            CJKCompatibility::IdeographicTelegraphSymbolForDayOne => '㏠',
            CJKCompatibility::IdeographicTelegraphSymbolForDayTwo => '㏡',
            CJKCompatibility::IdeographicTelegraphSymbolForDayThree => '㏢',
            CJKCompatibility::IdeographicTelegraphSymbolForDayFour => '㏣',
            CJKCompatibility::IdeographicTelegraphSymbolForDayFive => '㏤',
            CJKCompatibility::IdeographicTelegraphSymbolForDaySix => '㏥',
            CJKCompatibility::IdeographicTelegraphSymbolForDaySeven => '㏦',
            CJKCompatibility::IdeographicTelegraphSymbolForDayEight => '㏧',
            CJKCompatibility::IdeographicTelegraphSymbolForDayNine => '㏨',
            CJKCompatibility::IdeographicTelegraphSymbolForDayTen => '㏩',
            CJKCompatibility::IdeographicTelegraphSymbolForDayEleven => '㏪',
            CJKCompatibility::IdeographicTelegraphSymbolForDayTwelve => '㏫',
            CJKCompatibility::IdeographicTelegraphSymbolForDayThirteen => '㏬',
            CJKCompatibility::IdeographicTelegraphSymbolForDayFourteen => '㏭',
            CJKCompatibility::IdeographicTelegraphSymbolForDayFifteen => '㏮',
            CJKCompatibility::IdeographicTelegraphSymbolForDaySixteen => '㏯',
            CJKCompatibility::IdeographicTelegraphSymbolForDaySeventeen => '㏰',
            CJKCompatibility::IdeographicTelegraphSymbolForDayEighteen => '㏱',
            CJKCompatibility::IdeographicTelegraphSymbolForDayNineteen => '㏲',
            CJKCompatibility::IdeographicTelegraphSymbolForDayTwenty => '㏳',
            CJKCompatibility::IdeographicTelegraphSymbolForDayTwentyDashOne => '㏴',
            CJKCompatibility::IdeographicTelegraphSymbolForDayTwentyDashTwo => '㏵',
            CJKCompatibility::IdeographicTelegraphSymbolForDayTwentyDashThree => '㏶',
            CJKCompatibility::IdeographicTelegraphSymbolForDayTwentyDashFour => '㏷',
            CJKCompatibility::IdeographicTelegraphSymbolForDayTwentyDashFive => '㏸',
            CJKCompatibility::IdeographicTelegraphSymbolForDayTwentyDashSix => '㏹',
            CJKCompatibility::IdeographicTelegraphSymbolForDayTwentyDashSeven => '㏺',
            CJKCompatibility::IdeographicTelegraphSymbolForDayTwentyDashEight => '㏻',
            CJKCompatibility::IdeographicTelegraphSymbolForDayTwentyDashNine => '㏼',
            CJKCompatibility::IdeographicTelegraphSymbolForDayThirty => '㏽',
            CJKCompatibility::IdeographicTelegraphSymbolForDayThirtyDashOne => '㏾',
        }
    }
}

impl std::convert::TryFrom<char> for CJKCompatibility {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '㌀' => Ok(CJKCompatibility::SquareApaato),
            '㌁' => Ok(CJKCompatibility::SquareAruhua),
            '㌂' => Ok(CJKCompatibility::SquareAnpea),
            '㌃' => Ok(CJKCompatibility::SquareAaru),
            '㌄' => Ok(CJKCompatibility::SquareIningu),
            '㌅' => Ok(CJKCompatibility::SquareInti),
            '㌆' => Ok(CJKCompatibility::SquareUon),
            '㌇' => Ok(CJKCompatibility::SquareEsukuudo),
            '㌈' => Ok(CJKCompatibility::SquareEekaa),
            '㌉' => Ok(CJKCompatibility::SquareOnsu),
            '㌊' => Ok(CJKCompatibility::SquareOomu),
            '㌋' => Ok(CJKCompatibility::SquareKairi),
            '㌌' => Ok(CJKCompatibility::SquareKaratto),
            '㌍' => Ok(CJKCompatibility::SquareKarorii),
            '㌎' => Ok(CJKCompatibility::SquareGaron),
            '㌏' => Ok(CJKCompatibility::SquareGanma),
            '㌐' => Ok(CJKCompatibility::SquareGiga),
            '㌑' => Ok(CJKCompatibility::SquareGinii),
            '㌒' => Ok(CJKCompatibility::SquareKyurii),
            '㌓' => Ok(CJKCompatibility::SquareGirudaa),
            '㌔' => Ok(CJKCompatibility::SquareKiro),
            '㌕' => Ok(CJKCompatibility::SquareKiroguramu),
            '㌖' => Ok(CJKCompatibility::SquareKiromeetoru),
            '㌗' => Ok(CJKCompatibility::SquareKirowatto),
            '㌘' => Ok(CJKCompatibility::SquareGuramu),
            '㌙' => Ok(CJKCompatibility::SquareGuramuton),
            '㌚' => Ok(CJKCompatibility::SquareKuruzeiro),
            '㌛' => Ok(CJKCompatibility::SquareKuroone),
            '㌜' => Ok(CJKCompatibility::SquareKeesu),
            '㌝' => Ok(CJKCompatibility::SquareKoruna),
            '㌞' => Ok(CJKCompatibility::SquareKoopo),
            '㌟' => Ok(CJKCompatibility::SquareSaikuru),
            '㌠' => Ok(CJKCompatibility::SquareSantiimu),
            '㌡' => Ok(CJKCompatibility::SquareSiringu),
            '㌢' => Ok(CJKCompatibility::SquareSenti),
            '㌣' => Ok(CJKCompatibility::SquareSento),
            '㌤' => Ok(CJKCompatibility::SquareDaasu),
            '㌥' => Ok(CJKCompatibility::SquareDesi),
            '㌦' => Ok(CJKCompatibility::SquareDoru),
            '㌧' => Ok(CJKCompatibility::SquareTon),
            '㌨' => Ok(CJKCompatibility::SquareNano),
            '㌩' => Ok(CJKCompatibility::SquareNotto),
            '㌪' => Ok(CJKCompatibility::SquareHaitu),
            '㌫' => Ok(CJKCompatibility::SquarePaasento),
            '㌬' => Ok(CJKCompatibility::SquarePaatu),
            '㌭' => Ok(CJKCompatibility::SquareBaareru),
            '㌮' => Ok(CJKCompatibility::SquarePiasutoru),
            '㌯' => Ok(CJKCompatibility::SquarePikuru),
            '㌰' => Ok(CJKCompatibility::SquarePiko),
            '㌱' => Ok(CJKCompatibility::SquareBiru),
            '㌲' => Ok(CJKCompatibility::SquareHuaraddo),
            '㌳' => Ok(CJKCompatibility::SquareHuiito),
            '㌴' => Ok(CJKCompatibility::SquareBussyeru),
            '㌵' => Ok(CJKCompatibility::SquareHuran),
            '㌶' => Ok(CJKCompatibility::SquareHekutaaru),
            '㌷' => Ok(CJKCompatibility::SquarePeso),
            '㌸' => Ok(CJKCompatibility::SquarePenihi),
            '㌹' => Ok(CJKCompatibility::SquareHerutu),
            '㌺' => Ok(CJKCompatibility::SquarePensu),
            '㌻' => Ok(CJKCompatibility::SquarePeezi),
            '㌼' => Ok(CJKCompatibility::SquareBeeta),
            '㌽' => Ok(CJKCompatibility::SquarePointo),
            '㌾' => Ok(CJKCompatibility::SquareBoruto),
            '㌿' => Ok(CJKCompatibility::SquareHon),
            '㍀' => Ok(CJKCompatibility::SquarePondo),
            '㍁' => Ok(CJKCompatibility::SquareHooru),
            '㍂' => Ok(CJKCompatibility::SquareHoon),
            '㍃' => Ok(CJKCompatibility::SquareMaikuro),
            '㍄' => Ok(CJKCompatibility::SquareMairu),
            '㍅' => Ok(CJKCompatibility::SquareMahha),
            '㍆' => Ok(CJKCompatibility::SquareMaruku),
            '㍇' => Ok(CJKCompatibility::SquareMansyon),
            '㍈' => Ok(CJKCompatibility::SquareMikuron),
            '㍉' => Ok(CJKCompatibility::SquareMiri),
            '㍊' => Ok(CJKCompatibility::SquareMiribaaru),
            '㍋' => Ok(CJKCompatibility::SquareMega),
            '㍌' => Ok(CJKCompatibility::SquareMegaton),
            '㍍' => Ok(CJKCompatibility::SquareMeetoru),
            '㍎' => Ok(CJKCompatibility::SquareYaado),
            '㍏' => Ok(CJKCompatibility::SquareYaaru),
            '㍐' => Ok(CJKCompatibility::SquareYuan),
            '㍑' => Ok(CJKCompatibility::SquareRittoru),
            '㍒' => Ok(CJKCompatibility::SquareRira),
            '㍓' => Ok(CJKCompatibility::SquareRupii),
            '㍔' => Ok(CJKCompatibility::SquareRuuburu),
            '㍕' => Ok(CJKCompatibility::SquareRemu),
            '㍖' => Ok(CJKCompatibility::SquareRentogen),
            '㍗' => Ok(CJKCompatibility::SquareWatto),
            '㍘' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForHourZero),
            '㍙' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForHourOne),
            '㍚' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForHourTwo),
            '㍛' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForHourThree),
            '㍜' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForHourFour),
            '㍝' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForHourFive),
            '㍞' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForHourSix),
            '㍟' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForHourSeven),
            '㍠' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForHourEight),
            '㍡' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForHourNine),
            '㍢' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForHourTen),
            '㍣' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForHourEleven),
            '㍤' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForHourTwelve),
            '㍥' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForHourThirteen),
            '㍦' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForHourFourteen),
            '㍧' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForHourFifteen),
            '㍨' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForHourSixteen),
            '㍩' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForHourSeventeen),
            '㍪' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForHourEighteen),
            '㍫' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForHourNineteen),
            '㍬' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForHourTwenty),
            '㍭' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForHourTwentyDashOne),
            '㍮' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForHourTwentyDashTwo),
            '㍯' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForHourTwentyDashThree),
            '㍰' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForHourTwentyDashFour),
            '㍱' => Ok(CJKCompatibility::SquareHpa),
            '㍲' => Ok(CJKCompatibility::SquareDa),
            '㍳' => Ok(CJKCompatibility::SquareAu),
            '㍴' => Ok(CJKCompatibility::SquareBar),
            '㍵' => Ok(CJKCompatibility::SquareOv),
            '㍶' => Ok(CJKCompatibility::SquarePc),
            '㍷' => Ok(CJKCompatibility::SquareDm),
            '㍸' => Ok(CJKCompatibility::SquareDmSquared),
            '㍹' => Ok(CJKCompatibility::SquareDmCubed),
            '㍺' => Ok(CJKCompatibility::SquareIu),
            '㍻' => Ok(CJKCompatibility::SquareEraNameHeisei),
            '㍼' => Ok(CJKCompatibility::SquareEraNameSyouwa),
            '㍽' => Ok(CJKCompatibility::SquareEraNameTaisyou),
            '㍾' => Ok(CJKCompatibility::SquareEraNameMeizi),
            '㍿' => Ok(CJKCompatibility::SquareCorporation),
            '㎀' => Ok(CJKCompatibility::SquarePaAmps),
            '㎁' => Ok(CJKCompatibility::SquareNa),
            '㎂' => Ok(CJKCompatibility::SquareMuA),
            '㎃' => Ok(CJKCompatibility::SquareMa),
            '㎄' => Ok(CJKCompatibility::SquareKa),
            '㎅' => Ok(CJKCompatibility::SquareKb),
            '㎆' => Ok(CJKCompatibility::SquareMb),
            '㎇' => Ok(CJKCompatibility::SquareGb),
            '㎈' => Ok(CJKCompatibility::SquareCal),
            '㎉' => Ok(CJKCompatibility::SquareKcal),
            '㎊' => Ok(CJKCompatibility::SquarePf),
            '㎋' => Ok(CJKCompatibility::SquareNf),
            '㎌' => Ok(CJKCompatibility::SquareMuF),
            '㎍' => Ok(CJKCompatibility::SquareMuG),
            '㎎' => Ok(CJKCompatibility::SquareMg),
            '㎏' => Ok(CJKCompatibility::SquareKg),
            '㎐' => Ok(CJKCompatibility::SquareHz),
            '㎑' => Ok(CJKCompatibility::SquareKhz),
            '㎒' => Ok(CJKCompatibility::SquareMhz),
            '㎓' => Ok(CJKCompatibility::SquareGhz),
            '㎔' => Ok(CJKCompatibility::SquareThz),
            '㎕' => Ok(CJKCompatibility::SquareMuL),
            '㎖' => Ok(CJKCompatibility::SquareMl),
            '㎗' => Ok(CJKCompatibility::SquareDl),
            '㎘' => Ok(CJKCompatibility::SquareKl),
            '㎙' => Ok(CJKCompatibility::SquareFm),
            '㎚' => Ok(CJKCompatibility::SquareNm),
            '㎛' => Ok(CJKCompatibility::SquareMuM),
            '㎜' => Ok(CJKCompatibility::SquareMm),
            '㎝' => Ok(CJKCompatibility::SquareCm),
            '㎞' => Ok(CJKCompatibility::SquareKm),
            '㎟' => Ok(CJKCompatibility::SquareMmSquared),
            '㎠' => Ok(CJKCompatibility::SquareCmSquared),
            '㎡' => Ok(CJKCompatibility::SquareMSquared),
            '㎢' => Ok(CJKCompatibility::SquareKmSquared),
            '㎣' => Ok(CJKCompatibility::SquareMmCubed),
            '㎤' => Ok(CJKCompatibility::SquareCmCubed),
            '㎥' => Ok(CJKCompatibility::SquareMCubed),
            '㎦' => Ok(CJKCompatibility::SquareKmCubed),
            '㎧' => Ok(CJKCompatibility::SquareMOverS),
            '㎨' => Ok(CJKCompatibility::SquareMOverSSquared),
            '㎩' => Ok(CJKCompatibility::SquarePa),
            '㎪' => Ok(CJKCompatibility::SquareKpa),
            '㎫' => Ok(CJKCompatibility::SquareMpa),
            '㎬' => Ok(CJKCompatibility::SquareGpa),
            '㎭' => Ok(CJKCompatibility::SquareRad),
            '㎮' => Ok(CJKCompatibility::SquareRadOverS),
            '㎯' => Ok(CJKCompatibility::SquareRadOverSSquared),
            '㎰' => Ok(CJKCompatibility::SquarePs),
            '㎱' => Ok(CJKCompatibility::SquareNs),
            '㎲' => Ok(CJKCompatibility::SquareMuS),
            '㎳' => Ok(CJKCompatibility::SquareMs),
            '㎴' => Ok(CJKCompatibility::SquarePv),
            '㎵' => Ok(CJKCompatibility::SquareNv),
            '㎶' => Ok(CJKCompatibility::SquareMuV),
            '㎷' => Ok(CJKCompatibility::SquareMv),
            '㎸' => Ok(CJKCompatibility::SquareKv),
            '㎹' => Ok(CJKCompatibility::SquareMvMega),
            '㎺' => Ok(CJKCompatibility::SquarePw),
            '㎻' => Ok(CJKCompatibility::SquareNw),
            '㎼' => Ok(CJKCompatibility::SquareMuW),
            '㎽' => Ok(CJKCompatibility::SquareMw),
            '㎾' => Ok(CJKCompatibility::SquareKw),
            '㎿' => Ok(CJKCompatibility::SquareMwMega),
            '㏀' => Ok(CJKCompatibility::SquareKOhm),
            '㏁' => Ok(CJKCompatibility::SquareMOhm),
            '㏂' => Ok(CJKCompatibility::SquareAm),
            '㏃' => Ok(CJKCompatibility::SquareBq),
            '㏄' => Ok(CJKCompatibility::SquareCc),
            '㏅' => Ok(CJKCompatibility::SquareCd),
            '㏆' => Ok(CJKCompatibility::SquareCOverKg),
            '㏇' => Ok(CJKCompatibility::SquareCo),
            '㏈' => Ok(CJKCompatibility::SquareDb),
            '㏉' => Ok(CJKCompatibility::SquareGy),
            '㏊' => Ok(CJKCompatibility::SquareHa),
            '㏋' => Ok(CJKCompatibility::SquareHp),
            '㏌' => Ok(CJKCompatibility::SquareIn),
            '㏍' => Ok(CJKCompatibility::SquareKk),
            '㏎' => Ok(CJKCompatibility::SquareKmCapital),
            '㏏' => Ok(CJKCompatibility::SquareKt),
            '㏐' => Ok(CJKCompatibility::SquareLm),
            '㏑' => Ok(CJKCompatibility::SquareLn),
            '㏒' => Ok(CJKCompatibility::SquareLog),
            '㏓' => Ok(CJKCompatibility::SquareLx),
            '㏔' => Ok(CJKCompatibility::SquareMbSmall),
            '㏕' => Ok(CJKCompatibility::SquareMil),
            '㏖' => Ok(CJKCompatibility::SquareMol),
            '㏗' => Ok(CJKCompatibility::SquarePh),
            '㏘' => Ok(CJKCompatibility::SquarePm),
            '㏙' => Ok(CJKCompatibility::SquarePpm),
            '㏚' => Ok(CJKCompatibility::SquarePr),
            '㏛' => Ok(CJKCompatibility::SquareSr),
            '㏜' => Ok(CJKCompatibility::SquareSv),
            '㏝' => Ok(CJKCompatibility::SquareWb),
            '㏞' => Ok(CJKCompatibility::SquareVOverM),
            '㏟' => Ok(CJKCompatibility::SquareAOverM),
            '㏠' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForDayOne),
            '㏡' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForDayTwo),
            '㏢' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForDayThree),
            '㏣' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForDayFour),
            '㏤' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForDayFive),
            '㏥' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForDaySix),
            '㏦' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForDaySeven),
            '㏧' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForDayEight),
            '㏨' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForDayNine),
            '㏩' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForDayTen),
            '㏪' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForDayEleven),
            '㏫' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForDayTwelve),
            '㏬' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForDayThirteen),
            '㏭' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForDayFourteen),
            '㏮' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForDayFifteen),
            '㏯' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForDaySixteen),
            '㏰' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForDaySeventeen),
            '㏱' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForDayEighteen),
            '㏲' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForDayNineteen),
            '㏳' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForDayTwenty),
            '㏴' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForDayTwentyDashOne),
            '㏵' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForDayTwentyDashTwo),
            '㏶' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForDayTwentyDashThree),
            '㏷' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForDayTwentyDashFour),
            '㏸' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForDayTwentyDashFive),
            '㏹' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForDayTwentyDashSix),
            '㏺' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForDayTwentyDashSeven),
            '㏻' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForDayTwentyDashEight),
            '㏼' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForDayTwentyDashNine),
            '㏽' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForDayThirty),
            '㏾' => Ok(CJKCompatibility::IdeographicTelegraphSymbolForDayThirtyDashOne),
            _ => Err(()),
        }
    }
}

impl Into<u32> for CJKCompatibility {
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

impl std::convert::TryFrom<u32> for CJKCompatibility {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for CJKCompatibility {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl CJKCompatibility {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        CJKCompatibility::SquareApaato
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("CJKCompatibility{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
