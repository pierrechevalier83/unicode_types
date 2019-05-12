
/// An enum to represent all characters in the LatinExtendedD block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum LatinExtendedD {
    /// \u{a720}: '꜠'
    ModifierLetterStressAndHighTone,
    /// \u{a721}: '꜡'
    ModifierLetterStressAndLowTone,
    /// \u{a722}: 'Ꜣ'
    LatinCapitalLetterEgyptologicalAlef,
    /// \u{a723}: 'ꜣ'
    LatinSmallLetterEgyptologicalAlef,
    /// \u{a724}: 'Ꜥ'
    LatinCapitalLetterEgyptologicalAin,
    /// \u{a725}: 'ꜥ'
    LatinSmallLetterEgyptologicalAin,
    /// \u{a726}: 'Ꜧ'
    LatinCapitalLetterHeng,
    /// \u{a727}: 'ꜧ'
    LatinSmallLetterHeng,
    /// \u{a728}: 'Ꜩ'
    LatinCapitalLetterTz,
    /// \u{a729}: 'ꜩ'
    LatinSmallLetterTz,
    /// \u{a72a}: 'Ꜫ'
    LatinCapitalLetterTresillo,
    /// \u{a72b}: 'ꜫ'
    LatinSmallLetterTresillo,
    /// \u{a72c}: 'Ꜭ'
    LatinCapitalLetterCuatrillo,
    /// \u{a72d}: 'ꜭ'
    LatinSmallLetterCuatrillo,
    /// \u{a72e}: 'Ꜯ'
    LatinCapitalLetterCuatrilloWithComma,
    /// \u{a72f}: 'ꜯ'
    LatinSmallLetterCuatrilloWithComma,
    /// \u{a730}: 'ꜰ'
    LatinLetterSmallCapitalF,
    /// \u{a731}: 'ꜱ'
    LatinLetterSmallCapitalS,
    /// \u{a732}: 'Ꜳ'
    LatinCapitalLetterAa,
    /// \u{a733}: 'ꜳ'
    LatinSmallLetterAa,
    /// \u{a734}: 'Ꜵ'
    LatinCapitalLetterAo,
    /// \u{a735}: 'ꜵ'
    LatinSmallLetterAo,
    /// \u{a736}: 'Ꜷ'
    LatinCapitalLetterAu,
    /// \u{a737}: 'ꜷ'
    LatinSmallLetterAu,
    /// \u{a738}: 'Ꜹ'
    LatinCapitalLetterAv,
    /// \u{a739}: 'ꜹ'
    LatinSmallLetterAv,
    /// \u{a73a}: 'Ꜻ'
    LatinCapitalLetterAvWithHorizontalBar,
    /// \u{a73b}: 'ꜻ'
    LatinSmallLetterAvWithHorizontalBar,
    /// \u{a73c}: 'Ꜽ'
    LatinCapitalLetterAy,
    /// \u{a73d}: 'ꜽ'
    LatinSmallLetterAy,
    /// \u{a73e}: 'Ꜿ'
    LatinCapitalLetterReversedCWithDot,
    /// \u{a73f}: 'ꜿ'
    LatinSmallLetterReversedCWithDot,
    /// \u{a740}: 'Ꝁ'
    LatinCapitalLetterKWithStroke,
    /// \u{a741}: 'ꝁ'
    LatinSmallLetterKWithStroke,
    /// \u{a742}: 'Ꝃ'
    LatinCapitalLetterKWithDiagonalStroke,
    /// \u{a743}: 'ꝃ'
    LatinSmallLetterKWithDiagonalStroke,
    /// \u{a744}: 'Ꝅ'
    LatinCapitalLetterKWithStrokeAndDiagonalStroke,
    /// \u{a745}: 'ꝅ'
    LatinSmallLetterKWithStrokeAndDiagonalStroke,
    /// \u{a746}: 'Ꝇ'
    LatinCapitalLetterBrokenL,
    /// \u{a747}: 'ꝇ'
    LatinSmallLetterBrokenL,
    /// \u{a748}: 'Ꝉ'
    LatinCapitalLetterLWithHighStroke,
    /// \u{a749}: 'ꝉ'
    LatinSmallLetterLWithHighStroke,
    /// \u{a74a}: 'Ꝋ'
    LatinCapitalLetterOWithLongStrokeOverlay,
    /// \u{a74b}: 'ꝋ'
    LatinSmallLetterOWithLongStrokeOverlay,
    /// \u{a74c}: 'Ꝍ'
    LatinCapitalLetterOWithLoop,
    /// \u{a74d}: 'ꝍ'
    LatinSmallLetterOWithLoop,
    /// \u{a74e}: 'Ꝏ'
    LatinCapitalLetterOo,
    /// \u{a74f}: 'ꝏ'
    LatinSmallLetterOo,
    /// \u{a750}: 'Ꝑ'
    LatinCapitalLetterPWithStrokeThroughDescender,
    /// \u{a751}: 'ꝑ'
    LatinSmallLetterPWithStrokeThroughDescender,
    /// \u{a752}: 'Ꝓ'
    LatinCapitalLetterPWithFlourish,
    /// \u{a753}: 'ꝓ'
    LatinSmallLetterPWithFlourish,
    /// \u{a754}: 'Ꝕ'
    LatinCapitalLetterPWithSquirrelTail,
    /// \u{a755}: 'ꝕ'
    LatinSmallLetterPWithSquirrelTail,
    /// \u{a756}: 'Ꝗ'
    LatinCapitalLetterQWithStrokeThroughDescender,
    /// \u{a757}: 'ꝗ'
    LatinSmallLetterQWithStrokeThroughDescender,
    /// \u{a758}: 'Ꝙ'
    LatinCapitalLetterQWithDiagonalStroke,
    /// \u{a759}: 'ꝙ'
    LatinSmallLetterQWithDiagonalStroke,
    /// \u{a75a}: 'Ꝛ'
    LatinCapitalLetterRRotunda,
    /// \u{a75b}: 'ꝛ'
    LatinSmallLetterRRotunda,
    /// \u{a75c}: 'Ꝝ'
    LatinCapitalLetterRumRotunda,
    /// \u{a75d}: 'ꝝ'
    LatinSmallLetterRumRotunda,
    /// \u{a75e}: 'Ꝟ'
    LatinCapitalLetterVWithDiagonalStroke,
    /// \u{a75f}: 'ꝟ'
    LatinSmallLetterVWithDiagonalStroke,
    /// \u{a760}: 'Ꝡ'
    LatinCapitalLetterVy,
    /// \u{a761}: 'ꝡ'
    LatinSmallLetterVy,
    /// \u{a762}: 'Ꝣ'
    LatinCapitalLetterVisigothicZ,
    /// \u{a763}: 'ꝣ'
    LatinSmallLetterVisigothicZ,
    /// \u{a764}: 'Ꝥ'
    LatinCapitalLetterThornWithStroke,
    /// \u{a765}: 'ꝥ'
    LatinSmallLetterThornWithStroke,
    /// \u{a766}: 'Ꝧ'
    LatinCapitalLetterThornWithStrokeThroughDescender,
    /// \u{a767}: 'ꝧ'
    LatinSmallLetterThornWithStrokeThroughDescender,
    /// \u{a768}: 'Ꝩ'
    LatinCapitalLetterVend,
    /// \u{a769}: 'ꝩ'
    LatinSmallLetterVend,
    /// \u{a76a}: 'Ꝫ'
    LatinCapitalLetterEt,
    /// \u{a76b}: 'ꝫ'
    LatinSmallLetterEt,
    /// \u{a76c}: 'Ꝭ'
    LatinCapitalLetterIs,
    /// \u{a76d}: 'ꝭ'
    LatinSmallLetterIs,
    /// \u{a76e}: 'Ꝯ'
    LatinCapitalLetterCon,
    /// \u{a76f}: 'ꝯ'
    LatinSmallLetterCon,
    /// \u{a770}: 'ꝰ'
    ModifierLetterUs,
    /// \u{a771}: 'ꝱ'
    LatinSmallLetterDum,
    /// \u{a772}: 'ꝲ'
    LatinSmallLetterLum,
    /// \u{a773}: 'ꝳ'
    LatinSmallLetterMum,
    /// \u{a774}: 'ꝴ'
    LatinSmallLetterNum,
    /// \u{a775}: 'ꝵ'
    LatinSmallLetterRum,
    /// \u{a776}: 'ꝶ'
    LatinLetterSmallCapitalRum,
    /// \u{a777}: 'ꝷ'
    LatinSmallLetterTum,
    /// \u{a778}: 'ꝸ'
    LatinSmallLetterUm,
    /// \u{a779}: 'Ꝺ'
    LatinCapitalLetterInsularD,
    /// \u{a77a}: 'ꝺ'
    LatinSmallLetterInsularD,
    /// \u{a77b}: 'Ꝼ'
    LatinCapitalLetterInsularF,
    /// \u{a77c}: 'ꝼ'
    LatinSmallLetterInsularF,
    /// \u{a77d}: 'Ᵹ'
    LatinCapitalLetterInsularG,
    /// \u{a77e}: 'Ꝿ'
    LatinCapitalLetterTurnedInsularG,
    /// \u{a77f}: 'ꝿ'
    LatinSmallLetterTurnedInsularG,
    /// \u{a780}: 'Ꞁ'
    LatinCapitalLetterTurnedL,
    /// \u{a781}: 'ꞁ'
    LatinSmallLetterTurnedL,
    /// \u{a782}: 'Ꞃ'
    LatinCapitalLetterInsularR,
    /// \u{a783}: 'ꞃ'
    LatinSmallLetterInsularR,
    /// \u{a784}: 'Ꞅ'
    LatinCapitalLetterInsularS,
    /// \u{a785}: 'ꞅ'
    LatinSmallLetterInsularS,
    /// \u{a786}: 'Ꞇ'
    LatinCapitalLetterInsularT,
    /// \u{a787}: 'ꞇ'
    LatinSmallLetterInsularT,
    /// \u{a788}: 'ꞈ'
    ModifierLetterLowCircumflexAccent,
    /// \u{a789}: '꞉'
    ModifierLetterColon,
    /// \u{a78a}: '꞊'
    ModifierLetterShortEqualsSign,
    /// \u{a78b}: 'Ꞌ'
    LatinCapitalLetterSaltillo,
    /// \u{a78c}: 'ꞌ'
    LatinSmallLetterSaltillo,
    /// \u{a78d}: 'Ɥ'
    LatinCapitalLetterTurnedH,
    /// \u{a78e}: 'ꞎ'
    LatinSmallLetterLWithRetroflexHookAndBelt,
    /// \u{a78f}: 'ꞏ'
    LatinLetterSinologicalDot,
    /// \u{a790}: 'Ꞑ'
    LatinCapitalLetterNWithDescender,
    /// \u{a791}: 'ꞑ'
    LatinSmallLetterNWithDescender,
    /// \u{a792}: 'Ꞓ'
    LatinCapitalLetterCWithBar,
    /// \u{a793}: 'ꞓ'
    LatinSmallLetterCWithBar,
    /// \u{a794}: 'ꞔ'
    LatinSmallLetterCWithPalatalHook,
    /// \u{a795}: 'ꞕ'
    LatinSmallLetterHWithPalatalHook,
    /// \u{a796}: 'Ꞗ'
    LatinCapitalLetterBWithFlourish,
    /// \u{a797}: 'ꞗ'
    LatinSmallLetterBWithFlourish,
    /// \u{a798}: 'Ꞙ'
    LatinCapitalLetterFWithStroke,
    /// \u{a799}: 'ꞙ'
    LatinSmallLetterFWithStroke,
    /// \u{a79a}: 'Ꞛ'
    LatinCapitalLetterVolapukAe,
    /// \u{a79b}: 'ꞛ'
    LatinSmallLetterVolapukAe,
    /// \u{a79c}: 'Ꞝ'
    LatinCapitalLetterVolapukOe,
    /// \u{a79d}: 'ꞝ'
    LatinSmallLetterVolapukOe,
    /// \u{a79e}: 'Ꞟ'
    LatinCapitalLetterVolapukUe,
    /// \u{a79f}: 'ꞟ'
    LatinSmallLetterVolapukUe,
    /// \u{a7a0}: 'Ꞡ'
    LatinCapitalLetterGWithObliqueStroke,
    /// \u{a7a1}: 'ꞡ'
    LatinSmallLetterGWithObliqueStroke,
    /// \u{a7a2}: 'Ꞣ'
    LatinCapitalLetterKWithObliqueStroke,
    /// \u{a7a3}: 'ꞣ'
    LatinSmallLetterKWithObliqueStroke,
    /// \u{a7a4}: 'Ꞥ'
    LatinCapitalLetterNWithObliqueStroke,
    /// \u{a7a5}: 'ꞥ'
    LatinSmallLetterNWithObliqueStroke,
    /// \u{a7a6}: 'Ꞧ'
    LatinCapitalLetterRWithObliqueStroke,
    /// \u{a7a7}: 'ꞧ'
    LatinSmallLetterRWithObliqueStroke,
    /// \u{a7a8}: 'Ꞩ'
    LatinCapitalLetterSWithObliqueStroke,
    /// \u{a7a9}: 'ꞩ'
    LatinSmallLetterSWithObliqueStroke,
    /// \u{a7aa}: 'Ɦ'
    LatinCapitalLetterHWithHook,
    /// \u{a7ab}: 'Ɜ'
    LatinCapitalLetterReversedOpenE,
    /// \u{a7ac}: 'Ɡ'
    LatinCapitalLetterScriptG,
    /// \u{a7ad}: 'Ɬ'
    LatinCapitalLetterLWithBelt,
    /// \u{a7ae}: 'Ɪ'
    LatinCapitalLetterSmallCapitalI,
    /// \u{a7af}: 'ꞯ'
    LatinLetterSmallCapitalQ,
    /// \u{a7b0}: 'Ʞ'
    LatinCapitalLetterTurnedK,
    /// \u{a7b1}: 'Ʇ'
    LatinCapitalLetterTurnedT,
    /// \u{a7b2}: 'Ʝ'
    LatinCapitalLetterJWithCrossedDashTail,
    /// \u{a7b3}: 'Ꭓ'
    LatinCapitalLetterChi,
    /// \u{a7b4}: 'Ꞵ'
    LatinCapitalLetterBeta,
    /// \u{a7b5}: 'ꞵ'
    LatinSmallLetterBeta,
    /// \u{a7b6}: 'Ꞷ'
    LatinCapitalLetterOmega,
    /// \u{a7b7}: 'ꞷ'
    LatinSmallLetterOmega,
    /// \u{a7b8}: 'Ꞹ'
    LatinCapitalLetterUWithStroke,
    /// \u{a7b9}: 'ꞹ'
    LatinSmallLetterUWithStroke,
    /// \u{a7ba}: 'Ꞻ'
    LatinCapitalLetterGlottalA,
    /// \u{a7bb}: 'ꞻ'
    LatinSmallLetterGlottalA,
    /// \u{a7bc}: 'Ꞽ'
    LatinCapitalLetterGlottalI,
    /// \u{a7bd}: 'ꞽ'
    LatinSmallLetterGlottalI,
    /// \u{a7be}: 'Ꞿ'
    LatinCapitalLetterGlottalU,
    /// \u{a7bf}: 'ꞿ'
    LatinSmallLetterGlottalU,
    /// \u{a7c2}: 'Ꟃ'
    LatinCapitalLetterAnglicanaW,
    /// \u{a7c3}: 'ꟃ'
    LatinSmallLetterAnglicanaW,
    /// \u{a7c4}: 'Ꞔ'
    LatinCapitalLetterCWithPalatalHook,
    /// \u{a7c5}: 'Ʂ'
    LatinCapitalLetterSWithHook,
    /// \u{a7c6}: 'Ᶎ'
    LatinCapitalLetterZWithPalatalHook,
    /// \u{a7f7}: 'ꟷ'
    LatinEpigraphicLetterSidewaysI,
    /// \u{a7f8}: 'ꟸ'
    ModifierLetterCapitalHWithStroke,
    /// \u{a7f9}: 'ꟹ'
    ModifierLetterSmallLigatureOe,
    /// \u{a7fa}: 'ꟺ'
    LatinLetterSmallCapitalTurnedM,
    /// \u{a7fb}: 'ꟻ'
    LatinEpigraphicLetterReversedF,
    /// \u{a7fc}: 'ꟼ'
    LatinEpigraphicLetterReversedP,
    /// \u{a7fd}: 'ꟽ'
    LatinEpigraphicLetterInvertedM,
    /// \u{a7fe}: 'ꟾ'
    LatinEpigraphicLetterILonga,
}

impl Into<char> for LatinExtendedD {
    fn into(self) -> char {
        match self {
            LatinExtendedD::ModifierLetterStressAndHighTone => '꜠',
            LatinExtendedD::ModifierLetterStressAndLowTone => '꜡',
            LatinExtendedD::LatinCapitalLetterEgyptologicalAlef => 'Ꜣ',
            LatinExtendedD::LatinSmallLetterEgyptologicalAlef => 'ꜣ',
            LatinExtendedD::LatinCapitalLetterEgyptologicalAin => 'Ꜥ',
            LatinExtendedD::LatinSmallLetterEgyptologicalAin => 'ꜥ',
            LatinExtendedD::LatinCapitalLetterHeng => 'Ꜧ',
            LatinExtendedD::LatinSmallLetterHeng => 'ꜧ',
            LatinExtendedD::LatinCapitalLetterTz => 'Ꜩ',
            LatinExtendedD::LatinSmallLetterTz => 'ꜩ',
            LatinExtendedD::LatinCapitalLetterTresillo => 'Ꜫ',
            LatinExtendedD::LatinSmallLetterTresillo => 'ꜫ',
            LatinExtendedD::LatinCapitalLetterCuatrillo => 'Ꜭ',
            LatinExtendedD::LatinSmallLetterCuatrillo => 'ꜭ',
            LatinExtendedD::LatinCapitalLetterCuatrilloWithComma => 'Ꜯ',
            LatinExtendedD::LatinSmallLetterCuatrilloWithComma => 'ꜯ',
            LatinExtendedD::LatinLetterSmallCapitalF => 'ꜰ',
            LatinExtendedD::LatinLetterSmallCapitalS => 'ꜱ',
            LatinExtendedD::LatinCapitalLetterAa => 'Ꜳ',
            LatinExtendedD::LatinSmallLetterAa => 'ꜳ',
            LatinExtendedD::LatinCapitalLetterAo => 'Ꜵ',
            LatinExtendedD::LatinSmallLetterAo => 'ꜵ',
            LatinExtendedD::LatinCapitalLetterAu => 'Ꜷ',
            LatinExtendedD::LatinSmallLetterAu => 'ꜷ',
            LatinExtendedD::LatinCapitalLetterAv => 'Ꜹ',
            LatinExtendedD::LatinSmallLetterAv => 'ꜹ',
            LatinExtendedD::LatinCapitalLetterAvWithHorizontalBar => 'Ꜻ',
            LatinExtendedD::LatinSmallLetterAvWithHorizontalBar => 'ꜻ',
            LatinExtendedD::LatinCapitalLetterAy => 'Ꜽ',
            LatinExtendedD::LatinSmallLetterAy => 'ꜽ',
            LatinExtendedD::LatinCapitalLetterReversedCWithDot => 'Ꜿ',
            LatinExtendedD::LatinSmallLetterReversedCWithDot => 'ꜿ',
            LatinExtendedD::LatinCapitalLetterKWithStroke => 'Ꝁ',
            LatinExtendedD::LatinSmallLetterKWithStroke => 'ꝁ',
            LatinExtendedD::LatinCapitalLetterKWithDiagonalStroke => 'Ꝃ',
            LatinExtendedD::LatinSmallLetterKWithDiagonalStroke => 'ꝃ',
            LatinExtendedD::LatinCapitalLetterKWithStrokeAndDiagonalStroke => 'Ꝅ',
            LatinExtendedD::LatinSmallLetterKWithStrokeAndDiagonalStroke => 'ꝅ',
            LatinExtendedD::LatinCapitalLetterBrokenL => 'Ꝇ',
            LatinExtendedD::LatinSmallLetterBrokenL => 'ꝇ',
            LatinExtendedD::LatinCapitalLetterLWithHighStroke => 'Ꝉ',
            LatinExtendedD::LatinSmallLetterLWithHighStroke => 'ꝉ',
            LatinExtendedD::LatinCapitalLetterOWithLongStrokeOverlay => 'Ꝋ',
            LatinExtendedD::LatinSmallLetterOWithLongStrokeOverlay => 'ꝋ',
            LatinExtendedD::LatinCapitalLetterOWithLoop => 'Ꝍ',
            LatinExtendedD::LatinSmallLetterOWithLoop => 'ꝍ',
            LatinExtendedD::LatinCapitalLetterOo => 'Ꝏ',
            LatinExtendedD::LatinSmallLetterOo => 'ꝏ',
            LatinExtendedD::LatinCapitalLetterPWithStrokeThroughDescender => 'Ꝑ',
            LatinExtendedD::LatinSmallLetterPWithStrokeThroughDescender => 'ꝑ',
            LatinExtendedD::LatinCapitalLetterPWithFlourish => 'Ꝓ',
            LatinExtendedD::LatinSmallLetterPWithFlourish => 'ꝓ',
            LatinExtendedD::LatinCapitalLetterPWithSquirrelTail => 'Ꝕ',
            LatinExtendedD::LatinSmallLetterPWithSquirrelTail => 'ꝕ',
            LatinExtendedD::LatinCapitalLetterQWithStrokeThroughDescender => 'Ꝗ',
            LatinExtendedD::LatinSmallLetterQWithStrokeThroughDescender => 'ꝗ',
            LatinExtendedD::LatinCapitalLetterQWithDiagonalStroke => 'Ꝙ',
            LatinExtendedD::LatinSmallLetterQWithDiagonalStroke => 'ꝙ',
            LatinExtendedD::LatinCapitalLetterRRotunda => 'Ꝛ',
            LatinExtendedD::LatinSmallLetterRRotunda => 'ꝛ',
            LatinExtendedD::LatinCapitalLetterRumRotunda => 'Ꝝ',
            LatinExtendedD::LatinSmallLetterRumRotunda => 'ꝝ',
            LatinExtendedD::LatinCapitalLetterVWithDiagonalStroke => 'Ꝟ',
            LatinExtendedD::LatinSmallLetterVWithDiagonalStroke => 'ꝟ',
            LatinExtendedD::LatinCapitalLetterVy => 'Ꝡ',
            LatinExtendedD::LatinSmallLetterVy => 'ꝡ',
            LatinExtendedD::LatinCapitalLetterVisigothicZ => 'Ꝣ',
            LatinExtendedD::LatinSmallLetterVisigothicZ => 'ꝣ',
            LatinExtendedD::LatinCapitalLetterThornWithStroke => 'Ꝥ',
            LatinExtendedD::LatinSmallLetterThornWithStroke => 'ꝥ',
            LatinExtendedD::LatinCapitalLetterThornWithStrokeThroughDescender => 'Ꝧ',
            LatinExtendedD::LatinSmallLetterThornWithStrokeThroughDescender => 'ꝧ',
            LatinExtendedD::LatinCapitalLetterVend => 'Ꝩ',
            LatinExtendedD::LatinSmallLetterVend => 'ꝩ',
            LatinExtendedD::LatinCapitalLetterEt => 'Ꝫ',
            LatinExtendedD::LatinSmallLetterEt => 'ꝫ',
            LatinExtendedD::LatinCapitalLetterIs => 'Ꝭ',
            LatinExtendedD::LatinSmallLetterIs => 'ꝭ',
            LatinExtendedD::LatinCapitalLetterCon => 'Ꝯ',
            LatinExtendedD::LatinSmallLetterCon => 'ꝯ',
            LatinExtendedD::ModifierLetterUs => 'ꝰ',
            LatinExtendedD::LatinSmallLetterDum => 'ꝱ',
            LatinExtendedD::LatinSmallLetterLum => 'ꝲ',
            LatinExtendedD::LatinSmallLetterMum => 'ꝳ',
            LatinExtendedD::LatinSmallLetterNum => 'ꝴ',
            LatinExtendedD::LatinSmallLetterRum => 'ꝵ',
            LatinExtendedD::LatinLetterSmallCapitalRum => 'ꝶ',
            LatinExtendedD::LatinSmallLetterTum => 'ꝷ',
            LatinExtendedD::LatinSmallLetterUm => 'ꝸ',
            LatinExtendedD::LatinCapitalLetterInsularD => 'Ꝺ',
            LatinExtendedD::LatinSmallLetterInsularD => 'ꝺ',
            LatinExtendedD::LatinCapitalLetterInsularF => 'Ꝼ',
            LatinExtendedD::LatinSmallLetterInsularF => 'ꝼ',
            LatinExtendedD::LatinCapitalLetterInsularG => 'Ᵹ',
            LatinExtendedD::LatinCapitalLetterTurnedInsularG => 'Ꝿ',
            LatinExtendedD::LatinSmallLetterTurnedInsularG => 'ꝿ',
            LatinExtendedD::LatinCapitalLetterTurnedL => 'Ꞁ',
            LatinExtendedD::LatinSmallLetterTurnedL => 'ꞁ',
            LatinExtendedD::LatinCapitalLetterInsularR => 'Ꞃ',
            LatinExtendedD::LatinSmallLetterInsularR => 'ꞃ',
            LatinExtendedD::LatinCapitalLetterInsularS => 'Ꞅ',
            LatinExtendedD::LatinSmallLetterInsularS => 'ꞅ',
            LatinExtendedD::LatinCapitalLetterInsularT => 'Ꞇ',
            LatinExtendedD::LatinSmallLetterInsularT => 'ꞇ',
            LatinExtendedD::ModifierLetterLowCircumflexAccent => 'ꞈ',
            LatinExtendedD::ModifierLetterColon => '꞉',
            LatinExtendedD::ModifierLetterShortEqualsSign => '꞊',
            LatinExtendedD::LatinCapitalLetterSaltillo => 'Ꞌ',
            LatinExtendedD::LatinSmallLetterSaltillo => 'ꞌ',
            LatinExtendedD::LatinCapitalLetterTurnedH => 'Ɥ',
            LatinExtendedD::LatinSmallLetterLWithRetroflexHookAndBelt => 'ꞎ',
            LatinExtendedD::LatinLetterSinologicalDot => 'ꞏ',
            LatinExtendedD::LatinCapitalLetterNWithDescender => 'Ꞑ',
            LatinExtendedD::LatinSmallLetterNWithDescender => 'ꞑ',
            LatinExtendedD::LatinCapitalLetterCWithBar => 'Ꞓ',
            LatinExtendedD::LatinSmallLetterCWithBar => 'ꞓ',
            LatinExtendedD::LatinSmallLetterCWithPalatalHook => 'ꞔ',
            LatinExtendedD::LatinSmallLetterHWithPalatalHook => 'ꞕ',
            LatinExtendedD::LatinCapitalLetterBWithFlourish => 'Ꞗ',
            LatinExtendedD::LatinSmallLetterBWithFlourish => 'ꞗ',
            LatinExtendedD::LatinCapitalLetterFWithStroke => 'Ꞙ',
            LatinExtendedD::LatinSmallLetterFWithStroke => 'ꞙ',
            LatinExtendedD::LatinCapitalLetterVolapukAe => 'Ꞛ',
            LatinExtendedD::LatinSmallLetterVolapukAe => 'ꞛ',
            LatinExtendedD::LatinCapitalLetterVolapukOe => 'Ꞝ',
            LatinExtendedD::LatinSmallLetterVolapukOe => 'ꞝ',
            LatinExtendedD::LatinCapitalLetterVolapukUe => 'Ꞟ',
            LatinExtendedD::LatinSmallLetterVolapukUe => 'ꞟ',
            LatinExtendedD::LatinCapitalLetterGWithObliqueStroke => 'Ꞡ',
            LatinExtendedD::LatinSmallLetterGWithObliqueStroke => 'ꞡ',
            LatinExtendedD::LatinCapitalLetterKWithObliqueStroke => 'Ꞣ',
            LatinExtendedD::LatinSmallLetterKWithObliqueStroke => 'ꞣ',
            LatinExtendedD::LatinCapitalLetterNWithObliqueStroke => 'Ꞥ',
            LatinExtendedD::LatinSmallLetterNWithObliqueStroke => 'ꞥ',
            LatinExtendedD::LatinCapitalLetterRWithObliqueStroke => 'Ꞧ',
            LatinExtendedD::LatinSmallLetterRWithObliqueStroke => 'ꞧ',
            LatinExtendedD::LatinCapitalLetterSWithObliqueStroke => 'Ꞩ',
            LatinExtendedD::LatinSmallLetterSWithObliqueStroke => 'ꞩ',
            LatinExtendedD::LatinCapitalLetterHWithHook => 'Ɦ',
            LatinExtendedD::LatinCapitalLetterReversedOpenE => 'Ɜ',
            LatinExtendedD::LatinCapitalLetterScriptG => 'Ɡ',
            LatinExtendedD::LatinCapitalLetterLWithBelt => 'Ɬ',
            LatinExtendedD::LatinCapitalLetterSmallCapitalI => 'Ɪ',
            LatinExtendedD::LatinLetterSmallCapitalQ => 'ꞯ',
            LatinExtendedD::LatinCapitalLetterTurnedK => 'Ʞ',
            LatinExtendedD::LatinCapitalLetterTurnedT => 'Ʇ',
            LatinExtendedD::LatinCapitalLetterJWithCrossedDashTail => 'Ʝ',
            LatinExtendedD::LatinCapitalLetterChi => 'Ꭓ',
            LatinExtendedD::LatinCapitalLetterBeta => 'Ꞵ',
            LatinExtendedD::LatinSmallLetterBeta => 'ꞵ',
            LatinExtendedD::LatinCapitalLetterOmega => 'Ꞷ',
            LatinExtendedD::LatinSmallLetterOmega => 'ꞷ',
            LatinExtendedD::LatinCapitalLetterUWithStroke => 'Ꞹ',
            LatinExtendedD::LatinSmallLetterUWithStroke => 'ꞹ',
            LatinExtendedD::LatinCapitalLetterGlottalA => 'Ꞻ',
            LatinExtendedD::LatinSmallLetterGlottalA => 'ꞻ',
            LatinExtendedD::LatinCapitalLetterGlottalI => 'Ꞽ',
            LatinExtendedD::LatinSmallLetterGlottalI => 'ꞽ',
            LatinExtendedD::LatinCapitalLetterGlottalU => 'Ꞿ',
            LatinExtendedD::LatinSmallLetterGlottalU => 'ꞿ',
            LatinExtendedD::LatinCapitalLetterAnglicanaW => 'Ꟃ',
            LatinExtendedD::LatinSmallLetterAnglicanaW => 'ꟃ',
            LatinExtendedD::LatinCapitalLetterCWithPalatalHook => 'Ꞔ',
            LatinExtendedD::LatinCapitalLetterSWithHook => 'Ʂ',
            LatinExtendedD::LatinCapitalLetterZWithPalatalHook => 'Ᶎ',
            LatinExtendedD::LatinEpigraphicLetterSidewaysI => 'ꟷ',
            LatinExtendedD::ModifierLetterCapitalHWithStroke => 'ꟸ',
            LatinExtendedD::ModifierLetterSmallLigatureOe => 'ꟹ',
            LatinExtendedD::LatinLetterSmallCapitalTurnedM => 'ꟺ',
            LatinExtendedD::LatinEpigraphicLetterReversedF => 'ꟻ',
            LatinExtendedD::LatinEpigraphicLetterReversedP => 'ꟼ',
            LatinExtendedD::LatinEpigraphicLetterInvertedM => 'ꟽ',
            LatinExtendedD::LatinEpigraphicLetterILonga => 'ꟾ',
        }
    }
}

impl std::convert::TryFrom<char> for LatinExtendedD {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '꜠' => Ok(LatinExtendedD::ModifierLetterStressAndHighTone),
            '꜡' => Ok(LatinExtendedD::ModifierLetterStressAndLowTone),
            'Ꜣ' => Ok(LatinExtendedD::LatinCapitalLetterEgyptologicalAlef),
            'ꜣ' => Ok(LatinExtendedD::LatinSmallLetterEgyptologicalAlef),
            'Ꜥ' => Ok(LatinExtendedD::LatinCapitalLetterEgyptologicalAin),
            'ꜥ' => Ok(LatinExtendedD::LatinSmallLetterEgyptologicalAin),
            'Ꜧ' => Ok(LatinExtendedD::LatinCapitalLetterHeng),
            'ꜧ' => Ok(LatinExtendedD::LatinSmallLetterHeng),
            'Ꜩ' => Ok(LatinExtendedD::LatinCapitalLetterTz),
            'ꜩ' => Ok(LatinExtendedD::LatinSmallLetterTz),
            'Ꜫ' => Ok(LatinExtendedD::LatinCapitalLetterTresillo),
            'ꜫ' => Ok(LatinExtendedD::LatinSmallLetterTresillo),
            'Ꜭ' => Ok(LatinExtendedD::LatinCapitalLetterCuatrillo),
            'ꜭ' => Ok(LatinExtendedD::LatinSmallLetterCuatrillo),
            'Ꜯ' => Ok(LatinExtendedD::LatinCapitalLetterCuatrilloWithComma),
            'ꜯ' => Ok(LatinExtendedD::LatinSmallLetterCuatrilloWithComma),
            'ꜰ' => Ok(LatinExtendedD::LatinLetterSmallCapitalF),
            'ꜱ' => Ok(LatinExtendedD::LatinLetterSmallCapitalS),
            'Ꜳ' => Ok(LatinExtendedD::LatinCapitalLetterAa),
            'ꜳ' => Ok(LatinExtendedD::LatinSmallLetterAa),
            'Ꜵ' => Ok(LatinExtendedD::LatinCapitalLetterAo),
            'ꜵ' => Ok(LatinExtendedD::LatinSmallLetterAo),
            'Ꜷ' => Ok(LatinExtendedD::LatinCapitalLetterAu),
            'ꜷ' => Ok(LatinExtendedD::LatinSmallLetterAu),
            'Ꜹ' => Ok(LatinExtendedD::LatinCapitalLetterAv),
            'ꜹ' => Ok(LatinExtendedD::LatinSmallLetterAv),
            'Ꜻ' => Ok(LatinExtendedD::LatinCapitalLetterAvWithHorizontalBar),
            'ꜻ' => Ok(LatinExtendedD::LatinSmallLetterAvWithHorizontalBar),
            'Ꜽ' => Ok(LatinExtendedD::LatinCapitalLetterAy),
            'ꜽ' => Ok(LatinExtendedD::LatinSmallLetterAy),
            'Ꜿ' => Ok(LatinExtendedD::LatinCapitalLetterReversedCWithDot),
            'ꜿ' => Ok(LatinExtendedD::LatinSmallLetterReversedCWithDot),
            'Ꝁ' => Ok(LatinExtendedD::LatinCapitalLetterKWithStroke),
            'ꝁ' => Ok(LatinExtendedD::LatinSmallLetterKWithStroke),
            'Ꝃ' => Ok(LatinExtendedD::LatinCapitalLetterKWithDiagonalStroke),
            'ꝃ' => Ok(LatinExtendedD::LatinSmallLetterKWithDiagonalStroke),
            'Ꝅ' => Ok(LatinExtendedD::LatinCapitalLetterKWithStrokeAndDiagonalStroke),
            'ꝅ' => Ok(LatinExtendedD::LatinSmallLetterKWithStrokeAndDiagonalStroke),
            'Ꝇ' => Ok(LatinExtendedD::LatinCapitalLetterBrokenL),
            'ꝇ' => Ok(LatinExtendedD::LatinSmallLetterBrokenL),
            'Ꝉ' => Ok(LatinExtendedD::LatinCapitalLetterLWithHighStroke),
            'ꝉ' => Ok(LatinExtendedD::LatinSmallLetterLWithHighStroke),
            'Ꝋ' => Ok(LatinExtendedD::LatinCapitalLetterOWithLongStrokeOverlay),
            'ꝋ' => Ok(LatinExtendedD::LatinSmallLetterOWithLongStrokeOverlay),
            'Ꝍ' => Ok(LatinExtendedD::LatinCapitalLetterOWithLoop),
            'ꝍ' => Ok(LatinExtendedD::LatinSmallLetterOWithLoop),
            'Ꝏ' => Ok(LatinExtendedD::LatinCapitalLetterOo),
            'ꝏ' => Ok(LatinExtendedD::LatinSmallLetterOo),
            'Ꝑ' => Ok(LatinExtendedD::LatinCapitalLetterPWithStrokeThroughDescender),
            'ꝑ' => Ok(LatinExtendedD::LatinSmallLetterPWithStrokeThroughDescender),
            'Ꝓ' => Ok(LatinExtendedD::LatinCapitalLetterPWithFlourish),
            'ꝓ' => Ok(LatinExtendedD::LatinSmallLetterPWithFlourish),
            'Ꝕ' => Ok(LatinExtendedD::LatinCapitalLetterPWithSquirrelTail),
            'ꝕ' => Ok(LatinExtendedD::LatinSmallLetterPWithSquirrelTail),
            'Ꝗ' => Ok(LatinExtendedD::LatinCapitalLetterQWithStrokeThroughDescender),
            'ꝗ' => Ok(LatinExtendedD::LatinSmallLetterQWithStrokeThroughDescender),
            'Ꝙ' => Ok(LatinExtendedD::LatinCapitalLetterQWithDiagonalStroke),
            'ꝙ' => Ok(LatinExtendedD::LatinSmallLetterQWithDiagonalStroke),
            'Ꝛ' => Ok(LatinExtendedD::LatinCapitalLetterRRotunda),
            'ꝛ' => Ok(LatinExtendedD::LatinSmallLetterRRotunda),
            'Ꝝ' => Ok(LatinExtendedD::LatinCapitalLetterRumRotunda),
            'ꝝ' => Ok(LatinExtendedD::LatinSmallLetterRumRotunda),
            'Ꝟ' => Ok(LatinExtendedD::LatinCapitalLetterVWithDiagonalStroke),
            'ꝟ' => Ok(LatinExtendedD::LatinSmallLetterVWithDiagonalStroke),
            'Ꝡ' => Ok(LatinExtendedD::LatinCapitalLetterVy),
            'ꝡ' => Ok(LatinExtendedD::LatinSmallLetterVy),
            'Ꝣ' => Ok(LatinExtendedD::LatinCapitalLetterVisigothicZ),
            'ꝣ' => Ok(LatinExtendedD::LatinSmallLetterVisigothicZ),
            'Ꝥ' => Ok(LatinExtendedD::LatinCapitalLetterThornWithStroke),
            'ꝥ' => Ok(LatinExtendedD::LatinSmallLetterThornWithStroke),
            'Ꝧ' => Ok(LatinExtendedD::LatinCapitalLetterThornWithStrokeThroughDescender),
            'ꝧ' => Ok(LatinExtendedD::LatinSmallLetterThornWithStrokeThroughDescender),
            'Ꝩ' => Ok(LatinExtendedD::LatinCapitalLetterVend),
            'ꝩ' => Ok(LatinExtendedD::LatinSmallLetterVend),
            'Ꝫ' => Ok(LatinExtendedD::LatinCapitalLetterEt),
            'ꝫ' => Ok(LatinExtendedD::LatinSmallLetterEt),
            'Ꝭ' => Ok(LatinExtendedD::LatinCapitalLetterIs),
            'ꝭ' => Ok(LatinExtendedD::LatinSmallLetterIs),
            'Ꝯ' => Ok(LatinExtendedD::LatinCapitalLetterCon),
            'ꝯ' => Ok(LatinExtendedD::LatinSmallLetterCon),
            'ꝰ' => Ok(LatinExtendedD::ModifierLetterUs),
            'ꝱ' => Ok(LatinExtendedD::LatinSmallLetterDum),
            'ꝲ' => Ok(LatinExtendedD::LatinSmallLetterLum),
            'ꝳ' => Ok(LatinExtendedD::LatinSmallLetterMum),
            'ꝴ' => Ok(LatinExtendedD::LatinSmallLetterNum),
            'ꝵ' => Ok(LatinExtendedD::LatinSmallLetterRum),
            'ꝶ' => Ok(LatinExtendedD::LatinLetterSmallCapitalRum),
            'ꝷ' => Ok(LatinExtendedD::LatinSmallLetterTum),
            'ꝸ' => Ok(LatinExtendedD::LatinSmallLetterUm),
            'Ꝺ' => Ok(LatinExtendedD::LatinCapitalLetterInsularD),
            'ꝺ' => Ok(LatinExtendedD::LatinSmallLetterInsularD),
            'Ꝼ' => Ok(LatinExtendedD::LatinCapitalLetterInsularF),
            'ꝼ' => Ok(LatinExtendedD::LatinSmallLetterInsularF),
            'Ᵹ' => Ok(LatinExtendedD::LatinCapitalLetterInsularG),
            'Ꝿ' => Ok(LatinExtendedD::LatinCapitalLetterTurnedInsularG),
            'ꝿ' => Ok(LatinExtendedD::LatinSmallLetterTurnedInsularG),
            'Ꞁ' => Ok(LatinExtendedD::LatinCapitalLetterTurnedL),
            'ꞁ' => Ok(LatinExtendedD::LatinSmallLetterTurnedL),
            'Ꞃ' => Ok(LatinExtendedD::LatinCapitalLetterInsularR),
            'ꞃ' => Ok(LatinExtendedD::LatinSmallLetterInsularR),
            'Ꞅ' => Ok(LatinExtendedD::LatinCapitalLetterInsularS),
            'ꞅ' => Ok(LatinExtendedD::LatinSmallLetterInsularS),
            'Ꞇ' => Ok(LatinExtendedD::LatinCapitalLetterInsularT),
            'ꞇ' => Ok(LatinExtendedD::LatinSmallLetterInsularT),
            'ꞈ' => Ok(LatinExtendedD::ModifierLetterLowCircumflexAccent),
            '꞉' => Ok(LatinExtendedD::ModifierLetterColon),
            '꞊' => Ok(LatinExtendedD::ModifierLetterShortEqualsSign),
            'Ꞌ' => Ok(LatinExtendedD::LatinCapitalLetterSaltillo),
            'ꞌ' => Ok(LatinExtendedD::LatinSmallLetterSaltillo),
            'Ɥ' => Ok(LatinExtendedD::LatinCapitalLetterTurnedH),
            'ꞎ' => Ok(LatinExtendedD::LatinSmallLetterLWithRetroflexHookAndBelt),
            'ꞏ' => Ok(LatinExtendedD::LatinLetterSinologicalDot),
            'Ꞑ' => Ok(LatinExtendedD::LatinCapitalLetterNWithDescender),
            'ꞑ' => Ok(LatinExtendedD::LatinSmallLetterNWithDescender),
            'Ꞓ' => Ok(LatinExtendedD::LatinCapitalLetterCWithBar),
            'ꞓ' => Ok(LatinExtendedD::LatinSmallLetterCWithBar),
            'ꞔ' => Ok(LatinExtendedD::LatinSmallLetterCWithPalatalHook),
            'ꞕ' => Ok(LatinExtendedD::LatinSmallLetterHWithPalatalHook),
            'Ꞗ' => Ok(LatinExtendedD::LatinCapitalLetterBWithFlourish),
            'ꞗ' => Ok(LatinExtendedD::LatinSmallLetterBWithFlourish),
            'Ꞙ' => Ok(LatinExtendedD::LatinCapitalLetterFWithStroke),
            'ꞙ' => Ok(LatinExtendedD::LatinSmallLetterFWithStroke),
            'Ꞛ' => Ok(LatinExtendedD::LatinCapitalLetterVolapukAe),
            'ꞛ' => Ok(LatinExtendedD::LatinSmallLetterVolapukAe),
            'Ꞝ' => Ok(LatinExtendedD::LatinCapitalLetterVolapukOe),
            'ꞝ' => Ok(LatinExtendedD::LatinSmallLetterVolapukOe),
            'Ꞟ' => Ok(LatinExtendedD::LatinCapitalLetterVolapukUe),
            'ꞟ' => Ok(LatinExtendedD::LatinSmallLetterVolapukUe),
            'Ꞡ' => Ok(LatinExtendedD::LatinCapitalLetterGWithObliqueStroke),
            'ꞡ' => Ok(LatinExtendedD::LatinSmallLetterGWithObliqueStroke),
            'Ꞣ' => Ok(LatinExtendedD::LatinCapitalLetterKWithObliqueStroke),
            'ꞣ' => Ok(LatinExtendedD::LatinSmallLetterKWithObliqueStroke),
            'Ꞥ' => Ok(LatinExtendedD::LatinCapitalLetterNWithObliqueStroke),
            'ꞥ' => Ok(LatinExtendedD::LatinSmallLetterNWithObliqueStroke),
            'Ꞧ' => Ok(LatinExtendedD::LatinCapitalLetterRWithObliqueStroke),
            'ꞧ' => Ok(LatinExtendedD::LatinSmallLetterRWithObliqueStroke),
            'Ꞩ' => Ok(LatinExtendedD::LatinCapitalLetterSWithObliqueStroke),
            'ꞩ' => Ok(LatinExtendedD::LatinSmallLetterSWithObliqueStroke),
            'Ɦ' => Ok(LatinExtendedD::LatinCapitalLetterHWithHook),
            'Ɜ' => Ok(LatinExtendedD::LatinCapitalLetterReversedOpenE),
            'Ɡ' => Ok(LatinExtendedD::LatinCapitalLetterScriptG),
            'Ɬ' => Ok(LatinExtendedD::LatinCapitalLetterLWithBelt),
            'Ɪ' => Ok(LatinExtendedD::LatinCapitalLetterSmallCapitalI),
            'ꞯ' => Ok(LatinExtendedD::LatinLetterSmallCapitalQ),
            'Ʞ' => Ok(LatinExtendedD::LatinCapitalLetterTurnedK),
            'Ʇ' => Ok(LatinExtendedD::LatinCapitalLetterTurnedT),
            'Ʝ' => Ok(LatinExtendedD::LatinCapitalLetterJWithCrossedDashTail),
            'Ꭓ' => Ok(LatinExtendedD::LatinCapitalLetterChi),
            'Ꞵ' => Ok(LatinExtendedD::LatinCapitalLetterBeta),
            'ꞵ' => Ok(LatinExtendedD::LatinSmallLetterBeta),
            'Ꞷ' => Ok(LatinExtendedD::LatinCapitalLetterOmega),
            'ꞷ' => Ok(LatinExtendedD::LatinSmallLetterOmega),
            'Ꞹ' => Ok(LatinExtendedD::LatinCapitalLetterUWithStroke),
            'ꞹ' => Ok(LatinExtendedD::LatinSmallLetterUWithStroke),
            'Ꞻ' => Ok(LatinExtendedD::LatinCapitalLetterGlottalA),
            'ꞻ' => Ok(LatinExtendedD::LatinSmallLetterGlottalA),
            'Ꞽ' => Ok(LatinExtendedD::LatinCapitalLetterGlottalI),
            'ꞽ' => Ok(LatinExtendedD::LatinSmallLetterGlottalI),
            'Ꞿ' => Ok(LatinExtendedD::LatinCapitalLetterGlottalU),
            'ꞿ' => Ok(LatinExtendedD::LatinSmallLetterGlottalU),
            'Ꟃ' => Ok(LatinExtendedD::LatinCapitalLetterAnglicanaW),
            'ꟃ' => Ok(LatinExtendedD::LatinSmallLetterAnglicanaW),
            'Ꞔ' => Ok(LatinExtendedD::LatinCapitalLetterCWithPalatalHook),
            'Ʂ' => Ok(LatinExtendedD::LatinCapitalLetterSWithHook),
            'Ᶎ' => Ok(LatinExtendedD::LatinCapitalLetterZWithPalatalHook),
            'ꟷ' => Ok(LatinExtendedD::LatinEpigraphicLetterSidewaysI),
            'ꟸ' => Ok(LatinExtendedD::ModifierLetterCapitalHWithStroke),
            'ꟹ' => Ok(LatinExtendedD::ModifierLetterSmallLigatureOe),
            'ꟺ' => Ok(LatinExtendedD::LatinLetterSmallCapitalTurnedM),
            'ꟻ' => Ok(LatinExtendedD::LatinEpigraphicLetterReversedF),
            'ꟼ' => Ok(LatinExtendedD::LatinEpigraphicLetterReversedP),
            'ꟽ' => Ok(LatinExtendedD::LatinEpigraphicLetterInvertedM),
            'ꟾ' => Ok(LatinExtendedD::LatinEpigraphicLetterILonga),
            _ => Err(()),
        }
    }
}

impl Into<u32> for LatinExtendedD {
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

impl std::convert::TryFrom<u32> for LatinExtendedD {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for LatinExtendedD {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl LatinExtendedD {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        LatinExtendedD::ModifierLetterStressAndHighTone
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            LatinExtendedD::ModifierLetterStressAndHighTone => "modifier letter stress and high tone",
            LatinExtendedD::ModifierLetterStressAndLowTone => "modifier letter stress and low tone",
            LatinExtendedD::LatinCapitalLetterEgyptologicalAlef => "latin capital letter egyptological alef",
            LatinExtendedD::LatinSmallLetterEgyptologicalAlef => "latin small letter egyptological alef",
            LatinExtendedD::LatinCapitalLetterEgyptologicalAin => "latin capital letter egyptological ain",
            LatinExtendedD::LatinSmallLetterEgyptologicalAin => "latin small letter egyptological ain",
            LatinExtendedD::LatinCapitalLetterHeng => "latin capital letter heng",
            LatinExtendedD::LatinSmallLetterHeng => "latin small letter heng",
            LatinExtendedD::LatinCapitalLetterTz => "latin capital letter tz",
            LatinExtendedD::LatinSmallLetterTz => "latin small letter tz",
            LatinExtendedD::LatinCapitalLetterTresillo => "latin capital letter tresillo",
            LatinExtendedD::LatinSmallLetterTresillo => "latin small letter tresillo",
            LatinExtendedD::LatinCapitalLetterCuatrillo => "latin capital letter cuatrillo",
            LatinExtendedD::LatinSmallLetterCuatrillo => "latin small letter cuatrillo",
            LatinExtendedD::LatinCapitalLetterCuatrilloWithComma => "latin capital letter cuatrillo with comma",
            LatinExtendedD::LatinSmallLetterCuatrilloWithComma => "latin small letter cuatrillo with comma",
            LatinExtendedD::LatinLetterSmallCapitalF => "latin letter small capital f",
            LatinExtendedD::LatinLetterSmallCapitalS => "latin letter small capital s",
            LatinExtendedD::LatinCapitalLetterAa => "latin capital letter aa",
            LatinExtendedD::LatinSmallLetterAa => "latin small letter aa",
            LatinExtendedD::LatinCapitalLetterAo => "latin capital letter ao",
            LatinExtendedD::LatinSmallLetterAo => "latin small letter ao",
            LatinExtendedD::LatinCapitalLetterAu => "latin capital letter au",
            LatinExtendedD::LatinSmallLetterAu => "latin small letter au",
            LatinExtendedD::LatinCapitalLetterAv => "latin capital letter av",
            LatinExtendedD::LatinSmallLetterAv => "latin small letter av",
            LatinExtendedD::LatinCapitalLetterAvWithHorizontalBar => "latin capital letter av with horizontal bar",
            LatinExtendedD::LatinSmallLetterAvWithHorizontalBar => "latin small letter av with horizontal bar",
            LatinExtendedD::LatinCapitalLetterAy => "latin capital letter ay",
            LatinExtendedD::LatinSmallLetterAy => "latin small letter ay",
            LatinExtendedD::LatinCapitalLetterReversedCWithDot => "latin capital letter reversed c with dot",
            LatinExtendedD::LatinSmallLetterReversedCWithDot => "latin small letter reversed c with dot",
            LatinExtendedD::LatinCapitalLetterKWithStroke => "latin capital letter k with stroke",
            LatinExtendedD::LatinSmallLetterKWithStroke => "latin small letter k with stroke",
            LatinExtendedD::LatinCapitalLetterKWithDiagonalStroke => "latin capital letter k with diagonal stroke",
            LatinExtendedD::LatinSmallLetterKWithDiagonalStroke => "latin small letter k with diagonal stroke",
            LatinExtendedD::LatinCapitalLetterKWithStrokeAndDiagonalStroke => "latin capital letter k with stroke and diagonal stroke",
            LatinExtendedD::LatinSmallLetterKWithStrokeAndDiagonalStroke => "latin small letter k with stroke and diagonal stroke",
            LatinExtendedD::LatinCapitalLetterBrokenL => "latin capital letter broken l",
            LatinExtendedD::LatinSmallLetterBrokenL => "latin small letter broken l",
            LatinExtendedD::LatinCapitalLetterLWithHighStroke => "latin capital letter l with high stroke",
            LatinExtendedD::LatinSmallLetterLWithHighStroke => "latin small letter l with high stroke",
            LatinExtendedD::LatinCapitalLetterOWithLongStrokeOverlay => "latin capital letter o with long stroke overlay",
            LatinExtendedD::LatinSmallLetterOWithLongStrokeOverlay => "latin small letter o with long stroke overlay",
            LatinExtendedD::LatinCapitalLetterOWithLoop => "latin capital letter o with loop",
            LatinExtendedD::LatinSmallLetterOWithLoop => "latin small letter o with loop",
            LatinExtendedD::LatinCapitalLetterOo => "latin capital letter oo",
            LatinExtendedD::LatinSmallLetterOo => "latin small letter oo",
            LatinExtendedD::LatinCapitalLetterPWithStrokeThroughDescender => "latin capital letter p with stroke through descender",
            LatinExtendedD::LatinSmallLetterPWithStrokeThroughDescender => "latin small letter p with stroke through descender",
            LatinExtendedD::LatinCapitalLetterPWithFlourish => "latin capital letter p with flourish",
            LatinExtendedD::LatinSmallLetterPWithFlourish => "latin small letter p with flourish",
            LatinExtendedD::LatinCapitalLetterPWithSquirrelTail => "latin capital letter p with squirrel tail",
            LatinExtendedD::LatinSmallLetterPWithSquirrelTail => "latin small letter p with squirrel tail",
            LatinExtendedD::LatinCapitalLetterQWithStrokeThroughDescender => "latin capital letter q with stroke through descender",
            LatinExtendedD::LatinSmallLetterQWithStrokeThroughDescender => "latin small letter q with stroke through descender",
            LatinExtendedD::LatinCapitalLetterQWithDiagonalStroke => "latin capital letter q with diagonal stroke",
            LatinExtendedD::LatinSmallLetterQWithDiagonalStroke => "latin small letter q with diagonal stroke",
            LatinExtendedD::LatinCapitalLetterRRotunda => "latin capital letter r rotunda",
            LatinExtendedD::LatinSmallLetterRRotunda => "latin small letter r rotunda",
            LatinExtendedD::LatinCapitalLetterRumRotunda => "latin capital letter rum rotunda",
            LatinExtendedD::LatinSmallLetterRumRotunda => "latin small letter rum rotunda",
            LatinExtendedD::LatinCapitalLetterVWithDiagonalStroke => "latin capital letter v with diagonal stroke",
            LatinExtendedD::LatinSmallLetterVWithDiagonalStroke => "latin small letter v with diagonal stroke",
            LatinExtendedD::LatinCapitalLetterVy => "latin capital letter vy",
            LatinExtendedD::LatinSmallLetterVy => "latin small letter vy",
            LatinExtendedD::LatinCapitalLetterVisigothicZ => "latin capital letter visigothic z",
            LatinExtendedD::LatinSmallLetterVisigothicZ => "latin small letter visigothic z",
            LatinExtendedD::LatinCapitalLetterThornWithStroke => "latin capital letter thorn with stroke",
            LatinExtendedD::LatinSmallLetterThornWithStroke => "latin small letter thorn with stroke",
            LatinExtendedD::LatinCapitalLetterThornWithStrokeThroughDescender => "latin capital letter thorn with stroke through descender",
            LatinExtendedD::LatinSmallLetterThornWithStrokeThroughDescender => "latin small letter thorn with stroke through descender",
            LatinExtendedD::LatinCapitalLetterVend => "latin capital letter vend",
            LatinExtendedD::LatinSmallLetterVend => "latin small letter vend",
            LatinExtendedD::LatinCapitalLetterEt => "latin capital letter et",
            LatinExtendedD::LatinSmallLetterEt => "latin small letter et",
            LatinExtendedD::LatinCapitalLetterIs => "latin capital letter is",
            LatinExtendedD::LatinSmallLetterIs => "latin small letter is",
            LatinExtendedD::LatinCapitalLetterCon => "latin capital letter con",
            LatinExtendedD::LatinSmallLetterCon => "latin small letter con",
            LatinExtendedD::ModifierLetterUs => "modifier letter us",
            LatinExtendedD::LatinSmallLetterDum => "latin small letter dum",
            LatinExtendedD::LatinSmallLetterLum => "latin small letter lum",
            LatinExtendedD::LatinSmallLetterMum => "latin small letter mum",
            LatinExtendedD::LatinSmallLetterNum => "latin small letter num",
            LatinExtendedD::LatinSmallLetterRum => "latin small letter rum",
            LatinExtendedD::LatinLetterSmallCapitalRum => "latin letter small capital rum",
            LatinExtendedD::LatinSmallLetterTum => "latin small letter tum",
            LatinExtendedD::LatinSmallLetterUm => "latin small letter um",
            LatinExtendedD::LatinCapitalLetterInsularD => "latin capital letter insular d",
            LatinExtendedD::LatinSmallLetterInsularD => "latin small letter insular d",
            LatinExtendedD::LatinCapitalLetterInsularF => "latin capital letter insular f",
            LatinExtendedD::LatinSmallLetterInsularF => "latin small letter insular f",
            LatinExtendedD::LatinCapitalLetterInsularG => "latin capital letter insular g",
            LatinExtendedD::LatinCapitalLetterTurnedInsularG => "latin capital letter turned insular g",
            LatinExtendedD::LatinSmallLetterTurnedInsularG => "latin small letter turned insular g",
            LatinExtendedD::LatinCapitalLetterTurnedL => "latin capital letter turned l",
            LatinExtendedD::LatinSmallLetterTurnedL => "latin small letter turned l",
            LatinExtendedD::LatinCapitalLetterInsularR => "latin capital letter insular r",
            LatinExtendedD::LatinSmallLetterInsularR => "latin small letter insular r",
            LatinExtendedD::LatinCapitalLetterInsularS => "latin capital letter insular s",
            LatinExtendedD::LatinSmallLetterInsularS => "latin small letter insular s",
            LatinExtendedD::LatinCapitalLetterInsularT => "latin capital letter insular t",
            LatinExtendedD::LatinSmallLetterInsularT => "latin small letter insular t",
            LatinExtendedD::ModifierLetterLowCircumflexAccent => "modifier letter low circumflex accent",
            LatinExtendedD::ModifierLetterColon => "modifier letter colon",
            LatinExtendedD::ModifierLetterShortEqualsSign => "modifier letter short equals sign",
            LatinExtendedD::LatinCapitalLetterSaltillo => "latin capital letter saltillo",
            LatinExtendedD::LatinSmallLetterSaltillo => "latin small letter saltillo",
            LatinExtendedD::LatinCapitalLetterTurnedH => "latin capital letter turned h",
            LatinExtendedD::LatinSmallLetterLWithRetroflexHookAndBelt => "latin small letter l with retroflex hook and belt",
            LatinExtendedD::LatinLetterSinologicalDot => "latin letter sinological dot",
            LatinExtendedD::LatinCapitalLetterNWithDescender => "latin capital letter n with descender",
            LatinExtendedD::LatinSmallLetterNWithDescender => "latin small letter n with descender",
            LatinExtendedD::LatinCapitalLetterCWithBar => "latin capital letter c with bar",
            LatinExtendedD::LatinSmallLetterCWithBar => "latin small letter c with bar",
            LatinExtendedD::LatinSmallLetterCWithPalatalHook => "latin small letter c with palatal hook",
            LatinExtendedD::LatinSmallLetterHWithPalatalHook => "latin small letter h with palatal hook",
            LatinExtendedD::LatinCapitalLetterBWithFlourish => "latin capital letter b with flourish",
            LatinExtendedD::LatinSmallLetterBWithFlourish => "latin small letter b with flourish",
            LatinExtendedD::LatinCapitalLetterFWithStroke => "latin capital letter f with stroke",
            LatinExtendedD::LatinSmallLetterFWithStroke => "latin small letter f with stroke",
            LatinExtendedD::LatinCapitalLetterVolapukAe => "latin capital letter volapuk ae",
            LatinExtendedD::LatinSmallLetterVolapukAe => "latin small letter volapuk ae",
            LatinExtendedD::LatinCapitalLetterVolapukOe => "latin capital letter volapuk oe",
            LatinExtendedD::LatinSmallLetterVolapukOe => "latin small letter volapuk oe",
            LatinExtendedD::LatinCapitalLetterVolapukUe => "latin capital letter volapuk ue",
            LatinExtendedD::LatinSmallLetterVolapukUe => "latin small letter volapuk ue",
            LatinExtendedD::LatinCapitalLetterGWithObliqueStroke => "latin capital letter g with oblique stroke",
            LatinExtendedD::LatinSmallLetterGWithObliqueStroke => "latin small letter g with oblique stroke",
            LatinExtendedD::LatinCapitalLetterKWithObliqueStroke => "latin capital letter k with oblique stroke",
            LatinExtendedD::LatinSmallLetterKWithObliqueStroke => "latin small letter k with oblique stroke",
            LatinExtendedD::LatinCapitalLetterNWithObliqueStroke => "latin capital letter n with oblique stroke",
            LatinExtendedD::LatinSmallLetterNWithObliqueStroke => "latin small letter n with oblique stroke",
            LatinExtendedD::LatinCapitalLetterRWithObliqueStroke => "latin capital letter r with oblique stroke",
            LatinExtendedD::LatinSmallLetterRWithObliqueStroke => "latin small letter r with oblique stroke",
            LatinExtendedD::LatinCapitalLetterSWithObliqueStroke => "latin capital letter s with oblique stroke",
            LatinExtendedD::LatinSmallLetterSWithObliqueStroke => "latin small letter s with oblique stroke",
            LatinExtendedD::LatinCapitalLetterHWithHook => "latin capital letter h with hook",
            LatinExtendedD::LatinCapitalLetterReversedOpenE => "latin capital letter reversed open e",
            LatinExtendedD::LatinCapitalLetterScriptG => "latin capital letter script g",
            LatinExtendedD::LatinCapitalLetterLWithBelt => "latin capital letter l with belt",
            LatinExtendedD::LatinCapitalLetterSmallCapitalI => "latin capital letter small capital i",
            LatinExtendedD::LatinLetterSmallCapitalQ => "latin letter small capital q",
            LatinExtendedD::LatinCapitalLetterTurnedK => "latin capital letter turned k",
            LatinExtendedD::LatinCapitalLetterTurnedT => "latin capital letter turned t",
            LatinExtendedD::LatinCapitalLetterJWithCrossedDashTail => "latin capital letter j with crossed-tail",
            LatinExtendedD::LatinCapitalLetterChi => "latin capital letter chi",
            LatinExtendedD::LatinCapitalLetterBeta => "latin capital letter beta",
            LatinExtendedD::LatinSmallLetterBeta => "latin small letter beta",
            LatinExtendedD::LatinCapitalLetterOmega => "latin capital letter omega",
            LatinExtendedD::LatinSmallLetterOmega => "latin small letter omega",
            LatinExtendedD::LatinCapitalLetterUWithStroke => "latin capital letter u with stroke",
            LatinExtendedD::LatinSmallLetterUWithStroke => "latin small letter u with stroke",
            LatinExtendedD::LatinCapitalLetterGlottalA => "latin capital letter glottal a",
            LatinExtendedD::LatinSmallLetterGlottalA => "latin small letter glottal a",
            LatinExtendedD::LatinCapitalLetterGlottalI => "latin capital letter glottal i",
            LatinExtendedD::LatinSmallLetterGlottalI => "latin small letter glottal i",
            LatinExtendedD::LatinCapitalLetterGlottalU => "latin capital letter glottal u",
            LatinExtendedD::LatinSmallLetterGlottalU => "latin small letter glottal u",
            LatinExtendedD::LatinCapitalLetterAnglicanaW => "latin capital letter anglicana w",
            LatinExtendedD::LatinSmallLetterAnglicanaW => "latin small letter anglicana w",
            LatinExtendedD::LatinCapitalLetterCWithPalatalHook => "latin capital letter c with palatal hook",
            LatinExtendedD::LatinCapitalLetterSWithHook => "latin capital letter s with hook",
            LatinExtendedD::LatinCapitalLetterZWithPalatalHook => "latin capital letter z with palatal hook",
            LatinExtendedD::LatinEpigraphicLetterSidewaysI => "latin epigraphic letter sideways i",
            LatinExtendedD::ModifierLetterCapitalHWithStroke => "modifier letter capital h with stroke",
            LatinExtendedD::ModifierLetterSmallLigatureOe => "modifier letter small ligature oe",
            LatinExtendedD::LatinLetterSmallCapitalTurnedM => "latin letter small capital turned m",
            LatinExtendedD::LatinEpigraphicLetterReversedF => "latin epigraphic letter reversed f",
            LatinExtendedD::LatinEpigraphicLetterReversedP => "latin epigraphic letter reversed p",
            LatinExtendedD::LatinEpigraphicLetterInvertedM => "latin epigraphic letter inverted m",
            LatinExtendedD::LatinEpigraphicLetterILonga => "latin epigraphic letter i longa",
        }
    }
}
