
/// An enum to represent all characters in the AlchemicalSymbols block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum AlchemicalSymbols {
    /// \u{1f700}: '🜀'
    AlchemicalSymbolForQuintessence,
    /// \u{1f701}: '🜁'
    AlchemicalSymbolForAir,
    /// \u{1f702}: '🜂'
    AlchemicalSymbolForFire,
    /// \u{1f703}: '🜃'
    AlchemicalSymbolForEarth,
    /// \u{1f704}: '🜄'
    AlchemicalSymbolForWater,
    /// \u{1f705}: '🜅'
    AlchemicalSymbolForAquafortis,
    /// \u{1f706}: '🜆'
    AlchemicalSymbolForAquaRegia,
    /// \u{1f707}: '🜇'
    AlchemicalSymbolForAquaRegiaDash2,
    /// \u{1f708}: '🜈'
    AlchemicalSymbolForAquaVitae,
    /// \u{1f709}: '🜉'
    AlchemicalSymbolForAquaVitaeDash2,
    /// \u{1f70a}: '🜊'
    AlchemicalSymbolForVinegar,
    /// \u{1f70b}: '🜋'
    AlchemicalSymbolForVinegarDash2,
    /// \u{1f70c}: '🜌'
    AlchemicalSymbolForVinegarDash3,
    /// \u{1f70d}: '🜍'
    AlchemicalSymbolForSulfur,
    /// \u{1f70e}: '🜎'
    AlchemicalSymbolForPhilosophersSulfur,
    /// \u{1f70f}: '🜏'
    AlchemicalSymbolForBlackSulfur,
    /// \u{1f710}: '🜐'
    AlchemicalSymbolForMercurySublimate,
    /// \u{1f711}: '🜑'
    AlchemicalSymbolForMercurySublimateDash2,
    /// \u{1f712}: '🜒'
    AlchemicalSymbolForMercurySublimateDash3,
    /// \u{1f713}: '🜓'
    AlchemicalSymbolForCinnabar,
    /// \u{1f714}: '🜔'
    AlchemicalSymbolForSalt,
    /// \u{1f715}: '🜕'
    AlchemicalSymbolForNitre,
    /// \u{1f716}: '🜖'
    AlchemicalSymbolForVitriol,
    /// \u{1f717}: '🜗'
    AlchemicalSymbolForVitriolDash2,
    /// \u{1f718}: '🜘'
    AlchemicalSymbolForRockSalt,
    /// \u{1f719}: '🜙'
    AlchemicalSymbolForRockSaltDash2,
    /// \u{1f71a}: '🜚'
    AlchemicalSymbolForGold,
    /// \u{1f71b}: '🜛'
    AlchemicalSymbolForSilver,
    /// \u{1f71c}: '🜜'
    AlchemicalSymbolForIronOre,
    /// \u{1f71d}: '🜝'
    AlchemicalSymbolForIronOreDash2,
    /// \u{1f71e}: '🜞'
    AlchemicalSymbolForCrocusOfIron,
    /// \u{1f71f}: '🜟'
    AlchemicalSymbolForRegulusOfIron,
    /// \u{1f720}: '🜠'
    AlchemicalSymbolForCopperOre,
    /// \u{1f721}: '🜡'
    AlchemicalSymbolForIronDashCopperOre,
    /// \u{1f722}: '🜢'
    AlchemicalSymbolForSublimateOfCopper,
    /// \u{1f723}: '🜣'
    AlchemicalSymbolForCrocusOfCopper,
    /// \u{1f724}: '🜤'
    AlchemicalSymbolForCrocusOfCopperDash2,
    /// \u{1f725}: '🜥'
    AlchemicalSymbolForCopperAntimoniate,
    /// \u{1f726}: '🜦'
    AlchemicalSymbolForSaltOfCopperAntimoniate,
    /// \u{1f727}: '🜧'
    AlchemicalSymbolForSublimateOfSaltOfCopper,
    /// \u{1f728}: '🜨'
    AlchemicalSymbolForVerdigris,
    /// \u{1f729}: '🜩'
    AlchemicalSymbolForTinOre,
    /// \u{1f72a}: '🜪'
    AlchemicalSymbolForLeadOre,
    /// \u{1f72b}: '🜫'
    AlchemicalSymbolForAntimonyOre,
    /// \u{1f72c}: '🜬'
    AlchemicalSymbolForSublimateOfAntimony,
    /// \u{1f72d}: '🜭'
    AlchemicalSymbolForSaltOfAntimony,
    /// \u{1f72e}: '🜮'
    AlchemicalSymbolForSublimateOfSaltOfAntimony,
    /// \u{1f72f}: '🜯'
    AlchemicalSymbolForVinegarOfAntimony,
    /// \u{1f730}: '🜰'
    AlchemicalSymbolForRegulusOfAntimony,
    /// \u{1f731}: '🜱'
    AlchemicalSymbolForRegulusOfAntimonyDash2,
    /// \u{1f732}: '🜲'
    AlchemicalSymbolForRegulus,
    /// \u{1f733}: '🜳'
    AlchemicalSymbolForRegulusDash2,
    /// \u{1f734}: '🜴'
    AlchemicalSymbolForRegulusDash3,
    /// \u{1f735}: '🜵'
    AlchemicalSymbolForRegulusDash4,
    /// \u{1f736}: '🜶'
    AlchemicalSymbolForAlkali,
    /// \u{1f737}: '🜷'
    AlchemicalSymbolForAlkaliDash2,
    /// \u{1f738}: '🜸'
    AlchemicalSymbolForMarcasite,
    /// \u{1f739}: '🜹'
    AlchemicalSymbolForSalDashAmmoniac,
    /// \u{1f73a}: '🜺'
    AlchemicalSymbolForArsenic,
    /// \u{1f73b}: '🜻'
    AlchemicalSymbolForRealgar,
    /// \u{1f73c}: '🜼'
    AlchemicalSymbolForRealgarDash2,
    /// \u{1f73d}: '🜽'
    AlchemicalSymbolForAuripigment,
    /// \u{1f73e}: '🜾'
    AlchemicalSymbolForBismuthOre,
    /// \u{1f73f}: '🜿'
    AlchemicalSymbolForTartar,
    /// \u{1f740}: '🝀'
    AlchemicalSymbolForTartarDash2,
    /// \u{1f741}: '🝁'
    AlchemicalSymbolForQuickLime,
    /// \u{1f742}: '🝂'
    AlchemicalSymbolForBorax,
    /// \u{1f743}: '🝃'
    AlchemicalSymbolForBoraxDash2,
    /// \u{1f744}: '🝄'
    AlchemicalSymbolForBoraxDash3,
    /// \u{1f745}: '🝅'
    AlchemicalSymbolForAlum,
    /// \u{1f746}: '🝆'
    AlchemicalSymbolForOil,
    /// \u{1f747}: '🝇'
    AlchemicalSymbolForSpirit,
    /// \u{1f748}: '🝈'
    AlchemicalSymbolForTincture,
    /// \u{1f749}: '🝉'
    AlchemicalSymbolForGum,
    /// \u{1f74a}: '🝊'
    AlchemicalSymbolForWax,
    /// \u{1f74b}: '🝋'
    AlchemicalSymbolForPowder,
    /// \u{1f74c}: '🝌'
    AlchemicalSymbolForCalx,
    /// \u{1f74d}: '🝍'
    AlchemicalSymbolForTutty,
    /// \u{1f74e}: '🝎'
    AlchemicalSymbolForCaputMortuum,
    /// \u{1f74f}: '🝏'
    AlchemicalSymbolForScepterOfJove,
    /// \u{1f750}: '🝐'
    AlchemicalSymbolForCaduceus,
    /// \u{1f751}: '🝑'
    AlchemicalSymbolForTrident,
    /// \u{1f752}: '🝒'
    AlchemicalSymbolForStarredTrident,
    /// \u{1f753}: '🝓'
    AlchemicalSymbolForLodestone,
    /// \u{1f754}: '🝔'
    AlchemicalSymbolForSoap,
    /// \u{1f755}: '🝕'
    AlchemicalSymbolForUrine,
    /// \u{1f756}: '🝖'
    AlchemicalSymbolForHorseDung,
    /// \u{1f757}: '🝗'
    AlchemicalSymbolForAshes,
    /// \u{1f758}: '🝘'
    AlchemicalSymbolForPotAshes,
    /// \u{1f759}: '🝙'
    AlchemicalSymbolForBrick,
    /// \u{1f75a}: '🝚'
    AlchemicalSymbolForPowderedBrick,
    /// \u{1f75b}: '🝛'
    AlchemicalSymbolForAmalgam,
    /// \u{1f75c}: '🝜'
    AlchemicalSymbolForStratumSuperStratum,
    /// \u{1f75d}: '🝝'
    AlchemicalSymbolForStratumSuperStratumDash2,
    /// \u{1f75e}: '🝞'
    AlchemicalSymbolForSublimation,
    /// \u{1f75f}: '🝟'
    AlchemicalSymbolForPrecipitate,
    /// \u{1f760}: '🝠'
    AlchemicalSymbolForDistill,
    /// \u{1f761}: '🝡'
    AlchemicalSymbolForDissolve,
    /// \u{1f762}: '🝢'
    AlchemicalSymbolForDissolveDash2,
    /// \u{1f763}: '🝣'
    AlchemicalSymbolForPurify,
    /// \u{1f764}: '🝤'
    AlchemicalSymbolForPutrefaction,
    /// \u{1f765}: '🝥'
    AlchemicalSymbolForCrucible,
    /// \u{1f766}: '🝦'
    AlchemicalSymbolForCrucibleDash2,
    /// \u{1f767}: '🝧'
    AlchemicalSymbolForCrucibleDash3,
    /// \u{1f768}: '🝨'
    AlchemicalSymbolForCrucibleDash4,
    /// \u{1f769}: '🝩'
    AlchemicalSymbolForCrucibleDash5,
    /// \u{1f76a}: '🝪'
    AlchemicalSymbolForAlembic,
    /// \u{1f76b}: '🝫'
    AlchemicalSymbolForBathOfMary,
    /// \u{1f76c}: '🝬'
    AlchemicalSymbolForBathOfVapours,
    /// \u{1f76d}: '🝭'
    AlchemicalSymbolForRetort,
    /// \u{1f76e}: '🝮'
    AlchemicalSymbolForHour,
    /// \u{1f76f}: '🝯'
    AlchemicalSymbolForNight,
    /// \u{1f770}: '🝰'
    AlchemicalSymbolForDayDashNight,
    /// \u{1f771}: '🝱'
    AlchemicalSymbolForMonth,
    /// \u{1f772}: '🝲'
    AlchemicalSymbolForHalfDram,
    /// \u{1f773}: '🝳'
    AlchemicalSymbolForHalfOunce,
}

impl Into<char> for AlchemicalSymbols {
    fn into(self) -> char {
        match self {
            AlchemicalSymbols::AlchemicalSymbolForQuintessence => '🜀',
            AlchemicalSymbols::AlchemicalSymbolForAir => '🜁',
            AlchemicalSymbols::AlchemicalSymbolForFire => '🜂',
            AlchemicalSymbols::AlchemicalSymbolForEarth => '🜃',
            AlchemicalSymbols::AlchemicalSymbolForWater => '🜄',
            AlchemicalSymbols::AlchemicalSymbolForAquafortis => '🜅',
            AlchemicalSymbols::AlchemicalSymbolForAquaRegia => '🜆',
            AlchemicalSymbols::AlchemicalSymbolForAquaRegiaDash2 => '🜇',
            AlchemicalSymbols::AlchemicalSymbolForAquaVitae => '🜈',
            AlchemicalSymbols::AlchemicalSymbolForAquaVitaeDash2 => '🜉',
            AlchemicalSymbols::AlchemicalSymbolForVinegar => '🜊',
            AlchemicalSymbols::AlchemicalSymbolForVinegarDash2 => '🜋',
            AlchemicalSymbols::AlchemicalSymbolForVinegarDash3 => '🜌',
            AlchemicalSymbols::AlchemicalSymbolForSulfur => '🜍',
            AlchemicalSymbols::AlchemicalSymbolForPhilosophersSulfur => '🜎',
            AlchemicalSymbols::AlchemicalSymbolForBlackSulfur => '🜏',
            AlchemicalSymbols::AlchemicalSymbolForMercurySublimate => '🜐',
            AlchemicalSymbols::AlchemicalSymbolForMercurySublimateDash2 => '🜑',
            AlchemicalSymbols::AlchemicalSymbolForMercurySublimateDash3 => '🜒',
            AlchemicalSymbols::AlchemicalSymbolForCinnabar => '🜓',
            AlchemicalSymbols::AlchemicalSymbolForSalt => '🜔',
            AlchemicalSymbols::AlchemicalSymbolForNitre => '🜕',
            AlchemicalSymbols::AlchemicalSymbolForVitriol => '🜖',
            AlchemicalSymbols::AlchemicalSymbolForVitriolDash2 => '🜗',
            AlchemicalSymbols::AlchemicalSymbolForRockSalt => '🜘',
            AlchemicalSymbols::AlchemicalSymbolForRockSaltDash2 => '🜙',
            AlchemicalSymbols::AlchemicalSymbolForGold => '🜚',
            AlchemicalSymbols::AlchemicalSymbolForSilver => '🜛',
            AlchemicalSymbols::AlchemicalSymbolForIronOre => '🜜',
            AlchemicalSymbols::AlchemicalSymbolForIronOreDash2 => '🜝',
            AlchemicalSymbols::AlchemicalSymbolForCrocusOfIron => '🜞',
            AlchemicalSymbols::AlchemicalSymbolForRegulusOfIron => '🜟',
            AlchemicalSymbols::AlchemicalSymbolForCopperOre => '🜠',
            AlchemicalSymbols::AlchemicalSymbolForIronDashCopperOre => '🜡',
            AlchemicalSymbols::AlchemicalSymbolForSublimateOfCopper => '🜢',
            AlchemicalSymbols::AlchemicalSymbolForCrocusOfCopper => '🜣',
            AlchemicalSymbols::AlchemicalSymbolForCrocusOfCopperDash2 => '🜤',
            AlchemicalSymbols::AlchemicalSymbolForCopperAntimoniate => '🜥',
            AlchemicalSymbols::AlchemicalSymbolForSaltOfCopperAntimoniate => '🜦',
            AlchemicalSymbols::AlchemicalSymbolForSublimateOfSaltOfCopper => '🜧',
            AlchemicalSymbols::AlchemicalSymbolForVerdigris => '🜨',
            AlchemicalSymbols::AlchemicalSymbolForTinOre => '🜩',
            AlchemicalSymbols::AlchemicalSymbolForLeadOre => '🜪',
            AlchemicalSymbols::AlchemicalSymbolForAntimonyOre => '🜫',
            AlchemicalSymbols::AlchemicalSymbolForSublimateOfAntimony => '🜬',
            AlchemicalSymbols::AlchemicalSymbolForSaltOfAntimony => '🜭',
            AlchemicalSymbols::AlchemicalSymbolForSublimateOfSaltOfAntimony => '🜮',
            AlchemicalSymbols::AlchemicalSymbolForVinegarOfAntimony => '🜯',
            AlchemicalSymbols::AlchemicalSymbolForRegulusOfAntimony => '🜰',
            AlchemicalSymbols::AlchemicalSymbolForRegulusOfAntimonyDash2 => '🜱',
            AlchemicalSymbols::AlchemicalSymbolForRegulus => '🜲',
            AlchemicalSymbols::AlchemicalSymbolForRegulusDash2 => '🜳',
            AlchemicalSymbols::AlchemicalSymbolForRegulusDash3 => '🜴',
            AlchemicalSymbols::AlchemicalSymbolForRegulusDash4 => '🜵',
            AlchemicalSymbols::AlchemicalSymbolForAlkali => '🜶',
            AlchemicalSymbols::AlchemicalSymbolForAlkaliDash2 => '🜷',
            AlchemicalSymbols::AlchemicalSymbolForMarcasite => '🜸',
            AlchemicalSymbols::AlchemicalSymbolForSalDashAmmoniac => '🜹',
            AlchemicalSymbols::AlchemicalSymbolForArsenic => '🜺',
            AlchemicalSymbols::AlchemicalSymbolForRealgar => '🜻',
            AlchemicalSymbols::AlchemicalSymbolForRealgarDash2 => '🜼',
            AlchemicalSymbols::AlchemicalSymbolForAuripigment => '🜽',
            AlchemicalSymbols::AlchemicalSymbolForBismuthOre => '🜾',
            AlchemicalSymbols::AlchemicalSymbolForTartar => '🜿',
            AlchemicalSymbols::AlchemicalSymbolForTartarDash2 => '🝀',
            AlchemicalSymbols::AlchemicalSymbolForQuickLime => '🝁',
            AlchemicalSymbols::AlchemicalSymbolForBorax => '🝂',
            AlchemicalSymbols::AlchemicalSymbolForBoraxDash2 => '🝃',
            AlchemicalSymbols::AlchemicalSymbolForBoraxDash3 => '🝄',
            AlchemicalSymbols::AlchemicalSymbolForAlum => '🝅',
            AlchemicalSymbols::AlchemicalSymbolForOil => '🝆',
            AlchemicalSymbols::AlchemicalSymbolForSpirit => '🝇',
            AlchemicalSymbols::AlchemicalSymbolForTincture => '🝈',
            AlchemicalSymbols::AlchemicalSymbolForGum => '🝉',
            AlchemicalSymbols::AlchemicalSymbolForWax => '🝊',
            AlchemicalSymbols::AlchemicalSymbolForPowder => '🝋',
            AlchemicalSymbols::AlchemicalSymbolForCalx => '🝌',
            AlchemicalSymbols::AlchemicalSymbolForTutty => '🝍',
            AlchemicalSymbols::AlchemicalSymbolForCaputMortuum => '🝎',
            AlchemicalSymbols::AlchemicalSymbolForScepterOfJove => '🝏',
            AlchemicalSymbols::AlchemicalSymbolForCaduceus => '🝐',
            AlchemicalSymbols::AlchemicalSymbolForTrident => '🝑',
            AlchemicalSymbols::AlchemicalSymbolForStarredTrident => '🝒',
            AlchemicalSymbols::AlchemicalSymbolForLodestone => '🝓',
            AlchemicalSymbols::AlchemicalSymbolForSoap => '🝔',
            AlchemicalSymbols::AlchemicalSymbolForUrine => '🝕',
            AlchemicalSymbols::AlchemicalSymbolForHorseDung => '🝖',
            AlchemicalSymbols::AlchemicalSymbolForAshes => '🝗',
            AlchemicalSymbols::AlchemicalSymbolForPotAshes => '🝘',
            AlchemicalSymbols::AlchemicalSymbolForBrick => '🝙',
            AlchemicalSymbols::AlchemicalSymbolForPowderedBrick => '🝚',
            AlchemicalSymbols::AlchemicalSymbolForAmalgam => '🝛',
            AlchemicalSymbols::AlchemicalSymbolForStratumSuperStratum => '🝜',
            AlchemicalSymbols::AlchemicalSymbolForStratumSuperStratumDash2 => '🝝',
            AlchemicalSymbols::AlchemicalSymbolForSublimation => '🝞',
            AlchemicalSymbols::AlchemicalSymbolForPrecipitate => '🝟',
            AlchemicalSymbols::AlchemicalSymbolForDistill => '🝠',
            AlchemicalSymbols::AlchemicalSymbolForDissolve => '🝡',
            AlchemicalSymbols::AlchemicalSymbolForDissolveDash2 => '🝢',
            AlchemicalSymbols::AlchemicalSymbolForPurify => '🝣',
            AlchemicalSymbols::AlchemicalSymbolForPutrefaction => '🝤',
            AlchemicalSymbols::AlchemicalSymbolForCrucible => '🝥',
            AlchemicalSymbols::AlchemicalSymbolForCrucibleDash2 => '🝦',
            AlchemicalSymbols::AlchemicalSymbolForCrucibleDash3 => '🝧',
            AlchemicalSymbols::AlchemicalSymbolForCrucibleDash4 => '🝨',
            AlchemicalSymbols::AlchemicalSymbolForCrucibleDash5 => '🝩',
            AlchemicalSymbols::AlchemicalSymbolForAlembic => '🝪',
            AlchemicalSymbols::AlchemicalSymbolForBathOfMary => '🝫',
            AlchemicalSymbols::AlchemicalSymbolForBathOfVapours => '🝬',
            AlchemicalSymbols::AlchemicalSymbolForRetort => '🝭',
            AlchemicalSymbols::AlchemicalSymbolForHour => '🝮',
            AlchemicalSymbols::AlchemicalSymbolForNight => '🝯',
            AlchemicalSymbols::AlchemicalSymbolForDayDashNight => '🝰',
            AlchemicalSymbols::AlchemicalSymbolForMonth => '🝱',
            AlchemicalSymbols::AlchemicalSymbolForHalfDram => '🝲',
            AlchemicalSymbols::AlchemicalSymbolForHalfOunce => '🝳',
        }
    }
}

impl std::convert::TryFrom<char> for AlchemicalSymbols {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '🜀' => Ok(AlchemicalSymbols::AlchemicalSymbolForQuintessence),
            '🜁' => Ok(AlchemicalSymbols::AlchemicalSymbolForAir),
            '🜂' => Ok(AlchemicalSymbols::AlchemicalSymbolForFire),
            '🜃' => Ok(AlchemicalSymbols::AlchemicalSymbolForEarth),
            '🜄' => Ok(AlchemicalSymbols::AlchemicalSymbolForWater),
            '🜅' => Ok(AlchemicalSymbols::AlchemicalSymbolForAquafortis),
            '🜆' => Ok(AlchemicalSymbols::AlchemicalSymbolForAquaRegia),
            '🜇' => Ok(AlchemicalSymbols::AlchemicalSymbolForAquaRegiaDash2),
            '🜈' => Ok(AlchemicalSymbols::AlchemicalSymbolForAquaVitae),
            '🜉' => Ok(AlchemicalSymbols::AlchemicalSymbolForAquaVitaeDash2),
            '🜊' => Ok(AlchemicalSymbols::AlchemicalSymbolForVinegar),
            '🜋' => Ok(AlchemicalSymbols::AlchemicalSymbolForVinegarDash2),
            '🜌' => Ok(AlchemicalSymbols::AlchemicalSymbolForVinegarDash3),
            '🜍' => Ok(AlchemicalSymbols::AlchemicalSymbolForSulfur),
            '🜎' => Ok(AlchemicalSymbols::AlchemicalSymbolForPhilosophersSulfur),
            '🜏' => Ok(AlchemicalSymbols::AlchemicalSymbolForBlackSulfur),
            '🜐' => Ok(AlchemicalSymbols::AlchemicalSymbolForMercurySublimate),
            '🜑' => Ok(AlchemicalSymbols::AlchemicalSymbolForMercurySublimateDash2),
            '🜒' => Ok(AlchemicalSymbols::AlchemicalSymbolForMercurySublimateDash3),
            '🜓' => Ok(AlchemicalSymbols::AlchemicalSymbolForCinnabar),
            '🜔' => Ok(AlchemicalSymbols::AlchemicalSymbolForSalt),
            '🜕' => Ok(AlchemicalSymbols::AlchemicalSymbolForNitre),
            '🜖' => Ok(AlchemicalSymbols::AlchemicalSymbolForVitriol),
            '🜗' => Ok(AlchemicalSymbols::AlchemicalSymbolForVitriolDash2),
            '🜘' => Ok(AlchemicalSymbols::AlchemicalSymbolForRockSalt),
            '🜙' => Ok(AlchemicalSymbols::AlchemicalSymbolForRockSaltDash2),
            '🜚' => Ok(AlchemicalSymbols::AlchemicalSymbolForGold),
            '🜛' => Ok(AlchemicalSymbols::AlchemicalSymbolForSilver),
            '🜜' => Ok(AlchemicalSymbols::AlchemicalSymbolForIronOre),
            '🜝' => Ok(AlchemicalSymbols::AlchemicalSymbolForIronOreDash2),
            '🜞' => Ok(AlchemicalSymbols::AlchemicalSymbolForCrocusOfIron),
            '🜟' => Ok(AlchemicalSymbols::AlchemicalSymbolForRegulusOfIron),
            '🜠' => Ok(AlchemicalSymbols::AlchemicalSymbolForCopperOre),
            '🜡' => Ok(AlchemicalSymbols::AlchemicalSymbolForIronDashCopperOre),
            '🜢' => Ok(AlchemicalSymbols::AlchemicalSymbolForSublimateOfCopper),
            '🜣' => Ok(AlchemicalSymbols::AlchemicalSymbolForCrocusOfCopper),
            '🜤' => Ok(AlchemicalSymbols::AlchemicalSymbolForCrocusOfCopperDash2),
            '🜥' => Ok(AlchemicalSymbols::AlchemicalSymbolForCopperAntimoniate),
            '🜦' => Ok(AlchemicalSymbols::AlchemicalSymbolForSaltOfCopperAntimoniate),
            '🜧' => Ok(AlchemicalSymbols::AlchemicalSymbolForSublimateOfSaltOfCopper),
            '🜨' => Ok(AlchemicalSymbols::AlchemicalSymbolForVerdigris),
            '🜩' => Ok(AlchemicalSymbols::AlchemicalSymbolForTinOre),
            '🜪' => Ok(AlchemicalSymbols::AlchemicalSymbolForLeadOre),
            '🜫' => Ok(AlchemicalSymbols::AlchemicalSymbolForAntimonyOre),
            '🜬' => Ok(AlchemicalSymbols::AlchemicalSymbolForSublimateOfAntimony),
            '🜭' => Ok(AlchemicalSymbols::AlchemicalSymbolForSaltOfAntimony),
            '🜮' => Ok(AlchemicalSymbols::AlchemicalSymbolForSublimateOfSaltOfAntimony),
            '🜯' => Ok(AlchemicalSymbols::AlchemicalSymbolForVinegarOfAntimony),
            '🜰' => Ok(AlchemicalSymbols::AlchemicalSymbolForRegulusOfAntimony),
            '🜱' => Ok(AlchemicalSymbols::AlchemicalSymbolForRegulusOfAntimonyDash2),
            '🜲' => Ok(AlchemicalSymbols::AlchemicalSymbolForRegulus),
            '🜳' => Ok(AlchemicalSymbols::AlchemicalSymbolForRegulusDash2),
            '🜴' => Ok(AlchemicalSymbols::AlchemicalSymbolForRegulusDash3),
            '🜵' => Ok(AlchemicalSymbols::AlchemicalSymbolForRegulusDash4),
            '🜶' => Ok(AlchemicalSymbols::AlchemicalSymbolForAlkali),
            '🜷' => Ok(AlchemicalSymbols::AlchemicalSymbolForAlkaliDash2),
            '🜸' => Ok(AlchemicalSymbols::AlchemicalSymbolForMarcasite),
            '🜹' => Ok(AlchemicalSymbols::AlchemicalSymbolForSalDashAmmoniac),
            '🜺' => Ok(AlchemicalSymbols::AlchemicalSymbolForArsenic),
            '🜻' => Ok(AlchemicalSymbols::AlchemicalSymbolForRealgar),
            '🜼' => Ok(AlchemicalSymbols::AlchemicalSymbolForRealgarDash2),
            '🜽' => Ok(AlchemicalSymbols::AlchemicalSymbolForAuripigment),
            '🜾' => Ok(AlchemicalSymbols::AlchemicalSymbolForBismuthOre),
            '🜿' => Ok(AlchemicalSymbols::AlchemicalSymbolForTartar),
            '🝀' => Ok(AlchemicalSymbols::AlchemicalSymbolForTartarDash2),
            '🝁' => Ok(AlchemicalSymbols::AlchemicalSymbolForQuickLime),
            '🝂' => Ok(AlchemicalSymbols::AlchemicalSymbolForBorax),
            '🝃' => Ok(AlchemicalSymbols::AlchemicalSymbolForBoraxDash2),
            '🝄' => Ok(AlchemicalSymbols::AlchemicalSymbolForBoraxDash3),
            '🝅' => Ok(AlchemicalSymbols::AlchemicalSymbolForAlum),
            '🝆' => Ok(AlchemicalSymbols::AlchemicalSymbolForOil),
            '🝇' => Ok(AlchemicalSymbols::AlchemicalSymbolForSpirit),
            '🝈' => Ok(AlchemicalSymbols::AlchemicalSymbolForTincture),
            '🝉' => Ok(AlchemicalSymbols::AlchemicalSymbolForGum),
            '🝊' => Ok(AlchemicalSymbols::AlchemicalSymbolForWax),
            '🝋' => Ok(AlchemicalSymbols::AlchemicalSymbolForPowder),
            '🝌' => Ok(AlchemicalSymbols::AlchemicalSymbolForCalx),
            '🝍' => Ok(AlchemicalSymbols::AlchemicalSymbolForTutty),
            '🝎' => Ok(AlchemicalSymbols::AlchemicalSymbolForCaputMortuum),
            '🝏' => Ok(AlchemicalSymbols::AlchemicalSymbolForScepterOfJove),
            '🝐' => Ok(AlchemicalSymbols::AlchemicalSymbolForCaduceus),
            '🝑' => Ok(AlchemicalSymbols::AlchemicalSymbolForTrident),
            '🝒' => Ok(AlchemicalSymbols::AlchemicalSymbolForStarredTrident),
            '🝓' => Ok(AlchemicalSymbols::AlchemicalSymbolForLodestone),
            '🝔' => Ok(AlchemicalSymbols::AlchemicalSymbolForSoap),
            '🝕' => Ok(AlchemicalSymbols::AlchemicalSymbolForUrine),
            '🝖' => Ok(AlchemicalSymbols::AlchemicalSymbolForHorseDung),
            '🝗' => Ok(AlchemicalSymbols::AlchemicalSymbolForAshes),
            '🝘' => Ok(AlchemicalSymbols::AlchemicalSymbolForPotAshes),
            '🝙' => Ok(AlchemicalSymbols::AlchemicalSymbolForBrick),
            '🝚' => Ok(AlchemicalSymbols::AlchemicalSymbolForPowderedBrick),
            '🝛' => Ok(AlchemicalSymbols::AlchemicalSymbolForAmalgam),
            '🝜' => Ok(AlchemicalSymbols::AlchemicalSymbolForStratumSuperStratum),
            '🝝' => Ok(AlchemicalSymbols::AlchemicalSymbolForStratumSuperStratumDash2),
            '🝞' => Ok(AlchemicalSymbols::AlchemicalSymbolForSublimation),
            '🝟' => Ok(AlchemicalSymbols::AlchemicalSymbolForPrecipitate),
            '🝠' => Ok(AlchemicalSymbols::AlchemicalSymbolForDistill),
            '🝡' => Ok(AlchemicalSymbols::AlchemicalSymbolForDissolve),
            '🝢' => Ok(AlchemicalSymbols::AlchemicalSymbolForDissolveDash2),
            '🝣' => Ok(AlchemicalSymbols::AlchemicalSymbolForPurify),
            '🝤' => Ok(AlchemicalSymbols::AlchemicalSymbolForPutrefaction),
            '🝥' => Ok(AlchemicalSymbols::AlchemicalSymbolForCrucible),
            '🝦' => Ok(AlchemicalSymbols::AlchemicalSymbolForCrucibleDash2),
            '🝧' => Ok(AlchemicalSymbols::AlchemicalSymbolForCrucibleDash3),
            '🝨' => Ok(AlchemicalSymbols::AlchemicalSymbolForCrucibleDash4),
            '🝩' => Ok(AlchemicalSymbols::AlchemicalSymbolForCrucibleDash5),
            '🝪' => Ok(AlchemicalSymbols::AlchemicalSymbolForAlembic),
            '🝫' => Ok(AlchemicalSymbols::AlchemicalSymbolForBathOfMary),
            '🝬' => Ok(AlchemicalSymbols::AlchemicalSymbolForBathOfVapours),
            '🝭' => Ok(AlchemicalSymbols::AlchemicalSymbolForRetort),
            '🝮' => Ok(AlchemicalSymbols::AlchemicalSymbolForHour),
            '🝯' => Ok(AlchemicalSymbols::AlchemicalSymbolForNight),
            '🝰' => Ok(AlchemicalSymbols::AlchemicalSymbolForDayDashNight),
            '🝱' => Ok(AlchemicalSymbols::AlchemicalSymbolForMonth),
            '🝲' => Ok(AlchemicalSymbols::AlchemicalSymbolForHalfDram),
            '🝳' => Ok(AlchemicalSymbols::AlchemicalSymbolForHalfOunce),
            _ => Err(()),
        }
    }
}

impl Into<u32> for AlchemicalSymbols {
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

impl std::convert::TryFrom<u32> for AlchemicalSymbols {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for AlchemicalSymbols {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl AlchemicalSymbols {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        AlchemicalSymbols::AlchemicalSymbolForQuintessence
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            AlchemicalSymbols::AlchemicalSymbolForQuintessence => "alchemical symbol for quintessence",
            AlchemicalSymbols::AlchemicalSymbolForAir => "alchemical symbol for air",
            AlchemicalSymbols::AlchemicalSymbolForFire => "alchemical symbol for fire",
            AlchemicalSymbols::AlchemicalSymbolForEarth => "alchemical symbol for earth",
            AlchemicalSymbols::AlchemicalSymbolForWater => "alchemical symbol for water",
            AlchemicalSymbols::AlchemicalSymbolForAquafortis => "alchemical symbol for aquafortis",
            AlchemicalSymbols::AlchemicalSymbolForAquaRegia => "alchemical symbol for aqua regia",
            AlchemicalSymbols::AlchemicalSymbolForAquaRegiaDash2 => "alchemical symbol for aqua regia-2",
            AlchemicalSymbols::AlchemicalSymbolForAquaVitae => "alchemical symbol for aqua vitae",
            AlchemicalSymbols::AlchemicalSymbolForAquaVitaeDash2 => "alchemical symbol for aqua vitae-2",
            AlchemicalSymbols::AlchemicalSymbolForVinegar => "alchemical symbol for vinegar",
            AlchemicalSymbols::AlchemicalSymbolForVinegarDash2 => "alchemical symbol for vinegar-2",
            AlchemicalSymbols::AlchemicalSymbolForVinegarDash3 => "alchemical symbol for vinegar-3",
            AlchemicalSymbols::AlchemicalSymbolForSulfur => "alchemical symbol for sulfur",
            AlchemicalSymbols::AlchemicalSymbolForPhilosophersSulfur => "alchemical symbol for philosophers sulfur",
            AlchemicalSymbols::AlchemicalSymbolForBlackSulfur => "alchemical symbol for black sulfur",
            AlchemicalSymbols::AlchemicalSymbolForMercurySublimate => "alchemical symbol for mercury sublimate",
            AlchemicalSymbols::AlchemicalSymbolForMercurySublimateDash2 => "alchemical symbol for mercury sublimate-2",
            AlchemicalSymbols::AlchemicalSymbolForMercurySublimateDash3 => "alchemical symbol for mercury sublimate-3",
            AlchemicalSymbols::AlchemicalSymbolForCinnabar => "alchemical symbol for cinnabar",
            AlchemicalSymbols::AlchemicalSymbolForSalt => "alchemical symbol for salt",
            AlchemicalSymbols::AlchemicalSymbolForNitre => "alchemical symbol for nitre",
            AlchemicalSymbols::AlchemicalSymbolForVitriol => "alchemical symbol for vitriol",
            AlchemicalSymbols::AlchemicalSymbolForVitriolDash2 => "alchemical symbol for vitriol-2",
            AlchemicalSymbols::AlchemicalSymbolForRockSalt => "alchemical symbol for rock salt",
            AlchemicalSymbols::AlchemicalSymbolForRockSaltDash2 => "alchemical symbol for rock salt-2",
            AlchemicalSymbols::AlchemicalSymbolForGold => "alchemical symbol for gold",
            AlchemicalSymbols::AlchemicalSymbolForSilver => "alchemical symbol for silver",
            AlchemicalSymbols::AlchemicalSymbolForIronOre => "alchemical symbol for iron ore",
            AlchemicalSymbols::AlchemicalSymbolForIronOreDash2 => "alchemical symbol for iron ore-2",
            AlchemicalSymbols::AlchemicalSymbolForCrocusOfIron => "alchemical symbol for crocus of iron",
            AlchemicalSymbols::AlchemicalSymbolForRegulusOfIron => "alchemical symbol for regulus of iron",
            AlchemicalSymbols::AlchemicalSymbolForCopperOre => "alchemical symbol for copper ore",
            AlchemicalSymbols::AlchemicalSymbolForIronDashCopperOre => "alchemical symbol for iron-copper ore",
            AlchemicalSymbols::AlchemicalSymbolForSublimateOfCopper => "alchemical symbol for sublimate of copper",
            AlchemicalSymbols::AlchemicalSymbolForCrocusOfCopper => "alchemical symbol for crocus of copper",
            AlchemicalSymbols::AlchemicalSymbolForCrocusOfCopperDash2 => "alchemical symbol for crocus of copper-2",
            AlchemicalSymbols::AlchemicalSymbolForCopperAntimoniate => "alchemical symbol for copper antimoniate",
            AlchemicalSymbols::AlchemicalSymbolForSaltOfCopperAntimoniate => "alchemical symbol for salt of copper antimoniate",
            AlchemicalSymbols::AlchemicalSymbolForSublimateOfSaltOfCopper => "alchemical symbol for sublimate of salt of copper",
            AlchemicalSymbols::AlchemicalSymbolForVerdigris => "alchemical symbol for verdigris",
            AlchemicalSymbols::AlchemicalSymbolForTinOre => "alchemical symbol for tin ore",
            AlchemicalSymbols::AlchemicalSymbolForLeadOre => "alchemical symbol for lead ore",
            AlchemicalSymbols::AlchemicalSymbolForAntimonyOre => "alchemical symbol for antimony ore",
            AlchemicalSymbols::AlchemicalSymbolForSublimateOfAntimony => "alchemical symbol for sublimate of antimony",
            AlchemicalSymbols::AlchemicalSymbolForSaltOfAntimony => "alchemical symbol for salt of antimony",
            AlchemicalSymbols::AlchemicalSymbolForSublimateOfSaltOfAntimony => "alchemical symbol for sublimate of salt of antimony",
            AlchemicalSymbols::AlchemicalSymbolForVinegarOfAntimony => "alchemical symbol for vinegar of antimony",
            AlchemicalSymbols::AlchemicalSymbolForRegulusOfAntimony => "alchemical symbol for regulus of antimony",
            AlchemicalSymbols::AlchemicalSymbolForRegulusOfAntimonyDash2 => "alchemical symbol for regulus of antimony-2",
            AlchemicalSymbols::AlchemicalSymbolForRegulus => "alchemical symbol for regulus",
            AlchemicalSymbols::AlchemicalSymbolForRegulusDash2 => "alchemical symbol for regulus-2",
            AlchemicalSymbols::AlchemicalSymbolForRegulusDash3 => "alchemical symbol for regulus-3",
            AlchemicalSymbols::AlchemicalSymbolForRegulusDash4 => "alchemical symbol for regulus-4",
            AlchemicalSymbols::AlchemicalSymbolForAlkali => "alchemical symbol for alkali",
            AlchemicalSymbols::AlchemicalSymbolForAlkaliDash2 => "alchemical symbol for alkali-2",
            AlchemicalSymbols::AlchemicalSymbolForMarcasite => "alchemical symbol for marcasite",
            AlchemicalSymbols::AlchemicalSymbolForSalDashAmmoniac => "alchemical symbol for sal-ammoniac",
            AlchemicalSymbols::AlchemicalSymbolForArsenic => "alchemical symbol for arsenic",
            AlchemicalSymbols::AlchemicalSymbolForRealgar => "alchemical symbol for realgar",
            AlchemicalSymbols::AlchemicalSymbolForRealgarDash2 => "alchemical symbol for realgar-2",
            AlchemicalSymbols::AlchemicalSymbolForAuripigment => "alchemical symbol for auripigment",
            AlchemicalSymbols::AlchemicalSymbolForBismuthOre => "alchemical symbol for bismuth ore",
            AlchemicalSymbols::AlchemicalSymbolForTartar => "alchemical symbol for tartar",
            AlchemicalSymbols::AlchemicalSymbolForTartarDash2 => "alchemical symbol for tartar-2",
            AlchemicalSymbols::AlchemicalSymbolForQuickLime => "alchemical symbol for quick lime",
            AlchemicalSymbols::AlchemicalSymbolForBorax => "alchemical symbol for borax",
            AlchemicalSymbols::AlchemicalSymbolForBoraxDash2 => "alchemical symbol for borax-2",
            AlchemicalSymbols::AlchemicalSymbolForBoraxDash3 => "alchemical symbol for borax-3",
            AlchemicalSymbols::AlchemicalSymbolForAlum => "alchemical symbol for alum",
            AlchemicalSymbols::AlchemicalSymbolForOil => "alchemical symbol for oil",
            AlchemicalSymbols::AlchemicalSymbolForSpirit => "alchemical symbol for spirit",
            AlchemicalSymbols::AlchemicalSymbolForTincture => "alchemical symbol for tincture",
            AlchemicalSymbols::AlchemicalSymbolForGum => "alchemical symbol for gum",
            AlchemicalSymbols::AlchemicalSymbolForWax => "alchemical symbol for wax",
            AlchemicalSymbols::AlchemicalSymbolForPowder => "alchemical symbol for powder",
            AlchemicalSymbols::AlchemicalSymbolForCalx => "alchemical symbol for calx",
            AlchemicalSymbols::AlchemicalSymbolForTutty => "alchemical symbol for tutty",
            AlchemicalSymbols::AlchemicalSymbolForCaputMortuum => "alchemical symbol for caput mortuum",
            AlchemicalSymbols::AlchemicalSymbolForScepterOfJove => "alchemical symbol for scepter of jove",
            AlchemicalSymbols::AlchemicalSymbolForCaduceus => "alchemical symbol for caduceus",
            AlchemicalSymbols::AlchemicalSymbolForTrident => "alchemical symbol for trident",
            AlchemicalSymbols::AlchemicalSymbolForStarredTrident => "alchemical symbol for starred trident",
            AlchemicalSymbols::AlchemicalSymbolForLodestone => "alchemical symbol for lodestone",
            AlchemicalSymbols::AlchemicalSymbolForSoap => "alchemical symbol for soap",
            AlchemicalSymbols::AlchemicalSymbolForUrine => "alchemical symbol for urine",
            AlchemicalSymbols::AlchemicalSymbolForHorseDung => "alchemical symbol for horse dung",
            AlchemicalSymbols::AlchemicalSymbolForAshes => "alchemical symbol for ashes",
            AlchemicalSymbols::AlchemicalSymbolForPotAshes => "alchemical symbol for pot ashes",
            AlchemicalSymbols::AlchemicalSymbolForBrick => "alchemical symbol for brick",
            AlchemicalSymbols::AlchemicalSymbolForPowderedBrick => "alchemical symbol for powdered brick",
            AlchemicalSymbols::AlchemicalSymbolForAmalgam => "alchemical symbol for amalgam",
            AlchemicalSymbols::AlchemicalSymbolForStratumSuperStratum => "alchemical symbol for stratum super stratum",
            AlchemicalSymbols::AlchemicalSymbolForStratumSuperStratumDash2 => "alchemical symbol for stratum super stratum-2",
            AlchemicalSymbols::AlchemicalSymbolForSublimation => "alchemical symbol for sublimation",
            AlchemicalSymbols::AlchemicalSymbolForPrecipitate => "alchemical symbol for precipitate",
            AlchemicalSymbols::AlchemicalSymbolForDistill => "alchemical symbol for distill",
            AlchemicalSymbols::AlchemicalSymbolForDissolve => "alchemical symbol for dissolve",
            AlchemicalSymbols::AlchemicalSymbolForDissolveDash2 => "alchemical symbol for dissolve-2",
            AlchemicalSymbols::AlchemicalSymbolForPurify => "alchemical symbol for purify",
            AlchemicalSymbols::AlchemicalSymbolForPutrefaction => "alchemical symbol for putrefaction",
            AlchemicalSymbols::AlchemicalSymbolForCrucible => "alchemical symbol for crucible",
            AlchemicalSymbols::AlchemicalSymbolForCrucibleDash2 => "alchemical symbol for crucible-2",
            AlchemicalSymbols::AlchemicalSymbolForCrucibleDash3 => "alchemical symbol for crucible-3",
            AlchemicalSymbols::AlchemicalSymbolForCrucibleDash4 => "alchemical symbol for crucible-4",
            AlchemicalSymbols::AlchemicalSymbolForCrucibleDash5 => "alchemical symbol for crucible-5",
            AlchemicalSymbols::AlchemicalSymbolForAlembic => "alchemical symbol for alembic",
            AlchemicalSymbols::AlchemicalSymbolForBathOfMary => "alchemical symbol for bath of mary",
            AlchemicalSymbols::AlchemicalSymbolForBathOfVapours => "alchemical symbol for bath of vapours",
            AlchemicalSymbols::AlchemicalSymbolForRetort => "alchemical symbol for retort",
            AlchemicalSymbols::AlchemicalSymbolForHour => "alchemical symbol for hour",
            AlchemicalSymbols::AlchemicalSymbolForNight => "alchemical symbol for night",
            AlchemicalSymbols::AlchemicalSymbolForDayDashNight => "alchemical symbol for day-night",
            AlchemicalSymbols::AlchemicalSymbolForMonth => "alchemical symbol for month",
            AlchemicalSymbols::AlchemicalSymbolForHalfDram => "alchemical symbol for half dram",
            AlchemicalSymbols::AlchemicalSymbolForHalfOunce => "alchemical symbol for half ounce",
        }
    }
}
