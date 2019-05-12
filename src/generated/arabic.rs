/// A number of constants to give a name to all characters in this block.
mod constants {
    /// \u{600}: '؀'
    pub const NUMBER_SIGN: char = '؀';
    /// \u{601}: '؁'
    pub const SIGN_SANAH: char = '؁';
    /// \u{602}: '؂'
    pub const FOOTNOTE_MARKER: char = '؂';
    /// \u{603}: '؃'
    pub const SIGN_SAFHA: char = '؃';
    /// \u{604}: '؄'
    pub const SIGN_SAMVAT: char = '؄';
    /// \u{605}: '؅'
    pub const NUMBER_MARK_ABOVE: char = '؅';
    /// \u{606}: '؆'
    pub const DASH_INDIC_CUBE_ROOT: char = '؆';
    /// \u{607}: '؇'
    pub const DASH_INDIC_FOURTH_ROOT: char = '؇';
    /// \u{608}: '؈'
    pub const RAY: char = '؈';
    /// \u{609}: '؉'
    pub const DASH_INDIC_PER_MILLE_SIGN: char = '؉';
    /// \u{60a}: '؊'
    pub const DASH_INDIC_PER_TEN_THOUSAND_SIGN: char = '؊';
    /// \u{60b}: '؋'
    pub const AFGHANI_SIGN: char = '؋';
    /// \u{60c}: '،'
    pub const COMMA: char = '،';
    /// \u{60d}: '؍'
    pub const DATE_SEPARATOR: char = '؍';
    /// \u{60e}: '؎'
    pub const POETIC_VERSE_SIGN: char = '؎';
    /// \u{60f}: '؏'
    pub const SIGN_MISRA: char = '؏';
    /// \u{610}: 'ؐ'
    pub const SIGN_SALLALLAHOU_ALAYHE_WASSALLAM: char = 'ؐ';
    /// \u{611}: 'ؑ'
    pub const SIGN_ALAYHE_ASSALLAM: char = 'ؑ';
    /// \u{612}: 'ؒ'
    pub const SIGN_RAHMATULLAH_ALAYHE: char = 'ؒ';
    /// \u{613}: 'ؓ'
    pub const SIGN_RADI_ALLAHOU_ANHU: char = 'ؓ';
    /// \u{614}: 'ؔ'
    pub const SIGN_TAKHALLUS: char = 'ؔ';
    /// \u{615}: 'ؕ'
    pub const SMALL_HIGH_TAH: char = 'ؕ';
    /// \u{616}: 'ؖ'
    pub const SMALL_HIGH_LIGATURE_ALEF_WITH_LAM_WITH_YEH: char = 'ؖ';
    /// \u{617}: 'ؗ'
    pub const SMALL_HIGH_ZAIN: char = 'ؗ';
    /// \u{618}: 'ؘ'
    pub const SMALL_FATHA: char = 'ؘ';
    /// \u{619}: 'ؙ'
    pub const SMALL_DAMMA: char = 'ؙ';
    /// \u{61a}: 'ؚ'
    pub const SMALL_KASRA: char = 'ؚ';
    /// \u{61b}: '؛'
    pub const SEMICOLON: char = '؛';
    /// \u{61c}: '؜'
    pub const LETTER_MARK: char = '؜';
    /// \u{61e}: '؞'
    pub const TRIPLE_DOT_PUNCTUATION_MARK: char = '؞';
    /// \u{61f}: '؟'
    pub const QUESTION_MARK: char = '؟';
    /// \u{620}: 'ؠ'
    pub const LETTER_KASHMIRI_YEH: char = 'ؠ';
    /// \u{621}: 'ء'
    pub const LETTER_HAMZA: char = 'ء';
    /// \u{622}: 'آ'
    pub const LETTER_ALEF_WITH_MADDA_ABOVE: char = 'آ';
    /// \u{623}: 'أ'
    pub const LETTER_ALEF_WITH_HAMZA_ABOVE: char = 'أ';
    /// \u{624}: 'ؤ'
    pub const LETTER_WAW_WITH_HAMZA_ABOVE: char = 'ؤ';
    /// \u{625}: 'إ'
    pub const LETTER_ALEF_WITH_HAMZA_BELOW: char = 'إ';
    /// \u{626}: 'ئ'
    pub const LETTER_YEH_WITH_HAMZA_ABOVE: char = 'ئ';
    /// \u{627}: 'ا'
    pub const LETTER_ALEF: char = 'ا';
    /// \u{628}: 'ب'
    pub const LETTER_BEH: char = 'ب';
    /// \u{629}: 'ة'
    pub const LETTER_TEH_MARBUTA: char = 'ة';
    /// \u{62a}: 'ت'
    pub const LETTER_TEH: char = 'ت';
    /// \u{62b}: 'ث'
    pub const LETTER_THEH: char = 'ث';
    /// \u{62c}: 'ج'
    pub const LETTER_JEEM: char = 'ج';
    /// \u{62d}: 'ح'
    pub const LETTER_HAH: char = 'ح';
    /// \u{62e}: 'خ'
    pub const LETTER_KHAH: char = 'خ';
    /// \u{62f}: 'د'
    pub const LETTER_DAL: char = 'د';
    /// \u{630}: 'ذ'
    pub const LETTER_THAL: char = 'ذ';
    /// \u{631}: 'ر'
    pub const LETTER_REH: char = 'ر';
    /// \u{632}: 'ز'
    pub const LETTER_ZAIN: char = 'ز';
    /// \u{633}: 'س'
    pub const LETTER_SEEN: char = 'س';
    /// \u{634}: 'ش'
    pub const LETTER_SHEEN: char = 'ش';
    /// \u{635}: 'ص'
    pub const LETTER_SAD: char = 'ص';
    /// \u{636}: 'ض'
    pub const LETTER_DAD: char = 'ض';
    /// \u{637}: 'ط'
    pub const LETTER_TAH: char = 'ط';
    /// \u{638}: 'ظ'
    pub const LETTER_ZAH: char = 'ظ';
    /// \u{639}: 'ع'
    pub const LETTER_AIN: char = 'ع';
    /// \u{63a}: 'غ'
    pub const LETTER_GHAIN: char = 'غ';
    /// \u{63b}: 'ػ'
    pub const LETTER_KEHEH_WITH_TWO_DOTS_ABOVE: char = 'ػ';
    /// \u{63c}: 'ؼ'
    pub const LETTER_KEHEH_WITH_THREE_DOTS_BELOW: char = 'ؼ';
    /// \u{63d}: 'ؽ'
    pub const LETTER_FARSI_YEH_WITH_INVERTED_V: char = 'ؽ';
    /// \u{63e}: 'ؾ'
    pub const LETTER_FARSI_YEH_WITH_TWO_DOTS_ABOVE: char = 'ؾ';
    /// \u{63f}: 'ؿ'
    pub const LETTER_FARSI_YEH_WITH_THREE_DOTS_ABOVE: char = 'ؿ';
    /// \u{640}: 'ـ'
    pub const TATWEEL: char = 'ـ';
    /// \u{641}: 'ف'
    pub const LETTER_FEH: char = 'ف';
    /// \u{642}: 'ق'
    pub const LETTER_QAF: char = 'ق';
    /// \u{643}: 'ك'
    pub const LETTER_KAF: char = 'ك';
    /// \u{644}: 'ل'
    pub const LETTER_LAM: char = 'ل';
    /// \u{645}: 'م'
    pub const LETTER_MEEM: char = 'م';
    /// \u{646}: 'ن'
    pub const LETTER_NOON: char = 'ن';
    /// \u{647}: 'ه'
    pub const LETTER_HEH: char = 'ه';
    /// \u{648}: 'و'
    pub const LETTER_WAW: char = 'و';
    /// \u{649}: 'ى'
    pub const LETTER_ALEF_MAKSURA: char = 'ى';
    /// \u{64a}: 'ي'
    pub const LETTER_YEH: char = 'ي';
    /// \u{64b}: 'ً'
    pub const FATHATAN: char = 'ً';
    /// \u{64c}: 'ٌ'
    pub const DAMMATAN: char = 'ٌ';
    /// \u{64d}: 'ٍ'
    pub const KASRATAN: char = 'ٍ';
    /// \u{64e}: 'َ'
    pub const FATHA: char = 'َ';
    /// \u{64f}: 'ُ'
    pub const DAMMA: char = 'ُ';
    /// \u{650}: 'ِ'
    pub const KASRA: char = 'ِ';
    /// \u{651}: 'ّ'
    pub const SHADDA: char = 'ّ';
    /// \u{652}: 'ْ'
    pub const SUKUN: char = 'ْ';
    /// \u{653}: 'ٓ'
    pub const MADDAH_ABOVE: char = 'ٓ';
    /// \u{654}: 'ٔ'
    pub const HAMZA_ABOVE: char = 'ٔ';
    /// \u{655}: 'ٕ'
    pub const HAMZA_BELOW: char = 'ٕ';
    /// \u{656}: 'ٖ'
    pub const SUBSCRIPT_ALEF: char = 'ٖ';
    /// \u{657}: 'ٗ'
    pub const INVERTED_DAMMA: char = 'ٗ';
    /// \u{658}: '٘'
    pub const MARK_NOON_GHUNNA: char = '٘';
    /// \u{659}: 'ٙ'
    pub const ZWARAKAY: char = 'ٙ';
    /// \u{65a}: 'ٚ'
    pub const VOWEL_SIGN_SMALL_V_ABOVE: char = 'ٚ';
    /// \u{65b}: 'ٛ'
    pub const VOWEL_SIGN_INVERTED_SMALL_V_ABOVE: char = 'ٛ';
    /// \u{65c}: 'ٜ'
    pub const VOWEL_SIGN_DOT_BELOW: char = 'ٜ';
    /// \u{65d}: 'ٝ'
    pub const REVERSED_DAMMA: char = 'ٝ';
    /// \u{65e}: 'ٞ'
    pub const FATHA_WITH_TWO_DOTS: char = 'ٞ';
    /// \u{65f}: 'ٟ'
    pub const WAVY_HAMZA_BELOW: char = 'ٟ';
    /// \u{660}: '٠'
    pub const DASH_INDIC_DIGIT_ZERO: char = '٠';
    /// \u{661}: '١'
    pub const DASH_INDIC_DIGIT_ONE: char = '١';
    /// \u{662}: '٢'
    pub const DASH_INDIC_DIGIT_TWO: char = '٢';
    /// \u{663}: '٣'
    pub const DASH_INDIC_DIGIT_THREE: char = '٣';
    /// \u{664}: '٤'
    pub const DASH_INDIC_DIGIT_FOUR: char = '٤';
    /// \u{665}: '٥'
    pub const DASH_INDIC_DIGIT_FIVE: char = '٥';
    /// \u{666}: '٦'
    pub const DASH_INDIC_DIGIT_SIX: char = '٦';
    /// \u{667}: '٧'
    pub const DASH_INDIC_DIGIT_SEVEN: char = '٧';
    /// \u{668}: '٨'
    pub const DASH_INDIC_DIGIT_EIGHT: char = '٨';
    /// \u{669}: '٩'
    pub const DASH_INDIC_DIGIT_NINE: char = '٩';
    /// \u{66a}: '٪'
    pub const PERCENT_SIGN: char = '٪';
    /// \u{66b}: '٫'
    pub const DECIMAL_SEPARATOR: char = '٫';
    /// \u{66c}: '٬'
    pub const THOUSANDS_SEPARATOR: char = '٬';
    /// \u{66d}: '٭'
    pub const FIVE_POINTED_STAR: char = '٭';
    /// \u{66e}: 'ٮ'
    pub const LETTER_DOTLESS_BEH: char = 'ٮ';
    /// \u{66f}: 'ٯ'
    pub const LETTER_DOTLESS_QAF: char = 'ٯ';
    /// \u{670}: 'ٰ'
    pub const LETTER_SUPERSCRIPT_ALEF: char = 'ٰ';
    /// \u{671}: 'ٱ'
    pub const LETTER_ALEF_WASLA: char = 'ٱ';
    /// \u{672}: 'ٲ'
    pub const LETTER_ALEF_WITH_WAVY_HAMZA_ABOVE: char = 'ٲ';
    /// \u{673}: 'ٳ'
    pub const LETTER_ALEF_WITH_WAVY_HAMZA_BELOW: char = 'ٳ';
    /// \u{674}: 'ٴ'
    pub const LETTER_HIGH_HAMZA: char = 'ٴ';
    /// \u{675}: 'ٵ'
    pub const LETTER_HIGH_HAMZA_ALEF: char = 'ٵ';
    /// \u{676}: 'ٶ'
    pub const LETTER_HIGH_HAMZA_WAW: char = 'ٶ';
    /// \u{677}: 'ٷ'
    pub const LETTER_U_WITH_HAMZA_ABOVE: char = 'ٷ';
    /// \u{678}: 'ٸ'
    pub const LETTER_HIGH_HAMZA_YEH: char = 'ٸ';
    /// \u{679}: 'ٹ'
    pub const LETTER_TTEH: char = 'ٹ';
    /// \u{67a}: 'ٺ'
    pub const LETTER_TTEHEH: char = 'ٺ';
    /// \u{67b}: 'ٻ'
    pub const LETTER_BEEH: char = 'ٻ';
    /// \u{67c}: 'ټ'
    pub const LETTER_TEH_WITH_RING: char = 'ټ';
    /// \u{67d}: 'ٽ'
    pub const LETTER_TEH_WITH_THREE_DOTS_ABOVE_DOWNWARDS: char = 'ٽ';
    /// \u{67e}: 'پ'
    pub const LETTER_PEH: char = 'پ';
    /// \u{67f}: 'ٿ'
    pub const LETTER_TEHEH: char = 'ٿ';
    /// \u{680}: 'ڀ'
    pub const LETTER_BEHEH: char = 'ڀ';
    /// \u{681}: 'ځ'
    pub const LETTER_HAH_WITH_HAMZA_ABOVE: char = 'ځ';
    /// \u{682}: 'ڂ'
    pub const LETTER_HAH_WITH_TWO_DOTS_VERTICAL_ABOVE: char = 'ڂ';
    /// \u{683}: 'ڃ'
    pub const LETTER_NYEH: char = 'ڃ';
    /// \u{684}: 'ڄ'
    pub const LETTER_DYEH: char = 'ڄ';
    /// \u{685}: 'څ'
    pub const LETTER_HAH_WITH_THREE_DOTS_ABOVE: char = 'څ';
    /// \u{686}: 'چ'
    pub const LETTER_TCHEH: char = 'چ';
    /// \u{687}: 'ڇ'
    pub const LETTER_TCHEHEH: char = 'ڇ';
    /// \u{688}: 'ڈ'
    pub const LETTER_DDAL: char = 'ڈ';
    /// \u{689}: 'ډ'
    pub const LETTER_DAL_WITH_RING: char = 'ډ';
    /// \u{68a}: 'ڊ'
    pub const LETTER_DAL_WITH_DOT_BELOW: char = 'ڊ';
    /// \u{68b}: 'ڋ'
    pub const LETTER_DAL_WITH_DOT_BELOW_AND_SMALL_TAH: char = 'ڋ';
    /// \u{68c}: 'ڌ'
    pub const LETTER_DAHAL: char = 'ڌ';
    /// \u{68d}: 'ڍ'
    pub const LETTER_DDAHAL: char = 'ڍ';
    /// \u{68e}: 'ڎ'
    pub const LETTER_DUL: char = 'ڎ';
    /// \u{68f}: 'ڏ'
    pub const LETTER_DAL_WITH_THREE_DOTS_ABOVE_DOWNWARDS: char = 'ڏ';
    /// \u{690}: 'ڐ'
    pub const LETTER_DAL_WITH_FOUR_DOTS_ABOVE: char = 'ڐ';
    /// \u{691}: 'ڑ'
    pub const LETTER_RREH: char = 'ڑ';
    /// \u{692}: 'ڒ'
    pub const LETTER_REH_WITH_SMALL_V: char = 'ڒ';
    /// \u{693}: 'ړ'
    pub const LETTER_REH_WITH_RING: char = 'ړ';
    /// \u{694}: 'ڔ'
    pub const LETTER_REH_WITH_DOT_BELOW: char = 'ڔ';
    /// \u{695}: 'ڕ'
    pub const LETTER_REH_WITH_SMALL_V_BELOW: char = 'ڕ';
    /// \u{696}: 'ږ'
    pub const LETTER_REH_WITH_DOT_BELOW_AND_DOT_ABOVE: char = 'ږ';
    /// \u{697}: 'ڗ'
    pub const LETTER_REH_WITH_TWO_DOTS_ABOVE: char = 'ڗ';
    /// \u{698}: 'ژ'
    pub const LETTER_JEH: char = 'ژ';
    /// \u{699}: 'ڙ'
    pub const LETTER_REH_WITH_FOUR_DOTS_ABOVE: char = 'ڙ';
    /// \u{69a}: 'ښ'
    pub const LETTER_SEEN_WITH_DOT_BELOW_AND_DOT_ABOVE: char = 'ښ';
    /// \u{69b}: 'ڛ'
    pub const LETTER_SEEN_WITH_THREE_DOTS_BELOW: char = 'ڛ';
    /// \u{69c}: 'ڜ'
    pub const LETTER_SEEN_WITH_THREE_DOTS_BELOW_AND_THREE_DOTS_ABOVE: char = 'ڜ';
    /// \u{69d}: 'ڝ'
    pub const LETTER_SAD_WITH_TWO_DOTS_BELOW: char = 'ڝ';
    /// \u{69e}: 'ڞ'
    pub const LETTER_SAD_WITH_THREE_DOTS_ABOVE: char = 'ڞ';
    /// \u{69f}: 'ڟ'
    pub const LETTER_TAH_WITH_THREE_DOTS_ABOVE: char = 'ڟ';
    /// \u{6a0}: 'ڠ'
    pub const LETTER_AIN_WITH_THREE_DOTS_ABOVE: char = 'ڠ';
    /// \u{6a1}: 'ڡ'
    pub const LETTER_DOTLESS_FEH: char = 'ڡ';
    /// \u{6a2}: 'ڢ'
    pub const LETTER_FEH_WITH_DOT_MOVED_BELOW: char = 'ڢ';
    /// \u{6a3}: 'ڣ'
    pub const LETTER_FEH_WITH_DOT_BELOW: char = 'ڣ';
    /// \u{6a4}: 'ڤ'
    pub const LETTER_VEH: char = 'ڤ';
    /// \u{6a5}: 'ڥ'
    pub const LETTER_FEH_WITH_THREE_DOTS_BELOW: char = 'ڥ';
    /// \u{6a6}: 'ڦ'
    pub const LETTER_PEHEH: char = 'ڦ';
    /// \u{6a7}: 'ڧ'
    pub const LETTER_QAF_WITH_DOT_ABOVE: char = 'ڧ';
    /// \u{6a8}: 'ڨ'
    pub const LETTER_QAF_WITH_THREE_DOTS_ABOVE: char = 'ڨ';
    /// \u{6a9}: 'ک'
    pub const LETTER_KEHEH: char = 'ک';
    /// \u{6aa}: 'ڪ'
    pub const LETTER_SWASH_KAF: char = 'ڪ';
    /// \u{6ab}: 'ګ'
    pub const LETTER_KAF_WITH_RING: char = 'ګ';
    /// \u{6ac}: 'ڬ'
    pub const LETTER_KAF_WITH_DOT_ABOVE: char = 'ڬ';
    /// \u{6ad}: 'ڭ'
    pub const LETTER_NG: char = 'ڭ';
    /// \u{6ae}: 'ڮ'
    pub const LETTER_KAF_WITH_THREE_DOTS_BELOW: char = 'ڮ';
    /// \u{6af}: 'گ'
    pub const LETTER_GAF: char = 'گ';
    /// \u{6b0}: 'ڰ'
    pub const LETTER_GAF_WITH_RING: char = 'ڰ';
    /// \u{6b1}: 'ڱ'
    pub const LETTER_NGOEH: char = 'ڱ';
    /// \u{6b2}: 'ڲ'
    pub const LETTER_GAF_WITH_TWO_DOTS_BELOW: char = 'ڲ';
    /// \u{6b3}: 'ڳ'
    pub const LETTER_GUEH: char = 'ڳ';
    /// \u{6b4}: 'ڴ'
    pub const LETTER_GAF_WITH_THREE_DOTS_ABOVE: char = 'ڴ';
    /// \u{6b5}: 'ڵ'
    pub const LETTER_LAM_WITH_SMALL_V: char = 'ڵ';
    /// \u{6b6}: 'ڶ'
    pub const LETTER_LAM_WITH_DOT_ABOVE: char = 'ڶ';
    /// \u{6b7}: 'ڷ'
    pub const LETTER_LAM_WITH_THREE_DOTS_ABOVE: char = 'ڷ';
    /// \u{6b8}: 'ڸ'
    pub const LETTER_LAM_WITH_THREE_DOTS_BELOW: char = 'ڸ';
    /// \u{6b9}: 'ڹ'
    pub const LETTER_NOON_WITH_DOT_BELOW: char = 'ڹ';
    /// \u{6ba}: 'ں'
    pub const LETTER_NOON_GHUNNA: char = 'ں';
    /// \u{6bb}: 'ڻ'
    pub const LETTER_RNOON: char = 'ڻ';
    /// \u{6bc}: 'ڼ'
    pub const LETTER_NOON_WITH_RING: char = 'ڼ';
    /// \u{6bd}: 'ڽ'
    pub const LETTER_NOON_WITH_THREE_DOTS_ABOVE: char = 'ڽ';
    /// \u{6be}: 'ھ'
    pub const LETTER_HEH_DOACHASHMEE: char = 'ھ';
    /// \u{6bf}: 'ڿ'
    pub const LETTER_TCHEH_WITH_DOT_ABOVE: char = 'ڿ';
    /// \u{6c0}: 'ۀ'
    pub const LETTER_HEH_WITH_YEH_ABOVE: char = 'ۀ';
    /// \u{6c1}: 'ہ'
    pub const LETTER_HEH_GOAL: char = 'ہ';
    /// \u{6c2}: 'ۂ'
    pub const LETTER_HEH_GOAL_WITH_HAMZA_ABOVE: char = 'ۂ';
    /// \u{6c3}: 'ۃ'
    pub const LETTER_TEH_MARBUTA_GOAL: char = 'ۃ';
    /// \u{6c4}: 'ۄ'
    pub const LETTER_WAW_WITH_RING: char = 'ۄ';
    /// \u{6c5}: 'ۅ'
    pub const LETTER_KIRGHIZ_OE: char = 'ۅ';
    /// \u{6c6}: 'ۆ'
    pub const LETTER_OE: char = 'ۆ';
    /// \u{6c7}: 'ۇ'
    pub const LETTER_U: char = 'ۇ';
    /// \u{6c8}: 'ۈ'
    pub const LETTER_YU: char = 'ۈ';
    /// \u{6c9}: 'ۉ'
    pub const LETTER_KIRGHIZ_YU: char = 'ۉ';
    /// \u{6ca}: 'ۊ'
    pub const LETTER_WAW_WITH_TWO_DOTS_ABOVE: char = 'ۊ';
    /// \u{6cb}: 'ۋ'
    pub const LETTER_VE: char = 'ۋ';
    /// \u{6cc}: 'ی'
    pub const LETTER_FARSI_YEH: char = 'ی';
    /// \u{6cd}: 'ۍ'
    pub const LETTER_YEH_WITH_TAIL: char = 'ۍ';
    /// \u{6ce}: 'ێ'
    pub const LETTER_YEH_WITH_SMALL_V: char = 'ێ';
    /// \u{6cf}: 'ۏ'
    pub const LETTER_WAW_WITH_DOT_ABOVE: char = 'ۏ';
    /// \u{6d0}: 'ې'
    pub const LETTER_E: char = 'ې';
    /// \u{6d1}: 'ۑ'
    pub const LETTER_YEH_WITH_THREE_DOTS_BELOW: char = 'ۑ';
    /// \u{6d2}: 'ے'
    pub const LETTER_YEH_BARREE: char = 'ے';
    /// \u{6d3}: 'ۓ'
    pub const LETTER_YEH_BARREE_WITH_HAMZA_ABOVE: char = 'ۓ';
    /// \u{6d4}: '۔'
    pub const FULL_STOP: char = '۔';
    /// \u{6d5}: 'ە'
    pub const LETTER_AE: char = 'ە';
    /// \u{6d6}: 'ۖ'
    pub const SMALL_HIGH_LIGATURE_SAD_WITH_LAM_WITH_ALEF_MAKSURA: char = 'ۖ';
    /// \u{6d7}: 'ۗ'
    pub const SMALL_HIGH_LIGATURE_QAF_WITH_LAM_WITH_ALEF_MAKSURA: char = 'ۗ';
    /// \u{6d8}: 'ۘ'
    pub const SMALL_HIGH_MEEM_INITIAL_FORM: char = 'ۘ';
    /// \u{6d9}: 'ۙ'
    pub const SMALL_HIGH_LAM_ALEF: char = 'ۙ';
    /// \u{6da}: 'ۚ'
    pub const SMALL_HIGH_JEEM: char = 'ۚ';
    /// \u{6db}: 'ۛ'
    pub const SMALL_HIGH_THREE_DOTS: char = 'ۛ';
    /// \u{6dc}: 'ۜ'
    pub const SMALL_HIGH_SEEN: char = 'ۜ';
    /// \u{6dd}: '۝'
    pub const END_OF_AYAH: char = '۝';
    /// \u{6de}: '۞'
    pub const START_OF_RUB_EL_HIZB: char = '۞';
    /// \u{6df}: '۟'
    pub const SMALL_HIGH_ROUNDED_ZERO: char = '۟';
    /// \u{6e0}: '۠'
    pub const SMALL_HIGH_UPRIGHT_RECTANGULAR_ZERO: char = '۠';
    /// \u{6e1}: 'ۡ'
    pub const SMALL_HIGH_DOTLESS_HEAD_OF_KHAH: char = 'ۡ';
    /// \u{6e2}: 'ۢ'
    pub const SMALL_HIGH_MEEM_ISOLATED_FORM: char = 'ۢ';
    /// \u{6e3}: 'ۣ'
    pub const SMALL_LOW_SEEN: char = 'ۣ';
    /// \u{6e4}: 'ۤ'
    pub const SMALL_HIGH_MADDA: char = 'ۤ';
    /// \u{6e5}: 'ۥ'
    pub const SMALL_WAW: char = 'ۥ';
    /// \u{6e6}: 'ۦ'
    pub const SMALL_YEH: char = 'ۦ';
    /// \u{6e7}: 'ۧ'
    pub const SMALL_HIGH_YEH: char = 'ۧ';
    /// \u{6e8}: 'ۨ'
    pub const SMALL_HIGH_NOON: char = 'ۨ';
    /// \u{6e9}: '۩'
    pub const PLACE_OF_SAJDAH: char = '۩';
    /// \u{6ea}: '۪'
    pub const EMPTY_CENTRE_LOW_STOP: char = '۪';
    /// \u{6eb}: '۫'
    pub const EMPTY_CENTRE_HIGH_STOP: char = '۫';
    /// \u{6ec}: '۬'
    pub const ROUNDED_HIGH_STOP_WITH_FILLED_CENTRE: char = '۬';
    /// \u{6ed}: 'ۭ'
    pub const SMALL_LOW_MEEM: char = 'ۭ';
    /// \u{6ee}: 'ۮ'
    pub const LETTER_DAL_WITH_INVERTED_V: char = 'ۮ';
    /// \u{6ef}: 'ۯ'
    pub const LETTER_REH_WITH_INVERTED_V: char = 'ۯ';
    /// \u{6f0}: '۰'
    pub const EXTENDED_DASH_INDIC_DIGIT_ZERO: char = '۰';
    /// \u{6f1}: '۱'
    pub const EXTENDED_DASH_INDIC_DIGIT_ONE: char = '۱';
    /// \u{6f2}: '۲'
    pub const EXTENDED_DASH_INDIC_DIGIT_TWO: char = '۲';
    /// \u{6f3}: '۳'
    pub const EXTENDED_DASH_INDIC_DIGIT_THREE: char = '۳';
    /// \u{6f4}: '۴'
    pub const EXTENDED_DASH_INDIC_DIGIT_FOUR: char = '۴';
    /// \u{6f5}: '۵'
    pub const EXTENDED_DASH_INDIC_DIGIT_FIVE: char = '۵';
    /// \u{6f6}: '۶'
    pub const EXTENDED_DASH_INDIC_DIGIT_SIX: char = '۶';
    /// \u{6f7}: '۷'
    pub const EXTENDED_DASH_INDIC_DIGIT_SEVEN: char = '۷';
    /// \u{6f8}: '۸'
    pub const EXTENDED_DASH_INDIC_DIGIT_EIGHT: char = '۸';
    /// \u{6f9}: '۹'
    pub const EXTENDED_DASH_INDIC_DIGIT_NINE: char = '۹';
    /// \u{6fa}: 'ۺ'
    pub const LETTER_SHEEN_WITH_DOT_BELOW: char = 'ۺ';
    /// \u{6fb}: 'ۻ'
    pub const LETTER_DAD_WITH_DOT_BELOW: char = 'ۻ';
    /// \u{6fc}: 'ۼ'
    pub const LETTER_GHAIN_WITH_DOT_BELOW: char = 'ۼ';
    /// \u{6fd}: '۽'
    pub const SIGN_SINDHI_AMPERSAND: char = '۽';
    /// \u{6fe}: '۾'
    pub const SIGN_SINDHI_POSTPOSITION_MEN: char = '۾';
}

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
        use constants::*;
        match self {
            Arabic::NumberSign => NUMBER_SIGN,
            Arabic::SignSanah => SIGN_SANAH,
            Arabic::FootnoteMarker => FOOTNOTE_MARKER,
            Arabic::SignSafha => SIGN_SAFHA,
            Arabic::SignSamvat => SIGN_SAMVAT,
            Arabic::NumberMarkAbove => NUMBER_MARK_ABOVE,
            Arabic::DashIndicCubeRoot => DASH_INDIC_CUBE_ROOT,
            Arabic::DashIndicFourthRoot => DASH_INDIC_FOURTH_ROOT,
            Arabic::Ray => RAY,
            Arabic::DashIndicPerMilleSign => DASH_INDIC_PER_MILLE_SIGN,
            Arabic::DashIndicPerTenThousandSign => DASH_INDIC_PER_TEN_THOUSAND_SIGN,
            Arabic::AfghaniSign => AFGHANI_SIGN,
            Arabic::Comma => COMMA,
            Arabic::DateSeparator => DATE_SEPARATOR,
            Arabic::PoeticVerseSign => POETIC_VERSE_SIGN,
            Arabic::SignMisra => SIGN_MISRA,
            Arabic::SignSallallahouAlayheWassallam => SIGN_SALLALLAHOU_ALAYHE_WASSALLAM,
            Arabic::SignAlayheAssallam => SIGN_ALAYHE_ASSALLAM,
            Arabic::SignRahmatullahAlayhe => SIGN_RAHMATULLAH_ALAYHE,
            Arabic::SignRadiAllahouAnhu => SIGN_RADI_ALLAHOU_ANHU,
            Arabic::SignTakhallus => SIGN_TAKHALLUS,
            Arabic::SmallHighTah => SMALL_HIGH_TAH,
            Arabic::SmallHighLigatureAlefWithLamWithYeh => SMALL_HIGH_LIGATURE_ALEF_WITH_LAM_WITH_YEH,
            Arabic::SmallHighZain => SMALL_HIGH_ZAIN,
            Arabic::SmallFatha => SMALL_FATHA,
            Arabic::SmallDamma => SMALL_DAMMA,
            Arabic::SmallKasra => SMALL_KASRA,
            Arabic::Semicolon => SEMICOLON,
            Arabic::LetterMark => LETTER_MARK,
            Arabic::TripleDotPunctuationMark => TRIPLE_DOT_PUNCTUATION_MARK,
            Arabic::QuestionMark => QUESTION_MARK,
            Arabic::LetterKashmiriYeh => LETTER_KASHMIRI_YEH,
            Arabic::LetterHamza => LETTER_HAMZA,
            Arabic::LetterAlefWithMaddaAbove => LETTER_ALEF_WITH_MADDA_ABOVE,
            Arabic::LetterAlefWithHamzaAbove => LETTER_ALEF_WITH_HAMZA_ABOVE,
            Arabic::LetterWawWithHamzaAbove => LETTER_WAW_WITH_HAMZA_ABOVE,
            Arabic::LetterAlefWithHamzaBelow => LETTER_ALEF_WITH_HAMZA_BELOW,
            Arabic::LetterYehWithHamzaAbove => LETTER_YEH_WITH_HAMZA_ABOVE,
            Arabic::LetterAlef => LETTER_ALEF,
            Arabic::LetterBeh => LETTER_BEH,
            Arabic::LetterTehMarbuta => LETTER_TEH_MARBUTA,
            Arabic::LetterTeh => LETTER_TEH,
            Arabic::LetterTheh => LETTER_THEH,
            Arabic::LetterJeem => LETTER_JEEM,
            Arabic::LetterHah => LETTER_HAH,
            Arabic::LetterKhah => LETTER_KHAH,
            Arabic::LetterDal => LETTER_DAL,
            Arabic::LetterThal => LETTER_THAL,
            Arabic::LetterReh => LETTER_REH,
            Arabic::LetterZain => LETTER_ZAIN,
            Arabic::LetterSeen => LETTER_SEEN,
            Arabic::LetterSheen => LETTER_SHEEN,
            Arabic::LetterSad => LETTER_SAD,
            Arabic::LetterDad => LETTER_DAD,
            Arabic::LetterTah => LETTER_TAH,
            Arabic::LetterZah => LETTER_ZAH,
            Arabic::LetterAin => LETTER_AIN,
            Arabic::LetterGhain => LETTER_GHAIN,
            Arabic::LetterKehehWithTwoDotsAbove => LETTER_KEHEH_WITH_TWO_DOTS_ABOVE,
            Arabic::LetterKehehWithThreeDotsBelow => LETTER_KEHEH_WITH_THREE_DOTS_BELOW,
            Arabic::LetterFarsiYehWithInvertedV => LETTER_FARSI_YEH_WITH_INVERTED_V,
            Arabic::LetterFarsiYehWithTwoDotsAbove => LETTER_FARSI_YEH_WITH_TWO_DOTS_ABOVE,
            Arabic::LetterFarsiYehWithThreeDotsAbove => LETTER_FARSI_YEH_WITH_THREE_DOTS_ABOVE,
            Arabic::Tatweel => TATWEEL,
            Arabic::LetterFeh => LETTER_FEH,
            Arabic::LetterQaf => LETTER_QAF,
            Arabic::LetterKaf => LETTER_KAF,
            Arabic::LetterLam => LETTER_LAM,
            Arabic::LetterMeem => LETTER_MEEM,
            Arabic::LetterNoon => LETTER_NOON,
            Arabic::LetterHeh => LETTER_HEH,
            Arabic::LetterWaw => LETTER_WAW,
            Arabic::LetterAlefMaksura => LETTER_ALEF_MAKSURA,
            Arabic::LetterYeh => LETTER_YEH,
            Arabic::Fathatan => FATHATAN,
            Arabic::Dammatan => DAMMATAN,
            Arabic::Kasratan => KASRATAN,
            Arabic::Fatha => FATHA,
            Arabic::Damma => DAMMA,
            Arabic::Kasra => KASRA,
            Arabic::Shadda => SHADDA,
            Arabic::Sukun => SUKUN,
            Arabic::MaddahAbove => MADDAH_ABOVE,
            Arabic::HamzaAbove => HAMZA_ABOVE,
            Arabic::HamzaBelow => HAMZA_BELOW,
            Arabic::SubscriptAlef => SUBSCRIPT_ALEF,
            Arabic::InvertedDamma => INVERTED_DAMMA,
            Arabic::MarkNoonGhunna => MARK_NOON_GHUNNA,
            Arabic::Zwarakay => ZWARAKAY,
            Arabic::VowelSignSmallVAbove => VOWEL_SIGN_SMALL_V_ABOVE,
            Arabic::VowelSignInvertedSmallVAbove => VOWEL_SIGN_INVERTED_SMALL_V_ABOVE,
            Arabic::VowelSignDotBelow => VOWEL_SIGN_DOT_BELOW,
            Arabic::ReversedDamma => REVERSED_DAMMA,
            Arabic::FathaWithTwoDots => FATHA_WITH_TWO_DOTS,
            Arabic::WavyHamzaBelow => WAVY_HAMZA_BELOW,
            Arabic::DashIndicDigitZero => DASH_INDIC_DIGIT_ZERO,
            Arabic::DashIndicDigitOne => DASH_INDIC_DIGIT_ONE,
            Arabic::DashIndicDigitTwo => DASH_INDIC_DIGIT_TWO,
            Arabic::DashIndicDigitThree => DASH_INDIC_DIGIT_THREE,
            Arabic::DashIndicDigitFour => DASH_INDIC_DIGIT_FOUR,
            Arabic::DashIndicDigitFive => DASH_INDIC_DIGIT_FIVE,
            Arabic::DashIndicDigitSix => DASH_INDIC_DIGIT_SIX,
            Arabic::DashIndicDigitSeven => DASH_INDIC_DIGIT_SEVEN,
            Arabic::DashIndicDigitEight => DASH_INDIC_DIGIT_EIGHT,
            Arabic::DashIndicDigitNine => DASH_INDIC_DIGIT_NINE,
            Arabic::PercentSign => PERCENT_SIGN,
            Arabic::DecimalSeparator => DECIMAL_SEPARATOR,
            Arabic::ThousandsSeparator => THOUSANDS_SEPARATOR,
            Arabic::FivePointedStar => FIVE_POINTED_STAR,
            Arabic::LetterDotlessBeh => LETTER_DOTLESS_BEH,
            Arabic::LetterDotlessQaf => LETTER_DOTLESS_QAF,
            Arabic::LetterSuperscriptAlef => LETTER_SUPERSCRIPT_ALEF,
            Arabic::LetterAlefWasla => LETTER_ALEF_WASLA,
            Arabic::LetterAlefWithWavyHamzaAbove => LETTER_ALEF_WITH_WAVY_HAMZA_ABOVE,
            Arabic::LetterAlefWithWavyHamzaBelow => LETTER_ALEF_WITH_WAVY_HAMZA_BELOW,
            Arabic::LetterHighHamza => LETTER_HIGH_HAMZA,
            Arabic::LetterHighHamzaAlef => LETTER_HIGH_HAMZA_ALEF,
            Arabic::LetterHighHamzaWaw => LETTER_HIGH_HAMZA_WAW,
            Arabic::LetterUWithHamzaAbove => LETTER_U_WITH_HAMZA_ABOVE,
            Arabic::LetterHighHamzaYeh => LETTER_HIGH_HAMZA_YEH,
            Arabic::LetterTteh => LETTER_TTEH,
            Arabic::LetterTteheh => LETTER_TTEHEH,
            Arabic::LetterBeeh => LETTER_BEEH,
            Arabic::LetterTehWithRing => LETTER_TEH_WITH_RING,
            Arabic::LetterTehWithThreeDotsAboveDownwards => LETTER_TEH_WITH_THREE_DOTS_ABOVE_DOWNWARDS,
            Arabic::LetterPeh => LETTER_PEH,
            Arabic::LetterTeheh => LETTER_TEHEH,
            Arabic::LetterBeheh => LETTER_BEHEH,
            Arabic::LetterHahWithHamzaAbove => LETTER_HAH_WITH_HAMZA_ABOVE,
            Arabic::LetterHahWithTwoDotsVerticalAbove => LETTER_HAH_WITH_TWO_DOTS_VERTICAL_ABOVE,
            Arabic::LetterNyeh => LETTER_NYEH,
            Arabic::LetterDyeh => LETTER_DYEH,
            Arabic::LetterHahWithThreeDotsAbove => LETTER_HAH_WITH_THREE_DOTS_ABOVE,
            Arabic::LetterTcheh => LETTER_TCHEH,
            Arabic::LetterTcheheh => LETTER_TCHEHEH,
            Arabic::LetterDdal => LETTER_DDAL,
            Arabic::LetterDalWithRing => LETTER_DAL_WITH_RING,
            Arabic::LetterDalWithDotBelow => LETTER_DAL_WITH_DOT_BELOW,
            Arabic::LetterDalWithDotBelowAndSmallTah => LETTER_DAL_WITH_DOT_BELOW_AND_SMALL_TAH,
            Arabic::LetterDahal => LETTER_DAHAL,
            Arabic::LetterDdahal => LETTER_DDAHAL,
            Arabic::LetterDul => LETTER_DUL,
            Arabic::LetterDalWithThreeDotsAboveDownwards => LETTER_DAL_WITH_THREE_DOTS_ABOVE_DOWNWARDS,
            Arabic::LetterDalWithFourDotsAbove => LETTER_DAL_WITH_FOUR_DOTS_ABOVE,
            Arabic::LetterRreh => LETTER_RREH,
            Arabic::LetterRehWithSmallV => LETTER_REH_WITH_SMALL_V,
            Arabic::LetterRehWithRing => LETTER_REH_WITH_RING,
            Arabic::LetterRehWithDotBelow => LETTER_REH_WITH_DOT_BELOW,
            Arabic::LetterRehWithSmallVBelow => LETTER_REH_WITH_SMALL_V_BELOW,
            Arabic::LetterRehWithDotBelowAndDotAbove => LETTER_REH_WITH_DOT_BELOW_AND_DOT_ABOVE,
            Arabic::LetterRehWithTwoDotsAbove => LETTER_REH_WITH_TWO_DOTS_ABOVE,
            Arabic::LetterJeh => LETTER_JEH,
            Arabic::LetterRehWithFourDotsAbove => LETTER_REH_WITH_FOUR_DOTS_ABOVE,
            Arabic::LetterSeenWithDotBelowAndDotAbove => LETTER_SEEN_WITH_DOT_BELOW_AND_DOT_ABOVE,
            Arabic::LetterSeenWithThreeDotsBelow => LETTER_SEEN_WITH_THREE_DOTS_BELOW,
            Arabic::LetterSeenWithThreeDotsBelowAndThreeDotsAbove => LETTER_SEEN_WITH_THREE_DOTS_BELOW_AND_THREE_DOTS_ABOVE,
            Arabic::LetterSadWithTwoDotsBelow => LETTER_SAD_WITH_TWO_DOTS_BELOW,
            Arabic::LetterSadWithThreeDotsAbove => LETTER_SAD_WITH_THREE_DOTS_ABOVE,
            Arabic::LetterTahWithThreeDotsAbove => LETTER_TAH_WITH_THREE_DOTS_ABOVE,
            Arabic::LetterAinWithThreeDotsAbove => LETTER_AIN_WITH_THREE_DOTS_ABOVE,
            Arabic::LetterDotlessFeh => LETTER_DOTLESS_FEH,
            Arabic::LetterFehWithDotMovedBelow => LETTER_FEH_WITH_DOT_MOVED_BELOW,
            Arabic::LetterFehWithDotBelow => LETTER_FEH_WITH_DOT_BELOW,
            Arabic::LetterVeh => LETTER_VEH,
            Arabic::LetterFehWithThreeDotsBelow => LETTER_FEH_WITH_THREE_DOTS_BELOW,
            Arabic::LetterPeheh => LETTER_PEHEH,
            Arabic::LetterQafWithDotAbove => LETTER_QAF_WITH_DOT_ABOVE,
            Arabic::LetterQafWithThreeDotsAbove => LETTER_QAF_WITH_THREE_DOTS_ABOVE,
            Arabic::LetterKeheh => LETTER_KEHEH,
            Arabic::LetterSwashKaf => LETTER_SWASH_KAF,
            Arabic::LetterKafWithRing => LETTER_KAF_WITH_RING,
            Arabic::LetterKafWithDotAbove => LETTER_KAF_WITH_DOT_ABOVE,
            Arabic::LetterNg => LETTER_NG,
            Arabic::LetterKafWithThreeDotsBelow => LETTER_KAF_WITH_THREE_DOTS_BELOW,
            Arabic::LetterGaf => LETTER_GAF,
            Arabic::LetterGafWithRing => LETTER_GAF_WITH_RING,
            Arabic::LetterNgoeh => LETTER_NGOEH,
            Arabic::LetterGafWithTwoDotsBelow => LETTER_GAF_WITH_TWO_DOTS_BELOW,
            Arabic::LetterGueh => LETTER_GUEH,
            Arabic::LetterGafWithThreeDotsAbove => LETTER_GAF_WITH_THREE_DOTS_ABOVE,
            Arabic::LetterLamWithSmallV => LETTER_LAM_WITH_SMALL_V,
            Arabic::LetterLamWithDotAbove => LETTER_LAM_WITH_DOT_ABOVE,
            Arabic::LetterLamWithThreeDotsAbove => LETTER_LAM_WITH_THREE_DOTS_ABOVE,
            Arabic::LetterLamWithThreeDotsBelow => LETTER_LAM_WITH_THREE_DOTS_BELOW,
            Arabic::LetterNoonWithDotBelow => LETTER_NOON_WITH_DOT_BELOW,
            Arabic::LetterNoonGhunna => LETTER_NOON_GHUNNA,
            Arabic::LetterRnoon => LETTER_RNOON,
            Arabic::LetterNoonWithRing => LETTER_NOON_WITH_RING,
            Arabic::LetterNoonWithThreeDotsAbove => LETTER_NOON_WITH_THREE_DOTS_ABOVE,
            Arabic::LetterHehDoachashmee => LETTER_HEH_DOACHASHMEE,
            Arabic::LetterTchehWithDotAbove => LETTER_TCHEH_WITH_DOT_ABOVE,
            Arabic::LetterHehWithYehAbove => LETTER_HEH_WITH_YEH_ABOVE,
            Arabic::LetterHehGoal => LETTER_HEH_GOAL,
            Arabic::LetterHehGoalWithHamzaAbove => LETTER_HEH_GOAL_WITH_HAMZA_ABOVE,
            Arabic::LetterTehMarbutaGoal => LETTER_TEH_MARBUTA_GOAL,
            Arabic::LetterWawWithRing => LETTER_WAW_WITH_RING,
            Arabic::LetterKirghizOe => LETTER_KIRGHIZ_OE,
            Arabic::LetterOe => LETTER_OE,
            Arabic::LetterU => LETTER_U,
            Arabic::LetterYu => LETTER_YU,
            Arabic::LetterKirghizYu => LETTER_KIRGHIZ_YU,
            Arabic::LetterWawWithTwoDotsAbove => LETTER_WAW_WITH_TWO_DOTS_ABOVE,
            Arabic::LetterVe => LETTER_VE,
            Arabic::LetterFarsiYeh => LETTER_FARSI_YEH,
            Arabic::LetterYehWithTail => LETTER_YEH_WITH_TAIL,
            Arabic::LetterYehWithSmallV => LETTER_YEH_WITH_SMALL_V,
            Arabic::LetterWawWithDotAbove => LETTER_WAW_WITH_DOT_ABOVE,
            Arabic::LetterE => LETTER_E,
            Arabic::LetterYehWithThreeDotsBelow => LETTER_YEH_WITH_THREE_DOTS_BELOW,
            Arabic::LetterYehBarree => LETTER_YEH_BARREE,
            Arabic::LetterYehBarreeWithHamzaAbove => LETTER_YEH_BARREE_WITH_HAMZA_ABOVE,
            Arabic::FullStop => FULL_STOP,
            Arabic::LetterAe => LETTER_AE,
            Arabic::SmallHighLigatureSadWithLamWithAlefMaksura => SMALL_HIGH_LIGATURE_SAD_WITH_LAM_WITH_ALEF_MAKSURA,
            Arabic::SmallHighLigatureQafWithLamWithAlefMaksura => SMALL_HIGH_LIGATURE_QAF_WITH_LAM_WITH_ALEF_MAKSURA,
            Arabic::SmallHighMeemInitialForm => SMALL_HIGH_MEEM_INITIAL_FORM,
            Arabic::SmallHighLamAlef => SMALL_HIGH_LAM_ALEF,
            Arabic::SmallHighJeem => SMALL_HIGH_JEEM,
            Arabic::SmallHighThreeDots => SMALL_HIGH_THREE_DOTS,
            Arabic::SmallHighSeen => SMALL_HIGH_SEEN,
            Arabic::EndOfAyah => END_OF_AYAH,
            Arabic::StartOfRubElHizb => START_OF_RUB_EL_HIZB,
            Arabic::SmallHighRoundedZero => SMALL_HIGH_ROUNDED_ZERO,
            Arabic::SmallHighUprightRectangularZero => SMALL_HIGH_UPRIGHT_RECTANGULAR_ZERO,
            Arabic::SmallHighDotlessHeadOfKhah => SMALL_HIGH_DOTLESS_HEAD_OF_KHAH,
            Arabic::SmallHighMeemIsolatedForm => SMALL_HIGH_MEEM_ISOLATED_FORM,
            Arabic::SmallLowSeen => SMALL_LOW_SEEN,
            Arabic::SmallHighMadda => SMALL_HIGH_MADDA,
            Arabic::SmallWaw => SMALL_WAW,
            Arabic::SmallYeh => SMALL_YEH,
            Arabic::SmallHighYeh => SMALL_HIGH_YEH,
            Arabic::SmallHighNoon => SMALL_HIGH_NOON,
            Arabic::PlaceOfSajdah => PLACE_OF_SAJDAH,
            Arabic::EmptyCentreLowStop => EMPTY_CENTRE_LOW_STOP,
            Arabic::EmptyCentreHighStop => EMPTY_CENTRE_HIGH_STOP,
            Arabic::RoundedHighStopWithFilledCentre => ROUNDED_HIGH_STOP_WITH_FILLED_CENTRE,
            Arabic::SmallLowMeem => SMALL_LOW_MEEM,
            Arabic::LetterDalWithInvertedV => LETTER_DAL_WITH_INVERTED_V,
            Arabic::LetterRehWithInvertedV => LETTER_REH_WITH_INVERTED_V,
            Arabic::ExtendedDashIndicDigitZero => EXTENDED_DASH_INDIC_DIGIT_ZERO,
            Arabic::ExtendedDashIndicDigitOne => EXTENDED_DASH_INDIC_DIGIT_ONE,
            Arabic::ExtendedDashIndicDigitTwo => EXTENDED_DASH_INDIC_DIGIT_TWO,
            Arabic::ExtendedDashIndicDigitThree => EXTENDED_DASH_INDIC_DIGIT_THREE,
            Arabic::ExtendedDashIndicDigitFour => EXTENDED_DASH_INDIC_DIGIT_FOUR,
            Arabic::ExtendedDashIndicDigitFive => EXTENDED_DASH_INDIC_DIGIT_FIVE,
            Arabic::ExtendedDashIndicDigitSix => EXTENDED_DASH_INDIC_DIGIT_SIX,
            Arabic::ExtendedDashIndicDigitSeven => EXTENDED_DASH_INDIC_DIGIT_SEVEN,
            Arabic::ExtendedDashIndicDigitEight => EXTENDED_DASH_INDIC_DIGIT_EIGHT,
            Arabic::ExtendedDashIndicDigitNine => EXTENDED_DASH_INDIC_DIGIT_NINE,
            Arabic::LetterSheenWithDotBelow => LETTER_SHEEN_WITH_DOT_BELOW,
            Arabic::LetterDadWithDotBelow => LETTER_DAD_WITH_DOT_BELOW,
            Arabic::LetterGhainWithDotBelow => LETTER_GHAIN_WITH_DOT_BELOW,
            Arabic::SignSindhiAmpersand => SIGN_SINDHI_AMPERSAND,
            Arabic::SignSindhiPostpositionMen => SIGN_SINDHI_POSTPOSITION_MEN,
        }
    }
}

impl std::convert::TryFrom<char> for Arabic {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            NUMBER_SIGN => Ok(Arabic::NumberSign),
            SIGN_SANAH => Ok(Arabic::SignSanah),
            FOOTNOTE_MARKER => Ok(Arabic::FootnoteMarker),
            SIGN_SAFHA => Ok(Arabic::SignSafha),
            SIGN_SAMVAT => Ok(Arabic::SignSamvat),
            NUMBER_MARK_ABOVE => Ok(Arabic::NumberMarkAbove),
            DASH_INDIC_CUBE_ROOT => Ok(Arabic::DashIndicCubeRoot),
            DASH_INDIC_FOURTH_ROOT => Ok(Arabic::DashIndicFourthRoot),
            RAY => Ok(Arabic::Ray),
            DASH_INDIC_PER_MILLE_SIGN => Ok(Arabic::DashIndicPerMilleSign),
            DASH_INDIC_PER_TEN_THOUSAND_SIGN => Ok(Arabic::DashIndicPerTenThousandSign),
            AFGHANI_SIGN => Ok(Arabic::AfghaniSign),
            COMMA => Ok(Arabic::Comma),
            DATE_SEPARATOR => Ok(Arabic::DateSeparator),
            POETIC_VERSE_SIGN => Ok(Arabic::PoeticVerseSign),
            SIGN_MISRA => Ok(Arabic::SignMisra),
            SIGN_SALLALLAHOU_ALAYHE_WASSALLAM => Ok(Arabic::SignSallallahouAlayheWassallam),
            SIGN_ALAYHE_ASSALLAM => Ok(Arabic::SignAlayheAssallam),
            SIGN_RAHMATULLAH_ALAYHE => Ok(Arabic::SignRahmatullahAlayhe),
            SIGN_RADI_ALLAHOU_ANHU => Ok(Arabic::SignRadiAllahouAnhu),
            SIGN_TAKHALLUS => Ok(Arabic::SignTakhallus),
            SMALL_HIGH_TAH => Ok(Arabic::SmallHighTah),
            SMALL_HIGH_LIGATURE_ALEF_WITH_LAM_WITH_YEH => Ok(Arabic::SmallHighLigatureAlefWithLamWithYeh),
            SMALL_HIGH_ZAIN => Ok(Arabic::SmallHighZain),
            SMALL_FATHA => Ok(Arabic::SmallFatha),
            SMALL_DAMMA => Ok(Arabic::SmallDamma),
            SMALL_KASRA => Ok(Arabic::SmallKasra),
            SEMICOLON => Ok(Arabic::Semicolon),
            LETTER_MARK => Ok(Arabic::LetterMark),
            TRIPLE_DOT_PUNCTUATION_MARK => Ok(Arabic::TripleDotPunctuationMark),
            QUESTION_MARK => Ok(Arabic::QuestionMark),
            LETTER_KASHMIRI_YEH => Ok(Arabic::LetterKashmiriYeh),
            LETTER_HAMZA => Ok(Arabic::LetterHamza),
            LETTER_ALEF_WITH_MADDA_ABOVE => Ok(Arabic::LetterAlefWithMaddaAbove),
            LETTER_ALEF_WITH_HAMZA_ABOVE => Ok(Arabic::LetterAlefWithHamzaAbove),
            LETTER_WAW_WITH_HAMZA_ABOVE => Ok(Arabic::LetterWawWithHamzaAbove),
            LETTER_ALEF_WITH_HAMZA_BELOW => Ok(Arabic::LetterAlefWithHamzaBelow),
            LETTER_YEH_WITH_HAMZA_ABOVE => Ok(Arabic::LetterYehWithHamzaAbove),
            LETTER_ALEF => Ok(Arabic::LetterAlef),
            LETTER_BEH => Ok(Arabic::LetterBeh),
            LETTER_TEH_MARBUTA => Ok(Arabic::LetterTehMarbuta),
            LETTER_TEH => Ok(Arabic::LetterTeh),
            LETTER_THEH => Ok(Arabic::LetterTheh),
            LETTER_JEEM => Ok(Arabic::LetterJeem),
            LETTER_HAH => Ok(Arabic::LetterHah),
            LETTER_KHAH => Ok(Arabic::LetterKhah),
            LETTER_DAL => Ok(Arabic::LetterDal),
            LETTER_THAL => Ok(Arabic::LetterThal),
            LETTER_REH => Ok(Arabic::LetterReh),
            LETTER_ZAIN => Ok(Arabic::LetterZain),
            LETTER_SEEN => Ok(Arabic::LetterSeen),
            LETTER_SHEEN => Ok(Arabic::LetterSheen),
            LETTER_SAD => Ok(Arabic::LetterSad),
            LETTER_DAD => Ok(Arabic::LetterDad),
            LETTER_TAH => Ok(Arabic::LetterTah),
            LETTER_ZAH => Ok(Arabic::LetterZah),
            LETTER_AIN => Ok(Arabic::LetterAin),
            LETTER_GHAIN => Ok(Arabic::LetterGhain),
            LETTER_KEHEH_WITH_TWO_DOTS_ABOVE => Ok(Arabic::LetterKehehWithTwoDotsAbove),
            LETTER_KEHEH_WITH_THREE_DOTS_BELOW => Ok(Arabic::LetterKehehWithThreeDotsBelow),
            LETTER_FARSI_YEH_WITH_INVERTED_V => Ok(Arabic::LetterFarsiYehWithInvertedV),
            LETTER_FARSI_YEH_WITH_TWO_DOTS_ABOVE => Ok(Arabic::LetterFarsiYehWithTwoDotsAbove),
            LETTER_FARSI_YEH_WITH_THREE_DOTS_ABOVE => Ok(Arabic::LetterFarsiYehWithThreeDotsAbove),
            TATWEEL => Ok(Arabic::Tatweel),
            LETTER_FEH => Ok(Arabic::LetterFeh),
            LETTER_QAF => Ok(Arabic::LetterQaf),
            LETTER_KAF => Ok(Arabic::LetterKaf),
            LETTER_LAM => Ok(Arabic::LetterLam),
            LETTER_MEEM => Ok(Arabic::LetterMeem),
            LETTER_NOON => Ok(Arabic::LetterNoon),
            LETTER_HEH => Ok(Arabic::LetterHeh),
            LETTER_WAW => Ok(Arabic::LetterWaw),
            LETTER_ALEF_MAKSURA => Ok(Arabic::LetterAlefMaksura),
            LETTER_YEH => Ok(Arabic::LetterYeh),
            FATHATAN => Ok(Arabic::Fathatan),
            DAMMATAN => Ok(Arabic::Dammatan),
            KASRATAN => Ok(Arabic::Kasratan),
            FATHA => Ok(Arabic::Fatha),
            DAMMA => Ok(Arabic::Damma),
            KASRA => Ok(Arabic::Kasra),
            SHADDA => Ok(Arabic::Shadda),
            SUKUN => Ok(Arabic::Sukun),
            MADDAH_ABOVE => Ok(Arabic::MaddahAbove),
            HAMZA_ABOVE => Ok(Arabic::HamzaAbove),
            HAMZA_BELOW => Ok(Arabic::HamzaBelow),
            SUBSCRIPT_ALEF => Ok(Arabic::SubscriptAlef),
            INVERTED_DAMMA => Ok(Arabic::InvertedDamma),
            MARK_NOON_GHUNNA => Ok(Arabic::MarkNoonGhunna),
            ZWARAKAY => Ok(Arabic::Zwarakay),
            VOWEL_SIGN_SMALL_V_ABOVE => Ok(Arabic::VowelSignSmallVAbove),
            VOWEL_SIGN_INVERTED_SMALL_V_ABOVE => Ok(Arabic::VowelSignInvertedSmallVAbove),
            VOWEL_SIGN_DOT_BELOW => Ok(Arabic::VowelSignDotBelow),
            REVERSED_DAMMA => Ok(Arabic::ReversedDamma),
            FATHA_WITH_TWO_DOTS => Ok(Arabic::FathaWithTwoDots),
            WAVY_HAMZA_BELOW => Ok(Arabic::WavyHamzaBelow),
            DASH_INDIC_DIGIT_ZERO => Ok(Arabic::DashIndicDigitZero),
            DASH_INDIC_DIGIT_ONE => Ok(Arabic::DashIndicDigitOne),
            DASH_INDIC_DIGIT_TWO => Ok(Arabic::DashIndicDigitTwo),
            DASH_INDIC_DIGIT_THREE => Ok(Arabic::DashIndicDigitThree),
            DASH_INDIC_DIGIT_FOUR => Ok(Arabic::DashIndicDigitFour),
            DASH_INDIC_DIGIT_FIVE => Ok(Arabic::DashIndicDigitFive),
            DASH_INDIC_DIGIT_SIX => Ok(Arabic::DashIndicDigitSix),
            DASH_INDIC_DIGIT_SEVEN => Ok(Arabic::DashIndicDigitSeven),
            DASH_INDIC_DIGIT_EIGHT => Ok(Arabic::DashIndicDigitEight),
            DASH_INDIC_DIGIT_NINE => Ok(Arabic::DashIndicDigitNine),
            PERCENT_SIGN => Ok(Arabic::PercentSign),
            DECIMAL_SEPARATOR => Ok(Arabic::DecimalSeparator),
            THOUSANDS_SEPARATOR => Ok(Arabic::ThousandsSeparator),
            FIVE_POINTED_STAR => Ok(Arabic::FivePointedStar),
            LETTER_DOTLESS_BEH => Ok(Arabic::LetterDotlessBeh),
            LETTER_DOTLESS_QAF => Ok(Arabic::LetterDotlessQaf),
            LETTER_SUPERSCRIPT_ALEF => Ok(Arabic::LetterSuperscriptAlef),
            LETTER_ALEF_WASLA => Ok(Arabic::LetterAlefWasla),
            LETTER_ALEF_WITH_WAVY_HAMZA_ABOVE => Ok(Arabic::LetterAlefWithWavyHamzaAbove),
            LETTER_ALEF_WITH_WAVY_HAMZA_BELOW => Ok(Arabic::LetterAlefWithWavyHamzaBelow),
            LETTER_HIGH_HAMZA => Ok(Arabic::LetterHighHamza),
            LETTER_HIGH_HAMZA_ALEF => Ok(Arabic::LetterHighHamzaAlef),
            LETTER_HIGH_HAMZA_WAW => Ok(Arabic::LetterHighHamzaWaw),
            LETTER_U_WITH_HAMZA_ABOVE => Ok(Arabic::LetterUWithHamzaAbove),
            LETTER_HIGH_HAMZA_YEH => Ok(Arabic::LetterHighHamzaYeh),
            LETTER_TTEH => Ok(Arabic::LetterTteh),
            LETTER_TTEHEH => Ok(Arabic::LetterTteheh),
            LETTER_BEEH => Ok(Arabic::LetterBeeh),
            LETTER_TEH_WITH_RING => Ok(Arabic::LetterTehWithRing),
            LETTER_TEH_WITH_THREE_DOTS_ABOVE_DOWNWARDS => Ok(Arabic::LetterTehWithThreeDotsAboveDownwards),
            LETTER_PEH => Ok(Arabic::LetterPeh),
            LETTER_TEHEH => Ok(Arabic::LetterTeheh),
            LETTER_BEHEH => Ok(Arabic::LetterBeheh),
            LETTER_HAH_WITH_HAMZA_ABOVE => Ok(Arabic::LetterHahWithHamzaAbove),
            LETTER_HAH_WITH_TWO_DOTS_VERTICAL_ABOVE => Ok(Arabic::LetterHahWithTwoDotsVerticalAbove),
            LETTER_NYEH => Ok(Arabic::LetterNyeh),
            LETTER_DYEH => Ok(Arabic::LetterDyeh),
            LETTER_HAH_WITH_THREE_DOTS_ABOVE => Ok(Arabic::LetterHahWithThreeDotsAbove),
            LETTER_TCHEH => Ok(Arabic::LetterTcheh),
            LETTER_TCHEHEH => Ok(Arabic::LetterTcheheh),
            LETTER_DDAL => Ok(Arabic::LetterDdal),
            LETTER_DAL_WITH_RING => Ok(Arabic::LetterDalWithRing),
            LETTER_DAL_WITH_DOT_BELOW => Ok(Arabic::LetterDalWithDotBelow),
            LETTER_DAL_WITH_DOT_BELOW_AND_SMALL_TAH => Ok(Arabic::LetterDalWithDotBelowAndSmallTah),
            LETTER_DAHAL => Ok(Arabic::LetterDahal),
            LETTER_DDAHAL => Ok(Arabic::LetterDdahal),
            LETTER_DUL => Ok(Arabic::LetterDul),
            LETTER_DAL_WITH_THREE_DOTS_ABOVE_DOWNWARDS => Ok(Arabic::LetterDalWithThreeDotsAboveDownwards),
            LETTER_DAL_WITH_FOUR_DOTS_ABOVE => Ok(Arabic::LetterDalWithFourDotsAbove),
            LETTER_RREH => Ok(Arabic::LetterRreh),
            LETTER_REH_WITH_SMALL_V => Ok(Arabic::LetterRehWithSmallV),
            LETTER_REH_WITH_RING => Ok(Arabic::LetterRehWithRing),
            LETTER_REH_WITH_DOT_BELOW => Ok(Arabic::LetterRehWithDotBelow),
            LETTER_REH_WITH_SMALL_V_BELOW => Ok(Arabic::LetterRehWithSmallVBelow),
            LETTER_REH_WITH_DOT_BELOW_AND_DOT_ABOVE => Ok(Arabic::LetterRehWithDotBelowAndDotAbove),
            LETTER_REH_WITH_TWO_DOTS_ABOVE => Ok(Arabic::LetterRehWithTwoDotsAbove),
            LETTER_JEH => Ok(Arabic::LetterJeh),
            LETTER_REH_WITH_FOUR_DOTS_ABOVE => Ok(Arabic::LetterRehWithFourDotsAbove),
            LETTER_SEEN_WITH_DOT_BELOW_AND_DOT_ABOVE => Ok(Arabic::LetterSeenWithDotBelowAndDotAbove),
            LETTER_SEEN_WITH_THREE_DOTS_BELOW => Ok(Arabic::LetterSeenWithThreeDotsBelow),
            LETTER_SEEN_WITH_THREE_DOTS_BELOW_AND_THREE_DOTS_ABOVE => Ok(Arabic::LetterSeenWithThreeDotsBelowAndThreeDotsAbove),
            LETTER_SAD_WITH_TWO_DOTS_BELOW => Ok(Arabic::LetterSadWithTwoDotsBelow),
            LETTER_SAD_WITH_THREE_DOTS_ABOVE => Ok(Arabic::LetterSadWithThreeDotsAbove),
            LETTER_TAH_WITH_THREE_DOTS_ABOVE => Ok(Arabic::LetterTahWithThreeDotsAbove),
            LETTER_AIN_WITH_THREE_DOTS_ABOVE => Ok(Arabic::LetterAinWithThreeDotsAbove),
            LETTER_DOTLESS_FEH => Ok(Arabic::LetterDotlessFeh),
            LETTER_FEH_WITH_DOT_MOVED_BELOW => Ok(Arabic::LetterFehWithDotMovedBelow),
            LETTER_FEH_WITH_DOT_BELOW => Ok(Arabic::LetterFehWithDotBelow),
            LETTER_VEH => Ok(Arabic::LetterVeh),
            LETTER_FEH_WITH_THREE_DOTS_BELOW => Ok(Arabic::LetterFehWithThreeDotsBelow),
            LETTER_PEHEH => Ok(Arabic::LetterPeheh),
            LETTER_QAF_WITH_DOT_ABOVE => Ok(Arabic::LetterQafWithDotAbove),
            LETTER_QAF_WITH_THREE_DOTS_ABOVE => Ok(Arabic::LetterQafWithThreeDotsAbove),
            LETTER_KEHEH => Ok(Arabic::LetterKeheh),
            LETTER_SWASH_KAF => Ok(Arabic::LetterSwashKaf),
            LETTER_KAF_WITH_RING => Ok(Arabic::LetterKafWithRing),
            LETTER_KAF_WITH_DOT_ABOVE => Ok(Arabic::LetterKafWithDotAbove),
            LETTER_NG => Ok(Arabic::LetterNg),
            LETTER_KAF_WITH_THREE_DOTS_BELOW => Ok(Arabic::LetterKafWithThreeDotsBelow),
            LETTER_GAF => Ok(Arabic::LetterGaf),
            LETTER_GAF_WITH_RING => Ok(Arabic::LetterGafWithRing),
            LETTER_NGOEH => Ok(Arabic::LetterNgoeh),
            LETTER_GAF_WITH_TWO_DOTS_BELOW => Ok(Arabic::LetterGafWithTwoDotsBelow),
            LETTER_GUEH => Ok(Arabic::LetterGueh),
            LETTER_GAF_WITH_THREE_DOTS_ABOVE => Ok(Arabic::LetterGafWithThreeDotsAbove),
            LETTER_LAM_WITH_SMALL_V => Ok(Arabic::LetterLamWithSmallV),
            LETTER_LAM_WITH_DOT_ABOVE => Ok(Arabic::LetterLamWithDotAbove),
            LETTER_LAM_WITH_THREE_DOTS_ABOVE => Ok(Arabic::LetterLamWithThreeDotsAbove),
            LETTER_LAM_WITH_THREE_DOTS_BELOW => Ok(Arabic::LetterLamWithThreeDotsBelow),
            LETTER_NOON_WITH_DOT_BELOW => Ok(Arabic::LetterNoonWithDotBelow),
            LETTER_NOON_GHUNNA => Ok(Arabic::LetterNoonGhunna),
            LETTER_RNOON => Ok(Arabic::LetterRnoon),
            LETTER_NOON_WITH_RING => Ok(Arabic::LetterNoonWithRing),
            LETTER_NOON_WITH_THREE_DOTS_ABOVE => Ok(Arabic::LetterNoonWithThreeDotsAbove),
            LETTER_HEH_DOACHASHMEE => Ok(Arabic::LetterHehDoachashmee),
            LETTER_TCHEH_WITH_DOT_ABOVE => Ok(Arabic::LetterTchehWithDotAbove),
            LETTER_HEH_WITH_YEH_ABOVE => Ok(Arabic::LetterHehWithYehAbove),
            LETTER_HEH_GOAL => Ok(Arabic::LetterHehGoal),
            LETTER_HEH_GOAL_WITH_HAMZA_ABOVE => Ok(Arabic::LetterHehGoalWithHamzaAbove),
            LETTER_TEH_MARBUTA_GOAL => Ok(Arabic::LetterTehMarbutaGoal),
            LETTER_WAW_WITH_RING => Ok(Arabic::LetterWawWithRing),
            LETTER_KIRGHIZ_OE => Ok(Arabic::LetterKirghizOe),
            LETTER_OE => Ok(Arabic::LetterOe),
            LETTER_U => Ok(Arabic::LetterU),
            LETTER_YU => Ok(Arabic::LetterYu),
            LETTER_KIRGHIZ_YU => Ok(Arabic::LetterKirghizYu),
            LETTER_WAW_WITH_TWO_DOTS_ABOVE => Ok(Arabic::LetterWawWithTwoDotsAbove),
            LETTER_VE => Ok(Arabic::LetterVe),
            LETTER_FARSI_YEH => Ok(Arabic::LetterFarsiYeh),
            LETTER_YEH_WITH_TAIL => Ok(Arabic::LetterYehWithTail),
            LETTER_YEH_WITH_SMALL_V => Ok(Arabic::LetterYehWithSmallV),
            LETTER_WAW_WITH_DOT_ABOVE => Ok(Arabic::LetterWawWithDotAbove),
            LETTER_E => Ok(Arabic::LetterE),
            LETTER_YEH_WITH_THREE_DOTS_BELOW => Ok(Arabic::LetterYehWithThreeDotsBelow),
            LETTER_YEH_BARREE => Ok(Arabic::LetterYehBarree),
            LETTER_YEH_BARREE_WITH_HAMZA_ABOVE => Ok(Arabic::LetterYehBarreeWithHamzaAbove),
            FULL_STOP => Ok(Arabic::FullStop),
            LETTER_AE => Ok(Arabic::LetterAe),
            SMALL_HIGH_LIGATURE_SAD_WITH_LAM_WITH_ALEF_MAKSURA => Ok(Arabic::SmallHighLigatureSadWithLamWithAlefMaksura),
            SMALL_HIGH_LIGATURE_QAF_WITH_LAM_WITH_ALEF_MAKSURA => Ok(Arabic::SmallHighLigatureQafWithLamWithAlefMaksura),
            SMALL_HIGH_MEEM_INITIAL_FORM => Ok(Arabic::SmallHighMeemInitialForm),
            SMALL_HIGH_LAM_ALEF => Ok(Arabic::SmallHighLamAlef),
            SMALL_HIGH_JEEM => Ok(Arabic::SmallHighJeem),
            SMALL_HIGH_THREE_DOTS => Ok(Arabic::SmallHighThreeDots),
            SMALL_HIGH_SEEN => Ok(Arabic::SmallHighSeen),
            END_OF_AYAH => Ok(Arabic::EndOfAyah),
            START_OF_RUB_EL_HIZB => Ok(Arabic::StartOfRubElHizb),
            SMALL_HIGH_ROUNDED_ZERO => Ok(Arabic::SmallHighRoundedZero),
            SMALL_HIGH_UPRIGHT_RECTANGULAR_ZERO => Ok(Arabic::SmallHighUprightRectangularZero),
            SMALL_HIGH_DOTLESS_HEAD_OF_KHAH => Ok(Arabic::SmallHighDotlessHeadOfKhah),
            SMALL_HIGH_MEEM_ISOLATED_FORM => Ok(Arabic::SmallHighMeemIsolatedForm),
            SMALL_LOW_SEEN => Ok(Arabic::SmallLowSeen),
            SMALL_HIGH_MADDA => Ok(Arabic::SmallHighMadda),
            SMALL_WAW => Ok(Arabic::SmallWaw),
            SMALL_YEH => Ok(Arabic::SmallYeh),
            SMALL_HIGH_YEH => Ok(Arabic::SmallHighYeh),
            SMALL_HIGH_NOON => Ok(Arabic::SmallHighNoon),
            PLACE_OF_SAJDAH => Ok(Arabic::PlaceOfSajdah),
            EMPTY_CENTRE_LOW_STOP => Ok(Arabic::EmptyCentreLowStop),
            EMPTY_CENTRE_HIGH_STOP => Ok(Arabic::EmptyCentreHighStop),
            ROUNDED_HIGH_STOP_WITH_FILLED_CENTRE => Ok(Arabic::RoundedHighStopWithFilledCentre),
            SMALL_LOW_MEEM => Ok(Arabic::SmallLowMeem),
            LETTER_DAL_WITH_INVERTED_V => Ok(Arabic::LetterDalWithInvertedV),
            LETTER_REH_WITH_INVERTED_V => Ok(Arabic::LetterRehWithInvertedV),
            EXTENDED_DASH_INDIC_DIGIT_ZERO => Ok(Arabic::ExtendedDashIndicDigitZero),
            EXTENDED_DASH_INDIC_DIGIT_ONE => Ok(Arabic::ExtendedDashIndicDigitOne),
            EXTENDED_DASH_INDIC_DIGIT_TWO => Ok(Arabic::ExtendedDashIndicDigitTwo),
            EXTENDED_DASH_INDIC_DIGIT_THREE => Ok(Arabic::ExtendedDashIndicDigitThree),
            EXTENDED_DASH_INDIC_DIGIT_FOUR => Ok(Arabic::ExtendedDashIndicDigitFour),
            EXTENDED_DASH_INDIC_DIGIT_FIVE => Ok(Arabic::ExtendedDashIndicDigitFive),
            EXTENDED_DASH_INDIC_DIGIT_SIX => Ok(Arabic::ExtendedDashIndicDigitSix),
            EXTENDED_DASH_INDIC_DIGIT_SEVEN => Ok(Arabic::ExtendedDashIndicDigitSeven),
            EXTENDED_DASH_INDIC_DIGIT_EIGHT => Ok(Arabic::ExtendedDashIndicDigitEight),
            EXTENDED_DASH_INDIC_DIGIT_NINE => Ok(Arabic::ExtendedDashIndicDigitNine),
            LETTER_SHEEN_WITH_DOT_BELOW => Ok(Arabic::LetterSheenWithDotBelow),
            LETTER_DAD_WITH_DOT_BELOW => Ok(Arabic::LetterDadWithDotBelow),
            LETTER_GHAIN_WITH_DOT_BELOW => Ok(Arabic::LetterGhainWithDotBelow),
            SIGN_SINDHI_AMPERSAND => Ok(Arabic::SignSindhiAmpersand),
            SIGN_SINDHI_POSTPOSITION_MEN => Ok(Arabic::SignSindhiPostpositionMen),
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
