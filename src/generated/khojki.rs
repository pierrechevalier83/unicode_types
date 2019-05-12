
/// An enum to represent all characters in the Khojki block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Khojki {
    /// \u{11200}: '𑈀'
    LetterA,
    /// \u{11201}: '𑈁'
    LetterAa,
    /// \u{11202}: '𑈂'
    LetterI,
    /// \u{11203}: '𑈃'
    LetterU,
    /// \u{11204}: '𑈄'
    LetterE,
    /// \u{11205}: '𑈅'
    LetterAi,
    /// \u{11206}: '𑈆'
    LetterO,
    /// \u{11207}: '𑈇'
    LetterAu,
    /// \u{11208}: '𑈈'
    LetterKa,
    /// \u{11209}: '𑈉'
    LetterKha,
    /// \u{1120a}: '𑈊'
    LetterGa,
    /// \u{1120b}: '𑈋'
    LetterGga,
    /// \u{1120c}: '𑈌'
    LetterGha,
    /// \u{1120d}: '𑈍'
    LetterNga,
    /// \u{1120e}: '𑈎'
    LetterCa,
    /// \u{1120f}: '𑈏'
    LetterCha,
    /// \u{11210}: '𑈐'
    LetterJa,
    /// \u{11211}: '𑈑'
    LetterJja,
    /// \u{11213}: '𑈓'
    LetterNya,
    /// \u{11214}: '𑈔'
    LetterTta,
    /// \u{11215}: '𑈕'
    LetterTtha,
    /// \u{11216}: '𑈖'
    LetterDda,
    /// \u{11217}: '𑈗'
    LetterDdha,
    /// \u{11218}: '𑈘'
    LetterNna,
    /// \u{11219}: '𑈙'
    LetterTa,
    /// \u{1121a}: '𑈚'
    LetterTha,
    /// \u{1121b}: '𑈛'
    LetterDa,
    /// \u{1121c}: '𑈜'
    LetterDdda,
    /// \u{1121d}: '𑈝'
    LetterDha,
    /// \u{1121e}: '𑈞'
    LetterNa,
    /// \u{1121f}: '𑈟'
    LetterPa,
    /// \u{11220}: '𑈠'
    LetterPha,
    /// \u{11221}: '𑈡'
    LetterBa,
    /// \u{11222}: '𑈢'
    LetterBba,
    /// \u{11223}: '𑈣'
    LetterBha,
    /// \u{11224}: '𑈤'
    LetterMa,
    /// \u{11225}: '𑈥'
    LetterYa,
    /// \u{11226}: '𑈦'
    LetterRa,
    /// \u{11227}: '𑈧'
    LetterLa,
    /// \u{11228}: '𑈨'
    LetterVa,
    /// \u{11229}: '𑈩'
    LetterSa,
    /// \u{1122a}: '𑈪'
    LetterHa,
    /// \u{1122b}: '𑈫'
    LetterLla,
    /// \u{1122c}: '𑈬'
    VowelSignAa,
    /// \u{1122d}: '𑈭'
    VowelSignI,
    /// \u{1122e}: '𑈮'
    VowelSignIi,
    /// \u{1122f}: '𑈯'
    VowelSignU,
    /// \u{11230}: '𑈰'
    VowelSignE,
    /// \u{11231}: '𑈱'
    VowelSignAi,
    /// \u{11232}: '𑈲'
    VowelSignO,
    /// \u{11233}: '𑈳'
    VowelSignAu,
    /// \u{11234}: '𑈴'
    SignAnusvara,
    /// \u{11235}: '𑈵'
    SignVirama,
    /// \u{11236}: '𑈶'
    SignNukta,
    /// \u{11237}: '𑈷'
    SignShadda,
    /// \u{11238}: '𑈸'
    Danda,
    /// \u{11239}: '𑈹'
    DoubleDanda,
    /// \u{1123a}: '𑈺'
    WordSeparator,
    /// \u{1123b}: '𑈻'
    SectionMark,
    /// \u{1123c}: '𑈼'
    DoubleSectionMark,
    /// \u{1123d}: '𑈽'
    AbbreviationSign,
    /// \u{1123e}: '𑈾'
    SignSukun,
}

impl Into<char> for Khojki {
    fn into(self) -> char {
        match self {
            Khojki::LetterA => '𑈀',
            Khojki::LetterAa => '𑈁',
            Khojki::LetterI => '𑈂',
            Khojki::LetterU => '𑈃',
            Khojki::LetterE => '𑈄',
            Khojki::LetterAi => '𑈅',
            Khojki::LetterO => '𑈆',
            Khojki::LetterAu => '𑈇',
            Khojki::LetterKa => '𑈈',
            Khojki::LetterKha => '𑈉',
            Khojki::LetterGa => '𑈊',
            Khojki::LetterGga => '𑈋',
            Khojki::LetterGha => '𑈌',
            Khojki::LetterNga => '𑈍',
            Khojki::LetterCa => '𑈎',
            Khojki::LetterCha => '𑈏',
            Khojki::LetterJa => '𑈐',
            Khojki::LetterJja => '𑈑',
            Khojki::LetterNya => '𑈓',
            Khojki::LetterTta => '𑈔',
            Khojki::LetterTtha => '𑈕',
            Khojki::LetterDda => '𑈖',
            Khojki::LetterDdha => '𑈗',
            Khojki::LetterNna => '𑈘',
            Khojki::LetterTa => '𑈙',
            Khojki::LetterTha => '𑈚',
            Khojki::LetterDa => '𑈛',
            Khojki::LetterDdda => '𑈜',
            Khojki::LetterDha => '𑈝',
            Khojki::LetterNa => '𑈞',
            Khojki::LetterPa => '𑈟',
            Khojki::LetterPha => '𑈠',
            Khojki::LetterBa => '𑈡',
            Khojki::LetterBba => '𑈢',
            Khojki::LetterBha => '𑈣',
            Khojki::LetterMa => '𑈤',
            Khojki::LetterYa => '𑈥',
            Khojki::LetterRa => '𑈦',
            Khojki::LetterLa => '𑈧',
            Khojki::LetterVa => '𑈨',
            Khojki::LetterSa => '𑈩',
            Khojki::LetterHa => '𑈪',
            Khojki::LetterLla => '𑈫',
            Khojki::VowelSignAa => '𑈬',
            Khojki::VowelSignI => '𑈭',
            Khojki::VowelSignIi => '𑈮',
            Khojki::VowelSignU => '𑈯',
            Khojki::VowelSignE => '𑈰',
            Khojki::VowelSignAi => '𑈱',
            Khojki::VowelSignO => '𑈲',
            Khojki::VowelSignAu => '𑈳',
            Khojki::SignAnusvara => '𑈴',
            Khojki::SignVirama => '𑈵',
            Khojki::SignNukta => '𑈶',
            Khojki::SignShadda => '𑈷',
            Khojki::Danda => '𑈸',
            Khojki::DoubleDanda => '𑈹',
            Khojki::WordSeparator => '𑈺',
            Khojki::SectionMark => '𑈻',
            Khojki::DoubleSectionMark => '𑈼',
            Khojki::AbbreviationSign => '𑈽',
            Khojki::SignSukun => '𑈾',
        }
    }
}

impl std::convert::TryFrom<char> for Khojki {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𑈀' => Ok(Khojki::LetterA),
            '𑈁' => Ok(Khojki::LetterAa),
            '𑈂' => Ok(Khojki::LetterI),
            '𑈃' => Ok(Khojki::LetterU),
            '𑈄' => Ok(Khojki::LetterE),
            '𑈅' => Ok(Khojki::LetterAi),
            '𑈆' => Ok(Khojki::LetterO),
            '𑈇' => Ok(Khojki::LetterAu),
            '𑈈' => Ok(Khojki::LetterKa),
            '𑈉' => Ok(Khojki::LetterKha),
            '𑈊' => Ok(Khojki::LetterGa),
            '𑈋' => Ok(Khojki::LetterGga),
            '𑈌' => Ok(Khojki::LetterGha),
            '𑈍' => Ok(Khojki::LetterNga),
            '𑈎' => Ok(Khojki::LetterCa),
            '𑈏' => Ok(Khojki::LetterCha),
            '𑈐' => Ok(Khojki::LetterJa),
            '𑈑' => Ok(Khojki::LetterJja),
            '𑈓' => Ok(Khojki::LetterNya),
            '𑈔' => Ok(Khojki::LetterTta),
            '𑈕' => Ok(Khojki::LetterTtha),
            '𑈖' => Ok(Khojki::LetterDda),
            '𑈗' => Ok(Khojki::LetterDdha),
            '𑈘' => Ok(Khojki::LetterNna),
            '𑈙' => Ok(Khojki::LetterTa),
            '𑈚' => Ok(Khojki::LetterTha),
            '𑈛' => Ok(Khojki::LetterDa),
            '𑈜' => Ok(Khojki::LetterDdda),
            '𑈝' => Ok(Khojki::LetterDha),
            '𑈞' => Ok(Khojki::LetterNa),
            '𑈟' => Ok(Khojki::LetterPa),
            '𑈠' => Ok(Khojki::LetterPha),
            '𑈡' => Ok(Khojki::LetterBa),
            '𑈢' => Ok(Khojki::LetterBba),
            '𑈣' => Ok(Khojki::LetterBha),
            '𑈤' => Ok(Khojki::LetterMa),
            '𑈥' => Ok(Khojki::LetterYa),
            '𑈦' => Ok(Khojki::LetterRa),
            '𑈧' => Ok(Khojki::LetterLa),
            '𑈨' => Ok(Khojki::LetterVa),
            '𑈩' => Ok(Khojki::LetterSa),
            '𑈪' => Ok(Khojki::LetterHa),
            '𑈫' => Ok(Khojki::LetterLla),
            '𑈬' => Ok(Khojki::VowelSignAa),
            '𑈭' => Ok(Khojki::VowelSignI),
            '𑈮' => Ok(Khojki::VowelSignIi),
            '𑈯' => Ok(Khojki::VowelSignU),
            '𑈰' => Ok(Khojki::VowelSignE),
            '𑈱' => Ok(Khojki::VowelSignAi),
            '𑈲' => Ok(Khojki::VowelSignO),
            '𑈳' => Ok(Khojki::VowelSignAu),
            '𑈴' => Ok(Khojki::SignAnusvara),
            '𑈵' => Ok(Khojki::SignVirama),
            '𑈶' => Ok(Khojki::SignNukta),
            '𑈷' => Ok(Khojki::SignShadda),
            '𑈸' => Ok(Khojki::Danda),
            '𑈹' => Ok(Khojki::DoubleDanda),
            '𑈺' => Ok(Khojki::WordSeparator),
            '𑈻' => Ok(Khojki::SectionMark),
            '𑈼' => Ok(Khojki::DoubleSectionMark),
            '𑈽' => Ok(Khojki::AbbreviationSign),
            '𑈾' => Ok(Khojki::SignSukun),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Khojki {
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

impl std::convert::TryFrom<u32> for Khojki {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Khojki {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Khojki {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Khojki::LetterA
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Khojki{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
