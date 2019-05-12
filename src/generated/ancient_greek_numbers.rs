
/// An enum to represent all characters in the AncientGreekNumbers block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum AncientGreekNumbers {
    /// \u{10140}: '𐅀'
    GreekAcrophonicAtticOneQuarter,
    /// \u{10141}: '𐅁'
    GreekAcrophonicAtticOneHalf,
    /// \u{10142}: '𐅂'
    GreekAcrophonicAtticOneDrachma,
    /// \u{10143}: '𐅃'
    GreekAcrophonicAtticFive,
    /// \u{10144}: '𐅄'
    GreekAcrophonicAtticFifty,
    /// \u{10145}: '𐅅'
    GreekAcrophonicAtticFiveHundred,
    /// \u{10146}: '𐅆'
    GreekAcrophonicAtticFiveThousand,
    /// \u{10147}: '𐅇'
    GreekAcrophonicAtticFiftyThousand,
    /// \u{10148}: '𐅈'
    GreekAcrophonicAtticFiveTalents,
    /// \u{10149}: '𐅉'
    GreekAcrophonicAtticTenTalents,
    /// \u{1014a}: '𐅊'
    GreekAcrophonicAtticFiftyTalents,
    /// \u{1014b}: '𐅋'
    GreekAcrophonicAtticOneHundredTalents,
    /// \u{1014c}: '𐅌'
    GreekAcrophonicAtticFiveHundredTalents,
    /// \u{1014d}: '𐅍'
    GreekAcrophonicAtticOneThousandTalents,
    /// \u{1014e}: '𐅎'
    GreekAcrophonicAtticFiveThousandTalents,
    /// \u{1014f}: '𐅏'
    GreekAcrophonicAtticFiveStaters,
    /// \u{10150}: '𐅐'
    GreekAcrophonicAtticTenStaters,
    /// \u{10151}: '𐅑'
    GreekAcrophonicAtticFiftyStaters,
    /// \u{10152}: '𐅒'
    GreekAcrophonicAtticOneHundredStaters,
    /// \u{10153}: '𐅓'
    GreekAcrophonicAtticFiveHundredStaters,
    /// \u{10154}: '𐅔'
    GreekAcrophonicAtticOneThousandStaters,
    /// \u{10155}: '𐅕'
    GreekAcrophonicAtticTenThousandStaters,
    /// \u{10156}: '𐅖'
    GreekAcrophonicAtticFiftyThousandStaters,
    /// \u{10157}: '𐅗'
    GreekAcrophonicAtticTenMnas,
    /// \u{10158}: '𐅘'
    GreekAcrophonicHeraeumOnePlethron,
    /// \u{10159}: '𐅙'
    GreekAcrophonicThespianOne,
    /// \u{1015a}: '𐅚'
    GreekAcrophonicHermionianOne,
    /// \u{1015b}: '𐅛'
    GreekAcrophonicEpidaureanTwo,
    /// \u{1015c}: '𐅜'
    GreekAcrophonicThespianTwo,
    /// \u{1015d}: '𐅝'
    GreekAcrophonicCyrenaicTwoDrachmas,
    /// \u{1015e}: '𐅞'
    GreekAcrophonicEpidaureanTwoDrachmas,
    /// \u{1015f}: '𐅟'
    GreekAcrophonicTroezenianFive,
    /// \u{10160}: '𐅠'
    GreekAcrophonicTroezenianTen,
    /// \u{10161}: '𐅡'
    GreekAcrophonicTroezenianTenAlternateForm,
    /// \u{10162}: '𐅢'
    GreekAcrophonicHermionianTen,
    /// \u{10163}: '𐅣'
    GreekAcrophonicMessenianTen,
    /// \u{10164}: '𐅤'
    GreekAcrophonicThespianTen,
    /// \u{10165}: '𐅥'
    GreekAcrophonicThespianThirty,
    /// \u{10166}: '𐅦'
    GreekAcrophonicTroezenianFifty,
    /// \u{10167}: '𐅧'
    GreekAcrophonicTroezenianFiftyAlternateForm,
    /// \u{10168}: '𐅨'
    GreekAcrophonicHermionianFifty,
    /// \u{10169}: '𐅩'
    GreekAcrophonicThespianFifty,
    /// \u{1016a}: '𐅪'
    GreekAcrophonicThespianOneHundred,
    /// \u{1016b}: '𐅫'
    GreekAcrophonicThespianThreeHundred,
    /// \u{1016c}: '𐅬'
    GreekAcrophonicEpidaureanFiveHundred,
    /// \u{1016d}: '𐅭'
    GreekAcrophonicTroezenianFiveHundred,
    /// \u{1016e}: '𐅮'
    GreekAcrophonicThespianFiveHundred,
    /// \u{1016f}: '𐅯'
    GreekAcrophonicCarystianFiveHundred,
    /// \u{10170}: '𐅰'
    GreekAcrophonicNaxianFiveHundred,
    /// \u{10171}: '𐅱'
    GreekAcrophonicThespianOneThousand,
    /// \u{10172}: '𐅲'
    GreekAcrophonicThespianFiveThousand,
    /// \u{10173}: '𐅳'
    GreekAcrophonicDelphicFiveMnas,
    /// \u{10174}: '𐅴'
    GreekAcrophonicStratianFiftyMnas,
    /// \u{10175}: '𐅵'
    GreekOneHalfSign,
    /// \u{10176}: '𐅶'
    GreekOneHalfSignAlternateForm,
    /// \u{10177}: '𐅷'
    GreekTwoThirdsSign,
    /// \u{10178}: '𐅸'
    GreekThreeQuartersSign,
    /// \u{10179}: '𐅹'
    GreekYearSign,
    /// \u{1017a}: '𐅺'
    GreekTalentSign,
    /// \u{1017b}: '𐅻'
    GreekDrachmaSign,
    /// \u{1017c}: '𐅼'
    GreekObolSign,
    /// \u{1017d}: '𐅽'
    GreekTwoObolsSign,
    /// \u{1017e}: '𐅾'
    GreekThreeObolsSign,
    /// \u{1017f}: '𐅿'
    GreekFourObolsSign,
    /// \u{10180}: '𐆀'
    GreekFiveObolsSign,
    /// \u{10181}: '𐆁'
    GreekMetretesSign,
    /// \u{10182}: '𐆂'
    GreekKyathosBaseSign,
    /// \u{10183}: '𐆃'
    GreekLitraSign,
    /// \u{10184}: '𐆄'
    GreekOunkiaSign,
    /// \u{10185}: '𐆅'
    GreekXestesSign,
    /// \u{10186}: '𐆆'
    GreekArtabeSign,
    /// \u{10187}: '𐆇'
    GreekArouraSign,
    /// \u{10188}: '𐆈'
    GreekGrammaSign,
    /// \u{10189}: '𐆉'
    GreekTryblionBaseSign,
    /// \u{1018a}: '𐆊'
    GreekZeroSign,
    /// \u{1018b}: '𐆋'
    GreekOneQuarterSign,
    /// \u{1018c}: '𐆌'
    GreekSinusoidSign,
    /// \u{1018d}: '𐆍'
    GreekIndictionSign,
    /// \u{1018e}: '𐆎'
    NomismaSign,
}

impl Into<char> for AncientGreekNumbers {
    fn into(self) -> char {
        match self {
            AncientGreekNumbers::GreekAcrophonicAtticOneQuarter => '𐅀',
            AncientGreekNumbers::GreekAcrophonicAtticOneHalf => '𐅁',
            AncientGreekNumbers::GreekAcrophonicAtticOneDrachma => '𐅂',
            AncientGreekNumbers::GreekAcrophonicAtticFive => '𐅃',
            AncientGreekNumbers::GreekAcrophonicAtticFifty => '𐅄',
            AncientGreekNumbers::GreekAcrophonicAtticFiveHundred => '𐅅',
            AncientGreekNumbers::GreekAcrophonicAtticFiveThousand => '𐅆',
            AncientGreekNumbers::GreekAcrophonicAtticFiftyThousand => '𐅇',
            AncientGreekNumbers::GreekAcrophonicAtticFiveTalents => '𐅈',
            AncientGreekNumbers::GreekAcrophonicAtticTenTalents => '𐅉',
            AncientGreekNumbers::GreekAcrophonicAtticFiftyTalents => '𐅊',
            AncientGreekNumbers::GreekAcrophonicAtticOneHundredTalents => '𐅋',
            AncientGreekNumbers::GreekAcrophonicAtticFiveHundredTalents => '𐅌',
            AncientGreekNumbers::GreekAcrophonicAtticOneThousandTalents => '𐅍',
            AncientGreekNumbers::GreekAcrophonicAtticFiveThousandTalents => '𐅎',
            AncientGreekNumbers::GreekAcrophonicAtticFiveStaters => '𐅏',
            AncientGreekNumbers::GreekAcrophonicAtticTenStaters => '𐅐',
            AncientGreekNumbers::GreekAcrophonicAtticFiftyStaters => '𐅑',
            AncientGreekNumbers::GreekAcrophonicAtticOneHundredStaters => '𐅒',
            AncientGreekNumbers::GreekAcrophonicAtticFiveHundredStaters => '𐅓',
            AncientGreekNumbers::GreekAcrophonicAtticOneThousandStaters => '𐅔',
            AncientGreekNumbers::GreekAcrophonicAtticTenThousandStaters => '𐅕',
            AncientGreekNumbers::GreekAcrophonicAtticFiftyThousandStaters => '𐅖',
            AncientGreekNumbers::GreekAcrophonicAtticTenMnas => '𐅗',
            AncientGreekNumbers::GreekAcrophonicHeraeumOnePlethron => '𐅘',
            AncientGreekNumbers::GreekAcrophonicThespianOne => '𐅙',
            AncientGreekNumbers::GreekAcrophonicHermionianOne => '𐅚',
            AncientGreekNumbers::GreekAcrophonicEpidaureanTwo => '𐅛',
            AncientGreekNumbers::GreekAcrophonicThespianTwo => '𐅜',
            AncientGreekNumbers::GreekAcrophonicCyrenaicTwoDrachmas => '𐅝',
            AncientGreekNumbers::GreekAcrophonicEpidaureanTwoDrachmas => '𐅞',
            AncientGreekNumbers::GreekAcrophonicTroezenianFive => '𐅟',
            AncientGreekNumbers::GreekAcrophonicTroezenianTen => '𐅠',
            AncientGreekNumbers::GreekAcrophonicTroezenianTenAlternateForm => '𐅡',
            AncientGreekNumbers::GreekAcrophonicHermionianTen => '𐅢',
            AncientGreekNumbers::GreekAcrophonicMessenianTen => '𐅣',
            AncientGreekNumbers::GreekAcrophonicThespianTen => '𐅤',
            AncientGreekNumbers::GreekAcrophonicThespianThirty => '𐅥',
            AncientGreekNumbers::GreekAcrophonicTroezenianFifty => '𐅦',
            AncientGreekNumbers::GreekAcrophonicTroezenianFiftyAlternateForm => '𐅧',
            AncientGreekNumbers::GreekAcrophonicHermionianFifty => '𐅨',
            AncientGreekNumbers::GreekAcrophonicThespianFifty => '𐅩',
            AncientGreekNumbers::GreekAcrophonicThespianOneHundred => '𐅪',
            AncientGreekNumbers::GreekAcrophonicThespianThreeHundred => '𐅫',
            AncientGreekNumbers::GreekAcrophonicEpidaureanFiveHundred => '𐅬',
            AncientGreekNumbers::GreekAcrophonicTroezenianFiveHundred => '𐅭',
            AncientGreekNumbers::GreekAcrophonicThespianFiveHundred => '𐅮',
            AncientGreekNumbers::GreekAcrophonicCarystianFiveHundred => '𐅯',
            AncientGreekNumbers::GreekAcrophonicNaxianFiveHundred => '𐅰',
            AncientGreekNumbers::GreekAcrophonicThespianOneThousand => '𐅱',
            AncientGreekNumbers::GreekAcrophonicThespianFiveThousand => '𐅲',
            AncientGreekNumbers::GreekAcrophonicDelphicFiveMnas => '𐅳',
            AncientGreekNumbers::GreekAcrophonicStratianFiftyMnas => '𐅴',
            AncientGreekNumbers::GreekOneHalfSign => '𐅵',
            AncientGreekNumbers::GreekOneHalfSignAlternateForm => '𐅶',
            AncientGreekNumbers::GreekTwoThirdsSign => '𐅷',
            AncientGreekNumbers::GreekThreeQuartersSign => '𐅸',
            AncientGreekNumbers::GreekYearSign => '𐅹',
            AncientGreekNumbers::GreekTalentSign => '𐅺',
            AncientGreekNumbers::GreekDrachmaSign => '𐅻',
            AncientGreekNumbers::GreekObolSign => '𐅼',
            AncientGreekNumbers::GreekTwoObolsSign => '𐅽',
            AncientGreekNumbers::GreekThreeObolsSign => '𐅾',
            AncientGreekNumbers::GreekFourObolsSign => '𐅿',
            AncientGreekNumbers::GreekFiveObolsSign => '𐆀',
            AncientGreekNumbers::GreekMetretesSign => '𐆁',
            AncientGreekNumbers::GreekKyathosBaseSign => '𐆂',
            AncientGreekNumbers::GreekLitraSign => '𐆃',
            AncientGreekNumbers::GreekOunkiaSign => '𐆄',
            AncientGreekNumbers::GreekXestesSign => '𐆅',
            AncientGreekNumbers::GreekArtabeSign => '𐆆',
            AncientGreekNumbers::GreekArouraSign => '𐆇',
            AncientGreekNumbers::GreekGrammaSign => '𐆈',
            AncientGreekNumbers::GreekTryblionBaseSign => '𐆉',
            AncientGreekNumbers::GreekZeroSign => '𐆊',
            AncientGreekNumbers::GreekOneQuarterSign => '𐆋',
            AncientGreekNumbers::GreekSinusoidSign => '𐆌',
            AncientGreekNumbers::GreekIndictionSign => '𐆍',
            AncientGreekNumbers::NomismaSign => '𐆎',
        }
    }
}

impl std::convert::TryFrom<char> for AncientGreekNumbers {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𐅀' => Ok(AncientGreekNumbers::GreekAcrophonicAtticOneQuarter),
            '𐅁' => Ok(AncientGreekNumbers::GreekAcrophonicAtticOneHalf),
            '𐅂' => Ok(AncientGreekNumbers::GreekAcrophonicAtticOneDrachma),
            '𐅃' => Ok(AncientGreekNumbers::GreekAcrophonicAtticFive),
            '𐅄' => Ok(AncientGreekNumbers::GreekAcrophonicAtticFifty),
            '𐅅' => Ok(AncientGreekNumbers::GreekAcrophonicAtticFiveHundred),
            '𐅆' => Ok(AncientGreekNumbers::GreekAcrophonicAtticFiveThousand),
            '𐅇' => Ok(AncientGreekNumbers::GreekAcrophonicAtticFiftyThousand),
            '𐅈' => Ok(AncientGreekNumbers::GreekAcrophonicAtticFiveTalents),
            '𐅉' => Ok(AncientGreekNumbers::GreekAcrophonicAtticTenTalents),
            '𐅊' => Ok(AncientGreekNumbers::GreekAcrophonicAtticFiftyTalents),
            '𐅋' => Ok(AncientGreekNumbers::GreekAcrophonicAtticOneHundredTalents),
            '𐅌' => Ok(AncientGreekNumbers::GreekAcrophonicAtticFiveHundredTalents),
            '𐅍' => Ok(AncientGreekNumbers::GreekAcrophonicAtticOneThousandTalents),
            '𐅎' => Ok(AncientGreekNumbers::GreekAcrophonicAtticFiveThousandTalents),
            '𐅏' => Ok(AncientGreekNumbers::GreekAcrophonicAtticFiveStaters),
            '𐅐' => Ok(AncientGreekNumbers::GreekAcrophonicAtticTenStaters),
            '𐅑' => Ok(AncientGreekNumbers::GreekAcrophonicAtticFiftyStaters),
            '𐅒' => Ok(AncientGreekNumbers::GreekAcrophonicAtticOneHundredStaters),
            '𐅓' => Ok(AncientGreekNumbers::GreekAcrophonicAtticFiveHundredStaters),
            '𐅔' => Ok(AncientGreekNumbers::GreekAcrophonicAtticOneThousandStaters),
            '𐅕' => Ok(AncientGreekNumbers::GreekAcrophonicAtticTenThousandStaters),
            '𐅖' => Ok(AncientGreekNumbers::GreekAcrophonicAtticFiftyThousandStaters),
            '𐅗' => Ok(AncientGreekNumbers::GreekAcrophonicAtticTenMnas),
            '𐅘' => Ok(AncientGreekNumbers::GreekAcrophonicHeraeumOnePlethron),
            '𐅙' => Ok(AncientGreekNumbers::GreekAcrophonicThespianOne),
            '𐅚' => Ok(AncientGreekNumbers::GreekAcrophonicHermionianOne),
            '𐅛' => Ok(AncientGreekNumbers::GreekAcrophonicEpidaureanTwo),
            '𐅜' => Ok(AncientGreekNumbers::GreekAcrophonicThespianTwo),
            '𐅝' => Ok(AncientGreekNumbers::GreekAcrophonicCyrenaicTwoDrachmas),
            '𐅞' => Ok(AncientGreekNumbers::GreekAcrophonicEpidaureanTwoDrachmas),
            '𐅟' => Ok(AncientGreekNumbers::GreekAcrophonicTroezenianFive),
            '𐅠' => Ok(AncientGreekNumbers::GreekAcrophonicTroezenianTen),
            '𐅡' => Ok(AncientGreekNumbers::GreekAcrophonicTroezenianTenAlternateForm),
            '𐅢' => Ok(AncientGreekNumbers::GreekAcrophonicHermionianTen),
            '𐅣' => Ok(AncientGreekNumbers::GreekAcrophonicMessenianTen),
            '𐅤' => Ok(AncientGreekNumbers::GreekAcrophonicThespianTen),
            '𐅥' => Ok(AncientGreekNumbers::GreekAcrophonicThespianThirty),
            '𐅦' => Ok(AncientGreekNumbers::GreekAcrophonicTroezenianFifty),
            '𐅧' => Ok(AncientGreekNumbers::GreekAcrophonicTroezenianFiftyAlternateForm),
            '𐅨' => Ok(AncientGreekNumbers::GreekAcrophonicHermionianFifty),
            '𐅩' => Ok(AncientGreekNumbers::GreekAcrophonicThespianFifty),
            '𐅪' => Ok(AncientGreekNumbers::GreekAcrophonicThespianOneHundred),
            '𐅫' => Ok(AncientGreekNumbers::GreekAcrophonicThespianThreeHundred),
            '𐅬' => Ok(AncientGreekNumbers::GreekAcrophonicEpidaureanFiveHundred),
            '𐅭' => Ok(AncientGreekNumbers::GreekAcrophonicTroezenianFiveHundred),
            '𐅮' => Ok(AncientGreekNumbers::GreekAcrophonicThespianFiveHundred),
            '𐅯' => Ok(AncientGreekNumbers::GreekAcrophonicCarystianFiveHundred),
            '𐅰' => Ok(AncientGreekNumbers::GreekAcrophonicNaxianFiveHundred),
            '𐅱' => Ok(AncientGreekNumbers::GreekAcrophonicThespianOneThousand),
            '𐅲' => Ok(AncientGreekNumbers::GreekAcrophonicThespianFiveThousand),
            '𐅳' => Ok(AncientGreekNumbers::GreekAcrophonicDelphicFiveMnas),
            '𐅴' => Ok(AncientGreekNumbers::GreekAcrophonicStratianFiftyMnas),
            '𐅵' => Ok(AncientGreekNumbers::GreekOneHalfSign),
            '𐅶' => Ok(AncientGreekNumbers::GreekOneHalfSignAlternateForm),
            '𐅷' => Ok(AncientGreekNumbers::GreekTwoThirdsSign),
            '𐅸' => Ok(AncientGreekNumbers::GreekThreeQuartersSign),
            '𐅹' => Ok(AncientGreekNumbers::GreekYearSign),
            '𐅺' => Ok(AncientGreekNumbers::GreekTalentSign),
            '𐅻' => Ok(AncientGreekNumbers::GreekDrachmaSign),
            '𐅼' => Ok(AncientGreekNumbers::GreekObolSign),
            '𐅽' => Ok(AncientGreekNumbers::GreekTwoObolsSign),
            '𐅾' => Ok(AncientGreekNumbers::GreekThreeObolsSign),
            '𐅿' => Ok(AncientGreekNumbers::GreekFourObolsSign),
            '𐆀' => Ok(AncientGreekNumbers::GreekFiveObolsSign),
            '𐆁' => Ok(AncientGreekNumbers::GreekMetretesSign),
            '𐆂' => Ok(AncientGreekNumbers::GreekKyathosBaseSign),
            '𐆃' => Ok(AncientGreekNumbers::GreekLitraSign),
            '𐆄' => Ok(AncientGreekNumbers::GreekOunkiaSign),
            '𐆅' => Ok(AncientGreekNumbers::GreekXestesSign),
            '𐆆' => Ok(AncientGreekNumbers::GreekArtabeSign),
            '𐆇' => Ok(AncientGreekNumbers::GreekArouraSign),
            '𐆈' => Ok(AncientGreekNumbers::GreekGrammaSign),
            '𐆉' => Ok(AncientGreekNumbers::GreekTryblionBaseSign),
            '𐆊' => Ok(AncientGreekNumbers::GreekZeroSign),
            '𐆋' => Ok(AncientGreekNumbers::GreekOneQuarterSign),
            '𐆌' => Ok(AncientGreekNumbers::GreekSinusoidSign),
            '𐆍' => Ok(AncientGreekNumbers::GreekIndictionSign),
            '𐆎' => Ok(AncientGreekNumbers::NomismaSign),
            _ => Err(()),
        }
    }
}

impl Into<u32> for AncientGreekNumbers {
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

impl std::convert::TryFrom<u32> for AncientGreekNumbers {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for AncientGreekNumbers {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl AncientGreekNumbers {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        AncientGreekNumbers::GreekAcrophonicAtticOneQuarter
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            AncientGreekNumbers::GreekAcrophonicAtticOneQuarter => "greek acrophonic attic one quarter",
            AncientGreekNumbers::GreekAcrophonicAtticOneHalf => "greek acrophonic attic one half",
            AncientGreekNumbers::GreekAcrophonicAtticOneDrachma => "greek acrophonic attic one drachma",
            AncientGreekNumbers::GreekAcrophonicAtticFive => "greek acrophonic attic five",
            AncientGreekNumbers::GreekAcrophonicAtticFifty => "greek acrophonic attic fifty",
            AncientGreekNumbers::GreekAcrophonicAtticFiveHundred => "greek acrophonic attic five hundred",
            AncientGreekNumbers::GreekAcrophonicAtticFiveThousand => "greek acrophonic attic five thousand",
            AncientGreekNumbers::GreekAcrophonicAtticFiftyThousand => "greek acrophonic attic fifty thousand",
            AncientGreekNumbers::GreekAcrophonicAtticFiveTalents => "greek acrophonic attic five talents",
            AncientGreekNumbers::GreekAcrophonicAtticTenTalents => "greek acrophonic attic ten talents",
            AncientGreekNumbers::GreekAcrophonicAtticFiftyTalents => "greek acrophonic attic fifty talents",
            AncientGreekNumbers::GreekAcrophonicAtticOneHundredTalents => "greek acrophonic attic one hundred talents",
            AncientGreekNumbers::GreekAcrophonicAtticFiveHundredTalents => "greek acrophonic attic five hundred talents",
            AncientGreekNumbers::GreekAcrophonicAtticOneThousandTalents => "greek acrophonic attic one thousand talents",
            AncientGreekNumbers::GreekAcrophonicAtticFiveThousandTalents => "greek acrophonic attic five thousand talents",
            AncientGreekNumbers::GreekAcrophonicAtticFiveStaters => "greek acrophonic attic five staters",
            AncientGreekNumbers::GreekAcrophonicAtticTenStaters => "greek acrophonic attic ten staters",
            AncientGreekNumbers::GreekAcrophonicAtticFiftyStaters => "greek acrophonic attic fifty staters",
            AncientGreekNumbers::GreekAcrophonicAtticOneHundredStaters => "greek acrophonic attic one hundred staters",
            AncientGreekNumbers::GreekAcrophonicAtticFiveHundredStaters => "greek acrophonic attic five hundred staters",
            AncientGreekNumbers::GreekAcrophonicAtticOneThousandStaters => "greek acrophonic attic one thousand staters",
            AncientGreekNumbers::GreekAcrophonicAtticTenThousandStaters => "greek acrophonic attic ten thousand staters",
            AncientGreekNumbers::GreekAcrophonicAtticFiftyThousandStaters => "greek acrophonic attic fifty thousand staters",
            AncientGreekNumbers::GreekAcrophonicAtticTenMnas => "greek acrophonic attic ten mnas",
            AncientGreekNumbers::GreekAcrophonicHeraeumOnePlethron => "greek acrophonic heraeum one plethron",
            AncientGreekNumbers::GreekAcrophonicThespianOne => "greek acrophonic thespian one",
            AncientGreekNumbers::GreekAcrophonicHermionianOne => "greek acrophonic hermionian one",
            AncientGreekNumbers::GreekAcrophonicEpidaureanTwo => "greek acrophonic epidaurean two",
            AncientGreekNumbers::GreekAcrophonicThespianTwo => "greek acrophonic thespian two",
            AncientGreekNumbers::GreekAcrophonicCyrenaicTwoDrachmas => "greek acrophonic cyrenaic two drachmas",
            AncientGreekNumbers::GreekAcrophonicEpidaureanTwoDrachmas => "greek acrophonic epidaurean two drachmas",
            AncientGreekNumbers::GreekAcrophonicTroezenianFive => "greek acrophonic troezenian five",
            AncientGreekNumbers::GreekAcrophonicTroezenianTen => "greek acrophonic troezenian ten",
            AncientGreekNumbers::GreekAcrophonicTroezenianTenAlternateForm => "greek acrophonic troezenian ten alternate form",
            AncientGreekNumbers::GreekAcrophonicHermionianTen => "greek acrophonic hermionian ten",
            AncientGreekNumbers::GreekAcrophonicMessenianTen => "greek acrophonic messenian ten",
            AncientGreekNumbers::GreekAcrophonicThespianTen => "greek acrophonic thespian ten",
            AncientGreekNumbers::GreekAcrophonicThespianThirty => "greek acrophonic thespian thirty",
            AncientGreekNumbers::GreekAcrophonicTroezenianFifty => "greek acrophonic troezenian fifty",
            AncientGreekNumbers::GreekAcrophonicTroezenianFiftyAlternateForm => "greek acrophonic troezenian fifty alternate form",
            AncientGreekNumbers::GreekAcrophonicHermionianFifty => "greek acrophonic hermionian fifty",
            AncientGreekNumbers::GreekAcrophonicThespianFifty => "greek acrophonic thespian fifty",
            AncientGreekNumbers::GreekAcrophonicThespianOneHundred => "greek acrophonic thespian one hundred",
            AncientGreekNumbers::GreekAcrophonicThespianThreeHundred => "greek acrophonic thespian three hundred",
            AncientGreekNumbers::GreekAcrophonicEpidaureanFiveHundred => "greek acrophonic epidaurean five hundred",
            AncientGreekNumbers::GreekAcrophonicTroezenianFiveHundred => "greek acrophonic troezenian five hundred",
            AncientGreekNumbers::GreekAcrophonicThespianFiveHundred => "greek acrophonic thespian five hundred",
            AncientGreekNumbers::GreekAcrophonicCarystianFiveHundred => "greek acrophonic carystian five hundred",
            AncientGreekNumbers::GreekAcrophonicNaxianFiveHundred => "greek acrophonic naxian five hundred",
            AncientGreekNumbers::GreekAcrophonicThespianOneThousand => "greek acrophonic thespian one thousand",
            AncientGreekNumbers::GreekAcrophonicThespianFiveThousand => "greek acrophonic thespian five thousand",
            AncientGreekNumbers::GreekAcrophonicDelphicFiveMnas => "greek acrophonic delphic five mnas",
            AncientGreekNumbers::GreekAcrophonicStratianFiftyMnas => "greek acrophonic stratian fifty mnas",
            AncientGreekNumbers::GreekOneHalfSign => "greek one half sign",
            AncientGreekNumbers::GreekOneHalfSignAlternateForm => "greek one half sign alternate form",
            AncientGreekNumbers::GreekTwoThirdsSign => "greek two thirds sign",
            AncientGreekNumbers::GreekThreeQuartersSign => "greek three quarters sign",
            AncientGreekNumbers::GreekYearSign => "greek year sign",
            AncientGreekNumbers::GreekTalentSign => "greek talent sign",
            AncientGreekNumbers::GreekDrachmaSign => "greek drachma sign",
            AncientGreekNumbers::GreekObolSign => "greek obol sign",
            AncientGreekNumbers::GreekTwoObolsSign => "greek two obols sign",
            AncientGreekNumbers::GreekThreeObolsSign => "greek three obols sign",
            AncientGreekNumbers::GreekFourObolsSign => "greek four obols sign",
            AncientGreekNumbers::GreekFiveObolsSign => "greek five obols sign",
            AncientGreekNumbers::GreekMetretesSign => "greek metretes sign",
            AncientGreekNumbers::GreekKyathosBaseSign => "greek kyathos base sign",
            AncientGreekNumbers::GreekLitraSign => "greek litra sign",
            AncientGreekNumbers::GreekOunkiaSign => "greek ounkia sign",
            AncientGreekNumbers::GreekXestesSign => "greek xestes sign",
            AncientGreekNumbers::GreekArtabeSign => "greek artabe sign",
            AncientGreekNumbers::GreekArouraSign => "greek aroura sign",
            AncientGreekNumbers::GreekGrammaSign => "greek gramma sign",
            AncientGreekNumbers::GreekTryblionBaseSign => "greek tryblion base sign",
            AncientGreekNumbers::GreekZeroSign => "greek zero sign",
            AncientGreekNumbers::GreekOneQuarterSign => "greek one quarter sign",
            AncientGreekNumbers::GreekSinusoidSign => "greek sinusoid sign",
            AncientGreekNumbers::GreekIndictionSign => "greek indiction sign",
            AncientGreekNumbers::NomismaSign => "nomisma sign",
        }
    }
}
