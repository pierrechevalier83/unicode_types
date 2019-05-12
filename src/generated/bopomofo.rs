
/// An enum to represent all characters in the Bopomofo block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Bopomofo {
    /// \u{3105}: 'ㄅ'
    LetterB,
    /// \u{3106}: 'ㄆ'
    LetterP,
    /// \u{3107}: 'ㄇ'
    LetterM,
    /// \u{3108}: 'ㄈ'
    LetterF,
    /// \u{3109}: 'ㄉ'
    LetterD,
    /// \u{310a}: 'ㄊ'
    LetterT,
    /// \u{310b}: 'ㄋ'
    LetterN,
    /// \u{310c}: 'ㄌ'
    LetterL,
    /// \u{310d}: 'ㄍ'
    LetterG,
    /// \u{310e}: 'ㄎ'
    LetterK,
    /// \u{310f}: 'ㄏ'
    LetterH,
    /// \u{3110}: 'ㄐ'
    LetterJ,
    /// \u{3111}: 'ㄑ'
    LetterQ,
    /// \u{3112}: 'ㄒ'
    LetterX,
    /// \u{3113}: 'ㄓ'
    LetterZh,
    /// \u{3114}: 'ㄔ'
    LetterCh,
    /// \u{3115}: 'ㄕ'
    LetterSh,
    /// \u{3116}: 'ㄖ'
    LetterR,
    /// \u{3117}: 'ㄗ'
    LetterZ,
    /// \u{3118}: 'ㄘ'
    LetterC,
    /// \u{3119}: 'ㄙ'
    LetterS,
    /// \u{311a}: 'ㄚ'
    LetterA,
    /// \u{311b}: 'ㄛ'
    LetterO,
    /// \u{311c}: 'ㄜ'
    LetterE,
    /// \u{311d}: 'ㄝ'
    LetterEh,
    /// \u{311e}: 'ㄞ'
    LetterAi,
    /// \u{311f}: 'ㄟ'
    LetterEi,
    /// \u{3120}: 'ㄠ'
    LetterAu,
    /// \u{3121}: 'ㄡ'
    LetterOu,
    /// \u{3122}: 'ㄢ'
    LetterAn,
    /// \u{3123}: 'ㄣ'
    LetterEn,
    /// \u{3124}: 'ㄤ'
    LetterAng,
    /// \u{3125}: 'ㄥ'
    LetterEng,
    /// \u{3126}: 'ㄦ'
    LetterEr,
    /// \u{3127}: 'ㄧ'
    LetterI,
    /// \u{3128}: 'ㄨ'
    LetterU,
    /// \u{3129}: 'ㄩ'
    LetterIu,
    /// \u{312a}: 'ㄪ'
    LetterV,
    /// \u{312b}: 'ㄫ'
    LetterNg,
    /// \u{312c}: 'ㄬ'
    LetterGn,
    /// \u{312d}: 'ㄭ'
    LetterIh,
    /// \u{312e}: 'ㄮ'
    LetterOWithDotAbove,
}

impl Into<char> for Bopomofo {
    fn into(self) -> char {
        match self {
            Bopomofo::LetterB => 'ㄅ',
            Bopomofo::LetterP => 'ㄆ',
            Bopomofo::LetterM => 'ㄇ',
            Bopomofo::LetterF => 'ㄈ',
            Bopomofo::LetterD => 'ㄉ',
            Bopomofo::LetterT => 'ㄊ',
            Bopomofo::LetterN => 'ㄋ',
            Bopomofo::LetterL => 'ㄌ',
            Bopomofo::LetterG => 'ㄍ',
            Bopomofo::LetterK => 'ㄎ',
            Bopomofo::LetterH => 'ㄏ',
            Bopomofo::LetterJ => 'ㄐ',
            Bopomofo::LetterQ => 'ㄑ',
            Bopomofo::LetterX => 'ㄒ',
            Bopomofo::LetterZh => 'ㄓ',
            Bopomofo::LetterCh => 'ㄔ',
            Bopomofo::LetterSh => 'ㄕ',
            Bopomofo::LetterR => 'ㄖ',
            Bopomofo::LetterZ => 'ㄗ',
            Bopomofo::LetterC => 'ㄘ',
            Bopomofo::LetterS => 'ㄙ',
            Bopomofo::LetterA => 'ㄚ',
            Bopomofo::LetterO => 'ㄛ',
            Bopomofo::LetterE => 'ㄜ',
            Bopomofo::LetterEh => 'ㄝ',
            Bopomofo::LetterAi => 'ㄞ',
            Bopomofo::LetterEi => 'ㄟ',
            Bopomofo::LetterAu => 'ㄠ',
            Bopomofo::LetterOu => 'ㄡ',
            Bopomofo::LetterAn => 'ㄢ',
            Bopomofo::LetterEn => 'ㄣ',
            Bopomofo::LetterAng => 'ㄤ',
            Bopomofo::LetterEng => 'ㄥ',
            Bopomofo::LetterEr => 'ㄦ',
            Bopomofo::LetterI => 'ㄧ',
            Bopomofo::LetterU => 'ㄨ',
            Bopomofo::LetterIu => 'ㄩ',
            Bopomofo::LetterV => 'ㄪ',
            Bopomofo::LetterNg => 'ㄫ',
            Bopomofo::LetterGn => 'ㄬ',
            Bopomofo::LetterIh => 'ㄭ',
            Bopomofo::LetterOWithDotAbove => 'ㄮ',
        }
    }
}

impl std::convert::TryFrom<char> for Bopomofo {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ㄅ' => Ok(Bopomofo::LetterB),
            'ㄆ' => Ok(Bopomofo::LetterP),
            'ㄇ' => Ok(Bopomofo::LetterM),
            'ㄈ' => Ok(Bopomofo::LetterF),
            'ㄉ' => Ok(Bopomofo::LetterD),
            'ㄊ' => Ok(Bopomofo::LetterT),
            'ㄋ' => Ok(Bopomofo::LetterN),
            'ㄌ' => Ok(Bopomofo::LetterL),
            'ㄍ' => Ok(Bopomofo::LetterG),
            'ㄎ' => Ok(Bopomofo::LetterK),
            'ㄏ' => Ok(Bopomofo::LetterH),
            'ㄐ' => Ok(Bopomofo::LetterJ),
            'ㄑ' => Ok(Bopomofo::LetterQ),
            'ㄒ' => Ok(Bopomofo::LetterX),
            'ㄓ' => Ok(Bopomofo::LetterZh),
            'ㄔ' => Ok(Bopomofo::LetterCh),
            'ㄕ' => Ok(Bopomofo::LetterSh),
            'ㄖ' => Ok(Bopomofo::LetterR),
            'ㄗ' => Ok(Bopomofo::LetterZ),
            'ㄘ' => Ok(Bopomofo::LetterC),
            'ㄙ' => Ok(Bopomofo::LetterS),
            'ㄚ' => Ok(Bopomofo::LetterA),
            'ㄛ' => Ok(Bopomofo::LetterO),
            'ㄜ' => Ok(Bopomofo::LetterE),
            'ㄝ' => Ok(Bopomofo::LetterEh),
            'ㄞ' => Ok(Bopomofo::LetterAi),
            'ㄟ' => Ok(Bopomofo::LetterEi),
            'ㄠ' => Ok(Bopomofo::LetterAu),
            'ㄡ' => Ok(Bopomofo::LetterOu),
            'ㄢ' => Ok(Bopomofo::LetterAn),
            'ㄣ' => Ok(Bopomofo::LetterEn),
            'ㄤ' => Ok(Bopomofo::LetterAng),
            'ㄥ' => Ok(Bopomofo::LetterEng),
            'ㄦ' => Ok(Bopomofo::LetterEr),
            'ㄧ' => Ok(Bopomofo::LetterI),
            'ㄨ' => Ok(Bopomofo::LetterU),
            'ㄩ' => Ok(Bopomofo::LetterIu),
            'ㄪ' => Ok(Bopomofo::LetterV),
            'ㄫ' => Ok(Bopomofo::LetterNg),
            'ㄬ' => Ok(Bopomofo::LetterGn),
            'ㄭ' => Ok(Bopomofo::LetterIh),
            'ㄮ' => Ok(Bopomofo::LetterOWithDotAbove),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Bopomofo {
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

impl std::convert::TryFrom<u32> for Bopomofo {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Bopomofo {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Bopomofo {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Bopomofo::LetterB
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Bopomofo{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
