
/// An enum to represent all characters in the Medefaidrin block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Medefaidrin {
    /// \u{16e40}: '𖹀'
    CapitalLetterM,
    /// \u{16e41}: '𖹁'
    CapitalLetterS,
    /// \u{16e42}: '𖹂'
    CapitalLetterV,
    /// \u{16e43}: '𖹃'
    CapitalLetterW,
    /// \u{16e44}: '𖹄'
    CapitalLetterAtiu,
    /// \u{16e45}: '𖹅'
    CapitalLetterZ,
    /// \u{16e46}: '𖹆'
    CapitalLetterKp,
    /// \u{16e47}: '𖹇'
    CapitalLetterP,
    /// \u{16e48}: '𖹈'
    CapitalLetterT,
    /// \u{16e49}: '𖹉'
    CapitalLetterG,
    /// \u{16e4a}: '𖹊'
    CapitalLetterF,
    /// \u{16e4b}: '𖹋'
    CapitalLetterI,
    /// \u{16e4c}: '𖹌'
    CapitalLetterK,
    /// \u{16e4d}: '𖹍'
    CapitalLetterA,
    /// \u{16e4e}: '𖹎'
    CapitalLetterJ,
    /// \u{16e4f}: '𖹏'
    CapitalLetterE,
    /// \u{16e50}: '𖹐'
    CapitalLetterB,
    /// \u{16e51}: '𖹑'
    CapitalLetterC,
    /// \u{16e52}: '𖹒'
    CapitalLetterU,
    /// \u{16e53}: '𖹓'
    CapitalLetterYu,
    /// \u{16e54}: '𖹔'
    CapitalLetterL,
    /// \u{16e55}: '𖹕'
    CapitalLetterQ,
    /// \u{16e56}: '𖹖'
    CapitalLetterHp,
    /// \u{16e57}: '𖹗'
    CapitalLetterNy,
    /// \u{16e58}: '𖹘'
    CapitalLetterX,
    /// \u{16e59}: '𖹙'
    CapitalLetterD,
    /// \u{16e5a}: '𖹚'
    CapitalLetterOe,
    /// \u{16e5b}: '𖹛'
    CapitalLetterN,
    /// \u{16e5c}: '𖹜'
    CapitalLetterR,
    /// \u{16e5d}: '𖹝'
    CapitalLetterO,
    /// \u{16e5e}: '𖹞'
    CapitalLetterAi,
    /// \u{16e5f}: '𖹟'
    CapitalLetterY,
    /// \u{16e60}: '𖹠'
    SmallLetterM,
    /// \u{16e61}: '𖹡'
    SmallLetterS,
    /// \u{16e62}: '𖹢'
    SmallLetterV,
    /// \u{16e63}: '𖹣'
    SmallLetterW,
    /// \u{16e64}: '𖹤'
    SmallLetterAtiu,
    /// \u{16e65}: '𖹥'
    SmallLetterZ,
    /// \u{16e66}: '𖹦'
    SmallLetterKp,
    /// \u{16e67}: '𖹧'
    SmallLetterP,
    /// \u{16e68}: '𖹨'
    SmallLetterT,
    /// \u{16e69}: '𖹩'
    SmallLetterG,
    /// \u{16e6a}: '𖹪'
    SmallLetterF,
    /// \u{16e6b}: '𖹫'
    SmallLetterI,
    /// \u{16e6c}: '𖹬'
    SmallLetterK,
    /// \u{16e6d}: '𖹭'
    SmallLetterA,
    /// \u{16e6e}: '𖹮'
    SmallLetterJ,
    /// \u{16e6f}: '𖹯'
    SmallLetterE,
    /// \u{16e70}: '𖹰'
    SmallLetterB,
    /// \u{16e71}: '𖹱'
    SmallLetterC,
    /// \u{16e72}: '𖹲'
    SmallLetterU,
    /// \u{16e73}: '𖹳'
    SmallLetterYu,
    /// \u{16e74}: '𖹴'
    SmallLetterL,
    /// \u{16e75}: '𖹵'
    SmallLetterQ,
    /// \u{16e76}: '𖹶'
    SmallLetterHp,
    /// \u{16e77}: '𖹷'
    SmallLetterNy,
    /// \u{16e78}: '𖹸'
    SmallLetterX,
    /// \u{16e79}: '𖹹'
    SmallLetterD,
    /// \u{16e7a}: '𖹺'
    SmallLetterOe,
    /// \u{16e7b}: '𖹻'
    SmallLetterN,
    /// \u{16e7c}: '𖹼'
    SmallLetterR,
    /// \u{16e7d}: '𖹽'
    SmallLetterO,
    /// \u{16e7e}: '𖹾'
    SmallLetterAi,
    /// \u{16e7f}: '𖹿'
    SmallLetterY,
    /// \u{16e80}: '𖺀'
    DigitZero,
    /// \u{16e81}: '𖺁'
    DigitOne,
    /// \u{16e82}: '𖺂'
    DigitTwo,
    /// \u{16e83}: '𖺃'
    DigitThree,
    /// \u{16e84}: '𖺄'
    DigitFour,
    /// \u{16e85}: '𖺅'
    DigitFive,
    /// \u{16e86}: '𖺆'
    DigitSix,
    /// \u{16e87}: '𖺇'
    DigitSeven,
    /// \u{16e88}: '𖺈'
    DigitEight,
    /// \u{16e89}: '𖺉'
    DigitNine,
    /// \u{16e8a}: '𖺊'
    NumberTen,
    /// \u{16e8b}: '𖺋'
    NumberEleven,
    /// \u{16e8c}: '𖺌'
    NumberTwelve,
    /// \u{16e8d}: '𖺍'
    NumberThirteen,
    /// \u{16e8e}: '𖺎'
    NumberFourteen,
    /// \u{16e8f}: '𖺏'
    NumberFifteen,
    /// \u{16e90}: '𖺐'
    NumberSixteen,
    /// \u{16e91}: '𖺑'
    NumberSeventeen,
    /// \u{16e92}: '𖺒'
    NumberEighteen,
    /// \u{16e93}: '𖺓'
    NumberNineteen,
    /// \u{16e94}: '𖺔'
    DigitOneAlternateForm,
    /// \u{16e95}: '𖺕'
    DigitTwoAlternateForm,
    /// \u{16e96}: '𖺖'
    DigitThreeAlternateForm,
    /// \u{16e97}: '𖺗'
    Comma,
    /// \u{16e98}: '𖺘'
    FullStop,
    /// \u{16e99}: '𖺙'
    SymbolAiva,
    /// \u{16e9a}: '𖺚'
    ExclamationOh,
}

impl Into<char> for Medefaidrin {
    fn into(self) -> char {
        match self {
            Medefaidrin::CapitalLetterM => '𖹀',
            Medefaidrin::CapitalLetterS => '𖹁',
            Medefaidrin::CapitalLetterV => '𖹂',
            Medefaidrin::CapitalLetterW => '𖹃',
            Medefaidrin::CapitalLetterAtiu => '𖹄',
            Medefaidrin::CapitalLetterZ => '𖹅',
            Medefaidrin::CapitalLetterKp => '𖹆',
            Medefaidrin::CapitalLetterP => '𖹇',
            Medefaidrin::CapitalLetterT => '𖹈',
            Medefaidrin::CapitalLetterG => '𖹉',
            Medefaidrin::CapitalLetterF => '𖹊',
            Medefaidrin::CapitalLetterI => '𖹋',
            Medefaidrin::CapitalLetterK => '𖹌',
            Medefaidrin::CapitalLetterA => '𖹍',
            Medefaidrin::CapitalLetterJ => '𖹎',
            Medefaidrin::CapitalLetterE => '𖹏',
            Medefaidrin::CapitalLetterB => '𖹐',
            Medefaidrin::CapitalLetterC => '𖹑',
            Medefaidrin::CapitalLetterU => '𖹒',
            Medefaidrin::CapitalLetterYu => '𖹓',
            Medefaidrin::CapitalLetterL => '𖹔',
            Medefaidrin::CapitalLetterQ => '𖹕',
            Medefaidrin::CapitalLetterHp => '𖹖',
            Medefaidrin::CapitalLetterNy => '𖹗',
            Medefaidrin::CapitalLetterX => '𖹘',
            Medefaidrin::CapitalLetterD => '𖹙',
            Medefaidrin::CapitalLetterOe => '𖹚',
            Medefaidrin::CapitalLetterN => '𖹛',
            Medefaidrin::CapitalLetterR => '𖹜',
            Medefaidrin::CapitalLetterO => '𖹝',
            Medefaidrin::CapitalLetterAi => '𖹞',
            Medefaidrin::CapitalLetterY => '𖹟',
            Medefaidrin::SmallLetterM => '𖹠',
            Medefaidrin::SmallLetterS => '𖹡',
            Medefaidrin::SmallLetterV => '𖹢',
            Medefaidrin::SmallLetterW => '𖹣',
            Medefaidrin::SmallLetterAtiu => '𖹤',
            Medefaidrin::SmallLetterZ => '𖹥',
            Medefaidrin::SmallLetterKp => '𖹦',
            Medefaidrin::SmallLetterP => '𖹧',
            Medefaidrin::SmallLetterT => '𖹨',
            Medefaidrin::SmallLetterG => '𖹩',
            Medefaidrin::SmallLetterF => '𖹪',
            Medefaidrin::SmallLetterI => '𖹫',
            Medefaidrin::SmallLetterK => '𖹬',
            Medefaidrin::SmallLetterA => '𖹭',
            Medefaidrin::SmallLetterJ => '𖹮',
            Medefaidrin::SmallLetterE => '𖹯',
            Medefaidrin::SmallLetterB => '𖹰',
            Medefaidrin::SmallLetterC => '𖹱',
            Medefaidrin::SmallLetterU => '𖹲',
            Medefaidrin::SmallLetterYu => '𖹳',
            Medefaidrin::SmallLetterL => '𖹴',
            Medefaidrin::SmallLetterQ => '𖹵',
            Medefaidrin::SmallLetterHp => '𖹶',
            Medefaidrin::SmallLetterNy => '𖹷',
            Medefaidrin::SmallLetterX => '𖹸',
            Medefaidrin::SmallLetterD => '𖹹',
            Medefaidrin::SmallLetterOe => '𖹺',
            Medefaidrin::SmallLetterN => '𖹻',
            Medefaidrin::SmallLetterR => '𖹼',
            Medefaidrin::SmallLetterO => '𖹽',
            Medefaidrin::SmallLetterAi => '𖹾',
            Medefaidrin::SmallLetterY => '𖹿',
            Medefaidrin::DigitZero => '𖺀',
            Medefaidrin::DigitOne => '𖺁',
            Medefaidrin::DigitTwo => '𖺂',
            Medefaidrin::DigitThree => '𖺃',
            Medefaidrin::DigitFour => '𖺄',
            Medefaidrin::DigitFive => '𖺅',
            Medefaidrin::DigitSix => '𖺆',
            Medefaidrin::DigitSeven => '𖺇',
            Medefaidrin::DigitEight => '𖺈',
            Medefaidrin::DigitNine => '𖺉',
            Medefaidrin::NumberTen => '𖺊',
            Medefaidrin::NumberEleven => '𖺋',
            Medefaidrin::NumberTwelve => '𖺌',
            Medefaidrin::NumberThirteen => '𖺍',
            Medefaidrin::NumberFourteen => '𖺎',
            Medefaidrin::NumberFifteen => '𖺏',
            Medefaidrin::NumberSixteen => '𖺐',
            Medefaidrin::NumberSeventeen => '𖺑',
            Medefaidrin::NumberEighteen => '𖺒',
            Medefaidrin::NumberNineteen => '𖺓',
            Medefaidrin::DigitOneAlternateForm => '𖺔',
            Medefaidrin::DigitTwoAlternateForm => '𖺕',
            Medefaidrin::DigitThreeAlternateForm => '𖺖',
            Medefaidrin::Comma => '𖺗',
            Medefaidrin::FullStop => '𖺘',
            Medefaidrin::SymbolAiva => '𖺙',
            Medefaidrin::ExclamationOh => '𖺚',
        }
    }
}

impl std::convert::TryFrom<char> for Medefaidrin {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𖹀' => Ok(Medefaidrin::CapitalLetterM),
            '𖹁' => Ok(Medefaidrin::CapitalLetterS),
            '𖹂' => Ok(Medefaidrin::CapitalLetterV),
            '𖹃' => Ok(Medefaidrin::CapitalLetterW),
            '𖹄' => Ok(Medefaidrin::CapitalLetterAtiu),
            '𖹅' => Ok(Medefaidrin::CapitalLetterZ),
            '𖹆' => Ok(Medefaidrin::CapitalLetterKp),
            '𖹇' => Ok(Medefaidrin::CapitalLetterP),
            '𖹈' => Ok(Medefaidrin::CapitalLetterT),
            '𖹉' => Ok(Medefaidrin::CapitalLetterG),
            '𖹊' => Ok(Medefaidrin::CapitalLetterF),
            '𖹋' => Ok(Medefaidrin::CapitalLetterI),
            '𖹌' => Ok(Medefaidrin::CapitalLetterK),
            '𖹍' => Ok(Medefaidrin::CapitalLetterA),
            '𖹎' => Ok(Medefaidrin::CapitalLetterJ),
            '𖹏' => Ok(Medefaidrin::CapitalLetterE),
            '𖹐' => Ok(Medefaidrin::CapitalLetterB),
            '𖹑' => Ok(Medefaidrin::CapitalLetterC),
            '𖹒' => Ok(Medefaidrin::CapitalLetterU),
            '𖹓' => Ok(Medefaidrin::CapitalLetterYu),
            '𖹔' => Ok(Medefaidrin::CapitalLetterL),
            '𖹕' => Ok(Medefaidrin::CapitalLetterQ),
            '𖹖' => Ok(Medefaidrin::CapitalLetterHp),
            '𖹗' => Ok(Medefaidrin::CapitalLetterNy),
            '𖹘' => Ok(Medefaidrin::CapitalLetterX),
            '𖹙' => Ok(Medefaidrin::CapitalLetterD),
            '𖹚' => Ok(Medefaidrin::CapitalLetterOe),
            '𖹛' => Ok(Medefaidrin::CapitalLetterN),
            '𖹜' => Ok(Medefaidrin::CapitalLetterR),
            '𖹝' => Ok(Medefaidrin::CapitalLetterO),
            '𖹞' => Ok(Medefaidrin::CapitalLetterAi),
            '𖹟' => Ok(Medefaidrin::CapitalLetterY),
            '𖹠' => Ok(Medefaidrin::SmallLetterM),
            '𖹡' => Ok(Medefaidrin::SmallLetterS),
            '𖹢' => Ok(Medefaidrin::SmallLetterV),
            '𖹣' => Ok(Medefaidrin::SmallLetterW),
            '𖹤' => Ok(Medefaidrin::SmallLetterAtiu),
            '𖹥' => Ok(Medefaidrin::SmallLetterZ),
            '𖹦' => Ok(Medefaidrin::SmallLetterKp),
            '𖹧' => Ok(Medefaidrin::SmallLetterP),
            '𖹨' => Ok(Medefaidrin::SmallLetterT),
            '𖹩' => Ok(Medefaidrin::SmallLetterG),
            '𖹪' => Ok(Medefaidrin::SmallLetterF),
            '𖹫' => Ok(Medefaidrin::SmallLetterI),
            '𖹬' => Ok(Medefaidrin::SmallLetterK),
            '𖹭' => Ok(Medefaidrin::SmallLetterA),
            '𖹮' => Ok(Medefaidrin::SmallLetterJ),
            '𖹯' => Ok(Medefaidrin::SmallLetterE),
            '𖹰' => Ok(Medefaidrin::SmallLetterB),
            '𖹱' => Ok(Medefaidrin::SmallLetterC),
            '𖹲' => Ok(Medefaidrin::SmallLetterU),
            '𖹳' => Ok(Medefaidrin::SmallLetterYu),
            '𖹴' => Ok(Medefaidrin::SmallLetterL),
            '𖹵' => Ok(Medefaidrin::SmallLetterQ),
            '𖹶' => Ok(Medefaidrin::SmallLetterHp),
            '𖹷' => Ok(Medefaidrin::SmallLetterNy),
            '𖹸' => Ok(Medefaidrin::SmallLetterX),
            '𖹹' => Ok(Medefaidrin::SmallLetterD),
            '𖹺' => Ok(Medefaidrin::SmallLetterOe),
            '𖹻' => Ok(Medefaidrin::SmallLetterN),
            '𖹼' => Ok(Medefaidrin::SmallLetterR),
            '𖹽' => Ok(Medefaidrin::SmallLetterO),
            '𖹾' => Ok(Medefaidrin::SmallLetterAi),
            '𖹿' => Ok(Medefaidrin::SmallLetterY),
            '𖺀' => Ok(Medefaidrin::DigitZero),
            '𖺁' => Ok(Medefaidrin::DigitOne),
            '𖺂' => Ok(Medefaidrin::DigitTwo),
            '𖺃' => Ok(Medefaidrin::DigitThree),
            '𖺄' => Ok(Medefaidrin::DigitFour),
            '𖺅' => Ok(Medefaidrin::DigitFive),
            '𖺆' => Ok(Medefaidrin::DigitSix),
            '𖺇' => Ok(Medefaidrin::DigitSeven),
            '𖺈' => Ok(Medefaidrin::DigitEight),
            '𖺉' => Ok(Medefaidrin::DigitNine),
            '𖺊' => Ok(Medefaidrin::NumberTen),
            '𖺋' => Ok(Medefaidrin::NumberEleven),
            '𖺌' => Ok(Medefaidrin::NumberTwelve),
            '𖺍' => Ok(Medefaidrin::NumberThirteen),
            '𖺎' => Ok(Medefaidrin::NumberFourteen),
            '𖺏' => Ok(Medefaidrin::NumberFifteen),
            '𖺐' => Ok(Medefaidrin::NumberSixteen),
            '𖺑' => Ok(Medefaidrin::NumberSeventeen),
            '𖺒' => Ok(Medefaidrin::NumberEighteen),
            '𖺓' => Ok(Medefaidrin::NumberNineteen),
            '𖺔' => Ok(Medefaidrin::DigitOneAlternateForm),
            '𖺕' => Ok(Medefaidrin::DigitTwoAlternateForm),
            '𖺖' => Ok(Medefaidrin::DigitThreeAlternateForm),
            '𖺗' => Ok(Medefaidrin::Comma),
            '𖺘' => Ok(Medefaidrin::FullStop),
            '𖺙' => Ok(Medefaidrin::SymbolAiva),
            '𖺚' => Ok(Medefaidrin::ExclamationOh),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Medefaidrin {
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

impl std::convert::TryFrom<u32> for Medefaidrin {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Medefaidrin {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Medefaidrin {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Medefaidrin::CapitalLetterM
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Medefaidrin{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
