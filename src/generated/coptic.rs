/// \u{2c80} → \u{2cff}
///
/// Ⲁ ⲁ Ⲃ ⲃ Ⲅ ⲅ Ⲇ ⲇ Ⲉ ⲉ Ⲋ ⲋ Ⲍ ⲍ Ⲏ ⲏ\
/// Ⲑ ⲑ Ⲓ ⲓ Ⲕ ⲕ Ⲗ ⲗ Ⲙ ⲙ Ⲛ ⲛ Ⲝ ⲝ Ⲟ ⲟ\
/// Ⲡ ⲡ Ⲣ ⲣ Ⲥ ⲥ Ⲧ ⲧ Ⲩ ⲩ Ⲫ ⲫ Ⲭ ⲭ Ⲯ ⲯ\
/// Ⲱ ⲱ Ⲳ ⲳ Ⲵ ⲵ Ⲷ ⲷ Ⲹ ⲹ Ⲻ ⲻ Ⲽ ⲽ Ⲿ ⲿ\
/// Ⳁ ⳁ Ⳃ ⳃ Ⳅ ⳅ Ⳇ ⳇ Ⳉ ⳉ Ⳋ ⳋ Ⳍ ⳍ Ⳏ ⳏ\
/// Ⳑ ⳑ Ⳓ ⳓ Ⳕ ⳕ Ⳗ ⳗ Ⳙ ⳙ Ⳛ ⳛ Ⳝ ⳝ Ⳟ ⳟ\
/// Ⳡ ⳡ Ⳣ ⳣ ⳤ ⳥ ⳦ ⳧ ⳨ ⳩ ⳪ Ⳬ ⳬ Ⳮ ⳮ ⳯\
/// ⳰ ⳱ Ⳳ ⳳ ⳹ ⳺ ⳻ ⳼ ⳽ ⳾\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{2c80}: 'Ⲁ'
    pub const CAPITAL_LETTER_ALFA: char = 'Ⲁ';
    /// \u{2c81}: 'ⲁ'
    pub const SMALL_LETTER_ALFA: char = 'ⲁ';
    /// \u{2c82}: 'Ⲃ'
    pub const CAPITAL_LETTER_VIDA: char = 'Ⲃ';
    /// \u{2c83}: 'ⲃ'
    pub const SMALL_LETTER_VIDA: char = 'ⲃ';
    /// \u{2c84}: 'Ⲅ'
    pub const CAPITAL_LETTER_GAMMA: char = 'Ⲅ';
    /// \u{2c85}: 'ⲅ'
    pub const SMALL_LETTER_GAMMA: char = 'ⲅ';
    /// \u{2c86}: 'Ⲇ'
    pub const CAPITAL_LETTER_DALDA: char = 'Ⲇ';
    /// \u{2c87}: 'ⲇ'
    pub const SMALL_LETTER_DALDA: char = 'ⲇ';
    /// \u{2c88}: 'Ⲉ'
    pub const CAPITAL_LETTER_EIE: char = 'Ⲉ';
    /// \u{2c89}: 'ⲉ'
    pub const SMALL_LETTER_EIE: char = 'ⲉ';
    /// \u{2c8a}: 'Ⲋ'
    pub const CAPITAL_LETTER_SOU: char = 'Ⲋ';
    /// \u{2c8b}: 'ⲋ'
    pub const SMALL_LETTER_SOU: char = 'ⲋ';
    /// \u{2c8c}: 'Ⲍ'
    pub const CAPITAL_LETTER_ZATA: char = 'Ⲍ';
    /// \u{2c8d}: 'ⲍ'
    pub const SMALL_LETTER_ZATA: char = 'ⲍ';
    /// \u{2c8e}: 'Ⲏ'
    pub const CAPITAL_LETTER_HATE: char = 'Ⲏ';
    /// \u{2c8f}: 'ⲏ'
    pub const SMALL_LETTER_HATE: char = 'ⲏ';
    /// \u{2c90}: 'Ⲑ'
    pub const CAPITAL_LETTER_THETHE: char = 'Ⲑ';
    /// \u{2c91}: 'ⲑ'
    pub const SMALL_LETTER_THETHE: char = 'ⲑ';
    /// \u{2c92}: 'Ⲓ'
    pub const CAPITAL_LETTER_IAUDA: char = 'Ⲓ';
    /// \u{2c93}: 'ⲓ'
    pub const SMALL_LETTER_IAUDA: char = 'ⲓ';
    /// \u{2c94}: 'Ⲕ'
    pub const CAPITAL_LETTER_KAPA: char = 'Ⲕ';
    /// \u{2c95}: 'ⲕ'
    pub const SMALL_LETTER_KAPA: char = 'ⲕ';
    /// \u{2c96}: 'Ⲗ'
    pub const CAPITAL_LETTER_LAULA: char = 'Ⲗ';
    /// \u{2c97}: 'ⲗ'
    pub const SMALL_LETTER_LAULA: char = 'ⲗ';
    /// \u{2c98}: 'Ⲙ'
    pub const CAPITAL_LETTER_MI: char = 'Ⲙ';
    /// \u{2c99}: 'ⲙ'
    pub const SMALL_LETTER_MI: char = 'ⲙ';
    /// \u{2c9a}: 'Ⲛ'
    pub const CAPITAL_LETTER_NI: char = 'Ⲛ';
    /// \u{2c9b}: 'ⲛ'
    pub const SMALL_LETTER_NI: char = 'ⲛ';
    /// \u{2c9c}: 'Ⲝ'
    pub const CAPITAL_LETTER_KSI: char = 'Ⲝ';
    /// \u{2c9d}: 'ⲝ'
    pub const SMALL_LETTER_KSI: char = 'ⲝ';
    /// \u{2c9e}: 'Ⲟ'
    pub const CAPITAL_LETTER_O: char = 'Ⲟ';
    /// \u{2c9f}: 'ⲟ'
    pub const SMALL_LETTER_O: char = 'ⲟ';
    /// \u{2ca0}: 'Ⲡ'
    pub const CAPITAL_LETTER_PI: char = 'Ⲡ';
    /// \u{2ca1}: 'ⲡ'
    pub const SMALL_LETTER_PI: char = 'ⲡ';
    /// \u{2ca2}: 'Ⲣ'
    pub const CAPITAL_LETTER_RO: char = 'Ⲣ';
    /// \u{2ca3}: 'ⲣ'
    pub const SMALL_LETTER_RO: char = 'ⲣ';
    /// \u{2ca4}: 'Ⲥ'
    pub const CAPITAL_LETTER_SIMA: char = 'Ⲥ';
    /// \u{2ca5}: 'ⲥ'
    pub const SMALL_LETTER_SIMA: char = 'ⲥ';
    /// \u{2ca6}: 'Ⲧ'
    pub const CAPITAL_LETTER_TAU: char = 'Ⲧ';
    /// \u{2ca7}: 'ⲧ'
    pub const SMALL_LETTER_TAU: char = 'ⲧ';
    /// \u{2ca8}: 'Ⲩ'
    pub const CAPITAL_LETTER_UA: char = 'Ⲩ';
    /// \u{2ca9}: 'ⲩ'
    pub const SMALL_LETTER_UA: char = 'ⲩ';
    /// \u{2caa}: 'Ⲫ'
    pub const CAPITAL_LETTER_FI: char = 'Ⲫ';
    /// \u{2cab}: 'ⲫ'
    pub const SMALL_LETTER_FI: char = 'ⲫ';
    /// \u{2cac}: 'Ⲭ'
    pub const CAPITAL_LETTER_KHI: char = 'Ⲭ';
    /// \u{2cad}: 'ⲭ'
    pub const SMALL_LETTER_KHI: char = 'ⲭ';
    /// \u{2cae}: 'Ⲯ'
    pub const CAPITAL_LETTER_PSI: char = 'Ⲯ';
    /// \u{2caf}: 'ⲯ'
    pub const SMALL_LETTER_PSI: char = 'ⲯ';
    /// \u{2cb0}: 'Ⲱ'
    pub const CAPITAL_LETTER_OOU: char = 'Ⲱ';
    /// \u{2cb1}: 'ⲱ'
    pub const SMALL_LETTER_OOU: char = 'ⲱ';
    /// \u{2cb2}: 'Ⲳ'
    pub const CAPITAL_LETTER_DIALECT_DASH_P_ALEF: char = 'Ⲳ';
    /// \u{2cb3}: 'ⲳ'
    pub const SMALL_LETTER_DIALECT_DASH_P_ALEF: char = 'ⲳ';
    /// \u{2cb4}: 'Ⲵ'
    pub const CAPITAL_LETTER_OLD_AIN: char = 'Ⲵ';
    /// \u{2cb5}: 'ⲵ'
    pub const SMALL_LETTER_OLD_AIN: char = 'ⲵ';
    /// \u{2cb6}: 'Ⲷ'
    pub const CAPITAL_LETTER_CRYPTOGRAMMIC_EIE: char = 'Ⲷ';
    /// \u{2cb7}: 'ⲷ'
    pub const SMALL_LETTER_CRYPTOGRAMMIC_EIE: char = 'ⲷ';
    /// \u{2cb8}: 'Ⲹ'
    pub const CAPITAL_LETTER_DIALECT_DASH_P_KAPA: char = 'Ⲹ';
    /// \u{2cb9}: 'ⲹ'
    pub const SMALL_LETTER_DIALECT_DASH_P_KAPA: char = 'ⲹ';
    /// \u{2cba}: 'Ⲻ'
    pub const CAPITAL_LETTER_DIALECT_DASH_P_NI: char = 'Ⲻ';
    /// \u{2cbb}: 'ⲻ'
    pub const SMALL_LETTER_DIALECT_DASH_P_NI: char = 'ⲻ';
    /// \u{2cbc}: 'Ⲽ'
    pub const CAPITAL_LETTER_CRYPTOGRAMMIC_NI: char = 'Ⲽ';
    /// \u{2cbd}: 'ⲽ'
    pub const SMALL_LETTER_CRYPTOGRAMMIC_NI: char = 'ⲽ';
    /// \u{2cbe}: 'Ⲿ'
    pub const CAPITAL_LETTER_OLD_OOU: char = 'Ⲿ';
    /// \u{2cbf}: 'ⲿ'
    pub const SMALL_LETTER_OLD_OOU: char = 'ⲿ';
    /// \u{2cc0}: 'Ⳁ'
    pub const CAPITAL_LETTER_SAMPI: char = 'Ⳁ';
    /// \u{2cc1}: 'ⳁ'
    pub const SMALL_LETTER_SAMPI: char = 'ⳁ';
    /// \u{2cc2}: 'Ⳃ'
    pub const CAPITAL_LETTER_CROSSED_SHEI: char = 'Ⳃ';
    /// \u{2cc3}: 'ⳃ'
    pub const SMALL_LETTER_CROSSED_SHEI: char = 'ⳃ';
    /// \u{2cc4}: 'Ⳅ'
    pub const CAPITAL_LETTER_OLD_SHEI: char = 'Ⳅ';
    /// \u{2cc5}: 'ⳅ'
    pub const SMALL_LETTER_OLD_SHEI: char = 'ⳅ';
    /// \u{2cc6}: 'Ⳇ'
    pub const CAPITAL_LETTER_OLD_ESH: char = 'Ⳇ';
    /// \u{2cc7}: 'ⳇ'
    pub const SMALL_LETTER_OLD_ESH: char = 'ⳇ';
    /// \u{2cc8}: 'Ⳉ'
    pub const CAPITAL_LETTER_AKHMIMIC_KHEI: char = 'Ⳉ';
    /// \u{2cc9}: 'ⳉ'
    pub const SMALL_LETTER_AKHMIMIC_KHEI: char = 'ⳉ';
    /// \u{2cca}: 'Ⳋ'
    pub const CAPITAL_LETTER_DIALECT_DASH_P_HORI: char = 'Ⳋ';
    /// \u{2ccb}: 'ⳋ'
    pub const SMALL_LETTER_DIALECT_DASH_P_HORI: char = 'ⳋ';
    /// \u{2ccc}: 'Ⳍ'
    pub const CAPITAL_LETTER_OLD_HORI: char = 'Ⳍ';
    /// \u{2ccd}: 'ⳍ'
    pub const SMALL_LETTER_OLD_HORI: char = 'ⳍ';
    /// \u{2cce}: 'Ⳏ'
    pub const CAPITAL_LETTER_OLD_HA: char = 'Ⳏ';
    /// \u{2ccf}: 'ⳏ'
    pub const SMALL_LETTER_OLD_HA: char = 'ⳏ';
    /// \u{2cd0}: 'Ⳑ'
    pub const CAPITAL_LETTER_L_DASH_SHAPED_HA: char = 'Ⳑ';
    /// \u{2cd1}: 'ⳑ'
    pub const SMALL_LETTER_L_DASH_SHAPED_HA: char = 'ⳑ';
    /// \u{2cd2}: 'Ⳓ'
    pub const CAPITAL_LETTER_OLD_HEI: char = 'Ⳓ';
    /// \u{2cd3}: 'ⳓ'
    pub const SMALL_LETTER_OLD_HEI: char = 'ⳓ';
    /// \u{2cd4}: 'Ⳕ'
    pub const CAPITAL_LETTER_OLD_HAT: char = 'Ⳕ';
    /// \u{2cd5}: 'ⳕ'
    pub const SMALL_LETTER_OLD_HAT: char = 'ⳕ';
    /// \u{2cd6}: 'Ⳗ'
    pub const CAPITAL_LETTER_OLD_GANGIA: char = 'Ⳗ';
    /// \u{2cd7}: 'ⳗ'
    pub const SMALL_LETTER_OLD_GANGIA: char = 'ⳗ';
    /// \u{2cd8}: 'Ⳙ'
    pub const CAPITAL_LETTER_OLD_DJA: char = 'Ⳙ';
    /// \u{2cd9}: 'ⳙ'
    pub const SMALL_LETTER_OLD_DJA: char = 'ⳙ';
    /// \u{2cda}: 'Ⳛ'
    pub const CAPITAL_LETTER_OLD_SHIMA: char = 'Ⳛ';
    /// \u{2cdb}: 'ⳛ'
    pub const SMALL_LETTER_OLD_SHIMA: char = 'ⳛ';
    /// \u{2cdc}: 'Ⳝ'
    pub const CAPITAL_LETTER_OLD_NUBIAN_SHIMA: char = 'Ⳝ';
    /// \u{2cdd}: 'ⳝ'
    pub const SMALL_LETTER_OLD_NUBIAN_SHIMA: char = 'ⳝ';
    /// \u{2cde}: 'Ⳟ'
    pub const CAPITAL_LETTER_OLD_NUBIAN_NGI: char = 'Ⳟ';
    /// \u{2cdf}: 'ⳟ'
    pub const SMALL_LETTER_OLD_NUBIAN_NGI: char = 'ⳟ';
    /// \u{2ce0}: 'Ⳡ'
    pub const CAPITAL_LETTER_OLD_NUBIAN_NYI: char = 'Ⳡ';
    /// \u{2ce1}: 'ⳡ'
    pub const SMALL_LETTER_OLD_NUBIAN_NYI: char = 'ⳡ';
    /// \u{2ce2}: 'Ⳣ'
    pub const CAPITAL_LETTER_OLD_NUBIAN_WAU: char = 'Ⳣ';
    /// \u{2ce3}: 'ⳣ'
    pub const SMALL_LETTER_OLD_NUBIAN_WAU: char = 'ⳣ';
    /// \u{2ce4}: 'ⳤ'
    pub const SYMBOL_KAI: char = 'ⳤ';
    /// \u{2ce5}: '⳥'
    pub const SYMBOL_MI_RO: char = '⳥';
    /// \u{2ce6}: '⳦'
    pub const SYMBOL_PI_RO: char = '⳦';
    /// \u{2ce7}: '⳧'
    pub const SYMBOL_STAUROS: char = '⳧';
    /// \u{2ce8}: '⳨'
    pub const SYMBOL_TAU_RO: char = '⳨';
    /// \u{2ce9}: '⳩'
    pub const SYMBOL_KHI_RO: char = '⳩';
    /// \u{2cea}: '⳪'
    pub const SYMBOL_SHIMA_SIMA: char = '⳪';
    /// \u{2ceb}: 'Ⳬ'
    pub const CAPITAL_LETTER_CRYPTOGRAMMIC_SHEI: char = 'Ⳬ';
    /// \u{2cec}: 'ⳬ'
    pub const SMALL_LETTER_CRYPTOGRAMMIC_SHEI: char = 'ⳬ';
    /// \u{2ced}: 'Ⳮ'
    pub const CAPITAL_LETTER_CRYPTOGRAMMIC_GANGIA: char = 'Ⳮ';
    /// \u{2cee}: 'ⳮ'
    pub const SMALL_LETTER_CRYPTOGRAMMIC_GANGIA: char = 'ⳮ';
    /// \u{2cef}: '⳯'
    pub const COMBINING_NI_ABOVE: char = '⳯';
    /// \u{2cf0}: '⳰'
    pub const COMBINING_SPIRITUS_ASPER: char = '⳰';
    /// \u{2cf1}: '⳱'
    pub const COMBINING_SPIRITUS_LENIS: char = '⳱';
    /// \u{2cf2}: 'Ⳳ'
    pub const CAPITAL_LETTER_BOHAIRIC_KHEI: char = 'Ⳳ';
    /// \u{2cf3}: 'ⳳ'
    pub const SMALL_LETTER_BOHAIRIC_KHEI: char = 'ⳳ';
    /// \u{2cf9}: '⳹'
    pub const OLD_NUBIAN_FULL_STOP: char = '⳹';
    /// \u{2cfa}: '⳺'
    pub const OLD_NUBIAN_DIRECT_QUESTION_MARK: char = '⳺';
    /// \u{2cfb}: '⳻'
    pub const OLD_NUBIAN_INDIRECT_QUESTION_MARK: char = '⳻';
    /// \u{2cfc}: '⳼'
    pub const OLD_NUBIAN_VERSE_DIVIDER: char = '⳼';
    /// \u{2cfd}: '⳽'
    pub const FRACTION_ONE_HALF: char = '⳽';
    /// \u{2cfe}: '⳾'
    pub const FULL_STOP: char = '⳾';
}

/// An enum to represent all characters in the Coptic block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Coptic {
    /// \u{2c80}: 'Ⲁ'
    CapitalLetterAlfa,
    /// \u{2c81}: 'ⲁ'
    SmallLetterAlfa,
    /// \u{2c82}: 'Ⲃ'
    CapitalLetterVida,
    /// \u{2c83}: 'ⲃ'
    SmallLetterVida,
    /// \u{2c84}: 'Ⲅ'
    CapitalLetterGamma,
    /// \u{2c85}: 'ⲅ'
    SmallLetterGamma,
    /// \u{2c86}: 'Ⲇ'
    CapitalLetterDalda,
    /// \u{2c87}: 'ⲇ'
    SmallLetterDalda,
    /// \u{2c88}: 'Ⲉ'
    CapitalLetterEie,
    /// \u{2c89}: 'ⲉ'
    SmallLetterEie,
    /// \u{2c8a}: 'Ⲋ'
    CapitalLetterSou,
    /// \u{2c8b}: 'ⲋ'
    SmallLetterSou,
    /// \u{2c8c}: 'Ⲍ'
    CapitalLetterZata,
    /// \u{2c8d}: 'ⲍ'
    SmallLetterZata,
    /// \u{2c8e}: 'Ⲏ'
    CapitalLetterHate,
    /// \u{2c8f}: 'ⲏ'
    SmallLetterHate,
    /// \u{2c90}: 'Ⲑ'
    CapitalLetterThethe,
    /// \u{2c91}: 'ⲑ'
    SmallLetterThethe,
    /// \u{2c92}: 'Ⲓ'
    CapitalLetterIauda,
    /// \u{2c93}: 'ⲓ'
    SmallLetterIauda,
    /// \u{2c94}: 'Ⲕ'
    CapitalLetterKapa,
    /// \u{2c95}: 'ⲕ'
    SmallLetterKapa,
    /// \u{2c96}: 'Ⲗ'
    CapitalLetterLaula,
    /// \u{2c97}: 'ⲗ'
    SmallLetterLaula,
    /// \u{2c98}: 'Ⲙ'
    CapitalLetterMi,
    /// \u{2c99}: 'ⲙ'
    SmallLetterMi,
    /// \u{2c9a}: 'Ⲛ'
    CapitalLetterNi,
    /// \u{2c9b}: 'ⲛ'
    SmallLetterNi,
    /// \u{2c9c}: 'Ⲝ'
    CapitalLetterKsi,
    /// \u{2c9d}: 'ⲝ'
    SmallLetterKsi,
    /// \u{2c9e}: 'Ⲟ'
    CapitalLetterO,
    /// \u{2c9f}: 'ⲟ'
    SmallLetterO,
    /// \u{2ca0}: 'Ⲡ'
    CapitalLetterPi,
    /// \u{2ca1}: 'ⲡ'
    SmallLetterPi,
    /// \u{2ca2}: 'Ⲣ'
    CapitalLetterRo,
    /// \u{2ca3}: 'ⲣ'
    SmallLetterRo,
    /// \u{2ca4}: 'Ⲥ'
    CapitalLetterSima,
    /// \u{2ca5}: 'ⲥ'
    SmallLetterSima,
    /// \u{2ca6}: 'Ⲧ'
    CapitalLetterTau,
    /// \u{2ca7}: 'ⲧ'
    SmallLetterTau,
    /// \u{2ca8}: 'Ⲩ'
    CapitalLetterUa,
    /// \u{2ca9}: 'ⲩ'
    SmallLetterUa,
    /// \u{2caa}: 'Ⲫ'
    CapitalLetterFi,
    /// \u{2cab}: 'ⲫ'
    SmallLetterFi,
    /// \u{2cac}: 'Ⲭ'
    CapitalLetterKhi,
    /// \u{2cad}: 'ⲭ'
    SmallLetterKhi,
    /// \u{2cae}: 'Ⲯ'
    CapitalLetterPsi,
    /// \u{2caf}: 'ⲯ'
    SmallLetterPsi,
    /// \u{2cb0}: 'Ⲱ'
    CapitalLetterOou,
    /// \u{2cb1}: 'ⲱ'
    SmallLetterOou,
    /// \u{2cb2}: 'Ⲳ'
    CapitalLetterDialectDashPAlef,
    /// \u{2cb3}: 'ⲳ'
    SmallLetterDialectDashPAlef,
    /// \u{2cb4}: 'Ⲵ'
    CapitalLetterOldAin,
    /// \u{2cb5}: 'ⲵ'
    SmallLetterOldAin,
    /// \u{2cb6}: 'Ⲷ'
    CapitalLetterCryptogrammicEie,
    /// \u{2cb7}: 'ⲷ'
    SmallLetterCryptogrammicEie,
    /// \u{2cb8}: 'Ⲹ'
    CapitalLetterDialectDashPKapa,
    /// \u{2cb9}: 'ⲹ'
    SmallLetterDialectDashPKapa,
    /// \u{2cba}: 'Ⲻ'
    CapitalLetterDialectDashPNi,
    /// \u{2cbb}: 'ⲻ'
    SmallLetterDialectDashPNi,
    /// \u{2cbc}: 'Ⲽ'
    CapitalLetterCryptogrammicNi,
    /// \u{2cbd}: 'ⲽ'
    SmallLetterCryptogrammicNi,
    /// \u{2cbe}: 'Ⲿ'
    CapitalLetterOldOou,
    /// \u{2cbf}: 'ⲿ'
    SmallLetterOldOou,
    /// \u{2cc0}: 'Ⳁ'
    CapitalLetterSampi,
    /// \u{2cc1}: 'ⳁ'
    SmallLetterSampi,
    /// \u{2cc2}: 'Ⳃ'
    CapitalLetterCrossedShei,
    /// \u{2cc3}: 'ⳃ'
    SmallLetterCrossedShei,
    /// \u{2cc4}: 'Ⳅ'
    CapitalLetterOldShei,
    /// \u{2cc5}: 'ⳅ'
    SmallLetterOldShei,
    /// \u{2cc6}: 'Ⳇ'
    CapitalLetterOldEsh,
    /// \u{2cc7}: 'ⳇ'
    SmallLetterOldEsh,
    /// \u{2cc8}: 'Ⳉ'
    CapitalLetterAkhmimicKhei,
    /// \u{2cc9}: 'ⳉ'
    SmallLetterAkhmimicKhei,
    /// \u{2cca}: 'Ⳋ'
    CapitalLetterDialectDashPHori,
    /// \u{2ccb}: 'ⳋ'
    SmallLetterDialectDashPHori,
    /// \u{2ccc}: 'Ⳍ'
    CapitalLetterOldHori,
    /// \u{2ccd}: 'ⳍ'
    SmallLetterOldHori,
    /// \u{2cce}: 'Ⳏ'
    CapitalLetterOldHa,
    /// \u{2ccf}: 'ⳏ'
    SmallLetterOldHa,
    /// \u{2cd0}: 'Ⳑ'
    CapitalLetterLDashShapedHa,
    /// \u{2cd1}: 'ⳑ'
    SmallLetterLDashShapedHa,
    /// \u{2cd2}: 'Ⳓ'
    CapitalLetterOldHei,
    /// \u{2cd3}: 'ⳓ'
    SmallLetterOldHei,
    /// \u{2cd4}: 'Ⳕ'
    CapitalLetterOldHat,
    /// \u{2cd5}: 'ⳕ'
    SmallLetterOldHat,
    /// \u{2cd6}: 'Ⳗ'
    CapitalLetterOldGangia,
    /// \u{2cd7}: 'ⳗ'
    SmallLetterOldGangia,
    /// \u{2cd8}: 'Ⳙ'
    CapitalLetterOldDja,
    /// \u{2cd9}: 'ⳙ'
    SmallLetterOldDja,
    /// \u{2cda}: 'Ⳛ'
    CapitalLetterOldShima,
    /// \u{2cdb}: 'ⳛ'
    SmallLetterOldShima,
    /// \u{2cdc}: 'Ⳝ'
    CapitalLetterOldNubianShima,
    /// \u{2cdd}: 'ⳝ'
    SmallLetterOldNubianShima,
    /// \u{2cde}: 'Ⳟ'
    CapitalLetterOldNubianNgi,
    /// \u{2cdf}: 'ⳟ'
    SmallLetterOldNubianNgi,
    /// \u{2ce0}: 'Ⳡ'
    CapitalLetterOldNubianNyi,
    /// \u{2ce1}: 'ⳡ'
    SmallLetterOldNubianNyi,
    /// \u{2ce2}: 'Ⳣ'
    CapitalLetterOldNubianWau,
    /// \u{2ce3}: 'ⳣ'
    SmallLetterOldNubianWau,
    /// \u{2ce4}: 'ⳤ'
    SymbolKai,
    /// \u{2ce5}: '⳥'
    SymbolMiRo,
    /// \u{2ce6}: '⳦'
    SymbolPiRo,
    /// \u{2ce7}: '⳧'
    SymbolStauros,
    /// \u{2ce8}: '⳨'
    SymbolTauRo,
    /// \u{2ce9}: '⳩'
    SymbolKhiRo,
    /// \u{2cea}: '⳪'
    SymbolShimaSima,
    /// \u{2ceb}: 'Ⳬ'
    CapitalLetterCryptogrammicShei,
    /// \u{2cec}: 'ⳬ'
    SmallLetterCryptogrammicShei,
    /// \u{2ced}: 'Ⳮ'
    CapitalLetterCryptogrammicGangia,
    /// \u{2cee}: 'ⳮ'
    SmallLetterCryptogrammicGangia,
    /// \u{2cef}: '⳯'
    CombiningNiAbove,
    /// \u{2cf0}: '⳰'
    CombiningSpiritusAsper,
    /// \u{2cf1}: '⳱'
    CombiningSpiritusLenis,
    /// \u{2cf2}: 'Ⳳ'
    CapitalLetterBohairicKhei,
    /// \u{2cf3}: 'ⳳ'
    SmallLetterBohairicKhei,
    /// \u{2cf9}: '⳹'
    OldNubianFullStop,
    /// \u{2cfa}: '⳺'
    OldNubianDirectQuestionMark,
    /// \u{2cfb}: '⳻'
    OldNubianIndirectQuestionMark,
    /// \u{2cfc}: '⳼'
    OldNubianVerseDivider,
    /// \u{2cfd}: '⳽'
    FractionOneHalf,
    /// \u{2cfe}: '⳾'
    FullStop,
}

impl Into<char> for Coptic {
    fn into(self) -> char {
        use constants::*;
        match self {
            Coptic::CapitalLetterAlfa => CAPITAL_LETTER_ALFA,
            Coptic::SmallLetterAlfa => SMALL_LETTER_ALFA,
            Coptic::CapitalLetterVida => CAPITAL_LETTER_VIDA,
            Coptic::SmallLetterVida => SMALL_LETTER_VIDA,
            Coptic::CapitalLetterGamma => CAPITAL_LETTER_GAMMA,
            Coptic::SmallLetterGamma => SMALL_LETTER_GAMMA,
            Coptic::CapitalLetterDalda => CAPITAL_LETTER_DALDA,
            Coptic::SmallLetterDalda => SMALL_LETTER_DALDA,
            Coptic::CapitalLetterEie => CAPITAL_LETTER_EIE,
            Coptic::SmallLetterEie => SMALL_LETTER_EIE,
            Coptic::CapitalLetterSou => CAPITAL_LETTER_SOU,
            Coptic::SmallLetterSou => SMALL_LETTER_SOU,
            Coptic::CapitalLetterZata => CAPITAL_LETTER_ZATA,
            Coptic::SmallLetterZata => SMALL_LETTER_ZATA,
            Coptic::CapitalLetterHate => CAPITAL_LETTER_HATE,
            Coptic::SmallLetterHate => SMALL_LETTER_HATE,
            Coptic::CapitalLetterThethe => CAPITAL_LETTER_THETHE,
            Coptic::SmallLetterThethe => SMALL_LETTER_THETHE,
            Coptic::CapitalLetterIauda => CAPITAL_LETTER_IAUDA,
            Coptic::SmallLetterIauda => SMALL_LETTER_IAUDA,
            Coptic::CapitalLetterKapa => CAPITAL_LETTER_KAPA,
            Coptic::SmallLetterKapa => SMALL_LETTER_KAPA,
            Coptic::CapitalLetterLaula => CAPITAL_LETTER_LAULA,
            Coptic::SmallLetterLaula => SMALL_LETTER_LAULA,
            Coptic::CapitalLetterMi => CAPITAL_LETTER_MI,
            Coptic::SmallLetterMi => SMALL_LETTER_MI,
            Coptic::CapitalLetterNi => CAPITAL_LETTER_NI,
            Coptic::SmallLetterNi => SMALL_LETTER_NI,
            Coptic::CapitalLetterKsi => CAPITAL_LETTER_KSI,
            Coptic::SmallLetterKsi => SMALL_LETTER_KSI,
            Coptic::CapitalLetterO => CAPITAL_LETTER_O,
            Coptic::SmallLetterO => SMALL_LETTER_O,
            Coptic::CapitalLetterPi => CAPITAL_LETTER_PI,
            Coptic::SmallLetterPi => SMALL_LETTER_PI,
            Coptic::CapitalLetterRo => CAPITAL_LETTER_RO,
            Coptic::SmallLetterRo => SMALL_LETTER_RO,
            Coptic::CapitalLetterSima => CAPITAL_LETTER_SIMA,
            Coptic::SmallLetterSima => SMALL_LETTER_SIMA,
            Coptic::CapitalLetterTau => CAPITAL_LETTER_TAU,
            Coptic::SmallLetterTau => SMALL_LETTER_TAU,
            Coptic::CapitalLetterUa => CAPITAL_LETTER_UA,
            Coptic::SmallLetterUa => SMALL_LETTER_UA,
            Coptic::CapitalLetterFi => CAPITAL_LETTER_FI,
            Coptic::SmallLetterFi => SMALL_LETTER_FI,
            Coptic::CapitalLetterKhi => CAPITAL_LETTER_KHI,
            Coptic::SmallLetterKhi => SMALL_LETTER_KHI,
            Coptic::CapitalLetterPsi => CAPITAL_LETTER_PSI,
            Coptic::SmallLetterPsi => SMALL_LETTER_PSI,
            Coptic::CapitalLetterOou => CAPITAL_LETTER_OOU,
            Coptic::SmallLetterOou => SMALL_LETTER_OOU,
            Coptic::CapitalLetterDialectDashPAlef => CAPITAL_LETTER_DIALECT_DASH_P_ALEF,
            Coptic::SmallLetterDialectDashPAlef => SMALL_LETTER_DIALECT_DASH_P_ALEF,
            Coptic::CapitalLetterOldAin => CAPITAL_LETTER_OLD_AIN,
            Coptic::SmallLetterOldAin => SMALL_LETTER_OLD_AIN,
            Coptic::CapitalLetterCryptogrammicEie => CAPITAL_LETTER_CRYPTOGRAMMIC_EIE,
            Coptic::SmallLetterCryptogrammicEie => SMALL_LETTER_CRYPTOGRAMMIC_EIE,
            Coptic::CapitalLetterDialectDashPKapa => CAPITAL_LETTER_DIALECT_DASH_P_KAPA,
            Coptic::SmallLetterDialectDashPKapa => SMALL_LETTER_DIALECT_DASH_P_KAPA,
            Coptic::CapitalLetterDialectDashPNi => CAPITAL_LETTER_DIALECT_DASH_P_NI,
            Coptic::SmallLetterDialectDashPNi => SMALL_LETTER_DIALECT_DASH_P_NI,
            Coptic::CapitalLetterCryptogrammicNi => CAPITAL_LETTER_CRYPTOGRAMMIC_NI,
            Coptic::SmallLetterCryptogrammicNi => SMALL_LETTER_CRYPTOGRAMMIC_NI,
            Coptic::CapitalLetterOldOou => CAPITAL_LETTER_OLD_OOU,
            Coptic::SmallLetterOldOou => SMALL_LETTER_OLD_OOU,
            Coptic::CapitalLetterSampi => CAPITAL_LETTER_SAMPI,
            Coptic::SmallLetterSampi => SMALL_LETTER_SAMPI,
            Coptic::CapitalLetterCrossedShei => CAPITAL_LETTER_CROSSED_SHEI,
            Coptic::SmallLetterCrossedShei => SMALL_LETTER_CROSSED_SHEI,
            Coptic::CapitalLetterOldShei => CAPITAL_LETTER_OLD_SHEI,
            Coptic::SmallLetterOldShei => SMALL_LETTER_OLD_SHEI,
            Coptic::CapitalLetterOldEsh => CAPITAL_LETTER_OLD_ESH,
            Coptic::SmallLetterOldEsh => SMALL_LETTER_OLD_ESH,
            Coptic::CapitalLetterAkhmimicKhei => CAPITAL_LETTER_AKHMIMIC_KHEI,
            Coptic::SmallLetterAkhmimicKhei => SMALL_LETTER_AKHMIMIC_KHEI,
            Coptic::CapitalLetterDialectDashPHori => CAPITAL_LETTER_DIALECT_DASH_P_HORI,
            Coptic::SmallLetterDialectDashPHori => SMALL_LETTER_DIALECT_DASH_P_HORI,
            Coptic::CapitalLetterOldHori => CAPITAL_LETTER_OLD_HORI,
            Coptic::SmallLetterOldHori => SMALL_LETTER_OLD_HORI,
            Coptic::CapitalLetterOldHa => CAPITAL_LETTER_OLD_HA,
            Coptic::SmallLetterOldHa => SMALL_LETTER_OLD_HA,
            Coptic::CapitalLetterLDashShapedHa => CAPITAL_LETTER_L_DASH_SHAPED_HA,
            Coptic::SmallLetterLDashShapedHa => SMALL_LETTER_L_DASH_SHAPED_HA,
            Coptic::CapitalLetterOldHei => CAPITAL_LETTER_OLD_HEI,
            Coptic::SmallLetterOldHei => SMALL_LETTER_OLD_HEI,
            Coptic::CapitalLetterOldHat => CAPITAL_LETTER_OLD_HAT,
            Coptic::SmallLetterOldHat => SMALL_LETTER_OLD_HAT,
            Coptic::CapitalLetterOldGangia => CAPITAL_LETTER_OLD_GANGIA,
            Coptic::SmallLetterOldGangia => SMALL_LETTER_OLD_GANGIA,
            Coptic::CapitalLetterOldDja => CAPITAL_LETTER_OLD_DJA,
            Coptic::SmallLetterOldDja => SMALL_LETTER_OLD_DJA,
            Coptic::CapitalLetterOldShima => CAPITAL_LETTER_OLD_SHIMA,
            Coptic::SmallLetterOldShima => SMALL_LETTER_OLD_SHIMA,
            Coptic::CapitalLetterOldNubianShima => CAPITAL_LETTER_OLD_NUBIAN_SHIMA,
            Coptic::SmallLetterOldNubianShima => SMALL_LETTER_OLD_NUBIAN_SHIMA,
            Coptic::CapitalLetterOldNubianNgi => CAPITAL_LETTER_OLD_NUBIAN_NGI,
            Coptic::SmallLetterOldNubianNgi => SMALL_LETTER_OLD_NUBIAN_NGI,
            Coptic::CapitalLetterOldNubianNyi => CAPITAL_LETTER_OLD_NUBIAN_NYI,
            Coptic::SmallLetterOldNubianNyi => SMALL_LETTER_OLD_NUBIAN_NYI,
            Coptic::CapitalLetterOldNubianWau => CAPITAL_LETTER_OLD_NUBIAN_WAU,
            Coptic::SmallLetterOldNubianWau => SMALL_LETTER_OLD_NUBIAN_WAU,
            Coptic::SymbolKai => SYMBOL_KAI,
            Coptic::SymbolMiRo => SYMBOL_MI_RO,
            Coptic::SymbolPiRo => SYMBOL_PI_RO,
            Coptic::SymbolStauros => SYMBOL_STAUROS,
            Coptic::SymbolTauRo => SYMBOL_TAU_RO,
            Coptic::SymbolKhiRo => SYMBOL_KHI_RO,
            Coptic::SymbolShimaSima => SYMBOL_SHIMA_SIMA,
            Coptic::CapitalLetterCryptogrammicShei => CAPITAL_LETTER_CRYPTOGRAMMIC_SHEI,
            Coptic::SmallLetterCryptogrammicShei => SMALL_LETTER_CRYPTOGRAMMIC_SHEI,
            Coptic::CapitalLetterCryptogrammicGangia => CAPITAL_LETTER_CRYPTOGRAMMIC_GANGIA,
            Coptic::SmallLetterCryptogrammicGangia => SMALL_LETTER_CRYPTOGRAMMIC_GANGIA,
            Coptic::CombiningNiAbove => COMBINING_NI_ABOVE,
            Coptic::CombiningSpiritusAsper => COMBINING_SPIRITUS_ASPER,
            Coptic::CombiningSpiritusLenis => COMBINING_SPIRITUS_LENIS,
            Coptic::CapitalLetterBohairicKhei => CAPITAL_LETTER_BOHAIRIC_KHEI,
            Coptic::SmallLetterBohairicKhei => SMALL_LETTER_BOHAIRIC_KHEI,
            Coptic::OldNubianFullStop => OLD_NUBIAN_FULL_STOP,
            Coptic::OldNubianDirectQuestionMark => OLD_NUBIAN_DIRECT_QUESTION_MARK,
            Coptic::OldNubianIndirectQuestionMark => OLD_NUBIAN_INDIRECT_QUESTION_MARK,
            Coptic::OldNubianVerseDivider => OLD_NUBIAN_VERSE_DIVIDER,
            Coptic::FractionOneHalf => FRACTION_ONE_HALF,
            Coptic::FullStop => FULL_STOP,
        }
    }
}

impl std::convert::TryFrom<char> for Coptic {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            CAPITAL_LETTER_ALFA => Ok(Coptic::CapitalLetterAlfa),
            SMALL_LETTER_ALFA => Ok(Coptic::SmallLetterAlfa),
            CAPITAL_LETTER_VIDA => Ok(Coptic::CapitalLetterVida),
            SMALL_LETTER_VIDA => Ok(Coptic::SmallLetterVida),
            CAPITAL_LETTER_GAMMA => Ok(Coptic::CapitalLetterGamma),
            SMALL_LETTER_GAMMA => Ok(Coptic::SmallLetterGamma),
            CAPITAL_LETTER_DALDA => Ok(Coptic::CapitalLetterDalda),
            SMALL_LETTER_DALDA => Ok(Coptic::SmallLetterDalda),
            CAPITAL_LETTER_EIE => Ok(Coptic::CapitalLetterEie),
            SMALL_LETTER_EIE => Ok(Coptic::SmallLetterEie),
            CAPITAL_LETTER_SOU => Ok(Coptic::CapitalLetterSou),
            SMALL_LETTER_SOU => Ok(Coptic::SmallLetterSou),
            CAPITAL_LETTER_ZATA => Ok(Coptic::CapitalLetterZata),
            SMALL_LETTER_ZATA => Ok(Coptic::SmallLetterZata),
            CAPITAL_LETTER_HATE => Ok(Coptic::CapitalLetterHate),
            SMALL_LETTER_HATE => Ok(Coptic::SmallLetterHate),
            CAPITAL_LETTER_THETHE => Ok(Coptic::CapitalLetterThethe),
            SMALL_LETTER_THETHE => Ok(Coptic::SmallLetterThethe),
            CAPITAL_LETTER_IAUDA => Ok(Coptic::CapitalLetterIauda),
            SMALL_LETTER_IAUDA => Ok(Coptic::SmallLetterIauda),
            CAPITAL_LETTER_KAPA => Ok(Coptic::CapitalLetterKapa),
            SMALL_LETTER_KAPA => Ok(Coptic::SmallLetterKapa),
            CAPITAL_LETTER_LAULA => Ok(Coptic::CapitalLetterLaula),
            SMALL_LETTER_LAULA => Ok(Coptic::SmallLetterLaula),
            CAPITAL_LETTER_MI => Ok(Coptic::CapitalLetterMi),
            SMALL_LETTER_MI => Ok(Coptic::SmallLetterMi),
            CAPITAL_LETTER_NI => Ok(Coptic::CapitalLetterNi),
            SMALL_LETTER_NI => Ok(Coptic::SmallLetterNi),
            CAPITAL_LETTER_KSI => Ok(Coptic::CapitalLetterKsi),
            SMALL_LETTER_KSI => Ok(Coptic::SmallLetterKsi),
            CAPITAL_LETTER_O => Ok(Coptic::CapitalLetterO),
            SMALL_LETTER_O => Ok(Coptic::SmallLetterO),
            CAPITAL_LETTER_PI => Ok(Coptic::CapitalLetterPi),
            SMALL_LETTER_PI => Ok(Coptic::SmallLetterPi),
            CAPITAL_LETTER_RO => Ok(Coptic::CapitalLetterRo),
            SMALL_LETTER_RO => Ok(Coptic::SmallLetterRo),
            CAPITAL_LETTER_SIMA => Ok(Coptic::CapitalLetterSima),
            SMALL_LETTER_SIMA => Ok(Coptic::SmallLetterSima),
            CAPITAL_LETTER_TAU => Ok(Coptic::CapitalLetterTau),
            SMALL_LETTER_TAU => Ok(Coptic::SmallLetterTau),
            CAPITAL_LETTER_UA => Ok(Coptic::CapitalLetterUa),
            SMALL_LETTER_UA => Ok(Coptic::SmallLetterUa),
            CAPITAL_LETTER_FI => Ok(Coptic::CapitalLetterFi),
            SMALL_LETTER_FI => Ok(Coptic::SmallLetterFi),
            CAPITAL_LETTER_KHI => Ok(Coptic::CapitalLetterKhi),
            SMALL_LETTER_KHI => Ok(Coptic::SmallLetterKhi),
            CAPITAL_LETTER_PSI => Ok(Coptic::CapitalLetterPsi),
            SMALL_LETTER_PSI => Ok(Coptic::SmallLetterPsi),
            CAPITAL_LETTER_OOU => Ok(Coptic::CapitalLetterOou),
            SMALL_LETTER_OOU => Ok(Coptic::SmallLetterOou),
            CAPITAL_LETTER_DIALECT_DASH_P_ALEF => Ok(Coptic::CapitalLetterDialectDashPAlef),
            SMALL_LETTER_DIALECT_DASH_P_ALEF => Ok(Coptic::SmallLetterDialectDashPAlef),
            CAPITAL_LETTER_OLD_AIN => Ok(Coptic::CapitalLetterOldAin),
            SMALL_LETTER_OLD_AIN => Ok(Coptic::SmallLetterOldAin),
            CAPITAL_LETTER_CRYPTOGRAMMIC_EIE => Ok(Coptic::CapitalLetterCryptogrammicEie),
            SMALL_LETTER_CRYPTOGRAMMIC_EIE => Ok(Coptic::SmallLetterCryptogrammicEie),
            CAPITAL_LETTER_DIALECT_DASH_P_KAPA => Ok(Coptic::CapitalLetterDialectDashPKapa),
            SMALL_LETTER_DIALECT_DASH_P_KAPA => Ok(Coptic::SmallLetterDialectDashPKapa),
            CAPITAL_LETTER_DIALECT_DASH_P_NI => Ok(Coptic::CapitalLetterDialectDashPNi),
            SMALL_LETTER_DIALECT_DASH_P_NI => Ok(Coptic::SmallLetterDialectDashPNi),
            CAPITAL_LETTER_CRYPTOGRAMMIC_NI => Ok(Coptic::CapitalLetterCryptogrammicNi),
            SMALL_LETTER_CRYPTOGRAMMIC_NI => Ok(Coptic::SmallLetterCryptogrammicNi),
            CAPITAL_LETTER_OLD_OOU => Ok(Coptic::CapitalLetterOldOou),
            SMALL_LETTER_OLD_OOU => Ok(Coptic::SmallLetterOldOou),
            CAPITAL_LETTER_SAMPI => Ok(Coptic::CapitalLetterSampi),
            SMALL_LETTER_SAMPI => Ok(Coptic::SmallLetterSampi),
            CAPITAL_LETTER_CROSSED_SHEI => Ok(Coptic::CapitalLetterCrossedShei),
            SMALL_LETTER_CROSSED_SHEI => Ok(Coptic::SmallLetterCrossedShei),
            CAPITAL_LETTER_OLD_SHEI => Ok(Coptic::CapitalLetterOldShei),
            SMALL_LETTER_OLD_SHEI => Ok(Coptic::SmallLetterOldShei),
            CAPITAL_LETTER_OLD_ESH => Ok(Coptic::CapitalLetterOldEsh),
            SMALL_LETTER_OLD_ESH => Ok(Coptic::SmallLetterOldEsh),
            CAPITAL_LETTER_AKHMIMIC_KHEI => Ok(Coptic::CapitalLetterAkhmimicKhei),
            SMALL_LETTER_AKHMIMIC_KHEI => Ok(Coptic::SmallLetterAkhmimicKhei),
            CAPITAL_LETTER_DIALECT_DASH_P_HORI => Ok(Coptic::CapitalLetterDialectDashPHori),
            SMALL_LETTER_DIALECT_DASH_P_HORI => Ok(Coptic::SmallLetterDialectDashPHori),
            CAPITAL_LETTER_OLD_HORI => Ok(Coptic::CapitalLetterOldHori),
            SMALL_LETTER_OLD_HORI => Ok(Coptic::SmallLetterOldHori),
            CAPITAL_LETTER_OLD_HA => Ok(Coptic::CapitalLetterOldHa),
            SMALL_LETTER_OLD_HA => Ok(Coptic::SmallLetterOldHa),
            CAPITAL_LETTER_L_DASH_SHAPED_HA => Ok(Coptic::CapitalLetterLDashShapedHa),
            SMALL_LETTER_L_DASH_SHAPED_HA => Ok(Coptic::SmallLetterLDashShapedHa),
            CAPITAL_LETTER_OLD_HEI => Ok(Coptic::CapitalLetterOldHei),
            SMALL_LETTER_OLD_HEI => Ok(Coptic::SmallLetterOldHei),
            CAPITAL_LETTER_OLD_HAT => Ok(Coptic::CapitalLetterOldHat),
            SMALL_LETTER_OLD_HAT => Ok(Coptic::SmallLetterOldHat),
            CAPITAL_LETTER_OLD_GANGIA => Ok(Coptic::CapitalLetterOldGangia),
            SMALL_LETTER_OLD_GANGIA => Ok(Coptic::SmallLetterOldGangia),
            CAPITAL_LETTER_OLD_DJA => Ok(Coptic::CapitalLetterOldDja),
            SMALL_LETTER_OLD_DJA => Ok(Coptic::SmallLetterOldDja),
            CAPITAL_LETTER_OLD_SHIMA => Ok(Coptic::CapitalLetterOldShima),
            SMALL_LETTER_OLD_SHIMA => Ok(Coptic::SmallLetterOldShima),
            CAPITAL_LETTER_OLD_NUBIAN_SHIMA => Ok(Coptic::CapitalLetterOldNubianShima),
            SMALL_LETTER_OLD_NUBIAN_SHIMA => Ok(Coptic::SmallLetterOldNubianShima),
            CAPITAL_LETTER_OLD_NUBIAN_NGI => Ok(Coptic::CapitalLetterOldNubianNgi),
            SMALL_LETTER_OLD_NUBIAN_NGI => Ok(Coptic::SmallLetterOldNubianNgi),
            CAPITAL_LETTER_OLD_NUBIAN_NYI => Ok(Coptic::CapitalLetterOldNubianNyi),
            SMALL_LETTER_OLD_NUBIAN_NYI => Ok(Coptic::SmallLetterOldNubianNyi),
            CAPITAL_LETTER_OLD_NUBIAN_WAU => Ok(Coptic::CapitalLetterOldNubianWau),
            SMALL_LETTER_OLD_NUBIAN_WAU => Ok(Coptic::SmallLetterOldNubianWau),
            SYMBOL_KAI => Ok(Coptic::SymbolKai),
            SYMBOL_MI_RO => Ok(Coptic::SymbolMiRo),
            SYMBOL_PI_RO => Ok(Coptic::SymbolPiRo),
            SYMBOL_STAUROS => Ok(Coptic::SymbolStauros),
            SYMBOL_TAU_RO => Ok(Coptic::SymbolTauRo),
            SYMBOL_KHI_RO => Ok(Coptic::SymbolKhiRo),
            SYMBOL_SHIMA_SIMA => Ok(Coptic::SymbolShimaSima),
            CAPITAL_LETTER_CRYPTOGRAMMIC_SHEI => Ok(Coptic::CapitalLetterCryptogrammicShei),
            SMALL_LETTER_CRYPTOGRAMMIC_SHEI => Ok(Coptic::SmallLetterCryptogrammicShei),
            CAPITAL_LETTER_CRYPTOGRAMMIC_GANGIA => Ok(Coptic::CapitalLetterCryptogrammicGangia),
            SMALL_LETTER_CRYPTOGRAMMIC_GANGIA => Ok(Coptic::SmallLetterCryptogrammicGangia),
            COMBINING_NI_ABOVE => Ok(Coptic::CombiningNiAbove),
            COMBINING_SPIRITUS_ASPER => Ok(Coptic::CombiningSpiritusAsper),
            COMBINING_SPIRITUS_LENIS => Ok(Coptic::CombiningSpiritusLenis),
            CAPITAL_LETTER_BOHAIRIC_KHEI => Ok(Coptic::CapitalLetterBohairicKhei),
            SMALL_LETTER_BOHAIRIC_KHEI => Ok(Coptic::SmallLetterBohairicKhei),
            OLD_NUBIAN_FULL_STOP => Ok(Coptic::OldNubianFullStop),
            OLD_NUBIAN_DIRECT_QUESTION_MARK => Ok(Coptic::OldNubianDirectQuestionMark),
            OLD_NUBIAN_INDIRECT_QUESTION_MARK => Ok(Coptic::OldNubianIndirectQuestionMark),
            OLD_NUBIAN_VERSE_DIVIDER => Ok(Coptic::OldNubianVerseDivider),
            FRACTION_ONE_HALF => Ok(Coptic::FractionOneHalf),
            FULL_STOP => Ok(Coptic::FullStop),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Coptic {
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

impl std::convert::TryFrom<u32> for Coptic {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Coptic {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Coptic {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Coptic::CapitalLetterAlfa
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Coptic::CapitalLetterAlfa => "coptic capital letter alfa",
            Coptic::SmallLetterAlfa => "coptic small letter alfa",
            Coptic::CapitalLetterVida => "coptic capital letter vida",
            Coptic::SmallLetterVida => "coptic small letter vida",
            Coptic::CapitalLetterGamma => "coptic capital letter gamma",
            Coptic::SmallLetterGamma => "coptic small letter gamma",
            Coptic::CapitalLetterDalda => "coptic capital letter dalda",
            Coptic::SmallLetterDalda => "coptic small letter dalda",
            Coptic::CapitalLetterEie => "coptic capital letter eie",
            Coptic::SmallLetterEie => "coptic small letter eie",
            Coptic::CapitalLetterSou => "coptic capital letter sou",
            Coptic::SmallLetterSou => "coptic small letter sou",
            Coptic::CapitalLetterZata => "coptic capital letter zata",
            Coptic::SmallLetterZata => "coptic small letter zata",
            Coptic::CapitalLetterHate => "coptic capital letter hate",
            Coptic::SmallLetterHate => "coptic small letter hate",
            Coptic::CapitalLetterThethe => "coptic capital letter thethe",
            Coptic::SmallLetterThethe => "coptic small letter thethe",
            Coptic::CapitalLetterIauda => "coptic capital letter iauda",
            Coptic::SmallLetterIauda => "coptic small letter iauda",
            Coptic::CapitalLetterKapa => "coptic capital letter kapa",
            Coptic::SmallLetterKapa => "coptic small letter kapa",
            Coptic::CapitalLetterLaula => "coptic capital letter laula",
            Coptic::SmallLetterLaula => "coptic small letter laula",
            Coptic::CapitalLetterMi => "coptic capital letter mi",
            Coptic::SmallLetterMi => "coptic small letter mi",
            Coptic::CapitalLetterNi => "coptic capital letter ni",
            Coptic::SmallLetterNi => "coptic small letter ni",
            Coptic::CapitalLetterKsi => "coptic capital letter ksi",
            Coptic::SmallLetterKsi => "coptic small letter ksi",
            Coptic::CapitalLetterO => "coptic capital letter o",
            Coptic::SmallLetterO => "coptic small letter o",
            Coptic::CapitalLetterPi => "coptic capital letter pi",
            Coptic::SmallLetterPi => "coptic small letter pi",
            Coptic::CapitalLetterRo => "coptic capital letter ro",
            Coptic::SmallLetterRo => "coptic small letter ro",
            Coptic::CapitalLetterSima => "coptic capital letter sima",
            Coptic::SmallLetterSima => "coptic small letter sima",
            Coptic::CapitalLetterTau => "coptic capital letter tau",
            Coptic::SmallLetterTau => "coptic small letter tau",
            Coptic::CapitalLetterUa => "coptic capital letter ua",
            Coptic::SmallLetterUa => "coptic small letter ua",
            Coptic::CapitalLetterFi => "coptic capital letter fi",
            Coptic::SmallLetterFi => "coptic small letter fi",
            Coptic::CapitalLetterKhi => "coptic capital letter khi",
            Coptic::SmallLetterKhi => "coptic small letter khi",
            Coptic::CapitalLetterPsi => "coptic capital letter psi",
            Coptic::SmallLetterPsi => "coptic small letter psi",
            Coptic::CapitalLetterOou => "coptic capital letter oou",
            Coptic::SmallLetterOou => "coptic small letter oou",
            Coptic::CapitalLetterDialectDashPAlef => "coptic capital letter dialect-p alef",
            Coptic::SmallLetterDialectDashPAlef => "coptic small letter dialect-p alef",
            Coptic::CapitalLetterOldAin => "coptic capital letter old coptic ain",
            Coptic::SmallLetterOldAin => "coptic small letter old coptic ain",
            Coptic::CapitalLetterCryptogrammicEie => "coptic capital letter cryptogrammic eie",
            Coptic::SmallLetterCryptogrammicEie => "coptic small letter cryptogrammic eie",
            Coptic::CapitalLetterDialectDashPKapa => "coptic capital letter dialect-p kapa",
            Coptic::SmallLetterDialectDashPKapa => "coptic small letter dialect-p kapa",
            Coptic::CapitalLetterDialectDashPNi => "coptic capital letter dialect-p ni",
            Coptic::SmallLetterDialectDashPNi => "coptic small letter dialect-p ni",
            Coptic::CapitalLetterCryptogrammicNi => "coptic capital letter cryptogrammic ni",
            Coptic::SmallLetterCryptogrammicNi => "coptic small letter cryptogrammic ni",
            Coptic::CapitalLetterOldOou => "coptic capital letter old coptic oou",
            Coptic::SmallLetterOldOou => "coptic small letter old coptic oou",
            Coptic::CapitalLetterSampi => "coptic capital letter sampi",
            Coptic::SmallLetterSampi => "coptic small letter sampi",
            Coptic::CapitalLetterCrossedShei => "coptic capital letter crossed shei",
            Coptic::SmallLetterCrossedShei => "coptic small letter crossed shei",
            Coptic::CapitalLetterOldShei => "coptic capital letter old coptic shei",
            Coptic::SmallLetterOldShei => "coptic small letter old coptic shei",
            Coptic::CapitalLetterOldEsh => "coptic capital letter old coptic esh",
            Coptic::SmallLetterOldEsh => "coptic small letter old coptic esh",
            Coptic::CapitalLetterAkhmimicKhei => "coptic capital letter akhmimic khei",
            Coptic::SmallLetterAkhmimicKhei => "coptic small letter akhmimic khei",
            Coptic::CapitalLetterDialectDashPHori => "coptic capital letter dialect-p hori",
            Coptic::SmallLetterDialectDashPHori => "coptic small letter dialect-p hori",
            Coptic::CapitalLetterOldHori => "coptic capital letter old coptic hori",
            Coptic::SmallLetterOldHori => "coptic small letter old coptic hori",
            Coptic::CapitalLetterOldHa => "coptic capital letter old coptic ha",
            Coptic::SmallLetterOldHa => "coptic small letter old coptic ha",
            Coptic::CapitalLetterLDashShapedHa => "coptic capital letter l-shaped ha",
            Coptic::SmallLetterLDashShapedHa => "coptic small letter l-shaped ha",
            Coptic::CapitalLetterOldHei => "coptic capital letter old coptic hei",
            Coptic::SmallLetterOldHei => "coptic small letter old coptic hei",
            Coptic::CapitalLetterOldHat => "coptic capital letter old coptic hat",
            Coptic::SmallLetterOldHat => "coptic small letter old coptic hat",
            Coptic::CapitalLetterOldGangia => "coptic capital letter old coptic gangia",
            Coptic::SmallLetterOldGangia => "coptic small letter old coptic gangia",
            Coptic::CapitalLetterOldDja => "coptic capital letter old coptic dja",
            Coptic::SmallLetterOldDja => "coptic small letter old coptic dja",
            Coptic::CapitalLetterOldShima => "coptic capital letter old coptic shima",
            Coptic::SmallLetterOldShima => "coptic small letter old coptic shima",
            Coptic::CapitalLetterOldNubianShima => "coptic capital letter old nubian shima",
            Coptic::SmallLetterOldNubianShima => "coptic small letter old nubian shima",
            Coptic::CapitalLetterOldNubianNgi => "coptic capital letter old nubian ngi",
            Coptic::SmallLetterOldNubianNgi => "coptic small letter old nubian ngi",
            Coptic::CapitalLetterOldNubianNyi => "coptic capital letter old nubian nyi",
            Coptic::SmallLetterOldNubianNyi => "coptic small letter old nubian nyi",
            Coptic::CapitalLetterOldNubianWau => "coptic capital letter old nubian wau",
            Coptic::SmallLetterOldNubianWau => "coptic small letter old nubian wau",
            Coptic::SymbolKai => "coptic symbol kai",
            Coptic::SymbolMiRo => "coptic symbol mi ro",
            Coptic::SymbolPiRo => "coptic symbol pi ro",
            Coptic::SymbolStauros => "coptic symbol stauros",
            Coptic::SymbolTauRo => "coptic symbol tau ro",
            Coptic::SymbolKhiRo => "coptic symbol khi ro",
            Coptic::SymbolShimaSima => "coptic symbol shima sima",
            Coptic::CapitalLetterCryptogrammicShei => "coptic capital letter cryptogrammic shei",
            Coptic::SmallLetterCryptogrammicShei => "coptic small letter cryptogrammic shei",
            Coptic::CapitalLetterCryptogrammicGangia => "coptic capital letter cryptogrammic gangia",
            Coptic::SmallLetterCryptogrammicGangia => "coptic small letter cryptogrammic gangia",
            Coptic::CombiningNiAbove => "coptic combining ni above",
            Coptic::CombiningSpiritusAsper => "coptic combining spiritus asper",
            Coptic::CombiningSpiritusLenis => "coptic combining spiritus lenis",
            Coptic::CapitalLetterBohairicKhei => "coptic capital letter bohairic khei",
            Coptic::SmallLetterBohairicKhei => "coptic small letter bohairic khei",
            Coptic::OldNubianFullStop => "coptic old nubian full stop",
            Coptic::OldNubianDirectQuestionMark => "coptic old nubian direct question mark",
            Coptic::OldNubianIndirectQuestionMark => "coptic old nubian indirect question mark",
            Coptic::OldNubianVerseDivider => "coptic old nubian verse divider",
            Coptic::FractionOneHalf => "coptic fraction one half",
            Coptic::FullStop => "coptic full stop",
        }
    }
}
