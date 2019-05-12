
/// An enum to represent all characters in the Sharada block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Sharada {
    /// \u{11180}: '𑆀'
    SignCandrabindu,
    /// \u{11181}: '𑆁'
    SignAnusvara,
    /// \u{11182}: '𑆂'
    SignVisarga,
    /// \u{11183}: '𑆃'
    LetterA,
    /// \u{11184}: '𑆄'
    LetterAa,
    /// \u{11185}: '𑆅'
    LetterI,
    /// \u{11186}: '𑆆'
    LetterIi,
    /// \u{11187}: '𑆇'
    LetterU,
    /// \u{11188}: '𑆈'
    LetterUu,
    /// \u{11189}: '𑆉'
    LetterVocalicR,
    /// \u{1118a}: '𑆊'
    LetterVocalicRr,
    /// \u{1118b}: '𑆋'
    LetterVocalicL,
    /// \u{1118c}: '𑆌'
    LetterVocalicLl,
    /// \u{1118d}: '𑆍'
    LetterE,
    /// \u{1118e}: '𑆎'
    LetterAi,
    /// \u{1118f}: '𑆏'
    LetterO,
    /// \u{11190}: '𑆐'
    LetterAu,
    /// \u{11191}: '𑆑'
    LetterKa,
    /// \u{11192}: '𑆒'
    LetterKha,
    /// \u{11193}: '𑆓'
    LetterGa,
    /// \u{11194}: '𑆔'
    LetterGha,
    /// \u{11195}: '𑆕'
    LetterNga,
    /// \u{11196}: '𑆖'
    LetterCa,
    /// \u{11197}: '𑆗'
    LetterCha,
    /// \u{11198}: '𑆘'
    LetterJa,
    /// \u{11199}: '𑆙'
    LetterJha,
    /// \u{1119a}: '𑆚'
    LetterNya,
    /// \u{1119b}: '𑆛'
    LetterTta,
    /// \u{1119c}: '𑆜'
    LetterTtha,
    /// \u{1119d}: '𑆝'
    LetterDda,
    /// \u{1119e}: '𑆞'
    LetterDdha,
    /// \u{1119f}: '𑆟'
    LetterNna,
    /// \u{111a0}: '𑆠'
    LetterTa,
    /// \u{111a1}: '𑆡'
    LetterTha,
    /// \u{111a2}: '𑆢'
    LetterDa,
    /// \u{111a3}: '𑆣'
    LetterDha,
    /// \u{111a4}: '𑆤'
    LetterNa,
    /// \u{111a5}: '𑆥'
    LetterPa,
    /// \u{111a6}: '𑆦'
    LetterPha,
    /// \u{111a7}: '𑆧'
    LetterBa,
    /// \u{111a8}: '𑆨'
    LetterBha,
    /// \u{111a9}: '𑆩'
    LetterMa,
    /// \u{111aa}: '𑆪'
    LetterYa,
    /// \u{111ab}: '𑆫'
    LetterRa,
    /// \u{111ac}: '𑆬'
    LetterLa,
    /// \u{111ad}: '𑆭'
    LetterLla,
    /// \u{111ae}: '𑆮'
    LetterVa,
    /// \u{111af}: '𑆯'
    LetterSha,
    /// \u{111b0}: '𑆰'
    LetterSsa,
    /// \u{111b1}: '𑆱'
    LetterSa,
    /// \u{111b2}: '𑆲'
    LetterHa,
    /// \u{111b3}: '𑆳'
    VowelSignAa,
    /// \u{111b4}: '𑆴'
    VowelSignI,
    /// \u{111b5}: '𑆵'
    VowelSignIi,
    /// \u{111b6}: '𑆶'
    VowelSignU,
    /// \u{111b7}: '𑆷'
    VowelSignUu,
    /// \u{111b8}: '𑆸'
    VowelSignVocalicR,
    /// \u{111b9}: '𑆹'
    VowelSignVocalicRr,
    /// \u{111ba}: '𑆺'
    VowelSignVocalicL,
    /// \u{111bb}: '𑆻'
    VowelSignVocalicLl,
    /// \u{111bc}: '𑆼'
    VowelSignE,
    /// \u{111bd}: '𑆽'
    VowelSignAi,
    /// \u{111be}: '𑆾'
    VowelSignO,
    /// \u{111bf}: '𑆿'
    VowelSignAu,
    /// \u{111c0}: '𑇀'
    SignVirama,
    /// \u{111c1}: '𑇁'
    SignAvagraha,
    /// \u{111c2}: '𑇂'
    SignJihvamuliya,
    /// \u{111c3}: '𑇃'
    SignUpadhmaniya,
    /// \u{111c4}: '𑇄'
    Om,
    /// \u{111c5}: '𑇅'
    Danda,
    /// \u{111c6}: '𑇆'
    DoubleDanda,
    /// \u{111c7}: '𑇇'
    AbbreviationSign,
    /// \u{111c8}: '𑇈'
    Separator,
    /// \u{111c9}: '𑇉'
    SandhiMark,
    /// \u{111ca}: '𑇊'
    SignNukta,
    /// \u{111cb}: '𑇋'
    VowelModifierMark,
    /// \u{111cc}: '𑇌'
    ExtraShortVowelMark,
    /// \u{111cd}: '𑇍'
    SutraMark,
    /// \u{111d0}: '𑇐'
    DigitZero,
    /// \u{111d1}: '𑇑'
    DigitOne,
    /// \u{111d2}: '𑇒'
    DigitTwo,
    /// \u{111d3}: '𑇓'
    DigitThree,
    /// \u{111d4}: '𑇔'
    DigitFour,
    /// \u{111d5}: '𑇕'
    DigitFive,
    /// \u{111d6}: '𑇖'
    DigitSix,
    /// \u{111d7}: '𑇗'
    DigitSeven,
    /// \u{111d8}: '𑇘'
    DigitEight,
    /// \u{111d9}: '𑇙'
    DigitNine,
    /// \u{111da}: '𑇚'
    Ekam,
    /// \u{111db}: '𑇛'
    SignSiddham,
    /// \u{111dc}: '𑇜'
    Headstroke,
    /// \u{111dd}: '𑇝'
    ContinuationSign,
    /// \u{111de}: '𑇞'
    SectionMarkDash1,
}

impl Into<char> for Sharada {
    fn into(self) -> char {
        match self {
            Sharada::SignCandrabindu => '𑆀',
            Sharada::SignAnusvara => '𑆁',
            Sharada::SignVisarga => '𑆂',
            Sharada::LetterA => '𑆃',
            Sharada::LetterAa => '𑆄',
            Sharada::LetterI => '𑆅',
            Sharada::LetterIi => '𑆆',
            Sharada::LetterU => '𑆇',
            Sharada::LetterUu => '𑆈',
            Sharada::LetterVocalicR => '𑆉',
            Sharada::LetterVocalicRr => '𑆊',
            Sharada::LetterVocalicL => '𑆋',
            Sharada::LetterVocalicLl => '𑆌',
            Sharada::LetterE => '𑆍',
            Sharada::LetterAi => '𑆎',
            Sharada::LetterO => '𑆏',
            Sharada::LetterAu => '𑆐',
            Sharada::LetterKa => '𑆑',
            Sharada::LetterKha => '𑆒',
            Sharada::LetterGa => '𑆓',
            Sharada::LetterGha => '𑆔',
            Sharada::LetterNga => '𑆕',
            Sharada::LetterCa => '𑆖',
            Sharada::LetterCha => '𑆗',
            Sharada::LetterJa => '𑆘',
            Sharada::LetterJha => '𑆙',
            Sharada::LetterNya => '𑆚',
            Sharada::LetterTta => '𑆛',
            Sharada::LetterTtha => '𑆜',
            Sharada::LetterDda => '𑆝',
            Sharada::LetterDdha => '𑆞',
            Sharada::LetterNna => '𑆟',
            Sharada::LetterTa => '𑆠',
            Sharada::LetterTha => '𑆡',
            Sharada::LetterDa => '𑆢',
            Sharada::LetterDha => '𑆣',
            Sharada::LetterNa => '𑆤',
            Sharada::LetterPa => '𑆥',
            Sharada::LetterPha => '𑆦',
            Sharada::LetterBa => '𑆧',
            Sharada::LetterBha => '𑆨',
            Sharada::LetterMa => '𑆩',
            Sharada::LetterYa => '𑆪',
            Sharada::LetterRa => '𑆫',
            Sharada::LetterLa => '𑆬',
            Sharada::LetterLla => '𑆭',
            Sharada::LetterVa => '𑆮',
            Sharada::LetterSha => '𑆯',
            Sharada::LetterSsa => '𑆰',
            Sharada::LetterSa => '𑆱',
            Sharada::LetterHa => '𑆲',
            Sharada::VowelSignAa => '𑆳',
            Sharada::VowelSignI => '𑆴',
            Sharada::VowelSignIi => '𑆵',
            Sharada::VowelSignU => '𑆶',
            Sharada::VowelSignUu => '𑆷',
            Sharada::VowelSignVocalicR => '𑆸',
            Sharada::VowelSignVocalicRr => '𑆹',
            Sharada::VowelSignVocalicL => '𑆺',
            Sharada::VowelSignVocalicLl => '𑆻',
            Sharada::VowelSignE => '𑆼',
            Sharada::VowelSignAi => '𑆽',
            Sharada::VowelSignO => '𑆾',
            Sharada::VowelSignAu => '𑆿',
            Sharada::SignVirama => '𑇀',
            Sharada::SignAvagraha => '𑇁',
            Sharada::SignJihvamuliya => '𑇂',
            Sharada::SignUpadhmaniya => '𑇃',
            Sharada::Om => '𑇄',
            Sharada::Danda => '𑇅',
            Sharada::DoubleDanda => '𑇆',
            Sharada::AbbreviationSign => '𑇇',
            Sharada::Separator => '𑇈',
            Sharada::SandhiMark => '𑇉',
            Sharada::SignNukta => '𑇊',
            Sharada::VowelModifierMark => '𑇋',
            Sharada::ExtraShortVowelMark => '𑇌',
            Sharada::SutraMark => '𑇍',
            Sharada::DigitZero => '𑇐',
            Sharada::DigitOne => '𑇑',
            Sharada::DigitTwo => '𑇒',
            Sharada::DigitThree => '𑇓',
            Sharada::DigitFour => '𑇔',
            Sharada::DigitFive => '𑇕',
            Sharada::DigitSix => '𑇖',
            Sharada::DigitSeven => '𑇗',
            Sharada::DigitEight => '𑇘',
            Sharada::DigitNine => '𑇙',
            Sharada::Ekam => '𑇚',
            Sharada::SignSiddham => '𑇛',
            Sharada::Headstroke => '𑇜',
            Sharada::ContinuationSign => '𑇝',
            Sharada::SectionMarkDash1 => '𑇞',
        }
    }
}

impl std::convert::TryFrom<char> for Sharada {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𑆀' => Ok(Sharada::SignCandrabindu),
            '𑆁' => Ok(Sharada::SignAnusvara),
            '𑆂' => Ok(Sharada::SignVisarga),
            '𑆃' => Ok(Sharada::LetterA),
            '𑆄' => Ok(Sharada::LetterAa),
            '𑆅' => Ok(Sharada::LetterI),
            '𑆆' => Ok(Sharada::LetterIi),
            '𑆇' => Ok(Sharada::LetterU),
            '𑆈' => Ok(Sharada::LetterUu),
            '𑆉' => Ok(Sharada::LetterVocalicR),
            '𑆊' => Ok(Sharada::LetterVocalicRr),
            '𑆋' => Ok(Sharada::LetterVocalicL),
            '𑆌' => Ok(Sharada::LetterVocalicLl),
            '𑆍' => Ok(Sharada::LetterE),
            '𑆎' => Ok(Sharada::LetterAi),
            '𑆏' => Ok(Sharada::LetterO),
            '𑆐' => Ok(Sharada::LetterAu),
            '𑆑' => Ok(Sharada::LetterKa),
            '𑆒' => Ok(Sharada::LetterKha),
            '𑆓' => Ok(Sharada::LetterGa),
            '𑆔' => Ok(Sharada::LetterGha),
            '𑆕' => Ok(Sharada::LetterNga),
            '𑆖' => Ok(Sharada::LetterCa),
            '𑆗' => Ok(Sharada::LetterCha),
            '𑆘' => Ok(Sharada::LetterJa),
            '𑆙' => Ok(Sharada::LetterJha),
            '𑆚' => Ok(Sharada::LetterNya),
            '𑆛' => Ok(Sharada::LetterTta),
            '𑆜' => Ok(Sharada::LetterTtha),
            '𑆝' => Ok(Sharada::LetterDda),
            '𑆞' => Ok(Sharada::LetterDdha),
            '𑆟' => Ok(Sharada::LetterNna),
            '𑆠' => Ok(Sharada::LetterTa),
            '𑆡' => Ok(Sharada::LetterTha),
            '𑆢' => Ok(Sharada::LetterDa),
            '𑆣' => Ok(Sharada::LetterDha),
            '𑆤' => Ok(Sharada::LetterNa),
            '𑆥' => Ok(Sharada::LetterPa),
            '𑆦' => Ok(Sharada::LetterPha),
            '𑆧' => Ok(Sharada::LetterBa),
            '𑆨' => Ok(Sharada::LetterBha),
            '𑆩' => Ok(Sharada::LetterMa),
            '𑆪' => Ok(Sharada::LetterYa),
            '𑆫' => Ok(Sharada::LetterRa),
            '𑆬' => Ok(Sharada::LetterLa),
            '𑆭' => Ok(Sharada::LetterLla),
            '𑆮' => Ok(Sharada::LetterVa),
            '𑆯' => Ok(Sharada::LetterSha),
            '𑆰' => Ok(Sharada::LetterSsa),
            '𑆱' => Ok(Sharada::LetterSa),
            '𑆲' => Ok(Sharada::LetterHa),
            '𑆳' => Ok(Sharada::VowelSignAa),
            '𑆴' => Ok(Sharada::VowelSignI),
            '𑆵' => Ok(Sharada::VowelSignIi),
            '𑆶' => Ok(Sharada::VowelSignU),
            '𑆷' => Ok(Sharada::VowelSignUu),
            '𑆸' => Ok(Sharada::VowelSignVocalicR),
            '𑆹' => Ok(Sharada::VowelSignVocalicRr),
            '𑆺' => Ok(Sharada::VowelSignVocalicL),
            '𑆻' => Ok(Sharada::VowelSignVocalicLl),
            '𑆼' => Ok(Sharada::VowelSignE),
            '𑆽' => Ok(Sharada::VowelSignAi),
            '𑆾' => Ok(Sharada::VowelSignO),
            '𑆿' => Ok(Sharada::VowelSignAu),
            '𑇀' => Ok(Sharada::SignVirama),
            '𑇁' => Ok(Sharada::SignAvagraha),
            '𑇂' => Ok(Sharada::SignJihvamuliya),
            '𑇃' => Ok(Sharada::SignUpadhmaniya),
            '𑇄' => Ok(Sharada::Om),
            '𑇅' => Ok(Sharada::Danda),
            '𑇆' => Ok(Sharada::DoubleDanda),
            '𑇇' => Ok(Sharada::AbbreviationSign),
            '𑇈' => Ok(Sharada::Separator),
            '𑇉' => Ok(Sharada::SandhiMark),
            '𑇊' => Ok(Sharada::SignNukta),
            '𑇋' => Ok(Sharada::VowelModifierMark),
            '𑇌' => Ok(Sharada::ExtraShortVowelMark),
            '𑇍' => Ok(Sharada::SutraMark),
            '𑇐' => Ok(Sharada::DigitZero),
            '𑇑' => Ok(Sharada::DigitOne),
            '𑇒' => Ok(Sharada::DigitTwo),
            '𑇓' => Ok(Sharada::DigitThree),
            '𑇔' => Ok(Sharada::DigitFour),
            '𑇕' => Ok(Sharada::DigitFive),
            '𑇖' => Ok(Sharada::DigitSix),
            '𑇗' => Ok(Sharada::DigitSeven),
            '𑇘' => Ok(Sharada::DigitEight),
            '𑇙' => Ok(Sharada::DigitNine),
            '𑇚' => Ok(Sharada::Ekam),
            '𑇛' => Ok(Sharada::SignSiddham),
            '𑇜' => Ok(Sharada::Headstroke),
            '𑇝' => Ok(Sharada::ContinuationSign),
            '𑇞' => Ok(Sharada::SectionMarkDash1),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Sharada {
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

impl std::convert::TryFrom<u32> for Sharada {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Sharada {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Sharada {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Sharada::SignCandrabindu
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Sharada{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
