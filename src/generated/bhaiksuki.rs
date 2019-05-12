
/// An enum to represent all characters in the Bhaiksuki block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Bhaiksuki {
    /// \u{11c00}: '𑰀'
    LetterA,
    /// \u{11c01}: '𑰁'
    LetterAa,
    /// \u{11c02}: '𑰂'
    LetterI,
    /// \u{11c03}: '𑰃'
    LetterIi,
    /// \u{11c04}: '𑰄'
    LetterU,
    /// \u{11c05}: '𑰅'
    LetterUu,
    /// \u{11c06}: '𑰆'
    LetterVocalicR,
    /// \u{11c07}: '𑰇'
    LetterVocalicRr,
    /// \u{11c08}: '𑰈'
    LetterVocalicL,
    /// \u{11c0a}: '𑰊'
    LetterE,
    /// \u{11c0b}: '𑰋'
    LetterAi,
    /// \u{11c0c}: '𑰌'
    LetterO,
    /// \u{11c0d}: '𑰍'
    LetterAu,
    /// \u{11c0e}: '𑰎'
    LetterKa,
    /// \u{11c0f}: '𑰏'
    LetterKha,
    /// \u{11c10}: '𑰐'
    LetterGa,
    /// \u{11c11}: '𑰑'
    LetterGha,
    /// \u{11c12}: '𑰒'
    LetterNga,
    /// \u{11c13}: '𑰓'
    LetterCa,
    /// \u{11c14}: '𑰔'
    LetterCha,
    /// \u{11c15}: '𑰕'
    LetterJa,
    /// \u{11c16}: '𑰖'
    LetterJha,
    /// \u{11c17}: '𑰗'
    LetterNya,
    /// \u{11c18}: '𑰘'
    LetterTta,
    /// \u{11c19}: '𑰙'
    LetterTtha,
    /// \u{11c1a}: '𑰚'
    LetterDda,
    /// \u{11c1b}: '𑰛'
    LetterDdha,
    /// \u{11c1c}: '𑰜'
    LetterNna,
    /// \u{11c1d}: '𑰝'
    LetterTa,
    /// \u{11c1e}: '𑰞'
    LetterTha,
    /// \u{11c1f}: '𑰟'
    LetterDa,
    /// \u{11c20}: '𑰠'
    LetterDha,
    /// \u{11c21}: '𑰡'
    LetterNa,
    /// \u{11c22}: '𑰢'
    LetterPa,
    /// \u{11c23}: '𑰣'
    LetterPha,
    /// \u{11c24}: '𑰤'
    LetterBa,
    /// \u{11c25}: '𑰥'
    LetterBha,
    /// \u{11c26}: '𑰦'
    LetterMa,
    /// \u{11c27}: '𑰧'
    LetterYa,
    /// \u{11c28}: '𑰨'
    LetterRa,
    /// \u{11c29}: '𑰩'
    LetterLa,
    /// \u{11c2a}: '𑰪'
    LetterVa,
    /// \u{11c2b}: '𑰫'
    LetterSha,
    /// \u{11c2c}: '𑰬'
    LetterSsa,
    /// \u{11c2d}: '𑰭'
    LetterSa,
    /// \u{11c2e}: '𑰮'
    LetterHa,
    /// \u{11c2f}: '𑰯'
    VowelSignAa,
    /// \u{11c30}: '𑰰'
    VowelSignI,
    /// \u{11c31}: '𑰱'
    VowelSignIi,
    /// \u{11c32}: '𑰲'
    VowelSignU,
    /// \u{11c33}: '𑰳'
    VowelSignUu,
    /// \u{11c34}: '𑰴'
    VowelSignVocalicR,
    /// \u{11c35}: '𑰵'
    VowelSignVocalicRr,
    /// \u{11c36}: '𑰶'
    VowelSignVocalicL,
    /// \u{11c38}: '𑰸'
    VowelSignE,
    /// \u{11c39}: '𑰹'
    VowelSignAi,
    /// \u{11c3a}: '𑰺'
    VowelSignO,
    /// \u{11c3b}: '𑰻'
    VowelSignAu,
    /// \u{11c3c}: '𑰼'
    SignCandrabindu,
    /// \u{11c3d}: '𑰽'
    SignAnusvara,
    /// \u{11c3e}: '𑰾'
    SignVisarga,
    /// \u{11c3f}: '𑰿'
    SignVirama,
    /// \u{11c40}: '𑱀'
    SignAvagraha,
    /// \u{11c41}: '𑱁'
    Danda,
    /// \u{11c42}: '𑱂'
    DoubleDanda,
    /// \u{11c43}: '𑱃'
    WordSeparator,
    /// \u{11c44}: '𑱄'
    GapFillerDash1,
    /// \u{11c45}: '𑱅'
    GapFillerDash2,
    /// \u{11c50}: '𑱐'
    DigitZero,
    /// \u{11c51}: '𑱑'
    DigitOne,
    /// \u{11c52}: '𑱒'
    DigitTwo,
    /// \u{11c53}: '𑱓'
    DigitThree,
    /// \u{11c54}: '𑱔'
    DigitFour,
    /// \u{11c55}: '𑱕'
    DigitFive,
    /// \u{11c56}: '𑱖'
    DigitSix,
    /// \u{11c57}: '𑱗'
    DigitSeven,
    /// \u{11c58}: '𑱘'
    DigitEight,
    /// \u{11c59}: '𑱙'
    DigitNine,
    /// \u{11c5a}: '𑱚'
    NumberOne,
    /// \u{11c5b}: '𑱛'
    NumberTwo,
    /// \u{11c5c}: '𑱜'
    NumberThree,
    /// \u{11c5d}: '𑱝'
    NumberFour,
    /// \u{11c5e}: '𑱞'
    NumberFive,
    /// \u{11c5f}: '𑱟'
    NumberSix,
    /// \u{11c60}: '𑱠'
    NumberSeven,
    /// \u{11c61}: '𑱡'
    NumberEight,
    /// \u{11c62}: '𑱢'
    NumberNine,
    /// \u{11c63}: '𑱣'
    NumberTen,
    /// \u{11c64}: '𑱤'
    NumberTwenty,
    /// \u{11c65}: '𑱥'
    NumberThirty,
    /// \u{11c66}: '𑱦'
    NumberForty,
    /// \u{11c67}: '𑱧'
    NumberFifty,
    /// \u{11c68}: '𑱨'
    NumberSixty,
    /// \u{11c69}: '𑱩'
    NumberSeventy,
    /// \u{11c6a}: '𑱪'
    NumberEighty,
    /// \u{11c6b}: '𑱫'
    NumberNinety,
    /// \u{11c6c}: '𑱬'
    HundredsUnitMark,
}

impl Into<char> for Bhaiksuki {
    fn into(self) -> char {
        match self {
            Bhaiksuki::LetterA => '𑰀',
            Bhaiksuki::LetterAa => '𑰁',
            Bhaiksuki::LetterI => '𑰂',
            Bhaiksuki::LetterIi => '𑰃',
            Bhaiksuki::LetterU => '𑰄',
            Bhaiksuki::LetterUu => '𑰅',
            Bhaiksuki::LetterVocalicR => '𑰆',
            Bhaiksuki::LetterVocalicRr => '𑰇',
            Bhaiksuki::LetterVocalicL => '𑰈',
            Bhaiksuki::LetterE => '𑰊',
            Bhaiksuki::LetterAi => '𑰋',
            Bhaiksuki::LetterO => '𑰌',
            Bhaiksuki::LetterAu => '𑰍',
            Bhaiksuki::LetterKa => '𑰎',
            Bhaiksuki::LetterKha => '𑰏',
            Bhaiksuki::LetterGa => '𑰐',
            Bhaiksuki::LetterGha => '𑰑',
            Bhaiksuki::LetterNga => '𑰒',
            Bhaiksuki::LetterCa => '𑰓',
            Bhaiksuki::LetterCha => '𑰔',
            Bhaiksuki::LetterJa => '𑰕',
            Bhaiksuki::LetterJha => '𑰖',
            Bhaiksuki::LetterNya => '𑰗',
            Bhaiksuki::LetterTta => '𑰘',
            Bhaiksuki::LetterTtha => '𑰙',
            Bhaiksuki::LetterDda => '𑰚',
            Bhaiksuki::LetterDdha => '𑰛',
            Bhaiksuki::LetterNna => '𑰜',
            Bhaiksuki::LetterTa => '𑰝',
            Bhaiksuki::LetterTha => '𑰞',
            Bhaiksuki::LetterDa => '𑰟',
            Bhaiksuki::LetterDha => '𑰠',
            Bhaiksuki::LetterNa => '𑰡',
            Bhaiksuki::LetterPa => '𑰢',
            Bhaiksuki::LetterPha => '𑰣',
            Bhaiksuki::LetterBa => '𑰤',
            Bhaiksuki::LetterBha => '𑰥',
            Bhaiksuki::LetterMa => '𑰦',
            Bhaiksuki::LetterYa => '𑰧',
            Bhaiksuki::LetterRa => '𑰨',
            Bhaiksuki::LetterLa => '𑰩',
            Bhaiksuki::LetterVa => '𑰪',
            Bhaiksuki::LetterSha => '𑰫',
            Bhaiksuki::LetterSsa => '𑰬',
            Bhaiksuki::LetterSa => '𑰭',
            Bhaiksuki::LetterHa => '𑰮',
            Bhaiksuki::VowelSignAa => '𑰯',
            Bhaiksuki::VowelSignI => '𑰰',
            Bhaiksuki::VowelSignIi => '𑰱',
            Bhaiksuki::VowelSignU => '𑰲',
            Bhaiksuki::VowelSignUu => '𑰳',
            Bhaiksuki::VowelSignVocalicR => '𑰴',
            Bhaiksuki::VowelSignVocalicRr => '𑰵',
            Bhaiksuki::VowelSignVocalicL => '𑰶',
            Bhaiksuki::VowelSignE => '𑰸',
            Bhaiksuki::VowelSignAi => '𑰹',
            Bhaiksuki::VowelSignO => '𑰺',
            Bhaiksuki::VowelSignAu => '𑰻',
            Bhaiksuki::SignCandrabindu => '𑰼',
            Bhaiksuki::SignAnusvara => '𑰽',
            Bhaiksuki::SignVisarga => '𑰾',
            Bhaiksuki::SignVirama => '𑰿',
            Bhaiksuki::SignAvagraha => '𑱀',
            Bhaiksuki::Danda => '𑱁',
            Bhaiksuki::DoubleDanda => '𑱂',
            Bhaiksuki::WordSeparator => '𑱃',
            Bhaiksuki::GapFillerDash1 => '𑱄',
            Bhaiksuki::GapFillerDash2 => '𑱅',
            Bhaiksuki::DigitZero => '𑱐',
            Bhaiksuki::DigitOne => '𑱑',
            Bhaiksuki::DigitTwo => '𑱒',
            Bhaiksuki::DigitThree => '𑱓',
            Bhaiksuki::DigitFour => '𑱔',
            Bhaiksuki::DigitFive => '𑱕',
            Bhaiksuki::DigitSix => '𑱖',
            Bhaiksuki::DigitSeven => '𑱗',
            Bhaiksuki::DigitEight => '𑱘',
            Bhaiksuki::DigitNine => '𑱙',
            Bhaiksuki::NumberOne => '𑱚',
            Bhaiksuki::NumberTwo => '𑱛',
            Bhaiksuki::NumberThree => '𑱜',
            Bhaiksuki::NumberFour => '𑱝',
            Bhaiksuki::NumberFive => '𑱞',
            Bhaiksuki::NumberSix => '𑱟',
            Bhaiksuki::NumberSeven => '𑱠',
            Bhaiksuki::NumberEight => '𑱡',
            Bhaiksuki::NumberNine => '𑱢',
            Bhaiksuki::NumberTen => '𑱣',
            Bhaiksuki::NumberTwenty => '𑱤',
            Bhaiksuki::NumberThirty => '𑱥',
            Bhaiksuki::NumberForty => '𑱦',
            Bhaiksuki::NumberFifty => '𑱧',
            Bhaiksuki::NumberSixty => '𑱨',
            Bhaiksuki::NumberSeventy => '𑱩',
            Bhaiksuki::NumberEighty => '𑱪',
            Bhaiksuki::NumberNinety => '𑱫',
            Bhaiksuki::HundredsUnitMark => '𑱬',
        }
    }
}

impl std::convert::TryFrom<char> for Bhaiksuki {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𑰀' => Ok(Bhaiksuki::LetterA),
            '𑰁' => Ok(Bhaiksuki::LetterAa),
            '𑰂' => Ok(Bhaiksuki::LetterI),
            '𑰃' => Ok(Bhaiksuki::LetterIi),
            '𑰄' => Ok(Bhaiksuki::LetterU),
            '𑰅' => Ok(Bhaiksuki::LetterUu),
            '𑰆' => Ok(Bhaiksuki::LetterVocalicR),
            '𑰇' => Ok(Bhaiksuki::LetterVocalicRr),
            '𑰈' => Ok(Bhaiksuki::LetterVocalicL),
            '𑰊' => Ok(Bhaiksuki::LetterE),
            '𑰋' => Ok(Bhaiksuki::LetterAi),
            '𑰌' => Ok(Bhaiksuki::LetterO),
            '𑰍' => Ok(Bhaiksuki::LetterAu),
            '𑰎' => Ok(Bhaiksuki::LetterKa),
            '𑰏' => Ok(Bhaiksuki::LetterKha),
            '𑰐' => Ok(Bhaiksuki::LetterGa),
            '𑰑' => Ok(Bhaiksuki::LetterGha),
            '𑰒' => Ok(Bhaiksuki::LetterNga),
            '𑰓' => Ok(Bhaiksuki::LetterCa),
            '𑰔' => Ok(Bhaiksuki::LetterCha),
            '𑰕' => Ok(Bhaiksuki::LetterJa),
            '𑰖' => Ok(Bhaiksuki::LetterJha),
            '𑰗' => Ok(Bhaiksuki::LetterNya),
            '𑰘' => Ok(Bhaiksuki::LetterTta),
            '𑰙' => Ok(Bhaiksuki::LetterTtha),
            '𑰚' => Ok(Bhaiksuki::LetterDda),
            '𑰛' => Ok(Bhaiksuki::LetterDdha),
            '𑰜' => Ok(Bhaiksuki::LetterNna),
            '𑰝' => Ok(Bhaiksuki::LetterTa),
            '𑰞' => Ok(Bhaiksuki::LetterTha),
            '𑰟' => Ok(Bhaiksuki::LetterDa),
            '𑰠' => Ok(Bhaiksuki::LetterDha),
            '𑰡' => Ok(Bhaiksuki::LetterNa),
            '𑰢' => Ok(Bhaiksuki::LetterPa),
            '𑰣' => Ok(Bhaiksuki::LetterPha),
            '𑰤' => Ok(Bhaiksuki::LetterBa),
            '𑰥' => Ok(Bhaiksuki::LetterBha),
            '𑰦' => Ok(Bhaiksuki::LetterMa),
            '𑰧' => Ok(Bhaiksuki::LetterYa),
            '𑰨' => Ok(Bhaiksuki::LetterRa),
            '𑰩' => Ok(Bhaiksuki::LetterLa),
            '𑰪' => Ok(Bhaiksuki::LetterVa),
            '𑰫' => Ok(Bhaiksuki::LetterSha),
            '𑰬' => Ok(Bhaiksuki::LetterSsa),
            '𑰭' => Ok(Bhaiksuki::LetterSa),
            '𑰮' => Ok(Bhaiksuki::LetterHa),
            '𑰯' => Ok(Bhaiksuki::VowelSignAa),
            '𑰰' => Ok(Bhaiksuki::VowelSignI),
            '𑰱' => Ok(Bhaiksuki::VowelSignIi),
            '𑰲' => Ok(Bhaiksuki::VowelSignU),
            '𑰳' => Ok(Bhaiksuki::VowelSignUu),
            '𑰴' => Ok(Bhaiksuki::VowelSignVocalicR),
            '𑰵' => Ok(Bhaiksuki::VowelSignVocalicRr),
            '𑰶' => Ok(Bhaiksuki::VowelSignVocalicL),
            '𑰸' => Ok(Bhaiksuki::VowelSignE),
            '𑰹' => Ok(Bhaiksuki::VowelSignAi),
            '𑰺' => Ok(Bhaiksuki::VowelSignO),
            '𑰻' => Ok(Bhaiksuki::VowelSignAu),
            '𑰼' => Ok(Bhaiksuki::SignCandrabindu),
            '𑰽' => Ok(Bhaiksuki::SignAnusvara),
            '𑰾' => Ok(Bhaiksuki::SignVisarga),
            '𑰿' => Ok(Bhaiksuki::SignVirama),
            '𑱀' => Ok(Bhaiksuki::SignAvagraha),
            '𑱁' => Ok(Bhaiksuki::Danda),
            '𑱂' => Ok(Bhaiksuki::DoubleDanda),
            '𑱃' => Ok(Bhaiksuki::WordSeparator),
            '𑱄' => Ok(Bhaiksuki::GapFillerDash1),
            '𑱅' => Ok(Bhaiksuki::GapFillerDash2),
            '𑱐' => Ok(Bhaiksuki::DigitZero),
            '𑱑' => Ok(Bhaiksuki::DigitOne),
            '𑱒' => Ok(Bhaiksuki::DigitTwo),
            '𑱓' => Ok(Bhaiksuki::DigitThree),
            '𑱔' => Ok(Bhaiksuki::DigitFour),
            '𑱕' => Ok(Bhaiksuki::DigitFive),
            '𑱖' => Ok(Bhaiksuki::DigitSix),
            '𑱗' => Ok(Bhaiksuki::DigitSeven),
            '𑱘' => Ok(Bhaiksuki::DigitEight),
            '𑱙' => Ok(Bhaiksuki::DigitNine),
            '𑱚' => Ok(Bhaiksuki::NumberOne),
            '𑱛' => Ok(Bhaiksuki::NumberTwo),
            '𑱜' => Ok(Bhaiksuki::NumberThree),
            '𑱝' => Ok(Bhaiksuki::NumberFour),
            '𑱞' => Ok(Bhaiksuki::NumberFive),
            '𑱟' => Ok(Bhaiksuki::NumberSix),
            '𑱠' => Ok(Bhaiksuki::NumberSeven),
            '𑱡' => Ok(Bhaiksuki::NumberEight),
            '𑱢' => Ok(Bhaiksuki::NumberNine),
            '𑱣' => Ok(Bhaiksuki::NumberTen),
            '𑱤' => Ok(Bhaiksuki::NumberTwenty),
            '𑱥' => Ok(Bhaiksuki::NumberThirty),
            '𑱦' => Ok(Bhaiksuki::NumberForty),
            '𑱧' => Ok(Bhaiksuki::NumberFifty),
            '𑱨' => Ok(Bhaiksuki::NumberSixty),
            '𑱩' => Ok(Bhaiksuki::NumberSeventy),
            '𑱪' => Ok(Bhaiksuki::NumberEighty),
            '𑱫' => Ok(Bhaiksuki::NumberNinety),
            '𑱬' => Ok(Bhaiksuki::HundredsUnitMark),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Bhaiksuki {
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

impl std::convert::TryFrom<u32> for Bhaiksuki {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Bhaiksuki {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Bhaiksuki {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Bhaiksuki::LetterA
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Bhaiksuki{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
