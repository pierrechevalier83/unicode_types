
/// An enum to represent all characters in the ArabicPresentationFormsB block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ArabicPresentationFormsB {
    /// \u{fe70}: 'ﹰ'
    ArabicFathatanIsolatedForm,
    /// \u{fe71}: 'ﹱ'
    ArabicTatweelWithFathatanAbove,
    /// \u{fe72}: 'ﹲ'
    ArabicDammatanIsolatedForm,
    /// \u{fe73}: 'ﹳ'
    ArabicTailFragment,
    /// \u{fe74}: 'ﹴ'
    ArabicKasratanIsolatedForm,
    /// \u{fe76}: 'ﹶ'
    ArabicFathaIsolatedForm,
    /// \u{fe77}: 'ﹷ'
    ArabicFathaMedialForm,
    /// \u{fe78}: 'ﹸ'
    ArabicDammaIsolatedForm,
    /// \u{fe79}: 'ﹹ'
    ArabicDammaMedialForm,
    /// \u{fe7a}: 'ﹺ'
    ArabicKasraIsolatedForm,
    /// \u{fe7b}: 'ﹻ'
    ArabicKasraMedialForm,
    /// \u{fe7c}: 'ﹼ'
    ArabicShaddaIsolatedForm,
    /// \u{fe7d}: 'ﹽ'
    ArabicShaddaMedialForm,
    /// \u{fe7e}: 'ﹾ'
    ArabicSukunIsolatedForm,
    /// \u{fe7f}: 'ﹿ'
    ArabicSukunMedialForm,
    /// \u{fe80}: 'ﺀ'
    ArabicLetterHamzaIsolatedForm,
    /// \u{fe81}: 'ﺁ'
    ArabicLetterAlefWithMaddaAboveIsolatedForm,
    /// \u{fe82}: 'ﺂ'
    ArabicLetterAlefWithMaddaAboveFinalForm,
    /// \u{fe83}: 'ﺃ'
    ArabicLetterAlefWithHamzaAboveIsolatedForm,
    /// \u{fe84}: 'ﺄ'
    ArabicLetterAlefWithHamzaAboveFinalForm,
    /// \u{fe85}: 'ﺅ'
    ArabicLetterWawWithHamzaAboveIsolatedForm,
    /// \u{fe86}: 'ﺆ'
    ArabicLetterWawWithHamzaAboveFinalForm,
    /// \u{fe87}: 'ﺇ'
    ArabicLetterAlefWithHamzaBelowIsolatedForm,
    /// \u{fe88}: 'ﺈ'
    ArabicLetterAlefWithHamzaBelowFinalForm,
    /// \u{fe89}: 'ﺉ'
    ArabicLetterYehWithHamzaAboveIsolatedForm,
    /// \u{fe8a}: 'ﺊ'
    ArabicLetterYehWithHamzaAboveFinalForm,
    /// \u{fe8b}: 'ﺋ'
    ArabicLetterYehWithHamzaAboveInitialForm,
    /// \u{fe8c}: 'ﺌ'
    ArabicLetterYehWithHamzaAboveMedialForm,
    /// \u{fe8d}: 'ﺍ'
    ArabicLetterAlefIsolatedForm,
    /// \u{fe8e}: 'ﺎ'
    ArabicLetterAlefFinalForm,
    /// \u{fe8f}: 'ﺏ'
    ArabicLetterBehIsolatedForm,
    /// \u{fe90}: 'ﺐ'
    ArabicLetterBehFinalForm,
    /// \u{fe91}: 'ﺑ'
    ArabicLetterBehInitialForm,
    /// \u{fe92}: 'ﺒ'
    ArabicLetterBehMedialForm,
    /// \u{fe93}: 'ﺓ'
    ArabicLetterTehMarbutaIsolatedForm,
    /// \u{fe94}: 'ﺔ'
    ArabicLetterTehMarbutaFinalForm,
    /// \u{fe95}: 'ﺕ'
    ArabicLetterTehIsolatedForm,
    /// \u{fe96}: 'ﺖ'
    ArabicLetterTehFinalForm,
    /// \u{fe97}: 'ﺗ'
    ArabicLetterTehInitialForm,
    /// \u{fe98}: 'ﺘ'
    ArabicLetterTehMedialForm,
    /// \u{fe99}: 'ﺙ'
    ArabicLetterThehIsolatedForm,
    /// \u{fe9a}: 'ﺚ'
    ArabicLetterThehFinalForm,
    /// \u{fe9b}: 'ﺛ'
    ArabicLetterThehInitialForm,
    /// \u{fe9c}: 'ﺜ'
    ArabicLetterThehMedialForm,
    /// \u{fe9d}: 'ﺝ'
    ArabicLetterJeemIsolatedForm,
    /// \u{fe9e}: 'ﺞ'
    ArabicLetterJeemFinalForm,
    /// \u{fe9f}: 'ﺟ'
    ArabicLetterJeemInitialForm,
    /// \u{fea0}: 'ﺠ'
    ArabicLetterJeemMedialForm,
    /// \u{fea1}: 'ﺡ'
    ArabicLetterHahIsolatedForm,
    /// \u{fea2}: 'ﺢ'
    ArabicLetterHahFinalForm,
    /// \u{fea3}: 'ﺣ'
    ArabicLetterHahInitialForm,
    /// \u{fea4}: 'ﺤ'
    ArabicLetterHahMedialForm,
    /// \u{fea5}: 'ﺥ'
    ArabicLetterKhahIsolatedForm,
    /// \u{fea6}: 'ﺦ'
    ArabicLetterKhahFinalForm,
    /// \u{fea7}: 'ﺧ'
    ArabicLetterKhahInitialForm,
    /// \u{fea8}: 'ﺨ'
    ArabicLetterKhahMedialForm,
    /// \u{fea9}: 'ﺩ'
    ArabicLetterDalIsolatedForm,
    /// \u{feaa}: 'ﺪ'
    ArabicLetterDalFinalForm,
    /// \u{feab}: 'ﺫ'
    ArabicLetterThalIsolatedForm,
    /// \u{feac}: 'ﺬ'
    ArabicLetterThalFinalForm,
    /// \u{fead}: 'ﺭ'
    ArabicLetterRehIsolatedForm,
    /// \u{feae}: 'ﺮ'
    ArabicLetterRehFinalForm,
    /// \u{feaf}: 'ﺯ'
    ArabicLetterZainIsolatedForm,
    /// \u{feb0}: 'ﺰ'
    ArabicLetterZainFinalForm,
    /// \u{feb1}: 'ﺱ'
    ArabicLetterSeenIsolatedForm,
    /// \u{feb2}: 'ﺲ'
    ArabicLetterSeenFinalForm,
    /// \u{feb3}: 'ﺳ'
    ArabicLetterSeenInitialForm,
    /// \u{feb4}: 'ﺴ'
    ArabicLetterSeenMedialForm,
    /// \u{feb5}: 'ﺵ'
    ArabicLetterSheenIsolatedForm,
    /// \u{feb6}: 'ﺶ'
    ArabicLetterSheenFinalForm,
    /// \u{feb7}: 'ﺷ'
    ArabicLetterSheenInitialForm,
    /// \u{feb8}: 'ﺸ'
    ArabicLetterSheenMedialForm,
    /// \u{feb9}: 'ﺹ'
    ArabicLetterSadIsolatedForm,
    /// \u{feba}: 'ﺺ'
    ArabicLetterSadFinalForm,
    /// \u{febb}: 'ﺻ'
    ArabicLetterSadInitialForm,
    /// \u{febc}: 'ﺼ'
    ArabicLetterSadMedialForm,
    /// \u{febd}: 'ﺽ'
    ArabicLetterDadIsolatedForm,
    /// \u{febe}: 'ﺾ'
    ArabicLetterDadFinalForm,
    /// \u{febf}: 'ﺿ'
    ArabicLetterDadInitialForm,
    /// \u{fec0}: 'ﻀ'
    ArabicLetterDadMedialForm,
    /// \u{fec1}: 'ﻁ'
    ArabicLetterTahIsolatedForm,
    /// \u{fec2}: 'ﻂ'
    ArabicLetterTahFinalForm,
    /// \u{fec3}: 'ﻃ'
    ArabicLetterTahInitialForm,
    /// \u{fec4}: 'ﻄ'
    ArabicLetterTahMedialForm,
    /// \u{fec5}: 'ﻅ'
    ArabicLetterZahIsolatedForm,
    /// \u{fec6}: 'ﻆ'
    ArabicLetterZahFinalForm,
    /// \u{fec7}: 'ﻇ'
    ArabicLetterZahInitialForm,
    /// \u{fec8}: 'ﻈ'
    ArabicLetterZahMedialForm,
    /// \u{fec9}: 'ﻉ'
    ArabicLetterAinIsolatedForm,
    /// \u{feca}: 'ﻊ'
    ArabicLetterAinFinalForm,
    /// \u{fecb}: 'ﻋ'
    ArabicLetterAinInitialForm,
    /// \u{fecc}: 'ﻌ'
    ArabicLetterAinMedialForm,
    /// \u{fecd}: 'ﻍ'
    ArabicLetterGhainIsolatedForm,
    /// \u{fece}: 'ﻎ'
    ArabicLetterGhainFinalForm,
    /// \u{fecf}: 'ﻏ'
    ArabicLetterGhainInitialForm,
    /// \u{fed0}: 'ﻐ'
    ArabicLetterGhainMedialForm,
    /// \u{fed1}: 'ﻑ'
    ArabicLetterFehIsolatedForm,
    /// \u{fed2}: 'ﻒ'
    ArabicLetterFehFinalForm,
    /// \u{fed3}: 'ﻓ'
    ArabicLetterFehInitialForm,
    /// \u{fed4}: 'ﻔ'
    ArabicLetterFehMedialForm,
    /// \u{fed5}: 'ﻕ'
    ArabicLetterQafIsolatedForm,
    /// \u{fed6}: 'ﻖ'
    ArabicLetterQafFinalForm,
    /// \u{fed7}: 'ﻗ'
    ArabicLetterQafInitialForm,
    /// \u{fed8}: 'ﻘ'
    ArabicLetterQafMedialForm,
    /// \u{fed9}: 'ﻙ'
    ArabicLetterKafIsolatedForm,
    /// \u{feda}: 'ﻚ'
    ArabicLetterKafFinalForm,
    /// \u{fedb}: 'ﻛ'
    ArabicLetterKafInitialForm,
    /// \u{fedc}: 'ﻜ'
    ArabicLetterKafMedialForm,
    /// \u{fedd}: 'ﻝ'
    ArabicLetterLamIsolatedForm,
    /// \u{fede}: 'ﻞ'
    ArabicLetterLamFinalForm,
    /// \u{fedf}: 'ﻟ'
    ArabicLetterLamInitialForm,
    /// \u{fee0}: 'ﻠ'
    ArabicLetterLamMedialForm,
    /// \u{fee1}: 'ﻡ'
    ArabicLetterMeemIsolatedForm,
    /// \u{fee2}: 'ﻢ'
    ArabicLetterMeemFinalForm,
    /// \u{fee3}: 'ﻣ'
    ArabicLetterMeemInitialForm,
    /// \u{fee4}: 'ﻤ'
    ArabicLetterMeemMedialForm,
    /// \u{fee5}: 'ﻥ'
    ArabicLetterNoonIsolatedForm,
    /// \u{fee6}: 'ﻦ'
    ArabicLetterNoonFinalForm,
    /// \u{fee7}: 'ﻧ'
    ArabicLetterNoonInitialForm,
    /// \u{fee8}: 'ﻨ'
    ArabicLetterNoonMedialForm,
    /// \u{fee9}: 'ﻩ'
    ArabicLetterHehIsolatedForm,
    /// \u{feea}: 'ﻪ'
    ArabicLetterHehFinalForm,
    /// \u{feeb}: 'ﻫ'
    ArabicLetterHehInitialForm,
    /// \u{feec}: 'ﻬ'
    ArabicLetterHehMedialForm,
    /// \u{feed}: 'ﻭ'
    ArabicLetterWawIsolatedForm,
    /// \u{feee}: 'ﻮ'
    ArabicLetterWawFinalForm,
    /// \u{feef}: 'ﻯ'
    ArabicLetterAlefMaksuraIsolatedForm,
    /// \u{fef0}: 'ﻰ'
    ArabicLetterAlefMaksuraFinalForm,
    /// \u{fef1}: 'ﻱ'
    ArabicLetterYehIsolatedForm,
    /// \u{fef2}: 'ﻲ'
    ArabicLetterYehFinalForm,
    /// \u{fef3}: 'ﻳ'
    ArabicLetterYehInitialForm,
    /// \u{fef4}: 'ﻴ'
    ArabicLetterYehMedialForm,
    /// \u{fef5}: 'ﻵ'
    ArabicLigatureLamWithAlefWithMaddaAboveIsolatedForm,
    /// \u{fef6}: 'ﻶ'
    ArabicLigatureLamWithAlefWithMaddaAboveFinalForm,
    /// \u{fef7}: 'ﻷ'
    ArabicLigatureLamWithAlefWithHamzaAboveIsolatedForm,
    /// \u{fef8}: 'ﻸ'
    ArabicLigatureLamWithAlefWithHamzaAboveFinalForm,
    /// \u{fef9}: 'ﻹ'
    ArabicLigatureLamWithAlefWithHamzaBelowIsolatedForm,
    /// \u{fefa}: 'ﻺ'
    ArabicLigatureLamWithAlefWithHamzaBelowFinalForm,
    /// \u{fefb}: 'ﻻ'
    ArabicLigatureLamWithAlefIsolatedForm,
    /// \u{fefc}: 'ﻼ'
    ArabicLigatureLamWithAlefFinalForm,
}

impl Into<char> for ArabicPresentationFormsB {
    fn into(self) -> char {
        match self {
            ArabicPresentationFormsB::ArabicFathatanIsolatedForm => 'ﹰ',
            ArabicPresentationFormsB::ArabicTatweelWithFathatanAbove => 'ﹱ',
            ArabicPresentationFormsB::ArabicDammatanIsolatedForm => 'ﹲ',
            ArabicPresentationFormsB::ArabicTailFragment => 'ﹳ',
            ArabicPresentationFormsB::ArabicKasratanIsolatedForm => 'ﹴ',
            ArabicPresentationFormsB::ArabicFathaIsolatedForm => 'ﹶ',
            ArabicPresentationFormsB::ArabicFathaMedialForm => 'ﹷ',
            ArabicPresentationFormsB::ArabicDammaIsolatedForm => 'ﹸ',
            ArabicPresentationFormsB::ArabicDammaMedialForm => 'ﹹ',
            ArabicPresentationFormsB::ArabicKasraIsolatedForm => 'ﹺ',
            ArabicPresentationFormsB::ArabicKasraMedialForm => 'ﹻ',
            ArabicPresentationFormsB::ArabicShaddaIsolatedForm => 'ﹼ',
            ArabicPresentationFormsB::ArabicShaddaMedialForm => 'ﹽ',
            ArabicPresentationFormsB::ArabicSukunIsolatedForm => 'ﹾ',
            ArabicPresentationFormsB::ArabicSukunMedialForm => 'ﹿ',
            ArabicPresentationFormsB::ArabicLetterHamzaIsolatedForm => 'ﺀ',
            ArabicPresentationFormsB::ArabicLetterAlefWithMaddaAboveIsolatedForm => 'ﺁ',
            ArabicPresentationFormsB::ArabicLetterAlefWithMaddaAboveFinalForm => 'ﺂ',
            ArabicPresentationFormsB::ArabicLetterAlefWithHamzaAboveIsolatedForm => 'ﺃ',
            ArabicPresentationFormsB::ArabicLetterAlefWithHamzaAboveFinalForm => 'ﺄ',
            ArabicPresentationFormsB::ArabicLetterWawWithHamzaAboveIsolatedForm => 'ﺅ',
            ArabicPresentationFormsB::ArabicLetterWawWithHamzaAboveFinalForm => 'ﺆ',
            ArabicPresentationFormsB::ArabicLetterAlefWithHamzaBelowIsolatedForm => 'ﺇ',
            ArabicPresentationFormsB::ArabicLetterAlefWithHamzaBelowFinalForm => 'ﺈ',
            ArabicPresentationFormsB::ArabicLetterYehWithHamzaAboveIsolatedForm => 'ﺉ',
            ArabicPresentationFormsB::ArabicLetterYehWithHamzaAboveFinalForm => 'ﺊ',
            ArabicPresentationFormsB::ArabicLetterYehWithHamzaAboveInitialForm => 'ﺋ',
            ArabicPresentationFormsB::ArabicLetterYehWithHamzaAboveMedialForm => 'ﺌ',
            ArabicPresentationFormsB::ArabicLetterAlefIsolatedForm => 'ﺍ',
            ArabicPresentationFormsB::ArabicLetterAlefFinalForm => 'ﺎ',
            ArabicPresentationFormsB::ArabicLetterBehIsolatedForm => 'ﺏ',
            ArabicPresentationFormsB::ArabicLetterBehFinalForm => 'ﺐ',
            ArabicPresentationFormsB::ArabicLetterBehInitialForm => 'ﺑ',
            ArabicPresentationFormsB::ArabicLetterBehMedialForm => 'ﺒ',
            ArabicPresentationFormsB::ArabicLetterTehMarbutaIsolatedForm => 'ﺓ',
            ArabicPresentationFormsB::ArabicLetterTehMarbutaFinalForm => 'ﺔ',
            ArabicPresentationFormsB::ArabicLetterTehIsolatedForm => 'ﺕ',
            ArabicPresentationFormsB::ArabicLetterTehFinalForm => 'ﺖ',
            ArabicPresentationFormsB::ArabicLetterTehInitialForm => 'ﺗ',
            ArabicPresentationFormsB::ArabicLetterTehMedialForm => 'ﺘ',
            ArabicPresentationFormsB::ArabicLetterThehIsolatedForm => 'ﺙ',
            ArabicPresentationFormsB::ArabicLetterThehFinalForm => 'ﺚ',
            ArabicPresentationFormsB::ArabicLetterThehInitialForm => 'ﺛ',
            ArabicPresentationFormsB::ArabicLetterThehMedialForm => 'ﺜ',
            ArabicPresentationFormsB::ArabicLetterJeemIsolatedForm => 'ﺝ',
            ArabicPresentationFormsB::ArabicLetterJeemFinalForm => 'ﺞ',
            ArabicPresentationFormsB::ArabicLetterJeemInitialForm => 'ﺟ',
            ArabicPresentationFormsB::ArabicLetterJeemMedialForm => 'ﺠ',
            ArabicPresentationFormsB::ArabicLetterHahIsolatedForm => 'ﺡ',
            ArabicPresentationFormsB::ArabicLetterHahFinalForm => 'ﺢ',
            ArabicPresentationFormsB::ArabicLetterHahInitialForm => 'ﺣ',
            ArabicPresentationFormsB::ArabicLetterHahMedialForm => 'ﺤ',
            ArabicPresentationFormsB::ArabicLetterKhahIsolatedForm => 'ﺥ',
            ArabicPresentationFormsB::ArabicLetterKhahFinalForm => 'ﺦ',
            ArabicPresentationFormsB::ArabicLetterKhahInitialForm => 'ﺧ',
            ArabicPresentationFormsB::ArabicLetterKhahMedialForm => 'ﺨ',
            ArabicPresentationFormsB::ArabicLetterDalIsolatedForm => 'ﺩ',
            ArabicPresentationFormsB::ArabicLetterDalFinalForm => 'ﺪ',
            ArabicPresentationFormsB::ArabicLetterThalIsolatedForm => 'ﺫ',
            ArabicPresentationFormsB::ArabicLetterThalFinalForm => 'ﺬ',
            ArabicPresentationFormsB::ArabicLetterRehIsolatedForm => 'ﺭ',
            ArabicPresentationFormsB::ArabicLetterRehFinalForm => 'ﺮ',
            ArabicPresentationFormsB::ArabicLetterZainIsolatedForm => 'ﺯ',
            ArabicPresentationFormsB::ArabicLetterZainFinalForm => 'ﺰ',
            ArabicPresentationFormsB::ArabicLetterSeenIsolatedForm => 'ﺱ',
            ArabicPresentationFormsB::ArabicLetterSeenFinalForm => 'ﺲ',
            ArabicPresentationFormsB::ArabicLetterSeenInitialForm => 'ﺳ',
            ArabicPresentationFormsB::ArabicLetterSeenMedialForm => 'ﺴ',
            ArabicPresentationFormsB::ArabicLetterSheenIsolatedForm => 'ﺵ',
            ArabicPresentationFormsB::ArabicLetterSheenFinalForm => 'ﺶ',
            ArabicPresentationFormsB::ArabicLetterSheenInitialForm => 'ﺷ',
            ArabicPresentationFormsB::ArabicLetterSheenMedialForm => 'ﺸ',
            ArabicPresentationFormsB::ArabicLetterSadIsolatedForm => 'ﺹ',
            ArabicPresentationFormsB::ArabicLetterSadFinalForm => 'ﺺ',
            ArabicPresentationFormsB::ArabicLetterSadInitialForm => 'ﺻ',
            ArabicPresentationFormsB::ArabicLetterSadMedialForm => 'ﺼ',
            ArabicPresentationFormsB::ArabicLetterDadIsolatedForm => 'ﺽ',
            ArabicPresentationFormsB::ArabicLetterDadFinalForm => 'ﺾ',
            ArabicPresentationFormsB::ArabicLetterDadInitialForm => 'ﺿ',
            ArabicPresentationFormsB::ArabicLetterDadMedialForm => 'ﻀ',
            ArabicPresentationFormsB::ArabicLetterTahIsolatedForm => 'ﻁ',
            ArabicPresentationFormsB::ArabicLetterTahFinalForm => 'ﻂ',
            ArabicPresentationFormsB::ArabicLetterTahInitialForm => 'ﻃ',
            ArabicPresentationFormsB::ArabicLetterTahMedialForm => 'ﻄ',
            ArabicPresentationFormsB::ArabicLetterZahIsolatedForm => 'ﻅ',
            ArabicPresentationFormsB::ArabicLetterZahFinalForm => 'ﻆ',
            ArabicPresentationFormsB::ArabicLetterZahInitialForm => 'ﻇ',
            ArabicPresentationFormsB::ArabicLetterZahMedialForm => 'ﻈ',
            ArabicPresentationFormsB::ArabicLetterAinIsolatedForm => 'ﻉ',
            ArabicPresentationFormsB::ArabicLetterAinFinalForm => 'ﻊ',
            ArabicPresentationFormsB::ArabicLetterAinInitialForm => 'ﻋ',
            ArabicPresentationFormsB::ArabicLetterAinMedialForm => 'ﻌ',
            ArabicPresentationFormsB::ArabicLetterGhainIsolatedForm => 'ﻍ',
            ArabicPresentationFormsB::ArabicLetterGhainFinalForm => 'ﻎ',
            ArabicPresentationFormsB::ArabicLetterGhainInitialForm => 'ﻏ',
            ArabicPresentationFormsB::ArabicLetterGhainMedialForm => 'ﻐ',
            ArabicPresentationFormsB::ArabicLetterFehIsolatedForm => 'ﻑ',
            ArabicPresentationFormsB::ArabicLetterFehFinalForm => 'ﻒ',
            ArabicPresentationFormsB::ArabicLetterFehInitialForm => 'ﻓ',
            ArabicPresentationFormsB::ArabicLetterFehMedialForm => 'ﻔ',
            ArabicPresentationFormsB::ArabicLetterQafIsolatedForm => 'ﻕ',
            ArabicPresentationFormsB::ArabicLetterQafFinalForm => 'ﻖ',
            ArabicPresentationFormsB::ArabicLetterQafInitialForm => 'ﻗ',
            ArabicPresentationFormsB::ArabicLetterQafMedialForm => 'ﻘ',
            ArabicPresentationFormsB::ArabicLetterKafIsolatedForm => 'ﻙ',
            ArabicPresentationFormsB::ArabicLetterKafFinalForm => 'ﻚ',
            ArabicPresentationFormsB::ArabicLetterKafInitialForm => 'ﻛ',
            ArabicPresentationFormsB::ArabicLetterKafMedialForm => 'ﻜ',
            ArabicPresentationFormsB::ArabicLetterLamIsolatedForm => 'ﻝ',
            ArabicPresentationFormsB::ArabicLetterLamFinalForm => 'ﻞ',
            ArabicPresentationFormsB::ArabicLetterLamInitialForm => 'ﻟ',
            ArabicPresentationFormsB::ArabicLetterLamMedialForm => 'ﻠ',
            ArabicPresentationFormsB::ArabicLetterMeemIsolatedForm => 'ﻡ',
            ArabicPresentationFormsB::ArabicLetterMeemFinalForm => 'ﻢ',
            ArabicPresentationFormsB::ArabicLetterMeemInitialForm => 'ﻣ',
            ArabicPresentationFormsB::ArabicLetterMeemMedialForm => 'ﻤ',
            ArabicPresentationFormsB::ArabicLetterNoonIsolatedForm => 'ﻥ',
            ArabicPresentationFormsB::ArabicLetterNoonFinalForm => 'ﻦ',
            ArabicPresentationFormsB::ArabicLetterNoonInitialForm => 'ﻧ',
            ArabicPresentationFormsB::ArabicLetterNoonMedialForm => 'ﻨ',
            ArabicPresentationFormsB::ArabicLetterHehIsolatedForm => 'ﻩ',
            ArabicPresentationFormsB::ArabicLetterHehFinalForm => 'ﻪ',
            ArabicPresentationFormsB::ArabicLetterHehInitialForm => 'ﻫ',
            ArabicPresentationFormsB::ArabicLetterHehMedialForm => 'ﻬ',
            ArabicPresentationFormsB::ArabicLetterWawIsolatedForm => 'ﻭ',
            ArabicPresentationFormsB::ArabicLetterWawFinalForm => 'ﻮ',
            ArabicPresentationFormsB::ArabicLetterAlefMaksuraIsolatedForm => 'ﻯ',
            ArabicPresentationFormsB::ArabicLetterAlefMaksuraFinalForm => 'ﻰ',
            ArabicPresentationFormsB::ArabicLetterYehIsolatedForm => 'ﻱ',
            ArabicPresentationFormsB::ArabicLetterYehFinalForm => 'ﻲ',
            ArabicPresentationFormsB::ArabicLetterYehInitialForm => 'ﻳ',
            ArabicPresentationFormsB::ArabicLetterYehMedialForm => 'ﻴ',
            ArabicPresentationFormsB::ArabicLigatureLamWithAlefWithMaddaAboveIsolatedForm => 'ﻵ',
            ArabicPresentationFormsB::ArabicLigatureLamWithAlefWithMaddaAboveFinalForm => 'ﻶ',
            ArabicPresentationFormsB::ArabicLigatureLamWithAlefWithHamzaAboveIsolatedForm => 'ﻷ',
            ArabicPresentationFormsB::ArabicLigatureLamWithAlefWithHamzaAboveFinalForm => 'ﻸ',
            ArabicPresentationFormsB::ArabicLigatureLamWithAlefWithHamzaBelowIsolatedForm => 'ﻹ',
            ArabicPresentationFormsB::ArabicLigatureLamWithAlefWithHamzaBelowFinalForm => 'ﻺ',
            ArabicPresentationFormsB::ArabicLigatureLamWithAlefIsolatedForm => 'ﻻ',
            ArabicPresentationFormsB::ArabicLigatureLamWithAlefFinalForm => 'ﻼ',
        }
    }
}

impl std::convert::TryFrom<char> for ArabicPresentationFormsB {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ﹰ' => Ok(ArabicPresentationFormsB::ArabicFathatanIsolatedForm),
            'ﹱ' => Ok(ArabicPresentationFormsB::ArabicTatweelWithFathatanAbove),
            'ﹲ' => Ok(ArabicPresentationFormsB::ArabicDammatanIsolatedForm),
            'ﹳ' => Ok(ArabicPresentationFormsB::ArabicTailFragment),
            'ﹴ' => Ok(ArabicPresentationFormsB::ArabicKasratanIsolatedForm),
            'ﹶ' => Ok(ArabicPresentationFormsB::ArabicFathaIsolatedForm),
            'ﹷ' => Ok(ArabicPresentationFormsB::ArabicFathaMedialForm),
            'ﹸ' => Ok(ArabicPresentationFormsB::ArabicDammaIsolatedForm),
            'ﹹ' => Ok(ArabicPresentationFormsB::ArabicDammaMedialForm),
            'ﹺ' => Ok(ArabicPresentationFormsB::ArabicKasraIsolatedForm),
            'ﹻ' => Ok(ArabicPresentationFormsB::ArabicKasraMedialForm),
            'ﹼ' => Ok(ArabicPresentationFormsB::ArabicShaddaIsolatedForm),
            'ﹽ' => Ok(ArabicPresentationFormsB::ArabicShaddaMedialForm),
            'ﹾ' => Ok(ArabicPresentationFormsB::ArabicSukunIsolatedForm),
            'ﹿ' => Ok(ArabicPresentationFormsB::ArabicSukunMedialForm),
            'ﺀ' => Ok(ArabicPresentationFormsB::ArabicLetterHamzaIsolatedForm),
            'ﺁ' => Ok(ArabicPresentationFormsB::ArabicLetterAlefWithMaddaAboveIsolatedForm),
            'ﺂ' => Ok(ArabicPresentationFormsB::ArabicLetterAlefWithMaddaAboveFinalForm),
            'ﺃ' => Ok(ArabicPresentationFormsB::ArabicLetterAlefWithHamzaAboveIsolatedForm),
            'ﺄ' => Ok(ArabicPresentationFormsB::ArabicLetterAlefWithHamzaAboveFinalForm),
            'ﺅ' => Ok(ArabicPresentationFormsB::ArabicLetterWawWithHamzaAboveIsolatedForm),
            'ﺆ' => Ok(ArabicPresentationFormsB::ArabicLetterWawWithHamzaAboveFinalForm),
            'ﺇ' => Ok(ArabicPresentationFormsB::ArabicLetterAlefWithHamzaBelowIsolatedForm),
            'ﺈ' => Ok(ArabicPresentationFormsB::ArabicLetterAlefWithHamzaBelowFinalForm),
            'ﺉ' => Ok(ArabicPresentationFormsB::ArabicLetterYehWithHamzaAboveIsolatedForm),
            'ﺊ' => Ok(ArabicPresentationFormsB::ArabicLetterYehWithHamzaAboveFinalForm),
            'ﺋ' => Ok(ArabicPresentationFormsB::ArabicLetterYehWithHamzaAboveInitialForm),
            'ﺌ' => Ok(ArabicPresentationFormsB::ArabicLetterYehWithHamzaAboveMedialForm),
            'ﺍ' => Ok(ArabicPresentationFormsB::ArabicLetterAlefIsolatedForm),
            'ﺎ' => Ok(ArabicPresentationFormsB::ArabicLetterAlefFinalForm),
            'ﺏ' => Ok(ArabicPresentationFormsB::ArabicLetterBehIsolatedForm),
            'ﺐ' => Ok(ArabicPresentationFormsB::ArabicLetterBehFinalForm),
            'ﺑ' => Ok(ArabicPresentationFormsB::ArabicLetterBehInitialForm),
            'ﺒ' => Ok(ArabicPresentationFormsB::ArabicLetterBehMedialForm),
            'ﺓ' => Ok(ArabicPresentationFormsB::ArabicLetterTehMarbutaIsolatedForm),
            'ﺔ' => Ok(ArabicPresentationFormsB::ArabicLetterTehMarbutaFinalForm),
            'ﺕ' => Ok(ArabicPresentationFormsB::ArabicLetterTehIsolatedForm),
            'ﺖ' => Ok(ArabicPresentationFormsB::ArabicLetterTehFinalForm),
            'ﺗ' => Ok(ArabicPresentationFormsB::ArabicLetterTehInitialForm),
            'ﺘ' => Ok(ArabicPresentationFormsB::ArabicLetterTehMedialForm),
            'ﺙ' => Ok(ArabicPresentationFormsB::ArabicLetterThehIsolatedForm),
            'ﺚ' => Ok(ArabicPresentationFormsB::ArabicLetterThehFinalForm),
            'ﺛ' => Ok(ArabicPresentationFormsB::ArabicLetterThehInitialForm),
            'ﺜ' => Ok(ArabicPresentationFormsB::ArabicLetterThehMedialForm),
            'ﺝ' => Ok(ArabicPresentationFormsB::ArabicLetterJeemIsolatedForm),
            'ﺞ' => Ok(ArabicPresentationFormsB::ArabicLetterJeemFinalForm),
            'ﺟ' => Ok(ArabicPresentationFormsB::ArabicLetterJeemInitialForm),
            'ﺠ' => Ok(ArabicPresentationFormsB::ArabicLetterJeemMedialForm),
            'ﺡ' => Ok(ArabicPresentationFormsB::ArabicLetterHahIsolatedForm),
            'ﺢ' => Ok(ArabicPresentationFormsB::ArabicLetterHahFinalForm),
            'ﺣ' => Ok(ArabicPresentationFormsB::ArabicLetterHahInitialForm),
            'ﺤ' => Ok(ArabicPresentationFormsB::ArabicLetterHahMedialForm),
            'ﺥ' => Ok(ArabicPresentationFormsB::ArabicLetterKhahIsolatedForm),
            'ﺦ' => Ok(ArabicPresentationFormsB::ArabicLetterKhahFinalForm),
            'ﺧ' => Ok(ArabicPresentationFormsB::ArabicLetterKhahInitialForm),
            'ﺨ' => Ok(ArabicPresentationFormsB::ArabicLetterKhahMedialForm),
            'ﺩ' => Ok(ArabicPresentationFormsB::ArabicLetterDalIsolatedForm),
            'ﺪ' => Ok(ArabicPresentationFormsB::ArabicLetterDalFinalForm),
            'ﺫ' => Ok(ArabicPresentationFormsB::ArabicLetterThalIsolatedForm),
            'ﺬ' => Ok(ArabicPresentationFormsB::ArabicLetterThalFinalForm),
            'ﺭ' => Ok(ArabicPresentationFormsB::ArabicLetterRehIsolatedForm),
            'ﺮ' => Ok(ArabicPresentationFormsB::ArabicLetterRehFinalForm),
            'ﺯ' => Ok(ArabicPresentationFormsB::ArabicLetterZainIsolatedForm),
            'ﺰ' => Ok(ArabicPresentationFormsB::ArabicLetterZainFinalForm),
            'ﺱ' => Ok(ArabicPresentationFormsB::ArabicLetterSeenIsolatedForm),
            'ﺲ' => Ok(ArabicPresentationFormsB::ArabicLetterSeenFinalForm),
            'ﺳ' => Ok(ArabicPresentationFormsB::ArabicLetterSeenInitialForm),
            'ﺴ' => Ok(ArabicPresentationFormsB::ArabicLetterSeenMedialForm),
            'ﺵ' => Ok(ArabicPresentationFormsB::ArabicLetterSheenIsolatedForm),
            'ﺶ' => Ok(ArabicPresentationFormsB::ArabicLetterSheenFinalForm),
            'ﺷ' => Ok(ArabicPresentationFormsB::ArabicLetterSheenInitialForm),
            'ﺸ' => Ok(ArabicPresentationFormsB::ArabicLetterSheenMedialForm),
            'ﺹ' => Ok(ArabicPresentationFormsB::ArabicLetterSadIsolatedForm),
            'ﺺ' => Ok(ArabicPresentationFormsB::ArabicLetterSadFinalForm),
            'ﺻ' => Ok(ArabicPresentationFormsB::ArabicLetterSadInitialForm),
            'ﺼ' => Ok(ArabicPresentationFormsB::ArabicLetterSadMedialForm),
            'ﺽ' => Ok(ArabicPresentationFormsB::ArabicLetterDadIsolatedForm),
            'ﺾ' => Ok(ArabicPresentationFormsB::ArabicLetterDadFinalForm),
            'ﺿ' => Ok(ArabicPresentationFormsB::ArabicLetterDadInitialForm),
            'ﻀ' => Ok(ArabicPresentationFormsB::ArabicLetterDadMedialForm),
            'ﻁ' => Ok(ArabicPresentationFormsB::ArabicLetterTahIsolatedForm),
            'ﻂ' => Ok(ArabicPresentationFormsB::ArabicLetterTahFinalForm),
            'ﻃ' => Ok(ArabicPresentationFormsB::ArabicLetterTahInitialForm),
            'ﻄ' => Ok(ArabicPresentationFormsB::ArabicLetterTahMedialForm),
            'ﻅ' => Ok(ArabicPresentationFormsB::ArabicLetterZahIsolatedForm),
            'ﻆ' => Ok(ArabicPresentationFormsB::ArabicLetterZahFinalForm),
            'ﻇ' => Ok(ArabicPresentationFormsB::ArabicLetterZahInitialForm),
            'ﻈ' => Ok(ArabicPresentationFormsB::ArabicLetterZahMedialForm),
            'ﻉ' => Ok(ArabicPresentationFormsB::ArabicLetterAinIsolatedForm),
            'ﻊ' => Ok(ArabicPresentationFormsB::ArabicLetterAinFinalForm),
            'ﻋ' => Ok(ArabicPresentationFormsB::ArabicLetterAinInitialForm),
            'ﻌ' => Ok(ArabicPresentationFormsB::ArabicLetterAinMedialForm),
            'ﻍ' => Ok(ArabicPresentationFormsB::ArabicLetterGhainIsolatedForm),
            'ﻎ' => Ok(ArabicPresentationFormsB::ArabicLetterGhainFinalForm),
            'ﻏ' => Ok(ArabicPresentationFormsB::ArabicLetterGhainInitialForm),
            'ﻐ' => Ok(ArabicPresentationFormsB::ArabicLetterGhainMedialForm),
            'ﻑ' => Ok(ArabicPresentationFormsB::ArabicLetterFehIsolatedForm),
            'ﻒ' => Ok(ArabicPresentationFormsB::ArabicLetterFehFinalForm),
            'ﻓ' => Ok(ArabicPresentationFormsB::ArabicLetterFehInitialForm),
            'ﻔ' => Ok(ArabicPresentationFormsB::ArabicLetterFehMedialForm),
            'ﻕ' => Ok(ArabicPresentationFormsB::ArabicLetterQafIsolatedForm),
            'ﻖ' => Ok(ArabicPresentationFormsB::ArabicLetterQafFinalForm),
            'ﻗ' => Ok(ArabicPresentationFormsB::ArabicLetterQafInitialForm),
            'ﻘ' => Ok(ArabicPresentationFormsB::ArabicLetterQafMedialForm),
            'ﻙ' => Ok(ArabicPresentationFormsB::ArabicLetterKafIsolatedForm),
            'ﻚ' => Ok(ArabicPresentationFormsB::ArabicLetterKafFinalForm),
            'ﻛ' => Ok(ArabicPresentationFormsB::ArabicLetterKafInitialForm),
            'ﻜ' => Ok(ArabicPresentationFormsB::ArabicLetterKafMedialForm),
            'ﻝ' => Ok(ArabicPresentationFormsB::ArabicLetterLamIsolatedForm),
            'ﻞ' => Ok(ArabicPresentationFormsB::ArabicLetterLamFinalForm),
            'ﻟ' => Ok(ArabicPresentationFormsB::ArabicLetterLamInitialForm),
            'ﻠ' => Ok(ArabicPresentationFormsB::ArabicLetterLamMedialForm),
            'ﻡ' => Ok(ArabicPresentationFormsB::ArabicLetterMeemIsolatedForm),
            'ﻢ' => Ok(ArabicPresentationFormsB::ArabicLetterMeemFinalForm),
            'ﻣ' => Ok(ArabicPresentationFormsB::ArabicLetterMeemInitialForm),
            'ﻤ' => Ok(ArabicPresentationFormsB::ArabicLetterMeemMedialForm),
            'ﻥ' => Ok(ArabicPresentationFormsB::ArabicLetterNoonIsolatedForm),
            'ﻦ' => Ok(ArabicPresentationFormsB::ArabicLetterNoonFinalForm),
            'ﻧ' => Ok(ArabicPresentationFormsB::ArabicLetterNoonInitialForm),
            'ﻨ' => Ok(ArabicPresentationFormsB::ArabicLetterNoonMedialForm),
            'ﻩ' => Ok(ArabicPresentationFormsB::ArabicLetterHehIsolatedForm),
            'ﻪ' => Ok(ArabicPresentationFormsB::ArabicLetterHehFinalForm),
            'ﻫ' => Ok(ArabicPresentationFormsB::ArabicLetterHehInitialForm),
            'ﻬ' => Ok(ArabicPresentationFormsB::ArabicLetterHehMedialForm),
            'ﻭ' => Ok(ArabicPresentationFormsB::ArabicLetterWawIsolatedForm),
            'ﻮ' => Ok(ArabicPresentationFormsB::ArabicLetterWawFinalForm),
            'ﻯ' => Ok(ArabicPresentationFormsB::ArabicLetterAlefMaksuraIsolatedForm),
            'ﻰ' => Ok(ArabicPresentationFormsB::ArabicLetterAlefMaksuraFinalForm),
            'ﻱ' => Ok(ArabicPresentationFormsB::ArabicLetterYehIsolatedForm),
            'ﻲ' => Ok(ArabicPresentationFormsB::ArabicLetterYehFinalForm),
            'ﻳ' => Ok(ArabicPresentationFormsB::ArabicLetterYehInitialForm),
            'ﻴ' => Ok(ArabicPresentationFormsB::ArabicLetterYehMedialForm),
            'ﻵ' => Ok(ArabicPresentationFormsB::ArabicLigatureLamWithAlefWithMaddaAboveIsolatedForm),
            'ﻶ' => Ok(ArabicPresentationFormsB::ArabicLigatureLamWithAlefWithMaddaAboveFinalForm),
            'ﻷ' => Ok(ArabicPresentationFormsB::ArabicLigatureLamWithAlefWithHamzaAboveIsolatedForm),
            'ﻸ' => Ok(ArabicPresentationFormsB::ArabicLigatureLamWithAlefWithHamzaAboveFinalForm),
            'ﻹ' => Ok(ArabicPresentationFormsB::ArabicLigatureLamWithAlefWithHamzaBelowIsolatedForm),
            'ﻺ' => Ok(ArabicPresentationFormsB::ArabicLigatureLamWithAlefWithHamzaBelowFinalForm),
            'ﻻ' => Ok(ArabicPresentationFormsB::ArabicLigatureLamWithAlefIsolatedForm),
            'ﻼ' => Ok(ArabicPresentationFormsB::ArabicLigatureLamWithAlefFinalForm),
            _ => Err(()),
        }
    }
}

impl Into<u32> for ArabicPresentationFormsB {
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

impl std::convert::TryFrom<u32> for ArabicPresentationFormsB {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for ArabicPresentationFormsB {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl ArabicPresentationFormsB {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        ArabicPresentationFormsB::ArabicFathatanIsolatedForm
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("ArabicPresentationFormsB{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
