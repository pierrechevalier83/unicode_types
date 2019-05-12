/// A number of constants to give a name to all characters in this block.
mod constants {
    /// \u{ab01}: 'ꬁ'
    pub const ETHIOPIC_SYLLABLE_TTHU: char = 'ꬁ';
    /// \u{ab02}: 'ꬂ'
    pub const ETHIOPIC_SYLLABLE_TTHI: char = 'ꬂ';
    /// \u{ab03}: 'ꬃ'
    pub const ETHIOPIC_SYLLABLE_TTHAA: char = 'ꬃ';
    /// \u{ab04}: 'ꬄ'
    pub const ETHIOPIC_SYLLABLE_TTHEE: char = 'ꬄ';
    /// \u{ab05}: 'ꬅ'
    pub const ETHIOPIC_SYLLABLE_TTHE: char = 'ꬅ';
    /// \u{ab06}: 'ꬆ'
    pub const ETHIOPIC_SYLLABLE_TTHO: char = 'ꬆ';
    /// \u{ab09}: 'ꬉ'
    pub const ETHIOPIC_SYLLABLE_DDHU: char = 'ꬉ';
    /// \u{ab0a}: 'ꬊ'
    pub const ETHIOPIC_SYLLABLE_DDHI: char = 'ꬊ';
    /// \u{ab0b}: 'ꬋ'
    pub const ETHIOPIC_SYLLABLE_DDHAA: char = 'ꬋ';
    /// \u{ab0c}: 'ꬌ'
    pub const ETHIOPIC_SYLLABLE_DDHEE: char = 'ꬌ';
    /// \u{ab0d}: 'ꬍ'
    pub const ETHIOPIC_SYLLABLE_DDHE: char = 'ꬍ';
    /// \u{ab0e}: 'ꬎ'
    pub const ETHIOPIC_SYLLABLE_DDHO: char = 'ꬎ';
    /// \u{ab11}: 'ꬑ'
    pub const ETHIOPIC_SYLLABLE_DZU: char = 'ꬑ';
    /// \u{ab12}: 'ꬒ'
    pub const ETHIOPIC_SYLLABLE_DZI: char = 'ꬒ';
    /// \u{ab13}: 'ꬓ'
    pub const ETHIOPIC_SYLLABLE_DZAA: char = 'ꬓ';
    /// \u{ab14}: 'ꬔ'
    pub const ETHIOPIC_SYLLABLE_DZEE: char = 'ꬔ';
    /// \u{ab15}: 'ꬕ'
    pub const ETHIOPIC_SYLLABLE_DZE: char = 'ꬕ';
    /// \u{ab16}: 'ꬖ'
    pub const ETHIOPIC_SYLLABLE_DZO: char = 'ꬖ';
    /// \u{ab20}: 'ꬠ'
    pub const ETHIOPIC_SYLLABLE_CCHHA: char = 'ꬠ';
    /// \u{ab21}: 'ꬡ'
    pub const ETHIOPIC_SYLLABLE_CCHHU: char = 'ꬡ';
    /// \u{ab22}: 'ꬢ'
    pub const ETHIOPIC_SYLLABLE_CCHHI: char = 'ꬢ';
    /// \u{ab23}: 'ꬣ'
    pub const ETHIOPIC_SYLLABLE_CCHHAA: char = 'ꬣ';
    /// \u{ab24}: 'ꬤ'
    pub const ETHIOPIC_SYLLABLE_CCHHEE: char = 'ꬤ';
    /// \u{ab25}: 'ꬥ'
    pub const ETHIOPIC_SYLLABLE_CCHHE: char = 'ꬥ';
    /// \u{ab26}: 'ꬦ'
    pub const ETHIOPIC_SYLLABLE_CCHHO: char = 'ꬦ';
    /// \u{ab28}: 'ꬨ'
    pub const ETHIOPIC_SYLLABLE_BBA: char = 'ꬨ';
    /// \u{ab29}: 'ꬩ'
    pub const ETHIOPIC_SYLLABLE_BBU: char = 'ꬩ';
    /// \u{ab2a}: 'ꬪ'
    pub const ETHIOPIC_SYLLABLE_BBI: char = 'ꬪ';
    /// \u{ab2b}: 'ꬫ'
    pub const ETHIOPIC_SYLLABLE_BBAA: char = 'ꬫ';
    /// \u{ab2c}: 'ꬬ'
    pub const ETHIOPIC_SYLLABLE_BBEE: char = 'ꬬ';
    /// \u{ab2d}: 'ꬭ'
    pub const ETHIOPIC_SYLLABLE_BBE: char = 'ꬭ';
    /// \u{ab2e}: 'ꬮ'
    pub const ETHIOPIC_SYLLABLE_BBO: char = 'ꬮ';
}

/// An enum to represent all characters in the EthiopicExtendedA block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum EthiopicExtendedA {
    /// \u{ab01}: 'ꬁ'
    EthiopicSyllableTthu,
    /// \u{ab02}: 'ꬂ'
    EthiopicSyllableTthi,
    /// \u{ab03}: 'ꬃ'
    EthiopicSyllableTthaa,
    /// \u{ab04}: 'ꬄ'
    EthiopicSyllableTthee,
    /// \u{ab05}: 'ꬅ'
    EthiopicSyllableTthe,
    /// \u{ab06}: 'ꬆ'
    EthiopicSyllableTtho,
    /// \u{ab09}: 'ꬉ'
    EthiopicSyllableDdhu,
    /// \u{ab0a}: 'ꬊ'
    EthiopicSyllableDdhi,
    /// \u{ab0b}: 'ꬋ'
    EthiopicSyllableDdhaa,
    /// \u{ab0c}: 'ꬌ'
    EthiopicSyllableDdhee,
    /// \u{ab0d}: 'ꬍ'
    EthiopicSyllableDdhe,
    /// \u{ab0e}: 'ꬎ'
    EthiopicSyllableDdho,
    /// \u{ab11}: 'ꬑ'
    EthiopicSyllableDzu,
    /// \u{ab12}: 'ꬒ'
    EthiopicSyllableDzi,
    /// \u{ab13}: 'ꬓ'
    EthiopicSyllableDzaa,
    /// \u{ab14}: 'ꬔ'
    EthiopicSyllableDzee,
    /// \u{ab15}: 'ꬕ'
    EthiopicSyllableDze,
    /// \u{ab16}: 'ꬖ'
    EthiopicSyllableDzo,
    /// \u{ab20}: 'ꬠ'
    EthiopicSyllableCchha,
    /// \u{ab21}: 'ꬡ'
    EthiopicSyllableCchhu,
    /// \u{ab22}: 'ꬢ'
    EthiopicSyllableCchhi,
    /// \u{ab23}: 'ꬣ'
    EthiopicSyllableCchhaa,
    /// \u{ab24}: 'ꬤ'
    EthiopicSyllableCchhee,
    /// \u{ab25}: 'ꬥ'
    EthiopicSyllableCchhe,
    /// \u{ab26}: 'ꬦ'
    EthiopicSyllableCchho,
    /// \u{ab28}: 'ꬨ'
    EthiopicSyllableBba,
    /// \u{ab29}: 'ꬩ'
    EthiopicSyllableBbu,
    /// \u{ab2a}: 'ꬪ'
    EthiopicSyllableBbi,
    /// \u{ab2b}: 'ꬫ'
    EthiopicSyllableBbaa,
    /// \u{ab2c}: 'ꬬ'
    EthiopicSyllableBbee,
    /// \u{ab2d}: 'ꬭ'
    EthiopicSyllableBbe,
    /// \u{ab2e}: 'ꬮ'
    EthiopicSyllableBbo,
}

impl Into<char> for EthiopicExtendedA {
    fn into(self) -> char {
        use constants::*;
        match self {
            EthiopicExtendedA::EthiopicSyllableTthu => ETHIOPIC_SYLLABLE_TTHU,
            EthiopicExtendedA::EthiopicSyllableTthi => ETHIOPIC_SYLLABLE_TTHI,
            EthiopicExtendedA::EthiopicSyllableTthaa => ETHIOPIC_SYLLABLE_TTHAA,
            EthiopicExtendedA::EthiopicSyllableTthee => ETHIOPIC_SYLLABLE_TTHEE,
            EthiopicExtendedA::EthiopicSyllableTthe => ETHIOPIC_SYLLABLE_TTHE,
            EthiopicExtendedA::EthiopicSyllableTtho => ETHIOPIC_SYLLABLE_TTHO,
            EthiopicExtendedA::EthiopicSyllableDdhu => ETHIOPIC_SYLLABLE_DDHU,
            EthiopicExtendedA::EthiopicSyllableDdhi => ETHIOPIC_SYLLABLE_DDHI,
            EthiopicExtendedA::EthiopicSyllableDdhaa => ETHIOPIC_SYLLABLE_DDHAA,
            EthiopicExtendedA::EthiopicSyllableDdhee => ETHIOPIC_SYLLABLE_DDHEE,
            EthiopicExtendedA::EthiopicSyllableDdhe => ETHIOPIC_SYLLABLE_DDHE,
            EthiopicExtendedA::EthiopicSyllableDdho => ETHIOPIC_SYLLABLE_DDHO,
            EthiopicExtendedA::EthiopicSyllableDzu => ETHIOPIC_SYLLABLE_DZU,
            EthiopicExtendedA::EthiopicSyllableDzi => ETHIOPIC_SYLLABLE_DZI,
            EthiopicExtendedA::EthiopicSyllableDzaa => ETHIOPIC_SYLLABLE_DZAA,
            EthiopicExtendedA::EthiopicSyllableDzee => ETHIOPIC_SYLLABLE_DZEE,
            EthiopicExtendedA::EthiopicSyllableDze => ETHIOPIC_SYLLABLE_DZE,
            EthiopicExtendedA::EthiopicSyllableDzo => ETHIOPIC_SYLLABLE_DZO,
            EthiopicExtendedA::EthiopicSyllableCchha => ETHIOPIC_SYLLABLE_CCHHA,
            EthiopicExtendedA::EthiopicSyllableCchhu => ETHIOPIC_SYLLABLE_CCHHU,
            EthiopicExtendedA::EthiopicSyllableCchhi => ETHIOPIC_SYLLABLE_CCHHI,
            EthiopicExtendedA::EthiopicSyllableCchhaa => ETHIOPIC_SYLLABLE_CCHHAA,
            EthiopicExtendedA::EthiopicSyllableCchhee => ETHIOPIC_SYLLABLE_CCHHEE,
            EthiopicExtendedA::EthiopicSyllableCchhe => ETHIOPIC_SYLLABLE_CCHHE,
            EthiopicExtendedA::EthiopicSyllableCchho => ETHIOPIC_SYLLABLE_CCHHO,
            EthiopicExtendedA::EthiopicSyllableBba => ETHIOPIC_SYLLABLE_BBA,
            EthiopicExtendedA::EthiopicSyllableBbu => ETHIOPIC_SYLLABLE_BBU,
            EthiopicExtendedA::EthiopicSyllableBbi => ETHIOPIC_SYLLABLE_BBI,
            EthiopicExtendedA::EthiopicSyllableBbaa => ETHIOPIC_SYLLABLE_BBAA,
            EthiopicExtendedA::EthiopicSyllableBbee => ETHIOPIC_SYLLABLE_BBEE,
            EthiopicExtendedA::EthiopicSyllableBbe => ETHIOPIC_SYLLABLE_BBE,
            EthiopicExtendedA::EthiopicSyllableBbo => ETHIOPIC_SYLLABLE_BBO,
        }
    }
}

impl std::convert::TryFrom<char> for EthiopicExtendedA {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            ETHIOPIC_SYLLABLE_TTHU => Ok(EthiopicExtendedA::EthiopicSyllableTthu),
            ETHIOPIC_SYLLABLE_TTHI => Ok(EthiopicExtendedA::EthiopicSyllableTthi),
            ETHIOPIC_SYLLABLE_TTHAA => Ok(EthiopicExtendedA::EthiopicSyllableTthaa),
            ETHIOPIC_SYLLABLE_TTHEE => Ok(EthiopicExtendedA::EthiopicSyllableTthee),
            ETHIOPIC_SYLLABLE_TTHE => Ok(EthiopicExtendedA::EthiopicSyllableTthe),
            ETHIOPIC_SYLLABLE_TTHO => Ok(EthiopicExtendedA::EthiopicSyllableTtho),
            ETHIOPIC_SYLLABLE_DDHU => Ok(EthiopicExtendedA::EthiopicSyllableDdhu),
            ETHIOPIC_SYLLABLE_DDHI => Ok(EthiopicExtendedA::EthiopicSyllableDdhi),
            ETHIOPIC_SYLLABLE_DDHAA => Ok(EthiopicExtendedA::EthiopicSyllableDdhaa),
            ETHIOPIC_SYLLABLE_DDHEE => Ok(EthiopicExtendedA::EthiopicSyllableDdhee),
            ETHIOPIC_SYLLABLE_DDHE => Ok(EthiopicExtendedA::EthiopicSyllableDdhe),
            ETHIOPIC_SYLLABLE_DDHO => Ok(EthiopicExtendedA::EthiopicSyllableDdho),
            ETHIOPIC_SYLLABLE_DZU => Ok(EthiopicExtendedA::EthiopicSyllableDzu),
            ETHIOPIC_SYLLABLE_DZI => Ok(EthiopicExtendedA::EthiopicSyllableDzi),
            ETHIOPIC_SYLLABLE_DZAA => Ok(EthiopicExtendedA::EthiopicSyllableDzaa),
            ETHIOPIC_SYLLABLE_DZEE => Ok(EthiopicExtendedA::EthiopicSyllableDzee),
            ETHIOPIC_SYLLABLE_DZE => Ok(EthiopicExtendedA::EthiopicSyllableDze),
            ETHIOPIC_SYLLABLE_DZO => Ok(EthiopicExtendedA::EthiopicSyllableDzo),
            ETHIOPIC_SYLLABLE_CCHHA => Ok(EthiopicExtendedA::EthiopicSyllableCchha),
            ETHIOPIC_SYLLABLE_CCHHU => Ok(EthiopicExtendedA::EthiopicSyllableCchhu),
            ETHIOPIC_SYLLABLE_CCHHI => Ok(EthiopicExtendedA::EthiopicSyllableCchhi),
            ETHIOPIC_SYLLABLE_CCHHAA => Ok(EthiopicExtendedA::EthiopicSyllableCchhaa),
            ETHIOPIC_SYLLABLE_CCHHEE => Ok(EthiopicExtendedA::EthiopicSyllableCchhee),
            ETHIOPIC_SYLLABLE_CCHHE => Ok(EthiopicExtendedA::EthiopicSyllableCchhe),
            ETHIOPIC_SYLLABLE_CCHHO => Ok(EthiopicExtendedA::EthiopicSyllableCchho),
            ETHIOPIC_SYLLABLE_BBA => Ok(EthiopicExtendedA::EthiopicSyllableBba),
            ETHIOPIC_SYLLABLE_BBU => Ok(EthiopicExtendedA::EthiopicSyllableBbu),
            ETHIOPIC_SYLLABLE_BBI => Ok(EthiopicExtendedA::EthiopicSyllableBbi),
            ETHIOPIC_SYLLABLE_BBAA => Ok(EthiopicExtendedA::EthiopicSyllableBbaa),
            ETHIOPIC_SYLLABLE_BBEE => Ok(EthiopicExtendedA::EthiopicSyllableBbee),
            ETHIOPIC_SYLLABLE_BBE => Ok(EthiopicExtendedA::EthiopicSyllableBbe),
            ETHIOPIC_SYLLABLE_BBO => Ok(EthiopicExtendedA::EthiopicSyllableBbo),
            _ => Err(()),
        }
    }
}

impl Into<u32> for EthiopicExtendedA {
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

impl std::convert::TryFrom<u32> for EthiopicExtendedA {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for EthiopicExtendedA {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl EthiopicExtendedA {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        EthiopicExtendedA::EthiopicSyllableTthu
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            EthiopicExtendedA::EthiopicSyllableTthu => "ethiopic syllable tthu",
            EthiopicExtendedA::EthiopicSyllableTthi => "ethiopic syllable tthi",
            EthiopicExtendedA::EthiopicSyllableTthaa => "ethiopic syllable tthaa",
            EthiopicExtendedA::EthiopicSyllableTthee => "ethiopic syllable tthee",
            EthiopicExtendedA::EthiopicSyllableTthe => "ethiopic syllable tthe",
            EthiopicExtendedA::EthiopicSyllableTtho => "ethiopic syllable ttho",
            EthiopicExtendedA::EthiopicSyllableDdhu => "ethiopic syllable ddhu",
            EthiopicExtendedA::EthiopicSyllableDdhi => "ethiopic syllable ddhi",
            EthiopicExtendedA::EthiopicSyllableDdhaa => "ethiopic syllable ddhaa",
            EthiopicExtendedA::EthiopicSyllableDdhee => "ethiopic syllable ddhee",
            EthiopicExtendedA::EthiopicSyllableDdhe => "ethiopic syllable ddhe",
            EthiopicExtendedA::EthiopicSyllableDdho => "ethiopic syllable ddho",
            EthiopicExtendedA::EthiopicSyllableDzu => "ethiopic syllable dzu",
            EthiopicExtendedA::EthiopicSyllableDzi => "ethiopic syllable dzi",
            EthiopicExtendedA::EthiopicSyllableDzaa => "ethiopic syllable dzaa",
            EthiopicExtendedA::EthiopicSyllableDzee => "ethiopic syllable dzee",
            EthiopicExtendedA::EthiopicSyllableDze => "ethiopic syllable dze",
            EthiopicExtendedA::EthiopicSyllableDzo => "ethiopic syllable dzo",
            EthiopicExtendedA::EthiopicSyllableCchha => "ethiopic syllable cchha",
            EthiopicExtendedA::EthiopicSyllableCchhu => "ethiopic syllable cchhu",
            EthiopicExtendedA::EthiopicSyllableCchhi => "ethiopic syllable cchhi",
            EthiopicExtendedA::EthiopicSyllableCchhaa => "ethiopic syllable cchhaa",
            EthiopicExtendedA::EthiopicSyllableCchhee => "ethiopic syllable cchhee",
            EthiopicExtendedA::EthiopicSyllableCchhe => "ethiopic syllable cchhe",
            EthiopicExtendedA::EthiopicSyllableCchho => "ethiopic syllable cchho",
            EthiopicExtendedA::EthiopicSyllableBba => "ethiopic syllable bba",
            EthiopicExtendedA::EthiopicSyllableBbu => "ethiopic syllable bbu",
            EthiopicExtendedA::EthiopicSyllableBbi => "ethiopic syllable bbi",
            EthiopicExtendedA::EthiopicSyllableBbaa => "ethiopic syllable bbaa",
            EthiopicExtendedA::EthiopicSyllableBbee => "ethiopic syllable bbee",
            EthiopicExtendedA::EthiopicSyllableBbe => "ethiopic syllable bbe",
            EthiopicExtendedA::EthiopicSyllableBbo => "ethiopic syllable bbo",
        }
    }
}
