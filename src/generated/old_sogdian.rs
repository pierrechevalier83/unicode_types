/// \u{10f00} → \u{10f2f}
///
/// 𐼀 𐼁 𐼂 𐼃 𐼄 𐼅 𐼆 𐼇 𐼈 𐼉 𐼊 𐼋 𐼌 𐼍 𐼎 𐼏\
/// 𐼐 𐼑 𐼒 𐼓 𐼔 𐼕 𐼖 𐼗 𐼘 𐼙 𐼚 𐼛 𐼜 𐼝 𐼞 𐼟\
/// 𐼠 𐼡 𐼢 𐼣 𐼤 𐼥 𐼦 𐼧\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{10f00}: '𐼀'
    pub const LETTER_ALEPH: char = '𐼀';
    /// \u{10f01}: '𐼁'
    pub const LETTER_FINAL_ALEPH: char = '𐼁';
    /// \u{10f02}: '𐼂'
    pub const LETTER_BETH: char = '𐼂';
    /// \u{10f03}: '𐼃'
    pub const LETTER_FINAL_BETH: char = '𐼃';
    /// \u{10f04}: '𐼄'
    pub const LETTER_GIMEL: char = '𐼄';
    /// \u{10f05}: '𐼅'
    pub const LETTER_HE: char = '𐼅';
    /// \u{10f06}: '𐼆'
    pub const LETTER_FINAL_HE: char = '𐼆';
    /// \u{10f07}: '𐼇'
    pub const LETTER_WAW: char = '𐼇';
    /// \u{10f08}: '𐼈'
    pub const LETTER_ZAYIN: char = '𐼈';
    /// \u{10f09}: '𐼉'
    pub const LETTER_HETH: char = '𐼉';
    /// \u{10f0a}: '𐼊'
    pub const LETTER_YODH: char = '𐼊';
    /// \u{10f0b}: '𐼋'
    pub const LETTER_KAPH: char = '𐼋';
    /// \u{10f0c}: '𐼌'
    pub const LETTER_LAMEDH: char = '𐼌';
    /// \u{10f0d}: '𐼍'
    pub const LETTER_MEM: char = '𐼍';
    /// \u{10f0e}: '𐼎'
    pub const LETTER_NUN: char = '𐼎';
    /// \u{10f0f}: '𐼏'
    pub const LETTER_FINAL_NUN: char = '𐼏';
    /// \u{10f10}: '𐼐'
    pub const LETTER_FINAL_NUN_WITH_VERTICAL_TAIL: char = '𐼐';
    /// \u{10f11}: '𐼑'
    pub const LETTER_SAMEKH: char = '𐼑';
    /// \u{10f12}: '𐼒'
    pub const LETTER_AYIN: char = '𐼒';
    /// \u{10f13}: '𐼓'
    pub const LETTER_ALTERNATE_AYIN: char = '𐼓';
    /// \u{10f14}: '𐼔'
    pub const LETTER_PE: char = '𐼔';
    /// \u{10f15}: '𐼕'
    pub const LETTER_SADHE: char = '𐼕';
    /// \u{10f16}: '𐼖'
    pub const LETTER_FINAL_SADHE: char = '𐼖';
    /// \u{10f17}: '𐼗'
    pub const LETTER_FINAL_SADHE_WITH_VERTICAL_TAIL: char = '𐼗';
    /// \u{10f18}: '𐼘'
    pub const LETTER_RESH_DASH_AYIN_DASH_DALETH: char = '𐼘';
    /// \u{10f19}: '𐼙'
    pub const LETTER_SHIN: char = '𐼙';
    /// \u{10f1a}: '𐼚'
    pub const LETTER_TAW: char = '𐼚';
    /// \u{10f1b}: '𐼛'
    pub const LETTER_FINAL_TAW: char = '𐼛';
    /// \u{10f1c}: '𐼜'
    pub const LETTER_FINAL_TAW_WITH_VERTICAL_TAIL: char = '𐼜';
    /// \u{10f1d}: '𐼝'
    pub const NUMBER_ONE: char = '𐼝';
    /// \u{10f1e}: '𐼞'
    pub const NUMBER_TWO: char = '𐼞';
    /// \u{10f1f}: '𐼟'
    pub const NUMBER_THREE: char = '𐼟';
    /// \u{10f20}: '𐼠'
    pub const NUMBER_FOUR: char = '𐼠';
    /// \u{10f21}: '𐼡'
    pub const NUMBER_FIVE: char = '𐼡';
    /// \u{10f22}: '𐼢'
    pub const NUMBER_TEN: char = '𐼢';
    /// \u{10f23}: '𐼣'
    pub const NUMBER_TWENTY: char = '𐼣';
    /// \u{10f24}: '𐼤'
    pub const NUMBER_THIRTY: char = '𐼤';
    /// \u{10f25}: '𐼥'
    pub const NUMBER_ONE_HUNDRED: char = '𐼥';
    /// \u{10f26}: '𐼦'
    pub const FRACTION_ONE_HALF: char = '𐼦';
    /// \u{10f27}: '𐼧'
    pub const LIGATURE_AYIN_DASH_DALETH: char = '𐼧';
}

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
        use constants::*;
        match self {
            OldSogdian::LetterAleph => LETTER_ALEPH,
            OldSogdian::LetterFinalAleph => LETTER_FINAL_ALEPH,
            OldSogdian::LetterBeth => LETTER_BETH,
            OldSogdian::LetterFinalBeth => LETTER_FINAL_BETH,
            OldSogdian::LetterGimel => LETTER_GIMEL,
            OldSogdian::LetterHe => LETTER_HE,
            OldSogdian::LetterFinalHe => LETTER_FINAL_HE,
            OldSogdian::LetterWaw => LETTER_WAW,
            OldSogdian::LetterZayin => LETTER_ZAYIN,
            OldSogdian::LetterHeth => LETTER_HETH,
            OldSogdian::LetterYodh => LETTER_YODH,
            OldSogdian::LetterKaph => LETTER_KAPH,
            OldSogdian::LetterLamedh => LETTER_LAMEDH,
            OldSogdian::LetterMem => LETTER_MEM,
            OldSogdian::LetterNun => LETTER_NUN,
            OldSogdian::LetterFinalNun => LETTER_FINAL_NUN,
            OldSogdian::LetterFinalNunWithVerticalTail => LETTER_FINAL_NUN_WITH_VERTICAL_TAIL,
            OldSogdian::LetterSamekh => LETTER_SAMEKH,
            OldSogdian::LetterAyin => LETTER_AYIN,
            OldSogdian::LetterAlternateAyin => LETTER_ALTERNATE_AYIN,
            OldSogdian::LetterPe => LETTER_PE,
            OldSogdian::LetterSadhe => LETTER_SADHE,
            OldSogdian::LetterFinalSadhe => LETTER_FINAL_SADHE,
            OldSogdian::LetterFinalSadheWithVerticalTail => LETTER_FINAL_SADHE_WITH_VERTICAL_TAIL,
            OldSogdian::LetterReshDashAyinDashDaleth => LETTER_RESH_DASH_AYIN_DASH_DALETH,
            OldSogdian::LetterShin => LETTER_SHIN,
            OldSogdian::LetterTaw => LETTER_TAW,
            OldSogdian::LetterFinalTaw => LETTER_FINAL_TAW,
            OldSogdian::LetterFinalTawWithVerticalTail => LETTER_FINAL_TAW_WITH_VERTICAL_TAIL,
            OldSogdian::NumberOne => NUMBER_ONE,
            OldSogdian::NumberTwo => NUMBER_TWO,
            OldSogdian::NumberThree => NUMBER_THREE,
            OldSogdian::NumberFour => NUMBER_FOUR,
            OldSogdian::NumberFive => NUMBER_FIVE,
            OldSogdian::NumberTen => NUMBER_TEN,
            OldSogdian::NumberTwenty => NUMBER_TWENTY,
            OldSogdian::NumberThirty => NUMBER_THIRTY,
            OldSogdian::NumberOneHundred => NUMBER_ONE_HUNDRED,
            OldSogdian::FractionOneHalf => FRACTION_ONE_HALF,
            OldSogdian::LigatureAyinDashDaleth => LIGATURE_AYIN_DASH_DALETH,
        }
    }
}

impl std::convert::TryFrom<char> for OldSogdian {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_ALEPH => Ok(OldSogdian::LetterAleph),
            LETTER_FINAL_ALEPH => Ok(OldSogdian::LetterFinalAleph),
            LETTER_BETH => Ok(OldSogdian::LetterBeth),
            LETTER_FINAL_BETH => Ok(OldSogdian::LetterFinalBeth),
            LETTER_GIMEL => Ok(OldSogdian::LetterGimel),
            LETTER_HE => Ok(OldSogdian::LetterHe),
            LETTER_FINAL_HE => Ok(OldSogdian::LetterFinalHe),
            LETTER_WAW => Ok(OldSogdian::LetterWaw),
            LETTER_ZAYIN => Ok(OldSogdian::LetterZayin),
            LETTER_HETH => Ok(OldSogdian::LetterHeth),
            LETTER_YODH => Ok(OldSogdian::LetterYodh),
            LETTER_KAPH => Ok(OldSogdian::LetterKaph),
            LETTER_LAMEDH => Ok(OldSogdian::LetterLamedh),
            LETTER_MEM => Ok(OldSogdian::LetterMem),
            LETTER_NUN => Ok(OldSogdian::LetterNun),
            LETTER_FINAL_NUN => Ok(OldSogdian::LetterFinalNun),
            LETTER_FINAL_NUN_WITH_VERTICAL_TAIL => Ok(OldSogdian::LetterFinalNunWithVerticalTail),
            LETTER_SAMEKH => Ok(OldSogdian::LetterSamekh),
            LETTER_AYIN => Ok(OldSogdian::LetterAyin),
            LETTER_ALTERNATE_AYIN => Ok(OldSogdian::LetterAlternateAyin),
            LETTER_PE => Ok(OldSogdian::LetterPe),
            LETTER_SADHE => Ok(OldSogdian::LetterSadhe),
            LETTER_FINAL_SADHE => Ok(OldSogdian::LetterFinalSadhe),
            LETTER_FINAL_SADHE_WITH_VERTICAL_TAIL => Ok(OldSogdian::LetterFinalSadheWithVerticalTail),
            LETTER_RESH_DASH_AYIN_DASH_DALETH => Ok(OldSogdian::LetterReshDashAyinDashDaleth),
            LETTER_SHIN => Ok(OldSogdian::LetterShin),
            LETTER_TAW => Ok(OldSogdian::LetterTaw),
            LETTER_FINAL_TAW => Ok(OldSogdian::LetterFinalTaw),
            LETTER_FINAL_TAW_WITH_VERTICAL_TAIL => Ok(OldSogdian::LetterFinalTawWithVerticalTail),
            NUMBER_ONE => Ok(OldSogdian::NumberOne),
            NUMBER_TWO => Ok(OldSogdian::NumberTwo),
            NUMBER_THREE => Ok(OldSogdian::NumberThree),
            NUMBER_FOUR => Ok(OldSogdian::NumberFour),
            NUMBER_FIVE => Ok(OldSogdian::NumberFive),
            NUMBER_TEN => Ok(OldSogdian::NumberTen),
            NUMBER_TWENTY => Ok(OldSogdian::NumberTwenty),
            NUMBER_THIRTY => Ok(OldSogdian::NumberThirty),
            NUMBER_ONE_HUNDRED => Ok(OldSogdian::NumberOneHundred),
            FRACTION_ONE_HALF => Ok(OldSogdian::FractionOneHalf),
            LIGATURE_AYIN_DASH_DALETH => Ok(OldSogdian::LigatureAyinDashDaleth),
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
