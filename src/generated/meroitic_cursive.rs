
/// An enum to represent all characters in the MeroiticCursive block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum MeroiticCursive {
    /// \u{109a0}: '𐦠'
    LetterA,
    /// \u{109a1}: '𐦡'
    LetterE,
    /// \u{109a2}: '𐦢'
    LetterI,
    /// \u{109a3}: '𐦣'
    LetterO,
    /// \u{109a4}: '𐦤'
    LetterYa,
    /// \u{109a5}: '𐦥'
    LetterWa,
    /// \u{109a6}: '𐦦'
    LetterBa,
    /// \u{109a7}: '𐦧'
    LetterPa,
    /// \u{109a8}: '𐦨'
    LetterMa,
    /// \u{109a9}: '𐦩'
    LetterNa,
    /// \u{109aa}: '𐦪'
    LetterNe,
    /// \u{109ab}: '𐦫'
    LetterRa,
    /// \u{109ac}: '𐦬'
    LetterLa,
    /// \u{109ad}: '𐦭'
    LetterKha,
    /// \u{109ae}: '𐦮'
    LetterHha,
    /// \u{109af}: '𐦯'
    LetterSa,
    /// \u{109b0}: '𐦰'
    LetterArchaicSa,
    /// \u{109b1}: '𐦱'
    LetterSe,
    /// \u{109b2}: '𐦲'
    LetterKa,
    /// \u{109b3}: '𐦳'
    LetterQa,
    /// \u{109b4}: '𐦴'
    LetterTa,
    /// \u{109b5}: '𐦵'
    LetterTe,
    /// \u{109b6}: '𐦶'
    LetterTo,
    /// \u{109b7}: '𐦷'
    LetterDa,
    /// \u{109bc}: '𐦼'
    FractionElevenTwelfths,
    /// \u{109bd}: '𐦽'
    FractionOneHalf,
    /// \u{109be}: '𐦾'
    LogogramRmt,
    /// \u{109bf}: '𐦿'
    LogogramImn,
    /// \u{109c0}: '𐧀'
    NumberOne,
    /// \u{109c1}: '𐧁'
    NumberTwo,
    /// \u{109c2}: '𐧂'
    NumberThree,
    /// \u{109c3}: '𐧃'
    NumberFour,
    /// \u{109c4}: '𐧄'
    NumberFive,
    /// \u{109c5}: '𐧅'
    NumberSix,
    /// \u{109c6}: '𐧆'
    NumberSeven,
    /// \u{109c7}: '𐧇'
    NumberEight,
    /// \u{109c8}: '𐧈'
    NumberNine,
    /// \u{109c9}: '𐧉'
    NumberTen,
    /// \u{109ca}: '𐧊'
    NumberTwenty,
    /// \u{109cb}: '𐧋'
    NumberThirty,
    /// \u{109cc}: '𐧌'
    NumberForty,
    /// \u{109cd}: '𐧍'
    NumberFifty,
    /// \u{109ce}: '𐧎'
    NumberSixty,
    /// \u{109cf}: '𐧏'
    NumberSeventy,
    /// \u{109d2}: '𐧒'
    NumberOneHundred,
    /// \u{109d3}: '𐧓'
    NumberTwoHundred,
    /// \u{109d4}: '𐧔'
    NumberThreeHundred,
    /// \u{109d5}: '𐧕'
    NumberFourHundred,
    /// \u{109d6}: '𐧖'
    NumberFiveHundred,
    /// \u{109d7}: '𐧗'
    NumberSixHundred,
    /// \u{109d8}: '𐧘'
    NumberSevenHundred,
    /// \u{109d9}: '𐧙'
    NumberEightHundred,
    /// \u{109da}: '𐧚'
    NumberNineHundred,
    /// \u{109db}: '𐧛'
    NumberOneThousand,
    /// \u{109dc}: '𐧜'
    NumberTwoThousand,
    /// \u{109dd}: '𐧝'
    NumberThreeThousand,
    /// \u{109de}: '𐧞'
    NumberFourThousand,
    /// \u{109df}: '𐧟'
    NumberFiveThousand,
    /// \u{109e0}: '𐧠'
    NumberSixThousand,
    /// \u{109e1}: '𐧡'
    NumberSevenThousand,
    /// \u{109e2}: '𐧢'
    NumberEightThousand,
    /// \u{109e3}: '𐧣'
    NumberNineThousand,
    /// \u{109e4}: '𐧤'
    NumberTenThousand,
    /// \u{109e5}: '𐧥'
    NumberTwentyThousand,
    /// \u{109e6}: '𐧦'
    NumberThirtyThousand,
    /// \u{109e7}: '𐧧'
    NumberFortyThousand,
    /// \u{109e8}: '𐧨'
    NumberFiftyThousand,
    /// \u{109e9}: '𐧩'
    NumberSixtyThousand,
    /// \u{109ea}: '𐧪'
    NumberSeventyThousand,
    /// \u{109eb}: '𐧫'
    NumberEightyThousand,
    /// \u{109ec}: '𐧬'
    NumberNinetyThousand,
    /// \u{109ed}: '𐧭'
    NumberOneHundredThousand,
    /// \u{109ee}: '𐧮'
    NumberTwoHundredThousand,
    /// \u{109ef}: '𐧯'
    NumberThreeHundredThousand,
    /// \u{109f0}: '𐧰'
    NumberFourHundredThousand,
    /// \u{109f1}: '𐧱'
    NumberFiveHundredThousand,
    /// \u{109f2}: '𐧲'
    NumberSixHundredThousand,
    /// \u{109f3}: '𐧳'
    NumberSevenHundredThousand,
    /// \u{109f4}: '𐧴'
    NumberEightHundredThousand,
    /// \u{109f5}: '𐧵'
    NumberNineHundredThousand,
    /// \u{109f6}: '𐧶'
    FractionOneTwelfth,
    /// \u{109f7}: '𐧷'
    FractionTwoTwelfths,
    /// \u{109f8}: '𐧸'
    FractionThreeTwelfths,
    /// \u{109f9}: '𐧹'
    FractionFourTwelfths,
    /// \u{109fa}: '𐧺'
    FractionFiveTwelfths,
    /// \u{109fb}: '𐧻'
    FractionSixTwelfths,
    /// \u{109fc}: '𐧼'
    FractionSevenTwelfths,
    /// \u{109fd}: '𐧽'
    FractionEightTwelfths,
    /// \u{109fe}: '𐧾'
    FractionNineTwelfths,
}

impl Into<char> for MeroiticCursive {
    fn into(self) -> char {
        match self {
            MeroiticCursive::LetterA => '𐦠',
            MeroiticCursive::LetterE => '𐦡',
            MeroiticCursive::LetterI => '𐦢',
            MeroiticCursive::LetterO => '𐦣',
            MeroiticCursive::LetterYa => '𐦤',
            MeroiticCursive::LetterWa => '𐦥',
            MeroiticCursive::LetterBa => '𐦦',
            MeroiticCursive::LetterPa => '𐦧',
            MeroiticCursive::LetterMa => '𐦨',
            MeroiticCursive::LetterNa => '𐦩',
            MeroiticCursive::LetterNe => '𐦪',
            MeroiticCursive::LetterRa => '𐦫',
            MeroiticCursive::LetterLa => '𐦬',
            MeroiticCursive::LetterKha => '𐦭',
            MeroiticCursive::LetterHha => '𐦮',
            MeroiticCursive::LetterSa => '𐦯',
            MeroiticCursive::LetterArchaicSa => '𐦰',
            MeroiticCursive::LetterSe => '𐦱',
            MeroiticCursive::LetterKa => '𐦲',
            MeroiticCursive::LetterQa => '𐦳',
            MeroiticCursive::LetterTa => '𐦴',
            MeroiticCursive::LetterTe => '𐦵',
            MeroiticCursive::LetterTo => '𐦶',
            MeroiticCursive::LetterDa => '𐦷',
            MeroiticCursive::FractionElevenTwelfths => '𐦼',
            MeroiticCursive::FractionOneHalf => '𐦽',
            MeroiticCursive::LogogramRmt => '𐦾',
            MeroiticCursive::LogogramImn => '𐦿',
            MeroiticCursive::NumberOne => '𐧀',
            MeroiticCursive::NumberTwo => '𐧁',
            MeroiticCursive::NumberThree => '𐧂',
            MeroiticCursive::NumberFour => '𐧃',
            MeroiticCursive::NumberFive => '𐧄',
            MeroiticCursive::NumberSix => '𐧅',
            MeroiticCursive::NumberSeven => '𐧆',
            MeroiticCursive::NumberEight => '𐧇',
            MeroiticCursive::NumberNine => '𐧈',
            MeroiticCursive::NumberTen => '𐧉',
            MeroiticCursive::NumberTwenty => '𐧊',
            MeroiticCursive::NumberThirty => '𐧋',
            MeroiticCursive::NumberForty => '𐧌',
            MeroiticCursive::NumberFifty => '𐧍',
            MeroiticCursive::NumberSixty => '𐧎',
            MeroiticCursive::NumberSeventy => '𐧏',
            MeroiticCursive::NumberOneHundred => '𐧒',
            MeroiticCursive::NumberTwoHundred => '𐧓',
            MeroiticCursive::NumberThreeHundred => '𐧔',
            MeroiticCursive::NumberFourHundred => '𐧕',
            MeroiticCursive::NumberFiveHundred => '𐧖',
            MeroiticCursive::NumberSixHundred => '𐧗',
            MeroiticCursive::NumberSevenHundred => '𐧘',
            MeroiticCursive::NumberEightHundred => '𐧙',
            MeroiticCursive::NumberNineHundred => '𐧚',
            MeroiticCursive::NumberOneThousand => '𐧛',
            MeroiticCursive::NumberTwoThousand => '𐧜',
            MeroiticCursive::NumberThreeThousand => '𐧝',
            MeroiticCursive::NumberFourThousand => '𐧞',
            MeroiticCursive::NumberFiveThousand => '𐧟',
            MeroiticCursive::NumberSixThousand => '𐧠',
            MeroiticCursive::NumberSevenThousand => '𐧡',
            MeroiticCursive::NumberEightThousand => '𐧢',
            MeroiticCursive::NumberNineThousand => '𐧣',
            MeroiticCursive::NumberTenThousand => '𐧤',
            MeroiticCursive::NumberTwentyThousand => '𐧥',
            MeroiticCursive::NumberThirtyThousand => '𐧦',
            MeroiticCursive::NumberFortyThousand => '𐧧',
            MeroiticCursive::NumberFiftyThousand => '𐧨',
            MeroiticCursive::NumberSixtyThousand => '𐧩',
            MeroiticCursive::NumberSeventyThousand => '𐧪',
            MeroiticCursive::NumberEightyThousand => '𐧫',
            MeroiticCursive::NumberNinetyThousand => '𐧬',
            MeroiticCursive::NumberOneHundredThousand => '𐧭',
            MeroiticCursive::NumberTwoHundredThousand => '𐧮',
            MeroiticCursive::NumberThreeHundredThousand => '𐧯',
            MeroiticCursive::NumberFourHundredThousand => '𐧰',
            MeroiticCursive::NumberFiveHundredThousand => '𐧱',
            MeroiticCursive::NumberSixHundredThousand => '𐧲',
            MeroiticCursive::NumberSevenHundredThousand => '𐧳',
            MeroiticCursive::NumberEightHundredThousand => '𐧴',
            MeroiticCursive::NumberNineHundredThousand => '𐧵',
            MeroiticCursive::FractionOneTwelfth => '𐧶',
            MeroiticCursive::FractionTwoTwelfths => '𐧷',
            MeroiticCursive::FractionThreeTwelfths => '𐧸',
            MeroiticCursive::FractionFourTwelfths => '𐧹',
            MeroiticCursive::FractionFiveTwelfths => '𐧺',
            MeroiticCursive::FractionSixTwelfths => '𐧻',
            MeroiticCursive::FractionSevenTwelfths => '𐧼',
            MeroiticCursive::FractionEightTwelfths => '𐧽',
            MeroiticCursive::FractionNineTwelfths => '𐧾',
        }
    }
}

impl std::convert::TryFrom<char> for MeroiticCursive {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𐦠' => Ok(MeroiticCursive::LetterA),
            '𐦡' => Ok(MeroiticCursive::LetterE),
            '𐦢' => Ok(MeroiticCursive::LetterI),
            '𐦣' => Ok(MeroiticCursive::LetterO),
            '𐦤' => Ok(MeroiticCursive::LetterYa),
            '𐦥' => Ok(MeroiticCursive::LetterWa),
            '𐦦' => Ok(MeroiticCursive::LetterBa),
            '𐦧' => Ok(MeroiticCursive::LetterPa),
            '𐦨' => Ok(MeroiticCursive::LetterMa),
            '𐦩' => Ok(MeroiticCursive::LetterNa),
            '𐦪' => Ok(MeroiticCursive::LetterNe),
            '𐦫' => Ok(MeroiticCursive::LetterRa),
            '𐦬' => Ok(MeroiticCursive::LetterLa),
            '𐦭' => Ok(MeroiticCursive::LetterKha),
            '𐦮' => Ok(MeroiticCursive::LetterHha),
            '𐦯' => Ok(MeroiticCursive::LetterSa),
            '𐦰' => Ok(MeroiticCursive::LetterArchaicSa),
            '𐦱' => Ok(MeroiticCursive::LetterSe),
            '𐦲' => Ok(MeroiticCursive::LetterKa),
            '𐦳' => Ok(MeroiticCursive::LetterQa),
            '𐦴' => Ok(MeroiticCursive::LetterTa),
            '𐦵' => Ok(MeroiticCursive::LetterTe),
            '𐦶' => Ok(MeroiticCursive::LetterTo),
            '𐦷' => Ok(MeroiticCursive::LetterDa),
            '𐦼' => Ok(MeroiticCursive::FractionElevenTwelfths),
            '𐦽' => Ok(MeroiticCursive::FractionOneHalf),
            '𐦾' => Ok(MeroiticCursive::LogogramRmt),
            '𐦿' => Ok(MeroiticCursive::LogogramImn),
            '𐧀' => Ok(MeroiticCursive::NumberOne),
            '𐧁' => Ok(MeroiticCursive::NumberTwo),
            '𐧂' => Ok(MeroiticCursive::NumberThree),
            '𐧃' => Ok(MeroiticCursive::NumberFour),
            '𐧄' => Ok(MeroiticCursive::NumberFive),
            '𐧅' => Ok(MeroiticCursive::NumberSix),
            '𐧆' => Ok(MeroiticCursive::NumberSeven),
            '𐧇' => Ok(MeroiticCursive::NumberEight),
            '𐧈' => Ok(MeroiticCursive::NumberNine),
            '𐧉' => Ok(MeroiticCursive::NumberTen),
            '𐧊' => Ok(MeroiticCursive::NumberTwenty),
            '𐧋' => Ok(MeroiticCursive::NumberThirty),
            '𐧌' => Ok(MeroiticCursive::NumberForty),
            '𐧍' => Ok(MeroiticCursive::NumberFifty),
            '𐧎' => Ok(MeroiticCursive::NumberSixty),
            '𐧏' => Ok(MeroiticCursive::NumberSeventy),
            '𐧒' => Ok(MeroiticCursive::NumberOneHundred),
            '𐧓' => Ok(MeroiticCursive::NumberTwoHundred),
            '𐧔' => Ok(MeroiticCursive::NumberThreeHundred),
            '𐧕' => Ok(MeroiticCursive::NumberFourHundred),
            '𐧖' => Ok(MeroiticCursive::NumberFiveHundred),
            '𐧗' => Ok(MeroiticCursive::NumberSixHundred),
            '𐧘' => Ok(MeroiticCursive::NumberSevenHundred),
            '𐧙' => Ok(MeroiticCursive::NumberEightHundred),
            '𐧚' => Ok(MeroiticCursive::NumberNineHundred),
            '𐧛' => Ok(MeroiticCursive::NumberOneThousand),
            '𐧜' => Ok(MeroiticCursive::NumberTwoThousand),
            '𐧝' => Ok(MeroiticCursive::NumberThreeThousand),
            '𐧞' => Ok(MeroiticCursive::NumberFourThousand),
            '𐧟' => Ok(MeroiticCursive::NumberFiveThousand),
            '𐧠' => Ok(MeroiticCursive::NumberSixThousand),
            '𐧡' => Ok(MeroiticCursive::NumberSevenThousand),
            '𐧢' => Ok(MeroiticCursive::NumberEightThousand),
            '𐧣' => Ok(MeroiticCursive::NumberNineThousand),
            '𐧤' => Ok(MeroiticCursive::NumberTenThousand),
            '𐧥' => Ok(MeroiticCursive::NumberTwentyThousand),
            '𐧦' => Ok(MeroiticCursive::NumberThirtyThousand),
            '𐧧' => Ok(MeroiticCursive::NumberFortyThousand),
            '𐧨' => Ok(MeroiticCursive::NumberFiftyThousand),
            '𐧩' => Ok(MeroiticCursive::NumberSixtyThousand),
            '𐧪' => Ok(MeroiticCursive::NumberSeventyThousand),
            '𐧫' => Ok(MeroiticCursive::NumberEightyThousand),
            '𐧬' => Ok(MeroiticCursive::NumberNinetyThousand),
            '𐧭' => Ok(MeroiticCursive::NumberOneHundredThousand),
            '𐧮' => Ok(MeroiticCursive::NumberTwoHundredThousand),
            '𐧯' => Ok(MeroiticCursive::NumberThreeHundredThousand),
            '𐧰' => Ok(MeroiticCursive::NumberFourHundredThousand),
            '𐧱' => Ok(MeroiticCursive::NumberFiveHundredThousand),
            '𐧲' => Ok(MeroiticCursive::NumberSixHundredThousand),
            '𐧳' => Ok(MeroiticCursive::NumberSevenHundredThousand),
            '𐧴' => Ok(MeroiticCursive::NumberEightHundredThousand),
            '𐧵' => Ok(MeroiticCursive::NumberNineHundredThousand),
            '𐧶' => Ok(MeroiticCursive::FractionOneTwelfth),
            '𐧷' => Ok(MeroiticCursive::FractionTwoTwelfths),
            '𐧸' => Ok(MeroiticCursive::FractionThreeTwelfths),
            '𐧹' => Ok(MeroiticCursive::FractionFourTwelfths),
            '𐧺' => Ok(MeroiticCursive::FractionFiveTwelfths),
            '𐧻' => Ok(MeroiticCursive::FractionSixTwelfths),
            '𐧼' => Ok(MeroiticCursive::FractionSevenTwelfths),
            '𐧽' => Ok(MeroiticCursive::FractionEightTwelfths),
            '𐧾' => Ok(MeroiticCursive::FractionNineTwelfths),
            _ => Err(()),
        }
    }
}

impl Into<u32> for MeroiticCursive {
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

impl std::convert::TryFrom<u32> for MeroiticCursive {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for MeroiticCursive {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl MeroiticCursive {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        MeroiticCursive::LetterA
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("MeroiticCursive{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
