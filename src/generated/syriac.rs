
/// An enum to represent all characters in the Syriac block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Syriac {
    /// \u{700}: '܀'
    EndOfParagraph,
    /// \u{701}: '܁'
    SupralinearFullStop,
    /// \u{702}: '܂'
    SublinearFullStop,
    /// \u{703}: '܃'
    SupralinearColon,
    /// \u{704}: '܄'
    SublinearColon,
    /// \u{705}: '܅'
    HorizontalColon,
    /// \u{706}: '܆'
    ColonSkewedLeft,
    /// \u{707}: '܇'
    ColonSkewedRight,
    /// \u{708}: '܈'
    SupralinearColonSkewedLeft,
    /// \u{709}: '܉'
    SublinearColonSkewedRight,
    /// \u{70a}: '܊'
    Contraction,
    /// \u{70b}: '܋'
    HarkleanObelus,
    /// \u{70c}: '܌'
    HarkleanMetobelus,
    /// \u{70d}: '܍'
    HarkleanAsteriscus,
    /// \u{70f}: '܏'
    AbbreviationMark,
    /// \u{710}: 'ܐ'
    LetterAlaph,
    /// \u{711}: 'ܑ'
    LetterSuperscriptAlaph,
    /// \u{712}: 'ܒ'
    LetterBeth,
    /// \u{713}: 'ܓ'
    LetterGamal,
    /// \u{714}: 'ܔ'
    LetterGamalGarshuni,
    /// \u{715}: 'ܕ'
    LetterDalath,
    /// \u{716}: 'ܖ'
    LetterDotlessDalathRish,
    /// \u{717}: 'ܗ'
    LetterHe,
    /// \u{718}: 'ܘ'
    LetterWaw,
    /// \u{719}: 'ܙ'
    LetterZain,
    /// \u{71a}: 'ܚ'
    LetterHeth,
    /// \u{71b}: 'ܛ'
    LetterTeth,
    /// \u{71c}: 'ܜ'
    LetterTethGarshuni,
    /// \u{71d}: 'ܝ'
    LetterYudh,
    /// \u{71e}: 'ܞ'
    LetterYudhHe,
    /// \u{71f}: 'ܟ'
    LetterKaph,
    /// \u{720}: 'ܠ'
    LetterLamadh,
    /// \u{721}: 'ܡ'
    LetterMim,
    /// \u{722}: 'ܢ'
    LetterNun,
    /// \u{723}: 'ܣ'
    LetterSemkath,
    /// \u{724}: 'ܤ'
    LetterFinalSemkath,
    /// \u{725}: 'ܥ'
    LetterE,
    /// \u{726}: 'ܦ'
    LetterPe,
    /// \u{727}: 'ܧ'
    LetterReversedPe,
    /// \u{728}: 'ܨ'
    LetterSadhe,
    /// \u{729}: 'ܩ'
    LetterQaph,
    /// \u{72a}: 'ܪ'
    LetterRish,
    /// \u{72b}: 'ܫ'
    LetterShin,
    /// \u{72c}: 'ܬ'
    LetterTaw,
    /// \u{72d}: 'ܭ'
    LetterPersianBheth,
    /// \u{72e}: 'ܮ'
    LetterPersianGhamal,
    /// \u{72f}: 'ܯ'
    LetterPersianDhalath,
    /// \u{730}: 'ܰ'
    PthahaAbove,
    /// \u{731}: 'ܱ'
    PthahaBelow,
    /// \u{732}: 'ܲ'
    PthahaDotted,
    /// \u{733}: 'ܳ'
    ZqaphaAbove,
    /// \u{734}: 'ܴ'
    ZqaphaBelow,
    /// \u{735}: 'ܵ'
    ZqaphaDotted,
    /// \u{736}: 'ܶ'
    RbasaAbove,
    /// \u{737}: 'ܷ'
    RbasaBelow,
    /// \u{738}: 'ܸ'
    DottedZlamaHorizontal,
    /// \u{739}: 'ܹ'
    DottedZlamaAngular,
    /// \u{73a}: 'ܺ'
    HbasaAbove,
    /// \u{73b}: 'ܻ'
    HbasaBelow,
    /// \u{73c}: 'ܼ'
    HbasaDashEsasaDotted,
    /// \u{73d}: 'ܽ'
    EsasaAbove,
    /// \u{73e}: 'ܾ'
    EsasaBelow,
    /// \u{73f}: 'ܿ'
    Rwaha,
    /// \u{740}: '݀'
    FeminineDot,
    /// \u{741}: '݁'
    Qushshaya,
    /// \u{742}: '݂'
    Rukkakha,
    /// \u{743}: '݃'
    TwoVerticalDotsAbove,
    /// \u{744}: '݄'
    TwoVerticalDotsBelow,
    /// \u{745}: '݅'
    ThreeDotsAbove,
    /// \u{746}: '݆'
    ThreeDotsBelow,
    /// \u{747}: '݇'
    ObliqueLineAbove,
    /// \u{748}: '݈'
    ObliqueLineBelow,
    /// \u{749}: '݉'
    Music,
    /// \u{74a}: '݊'
    Barrekh,
    /// \u{74d}: 'ݍ'
    LetterSogdianZhain,
    /// \u{74e}: 'ݎ'
    LetterSogdianKhaph,
}

impl Into<char> for Syriac {
    fn into(self) -> char {
        match self {
            Syriac::EndOfParagraph => '܀',
            Syriac::SupralinearFullStop => '܁',
            Syriac::SublinearFullStop => '܂',
            Syriac::SupralinearColon => '܃',
            Syriac::SublinearColon => '܄',
            Syriac::HorizontalColon => '܅',
            Syriac::ColonSkewedLeft => '܆',
            Syriac::ColonSkewedRight => '܇',
            Syriac::SupralinearColonSkewedLeft => '܈',
            Syriac::SublinearColonSkewedRight => '܉',
            Syriac::Contraction => '܊',
            Syriac::HarkleanObelus => '܋',
            Syriac::HarkleanMetobelus => '܌',
            Syriac::HarkleanAsteriscus => '܍',
            Syriac::AbbreviationMark => '܏',
            Syriac::LetterAlaph => 'ܐ',
            Syriac::LetterSuperscriptAlaph => 'ܑ',
            Syriac::LetterBeth => 'ܒ',
            Syriac::LetterGamal => 'ܓ',
            Syriac::LetterGamalGarshuni => 'ܔ',
            Syriac::LetterDalath => 'ܕ',
            Syriac::LetterDotlessDalathRish => 'ܖ',
            Syriac::LetterHe => 'ܗ',
            Syriac::LetterWaw => 'ܘ',
            Syriac::LetterZain => 'ܙ',
            Syriac::LetterHeth => 'ܚ',
            Syriac::LetterTeth => 'ܛ',
            Syriac::LetterTethGarshuni => 'ܜ',
            Syriac::LetterYudh => 'ܝ',
            Syriac::LetterYudhHe => 'ܞ',
            Syriac::LetterKaph => 'ܟ',
            Syriac::LetterLamadh => 'ܠ',
            Syriac::LetterMim => 'ܡ',
            Syriac::LetterNun => 'ܢ',
            Syriac::LetterSemkath => 'ܣ',
            Syriac::LetterFinalSemkath => 'ܤ',
            Syriac::LetterE => 'ܥ',
            Syriac::LetterPe => 'ܦ',
            Syriac::LetterReversedPe => 'ܧ',
            Syriac::LetterSadhe => 'ܨ',
            Syriac::LetterQaph => 'ܩ',
            Syriac::LetterRish => 'ܪ',
            Syriac::LetterShin => 'ܫ',
            Syriac::LetterTaw => 'ܬ',
            Syriac::LetterPersianBheth => 'ܭ',
            Syriac::LetterPersianGhamal => 'ܮ',
            Syriac::LetterPersianDhalath => 'ܯ',
            Syriac::PthahaAbove => 'ܰ',
            Syriac::PthahaBelow => 'ܱ',
            Syriac::PthahaDotted => 'ܲ',
            Syriac::ZqaphaAbove => 'ܳ',
            Syriac::ZqaphaBelow => 'ܴ',
            Syriac::ZqaphaDotted => 'ܵ',
            Syriac::RbasaAbove => 'ܶ',
            Syriac::RbasaBelow => 'ܷ',
            Syriac::DottedZlamaHorizontal => 'ܸ',
            Syriac::DottedZlamaAngular => 'ܹ',
            Syriac::HbasaAbove => 'ܺ',
            Syriac::HbasaBelow => 'ܻ',
            Syriac::HbasaDashEsasaDotted => 'ܼ',
            Syriac::EsasaAbove => 'ܽ',
            Syriac::EsasaBelow => 'ܾ',
            Syriac::Rwaha => 'ܿ',
            Syriac::FeminineDot => '݀',
            Syriac::Qushshaya => '݁',
            Syriac::Rukkakha => '݂',
            Syriac::TwoVerticalDotsAbove => '݃',
            Syriac::TwoVerticalDotsBelow => '݄',
            Syriac::ThreeDotsAbove => '݅',
            Syriac::ThreeDotsBelow => '݆',
            Syriac::ObliqueLineAbove => '݇',
            Syriac::ObliqueLineBelow => '݈',
            Syriac::Music => '݉',
            Syriac::Barrekh => '݊',
            Syriac::LetterSogdianZhain => 'ݍ',
            Syriac::LetterSogdianKhaph => 'ݎ',
        }
    }
}

impl std::convert::TryFrom<char> for Syriac {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '܀' => Ok(Syriac::EndOfParagraph),
            '܁' => Ok(Syriac::SupralinearFullStop),
            '܂' => Ok(Syriac::SublinearFullStop),
            '܃' => Ok(Syriac::SupralinearColon),
            '܄' => Ok(Syriac::SublinearColon),
            '܅' => Ok(Syriac::HorizontalColon),
            '܆' => Ok(Syriac::ColonSkewedLeft),
            '܇' => Ok(Syriac::ColonSkewedRight),
            '܈' => Ok(Syriac::SupralinearColonSkewedLeft),
            '܉' => Ok(Syriac::SublinearColonSkewedRight),
            '܊' => Ok(Syriac::Contraction),
            '܋' => Ok(Syriac::HarkleanObelus),
            '܌' => Ok(Syriac::HarkleanMetobelus),
            '܍' => Ok(Syriac::HarkleanAsteriscus),
            '܏' => Ok(Syriac::AbbreviationMark),
            'ܐ' => Ok(Syriac::LetterAlaph),
            'ܑ' => Ok(Syriac::LetterSuperscriptAlaph),
            'ܒ' => Ok(Syriac::LetterBeth),
            'ܓ' => Ok(Syriac::LetterGamal),
            'ܔ' => Ok(Syriac::LetterGamalGarshuni),
            'ܕ' => Ok(Syriac::LetterDalath),
            'ܖ' => Ok(Syriac::LetterDotlessDalathRish),
            'ܗ' => Ok(Syriac::LetterHe),
            'ܘ' => Ok(Syriac::LetterWaw),
            'ܙ' => Ok(Syriac::LetterZain),
            'ܚ' => Ok(Syriac::LetterHeth),
            'ܛ' => Ok(Syriac::LetterTeth),
            'ܜ' => Ok(Syriac::LetterTethGarshuni),
            'ܝ' => Ok(Syriac::LetterYudh),
            'ܞ' => Ok(Syriac::LetterYudhHe),
            'ܟ' => Ok(Syriac::LetterKaph),
            'ܠ' => Ok(Syriac::LetterLamadh),
            'ܡ' => Ok(Syriac::LetterMim),
            'ܢ' => Ok(Syriac::LetterNun),
            'ܣ' => Ok(Syriac::LetterSemkath),
            'ܤ' => Ok(Syriac::LetterFinalSemkath),
            'ܥ' => Ok(Syriac::LetterE),
            'ܦ' => Ok(Syriac::LetterPe),
            'ܧ' => Ok(Syriac::LetterReversedPe),
            'ܨ' => Ok(Syriac::LetterSadhe),
            'ܩ' => Ok(Syriac::LetterQaph),
            'ܪ' => Ok(Syriac::LetterRish),
            'ܫ' => Ok(Syriac::LetterShin),
            'ܬ' => Ok(Syriac::LetterTaw),
            'ܭ' => Ok(Syriac::LetterPersianBheth),
            'ܮ' => Ok(Syriac::LetterPersianGhamal),
            'ܯ' => Ok(Syriac::LetterPersianDhalath),
            'ܰ' => Ok(Syriac::PthahaAbove),
            'ܱ' => Ok(Syriac::PthahaBelow),
            'ܲ' => Ok(Syriac::PthahaDotted),
            'ܳ' => Ok(Syriac::ZqaphaAbove),
            'ܴ' => Ok(Syriac::ZqaphaBelow),
            'ܵ' => Ok(Syriac::ZqaphaDotted),
            'ܶ' => Ok(Syriac::RbasaAbove),
            'ܷ' => Ok(Syriac::RbasaBelow),
            'ܸ' => Ok(Syriac::DottedZlamaHorizontal),
            'ܹ' => Ok(Syriac::DottedZlamaAngular),
            'ܺ' => Ok(Syriac::HbasaAbove),
            'ܻ' => Ok(Syriac::HbasaBelow),
            'ܼ' => Ok(Syriac::HbasaDashEsasaDotted),
            'ܽ' => Ok(Syriac::EsasaAbove),
            'ܾ' => Ok(Syriac::EsasaBelow),
            'ܿ' => Ok(Syriac::Rwaha),
            '݀' => Ok(Syriac::FeminineDot),
            '݁' => Ok(Syriac::Qushshaya),
            '݂' => Ok(Syriac::Rukkakha),
            '݃' => Ok(Syriac::TwoVerticalDotsAbove),
            '݄' => Ok(Syriac::TwoVerticalDotsBelow),
            '݅' => Ok(Syriac::ThreeDotsAbove),
            '݆' => Ok(Syriac::ThreeDotsBelow),
            '݇' => Ok(Syriac::ObliqueLineAbove),
            '݈' => Ok(Syriac::ObliqueLineBelow),
            '݉' => Ok(Syriac::Music),
            '݊' => Ok(Syriac::Barrekh),
            'ݍ' => Ok(Syriac::LetterSogdianZhain),
            'ݎ' => Ok(Syriac::LetterSogdianKhaph),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Syriac {
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

impl std::convert::TryFrom<u32> for Syriac {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Syriac {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Syriac {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Syriac::EndOfParagraph
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Syriac{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
