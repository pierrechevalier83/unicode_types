
/// An enum to represent all characters in the MiscellaneousSymbolsandPictographs block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum MiscellaneousSymbolsandPictographs {
    /// \u{1f300}: '🌀'
    Cyclone,
    /// \u{1f301}: '🌁'
    Foggy,
    /// \u{1f302}: '🌂'
    ClosedUmbrella,
    /// \u{1f303}: '🌃'
    NightWithStars,
    /// \u{1f304}: '🌄'
    SunriseOverMountains,
    /// \u{1f305}: '🌅'
    Sunrise,
    /// \u{1f306}: '🌆'
    CityscapeAtDusk,
    /// \u{1f307}: '🌇'
    SunsetOverBuildings,
    /// \u{1f308}: '🌈'
    Rainbow,
    /// \u{1f309}: '🌉'
    BridgeAtNight,
    /// \u{1f30a}: '🌊'
    WaterWave,
    /// \u{1f30b}: '🌋'
    Volcano,
    /// \u{1f30c}: '🌌'
    MilkyWay,
    /// \u{1f30d}: '🌍'
    EarthGlobeEuropeDashAfrica,
    /// \u{1f30e}: '🌎'
    EarthGlobeAmericas,
    /// \u{1f30f}: '🌏'
    EarthGlobeAsiaDashAustralia,
    /// \u{1f310}: '🌐'
    GlobeWithMeridians,
    /// \u{1f311}: '🌑'
    NewMoonSymbol,
    /// \u{1f312}: '🌒'
    WaxingCrescentMoonSymbol,
    /// \u{1f313}: '🌓'
    FirstQuarterMoonSymbol,
    /// \u{1f314}: '🌔'
    WaxingGibbousMoonSymbol,
    /// \u{1f315}: '🌕'
    FullMoonSymbol,
    /// \u{1f316}: '🌖'
    WaningGibbousMoonSymbol,
    /// \u{1f317}: '🌗'
    LastQuarterMoonSymbol,
    /// \u{1f318}: '🌘'
    WaningCrescentMoonSymbol,
    /// \u{1f319}: '🌙'
    CrescentMoon,
    /// \u{1f31a}: '🌚'
    NewMoonWithFace,
    /// \u{1f31b}: '🌛'
    FirstQuarterMoonWithFace,
    /// \u{1f31c}: '🌜'
    LastQuarterMoonWithFace,
    /// \u{1f31d}: '🌝'
    FullMoonWithFace,
    /// \u{1f31e}: '🌞'
    SunWithFace,
    /// \u{1f31f}: '🌟'
    GlowingStar,
    /// \u{1f320}: '🌠'
    ShootingStar,
    /// \u{1f321}: '🌡'
    Thermometer,
    /// \u{1f322}: '🌢'
    BlackDroplet,
    /// \u{1f323}: '🌣'
    WhiteSun,
    /// \u{1f324}: '🌤'
    WhiteSunWithSmallCloud,
    /// \u{1f325}: '🌥'
    WhiteSunBehindCloud,
    /// \u{1f326}: '🌦'
    WhiteSunBehindCloudWithRain,
    /// \u{1f327}: '🌧'
    CloudWithRain,
    /// \u{1f328}: '🌨'
    CloudWithSnow,
    /// \u{1f329}: '🌩'
    CloudWithLightning,
    /// \u{1f32a}: '🌪'
    CloudWithTornado,
    /// \u{1f32b}: '🌫'
    Fog,
    /// \u{1f32c}: '🌬'
    WindBlowingFace,
    /// \u{1f32d}: '🌭'
    HotDog,
    /// \u{1f32e}: '🌮'
    Taco,
    /// \u{1f32f}: '🌯'
    Burrito,
    /// \u{1f330}: '🌰'
    Chestnut,
    /// \u{1f331}: '🌱'
    Seedling,
    /// \u{1f332}: '🌲'
    EvergreenTree,
    /// \u{1f333}: '🌳'
    DeciduousTree,
    /// \u{1f334}: '🌴'
    PalmTree,
    /// \u{1f335}: '🌵'
    Cactus,
    /// \u{1f336}: '🌶'
    HotPepper,
    /// \u{1f337}: '🌷'
    Tulip,
    /// \u{1f338}: '🌸'
    CherryBlossom,
    /// \u{1f339}: '🌹'
    Rose,
    /// \u{1f33a}: '🌺'
    Hibiscus,
    /// \u{1f33b}: '🌻'
    Sunflower,
    /// \u{1f33c}: '🌼'
    Blossom,
    /// \u{1f33d}: '🌽'
    EarOfMaize,
    /// \u{1f33e}: '🌾'
    EarOfRice,
    /// \u{1f33f}: '🌿'
    Herb,
    /// \u{1f340}: '🍀'
    FourLeafClover,
    /// \u{1f341}: '🍁'
    MapleLeaf,
    /// \u{1f342}: '🍂'
    FallenLeaf,
    /// \u{1f343}: '🍃'
    LeafFlutteringInWind,
    /// \u{1f344}: '🍄'
    Mushroom,
    /// \u{1f345}: '🍅'
    Tomato,
    /// \u{1f346}: '🍆'
    Aubergine,
    /// \u{1f347}: '🍇'
    Grapes,
    /// \u{1f348}: '🍈'
    Melon,
    /// \u{1f349}: '🍉'
    Watermelon,
    /// \u{1f34a}: '🍊'
    Tangerine,
    /// \u{1f34b}: '🍋'
    Lemon,
    /// \u{1f34c}: '🍌'
    Banana,
    /// \u{1f34d}: '🍍'
    Pineapple,
    /// \u{1f34e}: '🍎'
    RedApple,
    /// \u{1f34f}: '🍏'
    GreenApple,
    /// \u{1f350}: '🍐'
    Pear,
    /// \u{1f351}: '🍑'
    Peach,
    /// \u{1f352}: '🍒'
    Cherries,
    /// \u{1f353}: '🍓'
    Strawberry,
    /// \u{1f354}: '🍔'
    Hamburger,
    /// \u{1f355}: '🍕'
    SliceOfPizza,
    /// \u{1f356}: '🍖'
    MeatOnBone,
    /// \u{1f357}: '🍗'
    PoultryLeg,
    /// \u{1f358}: '🍘'
    RiceCracker,
    /// \u{1f359}: '🍙'
    RiceBall,
    /// \u{1f35a}: '🍚'
    CookedRice,
    /// \u{1f35b}: '🍛'
    CurryAndRice,
    /// \u{1f35c}: '🍜'
    SteamingBowl,
    /// \u{1f35d}: '🍝'
    Spaghetti,
    /// \u{1f35e}: '🍞'
    Bread,
    /// \u{1f35f}: '🍟'
    FrenchFries,
    /// \u{1f360}: '🍠'
    RoastedSweetPotato,
    /// \u{1f361}: '🍡'
    Dango,
    /// \u{1f362}: '🍢'
    Oden,
    /// \u{1f363}: '🍣'
    Sushi,
    /// \u{1f364}: '🍤'
    FriedShrimp,
    /// \u{1f365}: '🍥'
    FishCakeWithSwirlDesign,
    /// \u{1f366}: '🍦'
    SoftIceCream,
    /// \u{1f367}: '🍧'
    ShavedIce,
    /// \u{1f368}: '🍨'
    IceCream,
    /// \u{1f369}: '🍩'
    Doughnut,
    /// \u{1f36a}: '🍪'
    Cookie,
    /// \u{1f36b}: '🍫'
    ChocolateBar,
    /// \u{1f36c}: '🍬'
    Candy,
    /// \u{1f36d}: '🍭'
    Lollipop,
    /// \u{1f36e}: '🍮'
    Custard,
    /// \u{1f36f}: '🍯'
    HoneyPot,
    /// \u{1f370}: '🍰'
    Shortcake,
    /// \u{1f371}: '🍱'
    BentoBox,
    /// \u{1f372}: '🍲'
    PotOfFood,
    /// \u{1f373}: '🍳'
    Cooking,
    /// \u{1f374}: '🍴'
    ForkAndKnife,
    /// \u{1f375}: '🍵'
    TeacupWithoutHandle,
    /// \u{1f376}: '🍶'
    SakeBottleAndCup,
    /// \u{1f377}: '🍷'
    WineGlass,
    /// \u{1f378}: '🍸'
    CocktailGlass,
    /// \u{1f379}: '🍹'
    TropicalDrink,
    /// \u{1f37a}: '🍺'
    BeerMug,
    /// \u{1f37b}: '🍻'
    ClinkingBeerMugs,
    /// \u{1f37c}: '🍼'
    BabyBottle,
    /// \u{1f37d}: '🍽'
    ForkAndKnifeWithPlate,
    /// \u{1f37e}: '🍾'
    BottleWithPoppingCork,
    /// \u{1f37f}: '🍿'
    Popcorn,
    /// \u{1f380}: '🎀'
    Ribbon,
    /// \u{1f381}: '🎁'
    WrappedPresent,
    /// \u{1f382}: '🎂'
    BirthdayCake,
    /// \u{1f383}: '🎃'
    JackDashODashLantern,
    /// \u{1f384}: '🎄'
    ChristmasTree,
    /// \u{1f385}: '🎅'
    FatherChristmas,
    /// \u{1f386}: '🎆'
    Fireworks,
    /// \u{1f387}: '🎇'
    FireworkSparkler,
    /// \u{1f388}: '🎈'
    Balloon,
    /// \u{1f389}: '🎉'
    PartyPopper,
    /// \u{1f38a}: '🎊'
    ConfettiBall,
    /// \u{1f38b}: '🎋'
    TanabataTree,
    /// \u{1f38c}: '🎌'
    CrossedFlags,
    /// \u{1f38d}: '🎍'
    PineDecoration,
    /// \u{1f38e}: '🎎'
    JapaneseDolls,
    /// \u{1f38f}: '🎏'
    CarpStreamer,
    /// \u{1f390}: '🎐'
    WindChime,
    /// \u{1f391}: '🎑'
    MoonViewingCeremony,
    /// \u{1f392}: '🎒'
    SchoolSatchel,
    /// \u{1f393}: '🎓'
    GraduationCap,
    /// \u{1f394}: '🎔'
    HeartWithTipOnTheLeft,
    /// \u{1f395}: '🎕'
    BouquetOfFlowers,
    /// \u{1f396}: '🎖'
    MilitaryMedal,
    /// \u{1f397}: '🎗'
    ReminderRibbon,
    /// \u{1f398}: '🎘'
    MusicalKeyboardWithJacks,
    /// \u{1f399}: '🎙'
    StudioMicrophone,
    /// \u{1f39a}: '🎚'
    LevelSlider,
    /// \u{1f39b}: '🎛'
    ControlKnobs,
    /// \u{1f39c}: '🎜'
    BeamedAscendingMusicalNotes,
    /// \u{1f39d}: '🎝'
    BeamedDescendingMusicalNotes,
    /// \u{1f39e}: '🎞'
    FilmFrames,
    /// \u{1f39f}: '🎟'
    AdmissionTickets,
    /// \u{1f3a0}: '🎠'
    CarouselHorse,
    /// \u{1f3a1}: '🎡'
    FerrisWheel,
    /// \u{1f3a2}: '🎢'
    RollerCoaster,
    /// \u{1f3a3}: '🎣'
    FishingPoleAndFish,
    /// \u{1f3a4}: '🎤'
    Microphone,
    /// \u{1f3a5}: '🎥'
    MovieCamera,
    /// \u{1f3a6}: '🎦'
    Cinema,
    /// \u{1f3a7}: '🎧'
    Headphone,
    /// \u{1f3a8}: '🎨'
    ArtistPalette,
    /// \u{1f3a9}: '🎩'
    TopHat,
    /// \u{1f3aa}: '🎪'
    CircusTent,
    /// \u{1f3ab}: '🎫'
    Ticket,
    /// \u{1f3ac}: '🎬'
    ClapperBoard,
    /// \u{1f3ad}: '🎭'
    PerformingArts,
    /// \u{1f3ae}: '🎮'
    VideoGame,
    /// \u{1f3af}: '🎯'
    DirectHit,
    /// \u{1f3b0}: '🎰'
    SlotMachine,
    /// \u{1f3b1}: '🎱'
    Billiards,
    /// \u{1f3b2}: '🎲'
    GameDie,
    /// \u{1f3b3}: '🎳'
    Bowling,
    /// \u{1f3b4}: '🎴'
    FlowerPlayingCards,
    /// \u{1f3b5}: '🎵'
    MusicalNote,
    /// \u{1f3b6}: '🎶'
    MultipleMusicalNotes,
    /// \u{1f3b7}: '🎷'
    Saxophone,
    /// \u{1f3b8}: '🎸'
    Guitar,
    /// \u{1f3b9}: '🎹'
    MusicalKeyboard,
    /// \u{1f3ba}: '🎺'
    Trumpet,
    /// \u{1f3bb}: '🎻'
    Violin,
    /// \u{1f3bc}: '🎼'
    MusicalScore,
    /// \u{1f3bd}: '🎽'
    RunningShirtWithSash,
    /// \u{1f3be}: '🎾'
    TennisRacquetAndBall,
    /// \u{1f3bf}: '🎿'
    SkiAndSkiBoot,
    /// \u{1f3c0}: '🏀'
    BasketballAndHoop,
    /// \u{1f3c1}: '🏁'
    ChequeredFlag,
    /// \u{1f3c2}: '🏂'
    Snowboarder,
    /// \u{1f3c3}: '🏃'
    Runner,
    /// \u{1f3c4}: '🏄'
    Surfer,
    /// \u{1f3c5}: '🏅'
    SportsMedal,
    /// \u{1f3c6}: '🏆'
    Trophy,
    /// \u{1f3c7}: '🏇'
    HorseRacing,
    /// \u{1f3c8}: '🏈'
    AmericanFootball,
    /// \u{1f3c9}: '🏉'
    RugbyFootball,
    /// \u{1f3ca}: '🏊'
    Swimmer,
    /// \u{1f3cb}: '🏋'
    WeightLifter,
    /// \u{1f3cc}: '🏌'
    Golfer,
    /// \u{1f3cd}: '🏍'
    RacingMotorcycle,
    /// \u{1f3ce}: '🏎'
    RacingCar,
    /// \u{1f3cf}: '🏏'
    CricketBatAndBall,
    /// \u{1f3d0}: '🏐'
    Volleyball,
    /// \u{1f3d1}: '🏑'
    FieldHockeyStickAndBall,
    /// \u{1f3d2}: '🏒'
    IceHockeyStickAndPuck,
    /// \u{1f3d3}: '🏓'
    TableTennisPaddleAndBall,
    /// \u{1f3d4}: '🏔'
    SnowCappedMountain,
    /// \u{1f3d5}: '🏕'
    Camping,
    /// \u{1f3d6}: '🏖'
    BeachWithUmbrella,
    /// \u{1f3d7}: '🏗'
    BuildingConstruction,
    /// \u{1f3d8}: '🏘'
    HouseBuildings,
    /// \u{1f3d9}: '🏙'
    Cityscape,
    /// \u{1f3da}: '🏚'
    DerelictHouseBuilding,
    /// \u{1f3db}: '🏛'
    ClassicalBuilding,
    /// \u{1f3dc}: '🏜'
    Desert,
    /// \u{1f3dd}: '🏝'
    DesertIsland,
    /// \u{1f3de}: '🏞'
    NationalPark,
    /// \u{1f3df}: '🏟'
    Stadium,
    /// \u{1f3e0}: '🏠'
    HouseBuilding,
    /// \u{1f3e1}: '🏡'
    HouseWithGarden,
    /// \u{1f3e2}: '🏢'
    OfficeBuilding,
    /// \u{1f3e3}: '🏣'
    JapanesePostOffice,
    /// \u{1f3e4}: '🏤'
    EuropeanPostOffice,
    /// \u{1f3e5}: '🏥'
    Hospital,
    /// \u{1f3e6}: '🏦'
    Bank,
    /// \u{1f3e7}: '🏧'
    AutomatedTellerMachine,
    /// \u{1f3e8}: '🏨'
    Hotel,
    /// \u{1f3e9}: '🏩'
    LoveHotel,
    /// \u{1f3ea}: '🏪'
    ConvenienceStore,
    /// \u{1f3eb}: '🏫'
    School,
    /// \u{1f3ec}: '🏬'
    DepartmentStore,
    /// \u{1f3ed}: '🏭'
    Factory,
    /// \u{1f3ee}: '🏮'
    IzakayaLantern,
    /// \u{1f3ef}: '🏯'
    JapaneseCastle,
    /// \u{1f3f0}: '🏰'
    EuropeanCastle,
    /// \u{1f3f1}: '🏱'
    WhitePennant,
    /// \u{1f3f2}: '🏲'
    BlackPennant,
    /// \u{1f3f3}: '🏳'
    WavingWhiteFlag,
    /// \u{1f3f4}: '🏴'
    WavingBlackFlag,
    /// \u{1f3f5}: '🏵'
    Rosette,
    /// \u{1f3f6}: '🏶'
    BlackRosette,
    /// \u{1f3f7}: '🏷'
    Label,
    /// \u{1f3f8}: '🏸'
    BadmintonRacquetAndShuttlecock,
    /// \u{1f3f9}: '🏹'
    BowAndArrow,
    /// \u{1f3fa}: '🏺'
    Amphora,
    /// \u{1f3fb}: '🏻'
    EmojiModifierFitzpatrickTypeDash1Dash2,
    /// \u{1f3fc}: '🏼'
    EmojiModifierFitzpatrickTypeDash3,
    /// \u{1f3fd}: '🏽'
    EmojiModifierFitzpatrickTypeDash4,
    /// \u{1f3fe}: '🏾'
    EmojiModifierFitzpatrickTypeDash5,
    /// \u{1f3ff}: '🏿'
    EmojiModifierFitzpatrickTypeDash6,
    /// \u{1f400}: '🐀'
    Rat,
    /// \u{1f401}: '🐁'
    Mouse,
    /// \u{1f402}: '🐂'
    Ox,
    /// \u{1f403}: '🐃'
    WaterBuffalo,
    /// \u{1f404}: '🐄'
    Cow,
    /// \u{1f405}: '🐅'
    Tiger,
    /// \u{1f406}: '🐆'
    Leopard,
    /// \u{1f407}: '🐇'
    Rabbit,
    /// \u{1f408}: '🐈'
    Cat,
    /// \u{1f409}: '🐉'
    Dragon,
    /// \u{1f40a}: '🐊'
    Crocodile,
    /// \u{1f40b}: '🐋'
    Whale,
    /// \u{1f40c}: '🐌'
    Snail,
    /// \u{1f40d}: '🐍'
    Snake,
    /// \u{1f40e}: '🐎'
    Horse,
    /// \u{1f40f}: '🐏'
    Ram,
    /// \u{1f410}: '🐐'
    Goat,
    /// \u{1f411}: '🐑'
    Sheep,
    /// \u{1f412}: '🐒'
    Monkey,
    /// \u{1f413}: '🐓'
    Rooster,
    /// \u{1f414}: '🐔'
    Chicken,
    /// \u{1f415}: '🐕'
    Dog,
    /// \u{1f416}: '🐖'
    Pig,
    /// \u{1f417}: '🐗'
    Boar,
    /// \u{1f418}: '🐘'
    Elephant,
    /// \u{1f419}: '🐙'
    Octopus,
    /// \u{1f41a}: '🐚'
    SpiralShell,
    /// \u{1f41b}: '🐛'
    Bug,
    /// \u{1f41c}: '🐜'
    Ant,
    /// \u{1f41d}: '🐝'
    Honeybee,
    /// \u{1f41e}: '🐞'
    LadyBeetle,
    /// \u{1f41f}: '🐟'
    Fish,
    /// \u{1f420}: '🐠'
    TropicalFish,
    /// \u{1f421}: '🐡'
    Blowfish,
    /// \u{1f422}: '🐢'
    Turtle,
    /// \u{1f423}: '🐣'
    HatchingChick,
    /// \u{1f424}: '🐤'
    BabyChick,
    /// \u{1f425}: '🐥'
    FrontDashFacingBabyChick,
    /// \u{1f426}: '🐦'
    Bird,
    /// \u{1f427}: '🐧'
    Penguin,
    /// \u{1f428}: '🐨'
    Koala,
    /// \u{1f429}: '🐩'
    Poodle,
    /// \u{1f42a}: '🐪'
    DromedaryCamel,
    /// \u{1f42b}: '🐫'
    BactrianCamel,
    /// \u{1f42c}: '🐬'
    Dolphin,
    /// \u{1f42d}: '🐭'
    MouseFace,
    /// \u{1f42e}: '🐮'
    CowFace,
    /// \u{1f42f}: '🐯'
    TigerFace,
    /// \u{1f430}: '🐰'
    RabbitFace,
    /// \u{1f431}: '🐱'
    CatFace,
    /// \u{1f432}: '🐲'
    DragonFace,
    /// \u{1f433}: '🐳'
    SpoutingWhale,
    /// \u{1f434}: '🐴'
    HorseFace,
    /// \u{1f435}: '🐵'
    MonkeyFace,
    /// \u{1f436}: '🐶'
    DogFace,
    /// \u{1f437}: '🐷'
    PigFace,
    /// \u{1f438}: '🐸'
    FrogFace,
    /// \u{1f439}: '🐹'
    HamsterFace,
    /// \u{1f43a}: '🐺'
    WolfFace,
    /// \u{1f43b}: '🐻'
    BearFace,
    /// \u{1f43c}: '🐼'
    PandaFace,
    /// \u{1f43d}: '🐽'
    PigNose,
    /// \u{1f43e}: '🐾'
    PawPrints,
    /// \u{1f43f}: '🐿'
    Chipmunk,
    /// \u{1f440}: '👀'
    Eyes,
    /// \u{1f441}: '👁'
    Eye,
    /// \u{1f442}: '👂'
    Ear,
    /// \u{1f443}: '👃'
    Nose,
    /// \u{1f444}: '👄'
    Mouth,
    /// \u{1f445}: '👅'
    Tongue,
    /// \u{1f446}: '👆'
    WhiteUpPointingBackhandIndex,
    /// \u{1f447}: '👇'
    WhiteDownPointingBackhandIndex,
    /// \u{1f448}: '👈'
    WhiteLeftPointingBackhandIndex,
    /// \u{1f449}: '👉'
    WhiteRightPointingBackhandIndex,
    /// \u{1f44a}: '👊'
    FistedHandSign,
    /// \u{1f44b}: '👋'
    WavingHandSign,
    /// \u{1f44c}: '👌'
    OkHandSign,
    /// \u{1f44d}: '👍'
    ThumbsUpSign,
    /// \u{1f44e}: '👎'
    ThumbsDownSign,
    /// \u{1f44f}: '👏'
    ClappingHandsSign,
    /// \u{1f450}: '👐'
    OpenHandsSign,
    /// \u{1f451}: '👑'
    Crown,
    /// \u{1f452}: '👒'
    WomansHat,
    /// \u{1f453}: '👓'
    Eyeglasses,
    /// \u{1f454}: '👔'
    Necktie,
    /// \u{1f455}: '👕'
    TDashShirt,
    /// \u{1f456}: '👖'
    Jeans,
    /// \u{1f457}: '👗'
    Dress,
    /// \u{1f458}: '👘'
    Kimono,
    /// \u{1f459}: '👙'
    Bikini,
    /// \u{1f45a}: '👚'
    WomansClothes,
    /// \u{1f45b}: '👛'
    Purse,
    /// \u{1f45c}: '👜'
    Handbag,
    /// \u{1f45d}: '👝'
    Pouch,
    /// \u{1f45e}: '👞'
    MansShoe,
    /// \u{1f45f}: '👟'
    AthleticShoe,
    /// \u{1f460}: '👠'
    HighDashHeeledShoe,
    /// \u{1f461}: '👡'
    WomansSandal,
    /// \u{1f462}: '👢'
    WomansBoots,
    /// \u{1f463}: '👣'
    Footprints,
    /// \u{1f464}: '👤'
    BustInSilhouette,
    /// \u{1f465}: '👥'
    BustsInSilhouette,
    /// \u{1f466}: '👦'
    Boy,
    /// \u{1f467}: '👧'
    Girl,
    /// \u{1f468}: '👨'
    Man,
    /// \u{1f469}: '👩'
    Woman,
    /// \u{1f46a}: '👪'
    Family,
    /// \u{1f46b}: '👫'
    ManAndWomanHoldingHands,
    /// \u{1f46c}: '👬'
    TwoMenHoldingHands,
    /// \u{1f46d}: '👭'
    TwoWomenHoldingHands,
    /// \u{1f46e}: '👮'
    PoliceOfficer,
    /// \u{1f46f}: '👯'
    WomanWithBunnyEars,
    /// \u{1f470}: '👰'
    BrideWithVeil,
    /// \u{1f471}: '👱'
    PersonWithBlondHair,
    /// \u{1f472}: '👲'
    ManWithGuaPiMao,
    /// \u{1f473}: '👳'
    ManWithTurban,
    /// \u{1f474}: '👴'
    OlderMan,
    /// \u{1f475}: '👵'
    OlderWoman,
    /// \u{1f476}: '👶'
    Baby,
    /// \u{1f477}: '👷'
    ConstructionWorker,
    /// \u{1f478}: '👸'
    Princess,
    /// \u{1f479}: '👹'
    JapaneseOgre,
    /// \u{1f47a}: '👺'
    JapaneseGoblin,
    /// \u{1f47b}: '👻'
    Ghost,
    /// \u{1f47c}: '👼'
    BabyAngel,
    /// \u{1f47d}: '👽'
    ExtraterrestrialAlien,
    /// \u{1f47e}: '👾'
    AlienMonster,
    /// \u{1f47f}: '👿'
    Imp,
    /// \u{1f480}: '💀'
    Skull,
    /// \u{1f481}: '💁'
    InformationDeskPerson,
    /// \u{1f482}: '💂'
    Guardsman,
    /// \u{1f483}: '💃'
    Dancer,
    /// \u{1f484}: '💄'
    Lipstick,
    /// \u{1f485}: '💅'
    NailPolish,
    /// \u{1f486}: '💆'
    FaceMassage,
    /// \u{1f487}: '💇'
    Haircut,
    /// \u{1f488}: '💈'
    BarberPole,
    /// \u{1f489}: '💉'
    Syringe,
    /// \u{1f48a}: '💊'
    Pill,
    /// \u{1f48b}: '💋'
    KissMark,
    /// \u{1f48c}: '💌'
    LoveLetter,
    /// \u{1f48d}: '💍'
    Ring,
    /// \u{1f48e}: '💎'
    GemStone,
    /// \u{1f48f}: '💏'
    Kiss,
    /// \u{1f490}: '💐'
    Bouquet,
    /// \u{1f491}: '💑'
    CoupleWithHeart,
    /// \u{1f492}: '💒'
    Wedding,
    /// \u{1f493}: '💓'
    BeatingHeart,
    /// \u{1f494}: '💔'
    BrokenHeart,
    /// \u{1f495}: '💕'
    TwoHearts,
    /// \u{1f496}: '💖'
    SparklingHeart,
    /// \u{1f497}: '💗'
    GrowingHeart,
    /// \u{1f498}: '💘'
    HeartWithArrow,
    /// \u{1f499}: '💙'
    BlueHeart,
    /// \u{1f49a}: '💚'
    GreenHeart,
    /// \u{1f49b}: '💛'
    YellowHeart,
    /// \u{1f49c}: '💜'
    PurpleHeart,
    /// \u{1f49d}: '💝'
    HeartWithRibbon,
    /// \u{1f49e}: '💞'
    RevolvingHearts,
    /// \u{1f49f}: '💟'
    HeartDecoration,
    /// \u{1f4a0}: '💠'
    DiamondShapeWithADotInside,
    /// \u{1f4a1}: '💡'
    ElectricLightBulb,
    /// \u{1f4a2}: '💢'
    AngerSymbol,
    /// \u{1f4a3}: '💣'
    Bomb,
    /// \u{1f4a4}: '💤'
    SleepingSymbol,
    /// \u{1f4a5}: '💥'
    CollisionSymbol,
    /// \u{1f4a6}: '💦'
    SplashingSweatSymbol,
    /// \u{1f4a7}: '💧'
    Droplet,
    /// \u{1f4a8}: '💨'
    DashSymbol,
    /// \u{1f4a9}: '💩'
    PileOfPoo,
    /// \u{1f4aa}: '💪'
    FlexedBiceps,
    /// \u{1f4ab}: '💫'
    DizzySymbol,
    /// \u{1f4ac}: '💬'
    SpeechBalloon,
    /// \u{1f4ad}: '💭'
    ThoughtBalloon,
    /// \u{1f4ae}: '💮'
    WhiteFlower,
    /// \u{1f4af}: '💯'
    HundredPointsSymbol,
    /// \u{1f4b0}: '💰'
    MoneyBag,
    /// \u{1f4b1}: '💱'
    CurrencyExchange,
    /// \u{1f4b2}: '💲'
    HeavyDollarSign,
    /// \u{1f4b3}: '💳'
    CreditCard,
    /// \u{1f4b4}: '💴'
    BanknoteWithYenSign,
    /// \u{1f4b5}: '💵'
    BanknoteWithDollarSign,
    /// \u{1f4b6}: '💶'
    BanknoteWithEuroSign,
    /// \u{1f4b7}: '💷'
    BanknoteWithPoundSign,
    /// \u{1f4b8}: '💸'
    MoneyWithWings,
    /// \u{1f4b9}: '💹'
    ChartWithUpwardsTrendAndYenSign,
    /// \u{1f4ba}: '💺'
    Seat,
    /// \u{1f4bb}: '💻'
    PersonalComputer,
    /// \u{1f4bc}: '💼'
    Briefcase,
    /// \u{1f4bd}: '💽'
    Minidisc,
    /// \u{1f4be}: '💾'
    FloppyDisk,
    /// \u{1f4bf}: '💿'
    OpticalDisc,
    /// \u{1f4c0}: '📀'
    Dvd,
    /// \u{1f4c1}: '📁'
    FileFolder,
    /// \u{1f4c2}: '📂'
    OpenFileFolder,
    /// \u{1f4c3}: '📃'
    PageWithCurl,
    /// \u{1f4c4}: '📄'
    PageFacingUp,
    /// \u{1f4c5}: '📅'
    Calendar,
    /// \u{1f4c6}: '📆'
    TearDashOffCalendar,
    /// \u{1f4c7}: '📇'
    CardIndex,
    /// \u{1f4c8}: '📈'
    ChartWithUpwardsTrend,
    /// \u{1f4c9}: '📉'
    ChartWithDownwardsTrend,
    /// \u{1f4ca}: '📊'
    BarChart,
    /// \u{1f4cb}: '📋'
    Clipboard,
    /// \u{1f4cc}: '📌'
    Pushpin,
    /// \u{1f4cd}: '📍'
    RoundPushpin,
    /// \u{1f4ce}: '📎'
    Paperclip,
    /// \u{1f4cf}: '📏'
    StraightRuler,
    /// \u{1f4d0}: '📐'
    TriangularRuler,
    /// \u{1f4d1}: '📑'
    BookmarkTabs,
    /// \u{1f4d2}: '📒'
    Ledger,
    /// \u{1f4d3}: '📓'
    Notebook,
    /// \u{1f4d4}: '📔'
    NotebookWithDecorativeCover,
    /// \u{1f4d5}: '📕'
    ClosedBook,
    /// \u{1f4d6}: '📖'
    OpenBook,
    /// \u{1f4d7}: '📗'
    GreenBook,
    /// \u{1f4d8}: '📘'
    BlueBook,
    /// \u{1f4d9}: '📙'
    OrangeBook,
    /// \u{1f4da}: '📚'
    Books,
    /// \u{1f4db}: '📛'
    NameBadge,
    /// \u{1f4dc}: '📜'
    Scroll,
    /// \u{1f4dd}: '📝'
    Memo,
    /// \u{1f4de}: '📞'
    TelephoneReceiver,
    /// \u{1f4df}: '📟'
    Pager,
    /// \u{1f4e0}: '📠'
    FaxMachine,
    /// \u{1f4e1}: '📡'
    SatelliteAntenna,
    /// \u{1f4e2}: '📢'
    PublicAddressLoudspeaker,
    /// \u{1f4e3}: '📣'
    CheeringMegaphone,
    /// \u{1f4e4}: '📤'
    OutboxTray,
    /// \u{1f4e5}: '📥'
    InboxTray,
    /// \u{1f4e6}: '📦'
    Package,
    /// \u{1f4e7}: '📧'
    EDashMailSymbol,
    /// \u{1f4e8}: '📨'
    IncomingEnvelope,
    /// \u{1f4e9}: '📩'
    EnvelopeWithDownwardsArrowAbove,
    /// \u{1f4ea}: '📪'
    ClosedMailboxWithLoweredFlag,
    /// \u{1f4eb}: '📫'
    ClosedMailboxWithRaisedFlag,
    /// \u{1f4ec}: '📬'
    OpenMailboxWithRaisedFlag,
    /// \u{1f4ed}: '📭'
    OpenMailboxWithLoweredFlag,
    /// \u{1f4ee}: '📮'
    Postbox,
    /// \u{1f4ef}: '📯'
    PostalHorn,
    /// \u{1f4f0}: '📰'
    Newspaper,
    /// \u{1f4f1}: '📱'
    MobilePhone,
    /// \u{1f4f2}: '📲'
    MobilePhoneWithRightwardsArrowAtLeft,
    /// \u{1f4f3}: '📳'
    VibrationMode,
    /// \u{1f4f4}: '📴'
    MobilePhoneOff,
    /// \u{1f4f5}: '📵'
    NoMobilePhones,
    /// \u{1f4f6}: '📶'
    AntennaWithBars,
    /// \u{1f4f7}: '📷'
    Camera,
    /// \u{1f4f8}: '📸'
    CameraWithFlash,
    /// \u{1f4f9}: '📹'
    VideoCamera,
    /// \u{1f4fa}: '📺'
    Television,
    /// \u{1f4fb}: '📻'
    Radio,
    /// \u{1f4fc}: '📼'
    Videocassette,
    /// \u{1f4fd}: '📽'
    FilmProjector,
    /// \u{1f4fe}: '📾'
    PortableStereo,
    /// \u{1f4ff}: '📿'
    PrayerBeads,
    /// \u{1f500}: '🔀'
    TwistedRightwardsArrows,
    /// \u{1f501}: '🔁'
    ClockwiseRightwardsAndLeftwardsOpenCircleArrows,
    /// \u{1f502}: '🔂'
    ClockwiseRightwardsAndLeftwardsOpenCircleArrowsWithCircledOneOverlay,
    /// \u{1f503}: '🔃'
    ClockwiseDownwardsAndUpwardsOpenCircleArrows,
    /// \u{1f504}: '🔄'
    AnticlockwiseDownwardsAndUpwardsOpenCircleArrows,
    /// \u{1f505}: '🔅'
    LowBrightnessSymbol,
    /// \u{1f506}: '🔆'
    HighBrightnessSymbol,
    /// \u{1f507}: '🔇'
    SpeakerWithCancellationStroke,
    /// \u{1f508}: '🔈'
    Speaker,
    /// \u{1f509}: '🔉'
    SpeakerWithOneSoundWave,
    /// \u{1f50a}: '🔊'
    SpeakerWithThreeSoundWaves,
    /// \u{1f50b}: '🔋'
    Battery,
    /// \u{1f50c}: '🔌'
    ElectricPlug,
    /// \u{1f50d}: '🔍'
    LeftDashPointingMagnifyingGlass,
    /// \u{1f50e}: '🔎'
    RightDashPointingMagnifyingGlass,
    /// \u{1f50f}: '🔏'
    LockWithInkPen,
    /// \u{1f510}: '🔐'
    ClosedLockWithKey,
    /// \u{1f511}: '🔑'
    Key,
    /// \u{1f512}: '🔒'
    Lock,
    /// \u{1f513}: '🔓'
    OpenLock,
    /// \u{1f514}: '🔔'
    Bell,
    /// \u{1f515}: '🔕'
    BellWithCancellationStroke,
    /// \u{1f516}: '🔖'
    Bookmark,
    /// \u{1f517}: '🔗'
    LinkSymbol,
    /// \u{1f518}: '🔘'
    RadioButton,
    /// \u{1f519}: '🔙'
    BackWithLeftwardsArrowAbove,
    /// \u{1f51a}: '🔚'
    EndWithLeftwardsArrowAbove,
    /// \u{1f51b}: '🔛'
    OnWithExclamationMarkWithLeftRightArrowAbove,
    /// \u{1f51c}: '🔜'
    SoonWithRightwardsArrowAbove,
    /// \u{1f51d}: '🔝'
    TopWithUpwardsArrowAbove,
    /// \u{1f51e}: '🔞'
    NoOneUnderEighteenSymbol,
    /// \u{1f51f}: '🔟'
    KeycapTen,
    /// \u{1f520}: '🔠'
    InputSymbolForLatinCapitalLetters,
    /// \u{1f521}: '🔡'
    InputSymbolForLatinSmallLetters,
    /// \u{1f522}: '🔢'
    InputSymbolForNumbers,
    /// \u{1f523}: '🔣'
    InputSymbolForSymbols,
    /// \u{1f524}: '🔤'
    InputSymbolForLatinLetters,
    /// \u{1f525}: '🔥'
    Fire,
    /// \u{1f526}: '🔦'
    ElectricTorch,
    /// \u{1f527}: '🔧'
    Wrench,
    /// \u{1f528}: '🔨'
    Hammer,
    /// \u{1f529}: '🔩'
    NutAndBolt,
    /// \u{1f52a}: '🔪'
    Hocho,
    /// \u{1f52b}: '🔫'
    Pistol,
    /// \u{1f52c}: '🔬'
    Microscope,
    /// \u{1f52d}: '🔭'
    Telescope,
    /// \u{1f52e}: '🔮'
    CrystalBall,
    /// \u{1f52f}: '🔯'
    SixPointedStarWithMiddleDot,
    /// \u{1f530}: '🔰'
    JapaneseSymbolForBeginner,
    /// \u{1f531}: '🔱'
    TridentEmblem,
    /// \u{1f532}: '🔲'
    BlackSquareButton,
    /// \u{1f533}: '🔳'
    WhiteSquareButton,
    /// \u{1f534}: '🔴'
    LargeRedCircle,
    /// \u{1f535}: '🔵'
    LargeBlueCircle,
    /// \u{1f536}: '🔶'
    LargeOrangeDiamond,
    /// \u{1f537}: '🔷'
    LargeBlueDiamond,
    /// \u{1f538}: '🔸'
    SmallOrangeDiamond,
    /// \u{1f539}: '🔹'
    SmallBlueDiamond,
    /// \u{1f53a}: '🔺'
    UpDashPointingRedTriangle,
    /// \u{1f53b}: '🔻'
    DownDashPointingRedTriangle,
    /// \u{1f53c}: '🔼'
    UpDashPointingSmallRedTriangle,
    /// \u{1f53d}: '🔽'
    DownDashPointingSmallRedTriangle,
    /// \u{1f53e}: '🔾'
    LowerRightShadowedWhiteCircle,
    /// \u{1f53f}: '🔿'
    UpperRightShadowedWhiteCircle,
    /// \u{1f540}: '🕀'
    CircledCrossPommee,
    /// \u{1f541}: '🕁'
    CrossPommeeWithHalfDashCircleBelow,
    /// \u{1f542}: '🕂'
    CrossPommee,
    /// \u{1f543}: '🕃'
    NotchedLeftSemicircleWithThreeDots,
    /// \u{1f544}: '🕄'
    NotchedRightSemicircleWithThreeDots,
    /// \u{1f545}: '🕅'
    SymbolForMarksChapter,
    /// \u{1f546}: '🕆'
    WhiteLatinCross,
    /// \u{1f547}: '🕇'
    HeavyLatinCross,
    /// \u{1f548}: '🕈'
    CelticCross,
    /// \u{1f549}: '🕉'
    OmSymbol,
    /// \u{1f54a}: '🕊'
    DoveOfPeace,
    /// \u{1f54b}: '🕋'
    Kaaba,
    /// \u{1f54c}: '🕌'
    Mosque,
    /// \u{1f54d}: '🕍'
    Synagogue,
    /// \u{1f54e}: '🕎'
    MenorahWithNineBranches,
    /// \u{1f54f}: '🕏'
    BowlOfHygieia,
    /// \u{1f550}: '🕐'
    ClockFaceOneOclock,
    /// \u{1f551}: '🕑'
    ClockFaceTwoOclock,
    /// \u{1f552}: '🕒'
    ClockFaceThreeOclock,
    /// \u{1f553}: '🕓'
    ClockFaceFourOclock,
    /// \u{1f554}: '🕔'
    ClockFaceFiveOclock,
    /// \u{1f555}: '🕕'
    ClockFaceSixOclock,
    /// \u{1f556}: '🕖'
    ClockFaceSevenOclock,
    /// \u{1f557}: '🕗'
    ClockFaceEightOclock,
    /// \u{1f558}: '🕘'
    ClockFaceNineOclock,
    /// \u{1f559}: '🕙'
    ClockFaceTenOclock,
    /// \u{1f55a}: '🕚'
    ClockFaceElevenOclock,
    /// \u{1f55b}: '🕛'
    ClockFaceTwelveOclock,
    /// \u{1f55c}: '🕜'
    ClockFaceOneDashThirty,
    /// \u{1f55d}: '🕝'
    ClockFaceTwoDashThirty,
    /// \u{1f55e}: '🕞'
    ClockFaceThreeDashThirty,
    /// \u{1f55f}: '🕟'
    ClockFaceFourDashThirty,
    /// \u{1f560}: '🕠'
    ClockFaceFiveDashThirty,
    /// \u{1f561}: '🕡'
    ClockFaceSixDashThirty,
    /// \u{1f562}: '🕢'
    ClockFaceSevenDashThirty,
    /// \u{1f563}: '🕣'
    ClockFaceEightDashThirty,
    /// \u{1f564}: '🕤'
    ClockFaceNineDashThirty,
    /// \u{1f565}: '🕥'
    ClockFaceTenDashThirty,
    /// \u{1f566}: '🕦'
    ClockFaceElevenDashThirty,
    /// \u{1f567}: '🕧'
    ClockFaceTwelveDashThirty,
    /// \u{1f568}: '🕨'
    RightSpeaker,
    /// \u{1f569}: '🕩'
    RightSpeakerWithOneSoundWave,
    /// \u{1f56a}: '🕪'
    RightSpeakerWithThreeSoundWaves,
    /// \u{1f56b}: '🕫'
    Bullhorn,
    /// \u{1f56c}: '🕬'
    BullhornWithSoundWaves,
    /// \u{1f56d}: '🕭'
    RingingBell,
    /// \u{1f56e}: '🕮'
    Book,
    /// \u{1f56f}: '🕯'
    Candle,
    /// \u{1f570}: '🕰'
    MantelpieceClock,
    /// \u{1f571}: '🕱'
    BlackSkullAndCrossbones,
    /// \u{1f572}: '🕲'
    NoPiracy,
    /// \u{1f573}: '🕳'
    Hole,
    /// \u{1f574}: '🕴'
    ManInBusinessSuitLevitating,
    /// \u{1f575}: '🕵'
    SleuthOrSpy,
    /// \u{1f576}: '🕶'
    DarkSunglasses,
    /// \u{1f577}: '🕷'
    Spider,
    /// \u{1f578}: '🕸'
    SpiderWeb,
    /// \u{1f579}: '🕹'
    Joystick,
    /// \u{1f57a}: '🕺'
    ManDancing,
    /// \u{1f57b}: '🕻'
    LeftHandTelephoneReceiver,
    /// \u{1f57c}: '🕼'
    TelephoneReceiverWithPage,
    /// \u{1f57d}: '🕽'
    RightHandTelephoneReceiver,
    /// \u{1f57e}: '🕾'
    WhiteTouchtoneTelephone,
    /// \u{1f57f}: '🕿'
    BlackTouchtoneTelephone,
    /// \u{1f580}: '🖀'
    TelephoneOnTopOfModem,
    /// \u{1f581}: '🖁'
    ClamshellMobilePhone,
    /// \u{1f582}: '🖂'
    BackOfEnvelope,
    /// \u{1f583}: '🖃'
    StampedEnvelope,
    /// \u{1f584}: '🖄'
    EnvelopeWithLightning,
    /// \u{1f585}: '🖅'
    FlyingEnvelope,
    /// \u{1f586}: '🖆'
    PenOverStampedEnvelope,
    /// \u{1f587}: '🖇'
    LinkedPaperclips,
    /// \u{1f588}: '🖈'
    BlackPushpin,
    /// \u{1f589}: '🖉'
    LowerLeftPencil,
    /// \u{1f58a}: '🖊'
    LowerLeftBallpointPen,
    /// \u{1f58b}: '🖋'
    LowerLeftFountainPen,
    /// \u{1f58c}: '🖌'
    LowerLeftPaintbrush,
    /// \u{1f58d}: '🖍'
    LowerLeftCrayon,
    /// \u{1f58e}: '🖎'
    LeftWritingHand,
    /// \u{1f58f}: '🖏'
    TurnedOkHandSign,
    /// \u{1f590}: '🖐'
    RaisedHandWithFingersSplayed,
    /// \u{1f591}: '🖑'
    ReversedRaisedHandWithFingersSplayed,
    /// \u{1f592}: '🖒'
    ReversedThumbsUpSign,
    /// \u{1f593}: '🖓'
    ReversedThumbsDownSign,
    /// \u{1f594}: '🖔'
    ReversedVictoryHand,
    /// \u{1f595}: '🖕'
    ReversedHandWithMiddleFingerExtended,
    /// \u{1f596}: '🖖'
    RaisedHandWithPartBetweenMiddleAndRingFingers,
    /// \u{1f597}: '🖗'
    WhiteDownPointingLeftHandIndex,
    /// \u{1f598}: '🖘'
    SidewaysWhiteLeftPointingIndex,
    /// \u{1f599}: '🖙'
    SidewaysWhiteRightPointingIndex,
    /// \u{1f59a}: '🖚'
    SidewaysBlackLeftPointingIndex,
    /// \u{1f59b}: '🖛'
    SidewaysBlackRightPointingIndex,
    /// \u{1f59c}: '🖜'
    BlackLeftPointingBackhandIndex,
    /// \u{1f59d}: '🖝'
    BlackRightPointingBackhandIndex,
    /// \u{1f59e}: '🖞'
    SidewaysWhiteUpPointingIndex,
    /// \u{1f59f}: '🖟'
    SidewaysWhiteDownPointingIndex,
    /// \u{1f5a0}: '🖠'
    SidewaysBlackUpPointingIndex,
    /// \u{1f5a1}: '🖡'
    SidewaysBlackDownPointingIndex,
    /// \u{1f5a2}: '🖢'
    BlackUpPointingBackhandIndex,
    /// \u{1f5a3}: '🖣'
    BlackDownPointingBackhandIndex,
    /// \u{1f5a4}: '🖤'
    BlackHeart,
    /// \u{1f5a5}: '🖥'
    DesktopComputer,
    /// \u{1f5a6}: '🖦'
    KeyboardAndMouse,
    /// \u{1f5a7}: '🖧'
    ThreeNetworkedComputers,
    /// \u{1f5a8}: '🖨'
    Printer,
    /// \u{1f5a9}: '🖩'
    PocketCalculator,
    /// \u{1f5aa}: '🖪'
    BlackHardShellFloppyDisk,
    /// \u{1f5ab}: '🖫'
    WhiteHardShellFloppyDisk,
    /// \u{1f5ac}: '🖬'
    SoftShellFloppyDisk,
    /// \u{1f5ad}: '🖭'
    TapeCartridge,
    /// \u{1f5ae}: '🖮'
    WiredKeyboard,
    /// \u{1f5af}: '🖯'
    OneButtonMouse,
    /// \u{1f5b0}: '🖰'
    TwoButtonMouse,
    /// \u{1f5b1}: '🖱'
    ThreeButtonMouse,
    /// \u{1f5b2}: '🖲'
    Trackball,
    /// \u{1f5b3}: '🖳'
    OldPersonalComputer,
    /// \u{1f5b4}: '🖴'
    HardDisk,
    /// \u{1f5b5}: '🖵'
    Screen,
    /// \u{1f5b6}: '🖶'
    PrinterIcon,
    /// \u{1f5b7}: '🖷'
    FaxIcon,
    /// \u{1f5b8}: '🖸'
    OpticalDiscIcon,
    /// \u{1f5b9}: '🖹'
    DocumentWithText,
    /// \u{1f5ba}: '🖺'
    DocumentWithTextAndPicture,
    /// \u{1f5bb}: '🖻'
    DocumentWithPicture,
    /// \u{1f5bc}: '🖼'
    FrameWithPicture,
    /// \u{1f5bd}: '🖽'
    FrameWithTiles,
    /// \u{1f5be}: '🖾'
    FrameWithAnX,
    /// \u{1f5bf}: '🖿'
    BlackFolder,
    /// \u{1f5c0}: '🗀'
    Folder,
    /// \u{1f5c1}: '🗁'
    OpenFolder,
    /// \u{1f5c2}: '🗂'
    CardIndexDividers,
    /// \u{1f5c3}: '🗃'
    CardFileBox,
    /// \u{1f5c4}: '🗄'
    FileCabinet,
    /// \u{1f5c5}: '🗅'
    EmptyNote,
    /// \u{1f5c6}: '🗆'
    EmptyNotePage,
    /// \u{1f5c7}: '🗇'
    EmptyNotePad,
    /// \u{1f5c8}: '🗈'
    Note,
    /// \u{1f5c9}: '🗉'
    NotePage,
    /// \u{1f5ca}: '🗊'
    NotePad,
    /// \u{1f5cb}: '🗋'
    EmptyDocument,
    /// \u{1f5cc}: '🗌'
    EmptyPage,
    /// \u{1f5cd}: '🗍'
    EmptyPages,
    /// \u{1f5ce}: '🗎'
    Document,
    /// \u{1f5cf}: '🗏'
    Page,
    /// \u{1f5d0}: '🗐'
    Pages,
    /// \u{1f5d1}: '🗑'
    Wastebasket,
    /// \u{1f5d2}: '🗒'
    SpiralNotePad,
    /// \u{1f5d3}: '🗓'
    SpiralCalendarPad,
    /// \u{1f5d4}: '🗔'
    DesktopWindow,
    /// \u{1f5d5}: '🗕'
    Minimize,
    /// \u{1f5d6}: '🗖'
    Maximize,
    /// \u{1f5d7}: '🗗'
    Overlap,
    /// \u{1f5d8}: '🗘'
    ClockwiseRightAndLeftSemicircleArrows,
    /// \u{1f5d9}: '🗙'
    CancellationX,
    /// \u{1f5da}: '🗚'
    IncreaseFontSizeSymbol,
    /// \u{1f5db}: '🗛'
    DecreaseFontSizeSymbol,
    /// \u{1f5dc}: '🗜'
    Compression,
    /// \u{1f5dd}: '🗝'
    OldKey,
    /// \u{1f5de}: '🗞'
    RolledDashUpNewspaper,
    /// \u{1f5df}: '🗟'
    PageWithCircledText,
    /// \u{1f5e0}: '🗠'
    StockChart,
    /// \u{1f5e1}: '🗡'
    DaggerKnife,
    /// \u{1f5e2}: '🗢'
    Lips,
    /// \u{1f5e3}: '🗣'
    SpeakingHeadInSilhouette,
    /// \u{1f5e4}: '🗤'
    ThreeRaysAbove,
    /// \u{1f5e5}: '🗥'
    ThreeRaysBelow,
    /// \u{1f5e6}: '🗦'
    ThreeRaysLeft,
    /// \u{1f5e7}: '🗧'
    ThreeRaysRight,
    /// \u{1f5e8}: '🗨'
    LeftSpeechBubble,
    /// \u{1f5e9}: '🗩'
    RightSpeechBubble,
    /// \u{1f5ea}: '🗪'
    TwoSpeechBubbles,
    /// \u{1f5eb}: '🗫'
    ThreeSpeechBubbles,
    /// \u{1f5ec}: '🗬'
    LeftThoughtBubble,
    /// \u{1f5ed}: '🗭'
    RightThoughtBubble,
    /// \u{1f5ee}: '🗮'
    LeftAngerBubble,
    /// \u{1f5ef}: '🗯'
    RightAngerBubble,
    /// \u{1f5f0}: '🗰'
    MoodBubble,
    /// \u{1f5f1}: '🗱'
    LightningMoodBubble,
    /// \u{1f5f2}: '🗲'
    LightningMood,
    /// \u{1f5f3}: '🗳'
    BallotBoxWithBallot,
    /// \u{1f5f4}: '🗴'
    BallotScriptX,
    /// \u{1f5f5}: '🗵'
    BallotBoxWithScriptX,
    /// \u{1f5f6}: '🗶'
    BallotBoldScriptX,
    /// \u{1f5f7}: '🗷'
    BallotBoxWithBoldScriptX,
    /// \u{1f5f8}: '🗸'
    LightCheckMark,
    /// \u{1f5f9}: '🗹'
    BallotBoxWithBoldCheck,
    /// \u{1f5fa}: '🗺'
    WorldMap,
    /// \u{1f5fb}: '🗻'
    MountFuji,
    /// \u{1f5fc}: '🗼'
    TokyoTower,
    /// \u{1f5fd}: '🗽'
    StatueOfLiberty,
    /// \u{1f5fe}: '🗾'
    SilhouetteOfJapan,
}

impl Into<char> for MiscellaneousSymbolsandPictographs {
    fn into(self) -> char {
        match self {
            MiscellaneousSymbolsandPictographs::Cyclone => '🌀',
            MiscellaneousSymbolsandPictographs::Foggy => '🌁',
            MiscellaneousSymbolsandPictographs::ClosedUmbrella => '🌂',
            MiscellaneousSymbolsandPictographs::NightWithStars => '🌃',
            MiscellaneousSymbolsandPictographs::SunriseOverMountains => '🌄',
            MiscellaneousSymbolsandPictographs::Sunrise => '🌅',
            MiscellaneousSymbolsandPictographs::CityscapeAtDusk => '🌆',
            MiscellaneousSymbolsandPictographs::SunsetOverBuildings => '🌇',
            MiscellaneousSymbolsandPictographs::Rainbow => '🌈',
            MiscellaneousSymbolsandPictographs::BridgeAtNight => '🌉',
            MiscellaneousSymbolsandPictographs::WaterWave => '🌊',
            MiscellaneousSymbolsandPictographs::Volcano => '🌋',
            MiscellaneousSymbolsandPictographs::MilkyWay => '🌌',
            MiscellaneousSymbolsandPictographs::EarthGlobeEuropeDashAfrica => '🌍',
            MiscellaneousSymbolsandPictographs::EarthGlobeAmericas => '🌎',
            MiscellaneousSymbolsandPictographs::EarthGlobeAsiaDashAustralia => '🌏',
            MiscellaneousSymbolsandPictographs::GlobeWithMeridians => '🌐',
            MiscellaneousSymbolsandPictographs::NewMoonSymbol => '🌑',
            MiscellaneousSymbolsandPictographs::WaxingCrescentMoonSymbol => '🌒',
            MiscellaneousSymbolsandPictographs::FirstQuarterMoonSymbol => '🌓',
            MiscellaneousSymbolsandPictographs::WaxingGibbousMoonSymbol => '🌔',
            MiscellaneousSymbolsandPictographs::FullMoonSymbol => '🌕',
            MiscellaneousSymbolsandPictographs::WaningGibbousMoonSymbol => '🌖',
            MiscellaneousSymbolsandPictographs::LastQuarterMoonSymbol => '🌗',
            MiscellaneousSymbolsandPictographs::WaningCrescentMoonSymbol => '🌘',
            MiscellaneousSymbolsandPictographs::CrescentMoon => '🌙',
            MiscellaneousSymbolsandPictographs::NewMoonWithFace => '🌚',
            MiscellaneousSymbolsandPictographs::FirstQuarterMoonWithFace => '🌛',
            MiscellaneousSymbolsandPictographs::LastQuarterMoonWithFace => '🌜',
            MiscellaneousSymbolsandPictographs::FullMoonWithFace => '🌝',
            MiscellaneousSymbolsandPictographs::SunWithFace => '🌞',
            MiscellaneousSymbolsandPictographs::GlowingStar => '🌟',
            MiscellaneousSymbolsandPictographs::ShootingStar => '🌠',
            MiscellaneousSymbolsandPictographs::Thermometer => '🌡',
            MiscellaneousSymbolsandPictographs::BlackDroplet => '🌢',
            MiscellaneousSymbolsandPictographs::WhiteSun => '🌣',
            MiscellaneousSymbolsandPictographs::WhiteSunWithSmallCloud => '🌤',
            MiscellaneousSymbolsandPictographs::WhiteSunBehindCloud => '🌥',
            MiscellaneousSymbolsandPictographs::WhiteSunBehindCloudWithRain => '🌦',
            MiscellaneousSymbolsandPictographs::CloudWithRain => '🌧',
            MiscellaneousSymbolsandPictographs::CloudWithSnow => '🌨',
            MiscellaneousSymbolsandPictographs::CloudWithLightning => '🌩',
            MiscellaneousSymbolsandPictographs::CloudWithTornado => '🌪',
            MiscellaneousSymbolsandPictographs::Fog => '🌫',
            MiscellaneousSymbolsandPictographs::WindBlowingFace => '🌬',
            MiscellaneousSymbolsandPictographs::HotDog => '🌭',
            MiscellaneousSymbolsandPictographs::Taco => '🌮',
            MiscellaneousSymbolsandPictographs::Burrito => '🌯',
            MiscellaneousSymbolsandPictographs::Chestnut => '🌰',
            MiscellaneousSymbolsandPictographs::Seedling => '🌱',
            MiscellaneousSymbolsandPictographs::EvergreenTree => '🌲',
            MiscellaneousSymbolsandPictographs::DeciduousTree => '🌳',
            MiscellaneousSymbolsandPictographs::PalmTree => '🌴',
            MiscellaneousSymbolsandPictographs::Cactus => '🌵',
            MiscellaneousSymbolsandPictographs::HotPepper => '🌶',
            MiscellaneousSymbolsandPictographs::Tulip => '🌷',
            MiscellaneousSymbolsandPictographs::CherryBlossom => '🌸',
            MiscellaneousSymbolsandPictographs::Rose => '🌹',
            MiscellaneousSymbolsandPictographs::Hibiscus => '🌺',
            MiscellaneousSymbolsandPictographs::Sunflower => '🌻',
            MiscellaneousSymbolsandPictographs::Blossom => '🌼',
            MiscellaneousSymbolsandPictographs::EarOfMaize => '🌽',
            MiscellaneousSymbolsandPictographs::EarOfRice => '🌾',
            MiscellaneousSymbolsandPictographs::Herb => '🌿',
            MiscellaneousSymbolsandPictographs::FourLeafClover => '🍀',
            MiscellaneousSymbolsandPictographs::MapleLeaf => '🍁',
            MiscellaneousSymbolsandPictographs::FallenLeaf => '🍂',
            MiscellaneousSymbolsandPictographs::LeafFlutteringInWind => '🍃',
            MiscellaneousSymbolsandPictographs::Mushroom => '🍄',
            MiscellaneousSymbolsandPictographs::Tomato => '🍅',
            MiscellaneousSymbolsandPictographs::Aubergine => '🍆',
            MiscellaneousSymbolsandPictographs::Grapes => '🍇',
            MiscellaneousSymbolsandPictographs::Melon => '🍈',
            MiscellaneousSymbolsandPictographs::Watermelon => '🍉',
            MiscellaneousSymbolsandPictographs::Tangerine => '🍊',
            MiscellaneousSymbolsandPictographs::Lemon => '🍋',
            MiscellaneousSymbolsandPictographs::Banana => '🍌',
            MiscellaneousSymbolsandPictographs::Pineapple => '🍍',
            MiscellaneousSymbolsandPictographs::RedApple => '🍎',
            MiscellaneousSymbolsandPictographs::GreenApple => '🍏',
            MiscellaneousSymbolsandPictographs::Pear => '🍐',
            MiscellaneousSymbolsandPictographs::Peach => '🍑',
            MiscellaneousSymbolsandPictographs::Cherries => '🍒',
            MiscellaneousSymbolsandPictographs::Strawberry => '🍓',
            MiscellaneousSymbolsandPictographs::Hamburger => '🍔',
            MiscellaneousSymbolsandPictographs::SliceOfPizza => '🍕',
            MiscellaneousSymbolsandPictographs::MeatOnBone => '🍖',
            MiscellaneousSymbolsandPictographs::PoultryLeg => '🍗',
            MiscellaneousSymbolsandPictographs::RiceCracker => '🍘',
            MiscellaneousSymbolsandPictographs::RiceBall => '🍙',
            MiscellaneousSymbolsandPictographs::CookedRice => '🍚',
            MiscellaneousSymbolsandPictographs::CurryAndRice => '🍛',
            MiscellaneousSymbolsandPictographs::SteamingBowl => '🍜',
            MiscellaneousSymbolsandPictographs::Spaghetti => '🍝',
            MiscellaneousSymbolsandPictographs::Bread => '🍞',
            MiscellaneousSymbolsandPictographs::FrenchFries => '🍟',
            MiscellaneousSymbolsandPictographs::RoastedSweetPotato => '🍠',
            MiscellaneousSymbolsandPictographs::Dango => '🍡',
            MiscellaneousSymbolsandPictographs::Oden => '🍢',
            MiscellaneousSymbolsandPictographs::Sushi => '🍣',
            MiscellaneousSymbolsandPictographs::FriedShrimp => '🍤',
            MiscellaneousSymbolsandPictographs::FishCakeWithSwirlDesign => '🍥',
            MiscellaneousSymbolsandPictographs::SoftIceCream => '🍦',
            MiscellaneousSymbolsandPictographs::ShavedIce => '🍧',
            MiscellaneousSymbolsandPictographs::IceCream => '🍨',
            MiscellaneousSymbolsandPictographs::Doughnut => '🍩',
            MiscellaneousSymbolsandPictographs::Cookie => '🍪',
            MiscellaneousSymbolsandPictographs::ChocolateBar => '🍫',
            MiscellaneousSymbolsandPictographs::Candy => '🍬',
            MiscellaneousSymbolsandPictographs::Lollipop => '🍭',
            MiscellaneousSymbolsandPictographs::Custard => '🍮',
            MiscellaneousSymbolsandPictographs::HoneyPot => '🍯',
            MiscellaneousSymbolsandPictographs::Shortcake => '🍰',
            MiscellaneousSymbolsandPictographs::BentoBox => '🍱',
            MiscellaneousSymbolsandPictographs::PotOfFood => '🍲',
            MiscellaneousSymbolsandPictographs::Cooking => '🍳',
            MiscellaneousSymbolsandPictographs::ForkAndKnife => '🍴',
            MiscellaneousSymbolsandPictographs::TeacupWithoutHandle => '🍵',
            MiscellaneousSymbolsandPictographs::SakeBottleAndCup => '🍶',
            MiscellaneousSymbolsandPictographs::WineGlass => '🍷',
            MiscellaneousSymbolsandPictographs::CocktailGlass => '🍸',
            MiscellaneousSymbolsandPictographs::TropicalDrink => '🍹',
            MiscellaneousSymbolsandPictographs::BeerMug => '🍺',
            MiscellaneousSymbolsandPictographs::ClinkingBeerMugs => '🍻',
            MiscellaneousSymbolsandPictographs::BabyBottle => '🍼',
            MiscellaneousSymbolsandPictographs::ForkAndKnifeWithPlate => '🍽',
            MiscellaneousSymbolsandPictographs::BottleWithPoppingCork => '🍾',
            MiscellaneousSymbolsandPictographs::Popcorn => '🍿',
            MiscellaneousSymbolsandPictographs::Ribbon => '🎀',
            MiscellaneousSymbolsandPictographs::WrappedPresent => '🎁',
            MiscellaneousSymbolsandPictographs::BirthdayCake => '🎂',
            MiscellaneousSymbolsandPictographs::JackDashODashLantern => '🎃',
            MiscellaneousSymbolsandPictographs::ChristmasTree => '🎄',
            MiscellaneousSymbolsandPictographs::FatherChristmas => '🎅',
            MiscellaneousSymbolsandPictographs::Fireworks => '🎆',
            MiscellaneousSymbolsandPictographs::FireworkSparkler => '🎇',
            MiscellaneousSymbolsandPictographs::Balloon => '🎈',
            MiscellaneousSymbolsandPictographs::PartyPopper => '🎉',
            MiscellaneousSymbolsandPictographs::ConfettiBall => '🎊',
            MiscellaneousSymbolsandPictographs::TanabataTree => '🎋',
            MiscellaneousSymbolsandPictographs::CrossedFlags => '🎌',
            MiscellaneousSymbolsandPictographs::PineDecoration => '🎍',
            MiscellaneousSymbolsandPictographs::JapaneseDolls => '🎎',
            MiscellaneousSymbolsandPictographs::CarpStreamer => '🎏',
            MiscellaneousSymbolsandPictographs::WindChime => '🎐',
            MiscellaneousSymbolsandPictographs::MoonViewingCeremony => '🎑',
            MiscellaneousSymbolsandPictographs::SchoolSatchel => '🎒',
            MiscellaneousSymbolsandPictographs::GraduationCap => '🎓',
            MiscellaneousSymbolsandPictographs::HeartWithTipOnTheLeft => '🎔',
            MiscellaneousSymbolsandPictographs::BouquetOfFlowers => '🎕',
            MiscellaneousSymbolsandPictographs::MilitaryMedal => '🎖',
            MiscellaneousSymbolsandPictographs::ReminderRibbon => '🎗',
            MiscellaneousSymbolsandPictographs::MusicalKeyboardWithJacks => '🎘',
            MiscellaneousSymbolsandPictographs::StudioMicrophone => '🎙',
            MiscellaneousSymbolsandPictographs::LevelSlider => '🎚',
            MiscellaneousSymbolsandPictographs::ControlKnobs => '🎛',
            MiscellaneousSymbolsandPictographs::BeamedAscendingMusicalNotes => '🎜',
            MiscellaneousSymbolsandPictographs::BeamedDescendingMusicalNotes => '🎝',
            MiscellaneousSymbolsandPictographs::FilmFrames => '🎞',
            MiscellaneousSymbolsandPictographs::AdmissionTickets => '🎟',
            MiscellaneousSymbolsandPictographs::CarouselHorse => '🎠',
            MiscellaneousSymbolsandPictographs::FerrisWheel => '🎡',
            MiscellaneousSymbolsandPictographs::RollerCoaster => '🎢',
            MiscellaneousSymbolsandPictographs::FishingPoleAndFish => '🎣',
            MiscellaneousSymbolsandPictographs::Microphone => '🎤',
            MiscellaneousSymbolsandPictographs::MovieCamera => '🎥',
            MiscellaneousSymbolsandPictographs::Cinema => '🎦',
            MiscellaneousSymbolsandPictographs::Headphone => '🎧',
            MiscellaneousSymbolsandPictographs::ArtistPalette => '🎨',
            MiscellaneousSymbolsandPictographs::TopHat => '🎩',
            MiscellaneousSymbolsandPictographs::CircusTent => '🎪',
            MiscellaneousSymbolsandPictographs::Ticket => '🎫',
            MiscellaneousSymbolsandPictographs::ClapperBoard => '🎬',
            MiscellaneousSymbolsandPictographs::PerformingArts => '🎭',
            MiscellaneousSymbolsandPictographs::VideoGame => '🎮',
            MiscellaneousSymbolsandPictographs::DirectHit => '🎯',
            MiscellaneousSymbolsandPictographs::SlotMachine => '🎰',
            MiscellaneousSymbolsandPictographs::Billiards => '🎱',
            MiscellaneousSymbolsandPictographs::GameDie => '🎲',
            MiscellaneousSymbolsandPictographs::Bowling => '🎳',
            MiscellaneousSymbolsandPictographs::FlowerPlayingCards => '🎴',
            MiscellaneousSymbolsandPictographs::MusicalNote => '🎵',
            MiscellaneousSymbolsandPictographs::MultipleMusicalNotes => '🎶',
            MiscellaneousSymbolsandPictographs::Saxophone => '🎷',
            MiscellaneousSymbolsandPictographs::Guitar => '🎸',
            MiscellaneousSymbolsandPictographs::MusicalKeyboard => '🎹',
            MiscellaneousSymbolsandPictographs::Trumpet => '🎺',
            MiscellaneousSymbolsandPictographs::Violin => '🎻',
            MiscellaneousSymbolsandPictographs::MusicalScore => '🎼',
            MiscellaneousSymbolsandPictographs::RunningShirtWithSash => '🎽',
            MiscellaneousSymbolsandPictographs::TennisRacquetAndBall => '🎾',
            MiscellaneousSymbolsandPictographs::SkiAndSkiBoot => '🎿',
            MiscellaneousSymbolsandPictographs::BasketballAndHoop => '🏀',
            MiscellaneousSymbolsandPictographs::ChequeredFlag => '🏁',
            MiscellaneousSymbolsandPictographs::Snowboarder => '🏂',
            MiscellaneousSymbolsandPictographs::Runner => '🏃',
            MiscellaneousSymbolsandPictographs::Surfer => '🏄',
            MiscellaneousSymbolsandPictographs::SportsMedal => '🏅',
            MiscellaneousSymbolsandPictographs::Trophy => '🏆',
            MiscellaneousSymbolsandPictographs::HorseRacing => '🏇',
            MiscellaneousSymbolsandPictographs::AmericanFootball => '🏈',
            MiscellaneousSymbolsandPictographs::RugbyFootball => '🏉',
            MiscellaneousSymbolsandPictographs::Swimmer => '🏊',
            MiscellaneousSymbolsandPictographs::WeightLifter => '🏋',
            MiscellaneousSymbolsandPictographs::Golfer => '🏌',
            MiscellaneousSymbolsandPictographs::RacingMotorcycle => '🏍',
            MiscellaneousSymbolsandPictographs::RacingCar => '🏎',
            MiscellaneousSymbolsandPictographs::CricketBatAndBall => '🏏',
            MiscellaneousSymbolsandPictographs::Volleyball => '🏐',
            MiscellaneousSymbolsandPictographs::FieldHockeyStickAndBall => '🏑',
            MiscellaneousSymbolsandPictographs::IceHockeyStickAndPuck => '🏒',
            MiscellaneousSymbolsandPictographs::TableTennisPaddleAndBall => '🏓',
            MiscellaneousSymbolsandPictographs::SnowCappedMountain => '🏔',
            MiscellaneousSymbolsandPictographs::Camping => '🏕',
            MiscellaneousSymbolsandPictographs::BeachWithUmbrella => '🏖',
            MiscellaneousSymbolsandPictographs::BuildingConstruction => '🏗',
            MiscellaneousSymbolsandPictographs::HouseBuildings => '🏘',
            MiscellaneousSymbolsandPictographs::Cityscape => '🏙',
            MiscellaneousSymbolsandPictographs::DerelictHouseBuilding => '🏚',
            MiscellaneousSymbolsandPictographs::ClassicalBuilding => '🏛',
            MiscellaneousSymbolsandPictographs::Desert => '🏜',
            MiscellaneousSymbolsandPictographs::DesertIsland => '🏝',
            MiscellaneousSymbolsandPictographs::NationalPark => '🏞',
            MiscellaneousSymbolsandPictographs::Stadium => '🏟',
            MiscellaneousSymbolsandPictographs::HouseBuilding => '🏠',
            MiscellaneousSymbolsandPictographs::HouseWithGarden => '🏡',
            MiscellaneousSymbolsandPictographs::OfficeBuilding => '🏢',
            MiscellaneousSymbolsandPictographs::JapanesePostOffice => '🏣',
            MiscellaneousSymbolsandPictographs::EuropeanPostOffice => '🏤',
            MiscellaneousSymbolsandPictographs::Hospital => '🏥',
            MiscellaneousSymbolsandPictographs::Bank => '🏦',
            MiscellaneousSymbolsandPictographs::AutomatedTellerMachine => '🏧',
            MiscellaneousSymbolsandPictographs::Hotel => '🏨',
            MiscellaneousSymbolsandPictographs::LoveHotel => '🏩',
            MiscellaneousSymbolsandPictographs::ConvenienceStore => '🏪',
            MiscellaneousSymbolsandPictographs::School => '🏫',
            MiscellaneousSymbolsandPictographs::DepartmentStore => '🏬',
            MiscellaneousSymbolsandPictographs::Factory => '🏭',
            MiscellaneousSymbolsandPictographs::IzakayaLantern => '🏮',
            MiscellaneousSymbolsandPictographs::JapaneseCastle => '🏯',
            MiscellaneousSymbolsandPictographs::EuropeanCastle => '🏰',
            MiscellaneousSymbolsandPictographs::WhitePennant => '🏱',
            MiscellaneousSymbolsandPictographs::BlackPennant => '🏲',
            MiscellaneousSymbolsandPictographs::WavingWhiteFlag => '🏳',
            MiscellaneousSymbolsandPictographs::WavingBlackFlag => '🏴',
            MiscellaneousSymbolsandPictographs::Rosette => '🏵',
            MiscellaneousSymbolsandPictographs::BlackRosette => '🏶',
            MiscellaneousSymbolsandPictographs::Label => '🏷',
            MiscellaneousSymbolsandPictographs::BadmintonRacquetAndShuttlecock => '🏸',
            MiscellaneousSymbolsandPictographs::BowAndArrow => '🏹',
            MiscellaneousSymbolsandPictographs::Amphora => '🏺',
            MiscellaneousSymbolsandPictographs::EmojiModifierFitzpatrickTypeDash1Dash2 => '🏻',
            MiscellaneousSymbolsandPictographs::EmojiModifierFitzpatrickTypeDash3 => '🏼',
            MiscellaneousSymbolsandPictographs::EmojiModifierFitzpatrickTypeDash4 => '🏽',
            MiscellaneousSymbolsandPictographs::EmojiModifierFitzpatrickTypeDash5 => '🏾',
            MiscellaneousSymbolsandPictographs::EmojiModifierFitzpatrickTypeDash6 => '🏿',
            MiscellaneousSymbolsandPictographs::Rat => '🐀',
            MiscellaneousSymbolsandPictographs::Mouse => '🐁',
            MiscellaneousSymbolsandPictographs::Ox => '🐂',
            MiscellaneousSymbolsandPictographs::WaterBuffalo => '🐃',
            MiscellaneousSymbolsandPictographs::Cow => '🐄',
            MiscellaneousSymbolsandPictographs::Tiger => '🐅',
            MiscellaneousSymbolsandPictographs::Leopard => '🐆',
            MiscellaneousSymbolsandPictographs::Rabbit => '🐇',
            MiscellaneousSymbolsandPictographs::Cat => '🐈',
            MiscellaneousSymbolsandPictographs::Dragon => '🐉',
            MiscellaneousSymbolsandPictographs::Crocodile => '🐊',
            MiscellaneousSymbolsandPictographs::Whale => '🐋',
            MiscellaneousSymbolsandPictographs::Snail => '🐌',
            MiscellaneousSymbolsandPictographs::Snake => '🐍',
            MiscellaneousSymbolsandPictographs::Horse => '🐎',
            MiscellaneousSymbolsandPictographs::Ram => '🐏',
            MiscellaneousSymbolsandPictographs::Goat => '🐐',
            MiscellaneousSymbolsandPictographs::Sheep => '🐑',
            MiscellaneousSymbolsandPictographs::Monkey => '🐒',
            MiscellaneousSymbolsandPictographs::Rooster => '🐓',
            MiscellaneousSymbolsandPictographs::Chicken => '🐔',
            MiscellaneousSymbolsandPictographs::Dog => '🐕',
            MiscellaneousSymbolsandPictographs::Pig => '🐖',
            MiscellaneousSymbolsandPictographs::Boar => '🐗',
            MiscellaneousSymbolsandPictographs::Elephant => '🐘',
            MiscellaneousSymbolsandPictographs::Octopus => '🐙',
            MiscellaneousSymbolsandPictographs::SpiralShell => '🐚',
            MiscellaneousSymbolsandPictographs::Bug => '🐛',
            MiscellaneousSymbolsandPictographs::Ant => '🐜',
            MiscellaneousSymbolsandPictographs::Honeybee => '🐝',
            MiscellaneousSymbolsandPictographs::LadyBeetle => '🐞',
            MiscellaneousSymbolsandPictographs::Fish => '🐟',
            MiscellaneousSymbolsandPictographs::TropicalFish => '🐠',
            MiscellaneousSymbolsandPictographs::Blowfish => '🐡',
            MiscellaneousSymbolsandPictographs::Turtle => '🐢',
            MiscellaneousSymbolsandPictographs::HatchingChick => '🐣',
            MiscellaneousSymbolsandPictographs::BabyChick => '🐤',
            MiscellaneousSymbolsandPictographs::FrontDashFacingBabyChick => '🐥',
            MiscellaneousSymbolsandPictographs::Bird => '🐦',
            MiscellaneousSymbolsandPictographs::Penguin => '🐧',
            MiscellaneousSymbolsandPictographs::Koala => '🐨',
            MiscellaneousSymbolsandPictographs::Poodle => '🐩',
            MiscellaneousSymbolsandPictographs::DromedaryCamel => '🐪',
            MiscellaneousSymbolsandPictographs::BactrianCamel => '🐫',
            MiscellaneousSymbolsandPictographs::Dolphin => '🐬',
            MiscellaneousSymbolsandPictographs::MouseFace => '🐭',
            MiscellaneousSymbolsandPictographs::CowFace => '🐮',
            MiscellaneousSymbolsandPictographs::TigerFace => '🐯',
            MiscellaneousSymbolsandPictographs::RabbitFace => '🐰',
            MiscellaneousSymbolsandPictographs::CatFace => '🐱',
            MiscellaneousSymbolsandPictographs::DragonFace => '🐲',
            MiscellaneousSymbolsandPictographs::SpoutingWhale => '🐳',
            MiscellaneousSymbolsandPictographs::HorseFace => '🐴',
            MiscellaneousSymbolsandPictographs::MonkeyFace => '🐵',
            MiscellaneousSymbolsandPictographs::DogFace => '🐶',
            MiscellaneousSymbolsandPictographs::PigFace => '🐷',
            MiscellaneousSymbolsandPictographs::FrogFace => '🐸',
            MiscellaneousSymbolsandPictographs::HamsterFace => '🐹',
            MiscellaneousSymbolsandPictographs::WolfFace => '🐺',
            MiscellaneousSymbolsandPictographs::BearFace => '🐻',
            MiscellaneousSymbolsandPictographs::PandaFace => '🐼',
            MiscellaneousSymbolsandPictographs::PigNose => '🐽',
            MiscellaneousSymbolsandPictographs::PawPrints => '🐾',
            MiscellaneousSymbolsandPictographs::Chipmunk => '🐿',
            MiscellaneousSymbolsandPictographs::Eyes => '👀',
            MiscellaneousSymbolsandPictographs::Eye => '👁',
            MiscellaneousSymbolsandPictographs::Ear => '👂',
            MiscellaneousSymbolsandPictographs::Nose => '👃',
            MiscellaneousSymbolsandPictographs::Mouth => '👄',
            MiscellaneousSymbolsandPictographs::Tongue => '👅',
            MiscellaneousSymbolsandPictographs::WhiteUpPointingBackhandIndex => '👆',
            MiscellaneousSymbolsandPictographs::WhiteDownPointingBackhandIndex => '👇',
            MiscellaneousSymbolsandPictographs::WhiteLeftPointingBackhandIndex => '👈',
            MiscellaneousSymbolsandPictographs::WhiteRightPointingBackhandIndex => '👉',
            MiscellaneousSymbolsandPictographs::FistedHandSign => '👊',
            MiscellaneousSymbolsandPictographs::WavingHandSign => '👋',
            MiscellaneousSymbolsandPictographs::OkHandSign => '👌',
            MiscellaneousSymbolsandPictographs::ThumbsUpSign => '👍',
            MiscellaneousSymbolsandPictographs::ThumbsDownSign => '👎',
            MiscellaneousSymbolsandPictographs::ClappingHandsSign => '👏',
            MiscellaneousSymbolsandPictographs::OpenHandsSign => '👐',
            MiscellaneousSymbolsandPictographs::Crown => '👑',
            MiscellaneousSymbolsandPictographs::WomansHat => '👒',
            MiscellaneousSymbolsandPictographs::Eyeglasses => '👓',
            MiscellaneousSymbolsandPictographs::Necktie => '👔',
            MiscellaneousSymbolsandPictographs::TDashShirt => '👕',
            MiscellaneousSymbolsandPictographs::Jeans => '👖',
            MiscellaneousSymbolsandPictographs::Dress => '👗',
            MiscellaneousSymbolsandPictographs::Kimono => '👘',
            MiscellaneousSymbolsandPictographs::Bikini => '👙',
            MiscellaneousSymbolsandPictographs::WomansClothes => '👚',
            MiscellaneousSymbolsandPictographs::Purse => '👛',
            MiscellaneousSymbolsandPictographs::Handbag => '👜',
            MiscellaneousSymbolsandPictographs::Pouch => '👝',
            MiscellaneousSymbolsandPictographs::MansShoe => '👞',
            MiscellaneousSymbolsandPictographs::AthleticShoe => '👟',
            MiscellaneousSymbolsandPictographs::HighDashHeeledShoe => '👠',
            MiscellaneousSymbolsandPictographs::WomansSandal => '👡',
            MiscellaneousSymbolsandPictographs::WomansBoots => '👢',
            MiscellaneousSymbolsandPictographs::Footprints => '👣',
            MiscellaneousSymbolsandPictographs::BustInSilhouette => '👤',
            MiscellaneousSymbolsandPictographs::BustsInSilhouette => '👥',
            MiscellaneousSymbolsandPictographs::Boy => '👦',
            MiscellaneousSymbolsandPictographs::Girl => '👧',
            MiscellaneousSymbolsandPictographs::Man => '👨',
            MiscellaneousSymbolsandPictographs::Woman => '👩',
            MiscellaneousSymbolsandPictographs::Family => '👪',
            MiscellaneousSymbolsandPictographs::ManAndWomanHoldingHands => '👫',
            MiscellaneousSymbolsandPictographs::TwoMenHoldingHands => '👬',
            MiscellaneousSymbolsandPictographs::TwoWomenHoldingHands => '👭',
            MiscellaneousSymbolsandPictographs::PoliceOfficer => '👮',
            MiscellaneousSymbolsandPictographs::WomanWithBunnyEars => '👯',
            MiscellaneousSymbolsandPictographs::BrideWithVeil => '👰',
            MiscellaneousSymbolsandPictographs::PersonWithBlondHair => '👱',
            MiscellaneousSymbolsandPictographs::ManWithGuaPiMao => '👲',
            MiscellaneousSymbolsandPictographs::ManWithTurban => '👳',
            MiscellaneousSymbolsandPictographs::OlderMan => '👴',
            MiscellaneousSymbolsandPictographs::OlderWoman => '👵',
            MiscellaneousSymbolsandPictographs::Baby => '👶',
            MiscellaneousSymbolsandPictographs::ConstructionWorker => '👷',
            MiscellaneousSymbolsandPictographs::Princess => '👸',
            MiscellaneousSymbolsandPictographs::JapaneseOgre => '👹',
            MiscellaneousSymbolsandPictographs::JapaneseGoblin => '👺',
            MiscellaneousSymbolsandPictographs::Ghost => '👻',
            MiscellaneousSymbolsandPictographs::BabyAngel => '👼',
            MiscellaneousSymbolsandPictographs::ExtraterrestrialAlien => '👽',
            MiscellaneousSymbolsandPictographs::AlienMonster => '👾',
            MiscellaneousSymbolsandPictographs::Imp => '👿',
            MiscellaneousSymbolsandPictographs::Skull => '💀',
            MiscellaneousSymbolsandPictographs::InformationDeskPerson => '💁',
            MiscellaneousSymbolsandPictographs::Guardsman => '💂',
            MiscellaneousSymbolsandPictographs::Dancer => '💃',
            MiscellaneousSymbolsandPictographs::Lipstick => '💄',
            MiscellaneousSymbolsandPictographs::NailPolish => '💅',
            MiscellaneousSymbolsandPictographs::FaceMassage => '💆',
            MiscellaneousSymbolsandPictographs::Haircut => '💇',
            MiscellaneousSymbolsandPictographs::BarberPole => '💈',
            MiscellaneousSymbolsandPictographs::Syringe => '💉',
            MiscellaneousSymbolsandPictographs::Pill => '💊',
            MiscellaneousSymbolsandPictographs::KissMark => '💋',
            MiscellaneousSymbolsandPictographs::LoveLetter => '💌',
            MiscellaneousSymbolsandPictographs::Ring => '💍',
            MiscellaneousSymbolsandPictographs::GemStone => '💎',
            MiscellaneousSymbolsandPictographs::Kiss => '💏',
            MiscellaneousSymbolsandPictographs::Bouquet => '💐',
            MiscellaneousSymbolsandPictographs::CoupleWithHeart => '💑',
            MiscellaneousSymbolsandPictographs::Wedding => '💒',
            MiscellaneousSymbolsandPictographs::BeatingHeart => '💓',
            MiscellaneousSymbolsandPictographs::BrokenHeart => '💔',
            MiscellaneousSymbolsandPictographs::TwoHearts => '💕',
            MiscellaneousSymbolsandPictographs::SparklingHeart => '💖',
            MiscellaneousSymbolsandPictographs::GrowingHeart => '💗',
            MiscellaneousSymbolsandPictographs::HeartWithArrow => '💘',
            MiscellaneousSymbolsandPictographs::BlueHeart => '💙',
            MiscellaneousSymbolsandPictographs::GreenHeart => '💚',
            MiscellaneousSymbolsandPictographs::YellowHeart => '💛',
            MiscellaneousSymbolsandPictographs::PurpleHeart => '💜',
            MiscellaneousSymbolsandPictographs::HeartWithRibbon => '💝',
            MiscellaneousSymbolsandPictographs::RevolvingHearts => '💞',
            MiscellaneousSymbolsandPictographs::HeartDecoration => '💟',
            MiscellaneousSymbolsandPictographs::DiamondShapeWithADotInside => '💠',
            MiscellaneousSymbolsandPictographs::ElectricLightBulb => '💡',
            MiscellaneousSymbolsandPictographs::AngerSymbol => '💢',
            MiscellaneousSymbolsandPictographs::Bomb => '💣',
            MiscellaneousSymbolsandPictographs::SleepingSymbol => '💤',
            MiscellaneousSymbolsandPictographs::CollisionSymbol => '💥',
            MiscellaneousSymbolsandPictographs::SplashingSweatSymbol => '💦',
            MiscellaneousSymbolsandPictographs::Droplet => '💧',
            MiscellaneousSymbolsandPictographs::DashSymbol => '💨',
            MiscellaneousSymbolsandPictographs::PileOfPoo => '💩',
            MiscellaneousSymbolsandPictographs::FlexedBiceps => '💪',
            MiscellaneousSymbolsandPictographs::DizzySymbol => '💫',
            MiscellaneousSymbolsandPictographs::SpeechBalloon => '💬',
            MiscellaneousSymbolsandPictographs::ThoughtBalloon => '💭',
            MiscellaneousSymbolsandPictographs::WhiteFlower => '💮',
            MiscellaneousSymbolsandPictographs::HundredPointsSymbol => '💯',
            MiscellaneousSymbolsandPictographs::MoneyBag => '💰',
            MiscellaneousSymbolsandPictographs::CurrencyExchange => '💱',
            MiscellaneousSymbolsandPictographs::HeavyDollarSign => '💲',
            MiscellaneousSymbolsandPictographs::CreditCard => '💳',
            MiscellaneousSymbolsandPictographs::BanknoteWithYenSign => '💴',
            MiscellaneousSymbolsandPictographs::BanknoteWithDollarSign => '💵',
            MiscellaneousSymbolsandPictographs::BanknoteWithEuroSign => '💶',
            MiscellaneousSymbolsandPictographs::BanknoteWithPoundSign => '💷',
            MiscellaneousSymbolsandPictographs::MoneyWithWings => '💸',
            MiscellaneousSymbolsandPictographs::ChartWithUpwardsTrendAndYenSign => '💹',
            MiscellaneousSymbolsandPictographs::Seat => '💺',
            MiscellaneousSymbolsandPictographs::PersonalComputer => '💻',
            MiscellaneousSymbolsandPictographs::Briefcase => '💼',
            MiscellaneousSymbolsandPictographs::Minidisc => '💽',
            MiscellaneousSymbolsandPictographs::FloppyDisk => '💾',
            MiscellaneousSymbolsandPictographs::OpticalDisc => '💿',
            MiscellaneousSymbolsandPictographs::Dvd => '📀',
            MiscellaneousSymbolsandPictographs::FileFolder => '📁',
            MiscellaneousSymbolsandPictographs::OpenFileFolder => '📂',
            MiscellaneousSymbolsandPictographs::PageWithCurl => '📃',
            MiscellaneousSymbolsandPictographs::PageFacingUp => '📄',
            MiscellaneousSymbolsandPictographs::Calendar => '📅',
            MiscellaneousSymbolsandPictographs::TearDashOffCalendar => '📆',
            MiscellaneousSymbolsandPictographs::CardIndex => '📇',
            MiscellaneousSymbolsandPictographs::ChartWithUpwardsTrend => '📈',
            MiscellaneousSymbolsandPictographs::ChartWithDownwardsTrend => '📉',
            MiscellaneousSymbolsandPictographs::BarChart => '📊',
            MiscellaneousSymbolsandPictographs::Clipboard => '📋',
            MiscellaneousSymbolsandPictographs::Pushpin => '📌',
            MiscellaneousSymbolsandPictographs::RoundPushpin => '📍',
            MiscellaneousSymbolsandPictographs::Paperclip => '📎',
            MiscellaneousSymbolsandPictographs::StraightRuler => '📏',
            MiscellaneousSymbolsandPictographs::TriangularRuler => '📐',
            MiscellaneousSymbolsandPictographs::BookmarkTabs => '📑',
            MiscellaneousSymbolsandPictographs::Ledger => '📒',
            MiscellaneousSymbolsandPictographs::Notebook => '📓',
            MiscellaneousSymbolsandPictographs::NotebookWithDecorativeCover => '📔',
            MiscellaneousSymbolsandPictographs::ClosedBook => '📕',
            MiscellaneousSymbolsandPictographs::OpenBook => '📖',
            MiscellaneousSymbolsandPictographs::GreenBook => '📗',
            MiscellaneousSymbolsandPictographs::BlueBook => '📘',
            MiscellaneousSymbolsandPictographs::OrangeBook => '📙',
            MiscellaneousSymbolsandPictographs::Books => '📚',
            MiscellaneousSymbolsandPictographs::NameBadge => '📛',
            MiscellaneousSymbolsandPictographs::Scroll => '📜',
            MiscellaneousSymbolsandPictographs::Memo => '📝',
            MiscellaneousSymbolsandPictographs::TelephoneReceiver => '📞',
            MiscellaneousSymbolsandPictographs::Pager => '📟',
            MiscellaneousSymbolsandPictographs::FaxMachine => '📠',
            MiscellaneousSymbolsandPictographs::SatelliteAntenna => '📡',
            MiscellaneousSymbolsandPictographs::PublicAddressLoudspeaker => '📢',
            MiscellaneousSymbolsandPictographs::CheeringMegaphone => '📣',
            MiscellaneousSymbolsandPictographs::OutboxTray => '📤',
            MiscellaneousSymbolsandPictographs::InboxTray => '📥',
            MiscellaneousSymbolsandPictographs::Package => '📦',
            MiscellaneousSymbolsandPictographs::EDashMailSymbol => '📧',
            MiscellaneousSymbolsandPictographs::IncomingEnvelope => '📨',
            MiscellaneousSymbolsandPictographs::EnvelopeWithDownwardsArrowAbove => '📩',
            MiscellaneousSymbolsandPictographs::ClosedMailboxWithLoweredFlag => '📪',
            MiscellaneousSymbolsandPictographs::ClosedMailboxWithRaisedFlag => '📫',
            MiscellaneousSymbolsandPictographs::OpenMailboxWithRaisedFlag => '📬',
            MiscellaneousSymbolsandPictographs::OpenMailboxWithLoweredFlag => '📭',
            MiscellaneousSymbolsandPictographs::Postbox => '📮',
            MiscellaneousSymbolsandPictographs::PostalHorn => '📯',
            MiscellaneousSymbolsandPictographs::Newspaper => '📰',
            MiscellaneousSymbolsandPictographs::MobilePhone => '📱',
            MiscellaneousSymbolsandPictographs::MobilePhoneWithRightwardsArrowAtLeft => '📲',
            MiscellaneousSymbolsandPictographs::VibrationMode => '📳',
            MiscellaneousSymbolsandPictographs::MobilePhoneOff => '📴',
            MiscellaneousSymbolsandPictographs::NoMobilePhones => '📵',
            MiscellaneousSymbolsandPictographs::AntennaWithBars => '📶',
            MiscellaneousSymbolsandPictographs::Camera => '📷',
            MiscellaneousSymbolsandPictographs::CameraWithFlash => '📸',
            MiscellaneousSymbolsandPictographs::VideoCamera => '📹',
            MiscellaneousSymbolsandPictographs::Television => '📺',
            MiscellaneousSymbolsandPictographs::Radio => '📻',
            MiscellaneousSymbolsandPictographs::Videocassette => '📼',
            MiscellaneousSymbolsandPictographs::FilmProjector => '📽',
            MiscellaneousSymbolsandPictographs::PortableStereo => '📾',
            MiscellaneousSymbolsandPictographs::PrayerBeads => '📿',
            MiscellaneousSymbolsandPictographs::TwistedRightwardsArrows => '🔀',
            MiscellaneousSymbolsandPictographs::ClockwiseRightwardsAndLeftwardsOpenCircleArrows => '🔁',
            MiscellaneousSymbolsandPictographs::ClockwiseRightwardsAndLeftwardsOpenCircleArrowsWithCircledOneOverlay => '🔂',
            MiscellaneousSymbolsandPictographs::ClockwiseDownwardsAndUpwardsOpenCircleArrows => '🔃',
            MiscellaneousSymbolsandPictographs::AnticlockwiseDownwardsAndUpwardsOpenCircleArrows => '🔄',
            MiscellaneousSymbolsandPictographs::LowBrightnessSymbol => '🔅',
            MiscellaneousSymbolsandPictographs::HighBrightnessSymbol => '🔆',
            MiscellaneousSymbolsandPictographs::SpeakerWithCancellationStroke => '🔇',
            MiscellaneousSymbolsandPictographs::Speaker => '🔈',
            MiscellaneousSymbolsandPictographs::SpeakerWithOneSoundWave => '🔉',
            MiscellaneousSymbolsandPictographs::SpeakerWithThreeSoundWaves => '🔊',
            MiscellaneousSymbolsandPictographs::Battery => '🔋',
            MiscellaneousSymbolsandPictographs::ElectricPlug => '🔌',
            MiscellaneousSymbolsandPictographs::LeftDashPointingMagnifyingGlass => '🔍',
            MiscellaneousSymbolsandPictographs::RightDashPointingMagnifyingGlass => '🔎',
            MiscellaneousSymbolsandPictographs::LockWithInkPen => '🔏',
            MiscellaneousSymbolsandPictographs::ClosedLockWithKey => '🔐',
            MiscellaneousSymbolsandPictographs::Key => '🔑',
            MiscellaneousSymbolsandPictographs::Lock => '🔒',
            MiscellaneousSymbolsandPictographs::OpenLock => '🔓',
            MiscellaneousSymbolsandPictographs::Bell => '🔔',
            MiscellaneousSymbolsandPictographs::BellWithCancellationStroke => '🔕',
            MiscellaneousSymbolsandPictographs::Bookmark => '🔖',
            MiscellaneousSymbolsandPictographs::LinkSymbol => '🔗',
            MiscellaneousSymbolsandPictographs::RadioButton => '🔘',
            MiscellaneousSymbolsandPictographs::BackWithLeftwardsArrowAbove => '🔙',
            MiscellaneousSymbolsandPictographs::EndWithLeftwardsArrowAbove => '🔚',
            MiscellaneousSymbolsandPictographs::OnWithExclamationMarkWithLeftRightArrowAbove => '🔛',
            MiscellaneousSymbolsandPictographs::SoonWithRightwardsArrowAbove => '🔜',
            MiscellaneousSymbolsandPictographs::TopWithUpwardsArrowAbove => '🔝',
            MiscellaneousSymbolsandPictographs::NoOneUnderEighteenSymbol => '🔞',
            MiscellaneousSymbolsandPictographs::KeycapTen => '🔟',
            MiscellaneousSymbolsandPictographs::InputSymbolForLatinCapitalLetters => '🔠',
            MiscellaneousSymbolsandPictographs::InputSymbolForLatinSmallLetters => '🔡',
            MiscellaneousSymbolsandPictographs::InputSymbolForNumbers => '🔢',
            MiscellaneousSymbolsandPictographs::InputSymbolForSymbols => '🔣',
            MiscellaneousSymbolsandPictographs::InputSymbolForLatinLetters => '🔤',
            MiscellaneousSymbolsandPictographs::Fire => '🔥',
            MiscellaneousSymbolsandPictographs::ElectricTorch => '🔦',
            MiscellaneousSymbolsandPictographs::Wrench => '🔧',
            MiscellaneousSymbolsandPictographs::Hammer => '🔨',
            MiscellaneousSymbolsandPictographs::NutAndBolt => '🔩',
            MiscellaneousSymbolsandPictographs::Hocho => '🔪',
            MiscellaneousSymbolsandPictographs::Pistol => '🔫',
            MiscellaneousSymbolsandPictographs::Microscope => '🔬',
            MiscellaneousSymbolsandPictographs::Telescope => '🔭',
            MiscellaneousSymbolsandPictographs::CrystalBall => '🔮',
            MiscellaneousSymbolsandPictographs::SixPointedStarWithMiddleDot => '🔯',
            MiscellaneousSymbolsandPictographs::JapaneseSymbolForBeginner => '🔰',
            MiscellaneousSymbolsandPictographs::TridentEmblem => '🔱',
            MiscellaneousSymbolsandPictographs::BlackSquareButton => '🔲',
            MiscellaneousSymbolsandPictographs::WhiteSquareButton => '🔳',
            MiscellaneousSymbolsandPictographs::LargeRedCircle => '🔴',
            MiscellaneousSymbolsandPictographs::LargeBlueCircle => '🔵',
            MiscellaneousSymbolsandPictographs::LargeOrangeDiamond => '🔶',
            MiscellaneousSymbolsandPictographs::LargeBlueDiamond => '🔷',
            MiscellaneousSymbolsandPictographs::SmallOrangeDiamond => '🔸',
            MiscellaneousSymbolsandPictographs::SmallBlueDiamond => '🔹',
            MiscellaneousSymbolsandPictographs::UpDashPointingRedTriangle => '🔺',
            MiscellaneousSymbolsandPictographs::DownDashPointingRedTriangle => '🔻',
            MiscellaneousSymbolsandPictographs::UpDashPointingSmallRedTriangle => '🔼',
            MiscellaneousSymbolsandPictographs::DownDashPointingSmallRedTriangle => '🔽',
            MiscellaneousSymbolsandPictographs::LowerRightShadowedWhiteCircle => '🔾',
            MiscellaneousSymbolsandPictographs::UpperRightShadowedWhiteCircle => '🔿',
            MiscellaneousSymbolsandPictographs::CircledCrossPommee => '🕀',
            MiscellaneousSymbolsandPictographs::CrossPommeeWithHalfDashCircleBelow => '🕁',
            MiscellaneousSymbolsandPictographs::CrossPommee => '🕂',
            MiscellaneousSymbolsandPictographs::NotchedLeftSemicircleWithThreeDots => '🕃',
            MiscellaneousSymbolsandPictographs::NotchedRightSemicircleWithThreeDots => '🕄',
            MiscellaneousSymbolsandPictographs::SymbolForMarksChapter => '🕅',
            MiscellaneousSymbolsandPictographs::WhiteLatinCross => '🕆',
            MiscellaneousSymbolsandPictographs::HeavyLatinCross => '🕇',
            MiscellaneousSymbolsandPictographs::CelticCross => '🕈',
            MiscellaneousSymbolsandPictographs::OmSymbol => '🕉',
            MiscellaneousSymbolsandPictographs::DoveOfPeace => '🕊',
            MiscellaneousSymbolsandPictographs::Kaaba => '🕋',
            MiscellaneousSymbolsandPictographs::Mosque => '🕌',
            MiscellaneousSymbolsandPictographs::Synagogue => '🕍',
            MiscellaneousSymbolsandPictographs::MenorahWithNineBranches => '🕎',
            MiscellaneousSymbolsandPictographs::BowlOfHygieia => '🕏',
            MiscellaneousSymbolsandPictographs::ClockFaceOneOclock => '🕐',
            MiscellaneousSymbolsandPictographs::ClockFaceTwoOclock => '🕑',
            MiscellaneousSymbolsandPictographs::ClockFaceThreeOclock => '🕒',
            MiscellaneousSymbolsandPictographs::ClockFaceFourOclock => '🕓',
            MiscellaneousSymbolsandPictographs::ClockFaceFiveOclock => '🕔',
            MiscellaneousSymbolsandPictographs::ClockFaceSixOclock => '🕕',
            MiscellaneousSymbolsandPictographs::ClockFaceSevenOclock => '🕖',
            MiscellaneousSymbolsandPictographs::ClockFaceEightOclock => '🕗',
            MiscellaneousSymbolsandPictographs::ClockFaceNineOclock => '🕘',
            MiscellaneousSymbolsandPictographs::ClockFaceTenOclock => '🕙',
            MiscellaneousSymbolsandPictographs::ClockFaceElevenOclock => '🕚',
            MiscellaneousSymbolsandPictographs::ClockFaceTwelveOclock => '🕛',
            MiscellaneousSymbolsandPictographs::ClockFaceOneDashThirty => '🕜',
            MiscellaneousSymbolsandPictographs::ClockFaceTwoDashThirty => '🕝',
            MiscellaneousSymbolsandPictographs::ClockFaceThreeDashThirty => '🕞',
            MiscellaneousSymbolsandPictographs::ClockFaceFourDashThirty => '🕟',
            MiscellaneousSymbolsandPictographs::ClockFaceFiveDashThirty => '🕠',
            MiscellaneousSymbolsandPictographs::ClockFaceSixDashThirty => '🕡',
            MiscellaneousSymbolsandPictographs::ClockFaceSevenDashThirty => '🕢',
            MiscellaneousSymbolsandPictographs::ClockFaceEightDashThirty => '🕣',
            MiscellaneousSymbolsandPictographs::ClockFaceNineDashThirty => '🕤',
            MiscellaneousSymbolsandPictographs::ClockFaceTenDashThirty => '🕥',
            MiscellaneousSymbolsandPictographs::ClockFaceElevenDashThirty => '🕦',
            MiscellaneousSymbolsandPictographs::ClockFaceTwelveDashThirty => '🕧',
            MiscellaneousSymbolsandPictographs::RightSpeaker => '🕨',
            MiscellaneousSymbolsandPictographs::RightSpeakerWithOneSoundWave => '🕩',
            MiscellaneousSymbolsandPictographs::RightSpeakerWithThreeSoundWaves => '🕪',
            MiscellaneousSymbolsandPictographs::Bullhorn => '🕫',
            MiscellaneousSymbolsandPictographs::BullhornWithSoundWaves => '🕬',
            MiscellaneousSymbolsandPictographs::RingingBell => '🕭',
            MiscellaneousSymbolsandPictographs::Book => '🕮',
            MiscellaneousSymbolsandPictographs::Candle => '🕯',
            MiscellaneousSymbolsandPictographs::MantelpieceClock => '🕰',
            MiscellaneousSymbolsandPictographs::BlackSkullAndCrossbones => '🕱',
            MiscellaneousSymbolsandPictographs::NoPiracy => '🕲',
            MiscellaneousSymbolsandPictographs::Hole => '🕳',
            MiscellaneousSymbolsandPictographs::ManInBusinessSuitLevitating => '🕴',
            MiscellaneousSymbolsandPictographs::SleuthOrSpy => '🕵',
            MiscellaneousSymbolsandPictographs::DarkSunglasses => '🕶',
            MiscellaneousSymbolsandPictographs::Spider => '🕷',
            MiscellaneousSymbolsandPictographs::SpiderWeb => '🕸',
            MiscellaneousSymbolsandPictographs::Joystick => '🕹',
            MiscellaneousSymbolsandPictographs::ManDancing => '🕺',
            MiscellaneousSymbolsandPictographs::LeftHandTelephoneReceiver => '🕻',
            MiscellaneousSymbolsandPictographs::TelephoneReceiverWithPage => '🕼',
            MiscellaneousSymbolsandPictographs::RightHandTelephoneReceiver => '🕽',
            MiscellaneousSymbolsandPictographs::WhiteTouchtoneTelephone => '🕾',
            MiscellaneousSymbolsandPictographs::BlackTouchtoneTelephone => '🕿',
            MiscellaneousSymbolsandPictographs::TelephoneOnTopOfModem => '🖀',
            MiscellaneousSymbolsandPictographs::ClamshellMobilePhone => '🖁',
            MiscellaneousSymbolsandPictographs::BackOfEnvelope => '🖂',
            MiscellaneousSymbolsandPictographs::StampedEnvelope => '🖃',
            MiscellaneousSymbolsandPictographs::EnvelopeWithLightning => '🖄',
            MiscellaneousSymbolsandPictographs::FlyingEnvelope => '🖅',
            MiscellaneousSymbolsandPictographs::PenOverStampedEnvelope => '🖆',
            MiscellaneousSymbolsandPictographs::LinkedPaperclips => '🖇',
            MiscellaneousSymbolsandPictographs::BlackPushpin => '🖈',
            MiscellaneousSymbolsandPictographs::LowerLeftPencil => '🖉',
            MiscellaneousSymbolsandPictographs::LowerLeftBallpointPen => '🖊',
            MiscellaneousSymbolsandPictographs::LowerLeftFountainPen => '🖋',
            MiscellaneousSymbolsandPictographs::LowerLeftPaintbrush => '🖌',
            MiscellaneousSymbolsandPictographs::LowerLeftCrayon => '🖍',
            MiscellaneousSymbolsandPictographs::LeftWritingHand => '🖎',
            MiscellaneousSymbolsandPictographs::TurnedOkHandSign => '🖏',
            MiscellaneousSymbolsandPictographs::RaisedHandWithFingersSplayed => '🖐',
            MiscellaneousSymbolsandPictographs::ReversedRaisedHandWithFingersSplayed => '🖑',
            MiscellaneousSymbolsandPictographs::ReversedThumbsUpSign => '🖒',
            MiscellaneousSymbolsandPictographs::ReversedThumbsDownSign => '🖓',
            MiscellaneousSymbolsandPictographs::ReversedVictoryHand => '🖔',
            MiscellaneousSymbolsandPictographs::ReversedHandWithMiddleFingerExtended => '🖕',
            MiscellaneousSymbolsandPictographs::RaisedHandWithPartBetweenMiddleAndRingFingers => '🖖',
            MiscellaneousSymbolsandPictographs::WhiteDownPointingLeftHandIndex => '🖗',
            MiscellaneousSymbolsandPictographs::SidewaysWhiteLeftPointingIndex => '🖘',
            MiscellaneousSymbolsandPictographs::SidewaysWhiteRightPointingIndex => '🖙',
            MiscellaneousSymbolsandPictographs::SidewaysBlackLeftPointingIndex => '🖚',
            MiscellaneousSymbolsandPictographs::SidewaysBlackRightPointingIndex => '🖛',
            MiscellaneousSymbolsandPictographs::BlackLeftPointingBackhandIndex => '🖜',
            MiscellaneousSymbolsandPictographs::BlackRightPointingBackhandIndex => '🖝',
            MiscellaneousSymbolsandPictographs::SidewaysWhiteUpPointingIndex => '🖞',
            MiscellaneousSymbolsandPictographs::SidewaysWhiteDownPointingIndex => '🖟',
            MiscellaneousSymbolsandPictographs::SidewaysBlackUpPointingIndex => '🖠',
            MiscellaneousSymbolsandPictographs::SidewaysBlackDownPointingIndex => '🖡',
            MiscellaneousSymbolsandPictographs::BlackUpPointingBackhandIndex => '🖢',
            MiscellaneousSymbolsandPictographs::BlackDownPointingBackhandIndex => '🖣',
            MiscellaneousSymbolsandPictographs::BlackHeart => '🖤',
            MiscellaneousSymbolsandPictographs::DesktopComputer => '🖥',
            MiscellaneousSymbolsandPictographs::KeyboardAndMouse => '🖦',
            MiscellaneousSymbolsandPictographs::ThreeNetworkedComputers => '🖧',
            MiscellaneousSymbolsandPictographs::Printer => '🖨',
            MiscellaneousSymbolsandPictographs::PocketCalculator => '🖩',
            MiscellaneousSymbolsandPictographs::BlackHardShellFloppyDisk => '🖪',
            MiscellaneousSymbolsandPictographs::WhiteHardShellFloppyDisk => '🖫',
            MiscellaneousSymbolsandPictographs::SoftShellFloppyDisk => '🖬',
            MiscellaneousSymbolsandPictographs::TapeCartridge => '🖭',
            MiscellaneousSymbolsandPictographs::WiredKeyboard => '🖮',
            MiscellaneousSymbolsandPictographs::OneButtonMouse => '🖯',
            MiscellaneousSymbolsandPictographs::TwoButtonMouse => '🖰',
            MiscellaneousSymbolsandPictographs::ThreeButtonMouse => '🖱',
            MiscellaneousSymbolsandPictographs::Trackball => '🖲',
            MiscellaneousSymbolsandPictographs::OldPersonalComputer => '🖳',
            MiscellaneousSymbolsandPictographs::HardDisk => '🖴',
            MiscellaneousSymbolsandPictographs::Screen => '🖵',
            MiscellaneousSymbolsandPictographs::PrinterIcon => '🖶',
            MiscellaneousSymbolsandPictographs::FaxIcon => '🖷',
            MiscellaneousSymbolsandPictographs::OpticalDiscIcon => '🖸',
            MiscellaneousSymbolsandPictographs::DocumentWithText => '🖹',
            MiscellaneousSymbolsandPictographs::DocumentWithTextAndPicture => '🖺',
            MiscellaneousSymbolsandPictographs::DocumentWithPicture => '🖻',
            MiscellaneousSymbolsandPictographs::FrameWithPicture => '🖼',
            MiscellaneousSymbolsandPictographs::FrameWithTiles => '🖽',
            MiscellaneousSymbolsandPictographs::FrameWithAnX => '🖾',
            MiscellaneousSymbolsandPictographs::BlackFolder => '🖿',
            MiscellaneousSymbolsandPictographs::Folder => '🗀',
            MiscellaneousSymbolsandPictographs::OpenFolder => '🗁',
            MiscellaneousSymbolsandPictographs::CardIndexDividers => '🗂',
            MiscellaneousSymbolsandPictographs::CardFileBox => '🗃',
            MiscellaneousSymbolsandPictographs::FileCabinet => '🗄',
            MiscellaneousSymbolsandPictographs::EmptyNote => '🗅',
            MiscellaneousSymbolsandPictographs::EmptyNotePage => '🗆',
            MiscellaneousSymbolsandPictographs::EmptyNotePad => '🗇',
            MiscellaneousSymbolsandPictographs::Note => '🗈',
            MiscellaneousSymbolsandPictographs::NotePage => '🗉',
            MiscellaneousSymbolsandPictographs::NotePad => '🗊',
            MiscellaneousSymbolsandPictographs::EmptyDocument => '🗋',
            MiscellaneousSymbolsandPictographs::EmptyPage => '🗌',
            MiscellaneousSymbolsandPictographs::EmptyPages => '🗍',
            MiscellaneousSymbolsandPictographs::Document => '🗎',
            MiscellaneousSymbolsandPictographs::Page => '🗏',
            MiscellaneousSymbolsandPictographs::Pages => '🗐',
            MiscellaneousSymbolsandPictographs::Wastebasket => '🗑',
            MiscellaneousSymbolsandPictographs::SpiralNotePad => '🗒',
            MiscellaneousSymbolsandPictographs::SpiralCalendarPad => '🗓',
            MiscellaneousSymbolsandPictographs::DesktopWindow => '🗔',
            MiscellaneousSymbolsandPictographs::Minimize => '🗕',
            MiscellaneousSymbolsandPictographs::Maximize => '🗖',
            MiscellaneousSymbolsandPictographs::Overlap => '🗗',
            MiscellaneousSymbolsandPictographs::ClockwiseRightAndLeftSemicircleArrows => '🗘',
            MiscellaneousSymbolsandPictographs::CancellationX => '🗙',
            MiscellaneousSymbolsandPictographs::IncreaseFontSizeSymbol => '🗚',
            MiscellaneousSymbolsandPictographs::DecreaseFontSizeSymbol => '🗛',
            MiscellaneousSymbolsandPictographs::Compression => '🗜',
            MiscellaneousSymbolsandPictographs::OldKey => '🗝',
            MiscellaneousSymbolsandPictographs::RolledDashUpNewspaper => '🗞',
            MiscellaneousSymbolsandPictographs::PageWithCircledText => '🗟',
            MiscellaneousSymbolsandPictographs::StockChart => '🗠',
            MiscellaneousSymbolsandPictographs::DaggerKnife => '🗡',
            MiscellaneousSymbolsandPictographs::Lips => '🗢',
            MiscellaneousSymbolsandPictographs::SpeakingHeadInSilhouette => '🗣',
            MiscellaneousSymbolsandPictographs::ThreeRaysAbove => '🗤',
            MiscellaneousSymbolsandPictographs::ThreeRaysBelow => '🗥',
            MiscellaneousSymbolsandPictographs::ThreeRaysLeft => '🗦',
            MiscellaneousSymbolsandPictographs::ThreeRaysRight => '🗧',
            MiscellaneousSymbolsandPictographs::LeftSpeechBubble => '🗨',
            MiscellaneousSymbolsandPictographs::RightSpeechBubble => '🗩',
            MiscellaneousSymbolsandPictographs::TwoSpeechBubbles => '🗪',
            MiscellaneousSymbolsandPictographs::ThreeSpeechBubbles => '🗫',
            MiscellaneousSymbolsandPictographs::LeftThoughtBubble => '🗬',
            MiscellaneousSymbolsandPictographs::RightThoughtBubble => '🗭',
            MiscellaneousSymbolsandPictographs::LeftAngerBubble => '🗮',
            MiscellaneousSymbolsandPictographs::RightAngerBubble => '🗯',
            MiscellaneousSymbolsandPictographs::MoodBubble => '🗰',
            MiscellaneousSymbolsandPictographs::LightningMoodBubble => '🗱',
            MiscellaneousSymbolsandPictographs::LightningMood => '🗲',
            MiscellaneousSymbolsandPictographs::BallotBoxWithBallot => '🗳',
            MiscellaneousSymbolsandPictographs::BallotScriptX => '🗴',
            MiscellaneousSymbolsandPictographs::BallotBoxWithScriptX => '🗵',
            MiscellaneousSymbolsandPictographs::BallotBoldScriptX => '🗶',
            MiscellaneousSymbolsandPictographs::BallotBoxWithBoldScriptX => '🗷',
            MiscellaneousSymbolsandPictographs::LightCheckMark => '🗸',
            MiscellaneousSymbolsandPictographs::BallotBoxWithBoldCheck => '🗹',
            MiscellaneousSymbolsandPictographs::WorldMap => '🗺',
            MiscellaneousSymbolsandPictographs::MountFuji => '🗻',
            MiscellaneousSymbolsandPictographs::TokyoTower => '🗼',
            MiscellaneousSymbolsandPictographs::StatueOfLiberty => '🗽',
            MiscellaneousSymbolsandPictographs::SilhouetteOfJapan => '🗾',
        }
    }
}

impl std::convert::TryFrom<char> for MiscellaneousSymbolsandPictographs {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '🌀' => Ok(MiscellaneousSymbolsandPictographs::Cyclone),
            '🌁' => Ok(MiscellaneousSymbolsandPictographs::Foggy),
            '🌂' => Ok(MiscellaneousSymbolsandPictographs::ClosedUmbrella),
            '🌃' => Ok(MiscellaneousSymbolsandPictographs::NightWithStars),
            '🌄' => Ok(MiscellaneousSymbolsandPictographs::SunriseOverMountains),
            '🌅' => Ok(MiscellaneousSymbolsandPictographs::Sunrise),
            '🌆' => Ok(MiscellaneousSymbolsandPictographs::CityscapeAtDusk),
            '🌇' => Ok(MiscellaneousSymbolsandPictographs::SunsetOverBuildings),
            '🌈' => Ok(MiscellaneousSymbolsandPictographs::Rainbow),
            '🌉' => Ok(MiscellaneousSymbolsandPictographs::BridgeAtNight),
            '🌊' => Ok(MiscellaneousSymbolsandPictographs::WaterWave),
            '🌋' => Ok(MiscellaneousSymbolsandPictographs::Volcano),
            '🌌' => Ok(MiscellaneousSymbolsandPictographs::MilkyWay),
            '🌍' => Ok(MiscellaneousSymbolsandPictographs::EarthGlobeEuropeDashAfrica),
            '🌎' => Ok(MiscellaneousSymbolsandPictographs::EarthGlobeAmericas),
            '🌏' => Ok(MiscellaneousSymbolsandPictographs::EarthGlobeAsiaDashAustralia),
            '🌐' => Ok(MiscellaneousSymbolsandPictographs::GlobeWithMeridians),
            '🌑' => Ok(MiscellaneousSymbolsandPictographs::NewMoonSymbol),
            '🌒' => Ok(MiscellaneousSymbolsandPictographs::WaxingCrescentMoonSymbol),
            '🌓' => Ok(MiscellaneousSymbolsandPictographs::FirstQuarterMoonSymbol),
            '🌔' => Ok(MiscellaneousSymbolsandPictographs::WaxingGibbousMoonSymbol),
            '🌕' => Ok(MiscellaneousSymbolsandPictographs::FullMoonSymbol),
            '🌖' => Ok(MiscellaneousSymbolsandPictographs::WaningGibbousMoonSymbol),
            '🌗' => Ok(MiscellaneousSymbolsandPictographs::LastQuarterMoonSymbol),
            '🌘' => Ok(MiscellaneousSymbolsandPictographs::WaningCrescentMoonSymbol),
            '🌙' => Ok(MiscellaneousSymbolsandPictographs::CrescentMoon),
            '🌚' => Ok(MiscellaneousSymbolsandPictographs::NewMoonWithFace),
            '🌛' => Ok(MiscellaneousSymbolsandPictographs::FirstQuarterMoonWithFace),
            '🌜' => Ok(MiscellaneousSymbolsandPictographs::LastQuarterMoonWithFace),
            '🌝' => Ok(MiscellaneousSymbolsandPictographs::FullMoonWithFace),
            '🌞' => Ok(MiscellaneousSymbolsandPictographs::SunWithFace),
            '🌟' => Ok(MiscellaneousSymbolsandPictographs::GlowingStar),
            '🌠' => Ok(MiscellaneousSymbolsandPictographs::ShootingStar),
            '🌡' => Ok(MiscellaneousSymbolsandPictographs::Thermometer),
            '🌢' => Ok(MiscellaneousSymbolsandPictographs::BlackDroplet),
            '🌣' => Ok(MiscellaneousSymbolsandPictographs::WhiteSun),
            '🌤' => Ok(MiscellaneousSymbolsandPictographs::WhiteSunWithSmallCloud),
            '🌥' => Ok(MiscellaneousSymbolsandPictographs::WhiteSunBehindCloud),
            '🌦' => Ok(MiscellaneousSymbolsandPictographs::WhiteSunBehindCloudWithRain),
            '🌧' => Ok(MiscellaneousSymbolsandPictographs::CloudWithRain),
            '🌨' => Ok(MiscellaneousSymbolsandPictographs::CloudWithSnow),
            '🌩' => Ok(MiscellaneousSymbolsandPictographs::CloudWithLightning),
            '🌪' => Ok(MiscellaneousSymbolsandPictographs::CloudWithTornado),
            '🌫' => Ok(MiscellaneousSymbolsandPictographs::Fog),
            '🌬' => Ok(MiscellaneousSymbolsandPictographs::WindBlowingFace),
            '🌭' => Ok(MiscellaneousSymbolsandPictographs::HotDog),
            '🌮' => Ok(MiscellaneousSymbolsandPictographs::Taco),
            '🌯' => Ok(MiscellaneousSymbolsandPictographs::Burrito),
            '🌰' => Ok(MiscellaneousSymbolsandPictographs::Chestnut),
            '🌱' => Ok(MiscellaneousSymbolsandPictographs::Seedling),
            '🌲' => Ok(MiscellaneousSymbolsandPictographs::EvergreenTree),
            '🌳' => Ok(MiscellaneousSymbolsandPictographs::DeciduousTree),
            '🌴' => Ok(MiscellaneousSymbolsandPictographs::PalmTree),
            '🌵' => Ok(MiscellaneousSymbolsandPictographs::Cactus),
            '🌶' => Ok(MiscellaneousSymbolsandPictographs::HotPepper),
            '🌷' => Ok(MiscellaneousSymbolsandPictographs::Tulip),
            '🌸' => Ok(MiscellaneousSymbolsandPictographs::CherryBlossom),
            '🌹' => Ok(MiscellaneousSymbolsandPictographs::Rose),
            '🌺' => Ok(MiscellaneousSymbolsandPictographs::Hibiscus),
            '🌻' => Ok(MiscellaneousSymbolsandPictographs::Sunflower),
            '🌼' => Ok(MiscellaneousSymbolsandPictographs::Blossom),
            '🌽' => Ok(MiscellaneousSymbolsandPictographs::EarOfMaize),
            '🌾' => Ok(MiscellaneousSymbolsandPictographs::EarOfRice),
            '🌿' => Ok(MiscellaneousSymbolsandPictographs::Herb),
            '🍀' => Ok(MiscellaneousSymbolsandPictographs::FourLeafClover),
            '🍁' => Ok(MiscellaneousSymbolsandPictographs::MapleLeaf),
            '🍂' => Ok(MiscellaneousSymbolsandPictographs::FallenLeaf),
            '🍃' => Ok(MiscellaneousSymbolsandPictographs::LeafFlutteringInWind),
            '🍄' => Ok(MiscellaneousSymbolsandPictographs::Mushroom),
            '🍅' => Ok(MiscellaneousSymbolsandPictographs::Tomato),
            '🍆' => Ok(MiscellaneousSymbolsandPictographs::Aubergine),
            '🍇' => Ok(MiscellaneousSymbolsandPictographs::Grapes),
            '🍈' => Ok(MiscellaneousSymbolsandPictographs::Melon),
            '🍉' => Ok(MiscellaneousSymbolsandPictographs::Watermelon),
            '🍊' => Ok(MiscellaneousSymbolsandPictographs::Tangerine),
            '🍋' => Ok(MiscellaneousSymbolsandPictographs::Lemon),
            '🍌' => Ok(MiscellaneousSymbolsandPictographs::Banana),
            '🍍' => Ok(MiscellaneousSymbolsandPictographs::Pineapple),
            '🍎' => Ok(MiscellaneousSymbolsandPictographs::RedApple),
            '🍏' => Ok(MiscellaneousSymbolsandPictographs::GreenApple),
            '🍐' => Ok(MiscellaneousSymbolsandPictographs::Pear),
            '🍑' => Ok(MiscellaneousSymbolsandPictographs::Peach),
            '🍒' => Ok(MiscellaneousSymbolsandPictographs::Cherries),
            '🍓' => Ok(MiscellaneousSymbolsandPictographs::Strawberry),
            '🍔' => Ok(MiscellaneousSymbolsandPictographs::Hamburger),
            '🍕' => Ok(MiscellaneousSymbolsandPictographs::SliceOfPizza),
            '🍖' => Ok(MiscellaneousSymbolsandPictographs::MeatOnBone),
            '🍗' => Ok(MiscellaneousSymbolsandPictographs::PoultryLeg),
            '🍘' => Ok(MiscellaneousSymbolsandPictographs::RiceCracker),
            '🍙' => Ok(MiscellaneousSymbolsandPictographs::RiceBall),
            '🍚' => Ok(MiscellaneousSymbolsandPictographs::CookedRice),
            '🍛' => Ok(MiscellaneousSymbolsandPictographs::CurryAndRice),
            '🍜' => Ok(MiscellaneousSymbolsandPictographs::SteamingBowl),
            '🍝' => Ok(MiscellaneousSymbolsandPictographs::Spaghetti),
            '🍞' => Ok(MiscellaneousSymbolsandPictographs::Bread),
            '🍟' => Ok(MiscellaneousSymbolsandPictographs::FrenchFries),
            '🍠' => Ok(MiscellaneousSymbolsandPictographs::RoastedSweetPotato),
            '🍡' => Ok(MiscellaneousSymbolsandPictographs::Dango),
            '🍢' => Ok(MiscellaneousSymbolsandPictographs::Oden),
            '🍣' => Ok(MiscellaneousSymbolsandPictographs::Sushi),
            '🍤' => Ok(MiscellaneousSymbolsandPictographs::FriedShrimp),
            '🍥' => Ok(MiscellaneousSymbolsandPictographs::FishCakeWithSwirlDesign),
            '🍦' => Ok(MiscellaneousSymbolsandPictographs::SoftIceCream),
            '🍧' => Ok(MiscellaneousSymbolsandPictographs::ShavedIce),
            '🍨' => Ok(MiscellaneousSymbolsandPictographs::IceCream),
            '🍩' => Ok(MiscellaneousSymbolsandPictographs::Doughnut),
            '🍪' => Ok(MiscellaneousSymbolsandPictographs::Cookie),
            '🍫' => Ok(MiscellaneousSymbolsandPictographs::ChocolateBar),
            '🍬' => Ok(MiscellaneousSymbolsandPictographs::Candy),
            '🍭' => Ok(MiscellaneousSymbolsandPictographs::Lollipop),
            '🍮' => Ok(MiscellaneousSymbolsandPictographs::Custard),
            '🍯' => Ok(MiscellaneousSymbolsandPictographs::HoneyPot),
            '🍰' => Ok(MiscellaneousSymbolsandPictographs::Shortcake),
            '🍱' => Ok(MiscellaneousSymbolsandPictographs::BentoBox),
            '🍲' => Ok(MiscellaneousSymbolsandPictographs::PotOfFood),
            '🍳' => Ok(MiscellaneousSymbolsandPictographs::Cooking),
            '🍴' => Ok(MiscellaneousSymbolsandPictographs::ForkAndKnife),
            '🍵' => Ok(MiscellaneousSymbolsandPictographs::TeacupWithoutHandle),
            '🍶' => Ok(MiscellaneousSymbolsandPictographs::SakeBottleAndCup),
            '🍷' => Ok(MiscellaneousSymbolsandPictographs::WineGlass),
            '🍸' => Ok(MiscellaneousSymbolsandPictographs::CocktailGlass),
            '🍹' => Ok(MiscellaneousSymbolsandPictographs::TropicalDrink),
            '🍺' => Ok(MiscellaneousSymbolsandPictographs::BeerMug),
            '🍻' => Ok(MiscellaneousSymbolsandPictographs::ClinkingBeerMugs),
            '🍼' => Ok(MiscellaneousSymbolsandPictographs::BabyBottle),
            '🍽' => Ok(MiscellaneousSymbolsandPictographs::ForkAndKnifeWithPlate),
            '🍾' => Ok(MiscellaneousSymbolsandPictographs::BottleWithPoppingCork),
            '🍿' => Ok(MiscellaneousSymbolsandPictographs::Popcorn),
            '🎀' => Ok(MiscellaneousSymbolsandPictographs::Ribbon),
            '🎁' => Ok(MiscellaneousSymbolsandPictographs::WrappedPresent),
            '🎂' => Ok(MiscellaneousSymbolsandPictographs::BirthdayCake),
            '🎃' => Ok(MiscellaneousSymbolsandPictographs::JackDashODashLantern),
            '🎄' => Ok(MiscellaneousSymbolsandPictographs::ChristmasTree),
            '🎅' => Ok(MiscellaneousSymbolsandPictographs::FatherChristmas),
            '🎆' => Ok(MiscellaneousSymbolsandPictographs::Fireworks),
            '🎇' => Ok(MiscellaneousSymbolsandPictographs::FireworkSparkler),
            '🎈' => Ok(MiscellaneousSymbolsandPictographs::Balloon),
            '🎉' => Ok(MiscellaneousSymbolsandPictographs::PartyPopper),
            '🎊' => Ok(MiscellaneousSymbolsandPictographs::ConfettiBall),
            '🎋' => Ok(MiscellaneousSymbolsandPictographs::TanabataTree),
            '🎌' => Ok(MiscellaneousSymbolsandPictographs::CrossedFlags),
            '🎍' => Ok(MiscellaneousSymbolsandPictographs::PineDecoration),
            '🎎' => Ok(MiscellaneousSymbolsandPictographs::JapaneseDolls),
            '🎏' => Ok(MiscellaneousSymbolsandPictographs::CarpStreamer),
            '🎐' => Ok(MiscellaneousSymbolsandPictographs::WindChime),
            '🎑' => Ok(MiscellaneousSymbolsandPictographs::MoonViewingCeremony),
            '🎒' => Ok(MiscellaneousSymbolsandPictographs::SchoolSatchel),
            '🎓' => Ok(MiscellaneousSymbolsandPictographs::GraduationCap),
            '🎔' => Ok(MiscellaneousSymbolsandPictographs::HeartWithTipOnTheLeft),
            '🎕' => Ok(MiscellaneousSymbolsandPictographs::BouquetOfFlowers),
            '🎖' => Ok(MiscellaneousSymbolsandPictographs::MilitaryMedal),
            '🎗' => Ok(MiscellaneousSymbolsandPictographs::ReminderRibbon),
            '🎘' => Ok(MiscellaneousSymbolsandPictographs::MusicalKeyboardWithJacks),
            '🎙' => Ok(MiscellaneousSymbolsandPictographs::StudioMicrophone),
            '🎚' => Ok(MiscellaneousSymbolsandPictographs::LevelSlider),
            '🎛' => Ok(MiscellaneousSymbolsandPictographs::ControlKnobs),
            '🎜' => Ok(MiscellaneousSymbolsandPictographs::BeamedAscendingMusicalNotes),
            '🎝' => Ok(MiscellaneousSymbolsandPictographs::BeamedDescendingMusicalNotes),
            '🎞' => Ok(MiscellaneousSymbolsandPictographs::FilmFrames),
            '🎟' => Ok(MiscellaneousSymbolsandPictographs::AdmissionTickets),
            '🎠' => Ok(MiscellaneousSymbolsandPictographs::CarouselHorse),
            '🎡' => Ok(MiscellaneousSymbolsandPictographs::FerrisWheel),
            '🎢' => Ok(MiscellaneousSymbolsandPictographs::RollerCoaster),
            '🎣' => Ok(MiscellaneousSymbolsandPictographs::FishingPoleAndFish),
            '🎤' => Ok(MiscellaneousSymbolsandPictographs::Microphone),
            '🎥' => Ok(MiscellaneousSymbolsandPictographs::MovieCamera),
            '🎦' => Ok(MiscellaneousSymbolsandPictographs::Cinema),
            '🎧' => Ok(MiscellaneousSymbolsandPictographs::Headphone),
            '🎨' => Ok(MiscellaneousSymbolsandPictographs::ArtistPalette),
            '🎩' => Ok(MiscellaneousSymbolsandPictographs::TopHat),
            '🎪' => Ok(MiscellaneousSymbolsandPictographs::CircusTent),
            '🎫' => Ok(MiscellaneousSymbolsandPictographs::Ticket),
            '🎬' => Ok(MiscellaneousSymbolsandPictographs::ClapperBoard),
            '🎭' => Ok(MiscellaneousSymbolsandPictographs::PerformingArts),
            '🎮' => Ok(MiscellaneousSymbolsandPictographs::VideoGame),
            '🎯' => Ok(MiscellaneousSymbolsandPictographs::DirectHit),
            '🎰' => Ok(MiscellaneousSymbolsandPictographs::SlotMachine),
            '🎱' => Ok(MiscellaneousSymbolsandPictographs::Billiards),
            '🎲' => Ok(MiscellaneousSymbolsandPictographs::GameDie),
            '🎳' => Ok(MiscellaneousSymbolsandPictographs::Bowling),
            '🎴' => Ok(MiscellaneousSymbolsandPictographs::FlowerPlayingCards),
            '🎵' => Ok(MiscellaneousSymbolsandPictographs::MusicalNote),
            '🎶' => Ok(MiscellaneousSymbolsandPictographs::MultipleMusicalNotes),
            '🎷' => Ok(MiscellaneousSymbolsandPictographs::Saxophone),
            '🎸' => Ok(MiscellaneousSymbolsandPictographs::Guitar),
            '🎹' => Ok(MiscellaneousSymbolsandPictographs::MusicalKeyboard),
            '🎺' => Ok(MiscellaneousSymbolsandPictographs::Trumpet),
            '🎻' => Ok(MiscellaneousSymbolsandPictographs::Violin),
            '🎼' => Ok(MiscellaneousSymbolsandPictographs::MusicalScore),
            '🎽' => Ok(MiscellaneousSymbolsandPictographs::RunningShirtWithSash),
            '🎾' => Ok(MiscellaneousSymbolsandPictographs::TennisRacquetAndBall),
            '🎿' => Ok(MiscellaneousSymbolsandPictographs::SkiAndSkiBoot),
            '🏀' => Ok(MiscellaneousSymbolsandPictographs::BasketballAndHoop),
            '🏁' => Ok(MiscellaneousSymbolsandPictographs::ChequeredFlag),
            '🏂' => Ok(MiscellaneousSymbolsandPictographs::Snowboarder),
            '🏃' => Ok(MiscellaneousSymbolsandPictographs::Runner),
            '🏄' => Ok(MiscellaneousSymbolsandPictographs::Surfer),
            '🏅' => Ok(MiscellaneousSymbolsandPictographs::SportsMedal),
            '🏆' => Ok(MiscellaneousSymbolsandPictographs::Trophy),
            '🏇' => Ok(MiscellaneousSymbolsandPictographs::HorseRacing),
            '🏈' => Ok(MiscellaneousSymbolsandPictographs::AmericanFootball),
            '🏉' => Ok(MiscellaneousSymbolsandPictographs::RugbyFootball),
            '🏊' => Ok(MiscellaneousSymbolsandPictographs::Swimmer),
            '🏋' => Ok(MiscellaneousSymbolsandPictographs::WeightLifter),
            '🏌' => Ok(MiscellaneousSymbolsandPictographs::Golfer),
            '🏍' => Ok(MiscellaneousSymbolsandPictographs::RacingMotorcycle),
            '🏎' => Ok(MiscellaneousSymbolsandPictographs::RacingCar),
            '🏏' => Ok(MiscellaneousSymbolsandPictographs::CricketBatAndBall),
            '🏐' => Ok(MiscellaneousSymbolsandPictographs::Volleyball),
            '🏑' => Ok(MiscellaneousSymbolsandPictographs::FieldHockeyStickAndBall),
            '🏒' => Ok(MiscellaneousSymbolsandPictographs::IceHockeyStickAndPuck),
            '🏓' => Ok(MiscellaneousSymbolsandPictographs::TableTennisPaddleAndBall),
            '🏔' => Ok(MiscellaneousSymbolsandPictographs::SnowCappedMountain),
            '🏕' => Ok(MiscellaneousSymbolsandPictographs::Camping),
            '🏖' => Ok(MiscellaneousSymbolsandPictographs::BeachWithUmbrella),
            '🏗' => Ok(MiscellaneousSymbolsandPictographs::BuildingConstruction),
            '🏘' => Ok(MiscellaneousSymbolsandPictographs::HouseBuildings),
            '🏙' => Ok(MiscellaneousSymbolsandPictographs::Cityscape),
            '🏚' => Ok(MiscellaneousSymbolsandPictographs::DerelictHouseBuilding),
            '🏛' => Ok(MiscellaneousSymbolsandPictographs::ClassicalBuilding),
            '🏜' => Ok(MiscellaneousSymbolsandPictographs::Desert),
            '🏝' => Ok(MiscellaneousSymbolsandPictographs::DesertIsland),
            '🏞' => Ok(MiscellaneousSymbolsandPictographs::NationalPark),
            '🏟' => Ok(MiscellaneousSymbolsandPictographs::Stadium),
            '🏠' => Ok(MiscellaneousSymbolsandPictographs::HouseBuilding),
            '🏡' => Ok(MiscellaneousSymbolsandPictographs::HouseWithGarden),
            '🏢' => Ok(MiscellaneousSymbolsandPictographs::OfficeBuilding),
            '🏣' => Ok(MiscellaneousSymbolsandPictographs::JapanesePostOffice),
            '🏤' => Ok(MiscellaneousSymbolsandPictographs::EuropeanPostOffice),
            '🏥' => Ok(MiscellaneousSymbolsandPictographs::Hospital),
            '🏦' => Ok(MiscellaneousSymbolsandPictographs::Bank),
            '🏧' => Ok(MiscellaneousSymbolsandPictographs::AutomatedTellerMachine),
            '🏨' => Ok(MiscellaneousSymbolsandPictographs::Hotel),
            '🏩' => Ok(MiscellaneousSymbolsandPictographs::LoveHotel),
            '🏪' => Ok(MiscellaneousSymbolsandPictographs::ConvenienceStore),
            '🏫' => Ok(MiscellaneousSymbolsandPictographs::School),
            '🏬' => Ok(MiscellaneousSymbolsandPictographs::DepartmentStore),
            '🏭' => Ok(MiscellaneousSymbolsandPictographs::Factory),
            '🏮' => Ok(MiscellaneousSymbolsandPictographs::IzakayaLantern),
            '🏯' => Ok(MiscellaneousSymbolsandPictographs::JapaneseCastle),
            '🏰' => Ok(MiscellaneousSymbolsandPictographs::EuropeanCastle),
            '🏱' => Ok(MiscellaneousSymbolsandPictographs::WhitePennant),
            '🏲' => Ok(MiscellaneousSymbolsandPictographs::BlackPennant),
            '🏳' => Ok(MiscellaneousSymbolsandPictographs::WavingWhiteFlag),
            '🏴' => Ok(MiscellaneousSymbolsandPictographs::WavingBlackFlag),
            '🏵' => Ok(MiscellaneousSymbolsandPictographs::Rosette),
            '🏶' => Ok(MiscellaneousSymbolsandPictographs::BlackRosette),
            '🏷' => Ok(MiscellaneousSymbolsandPictographs::Label),
            '🏸' => Ok(MiscellaneousSymbolsandPictographs::BadmintonRacquetAndShuttlecock),
            '🏹' => Ok(MiscellaneousSymbolsandPictographs::BowAndArrow),
            '🏺' => Ok(MiscellaneousSymbolsandPictographs::Amphora),
            '🏻' => Ok(MiscellaneousSymbolsandPictographs::EmojiModifierFitzpatrickTypeDash1Dash2),
            '🏼' => Ok(MiscellaneousSymbolsandPictographs::EmojiModifierFitzpatrickTypeDash3),
            '🏽' => Ok(MiscellaneousSymbolsandPictographs::EmojiModifierFitzpatrickTypeDash4),
            '🏾' => Ok(MiscellaneousSymbolsandPictographs::EmojiModifierFitzpatrickTypeDash5),
            '🏿' => Ok(MiscellaneousSymbolsandPictographs::EmojiModifierFitzpatrickTypeDash6),
            '🐀' => Ok(MiscellaneousSymbolsandPictographs::Rat),
            '🐁' => Ok(MiscellaneousSymbolsandPictographs::Mouse),
            '🐂' => Ok(MiscellaneousSymbolsandPictographs::Ox),
            '🐃' => Ok(MiscellaneousSymbolsandPictographs::WaterBuffalo),
            '🐄' => Ok(MiscellaneousSymbolsandPictographs::Cow),
            '🐅' => Ok(MiscellaneousSymbolsandPictographs::Tiger),
            '🐆' => Ok(MiscellaneousSymbolsandPictographs::Leopard),
            '🐇' => Ok(MiscellaneousSymbolsandPictographs::Rabbit),
            '🐈' => Ok(MiscellaneousSymbolsandPictographs::Cat),
            '🐉' => Ok(MiscellaneousSymbolsandPictographs::Dragon),
            '🐊' => Ok(MiscellaneousSymbolsandPictographs::Crocodile),
            '🐋' => Ok(MiscellaneousSymbolsandPictographs::Whale),
            '🐌' => Ok(MiscellaneousSymbolsandPictographs::Snail),
            '🐍' => Ok(MiscellaneousSymbolsandPictographs::Snake),
            '🐎' => Ok(MiscellaneousSymbolsandPictographs::Horse),
            '🐏' => Ok(MiscellaneousSymbolsandPictographs::Ram),
            '🐐' => Ok(MiscellaneousSymbolsandPictographs::Goat),
            '🐑' => Ok(MiscellaneousSymbolsandPictographs::Sheep),
            '🐒' => Ok(MiscellaneousSymbolsandPictographs::Monkey),
            '🐓' => Ok(MiscellaneousSymbolsandPictographs::Rooster),
            '🐔' => Ok(MiscellaneousSymbolsandPictographs::Chicken),
            '🐕' => Ok(MiscellaneousSymbolsandPictographs::Dog),
            '🐖' => Ok(MiscellaneousSymbolsandPictographs::Pig),
            '🐗' => Ok(MiscellaneousSymbolsandPictographs::Boar),
            '🐘' => Ok(MiscellaneousSymbolsandPictographs::Elephant),
            '🐙' => Ok(MiscellaneousSymbolsandPictographs::Octopus),
            '🐚' => Ok(MiscellaneousSymbolsandPictographs::SpiralShell),
            '🐛' => Ok(MiscellaneousSymbolsandPictographs::Bug),
            '🐜' => Ok(MiscellaneousSymbolsandPictographs::Ant),
            '🐝' => Ok(MiscellaneousSymbolsandPictographs::Honeybee),
            '🐞' => Ok(MiscellaneousSymbolsandPictographs::LadyBeetle),
            '🐟' => Ok(MiscellaneousSymbolsandPictographs::Fish),
            '🐠' => Ok(MiscellaneousSymbolsandPictographs::TropicalFish),
            '🐡' => Ok(MiscellaneousSymbolsandPictographs::Blowfish),
            '🐢' => Ok(MiscellaneousSymbolsandPictographs::Turtle),
            '🐣' => Ok(MiscellaneousSymbolsandPictographs::HatchingChick),
            '🐤' => Ok(MiscellaneousSymbolsandPictographs::BabyChick),
            '🐥' => Ok(MiscellaneousSymbolsandPictographs::FrontDashFacingBabyChick),
            '🐦' => Ok(MiscellaneousSymbolsandPictographs::Bird),
            '🐧' => Ok(MiscellaneousSymbolsandPictographs::Penguin),
            '🐨' => Ok(MiscellaneousSymbolsandPictographs::Koala),
            '🐩' => Ok(MiscellaneousSymbolsandPictographs::Poodle),
            '🐪' => Ok(MiscellaneousSymbolsandPictographs::DromedaryCamel),
            '🐫' => Ok(MiscellaneousSymbolsandPictographs::BactrianCamel),
            '🐬' => Ok(MiscellaneousSymbolsandPictographs::Dolphin),
            '🐭' => Ok(MiscellaneousSymbolsandPictographs::MouseFace),
            '🐮' => Ok(MiscellaneousSymbolsandPictographs::CowFace),
            '🐯' => Ok(MiscellaneousSymbolsandPictographs::TigerFace),
            '🐰' => Ok(MiscellaneousSymbolsandPictographs::RabbitFace),
            '🐱' => Ok(MiscellaneousSymbolsandPictographs::CatFace),
            '🐲' => Ok(MiscellaneousSymbolsandPictographs::DragonFace),
            '🐳' => Ok(MiscellaneousSymbolsandPictographs::SpoutingWhale),
            '🐴' => Ok(MiscellaneousSymbolsandPictographs::HorseFace),
            '🐵' => Ok(MiscellaneousSymbolsandPictographs::MonkeyFace),
            '🐶' => Ok(MiscellaneousSymbolsandPictographs::DogFace),
            '🐷' => Ok(MiscellaneousSymbolsandPictographs::PigFace),
            '🐸' => Ok(MiscellaneousSymbolsandPictographs::FrogFace),
            '🐹' => Ok(MiscellaneousSymbolsandPictographs::HamsterFace),
            '🐺' => Ok(MiscellaneousSymbolsandPictographs::WolfFace),
            '🐻' => Ok(MiscellaneousSymbolsandPictographs::BearFace),
            '🐼' => Ok(MiscellaneousSymbolsandPictographs::PandaFace),
            '🐽' => Ok(MiscellaneousSymbolsandPictographs::PigNose),
            '🐾' => Ok(MiscellaneousSymbolsandPictographs::PawPrints),
            '🐿' => Ok(MiscellaneousSymbolsandPictographs::Chipmunk),
            '👀' => Ok(MiscellaneousSymbolsandPictographs::Eyes),
            '👁' => Ok(MiscellaneousSymbolsandPictographs::Eye),
            '👂' => Ok(MiscellaneousSymbolsandPictographs::Ear),
            '👃' => Ok(MiscellaneousSymbolsandPictographs::Nose),
            '👄' => Ok(MiscellaneousSymbolsandPictographs::Mouth),
            '👅' => Ok(MiscellaneousSymbolsandPictographs::Tongue),
            '👆' => Ok(MiscellaneousSymbolsandPictographs::WhiteUpPointingBackhandIndex),
            '👇' => Ok(MiscellaneousSymbolsandPictographs::WhiteDownPointingBackhandIndex),
            '👈' => Ok(MiscellaneousSymbolsandPictographs::WhiteLeftPointingBackhandIndex),
            '👉' => Ok(MiscellaneousSymbolsandPictographs::WhiteRightPointingBackhandIndex),
            '👊' => Ok(MiscellaneousSymbolsandPictographs::FistedHandSign),
            '👋' => Ok(MiscellaneousSymbolsandPictographs::WavingHandSign),
            '👌' => Ok(MiscellaneousSymbolsandPictographs::OkHandSign),
            '👍' => Ok(MiscellaneousSymbolsandPictographs::ThumbsUpSign),
            '👎' => Ok(MiscellaneousSymbolsandPictographs::ThumbsDownSign),
            '👏' => Ok(MiscellaneousSymbolsandPictographs::ClappingHandsSign),
            '👐' => Ok(MiscellaneousSymbolsandPictographs::OpenHandsSign),
            '👑' => Ok(MiscellaneousSymbolsandPictographs::Crown),
            '👒' => Ok(MiscellaneousSymbolsandPictographs::WomansHat),
            '👓' => Ok(MiscellaneousSymbolsandPictographs::Eyeglasses),
            '👔' => Ok(MiscellaneousSymbolsandPictographs::Necktie),
            '👕' => Ok(MiscellaneousSymbolsandPictographs::TDashShirt),
            '👖' => Ok(MiscellaneousSymbolsandPictographs::Jeans),
            '👗' => Ok(MiscellaneousSymbolsandPictographs::Dress),
            '👘' => Ok(MiscellaneousSymbolsandPictographs::Kimono),
            '👙' => Ok(MiscellaneousSymbolsandPictographs::Bikini),
            '👚' => Ok(MiscellaneousSymbolsandPictographs::WomansClothes),
            '👛' => Ok(MiscellaneousSymbolsandPictographs::Purse),
            '👜' => Ok(MiscellaneousSymbolsandPictographs::Handbag),
            '👝' => Ok(MiscellaneousSymbolsandPictographs::Pouch),
            '👞' => Ok(MiscellaneousSymbolsandPictographs::MansShoe),
            '👟' => Ok(MiscellaneousSymbolsandPictographs::AthleticShoe),
            '👠' => Ok(MiscellaneousSymbolsandPictographs::HighDashHeeledShoe),
            '👡' => Ok(MiscellaneousSymbolsandPictographs::WomansSandal),
            '👢' => Ok(MiscellaneousSymbolsandPictographs::WomansBoots),
            '👣' => Ok(MiscellaneousSymbolsandPictographs::Footprints),
            '👤' => Ok(MiscellaneousSymbolsandPictographs::BustInSilhouette),
            '👥' => Ok(MiscellaneousSymbolsandPictographs::BustsInSilhouette),
            '👦' => Ok(MiscellaneousSymbolsandPictographs::Boy),
            '👧' => Ok(MiscellaneousSymbolsandPictographs::Girl),
            '👨' => Ok(MiscellaneousSymbolsandPictographs::Man),
            '👩' => Ok(MiscellaneousSymbolsandPictographs::Woman),
            '👪' => Ok(MiscellaneousSymbolsandPictographs::Family),
            '👫' => Ok(MiscellaneousSymbolsandPictographs::ManAndWomanHoldingHands),
            '👬' => Ok(MiscellaneousSymbolsandPictographs::TwoMenHoldingHands),
            '👭' => Ok(MiscellaneousSymbolsandPictographs::TwoWomenHoldingHands),
            '👮' => Ok(MiscellaneousSymbolsandPictographs::PoliceOfficer),
            '👯' => Ok(MiscellaneousSymbolsandPictographs::WomanWithBunnyEars),
            '👰' => Ok(MiscellaneousSymbolsandPictographs::BrideWithVeil),
            '👱' => Ok(MiscellaneousSymbolsandPictographs::PersonWithBlondHair),
            '👲' => Ok(MiscellaneousSymbolsandPictographs::ManWithGuaPiMao),
            '👳' => Ok(MiscellaneousSymbolsandPictographs::ManWithTurban),
            '👴' => Ok(MiscellaneousSymbolsandPictographs::OlderMan),
            '👵' => Ok(MiscellaneousSymbolsandPictographs::OlderWoman),
            '👶' => Ok(MiscellaneousSymbolsandPictographs::Baby),
            '👷' => Ok(MiscellaneousSymbolsandPictographs::ConstructionWorker),
            '👸' => Ok(MiscellaneousSymbolsandPictographs::Princess),
            '👹' => Ok(MiscellaneousSymbolsandPictographs::JapaneseOgre),
            '👺' => Ok(MiscellaneousSymbolsandPictographs::JapaneseGoblin),
            '👻' => Ok(MiscellaneousSymbolsandPictographs::Ghost),
            '👼' => Ok(MiscellaneousSymbolsandPictographs::BabyAngel),
            '👽' => Ok(MiscellaneousSymbolsandPictographs::ExtraterrestrialAlien),
            '👾' => Ok(MiscellaneousSymbolsandPictographs::AlienMonster),
            '👿' => Ok(MiscellaneousSymbolsandPictographs::Imp),
            '💀' => Ok(MiscellaneousSymbolsandPictographs::Skull),
            '💁' => Ok(MiscellaneousSymbolsandPictographs::InformationDeskPerson),
            '💂' => Ok(MiscellaneousSymbolsandPictographs::Guardsman),
            '💃' => Ok(MiscellaneousSymbolsandPictographs::Dancer),
            '💄' => Ok(MiscellaneousSymbolsandPictographs::Lipstick),
            '💅' => Ok(MiscellaneousSymbolsandPictographs::NailPolish),
            '💆' => Ok(MiscellaneousSymbolsandPictographs::FaceMassage),
            '💇' => Ok(MiscellaneousSymbolsandPictographs::Haircut),
            '💈' => Ok(MiscellaneousSymbolsandPictographs::BarberPole),
            '💉' => Ok(MiscellaneousSymbolsandPictographs::Syringe),
            '💊' => Ok(MiscellaneousSymbolsandPictographs::Pill),
            '💋' => Ok(MiscellaneousSymbolsandPictographs::KissMark),
            '💌' => Ok(MiscellaneousSymbolsandPictographs::LoveLetter),
            '💍' => Ok(MiscellaneousSymbolsandPictographs::Ring),
            '💎' => Ok(MiscellaneousSymbolsandPictographs::GemStone),
            '💏' => Ok(MiscellaneousSymbolsandPictographs::Kiss),
            '💐' => Ok(MiscellaneousSymbolsandPictographs::Bouquet),
            '💑' => Ok(MiscellaneousSymbolsandPictographs::CoupleWithHeart),
            '💒' => Ok(MiscellaneousSymbolsandPictographs::Wedding),
            '💓' => Ok(MiscellaneousSymbolsandPictographs::BeatingHeart),
            '💔' => Ok(MiscellaneousSymbolsandPictographs::BrokenHeart),
            '💕' => Ok(MiscellaneousSymbolsandPictographs::TwoHearts),
            '💖' => Ok(MiscellaneousSymbolsandPictographs::SparklingHeart),
            '💗' => Ok(MiscellaneousSymbolsandPictographs::GrowingHeart),
            '💘' => Ok(MiscellaneousSymbolsandPictographs::HeartWithArrow),
            '💙' => Ok(MiscellaneousSymbolsandPictographs::BlueHeart),
            '💚' => Ok(MiscellaneousSymbolsandPictographs::GreenHeart),
            '💛' => Ok(MiscellaneousSymbolsandPictographs::YellowHeart),
            '💜' => Ok(MiscellaneousSymbolsandPictographs::PurpleHeart),
            '💝' => Ok(MiscellaneousSymbolsandPictographs::HeartWithRibbon),
            '💞' => Ok(MiscellaneousSymbolsandPictographs::RevolvingHearts),
            '💟' => Ok(MiscellaneousSymbolsandPictographs::HeartDecoration),
            '💠' => Ok(MiscellaneousSymbolsandPictographs::DiamondShapeWithADotInside),
            '💡' => Ok(MiscellaneousSymbolsandPictographs::ElectricLightBulb),
            '💢' => Ok(MiscellaneousSymbolsandPictographs::AngerSymbol),
            '💣' => Ok(MiscellaneousSymbolsandPictographs::Bomb),
            '💤' => Ok(MiscellaneousSymbolsandPictographs::SleepingSymbol),
            '💥' => Ok(MiscellaneousSymbolsandPictographs::CollisionSymbol),
            '💦' => Ok(MiscellaneousSymbolsandPictographs::SplashingSweatSymbol),
            '💧' => Ok(MiscellaneousSymbolsandPictographs::Droplet),
            '💨' => Ok(MiscellaneousSymbolsandPictographs::DashSymbol),
            '💩' => Ok(MiscellaneousSymbolsandPictographs::PileOfPoo),
            '💪' => Ok(MiscellaneousSymbolsandPictographs::FlexedBiceps),
            '💫' => Ok(MiscellaneousSymbolsandPictographs::DizzySymbol),
            '💬' => Ok(MiscellaneousSymbolsandPictographs::SpeechBalloon),
            '💭' => Ok(MiscellaneousSymbolsandPictographs::ThoughtBalloon),
            '💮' => Ok(MiscellaneousSymbolsandPictographs::WhiteFlower),
            '💯' => Ok(MiscellaneousSymbolsandPictographs::HundredPointsSymbol),
            '💰' => Ok(MiscellaneousSymbolsandPictographs::MoneyBag),
            '💱' => Ok(MiscellaneousSymbolsandPictographs::CurrencyExchange),
            '💲' => Ok(MiscellaneousSymbolsandPictographs::HeavyDollarSign),
            '💳' => Ok(MiscellaneousSymbolsandPictographs::CreditCard),
            '💴' => Ok(MiscellaneousSymbolsandPictographs::BanknoteWithYenSign),
            '💵' => Ok(MiscellaneousSymbolsandPictographs::BanknoteWithDollarSign),
            '💶' => Ok(MiscellaneousSymbolsandPictographs::BanknoteWithEuroSign),
            '💷' => Ok(MiscellaneousSymbolsandPictographs::BanknoteWithPoundSign),
            '💸' => Ok(MiscellaneousSymbolsandPictographs::MoneyWithWings),
            '💹' => Ok(MiscellaneousSymbolsandPictographs::ChartWithUpwardsTrendAndYenSign),
            '💺' => Ok(MiscellaneousSymbolsandPictographs::Seat),
            '💻' => Ok(MiscellaneousSymbolsandPictographs::PersonalComputer),
            '💼' => Ok(MiscellaneousSymbolsandPictographs::Briefcase),
            '💽' => Ok(MiscellaneousSymbolsandPictographs::Minidisc),
            '💾' => Ok(MiscellaneousSymbolsandPictographs::FloppyDisk),
            '💿' => Ok(MiscellaneousSymbolsandPictographs::OpticalDisc),
            '📀' => Ok(MiscellaneousSymbolsandPictographs::Dvd),
            '📁' => Ok(MiscellaneousSymbolsandPictographs::FileFolder),
            '📂' => Ok(MiscellaneousSymbolsandPictographs::OpenFileFolder),
            '📃' => Ok(MiscellaneousSymbolsandPictographs::PageWithCurl),
            '📄' => Ok(MiscellaneousSymbolsandPictographs::PageFacingUp),
            '📅' => Ok(MiscellaneousSymbolsandPictographs::Calendar),
            '📆' => Ok(MiscellaneousSymbolsandPictographs::TearDashOffCalendar),
            '📇' => Ok(MiscellaneousSymbolsandPictographs::CardIndex),
            '📈' => Ok(MiscellaneousSymbolsandPictographs::ChartWithUpwardsTrend),
            '📉' => Ok(MiscellaneousSymbolsandPictographs::ChartWithDownwardsTrend),
            '📊' => Ok(MiscellaneousSymbolsandPictographs::BarChart),
            '📋' => Ok(MiscellaneousSymbolsandPictographs::Clipboard),
            '📌' => Ok(MiscellaneousSymbolsandPictographs::Pushpin),
            '📍' => Ok(MiscellaneousSymbolsandPictographs::RoundPushpin),
            '📎' => Ok(MiscellaneousSymbolsandPictographs::Paperclip),
            '📏' => Ok(MiscellaneousSymbolsandPictographs::StraightRuler),
            '📐' => Ok(MiscellaneousSymbolsandPictographs::TriangularRuler),
            '📑' => Ok(MiscellaneousSymbolsandPictographs::BookmarkTabs),
            '📒' => Ok(MiscellaneousSymbolsandPictographs::Ledger),
            '📓' => Ok(MiscellaneousSymbolsandPictographs::Notebook),
            '📔' => Ok(MiscellaneousSymbolsandPictographs::NotebookWithDecorativeCover),
            '📕' => Ok(MiscellaneousSymbolsandPictographs::ClosedBook),
            '📖' => Ok(MiscellaneousSymbolsandPictographs::OpenBook),
            '📗' => Ok(MiscellaneousSymbolsandPictographs::GreenBook),
            '📘' => Ok(MiscellaneousSymbolsandPictographs::BlueBook),
            '📙' => Ok(MiscellaneousSymbolsandPictographs::OrangeBook),
            '📚' => Ok(MiscellaneousSymbolsandPictographs::Books),
            '📛' => Ok(MiscellaneousSymbolsandPictographs::NameBadge),
            '📜' => Ok(MiscellaneousSymbolsandPictographs::Scroll),
            '📝' => Ok(MiscellaneousSymbolsandPictographs::Memo),
            '📞' => Ok(MiscellaneousSymbolsandPictographs::TelephoneReceiver),
            '📟' => Ok(MiscellaneousSymbolsandPictographs::Pager),
            '📠' => Ok(MiscellaneousSymbolsandPictographs::FaxMachine),
            '📡' => Ok(MiscellaneousSymbolsandPictographs::SatelliteAntenna),
            '📢' => Ok(MiscellaneousSymbolsandPictographs::PublicAddressLoudspeaker),
            '📣' => Ok(MiscellaneousSymbolsandPictographs::CheeringMegaphone),
            '📤' => Ok(MiscellaneousSymbolsandPictographs::OutboxTray),
            '📥' => Ok(MiscellaneousSymbolsandPictographs::InboxTray),
            '📦' => Ok(MiscellaneousSymbolsandPictographs::Package),
            '📧' => Ok(MiscellaneousSymbolsandPictographs::EDashMailSymbol),
            '📨' => Ok(MiscellaneousSymbolsandPictographs::IncomingEnvelope),
            '📩' => Ok(MiscellaneousSymbolsandPictographs::EnvelopeWithDownwardsArrowAbove),
            '📪' => Ok(MiscellaneousSymbolsandPictographs::ClosedMailboxWithLoweredFlag),
            '📫' => Ok(MiscellaneousSymbolsandPictographs::ClosedMailboxWithRaisedFlag),
            '📬' => Ok(MiscellaneousSymbolsandPictographs::OpenMailboxWithRaisedFlag),
            '📭' => Ok(MiscellaneousSymbolsandPictographs::OpenMailboxWithLoweredFlag),
            '📮' => Ok(MiscellaneousSymbolsandPictographs::Postbox),
            '📯' => Ok(MiscellaneousSymbolsandPictographs::PostalHorn),
            '📰' => Ok(MiscellaneousSymbolsandPictographs::Newspaper),
            '📱' => Ok(MiscellaneousSymbolsandPictographs::MobilePhone),
            '📲' => Ok(MiscellaneousSymbolsandPictographs::MobilePhoneWithRightwardsArrowAtLeft),
            '📳' => Ok(MiscellaneousSymbolsandPictographs::VibrationMode),
            '📴' => Ok(MiscellaneousSymbolsandPictographs::MobilePhoneOff),
            '📵' => Ok(MiscellaneousSymbolsandPictographs::NoMobilePhones),
            '📶' => Ok(MiscellaneousSymbolsandPictographs::AntennaWithBars),
            '📷' => Ok(MiscellaneousSymbolsandPictographs::Camera),
            '📸' => Ok(MiscellaneousSymbolsandPictographs::CameraWithFlash),
            '📹' => Ok(MiscellaneousSymbolsandPictographs::VideoCamera),
            '📺' => Ok(MiscellaneousSymbolsandPictographs::Television),
            '📻' => Ok(MiscellaneousSymbolsandPictographs::Radio),
            '📼' => Ok(MiscellaneousSymbolsandPictographs::Videocassette),
            '📽' => Ok(MiscellaneousSymbolsandPictographs::FilmProjector),
            '📾' => Ok(MiscellaneousSymbolsandPictographs::PortableStereo),
            '📿' => Ok(MiscellaneousSymbolsandPictographs::PrayerBeads),
            '🔀' => Ok(MiscellaneousSymbolsandPictographs::TwistedRightwardsArrows),
            '🔁' => Ok(MiscellaneousSymbolsandPictographs::ClockwiseRightwardsAndLeftwardsOpenCircleArrows),
            '🔂' => Ok(MiscellaneousSymbolsandPictographs::ClockwiseRightwardsAndLeftwardsOpenCircleArrowsWithCircledOneOverlay),
            '🔃' => Ok(MiscellaneousSymbolsandPictographs::ClockwiseDownwardsAndUpwardsOpenCircleArrows),
            '🔄' => Ok(MiscellaneousSymbolsandPictographs::AnticlockwiseDownwardsAndUpwardsOpenCircleArrows),
            '🔅' => Ok(MiscellaneousSymbolsandPictographs::LowBrightnessSymbol),
            '🔆' => Ok(MiscellaneousSymbolsandPictographs::HighBrightnessSymbol),
            '🔇' => Ok(MiscellaneousSymbolsandPictographs::SpeakerWithCancellationStroke),
            '🔈' => Ok(MiscellaneousSymbolsandPictographs::Speaker),
            '🔉' => Ok(MiscellaneousSymbolsandPictographs::SpeakerWithOneSoundWave),
            '🔊' => Ok(MiscellaneousSymbolsandPictographs::SpeakerWithThreeSoundWaves),
            '🔋' => Ok(MiscellaneousSymbolsandPictographs::Battery),
            '🔌' => Ok(MiscellaneousSymbolsandPictographs::ElectricPlug),
            '🔍' => Ok(MiscellaneousSymbolsandPictographs::LeftDashPointingMagnifyingGlass),
            '🔎' => Ok(MiscellaneousSymbolsandPictographs::RightDashPointingMagnifyingGlass),
            '🔏' => Ok(MiscellaneousSymbolsandPictographs::LockWithInkPen),
            '🔐' => Ok(MiscellaneousSymbolsandPictographs::ClosedLockWithKey),
            '🔑' => Ok(MiscellaneousSymbolsandPictographs::Key),
            '🔒' => Ok(MiscellaneousSymbolsandPictographs::Lock),
            '🔓' => Ok(MiscellaneousSymbolsandPictographs::OpenLock),
            '🔔' => Ok(MiscellaneousSymbolsandPictographs::Bell),
            '🔕' => Ok(MiscellaneousSymbolsandPictographs::BellWithCancellationStroke),
            '🔖' => Ok(MiscellaneousSymbolsandPictographs::Bookmark),
            '🔗' => Ok(MiscellaneousSymbolsandPictographs::LinkSymbol),
            '🔘' => Ok(MiscellaneousSymbolsandPictographs::RadioButton),
            '🔙' => Ok(MiscellaneousSymbolsandPictographs::BackWithLeftwardsArrowAbove),
            '🔚' => Ok(MiscellaneousSymbolsandPictographs::EndWithLeftwardsArrowAbove),
            '🔛' => Ok(MiscellaneousSymbolsandPictographs::OnWithExclamationMarkWithLeftRightArrowAbove),
            '🔜' => Ok(MiscellaneousSymbolsandPictographs::SoonWithRightwardsArrowAbove),
            '🔝' => Ok(MiscellaneousSymbolsandPictographs::TopWithUpwardsArrowAbove),
            '🔞' => Ok(MiscellaneousSymbolsandPictographs::NoOneUnderEighteenSymbol),
            '🔟' => Ok(MiscellaneousSymbolsandPictographs::KeycapTen),
            '🔠' => Ok(MiscellaneousSymbolsandPictographs::InputSymbolForLatinCapitalLetters),
            '🔡' => Ok(MiscellaneousSymbolsandPictographs::InputSymbolForLatinSmallLetters),
            '🔢' => Ok(MiscellaneousSymbolsandPictographs::InputSymbolForNumbers),
            '🔣' => Ok(MiscellaneousSymbolsandPictographs::InputSymbolForSymbols),
            '🔤' => Ok(MiscellaneousSymbolsandPictographs::InputSymbolForLatinLetters),
            '🔥' => Ok(MiscellaneousSymbolsandPictographs::Fire),
            '🔦' => Ok(MiscellaneousSymbolsandPictographs::ElectricTorch),
            '🔧' => Ok(MiscellaneousSymbolsandPictographs::Wrench),
            '🔨' => Ok(MiscellaneousSymbolsandPictographs::Hammer),
            '🔩' => Ok(MiscellaneousSymbolsandPictographs::NutAndBolt),
            '🔪' => Ok(MiscellaneousSymbolsandPictographs::Hocho),
            '🔫' => Ok(MiscellaneousSymbolsandPictographs::Pistol),
            '🔬' => Ok(MiscellaneousSymbolsandPictographs::Microscope),
            '🔭' => Ok(MiscellaneousSymbolsandPictographs::Telescope),
            '🔮' => Ok(MiscellaneousSymbolsandPictographs::CrystalBall),
            '🔯' => Ok(MiscellaneousSymbolsandPictographs::SixPointedStarWithMiddleDot),
            '🔰' => Ok(MiscellaneousSymbolsandPictographs::JapaneseSymbolForBeginner),
            '🔱' => Ok(MiscellaneousSymbolsandPictographs::TridentEmblem),
            '🔲' => Ok(MiscellaneousSymbolsandPictographs::BlackSquareButton),
            '🔳' => Ok(MiscellaneousSymbolsandPictographs::WhiteSquareButton),
            '🔴' => Ok(MiscellaneousSymbolsandPictographs::LargeRedCircle),
            '🔵' => Ok(MiscellaneousSymbolsandPictographs::LargeBlueCircle),
            '🔶' => Ok(MiscellaneousSymbolsandPictographs::LargeOrangeDiamond),
            '🔷' => Ok(MiscellaneousSymbolsandPictographs::LargeBlueDiamond),
            '🔸' => Ok(MiscellaneousSymbolsandPictographs::SmallOrangeDiamond),
            '🔹' => Ok(MiscellaneousSymbolsandPictographs::SmallBlueDiamond),
            '🔺' => Ok(MiscellaneousSymbolsandPictographs::UpDashPointingRedTriangle),
            '🔻' => Ok(MiscellaneousSymbolsandPictographs::DownDashPointingRedTriangle),
            '🔼' => Ok(MiscellaneousSymbolsandPictographs::UpDashPointingSmallRedTriangle),
            '🔽' => Ok(MiscellaneousSymbolsandPictographs::DownDashPointingSmallRedTriangle),
            '🔾' => Ok(MiscellaneousSymbolsandPictographs::LowerRightShadowedWhiteCircle),
            '🔿' => Ok(MiscellaneousSymbolsandPictographs::UpperRightShadowedWhiteCircle),
            '🕀' => Ok(MiscellaneousSymbolsandPictographs::CircledCrossPommee),
            '🕁' => Ok(MiscellaneousSymbolsandPictographs::CrossPommeeWithHalfDashCircleBelow),
            '🕂' => Ok(MiscellaneousSymbolsandPictographs::CrossPommee),
            '🕃' => Ok(MiscellaneousSymbolsandPictographs::NotchedLeftSemicircleWithThreeDots),
            '🕄' => Ok(MiscellaneousSymbolsandPictographs::NotchedRightSemicircleWithThreeDots),
            '🕅' => Ok(MiscellaneousSymbolsandPictographs::SymbolForMarksChapter),
            '🕆' => Ok(MiscellaneousSymbolsandPictographs::WhiteLatinCross),
            '🕇' => Ok(MiscellaneousSymbolsandPictographs::HeavyLatinCross),
            '🕈' => Ok(MiscellaneousSymbolsandPictographs::CelticCross),
            '🕉' => Ok(MiscellaneousSymbolsandPictographs::OmSymbol),
            '🕊' => Ok(MiscellaneousSymbolsandPictographs::DoveOfPeace),
            '🕋' => Ok(MiscellaneousSymbolsandPictographs::Kaaba),
            '🕌' => Ok(MiscellaneousSymbolsandPictographs::Mosque),
            '🕍' => Ok(MiscellaneousSymbolsandPictographs::Synagogue),
            '🕎' => Ok(MiscellaneousSymbolsandPictographs::MenorahWithNineBranches),
            '🕏' => Ok(MiscellaneousSymbolsandPictographs::BowlOfHygieia),
            '🕐' => Ok(MiscellaneousSymbolsandPictographs::ClockFaceOneOclock),
            '🕑' => Ok(MiscellaneousSymbolsandPictographs::ClockFaceTwoOclock),
            '🕒' => Ok(MiscellaneousSymbolsandPictographs::ClockFaceThreeOclock),
            '🕓' => Ok(MiscellaneousSymbolsandPictographs::ClockFaceFourOclock),
            '🕔' => Ok(MiscellaneousSymbolsandPictographs::ClockFaceFiveOclock),
            '🕕' => Ok(MiscellaneousSymbolsandPictographs::ClockFaceSixOclock),
            '🕖' => Ok(MiscellaneousSymbolsandPictographs::ClockFaceSevenOclock),
            '🕗' => Ok(MiscellaneousSymbolsandPictographs::ClockFaceEightOclock),
            '🕘' => Ok(MiscellaneousSymbolsandPictographs::ClockFaceNineOclock),
            '🕙' => Ok(MiscellaneousSymbolsandPictographs::ClockFaceTenOclock),
            '🕚' => Ok(MiscellaneousSymbolsandPictographs::ClockFaceElevenOclock),
            '🕛' => Ok(MiscellaneousSymbolsandPictographs::ClockFaceTwelveOclock),
            '🕜' => Ok(MiscellaneousSymbolsandPictographs::ClockFaceOneDashThirty),
            '🕝' => Ok(MiscellaneousSymbolsandPictographs::ClockFaceTwoDashThirty),
            '🕞' => Ok(MiscellaneousSymbolsandPictographs::ClockFaceThreeDashThirty),
            '🕟' => Ok(MiscellaneousSymbolsandPictographs::ClockFaceFourDashThirty),
            '🕠' => Ok(MiscellaneousSymbolsandPictographs::ClockFaceFiveDashThirty),
            '🕡' => Ok(MiscellaneousSymbolsandPictographs::ClockFaceSixDashThirty),
            '🕢' => Ok(MiscellaneousSymbolsandPictographs::ClockFaceSevenDashThirty),
            '🕣' => Ok(MiscellaneousSymbolsandPictographs::ClockFaceEightDashThirty),
            '🕤' => Ok(MiscellaneousSymbolsandPictographs::ClockFaceNineDashThirty),
            '🕥' => Ok(MiscellaneousSymbolsandPictographs::ClockFaceTenDashThirty),
            '🕦' => Ok(MiscellaneousSymbolsandPictographs::ClockFaceElevenDashThirty),
            '🕧' => Ok(MiscellaneousSymbolsandPictographs::ClockFaceTwelveDashThirty),
            '🕨' => Ok(MiscellaneousSymbolsandPictographs::RightSpeaker),
            '🕩' => Ok(MiscellaneousSymbolsandPictographs::RightSpeakerWithOneSoundWave),
            '🕪' => Ok(MiscellaneousSymbolsandPictographs::RightSpeakerWithThreeSoundWaves),
            '🕫' => Ok(MiscellaneousSymbolsandPictographs::Bullhorn),
            '🕬' => Ok(MiscellaneousSymbolsandPictographs::BullhornWithSoundWaves),
            '🕭' => Ok(MiscellaneousSymbolsandPictographs::RingingBell),
            '🕮' => Ok(MiscellaneousSymbolsandPictographs::Book),
            '🕯' => Ok(MiscellaneousSymbolsandPictographs::Candle),
            '🕰' => Ok(MiscellaneousSymbolsandPictographs::MantelpieceClock),
            '🕱' => Ok(MiscellaneousSymbolsandPictographs::BlackSkullAndCrossbones),
            '🕲' => Ok(MiscellaneousSymbolsandPictographs::NoPiracy),
            '🕳' => Ok(MiscellaneousSymbolsandPictographs::Hole),
            '🕴' => Ok(MiscellaneousSymbolsandPictographs::ManInBusinessSuitLevitating),
            '🕵' => Ok(MiscellaneousSymbolsandPictographs::SleuthOrSpy),
            '🕶' => Ok(MiscellaneousSymbolsandPictographs::DarkSunglasses),
            '🕷' => Ok(MiscellaneousSymbolsandPictographs::Spider),
            '🕸' => Ok(MiscellaneousSymbolsandPictographs::SpiderWeb),
            '🕹' => Ok(MiscellaneousSymbolsandPictographs::Joystick),
            '🕺' => Ok(MiscellaneousSymbolsandPictographs::ManDancing),
            '🕻' => Ok(MiscellaneousSymbolsandPictographs::LeftHandTelephoneReceiver),
            '🕼' => Ok(MiscellaneousSymbolsandPictographs::TelephoneReceiverWithPage),
            '🕽' => Ok(MiscellaneousSymbolsandPictographs::RightHandTelephoneReceiver),
            '🕾' => Ok(MiscellaneousSymbolsandPictographs::WhiteTouchtoneTelephone),
            '🕿' => Ok(MiscellaneousSymbolsandPictographs::BlackTouchtoneTelephone),
            '🖀' => Ok(MiscellaneousSymbolsandPictographs::TelephoneOnTopOfModem),
            '🖁' => Ok(MiscellaneousSymbolsandPictographs::ClamshellMobilePhone),
            '🖂' => Ok(MiscellaneousSymbolsandPictographs::BackOfEnvelope),
            '🖃' => Ok(MiscellaneousSymbolsandPictographs::StampedEnvelope),
            '🖄' => Ok(MiscellaneousSymbolsandPictographs::EnvelopeWithLightning),
            '🖅' => Ok(MiscellaneousSymbolsandPictographs::FlyingEnvelope),
            '🖆' => Ok(MiscellaneousSymbolsandPictographs::PenOverStampedEnvelope),
            '🖇' => Ok(MiscellaneousSymbolsandPictographs::LinkedPaperclips),
            '🖈' => Ok(MiscellaneousSymbolsandPictographs::BlackPushpin),
            '🖉' => Ok(MiscellaneousSymbolsandPictographs::LowerLeftPencil),
            '🖊' => Ok(MiscellaneousSymbolsandPictographs::LowerLeftBallpointPen),
            '🖋' => Ok(MiscellaneousSymbolsandPictographs::LowerLeftFountainPen),
            '🖌' => Ok(MiscellaneousSymbolsandPictographs::LowerLeftPaintbrush),
            '🖍' => Ok(MiscellaneousSymbolsandPictographs::LowerLeftCrayon),
            '🖎' => Ok(MiscellaneousSymbolsandPictographs::LeftWritingHand),
            '🖏' => Ok(MiscellaneousSymbolsandPictographs::TurnedOkHandSign),
            '🖐' => Ok(MiscellaneousSymbolsandPictographs::RaisedHandWithFingersSplayed),
            '🖑' => Ok(MiscellaneousSymbolsandPictographs::ReversedRaisedHandWithFingersSplayed),
            '🖒' => Ok(MiscellaneousSymbolsandPictographs::ReversedThumbsUpSign),
            '🖓' => Ok(MiscellaneousSymbolsandPictographs::ReversedThumbsDownSign),
            '🖔' => Ok(MiscellaneousSymbolsandPictographs::ReversedVictoryHand),
            '🖕' => Ok(MiscellaneousSymbolsandPictographs::ReversedHandWithMiddleFingerExtended),
            '🖖' => Ok(MiscellaneousSymbolsandPictographs::RaisedHandWithPartBetweenMiddleAndRingFingers),
            '🖗' => Ok(MiscellaneousSymbolsandPictographs::WhiteDownPointingLeftHandIndex),
            '🖘' => Ok(MiscellaneousSymbolsandPictographs::SidewaysWhiteLeftPointingIndex),
            '🖙' => Ok(MiscellaneousSymbolsandPictographs::SidewaysWhiteRightPointingIndex),
            '🖚' => Ok(MiscellaneousSymbolsandPictographs::SidewaysBlackLeftPointingIndex),
            '🖛' => Ok(MiscellaneousSymbolsandPictographs::SidewaysBlackRightPointingIndex),
            '🖜' => Ok(MiscellaneousSymbolsandPictographs::BlackLeftPointingBackhandIndex),
            '🖝' => Ok(MiscellaneousSymbolsandPictographs::BlackRightPointingBackhandIndex),
            '🖞' => Ok(MiscellaneousSymbolsandPictographs::SidewaysWhiteUpPointingIndex),
            '🖟' => Ok(MiscellaneousSymbolsandPictographs::SidewaysWhiteDownPointingIndex),
            '🖠' => Ok(MiscellaneousSymbolsandPictographs::SidewaysBlackUpPointingIndex),
            '🖡' => Ok(MiscellaneousSymbolsandPictographs::SidewaysBlackDownPointingIndex),
            '🖢' => Ok(MiscellaneousSymbolsandPictographs::BlackUpPointingBackhandIndex),
            '🖣' => Ok(MiscellaneousSymbolsandPictographs::BlackDownPointingBackhandIndex),
            '🖤' => Ok(MiscellaneousSymbolsandPictographs::BlackHeart),
            '🖥' => Ok(MiscellaneousSymbolsandPictographs::DesktopComputer),
            '🖦' => Ok(MiscellaneousSymbolsandPictographs::KeyboardAndMouse),
            '🖧' => Ok(MiscellaneousSymbolsandPictographs::ThreeNetworkedComputers),
            '🖨' => Ok(MiscellaneousSymbolsandPictographs::Printer),
            '🖩' => Ok(MiscellaneousSymbolsandPictographs::PocketCalculator),
            '🖪' => Ok(MiscellaneousSymbolsandPictographs::BlackHardShellFloppyDisk),
            '🖫' => Ok(MiscellaneousSymbolsandPictographs::WhiteHardShellFloppyDisk),
            '🖬' => Ok(MiscellaneousSymbolsandPictographs::SoftShellFloppyDisk),
            '🖭' => Ok(MiscellaneousSymbolsandPictographs::TapeCartridge),
            '🖮' => Ok(MiscellaneousSymbolsandPictographs::WiredKeyboard),
            '🖯' => Ok(MiscellaneousSymbolsandPictographs::OneButtonMouse),
            '🖰' => Ok(MiscellaneousSymbolsandPictographs::TwoButtonMouse),
            '🖱' => Ok(MiscellaneousSymbolsandPictographs::ThreeButtonMouse),
            '🖲' => Ok(MiscellaneousSymbolsandPictographs::Trackball),
            '🖳' => Ok(MiscellaneousSymbolsandPictographs::OldPersonalComputer),
            '🖴' => Ok(MiscellaneousSymbolsandPictographs::HardDisk),
            '🖵' => Ok(MiscellaneousSymbolsandPictographs::Screen),
            '🖶' => Ok(MiscellaneousSymbolsandPictographs::PrinterIcon),
            '🖷' => Ok(MiscellaneousSymbolsandPictographs::FaxIcon),
            '🖸' => Ok(MiscellaneousSymbolsandPictographs::OpticalDiscIcon),
            '🖹' => Ok(MiscellaneousSymbolsandPictographs::DocumentWithText),
            '🖺' => Ok(MiscellaneousSymbolsandPictographs::DocumentWithTextAndPicture),
            '🖻' => Ok(MiscellaneousSymbolsandPictographs::DocumentWithPicture),
            '🖼' => Ok(MiscellaneousSymbolsandPictographs::FrameWithPicture),
            '🖽' => Ok(MiscellaneousSymbolsandPictographs::FrameWithTiles),
            '🖾' => Ok(MiscellaneousSymbolsandPictographs::FrameWithAnX),
            '🖿' => Ok(MiscellaneousSymbolsandPictographs::BlackFolder),
            '🗀' => Ok(MiscellaneousSymbolsandPictographs::Folder),
            '🗁' => Ok(MiscellaneousSymbolsandPictographs::OpenFolder),
            '🗂' => Ok(MiscellaneousSymbolsandPictographs::CardIndexDividers),
            '🗃' => Ok(MiscellaneousSymbolsandPictographs::CardFileBox),
            '🗄' => Ok(MiscellaneousSymbolsandPictographs::FileCabinet),
            '🗅' => Ok(MiscellaneousSymbolsandPictographs::EmptyNote),
            '🗆' => Ok(MiscellaneousSymbolsandPictographs::EmptyNotePage),
            '🗇' => Ok(MiscellaneousSymbolsandPictographs::EmptyNotePad),
            '🗈' => Ok(MiscellaneousSymbolsandPictographs::Note),
            '🗉' => Ok(MiscellaneousSymbolsandPictographs::NotePage),
            '🗊' => Ok(MiscellaneousSymbolsandPictographs::NotePad),
            '🗋' => Ok(MiscellaneousSymbolsandPictographs::EmptyDocument),
            '🗌' => Ok(MiscellaneousSymbolsandPictographs::EmptyPage),
            '🗍' => Ok(MiscellaneousSymbolsandPictographs::EmptyPages),
            '🗎' => Ok(MiscellaneousSymbolsandPictographs::Document),
            '🗏' => Ok(MiscellaneousSymbolsandPictographs::Page),
            '🗐' => Ok(MiscellaneousSymbolsandPictographs::Pages),
            '🗑' => Ok(MiscellaneousSymbolsandPictographs::Wastebasket),
            '🗒' => Ok(MiscellaneousSymbolsandPictographs::SpiralNotePad),
            '🗓' => Ok(MiscellaneousSymbolsandPictographs::SpiralCalendarPad),
            '🗔' => Ok(MiscellaneousSymbolsandPictographs::DesktopWindow),
            '🗕' => Ok(MiscellaneousSymbolsandPictographs::Minimize),
            '🗖' => Ok(MiscellaneousSymbolsandPictographs::Maximize),
            '🗗' => Ok(MiscellaneousSymbolsandPictographs::Overlap),
            '🗘' => Ok(MiscellaneousSymbolsandPictographs::ClockwiseRightAndLeftSemicircleArrows),
            '🗙' => Ok(MiscellaneousSymbolsandPictographs::CancellationX),
            '🗚' => Ok(MiscellaneousSymbolsandPictographs::IncreaseFontSizeSymbol),
            '🗛' => Ok(MiscellaneousSymbolsandPictographs::DecreaseFontSizeSymbol),
            '🗜' => Ok(MiscellaneousSymbolsandPictographs::Compression),
            '🗝' => Ok(MiscellaneousSymbolsandPictographs::OldKey),
            '🗞' => Ok(MiscellaneousSymbolsandPictographs::RolledDashUpNewspaper),
            '🗟' => Ok(MiscellaneousSymbolsandPictographs::PageWithCircledText),
            '🗠' => Ok(MiscellaneousSymbolsandPictographs::StockChart),
            '🗡' => Ok(MiscellaneousSymbolsandPictographs::DaggerKnife),
            '🗢' => Ok(MiscellaneousSymbolsandPictographs::Lips),
            '🗣' => Ok(MiscellaneousSymbolsandPictographs::SpeakingHeadInSilhouette),
            '🗤' => Ok(MiscellaneousSymbolsandPictographs::ThreeRaysAbove),
            '🗥' => Ok(MiscellaneousSymbolsandPictographs::ThreeRaysBelow),
            '🗦' => Ok(MiscellaneousSymbolsandPictographs::ThreeRaysLeft),
            '🗧' => Ok(MiscellaneousSymbolsandPictographs::ThreeRaysRight),
            '🗨' => Ok(MiscellaneousSymbolsandPictographs::LeftSpeechBubble),
            '🗩' => Ok(MiscellaneousSymbolsandPictographs::RightSpeechBubble),
            '🗪' => Ok(MiscellaneousSymbolsandPictographs::TwoSpeechBubbles),
            '🗫' => Ok(MiscellaneousSymbolsandPictographs::ThreeSpeechBubbles),
            '🗬' => Ok(MiscellaneousSymbolsandPictographs::LeftThoughtBubble),
            '🗭' => Ok(MiscellaneousSymbolsandPictographs::RightThoughtBubble),
            '🗮' => Ok(MiscellaneousSymbolsandPictographs::LeftAngerBubble),
            '🗯' => Ok(MiscellaneousSymbolsandPictographs::RightAngerBubble),
            '🗰' => Ok(MiscellaneousSymbolsandPictographs::MoodBubble),
            '🗱' => Ok(MiscellaneousSymbolsandPictographs::LightningMoodBubble),
            '🗲' => Ok(MiscellaneousSymbolsandPictographs::LightningMood),
            '🗳' => Ok(MiscellaneousSymbolsandPictographs::BallotBoxWithBallot),
            '🗴' => Ok(MiscellaneousSymbolsandPictographs::BallotScriptX),
            '🗵' => Ok(MiscellaneousSymbolsandPictographs::BallotBoxWithScriptX),
            '🗶' => Ok(MiscellaneousSymbolsandPictographs::BallotBoldScriptX),
            '🗷' => Ok(MiscellaneousSymbolsandPictographs::BallotBoxWithBoldScriptX),
            '🗸' => Ok(MiscellaneousSymbolsandPictographs::LightCheckMark),
            '🗹' => Ok(MiscellaneousSymbolsandPictographs::BallotBoxWithBoldCheck),
            '🗺' => Ok(MiscellaneousSymbolsandPictographs::WorldMap),
            '🗻' => Ok(MiscellaneousSymbolsandPictographs::MountFuji),
            '🗼' => Ok(MiscellaneousSymbolsandPictographs::TokyoTower),
            '🗽' => Ok(MiscellaneousSymbolsandPictographs::StatueOfLiberty),
            '🗾' => Ok(MiscellaneousSymbolsandPictographs::SilhouetteOfJapan),
            _ => Err(()),
        }
    }
}

impl Into<u32> for MiscellaneousSymbolsandPictographs {
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

impl std::convert::TryFrom<u32> for MiscellaneousSymbolsandPictographs {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for MiscellaneousSymbolsandPictographs {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl MiscellaneousSymbolsandPictographs {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        MiscellaneousSymbolsandPictographs::Cyclone
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("MiscellaneousSymbolsandPictographs{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
