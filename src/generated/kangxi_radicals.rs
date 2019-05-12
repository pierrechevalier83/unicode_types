
/// An enum to represent all characters in the KangxiRadicals block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum KangxiRadicals {
    /// \u{2f00}: '⼀'
    KangxiRadicalOne,
    /// \u{2f01}: '⼁'
    KangxiRadicalLine,
    /// \u{2f02}: '⼂'
    KangxiRadicalDot,
    /// \u{2f03}: '⼃'
    KangxiRadicalSlash,
    /// \u{2f04}: '⼄'
    KangxiRadicalSecond,
    /// \u{2f05}: '⼅'
    KangxiRadicalHook,
    /// \u{2f06}: '⼆'
    KangxiRadicalTwo,
    /// \u{2f07}: '⼇'
    KangxiRadicalLid,
    /// \u{2f08}: '⼈'
    KangxiRadicalMan,
    /// \u{2f09}: '⼉'
    KangxiRadicalLegs,
    /// \u{2f0a}: '⼊'
    KangxiRadicalEnter,
    /// \u{2f0b}: '⼋'
    KangxiRadicalEight,
    /// \u{2f0c}: '⼌'
    KangxiRadicalDownBox,
    /// \u{2f0d}: '⼍'
    KangxiRadicalCover,
    /// \u{2f0e}: '⼎'
    KangxiRadicalIce,
    /// \u{2f0f}: '⼏'
    KangxiRadicalTable,
    /// \u{2f10}: '⼐'
    KangxiRadicalOpenBox,
    /// \u{2f11}: '⼑'
    KangxiRadicalKnife,
    /// \u{2f12}: '⼒'
    KangxiRadicalPower,
    /// \u{2f13}: '⼓'
    KangxiRadicalWrap,
    /// \u{2f14}: '⼔'
    KangxiRadicalSpoon,
    /// \u{2f15}: '⼕'
    KangxiRadicalRightOpenBox,
    /// \u{2f16}: '⼖'
    KangxiRadicalHidingEnclosure,
    /// \u{2f17}: '⼗'
    KangxiRadicalTen,
    /// \u{2f18}: '⼘'
    KangxiRadicalDivination,
    /// \u{2f19}: '⼙'
    KangxiRadicalSeal,
    /// \u{2f1a}: '⼚'
    KangxiRadicalCliff,
    /// \u{2f1b}: '⼛'
    KangxiRadicalPrivate,
    /// \u{2f1c}: '⼜'
    KangxiRadicalAgain,
    /// \u{2f1d}: '⼝'
    KangxiRadicalMouth,
    /// \u{2f1e}: '⼞'
    KangxiRadicalEnclosure,
    /// \u{2f1f}: '⼟'
    KangxiRadicalEarth,
    /// \u{2f20}: '⼠'
    KangxiRadicalScholar,
    /// \u{2f21}: '⼡'
    KangxiRadicalGo,
    /// \u{2f22}: '⼢'
    KangxiRadicalGoSlowly,
    /// \u{2f23}: '⼣'
    KangxiRadicalEvening,
    /// \u{2f24}: '⼤'
    KangxiRadicalBig,
    /// \u{2f25}: '⼥'
    KangxiRadicalWoman,
    /// \u{2f26}: '⼦'
    KangxiRadicalChild,
    /// \u{2f27}: '⼧'
    KangxiRadicalRoof,
    /// \u{2f28}: '⼨'
    KangxiRadicalInch,
    /// \u{2f29}: '⼩'
    KangxiRadicalSmall,
    /// \u{2f2a}: '⼪'
    KangxiRadicalLame,
    /// \u{2f2b}: '⼫'
    KangxiRadicalCorpse,
    /// \u{2f2c}: '⼬'
    KangxiRadicalSprout,
    /// \u{2f2d}: '⼭'
    KangxiRadicalMountain,
    /// \u{2f2e}: '⼮'
    KangxiRadicalRiver,
    /// \u{2f2f}: '⼯'
    KangxiRadicalWork,
    /// \u{2f30}: '⼰'
    KangxiRadicalOneself,
    /// \u{2f31}: '⼱'
    KangxiRadicalTurban,
    /// \u{2f32}: '⼲'
    KangxiRadicalDry,
    /// \u{2f33}: '⼳'
    KangxiRadicalShortThread,
    /// \u{2f34}: '⼴'
    KangxiRadicalDottedCliff,
    /// \u{2f35}: '⼵'
    KangxiRadicalLongStride,
    /// \u{2f36}: '⼶'
    KangxiRadicalTwoHands,
    /// \u{2f37}: '⼷'
    KangxiRadicalShoot,
    /// \u{2f38}: '⼸'
    KangxiRadicalBow,
    /// \u{2f39}: '⼹'
    KangxiRadicalSnout,
    /// \u{2f3a}: '⼺'
    KangxiRadicalBristle,
    /// \u{2f3b}: '⼻'
    KangxiRadicalStep,
    /// \u{2f3c}: '⼼'
    KangxiRadicalHeart,
    /// \u{2f3d}: '⼽'
    KangxiRadicalHalberd,
    /// \u{2f3e}: '⼾'
    KangxiRadicalDoor,
    /// \u{2f3f}: '⼿'
    KangxiRadicalHand,
    /// \u{2f40}: '⽀'
    KangxiRadicalBranch,
    /// \u{2f41}: '⽁'
    KangxiRadicalRap,
    /// \u{2f42}: '⽂'
    KangxiRadicalScript,
    /// \u{2f43}: '⽃'
    KangxiRadicalDipper,
    /// \u{2f44}: '⽄'
    KangxiRadicalAxe,
    /// \u{2f45}: '⽅'
    KangxiRadicalSquare,
    /// \u{2f46}: '⽆'
    KangxiRadicalNot,
    /// \u{2f47}: '⽇'
    KangxiRadicalSun,
    /// \u{2f48}: '⽈'
    KangxiRadicalSay,
    /// \u{2f49}: '⽉'
    KangxiRadicalMoon,
    /// \u{2f4a}: '⽊'
    KangxiRadicalTree,
    /// \u{2f4b}: '⽋'
    KangxiRadicalLack,
    /// \u{2f4c}: '⽌'
    KangxiRadicalStop,
    /// \u{2f4d}: '⽍'
    KangxiRadicalDeath,
    /// \u{2f4e}: '⽎'
    KangxiRadicalWeapon,
    /// \u{2f4f}: '⽏'
    KangxiRadicalDoNot,
    /// \u{2f50}: '⽐'
    KangxiRadicalCompare,
    /// \u{2f51}: '⽑'
    KangxiRadicalFur,
    /// \u{2f52}: '⽒'
    KangxiRadicalClan,
    /// \u{2f53}: '⽓'
    KangxiRadicalSteam,
    /// \u{2f54}: '⽔'
    KangxiRadicalWater,
    /// \u{2f55}: '⽕'
    KangxiRadicalFire,
    /// \u{2f56}: '⽖'
    KangxiRadicalClaw,
    /// \u{2f57}: '⽗'
    KangxiRadicalFather,
    /// \u{2f58}: '⽘'
    KangxiRadicalDoubleX,
    /// \u{2f59}: '⽙'
    KangxiRadicalHalfTreeTrunk,
    /// \u{2f5a}: '⽚'
    KangxiRadicalSlice,
    /// \u{2f5b}: '⽛'
    KangxiRadicalFang,
    /// \u{2f5c}: '⽜'
    KangxiRadicalCow,
    /// \u{2f5d}: '⽝'
    KangxiRadicalDog,
    /// \u{2f5e}: '⽞'
    KangxiRadicalProfound,
    /// \u{2f5f}: '⽟'
    KangxiRadicalJade,
    /// \u{2f60}: '⽠'
    KangxiRadicalMelon,
    /// \u{2f61}: '⽡'
    KangxiRadicalTile,
    /// \u{2f62}: '⽢'
    KangxiRadicalSweet,
    /// \u{2f63}: '⽣'
    KangxiRadicalLife,
    /// \u{2f64}: '⽤'
    KangxiRadicalUse,
    /// \u{2f65}: '⽥'
    KangxiRadicalField,
    /// \u{2f66}: '⽦'
    KangxiRadicalBoltOfCloth,
    /// \u{2f67}: '⽧'
    KangxiRadicalSickness,
    /// \u{2f68}: '⽨'
    KangxiRadicalDottedTent,
    /// \u{2f69}: '⽩'
    KangxiRadicalWhite,
    /// \u{2f6a}: '⽪'
    KangxiRadicalSkin,
    /// \u{2f6b}: '⽫'
    KangxiRadicalDish,
    /// \u{2f6c}: '⽬'
    KangxiRadicalEye,
    /// \u{2f6d}: '⽭'
    KangxiRadicalSpear,
    /// \u{2f6e}: '⽮'
    KangxiRadicalArrow,
    /// \u{2f6f}: '⽯'
    KangxiRadicalStone,
    /// \u{2f70}: '⽰'
    KangxiRadicalSpirit,
    /// \u{2f71}: '⽱'
    KangxiRadicalTrack,
    /// \u{2f72}: '⽲'
    KangxiRadicalGrain,
    /// \u{2f73}: '⽳'
    KangxiRadicalCave,
    /// \u{2f74}: '⽴'
    KangxiRadicalStand,
    /// \u{2f75}: '⽵'
    KangxiRadicalBamboo,
    /// \u{2f76}: '⽶'
    KangxiRadicalRice,
    /// \u{2f77}: '⽷'
    KangxiRadicalSilk,
    /// \u{2f78}: '⽸'
    KangxiRadicalJar,
    /// \u{2f79}: '⽹'
    KangxiRadicalNet,
    /// \u{2f7a}: '⽺'
    KangxiRadicalSheep,
    /// \u{2f7b}: '⽻'
    KangxiRadicalFeather,
    /// \u{2f7c}: '⽼'
    KangxiRadicalOld,
    /// \u{2f7d}: '⽽'
    KangxiRadicalAnd,
    /// \u{2f7e}: '⽾'
    KangxiRadicalPlow,
    /// \u{2f7f}: '⽿'
    KangxiRadicalEar,
    /// \u{2f80}: '⾀'
    KangxiRadicalBrush,
    /// \u{2f81}: '⾁'
    KangxiRadicalMeat,
    /// \u{2f82}: '⾂'
    KangxiRadicalMinister,
    /// \u{2f83}: '⾃'
    KangxiRadicalSelf,
    /// \u{2f84}: '⾄'
    KangxiRadicalArrive,
    /// \u{2f85}: '⾅'
    KangxiRadicalMortar,
    /// \u{2f86}: '⾆'
    KangxiRadicalTongue,
    /// \u{2f87}: '⾇'
    KangxiRadicalOppose,
    /// \u{2f88}: '⾈'
    KangxiRadicalBoat,
    /// \u{2f89}: '⾉'
    KangxiRadicalStopping,
    /// \u{2f8a}: '⾊'
    KangxiRadicalColor,
    /// \u{2f8b}: '⾋'
    KangxiRadicalGrass,
    /// \u{2f8c}: '⾌'
    KangxiRadicalTiger,
    /// \u{2f8d}: '⾍'
    KangxiRadicalInsect,
    /// \u{2f8e}: '⾎'
    KangxiRadicalBlood,
    /// \u{2f8f}: '⾏'
    KangxiRadicalWalkEnclosure,
    /// \u{2f90}: '⾐'
    KangxiRadicalClothes,
    /// \u{2f91}: '⾑'
    KangxiRadicalWest,
    /// \u{2f92}: '⾒'
    KangxiRadicalSee,
    /// \u{2f93}: '⾓'
    KangxiRadicalHorn,
    /// \u{2f94}: '⾔'
    KangxiRadicalSpeech,
    /// \u{2f95}: '⾕'
    KangxiRadicalValley,
    /// \u{2f96}: '⾖'
    KangxiRadicalBean,
    /// \u{2f97}: '⾗'
    KangxiRadicalPig,
    /// \u{2f98}: '⾘'
    KangxiRadicalBadger,
    /// \u{2f99}: '⾙'
    KangxiRadicalShell,
    /// \u{2f9a}: '⾚'
    KangxiRadicalRed,
    /// \u{2f9b}: '⾛'
    KangxiRadicalRun,
    /// \u{2f9c}: '⾜'
    KangxiRadicalFoot,
    /// \u{2f9d}: '⾝'
    KangxiRadicalBody,
    /// \u{2f9e}: '⾞'
    KangxiRadicalCart,
    /// \u{2f9f}: '⾟'
    KangxiRadicalBitter,
    /// \u{2fa0}: '⾠'
    KangxiRadicalMorning,
    /// \u{2fa1}: '⾡'
    KangxiRadicalWalk,
    /// \u{2fa2}: '⾢'
    KangxiRadicalCity,
    /// \u{2fa3}: '⾣'
    KangxiRadicalWine,
    /// \u{2fa4}: '⾤'
    KangxiRadicalDistinguish,
    /// \u{2fa5}: '⾥'
    KangxiRadicalVillage,
    /// \u{2fa6}: '⾦'
    KangxiRadicalGold,
    /// \u{2fa7}: '⾧'
    KangxiRadicalLong,
    /// \u{2fa8}: '⾨'
    KangxiRadicalGate,
    /// \u{2fa9}: '⾩'
    KangxiRadicalMound,
    /// \u{2faa}: '⾪'
    KangxiRadicalSlave,
    /// \u{2fab}: '⾫'
    KangxiRadicalShortTailedBird,
    /// \u{2fac}: '⾬'
    KangxiRadicalRain,
    /// \u{2fad}: '⾭'
    KangxiRadicalBlue,
    /// \u{2fae}: '⾮'
    KangxiRadicalWrong,
    /// \u{2faf}: '⾯'
    KangxiRadicalFace,
    /// \u{2fb0}: '⾰'
    KangxiRadicalLeather,
    /// \u{2fb1}: '⾱'
    KangxiRadicalTannedLeather,
    /// \u{2fb2}: '⾲'
    KangxiRadicalLeek,
    /// \u{2fb3}: '⾳'
    KangxiRadicalSound,
    /// \u{2fb4}: '⾴'
    KangxiRadicalLeaf,
    /// \u{2fb5}: '⾵'
    KangxiRadicalWind,
    /// \u{2fb6}: '⾶'
    KangxiRadicalFly,
    /// \u{2fb7}: '⾷'
    KangxiRadicalEat,
    /// \u{2fb8}: '⾸'
    KangxiRadicalHead,
    /// \u{2fb9}: '⾹'
    KangxiRadicalFragrant,
    /// \u{2fba}: '⾺'
    KangxiRadicalHorse,
    /// \u{2fbb}: '⾻'
    KangxiRadicalBone,
    /// \u{2fbc}: '⾼'
    KangxiRadicalTall,
    /// \u{2fbd}: '⾽'
    KangxiRadicalHair,
    /// \u{2fbe}: '⾾'
    KangxiRadicalFight,
    /// \u{2fbf}: '⾿'
    KangxiRadicalSacrificialWine,
    /// \u{2fc0}: '⿀'
    KangxiRadicalCauldron,
    /// \u{2fc1}: '⿁'
    KangxiRadicalGhost,
    /// \u{2fc2}: '⿂'
    KangxiRadicalFish,
    /// \u{2fc3}: '⿃'
    KangxiRadicalBird,
    /// \u{2fc4}: '⿄'
    KangxiRadicalSalt,
    /// \u{2fc5}: '⿅'
    KangxiRadicalDeer,
    /// \u{2fc6}: '⿆'
    KangxiRadicalWheat,
    /// \u{2fc7}: '⿇'
    KangxiRadicalHemp,
    /// \u{2fc8}: '⿈'
    KangxiRadicalYellow,
    /// \u{2fc9}: '⿉'
    KangxiRadicalMillet,
    /// \u{2fca}: '⿊'
    KangxiRadicalBlack,
    /// \u{2fcb}: '⿋'
    KangxiRadicalEmbroidery,
    /// \u{2fcc}: '⿌'
    KangxiRadicalFrog,
    /// \u{2fcd}: '⿍'
    KangxiRadicalTripod,
    /// \u{2fce}: '⿎'
    KangxiRadicalDrum,
    /// \u{2fcf}: '⿏'
    KangxiRadicalRat,
    /// \u{2fd0}: '⿐'
    KangxiRadicalNose,
    /// \u{2fd1}: '⿑'
    KangxiRadicalEven,
    /// \u{2fd2}: '⿒'
    KangxiRadicalTooth,
    /// \u{2fd3}: '⿓'
    KangxiRadicalDragon,
    /// \u{2fd4}: '⿔'
    KangxiRadicalTurtle,
    /// \u{2fd5}: '⿕'
    KangxiRadicalFlute,
}

impl Into<char> for KangxiRadicals {
    fn into(self) -> char {
        match self {
            KangxiRadicals::KangxiRadicalOne => '⼀',
            KangxiRadicals::KangxiRadicalLine => '⼁',
            KangxiRadicals::KangxiRadicalDot => '⼂',
            KangxiRadicals::KangxiRadicalSlash => '⼃',
            KangxiRadicals::KangxiRadicalSecond => '⼄',
            KangxiRadicals::KangxiRadicalHook => '⼅',
            KangxiRadicals::KangxiRadicalTwo => '⼆',
            KangxiRadicals::KangxiRadicalLid => '⼇',
            KangxiRadicals::KangxiRadicalMan => '⼈',
            KangxiRadicals::KangxiRadicalLegs => '⼉',
            KangxiRadicals::KangxiRadicalEnter => '⼊',
            KangxiRadicals::KangxiRadicalEight => '⼋',
            KangxiRadicals::KangxiRadicalDownBox => '⼌',
            KangxiRadicals::KangxiRadicalCover => '⼍',
            KangxiRadicals::KangxiRadicalIce => '⼎',
            KangxiRadicals::KangxiRadicalTable => '⼏',
            KangxiRadicals::KangxiRadicalOpenBox => '⼐',
            KangxiRadicals::KangxiRadicalKnife => '⼑',
            KangxiRadicals::KangxiRadicalPower => '⼒',
            KangxiRadicals::KangxiRadicalWrap => '⼓',
            KangxiRadicals::KangxiRadicalSpoon => '⼔',
            KangxiRadicals::KangxiRadicalRightOpenBox => '⼕',
            KangxiRadicals::KangxiRadicalHidingEnclosure => '⼖',
            KangxiRadicals::KangxiRadicalTen => '⼗',
            KangxiRadicals::KangxiRadicalDivination => '⼘',
            KangxiRadicals::KangxiRadicalSeal => '⼙',
            KangxiRadicals::KangxiRadicalCliff => '⼚',
            KangxiRadicals::KangxiRadicalPrivate => '⼛',
            KangxiRadicals::KangxiRadicalAgain => '⼜',
            KangxiRadicals::KangxiRadicalMouth => '⼝',
            KangxiRadicals::KangxiRadicalEnclosure => '⼞',
            KangxiRadicals::KangxiRadicalEarth => '⼟',
            KangxiRadicals::KangxiRadicalScholar => '⼠',
            KangxiRadicals::KangxiRadicalGo => '⼡',
            KangxiRadicals::KangxiRadicalGoSlowly => '⼢',
            KangxiRadicals::KangxiRadicalEvening => '⼣',
            KangxiRadicals::KangxiRadicalBig => '⼤',
            KangxiRadicals::KangxiRadicalWoman => '⼥',
            KangxiRadicals::KangxiRadicalChild => '⼦',
            KangxiRadicals::KangxiRadicalRoof => '⼧',
            KangxiRadicals::KangxiRadicalInch => '⼨',
            KangxiRadicals::KangxiRadicalSmall => '⼩',
            KangxiRadicals::KangxiRadicalLame => '⼪',
            KangxiRadicals::KangxiRadicalCorpse => '⼫',
            KangxiRadicals::KangxiRadicalSprout => '⼬',
            KangxiRadicals::KangxiRadicalMountain => '⼭',
            KangxiRadicals::KangxiRadicalRiver => '⼮',
            KangxiRadicals::KangxiRadicalWork => '⼯',
            KangxiRadicals::KangxiRadicalOneself => '⼰',
            KangxiRadicals::KangxiRadicalTurban => '⼱',
            KangxiRadicals::KangxiRadicalDry => '⼲',
            KangxiRadicals::KangxiRadicalShortThread => '⼳',
            KangxiRadicals::KangxiRadicalDottedCliff => '⼴',
            KangxiRadicals::KangxiRadicalLongStride => '⼵',
            KangxiRadicals::KangxiRadicalTwoHands => '⼶',
            KangxiRadicals::KangxiRadicalShoot => '⼷',
            KangxiRadicals::KangxiRadicalBow => '⼸',
            KangxiRadicals::KangxiRadicalSnout => '⼹',
            KangxiRadicals::KangxiRadicalBristle => '⼺',
            KangxiRadicals::KangxiRadicalStep => '⼻',
            KangxiRadicals::KangxiRadicalHeart => '⼼',
            KangxiRadicals::KangxiRadicalHalberd => '⼽',
            KangxiRadicals::KangxiRadicalDoor => '⼾',
            KangxiRadicals::KangxiRadicalHand => '⼿',
            KangxiRadicals::KangxiRadicalBranch => '⽀',
            KangxiRadicals::KangxiRadicalRap => '⽁',
            KangxiRadicals::KangxiRadicalScript => '⽂',
            KangxiRadicals::KangxiRadicalDipper => '⽃',
            KangxiRadicals::KangxiRadicalAxe => '⽄',
            KangxiRadicals::KangxiRadicalSquare => '⽅',
            KangxiRadicals::KangxiRadicalNot => '⽆',
            KangxiRadicals::KangxiRadicalSun => '⽇',
            KangxiRadicals::KangxiRadicalSay => '⽈',
            KangxiRadicals::KangxiRadicalMoon => '⽉',
            KangxiRadicals::KangxiRadicalTree => '⽊',
            KangxiRadicals::KangxiRadicalLack => '⽋',
            KangxiRadicals::KangxiRadicalStop => '⽌',
            KangxiRadicals::KangxiRadicalDeath => '⽍',
            KangxiRadicals::KangxiRadicalWeapon => '⽎',
            KangxiRadicals::KangxiRadicalDoNot => '⽏',
            KangxiRadicals::KangxiRadicalCompare => '⽐',
            KangxiRadicals::KangxiRadicalFur => '⽑',
            KangxiRadicals::KangxiRadicalClan => '⽒',
            KangxiRadicals::KangxiRadicalSteam => '⽓',
            KangxiRadicals::KangxiRadicalWater => '⽔',
            KangxiRadicals::KangxiRadicalFire => '⽕',
            KangxiRadicals::KangxiRadicalClaw => '⽖',
            KangxiRadicals::KangxiRadicalFather => '⽗',
            KangxiRadicals::KangxiRadicalDoubleX => '⽘',
            KangxiRadicals::KangxiRadicalHalfTreeTrunk => '⽙',
            KangxiRadicals::KangxiRadicalSlice => '⽚',
            KangxiRadicals::KangxiRadicalFang => '⽛',
            KangxiRadicals::KangxiRadicalCow => '⽜',
            KangxiRadicals::KangxiRadicalDog => '⽝',
            KangxiRadicals::KangxiRadicalProfound => '⽞',
            KangxiRadicals::KangxiRadicalJade => '⽟',
            KangxiRadicals::KangxiRadicalMelon => '⽠',
            KangxiRadicals::KangxiRadicalTile => '⽡',
            KangxiRadicals::KangxiRadicalSweet => '⽢',
            KangxiRadicals::KangxiRadicalLife => '⽣',
            KangxiRadicals::KangxiRadicalUse => '⽤',
            KangxiRadicals::KangxiRadicalField => '⽥',
            KangxiRadicals::KangxiRadicalBoltOfCloth => '⽦',
            KangxiRadicals::KangxiRadicalSickness => '⽧',
            KangxiRadicals::KangxiRadicalDottedTent => '⽨',
            KangxiRadicals::KangxiRadicalWhite => '⽩',
            KangxiRadicals::KangxiRadicalSkin => '⽪',
            KangxiRadicals::KangxiRadicalDish => '⽫',
            KangxiRadicals::KangxiRadicalEye => '⽬',
            KangxiRadicals::KangxiRadicalSpear => '⽭',
            KangxiRadicals::KangxiRadicalArrow => '⽮',
            KangxiRadicals::KangxiRadicalStone => '⽯',
            KangxiRadicals::KangxiRadicalSpirit => '⽰',
            KangxiRadicals::KangxiRadicalTrack => '⽱',
            KangxiRadicals::KangxiRadicalGrain => '⽲',
            KangxiRadicals::KangxiRadicalCave => '⽳',
            KangxiRadicals::KangxiRadicalStand => '⽴',
            KangxiRadicals::KangxiRadicalBamboo => '⽵',
            KangxiRadicals::KangxiRadicalRice => '⽶',
            KangxiRadicals::KangxiRadicalSilk => '⽷',
            KangxiRadicals::KangxiRadicalJar => '⽸',
            KangxiRadicals::KangxiRadicalNet => '⽹',
            KangxiRadicals::KangxiRadicalSheep => '⽺',
            KangxiRadicals::KangxiRadicalFeather => '⽻',
            KangxiRadicals::KangxiRadicalOld => '⽼',
            KangxiRadicals::KangxiRadicalAnd => '⽽',
            KangxiRadicals::KangxiRadicalPlow => '⽾',
            KangxiRadicals::KangxiRadicalEar => '⽿',
            KangxiRadicals::KangxiRadicalBrush => '⾀',
            KangxiRadicals::KangxiRadicalMeat => '⾁',
            KangxiRadicals::KangxiRadicalMinister => '⾂',
            KangxiRadicals::KangxiRadicalSelf => '⾃',
            KangxiRadicals::KangxiRadicalArrive => '⾄',
            KangxiRadicals::KangxiRadicalMortar => '⾅',
            KangxiRadicals::KangxiRadicalTongue => '⾆',
            KangxiRadicals::KangxiRadicalOppose => '⾇',
            KangxiRadicals::KangxiRadicalBoat => '⾈',
            KangxiRadicals::KangxiRadicalStopping => '⾉',
            KangxiRadicals::KangxiRadicalColor => '⾊',
            KangxiRadicals::KangxiRadicalGrass => '⾋',
            KangxiRadicals::KangxiRadicalTiger => '⾌',
            KangxiRadicals::KangxiRadicalInsect => '⾍',
            KangxiRadicals::KangxiRadicalBlood => '⾎',
            KangxiRadicals::KangxiRadicalWalkEnclosure => '⾏',
            KangxiRadicals::KangxiRadicalClothes => '⾐',
            KangxiRadicals::KangxiRadicalWest => '⾑',
            KangxiRadicals::KangxiRadicalSee => '⾒',
            KangxiRadicals::KangxiRadicalHorn => '⾓',
            KangxiRadicals::KangxiRadicalSpeech => '⾔',
            KangxiRadicals::KangxiRadicalValley => '⾕',
            KangxiRadicals::KangxiRadicalBean => '⾖',
            KangxiRadicals::KangxiRadicalPig => '⾗',
            KangxiRadicals::KangxiRadicalBadger => '⾘',
            KangxiRadicals::KangxiRadicalShell => '⾙',
            KangxiRadicals::KangxiRadicalRed => '⾚',
            KangxiRadicals::KangxiRadicalRun => '⾛',
            KangxiRadicals::KangxiRadicalFoot => '⾜',
            KangxiRadicals::KangxiRadicalBody => '⾝',
            KangxiRadicals::KangxiRadicalCart => '⾞',
            KangxiRadicals::KangxiRadicalBitter => '⾟',
            KangxiRadicals::KangxiRadicalMorning => '⾠',
            KangxiRadicals::KangxiRadicalWalk => '⾡',
            KangxiRadicals::KangxiRadicalCity => '⾢',
            KangxiRadicals::KangxiRadicalWine => '⾣',
            KangxiRadicals::KangxiRadicalDistinguish => '⾤',
            KangxiRadicals::KangxiRadicalVillage => '⾥',
            KangxiRadicals::KangxiRadicalGold => '⾦',
            KangxiRadicals::KangxiRadicalLong => '⾧',
            KangxiRadicals::KangxiRadicalGate => '⾨',
            KangxiRadicals::KangxiRadicalMound => '⾩',
            KangxiRadicals::KangxiRadicalSlave => '⾪',
            KangxiRadicals::KangxiRadicalShortTailedBird => '⾫',
            KangxiRadicals::KangxiRadicalRain => '⾬',
            KangxiRadicals::KangxiRadicalBlue => '⾭',
            KangxiRadicals::KangxiRadicalWrong => '⾮',
            KangxiRadicals::KangxiRadicalFace => '⾯',
            KangxiRadicals::KangxiRadicalLeather => '⾰',
            KangxiRadicals::KangxiRadicalTannedLeather => '⾱',
            KangxiRadicals::KangxiRadicalLeek => '⾲',
            KangxiRadicals::KangxiRadicalSound => '⾳',
            KangxiRadicals::KangxiRadicalLeaf => '⾴',
            KangxiRadicals::KangxiRadicalWind => '⾵',
            KangxiRadicals::KangxiRadicalFly => '⾶',
            KangxiRadicals::KangxiRadicalEat => '⾷',
            KangxiRadicals::KangxiRadicalHead => '⾸',
            KangxiRadicals::KangxiRadicalFragrant => '⾹',
            KangxiRadicals::KangxiRadicalHorse => '⾺',
            KangxiRadicals::KangxiRadicalBone => '⾻',
            KangxiRadicals::KangxiRadicalTall => '⾼',
            KangxiRadicals::KangxiRadicalHair => '⾽',
            KangxiRadicals::KangxiRadicalFight => '⾾',
            KangxiRadicals::KangxiRadicalSacrificialWine => '⾿',
            KangxiRadicals::KangxiRadicalCauldron => '⿀',
            KangxiRadicals::KangxiRadicalGhost => '⿁',
            KangxiRadicals::KangxiRadicalFish => '⿂',
            KangxiRadicals::KangxiRadicalBird => '⿃',
            KangxiRadicals::KangxiRadicalSalt => '⿄',
            KangxiRadicals::KangxiRadicalDeer => '⿅',
            KangxiRadicals::KangxiRadicalWheat => '⿆',
            KangxiRadicals::KangxiRadicalHemp => '⿇',
            KangxiRadicals::KangxiRadicalYellow => '⿈',
            KangxiRadicals::KangxiRadicalMillet => '⿉',
            KangxiRadicals::KangxiRadicalBlack => '⿊',
            KangxiRadicals::KangxiRadicalEmbroidery => '⿋',
            KangxiRadicals::KangxiRadicalFrog => '⿌',
            KangxiRadicals::KangxiRadicalTripod => '⿍',
            KangxiRadicals::KangxiRadicalDrum => '⿎',
            KangxiRadicals::KangxiRadicalRat => '⿏',
            KangxiRadicals::KangxiRadicalNose => '⿐',
            KangxiRadicals::KangxiRadicalEven => '⿑',
            KangxiRadicals::KangxiRadicalTooth => '⿒',
            KangxiRadicals::KangxiRadicalDragon => '⿓',
            KangxiRadicals::KangxiRadicalTurtle => '⿔',
            KangxiRadicals::KangxiRadicalFlute => '⿕',
        }
    }
}

impl std::convert::TryFrom<char> for KangxiRadicals {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '⼀' => Ok(KangxiRadicals::KangxiRadicalOne),
            '⼁' => Ok(KangxiRadicals::KangxiRadicalLine),
            '⼂' => Ok(KangxiRadicals::KangxiRadicalDot),
            '⼃' => Ok(KangxiRadicals::KangxiRadicalSlash),
            '⼄' => Ok(KangxiRadicals::KangxiRadicalSecond),
            '⼅' => Ok(KangxiRadicals::KangxiRadicalHook),
            '⼆' => Ok(KangxiRadicals::KangxiRadicalTwo),
            '⼇' => Ok(KangxiRadicals::KangxiRadicalLid),
            '⼈' => Ok(KangxiRadicals::KangxiRadicalMan),
            '⼉' => Ok(KangxiRadicals::KangxiRadicalLegs),
            '⼊' => Ok(KangxiRadicals::KangxiRadicalEnter),
            '⼋' => Ok(KangxiRadicals::KangxiRadicalEight),
            '⼌' => Ok(KangxiRadicals::KangxiRadicalDownBox),
            '⼍' => Ok(KangxiRadicals::KangxiRadicalCover),
            '⼎' => Ok(KangxiRadicals::KangxiRadicalIce),
            '⼏' => Ok(KangxiRadicals::KangxiRadicalTable),
            '⼐' => Ok(KangxiRadicals::KangxiRadicalOpenBox),
            '⼑' => Ok(KangxiRadicals::KangxiRadicalKnife),
            '⼒' => Ok(KangxiRadicals::KangxiRadicalPower),
            '⼓' => Ok(KangxiRadicals::KangxiRadicalWrap),
            '⼔' => Ok(KangxiRadicals::KangxiRadicalSpoon),
            '⼕' => Ok(KangxiRadicals::KangxiRadicalRightOpenBox),
            '⼖' => Ok(KangxiRadicals::KangxiRadicalHidingEnclosure),
            '⼗' => Ok(KangxiRadicals::KangxiRadicalTen),
            '⼘' => Ok(KangxiRadicals::KangxiRadicalDivination),
            '⼙' => Ok(KangxiRadicals::KangxiRadicalSeal),
            '⼚' => Ok(KangxiRadicals::KangxiRadicalCliff),
            '⼛' => Ok(KangxiRadicals::KangxiRadicalPrivate),
            '⼜' => Ok(KangxiRadicals::KangxiRadicalAgain),
            '⼝' => Ok(KangxiRadicals::KangxiRadicalMouth),
            '⼞' => Ok(KangxiRadicals::KangxiRadicalEnclosure),
            '⼟' => Ok(KangxiRadicals::KangxiRadicalEarth),
            '⼠' => Ok(KangxiRadicals::KangxiRadicalScholar),
            '⼡' => Ok(KangxiRadicals::KangxiRadicalGo),
            '⼢' => Ok(KangxiRadicals::KangxiRadicalGoSlowly),
            '⼣' => Ok(KangxiRadicals::KangxiRadicalEvening),
            '⼤' => Ok(KangxiRadicals::KangxiRadicalBig),
            '⼥' => Ok(KangxiRadicals::KangxiRadicalWoman),
            '⼦' => Ok(KangxiRadicals::KangxiRadicalChild),
            '⼧' => Ok(KangxiRadicals::KangxiRadicalRoof),
            '⼨' => Ok(KangxiRadicals::KangxiRadicalInch),
            '⼩' => Ok(KangxiRadicals::KangxiRadicalSmall),
            '⼪' => Ok(KangxiRadicals::KangxiRadicalLame),
            '⼫' => Ok(KangxiRadicals::KangxiRadicalCorpse),
            '⼬' => Ok(KangxiRadicals::KangxiRadicalSprout),
            '⼭' => Ok(KangxiRadicals::KangxiRadicalMountain),
            '⼮' => Ok(KangxiRadicals::KangxiRadicalRiver),
            '⼯' => Ok(KangxiRadicals::KangxiRadicalWork),
            '⼰' => Ok(KangxiRadicals::KangxiRadicalOneself),
            '⼱' => Ok(KangxiRadicals::KangxiRadicalTurban),
            '⼲' => Ok(KangxiRadicals::KangxiRadicalDry),
            '⼳' => Ok(KangxiRadicals::KangxiRadicalShortThread),
            '⼴' => Ok(KangxiRadicals::KangxiRadicalDottedCliff),
            '⼵' => Ok(KangxiRadicals::KangxiRadicalLongStride),
            '⼶' => Ok(KangxiRadicals::KangxiRadicalTwoHands),
            '⼷' => Ok(KangxiRadicals::KangxiRadicalShoot),
            '⼸' => Ok(KangxiRadicals::KangxiRadicalBow),
            '⼹' => Ok(KangxiRadicals::KangxiRadicalSnout),
            '⼺' => Ok(KangxiRadicals::KangxiRadicalBristle),
            '⼻' => Ok(KangxiRadicals::KangxiRadicalStep),
            '⼼' => Ok(KangxiRadicals::KangxiRadicalHeart),
            '⼽' => Ok(KangxiRadicals::KangxiRadicalHalberd),
            '⼾' => Ok(KangxiRadicals::KangxiRadicalDoor),
            '⼿' => Ok(KangxiRadicals::KangxiRadicalHand),
            '⽀' => Ok(KangxiRadicals::KangxiRadicalBranch),
            '⽁' => Ok(KangxiRadicals::KangxiRadicalRap),
            '⽂' => Ok(KangxiRadicals::KangxiRadicalScript),
            '⽃' => Ok(KangxiRadicals::KangxiRadicalDipper),
            '⽄' => Ok(KangxiRadicals::KangxiRadicalAxe),
            '⽅' => Ok(KangxiRadicals::KangxiRadicalSquare),
            '⽆' => Ok(KangxiRadicals::KangxiRadicalNot),
            '⽇' => Ok(KangxiRadicals::KangxiRadicalSun),
            '⽈' => Ok(KangxiRadicals::KangxiRadicalSay),
            '⽉' => Ok(KangxiRadicals::KangxiRadicalMoon),
            '⽊' => Ok(KangxiRadicals::KangxiRadicalTree),
            '⽋' => Ok(KangxiRadicals::KangxiRadicalLack),
            '⽌' => Ok(KangxiRadicals::KangxiRadicalStop),
            '⽍' => Ok(KangxiRadicals::KangxiRadicalDeath),
            '⽎' => Ok(KangxiRadicals::KangxiRadicalWeapon),
            '⽏' => Ok(KangxiRadicals::KangxiRadicalDoNot),
            '⽐' => Ok(KangxiRadicals::KangxiRadicalCompare),
            '⽑' => Ok(KangxiRadicals::KangxiRadicalFur),
            '⽒' => Ok(KangxiRadicals::KangxiRadicalClan),
            '⽓' => Ok(KangxiRadicals::KangxiRadicalSteam),
            '⽔' => Ok(KangxiRadicals::KangxiRadicalWater),
            '⽕' => Ok(KangxiRadicals::KangxiRadicalFire),
            '⽖' => Ok(KangxiRadicals::KangxiRadicalClaw),
            '⽗' => Ok(KangxiRadicals::KangxiRadicalFather),
            '⽘' => Ok(KangxiRadicals::KangxiRadicalDoubleX),
            '⽙' => Ok(KangxiRadicals::KangxiRadicalHalfTreeTrunk),
            '⽚' => Ok(KangxiRadicals::KangxiRadicalSlice),
            '⽛' => Ok(KangxiRadicals::KangxiRadicalFang),
            '⽜' => Ok(KangxiRadicals::KangxiRadicalCow),
            '⽝' => Ok(KangxiRadicals::KangxiRadicalDog),
            '⽞' => Ok(KangxiRadicals::KangxiRadicalProfound),
            '⽟' => Ok(KangxiRadicals::KangxiRadicalJade),
            '⽠' => Ok(KangxiRadicals::KangxiRadicalMelon),
            '⽡' => Ok(KangxiRadicals::KangxiRadicalTile),
            '⽢' => Ok(KangxiRadicals::KangxiRadicalSweet),
            '⽣' => Ok(KangxiRadicals::KangxiRadicalLife),
            '⽤' => Ok(KangxiRadicals::KangxiRadicalUse),
            '⽥' => Ok(KangxiRadicals::KangxiRadicalField),
            '⽦' => Ok(KangxiRadicals::KangxiRadicalBoltOfCloth),
            '⽧' => Ok(KangxiRadicals::KangxiRadicalSickness),
            '⽨' => Ok(KangxiRadicals::KangxiRadicalDottedTent),
            '⽩' => Ok(KangxiRadicals::KangxiRadicalWhite),
            '⽪' => Ok(KangxiRadicals::KangxiRadicalSkin),
            '⽫' => Ok(KangxiRadicals::KangxiRadicalDish),
            '⽬' => Ok(KangxiRadicals::KangxiRadicalEye),
            '⽭' => Ok(KangxiRadicals::KangxiRadicalSpear),
            '⽮' => Ok(KangxiRadicals::KangxiRadicalArrow),
            '⽯' => Ok(KangxiRadicals::KangxiRadicalStone),
            '⽰' => Ok(KangxiRadicals::KangxiRadicalSpirit),
            '⽱' => Ok(KangxiRadicals::KangxiRadicalTrack),
            '⽲' => Ok(KangxiRadicals::KangxiRadicalGrain),
            '⽳' => Ok(KangxiRadicals::KangxiRadicalCave),
            '⽴' => Ok(KangxiRadicals::KangxiRadicalStand),
            '⽵' => Ok(KangxiRadicals::KangxiRadicalBamboo),
            '⽶' => Ok(KangxiRadicals::KangxiRadicalRice),
            '⽷' => Ok(KangxiRadicals::KangxiRadicalSilk),
            '⽸' => Ok(KangxiRadicals::KangxiRadicalJar),
            '⽹' => Ok(KangxiRadicals::KangxiRadicalNet),
            '⽺' => Ok(KangxiRadicals::KangxiRadicalSheep),
            '⽻' => Ok(KangxiRadicals::KangxiRadicalFeather),
            '⽼' => Ok(KangxiRadicals::KangxiRadicalOld),
            '⽽' => Ok(KangxiRadicals::KangxiRadicalAnd),
            '⽾' => Ok(KangxiRadicals::KangxiRadicalPlow),
            '⽿' => Ok(KangxiRadicals::KangxiRadicalEar),
            '⾀' => Ok(KangxiRadicals::KangxiRadicalBrush),
            '⾁' => Ok(KangxiRadicals::KangxiRadicalMeat),
            '⾂' => Ok(KangxiRadicals::KangxiRadicalMinister),
            '⾃' => Ok(KangxiRadicals::KangxiRadicalSelf),
            '⾄' => Ok(KangxiRadicals::KangxiRadicalArrive),
            '⾅' => Ok(KangxiRadicals::KangxiRadicalMortar),
            '⾆' => Ok(KangxiRadicals::KangxiRadicalTongue),
            '⾇' => Ok(KangxiRadicals::KangxiRadicalOppose),
            '⾈' => Ok(KangxiRadicals::KangxiRadicalBoat),
            '⾉' => Ok(KangxiRadicals::KangxiRadicalStopping),
            '⾊' => Ok(KangxiRadicals::KangxiRadicalColor),
            '⾋' => Ok(KangxiRadicals::KangxiRadicalGrass),
            '⾌' => Ok(KangxiRadicals::KangxiRadicalTiger),
            '⾍' => Ok(KangxiRadicals::KangxiRadicalInsect),
            '⾎' => Ok(KangxiRadicals::KangxiRadicalBlood),
            '⾏' => Ok(KangxiRadicals::KangxiRadicalWalkEnclosure),
            '⾐' => Ok(KangxiRadicals::KangxiRadicalClothes),
            '⾑' => Ok(KangxiRadicals::KangxiRadicalWest),
            '⾒' => Ok(KangxiRadicals::KangxiRadicalSee),
            '⾓' => Ok(KangxiRadicals::KangxiRadicalHorn),
            '⾔' => Ok(KangxiRadicals::KangxiRadicalSpeech),
            '⾕' => Ok(KangxiRadicals::KangxiRadicalValley),
            '⾖' => Ok(KangxiRadicals::KangxiRadicalBean),
            '⾗' => Ok(KangxiRadicals::KangxiRadicalPig),
            '⾘' => Ok(KangxiRadicals::KangxiRadicalBadger),
            '⾙' => Ok(KangxiRadicals::KangxiRadicalShell),
            '⾚' => Ok(KangxiRadicals::KangxiRadicalRed),
            '⾛' => Ok(KangxiRadicals::KangxiRadicalRun),
            '⾜' => Ok(KangxiRadicals::KangxiRadicalFoot),
            '⾝' => Ok(KangxiRadicals::KangxiRadicalBody),
            '⾞' => Ok(KangxiRadicals::KangxiRadicalCart),
            '⾟' => Ok(KangxiRadicals::KangxiRadicalBitter),
            '⾠' => Ok(KangxiRadicals::KangxiRadicalMorning),
            '⾡' => Ok(KangxiRadicals::KangxiRadicalWalk),
            '⾢' => Ok(KangxiRadicals::KangxiRadicalCity),
            '⾣' => Ok(KangxiRadicals::KangxiRadicalWine),
            '⾤' => Ok(KangxiRadicals::KangxiRadicalDistinguish),
            '⾥' => Ok(KangxiRadicals::KangxiRadicalVillage),
            '⾦' => Ok(KangxiRadicals::KangxiRadicalGold),
            '⾧' => Ok(KangxiRadicals::KangxiRadicalLong),
            '⾨' => Ok(KangxiRadicals::KangxiRadicalGate),
            '⾩' => Ok(KangxiRadicals::KangxiRadicalMound),
            '⾪' => Ok(KangxiRadicals::KangxiRadicalSlave),
            '⾫' => Ok(KangxiRadicals::KangxiRadicalShortTailedBird),
            '⾬' => Ok(KangxiRadicals::KangxiRadicalRain),
            '⾭' => Ok(KangxiRadicals::KangxiRadicalBlue),
            '⾮' => Ok(KangxiRadicals::KangxiRadicalWrong),
            '⾯' => Ok(KangxiRadicals::KangxiRadicalFace),
            '⾰' => Ok(KangxiRadicals::KangxiRadicalLeather),
            '⾱' => Ok(KangxiRadicals::KangxiRadicalTannedLeather),
            '⾲' => Ok(KangxiRadicals::KangxiRadicalLeek),
            '⾳' => Ok(KangxiRadicals::KangxiRadicalSound),
            '⾴' => Ok(KangxiRadicals::KangxiRadicalLeaf),
            '⾵' => Ok(KangxiRadicals::KangxiRadicalWind),
            '⾶' => Ok(KangxiRadicals::KangxiRadicalFly),
            '⾷' => Ok(KangxiRadicals::KangxiRadicalEat),
            '⾸' => Ok(KangxiRadicals::KangxiRadicalHead),
            '⾹' => Ok(KangxiRadicals::KangxiRadicalFragrant),
            '⾺' => Ok(KangxiRadicals::KangxiRadicalHorse),
            '⾻' => Ok(KangxiRadicals::KangxiRadicalBone),
            '⾼' => Ok(KangxiRadicals::KangxiRadicalTall),
            '⾽' => Ok(KangxiRadicals::KangxiRadicalHair),
            '⾾' => Ok(KangxiRadicals::KangxiRadicalFight),
            '⾿' => Ok(KangxiRadicals::KangxiRadicalSacrificialWine),
            '⿀' => Ok(KangxiRadicals::KangxiRadicalCauldron),
            '⿁' => Ok(KangxiRadicals::KangxiRadicalGhost),
            '⿂' => Ok(KangxiRadicals::KangxiRadicalFish),
            '⿃' => Ok(KangxiRadicals::KangxiRadicalBird),
            '⿄' => Ok(KangxiRadicals::KangxiRadicalSalt),
            '⿅' => Ok(KangxiRadicals::KangxiRadicalDeer),
            '⿆' => Ok(KangxiRadicals::KangxiRadicalWheat),
            '⿇' => Ok(KangxiRadicals::KangxiRadicalHemp),
            '⿈' => Ok(KangxiRadicals::KangxiRadicalYellow),
            '⿉' => Ok(KangxiRadicals::KangxiRadicalMillet),
            '⿊' => Ok(KangxiRadicals::KangxiRadicalBlack),
            '⿋' => Ok(KangxiRadicals::KangxiRadicalEmbroidery),
            '⿌' => Ok(KangxiRadicals::KangxiRadicalFrog),
            '⿍' => Ok(KangxiRadicals::KangxiRadicalTripod),
            '⿎' => Ok(KangxiRadicals::KangxiRadicalDrum),
            '⿏' => Ok(KangxiRadicals::KangxiRadicalRat),
            '⿐' => Ok(KangxiRadicals::KangxiRadicalNose),
            '⿑' => Ok(KangxiRadicals::KangxiRadicalEven),
            '⿒' => Ok(KangxiRadicals::KangxiRadicalTooth),
            '⿓' => Ok(KangxiRadicals::KangxiRadicalDragon),
            '⿔' => Ok(KangxiRadicals::KangxiRadicalTurtle),
            '⿕' => Ok(KangxiRadicals::KangxiRadicalFlute),
            _ => Err(()),
        }
    }
}

impl Into<u32> for KangxiRadicals {
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

impl std::convert::TryFrom<u32> for KangxiRadicals {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for KangxiRadicals {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl KangxiRadicals {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        KangxiRadicals::KangxiRadicalOne
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            KangxiRadicals::KangxiRadicalOne => "kangxi radical one",
            KangxiRadicals::KangxiRadicalLine => "kangxi radical line",
            KangxiRadicals::KangxiRadicalDot => "kangxi radical dot",
            KangxiRadicals::KangxiRadicalSlash => "kangxi radical slash",
            KangxiRadicals::KangxiRadicalSecond => "kangxi radical second",
            KangxiRadicals::KangxiRadicalHook => "kangxi radical hook",
            KangxiRadicals::KangxiRadicalTwo => "kangxi radical two",
            KangxiRadicals::KangxiRadicalLid => "kangxi radical lid",
            KangxiRadicals::KangxiRadicalMan => "kangxi radical man",
            KangxiRadicals::KangxiRadicalLegs => "kangxi radical legs",
            KangxiRadicals::KangxiRadicalEnter => "kangxi radical enter",
            KangxiRadicals::KangxiRadicalEight => "kangxi radical eight",
            KangxiRadicals::KangxiRadicalDownBox => "kangxi radical down box",
            KangxiRadicals::KangxiRadicalCover => "kangxi radical cover",
            KangxiRadicals::KangxiRadicalIce => "kangxi radical ice",
            KangxiRadicals::KangxiRadicalTable => "kangxi radical table",
            KangxiRadicals::KangxiRadicalOpenBox => "kangxi radical open box",
            KangxiRadicals::KangxiRadicalKnife => "kangxi radical knife",
            KangxiRadicals::KangxiRadicalPower => "kangxi radical power",
            KangxiRadicals::KangxiRadicalWrap => "kangxi radical wrap",
            KangxiRadicals::KangxiRadicalSpoon => "kangxi radical spoon",
            KangxiRadicals::KangxiRadicalRightOpenBox => "kangxi radical right open box",
            KangxiRadicals::KangxiRadicalHidingEnclosure => "kangxi radical hiding enclosure",
            KangxiRadicals::KangxiRadicalTen => "kangxi radical ten",
            KangxiRadicals::KangxiRadicalDivination => "kangxi radical divination",
            KangxiRadicals::KangxiRadicalSeal => "kangxi radical seal",
            KangxiRadicals::KangxiRadicalCliff => "kangxi radical cliff",
            KangxiRadicals::KangxiRadicalPrivate => "kangxi radical private",
            KangxiRadicals::KangxiRadicalAgain => "kangxi radical again",
            KangxiRadicals::KangxiRadicalMouth => "kangxi radical mouth",
            KangxiRadicals::KangxiRadicalEnclosure => "kangxi radical enclosure",
            KangxiRadicals::KangxiRadicalEarth => "kangxi radical earth",
            KangxiRadicals::KangxiRadicalScholar => "kangxi radical scholar",
            KangxiRadicals::KangxiRadicalGo => "kangxi radical go",
            KangxiRadicals::KangxiRadicalGoSlowly => "kangxi radical go slowly",
            KangxiRadicals::KangxiRadicalEvening => "kangxi radical evening",
            KangxiRadicals::KangxiRadicalBig => "kangxi radical big",
            KangxiRadicals::KangxiRadicalWoman => "kangxi radical woman",
            KangxiRadicals::KangxiRadicalChild => "kangxi radical child",
            KangxiRadicals::KangxiRadicalRoof => "kangxi radical roof",
            KangxiRadicals::KangxiRadicalInch => "kangxi radical inch",
            KangxiRadicals::KangxiRadicalSmall => "kangxi radical small",
            KangxiRadicals::KangxiRadicalLame => "kangxi radical lame",
            KangxiRadicals::KangxiRadicalCorpse => "kangxi radical corpse",
            KangxiRadicals::KangxiRadicalSprout => "kangxi radical sprout",
            KangxiRadicals::KangxiRadicalMountain => "kangxi radical mountain",
            KangxiRadicals::KangxiRadicalRiver => "kangxi radical river",
            KangxiRadicals::KangxiRadicalWork => "kangxi radical work",
            KangxiRadicals::KangxiRadicalOneself => "kangxi radical oneself",
            KangxiRadicals::KangxiRadicalTurban => "kangxi radical turban",
            KangxiRadicals::KangxiRadicalDry => "kangxi radical dry",
            KangxiRadicals::KangxiRadicalShortThread => "kangxi radical short thread",
            KangxiRadicals::KangxiRadicalDottedCliff => "kangxi radical dotted cliff",
            KangxiRadicals::KangxiRadicalLongStride => "kangxi radical long stride",
            KangxiRadicals::KangxiRadicalTwoHands => "kangxi radical two hands",
            KangxiRadicals::KangxiRadicalShoot => "kangxi radical shoot",
            KangxiRadicals::KangxiRadicalBow => "kangxi radical bow",
            KangxiRadicals::KangxiRadicalSnout => "kangxi radical snout",
            KangxiRadicals::KangxiRadicalBristle => "kangxi radical bristle",
            KangxiRadicals::KangxiRadicalStep => "kangxi radical step",
            KangxiRadicals::KangxiRadicalHeart => "kangxi radical heart",
            KangxiRadicals::KangxiRadicalHalberd => "kangxi radical halberd",
            KangxiRadicals::KangxiRadicalDoor => "kangxi radical door",
            KangxiRadicals::KangxiRadicalHand => "kangxi radical hand",
            KangxiRadicals::KangxiRadicalBranch => "kangxi radical branch",
            KangxiRadicals::KangxiRadicalRap => "kangxi radical rap",
            KangxiRadicals::KangxiRadicalScript => "kangxi radical script",
            KangxiRadicals::KangxiRadicalDipper => "kangxi radical dipper",
            KangxiRadicals::KangxiRadicalAxe => "kangxi radical axe",
            KangxiRadicals::KangxiRadicalSquare => "kangxi radical square",
            KangxiRadicals::KangxiRadicalNot => "kangxi radical not",
            KangxiRadicals::KangxiRadicalSun => "kangxi radical sun",
            KangxiRadicals::KangxiRadicalSay => "kangxi radical say",
            KangxiRadicals::KangxiRadicalMoon => "kangxi radical moon",
            KangxiRadicals::KangxiRadicalTree => "kangxi radical tree",
            KangxiRadicals::KangxiRadicalLack => "kangxi radical lack",
            KangxiRadicals::KangxiRadicalStop => "kangxi radical stop",
            KangxiRadicals::KangxiRadicalDeath => "kangxi radical death",
            KangxiRadicals::KangxiRadicalWeapon => "kangxi radical weapon",
            KangxiRadicals::KangxiRadicalDoNot => "kangxi radical do not",
            KangxiRadicals::KangxiRadicalCompare => "kangxi radical compare",
            KangxiRadicals::KangxiRadicalFur => "kangxi radical fur",
            KangxiRadicals::KangxiRadicalClan => "kangxi radical clan",
            KangxiRadicals::KangxiRadicalSteam => "kangxi radical steam",
            KangxiRadicals::KangxiRadicalWater => "kangxi radical water",
            KangxiRadicals::KangxiRadicalFire => "kangxi radical fire",
            KangxiRadicals::KangxiRadicalClaw => "kangxi radical claw",
            KangxiRadicals::KangxiRadicalFather => "kangxi radical father",
            KangxiRadicals::KangxiRadicalDoubleX => "kangxi radical double x",
            KangxiRadicals::KangxiRadicalHalfTreeTrunk => "kangxi radical half tree trunk",
            KangxiRadicals::KangxiRadicalSlice => "kangxi radical slice",
            KangxiRadicals::KangxiRadicalFang => "kangxi radical fang",
            KangxiRadicals::KangxiRadicalCow => "kangxi radical cow",
            KangxiRadicals::KangxiRadicalDog => "kangxi radical dog",
            KangxiRadicals::KangxiRadicalProfound => "kangxi radical profound",
            KangxiRadicals::KangxiRadicalJade => "kangxi radical jade",
            KangxiRadicals::KangxiRadicalMelon => "kangxi radical melon",
            KangxiRadicals::KangxiRadicalTile => "kangxi radical tile",
            KangxiRadicals::KangxiRadicalSweet => "kangxi radical sweet",
            KangxiRadicals::KangxiRadicalLife => "kangxi radical life",
            KangxiRadicals::KangxiRadicalUse => "kangxi radical use",
            KangxiRadicals::KangxiRadicalField => "kangxi radical field",
            KangxiRadicals::KangxiRadicalBoltOfCloth => "kangxi radical bolt of cloth",
            KangxiRadicals::KangxiRadicalSickness => "kangxi radical sickness",
            KangxiRadicals::KangxiRadicalDottedTent => "kangxi radical dotted tent",
            KangxiRadicals::KangxiRadicalWhite => "kangxi radical white",
            KangxiRadicals::KangxiRadicalSkin => "kangxi radical skin",
            KangxiRadicals::KangxiRadicalDish => "kangxi radical dish",
            KangxiRadicals::KangxiRadicalEye => "kangxi radical eye",
            KangxiRadicals::KangxiRadicalSpear => "kangxi radical spear",
            KangxiRadicals::KangxiRadicalArrow => "kangxi radical arrow",
            KangxiRadicals::KangxiRadicalStone => "kangxi radical stone",
            KangxiRadicals::KangxiRadicalSpirit => "kangxi radical spirit",
            KangxiRadicals::KangxiRadicalTrack => "kangxi radical track",
            KangxiRadicals::KangxiRadicalGrain => "kangxi radical grain",
            KangxiRadicals::KangxiRadicalCave => "kangxi radical cave",
            KangxiRadicals::KangxiRadicalStand => "kangxi radical stand",
            KangxiRadicals::KangxiRadicalBamboo => "kangxi radical bamboo",
            KangxiRadicals::KangxiRadicalRice => "kangxi radical rice",
            KangxiRadicals::KangxiRadicalSilk => "kangxi radical silk",
            KangxiRadicals::KangxiRadicalJar => "kangxi radical jar",
            KangxiRadicals::KangxiRadicalNet => "kangxi radical net",
            KangxiRadicals::KangxiRadicalSheep => "kangxi radical sheep",
            KangxiRadicals::KangxiRadicalFeather => "kangxi radical feather",
            KangxiRadicals::KangxiRadicalOld => "kangxi radical old",
            KangxiRadicals::KangxiRadicalAnd => "kangxi radical and",
            KangxiRadicals::KangxiRadicalPlow => "kangxi radical plow",
            KangxiRadicals::KangxiRadicalEar => "kangxi radical ear",
            KangxiRadicals::KangxiRadicalBrush => "kangxi radical brush",
            KangxiRadicals::KangxiRadicalMeat => "kangxi radical meat",
            KangxiRadicals::KangxiRadicalMinister => "kangxi radical minister",
            KangxiRadicals::KangxiRadicalSelf => "kangxi radical self",
            KangxiRadicals::KangxiRadicalArrive => "kangxi radical arrive",
            KangxiRadicals::KangxiRadicalMortar => "kangxi radical mortar",
            KangxiRadicals::KangxiRadicalTongue => "kangxi radical tongue",
            KangxiRadicals::KangxiRadicalOppose => "kangxi radical oppose",
            KangxiRadicals::KangxiRadicalBoat => "kangxi radical boat",
            KangxiRadicals::KangxiRadicalStopping => "kangxi radical stopping",
            KangxiRadicals::KangxiRadicalColor => "kangxi radical color",
            KangxiRadicals::KangxiRadicalGrass => "kangxi radical grass",
            KangxiRadicals::KangxiRadicalTiger => "kangxi radical tiger",
            KangxiRadicals::KangxiRadicalInsect => "kangxi radical insect",
            KangxiRadicals::KangxiRadicalBlood => "kangxi radical blood",
            KangxiRadicals::KangxiRadicalWalkEnclosure => "kangxi radical walk enclosure",
            KangxiRadicals::KangxiRadicalClothes => "kangxi radical clothes",
            KangxiRadicals::KangxiRadicalWest => "kangxi radical west",
            KangxiRadicals::KangxiRadicalSee => "kangxi radical see",
            KangxiRadicals::KangxiRadicalHorn => "kangxi radical horn",
            KangxiRadicals::KangxiRadicalSpeech => "kangxi radical speech",
            KangxiRadicals::KangxiRadicalValley => "kangxi radical valley",
            KangxiRadicals::KangxiRadicalBean => "kangxi radical bean",
            KangxiRadicals::KangxiRadicalPig => "kangxi radical pig",
            KangxiRadicals::KangxiRadicalBadger => "kangxi radical badger",
            KangxiRadicals::KangxiRadicalShell => "kangxi radical shell",
            KangxiRadicals::KangxiRadicalRed => "kangxi radical red",
            KangxiRadicals::KangxiRadicalRun => "kangxi radical run",
            KangxiRadicals::KangxiRadicalFoot => "kangxi radical foot",
            KangxiRadicals::KangxiRadicalBody => "kangxi radical body",
            KangxiRadicals::KangxiRadicalCart => "kangxi radical cart",
            KangxiRadicals::KangxiRadicalBitter => "kangxi radical bitter",
            KangxiRadicals::KangxiRadicalMorning => "kangxi radical morning",
            KangxiRadicals::KangxiRadicalWalk => "kangxi radical walk",
            KangxiRadicals::KangxiRadicalCity => "kangxi radical city",
            KangxiRadicals::KangxiRadicalWine => "kangxi radical wine",
            KangxiRadicals::KangxiRadicalDistinguish => "kangxi radical distinguish",
            KangxiRadicals::KangxiRadicalVillage => "kangxi radical village",
            KangxiRadicals::KangxiRadicalGold => "kangxi radical gold",
            KangxiRadicals::KangxiRadicalLong => "kangxi radical long",
            KangxiRadicals::KangxiRadicalGate => "kangxi radical gate",
            KangxiRadicals::KangxiRadicalMound => "kangxi radical mound",
            KangxiRadicals::KangxiRadicalSlave => "kangxi radical slave",
            KangxiRadicals::KangxiRadicalShortTailedBird => "kangxi radical short tailed bird",
            KangxiRadicals::KangxiRadicalRain => "kangxi radical rain",
            KangxiRadicals::KangxiRadicalBlue => "kangxi radical blue",
            KangxiRadicals::KangxiRadicalWrong => "kangxi radical wrong",
            KangxiRadicals::KangxiRadicalFace => "kangxi radical face",
            KangxiRadicals::KangxiRadicalLeather => "kangxi radical leather",
            KangxiRadicals::KangxiRadicalTannedLeather => "kangxi radical tanned leather",
            KangxiRadicals::KangxiRadicalLeek => "kangxi radical leek",
            KangxiRadicals::KangxiRadicalSound => "kangxi radical sound",
            KangxiRadicals::KangxiRadicalLeaf => "kangxi radical leaf",
            KangxiRadicals::KangxiRadicalWind => "kangxi radical wind",
            KangxiRadicals::KangxiRadicalFly => "kangxi radical fly",
            KangxiRadicals::KangxiRadicalEat => "kangxi radical eat",
            KangxiRadicals::KangxiRadicalHead => "kangxi radical head",
            KangxiRadicals::KangxiRadicalFragrant => "kangxi radical fragrant",
            KangxiRadicals::KangxiRadicalHorse => "kangxi radical horse",
            KangxiRadicals::KangxiRadicalBone => "kangxi radical bone",
            KangxiRadicals::KangxiRadicalTall => "kangxi radical tall",
            KangxiRadicals::KangxiRadicalHair => "kangxi radical hair",
            KangxiRadicals::KangxiRadicalFight => "kangxi radical fight",
            KangxiRadicals::KangxiRadicalSacrificialWine => "kangxi radical sacrificial wine",
            KangxiRadicals::KangxiRadicalCauldron => "kangxi radical cauldron",
            KangxiRadicals::KangxiRadicalGhost => "kangxi radical ghost",
            KangxiRadicals::KangxiRadicalFish => "kangxi radical fish",
            KangxiRadicals::KangxiRadicalBird => "kangxi radical bird",
            KangxiRadicals::KangxiRadicalSalt => "kangxi radical salt",
            KangxiRadicals::KangxiRadicalDeer => "kangxi radical deer",
            KangxiRadicals::KangxiRadicalWheat => "kangxi radical wheat",
            KangxiRadicals::KangxiRadicalHemp => "kangxi radical hemp",
            KangxiRadicals::KangxiRadicalYellow => "kangxi radical yellow",
            KangxiRadicals::KangxiRadicalMillet => "kangxi radical millet",
            KangxiRadicals::KangxiRadicalBlack => "kangxi radical black",
            KangxiRadicals::KangxiRadicalEmbroidery => "kangxi radical embroidery",
            KangxiRadicals::KangxiRadicalFrog => "kangxi radical frog",
            KangxiRadicals::KangxiRadicalTripod => "kangxi radical tripod",
            KangxiRadicals::KangxiRadicalDrum => "kangxi radical drum",
            KangxiRadicals::KangxiRadicalRat => "kangxi radical rat",
            KangxiRadicals::KangxiRadicalNose => "kangxi radical nose",
            KangxiRadicals::KangxiRadicalEven => "kangxi radical even",
            KangxiRadicals::KangxiRadicalTooth => "kangxi radical tooth",
            KangxiRadicals::KangxiRadicalDragon => "kangxi radical dragon",
            KangxiRadicals::KangxiRadicalTurtle => "kangxi radical turtle",
            KangxiRadicals::KangxiRadicalFlute => "kangxi radical flute",
        }
    }
}
