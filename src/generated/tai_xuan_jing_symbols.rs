
/// An enum to represent all characters in the TaiXuanJingSymbols block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum TaiXuanJingSymbols {
    /// \u{1d300}: '𝌀'
    MonogramForEarth,
    /// \u{1d301}: '𝌁'
    DigramForHeavenlyEarth,
    /// \u{1d302}: '𝌂'
    DigramForHumanEarth,
    /// \u{1d303}: '𝌃'
    DigramForEarthlyHeaven,
    /// \u{1d304}: '𝌄'
    DigramForEarthlyHuman,
    /// \u{1d305}: '𝌅'
    DigramForEarth,
    /// \u{1d306}: '𝌆'
    TetragramForCentre,
    /// \u{1d307}: '𝌇'
    TetragramForFullCircle,
    /// \u{1d308}: '𝌈'
    TetragramForMired,
    /// \u{1d309}: '𝌉'
    TetragramForBarrier,
    /// \u{1d30a}: '𝌊'
    TetragramForKeepingSmall,
    /// \u{1d30b}: '𝌋'
    TetragramForContrariety,
    /// \u{1d30c}: '𝌌'
    TetragramForAscent,
    /// \u{1d30d}: '𝌍'
    TetragramForOpposition,
    /// \u{1d30e}: '𝌎'
    TetragramForBranchingOut,
    /// \u{1d30f}: '𝌏'
    TetragramForDefectivenessOrDistortion,
    /// \u{1d310}: '𝌐'
    TetragramForDivergence,
    /// \u{1d311}: '𝌑'
    TetragramForYouthfulness,
    /// \u{1d312}: '𝌒'
    TetragramForIncrease,
    /// \u{1d313}: '𝌓'
    TetragramForPenetration,
    /// \u{1d314}: '𝌔'
    TetragramForReach,
    /// \u{1d315}: '𝌕'
    TetragramForContact,
    /// \u{1d316}: '𝌖'
    TetragramForHoldingBack,
    /// \u{1d317}: '𝌗'
    TetragramForWaiting,
    /// \u{1d318}: '𝌘'
    TetragramForFollowing,
    /// \u{1d319}: '𝌙'
    TetragramForAdvance,
    /// \u{1d31a}: '𝌚'
    TetragramForRelease,
    /// \u{1d31b}: '𝌛'
    TetragramForResistance,
    /// \u{1d31c}: '𝌜'
    TetragramForEase,
    /// \u{1d31d}: '𝌝'
    TetragramForJoy,
    /// \u{1d31e}: '𝌞'
    TetragramForContention,
    /// \u{1d31f}: '𝌟'
    TetragramForEndeavour,
    /// \u{1d320}: '𝌠'
    TetragramForDuties,
    /// \u{1d321}: '𝌡'
    TetragramForChange,
    /// \u{1d322}: '𝌢'
    TetragramForDecisiveness,
    /// \u{1d323}: '𝌣'
    TetragramForBoldResolution,
    /// \u{1d324}: '𝌤'
    TetragramForPacking,
    /// \u{1d325}: '𝌥'
    TetragramForLegion,
    /// \u{1d326}: '𝌦'
    TetragramForCloseness,
    /// \u{1d327}: '𝌧'
    TetragramForKinship,
    /// \u{1d328}: '𝌨'
    TetragramForGathering,
    /// \u{1d329}: '𝌩'
    TetragramForStrength,
    /// \u{1d32a}: '𝌪'
    TetragramForPurity,
    /// \u{1d32b}: '𝌫'
    TetragramForFullness,
    /// \u{1d32c}: '𝌬'
    TetragramForResidence,
    /// \u{1d32d}: '𝌭'
    TetragramForLawOrModel,
    /// \u{1d32e}: '𝌮'
    TetragramForResponse,
    /// \u{1d32f}: '𝌯'
    TetragramForGoingToMeet,
    /// \u{1d330}: '𝌰'
    TetragramForEncounters,
    /// \u{1d331}: '𝌱'
    TetragramForStove,
    /// \u{1d332}: '𝌲'
    TetragramForGreatness,
    /// \u{1d333}: '𝌳'
    TetragramForEnlargement,
    /// \u{1d334}: '𝌴'
    TetragramForPattern,
    /// \u{1d335}: '𝌵'
    TetragramForRitual,
    /// \u{1d336}: '𝌶'
    TetragramForFlight,
    /// \u{1d337}: '𝌷'
    TetragramForVastnessOrWasting,
    /// \u{1d338}: '𝌸'
    TetragramForConstancy,
    /// \u{1d339}: '𝌹'
    TetragramForMeasure,
    /// \u{1d33a}: '𝌺'
    TetragramForEternity,
    /// \u{1d33b}: '𝌻'
    TetragramForUnity,
    /// \u{1d33c}: '𝌼'
    TetragramForDiminishment,
    /// \u{1d33d}: '𝌽'
    TetragramForClosedMouth,
    /// \u{1d33e}: '𝌾'
    TetragramForGuardedness,
    /// \u{1d33f}: '𝌿'
    TetragramForGatheringIn,
    /// \u{1d340}: '𝍀'
    TetragramForMassing,
    /// \u{1d341}: '𝍁'
    TetragramForAccumulation,
    /// \u{1d342}: '𝍂'
    TetragramForEmbellishment,
    /// \u{1d343}: '𝍃'
    TetragramForDoubt,
    /// \u{1d344}: '𝍄'
    TetragramForWatch,
    /// \u{1d345}: '𝍅'
    TetragramForSinking,
    /// \u{1d346}: '𝍆'
    TetragramForInner,
    /// \u{1d347}: '𝍇'
    TetragramForDeparture,
    /// \u{1d348}: '𝍈'
    TetragramForDarkening,
    /// \u{1d349}: '𝍉'
    TetragramForDimming,
    /// \u{1d34a}: '𝍊'
    TetragramForExhaustion,
    /// \u{1d34b}: '𝍋'
    TetragramForSeverance,
    /// \u{1d34c}: '𝍌'
    TetragramForStoppage,
    /// \u{1d34d}: '𝍍'
    TetragramForHardness,
    /// \u{1d34e}: '𝍎'
    TetragramForCompletion,
    /// \u{1d34f}: '𝍏'
    TetragramForClosure,
    /// \u{1d350}: '𝍐'
    TetragramForFailure,
    /// \u{1d351}: '𝍑'
    TetragramForAggravation,
    /// \u{1d352}: '𝍒'
    TetragramForCompliance,
    /// \u{1d353}: '𝍓'
    TetragramForOnTheVerge,
    /// \u{1d354}: '𝍔'
    TetragramForDifficulties,
    /// \u{1d355}: '𝍕'
    TetragramForLabouring,
    /// \u{1d356}: '𝍖'
    TetragramForFostering,
}

impl Into<char> for TaiXuanJingSymbols {
    fn into(self) -> char {
        match self {
            TaiXuanJingSymbols::MonogramForEarth => '𝌀',
            TaiXuanJingSymbols::DigramForHeavenlyEarth => '𝌁',
            TaiXuanJingSymbols::DigramForHumanEarth => '𝌂',
            TaiXuanJingSymbols::DigramForEarthlyHeaven => '𝌃',
            TaiXuanJingSymbols::DigramForEarthlyHuman => '𝌄',
            TaiXuanJingSymbols::DigramForEarth => '𝌅',
            TaiXuanJingSymbols::TetragramForCentre => '𝌆',
            TaiXuanJingSymbols::TetragramForFullCircle => '𝌇',
            TaiXuanJingSymbols::TetragramForMired => '𝌈',
            TaiXuanJingSymbols::TetragramForBarrier => '𝌉',
            TaiXuanJingSymbols::TetragramForKeepingSmall => '𝌊',
            TaiXuanJingSymbols::TetragramForContrariety => '𝌋',
            TaiXuanJingSymbols::TetragramForAscent => '𝌌',
            TaiXuanJingSymbols::TetragramForOpposition => '𝌍',
            TaiXuanJingSymbols::TetragramForBranchingOut => '𝌎',
            TaiXuanJingSymbols::TetragramForDefectivenessOrDistortion => '𝌏',
            TaiXuanJingSymbols::TetragramForDivergence => '𝌐',
            TaiXuanJingSymbols::TetragramForYouthfulness => '𝌑',
            TaiXuanJingSymbols::TetragramForIncrease => '𝌒',
            TaiXuanJingSymbols::TetragramForPenetration => '𝌓',
            TaiXuanJingSymbols::TetragramForReach => '𝌔',
            TaiXuanJingSymbols::TetragramForContact => '𝌕',
            TaiXuanJingSymbols::TetragramForHoldingBack => '𝌖',
            TaiXuanJingSymbols::TetragramForWaiting => '𝌗',
            TaiXuanJingSymbols::TetragramForFollowing => '𝌘',
            TaiXuanJingSymbols::TetragramForAdvance => '𝌙',
            TaiXuanJingSymbols::TetragramForRelease => '𝌚',
            TaiXuanJingSymbols::TetragramForResistance => '𝌛',
            TaiXuanJingSymbols::TetragramForEase => '𝌜',
            TaiXuanJingSymbols::TetragramForJoy => '𝌝',
            TaiXuanJingSymbols::TetragramForContention => '𝌞',
            TaiXuanJingSymbols::TetragramForEndeavour => '𝌟',
            TaiXuanJingSymbols::TetragramForDuties => '𝌠',
            TaiXuanJingSymbols::TetragramForChange => '𝌡',
            TaiXuanJingSymbols::TetragramForDecisiveness => '𝌢',
            TaiXuanJingSymbols::TetragramForBoldResolution => '𝌣',
            TaiXuanJingSymbols::TetragramForPacking => '𝌤',
            TaiXuanJingSymbols::TetragramForLegion => '𝌥',
            TaiXuanJingSymbols::TetragramForCloseness => '𝌦',
            TaiXuanJingSymbols::TetragramForKinship => '𝌧',
            TaiXuanJingSymbols::TetragramForGathering => '𝌨',
            TaiXuanJingSymbols::TetragramForStrength => '𝌩',
            TaiXuanJingSymbols::TetragramForPurity => '𝌪',
            TaiXuanJingSymbols::TetragramForFullness => '𝌫',
            TaiXuanJingSymbols::TetragramForResidence => '𝌬',
            TaiXuanJingSymbols::TetragramForLawOrModel => '𝌭',
            TaiXuanJingSymbols::TetragramForResponse => '𝌮',
            TaiXuanJingSymbols::TetragramForGoingToMeet => '𝌯',
            TaiXuanJingSymbols::TetragramForEncounters => '𝌰',
            TaiXuanJingSymbols::TetragramForStove => '𝌱',
            TaiXuanJingSymbols::TetragramForGreatness => '𝌲',
            TaiXuanJingSymbols::TetragramForEnlargement => '𝌳',
            TaiXuanJingSymbols::TetragramForPattern => '𝌴',
            TaiXuanJingSymbols::TetragramForRitual => '𝌵',
            TaiXuanJingSymbols::TetragramForFlight => '𝌶',
            TaiXuanJingSymbols::TetragramForVastnessOrWasting => '𝌷',
            TaiXuanJingSymbols::TetragramForConstancy => '𝌸',
            TaiXuanJingSymbols::TetragramForMeasure => '𝌹',
            TaiXuanJingSymbols::TetragramForEternity => '𝌺',
            TaiXuanJingSymbols::TetragramForUnity => '𝌻',
            TaiXuanJingSymbols::TetragramForDiminishment => '𝌼',
            TaiXuanJingSymbols::TetragramForClosedMouth => '𝌽',
            TaiXuanJingSymbols::TetragramForGuardedness => '𝌾',
            TaiXuanJingSymbols::TetragramForGatheringIn => '𝌿',
            TaiXuanJingSymbols::TetragramForMassing => '𝍀',
            TaiXuanJingSymbols::TetragramForAccumulation => '𝍁',
            TaiXuanJingSymbols::TetragramForEmbellishment => '𝍂',
            TaiXuanJingSymbols::TetragramForDoubt => '𝍃',
            TaiXuanJingSymbols::TetragramForWatch => '𝍄',
            TaiXuanJingSymbols::TetragramForSinking => '𝍅',
            TaiXuanJingSymbols::TetragramForInner => '𝍆',
            TaiXuanJingSymbols::TetragramForDeparture => '𝍇',
            TaiXuanJingSymbols::TetragramForDarkening => '𝍈',
            TaiXuanJingSymbols::TetragramForDimming => '𝍉',
            TaiXuanJingSymbols::TetragramForExhaustion => '𝍊',
            TaiXuanJingSymbols::TetragramForSeverance => '𝍋',
            TaiXuanJingSymbols::TetragramForStoppage => '𝍌',
            TaiXuanJingSymbols::TetragramForHardness => '𝍍',
            TaiXuanJingSymbols::TetragramForCompletion => '𝍎',
            TaiXuanJingSymbols::TetragramForClosure => '𝍏',
            TaiXuanJingSymbols::TetragramForFailure => '𝍐',
            TaiXuanJingSymbols::TetragramForAggravation => '𝍑',
            TaiXuanJingSymbols::TetragramForCompliance => '𝍒',
            TaiXuanJingSymbols::TetragramForOnTheVerge => '𝍓',
            TaiXuanJingSymbols::TetragramForDifficulties => '𝍔',
            TaiXuanJingSymbols::TetragramForLabouring => '𝍕',
            TaiXuanJingSymbols::TetragramForFostering => '𝍖',
        }
    }
}

impl std::convert::TryFrom<char> for TaiXuanJingSymbols {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𝌀' => Ok(TaiXuanJingSymbols::MonogramForEarth),
            '𝌁' => Ok(TaiXuanJingSymbols::DigramForHeavenlyEarth),
            '𝌂' => Ok(TaiXuanJingSymbols::DigramForHumanEarth),
            '𝌃' => Ok(TaiXuanJingSymbols::DigramForEarthlyHeaven),
            '𝌄' => Ok(TaiXuanJingSymbols::DigramForEarthlyHuman),
            '𝌅' => Ok(TaiXuanJingSymbols::DigramForEarth),
            '𝌆' => Ok(TaiXuanJingSymbols::TetragramForCentre),
            '𝌇' => Ok(TaiXuanJingSymbols::TetragramForFullCircle),
            '𝌈' => Ok(TaiXuanJingSymbols::TetragramForMired),
            '𝌉' => Ok(TaiXuanJingSymbols::TetragramForBarrier),
            '𝌊' => Ok(TaiXuanJingSymbols::TetragramForKeepingSmall),
            '𝌋' => Ok(TaiXuanJingSymbols::TetragramForContrariety),
            '𝌌' => Ok(TaiXuanJingSymbols::TetragramForAscent),
            '𝌍' => Ok(TaiXuanJingSymbols::TetragramForOpposition),
            '𝌎' => Ok(TaiXuanJingSymbols::TetragramForBranchingOut),
            '𝌏' => Ok(TaiXuanJingSymbols::TetragramForDefectivenessOrDistortion),
            '𝌐' => Ok(TaiXuanJingSymbols::TetragramForDivergence),
            '𝌑' => Ok(TaiXuanJingSymbols::TetragramForYouthfulness),
            '𝌒' => Ok(TaiXuanJingSymbols::TetragramForIncrease),
            '𝌓' => Ok(TaiXuanJingSymbols::TetragramForPenetration),
            '𝌔' => Ok(TaiXuanJingSymbols::TetragramForReach),
            '𝌕' => Ok(TaiXuanJingSymbols::TetragramForContact),
            '𝌖' => Ok(TaiXuanJingSymbols::TetragramForHoldingBack),
            '𝌗' => Ok(TaiXuanJingSymbols::TetragramForWaiting),
            '𝌘' => Ok(TaiXuanJingSymbols::TetragramForFollowing),
            '𝌙' => Ok(TaiXuanJingSymbols::TetragramForAdvance),
            '𝌚' => Ok(TaiXuanJingSymbols::TetragramForRelease),
            '𝌛' => Ok(TaiXuanJingSymbols::TetragramForResistance),
            '𝌜' => Ok(TaiXuanJingSymbols::TetragramForEase),
            '𝌝' => Ok(TaiXuanJingSymbols::TetragramForJoy),
            '𝌞' => Ok(TaiXuanJingSymbols::TetragramForContention),
            '𝌟' => Ok(TaiXuanJingSymbols::TetragramForEndeavour),
            '𝌠' => Ok(TaiXuanJingSymbols::TetragramForDuties),
            '𝌡' => Ok(TaiXuanJingSymbols::TetragramForChange),
            '𝌢' => Ok(TaiXuanJingSymbols::TetragramForDecisiveness),
            '𝌣' => Ok(TaiXuanJingSymbols::TetragramForBoldResolution),
            '𝌤' => Ok(TaiXuanJingSymbols::TetragramForPacking),
            '𝌥' => Ok(TaiXuanJingSymbols::TetragramForLegion),
            '𝌦' => Ok(TaiXuanJingSymbols::TetragramForCloseness),
            '𝌧' => Ok(TaiXuanJingSymbols::TetragramForKinship),
            '𝌨' => Ok(TaiXuanJingSymbols::TetragramForGathering),
            '𝌩' => Ok(TaiXuanJingSymbols::TetragramForStrength),
            '𝌪' => Ok(TaiXuanJingSymbols::TetragramForPurity),
            '𝌫' => Ok(TaiXuanJingSymbols::TetragramForFullness),
            '𝌬' => Ok(TaiXuanJingSymbols::TetragramForResidence),
            '𝌭' => Ok(TaiXuanJingSymbols::TetragramForLawOrModel),
            '𝌮' => Ok(TaiXuanJingSymbols::TetragramForResponse),
            '𝌯' => Ok(TaiXuanJingSymbols::TetragramForGoingToMeet),
            '𝌰' => Ok(TaiXuanJingSymbols::TetragramForEncounters),
            '𝌱' => Ok(TaiXuanJingSymbols::TetragramForStove),
            '𝌲' => Ok(TaiXuanJingSymbols::TetragramForGreatness),
            '𝌳' => Ok(TaiXuanJingSymbols::TetragramForEnlargement),
            '𝌴' => Ok(TaiXuanJingSymbols::TetragramForPattern),
            '𝌵' => Ok(TaiXuanJingSymbols::TetragramForRitual),
            '𝌶' => Ok(TaiXuanJingSymbols::TetragramForFlight),
            '𝌷' => Ok(TaiXuanJingSymbols::TetragramForVastnessOrWasting),
            '𝌸' => Ok(TaiXuanJingSymbols::TetragramForConstancy),
            '𝌹' => Ok(TaiXuanJingSymbols::TetragramForMeasure),
            '𝌺' => Ok(TaiXuanJingSymbols::TetragramForEternity),
            '𝌻' => Ok(TaiXuanJingSymbols::TetragramForUnity),
            '𝌼' => Ok(TaiXuanJingSymbols::TetragramForDiminishment),
            '𝌽' => Ok(TaiXuanJingSymbols::TetragramForClosedMouth),
            '𝌾' => Ok(TaiXuanJingSymbols::TetragramForGuardedness),
            '𝌿' => Ok(TaiXuanJingSymbols::TetragramForGatheringIn),
            '𝍀' => Ok(TaiXuanJingSymbols::TetragramForMassing),
            '𝍁' => Ok(TaiXuanJingSymbols::TetragramForAccumulation),
            '𝍂' => Ok(TaiXuanJingSymbols::TetragramForEmbellishment),
            '𝍃' => Ok(TaiXuanJingSymbols::TetragramForDoubt),
            '𝍄' => Ok(TaiXuanJingSymbols::TetragramForWatch),
            '𝍅' => Ok(TaiXuanJingSymbols::TetragramForSinking),
            '𝍆' => Ok(TaiXuanJingSymbols::TetragramForInner),
            '𝍇' => Ok(TaiXuanJingSymbols::TetragramForDeparture),
            '𝍈' => Ok(TaiXuanJingSymbols::TetragramForDarkening),
            '𝍉' => Ok(TaiXuanJingSymbols::TetragramForDimming),
            '𝍊' => Ok(TaiXuanJingSymbols::TetragramForExhaustion),
            '𝍋' => Ok(TaiXuanJingSymbols::TetragramForSeverance),
            '𝍌' => Ok(TaiXuanJingSymbols::TetragramForStoppage),
            '𝍍' => Ok(TaiXuanJingSymbols::TetragramForHardness),
            '𝍎' => Ok(TaiXuanJingSymbols::TetragramForCompletion),
            '𝍏' => Ok(TaiXuanJingSymbols::TetragramForClosure),
            '𝍐' => Ok(TaiXuanJingSymbols::TetragramForFailure),
            '𝍑' => Ok(TaiXuanJingSymbols::TetragramForAggravation),
            '𝍒' => Ok(TaiXuanJingSymbols::TetragramForCompliance),
            '𝍓' => Ok(TaiXuanJingSymbols::TetragramForOnTheVerge),
            '𝍔' => Ok(TaiXuanJingSymbols::TetragramForDifficulties),
            '𝍕' => Ok(TaiXuanJingSymbols::TetragramForLabouring),
            '𝍖' => Ok(TaiXuanJingSymbols::TetragramForFostering),
            _ => Err(()),
        }
    }
}

impl Into<u32> for TaiXuanJingSymbols {
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

impl std::convert::TryFrom<u32> for TaiXuanJingSymbols {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for TaiXuanJingSymbols {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl TaiXuanJingSymbols {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        TaiXuanJingSymbols::MonogramForEarth
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            TaiXuanJingSymbols::MonogramForEarth => "monogram for earth",
            TaiXuanJingSymbols::DigramForHeavenlyEarth => "digram for heavenly earth",
            TaiXuanJingSymbols::DigramForHumanEarth => "digram for human earth",
            TaiXuanJingSymbols::DigramForEarthlyHeaven => "digram for earthly heaven",
            TaiXuanJingSymbols::DigramForEarthlyHuman => "digram for earthly human",
            TaiXuanJingSymbols::DigramForEarth => "digram for earth",
            TaiXuanJingSymbols::TetragramForCentre => "tetragram for centre",
            TaiXuanJingSymbols::TetragramForFullCircle => "tetragram for full circle",
            TaiXuanJingSymbols::TetragramForMired => "tetragram for mired",
            TaiXuanJingSymbols::TetragramForBarrier => "tetragram for barrier",
            TaiXuanJingSymbols::TetragramForKeepingSmall => "tetragram for keeping small",
            TaiXuanJingSymbols::TetragramForContrariety => "tetragram for contrariety",
            TaiXuanJingSymbols::TetragramForAscent => "tetragram for ascent",
            TaiXuanJingSymbols::TetragramForOpposition => "tetragram for opposition",
            TaiXuanJingSymbols::TetragramForBranchingOut => "tetragram for branching out",
            TaiXuanJingSymbols::TetragramForDefectivenessOrDistortion => "tetragram for defectiveness or distortion",
            TaiXuanJingSymbols::TetragramForDivergence => "tetragram for divergence",
            TaiXuanJingSymbols::TetragramForYouthfulness => "tetragram for youthfulness",
            TaiXuanJingSymbols::TetragramForIncrease => "tetragram for increase",
            TaiXuanJingSymbols::TetragramForPenetration => "tetragram for penetration",
            TaiXuanJingSymbols::TetragramForReach => "tetragram for reach",
            TaiXuanJingSymbols::TetragramForContact => "tetragram for contact",
            TaiXuanJingSymbols::TetragramForHoldingBack => "tetragram for holding back",
            TaiXuanJingSymbols::TetragramForWaiting => "tetragram for waiting",
            TaiXuanJingSymbols::TetragramForFollowing => "tetragram for following",
            TaiXuanJingSymbols::TetragramForAdvance => "tetragram for advance",
            TaiXuanJingSymbols::TetragramForRelease => "tetragram for release",
            TaiXuanJingSymbols::TetragramForResistance => "tetragram for resistance",
            TaiXuanJingSymbols::TetragramForEase => "tetragram for ease",
            TaiXuanJingSymbols::TetragramForJoy => "tetragram for joy",
            TaiXuanJingSymbols::TetragramForContention => "tetragram for contention",
            TaiXuanJingSymbols::TetragramForEndeavour => "tetragram for endeavour",
            TaiXuanJingSymbols::TetragramForDuties => "tetragram for duties",
            TaiXuanJingSymbols::TetragramForChange => "tetragram for change",
            TaiXuanJingSymbols::TetragramForDecisiveness => "tetragram for decisiveness",
            TaiXuanJingSymbols::TetragramForBoldResolution => "tetragram for bold resolution",
            TaiXuanJingSymbols::TetragramForPacking => "tetragram for packing",
            TaiXuanJingSymbols::TetragramForLegion => "tetragram for legion",
            TaiXuanJingSymbols::TetragramForCloseness => "tetragram for closeness",
            TaiXuanJingSymbols::TetragramForKinship => "tetragram for kinship",
            TaiXuanJingSymbols::TetragramForGathering => "tetragram for gathering",
            TaiXuanJingSymbols::TetragramForStrength => "tetragram for strength",
            TaiXuanJingSymbols::TetragramForPurity => "tetragram for purity",
            TaiXuanJingSymbols::TetragramForFullness => "tetragram for fullness",
            TaiXuanJingSymbols::TetragramForResidence => "tetragram for residence",
            TaiXuanJingSymbols::TetragramForLawOrModel => "tetragram for law or model",
            TaiXuanJingSymbols::TetragramForResponse => "tetragram for response",
            TaiXuanJingSymbols::TetragramForGoingToMeet => "tetragram for going to meet",
            TaiXuanJingSymbols::TetragramForEncounters => "tetragram for encounters",
            TaiXuanJingSymbols::TetragramForStove => "tetragram for stove",
            TaiXuanJingSymbols::TetragramForGreatness => "tetragram for greatness",
            TaiXuanJingSymbols::TetragramForEnlargement => "tetragram for enlargement",
            TaiXuanJingSymbols::TetragramForPattern => "tetragram for pattern",
            TaiXuanJingSymbols::TetragramForRitual => "tetragram for ritual",
            TaiXuanJingSymbols::TetragramForFlight => "tetragram for flight",
            TaiXuanJingSymbols::TetragramForVastnessOrWasting => "tetragram for vastness or wasting",
            TaiXuanJingSymbols::TetragramForConstancy => "tetragram for constancy",
            TaiXuanJingSymbols::TetragramForMeasure => "tetragram for measure",
            TaiXuanJingSymbols::TetragramForEternity => "tetragram for eternity",
            TaiXuanJingSymbols::TetragramForUnity => "tetragram for unity",
            TaiXuanJingSymbols::TetragramForDiminishment => "tetragram for diminishment",
            TaiXuanJingSymbols::TetragramForClosedMouth => "tetragram for closed mouth",
            TaiXuanJingSymbols::TetragramForGuardedness => "tetragram for guardedness",
            TaiXuanJingSymbols::TetragramForGatheringIn => "tetragram for gathering in",
            TaiXuanJingSymbols::TetragramForMassing => "tetragram for massing",
            TaiXuanJingSymbols::TetragramForAccumulation => "tetragram for accumulation",
            TaiXuanJingSymbols::TetragramForEmbellishment => "tetragram for embellishment",
            TaiXuanJingSymbols::TetragramForDoubt => "tetragram for doubt",
            TaiXuanJingSymbols::TetragramForWatch => "tetragram for watch",
            TaiXuanJingSymbols::TetragramForSinking => "tetragram for sinking",
            TaiXuanJingSymbols::TetragramForInner => "tetragram for inner",
            TaiXuanJingSymbols::TetragramForDeparture => "tetragram for departure",
            TaiXuanJingSymbols::TetragramForDarkening => "tetragram for darkening",
            TaiXuanJingSymbols::TetragramForDimming => "tetragram for dimming",
            TaiXuanJingSymbols::TetragramForExhaustion => "tetragram for exhaustion",
            TaiXuanJingSymbols::TetragramForSeverance => "tetragram for severance",
            TaiXuanJingSymbols::TetragramForStoppage => "tetragram for stoppage",
            TaiXuanJingSymbols::TetragramForHardness => "tetragram for hardness",
            TaiXuanJingSymbols::TetragramForCompletion => "tetragram for completion",
            TaiXuanJingSymbols::TetragramForClosure => "tetragram for closure",
            TaiXuanJingSymbols::TetragramForFailure => "tetragram for failure",
            TaiXuanJingSymbols::TetragramForAggravation => "tetragram for aggravation",
            TaiXuanJingSymbols::TetragramForCompliance => "tetragram for compliance",
            TaiXuanJingSymbols::TetragramForOnTheVerge => "tetragram for on the verge",
            TaiXuanJingSymbols::TetragramForDifficulties => "tetragram for difficulties",
            TaiXuanJingSymbols::TetragramForLabouring => "tetragram for labouring",
            TaiXuanJingSymbols::TetragramForFostering => "tetragram for fostering",
        }
    }
}
