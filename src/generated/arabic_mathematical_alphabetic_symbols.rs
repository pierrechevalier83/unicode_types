
/// An enum to represent all characters in the ArabicMathematicalAlphabeticSymbols block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ArabicMathematicalAlphabeticSymbols {
    /// \u{1ee00}: '𞸀'
    ArabicMathematicalAlef,
    /// \u{1ee01}: '𞸁'
    ArabicMathematicalBeh,
    /// \u{1ee02}: '𞸂'
    ArabicMathematicalJeem,
    /// \u{1ee03}: '𞸃'
    ArabicMathematicalDal,
    /// \u{1ee05}: '𞸅'
    ArabicMathematicalWaw,
    /// \u{1ee06}: '𞸆'
    ArabicMathematicalZain,
    /// \u{1ee07}: '𞸇'
    ArabicMathematicalHah,
    /// \u{1ee08}: '𞸈'
    ArabicMathematicalTah,
    /// \u{1ee09}: '𞸉'
    ArabicMathematicalYeh,
    /// \u{1ee0a}: '𞸊'
    ArabicMathematicalKaf,
    /// \u{1ee0b}: '𞸋'
    ArabicMathematicalLam,
    /// \u{1ee0c}: '𞸌'
    ArabicMathematicalMeem,
    /// \u{1ee0d}: '𞸍'
    ArabicMathematicalNoon,
    /// \u{1ee0e}: '𞸎'
    ArabicMathematicalSeen,
    /// \u{1ee0f}: '𞸏'
    ArabicMathematicalAin,
    /// \u{1ee10}: '𞸐'
    ArabicMathematicalFeh,
    /// \u{1ee11}: '𞸑'
    ArabicMathematicalSad,
    /// \u{1ee12}: '𞸒'
    ArabicMathematicalQaf,
    /// \u{1ee13}: '𞸓'
    ArabicMathematicalReh,
    /// \u{1ee14}: '𞸔'
    ArabicMathematicalSheen,
    /// \u{1ee15}: '𞸕'
    ArabicMathematicalTeh,
    /// \u{1ee16}: '𞸖'
    ArabicMathematicalTheh,
    /// \u{1ee17}: '𞸗'
    ArabicMathematicalKhah,
    /// \u{1ee18}: '𞸘'
    ArabicMathematicalThal,
    /// \u{1ee19}: '𞸙'
    ArabicMathematicalDad,
    /// \u{1ee1a}: '𞸚'
    ArabicMathematicalZah,
    /// \u{1ee1b}: '𞸛'
    ArabicMathematicalGhain,
    /// \u{1ee1c}: '𞸜'
    ArabicMathematicalDotlessBeh,
    /// \u{1ee1d}: '𞸝'
    ArabicMathematicalDotlessNoon,
    /// \u{1ee1e}: '𞸞'
    ArabicMathematicalDotlessFeh,
    /// \u{1ee1f}: '𞸟'
    ArabicMathematicalDotlessQaf,
    /// \u{1ee21}: '𞸡'
    ArabicMathematicalInitialBeh,
    /// \u{1ee22}: '𞸢'
    ArabicMathematicalInitialJeem,
    /// \u{1ee24}: '𞸤'
    ArabicMathematicalInitialHeh,
    /// \u{1ee27}: '𞸧'
    ArabicMathematicalInitialHah,
    /// \u{1ee29}: '𞸩'
    ArabicMathematicalInitialYeh,
    /// \u{1ee2a}: '𞸪'
    ArabicMathematicalInitialKaf,
    /// \u{1ee2b}: '𞸫'
    ArabicMathematicalInitialLam,
    /// \u{1ee2c}: '𞸬'
    ArabicMathematicalInitialMeem,
    /// \u{1ee2d}: '𞸭'
    ArabicMathematicalInitialNoon,
    /// \u{1ee2e}: '𞸮'
    ArabicMathematicalInitialSeen,
    /// \u{1ee2f}: '𞸯'
    ArabicMathematicalInitialAin,
    /// \u{1ee30}: '𞸰'
    ArabicMathematicalInitialFeh,
    /// \u{1ee31}: '𞸱'
    ArabicMathematicalInitialSad,
    /// \u{1ee32}: '𞸲'
    ArabicMathematicalInitialQaf,
    /// \u{1ee34}: '𞸴'
    ArabicMathematicalInitialSheen,
    /// \u{1ee35}: '𞸵'
    ArabicMathematicalInitialTeh,
    /// \u{1ee36}: '𞸶'
    ArabicMathematicalInitialTheh,
    /// \u{1ee37}: '𞸷'
    ArabicMathematicalInitialKhah,
    /// \u{1ee39}: '𞸹'
    ArabicMathematicalInitialDad,
    /// \u{1ee3b}: '𞸻'
    ArabicMathematicalInitialGhain,
    /// \u{1ee42}: '𞹂'
    ArabicMathematicalTailedJeem,
    /// \u{1ee47}: '𞹇'
    ArabicMathematicalTailedHah,
    /// \u{1ee49}: '𞹉'
    ArabicMathematicalTailedYeh,
    /// \u{1ee4b}: '𞹋'
    ArabicMathematicalTailedLam,
    /// \u{1ee4d}: '𞹍'
    ArabicMathematicalTailedNoon,
    /// \u{1ee4e}: '𞹎'
    ArabicMathematicalTailedSeen,
    /// \u{1ee4f}: '𞹏'
    ArabicMathematicalTailedAin,
    /// \u{1ee51}: '𞹑'
    ArabicMathematicalTailedSad,
    /// \u{1ee52}: '𞹒'
    ArabicMathematicalTailedQaf,
    /// \u{1ee54}: '𞹔'
    ArabicMathematicalTailedSheen,
    /// \u{1ee57}: '𞹗'
    ArabicMathematicalTailedKhah,
    /// \u{1ee59}: '𞹙'
    ArabicMathematicalTailedDad,
    /// \u{1ee5b}: '𞹛'
    ArabicMathematicalTailedGhain,
    /// \u{1ee5d}: '𞹝'
    ArabicMathematicalTailedDotlessNoon,
    /// \u{1ee5f}: '𞹟'
    ArabicMathematicalTailedDotlessQaf,
    /// \u{1ee61}: '𞹡'
    ArabicMathematicalStretchedBeh,
    /// \u{1ee62}: '𞹢'
    ArabicMathematicalStretchedJeem,
    /// \u{1ee64}: '𞹤'
    ArabicMathematicalStretchedHeh,
    /// \u{1ee67}: '𞹧'
    ArabicMathematicalStretchedHah,
    /// \u{1ee68}: '𞹨'
    ArabicMathematicalStretchedTah,
    /// \u{1ee69}: '𞹩'
    ArabicMathematicalStretchedYeh,
    /// \u{1ee6a}: '𞹪'
    ArabicMathematicalStretchedKaf,
    /// \u{1ee6c}: '𞹬'
    ArabicMathematicalStretchedMeem,
    /// \u{1ee6d}: '𞹭'
    ArabicMathematicalStretchedNoon,
    /// \u{1ee6e}: '𞹮'
    ArabicMathematicalStretchedSeen,
    /// \u{1ee6f}: '𞹯'
    ArabicMathematicalStretchedAin,
    /// \u{1ee70}: '𞹰'
    ArabicMathematicalStretchedFeh,
    /// \u{1ee71}: '𞹱'
    ArabicMathematicalStretchedSad,
    /// \u{1ee72}: '𞹲'
    ArabicMathematicalStretchedQaf,
    /// \u{1ee74}: '𞹴'
    ArabicMathematicalStretchedSheen,
    /// \u{1ee75}: '𞹵'
    ArabicMathematicalStretchedTeh,
    /// \u{1ee76}: '𞹶'
    ArabicMathematicalStretchedTheh,
    /// \u{1ee77}: '𞹷'
    ArabicMathematicalStretchedKhah,
    /// \u{1ee79}: '𞹹'
    ArabicMathematicalStretchedDad,
    /// \u{1ee7a}: '𞹺'
    ArabicMathematicalStretchedZah,
    /// \u{1ee7b}: '𞹻'
    ArabicMathematicalStretchedGhain,
    /// \u{1ee7c}: '𞹼'
    ArabicMathematicalStretchedDotlessBeh,
    /// \u{1ee7e}: '𞹾'
    ArabicMathematicalStretchedDotlessFeh,
    /// \u{1ee80}: '𞺀'
    ArabicMathematicalLoopedAlef,
    /// \u{1ee81}: '𞺁'
    ArabicMathematicalLoopedBeh,
    /// \u{1ee82}: '𞺂'
    ArabicMathematicalLoopedJeem,
    /// \u{1ee83}: '𞺃'
    ArabicMathematicalLoopedDal,
    /// \u{1ee84}: '𞺄'
    ArabicMathematicalLoopedHeh,
    /// \u{1ee85}: '𞺅'
    ArabicMathematicalLoopedWaw,
    /// \u{1ee86}: '𞺆'
    ArabicMathematicalLoopedZain,
    /// \u{1ee87}: '𞺇'
    ArabicMathematicalLoopedHah,
    /// \u{1ee88}: '𞺈'
    ArabicMathematicalLoopedTah,
    /// \u{1ee89}: '𞺉'
    ArabicMathematicalLoopedYeh,
    /// \u{1ee8b}: '𞺋'
    ArabicMathematicalLoopedLam,
    /// \u{1ee8c}: '𞺌'
    ArabicMathematicalLoopedMeem,
    /// \u{1ee8d}: '𞺍'
    ArabicMathematicalLoopedNoon,
    /// \u{1ee8e}: '𞺎'
    ArabicMathematicalLoopedSeen,
    /// \u{1ee8f}: '𞺏'
    ArabicMathematicalLoopedAin,
    /// \u{1ee90}: '𞺐'
    ArabicMathematicalLoopedFeh,
    /// \u{1ee91}: '𞺑'
    ArabicMathematicalLoopedSad,
    /// \u{1ee92}: '𞺒'
    ArabicMathematicalLoopedQaf,
    /// \u{1ee93}: '𞺓'
    ArabicMathematicalLoopedReh,
    /// \u{1ee94}: '𞺔'
    ArabicMathematicalLoopedSheen,
    /// \u{1ee95}: '𞺕'
    ArabicMathematicalLoopedTeh,
    /// \u{1ee96}: '𞺖'
    ArabicMathematicalLoopedTheh,
    /// \u{1ee97}: '𞺗'
    ArabicMathematicalLoopedKhah,
    /// \u{1ee98}: '𞺘'
    ArabicMathematicalLoopedThal,
    /// \u{1ee99}: '𞺙'
    ArabicMathematicalLoopedDad,
    /// \u{1ee9a}: '𞺚'
    ArabicMathematicalLoopedZah,
    /// \u{1ee9b}: '𞺛'
    ArabicMathematicalLoopedGhain,
    /// \u{1eea1}: '𞺡'
    ArabicMathematicalDoubleDashStruckBeh,
    /// \u{1eea2}: '𞺢'
    ArabicMathematicalDoubleDashStruckJeem,
    /// \u{1eea3}: '𞺣'
    ArabicMathematicalDoubleDashStruckDal,
    /// \u{1eea5}: '𞺥'
    ArabicMathematicalDoubleDashStruckWaw,
    /// \u{1eea6}: '𞺦'
    ArabicMathematicalDoubleDashStruckZain,
    /// \u{1eea7}: '𞺧'
    ArabicMathematicalDoubleDashStruckHah,
    /// \u{1eea8}: '𞺨'
    ArabicMathematicalDoubleDashStruckTah,
    /// \u{1eea9}: '𞺩'
    ArabicMathematicalDoubleDashStruckYeh,
    /// \u{1eeab}: '𞺫'
    ArabicMathematicalDoubleDashStruckLam,
    /// \u{1eeac}: '𞺬'
    ArabicMathematicalDoubleDashStruckMeem,
    /// \u{1eead}: '𞺭'
    ArabicMathematicalDoubleDashStruckNoon,
    /// \u{1eeae}: '𞺮'
    ArabicMathematicalDoubleDashStruckSeen,
    /// \u{1eeaf}: '𞺯'
    ArabicMathematicalDoubleDashStruckAin,
    /// \u{1eeb0}: '𞺰'
    ArabicMathematicalDoubleDashStruckFeh,
    /// \u{1eeb1}: '𞺱'
    ArabicMathematicalDoubleDashStruckSad,
    /// \u{1eeb2}: '𞺲'
    ArabicMathematicalDoubleDashStruckQaf,
    /// \u{1eeb3}: '𞺳'
    ArabicMathematicalDoubleDashStruckReh,
    /// \u{1eeb4}: '𞺴'
    ArabicMathematicalDoubleDashStruckSheen,
    /// \u{1eeb5}: '𞺵'
    ArabicMathematicalDoubleDashStruckTeh,
    /// \u{1eeb6}: '𞺶'
    ArabicMathematicalDoubleDashStruckTheh,
    /// \u{1eeb7}: '𞺷'
    ArabicMathematicalDoubleDashStruckKhah,
    /// \u{1eeb8}: '𞺸'
    ArabicMathematicalDoubleDashStruckThal,
    /// \u{1eeb9}: '𞺹'
    ArabicMathematicalDoubleDashStruckDad,
    /// \u{1eeba}: '𞺺'
    ArabicMathematicalDoubleDashStruckZah,
    /// \u{1eebb}: '𞺻'
    ArabicMathematicalDoubleDashStruckGhain,
    /// \u{1eef0}: '𞻰'
    ArabicMathematicalOperatorMeemWithHahWithTatweel,
    /// \u{1eef1}: '𞻱'
    ArabicMathematicalOperatorHahWithDal,
}

impl Into<char> for ArabicMathematicalAlphabeticSymbols {
    fn into(self) -> char {
        match self {
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalAlef => '𞸀',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalBeh => '𞸁',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalJeem => '𞸂',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDal => '𞸃',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalWaw => '𞸅',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalZain => '𞸆',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalHah => '𞸇',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTah => '𞸈',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalYeh => '𞸉',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalKaf => '𞸊',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLam => '𞸋',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalMeem => '𞸌',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalNoon => '𞸍',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalSeen => '𞸎',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalAin => '𞸏',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalFeh => '𞸐',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalSad => '𞸑',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalQaf => '𞸒',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalReh => '𞸓',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalSheen => '𞸔',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTeh => '𞸕',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTheh => '𞸖',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalKhah => '𞸗',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalThal => '𞸘',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDad => '𞸙',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalZah => '𞸚',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalGhain => '𞸛',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDotlessBeh => '𞸜',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDotlessNoon => '𞸝',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDotlessFeh => '𞸞',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDotlessQaf => '𞸟',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialBeh => '𞸡',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialJeem => '𞸢',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialHeh => '𞸤',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialHah => '𞸧',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialYeh => '𞸩',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialKaf => '𞸪',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialLam => '𞸫',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialMeem => '𞸬',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialNoon => '𞸭',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialSeen => '𞸮',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialAin => '𞸯',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialFeh => '𞸰',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialSad => '𞸱',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialQaf => '𞸲',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialSheen => '𞸴',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialTeh => '𞸵',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialTheh => '𞸶',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialKhah => '𞸷',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialDad => '𞸹',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialGhain => '𞸻',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTailedJeem => '𞹂',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTailedHah => '𞹇',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTailedYeh => '𞹉',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTailedLam => '𞹋',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTailedNoon => '𞹍',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTailedSeen => '𞹎',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTailedAin => '𞹏',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTailedSad => '𞹑',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTailedQaf => '𞹒',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTailedSheen => '𞹔',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTailedKhah => '𞹗',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTailedDad => '𞹙',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTailedGhain => '𞹛',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTailedDotlessNoon => '𞹝',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTailedDotlessQaf => '𞹟',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedBeh => '𞹡',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedJeem => '𞹢',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedHeh => '𞹤',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedHah => '𞹧',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedTah => '𞹨',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedYeh => '𞹩',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedKaf => '𞹪',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedMeem => '𞹬',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedNoon => '𞹭',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedSeen => '𞹮',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedAin => '𞹯',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedFeh => '𞹰',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedSad => '𞹱',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedQaf => '𞹲',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedSheen => '𞹴',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedTeh => '𞹵',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedTheh => '𞹶',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedKhah => '𞹷',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedDad => '𞹹',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedZah => '𞹺',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedGhain => '𞹻',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedDotlessBeh => '𞹼',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedDotlessFeh => '𞹾',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedAlef => '𞺀',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedBeh => '𞺁',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedJeem => '𞺂',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedDal => '𞺃',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedHeh => '𞺄',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedWaw => '𞺅',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedZain => '𞺆',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedHah => '𞺇',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedTah => '𞺈',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedYeh => '𞺉',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedLam => '𞺋',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedMeem => '𞺌',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedNoon => '𞺍',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedSeen => '𞺎',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedAin => '𞺏',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedFeh => '𞺐',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedSad => '𞺑',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedQaf => '𞺒',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedReh => '𞺓',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedSheen => '𞺔',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedTeh => '𞺕',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedTheh => '𞺖',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedKhah => '𞺗',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedThal => '𞺘',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedDad => '𞺙',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedZah => '𞺚',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedGhain => '𞺛',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckBeh => '𞺡',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckJeem => '𞺢',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckDal => '𞺣',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckWaw => '𞺥',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckZain => '𞺦',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckHah => '𞺧',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckTah => '𞺨',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckYeh => '𞺩',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckLam => '𞺫',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckMeem => '𞺬',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckNoon => '𞺭',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckSeen => '𞺮',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckAin => '𞺯',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckFeh => '𞺰',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckSad => '𞺱',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckQaf => '𞺲',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckReh => '𞺳',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckSheen => '𞺴',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckTeh => '𞺵',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckTheh => '𞺶',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckKhah => '𞺷',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckThal => '𞺸',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckDad => '𞺹',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckZah => '𞺺',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckGhain => '𞺻',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalOperatorMeemWithHahWithTatweel => '𞻰',
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalOperatorHahWithDal => '𞻱',
        }
    }
}

impl std::convert::TryFrom<char> for ArabicMathematicalAlphabeticSymbols {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𞸀' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalAlef),
            '𞸁' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalBeh),
            '𞸂' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalJeem),
            '𞸃' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDal),
            '𞸅' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalWaw),
            '𞸆' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalZain),
            '𞸇' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalHah),
            '𞸈' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTah),
            '𞸉' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalYeh),
            '𞸊' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalKaf),
            '𞸋' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLam),
            '𞸌' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalMeem),
            '𞸍' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalNoon),
            '𞸎' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalSeen),
            '𞸏' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalAin),
            '𞸐' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalFeh),
            '𞸑' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalSad),
            '𞸒' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalQaf),
            '𞸓' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalReh),
            '𞸔' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalSheen),
            '𞸕' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTeh),
            '𞸖' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTheh),
            '𞸗' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalKhah),
            '𞸘' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalThal),
            '𞸙' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDad),
            '𞸚' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalZah),
            '𞸛' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalGhain),
            '𞸜' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDotlessBeh),
            '𞸝' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDotlessNoon),
            '𞸞' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDotlessFeh),
            '𞸟' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDotlessQaf),
            '𞸡' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialBeh),
            '𞸢' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialJeem),
            '𞸤' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialHeh),
            '𞸧' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialHah),
            '𞸩' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialYeh),
            '𞸪' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialKaf),
            '𞸫' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialLam),
            '𞸬' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialMeem),
            '𞸭' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialNoon),
            '𞸮' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialSeen),
            '𞸯' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialAin),
            '𞸰' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialFeh),
            '𞸱' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialSad),
            '𞸲' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialQaf),
            '𞸴' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialSheen),
            '𞸵' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialTeh),
            '𞸶' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialTheh),
            '𞸷' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialKhah),
            '𞸹' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialDad),
            '𞸻' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialGhain),
            '𞹂' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTailedJeem),
            '𞹇' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTailedHah),
            '𞹉' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTailedYeh),
            '𞹋' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTailedLam),
            '𞹍' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTailedNoon),
            '𞹎' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTailedSeen),
            '𞹏' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTailedAin),
            '𞹑' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTailedSad),
            '𞹒' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTailedQaf),
            '𞹔' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTailedSheen),
            '𞹗' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTailedKhah),
            '𞹙' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTailedDad),
            '𞹛' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTailedGhain),
            '𞹝' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTailedDotlessNoon),
            '𞹟' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTailedDotlessQaf),
            '𞹡' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedBeh),
            '𞹢' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedJeem),
            '𞹤' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedHeh),
            '𞹧' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedHah),
            '𞹨' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedTah),
            '𞹩' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedYeh),
            '𞹪' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedKaf),
            '𞹬' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedMeem),
            '𞹭' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedNoon),
            '𞹮' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedSeen),
            '𞹯' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedAin),
            '𞹰' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedFeh),
            '𞹱' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedSad),
            '𞹲' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedQaf),
            '𞹴' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedSheen),
            '𞹵' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedTeh),
            '𞹶' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedTheh),
            '𞹷' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedKhah),
            '𞹹' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedDad),
            '𞹺' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedZah),
            '𞹻' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedGhain),
            '𞹼' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedDotlessBeh),
            '𞹾' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedDotlessFeh),
            '𞺀' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedAlef),
            '𞺁' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedBeh),
            '𞺂' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedJeem),
            '𞺃' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedDal),
            '𞺄' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedHeh),
            '𞺅' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedWaw),
            '𞺆' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedZain),
            '𞺇' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedHah),
            '𞺈' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedTah),
            '𞺉' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedYeh),
            '𞺋' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedLam),
            '𞺌' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedMeem),
            '𞺍' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedNoon),
            '𞺎' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedSeen),
            '𞺏' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedAin),
            '𞺐' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedFeh),
            '𞺑' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedSad),
            '𞺒' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedQaf),
            '𞺓' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedReh),
            '𞺔' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedSheen),
            '𞺕' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedTeh),
            '𞺖' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedTheh),
            '𞺗' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedKhah),
            '𞺘' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedThal),
            '𞺙' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedDad),
            '𞺚' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedZah),
            '𞺛' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedGhain),
            '𞺡' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckBeh),
            '𞺢' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckJeem),
            '𞺣' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckDal),
            '𞺥' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckWaw),
            '𞺦' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckZain),
            '𞺧' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckHah),
            '𞺨' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckTah),
            '𞺩' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckYeh),
            '𞺫' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckLam),
            '𞺬' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckMeem),
            '𞺭' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckNoon),
            '𞺮' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckSeen),
            '𞺯' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckAin),
            '𞺰' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckFeh),
            '𞺱' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckSad),
            '𞺲' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckQaf),
            '𞺳' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckReh),
            '𞺴' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckSheen),
            '𞺵' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckTeh),
            '𞺶' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckTheh),
            '𞺷' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckKhah),
            '𞺸' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckThal),
            '𞺹' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckDad),
            '𞺺' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckZah),
            '𞺻' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckGhain),
            '𞻰' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalOperatorMeemWithHahWithTatweel),
            '𞻱' => Ok(ArabicMathematicalAlphabeticSymbols::ArabicMathematicalOperatorHahWithDal),
            _ => Err(()),
        }
    }
}

impl Into<u32> for ArabicMathematicalAlphabeticSymbols {
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

impl std::convert::TryFrom<u32> for ArabicMathematicalAlphabeticSymbols {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for ArabicMathematicalAlphabeticSymbols {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl ArabicMathematicalAlphabeticSymbols {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        ArabicMathematicalAlphabeticSymbols::ArabicMathematicalAlef
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalAlef => "arabic mathematical alef",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalBeh => "arabic mathematical beh",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalJeem => "arabic mathematical jeem",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDal => "arabic mathematical dal",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalWaw => "arabic mathematical waw",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalZain => "arabic mathematical zain",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalHah => "arabic mathematical hah",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTah => "arabic mathematical tah",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalYeh => "arabic mathematical yeh",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalKaf => "arabic mathematical kaf",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLam => "arabic mathematical lam",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalMeem => "arabic mathematical meem",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalNoon => "arabic mathematical noon",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalSeen => "arabic mathematical seen",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalAin => "arabic mathematical ain",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalFeh => "arabic mathematical feh",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalSad => "arabic mathematical sad",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalQaf => "arabic mathematical qaf",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalReh => "arabic mathematical reh",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalSheen => "arabic mathematical sheen",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTeh => "arabic mathematical teh",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTheh => "arabic mathematical theh",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalKhah => "arabic mathematical khah",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalThal => "arabic mathematical thal",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDad => "arabic mathematical dad",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalZah => "arabic mathematical zah",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalGhain => "arabic mathematical ghain",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDotlessBeh => "arabic mathematical dotless beh",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDotlessNoon => "arabic mathematical dotless noon",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDotlessFeh => "arabic mathematical dotless feh",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDotlessQaf => "arabic mathematical dotless qaf",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialBeh => "arabic mathematical initial beh",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialJeem => "arabic mathematical initial jeem",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialHeh => "arabic mathematical initial heh",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialHah => "arabic mathematical initial hah",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialYeh => "arabic mathematical initial yeh",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialKaf => "arabic mathematical initial kaf",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialLam => "arabic mathematical initial lam",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialMeem => "arabic mathematical initial meem",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialNoon => "arabic mathematical initial noon",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialSeen => "arabic mathematical initial seen",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialAin => "arabic mathematical initial ain",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialFeh => "arabic mathematical initial feh",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialSad => "arabic mathematical initial sad",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialQaf => "arabic mathematical initial qaf",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialSheen => "arabic mathematical initial sheen",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialTeh => "arabic mathematical initial teh",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialTheh => "arabic mathematical initial theh",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialKhah => "arabic mathematical initial khah",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialDad => "arabic mathematical initial dad",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalInitialGhain => "arabic mathematical initial ghain",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTailedJeem => "arabic mathematical tailed jeem",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTailedHah => "arabic mathematical tailed hah",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTailedYeh => "arabic mathematical tailed yeh",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTailedLam => "arabic mathematical tailed lam",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTailedNoon => "arabic mathematical tailed noon",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTailedSeen => "arabic mathematical tailed seen",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTailedAin => "arabic mathematical tailed ain",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTailedSad => "arabic mathematical tailed sad",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTailedQaf => "arabic mathematical tailed qaf",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTailedSheen => "arabic mathematical tailed sheen",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTailedKhah => "arabic mathematical tailed khah",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTailedDad => "arabic mathematical tailed dad",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTailedGhain => "arabic mathematical tailed ghain",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTailedDotlessNoon => "arabic mathematical tailed dotless noon",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalTailedDotlessQaf => "arabic mathematical tailed dotless qaf",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedBeh => "arabic mathematical stretched beh",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedJeem => "arabic mathematical stretched jeem",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedHeh => "arabic mathematical stretched heh",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedHah => "arabic mathematical stretched hah",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedTah => "arabic mathematical stretched tah",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedYeh => "arabic mathematical stretched yeh",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedKaf => "arabic mathematical stretched kaf",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedMeem => "arabic mathematical stretched meem",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedNoon => "arabic mathematical stretched noon",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedSeen => "arabic mathematical stretched seen",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedAin => "arabic mathematical stretched ain",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedFeh => "arabic mathematical stretched feh",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedSad => "arabic mathematical stretched sad",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedQaf => "arabic mathematical stretched qaf",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedSheen => "arabic mathematical stretched sheen",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedTeh => "arabic mathematical stretched teh",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedTheh => "arabic mathematical stretched theh",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedKhah => "arabic mathematical stretched khah",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedDad => "arabic mathematical stretched dad",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedZah => "arabic mathematical stretched zah",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedGhain => "arabic mathematical stretched ghain",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedDotlessBeh => "arabic mathematical stretched dotless beh",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalStretchedDotlessFeh => "arabic mathematical stretched dotless feh",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedAlef => "arabic mathematical looped alef",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedBeh => "arabic mathematical looped beh",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedJeem => "arabic mathematical looped jeem",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedDal => "arabic mathematical looped dal",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedHeh => "arabic mathematical looped heh",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedWaw => "arabic mathematical looped waw",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedZain => "arabic mathematical looped zain",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedHah => "arabic mathematical looped hah",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedTah => "arabic mathematical looped tah",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedYeh => "arabic mathematical looped yeh",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedLam => "arabic mathematical looped lam",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedMeem => "arabic mathematical looped meem",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedNoon => "arabic mathematical looped noon",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedSeen => "arabic mathematical looped seen",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedAin => "arabic mathematical looped ain",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedFeh => "arabic mathematical looped feh",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedSad => "arabic mathematical looped sad",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedQaf => "arabic mathematical looped qaf",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedReh => "arabic mathematical looped reh",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedSheen => "arabic mathematical looped sheen",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedTeh => "arabic mathematical looped teh",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedTheh => "arabic mathematical looped theh",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedKhah => "arabic mathematical looped khah",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedThal => "arabic mathematical looped thal",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedDad => "arabic mathematical looped dad",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedZah => "arabic mathematical looped zah",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalLoopedGhain => "arabic mathematical looped ghain",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckBeh => "arabic mathematical double-struck beh",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckJeem => "arabic mathematical double-struck jeem",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckDal => "arabic mathematical double-struck dal",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckWaw => "arabic mathematical double-struck waw",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckZain => "arabic mathematical double-struck zain",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckHah => "arabic mathematical double-struck hah",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckTah => "arabic mathematical double-struck tah",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckYeh => "arabic mathematical double-struck yeh",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckLam => "arabic mathematical double-struck lam",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckMeem => "arabic mathematical double-struck meem",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckNoon => "arabic mathematical double-struck noon",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckSeen => "arabic mathematical double-struck seen",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckAin => "arabic mathematical double-struck ain",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckFeh => "arabic mathematical double-struck feh",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckSad => "arabic mathematical double-struck sad",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckQaf => "arabic mathematical double-struck qaf",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckReh => "arabic mathematical double-struck reh",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckSheen => "arabic mathematical double-struck sheen",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckTeh => "arabic mathematical double-struck teh",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckTheh => "arabic mathematical double-struck theh",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckKhah => "arabic mathematical double-struck khah",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckThal => "arabic mathematical double-struck thal",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckDad => "arabic mathematical double-struck dad",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckZah => "arabic mathematical double-struck zah",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalDoubleDashStruckGhain => "arabic mathematical double-struck ghain",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalOperatorMeemWithHahWithTatweel => "arabic mathematical operator meem with hah with tatweel",
            ArabicMathematicalAlphabeticSymbols::ArabicMathematicalOperatorHahWithDal => "arabic mathematical operator hah with dal",
        }
    }
}
