/// \u{1cd0} → \u{1cff}
///
/// ᳐ ᳑ ᳒ ᳓ ᳔ ᳕ ᳖ ᳗ ᳘ ᳙ ᳚ ᳛ ᳜ ᳝ ᳞ ᳟\
/// ᳠ ᳡ ᳢ ᳣ ᳤ ᳥ ᳦ ᳧ ᳨ ᳩ ᳪ ᳫ ᳬ ᳭ ᳮ ᳯ\
/// ᳰ ᳱ ᳲ ᳳ ᳴ ᳵ ᳶ ᳷ ᳸ ᳹ ᳺ\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{1cd0}: '᳐'
    pub const VEDIC_TONE_KARSHANA: char = '᳐';
    /// \u{1cd1}: '᳑'
    pub const VEDIC_TONE_SHARA: char = '᳑';
    /// \u{1cd2}: '᳒'
    pub const VEDIC_TONE_PRENKHA: char = '᳒';
    /// \u{1cd3}: '᳓'
    pub const VEDIC_SIGN_NIHSHVASA: char = '᳓';
    /// \u{1cd4}: '᳔'
    pub const VEDIC_SIGN_YAJURVEDIC_MIDLINE_SVARITA: char = '᳔';
    /// \u{1cd5}: '᳕'
    pub const VEDIC_TONE_YAJURVEDIC_AGGRAVATED_INDEPENDENT_SVARITA: char = '᳕';
    /// \u{1cd6}: '᳖'
    pub const VEDIC_TONE_YAJURVEDIC_INDEPENDENT_SVARITA: char = '᳖';
    /// \u{1cd7}: '᳗'
    pub const VEDIC_TONE_YAJURVEDIC_KATHAKA_INDEPENDENT_SVARITA: char = '᳗';
    /// \u{1cd8}: '᳘'
    pub const VEDIC_TONE_CANDRA_BELOW: char = '᳘';
    /// \u{1cd9}: '᳙'
    pub const VEDIC_TONE_YAJURVEDIC_KATHAKA_INDEPENDENT_SVARITA_SCHROEDER: char = '᳙';
    /// \u{1cda}: '᳚'
    pub const VEDIC_TONE_DOUBLE_SVARITA: char = '᳚';
    /// \u{1cdb}: '᳛'
    pub const VEDIC_TONE_TRIPLE_SVARITA: char = '᳛';
    /// \u{1cdc}: '᳜'
    pub const VEDIC_TONE_KATHAKA_ANUDATTA: char = '᳜';
    /// \u{1cdd}: '᳝'
    pub const VEDIC_TONE_DOT_BELOW: char = '᳝';
    /// \u{1cde}: '᳞'
    pub const VEDIC_TONE_TWO_DOTS_BELOW: char = '᳞';
    /// \u{1cdf}: '᳟'
    pub const VEDIC_TONE_THREE_DOTS_BELOW: char = '᳟';
    /// \u{1ce0}: '᳠'
    pub const VEDIC_TONE_RIGVEDIC_KASHMIRI_INDEPENDENT_SVARITA: char = '᳠';
    /// \u{1ce1}: '᳡'
    pub const VEDIC_TONE_ATHARVAVEDIC_INDEPENDENT_SVARITA: char = '᳡';
    /// \u{1ce2}: '᳢'
    pub const VEDIC_SIGN_VISARGA_SVARITA: char = '᳢';
    /// \u{1ce3}: '᳣'
    pub const VEDIC_SIGN_VISARGA_UDATTA: char = '᳣';
    /// \u{1ce4}: '᳤'
    pub const VEDIC_SIGN_REVERSED_VISARGA_UDATTA: char = '᳤';
    /// \u{1ce5}: '᳥'
    pub const VEDIC_SIGN_VISARGA_ANUDATTA: char = '᳥';
    /// \u{1ce6}: '᳦'
    pub const VEDIC_SIGN_REVERSED_VISARGA_ANUDATTA: char = '᳦';
    /// \u{1ce7}: '᳧'
    pub const VEDIC_SIGN_VISARGA_UDATTA_WITH_TAIL: char = '᳧';
    /// \u{1ce8}: '᳨'
    pub const VEDIC_SIGN_VISARGA_ANUDATTA_WITH_TAIL: char = '᳨';
    /// \u{1ce9}: 'ᳩ'
    pub const VEDIC_SIGN_ANUSVARA_ANTARGOMUKHA: char = 'ᳩ';
    /// \u{1cea}: 'ᳪ'
    pub const VEDIC_SIGN_ANUSVARA_BAHIRGOMUKHA: char = 'ᳪ';
    /// \u{1ceb}: 'ᳫ'
    pub const VEDIC_SIGN_ANUSVARA_VAMAGOMUKHA: char = 'ᳫ';
    /// \u{1cec}: 'ᳬ'
    pub const VEDIC_SIGN_ANUSVARA_VAMAGOMUKHA_WITH_TAIL: char = 'ᳬ';
    /// \u{1ced}: '᳭'
    pub const VEDIC_SIGN_TIRYAK: char = '᳭';
    /// \u{1cee}: 'ᳮ'
    pub const VEDIC_SIGN_HEXIFORM_LONG_ANUSVARA: char = 'ᳮ';
    /// \u{1cef}: 'ᳯ'
    pub const VEDIC_SIGN_LONG_ANUSVARA: char = 'ᳯ';
    /// \u{1cf0}: 'ᳰ'
    pub const VEDIC_SIGN_RTHANG_LONG_ANUSVARA: char = 'ᳰ';
    /// \u{1cf1}: 'ᳱ'
    pub const VEDIC_SIGN_ANUSVARA_UBHAYATO_MUKHA: char = 'ᳱ';
    /// \u{1cf2}: 'ᳲ'
    pub const VEDIC_SIGN_ARDHAVISARGA: char = 'ᳲ';
    /// \u{1cf3}: 'ᳳ'
    pub const VEDIC_SIGN_ROTATED_ARDHAVISARGA: char = 'ᳳ';
    /// \u{1cf4}: '᳴'
    pub const VEDIC_TONE_CANDRA_ABOVE: char = '᳴';
    /// \u{1cf5}: 'ᳵ'
    pub const VEDIC_SIGN_JIHVAMULIYA: char = 'ᳵ';
    /// \u{1cf6}: 'ᳶ'
    pub const VEDIC_SIGN_UPADHMANIYA: char = 'ᳶ';
    /// \u{1cf7}: '᳷'
    pub const VEDIC_SIGN_ATIKRAMA: char = '᳷';
    /// \u{1cf8}: '᳸'
    pub const VEDIC_TONE_RING_ABOVE: char = '᳸';
    /// \u{1cf9}: '᳹'
    pub const VEDIC_TONE_DOUBLE_RING_ABOVE: char = '᳹';
    /// \u{1cfa}: 'ᳺ'
    pub const VEDIC_SIGN_DOUBLE_ANUSVARA_ANTARGOMUKHA: char = 'ᳺ';
}

/// An enum to represent all characters in the VedicExtensions block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum VedicExtensions {
    /// \u{1cd0}: '᳐'
    VedicToneKarshana,
    /// \u{1cd1}: '᳑'
    VedicToneShara,
    /// \u{1cd2}: '᳒'
    VedicTonePrenkha,
    /// \u{1cd3}: '᳓'
    VedicSignNihshvasa,
    /// \u{1cd4}: '᳔'
    VedicSignYajurvedicMidlineSvarita,
    /// \u{1cd5}: '᳕'
    VedicToneYajurvedicAggravatedIndependentSvarita,
    /// \u{1cd6}: '᳖'
    VedicToneYajurvedicIndependentSvarita,
    /// \u{1cd7}: '᳗'
    VedicToneYajurvedicKathakaIndependentSvarita,
    /// \u{1cd8}: '᳘'
    VedicToneCandraBelow,
    /// \u{1cd9}: '᳙'
    VedicToneYajurvedicKathakaIndependentSvaritaSchroeder,
    /// \u{1cda}: '᳚'
    VedicToneDoubleSvarita,
    /// \u{1cdb}: '᳛'
    VedicToneTripleSvarita,
    /// \u{1cdc}: '᳜'
    VedicToneKathakaAnudatta,
    /// \u{1cdd}: '᳝'
    VedicToneDotBelow,
    /// \u{1cde}: '᳞'
    VedicToneTwoDotsBelow,
    /// \u{1cdf}: '᳟'
    VedicToneThreeDotsBelow,
    /// \u{1ce0}: '᳠'
    VedicToneRigvedicKashmiriIndependentSvarita,
    /// \u{1ce1}: '᳡'
    VedicToneAtharvavedicIndependentSvarita,
    /// \u{1ce2}: '᳢'
    VedicSignVisargaSvarita,
    /// \u{1ce3}: '᳣'
    VedicSignVisargaUdatta,
    /// \u{1ce4}: '᳤'
    VedicSignReversedVisargaUdatta,
    /// \u{1ce5}: '᳥'
    VedicSignVisargaAnudatta,
    /// \u{1ce6}: '᳦'
    VedicSignReversedVisargaAnudatta,
    /// \u{1ce7}: '᳧'
    VedicSignVisargaUdattaWithTail,
    /// \u{1ce8}: '᳨'
    VedicSignVisargaAnudattaWithTail,
    /// \u{1ce9}: 'ᳩ'
    VedicSignAnusvaraAntargomukha,
    /// \u{1cea}: 'ᳪ'
    VedicSignAnusvaraBahirgomukha,
    /// \u{1ceb}: 'ᳫ'
    VedicSignAnusvaraVamagomukha,
    /// \u{1cec}: 'ᳬ'
    VedicSignAnusvaraVamagomukhaWithTail,
    /// \u{1ced}: '᳭'
    VedicSignTiryak,
    /// \u{1cee}: 'ᳮ'
    VedicSignHexiformLongAnusvara,
    /// \u{1cef}: 'ᳯ'
    VedicSignLongAnusvara,
    /// \u{1cf0}: 'ᳰ'
    VedicSignRthangLongAnusvara,
    /// \u{1cf1}: 'ᳱ'
    VedicSignAnusvaraUbhayatoMukha,
    /// \u{1cf2}: 'ᳲ'
    VedicSignArdhavisarga,
    /// \u{1cf3}: 'ᳳ'
    VedicSignRotatedArdhavisarga,
    /// \u{1cf4}: '᳴'
    VedicToneCandraAbove,
    /// \u{1cf5}: 'ᳵ'
    VedicSignJihvamuliya,
    /// \u{1cf6}: 'ᳶ'
    VedicSignUpadhmaniya,
    /// \u{1cf7}: '᳷'
    VedicSignAtikrama,
    /// \u{1cf8}: '᳸'
    VedicToneRingAbove,
    /// \u{1cf9}: '᳹'
    VedicToneDoubleRingAbove,
    /// \u{1cfa}: 'ᳺ'
    VedicSignDoubleAnusvaraAntargomukha,
}

impl Into<char> for VedicExtensions {
    fn into(self) -> char {
        use constants::*;
        match self {
            VedicExtensions::VedicToneKarshana => VEDIC_TONE_KARSHANA,
            VedicExtensions::VedicToneShara => VEDIC_TONE_SHARA,
            VedicExtensions::VedicTonePrenkha => VEDIC_TONE_PRENKHA,
            VedicExtensions::VedicSignNihshvasa => VEDIC_SIGN_NIHSHVASA,
            VedicExtensions::VedicSignYajurvedicMidlineSvarita => VEDIC_SIGN_YAJURVEDIC_MIDLINE_SVARITA,
            VedicExtensions::VedicToneYajurvedicAggravatedIndependentSvarita => VEDIC_TONE_YAJURVEDIC_AGGRAVATED_INDEPENDENT_SVARITA,
            VedicExtensions::VedicToneYajurvedicIndependentSvarita => VEDIC_TONE_YAJURVEDIC_INDEPENDENT_SVARITA,
            VedicExtensions::VedicToneYajurvedicKathakaIndependentSvarita => VEDIC_TONE_YAJURVEDIC_KATHAKA_INDEPENDENT_SVARITA,
            VedicExtensions::VedicToneCandraBelow => VEDIC_TONE_CANDRA_BELOW,
            VedicExtensions::VedicToneYajurvedicKathakaIndependentSvaritaSchroeder => VEDIC_TONE_YAJURVEDIC_KATHAKA_INDEPENDENT_SVARITA_SCHROEDER,
            VedicExtensions::VedicToneDoubleSvarita => VEDIC_TONE_DOUBLE_SVARITA,
            VedicExtensions::VedicToneTripleSvarita => VEDIC_TONE_TRIPLE_SVARITA,
            VedicExtensions::VedicToneKathakaAnudatta => VEDIC_TONE_KATHAKA_ANUDATTA,
            VedicExtensions::VedicToneDotBelow => VEDIC_TONE_DOT_BELOW,
            VedicExtensions::VedicToneTwoDotsBelow => VEDIC_TONE_TWO_DOTS_BELOW,
            VedicExtensions::VedicToneThreeDotsBelow => VEDIC_TONE_THREE_DOTS_BELOW,
            VedicExtensions::VedicToneRigvedicKashmiriIndependentSvarita => VEDIC_TONE_RIGVEDIC_KASHMIRI_INDEPENDENT_SVARITA,
            VedicExtensions::VedicToneAtharvavedicIndependentSvarita => VEDIC_TONE_ATHARVAVEDIC_INDEPENDENT_SVARITA,
            VedicExtensions::VedicSignVisargaSvarita => VEDIC_SIGN_VISARGA_SVARITA,
            VedicExtensions::VedicSignVisargaUdatta => VEDIC_SIGN_VISARGA_UDATTA,
            VedicExtensions::VedicSignReversedVisargaUdatta => VEDIC_SIGN_REVERSED_VISARGA_UDATTA,
            VedicExtensions::VedicSignVisargaAnudatta => VEDIC_SIGN_VISARGA_ANUDATTA,
            VedicExtensions::VedicSignReversedVisargaAnudatta => VEDIC_SIGN_REVERSED_VISARGA_ANUDATTA,
            VedicExtensions::VedicSignVisargaUdattaWithTail => VEDIC_SIGN_VISARGA_UDATTA_WITH_TAIL,
            VedicExtensions::VedicSignVisargaAnudattaWithTail => VEDIC_SIGN_VISARGA_ANUDATTA_WITH_TAIL,
            VedicExtensions::VedicSignAnusvaraAntargomukha => VEDIC_SIGN_ANUSVARA_ANTARGOMUKHA,
            VedicExtensions::VedicSignAnusvaraBahirgomukha => VEDIC_SIGN_ANUSVARA_BAHIRGOMUKHA,
            VedicExtensions::VedicSignAnusvaraVamagomukha => VEDIC_SIGN_ANUSVARA_VAMAGOMUKHA,
            VedicExtensions::VedicSignAnusvaraVamagomukhaWithTail => VEDIC_SIGN_ANUSVARA_VAMAGOMUKHA_WITH_TAIL,
            VedicExtensions::VedicSignTiryak => VEDIC_SIGN_TIRYAK,
            VedicExtensions::VedicSignHexiformLongAnusvara => VEDIC_SIGN_HEXIFORM_LONG_ANUSVARA,
            VedicExtensions::VedicSignLongAnusvara => VEDIC_SIGN_LONG_ANUSVARA,
            VedicExtensions::VedicSignRthangLongAnusvara => VEDIC_SIGN_RTHANG_LONG_ANUSVARA,
            VedicExtensions::VedicSignAnusvaraUbhayatoMukha => VEDIC_SIGN_ANUSVARA_UBHAYATO_MUKHA,
            VedicExtensions::VedicSignArdhavisarga => VEDIC_SIGN_ARDHAVISARGA,
            VedicExtensions::VedicSignRotatedArdhavisarga => VEDIC_SIGN_ROTATED_ARDHAVISARGA,
            VedicExtensions::VedicToneCandraAbove => VEDIC_TONE_CANDRA_ABOVE,
            VedicExtensions::VedicSignJihvamuliya => VEDIC_SIGN_JIHVAMULIYA,
            VedicExtensions::VedicSignUpadhmaniya => VEDIC_SIGN_UPADHMANIYA,
            VedicExtensions::VedicSignAtikrama => VEDIC_SIGN_ATIKRAMA,
            VedicExtensions::VedicToneRingAbove => VEDIC_TONE_RING_ABOVE,
            VedicExtensions::VedicToneDoubleRingAbove => VEDIC_TONE_DOUBLE_RING_ABOVE,
            VedicExtensions::VedicSignDoubleAnusvaraAntargomukha => VEDIC_SIGN_DOUBLE_ANUSVARA_ANTARGOMUKHA,
        }
    }
}

impl std::convert::TryFrom<char> for VedicExtensions {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            VEDIC_TONE_KARSHANA => Ok(VedicExtensions::VedicToneKarshana),
            VEDIC_TONE_SHARA => Ok(VedicExtensions::VedicToneShara),
            VEDIC_TONE_PRENKHA => Ok(VedicExtensions::VedicTonePrenkha),
            VEDIC_SIGN_NIHSHVASA => Ok(VedicExtensions::VedicSignNihshvasa),
            VEDIC_SIGN_YAJURVEDIC_MIDLINE_SVARITA => Ok(VedicExtensions::VedicSignYajurvedicMidlineSvarita),
            VEDIC_TONE_YAJURVEDIC_AGGRAVATED_INDEPENDENT_SVARITA => Ok(VedicExtensions::VedicToneYajurvedicAggravatedIndependentSvarita),
            VEDIC_TONE_YAJURVEDIC_INDEPENDENT_SVARITA => Ok(VedicExtensions::VedicToneYajurvedicIndependentSvarita),
            VEDIC_TONE_YAJURVEDIC_KATHAKA_INDEPENDENT_SVARITA => Ok(VedicExtensions::VedicToneYajurvedicKathakaIndependentSvarita),
            VEDIC_TONE_CANDRA_BELOW => Ok(VedicExtensions::VedicToneCandraBelow),
            VEDIC_TONE_YAJURVEDIC_KATHAKA_INDEPENDENT_SVARITA_SCHROEDER => Ok(VedicExtensions::VedicToneYajurvedicKathakaIndependentSvaritaSchroeder),
            VEDIC_TONE_DOUBLE_SVARITA => Ok(VedicExtensions::VedicToneDoubleSvarita),
            VEDIC_TONE_TRIPLE_SVARITA => Ok(VedicExtensions::VedicToneTripleSvarita),
            VEDIC_TONE_KATHAKA_ANUDATTA => Ok(VedicExtensions::VedicToneKathakaAnudatta),
            VEDIC_TONE_DOT_BELOW => Ok(VedicExtensions::VedicToneDotBelow),
            VEDIC_TONE_TWO_DOTS_BELOW => Ok(VedicExtensions::VedicToneTwoDotsBelow),
            VEDIC_TONE_THREE_DOTS_BELOW => Ok(VedicExtensions::VedicToneThreeDotsBelow),
            VEDIC_TONE_RIGVEDIC_KASHMIRI_INDEPENDENT_SVARITA => Ok(VedicExtensions::VedicToneRigvedicKashmiriIndependentSvarita),
            VEDIC_TONE_ATHARVAVEDIC_INDEPENDENT_SVARITA => Ok(VedicExtensions::VedicToneAtharvavedicIndependentSvarita),
            VEDIC_SIGN_VISARGA_SVARITA => Ok(VedicExtensions::VedicSignVisargaSvarita),
            VEDIC_SIGN_VISARGA_UDATTA => Ok(VedicExtensions::VedicSignVisargaUdatta),
            VEDIC_SIGN_REVERSED_VISARGA_UDATTA => Ok(VedicExtensions::VedicSignReversedVisargaUdatta),
            VEDIC_SIGN_VISARGA_ANUDATTA => Ok(VedicExtensions::VedicSignVisargaAnudatta),
            VEDIC_SIGN_REVERSED_VISARGA_ANUDATTA => Ok(VedicExtensions::VedicSignReversedVisargaAnudatta),
            VEDIC_SIGN_VISARGA_UDATTA_WITH_TAIL => Ok(VedicExtensions::VedicSignVisargaUdattaWithTail),
            VEDIC_SIGN_VISARGA_ANUDATTA_WITH_TAIL => Ok(VedicExtensions::VedicSignVisargaAnudattaWithTail),
            VEDIC_SIGN_ANUSVARA_ANTARGOMUKHA => Ok(VedicExtensions::VedicSignAnusvaraAntargomukha),
            VEDIC_SIGN_ANUSVARA_BAHIRGOMUKHA => Ok(VedicExtensions::VedicSignAnusvaraBahirgomukha),
            VEDIC_SIGN_ANUSVARA_VAMAGOMUKHA => Ok(VedicExtensions::VedicSignAnusvaraVamagomukha),
            VEDIC_SIGN_ANUSVARA_VAMAGOMUKHA_WITH_TAIL => Ok(VedicExtensions::VedicSignAnusvaraVamagomukhaWithTail),
            VEDIC_SIGN_TIRYAK => Ok(VedicExtensions::VedicSignTiryak),
            VEDIC_SIGN_HEXIFORM_LONG_ANUSVARA => Ok(VedicExtensions::VedicSignHexiformLongAnusvara),
            VEDIC_SIGN_LONG_ANUSVARA => Ok(VedicExtensions::VedicSignLongAnusvara),
            VEDIC_SIGN_RTHANG_LONG_ANUSVARA => Ok(VedicExtensions::VedicSignRthangLongAnusvara),
            VEDIC_SIGN_ANUSVARA_UBHAYATO_MUKHA => Ok(VedicExtensions::VedicSignAnusvaraUbhayatoMukha),
            VEDIC_SIGN_ARDHAVISARGA => Ok(VedicExtensions::VedicSignArdhavisarga),
            VEDIC_SIGN_ROTATED_ARDHAVISARGA => Ok(VedicExtensions::VedicSignRotatedArdhavisarga),
            VEDIC_TONE_CANDRA_ABOVE => Ok(VedicExtensions::VedicToneCandraAbove),
            VEDIC_SIGN_JIHVAMULIYA => Ok(VedicExtensions::VedicSignJihvamuliya),
            VEDIC_SIGN_UPADHMANIYA => Ok(VedicExtensions::VedicSignUpadhmaniya),
            VEDIC_SIGN_ATIKRAMA => Ok(VedicExtensions::VedicSignAtikrama),
            VEDIC_TONE_RING_ABOVE => Ok(VedicExtensions::VedicToneRingAbove),
            VEDIC_TONE_DOUBLE_RING_ABOVE => Ok(VedicExtensions::VedicToneDoubleRingAbove),
            VEDIC_SIGN_DOUBLE_ANUSVARA_ANTARGOMUKHA => Ok(VedicExtensions::VedicSignDoubleAnusvaraAntargomukha),
            _ => Err(()),
        }
    }
}

impl Into<u32> for VedicExtensions {
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

impl std::convert::TryFrom<u32> for VedicExtensions {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for VedicExtensions {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl VedicExtensions {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        VedicExtensions::VedicToneKarshana
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            VedicExtensions::VedicToneKarshana => "vedic tone karshana",
            VedicExtensions::VedicToneShara => "vedic tone shara",
            VedicExtensions::VedicTonePrenkha => "vedic tone prenkha",
            VedicExtensions::VedicSignNihshvasa => "vedic sign nihshvasa",
            VedicExtensions::VedicSignYajurvedicMidlineSvarita => "vedic sign yajurvedic midline svarita",
            VedicExtensions::VedicToneYajurvedicAggravatedIndependentSvarita => "vedic tone yajurvedic aggravated independent svarita",
            VedicExtensions::VedicToneYajurvedicIndependentSvarita => "vedic tone yajurvedic independent svarita",
            VedicExtensions::VedicToneYajurvedicKathakaIndependentSvarita => "vedic tone yajurvedic kathaka independent svarita",
            VedicExtensions::VedicToneCandraBelow => "vedic tone candra below",
            VedicExtensions::VedicToneYajurvedicKathakaIndependentSvaritaSchroeder => "vedic tone yajurvedic kathaka independent svarita schroeder",
            VedicExtensions::VedicToneDoubleSvarita => "vedic tone double svarita",
            VedicExtensions::VedicToneTripleSvarita => "vedic tone triple svarita",
            VedicExtensions::VedicToneKathakaAnudatta => "vedic tone kathaka anudatta",
            VedicExtensions::VedicToneDotBelow => "vedic tone dot below",
            VedicExtensions::VedicToneTwoDotsBelow => "vedic tone two dots below",
            VedicExtensions::VedicToneThreeDotsBelow => "vedic tone three dots below",
            VedicExtensions::VedicToneRigvedicKashmiriIndependentSvarita => "vedic tone rigvedic kashmiri independent svarita",
            VedicExtensions::VedicToneAtharvavedicIndependentSvarita => "vedic tone atharvavedic independent svarita",
            VedicExtensions::VedicSignVisargaSvarita => "vedic sign visarga svarita",
            VedicExtensions::VedicSignVisargaUdatta => "vedic sign visarga udatta",
            VedicExtensions::VedicSignReversedVisargaUdatta => "vedic sign reversed visarga udatta",
            VedicExtensions::VedicSignVisargaAnudatta => "vedic sign visarga anudatta",
            VedicExtensions::VedicSignReversedVisargaAnudatta => "vedic sign reversed visarga anudatta",
            VedicExtensions::VedicSignVisargaUdattaWithTail => "vedic sign visarga udatta with tail",
            VedicExtensions::VedicSignVisargaAnudattaWithTail => "vedic sign visarga anudatta with tail",
            VedicExtensions::VedicSignAnusvaraAntargomukha => "vedic sign anusvara antargomukha",
            VedicExtensions::VedicSignAnusvaraBahirgomukha => "vedic sign anusvara bahirgomukha",
            VedicExtensions::VedicSignAnusvaraVamagomukha => "vedic sign anusvara vamagomukha",
            VedicExtensions::VedicSignAnusvaraVamagomukhaWithTail => "vedic sign anusvara vamagomukha with tail",
            VedicExtensions::VedicSignTiryak => "vedic sign tiryak",
            VedicExtensions::VedicSignHexiformLongAnusvara => "vedic sign hexiform long anusvara",
            VedicExtensions::VedicSignLongAnusvara => "vedic sign long anusvara",
            VedicExtensions::VedicSignRthangLongAnusvara => "vedic sign rthang long anusvara",
            VedicExtensions::VedicSignAnusvaraUbhayatoMukha => "vedic sign anusvara ubhayato mukha",
            VedicExtensions::VedicSignArdhavisarga => "vedic sign ardhavisarga",
            VedicExtensions::VedicSignRotatedArdhavisarga => "vedic sign rotated ardhavisarga",
            VedicExtensions::VedicToneCandraAbove => "vedic tone candra above",
            VedicExtensions::VedicSignJihvamuliya => "vedic sign jihvamuliya",
            VedicExtensions::VedicSignUpadhmaniya => "vedic sign upadhmaniya",
            VedicExtensions::VedicSignAtikrama => "vedic sign atikrama",
            VedicExtensions::VedicToneRingAbove => "vedic tone ring above",
            VedicExtensions::VedicToneDoubleRingAbove => "vedic tone double ring above",
            VedicExtensions::VedicSignDoubleAnusvaraAntargomukha => "vedic sign double anusvara antargomukha",
        }
    }
}
