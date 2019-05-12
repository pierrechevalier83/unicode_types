
/// An enum to represent all characters in the TamilSupplement block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum TamilSupplement {
    /// \u{11fc0}: '𑿀'
    TamilFractionOneThreeDashHundredDashAndDashTwentieth,
    /// \u{11fc1}: '𑿁'
    TamilFractionOneOneDashHundredDashAndDashSixtieth,
    /// \u{11fc2}: '𑿂'
    TamilFractionOneEightieth,
    /// \u{11fc3}: '𑿃'
    TamilFractionOneSixtyDashFourth,
    /// \u{11fc4}: '𑿄'
    TamilFractionOneFortieth,
    /// \u{11fc5}: '𑿅'
    TamilFractionOneThirtyDashSecond,
    /// \u{11fc6}: '𑿆'
    TamilFractionThreeEightieths,
    /// \u{11fc7}: '𑿇'
    TamilFractionThreeSixtyDashFourths,
    /// \u{11fc8}: '𑿈'
    TamilFractionOneTwentieth,
    /// \u{11fc9}: '𑿉'
    TamilFractionOneSixteenthDash1,
    /// \u{11fca}: '𑿊'
    TamilFractionOneSixteenthDash2,
    /// \u{11fcb}: '𑿋'
    TamilFractionOneTenth,
    /// \u{11fcc}: '𑿌'
    TamilFractionOneEighth,
    /// \u{11fcd}: '𑿍'
    TamilFractionThreeTwentieths,
    /// \u{11fce}: '𑿎'
    TamilFractionThreeSixteenths,
    /// \u{11fcf}: '𑿏'
    TamilFractionOneFifth,
    /// \u{11fd0}: '𑿐'
    TamilFractionOneQuarter,
    /// \u{11fd1}: '𑿑'
    TamilFractionOneHalfDash1,
    /// \u{11fd2}: '𑿒'
    TamilFractionOneHalfDash2,
    /// \u{11fd3}: '𑿓'
    TamilFractionThreeQuarters,
    /// \u{11fd4}: '𑿔'
    TamilFractionDownscalingFactorKiizh,
    /// \u{11fd5}: '𑿕'
    TamilSignNel,
    /// \u{11fd6}: '𑿖'
    TamilSignCevitu,
    /// \u{11fd7}: '𑿗'
    TamilSignAazhaakku,
    /// \u{11fd8}: '𑿘'
    TamilSignUzhakku,
    /// \u{11fd9}: '𑿙'
    TamilSignMuuvuzhakku,
    /// \u{11fda}: '𑿚'
    TamilSignKuruni,
    /// \u{11fdb}: '𑿛'
    TamilSignPathakku,
    /// \u{11fdc}: '𑿜'
    TamilSignMukkuruni,
    /// \u{11fdd}: '𑿝'
    TamilSignKaacu,
    /// \u{11fde}: '𑿞'
    TamilSignPanam,
    /// \u{11fdf}: '𑿟'
    TamilSignPon,
    /// \u{11fe0}: '𑿠'
    TamilSignVaraakan,
    /// \u{11fe1}: '𑿡'
    TamilSignPaaram,
    /// \u{11fe2}: '𑿢'
    TamilSignKuzhi,
    /// \u{11fe3}: '𑿣'
    TamilSignVeli,
    /// \u{11fe4}: '𑿤'
    TamilWetCultivationSign,
    /// \u{11fe5}: '𑿥'
    TamilDryCultivationSign,
    /// \u{11fe6}: '𑿦'
    TamilLandSign,
    /// \u{11fe7}: '𑿧'
    TamilSaltPanSign,
    /// \u{11fe8}: '𑿨'
    TamilTraditionalCreditSign,
    /// \u{11fe9}: '𑿩'
    TamilTraditionalNumberSign,
    /// \u{11fea}: '𑿪'
    TamilCurrentSign,
    /// \u{11feb}: '𑿫'
    TamilAndOddSign,
    /// \u{11fec}: '𑿬'
    TamilSpentSign,
    /// \u{11fed}: '𑿭'
    TamilTotalSign,
    /// \u{11fee}: '𑿮'
    TamilInPossessionSign,
    /// \u{11fef}: '𑿯'
    TamilStartingFromSign,
    /// \u{11ff0}: '𑿰'
    TamilSignMuthaliya,
    /// \u{11ff1}: '𑿱'
    TamilSignVakaiyaraa,
}

impl Into<char> for TamilSupplement {
    fn into(self) -> char {
        match self {
            TamilSupplement::TamilFractionOneThreeDashHundredDashAndDashTwentieth => '𑿀',
            TamilSupplement::TamilFractionOneOneDashHundredDashAndDashSixtieth => '𑿁',
            TamilSupplement::TamilFractionOneEightieth => '𑿂',
            TamilSupplement::TamilFractionOneSixtyDashFourth => '𑿃',
            TamilSupplement::TamilFractionOneFortieth => '𑿄',
            TamilSupplement::TamilFractionOneThirtyDashSecond => '𑿅',
            TamilSupplement::TamilFractionThreeEightieths => '𑿆',
            TamilSupplement::TamilFractionThreeSixtyDashFourths => '𑿇',
            TamilSupplement::TamilFractionOneTwentieth => '𑿈',
            TamilSupplement::TamilFractionOneSixteenthDash1 => '𑿉',
            TamilSupplement::TamilFractionOneSixteenthDash2 => '𑿊',
            TamilSupplement::TamilFractionOneTenth => '𑿋',
            TamilSupplement::TamilFractionOneEighth => '𑿌',
            TamilSupplement::TamilFractionThreeTwentieths => '𑿍',
            TamilSupplement::TamilFractionThreeSixteenths => '𑿎',
            TamilSupplement::TamilFractionOneFifth => '𑿏',
            TamilSupplement::TamilFractionOneQuarter => '𑿐',
            TamilSupplement::TamilFractionOneHalfDash1 => '𑿑',
            TamilSupplement::TamilFractionOneHalfDash2 => '𑿒',
            TamilSupplement::TamilFractionThreeQuarters => '𑿓',
            TamilSupplement::TamilFractionDownscalingFactorKiizh => '𑿔',
            TamilSupplement::TamilSignNel => '𑿕',
            TamilSupplement::TamilSignCevitu => '𑿖',
            TamilSupplement::TamilSignAazhaakku => '𑿗',
            TamilSupplement::TamilSignUzhakku => '𑿘',
            TamilSupplement::TamilSignMuuvuzhakku => '𑿙',
            TamilSupplement::TamilSignKuruni => '𑿚',
            TamilSupplement::TamilSignPathakku => '𑿛',
            TamilSupplement::TamilSignMukkuruni => '𑿜',
            TamilSupplement::TamilSignKaacu => '𑿝',
            TamilSupplement::TamilSignPanam => '𑿞',
            TamilSupplement::TamilSignPon => '𑿟',
            TamilSupplement::TamilSignVaraakan => '𑿠',
            TamilSupplement::TamilSignPaaram => '𑿡',
            TamilSupplement::TamilSignKuzhi => '𑿢',
            TamilSupplement::TamilSignVeli => '𑿣',
            TamilSupplement::TamilWetCultivationSign => '𑿤',
            TamilSupplement::TamilDryCultivationSign => '𑿥',
            TamilSupplement::TamilLandSign => '𑿦',
            TamilSupplement::TamilSaltPanSign => '𑿧',
            TamilSupplement::TamilTraditionalCreditSign => '𑿨',
            TamilSupplement::TamilTraditionalNumberSign => '𑿩',
            TamilSupplement::TamilCurrentSign => '𑿪',
            TamilSupplement::TamilAndOddSign => '𑿫',
            TamilSupplement::TamilSpentSign => '𑿬',
            TamilSupplement::TamilTotalSign => '𑿭',
            TamilSupplement::TamilInPossessionSign => '𑿮',
            TamilSupplement::TamilStartingFromSign => '𑿯',
            TamilSupplement::TamilSignMuthaliya => '𑿰',
            TamilSupplement::TamilSignVakaiyaraa => '𑿱',
        }
    }
}

impl std::convert::TryFrom<char> for TamilSupplement {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𑿀' => Ok(TamilSupplement::TamilFractionOneThreeDashHundredDashAndDashTwentieth),
            '𑿁' => Ok(TamilSupplement::TamilFractionOneOneDashHundredDashAndDashSixtieth),
            '𑿂' => Ok(TamilSupplement::TamilFractionOneEightieth),
            '𑿃' => Ok(TamilSupplement::TamilFractionOneSixtyDashFourth),
            '𑿄' => Ok(TamilSupplement::TamilFractionOneFortieth),
            '𑿅' => Ok(TamilSupplement::TamilFractionOneThirtyDashSecond),
            '𑿆' => Ok(TamilSupplement::TamilFractionThreeEightieths),
            '𑿇' => Ok(TamilSupplement::TamilFractionThreeSixtyDashFourths),
            '𑿈' => Ok(TamilSupplement::TamilFractionOneTwentieth),
            '𑿉' => Ok(TamilSupplement::TamilFractionOneSixteenthDash1),
            '𑿊' => Ok(TamilSupplement::TamilFractionOneSixteenthDash2),
            '𑿋' => Ok(TamilSupplement::TamilFractionOneTenth),
            '𑿌' => Ok(TamilSupplement::TamilFractionOneEighth),
            '𑿍' => Ok(TamilSupplement::TamilFractionThreeTwentieths),
            '𑿎' => Ok(TamilSupplement::TamilFractionThreeSixteenths),
            '𑿏' => Ok(TamilSupplement::TamilFractionOneFifth),
            '𑿐' => Ok(TamilSupplement::TamilFractionOneQuarter),
            '𑿑' => Ok(TamilSupplement::TamilFractionOneHalfDash1),
            '𑿒' => Ok(TamilSupplement::TamilFractionOneHalfDash2),
            '𑿓' => Ok(TamilSupplement::TamilFractionThreeQuarters),
            '𑿔' => Ok(TamilSupplement::TamilFractionDownscalingFactorKiizh),
            '𑿕' => Ok(TamilSupplement::TamilSignNel),
            '𑿖' => Ok(TamilSupplement::TamilSignCevitu),
            '𑿗' => Ok(TamilSupplement::TamilSignAazhaakku),
            '𑿘' => Ok(TamilSupplement::TamilSignUzhakku),
            '𑿙' => Ok(TamilSupplement::TamilSignMuuvuzhakku),
            '𑿚' => Ok(TamilSupplement::TamilSignKuruni),
            '𑿛' => Ok(TamilSupplement::TamilSignPathakku),
            '𑿜' => Ok(TamilSupplement::TamilSignMukkuruni),
            '𑿝' => Ok(TamilSupplement::TamilSignKaacu),
            '𑿞' => Ok(TamilSupplement::TamilSignPanam),
            '𑿟' => Ok(TamilSupplement::TamilSignPon),
            '𑿠' => Ok(TamilSupplement::TamilSignVaraakan),
            '𑿡' => Ok(TamilSupplement::TamilSignPaaram),
            '𑿢' => Ok(TamilSupplement::TamilSignKuzhi),
            '𑿣' => Ok(TamilSupplement::TamilSignVeli),
            '𑿤' => Ok(TamilSupplement::TamilWetCultivationSign),
            '𑿥' => Ok(TamilSupplement::TamilDryCultivationSign),
            '𑿦' => Ok(TamilSupplement::TamilLandSign),
            '𑿧' => Ok(TamilSupplement::TamilSaltPanSign),
            '𑿨' => Ok(TamilSupplement::TamilTraditionalCreditSign),
            '𑿩' => Ok(TamilSupplement::TamilTraditionalNumberSign),
            '𑿪' => Ok(TamilSupplement::TamilCurrentSign),
            '𑿫' => Ok(TamilSupplement::TamilAndOddSign),
            '𑿬' => Ok(TamilSupplement::TamilSpentSign),
            '𑿭' => Ok(TamilSupplement::TamilTotalSign),
            '𑿮' => Ok(TamilSupplement::TamilInPossessionSign),
            '𑿯' => Ok(TamilSupplement::TamilStartingFromSign),
            '𑿰' => Ok(TamilSupplement::TamilSignMuthaliya),
            '𑿱' => Ok(TamilSupplement::TamilSignVakaiyaraa),
            _ => Err(()),
        }
    }
}

impl Into<u32> for TamilSupplement {
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

impl std::convert::TryFrom<u32> for TamilSupplement {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for TamilSupplement {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl TamilSupplement {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        TamilSupplement::TamilFractionOneThreeDashHundredDashAndDashTwentieth
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            TamilSupplement::TamilFractionOneThreeDashHundredDashAndDashTwentieth => "tamil fraction one three-hundred-and-twentieth",
            TamilSupplement::TamilFractionOneOneDashHundredDashAndDashSixtieth => "tamil fraction one one-hundred-and-sixtieth",
            TamilSupplement::TamilFractionOneEightieth => "tamil fraction one eightieth",
            TamilSupplement::TamilFractionOneSixtyDashFourth => "tamil fraction one sixty-fourth",
            TamilSupplement::TamilFractionOneFortieth => "tamil fraction one fortieth",
            TamilSupplement::TamilFractionOneThirtyDashSecond => "tamil fraction one thirty-second",
            TamilSupplement::TamilFractionThreeEightieths => "tamil fraction three eightieths",
            TamilSupplement::TamilFractionThreeSixtyDashFourths => "tamil fraction three sixty-fourths",
            TamilSupplement::TamilFractionOneTwentieth => "tamil fraction one twentieth",
            TamilSupplement::TamilFractionOneSixteenthDash1 => "tamil fraction one sixteenth-1",
            TamilSupplement::TamilFractionOneSixteenthDash2 => "tamil fraction one sixteenth-2",
            TamilSupplement::TamilFractionOneTenth => "tamil fraction one tenth",
            TamilSupplement::TamilFractionOneEighth => "tamil fraction one eighth",
            TamilSupplement::TamilFractionThreeTwentieths => "tamil fraction three twentieths",
            TamilSupplement::TamilFractionThreeSixteenths => "tamil fraction three sixteenths",
            TamilSupplement::TamilFractionOneFifth => "tamil fraction one fifth",
            TamilSupplement::TamilFractionOneQuarter => "tamil fraction one quarter",
            TamilSupplement::TamilFractionOneHalfDash1 => "tamil fraction one half-1",
            TamilSupplement::TamilFractionOneHalfDash2 => "tamil fraction one half-2",
            TamilSupplement::TamilFractionThreeQuarters => "tamil fraction three quarters",
            TamilSupplement::TamilFractionDownscalingFactorKiizh => "tamil fraction downscaling factor kiizh",
            TamilSupplement::TamilSignNel => "tamil sign nel",
            TamilSupplement::TamilSignCevitu => "tamil sign cevitu",
            TamilSupplement::TamilSignAazhaakku => "tamil sign aazhaakku",
            TamilSupplement::TamilSignUzhakku => "tamil sign uzhakku",
            TamilSupplement::TamilSignMuuvuzhakku => "tamil sign muuvuzhakku",
            TamilSupplement::TamilSignKuruni => "tamil sign kuruni",
            TamilSupplement::TamilSignPathakku => "tamil sign pathakku",
            TamilSupplement::TamilSignMukkuruni => "tamil sign mukkuruni",
            TamilSupplement::TamilSignKaacu => "tamil sign kaacu",
            TamilSupplement::TamilSignPanam => "tamil sign panam",
            TamilSupplement::TamilSignPon => "tamil sign pon",
            TamilSupplement::TamilSignVaraakan => "tamil sign varaakan",
            TamilSupplement::TamilSignPaaram => "tamil sign paaram",
            TamilSupplement::TamilSignKuzhi => "tamil sign kuzhi",
            TamilSupplement::TamilSignVeli => "tamil sign veli",
            TamilSupplement::TamilWetCultivationSign => "tamil wet cultivation sign",
            TamilSupplement::TamilDryCultivationSign => "tamil dry cultivation sign",
            TamilSupplement::TamilLandSign => "tamil land sign",
            TamilSupplement::TamilSaltPanSign => "tamil salt pan sign",
            TamilSupplement::TamilTraditionalCreditSign => "tamil traditional credit sign",
            TamilSupplement::TamilTraditionalNumberSign => "tamil traditional number sign",
            TamilSupplement::TamilCurrentSign => "tamil current sign",
            TamilSupplement::TamilAndOddSign => "tamil and odd sign",
            TamilSupplement::TamilSpentSign => "tamil spent sign",
            TamilSupplement::TamilTotalSign => "tamil total sign",
            TamilSupplement::TamilInPossessionSign => "tamil in possession sign",
            TamilSupplement::TamilStartingFromSign => "tamil starting from sign",
            TamilSupplement::TamilSignMuthaliya => "tamil sign muthaliya",
            TamilSupplement::TamilSignVakaiyaraa => "tamil sign vakaiyaraa",
        }
    }
}
