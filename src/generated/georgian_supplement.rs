/// A number of constants to give a name to all characters in this block.
mod constants {
    /// \u{2d00}: 'ⴀ'
    pub const GEORGIAN_SMALL_LETTER_AN: char = 'ⴀ';
    /// \u{2d01}: 'ⴁ'
    pub const GEORGIAN_SMALL_LETTER_BAN: char = 'ⴁ';
    /// \u{2d02}: 'ⴂ'
    pub const GEORGIAN_SMALL_LETTER_GAN: char = 'ⴂ';
    /// \u{2d03}: 'ⴃ'
    pub const GEORGIAN_SMALL_LETTER_DON: char = 'ⴃ';
    /// \u{2d04}: 'ⴄ'
    pub const GEORGIAN_SMALL_LETTER_EN: char = 'ⴄ';
    /// \u{2d05}: 'ⴅ'
    pub const GEORGIAN_SMALL_LETTER_VIN: char = 'ⴅ';
    /// \u{2d06}: 'ⴆ'
    pub const GEORGIAN_SMALL_LETTER_ZEN: char = 'ⴆ';
    /// \u{2d07}: 'ⴇ'
    pub const GEORGIAN_SMALL_LETTER_TAN: char = 'ⴇ';
    /// \u{2d08}: 'ⴈ'
    pub const GEORGIAN_SMALL_LETTER_IN: char = 'ⴈ';
    /// \u{2d09}: 'ⴉ'
    pub const GEORGIAN_SMALL_LETTER_KAN: char = 'ⴉ';
    /// \u{2d0a}: 'ⴊ'
    pub const GEORGIAN_SMALL_LETTER_LAS: char = 'ⴊ';
    /// \u{2d0b}: 'ⴋ'
    pub const GEORGIAN_SMALL_LETTER_MAN: char = 'ⴋ';
    /// \u{2d0c}: 'ⴌ'
    pub const GEORGIAN_SMALL_LETTER_NAR: char = 'ⴌ';
    /// \u{2d0d}: 'ⴍ'
    pub const GEORGIAN_SMALL_LETTER_ON: char = 'ⴍ';
    /// \u{2d0e}: 'ⴎ'
    pub const GEORGIAN_SMALL_LETTER_PAR: char = 'ⴎ';
    /// \u{2d0f}: 'ⴏ'
    pub const GEORGIAN_SMALL_LETTER_ZHAR: char = 'ⴏ';
    /// \u{2d10}: 'ⴐ'
    pub const GEORGIAN_SMALL_LETTER_RAE: char = 'ⴐ';
    /// \u{2d11}: 'ⴑ'
    pub const GEORGIAN_SMALL_LETTER_SAN: char = 'ⴑ';
    /// \u{2d12}: 'ⴒ'
    pub const GEORGIAN_SMALL_LETTER_TAR: char = 'ⴒ';
    /// \u{2d13}: 'ⴓ'
    pub const GEORGIAN_SMALL_LETTER_UN: char = 'ⴓ';
    /// \u{2d14}: 'ⴔ'
    pub const GEORGIAN_SMALL_LETTER_PHAR: char = 'ⴔ';
    /// \u{2d15}: 'ⴕ'
    pub const GEORGIAN_SMALL_LETTER_KHAR: char = 'ⴕ';
    /// \u{2d16}: 'ⴖ'
    pub const GEORGIAN_SMALL_LETTER_GHAN: char = 'ⴖ';
    /// \u{2d17}: 'ⴗ'
    pub const GEORGIAN_SMALL_LETTER_QAR: char = 'ⴗ';
    /// \u{2d18}: 'ⴘ'
    pub const GEORGIAN_SMALL_LETTER_SHIN: char = 'ⴘ';
    /// \u{2d19}: 'ⴙ'
    pub const GEORGIAN_SMALL_LETTER_CHIN: char = 'ⴙ';
    /// \u{2d1a}: 'ⴚ'
    pub const GEORGIAN_SMALL_LETTER_CAN: char = 'ⴚ';
    /// \u{2d1b}: 'ⴛ'
    pub const GEORGIAN_SMALL_LETTER_JIL: char = 'ⴛ';
    /// \u{2d1c}: 'ⴜ'
    pub const GEORGIAN_SMALL_LETTER_CIL: char = 'ⴜ';
    /// \u{2d1d}: 'ⴝ'
    pub const GEORGIAN_SMALL_LETTER_CHAR: char = 'ⴝ';
    /// \u{2d1e}: 'ⴞ'
    pub const GEORGIAN_SMALL_LETTER_XAN: char = 'ⴞ';
    /// \u{2d1f}: 'ⴟ'
    pub const GEORGIAN_SMALL_LETTER_JHAN: char = 'ⴟ';
    /// \u{2d20}: 'ⴠ'
    pub const GEORGIAN_SMALL_LETTER_HAE: char = 'ⴠ';
    /// \u{2d21}: 'ⴡ'
    pub const GEORGIAN_SMALL_LETTER_HE: char = 'ⴡ';
    /// \u{2d22}: 'ⴢ'
    pub const GEORGIAN_SMALL_LETTER_HIE: char = 'ⴢ';
    /// \u{2d23}: 'ⴣ'
    pub const GEORGIAN_SMALL_LETTER_WE: char = 'ⴣ';
    /// \u{2d24}: 'ⴤ'
    pub const GEORGIAN_SMALL_LETTER_HAR: char = 'ⴤ';
    /// \u{2d25}: 'ⴥ'
    pub const GEORGIAN_SMALL_LETTER_HOE: char = 'ⴥ';
    /// \u{2d27}: 'ⴧ'
    pub const GEORGIAN_SMALL_LETTER_YN: char = 'ⴧ';
    /// \u{2d2d}: 'ⴭ'
    pub const GEORGIAN_SMALL_LETTER_AEN: char = 'ⴭ';
}

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
        use constants::*;
        match self {
            GeorgianSupplement::GeorgianSmallLetterAn => GEORGIAN_SMALL_LETTER_AN,
            GeorgianSupplement::GeorgianSmallLetterBan => GEORGIAN_SMALL_LETTER_BAN,
            GeorgianSupplement::GeorgianSmallLetterGan => GEORGIAN_SMALL_LETTER_GAN,
            GeorgianSupplement::GeorgianSmallLetterDon => GEORGIAN_SMALL_LETTER_DON,
            GeorgianSupplement::GeorgianSmallLetterEn => GEORGIAN_SMALL_LETTER_EN,
            GeorgianSupplement::GeorgianSmallLetterVin => GEORGIAN_SMALL_LETTER_VIN,
            GeorgianSupplement::GeorgianSmallLetterZen => GEORGIAN_SMALL_LETTER_ZEN,
            GeorgianSupplement::GeorgianSmallLetterTan => GEORGIAN_SMALL_LETTER_TAN,
            GeorgianSupplement::GeorgianSmallLetterIn => GEORGIAN_SMALL_LETTER_IN,
            GeorgianSupplement::GeorgianSmallLetterKan => GEORGIAN_SMALL_LETTER_KAN,
            GeorgianSupplement::GeorgianSmallLetterLas => GEORGIAN_SMALL_LETTER_LAS,
            GeorgianSupplement::GeorgianSmallLetterMan => GEORGIAN_SMALL_LETTER_MAN,
            GeorgianSupplement::GeorgianSmallLetterNar => GEORGIAN_SMALL_LETTER_NAR,
            GeorgianSupplement::GeorgianSmallLetterOn => GEORGIAN_SMALL_LETTER_ON,
            GeorgianSupplement::GeorgianSmallLetterPar => GEORGIAN_SMALL_LETTER_PAR,
            GeorgianSupplement::GeorgianSmallLetterZhar => GEORGIAN_SMALL_LETTER_ZHAR,
            GeorgianSupplement::GeorgianSmallLetterRae => GEORGIAN_SMALL_LETTER_RAE,
            GeorgianSupplement::GeorgianSmallLetterSan => GEORGIAN_SMALL_LETTER_SAN,
            GeorgianSupplement::GeorgianSmallLetterTar => GEORGIAN_SMALL_LETTER_TAR,
            GeorgianSupplement::GeorgianSmallLetterUn => GEORGIAN_SMALL_LETTER_UN,
            GeorgianSupplement::GeorgianSmallLetterPhar => GEORGIAN_SMALL_LETTER_PHAR,
            GeorgianSupplement::GeorgianSmallLetterKhar => GEORGIAN_SMALL_LETTER_KHAR,
            GeorgianSupplement::GeorgianSmallLetterGhan => GEORGIAN_SMALL_LETTER_GHAN,
            GeorgianSupplement::GeorgianSmallLetterQar => GEORGIAN_SMALL_LETTER_QAR,
            GeorgianSupplement::GeorgianSmallLetterShin => GEORGIAN_SMALL_LETTER_SHIN,
            GeorgianSupplement::GeorgianSmallLetterChin => GEORGIAN_SMALL_LETTER_CHIN,
            GeorgianSupplement::GeorgianSmallLetterCan => GEORGIAN_SMALL_LETTER_CAN,
            GeorgianSupplement::GeorgianSmallLetterJil => GEORGIAN_SMALL_LETTER_JIL,
            GeorgianSupplement::GeorgianSmallLetterCil => GEORGIAN_SMALL_LETTER_CIL,
            GeorgianSupplement::GeorgianSmallLetterChar => GEORGIAN_SMALL_LETTER_CHAR,
            GeorgianSupplement::GeorgianSmallLetterXan => GEORGIAN_SMALL_LETTER_XAN,
            GeorgianSupplement::GeorgianSmallLetterJhan => GEORGIAN_SMALL_LETTER_JHAN,
            GeorgianSupplement::GeorgianSmallLetterHae => GEORGIAN_SMALL_LETTER_HAE,
            GeorgianSupplement::GeorgianSmallLetterHe => GEORGIAN_SMALL_LETTER_HE,
            GeorgianSupplement::GeorgianSmallLetterHie => GEORGIAN_SMALL_LETTER_HIE,
            GeorgianSupplement::GeorgianSmallLetterWe => GEORGIAN_SMALL_LETTER_WE,
            GeorgianSupplement::GeorgianSmallLetterHar => GEORGIAN_SMALL_LETTER_HAR,
            GeorgianSupplement::GeorgianSmallLetterHoe => GEORGIAN_SMALL_LETTER_HOE,
            GeorgianSupplement::GeorgianSmallLetterYn => GEORGIAN_SMALL_LETTER_YN,
            GeorgianSupplement::GeorgianSmallLetterAen => GEORGIAN_SMALL_LETTER_AEN,
        }
    }
}

impl std::convert::TryFrom<char> for GeorgianSupplement {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            GEORGIAN_SMALL_LETTER_AN => Ok(GeorgianSupplement::GeorgianSmallLetterAn),
            GEORGIAN_SMALL_LETTER_BAN => Ok(GeorgianSupplement::GeorgianSmallLetterBan),
            GEORGIAN_SMALL_LETTER_GAN => Ok(GeorgianSupplement::GeorgianSmallLetterGan),
            GEORGIAN_SMALL_LETTER_DON => Ok(GeorgianSupplement::GeorgianSmallLetterDon),
            GEORGIAN_SMALL_LETTER_EN => Ok(GeorgianSupplement::GeorgianSmallLetterEn),
            GEORGIAN_SMALL_LETTER_VIN => Ok(GeorgianSupplement::GeorgianSmallLetterVin),
            GEORGIAN_SMALL_LETTER_ZEN => Ok(GeorgianSupplement::GeorgianSmallLetterZen),
            GEORGIAN_SMALL_LETTER_TAN => Ok(GeorgianSupplement::GeorgianSmallLetterTan),
            GEORGIAN_SMALL_LETTER_IN => Ok(GeorgianSupplement::GeorgianSmallLetterIn),
            GEORGIAN_SMALL_LETTER_KAN => Ok(GeorgianSupplement::GeorgianSmallLetterKan),
            GEORGIAN_SMALL_LETTER_LAS => Ok(GeorgianSupplement::GeorgianSmallLetterLas),
            GEORGIAN_SMALL_LETTER_MAN => Ok(GeorgianSupplement::GeorgianSmallLetterMan),
            GEORGIAN_SMALL_LETTER_NAR => Ok(GeorgianSupplement::GeorgianSmallLetterNar),
            GEORGIAN_SMALL_LETTER_ON => Ok(GeorgianSupplement::GeorgianSmallLetterOn),
            GEORGIAN_SMALL_LETTER_PAR => Ok(GeorgianSupplement::GeorgianSmallLetterPar),
            GEORGIAN_SMALL_LETTER_ZHAR => Ok(GeorgianSupplement::GeorgianSmallLetterZhar),
            GEORGIAN_SMALL_LETTER_RAE => Ok(GeorgianSupplement::GeorgianSmallLetterRae),
            GEORGIAN_SMALL_LETTER_SAN => Ok(GeorgianSupplement::GeorgianSmallLetterSan),
            GEORGIAN_SMALL_LETTER_TAR => Ok(GeorgianSupplement::GeorgianSmallLetterTar),
            GEORGIAN_SMALL_LETTER_UN => Ok(GeorgianSupplement::GeorgianSmallLetterUn),
            GEORGIAN_SMALL_LETTER_PHAR => Ok(GeorgianSupplement::GeorgianSmallLetterPhar),
            GEORGIAN_SMALL_LETTER_KHAR => Ok(GeorgianSupplement::GeorgianSmallLetterKhar),
            GEORGIAN_SMALL_LETTER_GHAN => Ok(GeorgianSupplement::GeorgianSmallLetterGhan),
            GEORGIAN_SMALL_LETTER_QAR => Ok(GeorgianSupplement::GeorgianSmallLetterQar),
            GEORGIAN_SMALL_LETTER_SHIN => Ok(GeorgianSupplement::GeorgianSmallLetterShin),
            GEORGIAN_SMALL_LETTER_CHIN => Ok(GeorgianSupplement::GeorgianSmallLetterChin),
            GEORGIAN_SMALL_LETTER_CAN => Ok(GeorgianSupplement::GeorgianSmallLetterCan),
            GEORGIAN_SMALL_LETTER_JIL => Ok(GeorgianSupplement::GeorgianSmallLetterJil),
            GEORGIAN_SMALL_LETTER_CIL => Ok(GeorgianSupplement::GeorgianSmallLetterCil),
            GEORGIAN_SMALL_LETTER_CHAR => Ok(GeorgianSupplement::GeorgianSmallLetterChar),
            GEORGIAN_SMALL_LETTER_XAN => Ok(GeorgianSupplement::GeorgianSmallLetterXan),
            GEORGIAN_SMALL_LETTER_JHAN => Ok(GeorgianSupplement::GeorgianSmallLetterJhan),
            GEORGIAN_SMALL_LETTER_HAE => Ok(GeorgianSupplement::GeorgianSmallLetterHae),
            GEORGIAN_SMALL_LETTER_HE => Ok(GeorgianSupplement::GeorgianSmallLetterHe),
            GEORGIAN_SMALL_LETTER_HIE => Ok(GeorgianSupplement::GeorgianSmallLetterHie),
            GEORGIAN_SMALL_LETTER_WE => Ok(GeorgianSupplement::GeorgianSmallLetterWe),
            GEORGIAN_SMALL_LETTER_HAR => Ok(GeorgianSupplement::GeorgianSmallLetterHar),
            GEORGIAN_SMALL_LETTER_HOE => Ok(GeorgianSupplement::GeorgianSmallLetterHoe),
            GEORGIAN_SMALL_LETTER_YN => Ok(GeorgianSupplement::GeorgianSmallLetterYn),
            GEORGIAN_SMALL_LETTER_AEN => Ok(GeorgianSupplement::GeorgianSmallLetterAen),
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

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            GeorgianSupplement::GeorgianSmallLetterAn => "georgian small letter an",
            GeorgianSupplement::GeorgianSmallLetterBan => "georgian small letter ban",
            GeorgianSupplement::GeorgianSmallLetterGan => "georgian small letter gan",
            GeorgianSupplement::GeorgianSmallLetterDon => "georgian small letter don",
            GeorgianSupplement::GeorgianSmallLetterEn => "georgian small letter en",
            GeorgianSupplement::GeorgianSmallLetterVin => "georgian small letter vin",
            GeorgianSupplement::GeorgianSmallLetterZen => "georgian small letter zen",
            GeorgianSupplement::GeorgianSmallLetterTan => "georgian small letter tan",
            GeorgianSupplement::GeorgianSmallLetterIn => "georgian small letter in",
            GeorgianSupplement::GeorgianSmallLetterKan => "georgian small letter kan",
            GeorgianSupplement::GeorgianSmallLetterLas => "georgian small letter las",
            GeorgianSupplement::GeorgianSmallLetterMan => "georgian small letter man",
            GeorgianSupplement::GeorgianSmallLetterNar => "georgian small letter nar",
            GeorgianSupplement::GeorgianSmallLetterOn => "georgian small letter on",
            GeorgianSupplement::GeorgianSmallLetterPar => "georgian small letter par",
            GeorgianSupplement::GeorgianSmallLetterZhar => "georgian small letter zhar",
            GeorgianSupplement::GeorgianSmallLetterRae => "georgian small letter rae",
            GeorgianSupplement::GeorgianSmallLetterSan => "georgian small letter san",
            GeorgianSupplement::GeorgianSmallLetterTar => "georgian small letter tar",
            GeorgianSupplement::GeorgianSmallLetterUn => "georgian small letter un",
            GeorgianSupplement::GeorgianSmallLetterPhar => "georgian small letter phar",
            GeorgianSupplement::GeorgianSmallLetterKhar => "georgian small letter khar",
            GeorgianSupplement::GeorgianSmallLetterGhan => "georgian small letter ghan",
            GeorgianSupplement::GeorgianSmallLetterQar => "georgian small letter qar",
            GeorgianSupplement::GeorgianSmallLetterShin => "georgian small letter shin",
            GeorgianSupplement::GeorgianSmallLetterChin => "georgian small letter chin",
            GeorgianSupplement::GeorgianSmallLetterCan => "georgian small letter can",
            GeorgianSupplement::GeorgianSmallLetterJil => "georgian small letter jil",
            GeorgianSupplement::GeorgianSmallLetterCil => "georgian small letter cil",
            GeorgianSupplement::GeorgianSmallLetterChar => "georgian small letter char",
            GeorgianSupplement::GeorgianSmallLetterXan => "georgian small letter xan",
            GeorgianSupplement::GeorgianSmallLetterJhan => "georgian small letter jhan",
            GeorgianSupplement::GeorgianSmallLetterHae => "georgian small letter hae",
            GeorgianSupplement::GeorgianSmallLetterHe => "georgian small letter he",
            GeorgianSupplement::GeorgianSmallLetterHie => "georgian small letter hie",
            GeorgianSupplement::GeorgianSmallLetterWe => "georgian small letter we",
            GeorgianSupplement::GeorgianSmallLetterHar => "georgian small letter har",
            GeorgianSupplement::GeorgianSmallLetterHoe => "georgian small letter hoe",
            GeorgianSupplement::GeorgianSmallLetterYn => "georgian small letter yn",
            GeorgianSupplement::GeorgianSmallLetterAen => "georgian small letter aen",
        }
    }
}
