
/// An enum to represent all characters in the MiscellaneousSymbols block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum MiscellaneousSymbols {
    /// \u{2600}: '☀'
    BlackSunWithRays,
    /// \u{2601}: '☁'
    Cloud,
    /// \u{2602}: '☂'
    Umbrella,
    /// \u{2603}: '☃'
    Snowman,
    /// \u{2604}: '☄'
    Comet,
    /// \u{2605}: '★'
    BlackStar,
    /// \u{2606}: '☆'
    WhiteStar,
    /// \u{2607}: '☇'
    Lightning,
    /// \u{2608}: '☈'
    Thunderstorm,
    /// \u{2609}: '☉'
    Sun,
    /// \u{260a}: '☊'
    AscendingNode,
    /// \u{260b}: '☋'
    DescendingNode,
    /// \u{260c}: '☌'
    Conjunction,
    /// \u{260d}: '☍'
    Opposition,
    /// \u{260e}: '☎'
    BlackTelephone,
    /// \u{260f}: '☏'
    WhiteTelephone,
    /// \u{2610}: '☐'
    BallotBox,
    /// \u{2611}: '☑'
    BallotBoxWithCheck,
    /// \u{2612}: '☒'
    BallotBoxWithX,
    /// \u{2613}: '☓'
    Saltire,
    /// \u{2614}: '☔'
    UmbrellaWithRainDrops,
    /// \u{2615}: '☕'
    HotBeverage,
    /// \u{2616}: '☖'
    WhiteShogiPiece,
    /// \u{2617}: '☗'
    BlackShogiPiece,
    /// \u{2618}: '☘'
    Shamrock,
    /// \u{2619}: '☙'
    ReversedRotatedFloralHeartBullet,
    /// \u{261a}: '☚'
    BlackLeftPointingIndex,
    /// \u{261b}: '☛'
    BlackRightPointingIndex,
    /// \u{261c}: '☜'
    WhiteLeftPointingIndex,
    /// \u{261d}: '☝'
    WhiteUpPointingIndex,
    /// \u{261e}: '☞'
    WhiteRightPointingIndex,
    /// \u{261f}: '☟'
    WhiteDownPointingIndex,
    /// \u{2620}: '☠'
    SkullAndCrossbones,
    /// \u{2621}: '☡'
    CautionSign,
    /// \u{2622}: '☢'
    RadioactiveSign,
    /// \u{2623}: '☣'
    BiohazardSign,
    /// \u{2624}: '☤'
    Caduceus,
    /// \u{2625}: '☥'
    Ankh,
    /// \u{2626}: '☦'
    OrthodoxCross,
    /// \u{2627}: '☧'
    ChiRho,
    /// \u{2628}: '☨'
    CrossOfLorraine,
    /// \u{2629}: '☩'
    CrossOfJerusalem,
    /// \u{262a}: '☪'
    StarAndCrescent,
    /// \u{262b}: '☫'
    FarsiSymbol,
    /// \u{262c}: '☬'
    AdiShakti,
    /// \u{262d}: '☭'
    HammerAndSickle,
    /// \u{262e}: '☮'
    PeaceSymbol,
    /// \u{262f}: '☯'
    YinYang,
    /// \u{2630}: '☰'
    TrigramForHeaven,
    /// \u{2631}: '☱'
    TrigramForLake,
    /// \u{2632}: '☲'
    TrigramForFire,
    /// \u{2633}: '☳'
    TrigramForThunder,
    /// \u{2634}: '☴'
    TrigramForWind,
    /// \u{2635}: '☵'
    TrigramForWater,
    /// \u{2636}: '☶'
    TrigramForMountain,
    /// \u{2637}: '☷'
    TrigramForEarth,
    /// \u{2638}: '☸'
    WheelOfDharma,
    /// \u{2639}: '☹'
    WhiteFrowningFace,
    /// \u{263a}: '☺'
    WhiteSmilingFace,
    /// \u{263b}: '☻'
    BlackSmilingFace,
    /// \u{263c}: '☼'
    WhiteSunWithRays,
    /// \u{263d}: '☽'
    FirstQuarterMoon,
    /// \u{263e}: '☾'
    LastQuarterMoon,
    /// \u{263f}: '☿'
    Mercury,
    /// \u{2640}: '♀'
    FemaleSign,
    /// \u{2641}: '♁'
    Earth,
    /// \u{2642}: '♂'
    MaleSign,
    /// \u{2643}: '♃'
    Jupiter,
    /// \u{2644}: '♄'
    Saturn,
    /// \u{2645}: '♅'
    Uranus,
    /// \u{2646}: '♆'
    Neptune,
    /// \u{2647}: '♇'
    Pluto,
    /// \u{2648}: '♈'
    Aries,
    /// \u{2649}: '♉'
    Taurus,
    /// \u{264a}: '♊'
    Gemini,
    /// \u{264b}: '♋'
    Cancer,
    /// \u{264c}: '♌'
    Leo,
    /// \u{264d}: '♍'
    Virgo,
    /// \u{264e}: '♎'
    Libra,
    /// \u{264f}: '♏'
    Scorpius,
    /// \u{2650}: '♐'
    Sagittarius,
    /// \u{2651}: '♑'
    Capricorn,
    /// \u{2652}: '♒'
    Aquarius,
    /// \u{2653}: '♓'
    Pisces,
    /// \u{2654}: '♔'
    WhiteChessKing,
    /// \u{2655}: '♕'
    WhiteChessQueen,
    /// \u{2656}: '♖'
    WhiteChessRook,
    /// \u{2657}: '♗'
    WhiteChessBishop,
    /// \u{2658}: '♘'
    WhiteChessKnight,
    /// \u{2659}: '♙'
    WhiteChessPawn,
    /// \u{265a}: '♚'
    BlackChessKing,
    /// \u{265b}: '♛'
    BlackChessQueen,
    /// \u{265c}: '♜'
    BlackChessRook,
    /// \u{265d}: '♝'
    BlackChessBishop,
    /// \u{265e}: '♞'
    BlackChessKnight,
    /// \u{265f}: '♟'
    BlackChessPawn,
    /// \u{2660}: '♠'
    BlackSpadeSuit,
    /// \u{2661}: '♡'
    WhiteHeartSuit,
    /// \u{2662}: '♢'
    WhiteDiamondSuit,
    /// \u{2663}: '♣'
    BlackClubSuit,
    /// \u{2664}: '♤'
    WhiteSpadeSuit,
    /// \u{2665}: '♥'
    BlackHeartSuit,
    /// \u{2666}: '♦'
    BlackDiamondSuit,
    /// \u{2667}: '♧'
    WhiteClubSuit,
    /// \u{2668}: '♨'
    HotSprings,
    /// \u{2669}: '♩'
    QuarterNote,
    /// \u{266a}: '♪'
    EighthNote,
    /// \u{266b}: '♫'
    BeamedEighthNotes,
    /// \u{266c}: '♬'
    BeamedSixteenthNotes,
    /// \u{266d}: '♭'
    MusicFlatSign,
    /// \u{266e}: '♮'
    MusicNaturalSign,
    /// \u{266f}: '♯'
    MusicSharpSign,
    /// \u{2670}: '♰'
    WestSyriacCross,
    /// \u{2671}: '♱'
    EastSyriacCross,
    /// \u{2672}: '♲'
    UniversalRecyclingSymbol,
    /// \u{2673}: '♳'
    RecyclingSymbolForTypeDash1Plastics,
    /// \u{2674}: '♴'
    RecyclingSymbolForTypeDash2Plastics,
    /// \u{2675}: '♵'
    RecyclingSymbolForTypeDash3Plastics,
    /// \u{2676}: '♶'
    RecyclingSymbolForTypeDash4Plastics,
    /// \u{2677}: '♷'
    RecyclingSymbolForTypeDash5Plastics,
    /// \u{2678}: '♸'
    RecyclingSymbolForTypeDash6Plastics,
    /// \u{2679}: '♹'
    RecyclingSymbolForTypeDash7Plastics,
    /// \u{267a}: '♺'
    RecyclingSymbolForGenericMaterials,
    /// \u{267b}: '♻'
    BlackUniversalRecyclingSymbol,
    /// \u{267c}: '♼'
    RecycledPaperSymbol,
    /// \u{267d}: '♽'
    PartiallyDashRecycledPaperSymbol,
    /// \u{267e}: '♾'
    PermanentPaperSign,
    /// \u{267f}: '♿'
    WheelchairSymbol,
    /// \u{2680}: '⚀'
    DieFaceDash1,
    /// \u{2681}: '⚁'
    DieFaceDash2,
    /// \u{2682}: '⚂'
    DieFaceDash3,
    /// \u{2683}: '⚃'
    DieFaceDash4,
    /// \u{2684}: '⚄'
    DieFaceDash5,
    /// \u{2685}: '⚅'
    DieFaceDash6,
    /// \u{2686}: '⚆'
    WhiteCircleWithDotRight,
    /// \u{2687}: '⚇'
    WhiteCircleWithTwoDots,
    /// \u{2688}: '⚈'
    BlackCircleWithWhiteDotRight,
    /// \u{2689}: '⚉'
    BlackCircleWithTwoWhiteDots,
    /// \u{268a}: '⚊'
    MonogramForYang,
    /// \u{268b}: '⚋'
    MonogramForYin,
    /// \u{268c}: '⚌'
    DigramForGreaterYang,
    /// \u{268d}: '⚍'
    DigramForLesserYin,
    /// \u{268e}: '⚎'
    DigramForLesserYang,
    /// \u{268f}: '⚏'
    DigramForGreaterYin,
    /// \u{2690}: '⚐'
    WhiteFlag,
    /// \u{2691}: '⚑'
    BlackFlag,
    /// \u{2692}: '⚒'
    HammerAndPick,
    /// \u{2693}: '⚓'
    Anchor,
    /// \u{2694}: '⚔'
    CrossedSwords,
    /// \u{2695}: '⚕'
    StaffOfAesculapius,
    /// \u{2696}: '⚖'
    Scales,
    /// \u{2697}: '⚗'
    Alembic,
    /// \u{2698}: '⚘'
    Flower,
    /// \u{2699}: '⚙'
    Gear,
    /// \u{269a}: '⚚'
    StaffOfHermes,
    /// \u{269b}: '⚛'
    AtomSymbol,
    /// \u{269c}: '⚜'
    FleurDashDeDashLis,
    /// \u{269d}: '⚝'
    OutlinedWhiteStar,
    /// \u{269e}: '⚞'
    ThreeLinesConvergingRight,
    /// \u{269f}: '⚟'
    ThreeLinesConvergingLeft,
    /// \u{26a0}: '⚠'
    WarningSign,
    /// \u{26a1}: '⚡'
    HighVoltageSign,
    /// \u{26a2}: '⚢'
    DoubledFemaleSign,
    /// \u{26a3}: '⚣'
    DoubledMaleSign,
    /// \u{26a4}: '⚤'
    InterlockedFemaleAndMaleSign,
    /// \u{26a5}: '⚥'
    MaleAndFemaleSign,
    /// \u{26a6}: '⚦'
    MaleWithStrokeSign,
    /// \u{26a7}: '⚧'
    MaleWithStrokeAndMaleAndFemaleSign,
    /// \u{26a8}: '⚨'
    VerticalMaleWithStrokeSign,
    /// \u{26a9}: '⚩'
    HorizontalMaleWithStrokeSign,
    /// \u{26aa}: '⚪'
    MediumWhiteCircle,
    /// \u{26ab}: '⚫'
    MediumBlackCircle,
    /// \u{26ac}: '⚬'
    MediumSmallWhiteCircle,
    /// \u{26ad}: '⚭'
    MarriageSymbol,
    /// \u{26ae}: '⚮'
    DivorceSymbol,
    /// \u{26af}: '⚯'
    UnmarriedPartnershipSymbol,
    /// \u{26b0}: '⚰'
    Coffin,
    /// \u{26b1}: '⚱'
    FuneralUrn,
    /// \u{26b2}: '⚲'
    Neuter,
    /// \u{26b3}: '⚳'
    Ceres,
    /// \u{26b4}: '⚴'
    Pallas,
    /// \u{26b5}: '⚵'
    Juno,
    /// \u{26b6}: '⚶'
    Vesta,
    /// \u{26b7}: '⚷'
    Chiron,
    /// \u{26b8}: '⚸'
    BlackMoonLilith,
    /// \u{26b9}: '⚹'
    Sextile,
    /// \u{26ba}: '⚺'
    Semisextile,
    /// \u{26bb}: '⚻'
    Quincunx,
    /// \u{26bc}: '⚼'
    Sesquiquadrate,
    /// \u{26bd}: '⚽'
    SoccerBall,
    /// \u{26be}: '⚾'
    Baseball,
    /// \u{26bf}: '⚿'
    SquaredKey,
    /// \u{26c0}: '⛀'
    WhiteDraughtsMan,
    /// \u{26c1}: '⛁'
    WhiteDraughtsKing,
    /// \u{26c2}: '⛂'
    BlackDraughtsMan,
    /// \u{26c3}: '⛃'
    BlackDraughtsKing,
    /// \u{26c4}: '⛄'
    SnowmanWithoutSnow,
    /// \u{26c5}: '⛅'
    SunBehindCloud,
    /// \u{26c6}: '⛆'
    Rain,
    /// \u{26c7}: '⛇'
    BlackSnowman,
    /// \u{26c8}: '⛈'
    ThunderCloudAndRain,
    /// \u{26c9}: '⛉'
    TurnedWhiteShogiPiece,
    /// \u{26ca}: '⛊'
    TurnedBlackShogiPiece,
    /// \u{26cb}: '⛋'
    WhiteDiamondInSquare,
    /// \u{26cc}: '⛌'
    CrossingLanes,
    /// \u{26cd}: '⛍'
    DisabledCar,
    /// \u{26ce}: '⛎'
    Ophiuchus,
    /// \u{26cf}: '⛏'
    Pick,
    /// \u{26d0}: '⛐'
    CarSliding,
    /// \u{26d1}: '⛑'
    HelmetWithWhiteCross,
    /// \u{26d2}: '⛒'
    CircledCrossingLanes,
    /// \u{26d3}: '⛓'
    Chains,
    /// \u{26d4}: '⛔'
    NoEntry,
    /// \u{26d5}: '⛕'
    AlternateOneDashWayLeftWayTraffic,
    /// \u{26d6}: '⛖'
    BlackTwoDashWayLeftWayTraffic,
    /// \u{26d7}: '⛗'
    WhiteTwoDashWayLeftWayTraffic,
    /// \u{26d8}: '⛘'
    BlackLeftLaneMerge,
    /// \u{26d9}: '⛙'
    WhiteLeftLaneMerge,
    /// \u{26da}: '⛚'
    DriveSlowSign,
    /// \u{26db}: '⛛'
    HeavyWhiteDownDashPointingTriangle,
    /// \u{26dc}: '⛜'
    LeftClosedEntry,
    /// \u{26dd}: '⛝'
    SquaredSaltire,
    /// \u{26de}: '⛞'
    FallingDiagonalInWhiteCircleInBlackSquare,
    /// \u{26df}: '⛟'
    BlackTruck,
    /// \u{26e0}: '⛠'
    RestrictedLeftEntryDash1,
    /// \u{26e1}: '⛡'
    RestrictedLeftEntryDash2,
    /// \u{26e2}: '⛢'
    AstronomicalSymbolForUranus,
    /// \u{26e3}: '⛣'
    HeavyCircleWithStrokeAndTwoDotsAbove,
    /// \u{26e4}: '⛤'
    Pentagram,
    /// \u{26e5}: '⛥'
    RightDashHandedInterlacedPentagram,
    /// \u{26e6}: '⛦'
    LeftDashHandedInterlacedPentagram,
    /// \u{26e7}: '⛧'
    InvertedPentagram,
    /// \u{26e8}: '⛨'
    BlackCrossOnShield,
    /// \u{26e9}: '⛩'
    ShintoShrine,
    /// \u{26ea}: '⛪'
    Church,
    /// \u{26eb}: '⛫'
    Castle,
    /// \u{26ec}: '⛬'
    HistoricSite,
    /// \u{26ed}: '⛭'
    GearWithoutHub,
    /// \u{26ee}: '⛮'
    GearWithHandles,
    /// \u{26ef}: '⛯'
    MapSymbolForLighthouse,
    /// \u{26f0}: '⛰'
    Mountain,
    /// \u{26f1}: '⛱'
    UmbrellaOnGround,
    /// \u{26f2}: '⛲'
    Fountain,
    /// \u{26f3}: '⛳'
    FlagInHole,
    /// \u{26f4}: '⛴'
    Ferry,
    /// \u{26f5}: '⛵'
    Sailboat,
    /// \u{26f6}: '⛶'
    SquareFourCorners,
    /// \u{26f7}: '⛷'
    Skier,
    /// \u{26f8}: '⛸'
    IceSkate,
    /// \u{26f9}: '⛹'
    PersonWithBall,
    /// \u{26fa}: '⛺'
    Tent,
    /// \u{26fb}: '⛻'
    JapaneseBankSymbol,
    /// \u{26fc}: '⛼'
    HeadstoneGraveyardSymbol,
    /// \u{26fd}: '⛽'
    FuelPump,
    /// \u{26fe}: '⛾'
    CupOnBlackSquare,
}

impl Into<char> for MiscellaneousSymbols {
    fn into(self) -> char {
        match self {
            MiscellaneousSymbols::BlackSunWithRays => '☀',
            MiscellaneousSymbols::Cloud => '☁',
            MiscellaneousSymbols::Umbrella => '☂',
            MiscellaneousSymbols::Snowman => '☃',
            MiscellaneousSymbols::Comet => '☄',
            MiscellaneousSymbols::BlackStar => '★',
            MiscellaneousSymbols::WhiteStar => '☆',
            MiscellaneousSymbols::Lightning => '☇',
            MiscellaneousSymbols::Thunderstorm => '☈',
            MiscellaneousSymbols::Sun => '☉',
            MiscellaneousSymbols::AscendingNode => '☊',
            MiscellaneousSymbols::DescendingNode => '☋',
            MiscellaneousSymbols::Conjunction => '☌',
            MiscellaneousSymbols::Opposition => '☍',
            MiscellaneousSymbols::BlackTelephone => '☎',
            MiscellaneousSymbols::WhiteTelephone => '☏',
            MiscellaneousSymbols::BallotBox => '☐',
            MiscellaneousSymbols::BallotBoxWithCheck => '☑',
            MiscellaneousSymbols::BallotBoxWithX => '☒',
            MiscellaneousSymbols::Saltire => '☓',
            MiscellaneousSymbols::UmbrellaWithRainDrops => '☔',
            MiscellaneousSymbols::HotBeverage => '☕',
            MiscellaneousSymbols::WhiteShogiPiece => '☖',
            MiscellaneousSymbols::BlackShogiPiece => '☗',
            MiscellaneousSymbols::Shamrock => '☘',
            MiscellaneousSymbols::ReversedRotatedFloralHeartBullet => '☙',
            MiscellaneousSymbols::BlackLeftPointingIndex => '☚',
            MiscellaneousSymbols::BlackRightPointingIndex => '☛',
            MiscellaneousSymbols::WhiteLeftPointingIndex => '☜',
            MiscellaneousSymbols::WhiteUpPointingIndex => '☝',
            MiscellaneousSymbols::WhiteRightPointingIndex => '☞',
            MiscellaneousSymbols::WhiteDownPointingIndex => '☟',
            MiscellaneousSymbols::SkullAndCrossbones => '☠',
            MiscellaneousSymbols::CautionSign => '☡',
            MiscellaneousSymbols::RadioactiveSign => '☢',
            MiscellaneousSymbols::BiohazardSign => '☣',
            MiscellaneousSymbols::Caduceus => '☤',
            MiscellaneousSymbols::Ankh => '☥',
            MiscellaneousSymbols::OrthodoxCross => '☦',
            MiscellaneousSymbols::ChiRho => '☧',
            MiscellaneousSymbols::CrossOfLorraine => '☨',
            MiscellaneousSymbols::CrossOfJerusalem => '☩',
            MiscellaneousSymbols::StarAndCrescent => '☪',
            MiscellaneousSymbols::FarsiSymbol => '☫',
            MiscellaneousSymbols::AdiShakti => '☬',
            MiscellaneousSymbols::HammerAndSickle => '☭',
            MiscellaneousSymbols::PeaceSymbol => '☮',
            MiscellaneousSymbols::YinYang => '☯',
            MiscellaneousSymbols::TrigramForHeaven => '☰',
            MiscellaneousSymbols::TrigramForLake => '☱',
            MiscellaneousSymbols::TrigramForFire => '☲',
            MiscellaneousSymbols::TrigramForThunder => '☳',
            MiscellaneousSymbols::TrigramForWind => '☴',
            MiscellaneousSymbols::TrigramForWater => '☵',
            MiscellaneousSymbols::TrigramForMountain => '☶',
            MiscellaneousSymbols::TrigramForEarth => '☷',
            MiscellaneousSymbols::WheelOfDharma => '☸',
            MiscellaneousSymbols::WhiteFrowningFace => '☹',
            MiscellaneousSymbols::WhiteSmilingFace => '☺',
            MiscellaneousSymbols::BlackSmilingFace => '☻',
            MiscellaneousSymbols::WhiteSunWithRays => '☼',
            MiscellaneousSymbols::FirstQuarterMoon => '☽',
            MiscellaneousSymbols::LastQuarterMoon => '☾',
            MiscellaneousSymbols::Mercury => '☿',
            MiscellaneousSymbols::FemaleSign => '♀',
            MiscellaneousSymbols::Earth => '♁',
            MiscellaneousSymbols::MaleSign => '♂',
            MiscellaneousSymbols::Jupiter => '♃',
            MiscellaneousSymbols::Saturn => '♄',
            MiscellaneousSymbols::Uranus => '♅',
            MiscellaneousSymbols::Neptune => '♆',
            MiscellaneousSymbols::Pluto => '♇',
            MiscellaneousSymbols::Aries => '♈',
            MiscellaneousSymbols::Taurus => '♉',
            MiscellaneousSymbols::Gemini => '♊',
            MiscellaneousSymbols::Cancer => '♋',
            MiscellaneousSymbols::Leo => '♌',
            MiscellaneousSymbols::Virgo => '♍',
            MiscellaneousSymbols::Libra => '♎',
            MiscellaneousSymbols::Scorpius => '♏',
            MiscellaneousSymbols::Sagittarius => '♐',
            MiscellaneousSymbols::Capricorn => '♑',
            MiscellaneousSymbols::Aquarius => '♒',
            MiscellaneousSymbols::Pisces => '♓',
            MiscellaneousSymbols::WhiteChessKing => '♔',
            MiscellaneousSymbols::WhiteChessQueen => '♕',
            MiscellaneousSymbols::WhiteChessRook => '♖',
            MiscellaneousSymbols::WhiteChessBishop => '♗',
            MiscellaneousSymbols::WhiteChessKnight => '♘',
            MiscellaneousSymbols::WhiteChessPawn => '♙',
            MiscellaneousSymbols::BlackChessKing => '♚',
            MiscellaneousSymbols::BlackChessQueen => '♛',
            MiscellaneousSymbols::BlackChessRook => '♜',
            MiscellaneousSymbols::BlackChessBishop => '♝',
            MiscellaneousSymbols::BlackChessKnight => '♞',
            MiscellaneousSymbols::BlackChessPawn => '♟',
            MiscellaneousSymbols::BlackSpadeSuit => '♠',
            MiscellaneousSymbols::WhiteHeartSuit => '♡',
            MiscellaneousSymbols::WhiteDiamondSuit => '♢',
            MiscellaneousSymbols::BlackClubSuit => '♣',
            MiscellaneousSymbols::WhiteSpadeSuit => '♤',
            MiscellaneousSymbols::BlackHeartSuit => '♥',
            MiscellaneousSymbols::BlackDiamondSuit => '♦',
            MiscellaneousSymbols::WhiteClubSuit => '♧',
            MiscellaneousSymbols::HotSprings => '♨',
            MiscellaneousSymbols::QuarterNote => '♩',
            MiscellaneousSymbols::EighthNote => '♪',
            MiscellaneousSymbols::BeamedEighthNotes => '♫',
            MiscellaneousSymbols::BeamedSixteenthNotes => '♬',
            MiscellaneousSymbols::MusicFlatSign => '♭',
            MiscellaneousSymbols::MusicNaturalSign => '♮',
            MiscellaneousSymbols::MusicSharpSign => '♯',
            MiscellaneousSymbols::WestSyriacCross => '♰',
            MiscellaneousSymbols::EastSyriacCross => '♱',
            MiscellaneousSymbols::UniversalRecyclingSymbol => '♲',
            MiscellaneousSymbols::RecyclingSymbolForTypeDash1Plastics => '♳',
            MiscellaneousSymbols::RecyclingSymbolForTypeDash2Plastics => '♴',
            MiscellaneousSymbols::RecyclingSymbolForTypeDash3Plastics => '♵',
            MiscellaneousSymbols::RecyclingSymbolForTypeDash4Plastics => '♶',
            MiscellaneousSymbols::RecyclingSymbolForTypeDash5Plastics => '♷',
            MiscellaneousSymbols::RecyclingSymbolForTypeDash6Plastics => '♸',
            MiscellaneousSymbols::RecyclingSymbolForTypeDash7Plastics => '♹',
            MiscellaneousSymbols::RecyclingSymbolForGenericMaterials => '♺',
            MiscellaneousSymbols::BlackUniversalRecyclingSymbol => '♻',
            MiscellaneousSymbols::RecycledPaperSymbol => '♼',
            MiscellaneousSymbols::PartiallyDashRecycledPaperSymbol => '♽',
            MiscellaneousSymbols::PermanentPaperSign => '♾',
            MiscellaneousSymbols::WheelchairSymbol => '♿',
            MiscellaneousSymbols::DieFaceDash1 => '⚀',
            MiscellaneousSymbols::DieFaceDash2 => '⚁',
            MiscellaneousSymbols::DieFaceDash3 => '⚂',
            MiscellaneousSymbols::DieFaceDash4 => '⚃',
            MiscellaneousSymbols::DieFaceDash5 => '⚄',
            MiscellaneousSymbols::DieFaceDash6 => '⚅',
            MiscellaneousSymbols::WhiteCircleWithDotRight => '⚆',
            MiscellaneousSymbols::WhiteCircleWithTwoDots => '⚇',
            MiscellaneousSymbols::BlackCircleWithWhiteDotRight => '⚈',
            MiscellaneousSymbols::BlackCircleWithTwoWhiteDots => '⚉',
            MiscellaneousSymbols::MonogramForYang => '⚊',
            MiscellaneousSymbols::MonogramForYin => '⚋',
            MiscellaneousSymbols::DigramForGreaterYang => '⚌',
            MiscellaneousSymbols::DigramForLesserYin => '⚍',
            MiscellaneousSymbols::DigramForLesserYang => '⚎',
            MiscellaneousSymbols::DigramForGreaterYin => '⚏',
            MiscellaneousSymbols::WhiteFlag => '⚐',
            MiscellaneousSymbols::BlackFlag => '⚑',
            MiscellaneousSymbols::HammerAndPick => '⚒',
            MiscellaneousSymbols::Anchor => '⚓',
            MiscellaneousSymbols::CrossedSwords => '⚔',
            MiscellaneousSymbols::StaffOfAesculapius => '⚕',
            MiscellaneousSymbols::Scales => '⚖',
            MiscellaneousSymbols::Alembic => '⚗',
            MiscellaneousSymbols::Flower => '⚘',
            MiscellaneousSymbols::Gear => '⚙',
            MiscellaneousSymbols::StaffOfHermes => '⚚',
            MiscellaneousSymbols::AtomSymbol => '⚛',
            MiscellaneousSymbols::FleurDashDeDashLis => '⚜',
            MiscellaneousSymbols::OutlinedWhiteStar => '⚝',
            MiscellaneousSymbols::ThreeLinesConvergingRight => '⚞',
            MiscellaneousSymbols::ThreeLinesConvergingLeft => '⚟',
            MiscellaneousSymbols::WarningSign => '⚠',
            MiscellaneousSymbols::HighVoltageSign => '⚡',
            MiscellaneousSymbols::DoubledFemaleSign => '⚢',
            MiscellaneousSymbols::DoubledMaleSign => '⚣',
            MiscellaneousSymbols::InterlockedFemaleAndMaleSign => '⚤',
            MiscellaneousSymbols::MaleAndFemaleSign => '⚥',
            MiscellaneousSymbols::MaleWithStrokeSign => '⚦',
            MiscellaneousSymbols::MaleWithStrokeAndMaleAndFemaleSign => '⚧',
            MiscellaneousSymbols::VerticalMaleWithStrokeSign => '⚨',
            MiscellaneousSymbols::HorizontalMaleWithStrokeSign => '⚩',
            MiscellaneousSymbols::MediumWhiteCircle => '⚪',
            MiscellaneousSymbols::MediumBlackCircle => '⚫',
            MiscellaneousSymbols::MediumSmallWhiteCircle => '⚬',
            MiscellaneousSymbols::MarriageSymbol => '⚭',
            MiscellaneousSymbols::DivorceSymbol => '⚮',
            MiscellaneousSymbols::UnmarriedPartnershipSymbol => '⚯',
            MiscellaneousSymbols::Coffin => '⚰',
            MiscellaneousSymbols::FuneralUrn => '⚱',
            MiscellaneousSymbols::Neuter => '⚲',
            MiscellaneousSymbols::Ceres => '⚳',
            MiscellaneousSymbols::Pallas => '⚴',
            MiscellaneousSymbols::Juno => '⚵',
            MiscellaneousSymbols::Vesta => '⚶',
            MiscellaneousSymbols::Chiron => '⚷',
            MiscellaneousSymbols::BlackMoonLilith => '⚸',
            MiscellaneousSymbols::Sextile => '⚹',
            MiscellaneousSymbols::Semisextile => '⚺',
            MiscellaneousSymbols::Quincunx => '⚻',
            MiscellaneousSymbols::Sesquiquadrate => '⚼',
            MiscellaneousSymbols::SoccerBall => '⚽',
            MiscellaneousSymbols::Baseball => '⚾',
            MiscellaneousSymbols::SquaredKey => '⚿',
            MiscellaneousSymbols::WhiteDraughtsMan => '⛀',
            MiscellaneousSymbols::WhiteDraughtsKing => '⛁',
            MiscellaneousSymbols::BlackDraughtsMan => '⛂',
            MiscellaneousSymbols::BlackDraughtsKing => '⛃',
            MiscellaneousSymbols::SnowmanWithoutSnow => '⛄',
            MiscellaneousSymbols::SunBehindCloud => '⛅',
            MiscellaneousSymbols::Rain => '⛆',
            MiscellaneousSymbols::BlackSnowman => '⛇',
            MiscellaneousSymbols::ThunderCloudAndRain => '⛈',
            MiscellaneousSymbols::TurnedWhiteShogiPiece => '⛉',
            MiscellaneousSymbols::TurnedBlackShogiPiece => '⛊',
            MiscellaneousSymbols::WhiteDiamondInSquare => '⛋',
            MiscellaneousSymbols::CrossingLanes => '⛌',
            MiscellaneousSymbols::DisabledCar => '⛍',
            MiscellaneousSymbols::Ophiuchus => '⛎',
            MiscellaneousSymbols::Pick => '⛏',
            MiscellaneousSymbols::CarSliding => '⛐',
            MiscellaneousSymbols::HelmetWithWhiteCross => '⛑',
            MiscellaneousSymbols::CircledCrossingLanes => '⛒',
            MiscellaneousSymbols::Chains => '⛓',
            MiscellaneousSymbols::NoEntry => '⛔',
            MiscellaneousSymbols::AlternateOneDashWayLeftWayTraffic => '⛕',
            MiscellaneousSymbols::BlackTwoDashWayLeftWayTraffic => '⛖',
            MiscellaneousSymbols::WhiteTwoDashWayLeftWayTraffic => '⛗',
            MiscellaneousSymbols::BlackLeftLaneMerge => '⛘',
            MiscellaneousSymbols::WhiteLeftLaneMerge => '⛙',
            MiscellaneousSymbols::DriveSlowSign => '⛚',
            MiscellaneousSymbols::HeavyWhiteDownDashPointingTriangle => '⛛',
            MiscellaneousSymbols::LeftClosedEntry => '⛜',
            MiscellaneousSymbols::SquaredSaltire => '⛝',
            MiscellaneousSymbols::FallingDiagonalInWhiteCircleInBlackSquare => '⛞',
            MiscellaneousSymbols::BlackTruck => '⛟',
            MiscellaneousSymbols::RestrictedLeftEntryDash1 => '⛠',
            MiscellaneousSymbols::RestrictedLeftEntryDash2 => '⛡',
            MiscellaneousSymbols::AstronomicalSymbolForUranus => '⛢',
            MiscellaneousSymbols::HeavyCircleWithStrokeAndTwoDotsAbove => '⛣',
            MiscellaneousSymbols::Pentagram => '⛤',
            MiscellaneousSymbols::RightDashHandedInterlacedPentagram => '⛥',
            MiscellaneousSymbols::LeftDashHandedInterlacedPentagram => '⛦',
            MiscellaneousSymbols::InvertedPentagram => '⛧',
            MiscellaneousSymbols::BlackCrossOnShield => '⛨',
            MiscellaneousSymbols::ShintoShrine => '⛩',
            MiscellaneousSymbols::Church => '⛪',
            MiscellaneousSymbols::Castle => '⛫',
            MiscellaneousSymbols::HistoricSite => '⛬',
            MiscellaneousSymbols::GearWithoutHub => '⛭',
            MiscellaneousSymbols::GearWithHandles => '⛮',
            MiscellaneousSymbols::MapSymbolForLighthouse => '⛯',
            MiscellaneousSymbols::Mountain => '⛰',
            MiscellaneousSymbols::UmbrellaOnGround => '⛱',
            MiscellaneousSymbols::Fountain => '⛲',
            MiscellaneousSymbols::FlagInHole => '⛳',
            MiscellaneousSymbols::Ferry => '⛴',
            MiscellaneousSymbols::Sailboat => '⛵',
            MiscellaneousSymbols::SquareFourCorners => '⛶',
            MiscellaneousSymbols::Skier => '⛷',
            MiscellaneousSymbols::IceSkate => '⛸',
            MiscellaneousSymbols::PersonWithBall => '⛹',
            MiscellaneousSymbols::Tent => '⛺',
            MiscellaneousSymbols::JapaneseBankSymbol => '⛻',
            MiscellaneousSymbols::HeadstoneGraveyardSymbol => '⛼',
            MiscellaneousSymbols::FuelPump => '⛽',
            MiscellaneousSymbols::CupOnBlackSquare => '⛾',
        }
    }
}

impl std::convert::TryFrom<char> for MiscellaneousSymbols {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '☀' => Ok(MiscellaneousSymbols::BlackSunWithRays),
            '☁' => Ok(MiscellaneousSymbols::Cloud),
            '☂' => Ok(MiscellaneousSymbols::Umbrella),
            '☃' => Ok(MiscellaneousSymbols::Snowman),
            '☄' => Ok(MiscellaneousSymbols::Comet),
            '★' => Ok(MiscellaneousSymbols::BlackStar),
            '☆' => Ok(MiscellaneousSymbols::WhiteStar),
            '☇' => Ok(MiscellaneousSymbols::Lightning),
            '☈' => Ok(MiscellaneousSymbols::Thunderstorm),
            '☉' => Ok(MiscellaneousSymbols::Sun),
            '☊' => Ok(MiscellaneousSymbols::AscendingNode),
            '☋' => Ok(MiscellaneousSymbols::DescendingNode),
            '☌' => Ok(MiscellaneousSymbols::Conjunction),
            '☍' => Ok(MiscellaneousSymbols::Opposition),
            '☎' => Ok(MiscellaneousSymbols::BlackTelephone),
            '☏' => Ok(MiscellaneousSymbols::WhiteTelephone),
            '☐' => Ok(MiscellaneousSymbols::BallotBox),
            '☑' => Ok(MiscellaneousSymbols::BallotBoxWithCheck),
            '☒' => Ok(MiscellaneousSymbols::BallotBoxWithX),
            '☓' => Ok(MiscellaneousSymbols::Saltire),
            '☔' => Ok(MiscellaneousSymbols::UmbrellaWithRainDrops),
            '☕' => Ok(MiscellaneousSymbols::HotBeverage),
            '☖' => Ok(MiscellaneousSymbols::WhiteShogiPiece),
            '☗' => Ok(MiscellaneousSymbols::BlackShogiPiece),
            '☘' => Ok(MiscellaneousSymbols::Shamrock),
            '☙' => Ok(MiscellaneousSymbols::ReversedRotatedFloralHeartBullet),
            '☚' => Ok(MiscellaneousSymbols::BlackLeftPointingIndex),
            '☛' => Ok(MiscellaneousSymbols::BlackRightPointingIndex),
            '☜' => Ok(MiscellaneousSymbols::WhiteLeftPointingIndex),
            '☝' => Ok(MiscellaneousSymbols::WhiteUpPointingIndex),
            '☞' => Ok(MiscellaneousSymbols::WhiteRightPointingIndex),
            '☟' => Ok(MiscellaneousSymbols::WhiteDownPointingIndex),
            '☠' => Ok(MiscellaneousSymbols::SkullAndCrossbones),
            '☡' => Ok(MiscellaneousSymbols::CautionSign),
            '☢' => Ok(MiscellaneousSymbols::RadioactiveSign),
            '☣' => Ok(MiscellaneousSymbols::BiohazardSign),
            '☤' => Ok(MiscellaneousSymbols::Caduceus),
            '☥' => Ok(MiscellaneousSymbols::Ankh),
            '☦' => Ok(MiscellaneousSymbols::OrthodoxCross),
            '☧' => Ok(MiscellaneousSymbols::ChiRho),
            '☨' => Ok(MiscellaneousSymbols::CrossOfLorraine),
            '☩' => Ok(MiscellaneousSymbols::CrossOfJerusalem),
            '☪' => Ok(MiscellaneousSymbols::StarAndCrescent),
            '☫' => Ok(MiscellaneousSymbols::FarsiSymbol),
            '☬' => Ok(MiscellaneousSymbols::AdiShakti),
            '☭' => Ok(MiscellaneousSymbols::HammerAndSickle),
            '☮' => Ok(MiscellaneousSymbols::PeaceSymbol),
            '☯' => Ok(MiscellaneousSymbols::YinYang),
            '☰' => Ok(MiscellaneousSymbols::TrigramForHeaven),
            '☱' => Ok(MiscellaneousSymbols::TrigramForLake),
            '☲' => Ok(MiscellaneousSymbols::TrigramForFire),
            '☳' => Ok(MiscellaneousSymbols::TrigramForThunder),
            '☴' => Ok(MiscellaneousSymbols::TrigramForWind),
            '☵' => Ok(MiscellaneousSymbols::TrigramForWater),
            '☶' => Ok(MiscellaneousSymbols::TrigramForMountain),
            '☷' => Ok(MiscellaneousSymbols::TrigramForEarth),
            '☸' => Ok(MiscellaneousSymbols::WheelOfDharma),
            '☹' => Ok(MiscellaneousSymbols::WhiteFrowningFace),
            '☺' => Ok(MiscellaneousSymbols::WhiteSmilingFace),
            '☻' => Ok(MiscellaneousSymbols::BlackSmilingFace),
            '☼' => Ok(MiscellaneousSymbols::WhiteSunWithRays),
            '☽' => Ok(MiscellaneousSymbols::FirstQuarterMoon),
            '☾' => Ok(MiscellaneousSymbols::LastQuarterMoon),
            '☿' => Ok(MiscellaneousSymbols::Mercury),
            '♀' => Ok(MiscellaneousSymbols::FemaleSign),
            '♁' => Ok(MiscellaneousSymbols::Earth),
            '♂' => Ok(MiscellaneousSymbols::MaleSign),
            '♃' => Ok(MiscellaneousSymbols::Jupiter),
            '♄' => Ok(MiscellaneousSymbols::Saturn),
            '♅' => Ok(MiscellaneousSymbols::Uranus),
            '♆' => Ok(MiscellaneousSymbols::Neptune),
            '♇' => Ok(MiscellaneousSymbols::Pluto),
            '♈' => Ok(MiscellaneousSymbols::Aries),
            '♉' => Ok(MiscellaneousSymbols::Taurus),
            '♊' => Ok(MiscellaneousSymbols::Gemini),
            '♋' => Ok(MiscellaneousSymbols::Cancer),
            '♌' => Ok(MiscellaneousSymbols::Leo),
            '♍' => Ok(MiscellaneousSymbols::Virgo),
            '♎' => Ok(MiscellaneousSymbols::Libra),
            '♏' => Ok(MiscellaneousSymbols::Scorpius),
            '♐' => Ok(MiscellaneousSymbols::Sagittarius),
            '♑' => Ok(MiscellaneousSymbols::Capricorn),
            '♒' => Ok(MiscellaneousSymbols::Aquarius),
            '♓' => Ok(MiscellaneousSymbols::Pisces),
            '♔' => Ok(MiscellaneousSymbols::WhiteChessKing),
            '♕' => Ok(MiscellaneousSymbols::WhiteChessQueen),
            '♖' => Ok(MiscellaneousSymbols::WhiteChessRook),
            '♗' => Ok(MiscellaneousSymbols::WhiteChessBishop),
            '♘' => Ok(MiscellaneousSymbols::WhiteChessKnight),
            '♙' => Ok(MiscellaneousSymbols::WhiteChessPawn),
            '♚' => Ok(MiscellaneousSymbols::BlackChessKing),
            '♛' => Ok(MiscellaneousSymbols::BlackChessQueen),
            '♜' => Ok(MiscellaneousSymbols::BlackChessRook),
            '♝' => Ok(MiscellaneousSymbols::BlackChessBishop),
            '♞' => Ok(MiscellaneousSymbols::BlackChessKnight),
            '♟' => Ok(MiscellaneousSymbols::BlackChessPawn),
            '♠' => Ok(MiscellaneousSymbols::BlackSpadeSuit),
            '♡' => Ok(MiscellaneousSymbols::WhiteHeartSuit),
            '♢' => Ok(MiscellaneousSymbols::WhiteDiamondSuit),
            '♣' => Ok(MiscellaneousSymbols::BlackClubSuit),
            '♤' => Ok(MiscellaneousSymbols::WhiteSpadeSuit),
            '♥' => Ok(MiscellaneousSymbols::BlackHeartSuit),
            '♦' => Ok(MiscellaneousSymbols::BlackDiamondSuit),
            '♧' => Ok(MiscellaneousSymbols::WhiteClubSuit),
            '♨' => Ok(MiscellaneousSymbols::HotSprings),
            '♩' => Ok(MiscellaneousSymbols::QuarterNote),
            '♪' => Ok(MiscellaneousSymbols::EighthNote),
            '♫' => Ok(MiscellaneousSymbols::BeamedEighthNotes),
            '♬' => Ok(MiscellaneousSymbols::BeamedSixteenthNotes),
            '♭' => Ok(MiscellaneousSymbols::MusicFlatSign),
            '♮' => Ok(MiscellaneousSymbols::MusicNaturalSign),
            '♯' => Ok(MiscellaneousSymbols::MusicSharpSign),
            '♰' => Ok(MiscellaneousSymbols::WestSyriacCross),
            '♱' => Ok(MiscellaneousSymbols::EastSyriacCross),
            '♲' => Ok(MiscellaneousSymbols::UniversalRecyclingSymbol),
            '♳' => Ok(MiscellaneousSymbols::RecyclingSymbolForTypeDash1Plastics),
            '♴' => Ok(MiscellaneousSymbols::RecyclingSymbolForTypeDash2Plastics),
            '♵' => Ok(MiscellaneousSymbols::RecyclingSymbolForTypeDash3Plastics),
            '♶' => Ok(MiscellaneousSymbols::RecyclingSymbolForTypeDash4Plastics),
            '♷' => Ok(MiscellaneousSymbols::RecyclingSymbolForTypeDash5Plastics),
            '♸' => Ok(MiscellaneousSymbols::RecyclingSymbolForTypeDash6Plastics),
            '♹' => Ok(MiscellaneousSymbols::RecyclingSymbolForTypeDash7Plastics),
            '♺' => Ok(MiscellaneousSymbols::RecyclingSymbolForGenericMaterials),
            '♻' => Ok(MiscellaneousSymbols::BlackUniversalRecyclingSymbol),
            '♼' => Ok(MiscellaneousSymbols::RecycledPaperSymbol),
            '♽' => Ok(MiscellaneousSymbols::PartiallyDashRecycledPaperSymbol),
            '♾' => Ok(MiscellaneousSymbols::PermanentPaperSign),
            '♿' => Ok(MiscellaneousSymbols::WheelchairSymbol),
            '⚀' => Ok(MiscellaneousSymbols::DieFaceDash1),
            '⚁' => Ok(MiscellaneousSymbols::DieFaceDash2),
            '⚂' => Ok(MiscellaneousSymbols::DieFaceDash3),
            '⚃' => Ok(MiscellaneousSymbols::DieFaceDash4),
            '⚄' => Ok(MiscellaneousSymbols::DieFaceDash5),
            '⚅' => Ok(MiscellaneousSymbols::DieFaceDash6),
            '⚆' => Ok(MiscellaneousSymbols::WhiteCircleWithDotRight),
            '⚇' => Ok(MiscellaneousSymbols::WhiteCircleWithTwoDots),
            '⚈' => Ok(MiscellaneousSymbols::BlackCircleWithWhiteDotRight),
            '⚉' => Ok(MiscellaneousSymbols::BlackCircleWithTwoWhiteDots),
            '⚊' => Ok(MiscellaneousSymbols::MonogramForYang),
            '⚋' => Ok(MiscellaneousSymbols::MonogramForYin),
            '⚌' => Ok(MiscellaneousSymbols::DigramForGreaterYang),
            '⚍' => Ok(MiscellaneousSymbols::DigramForLesserYin),
            '⚎' => Ok(MiscellaneousSymbols::DigramForLesserYang),
            '⚏' => Ok(MiscellaneousSymbols::DigramForGreaterYin),
            '⚐' => Ok(MiscellaneousSymbols::WhiteFlag),
            '⚑' => Ok(MiscellaneousSymbols::BlackFlag),
            '⚒' => Ok(MiscellaneousSymbols::HammerAndPick),
            '⚓' => Ok(MiscellaneousSymbols::Anchor),
            '⚔' => Ok(MiscellaneousSymbols::CrossedSwords),
            '⚕' => Ok(MiscellaneousSymbols::StaffOfAesculapius),
            '⚖' => Ok(MiscellaneousSymbols::Scales),
            '⚗' => Ok(MiscellaneousSymbols::Alembic),
            '⚘' => Ok(MiscellaneousSymbols::Flower),
            '⚙' => Ok(MiscellaneousSymbols::Gear),
            '⚚' => Ok(MiscellaneousSymbols::StaffOfHermes),
            '⚛' => Ok(MiscellaneousSymbols::AtomSymbol),
            '⚜' => Ok(MiscellaneousSymbols::FleurDashDeDashLis),
            '⚝' => Ok(MiscellaneousSymbols::OutlinedWhiteStar),
            '⚞' => Ok(MiscellaneousSymbols::ThreeLinesConvergingRight),
            '⚟' => Ok(MiscellaneousSymbols::ThreeLinesConvergingLeft),
            '⚠' => Ok(MiscellaneousSymbols::WarningSign),
            '⚡' => Ok(MiscellaneousSymbols::HighVoltageSign),
            '⚢' => Ok(MiscellaneousSymbols::DoubledFemaleSign),
            '⚣' => Ok(MiscellaneousSymbols::DoubledMaleSign),
            '⚤' => Ok(MiscellaneousSymbols::InterlockedFemaleAndMaleSign),
            '⚥' => Ok(MiscellaneousSymbols::MaleAndFemaleSign),
            '⚦' => Ok(MiscellaneousSymbols::MaleWithStrokeSign),
            '⚧' => Ok(MiscellaneousSymbols::MaleWithStrokeAndMaleAndFemaleSign),
            '⚨' => Ok(MiscellaneousSymbols::VerticalMaleWithStrokeSign),
            '⚩' => Ok(MiscellaneousSymbols::HorizontalMaleWithStrokeSign),
            '⚪' => Ok(MiscellaneousSymbols::MediumWhiteCircle),
            '⚫' => Ok(MiscellaneousSymbols::MediumBlackCircle),
            '⚬' => Ok(MiscellaneousSymbols::MediumSmallWhiteCircle),
            '⚭' => Ok(MiscellaneousSymbols::MarriageSymbol),
            '⚮' => Ok(MiscellaneousSymbols::DivorceSymbol),
            '⚯' => Ok(MiscellaneousSymbols::UnmarriedPartnershipSymbol),
            '⚰' => Ok(MiscellaneousSymbols::Coffin),
            '⚱' => Ok(MiscellaneousSymbols::FuneralUrn),
            '⚲' => Ok(MiscellaneousSymbols::Neuter),
            '⚳' => Ok(MiscellaneousSymbols::Ceres),
            '⚴' => Ok(MiscellaneousSymbols::Pallas),
            '⚵' => Ok(MiscellaneousSymbols::Juno),
            '⚶' => Ok(MiscellaneousSymbols::Vesta),
            '⚷' => Ok(MiscellaneousSymbols::Chiron),
            '⚸' => Ok(MiscellaneousSymbols::BlackMoonLilith),
            '⚹' => Ok(MiscellaneousSymbols::Sextile),
            '⚺' => Ok(MiscellaneousSymbols::Semisextile),
            '⚻' => Ok(MiscellaneousSymbols::Quincunx),
            '⚼' => Ok(MiscellaneousSymbols::Sesquiquadrate),
            '⚽' => Ok(MiscellaneousSymbols::SoccerBall),
            '⚾' => Ok(MiscellaneousSymbols::Baseball),
            '⚿' => Ok(MiscellaneousSymbols::SquaredKey),
            '⛀' => Ok(MiscellaneousSymbols::WhiteDraughtsMan),
            '⛁' => Ok(MiscellaneousSymbols::WhiteDraughtsKing),
            '⛂' => Ok(MiscellaneousSymbols::BlackDraughtsMan),
            '⛃' => Ok(MiscellaneousSymbols::BlackDraughtsKing),
            '⛄' => Ok(MiscellaneousSymbols::SnowmanWithoutSnow),
            '⛅' => Ok(MiscellaneousSymbols::SunBehindCloud),
            '⛆' => Ok(MiscellaneousSymbols::Rain),
            '⛇' => Ok(MiscellaneousSymbols::BlackSnowman),
            '⛈' => Ok(MiscellaneousSymbols::ThunderCloudAndRain),
            '⛉' => Ok(MiscellaneousSymbols::TurnedWhiteShogiPiece),
            '⛊' => Ok(MiscellaneousSymbols::TurnedBlackShogiPiece),
            '⛋' => Ok(MiscellaneousSymbols::WhiteDiamondInSquare),
            '⛌' => Ok(MiscellaneousSymbols::CrossingLanes),
            '⛍' => Ok(MiscellaneousSymbols::DisabledCar),
            '⛎' => Ok(MiscellaneousSymbols::Ophiuchus),
            '⛏' => Ok(MiscellaneousSymbols::Pick),
            '⛐' => Ok(MiscellaneousSymbols::CarSliding),
            '⛑' => Ok(MiscellaneousSymbols::HelmetWithWhiteCross),
            '⛒' => Ok(MiscellaneousSymbols::CircledCrossingLanes),
            '⛓' => Ok(MiscellaneousSymbols::Chains),
            '⛔' => Ok(MiscellaneousSymbols::NoEntry),
            '⛕' => Ok(MiscellaneousSymbols::AlternateOneDashWayLeftWayTraffic),
            '⛖' => Ok(MiscellaneousSymbols::BlackTwoDashWayLeftWayTraffic),
            '⛗' => Ok(MiscellaneousSymbols::WhiteTwoDashWayLeftWayTraffic),
            '⛘' => Ok(MiscellaneousSymbols::BlackLeftLaneMerge),
            '⛙' => Ok(MiscellaneousSymbols::WhiteLeftLaneMerge),
            '⛚' => Ok(MiscellaneousSymbols::DriveSlowSign),
            '⛛' => Ok(MiscellaneousSymbols::HeavyWhiteDownDashPointingTriangle),
            '⛜' => Ok(MiscellaneousSymbols::LeftClosedEntry),
            '⛝' => Ok(MiscellaneousSymbols::SquaredSaltire),
            '⛞' => Ok(MiscellaneousSymbols::FallingDiagonalInWhiteCircleInBlackSquare),
            '⛟' => Ok(MiscellaneousSymbols::BlackTruck),
            '⛠' => Ok(MiscellaneousSymbols::RestrictedLeftEntryDash1),
            '⛡' => Ok(MiscellaneousSymbols::RestrictedLeftEntryDash2),
            '⛢' => Ok(MiscellaneousSymbols::AstronomicalSymbolForUranus),
            '⛣' => Ok(MiscellaneousSymbols::HeavyCircleWithStrokeAndTwoDotsAbove),
            '⛤' => Ok(MiscellaneousSymbols::Pentagram),
            '⛥' => Ok(MiscellaneousSymbols::RightDashHandedInterlacedPentagram),
            '⛦' => Ok(MiscellaneousSymbols::LeftDashHandedInterlacedPentagram),
            '⛧' => Ok(MiscellaneousSymbols::InvertedPentagram),
            '⛨' => Ok(MiscellaneousSymbols::BlackCrossOnShield),
            '⛩' => Ok(MiscellaneousSymbols::ShintoShrine),
            '⛪' => Ok(MiscellaneousSymbols::Church),
            '⛫' => Ok(MiscellaneousSymbols::Castle),
            '⛬' => Ok(MiscellaneousSymbols::HistoricSite),
            '⛭' => Ok(MiscellaneousSymbols::GearWithoutHub),
            '⛮' => Ok(MiscellaneousSymbols::GearWithHandles),
            '⛯' => Ok(MiscellaneousSymbols::MapSymbolForLighthouse),
            '⛰' => Ok(MiscellaneousSymbols::Mountain),
            '⛱' => Ok(MiscellaneousSymbols::UmbrellaOnGround),
            '⛲' => Ok(MiscellaneousSymbols::Fountain),
            '⛳' => Ok(MiscellaneousSymbols::FlagInHole),
            '⛴' => Ok(MiscellaneousSymbols::Ferry),
            '⛵' => Ok(MiscellaneousSymbols::Sailboat),
            '⛶' => Ok(MiscellaneousSymbols::SquareFourCorners),
            '⛷' => Ok(MiscellaneousSymbols::Skier),
            '⛸' => Ok(MiscellaneousSymbols::IceSkate),
            '⛹' => Ok(MiscellaneousSymbols::PersonWithBall),
            '⛺' => Ok(MiscellaneousSymbols::Tent),
            '⛻' => Ok(MiscellaneousSymbols::JapaneseBankSymbol),
            '⛼' => Ok(MiscellaneousSymbols::HeadstoneGraveyardSymbol),
            '⛽' => Ok(MiscellaneousSymbols::FuelPump),
            '⛾' => Ok(MiscellaneousSymbols::CupOnBlackSquare),
            _ => Err(()),
        }
    }
}

impl Into<u32> for MiscellaneousSymbols {
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

impl std::convert::TryFrom<u32> for MiscellaneousSymbols {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for MiscellaneousSymbols {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl MiscellaneousSymbols {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        MiscellaneousSymbols::BlackSunWithRays
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            MiscellaneousSymbols::BlackSunWithRays => "black sun with rays",
            MiscellaneousSymbols::Cloud => "cloud",
            MiscellaneousSymbols::Umbrella => "umbrella",
            MiscellaneousSymbols::Snowman => "snowman",
            MiscellaneousSymbols::Comet => "comet",
            MiscellaneousSymbols::BlackStar => "black star",
            MiscellaneousSymbols::WhiteStar => "white star",
            MiscellaneousSymbols::Lightning => "lightning",
            MiscellaneousSymbols::Thunderstorm => "thunderstorm",
            MiscellaneousSymbols::Sun => "sun",
            MiscellaneousSymbols::AscendingNode => "ascending node",
            MiscellaneousSymbols::DescendingNode => "descending node",
            MiscellaneousSymbols::Conjunction => "conjunction",
            MiscellaneousSymbols::Opposition => "opposition",
            MiscellaneousSymbols::BlackTelephone => "black telephone",
            MiscellaneousSymbols::WhiteTelephone => "white telephone",
            MiscellaneousSymbols::BallotBox => "ballot box",
            MiscellaneousSymbols::BallotBoxWithCheck => "ballot box with check",
            MiscellaneousSymbols::BallotBoxWithX => "ballot box with x",
            MiscellaneousSymbols::Saltire => "saltire",
            MiscellaneousSymbols::UmbrellaWithRainDrops => "umbrella with rain drops",
            MiscellaneousSymbols::HotBeverage => "hot beverage",
            MiscellaneousSymbols::WhiteShogiPiece => "white shogi piece",
            MiscellaneousSymbols::BlackShogiPiece => "black shogi piece",
            MiscellaneousSymbols::Shamrock => "shamrock",
            MiscellaneousSymbols::ReversedRotatedFloralHeartBullet => "reversed rotated floral heart bullet",
            MiscellaneousSymbols::BlackLeftPointingIndex => "black left pointing index",
            MiscellaneousSymbols::BlackRightPointingIndex => "black right pointing index",
            MiscellaneousSymbols::WhiteLeftPointingIndex => "white left pointing index",
            MiscellaneousSymbols::WhiteUpPointingIndex => "white up pointing index",
            MiscellaneousSymbols::WhiteRightPointingIndex => "white right pointing index",
            MiscellaneousSymbols::WhiteDownPointingIndex => "white down pointing index",
            MiscellaneousSymbols::SkullAndCrossbones => "skull and crossbones",
            MiscellaneousSymbols::CautionSign => "caution sign",
            MiscellaneousSymbols::RadioactiveSign => "radioactive sign",
            MiscellaneousSymbols::BiohazardSign => "biohazard sign",
            MiscellaneousSymbols::Caduceus => "caduceus",
            MiscellaneousSymbols::Ankh => "ankh",
            MiscellaneousSymbols::OrthodoxCross => "orthodox cross",
            MiscellaneousSymbols::ChiRho => "chi rho",
            MiscellaneousSymbols::CrossOfLorraine => "cross of lorraine",
            MiscellaneousSymbols::CrossOfJerusalem => "cross of jerusalem",
            MiscellaneousSymbols::StarAndCrescent => "star and crescent",
            MiscellaneousSymbols::FarsiSymbol => "farsi symbol",
            MiscellaneousSymbols::AdiShakti => "adi shakti",
            MiscellaneousSymbols::HammerAndSickle => "hammer and sickle",
            MiscellaneousSymbols::PeaceSymbol => "peace symbol",
            MiscellaneousSymbols::YinYang => "yin yang",
            MiscellaneousSymbols::TrigramForHeaven => "trigram for heaven",
            MiscellaneousSymbols::TrigramForLake => "trigram for lake",
            MiscellaneousSymbols::TrigramForFire => "trigram for fire",
            MiscellaneousSymbols::TrigramForThunder => "trigram for thunder",
            MiscellaneousSymbols::TrigramForWind => "trigram for wind",
            MiscellaneousSymbols::TrigramForWater => "trigram for water",
            MiscellaneousSymbols::TrigramForMountain => "trigram for mountain",
            MiscellaneousSymbols::TrigramForEarth => "trigram for earth",
            MiscellaneousSymbols::WheelOfDharma => "wheel of dharma",
            MiscellaneousSymbols::WhiteFrowningFace => "white frowning face",
            MiscellaneousSymbols::WhiteSmilingFace => "white smiling face",
            MiscellaneousSymbols::BlackSmilingFace => "black smiling face",
            MiscellaneousSymbols::WhiteSunWithRays => "white sun with rays",
            MiscellaneousSymbols::FirstQuarterMoon => "first quarter moon",
            MiscellaneousSymbols::LastQuarterMoon => "last quarter moon",
            MiscellaneousSymbols::Mercury => "mercury",
            MiscellaneousSymbols::FemaleSign => "female sign",
            MiscellaneousSymbols::Earth => "earth",
            MiscellaneousSymbols::MaleSign => "male sign",
            MiscellaneousSymbols::Jupiter => "jupiter",
            MiscellaneousSymbols::Saturn => "saturn",
            MiscellaneousSymbols::Uranus => "uranus",
            MiscellaneousSymbols::Neptune => "neptune",
            MiscellaneousSymbols::Pluto => "pluto",
            MiscellaneousSymbols::Aries => "aries",
            MiscellaneousSymbols::Taurus => "taurus",
            MiscellaneousSymbols::Gemini => "gemini",
            MiscellaneousSymbols::Cancer => "cancer",
            MiscellaneousSymbols::Leo => "leo",
            MiscellaneousSymbols::Virgo => "virgo",
            MiscellaneousSymbols::Libra => "libra",
            MiscellaneousSymbols::Scorpius => "scorpius",
            MiscellaneousSymbols::Sagittarius => "sagittarius",
            MiscellaneousSymbols::Capricorn => "capricorn",
            MiscellaneousSymbols::Aquarius => "aquarius",
            MiscellaneousSymbols::Pisces => "pisces",
            MiscellaneousSymbols::WhiteChessKing => "white chess king",
            MiscellaneousSymbols::WhiteChessQueen => "white chess queen",
            MiscellaneousSymbols::WhiteChessRook => "white chess rook",
            MiscellaneousSymbols::WhiteChessBishop => "white chess bishop",
            MiscellaneousSymbols::WhiteChessKnight => "white chess knight",
            MiscellaneousSymbols::WhiteChessPawn => "white chess pawn",
            MiscellaneousSymbols::BlackChessKing => "black chess king",
            MiscellaneousSymbols::BlackChessQueen => "black chess queen",
            MiscellaneousSymbols::BlackChessRook => "black chess rook",
            MiscellaneousSymbols::BlackChessBishop => "black chess bishop",
            MiscellaneousSymbols::BlackChessKnight => "black chess knight",
            MiscellaneousSymbols::BlackChessPawn => "black chess pawn",
            MiscellaneousSymbols::BlackSpadeSuit => "black spade suit",
            MiscellaneousSymbols::WhiteHeartSuit => "white heart suit",
            MiscellaneousSymbols::WhiteDiamondSuit => "white diamond suit",
            MiscellaneousSymbols::BlackClubSuit => "black club suit",
            MiscellaneousSymbols::WhiteSpadeSuit => "white spade suit",
            MiscellaneousSymbols::BlackHeartSuit => "black heart suit",
            MiscellaneousSymbols::BlackDiamondSuit => "black diamond suit",
            MiscellaneousSymbols::WhiteClubSuit => "white club suit",
            MiscellaneousSymbols::HotSprings => "hot springs",
            MiscellaneousSymbols::QuarterNote => "quarter note",
            MiscellaneousSymbols::EighthNote => "eighth note",
            MiscellaneousSymbols::BeamedEighthNotes => "beamed eighth notes",
            MiscellaneousSymbols::BeamedSixteenthNotes => "beamed sixteenth notes",
            MiscellaneousSymbols::MusicFlatSign => "music flat sign",
            MiscellaneousSymbols::MusicNaturalSign => "music natural sign",
            MiscellaneousSymbols::MusicSharpSign => "music sharp sign",
            MiscellaneousSymbols::WestSyriacCross => "west syriac cross",
            MiscellaneousSymbols::EastSyriacCross => "east syriac cross",
            MiscellaneousSymbols::UniversalRecyclingSymbol => "universal recycling symbol",
            MiscellaneousSymbols::RecyclingSymbolForTypeDash1Plastics => "recycling symbol for type-1 plastics",
            MiscellaneousSymbols::RecyclingSymbolForTypeDash2Plastics => "recycling symbol for type-2 plastics",
            MiscellaneousSymbols::RecyclingSymbolForTypeDash3Plastics => "recycling symbol for type-3 plastics",
            MiscellaneousSymbols::RecyclingSymbolForTypeDash4Plastics => "recycling symbol for type-4 plastics",
            MiscellaneousSymbols::RecyclingSymbolForTypeDash5Plastics => "recycling symbol for type-5 plastics",
            MiscellaneousSymbols::RecyclingSymbolForTypeDash6Plastics => "recycling symbol for type-6 plastics",
            MiscellaneousSymbols::RecyclingSymbolForTypeDash7Plastics => "recycling symbol for type-7 plastics",
            MiscellaneousSymbols::RecyclingSymbolForGenericMaterials => "recycling symbol for generic materials",
            MiscellaneousSymbols::BlackUniversalRecyclingSymbol => "black universal recycling symbol",
            MiscellaneousSymbols::RecycledPaperSymbol => "recycled paper symbol",
            MiscellaneousSymbols::PartiallyDashRecycledPaperSymbol => "partially-recycled paper symbol",
            MiscellaneousSymbols::PermanentPaperSign => "permanent paper sign",
            MiscellaneousSymbols::WheelchairSymbol => "wheelchair symbol",
            MiscellaneousSymbols::DieFaceDash1 => "die face-1",
            MiscellaneousSymbols::DieFaceDash2 => "die face-2",
            MiscellaneousSymbols::DieFaceDash3 => "die face-3",
            MiscellaneousSymbols::DieFaceDash4 => "die face-4",
            MiscellaneousSymbols::DieFaceDash5 => "die face-5",
            MiscellaneousSymbols::DieFaceDash6 => "die face-6",
            MiscellaneousSymbols::WhiteCircleWithDotRight => "white circle with dot right",
            MiscellaneousSymbols::WhiteCircleWithTwoDots => "white circle with two dots",
            MiscellaneousSymbols::BlackCircleWithWhiteDotRight => "black circle with white dot right",
            MiscellaneousSymbols::BlackCircleWithTwoWhiteDots => "black circle with two white dots",
            MiscellaneousSymbols::MonogramForYang => "monogram for yang",
            MiscellaneousSymbols::MonogramForYin => "monogram for yin",
            MiscellaneousSymbols::DigramForGreaterYang => "digram for greater yang",
            MiscellaneousSymbols::DigramForLesserYin => "digram for lesser yin",
            MiscellaneousSymbols::DigramForLesserYang => "digram for lesser yang",
            MiscellaneousSymbols::DigramForGreaterYin => "digram for greater yin",
            MiscellaneousSymbols::WhiteFlag => "white flag",
            MiscellaneousSymbols::BlackFlag => "black flag",
            MiscellaneousSymbols::HammerAndPick => "hammer and pick",
            MiscellaneousSymbols::Anchor => "anchor",
            MiscellaneousSymbols::CrossedSwords => "crossed swords",
            MiscellaneousSymbols::StaffOfAesculapius => "staff of aesculapius",
            MiscellaneousSymbols::Scales => "scales",
            MiscellaneousSymbols::Alembic => "alembic",
            MiscellaneousSymbols::Flower => "flower",
            MiscellaneousSymbols::Gear => "gear",
            MiscellaneousSymbols::StaffOfHermes => "staff of hermes",
            MiscellaneousSymbols::AtomSymbol => "atom symbol",
            MiscellaneousSymbols::FleurDashDeDashLis => "fleur-de-lis",
            MiscellaneousSymbols::OutlinedWhiteStar => "outlined white star",
            MiscellaneousSymbols::ThreeLinesConvergingRight => "three lines converging right",
            MiscellaneousSymbols::ThreeLinesConvergingLeft => "three lines converging left",
            MiscellaneousSymbols::WarningSign => "warning sign",
            MiscellaneousSymbols::HighVoltageSign => "high voltage sign",
            MiscellaneousSymbols::DoubledFemaleSign => "doubled female sign",
            MiscellaneousSymbols::DoubledMaleSign => "doubled male sign",
            MiscellaneousSymbols::InterlockedFemaleAndMaleSign => "interlocked female and male sign",
            MiscellaneousSymbols::MaleAndFemaleSign => "male and female sign",
            MiscellaneousSymbols::MaleWithStrokeSign => "male with stroke sign",
            MiscellaneousSymbols::MaleWithStrokeAndMaleAndFemaleSign => "male with stroke and male and female sign",
            MiscellaneousSymbols::VerticalMaleWithStrokeSign => "vertical male with stroke sign",
            MiscellaneousSymbols::HorizontalMaleWithStrokeSign => "horizontal male with stroke sign",
            MiscellaneousSymbols::MediumWhiteCircle => "medium white circle",
            MiscellaneousSymbols::MediumBlackCircle => "medium black circle",
            MiscellaneousSymbols::MediumSmallWhiteCircle => "medium small white circle",
            MiscellaneousSymbols::MarriageSymbol => "marriage symbol",
            MiscellaneousSymbols::DivorceSymbol => "divorce symbol",
            MiscellaneousSymbols::UnmarriedPartnershipSymbol => "unmarried partnership symbol",
            MiscellaneousSymbols::Coffin => "coffin",
            MiscellaneousSymbols::FuneralUrn => "funeral urn",
            MiscellaneousSymbols::Neuter => "neuter",
            MiscellaneousSymbols::Ceres => "ceres",
            MiscellaneousSymbols::Pallas => "pallas",
            MiscellaneousSymbols::Juno => "juno",
            MiscellaneousSymbols::Vesta => "vesta",
            MiscellaneousSymbols::Chiron => "chiron",
            MiscellaneousSymbols::BlackMoonLilith => "black moon lilith",
            MiscellaneousSymbols::Sextile => "sextile",
            MiscellaneousSymbols::Semisextile => "semisextile",
            MiscellaneousSymbols::Quincunx => "quincunx",
            MiscellaneousSymbols::Sesquiquadrate => "sesquiquadrate",
            MiscellaneousSymbols::SoccerBall => "soccer ball",
            MiscellaneousSymbols::Baseball => "baseball",
            MiscellaneousSymbols::SquaredKey => "squared key",
            MiscellaneousSymbols::WhiteDraughtsMan => "white draughts man",
            MiscellaneousSymbols::WhiteDraughtsKing => "white draughts king",
            MiscellaneousSymbols::BlackDraughtsMan => "black draughts man",
            MiscellaneousSymbols::BlackDraughtsKing => "black draughts king",
            MiscellaneousSymbols::SnowmanWithoutSnow => "snowman without snow",
            MiscellaneousSymbols::SunBehindCloud => "sun behind cloud",
            MiscellaneousSymbols::Rain => "rain",
            MiscellaneousSymbols::BlackSnowman => "black snowman",
            MiscellaneousSymbols::ThunderCloudAndRain => "thunder cloud and rain",
            MiscellaneousSymbols::TurnedWhiteShogiPiece => "turned white shogi piece",
            MiscellaneousSymbols::TurnedBlackShogiPiece => "turned black shogi piece",
            MiscellaneousSymbols::WhiteDiamondInSquare => "white diamond in square",
            MiscellaneousSymbols::CrossingLanes => "crossing lanes",
            MiscellaneousSymbols::DisabledCar => "disabled car",
            MiscellaneousSymbols::Ophiuchus => "ophiuchus",
            MiscellaneousSymbols::Pick => "pick",
            MiscellaneousSymbols::CarSliding => "car sliding",
            MiscellaneousSymbols::HelmetWithWhiteCross => "helmet with white cross",
            MiscellaneousSymbols::CircledCrossingLanes => "circled crossing lanes",
            MiscellaneousSymbols::Chains => "chains",
            MiscellaneousSymbols::NoEntry => "no entry",
            MiscellaneousSymbols::AlternateOneDashWayLeftWayTraffic => "alternate one-way left way traffic",
            MiscellaneousSymbols::BlackTwoDashWayLeftWayTraffic => "black two-way left way traffic",
            MiscellaneousSymbols::WhiteTwoDashWayLeftWayTraffic => "white two-way left way traffic",
            MiscellaneousSymbols::BlackLeftLaneMerge => "black left lane merge",
            MiscellaneousSymbols::WhiteLeftLaneMerge => "white left lane merge",
            MiscellaneousSymbols::DriveSlowSign => "drive slow sign",
            MiscellaneousSymbols::HeavyWhiteDownDashPointingTriangle => "heavy white down-pointing triangle",
            MiscellaneousSymbols::LeftClosedEntry => "left closed entry",
            MiscellaneousSymbols::SquaredSaltire => "squared saltire",
            MiscellaneousSymbols::FallingDiagonalInWhiteCircleInBlackSquare => "falling diagonal in white circle in black square",
            MiscellaneousSymbols::BlackTruck => "black truck",
            MiscellaneousSymbols::RestrictedLeftEntryDash1 => "restricted left entry-1",
            MiscellaneousSymbols::RestrictedLeftEntryDash2 => "restricted left entry-2",
            MiscellaneousSymbols::AstronomicalSymbolForUranus => "astronomical symbol for uranus",
            MiscellaneousSymbols::HeavyCircleWithStrokeAndTwoDotsAbove => "heavy circle with stroke and two dots above",
            MiscellaneousSymbols::Pentagram => "pentagram",
            MiscellaneousSymbols::RightDashHandedInterlacedPentagram => "right-handed interlaced pentagram",
            MiscellaneousSymbols::LeftDashHandedInterlacedPentagram => "left-handed interlaced pentagram",
            MiscellaneousSymbols::InvertedPentagram => "inverted pentagram",
            MiscellaneousSymbols::BlackCrossOnShield => "black cross on shield",
            MiscellaneousSymbols::ShintoShrine => "shinto shrine",
            MiscellaneousSymbols::Church => "church",
            MiscellaneousSymbols::Castle => "castle",
            MiscellaneousSymbols::HistoricSite => "historic site",
            MiscellaneousSymbols::GearWithoutHub => "gear without hub",
            MiscellaneousSymbols::GearWithHandles => "gear with handles",
            MiscellaneousSymbols::MapSymbolForLighthouse => "map symbol for lighthouse",
            MiscellaneousSymbols::Mountain => "mountain",
            MiscellaneousSymbols::UmbrellaOnGround => "umbrella on ground",
            MiscellaneousSymbols::Fountain => "fountain",
            MiscellaneousSymbols::FlagInHole => "flag in hole",
            MiscellaneousSymbols::Ferry => "ferry",
            MiscellaneousSymbols::Sailboat => "sailboat",
            MiscellaneousSymbols::SquareFourCorners => "square four corners",
            MiscellaneousSymbols::Skier => "skier",
            MiscellaneousSymbols::IceSkate => "ice skate",
            MiscellaneousSymbols::PersonWithBall => "person with ball",
            MiscellaneousSymbols::Tent => "tent",
            MiscellaneousSymbols::JapaneseBankSymbol => "japanese bank symbol",
            MiscellaneousSymbols::HeadstoneGraveyardSymbol => "headstone graveyard symbol",
            MiscellaneousSymbols::FuelPump => "fuel pump",
            MiscellaneousSymbols::CupOnBlackSquare => "cup on black square",
        }
    }
}
