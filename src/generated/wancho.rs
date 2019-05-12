
/// An enum to represent all characters in the Wancho block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Wancho {
    /// \u{1e2c0}: '𞋀'
    LetterAa,
    /// \u{1e2c1}: '𞋁'
    LetterA,
    /// \u{1e2c2}: '𞋂'
    LetterBa,
    /// \u{1e2c3}: '𞋃'
    LetterCa,
    /// \u{1e2c4}: '𞋄'
    LetterDa,
    /// \u{1e2c5}: '𞋅'
    LetterGa,
    /// \u{1e2c6}: '𞋆'
    LetterYa,
    /// \u{1e2c7}: '𞋇'
    LetterPha,
    /// \u{1e2c8}: '𞋈'
    LetterLa,
    /// \u{1e2c9}: '𞋉'
    LetterNa,
    /// \u{1e2ca}: '𞋊'
    LetterPa,
    /// \u{1e2cb}: '𞋋'
    LetterTa,
    /// \u{1e2cc}: '𞋌'
    LetterTha,
    /// \u{1e2cd}: '𞋍'
    LetterFa,
    /// \u{1e2ce}: '𞋎'
    LetterSa,
    /// \u{1e2cf}: '𞋏'
    LetterSha,
    /// \u{1e2d0}: '𞋐'
    LetterJa,
    /// \u{1e2d1}: '𞋑'
    LetterZa,
    /// \u{1e2d2}: '𞋒'
    LetterWa,
    /// \u{1e2d3}: '𞋓'
    LetterVa,
    /// \u{1e2d4}: '𞋔'
    LetterKa,
    /// \u{1e2d5}: '𞋕'
    LetterO,
    /// \u{1e2d6}: '𞋖'
    LetterAu,
    /// \u{1e2d7}: '𞋗'
    LetterRa,
    /// \u{1e2d8}: '𞋘'
    LetterMa,
    /// \u{1e2d9}: '𞋙'
    LetterKha,
    /// \u{1e2da}: '𞋚'
    LetterHa,
    /// \u{1e2db}: '𞋛'
    LetterE,
    /// \u{1e2dc}: '𞋜'
    LetterI,
    /// \u{1e2dd}: '𞋝'
    LetterNga,
    /// \u{1e2de}: '𞋞'
    LetterU,
    /// \u{1e2df}: '𞋟'
    LetterLlha,
    /// \u{1e2e0}: '𞋠'
    LetterTsa,
    /// \u{1e2e1}: '𞋡'
    LetterTra,
    /// \u{1e2e2}: '𞋢'
    LetterOng,
    /// \u{1e2e3}: '𞋣'
    LetterAang,
    /// \u{1e2e4}: '𞋤'
    LetterAng,
    /// \u{1e2e5}: '𞋥'
    LetterIng,
    /// \u{1e2e6}: '𞋦'
    LetterOn,
    /// \u{1e2e7}: '𞋧'
    LetterEn,
    /// \u{1e2e8}: '𞋨'
    LetterAan,
    /// \u{1e2e9}: '𞋩'
    LetterNya,
    /// \u{1e2ea}: '𞋪'
    LetterUen,
    /// \u{1e2eb}: '𞋫'
    LetterYih,
    /// \u{1e2ec}: '𞋬'
    ToneTup,
    /// \u{1e2ed}: '𞋭'
    ToneTupni,
    /// \u{1e2ee}: '𞋮'
    ToneKoi,
    /// \u{1e2ef}: '𞋯'
    ToneKoini,
    /// \u{1e2f0}: '𞋰'
    DigitZero,
    /// \u{1e2f1}: '𞋱'
    DigitOne,
    /// \u{1e2f2}: '𞋲'
    DigitTwo,
    /// \u{1e2f3}: '𞋳'
    DigitThree,
    /// \u{1e2f4}: '𞋴'
    DigitFour,
    /// \u{1e2f5}: '𞋵'
    DigitFive,
    /// \u{1e2f6}: '𞋶'
    DigitSix,
    /// \u{1e2f7}: '𞋷'
    DigitSeven,
    /// \u{1e2f8}: '𞋸'
    DigitEight,
    /// \u{1e2f9}: '𞋹'
    DigitNine,
}

impl Into<char> for Wancho {
    fn into(self) -> char {
        match self {
            Wancho::LetterAa => '𞋀',
            Wancho::LetterA => '𞋁',
            Wancho::LetterBa => '𞋂',
            Wancho::LetterCa => '𞋃',
            Wancho::LetterDa => '𞋄',
            Wancho::LetterGa => '𞋅',
            Wancho::LetterYa => '𞋆',
            Wancho::LetterPha => '𞋇',
            Wancho::LetterLa => '𞋈',
            Wancho::LetterNa => '𞋉',
            Wancho::LetterPa => '𞋊',
            Wancho::LetterTa => '𞋋',
            Wancho::LetterTha => '𞋌',
            Wancho::LetterFa => '𞋍',
            Wancho::LetterSa => '𞋎',
            Wancho::LetterSha => '𞋏',
            Wancho::LetterJa => '𞋐',
            Wancho::LetterZa => '𞋑',
            Wancho::LetterWa => '𞋒',
            Wancho::LetterVa => '𞋓',
            Wancho::LetterKa => '𞋔',
            Wancho::LetterO => '𞋕',
            Wancho::LetterAu => '𞋖',
            Wancho::LetterRa => '𞋗',
            Wancho::LetterMa => '𞋘',
            Wancho::LetterKha => '𞋙',
            Wancho::LetterHa => '𞋚',
            Wancho::LetterE => '𞋛',
            Wancho::LetterI => '𞋜',
            Wancho::LetterNga => '𞋝',
            Wancho::LetterU => '𞋞',
            Wancho::LetterLlha => '𞋟',
            Wancho::LetterTsa => '𞋠',
            Wancho::LetterTra => '𞋡',
            Wancho::LetterOng => '𞋢',
            Wancho::LetterAang => '𞋣',
            Wancho::LetterAng => '𞋤',
            Wancho::LetterIng => '𞋥',
            Wancho::LetterOn => '𞋦',
            Wancho::LetterEn => '𞋧',
            Wancho::LetterAan => '𞋨',
            Wancho::LetterNya => '𞋩',
            Wancho::LetterUen => '𞋪',
            Wancho::LetterYih => '𞋫',
            Wancho::ToneTup => '𞋬',
            Wancho::ToneTupni => '𞋭',
            Wancho::ToneKoi => '𞋮',
            Wancho::ToneKoini => '𞋯',
            Wancho::DigitZero => '𞋰',
            Wancho::DigitOne => '𞋱',
            Wancho::DigitTwo => '𞋲',
            Wancho::DigitThree => '𞋳',
            Wancho::DigitFour => '𞋴',
            Wancho::DigitFive => '𞋵',
            Wancho::DigitSix => '𞋶',
            Wancho::DigitSeven => '𞋷',
            Wancho::DigitEight => '𞋸',
            Wancho::DigitNine => '𞋹',
        }
    }
}

impl std::convert::TryFrom<char> for Wancho {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𞋀' => Ok(Wancho::LetterAa),
            '𞋁' => Ok(Wancho::LetterA),
            '𞋂' => Ok(Wancho::LetterBa),
            '𞋃' => Ok(Wancho::LetterCa),
            '𞋄' => Ok(Wancho::LetterDa),
            '𞋅' => Ok(Wancho::LetterGa),
            '𞋆' => Ok(Wancho::LetterYa),
            '𞋇' => Ok(Wancho::LetterPha),
            '𞋈' => Ok(Wancho::LetterLa),
            '𞋉' => Ok(Wancho::LetterNa),
            '𞋊' => Ok(Wancho::LetterPa),
            '𞋋' => Ok(Wancho::LetterTa),
            '𞋌' => Ok(Wancho::LetterTha),
            '𞋍' => Ok(Wancho::LetterFa),
            '𞋎' => Ok(Wancho::LetterSa),
            '𞋏' => Ok(Wancho::LetterSha),
            '𞋐' => Ok(Wancho::LetterJa),
            '𞋑' => Ok(Wancho::LetterZa),
            '𞋒' => Ok(Wancho::LetterWa),
            '𞋓' => Ok(Wancho::LetterVa),
            '𞋔' => Ok(Wancho::LetterKa),
            '𞋕' => Ok(Wancho::LetterO),
            '𞋖' => Ok(Wancho::LetterAu),
            '𞋗' => Ok(Wancho::LetterRa),
            '𞋘' => Ok(Wancho::LetterMa),
            '𞋙' => Ok(Wancho::LetterKha),
            '𞋚' => Ok(Wancho::LetterHa),
            '𞋛' => Ok(Wancho::LetterE),
            '𞋜' => Ok(Wancho::LetterI),
            '𞋝' => Ok(Wancho::LetterNga),
            '𞋞' => Ok(Wancho::LetterU),
            '𞋟' => Ok(Wancho::LetterLlha),
            '𞋠' => Ok(Wancho::LetterTsa),
            '𞋡' => Ok(Wancho::LetterTra),
            '𞋢' => Ok(Wancho::LetterOng),
            '𞋣' => Ok(Wancho::LetterAang),
            '𞋤' => Ok(Wancho::LetterAng),
            '𞋥' => Ok(Wancho::LetterIng),
            '𞋦' => Ok(Wancho::LetterOn),
            '𞋧' => Ok(Wancho::LetterEn),
            '𞋨' => Ok(Wancho::LetterAan),
            '𞋩' => Ok(Wancho::LetterNya),
            '𞋪' => Ok(Wancho::LetterUen),
            '𞋫' => Ok(Wancho::LetterYih),
            '𞋬' => Ok(Wancho::ToneTup),
            '𞋭' => Ok(Wancho::ToneTupni),
            '𞋮' => Ok(Wancho::ToneKoi),
            '𞋯' => Ok(Wancho::ToneKoini),
            '𞋰' => Ok(Wancho::DigitZero),
            '𞋱' => Ok(Wancho::DigitOne),
            '𞋲' => Ok(Wancho::DigitTwo),
            '𞋳' => Ok(Wancho::DigitThree),
            '𞋴' => Ok(Wancho::DigitFour),
            '𞋵' => Ok(Wancho::DigitFive),
            '𞋶' => Ok(Wancho::DigitSix),
            '𞋷' => Ok(Wancho::DigitSeven),
            '𞋸' => Ok(Wancho::DigitEight),
            '𞋹' => Ok(Wancho::DigitNine),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Wancho {
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

impl std::convert::TryFrom<u32> for Wancho {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Wancho {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Wancho {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Wancho::LetterAa
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Wancho{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
