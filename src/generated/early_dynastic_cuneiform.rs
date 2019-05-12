
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

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("EarlyDynasticCuneiform{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
