
/// An enum to represent all characters in the Manichaean block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Manichaean {
    /// \u{10ac0}: '𐫀'
    LetterAleph,
    /// \u{10ac1}: '𐫁'
    LetterBeth,
    /// \u{10ac2}: '𐫂'
    LetterBheth,
    /// \u{10ac3}: '𐫃'
    LetterGimel,
    /// \u{10ac4}: '𐫄'
    LetterGhimel,
    /// \u{10ac5}: '𐫅'
    LetterDaleth,
    /// \u{10ac6}: '𐫆'
    LetterHe,
    /// \u{10ac7}: '𐫇'
    LetterWaw,
    /// \u{10ac8}: '𐫈'
    SignUd,
    /// \u{10ac9}: '𐫉'
    LetterZayin,
    /// \u{10aca}: '𐫊'
    LetterZhayin,
    /// \u{10acb}: '𐫋'
    LetterJayin,
    /// \u{10acc}: '𐫌'
    LetterJhayin,
    /// \u{10acd}: '𐫍'
    LetterHeth,
    /// \u{10ace}: '𐫎'
    LetterTeth,
    /// \u{10acf}: '𐫏'
    LetterYodh,
    /// \u{10ad0}: '𐫐'
    LetterKaph,
    /// \u{10ad1}: '𐫑'
    LetterXaph,
    /// \u{10ad2}: '𐫒'
    LetterKhaph,
    /// \u{10ad3}: '𐫓'
    LetterLamedh,
    /// \u{10ad4}: '𐫔'
    LetterDhamedh,
    /// \u{10ad5}: '𐫕'
    LetterThamedh,
    /// \u{10ad6}: '𐫖'
    LetterMem,
    /// \u{10ad7}: '𐫗'
    LetterNun,
    /// \u{10ad8}: '𐫘'
    LetterSamekh,
    /// \u{10ad9}: '𐫙'
    LetterAyin,
    /// \u{10ada}: '𐫚'
    LetterAayin,
    /// \u{10adb}: '𐫛'
    LetterPe,
    /// \u{10adc}: '𐫜'
    LetterFe,
    /// \u{10add}: '𐫝'
    LetterSadhe,
    /// \u{10ade}: '𐫞'
    LetterQoph,
    /// \u{10adf}: '𐫟'
    LetterXoph,
    /// \u{10ae0}: '𐫠'
    LetterQhoph,
    /// \u{10ae1}: '𐫡'
    LetterResh,
    /// \u{10ae2}: '𐫢'
    LetterShin,
    /// \u{10ae3}: '𐫣'
    LetterSshin,
    /// \u{10ae4}: '𐫤'
    LetterTaw,
    /// \u{10ae5}: '𐫥'
    AbbreviationMarkAbove,
    /// \u{10ae6}: '𐫦'
    AbbreviationMarkBelow,
    /// \u{10aeb}: '𐫫'
    NumberOne,
    /// \u{10aec}: '𐫬'
    NumberFive,
    /// \u{10aed}: '𐫭'
    NumberTen,
    /// \u{10aee}: '𐫮'
    NumberTwenty,
    /// \u{10aef}: '𐫯'
    NumberOneHundred,
    /// \u{10af0}: '𐫰'
    PunctuationStar,
    /// \u{10af1}: '𐫱'
    PunctuationFleuron,
    /// \u{10af2}: '𐫲'
    PunctuationDoubleDotWithinDot,
    /// \u{10af3}: '𐫳'
    PunctuationDotWithinDot,
    /// \u{10af4}: '𐫴'
    PunctuationDot,
    /// \u{10af5}: '𐫵'
    PunctuationTwoDots,
    /// \u{10af6}: '𐫶'
    PunctuationLineFiller,
}

impl Into<char> for Manichaean {
    fn into(self) -> char {
        match self {
            Manichaean::LetterAleph => '𐫀',
            Manichaean::LetterBeth => '𐫁',
            Manichaean::LetterBheth => '𐫂',
            Manichaean::LetterGimel => '𐫃',
            Manichaean::LetterGhimel => '𐫄',
            Manichaean::LetterDaleth => '𐫅',
            Manichaean::LetterHe => '𐫆',
            Manichaean::LetterWaw => '𐫇',
            Manichaean::SignUd => '𐫈',
            Manichaean::LetterZayin => '𐫉',
            Manichaean::LetterZhayin => '𐫊',
            Manichaean::LetterJayin => '𐫋',
            Manichaean::LetterJhayin => '𐫌',
            Manichaean::LetterHeth => '𐫍',
            Manichaean::LetterTeth => '𐫎',
            Manichaean::LetterYodh => '𐫏',
            Manichaean::LetterKaph => '𐫐',
            Manichaean::LetterXaph => '𐫑',
            Manichaean::LetterKhaph => '𐫒',
            Manichaean::LetterLamedh => '𐫓',
            Manichaean::LetterDhamedh => '𐫔',
            Manichaean::LetterThamedh => '𐫕',
            Manichaean::LetterMem => '𐫖',
            Manichaean::LetterNun => '𐫗',
            Manichaean::LetterSamekh => '𐫘',
            Manichaean::LetterAyin => '𐫙',
            Manichaean::LetterAayin => '𐫚',
            Manichaean::LetterPe => '𐫛',
            Manichaean::LetterFe => '𐫜',
            Manichaean::LetterSadhe => '𐫝',
            Manichaean::LetterQoph => '𐫞',
            Manichaean::LetterXoph => '𐫟',
            Manichaean::LetterQhoph => '𐫠',
            Manichaean::LetterResh => '𐫡',
            Manichaean::LetterShin => '𐫢',
            Manichaean::LetterSshin => '𐫣',
            Manichaean::LetterTaw => '𐫤',
            Manichaean::AbbreviationMarkAbove => '𐫥',
            Manichaean::AbbreviationMarkBelow => '𐫦',
            Manichaean::NumberOne => '𐫫',
            Manichaean::NumberFive => '𐫬',
            Manichaean::NumberTen => '𐫭',
            Manichaean::NumberTwenty => '𐫮',
            Manichaean::NumberOneHundred => '𐫯',
            Manichaean::PunctuationStar => '𐫰',
            Manichaean::PunctuationFleuron => '𐫱',
            Manichaean::PunctuationDoubleDotWithinDot => '𐫲',
            Manichaean::PunctuationDotWithinDot => '𐫳',
            Manichaean::PunctuationDot => '𐫴',
            Manichaean::PunctuationTwoDots => '𐫵',
            Manichaean::PunctuationLineFiller => '𐫶',
        }
    }
}

impl std::convert::TryFrom<char> for Manichaean {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𐫀' => Ok(Manichaean::LetterAleph),
            '𐫁' => Ok(Manichaean::LetterBeth),
            '𐫂' => Ok(Manichaean::LetterBheth),
            '𐫃' => Ok(Manichaean::LetterGimel),
            '𐫄' => Ok(Manichaean::LetterGhimel),
            '𐫅' => Ok(Manichaean::LetterDaleth),
            '𐫆' => Ok(Manichaean::LetterHe),
            '𐫇' => Ok(Manichaean::LetterWaw),
            '𐫈' => Ok(Manichaean::SignUd),
            '𐫉' => Ok(Manichaean::LetterZayin),
            '𐫊' => Ok(Manichaean::LetterZhayin),
            '𐫋' => Ok(Manichaean::LetterJayin),
            '𐫌' => Ok(Manichaean::LetterJhayin),
            '𐫍' => Ok(Manichaean::LetterHeth),
            '𐫎' => Ok(Manichaean::LetterTeth),
            '𐫏' => Ok(Manichaean::LetterYodh),
            '𐫐' => Ok(Manichaean::LetterKaph),
            '𐫑' => Ok(Manichaean::LetterXaph),
            '𐫒' => Ok(Manichaean::LetterKhaph),
            '𐫓' => Ok(Manichaean::LetterLamedh),
            '𐫔' => Ok(Manichaean::LetterDhamedh),
            '𐫕' => Ok(Manichaean::LetterThamedh),
            '𐫖' => Ok(Manichaean::LetterMem),
            '𐫗' => Ok(Manichaean::LetterNun),
            '𐫘' => Ok(Manichaean::LetterSamekh),
            '𐫙' => Ok(Manichaean::LetterAyin),
            '𐫚' => Ok(Manichaean::LetterAayin),
            '𐫛' => Ok(Manichaean::LetterPe),
            '𐫜' => Ok(Manichaean::LetterFe),
            '𐫝' => Ok(Manichaean::LetterSadhe),
            '𐫞' => Ok(Manichaean::LetterQoph),
            '𐫟' => Ok(Manichaean::LetterXoph),
            '𐫠' => Ok(Manichaean::LetterQhoph),
            '𐫡' => Ok(Manichaean::LetterResh),
            '𐫢' => Ok(Manichaean::LetterShin),
            '𐫣' => Ok(Manichaean::LetterSshin),
            '𐫤' => Ok(Manichaean::LetterTaw),
            '𐫥' => Ok(Manichaean::AbbreviationMarkAbove),
            '𐫦' => Ok(Manichaean::AbbreviationMarkBelow),
            '𐫫' => Ok(Manichaean::NumberOne),
            '𐫬' => Ok(Manichaean::NumberFive),
            '𐫭' => Ok(Manichaean::NumberTen),
            '𐫮' => Ok(Manichaean::NumberTwenty),
            '𐫯' => Ok(Manichaean::NumberOneHundred),
            '𐫰' => Ok(Manichaean::PunctuationStar),
            '𐫱' => Ok(Manichaean::PunctuationFleuron),
            '𐫲' => Ok(Manichaean::PunctuationDoubleDotWithinDot),
            '𐫳' => Ok(Manichaean::PunctuationDotWithinDot),
            '𐫴' => Ok(Manichaean::PunctuationDot),
            '𐫵' => Ok(Manichaean::PunctuationTwoDots),
            '𐫶' => Ok(Manichaean::PunctuationLineFiller),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Manichaean {
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

impl std::convert::TryFrom<u32> for Manichaean {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Manichaean {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Manichaean {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Manichaean::LetterAleph
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Manichaean{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
