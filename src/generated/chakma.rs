
/// An enum to represent all characters in the Chakma block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Chakma {
    /// \u{11100}: '𑄀'
    SignCandrabindu,
    /// \u{11101}: '𑄁'
    SignAnusvara,
    /// \u{11102}: '𑄂'
    SignVisarga,
    /// \u{11103}: '𑄃'
    LetterAa,
    /// \u{11104}: '𑄄'
    LetterI,
    /// \u{11105}: '𑄅'
    LetterU,
    /// \u{11106}: '𑄆'
    LetterE,
    /// \u{11107}: '𑄇'
    LetterKaa,
    /// \u{11108}: '𑄈'
    LetterKhaa,
    /// \u{11109}: '𑄉'
    LetterGaa,
    /// \u{1110a}: '𑄊'
    LetterGhaa,
    /// \u{1110b}: '𑄋'
    LetterNgaa,
    /// \u{1110c}: '𑄌'
    LetterCaa,
    /// \u{1110d}: '𑄍'
    LetterChaa,
    /// \u{1110e}: '𑄎'
    LetterJaa,
    /// \u{1110f}: '𑄏'
    LetterJhaa,
    /// \u{11110}: '𑄐'
    LetterNyaa,
    /// \u{11111}: '𑄑'
    LetterTtaa,
    /// \u{11112}: '𑄒'
    LetterTthaa,
    /// \u{11113}: '𑄓'
    LetterDdaa,
    /// \u{11114}: '𑄔'
    LetterDdhaa,
    /// \u{11115}: '𑄕'
    LetterNnaa,
    /// \u{11116}: '𑄖'
    LetterTaa,
    /// \u{11117}: '𑄗'
    LetterThaa,
    /// \u{11118}: '𑄘'
    LetterDaa,
    /// \u{11119}: '𑄙'
    LetterDhaa,
    /// \u{1111a}: '𑄚'
    LetterNaa,
    /// \u{1111b}: '𑄛'
    LetterPaa,
    /// \u{1111c}: '𑄜'
    LetterPhaa,
    /// \u{1111d}: '𑄝'
    LetterBaa,
    /// \u{1111e}: '𑄞'
    LetterBhaa,
    /// \u{1111f}: '𑄟'
    LetterMaa,
    /// \u{11120}: '𑄠'
    LetterYyaa,
    /// \u{11121}: '𑄡'
    LetterYaa,
    /// \u{11122}: '𑄢'
    LetterRaa,
    /// \u{11123}: '𑄣'
    LetterLaa,
    /// \u{11124}: '𑄤'
    LetterWaa,
    /// \u{11125}: '𑄥'
    LetterSaa,
    /// \u{11126}: '𑄦'
    LetterHaa,
    /// \u{11127}: '𑄧'
    VowelSignA,
    /// \u{11128}: '𑄨'
    VowelSignI,
    /// \u{11129}: '𑄩'
    VowelSignIi,
    /// \u{1112a}: '𑄪'
    VowelSignU,
    /// \u{1112b}: '𑄫'
    VowelSignUu,
    /// \u{1112c}: '𑄬'
    VowelSignE,
    /// \u{1112d}: '𑄭'
    VowelSignAi,
    /// \u{1112e}: '𑄮'
    VowelSignO,
    /// \u{1112f}: '𑄯'
    VowelSignAu,
    /// \u{11130}: '𑄰'
    VowelSignOi,
    /// \u{11131}: '𑄱'
    OMark,
    /// \u{11132}: '𑄲'
    AuMark,
    /// \u{11133}: '𑄳'
    Virama,
    /// \u{11134}: '𑄴'
    Maayyaa,
    /// \u{11136}: '𑄶'
    DigitZero,
    /// \u{11137}: '𑄷'
    DigitOne,
    /// \u{11138}: '𑄸'
    DigitTwo,
    /// \u{11139}: '𑄹'
    DigitThree,
    /// \u{1113a}: '𑄺'
    DigitFour,
    /// \u{1113b}: '𑄻'
    DigitFive,
    /// \u{1113c}: '𑄼'
    DigitSix,
    /// \u{1113d}: '𑄽'
    DigitSeven,
    /// \u{1113e}: '𑄾'
    DigitEight,
    /// \u{1113f}: '𑄿'
    DigitNine,
    /// \u{11140}: '𑅀'
    SectionMark,
    /// \u{11141}: '𑅁'
    Danda,
    /// \u{11142}: '𑅂'
    DoubleDanda,
    /// \u{11143}: '𑅃'
    QuestionMark,
    /// \u{11144}: '𑅄'
    LetterLhaa,
    /// \u{11145}: '𑅅'
    VowelSignAa,
    /// \u{11146}: '𑅆'
    VowelSignEi,
}

impl Into<char> for Chakma {
    fn into(self) -> char {
        match self {
            Chakma::SignCandrabindu => '𑄀',
            Chakma::SignAnusvara => '𑄁',
            Chakma::SignVisarga => '𑄂',
            Chakma::LetterAa => '𑄃',
            Chakma::LetterI => '𑄄',
            Chakma::LetterU => '𑄅',
            Chakma::LetterE => '𑄆',
            Chakma::LetterKaa => '𑄇',
            Chakma::LetterKhaa => '𑄈',
            Chakma::LetterGaa => '𑄉',
            Chakma::LetterGhaa => '𑄊',
            Chakma::LetterNgaa => '𑄋',
            Chakma::LetterCaa => '𑄌',
            Chakma::LetterChaa => '𑄍',
            Chakma::LetterJaa => '𑄎',
            Chakma::LetterJhaa => '𑄏',
            Chakma::LetterNyaa => '𑄐',
            Chakma::LetterTtaa => '𑄑',
            Chakma::LetterTthaa => '𑄒',
            Chakma::LetterDdaa => '𑄓',
            Chakma::LetterDdhaa => '𑄔',
            Chakma::LetterNnaa => '𑄕',
            Chakma::LetterTaa => '𑄖',
            Chakma::LetterThaa => '𑄗',
            Chakma::LetterDaa => '𑄘',
            Chakma::LetterDhaa => '𑄙',
            Chakma::LetterNaa => '𑄚',
            Chakma::LetterPaa => '𑄛',
            Chakma::LetterPhaa => '𑄜',
            Chakma::LetterBaa => '𑄝',
            Chakma::LetterBhaa => '𑄞',
            Chakma::LetterMaa => '𑄟',
            Chakma::LetterYyaa => '𑄠',
            Chakma::LetterYaa => '𑄡',
            Chakma::LetterRaa => '𑄢',
            Chakma::LetterLaa => '𑄣',
            Chakma::LetterWaa => '𑄤',
            Chakma::LetterSaa => '𑄥',
            Chakma::LetterHaa => '𑄦',
            Chakma::VowelSignA => '𑄧',
            Chakma::VowelSignI => '𑄨',
            Chakma::VowelSignIi => '𑄩',
            Chakma::VowelSignU => '𑄪',
            Chakma::VowelSignUu => '𑄫',
            Chakma::VowelSignE => '𑄬',
            Chakma::VowelSignAi => '𑄭',
            Chakma::VowelSignO => '𑄮',
            Chakma::VowelSignAu => '𑄯',
            Chakma::VowelSignOi => '𑄰',
            Chakma::OMark => '𑄱',
            Chakma::AuMark => '𑄲',
            Chakma::Virama => '𑄳',
            Chakma::Maayyaa => '𑄴',
            Chakma::DigitZero => '𑄶',
            Chakma::DigitOne => '𑄷',
            Chakma::DigitTwo => '𑄸',
            Chakma::DigitThree => '𑄹',
            Chakma::DigitFour => '𑄺',
            Chakma::DigitFive => '𑄻',
            Chakma::DigitSix => '𑄼',
            Chakma::DigitSeven => '𑄽',
            Chakma::DigitEight => '𑄾',
            Chakma::DigitNine => '𑄿',
            Chakma::SectionMark => '𑅀',
            Chakma::Danda => '𑅁',
            Chakma::DoubleDanda => '𑅂',
            Chakma::QuestionMark => '𑅃',
            Chakma::LetterLhaa => '𑅄',
            Chakma::VowelSignAa => '𑅅',
            Chakma::VowelSignEi => '𑅆',
        }
    }
}

impl std::convert::TryFrom<char> for Chakma {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𑄀' => Ok(Chakma::SignCandrabindu),
            '𑄁' => Ok(Chakma::SignAnusvara),
            '𑄂' => Ok(Chakma::SignVisarga),
            '𑄃' => Ok(Chakma::LetterAa),
            '𑄄' => Ok(Chakma::LetterI),
            '𑄅' => Ok(Chakma::LetterU),
            '𑄆' => Ok(Chakma::LetterE),
            '𑄇' => Ok(Chakma::LetterKaa),
            '𑄈' => Ok(Chakma::LetterKhaa),
            '𑄉' => Ok(Chakma::LetterGaa),
            '𑄊' => Ok(Chakma::LetterGhaa),
            '𑄋' => Ok(Chakma::LetterNgaa),
            '𑄌' => Ok(Chakma::LetterCaa),
            '𑄍' => Ok(Chakma::LetterChaa),
            '𑄎' => Ok(Chakma::LetterJaa),
            '𑄏' => Ok(Chakma::LetterJhaa),
            '𑄐' => Ok(Chakma::LetterNyaa),
            '𑄑' => Ok(Chakma::LetterTtaa),
            '𑄒' => Ok(Chakma::LetterTthaa),
            '𑄓' => Ok(Chakma::LetterDdaa),
            '𑄔' => Ok(Chakma::LetterDdhaa),
            '𑄕' => Ok(Chakma::LetterNnaa),
            '𑄖' => Ok(Chakma::LetterTaa),
            '𑄗' => Ok(Chakma::LetterThaa),
            '𑄘' => Ok(Chakma::LetterDaa),
            '𑄙' => Ok(Chakma::LetterDhaa),
            '𑄚' => Ok(Chakma::LetterNaa),
            '𑄛' => Ok(Chakma::LetterPaa),
            '𑄜' => Ok(Chakma::LetterPhaa),
            '𑄝' => Ok(Chakma::LetterBaa),
            '𑄞' => Ok(Chakma::LetterBhaa),
            '𑄟' => Ok(Chakma::LetterMaa),
            '𑄠' => Ok(Chakma::LetterYyaa),
            '𑄡' => Ok(Chakma::LetterYaa),
            '𑄢' => Ok(Chakma::LetterRaa),
            '𑄣' => Ok(Chakma::LetterLaa),
            '𑄤' => Ok(Chakma::LetterWaa),
            '𑄥' => Ok(Chakma::LetterSaa),
            '𑄦' => Ok(Chakma::LetterHaa),
            '𑄧' => Ok(Chakma::VowelSignA),
            '𑄨' => Ok(Chakma::VowelSignI),
            '𑄩' => Ok(Chakma::VowelSignIi),
            '𑄪' => Ok(Chakma::VowelSignU),
            '𑄫' => Ok(Chakma::VowelSignUu),
            '𑄬' => Ok(Chakma::VowelSignE),
            '𑄭' => Ok(Chakma::VowelSignAi),
            '𑄮' => Ok(Chakma::VowelSignO),
            '𑄯' => Ok(Chakma::VowelSignAu),
            '𑄰' => Ok(Chakma::VowelSignOi),
            '𑄱' => Ok(Chakma::OMark),
            '𑄲' => Ok(Chakma::AuMark),
            '𑄳' => Ok(Chakma::Virama),
            '𑄴' => Ok(Chakma::Maayyaa),
            '𑄶' => Ok(Chakma::DigitZero),
            '𑄷' => Ok(Chakma::DigitOne),
            '𑄸' => Ok(Chakma::DigitTwo),
            '𑄹' => Ok(Chakma::DigitThree),
            '𑄺' => Ok(Chakma::DigitFour),
            '𑄻' => Ok(Chakma::DigitFive),
            '𑄼' => Ok(Chakma::DigitSix),
            '𑄽' => Ok(Chakma::DigitSeven),
            '𑄾' => Ok(Chakma::DigitEight),
            '𑄿' => Ok(Chakma::DigitNine),
            '𑅀' => Ok(Chakma::SectionMark),
            '𑅁' => Ok(Chakma::Danda),
            '𑅂' => Ok(Chakma::DoubleDanda),
            '𑅃' => Ok(Chakma::QuestionMark),
            '𑅄' => Ok(Chakma::LetterLhaa),
            '𑅅' => Ok(Chakma::VowelSignAa),
            '𑅆' => Ok(Chakma::VowelSignEi),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Chakma {
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

impl std::convert::TryFrom<u32> for Chakma {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Chakma {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Chakma {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Chakma::SignCandrabindu
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Chakma::SignCandrabindu => "chakma sign candrabindu",
            Chakma::SignAnusvara => "chakma sign anusvara",
            Chakma::SignVisarga => "chakma sign visarga",
            Chakma::LetterAa => "chakma letter aa",
            Chakma::LetterI => "chakma letter i",
            Chakma::LetterU => "chakma letter u",
            Chakma::LetterE => "chakma letter e",
            Chakma::LetterKaa => "chakma letter kaa",
            Chakma::LetterKhaa => "chakma letter khaa",
            Chakma::LetterGaa => "chakma letter gaa",
            Chakma::LetterGhaa => "chakma letter ghaa",
            Chakma::LetterNgaa => "chakma letter ngaa",
            Chakma::LetterCaa => "chakma letter caa",
            Chakma::LetterChaa => "chakma letter chaa",
            Chakma::LetterJaa => "chakma letter jaa",
            Chakma::LetterJhaa => "chakma letter jhaa",
            Chakma::LetterNyaa => "chakma letter nyaa",
            Chakma::LetterTtaa => "chakma letter ttaa",
            Chakma::LetterTthaa => "chakma letter tthaa",
            Chakma::LetterDdaa => "chakma letter ddaa",
            Chakma::LetterDdhaa => "chakma letter ddhaa",
            Chakma::LetterNnaa => "chakma letter nnaa",
            Chakma::LetterTaa => "chakma letter taa",
            Chakma::LetterThaa => "chakma letter thaa",
            Chakma::LetterDaa => "chakma letter daa",
            Chakma::LetterDhaa => "chakma letter dhaa",
            Chakma::LetterNaa => "chakma letter naa",
            Chakma::LetterPaa => "chakma letter paa",
            Chakma::LetterPhaa => "chakma letter phaa",
            Chakma::LetterBaa => "chakma letter baa",
            Chakma::LetterBhaa => "chakma letter bhaa",
            Chakma::LetterMaa => "chakma letter maa",
            Chakma::LetterYyaa => "chakma letter yyaa",
            Chakma::LetterYaa => "chakma letter yaa",
            Chakma::LetterRaa => "chakma letter raa",
            Chakma::LetterLaa => "chakma letter laa",
            Chakma::LetterWaa => "chakma letter waa",
            Chakma::LetterSaa => "chakma letter saa",
            Chakma::LetterHaa => "chakma letter haa",
            Chakma::VowelSignA => "chakma vowel sign a",
            Chakma::VowelSignI => "chakma vowel sign i",
            Chakma::VowelSignIi => "chakma vowel sign ii",
            Chakma::VowelSignU => "chakma vowel sign u",
            Chakma::VowelSignUu => "chakma vowel sign uu",
            Chakma::VowelSignE => "chakma vowel sign e",
            Chakma::VowelSignAi => "chakma vowel sign ai",
            Chakma::VowelSignO => "chakma vowel sign o",
            Chakma::VowelSignAu => "chakma vowel sign au",
            Chakma::VowelSignOi => "chakma vowel sign oi",
            Chakma::OMark => "chakma o mark",
            Chakma::AuMark => "chakma au mark",
            Chakma::Virama => "chakma virama",
            Chakma::Maayyaa => "chakma maayyaa",
            Chakma::DigitZero => "chakma digit zero",
            Chakma::DigitOne => "chakma digit one",
            Chakma::DigitTwo => "chakma digit two",
            Chakma::DigitThree => "chakma digit three",
            Chakma::DigitFour => "chakma digit four",
            Chakma::DigitFive => "chakma digit five",
            Chakma::DigitSix => "chakma digit six",
            Chakma::DigitSeven => "chakma digit seven",
            Chakma::DigitEight => "chakma digit eight",
            Chakma::DigitNine => "chakma digit nine",
            Chakma::SectionMark => "chakma section mark",
            Chakma::Danda => "chakma danda",
            Chakma::DoubleDanda => "chakma double danda",
            Chakma::QuestionMark => "chakma question mark",
            Chakma::LetterLhaa => "chakma letter lhaa",
            Chakma::VowelSignAa => "chakma vowel sign aa",
            Chakma::VowelSignEi => "chakma vowel sign ei",
        }
    }
}
