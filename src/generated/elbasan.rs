/// \u{10500} → \u{1052f}\
///\
/// 𐔀 𐔁 𐔂 𐔃 𐔄 𐔅 𐔆 𐔇 𐔈 𐔉 𐔊 𐔋 𐔌 𐔍 𐔎 𐔏\
/// 𐔐 𐔑 𐔒 𐔓 𐔔 𐔕 𐔖 𐔗 𐔘 𐔙 𐔚 𐔛 𐔜 𐔝 𐔞 𐔟\
/// 𐔠 𐔡 𐔢 𐔣 𐔤 𐔥 𐔦 𐔧\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{10500}: '𐔀'
    pub const LETTER_A: char = '𐔀';
    /// \u{10501}: '𐔁'
    pub const LETTER_BE: char = '𐔁';
    /// \u{10502}: '𐔂'
    pub const LETTER_CE: char = '𐔂';
    /// \u{10503}: '𐔃'
    pub const LETTER_CHE: char = '𐔃';
    /// \u{10504}: '𐔄'
    pub const LETTER_DE: char = '𐔄';
    /// \u{10505}: '𐔅'
    pub const LETTER_NDE: char = '𐔅';
    /// \u{10506}: '𐔆'
    pub const LETTER_DHE: char = '𐔆';
    /// \u{10507}: '𐔇'
    pub const LETTER_EI: char = '𐔇';
    /// \u{10508}: '𐔈'
    pub const LETTER_E: char = '𐔈';
    /// \u{10509}: '𐔉'
    pub const LETTER_FE: char = '𐔉';
    /// \u{1050a}: '𐔊'
    pub const LETTER_GE: char = '𐔊';
    /// \u{1050b}: '𐔋'
    pub const LETTER_GJE: char = '𐔋';
    /// \u{1050c}: '𐔌'
    pub const LETTER_HE: char = '𐔌';
    /// \u{1050d}: '𐔍'
    pub const LETTER_I: char = '𐔍';
    /// \u{1050e}: '𐔎'
    pub const LETTER_JE: char = '𐔎';
    /// \u{1050f}: '𐔏'
    pub const LETTER_KE: char = '𐔏';
    /// \u{10510}: '𐔐'
    pub const LETTER_LE: char = '𐔐';
    /// \u{10511}: '𐔑'
    pub const LETTER_LLE: char = '𐔑';
    /// \u{10512}: '𐔒'
    pub const LETTER_ME: char = '𐔒';
    /// \u{10513}: '𐔓'
    pub const LETTER_NE: char = '𐔓';
    /// \u{10514}: '𐔔'
    pub const LETTER_NA: char = '𐔔';
    /// \u{10515}: '𐔕'
    pub const LETTER_NJE: char = '𐔕';
    /// \u{10516}: '𐔖'
    pub const LETTER_O: char = '𐔖';
    /// \u{10517}: '𐔗'
    pub const LETTER_PE: char = '𐔗';
    /// \u{10518}: '𐔘'
    pub const LETTER_QE: char = '𐔘';
    /// \u{10519}: '𐔙'
    pub const LETTER_RE: char = '𐔙';
    /// \u{1051a}: '𐔚'
    pub const LETTER_RRE: char = '𐔚';
    /// \u{1051b}: '𐔛'
    pub const LETTER_SE: char = '𐔛';
    /// \u{1051c}: '𐔜'
    pub const LETTER_SHE: char = '𐔜';
    /// \u{1051d}: '𐔝'
    pub const LETTER_TE: char = '𐔝';
    /// \u{1051e}: '𐔞'
    pub const LETTER_THE: char = '𐔞';
    /// \u{1051f}: '𐔟'
    pub const LETTER_U: char = '𐔟';
    /// \u{10520}: '𐔠'
    pub const LETTER_VE: char = '𐔠';
    /// \u{10521}: '𐔡'
    pub const LETTER_XE: char = '𐔡';
    /// \u{10522}: '𐔢'
    pub const LETTER_Y: char = '𐔢';
    /// \u{10523}: '𐔣'
    pub const LETTER_ZE: char = '𐔣';
    /// \u{10524}: '𐔤'
    pub const LETTER_ZHE: char = '𐔤';
    /// \u{10525}: '𐔥'
    pub const LETTER_GHE: char = '𐔥';
    /// \u{10526}: '𐔦'
    pub const LETTER_GHAMMA: char = '𐔦';
    /// \u{10527}: '𐔧'
    pub const LETTER_KHE: char = '𐔧';
}

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
        use constants::*;
        match self {
            Elbasan::LetterA => LETTER_A,
            Elbasan::LetterBe => LETTER_BE,
            Elbasan::LetterCe => LETTER_CE,
            Elbasan::LetterChe => LETTER_CHE,
            Elbasan::LetterDe => LETTER_DE,
            Elbasan::LetterNde => LETTER_NDE,
            Elbasan::LetterDhe => LETTER_DHE,
            Elbasan::LetterEi => LETTER_EI,
            Elbasan::LetterE => LETTER_E,
            Elbasan::LetterFe => LETTER_FE,
            Elbasan::LetterGe => LETTER_GE,
            Elbasan::LetterGje => LETTER_GJE,
            Elbasan::LetterHe => LETTER_HE,
            Elbasan::LetterI => LETTER_I,
            Elbasan::LetterJe => LETTER_JE,
            Elbasan::LetterKe => LETTER_KE,
            Elbasan::LetterLe => LETTER_LE,
            Elbasan::LetterLle => LETTER_LLE,
            Elbasan::LetterMe => LETTER_ME,
            Elbasan::LetterNe => LETTER_NE,
            Elbasan::LetterNa => LETTER_NA,
            Elbasan::LetterNje => LETTER_NJE,
            Elbasan::LetterO => LETTER_O,
            Elbasan::LetterPe => LETTER_PE,
            Elbasan::LetterQe => LETTER_QE,
            Elbasan::LetterRe => LETTER_RE,
            Elbasan::LetterRre => LETTER_RRE,
            Elbasan::LetterSe => LETTER_SE,
            Elbasan::LetterShe => LETTER_SHE,
            Elbasan::LetterTe => LETTER_TE,
            Elbasan::LetterThe => LETTER_THE,
            Elbasan::LetterU => LETTER_U,
            Elbasan::LetterVe => LETTER_VE,
            Elbasan::LetterXe => LETTER_XE,
            Elbasan::LetterY => LETTER_Y,
            Elbasan::LetterZe => LETTER_ZE,
            Elbasan::LetterZhe => LETTER_ZHE,
            Elbasan::LetterGhe => LETTER_GHE,
            Elbasan::LetterGhamma => LETTER_GHAMMA,
            Elbasan::LetterKhe => LETTER_KHE,
        }
    }
}

impl std::convert::TryFrom<char> for Elbasan {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_A => Ok(Elbasan::LetterA),
            LETTER_BE => Ok(Elbasan::LetterBe),
            LETTER_CE => Ok(Elbasan::LetterCe),
            LETTER_CHE => Ok(Elbasan::LetterChe),
            LETTER_DE => Ok(Elbasan::LetterDe),
            LETTER_NDE => Ok(Elbasan::LetterNde),
            LETTER_DHE => Ok(Elbasan::LetterDhe),
            LETTER_EI => Ok(Elbasan::LetterEi),
            LETTER_E => Ok(Elbasan::LetterE),
            LETTER_FE => Ok(Elbasan::LetterFe),
            LETTER_GE => Ok(Elbasan::LetterGe),
            LETTER_GJE => Ok(Elbasan::LetterGje),
            LETTER_HE => Ok(Elbasan::LetterHe),
            LETTER_I => Ok(Elbasan::LetterI),
            LETTER_JE => Ok(Elbasan::LetterJe),
            LETTER_KE => Ok(Elbasan::LetterKe),
            LETTER_LE => Ok(Elbasan::LetterLe),
            LETTER_LLE => Ok(Elbasan::LetterLle),
            LETTER_ME => Ok(Elbasan::LetterMe),
            LETTER_NE => Ok(Elbasan::LetterNe),
            LETTER_NA => Ok(Elbasan::LetterNa),
            LETTER_NJE => Ok(Elbasan::LetterNje),
            LETTER_O => Ok(Elbasan::LetterO),
            LETTER_PE => Ok(Elbasan::LetterPe),
            LETTER_QE => Ok(Elbasan::LetterQe),
            LETTER_RE => Ok(Elbasan::LetterRe),
            LETTER_RRE => Ok(Elbasan::LetterRre),
            LETTER_SE => Ok(Elbasan::LetterSe),
            LETTER_SHE => Ok(Elbasan::LetterShe),
            LETTER_TE => Ok(Elbasan::LetterTe),
            LETTER_THE => Ok(Elbasan::LetterThe),
            LETTER_U => Ok(Elbasan::LetterU),
            LETTER_VE => Ok(Elbasan::LetterVe),
            LETTER_XE => Ok(Elbasan::LetterXe),
            LETTER_Y => Ok(Elbasan::LetterY),
            LETTER_ZE => Ok(Elbasan::LetterZe),
            LETTER_ZHE => Ok(Elbasan::LetterZhe),
            LETTER_GHE => Ok(Elbasan::LetterGhe),
            LETTER_GHAMMA => Ok(Elbasan::LetterGhamma),
            LETTER_KHE => Ok(Elbasan::LetterKhe),
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

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Elbasan::LetterA => "elbasan letter a",
            Elbasan::LetterBe => "elbasan letter be",
            Elbasan::LetterCe => "elbasan letter ce",
            Elbasan::LetterChe => "elbasan letter che",
            Elbasan::LetterDe => "elbasan letter de",
            Elbasan::LetterNde => "elbasan letter nde",
            Elbasan::LetterDhe => "elbasan letter dhe",
            Elbasan::LetterEi => "elbasan letter ei",
            Elbasan::LetterE => "elbasan letter e",
            Elbasan::LetterFe => "elbasan letter fe",
            Elbasan::LetterGe => "elbasan letter ge",
            Elbasan::LetterGje => "elbasan letter gje",
            Elbasan::LetterHe => "elbasan letter he",
            Elbasan::LetterI => "elbasan letter i",
            Elbasan::LetterJe => "elbasan letter je",
            Elbasan::LetterKe => "elbasan letter ke",
            Elbasan::LetterLe => "elbasan letter le",
            Elbasan::LetterLle => "elbasan letter lle",
            Elbasan::LetterMe => "elbasan letter me",
            Elbasan::LetterNe => "elbasan letter ne",
            Elbasan::LetterNa => "elbasan letter na",
            Elbasan::LetterNje => "elbasan letter nje",
            Elbasan::LetterO => "elbasan letter o",
            Elbasan::LetterPe => "elbasan letter pe",
            Elbasan::LetterQe => "elbasan letter qe",
            Elbasan::LetterRe => "elbasan letter re",
            Elbasan::LetterRre => "elbasan letter rre",
            Elbasan::LetterSe => "elbasan letter se",
            Elbasan::LetterShe => "elbasan letter she",
            Elbasan::LetterTe => "elbasan letter te",
            Elbasan::LetterThe => "elbasan letter the",
            Elbasan::LetterU => "elbasan letter u",
            Elbasan::LetterVe => "elbasan letter ve",
            Elbasan::LetterXe => "elbasan letter xe",
            Elbasan::LetterY => "elbasan letter y",
            Elbasan::LetterZe => "elbasan letter ze",
            Elbasan::LetterZhe => "elbasan letter zhe",
            Elbasan::LetterGhe => "elbasan letter ghe",
            Elbasan::LetterGhamma => "elbasan letter ghamma",
            Elbasan::LetterKhe => "elbasan letter khe",
        }
    }
}
