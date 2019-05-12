
/// An enum to represent all characters in the Elbasan block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Elbasan {
    /// \u{10500}: '𐔀'
    LetterA,
    /// \u{10501}: '𐔁'
    LetterBe,
    /// \u{10502}: '𐔂'
    LetterCe,
    /// \u{10503}: '𐔃'
    LetterChe,
    /// \u{10504}: '𐔄'
    LetterDe,
    /// \u{10505}: '𐔅'
    LetterNde,
    /// \u{10506}: '𐔆'
    LetterDhe,
    /// \u{10507}: '𐔇'
    LetterEi,
    /// \u{10508}: '𐔈'
    LetterE,
    /// \u{10509}: '𐔉'
    LetterFe,
    /// \u{1050a}: '𐔊'
    LetterGe,
    /// \u{1050b}: '𐔋'
    LetterGje,
    /// \u{1050c}: '𐔌'
    LetterHe,
    /// \u{1050d}: '𐔍'
    LetterI,
    /// \u{1050e}: '𐔎'
    LetterJe,
    /// \u{1050f}: '𐔏'
    LetterKe,
    /// \u{10510}: '𐔐'
    LetterLe,
    /// \u{10511}: '𐔑'
    LetterLle,
    /// \u{10512}: '𐔒'
    LetterMe,
    /// \u{10513}: '𐔓'
    LetterNe,
    /// \u{10514}: '𐔔'
    LetterNa,
    /// \u{10515}: '𐔕'
    LetterNje,
    /// \u{10516}: '𐔖'
    LetterO,
    /// \u{10517}: '𐔗'
    LetterPe,
    /// \u{10518}: '𐔘'
    LetterQe,
    /// \u{10519}: '𐔙'
    LetterRe,
    /// \u{1051a}: '𐔚'
    LetterRre,
    /// \u{1051b}: '𐔛'
    LetterSe,
    /// \u{1051c}: '𐔜'
    LetterShe,
    /// \u{1051d}: '𐔝'
    LetterTe,
    /// \u{1051e}: '𐔞'
    LetterThe,
    /// \u{1051f}: '𐔟'
    LetterU,
    /// \u{10520}: '𐔠'
    LetterVe,
    /// \u{10521}: '𐔡'
    LetterXe,
    /// \u{10522}: '𐔢'
    LetterY,
    /// \u{10523}: '𐔣'
    LetterZe,
    /// \u{10524}: '𐔤'
    LetterZhe,
    /// \u{10525}: '𐔥'
    LetterGhe,
    /// \u{10526}: '𐔦'
    LetterGhamma,
    /// \u{10527}: '𐔧'
    LetterKhe,
}

impl Into<char> for Elbasan {
    fn into(self) -> char {
        match self {
            Elbasan::LetterA => '𐔀',
            Elbasan::LetterBe => '𐔁',
            Elbasan::LetterCe => '𐔂',
            Elbasan::LetterChe => '𐔃',
            Elbasan::LetterDe => '𐔄',
            Elbasan::LetterNde => '𐔅',
            Elbasan::LetterDhe => '𐔆',
            Elbasan::LetterEi => '𐔇',
            Elbasan::LetterE => '𐔈',
            Elbasan::LetterFe => '𐔉',
            Elbasan::LetterGe => '𐔊',
            Elbasan::LetterGje => '𐔋',
            Elbasan::LetterHe => '𐔌',
            Elbasan::LetterI => '𐔍',
            Elbasan::LetterJe => '𐔎',
            Elbasan::LetterKe => '𐔏',
            Elbasan::LetterLe => '𐔐',
            Elbasan::LetterLle => '𐔑',
            Elbasan::LetterMe => '𐔒',
            Elbasan::LetterNe => '𐔓',
            Elbasan::LetterNa => '𐔔',
            Elbasan::LetterNje => '𐔕',
            Elbasan::LetterO => '𐔖',
            Elbasan::LetterPe => '𐔗',
            Elbasan::LetterQe => '𐔘',
            Elbasan::LetterRe => '𐔙',
            Elbasan::LetterRre => '𐔚',
            Elbasan::LetterSe => '𐔛',
            Elbasan::LetterShe => '𐔜',
            Elbasan::LetterTe => '𐔝',
            Elbasan::LetterThe => '𐔞',
            Elbasan::LetterU => '𐔟',
            Elbasan::LetterVe => '𐔠',
            Elbasan::LetterXe => '𐔡',
            Elbasan::LetterY => '𐔢',
            Elbasan::LetterZe => '𐔣',
            Elbasan::LetterZhe => '𐔤',
            Elbasan::LetterGhe => '𐔥',
            Elbasan::LetterGhamma => '𐔦',
            Elbasan::LetterKhe => '𐔧',
        }
    }
}

impl std::convert::TryFrom<char> for Elbasan {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𐔀' => Ok(Elbasan::LetterA),
            '𐔁' => Ok(Elbasan::LetterBe),
            '𐔂' => Ok(Elbasan::LetterCe),
            '𐔃' => Ok(Elbasan::LetterChe),
            '𐔄' => Ok(Elbasan::LetterDe),
            '𐔅' => Ok(Elbasan::LetterNde),
            '𐔆' => Ok(Elbasan::LetterDhe),
            '𐔇' => Ok(Elbasan::LetterEi),
            '𐔈' => Ok(Elbasan::LetterE),
            '𐔉' => Ok(Elbasan::LetterFe),
            '𐔊' => Ok(Elbasan::LetterGe),
            '𐔋' => Ok(Elbasan::LetterGje),
            '𐔌' => Ok(Elbasan::LetterHe),
            '𐔍' => Ok(Elbasan::LetterI),
            '𐔎' => Ok(Elbasan::LetterJe),
            '𐔏' => Ok(Elbasan::LetterKe),
            '𐔐' => Ok(Elbasan::LetterLe),
            '𐔑' => Ok(Elbasan::LetterLle),
            '𐔒' => Ok(Elbasan::LetterMe),
            '𐔓' => Ok(Elbasan::LetterNe),
            '𐔔' => Ok(Elbasan::LetterNa),
            '𐔕' => Ok(Elbasan::LetterNje),
            '𐔖' => Ok(Elbasan::LetterO),
            '𐔗' => Ok(Elbasan::LetterPe),
            '𐔘' => Ok(Elbasan::LetterQe),
            '𐔙' => Ok(Elbasan::LetterRe),
            '𐔚' => Ok(Elbasan::LetterRre),
            '𐔛' => Ok(Elbasan::LetterSe),
            '𐔜' => Ok(Elbasan::LetterShe),
            '𐔝' => Ok(Elbasan::LetterTe),
            '𐔞' => Ok(Elbasan::LetterThe),
            '𐔟' => Ok(Elbasan::LetterU),
            '𐔠' => Ok(Elbasan::LetterVe),
            '𐔡' => Ok(Elbasan::LetterXe),
            '𐔢' => Ok(Elbasan::LetterY),
            '𐔣' => Ok(Elbasan::LetterZe),
            '𐔤' => Ok(Elbasan::LetterZhe),
            '𐔥' => Ok(Elbasan::LetterGhe),
            '𐔦' => Ok(Elbasan::LetterGhamma),
            '𐔧' => Ok(Elbasan::LetterKhe),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Elbasan {
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

impl std::convert::TryFrom<u32> for Elbasan {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Elbasan {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Elbasan {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Elbasan::LetterA
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Elbasan{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
