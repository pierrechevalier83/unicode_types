/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{10280}: '𐊀'
    pub const LETTER_A: char = '𐊀';
    /// \u{10281}: '𐊁'
    pub const LETTER_E: char = '𐊁';
    /// \u{10282}: '𐊂'
    pub const LETTER_B: char = '𐊂';
    /// \u{10283}: '𐊃'
    pub const LETTER_BH: char = '𐊃';
    /// \u{10284}: '𐊄'
    pub const LETTER_G: char = '𐊄';
    /// \u{10285}: '𐊅'
    pub const LETTER_D: char = '𐊅';
    /// \u{10286}: '𐊆'
    pub const LETTER_I: char = '𐊆';
    /// \u{10287}: '𐊇'
    pub const LETTER_W: char = '𐊇';
    /// \u{10288}: '𐊈'
    pub const LETTER_Z: char = '𐊈';
    /// \u{10289}: '𐊉'
    pub const LETTER_TH: char = '𐊉';
    /// \u{1028a}: '𐊊'
    pub const LETTER_J: char = '𐊊';
    /// \u{1028b}: '𐊋'
    pub const LETTER_K: char = '𐊋';
    /// \u{1028c}: '𐊌'
    pub const LETTER_Q: char = '𐊌';
    /// \u{1028d}: '𐊍'
    pub const LETTER_L: char = '𐊍';
    /// \u{1028e}: '𐊎'
    pub const LETTER_M: char = '𐊎';
    /// \u{1028f}: '𐊏'
    pub const LETTER_N: char = '𐊏';
    /// \u{10290}: '𐊐'
    pub const LETTER_MM: char = '𐊐';
    /// \u{10291}: '𐊑'
    pub const LETTER_NN: char = '𐊑';
    /// \u{10292}: '𐊒'
    pub const LETTER_U: char = '𐊒';
    /// \u{10293}: '𐊓'
    pub const LETTER_P: char = '𐊓';
    /// \u{10294}: '𐊔'
    pub const LETTER_KK: char = '𐊔';
    /// \u{10295}: '𐊕'
    pub const LETTER_R: char = '𐊕';
    /// \u{10296}: '𐊖'
    pub const LETTER_S: char = '𐊖';
    /// \u{10297}: '𐊗'
    pub const LETTER_T: char = '𐊗';
    /// \u{10298}: '𐊘'
    pub const LETTER_TT: char = '𐊘';
    /// \u{10299}: '𐊙'
    pub const LETTER_AN: char = '𐊙';
    /// \u{1029a}: '𐊚'
    pub const LETTER_EN: char = '𐊚';
    /// \u{1029b}: '𐊛'
    pub const LETTER_H: char = '𐊛';
    /// \u{1029c}: '𐊜'
    pub const LETTER_X: char = '𐊜';
}

/// An enum to represent all characters in the Lycian block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Lycian {
    /// \u{10280}: '𐊀'
    LetterA,
    /// \u{10281}: '𐊁'
    LetterE,
    /// \u{10282}: '𐊂'
    LetterB,
    /// \u{10283}: '𐊃'
    LetterBh,
    /// \u{10284}: '𐊄'
    LetterG,
    /// \u{10285}: '𐊅'
    LetterD,
    /// \u{10286}: '𐊆'
    LetterI,
    /// \u{10287}: '𐊇'
    LetterW,
    /// \u{10288}: '𐊈'
    LetterZ,
    /// \u{10289}: '𐊉'
    LetterTh,
    /// \u{1028a}: '𐊊'
    LetterJ,
    /// \u{1028b}: '𐊋'
    LetterK,
    /// \u{1028c}: '𐊌'
    LetterQ,
    /// \u{1028d}: '𐊍'
    LetterL,
    /// \u{1028e}: '𐊎'
    LetterM,
    /// \u{1028f}: '𐊏'
    LetterN,
    /// \u{10290}: '𐊐'
    LetterMm,
    /// \u{10291}: '𐊑'
    LetterNn,
    /// \u{10292}: '𐊒'
    LetterU,
    /// \u{10293}: '𐊓'
    LetterP,
    /// \u{10294}: '𐊔'
    LetterKk,
    /// \u{10295}: '𐊕'
    LetterR,
    /// \u{10296}: '𐊖'
    LetterS,
    /// \u{10297}: '𐊗'
    LetterT,
    /// \u{10298}: '𐊘'
    LetterTt,
    /// \u{10299}: '𐊙'
    LetterAn,
    /// \u{1029a}: '𐊚'
    LetterEn,
    /// \u{1029b}: '𐊛'
    LetterH,
    /// \u{1029c}: '𐊜'
    LetterX,
}

impl Into<char> for Lycian {
    fn into(self) -> char {
        use constants::*;
        match self {
            Lycian::LetterA => LETTER_A,
            Lycian::LetterE => LETTER_E,
            Lycian::LetterB => LETTER_B,
            Lycian::LetterBh => LETTER_BH,
            Lycian::LetterG => LETTER_G,
            Lycian::LetterD => LETTER_D,
            Lycian::LetterI => LETTER_I,
            Lycian::LetterW => LETTER_W,
            Lycian::LetterZ => LETTER_Z,
            Lycian::LetterTh => LETTER_TH,
            Lycian::LetterJ => LETTER_J,
            Lycian::LetterK => LETTER_K,
            Lycian::LetterQ => LETTER_Q,
            Lycian::LetterL => LETTER_L,
            Lycian::LetterM => LETTER_M,
            Lycian::LetterN => LETTER_N,
            Lycian::LetterMm => LETTER_MM,
            Lycian::LetterNn => LETTER_NN,
            Lycian::LetterU => LETTER_U,
            Lycian::LetterP => LETTER_P,
            Lycian::LetterKk => LETTER_KK,
            Lycian::LetterR => LETTER_R,
            Lycian::LetterS => LETTER_S,
            Lycian::LetterT => LETTER_T,
            Lycian::LetterTt => LETTER_TT,
            Lycian::LetterAn => LETTER_AN,
            Lycian::LetterEn => LETTER_EN,
            Lycian::LetterH => LETTER_H,
            Lycian::LetterX => LETTER_X,
        }
    }
}

impl std::convert::TryFrom<char> for Lycian {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_A => Ok(Lycian::LetterA),
            LETTER_E => Ok(Lycian::LetterE),
            LETTER_B => Ok(Lycian::LetterB),
            LETTER_BH => Ok(Lycian::LetterBh),
            LETTER_G => Ok(Lycian::LetterG),
            LETTER_D => Ok(Lycian::LetterD),
            LETTER_I => Ok(Lycian::LetterI),
            LETTER_W => Ok(Lycian::LetterW),
            LETTER_Z => Ok(Lycian::LetterZ),
            LETTER_TH => Ok(Lycian::LetterTh),
            LETTER_J => Ok(Lycian::LetterJ),
            LETTER_K => Ok(Lycian::LetterK),
            LETTER_Q => Ok(Lycian::LetterQ),
            LETTER_L => Ok(Lycian::LetterL),
            LETTER_M => Ok(Lycian::LetterM),
            LETTER_N => Ok(Lycian::LetterN),
            LETTER_MM => Ok(Lycian::LetterMm),
            LETTER_NN => Ok(Lycian::LetterNn),
            LETTER_U => Ok(Lycian::LetterU),
            LETTER_P => Ok(Lycian::LetterP),
            LETTER_KK => Ok(Lycian::LetterKk),
            LETTER_R => Ok(Lycian::LetterR),
            LETTER_S => Ok(Lycian::LetterS),
            LETTER_T => Ok(Lycian::LetterT),
            LETTER_TT => Ok(Lycian::LetterTt),
            LETTER_AN => Ok(Lycian::LetterAn),
            LETTER_EN => Ok(Lycian::LetterEn),
            LETTER_H => Ok(Lycian::LetterH),
            LETTER_X => Ok(Lycian::LetterX),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Lycian {
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

impl std::convert::TryFrom<u32> for Lycian {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Lycian {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Lycian {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Lycian::LetterA
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Lycian::LetterA => "lycian letter a",
            Lycian::LetterE => "lycian letter e",
            Lycian::LetterB => "lycian letter b",
            Lycian::LetterBh => "lycian letter bh",
            Lycian::LetterG => "lycian letter g",
            Lycian::LetterD => "lycian letter d",
            Lycian::LetterI => "lycian letter i",
            Lycian::LetterW => "lycian letter w",
            Lycian::LetterZ => "lycian letter z",
            Lycian::LetterTh => "lycian letter th",
            Lycian::LetterJ => "lycian letter j",
            Lycian::LetterK => "lycian letter k",
            Lycian::LetterQ => "lycian letter q",
            Lycian::LetterL => "lycian letter l",
            Lycian::LetterM => "lycian letter m",
            Lycian::LetterN => "lycian letter n",
            Lycian::LetterMm => "lycian letter mm",
            Lycian::LetterNn => "lycian letter nn",
            Lycian::LetterU => "lycian letter u",
            Lycian::LetterP => "lycian letter p",
            Lycian::LetterKk => "lycian letter kk",
            Lycian::LetterR => "lycian letter r",
            Lycian::LetterS => "lycian letter s",
            Lycian::LetterT => "lycian letter t",
            Lycian::LetterTt => "lycian letter tt",
            Lycian::LetterAn => "lycian letter an",
            Lycian::LetterEn => "lycian letter en",
            Lycian::LetterH => "lycian letter h",
            Lycian::LetterX => "lycian letter x",
        }
    }
}
