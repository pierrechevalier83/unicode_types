
/// An enum to represent all characters in the LatinExtendedB block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum LatinExtendedB {
    /// \u{180}: 'ƀ'
    LatinSmallLetterBWithStroke,
    /// \u{181}: 'Ɓ'
    LatinCapitalLetterBWithHook,
    /// \u{182}: 'Ƃ'
    LatinCapitalLetterBWithTopbar,
    /// \u{183}: 'ƃ'
    LatinSmallLetterBWithTopbar,
    /// \u{184}: 'Ƅ'
    LatinCapitalLetterToneSix,
    /// \u{185}: 'ƅ'
    LatinSmallLetterToneSix,
    /// \u{186}: 'Ɔ'
    LatinCapitalLetterOpenO,
    /// \u{187}: 'Ƈ'
    LatinCapitalLetterCWithHook,
    /// \u{188}: 'ƈ'
    LatinSmallLetterCWithHook,
    /// \u{189}: 'Ɖ'
    LatinCapitalLetterAfricanD,
    /// \u{18a}: 'Ɗ'
    LatinCapitalLetterDWithHook,
    /// \u{18b}: 'Ƌ'
    LatinCapitalLetterDWithTopbar,
    /// \u{18c}: 'ƌ'
    LatinSmallLetterDWithTopbar,
    /// \u{18d}: 'ƍ'
    LatinSmallLetterTurnedDelta,
    /// \u{18e}: 'Ǝ'
    LatinCapitalLetterReversedE,
    /// \u{18f}: 'Ə'
    LatinCapitalLetterSchwa,
    /// \u{190}: 'Ɛ'
    LatinCapitalLetterOpenE,
    /// \u{191}: 'Ƒ'
    LatinCapitalLetterFWithHook,
    /// \u{192}: 'ƒ'
    LatinSmallLetterFWithHook,
    /// \u{193}: 'Ɠ'
    LatinCapitalLetterGWithHook,
    /// \u{194}: 'Ɣ'
    LatinCapitalLetterGamma,
    /// \u{195}: 'ƕ'
    LatinSmallLetterHv,
    /// \u{196}: 'Ɩ'
    LatinCapitalLetterIota,
    /// \u{197}: 'Ɨ'
    LatinCapitalLetterIWithStroke,
    /// \u{198}: 'Ƙ'
    LatinCapitalLetterKWithHook,
    /// \u{199}: 'ƙ'
    LatinSmallLetterKWithHook,
    /// \u{19a}: 'ƚ'
    LatinSmallLetterLWithBar,
    /// \u{19b}: 'ƛ'
    LatinSmallLetterLambdaWithStroke,
    /// \u{19c}: 'Ɯ'
    LatinCapitalLetterTurnedM,
    /// \u{19d}: 'Ɲ'
    LatinCapitalLetterNWithLeftHook,
    /// \u{19e}: 'ƞ'
    LatinSmallLetterNWithLongRightLeg,
    /// \u{19f}: 'Ɵ'
    LatinCapitalLetterOWithMiddleTilde,
    /// \u{1a0}: 'Ơ'
    LatinCapitalLetterOWithHorn,
    /// \u{1a1}: 'ơ'
    LatinSmallLetterOWithHorn,
    /// \u{1a2}: 'Ƣ'
    LatinCapitalLetterOi,
    /// \u{1a3}: 'ƣ'
    LatinSmallLetterOi,
    /// \u{1a4}: 'Ƥ'
    LatinCapitalLetterPWithHook,
    /// \u{1a5}: 'ƥ'
    LatinSmallLetterPWithHook,
    /// \u{1a6}: 'Ʀ'
    LatinLetterYr,
    /// \u{1a7}: 'Ƨ'
    LatinCapitalLetterToneTwo,
    /// \u{1a8}: 'ƨ'
    LatinSmallLetterToneTwo,
    /// \u{1a9}: 'Ʃ'
    LatinCapitalLetterEsh,
    /// \u{1aa}: 'ƪ'
    LatinLetterReversedEshLoop,
    /// \u{1ab}: 'ƫ'
    LatinSmallLetterTWithPalatalHook,
    /// \u{1ac}: 'Ƭ'
    LatinCapitalLetterTWithHook,
    /// \u{1ad}: 'ƭ'
    LatinSmallLetterTWithHook,
    /// \u{1ae}: 'Ʈ'
    LatinCapitalLetterTWithRetroflexHook,
    /// \u{1af}: 'Ư'
    LatinCapitalLetterUWithHorn,
    /// \u{1b0}: 'ư'
    LatinSmallLetterUWithHorn,
    /// \u{1b1}: 'Ʊ'
    LatinCapitalLetterUpsilon,
    /// \u{1b2}: 'Ʋ'
    LatinCapitalLetterVWithHook,
    /// \u{1b3}: 'Ƴ'
    LatinCapitalLetterYWithHook,
    /// \u{1b4}: 'ƴ'
    LatinSmallLetterYWithHook,
    /// \u{1b5}: 'Ƶ'
    LatinCapitalLetterZWithStroke,
    /// \u{1b6}: 'ƶ'
    LatinSmallLetterZWithStroke,
    /// \u{1b7}: 'Ʒ'
    LatinCapitalLetterEzh,
    /// \u{1b8}: 'Ƹ'
    LatinCapitalLetterEzhReversed,
    /// \u{1b9}: 'ƹ'
    LatinSmallLetterEzhReversed,
    /// \u{1ba}: 'ƺ'
    LatinSmallLetterEzhWithTail,
    /// \u{1bb}: 'ƻ'
    LatinLetterTwoWithStroke,
    /// \u{1bc}: 'Ƽ'
    LatinCapitalLetterToneFive,
    /// \u{1bd}: 'ƽ'
    LatinSmallLetterToneFive,
    /// \u{1be}: 'ƾ'
    LatinLetterInvertedGlottalStopWithStroke,
    /// \u{1bf}: 'ƿ'
    LatinLetterWynn,
    /// \u{1c0}: 'ǀ'
    LatinLetterDentalClick,
    /// \u{1c1}: 'ǁ'
    LatinLetterLateralClick,
    /// \u{1c2}: 'ǂ'
    LatinLetterAlveolarClick,
    /// \u{1c3}: 'ǃ'
    LatinLetterRetroflexClick,
    /// \u{1c4}: 'Ǆ'
    LatinCapitalLetterDzWithCaron,
    /// \u{1c5}: 'ǅ'
    LatinCapitalLetterDWithSmallLetterZWithCaron,
    /// \u{1c6}: 'ǆ'
    LatinSmallLetterDzWithCaron,
    /// \u{1c7}: 'Ǉ'
    LatinCapitalLetterLj,
    /// \u{1c8}: 'ǈ'
    LatinCapitalLetterLWithSmallLetterJ,
    /// \u{1c9}: 'ǉ'
    LatinSmallLetterLj,
    /// \u{1ca}: 'Ǌ'
    LatinCapitalLetterNj,
    /// \u{1cb}: 'ǋ'
    LatinCapitalLetterNWithSmallLetterJ,
    /// \u{1cc}: 'ǌ'
    LatinSmallLetterNj,
    /// \u{1cd}: 'Ǎ'
    LatinCapitalLetterAWithCaron,
    /// \u{1ce}: 'ǎ'
    LatinSmallLetterAWithCaron,
    /// \u{1cf}: 'Ǐ'
    LatinCapitalLetterIWithCaron,
    /// \u{1d0}: 'ǐ'
    LatinSmallLetterIWithCaron,
    /// \u{1d1}: 'Ǒ'
    LatinCapitalLetterOWithCaron,
    /// \u{1d2}: 'ǒ'
    LatinSmallLetterOWithCaron,
    /// \u{1d3}: 'Ǔ'
    LatinCapitalLetterUWithCaron,
    /// \u{1d4}: 'ǔ'
    LatinSmallLetterUWithCaron,
    /// \u{1d5}: 'Ǖ'
    LatinCapitalLetterUWithDiaeresisAndMacron,
    /// \u{1d6}: 'ǖ'
    LatinSmallLetterUWithDiaeresisAndMacron,
    /// \u{1d7}: 'Ǘ'
    LatinCapitalLetterUWithDiaeresisAndAcute,
    /// \u{1d8}: 'ǘ'
    LatinSmallLetterUWithDiaeresisAndAcute,
    /// \u{1d9}: 'Ǚ'
    LatinCapitalLetterUWithDiaeresisAndCaron,
    /// \u{1da}: 'ǚ'
    LatinSmallLetterUWithDiaeresisAndCaron,
    /// \u{1db}: 'Ǜ'
    LatinCapitalLetterUWithDiaeresisAndGrave,
    /// \u{1dc}: 'ǜ'
    LatinSmallLetterUWithDiaeresisAndGrave,
    /// \u{1dd}: 'ǝ'
    LatinSmallLetterTurnedE,
    /// \u{1de}: 'Ǟ'
    LatinCapitalLetterAWithDiaeresisAndMacron,
    /// \u{1df}: 'ǟ'
    LatinSmallLetterAWithDiaeresisAndMacron,
    /// \u{1e0}: 'Ǡ'
    LatinCapitalLetterAWithDotAboveAndMacron,
    /// \u{1e1}: 'ǡ'
    LatinSmallLetterAWithDotAboveAndMacron,
    /// \u{1e2}: 'Ǣ'
    LatinCapitalLetterAeWithMacron,
    /// \u{1e3}: 'ǣ'
    LatinSmallLetterAeWithMacron,
    /// \u{1e4}: 'Ǥ'
    LatinCapitalLetterGWithStroke,
    /// \u{1e5}: 'ǥ'
    LatinSmallLetterGWithStroke,
    /// \u{1e6}: 'Ǧ'
    LatinCapitalLetterGWithCaron,
    /// \u{1e7}: 'ǧ'
    LatinSmallLetterGWithCaron,
    /// \u{1e8}: 'Ǩ'
    LatinCapitalLetterKWithCaron,
    /// \u{1e9}: 'ǩ'
    LatinSmallLetterKWithCaron,
    /// \u{1ea}: 'Ǫ'
    LatinCapitalLetterOWithOgonek,
    /// \u{1eb}: 'ǫ'
    LatinSmallLetterOWithOgonek,
    /// \u{1ec}: 'Ǭ'
    LatinCapitalLetterOWithOgonekAndMacron,
    /// \u{1ed}: 'ǭ'
    LatinSmallLetterOWithOgonekAndMacron,
    /// \u{1ee}: 'Ǯ'
    LatinCapitalLetterEzhWithCaron,
    /// \u{1ef}: 'ǯ'
    LatinSmallLetterEzhWithCaron,
    /// \u{1f0}: 'ǰ'
    LatinSmallLetterJWithCaron,
    /// \u{1f1}: 'Ǳ'
    LatinCapitalLetterDz,
    /// \u{1f2}: 'ǲ'
    LatinCapitalLetterDWithSmallLetterZ,
    /// \u{1f3}: 'ǳ'
    LatinSmallLetterDz,
    /// \u{1f4}: 'Ǵ'
    LatinCapitalLetterGWithAcute,
    /// \u{1f5}: 'ǵ'
    LatinSmallLetterGWithAcute,
    /// \u{1f6}: 'Ƕ'
    LatinCapitalLetterHwair,
    /// \u{1f7}: 'Ƿ'
    LatinCapitalLetterWynn,
    /// \u{1f8}: 'Ǹ'
    LatinCapitalLetterNWithGrave,
    /// \u{1f9}: 'ǹ'
    LatinSmallLetterNWithGrave,
    /// \u{1fa}: 'Ǻ'
    LatinCapitalLetterAWithRingAboveAndAcute,
    /// \u{1fb}: 'ǻ'
    LatinSmallLetterAWithRingAboveAndAcute,
    /// \u{1fc}: 'Ǽ'
    LatinCapitalLetterAeWithAcute,
    /// \u{1fd}: 'ǽ'
    LatinSmallLetterAeWithAcute,
    /// \u{1fe}: 'Ǿ'
    LatinCapitalLetterOWithStrokeAndAcute,
    /// \u{1ff}: 'ǿ'
    LatinSmallLetterOWithStrokeAndAcute,
    /// \u{200}: 'Ȁ'
    LatinCapitalLetterAWithDoubleGrave,
    /// \u{201}: 'ȁ'
    LatinSmallLetterAWithDoubleGrave,
    /// \u{202}: 'Ȃ'
    LatinCapitalLetterAWithInvertedBreve,
    /// \u{203}: 'ȃ'
    LatinSmallLetterAWithInvertedBreve,
    /// \u{204}: 'Ȅ'
    LatinCapitalLetterEWithDoubleGrave,
    /// \u{205}: 'ȅ'
    LatinSmallLetterEWithDoubleGrave,
    /// \u{206}: 'Ȇ'
    LatinCapitalLetterEWithInvertedBreve,
    /// \u{207}: 'ȇ'
    LatinSmallLetterEWithInvertedBreve,
    /// \u{208}: 'Ȉ'
    LatinCapitalLetterIWithDoubleGrave,
    /// \u{209}: 'ȉ'
    LatinSmallLetterIWithDoubleGrave,
    /// \u{20a}: 'Ȋ'
    LatinCapitalLetterIWithInvertedBreve,
    /// \u{20b}: 'ȋ'
    LatinSmallLetterIWithInvertedBreve,
    /// \u{20c}: 'Ȍ'
    LatinCapitalLetterOWithDoubleGrave,
    /// \u{20d}: 'ȍ'
    LatinSmallLetterOWithDoubleGrave,
    /// \u{20e}: 'Ȏ'
    LatinCapitalLetterOWithInvertedBreve,
    /// \u{20f}: 'ȏ'
    LatinSmallLetterOWithInvertedBreve,
    /// \u{210}: 'Ȑ'
    LatinCapitalLetterRWithDoubleGrave,
    /// \u{211}: 'ȑ'
    LatinSmallLetterRWithDoubleGrave,
    /// \u{212}: 'Ȓ'
    LatinCapitalLetterRWithInvertedBreve,
    /// \u{213}: 'ȓ'
    LatinSmallLetterRWithInvertedBreve,
    /// \u{214}: 'Ȕ'
    LatinCapitalLetterUWithDoubleGrave,
    /// \u{215}: 'ȕ'
    LatinSmallLetterUWithDoubleGrave,
    /// \u{216}: 'Ȗ'
    LatinCapitalLetterUWithInvertedBreve,
    /// \u{217}: 'ȗ'
    LatinSmallLetterUWithInvertedBreve,
    /// \u{218}: 'Ș'
    LatinCapitalLetterSWithCommaBelow,
    /// \u{219}: 'ș'
    LatinSmallLetterSWithCommaBelow,
    /// \u{21a}: 'Ț'
    LatinCapitalLetterTWithCommaBelow,
    /// \u{21b}: 'ț'
    LatinSmallLetterTWithCommaBelow,
    /// \u{21c}: 'Ȝ'
    LatinCapitalLetterYogh,
    /// \u{21d}: 'ȝ'
    LatinSmallLetterYogh,
    /// \u{21e}: 'Ȟ'
    LatinCapitalLetterHWithCaron,
    /// \u{21f}: 'ȟ'
    LatinSmallLetterHWithCaron,
    /// \u{220}: 'Ƞ'
    LatinCapitalLetterNWithLongRightLeg,
    /// \u{221}: 'ȡ'
    LatinSmallLetterDWithCurl,
    /// \u{222}: 'Ȣ'
    LatinCapitalLetterOu,
    /// \u{223}: 'ȣ'
    LatinSmallLetterOu,
    /// \u{224}: 'Ȥ'
    LatinCapitalLetterZWithHook,
    /// \u{225}: 'ȥ'
    LatinSmallLetterZWithHook,
    /// \u{226}: 'Ȧ'
    LatinCapitalLetterAWithDotAbove,
    /// \u{227}: 'ȧ'
    LatinSmallLetterAWithDotAbove,
    /// \u{228}: 'Ȩ'
    LatinCapitalLetterEWithCedilla,
    /// \u{229}: 'ȩ'
    LatinSmallLetterEWithCedilla,
    /// \u{22a}: 'Ȫ'
    LatinCapitalLetterOWithDiaeresisAndMacron,
    /// \u{22b}: 'ȫ'
    LatinSmallLetterOWithDiaeresisAndMacron,
    /// \u{22c}: 'Ȭ'
    LatinCapitalLetterOWithTildeAndMacron,
    /// \u{22d}: 'ȭ'
    LatinSmallLetterOWithTildeAndMacron,
    /// \u{22e}: 'Ȯ'
    LatinCapitalLetterOWithDotAbove,
    /// \u{22f}: 'ȯ'
    LatinSmallLetterOWithDotAbove,
    /// \u{230}: 'Ȱ'
    LatinCapitalLetterOWithDotAboveAndMacron,
    /// \u{231}: 'ȱ'
    LatinSmallLetterOWithDotAboveAndMacron,
    /// \u{232}: 'Ȳ'
    LatinCapitalLetterYWithMacron,
    /// \u{233}: 'ȳ'
    LatinSmallLetterYWithMacron,
    /// \u{234}: 'ȴ'
    LatinSmallLetterLWithCurl,
    /// \u{235}: 'ȵ'
    LatinSmallLetterNWithCurl,
    /// \u{236}: 'ȶ'
    LatinSmallLetterTWithCurl,
    /// \u{237}: 'ȷ'
    LatinSmallLetterDotlessJ,
    /// \u{238}: 'ȸ'
    LatinSmallLetterDbDigraph,
    /// \u{239}: 'ȹ'
    LatinSmallLetterQpDigraph,
    /// \u{23a}: 'Ⱥ'
    LatinCapitalLetterAWithStroke,
    /// \u{23b}: 'Ȼ'
    LatinCapitalLetterCWithStroke,
    /// \u{23c}: 'ȼ'
    LatinSmallLetterCWithStroke,
    /// \u{23d}: 'Ƚ'
    LatinCapitalLetterLWithBar,
    /// \u{23e}: 'Ⱦ'
    LatinCapitalLetterTWithDiagonalStroke,
    /// \u{23f}: 'ȿ'
    LatinSmallLetterSWithSwashTail,
    /// \u{240}: 'ɀ'
    LatinSmallLetterZWithSwashTail,
    /// \u{241}: 'Ɂ'
    LatinCapitalLetterGlottalStop,
    /// \u{242}: 'ɂ'
    LatinSmallLetterGlottalStop,
    /// \u{243}: 'Ƀ'
    LatinCapitalLetterBWithStroke,
    /// \u{244}: 'Ʉ'
    LatinCapitalLetterUBar,
    /// \u{245}: 'Ʌ'
    LatinCapitalLetterTurnedV,
    /// \u{246}: 'Ɇ'
    LatinCapitalLetterEWithStroke,
    /// \u{247}: 'ɇ'
    LatinSmallLetterEWithStroke,
    /// \u{248}: 'Ɉ'
    LatinCapitalLetterJWithStroke,
    /// \u{249}: 'ɉ'
    LatinSmallLetterJWithStroke,
    /// \u{24a}: 'Ɋ'
    LatinCapitalLetterSmallQWithHookTail,
    /// \u{24b}: 'ɋ'
    LatinSmallLetterQWithHookTail,
    /// \u{24c}: 'Ɍ'
    LatinCapitalLetterRWithStroke,
    /// \u{24d}: 'ɍ'
    LatinSmallLetterRWithStroke,
    /// \u{24e}: 'Ɏ'
    LatinCapitalLetterYWithStroke,
}

impl Into<char> for LatinExtendedB {
    fn into(self) -> char {
        match self {
            LatinExtendedB::LatinSmallLetterBWithStroke => 'ƀ',
            LatinExtendedB::LatinCapitalLetterBWithHook => 'Ɓ',
            LatinExtendedB::LatinCapitalLetterBWithTopbar => 'Ƃ',
            LatinExtendedB::LatinSmallLetterBWithTopbar => 'ƃ',
            LatinExtendedB::LatinCapitalLetterToneSix => 'Ƅ',
            LatinExtendedB::LatinSmallLetterToneSix => 'ƅ',
            LatinExtendedB::LatinCapitalLetterOpenO => 'Ɔ',
            LatinExtendedB::LatinCapitalLetterCWithHook => 'Ƈ',
            LatinExtendedB::LatinSmallLetterCWithHook => 'ƈ',
            LatinExtendedB::LatinCapitalLetterAfricanD => 'Ɖ',
            LatinExtendedB::LatinCapitalLetterDWithHook => 'Ɗ',
            LatinExtendedB::LatinCapitalLetterDWithTopbar => 'Ƌ',
            LatinExtendedB::LatinSmallLetterDWithTopbar => 'ƌ',
            LatinExtendedB::LatinSmallLetterTurnedDelta => 'ƍ',
            LatinExtendedB::LatinCapitalLetterReversedE => 'Ǝ',
            LatinExtendedB::LatinCapitalLetterSchwa => 'Ə',
            LatinExtendedB::LatinCapitalLetterOpenE => 'Ɛ',
            LatinExtendedB::LatinCapitalLetterFWithHook => 'Ƒ',
            LatinExtendedB::LatinSmallLetterFWithHook => 'ƒ',
            LatinExtendedB::LatinCapitalLetterGWithHook => 'Ɠ',
            LatinExtendedB::LatinCapitalLetterGamma => 'Ɣ',
            LatinExtendedB::LatinSmallLetterHv => 'ƕ',
            LatinExtendedB::LatinCapitalLetterIota => 'Ɩ',
            LatinExtendedB::LatinCapitalLetterIWithStroke => 'Ɨ',
            LatinExtendedB::LatinCapitalLetterKWithHook => 'Ƙ',
            LatinExtendedB::LatinSmallLetterKWithHook => 'ƙ',
            LatinExtendedB::LatinSmallLetterLWithBar => 'ƚ',
            LatinExtendedB::LatinSmallLetterLambdaWithStroke => 'ƛ',
            LatinExtendedB::LatinCapitalLetterTurnedM => 'Ɯ',
            LatinExtendedB::LatinCapitalLetterNWithLeftHook => 'Ɲ',
            LatinExtendedB::LatinSmallLetterNWithLongRightLeg => 'ƞ',
            LatinExtendedB::LatinCapitalLetterOWithMiddleTilde => 'Ɵ',
            LatinExtendedB::LatinCapitalLetterOWithHorn => 'Ơ',
            LatinExtendedB::LatinSmallLetterOWithHorn => 'ơ',
            LatinExtendedB::LatinCapitalLetterOi => 'Ƣ',
            LatinExtendedB::LatinSmallLetterOi => 'ƣ',
            LatinExtendedB::LatinCapitalLetterPWithHook => 'Ƥ',
            LatinExtendedB::LatinSmallLetterPWithHook => 'ƥ',
            LatinExtendedB::LatinLetterYr => 'Ʀ',
            LatinExtendedB::LatinCapitalLetterToneTwo => 'Ƨ',
            LatinExtendedB::LatinSmallLetterToneTwo => 'ƨ',
            LatinExtendedB::LatinCapitalLetterEsh => 'Ʃ',
            LatinExtendedB::LatinLetterReversedEshLoop => 'ƪ',
            LatinExtendedB::LatinSmallLetterTWithPalatalHook => 'ƫ',
            LatinExtendedB::LatinCapitalLetterTWithHook => 'Ƭ',
            LatinExtendedB::LatinSmallLetterTWithHook => 'ƭ',
            LatinExtendedB::LatinCapitalLetterTWithRetroflexHook => 'Ʈ',
            LatinExtendedB::LatinCapitalLetterUWithHorn => 'Ư',
            LatinExtendedB::LatinSmallLetterUWithHorn => 'ư',
            LatinExtendedB::LatinCapitalLetterUpsilon => 'Ʊ',
            LatinExtendedB::LatinCapitalLetterVWithHook => 'Ʋ',
            LatinExtendedB::LatinCapitalLetterYWithHook => 'Ƴ',
            LatinExtendedB::LatinSmallLetterYWithHook => 'ƴ',
            LatinExtendedB::LatinCapitalLetterZWithStroke => 'Ƶ',
            LatinExtendedB::LatinSmallLetterZWithStroke => 'ƶ',
            LatinExtendedB::LatinCapitalLetterEzh => 'Ʒ',
            LatinExtendedB::LatinCapitalLetterEzhReversed => 'Ƹ',
            LatinExtendedB::LatinSmallLetterEzhReversed => 'ƹ',
            LatinExtendedB::LatinSmallLetterEzhWithTail => 'ƺ',
            LatinExtendedB::LatinLetterTwoWithStroke => 'ƻ',
            LatinExtendedB::LatinCapitalLetterToneFive => 'Ƽ',
            LatinExtendedB::LatinSmallLetterToneFive => 'ƽ',
            LatinExtendedB::LatinLetterInvertedGlottalStopWithStroke => 'ƾ',
            LatinExtendedB::LatinLetterWynn => 'ƿ',
            LatinExtendedB::LatinLetterDentalClick => 'ǀ',
            LatinExtendedB::LatinLetterLateralClick => 'ǁ',
            LatinExtendedB::LatinLetterAlveolarClick => 'ǂ',
            LatinExtendedB::LatinLetterRetroflexClick => 'ǃ',
            LatinExtendedB::LatinCapitalLetterDzWithCaron => 'Ǆ',
            LatinExtendedB::LatinCapitalLetterDWithSmallLetterZWithCaron => 'ǅ',
            LatinExtendedB::LatinSmallLetterDzWithCaron => 'ǆ',
            LatinExtendedB::LatinCapitalLetterLj => 'Ǉ',
            LatinExtendedB::LatinCapitalLetterLWithSmallLetterJ => 'ǈ',
            LatinExtendedB::LatinSmallLetterLj => 'ǉ',
            LatinExtendedB::LatinCapitalLetterNj => 'Ǌ',
            LatinExtendedB::LatinCapitalLetterNWithSmallLetterJ => 'ǋ',
            LatinExtendedB::LatinSmallLetterNj => 'ǌ',
            LatinExtendedB::LatinCapitalLetterAWithCaron => 'Ǎ',
            LatinExtendedB::LatinSmallLetterAWithCaron => 'ǎ',
            LatinExtendedB::LatinCapitalLetterIWithCaron => 'Ǐ',
            LatinExtendedB::LatinSmallLetterIWithCaron => 'ǐ',
            LatinExtendedB::LatinCapitalLetterOWithCaron => 'Ǒ',
            LatinExtendedB::LatinSmallLetterOWithCaron => 'ǒ',
            LatinExtendedB::LatinCapitalLetterUWithCaron => 'Ǔ',
            LatinExtendedB::LatinSmallLetterUWithCaron => 'ǔ',
            LatinExtendedB::LatinCapitalLetterUWithDiaeresisAndMacron => 'Ǖ',
            LatinExtendedB::LatinSmallLetterUWithDiaeresisAndMacron => 'ǖ',
            LatinExtendedB::LatinCapitalLetterUWithDiaeresisAndAcute => 'Ǘ',
            LatinExtendedB::LatinSmallLetterUWithDiaeresisAndAcute => 'ǘ',
            LatinExtendedB::LatinCapitalLetterUWithDiaeresisAndCaron => 'Ǚ',
            LatinExtendedB::LatinSmallLetterUWithDiaeresisAndCaron => 'ǚ',
            LatinExtendedB::LatinCapitalLetterUWithDiaeresisAndGrave => 'Ǜ',
            LatinExtendedB::LatinSmallLetterUWithDiaeresisAndGrave => 'ǜ',
            LatinExtendedB::LatinSmallLetterTurnedE => 'ǝ',
            LatinExtendedB::LatinCapitalLetterAWithDiaeresisAndMacron => 'Ǟ',
            LatinExtendedB::LatinSmallLetterAWithDiaeresisAndMacron => 'ǟ',
            LatinExtendedB::LatinCapitalLetterAWithDotAboveAndMacron => 'Ǡ',
            LatinExtendedB::LatinSmallLetterAWithDotAboveAndMacron => 'ǡ',
            LatinExtendedB::LatinCapitalLetterAeWithMacron => 'Ǣ',
            LatinExtendedB::LatinSmallLetterAeWithMacron => 'ǣ',
            LatinExtendedB::LatinCapitalLetterGWithStroke => 'Ǥ',
            LatinExtendedB::LatinSmallLetterGWithStroke => 'ǥ',
            LatinExtendedB::LatinCapitalLetterGWithCaron => 'Ǧ',
            LatinExtendedB::LatinSmallLetterGWithCaron => 'ǧ',
            LatinExtendedB::LatinCapitalLetterKWithCaron => 'Ǩ',
            LatinExtendedB::LatinSmallLetterKWithCaron => 'ǩ',
            LatinExtendedB::LatinCapitalLetterOWithOgonek => 'Ǫ',
            LatinExtendedB::LatinSmallLetterOWithOgonek => 'ǫ',
            LatinExtendedB::LatinCapitalLetterOWithOgonekAndMacron => 'Ǭ',
            LatinExtendedB::LatinSmallLetterOWithOgonekAndMacron => 'ǭ',
            LatinExtendedB::LatinCapitalLetterEzhWithCaron => 'Ǯ',
            LatinExtendedB::LatinSmallLetterEzhWithCaron => 'ǯ',
            LatinExtendedB::LatinSmallLetterJWithCaron => 'ǰ',
            LatinExtendedB::LatinCapitalLetterDz => 'Ǳ',
            LatinExtendedB::LatinCapitalLetterDWithSmallLetterZ => 'ǲ',
            LatinExtendedB::LatinSmallLetterDz => 'ǳ',
            LatinExtendedB::LatinCapitalLetterGWithAcute => 'Ǵ',
            LatinExtendedB::LatinSmallLetterGWithAcute => 'ǵ',
            LatinExtendedB::LatinCapitalLetterHwair => 'Ƕ',
            LatinExtendedB::LatinCapitalLetterWynn => 'Ƿ',
            LatinExtendedB::LatinCapitalLetterNWithGrave => 'Ǹ',
            LatinExtendedB::LatinSmallLetterNWithGrave => 'ǹ',
            LatinExtendedB::LatinCapitalLetterAWithRingAboveAndAcute => 'Ǻ',
            LatinExtendedB::LatinSmallLetterAWithRingAboveAndAcute => 'ǻ',
            LatinExtendedB::LatinCapitalLetterAeWithAcute => 'Ǽ',
            LatinExtendedB::LatinSmallLetterAeWithAcute => 'ǽ',
            LatinExtendedB::LatinCapitalLetterOWithStrokeAndAcute => 'Ǿ',
            LatinExtendedB::LatinSmallLetterOWithStrokeAndAcute => 'ǿ',
            LatinExtendedB::LatinCapitalLetterAWithDoubleGrave => 'Ȁ',
            LatinExtendedB::LatinSmallLetterAWithDoubleGrave => 'ȁ',
            LatinExtendedB::LatinCapitalLetterAWithInvertedBreve => 'Ȃ',
            LatinExtendedB::LatinSmallLetterAWithInvertedBreve => 'ȃ',
            LatinExtendedB::LatinCapitalLetterEWithDoubleGrave => 'Ȅ',
            LatinExtendedB::LatinSmallLetterEWithDoubleGrave => 'ȅ',
            LatinExtendedB::LatinCapitalLetterEWithInvertedBreve => 'Ȇ',
            LatinExtendedB::LatinSmallLetterEWithInvertedBreve => 'ȇ',
            LatinExtendedB::LatinCapitalLetterIWithDoubleGrave => 'Ȉ',
            LatinExtendedB::LatinSmallLetterIWithDoubleGrave => 'ȉ',
            LatinExtendedB::LatinCapitalLetterIWithInvertedBreve => 'Ȋ',
            LatinExtendedB::LatinSmallLetterIWithInvertedBreve => 'ȋ',
            LatinExtendedB::LatinCapitalLetterOWithDoubleGrave => 'Ȍ',
            LatinExtendedB::LatinSmallLetterOWithDoubleGrave => 'ȍ',
            LatinExtendedB::LatinCapitalLetterOWithInvertedBreve => 'Ȏ',
            LatinExtendedB::LatinSmallLetterOWithInvertedBreve => 'ȏ',
            LatinExtendedB::LatinCapitalLetterRWithDoubleGrave => 'Ȑ',
            LatinExtendedB::LatinSmallLetterRWithDoubleGrave => 'ȑ',
            LatinExtendedB::LatinCapitalLetterRWithInvertedBreve => 'Ȓ',
            LatinExtendedB::LatinSmallLetterRWithInvertedBreve => 'ȓ',
            LatinExtendedB::LatinCapitalLetterUWithDoubleGrave => 'Ȕ',
            LatinExtendedB::LatinSmallLetterUWithDoubleGrave => 'ȕ',
            LatinExtendedB::LatinCapitalLetterUWithInvertedBreve => 'Ȗ',
            LatinExtendedB::LatinSmallLetterUWithInvertedBreve => 'ȗ',
            LatinExtendedB::LatinCapitalLetterSWithCommaBelow => 'Ș',
            LatinExtendedB::LatinSmallLetterSWithCommaBelow => 'ș',
            LatinExtendedB::LatinCapitalLetterTWithCommaBelow => 'Ț',
            LatinExtendedB::LatinSmallLetterTWithCommaBelow => 'ț',
            LatinExtendedB::LatinCapitalLetterYogh => 'Ȝ',
            LatinExtendedB::LatinSmallLetterYogh => 'ȝ',
            LatinExtendedB::LatinCapitalLetterHWithCaron => 'Ȟ',
            LatinExtendedB::LatinSmallLetterHWithCaron => 'ȟ',
            LatinExtendedB::LatinCapitalLetterNWithLongRightLeg => 'Ƞ',
            LatinExtendedB::LatinSmallLetterDWithCurl => 'ȡ',
            LatinExtendedB::LatinCapitalLetterOu => 'Ȣ',
            LatinExtendedB::LatinSmallLetterOu => 'ȣ',
            LatinExtendedB::LatinCapitalLetterZWithHook => 'Ȥ',
            LatinExtendedB::LatinSmallLetterZWithHook => 'ȥ',
            LatinExtendedB::LatinCapitalLetterAWithDotAbove => 'Ȧ',
            LatinExtendedB::LatinSmallLetterAWithDotAbove => 'ȧ',
            LatinExtendedB::LatinCapitalLetterEWithCedilla => 'Ȩ',
            LatinExtendedB::LatinSmallLetterEWithCedilla => 'ȩ',
            LatinExtendedB::LatinCapitalLetterOWithDiaeresisAndMacron => 'Ȫ',
            LatinExtendedB::LatinSmallLetterOWithDiaeresisAndMacron => 'ȫ',
            LatinExtendedB::LatinCapitalLetterOWithTildeAndMacron => 'Ȭ',
            LatinExtendedB::LatinSmallLetterOWithTildeAndMacron => 'ȭ',
            LatinExtendedB::LatinCapitalLetterOWithDotAbove => 'Ȯ',
            LatinExtendedB::LatinSmallLetterOWithDotAbove => 'ȯ',
            LatinExtendedB::LatinCapitalLetterOWithDotAboveAndMacron => 'Ȱ',
            LatinExtendedB::LatinSmallLetterOWithDotAboveAndMacron => 'ȱ',
            LatinExtendedB::LatinCapitalLetterYWithMacron => 'Ȳ',
            LatinExtendedB::LatinSmallLetterYWithMacron => 'ȳ',
            LatinExtendedB::LatinSmallLetterLWithCurl => 'ȴ',
            LatinExtendedB::LatinSmallLetterNWithCurl => 'ȵ',
            LatinExtendedB::LatinSmallLetterTWithCurl => 'ȶ',
            LatinExtendedB::LatinSmallLetterDotlessJ => 'ȷ',
            LatinExtendedB::LatinSmallLetterDbDigraph => 'ȸ',
            LatinExtendedB::LatinSmallLetterQpDigraph => 'ȹ',
            LatinExtendedB::LatinCapitalLetterAWithStroke => 'Ⱥ',
            LatinExtendedB::LatinCapitalLetterCWithStroke => 'Ȼ',
            LatinExtendedB::LatinSmallLetterCWithStroke => 'ȼ',
            LatinExtendedB::LatinCapitalLetterLWithBar => 'Ƚ',
            LatinExtendedB::LatinCapitalLetterTWithDiagonalStroke => 'Ⱦ',
            LatinExtendedB::LatinSmallLetterSWithSwashTail => 'ȿ',
            LatinExtendedB::LatinSmallLetterZWithSwashTail => 'ɀ',
            LatinExtendedB::LatinCapitalLetterGlottalStop => 'Ɂ',
            LatinExtendedB::LatinSmallLetterGlottalStop => 'ɂ',
            LatinExtendedB::LatinCapitalLetterBWithStroke => 'Ƀ',
            LatinExtendedB::LatinCapitalLetterUBar => 'Ʉ',
            LatinExtendedB::LatinCapitalLetterTurnedV => 'Ʌ',
            LatinExtendedB::LatinCapitalLetterEWithStroke => 'Ɇ',
            LatinExtendedB::LatinSmallLetterEWithStroke => 'ɇ',
            LatinExtendedB::LatinCapitalLetterJWithStroke => 'Ɉ',
            LatinExtendedB::LatinSmallLetterJWithStroke => 'ɉ',
            LatinExtendedB::LatinCapitalLetterSmallQWithHookTail => 'Ɋ',
            LatinExtendedB::LatinSmallLetterQWithHookTail => 'ɋ',
            LatinExtendedB::LatinCapitalLetterRWithStroke => 'Ɍ',
            LatinExtendedB::LatinSmallLetterRWithStroke => 'ɍ',
            LatinExtendedB::LatinCapitalLetterYWithStroke => 'Ɏ',
        }
    }
}

impl std::convert::TryFrom<char> for LatinExtendedB {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ƀ' => Ok(LatinExtendedB::LatinSmallLetterBWithStroke),
            'Ɓ' => Ok(LatinExtendedB::LatinCapitalLetterBWithHook),
            'Ƃ' => Ok(LatinExtendedB::LatinCapitalLetterBWithTopbar),
            'ƃ' => Ok(LatinExtendedB::LatinSmallLetterBWithTopbar),
            'Ƅ' => Ok(LatinExtendedB::LatinCapitalLetterToneSix),
            'ƅ' => Ok(LatinExtendedB::LatinSmallLetterToneSix),
            'Ɔ' => Ok(LatinExtendedB::LatinCapitalLetterOpenO),
            'Ƈ' => Ok(LatinExtendedB::LatinCapitalLetterCWithHook),
            'ƈ' => Ok(LatinExtendedB::LatinSmallLetterCWithHook),
            'Ɖ' => Ok(LatinExtendedB::LatinCapitalLetterAfricanD),
            'Ɗ' => Ok(LatinExtendedB::LatinCapitalLetterDWithHook),
            'Ƌ' => Ok(LatinExtendedB::LatinCapitalLetterDWithTopbar),
            'ƌ' => Ok(LatinExtendedB::LatinSmallLetterDWithTopbar),
            'ƍ' => Ok(LatinExtendedB::LatinSmallLetterTurnedDelta),
            'Ǝ' => Ok(LatinExtendedB::LatinCapitalLetterReversedE),
            'Ə' => Ok(LatinExtendedB::LatinCapitalLetterSchwa),
            'Ɛ' => Ok(LatinExtendedB::LatinCapitalLetterOpenE),
            'Ƒ' => Ok(LatinExtendedB::LatinCapitalLetterFWithHook),
            'ƒ' => Ok(LatinExtendedB::LatinSmallLetterFWithHook),
            'Ɠ' => Ok(LatinExtendedB::LatinCapitalLetterGWithHook),
            'Ɣ' => Ok(LatinExtendedB::LatinCapitalLetterGamma),
            'ƕ' => Ok(LatinExtendedB::LatinSmallLetterHv),
            'Ɩ' => Ok(LatinExtendedB::LatinCapitalLetterIota),
            'Ɨ' => Ok(LatinExtendedB::LatinCapitalLetterIWithStroke),
            'Ƙ' => Ok(LatinExtendedB::LatinCapitalLetterKWithHook),
            'ƙ' => Ok(LatinExtendedB::LatinSmallLetterKWithHook),
            'ƚ' => Ok(LatinExtendedB::LatinSmallLetterLWithBar),
            'ƛ' => Ok(LatinExtendedB::LatinSmallLetterLambdaWithStroke),
            'Ɯ' => Ok(LatinExtendedB::LatinCapitalLetterTurnedM),
            'Ɲ' => Ok(LatinExtendedB::LatinCapitalLetterNWithLeftHook),
            'ƞ' => Ok(LatinExtendedB::LatinSmallLetterNWithLongRightLeg),
            'Ɵ' => Ok(LatinExtendedB::LatinCapitalLetterOWithMiddleTilde),
            'Ơ' => Ok(LatinExtendedB::LatinCapitalLetterOWithHorn),
            'ơ' => Ok(LatinExtendedB::LatinSmallLetterOWithHorn),
            'Ƣ' => Ok(LatinExtendedB::LatinCapitalLetterOi),
            'ƣ' => Ok(LatinExtendedB::LatinSmallLetterOi),
            'Ƥ' => Ok(LatinExtendedB::LatinCapitalLetterPWithHook),
            'ƥ' => Ok(LatinExtendedB::LatinSmallLetterPWithHook),
            'Ʀ' => Ok(LatinExtendedB::LatinLetterYr),
            'Ƨ' => Ok(LatinExtendedB::LatinCapitalLetterToneTwo),
            'ƨ' => Ok(LatinExtendedB::LatinSmallLetterToneTwo),
            'Ʃ' => Ok(LatinExtendedB::LatinCapitalLetterEsh),
            'ƪ' => Ok(LatinExtendedB::LatinLetterReversedEshLoop),
            'ƫ' => Ok(LatinExtendedB::LatinSmallLetterTWithPalatalHook),
            'Ƭ' => Ok(LatinExtendedB::LatinCapitalLetterTWithHook),
            'ƭ' => Ok(LatinExtendedB::LatinSmallLetterTWithHook),
            'Ʈ' => Ok(LatinExtendedB::LatinCapitalLetterTWithRetroflexHook),
            'Ư' => Ok(LatinExtendedB::LatinCapitalLetterUWithHorn),
            'ư' => Ok(LatinExtendedB::LatinSmallLetterUWithHorn),
            'Ʊ' => Ok(LatinExtendedB::LatinCapitalLetterUpsilon),
            'Ʋ' => Ok(LatinExtendedB::LatinCapitalLetterVWithHook),
            'Ƴ' => Ok(LatinExtendedB::LatinCapitalLetterYWithHook),
            'ƴ' => Ok(LatinExtendedB::LatinSmallLetterYWithHook),
            'Ƶ' => Ok(LatinExtendedB::LatinCapitalLetterZWithStroke),
            'ƶ' => Ok(LatinExtendedB::LatinSmallLetterZWithStroke),
            'Ʒ' => Ok(LatinExtendedB::LatinCapitalLetterEzh),
            'Ƹ' => Ok(LatinExtendedB::LatinCapitalLetterEzhReversed),
            'ƹ' => Ok(LatinExtendedB::LatinSmallLetterEzhReversed),
            'ƺ' => Ok(LatinExtendedB::LatinSmallLetterEzhWithTail),
            'ƻ' => Ok(LatinExtendedB::LatinLetterTwoWithStroke),
            'Ƽ' => Ok(LatinExtendedB::LatinCapitalLetterToneFive),
            'ƽ' => Ok(LatinExtendedB::LatinSmallLetterToneFive),
            'ƾ' => Ok(LatinExtendedB::LatinLetterInvertedGlottalStopWithStroke),
            'ƿ' => Ok(LatinExtendedB::LatinLetterWynn),
            'ǀ' => Ok(LatinExtendedB::LatinLetterDentalClick),
            'ǁ' => Ok(LatinExtendedB::LatinLetterLateralClick),
            'ǂ' => Ok(LatinExtendedB::LatinLetterAlveolarClick),
            'ǃ' => Ok(LatinExtendedB::LatinLetterRetroflexClick),
            'Ǆ' => Ok(LatinExtendedB::LatinCapitalLetterDzWithCaron),
            'ǅ' => Ok(LatinExtendedB::LatinCapitalLetterDWithSmallLetterZWithCaron),
            'ǆ' => Ok(LatinExtendedB::LatinSmallLetterDzWithCaron),
            'Ǉ' => Ok(LatinExtendedB::LatinCapitalLetterLj),
            'ǈ' => Ok(LatinExtendedB::LatinCapitalLetterLWithSmallLetterJ),
            'ǉ' => Ok(LatinExtendedB::LatinSmallLetterLj),
            'Ǌ' => Ok(LatinExtendedB::LatinCapitalLetterNj),
            'ǋ' => Ok(LatinExtendedB::LatinCapitalLetterNWithSmallLetterJ),
            'ǌ' => Ok(LatinExtendedB::LatinSmallLetterNj),
            'Ǎ' => Ok(LatinExtendedB::LatinCapitalLetterAWithCaron),
            'ǎ' => Ok(LatinExtendedB::LatinSmallLetterAWithCaron),
            'Ǐ' => Ok(LatinExtendedB::LatinCapitalLetterIWithCaron),
            'ǐ' => Ok(LatinExtendedB::LatinSmallLetterIWithCaron),
            'Ǒ' => Ok(LatinExtendedB::LatinCapitalLetterOWithCaron),
            'ǒ' => Ok(LatinExtendedB::LatinSmallLetterOWithCaron),
            'Ǔ' => Ok(LatinExtendedB::LatinCapitalLetterUWithCaron),
            'ǔ' => Ok(LatinExtendedB::LatinSmallLetterUWithCaron),
            'Ǖ' => Ok(LatinExtendedB::LatinCapitalLetterUWithDiaeresisAndMacron),
            'ǖ' => Ok(LatinExtendedB::LatinSmallLetterUWithDiaeresisAndMacron),
            'Ǘ' => Ok(LatinExtendedB::LatinCapitalLetterUWithDiaeresisAndAcute),
            'ǘ' => Ok(LatinExtendedB::LatinSmallLetterUWithDiaeresisAndAcute),
            'Ǚ' => Ok(LatinExtendedB::LatinCapitalLetterUWithDiaeresisAndCaron),
            'ǚ' => Ok(LatinExtendedB::LatinSmallLetterUWithDiaeresisAndCaron),
            'Ǜ' => Ok(LatinExtendedB::LatinCapitalLetterUWithDiaeresisAndGrave),
            'ǜ' => Ok(LatinExtendedB::LatinSmallLetterUWithDiaeresisAndGrave),
            'ǝ' => Ok(LatinExtendedB::LatinSmallLetterTurnedE),
            'Ǟ' => Ok(LatinExtendedB::LatinCapitalLetterAWithDiaeresisAndMacron),
            'ǟ' => Ok(LatinExtendedB::LatinSmallLetterAWithDiaeresisAndMacron),
            'Ǡ' => Ok(LatinExtendedB::LatinCapitalLetterAWithDotAboveAndMacron),
            'ǡ' => Ok(LatinExtendedB::LatinSmallLetterAWithDotAboveAndMacron),
            'Ǣ' => Ok(LatinExtendedB::LatinCapitalLetterAeWithMacron),
            'ǣ' => Ok(LatinExtendedB::LatinSmallLetterAeWithMacron),
            'Ǥ' => Ok(LatinExtendedB::LatinCapitalLetterGWithStroke),
            'ǥ' => Ok(LatinExtendedB::LatinSmallLetterGWithStroke),
            'Ǧ' => Ok(LatinExtendedB::LatinCapitalLetterGWithCaron),
            'ǧ' => Ok(LatinExtendedB::LatinSmallLetterGWithCaron),
            'Ǩ' => Ok(LatinExtendedB::LatinCapitalLetterKWithCaron),
            'ǩ' => Ok(LatinExtendedB::LatinSmallLetterKWithCaron),
            'Ǫ' => Ok(LatinExtendedB::LatinCapitalLetterOWithOgonek),
            'ǫ' => Ok(LatinExtendedB::LatinSmallLetterOWithOgonek),
            'Ǭ' => Ok(LatinExtendedB::LatinCapitalLetterOWithOgonekAndMacron),
            'ǭ' => Ok(LatinExtendedB::LatinSmallLetterOWithOgonekAndMacron),
            'Ǯ' => Ok(LatinExtendedB::LatinCapitalLetterEzhWithCaron),
            'ǯ' => Ok(LatinExtendedB::LatinSmallLetterEzhWithCaron),
            'ǰ' => Ok(LatinExtendedB::LatinSmallLetterJWithCaron),
            'Ǳ' => Ok(LatinExtendedB::LatinCapitalLetterDz),
            'ǲ' => Ok(LatinExtendedB::LatinCapitalLetterDWithSmallLetterZ),
            'ǳ' => Ok(LatinExtendedB::LatinSmallLetterDz),
            'Ǵ' => Ok(LatinExtendedB::LatinCapitalLetterGWithAcute),
            'ǵ' => Ok(LatinExtendedB::LatinSmallLetterGWithAcute),
            'Ƕ' => Ok(LatinExtendedB::LatinCapitalLetterHwair),
            'Ƿ' => Ok(LatinExtendedB::LatinCapitalLetterWynn),
            'Ǹ' => Ok(LatinExtendedB::LatinCapitalLetterNWithGrave),
            'ǹ' => Ok(LatinExtendedB::LatinSmallLetterNWithGrave),
            'Ǻ' => Ok(LatinExtendedB::LatinCapitalLetterAWithRingAboveAndAcute),
            'ǻ' => Ok(LatinExtendedB::LatinSmallLetterAWithRingAboveAndAcute),
            'Ǽ' => Ok(LatinExtendedB::LatinCapitalLetterAeWithAcute),
            'ǽ' => Ok(LatinExtendedB::LatinSmallLetterAeWithAcute),
            'Ǿ' => Ok(LatinExtendedB::LatinCapitalLetterOWithStrokeAndAcute),
            'ǿ' => Ok(LatinExtendedB::LatinSmallLetterOWithStrokeAndAcute),
            'Ȁ' => Ok(LatinExtendedB::LatinCapitalLetterAWithDoubleGrave),
            'ȁ' => Ok(LatinExtendedB::LatinSmallLetterAWithDoubleGrave),
            'Ȃ' => Ok(LatinExtendedB::LatinCapitalLetterAWithInvertedBreve),
            'ȃ' => Ok(LatinExtendedB::LatinSmallLetterAWithInvertedBreve),
            'Ȅ' => Ok(LatinExtendedB::LatinCapitalLetterEWithDoubleGrave),
            'ȅ' => Ok(LatinExtendedB::LatinSmallLetterEWithDoubleGrave),
            'Ȇ' => Ok(LatinExtendedB::LatinCapitalLetterEWithInvertedBreve),
            'ȇ' => Ok(LatinExtendedB::LatinSmallLetterEWithInvertedBreve),
            'Ȉ' => Ok(LatinExtendedB::LatinCapitalLetterIWithDoubleGrave),
            'ȉ' => Ok(LatinExtendedB::LatinSmallLetterIWithDoubleGrave),
            'Ȋ' => Ok(LatinExtendedB::LatinCapitalLetterIWithInvertedBreve),
            'ȋ' => Ok(LatinExtendedB::LatinSmallLetterIWithInvertedBreve),
            'Ȍ' => Ok(LatinExtendedB::LatinCapitalLetterOWithDoubleGrave),
            'ȍ' => Ok(LatinExtendedB::LatinSmallLetterOWithDoubleGrave),
            'Ȏ' => Ok(LatinExtendedB::LatinCapitalLetterOWithInvertedBreve),
            'ȏ' => Ok(LatinExtendedB::LatinSmallLetterOWithInvertedBreve),
            'Ȑ' => Ok(LatinExtendedB::LatinCapitalLetterRWithDoubleGrave),
            'ȑ' => Ok(LatinExtendedB::LatinSmallLetterRWithDoubleGrave),
            'Ȓ' => Ok(LatinExtendedB::LatinCapitalLetterRWithInvertedBreve),
            'ȓ' => Ok(LatinExtendedB::LatinSmallLetterRWithInvertedBreve),
            'Ȕ' => Ok(LatinExtendedB::LatinCapitalLetterUWithDoubleGrave),
            'ȕ' => Ok(LatinExtendedB::LatinSmallLetterUWithDoubleGrave),
            'Ȗ' => Ok(LatinExtendedB::LatinCapitalLetterUWithInvertedBreve),
            'ȗ' => Ok(LatinExtendedB::LatinSmallLetterUWithInvertedBreve),
            'Ș' => Ok(LatinExtendedB::LatinCapitalLetterSWithCommaBelow),
            'ș' => Ok(LatinExtendedB::LatinSmallLetterSWithCommaBelow),
            'Ț' => Ok(LatinExtendedB::LatinCapitalLetterTWithCommaBelow),
            'ț' => Ok(LatinExtendedB::LatinSmallLetterTWithCommaBelow),
            'Ȝ' => Ok(LatinExtendedB::LatinCapitalLetterYogh),
            'ȝ' => Ok(LatinExtendedB::LatinSmallLetterYogh),
            'Ȟ' => Ok(LatinExtendedB::LatinCapitalLetterHWithCaron),
            'ȟ' => Ok(LatinExtendedB::LatinSmallLetterHWithCaron),
            'Ƞ' => Ok(LatinExtendedB::LatinCapitalLetterNWithLongRightLeg),
            'ȡ' => Ok(LatinExtendedB::LatinSmallLetterDWithCurl),
            'Ȣ' => Ok(LatinExtendedB::LatinCapitalLetterOu),
            'ȣ' => Ok(LatinExtendedB::LatinSmallLetterOu),
            'Ȥ' => Ok(LatinExtendedB::LatinCapitalLetterZWithHook),
            'ȥ' => Ok(LatinExtendedB::LatinSmallLetterZWithHook),
            'Ȧ' => Ok(LatinExtendedB::LatinCapitalLetterAWithDotAbove),
            'ȧ' => Ok(LatinExtendedB::LatinSmallLetterAWithDotAbove),
            'Ȩ' => Ok(LatinExtendedB::LatinCapitalLetterEWithCedilla),
            'ȩ' => Ok(LatinExtendedB::LatinSmallLetterEWithCedilla),
            'Ȫ' => Ok(LatinExtendedB::LatinCapitalLetterOWithDiaeresisAndMacron),
            'ȫ' => Ok(LatinExtendedB::LatinSmallLetterOWithDiaeresisAndMacron),
            'Ȭ' => Ok(LatinExtendedB::LatinCapitalLetterOWithTildeAndMacron),
            'ȭ' => Ok(LatinExtendedB::LatinSmallLetterOWithTildeAndMacron),
            'Ȯ' => Ok(LatinExtendedB::LatinCapitalLetterOWithDotAbove),
            'ȯ' => Ok(LatinExtendedB::LatinSmallLetterOWithDotAbove),
            'Ȱ' => Ok(LatinExtendedB::LatinCapitalLetterOWithDotAboveAndMacron),
            'ȱ' => Ok(LatinExtendedB::LatinSmallLetterOWithDotAboveAndMacron),
            'Ȳ' => Ok(LatinExtendedB::LatinCapitalLetterYWithMacron),
            'ȳ' => Ok(LatinExtendedB::LatinSmallLetterYWithMacron),
            'ȴ' => Ok(LatinExtendedB::LatinSmallLetterLWithCurl),
            'ȵ' => Ok(LatinExtendedB::LatinSmallLetterNWithCurl),
            'ȶ' => Ok(LatinExtendedB::LatinSmallLetterTWithCurl),
            'ȷ' => Ok(LatinExtendedB::LatinSmallLetterDotlessJ),
            'ȸ' => Ok(LatinExtendedB::LatinSmallLetterDbDigraph),
            'ȹ' => Ok(LatinExtendedB::LatinSmallLetterQpDigraph),
            'Ⱥ' => Ok(LatinExtendedB::LatinCapitalLetterAWithStroke),
            'Ȼ' => Ok(LatinExtendedB::LatinCapitalLetterCWithStroke),
            'ȼ' => Ok(LatinExtendedB::LatinSmallLetterCWithStroke),
            'Ƚ' => Ok(LatinExtendedB::LatinCapitalLetterLWithBar),
            'Ⱦ' => Ok(LatinExtendedB::LatinCapitalLetterTWithDiagonalStroke),
            'ȿ' => Ok(LatinExtendedB::LatinSmallLetterSWithSwashTail),
            'ɀ' => Ok(LatinExtendedB::LatinSmallLetterZWithSwashTail),
            'Ɂ' => Ok(LatinExtendedB::LatinCapitalLetterGlottalStop),
            'ɂ' => Ok(LatinExtendedB::LatinSmallLetterGlottalStop),
            'Ƀ' => Ok(LatinExtendedB::LatinCapitalLetterBWithStroke),
            'Ʉ' => Ok(LatinExtendedB::LatinCapitalLetterUBar),
            'Ʌ' => Ok(LatinExtendedB::LatinCapitalLetterTurnedV),
            'Ɇ' => Ok(LatinExtendedB::LatinCapitalLetterEWithStroke),
            'ɇ' => Ok(LatinExtendedB::LatinSmallLetterEWithStroke),
            'Ɉ' => Ok(LatinExtendedB::LatinCapitalLetterJWithStroke),
            'ɉ' => Ok(LatinExtendedB::LatinSmallLetterJWithStroke),
            'Ɋ' => Ok(LatinExtendedB::LatinCapitalLetterSmallQWithHookTail),
            'ɋ' => Ok(LatinExtendedB::LatinSmallLetterQWithHookTail),
            'Ɍ' => Ok(LatinExtendedB::LatinCapitalLetterRWithStroke),
            'ɍ' => Ok(LatinExtendedB::LatinSmallLetterRWithStroke),
            'Ɏ' => Ok(LatinExtendedB::LatinCapitalLetterYWithStroke),
            _ => Err(()),
        }
    }
}

impl Into<u32> for LatinExtendedB {
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

impl std::convert::TryFrom<u32> for LatinExtendedB {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for LatinExtendedB {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl LatinExtendedB {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        LatinExtendedB::LatinSmallLetterBWithStroke
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("LatinExtendedB{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
