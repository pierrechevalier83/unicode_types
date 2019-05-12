/// \u{10920} → \u{1093f}\
///\
/// 𐤠 𐤡 𐤢 𐤣 𐤤 𐤥 𐤦 𐤧 𐤨 𐤩 𐤪 𐤫 𐤬 𐤭 𐤮 𐤯
/// 𐤰 𐤱 𐤲 𐤳 𐤴 𐤵 𐤶 𐤷 𐤸 𐤹
pub mod constants {
    /// \u{10920}: '𐤠'
    pub const LETTER_A: char = '𐤠';
    /// \u{10921}: '𐤡'
    pub const LETTER_B: char = '𐤡';
    /// \u{10922}: '𐤢'
    pub const LETTER_G: char = '𐤢';
    /// \u{10923}: '𐤣'
    pub const LETTER_D: char = '𐤣';
    /// \u{10924}: '𐤤'
    pub const LETTER_E: char = '𐤤';
    /// \u{10925}: '𐤥'
    pub const LETTER_V: char = '𐤥';
    /// \u{10926}: '𐤦'
    pub const LETTER_I: char = '𐤦';
    /// \u{10927}: '𐤧'
    pub const LETTER_Y: char = '𐤧';
    /// \u{10928}: '𐤨'
    pub const LETTER_K: char = '𐤨';
    /// \u{10929}: '𐤩'
    pub const LETTER_L: char = '𐤩';
    /// \u{1092a}: '𐤪'
    pub const LETTER_M: char = '𐤪';
    /// \u{1092b}: '𐤫'
    pub const LETTER_N: char = '𐤫';
    /// \u{1092c}: '𐤬'
    pub const LETTER_O: char = '𐤬';
    /// \u{1092d}: '𐤭'
    pub const LETTER_R: char = '𐤭';
    /// \u{1092e}: '𐤮'
    pub const LETTER_SS: char = '𐤮';
    /// \u{1092f}: '𐤯'
    pub const LETTER_T: char = '𐤯';
    /// \u{10930}: '𐤰'
    pub const LETTER_U: char = '𐤰';
    /// \u{10931}: '𐤱'
    pub const LETTER_F: char = '𐤱';
    /// \u{10932}: '𐤲'
    pub const LETTER_Q: char = '𐤲';
    /// \u{10933}: '𐤳'
    pub const LETTER_S: char = '𐤳';
    /// \u{10934}: '𐤴'
    pub const LETTER_TT: char = '𐤴';
    /// \u{10935}: '𐤵'
    pub const LETTER_AN: char = '𐤵';
    /// \u{10936}: '𐤶'
    pub const LETTER_EN: char = '𐤶';
    /// \u{10937}: '𐤷'
    pub const LETTER_LY: char = '𐤷';
    /// \u{10938}: '𐤸'
    pub const LETTER_NN: char = '𐤸';
    /// \u{10939}: '𐤹'
    pub const LETTER_C: char = '𐤹';
}

/// \u{10920} → \u{1093f}\
///\
/// 𐤠 𐤡 𐤢 𐤣 𐤤 𐤥 𐤦 𐤧 𐤨 𐤩 𐤪 𐤫 𐤬 𐤭 𐤮 𐤯
/// 𐤰 𐤱 𐤲 𐤳 𐤴 𐤵 𐤶 𐤷 𐤸 𐤹
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Lydian {
    /// \u{10920}: '𐤠'
    LetterA,
    /// \u{10921}: '𐤡'
    LetterB,
    /// \u{10922}: '𐤢'
    LetterG,
    /// \u{10923}: '𐤣'
    LetterD,
    /// \u{10924}: '𐤤'
    LetterE,
    /// \u{10925}: '𐤥'
    LetterV,
    /// \u{10926}: '𐤦'
    LetterI,
    /// \u{10927}: '𐤧'
    LetterY,
    /// \u{10928}: '𐤨'
    LetterK,
    /// \u{10929}: '𐤩'
    LetterL,
    /// \u{1092a}: '𐤪'
    LetterM,
    /// \u{1092b}: '𐤫'
    LetterN,
    /// \u{1092c}: '𐤬'
    LetterO,
    /// \u{1092d}: '𐤭'
    LetterR,
    /// \u{1092e}: '𐤮'
    LetterSs,
    /// \u{1092f}: '𐤯'
    LetterT,
    /// \u{10930}: '𐤰'
    LetterU,
    /// \u{10931}: '𐤱'
    LetterF,
    /// \u{10932}: '𐤲'
    LetterQ,
    /// \u{10933}: '𐤳'
    LetterS,
    /// \u{10934}: '𐤴'
    LetterTt,
    /// \u{10935}: '𐤵'
    LetterAn,
    /// \u{10936}: '𐤶'
    LetterEn,
    /// \u{10937}: '𐤷'
    LetterLy,
    /// \u{10938}: '𐤸'
    LetterNn,
    /// \u{10939}: '𐤹'
    LetterC,
}

impl Into<char> for Lydian {
    fn into(self) -> char {
        use constants::*;
        match self {
            Lydian::LetterA => LETTER_A,
            Lydian::LetterB => LETTER_B,
            Lydian::LetterG => LETTER_G,
            Lydian::LetterD => LETTER_D,
            Lydian::LetterE => LETTER_E,
            Lydian::LetterV => LETTER_V,
            Lydian::LetterI => LETTER_I,
            Lydian::LetterY => LETTER_Y,
            Lydian::LetterK => LETTER_K,
            Lydian::LetterL => LETTER_L,
            Lydian::LetterM => LETTER_M,
            Lydian::LetterN => LETTER_N,
            Lydian::LetterO => LETTER_O,
            Lydian::LetterR => LETTER_R,
            Lydian::LetterSs => LETTER_SS,
            Lydian::LetterT => LETTER_T,
            Lydian::LetterU => LETTER_U,
            Lydian::LetterF => LETTER_F,
            Lydian::LetterQ => LETTER_Q,
            Lydian::LetterS => LETTER_S,
            Lydian::LetterTt => LETTER_TT,
            Lydian::LetterAn => LETTER_AN,
            Lydian::LetterEn => LETTER_EN,
            Lydian::LetterLy => LETTER_LY,
            Lydian::LetterNn => LETTER_NN,
            Lydian::LetterC => LETTER_C,
        }
    }
}

impl std::convert::TryFrom<char> for Lydian {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_A => Ok(Lydian::LetterA),
            LETTER_B => Ok(Lydian::LetterB),
            LETTER_G => Ok(Lydian::LetterG),
            LETTER_D => Ok(Lydian::LetterD),
            LETTER_E => Ok(Lydian::LetterE),
            LETTER_V => Ok(Lydian::LetterV),
            LETTER_I => Ok(Lydian::LetterI),
            LETTER_Y => Ok(Lydian::LetterY),
            LETTER_K => Ok(Lydian::LetterK),
            LETTER_L => Ok(Lydian::LetterL),
            LETTER_M => Ok(Lydian::LetterM),
            LETTER_N => Ok(Lydian::LetterN),
            LETTER_O => Ok(Lydian::LetterO),
            LETTER_R => Ok(Lydian::LetterR),
            LETTER_SS => Ok(Lydian::LetterSs),
            LETTER_T => Ok(Lydian::LetterT),
            LETTER_U => Ok(Lydian::LetterU),
            LETTER_F => Ok(Lydian::LetterF),
            LETTER_Q => Ok(Lydian::LetterQ),
            LETTER_S => Ok(Lydian::LetterS),
            LETTER_TT => Ok(Lydian::LetterTt),
            LETTER_AN => Ok(Lydian::LetterAn),
            LETTER_EN => Ok(Lydian::LetterEn),
            LETTER_LY => Ok(Lydian::LetterLy),
            LETTER_NN => Ok(Lydian::LetterNn),
            LETTER_C => Ok(Lydian::LetterC),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Lydian {
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

impl std::convert::TryFrom<u32> for Lydian {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Lydian {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Lydian {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Lydian::LetterA
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Lydian::LetterA => "lydian letter a",
            Lydian::LetterB => "lydian letter b",
            Lydian::LetterG => "lydian letter g",
            Lydian::LetterD => "lydian letter d",
            Lydian::LetterE => "lydian letter e",
            Lydian::LetterV => "lydian letter v",
            Lydian::LetterI => "lydian letter i",
            Lydian::LetterY => "lydian letter y",
            Lydian::LetterK => "lydian letter k",
            Lydian::LetterL => "lydian letter l",
            Lydian::LetterM => "lydian letter m",
            Lydian::LetterN => "lydian letter n",
            Lydian::LetterO => "lydian letter o",
            Lydian::LetterR => "lydian letter r",
            Lydian::LetterSs => "lydian letter ss",
            Lydian::LetterT => "lydian letter t",
            Lydian::LetterU => "lydian letter u",
            Lydian::LetterF => "lydian letter f",
            Lydian::LetterQ => "lydian letter q",
            Lydian::LetterS => "lydian letter s",
            Lydian::LetterTt => "lydian letter tt",
            Lydian::LetterAn => "lydian letter an",
            Lydian::LetterEn => "lydian letter en",
            Lydian::LetterLy => "lydian letter ly",
            Lydian::LetterNn => "lydian letter nn",
            Lydian::LetterC => "lydian letter c",
        }
    }
}
