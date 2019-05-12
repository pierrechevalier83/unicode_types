
/// An enum to represent all characters in the GeorgianExtended block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum GeorgianExtended {
    /// \u{1c90}: 'Ა'
    GeorgianMtavruliCapitalLetterAn,
    /// \u{1c91}: 'Ბ'
    GeorgianMtavruliCapitalLetterBan,
    /// \u{1c92}: 'Გ'
    GeorgianMtavruliCapitalLetterGan,
    /// \u{1c93}: 'Დ'
    GeorgianMtavruliCapitalLetterDon,
    /// \u{1c94}: 'Ე'
    GeorgianMtavruliCapitalLetterEn,
    /// \u{1c95}: 'Ვ'
    GeorgianMtavruliCapitalLetterVin,
    /// \u{1c96}: 'Ზ'
    GeorgianMtavruliCapitalLetterZen,
    /// \u{1c97}: 'Თ'
    GeorgianMtavruliCapitalLetterTan,
    /// \u{1c98}: 'Ი'
    GeorgianMtavruliCapitalLetterIn,
    /// \u{1c99}: 'Კ'
    GeorgianMtavruliCapitalLetterKan,
    /// \u{1c9a}: 'Ლ'
    GeorgianMtavruliCapitalLetterLas,
    /// \u{1c9b}: 'Მ'
    GeorgianMtavruliCapitalLetterMan,
    /// \u{1c9c}: 'Ნ'
    GeorgianMtavruliCapitalLetterNar,
    /// \u{1c9d}: 'Ო'
    GeorgianMtavruliCapitalLetterOn,
    /// \u{1c9e}: 'Პ'
    GeorgianMtavruliCapitalLetterPar,
    /// \u{1c9f}: 'Ჟ'
    GeorgianMtavruliCapitalLetterZhar,
    /// \u{1ca0}: 'Რ'
    GeorgianMtavruliCapitalLetterRae,
    /// \u{1ca1}: 'Ს'
    GeorgianMtavruliCapitalLetterSan,
    /// \u{1ca2}: 'Ტ'
    GeorgianMtavruliCapitalLetterTar,
    /// \u{1ca3}: 'Უ'
    GeorgianMtavruliCapitalLetterUn,
    /// \u{1ca4}: 'Ფ'
    GeorgianMtavruliCapitalLetterPhar,
    /// \u{1ca5}: 'Ქ'
    GeorgianMtavruliCapitalLetterKhar,
    /// \u{1ca6}: 'Ღ'
    GeorgianMtavruliCapitalLetterGhan,
    /// \u{1ca7}: 'Ყ'
    GeorgianMtavruliCapitalLetterQar,
    /// \u{1ca8}: 'Შ'
    GeorgianMtavruliCapitalLetterShin,
    /// \u{1ca9}: 'Ჩ'
    GeorgianMtavruliCapitalLetterChin,
    /// \u{1caa}: 'Ც'
    GeorgianMtavruliCapitalLetterCan,
    /// \u{1cab}: 'Ძ'
    GeorgianMtavruliCapitalLetterJil,
    /// \u{1cac}: 'Წ'
    GeorgianMtavruliCapitalLetterCil,
    /// \u{1cad}: 'Ჭ'
    GeorgianMtavruliCapitalLetterChar,
    /// \u{1cae}: 'Ხ'
    GeorgianMtavruliCapitalLetterXan,
    /// \u{1caf}: 'Ჯ'
    GeorgianMtavruliCapitalLetterJhan,
    /// \u{1cb0}: 'Ჰ'
    GeorgianMtavruliCapitalLetterHae,
    /// \u{1cb1}: 'Ჱ'
    GeorgianMtavruliCapitalLetterHe,
    /// \u{1cb2}: 'Ჲ'
    GeorgianMtavruliCapitalLetterHie,
    /// \u{1cb3}: 'Ჳ'
    GeorgianMtavruliCapitalLetterWe,
    /// \u{1cb4}: 'Ჴ'
    GeorgianMtavruliCapitalLetterHar,
    /// \u{1cb5}: 'Ჵ'
    GeorgianMtavruliCapitalLetterHoe,
    /// \u{1cb6}: 'Ჶ'
    GeorgianMtavruliCapitalLetterFi,
    /// \u{1cb7}: 'Ჷ'
    GeorgianMtavruliCapitalLetterYn,
    /// \u{1cb8}: 'Ჸ'
    GeorgianMtavruliCapitalLetterElifi,
    /// \u{1cb9}: 'Ჹ'
    GeorgianMtavruliCapitalLetterTurnedGan,
    /// \u{1cba}: 'Ჺ'
    GeorgianMtavruliCapitalLetterAin,
    /// \u{1cbd}: 'Ჽ'
    GeorgianMtavruliCapitalLetterAen,
    /// \u{1cbe}: 'Ჾ'
    GeorgianMtavruliCapitalLetterHardSign,
}

impl Into<char> for GeorgianExtended {
    fn into(self) -> char {
        match self {
            GeorgianExtended::GeorgianMtavruliCapitalLetterAn => 'Ა',
            GeorgianExtended::GeorgianMtavruliCapitalLetterBan => 'Ბ',
            GeorgianExtended::GeorgianMtavruliCapitalLetterGan => 'Გ',
            GeorgianExtended::GeorgianMtavruliCapitalLetterDon => 'Დ',
            GeorgianExtended::GeorgianMtavruliCapitalLetterEn => 'Ე',
            GeorgianExtended::GeorgianMtavruliCapitalLetterVin => 'Ვ',
            GeorgianExtended::GeorgianMtavruliCapitalLetterZen => 'Ზ',
            GeorgianExtended::GeorgianMtavruliCapitalLetterTan => 'Თ',
            GeorgianExtended::GeorgianMtavruliCapitalLetterIn => 'Ი',
            GeorgianExtended::GeorgianMtavruliCapitalLetterKan => 'Კ',
            GeorgianExtended::GeorgianMtavruliCapitalLetterLas => 'Ლ',
            GeorgianExtended::GeorgianMtavruliCapitalLetterMan => 'Მ',
            GeorgianExtended::GeorgianMtavruliCapitalLetterNar => 'Ნ',
            GeorgianExtended::GeorgianMtavruliCapitalLetterOn => 'Ო',
            GeorgianExtended::GeorgianMtavruliCapitalLetterPar => 'Პ',
            GeorgianExtended::GeorgianMtavruliCapitalLetterZhar => 'Ჟ',
            GeorgianExtended::GeorgianMtavruliCapitalLetterRae => 'Რ',
            GeorgianExtended::GeorgianMtavruliCapitalLetterSan => 'Ს',
            GeorgianExtended::GeorgianMtavruliCapitalLetterTar => 'Ტ',
            GeorgianExtended::GeorgianMtavruliCapitalLetterUn => 'Უ',
            GeorgianExtended::GeorgianMtavruliCapitalLetterPhar => 'Ფ',
            GeorgianExtended::GeorgianMtavruliCapitalLetterKhar => 'Ქ',
            GeorgianExtended::GeorgianMtavruliCapitalLetterGhan => 'Ღ',
            GeorgianExtended::GeorgianMtavruliCapitalLetterQar => 'Ყ',
            GeorgianExtended::GeorgianMtavruliCapitalLetterShin => 'Შ',
            GeorgianExtended::GeorgianMtavruliCapitalLetterChin => 'Ჩ',
            GeorgianExtended::GeorgianMtavruliCapitalLetterCan => 'Ც',
            GeorgianExtended::GeorgianMtavruliCapitalLetterJil => 'Ძ',
            GeorgianExtended::GeorgianMtavruliCapitalLetterCil => 'Წ',
            GeorgianExtended::GeorgianMtavruliCapitalLetterChar => 'Ჭ',
            GeorgianExtended::GeorgianMtavruliCapitalLetterXan => 'Ხ',
            GeorgianExtended::GeorgianMtavruliCapitalLetterJhan => 'Ჯ',
            GeorgianExtended::GeorgianMtavruliCapitalLetterHae => 'Ჰ',
            GeorgianExtended::GeorgianMtavruliCapitalLetterHe => 'Ჱ',
            GeorgianExtended::GeorgianMtavruliCapitalLetterHie => 'Ჲ',
            GeorgianExtended::GeorgianMtavruliCapitalLetterWe => 'Ჳ',
            GeorgianExtended::GeorgianMtavruliCapitalLetterHar => 'Ჴ',
            GeorgianExtended::GeorgianMtavruliCapitalLetterHoe => 'Ჵ',
            GeorgianExtended::GeorgianMtavruliCapitalLetterFi => 'Ჶ',
            GeorgianExtended::GeorgianMtavruliCapitalLetterYn => 'Ჷ',
            GeorgianExtended::GeorgianMtavruliCapitalLetterElifi => 'Ჸ',
            GeorgianExtended::GeorgianMtavruliCapitalLetterTurnedGan => 'Ჹ',
            GeorgianExtended::GeorgianMtavruliCapitalLetterAin => 'Ჺ',
            GeorgianExtended::GeorgianMtavruliCapitalLetterAen => 'Ჽ',
            GeorgianExtended::GeorgianMtavruliCapitalLetterHardSign => 'Ჾ',
        }
    }
}

impl std::convert::TryFrom<char> for GeorgianExtended {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'Ა' => Ok(GeorgianExtended::GeorgianMtavruliCapitalLetterAn),
            'Ბ' => Ok(GeorgianExtended::GeorgianMtavruliCapitalLetterBan),
            'Გ' => Ok(GeorgianExtended::GeorgianMtavruliCapitalLetterGan),
            'Დ' => Ok(GeorgianExtended::GeorgianMtavruliCapitalLetterDon),
            'Ე' => Ok(GeorgianExtended::GeorgianMtavruliCapitalLetterEn),
            'Ვ' => Ok(GeorgianExtended::GeorgianMtavruliCapitalLetterVin),
            'Ზ' => Ok(GeorgianExtended::GeorgianMtavruliCapitalLetterZen),
            'Თ' => Ok(GeorgianExtended::GeorgianMtavruliCapitalLetterTan),
            'Ი' => Ok(GeorgianExtended::GeorgianMtavruliCapitalLetterIn),
            'Კ' => Ok(GeorgianExtended::GeorgianMtavruliCapitalLetterKan),
            'Ლ' => Ok(GeorgianExtended::GeorgianMtavruliCapitalLetterLas),
            'Მ' => Ok(GeorgianExtended::GeorgianMtavruliCapitalLetterMan),
            'Ნ' => Ok(GeorgianExtended::GeorgianMtavruliCapitalLetterNar),
            'Ო' => Ok(GeorgianExtended::GeorgianMtavruliCapitalLetterOn),
            'Პ' => Ok(GeorgianExtended::GeorgianMtavruliCapitalLetterPar),
            'Ჟ' => Ok(GeorgianExtended::GeorgianMtavruliCapitalLetterZhar),
            'Რ' => Ok(GeorgianExtended::GeorgianMtavruliCapitalLetterRae),
            'Ს' => Ok(GeorgianExtended::GeorgianMtavruliCapitalLetterSan),
            'Ტ' => Ok(GeorgianExtended::GeorgianMtavruliCapitalLetterTar),
            'Უ' => Ok(GeorgianExtended::GeorgianMtavruliCapitalLetterUn),
            'Ფ' => Ok(GeorgianExtended::GeorgianMtavruliCapitalLetterPhar),
            'Ქ' => Ok(GeorgianExtended::GeorgianMtavruliCapitalLetterKhar),
            'Ღ' => Ok(GeorgianExtended::GeorgianMtavruliCapitalLetterGhan),
            'Ყ' => Ok(GeorgianExtended::GeorgianMtavruliCapitalLetterQar),
            'Შ' => Ok(GeorgianExtended::GeorgianMtavruliCapitalLetterShin),
            'Ჩ' => Ok(GeorgianExtended::GeorgianMtavruliCapitalLetterChin),
            'Ც' => Ok(GeorgianExtended::GeorgianMtavruliCapitalLetterCan),
            'Ძ' => Ok(GeorgianExtended::GeorgianMtavruliCapitalLetterJil),
            'Წ' => Ok(GeorgianExtended::GeorgianMtavruliCapitalLetterCil),
            'Ჭ' => Ok(GeorgianExtended::GeorgianMtavruliCapitalLetterChar),
            'Ხ' => Ok(GeorgianExtended::GeorgianMtavruliCapitalLetterXan),
            'Ჯ' => Ok(GeorgianExtended::GeorgianMtavruliCapitalLetterJhan),
            'Ჰ' => Ok(GeorgianExtended::GeorgianMtavruliCapitalLetterHae),
            'Ჱ' => Ok(GeorgianExtended::GeorgianMtavruliCapitalLetterHe),
            'Ჲ' => Ok(GeorgianExtended::GeorgianMtavruliCapitalLetterHie),
            'Ჳ' => Ok(GeorgianExtended::GeorgianMtavruliCapitalLetterWe),
            'Ჴ' => Ok(GeorgianExtended::GeorgianMtavruliCapitalLetterHar),
            'Ჵ' => Ok(GeorgianExtended::GeorgianMtavruliCapitalLetterHoe),
            'Ჶ' => Ok(GeorgianExtended::GeorgianMtavruliCapitalLetterFi),
            'Ჷ' => Ok(GeorgianExtended::GeorgianMtavruliCapitalLetterYn),
            'Ჸ' => Ok(GeorgianExtended::GeorgianMtavruliCapitalLetterElifi),
            'Ჹ' => Ok(GeorgianExtended::GeorgianMtavruliCapitalLetterTurnedGan),
            'Ჺ' => Ok(GeorgianExtended::GeorgianMtavruliCapitalLetterAin),
            'Ჽ' => Ok(GeorgianExtended::GeorgianMtavruliCapitalLetterAen),
            'Ჾ' => Ok(GeorgianExtended::GeorgianMtavruliCapitalLetterHardSign),
            _ => Err(()),
        }
    }
}

impl Into<u32> for GeorgianExtended {
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

impl std::convert::TryFrom<u32> for GeorgianExtended {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for GeorgianExtended {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl GeorgianExtended {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        GeorgianExtended::GeorgianMtavruliCapitalLetterAn
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            GeorgianExtended::GeorgianMtavruliCapitalLetterAn => "georgian mtavruli capital letter an",
            GeorgianExtended::GeorgianMtavruliCapitalLetterBan => "georgian mtavruli capital letter ban",
            GeorgianExtended::GeorgianMtavruliCapitalLetterGan => "georgian mtavruli capital letter gan",
            GeorgianExtended::GeorgianMtavruliCapitalLetterDon => "georgian mtavruli capital letter don",
            GeorgianExtended::GeorgianMtavruliCapitalLetterEn => "georgian mtavruli capital letter en",
            GeorgianExtended::GeorgianMtavruliCapitalLetterVin => "georgian mtavruli capital letter vin",
            GeorgianExtended::GeorgianMtavruliCapitalLetterZen => "georgian mtavruli capital letter zen",
            GeorgianExtended::GeorgianMtavruliCapitalLetterTan => "georgian mtavruli capital letter tan",
            GeorgianExtended::GeorgianMtavruliCapitalLetterIn => "georgian mtavruli capital letter in",
            GeorgianExtended::GeorgianMtavruliCapitalLetterKan => "georgian mtavruli capital letter kan",
            GeorgianExtended::GeorgianMtavruliCapitalLetterLas => "georgian mtavruli capital letter las",
            GeorgianExtended::GeorgianMtavruliCapitalLetterMan => "georgian mtavruli capital letter man",
            GeorgianExtended::GeorgianMtavruliCapitalLetterNar => "georgian mtavruli capital letter nar",
            GeorgianExtended::GeorgianMtavruliCapitalLetterOn => "georgian mtavruli capital letter on",
            GeorgianExtended::GeorgianMtavruliCapitalLetterPar => "georgian mtavruli capital letter par",
            GeorgianExtended::GeorgianMtavruliCapitalLetterZhar => "georgian mtavruli capital letter zhar",
            GeorgianExtended::GeorgianMtavruliCapitalLetterRae => "georgian mtavruli capital letter rae",
            GeorgianExtended::GeorgianMtavruliCapitalLetterSan => "georgian mtavruli capital letter san",
            GeorgianExtended::GeorgianMtavruliCapitalLetterTar => "georgian mtavruli capital letter tar",
            GeorgianExtended::GeorgianMtavruliCapitalLetterUn => "georgian mtavruli capital letter un",
            GeorgianExtended::GeorgianMtavruliCapitalLetterPhar => "georgian mtavruli capital letter phar",
            GeorgianExtended::GeorgianMtavruliCapitalLetterKhar => "georgian mtavruli capital letter khar",
            GeorgianExtended::GeorgianMtavruliCapitalLetterGhan => "georgian mtavruli capital letter ghan",
            GeorgianExtended::GeorgianMtavruliCapitalLetterQar => "georgian mtavruli capital letter qar",
            GeorgianExtended::GeorgianMtavruliCapitalLetterShin => "georgian mtavruli capital letter shin",
            GeorgianExtended::GeorgianMtavruliCapitalLetterChin => "georgian mtavruli capital letter chin",
            GeorgianExtended::GeorgianMtavruliCapitalLetterCan => "georgian mtavruli capital letter can",
            GeorgianExtended::GeorgianMtavruliCapitalLetterJil => "georgian mtavruli capital letter jil",
            GeorgianExtended::GeorgianMtavruliCapitalLetterCil => "georgian mtavruli capital letter cil",
            GeorgianExtended::GeorgianMtavruliCapitalLetterChar => "georgian mtavruli capital letter char",
            GeorgianExtended::GeorgianMtavruliCapitalLetterXan => "georgian mtavruli capital letter xan",
            GeorgianExtended::GeorgianMtavruliCapitalLetterJhan => "georgian mtavruli capital letter jhan",
            GeorgianExtended::GeorgianMtavruliCapitalLetterHae => "georgian mtavruli capital letter hae",
            GeorgianExtended::GeorgianMtavruliCapitalLetterHe => "georgian mtavruli capital letter he",
            GeorgianExtended::GeorgianMtavruliCapitalLetterHie => "georgian mtavruli capital letter hie",
            GeorgianExtended::GeorgianMtavruliCapitalLetterWe => "georgian mtavruli capital letter we",
            GeorgianExtended::GeorgianMtavruliCapitalLetterHar => "georgian mtavruli capital letter har",
            GeorgianExtended::GeorgianMtavruliCapitalLetterHoe => "georgian mtavruli capital letter hoe",
            GeorgianExtended::GeorgianMtavruliCapitalLetterFi => "georgian mtavruli capital letter fi",
            GeorgianExtended::GeorgianMtavruliCapitalLetterYn => "georgian mtavruli capital letter yn",
            GeorgianExtended::GeorgianMtavruliCapitalLetterElifi => "georgian mtavruli capital letter elifi",
            GeorgianExtended::GeorgianMtavruliCapitalLetterTurnedGan => "georgian mtavruli capital letter turned gan",
            GeorgianExtended::GeorgianMtavruliCapitalLetterAin => "georgian mtavruli capital letter ain",
            GeorgianExtended::GeorgianMtavruliCapitalLetterAen => "georgian mtavruli capital letter aen",
            GeorgianExtended::GeorgianMtavruliCapitalLetterHardSign => "georgian mtavruli capital letter hard sign",
        }
    }
}
