/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{10980}: '𐦀'
    pub const MEROITIC_HIEROGLYPHIC_LETTER_A: char = '𐦀';
    /// \u{10981}: '𐦁'
    pub const MEROITIC_HIEROGLYPHIC_LETTER_E: char = '𐦁';
    /// \u{10982}: '𐦂'
    pub const MEROITIC_HIEROGLYPHIC_LETTER_I: char = '𐦂';
    /// \u{10983}: '𐦃'
    pub const MEROITIC_HIEROGLYPHIC_LETTER_O: char = '𐦃';
    /// \u{10984}: '𐦄'
    pub const MEROITIC_HIEROGLYPHIC_LETTER_YA: char = '𐦄';
    /// \u{10985}: '𐦅'
    pub const MEROITIC_HIEROGLYPHIC_LETTER_WA: char = '𐦅';
    /// \u{10986}: '𐦆'
    pub const MEROITIC_HIEROGLYPHIC_LETTER_BA: char = '𐦆';
    /// \u{10987}: '𐦇'
    pub const MEROITIC_HIEROGLYPHIC_LETTER_BA_DASH_2: char = '𐦇';
    /// \u{10988}: '𐦈'
    pub const MEROITIC_HIEROGLYPHIC_LETTER_PA: char = '𐦈';
    /// \u{10989}: '𐦉'
    pub const MEROITIC_HIEROGLYPHIC_LETTER_MA: char = '𐦉';
    /// \u{1098a}: '𐦊'
    pub const MEROITIC_HIEROGLYPHIC_LETTER_NA: char = '𐦊';
    /// \u{1098b}: '𐦋'
    pub const MEROITIC_HIEROGLYPHIC_LETTER_NA_DASH_2: char = '𐦋';
    /// \u{1098c}: '𐦌'
    pub const MEROITIC_HIEROGLYPHIC_LETTER_NE: char = '𐦌';
    /// \u{1098d}: '𐦍'
    pub const MEROITIC_HIEROGLYPHIC_LETTER_NE_DASH_2: char = '𐦍';
    /// \u{1098e}: '𐦎'
    pub const MEROITIC_HIEROGLYPHIC_LETTER_RA: char = '𐦎';
    /// \u{1098f}: '𐦏'
    pub const MEROITIC_HIEROGLYPHIC_LETTER_RA_DASH_2: char = '𐦏';
    /// \u{10990}: '𐦐'
    pub const MEROITIC_HIEROGLYPHIC_LETTER_LA: char = '𐦐';
    /// \u{10991}: '𐦑'
    pub const MEROITIC_HIEROGLYPHIC_LETTER_KHA: char = '𐦑';
    /// \u{10992}: '𐦒'
    pub const MEROITIC_HIEROGLYPHIC_LETTER_HHA: char = '𐦒';
    /// \u{10993}: '𐦓'
    pub const MEROITIC_HIEROGLYPHIC_LETTER_SA: char = '𐦓';
    /// \u{10994}: '𐦔'
    pub const MEROITIC_HIEROGLYPHIC_LETTER_SA_DASH_2: char = '𐦔';
    /// \u{10995}: '𐦕'
    pub const MEROITIC_HIEROGLYPHIC_LETTER_SE: char = '𐦕';
    /// \u{10996}: '𐦖'
    pub const MEROITIC_HIEROGLYPHIC_LETTER_KA: char = '𐦖';
    /// \u{10997}: '𐦗'
    pub const MEROITIC_HIEROGLYPHIC_LETTER_QA: char = '𐦗';
    /// \u{10998}: '𐦘'
    pub const MEROITIC_HIEROGLYPHIC_LETTER_TA: char = '𐦘';
    /// \u{10999}: '𐦙'
    pub const MEROITIC_HIEROGLYPHIC_LETTER_TA_DASH_2: char = '𐦙';
    /// \u{1099a}: '𐦚'
    pub const MEROITIC_HIEROGLYPHIC_LETTER_TE: char = '𐦚';
    /// \u{1099b}: '𐦛'
    pub const MEROITIC_HIEROGLYPHIC_LETTER_TE_DASH_2: char = '𐦛';
    /// \u{1099c}: '𐦜'
    pub const MEROITIC_HIEROGLYPHIC_LETTER_TO: char = '𐦜';
    /// \u{1099d}: '𐦝'
    pub const MEROITIC_HIEROGLYPHIC_LETTER_DA: char = '𐦝';
    /// \u{1099e}: '𐦞'
    pub const MEROITIC_HIEROGLYPHIC_SYMBOL_VIDJ: char = '𐦞';
}

/// An enum to represent all characters in the MeroiticHieroglyphs block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum MeroiticHieroglyphs {
    /// \u{10980}: '𐦀'
    MeroiticHieroglyphicLetterA,
    /// \u{10981}: '𐦁'
    MeroiticHieroglyphicLetterE,
    /// \u{10982}: '𐦂'
    MeroiticHieroglyphicLetterI,
    /// \u{10983}: '𐦃'
    MeroiticHieroglyphicLetterO,
    /// \u{10984}: '𐦄'
    MeroiticHieroglyphicLetterYa,
    /// \u{10985}: '𐦅'
    MeroiticHieroglyphicLetterWa,
    /// \u{10986}: '𐦆'
    MeroiticHieroglyphicLetterBa,
    /// \u{10987}: '𐦇'
    MeroiticHieroglyphicLetterBaDash2,
    /// \u{10988}: '𐦈'
    MeroiticHieroglyphicLetterPa,
    /// \u{10989}: '𐦉'
    MeroiticHieroglyphicLetterMa,
    /// \u{1098a}: '𐦊'
    MeroiticHieroglyphicLetterNa,
    /// \u{1098b}: '𐦋'
    MeroiticHieroglyphicLetterNaDash2,
    /// \u{1098c}: '𐦌'
    MeroiticHieroglyphicLetterNe,
    /// \u{1098d}: '𐦍'
    MeroiticHieroglyphicLetterNeDash2,
    /// \u{1098e}: '𐦎'
    MeroiticHieroglyphicLetterRa,
    /// \u{1098f}: '𐦏'
    MeroiticHieroglyphicLetterRaDash2,
    /// \u{10990}: '𐦐'
    MeroiticHieroglyphicLetterLa,
    /// \u{10991}: '𐦑'
    MeroiticHieroglyphicLetterKha,
    /// \u{10992}: '𐦒'
    MeroiticHieroglyphicLetterHha,
    /// \u{10993}: '𐦓'
    MeroiticHieroglyphicLetterSa,
    /// \u{10994}: '𐦔'
    MeroiticHieroglyphicLetterSaDash2,
    /// \u{10995}: '𐦕'
    MeroiticHieroglyphicLetterSe,
    /// \u{10996}: '𐦖'
    MeroiticHieroglyphicLetterKa,
    /// \u{10997}: '𐦗'
    MeroiticHieroglyphicLetterQa,
    /// \u{10998}: '𐦘'
    MeroiticHieroglyphicLetterTa,
    /// \u{10999}: '𐦙'
    MeroiticHieroglyphicLetterTaDash2,
    /// \u{1099a}: '𐦚'
    MeroiticHieroglyphicLetterTe,
    /// \u{1099b}: '𐦛'
    MeroiticHieroglyphicLetterTeDash2,
    /// \u{1099c}: '𐦜'
    MeroiticHieroglyphicLetterTo,
    /// \u{1099d}: '𐦝'
    MeroiticHieroglyphicLetterDa,
    /// \u{1099e}: '𐦞'
    MeroiticHieroglyphicSymbolVidj,
}

impl Into<char> for MeroiticHieroglyphs {
    fn into(self) -> char {
        use constants::*;
        match self {
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterA => MEROITIC_HIEROGLYPHIC_LETTER_A,
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterE => MEROITIC_HIEROGLYPHIC_LETTER_E,
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterI => MEROITIC_HIEROGLYPHIC_LETTER_I,
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterO => MEROITIC_HIEROGLYPHIC_LETTER_O,
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterYa => MEROITIC_HIEROGLYPHIC_LETTER_YA,
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterWa => MEROITIC_HIEROGLYPHIC_LETTER_WA,
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterBa => MEROITIC_HIEROGLYPHIC_LETTER_BA,
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterBaDash2 => MEROITIC_HIEROGLYPHIC_LETTER_BA_DASH_2,
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterPa => MEROITIC_HIEROGLYPHIC_LETTER_PA,
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterMa => MEROITIC_HIEROGLYPHIC_LETTER_MA,
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterNa => MEROITIC_HIEROGLYPHIC_LETTER_NA,
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterNaDash2 => MEROITIC_HIEROGLYPHIC_LETTER_NA_DASH_2,
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterNe => MEROITIC_HIEROGLYPHIC_LETTER_NE,
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterNeDash2 => MEROITIC_HIEROGLYPHIC_LETTER_NE_DASH_2,
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterRa => MEROITIC_HIEROGLYPHIC_LETTER_RA,
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterRaDash2 => MEROITIC_HIEROGLYPHIC_LETTER_RA_DASH_2,
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterLa => MEROITIC_HIEROGLYPHIC_LETTER_LA,
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterKha => MEROITIC_HIEROGLYPHIC_LETTER_KHA,
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterHha => MEROITIC_HIEROGLYPHIC_LETTER_HHA,
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterSa => MEROITIC_HIEROGLYPHIC_LETTER_SA,
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterSaDash2 => MEROITIC_HIEROGLYPHIC_LETTER_SA_DASH_2,
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterSe => MEROITIC_HIEROGLYPHIC_LETTER_SE,
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterKa => MEROITIC_HIEROGLYPHIC_LETTER_KA,
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterQa => MEROITIC_HIEROGLYPHIC_LETTER_QA,
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterTa => MEROITIC_HIEROGLYPHIC_LETTER_TA,
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterTaDash2 => MEROITIC_HIEROGLYPHIC_LETTER_TA_DASH_2,
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterTe => MEROITIC_HIEROGLYPHIC_LETTER_TE,
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterTeDash2 => MEROITIC_HIEROGLYPHIC_LETTER_TE_DASH_2,
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterTo => MEROITIC_HIEROGLYPHIC_LETTER_TO,
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterDa => MEROITIC_HIEROGLYPHIC_LETTER_DA,
            MeroiticHieroglyphs::MeroiticHieroglyphicSymbolVidj => MEROITIC_HIEROGLYPHIC_SYMBOL_VIDJ,
        }
    }
}

impl std::convert::TryFrom<char> for MeroiticHieroglyphs {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            MEROITIC_HIEROGLYPHIC_LETTER_A => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterA),
            MEROITIC_HIEROGLYPHIC_LETTER_E => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterE),
            MEROITIC_HIEROGLYPHIC_LETTER_I => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterI),
            MEROITIC_HIEROGLYPHIC_LETTER_O => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterO),
            MEROITIC_HIEROGLYPHIC_LETTER_YA => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterYa),
            MEROITIC_HIEROGLYPHIC_LETTER_WA => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterWa),
            MEROITIC_HIEROGLYPHIC_LETTER_BA => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterBa),
            MEROITIC_HIEROGLYPHIC_LETTER_BA_DASH_2 => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterBaDash2),
            MEROITIC_HIEROGLYPHIC_LETTER_PA => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterPa),
            MEROITIC_HIEROGLYPHIC_LETTER_MA => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterMa),
            MEROITIC_HIEROGLYPHIC_LETTER_NA => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterNa),
            MEROITIC_HIEROGLYPHIC_LETTER_NA_DASH_2 => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterNaDash2),
            MEROITIC_HIEROGLYPHIC_LETTER_NE => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterNe),
            MEROITIC_HIEROGLYPHIC_LETTER_NE_DASH_2 => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterNeDash2),
            MEROITIC_HIEROGLYPHIC_LETTER_RA => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterRa),
            MEROITIC_HIEROGLYPHIC_LETTER_RA_DASH_2 => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterRaDash2),
            MEROITIC_HIEROGLYPHIC_LETTER_LA => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterLa),
            MEROITIC_HIEROGLYPHIC_LETTER_KHA => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterKha),
            MEROITIC_HIEROGLYPHIC_LETTER_HHA => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterHha),
            MEROITIC_HIEROGLYPHIC_LETTER_SA => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterSa),
            MEROITIC_HIEROGLYPHIC_LETTER_SA_DASH_2 => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterSaDash2),
            MEROITIC_HIEROGLYPHIC_LETTER_SE => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterSe),
            MEROITIC_HIEROGLYPHIC_LETTER_KA => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterKa),
            MEROITIC_HIEROGLYPHIC_LETTER_QA => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterQa),
            MEROITIC_HIEROGLYPHIC_LETTER_TA => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterTa),
            MEROITIC_HIEROGLYPHIC_LETTER_TA_DASH_2 => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterTaDash2),
            MEROITIC_HIEROGLYPHIC_LETTER_TE => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterTe),
            MEROITIC_HIEROGLYPHIC_LETTER_TE_DASH_2 => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterTeDash2),
            MEROITIC_HIEROGLYPHIC_LETTER_TO => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterTo),
            MEROITIC_HIEROGLYPHIC_LETTER_DA => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterDa),
            MEROITIC_HIEROGLYPHIC_SYMBOL_VIDJ => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicSymbolVidj),
            _ => Err(()),
        }
    }
}

impl Into<u32> for MeroiticHieroglyphs {
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

impl std::convert::TryFrom<u32> for MeroiticHieroglyphs {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for MeroiticHieroglyphs {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl MeroiticHieroglyphs {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        MeroiticHieroglyphs::MeroiticHieroglyphicLetterA
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterA => "meroitic hieroglyphic letter a",
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterE => "meroitic hieroglyphic letter e",
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterI => "meroitic hieroglyphic letter i",
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterO => "meroitic hieroglyphic letter o",
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterYa => "meroitic hieroglyphic letter ya",
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterWa => "meroitic hieroglyphic letter wa",
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterBa => "meroitic hieroglyphic letter ba",
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterBaDash2 => "meroitic hieroglyphic letter ba-2",
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterPa => "meroitic hieroglyphic letter pa",
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterMa => "meroitic hieroglyphic letter ma",
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterNa => "meroitic hieroglyphic letter na",
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterNaDash2 => "meroitic hieroglyphic letter na-2",
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterNe => "meroitic hieroglyphic letter ne",
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterNeDash2 => "meroitic hieroglyphic letter ne-2",
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterRa => "meroitic hieroglyphic letter ra",
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterRaDash2 => "meroitic hieroglyphic letter ra-2",
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterLa => "meroitic hieroglyphic letter la",
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterKha => "meroitic hieroglyphic letter kha",
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterHha => "meroitic hieroglyphic letter hha",
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterSa => "meroitic hieroglyphic letter sa",
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterSaDash2 => "meroitic hieroglyphic letter sa-2",
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterSe => "meroitic hieroglyphic letter se",
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterKa => "meroitic hieroglyphic letter ka",
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterQa => "meroitic hieroglyphic letter qa",
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterTa => "meroitic hieroglyphic letter ta",
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterTaDash2 => "meroitic hieroglyphic letter ta-2",
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterTe => "meroitic hieroglyphic letter te",
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterTeDash2 => "meroitic hieroglyphic letter te-2",
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterTo => "meroitic hieroglyphic letter to",
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterDa => "meroitic hieroglyphic letter da",
            MeroiticHieroglyphs::MeroiticHieroglyphicSymbolVidj => "meroitic hieroglyphic symbol vidj",
        }
    }
}
