/// \u{10380} → \u{1039f}\
///\
/// 𐎀 𐎁 𐎂 𐎃 𐎄 𐎅 𐎆 𐎇 𐎈 𐎉 𐎊 𐎋 𐎌 𐎍 𐎎 𐎏
/// 𐎐 𐎑 𐎒 𐎓 𐎔 𐎕 𐎖 𐎗 𐎘 𐎙 𐎚 𐎛 𐎜 𐎝
pub mod constants {
    /// \u{10380}: '𐎀'
    pub const UGARITIC_LETTER_ALPA: char = '𐎀';
    /// \u{10381}: '𐎁'
    pub const UGARITIC_LETTER_BETA: char = '𐎁';
    /// \u{10382}: '𐎂'
    pub const UGARITIC_LETTER_GAMLA: char = '𐎂';
    /// \u{10383}: '𐎃'
    pub const UGARITIC_LETTER_KHA: char = '𐎃';
    /// \u{10384}: '𐎄'
    pub const UGARITIC_LETTER_DELTA: char = '𐎄';
    /// \u{10385}: '𐎅'
    pub const UGARITIC_LETTER_HO: char = '𐎅';
    /// \u{10386}: '𐎆'
    pub const UGARITIC_LETTER_WO: char = '𐎆';
    /// \u{10387}: '𐎇'
    pub const UGARITIC_LETTER_ZETA: char = '𐎇';
    /// \u{10388}: '𐎈'
    pub const UGARITIC_LETTER_HOTA: char = '𐎈';
    /// \u{10389}: '𐎉'
    pub const UGARITIC_LETTER_TET: char = '𐎉';
    /// \u{1038a}: '𐎊'
    pub const UGARITIC_LETTER_YOD: char = '𐎊';
    /// \u{1038b}: '𐎋'
    pub const UGARITIC_LETTER_KAF: char = '𐎋';
    /// \u{1038c}: '𐎌'
    pub const UGARITIC_LETTER_SHIN: char = '𐎌';
    /// \u{1038d}: '𐎍'
    pub const UGARITIC_LETTER_LAMDA: char = '𐎍';
    /// \u{1038e}: '𐎎'
    pub const UGARITIC_LETTER_MEM: char = '𐎎';
    /// \u{1038f}: '𐎏'
    pub const UGARITIC_LETTER_DHAL: char = '𐎏';
    /// \u{10390}: '𐎐'
    pub const UGARITIC_LETTER_NUN: char = '𐎐';
    /// \u{10391}: '𐎑'
    pub const UGARITIC_LETTER_ZU: char = '𐎑';
    /// \u{10392}: '𐎒'
    pub const UGARITIC_LETTER_SAMKA: char = '𐎒';
    /// \u{10393}: '𐎓'
    pub const UGARITIC_LETTER_AIN: char = '𐎓';
    /// \u{10394}: '𐎔'
    pub const UGARITIC_LETTER_PU: char = '𐎔';
    /// \u{10395}: '𐎕'
    pub const UGARITIC_LETTER_SADE: char = '𐎕';
    /// \u{10396}: '𐎖'
    pub const UGARITIC_LETTER_QOPA: char = '𐎖';
    /// \u{10397}: '𐎗'
    pub const UGARITIC_LETTER_RASHA: char = '𐎗';
    /// \u{10398}: '𐎘'
    pub const UGARITIC_LETTER_THANNA: char = '𐎘';
    /// \u{10399}: '𐎙'
    pub const UGARITIC_LETTER_GHAIN: char = '𐎙';
    /// \u{1039a}: '𐎚'
    pub const UGARITIC_LETTER_TO: char = '𐎚';
    /// \u{1039b}: '𐎛'
    pub const UGARITIC_LETTER_I: char = '𐎛';
    /// \u{1039c}: '𐎜'
    pub const UGARITIC_LETTER_U: char = '𐎜';
    /// \u{1039d}: '𐎝'
    pub const UGARITIC_LETTER_SSU: char = '𐎝';
}

/// \u{10380} → \u{1039f}\
///\
/// 𐎀 𐎁 𐎂 𐎃 𐎄 𐎅 𐎆 𐎇 𐎈 𐎉 𐎊 𐎋 𐎌 𐎍 𐎎 𐎏
/// 𐎐 𐎑 𐎒 𐎓 𐎔 𐎕 𐎖 𐎗 𐎘 𐎙 𐎚 𐎛 𐎜 𐎝
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Ugaritic {
    /// \u{10380}: '𐎀'
    UgariticLetterAlpa,
    /// \u{10381}: '𐎁'
    UgariticLetterBeta,
    /// \u{10382}: '𐎂'
    UgariticLetterGamla,
    /// \u{10383}: '𐎃'
    UgariticLetterKha,
    /// \u{10384}: '𐎄'
    UgariticLetterDelta,
    /// \u{10385}: '𐎅'
    UgariticLetterHo,
    /// \u{10386}: '𐎆'
    UgariticLetterWo,
    /// \u{10387}: '𐎇'
    UgariticLetterZeta,
    /// \u{10388}: '𐎈'
    UgariticLetterHota,
    /// \u{10389}: '𐎉'
    UgariticLetterTet,
    /// \u{1038a}: '𐎊'
    UgariticLetterYod,
    /// \u{1038b}: '𐎋'
    UgariticLetterKaf,
    /// \u{1038c}: '𐎌'
    UgariticLetterShin,
    /// \u{1038d}: '𐎍'
    UgariticLetterLamda,
    /// \u{1038e}: '𐎎'
    UgariticLetterMem,
    /// \u{1038f}: '𐎏'
    UgariticLetterDhal,
    /// \u{10390}: '𐎐'
    UgariticLetterNun,
    /// \u{10391}: '𐎑'
    UgariticLetterZu,
    /// \u{10392}: '𐎒'
    UgariticLetterSamka,
    /// \u{10393}: '𐎓'
    UgariticLetterAin,
    /// \u{10394}: '𐎔'
    UgariticLetterPu,
    /// \u{10395}: '𐎕'
    UgariticLetterSade,
    /// \u{10396}: '𐎖'
    UgariticLetterQopa,
    /// \u{10397}: '𐎗'
    UgariticLetterRasha,
    /// \u{10398}: '𐎘'
    UgariticLetterThanna,
    /// \u{10399}: '𐎙'
    UgariticLetterGhain,
    /// \u{1039a}: '𐎚'
    UgariticLetterTo,
    /// \u{1039b}: '𐎛'
    UgariticLetterI,
    /// \u{1039c}: '𐎜'
    UgariticLetterU,
    /// \u{1039d}: '𐎝'
    UgariticLetterSsu,
}

impl Into<char> for Ugaritic {
    fn into(self) -> char {
        use constants::*;
        match self {
            Ugaritic::UgariticLetterAlpa => UGARITIC_LETTER_ALPA,
            Ugaritic::UgariticLetterBeta => UGARITIC_LETTER_BETA,
            Ugaritic::UgariticLetterGamla => UGARITIC_LETTER_GAMLA,
            Ugaritic::UgariticLetterKha => UGARITIC_LETTER_KHA,
            Ugaritic::UgariticLetterDelta => UGARITIC_LETTER_DELTA,
            Ugaritic::UgariticLetterHo => UGARITIC_LETTER_HO,
            Ugaritic::UgariticLetterWo => UGARITIC_LETTER_WO,
            Ugaritic::UgariticLetterZeta => UGARITIC_LETTER_ZETA,
            Ugaritic::UgariticLetterHota => UGARITIC_LETTER_HOTA,
            Ugaritic::UgariticLetterTet => UGARITIC_LETTER_TET,
            Ugaritic::UgariticLetterYod => UGARITIC_LETTER_YOD,
            Ugaritic::UgariticLetterKaf => UGARITIC_LETTER_KAF,
            Ugaritic::UgariticLetterShin => UGARITIC_LETTER_SHIN,
            Ugaritic::UgariticLetterLamda => UGARITIC_LETTER_LAMDA,
            Ugaritic::UgariticLetterMem => UGARITIC_LETTER_MEM,
            Ugaritic::UgariticLetterDhal => UGARITIC_LETTER_DHAL,
            Ugaritic::UgariticLetterNun => UGARITIC_LETTER_NUN,
            Ugaritic::UgariticLetterZu => UGARITIC_LETTER_ZU,
            Ugaritic::UgariticLetterSamka => UGARITIC_LETTER_SAMKA,
            Ugaritic::UgariticLetterAin => UGARITIC_LETTER_AIN,
            Ugaritic::UgariticLetterPu => UGARITIC_LETTER_PU,
            Ugaritic::UgariticLetterSade => UGARITIC_LETTER_SADE,
            Ugaritic::UgariticLetterQopa => UGARITIC_LETTER_QOPA,
            Ugaritic::UgariticLetterRasha => UGARITIC_LETTER_RASHA,
            Ugaritic::UgariticLetterThanna => UGARITIC_LETTER_THANNA,
            Ugaritic::UgariticLetterGhain => UGARITIC_LETTER_GHAIN,
            Ugaritic::UgariticLetterTo => UGARITIC_LETTER_TO,
            Ugaritic::UgariticLetterI => UGARITIC_LETTER_I,
            Ugaritic::UgariticLetterU => UGARITIC_LETTER_U,
            Ugaritic::UgariticLetterSsu => UGARITIC_LETTER_SSU,
        }
    }
}

impl std::convert::TryFrom<char> for Ugaritic {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            UGARITIC_LETTER_ALPA => Ok(Ugaritic::UgariticLetterAlpa),
            UGARITIC_LETTER_BETA => Ok(Ugaritic::UgariticLetterBeta),
            UGARITIC_LETTER_GAMLA => Ok(Ugaritic::UgariticLetterGamla),
            UGARITIC_LETTER_KHA => Ok(Ugaritic::UgariticLetterKha),
            UGARITIC_LETTER_DELTA => Ok(Ugaritic::UgariticLetterDelta),
            UGARITIC_LETTER_HO => Ok(Ugaritic::UgariticLetterHo),
            UGARITIC_LETTER_WO => Ok(Ugaritic::UgariticLetterWo),
            UGARITIC_LETTER_ZETA => Ok(Ugaritic::UgariticLetterZeta),
            UGARITIC_LETTER_HOTA => Ok(Ugaritic::UgariticLetterHota),
            UGARITIC_LETTER_TET => Ok(Ugaritic::UgariticLetterTet),
            UGARITIC_LETTER_YOD => Ok(Ugaritic::UgariticLetterYod),
            UGARITIC_LETTER_KAF => Ok(Ugaritic::UgariticLetterKaf),
            UGARITIC_LETTER_SHIN => Ok(Ugaritic::UgariticLetterShin),
            UGARITIC_LETTER_LAMDA => Ok(Ugaritic::UgariticLetterLamda),
            UGARITIC_LETTER_MEM => Ok(Ugaritic::UgariticLetterMem),
            UGARITIC_LETTER_DHAL => Ok(Ugaritic::UgariticLetterDhal),
            UGARITIC_LETTER_NUN => Ok(Ugaritic::UgariticLetterNun),
            UGARITIC_LETTER_ZU => Ok(Ugaritic::UgariticLetterZu),
            UGARITIC_LETTER_SAMKA => Ok(Ugaritic::UgariticLetterSamka),
            UGARITIC_LETTER_AIN => Ok(Ugaritic::UgariticLetterAin),
            UGARITIC_LETTER_PU => Ok(Ugaritic::UgariticLetterPu),
            UGARITIC_LETTER_SADE => Ok(Ugaritic::UgariticLetterSade),
            UGARITIC_LETTER_QOPA => Ok(Ugaritic::UgariticLetterQopa),
            UGARITIC_LETTER_RASHA => Ok(Ugaritic::UgariticLetterRasha),
            UGARITIC_LETTER_THANNA => Ok(Ugaritic::UgariticLetterThanna),
            UGARITIC_LETTER_GHAIN => Ok(Ugaritic::UgariticLetterGhain),
            UGARITIC_LETTER_TO => Ok(Ugaritic::UgariticLetterTo),
            UGARITIC_LETTER_I => Ok(Ugaritic::UgariticLetterI),
            UGARITIC_LETTER_U => Ok(Ugaritic::UgariticLetterU),
            UGARITIC_LETTER_SSU => Ok(Ugaritic::UgariticLetterSsu),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Ugaritic {
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

impl std::convert::TryFrom<u32> for Ugaritic {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Ugaritic {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Ugaritic {
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        Ugaritic::UgariticLetterAlpa
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Ugaritic::UgariticLetterAlpa => "ugaritic letter alpa",
            Ugaritic::UgariticLetterBeta => "ugaritic letter beta",
            Ugaritic::UgariticLetterGamla => "ugaritic letter gamla",
            Ugaritic::UgariticLetterKha => "ugaritic letter kha",
            Ugaritic::UgariticLetterDelta => "ugaritic letter delta",
            Ugaritic::UgariticLetterHo => "ugaritic letter ho",
            Ugaritic::UgariticLetterWo => "ugaritic letter wo",
            Ugaritic::UgariticLetterZeta => "ugaritic letter zeta",
            Ugaritic::UgariticLetterHota => "ugaritic letter hota",
            Ugaritic::UgariticLetterTet => "ugaritic letter tet",
            Ugaritic::UgariticLetterYod => "ugaritic letter yod",
            Ugaritic::UgariticLetterKaf => "ugaritic letter kaf",
            Ugaritic::UgariticLetterShin => "ugaritic letter shin",
            Ugaritic::UgariticLetterLamda => "ugaritic letter lamda",
            Ugaritic::UgariticLetterMem => "ugaritic letter mem",
            Ugaritic::UgariticLetterDhal => "ugaritic letter dhal",
            Ugaritic::UgariticLetterNun => "ugaritic letter nun",
            Ugaritic::UgariticLetterZu => "ugaritic letter zu",
            Ugaritic::UgariticLetterSamka => "ugaritic letter samka",
            Ugaritic::UgariticLetterAin => "ugaritic letter ain",
            Ugaritic::UgariticLetterPu => "ugaritic letter pu",
            Ugaritic::UgariticLetterSade => "ugaritic letter sade",
            Ugaritic::UgariticLetterQopa => "ugaritic letter qopa",
            Ugaritic::UgariticLetterRasha => "ugaritic letter rasha",
            Ugaritic::UgariticLetterThanna => "ugaritic letter thanna",
            Ugaritic::UgariticLetterGhain => "ugaritic letter ghain",
            Ugaritic::UgariticLetterTo => "ugaritic letter to",
            Ugaritic::UgariticLetterI => "ugaritic letter i",
            Ugaritic::UgariticLetterU => "ugaritic letter u",
            Ugaritic::UgariticLetterSsu => "ugaritic letter ssu",
        }
    }
}
