/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{aa60}: 'ꩠ'
    pub const MYANMAR_LETTER_KHAMTI_GA: char = 'ꩠ';
    /// \u{aa61}: 'ꩡ'
    pub const MYANMAR_LETTER_KHAMTI_CA: char = 'ꩡ';
    /// \u{aa62}: 'ꩢ'
    pub const MYANMAR_LETTER_KHAMTI_CHA: char = 'ꩢ';
    /// \u{aa63}: 'ꩣ'
    pub const MYANMAR_LETTER_KHAMTI_JA: char = 'ꩣ';
    /// \u{aa64}: 'ꩤ'
    pub const MYANMAR_LETTER_KHAMTI_JHA: char = 'ꩤ';
    /// \u{aa65}: 'ꩥ'
    pub const MYANMAR_LETTER_KHAMTI_NYA: char = 'ꩥ';
    /// \u{aa66}: 'ꩦ'
    pub const MYANMAR_LETTER_KHAMTI_TTA: char = 'ꩦ';
    /// \u{aa67}: 'ꩧ'
    pub const MYANMAR_LETTER_KHAMTI_TTHA: char = 'ꩧ';
    /// \u{aa68}: 'ꩨ'
    pub const MYANMAR_LETTER_KHAMTI_DDA: char = 'ꩨ';
    /// \u{aa69}: 'ꩩ'
    pub const MYANMAR_LETTER_KHAMTI_DDHA: char = 'ꩩ';
    /// \u{aa6a}: 'ꩪ'
    pub const MYANMAR_LETTER_KHAMTI_DHA: char = 'ꩪ';
    /// \u{aa6b}: 'ꩫ'
    pub const MYANMAR_LETTER_KHAMTI_NA: char = 'ꩫ';
    /// \u{aa6c}: 'ꩬ'
    pub const MYANMAR_LETTER_KHAMTI_SA: char = 'ꩬ';
    /// \u{aa6d}: 'ꩭ'
    pub const MYANMAR_LETTER_KHAMTI_HA: char = 'ꩭ';
    /// \u{aa6e}: 'ꩮ'
    pub const MYANMAR_LETTER_KHAMTI_HHA: char = 'ꩮ';
    /// \u{aa6f}: 'ꩯ'
    pub const MYANMAR_LETTER_KHAMTI_FA: char = 'ꩯ';
    /// \u{aa70}: 'ꩰ'
    pub const MYANMAR_MODIFIER_LETTER_KHAMTI_REDUPLICATION: char = 'ꩰ';
    /// \u{aa71}: 'ꩱ'
    pub const MYANMAR_LETTER_KHAMTI_XA: char = 'ꩱ';
    /// \u{aa72}: 'ꩲ'
    pub const MYANMAR_LETTER_KHAMTI_ZA: char = 'ꩲ';
    /// \u{aa73}: 'ꩳ'
    pub const MYANMAR_LETTER_KHAMTI_RA: char = 'ꩳ';
    /// \u{aa74}: 'ꩴ'
    pub const MYANMAR_LOGOGRAM_KHAMTI_OAY: char = 'ꩴ';
    /// \u{aa75}: 'ꩵ'
    pub const MYANMAR_LOGOGRAM_KHAMTI_QN: char = 'ꩵ';
    /// \u{aa76}: 'ꩶ'
    pub const MYANMAR_LOGOGRAM_KHAMTI_HM: char = 'ꩶ';
    /// \u{aa77}: '꩷'
    pub const MYANMAR_SYMBOL_AITON_EXCLAMATION: char = '꩷';
    /// \u{aa78}: '꩸'
    pub const MYANMAR_SYMBOL_AITON_ONE: char = '꩸';
    /// \u{aa79}: '꩹'
    pub const MYANMAR_SYMBOL_AITON_TWO: char = '꩹';
    /// \u{aa7a}: 'ꩺ'
    pub const MYANMAR_LETTER_AITON_RA: char = 'ꩺ';
    /// \u{aa7b}: 'ꩻ'
    pub const MYANMAR_SIGN_PAO_KAREN_TONE: char = 'ꩻ';
    /// \u{aa7c}: 'ꩼ'
    pub const MYANMAR_SIGN_TAI_LAING_TONE_DASH_2: char = 'ꩼ';
    /// \u{aa7d}: 'ꩽ'
    pub const MYANMAR_SIGN_TAI_LAING_TONE_DASH_5: char = 'ꩽ';
    /// \u{aa7e}: 'ꩾ'
    pub const MYANMAR_LETTER_SHWE_PALAUNG_CHA: char = 'ꩾ';
}

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
        use constants::*;
        match self {
            MyanmarExtendedA::MyanmarLetterKhamtiGa => MYANMAR_LETTER_KHAMTI_GA,
            MyanmarExtendedA::MyanmarLetterKhamtiCa => MYANMAR_LETTER_KHAMTI_CA,
            MyanmarExtendedA::MyanmarLetterKhamtiCha => MYANMAR_LETTER_KHAMTI_CHA,
            MyanmarExtendedA::MyanmarLetterKhamtiJa => MYANMAR_LETTER_KHAMTI_JA,
            MyanmarExtendedA::MyanmarLetterKhamtiJha => MYANMAR_LETTER_KHAMTI_JHA,
            MyanmarExtendedA::MyanmarLetterKhamtiNya => MYANMAR_LETTER_KHAMTI_NYA,
            MyanmarExtendedA::MyanmarLetterKhamtiTta => MYANMAR_LETTER_KHAMTI_TTA,
            MyanmarExtendedA::MyanmarLetterKhamtiTtha => MYANMAR_LETTER_KHAMTI_TTHA,
            MyanmarExtendedA::MyanmarLetterKhamtiDda => MYANMAR_LETTER_KHAMTI_DDA,
            MyanmarExtendedA::MyanmarLetterKhamtiDdha => MYANMAR_LETTER_KHAMTI_DDHA,
            MyanmarExtendedA::MyanmarLetterKhamtiDha => MYANMAR_LETTER_KHAMTI_DHA,
            MyanmarExtendedA::MyanmarLetterKhamtiNa => MYANMAR_LETTER_KHAMTI_NA,
            MyanmarExtendedA::MyanmarLetterKhamtiSa => MYANMAR_LETTER_KHAMTI_SA,
            MyanmarExtendedA::MyanmarLetterKhamtiHa => MYANMAR_LETTER_KHAMTI_HA,
            MyanmarExtendedA::MyanmarLetterKhamtiHha => MYANMAR_LETTER_KHAMTI_HHA,
            MyanmarExtendedA::MyanmarLetterKhamtiFa => MYANMAR_LETTER_KHAMTI_FA,
            MyanmarExtendedA::MyanmarModifierLetterKhamtiReduplication => MYANMAR_MODIFIER_LETTER_KHAMTI_REDUPLICATION,
            MyanmarExtendedA::MyanmarLetterKhamtiXa => MYANMAR_LETTER_KHAMTI_XA,
            MyanmarExtendedA::MyanmarLetterKhamtiZa => MYANMAR_LETTER_KHAMTI_ZA,
            MyanmarExtendedA::MyanmarLetterKhamtiRa => MYANMAR_LETTER_KHAMTI_RA,
            MyanmarExtendedA::MyanmarLogogramKhamtiOay => MYANMAR_LOGOGRAM_KHAMTI_OAY,
            MyanmarExtendedA::MyanmarLogogramKhamtiQn => MYANMAR_LOGOGRAM_KHAMTI_QN,
            MyanmarExtendedA::MyanmarLogogramKhamtiHm => MYANMAR_LOGOGRAM_KHAMTI_HM,
            MyanmarExtendedA::MyanmarSymbolAitonExclamation => MYANMAR_SYMBOL_AITON_EXCLAMATION,
            MyanmarExtendedA::MyanmarSymbolAitonOne => MYANMAR_SYMBOL_AITON_ONE,
            MyanmarExtendedA::MyanmarSymbolAitonTwo => MYANMAR_SYMBOL_AITON_TWO,
            MyanmarExtendedA::MyanmarLetterAitonRa => MYANMAR_LETTER_AITON_RA,
            MyanmarExtendedA::MyanmarSignPaoKarenTone => MYANMAR_SIGN_PAO_KAREN_TONE,
            MyanmarExtendedA::MyanmarSignTaiLaingToneDash2 => MYANMAR_SIGN_TAI_LAING_TONE_DASH_2,
            MyanmarExtendedA::MyanmarSignTaiLaingToneDash5 => MYANMAR_SIGN_TAI_LAING_TONE_DASH_5,
            MyanmarExtendedA::MyanmarLetterShwePalaungCha => MYANMAR_LETTER_SHWE_PALAUNG_CHA,
        }
    }
}

impl std::convert::TryFrom<char> for MyanmarExtendedA {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            MYANMAR_LETTER_KHAMTI_GA => Ok(MyanmarExtendedA::MyanmarLetterKhamtiGa),
            MYANMAR_LETTER_KHAMTI_CA => Ok(MyanmarExtendedA::MyanmarLetterKhamtiCa),
            MYANMAR_LETTER_KHAMTI_CHA => Ok(MyanmarExtendedA::MyanmarLetterKhamtiCha),
            MYANMAR_LETTER_KHAMTI_JA => Ok(MyanmarExtendedA::MyanmarLetterKhamtiJa),
            MYANMAR_LETTER_KHAMTI_JHA => Ok(MyanmarExtendedA::MyanmarLetterKhamtiJha),
            MYANMAR_LETTER_KHAMTI_NYA => Ok(MyanmarExtendedA::MyanmarLetterKhamtiNya),
            MYANMAR_LETTER_KHAMTI_TTA => Ok(MyanmarExtendedA::MyanmarLetterKhamtiTta),
            MYANMAR_LETTER_KHAMTI_TTHA => Ok(MyanmarExtendedA::MyanmarLetterKhamtiTtha),
            MYANMAR_LETTER_KHAMTI_DDA => Ok(MyanmarExtendedA::MyanmarLetterKhamtiDda),
            MYANMAR_LETTER_KHAMTI_DDHA => Ok(MyanmarExtendedA::MyanmarLetterKhamtiDdha),
            MYANMAR_LETTER_KHAMTI_DHA => Ok(MyanmarExtendedA::MyanmarLetterKhamtiDha),
            MYANMAR_LETTER_KHAMTI_NA => Ok(MyanmarExtendedA::MyanmarLetterKhamtiNa),
            MYANMAR_LETTER_KHAMTI_SA => Ok(MyanmarExtendedA::MyanmarLetterKhamtiSa),
            MYANMAR_LETTER_KHAMTI_HA => Ok(MyanmarExtendedA::MyanmarLetterKhamtiHa),
            MYANMAR_LETTER_KHAMTI_HHA => Ok(MyanmarExtendedA::MyanmarLetterKhamtiHha),
            MYANMAR_LETTER_KHAMTI_FA => Ok(MyanmarExtendedA::MyanmarLetterKhamtiFa),
            MYANMAR_MODIFIER_LETTER_KHAMTI_REDUPLICATION => Ok(MyanmarExtendedA::MyanmarModifierLetterKhamtiReduplication),
            MYANMAR_LETTER_KHAMTI_XA => Ok(MyanmarExtendedA::MyanmarLetterKhamtiXa),
            MYANMAR_LETTER_KHAMTI_ZA => Ok(MyanmarExtendedA::MyanmarLetterKhamtiZa),
            MYANMAR_LETTER_KHAMTI_RA => Ok(MyanmarExtendedA::MyanmarLetterKhamtiRa),
            MYANMAR_LOGOGRAM_KHAMTI_OAY => Ok(MyanmarExtendedA::MyanmarLogogramKhamtiOay),
            MYANMAR_LOGOGRAM_KHAMTI_QN => Ok(MyanmarExtendedA::MyanmarLogogramKhamtiQn),
            MYANMAR_LOGOGRAM_KHAMTI_HM => Ok(MyanmarExtendedA::MyanmarLogogramKhamtiHm),
            MYANMAR_SYMBOL_AITON_EXCLAMATION => Ok(MyanmarExtendedA::MyanmarSymbolAitonExclamation),
            MYANMAR_SYMBOL_AITON_ONE => Ok(MyanmarExtendedA::MyanmarSymbolAitonOne),
            MYANMAR_SYMBOL_AITON_TWO => Ok(MyanmarExtendedA::MyanmarSymbolAitonTwo),
            MYANMAR_LETTER_AITON_RA => Ok(MyanmarExtendedA::MyanmarLetterAitonRa),
            MYANMAR_SIGN_PAO_KAREN_TONE => Ok(MyanmarExtendedA::MyanmarSignPaoKarenTone),
            MYANMAR_SIGN_TAI_LAING_TONE_DASH_2 => Ok(MyanmarExtendedA::MyanmarSignTaiLaingToneDash2),
            MYANMAR_SIGN_TAI_LAING_TONE_DASH_5 => Ok(MyanmarExtendedA::MyanmarSignTaiLaingToneDash5),
            MYANMAR_LETTER_SHWE_PALAUNG_CHA => Ok(MyanmarExtendedA::MyanmarLetterShwePalaungCha),
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

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            MyanmarExtendedA::MyanmarLetterKhamtiGa => "myanmar letter khamti ga",
            MyanmarExtendedA::MyanmarLetterKhamtiCa => "myanmar letter khamti ca",
            MyanmarExtendedA::MyanmarLetterKhamtiCha => "myanmar letter khamti cha",
            MyanmarExtendedA::MyanmarLetterKhamtiJa => "myanmar letter khamti ja",
            MyanmarExtendedA::MyanmarLetterKhamtiJha => "myanmar letter khamti jha",
            MyanmarExtendedA::MyanmarLetterKhamtiNya => "myanmar letter khamti nya",
            MyanmarExtendedA::MyanmarLetterKhamtiTta => "myanmar letter khamti tta",
            MyanmarExtendedA::MyanmarLetterKhamtiTtha => "myanmar letter khamti ttha",
            MyanmarExtendedA::MyanmarLetterKhamtiDda => "myanmar letter khamti dda",
            MyanmarExtendedA::MyanmarLetterKhamtiDdha => "myanmar letter khamti ddha",
            MyanmarExtendedA::MyanmarLetterKhamtiDha => "myanmar letter khamti dha",
            MyanmarExtendedA::MyanmarLetterKhamtiNa => "myanmar letter khamti na",
            MyanmarExtendedA::MyanmarLetterKhamtiSa => "myanmar letter khamti sa",
            MyanmarExtendedA::MyanmarLetterKhamtiHa => "myanmar letter khamti ha",
            MyanmarExtendedA::MyanmarLetterKhamtiHha => "myanmar letter khamti hha",
            MyanmarExtendedA::MyanmarLetterKhamtiFa => "myanmar letter khamti fa",
            MyanmarExtendedA::MyanmarModifierLetterKhamtiReduplication => "myanmar modifier letter khamti reduplication",
            MyanmarExtendedA::MyanmarLetterKhamtiXa => "myanmar letter khamti xa",
            MyanmarExtendedA::MyanmarLetterKhamtiZa => "myanmar letter khamti za",
            MyanmarExtendedA::MyanmarLetterKhamtiRa => "myanmar letter khamti ra",
            MyanmarExtendedA::MyanmarLogogramKhamtiOay => "myanmar logogram khamti oay",
            MyanmarExtendedA::MyanmarLogogramKhamtiQn => "myanmar logogram khamti qn",
            MyanmarExtendedA::MyanmarLogogramKhamtiHm => "myanmar logogram khamti hm",
            MyanmarExtendedA::MyanmarSymbolAitonExclamation => "myanmar symbol aiton exclamation",
            MyanmarExtendedA::MyanmarSymbolAitonOne => "myanmar symbol aiton one",
            MyanmarExtendedA::MyanmarSymbolAitonTwo => "myanmar symbol aiton two",
            MyanmarExtendedA::MyanmarLetterAitonRa => "myanmar letter aiton ra",
            MyanmarExtendedA::MyanmarSignPaoKarenTone => "myanmar sign pao karen tone",
            MyanmarExtendedA::MyanmarSignTaiLaingToneDash2 => "myanmar sign tai laing tone-2",
            MyanmarExtendedA::MyanmarSignTaiLaingToneDash5 => "myanmar sign tai laing tone-5",
            MyanmarExtendedA::MyanmarLetterShwePalaungCha => "myanmar letter shwe palaung cha",
        }
    }
}
