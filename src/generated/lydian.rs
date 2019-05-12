/// \u{10920} → \u{1093f}\
///\
/// 𐤠 𐤡 𐤢 𐤣 𐤤 𐤥 𐤦 𐤧 𐤨 𐤩 𐤪 𐤫 𐤬 𐤭 𐤮 𐤯
/// 𐤰 𐤱 𐤲 𐤳 𐤴 𐤵 𐤶 𐤷 𐤸 𐤹
pub mod constants {
    /// \u{10920}: '𐤠'
    pub const LYDIAN_LETTER_A: char = '𐤠';
    /// \u{10921}: '𐤡'
    pub const LYDIAN_LETTER_B: char = '𐤡';
    /// \u{10922}: '𐤢'
    pub const LYDIAN_LETTER_G: char = '𐤢';
    /// \u{10923}: '𐤣'
    pub const LYDIAN_LETTER_D: char = '𐤣';
    /// \u{10924}: '𐤤'
    pub const LYDIAN_LETTER_E: char = '𐤤';
    /// \u{10925}: '𐤥'
    pub const LYDIAN_LETTER_V: char = '𐤥';
    /// \u{10926}: '𐤦'
    pub const LYDIAN_LETTER_I: char = '𐤦';
    /// \u{10927}: '𐤧'
    pub const LYDIAN_LETTER_Y: char = '𐤧';
    /// \u{10928}: '𐤨'
    pub const LYDIAN_LETTER_K: char = '𐤨';
    /// \u{10929}: '𐤩'
    pub const LYDIAN_LETTER_L: char = '𐤩';
    /// \u{1092a}: '𐤪'
    pub const LYDIAN_LETTER_M: char = '𐤪';
    /// \u{1092b}: '𐤫'
    pub const LYDIAN_LETTER_N: char = '𐤫';
    /// \u{1092c}: '𐤬'
    pub const LYDIAN_LETTER_O: char = '𐤬';
    /// \u{1092d}: '𐤭'
    pub const LYDIAN_LETTER_R: char = '𐤭';
    /// \u{1092e}: '𐤮'
    pub const LYDIAN_LETTER_SS: char = '𐤮';
    /// \u{1092f}: '𐤯'
    pub const LYDIAN_LETTER_T: char = '𐤯';
    /// \u{10930}: '𐤰'
    pub const LYDIAN_LETTER_U: char = '𐤰';
    /// \u{10931}: '𐤱'
    pub const LYDIAN_LETTER_F: char = '𐤱';
    /// \u{10932}: '𐤲'
    pub const LYDIAN_LETTER_Q: char = '𐤲';
    /// \u{10933}: '𐤳'
    pub const LYDIAN_LETTER_S: char = '𐤳';
    /// \u{10934}: '𐤴'
    pub const LYDIAN_LETTER_TT: char = '𐤴';
    /// \u{10935}: '𐤵'
    pub const LYDIAN_LETTER_AN: char = '𐤵';
    /// \u{10936}: '𐤶'
    pub const LYDIAN_LETTER_EN: char = '𐤶';
    /// \u{10937}: '𐤷'
    pub const LYDIAN_LETTER_LY: char = '𐤷';
    /// \u{10938}: '𐤸'
    pub const LYDIAN_LETTER_NN: char = '𐤸';
    /// \u{10939}: '𐤹'
    pub const LYDIAN_LETTER_C: char = '𐤹';
}

/// \u{10920} → \u{1093f}\
///\
/// 𐤠 𐤡 𐤢 𐤣 𐤤 𐤥 𐤦 𐤧 𐤨 𐤩 𐤪 𐤫 𐤬 𐤭 𐤮 𐤯
/// 𐤰 𐤱 𐤲 𐤳 𐤴 𐤵 𐤶 𐤷 𐤸 𐤹
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Lydian {
    /// \u{10920}: '𐤠'
    LydianLetterA,
    /// \u{10921}: '𐤡'
    LydianLetterB,
    /// \u{10922}: '𐤢'
    LydianLetterG,
    /// \u{10923}: '𐤣'
    LydianLetterD,
    /// \u{10924}: '𐤤'
    LydianLetterE,
    /// \u{10925}: '𐤥'
    LydianLetterV,
    /// \u{10926}: '𐤦'
    LydianLetterI,
    /// \u{10927}: '𐤧'
    LydianLetterY,
    /// \u{10928}: '𐤨'
    LydianLetterK,
    /// \u{10929}: '𐤩'
    LydianLetterL,
    /// \u{1092a}: '𐤪'
    LydianLetterM,
    /// \u{1092b}: '𐤫'
    LydianLetterN,
    /// \u{1092c}: '𐤬'
    LydianLetterO,
    /// \u{1092d}: '𐤭'
    LydianLetterR,
    /// \u{1092e}: '𐤮'
    LydianLetterSs,
    /// \u{1092f}: '𐤯'
    LydianLetterT,
    /// \u{10930}: '𐤰'
    LydianLetterU,
    /// \u{10931}: '𐤱'
    LydianLetterF,
    /// \u{10932}: '𐤲'
    LydianLetterQ,
    /// \u{10933}: '𐤳'
    LydianLetterS,
    /// \u{10934}: '𐤴'
    LydianLetterTt,
    /// \u{10935}: '𐤵'
    LydianLetterAn,
    /// \u{10936}: '𐤶'
    LydianLetterEn,
    /// \u{10937}: '𐤷'
    LydianLetterLy,
    /// \u{10938}: '𐤸'
    LydianLetterNn,
    /// \u{10939}: '𐤹'
    LydianLetterC,
}

impl Into<char> for Lydian {
    fn into(self) -> char {
        use constants::*;
        match self {
            Lydian::LydianLetterA => LYDIAN_LETTER_A,
            Lydian::LydianLetterB => LYDIAN_LETTER_B,
            Lydian::LydianLetterG => LYDIAN_LETTER_G,
            Lydian::LydianLetterD => LYDIAN_LETTER_D,
            Lydian::LydianLetterE => LYDIAN_LETTER_E,
            Lydian::LydianLetterV => LYDIAN_LETTER_V,
            Lydian::LydianLetterI => LYDIAN_LETTER_I,
            Lydian::LydianLetterY => LYDIAN_LETTER_Y,
            Lydian::LydianLetterK => LYDIAN_LETTER_K,
            Lydian::LydianLetterL => LYDIAN_LETTER_L,
            Lydian::LydianLetterM => LYDIAN_LETTER_M,
            Lydian::LydianLetterN => LYDIAN_LETTER_N,
            Lydian::LydianLetterO => LYDIAN_LETTER_O,
            Lydian::LydianLetterR => LYDIAN_LETTER_R,
            Lydian::LydianLetterSs => LYDIAN_LETTER_SS,
            Lydian::LydianLetterT => LYDIAN_LETTER_T,
            Lydian::LydianLetterU => LYDIAN_LETTER_U,
            Lydian::LydianLetterF => LYDIAN_LETTER_F,
            Lydian::LydianLetterQ => LYDIAN_LETTER_Q,
            Lydian::LydianLetterS => LYDIAN_LETTER_S,
            Lydian::LydianLetterTt => LYDIAN_LETTER_TT,
            Lydian::LydianLetterAn => LYDIAN_LETTER_AN,
            Lydian::LydianLetterEn => LYDIAN_LETTER_EN,
            Lydian::LydianLetterLy => LYDIAN_LETTER_LY,
            Lydian::LydianLetterNn => LYDIAN_LETTER_NN,
            Lydian::LydianLetterC => LYDIAN_LETTER_C,
        }
    }
}

impl std::convert::TryFrom<char> for Lydian {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LYDIAN_LETTER_A => Ok(Lydian::LydianLetterA),
            LYDIAN_LETTER_B => Ok(Lydian::LydianLetterB),
            LYDIAN_LETTER_G => Ok(Lydian::LydianLetterG),
            LYDIAN_LETTER_D => Ok(Lydian::LydianLetterD),
            LYDIAN_LETTER_E => Ok(Lydian::LydianLetterE),
            LYDIAN_LETTER_V => Ok(Lydian::LydianLetterV),
            LYDIAN_LETTER_I => Ok(Lydian::LydianLetterI),
            LYDIAN_LETTER_Y => Ok(Lydian::LydianLetterY),
            LYDIAN_LETTER_K => Ok(Lydian::LydianLetterK),
            LYDIAN_LETTER_L => Ok(Lydian::LydianLetterL),
            LYDIAN_LETTER_M => Ok(Lydian::LydianLetterM),
            LYDIAN_LETTER_N => Ok(Lydian::LydianLetterN),
            LYDIAN_LETTER_O => Ok(Lydian::LydianLetterO),
            LYDIAN_LETTER_R => Ok(Lydian::LydianLetterR),
            LYDIAN_LETTER_SS => Ok(Lydian::LydianLetterSs),
            LYDIAN_LETTER_T => Ok(Lydian::LydianLetterT),
            LYDIAN_LETTER_U => Ok(Lydian::LydianLetterU),
            LYDIAN_LETTER_F => Ok(Lydian::LydianLetterF),
            LYDIAN_LETTER_Q => Ok(Lydian::LydianLetterQ),
            LYDIAN_LETTER_S => Ok(Lydian::LydianLetterS),
            LYDIAN_LETTER_TT => Ok(Lydian::LydianLetterTt),
            LYDIAN_LETTER_AN => Ok(Lydian::LydianLetterAn),
            LYDIAN_LETTER_EN => Ok(Lydian::LydianLetterEn),
            LYDIAN_LETTER_LY => Ok(Lydian::LydianLetterLy),
            LYDIAN_LETTER_NN => Ok(Lydian::LydianLetterNn),
            LYDIAN_LETTER_C => Ok(Lydian::LydianLetterC),
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
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        Lydian::LydianLetterA
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Lydian::LydianLetterA => "lydian letter a",
            Lydian::LydianLetterB => "lydian letter b",
            Lydian::LydianLetterG => "lydian letter g",
            Lydian::LydianLetterD => "lydian letter d",
            Lydian::LydianLetterE => "lydian letter e",
            Lydian::LydianLetterV => "lydian letter v",
            Lydian::LydianLetterI => "lydian letter i",
            Lydian::LydianLetterY => "lydian letter y",
            Lydian::LydianLetterK => "lydian letter k",
            Lydian::LydianLetterL => "lydian letter l",
            Lydian::LydianLetterM => "lydian letter m",
            Lydian::LydianLetterN => "lydian letter n",
            Lydian::LydianLetterO => "lydian letter o",
            Lydian::LydianLetterR => "lydian letter r",
            Lydian::LydianLetterSs => "lydian letter ss",
            Lydian::LydianLetterT => "lydian letter t",
            Lydian::LydianLetterU => "lydian letter u",
            Lydian::LydianLetterF => "lydian letter f",
            Lydian::LydianLetterQ => "lydian letter q",
            Lydian::LydianLetterS => "lydian letter s",
            Lydian::LydianLetterTt => "lydian letter tt",
            Lydian::LydianLetterAn => "lydian letter an",
            Lydian::LydianLetterEn => "lydian letter en",
            Lydian::LydianLetterLy => "lydian letter ly",
            Lydian::LydianLetterNn => "lydian letter nn",
            Lydian::LydianLetterC => "lydian letter c",
        }
    }
}
