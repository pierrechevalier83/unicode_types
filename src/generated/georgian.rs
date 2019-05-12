
/// An enum to represent all characters in the Georgian block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Georgian {
    /// \u{10a0}: 'Ⴀ'
    CapitalLetterAn,
    /// \u{10a1}: 'Ⴁ'
    CapitalLetterBan,
    /// \u{10a2}: 'Ⴂ'
    CapitalLetterGan,
    /// \u{10a3}: 'Ⴃ'
    CapitalLetterDon,
    /// \u{10a4}: 'Ⴄ'
    CapitalLetterEn,
    /// \u{10a5}: 'Ⴅ'
    CapitalLetterVin,
    /// \u{10a6}: 'Ⴆ'
    CapitalLetterZen,
    /// \u{10a7}: 'Ⴇ'
    CapitalLetterTan,
    /// \u{10a8}: 'Ⴈ'
    CapitalLetterIn,
    /// \u{10a9}: 'Ⴉ'
    CapitalLetterKan,
    /// \u{10aa}: 'Ⴊ'
    CapitalLetterLas,
    /// \u{10ab}: 'Ⴋ'
    CapitalLetterMan,
    /// \u{10ac}: 'Ⴌ'
    CapitalLetterNar,
    /// \u{10ad}: 'Ⴍ'
    CapitalLetterOn,
    /// \u{10ae}: 'Ⴎ'
    CapitalLetterPar,
    /// \u{10af}: 'Ⴏ'
    CapitalLetterZhar,
    /// \u{10b0}: 'Ⴐ'
    CapitalLetterRae,
    /// \u{10b1}: 'Ⴑ'
    CapitalLetterSan,
    /// \u{10b2}: 'Ⴒ'
    CapitalLetterTar,
    /// \u{10b3}: 'Ⴓ'
    CapitalLetterUn,
    /// \u{10b4}: 'Ⴔ'
    CapitalLetterPhar,
    /// \u{10b5}: 'Ⴕ'
    CapitalLetterKhar,
    /// \u{10b6}: 'Ⴖ'
    CapitalLetterGhan,
    /// \u{10b7}: 'Ⴗ'
    CapitalLetterQar,
    /// \u{10b8}: 'Ⴘ'
    CapitalLetterShin,
    /// \u{10b9}: 'Ⴙ'
    CapitalLetterChin,
    /// \u{10ba}: 'Ⴚ'
    CapitalLetterCan,
    /// \u{10bb}: 'Ⴛ'
    CapitalLetterJil,
    /// \u{10bc}: 'Ⴜ'
    CapitalLetterCil,
    /// \u{10bd}: 'Ⴝ'
    CapitalLetterChar,
    /// \u{10be}: 'Ⴞ'
    CapitalLetterXan,
    /// \u{10bf}: 'Ⴟ'
    CapitalLetterJhan,
    /// \u{10c0}: 'Ⴠ'
    CapitalLetterHae,
    /// \u{10c1}: 'Ⴡ'
    CapitalLetterHe,
    /// \u{10c2}: 'Ⴢ'
    CapitalLetterHie,
    /// \u{10c3}: 'Ⴣ'
    CapitalLetterWe,
    /// \u{10c4}: 'Ⴤ'
    CapitalLetterHar,
    /// \u{10c5}: 'Ⴥ'
    CapitalLetterHoe,
    /// \u{10c7}: 'Ⴧ'
    CapitalLetterYn,
    /// \u{10cd}: 'Ⴭ'
    CapitalLetterAen,
    /// \u{10d0}: 'ა'
    LetterAn,
    /// \u{10d1}: 'ბ'
    LetterBan,
    /// \u{10d2}: 'გ'
    LetterGan,
    /// \u{10d3}: 'დ'
    LetterDon,
    /// \u{10d4}: 'ე'
    LetterEn,
    /// \u{10d5}: 'ვ'
    LetterVin,
    /// \u{10d6}: 'ზ'
    LetterZen,
    /// \u{10d7}: 'თ'
    LetterTan,
    /// \u{10d8}: 'ი'
    LetterIn,
    /// \u{10d9}: 'კ'
    LetterKan,
    /// \u{10da}: 'ლ'
    LetterLas,
    /// \u{10db}: 'მ'
    LetterMan,
    /// \u{10dc}: 'ნ'
    LetterNar,
    /// \u{10dd}: 'ო'
    LetterOn,
    /// \u{10de}: 'პ'
    LetterPar,
    /// \u{10df}: 'ჟ'
    LetterZhar,
    /// \u{10e0}: 'რ'
    LetterRae,
    /// \u{10e1}: 'ს'
    LetterSan,
    /// \u{10e2}: 'ტ'
    LetterTar,
    /// \u{10e3}: 'უ'
    LetterUn,
    /// \u{10e4}: 'ფ'
    LetterPhar,
    /// \u{10e5}: 'ქ'
    LetterKhar,
    /// \u{10e6}: 'ღ'
    LetterGhan,
    /// \u{10e7}: 'ყ'
    LetterQar,
    /// \u{10e8}: 'შ'
    LetterShin,
    /// \u{10e9}: 'ჩ'
    LetterChin,
    /// \u{10ea}: 'ც'
    LetterCan,
    /// \u{10eb}: 'ძ'
    LetterJil,
    /// \u{10ec}: 'წ'
    LetterCil,
    /// \u{10ed}: 'ჭ'
    LetterChar,
    /// \u{10ee}: 'ხ'
    LetterXan,
    /// \u{10ef}: 'ჯ'
    LetterJhan,
    /// \u{10f0}: 'ჰ'
    LetterHae,
    /// \u{10f1}: 'ჱ'
    LetterHe,
    /// \u{10f2}: 'ჲ'
    LetterHie,
    /// \u{10f3}: 'ჳ'
    LetterWe,
    /// \u{10f4}: 'ჴ'
    LetterHar,
    /// \u{10f5}: 'ჵ'
    LetterHoe,
    /// \u{10f6}: 'ჶ'
    LetterFi,
    /// \u{10f7}: 'ჷ'
    LetterYn,
    /// \u{10f8}: 'ჸ'
    LetterElifi,
    /// \u{10f9}: 'ჹ'
    LetterTurnedGan,
    /// \u{10fa}: 'ჺ'
    LetterAin,
    /// \u{10fb}: '჻'
    ParagraphSeparator,
    /// \u{10fc}: 'ჼ'
    ModifierLetterNar,
    /// \u{10fd}: 'ჽ'
    LetterAen,
    /// \u{10fe}: 'ჾ'
    LetterHardSign,
}

impl Into<char> for Georgian {
    fn into(self) -> char {
        match self {
            Georgian::CapitalLetterAn => 'Ⴀ',
            Georgian::CapitalLetterBan => 'Ⴁ',
            Georgian::CapitalLetterGan => 'Ⴂ',
            Georgian::CapitalLetterDon => 'Ⴃ',
            Georgian::CapitalLetterEn => 'Ⴄ',
            Georgian::CapitalLetterVin => 'Ⴅ',
            Georgian::CapitalLetterZen => 'Ⴆ',
            Georgian::CapitalLetterTan => 'Ⴇ',
            Georgian::CapitalLetterIn => 'Ⴈ',
            Georgian::CapitalLetterKan => 'Ⴉ',
            Georgian::CapitalLetterLas => 'Ⴊ',
            Georgian::CapitalLetterMan => 'Ⴋ',
            Georgian::CapitalLetterNar => 'Ⴌ',
            Georgian::CapitalLetterOn => 'Ⴍ',
            Georgian::CapitalLetterPar => 'Ⴎ',
            Georgian::CapitalLetterZhar => 'Ⴏ',
            Georgian::CapitalLetterRae => 'Ⴐ',
            Georgian::CapitalLetterSan => 'Ⴑ',
            Georgian::CapitalLetterTar => 'Ⴒ',
            Georgian::CapitalLetterUn => 'Ⴓ',
            Georgian::CapitalLetterPhar => 'Ⴔ',
            Georgian::CapitalLetterKhar => 'Ⴕ',
            Georgian::CapitalLetterGhan => 'Ⴖ',
            Georgian::CapitalLetterQar => 'Ⴗ',
            Georgian::CapitalLetterShin => 'Ⴘ',
            Georgian::CapitalLetterChin => 'Ⴙ',
            Georgian::CapitalLetterCan => 'Ⴚ',
            Georgian::CapitalLetterJil => 'Ⴛ',
            Georgian::CapitalLetterCil => 'Ⴜ',
            Georgian::CapitalLetterChar => 'Ⴝ',
            Georgian::CapitalLetterXan => 'Ⴞ',
            Georgian::CapitalLetterJhan => 'Ⴟ',
            Georgian::CapitalLetterHae => 'Ⴠ',
            Georgian::CapitalLetterHe => 'Ⴡ',
            Georgian::CapitalLetterHie => 'Ⴢ',
            Georgian::CapitalLetterWe => 'Ⴣ',
            Georgian::CapitalLetterHar => 'Ⴤ',
            Georgian::CapitalLetterHoe => 'Ⴥ',
            Georgian::CapitalLetterYn => 'Ⴧ',
            Georgian::CapitalLetterAen => 'Ⴭ',
            Georgian::LetterAn => 'ა',
            Georgian::LetterBan => 'ბ',
            Georgian::LetterGan => 'გ',
            Georgian::LetterDon => 'დ',
            Georgian::LetterEn => 'ე',
            Georgian::LetterVin => 'ვ',
            Georgian::LetterZen => 'ზ',
            Georgian::LetterTan => 'თ',
            Georgian::LetterIn => 'ი',
            Georgian::LetterKan => 'კ',
            Georgian::LetterLas => 'ლ',
            Georgian::LetterMan => 'მ',
            Georgian::LetterNar => 'ნ',
            Georgian::LetterOn => 'ო',
            Georgian::LetterPar => 'პ',
            Georgian::LetterZhar => 'ჟ',
            Georgian::LetterRae => 'რ',
            Georgian::LetterSan => 'ს',
            Georgian::LetterTar => 'ტ',
            Georgian::LetterUn => 'უ',
            Georgian::LetterPhar => 'ფ',
            Georgian::LetterKhar => 'ქ',
            Georgian::LetterGhan => 'ღ',
            Georgian::LetterQar => 'ყ',
            Georgian::LetterShin => 'შ',
            Georgian::LetterChin => 'ჩ',
            Georgian::LetterCan => 'ც',
            Georgian::LetterJil => 'ძ',
            Georgian::LetterCil => 'წ',
            Georgian::LetterChar => 'ჭ',
            Georgian::LetterXan => 'ხ',
            Georgian::LetterJhan => 'ჯ',
            Georgian::LetterHae => 'ჰ',
            Georgian::LetterHe => 'ჱ',
            Georgian::LetterHie => 'ჲ',
            Georgian::LetterWe => 'ჳ',
            Georgian::LetterHar => 'ჴ',
            Georgian::LetterHoe => 'ჵ',
            Georgian::LetterFi => 'ჶ',
            Georgian::LetterYn => 'ჷ',
            Georgian::LetterElifi => 'ჸ',
            Georgian::LetterTurnedGan => 'ჹ',
            Georgian::LetterAin => 'ჺ',
            Georgian::ParagraphSeparator => '჻',
            Georgian::ModifierLetterNar => 'ჼ',
            Georgian::LetterAen => 'ჽ',
            Georgian::LetterHardSign => 'ჾ',
        }
    }
}

impl std::convert::TryFrom<char> for Georgian {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'Ⴀ' => Ok(Georgian::CapitalLetterAn),
            'Ⴁ' => Ok(Georgian::CapitalLetterBan),
            'Ⴂ' => Ok(Georgian::CapitalLetterGan),
            'Ⴃ' => Ok(Georgian::CapitalLetterDon),
            'Ⴄ' => Ok(Georgian::CapitalLetterEn),
            'Ⴅ' => Ok(Georgian::CapitalLetterVin),
            'Ⴆ' => Ok(Georgian::CapitalLetterZen),
            'Ⴇ' => Ok(Georgian::CapitalLetterTan),
            'Ⴈ' => Ok(Georgian::CapitalLetterIn),
            'Ⴉ' => Ok(Georgian::CapitalLetterKan),
            'Ⴊ' => Ok(Georgian::CapitalLetterLas),
            'Ⴋ' => Ok(Georgian::CapitalLetterMan),
            'Ⴌ' => Ok(Georgian::CapitalLetterNar),
            'Ⴍ' => Ok(Georgian::CapitalLetterOn),
            'Ⴎ' => Ok(Georgian::CapitalLetterPar),
            'Ⴏ' => Ok(Georgian::CapitalLetterZhar),
            'Ⴐ' => Ok(Georgian::CapitalLetterRae),
            'Ⴑ' => Ok(Georgian::CapitalLetterSan),
            'Ⴒ' => Ok(Georgian::CapitalLetterTar),
            'Ⴓ' => Ok(Georgian::CapitalLetterUn),
            'Ⴔ' => Ok(Georgian::CapitalLetterPhar),
            'Ⴕ' => Ok(Georgian::CapitalLetterKhar),
            'Ⴖ' => Ok(Georgian::CapitalLetterGhan),
            'Ⴗ' => Ok(Georgian::CapitalLetterQar),
            'Ⴘ' => Ok(Georgian::CapitalLetterShin),
            'Ⴙ' => Ok(Georgian::CapitalLetterChin),
            'Ⴚ' => Ok(Georgian::CapitalLetterCan),
            'Ⴛ' => Ok(Georgian::CapitalLetterJil),
            'Ⴜ' => Ok(Georgian::CapitalLetterCil),
            'Ⴝ' => Ok(Georgian::CapitalLetterChar),
            'Ⴞ' => Ok(Georgian::CapitalLetterXan),
            'Ⴟ' => Ok(Georgian::CapitalLetterJhan),
            'Ⴠ' => Ok(Georgian::CapitalLetterHae),
            'Ⴡ' => Ok(Georgian::CapitalLetterHe),
            'Ⴢ' => Ok(Georgian::CapitalLetterHie),
            'Ⴣ' => Ok(Georgian::CapitalLetterWe),
            'Ⴤ' => Ok(Georgian::CapitalLetterHar),
            'Ⴥ' => Ok(Georgian::CapitalLetterHoe),
            'Ⴧ' => Ok(Georgian::CapitalLetterYn),
            'Ⴭ' => Ok(Georgian::CapitalLetterAen),
            'ა' => Ok(Georgian::LetterAn),
            'ბ' => Ok(Georgian::LetterBan),
            'გ' => Ok(Georgian::LetterGan),
            'დ' => Ok(Georgian::LetterDon),
            'ე' => Ok(Georgian::LetterEn),
            'ვ' => Ok(Georgian::LetterVin),
            'ზ' => Ok(Georgian::LetterZen),
            'თ' => Ok(Georgian::LetterTan),
            'ი' => Ok(Georgian::LetterIn),
            'კ' => Ok(Georgian::LetterKan),
            'ლ' => Ok(Georgian::LetterLas),
            'მ' => Ok(Georgian::LetterMan),
            'ნ' => Ok(Georgian::LetterNar),
            'ო' => Ok(Georgian::LetterOn),
            'პ' => Ok(Georgian::LetterPar),
            'ჟ' => Ok(Georgian::LetterZhar),
            'რ' => Ok(Georgian::LetterRae),
            'ს' => Ok(Georgian::LetterSan),
            'ტ' => Ok(Georgian::LetterTar),
            'უ' => Ok(Georgian::LetterUn),
            'ფ' => Ok(Georgian::LetterPhar),
            'ქ' => Ok(Georgian::LetterKhar),
            'ღ' => Ok(Georgian::LetterGhan),
            'ყ' => Ok(Georgian::LetterQar),
            'შ' => Ok(Georgian::LetterShin),
            'ჩ' => Ok(Georgian::LetterChin),
            'ც' => Ok(Georgian::LetterCan),
            'ძ' => Ok(Georgian::LetterJil),
            'წ' => Ok(Georgian::LetterCil),
            'ჭ' => Ok(Georgian::LetterChar),
            'ხ' => Ok(Georgian::LetterXan),
            'ჯ' => Ok(Georgian::LetterJhan),
            'ჰ' => Ok(Georgian::LetterHae),
            'ჱ' => Ok(Georgian::LetterHe),
            'ჲ' => Ok(Georgian::LetterHie),
            'ჳ' => Ok(Georgian::LetterWe),
            'ჴ' => Ok(Georgian::LetterHar),
            'ჵ' => Ok(Georgian::LetterHoe),
            'ჶ' => Ok(Georgian::LetterFi),
            'ჷ' => Ok(Georgian::LetterYn),
            'ჸ' => Ok(Georgian::LetterElifi),
            'ჹ' => Ok(Georgian::LetterTurnedGan),
            'ჺ' => Ok(Georgian::LetterAin),
            '჻' => Ok(Georgian::ParagraphSeparator),
            'ჼ' => Ok(Georgian::ModifierLetterNar),
            'ჽ' => Ok(Georgian::LetterAen),
            'ჾ' => Ok(Georgian::LetterHardSign),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Georgian {
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

impl std::convert::TryFrom<u32> for Georgian {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Georgian {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Georgian {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Georgian::CapitalLetterAn
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Georgian::CapitalLetterAn => "georgian capital letter an",
            Georgian::CapitalLetterBan => "georgian capital letter ban",
            Georgian::CapitalLetterGan => "georgian capital letter gan",
            Georgian::CapitalLetterDon => "georgian capital letter don",
            Georgian::CapitalLetterEn => "georgian capital letter en",
            Georgian::CapitalLetterVin => "georgian capital letter vin",
            Georgian::CapitalLetterZen => "georgian capital letter zen",
            Georgian::CapitalLetterTan => "georgian capital letter tan",
            Georgian::CapitalLetterIn => "georgian capital letter in",
            Georgian::CapitalLetterKan => "georgian capital letter kan",
            Georgian::CapitalLetterLas => "georgian capital letter las",
            Georgian::CapitalLetterMan => "georgian capital letter man",
            Georgian::CapitalLetterNar => "georgian capital letter nar",
            Georgian::CapitalLetterOn => "georgian capital letter on",
            Georgian::CapitalLetterPar => "georgian capital letter par",
            Georgian::CapitalLetterZhar => "georgian capital letter zhar",
            Georgian::CapitalLetterRae => "georgian capital letter rae",
            Georgian::CapitalLetterSan => "georgian capital letter san",
            Georgian::CapitalLetterTar => "georgian capital letter tar",
            Georgian::CapitalLetterUn => "georgian capital letter un",
            Georgian::CapitalLetterPhar => "georgian capital letter phar",
            Georgian::CapitalLetterKhar => "georgian capital letter khar",
            Georgian::CapitalLetterGhan => "georgian capital letter ghan",
            Georgian::CapitalLetterQar => "georgian capital letter qar",
            Georgian::CapitalLetterShin => "georgian capital letter shin",
            Georgian::CapitalLetterChin => "georgian capital letter chin",
            Georgian::CapitalLetterCan => "georgian capital letter can",
            Georgian::CapitalLetterJil => "georgian capital letter jil",
            Georgian::CapitalLetterCil => "georgian capital letter cil",
            Georgian::CapitalLetterChar => "georgian capital letter char",
            Georgian::CapitalLetterXan => "georgian capital letter xan",
            Georgian::CapitalLetterJhan => "georgian capital letter jhan",
            Georgian::CapitalLetterHae => "georgian capital letter hae",
            Georgian::CapitalLetterHe => "georgian capital letter he",
            Georgian::CapitalLetterHie => "georgian capital letter hie",
            Georgian::CapitalLetterWe => "georgian capital letter we",
            Georgian::CapitalLetterHar => "georgian capital letter har",
            Georgian::CapitalLetterHoe => "georgian capital letter hoe",
            Georgian::CapitalLetterYn => "georgian capital letter yn",
            Georgian::CapitalLetterAen => "georgian capital letter aen",
            Georgian::LetterAn => "georgian letter an",
            Georgian::LetterBan => "georgian letter ban",
            Georgian::LetterGan => "georgian letter gan",
            Georgian::LetterDon => "georgian letter don",
            Georgian::LetterEn => "georgian letter en",
            Georgian::LetterVin => "georgian letter vin",
            Georgian::LetterZen => "georgian letter zen",
            Georgian::LetterTan => "georgian letter tan",
            Georgian::LetterIn => "georgian letter in",
            Georgian::LetterKan => "georgian letter kan",
            Georgian::LetterLas => "georgian letter las",
            Georgian::LetterMan => "georgian letter man",
            Georgian::LetterNar => "georgian letter nar",
            Georgian::LetterOn => "georgian letter on",
            Georgian::LetterPar => "georgian letter par",
            Georgian::LetterZhar => "georgian letter zhar",
            Georgian::LetterRae => "georgian letter rae",
            Georgian::LetterSan => "georgian letter san",
            Georgian::LetterTar => "georgian letter tar",
            Georgian::LetterUn => "georgian letter un",
            Georgian::LetterPhar => "georgian letter phar",
            Georgian::LetterKhar => "georgian letter khar",
            Georgian::LetterGhan => "georgian letter ghan",
            Georgian::LetterQar => "georgian letter qar",
            Georgian::LetterShin => "georgian letter shin",
            Georgian::LetterChin => "georgian letter chin",
            Georgian::LetterCan => "georgian letter can",
            Georgian::LetterJil => "georgian letter jil",
            Georgian::LetterCil => "georgian letter cil",
            Georgian::LetterChar => "georgian letter char",
            Georgian::LetterXan => "georgian letter xan",
            Georgian::LetterJhan => "georgian letter jhan",
            Georgian::LetterHae => "georgian letter hae",
            Georgian::LetterHe => "georgian letter he",
            Georgian::LetterHie => "georgian letter hie",
            Georgian::LetterWe => "georgian letter we",
            Georgian::LetterHar => "georgian letter har",
            Georgian::LetterHoe => "georgian letter hoe",
            Georgian::LetterFi => "georgian letter fi",
            Georgian::LetterYn => "georgian letter yn",
            Georgian::LetterElifi => "georgian letter elifi",
            Georgian::LetterTurnedGan => "georgian letter turned gan",
            Georgian::LetterAin => "georgian letter ain",
            Georgian::ParagraphSeparator => "georgian paragraph separator",
            Georgian::ModifierLetterNar => "modifier letter georgian nar",
            Georgian::LetterAen => "georgian letter aen",
            Georgian::LetterHardSign => "georgian letter hard sign",
        }
    }
}
