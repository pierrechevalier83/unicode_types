
/// An enum to represent all characters in the Tirhuta block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Tirhuta {
    /// \u{11480}: '𑒀'
    Anji,
    /// \u{11481}: '𑒁'
    LetterA,
    /// \u{11482}: '𑒂'
    LetterAa,
    /// \u{11483}: '𑒃'
    LetterI,
    /// \u{11484}: '𑒄'
    LetterIi,
    /// \u{11485}: '𑒅'
    LetterU,
    /// \u{11486}: '𑒆'
    LetterUu,
    /// \u{11487}: '𑒇'
    LetterVocalicR,
    /// \u{11488}: '𑒈'
    LetterVocalicRr,
    /// \u{11489}: '𑒉'
    LetterVocalicL,
    /// \u{1148a}: '𑒊'
    LetterVocalicLl,
    /// \u{1148b}: '𑒋'
    LetterE,
    /// \u{1148c}: '𑒌'
    LetterAi,
    /// \u{1148d}: '𑒍'
    LetterO,
    /// \u{1148e}: '𑒎'
    LetterAu,
    /// \u{1148f}: '𑒏'
    LetterKa,
    /// \u{11490}: '𑒐'
    LetterKha,
    /// \u{11491}: '𑒑'
    LetterGa,
    /// \u{11492}: '𑒒'
    LetterGha,
    /// \u{11493}: '𑒓'
    LetterNga,
    /// \u{11494}: '𑒔'
    LetterCa,
    /// \u{11495}: '𑒕'
    LetterCha,
    /// \u{11496}: '𑒖'
    LetterJa,
    /// \u{11497}: '𑒗'
    LetterJha,
    /// \u{11498}: '𑒘'
    LetterNya,
    /// \u{11499}: '𑒙'
    LetterTta,
    /// \u{1149a}: '𑒚'
    LetterTtha,
    /// \u{1149b}: '𑒛'
    LetterDda,
    /// \u{1149c}: '𑒜'
    LetterDdha,
    /// \u{1149d}: '𑒝'
    LetterNna,
    /// \u{1149e}: '𑒞'
    LetterTa,
    /// \u{1149f}: '𑒟'
    LetterTha,
    /// \u{114a0}: '𑒠'
    LetterDa,
    /// \u{114a1}: '𑒡'
    LetterDha,
    /// \u{114a2}: '𑒢'
    LetterNa,
    /// \u{114a3}: '𑒣'
    LetterPa,
    /// \u{114a4}: '𑒤'
    LetterPha,
    /// \u{114a5}: '𑒥'
    LetterBa,
    /// \u{114a6}: '𑒦'
    LetterBha,
    /// \u{114a7}: '𑒧'
    LetterMa,
    /// \u{114a8}: '𑒨'
    LetterYa,
    /// \u{114a9}: '𑒩'
    LetterRa,
    /// \u{114aa}: '𑒪'
    LetterLa,
    /// \u{114ab}: '𑒫'
    LetterVa,
    /// \u{114ac}: '𑒬'
    LetterSha,
    /// \u{114ad}: '𑒭'
    LetterSsa,
    /// \u{114ae}: '𑒮'
    LetterSa,
    /// \u{114af}: '𑒯'
    LetterHa,
    /// \u{114b0}: '𑒰'
    VowelSignAa,
    /// \u{114b1}: '𑒱'
    VowelSignI,
    /// \u{114b2}: '𑒲'
    VowelSignIi,
    /// \u{114b3}: '𑒳'
    VowelSignU,
    /// \u{114b4}: '𑒴'
    VowelSignUu,
    /// \u{114b5}: '𑒵'
    VowelSignVocalicR,
    /// \u{114b6}: '𑒶'
    VowelSignVocalicRr,
    /// \u{114b7}: '𑒷'
    VowelSignVocalicL,
    /// \u{114b8}: '𑒸'
    VowelSignVocalicLl,
    /// \u{114b9}: '𑒹'
    VowelSignE,
    /// \u{114ba}: '𑒺'
    VowelSignShortE,
    /// \u{114bb}: '𑒻'
    VowelSignAi,
    /// \u{114bc}: '𑒼'
    VowelSignO,
    /// \u{114bd}: '𑒽'
    VowelSignShortO,
    /// \u{114be}: '𑒾'
    VowelSignAu,
    /// \u{114bf}: '𑒿'
    SignCandrabindu,
    /// \u{114c0}: '𑓀'
    SignAnusvara,
    /// \u{114c1}: '𑓁'
    SignVisarga,
    /// \u{114c2}: '𑓂'
    SignVirama,
    /// \u{114c3}: '𑓃'
    SignNukta,
    /// \u{114c4}: '𑓄'
    SignAvagraha,
    /// \u{114c5}: '𑓅'
    Gvang,
    /// \u{114c6}: '𑓆'
    AbbreviationSign,
    /// \u{114c7}: '𑓇'
    Om,
    /// \u{114d0}: '𑓐'
    DigitZero,
    /// \u{114d1}: '𑓑'
    DigitOne,
    /// \u{114d2}: '𑓒'
    DigitTwo,
    /// \u{114d3}: '𑓓'
    DigitThree,
    /// \u{114d4}: '𑓔'
    DigitFour,
    /// \u{114d5}: '𑓕'
    DigitFive,
    /// \u{114d6}: '𑓖'
    DigitSix,
    /// \u{114d7}: '𑓗'
    DigitSeven,
    /// \u{114d8}: '𑓘'
    DigitEight,
    /// \u{114d9}: '𑓙'
    DigitNine,
}

impl Into<char> for Tirhuta {
    fn into(self) -> char {
        match self {
            Tirhuta::Anji => '𑒀',
            Tirhuta::LetterA => '𑒁',
            Tirhuta::LetterAa => '𑒂',
            Tirhuta::LetterI => '𑒃',
            Tirhuta::LetterIi => '𑒄',
            Tirhuta::LetterU => '𑒅',
            Tirhuta::LetterUu => '𑒆',
            Tirhuta::LetterVocalicR => '𑒇',
            Tirhuta::LetterVocalicRr => '𑒈',
            Tirhuta::LetterVocalicL => '𑒉',
            Tirhuta::LetterVocalicLl => '𑒊',
            Tirhuta::LetterE => '𑒋',
            Tirhuta::LetterAi => '𑒌',
            Tirhuta::LetterO => '𑒍',
            Tirhuta::LetterAu => '𑒎',
            Tirhuta::LetterKa => '𑒏',
            Tirhuta::LetterKha => '𑒐',
            Tirhuta::LetterGa => '𑒑',
            Tirhuta::LetterGha => '𑒒',
            Tirhuta::LetterNga => '𑒓',
            Tirhuta::LetterCa => '𑒔',
            Tirhuta::LetterCha => '𑒕',
            Tirhuta::LetterJa => '𑒖',
            Tirhuta::LetterJha => '𑒗',
            Tirhuta::LetterNya => '𑒘',
            Tirhuta::LetterTta => '𑒙',
            Tirhuta::LetterTtha => '𑒚',
            Tirhuta::LetterDda => '𑒛',
            Tirhuta::LetterDdha => '𑒜',
            Tirhuta::LetterNna => '𑒝',
            Tirhuta::LetterTa => '𑒞',
            Tirhuta::LetterTha => '𑒟',
            Tirhuta::LetterDa => '𑒠',
            Tirhuta::LetterDha => '𑒡',
            Tirhuta::LetterNa => '𑒢',
            Tirhuta::LetterPa => '𑒣',
            Tirhuta::LetterPha => '𑒤',
            Tirhuta::LetterBa => '𑒥',
            Tirhuta::LetterBha => '𑒦',
            Tirhuta::LetterMa => '𑒧',
            Tirhuta::LetterYa => '𑒨',
            Tirhuta::LetterRa => '𑒩',
            Tirhuta::LetterLa => '𑒪',
            Tirhuta::LetterVa => '𑒫',
            Tirhuta::LetterSha => '𑒬',
            Tirhuta::LetterSsa => '𑒭',
            Tirhuta::LetterSa => '𑒮',
            Tirhuta::LetterHa => '𑒯',
            Tirhuta::VowelSignAa => '𑒰',
            Tirhuta::VowelSignI => '𑒱',
            Tirhuta::VowelSignIi => '𑒲',
            Tirhuta::VowelSignU => '𑒳',
            Tirhuta::VowelSignUu => '𑒴',
            Tirhuta::VowelSignVocalicR => '𑒵',
            Tirhuta::VowelSignVocalicRr => '𑒶',
            Tirhuta::VowelSignVocalicL => '𑒷',
            Tirhuta::VowelSignVocalicLl => '𑒸',
            Tirhuta::VowelSignE => '𑒹',
            Tirhuta::VowelSignShortE => '𑒺',
            Tirhuta::VowelSignAi => '𑒻',
            Tirhuta::VowelSignO => '𑒼',
            Tirhuta::VowelSignShortO => '𑒽',
            Tirhuta::VowelSignAu => '𑒾',
            Tirhuta::SignCandrabindu => '𑒿',
            Tirhuta::SignAnusvara => '𑓀',
            Tirhuta::SignVisarga => '𑓁',
            Tirhuta::SignVirama => '𑓂',
            Tirhuta::SignNukta => '𑓃',
            Tirhuta::SignAvagraha => '𑓄',
            Tirhuta::Gvang => '𑓅',
            Tirhuta::AbbreviationSign => '𑓆',
            Tirhuta::Om => '𑓇',
            Tirhuta::DigitZero => '𑓐',
            Tirhuta::DigitOne => '𑓑',
            Tirhuta::DigitTwo => '𑓒',
            Tirhuta::DigitThree => '𑓓',
            Tirhuta::DigitFour => '𑓔',
            Tirhuta::DigitFive => '𑓕',
            Tirhuta::DigitSix => '𑓖',
            Tirhuta::DigitSeven => '𑓗',
            Tirhuta::DigitEight => '𑓘',
            Tirhuta::DigitNine => '𑓙',
        }
    }
}

impl std::convert::TryFrom<char> for Tirhuta {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𑒀' => Ok(Tirhuta::Anji),
            '𑒁' => Ok(Tirhuta::LetterA),
            '𑒂' => Ok(Tirhuta::LetterAa),
            '𑒃' => Ok(Tirhuta::LetterI),
            '𑒄' => Ok(Tirhuta::LetterIi),
            '𑒅' => Ok(Tirhuta::LetterU),
            '𑒆' => Ok(Tirhuta::LetterUu),
            '𑒇' => Ok(Tirhuta::LetterVocalicR),
            '𑒈' => Ok(Tirhuta::LetterVocalicRr),
            '𑒉' => Ok(Tirhuta::LetterVocalicL),
            '𑒊' => Ok(Tirhuta::LetterVocalicLl),
            '𑒋' => Ok(Tirhuta::LetterE),
            '𑒌' => Ok(Tirhuta::LetterAi),
            '𑒍' => Ok(Tirhuta::LetterO),
            '𑒎' => Ok(Tirhuta::LetterAu),
            '𑒏' => Ok(Tirhuta::LetterKa),
            '𑒐' => Ok(Tirhuta::LetterKha),
            '𑒑' => Ok(Tirhuta::LetterGa),
            '𑒒' => Ok(Tirhuta::LetterGha),
            '𑒓' => Ok(Tirhuta::LetterNga),
            '𑒔' => Ok(Tirhuta::LetterCa),
            '𑒕' => Ok(Tirhuta::LetterCha),
            '𑒖' => Ok(Tirhuta::LetterJa),
            '𑒗' => Ok(Tirhuta::LetterJha),
            '𑒘' => Ok(Tirhuta::LetterNya),
            '𑒙' => Ok(Tirhuta::LetterTta),
            '𑒚' => Ok(Tirhuta::LetterTtha),
            '𑒛' => Ok(Tirhuta::LetterDda),
            '𑒜' => Ok(Tirhuta::LetterDdha),
            '𑒝' => Ok(Tirhuta::LetterNna),
            '𑒞' => Ok(Tirhuta::LetterTa),
            '𑒟' => Ok(Tirhuta::LetterTha),
            '𑒠' => Ok(Tirhuta::LetterDa),
            '𑒡' => Ok(Tirhuta::LetterDha),
            '𑒢' => Ok(Tirhuta::LetterNa),
            '𑒣' => Ok(Tirhuta::LetterPa),
            '𑒤' => Ok(Tirhuta::LetterPha),
            '𑒥' => Ok(Tirhuta::LetterBa),
            '𑒦' => Ok(Tirhuta::LetterBha),
            '𑒧' => Ok(Tirhuta::LetterMa),
            '𑒨' => Ok(Tirhuta::LetterYa),
            '𑒩' => Ok(Tirhuta::LetterRa),
            '𑒪' => Ok(Tirhuta::LetterLa),
            '𑒫' => Ok(Tirhuta::LetterVa),
            '𑒬' => Ok(Tirhuta::LetterSha),
            '𑒭' => Ok(Tirhuta::LetterSsa),
            '𑒮' => Ok(Tirhuta::LetterSa),
            '𑒯' => Ok(Tirhuta::LetterHa),
            '𑒰' => Ok(Tirhuta::VowelSignAa),
            '𑒱' => Ok(Tirhuta::VowelSignI),
            '𑒲' => Ok(Tirhuta::VowelSignIi),
            '𑒳' => Ok(Tirhuta::VowelSignU),
            '𑒴' => Ok(Tirhuta::VowelSignUu),
            '𑒵' => Ok(Tirhuta::VowelSignVocalicR),
            '𑒶' => Ok(Tirhuta::VowelSignVocalicRr),
            '𑒷' => Ok(Tirhuta::VowelSignVocalicL),
            '𑒸' => Ok(Tirhuta::VowelSignVocalicLl),
            '𑒹' => Ok(Tirhuta::VowelSignE),
            '𑒺' => Ok(Tirhuta::VowelSignShortE),
            '𑒻' => Ok(Tirhuta::VowelSignAi),
            '𑒼' => Ok(Tirhuta::VowelSignO),
            '𑒽' => Ok(Tirhuta::VowelSignShortO),
            '𑒾' => Ok(Tirhuta::VowelSignAu),
            '𑒿' => Ok(Tirhuta::SignCandrabindu),
            '𑓀' => Ok(Tirhuta::SignAnusvara),
            '𑓁' => Ok(Tirhuta::SignVisarga),
            '𑓂' => Ok(Tirhuta::SignVirama),
            '𑓃' => Ok(Tirhuta::SignNukta),
            '𑓄' => Ok(Tirhuta::SignAvagraha),
            '𑓅' => Ok(Tirhuta::Gvang),
            '𑓆' => Ok(Tirhuta::AbbreviationSign),
            '𑓇' => Ok(Tirhuta::Om),
            '𑓐' => Ok(Tirhuta::DigitZero),
            '𑓑' => Ok(Tirhuta::DigitOne),
            '𑓒' => Ok(Tirhuta::DigitTwo),
            '𑓓' => Ok(Tirhuta::DigitThree),
            '𑓔' => Ok(Tirhuta::DigitFour),
            '𑓕' => Ok(Tirhuta::DigitFive),
            '𑓖' => Ok(Tirhuta::DigitSix),
            '𑓗' => Ok(Tirhuta::DigitSeven),
            '𑓘' => Ok(Tirhuta::DigitEight),
            '𑓙' => Ok(Tirhuta::DigitNine),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Tirhuta {
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

impl std::convert::TryFrom<u32> for Tirhuta {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Tirhuta {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Tirhuta {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Tirhuta::Anji
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Tirhuta::Anji => "tirhuta anji",
            Tirhuta::LetterA => "tirhuta letter a",
            Tirhuta::LetterAa => "tirhuta letter aa",
            Tirhuta::LetterI => "tirhuta letter i",
            Tirhuta::LetterIi => "tirhuta letter ii",
            Tirhuta::LetterU => "tirhuta letter u",
            Tirhuta::LetterUu => "tirhuta letter uu",
            Tirhuta::LetterVocalicR => "tirhuta letter vocalic r",
            Tirhuta::LetterVocalicRr => "tirhuta letter vocalic rr",
            Tirhuta::LetterVocalicL => "tirhuta letter vocalic l",
            Tirhuta::LetterVocalicLl => "tirhuta letter vocalic ll",
            Tirhuta::LetterE => "tirhuta letter e",
            Tirhuta::LetterAi => "tirhuta letter ai",
            Tirhuta::LetterO => "tirhuta letter o",
            Tirhuta::LetterAu => "tirhuta letter au",
            Tirhuta::LetterKa => "tirhuta letter ka",
            Tirhuta::LetterKha => "tirhuta letter kha",
            Tirhuta::LetterGa => "tirhuta letter ga",
            Tirhuta::LetterGha => "tirhuta letter gha",
            Tirhuta::LetterNga => "tirhuta letter nga",
            Tirhuta::LetterCa => "tirhuta letter ca",
            Tirhuta::LetterCha => "tirhuta letter cha",
            Tirhuta::LetterJa => "tirhuta letter ja",
            Tirhuta::LetterJha => "tirhuta letter jha",
            Tirhuta::LetterNya => "tirhuta letter nya",
            Tirhuta::LetterTta => "tirhuta letter tta",
            Tirhuta::LetterTtha => "tirhuta letter ttha",
            Tirhuta::LetterDda => "tirhuta letter dda",
            Tirhuta::LetterDdha => "tirhuta letter ddha",
            Tirhuta::LetterNna => "tirhuta letter nna",
            Tirhuta::LetterTa => "tirhuta letter ta",
            Tirhuta::LetterTha => "tirhuta letter tha",
            Tirhuta::LetterDa => "tirhuta letter da",
            Tirhuta::LetterDha => "tirhuta letter dha",
            Tirhuta::LetterNa => "tirhuta letter na",
            Tirhuta::LetterPa => "tirhuta letter pa",
            Tirhuta::LetterPha => "tirhuta letter pha",
            Tirhuta::LetterBa => "tirhuta letter ba",
            Tirhuta::LetterBha => "tirhuta letter bha",
            Tirhuta::LetterMa => "tirhuta letter ma",
            Tirhuta::LetterYa => "tirhuta letter ya",
            Tirhuta::LetterRa => "tirhuta letter ra",
            Tirhuta::LetterLa => "tirhuta letter la",
            Tirhuta::LetterVa => "tirhuta letter va",
            Tirhuta::LetterSha => "tirhuta letter sha",
            Tirhuta::LetterSsa => "tirhuta letter ssa",
            Tirhuta::LetterSa => "tirhuta letter sa",
            Tirhuta::LetterHa => "tirhuta letter ha",
            Tirhuta::VowelSignAa => "tirhuta vowel sign aa",
            Tirhuta::VowelSignI => "tirhuta vowel sign i",
            Tirhuta::VowelSignIi => "tirhuta vowel sign ii",
            Tirhuta::VowelSignU => "tirhuta vowel sign u",
            Tirhuta::VowelSignUu => "tirhuta vowel sign uu",
            Tirhuta::VowelSignVocalicR => "tirhuta vowel sign vocalic r",
            Tirhuta::VowelSignVocalicRr => "tirhuta vowel sign vocalic rr",
            Tirhuta::VowelSignVocalicL => "tirhuta vowel sign vocalic l",
            Tirhuta::VowelSignVocalicLl => "tirhuta vowel sign vocalic ll",
            Tirhuta::VowelSignE => "tirhuta vowel sign e",
            Tirhuta::VowelSignShortE => "tirhuta vowel sign short e",
            Tirhuta::VowelSignAi => "tirhuta vowel sign ai",
            Tirhuta::VowelSignO => "tirhuta vowel sign o",
            Tirhuta::VowelSignShortO => "tirhuta vowel sign short o",
            Tirhuta::VowelSignAu => "tirhuta vowel sign au",
            Tirhuta::SignCandrabindu => "tirhuta sign candrabindu",
            Tirhuta::SignAnusvara => "tirhuta sign anusvara",
            Tirhuta::SignVisarga => "tirhuta sign visarga",
            Tirhuta::SignVirama => "tirhuta sign virama",
            Tirhuta::SignNukta => "tirhuta sign nukta",
            Tirhuta::SignAvagraha => "tirhuta sign avagraha",
            Tirhuta::Gvang => "tirhuta gvang",
            Tirhuta::AbbreviationSign => "tirhuta abbreviation sign",
            Tirhuta::Om => "tirhuta om",
            Tirhuta::DigitZero => "tirhuta digit zero",
            Tirhuta::DigitOne => "tirhuta digit one",
            Tirhuta::DigitTwo => "tirhuta digit two",
            Tirhuta::DigitThree => "tirhuta digit three",
            Tirhuta::DigitFour => "tirhuta digit four",
            Tirhuta::DigitFive => "tirhuta digit five",
            Tirhuta::DigitSix => "tirhuta digit six",
            Tirhuta::DigitSeven => "tirhuta digit seven",
            Tirhuta::DigitEight => "tirhuta digit eight",
            Tirhuta::DigitNine => "tirhuta digit nine",
        }
    }
}
