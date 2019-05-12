
/// An enum to represent all characters in the ArabicSupplement block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ArabicSupplement {
    /// \u{750}: 'ݐ'
    ArabicLetterBehWithThreeDotsHorizontallyBelow,
    /// \u{751}: 'ݑ'
    ArabicLetterBehWithDotBelowAndThreeDotsAbove,
    /// \u{752}: 'ݒ'
    ArabicLetterBehWithThreeDotsPointingUpwardsBelow,
    /// \u{753}: 'ݓ'
    ArabicLetterBehWithThreeDotsPointingUpwardsBelowAndTwoDotsAbove,
    /// \u{754}: 'ݔ'
    ArabicLetterBehWithTwoDotsBelowAndDotAbove,
    /// \u{755}: 'ݕ'
    ArabicLetterBehWithInvertedSmallVBelow,
    /// \u{756}: 'ݖ'
    ArabicLetterBehWithSmallV,
    /// \u{757}: 'ݗ'
    ArabicLetterHahWithTwoDotsAbove,
    /// \u{758}: 'ݘ'
    ArabicLetterHahWithThreeDotsPointingUpwardsBelow,
    /// \u{759}: 'ݙ'
    ArabicLetterDalWithTwoDotsVerticallyBelowAndSmallTah,
    /// \u{75a}: 'ݚ'
    ArabicLetterDalWithInvertedSmallVBelow,
    /// \u{75b}: 'ݛ'
    ArabicLetterRehWithStroke,
    /// \u{75c}: 'ݜ'
    ArabicLetterSeenWithFourDotsAbove,
    /// \u{75d}: 'ݝ'
    ArabicLetterAinWithTwoDotsAbove,
    /// \u{75e}: 'ݞ'
    ArabicLetterAinWithThreeDotsPointingDownwardsAbove,
    /// \u{75f}: 'ݟ'
    ArabicLetterAinWithTwoDotsVerticallyAbove,
    /// \u{760}: 'ݠ'
    ArabicLetterFehWithTwoDotsBelow,
    /// \u{761}: 'ݡ'
    ArabicLetterFehWithThreeDotsPointingUpwardsBelow,
    /// \u{762}: 'ݢ'
    ArabicLetterKehehWithDotAbove,
    /// \u{763}: 'ݣ'
    ArabicLetterKehehWithThreeDotsAbove,
    /// \u{764}: 'ݤ'
    ArabicLetterKehehWithThreeDotsPointingUpwardsBelow,
    /// \u{765}: 'ݥ'
    ArabicLetterMeemWithDotAbove,
    /// \u{766}: 'ݦ'
    ArabicLetterMeemWithDotBelow,
    /// \u{767}: 'ݧ'
    ArabicLetterNoonWithTwoDotsBelow,
    /// \u{768}: 'ݨ'
    ArabicLetterNoonWithSmallTah,
    /// \u{769}: 'ݩ'
    ArabicLetterNoonWithSmallV,
    /// \u{76a}: 'ݪ'
    ArabicLetterLamWithBar,
    /// \u{76b}: 'ݫ'
    ArabicLetterRehWithTwoDotsVerticallyAbove,
    /// \u{76c}: 'ݬ'
    ArabicLetterRehWithHamzaAbove,
    /// \u{76d}: 'ݭ'
    ArabicLetterSeenWithTwoDotsVerticallyAbove,
    /// \u{76e}: 'ݮ'
    ArabicLetterHahWithSmallArabicLetterTahBelow,
    /// \u{76f}: 'ݯ'
    ArabicLetterHahWithSmallArabicLetterTahAndTwoDots,
    /// \u{770}: 'ݰ'
    ArabicLetterSeenWithSmallArabicLetterTahAndTwoDots,
    /// \u{771}: 'ݱ'
    ArabicLetterRehWithSmallArabicLetterTahAndTwoDots,
    /// \u{772}: 'ݲ'
    ArabicLetterHahWithSmallArabicLetterTahAbove,
    /// \u{773}: 'ݳ'
    ArabicLetterAlefWithExtendedArabicDashIndicDigitTwoAbove,
    /// \u{774}: 'ݴ'
    ArabicLetterAlefWithExtendedArabicDashIndicDigitThreeAbove,
    /// \u{775}: 'ݵ'
    ArabicLetterFarsiYehWithExtendedArabicDashIndicDigitTwoAbove,
    /// \u{776}: 'ݶ'
    ArabicLetterFarsiYehWithExtendedArabicDashIndicDigitThreeAbove,
    /// \u{777}: 'ݷ'
    ArabicLetterFarsiYehWithExtendedArabicDashIndicDigitFourBelow,
    /// \u{778}: 'ݸ'
    ArabicLetterWawWithExtendedArabicDashIndicDigitTwoAbove,
    /// \u{779}: 'ݹ'
    ArabicLetterWawWithExtendedArabicDashIndicDigitThreeAbove,
    /// \u{77a}: 'ݺ'
    ArabicLetterYehBarreeWithExtendedArabicDashIndicDigitTwoAbove,
    /// \u{77b}: 'ݻ'
    ArabicLetterYehBarreeWithExtendedArabicDashIndicDigitThreeAbove,
    /// \u{77c}: 'ݼ'
    ArabicLetterHahWithExtendedArabicDashIndicDigitFourBelow,
    /// \u{77d}: 'ݽ'
    ArabicLetterSeenWithExtendedArabicDashIndicDigitFourAbove,
    /// \u{77e}: 'ݾ'
    ArabicLetterSeenWithInvertedV,
}

impl Into<char> for ArabicSupplement {
    fn into(self) -> char {
        match self {
            ArabicSupplement::ArabicLetterBehWithThreeDotsHorizontallyBelow => 'ݐ',
            ArabicSupplement::ArabicLetterBehWithDotBelowAndThreeDotsAbove => 'ݑ',
            ArabicSupplement::ArabicLetterBehWithThreeDotsPointingUpwardsBelow => 'ݒ',
            ArabicSupplement::ArabicLetterBehWithThreeDotsPointingUpwardsBelowAndTwoDotsAbove => 'ݓ',
            ArabicSupplement::ArabicLetterBehWithTwoDotsBelowAndDotAbove => 'ݔ',
            ArabicSupplement::ArabicLetterBehWithInvertedSmallVBelow => 'ݕ',
            ArabicSupplement::ArabicLetterBehWithSmallV => 'ݖ',
            ArabicSupplement::ArabicLetterHahWithTwoDotsAbove => 'ݗ',
            ArabicSupplement::ArabicLetterHahWithThreeDotsPointingUpwardsBelow => 'ݘ',
            ArabicSupplement::ArabicLetterDalWithTwoDotsVerticallyBelowAndSmallTah => 'ݙ',
            ArabicSupplement::ArabicLetterDalWithInvertedSmallVBelow => 'ݚ',
            ArabicSupplement::ArabicLetterRehWithStroke => 'ݛ',
            ArabicSupplement::ArabicLetterSeenWithFourDotsAbove => 'ݜ',
            ArabicSupplement::ArabicLetterAinWithTwoDotsAbove => 'ݝ',
            ArabicSupplement::ArabicLetterAinWithThreeDotsPointingDownwardsAbove => 'ݞ',
            ArabicSupplement::ArabicLetterAinWithTwoDotsVerticallyAbove => 'ݟ',
            ArabicSupplement::ArabicLetterFehWithTwoDotsBelow => 'ݠ',
            ArabicSupplement::ArabicLetterFehWithThreeDotsPointingUpwardsBelow => 'ݡ',
            ArabicSupplement::ArabicLetterKehehWithDotAbove => 'ݢ',
            ArabicSupplement::ArabicLetterKehehWithThreeDotsAbove => 'ݣ',
            ArabicSupplement::ArabicLetterKehehWithThreeDotsPointingUpwardsBelow => 'ݤ',
            ArabicSupplement::ArabicLetterMeemWithDotAbove => 'ݥ',
            ArabicSupplement::ArabicLetterMeemWithDotBelow => 'ݦ',
            ArabicSupplement::ArabicLetterNoonWithTwoDotsBelow => 'ݧ',
            ArabicSupplement::ArabicLetterNoonWithSmallTah => 'ݨ',
            ArabicSupplement::ArabicLetterNoonWithSmallV => 'ݩ',
            ArabicSupplement::ArabicLetterLamWithBar => 'ݪ',
            ArabicSupplement::ArabicLetterRehWithTwoDotsVerticallyAbove => 'ݫ',
            ArabicSupplement::ArabicLetterRehWithHamzaAbove => 'ݬ',
            ArabicSupplement::ArabicLetterSeenWithTwoDotsVerticallyAbove => 'ݭ',
            ArabicSupplement::ArabicLetterHahWithSmallArabicLetterTahBelow => 'ݮ',
            ArabicSupplement::ArabicLetterHahWithSmallArabicLetterTahAndTwoDots => 'ݯ',
            ArabicSupplement::ArabicLetterSeenWithSmallArabicLetterTahAndTwoDots => 'ݰ',
            ArabicSupplement::ArabicLetterRehWithSmallArabicLetterTahAndTwoDots => 'ݱ',
            ArabicSupplement::ArabicLetterHahWithSmallArabicLetterTahAbove => 'ݲ',
            ArabicSupplement::ArabicLetterAlefWithExtendedArabicDashIndicDigitTwoAbove => 'ݳ',
            ArabicSupplement::ArabicLetterAlefWithExtendedArabicDashIndicDigitThreeAbove => 'ݴ',
            ArabicSupplement::ArabicLetterFarsiYehWithExtendedArabicDashIndicDigitTwoAbove => 'ݵ',
            ArabicSupplement::ArabicLetterFarsiYehWithExtendedArabicDashIndicDigitThreeAbove => 'ݶ',
            ArabicSupplement::ArabicLetterFarsiYehWithExtendedArabicDashIndicDigitFourBelow => 'ݷ',
            ArabicSupplement::ArabicLetterWawWithExtendedArabicDashIndicDigitTwoAbove => 'ݸ',
            ArabicSupplement::ArabicLetterWawWithExtendedArabicDashIndicDigitThreeAbove => 'ݹ',
            ArabicSupplement::ArabicLetterYehBarreeWithExtendedArabicDashIndicDigitTwoAbove => 'ݺ',
            ArabicSupplement::ArabicLetterYehBarreeWithExtendedArabicDashIndicDigitThreeAbove => 'ݻ',
            ArabicSupplement::ArabicLetterHahWithExtendedArabicDashIndicDigitFourBelow => 'ݼ',
            ArabicSupplement::ArabicLetterSeenWithExtendedArabicDashIndicDigitFourAbove => 'ݽ',
            ArabicSupplement::ArabicLetterSeenWithInvertedV => 'ݾ',
        }
    }
}

impl std::convert::TryFrom<char> for ArabicSupplement {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ݐ' => Ok(ArabicSupplement::ArabicLetterBehWithThreeDotsHorizontallyBelow),
            'ݑ' => Ok(ArabicSupplement::ArabicLetterBehWithDotBelowAndThreeDotsAbove),
            'ݒ' => Ok(ArabicSupplement::ArabicLetterBehWithThreeDotsPointingUpwardsBelow),
            'ݓ' => Ok(ArabicSupplement::ArabicLetterBehWithThreeDotsPointingUpwardsBelowAndTwoDotsAbove),
            'ݔ' => Ok(ArabicSupplement::ArabicLetterBehWithTwoDotsBelowAndDotAbove),
            'ݕ' => Ok(ArabicSupplement::ArabicLetterBehWithInvertedSmallVBelow),
            'ݖ' => Ok(ArabicSupplement::ArabicLetterBehWithSmallV),
            'ݗ' => Ok(ArabicSupplement::ArabicLetterHahWithTwoDotsAbove),
            'ݘ' => Ok(ArabicSupplement::ArabicLetterHahWithThreeDotsPointingUpwardsBelow),
            'ݙ' => Ok(ArabicSupplement::ArabicLetterDalWithTwoDotsVerticallyBelowAndSmallTah),
            'ݚ' => Ok(ArabicSupplement::ArabicLetterDalWithInvertedSmallVBelow),
            'ݛ' => Ok(ArabicSupplement::ArabicLetterRehWithStroke),
            'ݜ' => Ok(ArabicSupplement::ArabicLetterSeenWithFourDotsAbove),
            'ݝ' => Ok(ArabicSupplement::ArabicLetterAinWithTwoDotsAbove),
            'ݞ' => Ok(ArabicSupplement::ArabicLetterAinWithThreeDotsPointingDownwardsAbove),
            'ݟ' => Ok(ArabicSupplement::ArabicLetterAinWithTwoDotsVerticallyAbove),
            'ݠ' => Ok(ArabicSupplement::ArabicLetterFehWithTwoDotsBelow),
            'ݡ' => Ok(ArabicSupplement::ArabicLetterFehWithThreeDotsPointingUpwardsBelow),
            'ݢ' => Ok(ArabicSupplement::ArabicLetterKehehWithDotAbove),
            'ݣ' => Ok(ArabicSupplement::ArabicLetterKehehWithThreeDotsAbove),
            'ݤ' => Ok(ArabicSupplement::ArabicLetterKehehWithThreeDotsPointingUpwardsBelow),
            'ݥ' => Ok(ArabicSupplement::ArabicLetterMeemWithDotAbove),
            'ݦ' => Ok(ArabicSupplement::ArabicLetterMeemWithDotBelow),
            'ݧ' => Ok(ArabicSupplement::ArabicLetterNoonWithTwoDotsBelow),
            'ݨ' => Ok(ArabicSupplement::ArabicLetterNoonWithSmallTah),
            'ݩ' => Ok(ArabicSupplement::ArabicLetterNoonWithSmallV),
            'ݪ' => Ok(ArabicSupplement::ArabicLetterLamWithBar),
            'ݫ' => Ok(ArabicSupplement::ArabicLetterRehWithTwoDotsVerticallyAbove),
            'ݬ' => Ok(ArabicSupplement::ArabicLetterRehWithHamzaAbove),
            'ݭ' => Ok(ArabicSupplement::ArabicLetterSeenWithTwoDotsVerticallyAbove),
            'ݮ' => Ok(ArabicSupplement::ArabicLetterHahWithSmallArabicLetterTahBelow),
            'ݯ' => Ok(ArabicSupplement::ArabicLetterHahWithSmallArabicLetterTahAndTwoDots),
            'ݰ' => Ok(ArabicSupplement::ArabicLetterSeenWithSmallArabicLetterTahAndTwoDots),
            'ݱ' => Ok(ArabicSupplement::ArabicLetterRehWithSmallArabicLetterTahAndTwoDots),
            'ݲ' => Ok(ArabicSupplement::ArabicLetterHahWithSmallArabicLetterTahAbove),
            'ݳ' => Ok(ArabicSupplement::ArabicLetterAlefWithExtendedArabicDashIndicDigitTwoAbove),
            'ݴ' => Ok(ArabicSupplement::ArabicLetterAlefWithExtendedArabicDashIndicDigitThreeAbove),
            'ݵ' => Ok(ArabicSupplement::ArabicLetterFarsiYehWithExtendedArabicDashIndicDigitTwoAbove),
            'ݶ' => Ok(ArabicSupplement::ArabicLetterFarsiYehWithExtendedArabicDashIndicDigitThreeAbove),
            'ݷ' => Ok(ArabicSupplement::ArabicLetterFarsiYehWithExtendedArabicDashIndicDigitFourBelow),
            'ݸ' => Ok(ArabicSupplement::ArabicLetterWawWithExtendedArabicDashIndicDigitTwoAbove),
            'ݹ' => Ok(ArabicSupplement::ArabicLetterWawWithExtendedArabicDashIndicDigitThreeAbove),
            'ݺ' => Ok(ArabicSupplement::ArabicLetterYehBarreeWithExtendedArabicDashIndicDigitTwoAbove),
            'ݻ' => Ok(ArabicSupplement::ArabicLetterYehBarreeWithExtendedArabicDashIndicDigitThreeAbove),
            'ݼ' => Ok(ArabicSupplement::ArabicLetterHahWithExtendedArabicDashIndicDigitFourBelow),
            'ݽ' => Ok(ArabicSupplement::ArabicLetterSeenWithExtendedArabicDashIndicDigitFourAbove),
            'ݾ' => Ok(ArabicSupplement::ArabicLetterSeenWithInvertedV),
            _ => Err(()),
        }
    }
}

impl Into<u32> for ArabicSupplement {
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

impl std::convert::TryFrom<u32> for ArabicSupplement {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for ArabicSupplement {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl ArabicSupplement {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        ArabicSupplement::ArabicLetterBehWithThreeDotsHorizontallyBelow
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            ArabicSupplement::ArabicLetterBehWithThreeDotsHorizontallyBelow => "arabic letter beh with three dots horizontally below",
            ArabicSupplement::ArabicLetterBehWithDotBelowAndThreeDotsAbove => "arabic letter beh with dot below and three dots above",
            ArabicSupplement::ArabicLetterBehWithThreeDotsPointingUpwardsBelow => "arabic letter beh with three dots pointing upwards below",
            ArabicSupplement::ArabicLetterBehWithThreeDotsPointingUpwardsBelowAndTwoDotsAbove => "arabic letter beh with three dots pointing upwards below and two dots above",
            ArabicSupplement::ArabicLetterBehWithTwoDotsBelowAndDotAbove => "arabic letter beh with two dots below and dot above",
            ArabicSupplement::ArabicLetterBehWithInvertedSmallVBelow => "arabic letter beh with inverted small v below",
            ArabicSupplement::ArabicLetterBehWithSmallV => "arabic letter beh with small v",
            ArabicSupplement::ArabicLetterHahWithTwoDotsAbove => "arabic letter hah with two dots above",
            ArabicSupplement::ArabicLetterHahWithThreeDotsPointingUpwardsBelow => "arabic letter hah with three dots pointing upwards below",
            ArabicSupplement::ArabicLetterDalWithTwoDotsVerticallyBelowAndSmallTah => "arabic letter dal with two dots vertically below and small tah",
            ArabicSupplement::ArabicLetterDalWithInvertedSmallVBelow => "arabic letter dal with inverted small v below",
            ArabicSupplement::ArabicLetterRehWithStroke => "arabic letter reh with stroke",
            ArabicSupplement::ArabicLetterSeenWithFourDotsAbove => "arabic letter seen with four dots above",
            ArabicSupplement::ArabicLetterAinWithTwoDotsAbove => "arabic letter ain with two dots above",
            ArabicSupplement::ArabicLetterAinWithThreeDotsPointingDownwardsAbove => "arabic letter ain with three dots pointing downwards above",
            ArabicSupplement::ArabicLetterAinWithTwoDotsVerticallyAbove => "arabic letter ain with two dots vertically above",
            ArabicSupplement::ArabicLetterFehWithTwoDotsBelow => "arabic letter feh with two dots below",
            ArabicSupplement::ArabicLetterFehWithThreeDotsPointingUpwardsBelow => "arabic letter feh with three dots pointing upwards below",
            ArabicSupplement::ArabicLetterKehehWithDotAbove => "arabic letter keheh with dot above",
            ArabicSupplement::ArabicLetterKehehWithThreeDotsAbove => "arabic letter keheh with three dots above",
            ArabicSupplement::ArabicLetterKehehWithThreeDotsPointingUpwardsBelow => "arabic letter keheh with three dots pointing upwards below",
            ArabicSupplement::ArabicLetterMeemWithDotAbove => "arabic letter meem with dot above",
            ArabicSupplement::ArabicLetterMeemWithDotBelow => "arabic letter meem with dot below",
            ArabicSupplement::ArabicLetterNoonWithTwoDotsBelow => "arabic letter noon with two dots below",
            ArabicSupplement::ArabicLetterNoonWithSmallTah => "arabic letter noon with small tah",
            ArabicSupplement::ArabicLetterNoonWithSmallV => "arabic letter noon with small v",
            ArabicSupplement::ArabicLetterLamWithBar => "arabic letter lam with bar",
            ArabicSupplement::ArabicLetterRehWithTwoDotsVerticallyAbove => "arabic letter reh with two dots vertically above",
            ArabicSupplement::ArabicLetterRehWithHamzaAbove => "arabic letter reh with hamza above",
            ArabicSupplement::ArabicLetterSeenWithTwoDotsVerticallyAbove => "arabic letter seen with two dots vertically above",
            ArabicSupplement::ArabicLetterHahWithSmallArabicLetterTahBelow => "arabic letter hah with small arabic letter tah below",
            ArabicSupplement::ArabicLetterHahWithSmallArabicLetterTahAndTwoDots => "arabic letter hah with small arabic letter tah and two dots",
            ArabicSupplement::ArabicLetterSeenWithSmallArabicLetterTahAndTwoDots => "arabic letter seen with small arabic letter tah and two dots",
            ArabicSupplement::ArabicLetterRehWithSmallArabicLetterTahAndTwoDots => "arabic letter reh with small arabic letter tah and two dots",
            ArabicSupplement::ArabicLetterHahWithSmallArabicLetterTahAbove => "arabic letter hah with small arabic letter tah above",
            ArabicSupplement::ArabicLetterAlefWithExtendedArabicDashIndicDigitTwoAbove => "arabic letter alef with extended arabic-indic digit two above",
            ArabicSupplement::ArabicLetterAlefWithExtendedArabicDashIndicDigitThreeAbove => "arabic letter alef with extended arabic-indic digit three above",
            ArabicSupplement::ArabicLetterFarsiYehWithExtendedArabicDashIndicDigitTwoAbove => "arabic letter farsi yeh with extended arabic-indic digit two above",
            ArabicSupplement::ArabicLetterFarsiYehWithExtendedArabicDashIndicDigitThreeAbove => "arabic letter farsi yeh with extended arabic-indic digit three above",
            ArabicSupplement::ArabicLetterFarsiYehWithExtendedArabicDashIndicDigitFourBelow => "arabic letter farsi yeh with extended arabic-indic digit four below",
            ArabicSupplement::ArabicLetterWawWithExtendedArabicDashIndicDigitTwoAbove => "arabic letter waw with extended arabic-indic digit two above",
            ArabicSupplement::ArabicLetterWawWithExtendedArabicDashIndicDigitThreeAbove => "arabic letter waw with extended arabic-indic digit three above",
            ArabicSupplement::ArabicLetterYehBarreeWithExtendedArabicDashIndicDigitTwoAbove => "arabic letter yeh barree with extended arabic-indic digit two above",
            ArabicSupplement::ArabicLetterYehBarreeWithExtendedArabicDashIndicDigitThreeAbove => "arabic letter yeh barree with extended arabic-indic digit three above",
            ArabicSupplement::ArabicLetterHahWithExtendedArabicDashIndicDigitFourBelow => "arabic letter hah with extended arabic-indic digit four below",
            ArabicSupplement::ArabicLetterSeenWithExtendedArabicDashIndicDigitFourAbove => "arabic letter seen with extended arabic-indic digit four above",
            ArabicSupplement::ArabicLetterSeenWithInvertedV => "arabic letter seen with inverted v",
        }
    }
}
