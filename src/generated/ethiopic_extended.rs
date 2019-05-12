
/// An enum to represent all characters in the EthiopicExtended block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum EthiopicExtended {
    /// \u{2d80}: 'ⶀ'
    EthiopicSyllableLoa,
    /// \u{2d81}: 'ⶁ'
    EthiopicSyllableMoa,
    /// \u{2d82}: 'ⶂ'
    EthiopicSyllableRoa,
    /// \u{2d83}: 'ⶃ'
    EthiopicSyllableSoa,
    /// \u{2d84}: 'ⶄ'
    EthiopicSyllableShoa,
    /// \u{2d85}: 'ⶅ'
    EthiopicSyllableBoa,
    /// \u{2d86}: 'ⶆ'
    EthiopicSyllableToa,
    /// \u{2d87}: 'ⶇ'
    EthiopicSyllableCoa,
    /// \u{2d88}: 'ⶈ'
    EthiopicSyllableNoa,
    /// \u{2d89}: 'ⶉ'
    EthiopicSyllableNyoa,
    /// \u{2d8a}: 'ⶊ'
    EthiopicSyllableGlottalOa,
    /// \u{2d8b}: 'ⶋ'
    EthiopicSyllableZoa,
    /// \u{2d8c}: 'ⶌ'
    EthiopicSyllableDoa,
    /// \u{2d8d}: 'ⶍ'
    EthiopicSyllableDdoa,
    /// \u{2d8e}: 'ⶎ'
    EthiopicSyllableJoa,
    /// \u{2d8f}: 'ⶏ'
    EthiopicSyllableThoa,
    /// \u{2d90}: 'ⶐ'
    EthiopicSyllableChoa,
    /// \u{2d91}: 'ⶑ'
    EthiopicSyllablePhoa,
    /// \u{2d92}: 'ⶒ'
    EthiopicSyllablePoa,
    /// \u{2d93}: 'ⶓ'
    EthiopicSyllableGgwa,
    /// \u{2d94}: 'ⶔ'
    EthiopicSyllableGgwi,
    /// \u{2d95}: 'ⶕ'
    EthiopicSyllableGgwee,
    /// \u{2d96}: 'ⶖ'
    EthiopicSyllableGgwe,
    /// \u{2da0}: 'ⶠ'
    EthiopicSyllableSsa,
    /// \u{2da1}: 'ⶡ'
    EthiopicSyllableSsu,
    /// \u{2da2}: 'ⶢ'
    EthiopicSyllableSsi,
    /// \u{2da3}: 'ⶣ'
    EthiopicSyllableSsaa,
    /// \u{2da4}: 'ⶤ'
    EthiopicSyllableSsee,
    /// \u{2da5}: 'ⶥ'
    EthiopicSyllableSse,
    /// \u{2da6}: 'ⶦ'
    EthiopicSyllableSso,
    /// \u{2da8}: 'ⶨ'
    EthiopicSyllableCca,
    /// \u{2da9}: 'ⶩ'
    EthiopicSyllableCcu,
    /// \u{2daa}: 'ⶪ'
    EthiopicSyllableCci,
    /// \u{2dab}: 'ⶫ'
    EthiopicSyllableCcaa,
    /// \u{2dac}: 'ⶬ'
    EthiopicSyllableCcee,
    /// \u{2dad}: 'ⶭ'
    EthiopicSyllableCce,
    /// \u{2dae}: 'ⶮ'
    EthiopicSyllableCco,
    /// \u{2db0}: 'ⶰ'
    EthiopicSyllableZza,
    /// \u{2db1}: 'ⶱ'
    EthiopicSyllableZzu,
    /// \u{2db2}: 'ⶲ'
    EthiopicSyllableZzi,
    /// \u{2db3}: 'ⶳ'
    EthiopicSyllableZzaa,
    /// \u{2db4}: 'ⶴ'
    EthiopicSyllableZzee,
    /// \u{2db5}: 'ⶵ'
    EthiopicSyllableZze,
    /// \u{2db6}: 'ⶶ'
    EthiopicSyllableZzo,
    /// \u{2db8}: 'ⶸ'
    EthiopicSyllableCcha,
    /// \u{2db9}: 'ⶹ'
    EthiopicSyllableCchu,
    /// \u{2dba}: 'ⶺ'
    EthiopicSyllableCchi,
    /// \u{2dbb}: 'ⶻ'
    EthiopicSyllableCchaa,
    /// \u{2dbc}: 'ⶼ'
    EthiopicSyllableCchee,
    /// \u{2dbd}: 'ⶽ'
    EthiopicSyllableCche,
    /// \u{2dbe}: 'ⶾ'
    EthiopicSyllableCcho,
    /// \u{2dc0}: 'ⷀ'
    EthiopicSyllableQya,
    /// \u{2dc1}: 'ⷁ'
    EthiopicSyllableQyu,
    /// \u{2dc2}: 'ⷂ'
    EthiopicSyllableQyi,
    /// \u{2dc3}: 'ⷃ'
    EthiopicSyllableQyaa,
    /// \u{2dc4}: 'ⷄ'
    EthiopicSyllableQyee,
    /// \u{2dc5}: 'ⷅ'
    EthiopicSyllableQye,
    /// \u{2dc6}: 'ⷆ'
    EthiopicSyllableQyo,
    /// \u{2dc8}: 'ⷈ'
    EthiopicSyllableKya,
    /// \u{2dc9}: 'ⷉ'
    EthiopicSyllableKyu,
    /// \u{2dca}: 'ⷊ'
    EthiopicSyllableKyi,
    /// \u{2dcb}: 'ⷋ'
    EthiopicSyllableKyaa,
    /// \u{2dcc}: 'ⷌ'
    EthiopicSyllableKyee,
    /// \u{2dcd}: 'ⷍ'
    EthiopicSyllableKye,
    /// \u{2dce}: 'ⷎ'
    EthiopicSyllableKyo,
    /// \u{2dd0}: 'ⷐ'
    EthiopicSyllableXya,
    /// \u{2dd1}: 'ⷑ'
    EthiopicSyllableXyu,
    /// \u{2dd2}: 'ⷒ'
    EthiopicSyllableXyi,
    /// \u{2dd3}: 'ⷓ'
    EthiopicSyllableXyaa,
    /// \u{2dd4}: 'ⷔ'
    EthiopicSyllableXyee,
    /// \u{2dd5}: 'ⷕ'
    EthiopicSyllableXye,
    /// \u{2dd6}: 'ⷖ'
    EthiopicSyllableXyo,
    /// \u{2dd8}: 'ⷘ'
    EthiopicSyllableGya,
    /// \u{2dd9}: 'ⷙ'
    EthiopicSyllableGyu,
    /// \u{2dda}: 'ⷚ'
    EthiopicSyllableGyi,
    /// \u{2ddb}: 'ⷛ'
    EthiopicSyllableGyaa,
    /// \u{2ddc}: 'ⷜ'
    EthiopicSyllableGyee,
    /// \u{2ddd}: 'ⷝ'
    EthiopicSyllableGye,
    /// \u{2dde}: 'ⷞ'
    EthiopicSyllableGyo,
}

impl Into<char> for EthiopicExtended {
    fn into(self) -> char {
        match self {
            EthiopicExtended::EthiopicSyllableLoa => 'ⶀ',
            EthiopicExtended::EthiopicSyllableMoa => 'ⶁ',
            EthiopicExtended::EthiopicSyllableRoa => 'ⶂ',
            EthiopicExtended::EthiopicSyllableSoa => 'ⶃ',
            EthiopicExtended::EthiopicSyllableShoa => 'ⶄ',
            EthiopicExtended::EthiopicSyllableBoa => 'ⶅ',
            EthiopicExtended::EthiopicSyllableToa => 'ⶆ',
            EthiopicExtended::EthiopicSyllableCoa => 'ⶇ',
            EthiopicExtended::EthiopicSyllableNoa => 'ⶈ',
            EthiopicExtended::EthiopicSyllableNyoa => 'ⶉ',
            EthiopicExtended::EthiopicSyllableGlottalOa => 'ⶊ',
            EthiopicExtended::EthiopicSyllableZoa => 'ⶋ',
            EthiopicExtended::EthiopicSyllableDoa => 'ⶌ',
            EthiopicExtended::EthiopicSyllableDdoa => 'ⶍ',
            EthiopicExtended::EthiopicSyllableJoa => 'ⶎ',
            EthiopicExtended::EthiopicSyllableThoa => 'ⶏ',
            EthiopicExtended::EthiopicSyllableChoa => 'ⶐ',
            EthiopicExtended::EthiopicSyllablePhoa => 'ⶑ',
            EthiopicExtended::EthiopicSyllablePoa => 'ⶒ',
            EthiopicExtended::EthiopicSyllableGgwa => 'ⶓ',
            EthiopicExtended::EthiopicSyllableGgwi => 'ⶔ',
            EthiopicExtended::EthiopicSyllableGgwee => 'ⶕ',
            EthiopicExtended::EthiopicSyllableGgwe => 'ⶖ',
            EthiopicExtended::EthiopicSyllableSsa => 'ⶠ',
            EthiopicExtended::EthiopicSyllableSsu => 'ⶡ',
            EthiopicExtended::EthiopicSyllableSsi => 'ⶢ',
            EthiopicExtended::EthiopicSyllableSsaa => 'ⶣ',
            EthiopicExtended::EthiopicSyllableSsee => 'ⶤ',
            EthiopicExtended::EthiopicSyllableSse => 'ⶥ',
            EthiopicExtended::EthiopicSyllableSso => 'ⶦ',
            EthiopicExtended::EthiopicSyllableCca => 'ⶨ',
            EthiopicExtended::EthiopicSyllableCcu => 'ⶩ',
            EthiopicExtended::EthiopicSyllableCci => 'ⶪ',
            EthiopicExtended::EthiopicSyllableCcaa => 'ⶫ',
            EthiopicExtended::EthiopicSyllableCcee => 'ⶬ',
            EthiopicExtended::EthiopicSyllableCce => 'ⶭ',
            EthiopicExtended::EthiopicSyllableCco => 'ⶮ',
            EthiopicExtended::EthiopicSyllableZza => 'ⶰ',
            EthiopicExtended::EthiopicSyllableZzu => 'ⶱ',
            EthiopicExtended::EthiopicSyllableZzi => 'ⶲ',
            EthiopicExtended::EthiopicSyllableZzaa => 'ⶳ',
            EthiopicExtended::EthiopicSyllableZzee => 'ⶴ',
            EthiopicExtended::EthiopicSyllableZze => 'ⶵ',
            EthiopicExtended::EthiopicSyllableZzo => 'ⶶ',
            EthiopicExtended::EthiopicSyllableCcha => 'ⶸ',
            EthiopicExtended::EthiopicSyllableCchu => 'ⶹ',
            EthiopicExtended::EthiopicSyllableCchi => 'ⶺ',
            EthiopicExtended::EthiopicSyllableCchaa => 'ⶻ',
            EthiopicExtended::EthiopicSyllableCchee => 'ⶼ',
            EthiopicExtended::EthiopicSyllableCche => 'ⶽ',
            EthiopicExtended::EthiopicSyllableCcho => 'ⶾ',
            EthiopicExtended::EthiopicSyllableQya => 'ⷀ',
            EthiopicExtended::EthiopicSyllableQyu => 'ⷁ',
            EthiopicExtended::EthiopicSyllableQyi => 'ⷂ',
            EthiopicExtended::EthiopicSyllableQyaa => 'ⷃ',
            EthiopicExtended::EthiopicSyllableQyee => 'ⷄ',
            EthiopicExtended::EthiopicSyllableQye => 'ⷅ',
            EthiopicExtended::EthiopicSyllableQyo => 'ⷆ',
            EthiopicExtended::EthiopicSyllableKya => 'ⷈ',
            EthiopicExtended::EthiopicSyllableKyu => 'ⷉ',
            EthiopicExtended::EthiopicSyllableKyi => 'ⷊ',
            EthiopicExtended::EthiopicSyllableKyaa => 'ⷋ',
            EthiopicExtended::EthiopicSyllableKyee => 'ⷌ',
            EthiopicExtended::EthiopicSyllableKye => 'ⷍ',
            EthiopicExtended::EthiopicSyllableKyo => 'ⷎ',
            EthiopicExtended::EthiopicSyllableXya => 'ⷐ',
            EthiopicExtended::EthiopicSyllableXyu => 'ⷑ',
            EthiopicExtended::EthiopicSyllableXyi => 'ⷒ',
            EthiopicExtended::EthiopicSyllableXyaa => 'ⷓ',
            EthiopicExtended::EthiopicSyllableXyee => 'ⷔ',
            EthiopicExtended::EthiopicSyllableXye => 'ⷕ',
            EthiopicExtended::EthiopicSyllableXyo => 'ⷖ',
            EthiopicExtended::EthiopicSyllableGya => 'ⷘ',
            EthiopicExtended::EthiopicSyllableGyu => 'ⷙ',
            EthiopicExtended::EthiopicSyllableGyi => 'ⷚ',
            EthiopicExtended::EthiopicSyllableGyaa => 'ⷛ',
            EthiopicExtended::EthiopicSyllableGyee => 'ⷜ',
            EthiopicExtended::EthiopicSyllableGye => 'ⷝ',
            EthiopicExtended::EthiopicSyllableGyo => 'ⷞ',
        }
    }
}

impl std::convert::TryFrom<char> for EthiopicExtended {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ⶀ' => Ok(EthiopicExtended::EthiopicSyllableLoa),
            'ⶁ' => Ok(EthiopicExtended::EthiopicSyllableMoa),
            'ⶂ' => Ok(EthiopicExtended::EthiopicSyllableRoa),
            'ⶃ' => Ok(EthiopicExtended::EthiopicSyllableSoa),
            'ⶄ' => Ok(EthiopicExtended::EthiopicSyllableShoa),
            'ⶅ' => Ok(EthiopicExtended::EthiopicSyllableBoa),
            'ⶆ' => Ok(EthiopicExtended::EthiopicSyllableToa),
            'ⶇ' => Ok(EthiopicExtended::EthiopicSyllableCoa),
            'ⶈ' => Ok(EthiopicExtended::EthiopicSyllableNoa),
            'ⶉ' => Ok(EthiopicExtended::EthiopicSyllableNyoa),
            'ⶊ' => Ok(EthiopicExtended::EthiopicSyllableGlottalOa),
            'ⶋ' => Ok(EthiopicExtended::EthiopicSyllableZoa),
            'ⶌ' => Ok(EthiopicExtended::EthiopicSyllableDoa),
            'ⶍ' => Ok(EthiopicExtended::EthiopicSyllableDdoa),
            'ⶎ' => Ok(EthiopicExtended::EthiopicSyllableJoa),
            'ⶏ' => Ok(EthiopicExtended::EthiopicSyllableThoa),
            'ⶐ' => Ok(EthiopicExtended::EthiopicSyllableChoa),
            'ⶑ' => Ok(EthiopicExtended::EthiopicSyllablePhoa),
            'ⶒ' => Ok(EthiopicExtended::EthiopicSyllablePoa),
            'ⶓ' => Ok(EthiopicExtended::EthiopicSyllableGgwa),
            'ⶔ' => Ok(EthiopicExtended::EthiopicSyllableGgwi),
            'ⶕ' => Ok(EthiopicExtended::EthiopicSyllableGgwee),
            'ⶖ' => Ok(EthiopicExtended::EthiopicSyllableGgwe),
            'ⶠ' => Ok(EthiopicExtended::EthiopicSyllableSsa),
            'ⶡ' => Ok(EthiopicExtended::EthiopicSyllableSsu),
            'ⶢ' => Ok(EthiopicExtended::EthiopicSyllableSsi),
            'ⶣ' => Ok(EthiopicExtended::EthiopicSyllableSsaa),
            'ⶤ' => Ok(EthiopicExtended::EthiopicSyllableSsee),
            'ⶥ' => Ok(EthiopicExtended::EthiopicSyllableSse),
            'ⶦ' => Ok(EthiopicExtended::EthiopicSyllableSso),
            'ⶨ' => Ok(EthiopicExtended::EthiopicSyllableCca),
            'ⶩ' => Ok(EthiopicExtended::EthiopicSyllableCcu),
            'ⶪ' => Ok(EthiopicExtended::EthiopicSyllableCci),
            'ⶫ' => Ok(EthiopicExtended::EthiopicSyllableCcaa),
            'ⶬ' => Ok(EthiopicExtended::EthiopicSyllableCcee),
            'ⶭ' => Ok(EthiopicExtended::EthiopicSyllableCce),
            'ⶮ' => Ok(EthiopicExtended::EthiopicSyllableCco),
            'ⶰ' => Ok(EthiopicExtended::EthiopicSyllableZza),
            'ⶱ' => Ok(EthiopicExtended::EthiopicSyllableZzu),
            'ⶲ' => Ok(EthiopicExtended::EthiopicSyllableZzi),
            'ⶳ' => Ok(EthiopicExtended::EthiopicSyllableZzaa),
            'ⶴ' => Ok(EthiopicExtended::EthiopicSyllableZzee),
            'ⶵ' => Ok(EthiopicExtended::EthiopicSyllableZze),
            'ⶶ' => Ok(EthiopicExtended::EthiopicSyllableZzo),
            'ⶸ' => Ok(EthiopicExtended::EthiopicSyllableCcha),
            'ⶹ' => Ok(EthiopicExtended::EthiopicSyllableCchu),
            'ⶺ' => Ok(EthiopicExtended::EthiopicSyllableCchi),
            'ⶻ' => Ok(EthiopicExtended::EthiopicSyllableCchaa),
            'ⶼ' => Ok(EthiopicExtended::EthiopicSyllableCchee),
            'ⶽ' => Ok(EthiopicExtended::EthiopicSyllableCche),
            'ⶾ' => Ok(EthiopicExtended::EthiopicSyllableCcho),
            'ⷀ' => Ok(EthiopicExtended::EthiopicSyllableQya),
            'ⷁ' => Ok(EthiopicExtended::EthiopicSyllableQyu),
            'ⷂ' => Ok(EthiopicExtended::EthiopicSyllableQyi),
            'ⷃ' => Ok(EthiopicExtended::EthiopicSyllableQyaa),
            'ⷄ' => Ok(EthiopicExtended::EthiopicSyllableQyee),
            'ⷅ' => Ok(EthiopicExtended::EthiopicSyllableQye),
            'ⷆ' => Ok(EthiopicExtended::EthiopicSyllableQyo),
            'ⷈ' => Ok(EthiopicExtended::EthiopicSyllableKya),
            'ⷉ' => Ok(EthiopicExtended::EthiopicSyllableKyu),
            'ⷊ' => Ok(EthiopicExtended::EthiopicSyllableKyi),
            'ⷋ' => Ok(EthiopicExtended::EthiopicSyllableKyaa),
            'ⷌ' => Ok(EthiopicExtended::EthiopicSyllableKyee),
            'ⷍ' => Ok(EthiopicExtended::EthiopicSyllableKye),
            'ⷎ' => Ok(EthiopicExtended::EthiopicSyllableKyo),
            'ⷐ' => Ok(EthiopicExtended::EthiopicSyllableXya),
            'ⷑ' => Ok(EthiopicExtended::EthiopicSyllableXyu),
            'ⷒ' => Ok(EthiopicExtended::EthiopicSyllableXyi),
            'ⷓ' => Ok(EthiopicExtended::EthiopicSyllableXyaa),
            'ⷔ' => Ok(EthiopicExtended::EthiopicSyllableXyee),
            'ⷕ' => Ok(EthiopicExtended::EthiopicSyllableXye),
            'ⷖ' => Ok(EthiopicExtended::EthiopicSyllableXyo),
            'ⷘ' => Ok(EthiopicExtended::EthiopicSyllableGya),
            'ⷙ' => Ok(EthiopicExtended::EthiopicSyllableGyu),
            'ⷚ' => Ok(EthiopicExtended::EthiopicSyllableGyi),
            'ⷛ' => Ok(EthiopicExtended::EthiopicSyllableGyaa),
            'ⷜ' => Ok(EthiopicExtended::EthiopicSyllableGyee),
            'ⷝ' => Ok(EthiopicExtended::EthiopicSyllableGye),
            'ⷞ' => Ok(EthiopicExtended::EthiopicSyllableGyo),
            _ => Err(()),
        }
    }
}

impl Into<u32> for EthiopicExtended {
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

impl std::convert::TryFrom<u32> for EthiopicExtended {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for EthiopicExtended {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl EthiopicExtended {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        EthiopicExtended::EthiopicSyllableLoa
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("EthiopicExtended{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
