/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{31a0}: 'ㆠ'
    pub const BOPOMOFO_LETTER_BU: char = 'ㆠ';
    /// \u{31a1}: 'ㆡ'
    pub const BOPOMOFO_LETTER_ZI: char = 'ㆡ';
    /// \u{31a2}: 'ㆢ'
    pub const BOPOMOFO_LETTER_JI: char = 'ㆢ';
    /// \u{31a3}: 'ㆣ'
    pub const BOPOMOFO_LETTER_GU: char = 'ㆣ';
    /// \u{31a4}: 'ㆤ'
    pub const BOPOMOFO_LETTER_EE: char = 'ㆤ';
    /// \u{31a5}: 'ㆥ'
    pub const BOPOMOFO_LETTER_ENN: char = 'ㆥ';
    /// \u{31a6}: 'ㆦ'
    pub const BOPOMOFO_LETTER_OO: char = 'ㆦ';
    /// \u{31a7}: 'ㆧ'
    pub const BOPOMOFO_LETTER_ONN: char = 'ㆧ';
    /// \u{31a8}: 'ㆨ'
    pub const BOPOMOFO_LETTER_IR: char = 'ㆨ';
    /// \u{31a9}: 'ㆩ'
    pub const BOPOMOFO_LETTER_ANN: char = 'ㆩ';
    /// \u{31aa}: 'ㆪ'
    pub const BOPOMOFO_LETTER_INN: char = 'ㆪ';
    /// \u{31ab}: 'ㆫ'
    pub const BOPOMOFO_LETTER_UNN: char = 'ㆫ';
    /// \u{31ac}: 'ㆬ'
    pub const BOPOMOFO_LETTER_IM: char = 'ㆬ';
    /// \u{31ad}: 'ㆭ'
    pub const BOPOMOFO_LETTER_NGG: char = 'ㆭ';
    /// \u{31ae}: 'ㆮ'
    pub const BOPOMOFO_LETTER_AINN: char = 'ㆮ';
    /// \u{31af}: 'ㆯ'
    pub const BOPOMOFO_LETTER_AUNN: char = 'ㆯ';
    /// \u{31b0}: 'ㆰ'
    pub const BOPOMOFO_LETTER_AM: char = 'ㆰ';
    /// \u{31b1}: 'ㆱ'
    pub const BOPOMOFO_LETTER_OM: char = 'ㆱ';
    /// \u{31b2}: 'ㆲ'
    pub const BOPOMOFO_LETTER_ONG: char = 'ㆲ';
    /// \u{31b3}: 'ㆳ'
    pub const BOPOMOFO_LETTER_INNN: char = 'ㆳ';
    /// \u{31b4}: 'ㆴ'
    pub const BOPOMOFO_FINAL_LETTER_P: char = 'ㆴ';
    /// \u{31b5}: 'ㆵ'
    pub const BOPOMOFO_FINAL_LETTER_T: char = 'ㆵ';
    /// \u{31b6}: 'ㆶ'
    pub const BOPOMOFO_FINAL_LETTER_K: char = 'ㆶ';
    /// \u{31b7}: 'ㆷ'
    pub const BOPOMOFO_FINAL_LETTER_H: char = 'ㆷ';
    /// \u{31b8}: 'ㆸ'
    pub const BOPOMOFO_LETTER_GH: char = 'ㆸ';
    /// \u{31b9}: 'ㆹ'
    pub const BOPOMOFO_LETTER_LH: char = 'ㆹ';
    /// \u{31ba}: 'ㆺ'
    pub const BOPOMOFO_LETTER_ZY: char = 'ㆺ';
}

/// An enum to represent all characters in the BopomofoExtended block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum BopomofoExtended {
    /// \u{31a0}: 'ㆠ'
    BopomofoLetterBu,
    /// \u{31a1}: 'ㆡ'
    BopomofoLetterZi,
    /// \u{31a2}: 'ㆢ'
    BopomofoLetterJi,
    /// \u{31a3}: 'ㆣ'
    BopomofoLetterGu,
    /// \u{31a4}: 'ㆤ'
    BopomofoLetterEe,
    /// \u{31a5}: 'ㆥ'
    BopomofoLetterEnn,
    /// \u{31a6}: 'ㆦ'
    BopomofoLetterOo,
    /// \u{31a7}: 'ㆧ'
    BopomofoLetterOnn,
    /// \u{31a8}: 'ㆨ'
    BopomofoLetterIr,
    /// \u{31a9}: 'ㆩ'
    BopomofoLetterAnn,
    /// \u{31aa}: 'ㆪ'
    BopomofoLetterInn,
    /// \u{31ab}: 'ㆫ'
    BopomofoLetterUnn,
    /// \u{31ac}: 'ㆬ'
    BopomofoLetterIm,
    /// \u{31ad}: 'ㆭ'
    BopomofoLetterNgg,
    /// \u{31ae}: 'ㆮ'
    BopomofoLetterAinn,
    /// \u{31af}: 'ㆯ'
    BopomofoLetterAunn,
    /// \u{31b0}: 'ㆰ'
    BopomofoLetterAm,
    /// \u{31b1}: 'ㆱ'
    BopomofoLetterOm,
    /// \u{31b2}: 'ㆲ'
    BopomofoLetterOng,
    /// \u{31b3}: 'ㆳ'
    BopomofoLetterInnn,
    /// \u{31b4}: 'ㆴ'
    BopomofoFinalLetterP,
    /// \u{31b5}: 'ㆵ'
    BopomofoFinalLetterT,
    /// \u{31b6}: 'ㆶ'
    BopomofoFinalLetterK,
    /// \u{31b7}: 'ㆷ'
    BopomofoFinalLetterH,
    /// \u{31b8}: 'ㆸ'
    BopomofoLetterGh,
    /// \u{31b9}: 'ㆹ'
    BopomofoLetterLh,
    /// \u{31ba}: 'ㆺ'
    BopomofoLetterZy,
}

impl Into<char> for BopomofoExtended {
    fn into(self) -> char {
        use constants::*;
        match self {
            BopomofoExtended::BopomofoLetterBu => BOPOMOFO_LETTER_BU,
            BopomofoExtended::BopomofoLetterZi => BOPOMOFO_LETTER_ZI,
            BopomofoExtended::BopomofoLetterJi => BOPOMOFO_LETTER_JI,
            BopomofoExtended::BopomofoLetterGu => BOPOMOFO_LETTER_GU,
            BopomofoExtended::BopomofoLetterEe => BOPOMOFO_LETTER_EE,
            BopomofoExtended::BopomofoLetterEnn => BOPOMOFO_LETTER_ENN,
            BopomofoExtended::BopomofoLetterOo => BOPOMOFO_LETTER_OO,
            BopomofoExtended::BopomofoLetterOnn => BOPOMOFO_LETTER_ONN,
            BopomofoExtended::BopomofoLetterIr => BOPOMOFO_LETTER_IR,
            BopomofoExtended::BopomofoLetterAnn => BOPOMOFO_LETTER_ANN,
            BopomofoExtended::BopomofoLetterInn => BOPOMOFO_LETTER_INN,
            BopomofoExtended::BopomofoLetterUnn => BOPOMOFO_LETTER_UNN,
            BopomofoExtended::BopomofoLetterIm => BOPOMOFO_LETTER_IM,
            BopomofoExtended::BopomofoLetterNgg => BOPOMOFO_LETTER_NGG,
            BopomofoExtended::BopomofoLetterAinn => BOPOMOFO_LETTER_AINN,
            BopomofoExtended::BopomofoLetterAunn => BOPOMOFO_LETTER_AUNN,
            BopomofoExtended::BopomofoLetterAm => BOPOMOFO_LETTER_AM,
            BopomofoExtended::BopomofoLetterOm => BOPOMOFO_LETTER_OM,
            BopomofoExtended::BopomofoLetterOng => BOPOMOFO_LETTER_ONG,
            BopomofoExtended::BopomofoLetterInnn => BOPOMOFO_LETTER_INNN,
            BopomofoExtended::BopomofoFinalLetterP => BOPOMOFO_FINAL_LETTER_P,
            BopomofoExtended::BopomofoFinalLetterT => BOPOMOFO_FINAL_LETTER_T,
            BopomofoExtended::BopomofoFinalLetterK => BOPOMOFO_FINAL_LETTER_K,
            BopomofoExtended::BopomofoFinalLetterH => BOPOMOFO_FINAL_LETTER_H,
            BopomofoExtended::BopomofoLetterGh => BOPOMOFO_LETTER_GH,
            BopomofoExtended::BopomofoLetterLh => BOPOMOFO_LETTER_LH,
            BopomofoExtended::BopomofoLetterZy => BOPOMOFO_LETTER_ZY,
        }
    }
}

impl std::convert::TryFrom<char> for BopomofoExtended {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            BOPOMOFO_LETTER_BU => Ok(BopomofoExtended::BopomofoLetterBu),
            BOPOMOFO_LETTER_ZI => Ok(BopomofoExtended::BopomofoLetterZi),
            BOPOMOFO_LETTER_JI => Ok(BopomofoExtended::BopomofoLetterJi),
            BOPOMOFO_LETTER_GU => Ok(BopomofoExtended::BopomofoLetterGu),
            BOPOMOFO_LETTER_EE => Ok(BopomofoExtended::BopomofoLetterEe),
            BOPOMOFO_LETTER_ENN => Ok(BopomofoExtended::BopomofoLetterEnn),
            BOPOMOFO_LETTER_OO => Ok(BopomofoExtended::BopomofoLetterOo),
            BOPOMOFO_LETTER_ONN => Ok(BopomofoExtended::BopomofoLetterOnn),
            BOPOMOFO_LETTER_IR => Ok(BopomofoExtended::BopomofoLetterIr),
            BOPOMOFO_LETTER_ANN => Ok(BopomofoExtended::BopomofoLetterAnn),
            BOPOMOFO_LETTER_INN => Ok(BopomofoExtended::BopomofoLetterInn),
            BOPOMOFO_LETTER_UNN => Ok(BopomofoExtended::BopomofoLetterUnn),
            BOPOMOFO_LETTER_IM => Ok(BopomofoExtended::BopomofoLetterIm),
            BOPOMOFO_LETTER_NGG => Ok(BopomofoExtended::BopomofoLetterNgg),
            BOPOMOFO_LETTER_AINN => Ok(BopomofoExtended::BopomofoLetterAinn),
            BOPOMOFO_LETTER_AUNN => Ok(BopomofoExtended::BopomofoLetterAunn),
            BOPOMOFO_LETTER_AM => Ok(BopomofoExtended::BopomofoLetterAm),
            BOPOMOFO_LETTER_OM => Ok(BopomofoExtended::BopomofoLetterOm),
            BOPOMOFO_LETTER_ONG => Ok(BopomofoExtended::BopomofoLetterOng),
            BOPOMOFO_LETTER_INNN => Ok(BopomofoExtended::BopomofoLetterInnn),
            BOPOMOFO_FINAL_LETTER_P => Ok(BopomofoExtended::BopomofoFinalLetterP),
            BOPOMOFO_FINAL_LETTER_T => Ok(BopomofoExtended::BopomofoFinalLetterT),
            BOPOMOFO_FINAL_LETTER_K => Ok(BopomofoExtended::BopomofoFinalLetterK),
            BOPOMOFO_FINAL_LETTER_H => Ok(BopomofoExtended::BopomofoFinalLetterH),
            BOPOMOFO_LETTER_GH => Ok(BopomofoExtended::BopomofoLetterGh),
            BOPOMOFO_LETTER_LH => Ok(BopomofoExtended::BopomofoLetterLh),
            BOPOMOFO_LETTER_ZY => Ok(BopomofoExtended::BopomofoLetterZy),
            _ => Err(()),
        }
    }
}

impl Into<u32> for BopomofoExtended {
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

impl std::convert::TryFrom<u32> for BopomofoExtended {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for BopomofoExtended {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl BopomofoExtended {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        BopomofoExtended::BopomofoLetterBu
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            BopomofoExtended::BopomofoLetterBu => "bopomofo letter bu",
            BopomofoExtended::BopomofoLetterZi => "bopomofo letter zi",
            BopomofoExtended::BopomofoLetterJi => "bopomofo letter ji",
            BopomofoExtended::BopomofoLetterGu => "bopomofo letter gu",
            BopomofoExtended::BopomofoLetterEe => "bopomofo letter ee",
            BopomofoExtended::BopomofoLetterEnn => "bopomofo letter enn",
            BopomofoExtended::BopomofoLetterOo => "bopomofo letter oo",
            BopomofoExtended::BopomofoLetterOnn => "bopomofo letter onn",
            BopomofoExtended::BopomofoLetterIr => "bopomofo letter ir",
            BopomofoExtended::BopomofoLetterAnn => "bopomofo letter ann",
            BopomofoExtended::BopomofoLetterInn => "bopomofo letter inn",
            BopomofoExtended::BopomofoLetterUnn => "bopomofo letter unn",
            BopomofoExtended::BopomofoLetterIm => "bopomofo letter im",
            BopomofoExtended::BopomofoLetterNgg => "bopomofo letter ngg",
            BopomofoExtended::BopomofoLetterAinn => "bopomofo letter ainn",
            BopomofoExtended::BopomofoLetterAunn => "bopomofo letter aunn",
            BopomofoExtended::BopomofoLetterAm => "bopomofo letter am",
            BopomofoExtended::BopomofoLetterOm => "bopomofo letter om",
            BopomofoExtended::BopomofoLetterOng => "bopomofo letter ong",
            BopomofoExtended::BopomofoLetterInnn => "bopomofo letter innn",
            BopomofoExtended::BopomofoFinalLetterP => "bopomofo final letter p",
            BopomofoExtended::BopomofoFinalLetterT => "bopomofo final letter t",
            BopomofoExtended::BopomofoFinalLetterK => "bopomofo final letter k",
            BopomofoExtended::BopomofoFinalLetterH => "bopomofo final letter h",
            BopomofoExtended::BopomofoLetterGh => "bopomofo letter gh",
            BopomofoExtended::BopomofoLetterLh => "bopomofo letter lh",
            BopomofoExtended::BopomofoLetterZy => "bopomofo letter zy",
        }
    }
}
