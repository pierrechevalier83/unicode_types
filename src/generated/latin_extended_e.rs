
/// An enum to represent all characters in the LatinExtendedE block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum LatinExtendedE {
    /// \u{ab30}: 'ꬰ'
    LatinSmallLetterBarredAlpha,
    /// \u{ab31}: 'ꬱ'
    LatinSmallLetterAReversedDashSchwa,
    /// \u{ab32}: 'ꬲ'
    LatinSmallLetterBlackletterE,
    /// \u{ab33}: 'ꬳ'
    LatinSmallLetterBarredE,
    /// \u{ab34}: 'ꬴ'
    LatinSmallLetterEWithFlourish,
    /// \u{ab35}: 'ꬵ'
    LatinSmallLetterLenisF,
    /// \u{ab36}: 'ꬶ'
    LatinSmallLetterScriptGWithCrossedDashTail,
    /// \u{ab37}: 'ꬷ'
    LatinSmallLetterLWithInvertedLazyS,
    /// \u{ab38}: 'ꬸ'
    LatinSmallLetterLWithDoubleMiddleTilde,
    /// \u{ab39}: 'ꬹ'
    LatinSmallLetterLWithMiddleRing,
    /// \u{ab3a}: 'ꬺ'
    LatinSmallLetterMWithCrossedDashTail,
    /// \u{ab3b}: 'ꬻ'
    LatinSmallLetterNWithCrossedDashTail,
    /// \u{ab3c}: 'ꬼ'
    LatinSmallLetterEngWithCrossedDashTail,
    /// \u{ab3d}: 'ꬽ'
    LatinSmallLetterBlackletterO,
    /// \u{ab3e}: 'ꬾ'
    LatinSmallLetterBlackletterOWithStroke,
    /// \u{ab3f}: 'ꬿ'
    LatinSmallLetterOpenOWithStroke,
    /// \u{ab40}: 'ꭀ'
    LatinSmallLetterInvertedOe,
    /// \u{ab41}: 'ꭁ'
    LatinSmallLetterTurnedOeWithStroke,
    /// \u{ab42}: 'ꭂ'
    LatinSmallLetterTurnedOeWithHorizontalStroke,
    /// \u{ab43}: 'ꭃ'
    LatinSmallLetterTurnedOOpenDashO,
    /// \u{ab44}: 'ꭄ'
    LatinSmallLetterTurnedOOpenDashOWithStroke,
    /// \u{ab45}: 'ꭅ'
    LatinSmallLetterStirrupR,
    /// \u{ab46}: 'ꭆ'
    LatinLetterSmallCapitalRWithRightLeg,
    /// \u{ab47}: 'ꭇ'
    LatinSmallLetterRWithoutHandle,
    /// \u{ab48}: 'ꭈ'
    LatinSmallLetterDoubleR,
    /// \u{ab49}: 'ꭉ'
    LatinSmallLetterRWithCrossedDashTail,
    /// \u{ab4a}: 'ꭊ'
    LatinSmallLetterDoubleRWithCrossedDashTail,
    /// \u{ab4b}: 'ꭋ'
    LatinSmallLetterScriptR,
    /// \u{ab4c}: 'ꭌ'
    LatinSmallLetterScriptRWithRing,
    /// \u{ab4d}: 'ꭍ'
    LatinSmallLetterBaselineEsh,
    /// \u{ab4e}: 'ꭎ'
    LatinSmallLetterUWithShortRightLeg,
    /// \u{ab4f}: 'ꭏ'
    LatinSmallLetterUBarWithShortRightLeg,
    /// \u{ab50}: 'ꭐ'
    LatinSmallLetterUi,
    /// \u{ab51}: 'ꭑ'
    LatinSmallLetterTurnedUi,
    /// \u{ab52}: 'ꭒ'
    LatinSmallLetterUWithLeftHook,
    /// \u{ab53}: 'ꭓ'
    LatinSmallLetterChi,
    /// \u{ab54}: 'ꭔ'
    LatinSmallLetterChiWithLowRightRing,
    /// \u{ab55}: 'ꭕ'
    LatinSmallLetterChiWithLowLeftSerif,
    /// \u{ab56}: 'ꭖ'
    LatinSmallLetterXWithLowRightRing,
    /// \u{ab57}: 'ꭗ'
    LatinSmallLetterXWithLongLeftLeg,
    /// \u{ab58}: 'ꭘ'
    LatinSmallLetterXWithLongLeftLegAndLowRightRing,
    /// \u{ab59}: 'ꭙ'
    LatinSmallLetterXWithLongLeftLegWithSerif,
    /// \u{ab5a}: 'ꭚ'
    LatinSmallLetterYWithShortRightLeg,
    /// \u{ab5b}: '꭛'
    ModifierBreveWithInvertedBreve,
    /// \u{ab5c}: 'ꭜ'
    ModifierLetterSmallHeng,
    /// \u{ab5d}: 'ꭝ'
    ModifierLetterSmallLWithInvertedLazyS,
    /// \u{ab5e}: 'ꭞ'
    ModifierLetterSmallLWithMiddleTilde,
    /// \u{ab5f}: 'ꭟ'
    ModifierLetterSmallUWithLeftHook,
    /// \u{ab60}: 'ꭠ'
    LatinSmallLetterSakhaYat,
    /// \u{ab61}: 'ꭡ'
    LatinSmallLetterIotifiedE,
    /// \u{ab62}: 'ꭢ'
    LatinSmallLetterOpenOe,
    /// \u{ab63}: 'ꭣ'
    LatinSmallLetterUo,
    /// \u{ab64}: 'ꭤ'
    LatinSmallLetterInvertedAlpha,
    /// \u{ab65}: 'ꭥ'
    GreekLetterSmallCapitalOmega,
    /// \u{ab66}: 'ꭦ'
    LatinSmallLetterDzDigraphWithRetroflexHook,
    /// \u{ab67}: 'ꭧ'
    LatinSmallLetterTsDigraphWithRetroflexHook,
}

impl Into<char> for LatinExtendedE {
    fn into(self) -> char {
        match self {
            LatinExtendedE::LatinSmallLetterBarredAlpha => 'ꬰ',
            LatinExtendedE::LatinSmallLetterAReversedDashSchwa => 'ꬱ',
            LatinExtendedE::LatinSmallLetterBlackletterE => 'ꬲ',
            LatinExtendedE::LatinSmallLetterBarredE => 'ꬳ',
            LatinExtendedE::LatinSmallLetterEWithFlourish => 'ꬴ',
            LatinExtendedE::LatinSmallLetterLenisF => 'ꬵ',
            LatinExtendedE::LatinSmallLetterScriptGWithCrossedDashTail => 'ꬶ',
            LatinExtendedE::LatinSmallLetterLWithInvertedLazyS => 'ꬷ',
            LatinExtendedE::LatinSmallLetterLWithDoubleMiddleTilde => 'ꬸ',
            LatinExtendedE::LatinSmallLetterLWithMiddleRing => 'ꬹ',
            LatinExtendedE::LatinSmallLetterMWithCrossedDashTail => 'ꬺ',
            LatinExtendedE::LatinSmallLetterNWithCrossedDashTail => 'ꬻ',
            LatinExtendedE::LatinSmallLetterEngWithCrossedDashTail => 'ꬼ',
            LatinExtendedE::LatinSmallLetterBlackletterO => 'ꬽ',
            LatinExtendedE::LatinSmallLetterBlackletterOWithStroke => 'ꬾ',
            LatinExtendedE::LatinSmallLetterOpenOWithStroke => 'ꬿ',
            LatinExtendedE::LatinSmallLetterInvertedOe => 'ꭀ',
            LatinExtendedE::LatinSmallLetterTurnedOeWithStroke => 'ꭁ',
            LatinExtendedE::LatinSmallLetterTurnedOeWithHorizontalStroke => 'ꭂ',
            LatinExtendedE::LatinSmallLetterTurnedOOpenDashO => 'ꭃ',
            LatinExtendedE::LatinSmallLetterTurnedOOpenDashOWithStroke => 'ꭄ',
            LatinExtendedE::LatinSmallLetterStirrupR => 'ꭅ',
            LatinExtendedE::LatinLetterSmallCapitalRWithRightLeg => 'ꭆ',
            LatinExtendedE::LatinSmallLetterRWithoutHandle => 'ꭇ',
            LatinExtendedE::LatinSmallLetterDoubleR => 'ꭈ',
            LatinExtendedE::LatinSmallLetterRWithCrossedDashTail => 'ꭉ',
            LatinExtendedE::LatinSmallLetterDoubleRWithCrossedDashTail => 'ꭊ',
            LatinExtendedE::LatinSmallLetterScriptR => 'ꭋ',
            LatinExtendedE::LatinSmallLetterScriptRWithRing => 'ꭌ',
            LatinExtendedE::LatinSmallLetterBaselineEsh => 'ꭍ',
            LatinExtendedE::LatinSmallLetterUWithShortRightLeg => 'ꭎ',
            LatinExtendedE::LatinSmallLetterUBarWithShortRightLeg => 'ꭏ',
            LatinExtendedE::LatinSmallLetterUi => 'ꭐ',
            LatinExtendedE::LatinSmallLetterTurnedUi => 'ꭑ',
            LatinExtendedE::LatinSmallLetterUWithLeftHook => 'ꭒ',
            LatinExtendedE::LatinSmallLetterChi => 'ꭓ',
            LatinExtendedE::LatinSmallLetterChiWithLowRightRing => 'ꭔ',
            LatinExtendedE::LatinSmallLetterChiWithLowLeftSerif => 'ꭕ',
            LatinExtendedE::LatinSmallLetterXWithLowRightRing => 'ꭖ',
            LatinExtendedE::LatinSmallLetterXWithLongLeftLeg => 'ꭗ',
            LatinExtendedE::LatinSmallLetterXWithLongLeftLegAndLowRightRing => 'ꭘ',
            LatinExtendedE::LatinSmallLetterXWithLongLeftLegWithSerif => 'ꭙ',
            LatinExtendedE::LatinSmallLetterYWithShortRightLeg => 'ꭚ',
            LatinExtendedE::ModifierBreveWithInvertedBreve => '꭛',
            LatinExtendedE::ModifierLetterSmallHeng => 'ꭜ',
            LatinExtendedE::ModifierLetterSmallLWithInvertedLazyS => 'ꭝ',
            LatinExtendedE::ModifierLetterSmallLWithMiddleTilde => 'ꭞ',
            LatinExtendedE::ModifierLetterSmallUWithLeftHook => 'ꭟ',
            LatinExtendedE::LatinSmallLetterSakhaYat => 'ꭠ',
            LatinExtendedE::LatinSmallLetterIotifiedE => 'ꭡ',
            LatinExtendedE::LatinSmallLetterOpenOe => 'ꭢ',
            LatinExtendedE::LatinSmallLetterUo => 'ꭣ',
            LatinExtendedE::LatinSmallLetterInvertedAlpha => 'ꭤ',
            LatinExtendedE::GreekLetterSmallCapitalOmega => 'ꭥ',
            LatinExtendedE::LatinSmallLetterDzDigraphWithRetroflexHook => 'ꭦ',
            LatinExtendedE::LatinSmallLetterTsDigraphWithRetroflexHook => 'ꭧ',
        }
    }
}

impl std::convert::TryFrom<char> for LatinExtendedE {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ꬰ' => Ok(LatinExtendedE::LatinSmallLetterBarredAlpha),
            'ꬱ' => Ok(LatinExtendedE::LatinSmallLetterAReversedDashSchwa),
            'ꬲ' => Ok(LatinExtendedE::LatinSmallLetterBlackletterE),
            'ꬳ' => Ok(LatinExtendedE::LatinSmallLetterBarredE),
            'ꬴ' => Ok(LatinExtendedE::LatinSmallLetterEWithFlourish),
            'ꬵ' => Ok(LatinExtendedE::LatinSmallLetterLenisF),
            'ꬶ' => Ok(LatinExtendedE::LatinSmallLetterScriptGWithCrossedDashTail),
            'ꬷ' => Ok(LatinExtendedE::LatinSmallLetterLWithInvertedLazyS),
            'ꬸ' => Ok(LatinExtendedE::LatinSmallLetterLWithDoubleMiddleTilde),
            'ꬹ' => Ok(LatinExtendedE::LatinSmallLetterLWithMiddleRing),
            'ꬺ' => Ok(LatinExtendedE::LatinSmallLetterMWithCrossedDashTail),
            'ꬻ' => Ok(LatinExtendedE::LatinSmallLetterNWithCrossedDashTail),
            'ꬼ' => Ok(LatinExtendedE::LatinSmallLetterEngWithCrossedDashTail),
            'ꬽ' => Ok(LatinExtendedE::LatinSmallLetterBlackletterO),
            'ꬾ' => Ok(LatinExtendedE::LatinSmallLetterBlackletterOWithStroke),
            'ꬿ' => Ok(LatinExtendedE::LatinSmallLetterOpenOWithStroke),
            'ꭀ' => Ok(LatinExtendedE::LatinSmallLetterInvertedOe),
            'ꭁ' => Ok(LatinExtendedE::LatinSmallLetterTurnedOeWithStroke),
            'ꭂ' => Ok(LatinExtendedE::LatinSmallLetterTurnedOeWithHorizontalStroke),
            'ꭃ' => Ok(LatinExtendedE::LatinSmallLetterTurnedOOpenDashO),
            'ꭄ' => Ok(LatinExtendedE::LatinSmallLetterTurnedOOpenDashOWithStroke),
            'ꭅ' => Ok(LatinExtendedE::LatinSmallLetterStirrupR),
            'ꭆ' => Ok(LatinExtendedE::LatinLetterSmallCapitalRWithRightLeg),
            'ꭇ' => Ok(LatinExtendedE::LatinSmallLetterRWithoutHandle),
            'ꭈ' => Ok(LatinExtendedE::LatinSmallLetterDoubleR),
            'ꭉ' => Ok(LatinExtendedE::LatinSmallLetterRWithCrossedDashTail),
            'ꭊ' => Ok(LatinExtendedE::LatinSmallLetterDoubleRWithCrossedDashTail),
            'ꭋ' => Ok(LatinExtendedE::LatinSmallLetterScriptR),
            'ꭌ' => Ok(LatinExtendedE::LatinSmallLetterScriptRWithRing),
            'ꭍ' => Ok(LatinExtendedE::LatinSmallLetterBaselineEsh),
            'ꭎ' => Ok(LatinExtendedE::LatinSmallLetterUWithShortRightLeg),
            'ꭏ' => Ok(LatinExtendedE::LatinSmallLetterUBarWithShortRightLeg),
            'ꭐ' => Ok(LatinExtendedE::LatinSmallLetterUi),
            'ꭑ' => Ok(LatinExtendedE::LatinSmallLetterTurnedUi),
            'ꭒ' => Ok(LatinExtendedE::LatinSmallLetterUWithLeftHook),
            'ꭓ' => Ok(LatinExtendedE::LatinSmallLetterChi),
            'ꭔ' => Ok(LatinExtendedE::LatinSmallLetterChiWithLowRightRing),
            'ꭕ' => Ok(LatinExtendedE::LatinSmallLetterChiWithLowLeftSerif),
            'ꭖ' => Ok(LatinExtendedE::LatinSmallLetterXWithLowRightRing),
            'ꭗ' => Ok(LatinExtendedE::LatinSmallLetterXWithLongLeftLeg),
            'ꭘ' => Ok(LatinExtendedE::LatinSmallLetterXWithLongLeftLegAndLowRightRing),
            'ꭙ' => Ok(LatinExtendedE::LatinSmallLetterXWithLongLeftLegWithSerif),
            'ꭚ' => Ok(LatinExtendedE::LatinSmallLetterYWithShortRightLeg),
            '꭛' => Ok(LatinExtendedE::ModifierBreveWithInvertedBreve),
            'ꭜ' => Ok(LatinExtendedE::ModifierLetterSmallHeng),
            'ꭝ' => Ok(LatinExtendedE::ModifierLetterSmallLWithInvertedLazyS),
            'ꭞ' => Ok(LatinExtendedE::ModifierLetterSmallLWithMiddleTilde),
            'ꭟ' => Ok(LatinExtendedE::ModifierLetterSmallUWithLeftHook),
            'ꭠ' => Ok(LatinExtendedE::LatinSmallLetterSakhaYat),
            'ꭡ' => Ok(LatinExtendedE::LatinSmallLetterIotifiedE),
            'ꭢ' => Ok(LatinExtendedE::LatinSmallLetterOpenOe),
            'ꭣ' => Ok(LatinExtendedE::LatinSmallLetterUo),
            'ꭤ' => Ok(LatinExtendedE::LatinSmallLetterInvertedAlpha),
            'ꭥ' => Ok(LatinExtendedE::GreekLetterSmallCapitalOmega),
            'ꭦ' => Ok(LatinExtendedE::LatinSmallLetterDzDigraphWithRetroflexHook),
            'ꭧ' => Ok(LatinExtendedE::LatinSmallLetterTsDigraphWithRetroflexHook),
            _ => Err(()),
        }
    }
}

impl Into<u32> for LatinExtendedE {
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

impl std::convert::TryFrom<u32> for LatinExtendedE {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for LatinExtendedE {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl LatinExtendedE {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        LatinExtendedE::LatinSmallLetterBarredAlpha
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            LatinExtendedE::LatinSmallLetterBarredAlpha => "latin small letter barred alpha",
            LatinExtendedE::LatinSmallLetterAReversedDashSchwa => "latin small letter a reversed-schwa",
            LatinExtendedE::LatinSmallLetterBlackletterE => "latin small letter blackletter e",
            LatinExtendedE::LatinSmallLetterBarredE => "latin small letter barred e",
            LatinExtendedE::LatinSmallLetterEWithFlourish => "latin small letter e with flourish",
            LatinExtendedE::LatinSmallLetterLenisF => "latin small letter lenis f",
            LatinExtendedE::LatinSmallLetterScriptGWithCrossedDashTail => "latin small letter script g with crossed-tail",
            LatinExtendedE::LatinSmallLetterLWithInvertedLazyS => "latin small letter l with inverted lazy s",
            LatinExtendedE::LatinSmallLetterLWithDoubleMiddleTilde => "latin small letter l with double middle tilde",
            LatinExtendedE::LatinSmallLetterLWithMiddleRing => "latin small letter l with middle ring",
            LatinExtendedE::LatinSmallLetterMWithCrossedDashTail => "latin small letter m with crossed-tail",
            LatinExtendedE::LatinSmallLetterNWithCrossedDashTail => "latin small letter n with crossed-tail",
            LatinExtendedE::LatinSmallLetterEngWithCrossedDashTail => "latin small letter eng with crossed-tail",
            LatinExtendedE::LatinSmallLetterBlackletterO => "latin small letter blackletter o",
            LatinExtendedE::LatinSmallLetterBlackletterOWithStroke => "latin small letter blackletter o with stroke",
            LatinExtendedE::LatinSmallLetterOpenOWithStroke => "latin small letter open o with stroke",
            LatinExtendedE::LatinSmallLetterInvertedOe => "latin small letter inverted oe",
            LatinExtendedE::LatinSmallLetterTurnedOeWithStroke => "latin small letter turned oe with stroke",
            LatinExtendedE::LatinSmallLetterTurnedOeWithHorizontalStroke => "latin small letter turned oe with horizontal stroke",
            LatinExtendedE::LatinSmallLetterTurnedOOpenDashO => "latin small letter turned o open-o",
            LatinExtendedE::LatinSmallLetterTurnedOOpenDashOWithStroke => "latin small letter turned o open-o with stroke",
            LatinExtendedE::LatinSmallLetterStirrupR => "latin small letter stirrup r",
            LatinExtendedE::LatinLetterSmallCapitalRWithRightLeg => "latin letter small capital r with right leg",
            LatinExtendedE::LatinSmallLetterRWithoutHandle => "latin small letter r without handle",
            LatinExtendedE::LatinSmallLetterDoubleR => "latin small letter double r",
            LatinExtendedE::LatinSmallLetterRWithCrossedDashTail => "latin small letter r with crossed-tail",
            LatinExtendedE::LatinSmallLetterDoubleRWithCrossedDashTail => "latin small letter double r with crossed-tail",
            LatinExtendedE::LatinSmallLetterScriptR => "latin small letter script r",
            LatinExtendedE::LatinSmallLetterScriptRWithRing => "latin small letter script r with ring",
            LatinExtendedE::LatinSmallLetterBaselineEsh => "latin small letter baseline esh",
            LatinExtendedE::LatinSmallLetterUWithShortRightLeg => "latin small letter u with short right leg",
            LatinExtendedE::LatinSmallLetterUBarWithShortRightLeg => "latin small letter u bar with short right leg",
            LatinExtendedE::LatinSmallLetterUi => "latin small letter ui",
            LatinExtendedE::LatinSmallLetterTurnedUi => "latin small letter turned ui",
            LatinExtendedE::LatinSmallLetterUWithLeftHook => "latin small letter u with left hook",
            LatinExtendedE::LatinSmallLetterChi => "latin small letter chi",
            LatinExtendedE::LatinSmallLetterChiWithLowRightRing => "latin small letter chi with low right ring",
            LatinExtendedE::LatinSmallLetterChiWithLowLeftSerif => "latin small letter chi with low left serif",
            LatinExtendedE::LatinSmallLetterXWithLowRightRing => "latin small letter x with low right ring",
            LatinExtendedE::LatinSmallLetterXWithLongLeftLeg => "latin small letter x with long left leg",
            LatinExtendedE::LatinSmallLetterXWithLongLeftLegAndLowRightRing => "latin small letter x with long left leg and low right ring",
            LatinExtendedE::LatinSmallLetterXWithLongLeftLegWithSerif => "latin small letter x with long left leg with serif",
            LatinExtendedE::LatinSmallLetterYWithShortRightLeg => "latin small letter y with short right leg",
            LatinExtendedE::ModifierBreveWithInvertedBreve => "modifier breve with inverted breve",
            LatinExtendedE::ModifierLetterSmallHeng => "modifier letter small heng",
            LatinExtendedE::ModifierLetterSmallLWithInvertedLazyS => "modifier letter small l with inverted lazy s",
            LatinExtendedE::ModifierLetterSmallLWithMiddleTilde => "modifier letter small l with middle tilde",
            LatinExtendedE::ModifierLetterSmallUWithLeftHook => "modifier letter small u with left hook",
            LatinExtendedE::LatinSmallLetterSakhaYat => "latin small letter sakha yat",
            LatinExtendedE::LatinSmallLetterIotifiedE => "latin small letter iotified e",
            LatinExtendedE::LatinSmallLetterOpenOe => "latin small letter open oe",
            LatinExtendedE::LatinSmallLetterUo => "latin small letter uo",
            LatinExtendedE::LatinSmallLetterInvertedAlpha => "latin small letter inverted alpha",
            LatinExtendedE::GreekLetterSmallCapitalOmega => "greek letter small capital omega",
            LatinExtendedE::LatinSmallLetterDzDigraphWithRetroflexHook => "latin small letter dz digraph with retroflex hook",
            LatinExtendedE::LatinSmallLetterTsDigraphWithRetroflexHook => "latin small letter ts digraph with retroflex hook",
        }
    }
}
