
/// An enum to represent all characters in the YijingHexagramSymbols block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum YijingHexagramSymbols {
    /// \u{4dc0}: '䷀'
    HexagramForTheCreativeHeaven,
    /// \u{4dc1}: '䷁'
    HexagramForTheReceptiveEarth,
    /// \u{4dc2}: '䷂'
    HexagramForDifficultyAtTheBeginning,
    /// \u{4dc3}: '䷃'
    HexagramForYouthfulFolly,
    /// \u{4dc4}: '䷄'
    HexagramForWaiting,
    /// \u{4dc5}: '䷅'
    HexagramForConflict,
    /// \u{4dc6}: '䷆'
    HexagramForTheArmy,
    /// \u{4dc7}: '䷇'
    HexagramForHoldingTogether,
    /// \u{4dc8}: '䷈'
    HexagramForSmallTaming,
    /// \u{4dc9}: '䷉'
    HexagramForTreading,
    /// \u{4dca}: '䷊'
    HexagramForPeace,
    /// \u{4dcb}: '䷋'
    HexagramForStandstill,
    /// \u{4dcc}: '䷌'
    HexagramForFellowship,
    /// \u{4dcd}: '䷍'
    HexagramForGreatPossession,
    /// \u{4dce}: '䷎'
    HexagramForModesty,
    /// \u{4dcf}: '䷏'
    HexagramForEnthusiasm,
    /// \u{4dd0}: '䷐'
    HexagramForFollowing,
    /// \u{4dd1}: '䷑'
    HexagramForWorkOnTheDecayed,
    /// \u{4dd2}: '䷒'
    HexagramForApproach,
    /// \u{4dd3}: '䷓'
    HexagramForContemplation,
    /// \u{4dd4}: '䷔'
    HexagramForBitingThrough,
    /// \u{4dd5}: '䷕'
    HexagramForGrace,
    /// \u{4dd6}: '䷖'
    HexagramForSplittingApart,
    /// \u{4dd7}: '䷗'
    HexagramForReturn,
    /// \u{4dd8}: '䷘'
    HexagramForInnocence,
    /// \u{4dd9}: '䷙'
    HexagramForGreatTaming,
    /// \u{4dda}: '䷚'
    HexagramForMouthCorners,
    /// \u{4ddb}: '䷛'
    HexagramForGreatPreponderance,
    /// \u{4ddc}: '䷜'
    HexagramForTheAbysmalWater,
    /// \u{4ddd}: '䷝'
    HexagramForTheClingingFire,
    /// \u{4dde}: '䷞'
    HexagramForInfluence,
    /// \u{4ddf}: '䷟'
    HexagramForDuration,
    /// \u{4de0}: '䷠'
    HexagramForRetreat,
    /// \u{4de1}: '䷡'
    HexagramForGreatPower,
    /// \u{4de2}: '䷢'
    HexagramForProgress,
    /// \u{4de3}: '䷣'
    HexagramForDarkeningOfTheLight,
    /// \u{4de4}: '䷤'
    HexagramForTheFamily,
    /// \u{4de5}: '䷥'
    HexagramForOpposition,
    /// \u{4de6}: '䷦'
    HexagramForObstruction,
    /// \u{4de7}: '䷧'
    HexagramForDeliverance,
    /// \u{4de8}: '䷨'
    HexagramForDecrease,
    /// \u{4de9}: '䷩'
    HexagramForIncrease,
    /// \u{4dea}: '䷪'
    HexagramForBreakthrough,
    /// \u{4deb}: '䷫'
    HexagramForComingToMeet,
    /// \u{4dec}: '䷬'
    HexagramForGatheringTogether,
    /// \u{4ded}: '䷭'
    HexagramForPushingUpward,
    /// \u{4dee}: '䷮'
    HexagramForOppression,
    /// \u{4def}: '䷯'
    HexagramForTheWell,
    /// \u{4df0}: '䷰'
    HexagramForRevolution,
    /// \u{4df1}: '䷱'
    HexagramForTheCauldron,
    /// \u{4df2}: '䷲'
    HexagramForTheArousingThunder,
    /// \u{4df3}: '䷳'
    HexagramForTheKeepingStillMountain,
    /// \u{4df4}: '䷴'
    HexagramForDevelopment,
    /// \u{4df5}: '䷵'
    HexagramForTheMarryingMaiden,
    /// \u{4df6}: '䷶'
    HexagramForAbundance,
    /// \u{4df7}: '䷷'
    HexagramForTheWanderer,
    /// \u{4df8}: '䷸'
    HexagramForTheGentleWind,
    /// \u{4df9}: '䷹'
    HexagramForTheJoyousLake,
    /// \u{4dfa}: '䷺'
    HexagramForDispersion,
    /// \u{4dfb}: '䷻'
    HexagramForLimitation,
    /// \u{4dfc}: '䷼'
    HexagramForInnerTruth,
    /// \u{4dfd}: '䷽'
    HexagramForSmallPreponderance,
    /// \u{4dfe}: '䷾'
    HexagramForAfterCompletion,
}

impl Into<char> for YijingHexagramSymbols {
    fn into(self) -> char {
        match self {
            YijingHexagramSymbols::HexagramForTheCreativeHeaven => '䷀',
            YijingHexagramSymbols::HexagramForTheReceptiveEarth => '䷁',
            YijingHexagramSymbols::HexagramForDifficultyAtTheBeginning => '䷂',
            YijingHexagramSymbols::HexagramForYouthfulFolly => '䷃',
            YijingHexagramSymbols::HexagramForWaiting => '䷄',
            YijingHexagramSymbols::HexagramForConflict => '䷅',
            YijingHexagramSymbols::HexagramForTheArmy => '䷆',
            YijingHexagramSymbols::HexagramForHoldingTogether => '䷇',
            YijingHexagramSymbols::HexagramForSmallTaming => '䷈',
            YijingHexagramSymbols::HexagramForTreading => '䷉',
            YijingHexagramSymbols::HexagramForPeace => '䷊',
            YijingHexagramSymbols::HexagramForStandstill => '䷋',
            YijingHexagramSymbols::HexagramForFellowship => '䷌',
            YijingHexagramSymbols::HexagramForGreatPossession => '䷍',
            YijingHexagramSymbols::HexagramForModesty => '䷎',
            YijingHexagramSymbols::HexagramForEnthusiasm => '䷏',
            YijingHexagramSymbols::HexagramForFollowing => '䷐',
            YijingHexagramSymbols::HexagramForWorkOnTheDecayed => '䷑',
            YijingHexagramSymbols::HexagramForApproach => '䷒',
            YijingHexagramSymbols::HexagramForContemplation => '䷓',
            YijingHexagramSymbols::HexagramForBitingThrough => '䷔',
            YijingHexagramSymbols::HexagramForGrace => '䷕',
            YijingHexagramSymbols::HexagramForSplittingApart => '䷖',
            YijingHexagramSymbols::HexagramForReturn => '䷗',
            YijingHexagramSymbols::HexagramForInnocence => '䷘',
            YijingHexagramSymbols::HexagramForGreatTaming => '䷙',
            YijingHexagramSymbols::HexagramForMouthCorners => '䷚',
            YijingHexagramSymbols::HexagramForGreatPreponderance => '䷛',
            YijingHexagramSymbols::HexagramForTheAbysmalWater => '䷜',
            YijingHexagramSymbols::HexagramForTheClingingFire => '䷝',
            YijingHexagramSymbols::HexagramForInfluence => '䷞',
            YijingHexagramSymbols::HexagramForDuration => '䷟',
            YijingHexagramSymbols::HexagramForRetreat => '䷠',
            YijingHexagramSymbols::HexagramForGreatPower => '䷡',
            YijingHexagramSymbols::HexagramForProgress => '䷢',
            YijingHexagramSymbols::HexagramForDarkeningOfTheLight => '䷣',
            YijingHexagramSymbols::HexagramForTheFamily => '䷤',
            YijingHexagramSymbols::HexagramForOpposition => '䷥',
            YijingHexagramSymbols::HexagramForObstruction => '䷦',
            YijingHexagramSymbols::HexagramForDeliverance => '䷧',
            YijingHexagramSymbols::HexagramForDecrease => '䷨',
            YijingHexagramSymbols::HexagramForIncrease => '䷩',
            YijingHexagramSymbols::HexagramForBreakthrough => '䷪',
            YijingHexagramSymbols::HexagramForComingToMeet => '䷫',
            YijingHexagramSymbols::HexagramForGatheringTogether => '䷬',
            YijingHexagramSymbols::HexagramForPushingUpward => '䷭',
            YijingHexagramSymbols::HexagramForOppression => '䷮',
            YijingHexagramSymbols::HexagramForTheWell => '䷯',
            YijingHexagramSymbols::HexagramForRevolution => '䷰',
            YijingHexagramSymbols::HexagramForTheCauldron => '䷱',
            YijingHexagramSymbols::HexagramForTheArousingThunder => '䷲',
            YijingHexagramSymbols::HexagramForTheKeepingStillMountain => '䷳',
            YijingHexagramSymbols::HexagramForDevelopment => '䷴',
            YijingHexagramSymbols::HexagramForTheMarryingMaiden => '䷵',
            YijingHexagramSymbols::HexagramForAbundance => '䷶',
            YijingHexagramSymbols::HexagramForTheWanderer => '䷷',
            YijingHexagramSymbols::HexagramForTheGentleWind => '䷸',
            YijingHexagramSymbols::HexagramForTheJoyousLake => '䷹',
            YijingHexagramSymbols::HexagramForDispersion => '䷺',
            YijingHexagramSymbols::HexagramForLimitation => '䷻',
            YijingHexagramSymbols::HexagramForInnerTruth => '䷼',
            YijingHexagramSymbols::HexagramForSmallPreponderance => '䷽',
            YijingHexagramSymbols::HexagramForAfterCompletion => '䷾',
        }
    }
}

impl std::convert::TryFrom<char> for YijingHexagramSymbols {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '䷀' => Ok(YijingHexagramSymbols::HexagramForTheCreativeHeaven),
            '䷁' => Ok(YijingHexagramSymbols::HexagramForTheReceptiveEarth),
            '䷂' => Ok(YijingHexagramSymbols::HexagramForDifficultyAtTheBeginning),
            '䷃' => Ok(YijingHexagramSymbols::HexagramForYouthfulFolly),
            '䷄' => Ok(YijingHexagramSymbols::HexagramForWaiting),
            '䷅' => Ok(YijingHexagramSymbols::HexagramForConflict),
            '䷆' => Ok(YijingHexagramSymbols::HexagramForTheArmy),
            '䷇' => Ok(YijingHexagramSymbols::HexagramForHoldingTogether),
            '䷈' => Ok(YijingHexagramSymbols::HexagramForSmallTaming),
            '䷉' => Ok(YijingHexagramSymbols::HexagramForTreading),
            '䷊' => Ok(YijingHexagramSymbols::HexagramForPeace),
            '䷋' => Ok(YijingHexagramSymbols::HexagramForStandstill),
            '䷌' => Ok(YijingHexagramSymbols::HexagramForFellowship),
            '䷍' => Ok(YijingHexagramSymbols::HexagramForGreatPossession),
            '䷎' => Ok(YijingHexagramSymbols::HexagramForModesty),
            '䷏' => Ok(YijingHexagramSymbols::HexagramForEnthusiasm),
            '䷐' => Ok(YijingHexagramSymbols::HexagramForFollowing),
            '䷑' => Ok(YijingHexagramSymbols::HexagramForWorkOnTheDecayed),
            '䷒' => Ok(YijingHexagramSymbols::HexagramForApproach),
            '䷓' => Ok(YijingHexagramSymbols::HexagramForContemplation),
            '䷔' => Ok(YijingHexagramSymbols::HexagramForBitingThrough),
            '䷕' => Ok(YijingHexagramSymbols::HexagramForGrace),
            '䷖' => Ok(YijingHexagramSymbols::HexagramForSplittingApart),
            '䷗' => Ok(YijingHexagramSymbols::HexagramForReturn),
            '䷘' => Ok(YijingHexagramSymbols::HexagramForInnocence),
            '䷙' => Ok(YijingHexagramSymbols::HexagramForGreatTaming),
            '䷚' => Ok(YijingHexagramSymbols::HexagramForMouthCorners),
            '䷛' => Ok(YijingHexagramSymbols::HexagramForGreatPreponderance),
            '䷜' => Ok(YijingHexagramSymbols::HexagramForTheAbysmalWater),
            '䷝' => Ok(YijingHexagramSymbols::HexagramForTheClingingFire),
            '䷞' => Ok(YijingHexagramSymbols::HexagramForInfluence),
            '䷟' => Ok(YijingHexagramSymbols::HexagramForDuration),
            '䷠' => Ok(YijingHexagramSymbols::HexagramForRetreat),
            '䷡' => Ok(YijingHexagramSymbols::HexagramForGreatPower),
            '䷢' => Ok(YijingHexagramSymbols::HexagramForProgress),
            '䷣' => Ok(YijingHexagramSymbols::HexagramForDarkeningOfTheLight),
            '䷤' => Ok(YijingHexagramSymbols::HexagramForTheFamily),
            '䷥' => Ok(YijingHexagramSymbols::HexagramForOpposition),
            '䷦' => Ok(YijingHexagramSymbols::HexagramForObstruction),
            '䷧' => Ok(YijingHexagramSymbols::HexagramForDeliverance),
            '䷨' => Ok(YijingHexagramSymbols::HexagramForDecrease),
            '䷩' => Ok(YijingHexagramSymbols::HexagramForIncrease),
            '䷪' => Ok(YijingHexagramSymbols::HexagramForBreakthrough),
            '䷫' => Ok(YijingHexagramSymbols::HexagramForComingToMeet),
            '䷬' => Ok(YijingHexagramSymbols::HexagramForGatheringTogether),
            '䷭' => Ok(YijingHexagramSymbols::HexagramForPushingUpward),
            '䷮' => Ok(YijingHexagramSymbols::HexagramForOppression),
            '䷯' => Ok(YijingHexagramSymbols::HexagramForTheWell),
            '䷰' => Ok(YijingHexagramSymbols::HexagramForRevolution),
            '䷱' => Ok(YijingHexagramSymbols::HexagramForTheCauldron),
            '䷲' => Ok(YijingHexagramSymbols::HexagramForTheArousingThunder),
            '䷳' => Ok(YijingHexagramSymbols::HexagramForTheKeepingStillMountain),
            '䷴' => Ok(YijingHexagramSymbols::HexagramForDevelopment),
            '䷵' => Ok(YijingHexagramSymbols::HexagramForTheMarryingMaiden),
            '䷶' => Ok(YijingHexagramSymbols::HexagramForAbundance),
            '䷷' => Ok(YijingHexagramSymbols::HexagramForTheWanderer),
            '䷸' => Ok(YijingHexagramSymbols::HexagramForTheGentleWind),
            '䷹' => Ok(YijingHexagramSymbols::HexagramForTheJoyousLake),
            '䷺' => Ok(YijingHexagramSymbols::HexagramForDispersion),
            '䷻' => Ok(YijingHexagramSymbols::HexagramForLimitation),
            '䷼' => Ok(YijingHexagramSymbols::HexagramForInnerTruth),
            '䷽' => Ok(YijingHexagramSymbols::HexagramForSmallPreponderance),
            '䷾' => Ok(YijingHexagramSymbols::HexagramForAfterCompletion),
            _ => Err(()),
        }
    }
}

impl Into<u32> for YijingHexagramSymbols {
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

impl std::convert::TryFrom<u32> for YijingHexagramSymbols {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for YijingHexagramSymbols {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl YijingHexagramSymbols {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        YijingHexagramSymbols::HexagramForTheCreativeHeaven
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            YijingHexagramSymbols::HexagramForTheCreativeHeaven => "hexagram for the creative heaven",
            YijingHexagramSymbols::HexagramForTheReceptiveEarth => "hexagram for the receptive earth",
            YijingHexagramSymbols::HexagramForDifficultyAtTheBeginning => "hexagram for difficulty at the beginning",
            YijingHexagramSymbols::HexagramForYouthfulFolly => "hexagram for youthful folly",
            YijingHexagramSymbols::HexagramForWaiting => "hexagram for waiting",
            YijingHexagramSymbols::HexagramForConflict => "hexagram for conflict",
            YijingHexagramSymbols::HexagramForTheArmy => "hexagram for the army",
            YijingHexagramSymbols::HexagramForHoldingTogether => "hexagram for holding together",
            YijingHexagramSymbols::HexagramForSmallTaming => "hexagram for small taming",
            YijingHexagramSymbols::HexagramForTreading => "hexagram for treading",
            YijingHexagramSymbols::HexagramForPeace => "hexagram for peace",
            YijingHexagramSymbols::HexagramForStandstill => "hexagram for standstill",
            YijingHexagramSymbols::HexagramForFellowship => "hexagram for fellowship",
            YijingHexagramSymbols::HexagramForGreatPossession => "hexagram for great possession",
            YijingHexagramSymbols::HexagramForModesty => "hexagram for modesty",
            YijingHexagramSymbols::HexagramForEnthusiasm => "hexagram for enthusiasm",
            YijingHexagramSymbols::HexagramForFollowing => "hexagram for following",
            YijingHexagramSymbols::HexagramForWorkOnTheDecayed => "hexagram for work on the decayed",
            YijingHexagramSymbols::HexagramForApproach => "hexagram for approach",
            YijingHexagramSymbols::HexagramForContemplation => "hexagram for contemplation",
            YijingHexagramSymbols::HexagramForBitingThrough => "hexagram for biting through",
            YijingHexagramSymbols::HexagramForGrace => "hexagram for grace",
            YijingHexagramSymbols::HexagramForSplittingApart => "hexagram for splitting apart",
            YijingHexagramSymbols::HexagramForReturn => "hexagram for return",
            YijingHexagramSymbols::HexagramForInnocence => "hexagram for innocence",
            YijingHexagramSymbols::HexagramForGreatTaming => "hexagram for great taming",
            YijingHexagramSymbols::HexagramForMouthCorners => "hexagram for mouth corners",
            YijingHexagramSymbols::HexagramForGreatPreponderance => "hexagram for great preponderance",
            YijingHexagramSymbols::HexagramForTheAbysmalWater => "hexagram for the abysmal water",
            YijingHexagramSymbols::HexagramForTheClingingFire => "hexagram for the clinging fire",
            YijingHexagramSymbols::HexagramForInfluence => "hexagram for influence",
            YijingHexagramSymbols::HexagramForDuration => "hexagram for duration",
            YijingHexagramSymbols::HexagramForRetreat => "hexagram for retreat",
            YijingHexagramSymbols::HexagramForGreatPower => "hexagram for great power",
            YijingHexagramSymbols::HexagramForProgress => "hexagram for progress",
            YijingHexagramSymbols::HexagramForDarkeningOfTheLight => "hexagram for darkening of the light",
            YijingHexagramSymbols::HexagramForTheFamily => "hexagram for the family",
            YijingHexagramSymbols::HexagramForOpposition => "hexagram for opposition",
            YijingHexagramSymbols::HexagramForObstruction => "hexagram for obstruction",
            YijingHexagramSymbols::HexagramForDeliverance => "hexagram for deliverance",
            YijingHexagramSymbols::HexagramForDecrease => "hexagram for decrease",
            YijingHexagramSymbols::HexagramForIncrease => "hexagram for increase",
            YijingHexagramSymbols::HexagramForBreakthrough => "hexagram for breakthrough",
            YijingHexagramSymbols::HexagramForComingToMeet => "hexagram for coming to meet",
            YijingHexagramSymbols::HexagramForGatheringTogether => "hexagram for gathering together",
            YijingHexagramSymbols::HexagramForPushingUpward => "hexagram for pushing upward",
            YijingHexagramSymbols::HexagramForOppression => "hexagram for oppression",
            YijingHexagramSymbols::HexagramForTheWell => "hexagram for the well",
            YijingHexagramSymbols::HexagramForRevolution => "hexagram for revolution",
            YijingHexagramSymbols::HexagramForTheCauldron => "hexagram for the cauldron",
            YijingHexagramSymbols::HexagramForTheArousingThunder => "hexagram for the arousing thunder",
            YijingHexagramSymbols::HexagramForTheKeepingStillMountain => "hexagram for the keeping still mountain",
            YijingHexagramSymbols::HexagramForDevelopment => "hexagram for development",
            YijingHexagramSymbols::HexagramForTheMarryingMaiden => "hexagram for the marrying maiden",
            YijingHexagramSymbols::HexagramForAbundance => "hexagram for abundance",
            YijingHexagramSymbols::HexagramForTheWanderer => "hexagram for the wanderer",
            YijingHexagramSymbols::HexagramForTheGentleWind => "hexagram for the gentle wind",
            YijingHexagramSymbols::HexagramForTheJoyousLake => "hexagram for the joyous lake",
            YijingHexagramSymbols::HexagramForDispersion => "hexagram for dispersion",
            YijingHexagramSymbols::HexagramForLimitation => "hexagram for limitation",
            YijingHexagramSymbols::HexagramForInnerTruth => "hexagram for inner truth",
            YijingHexagramSymbols::HexagramForSmallPreponderance => "hexagram for small preponderance",
            YijingHexagramSymbols::HexagramForAfterCompletion => "hexagram for after completion",
        }
    }
}
