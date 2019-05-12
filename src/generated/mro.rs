/// \u{16a40} → \u{16a6f}\
///\
/// 𖩀 𖩁 𖩂 𖩃 𖩄 𖩅 𖩆 𖩇 𖩈 𖩉 𖩊 𖩋 𖩌 𖩍 𖩎 𖩏\
/// 𖩐 𖩑 𖩒 𖩓 𖩔 𖩕 𖩖 𖩗 𖩘 𖩙 𖩚 𖩛 𖩜 𖩝 𖩞 𖩠\
/// 𖩡 𖩢 𖩣 𖩤 𖩥 𖩦 𖩧 𖩨 𖩩 𖩮\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{16a40}: '𖩀'
    pub const LETTER_TA: char = '𖩀';
    /// \u{16a41}: '𖩁'
    pub const LETTER_NGI: char = '𖩁';
    /// \u{16a42}: '𖩂'
    pub const LETTER_YO: char = '𖩂';
    /// \u{16a43}: '𖩃'
    pub const LETTER_MIM: char = '𖩃';
    /// \u{16a44}: '𖩄'
    pub const LETTER_BA: char = '𖩄';
    /// \u{16a45}: '𖩅'
    pub const LETTER_DA: char = '𖩅';
    /// \u{16a46}: '𖩆'
    pub const LETTER_A: char = '𖩆';
    /// \u{16a47}: '𖩇'
    pub const LETTER_PHI: char = '𖩇';
    /// \u{16a48}: '𖩈'
    pub const LETTER_KHAI: char = '𖩈';
    /// \u{16a49}: '𖩉'
    pub const LETTER_HAO: char = '𖩉';
    /// \u{16a4a}: '𖩊'
    pub const LETTER_DAI: char = '𖩊';
    /// \u{16a4b}: '𖩋'
    pub const LETTER_CHU: char = '𖩋';
    /// \u{16a4c}: '𖩌'
    pub const LETTER_KEAAE: char = '𖩌';
    /// \u{16a4d}: '𖩍'
    pub const LETTER_OL: char = '𖩍';
    /// \u{16a4e}: '𖩎'
    pub const LETTER_MAEM: char = '𖩎';
    /// \u{16a4f}: '𖩏'
    pub const LETTER_NIN: char = '𖩏';
    /// \u{16a50}: '𖩐'
    pub const LETTER_PA: char = '𖩐';
    /// \u{16a51}: '𖩑'
    pub const LETTER_OO: char = '𖩑';
    /// \u{16a52}: '𖩒'
    pub const LETTER_O: char = '𖩒';
    /// \u{16a53}: '𖩓'
    pub const LETTER_RO: char = '𖩓';
    /// \u{16a54}: '𖩔'
    pub const LETTER_SHI: char = '𖩔';
    /// \u{16a55}: '𖩕'
    pub const LETTER_THEA: char = '𖩕';
    /// \u{16a56}: '𖩖'
    pub const LETTER_EA: char = '𖩖';
    /// \u{16a57}: '𖩗'
    pub const LETTER_WA: char = '𖩗';
    /// \u{16a58}: '𖩘'
    pub const LETTER_E: char = '𖩘';
    /// \u{16a59}: '𖩙'
    pub const LETTER_KO: char = '𖩙';
    /// \u{16a5a}: '𖩚'
    pub const LETTER_LAN: char = '𖩚';
    /// \u{16a5b}: '𖩛'
    pub const LETTER_LA: char = '𖩛';
    /// \u{16a5c}: '𖩜'
    pub const LETTER_HAI: char = '𖩜';
    /// \u{16a5d}: '𖩝'
    pub const LETTER_RI: char = '𖩝';
    /// \u{16a5e}: '𖩞'
    pub const LETTER_TEK: char = '𖩞';
    /// \u{16a60}: '𖩠'
    pub const DIGIT_ZERO: char = '𖩠';
    /// \u{16a61}: '𖩡'
    pub const DIGIT_ONE: char = '𖩡';
    /// \u{16a62}: '𖩢'
    pub const DIGIT_TWO: char = '𖩢';
    /// \u{16a63}: '𖩣'
    pub const DIGIT_THREE: char = '𖩣';
    /// \u{16a64}: '𖩤'
    pub const DIGIT_FOUR: char = '𖩤';
    /// \u{16a65}: '𖩥'
    pub const DIGIT_FIVE: char = '𖩥';
    /// \u{16a66}: '𖩦'
    pub const DIGIT_SIX: char = '𖩦';
    /// \u{16a67}: '𖩧'
    pub const DIGIT_SEVEN: char = '𖩧';
    /// \u{16a68}: '𖩨'
    pub const DIGIT_EIGHT: char = '𖩨';
    /// \u{16a69}: '𖩩'
    pub const DIGIT_NINE: char = '𖩩';
    /// \u{16a6e}: '𖩮'
    pub const DANDA: char = '𖩮';
}

/// An enum to represent all characters in the Mro block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Mro {
    /// \u{16a40}: '𖩀'
    LetterTa,
    /// \u{16a41}: '𖩁'
    LetterNgi,
    /// \u{16a42}: '𖩂'
    LetterYo,
    /// \u{16a43}: '𖩃'
    LetterMim,
    /// \u{16a44}: '𖩄'
    LetterBa,
    /// \u{16a45}: '𖩅'
    LetterDa,
    /// \u{16a46}: '𖩆'
    LetterA,
    /// \u{16a47}: '𖩇'
    LetterPhi,
    /// \u{16a48}: '𖩈'
    LetterKhai,
    /// \u{16a49}: '𖩉'
    LetterHao,
    /// \u{16a4a}: '𖩊'
    LetterDai,
    /// \u{16a4b}: '𖩋'
    LetterChu,
    /// \u{16a4c}: '𖩌'
    LetterKeaae,
    /// \u{16a4d}: '𖩍'
    LetterOl,
    /// \u{16a4e}: '𖩎'
    LetterMaem,
    /// \u{16a4f}: '𖩏'
    LetterNin,
    /// \u{16a50}: '𖩐'
    LetterPa,
    /// \u{16a51}: '𖩑'
    LetterOo,
    /// \u{16a52}: '𖩒'
    LetterO,
    /// \u{16a53}: '𖩓'
    LetterRo,
    /// \u{16a54}: '𖩔'
    LetterShi,
    /// \u{16a55}: '𖩕'
    LetterThea,
    /// \u{16a56}: '𖩖'
    LetterEa,
    /// \u{16a57}: '𖩗'
    LetterWa,
    /// \u{16a58}: '𖩘'
    LetterE,
    /// \u{16a59}: '𖩙'
    LetterKo,
    /// \u{16a5a}: '𖩚'
    LetterLan,
    /// \u{16a5b}: '𖩛'
    LetterLa,
    /// \u{16a5c}: '𖩜'
    LetterHai,
    /// \u{16a5d}: '𖩝'
    LetterRi,
    /// \u{16a5e}: '𖩞'
    LetterTek,
    /// \u{16a60}: '𖩠'
    DigitZero,
    /// \u{16a61}: '𖩡'
    DigitOne,
    /// \u{16a62}: '𖩢'
    DigitTwo,
    /// \u{16a63}: '𖩣'
    DigitThree,
    /// \u{16a64}: '𖩤'
    DigitFour,
    /// \u{16a65}: '𖩥'
    DigitFive,
    /// \u{16a66}: '𖩦'
    DigitSix,
    /// \u{16a67}: '𖩧'
    DigitSeven,
    /// \u{16a68}: '𖩨'
    DigitEight,
    /// \u{16a69}: '𖩩'
    DigitNine,
    /// \u{16a6e}: '𖩮'
    Danda,
}

impl Into<char> for Mro {
    fn into(self) -> char {
        use constants::*;
        match self {
            Mro::LetterTa => LETTER_TA,
            Mro::LetterNgi => LETTER_NGI,
            Mro::LetterYo => LETTER_YO,
            Mro::LetterMim => LETTER_MIM,
            Mro::LetterBa => LETTER_BA,
            Mro::LetterDa => LETTER_DA,
            Mro::LetterA => LETTER_A,
            Mro::LetterPhi => LETTER_PHI,
            Mro::LetterKhai => LETTER_KHAI,
            Mro::LetterHao => LETTER_HAO,
            Mro::LetterDai => LETTER_DAI,
            Mro::LetterChu => LETTER_CHU,
            Mro::LetterKeaae => LETTER_KEAAE,
            Mro::LetterOl => LETTER_OL,
            Mro::LetterMaem => LETTER_MAEM,
            Mro::LetterNin => LETTER_NIN,
            Mro::LetterPa => LETTER_PA,
            Mro::LetterOo => LETTER_OO,
            Mro::LetterO => LETTER_O,
            Mro::LetterRo => LETTER_RO,
            Mro::LetterShi => LETTER_SHI,
            Mro::LetterThea => LETTER_THEA,
            Mro::LetterEa => LETTER_EA,
            Mro::LetterWa => LETTER_WA,
            Mro::LetterE => LETTER_E,
            Mro::LetterKo => LETTER_KO,
            Mro::LetterLan => LETTER_LAN,
            Mro::LetterLa => LETTER_LA,
            Mro::LetterHai => LETTER_HAI,
            Mro::LetterRi => LETTER_RI,
            Mro::LetterTek => LETTER_TEK,
            Mro::DigitZero => DIGIT_ZERO,
            Mro::DigitOne => DIGIT_ONE,
            Mro::DigitTwo => DIGIT_TWO,
            Mro::DigitThree => DIGIT_THREE,
            Mro::DigitFour => DIGIT_FOUR,
            Mro::DigitFive => DIGIT_FIVE,
            Mro::DigitSix => DIGIT_SIX,
            Mro::DigitSeven => DIGIT_SEVEN,
            Mro::DigitEight => DIGIT_EIGHT,
            Mro::DigitNine => DIGIT_NINE,
            Mro::Danda => DANDA,
        }
    }
}

impl std::convert::TryFrom<char> for Mro {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_TA => Ok(Mro::LetterTa),
            LETTER_NGI => Ok(Mro::LetterNgi),
            LETTER_YO => Ok(Mro::LetterYo),
            LETTER_MIM => Ok(Mro::LetterMim),
            LETTER_BA => Ok(Mro::LetterBa),
            LETTER_DA => Ok(Mro::LetterDa),
            LETTER_A => Ok(Mro::LetterA),
            LETTER_PHI => Ok(Mro::LetterPhi),
            LETTER_KHAI => Ok(Mro::LetterKhai),
            LETTER_HAO => Ok(Mro::LetterHao),
            LETTER_DAI => Ok(Mro::LetterDai),
            LETTER_CHU => Ok(Mro::LetterChu),
            LETTER_KEAAE => Ok(Mro::LetterKeaae),
            LETTER_OL => Ok(Mro::LetterOl),
            LETTER_MAEM => Ok(Mro::LetterMaem),
            LETTER_NIN => Ok(Mro::LetterNin),
            LETTER_PA => Ok(Mro::LetterPa),
            LETTER_OO => Ok(Mro::LetterOo),
            LETTER_O => Ok(Mro::LetterO),
            LETTER_RO => Ok(Mro::LetterRo),
            LETTER_SHI => Ok(Mro::LetterShi),
            LETTER_THEA => Ok(Mro::LetterThea),
            LETTER_EA => Ok(Mro::LetterEa),
            LETTER_WA => Ok(Mro::LetterWa),
            LETTER_E => Ok(Mro::LetterE),
            LETTER_KO => Ok(Mro::LetterKo),
            LETTER_LAN => Ok(Mro::LetterLan),
            LETTER_LA => Ok(Mro::LetterLa),
            LETTER_HAI => Ok(Mro::LetterHai),
            LETTER_RI => Ok(Mro::LetterRi),
            LETTER_TEK => Ok(Mro::LetterTek),
            DIGIT_ZERO => Ok(Mro::DigitZero),
            DIGIT_ONE => Ok(Mro::DigitOne),
            DIGIT_TWO => Ok(Mro::DigitTwo),
            DIGIT_THREE => Ok(Mro::DigitThree),
            DIGIT_FOUR => Ok(Mro::DigitFour),
            DIGIT_FIVE => Ok(Mro::DigitFive),
            DIGIT_SIX => Ok(Mro::DigitSix),
            DIGIT_SEVEN => Ok(Mro::DigitSeven),
            DIGIT_EIGHT => Ok(Mro::DigitEight),
            DIGIT_NINE => Ok(Mro::DigitNine),
            DANDA => Ok(Mro::Danda),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Mro {
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

impl std::convert::TryFrom<u32> for Mro {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Mro {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Mro {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Mro::LetterTa
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Mro::LetterTa => "mro letter ta",
            Mro::LetterNgi => "mro letter ngi",
            Mro::LetterYo => "mro letter yo",
            Mro::LetterMim => "mro letter mim",
            Mro::LetterBa => "mro letter ba",
            Mro::LetterDa => "mro letter da",
            Mro::LetterA => "mro letter a",
            Mro::LetterPhi => "mro letter phi",
            Mro::LetterKhai => "mro letter khai",
            Mro::LetterHao => "mro letter hao",
            Mro::LetterDai => "mro letter dai",
            Mro::LetterChu => "mro letter chu",
            Mro::LetterKeaae => "mro letter keaae",
            Mro::LetterOl => "mro letter ol",
            Mro::LetterMaem => "mro letter maem",
            Mro::LetterNin => "mro letter nin",
            Mro::LetterPa => "mro letter pa",
            Mro::LetterOo => "mro letter oo",
            Mro::LetterO => "mro letter o",
            Mro::LetterRo => "mro letter ro",
            Mro::LetterShi => "mro letter shi",
            Mro::LetterThea => "mro letter thea",
            Mro::LetterEa => "mro letter ea",
            Mro::LetterWa => "mro letter wa",
            Mro::LetterE => "mro letter e",
            Mro::LetterKo => "mro letter ko",
            Mro::LetterLan => "mro letter lan",
            Mro::LetterLa => "mro letter la",
            Mro::LetterHai => "mro letter hai",
            Mro::LetterRi => "mro letter ri",
            Mro::LetterTek => "mro letter tek",
            Mro::DigitZero => "mro digit zero",
            Mro::DigitOne => "mro digit one",
            Mro::DigitTwo => "mro digit two",
            Mro::DigitThree => "mro digit three",
            Mro::DigitFour => "mro digit four",
            Mro::DigitFive => "mro digit five",
            Mro::DigitSix => "mro digit six",
            Mro::DigitSeven => "mro digit seven",
            Mro::DigitEight => "mro digit eight",
            Mro::DigitNine => "mro digit nine",
            Mro::Danda => "mro danda",
        }
    }
}
