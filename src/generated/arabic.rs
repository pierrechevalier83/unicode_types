
/// An enum to represent all characters in the Arabic block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Arabic {
    /// \u{600}: '؀'
    NumberSign,
    /// \u{601}: '؁'
    SignSanah,
    /// \u{602}: '؂'
    FootnoteMarker,
    /// \u{603}: '؃'
    SignSafha,
    /// \u{604}: '؄'
    SignSamvat,
    /// \u{605}: '؅'
    NumberMarkAbove,
    /// \u{606}: '؆'
    DashIndicCubeRoot,
    /// \u{607}: '؇'
    DashIndicFourthRoot,
    /// \u{608}: '؈'
    Ray,
    /// \u{609}: '؉'
    DashIndicPerMilleSign,
    /// \u{60a}: '؊'
    DashIndicPerTenThousandSign,
    /// \u{60b}: '؋'
    AfghaniSign,
    /// \u{60c}: '،'
    Comma,
    /// \u{60d}: '؍'
    DateSeparator,
    /// \u{60e}: '؎'
    PoeticVerseSign,
    /// \u{60f}: '؏'
    SignMisra,
    /// \u{610}: 'ؐ'
    SignSallallahouAlayheWassallam,
    /// \u{611}: 'ؑ'
    SignAlayheAssallam,
    /// \u{612}: 'ؒ'
    SignRahmatullahAlayhe,
    /// \u{613}: 'ؓ'
    SignRadiAllahouAnhu,
    /// \u{614}: 'ؔ'
    SignTakhallus,
    /// \u{615}: 'ؕ'
    SmallHighTah,
    /// \u{616}: 'ؖ'
    SmallHighLigatureAlefWithLamWithYeh,
    /// \u{617}: 'ؗ'
    SmallHighZain,
    /// \u{618}: 'ؘ'
    SmallFatha,
    /// \u{619}: 'ؙ'
    SmallDamma,
    /// \u{61a}: 'ؚ'
    SmallKasra,
    /// \u{61b}: '؛'
    Semicolon,
    /// \u{61c}: '؜'
    LetterMark,
    /// \u{61e}: '؞'
    TripleDotPunctuationMark,
    /// \u{61f}: '؟'
    QuestionMark,
    /// \u{620}: 'ؠ'
    LetterKashmiriYeh,
    /// \u{621}: 'ء'
    LetterHamza,
    /// \u{622}: 'آ'
    LetterAlefWithMaddaAbove,
    /// \u{623}: 'أ'
    LetterAlefWithHamzaAbove,
    /// \u{624}: 'ؤ'
    LetterWawWithHamzaAbove,
    /// \u{625}: 'إ'
    LetterAlefWithHamzaBelow,
    /// \u{626}: 'ئ'
    LetterYehWithHamzaAbove,
    /// \u{627}: 'ا'
    LetterAlef,
    /// \u{628}: 'ب'
    LetterBeh,
    /// \u{629}: 'ة'
    LetterTehMarbuta,
    /// \u{62a}: 'ت'
    LetterTeh,
    /// \u{62b}: 'ث'
    LetterTheh,
    /// \u{62c}: 'ج'
    LetterJeem,
    /// \u{62d}: 'ح'
    LetterHah,
    /// \u{62e}: 'خ'
    LetterKhah,
    /// \u{62f}: 'د'
    LetterDal,
    /// \u{630}: 'ذ'
    LetterThal,
    /// \u{631}: 'ر'
    LetterReh,
    /// \u{632}: 'ز'
    LetterZain,
    /// \u{633}: 'س'
    LetterSeen,
    /// \u{634}: 'ش'
    LetterSheen,
    /// \u{635}: 'ص'
    LetterSad,
    /// \u{636}: 'ض'
    LetterDad,
    /// \u{637}: 'ط'
    LetterTah,
    /// \u{638}: 'ظ'
    LetterZah,
    /// \u{639}: 'ع'
    LetterAin,
    /// \u{63a}: 'غ'
    LetterGhain,
    /// \u{63b}: 'ػ'
    LetterKehehWithTwoDotsAbove,
    /// \u{63c}: 'ؼ'
    LetterKehehWithThreeDotsBelow,
    /// \u{63d}: 'ؽ'
    LetterFarsiYehWithInvertedV,
    /// \u{63e}: 'ؾ'
    LetterFarsiYehWithTwoDotsAbove,
    /// \u{63f}: 'ؿ'
    LetterFarsiYehWithThreeDotsAbove,
    /// \u{640}: 'ـ'
    Tatweel,
    /// \u{641}: 'ف'
    LetterFeh,
    /// \u{642}: 'ق'
    LetterQaf,
    /// \u{643}: 'ك'
    LetterKaf,
    /// \u{644}: 'ل'
    LetterLam,
    /// \u{645}: 'م'
    LetterMeem,
    /// \u{646}: 'ن'
    LetterNoon,
    /// \u{647}: 'ه'
    LetterHeh,
    /// \u{648}: 'و'
    LetterWaw,
    /// \u{649}: 'ى'
    LetterAlefMaksura,
    /// \u{64a}: 'ي'
    LetterYeh,
    /// \u{64b}: 'ً'
    Fathatan,
    /// \u{64c}: 'ٌ'
    Dammatan,
    /// \u{64d}: 'ٍ'
    Kasratan,
    /// \u{64e}: 'َ'
    Fatha,
    /// \u{64f}: 'ُ'
    Damma,
    /// \u{650}: 'ِ'
    Kasra,
    /// \u{651}: 'ّ'
    Shadda,
    /// \u{652}: 'ْ'
    Sukun,
    /// \u{653}: 'ٓ'
    MaddahAbove,
    /// \u{654}: 'ٔ'
    HamzaAbove,
    /// \u{655}: 'ٕ'
    HamzaBelow,
    /// \u{656}: 'ٖ'
    SubscriptAlef,
    /// \u{657}: 'ٗ'
    InvertedDamma,
    /// \u{658}: '٘'
    MarkNoonGhunna,
    /// \u{659}: 'ٙ'
    Zwarakay,
    /// \u{65a}: 'ٚ'
    VowelSignSmallVAbove,
    /// \u{65b}: 'ٛ'
    VowelSignInvertedSmallVAbove,
    /// \u{65c}: 'ٜ'
    VowelSignDotBelow,
    /// \u{65d}: 'ٝ'
    ReversedDamma,
    /// \u{65e}: 'ٞ'
    FathaWithTwoDots,
    /// \u{65f}: 'ٟ'
    WavyHamzaBelow,
    /// \u{660}: '٠'
    DashIndicDigitZero,
    /// \u{661}: '١'
    DashIndicDigitOne,
    /// \u{662}: '٢'
    DashIndicDigitTwo,
    /// \u{663}: '٣'
    DashIndicDigitThree,
    /// \u{664}: '٤'
    DashIndicDigitFour,
    /// \u{665}: '٥'
    DashIndicDigitFive,
    /// \u{666}: '٦'
    DashIndicDigitSix,
    /// \u{667}: '٧'
    DashIndicDigitSeven,
    /// \u{668}: '٨'
    DashIndicDigitEight,
    /// \u{669}: '٩'
    DashIndicDigitNine,
    /// \u{66a}: '٪'
    PercentSign,
    /// \u{66b}: '٫'
    DecimalSeparator,
    /// \u{66c}: '٬'
    ThousandsSeparator,
    /// \u{66d}: '٭'
    FivePointedStar,
    /// \u{66e}: 'ٮ'
    LetterDotlessBeh,
    /// \u{66f}: 'ٯ'
    LetterDotlessQaf,
    /// \u{670}: 'ٰ'
    LetterSuperscriptAlef,
    /// \u{671}: 'ٱ'
    LetterAlefWasla,
    /// \u{672}: 'ٲ'
    LetterAlefWithWavyHamzaAbove,
    /// \u{673}: 'ٳ'
    LetterAlefWithWavyHamzaBelow,
    /// \u{674}: 'ٴ'
    LetterHighHamza,
    /// \u{675}: 'ٵ'
    LetterHighHamzaAlef,
    /// \u{676}: 'ٶ'
    LetterHighHamzaWaw,
    /// \u{677}: 'ٷ'
    LetterUWithHamzaAbove,
    /// \u{678}: 'ٸ'
    LetterHighHamzaYeh,
    /// \u{679}: 'ٹ'
    LetterTteh,
    /// \u{67a}: 'ٺ'
    LetterTteheh,
    /// \u{67b}: 'ٻ'
    LetterBeeh,
    /// \u{67c}: 'ټ'
    LetterTehWithRing,
    /// \u{67d}: 'ٽ'
    LetterTehWithThreeDotsAboveDownwards,
    /// \u{67e}: 'پ'
    LetterPeh,
    /// \u{67f}: 'ٿ'
    LetterTeheh,
    /// \u{680}: 'ڀ'
    LetterBeheh,
    /// \u{681}: 'ځ'
    LetterHahWithHamzaAbove,
    /// \u{682}: 'ڂ'
    LetterHahWithTwoDotsVerticalAbove,
    /// \u{683}: 'ڃ'
    LetterNyeh,
    /// \u{684}: 'ڄ'
    LetterDyeh,
    /// \u{685}: 'څ'
    LetterHahWithThreeDotsAbove,
    /// \u{686}: 'چ'
    LetterTcheh,
    /// \u{687}: 'ڇ'
    LetterTcheheh,
    /// \u{688}: 'ڈ'
    LetterDdal,
    /// \u{689}: 'ډ'
    LetterDalWithRing,
    /// \u{68a}: 'ڊ'
    LetterDalWithDotBelow,
    /// \u{68b}: 'ڋ'
    LetterDalWithDotBelowAndSmallTah,
    /// \u{68c}: 'ڌ'
    LetterDahal,
    /// \u{68d}: 'ڍ'
    LetterDdahal,
    /// \u{68e}: 'ڎ'
    LetterDul,
    /// \u{68f}: 'ڏ'
    LetterDalWithThreeDotsAboveDownwards,
    /// \u{690}: 'ڐ'
    LetterDalWithFourDotsAbove,
    /// \u{691}: 'ڑ'
    LetterRreh,
    /// \u{692}: 'ڒ'
    LetterRehWithSmallV,
    /// \u{693}: 'ړ'
    LetterRehWithRing,
    /// \u{694}: 'ڔ'
    LetterRehWithDotBelow,
    /// \u{695}: 'ڕ'
    LetterRehWithSmallVBelow,
    /// \u{696}: 'ږ'
    LetterRehWithDotBelowAndDotAbove,
    /// \u{697}: 'ڗ'
    LetterRehWithTwoDotsAbove,
    /// \u{698}: 'ژ'
    LetterJeh,
    /// \u{699}: 'ڙ'
    LetterRehWithFourDotsAbove,
    /// \u{69a}: 'ښ'
    LetterSeenWithDotBelowAndDotAbove,
    /// \u{69b}: 'ڛ'
    LetterSeenWithThreeDotsBelow,
    /// \u{69c}: 'ڜ'
    LetterSeenWithThreeDotsBelowAndThreeDotsAbove,
    /// \u{69d}: 'ڝ'
    LetterSadWithTwoDotsBelow,
    /// \u{69e}: 'ڞ'
    LetterSadWithThreeDotsAbove,
    /// \u{69f}: 'ڟ'
    LetterTahWithThreeDotsAbove,
    /// \u{6a0}: 'ڠ'
    LetterAinWithThreeDotsAbove,
    /// \u{6a1}: 'ڡ'
    LetterDotlessFeh,
    /// \u{6a2}: 'ڢ'
    LetterFehWithDotMovedBelow,
    /// \u{6a3}: 'ڣ'
    LetterFehWithDotBelow,
    /// \u{6a4}: 'ڤ'
    LetterVeh,
    /// \u{6a5}: 'ڥ'
    LetterFehWithThreeDotsBelow,
    /// \u{6a6}: 'ڦ'
    LetterPeheh,
    /// \u{6a7}: 'ڧ'
    LetterQafWithDotAbove,
    /// \u{6a8}: 'ڨ'
    LetterQafWithThreeDotsAbove,
    /// \u{6a9}: 'ک'
    LetterKeheh,
    /// \u{6aa}: 'ڪ'
    LetterSwashKaf,
    /// \u{6ab}: 'ګ'
    LetterKafWithRing,
    /// \u{6ac}: 'ڬ'
    LetterKafWithDotAbove,
    /// \u{6ad}: 'ڭ'
    LetterNg,
    /// \u{6ae}: 'ڮ'
    LetterKafWithThreeDotsBelow,
    /// \u{6af}: 'گ'
    LetterGaf,
    /// \u{6b0}: 'ڰ'
    LetterGafWithRing,
    /// \u{6b1}: 'ڱ'
    LetterNgoeh,
    /// \u{6b2}: 'ڲ'
    LetterGafWithTwoDotsBelow,
    /// \u{6b3}: 'ڳ'
    LetterGueh,
    /// \u{6b4}: 'ڴ'
    LetterGafWithThreeDotsAbove,
    /// \u{6b5}: 'ڵ'
    LetterLamWithSmallV,
    /// \u{6b6}: 'ڶ'
    LetterLamWithDotAbove,
    /// \u{6b7}: 'ڷ'
    LetterLamWithThreeDotsAbove,
    /// \u{6b8}: 'ڸ'
    LetterLamWithThreeDotsBelow,
    /// \u{6b9}: 'ڹ'
    LetterNoonWithDotBelow,
    /// \u{6ba}: 'ں'
    LetterNoonGhunna,
    /// \u{6bb}: 'ڻ'
    LetterRnoon,
    /// \u{6bc}: 'ڼ'
    LetterNoonWithRing,
    /// \u{6bd}: 'ڽ'
    LetterNoonWithThreeDotsAbove,
    /// \u{6be}: 'ھ'
    LetterHehDoachashmee,
    /// \u{6bf}: 'ڿ'
    LetterTchehWithDotAbove,
    /// \u{6c0}: 'ۀ'
    LetterHehWithYehAbove,
    /// \u{6c1}: 'ہ'
    LetterHehGoal,
    /// \u{6c2}: 'ۂ'
    LetterHehGoalWithHamzaAbove,
    /// \u{6c3}: 'ۃ'
    LetterTehMarbutaGoal,
    /// \u{6c4}: 'ۄ'
    LetterWawWithRing,
    /// \u{6c5}: 'ۅ'
    LetterKirghizOe,
    /// \u{6c6}: 'ۆ'
    LetterOe,
    /// \u{6c7}: 'ۇ'
    LetterU,
    /// \u{6c8}: 'ۈ'
    LetterYu,
    /// \u{6c9}: 'ۉ'
    LetterKirghizYu,
    /// \u{6ca}: 'ۊ'
    LetterWawWithTwoDotsAbove,
    /// \u{6cb}: 'ۋ'
    LetterVe,
    /// \u{6cc}: 'ی'
    LetterFarsiYeh,
    /// \u{6cd}: 'ۍ'
    LetterYehWithTail,
    /// \u{6ce}: 'ێ'
    LetterYehWithSmallV,
    /// \u{6cf}: 'ۏ'
    LetterWawWithDotAbove,
    /// \u{6d0}: 'ې'
    LetterE,
    /// \u{6d1}: 'ۑ'
    LetterYehWithThreeDotsBelow,
    /// \u{6d2}: 'ے'
    LetterYehBarree,
    /// \u{6d3}: 'ۓ'
    LetterYehBarreeWithHamzaAbove,
    /// \u{6d4}: '۔'
    FullStop,
    /// \u{6d5}: 'ە'
    LetterAe,
    /// \u{6d6}: 'ۖ'
    SmallHighLigatureSadWithLamWithAlefMaksura,
    /// \u{6d7}: 'ۗ'
    SmallHighLigatureQafWithLamWithAlefMaksura,
    /// \u{6d8}: 'ۘ'
    SmallHighMeemInitialForm,
    /// \u{6d9}: 'ۙ'
    SmallHighLamAlef,
    /// \u{6da}: 'ۚ'
    SmallHighJeem,
    /// \u{6db}: 'ۛ'
    SmallHighThreeDots,
    /// \u{6dc}: 'ۜ'
    SmallHighSeen,
    /// \u{6dd}: '۝'
    EndOfAyah,
    /// \u{6de}: '۞'
    StartOfRubElHizb,
    /// \u{6df}: '۟'
    SmallHighRoundedZero,
    /// \u{6e0}: '۠'
    SmallHighUprightRectangularZero,
    /// \u{6e1}: 'ۡ'
    SmallHighDotlessHeadOfKhah,
    /// \u{6e2}: 'ۢ'
    SmallHighMeemIsolatedForm,
    /// \u{6e3}: 'ۣ'
    SmallLowSeen,
    /// \u{6e4}: 'ۤ'
    SmallHighMadda,
    /// \u{6e5}: 'ۥ'
    SmallWaw,
    /// \u{6e6}: 'ۦ'
    SmallYeh,
    /// \u{6e7}: 'ۧ'
    SmallHighYeh,
    /// \u{6e8}: 'ۨ'
    SmallHighNoon,
    /// \u{6e9}: '۩'
    PlaceOfSajdah,
    /// \u{6ea}: '۪'
    EmptyCentreLowStop,
    /// \u{6eb}: '۫'
    EmptyCentreHighStop,
    /// \u{6ec}: '۬'
    RoundedHighStopWithFilledCentre,
    /// \u{6ed}: 'ۭ'
    SmallLowMeem,
    /// \u{6ee}: 'ۮ'
    LetterDalWithInvertedV,
    /// \u{6ef}: 'ۯ'
    LetterRehWithInvertedV,
    /// \u{6f0}: '۰'
    ExtendedDashIndicDigitZero,
    /// \u{6f1}: '۱'
    ExtendedDashIndicDigitOne,
    /// \u{6f2}: '۲'
    ExtendedDashIndicDigitTwo,
    /// \u{6f3}: '۳'
    ExtendedDashIndicDigitThree,
    /// \u{6f4}: '۴'
    ExtendedDashIndicDigitFour,
    /// \u{6f5}: '۵'
    ExtendedDashIndicDigitFive,
    /// \u{6f6}: '۶'
    ExtendedDashIndicDigitSix,
    /// \u{6f7}: '۷'
    ExtendedDashIndicDigitSeven,
    /// \u{6f8}: '۸'
    ExtendedDashIndicDigitEight,
    /// \u{6f9}: '۹'
    ExtendedDashIndicDigitNine,
    /// \u{6fa}: 'ۺ'
    LetterSheenWithDotBelow,
    /// \u{6fb}: 'ۻ'
    LetterDadWithDotBelow,
    /// \u{6fc}: 'ۼ'
    LetterGhainWithDotBelow,
    /// \u{6fd}: '۽'
    SignSindhiAmpersand,
    /// \u{6fe}: '۾'
    SignSindhiPostpositionMen,
}

impl Into<char> for Arabic {
    fn into(self) -> char {
        match self {
            Arabic::NumberSign => '؀',
            Arabic::SignSanah => '؁',
            Arabic::FootnoteMarker => '؂',
            Arabic::SignSafha => '؃',
            Arabic::SignSamvat => '؄',
            Arabic::NumberMarkAbove => '؅',
            Arabic::DashIndicCubeRoot => '؆',
            Arabic::DashIndicFourthRoot => '؇',
            Arabic::Ray => '؈',
            Arabic::DashIndicPerMilleSign => '؉',
            Arabic::DashIndicPerTenThousandSign => '؊',
            Arabic::AfghaniSign => '؋',
            Arabic::Comma => '،',
            Arabic::DateSeparator => '؍',
            Arabic::PoeticVerseSign => '؎',
            Arabic::SignMisra => '؏',
            Arabic::SignSallallahouAlayheWassallam => 'ؐ',
            Arabic::SignAlayheAssallam => 'ؑ',
            Arabic::SignRahmatullahAlayhe => 'ؒ',
            Arabic::SignRadiAllahouAnhu => 'ؓ',
            Arabic::SignTakhallus => 'ؔ',
            Arabic::SmallHighTah => 'ؕ',
            Arabic::SmallHighLigatureAlefWithLamWithYeh => 'ؖ',
            Arabic::SmallHighZain => 'ؗ',
            Arabic::SmallFatha => 'ؘ',
            Arabic::SmallDamma => 'ؙ',
            Arabic::SmallKasra => 'ؚ',
            Arabic::Semicolon => '؛',
            Arabic::LetterMark => '؜',
            Arabic::TripleDotPunctuationMark => '؞',
            Arabic::QuestionMark => '؟',
            Arabic::LetterKashmiriYeh => 'ؠ',
            Arabic::LetterHamza => 'ء',
            Arabic::LetterAlefWithMaddaAbove => 'آ',
            Arabic::LetterAlefWithHamzaAbove => 'أ',
            Arabic::LetterWawWithHamzaAbove => 'ؤ',
            Arabic::LetterAlefWithHamzaBelow => 'إ',
            Arabic::LetterYehWithHamzaAbove => 'ئ',
            Arabic::LetterAlef => 'ا',
            Arabic::LetterBeh => 'ب',
            Arabic::LetterTehMarbuta => 'ة',
            Arabic::LetterTeh => 'ت',
            Arabic::LetterTheh => 'ث',
            Arabic::LetterJeem => 'ج',
            Arabic::LetterHah => 'ح',
            Arabic::LetterKhah => 'خ',
            Arabic::LetterDal => 'د',
            Arabic::LetterThal => 'ذ',
            Arabic::LetterReh => 'ر',
            Arabic::LetterZain => 'ز',
            Arabic::LetterSeen => 'س',
            Arabic::LetterSheen => 'ش',
            Arabic::LetterSad => 'ص',
            Arabic::LetterDad => 'ض',
            Arabic::LetterTah => 'ط',
            Arabic::LetterZah => 'ظ',
            Arabic::LetterAin => 'ع',
            Arabic::LetterGhain => 'غ',
            Arabic::LetterKehehWithTwoDotsAbove => 'ػ',
            Arabic::LetterKehehWithThreeDotsBelow => 'ؼ',
            Arabic::LetterFarsiYehWithInvertedV => 'ؽ',
            Arabic::LetterFarsiYehWithTwoDotsAbove => 'ؾ',
            Arabic::LetterFarsiYehWithThreeDotsAbove => 'ؿ',
            Arabic::Tatweel => 'ـ',
            Arabic::LetterFeh => 'ف',
            Arabic::LetterQaf => 'ق',
            Arabic::LetterKaf => 'ك',
            Arabic::LetterLam => 'ل',
            Arabic::LetterMeem => 'م',
            Arabic::LetterNoon => 'ن',
            Arabic::LetterHeh => 'ه',
            Arabic::LetterWaw => 'و',
            Arabic::LetterAlefMaksura => 'ى',
            Arabic::LetterYeh => 'ي',
            Arabic::Fathatan => 'ً',
            Arabic::Dammatan => 'ٌ',
            Arabic::Kasratan => 'ٍ',
            Arabic::Fatha => 'َ',
            Arabic::Damma => 'ُ',
            Arabic::Kasra => 'ِ',
            Arabic::Shadda => 'ّ',
            Arabic::Sukun => 'ْ',
            Arabic::MaddahAbove => 'ٓ',
            Arabic::HamzaAbove => 'ٔ',
            Arabic::HamzaBelow => 'ٕ',
            Arabic::SubscriptAlef => 'ٖ',
            Arabic::InvertedDamma => 'ٗ',
            Arabic::MarkNoonGhunna => '٘',
            Arabic::Zwarakay => 'ٙ',
            Arabic::VowelSignSmallVAbove => 'ٚ',
            Arabic::VowelSignInvertedSmallVAbove => 'ٛ',
            Arabic::VowelSignDotBelow => 'ٜ',
            Arabic::ReversedDamma => 'ٝ',
            Arabic::FathaWithTwoDots => 'ٞ',
            Arabic::WavyHamzaBelow => 'ٟ',
            Arabic::DashIndicDigitZero => '٠',
            Arabic::DashIndicDigitOne => '١',
            Arabic::DashIndicDigitTwo => '٢',
            Arabic::DashIndicDigitThree => '٣',
            Arabic::DashIndicDigitFour => '٤',
            Arabic::DashIndicDigitFive => '٥',
            Arabic::DashIndicDigitSix => '٦',
            Arabic::DashIndicDigitSeven => '٧',
            Arabic::DashIndicDigitEight => '٨',
            Arabic::DashIndicDigitNine => '٩',
            Arabic::PercentSign => '٪',
            Arabic::DecimalSeparator => '٫',
            Arabic::ThousandsSeparator => '٬',
            Arabic::FivePointedStar => '٭',
            Arabic::LetterDotlessBeh => 'ٮ',
            Arabic::LetterDotlessQaf => 'ٯ',
            Arabic::LetterSuperscriptAlef => 'ٰ',
            Arabic::LetterAlefWasla => 'ٱ',
            Arabic::LetterAlefWithWavyHamzaAbove => 'ٲ',
            Arabic::LetterAlefWithWavyHamzaBelow => 'ٳ',
            Arabic::LetterHighHamza => 'ٴ',
            Arabic::LetterHighHamzaAlef => 'ٵ',
            Arabic::LetterHighHamzaWaw => 'ٶ',
            Arabic::LetterUWithHamzaAbove => 'ٷ',
            Arabic::LetterHighHamzaYeh => 'ٸ',
            Arabic::LetterTteh => 'ٹ',
            Arabic::LetterTteheh => 'ٺ',
            Arabic::LetterBeeh => 'ٻ',
            Arabic::LetterTehWithRing => 'ټ',
            Arabic::LetterTehWithThreeDotsAboveDownwards => 'ٽ',
            Arabic::LetterPeh => 'پ',
            Arabic::LetterTeheh => 'ٿ',
            Arabic::LetterBeheh => 'ڀ',
            Arabic::LetterHahWithHamzaAbove => 'ځ',
            Arabic::LetterHahWithTwoDotsVerticalAbove => 'ڂ',
            Arabic::LetterNyeh => 'ڃ',
            Arabic::LetterDyeh => 'ڄ',
            Arabic::LetterHahWithThreeDotsAbove => 'څ',
            Arabic::LetterTcheh => 'چ',
            Arabic::LetterTcheheh => 'ڇ',
            Arabic::LetterDdal => 'ڈ',
            Arabic::LetterDalWithRing => 'ډ',
            Arabic::LetterDalWithDotBelow => 'ڊ',
            Arabic::LetterDalWithDotBelowAndSmallTah => 'ڋ',
            Arabic::LetterDahal => 'ڌ',
            Arabic::LetterDdahal => 'ڍ',
            Arabic::LetterDul => 'ڎ',
            Arabic::LetterDalWithThreeDotsAboveDownwards => 'ڏ',
            Arabic::LetterDalWithFourDotsAbove => 'ڐ',
            Arabic::LetterRreh => 'ڑ',
            Arabic::LetterRehWithSmallV => 'ڒ',
            Arabic::LetterRehWithRing => 'ړ',
            Arabic::LetterRehWithDotBelow => 'ڔ',
            Arabic::LetterRehWithSmallVBelow => 'ڕ',
            Arabic::LetterRehWithDotBelowAndDotAbove => 'ږ',
            Arabic::LetterRehWithTwoDotsAbove => 'ڗ',
            Arabic::LetterJeh => 'ژ',
            Arabic::LetterRehWithFourDotsAbove => 'ڙ',
            Arabic::LetterSeenWithDotBelowAndDotAbove => 'ښ',
            Arabic::LetterSeenWithThreeDotsBelow => 'ڛ',
            Arabic::LetterSeenWithThreeDotsBelowAndThreeDotsAbove => 'ڜ',
            Arabic::LetterSadWithTwoDotsBelow => 'ڝ',
            Arabic::LetterSadWithThreeDotsAbove => 'ڞ',
            Arabic::LetterTahWithThreeDotsAbove => 'ڟ',
            Arabic::LetterAinWithThreeDotsAbove => 'ڠ',
            Arabic::LetterDotlessFeh => 'ڡ',
            Arabic::LetterFehWithDotMovedBelow => 'ڢ',
            Arabic::LetterFehWithDotBelow => 'ڣ',
            Arabic::LetterVeh => 'ڤ',
            Arabic::LetterFehWithThreeDotsBelow => 'ڥ',
            Arabic::LetterPeheh => 'ڦ',
            Arabic::LetterQafWithDotAbove => 'ڧ',
            Arabic::LetterQafWithThreeDotsAbove => 'ڨ',
            Arabic::LetterKeheh => 'ک',
            Arabic::LetterSwashKaf => 'ڪ',
            Arabic::LetterKafWithRing => 'ګ',
            Arabic::LetterKafWithDotAbove => 'ڬ',
            Arabic::LetterNg => 'ڭ',
            Arabic::LetterKafWithThreeDotsBelow => 'ڮ',
            Arabic::LetterGaf => 'گ',
            Arabic::LetterGafWithRing => 'ڰ',
            Arabic::LetterNgoeh => 'ڱ',
            Arabic::LetterGafWithTwoDotsBelow => 'ڲ',
            Arabic::LetterGueh => 'ڳ',
            Arabic::LetterGafWithThreeDotsAbove => 'ڴ',
            Arabic::LetterLamWithSmallV => 'ڵ',
            Arabic::LetterLamWithDotAbove => 'ڶ',
            Arabic::LetterLamWithThreeDotsAbove => 'ڷ',
            Arabic::LetterLamWithThreeDotsBelow => 'ڸ',
            Arabic::LetterNoonWithDotBelow => 'ڹ',
            Arabic::LetterNoonGhunna => 'ں',
            Arabic::LetterRnoon => 'ڻ',
            Arabic::LetterNoonWithRing => 'ڼ',
            Arabic::LetterNoonWithThreeDotsAbove => 'ڽ',
            Arabic::LetterHehDoachashmee => 'ھ',
            Arabic::LetterTchehWithDotAbove => 'ڿ',
            Arabic::LetterHehWithYehAbove => 'ۀ',
            Arabic::LetterHehGoal => 'ہ',
            Arabic::LetterHehGoalWithHamzaAbove => 'ۂ',
            Arabic::LetterTehMarbutaGoal => 'ۃ',
            Arabic::LetterWawWithRing => 'ۄ',
            Arabic::LetterKirghizOe => 'ۅ',
            Arabic::LetterOe => 'ۆ',
            Arabic::LetterU => 'ۇ',
            Arabic::LetterYu => 'ۈ',
            Arabic::LetterKirghizYu => 'ۉ',
            Arabic::LetterWawWithTwoDotsAbove => 'ۊ',
            Arabic::LetterVe => 'ۋ',
            Arabic::LetterFarsiYeh => 'ی',
            Arabic::LetterYehWithTail => 'ۍ',
            Arabic::LetterYehWithSmallV => 'ێ',
            Arabic::LetterWawWithDotAbove => 'ۏ',
            Arabic::LetterE => 'ې',
            Arabic::LetterYehWithThreeDotsBelow => 'ۑ',
            Arabic::LetterYehBarree => 'ے',
            Arabic::LetterYehBarreeWithHamzaAbove => 'ۓ',
            Arabic::FullStop => '۔',
            Arabic::LetterAe => 'ە',
            Arabic::SmallHighLigatureSadWithLamWithAlefMaksura => 'ۖ',
            Arabic::SmallHighLigatureQafWithLamWithAlefMaksura => 'ۗ',
            Arabic::SmallHighMeemInitialForm => 'ۘ',
            Arabic::SmallHighLamAlef => 'ۙ',
            Arabic::SmallHighJeem => 'ۚ',
            Arabic::SmallHighThreeDots => 'ۛ',
            Arabic::SmallHighSeen => 'ۜ',
            Arabic::EndOfAyah => '۝',
            Arabic::StartOfRubElHizb => '۞',
            Arabic::SmallHighRoundedZero => '۟',
            Arabic::SmallHighUprightRectangularZero => '۠',
            Arabic::SmallHighDotlessHeadOfKhah => 'ۡ',
            Arabic::SmallHighMeemIsolatedForm => 'ۢ',
            Arabic::SmallLowSeen => 'ۣ',
            Arabic::SmallHighMadda => 'ۤ',
            Arabic::SmallWaw => 'ۥ',
            Arabic::SmallYeh => 'ۦ',
            Arabic::SmallHighYeh => 'ۧ',
            Arabic::SmallHighNoon => 'ۨ',
            Arabic::PlaceOfSajdah => '۩',
            Arabic::EmptyCentreLowStop => '۪',
            Arabic::EmptyCentreHighStop => '۫',
            Arabic::RoundedHighStopWithFilledCentre => '۬',
            Arabic::SmallLowMeem => 'ۭ',
            Arabic::LetterDalWithInvertedV => 'ۮ',
            Arabic::LetterRehWithInvertedV => 'ۯ',
            Arabic::ExtendedDashIndicDigitZero => '۰',
            Arabic::ExtendedDashIndicDigitOne => '۱',
            Arabic::ExtendedDashIndicDigitTwo => '۲',
            Arabic::ExtendedDashIndicDigitThree => '۳',
            Arabic::ExtendedDashIndicDigitFour => '۴',
            Arabic::ExtendedDashIndicDigitFive => '۵',
            Arabic::ExtendedDashIndicDigitSix => '۶',
            Arabic::ExtendedDashIndicDigitSeven => '۷',
            Arabic::ExtendedDashIndicDigitEight => '۸',
            Arabic::ExtendedDashIndicDigitNine => '۹',
            Arabic::LetterSheenWithDotBelow => 'ۺ',
            Arabic::LetterDadWithDotBelow => 'ۻ',
            Arabic::LetterGhainWithDotBelow => 'ۼ',
            Arabic::SignSindhiAmpersand => '۽',
            Arabic::SignSindhiPostpositionMen => '۾',
        }
    }
}

impl std::convert::TryFrom<char> for Arabic {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '؀' => Ok(Arabic::NumberSign),
            '؁' => Ok(Arabic::SignSanah),
            '؂' => Ok(Arabic::FootnoteMarker),
            '؃' => Ok(Arabic::SignSafha),
            '؄' => Ok(Arabic::SignSamvat),
            '؅' => Ok(Arabic::NumberMarkAbove),
            '؆' => Ok(Arabic::DashIndicCubeRoot),
            '؇' => Ok(Arabic::DashIndicFourthRoot),
            '؈' => Ok(Arabic::Ray),
            '؉' => Ok(Arabic::DashIndicPerMilleSign),
            '؊' => Ok(Arabic::DashIndicPerTenThousandSign),
            '؋' => Ok(Arabic::AfghaniSign),
            '،' => Ok(Arabic::Comma),
            '؍' => Ok(Arabic::DateSeparator),
            '؎' => Ok(Arabic::PoeticVerseSign),
            '؏' => Ok(Arabic::SignMisra),
            'ؐ' => Ok(Arabic::SignSallallahouAlayheWassallam),
            'ؑ' => Ok(Arabic::SignAlayheAssallam),
            'ؒ' => Ok(Arabic::SignRahmatullahAlayhe),
            'ؓ' => Ok(Arabic::SignRadiAllahouAnhu),
            'ؔ' => Ok(Arabic::SignTakhallus),
            'ؕ' => Ok(Arabic::SmallHighTah),
            'ؖ' => Ok(Arabic::SmallHighLigatureAlefWithLamWithYeh),
            'ؗ' => Ok(Arabic::SmallHighZain),
            'ؘ' => Ok(Arabic::SmallFatha),
            'ؙ' => Ok(Arabic::SmallDamma),
            'ؚ' => Ok(Arabic::SmallKasra),
            '؛' => Ok(Arabic::Semicolon),
            '؜' => Ok(Arabic::LetterMark),
            '؞' => Ok(Arabic::TripleDotPunctuationMark),
            '؟' => Ok(Arabic::QuestionMark),
            'ؠ' => Ok(Arabic::LetterKashmiriYeh),
            'ء' => Ok(Arabic::LetterHamza),
            'آ' => Ok(Arabic::LetterAlefWithMaddaAbove),
            'أ' => Ok(Arabic::LetterAlefWithHamzaAbove),
            'ؤ' => Ok(Arabic::LetterWawWithHamzaAbove),
            'إ' => Ok(Arabic::LetterAlefWithHamzaBelow),
            'ئ' => Ok(Arabic::LetterYehWithHamzaAbove),
            'ا' => Ok(Arabic::LetterAlef),
            'ب' => Ok(Arabic::LetterBeh),
            'ة' => Ok(Arabic::LetterTehMarbuta),
            'ت' => Ok(Arabic::LetterTeh),
            'ث' => Ok(Arabic::LetterTheh),
            'ج' => Ok(Arabic::LetterJeem),
            'ح' => Ok(Arabic::LetterHah),
            'خ' => Ok(Arabic::LetterKhah),
            'د' => Ok(Arabic::LetterDal),
            'ذ' => Ok(Arabic::LetterThal),
            'ر' => Ok(Arabic::LetterReh),
            'ز' => Ok(Arabic::LetterZain),
            'س' => Ok(Arabic::LetterSeen),
            'ش' => Ok(Arabic::LetterSheen),
            'ص' => Ok(Arabic::LetterSad),
            'ض' => Ok(Arabic::LetterDad),
            'ط' => Ok(Arabic::LetterTah),
            'ظ' => Ok(Arabic::LetterZah),
            'ع' => Ok(Arabic::LetterAin),
            'غ' => Ok(Arabic::LetterGhain),
            'ػ' => Ok(Arabic::LetterKehehWithTwoDotsAbove),
            'ؼ' => Ok(Arabic::LetterKehehWithThreeDotsBelow),
            'ؽ' => Ok(Arabic::LetterFarsiYehWithInvertedV),
            'ؾ' => Ok(Arabic::LetterFarsiYehWithTwoDotsAbove),
            'ؿ' => Ok(Arabic::LetterFarsiYehWithThreeDotsAbove),
            'ـ' => Ok(Arabic::Tatweel),
            'ف' => Ok(Arabic::LetterFeh),
            'ق' => Ok(Arabic::LetterQaf),
            'ك' => Ok(Arabic::LetterKaf),
            'ل' => Ok(Arabic::LetterLam),
            'م' => Ok(Arabic::LetterMeem),
            'ن' => Ok(Arabic::LetterNoon),
            'ه' => Ok(Arabic::LetterHeh),
            'و' => Ok(Arabic::LetterWaw),
            'ى' => Ok(Arabic::LetterAlefMaksura),
            'ي' => Ok(Arabic::LetterYeh),
            'ً' => Ok(Arabic::Fathatan),
            'ٌ' => Ok(Arabic::Dammatan),
            'ٍ' => Ok(Arabic::Kasratan),
            'َ' => Ok(Arabic::Fatha),
            'ُ' => Ok(Arabic::Damma),
            'ِ' => Ok(Arabic::Kasra),
            'ّ' => Ok(Arabic::Shadda),
            'ْ' => Ok(Arabic::Sukun),
            'ٓ' => Ok(Arabic::MaddahAbove),
            'ٔ' => Ok(Arabic::HamzaAbove),
            'ٕ' => Ok(Arabic::HamzaBelow),
            'ٖ' => Ok(Arabic::SubscriptAlef),
            'ٗ' => Ok(Arabic::InvertedDamma),
            '٘' => Ok(Arabic::MarkNoonGhunna),
            'ٙ' => Ok(Arabic::Zwarakay),
            'ٚ' => Ok(Arabic::VowelSignSmallVAbove),
            'ٛ' => Ok(Arabic::VowelSignInvertedSmallVAbove),
            'ٜ' => Ok(Arabic::VowelSignDotBelow),
            'ٝ' => Ok(Arabic::ReversedDamma),
            'ٞ' => Ok(Arabic::FathaWithTwoDots),
            'ٟ' => Ok(Arabic::WavyHamzaBelow),
            '٠' => Ok(Arabic::DashIndicDigitZero),
            '١' => Ok(Arabic::DashIndicDigitOne),
            '٢' => Ok(Arabic::DashIndicDigitTwo),
            '٣' => Ok(Arabic::DashIndicDigitThree),
            '٤' => Ok(Arabic::DashIndicDigitFour),
            '٥' => Ok(Arabic::DashIndicDigitFive),
            '٦' => Ok(Arabic::DashIndicDigitSix),
            '٧' => Ok(Arabic::DashIndicDigitSeven),
            '٨' => Ok(Arabic::DashIndicDigitEight),
            '٩' => Ok(Arabic::DashIndicDigitNine),
            '٪' => Ok(Arabic::PercentSign),
            '٫' => Ok(Arabic::DecimalSeparator),
            '٬' => Ok(Arabic::ThousandsSeparator),
            '٭' => Ok(Arabic::FivePointedStar),
            'ٮ' => Ok(Arabic::LetterDotlessBeh),
            'ٯ' => Ok(Arabic::LetterDotlessQaf),
            'ٰ' => Ok(Arabic::LetterSuperscriptAlef),
            'ٱ' => Ok(Arabic::LetterAlefWasla),
            'ٲ' => Ok(Arabic::LetterAlefWithWavyHamzaAbove),
            'ٳ' => Ok(Arabic::LetterAlefWithWavyHamzaBelow),
            'ٴ' => Ok(Arabic::LetterHighHamza),
            'ٵ' => Ok(Arabic::LetterHighHamzaAlef),
            'ٶ' => Ok(Arabic::LetterHighHamzaWaw),
            'ٷ' => Ok(Arabic::LetterUWithHamzaAbove),
            'ٸ' => Ok(Arabic::LetterHighHamzaYeh),
            'ٹ' => Ok(Arabic::LetterTteh),
            'ٺ' => Ok(Arabic::LetterTteheh),
            'ٻ' => Ok(Arabic::LetterBeeh),
            'ټ' => Ok(Arabic::LetterTehWithRing),
            'ٽ' => Ok(Arabic::LetterTehWithThreeDotsAboveDownwards),
            'پ' => Ok(Arabic::LetterPeh),
            'ٿ' => Ok(Arabic::LetterTeheh),
            'ڀ' => Ok(Arabic::LetterBeheh),
            'ځ' => Ok(Arabic::LetterHahWithHamzaAbove),
            'ڂ' => Ok(Arabic::LetterHahWithTwoDotsVerticalAbove),
            'ڃ' => Ok(Arabic::LetterNyeh),
            'ڄ' => Ok(Arabic::LetterDyeh),
            'څ' => Ok(Arabic::LetterHahWithThreeDotsAbove),
            'چ' => Ok(Arabic::LetterTcheh),
            'ڇ' => Ok(Arabic::LetterTcheheh),
            'ڈ' => Ok(Arabic::LetterDdal),
            'ډ' => Ok(Arabic::LetterDalWithRing),
            'ڊ' => Ok(Arabic::LetterDalWithDotBelow),
            'ڋ' => Ok(Arabic::LetterDalWithDotBelowAndSmallTah),
            'ڌ' => Ok(Arabic::LetterDahal),
            'ڍ' => Ok(Arabic::LetterDdahal),
            'ڎ' => Ok(Arabic::LetterDul),
            'ڏ' => Ok(Arabic::LetterDalWithThreeDotsAboveDownwards),
            'ڐ' => Ok(Arabic::LetterDalWithFourDotsAbove),
            'ڑ' => Ok(Arabic::LetterRreh),
            'ڒ' => Ok(Arabic::LetterRehWithSmallV),
            'ړ' => Ok(Arabic::LetterRehWithRing),
            'ڔ' => Ok(Arabic::LetterRehWithDotBelow),
            'ڕ' => Ok(Arabic::LetterRehWithSmallVBelow),
            'ږ' => Ok(Arabic::LetterRehWithDotBelowAndDotAbove),
            'ڗ' => Ok(Arabic::LetterRehWithTwoDotsAbove),
            'ژ' => Ok(Arabic::LetterJeh),
            'ڙ' => Ok(Arabic::LetterRehWithFourDotsAbove),
            'ښ' => Ok(Arabic::LetterSeenWithDotBelowAndDotAbove),
            'ڛ' => Ok(Arabic::LetterSeenWithThreeDotsBelow),
            'ڜ' => Ok(Arabic::LetterSeenWithThreeDotsBelowAndThreeDotsAbove),
            'ڝ' => Ok(Arabic::LetterSadWithTwoDotsBelow),
            'ڞ' => Ok(Arabic::LetterSadWithThreeDotsAbove),
            'ڟ' => Ok(Arabic::LetterTahWithThreeDotsAbove),
            'ڠ' => Ok(Arabic::LetterAinWithThreeDotsAbove),
            'ڡ' => Ok(Arabic::LetterDotlessFeh),
            'ڢ' => Ok(Arabic::LetterFehWithDotMovedBelow),
            'ڣ' => Ok(Arabic::LetterFehWithDotBelow),
            'ڤ' => Ok(Arabic::LetterVeh),
            'ڥ' => Ok(Arabic::LetterFehWithThreeDotsBelow),
            'ڦ' => Ok(Arabic::LetterPeheh),
            'ڧ' => Ok(Arabic::LetterQafWithDotAbove),
            'ڨ' => Ok(Arabic::LetterQafWithThreeDotsAbove),
            'ک' => Ok(Arabic::LetterKeheh),
            'ڪ' => Ok(Arabic::LetterSwashKaf),
            'ګ' => Ok(Arabic::LetterKafWithRing),
            'ڬ' => Ok(Arabic::LetterKafWithDotAbove),
            'ڭ' => Ok(Arabic::LetterNg),
            'ڮ' => Ok(Arabic::LetterKafWithThreeDotsBelow),
            'گ' => Ok(Arabic::LetterGaf),
            'ڰ' => Ok(Arabic::LetterGafWithRing),
            'ڱ' => Ok(Arabic::LetterNgoeh),
            'ڲ' => Ok(Arabic::LetterGafWithTwoDotsBelow),
            'ڳ' => Ok(Arabic::LetterGueh),
            'ڴ' => Ok(Arabic::LetterGafWithThreeDotsAbove),
            'ڵ' => Ok(Arabic::LetterLamWithSmallV),
            'ڶ' => Ok(Arabic::LetterLamWithDotAbove),
            'ڷ' => Ok(Arabic::LetterLamWithThreeDotsAbove),
            'ڸ' => Ok(Arabic::LetterLamWithThreeDotsBelow),
            'ڹ' => Ok(Arabic::LetterNoonWithDotBelow),
            'ں' => Ok(Arabic::LetterNoonGhunna),
            'ڻ' => Ok(Arabic::LetterRnoon),
            'ڼ' => Ok(Arabic::LetterNoonWithRing),
            'ڽ' => Ok(Arabic::LetterNoonWithThreeDotsAbove),
            'ھ' => Ok(Arabic::LetterHehDoachashmee),
            'ڿ' => Ok(Arabic::LetterTchehWithDotAbove),
            'ۀ' => Ok(Arabic::LetterHehWithYehAbove),
            'ہ' => Ok(Arabic::LetterHehGoal),
            'ۂ' => Ok(Arabic::LetterHehGoalWithHamzaAbove),
            'ۃ' => Ok(Arabic::LetterTehMarbutaGoal),
            'ۄ' => Ok(Arabic::LetterWawWithRing),
            'ۅ' => Ok(Arabic::LetterKirghizOe),
            'ۆ' => Ok(Arabic::LetterOe),
            'ۇ' => Ok(Arabic::LetterU),
            'ۈ' => Ok(Arabic::LetterYu),
            'ۉ' => Ok(Arabic::LetterKirghizYu),
            'ۊ' => Ok(Arabic::LetterWawWithTwoDotsAbove),
            'ۋ' => Ok(Arabic::LetterVe),
            'ی' => Ok(Arabic::LetterFarsiYeh),
            'ۍ' => Ok(Arabic::LetterYehWithTail),
            'ێ' => Ok(Arabic::LetterYehWithSmallV),
            'ۏ' => Ok(Arabic::LetterWawWithDotAbove),
            'ې' => Ok(Arabic::LetterE),
            'ۑ' => Ok(Arabic::LetterYehWithThreeDotsBelow),
            'ے' => Ok(Arabic::LetterYehBarree),
            'ۓ' => Ok(Arabic::LetterYehBarreeWithHamzaAbove),
            '۔' => Ok(Arabic::FullStop),
            'ە' => Ok(Arabic::LetterAe),
            'ۖ' => Ok(Arabic::SmallHighLigatureSadWithLamWithAlefMaksura),
            'ۗ' => Ok(Arabic::SmallHighLigatureQafWithLamWithAlefMaksura),
            'ۘ' => Ok(Arabic::SmallHighMeemInitialForm),
            'ۙ' => Ok(Arabic::SmallHighLamAlef),
            'ۚ' => Ok(Arabic::SmallHighJeem),
            'ۛ' => Ok(Arabic::SmallHighThreeDots),
            'ۜ' => Ok(Arabic::SmallHighSeen),
            '۝' => Ok(Arabic::EndOfAyah),
            '۞' => Ok(Arabic::StartOfRubElHizb),
            '۟' => Ok(Arabic::SmallHighRoundedZero),
            '۠' => Ok(Arabic::SmallHighUprightRectangularZero),
            'ۡ' => Ok(Arabic::SmallHighDotlessHeadOfKhah),
            'ۢ' => Ok(Arabic::SmallHighMeemIsolatedForm),
            'ۣ' => Ok(Arabic::SmallLowSeen),
            'ۤ' => Ok(Arabic::SmallHighMadda),
            'ۥ' => Ok(Arabic::SmallWaw),
            'ۦ' => Ok(Arabic::SmallYeh),
            'ۧ' => Ok(Arabic::SmallHighYeh),
            'ۨ' => Ok(Arabic::SmallHighNoon),
            '۩' => Ok(Arabic::PlaceOfSajdah),
            '۪' => Ok(Arabic::EmptyCentreLowStop),
            '۫' => Ok(Arabic::EmptyCentreHighStop),
            '۬' => Ok(Arabic::RoundedHighStopWithFilledCentre),
            'ۭ' => Ok(Arabic::SmallLowMeem),
            'ۮ' => Ok(Arabic::LetterDalWithInvertedV),
            'ۯ' => Ok(Arabic::LetterRehWithInvertedV),
            '۰' => Ok(Arabic::ExtendedDashIndicDigitZero),
            '۱' => Ok(Arabic::ExtendedDashIndicDigitOne),
            '۲' => Ok(Arabic::ExtendedDashIndicDigitTwo),
            '۳' => Ok(Arabic::ExtendedDashIndicDigitThree),
            '۴' => Ok(Arabic::ExtendedDashIndicDigitFour),
            '۵' => Ok(Arabic::ExtendedDashIndicDigitFive),
            '۶' => Ok(Arabic::ExtendedDashIndicDigitSix),
            '۷' => Ok(Arabic::ExtendedDashIndicDigitSeven),
            '۸' => Ok(Arabic::ExtendedDashIndicDigitEight),
            '۹' => Ok(Arabic::ExtendedDashIndicDigitNine),
            'ۺ' => Ok(Arabic::LetterSheenWithDotBelow),
            'ۻ' => Ok(Arabic::LetterDadWithDotBelow),
            'ۼ' => Ok(Arabic::LetterGhainWithDotBelow),
            '۽' => Ok(Arabic::SignSindhiAmpersand),
            '۾' => Ok(Arabic::SignSindhiPostpositionMen),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Arabic {
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

impl std::convert::TryFrom<u32> for Arabic {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Arabic {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Arabic {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Arabic::NumberSign
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Arabic{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
