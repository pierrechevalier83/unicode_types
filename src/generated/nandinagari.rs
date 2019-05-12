
/// An enum to represent all characters in the Nandinagari block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Nandinagari {
    /// \u{119a0}: '𑦠'
    LetterA,
    /// \u{119a1}: '𑦡'
    LetterAa,
    /// \u{119a2}: '𑦢'
    LetterI,
    /// \u{119a3}: '𑦣'
    LetterIi,
    /// \u{119a4}: '𑦤'
    LetterU,
    /// \u{119a5}: '𑦥'
    LetterUu,
    /// \u{119a6}: '𑦦'
    LetterVocalicR,
    /// \u{119a7}: '𑦧'
    LetterVocalicRr,
    /// \u{119aa}: '𑦪'
    LetterE,
    /// \u{119ab}: '𑦫'
    LetterAi,
    /// \u{119ac}: '𑦬'
    LetterO,
    /// \u{119ad}: '𑦭'
    LetterAu,
    /// \u{119ae}: '𑦮'
    LetterKa,
    /// \u{119af}: '𑦯'
    LetterKha,
    /// \u{119b0}: '𑦰'
    LetterGa,
    /// \u{119b1}: '𑦱'
    LetterGha,
    /// \u{119b2}: '𑦲'
    LetterNga,
    /// \u{119b3}: '𑦳'
    LetterCa,
    /// \u{119b4}: '𑦴'
    LetterCha,
    /// \u{119b5}: '𑦵'
    LetterJa,
    /// \u{119b6}: '𑦶'
    LetterJha,
    /// \u{119b7}: '𑦷'
    LetterNya,
    /// \u{119b8}: '𑦸'
    LetterTta,
    /// \u{119b9}: '𑦹'
    LetterTtha,
    /// \u{119ba}: '𑦺'
    LetterDda,
    /// \u{119bb}: '𑦻'
    LetterDdha,
    /// \u{119bc}: '𑦼'
    LetterNna,
    /// \u{119bd}: '𑦽'
    LetterTa,
    /// \u{119be}: '𑦾'
    LetterTha,
    /// \u{119bf}: '𑦿'
    LetterDa,
    /// \u{119c0}: '𑧀'
    LetterDha,
    /// \u{119c1}: '𑧁'
    LetterNa,
    /// \u{119c2}: '𑧂'
    LetterPa,
    /// \u{119c3}: '𑧃'
    LetterPha,
    /// \u{119c4}: '𑧄'
    LetterBa,
    /// \u{119c5}: '𑧅'
    LetterBha,
    /// \u{119c6}: '𑧆'
    LetterMa,
    /// \u{119c7}: '𑧇'
    LetterYa,
    /// \u{119c8}: '𑧈'
    LetterRa,
    /// \u{119c9}: '𑧉'
    LetterLa,
    /// \u{119ca}: '𑧊'
    LetterVa,
    /// \u{119cb}: '𑧋'
    LetterSha,
    /// \u{119cc}: '𑧌'
    LetterSsa,
    /// \u{119cd}: '𑧍'
    LetterSa,
    /// \u{119ce}: '𑧎'
    LetterHa,
    /// \u{119cf}: '𑧏'
    LetterLla,
    /// \u{119d0}: '𑧐'
    LetterRra,
    /// \u{119d1}: '𑧑'
    VowelSignAa,
    /// \u{119d2}: '𑧒'
    VowelSignI,
    /// \u{119d3}: '𑧓'
    VowelSignIi,
    /// \u{119d4}: '𑧔'
    VowelSignU,
    /// \u{119d5}: '𑧕'
    VowelSignUu,
    /// \u{119d6}: '𑧖'
    VowelSignVocalicR,
    /// \u{119d7}: '𑧗'
    VowelSignVocalicRr,
    /// \u{119da}: '𑧚'
    VowelSignE,
    /// \u{119db}: '𑧛'
    VowelSignAi,
    /// \u{119dc}: '𑧜'
    VowelSignO,
    /// \u{119dd}: '𑧝'
    VowelSignAu,
    /// \u{119de}: '𑧞'
    SignAnusvara,
    /// \u{119df}: '𑧟'
    SignVisarga,
    /// \u{119e0}: '𑧠'
    SignVirama,
    /// \u{119e1}: '𑧡'
    SignAvagraha,
    /// \u{119e2}: '𑧢'
    SignSiddham,
    /// \u{119e3}: '𑧣'
    Headstroke,
    /// \u{119e4}: '𑧤'
    VowelSignPrishthamatraE,
}

impl Into<char> for Nandinagari {
    fn into(self) -> char {
        match self {
            Nandinagari::LetterA => '𑦠',
            Nandinagari::LetterAa => '𑦡',
            Nandinagari::LetterI => '𑦢',
            Nandinagari::LetterIi => '𑦣',
            Nandinagari::LetterU => '𑦤',
            Nandinagari::LetterUu => '𑦥',
            Nandinagari::LetterVocalicR => '𑦦',
            Nandinagari::LetterVocalicRr => '𑦧',
            Nandinagari::LetterE => '𑦪',
            Nandinagari::LetterAi => '𑦫',
            Nandinagari::LetterO => '𑦬',
            Nandinagari::LetterAu => '𑦭',
            Nandinagari::LetterKa => '𑦮',
            Nandinagari::LetterKha => '𑦯',
            Nandinagari::LetterGa => '𑦰',
            Nandinagari::LetterGha => '𑦱',
            Nandinagari::LetterNga => '𑦲',
            Nandinagari::LetterCa => '𑦳',
            Nandinagari::LetterCha => '𑦴',
            Nandinagari::LetterJa => '𑦵',
            Nandinagari::LetterJha => '𑦶',
            Nandinagari::LetterNya => '𑦷',
            Nandinagari::LetterTta => '𑦸',
            Nandinagari::LetterTtha => '𑦹',
            Nandinagari::LetterDda => '𑦺',
            Nandinagari::LetterDdha => '𑦻',
            Nandinagari::LetterNna => '𑦼',
            Nandinagari::LetterTa => '𑦽',
            Nandinagari::LetterTha => '𑦾',
            Nandinagari::LetterDa => '𑦿',
            Nandinagari::LetterDha => '𑧀',
            Nandinagari::LetterNa => '𑧁',
            Nandinagari::LetterPa => '𑧂',
            Nandinagari::LetterPha => '𑧃',
            Nandinagari::LetterBa => '𑧄',
            Nandinagari::LetterBha => '𑧅',
            Nandinagari::LetterMa => '𑧆',
            Nandinagari::LetterYa => '𑧇',
            Nandinagari::LetterRa => '𑧈',
            Nandinagari::LetterLa => '𑧉',
            Nandinagari::LetterVa => '𑧊',
            Nandinagari::LetterSha => '𑧋',
            Nandinagari::LetterSsa => '𑧌',
            Nandinagari::LetterSa => '𑧍',
            Nandinagari::LetterHa => '𑧎',
            Nandinagari::LetterLla => '𑧏',
            Nandinagari::LetterRra => '𑧐',
            Nandinagari::VowelSignAa => '𑧑',
            Nandinagari::VowelSignI => '𑧒',
            Nandinagari::VowelSignIi => '𑧓',
            Nandinagari::VowelSignU => '𑧔',
            Nandinagari::VowelSignUu => '𑧕',
            Nandinagari::VowelSignVocalicR => '𑧖',
            Nandinagari::VowelSignVocalicRr => '𑧗',
            Nandinagari::VowelSignE => '𑧚',
            Nandinagari::VowelSignAi => '𑧛',
            Nandinagari::VowelSignO => '𑧜',
            Nandinagari::VowelSignAu => '𑧝',
            Nandinagari::SignAnusvara => '𑧞',
            Nandinagari::SignVisarga => '𑧟',
            Nandinagari::SignVirama => '𑧠',
            Nandinagari::SignAvagraha => '𑧡',
            Nandinagari::SignSiddham => '𑧢',
            Nandinagari::Headstroke => '𑧣',
            Nandinagari::VowelSignPrishthamatraE => '𑧤',
        }
    }
}

impl std::convert::TryFrom<char> for Nandinagari {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𑦠' => Ok(Nandinagari::LetterA),
            '𑦡' => Ok(Nandinagari::LetterAa),
            '𑦢' => Ok(Nandinagari::LetterI),
            '𑦣' => Ok(Nandinagari::LetterIi),
            '𑦤' => Ok(Nandinagari::LetterU),
            '𑦥' => Ok(Nandinagari::LetterUu),
            '𑦦' => Ok(Nandinagari::LetterVocalicR),
            '𑦧' => Ok(Nandinagari::LetterVocalicRr),
            '𑦪' => Ok(Nandinagari::LetterE),
            '𑦫' => Ok(Nandinagari::LetterAi),
            '𑦬' => Ok(Nandinagari::LetterO),
            '𑦭' => Ok(Nandinagari::LetterAu),
            '𑦮' => Ok(Nandinagari::LetterKa),
            '𑦯' => Ok(Nandinagari::LetterKha),
            '𑦰' => Ok(Nandinagari::LetterGa),
            '𑦱' => Ok(Nandinagari::LetterGha),
            '𑦲' => Ok(Nandinagari::LetterNga),
            '𑦳' => Ok(Nandinagari::LetterCa),
            '𑦴' => Ok(Nandinagari::LetterCha),
            '𑦵' => Ok(Nandinagari::LetterJa),
            '𑦶' => Ok(Nandinagari::LetterJha),
            '𑦷' => Ok(Nandinagari::LetterNya),
            '𑦸' => Ok(Nandinagari::LetterTta),
            '𑦹' => Ok(Nandinagari::LetterTtha),
            '𑦺' => Ok(Nandinagari::LetterDda),
            '𑦻' => Ok(Nandinagari::LetterDdha),
            '𑦼' => Ok(Nandinagari::LetterNna),
            '𑦽' => Ok(Nandinagari::LetterTa),
            '𑦾' => Ok(Nandinagari::LetterTha),
            '𑦿' => Ok(Nandinagari::LetterDa),
            '𑧀' => Ok(Nandinagari::LetterDha),
            '𑧁' => Ok(Nandinagari::LetterNa),
            '𑧂' => Ok(Nandinagari::LetterPa),
            '𑧃' => Ok(Nandinagari::LetterPha),
            '𑧄' => Ok(Nandinagari::LetterBa),
            '𑧅' => Ok(Nandinagari::LetterBha),
            '𑧆' => Ok(Nandinagari::LetterMa),
            '𑧇' => Ok(Nandinagari::LetterYa),
            '𑧈' => Ok(Nandinagari::LetterRa),
            '𑧉' => Ok(Nandinagari::LetterLa),
            '𑧊' => Ok(Nandinagari::LetterVa),
            '𑧋' => Ok(Nandinagari::LetterSha),
            '𑧌' => Ok(Nandinagari::LetterSsa),
            '𑧍' => Ok(Nandinagari::LetterSa),
            '𑧎' => Ok(Nandinagari::LetterHa),
            '𑧏' => Ok(Nandinagari::LetterLla),
            '𑧐' => Ok(Nandinagari::LetterRra),
            '𑧑' => Ok(Nandinagari::VowelSignAa),
            '𑧒' => Ok(Nandinagari::VowelSignI),
            '𑧓' => Ok(Nandinagari::VowelSignIi),
            '𑧔' => Ok(Nandinagari::VowelSignU),
            '𑧕' => Ok(Nandinagari::VowelSignUu),
            '𑧖' => Ok(Nandinagari::VowelSignVocalicR),
            '𑧗' => Ok(Nandinagari::VowelSignVocalicRr),
            '𑧚' => Ok(Nandinagari::VowelSignE),
            '𑧛' => Ok(Nandinagari::VowelSignAi),
            '𑧜' => Ok(Nandinagari::VowelSignO),
            '𑧝' => Ok(Nandinagari::VowelSignAu),
            '𑧞' => Ok(Nandinagari::SignAnusvara),
            '𑧟' => Ok(Nandinagari::SignVisarga),
            '𑧠' => Ok(Nandinagari::SignVirama),
            '𑧡' => Ok(Nandinagari::SignAvagraha),
            '𑧢' => Ok(Nandinagari::SignSiddham),
            '𑧣' => Ok(Nandinagari::Headstroke),
            '𑧤' => Ok(Nandinagari::VowelSignPrishthamatraE),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Nandinagari {
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

impl std::convert::TryFrom<u32> for Nandinagari {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Nandinagari {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Nandinagari {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Nandinagari::LetterA
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Nandinagari::LetterA => "nandinagari letter a",
            Nandinagari::LetterAa => "nandinagari letter aa",
            Nandinagari::LetterI => "nandinagari letter i",
            Nandinagari::LetterIi => "nandinagari letter ii",
            Nandinagari::LetterU => "nandinagari letter u",
            Nandinagari::LetterUu => "nandinagari letter uu",
            Nandinagari::LetterVocalicR => "nandinagari letter vocalic r",
            Nandinagari::LetterVocalicRr => "nandinagari letter vocalic rr",
            Nandinagari::LetterE => "nandinagari letter e",
            Nandinagari::LetterAi => "nandinagari letter ai",
            Nandinagari::LetterO => "nandinagari letter o",
            Nandinagari::LetterAu => "nandinagari letter au",
            Nandinagari::LetterKa => "nandinagari letter ka",
            Nandinagari::LetterKha => "nandinagari letter kha",
            Nandinagari::LetterGa => "nandinagari letter ga",
            Nandinagari::LetterGha => "nandinagari letter gha",
            Nandinagari::LetterNga => "nandinagari letter nga",
            Nandinagari::LetterCa => "nandinagari letter ca",
            Nandinagari::LetterCha => "nandinagari letter cha",
            Nandinagari::LetterJa => "nandinagari letter ja",
            Nandinagari::LetterJha => "nandinagari letter jha",
            Nandinagari::LetterNya => "nandinagari letter nya",
            Nandinagari::LetterTta => "nandinagari letter tta",
            Nandinagari::LetterTtha => "nandinagari letter ttha",
            Nandinagari::LetterDda => "nandinagari letter dda",
            Nandinagari::LetterDdha => "nandinagari letter ddha",
            Nandinagari::LetterNna => "nandinagari letter nna",
            Nandinagari::LetterTa => "nandinagari letter ta",
            Nandinagari::LetterTha => "nandinagari letter tha",
            Nandinagari::LetterDa => "nandinagari letter da",
            Nandinagari::LetterDha => "nandinagari letter dha",
            Nandinagari::LetterNa => "nandinagari letter na",
            Nandinagari::LetterPa => "nandinagari letter pa",
            Nandinagari::LetterPha => "nandinagari letter pha",
            Nandinagari::LetterBa => "nandinagari letter ba",
            Nandinagari::LetterBha => "nandinagari letter bha",
            Nandinagari::LetterMa => "nandinagari letter ma",
            Nandinagari::LetterYa => "nandinagari letter ya",
            Nandinagari::LetterRa => "nandinagari letter ra",
            Nandinagari::LetterLa => "nandinagari letter la",
            Nandinagari::LetterVa => "nandinagari letter va",
            Nandinagari::LetterSha => "nandinagari letter sha",
            Nandinagari::LetterSsa => "nandinagari letter ssa",
            Nandinagari::LetterSa => "nandinagari letter sa",
            Nandinagari::LetterHa => "nandinagari letter ha",
            Nandinagari::LetterLla => "nandinagari letter lla",
            Nandinagari::LetterRra => "nandinagari letter rra",
            Nandinagari::VowelSignAa => "nandinagari vowel sign aa",
            Nandinagari::VowelSignI => "nandinagari vowel sign i",
            Nandinagari::VowelSignIi => "nandinagari vowel sign ii",
            Nandinagari::VowelSignU => "nandinagari vowel sign u",
            Nandinagari::VowelSignUu => "nandinagari vowel sign uu",
            Nandinagari::VowelSignVocalicR => "nandinagari vowel sign vocalic r",
            Nandinagari::VowelSignVocalicRr => "nandinagari vowel sign vocalic rr",
            Nandinagari::VowelSignE => "nandinagari vowel sign e",
            Nandinagari::VowelSignAi => "nandinagari vowel sign ai",
            Nandinagari::VowelSignO => "nandinagari vowel sign o",
            Nandinagari::VowelSignAu => "nandinagari vowel sign au",
            Nandinagari::SignAnusvara => "nandinagari sign anusvara",
            Nandinagari::SignVisarga => "nandinagari sign visarga",
            Nandinagari::SignVirama => "nandinagari sign virama",
            Nandinagari::SignAvagraha => "nandinagari sign avagraha",
            Nandinagari::SignSiddham => "nandinagari sign siddham",
            Nandinagari::Headstroke => "nandinagari headstroke",
            Nandinagari::VowelSignPrishthamatraE => "nandinagari vowel sign prishthamatra e",
        }
    }
}
