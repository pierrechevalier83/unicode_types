
/// An enum to represent all characters in the EnclosedAlphanumerics block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum EnclosedAlphanumerics {
    /// \u{2460}: '①'
    CircledDigitOne,
    /// \u{2461}: '②'
    CircledDigitTwo,
    /// \u{2462}: '③'
    CircledDigitThree,
    /// \u{2463}: '④'
    CircledDigitFour,
    /// \u{2464}: '⑤'
    CircledDigitFive,
    /// \u{2465}: '⑥'
    CircledDigitSix,
    /// \u{2466}: '⑦'
    CircledDigitSeven,
    /// \u{2467}: '⑧'
    CircledDigitEight,
    /// \u{2468}: '⑨'
    CircledDigitNine,
    /// \u{2469}: '⑩'
    CircledNumberTen,
    /// \u{246a}: '⑪'
    CircledNumberEleven,
    /// \u{246b}: '⑫'
    CircledNumberTwelve,
    /// \u{246c}: '⑬'
    CircledNumberThirteen,
    /// \u{246d}: '⑭'
    CircledNumberFourteen,
    /// \u{246e}: '⑮'
    CircledNumberFifteen,
    /// \u{246f}: '⑯'
    CircledNumberSixteen,
    /// \u{2470}: '⑰'
    CircledNumberSeventeen,
    /// \u{2471}: '⑱'
    CircledNumberEighteen,
    /// \u{2472}: '⑲'
    CircledNumberNineteen,
    /// \u{2473}: '⑳'
    CircledNumberTwenty,
    /// \u{2474}: '⑴'
    ParenthesizedDigitOne,
    /// \u{2475}: '⑵'
    ParenthesizedDigitTwo,
    /// \u{2476}: '⑶'
    ParenthesizedDigitThree,
    /// \u{2477}: '⑷'
    ParenthesizedDigitFour,
    /// \u{2478}: '⑸'
    ParenthesizedDigitFive,
    /// \u{2479}: '⑹'
    ParenthesizedDigitSix,
    /// \u{247a}: '⑺'
    ParenthesizedDigitSeven,
    /// \u{247b}: '⑻'
    ParenthesizedDigitEight,
    /// \u{247c}: '⑼'
    ParenthesizedDigitNine,
    /// \u{247d}: '⑽'
    ParenthesizedNumberTen,
    /// \u{247e}: '⑾'
    ParenthesizedNumberEleven,
    /// \u{247f}: '⑿'
    ParenthesizedNumberTwelve,
    /// \u{2480}: '⒀'
    ParenthesizedNumberThirteen,
    /// \u{2481}: '⒁'
    ParenthesizedNumberFourteen,
    /// \u{2482}: '⒂'
    ParenthesizedNumberFifteen,
    /// \u{2483}: '⒃'
    ParenthesizedNumberSixteen,
    /// \u{2484}: '⒄'
    ParenthesizedNumberSeventeen,
    /// \u{2485}: '⒅'
    ParenthesizedNumberEighteen,
    /// \u{2486}: '⒆'
    ParenthesizedNumberNineteen,
    /// \u{2487}: '⒇'
    ParenthesizedNumberTwenty,
    /// \u{2488}: '⒈'
    DigitOneFullStop,
    /// \u{2489}: '⒉'
    DigitTwoFullStop,
    /// \u{248a}: '⒊'
    DigitThreeFullStop,
    /// \u{248b}: '⒋'
    DigitFourFullStop,
    /// \u{248c}: '⒌'
    DigitFiveFullStop,
    /// \u{248d}: '⒍'
    DigitSixFullStop,
    /// \u{248e}: '⒎'
    DigitSevenFullStop,
    /// \u{248f}: '⒏'
    DigitEightFullStop,
    /// \u{2490}: '⒐'
    DigitNineFullStop,
    /// \u{2491}: '⒑'
    NumberTenFullStop,
    /// \u{2492}: '⒒'
    NumberElevenFullStop,
    /// \u{2493}: '⒓'
    NumberTwelveFullStop,
    /// \u{2494}: '⒔'
    NumberThirteenFullStop,
    /// \u{2495}: '⒕'
    NumberFourteenFullStop,
    /// \u{2496}: '⒖'
    NumberFifteenFullStop,
    /// \u{2497}: '⒗'
    NumberSixteenFullStop,
    /// \u{2498}: '⒘'
    NumberSeventeenFullStop,
    /// \u{2499}: '⒙'
    NumberEighteenFullStop,
    /// \u{249a}: '⒚'
    NumberNineteenFullStop,
    /// \u{249b}: '⒛'
    NumberTwentyFullStop,
    /// \u{249c}: '⒜'
    ParenthesizedLatinSmallLetterA,
    /// \u{249d}: '⒝'
    ParenthesizedLatinSmallLetterB,
    /// \u{249e}: '⒞'
    ParenthesizedLatinSmallLetterC,
    /// \u{249f}: '⒟'
    ParenthesizedLatinSmallLetterD,
    /// \u{24a0}: '⒠'
    ParenthesizedLatinSmallLetterE,
    /// \u{24a1}: '⒡'
    ParenthesizedLatinSmallLetterF,
    /// \u{24a2}: '⒢'
    ParenthesizedLatinSmallLetterG,
    /// \u{24a3}: '⒣'
    ParenthesizedLatinSmallLetterH,
    /// \u{24a4}: '⒤'
    ParenthesizedLatinSmallLetterI,
    /// \u{24a5}: '⒥'
    ParenthesizedLatinSmallLetterJ,
    /// \u{24a6}: '⒦'
    ParenthesizedLatinSmallLetterK,
    /// \u{24a7}: '⒧'
    ParenthesizedLatinSmallLetterL,
    /// \u{24a8}: '⒨'
    ParenthesizedLatinSmallLetterM,
    /// \u{24a9}: '⒩'
    ParenthesizedLatinSmallLetterN,
    /// \u{24aa}: '⒪'
    ParenthesizedLatinSmallLetterO,
    /// \u{24ab}: '⒫'
    ParenthesizedLatinSmallLetterP,
    /// \u{24ac}: '⒬'
    ParenthesizedLatinSmallLetterQ,
    /// \u{24ad}: '⒭'
    ParenthesizedLatinSmallLetterR,
    /// \u{24ae}: '⒮'
    ParenthesizedLatinSmallLetterS,
    /// \u{24af}: '⒯'
    ParenthesizedLatinSmallLetterT,
    /// \u{24b0}: '⒰'
    ParenthesizedLatinSmallLetterU,
    /// \u{24b1}: '⒱'
    ParenthesizedLatinSmallLetterV,
    /// \u{24b2}: '⒲'
    ParenthesizedLatinSmallLetterW,
    /// \u{24b3}: '⒳'
    ParenthesizedLatinSmallLetterX,
    /// \u{24b4}: '⒴'
    ParenthesizedLatinSmallLetterY,
    /// \u{24b5}: '⒵'
    ParenthesizedLatinSmallLetterZ,
    /// \u{24b6}: 'Ⓐ'
    CircledLatinCapitalLetterA,
    /// \u{24b7}: 'Ⓑ'
    CircledLatinCapitalLetterB,
    /// \u{24b8}: 'Ⓒ'
    CircledLatinCapitalLetterC,
    /// \u{24b9}: 'Ⓓ'
    CircledLatinCapitalLetterD,
    /// \u{24ba}: 'Ⓔ'
    CircledLatinCapitalLetterE,
    /// \u{24bb}: 'Ⓕ'
    CircledLatinCapitalLetterF,
    /// \u{24bc}: 'Ⓖ'
    CircledLatinCapitalLetterG,
    /// \u{24bd}: 'Ⓗ'
    CircledLatinCapitalLetterH,
    /// \u{24be}: 'Ⓘ'
    CircledLatinCapitalLetterI,
    /// \u{24bf}: 'Ⓙ'
    CircledLatinCapitalLetterJ,
    /// \u{24c0}: 'Ⓚ'
    CircledLatinCapitalLetterK,
    /// \u{24c1}: 'Ⓛ'
    CircledLatinCapitalLetterL,
    /// \u{24c2}: 'Ⓜ'
    CircledLatinCapitalLetterM,
    /// \u{24c3}: 'Ⓝ'
    CircledLatinCapitalLetterN,
    /// \u{24c4}: 'Ⓞ'
    CircledLatinCapitalLetterO,
    /// \u{24c5}: 'Ⓟ'
    CircledLatinCapitalLetterP,
    /// \u{24c6}: 'Ⓠ'
    CircledLatinCapitalLetterQ,
    /// \u{24c7}: 'Ⓡ'
    CircledLatinCapitalLetterR,
    /// \u{24c8}: 'Ⓢ'
    CircledLatinCapitalLetterS,
    /// \u{24c9}: 'Ⓣ'
    CircledLatinCapitalLetterT,
    /// \u{24ca}: 'Ⓤ'
    CircledLatinCapitalLetterU,
    /// \u{24cb}: 'Ⓥ'
    CircledLatinCapitalLetterV,
    /// \u{24cc}: 'Ⓦ'
    CircledLatinCapitalLetterW,
    /// \u{24cd}: 'Ⓧ'
    CircledLatinCapitalLetterX,
    /// \u{24ce}: 'Ⓨ'
    CircledLatinCapitalLetterY,
    /// \u{24cf}: 'Ⓩ'
    CircledLatinCapitalLetterZ,
    /// \u{24d0}: 'ⓐ'
    CircledLatinSmallLetterA,
    /// \u{24d1}: 'ⓑ'
    CircledLatinSmallLetterB,
    /// \u{24d2}: 'ⓒ'
    CircledLatinSmallLetterC,
    /// \u{24d3}: 'ⓓ'
    CircledLatinSmallLetterD,
    /// \u{24d4}: 'ⓔ'
    CircledLatinSmallLetterE,
    /// \u{24d5}: 'ⓕ'
    CircledLatinSmallLetterF,
    /// \u{24d6}: 'ⓖ'
    CircledLatinSmallLetterG,
    /// \u{24d7}: 'ⓗ'
    CircledLatinSmallLetterH,
    /// \u{24d8}: 'ⓘ'
    CircledLatinSmallLetterI,
    /// \u{24d9}: 'ⓙ'
    CircledLatinSmallLetterJ,
    /// \u{24da}: 'ⓚ'
    CircledLatinSmallLetterK,
    /// \u{24db}: 'ⓛ'
    CircledLatinSmallLetterL,
    /// \u{24dc}: 'ⓜ'
    CircledLatinSmallLetterM,
    /// \u{24dd}: 'ⓝ'
    CircledLatinSmallLetterN,
    /// \u{24de}: 'ⓞ'
    CircledLatinSmallLetterO,
    /// \u{24df}: 'ⓟ'
    CircledLatinSmallLetterP,
    /// \u{24e0}: 'ⓠ'
    CircledLatinSmallLetterQ,
    /// \u{24e1}: 'ⓡ'
    CircledLatinSmallLetterR,
    /// \u{24e2}: 'ⓢ'
    CircledLatinSmallLetterS,
    /// \u{24e3}: 'ⓣ'
    CircledLatinSmallLetterT,
    /// \u{24e4}: 'ⓤ'
    CircledLatinSmallLetterU,
    /// \u{24e5}: 'ⓥ'
    CircledLatinSmallLetterV,
    /// \u{24e6}: 'ⓦ'
    CircledLatinSmallLetterW,
    /// \u{24e7}: 'ⓧ'
    CircledLatinSmallLetterX,
    /// \u{24e8}: 'ⓨ'
    CircledLatinSmallLetterY,
    /// \u{24e9}: 'ⓩ'
    CircledLatinSmallLetterZ,
    /// \u{24ea}: '⓪'
    CircledDigitZero,
    /// \u{24eb}: '⓫'
    NegativeCircledNumberEleven,
    /// \u{24ec}: '⓬'
    NegativeCircledNumberTwelve,
    /// \u{24ed}: '⓭'
    NegativeCircledNumberThirteen,
    /// \u{24ee}: '⓮'
    NegativeCircledNumberFourteen,
    /// \u{24ef}: '⓯'
    NegativeCircledNumberFifteen,
    /// \u{24f0}: '⓰'
    NegativeCircledNumberSixteen,
    /// \u{24f1}: '⓱'
    NegativeCircledNumberSeventeen,
    /// \u{24f2}: '⓲'
    NegativeCircledNumberEighteen,
    /// \u{24f3}: '⓳'
    NegativeCircledNumberNineteen,
    /// \u{24f4}: '⓴'
    NegativeCircledNumberTwenty,
    /// \u{24f5}: '⓵'
    DoubleCircledDigitOne,
    /// \u{24f6}: '⓶'
    DoubleCircledDigitTwo,
    /// \u{24f7}: '⓷'
    DoubleCircledDigitThree,
    /// \u{24f8}: '⓸'
    DoubleCircledDigitFour,
    /// \u{24f9}: '⓹'
    DoubleCircledDigitFive,
    /// \u{24fa}: '⓺'
    DoubleCircledDigitSix,
    /// \u{24fb}: '⓻'
    DoubleCircledDigitSeven,
    /// \u{24fc}: '⓼'
    DoubleCircledDigitEight,
    /// \u{24fd}: '⓽'
    DoubleCircledDigitNine,
    /// \u{24fe}: '⓾'
    DoubleCircledNumberTen,
}

impl Into<char> for EnclosedAlphanumerics {
    fn into(self) -> char {
        match self {
            EnclosedAlphanumerics::CircledDigitOne => '①',
            EnclosedAlphanumerics::CircledDigitTwo => '②',
            EnclosedAlphanumerics::CircledDigitThree => '③',
            EnclosedAlphanumerics::CircledDigitFour => '④',
            EnclosedAlphanumerics::CircledDigitFive => '⑤',
            EnclosedAlphanumerics::CircledDigitSix => '⑥',
            EnclosedAlphanumerics::CircledDigitSeven => '⑦',
            EnclosedAlphanumerics::CircledDigitEight => '⑧',
            EnclosedAlphanumerics::CircledDigitNine => '⑨',
            EnclosedAlphanumerics::CircledNumberTen => '⑩',
            EnclosedAlphanumerics::CircledNumberEleven => '⑪',
            EnclosedAlphanumerics::CircledNumberTwelve => '⑫',
            EnclosedAlphanumerics::CircledNumberThirteen => '⑬',
            EnclosedAlphanumerics::CircledNumberFourteen => '⑭',
            EnclosedAlphanumerics::CircledNumberFifteen => '⑮',
            EnclosedAlphanumerics::CircledNumberSixteen => '⑯',
            EnclosedAlphanumerics::CircledNumberSeventeen => '⑰',
            EnclosedAlphanumerics::CircledNumberEighteen => '⑱',
            EnclosedAlphanumerics::CircledNumberNineteen => '⑲',
            EnclosedAlphanumerics::CircledNumberTwenty => '⑳',
            EnclosedAlphanumerics::ParenthesizedDigitOne => '⑴',
            EnclosedAlphanumerics::ParenthesizedDigitTwo => '⑵',
            EnclosedAlphanumerics::ParenthesizedDigitThree => '⑶',
            EnclosedAlphanumerics::ParenthesizedDigitFour => '⑷',
            EnclosedAlphanumerics::ParenthesizedDigitFive => '⑸',
            EnclosedAlphanumerics::ParenthesizedDigitSix => '⑹',
            EnclosedAlphanumerics::ParenthesizedDigitSeven => '⑺',
            EnclosedAlphanumerics::ParenthesizedDigitEight => '⑻',
            EnclosedAlphanumerics::ParenthesizedDigitNine => '⑼',
            EnclosedAlphanumerics::ParenthesizedNumberTen => '⑽',
            EnclosedAlphanumerics::ParenthesizedNumberEleven => '⑾',
            EnclosedAlphanumerics::ParenthesizedNumberTwelve => '⑿',
            EnclosedAlphanumerics::ParenthesizedNumberThirteen => '⒀',
            EnclosedAlphanumerics::ParenthesizedNumberFourteen => '⒁',
            EnclosedAlphanumerics::ParenthesizedNumberFifteen => '⒂',
            EnclosedAlphanumerics::ParenthesizedNumberSixteen => '⒃',
            EnclosedAlphanumerics::ParenthesizedNumberSeventeen => '⒄',
            EnclosedAlphanumerics::ParenthesizedNumberEighteen => '⒅',
            EnclosedAlphanumerics::ParenthesizedNumberNineteen => '⒆',
            EnclosedAlphanumerics::ParenthesizedNumberTwenty => '⒇',
            EnclosedAlphanumerics::DigitOneFullStop => '⒈',
            EnclosedAlphanumerics::DigitTwoFullStop => '⒉',
            EnclosedAlphanumerics::DigitThreeFullStop => '⒊',
            EnclosedAlphanumerics::DigitFourFullStop => '⒋',
            EnclosedAlphanumerics::DigitFiveFullStop => '⒌',
            EnclosedAlphanumerics::DigitSixFullStop => '⒍',
            EnclosedAlphanumerics::DigitSevenFullStop => '⒎',
            EnclosedAlphanumerics::DigitEightFullStop => '⒏',
            EnclosedAlphanumerics::DigitNineFullStop => '⒐',
            EnclosedAlphanumerics::NumberTenFullStop => '⒑',
            EnclosedAlphanumerics::NumberElevenFullStop => '⒒',
            EnclosedAlphanumerics::NumberTwelveFullStop => '⒓',
            EnclosedAlphanumerics::NumberThirteenFullStop => '⒔',
            EnclosedAlphanumerics::NumberFourteenFullStop => '⒕',
            EnclosedAlphanumerics::NumberFifteenFullStop => '⒖',
            EnclosedAlphanumerics::NumberSixteenFullStop => '⒗',
            EnclosedAlphanumerics::NumberSeventeenFullStop => '⒘',
            EnclosedAlphanumerics::NumberEighteenFullStop => '⒙',
            EnclosedAlphanumerics::NumberNineteenFullStop => '⒚',
            EnclosedAlphanumerics::NumberTwentyFullStop => '⒛',
            EnclosedAlphanumerics::ParenthesizedLatinSmallLetterA => '⒜',
            EnclosedAlphanumerics::ParenthesizedLatinSmallLetterB => '⒝',
            EnclosedAlphanumerics::ParenthesizedLatinSmallLetterC => '⒞',
            EnclosedAlphanumerics::ParenthesizedLatinSmallLetterD => '⒟',
            EnclosedAlphanumerics::ParenthesizedLatinSmallLetterE => '⒠',
            EnclosedAlphanumerics::ParenthesizedLatinSmallLetterF => '⒡',
            EnclosedAlphanumerics::ParenthesizedLatinSmallLetterG => '⒢',
            EnclosedAlphanumerics::ParenthesizedLatinSmallLetterH => '⒣',
            EnclosedAlphanumerics::ParenthesizedLatinSmallLetterI => '⒤',
            EnclosedAlphanumerics::ParenthesizedLatinSmallLetterJ => '⒥',
            EnclosedAlphanumerics::ParenthesizedLatinSmallLetterK => '⒦',
            EnclosedAlphanumerics::ParenthesizedLatinSmallLetterL => '⒧',
            EnclosedAlphanumerics::ParenthesizedLatinSmallLetterM => '⒨',
            EnclosedAlphanumerics::ParenthesizedLatinSmallLetterN => '⒩',
            EnclosedAlphanumerics::ParenthesizedLatinSmallLetterO => '⒪',
            EnclosedAlphanumerics::ParenthesizedLatinSmallLetterP => '⒫',
            EnclosedAlphanumerics::ParenthesizedLatinSmallLetterQ => '⒬',
            EnclosedAlphanumerics::ParenthesizedLatinSmallLetterR => '⒭',
            EnclosedAlphanumerics::ParenthesizedLatinSmallLetterS => '⒮',
            EnclosedAlphanumerics::ParenthesizedLatinSmallLetterT => '⒯',
            EnclosedAlphanumerics::ParenthesizedLatinSmallLetterU => '⒰',
            EnclosedAlphanumerics::ParenthesizedLatinSmallLetterV => '⒱',
            EnclosedAlphanumerics::ParenthesizedLatinSmallLetterW => '⒲',
            EnclosedAlphanumerics::ParenthesizedLatinSmallLetterX => '⒳',
            EnclosedAlphanumerics::ParenthesizedLatinSmallLetterY => '⒴',
            EnclosedAlphanumerics::ParenthesizedLatinSmallLetterZ => '⒵',
            EnclosedAlphanumerics::CircledLatinCapitalLetterA => 'Ⓐ',
            EnclosedAlphanumerics::CircledLatinCapitalLetterB => 'Ⓑ',
            EnclosedAlphanumerics::CircledLatinCapitalLetterC => 'Ⓒ',
            EnclosedAlphanumerics::CircledLatinCapitalLetterD => 'Ⓓ',
            EnclosedAlphanumerics::CircledLatinCapitalLetterE => 'Ⓔ',
            EnclosedAlphanumerics::CircledLatinCapitalLetterF => 'Ⓕ',
            EnclosedAlphanumerics::CircledLatinCapitalLetterG => 'Ⓖ',
            EnclosedAlphanumerics::CircledLatinCapitalLetterH => 'Ⓗ',
            EnclosedAlphanumerics::CircledLatinCapitalLetterI => 'Ⓘ',
            EnclosedAlphanumerics::CircledLatinCapitalLetterJ => 'Ⓙ',
            EnclosedAlphanumerics::CircledLatinCapitalLetterK => 'Ⓚ',
            EnclosedAlphanumerics::CircledLatinCapitalLetterL => 'Ⓛ',
            EnclosedAlphanumerics::CircledLatinCapitalLetterM => 'Ⓜ',
            EnclosedAlphanumerics::CircledLatinCapitalLetterN => 'Ⓝ',
            EnclosedAlphanumerics::CircledLatinCapitalLetterO => 'Ⓞ',
            EnclosedAlphanumerics::CircledLatinCapitalLetterP => 'Ⓟ',
            EnclosedAlphanumerics::CircledLatinCapitalLetterQ => 'Ⓠ',
            EnclosedAlphanumerics::CircledLatinCapitalLetterR => 'Ⓡ',
            EnclosedAlphanumerics::CircledLatinCapitalLetterS => 'Ⓢ',
            EnclosedAlphanumerics::CircledLatinCapitalLetterT => 'Ⓣ',
            EnclosedAlphanumerics::CircledLatinCapitalLetterU => 'Ⓤ',
            EnclosedAlphanumerics::CircledLatinCapitalLetterV => 'Ⓥ',
            EnclosedAlphanumerics::CircledLatinCapitalLetterW => 'Ⓦ',
            EnclosedAlphanumerics::CircledLatinCapitalLetterX => 'Ⓧ',
            EnclosedAlphanumerics::CircledLatinCapitalLetterY => 'Ⓨ',
            EnclosedAlphanumerics::CircledLatinCapitalLetterZ => 'Ⓩ',
            EnclosedAlphanumerics::CircledLatinSmallLetterA => 'ⓐ',
            EnclosedAlphanumerics::CircledLatinSmallLetterB => 'ⓑ',
            EnclosedAlphanumerics::CircledLatinSmallLetterC => 'ⓒ',
            EnclosedAlphanumerics::CircledLatinSmallLetterD => 'ⓓ',
            EnclosedAlphanumerics::CircledLatinSmallLetterE => 'ⓔ',
            EnclosedAlphanumerics::CircledLatinSmallLetterF => 'ⓕ',
            EnclosedAlphanumerics::CircledLatinSmallLetterG => 'ⓖ',
            EnclosedAlphanumerics::CircledLatinSmallLetterH => 'ⓗ',
            EnclosedAlphanumerics::CircledLatinSmallLetterI => 'ⓘ',
            EnclosedAlphanumerics::CircledLatinSmallLetterJ => 'ⓙ',
            EnclosedAlphanumerics::CircledLatinSmallLetterK => 'ⓚ',
            EnclosedAlphanumerics::CircledLatinSmallLetterL => 'ⓛ',
            EnclosedAlphanumerics::CircledLatinSmallLetterM => 'ⓜ',
            EnclosedAlphanumerics::CircledLatinSmallLetterN => 'ⓝ',
            EnclosedAlphanumerics::CircledLatinSmallLetterO => 'ⓞ',
            EnclosedAlphanumerics::CircledLatinSmallLetterP => 'ⓟ',
            EnclosedAlphanumerics::CircledLatinSmallLetterQ => 'ⓠ',
            EnclosedAlphanumerics::CircledLatinSmallLetterR => 'ⓡ',
            EnclosedAlphanumerics::CircledLatinSmallLetterS => 'ⓢ',
            EnclosedAlphanumerics::CircledLatinSmallLetterT => 'ⓣ',
            EnclosedAlphanumerics::CircledLatinSmallLetterU => 'ⓤ',
            EnclosedAlphanumerics::CircledLatinSmallLetterV => 'ⓥ',
            EnclosedAlphanumerics::CircledLatinSmallLetterW => 'ⓦ',
            EnclosedAlphanumerics::CircledLatinSmallLetterX => 'ⓧ',
            EnclosedAlphanumerics::CircledLatinSmallLetterY => 'ⓨ',
            EnclosedAlphanumerics::CircledLatinSmallLetterZ => 'ⓩ',
            EnclosedAlphanumerics::CircledDigitZero => '⓪',
            EnclosedAlphanumerics::NegativeCircledNumberEleven => '⓫',
            EnclosedAlphanumerics::NegativeCircledNumberTwelve => '⓬',
            EnclosedAlphanumerics::NegativeCircledNumberThirteen => '⓭',
            EnclosedAlphanumerics::NegativeCircledNumberFourteen => '⓮',
            EnclosedAlphanumerics::NegativeCircledNumberFifteen => '⓯',
            EnclosedAlphanumerics::NegativeCircledNumberSixteen => '⓰',
            EnclosedAlphanumerics::NegativeCircledNumberSeventeen => '⓱',
            EnclosedAlphanumerics::NegativeCircledNumberEighteen => '⓲',
            EnclosedAlphanumerics::NegativeCircledNumberNineteen => '⓳',
            EnclosedAlphanumerics::NegativeCircledNumberTwenty => '⓴',
            EnclosedAlphanumerics::DoubleCircledDigitOne => '⓵',
            EnclosedAlphanumerics::DoubleCircledDigitTwo => '⓶',
            EnclosedAlphanumerics::DoubleCircledDigitThree => '⓷',
            EnclosedAlphanumerics::DoubleCircledDigitFour => '⓸',
            EnclosedAlphanumerics::DoubleCircledDigitFive => '⓹',
            EnclosedAlphanumerics::DoubleCircledDigitSix => '⓺',
            EnclosedAlphanumerics::DoubleCircledDigitSeven => '⓻',
            EnclosedAlphanumerics::DoubleCircledDigitEight => '⓼',
            EnclosedAlphanumerics::DoubleCircledDigitNine => '⓽',
            EnclosedAlphanumerics::DoubleCircledNumberTen => '⓾',
        }
    }
}

impl std::convert::TryFrom<char> for EnclosedAlphanumerics {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '①' => Ok(EnclosedAlphanumerics::CircledDigitOne),
            '②' => Ok(EnclosedAlphanumerics::CircledDigitTwo),
            '③' => Ok(EnclosedAlphanumerics::CircledDigitThree),
            '④' => Ok(EnclosedAlphanumerics::CircledDigitFour),
            '⑤' => Ok(EnclosedAlphanumerics::CircledDigitFive),
            '⑥' => Ok(EnclosedAlphanumerics::CircledDigitSix),
            '⑦' => Ok(EnclosedAlphanumerics::CircledDigitSeven),
            '⑧' => Ok(EnclosedAlphanumerics::CircledDigitEight),
            '⑨' => Ok(EnclosedAlphanumerics::CircledDigitNine),
            '⑩' => Ok(EnclosedAlphanumerics::CircledNumberTen),
            '⑪' => Ok(EnclosedAlphanumerics::CircledNumberEleven),
            '⑫' => Ok(EnclosedAlphanumerics::CircledNumberTwelve),
            '⑬' => Ok(EnclosedAlphanumerics::CircledNumberThirteen),
            '⑭' => Ok(EnclosedAlphanumerics::CircledNumberFourteen),
            '⑮' => Ok(EnclosedAlphanumerics::CircledNumberFifteen),
            '⑯' => Ok(EnclosedAlphanumerics::CircledNumberSixteen),
            '⑰' => Ok(EnclosedAlphanumerics::CircledNumberSeventeen),
            '⑱' => Ok(EnclosedAlphanumerics::CircledNumberEighteen),
            '⑲' => Ok(EnclosedAlphanumerics::CircledNumberNineteen),
            '⑳' => Ok(EnclosedAlphanumerics::CircledNumberTwenty),
            '⑴' => Ok(EnclosedAlphanumerics::ParenthesizedDigitOne),
            '⑵' => Ok(EnclosedAlphanumerics::ParenthesizedDigitTwo),
            '⑶' => Ok(EnclosedAlphanumerics::ParenthesizedDigitThree),
            '⑷' => Ok(EnclosedAlphanumerics::ParenthesizedDigitFour),
            '⑸' => Ok(EnclosedAlphanumerics::ParenthesizedDigitFive),
            '⑹' => Ok(EnclosedAlphanumerics::ParenthesizedDigitSix),
            '⑺' => Ok(EnclosedAlphanumerics::ParenthesizedDigitSeven),
            '⑻' => Ok(EnclosedAlphanumerics::ParenthesizedDigitEight),
            '⑼' => Ok(EnclosedAlphanumerics::ParenthesizedDigitNine),
            '⑽' => Ok(EnclosedAlphanumerics::ParenthesizedNumberTen),
            '⑾' => Ok(EnclosedAlphanumerics::ParenthesizedNumberEleven),
            '⑿' => Ok(EnclosedAlphanumerics::ParenthesizedNumberTwelve),
            '⒀' => Ok(EnclosedAlphanumerics::ParenthesizedNumberThirteen),
            '⒁' => Ok(EnclosedAlphanumerics::ParenthesizedNumberFourteen),
            '⒂' => Ok(EnclosedAlphanumerics::ParenthesizedNumberFifteen),
            '⒃' => Ok(EnclosedAlphanumerics::ParenthesizedNumberSixteen),
            '⒄' => Ok(EnclosedAlphanumerics::ParenthesizedNumberSeventeen),
            '⒅' => Ok(EnclosedAlphanumerics::ParenthesizedNumberEighteen),
            '⒆' => Ok(EnclosedAlphanumerics::ParenthesizedNumberNineteen),
            '⒇' => Ok(EnclosedAlphanumerics::ParenthesizedNumberTwenty),
            '⒈' => Ok(EnclosedAlphanumerics::DigitOneFullStop),
            '⒉' => Ok(EnclosedAlphanumerics::DigitTwoFullStop),
            '⒊' => Ok(EnclosedAlphanumerics::DigitThreeFullStop),
            '⒋' => Ok(EnclosedAlphanumerics::DigitFourFullStop),
            '⒌' => Ok(EnclosedAlphanumerics::DigitFiveFullStop),
            '⒍' => Ok(EnclosedAlphanumerics::DigitSixFullStop),
            '⒎' => Ok(EnclosedAlphanumerics::DigitSevenFullStop),
            '⒏' => Ok(EnclosedAlphanumerics::DigitEightFullStop),
            '⒐' => Ok(EnclosedAlphanumerics::DigitNineFullStop),
            '⒑' => Ok(EnclosedAlphanumerics::NumberTenFullStop),
            '⒒' => Ok(EnclosedAlphanumerics::NumberElevenFullStop),
            '⒓' => Ok(EnclosedAlphanumerics::NumberTwelveFullStop),
            '⒔' => Ok(EnclosedAlphanumerics::NumberThirteenFullStop),
            '⒕' => Ok(EnclosedAlphanumerics::NumberFourteenFullStop),
            '⒖' => Ok(EnclosedAlphanumerics::NumberFifteenFullStop),
            '⒗' => Ok(EnclosedAlphanumerics::NumberSixteenFullStop),
            '⒘' => Ok(EnclosedAlphanumerics::NumberSeventeenFullStop),
            '⒙' => Ok(EnclosedAlphanumerics::NumberEighteenFullStop),
            '⒚' => Ok(EnclosedAlphanumerics::NumberNineteenFullStop),
            '⒛' => Ok(EnclosedAlphanumerics::NumberTwentyFullStop),
            '⒜' => Ok(EnclosedAlphanumerics::ParenthesizedLatinSmallLetterA),
            '⒝' => Ok(EnclosedAlphanumerics::ParenthesizedLatinSmallLetterB),
            '⒞' => Ok(EnclosedAlphanumerics::ParenthesizedLatinSmallLetterC),
            '⒟' => Ok(EnclosedAlphanumerics::ParenthesizedLatinSmallLetterD),
            '⒠' => Ok(EnclosedAlphanumerics::ParenthesizedLatinSmallLetterE),
            '⒡' => Ok(EnclosedAlphanumerics::ParenthesizedLatinSmallLetterF),
            '⒢' => Ok(EnclosedAlphanumerics::ParenthesizedLatinSmallLetterG),
            '⒣' => Ok(EnclosedAlphanumerics::ParenthesizedLatinSmallLetterH),
            '⒤' => Ok(EnclosedAlphanumerics::ParenthesizedLatinSmallLetterI),
            '⒥' => Ok(EnclosedAlphanumerics::ParenthesizedLatinSmallLetterJ),
            '⒦' => Ok(EnclosedAlphanumerics::ParenthesizedLatinSmallLetterK),
            '⒧' => Ok(EnclosedAlphanumerics::ParenthesizedLatinSmallLetterL),
            '⒨' => Ok(EnclosedAlphanumerics::ParenthesizedLatinSmallLetterM),
            '⒩' => Ok(EnclosedAlphanumerics::ParenthesizedLatinSmallLetterN),
            '⒪' => Ok(EnclosedAlphanumerics::ParenthesizedLatinSmallLetterO),
            '⒫' => Ok(EnclosedAlphanumerics::ParenthesizedLatinSmallLetterP),
            '⒬' => Ok(EnclosedAlphanumerics::ParenthesizedLatinSmallLetterQ),
            '⒭' => Ok(EnclosedAlphanumerics::ParenthesizedLatinSmallLetterR),
            '⒮' => Ok(EnclosedAlphanumerics::ParenthesizedLatinSmallLetterS),
            '⒯' => Ok(EnclosedAlphanumerics::ParenthesizedLatinSmallLetterT),
            '⒰' => Ok(EnclosedAlphanumerics::ParenthesizedLatinSmallLetterU),
            '⒱' => Ok(EnclosedAlphanumerics::ParenthesizedLatinSmallLetterV),
            '⒲' => Ok(EnclosedAlphanumerics::ParenthesizedLatinSmallLetterW),
            '⒳' => Ok(EnclosedAlphanumerics::ParenthesizedLatinSmallLetterX),
            '⒴' => Ok(EnclosedAlphanumerics::ParenthesizedLatinSmallLetterY),
            '⒵' => Ok(EnclosedAlphanumerics::ParenthesizedLatinSmallLetterZ),
            'Ⓐ' => Ok(EnclosedAlphanumerics::CircledLatinCapitalLetterA),
            'Ⓑ' => Ok(EnclosedAlphanumerics::CircledLatinCapitalLetterB),
            'Ⓒ' => Ok(EnclosedAlphanumerics::CircledLatinCapitalLetterC),
            'Ⓓ' => Ok(EnclosedAlphanumerics::CircledLatinCapitalLetterD),
            'Ⓔ' => Ok(EnclosedAlphanumerics::CircledLatinCapitalLetterE),
            'Ⓕ' => Ok(EnclosedAlphanumerics::CircledLatinCapitalLetterF),
            'Ⓖ' => Ok(EnclosedAlphanumerics::CircledLatinCapitalLetterG),
            'Ⓗ' => Ok(EnclosedAlphanumerics::CircledLatinCapitalLetterH),
            'Ⓘ' => Ok(EnclosedAlphanumerics::CircledLatinCapitalLetterI),
            'Ⓙ' => Ok(EnclosedAlphanumerics::CircledLatinCapitalLetterJ),
            'Ⓚ' => Ok(EnclosedAlphanumerics::CircledLatinCapitalLetterK),
            'Ⓛ' => Ok(EnclosedAlphanumerics::CircledLatinCapitalLetterL),
            'Ⓜ' => Ok(EnclosedAlphanumerics::CircledLatinCapitalLetterM),
            'Ⓝ' => Ok(EnclosedAlphanumerics::CircledLatinCapitalLetterN),
            'Ⓞ' => Ok(EnclosedAlphanumerics::CircledLatinCapitalLetterO),
            'Ⓟ' => Ok(EnclosedAlphanumerics::CircledLatinCapitalLetterP),
            'Ⓠ' => Ok(EnclosedAlphanumerics::CircledLatinCapitalLetterQ),
            'Ⓡ' => Ok(EnclosedAlphanumerics::CircledLatinCapitalLetterR),
            'Ⓢ' => Ok(EnclosedAlphanumerics::CircledLatinCapitalLetterS),
            'Ⓣ' => Ok(EnclosedAlphanumerics::CircledLatinCapitalLetterT),
            'Ⓤ' => Ok(EnclosedAlphanumerics::CircledLatinCapitalLetterU),
            'Ⓥ' => Ok(EnclosedAlphanumerics::CircledLatinCapitalLetterV),
            'Ⓦ' => Ok(EnclosedAlphanumerics::CircledLatinCapitalLetterW),
            'Ⓧ' => Ok(EnclosedAlphanumerics::CircledLatinCapitalLetterX),
            'Ⓨ' => Ok(EnclosedAlphanumerics::CircledLatinCapitalLetterY),
            'Ⓩ' => Ok(EnclosedAlphanumerics::CircledLatinCapitalLetterZ),
            'ⓐ' => Ok(EnclosedAlphanumerics::CircledLatinSmallLetterA),
            'ⓑ' => Ok(EnclosedAlphanumerics::CircledLatinSmallLetterB),
            'ⓒ' => Ok(EnclosedAlphanumerics::CircledLatinSmallLetterC),
            'ⓓ' => Ok(EnclosedAlphanumerics::CircledLatinSmallLetterD),
            'ⓔ' => Ok(EnclosedAlphanumerics::CircledLatinSmallLetterE),
            'ⓕ' => Ok(EnclosedAlphanumerics::CircledLatinSmallLetterF),
            'ⓖ' => Ok(EnclosedAlphanumerics::CircledLatinSmallLetterG),
            'ⓗ' => Ok(EnclosedAlphanumerics::CircledLatinSmallLetterH),
            'ⓘ' => Ok(EnclosedAlphanumerics::CircledLatinSmallLetterI),
            'ⓙ' => Ok(EnclosedAlphanumerics::CircledLatinSmallLetterJ),
            'ⓚ' => Ok(EnclosedAlphanumerics::CircledLatinSmallLetterK),
            'ⓛ' => Ok(EnclosedAlphanumerics::CircledLatinSmallLetterL),
            'ⓜ' => Ok(EnclosedAlphanumerics::CircledLatinSmallLetterM),
            'ⓝ' => Ok(EnclosedAlphanumerics::CircledLatinSmallLetterN),
            'ⓞ' => Ok(EnclosedAlphanumerics::CircledLatinSmallLetterO),
            'ⓟ' => Ok(EnclosedAlphanumerics::CircledLatinSmallLetterP),
            'ⓠ' => Ok(EnclosedAlphanumerics::CircledLatinSmallLetterQ),
            'ⓡ' => Ok(EnclosedAlphanumerics::CircledLatinSmallLetterR),
            'ⓢ' => Ok(EnclosedAlphanumerics::CircledLatinSmallLetterS),
            'ⓣ' => Ok(EnclosedAlphanumerics::CircledLatinSmallLetterT),
            'ⓤ' => Ok(EnclosedAlphanumerics::CircledLatinSmallLetterU),
            'ⓥ' => Ok(EnclosedAlphanumerics::CircledLatinSmallLetterV),
            'ⓦ' => Ok(EnclosedAlphanumerics::CircledLatinSmallLetterW),
            'ⓧ' => Ok(EnclosedAlphanumerics::CircledLatinSmallLetterX),
            'ⓨ' => Ok(EnclosedAlphanumerics::CircledLatinSmallLetterY),
            'ⓩ' => Ok(EnclosedAlphanumerics::CircledLatinSmallLetterZ),
            '⓪' => Ok(EnclosedAlphanumerics::CircledDigitZero),
            '⓫' => Ok(EnclosedAlphanumerics::NegativeCircledNumberEleven),
            '⓬' => Ok(EnclosedAlphanumerics::NegativeCircledNumberTwelve),
            '⓭' => Ok(EnclosedAlphanumerics::NegativeCircledNumberThirteen),
            '⓮' => Ok(EnclosedAlphanumerics::NegativeCircledNumberFourteen),
            '⓯' => Ok(EnclosedAlphanumerics::NegativeCircledNumberFifteen),
            '⓰' => Ok(EnclosedAlphanumerics::NegativeCircledNumberSixteen),
            '⓱' => Ok(EnclosedAlphanumerics::NegativeCircledNumberSeventeen),
            '⓲' => Ok(EnclosedAlphanumerics::NegativeCircledNumberEighteen),
            '⓳' => Ok(EnclosedAlphanumerics::NegativeCircledNumberNineteen),
            '⓴' => Ok(EnclosedAlphanumerics::NegativeCircledNumberTwenty),
            '⓵' => Ok(EnclosedAlphanumerics::DoubleCircledDigitOne),
            '⓶' => Ok(EnclosedAlphanumerics::DoubleCircledDigitTwo),
            '⓷' => Ok(EnclosedAlphanumerics::DoubleCircledDigitThree),
            '⓸' => Ok(EnclosedAlphanumerics::DoubleCircledDigitFour),
            '⓹' => Ok(EnclosedAlphanumerics::DoubleCircledDigitFive),
            '⓺' => Ok(EnclosedAlphanumerics::DoubleCircledDigitSix),
            '⓻' => Ok(EnclosedAlphanumerics::DoubleCircledDigitSeven),
            '⓼' => Ok(EnclosedAlphanumerics::DoubleCircledDigitEight),
            '⓽' => Ok(EnclosedAlphanumerics::DoubleCircledDigitNine),
            '⓾' => Ok(EnclosedAlphanumerics::DoubleCircledNumberTen),
            _ => Err(()),
        }
    }
}

impl Into<u32> for EnclosedAlphanumerics {
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

impl std::convert::TryFrom<u32> for EnclosedAlphanumerics {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for EnclosedAlphanumerics {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl EnclosedAlphanumerics {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        EnclosedAlphanumerics::CircledDigitOne
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("EnclosedAlphanumerics{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
