
/// An enum to represent all characters in the Grantha block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Grantha {
    /// \u{11300}: '𑌀'
    SignCombiningAnusvaraAbove,
    /// \u{11301}: '𑌁'
    SignCandrabindu,
    /// \u{11302}: '𑌂'
    SignAnusvara,
    /// \u{11303}: '𑌃'
    SignVisarga,
    /// \u{11305}: '𑌅'
    LetterA,
    /// \u{11306}: '𑌆'
    LetterAa,
    /// \u{11307}: '𑌇'
    LetterI,
    /// \u{11308}: '𑌈'
    LetterIi,
    /// \u{11309}: '𑌉'
    LetterU,
    /// \u{1130a}: '𑌊'
    LetterUu,
    /// \u{1130b}: '𑌋'
    LetterVocalicR,
    /// \u{1130c}: '𑌌'
    LetterVocalicL,
    /// \u{1130f}: '𑌏'
    LetterEe,
    /// \u{11310}: '𑌐'
    LetterAi,
    /// \u{11313}: '𑌓'
    LetterOo,
    /// \u{11314}: '𑌔'
    LetterAu,
    /// \u{11315}: '𑌕'
    LetterKa,
    /// \u{11316}: '𑌖'
    LetterKha,
    /// \u{11317}: '𑌗'
    LetterGa,
    /// \u{11318}: '𑌘'
    LetterGha,
    /// \u{11319}: '𑌙'
    LetterNga,
    /// \u{1131a}: '𑌚'
    LetterCa,
    /// \u{1131b}: '𑌛'
    LetterCha,
    /// \u{1131c}: '𑌜'
    LetterJa,
    /// \u{1131d}: '𑌝'
    LetterJha,
    /// \u{1131e}: '𑌞'
    LetterNya,
    /// \u{1131f}: '𑌟'
    LetterTta,
    /// \u{11320}: '𑌠'
    LetterTtha,
    /// \u{11321}: '𑌡'
    LetterDda,
    /// \u{11322}: '𑌢'
    LetterDdha,
    /// \u{11323}: '𑌣'
    LetterNna,
    /// \u{11324}: '𑌤'
    LetterTa,
    /// \u{11325}: '𑌥'
    LetterTha,
    /// \u{11326}: '𑌦'
    LetterDa,
    /// \u{11327}: '𑌧'
    LetterDha,
    /// \u{11328}: '𑌨'
    LetterNa,
    /// \u{1132a}: '𑌪'
    LetterPa,
    /// \u{1132b}: '𑌫'
    LetterPha,
    /// \u{1132c}: '𑌬'
    LetterBa,
    /// \u{1132d}: '𑌭'
    LetterBha,
    /// \u{1132e}: '𑌮'
    LetterMa,
    /// \u{1132f}: '𑌯'
    LetterYa,
    /// \u{11330}: '𑌰'
    LetterRa,
    /// \u{11332}: '𑌲'
    LetterLa,
    /// \u{11333}: '𑌳'
    LetterLla,
    /// \u{11335}: '𑌵'
    LetterVa,
    /// \u{11336}: '𑌶'
    LetterSha,
    /// \u{11337}: '𑌷'
    LetterSsa,
    /// \u{11338}: '𑌸'
    LetterSa,
    /// \u{11339}: '𑌹'
    LetterHa,
    /// \u{1133b}: '𑌻'
    CombiningBinduBelow,
    /// \u{1133c}: '𑌼'
    SignNukta,
    /// \u{1133d}: '𑌽'
    SignAvagraha,
    /// \u{1133e}: '𑌾'
    VowelSignAa,
    /// \u{1133f}: '𑌿'
    VowelSignI,
    /// \u{11340}: '𑍀'
    VowelSignIi,
    /// \u{11341}: '𑍁'
    VowelSignU,
    /// \u{11342}: '𑍂'
    VowelSignUu,
    /// \u{11343}: '𑍃'
    VowelSignVocalicR,
    /// \u{11344}: '𑍄'
    VowelSignVocalicRr,
    /// \u{11347}: '𑍇'
    VowelSignEe,
    /// \u{11348}: '𑍈'
    VowelSignAi,
    /// \u{1134b}: '𑍋'
    VowelSignOo,
    /// \u{1134c}: '𑍌'
    VowelSignAu,
    /// \u{1134d}: '𑍍'
    SignVirama,
    /// \u{11350}: '𑍐'
    Om,
    /// \u{11357}: '𑍗'
    AuLengthMark,
    /// \u{1135d}: '𑍝'
    SignPluta,
    /// \u{1135e}: '𑍞'
    LetterVedicAnusvara,
    /// \u{1135f}: '𑍟'
    LetterVedicDoubleAnusvara,
    /// \u{11360}: '𑍠'
    LetterVocalicRr,
    /// \u{11361}: '𑍡'
    LetterVocalicLl,
    /// \u{11362}: '𑍢'
    VowelSignVocalicL,
    /// \u{11363}: '𑍣'
    VowelSignVocalicLl,
    /// \u{11366}: '𑍦'
    CombiningDigitZero,
    /// \u{11367}: '𑍧'
    CombiningDigitOne,
    /// \u{11368}: '𑍨'
    CombiningDigitTwo,
    /// \u{11369}: '𑍩'
    CombiningDigitThree,
    /// \u{1136a}: '𑍪'
    CombiningDigitFour,
    /// \u{1136b}: '𑍫'
    CombiningDigitFive,
    /// \u{1136c}: '𑍬'
    CombiningDigitSix,
    /// \u{11370}: '𑍰'
    CombiningLetterA,
    /// \u{11371}: '𑍱'
    CombiningLetterKa,
    /// \u{11372}: '𑍲'
    CombiningLetterNa,
    /// \u{11373}: '𑍳'
    CombiningLetterVi,
    /// \u{11374}: '𑍴'
    CombiningLetterPa,
}

impl Into<char> for Grantha {
    fn into(self) -> char {
        match self {
            Grantha::SignCombiningAnusvaraAbove => '𑌀',
            Grantha::SignCandrabindu => '𑌁',
            Grantha::SignAnusvara => '𑌂',
            Grantha::SignVisarga => '𑌃',
            Grantha::LetterA => '𑌅',
            Grantha::LetterAa => '𑌆',
            Grantha::LetterI => '𑌇',
            Grantha::LetterIi => '𑌈',
            Grantha::LetterU => '𑌉',
            Grantha::LetterUu => '𑌊',
            Grantha::LetterVocalicR => '𑌋',
            Grantha::LetterVocalicL => '𑌌',
            Grantha::LetterEe => '𑌏',
            Grantha::LetterAi => '𑌐',
            Grantha::LetterOo => '𑌓',
            Grantha::LetterAu => '𑌔',
            Grantha::LetterKa => '𑌕',
            Grantha::LetterKha => '𑌖',
            Grantha::LetterGa => '𑌗',
            Grantha::LetterGha => '𑌘',
            Grantha::LetterNga => '𑌙',
            Grantha::LetterCa => '𑌚',
            Grantha::LetterCha => '𑌛',
            Grantha::LetterJa => '𑌜',
            Grantha::LetterJha => '𑌝',
            Grantha::LetterNya => '𑌞',
            Grantha::LetterTta => '𑌟',
            Grantha::LetterTtha => '𑌠',
            Grantha::LetterDda => '𑌡',
            Grantha::LetterDdha => '𑌢',
            Grantha::LetterNna => '𑌣',
            Grantha::LetterTa => '𑌤',
            Grantha::LetterTha => '𑌥',
            Grantha::LetterDa => '𑌦',
            Grantha::LetterDha => '𑌧',
            Grantha::LetterNa => '𑌨',
            Grantha::LetterPa => '𑌪',
            Grantha::LetterPha => '𑌫',
            Grantha::LetterBa => '𑌬',
            Grantha::LetterBha => '𑌭',
            Grantha::LetterMa => '𑌮',
            Grantha::LetterYa => '𑌯',
            Grantha::LetterRa => '𑌰',
            Grantha::LetterLa => '𑌲',
            Grantha::LetterLla => '𑌳',
            Grantha::LetterVa => '𑌵',
            Grantha::LetterSha => '𑌶',
            Grantha::LetterSsa => '𑌷',
            Grantha::LetterSa => '𑌸',
            Grantha::LetterHa => '𑌹',
            Grantha::CombiningBinduBelow => '𑌻',
            Grantha::SignNukta => '𑌼',
            Grantha::SignAvagraha => '𑌽',
            Grantha::VowelSignAa => '𑌾',
            Grantha::VowelSignI => '𑌿',
            Grantha::VowelSignIi => '𑍀',
            Grantha::VowelSignU => '𑍁',
            Grantha::VowelSignUu => '𑍂',
            Grantha::VowelSignVocalicR => '𑍃',
            Grantha::VowelSignVocalicRr => '𑍄',
            Grantha::VowelSignEe => '𑍇',
            Grantha::VowelSignAi => '𑍈',
            Grantha::VowelSignOo => '𑍋',
            Grantha::VowelSignAu => '𑍌',
            Grantha::SignVirama => '𑍍',
            Grantha::Om => '𑍐',
            Grantha::AuLengthMark => '𑍗',
            Grantha::SignPluta => '𑍝',
            Grantha::LetterVedicAnusvara => '𑍞',
            Grantha::LetterVedicDoubleAnusvara => '𑍟',
            Grantha::LetterVocalicRr => '𑍠',
            Grantha::LetterVocalicLl => '𑍡',
            Grantha::VowelSignVocalicL => '𑍢',
            Grantha::VowelSignVocalicLl => '𑍣',
            Grantha::CombiningDigitZero => '𑍦',
            Grantha::CombiningDigitOne => '𑍧',
            Grantha::CombiningDigitTwo => '𑍨',
            Grantha::CombiningDigitThree => '𑍩',
            Grantha::CombiningDigitFour => '𑍪',
            Grantha::CombiningDigitFive => '𑍫',
            Grantha::CombiningDigitSix => '𑍬',
            Grantha::CombiningLetterA => '𑍰',
            Grantha::CombiningLetterKa => '𑍱',
            Grantha::CombiningLetterNa => '𑍲',
            Grantha::CombiningLetterVi => '𑍳',
            Grantha::CombiningLetterPa => '𑍴',
        }
    }
}

impl std::convert::TryFrom<char> for Grantha {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𑌀' => Ok(Grantha::SignCombiningAnusvaraAbove),
            '𑌁' => Ok(Grantha::SignCandrabindu),
            '𑌂' => Ok(Grantha::SignAnusvara),
            '𑌃' => Ok(Grantha::SignVisarga),
            '𑌅' => Ok(Grantha::LetterA),
            '𑌆' => Ok(Grantha::LetterAa),
            '𑌇' => Ok(Grantha::LetterI),
            '𑌈' => Ok(Grantha::LetterIi),
            '𑌉' => Ok(Grantha::LetterU),
            '𑌊' => Ok(Grantha::LetterUu),
            '𑌋' => Ok(Grantha::LetterVocalicR),
            '𑌌' => Ok(Grantha::LetterVocalicL),
            '𑌏' => Ok(Grantha::LetterEe),
            '𑌐' => Ok(Grantha::LetterAi),
            '𑌓' => Ok(Grantha::LetterOo),
            '𑌔' => Ok(Grantha::LetterAu),
            '𑌕' => Ok(Grantha::LetterKa),
            '𑌖' => Ok(Grantha::LetterKha),
            '𑌗' => Ok(Grantha::LetterGa),
            '𑌘' => Ok(Grantha::LetterGha),
            '𑌙' => Ok(Grantha::LetterNga),
            '𑌚' => Ok(Grantha::LetterCa),
            '𑌛' => Ok(Grantha::LetterCha),
            '𑌜' => Ok(Grantha::LetterJa),
            '𑌝' => Ok(Grantha::LetterJha),
            '𑌞' => Ok(Grantha::LetterNya),
            '𑌟' => Ok(Grantha::LetterTta),
            '𑌠' => Ok(Grantha::LetterTtha),
            '𑌡' => Ok(Grantha::LetterDda),
            '𑌢' => Ok(Grantha::LetterDdha),
            '𑌣' => Ok(Grantha::LetterNna),
            '𑌤' => Ok(Grantha::LetterTa),
            '𑌥' => Ok(Grantha::LetterTha),
            '𑌦' => Ok(Grantha::LetterDa),
            '𑌧' => Ok(Grantha::LetterDha),
            '𑌨' => Ok(Grantha::LetterNa),
            '𑌪' => Ok(Grantha::LetterPa),
            '𑌫' => Ok(Grantha::LetterPha),
            '𑌬' => Ok(Grantha::LetterBa),
            '𑌭' => Ok(Grantha::LetterBha),
            '𑌮' => Ok(Grantha::LetterMa),
            '𑌯' => Ok(Grantha::LetterYa),
            '𑌰' => Ok(Grantha::LetterRa),
            '𑌲' => Ok(Grantha::LetterLa),
            '𑌳' => Ok(Grantha::LetterLla),
            '𑌵' => Ok(Grantha::LetterVa),
            '𑌶' => Ok(Grantha::LetterSha),
            '𑌷' => Ok(Grantha::LetterSsa),
            '𑌸' => Ok(Grantha::LetterSa),
            '𑌹' => Ok(Grantha::LetterHa),
            '𑌻' => Ok(Grantha::CombiningBinduBelow),
            '𑌼' => Ok(Grantha::SignNukta),
            '𑌽' => Ok(Grantha::SignAvagraha),
            '𑌾' => Ok(Grantha::VowelSignAa),
            '𑌿' => Ok(Grantha::VowelSignI),
            '𑍀' => Ok(Grantha::VowelSignIi),
            '𑍁' => Ok(Grantha::VowelSignU),
            '𑍂' => Ok(Grantha::VowelSignUu),
            '𑍃' => Ok(Grantha::VowelSignVocalicR),
            '𑍄' => Ok(Grantha::VowelSignVocalicRr),
            '𑍇' => Ok(Grantha::VowelSignEe),
            '𑍈' => Ok(Grantha::VowelSignAi),
            '𑍋' => Ok(Grantha::VowelSignOo),
            '𑍌' => Ok(Grantha::VowelSignAu),
            '𑍍' => Ok(Grantha::SignVirama),
            '𑍐' => Ok(Grantha::Om),
            '𑍗' => Ok(Grantha::AuLengthMark),
            '𑍝' => Ok(Grantha::SignPluta),
            '𑍞' => Ok(Grantha::LetterVedicAnusvara),
            '𑍟' => Ok(Grantha::LetterVedicDoubleAnusvara),
            '𑍠' => Ok(Grantha::LetterVocalicRr),
            '𑍡' => Ok(Grantha::LetterVocalicLl),
            '𑍢' => Ok(Grantha::VowelSignVocalicL),
            '𑍣' => Ok(Grantha::VowelSignVocalicLl),
            '𑍦' => Ok(Grantha::CombiningDigitZero),
            '𑍧' => Ok(Grantha::CombiningDigitOne),
            '𑍨' => Ok(Grantha::CombiningDigitTwo),
            '𑍩' => Ok(Grantha::CombiningDigitThree),
            '𑍪' => Ok(Grantha::CombiningDigitFour),
            '𑍫' => Ok(Grantha::CombiningDigitFive),
            '𑍬' => Ok(Grantha::CombiningDigitSix),
            '𑍰' => Ok(Grantha::CombiningLetterA),
            '𑍱' => Ok(Grantha::CombiningLetterKa),
            '𑍲' => Ok(Grantha::CombiningLetterNa),
            '𑍳' => Ok(Grantha::CombiningLetterVi),
            '𑍴' => Ok(Grantha::CombiningLetterPa),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Grantha {
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

impl std::convert::TryFrom<u32> for Grantha {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Grantha {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Grantha {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Grantha::SignCombiningAnusvaraAbove
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Grantha{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
