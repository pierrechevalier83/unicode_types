
/// An enum to represent all characters in the MasaramGondi block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum MasaramGondi {
    /// \u{11d00}: '𑴀'
    LetterA,
    /// \u{11d01}: '𑴁'
    LetterAa,
    /// \u{11d02}: '𑴂'
    LetterI,
    /// \u{11d03}: '𑴃'
    LetterIi,
    /// \u{11d04}: '𑴄'
    LetterU,
    /// \u{11d05}: '𑴅'
    LetterUu,
    /// \u{11d06}: '𑴆'
    LetterE,
    /// \u{11d08}: '𑴈'
    LetterAi,
    /// \u{11d09}: '𑴉'
    LetterO,
    /// \u{11d0b}: '𑴋'
    LetterAu,
    /// \u{11d0c}: '𑴌'
    LetterKa,
    /// \u{11d0d}: '𑴍'
    LetterKha,
    /// \u{11d0e}: '𑴎'
    LetterGa,
    /// \u{11d0f}: '𑴏'
    LetterGha,
    /// \u{11d10}: '𑴐'
    LetterNga,
    /// \u{11d11}: '𑴑'
    LetterCa,
    /// \u{11d12}: '𑴒'
    LetterCha,
    /// \u{11d13}: '𑴓'
    LetterJa,
    /// \u{11d14}: '𑴔'
    LetterJha,
    /// \u{11d15}: '𑴕'
    LetterNya,
    /// \u{11d16}: '𑴖'
    LetterTta,
    /// \u{11d17}: '𑴗'
    LetterTtha,
    /// \u{11d18}: '𑴘'
    LetterDda,
    /// \u{11d19}: '𑴙'
    LetterDdha,
    /// \u{11d1a}: '𑴚'
    LetterNna,
    /// \u{11d1b}: '𑴛'
    LetterTa,
    /// \u{11d1c}: '𑴜'
    LetterTha,
    /// \u{11d1d}: '𑴝'
    LetterDa,
    /// \u{11d1e}: '𑴞'
    LetterDha,
    /// \u{11d1f}: '𑴟'
    LetterNa,
    /// \u{11d20}: '𑴠'
    LetterPa,
    /// \u{11d21}: '𑴡'
    LetterPha,
    /// \u{11d22}: '𑴢'
    LetterBa,
    /// \u{11d23}: '𑴣'
    LetterBha,
    /// \u{11d24}: '𑴤'
    LetterMa,
    /// \u{11d25}: '𑴥'
    LetterYa,
    /// \u{11d26}: '𑴦'
    LetterRa,
    /// \u{11d27}: '𑴧'
    LetterLa,
    /// \u{11d28}: '𑴨'
    LetterVa,
    /// \u{11d29}: '𑴩'
    LetterSha,
    /// \u{11d2a}: '𑴪'
    LetterSsa,
    /// \u{11d2b}: '𑴫'
    LetterSa,
    /// \u{11d2c}: '𑴬'
    LetterHa,
    /// \u{11d2d}: '𑴭'
    LetterLla,
    /// \u{11d2e}: '𑴮'
    LetterKssa,
    /// \u{11d2f}: '𑴯'
    LetterJnya,
    /// \u{11d30}: '𑴰'
    LetterTra,
    /// \u{11d31}: '𑴱'
    VowelSignAa,
    /// \u{11d32}: '𑴲'
    VowelSignI,
    /// \u{11d33}: '𑴳'
    VowelSignIi,
    /// \u{11d34}: '𑴴'
    VowelSignU,
    /// \u{11d35}: '𑴵'
    VowelSignUu,
    /// \u{11d36}: '𑴶'
    VowelSignVocalicR,
    /// \u{11d3a}: '𑴺'
    VowelSignE,
    /// \u{11d3c}: '𑴼'
    VowelSignAi,
    /// \u{11d3d}: '𑴽'
    VowelSignO,
    /// \u{11d3f}: '𑴿'
    VowelSignAu,
    /// \u{11d40}: '𑵀'
    SignAnusvara,
    /// \u{11d41}: '𑵁'
    SignVisarga,
    /// \u{11d42}: '𑵂'
    SignNukta,
    /// \u{11d43}: '𑵃'
    SignCandra,
    /// \u{11d44}: '𑵄'
    SignHalanta,
    /// \u{11d45}: '𑵅'
    Virama,
    /// \u{11d46}: '𑵆'
    Repha,
    /// \u{11d47}: '𑵇'
    RaDashKara,
    /// \u{11d50}: '𑵐'
    DigitZero,
    /// \u{11d51}: '𑵑'
    DigitOne,
    /// \u{11d52}: '𑵒'
    DigitTwo,
    /// \u{11d53}: '𑵓'
    DigitThree,
    /// \u{11d54}: '𑵔'
    DigitFour,
    /// \u{11d55}: '𑵕'
    DigitFive,
    /// \u{11d56}: '𑵖'
    DigitSix,
    /// \u{11d57}: '𑵗'
    DigitSeven,
    /// \u{11d58}: '𑵘'
    DigitEight,
    /// \u{11d59}: '𑵙'
    DigitNine,
}

impl Into<char> for MasaramGondi {
    fn into(self) -> char {
        match self {
            MasaramGondi::LetterA => '𑴀',
            MasaramGondi::LetterAa => '𑴁',
            MasaramGondi::LetterI => '𑴂',
            MasaramGondi::LetterIi => '𑴃',
            MasaramGondi::LetterU => '𑴄',
            MasaramGondi::LetterUu => '𑴅',
            MasaramGondi::LetterE => '𑴆',
            MasaramGondi::LetterAi => '𑴈',
            MasaramGondi::LetterO => '𑴉',
            MasaramGondi::LetterAu => '𑴋',
            MasaramGondi::LetterKa => '𑴌',
            MasaramGondi::LetterKha => '𑴍',
            MasaramGondi::LetterGa => '𑴎',
            MasaramGondi::LetterGha => '𑴏',
            MasaramGondi::LetterNga => '𑴐',
            MasaramGondi::LetterCa => '𑴑',
            MasaramGondi::LetterCha => '𑴒',
            MasaramGondi::LetterJa => '𑴓',
            MasaramGondi::LetterJha => '𑴔',
            MasaramGondi::LetterNya => '𑴕',
            MasaramGondi::LetterTta => '𑴖',
            MasaramGondi::LetterTtha => '𑴗',
            MasaramGondi::LetterDda => '𑴘',
            MasaramGondi::LetterDdha => '𑴙',
            MasaramGondi::LetterNna => '𑴚',
            MasaramGondi::LetterTa => '𑴛',
            MasaramGondi::LetterTha => '𑴜',
            MasaramGondi::LetterDa => '𑴝',
            MasaramGondi::LetterDha => '𑴞',
            MasaramGondi::LetterNa => '𑴟',
            MasaramGondi::LetterPa => '𑴠',
            MasaramGondi::LetterPha => '𑴡',
            MasaramGondi::LetterBa => '𑴢',
            MasaramGondi::LetterBha => '𑴣',
            MasaramGondi::LetterMa => '𑴤',
            MasaramGondi::LetterYa => '𑴥',
            MasaramGondi::LetterRa => '𑴦',
            MasaramGondi::LetterLa => '𑴧',
            MasaramGondi::LetterVa => '𑴨',
            MasaramGondi::LetterSha => '𑴩',
            MasaramGondi::LetterSsa => '𑴪',
            MasaramGondi::LetterSa => '𑴫',
            MasaramGondi::LetterHa => '𑴬',
            MasaramGondi::LetterLla => '𑴭',
            MasaramGondi::LetterKssa => '𑴮',
            MasaramGondi::LetterJnya => '𑴯',
            MasaramGondi::LetterTra => '𑴰',
            MasaramGondi::VowelSignAa => '𑴱',
            MasaramGondi::VowelSignI => '𑴲',
            MasaramGondi::VowelSignIi => '𑴳',
            MasaramGondi::VowelSignU => '𑴴',
            MasaramGondi::VowelSignUu => '𑴵',
            MasaramGondi::VowelSignVocalicR => '𑴶',
            MasaramGondi::VowelSignE => '𑴺',
            MasaramGondi::VowelSignAi => '𑴼',
            MasaramGondi::VowelSignO => '𑴽',
            MasaramGondi::VowelSignAu => '𑴿',
            MasaramGondi::SignAnusvara => '𑵀',
            MasaramGondi::SignVisarga => '𑵁',
            MasaramGondi::SignNukta => '𑵂',
            MasaramGondi::SignCandra => '𑵃',
            MasaramGondi::SignHalanta => '𑵄',
            MasaramGondi::Virama => '𑵅',
            MasaramGondi::Repha => '𑵆',
            MasaramGondi::RaDashKara => '𑵇',
            MasaramGondi::DigitZero => '𑵐',
            MasaramGondi::DigitOne => '𑵑',
            MasaramGondi::DigitTwo => '𑵒',
            MasaramGondi::DigitThree => '𑵓',
            MasaramGondi::DigitFour => '𑵔',
            MasaramGondi::DigitFive => '𑵕',
            MasaramGondi::DigitSix => '𑵖',
            MasaramGondi::DigitSeven => '𑵗',
            MasaramGondi::DigitEight => '𑵘',
            MasaramGondi::DigitNine => '𑵙',
        }
    }
}

impl std::convert::TryFrom<char> for MasaramGondi {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𑴀' => Ok(MasaramGondi::LetterA),
            '𑴁' => Ok(MasaramGondi::LetterAa),
            '𑴂' => Ok(MasaramGondi::LetterI),
            '𑴃' => Ok(MasaramGondi::LetterIi),
            '𑴄' => Ok(MasaramGondi::LetterU),
            '𑴅' => Ok(MasaramGondi::LetterUu),
            '𑴆' => Ok(MasaramGondi::LetterE),
            '𑴈' => Ok(MasaramGondi::LetterAi),
            '𑴉' => Ok(MasaramGondi::LetterO),
            '𑴋' => Ok(MasaramGondi::LetterAu),
            '𑴌' => Ok(MasaramGondi::LetterKa),
            '𑴍' => Ok(MasaramGondi::LetterKha),
            '𑴎' => Ok(MasaramGondi::LetterGa),
            '𑴏' => Ok(MasaramGondi::LetterGha),
            '𑴐' => Ok(MasaramGondi::LetterNga),
            '𑴑' => Ok(MasaramGondi::LetterCa),
            '𑴒' => Ok(MasaramGondi::LetterCha),
            '𑴓' => Ok(MasaramGondi::LetterJa),
            '𑴔' => Ok(MasaramGondi::LetterJha),
            '𑴕' => Ok(MasaramGondi::LetterNya),
            '𑴖' => Ok(MasaramGondi::LetterTta),
            '𑴗' => Ok(MasaramGondi::LetterTtha),
            '𑴘' => Ok(MasaramGondi::LetterDda),
            '𑴙' => Ok(MasaramGondi::LetterDdha),
            '𑴚' => Ok(MasaramGondi::LetterNna),
            '𑴛' => Ok(MasaramGondi::LetterTa),
            '𑴜' => Ok(MasaramGondi::LetterTha),
            '𑴝' => Ok(MasaramGondi::LetterDa),
            '𑴞' => Ok(MasaramGondi::LetterDha),
            '𑴟' => Ok(MasaramGondi::LetterNa),
            '𑴠' => Ok(MasaramGondi::LetterPa),
            '𑴡' => Ok(MasaramGondi::LetterPha),
            '𑴢' => Ok(MasaramGondi::LetterBa),
            '𑴣' => Ok(MasaramGondi::LetterBha),
            '𑴤' => Ok(MasaramGondi::LetterMa),
            '𑴥' => Ok(MasaramGondi::LetterYa),
            '𑴦' => Ok(MasaramGondi::LetterRa),
            '𑴧' => Ok(MasaramGondi::LetterLa),
            '𑴨' => Ok(MasaramGondi::LetterVa),
            '𑴩' => Ok(MasaramGondi::LetterSha),
            '𑴪' => Ok(MasaramGondi::LetterSsa),
            '𑴫' => Ok(MasaramGondi::LetterSa),
            '𑴬' => Ok(MasaramGondi::LetterHa),
            '𑴭' => Ok(MasaramGondi::LetterLla),
            '𑴮' => Ok(MasaramGondi::LetterKssa),
            '𑴯' => Ok(MasaramGondi::LetterJnya),
            '𑴰' => Ok(MasaramGondi::LetterTra),
            '𑴱' => Ok(MasaramGondi::VowelSignAa),
            '𑴲' => Ok(MasaramGondi::VowelSignI),
            '𑴳' => Ok(MasaramGondi::VowelSignIi),
            '𑴴' => Ok(MasaramGondi::VowelSignU),
            '𑴵' => Ok(MasaramGondi::VowelSignUu),
            '𑴶' => Ok(MasaramGondi::VowelSignVocalicR),
            '𑴺' => Ok(MasaramGondi::VowelSignE),
            '𑴼' => Ok(MasaramGondi::VowelSignAi),
            '𑴽' => Ok(MasaramGondi::VowelSignO),
            '𑴿' => Ok(MasaramGondi::VowelSignAu),
            '𑵀' => Ok(MasaramGondi::SignAnusvara),
            '𑵁' => Ok(MasaramGondi::SignVisarga),
            '𑵂' => Ok(MasaramGondi::SignNukta),
            '𑵃' => Ok(MasaramGondi::SignCandra),
            '𑵄' => Ok(MasaramGondi::SignHalanta),
            '𑵅' => Ok(MasaramGondi::Virama),
            '𑵆' => Ok(MasaramGondi::Repha),
            '𑵇' => Ok(MasaramGondi::RaDashKara),
            '𑵐' => Ok(MasaramGondi::DigitZero),
            '𑵑' => Ok(MasaramGondi::DigitOne),
            '𑵒' => Ok(MasaramGondi::DigitTwo),
            '𑵓' => Ok(MasaramGondi::DigitThree),
            '𑵔' => Ok(MasaramGondi::DigitFour),
            '𑵕' => Ok(MasaramGondi::DigitFive),
            '𑵖' => Ok(MasaramGondi::DigitSix),
            '𑵗' => Ok(MasaramGondi::DigitSeven),
            '𑵘' => Ok(MasaramGondi::DigitEight),
            '𑵙' => Ok(MasaramGondi::DigitNine),
            _ => Err(()),
        }
    }
}

impl Into<u32> for MasaramGondi {
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

impl std::convert::TryFrom<u32> for MasaramGondi {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for MasaramGondi {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl MasaramGondi {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        MasaramGondi::LetterA
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("MasaramGondi{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
