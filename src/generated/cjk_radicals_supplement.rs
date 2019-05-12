/// \u{2e80} → \u{2eff}\
///\
/// ⺀ ⺁ ⺂ ⺃ ⺄ ⺅ ⺆ ⺇ ⺈ ⺉ ⺊ ⺋ ⺌ ⺍ ⺎ ⺏\
/// ⺐ ⺑ ⺒ ⺓ ⺔ ⺕ ⺖ ⺗ ⺘ ⺙ ⺛ ⺜ ⺝ ⺞ ⺟ ⺠\
/// ⺡ ⺢ ⺣ ⺤ ⺥ ⺦ ⺧ ⺨ ⺩ ⺪ ⺫ ⺬ ⺭ ⺮ ⺯ ⺰\
/// ⺱ ⺲ ⺳ ⺴ ⺵ ⺶ ⺷ ⺸ ⺹ ⺺ ⺻ ⺼ ⺽ ⺾ ⺿ ⻀\
/// ⻁ ⻂ ⻃ ⻄ ⻅ ⻆ ⻇ ⻈ ⻉ ⻊ ⻋ ⻌ ⻍ ⻎ ⻏ ⻐\
/// ⻑ ⻒ ⻓ ⻔ ⻕ ⻖ ⻗ ⻘ ⻙ ⻚ ⻛ ⻜ ⻝ ⻞ ⻟ ⻠\
/// ⻡ ⻢ ⻣ ⻤ ⻥ ⻦ ⻧ ⻨ ⻩ ⻪ ⻫ ⻬ ⻭ ⻮ ⻯ ⻰\
/// ⻱ ⻲ ⻳\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{2e80}: '⺀'
    pub const CJK_RADICAL_REPEAT: char = '⺀';
    /// \u{2e81}: '⺁'
    pub const CJK_RADICAL_CLIFF: char = '⺁';
    /// \u{2e82}: '⺂'
    pub const CJK_RADICAL_SECOND_ONE: char = '⺂';
    /// \u{2e83}: '⺃'
    pub const CJK_RADICAL_SECOND_TWO: char = '⺃';
    /// \u{2e84}: '⺄'
    pub const CJK_RADICAL_SECOND_THREE: char = '⺄';
    /// \u{2e85}: '⺅'
    pub const CJK_RADICAL_PERSON: char = '⺅';
    /// \u{2e86}: '⺆'
    pub const CJK_RADICAL_BOX: char = '⺆';
    /// \u{2e87}: '⺇'
    pub const CJK_RADICAL_TABLE: char = '⺇';
    /// \u{2e88}: '⺈'
    pub const CJK_RADICAL_KNIFE_ONE: char = '⺈';
    /// \u{2e89}: '⺉'
    pub const CJK_RADICAL_KNIFE_TWO: char = '⺉';
    /// \u{2e8a}: '⺊'
    pub const CJK_RADICAL_DIVINATION: char = '⺊';
    /// \u{2e8b}: '⺋'
    pub const CJK_RADICAL_SEAL: char = '⺋';
    /// \u{2e8c}: '⺌'
    pub const CJK_RADICAL_SMALL_ONE: char = '⺌';
    /// \u{2e8d}: '⺍'
    pub const CJK_RADICAL_SMALL_TWO: char = '⺍';
    /// \u{2e8e}: '⺎'
    pub const CJK_RADICAL_LAME_ONE: char = '⺎';
    /// \u{2e8f}: '⺏'
    pub const CJK_RADICAL_LAME_TWO: char = '⺏';
    /// \u{2e90}: '⺐'
    pub const CJK_RADICAL_LAME_THREE: char = '⺐';
    /// \u{2e91}: '⺑'
    pub const CJK_RADICAL_LAME_FOUR: char = '⺑';
    /// \u{2e92}: '⺒'
    pub const CJK_RADICAL_SNAKE: char = '⺒';
    /// \u{2e93}: '⺓'
    pub const CJK_RADICAL_THREAD: char = '⺓';
    /// \u{2e94}: '⺔'
    pub const CJK_RADICAL_SNOUT_ONE: char = '⺔';
    /// \u{2e95}: '⺕'
    pub const CJK_RADICAL_SNOUT_TWO: char = '⺕';
    /// \u{2e96}: '⺖'
    pub const CJK_RADICAL_HEART_ONE: char = '⺖';
    /// \u{2e97}: '⺗'
    pub const CJK_RADICAL_HEART_TWO: char = '⺗';
    /// \u{2e98}: '⺘'
    pub const CJK_RADICAL_HAND: char = '⺘';
    /// \u{2e99}: '⺙'
    pub const CJK_RADICAL_RAP: char = '⺙';
    /// \u{2e9b}: '⺛'
    pub const CJK_RADICAL_CHOKE: char = '⺛';
    /// \u{2e9c}: '⺜'
    pub const CJK_RADICAL_SUN: char = '⺜';
    /// \u{2e9d}: '⺝'
    pub const CJK_RADICAL_MOON: char = '⺝';
    /// \u{2e9e}: '⺞'
    pub const CJK_RADICAL_DEATH: char = '⺞';
    /// \u{2e9f}: '⺟'
    pub const CJK_RADICAL_MOTHER: char = '⺟';
    /// \u{2ea0}: '⺠'
    pub const CJK_RADICAL_CIVILIAN: char = '⺠';
    /// \u{2ea1}: '⺡'
    pub const CJK_RADICAL_WATER_ONE: char = '⺡';
    /// \u{2ea2}: '⺢'
    pub const CJK_RADICAL_WATER_TWO: char = '⺢';
    /// \u{2ea3}: '⺣'
    pub const CJK_RADICAL_FIRE: char = '⺣';
    /// \u{2ea4}: '⺤'
    pub const CJK_RADICAL_PAW_ONE: char = '⺤';
    /// \u{2ea5}: '⺥'
    pub const CJK_RADICAL_PAW_TWO: char = '⺥';
    /// \u{2ea6}: '⺦'
    pub const CJK_RADICAL_SIMPLIFIED_HALF_TREE_TRUNK: char = '⺦';
    /// \u{2ea7}: '⺧'
    pub const CJK_RADICAL_COW: char = '⺧';
    /// \u{2ea8}: '⺨'
    pub const CJK_RADICAL_DOG: char = '⺨';
    /// \u{2ea9}: '⺩'
    pub const CJK_RADICAL_JADE: char = '⺩';
    /// \u{2eaa}: '⺪'
    pub const CJK_RADICAL_BOLT_OF_CLOTH: char = '⺪';
    /// \u{2eab}: '⺫'
    pub const CJK_RADICAL_EYE: char = '⺫';
    /// \u{2eac}: '⺬'
    pub const CJK_RADICAL_SPIRIT_ONE: char = '⺬';
    /// \u{2ead}: '⺭'
    pub const CJK_RADICAL_SPIRIT_TWO: char = '⺭';
    /// \u{2eae}: '⺮'
    pub const CJK_RADICAL_BAMBOO: char = '⺮';
    /// \u{2eaf}: '⺯'
    pub const CJK_RADICAL_SILK: char = '⺯';
    /// \u{2eb0}: '⺰'
    pub const CJK_RADICAL_C_DASH_SIMPLIFIED_SILK: char = '⺰';
    /// \u{2eb1}: '⺱'
    pub const CJK_RADICAL_NET_ONE: char = '⺱';
    /// \u{2eb2}: '⺲'
    pub const CJK_RADICAL_NET_TWO: char = '⺲';
    /// \u{2eb3}: '⺳'
    pub const CJK_RADICAL_NET_THREE: char = '⺳';
    /// \u{2eb4}: '⺴'
    pub const CJK_RADICAL_NET_FOUR: char = '⺴';
    /// \u{2eb5}: '⺵'
    pub const CJK_RADICAL_MESH: char = '⺵';
    /// \u{2eb6}: '⺶'
    pub const CJK_RADICAL_SHEEP: char = '⺶';
    /// \u{2eb7}: '⺷'
    pub const CJK_RADICAL_RAM: char = '⺷';
    /// \u{2eb8}: '⺸'
    pub const CJK_RADICAL_EWE: char = '⺸';
    /// \u{2eb9}: '⺹'
    pub const CJK_RADICAL_OLD: char = '⺹';
    /// \u{2eba}: '⺺'
    pub const CJK_RADICAL_BRUSH_ONE: char = '⺺';
    /// \u{2ebb}: '⺻'
    pub const CJK_RADICAL_BRUSH_TWO: char = '⺻';
    /// \u{2ebc}: '⺼'
    pub const CJK_RADICAL_MEAT: char = '⺼';
    /// \u{2ebd}: '⺽'
    pub const CJK_RADICAL_MORTAR: char = '⺽';
    /// \u{2ebe}: '⺾'
    pub const CJK_RADICAL_GRASS_ONE: char = '⺾';
    /// \u{2ebf}: '⺿'
    pub const CJK_RADICAL_GRASS_TWO: char = '⺿';
    /// \u{2ec0}: '⻀'
    pub const CJK_RADICAL_GRASS_THREE: char = '⻀';
    /// \u{2ec1}: '⻁'
    pub const CJK_RADICAL_TIGER: char = '⻁';
    /// \u{2ec2}: '⻂'
    pub const CJK_RADICAL_CLOTHES: char = '⻂';
    /// \u{2ec3}: '⻃'
    pub const CJK_RADICAL_WEST_ONE: char = '⻃';
    /// \u{2ec4}: '⻄'
    pub const CJK_RADICAL_WEST_TWO: char = '⻄';
    /// \u{2ec5}: '⻅'
    pub const CJK_RADICAL_C_DASH_SIMPLIFIED_SEE: char = '⻅';
    /// \u{2ec6}: '⻆'
    pub const CJK_RADICAL_SIMPLIFIED_HORN: char = '⻆';
    /// \u{2ec7}: '⻇'
    pub const CJK_RADICAL_HORN: char = '⻇';
    /// \u{2ec8}: '⻈'
    pub const CJK_RADICAL_C_DASH_SIMPLIFIED_SPEECH: char = '⻈';
    /// \u{2ec9}: '⻉'
    pub const CJK_RADICAL_C_DASH_SIMPLIFIED_SHELL: char = '⻉';
    /// \u{2eca}: '⻊'
    pub const CJK_RADICAL_FOOT: char = '⻊';
    /// \u{2ecb}: '⻋'
    pub const CJK_RADICAL_C_DASH_SIMPLIFIED_CART: char = '⻋';
    /// \u{2ecc}: '⻌'
    pub const CJK_RADICAL_SIMPLIFIED_WALK: char = '⻌';
    /// \u{2ecd}: '⻍'
    pub const CJK_RADICAL_WALK_ONE: char = '⻍';
    /// \u{2ece}: '⻎'
    pub const CJK_RADICAL_WALK_TWO: char = '⻎';
    /// \u{2ecf}: '⻏'
    pub const CJK_RADICAL_CITY: char = '⻏';
    /// \u{2ed0}: '⻐'
    pub const CJK_RADICAL_C_DASH_SIMPLIFIED_GOLD: char = '⻐';
    /// \u{2ed1}: '⻑'
    pub const CJK_RADICAL_LONG_ONE: char = '⻑';
    /// \u{2ed2}: '⻒'
    pub const CJK_RADICAL_LONG_TWO: char = '⻒';
    /// \u{2ed3}: '⻓'
    pub const CJK_RADICAL_C_DASH_SIMPLIFIED_LONG: char = '⻓';
    /// \u{2ed4}: '⻔'
    pub const CJK_RADICAL_C_DASH_SIMPLIFIED_GATE: char = '⻔';
    /// \u{2ed5}: '⻕'
    pub const CJK_RADICAL_MOUND_ONE: char = '⻕';
    /// \u{2ed6}: '⻖'
    pub const CJK_RADICAL_MOUND_TWO: char = '⻖';
    /// \u{2ed7}: '⻗'
    pub const CJK_RADICAL_RAIN: char = '⻗';
    /// \u{2ed8}: '⻘'
    pub const CJK_RADICAL_BLUE: char = '⻘';
    /// \u{2ed9}: '⻙'
    pub const CJK_RADICAL_C_DASH_SIMPLIFIED_TANNED_LEATHER: char = '⻙';
    /// \u{2eda}: '⻚'
    pub const CJK_RADICAL_C_DASH_SIMPLIFIED_LEAF: char = '⻚';
    /// \u{2edb}: '⻛'
    pub const CJK_RADICAL_C_DASH_SIMPLIFIED_WIND: char = '⻛';
    /// \u{2edc}: '⻜'
    pub const CJK_RADICAL_C_DASH_SIMPLIFIED_FLY: char = '⻜';
    /// \u{2edd}: '⻝'
    pub const CJK_RADICAL_EAT_ONE: char = '⻝';
    /// \u{2ede}: '⻞'
    pub const CJK_RADICAL_EAT_TWO: char = '⻞';
    /// \u{2edf}: '⻟'
    pub const CJK_RADICAL_EAT_THREE: char = '⻟';
    /// \u{2ee0}: '⻠'
    pub const CJK_RADICAL_C_DASH_SIMPLIFIED_EAT: char = '⻠';
    /// \u{2ee1}: '⻡'
    pub const CJK_RADICAL_HEAD: char = '⻡';
    /// \u{2ee2}: '⻢'
    pub const CJK_RADICAL_C_DASH_SIMPLIFIED_HORSE: char = '⻢';
    /// \u{2ee3}: '⻣'
    pub const CJK_RADICAL_BONE: char = '⻣';
    /// \u{2ee4}: '⻤'
    pub const CJK_RADICAL_GHOST: char = '⻤';
    /// \u{2ee5}: '⻥'
    pub const CJK_RADICAL_C_DASH_SIMPLIFIED_FISH: char = '⻥';
    /// \u{2ee6}: '⻦'
    pub const CJK_RADICAL_C_DASH_SIMPLIFIED_BIRD: char = '⻦';
    /// \u{2ee7}: '⻧'
    pub const CJK_RADICAL_C_DASH_SIMPLIFIED_SALT: char = '⻧';
    /// \u{2ee8}: '⻨'
    pub const CJK_RADICAL_SIMPLIFIED_WHEAT: char = '⻨';
    /// \u{2ee9}: '⻩'
    pub const CJK_RADICAL_SIMPLIFIED_YELLOW: char = '⻩';
    /// \u{2eea}: '⻪'
    pub const CJK_RADICAL_C_DASH_SIMPLIFIED_FROG: char = '⻪';
    /// \u{2eeb}: '⻫'
    pub const CJK_RADICAL_J_DASH_SIMPLIFIED_EVEN: char = '⻫';
    /// \u{2eec}: '⻬'
    pub const CJK_RADICAL_C_DASH_SIMPLIFIED_EVEN: char = '⻬';
    /// \u{2eed}: '⻭'
    pub const CJK_RADICAL_J_DASH_SIMPLIFIED_TOOTH: char = '⻭';
    /// \u{2eee}: '⻮'
    pub const CJK_RADICAL_C_DASH_SIMPLIFIED_TOOTH: char = '⻮';
    /// \u{2eef}: '⻯'
    pub const CJK_RADICAL_J_DASH_SIMPLIFIED_DRAGON: char = '⻯';
    /// \u{2ef0}: '⻰'
    pub const CJK_RADICAL_C_DASH_SIMPLIFIED_DRAGON: char = '⻰';
    /// \u{2ef1}: '⻱'
    pub const CJK_RADICAL_TURTLE: char = '⻱';
    /// \u{2ef2}: '⻲'
    pub const CJK_RADICAL_J_DASH_SIMPLIFIED_TURTLE: char = '⻲';
    /// \u{2ef3}: '⻳'
    pub const CJK_RADICAL_C_DASH_SIMPLIFIED_TURTLE: char = '⻳';
}

/// An enum to represent all characters in the CJKRadicalsSupplement block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum CJKRadicalsSupplement {
    /// \u{2e80}: '⺀'
    CjkRadicalRepeat,
    /// \u{2e81}: '⺁'
    CjkRadicalCliff,
    /// \u{2e82}: '⺂'
    CjkRadicalSecondOne,
    /// \u{2e83}: '⺃'
    CjkRadicalSecondTwo,
    /// \u{2e84}: '⺄'
    CjkRadicalSecondThree,
    /// \u{2e85}: '⺅'
    CjkRadicalPerson,
    /// \u{2e86}: '⺆'
    CjkRadicalBox,
    /// \u{2e87}: '⺇'
    CjkRadicalTable,
    /// \u{2e88}: '⺈'
    CjkRadicalKnifeOne,
    /// \u{2e89}: '⺉'
    CjkRadicalKnifeTwo,
    /// \u{2e8a}: '⺊'
    CjkRadicalDivination,
    /// \u{2e8b}: '⺋'
    CjkRadicalSeal,
    /// \u{2e8c}: '⺌'
    CjkRadicalSmallOne,
    /// \u{2e8d}: '⺍'
    CjkRadicalSmallTwo,
    /// \u{2e8e}: '⺎'
    CjkRadicalLameOne,
    /// \u{2e8f}: '⺏'
    CjkRadicalLameTwo,
    /// \u{2e90}: '⺐'
    CjkRadicalLameThree,
    /// \u{2e91}: '⺑'
    CjkRadicalLameFour,
    /// \u{2e92}: '⺒'
    CjkRadicalSnake,
    /// \u{2e93}: '⺓'
    CjkRadicalThread,
    /// \u{2e94}: '⺔'
    CjkRadicalSnoutOne,
    /// \u{2e95}: '⺕'
    CjkRadicalSnoutTwo,
    /// \u{2e96}: '⺖'
    CjkRadicalHeartOne,
    /// \u{2e97}: '⺗'
    CjkRadicalHeartTwo,
    /// \u{2e98}: '⺘'
    CjkRadicalHand,
    /// \u{2e99}: '⺙'
    CjkRadicalRap,
    /// \u{2e9b}: '⺛'
    CjkRadicalChoke,
    /// \u{2e9c}: '⺜'
    CjkRadicalSun,
    /// \u{2e9d}: '⺝'
    CjkRadicalMoon,
    /// \u{2e9e}: '⺞'
    CjkRadicalDeath,
    /// \u{2e9f}: '⺟'
    CjkRadicalMother,
    /// \u{2ea0}: '⺠'
    CjkRadicalCivilian,
    /// \u{2ea1}: '⺡'
    CjkRadicalWaterOne,
    /// \u{2ea2}: '⺢'
    CjkRadicalWaterTwo,
    /// \u{2ea3}: '⺣'
    CjkRadicalFire,
    /// \u{2ea4}: '⺤'
    CjkRadicalPawOne,
    /// \u{2ea5}: '⺥'
    CjkRadicalPawTwo,
    /// \u{2ea6}: '⺦'
    CjkRadicalSimplifiedHalfTreeTrunk,
    /// \u{2ea7}: '⺧'
    CjkRadicalCow,
    /// \u{2ea8}: '⺨'
    CjkRadicalDog,
    /// \u{2ea9}: '⺩'
    CjkRadicalJade,
    /// \u{2eaa}: '⺪'
    CjkRadicalBoltOfCloth,
    /// \u{2eab}: '⺫'
    CjkRadicalEye,
    /// \u{2eac}: '⺬'
    CjkRadicalSpiritOne,
    /// \u{2ead}: '⺭'
    CjkRadicalSpiritTwo,
    /// \u{2eae}: '⺮'
    CjkRadicalBamboo,
    /// \u{2eaf}: '⺯'
    CjkRadicalSilk,
    /// \u{2eb0}: '⺰'
    CjkRadicalCDashSimplifiedSilk,
    /// \u{2eb1}: '⺱'
    CjkRadicalNetOne,
    /// \u{2eb2}: '⺲'
    CjkRadicalNetTwo,
    /// \u{2eb3}: '⺳'
    CjkRadicalNetThree,
    /// \u{2eb4}: '⺴'
    CjkRadicalNetFour,
    /// \u{2eb5}: '⺵'
    CjkRadicalMesh,
    /// \u{2eb6}: '⺶'
    CjkRadicalSheep,
    /// \u{2eb7}: '⺷'
    CjkRadicalRam,
    /// \u{2eb8}: '⺸'
    CjkRadicalEwe,
    /// \u{2eb9}: '⺹'
    CjkRadicalOld,
    /// \u{2eba}: '⺺'
    CjkRadicalBrushOne,
    /// \u{2ebb}: '⺻'
    CjkRadicalBrushTwo,
    /// \u{2ebc}: '⺼'
    CjkRadicalMeat,
    /// \u{2ebd}: '⺽'
    CjkRadicalMortar,
    /// \u{2ebe}: '⺾'
    CjkRadicalGrassOne,
    /// \u{2ebf}: '⺿'
    CjkRadicalGrassTwo,
    /// \u{2ec0}: '⻀'
    CjkRadicalGrassThree,
    /// \u{2ec1}: '⻁'
    CjkRadicalTiger,
    /// \u{2ec2}: '⻂'
    CjkRadicalClothes,
    /// \u{2ec3}: '⻃'
    CjkRadicalWestOne,
    /// \u{2ec4}: '⻄'
    CjkRadicalWestTwo,
    /// \u{2ec5}: '⻅'
    CjkRadicalCDashSimplifiedSee,
    /// \u{2ec6}: '⻆'
    CjkRadicalSimplifiedHorn,
    /// \u{2ec7}: '⻇'
    CjkRadicalHorn,
    /// \u{2ec8}: '⻈'
    CjkRadicalCDashSimplifiedSpeech,
    /// \u{2ec9}: '⻉'
    CjkRadicalCDashSimplifiedShell,
    /// \u{2eca}: '⻊'
    CjkRadicalFoot,
    /// \u{2ecb}: '⻋'
    CjkRadicalCDashSimplifiedCart,
    /// \u{2ecc}: '⻌'
    CjkRadicalSimplifiedWalk,
    /// \u{2ecd}: '⻍'
    CjkRadicalWalkOne,
    /// \u{2ece}: '⻎'
    CjkRadicalWalkTwo,
    /// \u{2ecf}: '⻏'
    CjkRadicalCity,
    /// \u{2ed0}: '⻐'
    CjkRadicalCDashSimplifiedGold,
    /// \u{2ed1}: '⻑'
    CjkRadicalLongOne,
    /// \u{2ed2}: '⻒'
    CjkRadicalLongTwo,
    /// \u{2ed3}: '⻓'
    CjkRadicalCDashSimplifiedLong,
    /// \u{2ed4}: '⻔'
    CjkRadicalCDashSimplifiedGate,
    /// \u{2ed5}: '⻕'
    CjkRadicalMoundOne,
    /// \u{2ed6}: '⻖'
    CjkRadicalMoundTwo,
    /// \u{2ed7}: '⻗'
    CjkRadicalRain,
    /// \u{2ed8}: '⻘'
    CjkRadicalBlue,
    /// \u{2ed9}: '⻙'
    CjkRadicalCDashSimplifiedTannedLeather,
    /// \u{2eda}: '⻚'
    CjkRadicalCDashSimplifiedLeaf,
    /// \u{2edb}: '⻛'
    CjkRadicalCDashSimplifiedWind,
    /// \u{2edc}: '⻜'
    CjkRadicalCDashSimplifiedFly,
    /// \u{2edd}: '⻝'
    CjkRadicalEatOne,
    /// \u{2ede}: '⻞'
    CjkRadicalEatTwo,
    /// \u{2edf}: '⻟'
    CjkRadicalEatThree,
    /// \u{2ee0}: '⻠'
    CjkRadicalCDashSimplifiedEat,
    /// \u{2ee1}: '⻡'
    CjkRadicalHead,
    /// \u{2ee2}: '⻢'
    CjkRadicalCDashSimplifiedHorse,
    /// \u{2ee3}: '⻣'
    CjkRadicalBone,
    /// \u{2ee4}: '⻤'
    CjkRadicalGhost,
    /// \u{2ee5}: '⻥'
    CjkRadicalCDashSimplifiedFish,
    /// \u{2ee6}: '⻦'
    CjkRadicalCDashSimplifiedBird,
    /// \u{2ee7}: '⻧'
    CjkRadicalCDashSimplifiedSalt,
    /// \u{2ee8}: '⻨'
    CjkRadicalSimplifiedWheat,
    /// \u{2ee9}: '⻩'
    CjkRadicalSimplifiedYellow,
    /// \u{2eea}: '⻪'
    CjkRadicalCDashSimplifiedFrog,
    /// \u{2eeb}: '⻫'
    CjkRadicalJDashSimplifiedEven,
    /// \u{2eec}: '⻬'
    CjkRadicalCDashSimplifiedEven,
    /// \u{2eed}: '⻭'
    CjkRadicalJDashSimplifiedTooth,
    /// \u{2eee}: '⻮'
    CjkRadicalCDashSimplifiedTooth,
    /// \u{2eef}: '⻯'
    CjkRadicalJDashSimplifiedDragon,
    /// \u{2ef0}: '⻰'
    CjkRadicalCDashSimplifiedDragon,
    /// \u{2ef1}: '⻱'
    CjkRadicalTurtle,
    /// \u{2ef2}: '⻲'
    CjkRadicalJDashSimplifiedTurtle,
    /// \u{2ef3}: '⻳'
    CjkRadicalCDashSimplifiedTurtle,
}

impl Into<char> for CJKRadicalsSupplement {
    fn into(self) -> char {
        use constants::*;
        match self {
            CJKRadicalsSupplement::CjkRadicalRepeat => CJK_RADICAL_REPEAT,
            CJKRadicalsSupplement::CjkRadicalCliff => CJK_RADICAL_CLIFF,
            CJKRadicalsSupplement::CjkRadicalSecondOne => CJK_RADICAL_SECOND_ONE,
            CJKRadicalsSupplement::CjkRadicalSecondTwo => CJK_RADICAL_SECOND_TWO,
            CJKRadicalsSupplement::CjkRadicalSecondThree => CJK_RADICAL_SECOND_THREE,
            CJKRadicalsSupplement::CjkRadicalPerson => CJK_RADICAL_PERSON,
            CJKRadicalsSupplement::CjkRadicalBox => CJK_RADICAL_BOX,
            CJKRadicalsSupplement::CjkRadicalTable => CJK_RADICAL_TABLE,
            CJKRadicalsSupplement::CjkRadicalKnifeOne => CJK_RADICAL_KNIFE_ONE,
            CJKRadicalsSupplement::CjkRadicalKnifeTwo => CJK_RADICAL_KNIFE_TWO,
            CJKRadicalsSupplement::CjkRadicalDivination => CJK_RADICAL_DIVINATION,
            CJKRadicalsSupplement::CjkRadicalSeal => CJK_RADICAL_SEAL,
            CJKRadicalsSupplement::CjkRadicalSmallOne => CJK_RADICAL_SMALL_ONE,
            CJKRadicalsSupplement::CjkRadicalSmallTwo => CJK_RADICAL_SMALL_TWO,
            CJKRadicalsSupplement::CjkRadicalLameOne => CJK_RADICAL_LAME_ONE,
            CJKRadicalsSupplement::CjkRadicalLameTwo => CJK_RADICAL_LAME_TWO,
            CJKRadicalsSupplement::CjkRadicalLameThree => CJK_RADICAL_LAME_THREE,
            CJKRadicalsSupplement::CjkRadicalLameFour => CJK_RADICAL_LAME_FOUR,
            CJKRadicalsSupplement::CjkRadicalSnake => CJK_RADICAL_SNAKE,
            CJKRadicalsSupplement::CjkRadicalThread => CJK_RADICAL_THREAD,
            CJKRadicalsSupplement::CjkRadicalSnoutOne => CJK_RADICAL_SNOUT_ONE,
            CJKRadicalsSupplement::CjkRadicalSnoutTwo => CJK_RADICAL_SNOUT_TWO,
            CJKRadicalsSupplement::CjkRadicalHeartOne => CJK_RADICAL_HEART_ONE,
            CJKRadicalsSupplement::CjkRadicalHeartTwo => CJK_RADICAL_HEART_TWO,
            CJKRadicalsSupplement::CjkRadicalHand => CJK_RADICAL_HAND,
            CJKRadicalsSupplement::CjkRadicalRap => CJK_RADICAL_RAP,
            CJKRadicalsSupplement::CjkRadicalChoke => CJK_RADICAL_CHOKE,
            CJKRadicalsSupplement::CjkRadicalSun => CJK_RADICAL_SUN,
            CJKRadicalsSupplement::CjkRadicalMoon => CJK_RADICAL_MOON,
            CJKRadicalsSupplement::CjkRadicalDeath => CJK_RADICAL_DEATH,
            CJKRadicalsSupplement::CjkRadicalMother => CJK_RADICAL_MOTHER,
            CJKRadicalsSupplement::CjkRadicalCivilian => CJK_RADICAL_CIVILIAN,
            CJKRadicalsSupplement::CjkRadicalWaterOne => CJK_RADICAL_WATER_ONE,
            CJKRadicalsSupplement::CjkRadicalWaterTwo => CJK_RADICAL_WATER_TWO,
            CJKRadicalsSupplement::CjkRadicalFire => CJK_RADICAL_FIRE,
            CJKRadicalsSupplement::CjkRadicalPawOne => CJK_RADICAL_PAW_ONE,
            CJKRadicalsSupplement::CjkRadicalPawTwo => CJK_RADICAL_PAW_TWO,
            CJKRadicalsSupplement::CjkRadicalSimplifiedHalfTreeTrunk => CJK_RADICAL_SIMPLIFIED_HALF_TREE_TRUNK,
            CJKRadicalsSupplement::CjkRadicalCow => CJK_RADICAL_COW,
            CJKRadicalsSupplement::CjkRadicalDog => CJK_RADICAL_DOG,
            CJKRadicalsSupplement::CjkRadicalJade => CJK_RADICAL_JADE,
            CJKRadicalsSupplement::CjkRadicalBoltOfCloth => CJK_RADICAL_BOLT_OF_CLOTH,
            CJKRadicalsSupplement::CjkRadicalEye => CJK_RADICAL_EYE,
            CJKRadicalsSupplement::CjkRadicalSpiritOne => CJK_RADICAL_SPIRIT_ONE,
            CJKRadicalsSupplement::CjkRadicalSpiritTwo => CJK_RADICAL_SPIRIT_TWO,
            CJKRadicalsSupplement::CjkRadicalBamboo => CJK_RADICAL_BAMBOO,
            CJKRadicalsSupplement::CjkRadicalSilk => CJK_RADICAL_SILK,
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedSilk => CJK_RADICAL_C_DASH_SIMPLIFIED_SILK,
            CJKRadicalsSupplement::CjkRadicalNetOne => CJK_RADICAL_NET_ONE,
            CJKRadicalsSupplement::CjkRadicalNetTwo => CJK_RADICAL_NET_TWO,
            CJKRadicalsSupplement::CjkRadicalNetThree => CJK_RADICAL_NET_THREE,
            CJKRadicalsSupplement::CjkRadicalNetFour => CJK_RADICAL_NET_FOUR,
            CJKRadicalsSupplement::CjkRadicalMesh => CJK_RADICAL_MESH,
            CJKRadicalsSupplement::CjkRadicalSheep => CJK_RADICAL_SHEEP,
            CJKRadicalsSupplement::CjkRadicalRam => CJK_RADICAL_RAM,
            CJKRadicalsSupplement::CjkRadicalEwe => CJK_RADICAL_EWE,
            CJKRadicalsSupplement::CjkRadicalOld => CJK_RADICAL_OLD,
            CJKRadicalsSupplement::CjkRadicalBrushOne => CJK_RADICAL_BRUSH_ONE,
            CJKRadicalsSupplement::CjkRadicalBrushTwo => CJK_RADICAL_BRUSH_TWO,
            CJKRadicalsSupplement::CjkRadicalMeat => CJK_RADICAL_MEAT,
            CJKRadicalsSupplement::CjkRadicalMortar => CJK_RADICAL_MORTAR,
            CJKRadicalsSupplement::CjkRadicalGrassOne => CJK_RADICAL_GRASS_ONE,
            CJKRadicalsSupplement::CjkRadicalGrassTwo => CJK_RADICAL_GRASS_TWO,
            CJKRadicalsSupplement::CjkRadicalGrassThree => CJK_RADICAL_GRASS_THREE,
            CJKRadicalsSupplement::CjkRadicalTiger => CJK_RADICAL_TIGER,
            CJKRadicalsSupplement::CjkRadicalClothes => CJK_RADICAL_CLOTHES,
            CJKRadicalsSupplement::CjkRadicalWestOne => CJK_RADICAL_WEST_ONE,
            CJKRadicalsSupplement::CjkRadicalWestTwo => CJK_RADICAL_WEST_TWO,
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedSee => CJK_RADICAL_C_DASH_SIMPLIFIED_SEE,
            CJKRadicalsSupplement::CjkRadicalSimplifiedHorn => CJK_RADICAL_SIMPLIFIED_HORN,
            CJKRadicalsSupplement::CjkRadicalHorn => CJK_RADICAL_HORN,
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedSpeech => CJK_RADICAL_C_DASH_SIMPLIFIED_SPEECH,
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedShell => CJK_RADICAL_C_DASH_SIMPLIFIED_SHELL,
            CJKRadicalsSupplement::CjkRadicalFoot => CJK_RADICAL_FOOT,
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedCart => CJK_RADICAL_C_DASH_SIMPLIFIED_CART,
            CJKRadicalsSupplement::CjkRadicalSimplifiedWalk => CJK_RADICAL_SIMPLIFIED_WALK,
            CJKRadicalsSupplement::CjkRadicalWalkOne => CJK_RADICAL_WALK_ONE,
            CJKRadicalsSupplement::CjkRadicalWalkTwo => CJK_RADICAL_WALK_TWO,
            CJKRadicalsSupplement::CjkRadicalCity => CJK_RADICAL_CITY,
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedGold => CJK_RADICAL_C_DASH_SIMPLIFIED_GOLD,
            CJKRadicalsSupplement::CjkRadicalLongOne => CJK_RADICAL_LONG_ONE,
            CJKRadicalsSupplement::CjkRadicalLongTwo => CJK_RADICAL_LONG_TWO,
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedLong => CJK_RADICAL_C_DASH_SIMPLIFIED_LONG,
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedGate => CJK_RADICAL_C_DASH_SIMPLIFIED_GATE,
            CJKRadicalsSupplement::CjkRadicalMoundOne => CJK_RADICAL_MOUND_ONE,
            CJKRadicalsSupplement::CjkRadicalMoundTwo => CJK_RADICAL_MOUND_TWO,
            CJKRadicalsSupplement::CjkRadicalRain => CJK_RADICAL_RAIN,
            CJKRadicalsSupplement::CjkRadicalBlue => CJK_RADICAL_BLUE,
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedTannedLeather => CJK_RADICAL_C_DASH_SIMPLIFIED_TANNED_LEATHER,
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedLeaf => CJK_RADICAL_C_DASH_SIMPLIFIED_LEAF,
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedWind => CJK_RADICAL_C_DASH_SIMPLIFIED_WIND,
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedFly => CJK_RADICAL_C_DASH_SIMPLIFIED_FLY,
            CJKRadicalsSupplement::CjkRadicalEatOne => CJK_RADICAL_EAT_ONE,
            CJKRadicalsSupplement::CjkRadicalEatTwo => CJK_RADICAL_EAT_TWO,
            CJKRadicalsSupplement::CjkRadicalEatThree => CJK_RADICAL_EAT_THREE,
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedEat => CJK_RADICAL_C_DASH_SIMPLIFIED_EAT,
            CJKRadicalsSupplement::CjkRadicalHead => CJK_RADICAL_HEAD,
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedHorse => CJK_RADICAL_C_DASH_SIMPLIFIED_HORSE,
            CJKRadicalsSupplement::CjkRadicalBone => CJK_RADICAL_BONE,
            CJKRadicalsSupplement::CjkRadicalGhost => CJK_RADICAL_GHOST,
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedFish => CJK_RADICAL_C_DASH_SIMPLIFIED_FISH,
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedBird => CJK_RADICAL_C_DASH_SIMPLIFIED_BIRD,
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedSalt => CJK_RADICAL_C_DASH_SIMPLIFIED_SALT,
            CJKRadicalsSupplement::CjkRadicalSimplifiedWheat => CJK_RADICAL_SIMPLIFIED_WHEAT,
            CJKRadicalsSupplement::CjkRadicalSimplifiedYellow => CJK_RADICAL_SIMPLIFIED_YELLOW,
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedFrog => CJK_RADICAL_C_DASH_SIMPLIFIED_FROG,
            CJKRadicalsSupplement::CjkRadicalJDashSimplifiedEven => CJK_RADICAL_J_DASH_SIMPLIFIED_EVEN,
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedEven => CJK_RADICAL_C_DASH_SIMPLIFIED_EVEN,
            CJKRadicalsSupplement::CjkRadicalJDashSimplifiedTooth => CJK_RADICAL_J_DASH_SIMPLIFIED_TOOTH,
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedTooth => CJK_RADICAL_C_DASH_SIMPLIFIED_TOOTH,
            CJKRadicalsSupplement::CjkRadicalJDashSimplifiedDragon => CJK_RADICAL_J_DASH_SIMPLIFIED_DRAGON,
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedDragon => CJK_RADICAL_C_DASH_SIMPLIFIED_DRAGON,
            CJKRadicalsSupplement::CjkRadicalTurtle => CJK_RADICAL_TURTLE,
            CJKRadicalsSupplement::CjkRadicalJDashSimplifiedTurtle => CJK_RADICAL_J_DASH_SIMPLIFIED_TURTLE,
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedTurtle => CJK_RADICAL_C_DASH_SIMPLIFIED_TURTLE,
        }
    }
}

impl std::convert::TryFrom<char> for CJKRadicalsSupplement {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            CJK_RADICAL_REPEAT => Ok(CJKRadicalsSupplement::CjkRadicalRepeat),
            CJK_RADICAL_CLIFF => Ok(CJKRadicalsSupplement::CjkRadicalCliff),
            CJK_RADICAL_SECOND_ONE => Ok(CJKRadicalsSupplement::CjkRadicalSecondOne),
            CJK_RADICAL_SECOND_TWO => Ok(CJKRadicalsSupplement::CjkRadicalSecondTwo),
            CJK_RADICAL_SECOND_THREE => Ok(CJKRadicalsSupplement::CjkRadicalSecondThree),
            CJK_RADICAL_PERSON => Ok(CJKRadicalsSupplement::CjkRadicalPerson),
            CJK_RADICAL_BOX => Ok(CJKRadicalsSupplement::CjkRadicalBox),
            CJK_RADICAL_TABLE => Ok(CJKRadicalsSupplement::CjkRadicalTable),
            CJK_RADICAL_KNIFE_ONE => Ok(CJKRadicalsSupplement::CjkRadicalKnifeOne),
            CJK_RADICAL_KNIFE_TWO => Ok(CJKRadicalsSupplement::CjkRadicalKnifeTwo),
            CJK_RADICAL_DIVINATION => Ok(CJKRadicalsSupplement::CjkRadicalDivination),
            CJK_RADICAL_SEAL => Ok(CJKRadicalsSupplement::CjkRadicalSeal),
            CJK_RADICAL_SMALL_ONE => Ok(CJKRadicalsSupplement::CjkRadicalSmallOne),
            CJK_RADICAL_SMALL_TWO => Ok(CJKRadicalsSupplement::CjkRadicalSmallTwo),
            CJK_RADICAL_LAME_ONE => Ok(CJKRadicalsSupplement::CjkRadicalLameOne),
            CJK_RADICAL_LAME_TWO => Ok(CJKRadicalsSupplement::CjkRadicalLameTwo),
            CJK_RADICAL_LAME_THREE => Ok(CJKRadicalsSupplement::CjkRadicalLameThree),
            CJK_RADICAL_LAME_FOUR => Ok(CJKRadicalsSupplement::CjkRadicalLameFour),
            CJK_RADICAL_SNAKE => Ok(CJKRadicalsSupplement::CjkRadicalSnake),
            CJK_RADICAL_THREAD => Ok(CJKRadicalsSupplement::CjkRadicalThread),
            CJK_RADICAL_SNOUT_ONE => Ok(CJKRadicalsSupplement::CjkRadicalSnoutOne),
            CJK_RADICAL_SNOUT_TWO => Ok(CJKRadicalsSupplement::CjkRadicalSnoutTwo),
            CJK_RADICAL_HEART_ONE => Ok(CJKRadicalsSupplement::CjkRadicalHeartOne),
            CJK_RADICAL_HEART_TWO => Ok(CJKRadicalsSupplement::CjkRadicalHeartTwo),
            CJK_RADICAL_HAND => Ok(CJKRadicalsSupplement::CjkRadicalHand),
            CJK_RADICAL_RAP => Ok(CJKRadicalsSupplement::CjkRadicalRap),
            CJK_RADICAL_CHOKE => Ok(CJKRadicalsSupplement::CjkRadicalChoke),
            CJK_RADICAL_SUN => Ok(CJKRadicalsSupplement::CjkRadicalSun),
            CJK_RADICAL_MOON => Ok(CJKRadicalsSupplement::CjkRadicalMoon),
            CJK_RADICAL_DEATH => Ok(CJKRadicalsSupplement::CjkRadicalDeath),
            CJK_RADICAL_MOTHER => Ok(CJKRadicalsSupplement::CjkRadicalMother),
            CJK_RADICAL_CIVILIAN => Ok(CJKRadicalsSupplement::CjkRadicalCivilian),
            CJK_RADICAL_WATER_ONE => Ok(CJKRadicalsSupplement::CjkRadicalWaterOne),
            CJK_RADICAL_WATER_TWO => Ok(CJKRadicalsSupplement::CjkRadicalWaterTwo),
            CJK_RADICAL_FIRE => Ok(CJKRadicalsSupplement::CjkRadicalFire),
            CJK_RADICAL_PAW_ONE => Ok(CJKRadicalsSupplement::CjkRadicalPawOne),
            CJK_RADICAL_PAW_TWO => Ok(CJKRadicalsSupplement::CjkRadicalPawTwo),
            CJK_RADICAL_SIMPLIFIED_HALF_TREE_TRUNK => Ok(CJKRadicalsSupplement::CjkRadicalSimplifiedHalfTreeTrunk),
            CJK_RADICAL_COW => Ok(CJKRadicalsSupplement::CjkRadicalCow),
            CJK_RADICAL_DOG => Ok(CJKRadicalsSupplement::CjkRadicalDog),
            CJK_RADICAL_JADE => Ok(CJKRadicalsSupplement::CjkRadicalJade),
            CJK_RADICAL_BOLT_OF_CLOTH => Ok(CJKRadicalsSupplement::CjkRadicalBoltOfCloth),
            CJK_RADICAL_EYE => Ok(CJKRadicalsSupplement::CjkRadicalEye),
            CJK_RADICAL_SPIRIT_ONE => Ok(CJKRadicalsSupplement::CjkRadicalSpiritOne),
            CJK_RADICAL_SPIRIT_TWO => Ok(CJKRadicalsSupplement::CjkRadicalSpiritTwo),
            CJK_RADICAL_BAMBOO => Ok(CJKRadicalsSupplement::CjkRadicalBamboo),
            CJK_RADICAL_SILK => Ok(CJKRadicalsSupplement::CjkRadicalSilk),
            CJK_RADICAL_C_DASH_SIMPLIFIED_SILK => Ok(CJKRadicalsSupplement::CjkRadicalCDashSimplifiedSilk),
            CJK_RADICAL_NET_ONE => Ok(CJKRadicalsSupplement::CjkRadicalNetOne),
            CJK_RADICAL_NET_TWO => Ok(CJKRadicalsSupplement::CjkRadicalNetTwo),
            CJK_RADICAL_NET_THREE => Ok(CJKRadicalsSupplement::CjkRadicalNetThree),
            CJK_RADICAL_NET_FOUR => Ok(CJKRadicalsSupplement::CjkRadicalNetFour),
            CJK_RADICAL_MESH => Ok(CJKRadicalsSupplement::CjkRadicalMesh),
            CJK_RADICAL_SHEEP => Ok(CJKRadicalsSupplement::CjkRadicalSheep),
            CJK_RADICAL_RAM => Ok(CJKRadicalsSupplement::CjkRadicalRam),
            CJK_RADICAL_EWE => Ok(CJKRadicalsSupplement::CjkRadicalEwe),
            CJK_RADICAL_OLD => Ok(CJKRadicalsSupplement::CjkRadicalOld),
            CJK_RADICAL_BRUSH_ONE => Ok(CJKRadicalsSupplement::CjkRadicalBrushOne),
            CJK_RADICAL_BRUSH_TWO => Ok(CJKRadicalsSupplement::CjkRadicalBrushTwo),
            CJK_RADICAL_MEAT => Ok(CJKRadicalsSupplement::CjkRadicalMeat),
            CJK_RADICAL_MORTAR => Ok(CJKRadicalsSupplement::CjkRadicalMortar),
            CJK_RADICAL_GRASS_ONE => Ok(CJKRadicalsSupplement::CjkRadicalGrassOne),
            CJK_RADICAL_GRASS_TWO => Ok(CJKRadicalsSupplement::CjkRadicalGrassTwo),
            CJK_RADICAL_GRASS_THREE => Ok(CJKRadicalsSupplement::CjkRadicalGrassThree),
            CJK_RADICAL_TIGER => Ok(CJKRadicalsSupplement::CjkRadicalTiger),
            CJK_RADICAL_CLOTHES => Ok(CJKRadicalsSupplement::CjkRadicalClothes),
            CJK_RADICAL_WEST_ONE => Ok(CJKRadicalsSupplement::CjkRadicalWestOne),
            CJK_RADICAL_WEST_TWO => Ok(CJKRadicalsSupplement::CjkRadicalWestTwo),
            CJK_RADICAL_C_DASH_SIMPLIFIED_SEE => Ok(CJKRadicalsSupplement::CjkRadicalCDashSimplifiedSee),
            CJK_RADICAL_SIMPLIFIED_HORN => Ok(CJKRadicalsSupplement::CjkRadicalSimplifiedHorn),
            CJK_RADICAL_HORN => Ok(CJKRadicalsSupplement::CjkRadicalHorn),
            CJK_RADICAL_C_DASH_SIMPLIFIED_SPEECH => Ok(CJKRadicalsSupplement::CjkRadicalCDashSimplifiedSpeech),
            CJK_RADICAL_C_DASH_SIMPLIFIED_SHELL => Ok(CJKRadicalsSupplement::CjkRadicalCDashSimplifiedShell),
            CJK_RADICAL_FOOT => Ok(CJKRadicalsSupplement::CjkRadicalFoot),
            CJK_RADICAL_C_DASH_SIMPLIFIED_CART => Ok(CJKRadicalsSupplement::CjkRadicalCDashSimplifiedCart),
            CJK_RADICAL_SIMPLIFIED_WALK => Ok(CJKRadicalsSupplement::CjkRadicalSimplifiedWalk),
            CJK_RADICAL_WALK_ONE => Ok(CJKRadicalsSupplement::CjkRadicalWalkOne),
            CJK_RADICAL_WALK_TWO => Ok(CJKRadicalsSupplement::CjkRadicalWalkTwo),
            CJK_RADICAL_CITY => Ok(CJKRadicalsSupplement::CjkRadicalCity),
            CJK_RADICAL_C_DASH_SIMPLIFIED_GOLD => Ok(CJKRadicalsSupplement::CjkRadicalCDashSimplifiedGold),
            CJK_RADICAL_LONG_ONE => Ok(CJKRadicalsSupplement::CjkRadicalLongOne),
            CJK_RADICAL_LONG_TWO => Ok(CJKRadicalsSupplement::CjkRadicalLongTwo),
            CJK_RADICAL_C_DASH_SIMPLIFIED_LONG => Ok(CJKRadicalsSupplement::CjkRadicalCDashSimplifiedLong),
            CJK_RADICAL_C_DASH_SIMPLIFIED_GATE => Ok(CJKRadicalsSupplement::CjkRadicalCDashSimplifiedGate),
            CJK_RADICAL_MOUND_ONE => Ok(CJKRadicalsSupplement::CjkRadicalMoundOne),
            CJK_RADICAL_MOUND_TWO => Ok(CJKRadicalsSupplement::CjkRadicalMoundTwo),
            CJK_RADICAL_RAIN => Ok(CJKRadicalsSupplement::CjkRadicalRain),
            CJK_RADICAL_BLUE => Ok(CJKRadicalsSupplement::CjkRadicalBlue),
            CJK_RADICAL_C_DASH_SIMPLIFIED_TANNED_LEATHER => Ok(CJKRadicalsSupplement::CjkRadicalCDashSimplifiedTannedLeather),
            CJK_RADICAL_C_DASH_SIMPLIFIED_LEAF => Ok(CJKRadicalsSupplement::CjkRadicalCDashSimplifiedLeaf),
            CJK_RADICAL_C_DASH_SIMPLIFIED_WIND => Ok(CJKRadicalsSupplement::CjkRadicalCDashSimplifiedWind),
            CJK_RADICAL_C_DASH_SIMPLIFIED_FLY => Ok(CJKRadicalsSupplement::CjkRadicalCDashSimplifiedFly),
            CJK_RADICAL_EAT_ONE => Ok(CJKRadicalsSupplement::CjkRadicalEatOne),
            CJK_RADICAL_EAT_TWO => Ok(CJKRadicalsSupplement::CjkRadicalEatTwo),
            CJK_RADICAL_EAT_THREE => Ok(CJKRadicalsSupplement::CjkRadicalEatThree),
            CJK_RADICAL_C_DASH_SIMPLIFIED_EAT => Ok(CJKRadicalsSupplement::CjkRadicalCDashSimplifiedEat),
            CJK_RADICAL_HEAD => Ok(CJKRadicalsSupplement::CjkRadicalHead),
            CJK_RADICAL_C_DASH_SIMPLIFIED_HORSE => Ok(CJKRadicalsSupplement::CjkRadicalCDashSimplifiedHorse),
            CJK_RADICAL_BONE => Ok(CJKRadicalsSupplement::CjkRadicalBone),
            CJK_RADICAL_GHOST => Ok(CJKRadicalsSupplement::CjkRadicalGhost),
            CJK_RADICAL_C_DASH_SIMPLIFIED_FISH => Ok(CJKRadicalsSupplement::CjkRadicalCDashSimplifiedFish),
            CJK_RADICAL_C_DASH_SIMPLIFIED_BIRD => Ok(CJKRadicalsSupplement::CjkRadicalCDashSimplifiedBird),
            CJK_RADICAL_C_DASH_SIMPLIFIED_SALT => Ok(CJKRadicalsSupplement::CjkRadicalCDashSimplifiedSalt),
            CJK_RADICAL_SIMPLIFIED_WHEAT => Ok(CJKRadicalsSupplement::CjkRadicalSimplifiedWheat),
            CJK_RADICAL_SIMPLIFIED_YELLOW => Ok(CJKRadicalsSupplement::CjkRadicalSimplifiedYellow),
            CJK_RADICAL_C_DASH_SIMPLIFIED_FROG => Ok(CJKRadicalsSupplement::CjkRadicalCDashSimplifiedFrog),
            CJK_RADICAL_J_DASH_SIMPLIFIED_EVEN => Ok(CJKRadicalsSupplement::CjkRadicalJDashSimplifiedEven),
            CJK_RADICAL_C_DASH_SIMPLIFIED_EVEN => Ok(CJKRadicalsSupplement::CjkRadicalCDashSimplifiedEven),
            CJK_RADICAL_J_DASH_SIMPLIFIED_TOOTH => Ok(CJKRadicalsSupplement::CjkRadicalJDashSimplifiedTooth),
            CJK_RADICAL_C_DASH_SIMPLIFIED_TOOTH => Ok(CJKRadicalsSupplement::CjkRadicalCDashSimplifiedTooth),
            CJK_RADICAL_J_DASH_SIMPLIFIED_DRAGON => Ok(CJKRadicalsSupplement::CjkRadicalJDashSimplifiedDragon),
            CJK_RADICAL_C_DASH_SIMPLIFIED_DRAGON => Ok(CJKRadicalsSupplement::CjkRadicalCDashSimplifiedDragon),
            CJK_RADICAL_TURTLE => Ok(CJKRadicalsSupplement::CjkRadicalTurtle),
            CJK_RADICAL_J_DASH_SIMPLIFIED_TURTLE => Ok(CJKRadicalsSupplement::CjkRadicalJDashSimplifiedTurtle),
            CJK_RADICAL_C_DASH_SIMPLIFIED_TURTLE => Ok(CJKRadicalsSupplement::CjkRadicalCDashSimplifiedTurtle),
            _ => Err(()),
        }
    }
}

impl Into<u32> for CJKRadicalsSupplement {
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

impl std::convert::TryFrom<u32> for CJKRadicalsSupplement {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for CJKRadicalsSupplement {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl CJKRadicalsSupplement {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        CJKRadicalsSupplement::CjkRadicalRepeat
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            CJKRadicalsSupplement::CjkRadicalRepeat => "cjk radical repeat",
            CJKRadicalsSupplement::CjkRadicalCliff => "cjk radical cliff",
            CJKRadicalsSupplement::CjkRadicalSecondOne => "cjk radical second one",
            CJKRadicalsSupplement::CjkRadicalSecondTwo => "cjk radical second two",
            CJKRadicalsSupplement::CjkRadicalSecondThree => "cjk radical second three",
            CJKRadicalsSupplement::CjkRadicalPerson => "cjk radical person",
            CJKRadicalsSupplement::CjkRadicalBox => "cjk radical box",
            CJKRadicalsSupplement::CjkRadicalTable => "cjk radical table",
            CJKRadicalsSupplement::CjkRadicalKnifeOne => "cjk radical knife one",
            CJKRadicalsSupplement::CjkRadicalKnifeTwo => "cjk radical knife two",
            CJKRadicalsSupplement::CjkRadicalDivination => "cjk radical divination",
            CJKRadicalsSupplement::CjkRadicalSeal => "cjk radical seal",
            CJKRadicalsSupplement::CjkRadicalSmallOne => "cjk radical small one",
            CJKRadicalsSupplement::CjkRadicalSmallTwo => "cjk radical small two",
            CJKRadicalsSupplement::CjkRadicalLameOne => "cjk radical lame one",
            CJKRadicalsSupplement::CjkRadicalLameTwo => "cjk radical lame two",
            CJKRadicalsSupplement::CjkRadicalLameThree => "cjk radical lame three",
            CJKRadicalsSupplement::CjkRadicalLameFour => "cjk radical lame four",
            CJKRadicalsSupplement::CjkRadicalSnake => "cjk radical snake",
            CJKRadicalsSupplement::CjkRadicalThread => "cjk radical thread",
            CJKRadicalsSupplement::CjkRadicalSnoutOne => "cjk radical snout one",
            CJKRadicalsSupplement::CjkRadicalSnoutTwo => "cjk radical snout two",
            CJKRadicalsSupplement::CjkRadicalHeartOne => "cjk radical heart one",
            CJKRadicalsSupplement::CjkRadicalHeartTwo => "cjk radical heart two",
            CJKRadicalsSupplement::CjkRadicalHand => "cjk radical hand",
            CJKRadicalsSupplement::CjkRadicalRap => "cjk radical rap",
            CJKRadicalsSupplement::CjkRadicalChoke => "cjk radical choke",
            CJKRadicalsSupplement::CjkRadicalSun => "cjk radical sun",
            CJKRadicalsSupplement::CjkRadicalMoon => "cjk radical moon",
            CJKRadicalsSupplement::CjkRadicalDeath => "cjk radical death",
            CJKRadicalsSupplement::CjkRadicalMother => "cjk radical mother",
            CJKRadicalsSupplement::CjkRadicalCivilian => "cjk radical civilian",
            CJKRadicalsSupplement::CjkRadicalWaterOne => "cjk radical water one",
            CJKRadicalsSupplement::CjkRadicalWaterTwo => "cjk radical water two",
            CJKRadicalsSupplement::CjkRadicalFire => "cjk radical fire",
            CJKRadicalsSupplement::CjkRadicalPawOne => "cjk radical paw one",
            CJKRadicalsSupplement::CjkRadicalPawTwo => "cjk radical paw two",
            CJKRadicalsSupplement::CjkRadicalSimplifiedHalfTreeTrunk => "cjk radical simplified half tree trunk",
            CJKRadicalsSupplement::CjkRadicalCow => "cjk radical cow",
            CJKRadicalsSupplement::CjkRadicalDog => "cjk radical dog",
            CJKRadicalsSupplement::CjkRadicalJade => "cjk radical jade",
            CJKRadicalsSupplement::CjkRadicalBoltOfCloth => "cjk radical bolt of cloth",
            CJKRadicalsSupplement::CjkRadicalEye => "cjk radical eye",
            CJKRadicalsSupplement::CjkRadicalSpiritOne => "cjk radical spirit one",
            CJKRadicalsSupplement::CjkRadicalSpiritTwo => "cjk radical spirit two",
            CJKRadicalsSupplement::CjkRadicalBamboo => "cjk radical bamboo",
            CJKRadicalsSupplement::CjkRadicalSilk => "cjk radical silk",
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedSilk => "cjk radical c-simplified silk",
            CJKRadicalsSupplement::CjkRadicalNetOne => "cjk radical net one",
            CJKRadicalsSupplement::CjkRadicalNetTwo => "cjk radical net two",
            CJKRadicalsSupplement::CjkRadicalNetThree => "cjk radical net three",
            CJKRadicalsSupplement::CjkRadicalNetFour => "cjk radical net four",
            CJKRadicalsSupplement::CjkRadicalMesh => "cjk radical mesh",
            CJKRadicalsSupplement::CjkRadicalSheep => "cjk radical sheep",
            CJKRadicalsSupplement::CjkRadicalRam => "cjk radical ram",
            CJKRadicalsSupplement::CjkRadicalEwe => "cjk radical ewe",
            CJKRadicalsSupplement::CjkRadicalOld => "cjk radical old",
            CJKRadicalsSupplement::CjkRadicalBrushOne => "cjk radical brush one",
            CJKRadicalsSupplement::CjkRadicalBrushTwo => "cjk radical brush two",
            CJKRadicalsSupplement::CjkRadicalMeat => "cjk radical meat",
            CJKRadicalsSupplement::CjkRadicalMortar => "cjk radical mortar",
            CJKRadicalsSupplement::CjkRadicalGrassOne => "cjk radical grass one",
            CJKRadicalsSupplement::CjkRadicalGrassTwo => "cjk radical grass two",
            CJKRadicalsSupplement::CjkRadicalGrassThree => "cjk radical grass three",
            CJKRadicalsSupplement::CjkRadicalTiger => "cjk radical tiger",
            CJKRadicalsSupplement::CjkRadicalClothes => "cjk radical clothes",
            CJKRadicalsSupplement::CjkRadicalWestOne => "cjk radical west one",
            CJKRadicalsSupplement::CjkRadicalWestTwo => "cjk radical west two",
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedSee => "cjk radical c-simplified see",
            CJKRadicalsSupplement::CjkRadicalSimplifiedHorn => "cjk radical simplified horn",
            CJKRadicalsSupplement::CjkRadicalHorn => "cjk radical horn",
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedSpeech => "cjk radical c-simplified speech",
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedShell => "cjk radical c-simplified shell",
            CJKRadicalsSupplement::CjkRadicalFoot => "cjk radical foot",
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedCart => "cjk radical c-simplified cart",
            CJKRadicalsSupplement::CjkRadicalSimplifiedWalk => "cjk radical simplified walk",
            CJKRadicalsSupplement::CjkRadicalWalkOne => "cjk radical walk one",
            CJKRadicalsSupplement::CjkRadicalWalkTwo => "cjk radical walk two",
            CJKRadicalsSupplement::CjkRadicalCity => "cjk radical city",
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedGold => "cjk radical c-simplified gold",
            CJKRadicalsSupplement::CjkRadicalLongOne => "cjk radical long one",
            CJKRadicalsSupplement::CjkRadicalLongTwo => "cjk radical long two",
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedLong => "cjk radical c-simplified long",
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedGate => "cjk radical c-simplified gate",
            CJKRadicalsSupplement::CjkRadicalMoundOne => "cjk radical mound one",
            CJKRadicalsSupplement::CjkRadicalMoundTwo => "cjk radical mound two",
            CJKRadicalsSupplement::CjkRadicalRain => "cjk radical rain",
            CJKRadicalsSupplement::CjkRadicalBlue => "cjk radical blue",
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedTannedLeather => "cjk radical c-simplified tanned leather",
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedLeaf => "cjk radical c-simplified leaf",
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedWind => "cjk radical c-simplified wind",
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedFly => "cjk radical c-simplified fly",
            CJKRadicalsSupplement::CjkRadicalEatOne => "cjk radical eat one",
            CJKRadicalsSupplement::CjkRadicalEatTwo => "cjk radical eat two",
            CJKRadicalsSupplement::CjkRadicalEatThree => "cjk radical eat three",
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedEat => "cjk radical c-simplified eat",
            CJKRadicalsSupplement::CjkRadicalHead => "cjk radical head",
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedHorse => "cjk radical c-simplified horse",
            CJKRadicalsSupplement::CjkRadicalBone => "cjk radical bone",
            CJKRadicalsSupplement::CjkRadicalGhost => "cjk radical ghost",
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedFish => "cjk radical c-simplified fish",
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedBird => "cjk radical c-simplified bird",
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedSalt => "cjk radical c-simplified salt",
            CJKRadicalsSupplement::CjkRadicalSimplifiedWheat => "cjk radical simplified wheat",
            CJKRadicalsSupplement::CjkRadicalSimplifiedYellow => "cjk radical simplified yellow",
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedFrog => "cjk radical c-simplified frog",
            CJKRadicalsSupplement::CjkRadicalJDashSimplifiedEven => "cjk radical j-simplified even",
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedEven => "cjk radical c-simplified even",
            CJKRadicalsSupplement::CjkRadicalJDashSimplifiedTooth => "cjk radical j-simplified tooth",
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedTooth => "cjk radical c-simplified tooth",
            CJKRadicalsSupplement::CjkRadicalJDashSimplifiedDragon => "cjk radical j-simplified dragon",
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedDragon => "cjk radical c-simplified dragon",
            CJKRadicalsSupplement::CjkRadicalTurtle => "cjk radical turtle",
            CJKRadicalsSupplement::CjkRadicalJDashSimplifiedTurtle => "cjk radical j-simplified turtle",
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedTurtle => "cjk radical c-simplified turtle",
        }
    }
}
