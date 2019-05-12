
/// An enum to represent all characters in the Newa block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Newa {
    /// \u{11400}: '𑐀'
    LetterA,
    /// \u{11401}: '𑐁'
    LetterAa,
    /// \u{11402}: '𑐂'
    LetterI,
    /// \u{11403}: '𑐃'
    LetterIi,
    /// \u{11404}: '𑐄'
    LetterU,
    /// \u{11405}: '𑐅'
    LetterUu,
    /// \u{11406}: '𑐆'
    LetterVocalicR,
    /// \u{11407}: '𑐇'
    LetterVocalicRr,
    /// \u{11408}: '𑐈'
    LetterVocalicL,
    /// \u{11409}: '𑐉'
    LetterVocalicLl,
    /// \u{1140a}: '𑐊'
    LetterE,
    /// \u{1140b}: '𑐋'
    LetterAi,
    /// \u{1140c}: '𑐌'
    LetterO,
    /// \u{1140d}: '𑐍'
    LetterAu,
    /// \u{1140e}: '𑐎'
    LetterKa,
    /// \u{1140f}: '𑐏'
    LetterKha,
    /// \u{11410}: '𑐐'
    LetterGa,
    /// \u{11411}: '𑐑'
    LetterGha,
    /// \u{11412}: '𑐒'
    LetterNga,
    /// \u{11413}: '𑐓'
    LetterNgha,
    /// \u{11414}: '𑐔'
    LetterCa,
    /// \u{11415}: '𑐕'
    LetterCha,
    /// \u{11416}: '𑐖'
    LetterJa,
    /// \u{11417}: '𑐗'
    LetterJha,
    /// \u{11418}: '𑐘'
    LetterNya,
    /// \u{11419}: '𑐙'
    LetterNyha,
    /// \u{1141a}: '𑐚'
    LetterTta,
    /// \u{1141b}: '𑐛'
    LetterTtha,
    /// \u{1141c}: '𑐜'
    LetterDda,
    /// \u{1141d}: '𑐝'
    LetterDdha,
    /// \u{1141e}: '𑐞'
    LetterNna,
    /// \u{1141f}: '𑐟'
    LetterTa,
    /// \u{11420}: '𑐠'
    LetterTha,
    /// \u{11421}: '𑐡'
    LetterDa,
    /// \u{11422}: '𑐢'
    LetterDha,
    /// \u{11423}: '𑐣'
    LetterNa,
    /// \u{11424}: '𑐤'
    LetterNha,
    /// \u{11425}: '𑐥'
    LetterPa,
    /// \u{11426}: '𑐦'
    LetterPha,
    /// \u{11427}: '𑐧'
    LetterBa,
    /// \u{11428}: '𑐨'
    LetterBha,
    /// \u{11429}: '𑐩'
    LetterMa,
    /// \u{1142a}: '𑐪'
    LetterMha,
    /// \u{1142b}: '𑐫'
    LetterYa,
    /// \u{1142c}: '𑐬'
    LetterRa,
    /// \u{1142d}: '𑐭'
    LetterRha,
    /// \u{1142e}: '𑐮'
    LetterLa,
    /// \u{1142f}: '𑐯'
    LetterLha,
    /// \u{11430}: '𑐰'
    LetterWa,
    /// \u{11431}: '𑐱'
    LetterSha,
    /// \u{11432}: '𑐲'
    LetterSsa,
    /// \u{11433}: '𑐳'
    LetterSa,
    /// \u{11434}: '𑐴'
    LetterHa,
    /// \u{11435}: '𑐵'
    VowelSignAa,
    /// \u{11436}: '𑐶'
    VowelSignI,
    /// \u{11437}: '𑐷'
    VowelSignIi,
    /// \u{11438}: '𑐸'
    VowelSignU,
    /// \u{11439}: '𑐹'
    VowelSignUu,
    /// \u{1143a}: '𑐺'
    VowelSignVocalicR,
    /// \u{1143b}: '𑐻'
    VowelSignVocalicRr,
    /// \u{1143c}: '𑐼'
    VowelSignVocalicL,
    /// \u{1143d}: '𑐽'
    VowelSignVocalicLl,
    /// \u{1143e}: '𑐾'
    VowelSignE,
    /// \u{1143f}: '𑐿'
    VowelSignAi,
    /// \u{11440}: '𑑀'
    VowelSignO,
    /// \u{11441}: '𑑁'
    VowelSignAu,
    /// \u{11442}: '𑑂'
    SignVirama,
    /// \u{11443}: '𑑃'
    SignCandrabindu,
    /// \u{11444}: '𑑄'
    SignAnusvara,
    /// \u{11445}: '𑑅'
    SignVisarga,
    /// \u{11446}: '𑑆'
    SignNukta,
    /// \u{11447}: '𑑇'
    SignAvagraha,
    /// \u{11448}: '𑑈'
    SignFinalAnusvara,
    /// \u{11449}: '𑑉'
    Om,
    /// \u{1144a}: '𑑊'
    Siddhi,
    /// \u{1144b}: '𑑋'
    Danda,
    /// \u{1144c}: '𑑌'
    DoubleDanda,
    /// \u{1144d}: '𑑍'
    Comma,
    /// \u{1144e}: '𑑎'
    GapFiller,
    /// \u{1144f}: '𑑏'
    AbbreviationSign,
    /// \u{11450}: '𑑐'
    DigitZero,
    /// \u{11451}: '𑑑'
    DigitOne,
    /// \u{11452}: '𑑒'
    DigitTwo,
    /// \u{11453}: '𑑓'
    DigitThree,
    /// \u{11454}: '𑑔'
    DigitFour,
    /// \u{11455}: '𑑕'
    DigitFive,
    /// \u{11456}: '𑑖'
    DigitSix,
    /// \u{11457}: '𑑗'
    DigitSeven,
    /// \u{11458}: '𑑘'
    DigitEight,
    /// \u{11459}: '𑑙'
    DigitNine,
    /// \u{1145b}: '𑑛'
    PlaceholderMark,
    /// \u{1145d}: '𑑝'
    InsertionSign,
    /// \u{1145e}: '𑑞'
    SandhiMark,
    /// \u{1145f}: '𑑟'
    LetterVedicAnusvara,
}

impl Into<char> for Newa {
    fn into(self) -> char {
        match self {
            Newa::LetterA => '𑐀',
            Newa::LetterAa => '𑐁',
            Newa::LetterI => '𑐂',
            Newa::LetterIi => '𑐃',
            Newa::LetterU => '𑐄',
            Newa::LetterUu => '𑐅',
            Newa::LetterVocalicR => '𑐆',
            Newa::LetterVocalicRr => '𑐇',
            Newa::LetterVocalicL => '𑐈',
            Newa::LetterVocalicLl => '𑐉',
            Newa::LetterE => '𑐊',
            Newa::LetterAi => '𑐋',
            Newa::LetterO => '𑐌',
            Newa::LetterAu => '𑐍',
            Newa::LetterKa => '𑐎',
            Newa::LetterKha => '𑐏',
            Newa::LetterGa => '𑐐',
            Newa::LetterGha => '𑐑',
            Newa::LetterNga => '𑐒',
            Newa::LetterNgha => '𑐓',
            Newa::LetterCa => '𑐔',
            Newa::LetterCha => '𑐕',
            Newa::LetterJa => '𑐖',
            Newa::LetterJha => '𑐗',
            Newa::LetterNya => '𑐘',
            Newa::LetterNyha => '𑐙',
            Newa::LetterTta => '𑐚',
            Newa::LetterTtha => '𑐛',
            Newa::LetterDda => '𑐜',
            Newa::LetterDdha => '𑐝',
            Newa::LetterNna => '𑐞',
            Newa::LetterTa => '𑐟',
            Newa::LetterTha => '𑐠',
            Newa::LetterDa => '𑐡',
            Newa::LetterDha => '𑐢',
            Newa::LetterNa => '𑐣',
            Newa::LetterNha => '𑐤',
            Newa::LetterPa => '𑐥',
            Newa::LetterPha => '𑐦',
            Newa::LetterBa => '𑐧',
            Newa::LetterBha => '𑐨',
            Newa::LetterMa => '𑐩',
            Newa::LetterMha => '𑐪',
            Newa::LetterYa => '𑐫',
            Newa::LetterRa => '𑐬',
            Newa::LetterRha => '𑐭',
            Newa::LetterLa => '𑐮',
            Newa::LetterLha => '𑐯',
            Newa::LetterWa => '𑐰',
            Newa::LetterSha => '𑐱',
            Newa::LetterSsa => '𑐲',
            Newa::LetterSa => '𑐳',
            Newa::LetterHa => '𑐴',
            Newa::VowelSignAa => '𑐵',
            Newa::VowelSignI => '𑐶',
            Newa::VowelSignIi => '𑐷',
            Newa::VowelSignU => '𑐸',
            Newa::VowelSignUu => '𑐹',
            Newa::VowelSignVocalicR => '𑐺',
            Newa::VowelSignVocalicRr => '𑐻',
            Newa::VowelSignVocalicL => '𑐼',
            Newa::VowelSignVocalicLl => '𑐽',
            Newa::VowelSignE => '𑐾',
            Newa::VowelSignAi => '𑐿',
            Newa::VowelSignO => '𑑀',
            Newa::VowelSignAu => '𑑁',
            Newa::SignVirama => '𑑂',
            Newa::SignCandrabindu => '𑑃',
            Newa::SignAnusvara => '𑑄',
            Newa::SignVisarga => '𑑅',
            Newa::SignNukta => '𑑆',
            Newa::SignAvagraha => '𑑇',
            Newa::SignFinalAnusvara => '𑑈',
            Newa::Om => '𑑉',
            Newa::Siddhi => '𑑊',
            Newa::Danda => '𑑋',
            Newa::DoubleDanda => '𑑌',
            Newa::Comma => '𑑍',
            Newa::GapFiller => '𑑎',
            Newa::AbbreviationSign => '𑑏',
            Newa::DigitZero => '𑑐',
            Newa::DigitOne => '𑑑',
            Newa::DigitTwo => '𑑒',
            Newa::DigitThree => '𑑓',
            Newa::DigitFour => '𑑔',
            Newa::DigitFive => '𑑕',
            Newa::DigitSix => '𑑖',
            Newa::DigitSeven => '𑑗',
            Newa::DigitEight => '𑑘',
            Newa::DigitNine => '𑑙',
            Newa::PlaceholderMark => '𑑛',
            Newa::InsertionSign => '𑑝',
            Newa::SandhiMark => '𑑞',
            Newa::LetterVedicAnusvara => '𑑟',
        }
    }
}

impl std::convert::TryFrom<char> for Newa {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𑐀' => Ok(Newa::LetterA),
            '𑐁' => Ok(Newa::LetterAa),
            '𑐂' => Ok(Newa::LetterI),
            '𑐃' => Ok(Newa::LetterIi),
            '𑐄' => Ok(Newa::LetterU),
            '𑐅' => Ok(Newa::LetterUu),
            '𑐆' => Ok(Newa::LetterVocalicR),
            '𑐇' => Ok(Newa::LetterVocalicRr),
            '𑐈' => Ok(Newa::LetterVocalicL),
            '𑐉' => Ok(Newa::LetterVocalicLl),
            '𑐊' => Ok(Newa::LetterE),
            '𑐋' => Ok(Newa::LetterAi),
            '𑐌' => Ok(Newa::LetterO),
            '𑐍' => Ok(Newa::LetterAu),
            '𑐎' => Ok(Newa::LetterKa),
            '𑐏' => Ok(Newa::LetterKha),
            '𑐐' => Ok(Newa::LetterGa),
            '𑐑' => Ok(Newa::LetterGha),
            '𑐒' => Ok(Newa::LetterNga),
            '𑐓' => Ok(Newa::LetterNgha),
            '𑐔' => Ok(Newa::LetterCa),
            '𑐕' => Ok(Newa::LetterCha),
            '𑐖' => Ok(Newa::LetterJa),
            '𑐗' => Ok(Newa::LetterJha),
            '𑐘' => Ok(Newa::LetterNya),
            '𑐙' => Ok(Newa::LetterNyha),
            '𑐚' => Ok(Newa::LetterTta),
            '𑐛' => Ok(Newa::LetterTtha),
            '𑐜' => Ok(Newa::LetterDda),
            '𑐝' => Ok(Newa::LetterDdha),
            '𑐞' => Ok(Newa::LetterNna),
            '𑐟' => Ok(Newa::LetterTa),
            '𑐠' => Ok(Newa::LetterTha),
            '𑐡' => Ok(Newa::LetterDa),
            '𑐢' => Ok(Newa::LetterDha),
            '𑐣' => Ok(Newa::LetterNa),
            '𑐤' => Ok(Newa::LetterNha),
            '𑐥' => Ok(Newa::LetterPa),
            '𑐦' => Ok(Newa::LetterPha),
            '𑐧' => Ok(Newa::LetterBa),
            '𑐨' => Ok(Newa::LetterBha),
            '𑐩' => Ok(Newa::LetterMa),
            '𑐪' => Ok(Newa::LetterMha),
            '𑐫' => Ok(Newa::LetterYa),
            '𑐬' => Ok(Newa::LetterRa),
            '𑐭' => Ok(Newa::LetterRha),
            '𑐮' => Ok(Newa::LetterLa),
            '𑐯' => Ok(Newa::LetterLha),
            '𑐰' => Ok(Newa::LetterWa),
            '𑐱' => Ok(Newa::LetterSha),
            '𑐲' => Ok(Newa::LetterSsa),
            '𑐳' => Ok(Newa::LetterSa),
            '𑐴' => Ok(Newa::LetterHa),
            '𑐵' => Ok(Newa::VowelSignAa),
            '𑐶' => Ok(Newa::VowelSignI),
            '𑐷' => Ok(Newa::VowelSignIi),
            '𑐸' => Ok(Newa::VowelSignU),
            '𑐹' => Ok(Newa::VowelSignUu),
            '𑐺' => Ok(Newa::VowelSignVocalicR),
            '𑐻' => Ok(Newa::VowelSignVocalicRr),
            '𑐼' => Ok(Newa::VowelSignVocalicL),
            '𑐽' => Ok(Newa::VowelSignVocalicLl),
            '𑐾' => Ok(Newa::VowelSignE),
            '𑐿' => Ok(Newa::VowelSignAi),
            '𑑀' => Ok(Newa::VowelSignO),
            '𑑁' => Ok(Newa::VowelSignAu),
            '𑑂' => Ok(Newa::SignVirama),
            '𑑃' => Ok(Newa::SignCandrabindu),
            '𑑄' => Ok(Newa::SignAnusvara),
            '𑑅' => Ok(Newa::SignVisarga),
            '𑑆' => Ok(Newa::SignNukta),
            '𑑇' => Ok(Newa::SignAvagraha),
            '𑑈' => Ok(Newa::SignFinalAnusvara),
            '𑑉' => Ok(Newa::Om),
            '𑑊' => Ok(Newa::Siddhi),
            '𑑋' => Ok(Newa::Danda),
            '𑑌' => Ok(Newa::DoubleDanda),
            '𑑍' => Ok(Newa::Comma),
            '𑑎' => Ok(Newa::GapFiller),
            '𑑏' => Ok(Newa::AbbreviationSign),
            '𑑐' => Ok(Newa::DigitZero),
            '𑑑' => Ok(Newa::DigitOne),
            '𑑒' => Ok(Newa::DigitTwo),
            '𑑓' => Ok(Newa::DigitThree),
            '𑑔' => Ok(Newa::DigitFour),
            '𑑕' => Ok(Newa::DigitFive),
            '𑑖' => Ok(Newa::DigitSix),
            '𑑗' => Ok(Newa::DigitSeven),
            '𑑘' => Ok(Newa::DigitEight),
            '𑑙' => Ok(Newa::DigitNine),
            '𑑛' => Ok(Newa::PlaceholderMark),
            '𑑝' => Ok(Newa::InsertionSign),
            '𑑞' => Ok(Newa::SandhiMark),
            '𑑟' => Ok(Newa::LetterVedicAnusvara),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Newa {
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

impl std::convert::TryFrom<u32> for Newa {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Newa {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Newa {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Newa::LetterA
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Newa{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
