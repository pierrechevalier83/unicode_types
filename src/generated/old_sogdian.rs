
/// An enum to represent all characters in the OldSogdian block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum OldSogdian {
    /// \u{10f00}: '𐼀'
    LetterAleph,
    /// \u{10f01}: '𐼁'
    LetterFinalAleph,
    /// \u{10f02}: '𐼂'
    LetterBeth,
    /// \u{10f03}: '𐼃'
    LetterFinalBeth,
    /// \u{10f04}: '𐼄'
    LetterGimel,
    /// \u{10f05}: '𐼅'
    LetterHe,
    /// \u{10f06}: '𐼆'
    LetterFinalHe,
    /// \u{10f07}: '𐼇'
    LetterWaw,
    /// \u{10f08}: '𐼈'
    LetterZayin,
    /// \u{10f09}: '𐼉'
    LetterHeth,
    /// \u{10f0a}: '𐼊'
    LetterYodh,
    /// \u{10f0b}: '𐼋'
    LetterKaph,
    /// \u{10f0c}: '𐼌'
    LetterLamedh,
    /// \u{10f0d}: '𐼍'
    LetterMem,
    /// \u{10f0e}: '𐼎'
    LetterNun,
    /// \u{10f0f}: '𐼏'
    LetterFinalNun,
    /// \u{10f10}: '𐼐'
    LetterFinalNunWithVerticalTail,
    /// \u{10f11}: '𐼑'
    LetterSamekh,
    /// \u{10f12}: '𐼒'
    LetterAyin,
    /// \u{10f13}: '𐼓'
    LetterAlternateAyin,
    /// \u{10f14}: '𐼔'
    LetterPe,
    /// \u{10f15}: '𐼕'
    LetterSadhe,
    /// \u{10f16}: '𐼖'
    LetterFinalSadhe,
    /// \u{10f17}: '𐼗'
    LetterFinalSadheWithVerticalTail,
    /// \u{10f18}: '𐼘'
    LetterReshDashAyinDashDaleth,
    /// \u{10f19}: '𐼙'
    LetterShin,
    /// \u{10f1a}: '𐼚'
    LetterTaw,
    /// \u{10f1b}: '𐼛'
    LetterFinalTaw,
    /// \u{10f1c}: '𐼜'
    LetterFinalTawWithVerticalTail,
    /// \u{10f1d}: '𐼝'
    NumberOne,
    /// \u{10f1e}: '𐼞'
    NumberTwo,
    /// \u{10f1f}: '𐼟'
    NumberThree,
    /// \u{10f20}: '𐼠'
    NumberFour,
    /// \u{10f21}: '𐼡'
    NumberFive,
    /// \u{10f22}: '𐼢'
    NumberTen,
    /// \u{10f23}: '𐼣'
    NumberTwenty,
    /// \u{10f24}: '𐼤'
    NumberThirty,
    /// \u{10f25}: '𐼥'
    NumberOneHundred,
    /// \u{10f26}: '𐼦'
    FractionOneHalf,
    /// \u{10f27}: '𐼧'
    LigatureAyinDashDaleth,
}

impl Into<char> for OldSogdian {
    fn into(self) -> char {
        match self {
            OldSogdian::LetterAleph => '𐼀',
            OldSogdian::LetterFinalAleph => '𐼁',
            OldSogdian::LetterBeth => '𐼂',
            OldSogdian::LetterFinalBeth => '𐼃',
            OldSogdian::LetterGimel => '𐼄',
            OldSogdian::LetterHe => '𐼅',
            OldSogdian::LetterFinalHe => '𐼆',
            OldSogdian::LetterWaw => '𐼇',
            OldSogdian::LetterZayin => '𐼈',
            OldSogdian::LetterHeth => '𐼉',
            OldSogdian::LetterYodh => '𐼊',
            OldSogdian::LetterKaph => '𐼋',
            OldSogdian::LetterLamedh => '𐼌',
            OldSogdian::LetterMem => '𐼍',
            OldSogdian::LetterNun => '𐼎',
            OldSogdian::LetterFinalNun => '𐼏',
            OldSogdian::LetterFinalNunWithVerticalTail => '𐼐',
            OldSogdian::LetterSamekh => '𐼑',
            OldSogdian::LetterAyin => '𐼒',
            OldSogdian::LetterAlternateAyin => '𐼓',
            OldSogdian::LetterPe => '𐼔',
            OldSogdian::LetterSadhe => '𐼕',
            OldSogdian::LetterFinalSadhe => '𐼖',
            OldSogdian::LetterFinalSadheWithVerticalTail => '𐼗',
            OldSogdian::LetterReshDashAyinDashDaleth => '𐼘',
            OldSogdian::LetterShin => '𐼙',
            OldSogdian::LetterTaw => '𐼚',
            OldSogdian::LetterFinalTaw => '𐼛',
            OldSogdian::LetterFinalTawWithVerticalTail => '𐼜',
            OldSogdian::NumberOne => '𐼝',
            OldSogdian::NumberTwo => '𐼞',
            OldSogdian::NumberThree => '𐼟',
            OldSogdian::NumberFour => '𐼠',
            OldSogdian::NumberFive => '𐼡',
            OldSogdian::NumberTen => '𐼢',
            OldSogdian::NumberTwenty => '𐼣',
            OldSogdian::NumberThirty => '𐼤',
            OldSogdian::NumberOneHundred => '𐼥',
            OldSogdian::FractionOneHalf => '𐼦',
            OldSogdian::LigatureAyinDashDaleth => '𐼧',
        }
    }
}

impl std::convert::TryFrom<char> for OldSogdian {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𐼀' => Ok(OldSogdian::LetterAleph),
            '𐼁' => Ok(OldSogdian::LetterFinalAleph),
            '𐼂' => Ok(OldSogdian::LetterBeth),
            '𐼃' => Ok(OldSogdian::LetterFinalBeth),
            '𐼄' => Ok(OldSogdian::LetterGimel),
            '𐼅' => Ok(OldSogdian::LetterHe),
            '𐼆' => Ok(OldSogdian::LetterFinalHe),
            '𐼇' => Ok(OldSogdian::LetterWaw),
            '𐼈' => Ok(OldSogdian::LetterZayin),
            '𐼉' => Ok(OldSogdian::LetterHeth),
            '𐼊' => Ok(OldSogdian::LetterYodh),
            '𐼋' => Ok(OldSogdian::LetterKaph),
            '𐼌' => Ok(OldSogdian::LetterLamedh),
            '𐼍' => Ok(OldSogdian::LetterMem),
            '𐼎' => Ok(OldSogdian::LetterNun),
            '𐼏' => Ok(OldSogdian::LetterFinalNun),
            '𐼐' => Ok(OldSogdian::LetterFinalNunWithVerticalTail),
            '𐼑' => Ok(OldSogdian::LetterSamekh),
            '𐼒' => Ok(OldSogdian::LetterAyin),
            '𐼓' => Ok(OldSogdian::LetterAlternateAyin),
            '𐼔' => Ok(OldSogdian::LetterPe),
            '𐼕' => Ok(OldSogdian::LetterSadhe),
            '𐼖' => Ok(OldSogdian::LetterFinalSadhe),
            '𐼗' => Ok(OldSogdian::LetterFinalSadheWithVerticalTail),
            '𐼘' => Ok(OldSogdian::LetterReshDashAyinDashDaleth),
            '𐼙' => Ok(OldSogdian::LetterShin),
            '𐼚' => Ok(OldSogdian::LetterTaw),
            '𐼛' => Ok(OldSogdian::LetterFinalTaw),
            '𐼜' => Ok(OldSogdian::LetterFinalTawWithVerticalTail),
            '𐼝' => Ok(OldSogdian::NumberOne),
            '𐼞' => Ok(OldSogdian::NumberTwo),
            '𐼟' => Ok(OldSogdian::NumberThree),
            '𐼠' => Ok(OldSogdian::NumberFour),
            '𐼡' => Ok(OldSogdian::NumberFive),
            '𐼢' => Ok(OldSogdian::NumberTen),
            '𐼣' => Ok(OldSogdian::NumberTwenty),
            '𐼤' => Ok(OldSogdian::NumberThirty),
            '𐼥' => Ok(OldSogdian::NumberOneHundred),
            '𐼦' => Ok(OldSogdian::FractionOneHalf),
            '𐼧' => Ok(OldSogdian::LigatureAyinDashDaleth),
            _ => Err(()),
        }
    }
}

impl Into<u32> for OldSogdian {
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

impl std::convert::TryFrom<u32> for OldSogdian {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for OldSogdian {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl OldSogdian {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        OldSogdian::LetterAleph
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            OldSogdian::LetterAleph => "old sogdian letter aleph",
            OldSogdian::LetterFinalAleph => "old sogdian letter final aleph",
            OldSogdian::LetterBeth => "old sogdian letter beth",
            OldSogdian::LetterFinalBeth => "old sogdian letter final beth",
            OldSogdian::LetterGimel => "old sogdian letter gimel",
            OldSogdian::LetterHe => "old sogdian letter he",
            OldSogdian::LetterFinalHe => "old sogdian letter final he",
            OldSogdian::LetterWaw => "old sogdian letter waw",
            OldSogdian::LetterZayin => "old sogdian letter zayin",
            OldSogdian::LetterHeth => "old sogdian letter heth",
            OldSogdian::LetterYodh => "old sogdian letter yodh",
            OldSogdian::LetterKaph => "old sogdian letter kaph",
            OldSogdian::LetterLamedh => "old sogdian letter lamedh",
            OldSogdian::LetterMem => "old sogdian letter mem",
            OldSogdian::LetterNun => "old sogdian letter nun",
            OldSogdian::LetterFinalNun => "old sogdian letter final nun",
            OldSogdian::LetterFinalNunWithVerticalTail => "old sogdian letter final nun with vertical tail",
            OldSogdian::LetterSamekh => "old sogdian letter samekh",
            OldSogdian::LetterAyin => "old sogdian letter ayin",
            OldSogdian::LetterAlternateAyin => "old sogdian letter alternate ayin",
            OldSogdian::LetterPe => "old sogdian letter pe",
            OldSogdian::LetterSadhe => "old sogdian letter sadhe",
            OldSogdian::LetterFinalSadhe => "old sogdian letter final sadhe",
            OldSogdian::LetterFinalSadheWithVerticalTail => "old sogdian letter final sadhe with vertical tail",
            OldSogdian::LetterReshDashAyinDashDaleth => "old sogdian letter resh-ayin-daleth",
            OldSogdian::LetterShin => "old sogdian letter shin",
            OldSogdian::LetterTaw => "old sogdian letter taw",
            OldSogdian::LetterFinalTaw => "old sogdian letter final taw",
            OldSogdian::LetterFinalTawWithVerticalTail => "old sogdian letter final taw with vertical tail",
            OldSogdian::NumberOne => "old sogdian number one",
            OldSogdian::NumberTwo => "old sogdian number two",
            OldSogdian::NumberThree => "old sogdian number three",
            OldSogdian::NumberFour => "old sogdian number four",
            OldSogdian::NumberFive => "old sogdian number five",
            OldSogdian::NumberTen => "old sogdian number ten",
            OldSogdian::NumberTwenty => "old sogdian number twenty",
            OldSogdian::NumberThirty => "old sogdian number thirty",
            OldSogdian::NumberOneHundred => "old sogdian number one hundred",
            OldSogdian::FractionOneHalf => "old sogdian fraction one half",
            OldSogdian::LigatureAyinDashDaleth => "old sogdian ligature ayin-daleth",
        }
    }
}
