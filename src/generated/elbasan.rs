/// \u{10500} → \u{1052f}\
///\
/// 𐔀 𐔁 𐔂 𐔃 𐔄 𐔅 𐔆 𐔇 𐔈 𐔉 𐔊 𐔋 𐔌 𐔍 𐔎 𐔏
/// 𐔐 𐔑 𐔒 𐔓 𐔔 𐔕 𐔖 𐔗 𐔘 𐔙 𐔚 𐔛 𐔜 𐔝 𐔞 𐔟
/// 𐔠 𐔡 𐔢 𐔣 𐔤 𐔥 𐔦 𐔧
pub mod constants {
    /// \u{10500}: '𐔀'
    pub const ELBASAN_LETTER_A: char = '𐔀';
    /// \u{10501}: '𐔁'
    pub const ELBASAN_LETTER_BE: char = '𐔁';
    /// \u{10502}: '𐔂'
    pub const ELBASAN_LETTER_CE: char = '𐔂';
    /// \u{10503}: '𐔃'
    pub const ELBASAN_LETTER_CHE: char = '𐔃';
    /// \u{10504}: '𐔄'
    pub const ELBASAN_LETTER_DE: char = '𐔄';
    /// \u{10505}: '𐔅'
    pub const ELBASAN_LETTER_NDE: char = '𐔅';
    /// \u{10506}: '𐔆'
    pub const ELBASAN_LETTER_DHE: char = '𐔆';
    /// \u{10507}: '𐔇'
    pub const ELBASAN_LETTER_EI: char = '𐔇';
    /// \u{10508}: '𐔈'
    pub const ELBASAN_LETTER_E: char = '𐔈';
    /// \u{10509}: '𐔉'
    pub const ELBASAN_LETTER_FE: char = '𐔉';
    /// \u{1050a}: '𐔊'
    pub const ELBASAN_LETTER_GE: char = '𐔊';
    /// \u{1050b}: '𐔋'
    pub const ELBASAN_LETTER_GJE: char = '𐔋';
    /// \u{1050c}: '𐔌'
    pub const ELBASAN_LETTER_HE: char = '𐔌';
    /// \u{1050d}: '𐔍'
    pub const ELBASAN_LETTER_I: char = '𐔍';
    /// \u{1050e}: '𐔎'
    pub const ELBASAN_LETTER_JE: char = '𐔎';
    /// \u{1050f}: '𐔏'
    pub const ELBASAN_LETTER_KE: char = '𐔏';
    /// \u{10510}: '𐔐'
    pub const ELBASAN_LETTER_LE: char = '𐔐';
    /// \u{10511}: '𐔑'
    pub const ELBASAN_LETTER_LLE: char = '𐔑';
    /// \u{10512}: '𐔒'
    pub const ELBASAN_LETTER_ME: char = '𐔒';
    /// \u{10513}: '𐔓'
    pub const ELBASAN_LETTER_NE: char = '𐔓';
    /// \u{10514}: '𐔔'
    pub const ELBASAN_LETTER_NA: char = '𐔔';
    /// \u{10515}: '𐔕'
    pub const ELBASAN_LETTER_NJE: char = '𐔕';
    /// \u{10516}: '𐔖'
    pub const ELBASAN_LETTER_O: char = '𐔖';
    /// \u{10517}: '𐔗'
    pub const ELBASAN_LETTER_PE: char = '𐔗';
    /// \u{10518}: '𐔘'
    pub const ELBASAN_LETTER_QE: char = '𐔘';
    /// \u{10519}: '𐔙'
    pub const ELBASAN_LETTER_RE: char = '𐔙';
    /// \u{1051a}: '𐔚'
    pub const ELBASAN_LETTER_RRE: char = '𐔚';
    /// \u{1051b}: '𐔛'
    pub const ELBASAN_LETTER_SE: char = '𐔛';
    /// \u{1051c}: '𐔜'
    pub const ELBASAN_LETTER_SHE: char = '𐔜';
    /// \u{1051d}: '𐔝'
    pub const ELBASAN_LETTER_TE: char = '𐔝';
    /// \u{1051e}: '𐔞'
    pub const ELBASAN_LETTER_THE: char = '𐔞';
    /// \u{1051f}: '𐔟'
    pub const ELBASAN_LETTER_U: char = '𐔟';
    /// \u{10520}: '𐔠'
    pub const ELBASAN_LETTER_VE: char = '𐔠';
    /// \u{10521}: '𐔡'
    pub const ELBASAN_LETTER_XE: char = '𐔡';
    /// \u{10522}: '𐔢'
    pub const ELBASAN_LETTER_Y: char = '𐔢';
    /// \u{10523}: '𐔣'
    pub const ELBASAN_LETTER_ZE: char = '𐔣';
    /// \u{10524}: '𐔤'
    pub const ELBASAN_LETTER_ZHE: char = '𐔤';
    /// \u{10525}: '𐔥'
    pub const ELBASAN_LETTER_GHE: char = '𐔥';
    /// \u{10526}: '𐔦'
    pub const ELBASAN_LETTER_GHAMMA: char = '𐔦';
    /// \u{10527}: '𐔧'
    pub const ELBASAN_LETTER_KHE: char = '𐔧';
}

/// \u{10500} → \u{1052f}\
///\
/// 𐔀 𐔁 𐔂 𐔃 𐔄 𐔅 𐔆 𐔇 𐔈 𐔉 𐔊 𐔋 𐔌 𐔍 𐔎 𐔏
/// 𐔐 𐔑 𐔒 𐔓 𐔔 𐔕 𐔖 𐔗 𐔘 𐔙 𐔚 𐔛 𐔜 𐔝 𐔞 𐔟
/// 𐔠 𐔡 𐔢 𐔣 𐔤 𐔥 𐔦 𐔧
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Elbasan {
    /// \u{10500}: '𐔀'
    ElbasanLetterA,
    /// \u{10501}: '𐔁'
    ElbasanLetterBe,
    /// \u{10502}: '𐔂'
    ElbasanLetterCe,
    /// \u{10503}: '𐔃'
    ElbasanLetterChe,
    /// \u{10504}: '𐔄'
    ElbasanLetterDe,
    /// \u{10505}: '𐔅'
    ElbasanLetterNde,
    /// \u{10506}: '𐔆'
    ElbasanLetterDhe,
    /// \u{10507}: '𐔇'
    ElbasanLetterEi,
    /// \u{10508}: '𐔈'
    ElbasanLetterE,
    /// \u{10509}: '𐔉'
    ElbasanLetterFe,
    /// \u{1050a}: '𐔊'
    ElbasanLetterGe,
    /// \u{1050b}: '𐔋'
    ElbasanLetterGje,
    /// \u{1050c}: '𐔌'
    ElbasanLetterHe,
    /// \u{1050d}: '𐔍'
    ElbasanLetterI,
    /// \u{1050e}: '𐔎'
    ElbasanLetterJe,
    /// \u{1050f}: '𐔏'
    ElbasanLetterKe,
    /// \u{10510}: '𐔐'
    ElbasanLetterLe,
    /// \u{10511}: '𐔑'
    ElbasanLetterLle,
    /// \u{10512}: '𐔒'
    ElbasanLetterMe,
    /// \u{10513}: '𐔓'
    ElbasanLetterNe,
    /// \u{10514}: '𐔔'
    ElbasanLetterNa,
    /// \u{10515}: '𐔕'
    ElbasanLetterNje,
    /// \u{10516}: '𐔖'
    ElbasanLetterO,
    /// \u{10517}: '𐔗'
    ElbasanLetterPe,
    /// \u{10518}: '𐔘'
    ElbasanLetterQe,
    /// \u{10519}: '𐔙'
    ElbasanLetterRe,
    /// \u{1051a}: '𐔚'
    ElbasanLetterRre,
    /// \u{1051b}: '𐔛'
    ElbasanLetterSe,
    /// \u{1051c}: '𐔜'
    ElbasanLetterShe,
    /// \u{1051d}: '𐔝'
    ElbasanLetterTe,
    /// \u{1051e}: '𐔞'
    ElbasanLetterThe,
    /// \u{1051f}: '𐔟'
    ElbasanLetterU,
    /// \u{10520}: '𐔠'
    ElbasanLetterVe,
    /// \u{10521}: '𐔡'
    ElbasanLetterXe,
    /// \u{10522}: '𐔢'
    ElbasanLetterY,
    /// \u{10523}: '𐔣'
    ElbasanLetterZe,
    /// \u{10524}: '𐔤'
    ElbasanLetterZhe,
    /// \u{10525}: '𐔥'
    ElbasanLetterGhe,
    /// \u{10526}: '𐔦'
    ElbasanLetterGhamma,
    /// \u{10527}: '𐔧'
    ElbasanLetterKhe,
}

impl Into<char> for Elbasan {
    fn into(self) -> char {
        use constants::*;
        match self {
            Elbasan::ElbasanLetterA => ELBASAN_LETTER_A,
            Elbasan::ElbasanLetterBe => ELBASAN_LETTER_BE,
            Elbasan::ElbasanLetterCe => ELBASAN_LETTER_CE,
            Elbasan::ElbasanLetterChe => ELBASAN_LETTER_CHE,
            Elbasan::ElbasanLetterDe => ELBASAN_LETTER_DE,
            Elbasan::ElbasanLetterNde => ELBASAN_LETTER_NDE,
            Elbasan::ElbasanLetterDhe => ELBASAN_LETTER_DHE,
            Elbasan::ElbasanLetterEi => ELBASAN_LETTER_EI,
            Elbasan::ElbasanLetterE => ELBASAN_LETTER_E,
            Elbasan::ElbasanLetterFe => ELBASAN_LETTER_FE,
            Elbasan::ElbasanLetterGe => ELBASAN_LETTER_GE,
            Elbasan::ElbasanLetterGje => ELBASAN_LETTER_GJE,
            Elbasan::ElbasanLetterHe => ELBASAN_LETTER_HE,
            Elbasan::ElbasanLetterI => ELBASAN_LETTER_I,
            Elbasan::ElbasanLetterJe => ELBASAN_LETTER_JE,
            Elbasan::ElbasanLetterKe => ELBASAN_LETTER_KE,
            Elbasan::ElbasanLetterLe => ELBASAN_LETTER_LE,
            Elbasan::ElbasanLetterLle => ELBASAN_LETTER_LLE,
            Elbasan::ElbasanLetterMe => ELBASAN_LETTER_ME,
            Elbasan::ElbasanLetterNe => ELBASAN_LETTER_NE,
            Elbasan::ElbasanLetterNa => ELBASAN_LETTER_NA,
            Elbasan::ElbasanLetterNje => ELBASAN_LETTER_NJE,
            Elbasan::ElbasanLetterO => ELBASAN_LETTER_O,
            Elbasan::ElbasanLetterPe => ELBASAN_LETTER_PE,
            Elbasan::ElbasanLetterQe => ELBASAN_LETTER_QE,
            Elbasan::ElbasanLetterRe => ELBASAN_LETTER_RE,
            Elbasan::ElbasanLetterRre => ELBASAN_LETTER_RRE,
            Elbasan::ElbasanLetterSe => ELBASAN_LETTER_SE,
            Elbasan::ElbasanLetterShe => ELBASAN_LETTER_SHE,
            Elbasan::ElbasanLetterTe => ELBASAN_LETTER_TE,
            Elbasan::ElbasanLetterThe => ELBASAN_LETTER_THE,
            Elbasan::ElbasanLetterU => ELBASAN_LETTER_U,
            Elbasan::ElbasanLetterVe => ELBASAN_LETTER_VE,
            Elbasan::ElbasanLetterXe => ELBASAN_LETTER_XE,
            Elbasan::ElbasanLetterY => ELBASAN_LETTER_Y,
            Elbasan::ElbasanLetterZe => ELBASAN_LETTER_ZE,
            Elbasan::ElbasanLetterZhe => ELBASAN_LETTER_ZHE,
            Elbasan::ElbasanLetterGhe => ELBASAN_LETTER_GHE,
            Elbasan::ElbasanLetterGhamma => ELBASAN_LETTER_GHAMMA,
            Elbasan::ElbasanLetterKhe => ELBASAN_LETTER_KHE,
        }
    }
}

impl std::convert::TryFrom<char> for Elbasan {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            ELBASAN_LETTER_A => Ok(Elbasan::ElbasanLetterA),
            ELBASAN_LETTER_BE => Ok(Elbasan::ElbasanLetterBe),
            ELBASAN_LETTER_CE => Ok(Elbasan::ElbasanLetterCe),
            ELBASAN_LETTER_CHE => Ok(Elbasan::ElbasanLetterChe),
            ELBASAN_LETTER_DE => Ok(Elbasan::ElbasanLetterDe),
            ELBASAN_LETTER_NDE => Ok(Elbasan::ElbasanLetterNde),
            ELBASAN_LETTER_DHE => Ok(Elbasan::ElbasanLetterDhe),
            ELBASAN_LETTER_EI => Ok(Elbasan::ElbasanLetterEi),
            ELBASAN_LETTER_E => Ok(Elbasan::ElbasanLetterE),
            ELBASAN_LETTER_FE => Ok(Elbasan::ElbasanLetterFe),
            ELBASAN_LETTER_GE => Ok(Elbasan::ElbasanLetterGe),
            ELBASAN_LETTER_GJE => Ok(Elbasan::ElbasanLetterGje),
            ELBASAN_LETTER_HE => Ok(Elbasan::ElbasanLetterHe),
            ELBASAN_LETTER_I => Ok(Elbasan::ElbasanLetterI),
            ELBASAN_LETTER_JE => Ok(Elbasan::ElbasanLetterJe),
            ELBASAN_LETTER_KE => Ok(Elbasan::ElbasanLetterKe),
            ELBASAN_LETTER_LE => Ok(Elbasan::ElbasanLetterLe),
            ELBASAN_LETTER_LLE => Ok(Elbasan::ElbasanLetterLle),
            ELBASAN_LETTER_ME => Ok(Elbasan::ElbasanLetterMe),
            ELBASAN_LETTER_NE => Ok(Elbasan::ElbasanLetterNe),
            ELBASAN_LETTER_NA => Ok(Elbasan::ElbasanLetterNa),
            ELBASAN_LETTER_NJE => Ok(Elbasan::ElbasanLetterNje),
            ELBASAN_LETTER_O => Ok(Elbasan::ElbasanLetterO),
            ELBASAN_LETTER_PE => Ok(Elbasan::ElbasanLetterPe),
            ELBASAN_LETTER_QE => Ok(Elbasan::ElbasanLetterQe),
            ELBASAN_LETTER_RE => Ok(Elbasan::ElbasanLetterRe),
            ELBASAN_LETTER_RRE => Ok(Elbasan::ElbasanLetterRre),
            ELBASAN_LETTER_SE => Ok(Elbasan::ElbasanLetterSe),
            ELBASAN_LETTER_SHE => Ok(Elbasan::ElbasanLetterShe),
            ELBASAN_LETTER_TE => Ok(Elbasan::ElbasanLetterTe),
            ELBASAN_LETTER_THE => Ok(Elbasan::ElbasanLetterThe),
            ELBASAN_LETTER_U => Ok(Elbasan::ElbasanLetterU),
            ELBASAN_LETTER_VE => Ok(Elbasan::ElbasanLetterVe),
            ELBASAN_LETTER_XE => Ok(Elbasan::ElbasanLetterXe),
            ELBASAN_LETTER_Y => Ok(Elbasan::ElbasanLetterY),
            ELBASAN_LETTER_ZE => Ok(Elbasan::ElbasanLetterZe),
            ELBASAN_LETTER_ZHE => Ok(Elbasan::ElbasanLetterZhe),
            ELBASAN_LETTER_GHE => Ok(Elbasan::ElbasanLetterGhe),
            ELBASAN_LETTER_GHAMMA => Ok(Elbasan::ElbasanLetterGhamma),
            ELBASAN_LETTER_KHE => Ok(Elbasan::ElbasanLetterKhe),
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
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        Elbasan::ElbasanLetterA
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Elbasan::ElbasanLetterA => "elbasan letter a",
            Elbasan::ElbasanLetterBe => "elbasan letter be",
            Elbasan::ElbasanLetterCe => "elbasan letter ce",
            Elbasan::ElbasanLetterChe => "elbasan letter che",
            Elbasan::ElbasanLetterDe => "elbasan letter de",
            Elbasan::ElbasanLetterNde => "elbasan letter nde",
            Elbasan::ElbasanLetterDhe => "elbasan letter dhe",
            Elbasan::ElbasanLetterEi => "elbasan letter ei",
            Elbasan::ElbasanLetterE => "elbasan letter e",
            Elbasan::ElbasanLetterFe => "elbasan letter fe",
            Elbasan::ElbasanLetterGe => "elbasan letter ge",
            Elbasan::ElbasanLetterGje => "elbasan letter gje",
            Elbasan::ElbasanLetterHe => "elbasan letter he",
            Elbasan::ElbasanLetterI => "elbasan letter i",
            Elbasan::ElbasanLetterJe => "elbasan letter je",
            Elbasan::ElbasanLetterKe => "elbasan letter ke",
            Elbasan::ElbasanLetterLe => "elbasan letter le",
            Elbasan::ElbasanLetterLle => "elbasan letter lle",
            Elbasan::ElbasanLetterMe => "elbasan letter me",
            Elbasan::ElbasanLetterNe => "elbasan letter ne",
            Elbasan::ElbasanLetterNa => "elbasan letter na",
            Elbasan::ElbasanLetterNje => "elbasan letter nje",
            Elbasan::ElbasanLetterO => "elbasan letter o",
            Elbasan::ElbasanLetterPe => "elbasan letter pe",
            Elbasan::ElbasanLetterQe => "elbasan letter qe",
            Elbasan::ElbasanLetterRe => "elbasan letter re",
            Elbasan::ElbasanLetterRre => "elbasan letter rre",
            Elbasan::ElbasanLetterSe => "elbasan letter se",
            Elbasan::ElbasanLetterShe => "elbasan letter she",
            Elbasan::ElbasanLetterTe => "elbasan letter te",
            Elbasan::ElbasanLetterThe => "elbasan letter the",
            Elbasan::ElbasanLetterU => "elbasan letter u",
            Elbasan::ElbasanLetterVe => "elbasan letter ve",
            Elbasan::ElbasanLetterXe => "elbasan letter xe",
            Elbasan::ElbasanLetterY => "elbasan letter y",
            Elbasan::ElbasanLetterZe => "elbasan letter ze",
            Elbasan::ElbasanLetterZhe => "elbasan letter zhe",
            Elbasan::ElbasanLetterGhe => "elbasan letter ghe",
            Elbasan::ElbasanLetterGhamma => "elbasan letter ghamma",
            Elbasan::ElbasanLetterKhe => "elbasan letter khe",
        }
    }
}
