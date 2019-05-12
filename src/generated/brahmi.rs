
/// An enum to represent all characters in the Brahmi block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Brahmi {
    /// \u{11000}: '𑀀'
    SignCandrabindu,
    /// \u{11001}: '𑀁'
    SignAnusvara,
    /// \u{11002}: '𑀂'
    SignVisarga,
    /// \u{11003}: '𑀃'
    SignJihvamuliya,
    /// \u{11004}: '𑀄'
    SignUpadhmaniya,
    /// \u{11005}: '𑀅'
    LetterA,
    /// \u{11006}: '𑀆'
    LetterAa,
    /// \u{11007}: '𑀇'
    LetterI,
    /// \u{11008}: '𑀈'
    LetterIi,
    /// \u{11009}: '𑀉'
    LetterU,
    /// \u{1100a}: '𑀊'
    LetterUu,
    /// \u{1100b}: '𑀋'
    LetterVocalicR,
    /// \u{1100c}: '𑀌'
    LetterVocalicRr,
    /// \u{1100d}: '𑀍'
    LetterVocalicL,
    /// \u{1100e}: '𑀎'
    LetterVocalicLl,
    /// \u{1100f}: '𑀏'
    LetterE,
    /// \u{11010}: '𑀐'
    LetterAi,
    /// \u{11011}: '𑀑'
    LetterO,
    /// \u{11012}: '𑀒'
    LetterAu,
    /// \u{11013}: '𑀓'
    LetterKa,
    /// \u{11014}: '𑀔'
    LetterKha,
    /// \u{11015}: '𑀕'
    LetterGa,
    /// \u{11016}: '𑀖'
    LetterGha,
    /// \u{11017}: '𑀗'
    LetterNga,
    /// \u{11018}: '𑀘'
    LetterCa,
    /// \u{11019}: '𑀙'
    LetterCha,
    /// \u{1101a}: '𑀚'
    LetterJa,
    /// \u{1101b}: '𑀛'
    LetterJha,
    /// \u{1101c}: '𑀜'
    LetterNya,
    /// \u{1101d}: '𑀝'
    LetterTta,
    /// \u{1101e}: '𑀞'
    LetterTtha,
    /// \u{1101f}: '𑀟'
    LetterDda,
    /// \u{11020}: '𑀠'
    LetterDdha,
    /// \u{11021}: '𑀡'
    LetterNna,
    /// \u{11022}: '𑀢'
    LetterTa,
    /// \u{11023}: '𑀣'
    LetterTha,
    /// \u{11024}: '𑀤'
    LetterDa,
    /// \u{11025}: '𑀥'
    LetterDha,
    /// \u{11026}: '𑀦'
    LetterNa,
    /// \u{11027}: '𑀧'
    LetterPa,
    /// \u{11028}: '𑀨'
    LetterPha,
    /// \u{11029}: '𑀩'
    LetterBa,
    /// \u{1102a}: '𑀪'
    LetterBha,
    /// \u{1102b}: '𑀫'
    LetterMa,
    /// \u{1102c}: '𑀬'
    LetterYa,
    /// \u{1102d}: '𑀭'
    LetterRa,
    /// \u{1102e}: '𑀮'
    LetterLa,
    /// \u{1102f}: '𑀯'
    LetterVa,
    /// \u{11030}: '𑀰'
    LetterSha,
    /// \u{11031}: '𑀱'
    LetterSsa,
    /// \u{11032}: '𑀲'
    LetterSa,
    /// \u{11033}: '𑀳'
    LetterHa,
    /// \u{11034}: '𑀴'
    LetterLla,
    /// \u{11035}: '𑀵'
    LetterOldTamilLlla,
    /// \u{11036}: '𑀶'
    LetterOldTamilRra,
    /// \u{11037}: '𑀷'
    LetterOldTamilNnna,
    /// \u{11038}: '𑀸'
    VowelSignAa,
    /// \u{11039}: '𑀹'
    VowelSignBhattiproluAa,
    /// \u{1103a}: '𑀺'
    VowelSignI,
    /// \u{1103b}: '𑀻'
    VowelSignIi,
    /// \u{1103c}: '𑀼'
    VowelSignU,
    /// \u{1103d}: '𑀽'
    VowelSignUu,
    /// \u{1103e}: '𑀾'
    VowelSignVocalicR,
    /// \u{1103f}: '𑀿'
    VowelSignVocalicRr,
    /// \u{11040}: '𑁀'
    VowelSignVocalicL,
    /// \u{11041}: '𑁁'
    VowelSignVocalicLl,
    /// \u{11042}: '𑁂'
    VowelSignE,
    /// \u{11043}: '𑁃'
    VowelSignAi,
    /// \u{11044}: '𑁄'
    VowelSignO,
    /// \u{11045}: '𑁅'
    VowelSignAu,
    /// \u{11046}: '𑁆'
    Virama,
    /// \u{11047}: '𑁇'
    Danda,
    /// \u{11048}: '𑁈'
    DoubleDanda,
    /// \u{11049}: '𑁉'
    PunctuationDot,
    /// \u{1104a}: '𑁊'
    PunctuationDoubleDot,
    /// \u{1104b}: '𑁋'
    PunctuationLine,
    /// \u{1104c}: '𑁌'
    PunctuationCrescentBar,
    /// \u{1104d}: '𑁍'
    PunctuationLotus,
    /// \u{11052}: '𑁒'
    NumberOne,
    /// \u{11053}: '𑁓'
    NumberTwo,
    /// \u{11054}: '𑁔'
    NumberThree,
    /// \u{11055}: '𑁕'
    NumberFour,
    /// \u{11056}: '𑁖'
    NumberFive,
    /// \u{11057}: '𑁗'
    NumberSix,
    /// \u{11058}: '𑁘'
    NumberSeven,
    /// \u{11059}: '𑁙'
    NumberEight,
    /// \u{1105a}: '𑁚'
    NumberNine,
    /// \u{1105b}: '𑁛'
    NumberTen,
    /// \u{1105c}: '𑁜'
    NumberTwenty,
    /// \u{1105d}: '𑁝'
    NumberThirty,
    /// \u{1105e}: '𑁞'
    NumberForty,
    /// \u{1105f}: '𑁟'
    NumberFifty,
    /// \u{11060}: '𑁠'
    NumberSixty,
    /// \u{11061}: '𑁡'
    NumberSeventy,
    /// \u{11062}: '𑁢'
    NumberEighty,
    /// \u{11063}: '𑁣'
    NumberNinety,
    /// \u{11064}: '𑁤'
    NumberOneHundred,
    /// \u{11065}: '𑁥'
    NumberOneThousand,
    /// \u{11066}: '𑁦'
    DigitZero,
    /// \u{11067}: '𑁧'
    DigitOne,
    /// \u{11068}: '𑁨'
    DigitTwo,
    /// \u{11069}: '𑁩'
    DigitThree,
    /// \u{1106a}: '𑁪'
    DigitFour,
    /// \u{1106b}: '𑁫'
    DigitFive,
    /// \u{1106c}: '𑁬'
    DigitSix,
    /// \u{1106d}: '𑁭'
    DigitSeven,
    /// \u{1106e}: '𑁮'
    DigitEight,
    /// \u{1106f}: '𑁯'
    DigitNine,
}

impl Into<char> for Brahmi {
    fn into(self) -> char {
        match self {
            Brahmi::SignCandrabindu => '𑀀',
            Brahmi::SignAnusvara => '𑀁',
            Brahmi::SignVisarga => '𑀂',
            Brahmi::SignJihvamuliya => '𑀃',
            Brahmi::SignUpadhmaniya => '𑀄',
            Brahmi::LetterA => '𑀅',
            Brahmi::LetterAa => '𑀆',
            Brahmi::LetterI => '𑀇',
            Brahmi::LetterIi => '𑀈',
            Brahmi::LetterU => '𑀉',
            Brahmi::LetterUu => '𑀊',
            Brahmi::LetterVocalicR => '𑀋',
            Brahmi::LetterVocalicRr => '𑀌',
            Brahmi::LetterVocalicL => '𑀍',
            Brahmi::LetterVocalicLl => '𑀎',
            Brahmi::LetterE => '𑀏',
            Brahmi::LetterAi => '𑀐',
            Brahmi::LetterO => '𑀑',
            Brahmi::LetterAu => '𑀒',
            Brahmi::LetterKa => '𑀓',
            Brahmi::LetterKha => '𑀔',
            Brahmi::LetterGa => '𑀕',
            Brahmi::LetterGha => '𑀖',
            Brahmi::LetterNga => '𑀗',
            Brahmi::LetterCa => '𑀘',
            Brahmi::LetterCha => '𑀙',
            Brahmi::LetterJa => '𑀚',
            Brahmi::LetterJha => '𑀛',
            Brahmi::LetterNya => '𑀜',
            Brahmi::LetterTta => '𑀝',
            Brahmi::LetterTtha => '𑀞',
            Brahmi::LetterDda => '𑀟',
            Brahmi::LetterDdha => '𑀠',
            Brahmi::LetterNna => '𑀡',
            Brahmi::LetterTa => '𑀢',
            Brahmi::LetterTha => '𑀣',
            Brahmi::LetterDa => '𑀤',
            Brahmi::LetterDha => '𑀥',
            Brahmi::LetterNa => '𑀦',
            Brahmi::LetterPa => '𑀧',
            Brahmi::LetterPha => '𑀨',
            Brahmi::LetterBa => '𑀩',
            Brahmi::LetterBha => '𑀪',
            Brahmi::LetterMa => '𑀫',
            Brahmi::LetterYa => '𑀬',
            Brahmi::LetterRa => '𑀭',
            Brahmi::LetterLa => '𑀮',
            Brahmi::LetterVa => '𑀯',
            Brahmi::LetterSha => '𑀰',
            Brahmi::LetterSsa => '𑀱',
            Brahmi::LetterSa => '𑀲',
            Brahmi::LetterHa => '𑀳',
            Brahmi::LetterLla => '𑀴',
            Brahmi::LetterOldTamilLlla => '𑀵',
            Brahmi::LetterOldTamilRra => '𑀶',
            Brahmi::LetterOldTamilNnna => '𑀷',
            Brahmi::VowelSignAa => '𑀸',
            Brahmi::VowelSignBhattiproluAa => '𑀹',
            Brahmi::VowelSignI => '𑀺',
            Brahmi::VowelSignIi => '𑀻',
            Brahmi::VowelSignU => '𑀼',
            Brahmi::VowelSignUu => '𑀽',
            Brahmi::VowelSignVocalicR => '𑀾',
            Brahmi::VowelSignVocalicRr => '𑀿',
            Brahmi::VowelSignVocalicL => '𑁀',
            Brahmi::VowelSignVocalicLl => '𑁁',
            Brahmi::VowelSignE => '𑁂',
            Brahmi::VowelSignAi => '𑁃',
            Brahmi::VowelSignO => '𑁄',
            Brahmi::VowelSignAu => '𑁅',
            Brahmi::Virama => '𑁆',
            Brahmi::Danda => '𑁇',
            Brahmi::DoubleDanda => '𑁈',
            Brahmi::PunctuationDot => '𑁉',
            Brahmi::PunctuationDoubleDot => '𑁊',
            Brahmi::PunctuationLine => '𑁋',
            Brahmi::PunctuationCrescentBar => '𑁌',
            Brahmi::PunctuationLotus => '𑁍',
            Brahmi::NumberOne => '𑁒',
            Brahmi::NumberTwo => '𑁓',
            Brahmi::NumberThree => '𑁔',
            Brahmi::NumberFour => '𑁕',
            Brahmi::NumberFive => '𑁖',
            Brahmi::NumberSix => '𑁗',
            Brahmi::NumberSeven => '𑁘',
            Brahmi::NumberEight => '𑁙',
            Brahmi::NumberNine => '𑁚',
            Brahmi::NumberTen => '𑁛',
            Brahmi::NumberTwenty => '𑁜',
            Brahmi::NumberThirty => '𑁝',
            Brahmi::NumberForty => '𑁞',
            Brahmi::NumberFifty => '𑁟',
            Brahmi::NumberSixty => '𑁠',
            Brahmi::NumberSeventy => '𑁡',
            Brahmi::NumberEighty => '𑁢',
            Brahmi::NumberNinety => '𑁣',
            Brahmi::NumberOneHundred => '𑁤',
            Brahmi::NumberOneThousand => '𑁥',
            Brahmi::DigitZero => '𑁦',
            Brahmi::DigitOne => '𑁧',
            Brahmi::DigitTwo => '𑁨',
            Brahmi::DigitThree => '𑁩',
            Brahmi::DigitFour => '𑁪',
            Brahmi::DigitFive => '𑁫',
            Brahmi::DigitSix => '𑁬',
            Brahmi::DigitSeven => '𑁭',
            Brahmi::DigitEight => '𑁮',
            Brahmi::DigitNine => '𑁯',
        }
    }
}

impl std::convert::TryFrom<char> for Brahmi {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𑀀' => Ok(Brahmi::SignCandrabindu),
            '𑀁' => Ok(Brahmi::SignAnusvara),
            '𑀂' => Ok(Brahmi::SignVisarga),
            '𑀃' => Ok(Brahmi::SignJihvamuliya),
            '𑀄' => Ok(Brahmi::SignUpadhmaniya),
            '𑀅' => Ok(Brahmi::LetterA),
            '𑀆' => Ok(Brahmi::LetterAa),
            '𑀇' => Ok(Brahmi::LetterI),
            '𑀈' => Ok(Brahmi::LetterIi),
            '𑀉' => Ok(Brahmi::LetterU),
            '𑀊' => Ok(Brahmi::LetterUu),
            '𑀋' => Ok(Brahmi::LetterVocalicR),
            '𑀌' => Ok(Brahmi::LetterVocalicRr),
            '𑀍' => Ok(Brahmi::LetterVocalicL),
            '𑀎' => Ok(Brahmi::LetterVocalicLl),
            '𑀏' => Ok(Brahmi::LetterE),
            '𑀐' => Ok(Brahmi::LetterAi),
            '𑀑' => Ok(Brahmi::LetterO),
            '𑀒' => Ok(Brahmi::LetterAu),
            '𑀓' => Ok(Brahmi::LetterKa),
            '𑀔' => Ok(Brahmi::LetterKha),
            '𑀕' => Ok(Brahmi::LetterGa),
            '𑀖' => Ok(Brahmi::LetterGha),
            '𑀗' => Ok(Brahmi::LetterNga),
            '𑀘' => Ok(Brahmi::LetterCa),
            '𑀙' => Ok(Brahmi::LetterCha),
            '𑀚' => Ok(Brahmi::LetterJa),
            '𑀛' => Ok(Brahmi::LetterJha),
            '𑀜' => Ok(Brahmi::LetterNya),
            '𑀝' => Ok(Brahmi::LetterTta),
            '𑀞' => Ok(Brahmi::LetterTtha),
            '𑀟' => Ok(Brahmi::LetterDda),
            '𑀠' => Ok(Brahmi::LetterDdha),
            '𑀡' => Ok(Brahmi::LetterNna),
            '𑀢' => Ok(Brahmi::LetterTa),
            '𑀣' => Ok(Brahmi::LetterTha),
            '𑀤' => Ok(Brahmi::LetterDa),
            '𑀥' => Ok(Brahmi::LetterDha),
            '𑀦' => Ok(Brahmi::LetterNa),
            '𑀧' => Ok(Brahmi::LetterPa),
            '𑀨' => Ok(Brahmi::LetterPha),
            '𑀩' => Ok(Brahmi::LetterBa),
            '𑀪' => Ok(Brahmi::LetterBha),
            '𑀫' => Ok(Brahmi::LetterMa),
            '𑀬' => Ok(Brahmi::LetterYa),
            '𑀭' => Ok(Brahmi::LetterRa),
            '𑀮' => Ok(Brahmi::LetterLa),
            '𑀯' => Ok(Brahmi::LetterVa),
            '𑀰' => Ok(Brahmi::LetterSha),
            '𑀱' => Ok(Brahmi::LetterSsa),
            '𑀲' => Ok(Brahmi::LetterSa),
            '𑀳' => Ok(Brahmi::LetterHa),
            '𑀴' => Ok(Brahmi::LetterLla),
            '𑀵' => Ok(Brahmi::LetterOldTamilLlla),
            '𑀶' => Ok(Brahmi::LetterOldTamilRra),
            '𑀷' => Ok(Brahmi::LetterOldTamilNnna),
            '𑀸' => Ok(Brahmi::VowelSignAa),
            '𑀹' => Ok(Brahmi::VowelSignBhattiproluAa),
            '𑀺' => Ok(Brahmi::VowelSignI),
            '𑀻' => Ok(Brahmi::VowelSignIi),
            '𑀼' => Ok(Brahmi::VowelSignU),
            '𑀽' => Ok(Brahmi::VowelSignUu),
            '𑀾' => Ok(Brahmi::VowelSignVocalicR),
            '𑀿' => Ok(Brahmi::VowelSignVocalicRr),
            '𑁀' => Ok(Brahmi::VowelSignVocalicL),
            '𑁁' => Ok(Brahmi::VowelSignVocalicLl),
            '𑁂' => Ok(Brahmi::VowelSignE),
            '𑁃' => Ok(Brahmi::VowelSignAi),
            '𑁄' => Ok(Brahmi::VowelSignO),
            '𑁅' => Ok(Brahmi::VowelSignAu),
            '𑁆' => Ok(Brahmi::Virama),
            '𑁇' => Ok(Brahmi::Danda),
            '𑁈' => Ok(Brahmi::DoubleDanda),
            '𑁉' => Ok(Brahmi::PunctuationDot),
            '𑁊' => Ok(Brahmi::PunctuationDoubleDot),
            '𑁋' => Ok(Brahmi::PunctuationLine),
            '𑁌' => Ok(Brahmi::PunctuationCrescentBar),
            '𑁍' => Ok(Brahmi::PunctuationLotus),
            '𑁒' => Ok(Brahmi::NumberOne),
            '𑁓' => Ok(Brahmi::NumberTwo),
            '𑁔' => Ok(Brahmi::NumberThree),
            '𑁕' => Ok(Brahmi::NumberFour),
            '𑁖' => Ok(Brahmi::NumberFive),
            '𑁗' => Ok(Brahmi::NumberSix),
            '𑁘' => Ok(Brahmi::NumberSeven),
            '𑁙' => Ok(Brahmi::NumberEight),
            '𑁚' => Ok(Brahmi::NumberNine),
            '𑁛' => Ok(Brahmi::NumberTen),
            '𑁜' => Ok(Brahmi::NumberTwenty),
            '𑁝' => Ok(Brahmi::NumberThirty),
            '𑁞' => Ok(Brahmi::NumberForty),
            '𑁟' => Ok(Brahmi::NumberFifty),
            '𑁠' => Ok(Brahmi::NumberSixty),
            '𑁡' => Ok(Brahmi::NumberSeventy),
            '𑁢' => Ok(Brahmi::NumberEighty),
            '𑁣' => Ok(Brahmi::NumberNinety),
            '𑁤' => Ok(Brahmi::NumberOneHundred),
            '𑁥' => Ok(Brahmi::NumberOneThousand),
            '𑁦' => Ok(Brahmi::DigitZero),
            '𑁧' => Ok(Brahmi::DigitOne),
            '𑁨' => Ok(Brahmi::DigitTwo),
            '𑁩' => Ok(Brahmi::DigitThree),
            '𑁪' => Ok(Brahmi::DigitFour),
            '𑁫' => Ok(Brahmi::DigitFive),
            '𑁬' => Ok(Brahmi::DigitSix),
            '𑁭' => Ok(Brahmi::DigitSeven),
            '𑁮' => Ok(Brahmi::DigitEight),
            '𑁯' => Ok(Brahmi::DigitNine),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Brahmi {
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

impl std::convert::TryFrom<u32> for Brahmi {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Brahmi {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Brahmi {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Brahmi::SignCandrabindu
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Brahmi{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
