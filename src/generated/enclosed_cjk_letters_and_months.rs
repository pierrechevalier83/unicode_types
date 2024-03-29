
/// An enum to represent all characters in the EnclosedCJKLettersandMonths block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum EnclosedCJKLettersandMonths {
    /// \u{3200}: '㈀'
    ParenthesizedHangulKiyeok,
    /// \u{3201}: '㈁'
    ParenthesizedHangulNieun,
    /// \u{3202}: '㈂'
    ParenthesizedHangulTikeut,
    /// \u{3203}: '㈃'
    ParenthesizedHangulRieul,
    /// \u{3204}: '㈄'
    ParenthesizedHangulMieum,
    /// \u{3205}: '㈅'
    ParenthesizedHangulPieup,
    /// \u{3206}: '㈆'
    ParenthesizedHangulSios,
    /// \u{3207}: '㈇'
    ParenthesizedHangulIeung,
    /// \u{3208}: '㈈'
    ParenthesizedHangulCieuc,
    /// \u{3209}: '㈉'
    ParenthesizedHangulChieuch,
    /// \u{320a}: '㈊'
    ParenthesizedHangulKhieukh,
    /// \u{320b}: '㈋'
    ParenthesizedHangulThieuth,
    /// \u{320c}: '㈌'
    ParenthesizedHangulPhieuph,
    /// \u{320d}: '㈍'
    ParenthesizedHangulHieuh,
    /// \u{320e}: '㈎'
    ParenthesizedHangulKiyeokA,
    /// \u{320f}: '㈏'
    ParenthesizedHangulNieunA,
    /// \u{3210}: '㈐'
    ParenthesizedHangulTikeutA,
    /// \u{3211}: '㈑'
    ParenthesizedHangulRieulA,
    /// \u{3212}: '㈒'
    ParenthesizedHangulMieumA,
    /// \u{3213}: '㈓'
    ParenthesizedHangulPieupA,
    /// \u{3214}: '㈔'
    ParenthesizedHangulSiosA,
    /// \u{3215}: '㈕'
    ParenthesizedHangulIeungA,
    /// \u{3216}: '㈖'
    ParenthesizedHangulCieucA,
    /// \u{3217}: '㈗'
    ParenthesizedHangulChieuchA,
    /// \u{3218}: '㈘'
    ParenthesizedHangulKhieukhA,
    /// \u{3219}: '㈙'
    ParenthesizedHangulThieuthA,
    /// \u{321a}: '㈚'
    ParenthesizedHangulPhieuphA,
    /// \u{321b}: '㈛'
    ParenthesizedHangulHieuhA,
    /// \u{321c}: '㈜'
    ParenthesizedHangulCieucU,
    /// \u{321d}: '㈝'
    ParenthesizedKoreanCharacterOjeon,
    /// \u{321e}: '㈞'
    ParenthesizedKoreanCharacterOHu,
    /// \u{3220}: '㈠'
    ParenthesizedIdeographOne,
    /// \u{3221}: '㈡'
    ParenthesizedIdeographTwo,
    /// \u{3222}: '㈢'
    ParenthesizedIdeographThree,
    /// \u{3223}: '㈣'
    ParenthesizedIdeographFour,
    /// \u{3224}: '㈤'
    ParenthesizedIdeographFive,
    /// \u{3225}: '㈥'
    ParenthesizedIdeographSix,
    /// \u{3226}: '㈦'
    ParenthesizedIdeographSeven,
    /// \u{3227}: '㈧'
    ParenthesizedIdeographEight,
    /// \u{3228}: '㈨'
    ParenthesizedIdeographNine,
    /// \u{3229}: '㈩'
    ParenthesizedIdeographTen,
    /// \u{322a}: '㈪'
    ParenthesizedIdeographMoon,
    /// \u{322b}: '㈫'
    ParenthesizedIdeographFire,
    /// \u{322c}: '㈬'
    ParenthesizedIdeographWater,
    /// \u{322d}: '㈭'
    ParenthesizedIdeographWood,
    /// \u{322e}: '㈮'
    ParenthesizedIdeographMetal,
    /// \u{322f}: '㈯'
    ParenthesizedIdeographEarth,
    /// \u{3230}: '㈰'
    ParenthesizedIdeographSun,
    /// \u{3231}: '㈱'
    ParenthesizedIdeographStock,
    /// \u{3232}: '㈲'
    ParenthesizedIdeographHave,
    /// \u{3233}: '㈳'
    ParenthesizedIdeographSociety,
    /// \u{3234}: '㈴'
    ParenthesizedIdeographName,
    /// \u{3235}: '㈵'
    ParenthesizedIdeographSpecial,
    /// \u{3236}: '㈶'
    ParenthesizedIdeographFinancial,
    /// \u{3237}: '㈷'
    ParenthesizedIdeographCongratulation,
    /// \u{3238}: '㈸'
    ParenthesizedIdeographLabor,
    /// \u{3239}: '㈹'
    ParenthesizedIdeographRepresent,
    /// \u{323a}: '㈺'
    ParenthesizedIdeographCall,
    /// \u{323b}: '㈻'
    ParenthesizedIdeographStudy,
    /// \u{323c}: '㈼'
    ParenthesizedIdeographSupervise,
    /// \u{323d}: '㈽'
    ParenthesizedIdeographEnterprise,
    /// \u{323e}: '㈾'
    ParenthesizedIdeographResource,
    /// \u{323f}: '㈿'
    ParenthesizedIdeographAlliance,
    /// \u{3240}: '㉀'
    ParenthesizedIdeographFestival,
    /// \u{3241}: '㉁'
    ParenthesizedIdeographRest,
    /// \u{3242}: '㉂'
    ParenthesizedIdeographSelf,
    /// \u{3243}: '㉃'
    ParenthesizedIdeographReach,
    /// \u{3244}: '㉄'
    CircledIdeographQuestion,
    /// \u{3245}: '㉅'
    CircledIdeographKindergarten,
    /// \u{3246}: '㉆'
    CircledIdeographSchool,
    /// \u{3247}: '㉇'
    CircledIdeographKoto,
    /// \u{3248}: '㉈'
    CircledNumberTenOnBlackSquare,
    /// \u{3249}: '㉉'
    CircledNumberTwentyOnBlackSquare,
    /// \u{324a}: '㉊'
    CircledNumberThirtyOnBlackSquare,
    /// \u{324b}: '㉋'
    CircledNumberFortyOnBlackSquare,
    /// \u{324c}: '㉌'
    CircledNumberFiftyOnBlackSquare,
    /// \u{324d}: '㉍'
    CircledNumberSixtyOnBlackSquare,
    /// \u{324e}: '㉎'
    CircledNumberSeventyOnBlackSquare,
    /// \u{324f}: '㉏'
    CircledNumberEightyOnBlackSquare,
    /// \u{3250}: '㉐'
    PartnershipSign,
    /// \u{3251}: '㉑'
    CircledNumberTwentyOne,
    /// \u{3252}: '㉒'
    CircledNumberTwentyTwo,
    /// \u{3253}: '㉓'
    CircledNumberTwentyThree,
    /// \u{3254}: '㉔'
    CircledNumberTwentyFour,
    /// \u{3255}: '㉕'
    CircledNumberTwentyFive,
    /// \u{3256}: '㉖'
    CircledNumberTwentySix,
    /// \u{3257}: '㉗'
    CircledNumberTwentySeven,
    /// \u{3258}: '㉘'
    CircledNumberTwentyEight,
    /// \u{3259}: '㉙'
    CircledNumberTwentyNine,
    /// \u{325a}: '㉚'
    CircledNumberThirty,
    /// \u{325b}: '㉛'
    CircledNumberThirtyOne,
    /// \u{325c}: '㉜'
    CircledNumberThirtyTwo,
    /// \u{325d}: '㉝'
    CircledNumberThirtyThree,
    /// \u{325e}: '㉞'
    CircledNumberThirtyFour,
    /// \u{325f}: '㉟'
    CircledNumberThirtyFive,
    /// \u{3260}: '㉠'
    CircledHangulKiyeok,
    /// \u{3261}: '㉡'
    CircledHangulNieun,
    /// \u{3262}: '㉢'
    CircledHangulTikeut,
    /// \u{3263}: '㉣'
    CircledHangulRieul,
    /// \u{3264}: '㉤'
    CircledHangulMieum,
    /// \u{3265}: '㉥'
    CircledHangulPieup,
    /// \u{3266}: '㉦'
    CircledHangulSios,
    /// \u{3267}: '㉧'
    CircledHangulIeung,
    /// \u{3268}: '㉨'
    CircledHangulCieuc,
    /// \u{3269}: '㉩'
    CircledHangulChieuch,
    /// \u{326a}: '㉪'
    CircledHangulKhieukh,
    /// \u{326b}: '㉫'
    CircledHangulThieuth,
    /// \u{326c}: '㉬'
    CircledHangulPhieuph,
    /// \u{326d}: '㉭'
    CircledHangulHieuh,
    /// \u{326e}: '㉮'
    CircledHangulKiyeokA,
    /// \u{326f}: '㉯'
    CircledHangulNieunA,
    /// \u{3270}: '㉰'
    CircledHangulTikeutA,
    /// \u{3271}: '㉱'
    CircledHangulRieulA,
    /// \u{3272}: '㉲'
    CircledHangulMieumA,
    /// \u{3273}: '㉳'
    CircledHangulPieupA,
    /// \u{3274}: '㉴'
    CircledHangulSiosA,
    /// \u{3275}: '㉵'
    CircledHangulIeungA,
    /// \u{3276}: '㉶'
    CircledHangulCieucA,
    /// \u{3277}: '㉷'
    CircledHangulChieuchA,
    /// \u{3278}: '㉸'
    CircledHangulKhieukhA,
    /// \u{3279}: '㉹'
    CircledHangulThieuthA,
    /// \u{327a}: '㉺'
    CircledHangulPhieuphA,
    /// \u{327b}: '㉻'
    CircledHangulHieuhA,
    /// \u{327c}: '㉼'
    CircledKoreanCharacterChamko,
    /// \u{327d}: '㉽'
    CircledKoreanCharacterJueui,
    /// \u{327e}: '㉾'
    CircledHangulIeungU,
    /// \u{327f}: '㉿'
    KoreanStandardSymbol,
    /// \u{3280}: '㊀'
    CircledIdeographOne,
    /// \u{3281}: '㊁'
    CircledIdeographTwo,
    /// \u{3282}: '㊂'
    CircledIdeographThree,
    /// \u{3283}: '㊃'
    CircledIdeographFour,
    /// \u{3284}: '㊄'
    CircledIdeographFive,
    /// \u{3285}: '㊅'
    CircledIdeographSix,
    /// \u{3286}: '㊆'
    CircledIdeographSeven,
    /// \u{3287}: '㊇'
    CircledIdeographEight,
    /// \u{3288}: '㊈'
    CircledIdeographNine,
    /// \u{3289}: '㊉'
    CircledIdeographTen,
    /// \u{328a}: '㊊'
    CircledIdeographMoon,
    /// \u{328b}: '㊋'
    CircledIdeographFire,
    /// \u{328c}: '㊌'
    CircledIdeographWater,
    /// \u{328d}: '㊍'
    CircledIdeographWood,
    /// \u{328e}: '㊎'
    CircledIdeographMetal,
    /// \u{328f}: '㊏'
    CircledIdeographEarth,
    /// \u{3290}: '㊐'
    CircledIdeographSun,
    /// \u{3291}: '㊑'
    CircledIdeographStock,
    /// \u{3292}: '㊒'
    CircledIdeographHave,
    /// \u{3293}: '㊓'
    CircledIdeographSociety,
    /// \u{3294}: '㊔'
    CircledIdeographName,
    /// \u{3295}: '㊕'
    CircledIdeographSpecial,
    /// \u{3296}: '㊖'
    CircledIdeographFinancial,
    /// \u{3297}: '㊗'
    CircledIdeographCongratulation,
    /// \u{3298}: '㊘'
    CircledIdeographLabor,
    /// \u{3299}: '㊙'
    CircledIdeographSecret,
    /// \u{329a}: '㊚'
    CircledIdeographMale,
    /// \u{329b}: '㊛'
    CircledIdeographFemale,
    /// \u{329c}: '㊜'
    CircledIdeographSuitable,
    /// \u{329d}: '㊝'
    CircledIdeographExcellent,
    /// \u{329e}: '㊞'
    CircledIdeographPrint,
    /// \u{329f}: '㊟'
    CircledIdeographAttention,
    /// \u{32a0}: '㊠'
    CircledIdeographItem,
    /// \u{32a1}: '㊡'
    CircledIdeographRest,
    /// \u{32a2}: '㊢'
    CircledIdeographCopy,
    /// \u{32a3}: '㊣'
    CircledIdeographCorrect,
    /// \u{32a4}: '㊤'
    CircledIdeographHigh,
    /// \u{32a5}: '㊥'
    CircledIdeographCentre,
    /// \u{32a6}: '㊦'
    CircledIdeographLow,
    /// \u{32a7}: '㊧'
    CircledIdeographLeft,
    /// \u{32a8}: '㊨'
    CircledIdeographRight,
    /// \u{32a9}: '㊩'
    CircledIdeographMedicine,
    /// \u{32aa}: '㊪'
    CircledIdeographReligion,
    /// \u{32ab}: '㊫'
    CircledIdeographStudy,
    /// \u{32ac}: '㊬'
    CircledIdeographSupervise,
    /// \u{32ad}: '㊭'
    CircledIdeographEnterprise,
    /// \u{32ae}: '㊮'
    CircledIdeographResource,
    /// \u{32af}: '㊯'
    CircledIdeographAlliance,
    /// \u{32b0}: '㊰'
    CircledIdeographNight,
    /// \u{32b1}: '㊱'
    CircledNumberThirtySix,
    /// \u{32b2}: '㊲'
    CircledNumberThirtySeven,
    /// \u{32b3}: '㊳'
    CircledNumberThirtyEight,
    /// \u{32b4}: '㊴'
    CircledNumberThirtyNine,
    /// \u{32b5}: '㊵'
    CircledNumberForty,
    /// \u{32b6}: '㊶'
    CircledNumberFortyOne,
    /// \u{32b7}: '㊷'
    CircledNumberFortyTwo,
    /// \u{32b8}: '㊸'
    CircledNumberFortyThree,
    /// \u{32b9}: '㊹'
    CircledNumberFortyFour,
    /// \u{32ba}: '㊺'
    CircledNumberFortyFive,
    /// \u{32bb}: '㊻'
    CircledNumberFortySix,
    /// \u{32bc}: '㊼'
    CircledNumberFortySeven,
    /// \u{32bd}: '㊽'
    CircledNumberFortyEight,
    /// \u{32be}: '㊾'
    CircledNumberFortyNine,
    /// \u{32bf}: '㊿'
    CircledNumberFifty,
    /// \u{32c0}: '㋀'
    IdeographicTelegraphSymbolForJanuary,
    /// \u{32c1}: '㋁'
    IdeographicTelegraphSymbolForFebruary,
    /// \u{32c2}: '㋂'
    IdeographicTelegraphSymbolForMarch,
    /// \u{32c3}: '㋃'
    IdeographicTelegraphSymbolForApril,
    /// \u{32c4}: '㋄'
    IdeographicTelegraphSymbolForMay,
    /// \u{32c5}: '㋅'
    IdeographicTelegraphSymbolForJune,
    /// \u{32c6}: '㋆'
    IdeographicTelegraphSymbolForJuly,
    /// \u{32c7}: '㋇'
    IdeographicTelegraphSymbolForAugust,
    /// \u{32c8}: '㋈'
    IdeographicTelegraphSymbolForSeptember,
    /// \u{32c9}: '㋉'
    IdeographicTelegraphSymbolForOctober,
    /// \u{32ca}: '㋊'
    IdeographicTelegraphSymbolForNovember,
    /// \u{32cb}: '㋋'
    IdeographicTelegraphSymbolForDecember,
    /// \u{32cc}: '㋌'
    SquareHg,
    /// \u{32cd}: '㋍'
    SquareErg,
    /// \u{32ce}: '㋎'
    SquareEv,
    /// \u{32cf}: '㋏'
    LimitedLiabilitySign,
    /// \u{32d0}: '㋐'
    CircledKatakanaA,
    /// \u{32d1}: '㋑'
    CircledKatakanaI,
    /// \u{32d2}: '㋒'
    CircledKatakanaU,
    /// \u{32d3}: '㋓'
    CircledKatakanaE,
    /// \u{32d4}: '㋔'
    CircledKatakanaO,
    /// \u{32d5}: '㋕'
    CircledKatakanaKa,
    /// \u{32d6}: '㋖'
    CircledKatakanaKi,
    /// \u{32d7}: '㋗'
    CircledKatakanaKu,
    /// \u{32d8}: '㋘'
    CircledKatakanaKe,
    /// \u{32d9}: '㋙'
    CircledKatakanaKo,
    /// \u{32da}: '㋚'
    CircledKatakanaSa,
    /// \u{32db}: '㋛'
    CircledKatakanaSi,
    /// \u{32dc}: '㋜'
    CircledKatakanaSu,
    /// \u{32dd}: '㋝'
    CircledKatakanaSe,
    /// \u{32de}: '㋞'
    CircledKatakanaSo,
    /// \u{32df}: '㋟'
    CircledKatakanaTa,
    /// \u{32e0}: '㋠'
    CircledKatakanaTi,
    /// \u{32e1}: '㋡'
    CircledKatakanaTu,
    /// \u{32e2}: '㋢'
    CircledKatakanaTe,
    /// \u{32e3}: '㋣'
    CircledKatakanaTo,
    /// \u{32e4}: '㋤'
    CircledKatakanaNa,
    /// \u{32e5}: '㋥'
    CircledKatakanaNi,
    /// \u{32e6}: '㋦'
    CircledKatakanaNu,
    /// \u{32e7}: '㋧'
    CircledKatakanaNe,
    /// \u{32e8}: '㋨'
    CircledKatakanaNo,
    /// \u{32e9}: '㋩'
    CircledKatakanaHa,
    /// \u{32ea}: '㋪'
    CircledKatakanaHi,
    /// \u{32eb}: '㋫'
    CircledKatakanaHu,
    /// \u{32ec}: '㋬'
    CircledKatakanaHe,
    /// \u{32ed}: '㋭'
    CircledKatakanaHo,
    /// \u{32ee}: '㋮'
    CircledKatakanaMa,
    /// \u{32ef}: '㋯'
    CircledKatakanaMi,
    /// \u{32f0}: '㋰'
    CircledKatakanaMu,
    /// \u{32f1}: '㋱'
    CircledKatakanaMe,
    /// \u{32f2}: '㋲'
    CircledKatakanaMo,
    /// \u{32f3}: '㋳'
    CircledKatakanaYa,
    /// \u{32f4}: '㋴'
    CircledKatakanaYu,
    /// \u{32f5}: '㋵'
    CircledKatakanaYo,
    /// \u{32f6}: '㋶'
    CircledKatakanaRa,
    /// \u{32f7}: '㋷'
    CircledKatakanaRi,
    /// \u{32f8}: '㋸'
    CircledKatakanaRu,
    /// \u{32f9}: '㋹'
    CircledKatakanaRe,
    /// \u{32fa}: '㋺'
    CircledKatakanaRo,
    /// \u{32fb}: '㋻'
    CircledKatakanaWa,
    /// \u{32fc}: '㋼'
    CircledKatakanaWi,
    /// \u{32fd}: '㋽'
    CircledKatakanaWe,
    /// \u{32fe}: '㋾'
    CircledKatakanaWo,
}

impl Into<char> for EnclosedCJKLettersandMonths {
    fn into(self) -> char {
        match self {
            EnclosedCJKLettersandMonths::ParenthesizedHangulKiyeok => '㈀',
            EnclosedCJKLettersandMonths::ParenthesizedHangulNieun => '㈁',
            EnclosedCJKLettersandMonths::ParenthesizedHangulTikeut => '㈂',
            EnclosedCJKLettersandMonths::ParenthesizedHangulRieul => '㈃',
            EnclosedCJKLettersandMonths::ParenthesizedHangulMieum => '㈄',
            EnclosedCJKLettersandMonths::ParenthesizedHangulPieup => '㈅',
            EnclosedCJKLettersandMonths::ParenthesizedHangulSios => '㈆',
            EnclosedCJKLettersandMonths::ParenthesizedHangulIeung => '㈇',
            EnclosedCJKLettersandMonths::ParenthesizedHangulCieuc => '㈈',
            EnclosedCJKLettersandMonths::ParenthesizedHangulChieuch => '㈉',
            EnclosedCJKLettersandMonths::ParenthesizedHangulKhieukh => '㈊',
            EnclosedCJKLettersandMonths::ParenthesizedHangulThieuth => '㈋',
            EnclosedCJKLettersandMonths::ParenthesizedHangulPhieuph => '㈌',
            EnclosedCJKLettersandMonths::ParenthesizedHangulHieuh => '㈍',
            EnclosedCJKLettersandMonths::ParenthesizedHangulKiyeokA => '㈎',
            EnclosedCJKLettersandMonths::ParenthesizedHangulNieunA => '㈏',
            EnclosedCJKLettersandMonths::ParenthesizedHangulTikeutA => '㈐',
            EnclosedCJKLettersandMonths::ParenthesizedHangulRieulA => '㈑',
            EnclosedCJKLettersandMonths::ParenthesizedHangulMieumA => '㈒',
            EnclosedCJKLettersandMonths::ParenthesizedHangulPieupA => '㈓',
            EnclosedCJKLettersandMonths::ParenthesizedHangulSiosA => '㈔',
            EnclosedCJKLettersandMonths::ParenthesizedHangulIeungA => '㈕',
            EnclosedCJKLettersandMonths::ParenthesizedHangulCieucA => '㈖',
            EnclosedCJKLettersandMonths::ParenthesizedHangulChieuchA => '㈗',
            EnclosedCJKLettersandMonths::ParenthesizedHangulKhieukhA => '㈘',
            EnclosedCJKLettersandMonths::ParenthesizedHangulThieuthA => '㈙',
            EnclosedCJKLettersandMonths::ParenthesizedHangulPhieuphA => '㈚',
            EnclosedCJKLettersandMonths::ParenthesizedHangulHieuhA => '㈛',
            EnclosedCJKLettersandMonths::ParenthesizedHangulCieucU => '㈜',
            EnclosedCJKLettersandMonths::ParenthesizedKoreanCharacterOjeon => '㈝',
            EnclosedCJKLettersandMonths::ParenthesizedKoreanCharacterOHu => '㈞',
            EnclosedCJKLettersandMonths::ParenthesizedIdeographOne => '㈠',
            EnclosedCJKLettersandMonths::ParenthesizedIdeographTwo => '㈡',
            EnclosedCJKLettersandMonths::ParenthesizedIdeographThree => '㈢',
            EnclosedCJKLettersandMonths::ParenthesizedIdeographFour => '㈣',
            EnclosedCJKLettersandMonths::ParenthesizedIdeographFive => '㈤',
            EnclosedCJKLettersandMonths::ParenthesizedIdeographSix => '㈥',
            EnclosedCJKLettersandMonths::ParenthesizedIdeographSeven => '㈦',
            EnclosedCJKLettersandMonths::ParenthesizedIdeographEight => '㈧',
            EnclosedCJKLettersandMonths::ParenthesizedIdeographNine => '㈨',
            EnclosedCJKLettersandMonths::ParenthesizedIdeographTen => '㈩',
            EnclosedCJKLettersandMonths::ParenthesizedIdeographMoon => '㈪',
            EnclosedCJKLettersandMonths::ParenthesizedIdeographFire => '㈫',
            EnclosedCJKLettersandMonths::ParenthesizedIdeographWater => '㈬',
            EnclosedCJKLettersandMonths::ParenthesizedIdeographWood => '㈭',
            EnclosedCJKLettersandMonths::ParenthesizedIdeographMetal => '㈮',
            EnclosedCJKLettersandMonths::ParenthesizedIdeographEarth => '㈯',
            EnclosedCJKLettersandMonths::ParenthesizedIdeographSun => '㈰',
            EnclosedCJKLettersandMonths::ParenthesizedIdeographStock => '㈱',
            EnclosedCJKLettersandMonths::ParenthesizedIdeographHave => '㈲',
            EnclosedCJKLettersandMonths::ParenthesizedIdeographSociety => '㈳',
            EnclosedCJKLettersandMonths::ParenthesizedIdeographName => '㈴',
            EnclosedCJKLettersandMonths::ParenthesizedIdeographSpecial => '㈵',
            EnclosedCJKLettersandMonths::ParenthesizedIdeographFinancial => '㈶',
            EnclosedCJKLettersandMonths::ParenthesizedIdeographCongratulation => '㈷',
            EnclosedCJKLettersandMonths::ParenthesizedIdeographLabor => '㈸',
            EnclosedCJKLettersandMonths::ParenthesizedIdeographRepresent => '㈹',
            EnclosedCJKLettersandMonths::ParenthesizedIdeographCall => '㈺',
            EnclosedCJKLettersandMonths::ParenthesizedIdeographStudy => '㈻',
            EnclosedCJKLettersandMonths::ParenthesizedIdeographSupervise => '㈼',
            EnclosedCJKLettersandMonths::ParenthesizedIdeographEnterprise => '㈽',
            EnclosedCJKLettersandMonths::ParenthesizedIdeographResource => '㈾',
            EnclosedCJKLettersandMonths::ParenthesizedIdeographAlliance => '㈿',
            EnclosedCJKLettersandMonths::ParenthesizedIdeographFestival => '㉀',
            EnclosedCJKLettersandMonths::ParenthesizedIdeographRest => '㉁',
            EnclosedCJKLettersandMonths::ParenthesizedIdeographSelf => '㉂',
            EnclosedCJKLettersandMonths::ParenthesizedIdeographReach => '㉃',
            EnclosedCJKLettersandMonths::CircledIdeographQuestion => '㉄',
            EnclosedCJKLettersandMonths::CircledIdeographKindergarten => '㉅',
            EnclosedCJKLettersandMonths::CircledIdeographSchool => '㉆',
            EnclosedCJKLettersandMonths::CircledIdeographKoto => '㉇',
            EnclosedCJKLettersandMonths::CircledNumberTenOnBlackSquare => '㉈',
            EnclosedCJKLettersandMonths::CircledNumberTwentyOnBlackSquare => '㉉',
            EnclosedCJKLettersandMonths::CircledNumberThirtyOnBlackSquare => '㉊',
            EnclosedCJKLettersandMonths::CircledNumberFortyOnBlackSquare => '㉋',
            EnclosedCJKLettersandMonths::CircledNumberFiftyOnBlackSquare => '㉌',
            EnclosedCJKLettersandMonths::CircledNumberSixtyOnBlackSquare => '㉍',
            EnclosedCJKLettersandMonths::CircledNumberSeventyOnBlackSquare => '㉎',
            EnclosedCJKLettersandMonths::CircledNumberEightyOnBlackSquare => '㉏',
            EnclosedCJKLettersandMonths::PartnershipSign => '㉐',
            EnclosedCJKLettersandMonths::CircledNumberTwentyOne => '㉑',
            EnclosedCJKLettersandMonths::CircledNumberTwentyTwo => '㉒',
            EnclosedCJKLettersandMonths::CircledNumberTwentyThree => '㉓',
            EnclosedCJKLettersandMonths::CircledNumberTwentyFour => '㉔',
            EnclosedCJKLettersandMonths::CircledNumberTwentyFive => '㉕',
            EnclosedCJKLettersandMonths::CircledNumberTwentySix => '㉖',
            EnclosedCJKLettersandMonths::CircledNumberTwentySeven => '㉗',
            EnclosedCJKLettersandMonths::CircledNumberTwentyEight => '㉘',
            EnclosedCJKLettersandMonths::CircledNumberTwentyNine => '㉙',
            EnclosedCJKLettersandMonths::CircledNumberThirty => '㉚',
            EnclosedCJKLettersandMonths::CircledNumberThirtyOne => '㉛',
            EnclosedCJKLettersandMonths::CircledNumberThirtyTwo => '㉜',
            EnclosedCJKLettersandMonths::CircledNumberThirtyThree => '㉝',
            EnclosedCJKLettersandMonths::CircledNumberThirtyFour => '㉞',
            EnclosedCJKLettersandMonths::CircledNumberThirtyFive => '㉟',
            EnclosedCJKLettersandMonths::CircledHangulKiyeok => '㉠',
            EnclosedCJKLettersandMonths::CircledHangulNieun => '㉡',
            EnclosedCJKLettersandMonths::CircledHangulTikeut => '㉢',
            EnclosedCJKLettersandMonths::CircledHangulRieul => '㉣',
            EnclosedCJKLettersandMonths::CircledHangulMieum => '㉤',
            EnclosedCJKLettersandMonths::CircledHangulPieup => '㉥',
            EnclosedCJKLettersandMonths::CircledHangulSios => '㉦',
            EnclosedCJKLettersandMonths::CircledHangulIeung => '㉧',
            EnclosedCJKLettersandMonths::CircledHangulCieuc => '㉨',
            EnclosedCJKLettersandMonths::CircledHangulChieuch => '㉩',
            EnclosedCJKLettersandMonths::CircledHangulKhieukh => '㉪',
            EnclosedCJKLettersandMonths::CircledHangulThieuth => '㉫',
            EnclosedCJKLettersandMonths::CircledHangulPhieuph => '㉬',
            EnclosedCJKLettersandMonths::CircledHangulHieuh => '㉭',
            EnclosedCJKLettersandMonths::CircledHangulKiyeokA => '㉮',
            EnclosedCJKLettersandMonths::CircledHangulNieunA => '㉯',
            EnclosedCJKLettersandMonths::CircledHangulTikeutA => '㉰',
            EnclosedCJKLettersandMonths::CircledHangulRieulA => '㉱',
            EnclosedCJKLettersandMonths::CircledHangulMieumA => '㉲',
            EnclosedCJKLettersandMonths::CircledHangulPieupA => '㉳',
            EnclosedCJKLettersandMonths::CircledHangulSiosA => '㉴',
            EnclosedCJKLettersandMonths::CircledHangulIeungA => '㉵',
            EnclosedCJKLettersandMonths::CircledHangulCieucA => '㉶',
            EnclosedCJKLettersandMonths::CircledHangulChieuchA => '㉷',
            EnclosedCJKLettersandMonths::CircledHangulKhieukhA => '㉸',
            EnclosedCJKLettersandMonths::CircledHangulThieuthA => '㉹',
            EnclosedCJKLettersandMonths::CircledHangulPhieuphA => '㉺',
            EnclosedCJKLettersandMonths::CircledHangulHieuhA => '㉻',
            EnclosedCJKLettersandMonths::CircledKoreanCharacterChamko => '㉼',
            EnclosedCJKLettersandMonths::CircledKoreanCharacterJueui => '㉽',
            EnclosedCJKLettersandMonths::CircledHangulIeungU => '㉾',
            EnclosedCJKLettersandMonths::KoreanStandardSymbol => '㉿',
            EnclosedCJKLettersandMonths::CircledIdeographOne => '㊀',
            EnclosedCJKLettersandMonths::CircledIdeographTwo => '㊁',
            EnclosedCJKLettersandMonths::CircledIdeographThree => '㊂',
            EnclosedCJKLettersandMonths::CircledIdeographFour => '㊃',
            EnclosedCJKLettersandMonths::CircledIdeographFive => '㊄',
            EnclosedCJKLettersandMonths::CircledIdeographSix => '㊅',
            EnclosedCJKLettersandMonths::CircledIdeographSeven => '㊆',
            EnclosedCJKLettersandMonths::CircledIdeographEight => '㊇',
            EnclosedCJKLettersandMonths::CircledIdeographNine => '㊈',
            EnclosedCJKLettersandMonths::CircledIdeographTen => '㊉',
            EnclosedCJKLettersandMonths::CircledIdeographMoon => '㊊',
            EnclosedCJKLettersandMonths::CircledIdeographFire => '㊋',
            EnclosedCJKLettersandMonths::CircledIdeographWater => '㊌',
            EnclosedCJKLettersandMonths::CircledIdeographWood => '㊍',
            EnclosedCJKLettersandMonths::CircledIdeographMetal => '㊎',
            EnclosedCJKLettersandMonths::CircledIdeographEarth => '㊏',
            EnclosedCJKLettersandMonths::CircledIdeographSun => '㊐',
            EnclosedCJKLettersandMonths::CircledIdeographStock => '㊑',
            EnclosedCJKLettersandMonths::CircledIdeographHave => '㊒',
            EnclosedCJKLettersandMonths::CircledIdeographSociety => '㊓',
            EnclosedCJKLettersandMonths::CircledIdeographName => '㊔',
            EnclosedCJKLettersandMonths::CircledIdeographSpecial => '㊕',
            EnclosedCJKLettersandMonths::CircledIdeographFinancial => '㊖',
            EnclosedCJKLettersandMonths::CircledIdeographCongratulation => '㊗',
            EnclosedCJKLettersandMonths::CircledIdeographLabor => '㊘',
            EnclosedCJKLettersandMonths::CircledIdeographSecret => '㊙',
            EnclosedCJKLettersandMonths::CircledIdeographMale => '㊚',
            EnclosedCJKLettersandMonths::CircledIdeographFemale => '㊛',
            EnclosedCJKLettersandMonths::CircledIdeographSuitable => '㊜',
            EnclosedCJKLettersandMonths::CircledIdeographExcellent => '㊝',
            EnclosedCJKLettersandMonths::CircledIdeographPrint => '㊞',
            EnclosedCJKLettersandMonths::CircledIdeographAttention => '㊟',
            EnclosedCJKLettersandMonths::CircledIdeographItem => '㊠',
            EnclosedCJKLettersandMonths::CircledIdeographRest => '㊡',
            EnclosedCJKLettersandMonths::CircledIdeographCopy => '㊢',
            EnclosedCJKLettersandMonths::CircledIdeographCorrect => '㊣',
            EnclosedCJKLettersandMonths::CircledIdeographHigh => '㊤',
            EnclosedCJKLettersandMonths::CircledIdeographCentre => '㊥',
            EnclosedCJKLettersandMonths::CircledIdeographLow => '㊦',
            EnclosedCJKLettersandMonths::CircledIdeographLeft => '㊧',
            EnclosedCJKLettersandMonths::CircledIdeographRight => '㊨',
            EnclosedCJKLettersandMonths::CircledIdeographMedicine => '㊩',
            EnclosedCJKLettersandMonths::CircledIdeographReligion => '㊪',
            EnclosedCJKLettersandMonths::CircledIdeographStudy => '㊫',
            EnclosedCJKLettersandMonths::CircledIdeographSupervise => '㊬',
            EnclosedCJKLettersandMonths::CircledIdeographEnterprise => '㊭',
            EnclosedCJKLettersandMonths::CircledIdeographResource => '㊮',
            EnclosedCJKLettersandMonths::CircledIdeographAlliance => '㊯',
            EnclosedCJKLettersandMonths::CircledIdeographNight => '㊰',
            EnclosedCJKLettersandMonths::CircledNumberThirtySix => '㊱',
            EnclosedCJKLettersandMonths::CircledNumberThirtySeven => '㊲',
            EnclosedCJKLettersandMonths::CircledNumberThirtyEight => '㊳',
            EnclosedCJKLettersandMonths::CircledNumberThirtyNine => '㊴',
            EnclosedCJKLettersandMonths::CircledNumberForty => '㊵',
            EnclosedCJKLettersandMonths::CircledNumberFortyOne => '㊶',
            EnclosedCJKLettersandMonths::CircledNumberFortyTwo => '㊷',
            EnclosedCJKLettersandMonths::CircledNumberFortyThree => '㊸',
            EnclosedCJKLettersandMonths::CircledNumberFortyFour => '㊹',
            EnclosedCJKLettersandMonths::CircledNumberFortyFive => '㊺',
            EnclosedCJKLettersandMonths::CircledNumberFortySix => '㊻',
            EnclosedCJKLettersandMonths::CircledNumberFortySeven => '㊼',
            EnclosedCJKLettersandMonths::CircledNumberFortyEight => '㊽',
            EnclosedCJKLettersandMonths::CircledNumberFortyNine => '㊾',
            EnclosedCJKLettersandMonths::CircledNumberFifty => '㊿',
            EnclosedCJKLettersandMonths::IdeographicTelegraphSymbolForJanuary => '㋀',
            EnclosedCJKLettersandMonths::IdeographicTelegraphSymbolForFebruary => '㋁',
            EnclosedCJKLettersandMonths::IdeographicTelegraphSymbolForMarch => '㋂',
            EnclosedCJKLettersandMonths::IdeographicTelegraphSymbolForApril => '㋃',
            EnclosedCJKLettersandMonths::IdeographicTelegraphSymbolForMay => '㋄',
            EnclosedCJKLettersandMonths::IdeographicTelegraphSymbolForJune => '㋅',
            EnclosedCJKLettersandMonths::IdeographicTelegraphSymbolForJuly => '㋆',
            EnclosedCJKLettersandMonths::IdeographicTelegraphSymbolForAugust => '㋇',
            EnclosedCJKLettersandMonths::IdeographicTelegraphSymbolForSeptember => '㋈',
            EnclosedCJKLettersandMonths::IdeographicTelegraphSymbolForOctober => '㋉',
            EnclosedCJKLettersandMonths::IdeographicTelegraphSymbolForNovember => '㋊',
            EnclosedCJKLettersandMonths::IdeographicTelegraphSymbolForDecember => '㋋',
            EnclosedCJKLettersandMonths::SquareHg => '㋌',
            EnclosedCJKLettersandMonths::SquareErg => '㋍',
            EnclosedCJKLettersandMonths::SquareEv => '㋎',
            EnclosedCJKLettersandMonths::LimitedLiabilitySign => '㋏',
            EnclosedCJKLettersandMonths::CircledKatakanaA => '㋐',
            EnclosedCJKLettersandMonths::CircledKatakanaI => '㋑',
            EnclosedCJKLettersandMonths::CircledKatakanaU => '㋒',
            EnclosedCJKLettersandMonths::CircledKatakanaE => '㋓',
            EnclosedCJKLettersandMonths::CircledKatakanaO => '㋔',
            EnclosedCJKLettersandMonths::CircledKatakanaKa => '㋕',
            EnclosedCJKLettersandMonths::CircledKatakanaKi => '㋖',
            EnclosedCJKLettersandMonths::CircledKatakanaKu => '㋗',
            EnclosedCJKLettersandMonths::CircledKatakanaKe => '㋘',
            EnclosedCJKLettersandMonths::CircledKatakanaKo => '㋙',
            EnclosedCJKLettersandMonths::CircledKatakanaSa => '㋚',
            EnclosedCJKLettersandMonths::CircledKatakanaSi => '㋛',
            EnclosedCJKLettersandMonths::CircledKatakanaSu => '㋜',
            EnclosedCJKLettersandMonths::CircledKatakanaSe => '㋝',
            EnclosedCJKLettersandMonths::CircledKatakanaSo => '㋞',
            EnclosedCJKLettersandMonths::CircledKatakanaTa => '㋟',
            EnclosedCJKLettersandMonths::CircledKatakanaTi => '㋠',
            EnclosedCJKLettersandMonths::CircledKatakanaTu => '㋡',
            EnclosedCJKLettersandMonths::CircledKatakanaTe => '㋢',
            EnclosedCJKLettersandMonths::CircledKatakanaTo => '㋣',
            EnclosedCJKLettersandMonths::CircledKatakanaNa => '㋤',
            EnclosedCJKLettersandMonths::CircledKatakanaNi => '㋥',
            EnclosedCJKLettersandMonths::CircledKatakanaNu => '㋦',
            EnclosedCJKLettersandMonths::CircledKatakanaNe => '㋧',
            EnclosedCJKLettersandMonths::CircledKatakanaNo => '㋨',
            EnclosedCJKLettersandMonths::CircledKatakanaHa => '㋩',
            EnclosedCJKLettersandMonths::CircledKatakanaHi => '㋪',
            EnclosedCJKLettersandMonths::CircledKatakanaHu => '㋫',
            EnclosedCJKLettersandMonths::CircledKatakanaHe => '㋬',
            EnclosedCJKLettersandMonths::CircledKatakanaHo => '㋭',
            EnclosedCJKLettersandMonths::CircledKatakanaMa => '㋮',
            EnclosedCJKLettersandMonths::CircledKatakanaMi => '㋯',
            EnclosedCJKLettersandMonths::CircledKatakanaMu => '㋰',
            EnclosedCJKLettersandMonths::CircledKatakanaMe => '㋱',
            EnclosedCJKLettersandMonths::CircledKatakanaMo => '㋲',
            EnclosedCJKLettersandMonths::CircledKatakanaYa => '㋳',
            EnclosedCJKLettersandMonths::CircledKatakanaYu => '㋴',
            EnclosedCJKLettersandMonths::CircledKatakanaYo => '㋵',
            EnclosedCJKLettersandMonths::CircledKatakanaRa => '㋶',
            EnclosedCJKLettersandMonths::CircledKatakanaRi => '㋷',
            EnclosedCJKLettersandMonths::CircledKatakanaRu => '㋸',
            EnclosedCJKLettersandMonths::CircledKatakanaRe => '㋹',
            EnclosedCJKLettersandMonths::CircledKatakanaRo => '㋺',
            EnclosedCJKLettersandMonths::CircledKatakanaWa => '㋻',
            EnclosedCJKLettersandMonths::CircledKatakanaWi => '㋼',
            EnclosedCJKLettersandMonths::CircledKatakanaWe => '㋽',
            EnclosedCJKLettersandMonths::CircledKatakanaWo => '㋾',
        }
    }
}

impl std::convert::TryFrom<char> for EnclosedCJKLettersandMonths {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '㈀' => Ok(EnclosedCJKLettersandMonths::ParenthesizedHangulKiyeok),
            '㈁' => Ok(EnclosedCJKLettersandMonths::ParenthesizedHangulNieun),
            '㈂' => Ok(EnclosedCJKLettersandMonths::ParenthesizedHangulTikeut),
            '㈃' => Ok(EnclosedCJKLettersandMonths::ParenthesizedHangulRieul),
            '㈄' => Ok(EnclosedCJKLettersandMonths::ParenthesizedHangulMieum),
            '㈅' => Ok(EnclosedCJKLettersandMonths::ParenthesizedHangulPieup),
            '㈆' => Ok(EnclosedCJKLettersandMonths::ParenthesizedHangulSios),
            '㈇' => Ok(EnclosedCJKLettersandMonths::ParenthesizedHangulIeung),
            '㈈' => Ok(EnclosedCJKLettersandMonths::ParenthesizedHangulCieuc),
            '㈉' => Ok(EnclosedCJKLettersandMonths::ParenthesizedHangulChieuch),
            '㈊' => Ok(EnclosedCJKLettersandMonths::ParenthesizedHangulKhieukh),
            '㈋' => Ok(EnclosedCJKLettersandMonths::ParenthesizedHangulThieuth),
            '㈌' => Ok(EnclosedCJKLettersandMonths::ParenthesizedHangulPhieuph),
            '㈍' => Ok(EnclosedCJKLettersandMonths::ParenthesizedHangulHieuh),
            '㈎' => Ok(EnclosedCJKLettersandMonths::ParenthesizedHangulKiyeokA),
            '㈏' => Ok(EnclosedCJKLettersandMonths::ParenthesizedHangulNieunA),
            '㈐' => Ok(EnclosedCJKLettersandMonths::ParenthesizedHangulTikeutA),
            '㈑' => Ok(EnclosedCJKLettersandMonths::ParenthesizedHangulRieulA),
            '㈒' => Ok(EnclosedCJKLettersandMonths::ParenthesizedHangulMieumA),
            '㈓' => Ok(EnclosedCJKLettersandMonths::ParenthesizedHangulPieupA),
            '㈔' => Ok(EnclosedCJKLettersandMonths::ParenthesizedHangulSiosA),
            '㈕' => Ok(EnclosedCJKLettersandMonths::ParenthesizedHangulIeungA),
            '㈖' => Ok(EnclosedCJKLettersandMonths::ParenthesizedHangulCieucA),
            '㈗' => Ok(EnclosedCJKLettersandMonths::ParenthesizedHangulChieuchA),
            '㈘' => Ok(EnclosedCJKLettersandMonths::ParenthesizedHangulKhieukhA),
            '㈙' => Ok(EnclosedCJKLettersandMonths::ParenthesizedHangulThieuthA),
            '㈚' => Ok(EnclosedCJKLettersandMonths::ParenthesizedHangulPhieuphA),
            '㈛' => Ok(EnclosedCJKLettersandMonths::ParenthesizedHangulHieuhA),
            '㈜' => Ok(EnclosedCJKLettersandMonths::ParenthesizedHangulCieucU),
            '㈝' => Ok(EnclosedCJKLettersandMonths::ParenthesizedKoreanCharacterOjeon),
            '㈞' => Ok(EnclosedCJKLettersandMonths::ParenthesizedKoreanCharacterOHu),
            '㈠' => Ok(EnclosedCJKLettersandMonths::ParenthesizedIdeographOne),
            '㈡' => Ok(EnclosedCJKLettersandMonths::ParenthesizedIdeographTwo),
            '㈢' => Ok(EnclosedCJKLettersandMonths::ParenthesizedIdeographThree),
            '㈣' => Ok(EnclosedCJKLettersandMonths::ParenthesizedIdeographFour),
            '㈤' => Ok(EnclosedCJKLettersandMonths::ParenthesizedIdeographFive),
            '㈥' => Ok(EnclosedCJKLettersandMonths::ParenthesizedIdeographSix),
            '㈦' => Ok(EnclosedCJKLettersandMonths::ParenthesizedIdeographSeven),
            '㈧' => Ok(EnclosedCJKLettersandMonths::ParenthesizedIdeographEight),
            '㈨' => Ok(EnclosedCJKLettersandMonths::ParenthesizedIdeographNine),
            '㈩' => Ok(EnclosedCJKLettersandMonths::ParenthesizedIdeographTen),
            '㈪' => Ok(EnclosedCJKLettersandMonths::ParenthesizedIdeographMoon),
            '㈫' => Ok(EnclosedCJKLettersandMonths::ParenthesizedIdeographFire),
            '㈬' => Ok(EnclosedCJKLettersandMonths::ParenthesizedIdeographWater),
            '㈭' => Ok(EnclosedCJKLettersandMonths::ParenthesizedIdeographWood),
            '㈮' => Ok(EnclosedCJKLettersandMonths::ParenthesizedIdeographMetal),
            '㈯' => Ok(EnclosedCJKLettersandMonths::ParenthesizedIdeographEarth),
            '㈰' => Ok(EnclosedCJKLettersandMonths::ParenthesizedIdeographSun),
            '㈱' => Ok(EnclosedCJKLettersandMonths::ParenthesizedIdeographStock),
            '㈲' => Ok(EnclosedCJKLettersandMonths::ParenthesizedIdeographHave),
            '㈳' => Ok(EnclosedCJKLettersandMonths::ParenthesizedIdeographSociety),
            '㈴' => Ok(EnclosedCJKLettersandMonths::ParenthesizedIdeographName),
            '㈵' => Ok(EnclosedCJKLettersandMonths::ParenthesizedIdeographSpecial),
            '㈶' => Ok(EnclosedCJKLettersandMonths::ParenthesizedIdeographFinancial),
            '㈷' => Ok(EnclosedCJKLettersandMonths::ParenthesizedIdeographCongratulation),
            '㈸' => Ok(EnclosedCJKLettersandMonths::ParenthesizedIdeographLabor),
            '㈹' => Ok(EnclosedCJKLettersandMonths::ParenthesizedIdeographRepresent),
            '㈺' => Ok(EnclosedCJKLettersandMonths::ParenthesizedIdeographCall),
            '㈻' => Ok(EnclosedCJKLettersandMonths::ParenthesizedIdeographStudy),
            '㈼' => Ok(EnclosedCJKLettersandMonths::ParenthesizedIdeographSupervise),
            '㈽' => Ok(EnclosedCJKLettersandMonths::ParenthesizedIdeographEnterprise),
            '㈾' => Ok(EnclosedCJKLettersandMonths::ParenthesizedIdeographResource),
            '㈿' => Ok(EnclosedCJKLettersandMonths::ParenthesizedIdeographAlliance),
            '㉀' => Ok(EnclosedCJKLettersandMonths::ParenthesizedIdeographFestival),
            '㉁' => Ok(EnclosedCJKLettersandMonths::ParenthesizedIdeographRest),
            '㉂' => Ok(EnclosedCJKLettersandMonths::ParenthesizedIdeographSelf),
            '㉃' => Ok(EnclosedCJKLettersandMonths::ParenthesizedIdeographReach),
            '㉄' => Ok(EnclosedCJKLettersandMonths::CircledIdeographQuestion),
            '㉅' => Ok(EnclosedCJKLettersandMonths::CircledIdeographKindergarten),
            '㉆' => Ok(EnclosedCJKLettersandMonths::CircledIdeographSchool),
            '㉇' => Ok(EnclosedCJKLettersandMonths::CircledIdeographKoto),
            '㉈' => Ok(EnclosedCJKLettersandMonths::CircledNumberTenOnBlackSquare),
            '㉉' => Ok(EnclosedCJKLettersandMonths::CircledNumberTwentyOnBlackSquare),
            '㉊' => Ok(EnclosedCJKLettersandMonths::CircledNumberThirtyOnBlackSquare),
            '㉋' => Ok(EnclosedCJKLettersandMonths::CircledNumberFortyOnBlackSquare),
            '㉌' => Ok(EnclosedCJKLettersandMonths::CircledNumberFiftyOnBlackSquare),
            '㉍' => Ok(EnclosedCJKLettersandMonths::CircledNumberSixtyOnBlackSquare),
            '㉎' => Ok(EnclosedCJKLettersandMonths::CircledNumberSeventyOnBlackSquare),
            '㉏' => Ok(EnclosedCJKLettersandMonths::CircledNumberEightyOnBlackSquare),
            '㉐' => Ok(EnclosedCJKLettersandMonths::PartnershipSign),
            '㉑' => Ok(EnclosedCJKLettersandMonths::CircledNumberTwentyOne),
            '㉒' => Ok(EnclosedCJKLettersandMonths::CircledNumberTwentyTwo),
            '㉓' => Ok(EnclosedCJKLettersandMonths::CircledNumberTwentyThree),
            '㉔' => Ok(EnclosedCJKLettersandMonths::CircledNumberTwentyFour),
            '㉕' => Ok(EnclosedCJKLettersandMonths::CircledNumberTwentyFive),
            '㉖' => Ok(EnclosedCJKLettersandMonths::CircledNumberTwentySix),
            '㉗' => Ok(EnclosedCJKLettersandMonths::CircledNumberTwentySeven),
            '㉘' => Ok(EnclosedCJKLettersandMonths::CircledNumberTwentyEight),
            '㉙' => Ok(EnclosedCJKLettersandMonths::CircledNumberTwentyNine),
            '㉚' => Ok(EnclosedCJKLettersandMonths::CircledNumberThirty),
            '㉛' => Ok(EnclosedCJKLettersandMonths::CircledNumberThirtyOne),
            '㉜' => Ok(EnclosedCJKLettersandMonths::CircledNumberThirtyTwo),
            '㉝' => Ok(EnclosedCJKLettersandMonths::CircledNumberThirtyThree),
            '㉞' => Ok(EnclosedCJKLettersandMonths::CircledNumberThirtyFour),
            '㉟' => Ok(EnclosedCJKLettersandMonths::CircledNumberThirtyFive),
            '㉠' => Ok(EnclosedCJKLettersandMonths::CircledHangulKiyeok),
            '㉡' => Ok(EnclosedCJKLettersandMonths::CircledHangulNieun),
            '㉢' => Ok(EnclosedCJKLettersandMonths::CircledHangulTikeut),
            '㉣' => Ok(EnclosedCJKLettersandMonths::CircledHangulRieul),
            '㉤' => Ok(EnclosedCJKLettersandMonths::CircledHangulMieum),
            '㉥' => Ok(EnclosedCJKLettersandMonths::CircledHangulPieup),
            '㉦' => Ok(EnclosedCJKLettersandMonths::CircledHangulSios),
            '㉧' => Ok(EnclosedCJKLettersandMonths::CircledHangulIeung),
            '㉨' => Ok(EnclosedCJKLettersandMonths::CircledHangulCieuc),
            '㉩' => Ok(EnclosedCJKLettersandMonths::CircledHangulChieuch),
            '㉪' => Ok(EnclosedCJKLettersandMonths::CircledHangulKhieukh),
            '㉫' => Ok(EnclosedCJKLettersandMonths::CircledHangulThieuth),
            '㉬' => Ok(EnclosedCJKLettersandMonths::CircledHangulPhieuph),
            '㉭' => Ok(EnclosedCJKLettersandMonths::CircledHangulHieuh),
            '㉮' => Ok(EnclosedCJKLettersandMonths::CircledHangulKiyeokA),
            '㉯' => Ok(EnclosedCJKLettersandMonths::CircledHangulNieunA),
            '㉰' => Ok(EnclosedCJKLettersandMonths::CircledHangulTikeutA),
            '㉱' => Ok(EnclosedCJKLettersandMonths::CircledHangulRieulA),
            '㉲' => Ok(EnclosedCJKLettersandMonths::CircledHangulMieumA),
            '㉳' => Ok(EnclosedCJKLettersandMonths::CircledHangulPieupA),
            '㉴' => Ok(EnclosedCJKLettersandMonths::CircledHangulSiosA),
            '㉵' => Ok(EnclosedCJKLettersandMonths::CircledHangulIeungA),
            '㉶' => Ok(EnclosedCJKLettersandMonths::CircledHangulCieucA),
            '㉷' => Ok(EnclosedCJKLettersandMonths::CircledHangulChieuchA),
            '㉸' => Ok(EnclosedCJKLettersandMonths::CircledHangulKhieukhA),
            '㉹' => Ok(EnclosedCJKLettersandMonths::CircledHangulThieuthA),
            '㉺' => Ok(EnclosedCJKLettersandMonths::CircledHangulPhieuphA),
            '㉻' => Ok(EnclosedCJKLettersandMonths::CircledHangulHieuhA),
            '㉼' => Ok(EnclosedCJKLettersandMonths::CircledKoreanCharacterChamko),
            '㉽' => Ok(EnclosedCJKLettersandMonths::CircledKoreanCharacterJueui),
            '㉾' => Ok(EnclosedCJKLettersandMonths::CircledHangulIeungU),
            '㉿' => Ok(EnclosedCJKLettersandMonths::KoreanStandardSymbol),
            '㊀' => Ok(EnclosedCJKLettersandMonths::CircledIdeographOne),
            '㊁' => Ok(EnclosedCJKLettersandMonths::CircledIdeographTwo),
            '㊂' => Ok(EnclosedCJKLettersandMonths::CircledIdeographThree),
            '㊃' => Ok(EnclosedCJKLettersandMonths::CircledIdeographFour),
            '㊄' => Ok(EnclosedCJKLettersandMonths::CircledIdeographFive),
            '㊅' => Ok(EnclosedCJKLettersandMonths::CircledIdeographSix),
            '㊆' => Ok(EnclosedCJKLettersandMonths::CircledIdeographSeven),
            '㊇' => Ok(EnclosedCJKLettersandMonths::CircledIdeographEight),
            '㊈' => Ok(EnclosedCJKLettersandMonths::CircledIdeographNine),
            '㊉' => Ok(EnclosedCJKLettersandMonths::CircledIdeographTen),
            '㊊' => Ok(EnclosedCJKLettersandMonths::CircledIdeographMoon),
            '㊋' => Ok(EnclosedCJKLettersandMonths::CircledIdeographFire),
            '㊌' => Ok(EnclosedCJKLettersandMonths::CircledIdeographWater),
            '㊍' => Ok(EnclosedCJKLettersandMonths::CircledIdeographWood),
            '㊎' => Ok(EnclosedCJKLettersandMonths::CircledIdeographMetal),
            '㊏' => Ok(EnclosedCJKLettersandMonths::CircledIdeographEarth),
            '㊐' => Ok(EnclosedCJKLettersandMonths::CircledIdeographSun),
            '㊑' => Ok(EnclosedCJKLettersandMonths::CircledIdeographStock),
            '㊒' => Ok(EnclosedCJKLettersandMonths::CircledIdeographHave),
            '㊓' => Ok(EnclosedCJKLettersandMonths::CircledIdeographSociety),
            '㊔' => Ok(EnclosedCJKLettersandMonths::CircledIdeographName),
            '㊕' => Ok(EnclosedCJKLettersandMonths::CircledIdeographSpecial),
            '㊖' => Ok(EnclosedCJKLettersandMonths::CircledIdeographFinancial),
            '㊗' => Ok(EnclosedCJKLettersandMonths::CircledIdeographCongratulation),
            '㊘' => Ok(EnclosedCJKLettersandMonths::CircledIdeographLabor),
            '㊙' => Ok(EnclosedCJKLettersandMonths::CircledIdeographSecret),
            '㊚' => Ok(EnclosedCJKLettersandMonths::CircledIdeographMale),
            '㊛' => Ok(EnclosedCJKLettersandMonths::CircledIdeographFemale),
            '㊜' => Ok(EnclosedCJKLettersandMonths::CircledIdeographSuitable),
            '㊝' => Ok(EnclosedCJKLettersandMonths::CircledIdeographExcellent),
            '㊞' => Ok(EnclosedCJKLettersandMonths::CircledIdeographPrint),
            '㊟' => Ok(EnclosedCJKLettersandMonths::CircledIdeographAttention),
            '㊠' => Ok(EnclosedCJKLettersandMonths::CircledIdeographItem),
            '㊡' => Ok(EnclosedCJKLettersandMonths::CircledIdeographRest),
            '㊢' => Ok(EnclosedCJKLettersandMonths::CircledIdeographCopy),
            '㊣' => Ok(EnclosedCJKLettersandMonths::CircledIdeographCorrect),
            '㊤' => Ok(EnclosedCJKLettersandMonths::CircledIdeographHigh),
            '㊥' => Ok(EnclosedCJKLettersandMonths::CircledIdeographCentre),
            '㊦' => Ok(EnclosedCJKLettersandMonths::CircledIdeographLow),
            '㊧' => Ok(EnclosedCJKLettersandMonths::CircledIdeographLeft),
            '㊨' => Ok(EnclosedCJKLettersandMonths::CircledIdeographRight),
            '㊩' => Ok(EnclosedCJKLettersandMonths::CircledIdeographMedicine),
            '㊪' => Ok(EnclosedCJKLettersandMonths::CircledIdeographReligion),
            '㊫' => Ok(EnclosedCJKLettersandMonths::CircledIdeographStudy),
            '㊬' => Ok(EnclosedCJKLettersandMonths::CircledIdeographSupervise),
            '㊭' => Ok(EnclosedCJKLettersandMonths::CircledIdeographEnterprise),
            '㊮' => Ok(EnclosedCJKLettersandMonths::CircledIdeographResource),
            '㊯' => Ok(EnclosedCJKLettersandMonths::CircledIdeographAlliance),
            '㊰' => Ok(EnclosedCJKLettersandMonths::CircledIdeographNight),
            '㊱' => Ok(EnclosedCJKLettersandMonths::CircledNumberThirtySix),
            '㊲' => Ok(EnclosedCJKLettersandMonths::CircledNumberThirtySeven),
            '㊳' => Ok(EnclosedCJKLettersandMonths::CircledNumberThirtyEight),
            '㊴' => Ok(EnclosedCJKLettersandMonths::CircledNumberThirtyNine),
            '㊵' => Ok(EnclosedCJKLettersandMonths::CircledNumberForty),
            '㊶' => Ok(EnclosedCJKLettersandMonths::CircledNumberFortyOne),
            '㊷' => Ok(EnclosedCJKLettersandMonths::CircledNumberFortyTwo),
            '㊸' => Ok(EnclosedCJKLettersandMonths::CircledNumberFortyThree),
            '㊹' => Ok(EnclosedCJKLettersandMonths::CircledNumberFortyFour),
            '㊺' => Ok(EnclosedCJKLettersandMonths::CircledNumberFortyFive),
            '㊻' => Ok(EnclosedCJKLettersandMonths::CircledNumberFortySix),
            '㊼' => Ok(EnclosedCJKLettersandMonths::CircledNumberFortySeven),
            '㊽' => Ok(EnclosedCJKLettersandMonths::CircledNumberFortyEight),
            '㊾' => Ok(EnclosedCJKLettersandMonths::CircledNumberFortyNine),
            '㊿' => Ok(EnclosedCJKLettersandMonths::CircledNumberFifty),
            '㋀' => Ok(EnclosedCJKLettersandMonths::IdeographicTelegraphSymbolForJanuary),
            '㋁' => Ok(EnclosedCJKLettersandMonths::IdeographicTelegraphSymbolForFebruary),
            '㋂' => Ok(EnclosedCJKLettersandMonths::IdeographicTelegraphSymbolForMarch),
            '㋃' => Ok(EnclosedCJKLettersandMonths::IdeographicTelegraphSymbolForApril),
            '㋄' => Ok(EnclosedCJKLettersandMonths::IdeographicTelegraphSymbolForMay),
            '㋅' => Ok(EnclosedCJKLettersandMonths::IdeographicTelegraphSymbolForJune),
            '㋆' => Ok(EnclosedCJKLettersandMonths::IdeographicTelegraphSymbolForJuly),
            '㋇' => Ok(EnclosedCJKLettersandMonths::IdeographicTelegraphSymbolForAugust),
            '㋈' => Ok(EnclosedCJKLettersandMonths::IdeographicTelegraphSymbolForSeptember),
            '㋉' => Ok(EnclosedCJKLettersandMonths::IdeographicTelegraphSymbolForOctober),
            '㋊' => Ok(EnclosedCJKLettersandMonths::IdeographicTelegraphSymbolForNovember),
            '㋋' => Ok(EnclosedCJKLettersandMonths::IdeographicTelegraphSymbolForDecember),
            '㋌' => Ok(EnclosedCJKLettersandMonths::SquareHg),
            '㋍' => Ok(EnclosedCJKLettersandMonths::SquareErg),
            '㋎' => Ok(EnclosedCJKLettersandMonths::SquareEv),
            '㋏' => Ok(EnclosedCJKLettersandMonths::LimitedLiabilitySign),
            '㋐' => Ok(EnclosedCJKLettersandMonths::CircledKatakanaA),
            '㋑' => Ok(EnclosedCJKLettersandMonths::CircledKatakanaI),
            '㋒' => Ok(EnclosedCJKLettersandMonths::CircledKatakanaU),
            '㋓' => Ok(EnclosedCJKLettersandMonths::CircledKatakanaE),
            '㋔' => Ok(EnclosedCJKLettersandMonths::CircledKatakanaO),
            '㋕' => Ok(EnclosedCJKLettersandMonths::CircledKatakanaKa),
            '㋖' => Ok(EnclosedCJKLettersandMonths::CircledKatakanaKi),
            '㋗' => Ok(EnclosedCJKLettersandMonths::CircledKatakanaKu),
            '㋘' => Ok(EnclosedCJKLettersandMonths::CircledKatakanaKe),
            '㋙' => Ok(EnclosedCJKLettersandMonths::CircledKatakanaKo),
            '㋚' => Ok(EnclosedCJKLettersandMonths::CircledKatakanaSa),
            '㋛' => Ok(EnclosedCJKLettersandMonths::CircledKatakanaSi),
            '㋜' => Ok(EnclosedCJKLettersandMonths::CircledKatakanaSu),
            '㋝' => Ok(EnclosedCJKLettersandMonths::CircledKatakanaSe),
            '㋞' => Ok(EnclosedCJKLettersandMonths::CircledKatakanaSo),
            '㋟' => Ok(EnclosedCJKLettersandMonths::CircledKatakanaTa),
            '㋠' => Ok(EnclosedCJKLettersandMonths::CircledKatakanaTi),
            '㋡' => Ok(EnclosedCJKLettersandMonths::CircledKatakanaTu),
            '㋢' => Ok(EnclosedCJKLettersandMonths::CircledKatakanaTe),
            '㋣' => Ok(EnclosedCJKLettersandMonths::CircledKatakanaTo),
            '㋤' => Ok(EnclosedCJKLettersandMonths::CircledKatakanaNa),
            '㋥' => Ok(EnclosedCJKLettersandMonths::CircledKatakanaNi),
            '㋦' => Ok(EnclosedCJKLettersandMonths::CircledKatakanaNu),
            '㋧' => Ok(EnclosedCJKLettersandMonths::CircledKatakanaNe),
            '㋨' => Ok(EnclosedCJKLettersandMonths::CircledKatakanaNo),
            '㋩' => Ok(EnclosedCJKLettersandMonths::CircledKatakanaHa),
            '㋪' => Ok(EnclosedCJKLettersandMonths::CircledKatakanaHi),
            '㋫' => Ok(EnclosedCJKLettersandMonths::CircledKatakanaHu),
            '㋬' => Ok(EnclosedCJKLettersandMonths::CircledKatakanaHe),
            '㋭' => Ok(EnclosedCJKLettersandMonths::CircledKatakanaHo),
            '㋮' => Ok(EnclosedCJKLettersandMonths::CircledKatakanaMa),
            '㋯' => Ok(EnclosedCJKLettersandMonths::CircledKatakanaMi),
            '㋰' => Ok(EnclosedCJKLettersandMonths::CircledKatakanaMu),
            '㋱' => Ok(EnclosedCJKLettersandMonths::CircledKatakanaMe),
            '㋲' => Ok(EnclosedCJKLettersandMonths::CircledKatakanaMo),
            '㋳' => Ok(EnclosedCJKLettersandMonths::CircledKatakanaYa),
            '㋴' => Ok(EnclosedCJKLettersandMonths::CircledKatakanaYu),
            '㋵' => Ok(EnclosedCJKLettersandMonths::CircledKatakanaYo),
            '㋶' => Ok(EnclosedCJKLettersandMonths::CircledKatakanaRa),
            '㋷' => Ok(EnclosedCJKLettersandMonths::CircledKatakanaRi),
            '㋸' => Ok(EnclosedCJKLettersandMonths::CircledKatakanaRu),
            '㋹' => Ok(EnclosedCJKLettersandMonths::CircledKatakanaRe),
            '㋺' => Ok(EnclosedCJKLettersandMonths::CircledKatakanaRo),
            '㋻' => Ok(EnclosedCJKLettersandMonths::CircledKatakanaWa),
            '㋼' => Ok(EnclosedCJKLettersandMonths::CircledKatakanaWi),
            '㋽' => Ok(EnclosedCJKLettersandMonths::CircledKatakanaWe),
            '㋾' => Ok(EnclosedCJKLettersandMonths::CircledKatakanaWo),
            _ => Err(()),
        }
    }
}

impl Into<u32> for EnclosedCJKLettersandMonths {
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

impl std::convert::TryFrom<u32> for EnclosedCJKLettersandMonths {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for EnclosedCJKLettersandMonths {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl EnclosedCJKLettersandMonths {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        EnclosedCJKLettersandMonths::ParenthesizedHangulKiyeok
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("EnclosedCJKLettersandMonths{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
