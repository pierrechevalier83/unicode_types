
/// An enum to represent all characters in the CuneiformNumbersandPunctuation block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum CuneiformNumbersandPunctuation {
    /// \u{12400}: '𒐀'
    CuneiformNumericSignTwoAsh,
    /// \u{12401}: '𒐁'
    CuneiformNumericSignThreeAsh,
    /// \u{12402}: '𒐂'
    CuneiformNumericSignFourAsh,
    /// \u{12403}: '𒐃'
    CuneiformNumericSignFiveAsh,
    /// \u{12404}: '𒐄'
    CuneiformNumericSignSixAsh,
    /// \u{12405}: '𒐅'
    CuneiformNumericSignSevenAsh,
    /// \u{12406}: '𒐆'
    CuneiformNumericSignEightAsh,
    /// \u{12407}: '𒐇'
    CuneiformNumericSignNineAsh,
    /// \u{12408}: '𒐈'
    CuneiformNumericSignThreeDish,
    /// \u{12409}: '𒐉'
    CuneiformNumericSignFourDish,
    /// \u{1240a}: '𒐊'
    CuneiformNumericSignFiveDish,
    /// \u{1240b}: '𒐋'
    CuneiformNumericSignSixDish,
    /// \u{1240c}: '𒐌'
    CuneiformNumericSignSevenDish,
    /// \u{1240d}: '𒐍'
    CuneiformNumericSignEightDish,
    /// \u{1240e}: '𒐎'
    CuneiformNumericSignNineDish,
    /// \u{1240f}: '𒐏'
    CuneiformNumericSignFourU,
    /// \u{12410}: '𒐐'
    CuneiformNumericSignFiveU,
    /// \u{12411}: '𒐑'
    CuneiformNumericSignSixU,
    /// \u{12412}: '𒐒'
    CuneiformNumericSignSevenU,
    /// \u{12413}: '𒐓'
    CuneiformNumericSignEightU,
    /// \u{12414}: '𒐔'
    CuneiformNumericSignNineU,
    /// \u{12415}: '𒐕'
    CuneiformNumericSignOneGesh2,
    /// \u{12416}: '𒐖'
    CuneiformNumericSignTwoGesh2,
    /// \u{12417}: '𒐗'
    CuneiformNumericSignThreeGesh2,
    /// \u{12418}: '𒐘'
    CuneiformNumericSignFourGesh2,
    /// \u{12419}: '𒐙'
    CuneiformNumericSignFiveGesh2,
    /// \u{1241a}: '𒐚'
    CuneiformNumericSignSixGesh2,
    /// \u{1241b}: '𒐛'
    CuneiformNumericSignSevenGesh2,
    /// \u{1241c}: '𒐜'
    CuneiformNumericSignEightGesh2,
    /// \u{1241d}: '𒐝'
    CuneiformNumericSignNineGesh2,
    /// \u{1241e}: '𒐞'
    CuneiformNumericSignOneGeshu,
    /// \u{1241f}: '𒐟'
    CuneiformNumericSignTwoGeshu,
    /// \u{12420}: '𒐠'
    CuneiformNumericSignThreeGeshu,
    /// \u{12421}: '𒐡'
    CuneiformNumericSignFourGeshu,
    /// \u{12422}: '𒐢'
    CuneiformNumericSignFiveGeshu,
    /// \u{12423}: '𒐣'
    CuneiformNumericSignTwoShar2,
    /// \u{12424}: '𒐤'
    CuneiformNumericSignThreeShar2,
    /// \u{12425}: '𒐥'
    CuneiformNumericSignThreeShar2VariantForm,
    /// \u{12426}: '𒐦'
    CuneiformNumericSignFourShar2,
    /// \u{12427}: '𒐧'
    CuneiformNumericSignFiveShar2,
    /// \u{12428}: '𒐨'
    CuneiformNumericSignSixShar2,
    /// \u{12429}: '𒐩'
    CuneiformNumericSignSevenShar2,
    /// \u{1242a}: '𒐪'
    CuneiformNumericSignEightShar2,
    /// \u{1242b}: '𒐫'
    CuneiformNumericSignNineShar2,
    /// \u{1242c}: '𒐬'
    CuneiformNumericSignOneSharu,
    /// \u{1242d}: '𒐭'
    CuneiformNumericSignTwoSharu,
    /// \u{1242e}: '𒐮'
    CuneiformNumericSignThreeSharu,
    /// \u{1242f}: '𒐯'
    CuneiformNumericSignThreeSharuVariantForm,
    /// \u{12430}: '𒐰'
    CuneiformNumericSignFourSharu,
    /// \u{12431}: '𒐱'
    CuneiformNumericSignFiveSharu,
    /// \u{12432}: '𒐲'
    CuneiformNumericSignShar2TimesGalPlusDish,
    /// \u{12433}: '𒐳'
    CuneiformNumericSignShar2TimesGalPlusMin,
    /// \u{12434}: '𒐴'
    CuneiformNumericSignOneBuru,
    /// \u{12435}: '𒐵'
    CuneiformNumericSignTwoBuru,
    /// \u{12436}: '𒐶'
    CuneiformNumericSignThreeBuru,
    /// \u{12437}: '𒐷'
    CuneiformNumericSignThreeBuruVariantForm,
    /// \u{12438}: '𒐸'
    CuneiformNumericSignFourBuru,
    /// \u{12439}: '𒐹'
    CuneiformNumericSignFiveBuru,
    /// \u{1243a}: '𒐺'
    CuneiformNumericSignThreeVariantFormEsh16,
    /// \u{1243b}: '𒐻'
    CuneiformNumericSignThreeVariantFormEsh21,
    /// \u{1243c}: '𒐼'
    CuneiformNumericSignFourVariantFormLimmu,
    /// \u{1243d}: '𒐽'
    CuneiformNumericSignFourVariantFormLimmu4,
    /// \u{1243e}: '𒐾'
    CuneiformNumericSignFourVariantFormLimmuA,
    /// \u{1243f}: '𒐿'
    CuneiformNumericSignFourVariantFormLimmuB,
    /// \u{12440}: '𒑀'
    CuneiformNumericSignSixVariantFormAsh9,
    /// \u{12441}: '𒑁'
    CuneiformNumericSignSevenVariantFormImin3,
    /// \u{12442}: '𒑂'
    CuneiformNumericSignSevenVariantFormIminA,
    /// \u{12443}: '𒑃'
    CuneiformNumericSignSevenVariantFormIminB,
    /// \u{12444}: '𒑄'
    CuneiformNumericSignEightVariantFormUssu,
    /// \u{12445}: '𒑅'
    CuneiformNumericSignEightVariantFormUssu3,
    /// \u{12446}: '𒑆'
    CuneiformNumericSignNineVariantFormIlimmu,
    /// \u{12447}: '𒑇'
    CuneiformNumericSignNineVariantFormIlimmu3,
    /// \u{12448}: '𒑈'
    CuneiformNumericSignNineVariantFormIlimmu4,
    /// \u{12449}: '𒑉'
    CuneiformNumericSignNineVariantFormIlimmuA,
    /// \u{1244a}: '𒑊'
    CuneiformNumericSignTwoAshTenu,
    /// \u{1244b}: '𒑋'
    CuneiformNumericSignThreeAshTenu,
    /// \u{1244c}: '𒑌'
    CuneiformNumericSignFourAshTenu,
    /// \u{1244d}: '𒑍'
    CuneiformNumericSignFiveAshTenu,
    /// \u{1244e}: '𒑎'
    CuneiformNumericSignSixAshTenu,
    /// \u{1244f}: '𒑏'
    CuneiformNumericSignOneBan2,
    /// \u{12450}: '𒑐'
    CuneiformNumericSignTwoBan2,
    /// \u{12451}: '𒑑'
    CuneiformNumericSignThreeBan2,
    /// \u{12452}: '𒑒'
    CuneiformNumericSignFourBan2,
    /// \u{12453}: '𒑓'
    CuneiformNumericSignFourBan2VariantForm,
    /// \u{12454}: '𒑔'
    CuneiformNumericSignFiveBan2,
    /// \u{12455}: '𒑕'
    CuneiformNumericSignFiveBan2VariantForm,
    /// \u{12456}: '𒑖'
    CuneiformNumericSignNigidamin,
    /// \u{12457}: '𒑗'
    CuneiformNumericSignNigidaesh,
    /// \u{12458}: '𒑘'
    CuneiformNumericSignOneEshe3,
    /// \u{12459}: '𒑙'
    CuneiformNumericSignTwoEshe3,
    /// \u{1245a}: '𒑚'
    CuneiformNumericSignOneThirdDish,
    /// \u{1245b}: '𒑛'
    CuneiformNumericSignTwoThirdsDish,
    /// \u{1245c}: '𒑜'
    CuneiformNumericSignFiveSixthsDish,
    /// \u{1245d}: '𒑝'
    CuneiformNumericSignOneThirdVariantFormA,
    /// \u{1245e}: '𒑞'
    CuneiformNumericSignTwoThirdsVariantFormA,
    /// \u{1245f}: '𒑟'
    CuneiformNumericSignOneEighthAsh,
    /// \u{12460}: '𒑠'
    CuneiformNumericSignOneQuarterAsh,
    /// \u{12461}: '𒑡'
    CuneiformNumericSignOldAssyrianOneSixth,
    /// \u{12462}: '𒑢'
    CuneiformNumericSignOldAssyrianOneQuarter,
    /// \u{12463}: '𒑣'
    CuneiformNumericSignOneQuarterGur,
    /// \u{12464}: '𒑤'
    CuneiformNumericSignOneHalfGur,
    /// \u{12465}: '𒑥'
    CuneiformNumericSignElamiteOneThird,
    /// \u{12466}: '𒑦'
    CuneiformNumericSignElamiteTwoThirds,
    /// \u{12467}: '𒑧'
    CuneiformNumericSignElamiteForty,
    /// \u{12468}: '𒑨'
    CuneiformNumericSignElamiteFifty,
    /// \u{12469}: '𒑩'
    CuneiformNumericSignFourUVariantForm,
    /// \u{1246a}: '𒑪'
    CuneiformNumericSignFiveUVariantForm,
    /// \u{1246b}: '𒑫'
    CuneiformNumericSignSixUVariantForm,
    /// \u{1246c}: '𒑬'
    CuneiformNumericSignSevenUVariantForm,
    /// \u{1246d}: '𒑭'
    CuneiformNumericSignEightUVariantForm,
    /// \u{1246e}: '𒑮'
    CuneiformNumericSignNineUVariantForm,
    /// \u{12470}: '𒑰'
    CuneiformPunctuationSignOldAssyrianWordDivider,
    /// \u{12471}: '𒑱'
    CuneiformPunctuationSignVerticalColon,
    /// \u{12472}: '𒑲'
    CuneiformPunctuationSignDiagonalColon,
    /// \u{12473}: '𒑳'
    CuneiformPunctuationSignDiagonalTricolon,
    /// \u{12474}: '𒑴'
    CuneiformPunctuationSignDiagonalQuadcolon,
}

impl Into<char> for CuneiformNumbersandPunctuation {
    fn into(self) -> char {
        match self {
            CuneiformNumbersandPunctuation::CuneiformNumericSignTwoAsh => '𒐀',
            CuneiformNumbersandPunctuation::CuneiformNumericSignThreeAsh => '𒐁',
            CuneiformNumbersandPunctuation::CuneiformNumericSignFourAsh => '𒐂',
            CuneiformNumbersandPunctuation::CuneiformNumericSignFiveAsh => '𒐃',
            CuneiformNumbersandPunctuation::CuneiformNumericSignSixAsh => '𒐄',
            CuneiformNumbersandPunctuation::CuneiformNumericSignSevenAsh => '𒐅',
            CuneiformNumbersandPunctuation::CuneiformNumericSignEightAsh => '𒐆',
            CuneiformNumbersandPunctuation::CuneiformNumericSignNineAsh => '𒐇',
            CuneiformNumbersandPunctuation::CuneiformNumericSignThreeDish => '𒐈',
            CuneiformNumbersandPunctuation::CuneiformNumericSignFourDish => '𒐉',
            CuneiformNumbersandPunctuation::CuneiformNumericSignFiveDish => '𒐊',
            CuneiformNumbersandPunctuation::CuneiformNumericSignSixDish => '𒐋',
            CuneiformNumbersandPunctuation::CuneiformNumericSignSevenDish => '𒐌',
            CuneiformNumbersandPunctuation::CuneiformNumericSignEightDish => '𒐍',
            CuneiformNumbersandPunctuation::CuneiformNumericSignNineDish => '𒐎',
            CuneiformNumbersandPunctuation::CuneiformNumericSignFourU => '𒐏',
            CuneiformNumbersandPunctuation::CuneiformNumericSignFiveU => '𒐐',
            CuneiformNumbersandPunctuation::CuneiformNumericSignSixU => '𒐑',
            CuneiformNumbersandPunctuation::CuneiformNumericSignSevenU => '𒐒',
            CuneiformNumbersandPunctuation::CuneiformNumericSignEightU => '𒐓',
            CuneiformNumbersandPunctuation::CuneiformNumericSignNineU => '𒐔',
            CuneiformNumbersandPunctuation::CuneiformNumericSignOneGesh2 => '𒐕',
            CuneiformNumbersandPunctuation::CuneiformNumericSignTwoGesh2 => '𒐖',
            CuneiformNumbersandPunctuation::CuneiformNumericSignThreeGesh2 => '𒐗',
            CuneiformNumbersandPunctuation::CuneiformNumericSignFourGesh2 => '𒐘',
            CuneiformNumbersandPunctuation::CuneiformNumericSignFiveGesh2 => '𒐙',
            CuneiformNumbersandPunctuation::CuneiformNumericSignSixGesh2 => '𒐚',
            CuneiformNumbersandPunctuation::CuneiformNumericSignSevenGesh2 => '𒐛',
            CuneiformNumbersandPunctuation::CuneiformNumericSignEightGesh2 => '𒐜',
            CuneiformNumbersandPunctuation::CuneiformNumericSignNineGesh2 => '𒐝',
            CuneiformNumbersandPunctuation::CuneiformNumericSignOneGeshu => '𒐞',
            CuneiformNumbersandPunctuation::CuneiformNumericSignTwoGeshu => '𒐟',
            CuneiformNumbersandPunctuation::CuneiformNumericSignThreeGeshu => '𒐠',
            CuneiformNumbersandPunctuation::CuneiformNumericSignFourGeshu => '𒐡',
            CuneiformNumbersandPunctuation::CuneiformNumericSignFiveGeshu => '𒐢',
            CuneiformNumbersandPunctuation::CuneiformNumericSignTwoShar2 => '𒐣',
            CuneiformNumbersandPunctuation::CuneiformNumericSignThreeShar2 => '𒐤',
            CuneiformNumbersandPunctuation::CuneiformNumericSignThreeShar2VariantForm => '𒐥',
            CuneiformNumbersandPunctuation::CuneiformNumericSignFourShar2 => '𒐦',
            CuneiformNumbersandPunctuation::CuneiformNumericSignFiveShar2 => '𒐧',
            CuneiformNumbersandPunctuation::CuneiformNumericSignSixShar2 => '𒐨',
            CuneiformNumbersandPunctuation::CuneiformNumericSignSevenShar2 => '𒐩',
            CuneiformNumbersandPunctuation::CuneiformNumericSignEightShar2 => '𒐪',
            CuneiformNumbersandPunctuation::CuneiformNumericSignNineShar2 => '𒐫',
            CuneiformNumbersandPunctuation::CuneiformNumericSignOneSharu => '𒐬',
            CuneiformNumbersandPunctuation::CuneiformNumericSignTwoSharu => '𒐭',
            CuneiformNumbersandPunctuation::CuneiformNumericSignThreeSharu => '𒐮',
            CuneiformNumbersandPunctuation::CuneiformNumericSignThreeSharuVariantForm => '𒐯',
            CuneiformNumbersandPunctuation::CuneiformNumericSignFourSharu => '𒐰',
            CuneiformNumbersandPunctuation::CuneiformNumericSignFiveSharu => '𒐱',
            CuneiformNumbersandPunctuation::CuneiformNumericSignShar2TimesGalPlusDish => '𒐲',
            CuneiformNumbersandPunctuation::CuneiformNumericSignShar2TimesGalPlusMin => '𒐳',
            CuneiformNumbersandPunctuation::CuneiformNumericSignOneBuru => '𒐴',
            CuneiformNumbersandPunctuation::CuneiformNumericSignTwoBuru => '𒐵',
            CuneiformNumbersandPunctuation::CuneiformNumericSignThreeBuru => '𒐶',
            CuneiformNumbersandPunctuation::CuneiformNumericSignThreeBuruVariantForm => '𒐷',
            CuneiformNumbersandPunctuation::CuneiformNumericSignFourBuru => '𒐸',
            CuneiformNumbersandPunctuation::CuneiformNumericSignFiveBuru => '𒐹',
            CuneiformNumbersandPunctuation::CuneiformNumericSignThreeVariantFormEsh16 => '𒐺',
            CuneiformNumbersandPunctuation::CuneiformNumericSignThreeVariantFormEsh21 => '𒐻',
            CuneiformNumbersandPunctuation::CuneiformNumericSignFourVariantFormLimmu => '𒐼',
            CuneiformNumbersandPunctuation::CuneiformNumericSignFourVariantFormLimmu4 => '𒐽',
            CuneiformNumbersandPunctuation::CuneiformNumericSignFourVariantFormLimmuA => '𒐾',
            CuneiformNumbersandPunctuation::CuneiformNumericSignFourVariantFormLimmuB => '𒐿',
            CuneiformNumbersandPunctuation::CuneiformNumericSignSixVariantFormAsh9 => '𒑀',
            CuneiformNumbersandPunctuation::CuneiformNumericSignSevenVariantFormImin3 => '𒑁',
            CuneiformNumbersandPunctuation::CuneiformNumericSignSevenVariantFormIminA => '𒑂',
            CuneiformNumbersandPunctuation::CuneiformNumericSignSevenVariantFormIminB => '𒑃',
            CuneiformNumbersandPunctuation::CuneiformNumericSignEightVariantFormUssu => '𒑄',
            CuneiformNumbersandPunctuation::CuneiformNumericSignEightVariantFormUssu3 => '𒑅',
            CuneiformNumbersandPunctuation::CuneiformNumericSignNineVariantFormIlimmu => '𒑆',
            CuneiformNumbersandPunctuation::CuneiformNumericSignNineVariantFormIlimmu3 => '𒑇',
            CuneiformNumbersandPunctuation::CuneiformNumericSignNineVariantFormIlimmu4 => '𒑈',
            CuneiformNumbersandPunctuation::CuneiformNumericSignNineVariantFormIlimmuA => '𒑉',
            CuneiformNumbersandPunctuation::CuneiformNumericSignTwoAshTenu => '𒑊',
            CuneiformNumbersandPunctuation::CuneiformNumericSignThreeAshTenu => '𒑋',
            CuneiformNumbersandPunctuation::CuneiformNumericSignFourAshTenu => '𒑌',
            CuneiformNumbersandPunctuation::CuneiformNumericSignFiveAshTenu => '𒑍',
            CuneiformNumbersandPunctuation::CuneiformNumericSignSixAshTenu => '𒑎',
            CuneiformNumbersandPunctuation::CuneiformNumericSignOneBan2 => '𒑏',
            CuneiformNumbersandPunctuation::CuneiformNumericSignTwoBan2 => '𒑐',
            CuneiformNumbersandPunctuation::CuneiformNumericSignThreeBan2 => '𒑑',
            CuneiformNumbersandPunctuation::CuneiformNumericSignFourBan2 => '𒑒',
            CuneiformNumbersandPunctuation::CuneiformNumericSignFourBan2VariantForm => '𒑓',
            CuneiformNumbersandPunctuation::CuneiformNumericSignFiveBan2 => '𒑔',
            CuneiformNumbersandPunctuation::CuneiformNumericSignFiveBan2VariantForm => '𒑕',
            CuneiformNumbersandPunctuation::CuneiformNumericSignNigidamin => '𒑖',
            CuneiformNumbersandPunctuation::CuneiformNumericSignNigidaesh => '𒑗',
            CuneiformNumbersandPunctuation::CuneiformNumericSignOneEshe3 => '𒑘',
            CuneiformNumbersandPunctuation::CuneiformNumericSignTwoEshe3 => '𒑙',
            CuneiformNumbersandPunctuation::CuneiformNumericSignOneThirdDish => '𒑚',
            CuneiformNumbersandPunctuation::CuneiformNumericSignTwoThirdsDish => '𒑛',
            CuneiformNumbersandPunctuation::CuneiformNumericSignFiveSixthsDish => '𒑜',
            CuneiformNumbersandPunctuation::CuneiformNumericSignOneThirdVariantFormA => '𒑝',
            CuneiformNumbersandPunctuation::CuneiformNumericSignTwoThirdsVariantFormA => '𒑞',
            CuneiformNumbersandPunctuation::CuneiformNumericSignOneEighthAsh => '𒑟',
            CuneiformNumbersandPunctuation::CuneiformNumericSignOneQuarterAsh => '𒑠',
            CuneiformNumbersandPunctuation::CuneiformNumericSignOldAssyrianOneSixth => '𒑡',
            CuneiformNumbersandPunctuation::CuneiformNumericSignOldAssyrianOneQuarter => '𒑢',
            CuneiformNumbersandPunctuation::CuneiformNumericSignOneQuarterGur => '𒑣',
            CuneiformNumbersandPunctuation::CuneiformNumericSignOneHalfGur => '𒑤',
            CuneiformNumbersandPunctuation::CuneiformNumericSignElamiteOneThird => '𒑥',
            CuneiformNumbersandPunctuation::CuneiformNumericSignElamiteTwoThirds => '𒑦',
            CuneiformNumbersandPunctuation::CuneiformNumericSignElamiteForty => '𒑧',
            CuneiformNumbersandPunctuation::CuneiformNumericSignElamiteFifty => '𒑨',
            CuneiformNumbersandPunctuation::CuneiformNumericSignFourUVariantForm => '𒑩',
            CuneiformNumbersandPunctuation::CuneiformNumericSignFiveUVariantForm => '𒑪',
            CuneiformNumbersandPunctuation::CuneiformNumericSignSixUVariantForm => '𒑫',
            CuneiformNumbersandPunctuation::CuneiformNumericSignSevenUVariantForm => '𒑬',
            CuneiformNumbersandPunctuation::CuneiformNumericSignEightUVariantForm => '𒑭',
            CuneiformNumbersandPunctuation::CuneiformNumericSignNineUVariantForm => '𒑮',
            CuneiformNumbersandPunctuation::CuneiformPunctuationSignOldAssyrianWordDivider => '𒑰',
            CuneiformNumbersandPunctuation::CuneiformPunctuationSignVerticalColon => '𒑱',
            CuneiformNumbersandPunctuation::CuneiformPunctuationSignDiagonalColon => '𒑲',
            CuneiformNumbersandPunctuation::CuneiformPunctuationSignDiagonalTricolon => '𒑳',
            CuneiformNumbersandPunctuation::CuneiformPunctuationSignDiagonalQuadcolon => '𒑴',
        }
    }
}

impl std::convert::TryFrom<char> for CuneiformNumbersandPunctuation {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𒐀' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignTwoAsh),
            '𒐁' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignThreeAsh),
            '𒐂' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignFourAsh),
            '𒐃' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignFiveAsh),
            '𒐄' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignSixAsh),
            '𒐅' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignSevenAsh),
            '𒐆' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignEightAsh),
            '𒐇' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignNineAsh),
            '𒐈' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignThreeDish),
            '𒐉' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignFourDish),
            '𒐊' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignFiveDish),
            '𒐋' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignSixDish),
            '𒐌' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignSevenDish),
            '𒐍' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignEightDish),
            '𒐎' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignNineDish),
            '𒐏' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignFourU),
            '𒐐' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignFiveU),
            '𒐑' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignSixU),
            '𒐒' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignSevenU),
            '𒐓' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignEightU),
            '𒐔' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignNineU),
            '𒐕' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignOneGesh2),
            '𒐖' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignTwoGesh2),
            '𒐗' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignThreeGesh2),
            '𒐘' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignFourGesh2),
            '𒐙' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignFiveGesh2),
            '𒐚' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignSixGesh2),
            '𒐛' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignSevenGesh2),
            '𒐜' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignEightGesh2),
            '𒐝' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignNineGesh2),
            '𒐞' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignOneGeshu),
            '𒐟' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignTwoGeshu),
            '𒐠' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignThreeGeshu),
            '𒐡' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignFourGeshu),
            '𒐢' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignFiveGeshu),
            '𒐣' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignTwoShar2),
            '𒐤' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignThreeShar2),
            '𒐥' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignThreeShar2VariantForm),
            '𒐦' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignFourShar2),
            '𒐧' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignFiveShar2),
            '𒐨' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignSixShar2),
            '𒐩' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignSevenShar2),
            '𒐪' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignEightShar2),
            '𒐫' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignNineShar2),
            '𒐬' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignOneSharu),
            '𒐭' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignTwoSharu),
            '𒐮' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignThreeSharu),
            '𒐯' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignThreeSharuVariantForm),
            '𒐰' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignFourSharu),
            '𒐱' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignFiveSharu),
            '𒐲' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignShar2TimesGalPlusDish),
            '𒐳' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignShar2TimesGalPlusMin),
            '𒐴' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignOneBuru),
            '𒐵' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignTwoBuru),
            '𒐶' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignThreeBuru),
            '𒐷' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignThreeBuruVariantForm),
            '𒐸' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignFourBuru),
            '𒐹' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignFiveBuru),
            '𒐺' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignThreeVariantFormEsh16),
            '𒐻' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignThreeVariantFormEsh21),
            '𒐼' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignFourVariantFormLimmu),
            '𒐽' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignFourVariantFormLimmu4),
            '𒐾' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignFourVariantFormLimmuA),
            '𒐿' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignFourVariantFormLimmuB),
            '𒑀' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignSixVariantFormAsh9),
            '𒑁' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignSevenVariantFormImin3),
            '𒑂' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignSevenVariantFormIminA),
            '𒑃' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignSevenVariantFormIminB),
            '𒑄' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignEightVariantFormUssu),
            '𒑅' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignEightVariantFormUssu3),
            '𒑆' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignNineVariantFormIlimmu),
            '𒑇' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignNineVariantFormIlimmu3),
            '𒑈' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignNineVariantFormIlimmu4),
            '𒑉' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignNineVariantFormIlimmuA),
            '𒑊' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignTwoAshTenu),
            '𒑋' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignThreeAshTenu),
            '𒑌' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignFourAshTenu),
            '𒑍' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignFiveAshTenu),
            '𒑎' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignSixAshTenu),
            '𒑏' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignOneBan2),
            '𒑐' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignTwoBan2),
            '𒑑' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignThreeBan2),
            '𒑒' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignFourBan2),
            '𒑓' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignFourBan2VariantForm),
            '𒑔' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignFiveBan2),
            '𒑕' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignFiveBan2VariantForm),
            '𒑖' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignNigidamin),
            '𒑗' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignNigidaesh),
            '𒑘' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignOneEshe3),
            '𒑙' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignTwoEshe3),
            '𒑚' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignOneThirdDish),
            '𒑛' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignTwoThirdsDish),
            '𒑜' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignFiveSixthsDish),
            '𒑝' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignOneThirdVariantFormA),
            '𒑞' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignTwoThirdsVariantFormA),
            '𒑟' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignOneEighthAsh),
            '𒑠' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignOneQuarterAsh),
            '𒑡' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignOldAssyrianOneSixth),
            '𒑢' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignOldAssyrianOneQuarter),
            '𒑣' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignOneQuarterGur),
            '𒑤' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignOneHalfGur),
            '𒑥' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignElamiteOneThird),
            '𒑦' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignElamiteTwoThirds),
            '𒑧' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignElamiteForty),
            '𒑨' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignElamiteFifty),
            '𒑩' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignFourUVariantForm),
            '𒑪' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignFiveUVariantForm),
            '𒑫' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignSixUVariantForm),
            '𒑬' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignSevenUVariantForm),
            '𒑭' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignEightUVariantForm),
            '𒑮' => Ok(CuneiformNumbersandPunctuation::CuneiformNumericSignNineUVariantForm),
            '𒑰' => Ok(CuneiformNumbersandPunctuation::CuneiformPunctuationSignOldAssyrianWordDivider),
            '𒑱' => Ok(CuneiformNumbersandPunctuation::CuneiformPunctuationSignVerticalColon),
            '𒑲' => Ok(CuneiformNumbersandPunctuation::CuneiformPunctuationSignDiagonalColon),
            '𒑳' => Ok(CuneiformNumbersandPunctuation::CuneiformPunctuationSignDiagonalTricolon),
            '𒑴' => Ok(CuneiformNumbersandPunctuation::CuneiformPunctuationSignDiagonalQuadcolon),
            _ => Err(()),
        }
    }
}

impl Into<u32> for CuneiformNumbersandPunctuation {
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

impl std::convert::TryFrom<u32> for CuneiformNumbersandPunctuation {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for CuneiformNumbersandPunctuation {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl CuneiformNumbersandPunctuation {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        CuneiformNumbersandPunctuation::CuneiformNumericSignTwoAsh
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("CuneiformNumbersandPunctuation{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
