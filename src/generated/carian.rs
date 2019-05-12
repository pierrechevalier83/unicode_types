/// A number of constants to give a name to all characters in this block.
mod constants {
    /// \u{102a0}: '𐊠'
    pub const LETTER_A: char = '𐊠';
    /// \u{102a1}: '𐊡'
    pub const LETTER_P2: char = '𐊡';
    /// \u{102a2}: '𐊢'
    pub const LETTER_D: char = '𐊢';
    /// \u{102a3}: '𐊣'
    pub const LETTER_L: char = '𐊣';
    /// \u{102a4}: '𐊤'
    pub const LETTER_UUU: char = '𐊤';
    /// \u{102a5}: '𐊥'
    pub const LETTER_R: char = '𐊥';
    /// \u{102a6}: '𐊦'
    pub const LETTER_LD: char = '𐊦';
    /// \u{102a7}: '𐊧'
    pub const LETTER_A2: char = '𐊧';
    /// \u{102a8}: '𐊨'
    pub const LETTER_Q: char = '𐊨';
    /// \u{102a9}: '𐊩'
    pub const LETTER_B: char = '𐊩';
    /// \u{102aa}: '𐊪'
    pub const LETTER_M: char = '𐊪';
    /// \u{102ab}: '𐊫'
    pub const LETTER_O: char = '𐊫';
    /// \u{102ac}: '𐊬'
    pub const LETTER_D2: char = '𐊬';
    /// \u{102ad}: '𐊭'
    pub const LETTER_T: char = '𐊭';
    /// \u{102ae}: '𐊮'
    pub const LETTER_SH: char = '𐊮';
    /// \u{102af}: '𐊯'
    pub const LETTER_SH2: char = '𐊯';
    /// \u{102b0}: '𐊰'
    pub const LETTER_S: char = '𐊰';
    /// \u{102b1}: '𐊱'
    pub const LETTER_C_DASH_18: char = '𐊱';
    /// \u{102b2}: '𐊲'
    pub const LETTER_U: char = '𐊲';
    /// \u{102b3}: '𐊳'
    pub const LETTER_NN: char = '𐊳';
    /// \u{102b4}: '𐊴'
    pub const LETTER_X: char = '𐊴';
    /// \u{102b5}: '𐊵'
    pub const LETTER_N: char = '𐊵';
    /// \u{102b6}: '𐊶'
    pub const LETTER_TT2: char = '𐊶';
    /// \u{102b7}: '𐊷'
    pub const LETTER_P: char = '𐊷';
    /// \u{102b8}: '𐊸'
    pub const LETTER_SS: char = '𐊸';
    /// \u{102b9}: '𐊹'
    pub const LETTER_I: char = '𐊹';
    /// \u{102ba}: '𐊺'
    pub const LETTER_E: char = '𐊺';
    /// \u{102bb}: '𐊻'
    pub const LETTER_UUUU: char = '𐊻';
    /// \u{102bc}: '𐊼'
    pub const LETTER_K: char = '𐊼';
    /// \u{102bd}: '𐊽'
    pub const LETTER_K2: char = '𐊽';
    /// \u{102be}: '𐊾'
    pub const LETTER_ND: char = '𐊾';
    /// \u{102bf}: '𐊿'
    pub const LETTER_UU: char = '𐊿';
    /// \u{102c0}: '𐋀'
    pub const LETTER_G: char = '𐋀';
    /// \u{102c1}: '𐋁'
    pub const LETTER_G2: char = '𐋁';
    /// \u{102c2}: '𐋂'
    pub const LETTER_ST: char = '𐋂';
    /// \u{102c3}: '𐋃'
    pub const LETTER_ST2: char = '𐋃';
    /// \u{102c4}: '𐋄'
    pub const LETTER_NG: char = '𐋄';
    /// \u{102c5}: '𐋅'
    pub const LETTER_II: char = '𐋅';
    /// \u{102c6}: '𐋆'
    pub const LETTER_C_DASH_39: char = '𐋆';
    /// \u{102c7}: '𐋇'
    pub const LETTER_TT: char = '𐋇';
    /// \u{102c8}: '𐋈'
    pub const LETTER_UUU2: char = '𐋈';
    /// \u{102c9}: '𐋉'
    pub const LETTER_RR: char = '𐋉';
    /// \u{102ca}: '𐋊'
    pub const LETTER_MB: char = '𐋊';
    /// \u{102cb}: '𐋋'
    pub const LETTER_MB2: char = '𐋋';
    /// \u{102cc}: '𐋌'
    pub const LETTER_MB3: char = '𐋌';
    /// \u{102cd}: '𐋍'
    pub const LETTER_MB4: char = '𐋍';
    /// \u{102ce}: '𐋎'
    pub const LETTER_LD2: char = '𐋎';
    /// \u{102cf}: '𐋏'
    pub const LETTER_E2: char = '𐋏';
    /// \u{102d0}: '𐋐'
    pub const LETTER_UUU3: char = '𐋐';
}

/// An enum to represent all characters in the Carian block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Carian {
    /// \u{102a0}: '𐊠'
    LetterA,
    /// \u{102a1}: '𐊡'
    LetterP2,
    /// \u{102a2}: '𐊢'
    LetterD,
    /// \u{102a3}: '𐊣'
    LetterL,
    /// \u{102a4}: '𐊤'
    LetterUuu,
    /// \u{102a5}: '𐊥'
    LetterR,
    /// \u{102a6}: '𐊦'
    LetterLd,
    /// \u{102a7}: '𐊧'
    LetterA2,
    /// \u{102a8}: '𐊨'
    LetterQ,
    /// \u{102a9}: '𐊩'
    LetterB,
    /// \u{102aa}: '𐊪'
    LetterM,
    /// \u{102ab}: '𐊫'
    LetterO,
    /// \u{102ac}: '𐊬'
    LetterD2,
    /// \u{102ad}: '𐊭'
    LetterT,
    /// \u{102ae}: '𐊮'
    LetterSh,
    /// \u{102af}: '𐊯'
    LetterSh2,
    /// \u{102b0}: '𐊰'
    LetterS,
    /// \u{102b1}: '𐊱'
    LetterCDash18,
    /// \u{102b2}: '𐊲'
    LetterU,
    /// \u{102b3}: '𐊳'
    LetterNn,
    /// \u{102b4}: '𐊴'
    LetterX,
    /// \u{102b5}: '𐊵'
    LetterN,
    /// \u{102b6}: '𐊶'
    LetterTt2,
    /// \u{102b7}: '𐊷'
    LetterP,
    /// \u{102b8}: '𐊸'
    LetterSs,
    /// \u{102b9}: '𐊹'
    LetterI,
    /// \u{102ba}: '𐊺'
    LetterE,
    /// \u{102bb}: '𐊻'
    LetterUuuu,
    /// \u{102bc}: '𐊼'
    LetterK,
    /// \u{102bd}: '𐊽'
    LetterK2,
    /// \u{102be}: '𐊾'
    LetterNd,
    /// \u{102bf}: '𐊿'
    LetterUu,
    /// \u{102c0}: '𐋀'
    LetterG,
    /// \u{102c1}: '𐋁'
    LetterG2,
    /// \u{102c2}: '𐋂'
    LetterSt,
    /// \u{102c3}: '𐋃'
    LetterSt2,
    /// \u{102c4}: '𐋄'
    LetterNg,
    /// \u{102c5}: '𐋅'
    LetterIi,
    /// \u{102c6}: '𐋆'
    LetterCDash39,
    /// \u{102c7}: '𐋇'
    LetterTt,
    /// \u{102c8}: '𐋈'
    LetterUuu2,
    /// \u{102c9}: '𐋉'
    LetterRr,
    /// \u{102ca}: '𐋊'
    LetterMb,
    /// \u{102cb}: '𐋋'
    LetterMb2,
    /// \u{102cc}: '𐋌'
    LetterMb3,
    /// \u{102cd}: '𐋍'
    LetterMb4,
    /// \u{102ce}: '𐋎'
    LetterLd2,
    /// \u{102cf}: '𐋏'
    LetterE2,
    /// \u{102d0}: '𐋐'
    LetterUuu3,
}

impl Into<char> for Carian {
    fn into(self) -> char {
        use constants::*;
        match self {
            Carian::LetterA => LETTER_A,
            Carian::LetterP2 => LETTER_P2,
            Carian::LetterD => LETTER_D,
            Carian::LetterL => LETTER_L,
            Carian::LetterUuu => LETTER_UUU,
            Carian::LetterR => LETTER_R,
            Carian::LetterLd => LETTER_LD,
            Carian::LetterA2 => LETTER_A2,
            Carian::LetterQ => LETTER_Q,
            Carian::LetterB => LETTER_B,
            Carian::LetterM => LETTER_M,
            Carian::LetterO => LETTER_O,
            Carian::LetterD2 => LETTER_D2,
            Carian::LetterT => LETTER_T,
            Carian::LetterSh => LETTER_SH,
            Carian::LetterSh2 => LETTER_SH2,
            Carian::LetterS => LETTER_S,
            Carian::LetterCDash18 => LETTER_C_DASH_18,
            Carian::LetterU => LETTER_U,
            Carian::LetterNn => LETTER_NN,
            Carian::LetterX => LETTER_X,
            Carian::LetterN => LETTER_N,
            Carian::LetterTt2 => LETTER_TT2,
            Carian::LetterP => LETTER_P,
            Carian::LetterSs => LETTER_SS,
            Carian::LetterI => LETTER_I,
            Carian::LetterE => LETTER_E,
            Carian::LetterUuuu => LETTER_UUUU,
            Carian::LetterK => LETTER_K,
            Carian::LetterK2 => LETTER_K2,
            Carian::LetterNd => LETTER_ND,
            Carian::LetterUu => LETTER_UU,
            Carian::LetterG => LETTER_G,
            Carian::LetterG2 => LETTER_G2,
            Carian::LetterSt => LETTER_ST,
            Carian::LetterSt2 => LETTER_ST2,
            Carian::LetterNg => LETTER_NG,
            Carian::LetterIi => LETTER_II,
            Carian::LetterCDash39 => LETTER_C_DASH_39,
            Carian::LetterTt => LETTER_TT,
            Carian::LetterUuu2 => LETTER_UUU2,
            Carian::LetterRr => LETTER_RR,
            Carian::LetterMb => LETTER_MB,
            Carian::LetterMb2 => LETTER_MB2,
            Carian::LetterMb3 => LETTER_MB3,
            Carian::LetterMb4 => LETTER_MB4,
            Carian::LetterLd2 => LETTER_LD2,
            Carian::LetterE2 => LETTER_E2,
            Carian::LetterUuu3 => LETTER_UUU3,
        }
    }
}

impl std::convert::TryFrom<char> for Carian {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_A => Ok(Carian::LetterA),
            LETTER_P2 => Ok(Carian::LetterP2),
            LETTER_D => Ok(Carian::LetterD),
            LETTER_L => Ok(Carian::LetterL),
            LETTER_UUU => Ok(Carian::LetterUuu),
            LETTER_R => Ok(Carian::LetterR),
            LETTER_LD => Ok(Carian::LetterLd),
            LETTER_A2 => Ok(Carian::LetterA2),
            LETTER_Q => Ok(Carian::LetterQ),
            LETTER_B => Ok(Carian::LetterB),
            LETTER_M => Ok(Carian::LetterM),
            LETTER_O => Ok(Carian::LetterO),
            LETTER_D2 => Ok(Carian::LetterD2),
            LETTER_T => Ok(Carian::LetterT),
            LETTER_SH => Ok(Carian::LetterSh),
            LETTER_SH2 => Ok(Carian::LetterSh2),
            LETTER_S => Ok(Carian::LetterS),
            LETTER_C_DASH_18 => Ok(Carian::LetterCDash18),
            LETTER_U => Ok(Carian::LetterU),
            LETTER_NN => Ok(Carian::LetterNn),
            LETTER_X => Ok(Carian::LetterX),
            LETTER_N => Ok(Carian::LetterN),
            LETTER_TT2 => Ok(Carian::LetterTt2),
            LETTER_P => Ok(Carian::LetterP),
            LETTER_SS => Ok(Carian::LetterSs),
            LETTER_I => Ok(Carian::LetterI),
            LETTER_E => Ok(Carian::LetterE),
            LETTER_UUUU => Ok(Carian::LetterUuuu),
            LETTER_K => Ok(Carian::LetterK),
            LETTER_K2 => Ok(Carian::LetterK2),
            LETTER_ND => Ok(Carian::LetterNd),
            LETTER_UU => Ok(Carian::LetterUu),
            LETTER_G => Ok(Carian::LetterG),
            LETTER_G2 => Ok(Carian::LetterG2),
            LETTER_ST => Ok(Carian::LetterSt),
            LETTER_ST2 => Ok(Carian::LetterSt2),
            LETTER_NG => Ok(Carian::LetterNg),
            LETTER_II => Ok(Carian::LetterIi),
            LETTER_C_DASH_39 => Ok(Carian::LetterCDash39),
            LETTER_TT => Ok(Carian::LetterTt),
            LETTER_UUU2 => Ok(Carian::LetterUuu2),
            LETTER_RR => Ok(Carian::LetterRr),
            LETTER_MB => Ok(Carian::LetterMb),
            LETTER_MB2 => Ok(Carian::LetterMb2),
            LETTER_MB3 => Ok(Carian::LetterMb3),
            LETTER_MB4 => Ok(Carian::LetterMb4),
            LETTER_LD2 => Ok(Carian::LetterLd2),
            LETTER_E2 => Ok(Carian::LetterE2),
            LETTER_UUU3 => Ok(Carian::LetterUuu3),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Carian {
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

impl std::convert::TryFrom<u32> for Carian {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Carian {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Carian {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Carian::LetterA
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Carian::LetterA => "carian letter a",
            Carian::LetterP2 => "carian letter p2",
            Carian::LetterD => "carian letter d",
            Carian::LetterL => "carian letter l",
            Carian::LetterUuu => "carian letter uuu",
            Carian::LetterR => "carian letter r",
            Carian::LetterLd => "carian letter ld",
            Carian::LetterA2 => "carian letter a2",
            Carian::LetterQ => "carian letter q",
            Carian::LetterB => "carian letter b",
            Carian::LetterM => "carian letter m",
            Carian::LetterO => "carian letter o",
            Carian::LetterD2 => "carian letter d2",
            Carian::LetterT => "carian letter t",
            Carian::LetterSh => "carian letter sh",
            Carian::LetterSh2 => "carian letter sh2",
            Carian::LetterS => "carian letter s",
            Carian::LetterCDash18 => "carian letter c-18",
            Carian::LetterU => "carian letter u",
            Carian::LetterNn => "carian letter nn",
            Carian::LetterX => "carian letter x",
            Carian::LetterN => "carian letter n",
            Carian::LetterTt2 => "carian letter tt2",
            Carian::LetterP => "carian letter p",
            Carian::LetterSs => "carian letter ss",
            Carian::LetterI => "carian letter i",
            Carian::LetterE => "carian letter e",
            Carian::LetterUuuu => "carian letter uuuu",
            Carian::LetterK => "carian letter k",
            Carian::LetterK2 => "carian letter k2",
            Carian::LetterNd => "carian letter nd",
            Carian::LetterUu => "carian letter uu",
            Carian::LetterG => "carian letter g",
            Carian::LetterG2 => "carian letter g2",
            Carian::LetterSt => "carian letter st",
            Carian::LetterSt2 => "carian letter st2",
            Carian::LetterNg => "carian letter ng",
            Carian::LetterIi => "carian letter ii",
            Carian::LetterCDash39 => "carian letter c-39",
            Carian::LetterTt => "carian letter tt",
            Carian::LetterUuu2 => "carian letter uuu2",
            Carian::LetterRr => "carian letter rr",
            Carian::LetterMb => "carian letter mb",
            Carian::LetterMb2 => "carian letter mb2",
            Carian::LetterMb3 => "carian letter mb3",
            Carian::LetterMb4 => "carian letter mb4",
            Carian::LetterLd2 => "carian letter ld2",
            Carian::LetterE2 => "carian letter e2",
            Carian::LetterUuu3 => "carian letter uuu3",
        }
    }
}
