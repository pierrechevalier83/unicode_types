
/// An enum to represent all characters in the Khudawadi block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Khudawadi {
    /// \u{112b0}: '𑊰'
    LetterA,
    /// \u{112b1}: '𑊱'
    LetterAa,
    /// \u{112b2}: '𑊲'
    LetterI,
    /// \u{112b3}: '𑊳'
    LetterIi,
    /// \u{112b4}: '𑊴'
    LetterU,
    /// \u{112b5}: '𑊵'
    LetterUu,
    /// \u{112b6}: '𑊶'
    LetterE,
    /// \u{112b7}: '𑊷'
    LetterAi,
    /// \u{112b8}: '𑊸'
    LetterO,
    /// \u{112b9}: '𑊹'
    LetterAu,
    /// \u{112ba}: '𑊺'
    LetterKa,
    /// \u{112bb}: '𑊻'
    LetterKha,
    /// \u{112bc}: '𑊼'
    LetterGa,
    /// \u{112bd}: '𑊽'
    LetterGga,
    /// \u{112be}: '𑊾'
    LetterGha,
    /// \u{112bf}: '𑊿'
    LetterNga,
    /// \u{112c0}: '𑋀'
    LetterCa,
    /// \u{112c1}: '𑋁'
    LetterCha,
    /// \u{112c2}: '𑋂'
    LetterJa,
    /// \u{112c3}: '𑋃'
    LetterJja,
    /// \u{112c4}: '𑋄'
    LetterJha,
    /// \u{112c5}: '𑋅'
    LetterNya,
    /// \u{112c6}: '𑋆'
    LetterTta,
    /// \u{112c7}: '𑋇'
    LetterTtha,
    /// \u{112c8}: '𑋈'
    LetterDda,
    /// \u{112c9}: '𑋉'
    LetterDdda,
    /// \u{112ca}: '𑋊'
    LetterRra,
    /// \u{112cb}: '𑋋'
    LetterDdha,
    /// \u{112cc}: '𑋌'
    LetterNna,
    /// \u{112cd}: '𑋍'
    LetterTa,
    /// \u{112ce}: '𑋎'
    LetterTha,
    /// \u{112cf}: '𑋏'
    LetterDa,
    /// \u{112d0}: '𑋐'
    LetterDha,
    /// \u{112d1}: '𑋑'
    LetterNa,
    /// \u{112d2}: '𑋒'
    LetterPa,
    /// \u{112d3}: '𑋓'
    LetterPha,
    /// \u{112d4}: '𑋔'
    LetterBa,
    /// \u{112d5}: '𑋕'
    LetterBba,
    /// \u{112d6}: '𑋖'
    LetterBha,
    /// \u{112d7}: '𑋗'
    LetterMa,
    /// \u{112d8}: '𑋘'
    LetterYa,
    /// \u{112d9}: '𑋙'
    LetterRa,
    /// \u{112da}: '𑋚'
    LetterLa,
    /// \u{112db}: '𑋛'
    LetterVa,
    /// \u{112dc}: '𑋜'
    LetterSha,
    /// \u{112dd}: '𑋝'
    LetterSa,
    /// \u{112de}: '𑋞'
    LetterHa,
    /// \u{112df}: '𑋟'
    SignAnusvara,
    /// \u{112e0}: '𑋠'
    VowelSignAa,
    /// \u{112e1}: '𑋡'
    VowelSignI,
    /// \u{112e2}: '𑋢'
    VowelSignIi,
    /// \u{112e3}: '𑋣'
    VowelSignU,
    /// \u{112e4}: '𑋤'
    VowelSignUu,
    /// \u{112e5}: '𑋥'
    VowelSignE,
    /// \u{112e6}: '𑋦'
    VowelSignAi,
    /// \u{112e7}: '𑋧'
    VowelSignO,
    /// \u{112e8}: '𑋨'
    VowelSignAu,
    /// \u{112e9}: '𑋩'
    SignNukta,
    /// \u{112ea}: '𑋪'
    SignVirama,
    /// \u{112f0}: '𑋰'
    DigitZero,
    /// \u{112f1}: '𑋱'
    DigitOne,
    /// \u{112f2}: '𑋲'
    DigitTwo,
    /// \u{112f3}: '𑋳'
    DigitThree,
    /// \u{112f4}: '𑋴'
    DigitFour,
    /// \u{112f5}: '𑋵'
    DigitFive,
    /// \u{112f6}: '𑋶'
    DigitSix,
    /// \u{112f7}: '𑋷'
    DigitSeven,
    /// \u{112f8}: '𑋸'
    DigitEight,
    /// \u{112f9}: '𑋹'
    DigitNine,
}

impl Into<char> for Khudawadi {
    fn into(self) -> char {
        match self {
            Khudawadi::LetterA => '𑊰',
            Khudawadi::LetterAa => '𑊱',
            Khudawadi::LetterI => '𑊲',
            Khudawadi::LetterIi => '𑊳',
            Khudawadi::LetterU => '𑊴',
            Khudawadi::LetterUu => '𑊵',
            Khudawadi::LetterE => '𑊶',
            Khudawadi::LetterAi => '𑊷',
            Khudawadi::LetterO => '𑊸',
            Khudawadi::LetterAu => '𑊹',
            Khudawadi::LetterKa => '𑊺',
            Khudawadi::LetterKha => '𑊻',
            Khudawadi::LetterGa => '𑊼',
            Khudawadi::LetterGga => '𑊽',
            Khudawadi::LetterGha => '𑊾',
            Khudawadi::LetterNga => '𑊿',
            Khudawadi::LetterCa => '𑋀',
            Khudawadi::LetterCha => '𑋁',
            Khudawadi::LetterJa => '𑋂',
            Khudawadi::LetterJja => '𑋃',
            Khudawadi::LetterJha => '𑋄',
            Khudawadi::LetterNya => '𑋅',
            Khudawadi::LetterTta => '𑋆',
            Khudawadi::LetterTtha => '𑋇',
            Khudawadi::LetterDda => '𑋈',
            Khudawadi::LetterDdda => '𑋉',
            Khudawadi::LetterRra => '𑋊',
            Khudawadi::LetterDdha => '𑋋',
            Khudawadi::LetterNna => '𑋌',
            Khudawadi::LetterTa => '𑋍',
            Khudawadi::LetterTha => '𑋎',
            Khudawadi::LetterDa => '𑋏',
            Khudawadi::LetterDha => '𑋐',
            Khudawadi::LetterNa => '𑋑',
            Khudawadi::LetterPa => '𑋒',
            Khudawadi::LetterPha => '𑋓',
            Khudawadi::LetterBa => '𑋔',
            Khudawadi::LetterBba => '𑋕',
            Khudawadi::LetterBha => '𑋖',
            Khudawadi::LetterMa => '𑋗',
            Khudawadi::LetterYa => '𑋘',
            Khudawadi::LetterRa => '𑋙',
            Khudawadi::LetterLa => '𑋚',
            Khudawadi::LetterVa => '𑋛',
            Khudawadi::LetterSha => '𑋜',
            Khudawadi::LetterSa => '𑋝',
            Khudawadi::LetterHa => '𑋞',
            Khudawadi::SignAnusvara => '𑋟',
            Khudawadi::VowelSignAa => '𑋠',
            Khudawadi::VowelSignI => '𑋡',
            Khudawadi::VowelSignIi => '𑋢',
            Khudawadi::VowelSignU => '𑋣',
            Khudawadi::VowelSignUu => '𑋤',
            Khudawadi::VowelSignE => '𑋥',
            Khudawadi::VowelSignAi => '𑋦',
            Khudawadi::VowelSignO => '𑋧',
            Khudawadi::VowelSignAu => '𑋨',
            Khudawadi::SignNukta => '𑋩',
            Khudawadi::SignVirama => '𑋪',
            Khudawadi::DigitZero => '𑋰',
            Khudawadi::DigitOne => '𑋱',
            Khudawadi::DigitTwo => '𑋲',
            Khudawadi::DigitThree => '𑋳',
            Khudawadi::DigitFour => '𑋴',
            Khudawadi::DigitFive => '𑋵',
            Khudawadi::DigitSix => '𑋶',
            Khudawadi::DigitSeven => '𑋷',
            Khudawadi::DigitEight => '𑋸',
            Khudawadi::DigitNine => '𑋹',
        }
    }
}

impl std::convert::TryFrom<char> for Khudawadi {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𑊰' => Ok(Khudawadi::LetterA),
            '𑊱' => Ok(Khudawadi::LetterAa),
            '𑊲' => Ok(Khudawadi::LetterI),
            '𑊳' => Ok(Khudawadi::LetterIi),
            '𑊴' => Ok(Khudawadi::LetterU),
            '𑊵' => Ok(Khudawadi::LetterUu),
            '𑊶' => Ok(Khudawadi::LetterE),
            '𑊷' => Ok(Khudawadi::LetterAi),
            '𑊸' => Ok(Khudawadi::LetterO),
            '𑊹' => Ok(Khudawadi::LetterAu),
            '𑊺' => Ok(Khudawadi::LetterKa),
            '𑊻' => Ok(Khudawadi::LetterKha),
            '𑊼' => Ok(Khudawadi::LetterGa),
            '𑊽' => Ok(Khudawadi::LetterGga),
            '𑊾' => Ok(Khudawadi::LetterGha),
            '𑊿' => Ok(Khudawadi::LetterNga),
            '𑋀' => Ok(Khudawadi::LetterCa),
            '𑋁' => Ok(Khudawadi::LetterCha),
            '𑋂' => Ok(Khudawadi::LetterJa),
            '𑋃' => Ok(Khudawadi::LetterJja),
            '𑋄' => Ok(Khudawadi::LetterJha),
            '𑋅' => Ok(Khudawadi::LetterNya),
            '𑋆' => Ok(Khudawadi::LetterTta),
            '𑋇' => Ok(Khudawadi::LetterTtha),
            '𑋈' => Ok(Khudawadi::LetterDda),
            '𑋉' => Ok(Khudawadi::LetterDdda),
            '𑋊' => Ok(Khudawadi::LetterRra),
            '𑋋' => Ok(Khudawadi::LetterDdha),
            '𑋌' => Ok(Khudawadi::LetterNna),
            '𑋍' => Ok(Khudawadi::LetterTa),
            '𑋎' => Ok(Khudawadi::LetterTha),
            '𑋏' => Ok(Khudawadi::LetterDa),
            '𑋐' => Ok(Khudawadi::LetterDha),
            '𑋑' => Ok(Khudawadi::LetterNa),
            '𑋒' => Ok(Khudawadi::LetterPa),
            '𑋓' => Ok(Khudawadi::LetterPha),
            '𑋔' => Ok(Khudawadi::LetterBa),
            '𑋕' => Ok(Khudawadi::LetterBba),
            '𑋖' => Ok(Khudawadi::LetterBha),
            '𑋗' => Ok(Khudawadi::LetterMa),
            '𑋘' => Ok(Khudawadi::LetterYa),
            '𑋙' => Ok(Khudawadi::LetterRa),
            '𑋚' => Ok(Khudawadi::LetterLa),
            '𑋛' => Ok(Khudawadi::LetterVa),
            '𑋜' => Ok(Khudawadi::LetterSha),
            '𑋝' => Ok(Khudawadi::LetterSa),
            '𑋞' => Ok(Khudawadi::LetterHa),
            '𑋟' => Ok(Khudawadi::SignAnusvara),
            '𑋠' => Ok(Khudawadi::VowelSignAa),
            '𑋡' => Ok(Khudawadi::VowelSignI),
            '𑋢' => Ok(Khudawadi::VowelSignIi),
            '𑋣' => Ok(Khudawadi::VowelSignU),
            '𑋤' => Ok(Khudawadi::VowelSignUu),
            '𑋥' => Ok(Khudawadi::VowelSignE),
            '𑋦' => Ok(Khudawadi::VowelSignAi),
            '𑋧' => Ok(Khudawadi::VowelSignO),
            '𑋨' => Ok(Khudawadi::VowelSignAu),
            '𑋩' => Ok(Khudawadi::SignNukta),
            '𑋪' => Ok(Khudawadi::SignVirama),
            '𑋰' => Ok(Khudawadi::DigitZero),
            '𑋱' => Ok(Khudawadi::DigitOne),
            '𑋲' => Ok(Khudawadi::DigitTwo),
            '𑋳' => Ok(Khudawadi::DigitThree),
            '𑋴' => Ok(Khudawadi::DigitFour),
            '𑋵' => Ok(Khudawadi::DigitFive),
            '𑋶' => Ok(Khudawadi::DigitSix),
            '𑋷' => Ok(Khudawadi::DigitSeven),
            '𑋸' => Ok(Khudawadi::DigitEight),
            '𑋹' => Ok(Khudawadi::DigitNine),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Khudawadi {
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

impl std::convert::TryFrom<u32> for Khudawadi {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Khudawadi {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Khudawadi {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Khudawadi::LetterA
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Khudawadi::LetterA => "khudawadi letter a",
            Khudawadi::LetterAa => "khudawadi letter aa",
            Khudawadi::LetterI => "khudawadi letter i",
            Khudawadi::LetterIi => "khudawadi letter ii",
            Khudawadi::LetterU => "khudawadi letter u",
            Khudawadi::LetterUu => "khudawadi letter uu",
            Khudawadi::LetterE => "khudawadi letter e",
            Khudawadi::LetterAi => "khudawadi letter ai",
            Khudawadi::LetterO => "khudawadi letter o",
            Khudawadi::LetterAu => "khudawadi letter au",
            Khudawadi::LetterKa => "khudawadi letter ka",
            Khudawadi::LetterKha => "khudawadi letter kha",
            Khudawadi::LetterGa => "khudawadi letter ga",
            Khudawadi::LetterGga => "khudawadi letter gga",
            Khudawadi::LetterGha => "khudawadi letter gha",
            Khudawadi::LetterNga => "khudawadi letter nga",
            Khudawadi::LetterCa => "khudawadi letter ca",
            Khudawadi::LetterCha => "khudawadi letter cha",
            Khudawadi::LetterJa => "khudawadi letter ja",
            Khudawadi::LetterJja => "khudawadi letter jja",
            Khudawadi::LetterJha => "khudawadi letter jha",
            Khudawadi::LetterNya => "khudawadi letter nya",
            Khudawadi::LetterTta => "khudawadi letter tta",
            Khudawadi::LetterTtha => "khudawadi letter ttha",
            Khudawadi::LetterDda => "khudawadi letter dda",
            Khudawadi::LetterDdda => "khudawadi letter ddda",
            Khudawadi::LetterRra => "khudawadi letter rra",
            Khudawadi::LetterDdha => "khudawadi letter ddha",
            Khudawadi::LetterNna => "khudawadi letter nna",
            Khudawadi::LetterTa => "khudawadi letter ta",
            Khudawadi::LetterTha => "khudawadi letter tha",
            Khudawadi::LetterDa => "khudawadi letter da",
            Khudawadi::LetterDha => "khudawadi letter dha",
            Khudawadi::LetterNa => "khudawadi letter na",
            Khudawadi::LetterPa => "khudawadi letter pa",
            Khudawadi::LetterPha => "khudawadi letter pha",
            Khudawadi::LetterBa => "khudawadi letter ba",
            Khudawadi::LetterBba => "khudawadi letter bba",
            Khudawadi::LetterBha => "khudawadi letter bha",
            Khudawadi::LetterMa => "khudawadi letter ma",
            Khudawadi::LetterYa => "khudawadi letter ya",
            Khudawadi::LetterRa => "khudawadi letter ra",
            Khudawadi::LetterLa => "khudawadi letter la",
            Khudawadi::LetterVa => "khudawadi letter va",
            Khudawadi::LetterSha => "khudawadi letter sha",
            Khudawadi::LetterSa => "khudawadi letter sa",
            Khudawadi::LetterHa => "khudawadi letter ha",
            Khudawadi::SignAnusvara => "khudawadi sign anusvara",
            Khudawadi::VowelSignAa => "khudawadi vowel sign aa",
            Khudawadi::VowelSignI => "khudawadi vowel sign i",
            Khudawadi::VowelSignIi => "khudawadi vowel sign ii",
            Khudawadi::VowelSignU => "khudawadi vowel sign u",
            Khudawadi::VowelSignUu => "khudawadi vowel sign uu",
            Khudawadi::VowelSignE => "khudawadi vowel sign e",
            Khudawadi::VowelSignAi => "khudawadi vowel sign ai",
            Khudawadi::VowelSignO => "khudawadi vowel sign o",
            Khudawadi::VowelSignAu => "khudawadi vowel sign au",
            Khudawadi::SignNukta => "khudawadi sign nukta",
            Khudawadi::SignVirama => "khudawadi sign virama",
            Khudawadi::DigitZero => "khudawadi digit zero",
            Khudawadi::DigitOne => "khudawadi digit one",
            Khudawadi::DigitTwo => "khudawadi digit two",
            Khudawadi::DigitThree => "khudawadi digit three",
            Khudawadi::DigitFour => "khudawadi digit four",
            Khudawadi::DigitFive => "khudawadi digit five",
            Khudawadi::DigitSix => "khudawadi digit six",
            Khudawadi::DigitSeven => "khudawadi digit seven",
            Khudawadi::DigitEight => "khudawadi digit eight",
            Khudawadi::DigitNine => "khudawadi digit nine",
        }
    }
}
