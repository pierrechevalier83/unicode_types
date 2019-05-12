/// \u{10280} → \u{1029f}\
///\
/// 𐊀 𐊁 𐊂 𐊃 𐊄 𐊅 𐊆 𐊇 𐊈 𐊉 𐊊 𐊋 𐊌 𐊍 𐊎 𐊏
/// 𐊐 𐊑 𐊒 𐊓 𐊔 𐊕 𐊖 𐊗 𐊘 𐊙 𐊚 𐊛 𐊜
pub mod constants {
    /// \u{10280}: '𐊀'
    pub const LYCIAN_LETTER_A: char = '𐊀';
    /// \u{10281}: '𐊁'
    pub const LYCIAN_LETTER_E: char = '𐊁';
    /// \u{10282}: '𐊂'
    pub const LYCIAN_LETTER_B: char = '𐊂';
    /// \u{10283}: '𐊃'
    pub const LYCIAN_LETTER_BH: char = '𐊃';
    /// \u{10284}: '𐊄'
    pub const LYCIAN_LETTER_G: char = '𐊄';
    /// \u{10285}: '𐊅'
    pub const LYCIAN_LETTER_D: char = '𐊅';
    /// \u{10286}: '𐊆'
    pub const LYCIAN_LETTER_I: char = '𐊆';
    /// \u{10287}: '𐊇'
    pub const LYCIAN_LETTER_W: char = '𐊇';
    /// \u{10288}: '𐊈'
    pub const LYCIAN_LETTER_Z: char = '𐊈';
    /// \u{10289}: '𐊉'
    pub const LYCIAN_LETTER_TH: char = '𐊉';
    /// \u{1028a}: '𐊊'
    pub const LYCIAN_LETTER_J: char = '𐊊';
    /// \u{1028b}: '𐊋'
    pub const LYCIAN_LETTER_K: char = '𐊋';
    /// \u{1028c}: '𐊌'
    pub const LYCIAN_LETTER_Q: char = '𐊌';
    /// \u{1028d}: '𐊍'
    pub const LYCIAN_LETTER_L: char = '𐊍';
    /// \u{1028e}: '𐊎'
    pub const LYCIAN_LETTER_M: char = '𐊎';
    /// \u{1028f}: '𐊏'
    pub const LYCIAN_LETTER_N: char = '𐊏';
    /// \u{10290}: '𐊐'
    pub const LYCIAN_LETTER_MM: char = '𐊐';
    /// \u{10291}: '𐊑'
    pub const LYCIAN_LETTER_NN: char = '𐊑';
    /// \u{10292}: '𐊒'
    pub const LYCIAN_LETTER_U: char = '𐊒';
    /// \u{10293}: '𐊓'
    pub const LYCIAN_LETTER_P: char = '𐊓';
    /// \u{10294}: '𐊔'
    pub const LYCIAN_LETTER_KK: char = '𐊔';
    /// \u{10295}: '𐊕'
    pub const LYCIAN_LETTER_R: char = '𐊕';
    /// \u{10296}: '𐊖'
    pub const LYCIAN_LETTER_S: char = '𐊖';
    /// \u{10297}: '𐊗'
    pub const LYCIAN_LETTER_T: char = '𐊗';
    /// \u{10298}: '𐊘'
    pub const LYCIAN_LETTER_TT: char = '𐊘';
    /// \u{10299}: '𐊙'
    pub const LYCIAN_LETTER_AN: char = '𐊙';
    /// \u{1029a}: '𐊚'
    pub const LYCIAN_LETTER_EN: char = '𐊚';
    /// \u{1029b}: '𐊛'
    pub const LYCIAN_LETTER_H: char = '𐊛';
    /// \u{1029c}: '𐊜'
    pub const LYCIAN_LETTER_X: char = '𐊜';
}

/// \u{10280} → \u{1029f}\
///\
/// 𐊀 𐊁 𐊂 𐊃 𐊄 𐊅 𐊆 𐊇 𐊈 𐊉 𐊊 𐊋 𐊌 𐊍 𐊎 𐊏
/// 𐊐 𐊑 𐊒 𐊓 𐊔 𐊕 𐊖 𐊗 𐊘 𐊙 𐊚 𐊛 𐊜
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Lycian {
    /// \u{10280}: '𐊀'
    LycianLetterA,
    /// \u{10281}: '𐊁'
    LycianLetterE,
    /// \u{10282}: '𐊂'
    LycianLetterB,
    /// \u{10283}: '𐊃'
    LycianLetterBh,
    /// \u{10284}: '𐊄'
    LycianLetterG,
    /// \u{10285}: '𐊅'
    LycianLetterD,
    /// \u{10286}: '𐊆'
    LycianLetterI,
    /// \u{10287}: '𐊇'
    LycianLetterW,
    /// \u{10288}: '𐊈'
    LycianLetterZ,
    /// \u{10289}: '𐊉'
    LycianLetterTh,
    /// \u{1028a}: '𐊊'
    LycianLetterJ,
    /// \u{1028b}: '𐊋'
    LycianLetterK,
    /// \u{1028c}: '𐊌'
    LycianLetterQ,
    /// \u{1028d}: '𐊍'
    LycianLetterL,
    /// \u{1028e}: '𐊎'
    LycianLetterM,
    /// \u{1028f}: '𐊏'
    LycianLetterN,
    /// \u{10290}: '𐊐'
    LycianLetterMm,
    /// \u{10291}: '𐊑'
    LycianLetterNn,
    /// \u{10292}: '𐊒'
    LycianLetterU,
    /// \u{10293}: '𐊓'
    LycianLetterP,
    /// \u{10294}: '𐊔'
    LycianLetterKk,
    /// \u{10295}: '𐊕'
    LycianLetterR,
    /// \u{10296}: '𐊖'
    LycianLetterS,
    /// \u{10297}: '𐊗'
    LycianLetterT,
    /// \u{10298}: '𐊘'
    LycianLetterTt,
    /// \u{10299}: '𐊙'
    LycianLetterAn,
    /// \u{1029a}: '𐊚'
    LycianLetterEn,
    /// \u{1029b}: '𐊛'
    LycianLetterH,
    /// \u{1029c}: '𐊜'
    LycianLetterX,
}

impl Into<char> for Lycian {
    fn into(self) -> char {
        use constants::*;
        match self {
            Lycian::LycianLetterA => LYCIAN_LETTER_A,
            Lycian::LycianLetterE => LYCIAN_LETTER_E,
            Lycian::LycianLetterB => LYCIAN_LETTER_B,
            Lycian::LycianLetterBh => LYCIAN_LETTER_BH,
            Lycian::LycianLetterG => LYCIAN_LETTER_G,
            Lycian::LycianLetterD => LYCIAN_LETTER_D,
            Lycian::LycianLetterI => LYCIAN_LETTER_I,
            Lycian::LycianLetterW => LYCIAN_LETTER_W,
            Lycian::LycianLetterZ => LYCIAN_LETTER_Z,
            Lycian::LycianLetterTh => LYCIAN_LETTER_TH,
            Lycian::LycianLetterJ => LYCIAN_LETTER_J,
            Lycian::LycianLetterK => LYCIAN_LETTER_K,
            Lycian::LycianLetterQ => LYCIAN_LETTER_Q,
            Lycian::LycianLetterL => LYCIAN_LETTER_L,
            Lycian::LycianLetterM => LYCIAN_LETTER_M,
            Lycian::LycianLetterN => LYCIAN_LETTER_N,
            Lycian::LycianLetterMm => LYCIAN_LETTER_MM,
            Lycian::LycianLetterNn => LYCIAN_LETTER_NN,
            Lycian::LycianLetterU => LYCIAN_LETTER_U,
            Lycian::LycianLetterP => LYCIAN_LETTER_P,
            Lycian::LycianLetterKk => LYCIAN_LETTER_KK,
            Lycian::LycianLetterR => LYCIAN_LETTER_R,
            Lycian::LycianLetterS => LYCIAN_LETTER_S,
            Lycian::LycianLetterT => LYCIAN_LETTER_T,
            Lycian::LycianLetterTt => LYCIAN_LETTER_TT,
            Lycian::LycianLetterAn => LYCIAN_LETTER_AN,
            Lycian::LycianLetterEn => LYCIAN_LETTER_EN,
            Lycian::LycianLetterH => LYCIAN_LETTER_H,
            Lycian::LycianLetterX => LYCIAN_LETTER_X,
        }
    }
}

impl std::convert::TryFrom<char> for Lycian {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LYCIAN_LETTER_A => Ok(Lycian::LycianLetterA),
            LYCIAN_LETTER_E => Ok(Lycian::LycianLetterE),
            LYCIAN_LETTER_B => Ok(Lycian::LycianLetterB),
            LYCIAN_LETTER_BH => Ok(Lycian::LycianLetterBh),
            LYCIAN_LETTER_G => Ok(Lycian::LycianLetterG),
            LYCIAN_LETTER_D => Ok(Lycian::LycianLetterD),
            LYCIAN_LETTER_I => Ok(Lycian::LycianLetterI),
            LYCIAN_LETTER_W => Ok(Lycian::LycianLetterW),
            LYCIAN_LETTER_Z => Ok(Lycian::LycianLetterZ),
            LYCIAN_LETTER_TH => Ok(Lycian::LycianLetterTh),
            LYCIAN_LETTER_J => Ok(Lycian::LycianLetterJ),
            LYCIAN_LETTER_K => Ok(Lycian::LycianLetterK),
            LYCIAN_LETTER_Q => Ok(Lycian::LycianLetterQ),
            LYCIAN_LETTER_L => Ok(Lycian::LycianLetterL),
            LYCIAN_LETTER_M => Ok(Lycian::LycianLetterM),
            LYCIAN_LETTER_N => Ok(Lycian::LycianLetterN),
            LYCIAN_LETTER_MM => Ok(Lycian::LycianLetterMm),
            LYCIAN_LETTER_NN => Ok(Lycian::LycianLetterNn),
            LYCIAN_LETTER_U => Ok(Lycian::LycianLetterU),
            LYCIAN_LETTER_P => Ok(Lycian::LycianLetterP),
            LYCIAN_LETTER_KK => Ok(Lycian::LycianLetterKk),
            LYCIAN_LETTER_R => Ok(Lycian::LycianLetterR),
            LYCIAN_LETTER_S => Ok(Lycian::LycianLetterS),
            LYCIAN_LETTER_T => Ok(Lycian::LycianLetterT),
            LYCIAN_LETTER_TT => Ok(Lycian::LycianLetterTt),
            LYCIAN_LETTER_AN => Ok(Lycian::LycianLetterAn),
            LYCIAN_LETTER_EN => Ok(Lycian::LycianLetterEn),
            LYCIAN_LETTER_H => Ok(Lycian::LycianLetterH),
            LYCIAN_LETTER_X => Ok(Lycian::LycianLetterX),
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
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        Lycian::LycianLetterA
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Lycian::LycianLetterA => "lycian letter a",
            Lycian::LycianLetterE => "lycian letter e",
            Lycian::LycianLetterB => "lycian letter b",
            Lycian::LycianLetterBh => "lycian letter bh",
            Lycian::LycianLetterG => "lycian letter g",
            Lycian::LycianLetterD => "lycian letter d",
            Lycian::LycianLetterI => "lycian letter i",
            Lycian::LycianLetterW => "lycian letter w",
            Lycian::LycianLetterZ => "lycian letter z",
            Lycian::LycianLetterTh => "lycian letter th",
            Lycian::LycianLetterJ => "lycian letter j",
            Lycian::LycianLetterK => "lycian letter k",
            Lycian::LycianLetterQ => "lycian letter q",
            Lycian::LycianLetterL => "lycian letter l",
            Lycian::LycianLetterM => "lycian letter m",
            Lycian::LycianLetterN => "lycian letter n",
            Lycian::LycianLetterMm => "lycian letter mm",
            Lycian::LycianLetterNn => "lycian letter nn",
            Lycian::LycianLetterU => "lycian letter u",
            Lycian::LycianLetterP => "lycian letter p",
            Lycian::LycianLetterKk => "lycian letter kk",
            Lycian::LycianLetterR => "lycian letter r",
            Lycian::LycianLetterS => "lycian letter s",
            Lycian::LycianLetterT => "lycian letter t",
            Lycian::LycianLetterTt => "lycian letter tt",
            Lycian::LycianLetterAn => "lycian letter an",
            Lycian::LycianLetterEn => "lycian letter en",
            Lycian::LycianLetterH => "lycian letter h",
            Lycian::LycianLetterX => "lycian letter x",
        }
    }
}
