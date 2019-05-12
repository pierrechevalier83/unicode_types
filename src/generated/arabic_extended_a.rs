
/// An enum to represent all characters in the ArabicExtendedA block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ArabicExtendedA {
    /// \u{8a0}: 'ࢠ'
    ArabicLetterBehWithSmallVBelow,
    /// \u{8a1}: 'ࢡ'
    ArabicLetterBehWithHamzaAbove,
    /// \u{8a2}: 'ࢢ'
    ArabicLetterJeemWithTwoDotsAbove,
    /// \u{8a3}: 'ࢣ'
    ArabicLetterTahWithTwoDotsAbove,
    /// \u{8a4}: 'ࢤ'
    ArabicLetterFehWithDotBelowAndThreeDotsAbove,
    /// \u{8a5}: 'ࢥ'
    ArabicLetterQafWithDotBelow,
    /// \u{8a6}: 'ࢦ'
    ArabicLetterLamWithDoubleBar,
    /// \u{8a7}: 'ࢧ'
    ArabicLetterMeemWithThreeDotsAbove,
    /// \u{8a8}: 'ࢨ'
    ArabicLetterYehWithTwoDotsBelowAndHamzaAbove,
    /// \u{8a9}: 'ࢩ'
    ArabicLetterYehWithTwoDotsBelowAndDotAbove,
    /// \u{8aa}: 'ࢪ'
    ArabicLetterRehWithLoop,
    /// \u{8ab}: 'ࢫ'
    ArabicLetterWawWithDotWithin,
    /// \u{8ac}: 'ࢬ'
    ArabicLetterRohingyaYeh,
    /// \u{8ad}: 'ࢭ'
    ArabicLetterLowAlef,
    /// \u{8ae}: 'ࢮ'
    ArabicLetterDalWithThreeDotsBelow,
    /// \u{8af}: 'ࢯ'
    ArabicLetterSadWithThreeDotsBelow,
    /// \u{8b0}: 'ࢰ'
    ArabicLetterGafWithInvertedStroke,
    /// \u{8b1}: 'ࢱ'
    ArabicLetterStraightWaw,
    /// \u{8b2}: 'ࢲ'
    ArabicLetterZainWithInvertedVAbove,
    /// \u{8b3}: 'ࢳ'
    ArabicLetterAinWithThreeDotsBelow,
    /// \u{8b4}: 'ࢴ'
    ArabicLetterKafWithDotBelow,
    /// \u{8b6}: 'ࢶ'
    ArabicLetterBehWithSmallMeemAbove,
    /// \u{8b7}: 'ࢷ'
    ArabicLetterPehWithSmallMeemAbove,
    /// \u{8b8}: 'ࢸ'
    ArabicLetterTehWithSmallTehAbove,
    /// \u{8b9}: 'ࢹ'
    ArabicLetterRehWithSmallNoonAbove,
    /// \u{8ba}: 'ࢺ'
    ArabicLetterYehWithTwoDotsBelowAndSmallNoonAbove,
    /// \u{8bb}: 'ࢻ'
    ArabicLetterAfricanFeh,
    /// \u{8bc}: 'ࢼ'
    ArabicLetterAfricanQaf,
    /// \u{8bd}: 'ࢽ'
    ArabicLetterAfricanNoon,
    /// \u{8d3}: '࣓'
    ArabicSmallLowWaw,
    /// \u{8d4}: 'ࣔ'
    ArabicSmallHighWordArDashRub,
    /// \u{8d5}: 'ࣕ'
    ArabicSmallHighSad,
    /// \u{8d6}: 'ࣖ'
    ArabicSmallHighAin,
    /// \u{8d7}: 'ࣗ'
    ArabicSmallHighQaf,
    /// \u{8d8}: 'ࣘ'
    ArabicSmallHighNoonWithKasra,
    /// \u{8d9}: 'ࣙ'
    ArabicSmallLowNoonWithKasra,
    /// \u{8da}: 'ࣚ'
    ArabicSmallHighWordAthDashThalatha,
    /// \u{8db}: 'ࣛ'
    ArabicSmallHighWordAsDashSajda,
    /// \u{8dc}: 'ࣜ'
    ArabicSmallHighWordAnDashNisf,
    /// \u{8dd}: 'ࣝ'
    ArabicSmallHighWordSakta,
    /// \u{8de}: 'ࣞ'
    ArabicSmallHighWordQif,
    /// \u{8df}: 'ࣟ'
    ArabicSmallHighWordWaqfa,
    /// \u{8e0}: '࣠'
    ArabicSmallHighFootnoteMarker,
    /// \u{8e1}: '࣡'
    ArabicSmallHighSignSafha,
    /// \u{8e2}: '࣢'
    ArabicDisputedEndOfAyah,
    /// \u{8e3}: 'ࣣ'
    ArabicTurnedDammaBelow,
    /// \u{8e4}: 'ࣤ'
    ArabicCurlyFatha,
    /// \u{8e5}: 'ࣥ'
    ArabicCurlyDamma,
    /// \u{8e6}: 'ࣦ'
    ArabicCurlyKasra,
    /// \u{8e7}: 'ࣧ'
    ArabicCurlyFathatan,
    /// \u{8e8}: 'ࣨ'
    ArabicCurlyDammatan,
    /// \u{8e9}: 'ࣩ'
    ArabicCurlyKasratan,
    /// \u{8ea}: '࣪'
    ArabicToneOneDotAbove,
    /// \u{8eb}: '࣫'
    ArabicToneTwoDotsAbove,
    /// \u{8ec}: '࣬'
    ArabicToneLoopAbove,
    /// \u{8ed}: '࣭'
    ArabicToneOneDotBelow,
    /// \u{8ee}: '࣮'
    ArabicToneTwoDotsBelow,
    /// \u{8ef}: '࣯'
    ArabicToneLoopBelow,
    /// \u{8f0}: 'ࣰ'
    ArabicOpenFathatan,
    /// \u{8f1}: 'ࣱ'
    ArabicOpenDammatan,
    /// \u{8f2}: 'ࣲ'
    ArabicOpenKasratan,
    /// \u{8f3}: 'ࣳ'
    ArabicSmallHighWaw,
    /// \u{8f4}: 'ࣴ'
    ArabicFathaWithRing,
    /// \u{8f5}: 'ࣵ'
    ArabicFathaWithDotAbove,
    /// \u{8f6}: 'ࣶ'
    ArabicKasraWithDotBelow,
    /// \u{8f7}: 'ࣷ'
    ArabicLeftArrowheadAbove,
    /// \u{8f8}: 'ࣸ'
    ArabicRightArrowheadAbove,
    /// \u{8f9}: 'ࣹ'
    ArabicLeftArrowheadBelow,
    /// \u{8fa}: 'ࣺ'
    ArabicRightArrowheadBelow,
    /// \u{8fb}: 'ࣻ'
    ArabicDoubleRightArrowheadAbove,
    /// \u{8fc}: 'ࣼ'
    ArabicDoubleRightArrowheadAboveWithDot,
    /// \u{8fd}: 'ࣽ'
    ArabicRightArrowheadAboveWithDot,
    /// \u{8fe}: 'ࣾ'
    ArabicDammaWithDot,
}

impl Into<char> for ArabicExtendedA {
    fn into(self) -> char {
        match self {
            ArabicExtendedA::ArabicLetterBehWithSmallVBelow => 'ࢠ',
            ArabicExtendedA::ArabicLetterBehWithHamzaAbove => 'ࢡ',
            ArabicExtendedA::ArabicLetterJeemWithTwoDotsAbove => 'ࢢ',
            ArabicExtendedA::ArabicLetterTahWithTwoDotsAbove => 'ࢣ',
            ArabicExtendedA::ArabicLetterFehWithDotBelowAndThreeDotsAbove => 'ࢤ',
            ArabicExtendedA::ArabicLetterQafWithDotBelow => 'ࢥ',
            ArabicExtendedA::ArabicLetterLamWithDoubleBar => 'ࢦ',
            ArabicExtendedA::ArabicLetterMeemWithThreeDotsAbove => 'ࢧ',
            ArabicExtendedA::ArabicLetterYehWithTwoDotsBelowAndHamzaAbove => 'ࢨ',
            ArabicExtendedA::ArabicLetterYehWithTwoDotsBelowAndDotAbove => 'ࢩ',
            ArabicExtendedA::ArabicLetterRehWithLoop => 'ࢪ',
            ArabicExtendedA::ArabicLetterWawWithDotWithin => 'ࢫ',
            ArabicExtendedA::ArabicLetterRohingyaYeh => 'ࢬ',
            ArabicExtendedA::ArabicLetterLowAlef => 'ࢭ',
            ArabicExtendedA::ArabicLetterDalWithThreeDotsBelow => 'ࢮ',
            ArabicExtendedA::ArabicLetterSadWithThreeDotsBelow => 'ࢯ',
            ArabicExtendedA::ArabicLetterGafWithInvertedStroke => 'ࢰ',
            ArabicExtendedA::ArabicLetterStraightWaw => 'ࢱ',
            ArabicExtendedA::ArabicLetterZainWithInvertedVAbove => 'ࢲ',
            ArabicExtendedA::ArabicLetterAinWithThreeDotsBelow => 'ࢳ',
            ArabicExtendedA::ArabicLetterKafWithDotBelow => 'ࢴ',
            ArabicExtendedA::ArabicLetterBehWithSmallMeemAbove => 'ࢶ',
            ArabicExtendedA::ArabicLetterPehWithSmallMeemAbove => 'ࢷ',
            ArabicExtendedA::ArabicLetterTehWithSmallTehAbove => 'ࢸ',
            ArabicExtendedA::ArabicLetterRehWithSmallNoonAbove => 'ࢹ',
            ArabicExtendedA::ArabicLetterYehWithTwoDotsBelowAndSmallNoonAbove => 'ࢺ',
            ArabicExtendedA::ArabicLetterAfricanFeh => 'ࢻ',
            ArabicExtendedA::ArabicLetterAfricanQaf => 'ࢼ',
            ArabicExtendedA::ArabicLetterAfricanNoon => 'ࢽ',
            ArabicExtendedA::ArabicSmallLowWaw => '࣓',
            ArabicExtendedA::ArabicSmallHighWordArDashRub => 'ࣔ',
            ArabicExtendedA::ArabicSmallHighSad => 'ࣕ',
            ArabicExtendedA::ArabicSmallHighAin => 'ࣖ',
            ArabicExtendedA::ArabicSmallHighQaf => 'ࣗ',
            ArabicExtendedA::ArabicSmallHighNoonWithKasra => 'ࣘ',
            ArabicExtendedA::ArabicSmallLowNoonWithKasra => 'ࣙ',
            ArabicExtendedA::ArabicSmallHighWordAthDashThalatha => 'ࣚ',
            ArabicExtendedA::ArabicSmallHighWordAsDashSajda => 'ࣛ',
            ArabicExtendedA::ArabicSmallHighWordAnDashNisf => 'ࣜ',
            ArabicExtendedA::ArabicSmallHighWordSakta => 'ࣝ',
            ArabicExtendedA::ArabicSmallHighWordQif => 'ࣞ',
            ArabicExtendedA::ArabicSmallHighWordWaqfa => 'ࣟ',
            ArabicExtendedA::ArabicSmallHighFootnoteMarker => '࣠',
            ArabicExtendedA::ArabicSmallHighSignSafha => '࣡',
            ArabicExtendedA::ArabicDisputedEndOfAyah => '࣢',
            ArabicExtendedA::ArabicTurnedDammaBelow => 'ࣣ',
            ArabicExtendedA::ArabicCurlyFatha => 'ࣤ',
            ArabicExtendedA::ArabicCurlyDamma => 'ࣥ',
            ArabicExtendedA::ArabicCurlyKasra => 'ࣦ',
            ArabicExtendedA::ArabicCurlyFathatan => 'ࣧ',
            ArabicExtendedA::ArabicCurlyDammatan => 'ࣨ',
            ArabicExtendedA::ArabicCurlyKasratan => 'ࣩ',
            ArabicExtendedA::ArabicToneOneDotAbove => '࣪',
            ArabicExtendedA::ArabicToneTwoDotsAbove => '࣫',
            ArabicExtendedA::ArabicToneLoopAbove => '࣬',
            ArabicExtendedA::ArabicToneOneDotBelow => '࣭',
            ArabicExtendedA::ArabicToneTwoDotsBelow => '࣮',
            ArabicExtendedA::ArabicToneLoopBelow => '࣯',
            ArabicExtendedA::ArabicOpenFathatan => 'ࣰ',
            ArabicExtendedA::ArabicOpenDammatan => 'ࣱ',
            ArabicExtendedA::ArabicOpenKasratan => 'ࣲ',
            ArabicExtendedA::ArabicSmallHighWaw => 'ࣳ',
            ArabicExtendedA::ArabicFathaWithRing => 'ࣴ',
            ArabicExtendedA::ArabicFathaWithDotAbove => 'ࣵ',
            ArabicExtendedA::ArabicKasraWithDotBelow => 'ࣶ',
            ArabicExtendedA::ArabicLeftArrowheadAbove => 'ࣷ',
            ArabicExtendedA::ArabicRightArrowheadAbove => 'ࣸ',
            ArabicExtendedA::ArabicLeftArrowheadBelow => 'ࣹ',
            ArabicExtendedA::ArabicRightArrowheadBelow => 'ࣺ',
            ArabicExtendedA::ArabicDoubleRightArrowheadAbove => 'ࣻ',
            ArabicExtendedA::ArabicDoubleRightArrowheadAboveWithDot => 'ࣼ',
            ArabicExtendedA::ArabicRightArrowheadAboveWithDot => 'ࣽ',
            ArabicExtendedA::ArabicDammaWithDot => 'ࣾ',
        }
    }
}

impl std::convert::TryFrom<char> for ArabicExtendedA {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ࢠ' => Ok(ArabicExtendedA::ArabicLetterBehWithSmallVBelow),
            'ࢡ' => Ok(ArabicExtendedA::ArabicLetterBehWithHamzaAbove),
            'ࢢ' => Ok(ArabicExtendedA::ArabicLetterJeemWithTwoDotsAbove),
            'ࢣ' => Ok(ArabicExtendedA::ArabicLetterTahWithTwoDotsAbove),
            'ࢤ' => Ok(ArabicExtendedA::ArabicLetterFehWithDotBelowAndThreeDotsAbove),
            'ࢥ' => Ok(ArabicExtendedA::ArabicLetterQafWithDotBelow),
            'ࢦ' => Ok(ArabicExtendedA::ArabicLetterLamWithDoubleBar),
            'ࢧ' => Ok(ArabicExtendedA::ArabicLetterMeemWithThreeDotsAbove),
            'ࢨ' => Ok(ArabicExtendedA::ArabicLetterYehWithTwoDotsBelowAndHamzaAbove),
            'ࢩ' => Ok(ArabicExtendedA::ArabicLetterYehWithTwoDotsBelowAndDotAbove),
            'ࢪ' => Ok(ArabicExtendedA::ArabicLetterRehWithLoop),
            'ࢫ' => Ok(ArabicExtendedA::ArabicLetterWawWithDotWithin),
            'ࢬ' => Ok(ArabicExtendedA::ArabicLetterRohingyaYeh),
            'ࢭ' => Ok(ArabicExtendedA::ArabicLetterLowAlef),
            'ࢮ' => Ok(ArabicExtendedA::ArabicLetterDalWithThreeDotsBelow),
            'ࢯ' => Ok(ArabicExtendedA::ArabicLetterSadWithThreeDotsBelow),
            'ࢰ' => Ok(ArabicExtendedA::ArabicLetterGafWithInvertedStroke),
            'ࢱ' => Ok(ArabicExtendedA::ArabicLetterStraightWaw),
            'ࢲ' => Ok(ArabicExtendedA::ArabicLetterZainWithInvertedVAbove),
            'ࢳ' => Ok(ArabicExtendedA::ArabicLetterAinWithThreeDotsBelow),
            'ࢴ' => Ok(ArabicExtendedA::ArabicLetterKafWithDotBelow),
            'ࢶ' => Ok(ArabicExtendedA::ArabicLetterBehWithSmallMeemAbove),
            'ࢷ' => Ok(ArabicExtendedA::ArabicLetterPehWithSmallMeemAbove),
            'ࢸ' => Ok(ArabicExtendedA::ArabicLetterTehWithSmallTehAbove),
            'ࢹ' => Ok(ArabicExtendedA::ArabicLetterRehWithSmallNoonAbove),
            'ࢺ' => Ok(ArabicExtendedA::ArabicLetterYehWithTwoDotsBelowAndSmallNoonAbove),
            'ࢻ' => Ok(ArabicExtendedA::ArabicLetterAfricanFeh),
            'ࢼ' => Ok(ArabicExtendedA::ArabicLetterAfricanQaf),
            'ࢽ' => Ok(ArabicExtendedA::ArabicLetterAfricanNoon),
            '࣓' => Ok(ArabicExtendedA::ArabicSmallLowWaw),
            'ࣔ' => Ok(ArabicExtendedA::ArabicSmallHighWordArDashRub),
            'ࣕ' => Ok(ArabicExtendedA::ArabicSmallHighSad),
            'ࣖ' => Ok(ArabicExtendedA::ArabicSmallHighAin),
            'ࣗ' => Ok(ArabicExtendedA::ArabicSmallHighQaf),
            'ࣘ' => Ok(ArabicExtendedA::ArabicSmallHighNoonWithKasra),
            'ࣙ' => Ok(ArabicExtendedA::ArabicSmallLowNoonWithKasra),
            'ࣚ' => Ok(ArabicExtendedA::ArabicSmallHighWordAthDashThalatha),
            'ࣛ' => Ok(ArabicExtendedA::ArabicSmallHighWordAsDashSajda),
            'ࣜ' => Ok(ArabicExtendedA::ArabicSmallHighWordAnDashNisf),
            'ࣝ' => Ok(ArabicExtendedA::ArabicSmallHighWordSakta),
            'ࣞ' => Ok(ArabicExtendedA::ArabicSmallHighWordQif),
            'ࣟ' => Ok(ArabicExtendedA::ArabicSmallHighWordWaqfa),
            '࣠' => Ok(ArabicExtendedA::ArabicSmallHighFootnoteMarker),
            '࣡' => Ok(ArabicExtendedA::ArabicSmallHighSignSafha),
            '࣢' => Ok(ArabicExtendedA::ArabicDisputedEndOfAyah),
            'ࣣ' => Ok(ArabicExtendedA::ArabicTurnedDammaBelow),
            'ࣤ' => Ok(ArabicExtendedA::ArabicCurlyFatha),
            'ࣥ' => Ok(ArabicExtendedA::ArabicCurlyDamma),
            'ࣦ' => Ok(ArabicExtendedA::ArabicCurlyKasra),
            'ࣧ' => Ok(ArabicExtendedA::ArabicCurlyFathatan),
            'ࣨ' => Ok(ArabicExtendedA::ArabicCurlyDammatan),
            'ࣩ' => Ok(ArabicExtendedA::ArabicCurlyKasratan),
            '࣪' => Ok(ArabicExtendedA::ArabicToneOneDotAbove),
            '࣫' => Ok(ArabicExtendedA::ArabicToneTwoDotsAbove),
            '࣬' => Ok(ArabicExtendedA::ArabicToneLoopAbove),
            '࣭' => Ok(ArabicExtendedA::ArabicToneOneDotBelow),
            '࣮' => Ok(ArabicExtendedA::ArabicToneTwoDotsBelow),
            '࣯' => Ok(ArabicExtendedA::ArabicToneLoopBelow),
            'ࣰ' => Ok(ArabicExtendedA::ArabicOpenFathatan),
            'ࣱ' => Ok(ArabicExtendedA::ArabicOpenDammatan),
            'ࣲ' => Ok(ArabicExtendedA::ArabicOpenKasratan),
            'ࣳ' => Ok(ArabicExtendedA::ArabicSmallHighWaw),
            'ࣴ' => Ok(ArabicExtendedA::ArabicFathaWithRing),
            'ࣵ' => Ok(ArabicExtendedA::ArabicFathaWithDotAbove),
            'ࣶ' => Ok(ArabicExtendedA::ArabicKasraWithDotBelow),
            'ࣷ' => Ok(ArabicExtendedA::ArabicLeftArrowheadAbove),
            'ࣸ' => Ok(ArabicExtendedA::ArabicRightArrowheadAbove),
            'ࣹ' => Ok(ArabicExtendedA::ArabicLeftArrowheadBelow),
            'ࣺ' => Ok(ArabicExtendedA::ArabicRightArrowheadBelow),
            'ࣻ' => Ok(ArabicExtendedA::ArabicDoubleRightArrowheadAbove),
            'ࣼ' => Ok(ArabicExtendedA::ArabicDoubleRightArrowheadAboveWithDot),
            'ࣽ' => Ok(ArabicExtendedA::ArabicRightArrowheadAboveWithDot),
            'ࣾ' => Ok(ArabicExtendedA::ArabicDammaWithDot),
            _ => Err(()),
        }
    }
}

impl Into<u32> for ArabicExtendedA {
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

impl std::convert::TryFrom<u32> for ArabicExtendedA {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for ArabicExtendedA {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl ArabicExtendedA {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        ArabicExtendedA::ArabicLetterBehWithSmallVBelow
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("ArabicExtendedA{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
