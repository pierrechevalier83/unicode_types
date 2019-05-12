
/// An enum to represent all characters in the Takri block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Takri {
    /// \u{11680}: '𑚀'
    LetterA,
    /// \u{11681}: '𑚁'
    LetterAa,
    /// \u{11682}: '𑚂'
    LetterI,
    /// \u{11683}: '𑚃'
    LetterIi,
    /// \u{11684}: '𑚄'
    LetterU,
    /// \u{11685}: '𑚅'
    LetterUu,
    /// \u{11686}: '𑚆'
    LetterE,
    /// \u{11687}: '𑚇'
    LetterAi,
    /// \u{11688}: '𑚈'
    LetterO,
    /// \u{11689}: '𑚉'
    LetterAu,
    /// \u{1168a}: '𑚊'
    LetterKa,
    /// \u{1168b}: '𑚋'
    LetterKha,
    /// \u{1168c}: '𑚌'
    LetterGa,
    /// \u{1168d}: '𑚍'
    LetterGha,
    /// \u{1168e}: '𑚎'
    LetterNga,
    /// \u{1168f}: '𑚏'
    LetterCa,
    /// \u{11690}: '𑚐'
    LetterCha,
    /// \u{11691}: '𑚑'
    LetterJa,
    /// \u{11692}: '𑚒'
    LetterJha,
    /// \u{11693}: '𑚓'
    LetterNya,
    /// \u{11694}: '𑚔'
    LetterTta,
    /// \u{11695}: '𑚕'
    LetterTtha,
    /// \u{11696}: '𑚖'
    LetterDda,
    /// \u{11697}: '𑚗'
    LetterDdha,
    /// \u{11698}: '𑚘'
    LetterNna,
    /// \u{11699}: '𑚙'
    LetterTa,
    /// \u{1169a}: '𑚚'
    LetterTha,
    /// \u{1169b}: '𑚛'
    LetterDa,
    /// \u{1169c}: '𑚜'
    LetterDha,
    /// \u{1169d}: '𑚝'
    LetterNa,
    /// \u{1169e}: '𑚞'
    LetterPa,
    /// \u{1169f}: '𑚟'
    LetterPha,
    /// \u{116a0}: '𑚠'
    LetterBa,
    /// \u{116a1}: '𑚡'
    LetterBha,
    /// \u{116a2}: '𑚢'
    LetterMa,
    /// \u{116a3}: '𑚣'
    LetterYa,
    /// \u{116a4}: '𑚤'
    LetterRa,
    /// \u{116a5}: '𑚥'
    LetterLa,
    /// \u{116a6}: '𑚦'
    LetterVa,
    /// \u{116a7}: '𑚧'
    LetterSha,
    /// \u{116a8}: '𑚨'
    LetterSa,
    /// \u{116a9}: '𑚩'
    LetterHa,
    /// \u{116aa}: '𑚪'
    LetterRra,
    /// \u{116ab}: '𑚫'
    SignAnusvara,
    /// \u{116ac}: '𑚬'
    SignVisarga,
    /// \u{116ad}: '𑚭'
    VowelSignAa,
    /// \u{116ae}: '𑚮'
    VowelSignI,
    /// \u{116af}: '𑚯'
    VowelSignIi,
    /// \u{116b0}: '𑚰'
    VowelSignU,
    /// \u{116b1}: '𑚱'
    VowelSignUu,
    /// \u{116b2}: '𑚲'
    VowelSignE,
    /// \u{116b3}: '𑚳'
    VowelSignAi,
    /// \u{116b4}: '𑚴'
    VowelSignO,
    /// \u{116b5}: '𑚵'
    VowelSignAu,
    /// \u{116b6}: '𑚶'
    SignVirama,
    /// \u{116b7}: '𑚷'
    SignNukta,
    /// \u{116b8}: '𑚸'
    LetterArchaicKha,
    /// \u{116c0}: '𑛀'
    DigitZero,
    /// \u{116c1}: '𑛁'
    DigitOne,
    /// \u{116c2}: '𑛂'
    DigitTwo,
    /// \u{116c3}: '𑛃'
    DigitThree,
    /// \u{116c4}: '𑛄'
    DigitFour,
    /// \u{116c5}: '𑛅'
    DigitFive,
    /// \u{116c6}: '𑛆'
    DigitSix,
    /// \u{116c7}: '𑛇'
    DigitSeven,
    /// \u{116c8}: '𑛈'
    DigitEight,
    /// \u{116c9}: '𑛉'
    DigitNine,
}

impl Into<char> for Takri {
    fn into(self) -> char {
        match self {
            Takri::LetterA => '𑚀',
            Takri::LetterAa => '𑚁',
            Takri::LetterI => '𑚂',
            Takri::LetterIi => '𑚃',
            Takri::LetterU => '𑚄',
            Takri::LetterUu => '𑚅',
            Takri::LetterE => '𑚆',
            Takri::LetterAi => '𑚇',
            Takri::LetterO => '𑚈',
            Takri::LetterAu => '𑚉',
            Takri::LetterKa => '𑚊',
            Takri::LetterKha => '𑚋',
            Takri::LetterGa => '𑚌',
            Takri::LetterGha => '𑚍',
            Takri::LetterNga => '𑚎',
            Takri::LetterCa => '𑚏',
            Takri::LetterCha => '𑚐',
            Takri::LetterJa => '𑚑',
            Takri::LetterJha => '𑚒',
            Takri::LetterNya => '𑚓',
            Takri::LetterTta => '𑚔',
            Takri::LetterTtha => '𑚕',
            Takri::LetterDda => '𑚖',
            Takri::LetterDdha => '𑚗',
            Takri::LetterNna => '𑚘',
            Takri::LetterTa => '𑚙',
            Takri::LetterTha => '𑚚',
            Takri::LetterDa => '𑚛',
            Takri::LetterDha => '𑚜',
            Takri::LetterNa => '𑚝',
            Takri::LetterPa => '𑚞',
            Takri::LetterPha => '𑚟',
            Takri::LetterBa => '𑚠',
            Takri::LetterBha => '𑚡',
            Takri::LetterMa => '𑚢',
            Takri::LetterYa => '𑚣',
            Takri::LetterRa => '𑚤',
            Takri::LetterLa => '𑚥',
            Takri::LetterVa => '𑚦',
            Takri::LetterSha => '𑚧',
            Takri::LetterSa => '𑚨',
            Takri::LetterHa => '𑚩',
            Takri::LetterRra => '𑚪',
            Takri::SignAnusvara => '𑚫',
            Takri::SignVisarga => '𑚬',
            Takri::VowelSignAa => '𑚭',
            Takri::VowelSignI => '𑚮',
            Takri::VowelSignIi => '𑚯',
            Takri::VowelSignU => '𑚰',
            Takri::VowelSignUu => '𑚱',
            Takri::VowelSignE => '𑚲',
            Takri::VowelSignAi => '𑚳',
            Takri::VowelSignO => '𑚴',
            Takri::VowelSignAu => '𑚵',
            Takri::SignVirama => '𑚶',
            Takri::SignNukta => '𑚷',
            Takri::LetterArchaicKha => '𑚸',
            Takri::DigitZero => '𑛀',
            Takri::DigitOne => '𑛁',
            Takri::DigitTwo => '𑛂',
            Takri::DigitThree => '𑛃',
            Takri::DigitFour => '𑛄',
            Takri::DigitFive => '𑛅',
            Takri::DigitSix => '𑛆',
            Takri::DigitSeven => '𑛇',
            Takri::DigitEight => '𑛈',
            Takri::DigitNine => '𑛉',
        }
    }
}

impl std::convert::TryFrom<char> for Takri {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𑚀' => Ok(Takri::LetterA),
            '𑚁' => Ok(Takri::LetterAa),
            '𑚂' => Ok(Takri::LetterI),
            '𑚃' => Ok(Takri::LetterIi),
            '𑚄' => Ok(Takri::LetterU),
            '𑚅' => Ok(Takri::LetterUu),
            '𑚆' => Ok(Takri::LetterE),
            '𑚇' => Ok(Takri::LetterAi),
            '𑚈' => Ok(Takri::LetterO),
            '𑚉' => Ok(Takri::LetterAu),
            '𑚊' => Ok(Takri::LetterKa),
            '𑚋' => Ok(Takri::LetterKha),
            '𑚌' => Ok(Takri::LetterGa),
            '𑚍' => Ok(Takri::LetterGha),
            '𑚎' => Ok(Takri::LetterNga),
            '𑚏' => Ok(Takri::LetterCa),
            '𑚐' => Ok(Takri::LetterCha),
            '𑚑' => Ok(Takri::LetterJa),
            '𑚒' => Ok(Takri::LetterJha),
            '𑚓' => Ok(Takri::LetterNya),
            '𑚔' => Ok(Takri::LetterTta),
            '𑚕' => Ok(Takri::LetterTtha),
            '𑚖' => Ok(Takri::LetterDda),
            '𑚗' => Ok(Takri::LetterDdha),
            '𑚘' => Ok(Takri::LetterNna),
            '𑚙' => Ok(Takri::LetterTa),
            '𑚚' => Ok(Takri::LetterTha),
            '𑚛' => Ok(Takri::LetterDa),
            '𑚜' => Ok(Takri::LetterDha),
            '𑚝' => Ok(Takri::LetterNa),
            '𑚞' => Ok(Takri::LetterPa),
            '𑚟' => Ok(Takri::LetterPha),
            '𑚠' => Ok(Takri::LetterBa),
            '𑚡' => Ok(Takri::LetterBha),
            '𑚢' => Ok(Takri::LetterMa),
            '𑚣' => Ok(Takri::LetterYa),
            '𑚤' => Ok(Takri::LetterRa),
            '𑚥' => Ok(Takri::LetterLa),
            '𑚦' => Ok(Takri::LetterVa),
            '𑚧' => Ok(Takri::LetterSha),
            '𑚨' => Ok(Takri::LetterSa),
            '𑚩' => Ok(Takri::LetterHa),
            '𑚪' => Ok(Takri::LetterRra),
            '𑚫' => Ok(Takri::SignAnusvara),
            '𑚬' => Ok(Takri::SignVisarga),
            '𑚭' => Ok(Takri::VowelSignAa),
            '𑚮' => Ok(Takri::VowelSignI),
            '𑚯' => Ok(Takri::VowelSignIi),
            '𑚰' => Ok(Takri::VowelSignU),
            '𑚱' => Ok(Takri::VowelSignUu),
            '𑚲' => Ok(Takri::VowelSignE),
            '𑚳' => Ok(Takri::VowelSignAi),
            '𑚴' => Ok(Takri::VowelSignO),
            '𑚵' => Ok(Takri::VowelSignAu),
            '𑚶' => Ok(Takri::SignVirama),
            '𑚷' => Ok(Takri::SignNukta),
            '𑚸' => Ok(Takri::LetterArchaicKha),
            '𑛀' => Ok(Takri::DigitZero),
            '𑛁' => Ok(Takri::DigitOne),
            '𑛂' => Ok(Takri::DigitTwo),
            '𑛃' => Ok(Takri::DigitThree),
            '𑛄' => Ok(Takri::DigitFour),
            '𑛅' => Ok(Takri::DigitFive),
            '𑛆' => Ok(Takri::DigitSix),
            '𑛇' => Ok(Takri::DigitSeven),
            '𑛈' => Ok(Takri::DigitEight),
            '𑛉' => Ok(Takri::DigitNine),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Takri {
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

impl std::convert::TryFrom<u32> for Takri {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Takri {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Takri {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Takri::LetterA
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Takri::LetterA => "takri letter a",
            Takri::LetterAa => "takri letter aa",
            Takri::LetterI => "takri letter i",
            Takri::LetterIi => "takri letter ii",
            Takri::LetterU => "takri letter u",
            Takri::LetterUu => "takri letter uu",
            Takri::LetterE => "takri letter e",
            Takri::LetterAi => "takri letter ai",
            Takri::LetterO => "takri letter o",
            Takri::LetterAu => "takri letter au",
            Takri::LetterKa => "takri letter ka",
            Takri::LetterKha => "takri letter kha",
            Takri::LetterGa => "takri letter ga",
            Takri::LetterGha => "takri letter gha",
            Takri::LetterNga => "takri letter nga",
            Takri::LetterCa => "takri letter ca",
            Takri::LetterCha => "takri letter cha",
            Takri::LetterJa => "takri letter ja",
            Takri::LetterJha => "takri letter jha",
            Takri::LetterNya => "takri letter nya",
            Takri::LetterTta => "takri letter tta",
            Takri::LetterTtha => "takri letter ttha",
            Takri::LetterDda => "takri letter dda",
            Takri::LetterDdha => "takri letter ddha",
            Takri::LetterNna => "takri letter nna",
            Takri::LetterTa => "takri letter ta",
            Takri::LetterTha => "takri letter tha",
            Takri::LetterDa => "takri letter da",
            Takri::LetterDha => "takri letter dha",
            Takri::LetterNa => "takri letter na",
            Takri::LetterPa => "takri letter pa",
            Takri::LetterPha => "takri letter pha",
            Takri::LetterBa => "takri letter ba",
            Takri::LetterBha => "takri letter bha",
            Takri::LetterMa => "takri letter ma",
            Takri::LetterYa => "takri letter ya",
            Takri::LetterRa => "takri letter ra",
            Takri::LetterLa => "takri letter la",
            Takri::LetterVa => "takri letter va",
            Takri::LetterSha => "takri letter sha",
            Takri::LetterSa => "takri letter sa",
            Takri::LetterHa => "takri letter ha",
            Takri::LetterRra => "takri letter rra",
            Takri::SignAnusvara => "takri sign anusvara",
            Takri::SignVisarga => "takri sign visarga",
            Takri::VowelSignAa => "takri vowel sign aa",
            Takri::VowelSignI => "takri vowel sign i",
            Takri::VowelSignIi => "takri vowel sign ii",
            Takri::VowelSignU => "takri vowel sign u",
            Takri::VowelSignUu => "takri vowel sign uu",
            Takri::VowelSignE => "takri vowel sign e",
            Takri::VowelSignAi => "takri vowel sign ai",
            Takri::VowelSignO => "takri vowel sign o",
            Takri::VowelSignAu => "takri vowel sign au",
            Takri::SignVirama => "takri sign virama",
            Takri::SignNukta => "takri sign nukta",
            Takri::LetterArchaicKha => "takri letter archaic kha",
            Takri::DigitZero => "takri digit zero",
            Takri::DigitOne => "takri digit one",
            Takri::DigitTwo => "takri digit two",
            Takri::DigitThree => "takri digit three",
            Takri::DigitFour => "takri digit four",
            Takri::DigitFive => "takri digit five",
            Takri::DigitSix => "takri digit six",
            Takri::DigitSeven => "takri digit seven",
            Takri::DigitEight => "takri digit eight",
            Takri::DigitNine => "takri digit nine",
        }
    }
}
