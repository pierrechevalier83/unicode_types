
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

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("PhoneticExtensionsSupplement{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
