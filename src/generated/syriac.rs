/// \u{700} → \u{74f}\
///\
/// ܀ ܁ ܂ ܃ ܄ ܅ ܆ ܇ ܈ ܉ ܊ ܋ ܌ ܍ ܏ ܐ
/// ܑ ܒ ܓ ܔ ܕ ܖ ܗ ܘ ܙ ܚ ܛ ܜ ܝ ܞ ܟ ܠ
/// ܡ ܢ ܣ ܤ ܥ ܦ ܧ ܨ ܩ ܪ ܫ ܬ ܭ ܮ ܯ ܰ
/// ܱ ܲ ܳ ܴ ܵ ܶ ܷ ܸ ܹ ܺ ܻ ܼ ܽ ܾ ܿ ݀
/// ݁ ݂ ݃ ݄ ݅ ݆ ݇ ݈ ݉ ݊ ݍ ݎ
pub mod constants {
    /// \u{700}: '܀'
    pub const END_OF_PARAGRAPH: char = '܀';
    /// \u{701}: '܁'
    pub const SUPRALINEAR_FULL_STOP: char = '܁';
    /// \u{702}: '܂'
    pub const SUBLINEAR_FULL_STOP: char = '܂';
    /// \u{703}: '܃'
    pub const SUPRALINEAR_COLON: char = '܃';
    /// \u{704}: '܄'
    pub const SUBLINEAR_COLON: char = '܄';
    /// \u{705}: '܅'
    pub const HORIZONTAL_COLON: char = '܅';
    /// \u{706}: '܆'
    pub const COLON_SKEWED_LEFT: char = '܆';
    /// \u{707}: '܇'
    pub const COLON_SKEWED_RIGHT: char = '܇';
    /// \u{708}: '܈'
    pub const SUPRALINEAR_COLON_SKEWED_LEFT: char = '܈';
    /// \u{709}: '܉'
    pub const SUBLINEAR_COLON_SKEWED_RIGHT: char = '܉';
    /// \u{70a}: '܊'
    pub const CONTRACTION: char = '܊';
    /// \u{70b}: '܋'
    pub const HARKLEAN_OBELUS: char = '܋';
    /// \u{70c}: '܌'
    pub const HARKLEAN_METOBELUS: char = '܌';
    /// \u{70d}: '܍'
    pub const HARKLEAN_ASTERISCUS: char = '܍';
    /// \u{70f}: '܏'
    pub const ABBREVIATION_MARK: char = '܏';
    /// \u{710}: 'ܐ'
    pub const LETTER_ALAPH: char = 'ܐ';
    /// \u{711}: 'ܑ'
    pub const LETTER_SUPERSCRIPT_ALAPH: char = 'ܑ';
    /// \u{712}: 'ܒ'
    pub const LETTER_BETH: char = 'ܒ';
    /// \u{713}: 'ܓ'
    pub const LETTER_GAMAL: char = 'ܓ';
    /// \u{714}: 'ܔ'
    pub const LETTER_GAMAL_GARSHUNI: char = 'ܔ';
    /// \u{715}: 'ܕ'
    pub const LETTER_DALATH: char = 'ܕ';
    /// \u{716}: 'ܖ'
    pub const LETTER_DOTLESS_DALATH_RISH: char = 'ܖ';
    /// \u{717}: 'ܗ'
    pub const LETTER_HE: char = 'ܗ';
    /// \u{718}: 'ܘ'
    pub const LETTER_WAW: char = 'ܘ';
    /// \u{719}: 'ܙ'
    pub const LETTER_ZAIN: char = 'ܙ';
    /// \u{71a}: 'ܚ'
    pub const LETTER_HETH: char = 'ܚ';
    /// \u{71b}: 'ܛ'
    pub const LETTER_TETH: char = 'ܛ';
    /// \u{71c}: 'ܜ'
    pub const LETTER_TETH_GARSHUNI: char = 'ܜ';
    /// \u{71d}: 'ܝ'
    pub const LETTER_YUDH: char = 'ܝ';
    /// \u{71e}: 'ܞ'
    pub const LETTER_YUDH_HE: char = 'ܞ';
    /// \u{71f}: 'ܟ'
    pub const LETTER_KAPH: char = 'ܟ';
    /// \u{720}: 'ܠ'
    pub const LETTER_LAMADH: char = 'ܠ';
    /// \u{721}: 'ܡ'
    pub const LETTER_MIM: char = 'ܡ';
    /// \u{722}: 'ܢ'
    pub const LETTER_NUN: char = 'ܢ';
    /// \u{723}: 'ܣ'
    pub const LETTER_SEMKATH: char = 'ܣ';
    /// \u{724}: 'ܤ'
    pub const LETTER_FINAL_SEMKATH: char = 'ܤ';
    /// \u{725}: 'ܥ'
    pub const LETTER_E: char = 'ܥ';
    /// \u{726}: 'ܦ'
    pub const LETTER_PE: char = 'ܦ';
    /// \u{727}: 'ܧ'
    pub const LETTER_REVERSED_PE: char = 'ܧ';
    /// \u{728}: 'ܨ'
    pub const LETTER_SADHE: char = 'ܨ';
    /// \u{729}: 'ܩ'
    pub const LETTER_QAPH: char = 'ܩ';
    /// \u{72a}: 'ܪ'
    pub const LETTER_RISH: char = 'ܪ';
    /// \u{72b}: 'ܫ'
    pub const LETTER_SHIN: char = 'ܫ';
    /// \u{72c}: 'ܬ'
    pub const LETTER_TAW: char = 'ܬ';
    /// \u{72d}: 'ܭ'
    pub const LETTER_PERSIAN_BHETH: char = 'ܭ';
    /// \u{72e}: 'ܮ'
    pub const LETTER_PERSIAN_GHAMAL: char = 'ܮ';
    /// \u{72f}: 'ܯ'
    pub const LETTER_PERSIAN_DHALATH: char = 'ܯ';
    /// \u{730}: 'ܰ'
    pub const PTHAHA_ABOVE: char = 'ܰ';
    /// \u{731}: 'ܱ'
    pub const PTHAHA_BELOW: char = 'ܱ';
    /// \u{732}: 'ܲ'
    pub const PTHAHA_DOTTED: char = 'ܲ';
    /// \u{733}: 'ܳ'
    pub const ZQAPHA_ABOVE: char = 'ܳ';
    /// \u{734}: 'ܴ'
    pub const ZQAPHA_BELOW: char = 'ܴ';
    /// \u{735}: 'ܵ'
    pub const ZQAPHA_DOTTED: char = 'ܵ';
    /// \u{736}: 'ܶ'
    pub const RBASA_ABOVE: char = 'ܶ';
    /// \u{737}: 'ܷ'
    pub const RBASA_BELOW: char = 'ܷ';
    /// \u{738}: 'ܸ'
    pub const DOTTED_ZLAMA_HORIZONTAL: char = 'ܸ';
    /// \u{739}: 'ܹ'
    pub const DOTTED_ZLAMA_ANGULAR: char = 'ܹ';
    /// \u{73a}: 'ܺ'
    pub const HBASA_ABOVE: char = 'ܺ';
    /// \u{73b}: 'ܻ'
    pub const HBASA_BELOW: char = 'ܻ';
    /// \u{73c}: 'ܼ'
    pub const HBASA_DASH_ESASA_DOTTED: char = 'ܼ';
    /// \u{73d}: 'ܽ'
    pub const ESASA_ABOVE: char = 'ܽ';
    /// \u{73e}: 'ܾ'
    pub const ESASA_BELOW: char = 'ܾ';
    /// \u{73f}: 'ܿ'
    pub const RWAHA: char = 'ܿ';
    /// \u{740}: '݀'
    pub const FEMININE_DOT: char = '݀';
    /// \u{741}: '݁'
    pub const QUSHSHAYA: char = '݁';
    /// \u{742}: '݂'
    pub const RUKKAKHA: char = '݂';
    /// \u{743}: '݃'
    pub const TWO_VERTICAL_DOTS_ABOVE: char = '݃';
    /// \u{744}: '݄'
    pub const TWO_VERTICAL_DOTS_BELOW: char = '݄';
    /// \u{745}: '݅'
    pub const THREE_DOTS_ABOVE: char = '݅';
    /// \u{746}: '݆'
    pub const THREE_DOTS_BELOW: char = '݆';
    /// \u{747}: '݇'
    pub const OBLIQUE_LINE_ABOVE: char = '݇';
    /// \u{748}: '݈'
    pub const OBLIQUE_LINE_BELOW: char = '݈';
    /// \u{749}: '݉'
    pub const MUSIC: char = '݉';
    /// \u{74a}: '݊'
    pub const BARREKH: char = '݊';
    /// \u{74d}: 'ݍ'
    pub const LETTER_SOGDIAN_ZHAIN: char = 'ݍ';
    /// \u{74e}: 'ݎ'
    pub const LETTER_SOGDIAN_KHAPH: char = 'ݎ';
}

/// \u{700} → \u{74f}\
///\
/// ܀ ܁ ܂ ܃ ܄ ܅ ܆ ܇ ܈ ܉ ܊ ܋ ܌ ܍ ܏ ܐ
/// ܑ ܒ ܓ ܔ ܕ ܖ ܗ ܘ ܙ ܚ ܛ ܜ ܝ ܞ ܟ ܠ
/// ܡ ܢ ܣ ܤ ܥ ܦ ܧ ܨ ܩ ܪ ܫ ܬ ܭ ܮ ܯ ܰ
/// ܱ ܲ ܳ ܴ ܵ ܶ ܷ ܸ ܹ ܺ ܻ ܼ ܽ ܾ ܿ ݀
/// ݁ ݂ ݃ ݄ ݅ ݆ ݇ ݈ ݉ ݊ ݍ ݎ
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
        use constants::*;
        match self {
            Syriac::EndOfParagraph => END_OF_PARAGRAPH,
            Syriac::SupralinearFullStop => SUPRALINEAR_FULL_STOP,
            Syriac::SublinearFullStop => SUBLINEAR_FULL_STOP,
            Syriac::SupralinearColon => SUPRALINEAR_COLON,
            Syriac::SublinearColon => SUBLINEAR_COLON,
            Syriac::HorizontalColon => HORIZONTAL_COLON,
            Syriac::ColonSkewedLeft => COLON_SKEWED_LEFT,
            Syriac::ColonSkewedRight => COLON_SKEWED_RIGHT,
            Syriac::SupralinearColonSkewedLeft => SUPRALINEAR_COLON_SKEWED_LEFT,
            Syriac::SublinearColonSkewedRight => SUBLINEAR_COLON_SKEWED_RIGHT,
            Syriac::Contraction => CONTRACTION,
            Syriac::HarkleanObelus => HARKLEAN_OBELUS,
            Syriac::HarkleanMetobelus => HARKLEAN_METOBELUS,
            Syriac::HarkleanAsteriscus => HARKLEAN_ASTERISCUS,
            Syriac::AbbreviationMark => ABBREVIATION_MARK,
            Syriac::LetterAlaph => LETTER_ALAPH,
            Syriac::LetterSuperscriptAlaph => LETTER_SUPERSCRIPT_ALAPH,
            Syriac::LetterBeth => LETTER_BETH,
            Syriac::LetterGamal => LETTER_GAMAL,
            Syriac::LetterGamalGarshuni => LETTER_GAMAL_GARSHUNI,
            Syriac::LetterDalath => LETTER_DALATH,
            Syriac::LetterDotlessDalathRish => LETTER_DOTLESS_DALATH_RISH,
            Syriac::LetterHe => LETTER_HE,
            Syriac::LetterWaw => LETTER_WAW,
            Syriac::LetterZain => LETTER_ZAIN,
            Syriac::LetterHeth => LETTER_HETH,
            Syriac::LetterTeth => LETTER_TETH,
            Syriac::LetterTethGarshuni => LETTER_TETH_GARSHUNI,
            Syriac::LetterYudh => LETTER_YUDH,
            Syriac::LetterYudhHe => LETTER_YUDH_HE,
            Syriac::LetterKaph => LETTER_KAPH,
            Syriac::LetterLamadh => LETTER_LAMADH,
            Syriac::LetterMim => LETTER_MIM,
            Syriac::LetterNun => LETTER_NUN,
            Syriac::LetterSemkath => LETTER_SEMKATH,
            Syriac::LetterFinalSemkath => LETTER_FINAL_SEMKATH,
            Syriac::LetterE => LETTER_E,
            Syriac::LetterPe => LETTER_PE,
            Syriac::LetterReversedPe => LETTER_REVERSED_PE,
            Syriac::LetterSadhe => LETTER_SADHE,
            Syriac::LetterQaph => LETTER_QAPH,
            Syriac::LetterRish => LETTER_RISH,
            Syriac::LetterShin => LETTER_SHIN,
            Syriac::LetterTaw => LETTER_TAW,
            Syriac::LetterPersianBheth => LETTER_PERSIAN_BHETH,
            Syriac::LetterPersianGhamal => LETTER_PERSIAN_GHAMAL,
            Syriac::LetterPersianDhalath => LETTER_PERSIAN_DHALATH,
            Syriac::PthahaAbove => PTHAHA_ABOVE,
            Syriac::PthahaBelow => PTHAHA_BELOW,
            Syriac::PthahaDotted => PTHAHA_DOTTED,
            Syriac::ZqaphaAbove => ZQAPHA_ABOVE,
            Syriac::ZqaphaBelow => ZQAPHA_BELOW,
            Syriac::ZqaphaDotted => ZQAPHA_DOTTED,
            Syriac::RbasaAbove => RBASA_ABOVE,
            Syriac::RbasaBelow => RBASA_BELOW,
            Syriac::DottedZlamaHorizontal => DOTTED_ZLAMA_HORIZONTAL,
            Syriac::DottedZlamaAngular => DOTTED_ZLAMA_ANGULAR,
            Syriac::HbasaAbove => HBASA_ABOVE,
            Syriac::HbasaBelow => HBASA_BELOW,
            Syriac::HbasaDashEsasaDotted => HBASA_DASH_ESASA_DOTTED,
            Syriac::EsasaAbove => ESASA_ABOVE,
            Syriac::EsasaBelow => ESASA_BELOW,
            Syriac::Rwaha => RWAHA,
            Syriac::FeminineDot => FEMININE_DOT,
            Syriac::Qushshaya => QUSHSHAYA,
            Syriac::Rukkakha => RUKKAKHA,
            Syriac::TwoVerticalDotsAbove => TWO_VERTICAL_DOTS_ABOVE,
            Syriac::TwoVerticalDotsBelow => TWO_VERTICAL_DOTS_BELOW,
            Syriac::ThreeDotsAbove => THREE_DOTS_ABOVE,
            Syriac::ThreeDotsBelow => THREE_DOTS_BELOW,
            Syriac::ObliqueLineAbove => OBLIQUE_LINE_ABOVE,
            Syriac::ObliqueLineBelow => OBLIQUE_LINE_BELOW,
            Syriac::Music => MUSIC,
            Syriac::Barrekh => BARREKH,
            Syriac::LetterSogdianZhain => LETTER_SOGDIAN_ZHAIN,
            Syriac::LetterSogdianKhaph => LETTER_SOGDIAN_KHAPH,
        }
    }
}

impl std::convert::TryFrom<char> for Syriac {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            END_OF_PARAGRAPH => Ok(Syriac::EndOfParagraph),
            SUPRALINEAR_FULL_STOP => Ok(Syriac::SupralinearFullStop),
            SUBLINEAR_FULL_STOP => Ok(Syriac::SublinearFullStop),
            SUPRALINEAR_COLON => Ok(Syriac::SupralinearColon),
            SUBLINEAR_COLON => Ok(Syriac::SublinearColon),
            HORIZONTAL_COLON => Ok(Syriac::HorizontalColon),
            COLON_SKEWED_LEFT => Ok(Syriac::ColonSkewedLeft),
            COLON_SKEWED_RIGHT => Ok(Syriac::ColonSkewedRight),
            SUPRALINEAR_COLON_SKEWED_LEFT => Ok(Syriac::SupralinearColonSkewedLeft),
            SUBLINEAR_COLON_SKEWED_RIGHT => Ok(Syriac::SublinearColonSkewedRight),
            CONTRACTION => Ok(Syriac::Contraction),
            HARKLEAN_OBELUS => Ok(Syriac::HarkleanObelus),
            HARKLEAN_METOBELUS => Ok(Syriac::HarkleanMetobelus),
            HARKLEAN_ASTERISCUS => Ok(Syriac::HarkleanAsteriscus),
            ABBREVIATION_MARK => Ok(Syriac::AbbreviationMark),
            LETTER_ALAPH => Ok(Syriac::LetterAlaph),
            LETTER_SUPERSCRIPT_ALAPH => Ok(Syriac::LetterSuperscriptAlaph),
            LETTER_BETH => Ok(Syriac::LetterBeth),
            LETTER_GAMAL => Ok(Syriac::LetterGamal),
            LETTER_GAMAL_GARSHUNI => Ok(Syriac::LetterGamalGarshuni),
            LETTER_DALATH => Ok(Syriac::LetterDalath),
            LETTER_DOTLESS_DALATH_RISH => Ok(Syriac::LetterDotlessDalathRish),
            LETTER_HE => Ok(Syriac::LetterHe),
            LETTER_WAW => Ok(Syriac::LetterWaw),
            LETTER_ZAIN => Ok(Syriac::LetterZain),
            LETTER_HETH => Ok(Syriac::LetterHeth),
            LETTER_TETH => Ok(Syriac::LetterTeth),
            LETTER_TETH_GARSHUNI => Ok(Syriac::LetterTethGarshuni),
            LETTER_YUDH => Ok(Syriac::LetterYudh),
            LETTER_YUDH_HE => Ok(Syriac::LetterYudhHe),
            LETTER_KAPH => Ok(Syriac::LetterKaph),
            LETTER_LAMADH => Ok(Syriac::LetterLamadh),
            LETTER_MIM => Ok(Syriac::LetterMim),
            LETTER_NUN => Ok(Syriac::LetterNun),
            LETTER_SEMKATH => Ok(Syriac::LetterSemkath),
            LETTER_FINAL_SEMKATH => Ok(Syriac::LetterFinalSemkath),
            LETTER_E => Ok(Syriac::LetterE),
            LETTER_PE => Ok(Syriac::LetterPe),
            LETTER_REVERSED_PE => Ok(Syriac::LetterReversedPe),
            LETTER_SADHE => Ok(Syriac::LetterSadhe),
            LETTER_QAPH => Ok(Syriac::LetterQaph),
            LETTER_RISH => Ok(Syriac::LetterRish),
            LETTER_SHIN => Ok(Syriac::LetterShin),
            LETTER_TAW => Ok(Syriac::LetterTaw),
            LETTER_PERSIAN_BHETH => Ok(Syriac::LetterPersianBheth),
            LETTER_PERSIAN_GHAMAL => Ok(Syriac::LetterPersianGhamal),
            LETTER_PERSIAN_DHALATH => Ok(Syriac::LetterPersianDhalath),
            PTHAHA_ABOVE => Ok(Syriac::PthahaAbove),
            PTHAHA_BELOW => Ok(Syriac::PthahaBelow),
            PTHAHA_DOTTED => Ok(Syriac::PthahaDotted),
            ZQAPHA_ABOVE => Ok(Syriac::ZqaphaAbove),
            ZQAPHA_BELOW => Ok(Syriac::ZqaphaBelow),
            ZQAPHA_DOTTED => Ok(Syriac::ZqaphaDotted),
            RBASA_ABOVE => Ok(Syriac::RbasaAbove),
            RBASA_BELOW => Ok(Syriac::RbasaBelow),
            DOTTED_ZLAMA_HORIZONTAL => Ok(Syriac::DottedZlamaHorizontal),
            DOTTED_ZLAMA_ANGULAR => Ok(Syriac::DottedZlamaAngular),
            HBASA_ABOVE => Ok(Syriac::HbasaAbove),
            HBASA_BELOW => Ok(Syriac::HbasaBelow),
            HBASA_DASH_ESASA_DOTTED => Ok(Syriac::HbasaDashEsasaDotted),
            ESASA_ABOVE => Ok(Syriac::EsasaAbove),
            ESASA_BELOW => Ok(Syriac::EsasaBelow),
            RWAHA => Ok(Syriac::Rwaha),
            FEMININE_DOT => Ok(Syriac::FeminineDot),
            QUSHSHAYA => Ok(Syriac::Qushshaya),
            RUKKAKHA => Ok(Syriac::Rukkakha),
            TWO_VERTICAL_DOTS_ABOVE => Ok(Syriac::TwoVerticalDotsAbove),
            TWO_VERTICAL_DOTS_BELOW => Ok(Syriac::TwoVerticalDotsBelow),
            THREE_DOTS_ABOVE => Ok(Syriac::ThreeDotsAbove),
            THREE_DOTS_BELOW => Ok(Syriac::ThreeDotsBelow),
            OBLIQUE_LINE_ABOVE => Ok(Syriac::ObliqueLineAbove),
            OBLIQUE_LINE_BELOW => Ok(Syriac::ObliqueLineBelow),
            MUSIC => Ok(Syriac::Music),
            BARREKH => Ok(Syriac::Barrekh),
            LETTER_SOGDIAN_ZHAIN => Ok(Syriac::LetterSogdianZhain),
            LETTER_SOGDIAN_KHAPH => Ok(Syriac::LetterSogdianKhaph),
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

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Syriac::EndOfParagraph => "syriac end of paragraph",
            Syriac::SupralinearFullStop => "syriac supralinear full stop",
            Syriac::SublinearFullStop => "syriac sublinear full stop",
            Syriac::SupralinearColon => "syriac supralinear colon",
            Syriac::SublinearColon => "syriac sublinear colon",
            Syriac::HorizontalColon => "syriac horizontal colon",
            Syriac::ColonSkewedLeft => "syriac colon skewed left",
            Syriac::ColonSkewedRight => "syriac colon skewed right",
            Syriac::SupralinearColonSkewedLeft => "syriac supralinear colon skewed left",
            Syriac::SublinearColonSkewedRight => "syriac sublinear colon skewed right",
            Syriac::Contraction => "syriac contraction",
            Syriac::HarkleanObelus => "syriac harklean obelus",
            Syriac::HarkleanMetobelus => "syriac harklean metobelus",
            Syriac::HarkleanAsteriscus => "syriac harklean asteriscus",
            Syriac::AbbreviationMark => "syriac abbreviation mark",
            Syriac::LetterAlaph => "syriac letter alaph",
            Syriac::LetterSuperscriptAlaph => "syriac letter superscript alaph",
            Syriac::LetterBeth => "syriac letter beth",
            Syriac::LetterGamal => "syriac letter gamal",
            Syriac::LetterGamalGarshuni => "syriac letter gamal garshuni",
            Syriac::LetterDalath => "syriac letter dalath",
            Syriac::LetterDotlessDalathRish => "syriac letter dotless dalath rish",
            Syriac::LetterHe => "syriac letter he",
            Syriac::LetterWaw => "syriac letter waw",
            Syriac::LetterZain => "syriac letter zain",
            Syriac::LetterHeth => "syriac letter heth",
            Syriac::LetterTeth => "syriac letter teth",
            Syriac::LetterTethGarshuni => "syriac letter teth garshuni",
            Syriac::LetterYudh => "syriac letter yudh",
            Syriac::LetterYudhHe => "syriac letter yudh he",
            Syriac::LetterKaph => "syriac letter kaph",
            Syriac::LetterLamadh => "syriac letter lamadh",
            Syriac::LetterMim => "syriac letter mim",
            Syriac::LetterNun => "syriac letter nun",
            Syriac::LetterSemkath => "syriac letter semkath",
            Syriac::LetterFinalSemkath => "syriac letter final semkath",
            Syriac::LetterE => "syriac letter e",
            Syriac::LetterPe => "syriac letter pe",
            Syriac::LetterReversedPe => "syriac letter reversed pe",
            Syriac::LetterSadhe => "syriac letter sadhe",
            Syriac::LetterQaph => "syriac letter qaph",
            Syriac::LetterRish => "syriac letter rish",
            Syriac::LetterShin => "syriac letter shin",
            Syriac::LetterTaw => "syriac letter taw",
            Syriac::LetterPersianBheth => "syriac letter persian bheth",
            Syriac::LetterPersianGhamal => "syriac letter persian ghamal",
            Syriac::LetterPersianDhalath => "syriac letter persian dhalath",
            Syriac::PthahaAbove => "syriac pthaha above",
            Syriac::PthahaBelow => "syriac pthaha below",
            Syriac::PthahaDotted => "syriac pthaha dotted",
            Syriac::ZqaphaAbove => "syriac zqapha above",
            Syriac::ZqaphaBelow => "syriac zqapha below",
            Syriac::ZqaphaDotted => "syriac zqapha dotted",
            Syriac::RbasaAbove => "syriac rbasa above",
            Syriac::RbasaBelow => "syriac rbasa below",
            Syriac::DottedZlamaHorizontal => "syriac dotted zlama horizontal",
            Syriac::DottedZlamaAngular => "syriac dotted zlama angular",
            Syriac::HbasaAbove => "syriac hbasa above",
            Syriac::HbasaBelow => "syriac hbasa below",
            Syriac::HbasaDashEsasaDotted => "syriac hbasa-esasa dotted",
            Syriac::EsasaAbove => "syriac esasa above",
            Syriac::EsasaBelow => "syriac esasa below",
            Syriac::Rwaha => "syriac rwaha",
            Syriac::FeminineDot => "syriac feminine dot",
            Syriac::Qushshaya => "syriac qushshaya",
            Syriac::Rukkakha => "syriac rukkakha",
            Syriac::TwoVerticalDotsAbove => "syriac two vertical dots above",
            Syriac::TwoVerticalDotsBelow => "syriac two vertical dots below",
            Syriac::ThreeDotsAbove => "syriac three dots above",
            Syriac::ThreeDotsBelow => "syriac three dots below",
            Syriac::ObliqueLineAbove => "syriac oblique line above",
            Syriac::ObliqueLineBelow => "syriac oblique line below",
            Syriac::Music => "syriac music",
            Syriac::Barrekh => "syriac barrekh",
            Syriac::LetterSogdianZhain => "syriac letter sogdian zhain",
            Syriac::LetterSogdianKhaph => "syriac letter sogdian khaph",
        }
    }
}
