
/// An enum to represent all characters in the Dogra block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Dogra {
    /// \u{11800}: '𑠀'
    LetterA,
    /// \u{11801}: '𑠁'
    LetterAa,
    /// \u{11802}: '𑠂'
    LetterI,
    /// \u{11803}: '𑠃'
    LetterIi,
    /// \u{11804}: '𑠄'
    LetterU,
    /// \u{11805}: '𑠅'
    LetterUu,
    /// \u{11806}: '𑠆'
    LetterE,
    /// \u{11807}: '𑠇'
    LetterAi,
    /// \u{11808}: '𑠈'
    LetterO,
    /// \u{11809}: '𑠉'
    LetterAu,
    /// \u{1180a}: '𑠊'
    LetterKa,
    /// \u{1180b}: '𑠋'
    LetterKha,
    /// \u{1180c}: '𑠌'
    LetterGa,
    /// \u{1180d}: '𑠍'
    LetterGha,
    /// \u{1180e}: '𑠎'
    LetterNga,
    /// \u{1180f}: '𑠏'
    LetterCa,
    /// \u{11810}: '𑠐'
    LetterCha,
    /// \u{11811}: '𑠑'
    LetterJa,
    /// \u{11812}: '𑠒'
    LetterJha,
    /// \u{11813}: '𑠓'
    LetterNya,
    /// \u{11814}: '𑠔'
    LetterTta,
    /// \u{11815}: '𑠕'
    LetterTtha,
    /// \u{11816}: '𑠖'
    LetterDda,
    /// \u{11817}: '𑠗'
    LetterDdha,
    /// \u{11818}: '𑠘'
    LetterNna,
    /// \u{11819}: '𑠙'
    LetterTa,
    /// \u{1181a}: '𑠚'
    LetterTha,
    /// \u{1181b}: '𑠛'
    LetterDa,
    /// \u{1181c}: '𑠜'
    LetterDha,
    /// \u{1181d}: '𑠝'
    LetterNa,
    /// \u{1181e}: '𑠞'
    LetterPa,
    /// \u{1181f}: '𑠟'
    LetterPha,
    /// \u{11820}: '𑠠'
    LetterBa,
    /// \u{11821}: '𑠡'
    LetterBha,
    /// \u{11822}: '𑠢'
    LetterMa,
    /// \u{11823}: '𑠣'
    LetterYa,
    /// \u{11824}: '𑠤'
    LetterRa,
    /// \u{11825}: '𑠥'
    LetterLa,
    /// \u{11826}: '𑠦'
    LetterVa,
    /// \u{11827}: '𑠧'
    LetterSha,
    /// \u{11828}: '𑠨'
    LetterSsa,
    /// \u{11829}: '𑠩'
    LetterSa,
    /// \u{1182a}: '𑠪'
    LetterHa,
    /// \u{1182b}: '𑠫'
    LetterRra,
    /// \u{1182c}: '𑠬'
    VowelSignAa,
    /// \u{1182d}: '𑠭'
    VowelSignI,
    /// \u{1182e}: '𑠮'
    VowelSignIi,
    /// \u{1182f}: '𑠯'
    VowelSignU,
    /// \u{11830}: '𑠰'
    VowelSignUu,
    /// \u{11831}: '𑠱'
    VowelSignVocalicR,
    /// \u{11832}: '𑠲'
    VowelSignVocalicRr,
    /// \u{11833}: '𑠳'
    VowelSignE,
    /// \u{11834}: '𑠴'
    VowelSignAi,
    /// \u{11835}: '𑠵'
    VowelSignO,
    /// \u{11836}: '𑠶'
    VowelSignAu,
    /// \u{11837}: '𑠷'
    SignAnusvara,
    /// \u{11838}: '𑠸'
    SignVisarga,
    /// \u{11839}: '𑠹'
    SignVirama,
    /// \u{1183a}: '𑠺'
    SignNukta,
    /// \u{1183b}: '𑠻'
    AbbreviationSign,
}

impl Into<char> for Dogra {
    fn into(self) -> char {
        match self {
            Dogra::LetterA => '𑠀',
            Dogra::LetterAa => '𑠁',
            Dogra::LetterI => '𑠂',
            Dogra::LetterIi => '𑠃',
            Dogra::LetterU => '𑠄',
            Dogra::LetterUu => '𑠅',
            Dogra::LetterE => '𑠆',
            Dogra::LetterAi => '𑠇',
            Dogra::LetterO => '𑠈',
            Dogra::LetterAu => '𑠉',
            Dogra::LetterKa => '𑠊',
            Dogra::LetterKha => '𑠋',
            Dogra::LetterGa => '𑠌',
            Dogra::LetterGha => '𑠍',
            Dogra::LetterNga => '𑠎',
            Dogra::LetterCa => '𑠏',
            Dogra::LetterCha => '𑠐',
            Dogra::LetterJa => '𑠑',
            Dogra::LetterJha => '𑠒',
            Dogra::LetterNya => '𑠓',
            Dogra::LetterTta => '𑠔',
            Dogra::LetterTtha => '𑠕',
            Dogra::LetterDda => '𑠖',
            Dogra::LetterDdha => '𑠗',
            Dogra::LetterNna => '𑠘',
            Dogra::LetterTa => '𑠙',
            Dogra::LetterTha => '𑠚',
            Dogra::LetterDa => '𑠛',
            Dogra::LetterDha => '𑠜',
            Dogra::LetterNa => '𑠝',
            Dogra::LetterPa => '𑠞',
            Dogra::LetterPha => '𑠟',
            Dogra::LetterBa => '𑠠',
            Dogra::LetterBha => '𑠡',
            Dogra::LetterMa => '𑠢',
            Dogra::LetterYa => '𑠣',
            Dogra::LetterRa => '𑠤',
            Dogra::LetterLa => '𑠥',
            Dogra::LetterVa => '𑠦',
            Dogra::LetterSha => '𑠧',
            Dogra::LetterSsa => '𑠨',
            Dogra::LetterSa => '𑠩',
            Dogra::LetterHa => '𑠪',
            Dogra::LetterRra => '𑠫',
            Dogra::VowelSignAa => '𑠬',
            Dogra::VowelSignI => '𑠭',
            Dogra::VowelSignIi => '𑠮',
            Dogra::VowelSignU => '𑠯',
            Dogra::VowelSignUu => '𑠰',
            Dogra::VowelSignVocalicR => '𑠱',
            Dogra::VowelSignVocalicRr => '𑠲',
            Dogra::VowelSignE => '𑠳',
            Dogra::VowelSignAi => '𑠴',
            Dogra::VowelSignO => '𑠵',
            Dogra::VowelSignAu => '𑠶',
            Dogra::SignAnusvara => '𑠷',
            Dogra::SignVisarga => '𑠸',
            Dogra::SignVirama => '𑠹',
            Dogra::SignNukta => '𑠺',
            Dogra::AbbreviationSign => '𑠻',
        }
    }
}

impl std::convert::TryFrom<char> for Dogra {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𑠀' => Ok(Dogra::LetterA),
            '𑠁' => Ok(Dogra::LetterAa),
            '𑠂' => Ok(Dogra::LetterI),
            '𑠃' => Ok(Dogra::LetterIi),
            '𑠄' => Ok(Dogra::LetterU),
            '𑠅' => Ok(Dogra::LetterUu),
            '𑠆' => Ok(Dogra::LetterE),
            '𑠇' => Ok(Dogra::LetterAi),
            '𑠈' => Ok(Dogra::LetterO),
            '𑠉' => Ok(Dogra::LetterAu),
            '𑠊' => Ok(Dogra::LetterKa),
            '𑠋' => Ok(Dogra::LetterKha),
            '𑠌' => Ok(Dogra::LetterGa),
            '𑠍' => Ok(Dogra::LetterGha),
            '𑠎' => Ok(Dogra::LetterNga),
            '𑠏' => Ok(Dogra::LetterCa),
            '𑠐' => Ok(Dogra::LetterCha),
            '𑠑' => Ok(Dogra::LetterJa),
            '𑠒' => Ok(Dogra::LetterJha),
            '𑠓' => Ok(Dogra::LetterNya),
            '𑠔' => Ok(Dogra::LetterTta),
            '𑠕' => Ok(Dogra::LetterTtha),
            '𑠖' => Ok(Dogra::LetterDda),
            '𑠗' => Ok(Dogra::LetterDdha),
            '𑠘' => Ok(Dogra::LetterNna),
            '𑠙' => Ok(Dogra::LetterTa),
            '𑠚' => Ok(Dogra::LetterTha),
            '𑠛' => Ok(Dogra::LetterDa),
            '𑠜' => Ok(Dogra::LetterDha),
            '𑠝' => Ok(Dogra::LetterNa),
            '𑠞' => Ok(Dogra::LetterPa),
            '𑠟' => Ok(Dogra::LetterPha),
            '𑠠' => Ok(Dogra::LetterBa),
            '𑠡' => Ok(Dogra::LetterBha),
            '𑠢' => Ok(Dogra::LetterMa),
            '𑠣' => Ok(Dogra::LetterYa),
            '𑠤' => Ok(Dogra::LetterRa),
            '𑠥' => Ok(Dogra::LetterLa),
            '𑠦' => Ok(Dogra::LetterVa),
            '𑠧' => Ok(Dogra::LetterSha),
            '𑠨' => Ok(Dogra::LetterSsa),
            '𑠩' => Ok(Dogra::LetterSa),
            '𑠪' => Ok(Dogra::LetterHa),
            '𑠫' => Ok(Dogra::LetterRra),
            '𑠬' => Ok(Dogra::VowelSignAa),
            '𑠭' => Ok(Dogra::VowelSignI),
            '𑠮' => Ok(Dogra::VowelSignIi),
            '𑠯' => Ok(Dogra::VowelSignU),
            '𑠰' => Ok(Dogra::VowelSignUu),
            '𑠱' => Ok(Dogra::VowelSignVocalicR),
            '𑠲' => Ok(Dogra::VowelSignVocalicRr),
            '𑠳' => Ok(Dogra::VowelSignE),
            '𑠴' => Ok(Dogra::VowelSignAi),
            '𑠵' => Ok(Dogra::VowelSignO),
            '𑠶' => Ok(Dogra::VowelSignAu),
            '𑠷' => Ok(Dogra::SignAnusvara),
            '𑠸' => Ok(Dogra::SignVisarga),
            '𑠹' => Ok(Dogra::SignVirama),
            '𑠺' => Ok(Dogra::SignNukta),
            '𑠻' => Ok(Dogra::AbbreviationSign),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Dogra {
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

impl std::convert::TryFrom<u32> for Dogra {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Dogra {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Dogra {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Dogra::LetterA
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Dogra{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
