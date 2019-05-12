
/// An enum to represent all characters in the Ahom block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Ahom {
    /// \u{11700}: '𑜀'
    LetterKa,
    /// \u{11701}: '𑜁'
    LetterKha,
    /// \u{11702}: '𑜂'
    LetterNga,
    /// \u{11703}: '𑜃'
    LetterNa,
    /// \u{11704}: '𑜄'
    LetterTa,
    /// \u{11705}: '𑜅'
    LetterAlternateTa,
    /// \u{11706}: '𑜆'
    LetterPa,
    /// \u{11707}: '𑜇'
    LetterPha,
    /// \u{11708}: '𑜈'
    LetterBa,
    /// \u{11709}: '𑜉'
    LetterMa,
    /// \u{1170a}: '𑜊'
    LetterJa,
    /// \u{1170b}: '𑜋'
    LetterCha,
    /// \u{1170c}: '𑜌'
    LetterTha,
    /// \u{1170d}: '𑜍'
    LetterRa,
    /// \u{1170e}: '𑜎'
    LetterLa,
    /// \u{1170f}: '𑜏'
    LetterSa,
    /// \u{11710}: '𑜐'
    LetterNya,
    /// \u{11711}: '𑜑'
    LetterHa,
    /// \u{11712}: '𑜒'
    LetterA,
    /// \u{11713}: '𑜓'
    LetterDa,
    /// \u{11714}: '𑜔'
    LetterDha,
    /// \u{11715}: '𑜕'
    LetterGa,
    /// \u{11716}: '𑜖'
    LetterAlternateGa,
    /// \u{11717}: '𑜗'
    LetterGha,
    /// \u{11718}: '𑜘'
    LetterBha,
    /// \u{11719}: '𑜙'
    LetterJha,
    /// \u{1171a}: '𑜚'
    LetterAlternateBa,
    /// \u{1171d}: '𑜝'
    ConsonantSignMedialLa,
    /// \u{1171e}: '𑜞'
    ConsonantSignMedialRa,
    /// \u{1171f}: '𑜟'
    ConsonantSignMedialLigatingRa,
    /// \u{11720}: '𑜠'
    VowelSignA,
    /// \u{11721}: '𑜡'
    VowelSignAa,
    /// \u{11722}: '𑜢'
    VowelSignI,
    /// \u{11723}: '𑜣'
    VowelSignIi,
    /// \u{11724}: '𑜤'
    VowelSignU,
    /// \u{11725}: '𑜥'
    VowelSignUu,
    /// \u{11726}: '𑜦'
    VowelSignE,
    /// \u{11727}: '𑜧'
    VowelSignAw,
    /// \u{11728}: '𑜨'
    VowelSignO,
    /// \u{11729}: '𑜩'
    VowelSignAi,
    /// \u{1172a}: '𑜪'
    VowelSignAm,
    /// \u{1172b}: '𑜫'
    SignKiller,
    /// \u{11730}: '𑜰'
    DigitZero,
    /// \u{11731}: '𑜱'
    DigitOne,
    /// \u{11732}: '𑜲'
    DigitTwo,
    /// \u{11733}: '𑜳'
    DigitThree,
    /// \u{11734}: '𑜴'
    DigitFour,
    /// \u{11735}: '𑜵'
    DigitFive,
    /// \u{11736}: '𑜶'
    DigitSix,
    /// \u{11737}: '𑜷'
    DigitSeven,
    /// \u{11738}: '𑜸'
    DigitEight,
    /// \u{11739}: '𑜹'
    DigitNine,
    /// \u{1173a}: '𑜺'
    NumberTen,
    /// \u{1173b}: '𑜻'
    NumberTwenty,
    /// \u{1173c}: '𑜼'
    SignSmallSection,
    /// \u{1173d}: '𑜽'
    SignSection,
    /// \u{1173e}: '𑜾'
    SignRulai,
}

impl Into<char> for Ahom {
    fn into(self) -> char {
        match self {
            Ahom::LetterKa => '𑜀',
            Ahom::LetterKha => '𑜁',
            Ahom::LetterNga => '𑜂',
            Ahom::LetterNa => '𑜃',
            Ahom::LetterTa => '𑜄',
            Ahom::LetterAlternateTa => '𑜅',
            Ahom::LetterPa => '𑜆',
            Ahom::LetterPha => '𑜇',
            Ahom::LetterBa => '𑜈',
            Ahom::LetterMa => '𑜉',
            Ahom::LetterJa => '𑜊',
            Ahom::LetterCha => '𑜋',
            Ahom::LetterTha => '𑜌',
            Ahom::LetterRa => '𑜍',
            Ahom::LetterLa => '𑜎',
            Ahom::LetterSa => '𑜏',
            Ahom::LetterNya => '𑜐',
            Ahom::LetterHa => '𑜑',
            Ahom::LetterA => '𑜒',
            Ahom::LetterDa => '𑜓',
            Ahom::LetterDha => '𑜔',
            Ahom::LetterGa => '𑜕',
            Ahom::LetterAlternateGa => '𑜖',
            Ahom::LetterGha => '𑜗',
            Ahom::LetterBha => '𑜘',
            Ahom::LetterJha => '𑜙',
            Ahom::LetterAlternateBa => '𑜚',
            Ahom::ConsonantSignMedialLa => '𑜝',
            Ahom::ConsonantSignMedialRa => '𑜞',
            Ahom::ConsonantSignMedialLigatingRa => '𑜟',
            Ahom::VowelSignA => '𑜠',
            Ahom::VowelSignAa => '𑜡',
            Ahom::VowelSignI => '𑜢',
            Ahom::VowelSignIi => '𑜣',
            Ahom::VowelSignU => '𑜤',
            Ahom::VowelSignUu => '𑜥',
            Ahom::VowelSignE => '𑜦',
            Ahom::VowelSignAw => '𑜧',
            Ahom::VowelSignO => '𑜨',
            Ahom::VowelSignAi => '𑜩',
            Ahom::VowelSignAm => '𑜪',
            Ahom::SignKiller => '𑜫',
            Ahom::DigitZero => '𑜰',
            Ahom::DigitOne => '𑜱',
            Ahom::DigitTwo => '𑜲',
            Ahom::DigitThree => '𑜳',
            Ahom::DigitFour => '𑜴',
            Ahom::DigitFive => '𑜵',
            Ahom::DigitSix => '𑜶',
            Ahom::DigitSeven => '𑜷',
            Ahom::DigitEight => '𑜸',
            Ahom::DigitNine => '𑜹',
            Ahom::NumberTen => '𑜺',
            Ahom::NumberTwenty => '𑜻',
            Ahom::SignSmallSection => '𑜼',
            Ahom::SignSection => '𑜽',
            Ahom::SignRulai => '𑜾',
        }
    }
}

impl std::convert::TryFrom<char> for Ahom {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𑜀' => Ok(Ahom::LetterKa),
            '𑜁' => Ok(Ahom::LetterKha),
            '𑜂' => Ok(Ahom::LetterNga),
            '𑜃' => Ok(Ahom::LetterNa),
            '𑜄' => Ok(Ahom::LetterTa),
            '𑜅' => Ok(Ahom::LetterAlternateTa),
            '𑜆' => Ok(Ahom::LetterPa),
            '𑜇' => Ok(Ahom::LetterPha),
            '𑜈' => Ok(Ahom::LetterBa),
            '𑜉' => Ok(Ahom::LetterMa),
            '𑜊' => Ok(Ahom::LetterJa),
            '𑜋' => Ok(Ahom::LetterCha),
            '𑜌' => Ok(Ahom::LetterTha),
            '𑜍' => Ok(Ahom::LetterRa),
            '𑜎' => Ok(Ahom::LetterLa),
            '𑜏' => Ok(Ahom::LetterSa),
            '𑜐' => Ok(Ahom::LetterNya),
            '𑜑' => Ok(Ahom::LetterHa),
            '𑜒' => Ok(Ahom::LetterA),
            '𑜓' => Ok(Ahom::LetterDa),
            '𑜔' => Ok(Ahom::LetterDha),
            '𑜕' => Ok(Ahom::LetterGa),
            '𑜖' => Ok(Ahom::LetterAlternateGa),
            '𑜗' => Ok(Ahom::LetterGha),
            '𑜘' => Ok(Ahom::LetterBha),
            '𑜙' => Ok(Ahom::LetterJha),
            '𑜚' => Ok(Ahom::LetterAlternateBa),
            '𑜝' => Ok(Ahom::ConsonantSignMedialLa),
            '𑜞' => Ok(Ahom::ConsonantSignMedialRa),
            '𑜟' => Ok(Ahom::ConsonantSignMedialLigatingRa),
            '𑜠' => Ok(Ahom::VowelSignA),
            '𑜡' => Ok(Ahom::VowelSignAa),
            '𑜢' => Ok(Ahom::VowelSignI),
            '𑜣' => Ok(Ahom::VowelSignIi),
            '𑜤' => Ok(Ahom::VowelSignU),
            '𑜥' => Ok(Ahom::VowelSignUu),
            '𑜦' => Ok(Ahom::VowelSignE),
            '𑜧' => Ok(Ahom::VowelSignAw),
            '𑜨' => Ok(Ahom::VowelSignO),
            '𑜩' => Ok(Ahom::VowelSignAi),
            '𑜪' => Ok(Ahom::VowelSignAm),
            '𑜫' => Ok(Ahom::SignKiller),
            '𑜰' => Ok(Ahom::DigitZero),
            '𑜱' => Ok(Ahom::DigitOne),
            '𑜲' => Ok(Ahom::DigitTwo),
            '𑜳' => Ok(Ahom::DigitThree),
            '𑜴' => Ok(Ahom::DigitFour),
            '𑜵' => Ok(Ahom::DigitFive),
            '𑜶' => Ok(Ahom::DigitSix),
            '𑜷' => Ok(Ahom::DigitSeven),
            '𑜸' => Ok(Ahom::DigitEight),
            '𑜹' => Ok(Ahom::DigitNine),
            '𑜺' => Ok(Ahom::NumberTen),
            '𑜻' => Ok(Ahom::NumberTwenty),
            '𑜼' => Ok(Ahom::SignSmallSection),
            '𑜽' => Ok(Ahom::SignSection),
            '𑜾' => Ok(Ahom::SignRulai),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Ahom {
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

impl std::convert::TryFrom<u32> for Ahom {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Ahom {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Ahom {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Ahom::LetterKa
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Ahom{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
