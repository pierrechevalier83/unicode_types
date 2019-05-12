
/// An enum to represent all characters in the HanifiRohingya block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum HanifiRohingya {
    /// \u{10d00}: '𐴀'
    LetterA,
    /// \u{10d01}: '𐴁'
    LetterBa,
    /// \u{10d02}: '𐴂'
    LetterPa,
    /// \u{10d03}: '𐴃'
    LetterTa,
    /// \u{10d04}: '𐴄'
    LetterTta,
    /// \u{10d05}: '𐴅'
    LetterJa,
    /// \u{10d06}: '𐴆'
    LetterCa,
    /// \u{10d07}: '𐴇'
    LetterHa,
    /// \u{10d08}: '𐴈'
    LetterKha,
    /// \u{10d09}: '𐴉'
    LetterFa,
    /// \u{10d0a}: '𐴊'
    LetterDa,
    /// \u{10d0b}: '𐴋'
    LetterDda,
    /// \u{10d0c}: '𐴌'
    LetterRa,
    /// \u{10d0d}: '𐴍'
    LetterRra,
    /// \u{10d0e}: '𐴎'
    LetterZa,
    /// \u{10d0f}: '𐴏'
    LetterSa,
    /// \u{10d10}: '𐴐'
    LetterSha,
    /// \u{10d11}: '𐴑'
    LetterKa,
    /// \u{10d12}: '𐴒'
    LetterGa,
    /// \u{10d13}: '𐴓'
    LetterLa,
    /// \u{10d14}: '𐴔'
    LetterMa,
    /// \u{10d15}: '𐴕'
    LetterNa,
    /// \u{10d16}: '𐴖'
    LetterWa,
    /// \u{10d17}: '𐴗'
    LetterKinnaWa,
    /// \u{10d18}: '𐴘'
    LetterYa,
    /// \u{10d19}: '𐴙'
    LetterKinnaYa,
    /// \u{10d1a}: '𐴚'
    LetterNga,
    /// \u{10d1b}: '𐴛'
    LetterNya,
    /// \u{10d1c}: '𐴜'
    LetterVa,
    /// \u{10d1d}: '𐴝'
    VowelA,
    /// \u{10d1e}: '𐴞'
    VowelI,
    /// \u{10d1f}: '𐴟'
    VowelU,
    /// \u{10d20}: '𐴠'
    VowelE,
    /// \u{10d21}: '𐴡'
    VowelO,
    /// \u{10d22}: '𐴢'
    MarkSakin,
    /// \u{10d23}: '𐴣'
    MarkNaKhonna,
    /// \u{10d24}: '𐴤'
    SignHarbahay,
    /// \u{10d25}: '𐴥'
    SignTahala,
    /// \u{10d26}: '𐴦'
    SignTana,
    /// \u{10d27}: '𐴧'
    SignTassi,
    /// \u{10d30}: '𐴰'
    DigitZero,
    /// \u{10d31}: '𐴱'
    DigitOne,
    /// \u{10d32}: '𐴲'
    DigitTwo,
    /// \u{10d33}: '𐴳'
    DigitThree,
    /// \u{10d34}: '𐴴'
    DigitFour,
    /// \u{10d35}: '𐴵'
    DigitFive,
    /// \u{10d36}: '𐴶'
    DigitSix,
    /// \u{10d37}: '𐴷'
    DigitSeven,
    /// \u{10d38}: '𐴸'
    DigitEight,
    /// \u{10d39}: '𐴹'
    DigitNine,
}

impl Into<char> for HanifiRohingya {
    fn into(self) -> char {
        match self {
            HanifiRohingya::LetterA => '𐴀',
            HanifiRohingya::LetterBa => '𐴁',
            HanifiRohingya::LetterPa => '𐴂',
            HanifiRohingya::LetterTa => '𐴃',
            HanifiRohingya::LetterTta => '𐴄',
            HanifiRohingya::LetterJa => '𐴅',
            HanifiRohingya::LetterCa => '𐴆',
            HanifiRohingya::LetterHa => '𐴇',
            HanifiRohingya::LetterKha => '𐴈',
            HanifiRohingya::LetterFa => '𐴉',
            HanifiRohingya::LetterDa => '𐴊',
            HanifiRohingya::LetterDda => '𐴋',
            HanifiRohingya::LetterRa => '𐴌',
            HanifiRohingya::LetterRra => '𐴍',
            HanifiRohingya::LetterZa => '𐴎',
            HanifiRohingya::LetterSa => '𐴏',
            HanifiRohingya::LetterSha => '𐴐',
            HanifiRohingya::LetterKa => '𐴑',
            HanifiRohingya::LetterGa => '𐴒',
            HanifiRohingya::LetterLa => '𐴓',
            HanifiRohingya::LetterMa => '𐴔',
            HanifiRohingya::LetterNa => '𐴕',
            HanifiRohingya::LetterWa => '𐴖',
            HanifiRohingya::LetterKinnaWa => '𐴗',
            HanifiRohingya::LetterYa => '𐴘',
            HanifiRohingya::LetterKinnaYa => '𐴙',
            HanifiRohingya::LetterNga => '𐴚',
            HanifiRohingya::LetterNya => '𐴛',
            HanifiRohingya::LetterVa => '𐴜',
            HanifiRohingya::VowelA => '𐴝',
            HanifiRohingya::VowelI => '𐴞',
            HanifiRohingya::VowelU => '𐴟',
            HanifiRohingya::VowelE => '𐴠',
            HanifiRohingya::VowelO => '𐴡',
            HanifiRohingya::MarkSakin => '𐴢',
            HanifiRohingya::MarkNaKhonna => '𐴣',
            HanifiRohingya::SignHarbahay => '𐴤',
            HanifiRohingya::SignTahala => '𐴥',
            HanifiRohingya::SignTana => '𐴦',
            HanifiRohingya::SignTassi => '𐴧',
            HanifiRohingya::DigitZero => '𐴰',
            HanifiRohingya::DigitOne => '𐴱',
            HanifiRohingya::DigitTwo => '𐴲',
            HanifiRohingya::DigitThree => '𐴳',
            HanifiRohingya::DigitFour => '𐴴',
            HanifiRohingya::DigitFive => '𐴵',
            HanifiRohingya::DigitSix => '𐴶',
            HanifiRohingya::DigitSeven => '𐴷',
            HanifiRohingya::DigitEight => '𐴸',
            HanifiRohingya::DigitNine => '𐴹',
        }
    }
}

impl std::convert::TryFrom<char> for HanifiRohingya {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𐴀' => Ok(HanifiRohingya::LetterA),
            '𐴁' => Ok(HanifiRohingya::LetterBa),
            '𐴂' => Ok(HanifiRohingya::LetterPa),
            '𐴃' => Ok(HanifiRohingya::LetterTa),
            '𐴄' => Ok(HanifiRohingya::LetterTta),
            '𐴅' => Ok(HanifiRohingya::LetterJa),
            '𐴆' => Ok(HanifiRohingya::LetterCa),
            '𐴇' => Ok(HanifiRohingya::LetterHa),
            '𐴈' => Ok(HanifiRohingya::LetterKha),
            '𐴉' => Ok(HanifiRohingya::LetterFa),
            '𐴊' => Ok(HanifiRohingya::LetterDa),
            '𐴋' => Ok(HanifiRohingya::LetterDda),
            '𐴌' => Ok(HanifiRohingya::LetterRa),
            '𐴍' => Ok(HanifiRohingya::LetterRra),
            '𐴎' => Ok(HanifiRohingya::LetterZa),
            '𐴏' => Ok(HanifiRohingya::LetterSa),
            '𐴐' => Ok(HanifiRohingya::LetterSha),
            '𐴑' => Ok(HanifiRohingya::LetterKa),
            '𐴒' => Ok(HanifiRohingya::LetterGa),
            '𐴓' => Ok(HanifiRohingya::LetterLa),
            '𐴔' => Ok(HanifiRohingya::LetterMa),
            '𐴕' => Ok(HanifiRohingya::LetterNa),
            '𐴖' => Ok(HanifiRohingya::LetterWa),
            '𐴗' => Ok(HanifiRohingya::LetterKinnaWa),
            '𐴘' => Ok(HanifiRohingya::LetterYa),
            '𐴙' => Ok(HanifiRohingya::LetterKinnaYa),
            '𐴚' => Ok(HanifiRohingya::LetterNga),
            '𐴛' => Ok(HanifiRohingya::LetterNya),
            '𐴜' => Ok(HanifiRohingya::LetterVa),
            '𐴝' => Ok(HanifiRohingya::VowelA),
            '𐴞' => Ok(HanifiRohingya::VowelI),
            '𐴟' => Ok(HanifiRohingya::VowelU),
            '𐴠' => Ok(HanifiRohingya::VowelE),
            '𐴡' => Ok(HanifiRohingya::VowelO),
            '𐴢' => Ok(HanifiRohingya::MarkSakin),
            '𐴣' => Ok(HanifiRohingya::MarkNaKhonna),
            '𐴤' => Ok(HanifiRohingya::SignHarbahay),
            '𐴥' => Ok(HanifiRohingya::SignTahala),
            '𐴦' => Ok(HanifiRohingya::SignTana),
            '𐴧' => Ok(HanifiRohingya::SignTassi),
            '𐴰' => Ok(HanifiRohingya::DigitZero),
            '𐴱' => Ok(HanifiRohingya::DigitOne),
            '𐴲' => Ok(HanifiRohingya::DigitTwo),
            '𐴳' => Ok(HanifiRohingya::DigitThree),
            '𐴴' => Ok(HanifiRohingya::DigitFour),
            '𐴵' => Ok(HanifiRohingya::DigitFive),
            '𐴶' => Ok(HanifiRohingya::DigitSix),
            '𐴷' => Ok(HanifiRohingya::DigitSeven),
            '𐴸' => Ok(HanifiRohingya::DigitEight),
            '𐴹' => Ok(HanifiRohingya::DigitNine),
            _ => Err(()),
        }
    }
}

impl Into<u32> for HanifiRohingya {
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

impl std::convert::TryFrom<u32> for HanifiRohingya {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for HanifiRohingya {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl HanifiRohingya {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        HanifiRohingya::LetterA
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("HanifiRohingya{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
