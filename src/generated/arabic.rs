
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

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Arabic::NumberSign => "arabic number sign",
            Arabic::SignSanah => "arabic sign sanah",
            Arabic::FootnoteMarker => "arabic footnote marker",
            Arabic::SignSafha => "arabic sign safha",
            Arabic::SignSamvat => "arabic sign samvat",
            Arabic::NumberMarkAbove => "arabic number mark above",
            Arabic::DashIndicCubeRoot => "arabic-indic cube root",
            Arabic::DashIndicFourthRoot => "arabic-indic fourth root",
            Arabic::Ray => "arabic ray",
            Arabic::DashIndicPerMilleSign => "arabic-indic per mille sign",
            Arabic::DashIndicPerTenThousandSign => "arabic-indic per ten thousand sign",
            Arabic::AfghaniSign => "afghani sign",
            Arabic::Comma => "arabic comma",
            Arabic::DateSeparator => "arabic date separator",
            Arabic::PoeticVerseSign => "arabic poetic verse sign",
            Arabic::SignMisra => "arabic sign misra",
            Arabic::SignSallallahouAlayheWassallam => "arabic sign sallallahou alayhe wassallam",
            Arabic::SignAlayheAssallam => "arabic sign alayhe assallam",
            Arabic::SignRahmatullahAlayhe => "arabic sign rahmatullah alayhe",
            Arabic::SignRadiAllahouAnhu => "arabic sign radi allahou anhu",
            Arabic::SignTakhallus => "arabic sign takhallus",
            Arabic::SmallHighTah => "arabic small high tah",
            Arabic::SmallHighLigatureAlefWithLamWithYeh => "arabic small high ligature alef with lam with yeh",
            Arabic::SmallHighZain => "arabic small high zain",
            Arabic::SmallFatha => "arabic small fatha",
            Arabic::SmallDamma => "arabic small damma",
            Arabic::SmallKasra => "arabic small kasra",
            Arabic::Semicolon => "arabic semicolon",
            Arabic::LetterMark => "arabic letter mark",
            Arabic::TripleDotPunctuationMark => "arabic triple dot punctuation mark",
            Arabic::QuestionMark => "arabic question mark",
            Arabic::LetterKashmiriYeh => "arabic letter kashmiri yeh",
            Arabic::LetterHamza => "arabic letter hamza",
            Arabic::LetterAlefWithMaddaAbove => "arabic letter alef with madda above",
            Arabic::LetterAlefWithHamzaAbove => "arabic letter alef with hamza above",
            Arabic::LetterWawWithHamzaAbove => "arabic letter waw with hamza above",
            Arabic::LetterAlefWithHamzaBelow => "arabic letter alef with hamza below",
            Arabic::LetterYehWithHamzaAbove => "arabic letter yeh with hamza above",
            Arabic::LetterAlef => "arabic letter alef",
            Arabic::LetterBeh => "arabic letter beh",
            Arabic::LetterTehMarbuta => "arabic letter teh marbuta",
            Arabic::LetterTeh => "arabic letter teh",
            Arabic::LetterTheh => "arabic letter theh",
            Arabic::LetterJeem => "arabic letter jeem",
            Arabic::LetterHah => "arabic letter hah",
            Arabic::LetterKhah => "arabic letter khah",
            Arabic::LetterDal => "arabic letter dal",
            Arabic::LetterThal => "arabic letter thal",
            Arabic::LetterReh => "arabic letter reh",
            Arabic::LetterZain => "arabic letter zain",
            Arabic::LetterSeen => "arabic letter seen",
            Arabic::LetterSheen => "arabic letter sheen",
            Arabic::LetterSad => "arabic letter sad",
            Arabic::LetterDad => "arabic letter dad",
            Arabic::LetterTah => "arabic letter tah",
            Arabic::LetterZah => "arabic letter zah",
            Arabic::LetterAin => "arabic letter ain",
            Arabic::LetterGhain => "arabic letter ghain",
            Arabic::LetterKehehWithTwoDotsAbove => "arabic letter keheh with two dots above",
            Arabic::LetterKehehWithThreeDotsBelow => "arabic letter keheh with three dots below",
            Arabic::LetterFarsiYehWithInvertedV => "arabic letter farsi yeh with inverted v",
            Arabic::LetterFarsiYehWithTwoDotsAbove => "arabic letter farsi yeh with two dots above",
            Arabic::LetterFarsiYehWithThreeDotsAbove => "arabic letter farsi yeh with three dots above",
            Arabic::Tatweel => "arabic tatweel",
            Arabic::LetterFeh => "arabic letter feh",
            Arabic::LetterQaf => "arabic letter qaf",
            Arabic::LetterKaf => "arabic letter kaf",
            Arabic::LetterLam => "arabic letter lam",
            Arabic::LetterMeem => "arabic letter meem",
            Arabic::LetterNoon => "arabic letter noon",
            Arabic::LetterHeh => "arabic letter heh",
            Arabic::LetterWaw => "arabic letter waw",
            Arabic::LetterAlefMaksura => "arabic letter alef maksura",
            Arabic::LetterYeh => "arabic letter yeh",
            Arabic::Fathatan => "arabic fathatan",
            Arabic::Dammatan => "arabic dammatan",
            Arabic::Kasratan => "arabic kasratan",
            Arabic::Fatha => "arabic fatha",
            Arabic::Damma => "arabic damma",
            Arabic::Kasra => "arabic kasra",
            Arabic::Shadda => "arabic shadda",
            Arabic::Sukun => "arabic sukun",
            Arabic::MaddahAbove => "arabic maddah above",
            Arabic::HamzaAbove => "arabic hamza above",
            Arabic::HamzaBelow => "arabic hamza below",
            Arabic::SubscriptAlef => "arabic subscript alef",
            Arabic::InvertedDamma => "arabic inverted damma",
            Arabic::MarkNoonGhunna => "arabic mark noon ghunna",
            Arabic::Zwarakay => "arabic zwarakay",
            Arabic::VowelSignSmallVAbove => "arabic vowel sign small v above",
            Arabic::VowelSignInvertedSmallVAbove => "arabic vowel sign inverted small v above",
            Arabic::VowelSignDotBelow => "arabic vowel sign dot below",
            Arabic::ReversedDamma => "arabic reversed damma",
            Arabic::FathaWithTwoDots => "arabic fatha with two dots",
            Arabic::WavyHamzaBelow => "arabic wavy hamza below",
            Arabic::DashIndicDigitZero => "arabic-indic digit zero",
            Arabic::DashIndicDigitOne => "arabic-indic digit one",
            Arabic::DashIndicDigitTwo => "arabic-indic digit two",
            Arabic::DashIndicDigitThree => "arabic-indic digit three",
            Arabic::DashIndicDigitFour => "arabic-indic digit four",
            Arabic::DashIndicDigitFive => "arabic-indic digit five",
            Arabic::DashIndicDigitSix => "arabic-indic digit six",
            Arabic::DashIndicDigitSeven => "arabic-indic digit seven",
            Arabic::DashIndicDigitEight => "arabic-indic digit eight",
            Arabic::DashIndicDigitNine => "arabic-indic digit nine",
            Arabic::PercentSign => "arabic percent sign",
            Arabic::DecimalSeparator => "arabic decimal separator",
            Arabic::ThousandsSeparator => "arabic thousands separator",
            Arabic::FivePointedStar => "arabic five pointed star",
            Arabic::LetterDotlessBeh => "arabic letter dotless beh",
            Arabic::LetterDotlessQaf => "arabic letter dotless qaf",
            Arabic::LetterSuperscriptAlef => "arabic letter superscript alef",
            Arabic::LetterAlefWasla => "arabic letter alef wasla",
            Arabic::LetterAlefWithWavyHamzaAbove => "arabic letter alef with wavy hamza above",
            Arabic::LetterAlefWithWavyHamzaBelow => "arabic letter alef with wavy hamza below",
            Arabic::LetterHighHamza => "arabic letter high hamza",
            Arabic::LetterHighHamzaAlef => "arabic letter high hamza alef",
            Arabic::LetterHighHamzaWaw => "arabic letter high hamza waw",
            Arabic::LetterUWithHamzaAbove => "arabic letter u with hamza above",
            Arabic::LetterHighHamzaYeh => "arabic letter high hamza yeh",
            Arabic::LetterTteh => "arabic letter tteh",
            Arabic::LetterTteheh => "arabic letter tteheh",
            Arabic::LetterBeeh => "arabic letter beeh",
            Arabic::LetterTehWithRing => "arabic letter teh with ring",
            Arabic::LetterTehWithThreeDotsAboveDownwards => "arabic letter teh with three dots above downwards",
            Arabic::LetterPeh => "arabic letter peh",
            Arabic::LetterTeheh => "arabic letter teheh",
            Arabic::LetterBeheh => "arabic letter beheh",
            Arabic::LetterHahWithHamzaAbove => "arabic letter hah with hamza above",
            Arabic::LetterHahWithTwoDotsVerticalAbove => "arabic letter hah with two dots vertical above",
            Arabic::LetterNyeh => "arabic letter nyeh",
            Arabic::LetterDyeh => "arabic letter dyeh",
            Arabic::LetterHahWithThreeDotsAbove => "arabic letter hah with three dots above",
            Arabic::LetterTcheh => "arabic letter tcheh",
            Arabic::LetterTcheheh => "arabic letter tcheheh",
            Arabic::LetterDdal => "arabic letter ddal",
            Arabic::LetterDalWithRing => "arabic letter dal with ring",
            Arabic::LetterDalWithDotBelow => "arabic letter dal with dot below",
            Arabic::LetterDalWithDotBelowAndSmallTah => "arabic letter dal with dot below and small tah",
            Arabic::LetterDahal => "arabic letter dahal",
            Arabic::LetterDdahal => "arabic letter ddahal",
            Arabic::LetterDul => "arabic letter dul",
            Arabic::LetterDalWithThreeDotsAboveDownwards => "arabic letter dal with three dots above downwards",
            Arabic::LetterDalWithFourDotsAbove => "arabic letter dal with four dots above",
            Arabic::LetterRreh => "arabic letter rreh",
            Arabic::LetterRehWithSmallV => "arabic letter reh with small v",
            Arabic::LetterRehWithRing => "arabic letter reh with ring",
            Arabic::LetterRehWithDotBelow => "arabic letter reh with dot below",
            Arabic::LetterRehWithSmallVBelow => "arabic letter reh with small v below",
            Arabic::LetterRehWithDotBelowAndDotAbove => "arabic letter reh with dot below and dot above",
            Arabic::LetterRehWithTwoDotsAbove => "arabic letter reh with two dots above",
            Arabic::LetterJeh => "arabic letter jeh",
            Arabic::LetterRehWithFourDotsAbove => "arabic letter reh with four dots above",
            Arabic::LetterSeenWithDotBelowAndDotAbove => "arabic letter seen with dot below and dot above",
            Arabic::LetterSeenWithThreeDotsBelow => "arabic letter seen with three dots below",
            Arabic::LetterSeenWithThreeDotsBelowAndThreeDotsAbove => "arabic letter seen with three dots below and three dots above",
            Arabic::LetterSadWithTwoDotsBelow => "arabic letter sad with two dots below",
            Arabic::LetterSadWithThreeDotsAbove => "arabic letter sad with three dots above",
            Arabic::LetterTahWithThreeDotsAbove => "arabic letter tah with three dots above",
            Arabic::LetterAinWithThreeDotsAbove => "arabic letter ain with three dots above",
            Arabic::LetterDotlessFeh => "arabic letter dotless feh",
            Arabic::LetterFehWithDotMovedBelow => "arabic letter feh with dot moved below",
            Arabic::LetterFehWithDotBelow => "arabic letter feh with dot below",
            Arabic::LetterVeh => "arabic letter veh",
            Arabic::LetterFehWithThreeDotsBelow => "arabic letter feh with three dots below",
            Arabic::LetterPeheh => "arabic letter peheh",
            Arabic::LetterQafWithDotAbove => "arabic letter qaf with dot above",
            Arabic::LetterQafWithThreeDotsAbove => "arabic letter qaf with three dots above",
            Arabic::LetterKeheh => "arabic letter keheh",
            Arabic::LetterSwashKaf => "arabic letter swash kaf",
            Arabic::LetterKafWithRing => "arabic letter kaf with ring",
            Arabic::LetterKafWithDotAbove => "arabic letter kaf with dot above",
            Arabic::LetterNg => "arabic letter ng",
            Arabic::LetterKafWithThreeDotsBelow => "arabic letter kaf with three dots below",
            Arabic::LetterGaf => "arabic letter gaf",
            Arabic::LetterGafWithRing => "arabic letter gaf with ring",
            Arabic::LetterNgoeh => "arabic letter ngoeh",
            Arabic::LetterGafWithTwoDotsBelow => "arabic letter gaf with two dots below",
            Arabic::LetterGueh => "arabic letter gueh",
            Arabic::LetterGafWithThreeDotsAbove => "arabic letter gaf with three dots above",
            Arabic::LetterLamWithSmallV => "arabic letter lam with small v",
            Arabic::LetterLamWithDotAbove => "arabic letter lam with dot above",
            Arabic::LetterLamWithThreeDotsAbove => "arabic letter lam with three dots above",
            Arabic::LetterLamWithThreeDotsBelow => "arabic letter lam with three dots below",
            Arabic::LetterNoonWithDotBelow => "arabic letter noon with dot below",
            Arabic::LetterNoonGhunna => "arabic letter noon ghunna",
            Arabic::LetterRnoon => "arabic letter rnoon",
            Arabic::LetterNoonWithRing => "arabic letter noon with ring",
            Arabic::LetterNoonWithThreeDotsAbove => "arabic letter noon with three dots above",
            Arabic::LetterHehDoachashmee => "arabic letter heh doachashmee",
            Arabic::LetterTchehWithDotAbove => "arabic letter tcheh with dot above",
            Arabic::LetterHehWithYehAbove => "arabic letter heh with yeh above",
            Arabic::LetterHehGoal => "arabic letter heh goal",
            Arabic::LetterHehGoalWithHamzaAbove => "arabic letter heh goal with hamza above",
            Arabic::LetterTehMarbutaGoal => "arabic letter teh marbuta goal",
            Arabic::LetterWawWithRing => "arabic letter waw with ring",
            Arabic::LetterKirghizOe => "arabic letter kirghiz oe",
            Arabic::LetterOe => "arabic letter oe",
            Arabic::LetterU => "arabic letter u",
            Arabic::LetterYu => "arabic letter yu",
            Arabic::LetterKirghizYu => "arabic letter kirghiz yu",
            Arabic::LetterWawWithTwoDotsAbove => "arabic letter waw with two dots above",
            Arabic::LetterVe => "arabic letter ve",
            Arabic::LetterFarsiYeh => "arabic letter farsi yeh",
            Arabic::LetterYehWithTail => "arabic letter yeh with tail",
            Arabic::LetterYehWithSmallV => "arabic letter yeh with small v",
            Arabic::LetterWawWithDotAbove => "arabic letter waw with dot above",
            Arabic::LetterE => "arabic letter e",
            Arabic::LetterYehWithThreeDotsBelow => "arabic letter yeh with three dots below",
            Arabic::LetterYehBarree => "arabic letter yeh barree",
            Arabic::LetterYehBarreeWithHamzaAbove => "arabic letter yeh barree with hamza above",
            Arabic::FullStop => "arabic full stop",
            Arabic::LetterAe => "arabic letter ae",
            Arabic::SmallHighLigatureSadWithLamWithAlefMaksura => "arabic small high ligature sad with lam with alef maksura",
            Arabic::SmallHighLigatureQafWithLamWithAlefMaksura => "arabic small high ligature qaf with lam with alef maksura",
            Arabic::SmallHighMeemInitialForm => "arabic small high meem initial form",
            Arabic::SmallHighLamAlef => "arabic small high lam alef",
            Arabic::SmallHighJeem => "arabic small high jeem",
            Arabic::SmallHighThreeDots => "arabic small high three dots",
            Arabic::SmallHighSeen => "arabic small high seen",
            Arabic::EndOfAyah => "arabic end of ayah",
            Arabic::StartOfRubElHizb => "arabic start of rub el hizb",
            Arabic::SmallHighRoundedZero => "arabic small high rounded zero",
            Arabic::SmallHighUprightRectangularZero => "arabic small high upright rectangular zero",
            Arabic::SmallHighDotlessHeadOfKhah => "arabic small high dotless head of khah",
            Arabic::SmallHighMeemIsolatedForm => "arabic small high meem isolated form",
            Arabic::SmallLowSeen => "arabic small low seen",
            Arabic::SmallHighMadda => "arabic small high madda",
            Arabic::SmallWaw => "arabic small waw",
            Arabic::SmallYeh => "arabic small yeh",
            Arabic::SmallHighYeh => "arabic small high yeh",
            Arabic::SmallHighNoon => "arabic small high noon",
            Arabic::PlaceOfSajdah => "arabic place of sajdah",
            Arabic::EmptyCentreLowStop => "arabic empty centre low stop",
            Arabic::EmptyCentreHighStop => "arabic empty centre high stop",
            Arabic::RoundedHighStopWithFilledCentre => "arabic rounded high stop with filled centre",
            Arabic::SmallLowMeem => "arabic small low meem",
            Arabic::LetterDalWithInvertedV => "arabic letter dal with inverted v",
            Arabic::LetterRehWithInvertedV => "arabic letter reh with inverted v",
            Arabic::ExtendedDashIndicDigitZero => "extended arabic-indic digit zero",
            Arabic::ExtendedDashIndicDigitOne => "extended arabic-indic digit one",
            Arabic::ExtendedDashIndicDigitTwo => "extended arabic-indic digit two",
            Arabic::ExtendedDashIndicDigitThree => "extended arabic-indic digit three",
            Arabic::ExtendedDashIndicDigitFour => "extended arabic-indic digit four",
            Arabic::ExtendedDashIndicDigitFive => "extended arabic-indic digit five",
            Arabic::ExtendedDashIndicDigitSix => "extended arabic-indic digit six",
            Arabic::ExtendedDashIndicDigitSeven => "extended arabic-indic digit seven",
            Arabic::ExtendedDashIndicDigitEight => "extended arabic-indic digit eight",
            Arabic::ExtendedDashIndicDigitNine => "extended arabic-indic digit nine",
            Arabic::LetterSheenWithDotBelow => "arabic letter sheen with dot below",
            Arabic::LetterDadWithDotBelow => "arabic letter dad with dot below",
            Arabic::LetterGhainWithDotBelow => "arabic letter ghain with dot below",
            Arabic::SignSindhiAmpersand => "arabic sign sindhi ampersand",
            Arabic::SignSindhiPostpositionMen => "arabic sign sindhi postposition men",
        }
    }
}
