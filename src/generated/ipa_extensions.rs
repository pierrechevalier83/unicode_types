
/// An enum to represent all characters in the IPAExtensions block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum IPAExtensions {
    /// \u{250}: 'ɐ'
    LatinSmallLetterTurnedA,
    /// \u{251}: 'ɑ'
    LatinSmallLetterAlpha,
    /// \u{252}: 'ɒ'
    LatinSmallLetterTurnedAlpha,
    /// \u{253}: 'ɓ'
    LatinSmallLetterBWithHook,
    /// \u{254}: 'ɔ'
    LatinSmallLetterOpenO,
    /// \u{255}: 'ɕ'
    LatinSmallLetterCWithCurl,
    /// \u{256}: 'ɖ'
    LatinSmallLetterDWithTail,
    /// \u{257}: 'ɗ'
    LatinSmallLetterDWithHook,
    /// \u{258}: 'ɘ'
    LatinSmallLetterReversedE,
    /// \u{259}: 'ə'
    LatinSmallLetterSchwa,
    /// \u{25a}: 'ɚ'
    LatinSmallLetterSchwaWithHook,
    /// \u{25b}: 'ɛ'
    LatinSmallLetterOpenE,
    /// \u{25c}: 'ɜ'
    LatinSmallLetterReversedOpenE,
    /// \u{25d}: 'ɝ'
    LatinSmallLetterReversedOpenEWithHook,
    /// \u{25e}: 'ɞ'
    LatinSmallLetterClosedReversedOpenE,
    /// \u{25f}: 'ɟ'
    LatinSmallLetterDotlessJWithStroke,
    /// \u{260}: 'ɠ'
    LatinSmallLetterGWithHook,
    /// \u{261}: 'ɡ'
    LatinSmallLetterScriptG,
    /// \u{262}: 'ɢ'
    LatinLetterSmallCapitalG,
    /// \u{263}: 'ɣ'
    LatinSmallLetterGamma,
    /// \u{264}: 'ɤ'
    LatinSmallLetterRamsHorn,
    /// \u{265}: 'ɥ'
    LatinSmallLetterTurnedH,
    /// \u{266}: 'ɦ'
    LatinSmallLetterHWithHook,
    /// \u{267}: 'ɧ'
    LatinSmallLetterHengWithHook,
    /// \u{268}: 'ɨ'
    LatinSmallLetterIWithStroke,
    /// \u{269}: 'ɩ'
    LatinSmallLetterIota,
    /// \u{26a}: 'ɪ'
    LatinLetterSmallCapitalI,
    /// \u{26b}: 'ɫ'
    LatinSmallLetterLWithMiddleTilde,
    /// \u{26c}: 'ɬ'
    LatinSmallLetterLWithBelt,
    /// \u{26d}: 'ɭ'
    LatinSmallLetterLWithRetroflexHook,
    /// \u{26e}: 'ɮ'
    LatinSmallLetterLezh,
    /// \u{26f}: 'ɯ'
    LatinSmallLetterTurnedM,
    /// \u{270}: 'ɰ'
    LatinSmallLetterTurnedMWithLongLeg,
    /// \u{271}: 'ɱ'
    LatinSmallLetterMWithHook,
    /// \u{272}: 'ɲ'
    LatinSmallLetterNWithLeftHook,
    /// \u{273}: 'ɳ'
    LatinSmallLetterNWithRetroflexHook,
    /// \u{274}: 'ɴ'
    LatinLetterSmallCapitalN,
    /// \u{275}: 'ɵ'
    LatinSmallLetterBarredO,
    /// \u{276}: 'ɶ'
    LatinLetterSmallCapitalOe,
    /// \u{277}: 'ɷ'
    LatinSmallLetterClosedOmega,
    /// \u{278}: 'ɸ'
    LatinSmallLetterPhi,
    /// \u{279}: 'ɹ'
    LatinSmallLetterTurnedR,
    /// \u{27a}: 'ɺ'
    LatinSmallLetterTurnedRWithLongLeg,
    /// \u{27b}: 'ɻ'
    LatinSmallLetterTurnedRWithHook,
    /// \u{27c}: 'ɼ'
    LatinSmallLetterRWithLongLeg,
    /// \u{27d}: 'ɽ'
    LatinSmallLetterRWithTail,
    /// \u{27e}: 'ɾ'
    LatinSmallLetterRWithFishhook,
    /// \u{27f}: 'ɿ'
    LatinSmallLetterReversedRWithFishhook,
    /// \u{280}: 'ʀ'
    LatinLetterSmallCapitalR,
    /// \u{281}: 'ʁ'
    LatinLetterSmallCapitalInvertedR,
    /// \u{282}: 'ʂ'
    LatinSmallLetterSWithHook,
    /// \u{283}: 'ʃ'
    LatinSmallLetterEsh,
    /// \u{284}: 'ʄ'
    LatinSmallLetterDotlessJWithStrokeAndHook,
    /// \u{285}: 'ʅ'
    LatinSmallLetterSquatReversedEsh,
    /// \u{286}: 'ʆ'
    LatinSmallLetterEshWithCurl,
    /// \u{287}: 'ʇ'
    LatinSmallLetterTurnedT,
    /// \u{288}: 'ʈ'
    LatinSmallLetterTWithRetroflexHook,
    /// \u{289}: 'ʉ'
    LatinSmallLetterUBar,
    /// \u{28a}: 'ʊ'
    LatinSmallLetterUpsilon,
    /// \u{28b}: 'ʋ'
    LatinSmallLetterVWithHook,
    /// \u{28c}: 'ʌ'
    LatinSmallLetterTurnedV,
    /// \u{28d}: 'ʍ'
    LatinSmallLetterTurnedW,
    /// \u{28e}: 'ʎ'
    LatinSmallLetterTurnedY,
    /// \u{28f}: 'ʏ'
    LatinLetterSmallCapitalY,
    /// \u{290}: 'ʐ'
    LatinSmallLetterZWithRetroflexHook,
    /// \u{291}: 'ʑ'
    LatinSmallLetterZWithCurl,
    /// \u{292}: 'ʒ'
    LatinSmallLetterEzh,
    /// \u{293}: 'ʓ'
    LatinSmallLetterEzhWithCurl,
    /// \u{294}: 'ʔ'
    LatinLetterGlottalStop,
    /// \u{295}: 'ʕ'
    LatinLetterPharyngealVoicedFricative,
    /// \u{296}: 'ʖ'
    LatinLetterInvertedGlottalStop,
    /// \u{297}: 'ʗ'
    LatinLetterStretchedC,
    /// \u{298}: 'ʘ'
    LatinLetterBilabialClick,
    /// \u{299}: 'ʙ'
    LatinLetterSmallCapitalB,
    /// \u{29a}: 'ʚ'
    LatinSmallLetterClosedOpenE,
    /// \u{29b}: 'ʛ'
    LatinLetterSmallCapitalGWithHook,
    /// \u{29c}: 'ʜ'
    LatinLetterSmallCapitalH,
    /// \u{29d}: 'ʝ'
    LatinSmallLetterJWithCrossedDashTail,
    /// \u{29e}: 'ʞ'
    LatinSmallLetterTurnedK,
    /// \u{29f}: 'ʟ'
    LatinLetterSmallCapitalL,
    /// \u{2a0}: 'ʠ'
    LatinSmallLetterQWithHook,
    /// \u{2a1}: 'ʡ'
    LatinLetterGlottalStopWithStroke,
    /// \u{2a2}: 'ʢ'
    LatinLetterReversedGlottalStopWithStroke,
    /// \u{2a3}: 'ʣ'
    LatinSmallLetterDzDigraph,
    /// \u{2a4}: 'ʤ'
    LatinSmallLetterDezhDigraph,
    /// \u{2a5}: 'ʥ'
    LatinSmallLetterDzDigraphWithCurl,
    /// \u{2a6}: 'ʦ'
    LatinSmallLetterTsDigraph,
    /// \u{2a7}: 'ʧ'
    LatinSmallLetterTeshDigraph,
    /// \u{2a8}: 'ʨ'
    LatinSmallLetterTcDigraphWithCurl,
    /// \u{2a9}: 'ʩ'
    LatinSmallLetterFengDigraph,
    /// \u{2aa}: 'ʪ'
    LatinSmallLetterLsDigraph,
    /// \u{2ab}: 'ʫ'
    LatinSmallLetterLzDigraph,
    /// \u{2ac}: 'ʬ'
    LatinLetterBilabialPercussive,
    /// \u{2ad}: 'ʭ'
    LatinLetterBidentalPercussive,
    /// \u{2ae}: 'ʮ'
    LatinSmallLetterTurnedHWithFishhook,
}

impl Into<char> for IPAExtensions {
    fn into(self) -> char {
        match self {
            IPAExtensions::LatinSmallLetterTurnedA => 'ɐ',
            IPAExtensions::LatinSmallLetterAlpha => 'ɑ',
            IPAExtensions::LatinSmallLetterTurnedAlpha => 'ɒ',
            IPAExtensions::LatinSmallLetterBWithHook => 'ɓ',
            IPAExtensions::LatinSmallLetterOpenO => 'ɔ',
            IPAExtensions::LatinSmallLetterCWithCurl => 'ɕ',
            IPAExtensions::LatinSmallLetterDWithTail => 'ɖ',
            IPAExtensions::LatinSmallLetterDWithHook => 'ɗ',
            IPAExtensions::LatinSmallLetterReversedE => 'ɘ',
            IPAExtensions::LatinSmallLetterSchwa => 'ə',
            IPAExtensions::LatinSmallLetterSchwaWithHook => 'ɚ',
            IPAExtensions::LatinSmallLetterOpenE => 'ɛ',
            IPAExtensions::LatinSmallLetterReversedOpenE => 'ɜ',
            IPAExtensions::LatinSmallLetterReversedOpenEWithHook => 'ɝ',
            IPAExtensions::LatinSmallLetterClosedReversedOpenE => 'ɞ',
            IPAExtensions::LatinSmallLetterDotlessJWithStroke => 'ɟ',
            IPAExtensions::LatinSmallLetterGWithHook => 'ɠ',
            IPAExtensions::LatinSmallLetterScriptG => 'ɡ',
            IPAExtensions::LatinLetterSmallCapitalG => 'ɢ',
            IPAExtensions::LatinSmallLetterGamma => 'ɣ',
            IPAExtensions::LatinSmallLetterRamsHorn => 'ɤ',
            IPAExtensions::LatinSmallLetterTurnedH => 'ɥ',
            IPAExtensions::LatinSmallLetterHWithHook => 'ɦ',
            IPAExtensions::LatinSmallLetterHengWithHook => 'ɧ',
            IPAExtensions::LatinSmallLetterIWithStroke => 'ɨ',
            IPAExtensions::LatinSmallLetterIota => 'ɩ',
            IPAExtensions::LatinLetterSmallCapitalI => 'ɪ',
            IPAExtensions::LatinSmallLetterLWithMiddleTilde => 'ɫ',
            IPAExtensions::LatinSmallLetterLWithBelt => 'ɬ',
            IPAExtensions::LatinSmallLetterLWithRetroflexHook => 'ɭ',
            IPAExtensions::LatinSmallLetterLezh => 'ɮ',
            IPAExtensions::LatinSmallLetterTurnedM => 'ɯ',
            IPAExtensions::LatinSmallLetterTurnedMWithLongLeg => 'ɰ',
            IPAExtensions::LatinSmallLetterMWithHook => 'ɱ',
            IPAExtensions::LatinSmallLetterNWithLeftHook => 'ɲ',
            IPAExtensions::LatinSmallLetterNWithRetroflexHook => 'ɳ',
            IPAExtensions::LatinLetterSmallCapitalN => 'ɴ',
            IPAExtensions::LatinSmallLetterBarredO => 'ɵ',
            IPAExtensions::LatinLetterSmallCapitalOe => 'ɶ',
            IPAExtensions::LatinSmallLetterClosedOmega => 'ɷ',
            IPAExtensions::LatinSmallLetterPhi => 'ɸ',
            IPAExtensions::LatinSmallLetterTurnedR => 'ɹ',
            IPAExtensions::LatinSmallLetterTurnedRWithLongLeg => 'ɺ',
            IPAExtensions::LatinSmallLetterTurnedRWithHook => 'ɻ',
            IPAExtensions::LatinSmallLetterRWithLongLeg => 'ɼ',
            IPAExtensions::LatinSmallLetterRWithTail => 'ɽ',
            IPAExtensions::LatinSmallLetterRWithFishhook => 'ɾ',
            IPAExtensions::LatinSmallLetterReversedRWithFishhook => 'ɿ',
            IPAExtensions::LatinLetterSmallCapitalR => 'ʀ',
            IPAExtensions::LatinLetterSmallCapitalInvertedR => 'ʁ',
            IPAExtensions::LatinSmallLetterSWithHook => 'ʂ',
            IPAExtensions::LatinSmallLetterEsh => 'ʃ',
            IPAExtensions::LatinSmallLetterDotlessJWithStrokeAndHook => 'ʄ',
            IPAExtensions::LatinSmallLetterSquatReversedEsh => 'ʅ',
            IPAExtensions::LatinSmallLetterEshWithCurl => 'ʆ',
            IPAExtensions::LatinSmallLetterTurnedT => 'ʇ',
            IPAExtensions::LatinSmallLetterTWithRetroflexHook => 'ʈ',
            IPAExtensions::LatinSmallLetterUBar => 'ʉ',
            IPAExtensions::LatinSmallLetterUpsilon => 'ʊ',
            IPAExtensions::LatinSmallLetterVWithHook => 'ʋ',
            IPAExtensions::LatinSmallLetterTurnedV => 'ʌ',
            IPAExtensions::LatinSmallLetterTurnedW => 'ʍ',
            IPAExtensions::LatinSmallLetterTurnedY => 'ʎ',
            IPAExtensions::LatinLetterSmallCapitalY => 'ʏ',
            IPAExtensions::LatinSmallLetterZWithRetroflexHook => 'ʐ',
            IPAExtensions::LatinSmallLetterZWithCurl => 'ʑ',
            IPAExtensions::LatinSmallLetterEzh => 'ʒ',
            IPAExtensions::LatinSmallLetterEzhWithCurl => 'ʓ',
            IPAExtensions::LatinLetterGlottalStop => 'ʔ',
            IPAExtensions::LatinLetterPharyngealVoicedFricative => 'ʕ',
            IPAExtensions::LatinLetterInvertedGlottalStop => 'ʖ',
            IPAExtensions::LatinLetterStretchedC => 'ʗ',
            IPAExtensions::LatinLetterBilabialClick => 'ʘ',
            IPAExtensions::LatinLetterSmallCapitalB => 'ʙ',
            IPAExtensions::LatinSmallLetterClosedOpenE => 'ʚ',
            IPAExtensions::LatinLetterSmallCapitalGWithHook => 'ʛ',
            IPAExtensions::LatinLetterSmallCapitalH => 'ʜ',
            IPAExtensions::LatinSmallLetterJWithCrossedDashTail => 'ʝ',
            IPAExtensions::LatinSmallLetterTurnedK => 'ʞ',
            IPAExtensions::LatinLetterSmallCapitalL => 'ʟ',
            IPAExtensions::LatinSmallLetterQWithHook => 'ʠ',
            IPAExtensions::LatinLetterGlottalStopWithStroke => 'ʡ',
            IPAExtensions::LatinLetterReversedGlottalStopWithStroke => 'ʢ',
            IPAExtensions::LatinSmallLetterDzDigraph => 'ʣ',
            IPAExtensions::LatinSmallLetterDezhDigraph => 'ʤ',
            IPAExtensions::LatinSmallLetterDzDigraphWithCurl => 'ʥ',
            IPAExtensions::LatinSmallLetterTsDigraph => 'ʦ',
            IPAExtensions::LatinSmallLetterTeshDigraph => 'ʧ',
            IPAExtensions::LatinSmallLetterTcDigraphWithCurl => 'ʨ',
            IPAExtensions::LatinSmallLetterFengDigraph => 'ʩ',
            IPAExtensions::LatinSmallLetterLsDigraph => 'ʪ',
            IPAExtensions::LatinSmallLetterLzDigraph => 'ʫ',
            IPAExtensions::LatinLetterBilabialPercussive => 'ʬ',
            IPAExtensions::LatinLetterBidentalPercussive => 'ʭ',
            IPAExtensions::LatinSmallLetterTurnedHWithFishhook => 'ʮ',
        }
    }
}

impl std::convert::TryFrom<char> for IPAExtensions {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ɐ' => Ok(IPAExtensions::LatinSmallLetterTurnedA),
            'ɑ' => Ok(IPAExtensions::LatinSmallLetterAlpha),
            'ɒ' => Ok(IPAExtensions::LatinSmallLetterTurnedAlpha),
            'ɓ' => Ok(IPAExtensions::LatinSmallLetterBWithHook),
            'ɔ' => Ok(IPAExtensions::LatinSmallLetterOpenO),
            'ɕ' => Ok(IPAExtensions::LatinSmallLetterCWithCurl),
            'ɖ' => Ok(IPAExtensions::LatinSmallLetterDWithTail),
            'ɗ' => Ok(IPAExtensions::LatinSmallLetterDWithHook),
            'ɘ' => Ok(IPAExtensions::LatinSmallLetterReversedE),
            'ə' => Ok(IPAExtensions::LatinSmallLetterSchwa),
            'ɚ' => Ok(IPAExtensions::LatinSmallLetterSchwaWithHook),
            'ɛ' => Ok(IPAExtensions::LatinSmallLetterOpenE),
            'ɜ' => Ok(IPAExtensions::LatinSmallLetterReversedOpenE),
            'ɝ' => Ok(IPAExtensions::LatinSmallLetterReversedOpenEWithHook),
            'ɞ' => Ok(IPAExtensions::LatinSmallLetterClosedReversedOpenE),
            'ɟ' => Ok(IPAExtensions::LatinSmallLetterDotlessJWithStroke),
            'ɠ' => Ok(IPAExtensions::LatinSmallLetterGWithHook),
            'ɡ' => Ok(IPAExtensions::LatinSmallLetterScriptG),
            'ɢ' => Ok(IPAExtensions::LatinLetterSmallCapitalG),
            'ɣ' => Ok(IPAExtensions::LatinSmallLetterGamma),
            'ɤ' => Ok(IPAExtensions::LatinSmallLetterRamsHorn),
            'ɥ' => Ok(IPAExtensions::LatinSmallLetterTurnedH),
            'ɦ' => Ok(IPAExtensions::LatinSmallLetterHWithHook),
            'ɧ' => Ok(IPAExtensions::LatinSmallLetterHengWithHook),
            'ɨ' => Ok(IPAExtensions::LatinSmallLetterIWithStroke),
            'ɩ' => Ok(IPAExtensions::LatinSmallLetterIota),
            'ɪ' => Ok(IPAExtensions::LatinLetterSmallCapitalI),
            'ɫ' => Ok(IPAExtensions::LatinSmallLetterLWithMiddleTilde),
            'ɬ' => Ok(IPAExtensions::LatinSmallLetterLWithBelt),
            'ɭ' => Ok(IPAExtensions::LatinSmallLetterLWithRetroflexHook),
            'ɮ' => Ok(IPAExtensions::LatinSmallLetterLezh),
            'ɯ' => Ok(IPAExtensions::LatinSmallLetterTurnedM),
            'ɰ' => Ok(IPAExtensions::LatinSmallLetterTurnedMWithLongLeg),
            'ɱ' => Ok(IPAExtensions::LatinSmallLetterMWithHook),
            'ɲ' => Ok(IPAExtensions::LatinSmallLetterNWithLeftHook),
            'ɳ' => Ok(IPAExtensions::LatinSmallLetterNWithRetroflexHook),
            'ɴ' => Ok(IPAExtensions::LatinLetterSmallCapitalN),
            'ɵ' => Ok(IPAExtensions::LatinSmallLetterBarredO),
            'ɶ' => Ok(IPAExtensions::LatinLetterSmallCapitalOe),
            'ɷ' => Ok(IPAExtensions::LatinSmallLetterClosedOmega),
            'ɸ' => Ok(IPAExtensions::LatinSmallLetterPhi),
            'ɹ' => Ok(IPAExtensions::LatinSmallLetterTurnedR),
            'ɺ' => Ok(IPAExtensions::LatinSmallLetterTurnedRWithLongLeg),
            'ɻ' => Ok(IPAExtensions::LatinSmallLetterTurnedRWithHook),
            'ɼ' => Ok(IPAExtensions::LatinSmallLetterRWithLongLeg),
            'ɽ' => Ok(IPAExtensions::LatinSmallLetterRWithTail),
            'ɾ' => Ok(IPAExtensions::LatinSmallLetterRWithFishhook),
            'ɿ' => Ok(IPAExtensions::LatinSmallLetterReversedRWithFishhook),
            'ʀ' => Ok(IPAExtensions::LatinLetterSmallCapitalR),
            'ʁ' => Ok(IPAExtensions::LatinLetterSmallCapitalInvertedR),
            'ʂ' => Ok(IPAExtensions::LatinSmallLetterSWithHook),
            'ʃ' => Ok(IPAExtensions::LatinSmallLetterEsh),
            'ʄ' => Ok(IPAExtensions::LatinSmallLetterDotlessJWithStrokeAndHook),
            'ʅ' => Ok(IPAExtensions::LatinSmallLetterSquatReversedEsh),
            'ʆ' => Ok(IPAExtensions::LatinSmallLetterEshWithCurl),
            'ʇ' => Ok(IPAExtensions::LatinSmallLetterTurnedT),
            'ʈ' => Ok(IPAExtensions::LatinSmallLetterTWithRetroflexHook),
            'ʉ' => Ok(IPAExtensions::LatinSmallLetterUBar),
            'ʊ' => Ok(IPAExtensions::LatinSmallLetterUpsilon),
            'ʋ' => Ok(IPAExtensions::LatinSmallLetterVWithHook),
            'ʌ' => Ok(IPAExtensions::LatinSmallLetterTurnedV),
            'ʍ' => Ok(IPAExtensions::LatinSmallLetterTurnedW),
            'ʎ' => Ok(IPAExtensions::LatinSmallLetterTurnedY),
            'ʏ' => Ok(IPAExtensions::LatinLetterSmallCapitalY),
            'ʐ' => Ok(IPAExtensions::LatinSmallLetterZWithRetroflexHook),
            'ʑ' => Ok(IPAExtensions::LatinSmallLetterZWithCurl),
            'ʒ' => Ok(IPAExtensions::LatinSmallLetterEzh),
            'ʓ' => Ok(IPAExtensions::LatinSmallLetterEzhWithCurl),
            'ʔ' => Ok(IPAExtensions::LatinLetterGlottalStop),
            'ʕ' => Ok(IPAExtensions::LatinLetterPharyngealVoicedFricative),
            'ʖ' => Ok(IPAExtensions::LatinLetterInvertedGlottalStop),
            'ʗ' => Ok(IPAExtensions::LatinLetterStretchedC),
            'ʘ' => Ok(IPAExtensions::LatinLetterBilabialClick),
            'ʙ' => Ok(IPAExtensions::LatinLetterSmallCapitalB),
            'ʚ' => Ok(IPAExtensions::LatinSmallLetterClosedOpenE),
            'ʛ' => Ok(IPAExtensions::LatinLetterSmallCapitalGWithHook),
            'ʜ' => Ok(IPAExtensions::LatinLetterSmallCapitalH),
            'ʝ' => Ok(IPAExtensions::LatinSmallLetterJWithCrossedDashTail),
            'ʞ' => Ok(IPAExtensions::LatinSmallLetterTurnedK),
            'ʟ' => Ok(IPAExtensions::LatinLetterSmallCapitalL),
            'ʠ' => Ok(IPAExtensions::LatinSmallLetterQWithHook),
            'ʡ' => Ok(IPAExtensions::LatinLetterGlottalStopWithStroke),
            'ʢ' => Ok(IPAExtensions::LatinLetterReversedGlottalStopWithStroke),
            'ʣ' => Ok(IPAExtensions::LatinSmallLetterDzDigraph),
            'ʤ' => Ok(IPAExtensions::LatinSmallLetterDezhDigraph),
            'ʥ' => Ok(IPAExtensions::LatinSmallLetterDzDigraphWithCurl),
            'ʦ' => Ok(IPAExtensions::LatinSmallLetterTsDigraph),
            'ʧ' => Ok(IPAExtensions::LatinSmallLetterTeshDigraph),
            'ʨ' => Ok(IPAExtensions::LatinSmallLetterTcDigraphWithCurl),
            'ʩ' => Ok(IPAExtensions::LatinSmallLetterFengDigraph),
            'ʪ' => Ok(IPAExtensions::LatinSmallLetterLsDigraph),
            'ʫ' => Ok(IPAExtensions::LatinSmallLetterLzDigraph),
            'ʬ' => Ok(IPAExtensions::LatinLetterBilabialPercussive),
            'ʭ' => Ok(IPAExtensions::LatinLetterBidentalPercussive),
            'ʮ' => Ok(IPAExtensions::LatinSmallLetterTurnedHWithFishhook),
            _ => Err(()),
        }
    }
}

impl Into<u32> for IPAExtensions {
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

impl std::convert::TryFrom<u32> for IPAExtensions {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for IPAExtensions {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl IPAExtensions {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        IPAExtensions::LatinSmallLetterTurnedA
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("IPAExtensions{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
