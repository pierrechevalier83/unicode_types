/// \u{10a0} → \u{10ff}
///
/// Ⴀ Ⴁ Ⴂ Ⴃ Ⴄ Ⴅ Ⴆ Ⴇ Ⴈ Ⴉ Ⴊ Ⴋ Ⴌ Ⴍ Ⴎ Ⴏ\
/// Ⴐ Ⴑ Ⴒ Ⴓ Ⴔ Ⴕ Ⴖ Ⴗ Ⴘ Ⴙ Ⴚ Ⴛ Ⴜ Ⴝ Ⴞ Ⴟ\
/// Ⴠ Ⴡ Ⴢ Ⴣ Ⴤ Ⴥ Ⴧ Ⴭ ა ბ გ დ ე ვ ზ თ\
/// ი კ ლ მ ნ ო პ ჟ რ ს ტ უ ფ ქ ღ ყ\
/// შ ჩ ც ძ წ ჭ ხ ჯ ჰ ჱ ჲ ჳ ჴ ჵ ჶ ჷ\
/// ჸ ჹ ჺ ჻ ჼ ჽ ჾ\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{10a0}: 'Ⴀ'
    pub const CAPITAL_LETTER_AN: char = 'Ⴀ';
    /// \u{10a1}: 'Ⴁ'
    pub const CAPITAL_LETTER_BAN: char = 'Ⴁ';
    /// \u{10a2}: 'Ⴂ'
    pub const CAPITAL_LETTER_GAN: char = 'Ⴂ';
    /// \u{10a3}: 'Ⴃ'
    pub const CAPITAL_LETTER_DON: char = 'Ⴃ';
    /// \u{10a4}: 'Ⴄ'
    pub const CAPITAL_LETTER_EN: char = 'Ⴄ';
    /// \u{10a5}: 'Ⴅ'
    pub const CAPITAL_LETTER_VIN: char = 'Ⴅ';
    /// \u{10a6}: 'Ⴆ'
    pub const CAPITAL_LETTER_ZEN: char = 'Ⴆ';
    /// \u{10a7}: 'Ⴇ'
    pub const CAPITAL_LETTER_TAN: char = 'Ⴇ';
    /// \u{10a8}: 'Ⴈ'
    pub const CAPITAL_LETTER_IN: char = 'Ⴈ';
    /// \u{10a9}: 'Ⴉ'
    pub const CAPITAL_LETTER_KAN: char = 'Ⴉ';
    /// \u{10aa}: 'Ⴊ'
    pub const CAPITAL_LETTER_LAS: char = 'Ⴊ';
    /// \u{10ab}: 'Ⴋ'
    pub const CAPITAL_LETTER_MAN: char = 'Ⴋ';
    /// \u{10ac}: 'Ⴌ'
    pub const CAPITAL_LETTER_NAR: char = 'Ⴌ';
    /// \u{10ad}: 'Ⴍ'
    pub const CAPITAL_LETTER_ON: char = 'Ⴍ';
    /// \u{10ae}: 'Ⴎ'
    pub const CAPITAL_LETTER_PAR: char = 'Ⴎ';
    /// \u{10af}: 'Ⴏ'
    pub const CAPITAL_LETTER_ZHAR: char = 'Ⴏ';
    /// \u{10b0}: 'Ⴐ'
    pub const CAPITAL_LETTER_RAE: char = 'Ⴐ';
    /// \u{10b1}: 'Ⴑ'
    pub const CAPITAL_LETTER_SAN: char = 'Ⴑ';
    /// \u{10b2}: 'Ⴒ'
    pub const CAPITAL_LETTER_TAR: char = 'Ⴒ';
    /// \u{10b3}: 'Ⴓ'
    pub const CAPITAL_LETTER_UN: char = 'Ⴓ';
    /// \u{10b4}: 'Ⴔ'
    pub const CAPITAL_LETTER_PHAR: char = 'Ⴔ';
    /// \u{10b5}: 'Ⴕ'
    pub const CAPITAL_LETTER_KHAR: char = 'Ⴕ';
    /// \u{10b6}: 'Ⴖ'
    pub const CAPITAL_LETTER_GHAN: char = 'Ⴖ';
    /// \u{10b7}: 'Ⴗ'
    pub const CAPITAL_LETTER_QAR: char = 'Ⴗ';
    /// \u{10b8}: 'Ⴘ'
    pub const CAPITAL_LETTER_SHIN: char = 'Ⴘ';
    /// \u{10b9}: 'Ⴙ'
    pub const CAPITAL_LETTER_CHIN: char = 'Ⴙ';
    /// \u{10ba}: 'Ⴚ'
    pub const CAPITAL_LETTER_CAN: char = 'Ⴚ';
    /// \u{10bb}: 'Ⴛ'
    pub const CAPITAL_LETTER_JIL: char = 'Ⴛ';
    /// \u{10bc}: 'Ⴜ'
    pub const CAPITAL_LETTER_CIL: char = 'Ⴜ';
    /// \u{10bd}: 'Ⴝ'
    pub const CAPITAL_LETTER_CHAR: char = 'Ⴝ';
    /// \u{10be}: 'Ⴞ'
    pub const CAPITAL_LETTER_XAN: char = 'Ⴞ';
    /// \u{10bf}: 'Ⴟ'
    pub const CAPITAL_LETTER_JHAN: char = 'Ⴟ';
    /// \u{10c0}: 'Ⴠ'
    pub const CAPITAL_LETTER_HAE: char = 'Ⴠ';
    /// \u{10c1}: 'Ⴡ'
    pub const CAPITAL_LETTER_HE: char = 'Ⴡ';
    /// \u{10c2}: 'Ⴢ'
    pub const CAPITAL_LETTER_HIE: char = 'Ⴢ';
    /// \u{10c3}: 'Ⴣ'
    pub const CAPITAL_LETTER_WE: char = 'Ⴣ';
    /// \u{10c4}: 'Ⴤ'
    pub const CAPITAL_LETTER_HAR: char = 'Ⴤ';
    /// \u{10c5}: 'Ⴥ'
    pub const CAPITAL_LETTER_HOE: char = 'Ⴥ';
    /// \u{10c7}: 'Ⴧ'
    pub const CAPITAL_LETTER_YN: char = 'Ⴧ';
    /// \u{10cd}: 'Ⴭ'
    pub const CAPITAL_LETTER_AEN: char = 'Ⴭ';
    /// \u{10d0}: 'ა'
    pub const LETTER_AN: char = 'ა';
    /// \u{10d1}: 'ბ'
    pub const LETTER_BAN: char = 'ბ';
    /// \u{10d2}: 'გ'
    pub const LETTER_GAN: char = 'გ';
    /// \u{10d3}: 'დ'
    pub const LETTER_DON: char = 'დ';
    /// \u{10d4}: 'ე'
    pub const LETTER_EN: char = 'ე';
    /// \u{10d5}: 'ვ'
    pub const LETTER_VIN: char = 'ვ';
    /// \u{10d6}: 'ზ'
    pub const LETTER_ZEN: char = 'ზ';
    /// \u{10d7}: 'თ'
    pub const LETTER_TAN: char = 'თ';
    /// \u{10d8}: 'ი'
    pub const LETTER_IN: char = 'ი';
    /// \u{10d9}: 'კ'
    pub const LETTER_KAN: char = 'კ';
    /// \u{10da}: 'ლ'
    pub const LETTER_LAS: char = 'ლ';
    /// \u{10db}: 'მ'
    pub const LETTER_MAN: char = 'მ';
    /// \u{10dc}: 'ნ'
    pub const LETTER_NAR: char = 'ნ';
    /// \u{10dd}: 'ო'
    pub const LETTER_ON: char = 'ო';
    /// \u{10de}: 'პ'
    pub const LETTER_PAR: char = 'პ';
    /// \u{10df}: 'ჟ'
    pub const LETTER_ZHAR: char = 'ჟ';
    /// \u{10e0}: 'რ'
    pub const LETTER_RAE: char = 'რ';
    /// \u{10e1}: 'ს'
    pub const LETTER_SAN: char = 'ს';
    /// \u{10e2}: 'ტ'
    pub const LETTER_TAR: char = 'ტ';
    /// \u{10e3}: 'უ'
    pub const LETTER_UN: char = 'უ';
    /// \u{10e4}: 'ფ'
    pub const LETTER_PHAR: char = 'ფ';
    /// \u{10e5}: 'ქ'
    pub const LETTER_KHAR: char = 'ქ';
    /// \u{10e6}: 'ღ'
    pub const LETTER_GHAN: char = 'ღ';
    /// \u{10e7}: 'ყ'
    pub const LETTER_QAR: char = 'ყ';
    /// \u{10e8}: 'შ'
    pub const LETTER_SHIN: char = 'შ';
    /// \u{10e9}: 'ჩ'
    pub const LETTER_CHIN: char = 'ჩ';
    /// \u{10ea}: 'ც'
    pub const LETTER_CAN: char = 'ც';
    /// \u{10eb}: 'ძ'
    pub const LETTER_JIL: char = 'ძ';
    /// \u{10ec}: 'წ'
    pub const LETTER_CIL: char = 'წ';
    /// \u{10ed}: 'ჭ'
    pub const LETTER_CHAR: char = 'ჭ';
    /// \u{10ee}: 'ხ'
    pub const LETTER_XAN: char = 'ხ';
    /// \u{10ef}: 'ჯ'
    pub const LETTER_JHAN: char = 'ჯ';
    /// \u{10f0}: 'ჰ'
    pub const LETTER_HAE: char = 'ჰ';
    /// \u{10f1}: 'ჱ'
    pub const LETTER_HE: char = 'ჱ';
    /// \u{10f2}: 'ჲ'
    pub const LETTER_HIE: char = 'ჲ';
    /// \u{10f3}: 'ჳ'
    pub const LETTER_WE: char = 'ჳ';
    /// \u{10f4}: 'ჴ'
    pub const LETTER_HAR: char = 'ჴ';
    /// \u{10f5}: 'ჵ'
    pub const LETTER_HOE: char = 'ჵ';
    /// \u{10f6}: 'ჶ'
    pub const LETTER_FI: char = 'ჶ';
    /// \u{10f7}: 'ჷ'
    pub const LETTER_YN: char = 'ჷ';
    /// \u{10f8}: 'ჸ'
    pub const LETTER_ELIFI: char = 'ჸ';
    /// \u{10f9}: 'ჹ'
    pub const LETTER_TURNED_GAN: char = 'ჹ';
    /// \u{10fa}: 'ჺ'
    pub const LETTER_AIN: char = 'ჺ';
    /// \u{10fb}: '჻'
    pub const PARAGRAPH_SEPARATOR: char = '჻';
    /// \u{10fc}: 'ჼ'
    pub const MODIFIER_LETTER_NAR: char = 'ჼ';
    /// \u{10fd}: 'ჽ'
    pub const LETTER_AEN: char = 'ჽ';
    /// \u{10fe}: 'ჾ'
    pub const LETTER_HARD_SIGN: char = 'ჾ';
}

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
        use constants::*;
        match self {
            Georgian::CapitalLetterAn => CAPITAL_LETTER_AN,
            Georgian::CapitalLetterBan => CAPITAL_LETTER_BAN,
            Georgian::CapitalLetterGan => CAPITAL_LETTER_GAN,
            Georgian::CapitalLetterDon => CAPITAL_LETTER_DON,
            Georgian::CapitalLetterEn => CAPITAL_LETTER_EN,
            Georgian::CapitalLetterVin => CAPITAL_LETTER_VIN,
            Georgian::CapitalLetterZen => CAPITAL_LETTER_ZEN,
            Georgian::CapitalLetterTan => CAPITAL_LETTER_TAN,
            Georgian::CapitalLetterIn => CAPITAL_LETTER_IN,
            Georgian::CapitalLetterKan => CAPITAL_LETTER_KAN,
            Georgian::CapitalLetterLas => CAPITAL_LETTER_LAS,
            Georgian::CapitalLetterMan => CAPITAL_LETTER_MAN,
            Georgian::CapitalLetterNar => CAPITAL_LETTER_NAR,
            Georgian::CapitalLetterOn => CAPITAL_LETTER_ON,
            Georgian::CapitalLetterPar => CAPITAL_LETTER_PAR,
            Georgian::CapitalLetterZhar => CAPITAL_LETTER_ZHAR,
            Georgian::CapitalLetterRae => CAPITAL_LETTER_RAE,
            Georgian::CapitalLetterSan => CAPITAL_LETTER_SAN,
            Georgian::CapitalLetterTar => CAPITAL_LETTER_TAR,
            Georgian::CapitalLetterUn => CAPITAL_LETTER_UN,
            Georgian::CapitalLetterPhar => CAPITAL_LETTER_PHAR,
            Georgian::CapitalLetterKhar => CAPITAL_LETTER_KHAR,
            Georgian::CapitalLetterGhan => CAPITAL_LETTER_GHAN,
            Georgian::CapitalLetterQar => CAPITAL_LETTER_QAR,
            Georgian::CapitalLetterShin => CAPITAL_LETTER_SHIN,
            Georgian::CapitalLetterChin => CAPITAL_LETTER_CHIN,
            Georgian::CapitalLetterCan => CAPITAL_LETTER_CAN,
            Georgian::CapitalLetterJil => CAPITAL_LETTER_JIL,
            Georgian::CapitalLetterCil => CAPITAL_LETTER_CIL,
            Georgian::CapitalLetterChar => CAPITAL_LETTER_CHAR,
            Georgian::CapitalLetterXan => CAPITAL_LETTER_XAN,
            Georgian::CapitalLetterJhan => CAPITAL_LETTER_JHAN,
            Georgian::CapitalLetterHae => CAPITAL_LETTER_HAE,
            Georgian::CapitalLetterHe => CAPITAL_LETTER_HE,
            Georgian::CapitalLetterHie => CAPITAL_LETTER_HIE,
            Georgian::CapitalLetterWe => CAPITAL_LETTER_WE,
            Georgian::CapitalLetterHar => CAPITAL_LETTER_HAR,
            Georgian::CapitalLetterHoe => CAPITAL_LETTER_HOE,
            Georgian::CapitalLetterYn => CAPITAL_LETTER_YN,
            Georgian::CapitalLetterAen => CAPITAL_LETTER_AEN,
            Georgian::LetterAn => LETTER_AN,
            Georgian::LetterBan => LETTER_BAN,
            Georgian::LetterGan => LETTER_GAN,
            Georgian::LetterDon => LETTER_DON,
            Georgian::LetterEn => LETTER_EN,
            Georgian::LetterVin => LETTER_VIN,
            Georgian::LetterZen => LETTER_ZEN,
            Georgian::LetterTan => LETTER_TAN,
            Georgian::LetterIn => LETTER_IN,
            Georgian::LetterKan => LETTER_KAN,
            Georgian::LetterLas => LETTER_LAS,
            Georgian::LetterMan => LETTER_MAN,
            Georgian::LetterNar => LETTER_NAR,
            Georgian::LetterOn => LETTER_ON,
            Georgian::LetterPar => LETTER_PAR,
            Georgian::LetterZhar => LETTER_ZHAR,
            Georgian::LetterRae => LETTER_RAE,
            Georgian::LetterSan => LETTER_SAN,
            Georgian::LetterTar => LETTER_TAR,
            Georgian::LetterUn => LETTER_UN,
            Georgian::LetterPhar => LETTER_PHAR,
            Georgian::LetterKhar => LETTER_KHAR,
            Georgian::LetterGhan => LETTER_GHAN,
            Georgian::LetterQar => LETTER_QAR,
            Georgian::LetterShin => LETTER_SHIN,
            Georgian::LetterChin => LETTER_CHIN,
            Georgian::LetterCan => LETTER_CAN,
            Georgian::LetterJil => LETTER_JIL,
            Georgian::LetterCil => LETTER_CIL,
            Georgian::LetterChar => LETTER_CHAR,
            Georgian::LetterXan => LETTER_XAN,
            Georgian::LetterJhan => LETTER_JHAN,
            Georgian::LetterHae => LETTER_HAE,
            Georgian::LetterHe => LETTER_HE,
            Georgian::LetterHie => LETTER_HIE,
            Georgian::LetterWe => LETTER_WE,
            Georgian::LetterHar => LETTER_HAR,
            Georgian::LetterHoe => LETTER_HOE,
            Georgian::LetterFi => LETTER_FI,
            Georgian::LetterYn => LETTER_YN,
            Georgian::LetterElifi => LETTER_ELIFI,
            Georgian::LetterTurnedGan => LETTER_TURNED_GAN,
            Georgian::LetterAin => LETTER_AIN,
            Georgian::ParagraphSeparator => PARAGRAPH_SEPARATOR,
            Georgian::ModifierLetterNar => MODIFIER_LETTER_NAR,
            Georgian::LetterAen => LETTER_AEN,
            Georgian::LetterHardSign => LETTER_HARD_SIGN,
        }
    }
}

impl std::convert::TryFrom<char> for Georgian {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            CAPITAL_LETTER_AN => Ok(Georgian::CapitalLetterAn),
            CAPITAL_LETTER_BAN => Ok(Georgian::CapitalLetterBan),
            CAPITAL_LETTER_GAN => Ok(Georgian::CapitalLetterGan),
            CAPITAL_LETTER_DON => Ok(Georgian::CapitalLetterDon),
            CAPITAL_LETTER_EN => Ok(Georgian::CapitalLetterEn),
            CAPITAL_LETTER_VIN => Ok(Georgian::CapitalLetterVin),
            CAPITAL_LETTER_ZEN => Ok(Georgian::CapitalLetterZen),
            CAPITAL_LETTER_TAN => Ok(Georgian::CapitalLetterTan),
            CAPITAL_LETTER_IN => Ok(Georgian::CapitalLetterIn),
            CAPITAL_LETTER_KAN => Ok(Georgian::CapitalLetterKan),
            CAPITAL_LETTER_LAS => Ok(Georgian::CapitalLetterLas),
            CAPITAL_LETTER_MAN => Ok(Georgian::CapitalLetterMan),
            CAPITAL_LETTER_NAR => Ok(Georgian::CapitalLetterNar),
            CAPITAL_LETTER_ON => Ok(Georgian::CapitalLetterOn),
            CAPITAL_LETTER_PAR => Ok(Georgian::CapitalLetterPar),
            CAPITAL_LETTER_ZHAR => Ok(Georgian::CapitalLetterZhar),
            CAPITAL_LETTER_RAE => Ok(Georgian::CapitalLetterRae),
            CAPITAL_LETTER_SAN => Ok(Georgian::CapitalLetterSan),
            CAPITAL_LETTER_TAR => Ok(Georgian::CapitalLetterTar),
            CAPITAL_LETTER_UN => Ok(Georgian::CapitalLetterUn),
            CAPITAL_LETTER_PHAR => Ok(Georgian::CapitalLetterPhar),
            CAPITAL_LETTER_KHAR => Ok(Georgian::CapitalLetterKhar),
            CAPITAL_LETTER_GHAN => Ok(Georgian::CapitalLetterGhan),
            CAPITAL_LETTER_QAR => Ok(Georgian::CapitalLetterQar),
            CAPITAL_LETTER_SHIN => Ok(Georgian::CapitalLetterShin),
            CAPITAL_LETTER_CHIN => Ok(Georgian::CapitalLetterChin),
            CAPITAL_LETTER_CAN => Ok(Georgian::CapitalLetterCan),
            CAPITAL_LETTER_JIL => Ok(Georgian::CapitalLetterJil),
            CAPITAL_LETTER_CIL => Ok(Georgian::CapitalLetterCil),
            CAPITAL_LETTER_CHAR => Ok(Georgian::CapitalLetterChar),
            CAPITAL_LETTER_XAN => Ok(Georgian::CapitalLetterXan),
            CAPITAL_LETTER_JHAN => Ok(Georgian::CapitalLetterJhan),
            CAPITAL_LETTER_HAE => Ok(Georgian::CapitalLetterHae),
            CAPITAL_LETTER_HE => Ok(Georgian::CapitalLetterHe),
            CAPITAL_LETTER_HIE => Ok(Georgian::CapitalLetterHie),
            CAPITAL_LETTER_WE => Ok(Georgian::CapitalLetterWe),
            CAPITAL_LETTER_HAR => Ok(Georgian::CapitalLetterHar),
            CAPITAL_LETTER_HOE => Ok(Georgian::CapitalLetterHoe),
            CAPITAL_LETTER_YN => Ok(Georgian::CapitalLetterYn),
            CAPITAL_LETTER_AEN => Ok(Georgian::CapitalLetterAen),
            LETTER_AN => Ok(Georgian::LetterAn),
            LETTER_BAN => Ok(Georgian::LetterBan),
            LETTER_GAN => Ok(Georgian::LetterGan),
            LETTER_DON => Ok(Georgian::LetterDon),
            LETTER_EN => Ok(Georgian::LetterEn),
            LETTER_VIN => Ok(Georgian::LetterVin),
            LETTER_ZEN => Ok(Georgian::LetterZen),
            LETTER_TAN => Ok(Georgian::LetterTan),
            LETTER_IN => Ok(Georgian::LetterIn),
            LETTER_KAN => Ok(Georgian::LetterKan),
            LETTER_LAS => Ok(Georgian::LetterLas),
            LETTER_MAN => Ok(Georgian::LetterMan),
            LETTER_NAR => Ok(Georgian::LetterNar),
            LETTER_ON => Ok(Georgian::LetterOn),
            LETTER_PAR => Ok(Georgian::LetterPar),
            LETTER_ZHAR => Ok(Georgian::LetterZhar),
            LETTER_RAE => Ok(Georgian::LetterRae),
            LETTER_SAN => Ok(Georgian::LetterSan),
            LETTER_TAR => Ok(Georgian::LetterTar),
            LETTER_UN => Ok(Georgian::LetterUn),
            LETTER_PHAR => Ok(Georgian::LetterPhar),
            LETTER_KHAR => Ok(Georgian::LetterKhar),
            LETTER_GHAN => Ok(Georgian::LetterGhan),
            LETTER_QAR => Ok(Georgian::LetterQar),
            LETTER_SHIN => Ok(Georgian::LetterShin),
            LETTER_CHIN => Ok(Georgian::LetterChin),
            LETTER_CAN => Ok(Georgian::LetterCan),
            LETTER_JIL => Ok(Georgian::LetterJil),
            LETTER_CIL => Ok(Georgian::LetterCil),
            LETTER_CHAR => Ok(Georgian::LetterChar),
            LETTER_XAN => Ok(Georgian::LetterXan),
            LETTER_JHAN => Ok(Georgian::LetterJhan),
            LETTER_HAE => Ok(Georgian::LetterHae),
            LETTER_HE => Ok(Georgian::LetterHe),
            LETTER_HIE => Ok(Georgian::LetterHie),
            LETTER_WE => Ok(Georgian::LetterWe),
            LETTER_HAR => Ok(Georgian::LetterHar),
            LETTER_HOE => Ok(Georgian::LetterHoe),
            LETTER_FI => Ok(Georgian::LetterFi),
            LETTER_YN => Ok(Georgian::LetterYn),
            LETTER_ELIFI => Ok(Georgian::LetterElifi),
            LETTER_TURNED_GAN => Ok(Georgian::LetterTurnedGan),
            LETTER_AIN => Ok(Georgian::LetterAin),
            PARAGRAPH_SEPARATOR => Ok(Georgian::ParagraphSeparator),
            MODIFIER_LETTER_NAR => Ok(Georgian::ModifierLetterNar),
            LETTER_AEN => Ok(Georgian::LetterAen),
            LETTER_HARD_SIGN => Ok(Georgian::LetterHardSign),
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
