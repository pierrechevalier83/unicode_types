
/// An enum to represent all characters in the CyrillicExtendedB block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum CyrillicExtendedB {
    /// \u{a640}: 'Ꙁ'
    CyrillicCapitalLetterZemlya,
    /// \u{a641}: 'ꙁ'
    CyrillicSmallLetterZemlya,
    /// \u{a642}: 'Ꙃ'
    CyrillicCapitalLetterDzelo,
    /// \u{a643}: 'ꙃ'
    CyrillicSmallLetterDzelo,
    /// \u{a644}: 'Ꙅ'
    CyrillicCapitalLetterReversedDze,
    /// \u{a645}: 'ꙅ'
    CyrillicSmallLetterReversedDze,
    /// \u{a646}: 'Ꙇ'
    CyrillicCapitalLetterIota,
    /// \u{a647}: 'ꙇ'
    CyrillicSmallLetterIota,
    /// \u{a648}: 'Ꙉ'
    CyrillicCapitalLetterDjerv,
    /// \u{a649}: 'ꙉ'
    CyrillicSmallLetterDjerv,
    /// \u{a64a}: 'Ꙋ'
    CyrillicCapitalLetterMonographUk,
    /// \u{a64b}: 'ꙋ'
    CyrillicSmallLetterMonographUk,
    /// \u{a64c}: 'Ꙍ'
    CyrillicCapitalLetterBroadOmega,
    /// \u{a64d}: 'ꙍ'
    CyrillicSmallLetterBroadOmega,
    /// \u{a64e}: 'Ꙏ'
    CyrillicCapitalLetterNeutralYer,
    /// \u{a64f}: 'ꙏ'
    CyrillicSmallLetterNeutralYer,
    /// \u{a650}: 'Ꙑ'
    CyrillicCapitalLetterYeruWithBackYer,
    /// \u{a651}: 'ꙑ'
    CyrillicSmallLetterYeruWithBackYer,
    /// \u{a652}: 'Ꙓ'
    CyrillicCapitalLetterIotifiedYat,
    /// \u{a653}: 'ꙓ'
    CyrillicSmallLetterIotifiedYat,
    /// \u{a654}: 'Ꙕ'
    CyrillicCapitalLetterReversedYu,
    /// \u{a655}: 'ꙕ'
    CyrillicSmallLetterReversedYu,
    /// \u{a656}: 'Ꙗ'
    CyrillicCapitalLetterIotifiedA,
    /// \u{a657}: 'ꙗ'
    CyrillicSmallLetterIotifiedA,
    /// \u{a658}: 'Ꙙ'
    CyrillicCapitalLetterClosedLittleYus,
    /// \u{a659}: 'ꙙ'
    CyrillicSmallLetterClosedLittleYus,
    /// \u{a65a}: 'Ꙛ'
    CyrillicCapitalLetterBlendedYus,
    /// \u{a65b}: 'ꙛ'
    CyrillicSmallLetterBlendedYus,
    /// \u{a65c}: 'Ꙝ'
    CyrillicCapitalLetterIotifiedClosedLittleYus,
    /// \u{a65d}: 'ꙝ'
    CyrillicSmallLetterIotifiedClosedLittleYus,
    /// \u{a65e}: 'Ꙟ'
    CyrillicCapitalLetterYn,
    /// \u{a65f}: 'ꙟ'
    CyrillicSmallLetterYn,
    /// \u{a660}: 'Ꙡ'
    CyrillicCapitalLetterReversedTse,
    /// \u{a661}: 'ꙡ'
    CyrillicSmallLetterReversedTse,
    /// \u{a662}: 'Ꙣ'
    CyrillicCapitalLetterSoftDe,
    /// \u{a663}: 'ꙣ'
    CyrillicSmallLetterSoftDe,
    /// \u{a664}: 'Ꙥ'
    CyrillicCapitalLetterSoftEl,
    /// \u{a665}: 'ꙥ'
    CyrillicSmallLetterSoftEl,
    /// \u{a666}: 'Ꙧ'
    CyrillicCapitalLetterSoftEm,
    /// \u{a667}: 'ꙧ'
    CyrillicSmallLetterSoftEm,
    /// \u{a668}: 'Ꙩ'
    CyrillicCapitalLetterMonocularO,
    /// \u{a669}: 'ꙩ'
    CyrillicSmallLetterMonocularO,
    /// \u{a66a}: 'Ꙫ'
    CyrillicCapitalLetterBinocularO,
    /// \u{a66b}: 'ꙫ'
    CyrillicSmallLetterBinocularO,
    /// \u{a66c}: 'Ꙭ'
    CyrillicCapitalLetterDoubleMonocularO,
    /// \u{a66d}: 'ꙭ'
    CyrillicSmallLetterDoubleMonocularO,
    /// \u{a66e}: 'ꙮ'
    CyrillicLetterMultiocularO,
    /// \u{a66f}: '꙯'
    CombiningCyrillicVzmet,
    /// \u{a670}: '꙰'
    CombiningCyrillicTenMillionsSign,
    /// \u{a671}: '꙱'
    CombiningCyrillicHundredMillionsSign,
    /// \u{a672}: '꙲'
    CombiningCyrillicThousandMillionsSign,
    /// \u{a673}: '꙳'
    SlavonicAsterisk,
    /// \u{a674}: 'ꙴ'
    CombiningCyrillicLetterUkrainianIe,
    /// \u{a675}: 'ꙵ'
    CombiningCyrillicLetterI,
    /// \u{a676}: 'ꙶ'
    CombiningCyrillicLetterYi,
    /// \u{a677}: 'ꙷ'
    CombiningCyrillicLetterU,
    /// \u{a678}: 'ꙸ'
    CombiningCyrillicLetterHardSign,
    /// \u{a679}: 'ꙹ'
    CombiningCyrillicLetterYeru,
    /// \u{a67a}: 'ꙺ'
    CombiningCyrillicLetterSoftSign,
    /// \u{a67b}: 'ꙻ'
    CombiningCyrillicLetterOmega,
    /// \u{a67c}: '꙼'
    CombiningCyrillicKavyka,
    /// \u{a67d}: '꙽'
    CombiningCyrillicPayerok,
    /// \u{a67e}: '꙾'
    CyrillicKavyka,
    /// \u{a67f}: 'ꙿ'
    CyrillicPayerok,
    /// \u{a680}: 'Ꚁ'
    CyrillicCapitalLetterDwe,
    /// \u{a681}: 'ꚁ'
    CyrillicSmallLetterDwe,
    /// \u{a682}: 'Ꚃ'
    CyrillicCapitalLetterDzwe,
    /// \u{a683}: 'ꚃ'
    CyrillicSmallLetterDzwe,
    /// \u{a684}: 'Ꚅ'
    CyrillicCapitalLetterZhwe,
    /// \u{a685}: 'ꚅ'
    CyrillicSmallLetterZhwe,
    /// \u{a686}: 'Ꚇ'
    CyrillicCapitalLetterCche,
    /// \u{a687}: 'ꚇ'
    CyrillicSmallLetterCche,
    /// \u{a688}: 'Ꚉ'
    CyrillicCapitalLetterDzze,
    /// \u{a689}: 'ꚉ'
    CyrillicSmallLetterDzze,
    /// \u{a68a}: 'Ꚋ'
    CyrillicCapitalLetterTeWithMiddleHook,
    /// \u{a68b}: 'ꚋ'
    CyrillicSmallLetterTeWithMiddleHook,
    /// \u{a68c}: 'Ꚍ'
    CyrillicCapitalLetterTwe,
    /// \u{a68d}: 'ꚍ'
    CyrillicSmallLetterTwe,
    /// \u{a68e}: 'Ꚏ'
    CyrillicCapitalLetterTswe,
    /// \u{a68f}: 'ꚏ'
    CyrillicSmallLetterTswe,
    /// \u{a690}: 'Ꚑ'
    CyrillicCapitalLetterTsse,
    /// \u{a691}: 'ꚑ'
    CyrillicSmallLetterTsse,
    /// \u{a692}: 'Ꚓ'
    CyrillicCapitalLetterTche,
    /// \u{a693}: 'ꚓ'
    CyrillicSmallLetterTche,
    /// \u{a694}: 'Ꚕ'
    CyrillicCapitalLetterHwe,
    /// \u{a695}: 'ꚕ'
    CyrillicSmallLetterHwe,
    /// \u{a696}: 'Ꚗ'
    CyrillicCapitalLetterShwe,
    /// \u{a697}: 'ꚗ'
    CyrillicSmallLetterShwe,
    /// \u{a698}: 'Ꚙ'
    CyrillicCapitalLetterDoubleO,
    /// \u{a699}: 'ꚙ'
    CyrillicSmallLetterDoubleO,
    /// \u{a69a}: 'Ꚛ'
    CyrillicCapitalLetterCrossedO,
    /// \u{a69b}: 'ꚛ'
    CyrillicSmallLetterCrossedO,
    /// \u{a69c}: 'ꚜ'
    ModifierLetterCyrillicHardSign,
    /// \u{a69d}: 'ꚝ'
    ModifierLetterCyrillicSoftSign,
    /// \u{a69e}: 'ꚞ'
    CombiningCyrillicLetterEf,
}

impl Into<char> for CyrillicExtendedB {
    fn into(self) -> char {
        match self {
            CyrillicExtendedB::CyrillicCapitalLetterZemlya => 'Ꙁ',
            CyrillicExtendedB::CyrillicSmallLetterZemlya => 'ꙁ',
            CyrillicExtendedB::CyrillicCapitalLetterDzelo => 'Ꙃ',
            CyrillicExtendedB::CyrillicSmallLetterDzelo => 'ꙃ',
            CyrillicExtendedB::CyrillicCapitalLetterReversedDze => 'Ꙅ',
            CyrillicExtendedB::CyrillicSmallLetterReversedDze => 'ꙅ',
            CyrillicExtendedB::CyrillicCapitalLetterIota => 'Ꙇ',
            CyrillicExtendedB::CyrillicSmallLetterIota => 'ꙇ',
            CyrillicExtendedB::CyrillicCapitalLetterDjerv => 'Ꙉ',
            CyrillicExtendedB::CyrillicSmallLetterDjerv => 'ꙉ',
            CyrillicExtendedB::CyrillicCapitalLetterMonographUk => 'Ꙋ',
            CyrillicExtendedB::CyrillicSmallLetterMonographUk => 'ꙋ',
            CyrillicExtendedB::CyrillicCapitalLetterBroadOmega => 'Ꙍ',
            CyrillicExtendedB::CyrillicSmallLetterBroadOmega => 'ꙍ',
            CyrillicExtendedB::CyrillicCapitalLetterNeutralYer => 'Ꙏ',
            CyrillicExtendedB::CyrillicSmallLetterNeutralYer => 'ꙏ',
            CyrillicExtendedB::CyrillicCapitalLetterYeruWithBackYer => 'Ꙑ',
            CyrillicExtendedB::CyrillicSmallLetterYeruWithBackYer => 'ꙑ',
            CyrillicExtendedB::CyrillicCapitalLetterIotifiedYat => 'Ꙓ',
            CyrillicExtendedB::CyrillicSmallLetterIotifiedYat => 'ꙓ',
            CyrillicExtendedB::CyrillicCapitalLetterReversedYu => 'Ꙕ',
            CyrillicExtendedB::CyrillicSmallLetterReversedYu => 'ꙕ',
            CyrillicExtendedB::CyrillicCapitalLetterIotifiedA => 'Ꙗ',
            CyrillicExtendedB::CyrillicSmallLetterIotifiedA => 'ꙗ',
            CyrillicExtendedB::CyrillicCapitalLetterClosedLittleYus => 'Ꙙ',
            CyrillicExtendedB::CyrillicSmallLetterClosedLittleYus => 'ꙙ',
            CyrillicExtendedB::CyrillicCapitalLetterBlendedYus => 'Ꙛ',
            CyrillicExtendedB::CyrillicSmallLetterBlendedYus => 'ꙛ',
            CyrillicExtendedB::CyrillicCapitalLetterIotifiedClosedLittleYus => 'Ꙝ',
            CyrillicExtendedB::CyrillicSmallLetterIotifiedClosedLittleYus => 'ꙝ',
            CyrillicExtendedB::CyrillicCapitalLetterYn => 'Ꙟ',
            CyrillicExtendedB::CyrillicSmallLetterYn => 'ꙟ',
            CyrillicExtendedB::CyrillicCapitalLetterReversedTse => 'Ꙡ',
            CyrillicExtendedB::CyrillicSmallLetterReversedTse => 'ꙡ',
            CyrillicExtendedB::CyrillicCapitalLetterSoftDe => 'Ꙣ',
            CyrillicExtendedB::CyrillicSmallLetterSoftDe => 'ꙣ',
            CyrillicExtendedB::CyrillicCapitalLetterSoftEl => 'Ꙥ',
            CyrillicExtendedB::CyrillicSmallLetterSoftEl => 'ꙥ',
            CyrillicExtendedB::CyrillicCapitalLetterSoftEm => 'Ꙧ',
            CyrillicExtendedB::CyrillicSmallLetterSoftEm => 'ꙧ',
            CyrillicExtendedB::CyrillicCapitalLetterMonocularO => 'Ꙩ',
            CyrillicExtendedB::CyrillicSmallLetterMonocularO => 'ꙩ',
            CyrillicExtendedB::CyrillicCapitalLetterBinocularO => 'Ꙫ',
            CyrillicExtendedB::CyrillicSmallLetterBinocularO => 'ꙫ',
            CyrillicExtendedB::CyrillicCapitalLetterDoubleMonocularO => 'Ꙭ',
            CyrillicExtendedB::CyrillicSmallLetterDoubleMonocularO => 'ꙭ',
            CyrillicExtendedB::CyrillicLetterMultiocularO => 'ꙮ',
            CyrillicExtendedB::CombiningCyrillicVzmet => '꙯',
            CyrillicExtendedB::CombiningCyrillicTenMillionsSign => '꙰',
            CyrillicExtendedB::CombiningCyrillicHundredMillionsSign => '꙱',
            CyrillicExtendedB::CombiningCyrillicThousandMillionsSign => '꙲',
            CyrillicExtendedB::SlavonicAsterisk => '꙳',
            CyrillicExtendedB::CombiningCyrillicLetterUkrainianIe => 'ꙴ',
            CyrillicExtendedB::CombiningCyrillicLetterI => 'ꙵ',
            CyrillicExtendedB::CombiningCyrillicLetterYi => 'ꙶ',
            CyrillicExtendedB::CombiningCyrillicLetterU => 'ꙷ',
            CyrillicExtendedB::CombiningCyrillicLetterHardSign => 'ꙸ',
            CyrillicExtendedB::CombiningCyrillicLetterYeru => 'ꙹ',
            CyrillicExtendedB::CombiningCyrillicLetterSoftSign => 'ꙺ',
            CyrillicExtendedB::CombiningCyrillicLetterOmega => 'ꙻ',
            CyrillicExtendedB::CombiningCyrillicKavyka => '꙼',
            CyrillicExtendedB::CombiningCyrillicPayerok => '꙽',
            CyrillicExtendedB::CyrillicKavyka => '꙾',
            CyrillicExtendedB::CyrillicPayerok => 'ꙿ',
            CyrillicExtendedB::CyrillicCapitalLetterDwe => 'Ꚁ',
            CyrillicExtendedB::CyrillicSmallLetterDwe => 'ꚁ',
            CyrillicExtendedB::CyrillicCapitalLetterDzwe => 'Ꚃ',
            CyrillicExtendedB::CyrillicSmallLetterDzwe => 'ꚃ',
            CyrillicExtendedB::CyrillicCapitalLetterZhwe => 'Ꚅ',
            CyrillicExtendedB::CyrillicSmallLetterZhwe => 'ꚅ',
            CyrillicExtendedB::CyrillicCapitalLetterCche => 'Ꚇ',
            CyrillicExtendedB::CyrillicSmallLetterCche => 'ꚇ',
            CyrillicExtendedB::CyrillicCapitalLetterDzze => 'Ꚉ',
            CyrillicExtendedB::CyrillicSmallLetterDzze => 'ꚉ',
            CyrillicExtendedB::CyrillicCapitalLetterTeWithMiddleHook => 'Ꚋ',
            CyrillicExtendedB::CyrillicSmallLetterTeWithMiddleHook => 'ꚋ',
            CyrillicExtendedB::CyrillicCapitalLetterTwe => 'Ꚍ',
            CyrillicExtendedB::CyrillicSmallLetterTwe => 'ꚍ',
            CyrillicExtendedB::CyrillicCapitalLetterTswe => 'Ꚏ',
            CyrillicExtendedB::CyrillicSmallLetterTswe => 'ꚏ',
            CyrillicExtendedB::CyrillicCapitalLetterTsse => 'Ꚑ',
            CyrillicExtendedB::CyrillicSmallLetterTsse => 'ꚑ',
            CyrillicExtendedB::CyrillicCapitalLetterTche => 'Ꚓ',
            CyrillicExtendedB::CyrillicSmallLetterTche => 'ꚓ',
            CyrillicExtendedB::CyrillicCapitalLetterHwe => 'Ꚕ',
            CyrillicExtendedB::CyrillicSmallLetterHwe => 'ꚕ',
            CyrillicExtendedB::CyrillicCapitalLetterShwe => 'Ꚗ',
            CyrillicExtendedB::CyrillicSmallLetterShwe => 'ꚗ',
            CyrillicExtendedB::CyrillicCapitalLetterDoubleO => 'Ꚙ',
            CyrillicExtendedB::CyrillicSmallLetterDoubleO => 'ꚙ',
            CyrillicExtendedB::CyrillicCapitalLetterCrossedO => 'Ꚛ',
            CyrillicExtendedB::CyrillicSmallLetterCrossedO => 'ꚛ',
            CyrillicExtendedB::ModifierLetterCyrillicHardSign => 'ꚜ',
            CyrillicExtendedB::ModifierLetterCyrillicSoftSign => 'ꚝ',
            CyrillicExtendedB::CombiningCyrillicLetterEf => 'ꚞ',
        }
    }
}

impl std::convert::TryFrom<char> for CyrillicExtendedB {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'Ꙁ' => Ok(CyrillicExtendedB::CyrillicCapitalLetterZemlya),
            'ꙁ' => Ok(CyrillicExtendedB::CyrillicSmallLetterZemlya),
            'Ꙃ' => Ok(CyrillicExtendedB::CyrillicCapitalLetterDzelo),
            'ꙃ' => Ok(CyrillicExtendedB::CyrillicSmallLetterDzelo),
            'Ꙅ' => Ok(CyrillicExtendedB::CyrillicCapitalLetterReversedDze),
            'ꙅ' => Ok(CyrillicExtendedB::CyrillicSmallLetterReversedDze),
            'Ꙇ' => Ok(CyrillicExtendedB::CyrillicCapitalLetterIota),
            'ꙇ' => Ok(CyrillicExtendedB::CyrillicSmallLetterIota),
            'Ꙉ' => Ok(CyrillicExtendedB::CyrillicCapitalLetterDjerv),
            'ꙉ' => Ok(CyrillicExtendedB::CyrillicSmallLetterDjerv),
            'Ꙋ' => Ok(CyrillicExtendedB::CyrillicCapitalLetterMonographUk),
            'ꙋ' => Ok(CyrillicExtendedB::CyrillicSmallLetterMonographUk),
            'Ꙍ' => Ok(CyrillicExtendedB::CyrillicCapitalLetterBroadOmega),
            'ꙍ' => Ok(CyrillicExtendedB::CyrillicSmallLetterBroadOmega),
            'Ꙏ' => Ok(CyrillicExtendedB::CyrillicCapitalLetterNeutralYer),
            'ꙏ' => Ok(CyrillicExtendedB::CyrillicSmallLetterNeutralYer),
            'Ꙑ' => Ok(CyrillicExtendedB::CyrillicCapitalLetterYeruWithBackYer),
            'ꙑ' => Ok(CyrillicExtendedB::CyrillicSmallLetterYeruWithBackYer),
            'Ꙓ' => Ok(CyrillicExtendedB::CyrillicCapitalLetterIotifiedYat),
            'ꙓ' => Ok(CyrillicExtendedB::CyrillicSmallLetterIotifiedYat),
            'Ꙕ' => Ok(CyrillicExtendedB::CyrillicCapitalLetterReversedYu),
            'ꙕ' => Ok(CyrillicExtendedB::CyrillicSmallLetterReversedYu),
            'Ꙗ' => Ok(CyrillicExtendedB::CyrillicCapitalLetterIotifiedA),
            'ꙗ' => Ok(CyrillicExtendedB::CyrillicSmallLetterIotifiedA),
            'Ꙙ' => Ok(CyrillicExtendedB::CyrillicCapitalLetterClosedLittleYus),
            'ꙙ' => Ok(CyrillicExtendedB::CyrillicSmallLetterClosedLittleYus),
            'Ꙛ' => Ok(CyrillicExtendedB::CyrillicCapitalLetterBlendedYus),
            'ꙛ' => Ok(CyrillicExtendedB::CyrillicSmallLetterBlendedYus),
            'Ꙝ' => Ok(CyrillicExtendedB::CyrillicCapitalLetterIotifiedClosedLittleYus),
            'ꙝ' => Ok(CyrillicExtendedB::CyrillicSmallLetterIotifiedClosedLittleYus),
            'Ꙟ' => Ok(CyrillicExtendedB::CyrillicCapitalLetterYn),
            'ꙟ' => Ok(CyrillicExtendedB::CyrillicSmallLetterYn),
            'Ꙡ' => Ok(CyrillicExtendedB::CyrillicCapitalLetterReversedTse),
            'ꙡ' => Ok(CyrillicExtendedB::CyrillicSmallLetterReversedTse),
            'Ꙣ' => Ok(CyrillicExtendedB::CyrillicCapitalLetterSoftDe),
            'ꙣ' => Ok(CyrillicExtendedB::CyrillicSmallLetterSoftDe),
            'Ꙥ' => Ok(CyrillicExtendedB::CyrillicCapitalLetterSoftEl),
            'ꙥ' => Ok(CyrillicExtendedB::CyrillicSmallLetterSoftEl),
            'Ꙧ' => Ok(CyrillicExtendedB::CyrillicCapitalLetterSoftEm),
            'ꙧ' => Ok(CyrillicExtendedB::CyrillicSmallLetterSoftEm),
            'Ꙩ' => Ok(CyrillicExtendedB::CyrillicCapitalLetterMonocularO),
            'ꙩ' => Ok(CyrillicExtendedB::CyrillicSmallLetterMonocularO),
            'Ꙫ' => Ok(CyrillicExtendedB::CyrillicCapitalLetterBinocularO),
            'ꙫ' => Ok(CyrillicExtendedB::CyrillicSmallLetterBinocularO),
            'Ꙭ' => Ok(CyrillicExtendedB::CyrillicCapitalLetterDoubleMonocularO),
            'ꙭ' => Ok(CyrillicExtendedB::CyrillicSmallLetterDoubleMonocularO),
            'ꙮ' => Ok(CyrillicExtendedB::CyrillicLetterMultiocularO),
            '꙯' => Ok(CyrillicExtendedB::CombiningCyrillicVzmet),
            '꙰' => Ok(CyrillicExtendedB::CombiningCyrillicTenMillionsSign),
            '꙱' => Ok(CyrillicExtendedB::CombiningCyrillicHundredMillionsSign),
            '꙲' => Ok(CyrillicExtendedB::CombiningCyrillicThousandMillionsSign),
            '꙳' => Ok(CyrillicExtendedB::SlavonicAsterisk),
            'ꙴ' => Ok(CyrillicExtendedB::CombiningCyrillicLetterUkrainianIe),
            'ꙵ' => Ok(CyrillicExtendedB::CombiningCyrillicLetterI),
            'ꙶ' => Ok(CyrillicExtendedB::CombiningCyrillicLetterYi),
            'ꙷ' => Ok(CyrillicExtendedB::CombiningCyrillicLetterU),
            'ꙸ' => Ok(CyrillicExtendedB::CombiningCyrillicLetterHardSign),
            'ꙹ' => Ok(CyrillicExtendedB::CombiningCyrillicLetterYeru),
            'ꙺ' => Ok(CyrillicExtendedB::CombiningCyrillicLetterSoftSign),
            'ꙻ' => Ok(CyrillicExtendedB::CombiningCyrillicLetterOmega),
            '꙼' => Ok(CyrillicExtendedB::CombiningCyrillicKavyka),
            '꙽' => Ok(CyrillicExtendedB::CombiningCyrillicPayerok),
            '꙾' => Ok(CyrillicExtendedB::CyrillicKavyka),
            'ꙿ' => Ok(CyrillicExtendedB::CyrillicPayerok),
            'Ꚁ' => Ok(CyrillicExtendedB::CyrillicCapitalLetterDwe),
            'ꚁ' => Ok(CyrillicExtendedB::CyrillicSmallLetterDwe),
            'Ꚃ' => Ok(CyrillicExtendedB::CyrillicCapitalLetterDzwe),
            'ꚃ' => Ok(CyrillicExtendedB::CyrillicSmallLetterDzwe),
            'Ꚅ' => Ok(CyrillicExtendedB::CyrillicCapitalLetterZhwe),
            'ꚅ' => Ok(CyrillicExtendedB::CyrillicSmallLetterZhwe),
            'Ꚇ' => Ok(CyrillicExtendedB::CyrillicCapitalLetterCche),
            'ꚇ' => Ok(CyrillicExtendedB::CyrillicSmallLetterCche),
            'Ꚉ' => Ok(CyrillicExtendedB::CyrillicCapitalLetterDzze),
            'ꚉ' => Ok(CyrillicExtendedB::CyrillicSmallLetterDzze),
            'Ꚋ' => Ok(CyrillicExtendedB::CyrillicCapitalLetterTeWithMiddleHook),
            'ꚋ' => Ok(CyrillicExtendedB::CyrillicSmallLetterTeWithMiddleHook),
            'Ꚍ' => Ok(CyrillicExtendedB::CyrillicCapitalLetterTwe),
            'ꚍ' => Ok(CyrillicExtendedB::CyrillicSmallLetterTwe),
            'Ꚏ' => Ok(CyrillicExtendedB::CyrillicCapitalLetterTswe),
            'ꚏ' => Ok(CyrillicExtendedB::CyrillicSmallLetterTswe),
            'Ꚑ' => Ok(CyrillicExtendedB::CyrillicCapitalLetterTsse),
            'ꚑ' => Ok(CyrillicExtendedB::CyrillicSmallLetterTsse),
            'Ꚓ' => Ok(CyrillicExtendedB::CyrillicCapitalLetterTche),
            'ꚓ' => Ok(CyrillicExtendedB::CyrillicSmallLetterTche),
            'Ꚕ' => Ok(CyrillicExtendedB::CyrillicCapitalLetterHwe),
            'ꚕ' => Ok(CyrillicExtendedB::CyrillicSmallLetterHwe),
            'Ꚗ' => Ok(CyrillicExtendedB::CyrillicCapitalLetterShwe),
            'ꚗ' => Ok(CyrillicExtendedB::CyrillicSmallLetterShwe),
            'Ꚙ' => Ok(CyrillicExtendedB::CyrillicCapitalLetterDoubleO),
            'ꚙ' => Ok(CyrillicExtendedB::CyrillicSmallLetterDoubleO),
            'Ꚛ' => Ok(CyrillicExtendedB::CyrillicCapitalLetterCrossedO),
            'ꚛ' => Ok(CyrillicExtendedB::CyrillicSmallLetterCrossedO),
            'ꚜ' => Ok(CyrillicExtendedB::ModifierLetterCyrillicHardSign),
            'ꚝ' => Ok(CyrillicExtendedB::ModifierLetterCyrillicSoftSign),
            'ꚞ' => Ok(CyrillicExtendedB::CombiningCyrillicLetterEf),
            _ => Err(()),
        }
    }
}

impl Into<u32> for CyrillicExtendedB {
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

impl std::convert::TryFrom<u32> for CyrillicExtendedB {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for CyrillicExtendedB {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl CyrillicExtendedB {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        CyrillicExtendedB::CyrillicCapitalLetterZemlya
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("CyrillicExtendedB{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
