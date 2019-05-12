
/// An enum to represent all characters in the EarlyDynasticCuneiform block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum EarlyDynasticCuneiform {
    /// \u{12480}: '𒒀'
    CuneiformSignAbTimesNunTenu,
    /// \u{12481}: '𒒁'
    CuneiformSignAbTimesShu2,
    /// \u{12482}: '𒒂'
    CuneiformSignAdTimesEsh2,
    /// \u{12483}: '𒒃'
    CuneiformSignBadTimesDishTenu,
    /// \u{12484}: '𒒄'
    CuneiformSignBahar2TimesAb2,
    /// \u{12485}: '𒒅'
    CuneiformSignBahar2TimesNi,
    /// \u{12486}: '𒒆'
    CuneiformSignBahar2TimesZa,
    /// \u{12487}: '𒒇'
    CuneiformSignBuOverBuTimesNa2,
    /// \u{12488}: '𒒈'
    CuneiformSignDaTimesTak4,
    /// \u{12489}: '𒒉'
    CuneiformSignDagTimesKur,
    /// \u{1248a}: '𒒊'
    CuneiformSignDimTimesIgi,
    /// \u{1248b}: '𒒋'
    CuneiformSignDimTimesUUU,
    /// \u{1248c}: '𒒌'
    CuneiformSignDim2TimesUd,
    /// \u{1248d}: '𒒍'
    CuneiformSignDugTimesAnshe,
    /// \u{1248e}: '𒒎'
    CuneiformSignDugTimesAsh,
    /// \u{1248f}: '𒒏'
    CuneiformSignDugTimesAshAtLeft,
    /// \u{12490}: '𒒐'
    CuneiformSignDugTimesDin,
    /// \u{12491}: '𒒑'
    CuneiformSignDugTimesDun,
    /// \u{12492}: '𒒒'
    CuneiformSignDugTimesErin2,
    /// \u{12493}: '𒒓'
    CuneiformSignDugTimesGa,
    /// \u{12494}: '𒒔'
    CuneiformSignDugTimesGi,
    /// \u{12495}: '𒒕'
    CuneiformSignDugTimesGir2Gunu,
    /// \u{12496}: '𒒖'
    CuneiformSignDugTimesGish,
    /// \u{12497}: '𒒗'
    CuneiformSignDugTimesHa,
    /// \u{12498}: '𒒘'
    CuneiformSignDugTimesHi,
    /// \u{12499}: '𒒙'
    CuneiformSignDugTimesIgiGunu,
    /// \u{1249a}: '𒒚'
    CuneiformSignDugTimesKaskal,
    /// \u{1249b}: '𒒛'
    CuneiformSignDugTimesKur,
    /// \u{1249c}: '𒒜'
    CuneiformSignDugTimesKushu2,
    /// \u{1249d}: '𒒝'
    CuneiformSignDugTimesKushu2PlusKaskal,
    /// \u{1249e}: '𒒞'
    CuneiformSignDugTimesLakDash020,
    /// \u{1249f}: '𒒟'
    CuneiformSignDugTimesLam,
    /// \u{124a0}: '𒒠'
    CuneiformSignDugTimesLamTimesKur,
    /// \u{124a1}: '𒒡'
    CuneiformSignDugTimesLuhPlusGish,
    /// \u{124a2}: '𒒢'
    CuneiformSignDugTimesMash,
    /// \u{124a3}: '𒒣'
    CuneiformSignDugTimesMes,
    /// \u{124a4}: '𒒤'
    CuneiformSignDugTimesMi,
    /// \u{124a5}: '𒒥'
    CuneiformSignDugTimesNi,
    /// \u{124a6}: '𒒦'
    CuneiformSignDugTimesPi,
    /// \u{124a7}: '𒒧'
    CuneiformSignDugTimesShe,
    /// \u{124a8}: '𒒨'
    CuneiformSignDugTimesSiGunu,
    /// \u{124a9}: '𒒩'
    CuneiformSignE2TimesKur,
    /// \u{124aa}: '𒒪'
    CuneiformSignE2TimesPap,
    /// \u{124ab}: '𒒫'
    CuneiformSignErin2X,
    /// \u{124ac}: '𒒬'
    CuneiformSignEsh2CrossingEsh2,
    /// \u{124ad}: '𒒭'
    CuneiformSignEzenSheshigTimesAsh,
    /// \u{124ae}: '𒒮'
    CuneiformSignEzenSheshigTimesHi,
    /// \u{124af}: '𒒯'
    CuneiformSignEzenSheshigTimesIgiGunu,
    /// \u{124b0}: '𒒰'
    CuneiformSignEzenSheshigTimesLa,
    /// \u{124b1}: '𒒱'
    CuneiformSignEzenSheshigTimesLal,
    /// \u{124b2}: '𒒲'
    CuneiformSignEzenSheshigTimesMe,
    /// \u{124b3}: '𒒳'
    CuneiformSignEzenSheshigTimesMes,
    /// \u{124b4}: '𒒴'
    CuneiformSignEzenSheshigTimesSu,
    /// \u{124b5}: '𒒵'
    CuneiformSignEzenTimesSu,
    /// \u{124b6}: '𒒶'
    CuneiformSignGa2TimesBahar2,
    /// \u{124b7}: '𒒷'
    CuneiformSignGa2TimesDimGunu,
    /// \u{124b8}: '𒒸'
    CuneiformSignGa2TimesDugTimesIgiGunu,
    /// \u{124b9}: '𒒹'
    CuneiformSignGa2TimesDugTimesKaskal,
    /// \u{124ba}: '𒒺'
    CuneiformSignGa2TimesEren,
    /// \u{124bb}: '𒒻'
    CuneiformSignGa2TimesGa,
    /// \u{124bc}: '𒒼'
    CuneiformSignGa2TimesGarPlusDi,
    /// \u{124bd}: '𒒽'
    CuneiformSignGa2TimesGarPlusNe,
    /// \u{124be}: '𒒾'
    CuneiformSignGa2TimesHaPlusA,
    /// \u{124bf}: '𒒿'
    CuneiformSignGa2TimesKushu2PlusKaskal,
    /// \u{124c0}: '𒓀'
    CuneiformSignGa2TimesLam,
    /// \u{124c1}: '𒓁'
    CuneiformSignGa2TimesLamTimesKur,
    /// \u{124c2}: '𒓂'
    CuneiformSignGa2TimesLuh,
    /// \u{124c3}: '𒓃'
    CuneiformSignGa2TimesMush,
    /// \u{124c4}: '𒓄'
    CuneiformSignGa2TimesNe,
    /// \u{124c5}: '𒓅'
    CuneiformSignGa2TimesNePlusE2,
    /// \u{124c6}: '𒓆'
    CuneiformSignGa2TimesNePlusGi,
    /// \u{124c7}: '𒓇'
    CuneiformSignGa2TimesShim,
    /// \u{124c8}: '𒓈'
    CuneiformSignGa2TimesZiz2,
    /// \u{124c9}: '𒓉'
    CuneiformSignGabaRotatedNinetyDegrees,
    /// \u{124ca}: '𒓊'
    CuneiformSignGeshtinTimesU,
    /// \u{124cb}: '𒓋'
    CuneiformSignGishTimesGishCrossingGish,
    /// \u{124cc}: '𒓌'
    CuneiformSignGu2TimesIgiGunu,
    /// \u{124cd}: '𒓍'
    CuneiformSignGudPlusGishTimesTak4,
    /// \u{124ce}: '𒓎'
    CuneiformSignHaTenuGunu,
    /// \u{124cf}: '𒓏'
    CuneiformSignHiTimesAshOverHiTimesAsh,
    /// \u{124d0}: '𒓐'
    CuneiformSignKaTimesBu,
    /// \u{124d1}: '𒓑'
    CuneiformSignKaTimesKa,
    /// \u{124d2}: '𒓒'
    CuneiformSignKaTimesUUU,
    /// \u{124d3}: '𒓓'
    CuneiformSignKaTimesUr,
    /// \u{124d4}: '𒓔'
    CuneiformSignLagabTimesZuOverZu,
    /// \u{124d5}: '𒓕'
    CuneiformSignLakDash003,
    /// \u{124d6}: '𒓖'
    CuneiformSignLakDash021,
    /// \u{124d7}: '𒓗'
    CuneiformSignLakDash025,
    /// \u{124d8}: '𒓘'
    CuneiformSignLakDash030,
    /// \u{124d9}: '𒓙'
    CuneiformSignLakDash050,
    /// \u{124da}: '𒓚'
    CuneiformSignLakDash051,
    /// \u{124db}: '𒓛'
    CuneiformSignLakDash062,
    /// \u{124dc}: '𒓜'
    CuneiformSignLakDash079OverLakDash079Gunu,
    /// \u{124dd}: '𒓝'
    CuneiformSignLakDash080,
    /// \u{124de}: '𒓞'
    CuneiformSignLakDash081OverLakDash081,
    /// \u{124df}: '𒓟'
    CuneiformSignLakDash092,
    /// \u{124e0}: '𒓠'
    CuneiformSignLakDash130,
    /// \u{124e1}: '𒓡'
    CuneiformSignLakDash142,
    /// \u{124e2}: '𒓢'
    CuneiformSignLakDash210,
    /// \u{124e3}: '𒓣'
    CuneiformSignLakDash219,
    /// \u{124e4}: '𒓤'
    CuneiformSignLakDash220,
    /// \u{124e5}: '𒓥'
    CuneiformSignLakDash225,
    /// \u{124e6}: '𒓦'
    CuneiformSignLakDash228,
    /// \u{124e7}: '𒓧'
    CuneiformSignLakDash238,
    /// \u{124e8}: '𒓨'
    CuneiformSignLakDash265,
    /// \u{124e9}: '𒓩'
    CuneiformSignLakDash266,
    /// \u{124ea}: '𒓪'
    CuneiformSignLakDash343,
    /// \u{124eb}: '𒓫'
    CuneiformSignLakDash347,
    /// \u{124ec}: '𒓬'
    CuneiformSignLakDash348,
    /// \u{124ed}: '𒓭'
    CuneiformSignLakDash383,
    /// \u{124ee}: '𒓮'
    CuneiformSignLakDash384,
    /// \u{124ef}: '𒓯'
    CuneiformSignLakDash390,
    /// \u{124f0}: '𒓰'
    CuneiformSignLakDash441,
    /// \u{124f1}: '𒓱'
    CuneiformSignLakDash449,
    /// \u{124f2}: '𒓲'
    CuneiformSignLakDash449TimesGu,
    /// \u{124f3}: '𒓳'
    CuneiformSignLakDash449TimesIgi,
    /// \u{124f4}: '𒓴'
    CuneiformSignLakDash449TimesPapPlusLu3,
    /// \u{124f5}: '𒓵'
    CuneiformSignLakDash449TimesPapPlusPapPlusLu3,
    /// \u{124f6}: '𒓶'
    CuneiformSignLakDash449TimesU2PlusBa,
    /// \u{124f7}: '𒓷'
    CuneiformSignLakDash450,
    /// \u{124f8}: '𒓸'
    CuneiformSignLakDash457,
    /// \u{124f9}: '𒓹'
    CuneiformSignLakDash470,
    /// \u{124fa}: '𒓺'
    CuneiformSignLakDash483,
    /// \u{124fb}: '𒓻'
    CuneiformSignLakDash490,
    /// \u{124fc}: '𒓼'
    CuneiformSignLakDash492,
    /// \u{124fd}: '𒓽'
    CuneiformSignLakDash493,
    /// \u{124fe}: '𒓾'
    CuneiformSignLakDash495,
    /// \u{124ff}: '𒓿'
    CuneiformSignLakDash550,
    /// \u{12500}: '𒔀'
    CuneiformSignLakDash608,
    /// \u{12501}: '𒔁'
    CuneiformSignLakDash617,
    /// \u{12502}: '𒔂'
    CuneiformSignLakDash617TimesAsh,
    /// \u{12503}: '𒔃'
    CuneiformSignLakDash617TimesBad,
    /// \u{12504}: '𒔄'
    CuneiformSignLakDash617TimesDun3GunuGunu,
    /// \u{12505}: '𒔅'
    CuneiformSignLakDash617TimesKu3,
    /// \u{12506}: '𒔆'
    CuneiformSignLakDash617TimesLa,
    /// \u{12507}: '𒔇'
    CuneiformSignLakDash617TimesTar,
    /// \u{12508}: '𒔈'
    CuneiformSignLakDash617TimesTe,
    /// \u{12509}: '𒔉'
    CuneiformSignLakDash617TimesU2,
    /// \u{1250a}: '𒔊'
    CuneiformSignLakDash617TimesUd,
    /// \u{1250b}: '𒔋'
    CuneiformSignLakDash617TimesUruda,
    /// \u{1250c}: '𒔌'
    CuneiformSignLakDash636,
    /// \u{1250d}: '𒔍'
    CuneiformSignLakDash648,
    /// \u{1250e}: '𒔎'
    CuneiformSignLakDash648TimesDub,
    /// \u{1250f}: '𒔏'
    CuneiformSignLakDash648TimesGa,
    /// \u{12510}: '𒔐'
    CuneiformSignLakDash648TimesIgi,
    /// \u{12511}: '𒔑'
    CuneiformSignLakDash648TimesIgiGunu,
    /// \u{12512}: '𒔒'
    CuneiformSignLakDash648TimesNi,
    /// \u{12513}: '𒔓'
    CuneiformSignLakDash648TimesPapPlusPapPlusLu3,
    /// \u{12514}: '𒔔'
    CuneiformSignLakDash648TimesSheshPlusKi,
    /// \u{12515}: '𒔕'
    CuneiformSignLakDash648TimesUd,
    /// \u{12516}: '𒔖'
    CuneiformSignLakDash648TimesUruda,
    /// \u{12517}: '𒔗'
    CuneiformSignLakDash724,
    /// \u{12518}: '𒔘'
    CuneiformSignLakDash749,
    /// \u{12519}: '𒔙'
    CuneiformSignLu2GunuTimesAsh,
    /// \u{1251a}: '𒔚'
    CuneiformSignLu2TimesDish,
    /// \u{1251b}: '𒔛'
    CuneiformSignLu2TimesHal,
    /// \u{1251c}: '𒔜'
    CuneiformSignLu2TimesPap,
    /// \u{1251d}: '𒔝'
    CuneiformSignLu2TimesPapPlusPapPlusLu3,
    /// \u{1251e}: '𒔞'
    CuneiformSignLu2TimesTak4,
    /// \u{1251f}: '𒔟'
    CuneiformSignMiPlusZa7,
    /// \u{12520}: '𒔠'
    CuneiformSignMushOverMushTimesGa,
    /// \u{12521}: '𒔡'
    CuneiformSignMushOverMushTimesKak,
    /// \u{12522}: '𒔢'
    CuneiformSignNinda2TimesDimGunu,
    /// \u{12523}: '𒔣'
    CuneiformSignNinda2TimesGish,
    /// \u{12524}: '𒔤'
    CuneiformSignNinda2TimesGul,
    /// \u{12525}: '𒔥'
    CuneiformSignNinda2TimesHi,
    /// \u{12526}: '𒔦'
    CuneiformSignNinda2TimesKesh2,
    /// \u{12527}: '𒔧'
    CuneiformSignNinda2TimesLakDash050,
    /// \u{12528}: '𒔨'
    CuneiformSignNinda2TimesMash,
    /// \u{12529}: '𒔩'
    CuneiformSignNinda2TimesPapPlusPap,
    /// \u{1252a}: '𒔪'
    CuneiformSignNinda2TimesU,
    /// \u{1252b}: '𒔫'
    CuneiformSignNinda2TimesUPlusU,
    /// \u{1252c}: '𒔬'
    CuneiformSignNinda2TimesUruda,
    /// \u{1252d}: '𒔭'
    CuneiformSignSagGunuTimesHa,
    /// \u{1252e}: '𒔮'
    CuneiformSignSagTimesEn,
    /// \u{1252f}: '𒔯'
    CuneiformSignSagTimesSheAtLeft,
    /// \u{12530}: '𒔰'
    CuneiformSignSagTimesTak4,
    /// \u{12531}: '𒔱'
    CuneiformSignSha6Tenu,
    /// \u{12532}: '𒔲'
    CuneiformSignSheOverShe,
    /// \u{12533}: '𒔳'
    CuneiformSignShePlusHub2,
    /// \u{12534}: '𒔴'
    CuneiformSignShePlusNam2,
    /// \u{12535}: '𒔵'
    CuneiformSignShePlusSar,
    /// \u{12536}: '𒔶'
    CuneiformSignShu2PlusDugTimesNi,
    /// \u{12537}: '𒔷'
    CuneiformSignShu2PlusE2TimesAn,
    /// \u{12538}: '𒔸'
    CuneiformSignSiTimesTak4,
    /// \u{12539}: '𒔹'
    CuneiformSignTak4PlusSag,
    /// \u{1253a}: '𒔺'
    CuneiformSignTumTimesGan2Tenu,
    /// \u{1253b}: '𒔻'
    CuneiformSignTumTimesThreeDish,
    /// \u{1253c}: '𒔼'
    CuneiformSignUr2Inverted,
    /// \u{1253d}: '𒔽'
    CuneiformSignUr2TimesUd,
    /// \u{1253e}: '𒔾'
    CuneiformSignUruTimesDara3,
    /// \u{1253f}: '𒔿'
    CuneiformSignUruTimesLakDash668,
    /// \u{12540}: '𒕀'
    CuneiformSignUruTimesLu3,
    /// \u{12541}: '𒕁'
    CuneiformSignZa7,
    /// \u{12542}: '𒕂'
    CuneiformSignZuOverZuPlusSar,
    /// \u{12543}: '𒕃'
    CuneiformSignZu5TimesThreeDishTenu,
}

impl Into<char> for EarlyDynasticCuneiform {
    fn into(self) -> char {
        match self {
            EarlyDynasticCuneiform::CuneiformSignAbTimesNunTenu => '𒒀',
            EarlyDynasticCuneiform::CuneiformSignAbTimesShu2 => '𒒁',
            EarlyDynasticCuneiform::CuneiformSignAdTimesEsh2 => '𒒂',
            EarlyDynasticCuneiform::CuneiformSignBadTimesDishTenu => '𒒃',
            EarlyDynasticCuneiform::CuneiformSignBahar2TimesAb2 => '𒒄',
            EarlyDynasticCuneiform::CuneiformSignBahar2TimesNi => '𒒅',
            EarlyDynasticCuneiform::CuneiformSignBahar2TimesZa => '𒒆',
            EarlyDynasticCuneiform::CuneiformSignBuOverBuTimesNa2 => '𒒇',
            EarlyDynasticCuneiform::CuneiformSignDaTimesTak4 => '𒒈',
            EarlyDynasticCuneiform::CuneiformSignDagTimesKur => '𒒉',
            EarlyDynasticCuneiform::CuneiformSignDimTimesIgi => '𒒊',
            EarlyDynasticCuneiform::CuneiformSignDimTimesUUU => '𒒋',
            EarlyDynasticCuneiform::CuneiformSignDim2TimesUd => '𒒌',
            EarlyDynasticCuneiform::CuneiformSignDugTimesAnshe => '𒒍',
            EarlyDynasticCuneiform::CuneiformSignDugTimesAsh => '𒒎',
            EarlyDynasticCuneiform::CuneiformSignDugTimesAshAtLeft => '𒒏',
            EarlyDynasticCuneiform::CuneiformSignDugTimesDin => '𒒐',
            EarlyDynasticCuneiform::CuneiformSignDugTimesDun => '𒒑',
            EarlyDynasticCuneiform::CuneiformSignDugTimesErin2 => '𒒒',
            EarlyDynasticCuneiform::CuneiformSignDugTimesGa => '𒒓',
            EarlyDynasticCuneiform::CuneiformSignDugTimesGi => '𒒔',
            EarlyDynasticCuneiform::CuneiformSignDugTimesGir2Gunu => '𒒕',
            EarlyDynasticCuneiform::CuneiformSignDugTimesGish => '𒒖',
            EarlyDynasticCuneiform::CuneiformSignDugTimesHa => '𒒗',
            EarlyDynasticCuneiform::CuneiformSignDugTimesHi => '𒒘',
            EarlyDynasticCuneiform::CuneiformSignDugTimesIgiGunu => '𒒙',
            EarlyDynasticCuneiform::CuneiformSignDugTimesKaskal => '𒒚',
            EarlyDynasticCuneiform::CuneiformSignDugTimesKur => '𒒛',
            EarlyDynasticCuneiform::CuneiformSignDugTimesKushu2 => '𒒜',
            EarlyDynasticCuneiform::CuneiformSignDugTimesKushu2PlusKaskal => '𒒝',
            EarlyDynasticCuneiform::CuneiformSignDugTimesLakDash020 => '𒒞',
            EarlyDynasticCuneiform::CuneiformSignDugTimesLam => '𒒟',
            EarlyDynasticCuneiform::CuneiformSignDugTimesLamTimesKur => '𒒠',
            EarlyDynasticCuneiform::CuneiformSignDugTimesLuhPlusGish => '𒒡',
            EarlyDynasticCuneiform::CuneiformSignDugTimesMash => '𒒢',
            EarlyDynasticCuneiform::CuneiformSignDugTimesMes => '𒒣',
            EarlyDynasticCuneiform::CuneiformSignDugTimesMi => '𒒤',
            EarlyDynasticCuneiform::CuneiformSignDugTimesNi => '𒒥',
            EarlyDynasticCuneiform::CuneiformSignDugTimesPi => '𒒦',
            EarlyDynasticCuneiform::CuneiformSignDugTimesShe => '𒒧',
            EarlyDynasticCuneiform::CuneiformSignDugTimesSiGunu => '𒒨',
            EarlyDynasticCuneiform::CuneiformSignE2TimesKur => '𒒩',
            EarlyDynasticCuneiform::CuneiformSignE2TimesPap => '𒒪',
            EarlyDynasticCuneiform::CuneiformSignErin2X => '𒒫',
            EarlyDynasticCuneiform::CuneiformSignEsh2CrossingEsh2 => '𒒬',
            EarlyDynasticCuneiform::CuneiformSignEzenSheshigTimesAsh => '𒒭',
            EarlyDynasticCuneiform::CuneiformSignEzenSheshigTimesHi => '𒒮',
            EarlyDynasticCuneiform::CuneiformSignEzenSheshigTimesIgiGunu => '𒒯',
            EarlyDynasticCuneiform::CuneiformSignEzenSheshigTimesLa => '𒒰',
            EarlyDynasticCuneiform::CuneiformSignEzenSheshigTimesLal => '𒒱',
            EarlyDynasticCuneiform::CuneiformSignEzenSheshigTimesMe => '𒒲',
            EarlyDynasticCuneiform::CuneiformSignEzenSheshigTimesMes => '𒒳',
            EarlyDynasticCuneiform::CuneiformSignEzenSheshigTimesSu => '𒒴',
            EarlyDynasticCuneiform::CuneiformSignEzenTimesSu => '𒒵',
            EarlyDynasticCuneiform::CuneiformSignGa2TimesBahar2 => '𒒶',
            EarlyDynasticCuneiform::CuneiformSignGa2TimesDimGunu => '𒒷',
            EarlyDynasticCuneiform::CuneiformSignGa2TimesDugTimesIgiGunu => '𒒸',
            EarlyDynasticCuneiform::CuneiformSignGa2TimesDugTimesKaskal => '𒒹',
            EarlyDynasticCuneiform::CuneiformSignGa2TimesEren => '𒒺',
            EarlyDynasticCuneiform::CuneiformSignGa2TimesGa => '𒒻',
            EarlyDynasticCuneiform::CuneiformSignGa2TimesGarPlusDi => '𒒼',
            EarlyDynasticCuneiform::CuneiformSignGa2TimesGarPlusNe => '𒒽',
            EarlyDynasticCuneiform::CuneiformSignGa2TimesHaPlusA => '𒒾',
            EarlyDynasticCuneiform::CuneiformSignGa2TimesKushu2PlusKaskal => '𒒿',
            EarlyDynasticCuneiform::CuneiformSignGa2TimesLam => '𒓀',
            EarlyDynasticCuneiform::CuneiformSignGa2TimesLamTimesKur => '𒓁',
            EarlyDynasticCuneiform::CuneiformSignGa2TimesLuh => '𒓂',
            EarlyDynasticCuneiform::CuneiformSignGa2TimesMush => '𒓃',
            EarlyDynasticCuneiform::CuneiformSignGa2TimesNe => '𒓄',
            EarlyDynasticCuneiform::CuneiformSignGa2TimesNePlusE2 => '𒓅',
            EarlyDynasticCuneiform::CuneiformSignGa2TimesNePlusGi => '𒓆',
            EarlyDynasticCuneiform::CuneiformSignGa2TimesShim => '𒓇',
            EarlyDynasticCuneiform::CuneiformSignGa2TimesZiz2 => '𒓈',
            EarlyDynasticCuneiform::CuneiformSignGabaRotatedNinetyDegrees => '𒓉',
            EarlyDynasticCuneiform::CuneiformSignGeshtinTimesU => '𒓊',
            EarlyDynasticCuneiform::CuneiformSignGishTimesGishCrossingGish => '𒓋',
            EarlyDynasticCuneiform::CuneiformSignGu2TimesIgiGunu => '𒓌',
            EarlyDynasticCuneiform::CuneiformSignGudPlusGishTimesTak4 => '𒓍',
            EarlyDynasticCuneiform::CuneiformSignHaTenuGunu => '𒓎',
            EarlyDynasticCuneiform::CuneiformSignHiTimesAshOverHiTimesAsh => '𒓏',
            EarlyDynasticCuneiform::CuneiformSignKaTimesBu => '𒓐',
            EarlyDynasticCuneiform::CuneiformSignKaTimesKa => '𒓑',
            EarlyDynasticCuneiform::CuneiformSignKaTimesUUU => '𒓒',
            EarlyDynasticCuneiform::CuneiformSignKaTimesUr => '𒓓',
            EarlyDynasticCuneiform::CuneiformSignLagabTimesZuOverZu => '𒓔',
            EarlyDynasticCuneiform::CuneiformSignLakDash003 => '𒓕',
            EarlyDynasticCuneiform::CuneiformSignLakDash021 => '𒓖',
            EarlyDynasticCuneiform::CuneiformSignLakDash025 => '𒓗',
            EarlyDynasticCuneiform::CuneiformSignLakDash030 => '𒓘',
            EarlyDynasticCuneiform::CuneiformSignLakDash050 => '𒓙',
            EarlyDynasticCuneiform::CuneiformSignLakDash051 => '𒓚',
            EarlyDynasticCuneiform::CuneiformSignLakDash062 => '𒓛',
            EarlyDynasticCuneiform::CuneiformSignLakDash079OverLakDash079Gunu => '𒓜',
            EarlyDynasticCuneiform::CuneiformSignLakDash080 => '𒓝',
            EarlyDynasticCuneiform::CuneiformSignLakDash081OverLakDash081 => '𒓞',
            EarlyDynasticCuneiform::CuneiformSignLakDash092 => '𒓟',
            EarlyDynasticCuneiform::CuneiformSignLakDash130 => '𒓠',
            EarlyDynasticCuneiform::CuneiformSignLakDash142 => '𒓡',
            EarlyDynasticCuneiform::CuneiformSignLakDash210 => '𒓢',
            EarlyDynasticCuneiform::CuneiformSignLakDash219 => '𒓣',
            EarlyDynasticCuneiform::CuneiformSignLakDash220 => '𒓤',
            EarlyDynasticCuneiform::CuneiformSignLakDash225 => '𒓥',
            EarlyDynasticCuneiform::CuneiformSignLakDash228 => '𒓦',
            EarlyDynasticCuneiform::CuneiformSignLakDash238 => '𒓧',
            EarlyDynasticCuneiform::CuneiformSignLakDash265 => '𒓨',
            EarlyDynasticCuneiform::CuneiformSignLakDash266 => '𒓩',
            EarlyDynasticCuneiform::CuneiformSignLakDash343 => '𒓪',
            EarlyDynasticCuneiform::CuneiformSignLakDash347 => '𒓫',
            EarlyDynasticCuneiform::CuneiformSignLakDash348 => '𒓬',
            EarlyDynasticCuneiform::CuneiformSignLakDash383 => '𒓭',
            EarlyDynasticCuneiform::CuneiformSignLakDash384 => '𒓮',
            EarlyDynasticCuneiform::CuneiformSignLakDash390 => '𒓯',
            EarlyDynasticCuneiform::CuneiformSignLakDash441 => '𒓰',
            EarlyDynasticCuneiform::CuneiformSignLakDash449 => '𒓱',
            EarlyDynasticCuneiform::CuneiformSignLakDash449TimesGu => '𒓲',
            EarlyDynasticCuneiform::CuneiformSignLakDash449TimesIgi => '𒓳',
            EarlyDynasticCuneiform::CuneiformSignLakDash449TimesPapPlusLu3 => '𒓴',
            EarlyDynasticCuneiform::CuneiformSignLakDash449TimesPapPlusPapPlusLu3 => '𒓵',
            EarlyDynasticCuneiform::CuneiformSignLakDash449TimesU2PlusBa => '𒓶',
            EarlyDynasticCuneiform::CuneiformSignLakDash450 => '𒓷',
            EarlyDynasticCuneiform::CuneiformSignLakDash457 => '𒓸',
            EarlyDynasticCuneiform::CuneiformSignLakDash470 => '𒓹',
            EarlyDynasticCuneiform::CuneiformSignLakDash483 => '𒓺',
            EarlyDynasticCuneiform::CuneiformSignLakDash490 => '𒓻',
            EarlyDynasticCuneiform::CuneiformSignLakDash492 => '𒓼',
            EarlyDynasticCuneiform::CuneiformSignLakDash493 => '𒓽',
            EarlyDynasticCuneiform::CuneiformSignLakDash495 => '𒓾',
            EarlyDynasticCuneiform::CuneiformSignLakDash550 => '𒓿',
            EarlyDynasticCuneiform::CuneiformSignLakDash608 => '𒔀',
            EarlyDynasticCuneiform::CuneiformSignLakDash617 => '𒔁',
            EarlyDynasticCuneiform::CuneiformSignLakDash617TimesAsh => '𒔂',
            EarlyDynasticCuneiform::CuneiformSignLakDash617TimesBad => '𒔃',
            EarlyDynasticCuneiform::CuneiformSignLakDash617TimesDun3GunuGunu => '𒔄',
            EarlyDynasticCuneiform::CuneiformSignLakDash617TimesKu3 => '𒔅',
            EarlyDynasticCuneiform::CuneiformSignLakDash617TimesLa => '𒔆',
            EarlyDynasticCuneiform::CuneiformSignLakDash617TimesTar => '𒔇',
            EarlyDynasticCuneiform::CuneiformSignLakDash617TimesTe => '𒔈',
            EarlyDynasticCuneiform::CuneiformSignLakDash617TimesU2 => '𒔉',
            EarlyDynasticCuneiform::CuneiformSignLakDash617TimesUd => '𒔊',
            EarlyDynasticCuneiform::CuneiformSignLakDash617TimesUruda => '𒔋',
            EarlyDynasticCuneiform::CuneiformSignLakDash636 => '𒔌',
            EarlyDynasticCuneiform::CuneiformSignLakDash648 => '𒔍',
            EarlyDynasticCuneiform::CuneiformSignLakDash648TimesDub => '𒔎',
            EarlyDynasticCuneiform::CuneiformSignLakDash648TimesGa => '𒔏',
            EarlyDynasticCuneiform::CuneiformSignLakDash648TimesIgi => '𒔐',
            EarlyDynasticCuneiform::CuneiformSignLakDash648TimesIgiGunu => '𒔑',
            EarlyDynasticCuneiform::CuneiformSignLakDash648TimesNi => '𒔒',
            EarlyDynasticCuneiform::CuneiformSignLakDash648TimesPapPlusPapPlusLu3 => '𒔓',
            EarlyDynasticCuneiform::CuneiformSignLakDash648TimesSheshPlusKi => '𒔔',
            EarlyDynasticCuneiform::CuneiformSignLakDash648TimesUd => '𒔕',
            EarlyDynasticCuneiform::CuneiformSignLakDash648TimesUruda => '𒔖',
            EarlyDynasticCuneiform::CuneiformSignLakDash724 => '𒔗',
            EarlyDynasticCuneiform::CuneiformSignLakDash749 => '𒔘',
            EarlyDynasticCuneiform::CuneiformSignLu2GunuTimesAsh => '𒔙',
            EarlyDynasticCuneiform::CuneiformSignLu2TimesDish => '𒔚',
            EarlyDynasticCuneiform::CuneiformSignLu2TimesHal => '𒔛',
            EarlyDynasticCuneiform::CuneiformSignLu2TimesPap => '𒔜',
            EarlyDynasticCuneiform::CuneiformSignLu2TimesPapPlusPapPlusLu3 => '𒔝',
            EarlyDynasticCuneiform::CuneiformSignLu2TimesTak4 => '𒔞',
            EarlyDynasticCuneiform::CuneiformSignMiPlusZa7 => '𒔟',
            EarlyDynasticCuneiform::CuneiformSignMushOverMushTimesGa => '𒔠',
            EarlyDynasticCuneiform::CuneiformSignMushOverMushTimesKak => '𒔡',
            EarlyDynasticCuneiform::CuneiformSignNinda2TimesDimGunu => '𒔢',
            EarlyDynasticCuneiform::CuneiformSignNinda2TimesGish => '𒔣',
            EarlyDynasticCuneiform::CuneiformSignNinda2TimesGul => '𒔤',
            EarlyDynasticCuneiform::CuneiformSignNinda2TimesHi => '𒔥',
            EarlyDynasticCuneiform::CuneiformSignNinda2TimesKesh2 => '𒔦',
            EarlyDynasticCuneiform::CuneiformSignNinda2TimesLakDash050 => '𒔧',
            EarlyDynasticCuneiform::CuneiformSignNinda2TimesMash => '𒔨',
            EarlyDynasticCuneiform::CuneiformSignNinda2TimesPapPlusPap => '𒔩',
            EarlyDynasticCuneiform::CuneiformSignNinda2TimesU => '𒔪',
            EarlyDynasticCuneiform::CuneiformSignNinda2TimesUPlusU => '𒔫',
            EarlyDynasticCuneiform::CuneiformSignNinda2TimesUruda => '𒔬',
            EarlyDynasticCuneiform::CuneiformSignSagGunuTimesHa => '𒔭',
            EarlyDynasticCuneiform::CuneiformSignSagTimesEn => '𒔮',
            EarlyDynasticCuneiform::CuneiformSignSagTimesSheAtLeft => '𒔯',
            EarlyDynasticCuneiform::CuneiformSignSagTimesTak4 => '𒔰',
            EarlyDynasticCuneiform::CuneiformSignSha6Tenu => '𒔱',
            EarlyDynasticCuneiform::CuneiformSignSheOverShe => '𒔲',
            EarlyDynasticCuneiform::CuneiformSignShePlusHub2 => '𒔳',
            EarlyDynasticCuneiform::CuneiformSignShePlusNam2 => '𒔴',
            EarlyDynasticCuneiform::CuneiformSignShePlusSar => '𒔵',
            EarlyDynasticCuneiform::CuneiformSignShu2PlusDugTimesNi => '𒔶',
            EarlyDynasticCuneiform::CuneiformSignShu2PlusE2TimesAn => '𒔷',
            EarlyDynasticCuneiform::CuneiformSignSiTimesTak4 => '𒔸',
            EarlyDynasticCuneiform::CuneiformSignTak4PlusSag => '𒔹',
            EarlyDynasticCuneiform::CuneiformSignTumTimesGan2Tenu => '𒔺',
            EarlyDynasticCuneiform::CuneiformSignTumTimesThreeDish => '𒔻',
            EarlyDynasticCuneiform::CuneiformSignUr2Inverted => '𒔼',
            EarlyDynasticCuneiform::CuneiformSignUr2TimesUd => '𒔽',
            EarlyDynasticCuneiform::CuneiformSignUruTimesDara3 => '𒔾',
            EarlyDynasticCuneiform::CuneiformSignUruTimesLakDash668 => '𒔿',
            EarlyDynasticCuneiform::CuneiformSignUruTimesLu3 => '𒕀',
            EarlyDynasticCuneiform::CuneiformSignZa7 => '𒕁',
            EarlyDynasticCuneiform::CuneiformSignZuOverZuPlusSar => '𒕂',
            EarlyDynasticCuneiform::CuneiformSignZu5TimesThreeDishTenu => '𒕃',
        }
    }
}

impl std::convert::TryFrom<char> for EarlyDynasticCuneiform {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𒒀' => Ok(EarlyDynasticCuneiform::CuneiformSignAbTimesNunTenu),
            '𒒁' => Ok(EarlyDynasticCuneiform::CuneiformSignAbTimesShu2),
            '𒒂' => Ok(EarlyDynasticCuneiform::CuneiformSignAdTimesEsh2),
            '𒒃' => Ok(EarlyDynasticCuneiform::CuneiformSignBadTimesDishTenu),
            '𒒄' => Ok(EarlyDynasticCuneiform::CuneiformSignBahar2TimesAb2),
            '𒒅' => Ok(EarlyDynasticCuneiform::CuneiformSignBahar2TimesNi),
            '𒒆' => Ok(EarlyDynasticCuneiform::CuneiformSignBahar2TimesZa),
            '𒒇' => Ok(EarlyDynasticCuneiform::CuneiformSignBuOverBuTimesNa2),
            '𒒈' => Ok(EarlyDynasticCuneiform::CuneiformSignDaTimesTak4),
            '𒒉' => Ok(EarlyDynasticCuneiform::CuneiformSignDagTimesKur),
            '𒒊' => Ok(EarlyDynasticCuneiform::CuneiformSignDimTimesIgi),
            '𒒋' => Ok(EarlyDynasticCuneiform::CuneiformSignDimTimesUUU),
            '𒒌' => Ok(EarlyDynasticCuneiform::CuneiformSignDim2TimesUd),
            '𒒍' => Ok(EarlyDynasticCuneiform::CuneiformSignDugTimesAnshe),
            '𒒎' => Ok(EarlyDynasticCuneiform::CuneiformSignDugTimesAsh),
            '𒒏' => Ok(EarlyDynasticCuneiform::CuneiformSignDugTimesAshAtLeft),
            '𒒐' => Ok(EarlyDynasticCuneiform::CuneiformSignDugTimesDin),
            '𒒑' => Ok(EarlyDynasticCuneiform::CuneiformSignDugTimesDun),
            '𒒒' => Ok(EarlyDynasticCuneiform::CuneiformSignDugTimesErin2),
            '𒒓' => Ok(EarlyDynasticCuneiform::CuneiformSignDugTimesGa),
            '𒒔' => Ok(EarlyDynasticCuneiform::CuneiformSignDugTimesGi),
            '𒒕' => Ok(EarlyDynasticCuneiform::CuneiformSignDugTimesGir2Gunu),
            '𒒖' => Ok(EarlyDynasticCuneiform::CuneiformSignDugTimesGish),
            '𒒗' => Ok(EarlyDynasticCuneiform::CuneiformSignDugTimesHa),
            '𒒘' => Ok(EarlyDynasticCuneiform::CuneiformSignDugTimesHi),
            '𒒙' => Ok(EarlyDynasticCuneiform::CuneiformSignDugTimesIgiGunu),
            '𒒚' => Ok(EarlyDynasticCuneiform::CuneiformSignDugTimesKaskal),
            '𒒛' => Ok(EarlyDynasticCuneiform::CuneiformSignDugTimesKur),
            '𒒜' => Ok(EarlyDynasticCuneiform::CuneiformSignDugTimesKushu2),
            '𒒝' => Ok(EarlyDynasticCuneiform::CuneiformSignDugTimesKushu2PlusKaskal),
            '𒒞' => Ok(EarlyDynasticCuneiform::CuneiformSignDugTimesLakDash020),
            '𒒟' => Ok(EarlyDynasticCuneiform::CuneiformSignDugTimesLam),
            '𒒠' => Ok(EarlyDynasticCuneiform::CuneiformSignDugTimesLamTimesKur),
            '𒒡' => Ok(EarlyDynasticCuneiform::CuneiformSignDugTimesLuhPlusGish),
            '𒒢' => Ok(EarlyDynasticCuneiform::CuneiformSignDugTimesMash),
            '𒒣' => Ok(EarlyDynasticCuneiform::CuneiformSignDugTimesMes),
            '𒒤' => Ok(EarlyDynasticCuneiform::CuneiformSignDugTimesMi),
            '𒒥' => Ok(EarlyDynasticCuneiform::CuneiformSignDugTimesNi),
            '𒒦' => Ok(EarlyDynasticCuneiform::CuneiformSignDugTimesPi),
            '𒒧' => Ok(EarlyDynasticCuneiform::CuneiformSignDugTimesShe),
            '𒒨' => Ok(EarlyDynasticCuneiform::CuneiformSignDugTimesSiGunu),
            '𒒩' => Ok(EarlyDynasticCuneiform::CuneiformSignE2TimesKur),
            '𒒪' => Ok(EarlyDynasticCuneiform::CuneiformSignE2TimesPap),
            '𒒫' => Ok(EarlyDynasticCuneiform::CuneiformSignErin2X),
            '𒒬' => Ok(EarlyDynasticCuneiform::CuneiformSignEsh2CrossingEsh2),
            '𒒭' => Ok(EarlyDynasticCuneiform::CuneiformSignEzenSheshigTimesAsh),
            '𒒮' => Ok(EarlyDynasticCuneiform::CuneiformSignEzenSheshigTimesHi),
            '𒒯' => Ok(EarlyDynasticCuneiform::CuneiformSignEzenSheshigTimesIgiGunu),
            '𒒰' => Ok(EarlyDynasticCuneiform::CuneiformSignEzenSheshigTimesLa),
            '𒒱' => Ok(EarlyDynasticCuneiform::CuneiformSignEzenSheshigTimesLal),
            '𒒲' => Ok(EarlyDynasticCuneiform::CuneiformSignEzenSheshigTimesMe),
            '𒒳' => Ok(EarlyDynasticCuneiform::CuneiformSignEzenSheshigTimesMes),
            '𒒴' => Ok(EarlyDynasticCuneiform::CuneiformSignEzenSheshigTimesSu),
            '𒒵' => Ok(EarlyDynasticCuneiform::CuneiformSignEzenTimesSu),
            '𒒶' => Ok(EarlyDynasticCuneiform::CuneiformSignGa2TimesBahar2),
            '𒒷' => Ok(EarlyDynasticCuneiform::CuneiformSignGa2TimesDimGunu),
            '𒒸' => Ok(EarlyDynasticCuneiform::CuneiformSignGa2TimesDugTimesIgiGunu),
            '𒒹' => Ok(EarlyDynasticCuneiform::CuneiformSignGa2TimesDugTimesKaskal),
            '𒒺' => Ok(EarlyDynasticCuneiform::CuneiformSignGa2TimesEren),
            '𒒻' => Ok(EarlyDynasticCuneiform::CuneiformSignGa2TimesGa),
            '𒒼' => Ok(EarlyDynasticCuneiform::CuneiformSignGa2TimesGarPlusDi),
            '𒒽' => Ok(EarlyDynasticCuneiform::CuneiformSignGa2TimesGarPlusNe),
            '𒒾' => Ok(EarlyDynasticCuneiform::CuneiformSignGa2TimesHaPlusA),
            '𒒿' => Ok(EarlyDynasticCuneiform::CuneiformSignGa2TimesKushu2PlusKaskal),
            '𒓀' => Ok(EarlyDynasticCuneiform::CuneiformSignGa2TimesLam),
            '𒓁' => Ok(EarlyDynasticCuneiform::CuneiformSignGa2TimesLamTimesKur),
            '𒓂' => Ok(EarlyDynasticCuneiform::CuneiformSignGa2TimesLuh),
            '𒓃' => Ok(EarlyDynasticCuneiform::CuneiformSignGa2TimesMush),
            '𒓄' => Ok(EarlyDynasticCuneiform::CuneiformSignGa2TimesNe),
            '𒓅' => Ok(EarlyDynasticCuneiform::CuneiformSignGa2TimesNePlusE2),
            '𒓆' => Ok(EarlyDynasticCuneiform::CuneiformSignGa2TimesNePlusGi),
            '𒓇' => Ok(EarlyDynasticCuneiform::CuneiformSignGa2TimesShim),
            '𒓈' => Ok(EarlyDynasticCuneiform::CuneiformSignGa2TimesZiz2),
            '𒓉' => Ok(EarlyDynasticCuneiform::CuneiformSignGabaRotatedNinetyDegrees),
            '𒓊' => Ok(EarlyDynasticCuneiform::CuneiformSignGeshtinTimesU),
            '𒓋' => Ok(EarlyDynasticCuneiform::CuneiformSignGishTimesGishCrossingGish),
            '𒓌' => Ok(EarlyDynasticCuneiform::CuneiformSignGu2TimesIgiGunu),
            '𒓍' => Ok(EarlyDynasticCuneiform::CuneiformSignGudPlusGishTimesTak4),
            '𒓎' => Ok(EarlyDynasticCuneiform::CuneiformSignHaTenuGunu),
            '𒓏' => Ok(EarlyDynasticCuneiform::CuneiformSignHiTimesAshOverHiTimesAsh),
            '𒓐' => Ok(EarlyDynasticCuneiform::CuneiformSignKaTimesBu),
            '𒓑' => Ok(EarlyDynasticCuneiform::CuneiformSignKaTimesKa),
            '𒓒' => Ok(EarlyDynasticCuneiform::CuneiformSignKaTimesUUU),
            '𒓓' => Ok(EarlyDynasticCuneiform::CuneiformSignKaTimesUr),
            '𒓔' => Ok(EarlyDynasticCuneiform::CuneiformSignLagabTimesZuOverZu),
            '𒓕' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash003),
            '𒓖' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash021),
            '𒓗' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash025),
            '𒓘' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash030),
            '𒓙' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash050),
            '𒓚' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash051),
            '𒓛' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash062),
            '𒓜' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash079OverLakDash079Gunu),
            '𒓝' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash080),
            '𒓞' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash081OverLakDash081),
            '𒓟' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash092),
            '𒓠' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash130),
            '𒓡' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash142),
            '𒓢' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash210),
            '𒓣' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash219),
            '𒓤' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash220),
            '𒓥' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash225),
            '𒓦' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash228),
            '𒓧' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash238),
            '𒓨' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash265),
            '𒓩' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash266),
            '𒓪' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash343),
            '𒓫' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash347),
            '𒓬' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash348),
            '𒓭' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash383),
            '𒓮' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash384),
            '𒓯' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash390),
            '𒓰' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash441),
            '𒓱' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash449),
            '𒓲' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash449TimesGu),
            '𒓳' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash449TimesIgi),
            '𒓴' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash449TimesPapPlusLu3),
            '𒓵' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash449TimesPapPlusPapPlusLu3),
            '𒓶' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash449TimesU2PlusBa),
            '𒓷' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash450),
            '𒓸' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash457),
            '𒓹' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash470),
            '𒓺' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash483),
            '𒓻' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash490),
            '𒓼' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash492),
            '𒓽' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash493),
            '𒓾' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash495),
            '𒓿' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash550),
            '𒔀' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash608),
            '𒔁' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash617),
            '𒔂' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash617TimesAsh),
            '𒔃' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash617TimesBad),
            '𒔄' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash617TimesDun3GunuGunu),
            '𒔅' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash617TimesKu3),
            '𒔆' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash617TimesLa),
            '𒔇' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash617TimesTar),
            '𒔈' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash617TimesTe),
            '𒔉' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash617TimesU2),
            '𒔊' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash617TimesUd),
            '𒔋' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash617TimesUruda),
            '𒔌' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash636),
            '𒔍' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash648),
            '𒔎' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash648TimesDub),
            '𒔏' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash648TimesGa),
            '𒔐' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash648TimesIgi),
            '𒔑' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash648TimesIgiGunu),
            '𒔒' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash648TimesNi),
            '𒔓' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash648TimesPapPlusPapPlusLu3),
            '𒔔' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash648TimesSheshPlusKi),
            '𒔕' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash648TimesUd),
            '𒔖' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash648TimesUruda),
            '𒔗' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash724),
            '𒔘' => Ok(EarlyDynasticCuneiform::CuneiformSignLakDash749),
            '𒔙' => Ok(EarlyDynasticCuneiform::CuneiformSignLu2GunuTimesAsh),
            '𒔚' => Ok(EarlyDynasticCuneiform::CuneiformSignLu2TimesDish),
            '𒔛' => Ok(EarlyDynasticCuneiform::CuneiformSignLu2TimesHal),
            '𒔜' => Ok(EarlyDynasticCuneiform::CuneiformSignLu2TimesPap),
            '𒔝' => Ok(EarlyDynasticCuneiform::CuneiformSignLu2TimesPapPlusPapPlusLu3),
            '𒔞' => Ok(EarlyDynasticCuneiform::CuneiformSignLu2TimesTak4),
            '𒔟' => Ok(EarlyDynasticCuneiform::CuneiformSignMiPlusZa7),
            '𒔠' => Ok(EarlyDynasticCuneiform::CuneiformSignMushOverMushTimesGa),
            '𒔡' => Ok(EarlyDynasticCuneiform::CuneiformSignMushOverMushTimesKak),
            '𒔢' => Ok(EarlyDynasticCuneiform::CuneiformSignNinda2TimesDimGunu),
            '𒔣' => Ok(EarlyDynasticCuneiform::CuneiformSignNinda2TimesGish),
            '𒔤' => Ok(EarlyDynasticCuneiform::CuneiformSignNinda2TimesGul),
            '𒔥' => Ok(EarlyDynasticCuneiform::CuneiformSignNinda2TimesHi),
            '𒔦' => Ok(EarlyDynasticCuneiform::CuneiformSignNinda2TimesKesh2),
            '𒔧' => Ok(EarlyDynasticCuneiform::CuneiformSignNinda2TimesLakDash050),
            '𒔨' => Ok(EarlyDynasticCuneiform::CuneiformSignNinda2TimesMash),
            '𒔩' => Ok(EarlyDynasticCuneiform::CuneiformSignNinda2TimesPapPlusPap),
            '𒔪' => Ok(EarlyDynasticCuneiform::CuneiformSignNinda2TimesU),
            '𒔫' => Ok(EarlyDynasticCuneiform::CuneiformSignNinda2TimesUPlusU),
            '𒔬' => Ok(EarlyDynasticCuneiform::CuneiformSignNinda2TimesUruda),
            '𒔭' => Ok(EarlyDynasticCuneiform::CuneiformSignSagGunuTimesHa),
            '𒔮' => Ok(EarlyDynasticCuneiform::CuneiformSignSagTimesEn),
            '𒔯' => Ok(EarlyDynasticCuneiform::CuneiformSignSagTimesSheAtLeft),
            '𒔰' => Ok(EarlyDynasticCuneiform::CuneiformSignSagTimesTak4),
            '𒔱' => Ok(EarlyDynasticCuneiform::CuneiformSignSha6Tenu),
            '𒔲' => Ok(EarlyDynasticCuneiform::CuneiformSignSheOverShe),
            '𒔳' => Ok(EarlyDynasticCuneiform::CuneiformSignShePlusHub2),
            '𒔴' => Ok(EarlyDynasticCuneiform::CuneiformSignShePlusNam2),
            '𒔵' => Ok(EarlyDynasticCuneiform::CuneiformSignShePlusSar),
            '𒔶' => Ok(EarlyDynasticCuneiform::CuneiformSignShu2PlusDugTimesNi),
            '𒔷' => Ok(EarlyDynasticCuneiform::CuneiformSignShu2PlusE2TimesAn),
            '𒔸' => Ok(EarlyDynasticCuneiform::CuneiformSignSiTimesTak4),
            '𒔹' => Ok(EarlyDynasticCuneiform::CuneiformSignTak4PlusSag),
            '𒔺' => Ok(EarlyDynasticCuneiform::CuneiformSignTumTimesGan2Tenu),
            '𒔻' => Ok(EarlyDynasticCuneiform::CuneiformSignTumTimesThreeDish),
            '𒔼' => Ok(EarlyDynasticCuneiform::CuneiformSignUr2Inverted),
            '𒔽' => Ok(EarlyDynasticCuneiform::CuneiformSignUr2TimesUd),
            '𒔾' => Ok(EarlyDynasticCuneiform::CuneiformSignUruTimesDara3),
            '𒔿' => Ok(EarlyDynasticCuneiform::CuneiformSignUruTimesLakDash668),
            '𒕀' => Ok(EarlyDynasticCuneiform::CuneiformSignUruTimesLu3),
            '𒕁' => Ok(EarlyDynasticCuneiform::CuneiformSignZa7),
            '𒕂' => Ok(EarlyDynasticCuneiform::CuneiformSignZuOverZuPlusSar),
            '𒕃' => Ok(EarlyDynasticCuneiform::CuneiformSignZu5TimesThreeDishTenu),
            _ => Err(()),
        }
    }
}

impl Into<u32> for EarlyDynasticCuneiform {
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

impl std::convert::TryFrom<u32> for EarlyDynasticCuneiform {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for EarlyDynasticCuneiform {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl EarlyDynasticCuneiform {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        EarlyDynasticCuneiform::CuneiformSignAbTimesNunTenu
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            EarlyDynasticCuneiform::CuneiformSignAbTimesNunTenu => "cuneiform sign ab times nun tenu",
            EarlyDynasticCuneiform::CuneiformSignAbTimesShu2 => "cuneiform sign ab times shu2",
            EarlyDynasticCuneiform::CuneiformSignAdTimesEsh2 => "cuneiform sign ad times esh2",
            EarlyDynasticCuneiform::CuneiformSignBadTimesDishTenu => "cuneiform sign bad times dish tenu",
            EarlyDynasticCuneiform::CuneiformSignBahar2TimesAb2 => "cuneiform sign bahar2 times ab2",
            EarlyDynasticCuneiform::CuneiformSignBahar2TimesNi => "cuneiform sign bahar2 times ni",
            EarlyDynasticCuneiform::CuneiformSignBahar2TimesZa => "cuneiform sign bahar2 times za",
            EarlyDynasticCuneiform::CuneiformSignBuOverBuTimesNa2 => "cuneiform sign bu over bu times na2",
            EarlyDynasticCuneiform::CuneiformSignDaTimesTak4 => "cuneiform sign da times tak4",
            EarlyDynasticCuneiform::CuneiformSignDagTimesKur => "cuneiform sign dag times kur",
            EarlyDynasticCuneiform::CuneiformSignDimTimesIgi => "cuneiform sign dim times igi",
            EarlyDynasticCuneiform::CuneiformSignDimTimesUUU => "cuneiform sign dim times u u u",
            EarlyDynasticCuneiform::CuneiformSignDim2TimesUd => "cuneiform sign dim2 times ud",
            EarlyDynasticCuneiform::CuneiformSignDugTimesAnshe => "cuneiform sign dug times anshe",
            EarlyDynasticCuneiform::CuneiformSignDugTimesAsh => "cuneiform sign dug times ash",
            EarlyDynasticCuneiform::CuneiformSignDugTimesAshAtLeft => "cuneiform sign dug times ash at left",
            EarlyDynasticCuneiform::CuneiformSignDugTimesDin => "cuneiform sign dug times din",
            EarlyDynasticCuneiform::CuneiformSignDugTimesDun => "cuneiform sign dug times dun",
            EarlyDynasticCuneiform::CuneiformSignDugTimesErin2 => "cuneiform sign dug times erin2",
            EarlyDynasticCuneiform::CuneiformSignDugTimesGa => "cuneiform sign dug times ga",
            EarlyDynasticCuneiform::CuneiformSignDugTimesGi => "cuneiform sign dug times gi",
            EarlyDynasticCuneiform::CuneiformSignDugTimesGir2Gunu => "cuneiform sign dug times gir2 gunu",
            EarlyDynasticCuneiform::CuneiformSignDugTimesGish => "cuneiform sign dug times gish",
            EarlyDynasticCuneiform::CuneiformSignDugTimesHa => "cuneiform sign dug times ha",
            EarlyDynasticCuneiform::CuneiformSignDugTimesHi => "cuneiform sign dug times hi",
            EarlyDynasticCuneiform::CuneiformSignDugTimesIgiGunu => "cuneiform sign dug times igi gunu",
            EarlyDynasticCuneiform::CuneiformSignDugTimesKaskal => "cuneiform sign dug times kaskal",
            EarlyDynasticCuneiform::CuneiformSignDugTimesKur => "cuneiform sign dug times kur",
            EarlyDynasticCuneiform::CuneiformSignDugTimesKushu2 => "cuneiform sign dug times kushu2",
            EarlyDynasticCuneiform::CuneiformSignDugTimesKushu2PlusKaskal => "cuneiform sign dug times kushu2 plus kaskal",
            EarlyDynasticCuneiform::CuneiformSignDugTimesLakDash020 => "cuneiform sign dug times lak-020",
            EarlyDynasticCuneiform::CuneiformSignDugTimesLam => "cuneiform sign dug times lam",
            EarlyDynasticCuneiform::CuneiformSignDugTimesLamTimesKur => "cuneiform sign dug times lam times kur",
            EarlyDynasticCuneiform::CuneiformSignDugTimesLuhPlusGish => "cuneiform sign dug times luh plus gish",
            EarlyDynasticCuneiform::CuneiformSignDugTimesMash => "cuneiform sign dug times mash",
            EarlyDynasticCuneiform::CuneiformSignDugTimesMes => "cuneiform sign dug times mes",
            EarlyDynasticCuneiform::CuneiformSignDugTimesMi => "cuneiform sign dug times mi",
            EarlyDynasticCuneiform::CuneiformSignDugTimesNi => "cuneiform sign dug times ni",
            EarlyDynasticCuneiform::CuneiformSignDugTimesPi => "cuneiform sign dug times pi",
            EarlyDynasticCuneiform::CuneiformSignDugTimesShe => "cuneiform sign dug times she",
            EarlyDynasticCuneiform::CuneiformSignDugTimesSiGunu => "cuneiform sign dug times si gunu",
            EarlyDynasticCuneiform::CuneiformSignE2TimesKur => "cuneiform sign e2 times kur",
            EarlyDynasticCuneiform::CuneiformSignE2TimesPap => "cuneiform sign e2 times pap",
            EarlyDynasticCuneiform::CuneiformSignErin2X => "cuneiform sign erin2 x",
            EarlyDynasticCuneiform::CuneiformSignEsh2CrossingEsh2 => "cuneiform sign esh2 crossing esh2",
            EarlyDynasticCuneiform::CuneiformSignEzenSheshigTimesAsh => "cuneiform sign ezen sheshig times ash",
            EarlyDynasticCuneiform::CuneiformSignEzenSheshigTimesHi => "cuneiform sign ezen sheshig times hi",
            EarlyDynasticCuneiform::CuneiformSignEzenSheshigTimesIgiGunu => "cuneiform sign ezen sheshig times igi gunu",
            EarlyDynasticCuneiform::CuneiformSignEzenSheshigTimesLa => "cuneiform sign ezen sheshig times la",
            EarlyDynasticCuneiform::CuneiformSignEzenSheshigTimesLal => "cuneiform sign ezen sheshig times lal",
            EarlyDynasticCuneiform::CuneiformSignEzenSheshigTimesMe => "cuneiform sign ezen sheshig times me",
            EarlyDynasticCuneiform::CuneiformSignEzenSheshigTimesMes => "cuneiform sign ezen sheshig times mes",
            EarlyDynasticCuneiform::CuneiformSignEzenSheshigTimesSu => "cuneiform sign ezen sheshig times su",
            EarlyDynasticCuneiform::CuneiformSignEzenTimesSu => "cuneiform sign ezen times su",
            EarlyDynasticCuneiform::CuneiformSignGa2TimesBahar2 => "cuneiform sign ga2 times bahar2",
            EarlyDynasticCuneiform::CuneiformSignGa2TimesDimGunu => "cuneiform sign ga2 times dim gunu",
            EarlyDynasticCuneiform::CuneiformSignGa2TimesDugTimesIgiGunu => "cuneiform sign ga2 times dug times igi gunu",
            EarlyDynasticCuneiform::CuneiformSignGa2TimesDugTimesKaskal => "cuneiform sign ga2 times dug times kaskal",
            EarlyDynasticCuneiform::CuneiformSignGa2TimesEren => "cuneiform sign ga2 times eren",
            EarlyDynasticCuneiform::CuneiformSignGa2TimesGa => "cuneiform sign ga2 times ga",
            EarlyDynasticCuneiform::CuneiformSignGa2TimesGarPlusDi => "cuneiform sign ga2 times gar plus di",
            EarlyDynasticCuneiform::CuneiformSignGa2TimesGarPlusNe => "cuneiform sign ga2 times gar plus ne",
            EarlyDynasticCuneiform::CuneiformSignGa2TimesHaPlusA => "cuneiform sign ga2 times ha plus a",
            EarlyDynasticCuneiform::CuneiformSignGa2TimesKushu2PlusKaskal => "cuneiform sign ga2 times kushu2 plus kaskal",
            EarlyDynasticCuneiform::CuneiformSignGa2TimesLam => "cuneiform sign ga2 times lam",
            EarlyDynasticCuneiform::CuneiformSignGa2TimesLamTimesKur => "cuneiform sign ga2 times lam times kur",
            EarlyDynasticCuneiform::CuneiformSignGa2TimesLuh => "cuneiform sign ga2 times luh",
            EarlyDynasticCuneiform::CuneiformSignGa2TimesMush => "cuneiform sign ga2 times mush",
            EarlyDynasticCuneiform::CuneiformSignGa2TimesNe => "cuneiform sign ga2 times ne",
            EarlyDynasticCuneiform::CuneiformSignGa2TimesNePlusE2 => "cuneiform sign ga2 times ne plus e2",
            EarlyDynasticCuneiform::CuneiformSignGa2TimesNePlusGi => "cuneiform sign ga2 times ne plus gi",
            EarlyDynasticCuneiform::CuneiformSignGa2TimesShim => "cuneiform sign ga2 times shim",
            EarlyDynasticCuneiform::CuneiformSignGa2TimesZiz2 => "cuneiform sign ga2 times ziz2",
            EarlyDynasticCuneiform::CuneiformSignGabaRotatedNinetyDegrees => "cuneiform sign gaba rotated ninety degrees",
            EarlyDynasticCuneiform::CuneiformSignGeshtinTimesU => "cuneiform sign geshtin times u",
            EarlyDynasticCuneiform::CuneiformSignGishTimesGishCrossingGish => "cuneiform sign gish times gish crossing gish",
            EarlyDynasticCuneiform::CuneiformSignGu2TimesIgiGunu => "cuneiform sign gu2 times igi gunu",
            EarlyDynasticCuneiform::CuneiformSignGudPlusGishTimesTak4 => "cuneiform sign gud plus gish times tak4",
            EarlyDynasticCuneiform::CuneiformSignHaTenuGunu => "cuneiform sign ha tenu gunu",
            EarlyDynasticCuneiform::CuneiformSignHiTimesAshOverHiTimesAsh => "cuneiform sign hi times ash over hi times ash",
            EarlyDynasticCuneiform::CuneiformSignKaTimesBu => "cuneiform sign ka times bu",
            EarlyDynasticCuneiform::CuneiformSignKaTimesKa => "cuneiform sign ka times ka",
            EarlyDynasticCuneiform::CuneiformSignKaTimesUUU => "cuneiform sign ka times u u u",
            EarlyDynasticCuneiform::CuneiformSignKaTimesUr => "cuneiform sign ka times ur",
            EarlyDynasticCuneiform::CuneiformSignLagabTimesZuOverZu => "cuneiform sign lagab times zu over zu",
            EarlyDynasticCuneiform::CuneiformSignLakDash003 => "cuneiform sign lak-003",
            EarlyDynasticCuneiform::CuneiformSignLakDash021 => "cuneiform sign lak-021",
            EarlyDynasticCuneiform::CuneiformSignLakDash025 => "cuneiform sign lak-025",
            EarlyDynasticCuneiform::CuneiformSignLakDash030 => "cuneiform sign lak-030",
            EarlyDynasticCuneiform::CuneiformSignLakDash050 => "cuneiform sign lak-050",
            EarlyDynasticCuneiform::CuneiformSignLakDash051 => "cuneiform sign lak-051",
            EarlyDynasticCuneiform::CuneiformSignLakDash062 => "cuneiform sign lak-062",
            EarlyDynasticCuneiform::CuneiformSignLakDash079OverLakDash079Gunu => "cuneiform sign lak-079 over lak-079 gunu",
            EarlyDynasticCuneiform::CuneiformSignLakDash080 => "cuneiform sign lak-080",
            EarlyDynasticCuneiform::CuneiformSignLakDash081OverLakDash081 => "cuneiform sign lak-081 over lak-081",
            EarlyDynasticCuneiform::CuneiformSignLakDash092 => "cuneiform sign lak-092",
            EarlyDynasticCuneiform::CuneiformSignLakDash130 => "cuneiform sign lak-130",
            EarlyDynasticCuneiform::CuneiformSignLakDash142 => "cuneiform sign lak-142",
            EarlyDynasticCuneiform::CuneiformSignLakDash210 => "cuneiform sign lak-210",
            EarlyDynasticCuneiform::CuneiformSignLakDash219 => "cuneiform sign lak-219",
            EarlyDynasticCuneiform::CuneiformSignLakDash220 => "cuneiform sign lak-220",
            EarlyDynasticCuneiform::CuneiformSignLakDash225 => "cuneiform sign lak-225",
            EarlyDynasticCuneiform::CuneiformSignLakDash228 => "cuneiform sign lak-228",
            EarlyDynasticCuneiform::CuneiformSignLakDash238 => "cuneiform sign lak-238",
            EarlyDynasticCuneiform::CuneiformSignLakDash265 => "cuneiform sign lak-265",
            EarlyDynasticCuneiform::CuneiformSignLakDash266 => "cuneiform sign lak-266",
            EarlyDynasticCuneiform::CuneiformSignLakDash343 => "cuneiform sign lak-343",
            EarlyDynasticCuneiform::CuneiformSignLakDash347 => "cuneiform sign lak-347",
            EarlyDynasticCuneiform::CuneiformSignLakDash348 => "cuneiform sign lak-348",
            EarlyDynasticCuneiform::CuneiformSignLakDash383 => "cuneiform sign lak-383",
            EarlyDynasticCuneiform::CuneiformSignLakDash384 => "cuneiform sign lak-384",
            EarlyDynasticCuneiform::CuneiformSignLakDash390 => "cuneiform sign lak-390",
            EarlyDynasticCuneiform::CuneiformSignLakDash441 => "cuneiform sign lak-441",
            EarlyDynasticCuneiform::CuneiformSignLakDash449 => "cuneiform sign lak-449",
            EarlyDynasticCuneiform::CuneiformSignLakDash449TimesGu => "cuneiform sign lak-449 times gu",
            EarlyDynasticCuneiform::CuneiformSignLakDash449TimesIgi => "cuneiform sign lak-449 times igi",
            EarlyDynasticCuneiform::CuneiformSignLakDash449TimesPapPlusLu3 => "cuneiform sign lak-449 times pap plus lu3",
            EarlyDynasticCuneiform::CuneiformSignLakDash449TimesPapPlusPapPlusLu3 => "cuneiform sign lak-449 times pap plus pap plus lu3",
            EarlyDynasticCuneiform::CuneiformSignLakDash449TimesU2PlusBa => "cuneiform sign lak-449 times u2 plus ba",
            EarlyDynasticCuneiform::CuneiformSignLakDash450 => "cuneiform sign lak-450",
            EarlyDynasticCuneiform::CuneiformSignLakDash457 => "cuneiform sign lak-457",
            EarlyDynasticCuneiform::CuneiformSignLakDash470 => "cuneiform sign lak-470",
            EarlyDynasticCuneiform::CuneiformSignLakDash483 => "cuneiform sign lak-483",
            EarlyDynasticCuneiform::CuneiformSignLakDash490 => "cuneiform sign lak-490",
            EarlyDynasticCuneiform::CuneiformSignLakDash492 => "cuneiform sign lak-492",
            EarlyDynasticCuneiform::CuneiformSignLakDash493 => "cuneiform sign lak-493",
            EarlyDynasticCuneiform::CuneiformSignLakDash495 => "cuneiform sign lak-495",
            EarlyDynasticCuneiform::CuneiformSignLakDash550 => "cuneiform sign lak-550",
            EarlyDynasticCuneiform::CuneiformSignLakDash608 => "cuneiform sign lak-608",
            EarlyDynasticCuneiform::CuneiformSignLakDash617 => "cuneiform sign lak-617",
            EarlyDynasticCuneiform::CuneiformSignLakDash617TimesAsh => "cuneiform sign lak-617 times ash",
            EarlyDynasticCuneiform::CuneiformSignLakDash617TimesBad => "cuneiform sign lak-617 times bad",
            EarlyDynasticCuneiform::CuneiformSignLakDash617TimesDun3GunuGunu => "cuneiform sign lak-617 times dun3 gunu gunu",
            EarlyDynasticCuneiform::CuneiformSignLakDash617TimesKu3 => "cuneiform sign lak-617 times ku3",
            EarlyDynasticCuneiform::CuneiformSignLakDash617TimesLa => "cuneiform sign lak-617 times la",
            EarlyDynasticCuneiform::CuneiformSignLakDash617TimesTar => "cuneiform sign lak-617 times tar",
            EarlyDynasticCuneiform::CuneiformSignLakDash617TimesTe => "cuneiform sign lak-617 times te",
            EarlyDynasticCuneiform::CuneiformSignLakDash617TimesU2 => "cuneiform sign lak-617 times u2",
            EarlyDynasticCuneiform::CuneiformSignLakDash617TimesUd => "cuneiform sign lak-617 times ud",
            EarlyDynasticCuneiform::CuneiformSignLakDash617TimesUruda => "cuneiform sign lak-617 times uruda",
            EarlyDynasticCuneiform::CuneiformSignLakDash636 => "cuneiform sign lak-636",
            EarlyDynasticCuneiform::CuneiformSignLakDash648 => "cuneiform sign lak-648",
            EarlyDynasticCuneiform::CuneiformSignLakDash648TimesDub => "cuneiform sign lak-648 times dub",
            EarlyDynasticCuneiform::CuneiformSignLakDash648TimesGa => "cuneiform sign lak-648 times ga",
            EarlyDynasticCuneiform::CuneiformSignLakDash648TimesIgi => "cuneiform sign lak-648 times igi",
            EarlyDynasticCuneiform::CuneiformSignLakDash648TimesIgiGunu => "cuneiform sign lak-648 times igi gunu",
            EarlyDynasticCuneiform::CuneiformSignLakDash648TimesNi => "cuneiform sign lak-648 times ni",
            EarlyDynasticCuneiform::CuneiformSignLakDash648TimesPapPlusPapPlusLu3 => "cuneiform sign lak-648 times pap plus pap plus lu3",
            EarlyDynasticCuneiform::CuneiformSignLakDash648TimesSheshPlusKi => "cuneiform sign lak-648 times shesh plus ki",
            EarlyDynasticCuneiform::CuneiformSignLakDash648TimesUd => "cuneiform sign lak-648 times ud",
            EarlyDynasticCuneiform::CuneiformSignLakDash648TimesUruda => "cuneiform sign lak-648 times uruda",
            EarlyDynasticCuneiform::CuneiformSignLakDash724 => "cuneiform sign lak-724",
            EarlyDynasticCuneiform::CuneiformSignLakDash749 => "cuneiform sign lak-749",
            EarlyDynasticCuneiform::CuneiformSignLu2GunuTimesAsh => "cuneiform sign lu2 gunu times ash",
            EarlyDynasticCuneiform::CuneiformSignLu2TimesDish => "cuneiform sign lu2 times dish",
            EarlyDynasticCuneiform::CuneiformSignLu2TimesHal => "cuneiform sign lu2 times hal",
            EarlyDynasticCuneiform::CuneiformSignLu2TimesPap => "cuneiform sign lu2 times pap",
            EarlyDynasticCuneiform::CuneiformSignLu2TimesPapPlusPapPlusLu3 => "cuneiform sign lu2 times pap plus pap plus lu3",
            EarlyDynasticCuneiform::CuneiformSignLu2TimesTak4 => "cuneiform sign lu2 times tak4",
            EarlyDynasticCuneiform::CuneiformSignMiPlusZa7 => "cuneiform sign mi plus za7",
            EarlyDynasticCuneiform::CuneiformSignMushOverMushTimesGa => "cuneiform sign mush over mush times ga",
            EarlyDynasticCuneiform::CuneiformSignMushOverMushTimesKak => "cuneiform sign mush over mush times kak",
            EarlyDynasticCuneiform::CuneiformSignNinda2TimesDimGunu => "cuneiform sign ninda2 times dim gunu",
            EarlyDynasticCuneiform::CuneiformSignNinda2TimesGish => "cuneiform sign ninda2 times gish",
            EarlyDynasticCuneiform::CuneiformSignNinda2TimesGul => "cuneiform sign ninda2 times gul",
            EarlyDynasticCuneiform::CuneiformSignNinda2TimesHi => "cuneiform sign ninda2 times hi",
            EarlyDynasticCuneiform::CuneiformSignNinda2TimesKesh2 => "cuneiform sign ninda2 times kesh2",
            EarlyDynasticCuneiform::CuneiformSignNinda2TimesLakDash050 => "cuneiform sign ninda2 times lak-050",
            EarlyDynasticCuneiform::CuneiformSignNinda2TimesMash => "cuneiform sign ninda2 times mash",
            EarlyDynasticCuneiform::CuneiformSignNinda2TimesPapPlusPap => "cuneiform sign ninda2 times pap plus pap",
            EarlyDynasticCuneiform::CuneiformSignNinda2TimesU => "cuneiform sign ninda2 times u",
            EarlyDynasticCuneiform::CuneiformSignNinda2TimesUPlusU => "cuneiform sign ninda2 times u plus u",
            EarlyDynasticCuneiform::CuneiformSignNinda2TimesUruda => "cuneiform sign ninda2 times uruda",
            EarlyDynasticCuneiform::CuneiformSignSagGunuTimesHa => "cuneiform sign sag gunu times ha",
            EarlyDynasticCuneiform::CuneiformSignSagTimesEn => "cuneiform sign sag times en",
            EarlyDynasticCuneiform::CuneiformSignSagTimesSheAtLeft => "cuneiform sign sag times she at left",
            EarlyDynasticCuneiform::CuneiformSignSagTimesTak4 => "cuneiform sign sag times tak4",
            EarlyDynasticCuneiform::CuneiformSignSha6Tenu => "cuneiform sign sha6 tenu",
            EarlyDynasticCuneiform::CuneiformSignSheOverShe => "cuneiform sign she over she",
            EarlyDynasticCuneiform::CuneiformSignShePlusHub2 => "cuneiform sign she plus hub2",
            EarlyDynasticCuneiform::CuneiformSignShePlusNam2 => "cuneiform sign she plus nam2",
            EarlyDynasticCuneiform::CuneiformSignShePlusSar => "cuneiform sign she plus sar",
            EarlyDynasticCuneiform::CuneiformSignShu2PlusDugTimesNi => "cuneiform sign shu2 plus dug times ni",
            EarlyDynasticCuneiform::CuneiformSignShu2PlusE2TimesAn => "cuneiform sign shu2 plus e2 times an",
            EarlyDynasticCuneiform::CuneiformSignSiTimesTak4 => "cuneiform sign si times tak4",
            EarlyDynasticCuneiform::CuneiformSignTak4PlusSag => "cuneiform sign tak4 plus sag",
            EarlyDynasticCuneiform::CuneiformSignTumTimesGan2Tenu => "cuneiform sign tum times gan2 tenu",
            EarlyDynasticCuneiform::CuneiformSignTumTimesThreeDish => "cuneiform sign tum times three dish",
            EarlyDynasticCuneiform::CuneiformSignUr2Inverted => "cuneiform sign ur2 inverted",
            EarlyDynasticCuneiform::CuneiformSignUr2TimesUd => "cuneiform sign ur2 times ud",
            EarlyDynasticCuneiform::CuneiformSignUruTimesDara3 => "cuneiform sign uru times dara3",
            EarlyDynasticCuneiform::CuneiformSignUruTimesLakDash668 => "cuneiform sign uru times lak-668",
            EarlyDynasticCuneiform::CuneiformSignUruTimesLu3 => "cuneiform sign uru times lu3",
            EarlyDynasticCuneiform::CuneiformSignZa7 => "cuneiform sign za7",
            EarlyDynasticCuneiform::CuneiformSignZuOverZuPlusSar => "cuneiform sign zu over zu plus sar",
            EarlyDynasticCuneiform::CuneiformSignZu5TimesThreeDishTenu => "cuneiform sign zu5 times three dish tenu",
        }
    }
}
