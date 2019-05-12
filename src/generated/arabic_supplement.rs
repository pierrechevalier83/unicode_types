
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

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("ArabicSupplement{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
