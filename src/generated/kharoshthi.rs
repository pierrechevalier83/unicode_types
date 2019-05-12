
/// An enum to represent all characters in the Kharoshthi block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Kharoshthi {
    /// \u{10a00}: '𐨀'
    LetterA,
    /// \u{10a01}: '𐨁'
    VowelSignI,
    /// \u{10a02}: '𐨂'
    VowelSignU,
    /// \u{10a03}: '𐨃'
    VowelSignVocalicR,
    /// \u{10a05}: '𐨅'
    VowelSignE,
    /// \u{10a06}: '𐨆'
    VowelSignO,
    /// \u{10a0c}: '𐨌'
    VowelLengthMark,
    /// \u{10a0d}: '𐨍'
    SignDoubleRingBelow,
    /// \u{10a0e}: '𐨎'
    SignAnusvara,
    /// \u{10a0f}: '𐨏'
    SignVisarga,
    /// \u{10a10}: '𐨐'
    LetterKa,
    /// \u{10a11}: '𐨑'
    LetterKha,
    /// \u{10a12}: '𐨒'
    LetterGa,
    /// \u{10a13}: '𐨓'
    LetterGha,
    /// \u{10a15}: '𐨕'
    LetterCa,
    /// \u{10a16}: '𐨖'
    LetterCha,
    /// \u{10a17}: '𐨗'
    LetterJa,
    /// \u{10a19}: '𐨙'
    LetterNya,
    /// \u{10a1a}: '𐨚'
    LetterTta,
    /// \u{10a1b}: '𐨛'
    LetterTtha,
    /// \u{10a1c}: '𐨜'
    LetterDda,
    /// \u{10a1d}: '𐨝'
    LetterDdha,
    /// \u{10a1e}: '𐨞'
    LetterNna,
    /// \u{10a1f}: '𐨟'
    LetterTa,
    /// \u{10a20}: '𐨠'
    LetterTha,
    /// \u{10a21}: '𐨡'
    LetterDa,
    /// \u{10a22}: '𐨢'
    LetterDha,
    /// \u{10a23}: '𐨣'
    LetterNa,
    /// \u{10a24}: '𐨤'
    LetterPa,
    /// \u{10a25}: '𐨥'
    LetterPha,
    /// \u{10a26}: '𐨦'
    LetterBa,
    /// \u{10a27}: '𐨧'
    LetterBha,
    /// \u{10a28}: '𐨨'
    LetterMa,
    /// \u{10a29}: '𐨩'
    LetterYa,
    /// \u{10a2a}: '𐨪'
    LetterRa,
    /// \u{10a2b}: '𐨫'
    LetterLa,
    /// \u{10a2c}: '𐨬'
    LetterVa,
    /// \u{10a2d}: '𐨭'
    LetterSha,
    /// \u{10a2e}: '𐨮'
    LetterSsa,
    /// \u{10a2f}: '𐨯'
    LetterSa,
    /// \u{10a30}: '𐨰'
    LetterZa,
    /// \u{10a31}: '𐨱'
    LetterHa,
    /// \u{10a32}: '𐨲'
    LetterKka,
    /// \u{10a33}: '𐨳'
    LetterTttha,
    /// \u{10a34}: '𐨴'
    LetterTtta,
    /// \u{10a35}: '𐨵'
    LetterVha,
    /// \u{10a38}: '𐨸'
    SignBarAbove,
    /// \u{10a39}: '𐨹'
    SignCauda,
    /// \u{10a3a}: '𐨺'
    SignDotBelow,
    /// \u{10a3f}: '𐨿'
    Virama,
    /// \u{10a40}: '𐩀'
    DigitOne,
    /// \u{10a41}: '𐩁'
    DigitTwo,
    /// \u{10a42}: '𐩂'
    DigitThree,
    /// \u{10a43}: '𐩃'
    DigitFour,
    /// \u{10a44}: '𐩄'
    NumberTen,
    /// \u{10a45}: '𐩅'
    NumberTwenty,
    /// \u{10a46}: '𐩆'
    NumberOneHundred,
    /// \u{10a47}: '𐩇'
    NumberOneThousand,
    /// \u{10a48}: '𐩈'
    FractionOneHalf,
    /// \u{10a50}: '𐩐'
    PunctuationDot,
    /// \u{10a51}: '𐩑'
    PunctuationSmallCircle,
    /// \u{10a52}: '𐩒'
    PunctuationCircle,
    /// \u{10a53}: '𐩓'
    PunctuationCrescentBar,
    /// \u{10a54}: '𐩔'
    PunctuationMangalam,
    /// \u{10a55}: '𐩕'
    PunctuationLotus,
    /// \u{10a56}: '𐩖'
    PunctuationDanda,
    /// \u{10a57}: '𐩗'
    PunctuationDoubleDanda,
    /// \u{10a58}: '𐩘'
    PunctuationLines,
}

impl Into<char> for Kharoshthi {
    fn into(self) -> char {
        match self {
            Kharoshthi::LetterA => '𐨀',
            Kharoshthi::VowelSignI => '𐨁',
            Kharoshthi::VowelSignU => '𐨂',
            Kharoshthi::VowelSignVocalicR => '𐨃',
            Kharoshthi::VowelSignE => '𐨅',
            Kharoshthi::VowelSignO => '𐨆',
            Kharoshthi::VowelLengthMark => '𐨌',
            Kharoshthi::SignDoubleRingBelow => '𐨍',
            Kharoshthi::SignAnusvara => '𐨎',
            Kharoshthi::SignVisarga => '𐨏',
            Kharoshthi::LetterKa => '𐨐',
            Kharoshthi::LetterKha => '𐨑',
            Kharoshthi::LetterGa => '𐨒',
            Kharoshthi::LetterGha => '𐨓',
            Kharoshthi::LetterCa => '𐨕',
            Kharoshthi::LetterCha => '𐨖',
            Kharoshthi::LetterJa => '𐨗',
            Kharoshthi::LetterNya => '𐨙',
            Kharoshthi::LetterTta => '𐨚',
            Kharoshthi::LetterTtha => '𐨛',
            Kharoshthi::LetterDda => '𐨜',
            Kharoshthi::LetterDdha => '𐨝',
            Kharoshthi::LetterNna => '𐨞',
            Kharoshthi::LetterTa => '𐨟',
            Kharoshthi::LetterTha => '𐨠',
            Kharoshthi::LetterDa => '𐨡',
            Kharoshthi::LetterDha => '𐨢',
            Kharoshthi::LetterNa => '𐨣',
            Kharoshthi::LetterPa => '𐨤',
            Kharoshthi::LetterPha => '𐨥',
            Kharoshthi::LetterBa => '𐨦',
            Kharoshthi::LetterBha => '𐨧',
            Kharoshthi::LetterMa => '𐨨',
            Kharoshthi::LetterYa => '𐨩',
            Kharoshthi::LetterRa => '𐨪',
            Kharoshthi::LetterLa => '𐨫',
            Kharoshthi::LetterVa => '𐨬',
            Kharoshthi::LetterSha => '𐨭',
            Kharoshthi::LetterSsa => '𐨮',
            Kharoshthi::LetterSa => '𐨯',
            Kharoshthi::LetterZa => '𐨰',
            Kharoshthi::LetterHa => '𐨱',
            Kharoshthi::LetterKka => '𐨲',
            Kharoshthi::LetterTttha => '𐨳',
            Kharoshthi::LetterTtta => '𐨴',
            Kharoshthi::LetterVha => '𐨵',
            Kharoshthi::SignBarAbove => '𐨸',
            Kharoshthi::SignCauda => '𐨹',
            Kharoshthi::SignDotBelow => '𐨺',
            Kharoshthi::Virama => '𐨿',
            Kharoshthi::DigitOne => '𐩀',
            Kharoshthi::DigitTwo => '𐩁',
            Kharoshthi::DigitThree => '𐩂',
            Kharoshthi::DigitFour => '𐩃',
            Kharoshthi::NumberTen => '𐩄',
            Kharoshthi::NumberTwenty => '𐩅',
            Kharoshthi::NumberOneHundred => '𐩆',
            Kharoshthi::NumberOneThousand => '𐩇',
            Kharoshthi::FractionOneHalf => '𐩈',
            Kharoshthi::PunctuationDot => '𐩐',
            Kharoshthi::PunctuationSmallCircle => '𐩑',
            Kharoshthi::PunctuationCircle => '𐩒',
            Kharoshthi::PunctuationCrescentBar => '𐩓',
            Kharoshthi::PunctuationMangalam => '𐩔',
            Kharoshthi::PunctuationLotus => '𐩕',
            Kharoshthi::PunctuationDanda => '𐩖',
            Kharoshthi::PunctuationDoubleDanda => '𐩗',
            Kharoshthi::PunctuationLines => '𐩘',
        }
    }
}

impl std::convert::TryFrom<char> for Kharoshthi {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𐨀' => Ok(Kharoshthi::LetterA),
            '𐨁' => Ok(Kharoshthi::VowelSignI),
            '𐨂' => Ok(Kharoshthi::VowelSignU),
            '𐨃' => Ok(Kharoshthi::VowelSignVocalicR),
            '𐨅' => Ok(Kharoshthi::VowelSignE),
            '𐨆' => Ok(Kharoshthi::VowelSignO),
            '𐨌' => Ok(Kharoshthi::VowelLengthMark),
            '𐨍' => Ok(Kharoshthi::SignDoubleRingBelow),
            '𐨎' => Ok(Kharoshthi::SignAnusvara),
            '𐨏' => Ok(Kharoshthi::SignVisarga),
            '𐨐' => Ok(Kharoshthi::LetterKa),
            '𐨑' => Ok(Kharoshthi::LetterKha),
            '𐨒' => Ok(Kharoshthi::LetterGa),
            '𐨓' => Ok(Kharoshthi::LetterGha),
            '𐨕' => Ok(Kharoshthi::LetterCa),
            '𐨖' => Ok(Kharoshthi::LetterCha),
            '𐨗' => Ok(Kharoshthi::LetterJa),
            '𐨙' => Ok(Kharoshthi::LetterNya),
            '𐨚' => Ok(Kharoshthi::LetterTta),
            '𐨛' => Ok(Kharoshthi::LetterTtha),
            '𐨜' => Ok(Kharoshthi::LetterDda),
            '𐨝' => Ok(Kharoshthi::LetterDdha),
            '𐨞' => Ok(Kharoshthi::LetterNna),
            '𐨟' => Ok(Kharoshthi::LetterTa),
            '𐨠' => Ok(Kharoshthi::LetterTha),
            '𐨡' => Ok(Kharoshthi::LetterDa),
            '𐨢' => Ok(Kharoshthi::LetterDha),
            '𐨣' => Ok(Kharoshthi::LetterNa),
            '𐨤' => Ok(Kharoshthi::LetterPa),
            '𐨥' => Ok(Kharoshthi::LetterPha),
            '𐨦' => Ok(Kharoshthi::LetterBa),
            '𐨧' => Ok(Kharoshthi::LetterBha),
            '𐨨' => Ok(Kharoshthi::LetterMa),
            '𐨩' => Ok(Kharoshthi::LetterYa),
            '𐨪' => Ok(Kharoshthi::LetterRa),
            '𐨫' => Ok(Kharoshthi::LetterLa),
            '𐨬' => Ok(Kharoshthi::LetterVa),
            '𐨭' => Ok(Kharoshthi::LetterSha),
            '𐨮' => Ok(Kharoshthi::LetterSsa),
            '𐨯' => Ok(Kharoshthi::LetterSa),
            '𐨰' => Ok(Kharoshthi::LetterZa),
            '𐨱' => Ok(Kharoshthi::LetterHa),
            '𐨲' => Ok(Kharoshthi::LetterKka),
            '𐨳' => Ok(Kharoshthi::LetterTttha),
            '𐨴' => Ok(Kharoshthi::LetterTtta),
            '𐨵' => Ok(Kharoshthi::LetterVha),
            '𐨸' => Ok(Kharoshthi::SignBarAbove),
            '𐨹' => Ok(Kharoshthi::SignCauda),
            '𐨺' => Ok(Kharoshthi::SignDotBelow),
            '𐨿' => Ok(Kharoshthi::Virama),
            '𐩀' => Ok(Kharoshthi::DigitOne),
            '𐩁' => Ok(Kharoshthi::DigitTwo),
            '𐩂' => Ok(Kharoshthi::DigitThree),
            '𐩃' => Ok(Kharoshthi::DigitFour),
            '𐩄' => Ok(Kharoshthi::NumberTen),
            '𐩅' => Ok(Kharoshthi::NumberTwenty),
            '𐩆' => Ok(Kharoshthi::NumberOneHundred),
            '𐩇' => Ok(Kharoshthi::NumberOneThousand),
            '𐩈' => Ok(Kharoshthi::FractionOneHalf),
            '𐩐' => Ok(Kharoshthi::PunctuationDot),
            '𐩑' => Ok(Kharoshthi::PunctuationSmallCircle),
            '𐩒' => Ok(Kharoshthi::PunctuationCircle),
            '𐩓' => Ok(Kharoshthi::PunctuationCrescentBar),
            '𐩔' => Ok(Kharoshthi::PunctuationMangalam),
            '𐩕' => Ok(Kharoshthi::PunctuationLotus),
            '𐩖' => Ok(Kharoshthi::PunctuationDanda),
            '𐩗' => Ok(Kharoshthi::PunctuationDoubleDanda),
            '𐩘' => Ok(Kharoshthi::PunctuationLines),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Kharoshthi {
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

impl std::convert::TryFrom<u32> for Kharoshthi {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Kharoshthi {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Kharoshthi {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Kharoshthi::LetterA
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Kharoshthi::LetterA => "kharoshthi letter a",
            Kharoshthi::VowelSignI => "kharoshthi vowel sign i",
            Kharoshthi::VowelSignU => "kharoshthi vowel sign u",
            Kharoshthi::VowelSignVocalicR => "kharoshthi vowel sign vocalic r",
            Kharoshthi::VowelSignE => "kharoshthi vowel sign e",
            Kharoshthi::VowelSignO => "kharoshthi vowel sign o",
            Kharoshthi::VowelLengthMark => "kharoshthi vowel length mark",
            Kharoshthi::SignDoubleRingBelow => "kharoshthi sign double ring below",
            Kharoshthi::SignAnusvara => "kharoshthi sign anusvara",
            Kharoshthi::SignVisarga => "kharoshthi sign visarga",
            Kharoshthi::LetterKa => "kharoshthi letter ka",
            Kharoshthi::LetterKha => "kharoshthi letter kha",
            Kharoshthi::LetterGa => "kharoshthi letter ga",
            Kharoshthi::LetterGha => "kharoshthi letter gha",
            Kharoshthi::LetterCa => "kharoshthi letter ca",
            Kharoshthi::LetterCha => "kharoshthi letter cha",
            Kharoshthi::LetterJa => "kharoshthi letter ja",
            Kharoshthi::LetterNya => "kharoshthi letter nya",
            Kharoshthi::LetterTta => "kharoshthi letter tta",
            Kharoshthi::LetterTtha => "kharoshthi letter ttha",
            Kharoshthi::LetterDda => "kharoshthi letter dda",
            Kharoshthi::LetterDdha => "kharoshthi letter ddha",
            Kharoshthi::LetterNna => "kharoshthi letter nna",
            Kharoshthi::LetterTa => "kharoshthi letter ta",
            Kharoshthi::LetterTha => "kharoshthi letter tha",
            Kharoshthi::LetterDa => "kharoshthi letter da",
            Kharoshthi::LetterDha => "kharoshthi letter dha",
            Kharoshthi::LetterNa => "kharoshthi letter na",
            Kharoshthi::LetterPa => "kharoshthi letter pa",
            Kharoshthi::LetterPha => "kharoshthi letter pha",
            Kharoshthi::LetterBa => "kharoshthi letter ba",
            Kharoshthi::LetterBha => "kharoshthi letter bha",
            Kharoshthi::LetterMa => "kharoshthi letter ma",
            Kharoshthi::LetterYa => "kharoshthi letter ya",
            Kharoshthi::LetterRa => "kharoshthi letter ra",
            Kharoshthi::LetterLa => "kharoshthi letter la",
            Kharoshthi::LetterVa => "kharoshthi letter va",
            Kharoshthi::LetterSha => "kharoshthi letter sha",
            Kharoshthi::LetterSsa => "kharoshthi letter ssa",
            Kharoshthi::LetterSa => "kharoshthi letter sa",
            Kharoshthi::LetterZa => "kharoshthi letter za",
            Kharoshthi::LetterHa => "kharoshthi letter ha",
            Kharoshthi::LetterKka => "kharoshthi letter kka",
            Kharoshthi::LetterTttha => "kharoshthi letter tttha",
            Kharoshthi::LetterTtta => "kharoshthi letter ttta",
            Kharoshthi::LetterVha => "kharoshthi letter vha",
            Kharoshthi::SignBarAbove => "kharoshthi sign bar above",
            Kharoshthi::SignCauda => "kharoshthi sign cauda",
            Kharoshthi::SignDotBelow => "kharoshthi sign dot below",
            Kharoshthi::Virama => "kharoshthi virama",
            Kharoshthi::DigitOne => "kharoshthi digit one",
            Kharoshthi::DigitTwo => "kharoshthi digit two",
            Kharoshthi::DigitThree => "kharoshthi digit three",
            Kharoshthi::DigitFour => "kharoshthi digit four",
            Kharoshthi::NumberTen => "kharoshthi number ten",
            Kharoshthi::NumberTwenty => "kharoshthi number twenty",
            Kharoshthi::NumberOneHundred => "kharoshthi number one hundred",
            Kharoshthi::NumberOneThousand => "kharoshthi number one thousand",
            Kharoshthi::FractionOneHalf => "kharoshthi fraction one half",
            Kharoshthi::PunctuationDot => "kharoshthi punctuation dot",
            Kharoshthi::PunctuationSmallCircle => "kharoshthi punctuation small circle",
            Kharoshthi::PunctuationCircle => "kharoshthi punctuation circle",
            Kharoshthi::PunctuationCrescentBar => "kharoshthi punctuation crescent bar",
            Kharoshthi::PunctuationMangalam => "kharoshthi punctuation mangalam",
            Kharoshthi::PunctuationLotus => "kharoshthi punctuation lotus",
            Kharoshthi::PunctuationDanda => "kharoshthi punctuation danda",
            Kharoshthi::PunctuationDoubleDanda => "kharoshthi punctuation double danda",
            Kharoshthi::PunctuationLines => "kharoshthi punctuation lines",
        }
    }
}
