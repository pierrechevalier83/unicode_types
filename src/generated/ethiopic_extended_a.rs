
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
        match self {
            EthiopicExtendedA::EthiopicSyllableTthu => 'ꬁ',
            EthiopicExtendedA::EthiopicSyllableTthi => 'ꬂ',
            EthiopicExtendedA::EthiopicSyllableTthaa => 'ꬃ',
            EthiopicExtendedA::EthiopicSyllableTthee => 'ꬄ',
            EthiopicExtendedA::EthiopicSyllableTthe => 'ꬅ',
            EthiopicExtendedA::EthiopicSyllableTtho => 'ꬆ',
            EthiopicExtendedA::EthiopicSyllableDdhu => 'ꬉ',
            EthiopicExtendedA::EthiopicSyllableDdhi => 'ꬊ',
            EthiopicExtendedA::EthiopicSyllableDdhaa => 'ꬋ',
            EthiopicExtendedA::EthiopicSyllableDdhee => 'ꬌ',
            EthiopicExtendedA::EthiopicSyllableDdhe => 'ꬍ',
            EthiopicExtendedA::EthiopicSyllableDdho => 'ꬎ',
            EthiopicExtendedA::EthiopicSyllableDzu => 'ꬑ',
            EthiopicExtendedA::EthiopicSyllableDzi => 'ꬒ',
            EthiopicExtendedA::EthiopicSyllableDzaa => 'ꬓ',
            EthiopicExtendedA::EthiopicSyllableDzee => 'ꬔ',
            EthiopicExtendedA::EthiopicSyllableDze => 'ꬕ',
            EthiopicExtendedA::EthiopicSyllableDzo => 'ꬖ',
            EthiopicExtendedA::EthiopicSyllableCchha => 'ꬠ',
            EthiopicExtendedA::EthiopicSyllableCchhu => 'ꬡ',
            EthiopicExtendedA::EthiopicSyllableCchhi => 'ꬢ',
            EthiopicExtendedA::EthiopicSyllableCchhaa => 'ꬣ',
            EthiopicExtendedA::EthiopicSyllableCchhee => 'ꬤ',
            EthiopicExtendedA::EthiopicSyllableCchhe => 'ꬥ',
            EthiopicExtendedA::EthiopicSyllableCchho => 'ꬦ',
            EthiopicExtendedA::EthiopicSyllableBba => 'ꬨ',
            EthiopicExtendedA::EthiopicSyllableBbu => 'ꬩ',
            EthiopicExtendedA::EthiopicSyllableBbi => 'ꬪ',
            EthiopicExtendedA::EthiopicSyllableBbaa => 'ꬫ',
            EthiopicExtendedA::EthiopicSyllableBbee => 'ꬬ',
            EthiopicExtendedA::EthiopicSyllableBbe => 'ꬭ',
            EthiopicExtendedA::EthiopicSyllableBbo => 'ꬮ',
        }
    }
}

impl std::convert::TryFrom<char> for EthiopicExtendedA {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ꬁ' => Ok(EthiopicExtendedA::EthiopicSyllableTthu),
            'ꬂ' => Ok(EthiopicExtendedA::EthiopicSyllableTthi),
            'ꬃ' => Ok(EthiopicExtendedA::EthiopicSyllableTthaa),
            'ꬄ' => Ok(EthiopicExtendedA::EthiopicSyllableTthee),
            'ꬅ' => Ok(EthiopicExtendedA::EthiopicSyllableTthe),
            'ꬆ' => Ok(EthiopicExtendedA::EthiopicSyllableTtho),
            'ꬉ' => Ok(EthiopicExtendedA::EthiopicSyllableDdhu),
            'ꬊ' => Ok(EthiopicExtendedA::EthiopicSyllableDdhi),
            'ꬋ' => Ok(EthiopicExtendedA::EthiopicSyllableDdhaa),
            'ꬌ' => Ok(EthiopicExtendedA::EthiopicSyllableDdhee),
            'ꬍ' => Ok(EthiopicExtendedA::EthiopicSyllableDdhe),
            'ꬎ' => Ok(EthiopicExtendedA::EthiopicSyllableDdho),
            'ꬑ' => Ok(EthiopicExtendedA::EthiopicSyllableDzu),
            'ꬒ' => Ok(EthiopicExtendedA::EthiopicSyllableDzi),
            'ꬓ' => Ok(EthiopicExtendedA::EthiopicSyllableDzaa),
            'ꬔ' => Ok(EthiopicExtendedA::EthiopicSyllableDzee),
            'ꬕ' => Ok(EthiopicExtendedA::EthiopicSyllableDze),
            'ꬖ' => Ok(EthiopicExtendedA::EthiopicSyllableDzo),
            'ꬠ' => Ok(EthiopicExtendedA::EthiopicSyllableCchha),
            'ꬡ' => Ok(EthiopicExtendedA::EthiopicSyllableCchhu),
            'ꬢ' => Ok(EthiopicExtendedA::EthiopicSyllableCchhi),
            'ꬣ' => Ok(EthiopicExtendedA::EthiopicSyllableCchhaa),
            'ꬤ' => Ok(EthiopicExtendedA::EthiopicSyllableCchhee),
            'ꬥ' => Ok(EthiopicExtendedA::EthiopicSyllableCchhe),
            'ꬦ' => Ok(EthiopicExtendedA::EthiopicSyllableCchho),
            'ꬨ' => Ok(EthiopicExtendedA::EthiopicSyllableBba),
            'ꬩ' => Ok(EthiopicExtendedA::EthiopicSyllableBbu),
            'ꬪ' => Ok(EthiopicExtendedA::EthiopicSyllableBbi),
            'ꬫ' => Ok(EthiopicExtendedA::EthiopicSyllableBbaa),
            'ꬬ' => Ok(EthiopicExtendedA::EthiopicSyllableBbee),
            'ꬭ' => Ok(EthiopicExtendedA::EthiopicSyllableBbe),
            'ꬮ' => Ok(EthiopicExtendedA::EthiopicSyllableBbo),
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

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("EthiopicExtendedA{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
