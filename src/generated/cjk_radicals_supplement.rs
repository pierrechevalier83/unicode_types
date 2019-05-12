
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
        match self {
            CJKRadicalsSupplement::CjkRadicalRepeat => '⺀',
            CJKRadicalsSupplement::CjkRadicalCliff => '⺁',
            CJKRadicalsSupplement::CjkRadicalSecondOne => '⺂',
            CJKRadicalsSupplement::CjkRadicalSecondTwo => '⺃',
            CJKRadicalsSupplement::CjkRadicalSecondThree => '⺄',
            CJKRadicalsSupplement::CjkRadicalPerson => '⺅',
            CJKRadicalsSupplement::CjkRadicalBox => '⺆',
            CJKRadicalsSupplement::CjkRadicalTable => '⺇',
            CJKRadicalsSupplement::CjkRadicalKnifeOne => '⺈',
            CJKRadicalsSupplement::CjkRadicalKnifeTwo => '⺉',
            CJKRadicalsSupplement::CjkRadicalDivination => '⺊',
            CJKRadicalsSupplement::CjkRadicalSeal => '⺋',
            CJKRadicalsSupplement::CjkRadicalSmallOne => '⺌',
            CJKRadicalsSupplement::CjkRadicalSmallTwo => '⺍',
            CJKRadicalsSupplement::CjkRadicalLameOne => '⺎',
            CJKRadicalsSupplement::CjkRadicalLameTwo => '⺏',
            CJKRadicalsSupplement::CjkRadicalLameThree => '⺐',
            CJKRadicalsSupplement::CjkRadicalLameFour => '⺑',
            CJKRadicalsSupplement::CjkRadicalSnake => '⺒',
            CJKRadicalsSupplement::CjkRadicalThread => '⺓',
            CJKRadicalsSupplement::CjkRadicalSnoutOne => '⺔',
            CJKRadicalsSupplement::CjkRadicalSnoutTwo => '⺕',
            CJKRadicalsSupplement::CjkRadicalHeartOne => '⺖',
            CJKRadicalsSupplement::CjkRadicalHeartTwo => '⺗',
            CJKRadicalsSupplement::CjkRadicalHand => '⺘',
            CJKRadicalsSupplement::CjkRadicalRap => '⺙',
            CJKRadicalsSupplement::CjkRadicalChoke => '⺛',
            CJKRadicalsSupplement::CjkRadicalSun => '⺜',
            CJKRadicalsSupplement::CjkRadicalMoon => '⺝',
            CJKRadicalsSupplement::CjkRadicalDeath => '⺞',
            CJKRadicalsSupplement::CjkRadicalMother => '⺟',
            CJKRadicalsSupplement::CjkRadicalCivilian => '⺠',
            CJKRadicalsSupplement::CjkRadicalWaterOne => '⺡',
            CJKRadicalsSupplement::CjkRadicalWaterTwo => '⺢',
            CJKRadicalsSupplement::CjkRadicalFire => '⺣',
            CJKRadicalsSupplement::CjkRadicalPawOne => '⺤',
            CJKRadicalsSupplement::CjkRadicalPawTwo => '⺥',
            CJKRadicalsSupplement::CjkRadicalSimplifiedHalfTreeTrunk => '⺦',
            CJKRadicalsSupplement::CjkRadicalCow => '⺧',
            CJKRadicalsSupplement::CjkRadicalDog => '⺨',
            CJKRadicalsSupplement::CjkRadicalJade => '⺩',
            CJKRadicalsSupplement::CjkRadicalBoltOfCloth => '⺪',
            CJKRadicalsSupplement::CjkRadicalEye => '⺫',
            CJKRadicalsSupplement::CjkRadicalSpiritOne => '⺬',
            CJKRadicalsSupplement::CjkRadicalSpiritTwo => '⺭',
            CJKRadicalsSupplement::CjkRadicalBamboo => '⺮',
            CJKRadicalsSupplement::CjkRadicalSilk => '⺯',
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedSilk => '⺰',
            CJKRadicalsSupplement::CjkRadicalNetOne => '⺱',
            CJKRadicalsSupplement::CjkRadicalNetTwo => '⺲',
            CJKRadicalsSupplement::CjkRadicalNetThree => '⺳',
            CJKRadicalsSupplement::CjkRadicalNetFour => '⺴',
            CJKRadicalsSupplement::CjkRadicalMesh => '⺵',
            CJKRadicalsSupplement::CjkRadicalSheep => '⺶',
            CJKRadicalsSupplement::CjkRadicalRam => '⺷',
            CJKRadicalsSupplement::CjkRadicalEwe => '⺸',
            CJKRadicalsSupplement::CjkRadicalOld => '⺹',
            CJKRadicalsSupplement::CjkRadicalBrushOne => '⺺',
            CJKRadicalsSupplement::CjkRadicalBrushTwo => '⺻',
            CJKRadicalsSupplement::CjkRadicalMeat => '⺼',
            CJKRadicalsSupplement::CjkRadicalMortar => '⺽',
            CJKRadicalsSupplement::CjkRadicalGrassOne => '⺾',
            CJKRadicalsSupplement::CjkRadicalGrassTwo => '⺿',
            CJKRadicalsSupplement::CjkRadicalGrassThree => '⻀',
            CJKRadicalsSupplement::CjkRadicalTiger => '⻁',
            CJKRadicalsSupplement::CjkRadicalClothes => '⻂',
            CJKRadicalsSupplement::CjkRadicalWestOne => '⻃',
            CJKRadicalsSupplement::CjkRadicalWestTwo => '⻄',
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedSee => '⻅',
            CJKRadicalsSupplement::CjkRadicalSimplifiedHorn => '⻆',
            CJKRadicalsSupplement::CjkRadicalHorn => '⻇',
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedSpeech => '⻈',
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedShell => '⻉',
            CJKRadicalsSupplement::CjkRadicalFoot => '⻊',
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedCart => '⻋',
            CJKRadicalsSupplement::CjkRadicalSimplifiedWalk => '⻌',
            CJKRadicalsSupplement::CjkRadicalWalkOne => '⻍',
            CJKRadicalsSupplement::CjkRadicalWalkTwo => '⻎',
            CJKRadicalsSupplement::CjkRadicalCity => '⻏',
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedGold => '⻐',
            CJKRadicalsSupplement::CjkRadicalLongOne => '⻑',
            CJKRadicalsSupplement::CjkRadicalLongTwo => '⻒',
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedLong => '⻓',
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedGate => '⻔',
            CJKRadicalsSupplement::CjkRadicalMoundOne => '⻕',
            CJKRadicalsSupplement::CjkRadicalMoundTwo => '⻖',
            CJKRadicalsSupplement::CjkRadicalRain => '⻗',
            CJKRadicalsSupplement::CjkRadicalBlue => '⻘',
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedTannedLeather => '⻙',
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedLeaf => '⻚',
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedWind => '⻛',
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedFly => '⻜',
            CJKRadicalsSupplement::CjkRadicalEatOne => '⻝',
            CJKRadicalsSupplement::CjkRadicalEatTwo => '⻞',
            CJKRadicalsSupplement::CjkRadicalEatThree => '⻟',
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedEat => '⻠',
            CJKRadicalsSupplement::CjkRadicalHead => '⻡',
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedHorse => '⻢',
            CJKRadicalsSupplement::CjkRadicalBone => '⻣',
            CJKRadicalsSupplement::CjkRadicalGhost => '⻤',
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedFish => '⻥',
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedBird => '⻦',
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedSalt => '⻧',
            CJKRadicalsSupplement::CjkRadicalSimplifiedWheat => '⻨',
            CJKRadicalsSupplement::CjkRadicalSimplifiedYellow => '⻩',
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedFrog => '⻪',
            CJKRadicalsSupplement::CjkRadicalJDashSimplifiedEven => '⻫',
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedEven => '⻬',
            CJKRadicalsSupplement::CjkRadicalJDashSimplifiedTooth => '⻭',
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedTooth => '⻮',
            CJKRadicalsSupplement::CjkRadicalJDashSimplifiedDragon => '⻯',
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedDragon => '⻰',
            CJKRadicalsSupplement::CjkRadicalTurtle => '⻱',
            CJKRadicalsSupplement::CjkRadicalJDashSimplifiedTurtle => '⻲',
            CJKRadicalsSupplement::CjkRadicalCDashSimplifiedTurtle => '⻳',
        }
    }
}

impl std::convert::TryFrom<char> for CJKRadicalsSupplement {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '⺀' => Ok(CJKRadicalsSupplement::CjkRadicalRepeat),
            '⺁' => Ok(CJKRadicalsSupplement::CjkRadicalCliff),
            '⺂' => Ok(CJKRadicalsSupplement::CjkRadicalSecondOne),
            '⺃' => Ok(CJKRadicalsSupplement::CjkRadicalSecondTwo),
            '⺄' => Ok(CJKRadicalsSupplement::CjkRadicalSecondThree),
            '⺅' => Ok(CJKRadicalsSupplement::CjkRadicalPerson),
            '⺆' => Ok(CJKRadicalsSupplement::CjkRadicalBox),
            '⺇' => Ok(CJKRadicalsSupplement::CjkRadicalTable),
            '⺈' => Ok(CJKRadicalsSupplement::CjkRadicalKnifeOne),
            '⺉' => Ok(CJKRadicalsSupplement::CjkRadicalKnifeTwo),
            '⺊' => Ok(CJKRadicalsSupplement::CjkRadicalDivination),
            '⺋' => Ok(CJKRadicalsSupplement::CjkRadicalSeal),
            '⺌' => Ok(CJKRadicalsSupplement::CjkRadicalSmallOne),
            '⺍' => Ok(CJKRadicalsSupplement::CjkRadicalSmallTwo),
            '⺎' => Ok(CJKRadicalsSupplement::CjkRadicalLameOne),
            '⺏' => Ok(CJKRadicalsSupplement::CjkRadicalLameTwo),
            '⺐' => Ok(CJKRadicalsSupplement::CjkRadicalLameThree),
            '⺑' => Ok(CJKRadicalsSupplement::CjkRadicalLameFour),
            '⺒' => Ok(CJKRadicalsSupplement::CjkRadicalSnake),
            '⺓' => Ok(CJKRadicalsSupplement::CjkRadicalThread),
            '⺔' => Ok(CJKRadicalsSupplement::CjkRadicalSnoutOne),
            '⺕' => Ok(CJKRadicalsSupplement::CjkRadicalSnoutTwo),
            '⺖' => Ok(CJKRadicalsSupplement::CjkRadicalHeartOne),
            '⺗' => Ok(CJKRadicalsSupplement::CjkRadicalHeartTwo),
            '⺘' => Ok(CJKRadicalsSupplement::CjkRadicalHand),
            '⺙' => Ok(CJKRadicalsSupplement::CjkRadicalRap),
            '⺛' => Ok(CJKRadicalsSupplement::CjkRadicalChoke),
            '⺜' => Ok(CJKRadicalsSupplement::CjkRadicalSun),
            '⺝' => Ok(CJKRadicalsSupplement::CjkRadicalMoon),
            '⺞' => Ok(CJKRadicalsSupplement::CjkRadicalDeath),
            '⺟' => Ok(CJKRadicalsSupplement::CjkRadicalMother),
            '⺠' => Ok(CJKRadicalsSupplement::CjkRadicalCivilian),
            '⺡' => Ok(CJKRadicalsSupplement::CjkRadicalWaterOne),
            '⺢' => Ok(CJKRadicalsSupplement::CjkRadicalWaterTwo),
            '⺣' => Ok(CJKRadicalsSupplement::CjkRadicalFire),
            '⺤' => Ok(CJKRadicalsSupplement::CjkRadicalPawOne),
            '⺥' => Ok(CJKRadicalsSupplement::CjkRadicalPawTwo),
            '⺦' => Ok(CJKRadicalsSupplement::CjkRadicalSimplifiedHalfTreeTrunk),
            '⺧' => Ok(CJKRadicalsSupplement::CjkRadicalCow),
            '⺨' => Ok(CJKRadicalsSupplement::CjkRadicalDog),
            '⺩' => Ok(CJKRadicalsSupplement::CjkRadicalJade),
            '⺪' => Ok(CJKRadicalsSupplement::CjkRadicalBoltOfCloth),
            '⺫' => Ok(CJKRadicalsSupplement::CjkRadicalEye),
            '⺬' => Ok(CJKRadicalsSupplement::CjkRadicalSpiritOne),
            '⺭' => Ok(CJKRadicalsSupplement::CjkRadicalSpiritTwo),
            '⺮' => Ok(CJKRadicalsSupplement::CjkRadicalBamboo),
            '⺯' => Ok(CJKRadicalsSupplement::CjkRadicalSilk),
            '⺰' => Ok(CJKRadicalsSupplement::CjkRadicalCDashSimplifiedSilk),
            '⺱' => Ok(CJKRadicalsSupplement::CjkRadicalNetOne),
            '⺲' => Ok(CJKRadicalsSupplement::CjkRadicalNetTwo),
            '⺳' => Ok(CJKRadicalsSupplement::CjkRadicalNetThree),
            '⺴' => Ok(CJKRadicalsSupplement::CjkRadicalNetFour),
            '⺵' => Ok(CJKRadicalsSupplement::CjkRadicalMesh),
            '⺶' => Ok(CJKRadicalsSupplement::CjkRadicalSheep),
            '⺷' => Ok(CJKRadicalsSupplement::CjkRadicalRam),
            '⺸' => Ok(CJKRadicalsSupplement::CjkRadicalEwe),
            '⺹' => Ok(CJKRadicalsSupplement::CjkRadicalOld),
            '⺺' => Ok(CJKRadicalsSupplement::CjkRadicalBrushOne),
            '⺻' => Ok(CJKRadicalsSupplement::CjkRadicalBrushTwo),
            '⺼' => Ok(CJKRadicalsSupplement::CjkRadicalMeat),
            '⺽' => Ok(CJKRadicalsSupplement::CjkRadicalMortar),
            '⺾' => Ok(CJKRadicalsSupplement::CjkRadicalGrassOne),
            '⺿' => Ok(CJKRadicalsSupplement::CjkRadicalGrassTwo),
            '⻀' => Ok(CJKRadicalsSupplement::CjkRadicalGrassThree),
            '⻁' => Ok(CJKRadicalsSupplement::CjkRadicalTiger),
            '⻂' => Ok(CJKRadicalsSupplement::CjkRadicalClothes),
            '⻃' => Ok(CJKRadicalsSupplement::CjkRadicalWestOne),
            '⻄' => Ok(CJKRadicalsSupplement::CjkRadicalWestTwo),
            '⻅' => Ok(CJKRadicalsSupplement::CjkRadicalCDashSimplifiedSee),
            '⻆' => Ok(CJKRadicalsSupplement::CjkRadicalSimplifiedHorn),
            '⻇' => Ok(CJKRadicalsSupplement::CjkRadicalHorn),
            '⻈' => Ok(CJKRadicalsSupplement::CjkRadicalCDashSimplifiedSpeech),
            '⻉' => Ok(CJKRadicalsSupplement::CjkRadicalCDashSimplifiedShell),
            '⻊' => Ok(CJKRadicalsSupplement::CjkRadicalFoot),
            '⻋' => Ok(CJKRadicalsSupplement::CjkRadicalCDashSimplifiedCart),
            '⻌' => Ok(CJKRadicalsSupplement::CjkRadicalSimplifiedWalk),
            '⻍' => Ok(CJKRadicalsSupplement::CjkRadicalWalkOne),
            '⻎' => Ok(CJKRadicalsSupplement::CjkRadicalWalkTwo),
            '⻏' => Ok(CJKRadicalsSupplement::CjkRadicalCity),
            '⻐' => Ok(CJKRadicalsSupplement::CjkRadicalCDashSimplifiedGold),
            '⻑' => Ok(CJKRadicalsSupplement::CjkRadicalLongOne),
            '⻒' => Ok(CJKRadicalsSupplement::CjkRadicalLongTwo),
            '⻓' => Ok(CJKRadicalsSupplement::CjkRadicalCDashSimplifiedLong),
            '⻔' => Ok(CJKRadicalsSupplement::CjkRadicalCDashSimplifiedGate),
            '⻕' => Ok(CJKRadicalsSupplement::CjkRadicalMoundOne),
            '⻖' => Ok(CJKRadicalsSupplement::CjkRadicalMoundTwo),
            '⻗' => Ok(CJKRadicalsSupplement::CjkRadicalRain),
            '⻘' => Ok(CJKRadicalsSupplement::CjkRadicalBlue),
            '⻙' => Ok(CJKRadicalsSupplement::CjkRadicalCDashSimplifiedTannedLeather),
            '⻚' => Ok(CJKRadicalsSupplement::CjkRadicalCDashSimplifiedLeaf),
            '⻛' => Ok(CJKRadicalsSupplement::CjkRadicalCDashSimplifiedWind),
            '⻜' => Ok(CJKRadicalsSupplement::CjkRadicalCDashSimplifiedFly),
            '⻝' => Ok(CJKRadicalsSupplement::CjkRadicalEatOne),
            '⻞' => Ok(CJKRadicalsSupplement::CjkRadicalEatTwo),
            '⻟' => Ok(CJKRadicalsSupplement::CjkRadicalEatThree),
            '⻠' => Ok(CJKRadicalsSupplement::CjkRadicalCDashSimplifiedEat),
            '⻡' => Ok(CJKRadicalsSupplement::CjkRadicalHead),
            '⻢' => Ok(CJKRadicalsSupplement::CjkRadicalCDashSimplifiedHorse),
            '⻣' => Ok(CJKRadicalsSupplement::CjkRadicalBone),
            '⻤' => Ok(CJKRadicalsSupplement::CjkRadicalGhost),
            '⻥' => Ok(CJKRadicalsSupplement::CjkRadicalCDashSimplifiedFish),
            '⻦' => Ok(CJKRadicalsSupplement::CjkRadicalCDashSimplifiedBird),
            '⻧' => Ok(CJKRadicalsSupplement::CjkRadicalCDashSimplifiedSalt),
            '⻨' => Ok(CJKRadicalsSupplement::CjkRadicalSimplifiedWheat),
            '⻩' => Ok(CJKRadicalsSupplement::CjkRadicalSimplifiedYellow),
            '⻪' => Ok(CJKRadicalsSupplement::CjkRadicalCDashSimplifiedFrog),
            '⻫' => Ok(CJKRadicalsSupplement::CjkRadicalJDashSimplifiedEven),
            '⻬' => Ok(CJKRadicalsSupplement::CjkRadicalCDashSimplifiedEven),
            '⻭' => Ok(CJKRadicalsSupplement::CjkRadicalJDashSimplifiedTooth),
            '⻮' => Ok(CJKRadicalsSupplement::CjkRadicalCDashSimplifiedTooth),
            '⻯' => Ok(CJKRadicalsSupplement::CjkRadicalJDashSimplifiedDragon),
            '⻰' => Ok(CJKRadicalsSupplement::CjkRadicalCDashSimplifiedDragon),
            '⻱' => Ok(CJKRadicalsSupplement::CjkRadicalTurtle),
            '⻲' => Ok(CJKRadicalsSupplement::CjkRadicalJDashSimplifiedTurtle),
            '⻳' => Ok(CJKRadicalsSupplement::CjkRadicalCDashSimplifiedTurtle),
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

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("CJKRadicalsSupplement{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
