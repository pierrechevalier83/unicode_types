
/// An enum to represent all characters in the Marchen block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Marchen {
    /// \u{11c70}: '𑱰'
    HeadMark,
    /// \u{11c71}: '𑱱'
    MarkShad,
    /// \u{11c72}: '𑱲'
    LetterKa,
    /// \u{11c73}: '𑱳'
    LetterKha,
    /// \u{11c74}: '𑱴'
    LetterGa,
    /// \u{11c75}: '𑱵'
    LetterNga,
    /// \u{11c76}: '𑱶'
    LetterCa,
    /// \u{11c77}: '𑱷'
    LetterCha,
    /// \u{11c78}: '𑱸'
    LetterJa,
    /// \u{11c79}: '𑱹'
    LetterNya,
    /// \u{11c7a}: '𑱺'
    LetterTa,
    /// \u{11c7b}: '𑱻'
    LetterTha,
    /// \u{11c7c}: '𑱼'
    LetterDa,
    /// \u{11c7d}: '𑱽'
    LetterNa,
    /// \u{11c7e}: '𑱾'
    LetterPa,
    /// \u{11c7f}: '𑱿'
    LetterPha,
    /// \u{11c80}: '𑲀'
    LetterBa,
    /// \u{11c81}: '𑲁'
    LetterMa,
    /// \u{11c82}: '𑲂'
    LetterTsa,
    /// \u{11c83}: '𑲃'
    LetterTsha,
    /// \u{11c84}: '𑲄'
    LetterDza,
    /// \u{11c85}: '𑲅'
    LetterWa,
    /// \u{11c86}: '𑲆'
    LetterZha,
    /// \u{11c87}: '𑲇'
    LetterZa,
    /// \u{11c88}: '𑲈'
    LetterDashA,
    /// \u{11c89}: '𑲉'
    LetterYa,
    /// \u{11c8a}: '𑲊'
    LetterRa,
    /// \u{11c8b}: '𑲋'
    LetterLa,
    /// \u{11c8c}: '𑲌'
    LetterSha,
    /// \u{11c8d}: '𑲍'
    LetterSa,
    /// \u{11c8e}: '𑲎'
    LetterHa,
    /// \u{11c8f}: '𑲏'
    LetterA,
    /// \u{11c92}: '𑲒'
    SubjoinedLetterKa,
    /// \u{11c93}: '𑲓'
    SubjoinedLetterKha,
    /// \u{11c94}: '𑲔'
    SubjoinedLetterGa,
    /// \u{11c95}: '𑲕'
    SubjoinedLetterNga,
    /// \u{11c96}: '𑲖'
    SubjoinedLetterCa,
    /// \u{11c97}: '𑲗'
    SubjoinedLetterCha,
    /// \u{11c98}: '𑲘'
    SubjoinedLetterJa,
    /// \u{11c99}: '𑲙'
    SubjoinedLetterNya,
    /// \u{11c9a}: '𑲚'
    SubjoinedLetterTa,
    /// \u{11c9b}: '𑲛'
    SubjoinedLetterTha,
    /// \u{11c9c}: '𑲜'
    SubjoinedLetterDa,
    /// \u{11c9d}: '𑲝'
    SubjoinedLetterNa,
    /// \u{11c9e}: '𑲞'
    SubjoinedLetterPa,
    /// \u{11c9f}: '𑲟'
    SubjoinedLetterPha,
    /// \u{11ca0}: '𑲠'
    SubjoinedLetterBa,
    /// \u{11ca1}: '𑲡'
    SubjoinedLetterMa,
    /// \u{11ca2}: '𑲢'
    SubjoinedLetterTsa,
    /// \u{11ca3}: '𑲣'
    SubjoinedLetterTsha,
    /// \u{11ca4}: '𑲤'
    SubjoinedLetterDza,
    /// \u{11ca5}: '𑲥'
    SubjoinedLetterWa,
    /// \u{11ca6}: '𑲦'
    SubjoinedLetterZha,
    /// \u{11ca7}: '𑲧'
    SubjoinedLetterZa,
    /// \u{11ca9}: '𑲩'
    SubjoinedLetterYa,
    /// \u{11caa}: '𑲪'
    SubjoinedLetterRa,
    /// \u{11cab}: '𑲫'
    SubjoinedLetterLa,
    /// \u{11cac}: '𑲬'
    SubjoinedLetterSha,
    /// \u{11cad}: '𑲭'
    SubjoinedLetterSa,
    /// \u{11cae}: '𑲮'
    SubjoinedLetterHa,
    /// \u{11caf}: '𑲯'
    SubjoinedLetterA,
    /// \u{11cb0}: '𑲰'
    VowelSignAa,
    /// \u{11cb1}: '𑲱'
    VowelSignI,
    /// \u{11cb2}: '𑲲'
    VowelSignU,
    /// \u{11cb3}: '𑲳'
    VowelSignE,
    /// \u{11cb4}: '𑲴'
    VowelSignO,
    /// \u{11cb5}: '𑲵'
    SignAnusvara,
    /// \u{11cb6}: '𑲶'
    SignCandrabindu,
}

impl Into<char> for Marchen {
    fn into(self) -> char {
        match self {
            Marchen::HeadMark => '𑱰',
            Marchen::MarkShad => '𑱱',
            Marchen::LetterKa => '𑱲',
            Marchen::LetterKha => '𑱳',
            Marchen::LetterGa => '𑱴',
            Marchen::LetterNga => '𑱵',
            Marchen::LetterCa => '𑱶',
            Marchen::LetterCha => '𑱷',
            Marchen::LetterJa => '𑱸',
            Marchen::LetterNya => '𑱹',
            Marchen::LetterTa => '𑱺',
            Marchen::LetterTha => '𑱻',
            Marchen::LetterDa => '𑱼',
            Marchen::LetterNa => '𑱽',
            Marchen::LetterPa => '𑱾',
            Marchen::LetterPha => '𑱿',
            Marchen::LetterBa => '𑲀',
            Marchen::LetterMa => '𑲁',
            Marchen::LetterTsa => '𑲂',
            Marchen::LetterTsha => '𑲃',
            Marchen::LetterDza => '𑲄',
            Marchen::LetterWa => '𑲅',
            Marchen::LetterZha => '𑲆',
            Marchen::LetterZa => '𑲇',
            Marchen::LetterDashA => '𑲈',
            Marchen::LetterYa => '𑲉',
            Marchen::LetterRa => '𑲊',
            Marchen::LetterLa => '𑲋',
            Marchen::LetterSha => '𑲌',
            Marchen::LetterSa => '𑲍',
            Marchen::LetterHa => '𑲎',
            Marchen::LetterA => '𑲏',
            Marchen::SubjoinedLetterKa => '𑲒',
            Marchen::SubjoinedLetterKha => '𑲓',
            Marchen::SubjoinedLetterGa => '𑲔',
            Marchen::SubjoinedLetterNga => '𑲕',
            Marchen::SubjoinedLetterCa => '𑲖',
            Marchen::SubjoinedLetterCha => '𑲗',
            Marchen::SubjoinedLetterJa => '𑲘',
            Marchen::SubjoinedLetterNya => '𑲙',
            Marchen::SubjoinedLetterTa => '𑲚',
            Marchen::SubjoinedLetterTha => '𑲛',
            Marchen::SubjoinedLetterDa => '𑲜',
            Marchen::SubjoinedLetterNa => '𑲝',
            Marchen::SubjoinedLetterPa => '𑲞',
            Marchen::SubjoinedLetterPha => '𑲟',
            Marchen::SubjoinedLetterBa => '𑲠',
            Marchen::SubjoinedLetterMa => '𑲡',
            Marchen::SubjoinedLetterTsa => '𑲢',
            Marchen::SubjoinedLetterTsha => '𑲣',
            Marchen::SubjoinedLetterDza => '𑲤',
            Marchen::SubjoinedLetterWa => '𑲥',
            Marchen::SubjoinedLetterZha => '𑲦',
            Marchen::SubjoinedLetterZa => '𑲧',
            Marchen::SubjoinedLetterYa => '𑲩',
            Marchen::SubjoinedLetterRa => '𑲪',
            Marchen::SubjoinedLetterLa => '𑲫',
            Marchen::SubjoinedLetterSha => '𑲬',
            Marchen::SubjoinedLetterSa => '𑲭',
            Marchen::SubjoinedLetterHa => '𑲮',
            Marchen::SubjoinedLetterA => '𑲯',
            Marchen::VowelSignAa => '𑲰',
            Marchen::VowelSignI => '𑲱',
            Marchen::VowelSignU => '𑲲',
            Marchen::VowelSignE => '𑲳',
            Marchen::VowelSignO => '𑲴',
            Marchen::SignAnusvara => '𑲵',
            Marchen::SignCandrabindu => '𑲶',
        }
    }
}

impl std::convert::TryFrom<char> for Marchen {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𑱰' => Ok(Marchen::HeadMark),
            '𑱱' => Ok(Marchen::MarkShad),
            '𑱲' => Ok(Marchen::LetterKa),
            '𑱳' => Ok(Marchen::LetterKha),
            '𑱴' => Ok(Marchen::LetterGa),
            '𑱵' => Ok(Marchen::LetterNga),
            '𑱶' => Ok(Marchen::LetterCa),
            '𑱷' => Ok(Marchen::LetterCha),
            '𑱸' => Ok(Marchen::LetterJa),
            '𑱹' => Ok(Marchen::LetterNya),
            '𑱺' => Ok(Marchen::LetterTa),
            '𑱻' => Ok(Marchen::LetterTha),
            '𑱼' => Ok(Marchen::LetterDa),
            '𑱽' => Ok(Marchen::LetterNa),
            '𑱾' => Ok(Marchen::LetterPa),
            '𑱿' => Ok(Marchen::LetterPha),
            '𑲀' => Ok(Marchen::LetterBa),
            '𑲁' => Ok(Marchen::LetterMa),
            '𑲂' => Ok(Marchen::LetterTsa),
            '𑲃' => Ok(Marchen::LetterTsha),
            '𑲄' => Ok(Marchen::LetterDza),
            '𑲅' => Ok(Marchen::LetterWa),
            '𑲆' => Ok(Marchen::LetterZha),
            '𑲇' => Ok(Marchen::LetterZa),
            '𑲈' => Ok(Marchen::LetterDashA),
            '𑲉' => Ok(Marchen::LetterYa),
            '𑲊' => Ok(Marchen::LetterRa),
            '𑲋' => Ok(Marchen::LetterLa),
            '𑲌' => Ok(Marchen::LetterSha),
            '𑲍' => Ok(Marchen::LetterSa),
            '𑲎' => Ok(Marchen::LetterHa),
            '𑲏' => Ok(Marchen::LetterA),
            '𑲒' => Ok(Marchen::SubjoinedLetterKa),
            '𑲓' => Ok(Marchen::SubjoinedLetterKha),
            '𑲔' => Ok(Marchen::SubjoinedLetterGa),
            '𑲕' => Ok(Marchen::SubjoinedLetterNga),
            '𑲖' => Ok(Marchen::SubjoinedLetterCa),
            '𑲗' => Ok(Marchen::SubjoinedLetterCha),
            '𑲘' => Ok(Marchen::SubjoinedLetterJa),
            '𑲙' => Ok(Marchen::SubjoinedLetterNya),
            '𑲚' => Ok(Marchen::SubjoinedLetterTa),
            '𑲛' => Ok(Marchen::SubjoinedLetterTha),
            '𑲜' => Ok(Marchen::SubjoinedLetterDa),
            '𑲝' => Ok(Marchen::SubjoinedLetterNa),
            '𑲞' => Ok(Marchen::SubjoinedLetterPa),
            '𑲟' => Ok(Marchen::SubjoinedLetterPha),
            '𑲠' => Ok(Marchen::SubjoinedLetterBa),
            '𑲡' => Ok(Marchen::SubjoinedLetterMa),
            '𑲢' => Ok(Marchen::SubjoinedLetterTsa),
            '𑲣' => Ok(Marchen::SubjoinedLetterTsha),
            '𑲤' => Ok(Marchen::SubjoinedLetterDza),
            '𑲥' => Ok(Marchen::SubjoinedLetterWa),
            '𑲦' => Ok(Marchen::SubjoinedLetterZha),
            '𑲧' => Ok(Marchen::SubjoinedLetterZa),
            '𑲩' => Ok(Marchen::SubjoinedLetterYa),
            '𑲪' => Ok(Marchen::SubjoinedLetterRa),
            '𑲫' => Ok(Marchen::SubjoinedLetterLa),
            '𑲬' => Ok(Marchen::SubjoinedLetterSha),
            '𑲭' => Ok(Marchen::SubjoinedLetterSa),
            '𑲮' => Ok(Marchen::SubjoinedLetterHa),
            '𑲯' => Ok(Marchen::SubjoinedLetterA),
            '𑲰' => Ok(Marchen::VowelSignAa),
            '𑲱' => Ok(Marchen::VowelSignI),
            '𑲲' => Ok(Marchen::VowelSignU),
            '𑲳' => Ok(Marchen::VowelSignE),
            '𑲴' => Ok(Marchen::VowelSignO),
            '𑲵' => Ok(Marchen::SignAnusvara),
            '𑲶' => Ok(Marchen::SignCandrabindu),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Marchen {
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

impl std::convert::TryFrom<u32> for Marchen {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Marchen {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Marchen {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Marchen::HeadMark
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Marchen::HeadMark => "marchen head mark",
            Marchen::MarkShad => "marchen mark shad",
            Marchen::LetterKa => "marchen letter ka",
            Marchen::LetterKha => "marchen letter kha",
            Marchen::LetterGa => "marchen letter ga",
            Marchen::LetterNga => "marchen letter nga",
            Marchen::LetterCa => "marchen letter ca",
            Marchen::LetterCha => "marchen letter cha",
            Marchen::LetterJa => "marchen letter ja",
            Marchen::LetterNya => "marchen letter nya",
            Marchen::LetterTa => "marchen letter ta",
            Marchen::LetterTha => "marchen letter tha",
            Marchen::LetterDa => "marchen letter da",
            Marchen::LetterNa => "marchen letter na",
            Marchen::LetterPa => "marchen letter pa",
            Marchen::LetterPha => "marchen letter pha",
            Marchen::LetterBa => "marchen letter ba",
            Marchen::LetterMa => "marchen letter ma",
            Marchen::LetterTsa => "marchen letter tsa",
            Marchen::LetterTsha => "marchen letter tsha",
            Marchen::LetterDza => "marchen letter dza",
            Marchen::LetterWa => "marchen letter wa",
            Marchen::LetterZha => "marchen letter zha",
            Marchen::LetterZa => "marchen letter za",
            Marchen::LetterDashA => "marchen letter -a",
            Marchen::LetterYa => "marchen letter ya",
            Marchen::LetterRa => "marchen letter ra",
            Marchen::LetterLa => "marchen letter la",
            Marchen::LetterSha => "marchen letter sha",
            Marchen::LetterSa => "marchen letter sa",
            Marchen::LetterHa => "marchen letter ha",
            Marchen::LetterA => "marchen letter a",
            Marchen::SubjoinedLetterKa => "marchen subjoined letter ka",
            Marchen::SubjoinedLetterKha => "marchen subjoined letter kha",
            Marchen::SubjoinedLetterGa => "marchen subjoined letter ga",
            Marchen::SubjoinedLetterNga => "marchen subjoined letter nga",
            Marchen::SubjoinedLetterCa => "marchen subjoined letter ca",
            Marchen::SubjoinedLetterCha => "marchen subjoined letter cha",
            Marchen::SubjoinedLetterJa => "marchen subjoined letter ja",
            Marchen::SubjoinedLetterNya => "marchen subjoined letter nya",
            Marchen::SubjoinedLetterTa => "marchen subjoined letter ta",
            Marchen::SubjoinedLetterTha => "marchen subjoined letter tha",
            Marchen::SubjoinedLetterDa => "marchen subjoined letter da",
            Marchen::SubjoinedLetterNa => "marchen subjoined letter na",
            Marchen::SubjoinedLetterPa => "marchen subjoined letter pa",
            Marchen::SubjoinedLetterPha => "marchen subjoined letter pha",
            Marchen::SubjoinedLetterBa => "marchen subjoined letter ba",
            Marchen::SubjoinedLetterMa => "marchen subjoined letter ma",
            Marchen::SubjoinedLetterTsa => "marchen subjoined letter tsa",
            Marchen::SubjoinedLetterTsha => "marchen subjoined letter tsha",
            Marchen::SubjoinedLetterDza => "marchen subjoined letter dza",
            Marchen::SubjoinedLetterWa => "marchen subjoined letter wa",
            Marchen::SubjoinedLetterZha => "marchen subjoined letter zha",
            Marchen::SubjoinedLetterZa => "marchen subjoined letter za",
            Marchen::SubjoinedLetterYa => "marchen subjoined letter ya",
            Marchen::SubjoinedLetterRa => "marchen subjoined letter ra",
            Marchen::SubjoinedLetterLa => "marchen subjoined letter la",
            Marchen::SubjoinedLetterSha => "marchen subjoined letter sha",
            Marchen::SubjoinedLetterSa => "marchen subjoined letter sa",
            Marchen::SubjoinedLetterHa => "marchen subjoined letter ha",
            Marchen::SubjoinedLetterA => "marchen subjoined letter a",
            Marchen::VowelSignAa => "marchen vowel sign aa",
            Marchen::VowelSignI => "marchen vowel sign i",
            Marchen::VowelSignU => "marchen vowel sign u",
            Marchen::VowelSignE => "marchen vowel sign e",
            Marchen::VowelSignO => "marchen vowel sign o",
            Marchen::SignAnusvara => "marchen sign anusvara",
            Marchen::SignCandrabindu => "marchen sign candrabindu",
        }
    }
}
