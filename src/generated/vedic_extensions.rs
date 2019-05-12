
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
        match self {
            VedicExtensions::VedicToneKarshana => '᳐',
            VedicExtensions::VedicToneShara => '᳑',
            VedicExtensions::VedicTonePrenkha => '᳒',
            VedicExtensions::VedicSignNihshvasa => '᳓',
            VedicExtensions::VedicSignYajurvedicMidlineSvarita => '᳔',
            VedicExtensions::VedicToneYajurvedicAggravatedIndependentSvarita => '᳕',
            VedicExtensions::VedicToneYajurvedicIndependentSvarita => '᳖',
            VedicExtensions::VedicToneYajurvedicKathakaIndependentSvarita => '᳗',
            VedicExtensions::VedicToneCandraBelow => '᳘',
            VedicExtensions::VedicToneYajurvedicKathakaIndependentSvaritaSchroeder => '᳙',
            VedicExtensions::VedicToneDoubleSvarita => '᳚',
            VedicExtensions::VedicToneTripleSvarita => '᳛',
            VedicExtensions::VedicToneKathakaAnudatta => '᳜',
            VedicExtensions::VedicToneDotBelow => '᳝',
            VedicExtensions::VedicToneTwoDotsBelow => '᳞',
            VedicExtensions::VedicToneThreeDotsBelow => '᳟',
            VedicExtensions::VedicToneRigvedicKashmiriIndependentSvarita => '᳠',
            VedicExtensions::VedicToneAtharvavedicIndependentSvarita => '᳡',
            VedicExtensions::VedicSignVisargaSvarita => '᳢',
            VedicExtensions::VedicSignVisargaUdatta => '᳣',
            VedicExtensions::VedicSignReversedVisargaUdatta => '᳤',
            VedicExtensions::VedicSignVisargaAnudatta => '᳥',
            VedicExtensions::VedicSignReversedVisargaAnudatta => '᳦',
            VedicExtensions::VedicSignVisargaUdattaWithTail => '᳧',
            VedicExtensions::VedicSignVisargaAnudattaWithTail => '᳨',
            VedicExtensions::VedicSignAnusvaraAntargomukha => 'ᳩ',
            VedicExtensions::VedicSignAnusvaraBahirgomukha => 'ᳪ',
            VedicExtensions::VedicSignAnusvaraVamagomukha => 'ᳫ',
            VedicExtensions::VedicSignAnusvaraVamagomukhaWithTail => 'ᳬ',
            VedicExtensions::VedicSignTiryak => '᳭',
            VedicExtensions::VedicSignHexiformLongAnusvara => 'ᳮ',
            VedicExtensions::VedicSignLongAnusvara => 'ᳯ',
            VedicExtensions::VedicSignRthangLongAnusvara => 'ᳰ',
            VedicExtensions::VedicSignAnusvaraUbhayatoMukha => 'ᳱ',
            VedicExtensions::VedicSignArdhavisarga => 'ᳲ',
            VedicExtensions::VedicSignRotatedArdhavisarga => 'ᳳ',
            VedicExtensions::VedicToneCandraAbove => '᳴',
            VedicExtensions::VedicSignJihvamuliya => 'ᳵ',
            VedicExtensions::VedicSignUpadhmaniya => 'ᳶ',
            VedicExtensions::VedicSignAtikrama => '᳷',
            VedicExtensions::VedicToneRingAbove => '᳸',
            VedicExtensions::VedicToneDoubleRingAbove => '᳹',
            VedicExtensions::VedicSignDoubleAnusvaraAntargomukha => 'ᳺ',
        }
    }
}

impl std::convert::TryFrom<char> for VedicExtensions {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '᳐' => Ok(VedicExtensions::VedicToneKarshana),
            '᳑' => Ok(VedicExtensions::VedicToneShara),
            '᳒' => Ok(VedicExtensions::VedicTonePrenkha),
            '᳓' => Ok(VedicExtensions::VedicSignNihshvasa),
            '᳔' => Ok(VedicExtensions::VedicSignYajurvedicMidlineSvarita),
            '᳕' => Ok(VedicExtensions::VedicToneYajurvedicAggravatedIndependentSvarita),
            '᳖' => Ok(VedicExtensions::VedicToneYajurvedicIndependentSvarita),
            '᳗' => Ok(VedicExtensions::VedicToneYajurvedicKathakaIndependentSvarita),
            '᳘' => Ok(VedicExtensions::VedicToneCandraBelow),
            '᳙' => Ok(VedicExtensions::VedicToneYajurvedicKathakaIndependentSvaritaSchroeder),
            '᳚' => Ok(VedicExtensions::VedicToneDoubleSvarita),
            '᳛' => Ok(VedicExtensions::VedicToneTripleSvarita),
            '᳜' => Ok(VedicExtensions::VedicToneKathakaAnudatta),
            '᳝' => Ok(VedicExtensions::VedicToneDotBelow),
            '᳞' => Ok(VedicExtensions::VedicToneTwoDotsBelow),
            '᳟' => Ok(VedicExtensions::VedicToneThreeDotsBelow),
            '᳠' => Ok(VedicExtensions::VedicToneRigvedicKashmiriIndependentSvarita),
            '᳡' => Ok(VedicExtensions::VedicToneAtharvavedicIndependentSvarita),
            '᳢' => Ok(VedicExtensions::VedicSignVisargaSvarita),
            '᳣' => Ok(VedicExtensions::VedicSignVisargaUdatta),
            '᳤' => Ok(VedicExtensions::VedicSignReversedVisargaUdatta),
            '᳥' => Ok(VedicExtensions::VedicSignVisargaAnudatta),
            '᳦' => Ok(VedicExtensions::VedicSignReversedVisargaAnudatta),
            '᳧' => Ok(VedicExtensions::VedicSignVisargaUdattaWithTail),
            '᳨' => Ok(VedicExtensions::VedicSignVisargaAnudattaWithTail),
            'ᳩ' => Ok(VedicExtensions::VedicSignAnusvaraAntargomukha),
            'ᳪ' => Ok(VedicExtensions::VedicSignAnusvaraBahirgomukha),
            'ᳫ' => Ok(VedicExtensions::VedicSignAnusvaraVamagomukha),
            'ᳬ' => Ok(VedicExtensions::VedicSignAnusvaraVamagomukhaWithTail),
            '᳭' => Ok(VedicExtensions::VedicSignTiryak),
            'ᳮ' => Ok(VedicExtensions::VedicSignHexiformLongAnusvara),
            'ᳯ' => Ok(VedicExtensions::VedicSignLongAnusvara),
            'ᳰ' => Ok(VedicExtensions::VedicSignRthangLongAnusvara),
            'ᳱ' => Ok(VedicExtensions::VedicSignAnusvaraUbhayatoMukha),
            'ᳲ' => Ok(VedicExtensions::VedicSignArdhavisarga),
            'ᳳ' => Ok(VedicExtensions::VedicSignRotatedArdhavisarga),
            '᳴' => Ok(VedicExtensions::VedicToneCandraAbove),
            'ᳵ' => Ok(VedicExtensions::VedicSignJihvamuliya),
            'ᳶ' => Ok(VedicExtensions::VedicSignUpadhmaniya),
            '᳷' => Ok(VedicExtensions::VedicSignAtikrama),
            '᳸' => Ok(VedicExtensions::VedicToneRingAbove),
            '᳹' => Ok(VedicExtensions::VedicToneDoubleRingAbove),
            'ᳺ' => Ok(VedicExtensions::VedicSignDoubleAnusvaraAntargomukha),
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

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("VedicExtensions{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
