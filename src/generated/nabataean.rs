
/// An enum to represent all characters in the Nabataean block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Nabataean {
    /// \u{10880}: '𐢀'
    LetterFinalAleph,
    /// \u{10881}: '𐢁'
    LetterAleph,
    /// \u{10882}: '𐢂'
    LetterFinalBeth,
    /// \u{10883}: '𐢃'
    LetterBeth,
    /// \u{10884}: '𐢄'
    LetterGimel,
    /// \u{10885}: '𐢅'
    LetterDaleth,
    /// \u{10886}: '𐢆'
    LetterFinalHe,
    /// \u{10887}: '𐢇'
    LetterHe,
    /// \u{10888}: '𐢈'
    LetterWaw,
    /// \u{10889}: '𐢉'
    LetterZayin,
    /// \u{1088a}: '𐢊'
    LetterHeth,
    /// \u{1088b}: '𐢋'
    LetterTeth,
    /// \u{1088c}: '𐢌'
    LetterFinalYodh,
    /// \u{1088d}: '𐢍'
    LetterYodh,
    /// \u{1088e}: '𐢎'
    LetterFinalKaph,
    /// \u{1088f}: '𐢏'
    LetterKaph,
    /// \u{10890}: '𐢐'
    LetterFinalLamedh,
    /// \u{10891}: '𐢑'
    LetterLamedh,
    /// \u{10892}: '𐢒'
    LetterFinalMem,
    /// \u{10893}: '𐢓'
    LetterMem,
    /// \u{10894}: '𐢔'
    LetterFinalNun,
    /// \u{10895}: '𐢕'
    LetterNun,
    /// \u{10896}: '𐢖'
    LetterSamekh,
    /// \u{10897}: '𐢗'
    LetterAyin,
    /// \u{10898}: '𐢘'
    LetterPe,
    /// \u{10899}: '𐢙'
    LetterSadhe,
    /// \u{1089a}: '𐢚'
    LetterQoph,
    /// \u{1089b}: '𐢛'
    LetterResh,
    /// \u{1089c}: '𐢜'
    LetterFinalShin,
    /// \u{1089d}: '𐢝'
    LetterShin,
    /// \u{1089e}: '𐢞'
    LetterTaw,
    /// \u{108a7}: '𐢧'
    NumberOne,
    /// \u{108a8}: '𐢨'
    NumberTwo,
    /// \u{108a9}: '𐢩'
    NumberThree,
    /// \u{108aa}: '𐢪'
    NumberFour,
    /// \u{108ab}: '𐢫'
    CruciformNumberFour,
    /// \u{108ac}: '𐢬'
    NumberFive,
    /// \u{108ad}: '𐢭'
    NumberTen,
    /// \u{108ae}: '𐢮'
    NumberTwenty,
}

impl Into<char> for Nabataean {
    fn into(self) -> char {
        match self {
            Nabataean::LetterFinalAleph => '𐢀',
            Nabataean::LetterAleph => '𐢁',
            Nabataean::LetterFinalBeth => '𐢂',
            Nabataean::LetterBeth => '𐢃',
            Nabataean::LetterGimel => '𐢄',
            Nabataean::LetterDaleth => '𐢅',
            Nabataean::LetterFinalHe => '𐢆',
            Nabataean::LetterHe => '𐢇',
            Nabataean::LetterWaw => '𐢈',
            Nabataean::LetterZayin => '𐢉',
            Nabataean::LetterHeth => '𐢊',
            Nabataean::LetterTeth => '𐢋',
            Nabataean::LetterFinalYodh => '𐢌',
            Nabataean::LetterYodh => '𐢍',
            Nabataean::LetterFinalKaph => '𐢎',
            Nabataean::LetterKaph => '𐢏',
            Nabataean::LetterFinalLamedh => '𐢐',
            Nabataean::LetterLamedh => '𐢑',
            Nabataean::LetterFinalMem => '𐢒',
            Nabataean::LetterMem => '𐢓',
            Nabataean::LetterFinalNun => '𐢔',
            Nabataean::LetterNun => '𐢕',
            Nabataean::LetterSamekh => '𐢖',
            Nabataean::LetterAyin => '𐢗',
            Nabataean::LetterPe => '𐢘',
            Nabataean::LetterSadhe => '𐢙',
            Nabataean::LetterQoph => '𐢚',
            Nabataean::LetterResh => '𐢛',
            Nabataean::LetterFinalShin => '𐢜',
            Nabataean::LetterShin => '𐢝',
            Nabataean::LetterTaw => '𐢞',
            Nabataean::NumberOne => '𐢧',
            Nabataean::NumberTwo => '𐢨',
            Nabataean::NumberThree => '𐢩',
            Nabataean::NumberFour => '𐢪',
            Nabataean::CruciformNumberFour => '𐢫',
            Nabataean::NumberFive => '𐢬',
            Nabataean::NumberTen => '𐢭',
            Nabataean::NumberTwenty => '𐢮',
        }
    }
}

impl std::convert::TryFrom<char> for Nabataean {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𐢀' => Ok(Nabataean::LetterFinalAleph),
            '𐢁' => Ok(Nabataean::LetterAleph),
            '𐢂' => Ok(Nabataean::LetterFinalBeth),
            '𐢃' => Ok(Nabataean::LetterBeth),
            '𐢄' => Ok(Nabataean::LetterGimel),
            '𐢅' => Ok(Nabataean::LetterDaleth),
            '𐢆' => Ok(Nabataean::LetterFinalHe),
            '𐢇' => Ok(Nabataean::LetterHe),
            '𐢈' => Ok(Nabataean::LetterWaw),
            '𐢉' => Ok(Nabataean::LetterZayin),
            '𐢊' => Ok(Nabataean::LetterHeth),
            '𐢋' => Ok(Nabataean::LetterTeth),
            '𐢌' => Ok(Nabataean::LetterFinalYodh),
            '𐢍' => Ok(Nabataean::LetterYodh),
            '𐢎' => Ok(Nabataean::LetterFinalKaph),
            '𐢏' => Ok(Nabataean::LetterKaph),
            '𐢐' => Ok(Nabataean::LetterFinalLamedh),
            '𐢑' => Ok(Nabataean::LetterLamedh),
            '𐢒' => Ok(Nabataean::LetterFinalMem),
            '𐢓' => Ok(Nabataean::LetterMem),
            '𐢔' => Ok(Nabataean::LetterFinalNun),
            '𐢕' => Ok(Nabataean::LetterNun),
            '𐢖' => Ok(Nabataean::LetterSamekh),
            '𐢗' => Ok(Nabataean::LetterAyin),
            '𐢘' => Ok(Nabataean::LetterPe),
            '𐢙' => Ok(Nabataean::LetterSadhe),
            '𐢚' => Ok(Nabataean::LetterQoph),
            '𐢛' => Ok(Nabataean::LetterResh),
            '𐢜' => Ok(Nabataean::LetterFinalShin),
            '𐢝' => Ok(Nabataean::LetterShin),
            '𐢞' => Ok(Nabataean::LetterTaw),
            '𐢧' => Ok(Nabataean::NumberOne),
            '𐢨' => Ok(Nabataean::NumberTwo),
            '𐢩' => Ok(Nabataean::NumberThree),
            '𐢪' => Ok(Nabataean::NumberFour),
            '𐢫' => Ok(Nabataean::CruciformNumberFour),
            '𐢬' => Ok(Nabataean::NumberFive),
            '𐢭' => Ok(Nabataean::NumberTen),
            '𐢮' => Ok(Nabataean::NumberTwenty),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Nabataean {
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

impl std::convert::TryFrom<u32> for Nabataean {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Nabataean {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Nabataean {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Nabataean::LetterFinalAleph
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Nabataean{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
