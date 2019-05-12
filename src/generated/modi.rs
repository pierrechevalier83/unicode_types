
/// An enum to represent all characters in the Modi block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Modi {
    /// \u{11600}: '𑘀'
    LetterA,
    /// \u{11601}: '𑘁'
    LetterAa,
    /// \u{11602}: '𑘂'
    LetterI,
    /// \u{11603}: '𑘃'
    LetterIi,
    /// \u{11604}: '𑘄'
    LetterU,
    /// \u{11605}: '𑘅'
    LetterUu,
    /// \u{11606}: '𑘆'
    LetterVocalicR,
    /// \u{11607}: '𑘇'
    LetterVocalicRr,
    /// \u{11608}: '𑘈'
    LetterVocalicL,
    /// \u{11609}: '𑘉'
    LetterVocalicLl,
    /// \u{1160a}: '𑘊'
    LetterE,
    /// \u{1160b}: '𑘋'
    LetterAi,
    /// \u{1160c}: '𑘌'
    LetterO,
    /// \u{1160d}: '𑘍'
    LetterAu,
    /// \u{1160e}: '𑘎'
    LetterKa,
    /// \u{1160f}: '𑘏'
    LetterKha,
    /// \u{11610}: '𑘐'
    LetterGa,
    /// \u{11611}: '𑘑'
    LetterGha,
    /// \u{11612}: '𑘒'
    LetterNga,
    /// \u{11613}: '𑘓'
    LetterCa,
    /// \u{11614}: '𑘔'
    LetterCha,
    /// \u{11615}: '𑘕'
    LetterJa,
    /// \u{11616}: '𑘖'
    LetterJha,
    /// \u{11617}: '𑘗'
    LetterNya,
    /// \u{11618}: '𑘘'
    LetterTta,
    /// \u{11619}: '𑘙'
    LetterTtha,
    /// \u{1161a}: '𑘚'
    LetterDda,
    /// \u{1161b}: '𑘛'
    LetterDdha,
    /// \u{1161c}: '𑘜'
    LetterNna,
    /// \u{1161d}: '𑘝'
    LetterTa,
    /// \u{1161e}: '𑘞'
    LetterTha,
    /// \u{1161f}: '𑘟'
    LetterDa,
    /// \u{11620}: '𑘠'
    LetterDha,
    /// \u{11621}: '𑘡'
    LetterNa,
    /// \u{11622}: '𑘢'
    LetterPa,
    /// \u{11623}: '𑘣'
    LetterPha,
    /// \u{11624}: '𑘤'
    LetterBa,
    /// \u{11625}: '𑘥'
    LetterBha,
    /// \u{11626}: '𑘦'
    LetterMa,
    /// \u{11627}: '𑘧'
    LetterYa,
    /// \u{11628}: '𑘨'
    LetterRa,
    /// \u{11629}: '𑘩'
    LetterLa,
    /// \u{1162a}: '𑘪'
    LetterVa,
    /// \u{1162b}: '𑘫'
    LetterSha,
    /// \u{1162c}: '𑘬'
    LetterSsa,
    /// \u{1162d}: '𑘭'
    LetterSa,
    /// \u{1162e}: '𑘮'
    LetterHa,
    /// \u{1162f}: '𑘯'
    LetterLla,
    /// \u{11630}: '𑘰'
    VowelSignAa,
    /// \u{11631}: '𑘱'
    VowelSignI,
    /// \u{11632}: '𑘲'
    VowelSignIi,
    /// \u{11633}: '𑘳'
    VowelSignU,
    /// \u{11634}: '𑘴'
    VowelSignUu,
    /// \u{11635}: '𑘵'
    VowelSignVocalicR,
    /// \u{11636}: '𑘶'
    VowelSignVocalicRr,
    /// \u{11637}: '𑘷'
    VowelSignVocalicL,
    /// \u{11638}: '𑘸'
    VowelSignVocalicLl,
    /// \u{11639}: '𑘹'
    VowelSignE,
    /// \u{1163a}: '𑘺'
    VowelSignAi,
    /// \u{1163b}: '𑘻'
    VowelSignO,
    /// \u{1163c}: '𑘼'
    VowelSignAu,
    /// \u{1163d}: '𑘽'
    SignAnusvara,
    /// \u{1163e}: '𑘾'
    SignVisarga,
    /// \u{1163f}: '𑘿'
    SignVirama,
    /// \u{11640}: '𑙀'
    SignArdhacandra,
    /// \u{11641}: '𑙁'
    Danda,
    /// \u{11642}: '𑙂'
    DoubleDanda,
    /// \u{11643}: '𑙃'
    AbbreviationSign,
    /// \u{11644}: '𑙄'
    SignHuva,
    /// \u{11650}: '𑙐'
    DigitZero,
    /// \u{11651}: '𑙑'
    DigitOne,
    /// \u{11652}: '𑙒'
    DigitTwo,
    /// \u{11653}: '𑙓'
    DigitThree,
    /// \u{11654}: '𑙔'
    DigitFour,
    /// \u{11655}: '𑙕'
    DigitFive,
    /// \u{11656}: '𑙖'
    DigitSix,
    /// \u{11657}: '𑙗'
    DigitSeven,
    /// \u{11658}: '𑙘'
    DigitEight,
    /// \u{11659}: '𑙙'
    DigitNine,
}

impl Into<char> for Modi {
    fn into(self) -> char {
        match self {
            Modi::LetterA => '𑘀',
            Modi::LetterAa => '𑘁',
            Modi::LetterI => '𑘂',
            Modi::LetterIi => '𑘃',
            Modi::LetterU => '𑘄',
            Modi::LetterUu => '𑘅',
            Modi::LetterVocalicR => '𑘆',
            Modi::LetterVocalicRr => '𑘇',
            Modi::LetterVocalicL => '𑘈',
            Modi::LetterVocalicLl => '𑘉',
            Modi::LetterE => '𑘊',
            Modi::LetterAi => '𑘋',
            Modi::LetterO => '𑘌',
            Modi::LetterAu => '𑘍',
            Modi::LetterKa => '𑘎',
            Modi::LetterKha => '𑘏',
            Modi::LetterGa => '𑘐',
            Modi::LetterGha => '𑘑',
            Modi::LetterNga => '𑘒',
            Modi::LetterCa => '𑘓',
            Modi::LetterCha => '𑘔',
            Modi::LetterJa => '𑘕',
            Modi::LetterJha => '𑘖',
            Modi::LetterNya => '𑘗',
            Modi::LetterTta => '𑘘',
            Modi::LetterTtha => '𑘙',
            Modi::LetterDda => '𑘚',
            Modi::LetterDdha => '𑘛',
            Modi::LetterNna => '𑘜',
            Modi::LetterTa => '𑘝',
            Modi::LetterTha => '𑘞',
            Modi::LetterDa => '𑘟',
            Modi::LetterDha => '𑘠',
            Modi::LetterNa => '𑘡',
            Modi::LetterPa => '𑘢',
            Modi::LetterPha => '𑘣',
            Modi::LetterBa => '𑘤',
            Modi::LetterBha => '𑘥',
            Modi::LetterMa => '𑘦',
            Modi::LetterYa => '𑘧',
            Modi::LetterRa => '𑘨',
            Modi::LetterLa => '𑘩',
            Modi::LetterVa => '𑘪',
            Modi::LetterSha => '𑘫',
            Modi::LetterSsa => '𑘬',
            Modi::LetterSa => '𑘭',
            Modi::LetterHa => '𑘮',
            Modi::LetterLla => '𑘯',
            Modi::VowelSignAa => '𑘰',
            Modi::VowelSignI => '𑘱',
            Modi::VowelSignIi => '𑘲',
            Modi::VowelSignU => '𑘳',
            Modi::VowelSignUu => '𑘴',
            Modi::VowelSignVocalicR => '𑘵',
            Modi::VowelSignVocalicRr => '𑘶',
            Modi::VowelSignVocalicL => '𑘷',
            Modi::VowelSignVocalicLl => '𑘸',
            Modi::VowelSignE => '𑘹',
            Modi::VowelSignAi => '𑘺',
            Modi::VowelSignO => '𑘻',
            Modi::VowelSignAu => '𑘼',
            Modi::SignAnusvara => '𑘽',
            Modi::SignVisarga => '𑘾',
            Modi::SignVirama => '𑘿',
            Modi::SignArdhacandra => '𑙀',
            Modi::Danda => '𑙁',
            Modi::DoubleDanda => '𑙂',
            Modi::AbbreviationSign => '𑙃',
            Modi::SignHuva => '𑙄',
            Modi::DigitZero => '𑙐',
            Modi::DigitOne => '𑙑',
            Modi::DigitTwo => '𑙒',
            Modi::DigitThree => '𑙓',
            Modi::DigitFour => '𑙔',
            Modi::DigitFive => '𑙕',
            Modi::DigitSix => '𑙖',
            Modi::DigitSeven => '𑙗',
            Modi::DigitEight => '𑙘',
            Modi::DigitNine => '𑙙',
        }
    }
}

impl std::convert::TryFrom<char> for Modi {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𑘀' => Ok(Modi::LetterA),
            '𑘁' => Ok(Modi::LetterAa),
            '𑘂' => Ok(Modi::LetterI),
            '𑘃' => Ok(Modi::LetterIi),
            '𑘄' => Ok(Modi::LetterU),
            '𑘅' => Ok(Modi::LetterUu),
            '𑘆' => Ok(Modi::LetterVocalicR),
            '𑘇' => Ok(Modi::LetterVocalicRr),
            '𑘈' => Ok(Modi::LetterVocalicL),
            '𑘉' => Ok(Modi::LetterVocalicLl),
            '𑘊' => Ok(Modi::LetterE),
            '𑘋' => Ok(Modi::LetterAi),
            '𑘌' => Ok(Modi::LetterO),
            '𑘍' => Ok(Modi::LetterAu),
            '𑘎' => Ok(Modi::LetterKa),
            '𑘏' => Ok(Modi::LetterKha),
            '𑘐' => Ok(Modi::LetterGa),
            '𑘑' => Ok(Modi::LetterGha),
            '𑘒' => Ok(Modi::LetterNga),
            '𑘓' => Ok(Modi::LetterCa),
            '𑘔' => Ok(Modi::LetterCha),
            '𑘕' => Ok(Modi::LetterJa),
            '𑘖' => Ok(Modi::LetterJha),
            '𑘗' => Ok(Modi::LetterNya),
            '𑘘' => Ok(Modi::LetterTta),
            '𑘙' => Ok(Modi::LetterTtha),
            '𑘚' => Ok(Modi::LetterDda),
            '𑘛' => Ok(Modi::LetterDdha),
            '𑘜' => Ok(Modi::LetterNna),
            '𑘝' => Ok(Modi::LetterTa),
            '𑘞' => Ok(Modi::LetterTha),
            '𑘟' => Ok(Modi::LetterDa),
            '𑘠' => Ok(Modi::LetterDha),
            '𑘡' => Ok(Modi::LetterNa),
            '𑘢' => Ok(Modi::LetterPa),
            '𑘣' => Ok(Modi::LetterPha),
            '𑘤' => Ok(Modi::LetterBa),
            '𑘥' => Ok(Modi::LetterBha),
            '𑘦' => Ok(Modi::LetterMa),
            '𑘧' => Ok(Modi::LetterYa),
            '𑘨' => Ok(Modi::LetterRa),
            '𑘩' => Ok(Modi::LetterLa),
            '𑘪' => Ok(Modi::LetterVa),
            '𑘫' => Ok(Modi::LetterSha),
            '𑘬' => Ok(Modi::LetterSsa),
            '𑘭' => Ok(Modi::LetterSa),
            '𑘮' => Ok(Modi::LetterHa),
            '𑘯' => Ok(Modi::LetterLla),
            '𑘰' => Ok(Modi::VowelSignAa),
            '𑘱' => Ok(Modi::VowelSignI),
            '𑘲' => Ok(Modi::VowelSignIi),
            '𑘳' => Ok(Modi::VowelSignU),
            '𑘴' => Ok(Modi::VowelSignUu),
            '𑘵' => Ok(Modi::VowelSignVocalicR),
            '𑘶' => Ok(Modi::VowelSignVocalicRr),
            '𑘷' => Ok(Modi::VowelSignVocalicL),
            '𑘸' => Ok(Modi::VowelSignVocalicLl),
            '𑘹' => Ok(Modi::VowelSignE),
            '𑘺' => Ok(Modi::VowelSignAi),
            '𑘻' => Ok(Modi::VowelSignO),
            '𑘼' => Ok(Modi::VowelSignAu),
            '𑘽' => Ok(Modi::SignAnusvara),
            '𑘾' => Ok(Modi::SignVisarga),
            '𑘿' => Ok(Modi::SignVirama),
            '𑙀' => Ok(Modi::SignArdhacandra),
            '𑙁' => Ok(Modi::Danda),
            '𑙂' => Ok(Modi::DoubleDanda),
            '𑙃' => Ok(Modi::AbbreviationSign),
            '𑙄' => Ok(Modi::SignHuva),
            '𑙐' => Ok(Modi::DigitZero),
            '𑙑' => Ok(Modi::DigitOne),
            '𑙒' => Ok(Modi::DigitTwo),
            '𑙓' => Ok(Modi::DigitThree),
            '𑙔' => Ok(Modi::DigitFour),
            '𑙕' => Ok(Modi::DigitFive),
            '𑙖' => Ok(Modi::DigitSix),
            '𑙗' => Ok(Modi::DigitSeven),
            '𑙘' => Ok(Modi::DigitEight),
            '𑙙' => Ok(Modi::DigitNine),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Modi {
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

impl std::convert::TryFrom<u32> for Modi {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Modi {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Modi {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Modi::LetterA
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Modi{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
