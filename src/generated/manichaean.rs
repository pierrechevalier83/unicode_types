/// \u{10ac0} → \u{10aff}\
///\
/// 𐫀 𐫁 𐫂 𐫃 𐫄 𐫅 𐫆 𐫇 𐫈 𐫉 𐫊 𐫋 𐫌 𐫍 𐫎 𐫏\
/// 𐫐 𐫑 𐫒 𐫓 𐫔 𐫕 𐫖 𐫗 𐫘 𐫙 𐫚 𐫛 𐫜 𐫝 𐫞 𐫟\
/// 𐫠 𐫡 𐫢 𐫣 𐫤 𐫥 𐫦 𐫫 𐫬 𐫭 𐫮 𐫯 𐫰 𐫱 𐫲 𐫳\
/// 𐫴 𐫵 𐫶\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{10ac0}: '𐫀'
    pub const LETTER_ALEPH: char = '𐫀';
    /// \u{10ac1}: '𐫁'
    pub const LETTER_BETH: char = '𐫁';
    /// \u{10ac2}: '𐫂'
    pub const LETTER_BHETH: char = '𐫂';
    /// \u{10ac3}: '𐫃'
    pub const LETTER_GIMEL: char = '𐫃';
    /// \u{10ac4}: '𐫄'
    pub const LETTER_GHIMEL: char = '𐫄';
    /// \u{10ac5}: '𐫅'
    pub const LETTER_DALETH: char = '𐫅';
    /// \u{10ac6}: '𐫆'
    pub const LETTER_HE: char = '𐫆';
    /// \u{10ac7}: '𐫇'
    pub const LETTER_WAW: char = '𐫇';
    /// \u{10ac8}: '𐫈'
    pub const SIGN_UD: char = '𐫈';
    /// \u{10ac9}: '𐫉'
    pub const LETTER_ZAYIN: char = '𐫉';
    /// \u{10aca}: '𐫊'
    pub const LETTER_ZHAYIN: char = '𐫊';
    /// \u{10acb}: '𐫋'
    pub const LETTER_JAYIN: char = '𐫋';
    /// \u{10acc}: '𐫌'
    pub const LETTER_JHAYIN: char = '𐫌';
    /// \u{10acd}: '𐫍'
    pub const LETTER_HETH: char = '𐫍';
    /// \u{10ace}: '𐫎'
    pub const LETTER_TETH: char = '𐫎';
    /// \u{10acf}: '𐫏'
    pub const LETTER_YODH: char = '𐫏';
    /// \u{10ad0}: '𐫐'
    pub const LETTER_KAPH: char = '𐫐';
    /// \u{10ad1}: '𐫑'
    pub const LETTER_XAPH: char = '𐫑';
    /// \u{10ad2}: '𐫒'
    pub const LETTER_KHAPH: char = '𐫒';
    /// \u{10ad3}: '𐫓'
    pub const LETTER_LAMEDH: char = '𐫓';
    /// \u{10ad4}: '𐫔'
    pub const LETTER_DHAMEDH: char = '𐫔';
    /// \u{10ad5}: '𐫕'
    pub const LETTER_THAMEDH: char = '𐫕';
    /// \u{10ad6}: '𐫖'
    pub const LETTER_MEM: char = '𐫖';
    /// \u{10ad7}: '𐫗'
    pub const LETTER_NUN: char = '𐫗';
    /// \u{10ad8}: '𐫘'
    pub const LETTER_SAMEKH: char = '𐫘';
    /// \u{10ad9}: '𐫙'
    pub const LETTER_AYIN: char = '𐫙';
    /// \u{10ada}: '𐫚'
    pub const LETTER_AAYIN: char = '𐫚';
    /// \u{10adb}: '𐫛'
    pub const LETTER_PE: char = '𐫛';
    /// \u{10adc}: '𐫜'
    pub const LETTER_FE: char = '𐫜';
    /// \u{10add}: '𐫝'
    pub const LETTER_SADHE: char = '𐫝';
    /// \u{10ade}: '𐫞'
    pub const LETTER_QOPH: char = '𐫞';
    /// \u{10adf}: '𐫟'
    pub const LETTER_XOPH: char = '𐫟';
    /// \u{10ae0}: '𐫠'
    pub const LETTER_QHOPH: char = '𐫠';
    /// \u{10ae1}: '𐫡'
    pub const LETTER_RESH: char = '𐫡';
    /// \u{10ae2}: '𐫢'
    pub const LETTER_SHIN: char = '𐫢';
    /// \u{10ae3}: '𐫣'
    pub const LETTER_SSHIN: char = '𐫣';
    /// \u{10ae4}: '𐫤'
    pub const LETTER_TAW: char = '𐫤';
    /// \u{10ae5}: '𐫥'
    pub const ABBREVIATION_MARK_ABOVE: char = '𐫥';
    /// \u{10ae6}: '𐫦'
    pub const ABBREVIATION_MARK_BELOW: char = '𐫦';
    /// \u{10aeb}: '𐫫'
    pub const NUMBER_ONE: char = '𐫫';
    /// \u{10aec}: '𐫬'
    pub const NUMBER_FIVE: char = '𐫬';
    /// \u{10aed}: '𐫭'
    pub const NUMBER_TEN: char = '𐫭';
    /// \u{10aee}: '𐫮'
    pub const NUMBER_TWENTY: char = '𐫮';
    /// \u{10aef}: '𐫯'
    pub const NUMBER_ONE_HUNDRED: char = '𐫯';
    /// \u{10af0}: '𐫰'
    pub const PUNCTUATION_STAR: char = '𐫰';
    /// \u{10af1}: '𐫱'
    pub const PUNCTUATION_FLEURON: char = '𐫱';
    /// \u{10af2}: '𐫲'
    pub const PUNCTUATION_DOUBLE_DOT_WITHIN_DOT: char = '𐫲';
    /// \u{10af3}: '𐫳'
    pub const PUNCTUATION_DOT_WITHIN_DOT: char = '𐫳';
    /// \u{10af4}: '𐫴'
    pub const PUNCTUATION_DOT: char = '𐫴';
    /// \u{10af5}: '𐫵'
    pub const PUNCTUATION_TWO_DOTS: char = '𐫵';
    /// \u{10af6}: '𐫶'
    pub const PUNCTUATION_LINE_FILLER: char = '𐫶';
}

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
        use constants::*;
        match self {
            Manichaean::LetterAleph => LETTER_ALEPH,
            Manichaean::LetterBeth => LETTER_BETH,
            Manichaean::LetterBheth => LETTER_BHETH,
            Manichaean::LetterGimel => LETTER_GIMEL,
            Manichaean::LetterGhimel => LETTER_GHIMEL,
            Manichaean::LetterDaleth => LETTER_DALETH,
            Manichaean::LetterHe => LETTER_HE,
            Manichaean::LetterWaw => LETTER_WAW,
            Manichaean::SignUd => SIGN_UD,
            Manichaean::LetterZayin => LETTER_ZAYIN,
            Manichaean::LetterZhayin => LETTER_ZHAYIN,
            Manichaean::LetterJayin => LETTER_JAYIN,
            Manichaean::LetterJhayin => LETTER_JHAYIN,
            Manichaean::LetterHeth => LETTER_HETH,
            Manichaean::LetterTeth => LETTER_TETH,
            Manichaean::LetterYodh => LETTER_YODH,
            Manichaean::LetterKaph => LETTER_KAPH,
            Manichaean::LetterXaph => LETTER_XAPH,
            Manichaean::LetterKhaph => LETTER_KHAPH,
            Manichaean::LetterLamedh => LETTER_LAMEDH,
            Manichaean::LetterDhamedh => LETTER_DHAMEDH,
            Manichaean::LetterThamedh => LETTER_THAMEDH,
            Manichaean::LetterMem => LETTER_MEM,
            Manichaean::LetterNun => LETTER_NUN,
            Manichaean::LetterSamekh => LETTER_SAMEKH,
            Manichaean::LetterAyin => LETTER_AYIN,
            Manichaean::LetterAayin => LETTER_AAYIN,
            Manichaean::LetterPe => LETTER_PE,
            Manichaean::LetterFe => LETTER_FE,
            Manichaean::LetterSadhe => LETTER_SADHE,
            Manichaean::LetterQoph => LETTER_QOPH,
            Manichaean::LetterXoph => LETTER_XOPH,
            Manichaean::LetterQhoph => LETTER_QHOPH,
            Manichaean::LetterResh => LETTER_RESH,
            Manichaean::LetterShin => LETTER_SHIN,
            Manichaean::LetterSshin => LETTER_SSHIN,
            Manichaean::LetterTaw => LETTER_TAW,
            Manichaean::AbbreviationMarkAbove => ABBREVIATION_MARK_ABOVE,
            Manichaean::AbbreviationMarkBelow => ABBREVIATION_MARK_BELOW,
            Manichaean::NumberOne => NUMBER_ONE,
            Manichaean::NumberFive => NUMBER_FIVE,
            Manichaean::NumberTen => NUMBER_TEN,
            Manichaean::NumberTwenty => NUMBER_TWENTY,
            Manichaean::NumberOneHundred => NUMBER_ONE_HUNDRED,
            Manichaean::PunctuationStar => PUNCTUATION_STAR,
            Manichaean::PunctuationFleuron => PUNCTUATION_FLEURON,
            Manichaean::PunctuationDoubleDotWithinDot => PUNCTUATION_DOUBLE_DOT_WITHIN_DOT,
            Manichaean::PunctuationDotWithinDot => PUNCTUATION_DOT_WITHIN_DOT,
            Manichaean::PunctuationDot => PUNCTUATION_DOT,
            Manichaean::PunctuationTwoDots => PUNCTUATION_TWO_DOTS,
            Manichaean::PunctuationLineFiller => PUNCTUATION_LINE_FILLER,
        }
    }
}

impl std::convert::TryFrom<char> for Manichaean {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_ALEPH => Ok(Manichaean::LetterAleph),
            LETTER_BETH => Ok(Manichaean::LetterBeth),
            LETTER_BHETH => Ok(Manichaean::LetterBheth),
            LETTER_GIMEL => Ok(Manichaean::LetterGimel),
            LETTER_GHIMEL => Ok(Manichaean::LetterGhimel),
            LETTER_DALETH => Ok(Manichaean::LetterDaleth),
            LETTER_HE => Ok(Manichaean::LetterHe),
            LETTER_WAW => Ok(Manichaean::LetterWaw),
            SIGN_UD => Ok(Manichaean::SignUd),
            LETTER_ZAYIN => Ok(Manichaean::LetterZayin),
            LETTER_ZHAYIN => Ok(Manichaean::LetterZhayin),
            LETTER_JAYIN => Ok(Manichaean::LetterJayin),
            LETTER_JHAYIN => Ok(Manichaean::LetterJhayin),
            LETTER_HETH => Ok(Manichaean::LetterHeth),
            LETTER_TETH => Ok(Manichaean::LetterTeth),
            LETTER_YODH => Ok(Manichaean::LetterYodh),
            LETTER_KAPH => Ok(Manichaean::LetterKaph),
            LETTER_XAPH => Ok(Manichaean::LetterXaph),
            LETTER_KHAPH => Ok(Manichaean::LetterKhaph),
            LETTER_LAMEDH => Ok(Manichaean::LetterLamedh),
            LETTER_DHAMEDH => Ok(Manichaean::LetterDhamedh),
            LETTER_THAMEDH => Ok(Manichaean::LetterThamedh),
            LETTER_MEM => Ok(Manichaean::LetterMem),
            LETTER_NUN => Ok(Manichaean::LetterNun),
            LETTER_SAMEKH => Ok(Manichaean::LetterSamekh),
            LETTER_AYIN => Ok(Manichaean::LetterAyin),
            LETTER_AAYIN => Ok(Manichaean::LetterAayin),
            LETTER_PE => Ok(Manichaean::LetterPe),
            LETTER_FE => Ok(Manichaean::LetterFe),
            LETTER_SADHE => Ok(Manichaean::LetterSadhe),
            LETTER_QOPH => Ok(Manichaean::LetterQoph),
            LETTER_XOPH => Ok(Manichaean::LetterXoph),
            LETTER_QHOPH => Ok(Manichaean::LetterQhoph),
            LETTER_RESH => Ok(Manichaean::LetterResh),
            LETTER_SHIN => Ok(Manichaean::LetterShin),
            LETTER_SSHIN => Ok(Manichaean::LetterSshin),
            LETTER_TAW => Ok(Manichaean::LetterTaw),
            ABBREVIATION_MARK_ABOVE => Ok(Manichaean::AbbreviationMarkAbove),
            ABBREVIATION_MARK_BELOW => Ok(Manichaean::AbbreviationMarkBelow),
            NUMBER_ONE => Ok(Manichaean::NumberOne),
            NUMBER_FIVE => Ok(Manichaean::NumberFive),
            NUMBER_TEN => Ok(Manichaean::NumberTen),
            NUMBER_TWENTY => Ok(Manichaean::NumberTwenty),
            NUMBER_ONE_HUNDRED => Ok(Manichaean::NumberOneHundred),
            PUNCTUATION_STAR => Ok(Manichaean::PunctuationStar),
            PUNCTUATION_FLEURON => Ok(Manichaean::PunctuationFleuron),
            PUNCTUATION_DOUBLE_DOT_WITHIN_DOT => Ok(Manichaean::PunctuationDoubleDotWithinDot),
            PUNCTUATION_DOT_WITHIN_DOT => Ok(Manichaean::PunctuationDotWithinDot),
            PUNCTUATION_DOT => Ok(Manichaean::PunctuationDot),
            PUNCTUATION_TWO_DOTS => Ok(Manichaean::PunctuationTwoDots),
            PUNCTUATION_LINE_FILLER => Ok(Manichaean::PunctuationLineFiller),
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

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Manichaean::LetterAleph => "manichaean letter aleph",
            Manichaean::LetterBeth => "manichaean letter beth",
            Manichaean::LetterBheth => "manichaean letter bheth",
            Manichaean::LetterGimel => "manichaean letter gimel",
            Manichaean::LetterGhimel => "manichaean letter ghimel",
            Manichaean::LetterDaleth => "manichaean letter daleth",
            Manichaean::LetterHe => "manichaean letter he",
            Manichaean::LetterWaw => "manichaean letter waw",
            Manichaean::SignUd => "manichaean sign ud",
            Manichaean::LetterZayin => "manichaean letter zayin",
            Manichaean::LetterZhayin => "manichaean letter zhayin",
            Manichaean::LetterJayin => "manichaean letter jayin",
            Manichaean::LetterJhayin => "manichaean letter jhayin",
            Manichaean::LetterHeth => "manichaean letter heth",
            Manichaean::LetterTeth => "manichaean letter teth",
            Manichaean::LetterYodh => "manichaean letter yodh",
            Manichaean::LetterKaph => "manichaean letter kaph",
            Manichaean::LetterXaph => "manichaean letter xaph",
            Manichaean::LetterKhaph => "manichaean letter khaph",
            Manichaean::LetterLamedh => "manichaean letter lamedh",
            Manichaean::LetterDhamedh => "manichaean letter dhamedh",
            Manichaean::LetterThamedh => "manichaean letter thamedh",
            Manichaean::LetterMem => "manichaean letter mem",
            Manichaean::LetterNun => "manichaean letter nun",
            Manichaean::LetterSamekh => "manichaean letter samekh",
            Manichaean::LetterAyin => "manichaean letter ayin",
            Manichaean::LetterAayin => "manichaean letter aayin",
            Manichaean::LetterPe => "manichaean letter pe",
            Manichaean::LetterFe => "manichaean letter fe",
            Manichaean::LetterSadhe => "manichaean letter sadhe",
            Manichaean::LetterQoph => "manichaean letter qoph",
            Manichaean::LetterXoph => "manichaean letter xoph",
            Manichaean::LetterQhoph => "manichaean letter qhoph",
            Manichaean::LetterResh => "manichaean letter resh",
            Manichaean::LetterShin => "manichaean letter shin",
            Manichaean::LetterSshin => "manichaean letter sshin",
            Manichaean::LetterTaw => "manichaean letter taw",
            Manichaean::AbbreviationMarkAbove => "manichaean abbreviation mark above",
            Manichaean::AbbreviationMarkBelow => "manichaean abbreviation mark below",
            Manichaean::NumberOne => "manichaean number one",
            Manichaean::NumberFive => "manichaean number five",
            Manichaean::NumberTen => "manichaean number ten",
            Manichaean::NumberTwenty => "manichaean number twenty",
            Manichaean::NumberOneHundred => "manichaean number one hundred",
            Manichaean::PunctuationStar => "manichaean punctuation star",
            Manichaean::PunctuationFleuron => "manichaean punctuation fleuron",
            Manichaean::PunctuationDoubleDotWithinDot => "manichaean punctuation double dot within dot",
            Manichaean::PunctuationDotWithinDot => "manichaean punctuation dot within dot",
            Manichaean::PunctuationDot => "manichaean punctuation dot",
            Manichaean::PunctuationTwoDots => "manichaean punctuation two dots",
            Manichaean::PunctuationLineFiller => "manichaean punctuation line filler",
        }
    }
}
