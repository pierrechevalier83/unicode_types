/// \u{10900} → \u{1091f}\
///\
/// 𐤀 𐤁 𐤂 𐤃 𐤄 𐤅 𐤆 𐤇 𐤈 𐤉 𐤊 𐤋 𐤌 𐤍 𐤎 𐤏
/// 𐤐 𐤑 𐤒 𐤓 𐤔 𐤕 𐤖 𐤗 𐤘 𐤙 𐤚 𐤛
pub mod constants {
    /// \u{10900}: '𐤀'
    pub const LETTER_ALF: char = '𐤀';
    /// \u{10901}: '𐤁'
    pub const LETTER_BET: char = '𐤁';
    /// \u{10902}: '𐤂'
    pub const LETTER_GAML: char = '𐤂';
    /// \u{10903}: '𐤃'
    pub const LETTER_DELT: char = '𐤃';
    /// \u{10904}: '𐤄'
    pub const LETTER_HE: char = '𐤄';
    /// \u{10905}: '𐤅'
    pub const LETTER_WAU: char = '𐤅';
    /// \u{10906}: '𐤆'
    pub const LETTER_ZAI: char = '𐤆';
    /// \u{10907}: '𐤇'
    pub const LETTER_HET: char = '𐤇';
    /// \u{10908}: '𐤈'
    pub const LETTER_TET: char = '𐤈';
    /// \u{10909}: '𐤉'
    pub const LETTER_YOD: char = '𐤉';
    /// \u{1090a}: '𐤊'
    pub const LETTER_KAF: char = '𐤊';
    /// \u{1090b}: '𐤋'
    pub const LETTER_LAMD: char = '𐤋';
    /// \u{1090c}: '𐤌'
    pub const LETTER_MEM: char = '𐤌';
    /// \u{1090d}: '𐤍'
    pub const LETTER_NUN: char = '𐤍';
    /// \u{1090e}: '𐤎'
    pub const LETTER_SEMK: char = '𐤎';
    /// \u{1090f}: '𐤏'
    pub const LETTER_AIN: char = '𐤏';
    /// \u{10910}: '𐤐'
    pub const LETTER_PE: char = '𐤐';
    /// \u{10911}: '𐤑'
    pub const LETTER_SADE: char = '𐤑';
    /// \u{10912}: '𐤒'
    pub const LETTER_QOF: char = '𐤒';
    /// \u{10913}: '𐤓'
    pub const LETTER_ROSH: char = '𐤓';
    /// \u{10914}: '𐤔'
    pub const LETTER_SHIN: char = '𐤔';
    /// \u{10915}: '𐤕'
    pub const LETTER_TAU: char = '𐤕';
    /// \u{10916}: '𐤖'
    pub const NUMBER_ONE: char = '𐤖';
    /// \u{10917}: '𐤗'
    pub const NUMBER_TEN: char = '𐤗';
    /// \u{10918}: '𐤘'
    pub const NUMBER_TWENTY: char = '𐤘';
    /// \u{10919}: '𐤙'
    pub const NUMBER_ONE_HUNDRED: char = '𐤙';
    /// \u{1091a}: '𐤚'
    pub const NUMBER_TWO: char = '𐤚';
    /// \u{1091b}: '𐤛'
    pub const NUMBER_THREE: char = '𐤛';
}

/// \u{10900} → \u{1091f}\
///\
/// 𐤀 𐤁 𐤂 𐤃 𐤄 𐤅 𐤆 𐤇 𐤈 𐤉 𐤊 𐤋 𐤌 𐤍 𐤎 𐤏
/// 𐤐 𐤑 𐤒 𐤓 𐤔 𐤕 𐤖 𐤗 𐤘 𐤙 𐤚 𐤛
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Phoenician {
    /// \u{10900}: '𐤀'
    LetterAlf,
    /// \u{10901}: '𐤁'
    LetterBet,
    /// \u{10902}: '𐤂'
    LetterGaml,
    /// \u{10903}: '𐤃'
    LetterDelt,
    /// \u{10904}: '𐤄'
    LetterHe,
    /// \u{10905}: '𐤅'
    LetterWau,
    /// \u{10906}: '𐤆'
    LetterZai,
    /// \u{10907}: '𐤇'
    LetterHet,
    /// \u{10908}: '𐤈'
    LetterTet,
    /// \u{10909}: '𐤉'
    LetterYod,
    /// \u{1090a}: '𐤊'
    LetterKaf,
    /// \u{1090b}: '𐤋'
    LetterLamd,
    /// \u{1090c}: '𐤌'
    LetterMem,
    /// \u{1090d}: '𐤍'
    LetterNun,
    /// \u{1090e}: '𐤎'
    LetterSemk,
    /// \u{1090f}: '𐤏'
    LetterAin,
    /// \u{10910}: '𐤐'
    LetterPe,
    /// \u{10911}: '𐤑'
    LetterSade,
    /// \u{10912}: '𐤒'
    LetterQof,
    /// \u{10913}: '𐤓'
    LetterRosh,
    /// \u{10914}: '𐤔'
    LetterShin,
    /// \u{10915}: '𐤕'
    LetterTau,
    /// \u{10916}: '𐤖'
    NumberOne,
    /// \u{10917}: '𐤗'
    NumberTen,
    /// \u{10918}: '𐤘'
    NumberTwenty,
    /// \u{10919}: '𐤙'
    NumberOneHundred,
    /// \u{1091a}: '𐤚'
    NumberTwo,
    /// \u{1091b}: '𐤛'
    NumberThree,
}

impl Into<char> for Phoenician {
    fn into(self) -> char {
        use constants::*;
        match self {
            Phoenician::LetterAlf => LETTER_ALF,
            Phoenician::LetterBet => LETTER_BET,
            Phoenician::LetterGaml => LETTER_GAML,
            Phoenician::LetterDelt => LETTER_DELT,
            Phoenician::LetterHe => LETTER_HE,
            Phoenician::LetterWau => LETTER_WAU,
            Phoenician::LetterZai => LETTER_ZAI,
            Phoenician::LetterHet => LETTER_HET,
            Phoenician::LetterTet => LETTER_TET,
            Phoenician::LetterYod => LETTER_YOD,
            Phoenician::LetterKaf => LETTER_KAF,
            Phoenician::LetterLamd => LETTER_LAMD,
            Phoenician::LetterMem => LETTER_MEM,
            Phoenician::LetterNun => LETTER_NUN,
            Phoenician::LetterSemk => LETTER_SEMK,
            Phoenician::LetterAin => LETTER_AIN,
            Phoenician::LetterPe => LETTER_PE,
            Phoenician::LetterSade => LETTER_SADE,
            Phoenician::LetterQof => LETTER_QOF,
            Phoenician::LetterRosh => LETTER_ROSH,
            Phoenician::LetterShin => LETTER_SHIN,
            Phoenician::LetterTau => LETTER_TAU,
            Phoenician::NumberOne => NUMBER_ONE,
            Phoenician::NumberTen => NUMBER_TEN,
            Phoenician::NumberTwenty => NUMBER_TWENTY,
            Phoenician::NumberOneHundred => NUMBER_ONE_HUNDRED,
            Phoenician::NumberTwo => NUMBER_TWO,
            Phoenician::NumberThree => NUMBER_THREE,
        }
    }
}

impl std::convert::TryFrom<char> for Phoenician {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_ALF => Ok(Phoenician::LetterAlf),
            LETTER_BET => Ok(Phoenician::LetterBet),
            LETTER_GAML => Ok(Phoenician::LetterGaml),
            LETTER_DELT => Ok(Phoenician::LetterDelt),
            LETTER_HE => Ok(Phoenician::LetterHe),
            LETTER_WAU => Ok(Phoenician::LetterWau),
            LETTER_ZAI => Ok(Phoenician::LetterZai),
            LETTER_HET => Ok(Phoenician::LetterHet),
            LETTER_TET => Ok(Phoenician::LetterTet),
            LETTER_YOD => Ok(Phoenician::LetterYod),
            LETTER_KAF => Ok(Phoenician::LetterKaf),
            LETTER_LAMD => Ok(Phoenician::LetterLamd),
            LETTER_MEM => Ok(Phoenician::LetterMem),
            LETTER_NUN => Ok(Phoenician::LetterNun),
            LETTER_SEMK => Ok(Phoenician::LetterSemk),
            LETTER_AIN => Ok(Phoenician::LetterAin),
            LETTER_PE => Ok(Phoenician::LetterPe),
            LETTER_SADE => Ok(Phoenician::LetterSade),
            LETTER_QOF => Ok(Phoenician::LetterQof),
            LETTER_ROSH => Ok(Phoenician::LetterRosh),
            LETTER_SHIN => Ok(Phoenician::LetterShin),
            LETTER_TAU => Ok(Phoenician::LetterTau),
            NUMBER_ONE => Ok(Phoenician::NumberOne),
            NUMBER_TEN => Ok(Phoenician::NumberTen),
            NUMBER_TWENTY => Ok(Phoenician::NumberTwenty),
            NUMBER_ONE_HUNDRED => Ok(Phoenician::NumberOneHundred),
            NUMBER_TWO => Ok(Phoenician::NumberTwo),
            NUMBER_THREE => Ok(Phoenician::NumberThree),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Phoenician {
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

impl std::convert::TryFrom<u32> for Phoenician {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Phoenician {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Phoenician {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Phoenician::LetterAlf
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Phoenician::LetterAlf => "phoenician letter alf",
            Phoenician::LetterBet => "phoenician letter bet",
            Phoenician::LetterGaml => "phoenician letter gaml",
            Phoenician::LetterDelt => "phoenician letter delt",
            Phoenician::LetterHe => "phoenician letter he",
            Phoenician::LetterWau => "phoenician letter wau",
            Phoenician::LetterZai => "phoenician letter zai",
            Phoenician::LetterHet => "phoenician letter het",
            Phoenician::LetterTet => "phoenician letter tet",
            Phoenician::LetterYod => "phoenician letter yod",
            Phoenician::LetterKaf => "phoenician letter kaf",
            Phoenician::LetterLamd => "phoenician letter lamd",
            Phoenician::LetterMem => "phoenician letter mem",
            Phoenician::LetterNun => "phoenician letter nun",
            Phoenician::LetterSemk => "phoenician letter semk",
            Phoenician::LetterAin => "phoenician letter ain",
            Phoenician::LetterPe => "phoenician letter pe",
            Phoenician::LetterSade => "phoenician letter sade",
            Phoenician::LetterQof => "phoenician letter qof",
            Phoenician::LetterRosh => "phoenician letter rosh",
            Phoenician::LetterShin => "phoenician letter shin",
            Phoenician::LetterTau => "phoenician letter tau",
            Phoenician::NumberOne => "phoenician number one",
            Phoenician::NumberTen => "phoenician number ten",
            Phoenician::NumberTwenty => "phoenician number twenty",
            Phoenician::NumberOneHundred => "phoenician number one hundred",
            Phoenician::NumberTwo => "phoenician number two",
            Phoenician::NumberThree => "phoenician number three",
        }
    }
}
