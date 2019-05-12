/// \u{10380} → \u{1039f}\
///\
/// 𐎀 𐎁 𐎂 𐎃 𐎄 𐎅 𐎆 𐎇 𐎈 𐎉 𐎊 𐎋 𐎌 𐎍 𐎎 𐎏\
/// 𐎐 𐎑 𐎒 𐎓 𐎔 𐎕 𐎖 𐎗 𐎘 𐎙 𐎚 𐎛 𐎜 𐎝\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{10380}: '𐎀'
    pub const LETTER_ALPA: char = '𐎀';
    /// \u{10381}: '𐎁'
    pub const LETTER_BETA: char = '𐎁';
    /// \u{10382}: '𐎂'
    pub const LETTER_GAMLA: char = '𐎂';
    /// \u{10383}: '𐎃'
    pub const LETTER_KHA: char = '𐎃';
    /// \u{10384}: '𐎄'
    pub const LETTER_DELTA: char = '𐎄';
    /// \u{10385}: '𐎅'
    pub const LETTER_HO: char = '𐎅';
    /// \u{10386}: '𐎆'
    pub const LETTER_WO: char = '𐎆';
    /// \u{10387}: '𐎇'
    pub const LETTER_ZETA: char = '𐎇';
    /// \u{10388}: '𐎈'
    pub const LETTER_HOTA: char = '𐎈';
    /// \u{10389}: '𐎉'
    pub const LETTER_TET: char = '𐎉';
    /// \u{1038a}: '𐎊'
    pub const LETTER_YOD: char = '𐎊';
    /// \u{1038b}: '𐎋'
    pub const LETTER_KAF: char = '𐎋';
    /// \u{1038c}: '𐎌'
    pub const LETTER_SHIN: char = '𐎌';
    /// \u{1038d}: '𐎍'
    pub const LETTER_LAMDA: char = '𐎍';
    /// \u{1038e}: '𐎎'
    pub const LETTER_MEM: char = '𐎎';
    /// \u{1038f}: '𐎏'
    pub const LETTER_DHAL: char = '𐎏';
    /// \u{10390}: '𐎐'
    pub const LETTER_NUN: char = '𐎐';
    /// \u{10391}: '𐎑'
    pub const LETTER_ZU: char = '𐎑';
    /// \u{10392}: '𐎒'
    pub const LETTER_SAMKA: char = '𐎒';
    /// \u{10393}: '𐎓'
    pub const LETTER_AIN: char = '𐎓';
    /// \u{10394}: '𐎔'
    pub const LETTER_PU: char = '𐎔';
    /// \u{10395}: '𐎕'
    pub const LETTER_SADE: char = '𐎕';
    /// \u{10396}: '𐎖'
    pub const LETTER_QOPA: char = '𐎖';
    /// \u{10397}: '𐎗'
    pub const LETTER_RASHA: char = '𐎗';
    /// \u{10398}: '𐎘'
    pub const LETTER_THANNA: char = '𐎘';
    /// \u{10399}: '𐎙'
    pub const LETTER_GHAIN: char = '𐎙';
    /// \u{1039a}: '𐎚'
    pub const LETTER_TO: char = '𐎚';
    /// \u{1039b}: '𐎛'
    pub const LETTER_I: char = '𐎛';
    /// \u{1039c}: '𐎜'
    pub const LETTER_U: char = '𐎜';
    /// \u{1039d}: '𐎝'
    pub const LETTER_SSU: char = '𐎝';
}

/// An enum to represent all characters in the Ugaritic block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Ugaritic {
    /// \u{10380}: '𐎀'
    LetterAlpa,
    /// \u{10381}: '𐎁'
    LetterBeta,
    /// \u{10382}: '𐎂'
    LetterGamla,
    /// \u{10383}: '𐎃'
    LetterKha,
    /// \u{10384}: '𐎄'
    LetterDelta,
    /// \u{10385}: '𐎅'
    LetterHo,
    /// \u{10386}: '𐎆'
    LetterWo,
    /// \u{10387}: '𐎇'
    LetterZeta,
    /// \u{10388}: '𐎈'
    LetterHota,
    /// \u{10389}: '𐎉'
    LetterTet,
    /// \u{1038a}: '𐎊'
    LetterYod,
    /// \u{1038b}: '𐎋'
    LetterKaf,
    /// \u{1038c}: '𐎌'
    LetterShin,
    /// \u{1038d}: '𐎍'
    LetterLamda,
    /// \u{1038e}: '𐎎'
    LetterMem,
    /// \u{1038f}: '𐎏'
    LetterDhal,
    /// \u{10390}: '𐎐'
    LetterNun,
    /// \u{10391}: '𐎑'
    LetterZu,
    /// \u{10392}: '𐎒'
    LetterSamka,
    /// \u{10393}: '𐎓'
    LetterAin,
    /// \u{10394}: '𐎔'
    LetterPu,
    /// \u{10395}: '𐎕'
    LetterSade,
    /// \u{10396}: '𐎖'
    LetterQopa,
    /// \u{10397}: '𐎗'
    LetterRasha,
    /// \u{10398}: '𐎘'
    LetterThanna,
    /// \u{10399}: '𐎙'
    LetterGhain,
    /// \u{1039a}: '𐎚'
    LetterTo,
    /// \u{1039b}: '𐎛'
    LetterI,
    /// \u{1039c}: '𐎜'
    LetterU,
    /// \u{1039d}: '𐎝'
    LetterSsu,
}

impl Into<char> for Ugaritic {
    fn into(self) -> char {
        use constants::*;
        match self {
            Ugaritic::LetterAlpa => LETTER_ALPA,
            Ugaritic::LetterBeta => LETTER_BETA,
            Ugaritic::LetterGamla => LETTER_GAMLA,
            Ugaritic::LetterKha => LETTER_KHA,
            Ugaritic::LetterDelta => LETTER_DELTA,
            Ugaritic::LetterHo => LETTER_HO,
            Ugaritic::LetterWo => LETTER_WO,
            Ugaritic::LetterZeta => LETTER_ZETA,
            Ugaritic::LetterHota => LETTER_HOTA,
            Ugaritic::LetterTet => LETTER_TET,
            Ugaritic::LetterYod => LETTER_YOD,
            Ugaritic::LetterKaf => LETTER_KAF,
            Ugaritic::LetterShin => LETTER_SHIN,
            Ugaritic::LetterLamda => LETTER_LAMDA,
            Ugaritic::LetterMem => LETTER_MEM,
            Ugaritic::LetterDhal => LETTER_DHAL,
            Ugaritic::LetterNun => LETTER_NUN,
            Ugaritic::LetterZu => LETTER_ZU,
            Ugaritic::LetterSamka => LETTER_SAMKA,
            Ugaritic::LetterAin => LETTER_AIN,
            Ugaritic::LetterPu => LETTER_PU,
            Ugaritic::LetterSade => LETTER_SADE,
            Ugaritic::LetterQopa => LETTER_QOPA,
            Ugaritic::LetterRasha => LETTER_RASHA,
            Ugaritic::LetterThanna => LETTER_THANNA,
            Ugaritic::LetterGhain => LETTER_GHAIN,
            Ugaritic::LetterTo => LETTER_TO,
            Ugaritic::LetterI => LETTER_I,
            Ugaritic::LetterU => LETTER_U,
            Ugaritic::LetterSsu => LETTER_SSU,
        }
    }
}

impl std::convert::TryFrom<char> for Ugaritic {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_ALPA => Ok(Ugaritic::LetterAlpa),
            LETTER_BETA => Ok(Ugaritic::LetterBeta),
            LETTER_GAMLA => Ok(Ugaritic::LetterGamla),
            LETTER_KHA => Ok(Ugaritic::LetterKha),
            LETTER_DELTA => Ok(Ugaritic::LetterDelta),
            LETTER_HO => Ok(Ugaritic::LetterHo),
            LETTER_WO => Ok(Ugaritic::LetterWo),
            LETTER_ZETA => Ok(Ugaritic::LetterZeta),
            LETTER_HOTA => Ok(Ugaritic::LetterHota),
            LETTER_TET => Ok(Ugaritic::LetterTet),
            LETTER_YOD => Ok(Ugaritic::LetterYod),
            LETTER_KAF => Ok(Ugaritic::LetterKaf),
            LETTER_SHIN => Ok(Ugaritic::LetterShin),
            LETTER_LAMDA => Ok(Ugaritic::LetterLamda),
            LETTER_MEM => Ok(Ugaritic::LetterMem),
            LETTER_DHAL => Ok(Ugaritic::LetterDhal),
            LETTER_NUN => Ok(Ugaritic::LetterNun),
            LETTER_ZU => Ok(Ugaritic::LetterZu),
            LETTER_SAMKA => Ok(Ugaritic::LetterSamka),
            LETTER_AIN => Ok(Ugaritic::LetterAin),
            LETTER_PU => Ok(Ugaritic::LetterPu),
            LETTER_SADE => Ok(Ugaritic::LetterSade),
            LETTER_QOPA => Ok(Ugaritic::LetterQopa),
            LETTER_RASHA => Ok(Ugaritic::LetterRasha),
            LETTER_THANNA => Ok(Ugaritic::LetterThanna),
            LETTER_GHAIN => Ok(Ugaritic::LetterGhain),
            LETTER_TO => Ok(Ugaritic::LetterTo),
            LETTER_I => Ok(Ugaritic::LetterI),
            LETTER_U => Ok(Ugaritic::LetterU),
            LETTER_SSU => Ok(Ugaritic::LetterSsu),
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
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Ugaritic::LetterAlpa
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Ugaritic::LetterAlpa => "ugaritic letter alpa",
            Ugaritic::LetterBeta => "ugaritic letter beta",
            Ugaritic::LetterGamla => "ugaritic letter gamla",
            Ugaritic::LetterKha => "ugaritic letter kha",
            Ugaritic::LetterDelta => "ugaritic letter delta",
            Ugaritic::LetterHo => "ugaritic letter ho",
            Ugaritic::LetterWo => "ugaritic letter wo",
            Ugaritic::LetterZeta => "ugaritic letter zeta",
            Ugaritic::LetterHota => "ugaritic letter hota",
            Ugaritic::LetterTet => "ugaritic letter tet",
            Ugaritic::LetterYod => "ugaritic letter yod",
            Ugaritic::LetterKaf => "ugaritic letter kaf",
            Ugaritic::LetterShin => "ugaritic letter shin",
            Ugaritic::LetterLamda => "ugaritic letter lamda",
            Ugaritic::LetterMem => "ugaritic letter mem",
            Ugaritic::LetterDhal => "ugaritic letter dhal",
            Ugaritic::LetterNun => "ugaritic letter nun",
            Ugaritic::LetterZu => "ugaritic letter zu",
            Ugaritic::LetterSamka => "ugaritic letter samka",
            Ugaritic::LetterAin => "ugaritic letter ain",
            Ugaritic::LetterPu => "ugaritic letter pu",
            Ugaritic::LetterSade => "ugaritic letter sade",
            Ugaritic::LetterQopa => "ugaritic letter qopa",
            Ugaritic::LetterRasha => "ugaritic letter rasha",
            Ugaritic::LetterThanna => "ugaritic letter thanna",
            Ugaritic::LetterGhain => "ugaritic letter ghain",
            Ugaritic::LetterTo => "ugaritic letter to",
            Ugaritic::LetterI => "ugaritic letter i",
            Ugaritic::LetterU => "ugaritic letter u",
            Ugaritic::LetterSsu => "ugaritic letter ssu",
        }
    }
}
