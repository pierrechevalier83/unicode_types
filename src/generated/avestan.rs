
/// An enum to represent all characters in the Avestan block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Avestan {
    /// \u{10b00}: '𐬀'
    LetterA,
    /// \u{10b01}: '𐬁'
    LetterAa,
    /// \u{10b02}: '𐬂'
    LetterAo,
    /// \u{10b03}: '𐬃'
    LetterAao,
    /// \u{10b04}: '𐬄'
    LetterAn,
    /// \u{10b05}: '𐬅'
    LetterAan,
    /// \u{10b06}: '𐬆'
    LetterAe,
    /// \u{10b07}: '𐬇'
    LetterAee,
    /// \u{10b08}: '𐬈'
    LetterE,
    /// \u{10b09}: '𐬉'
    LetterEe,
    /// \u{10b0a}: '𐬊'
    LetterO,
    /// \u{10b0b}: '𐬋'
    LetterOo,
    /// \u{10b0c}: '𐬌'
    LetterI,
    /// \u{10b0d}: '𐬍'
    LetterIi,
    /// \u{10b0e}: '𐬎'
    LetterU,
    /// \u{10b0f}: '𐬏'
    LetterUu,
    /// \u{10b10}: '𐬐'
    LetterKe,
    /// \u{10b11}: '𐬑'
    LetterXe,
    /// \u{10b12}: '𐬒'
    LetterXye,
    /// \u{10b13}: '𐬓'
    LetterXve,
    /// \u{10b14}: '𐬔'
    LetterGe,
    /// \u{10b15}: '𐬕'
    LetterGge,
    /// \u{10b16}: '𐬖'
    LetterGhe,
    /// \u{10b17}: '𐬗'
    LetterCe,
    /// \u{10b18}: '𐬘'
    LetterJe,
    /// \u{10b19}: '𐬙'
    LetterTe,
    /// \u{10b1a}: '𐬚'
    LetterThe,
    /// \u{10b1b}: '𐬛'
    LetterDe,
    /// \u{10b1c}: '𐬜'
    LetterDhe,
    /// \u{10b1d}: '𐬝'
    LetterTte,
    /// \u{10b1e}: '𐬞'
    LetterPe,
    /// \u{10b1f}: '𐬟'
    LetterFe,
    /// \u{10b20}: '𐬠'
    LetterBe,
    /// \u{10b21}: '𐬡'
    LetterBhe,
    /// \u{10b22}: '𐬢'
    LetterNge,
    /// \u{10b23}: '𐬣'
    LetterNgye,
    /// \u{10b24}: '𐬤'
    LetterNgve,
    /// \u{10b25}: '𐬥'
    LetterNe,
    /// \u{10b26}: '𐬦'
    LetterNye,
    /// \u{10b27}: '𐬧'
    LetterNne,
    /// \u{10b28}: '𐬨'
    LetterMe,
    /// \u{10b29}: '𐬩'
    LetterHme,
    /// \u{10b2a}: '𐬪'
    LetterYye,
    /// \u{10b2b}: '𐬫'
    LetterYe,
    /// \u{10b2c}: '𐬬'
    LetterVe,
    /// \u{10b2d}: '𐬭'
    LetterRe,
    /// \u{10b2e}: '𐬮'
    LetterLe,
    /// \u{10b2f}: '𐬯'
    LetterSe,
    /// \u{10b30}: '𐬰'
    LetterZe,
    /// \u{10b31}: '𐬱'
    LetterShe,
    /// \u{10b32}: '𐬲'
    LetterZhe,
    /// \u{10b33}: '𐬳'
    LetterShye,
    /// \u{10b34}: '𐬴'
    LetterSshe,
    /// \u{10b35}: '𐬵'
    LetterHe,
    /// \u{10b39}: '𐬹'
    AbbreviationMark,
    /// \u{10b3a}: '𐬺'
    TinyTwoDotsOverOneDotPunctuation,
    /// \u{10b3b}: '𐬻'
    SmallTwoDotsOverOneDotPunctuation,
    /// \u{10b3c}: '𐬼'
    LargeTwoDotsOverOneDotPunctuation,
    /// \u{10b3d}: '𐬽'
    LargeOneDotOverTwoDotsPunctuation,
    /// \u{10b3e}: '𐬾'
    LargeTwoRingsOverOneRingPunctuation,
}

impl Into<char> for Avestan {
    fn into(self) -> char {
        match self {
            Avestan::LetterA => '𐬀',
            Avestan::LetterAa => '𐬁',
            Avestan::LetterAo => '𐬂',
            Avestan::LetterAao => '𐬃',
            Avestan::LetterAn => '𐬄',
            Avestan::LetterAan => '𐬅',
            Avestan::LetterAe => '𐬆',
            Avestan::LetterAee => '𐬇',
            Avestan::LetterE => '𐬈',
            Avestan::LetterEe => '𐬉',
            Avestan::LetterO => '𐬊',
            Avestan::LetterOo => '𐬋',
            Avestan::LetterI => '𐬌',
            Avestan::LetterIi => '𐬍',
            Avestan::LetterU => '𐬎',
            Avestan::LetterUu => '𐬏',
            Avestan::LetterKe => '𐬐',
            Avestan::LetterXe => '𐬑',
            Avestan::LetterXye => '𐬒',
            Avestan::LetterXve => '𐬓',
            Avestan::LetterGe => '𐬔',
            Avestan::LetterGge => '𐬕',
            Avestan::LetterGhe => '𐬖',
            Avestan::LetterCe => '𐬗',
            Avestan::LetterJe => '𐬘',
            Avestan::LetterTe => '𐬙',
            Avestan::LetterThe => '𐬚',
            Avestan::LetterDe => '𐬛',
            Avestan::LetterDhe => '𐬜',
            Avestan::LetterTte => '𐬝',
            Avestan::LetterPe => '𐬞',
            Avestan::LetterFe => '𐬟',
            Avestan::LetterBe => '𐬠',
            Avestan::LetterBhe => '𐬡',
            Avestan::LetterNge => '𐬢',
            Avestan::LetterNgye => '𐬣',
            Avestan::LetterNgve => '𐬤',
            Avestan::LetterNe => '𐬥',
            Avestan::LetterNye => '𐬦',
            Avestan::LetterNne => '𐬧',
            Avestan::LetterMe => '𐬨',
            Avestan::LetterHme => '𐬩',
            Avestan::LetterYye => '𐬪',
            Avestan::LetterYe => '𐬫',
            Avestan::LetterVe => '𐬬',
            Avestan::LetterRe => '𐬭',
            Avestan::LetterLe => '𐬮',
            Avestan::LetterSe => '𐬯',
            Avestan::LetterZe => '𐬰',
            Avestan::LetterShe => '𐬱',
            Avestan::LetterZhe => '𐬲',
            Avestan::LetterShye => '𐬳',
            Avestan::LetterSshe => '𐬴',
            Avestan::LetterHe => '𐬵',
            Avestan::AbbreviationMark => '𐬹',
            Avestan::TinyTwoDotsOverOneDotPunctuation => '𐬺',
            Avestan::SmallTwoDotsOverOneDotPunctuation => '𐬻',
            Avestan::LargeTwoDotsOverOneDotPunctuation => '𐬼',
            Avestan::LargeOneDotOverTwoDotsPunctuation => '𐬽',
            Avestan::LargeTwoRingsOverOneRingPunctuation => '𐬾',
        }
    }
}

impl std::convert::TryFrom<char> for Avestan {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𐬀' => Ok(Avestan::LetterA),
            '𐬁' => Ok(Avestan::LetterAa),
            '𐬂' => Ok(Avestan::LetterAo),
            '𐬃' => Ok(Avestan::LetterAao),
            '𐬄' => Ok(Avestan::LetterAn),
            '𐬅' => Ok(Avestan::LetterAan),
            '𐬆' => Ok(Avestan::LetterAe),
            '𐬇' => Ok(Avestan::LetterAee),
            '𐬈' => Ok(Avestan::LetterE),
            '𐬉' => Ok(Avestan::LetterEe),
            '𐬊' => Ok(Avestan::LetterO),
            '𐬋' => Ok(Avestan::LetterOo),
            '𐬌' => Ok(Avestan::LetterI),
            '𐬍' => Ok(Avestan::LetterIi),
            '𐬎' => Ok(Avestan::LetterU),
            '𐬏' => Ok(Avestan::LetterUu),
            '𐬐' => Ok(Avestan::LetterKe),
            '𐬑' => Ok(Avestan::LetterXe),
            '𐬒' => Ok(Avestan::LetterXye),
            '𐬓' => Ok(Avestan::LetterXve),
            '𐬔' => Ok(Avestan::LetterGe),
            '𐬕' => Ok(Avestan::LetterGge),
            '𐬖' => Ok(Avestan::LetterGhe),
            '𐬗' => Ok(Avestan::LetterCe),
            '𐬘' => Ok(Avestan::LetterJe),
            '𐬙' => Ok(Avestan::LetterTe),
            '𐬚' => Ok(Avestan::LetterThe),
            '𐬛' => Ok(Avestan::LetterDe),
            '𐬜' => Ok(Avestan::LetterDhe),
            '𐬝' => Ok(Avestan::LetterTte),
            '𐬞' => Ok(Avestan::LetterPe),
            '𐬟' => Ok(Avestan::LetterFe),
            '𐬠' => Ok(Avestan::LetterBe),
            '𐬡' => Ok(Avestan::LetterBhe),
            '𐬢' => Ok(Avestan::LetterNge),
            '𐬣' => Ok(Avestan::LetterNgye),
            '𐬤' => Ok(Avestan::LetterNgve),
            '𐬥' => Ok(Avestan::LetterNe),
            '𐬦' => Ok(Avestan::LetterNye),
            '𐬧' => Ok(Avestan::LetterNne),
            '𐬨' => Ok(Avestan::LetterMe),
            '𐬩' => Ok(Avestan::LetterHme),
            '𐬪' => Ok(Avestan::LetterYye),
            '𐬫' => Ok(Avestan::LetterYe),
            '𐬬' => Ok(Avestan::LetterVe),
            '𐬭' => Ok(Avestan::LetterRe),
            '𐬮' => Ok(Avestan::LetterLe),
            '𐬯' => Ok(Avestan::LetterSe),
            '𐬰' => Ok(Avestan::LetterZe),
            '𐬱' => Ok(Avestan::LetterShe),
            '𐬲' => Ok(Avestan::LetterZhe),
            '𐬳' => Ok(Avestan::LetterShye),
            '𐬴' => Ok(Avestan::LetterSshe),
            '𐬵' => Ok(Avestan::LetterHe),
            '𐬹' => Ok(Avestan::AbbreviationMark),
            '𐬺' => Ok(Avestan::TinyTwoDotsOverOneDotPunctuation),
            '𐬻' => Ok(Avestan::SmallTwoDotsOverOneDotPunctuation),
            '𐬼' => Ok(Avestan::LargeTwoDotsOverOneDotPunctuation),
            '𐬽' => Ok(Avestan::LargeOneDotOverTwoDotsPunctuation),
            '𐬾' => Ok(Avestan::LargeTwoRingsOverOneRingPunctuation),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Avestan {
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

impl std::convert::TryFrom<u32> for Avestan {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Avestan {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Avestan {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Avestan::LetterA
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Avestan{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
