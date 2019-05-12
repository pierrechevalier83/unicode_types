/// \u{10900} → \u{1091f}\
///\
/// 𐤀 𐤁 𐤂 𐤃 𐤄 𐤅 𐤆 𐤇 𐤈 𐤉 𐤊 𐤋 𐤌 𐤍 𐤎 𐤏
/// 𐤐 𐤑 𐤒 𐤓 𐤔 𐤕 𐤖 𐤗 𐤘 𐤙 𐤚 𐤛
pub mod constants {
    /// \u{10900}: '𐤀'
    pub const PHOENICIAN_LETTER_ALF: char = '𐤀';
    /// \u{10901}: '𐤁'
    pub const PHOENICIAN_LETTER_BET: char = '𐤁';
    /// \u{10902}: '𐤂'
    pub const PHOENICIAN_LETTER_GAML: char = '𐤂';
    /// \u{10903}: '𐤃'
    pub const PHOENICIAN_LETTER_DELT: char = '𐤃';
    /// \u{10904}: '𐤄'
    pub const PHOENICIAN_LETTER_HE: char = '𐤄';
    /// \u{10905}: '𐤅'
    pub const PHOENICIAN_LETTER_WAU: char = '𐤅';
    /// \u{10906}: '𐤆'
    pub const PHOENICIAN_LETTER_ZAI: char = '𐤆';
    /// \u{10907}: '𐤇'
    pub const PHOENICIAN_LETTER_HET: char = '𐤇';
    /// \u{10908}: '𐤈'
    pub const PHOENICIAN_LETTER_TET: char = '𐤈';
    /// \u{10909}: '𐤉'
    pub const PHOENICIAN_LETTER_YOD: char = '𐤉';
    /// \u{1090a}: '𐤊'
    pub const PHOENICIAN_LETTER_KAF: char = '𐤊';
    /// \u{1090b}: '𐤋'
    pub const PHOENICIAN_LETTER_LAMD: char = '𐤋';
    /// \u{1090c}: '𐤌'
    pub const PHOENICIAN_LETTER_MEM: char = '𐤌';
    /// \u{1090d}: '𐤍'
    pub const PHOENICIAN_LETTER_NUN: char = '𐤍';
    /// \u{1090e}: '𐤎'
    pub const PHOENICIAN_LETTER_SEMK: char = '𐤎';
    /// \u{1090f}: '𐤏'
    pub const PHOENICIAN_LETTER_AIN: char = '𐤏';
    /// \u{10910}: '𐤐'
    pub const PHOENICIAN_LETTER_PE: char = '𐤐';
    /// \u{10911}: '𐤑'
    pub const PHOENICIAN_LETTER_SADE: char = '𐤑';
    /// \u{10912}: '𐤒'
    pub const PHOENICIAN_LETTER_QOF: char = '𐤒';
    /// \u{10913}: '𐤓'
    pub const PHOENICIAN_LETTER_ROSH: char = '𐤓';
    /// \u{10914}: '𐤔'
    pub const PHOENICIAN_LETTER_SHIN: char = '𐤔';
    /// \u{10915}: '𐤕'
    pub const PHOENICIAN_LETTER_TAU: char = '𐤕';
    /// \u{10916}: '𐤖'
    pub const PHOENICIAN_NUMBER_ONE: char = '𐤖';
    /// \u{10917}: '𐤗'
    pub const PHOENICIAN_NUMBER_TEN: char = '𐤗';
    /// \u{10918}: '𐤘'
    pub const PHOENICIAN_NUMBER_TWENTY: char = '𐤘';
    /// \u{10919}: '𐤙'
    pub const PHOENICIAN_NUMBER_ONE_HUNDRED: char = '𐤙';
    /// \u{1091a}: '𐤚'
    pub const PHOENICIAN_NUMBER_TWO: char = '𐤚';
    /// \u{1091b}: '𐤛'
    pub const PHOENICIAN_NUMBER_THREE: char = '𐤛';
}

/// \u{10900} → \u{1091f}\
///\
/// 𐤀 𐤁 𐤂 𐤃 𐤄 𐤅 𐤆 𐤇 𐤈 𐤉 𐤊 𐤋 𐤌 𐤍 𐤎 𐤏
/// 𐤐 𐤑 𐤒 𐤓 𐤔 𐤕 𐤖 𐤗 𐤘 𐤙 𐤚 𐤛
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Phoenician {
    /// \u{10900}: '𐤀'
    PhoenicianLetterAlf,
    /// \u{10901}: '𐤁'
    PhoenicianLetterBet,
    /// \u{10902}: '𐤂'
    PhoenicianLetterGaml,
    /// \u{10903}: '𐤃'
    PhoenicianLetterDelt,
    /// \u{10904}: '𐤄'
    PhoenicianLetterHe,
    /// \u{10905}: '𐤅'
    PhoenicianLetterWau,
    /// \u{10906}: '𐤆'
    PhoenicianLetterZai,
    /// \u{10907}: '𐤇'
    PhoenicianLetterHet,
    /// \u{10908}: '𐤈'
    PhoenicianLetterTet,
    /// \u{10909}: '𐤉'
    PhoenicianLetterYod,
    /// \u{1090a}: '𐤊'
    PhoenicianLetterKaf,
    /// \u{1090b}: '𐤋'
    PhoenicianLetterLamd,
    /// \u{1090c}: '𐤌'
    PhoenicianLetterMem,
    /// \u{1090d}: '𐤍'
    PhoenicianLetterNun,
    /// \u{1090e}: '𐤎'
    PhoenicianLetterSemk,
    /// \u{1090f}: '𐤏'
    PhoenicianLetterAin,
    /// \u{10910}: '𐤐'
    PhoenicianLetterPe,
    /// \u{10911}: '𐤑'
    PhoenicianLetterSade,
    /// \u{10912}: '𐤒'
    PhoenicianLetterQof,
    /// \u{10913}: '𐤓'
    PhoenicianLetterRosh,
    /// \u{10914}: '𐤔'
    PhoenicianLetterShin,
    /// \u{10915}: '𐤕'
    PhoenicianLetterTau,
    /// \u{10916}: '𐤖'
    PhoenicianNumberOne,
    /// \u{10917}: '𐤗'
    PhoenicianNumberTen,
    /// \u{10918}: '𐤘'
    PhoenicianNumberTwenty,
    /// \u{10919}: '𐤙'
    PhoenicianNumberOneHundred,
    /// \u{1091a}: '𐤚'
    PhoenicianNumberTwo,
    /// \u{1091b}: '𐤛'
    PhoenicianNumberThree,
}

impl Into<char> for Phoenician {
    fn into(self) -> char {
        use constants::*;
        match self {
            Phoenician::PhoenicianLetterAlf => PHOENICIAN_LETTER_ALF,
            Phoenician::PhoenicianLetterBet => PHOENICIAN_LETTER_BET,
            Phoenician::PhoenicianLetterGaml => PHOENICIAN_LETTER_GAML,
            Phoenician::PhoenicianLetterDelt => PHOENICIAN_LETTER_DELT,
            Phoenician::PhoenicianLetterHe => PHOENICIAN_LETTER_HE,
            Phoenician::PhoenicianLetterWau => PHOENICIAN_LETTER_WAU,
            Phoenician::PhoenicianLetterZai => PHOENICIAN_LETTER_ZAI,
            Phoenician::PhoenicianLetterHet => PHOENICIAN_LETTER_HET,
            Phoenician::PhoenicianLetterTet => PHOENICIAN_LETTER_TET,
            Phoenician::PhoenicianLetterYod => PHOENICIAN_LETTER_YOD,
            Phoenician::PhoenicianLetterKaf => PHOENICIAN_LETTER_KAF,
            Phoenician::PhoenicianLetterLamd => PHOENICIAN_LETTER_LAMD,
            Phoenician::PhoenicianLetterMem => PHOENICIAN_LETTER_MEM,
            Phoenician::PhoenicianLetterNun => PHOENICIAN_LETTER_NUN,
            Phoenician::PhoenicianLetterSemk => PHOENICIAN_LETTER_SEMK,
            Phoenician::PhoenicianLetterAin => PHOENICIAN_LETTER_AIN,
            Phoenician::PhoenicianLetterPe => PHOENICIAN_LETTER_PE,
            Phoenician::PhoenicianLetterSade => PHOENICIAN_LETTER_SADE,
            Phoenician::PhoenicianLetterQof => PHOENICIAN_LETTER_QOF,
            Phoenician::PhoenicianLetterRosh => PHOENICIAN_LETTER_ROSH,
            Phoenician::PhoenicianLetterShin => PHOENICIAN_LETTER_SHIN,
            Phoenician::PhoenicianLetterTau => PHOENICIAN_LETTER_TAU,
            Phoenician::PhoenicianNumberOne => PHOENICIAN_NUMBER_ONE,
            Phoenician::PhoenicianNumberTen => PHOENICIAN_NUMBER_TEN,
            Phoenician::PhoenicianNumberTwenty => PHOENICIAN_NUMBER_TWENTY,
            Phoenician::PhoenicianNumberOneHundred => PHOENICIAN_NUMBER_ONE_HUNDRED,
            Phoenician::PhoenicianNumberTwo => PHOENICIAN_NUMBER_TWO,
            Phoenician::PhoenicianNumberThree => PHOENICIAN_NUMBER_THREE,
        }
    }
}

impl std::convert::TryFrom<char> for Phoenician {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            PHOENICIAN_LETTER_ALF => Ok(Phoenician::PhoenicianLetterAlf),
            PHOENICIAN_LETTER_BET => Ok(Phoenician::PhoenicianLetterBet),
            PHOENICIAN_LETTER_GAML => Ok(Phoenician::PhoenicianLetterGaml),
            PHOENICIAN_LETTER_DELT => Ok(Phoenician::PhoenicianLetterDelt),
            PHOENICIAN_LETTER_HE => Ok(Phoenician::PhoenicianLetterHe),
            PHOENICIAN_LETTER_WAU => Ok(Phoenician::PhoenicianLetterWau),
            PHOENICIAN_LETTER_ZAI => Ok(Phoenician::PhoenicianLetterZai),
            PHOENICIAN_LETTER_HET => Ok(Phoenician::PhoenicianLetterHet),
            PHOENICIAN_LETTER_TET => Ok(Phoenician::PhoenicianLetterTet),
            PHOENICIAN_LETTER_YOD => Ok(Phoenician::PhoenicianLetterYod),
            PHOENICIAN_LETTER_KAF => Ok(Phoenician::PhoenicianLetterKaf),
            PHOENICIAN_LETTER_LAMD => Ok(Phoenician::PhoenicianLetterLamd),
            PHOENICIAN_LETTER_MEM => Ok(Phoenician::PhoenicianLetterMem),
            PHOENICIAN_LETTER_NUN => Ok(Phoenician::PhoenicianLetterNun),
            PHOENICIAN_LETTER_SEMK => Ok(Phoenician::PhoenicianLetterSemk),
            PHOENICIAN_LETTER_AIN => Ok(Phoenician::PhoenicianLetterAin),
            PHOENICIAN_LETTER_PE => Ok(Phoenician::PhoenicianLetterPe),
            PHOENICIAN_LETTER_SADE => Ok(Phoenician::PhoenicianLetterSade),
            PHOENICIAN_LETTER_QOF => Ok(Phoenician::PhoenicianLetterQof),
            PHOENICIAN_LETTER_ROSH => Ok(Phoenician::PhoenicianLetterRosh),
            PHOENICIAN_LETTER_SHIN => Ok(Phoenician::PhoenicianLetterShin),
            PHOENICIAN_LETTER_TAU => Ok(Phoenician::PhoenicianLetterTau),
            PHOENICIAN_NUMBER_ONE => Ok(Phoenician::PhoenicianNumberOne),
            PHOENICIAN_NUMBER_TEN => Ok(Phoenician::PhoenicianNumberTen),
            PHOENICIAN_NUMBER_TWENTY => Ok(Phoenician::PhoenicianNumberTwenty),
            PHOENICIAN_NUMBER_ONE_HUNDRED => Ok(Phoenician::PhoenicianNumberOneHundred),
            PHOENICIAN_NUMBER_TWO => Ok(Phoenician::PhoenicianNumberTwo),
            PHOENICIAN_NUMBER_THREE => Ok(Phoenician::PhoenicianNumberThree),
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
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        Phoenician::PhoenicianLetterAlf
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Phoenician::PhoenicianLetterAlf => "phoenician letter alf",
            Phoenician::PhoenicianLetterBet => "phoenician letter bet",
            Phoenician::PhoenicianLetterGaml => "phoenician letter gaml",
            Phoenician::PhoenicianLetterDelt => "phoenician letter delt",
            Phoenician::PhoenicianLetterHe => "phoenician letter he",
            Phoenician::PhoenicianLetterWau => "phoenician letter wau",
            Phoenician::PhoenicianLetterZai => "phoenician letter zai",
            Phoenician::PhoenicianLetterHet => "phoenician letter het",
            Phoenician::PhoenicianLetterTet => "phoenician letter tet",
            Phoenician::PhoenicianLetterYod => "phoenician letter yod",
            Phoenician::PhoenicianLetterKaf => "phoenician letter kaf",
            Phoenician::PhoenicianLetterLamd => "phoenician letter lamd",
            Phoenician::PhoenicianLetterMem => "phoenician letter mem",
            Phoenician::PhoenicianLetterNun => "phoenician letter nun",
            Phoenician::PhoenicianLetterSemk => "phoenician letter semk",
            Phoenician::PhoenicianLetterAin => "phoenician letter ain",
            Phoenician::PhoenicianLetterPe => "phoenician letter pe",
            Phoenician::PhoenicianLetterSade => "phoenician letter sade",
            Phoenician::PhoenicianLetterQof => "phoenician letter qof",
            Phoenician::PhoenicianLetterRosh => "phoenician letter rosh",
            Phoenician::PhoenicianLetterShin => "phoenician letter shin",
            Phoenician::PhoenicianLetterTau => "phoenician letter tau",
            Phoenician::PhoenicianNumberOne => "phoenician number one",
            Phoenician::PhoenicianNumberTen => "phoenician number ten",
            Phoenician::PhoenicianNumberTwenty => "phoenician number twenty",
            Phoenician::PhoenicianNumberOneHundred => "phoenician number one hundred",
            Phoenician::PhoenicianNumberTwo => "phoenician number two",
            Phoenician::PhoenicianNumberThree => "phoenician number three",
        }
    }
}
