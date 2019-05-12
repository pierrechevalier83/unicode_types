/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{3105}: 'ㄅ'
    pub const LETTER_B: char = 'ㄅ';
    /// \u{3106}: 'ㄆ'
    pub const LETTER_P: char = 'ㄆ';
    /// \u{3107}: 'ㄇ'
    pub const LETTER_M: char = 'ㄇ';
    /// \u{3108}: 'ㄈ'
    pub const LETTER_F: char = 'ㄈ';
    /// \u{3109}: 'ㄉ'
    pub const LETTER_D: char = 'ㄉ';
    /// \u{310a}: 'ㄊ'
    pub const LETTER_T: char = 'ㄊ';
    /// \u{310b}: 'ㄋ'
    pub const LETTER_N: char = 'ㄋ';
    /// \u{310c}: 'ㄌ'
    pub const LETTER_L: char = 'ㄌ';
    /// \u{310d}: 'ㄍ'
    pub const LETTER_G: char = 'ㄍ';
    /// \u{310e}: 'ㄎ'
    pub const LETTER_K: char = 'ㄎ';
    /// \u{310f}: 'ㄏ'
    pub const LETTER_H: char = 'ㄏ';
    /// \u{3110}: 'ㄐ'
    pub const LETTER_J: char = 'ㄐ';
    /// \u{3111}: 'ㄑ'
    pub const LETTER_Q: char = 'ㄑ';
    /// \u{3112}: 'ㄒ'
    pub const LETTER_X: char = 'ㄒ';
    /// \u{3113}: 'ㄓ'
    pub const LETTER_ZH: char = 'ㄓ';
    /// \u{3114}: 'ㄔ'
    pub const LETTER_CH: char = 'ㄔ';
    /// \u{3115}: 'ㄕ'
    pub const LETTER_SH: char = 'ㄕ';
    /// \u{3116}: 'ㄖ'
    pub const LETTER_R: char = 'ㄖ';
    /// \u{3117}: 'ㄗ'
    pub const LETTER_Z: char = 'ㄗ';
    /// \u{3118}: 'ㄘ'
    pub const LETTER_C: char = 'ㄘ';
    /// \u{3119}: 'ㄙ'
    pub const LETTER_S: char = 'ㄙ';
    /// \u{311a}: 'ㄚ'
    pub const LETTER_A: char = 'ㄚ';
    /// \u{311b}: 'ㄛ'
    pub const LETTER_O: char = 'ㄛ';
    /// \u{311c}: 'ㄜ'
    pub const LETTER_E: char = 'ㄜ';
    /// \u{311d}: 'ㄝ'
    pub const LETTER_EH: char = 'ㄝ';
    /// \u{311e}: 'ㄞ'
    pub const LETTER_AI: char = 'ㄞ';
    /// \u{311f}: 'ㄟ'
    pub const LETTER_EI: char = 'ㄟ';
    /// \u{3120}: 'ㄠ'
    pub const LETTER_AU: char = 'ㄠ';
    /// \u{3121}: 'ㄡ'
    pub const LETTER_OU: char = 'ㄡ';
    /// \u{3122}: 'ㄢ'
    pub const LETTER_AN: char = 'ㄢ';
    /// \u{3123}: 'ㄣ'
    pub const LETTER_EN: char = 'ㄣ';
    /// \u{3124}: 'ㄤ'
    pub const LETTER_ANG: char = 'ㄤ';
    /// \u{3125}: 'ㄥ'
    pub const LETTER_ENG: char = 'ㄥ';
    /// \u{3126}: 'ㄦ'
    pub const LETTER_ER: char = 'ㄦ';
    /// \u{3127}: 'ㄧ'
    pub const LETTER_I: char = 'ㄧ';
    /// \u{3128}: 'ㄨ'
    pub const LETTER_U: char = 'ㄨ';
    /// \u{3129}: 'ㄩ'
    pub const LETTER_IU: char = 'ㄩ';
    /// \u{312a}: 'ㄪ'
    pub const LETTER_V: char = 'ㄪ';
    /// \u{312b}: 'ㄫ'
    pub const LETTER_NG: char = 'ㄫ';
    /// \u{312c}: 'ㄬ'
    pub const LETTER_GN: char = 'ㄬ';
    /// \u{312d}: 'ㄭ'
    pub const LETTER_IH: char = 'ㄭ';
    /// \u{312e}: 'ㄮ'
    pub const LETTER_O_WITH_DOT_ABOVE: char = 'ㄮ';
}

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
        use constants::*;
        match self {
            Bopomofo::LetterB => LETTER_B,
            Bopomofo::LetterP => LETTER_P,
            Bopomofo::LetterM => LETTER_M,
            Bopomofo::LetterF => LETTER_F,
            Bopomofo::LetterD => LETTER_D,
            Bopomofo::LetterT => LETTER_T,
            Bopomofo::LetterN => LETTER_N,
            Bopomofo::LetterL => LETTER_L,
            Bopomofo::LetterG => LETTER_G,
            Bopomofo::LetterK => LETTER_K,
            Bopomofo::LetterH => LETTER_H,
            Bopomofo::LetterJ => LETTER_J,
            Bopomofo::LetterQ => LETTER_Q,
            Bopomofo::LetterX => LETTER_X,
            Bopomofo::LetterZh => LETTER_ZH,
            Bopomofo::LetterCh => LETTER_CH,
            Bopomofo::LetterSh => LETTER_SH,
            Bopomofo::LetterR => LETTER_R,
            Bopomofo::LetterZ => LETTER_Z,
            Bopomofo::LetterC => LETTER_C,
            Bopomofo::LetterS => LETTER_S,
            Bopomofo::LetterA => LETTER_A,
            Bopomofo::LetterO => LETTER_O,
            Bopomofo::LetterE => LETTER_E,
            Bopomofo::LetterEh => LETTER_EH,
            Bopomofo::LetterAi => LETTER_AI,
            Bopomofo::LetterEi => LETTER_EI,
            Bopomofo::LetterAu => LETTER_AU,
            Bopomofo::LetterOu => LETTER_OU,
            Bopomofo::LetterAn => LETTER_AN,
            Bopomofo::LetterEn => LETTER_EN,
            Bopomofo::LetterAng => LETTER_ANG,
            Bopomofo::LetterEng => LETTER_ENG,
            Bopomofo::LetterEr => LETTER_ER,
            Bopomofo::LetterI => LETTER_I,
            Bopomofo::LetterU => LETTER_U,
            Bopomofo::LetterIu => LETTER_IU,
            Bopomofo::LetterV => LETTER_V,
            Bopomofo::LetterNg => LETTER_NG,
            Bopomofo::LetterGn => LETTER_GN,
            Bopomofo::LetterIh => LETTER_IH,
            Bopomofo::LetterOWithDotAbove => LETTER_O_WITH_DOT_ABOVE,
        }
    }
}

impl std::convert::TryFrom<char> for Bopomofo {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_B => Ok(Bopomofo::LetterB),
            LETTER_P => Ok(Bopomofo::LetterP),
            LETTER_M => Ok(Bopomofo::LetterM),
            LETTER_F => Ok(Bopomofo::LetterF),
            LETTER_D => Ok(Bopomofo::LetterD),
            LETTER_T => Ok(Bopomofo::LetterT),
            LETTER_N => Ok(Bopomofo::LetterN),
            LETTER_L => Ok(Bopomofo::LetterL),
            LETTER_G => Ok(Bopomofo::LetterG),
            LETTER_K => Ok(Bopomofo::LetterK),
            LETTER_H => Ok(Bopomofo::LetterH),
            LETTER_J => Ok(Bopomofo::LetterJ),
            LETTER_Q => Ok(Bopomofo::LetterQ),
            LETTER_X => Ok(Bopomofo::LetterX),
            LETTER_ZH => Ok(Bopomofo::LetterZh),
            LETTER_CH => Ok(Bopomofo::LetterCh),
            LETTER_SH => Ok(Bopomofo::LetterSh),
            LETTER_R => Ok(Bopomofo::LetterR),
            LETTER_Z => Ok(Bopomofo::LetterZ),
            LETTER_C => Ok(Bopomofo::LetterC),
            LETTER_S => Ok(Bopomofo::LetterS),
            LETTER_A => Ok(Bopomofo::LetterA),
            LETTER_O => Ok(Bopomofo::LetterO),
            LETTER_E => Ok(Bopomofo::LetterE),
            LETTER_EH => Ok(Bopomofo::LetterEh),
            LETTER_AI => Ok(Bopomofo::LetterAi),
            LETTER_EI => Ok(Bopomofo::LetterEi),
            LETTER_AU => Ok(Bopomofo::LetterAu),
            LETTER_OU => Ok(Bopomofo::LetterOu),
            LETTER_AN => Ok(Bopomofo::LetterAn),
            LETTER_EN => Ok(Bopomofo::LetterEn),
            LETTER_ANG => Ok(Bopomofo::LetterAng),
            LETTER_ENG => Ok(Bopomofo::LetterEng),
            LETTER_ER => Ok(Bopomofo::LetterEr),
            LETTER_I => Ok(Bopomofo::LetterI),
            LETTER_U => Ok(Bopomofo::LetterU),
            LETTER_IU => Ok(Bopomofo::LetterIu),
            LETTER_V => Ok(Bopomofo::LetterV),
            LETTER_NG => Ok(Bopomofo::LetterNg),
            LETTER_GN => Ok(Bopomofo::LetterGn),
            LETTER_IH => Ok(Bopomofo::LetterIh),
            LETTER_O_WITH_DOT_ABOVE => Ok(Bopomofo::LetterOWithDotAbove),
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

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Bopomofo::LetterB => "bopomofo letter b",
            Bopomofo::LetterP => "bopomofo letter p",
            Bopomofo::LetterM => "bopomofo letter m",
            Bopomofo::LetterF => "bopomofo letter f",
            Bopomofo::LetterD => "bopomofo letter d",
            Bopomofo::LetterT => "bopomofo letter t",
            Bopomofo::LetterN => "bopomofo letter n",
            Bopomofo::LetterL => "bopomofo letter l",
            Bopomofo::LetterG => "bopomofo letter g",
            Bopomofo::LetterK => "bopomofo letter k",
            Bopomofo::LetterH => "bopomofo letter h",
            Bopomofo::LetterJ => "bopomofo letter j",
            Bopomofo::LetterQ => "bopomofo letter q",
            Bopomofo::LetterX => "bopomofo letter x",
            Bopomofo::LetterZh => "bopomofo letter zh",
            Bopomofo::LetterCh => "bopomofo letter ch",
            Bopomofo::LetterSh => "bopomofo letter sh",
            Bopomofo::LetterR => "bopomofo letter r",
            Bopomofo::LetterZ => "bopomofo letter z",
            Bopomofo::LetterC => "bopomofo letter c",
            Bopomofo::LetterS => "bopomofo letter s",
            Bopomofo::LetterA => "bopomofo letter a",
            Bopomofo::LetterO => "bopomofo letter o",
            Bopomofo::LetterE => "bopomofo letter e",
            Bopomofo::LetterEh => "bopomofo letter eh",
            Bopomofo::LetterAi => "bopomofo letter ai",
            Bopomofo::LetterEi => "bopomofo letter ei",
            Bopomofo::LetterAu => "bopomofo letter au",
            Bopomofo::LetterOu => "bopomofo letter ou",
            Bopomofo::LetterAn => "bopomofo letter an",
            Bopomofo::LetterEn => "bopomofo letter en",
            Bopomofo::LetterAng => "bopomofo letter ang",
            Bopomofo::LetterEng => "bopomofo letter eng",
            Bopomofo::LetterEr => "bopomofo letter er",
            Bopomofo::LetterI => "bopomofo letter i",
            Bopomofo::LetterU => "bopomofo letter u",
            Bopomofo::LetterIu => "bopomofo letter iu",
            Bopomofo::LetterV => "bopomofo letter v",
            Bopomofo::LetterNg => "bopomofo letter ng",
            Bopomofo::LetterGn => "bopomofo letter gn",
            Bopomofo::LetterIh => "bopomofo letter ih",
            Bopomofo::LetterOWithDotAbove => "bopomofo letter o with dot above",
        }
    }
}
