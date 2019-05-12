
/// An enum to represent all characters in the AlphabeticPresentationForms block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum AlphabeticPresentationForms {
    /// \u{fb00}: 'ﬀ'
    LatinSmallLigatureFf,
    /// \u{fb01}: 'ﬁ'
    LatinSmallLigatureFi,
    /// \u{fb02}: 'ﬂ'
    LatinSmallLigatureFl,
    /// \u{fb03}: 'ﬃ'
    LatinSmallLigatureFfi,
    /// \u{fb04}: 'ﬄ'
    LatinSmallLigatureFfl,
    /// \u{fb05}: 'ﬅ'
    LatinSmallLigatureLongST,
    /// \u{fb06}: 'ﬆ'
    LatinSmallLigatureSt,
    /// \u{fb13}: 'ﬓ'
    ArmenianSmallLigatureMenNow,
    /// \u{fb14}: 'ﬔ'
    ArmenianSmallLigatureMenEch,
    /// \u{fb15}: 'ﬕ'
    ArmenianSmallLigatureMenIni,
    /// \u{fb16}: 'ﬖ'
    ArmenianSmallLigatureVewNow,
    /// \u{fb17}: 'ﬗ'
    ArmenianSmallLigatureMenXeh,
    /// \u{fb1d}: 'יִ'
    HebrewLetterYodWithHiriq,
    /// \u{fb1e}: 'ﬞ'
    HebrewPointJudeoDashSpanishVarika,
    /// \u{fb1f}: 'ײַ'
    HebrewLigatureYiddishYodYodPatah,
    /// \u{fb20}: 'ﬠ'
    HebrewLetterAlternativeAyin,
    /// \u{fb21}: 'ﬡ'
    HebrewLetterWideAlef,
    /// \u{fb22}: 'ﬢ'
    HebrewLetterWideDalet,
    /// \u{fb23}: 'ﬣ'
    HebrewLetterWideHe,
    /// \u{fb24}: 'ﬤ'
    HebrewLetterWideKaf,
    /// \u{fb25}: 'ﬥ'
    HebrewLetterWideLamed,
    /// \u{fb26}: 'ﬦ'
    HebrewLetterWideFinalMem,
    /// \u{fb27}: 'ﬧ'
    HebrewLetterWideResh,
    /// \u{fb28}: 'ﬨ'
    HebrewLetterWideTav,
    /// \u{fb29}: '﬩'
    HebrewLetterAlternativePlusSign,
    /// \u{fb2a}: 'שׁ'
    HebrewLetterShinWithShinDot,
    /// \u{fb2b}: 'שׂ'
    HebrewLetterShinWithSinDot,
    /// \u{fb2c}: 'שּׁ'
    HebrewLetterShinWithDageshAndShinDot,
    /// \u{fb2d}: 'שּׂ'
    HebrewLetterShinWithDageshAndSinDot,
    /// \u{fb2e}: 'אַ'
    HebrewLetterAlefWithPatah,
    /// \u{fb2f}: 'אָ'
    HebrewLetterAlefWithQamats,
    /// \u{fb30}: 'אּ'
    HebrewLetterAlefWithMapiq,
    /// \u{fb31}: 'בּ'
    HebrewLetterBetWithDagesh,
    /// \u{fb32}: 'גּ'
    HebrewLetterGimelWithDagesh,
    /// \u{fb33}: 'דּ'
    HebrewLetterDaletWithDagesh,
    /// \u{fb34}: 'הּ'
    HebrewLetterHeWithMapiq,
    /// \u{fb35}: 'וּ'
    HebrewLetterVavWithDagesh,
    /// \u{fb36}: 'זּ'
    HebrewLetterZayinWithDagesh,
    /// \u{fb38}: 'טּ'
    HebrewLetterTetWithDagesh,
    /// \u{fb39}: 'יּ'
    HebrewLetterYodWithDagesh,
    /// \u{fb3a}: 'ךּ'
    HebrewLetterFinalKafWithDagesh,
    /// \u{fb3b}: 'כּ'
    HebrewLetterKafWithDagesh,
    /// \u{fb3c}: 'לּ'
    HebrewLetterLamedWithDagesh,
    /// \u{fb3e}: 'מּ'
    HebrewLetterMemWithDagesh,
    /// \u{fb40}: 'נּ'
    HebrewLetterNunWithDagesh,
    /// \u{fb41}: 'סּ'
    HebrewLetterSamekhWithDagesh,
    /// \u{fb43}: 'ףּ'
    HebrewLetterFinalPeWithDagesh,
    /// \u{fb44}: 'פּ'
    HebrewLetterPeWithDagesh,
    /// \u{fb46}: 'צּ'
    HebrewLetterTsadiWithDagesh,
    /// \u{fb47}: 'קּ'
    HebrewLetterQofWithDagesh,
    /// \u{fb48}: 'רּ'
    HebrewLetterReshWithDagesh,
    /// \u{fb49}: 'שּ'
    HebrewLetterShinWithDagesh,
    /// \u{fb4a}: 'תּ'
    HebrewLetterTavWithDagesh,
    /// \u{fb4b}: 'וֹ'
    HebrewLetterVavWithHolam,
    /// \u{fb4c}: 'בֿ'
    HebrewLetterBetWithRafe,
    /// \u{fb4d}: 'כֿ'
    HebrewLetterKafWithRafe,
    /// \u{fb4e}: 'פֿ'
    HebrewLetterPeWithRafe,
}

impl Into<char> for AlphabeticPresentationForms {
    fn into(self) -> char {
        match self {
            AlphabeticPresentationForms::LatinSmallLigatureFf => 'ﬀ',
            AlphabeticPresentationForms::LatinSmallLigatureFi => 'ﬁ',
            AlphabeticPresentationForms::LatinSmallLigatureFl => 'ﬂ',
            AlphabeticPresentationForms::LatinSmallLigatureFfi => 'ﬃ',
            AlphabeticPresentationForms::LatinSmallLigatureFfl => 'ﬄ',
            AlphabeticPresentationForms::LatinSmallLigatureLongST => 'ﬅ',
            AlphabeticPresentationForms::LatinSmallLigatureSt => 'ﬆ',
            AlphabeticPresentationForms::ArmenianSmallLigatureMenNow => 'ﬓ',
            AlphabeticPresentationForms::ArmenianSmallLigatureMenEch => 'ﬔ',
            AlphabeticPresentationForms::ArmenianSmallLigatureMenIni => 'ﬕ',
            AlphabeticPresentationForms::ArmenianSmallLigatureVewNow => 'ﬖ',
            AlphabeticPresentationForms::ArmenianSmallLigatureMenXeh => 'ﬗ',
            AlphabeticPresentationForms::HebrewLetterYodWithHiriq => 'יִ',
            AlphabeticPresentationForms::HebrewPointJudeoDashSpanishVarika => 'ﬞ',
            AlphabeticPresentationForms::HebrewLigatureYiddishYodYodPatah => 'ײַ',
            AlphabeticPresentationForms::HebrewLetterAlternativeAyin => 'ﬠ',
            AlphabeticPresentationForms::HebrewLetterWideAlef => 'ﬡ',
            AlphabeticPresentationForms::HebrewLetterWideDalet => 'ﬢ',
            AlphabeticPresentationForms::HebrewLetterWideHe => 'ﬣ',
            AlphabeticPresentationForms::HebrewLetterWideKaf => 'ﬤ',
            AlphabeticPresentationForms::HebrewLetterWideLamed => 'ﬥ',
            AlphabeticPresentationForms::HebrewLetterWideFinalMem => 'ﬦ',
            AlphabeticPresentationForms::HebrewLetterWideResh => 'ﬧ',
            AlphabeticPresentationForms::HebrewLetterWideTav => 'ﬨ',
            AlphabeticPresentationForms::HebrewLetterAlternativePlusSign => '﬩',
            AlphabeticPresentationForms::HebrewLetterShinWithShinDot => 'שׁ',
            AlphabeticPresentationForms::HebrewLetterShinWithSinDot => 'שׂ',
            AlphabeticPresentationForms::HebrewLetterShinWithDageshAndShinDot => 'שּׁ',
            AlphabeticPresentationForms::HebrewLetterShinWithDageshAndSinDot => 'שּׂ',
            AlphabeticPresentationForms::HebrewLetterAlefWithPatah => 'אַ',
            AlphabeticPresentationForms::HebrewLetterAlefWithQamats => 'אָ',
            AlphabeticPresentationForms::HebrewLetterAlefWithMapiq => 'אּ',
            AlphabeticPresentationForms::HebrewLetterBetWithDagesh => 'בּ',
            AlphabeticPresentationForms::HebrewLetterGimelWithDagesh => 'גּ',
            AlphabeticPresentationForms::HebrewLetterDaletWithDagesh => 'דּ',
            AlphabeticPresentationForms::HebrewLetterHeWithMapiq => 'הּ',
            AlphabeticPresentationForms::HebrewLetterVavWithDagesh => 'וּ',
            AlphabeticPresentationForms::HebrewLetterZayinWithDagesh => 'זּ',
            AlphabeticPresentationForms::HebrewLetterTetWithDagesh => 'טּ',
            AlphabeticPresentationForms::HebrewLetterYodWithDagesh => 'יּ',
            AlphabeticPresentationForms::HebrewLetterFinalKafWithDagesh => 'ךּ',
            AlphabeticPresentationForms::HebrewLetterKafWithDagesh => 'כּ',
            AlphabeticPresentationForms::HebrewLetterLamedWithDagesh => 'לּ',
            AlphabeticPresentationForms::HebrewLetterMemWithDagesh => 'מּ',
            AlphabeticPresentationForms::HebrewLetterNunWithDagesh => 'נּ',
            AlphabeticPresentationForms::HebrewLetterSamekhWithDagesh => 'סּ',
            AlphabeticPresentationForms::HebrewLetterFinalPeWithDagesh => 'ףּ',
            AlphabeticPresentationForms::HebrewLetterPeWithDagesh => 'פּ',
            AlphabeticPresentationForms::HebrewLetterTsadiWithDagesh => 'צּ',
            AlphabeticPresentationForms::HebrewLetterQofWithDagesh => 'קּ',
            AlphabeticPresentationForms::HebrewLetterReshWithDagesh => 'רּ',
            AlphabeticPresentationForms::HebrewLetterShinWithDagesh => 'שּ',
            AlphabeticPresentationForms::HebrewLetterTavWithDagesh => 'תּ',
            AlphabeticPresentationForms::HebrewLetterVavWithHolam => 'וֹ',
            AlphabeticPresentationForms::HebrewLetterBetWithRafe => 'בֿ',
            AlphabeticPresentationForms::HebrewLetterKafWithRafe => 'כֿ',
            AlphabeticPresentationForms::HebrewLetterPeWithRafe => 'פֿ',
        }
    }
}

impl std::convert::TryFrom<char> for AlphabeticPresentationForms {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ﬀ' => Ok(AlphabeticPresentationForms::LatinSmallLigatureFf),
            'ﬁ' => Ok(AlphabeticPresentationForms::LatinSmallLigatureFi),
            'ﬂ' => Ok(AlphabeticPresentationForms::LatinSmallLigatureFl),
            'ﬃ' => Ok(AlphabeticPresentationForms::LatinSmallLigatureFfi),
            'ﬄ' => Ok(AlphabeticPresentationForms::LatinSmallLigatureFfl),
            'ﬅ' => Ok(AlphabeticPresentationForms::LatinSmallLigatureLongST),
            'ﬆ' => Ok(AlphabeticPresentationForms::LatinSmallLigatureSt),
            'ﬓ' => Ok(AlphabeticPresentationForms::ArmenianSmallLigatureMenNow),
            'ﬔ' => Ok(AlphabeticPresentationForms::ArmenianSmallLigatureMenEch),
            'ﬕ' => Ok(AlphabeticPresentationForms::ArmenianSmallLigatureMenIni),
            'ﬖ' => Ok(AlphabeticPresentationForms::ArmenianSmallLigatureVewNow),
            'ﬗ' => Ok(AlphabeticPresentationForms::ArmenianSmallLigatureMenXeh),
            'יִ' => Ok(AlphabeticPresentationForms::HebrewLetterYodWithHiriq),
            'ﬞ' => Ok(AlphabeticPresentationForms::HebrewPointJudeoDashSpanishVarika),
            'ײַ' => Ok(AlphabeticPresentationForms::HebrewLigatureYiddishYodYodPatah),
            'ﬠ' => Ok(AlphabeticPresentationForms::HebrewLetterAlternativeAyin),
            'ﬡ' => Ok(AlphabeticPresentationForms::HebrewLetterWideAlef),
            'ﬢ' => Ok(AlphabeticPresentationForms::HebrewLetterWideDalet),
            'ﬣ' => Ok(AlphabeticPresentationForms::HebrewLetterWideHe),
            'ﬤ' => Ok(AlphabeticPresentationForms::HebrewLetterWideKaf),
            'ﬥ' => Ok(AlphabeticPresentationForms::HebrewLetterWideLamed),
            'ﬦ' => Ok(AlphabeticPresentationForms::HebrewLetterWideFinalMem),
            'ﬧ' => Ok(AlphabeticPresentationForms::HebrewLetterWideResh),
            'ﬨ' => Ok(AlphabeticPresentationForms::HebrewLetterWideTav),
            '﬩' => Ok(AlphabeticPresentationForms::HebrewLetterAlternativePlusSign),
            'שׁ' => Ok(AlphabeticPresentationForms::HebrewLetterShinWithShinDot),
            'שׂ' => Ok(AlphabeticPresentationForms::HebrewLetterShinWithSinDot),
            'שּׁ' => Ok(AlphabeticPresentationForms::HebrewLetterShinWithDageshAndShinDot),
            'שּׂ' => Ok(AlphabeticPresentationForms::HebrewLetterShinWithDageshAndSinDot),
            'אַ' => Ok(AlphabeticPresentationForms::HebrewLetterAlefWithPatah),
            'אָ' => Ok(AlphabeticPresentationForms::HebrewLetterAlefWithQamats),
            'אּ' => Ok(AlphabeticPresentationForms::HebrewLetterAlefWithMapiq),
            'בּ' => Ok(AlphabeticPresentationForms::HebrewLetterBetWithDagesh),
            'גּ' => Ok(AlphabeticPresentationForms::HebrewLetterGimelWithDagesh),
            'דּ' => Ok(AlphabeticPresentationForms::HebrewLetterDaletWithDagesh),
            'הּ' => Ok(AlphabeticPresentationForms::HebrewLetterHeWithMapiq),
            'וּ' => Ok(AlphabeticPresentationForms::HebrewLetterVavWithDagesh),
            'זּ' => Ok(AlphabeticPresentationForms::HebrewLetterZayinWithDagesh),
            'טּ' => Ok(AlphabeticPresentationForms::HebrewLetterTetWithDagesh),
            'יּ' => Ok(AlphabeticPresentationForms::HebrewLetterYodWithDagesh),
            'ךּ' => Ok(AlphabeticPresentationForms::HebrewLetterFinalKafWithDagesh),
            'כּ' => Ok(AlphabeticPresentationForms::HebrewLetterKafWithDagesh),
            'לּ' => Ok(AlphabeticPresentationForms::HebrewLetterLamedWithDagesh),
            'מּ' => Ok(AlphabeticPresentationForms::HebrewLetterMemWithDagesh),
            'נּ' => Ok(AlphabeticPresentationForms::HebrewLetterNunWithDagesh),
            'סּ' => Ok(AlphabeticPresentationForms::HebrewLetterSamekhWithDagesh),
            'ףּ' => Ok(AlphabeticPresentationForms::HebrewLetterFinalPeWithDagesh),
            'פּ' => Ok(AlphabeticPresentationForms::HebrewLetterPeWithDagesh),
            'צּ' => Ok(AlphabeticPresentationForms::HebrewLetterTsadiWithDagesh),
            'קּ' => Ok(AlphabeticPresentationForms::HebrewLetterQofWithDagesh),
            'רּ' => Ok(AlphabeticPresentationForms::HebrewLetterReshWithDagesh),
            'שּ' => Ok(AlphabeticPresentationForms::HebrewLetterShinWithDagesh),
            'תּ' => Ok(AlphabeticPresentationForms::HebrewLetterTavWithDagesh),
            'וֹ' => Ok(AlphabeticPresentationForms::HebrewLetterVavWithHolam),
            'בֿ' => Ok(AlphabeticPresentationForms::HebrewLetterBetWithRafe),
            'כֿ' => Ok(AlphabeticPresentationForms::HebrewLetterKafWithRafe),
            'פֿ' => Ok(AlphabeticPresentationForms::HebrewLetterPeWithRafe),
            _ => Err(()),
        }
    }
}

impl Into<u32> for AlphabeticPresentationForms {
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

impl std::convert::TryFrom<u32> for AlphabeticPresentationForms {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for AlphabeticPresentationForms {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl AlphabeticPresentationForms {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        AlphabeticPresentationForms::LatinSmallLigatureFf
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("AlphabeticPresentationForms{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
