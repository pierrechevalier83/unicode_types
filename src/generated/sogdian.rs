/// \u{10f30} → \u{10f6f}\
///\
/// 𐼰 𐼱 𐼲 𐼳 𐼴 𐼵 𐼶 𐼷 𐼸 𐼹 𐼺 𐼻 𐼼 𐼽 𐼾 𐼿
/// 𐽀 𐽁 𐽂 𐽃 𐽄 𐽅 𐽆 𐽇 𐽈 𐽉 𐽊 𐽋 𐽌 𐽍 𐽎 𐽏
/// 𐽐 𐽑 𐽒 𐽓 𐽔 𐽕 𐽖 𐽗 𐽘 𐽙
pub mod constants {
    /// \u{10f30}: '𐼰'
    pub const LETTER_ALEPH: char = '𐼰';
    /// \u{10f31}: '𐼱'
    pub const LETTER_BETH: char = '𐼱';
    /// \u{10f32}: '𐼲'
    pub const LETTER_GIMEL: char = '𐼲';
    /// \u{10f33}: '𐼳'
    pub const LETTER_HE: char = '𐼳';
    /// \u{10f34}: '𐼴'
    pub const LETTER_WAW: char = '𐼴';
    /// \u{10f35}: '𐼵'
    pub const LETTER_ZAYIN: char = '𐼵';
    /// \u{10f36}: '𐼶'
    pub const LETTER_HETH: char = '𐼶';
    /// \u{10f37}: '𐼷'
    pub const LETTER_YODH: char = '𐼷';
    /// \u{10f38}: '𐼸'
    pub const LETTER_KAPH: char = '𐼸';
    /// \u{10f39}: '𐼹'
    pub const LETTER_LAMEDH: char = '𐼹';
    /// \u{10f3a}: '𐼺'
    pub const LETTER_MEM: char = '𐼺';
    /// \u{10f3b}: '𐼻'
    pub const LETTER_NUN: char = '𐼻';
    /// \u{10f3c}: '𐼼'
    pub const LETTER_SAMEKH: char = '𐼼';
    /// \u{10f3d}: '𐼽'
    pub const LETTER_AYIN: char = '𐼽';
    /// \u{10f3e}: '𐼾'
    pub const LETTER_PE: char = '𐼾';
    /// \u{10f3f}: '𐼿'
    pub const LETTER_SADHE: char = '𐼿';
    /// \u{10f40}: '𐽀'
    pub const LETTER_RESH_DASH_AYIN: char = '𐽀';
    /// \u{10f41}: '𐽁'
    pub const LETTER_SHIN: char = '𐽁';
    /// \u{10f42}: '𐽂'
    pub const LETTER_TAW: char = '𐽂';
    /// \u{10f43}: '𐽃'
    pub const LETTER_FETH: char = '𐽃';
    /// \u{10f44}: '𐽄'
    pub const LETTER_LESH: char = '𐽄';
    /// \u{10f45}: '𐽅'
    pub const INDEPENDENT_SHIN: char = '𐽅';
    /// \u{10f46}: '𐽆'
    pub const COMBINING_DOT_BELOW: char = '𐽆';
    /// \u{10f47}: '𐽇'
    pub const COMBINING_TWO_DOTS_BELOW: char = '𐽇';
    /// \u{10f48}: '𐽈'
    pub const COMBINING_DOT_ABOVE: char = '𐽈';
    /// \u{10f49}: '𐽉'
    pub const COMBINING_TWO_DOTS_ABOVE: char = '𐽉';
    /// \u{10f4a}: '𐽊'
    pub const COMBINING_CURVE_ABOVE: char = '𐽊';
    /// \u{10f4b}: '𐽋'
    pub const COMBINING_CURVE_BELOW: char = '𐽋';
    /// \u{10f4c}: '𐽌'
    pub const COMBINING_HOOK_ABOVE: char = '𐽌';
    /// \u{10f4d}: '𐽍'
    pub const COMBINING_HOOK_BELOW: char = '𐽍';
    /// \u{10f4e}: '𐽎'
    pub const COMBINING_LONG_HOOK_BELOW: char = '𐽎';
    /// \u{10f4f}: '𐽏'
    pub const COMBINING_RESH_BELOW: char = '𐽏';
    /// \u{10f50}: '𐽐'
    pub const COMBINING_STROKE_BELOW: char = '𐽐';
    /// \u{10f51}: '𐽑'
    pub const NUMBER_ONE: char = '𐽑';
    /// \u{10f52}: '𐽒'
    pub const NUMBER_TEN: char = '𐽒';
    /// \u{10f53}: '𐽓'
    pub const NUMBER_TWENTY: char = '𐽓';
    /// \u{10f54}: '𐽔'
    pub const NUMBER_ONE_HUNDRED: char = '𐽔';
    /// \u{10f55}: '𐽕'
    pub const PUNCTUATION_TWO_VERTICAL_BARS: char = '𐽕';
    /// \u{10f56}: '𐽖'
    pub const PUNCTUATION_TWO_VERTICAL_BARS_WITH_DOTS: char = '𐽖';
    /// \u{10f57}: '𐽗'
    pub const PUNCTUATION_CIRCLE_WITH_DOT: char = '𐽗';
    /// \u{10f58}: '𐽘'
    pub const PUNCTUATION_TWO_CIRCLES_WITH_DOTS: char = '𐽘';
    /// \u{10f59}: '𐽙'
    pub const PUNCTUATION_HALF_CIRCLE_WITH_DOT: char = '𐽙';
}

/// \u{10f30} → \u{10f6f}\
///\
/// 𐼰 𐼱 𐼲 𐼳 𐼴 𐼵 𐼶 𐼷 𐼸 𐼹 𐼺 𐼻 𐼼 𐼽 𐼾 𐼿
/// 𐽀 𐽁 𐽂 𐽃 𐽄 𐽅 𐽆 𐽇 𐽈 𐽉 𐽊 𐽋 𐽌 𐽍 𐽎 𐽏
/// 𐽐 𐽑 𐽒 𐽓 𐽔 𐽕 𐽖 𐽗 𐽘 𐽙
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Sogdian {
    /// \u{10f30}: '𐼰'
    LetterAleph,
    /// \u{10f31}: '𐼱'
    LetterBeth,
    /// \u{10f32}: '𐼲'
    LetterGimel,
    /// \u{10f33}: '𐼳'
    LetterHe,
    /// \u{10f34}: '𐼴'
    LetterWaw,
    /// \u{10f35}: '𐼵'
    LetterZayin,
    /// \u{10f36}: '𐼶'
    LetterHeth,
    /// \u{10f37}: '𐼷'
    LetterYodh,
    /// \u{10f38}: '𐼸'
    LetterKaph,
    /// \u{10f39}: '𐼹'
    LetterLamedh,
    /// \u{10f3a}: '𐼺'
    LetterMem,
    /// \u{10f3b}: '𐼻'
    LetterNun,
    /// \u{10f3c}: '𐼼'
    LetterSamekh,
    /// \u{10f3d}: '𐼽'
    LetterAyin,
    /// \u{10f3e}: '𐼾'
    LetterPe,
    /// \u{10f3f}: '𐼿'
    LetterSadhe,
    /// \u{10f40}: '𐽀'
    LetterReshDashAyin,
    /// \u{10f41}: '𐽁'
    LetterShin,
    /// \u{10f42}: '𐽂'
    LetterTaw,
    /// \u{10f43}: '𐽃'
    LetterFeth,
    /// \u{10f44}: '𐽄'
    LetterLesh,
    /// \u{10f45}: '𐽅'
    IndependentShin,
    /// \u{10f46}: '𐽆'
    CombiningDotBelow,
    /// \u{10f47}: '𐽇'
    CombiningTwoDotsBelow,
    /// \u{10f48}: '𐽈'
    CombiningDotAbove,
    /// \u{10f49}: '𐽉'
    CombiningTwoDotsAbove,
    /// \u{10f4a}: '𐽊'
    CombiningCurveAbove,
    /// \u{10f4b}: '𐽋'
    CombiningCurveBelow,
    /// \u{10f4c}: '𐽌'
    CombiningHookAbove,
    /// \u{10f4d}: '𐽍'
    CombiningHookBelow,
    /// \u{10f4e}: '𐽎'
    CombiningLongHookBelow,
    /// \u{10f4f}: '𐽏'
    CombiningReshBelow,
    /// \u{10f50}: '𐽐'
    CombiningStrokeBelow,
    /// \u{10f51}: '𐽑'
    NumberOne,
    /// \u{10f52}: '𐽒'
    NumberTen,
    /// \u{10f53}: '𐽓'
    NumberTwenty,
    /// \u{10f54}: '𐽔'
    NumberOneHundred,
    /// \u{10f55}: '𐽕'
    PunctuationTwoVerticalBars,
    /// \u{10f56}: '𐽖'
    PunctuationTwoVerticalBarsWithDots,
    /// \u{10f57}: '𐽗'
    PunctuationCircleWithDot,
    /// \u{10f58}: '𐽘'
    PunctuationTwoCirclesWithDots,
    /// \u{10f59}: '𐽙'
    PunctuationHalfCircleWithDot,
}

impl Into<char> for Sogdian {
    fn into(self) -> char {
        use constants::*;
        match self {
            Sogdian::LetterAleph => LETTER_ALEPH,
            Sogdian::LetterBeth => LETTER_BETH,
            Sogdian::LetterGimel => LETTER_GIMEL,
            Sogdian::LetterHe => LETTER_HE,
            Sogdian::LetterWaw => LETTER_WAW,
            Sogdian::LetterZayin => LETTER_ZAYIN,
            Sogdian::LetterHeth => LETTER_HETH,
            Sogdian::LetterYodh => LETTER_YODH,
            Sogdian::LetterKaph => LETTER_KAPH,
            Sogdian::LetterLamedh => LETTER_LAMEDH,
            Sogdian::LetterMem => LETTER_MEM,
            Sogdian::LetterNun => LETTER_NUN,
            Sogdian::LetterSamekh => LETTER_SAMEKH,
            Sogdian::LetterAyin => LETTER_AYIN,
            Sogdian::LetterPe => LETTER_PE,
            Sogdian::LetterSadhe => LETTER_SADHE,
            Sogdian::LetterReshDashAyin => LETTER_RESH_DASH_AYIN,
            Sogdian::LetterShin => LETTER_SHIN,
            Sogdian::LetterTaw => LETTER_TAW,
            Sogdian::LetterFeth => LETTER_FETH,
            Sogdian::LetterLesh => LETTER_LESH,
            Sogdian::IndependentShin => INDEPENDENT_SHIN,
            Sogdian::CombiningDotBelow => COMBINING_DOT_BELOW,
            Sogdian::CombiningTwoDotsBelow => COMBINING_TWO_DOTS_BELOW,
            Sogdian::CombiningDotAbove => COMBINING_DOT_ABOVE,
            Sogdian::CombiningTwoDotsAbove => COMBINING_TWO_DOTS_ABOVE,
            Sogdian::CombiningCurveAbove => COMBINING_CURVE_ABOVE,
            Sogdian::CombiningCurveBelow => COMBINING_CURVE_BELOW,
            Sogdian::CombiningHookAbove => COMBINING_HOOK_ABOVE,
            Sogdian::CombiningHookBelow => COMBINING_HOOK_BELOW,
            Sogdian::CombiningLongHookBelow => COMBINING_LONG_HOOK_BELOW,
            Sogdian::CombiningReshBelow => COMBINING_RESH_BELOW,
            Sogdian::CombiningStrokeBelow => COMBINING_STROKE_BELOW,
            Sogdian::NumberOne => NUMBER_ONE,
            Sogdian::NumberTen => NUMBER_TEN,
            Sogdian::NumberTwenty => NUMBER_TWENTY,
            Sogdian::NumberOneHundred => NUMBER_ONE_HUNDRED,
            Sogdian::PunctuationTwoVerticalBars => PUNCTUATION_TWO_VERTICAL_BARS,
            Sogdian::PunctuationTwoVerticalBarsWithDots => PUNCTUATION_TWO_VERTICAL_BARS_WITH_DOTS,
            Sogdian::PunctuationCircleWithDot => PUNCTUATION_CIRCLE_WITH_DOT,
            Sogdian::PunctuationTwoCirclesWithDots => PUNCTUATION_TWO_CIRCLES_WITH_DOTS,
            Sogdian::PunctuationHalfCircleWithDot => PUNCTUATION_HALF_CIRCLE_WITH_DOT,
        }
    }
}

impl std::convert::TryFrom<char> for Sogdian {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_ALEPH => Ok(Sogdian::LetterAleph),
            LETTER_BETH => Ok(Sogdian::LetterBeth),
            LETTER_GIMEL => Ok(Sogdian::LetterGimel),
            LETTER_HE => Ok(Sogdian::LetterHe),
            LETTER_WAW => Ok(Sogdian::LetterWaw),
            LETTER_ZAYIN => Ok(Sogdian::LetterZayin),
            LETTER_HETH => Ok(Sogdian::LetterHeth),
            LETTER_YODH => Ok(Sogdian::LetterYodh),
            LETTER_KAPH => Ok(Sogdian::LetterKaph),
            LETTER_LAMEDH => Ok(Sogdian::LetterLamedh),
            LETTER_MEM => Ok(Sogdian::LetterMem),
            LETTER_NUN => Ok(Sogdian::LetterNun),
            LETTER_SAMEKH => Ok(Sogdian::LetterSamekh),
            LETTER_AYIN => Ok(Sogdian::LetterAyin),
            LETTER_PE => Ok(Sogdian::LetterPe),
            LETTER_SADHE => Ok(Sogdian::LetterSadhe),
            LETTER_RESH_DASH_AYIN => Ok(Sogdian::LetterReshDashAyin),
            LETTER_SHIN => Ok(Sogdian::LetterShin),
            LETTER_TAW => Ok(Sogdian::LetterTaw),
            LETTER_FETH => Ok(Sogdian::LetterFeth),
            LETTER_LESH => Ok(Sogdian::LetterLesh),
            INDEPENDENT_SHIN => Ok(Sogdian::IndependentShin),
            COMBINING_DOT_BELOW => Ok(Sogdian::CombiningDotBelow),
            COMBINING_TWO_DOTS_BELOW => Ok(Sogdian::CombiningTwoDotsBelow),
            COMBINING_DOT_ABOVE => Ok(Sogdian::CombiningDotAbove),
            COMBINING_TWO_DOTS_ABOVE => Ok(Sogdian::CombiningTwoDotsAbove),
            COMBINING_CURVE_ABOVE => Ok(Sogdian::CombiningCurveAbove),
            COMBINING_CURVE_BELOW => Ok(Sogdian::CombiningCurveBelow),
            COMBINING_HOOK_ABOVE => Ok(Sogdian::CombiningHookAbove),
            COMBINING_HOOK_BELOW => Ok(Sogdian::CombiningHookBelow),
            COMBINING_LONG_HOOK_BELOW => Ok(Sogdian::CombiningLongHookBelow),
            COMBINING_RESH_BELOW => Ok(Sogdian::CombiningReshBelow),
            COMBINING_STROKE_BELOW => Ok(Sogdian::CombiningStrokeBelow),
            NUMBER_ONE => Ok(Sogdian::NumberOne),
            NUMBER_TEN => Ok(Sogdian::NumberTen),
            NUMBER_TWENTY => Ok(Sogdian::NumberTwenty),
            NUMBER_ONE_HUNDRED => Ok(Sogdian::NumberOneHundred),
            PUNCTUATION_TWO_VERTICAL_BARS => Ok(Sogdian::PunctuationTwoVerticalBars),
            PUNCTUATION_TWO_VERTICAL_BARS_WITH_DOTS => Ok(Sogdian::PunctuationTwoVerticalBarsWithDots),
            PUNCTUATION_CIRCLE_WITH_DOT => Ok(Sogdian::PunctuationCircleWithDot),
            PUNCTUATION_TWO_CIRCLES_WITH_DOTS => Ok(Sogdian::PunctuationTwoCirclesWithDots),
            PUNCTUATION_HALF_CIRCLE_WITH_DOT => Ok(Sogdian::PunctuationHalfCircleWithDot),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Sogdian {
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

impl std::convert::TryFrom<u32> for Sogdian {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Sogdian {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Sogdian {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Sogdian::LetterAleph
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Sogdian::LetterAleph => "sogdian letter aleph",
            Sogdian::LetterBeth => "sogdian letter beth",
            Sogdian::LetterGimel => "sogdian letter gimel",
            Sogdian::LetterHe => "sogdian letter he",
            Sogdian::LetterWaw => "sogdian letter waw",
            Sogdian::LetterZayin => "sogdian letter zayin",
            Sogdian::LetterHeth => "sogdian letter heth",
            Sogdian::LetterYodh => "sogdian letter yodh",
            Sogdian::LetterKaph => "sogdian letter kaph",
            Sogdian::LetterLamedh => "sogdian letter lamedh",
            Sogdian::LetterMem => "sogdian letter mem",
            Sogdian::LetterNun => "sogdian letter nun",
            Sogdian::LetterSamekh => "sogdian letter samekh",
            Sogdian::LetterAyin => "sogdian letter ayin",
            Sogdian::LetterPe => "sogdian letter pe",
            Sogdian::LetterSadhe => "sogdian letter sadhe",
            Sogdian::LetterReshDashAyin => "sogdian letter resh-ayin",
            Sogdian::LetterShin => "sogdian letter shin",
            Sogdian::LetterTaw => "sogdian letter taw",
            Sogdian::LetterFeth => "sogdian letter feth",
            Sogdian::LetterLesh => "sogdian letter lesh",
            Sogdian::IndependentShin => "sogdian independent shin",
            Sogdian::CombiningDotBelow => "sogdian combining dot below",
            Sogdian::CombiningTwoDotsBelow => "sogdian combining two dots below",
            Sogdian::CombiningDotAbove => "sogdian combining dot above",
            Sogdian::CombiningTwoDotsAbove => "sogdian combining two dots above",
            Sogdian::CombiningCurveAbove => "sogdian combining curve above",
            Sogdian::CombiningCurveBelow => "sogdian combining curve below",
            Sogdian::CombiningHookAbove => "sogdian combining hook above",
            Sogdian::CombiningHookBelow => "sogdian combining hook below",
            Sogdian::CombiningLongHookBelow => "sogdian combining long hook below",
            Sogdian::CombiningReshBelow => "sogdian combining resh below",
            Sogdian::CombiningStrokeBelow => "sogdian combining stroke below",
            Sogdian::NumberOne => "sogdian number one",
            Sogdian::NumberTen => "sogdian number ten",
            Sogdian::NumberTwenty => "sogdian number twenty",
            Sogdian::NumberOneHundred => "sogdian number one hundred",
            Sogdian::PunctuationTwoVerticalBars => "sogdian punctuation two vertical bars",
            Sogdian::PunctuationTwoVerticalBarsWithDots => "sogdian punctuation two vertical bars with dots",
            Sogdian::PunctuationCircleWithDot => "sogdian punctuation circle with dot",
            Sogdian::PunctuationTwoCirclesWithDots => "sogdian punctuation two circles with dots",
            Sogdian::PunctuationHalfCircleWithDot => "sogdian punctuation half circle with dot",
        }
    }
}
