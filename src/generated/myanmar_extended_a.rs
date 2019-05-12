
/// An enum to represent all characters in the MyanmarExtendedA block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum MyanmarExtendedA {
    /// \u{aa60}: 'ꩠ'
    MyanmarLetterKhamtiGa,
    /// \u{aa61}: 'ꩡ'
    MyanmarLetterKhamtiCa,
    /// \u{aa62}: 'ꩢ'
    MyanmarLetterKhamtiCha,
    /// \u{aa63}: 'ꩣ'
    MyanmarLetterKhamtiJa,
    /// \u{aa64}: 'ꩤ'
    MyanmarLetterKhamtiJha,
    /// \u{aa65}: 'ꩥ'
    MyanmarLetterKhamtiNya,
    /// \u{aa66}: 'ꩦ'
    MyanmarLetterKhamtiTta,
    /// \u{aa67}: 'ꩧ'
    MyanmarLetterKhamtiTtha,
    /// \u{aa68}: 'ꩨ'
    MyanmarLetterKhamtiDda,
    /// \u{aa69}: 'ꩩ'
    MyanmarLetterKhamtiDdha,
    /// \u{aa6a}: 'ꩪ'
    MyanmarLetterKhamtiDha,
    /// \u{aa6b}: 'ꩫ'
    MyanmarLetterKhamtiNa,
    /// \u{aa6c}: 'ꩬ'
    MyanmarLetterKhamtiSa,
    /// \u{aa6d}: 'ꩭ'
    MyanmarLetterKhamtiHa,
    /// \u{aa6e}: 'ꩮ'
    MyanmarLetterKhamtiHha,
    /// \u{aa6f}: 'ꩯ'
    MyanmarLetterKhamtiFa,
    /// \u{aa70}: 'ꩰ'
    MyanmarModifierLetterKhamtiReduplication,
    /// \u{aa71}: 'ꩱ'
    MyanmarLetterKhamtiXa,
    /// \u{aa72}: 'ꩲ'
    MyanmarLetterKhamtiZa,
    /// \u{aa73}: 'ꩳ'
    MyanmarLetterKhamtiRa,
    /// \u{aa74}: 'ꩴ'
    MyanmarLogogramKhamtiOay,
    /// \u{aa75}: 'ꩵ'
    MyanmarLogogramKhamtiQn,
    /// \u{aa76}: 'ꩶ'
    MyanmarLogogramKhamtiHm,
    /// \u{aa77}: '꩷'
    MyanmarSymbolAitonExclamation,
    /// \u{aa78}: '꩸'
    MyanmarSymbolAitonOne,
    /// \u{aa79}: '꩹'
    MyanmarSymbolAitonTwo,
    /// \u{aa7a}: 'ꩺ'
    MyanmarLetterAitonRa,
    /// \u{aa7b}: 'ꩻ'
    MyanmarSignPaoKarenTone,
    /// \u{aa7c}: 'ꩼ'
    MyanmarSignTaiLaingToneDash2,
    /// \u{aa7d}: 'ꩽ'
    MyanmarSignTaiLaingToneDash5,
    /// \u{aa7e}: 'ꩾ'
    MyanmarLetterShwePalaungCha,
}

impl Into<char> for MyanmarExtendedA {
    fn into(self) -> char {
        match self {
            MyanmarExtendedA::MyanmarLetterKhamtiGa => 'ꩠ',
            MyanmarExtendedA::MyanmarLetterKhamtiCa => 'ꩡ',
            MyanmarExtendedA::MyanmarLetterKhamtiCha => 'ꩢ',
            MyanmarExtendedA::MyanmarLetterKhamtiJa => 'ꩣ',
            MyanmarExtendedA::MyanmarLetterKhamtiJha => 'ꩤ',
            MyanmarExtendedA::MyanmarLetterKhamtiNya => 'ꩥ',
            MyanmarExtendedA::MyanmarLetterKhamtiTta => 'ꩦ',
            MyanmarExtendedA::MyanmarLetterKhamtiTtha => 'ꩧ',
            MyanmarExtendedA::MyanmarLetterKhamtiDda => 'ꩨ',
            MyanmarExtendedA::MyanmarLetterKhamtiDdha => 'ꩩ',
            MyanmarExtendedA::MyanmarLetterKhamtiDha => 'ꩪ',
            MyanmarExtendedA::MyanmarLetterKhamtiNa => 'ꩫ',
            MyanmarExtendedA::MyanmarLetterKhamtiSa => 'ꩬ',
            MyanmarExtendedA::MyanmarLetterKhamtiHa => 'ꩭ',
            MyanmarExtendedA::MyanmarLetterKhamtiHha => 'ꩮ',
            MyanmarExtendedA::MyanmarLetterKhamtiFa => 'ꩯ',
            MyanmarExtendedA::MyanmarModifierLetterKhamtiReduplication => 'ꩰ',
            MyanmarExtendedA::MyanmarLetterKhamtiXa => 'ꩱ',
            MyanmarExtendedA::MyanmarLetterKhamtiZa => 'ꩲ',
            MyanmarExtendedA::MyanmarLetterKhamtiRa => 'ꩳ',
            MyanmarExtendedA::MyanmarLogogramKhamtiOay => 'ꩴ',
            MyanmarExtendedA::MyanmarLogogramKhamtiQn => 'ꩵ',
            MyanmarExtendedA::MyanmarLogogramKhamtiHm => 'ꩶ',
            MyanmarExtendedA::MyanmarSymbolAitonExclamation => '꩷',
            MyanmarExtendedA::MyanmarSymbolAitonOne => '꩸',
            MyanmarExtendedA::MyanmarSymbolAitonTwo => '꩹',
            MyanmarExtendedA::MyanmarLetterAitonRa => 'ꩺ',
            MyanmarExtendedA::MyanmarSignPaoKarenTone => 'ꩻ',
            MyanmarExtendedA::MyanmarSignTaiLaingToneDash2 => 'ꩼ',
            MyanmarExtendedA::MyanmarSignTaiLaingToneDash5 => 'ꩽ',
            MyanmarExtendedA::MyanmarLetterShwePalaungCha => 'ꩾ',
        }
    }
}

impl std::convert::TryFrom<char> for MyanmarExtendedA {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ꩠ' => Ok(MyanmarExtendedA::MyanmarLetterKhamtiGa),
            'ꩡ' => Ok(MyanmarExtendedA::MyanmarLetterKhamtiCa),
            'ꩢ' => Ok(MyanmarExtendedA::MyanmarLetterKhamtiCha),
            'ꩣ' => Ok(MyanmarExtendedA::MyanmarLetterKhamtiJa),
            'ꩤ' => Ok(MyanmarExtendedA::MyanmarLetterKhamtiJha),
            'ꩥ' => Ok(MyanmarExtendedA::MyanmarLetterKhamtiNya),
            'ꩦ' => Ok(MyanmarExtendedA::MyanmarLetterKhamtiTta),
            'ꩧ' => Ok(MyanmarExtendedA::MyanmarLetterKhamtiTtha),
            'ꩨ' => Ok(MyanmarExtendedA::MyanmarLetterKhamtiDda),
            'ꩩ' => Ok(MyanmarExtendedA::MyanmarLetterKhamtiDdha),
            'ꩪ' => Ok(MyanmarExtendedA::MyanmarLetterKhamtiDha),
            'ꩫ' => Ok(MyanmarExtendedA::MyanmarLetterKhamtiNa),
            'ꩬ' => Ok(MyanmarExtendedA::MyanmarLetterKhamtiSa),
            'ꩭ' => Ok(MyanmarExtendedA::MyanmarLetterKhamtiHa),
            'ꩮ' => Ok(MyanmarExtendedA::MyanmarLetterKhamtiHha),
            'ꩯ' => Ok(MyanmarExtendedA::MyanmarLetterKhamtiFa),
            'ꩰ' => Ok(MyanmarExtendedA::MyanmarModifierLetterKhamtiReduplication),
            'ꩱ' => Ok(MyanmarExtendedA::MyanmarLetterKhamtiXa),
            'ꩲ' => Ok(MyanmarExtendedA::MyanmarLetterKhamtiZa),
            'ꩳ' => Ok(MyanmarExtendedA::MyanmarLetterKhamtiRa),
            'ꩴ' => Ok(MyanmarExtendedA::MyanmarLogogramKhamtiOay),
            'ꩵ' => Ok(MyanmarExtendedA::MyanmarLogogramKhamtiQn),
            'ꩶ' => Ok(MyanmarExtendedA::MyanmarLogogramKhamtiHm),
            '꩷' => Ok(MyanmarExtendedA::MyanmarSymbolAitonExclamation),
            '꩸' => Ok(MyanmarExtendedA::MyanmarSymbolAitonOne),
            '꩹' => Ok(MyanmarExtendedA::MyanmarSymbolAitonTwo),
            'ꩺ' => Ok(MyanmarExtendedA::MyanmarLetterAitonRa),
            'ꩻ' => Ok(MyanmarExtendedA::MyanmarSignPaoKarenTone),
            'ꩼ' => Ok(MyanmarExtendedA::MyanmarSignTaiLaingToneDash2),
            'ꩽ' => Ok(MyanmarExtendedA::MyanmarSignTaiLaingToneDash5),
            'ꩾ' => Ok(MyanmarExtendedA::MyanmarLetterShwePalaungCha),
            _ => Err(()),
        }
    }
}

impl Into<u32> for MyanmarExtendedA {
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

impl std::convert::TryFrom<u32> for MyanmarExtendedA {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for MyanmarExtendedA {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl MyanmarExtendedA {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        MyanmarExtendedA::MyanmarLetterKhamtiGa
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("MyanmarExtendedA{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
