
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
        match self {
            Mro::LetterTa => '𖩀',
            Mro::LetterNgi => '𖩁',
            Mro::LetterYo => '𖩂',
            Mro::LetterMim => '𖩃',
            Mro::LetterBa => '𖩄',
            Mro::LetterDa => '𖩅',
            Mro::LetterA => '𖩆',
            Mro::LetterPhi => '𖩇',
            Mro::LetterKhai => '𖩈',
            Mro::LetterHao => '𖩉',
            Mro::LetterDai => '𖩊',
            Mro::LetterChu => '𖩋',
            Mro::LetterKeaae => '𖩌',
            Mro::LetterOl => '𖩍',
            Mro::LetterMaem => '𖩎',
            Mro::LetterNin => '𖩏',
            Mro::LetterPa => '𖩐',
            Mro::LetterOo => '𖩑',
            Mro::LetterO => '𖩒',
            Mro::LetterRo => '𖩓',
            Mro::LetterShi => '𖩔',
            Mro::LetterThea => '𖩕',
            Mro::LetterEa => '𖩖',
            Mro::LetterWa => '𖩗',
            Mro::LetterE => '𖩘',
            Mro::LetterKo => '𖩙',
            Mro::LetterLan => '𖩚',
            Mro::LetterLa => '𖩛',
            Mro::LetterHai => '𖩜',
            Mro::LetterRi => '𖩝',
            Mro::LetterTek => '𖩞',
            Mro::DigitZero => '𖩠',
            Mro::DigitOne => '𖩡',
            Mro::DigitTwo => '𖩢',
            Mro::DigitThree => '𖩣',
            Mro::DigitFour => '𖩤',
            Mro::DigitFive => '𖩥',
            Mro::DigitSix => '𖩦',
            Mro::DigitSeven => '𖩧',
            Mro::DigitEight => '𖩨',
            Mro::DigitNine => '𖩩',
            Mro::Danda => '𖩮',
        }
    }
}

impl std::convert::TryFrom<char> for Mro {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𖩀' => Ok(Mro::LetterTa),
            '𖩁' => Ok(Mro::LetterNgi),
            '𖩂' => Ok(Mro::LetterYo),
            '𖩃' => Ok(Mro::LetterMim),
            '𖩄' => Ok(Mro::LetterBa),
            '𖩅' => Ok(Mro::LetterDa),
            '𖩆' => Ok(Mro::LetterA),
            '𖩇' => Ok(Mro::LetterPhi),
            '𖩈' => Ok(Mro::LetterKhai),
            '𖩉' => Ok(Mro::LetterHao),
            '𖩊' => Ok(Mro::LetterDai),
            '𖩋' => Ok(Mro::LetterChu),
            '𖩌' => Ok(Mro::LetterKeaae),
            '𖩍' => Ok(Mro::LetterOl),
            '𖩎' => Ok(Mro::LetterMaem),
            '𖩏' => Ok(Mro::LetterNin),
            '𖩐' => Ok(Mro::LetterPa),
            '𖩑' => Ok(Mro::LetterOo),
            '𖩒' => Ok(Mro::LetterO),
            '𖩓' => Ok(Mro::LetterRo),
            '𖩔' => Ok(Mro::LetterShi),
            '𖩕' => Ok(Mro::LetterThea),
            '𖩖' => Ok(Mro::LetterEa),
            '𖩗' => Ok(Mro::LetterWa),
            '𖩘' => Ok(Mro::LetterE),
            '𖩙' => Ok(Mro::LetterKo),
            '𖩚' => Ok(Mro::LetterLan),
            '𖩛' => Ok(Mro::LetterLa),
            '𖩜' => Ok(Mro::LetterHai),
            '𖩝' => Ok(Mro::LetterRi),
            '𖩞' => Ok(Mro::LetterTek),
            '𖩠' => Ok(Mro::DigitZero),
            '𖩡' => Ok(Mro::DigitOne),
            '𖩢' => Ok(Mro::DigitTwo),
            '𖩣' => Ok(Mro::DigitThree),
            '𖩤' => Ok(Mro::DigitFour),
            '𖩥' => Ok(Mro::DigitFive),
            '𖩦' => Ok(Mro::DigitSix),
            '𖩧' => Ok(Mro::DigitSeven),
            '𖩨' => Ok(Mro::DigitEight),
            '𖩩' => Ok(Mro::DigitNine),
            '𖩮' => Ok(Mro::Danda),
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
