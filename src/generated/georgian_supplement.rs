
/// An enum to represent all characters in the GeorgianSupplement block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum GeorgianSupplement {
    /// \u{2d00}: 'ⴀ'
    GeorgianSmallLetterAn,
    /// \u{2d01}: 'ⴁ'
    GeorgianSmallLetterBan,
    /// \u{2d02}: 'ⴂ'
    GeorgianSmallLetterGan,
    /// \u{2d03}: 'ⴃ'
    GeorgianSmallLetterDon,
    /// \u{2d04}: 'ⴄ'
    GeorgianSmallLetterEn,
    /// \u{2d05}: 'ⴅ'
    GeorgianSmallLetterVin,
    /// \u{2d06}: 'ⴆ'
    GeorgianSmallLetterZen,
    /// \u{2d07}: 'ⴇ'
    GeorgianSmallLetterTan,
    /// \u{2d08}: 'ⴈ'
    GeorgianSmallLetterIn,
    /// \u{2d09}: 'ⴉ'
    GeorgianSmallLetterKan,
    /// \u{2d0a}: 'ⴊ'
    GeorgianSmallLetterLas,
    /// \u{2d0b}: 'ⴋ'
    GeorgianSmallLetterMan,
    /// \u{2d0c}: 'ⴌ'
    GeorgianSmallLetterNar,
    /// \u{2d0d}: 'ⴍ'
    GeorgianSmallLetterOn,
    /// \u{2d0e}: 'ⴎ'
    GeorgianSmallLetterPar,
    /// \u{2d0f}: 'ⴏ'
    GeorgianSmallLetterZhar,
    /// \u{2d10}: 'ⴐ'
    GeorgianSmallLetterRae,
    /// \u{2d11}: 'ⴑ'
    GeorgianSmallLetterSan,
    /// \u{2d12}: 'ⴒ'
    GeorgianSmallLetterTar,
    /// \u{2d13}: 'ⴓ'
    GeorgianSmallLetterUn,
    /// \u{2d14}: 'ⴔ'
    GeorgianSmallLetterPhar,
    /// \u{2d15}: 'ⴕ'
    GeorgianSmallLetterKhar,
    /// \u{2d16}: 'ⴖ'
    GeorgianSmallLetterGhan,
    /// \u{2d17}: 'ⴗ'
    GeorgianSmallLetterQar,
    /// \u{2d18}: 'ⴘ'
    GeorgianSmallLetterShin,
    /// \u{2d19}: 'ⴙ'
    GeorgianSmallLetterChin,
    /// \u{2d1a}: 'ⴚ'
    GeorgianSmallLetterCan,
    /// \u{2d1b}: 'ⴛ'
    GeorgianSmallLetterJil,
    /// \u{2d1c}: 'ⴜ'
    GeorgianSmallLetterCil,
    /// \u{2d1d}: 'ⴝ'
    GeorgianSmallLetterChar,
    /// \u{2d1e}: 'ⴞ'
    GeorgianSmallLetterXan,
    /// \u{2d1f}: 'ⴟ'
    GeorgianSmallLetterJhan,
    /// \u{2d20}: 'ⴠ'
    GeorgianSmallLetterHae,
    /// \u{2d21}: 'ⴡ'
    GeorgianSmallLetterHe,
    /// \u{2d22}: 'ⴢ'
    GeorgianSmallLetterHie,
    /// \u{2d23}: 'ⴣ'
    GeorgianSmallLetterWe,
    /// \u{2d24}: 'ⴤ'
    GeorgianSmallLetterHar,
    /// \u{2d25}: 'ⴥ'
    GeorgianSmallLetterHoe,
    /// \u{2d27}: 'ⴧ'
    GeorgianSmallLetterYn,
    /// \u{2d2d}: 'ⴭ'
    GeorgianSmallLetterAen,
}

impl Into<char> for GeorgianSupplement {
    fn into(self) -> char {
        match self {
            GeorgianSupplement::GeorgianSmallLetterAn => 'ⴀ',
            GeorgianSupplement::GeorgianSmallLetterBan => 'ⴁ',
            GeorgianSupplement::GeorgianSmallLetterGan => 'ⴂ',
            GeorgianSupplement::GeorgianSmallLetterDon => 'ⴃ',
            GeorgianSupplement::GeorgianSmallLetterEn => 'ⴄ',
            GeorgianSupplement::GeorgianSmallLetterVin => 'ⴅ',
            GeorgianSupplement::GeorgianSmallLetterZen => 'ⴆ',
            GeorgianSupplement::GeorgianSmallLetterTan => 'ⴇ',
            GeorgianSupplement::GeorgianSmallLetterIn => 'ⴈ',
            GeorgianSupplement::GeorgianSmallLetterKan => 'ⴉ',
            GeorgianSupplement::GeorgianSmallLetterLas => 'ⴊ',
            GeorgianSupplement::GeorgianSmallLetterMan => 'ⴋ',
            GeorgianSupplement::GeorgianSmallLetterNar => 'ⴌ',
            GeorgianSupplement::GeorgianSmallLetterOn => 'ⴍ',
            GeorgianSupplement::GeorgianSmallLetterPar => 'ⴎ',
            GeorgianSupplement::GeorgianSmallLetterZhar => 'ⴏ',
            GeorgianSupplement::GeorgianSmallLetterRae => 'ⴐ',
            GeorgianSupplement::GeorgianSmallLetterSan => 'ⴑ',
            GeorgianSupplement::GeorgianSmallLetterTar => 'ⴒ',
            GeorgianSupplement::GeorgianSmallLetterUn => 'ⴓ',
            GeorgianSupplement::GeorgianSmallLetterPhar => 'ⴔ',
            GeorgianSupplement::GeorgianSmallLetterKhar => 'ⴕ',
            GeorgianSupplement::GeorgianSmallLetterGhan => 'ⴖ',
            GeorgianSupplement::GeorgianSmallLetterQar => 'ⴗ',
            GeorgianSupplement::GeorgianSmallLetterShin => 'ⴘ',
            GeorgianSupplement::GeorgianSmallLetterChin => 'ⴙ',
            GeorgianSupplement::GeorgianSmallLetterCan => 'ⴚ',
            GeorgianSupplement::GeorgianSmallLetterJil => 'ⴛ',
            GeorgianSupplement::GeorgianSmallLetterCil => 'ⴜ',
            GeorgianSupplement::GeorgianSmallLetterChar => 'ⴝ',
            GeorgianSupplement::GeorgianSmallLetterXan => 'ⴞ',
            GeorgianSupplement::GeorgianSmallLetterJhan => 'ⴟ',
            GeorgianSupplement::GeorgianSmallLetterHae => 'ⴠ',
            GeorgianSupplement::GeorgianSmallLetterHe => 'ⴡ',
            GeorgianSupplement::GeorgianSmallLetterHie => 'ⴢ',
            GeorgianSupplement::GeorgianSmallLetterWe => 'ⴣ',
            GeorgianSupplement::GeorgianSmallLetterHar => 'ⴤ',
            GeorgianSupplement::GeorgianSmallLetterHoe => 'ⴥ',
            GeorgianSupplement::GeorgianSmallLetterYn => 'ⴧ',
            GeorgianSupplement::GeorgianSmallLetterAen => 'ⴭ',
        }
    }
}

impl std::convert::TryFrom<char> for GeorgianSupplement {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ⴀ' => Ok(GeorgianSupplement::GeorgianSmallLetterAn),
            'ⴁ' => Ok(GeorgianSupplement::GeorgianSmallLetterBan),
            'ⴂ' => Ok(GeorgianSupplement::GeorgianSmallLetterGan),
            'ⴃ' => Ok(GeorgianSupplement::GeorgianSmallLetterDon),
            'ⴄ' => Ok(GeorgianSupplement::GeorgianSmallLetterEn),
            'ⴅ' => Ok(GeorgianSupplement::GeorgianSmallLetterVin),
            'ⴆ' => Ok(GeorgianSupplement::GeorgianSmallLetterZen),
            'ⴇ' => Ok(GeorgianSupplement::GeorgianSmallLetterTan),
            'ⴈ' => Ok(GeorgianSupplement::GeorgianSmallLetterIn),
            'ⴉ' => Ok(GeorgianSupplement::GeorgianSmallLetterKan),
            'ⴊ' => Ok(GeorgianSupplement::GeorgianSmallLetterLas),
            'ⴋ' => Ok(GeorgianSupplement::GeorgianSmallLetterMan),
            'ⴌ' => Ok(GeorgianSupplement::GeorgianSmallLetterNar),
            'ⴍ' => Ok(GeorgianSupplement::GeorgianSmallLetterOn),
            'ⴎ' => Ok(GeorgianSupplement::GeorgianSmallLetterPar),
            'ⴏ' => Ok(GeorgianSupplement::GeorgianSmallLetterZhar),
            'ⴐ' => Ok(GeorgianSupplement::GeorgianSmallLetterRae),
            'ⴑ' => Ok(GeorgianSupplement::GeorgianSmallLetterSan),
            'ⴒ' => Ok(GeorgianSupplement::GeorgianSmallLetterTar),
            'ⴓ' => Ok(GeorgianSupplement::GeorgianSmallLetterUn),
            'ⴔ' => Ok(GeorgianSupplement::GeorgianSmallLetterPhar),
            'ⴕ' => Ok(GeorgianSupplement::GeorgianSmallLetterKhar),
            'ⴖ' => Ok(GeorgianSupplement::GeorgianSmallLetterGhan),
            'ⴗ' => Ok(GeorgianSupplement::GeorgianSmallLetterQar),
            'ⴘ' => Ok(GeorgianSupplement::GeorgianSmallLetterShin),
            'ⴙ' => Ok(GeorgianSupplement::GeorgianSmallLetterChin),
            'ⴚ' => Ok(GeorgianSupplement::GeorgianSmallLetterCan),
            'ⴛ' => Ok(GeorgianSupplement::GeorgianSmallLetterJil),
            'ⴜ' => Ok(GeorgianSupplement::GeorgianSmallLetterCil),
            'ⴝ' => Ok(GeorgianSupplement::GeorgianSmallLetterChar),
            'ⴞ' => Ok(GeorgianSupplement::GeorgianSmallLetterXan),
            'ⴟ' => Ok(GeorgianSupplement::GeorgianSmallLetterJhan),
            'ⴠ' => Ok(GeorgianSupplement::GeorgianSmallLetterHae),
            'ⴡ' => Ok(GeorgianSupplement::GeorgianSmallLetterHe),
            'ⴢ' => Ok(GeorgianSupplement::GeorgianSmallLetterHie),
            'ⴣ' => Ok(GeorgianSupplement::GeorgianSmallLetterWe),
            'ⴤ' => Ok(GeorgianSupplement::GeorgianSmallLetterHar),
            'ⴥ' => Ok(GeorgianSupplement::GeorgianSmallLetterHoe),
            'ⴧ' => Ok(GeorgianSupplement::GeorgianSmallLetterYn),
            'ⴭ' => Ok(GeorgianSupplement::GeorgianSmallLetterAen),
            _ => Err(()),
        }
    }
}

impl Into<u32> for GeorgianSupplement {
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

impl std::convert::TryFrom<u32> for GeorgianSupplement {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for GeorgianSupplement {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl GeorgianSupplement {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        GeorgianSupplement::GeorgianSmallLetterAn
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("GeorgianSupplement{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
