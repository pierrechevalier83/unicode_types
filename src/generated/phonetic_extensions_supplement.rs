
/// An enum to represent all characters in the PhoneticExtensionsSupplement block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum PhoneticExtensionsSupplement {
    /// \u{1d80}: 'ᶀ'
    LatinSmallLetterBWithPalatalHook,
    /// \u{1d81}: 'ᶁ'
    LatinSmallLetterDWithPalatalHook,
    /// \u{1d82}: 'ᶂ'
    LatinSmallLetterFWithPalatalHook,
    /// \u{1d83}: 'ᶃ'
    LatinSmallLetterGWithPalatalHook,
    /// \u{1d84}: 'ᶄ'
    LatinSmallLetterKWithPalatalHook,
    /// \u{1d85}: 'ᶅ'
    LatinSmallLetterLWithPalatalHook,
    /// \u{1d86}: 'ᶆ'
    LatinSmallLetterMWithPalatalHook,
    /// \u{1d87}: 'ᶇ'
    LatinSmallLetterNWithPalatalHook,
    /// \u{1d88}: 'ᶈ'
    LatinSmallLetterPWithPalatalHook,
    /// \u{1d89}: 'ᶉ'
    LatinSmallLetterRWithPalatalHook,
    /// \u{1d8a}: 'ᶊ'
    LatinSmallLetterSWithPalatalHook,
    /// \u{1d8b}: 'ᶋ'
    LatinSmallLetterEshWithPalatalHook,
    /// \u{1d8c}: 'ᶌ'
    LatinSmallLetterVWithPalatalHook,
    /// \u{1d8d}: 'ᶍ'
    LatinSmallLetterXWithPalatalHook,
    /// \u{1d8e}: 'ᶎ'
    LatinSmallLetterZWithPalatalHook,
    /// \u{1d8f}: 'ᶏ'
    LatinSmallLetterAWithRetroflexHook,
    /// \u{1d90}: 'ᶐ'
    LatinSmallLetterAlphaWithRetroflexHook,
    /// \u{1d91}: 'ᶑ'
    LatinSmallLetterDWithHookAndTail,
    /// \u{1d92}: 'ᶒ'
    LatinSmallLetterEWithRetroflexHook,
    /// \u{1d93}: 'ᶓ'
    LatinSmallLetterOpenEWithRetroflexHook,
    /// \u{1d94}: 'ᶔ'
    LatinSmallLetterReversedOpenEWithRetroflexHook,
    /// \u{1d95}: 'ᶕ'
    LatinSmallLetterSchwaWithRetroflexHook,
    /// \u{1d96}: 'ᶖ'
    LatinSmallLetterIWithRetroflexHook,
    /// \u{1d97}: 'ᶗ'
    LatinSmallLetterOpenOWithRetroflexHook,
    /// \u{1d98}: 'ᶘ'
    LatinSmallLetterEshWithRetroflexHook,
    /// \u{1d99}: 'ᶙ'
    LatinSmallLetterUWithRetroflexHook,
    /// \u{1d9a}: 'ᶚ'
    LatinSmallLetterEzhWithRetroflexHook,
    /// \u{1d9b}: 'ᶛ'
    ModifierLetterSmallTurnedAlpha,
    /// \u{1d9c}: 'ᶜ'
    ModifierLetterSmallC,
    /// \u{1d9d}: 'ᶝ'
    ModifierLetterSmallCWithCurl,
    /// \u{1d9e}: 'ᶞ'
    ModifierLetterSmallEth,
    /// \u{1d9f}: 'ᶟ'
    ModifierLetterSmallReversedOpenE,
    /// \u{1da0}: 'ᶠ'
    ModifierLetterSmallF,
    /// \u{1da1}: 'ᶡ'
    ModifierLetterSmallDotlessJWithStroke,
    /// \u{1da2}: 'ᶢ'
    ModifierLetterSmallScriptG,
    /// \u{1da3}: 'ᶣ'
    ModifierLetterSmallTurnedH,
    /// \u{1da4}: 'ᶤ'
    ModifierLetterSmallIWithStroke,
    /// \u{1da5}: 'ᶥ'
    ModifierLetterSmallIota,
    /// \u{1da6}: 'ᶦ'
    ModifierLetterSmallCapitalI,
    /// \u{1da7}: 'ᶧ'
    ModifierLetterSmallCapitalIWithStroke,
    /// \u{1da8}: 'ᶨ'
    ModifierLetterSmallJWithCrossedDashTail,
    /// \u{1da9}: 'ᶩ'
    ModifierLetterSmallLWithRetroflexHook,
    /// \u{1daa}: 'ᶪ'
    ModifierLetterSmallLWithPalatalHook,
    /// \u{1dab}: 'ᶫ'
    ModifierLetterSmallCapitalL,
    /// \u{1dac}: 'ᶬ'
    ModifierLetterSmallMWithHook,
    /// \u{1dad}: 'ᶭ'
    ModifierLetterSmallTurnedMWithLongLeg,
    /// \u{1dae}: 'ᶮ'
    ModifierLetterSmallNWithLeftHook,
    /// \u{1daf}: 'ᶯ'
    ModifierLetterSmallNWithRetroflexHook,
    /// \u{1db0}: 'ᶰ'
    ModifierLetterSmallCapitalN,
    /// \u{1db1}: 'ᶱ'
    ModifierLetterSmallBarredO,
    /// \u{1db2}: 'ᶲ'
    ModifierLetterSmallPhi,
    /// \u{1db3}: 'ᶳ'
    ModifierLetterSmallSWithHook,
    /// \u{1db4}: 'ᶴ'
    ModifierLetterSmallEsh,
    /// \u{1db5}: 'ᶵ'
    ModifierLetterSmallTWithPalatalHook,
    /// \u{1db6}: 'ᶶ'
    ModifierLetterSmallUBar,
    /// \u{1db7}: 'ᶷ'
    ModifierLetterSmallUpsilon,
    /// \u{1db8}: 'ᶸ'
    ModifierLetterSmallCapitalU,
    /// \u{1db9}: 'ᶹ'
    ModifierLetterSmallVWithHook,
    /// \u{1dba}: 'ᶺ'
    ModifierLetterSmallTurnedV,
    /// \u{1dbb}: 'ᶻ'
    ModifierLetterSmallZ,
    /// \u{1dbc}: 'ᶼ'
    ModifierLetterSmallZWithRetroflexHook,
    /// \u{1dbd}: 'ᶽ'
    ModifierLetterSmallZWithCurl,
    /// \u{1dbe}: 'ᶾ'
    ModifierLetterSmallEzh,
}

impl Into<char> for PhoneticExtensionsSupplement {
    fn into(self) -> char {
        match self {
            PhoneticExtensionsSupplement::LatinSmallLetterBWithPalatalHook => 'ᶀ',
            PhoneticExtensionsSupplement::LatinSmallLetterDWithPalatalHook => 'ᶁ',
            PhoneticExtensionsSupplement::LatinSmallLetterFWithPalatalHook => 'ᶂ',
            PhoneticExtensionsSupplement::LatinSmallLetterGWithPalatalHook => 'ᶃ',
            PhoneticExtensionsSupplement::LatinSmallLetterKWithPalatalHook => 'ᶄ',
            PhoneticExtensionsSupplement::LatinSmallLetterLWithPalatalHook => 'ᶅ',
            PhoneticExtensionsSupplement::LatinSmallLetterMWithPalatalHook => 'ᶆ',
            PhoneticExtensionsSupplement::LatinSmallLetterNWithPalatalHook => 'ᶇ',
            PhoneticExtensionsSupplement::LatinSmallLetterPWithPalatalHook => 'ᶈ',
            PhoneticExtensionsSupplement::LatinSmallLetterRWithPalatalHook => 'ᶉ',
            PhoneticExtensionsSupplement::LatinSmallLetterSWithPalatalHook => 'ᶊ',
            PhoneticExtensionsSupplement::LatinSmallLetterEshWithPalatalHook => 'ᶋ',
            PhoneticExtensionsSupplement::LatinSmallLetterVWithPalatalHook => 'ᶌ',
            PhoneticExtensionsSupplement::LatinSmallLetterXWithPalatalHook => 'ᶍ',
            PhoneticExtensionsSupplement::LatinSmallLetterZWithPalatalHook => 'ᶎ',
            PhoneticExtensionsSupplement::LatinSmallLetterAWithRetroflexHook => 'ᶏ',
            PhoneticExtensionsSupplement::LatinSmallLetterAlphaWithRetroflexHook => 'ᶐ',
            PhoneticExtensionsSupplement::LatinSmallLetterDWithHookAndTail => 'ᶑ',
            PhoneticExtensionsSupplement::LatinSmallLetterEWithRetroflexHook => 'ᶒ',
            PhoneticExtensionsSupplement::LatinSmallLetterOpenEWithRetroflexHook => 'ᶓ',
            PhoneticExtensionsSupplement::LatinSmallLetterReversedOpenEWithRetroflexHook => 'ᶔ',
            PhoneticExtensionsSupplement::LatinSmallLetterSchwaWithRetroflexHook => 'ᶕ',
            PhoneticExtensionsSupplement::LatinSmallLetterIWithRetroflexHook => 'ᶖ',
            PhoneticExtensionsSupplement::LatinSmallLetterOpenOWithRetroflexHook => 'ᶗ',
            PhoneticExtensionsSupplement::LatinSmallLetterEshWithRetroflexHook => 'ᶘ',
            PhoneticExtensionsSupplement::LatinSmallLetterUWithRetroflexHook => 'ᶙ',
            PhoneticExtensionsSupplement::LatinSmallLetterEzhWithRetroflexHook => 'ᶚ',
            PhoneticExtensionsSupplement::ModifierLetterSmallTurnedAlpha => 'ᶛ',
            PhoneticExtensionsSupplement::ModifierLetterSmallC => 'ᶜ',
            PhoneticExtensionsSupplement::ModifierLetterSmallCWithCurl => 'ᶝ',
            PhoneticExtensionsSupplement::ModifierLetterSmallEth => 'ᶞ',
            PhoneticExtensionsSupplement::ModifierLetterSmallReversedOpenE => 'ᶟ',
            PhoneticExtensionsSupplement::ModifierLetterSmallF => 'ᶠ',
            PhoneticExtensionsSupplement::ModifierLetterSmallDotlessJWithStroke => 'ᶡ',
            PhoneticExtensionsSupplement::ModifierLetterSmallScriptG => 'ᶢ',
            PhoneticExtensionsSupplement::ModifierLetterSmallTurnedH => 'ᶣ',
            PhoneticExtensionsSupplement::ModifierLetterSmallIWithStroke => 'ᶤ',
            PhoneticExtensionsSupplement::ModifierLetterSmallIota => 'ᶥ',
            PhoneticExtensionsSupplement::ModifierLetterSmallCapitalI => 'ᶦ',
            PhoneticExtensionsSupplement::ModifierLetterSmallCapitalIWithStroke => 'ᶧ',
            PhoneticExtensionsSupplement::ModifierLetterSmallJWithCrossedDashTail => 'ᶨ',
            PhoneticExtensionsSupplement::ModifierLetterSmallLWithRetroflexHook => 'ᶩ',
            PhoneticExtensionsSupplement::ModifierLetterSmallLWithPalatalHook => 'ᶪ',
            PhoneticExtensionsSupplement::ModifierLetterSmallCapitalL => 'ᶫ',
            PhoneticExtensionsSupplement::ModifierLetterSmallMWithHook => 'ᶬ',
            PhoneticExtensionsSupplement::ModifierLetterSmallTurnedMWithLongLeg => 'ᶭ',
            PhoneticExtensionsSupplement::ModifierLetterSmallNWithLeftHook => 'ᶮ',
            PhoneticExtensionsSupplement::ModifierLetterSmallNWithRetroflexHook => 'ᶯ',
            PhoneticExtensionsSupplement::ModifierLetterSmallCapitalN => 'ᶰ',
            PhoneticExtensionsSupplement::ModifierLetterSmallBarredO => 'ᶱ',
            PhoneticExtensionsSupplement::ModifierLetterSmallPhi => 'ᶲ',
            PhoneticExtensionsSupplement::ModifierLetterSmallSWithHook => 'ᶳ',
            PhoneticExtensionsSupplement::ModifierLetterSmallEsh => 'ᶴ',
            PhoneticExtensionsSupplement::ModifierLetterSmallTWithPalatalHook => 'ᶵ',
            PhoneticExtensionsSupplement::ModifierLetterSmallUBar => 'ᶶ',
            PhoneticExtensionsSupplement::ModifierLetterSmallUpsilon => 'ᶷ',
            PhoneticExtensionsSupplement::ModifierLetterSmallCapitalU => 'ᶸ',
            PhoneticExtensionsSupplement::ModifierLetterSmallVWithHook => 'ᶹ',
            PhoneticExtensionsSupplement::ModifierLetterSmallTurnedV => 'ᶺ',
            PhoneticExtensionsSupplement::ModifierLetterSmallZ => 'ᶻ',
            PhoneticExtensionsSupplement::ModifierLetterSmallZWithRetroflexHook => 'ᶼ',
            PhoneticExtensionsSupplement::ModifierLetterSmallZWithCurl => 'ᶽ',
            PhoneticExtensionsSupplement::ModifierLetterSmallEzh => 'ᶾ',
        }
    }
}

impl std::convert::TryFrom<char> for PhoneticExtensionsSupplement {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ᶀ' => Ok(PhoneticExtensionsSupplement::LatinSmallLetterBWithPalatalHook),
            'ᶁ' => Ok(PhoneticExtensionsSupplement::LatinSmallLetterDWithPalatalHook),
            'ᶂ' => Ok(PhoneticExtensionsSupplement::LatinSmallLetterFWithPalatalHook),
            'ᶃ' => Ok(PhoneticExtensionsSupplement::LatinSmallLetterGWithPalatalHook),
            'ᶄ' => Ok(PhoneticExtensionsSupplement::LatinSmallLetterKWithPalatalHook),
            'ᶅ' => Ok(PhoneticExtensionsSupplement::LatinSmallLetterLWithPalatalHook),
            'ᶆ' => Ok(PhoneticExtensionsSupplement::LatinSmallLetterMWithPalatalHook),
            'ᶇ' => Ok(PhoneticExtensionsSupplement::LatinSmallLetterNWithPalatalHook),
            'ᶈ' => Ok(PhoneticExtensionsSupplement::LatinSmallLetterPWithPalatalHook),
            'ᶉ' => Ok(PhoneticExtensionsSupplement::LatinSmallLetterRWithPalatalHook),
            'ᶊ' => Ok(PhoneticExtensionsSupplement::LatinSmallLetterSWithPalatalHook),
            'ᶋ' => Ok(PhoneticExtensionsSupplement::LatinSmallLetterEshWithPalatalHook),
            'ᶌ' => Ok(PhoneticExtensionsSupplement::LatinSmallLetterVWithPalatalHook),
            'ᶍ' => Ok(PhoneticExtensionsSupplement::LatinSmallLetterXWithPalatalHook),
            'ᶎ' => Ok(PhoneticExtensionsSupplement::LatinSmallLetterZWithPalatalHook),
            'ᶏ' => Ok(PhoneticExtensionsSupplement::LatinSmallLetterAWithRetroflexHook),
            'ᶐ' => Ok(PhoneticExtensionsSupplement::LatinSmallLetterAlphaWithRetroflexHook),
            'ᶑ' => Ok(PhoneticExtensionsSupplement::LatinSmallLetterDWithHookAndTail),
            'ᶒ' => Ok(PhoneticExtensionsSupplement::LatinSmallLetterEWithRetroflexHook),
            'ᶓ' => Ok(PhoneticExtensionsSupplement::LatinSmallLetterOpenEWithRetroflexHook),
            'ᶔ' => Ok(PhoneticExtensionsSupplement::LatinSmallLetterReversedOpenEWithRetroflexHook),
            'ᶕ' => Ok(PhoneticExtensionsSupplement::LatinSmallLetterSchwaWithRetroflexHook),
            'ᶖ' => Ok(PhoneticExtensionsSupplement::LatinSmallLetterIWithRetroflexHook),
            'ᶗ' => Ok(PhoneticExtensionsSupplement::LatinSmallLetterOpenOWithRetroflexHook),
            'ᶘ' => Ok(PhoneticExtensionsSupplement::LatinSmallLetterEshWithRetroflexHook),
            'ᶙ' => Ok(PhoneticExtensionsSupplement::LatinSmallLetterUWithRetroflexHook),
            'ᶚ' => Ok(PhoneticExtensionsSupplement::LatinSmallLetterEzhWithRetroflexHook),
            'ᶛ' => Ok(PhoneticExtensionsSupplement::ModifierLetterSmallTurnedAlpha),
            'ᶜ' => Ok(PhoneticExtensionsSupplement::ModifierLetterSmallC),
            'ᶝ' => Ok(PhoneticExtensionsSupplement::ModifierLetterSmallCWithCurl),
            'ᶞ' => Ok(PhoneticExtensionsSupplement::ModifierLetterSmallEth),
            'ᶟ' => Ok(PhoneticExtensionsSupplement::ModifierLetterSmallReversedOpenE),
            'ᶠ' => Ok(PhoneticExtensionsSupplement::ModifierLetterSmallF),
            'ᶡ' => Ok(PhoneticExtensionsSupplement::ModifierLetterSmallDotlessJWithStroke),
            'ᶢ' => Ok(PhoneticExtensionsSupplement::ModifierLetterSmallScriptG),
            'ᶣ' => Ok(PhoneticExtensionsSupplement::ModifierLetterSmallTurnedH),
            'ᶤ' => Ok(PhoneticExtensionsSupplement::ModifierLetterSmallIWithStroke),
            'ᶥ' => Ok(PhoneticExtensionsSupplement::ModifierLetterSmallIota),
            'ᶦ' => Ok(PhoneticExtensionsSupplement::ModifierLetterSmallCapitalI),
            'ᶧ' => Ok(PhoneticExtensionsSupplement::ModifierLetterSmallCapitalIWithStroke),
            'ᶨ' => Ok(PhoneticExtensionsSupplement::ModifierLetterSmallJWithCrossedDashTail),
            'ᶩ' => Ok(PhoneticExtensionsSupplement::ModifierLetterSmallLWithRetroflexHook),
            'ᶪ' => Ok(PhoneticExtensionsSupplement::ModifierLetterSmallLWithPalatalHook),
            'ᶫ' => Ok(PhoneticExtensionsSupplement::ModifierLetterSmallCapitalL),
            'ᶬ' => Ok(PhoneticExtensionsSupplement::ModifierLetterSmallMWithHook),
            'ᶭ' => Ok(PhoneticExtensionsSupplement::ModifierLetterSmallTurnedMWithLongLeg),
            'ᶮ' => Ok(PhoneticExtensionsSupplement::ModifierLetterSmallNWithLeftHook),
            'ᶯ' => Ok(PhoneticExtensionsSupplement::ModifierLetterSmallNWithRetroflexHook),
            'ᶰ' => Ok(PhoneticExtensionsSupplement::ModifierLetterSmallCapitalN),
            'ᶱ' => Ok(PhoneticExtensionsSupplement::ModifierLetterSmallBarredO),
            'ᶲ' => Ok(PhoneticExtensionsSupplement::ModifierLetterSmallPhi),
            'ᶳ' => Ok(PhoneticExtensionsSupplement::ModifierLetterSmallSWithHook),
            'ᶴ' => Ok(PhoneticExtensionsSupplement::ModifierLetterSmallEsh),
            'ᶵ' => Ok(PhoneticExtensionsSupplement::ModifierLetterSmallTWithPalatalHook),
            'ᶶ' => Ok(PhoneticExtensionsSupplement::ModifierLetterSmallUBar),
            'ᶷ' => Ok(PhoneticExtensionsSupplement::ModifierLetterSmallUpsilon),
            'ᶸ' => Ok(PhoneticExtensionsSupplement::ModifierLetterSmallCapitalU),
            'ᶹ' => Ok(PhoneticExtensionsSupplement::ModifierLetterSmallVWithHook),
            'ᶺ' => Ok(PhoneticExtensionsSupplement::ModifierLetterSmallTurnedV),
            'ᶻ' => Ok(PhoneticExtensionsSupplement::ModifierLetterSmallZ),
            'ᶼ' => Ok(PhoneticExtensionsSupplement::ModifierLetterSmallZWithRetroflexHook),
            'ᶽ' => Ok(PhoneticExtensionsSupplement::ModifierLetterSmallZWithCurl),
            'ᶾ' => Ok(PhoneticExtensionsSupplement::ModifierLetterSmallEzh),
            _ => Err(()),
        }
    }
}

impl Into<u32> for PhoneticExtensionsSupplement {
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

impl std::convert::TryFrom<u32> for PhoneticExtensionsSupplement {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for PhoneticExtensionsSupplement {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl PhoneticExtensionsSupplement {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        PhoneticExtensionsSupplement::LatinSmallLetterBWithPalatalHook
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            PhoneticExtensionsSupplement::LatinSmallLetterBWithPalatalHook => "latin small letter b with palatal hook",
            PhoneticExtensionsSupplement::LatinSmallLetterDWithPalatalHook => "latin small letter d with palatal hook",
            PhoneticExtensionsSupplement::LatinSmallLetterFWithPalatalHook => "latin small letter f with palatal hook",
            PhoneticExtensionsSupplement::LatinSmallLetterGWithPalatalHook => "latin small letter g with palatal hook",
            PhoneticExtensionsSupplement::LatinSmallLetterKWithPalatalHook => "latin small letter k with palatal hook",
            PhoneticExtensionsSupplement::LatinSmallLetterLWithPalatalHook => "latin small letter l with palatal hook",
            PhoneticExtensionsSupplement::LatinSmallLetterMWithPalatalHook => "latin small letter m with palatal hook",
            PhoneticExtensionsSupplement::LatinSmallLetterNWithPalatalHook => "latin small letter n with palatal hook",
            PhoneticExtensionsSupplement::LatinSmallLetterPWithPalatalHook => "latin small letter p with palatal hook",
            PhoneticExtensionsSupplement::LatinSmallLetterRWithPalatalHook => "latin small letter r with palatal hook",
            PhoneticExtensionsSupplement::LatinSmallLetterSWithPalatalHook => "latin small letter s with palatal hook",
            PhoneticExtensionsSupplement::LatinSmallLetterEshWithPalatalHook => "latin small letter esh with palatal hook",
            PhoneticExtensionsSupplement::LatinSmallLetterVWithPalatalHook => "latin small letter v with palatal hook",
            PhoneticExtensionsSupplement::LatinSmallLetterXWithPalatalHook => "latin small letter x with palatal hook",
            PhoneticExtensionsSupplement::LatinSmallLetterZWithPalatalHook => "latin small letter z with palatal hook",
            PhoneticExtensionsSupplement::LatinSmallLetterAWithRetroflexHook => "latin small letter a with retroflex hook",
            PhoneticExtensionsSupplement::LatinSmallLetterAlphaWithRetroflexHook => "latin small letter alpha with retroflex hook",
            PhoneticExtensionsSupplement::LatinSmallLetterDWithHookAndTail => "latin small letter d with hook and tail",
            PhoneticExtensionsSupplement::LatinSmallLetterEWithRetroflexHook => "latin small letter e with retroflex hook",
            PhoneticExtensionsSupplement::LatinSmallLetterOpenEWithRetroflexHook => "latin small letter open e with retroflex hook",
            PhoneticExtensionsSupplement::LatinSmallLetterReversedOpenEWithRetroflexHook => "latin small letter reversed open e with retroflex hook",
            PhoneticExtensionsSupplement::LatinSmallLetterSchwaWithRetroflexHook => "latin small letter schwa with retroflex hook",
            PhoneticExtensionsSupplement::LatinSmallLetterIWithRetroflexHook => "latin small letter i with retroflex hook",
            PhoneticExtensionsSupplement::LatinSmallLetterOpenOWithRetroflexHook => "latin small letter open o with retroflex hook",
            PhoneticExtensionsSupplement::LatinSmallLetterEshWithRetroflexHook => "latin small letter esh with retroflex hook",
            PhoneticExtensionsSupplement::LatinSmallLetterUWithRetroflexHook => "latin small letter u with retroflex hook",
            PhoneticExtensionsSupplement::LatinSmallLetterEzhWithRetroflexHook => "latin small letter ezh with retroflex hook",
            PhoneticExtensionsSupplement::ModifierLetterSmallTurnedAlpha => "modifier letter small turned alpha",
            PhoneticExtensionsSupplement::ModifierLetterSmallC => "modifier letter small c",
            PhoneticExtensionsSupplement::ModifierLetterSmallCWithCurl => "modifier letter small c with curl",
            PhoneticExtensionsSupplement::ModifierLetterSmallEth => "modifier letter small eth",
            PhoneticExtensionsSupplement::ModifierLetterSmallReversedOpenE => "modifier letter small reversed open e",
            PhoneticExtensionsSupplement::ModifierLetterSmallF => "modifier letter small f",
            PhoneticExtensionsSupplement::ModifierLetterSmallDotlessJWithStroke => "modifier letter small dotless j with stroke",
            PhoneticExtensionsSupplement::ModifierLetterSmallScriptG => "modifier letter small script g",
            PhoneticExtensionsSupplement::ModifierLetterSmallTurnedH => "modifier letter small turned h",
            PhoneticExtensionsSupplement::ModifierLetterSmallIWithStroke => "modifier letter small i with stroke",
            PhoneticExtensionsSupplement::ModifierLetterSmallIota => "modifier letter small iota",
            PhoneticExtensionsSupplement::ModifierLetterSmallCapitalI => "modifier letter small capital i",
            PhoneticExtensionsSupplement::ModifierLetterSmallCapitalIWithStroke => "modifier letter small capital i with stroke",
            PhoneticExtensionsSupplement::ModifierLetterSmallJWithCrossedDashTail => "modifier letter small j with crossed-tail",
            PhoneticExtensionsSupplement::ModifierLetterSmallLWithRetroflexHook => "modifier letter small l with retroflex hook",
            PhoneticExtensionsSupplement::ModifierLetterSmallLWithPalatalHook => "modifier letter small l with palatal hook",
            PhoneticExtensionsSupplement::ModifierLetterSmallCapitalL => "modifier letter small capital l",
            PhoneticExtensionsSupplement::ModifierLetterSmallMWithHook => "modifier letter small m with hook",
            PhoneticExtensionsSupplement::ModifierLetterSmallTurnedMWithLongLeg => "modifier letter small turned m with long leg",
            PhoneticExtensionsSupplement::ModifierLetterSmallNWithLeftHook => "modifier letter small n with left hook",
            PhoneticExtensionsSupplement::ModifierLetterSmallNWithRetroflexHook => "modifier letter small n with retroflex hook",
            PhoneticExtensionsSupplement::ModifierLetterSmallCapitalN => "modifier letter small capital n",
            PhoneticExtensionsSupplement::ModifierLetterSmallBarredO => "modifier letter small barred o",
            PhoneticExtensionsSupplement::ModifierLetterSmallPhi => "modifier letter small phi",
            PhoneticExtensionsSupplement::ModifierLetterSmallSWithHook => "modifier letter small s with hook",
            PhoneticExtensionsSupplement::ModifierLetterSmallEsh => "modifier letter small esh",
            PhoneticExtensionsSupplement::ModifierLetterSmallTWithPalatalHook => "modifier letter small t with palatal hook",
            PhoneticExtensionsSupplement::ModifierLetterSmallUBar => "modifier letter small u bar",
            PhoneticExtensionsSupplement::ModifierLetterSmallUpsilon => "modifier letter small upsilon",
            PhoneticExtensionsSupplement::ModifierLetterSmallCapitalU => "modifier letter small capital u",
            PhoneticExtensionsSupplement::ModifierLetterSmallVWithHook => "modifier letter small v with hook",
            PhoneticExtensionsSupplement::ModifierLetterSmallTurnedV => "modifier letter small turned v",
            PhoneticExtensionsSupplement::ModifierLetterSmallZ => "modifier letter small z",
            PhoneticExtensionsSupplement::ModifierLetterSmallZWithRetroflexHook => "modifier letter small z with retroflex hook",
            PhoneticExtensionsSupplement::ModifierLetterSmallZWithCurl => "modifier letter small z with curl",
            PhoneticExtensionsSupplement::ModifierLetterSmallEzh => "modifier letter small ezh",
        }
    }
}
