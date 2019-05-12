
/// An enum to represent all characters in the Phoenician block.
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
        match self {
            Phoenician::LetterAlf => '𐤀',
            Phoenician::LetterBet => '𐤁',
            Phoenician::LetterGaml => '𐤂',
            Phoenician::LetterDelt => '𐤃',
            Phoenician::LetterHe => '𐤄',
            Phoenician::LetterWau => '𐤅',
            Phoenician::LetterZai => '𐤆',
            Phoenician::LetterHet => '𐤇',
            Phoenician::LetterTet => '𐤈',
            Phoenician::LetterYod => '𐤉',
            Phoenician::LetterKaf => '𐤊',
            Phoenician::LetterLamd => '𐤋',
            Phoenician::LetterMem => '𐤌',
            Phoenician::LetterNun => '𐤍',
            Phoenician::LetterSemk => '𐤎',
            Phoenician::LetterAin => '𐤏',
            Phoenician::LetterPe => '𐤐',
            Phoenician::LetterSade => '𐤑',
            Phoenician::LetterQof => '𐤒',
            Phoenician::LetterRosh => '𐤓',
            Phoenician::LetterShin => '𐤔',
            Phoenician::LetterTau => '𐤕',
            Phoenician::NumberOne => '𐤖',
            Phoenician::NumberTen => '𐤗',
            Phoenician::NumberTwenty => '𐤘',
            Phoenician::NumberOneHundred => '𐤙',
            Phoenician::NumberTwo => '𐤚',
            Phoenician::NumberThree => '𐤛',
        }
    }
}

impl std::convert::TryFrom<char> for Phoenician {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𐤀' => Ok(Phoenician::LetterAlf),
            '𐤁' => Ok(Phoenician::LetterBet),
            '𐤂' => Ok(Phoenician::LetterGaml),
            '𐤃' => Ok(Phoenician::LetterDelt),
            '𐤄' => Ok(Phoenician::LetterHe),
            '𐤅' => Ok(Phoenician::LetterWau),
            '𐤆' => Ok(Phoenician::LetterZai),
            '𐤇' => Ok(Phoenician::LetterHet),
            '𐤈' => Ok(Phoenician::LetterTet),
            '𐤉' => Ok(Phoenician::LetterYod),
            '𐤊' => Ok(Phoenician::LetterKaf),
            '𐤋' => Ok(Phoenician::LetterLamd),
            '𐤌' => Ok(Phoenician::LetterMem),
            '𐤍' => Ok(Phoenician::LetterNun),
            '𐤎' => Ok(Phoenician::LetterSemk),
            '𐤏' => Ok(Phoenician::LetterAin),
            '𐤐' => Ok(Phoenician::LetterPe),
            '𐤑' => Ok(Phoenician::LetterSade),
            '𐤒' => Ok(Phoenician::LetterQof),
            '𐤓' => Ok(Phoenician::LetterRosh),
            '𐤔' => Ok(Phoenician::LetterShin),
            '𐤕' => Ok(Phoenician::LetterTau),
            '𐤖' => Ok(Phoenician::NumberOne),
            '𐤗' => Ok(Phoenician::NumberTen),
            '𐤘' => Ok(Phoenician::NumberTwenty),
            '𐤙' => Ok(Phoenician::NumberOneHundred),
            '𐤚' => Ok(Phoenician::NumberTwo),
            '𐤛' => Ok(Phoenician::NumberThree),
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

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Phoenician{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
