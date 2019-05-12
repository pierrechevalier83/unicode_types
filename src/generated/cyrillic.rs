/// \u{400} → \u{4ff}
///
/// Ѐ Ё Ђ Ѓ Є Ѕ І Ї Ј Љ Њ Ћ Ќ Ѝ Ў Џ\
/// А Б В Г Д Е Ж З И Й К Л М Н О П\
/// Р С Т У Ф Х Ц Ч Ш Щ Ъ Ы Ь Э Ю Я\
/// а б в г д е ж з и й к л м н о п\
/// р с т у ф х ц ч ш щ ъ ы ь э ю я\
/// ѐ ё ђ ѓ є ѕ і ї ј љ њ ћ ќ ѝ ў џ\
/// Ѡ ѡ Ѣ ѣ Ѥ ѥ Ѧ ѧ Ѩ ѩ Ѫ ѫ Ѭ ѭ Ѯ ѯ\
/// Ѱ ѱ Ѳ ѳ Ѵ ѵ Ѷ ѷ Ѹ ѹ Ѻ ѻ Ѽ ѽ Ѿ ѿ\
/// Ҁ ҁ ҂ ҃ ҄ ҅ ҆ ҇ ҈ ҉ Ҋ ҋ Ҍ ҍ Ҏ ҏ\
/// Ґ ґ Ғ ғ Ҕ ҕ Җ җ Ҙ ҙ Қ қ Ҝ ҝ Ҟ ҟ\
/// Ҡ ҡ Ң ң Ҥ ҥ Ҧ ҧ Ҩ ҩ Ҫ ҫ Ҭ ҭ Ү ү\
/// Ұ ұ Ҳ ҳ Ҵ ҵ Ҷ ҷ Ҹ ҹ Һ һ Ҽ ҽ Ҿ ҿ\
/// Ӏ Ӂ ӂ Ӄ ӄ Ӆ ӆ Ӈ ӈ Ӊ ӊ Ӌ ӌ Ӎ ӎ ӏ\
/// Ӑ ӑ Ӓ ӓ Ӕ ӕ Ӗ ӗ Ә ә Ӛ ӛ Ӝ ӝ Ӟ ӟ\
/// Ӡ ӡ Ӣ ӣ Ӥ ӥ Ӧ ӧ Ө ө Ӫ ӫ Ӭ ӭ Ӯ ӯ\
/// Ӱ ӱ Ӳ ӳ Ӵ ӵ Ӷ ӷ Ӹ ӹ Ӻ ӻ Ӽ ӽ Ӿ\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{400}: 'Ѐ'
    pub const CAPITAL_LETTER_IE_WITH_GRAVE: char = 'Ѐ';
    /// \u{401}: 'Ё'
    pub const CAPITAL_LETTER_IO: char = 'Ё';
    /// \u{402}: 'Ђ'
    pub const CAPITAL_LETTER_DJE: char = 'Ђ';
    /// \u{403}: 'Ѓ'
    pub const CAPITAL_LETTER_GJE: char = 'Ѓ';
    /// \u{404}: 'Є'
    pub const CAPITAL_LETTER_UKRAINIAN_IE: char = 'Є';
    /// \u{405}: 'Ѕ'
    pub const CAPITAL_LETTER_DZE: char = 'Ѕ';
    /// \u{406}: 'І'
    pub const CAPITAL_LETTER_BYELORUSSIAN_DASH_UKRAINIAN_I: char = 'І';
    /// \u{407}: 'Ї'
    pub const CAPITAL_LETTER_YI: char = 'Ї';
    /// \u{408}: 'Ј'
    pub const CAPITAL_LETTER_JE: char = 'Ј';
    /// \u{409}: 'Љ'
    pub const CAPITAL_LETTER_LJE: char = 'Љ';
    /// \u{40a}: 'Њ'
    pub const CAPITAL_LETTER_NJE: char = 'Њ';
    /// \u{40b}: 'Ћ'
    pub const CAPITAL_LETTER_TSHE: char = 'Ћ';
    /// \u{40c}: 'Ќ'
    pub const CAPITAL_LETTER_KJE: char = 'Ќ';
    /// \u{40d}: 'Ѝ'
    pub const CAPITAL_LETTER_I_WITH_GRAVE: char = 'Ѝ';
    /// \u{40e}: 'Ў'
    pub const CAPITAL_LETTER_SHORT_U: char = 'Ў';
    /// \u{40f}: 'Џ'
    pub const CAPITAL_LETTER_DZHE: char = 'Џ';
    /// \u{410}: 'А'
    pub const CAPITAL_LETTER_A: char = 'А';
    /// \u{411}: 'Б'
    pub const CAPITAL_LETTER_BE: char = 'Б';
    /// \u{412}: 'В'
    pub const CAPITAL_LETTER_VE: char = 'В';
    /// \u{413}: 'Г'
    pub const CAPITAL_LETTER_GHE: char = 'Г';
    /// \u{414}: 'Д'
    pub const CAPITAL_LETTER_DE: char = 'Д';
    /// \u{415}: 'Е'
    pub const CAPITAL_LETTER_IE: char = 'Е';
    /// \u{416}: 'Ж'
    pub const CAPITAL_LETTER_ZHE: char = 'Ж';
    /// \u{417}: 'З'
    pub const CAPITAL_LETTER_ZE: char = 'З';
    /// \u{418}: 'И'
    pub const CAPITAL_LETTER_I: char = 'И';
    /// \u{419}: 'Й'
    pub const CAPITAL_LETTER_SHORT_I: char = 'Й';
    /// \u{41a}: 'К'
    pub const CAPITAL_LETTER_KA: char = 'К';
    /// \u{41b}: 'Л'
    pub const CAPITAL_LETTER_EL: char = 'Л';
    /// \u{41c}: 'М'
    pub const CAPITAL_LETTER_EM: char = 'М';
    /// \u{41d}: 'Н'
    pub const CAPITAL_LETTER_EN: char = 'Н';
    /// \u{41e}: 'О'
    pub const CAPITAL_LETTER_O: char = 'О';
    /// \u{41f}: 'П'
    pub const CAPITAL_LETTER_PE: char = 'П';
    /// \u{420}: 'Р'
    pub const CAPITAL_LETTER_ER: char = 'Р';
    /// \u{421}: 'С'
    pub const CAPITAL_LETTER_ES: char = 'С';
    /// \u{422}: 'Т'
    pub const CAPITAL_LETTER_TE: char = 'Т';
    /// \u{423}: 'У'
    pub const CAPITAL_LETTER_U: char = 'У';
    /// \u{424}: 'Ф'
    pub const CAPITAL_LETTER_EF: char = 'Ф';
    /// \u{425}: 'Х'
    pub const CAPITAL_LETTER_HA: char = 'Х';
    /// \u{426}: 'Ц'
    pub const CAPITAL_LETTER_TSE: char = 'Ц';
    /// \u{427}: 'Ч'
    pub const CAPITAL_LETTER_CHE: char = 'Ч';
    /// \u{428}: 'Ш'
    pub const CAPITAL_LETTER_SHA: char = 'Ш';
    /// \u{429}: 'Щ'
    pub const CAPITAL_LETTER_SHCHA: char = 'Щ';
    /// \u{42a}: 'Ъ'
    pub const CAPITAL_LETTER_HARD_SIGN: char = 'Ъ';
    /// \u{42b}: 'Ы'
    pub const CAPITAL_LETTER_YERU: char = 'Ы';
    /// \u{42c}: 'Ь'
    pub const CAPITAL_LETTER_SOFT_SIGN: char = 'Ь';
    /// \u{42d}: 'Э'
    pub const CAPITAL_LETTER_E: char = 'Э';
    /// \u{42e}: 'Ю'
    pub const CAPITAL_LETTER_YU: char = 'Ю';
    /// \u{42f}: 'Я'
    pub const CAPITAL_LETTER_YA: char = 'Я';
    /// \u{430}: 'а'
    pub const SMALL_LETTER_A: char = 'а';
    /// \u{431}: 'б'
    pub const SMALL_LETTER_BE: char = 'б';
    /// \u{432}: 'в'
    pub const SMALL_LETTER_VE: char = 'в';
    /// \u{433}: 'г'
    pub const SMALL_LETTER_GHE: char = 'г';
    /// \u{434}: 'д'
    pub const SMALL_LETTER_DE: char = 'д';
    /// \u{435}: 'е'
    pub const SMALL_LETTER_IE: char = 'е';
    /// \u{436}: 'ж'
    pub const SMALL_LETTER_ZHE: char = 'ж';
    /// \u{437}: 'з'
    pub const SMALL_LETTER_ZE: char = 'з';
    /// \u{438}: 'и'
    pub const SMALL_LETTER_I: char = 'и';
    /// \u{439}: 'й'
    pub const SMALL_LETTER_SHORT_I: char = 'й';
    /// \u{43a}: 'к'
    pub const SMALL_LETTER_KA: char = 'к';
    /// \u{43b}: 'л'
    pub const SMALL_LETTER_EL: char = 'л';
    /// \u{43c}: 'м'
    pub const SMALL_LETTER_EM: char = 'м';
    /// \u{43d}: 'н'
    pub const SMALL_LETTER_EN: char = 'н';
    /// \u{43e}: 'о'
    pub const SMALL_LETTER_O: char = 'о';
    /// \u{43f}: 'п'
    pub const SMALL_LETTER_PE: char = 'п';
    /// \u{440}: 'р'
    pub const SMALL_LETTER_ER: char = 'р';
    /// \u{441}: 'с'
    pub const SMALL_LETTER_ES: char = 'с';
    /// \u{442}: 'т'
    pub const SMALL_LETTER_TE: char = 'т';
    /// \u{443}: 'у'
    pub const SMALL_LETTER_U: char = 'у';
    /// \u{444}: 'ф'
    pub const SMALL_LETTER_EF: char = 'ф';
    /// \u{445}: 'х'
    pub const SMALL_LETTER_HA: char = 'х';
    /// \u{446}: 'ц'
    pub const SMALL_LETTER_TSE: char = 'ц';
    /// \u{447}: 'ч'
    pub const SMALL_LETTER_CHE: char = 'ч';
    /// \u{448}: 'ш'
    pub const SMALL_LETTER_SHA: char = 'ш';
    /// \u{449}: 'щ'
    pub const SMALL_LETTER_SHCHA: char = 'щ';
    /// \u{44a}: 'ъ'
    pub const SMALL_LETTER_HARD_SIGN: char = 'ъ';
    /// \u{44b}: 'ы'
    pub const SMALL_LETTER_YERU: char = 'ы';
    /// \u{44c}: 'ь'
    pub const SMALL_LETTER_SOFT_SIGN: char = 'ь';
    /// \u{44d}: 'э'
    pub const SMALL_LETTER_E: char = 'э';
    /// \u{44e}: 'ю'
    pub const SMALL_LETTER_YU: char = 'ю';
    /// \u{44f}: 'я'
    pub const SMALL_LETTER_YA: char = 'я';
    /// \u{450}: 'ѐ'
    pub const SMALL_LETTER_IE_WITH_GRAVE: char = 'ѐ';
    /// \u{451}: 'ё'
    pub const SMALL_LETTER_IO: char = 'ё';
    /// \u{452}: 'ђ'
    pub const SMALL_LETTER_DJE: char = 'ђ';
    /// \u{453}: 'ѓ'
    pub const SMALL_LETTER_GJE: char = 'ѓ';
    /// \u{454}: 'є'
    pub const SMALL_LETTER_UKRAINIAN_IE: char = 'є';
    /// \u{455}: 'ѕ'
    pub const SMALL_LETTER_DZE: char = 'ѕ';
    /// \u{456}: 'і'
    pub const SMALL_LETTER_BYELORUSSIAN_DASH_UKRAINIAN_I: char = 'і';
    /// \u{457}: 'ї'
    pub const SMALL_LETTER_YI: char = 'ї';
    /// \u{458}: 'ј'
    pub const SMALL_LETTER_JE: char = 'ј';
    /// \u{459}: 'љ'
    pub const SMALL_LETTER_LJE: char = 'љ';
    /// \u{45a}: 'њ'
    pub const SMALL_LETTER_NJE: char = 'њ';
    /// \u{45b}: 'ћ'
    pub const SMALL_LETTER_TSHE: char = 'ћ';
    /// \u{45c}: 'ќ'
    pub const SMALL_LETTER_KJE: char = 'ќ';
    /// \u{45d}: 'ѝ'
    pub const SMALL_LETTER_I_WITH_GRAVE: char = 'ѝ';
    /// \u{45e}: 'ў'
    pub const SMALL_LETTER_SHORT_U: char = 'ў';
    /// \u{45f}: 'џ'
    pub const SMALL_LETTER_DZHE: char = 'џ';
    /// \u{460}: 'Ѡ'
    pub const CAPITAL_LETTER_OMEGA: char = 'Ѡ';
    /// \u{461}: 'ѡ'
    pub const SMALL_LETTER_OMEGA: char = 'ѡ';
    /// \u{462}: 'Ѣ'
    pub const CAPITAL_LETTER_YAT: char = 'Ѣ';
    /// \u{463}: 'ѣ'
    pub const SMALL_LETTER_YAT: char = 'ѣ';
    /// \u{464}: 'Ѥ'
    pub const CAPITAL_LETTER_IOTIFIED_E: char = 'Ѥ';
    /// \u{465}: 'ѥ'
    pub const SMALL_LETTER_IOTIFIED_E: char = 'ѥ';
    /// \u{466}: 'Ѧ'
    pub const CAPITAL_LETTER_LITTLE_YUS: char = 'Ѧ';
    /// \u{467}: 'ѧ'
    pub const SMALL_LETTER_LITTLE_YUS: char = 'ѧ';
    /// \u{468}: 'Ѩ'
    pub const CAPITAL_LETTER_IOTIFIED_LITTLE_YUS: char = 'Ѩ';
    /// \u{469}: 'ѩ'
    pub const SMALL_LETTER_IOTIFIED_LITTLE_YUS: char = 'ѩ';
    /// \u{46a}: 'Ѫ'
    pub const CAPITAL_LETTER_BIG_YUS: char = 'Ѫ';
    /// \u{46b}: 'ѫ'
    pub const SMALL_LETTER_BIG_YUS: char = 'ѫ';
    /// \u{46c}: 'Ѭ'
    pub const CAPITAL_LETTER_IOTIFIED_BIG_YUS: char = 'Ѭ';
    /// \u{46d}: 'ѭ'
    pub const SMALL_LETTER_IOTIFIED_BIG_YUS: char = 'ѭ';
    /// \u{46e}: 'Ѯ'
    pub const CAPITAL_LETTER_KSI: char = 'Ѯ';
    /// \u{46f}: 'ѯ'
    pub const SMALL_LETTER_KSI: char = 'ѯ';
    /// \u{470}: 'Ѱ'
    pub const CAPITAL_LETTER_PSI: char = 'Ѱ';
    /// \u{471}: 'ѱ'
    pub const SMALL_LETTER_PSI: char = 'ѱ';
    /// \u{472}: 'Ѳ'
    pub const CAPITAL_LETTER_FITA: char = 'Ѳ';
    /// \u{473}: 'ѳ'
    pub const SMALL_LETTER_FITA: char = 'ѳ';
    /// \u{474}: 'Ѵ'
    pub const CAPITAL_LETTER_IZHITSA: char = 'Ѵ';
    /// \u{475}: 'ѵ'
    pub const SMALL_LETTER_IZHITSA: char = 'ѵ';
    /// \u{476}: 'Ѷ'
    pub const CAPITAL_LETTER_IZHITSA_WITH_DOUBLE_GRAVE_ACCENT: char = 'Ѷ';
    /// \u{477}: 'ѷ'
    pub const SMALL_LETTER_IZHITSA_WITH_DOUBLE_GRAVE_ACCENT: char = 'ѷ';
    /// \u{478}: 'Ѹ'
    pub const CAPITAL_LETTER_UK: char = 'Ѹ';
    /// \u{479}: 'ѹ'
    pub const SMALL_LETTER_UK: char = 'ѹ';
    /// \u{47a}: 'Ѻ'
    pub const CAPITAL_LETTER_ROUND_OMEGA: char = 'Ѻ';
    /// \u{47b}: 'ѻ'
    pub const SMALL_LETTER_ROUND_OMEGA: char = 'ѻ';
    /// \u{47c}: 'Ѽ'
    pub const CAPITAL_LETTER_OMEGA_WITH_TITLO: char = 'Ѽ';
    /// \u{47d}: 'ѽ'
    pub const SMALL_LETTER_OMEGA_WITH_TITLO: char = 'ѽ';
    /// \u{47e}: 'Ѿ'
    pub const CAPITAL_LETTER_OT: char = 'Ѿ';
    /// \u{47f}: 'ѿ'
    pub const SMALL_LETTER_OT: char = 'ѿ';
    /// \u{480}: 'Ҁ'
    pub const CAPITAL_LETTER_KOPPA: char = 'Ҁ';
    /// \u{481}: 'ҁ'
    pub const SMALL_LETTER_KOPPA: char = 'ҁ';
    /// \u{482}: '҂'
    pub const THOUSANDS_SIGN: char = '҂';
    /// \u{483}: '҃'
    pub const COMBINING_TITLO: char = '҃';
    /// \u{484}: '҄'
    pub const COMBINING_PALATALIZATION: char = '҄';
    /// \u{485}: '҅'
    pub const COMBINING_DASIA_PNEUMATA: char = '҅';
    /// \u{486}: '҆'
    pub const COMBINING_PSILI_PNEUMATA: char = '҆';
    /// \u{487}: '҇'
    pub const COMBINING_POKRYTIE: char = '҇';
    /// \u{488}: '҈'
    pub const COMBINING_HUNDRED_THOUSANDS_SIGN: char = '҈';
    /// \u{489}: '҉'
    pub const COMBINING_MILLIONS_SIGN: char = '҉';
    /// \u{48a}: 'Ҋ'
    pub const CAPITAL_LETTER_SHORT_I_WITH_TAIL: char = 'Ҋ';
    /// \u{48b}: 'ҋ'
    pub const SMALL_LETTER_SHORT_I_WITH_TAIL: char = 'ҋ';
    /// \u{48c}: 'Ҍ'
    pub const CAPITAL_LETTER_SEMISOFT_SIGN: char = 'Ҍ';
    /// \u{48d}: 'ҍ'
    pub const SMALL_LETTER_SEMISOFT_SIGN: char = 'ҍ';
    /// \u{48e}: 'Ҏ'
    pub const CAPITAL_LETTER_ER_WITH_TICK: char = 'Ҏ';
    /// \u{48f}: 'ҏ'
    pub const SMALL_LETTER_ER_WITH_TICK: char = 'ҏ';
    /// \u{490}: 'Ґ'
    pub const CAPITAL_LETTER_GHE_WITH_UPTURN: char = 'Ґ';
    /// \u{491}: 'ґ'
    pub const SMALL_LETTER_GHE_WITH_UPTURN: char = 'ґ';
    /// \u{492}: 'Ғ'
    pub const CAPITAL_LETTER_GHE_WITH_STROKE: char = 'Ғ';
    /// \u{493}: 'ғ'
    pub const SMALL_LETTER_GHE_WITH_STROKE: char = 'ғ';
    /// \u{494}: 'Ҕ'
    pub const CAPITAL_LETTER_GHE_WITH_MIDDLE_HOOK: char = 'Ҕ';
    /// \u{495}: 'ҕ'
    pub const SMALL_LETTER_GHE_WITH_MIDDLE_HOOK: char = 'ҕ';
    /// \u{496}: 'Җ'
    pub const CAPITAL_LETTER_ZHE_WITH_DESCENDER: char = 'Җ';
    /// \u{497}: 'җ'
    pub const SMALL_LETTER_ZHE_WITH_DESCENDER: char = 'җ';
    /// \u{498}: 'Ҙ'
    pub const CAPITAL_LETTER_ZE_WITH_DESCENDER: char = 'Ҙ';
    /// \u{499}: 'ҙ'
    pub const SMALL_LETTER_ZE_WITH_DESCENDER: char = 'ҙ';
    /// \u{49a}: 'Қ'
    pub const CAPITAL_LETTER_KA_WITH_DESCENDER: char = 'Қ';
    /// \u{49b}: 'қ'
    pub const SMALL_LETTER_KA_WITH_DESCENDER: char = 'қ';
    /// \u{49c}: 'Ҝ'
    pub const CAPITAL_LETTER_KA_WITH_VERTICAL_STROKE: char = 'Ҝ';
    /// \u{49d}: 'ҝ'
    pub const SMALL_LETTER_KA_WITH_VERTICAL_STROKE: char = 'ҝ';
    /// \u{49e}: 'Ҟ'
    pub const CAPITAL_LETTER_KA_WITH_STROKE: char = 'Ҟ';
    /// \u{49f}: 'ҟ'
    pub const SMALL_LETTER_KA_WITH_STROKE: char = 'ҟ';
    /// \u{4a0}: 'Ҡ'
    pub const CAPITAL_LETTER_BASHKIR_KA: char = 'Ҡ';
    /// \u{4a1}: 'ҡ'
    pub const SMALL_LETTER_BASHKIR_KA: char = 'ҡ';
    /// \u{4a2}: 'Ң'
    pub const CAPITAL_LETTER_EN_WITH_DESCENDER: char = 'Ң';
    /// \u{4a3}: 'ң'
    pub const SMALL_LETTER_EN_WITH_DESCENDER: char = 'ң';
    /// \u{4a4}: 'Ҥ'
    pub const CAPITAL_LIGATURE_EN_GHE: char = 'Ҥ';
    /// \u{4a5}: 'ҥ'
    pub const SMALL_LIGATURE_EN_GHE: char = 'ҥ';
    /// \u{4a6}: 'Ҧ'
    pub const CAPITAL_LETTER_PE_WITH_MIDDLE_HOOK: char = 'Ҧ';
    /// \u{4a7}: 'ҧ'
    pub const SMALL_LETTER_PE_WITH_MIDDLE_HOOK: char = 'ҧ';
    /// \u{4a8}: 'Ҩ'
    pub const CAPITAL_LETTER_ABKHASIAN_HA: char = 'Ҩ';
    /// \u{4a9}: 'ҩ'
    pub const SMALL_LETTER_ABKHASIAN_HA: char = 'ҩ';
    /// \u{4aa}: 'Ҫ'
    pub const CAPITAL_LETTER_ES_WITH_DESCENDER: char = 'Ҫ';
    /// \u{4ab}: 'ҫ'
    pub const SMALL_LETTER_ES_WITH_DESCENDER: char = 'ҫ';
    /// \u{4ac}: 'Ҭ'
    pub const CAPITAL_LETTER_TE_WITH_DESCENDER: char = 'Ҭ';
    /// \u{4ad}: 'ҭ'
    pub const SMALL_LETTER_TE_WITH_DESCENDER: char = 'ҭ';
    /// \u{4ae}: 'Ү'
    pub const CAPITAL_LETTER_STRAIGHT_U: char = 'Ү';
    /// \u{4af}: 'ү'
    pub const SMALL_LETTER_STRAIGHT_U: char = 'ү';
    /// \u{4b0}: 'Ұ'
    pub const CAPITAL_LETTER_STRAIGHT_U_WITH_STROKE: char = 'Ұ';
    /// \u{4b1}: 'ұ'
    pub const SMALL_LETTER_STRAIGHT_U_WITH_STROKE: char = 'ұ';
    /// \u{4b2}: 'Ҳ'
    pub const CAPITAL_LETTER_HA_WITH_DESCENDER: char = 'Ҳ';
    /// \u{4b3}: 'ҳ'
    pub const SMALL_LETTER_HA_WITH_DESCENDER: char = 'ҳ';
    /// \u{4b4}: 'Ҵ'
    pub const CAPITAL_LIGATURE_TE_TSE: char = 'Ҵ';
    /// \u{4b5}: 'ҵ'
    pub const SMALL_LIGATURE_TE_TSE: char = 'ҵ';
    /// \u{4b6}: 'Ҷ'
    pub const CAPITAL_LETTER_CHE_WITH_DESCENDER: char = 'Ҷ';
    /// \u{4b7}: 'ҷ'
    pub const SMALL_LETTER_CHE_WITH_DESCENDER: char = 'ҷ';
    /// \u{4b8}: 'Ҹ'
    pub const CAPITAL_LETTER_CHE_WITH_VERTICAL_STROKE: char = 'Ҹ';
    /// \u{4b9}: 'ҹ'
    pub const SMALL_LETTER_CHE_WITH_VERTICAL_STROKE: char = 'ҹ';
    /// \u{4ba}: 'Һ'
    pub const CAPITAL_LETTER_SHHA: char = 'Һ';
    /// \u{4bb}: 'һ'
    pub const SMALL_LETTER_SHHA: char = 'һ';
    /// \u{4bc}: 'Ҽ'
    pub const CAPITAL_LETTER_ABKHASIAN_CHE: char = 'Ҽ';
    /// \u{4bd}: 'ҽ'
    pub const SMALL_LETTER_ABKHASIAN_CHE: char = 'ҽ';
    /// \u{4be}: 'Ҿ'
    pub const CAPITAL_LETTER_ABKHASIAN_CHE_WITH_DESCENDER: char = 'Ҿ';
    /// \u{4bf}: 'ҿ'
    pub const SMALL_LETTER_ABKHASIAN_CHE_WITH_DESCENDER: char = 'ҿ';
    /// \u{4c0}: 'Ӏ'
    pub const LETTER_PALOCHKA: char = 'Ӏ';
    /// \u{4c1}: 'Ӂ'
    pub const CAPITAL_LETTER_ZHE_WITH_BREVE: char = 'Ӂ';
    /// \u{4c2}: 'ӂ'
    pub const SMALL_LETTER_ZHE_WITH_BREVE: char = 'ӂ';
    /// \u{4c3}: 'Ӄ'
    pub const CAPITAL_LETTER_KA_WITH_HOOK: char = 'Ӄ';
    /// \u{4c4}: 'ӄ'
    pub const SMALL_LETTER_KA_WITH_HOOK: char = 'ӄ';
    /// \u{4c5}: 'Ӆ'
    pub const CAPITAL_LETTER_EL_WITH_TAIL: char = 'Ӆ';
    /// \u{4c6}: 'ӆ'
    pub const SMALL_LETTER_EL_WITH_TAIL: char = 'ӆ';
    /// \u{4c7}: 'Ӈ'
    pub const CAPITAL_LETTER_EN_WITH_HOOK: char = 'Ӈ';
    /// \u{4c8}: 'ӈ'
    pub const SMALL_LETTER_EN_WITH_HOOK: char = 'ӈ';
    /// \u{4c9}: 'Ӊ'
    pub const CAPITAL_LETTER_EN_WITH_TAIL: char = 'Ӊ';
    /// \u{4ca}: 'ӊ'
    pub const SMALL_LETTER_EN_WITH_TAIL: char = 'ӊ';
    /// \u{4cb}: 'Ӌ'
    pub const CAPITAL_LETTER_KHAKASSIAN_CHE: char = 'Ӌ';
    /// \u{4cc}: 'ӌ'
    pub const SMALL_LETTER_KHAKASSIAN_CHE: char = 'ӌ';
    /// \u{4cd}: 'Ӎ'
    pub const CAPITAL_LETTER_EM_WITH_TAIL: char = 'Ӎ';
    /// \u{4ce}: 'ӎ'
    pub const SMALL_LETTER_EM_WITH_TAIL: char = 'ӎ';
    /// \u{4cf}: 'ӏ'
    pub const SMALL_LETTER_PALOCHKA: char = 'ӏ';
    /// \u{4d0}: 'Ӑ'
    pub const CAPITAL_LETTER_A_WITH_BREVE: char = 'Ӑ';
    /// \u{4d1}: 'ӑ'
    pub const SMALL_LETTER_A_WITH_BREVE: char = 'ӑ';
    /// \u{4d2}: 'Ӓ'
    pub const CAPITAL_LETTER_A_WITH_DIAERESIS: char = 'Ӓ';
    /// \u{4d3}: 'ӓ'
    pub const SMALL_LETTER_A_WITH_DIAERESIS: char = 'ӓ';
    /// \u{4d4}: 'Ӕ'
    pub const CAPITAL_LIGATURE_A_IE: char = 'Ӕ';
    /// \u{4d5}: 'ӕ'
    pub const SMALL_LIGATURE_A_IE: char = 'ӕ';
    /// \u{4d6}: 'Ӗ'
    pub const CAPITAL_LETTER_IE_WITH_BREVE: char = 'Ӗ';
    /// \u{4d7}: 'ӗ'
    pub const SMALL_LETTER_IE_WITH_BREVE: char = 'ӗ';
    /// \u{4d8}: 'Ә'
    pub const CAPITAL_LETTER_SCHWA: char = 'Ә';
    /// \u{4d9}: 'ә'
    pub const SMALL_LETTER_SCHWA: char = 'ә';
    /// \u{4da}: 'Ӛ'
    pub const CAPITAL_LETTER_SCHWA_WITH_DIAERESIS: char = 'Ӛ';
    /// \u{4db}: 'ӛ'
    pub const SMALL_LETTER_SCHWA_WITH_DIAERESIS: char = 'ӛ';
    /// \u{4dc}: 'Ӝ'
    pub const CAPITAL_LETTER_ZHE_WITH_DIAERESIS: char = 'Ӝ';
    /// \u{4dd}: 'ӝ'
    pub const SMALL_LETTER_ZHE_WITH_DIAERESIS: char = 'ӝ';
    /// \u{4de}: 'Ӟ'
    pub const CAPITAL_LETTER_ZE_WITH_DIAERESIS: char = 'Ӟ';
    /// \u{4df}: 'ӟ'
    pub const SMALL_LETTER_ZE_WITH_DIAERESIS: char = 'ӟ';
    /// \u{4e0}: 'Ӡ'
    pub const CAPITAL_LETTER_ABKHASIAN_DZE: char = 'Ӡ';
    /// \u{4e1}: 'ӡ'
    pub const SMALL_LETTER_ABKHASIAN_DZE: char = 'ӡ';
    /// \u{4e2}: 'Ӣ'
    pub const CAPITAL_LETTER_I_WITH_MACRON: char = 'Ӣ';
    /// \u{4e3}: 'ӣ'
    pub const SMALL_LETTER_I_WITH_MACRON: char = 'ӣ';
    /// \u{4e4}: 'Ӥ'
    pub const CAPITAL_LETTER_I_WITH_DIAERESIS: char = 'Ӥ';
    /// \u{4e5}: 'ӥ'
    pub const SMALL_LETTER_I_WITH_DIAERESIS: char = 'ӥ';
    /// \u{4e6}: 'Ӧ'
    pub const CAPITAL_LETTER_O_WITH_DIAERESIS: char = 'Ӧ';
    /// \u{4e7}: 'ӧ'
    pub const SMALL_LETTER_O_WITH_DIAERESIS: char = 'ӧ';
    /// \u{4e8}: 'Ө'
    pub const CAPITAL_LETTER_BARRED_O: char = 'Ө';
    /// \u{4e9}: 'ө'
    pub const SMALL_LETTER_BARRED_O: char = 'ө';
    /// \u{4ea}: 'Ӫ'
    pub const CAPITAL_LETTER_BARRED_O_WITH_DIAERESIS: char = 'Ӫ';
    /// \u{4eb}: 'ӫ'
    pub const SMALL_LETTER_BARRED_O_WITH_DIAERESIS: char = 'ӫ';
    /// \u{4ec}: 'Ӭ'
    pub const CAPITAL_LETTER_E_WITH_DIAERESIS: char = 'Ӭ';
    /// \u{4ed}: 'ӭ'
    pub const SMALL_LETTER_E_WITH_DIAERESIS: char = 'ӭ';
    /// \u{4ee}: 'Ӯ'
    pub const CAPITAL_LETTER_U_WITH_MACRON: char = 'Ӯ';
    /// \u{4ef}: 'ӯ'
    pub const SMALL_LETTER_U_WITH_MACRON: char = 'ӯ';
    /// \u{4f0}: 'Ӱ'
    pub const CAPITAL_LETTER_U_WITH_DIAERESIS: char = 'Ӱ';
    /// \u{4f1}: 'ӱ'
    pub const SMALL_LETTER_U_WITH_DIAERESIS: char = 'ӱ';
    /// \u{4f2}: 'Ӳ'
    pub const CAPITAL_LETTER_U_WITH_DOUBLE_ACUTE: char = 'Ӳ';
    /// \u{4f3}: 'ӳ'
    pub const SMALL_LETTER_U_WITH_DOUBLE_ACUTE: char = 'ӳ';
    /// \u{4f4}: 'Ӵ'
    pub const CAPITAL_LETTER_CHE_WITH_DIAERESIS: char = 'Ӵ';
    /// \u{4f5}: 'ӵ'
    pub const SMALL_LETTER_CHE_WITH_DIAERESIS: char = 'ӵ';
    /// \u{4f6}: 'Ӷ'
    pub const CAPITAL_LETTER_GHE_WITH_DESCENDER: char = 'Ӷ';
    /// \u{4f7}: 'ӷ'
    pub const SMALL_LETTER_GHE_WITH_DESCENDER: char = 'ӷ';
    /// \u{4f8}: 'Ӹ'
    pub const CAPITAL_LETTER_YERU_WITH_DIAERESIS: char = 'Ӹ';
    /// \u{4f9}: 'ӹ'
    pub const SMALL_LETTER_YERU_WITH_DIAERESIS: char = 'ӹ';
    /// \u{4fa}: 'Ӻ'
    pub const CAPITAL_LETTER_GHE_WITH_STROKE_AND_HOOK: char = 'Ӻ';
    /// \u{4fb}: 'ӻ'
    pub const SMALL_LETTER_GHE_WITH_STROKE_AND_HOOK: char = 'ӻ';
    /// \u{4fc}: 'Ӽ'
    pub const CAPITAL_LETTER_HA_WITH_HOOK: char = 'Ӽ';
    /// \u{4fd}: 'ӽ'
    pub const SMALL_LETTER_HA_WITH_HOOK: char = 'ӽ';
    /// \u{4fe}: 'Ӿ'
    pub const CAPITAL_LETTER_HA_WITH_STROKE: char = 'Ӿ';
}

/// An enum to represent all characters in the Cyrillic block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Cyrillic {
    /// \u{400}: 'Ѐ'
    CapitalLetterIeWithGrave,
    /// \u{401}: 'Ё'
    CapitalLetterIo,
    /// \u{402}: 'Ђ'
    CapitalLetterDje,
    /// \u{403}: 'Ѓ'
    CapitalLetterGje,
    /// \u{404}: 'Є'
    CapitalLetterUkrainianIe,
    /// \u{405}: 'Ѕ'
    CapitalLetterDze,
    /// \u{406}: 'І'
    CapitalLetterByelorussianDashUkrainianI,
    /// \u{407}: 'Ї'
    CapitalLetterYi,
    /// \u{408}: 'Ј'
    CapitalLetterJe,
    /// \u{409}: 'Љ'
    CapitalLetterLje,
    /// \u{40a}: 'Њ'
    CapitalLetterNje,
    /// \u{40b}: 'Ћ'
    CapitalLetterTshe,
    /// \u{40c}: 'Ќ'
    CapitalLetterKje,
    /// \u{40d}: 'Ѝ'
    CapitalLetterIWithGrave,
    /// \u{40e}: 'Ў'
    CapitalLetterShortU,
    /// \u{40f}: 'Џ'
    CapitalLetterDzhe,
    /// \u{410}: 'А'
    CapitalLetterA,
    /// \u{411}: 'Б'
    CapitalLetterBe,
    /// \u{412}: 'В'
    CapitalLetterVe,
    /// \u{413}: 'Г'
    CapitalLetterGhe,
    /// \u{414}: 'Д'
    CapitalLetterDe,
    /// \u{415}: 'Е'
    CapitalLetterIe,
    /// \u{416}: 'Ж'
    CapitalLetterZhe,
    /// \u{417}: 'З'
    CapitalLetterZe,
    /// \u{418}: 'И'
    CapitalLetterI,
    /// \u{419}: 'Й'
    CapitalLetterShortI,
    /// \u{41a}: 'К'
    CapitalLetterKa,
    /// \u{41b}: 'Л'
    CapitalLetterEl,
    /// \u{41c}: 'М'
    CapitalLetterEm,
    /// \u{41d}: 'Н'
    CapitalLetterEn,
    /// \u{41e}: 'О'
    CapitalLetterO,
    /// \u{41f}: 'П'
    CapitalLetterPe,
    /// \u{420}: 'Р'
    CapitalLetterEr,
    /// \u{421}: 'С'
    CapitalLetterEs,
    /// \u{422}: 'Т'
    CapitalLetterTe,
    /// \u{423}: 'У'
    CapitalLetterU,
    /// \u{424}: 'Ф'
    CapitalLetterEf,
    /// \u{425}: 'Х'
    CapitalLetterHa,
    /// \u{426}: 'Ц'
    CapitalLetterTse,
    /// \u{427}: 'Ч'
    CapitalLetterChe,
    /// \u{428}: 'Ш'
    CapitalLetterSha,
    /// \u{429}: 'Щ'
    CapitalLetterShcha,
    /// \u{42a}: 'Ъ'
    CapitalLetterHardSign,
    /// \u{42b}: 'Ы'
    CapitalLetterYeru,
    /// \u{42c}: 'Ь'
    CapitalLetterSoftSign,
    /// \u{42d}: 'Э'
    CapitalLetterE,
    /// \u{42e}: 'Ю'
    CapitalLetterYu,
    /// \u{42f}: 'Я'
    CapitalLetterYa,
    /// \u{430}: 'а'
    SmallLetterA,
    /// \u{431}: 'б'
    SmallLetterBe,
    /// \u{432}: 'в'
    SmallLetterVe,
    /// \u{433}: 'г'
    SmallLetterGhe,
    /// \u{434}: 'д'
    SmallLetterDe,
    /// \u{435}: 'е'
    SmallLetterIe,
    /// \u{436}: 'ж'
    SmallLetterZhe,
    /// \u{437}: 'з'
    SmallLetterZe,
    /// \u{438}: 'и'
    SmallLetterI,
    /// \u{439}: 'й'
    SmallLetterShortI,
    /// \u{43a}: 'к'
    SmallLetterKa,
    /// \u{43b}: 'л'
    SmallLetterEl,
    /// \u{43c}: 'м'
    SmallLetterEm,
    /// \u{43d}: 'н'
    SmallLetterEn,
    /// \u{43e}: 'о'
    SmallLetterO,
    /// \u{43f}: 'п'
    SmallLetterPe,
    /// \u{440}: 'р'
    SmallLetterEr,
    /// \u{441}: 'с'
    SmallLetterEs,
    /// \u{442}: 'т'
    SmallLetterTe,
    /// \u{443}: 'у'
    SmallLetterU,
    /// \u{444}: 'ф'
    SmallLetterEf,
    /// \u{445}: 'х'
    SmallLetterHa,
    /// \u{446}: 'ц'
    SmallLetterTse,
    /// \u{447}: 'ч'
    SmallLetterChe,
    /// \u{448}: 'ш'
    SmallLetterSha,
    /// \u{449}: 'щ'
    SmallLetterShcha,
    /// \u{44a}: 'ъ'
    SmallLetterHardSign,
    /// \u{44b}: 'ы'
    SmallLetterYeru,
    /// \u{44c}: 'ь'
    SmallLetterSoftSign,
    /// \u{44d}: 'э'
    SmallLetterE,
    /// \u{44e}: 'ю'
    SmallLetterYu,
    /// \u{44f}: 'я'
    SmallLetterYa,
    /// \u{450}: 'ѐ'
    SmallLetterIeWithGrave,
    /// \u{451}: 'ё'
    SmallLetterIo,
    /// \u{452}: 'ђ'
    SmallLetterDje,
    /// \u{453}: 'ѓ'
    SmallLetterGje,
    /// \u{454}: 'є'
    SmallLetterUkrainianIe,
    /// \u{455}: 'ѕ'
    SmallLetterDze,
    /// \u{456}: 'і'
    SmallLetterByelorussianDashUkrainianI,
    /// \u{457}: 'ї'
    SmallLetterYi,
    /// \u{458}: 'ј'
    SmallLetterJe,
    /// \u{459}: 'љ'
    SmallLetterLje,
    /// \u{45a}: 'њ'
    SmallLetterNje,
    /// \u{45b}: 'ћ'
    SmallLetterTshe,
    /// \u{45c}: 'ќ'
    SmallLetterKje,
    /// \u{45d}: 'ѝ'
    SmallLetterIWithGrave,
    /// \u{45e}: 'ў'
    SmallLetterShortU,
    /// \u{45f}: 'џ'
    SmallLetterDzhe,
    /// \u{460}: 'Ѡ'
    CapitalLetterOmega,
    /// \u{461}: 'ѡ'
    SmallLetterOmega,
    /// \u{462}: 'Ѣ'
    CapitalLetterYat,
    /// \u{463}: 'ѣ'
    SmallLetterYat,
    /// \u{464}: 'Ѥ'
    CapitalLetterIotifiedE,
    /// \u{465}: 'ѥ'
    SmallLetterIotifiedE,
    /// \u{466}: 'Ѧ'
    CapitalLetterLittleYus,
    /// \u{467}: 'ѧ'
    SmallLetterLittleYus,
    /// \u{468}: 'Ѩ'
    CapitalLetterIotifiedLittleYus,
    /// \u{469}: 'ѩ'
    SmallLetterIotifiedLittleYus,
    /// \u{46a}: 'Ѫ'
    CapitalLetterBigYus,
    /// \u{46b}: 'ѫ'
    SmallLetterBigYus,
    /// \u{46c}: 'Ѭ'
    CapitalLetterIotifiedBigYus,
    /// \u{46d}: 'ѭ'
    SmallLetterIotifiedBigYus,
    /// \u{46e}: 'Ѯ'
    CapitalLetterKsi,
    /// \u{46f}: 'ѯ'
    SmallLetterKsi,
    /// \u{470}: 'Ѱ'
    CapitalLetterPsi,
    /// \u{471}: 'ѱ'
    SmallLetterPsi,
    /// \u{472}: 'Ѳ'
    CapitalLetterFita,
    /// \u{473}: 'ѳ'
    SmallLetterFita,
    /// \u{474}: 'Ѵ'
    CapitalLetterIzhitsa,
    /// \u{475}: 'ѵ'
    SmallLetterIzhitsa,
    /// \u{476}: 'Ѷ'
    CapitalLetterIzhitsaWithDoubleGraveAccent,
    /// \u{477}: 'ѷ'
    SmallLetterIzhitsaWithDoubleGraveAccent,
    /// \u{478}: 'Ѹ'
    CapitalLetterUk,
    /// \u{479}: 'ѹ'
    SmallLetterUk,
    /// \u{47a}: 'Ѻ'
    CapitalLetterRoundOmega,
    /// \u{47b}: 'ѻ'
    SmallLetterRoundOmega,
    /// \u{47c}: 'Ѽ'
    CapitalLetterOmegaWithTitlo,
    /// \u{47d}: 'ѽ'
    SmallLetterOmegaWithTitlo,
    /// \u{47e}: 'Ѿ'
    CapitalLetterOt,
    /// \u{47f}: 'ѿ'
    SmallLetterOt,
    /// \u{480}: 'Ҁ'
    CapitalLetterKoppa,
    /// \u{481}: 'ҁ'
    SmallLetterKoppa,
    /// \u{482}: '҂'
    ThousandsSign,
    /// \u{483}: '҃'
    CombiningTitlo,
    /// \u{484}: '҄'
    CombiningPalatalization,
    /// \u{485}: '҅'
    CombiningDasiaPneumata,
    /// \u{486}: '҆'
    CombiningPsiliPneumata,
    /// \u{487}: '҇'
    CombiningPokrytie,
    /// \u{488}: '҈'
    CombiningHundredThousandsSign,
    /// \u{489}: '҉'
    CombiningMillionsSign,
    /// \u{48a}: 'Ҋ'
    CapitalLetterShortIWithTail,
    /// \u{48b}: 'ҋ'
    SmallLetterShortIWithTail,
    /// \u{48c}: 'Ҍ'
    CapitalLetterSemisoftSign,
    /// \u{48d}: 'ҍ'
    SmallLetterSemisoftSign,
    /// \u{48e}: 'Ҏ'
    CapitalLetterErWithTick,
    /// \u{48f}: 'ҏ'
    SmallLetterErWithTick,
    /// \u{490}: 'Ґ'
    CapitalLetterGheWithUpturn,
    /// \u{491}: 'ґ'
    SmallLetterGheWithUpturn,
    /// \u{492}: 'Ғ'
    CapitalLetterGheWithStroke,
    /// \u{493}: 'ғ'
    SmallLetterGheWithStroke,
    /// \u{494}: 'Ҕ'
    CapitalLetterGheWithMiddleHook,
    /// \u{495}: 'ҕ'
    SmallLetterGheWithMiddleHook,
    /// \u{496}: 'Җ'
    CapitalLetterZheWithDescender,
    /// \u{497}: 'җ'
    SmallLetterZheWithDescender,
    /// \u{498}: 'Ҙ'
    CapitalLetterZeWithDescender,
    /// \u{499}: 'ҙ'
    SmallLetterZeWithDescender,
    /// \u{49a}: 'Қ'
    CapitalLetterKaWithDescender,
    /// \u{49b}: 'қ'
    SmallLetterKaWithDescender,
    /// \u{49c}: 'Ҝ'
    CapitalLetterKaWithVerticalStroke,
    /// \u{49d}: 'ҝ'
    SmallLetterKaWithVerticalStroke,
    /// \u{49e}: 'Ҟ'
    CapitalLetterKaWithStroke,
    /// \u{49f}: 'ҟ'
    SmallLetterKaWithStroke,
    /// \u{4a0}: 'Ҡ'
    CapitalLetterBashkirKa,
    /// \u{4a1}: 'ҡ'
    SmallLetterBashkirKa,
    /// \u{4a2}: 'Ң'
    CapitalLetterEnWithDescender,
    /// \u{4a3}: 'ң'
    SmallLetterEnWithDescender,
    /// \u{4a4}: 'Ҥ'
    CapitalLigatureEnGhe,
    /// \u{4a5}: 'ҥ'
    SmallLigatureEnGhe,
    /// \u{4a6}: 'Ҧ'
    CapitalLetterPeWithMiddleHook,
    /// \u{4a7}: 'ҧ'
    SmallLetterPeWithMiddleHook,
    /// \u{4a8}: 'Ҩ'
    CapitalLetterAbkhasianHa,
    /// \u{4a9}: 'ҩ'
    SmallLetterAbkhasianHa,
    /// \u{4aa}: 'Ҫ'
    CapitalLetterEsWithDescender,
    /// \u{4ab}: 'ҫ'
    SmallLetterEsWithDescender,
    /// \u{4ac}: 'Ҭ'
    CapitalLetterTeWithDescender,
    /// \u{4ad}: 'ҭ'
    SmallLetterTeWithDescender,
    /// \u{4ae}: 'Ү'
    CapitalLetterStraightU,
    /// \u{4af}: 'ү'
    SmallLetterStraightU,
    /// \u{4b0}: 'Ұ'
    CapitalLetterStraightUWithStroke,
    /// \u{4b1}: 'ұ'
    SmallLetterStraightUWithStroke,
    /// \u{4b2}: 'Ҳ'
    CapitalLetterHaWithDescender,
    /// \u{4b3}: 'ҳ'
    SmallLetterHaWithDescender,
    /// \u{4b4}: 'Ҵ'
    CapitalLigatureTeTse,
    /// \u{4b5}: 'ҵ'
    SmallLigatureTeTse,
    /// \u{4b6}: 'Ҷ'
    CapitalLetterCheWithDescender,
    /// \u{4b7}: 'ҷ'
    SmallLetterCheWithDescender,
    /// \u{4b8}: 'Ҹ'
    CapitalLetterCheWithVerticalStroke,
    /// \u{4b9}: 'ҹ'
    SmallLetterCheWithVerticalStroke,
    /// \u{4ba}: 'Һ'
    CapitalLetterShha,
    /// \u{4bb}: 'һ'
    SmallLetterShha,
    /// \u{4bc}: 'Ҽ'
    CapitalLetterAbkhasianChe,
    /// \u{4bd}: 'ҽ'
    SmallLetterAbkhasianChe,
    /// \u{4be}: 'Ҿ'
    CapitalLetterAbkhasianCheWithDescender,
    /// \u{4bf}: 'ҿ'
    SmallLetterAbkhasianCheWithDescender,
    /// \u{4c0}: 'Ӏ'
    LetterPalochka,
    /// \u{4c1}: 'Ӂ'
    CapitalLetterZheWithBreve,
    /// \u{4c2}: 'ӂ'
    SmallLetterZheWithBreve,
    /// \u{4c3}: 'Ӄ'
    CapitalLetterKaWithHook,
    /// \u{4c4}: 'ӄ'
    SmallLetterKaWithHook,
    /// \u{4c5}: 'Ӆ'
    CapitalLetterElWithTail,
    /// \u{4c6}: 'ӆ'
    SmallLetterElWithTail,
    /// \u{4c7}: 'Ӈ'
    CapitalLetterEnWithHook,
    /// \u{4c8}: 'ӈ'
    SmallLetterEnWithHook,
    /// \u{4c9}: 'Ӊ'
    CapitalLetterEnWithTail,
    /// \u{4ca}: 'ӊ'
    SmallLetterEnWithTail,
    /// \u{4cb}: 'Ӌ'
    CapitalLetterKhakassianChe,
    /// \u{4cc}: 'ӌ'
    SmallLetterKhakassianChe,
    /// \u{4cd}: 'Ӎ'
    CapitalLetterEmWithTail,
    /// \u{4ce}: 'ӎ'
    SmallLetterEmWithTail,
    /// \u{4cf}: 'ӏ'
    SmallLetterPalochka,
    /// \u{4d0}: 'Ӑ'
    CapitalLetterAWithBreve,
    /// \u{4d1}: 'ӑ'
    SmallLetterAWithBreve,
    /// \u{4d2}: 'Ӓ'
    CapitalLetterAWithDiaeresis,
    /// \u{4d3}: 'ӓ'
    SmallLetterAWithDiaeresis,
    /// \u{4d4}: 'Ӕ'
    CapitalLigatureAIe,
    /// \u{4d5}: 'ӕ'
    SmallLigatureAIe,
    /// \u{4d6}: 'Ӗ'
    CapitalLetterIeWithBreve,
    /// \u{4d7}: 'ӗ'
    SmallLetterIeWithBreve,
    /// \u{4d8}: 'Ә'
    CapitalLetterSchwa,
    /// \u{4d9}: 'ә'
    SmallLetterSchwa,
    /// \u{4da}: 'Ӛ'
    CapitalLetterSchwaWithDiaeresis,
    /// \u{4db}: 'ӛ'
    SmallLetterSchwaWithDiaeresis,
    /// \u{4dc}: 'Ӝ'
    CapitalLetterZheWithDiaeresis,
    /// \u{4dd}: 'ӝ'
    SmallLetterZheWithDiaeresis,
    /// \u{4de}: 'Ӟ'
    CapitalLetterZeWithDiaeresis,
    /// \u{4df}: 'ӟ'
    SmallLetterZeWithDiaeresis,
    /// \u{4e0}: 'Ӡ'
    CapitalLetterAbkhasianDze,
    /// \u{4e1}: 'ӡ'
    SmallLetterAbkhasianDze,
    /// \u{4e2}: 'Ӣ'
    CapitalLetterIWithMacron,
    /// \u{4e3}: 'ӣ'
    SmallLetterIWithMacron,
    /// \u{4e4}: 'Ӥ'
    CapitalLetterIWithDiaeresis,
    /// \u{4e5}: 'ӥ'
    SmallLetterIWithDiaeresis,
    /// \u{4e6}: 'Ӧ'
    CapitalLetterOWithDiaeresis,
    /// \u{4e7}: 'ӧ'
    SmallLetterOWithDiaeresis,
    /// \u{4e8}: 'Ө'
    CapitalLetterBarredO,
    /// \u{4e9}: 'ө'
    SmallLetterBarredO,
    /// \u{4ea}: 'Ӫ'
    CapitalLetterBarredOWithDiaeresis,
    /// \u{4eb}: 'ӫ'
    SmallLetterBarredOWithDiaeresis,
    /// \u{4ec}: 'Ӭ'
    CapitalLetterEWithDiaeresis,
    /// \u{4ed}: 'ӭ'
    SmallLetterEWithDiaeresis,
    /// \u{4ee}: 'Ӯ'
    CapitalLetterUWithMacron,
    /// \u{4ef}: 'ӯ'
    SmallLetterUWithMacron,
    /// \u{4f0}: 'Ӱ'
    CapitalLetterUWithDiaeresis,
    /// \u{4f1}: 'ӱ'
    SmallLetterUWithDiaeresis,
    /// \u{4f2}: 'Ӳ'
    CapitalLetterUWithDoubleAcute,
    /// \u{4f3}: 'ӳ'
    SmallLetterUWithDoubleAcute,
    /// \u{4f4}: 'Ӵ'
    CapitalLetterCheWithDiaeresis,
    /// \u{4f5}: 'ӵ'
    SmallLetterCheWithDiaeresis,
    /// \u{4f6}: 'Ӷ'
    CapitalLetterGheWithDescender,
    /// \u{4f7}: 'ӷ'
    SmallLetterGheWithDescender,
    /// \u{4f8}: 'Ӹ'
    CapitalLetterYeruWithDiaeresis,
    /// \u{4f9}: 'ӹ'
    SmallLetterYeruWithDiaeresis,
    /// \u{4fa}: 'Ӻ'
    CapitalLetterGheWithStrokeAndHook,
    /// \u{4fb}: 'ӻ'
    SmallLetterGheWithStrokeAndHook,
    /// \u{4fc}: 'Ӽ'
    CapitalLetterHaWithHook,
    /// \u{4fd}: 'ӽ'
    SmallLetterHaWithHook,
    /// \u{4fe}: 'Ӿ'
    CapitalLetterHaWithStroke,
}

impl Into<char> for Cyrillic {
    fn into(self) -> char {
        use constants::*;
        match self {
            Cyrillic::CapitalLetterIeWithGrave => CAPITAL_LETTER_IE_WITH_GRAVE,
            Cyrillic::CapitalLetterIo => CAPITAL_LETTER_IO,
            Cyrillic::CapitalLetterDje => CAPITAL_LETTER_DJE,
            Cyrillic::CapitalLetterGje => CAPITAL_LETTER_GJE,
            Cyrillic::CapitalLetterUkrainianIe => CAPITAL_LETTER_UKRAINIAN_IE,
            Cyrillic::CapitalLetterDze => CAPITAL_LETTER_DZE,
            Cyrillic::CapitalLetterByelorussianDashUkrainianI => CAPITAL_LETTER_BYELORUSSIAN_DASH_UKRAINIAN_I,
            Cyrillic::CapitalLetterYi => CAPITAL_LETTER_YI,
            Cyrillic::CapitalLetterJe => CAPITAL_LETTER_JE,
            Cyrillic::CapitalLetterLje => CAPITAL_LETTER_LJE,
            Cyrillic::CapitalLetterNje => CAPITAL_LETTER_NJE,
            Cyrillic::CapitalLetterTshe => CAPITAL_LETTER_TSHE,
            Cyrillic::CapitalLetterKje => CAPITAL_LETTER_KJE,
            Cyrillic::CapitalLetterIWithGrave => CAPITAL_LETTER_I_WITH_GRAVE,
            Cyrillic::CapitalLetterShortU => CAPITAL_LETTER_SHORT_U,
            Cyrillic::CapitalLetterDzhe => CAPITAL_LETTER_DZHE,
            Cyrillic::CapitalLetterA => CAPITAL_LETTER_A,
            Cyrillic::CapitalLetterBe => CAPITAL_LETTER_BE,
            Cyrillic::CapitalLetterVe => CAPITAL_LETTER_VE,
            Cyrillic::CapitalLetterGhe => CAPITAL_LETTER_GHE,
            Cyrillic::CapitalLetterDe => CAPITAL_LETTER_DE,
            Cyrillic::CapitalLetterIe => CAPITAL_LETTER_IE,
            Cyrillic::CapitalLetterZhe => CAPITAL_LETTER_ZHE,
            Cyrillic::CapitalLetterZe => CAPITAL_LETTER_ZE,
            Cyrillic::CapitalLetterI => CAPITAL_LETTER_I,
            Cyrillic::CapitalLetterShortI => CAPITAL_LETTER_SHORT_I,
            Cyrillic::CapitalLetterKa => CAPITAL_LETTER_KA,
            Cyrillic::CapitalLetterEl => CAPITAL_LETTER_EL,
            Cyrillic::CapitalLetterEm => CAPITAL_LETTER_EM,
            Cyrillic::CapitalLetterEn => CAPITAL_LETTER_EN,
            Cyrillic::CapitalLetterO => CAPITAL_LETTER_O,
            Cyrillic::CapitalLetterPe => CAPITAL_LETTER_PE,
            Cyrillic::CapitalLetterEr => CAPITAL_LETTER_ER,
            Cyrillic::CapitalLetterEs => CAPITAL_LETTER_ES,
            Cyrillic::CapitalLetterTe => CAPITAL_LETTER_TE,
            Cyrillic::CapitalLetterU => CAPITAL_LETTER_U,
            Cyrillic::CapitalLetterEf => CAPITAL_LETTER_EF,
            Cyrillic::CapitalLetterHa => CAPITAL_LETTER_HA,
            Cyrillic::CapitalLetterTse => CAPITAL_LETTER_TSE,
            Cyrillic::CapitalLetterChe => CAPITAL_LETTER_CHE,
            Cyrillic::CapitalLetterSha => CAPITAL_LETTER_SHA,
            Cyrillic::CapitalLetterShcha => CAPITAL_LETTER_SHCHA,
            Cyrillic::CapitalLetterHardSign => CAPITAL_LETTER_HARD_SIGN,
            Cyrillic::CapitalLetterYeru => CAPITAL_LETTER_YERU,
            Cyrillic::CapitalLetterSoftSign => CAPITAL_LETTER_SOFT_SIGN,
            Cyrillic::CapitalLetterE => CAPITAL_LETTER_E,
            Cyrillic::CapitalLetterYu => CAPITAL_LETTER_YU,
            Cyrillic::CapitalLetterYa => CAPITAL_LETTER_YA,
            Cyrillic::SmallLetterA => SMALL_LETTER_A,
            Cyrillic::SmallLetterBe => SMALL_LETTER_BE,
            Cyrillic::SmallLetterVe => SMALL_LETTER_VE,
            Cyrillic::SmallLetterGhe => SMALL_LETTER_GHE,
            Cyrillic::SmallLetterDe => SMALL_LETTER_DE,
            Cyrillic::SmallLetterIe => SMALL_LETTER_IE,
            Cyrillic::SmallLetterZhe => SMALL_LETTER_ZHE,
            Cyrillic::SmallLetterZe => SMALL_LETTER_ZE,
            Cyrillic::SmallLetterI => SMALL_LETTER_I,
            Cyrillic::SmallLetterShortI => SMALL_LETTER_SHORT_I,
            Cyrillic::SmallLetterKa => SMALL_LETTER_KA,
            Cyrillic::SmallLetterEl => SMALL_LETTER_EL,
            Cyrillic::SmallLetterEm => SMALL_LETTER_EM,
            Cyrillic::SmallLetterEn => SMALL_LETTER_EN,
            Cyrillic::SmallLetterO => SMALL_LETTER_O,
            Cyrillic::SmallLetterPe => SMALL_LETTER_PE,
            Cyrillic::SmallLetterEr => SMALL_LETTER_ER,
            Cyrillic::SmallLetterEs => SMALL_LETTER_ES,
            Cyrillic::SmallLetterTe => SMALL_LETTER_TE,
            Cyrillic::SmallLetterU => SMALL_LETTER_U,
            Cyrillic::SmallLetterEf => SMALL_LETTER_EF,
            Cyrillic::SmallLetterHa => SMALL_LETTER_HA,
            Cyrillic::SmallLetterTse => SMALL_LETTER_TSE,
            Cyrillic::SmallLetterChe => SMALL_LETTER_CHE,
            Cyrillic::SmallLetterSha => SMALL_LETTER_SHA,
            Cyrillic::SmallLetterShcha => SMALL_LETTER_SHCHA,
            Cyrillic::SmallLetterHardSign => SMALL_LETTER_HARD_SIGN,
            Cyrillic::SmallLetterYeru => SMALL_LETTER_YERU,
            Cyrillic::SmallLetterSoftSign => SMALL_LETTER_SOFT_SIGN,
            Cyrillic::SmallLetterE => SMALL_LETTER_E,
            Cyrillic::SmallLetterYu => SMALL_LETTER_YU,
            Cyrillic::SmallLetterYa => SMALL_LETTER_YA,
            Cyrillic::SmallLetterIeWithGrave => SMALL_LETTER_IE_WITH_GRAVE,
            Cyrillic::SmallLetterIo => SMALL_LETTER_IO,
            Cyrillic::SmallLetterDje => SMALL_LETTER_DJE,
            Cyrillic::SmallLetterGje => SMALL_LETTER_GJE,
            Cyrillic::SmallLetterUkrainianIe => SMALL_LETTER_UKRAINIAN_IE,
            Cyrillic::SmallLetterDze => SMALL_LETTER_DZE,
            Cyrillic::SmallLetterByelorussianDashUkrainianI => SMALL_LETTER_BYELORUSSIAN_DASH_UKRAINIAN_I,
            Cyrillic::SmallLetterYi => SMALL_LETTER_YI,
            Cyrillic::SmallLetterJe => SMALL_LETTER_JE,
            Cyrillic::SmallLetterLje => SMALL_LETTER_LJE,
            Cyrillic::SmallLetterNje => SMALL_LETTER_NJE,
            Cyrillic::SmallLetterTshe => SMALL_LETTER_TSHE,
            Cyrillic::SmallLetterKje => SMALL_LETTER_KJE,
            Cyrillic::SmallLetterIWithGrave => SMALL_LETTER_I_WITH_GRAVE,
            Cyrillic::SmallLetterShortU => SMALL_LETTER_SHORT_U,
            Cyrillic::SmallLetterDzhe => SMALL_LETTER_DZHE,
            Cyrillic::CapitalLetterOmega => CAPITAL_LETTER_OMEGA,
            Cyrillic::SmallLetterOmega => SMALL_LETTER_OMEGA,
            Cyrillic::CapitalLetterYat => CAPITAL_LETTER_YAT,
            Cyrillic::SmallLetterYat => SMALL_LETTER_YAT,
            Cyrillic::CapitalLetterIotifiedE => CAPITAL_LETTER_IOTIFIED_E,
            Cyrillic::SmallLetterIotifiedE => SMALL_LETTER_IOTIFIED_E,
            Cyrillic::CapitalLetterLittleYus => CAPITAL_LETTER_LITTLE_YUS,
            Cyrillic::SmallLetterLittleYus => SMALL_LETTER_LITTLE_YUS,
            Cyrillic::CapitalLetterIotifiedLittleYus => CAPITAL_LETTER_IOTIFIED_LITTLE_YUS,
            Cyrillic::SmallLetterIotifiedLittleYus => SMALL_LETTER_IOTIFIED_LITTLE_YUS,
            Cyrillic::CapitalLetterBigYus => CAPITAL_LETTER_BIG_YUS,
            Cyrillic::SmallLetterBigYus => SMALL_LETTER_BIG_YUS,
            Cyrillic::CapitalLetterIotifiedBigYus => CAPITAL_LETTER_IOTIFIED_BIG_YUS,
            Cyrillic::SmallLetterIotifiedBigYus => SMALL_LETTER_IOTIFIED_BIG_YUS,
            Cyrillic::CapitalLetterKsi => CAPITAL_LETTER_KSI,
            Cyrillic::SmallLetterKsi => SMALL_LETTER_KSI,
            Cyrillic::CapitalLetterPsi => CAPITAL_LETTER_PSI,
            Cyrillic::SmallLetterPsi => SMALL_LETTER_PSI,
            Cyrillic::CapitalLetterFita => CAPITAL_LETTER_FITA,
            Cyrillic::SmallLetterFita => SMALL_LETTER_FITA,
            Cyrillic::CapitalLetterIzhitsa => CAPITAL_LETTER_IZHITSA,
            Cyrillic::SmallLetterIzhitsa => SMALL_LETTER_IZHITSA,
            Cyrillic::CapitalLetterIzhitsaWithDoubleGraveAccent => CAPITAL_LETTER_IZHITSA_WITH_DOUBLE_GRAVE_ACCENT,
            Cyrillic::SmallLetterIzhitsaWithDoubleGraveAccent => SMALL_LETTER_IZHITSA_WITH_DOUBLE_GRAVE_ACCENT,
            Cyrillic::CapitalLetterUk => CAPITAL_LETTER_UK,
            Cyrillic::SmallLetterUk => SMALL_LETTER_UK,
            Cyrillic::CapitalLetterRoundOmega => CAPITAL_LETTER_ROUND_OMEGA,
            Cyrillic::SmallLetterRoundOmega => SMALL_LETTER_ROUND_OMEGA,
            Cyrillic::CapitalLetterOmegaWithTitlo => CAPITAL_LETTER_OMEGA_WITH_TITLO,
            Cyrillic::SmallLetterOmegaWithTitlo => SMALL_LETTER_OMEGA_WITH_TITLO,
            Cyrillic::CapitalLetterOt => CAPITAL_LETTER_OT,
            Cyrillic::SmallLetterOt => SMALL_LETTER_OT,
            Cyrillic::CapitalLetterKoppa => CAPITAL_LETTER_KOPPA,
            Cyrillic::SmallLetterKoppa => SMALL_LETTER_KOPPA,
            Cyrillic::ThousandsSign => THOUSANDS_SIGN,
            Cyrillic::CombiningTitlo => COMBINING_TITLO,
            Cyrillic::CombiningPalatalization => COMBINING_PALATALIZATION,
            Cyrillic::CombiningDasiaPneumata => COMBINING_DASIA_PNEUMATA,
            Cyrillic::CombiningPsiliPneumata => COMBINING_PSILI_PNEUMATA,
            Cyrillic::CombiningPokrytie => COMBINING_POKRYTIE,
            Cyrillic::CombiningHundredThousandsSign => COMBINING_HUNDRED_THOUSANDS_SIGN,
            Cyrillic::CombiningMillionsSign => COMBINING_MILLIONS_SIGN,
            Cyrillic::CapitalLetterShortIWithTail => CAPITAL_LETTER_SHORT_I_WITH_TAIL,
            Cyrillic::SmallLetterShortIWithTail => SMALL_LETTER_SHORT_I_WITH_TAIL,
            Cyrillic::CapitalLetterSemisoftSign => CAPITAL_LETTER_SEMISOFT_SIGN,
            Cyrillic::SmallLetterSemisoftSign => SMALL_LETTER_SEMISOFT_SIGN,
            Cyrillic::CapitalLetterErWithTick => CAPITAL_LETTER_ER_WITH_TICK,
            Cyrillic::SmallLetterErWithTick => SMALL_LETTER_ER_WITH_TICK,
            Cyrillic::CapitalLetterGheWithUpturn => CAPITAL_LETTER_GHE_WITH_UPTURN,
            Cyrillic::SmallLetterGheWithUpturn => SMALL_LETTER_GHE_WITH_UPTURN,
            Cyrillic::CapitalLetterGheWithStroke => CAPITAL_LETTER_GHE_WITH_STROKE,
            Cyrillic::SmallLetterGheWithStroke => SMALL_LETTER_GHE_WITH_STROKE,
            Cyrillic::CapitalLetterGheWithMiddleHook => CAPITAL_LETTER_GHE_WITH_MIDDLE_HOOK,
            Cyrillic::SmallLetterGheWithMiddleHook => SMALL_LETTER_GHE_WITH_MIDDLE_HOOK,
            Cyrillic::CapitalLetterZheWithDescender => CAPITAL_LETTER_ZHE_WITH_DESCENDER,
            Cyrillic::SmallLetterZheWithDescender => SMALL_LETTER_ZHE_WITH_DESCENDER,
            Cyrillic::CapitalLetterZeWithDescender => CAPITAL_LETTER_ZE_WITH_DESCENDER,
            Cyrillic::SmallLetterZeWithDescender => SMALL_LETTER_ZE_WITH_DESCENDER,
            Cyrillic::CapitalLetterKaWithDescender => CAPITAL_LETTER_KA_WITH_DESCENDER,
            Cyrillic::SmallLetterKaWithDescender => SMALL_LETTER_KA_WITH_DESCENDER,
            Cyrillic::CapitalLetterKaWithVerticalStroke => CAPITAL_LETTER_KA_WITH_VERTICAL_STROKE,
            Cyrillic::SmallLetterKaWithVerticalStroke => SMALL_LETTER_KA_WITH_VERTICAL_STROKE,
            Cyrillic::CapitalLetterKaWithStroke => CAPITAL_LETTER_KA_WITH_STROKE,
            Cyrillic::SmallLetterKaWithStroke => SMALL_LETTER_KA_WITH_STROKE,
            Cyrillic::CapitalLetterBashkirKa => CAPITAL_LETTER_BASHKIR_KA,
            Cyrillic::SmallLetterBashkirKa => SMALL_LETTER_BASHKIR_KA,
            Cyrillic::CapitalLetterEnWithDescender => CAPITAL_LETTER_EN_WITH_DESCENDER,
            Cyrillic::SmallLetterEnWithDescender => SMALL_LETTER_EN_WITH_DESCENDER,
            Cyrillic::CapitalLigatureEnGhe => CAPITAL_LIGATURE_EN_GHE,
            Cyrillic::SmallLigatureEnGhe => SMALL_LIGATURE_EN_GHE,
            Cyrillic::CapitalLetterPeWithMiddleHook => CAPITAL_LETTER_PE_WITH_MIDDLE_HOOK,
            Cyrillic::SmallLetterPeWithMiddleHook => SMALL_LETTER_PE_WITH_MIDDLE_HOOK,
            Cyrillic::CapitalLetterAbkhasianHa => CAPITAL_LETTER_ABKHASIAN_HA,
            Cyrillic::SmallLetterAbkhasianHa => SMALL_LETTER_ABKHASIAN_HA,
            Cyrillic::CapitalLetterEsWithDescender => CAPITAL_LETTER_ES_WITH_DESCENDER,
            Cyrillic::SmallLetterEsWithDescender => SMALL_LETTER_ES_WITH_DESCENDER,
            Cyrillic::CapitalLetterTeWithDescender => CAPITAL_LETTER_TE_WITH_DESCENDER,
            Cyrillic::SmallLetterTeWithDescender => SMALL_LETTER_TE_WITH_DESCENDER,
            Cyrillic::CapitalLetterStraightU => CAPITAL_LETTER_STRAIGHT_U,
            Cyrillic::SmallLetterStraightU => SMALL_LETTER_STRAIGHT_U,
            Cyrillic::CapitalLetterStraightUWithStroke => CAPITAL_LETTER_STRAIGHT_U_WITH_STROKE,
            Cyrillic::SmallLetterStraightUWithStroke => SMALL_LETTER_STRAIGHT_U_WITH_STROKE,
            Cyrillic::CapitalLetterHaWithDescender => CAPITAL_LETTER_HA_WITH_DESCENDER,
            Cyrillic::SmallLetterHaWithDescender => SMALL_LETTER_HA_WITH_DESCENDER,
            Cyrillic::CapitalLigatureTeTse => CAPITAL_LIGATURE_TE_TSE,
            Cyrillic::SmallLigatureTeTse => SMALL_LIGATURE_TE_TSE,
            Cyrillic::CapitalLetterCheWithDescender => CAPITAL_LETTER_CHE_WITH_DESCENDER,
            Cyrillic::SmallLetterCheWithDescender => SMALL_LETTER_CHE_WITH_DESCENDER,
            Cyrillic::CapitalLetterCheWithVerticalStroke => CAPITAL_LETTER_CHE_WITH_VERTICAL_STROKE,
            Cyrillic::SmallLetterCheWithVerticalStroke => SMALL_LETTER_CHE_WITH_VERTICAL_STROKE,
            Cyrillic::CapitalLetterShha => CAPITAL_LETTER_SHHA,
            Cyrillic::SmallLetterShha => SMALL_LETTER_SHHA,
            Cyrillic::CapitalLetterAbkhasianChe => CAPITAL_LETTER_ABKHASIAN_CHE,
            Cyrillic::SmallLetterAbkhasianChe => SMALL_LETTER_ABKHASIAN_CHE,
            Cyrillic::CapitalLetterAbkhasianCheWithDescender => CAPITAL_LETTER_ABKHASIAN_CHE_WITH_DESCENDER,
            Cyrillic::SmallLetterAbkhasianCheWithDescender => SMALL_LETTER_ABKHASIAN_CHE_WITH_DESCENDER,
            Cyrillic::LetterPalochka => LETTER_PALOCHKA,
            Cyrillic::CapitalLetterZheWithBreve => CAPITAL_LETTER_ZHE_WITH_BREVE,
            Cyrillic::SmallLetterZheWithBreve => SMALL_LETTER_ZHE_WITH_BREVE,
            Cyrillic::CapitalLetterKaWithHook => CAPITAL_LETTER_KA_WITH_HOOK,
            Cyrillic::SmallLetterKaWithHook => SMALL_LETTER_KA_WITH_HOOK,
            Cyrillic::CapitalLetterElWithTail => CAPITAL_LETTER_EL_WITH_TAIL,
            Cyrillic::SmallLetterElWithTail => SMALL_LETTER_EL_WITH_TAIL,
            Cyrillic::CapitalLetterEnWithHook => CAPITAL_LETTER_EN_WITH_HOOK,
            Cyrillic::SmallLetterEnWithHook => SMALL_LETTER_EN_WITH_HOOK,
            Cyrillic::CapitalLetterEnWithTail => CAPITAL_LETTER_EN_WITH_TAIL,
            Cyrillic::SmallLetterEnWithTail => SMALL_LETTER_EN_WITH_TAIL,
            Cyrillic::CapitalLetterKhakassianChe => CAPITAL_LETTER_KHAKASSIAN_CHE,
            Cyrillic::SmallLetterKhakassianChe => SMALL_LETTER_KHAKASSIAN_CHE,
            Cyrillic::CapitalLetterEmWithTail => CAPITAL_LETTER_EM_WITH_TAIL,
            Cyrillic::SmallLetterEmWithTail => SMALL_LETTER_EM_WITH_TAIL,
            Cyrillic::SmallLetterPalochka => SMALL_LETTER_PALOCHKA,
            Cyrillic::CapitalLetterAWithBreve => CAPITAL_LETTER_A_WITH_BREVE,
            Cyrillic::SmallLetterAWithBreve => SMALL_LETTER_A_WITH_BREVE,
            Cyrillic::CapitalLetterAWithDiaeresis => CAPITAL_LETTER_A_WITH_DIAERESIS,
            Cyrillic::SmallLetterAWithDiaeresis => SMALL_LETTER_A_WITH_DIAERESIS,
            Cyrillic::CapitalLigatureAIe => CAPITAL_LIGATURE_A_IE,
            Cyrillic::SmallLigatureAIe => SMALL_LIGATURE_A_IE,
            Cyrillic::CapitalLetterIeWithBreve => CAPITAL_LETTER_IE_WITH_BREVE,
            Cyrillic::SmallLetterIeWithBreve => SMALL_LETTER_IE_WITH_BREVE,
            Cyrillic::CapitalLetterSchwa => CAPITAL_LETTER_SCHWA,
            Cyrillic::SmallLetterSchwa => SMALL_LETTER_SCHWA,
            Cyrillic::CapitalLetterSchwaWithDiaeresis => CAPITAL_LETTER_SCHWA_WITH_DIAERESIS,
            Cyrillic::SmallLetterSchwaWithDiaeresis => SMALL_LETTER_SCHWA_WITH_DIAERESIS,
            Cyrillic::CapitalLetterZheWithDiaeresis => CAPITAL_LETTER_ZHE_WITH_DIAERESIS,
            Cyrillic::SmallLetterZheWithDiaeresis => SMALL_LETTER_ZHE_WITH_DIAERESIS,
            Cyrillic::CapitalLetterZeWithDiaeresis => CAPITAL_LETTER_ZE_WITH_DIAERESIS,
            Cyrillic::SmallLetterZeWithDiaeresis => SMALL_LETTER_ZE_WITH_DIAERESIS,
            Cyrillic::CapitalLetterAbkhasianDze => CAPITAL_LETTER_ABKHASIAN_DZE,
            Cyrillic::SmallLetterAbkhasianDze => SMALL_LETTER_ABKHASIAN_DZE,
            Cyrillic::CapitalLetterIWithMacron => CAPITAL_LETTER_I_WITH_MACRON,
            Cyrillic::SmallLetterIWithMacron => SMALL_LETTER_I_WITH_MACRON,
            Cyrillic::CapitalLetterIWithDiaeresis => CAPITAL_LETTER_I_WITH_DIAERESIS,
            Cyrillic::SmallLetterIWithDiaeresis => SMALL_LETTER_I_WITH_DIAERESIS,
            Cyrillic::CapitalLetterOWithDiaeresis => CAPITAL_LETTER_O_WITH_DIAERESIS,
            Cyrillic::SmallLetterOWithDiaeresis => SMALL_LETTER_O_WITH_DIAERESIS,
            Cyrillic::CapitalLetterBarredO => CAPITAL_LETTER_BARRED_O,
            Cyrillic::SmallLetterBarredO => SMALL_LETTER_BARRED_O,
            Cyrillic::CapitalLetterBarredOWithDiaeresis => CAPITAL_LETTER_BARRED_O_WITH_DIAERESIS,
            Cyrillic::SmallLetterBarredOWithDiaeresis => SMALL_LETTER_BARRED_O_WITH_DIAERESIS,
            Cyrillic::CapitalLetterEWithDiaeresis => CAPITAL_LETTER_E_WITH_DIAERESIS,
            Cyrillic::SmallLetterEWithDiaeresis => SMALL_LETTER_E_WITH_DIAERESIS,
            Cyrillic::CapitalLetterUWithMacron => CAPITAL_LETTER_U_WITH_MACRON,
            Cyrillic::SmallLetterUWithMacron => SMALL_LETTER_U_WITH_MACRON,
            Cyrillic::CapitalLetterUWithDiaeresis => CAPITAL_LETTER_U_WITH_DIAERESIS,
            Cyrillic::SmallLetterUWithDiaeresis => SMALL_LETTER_U_WITH_DIAERESIS,
            Cyrillic::CapitalLetterUWithDoubleAcute => CAPITAL_LETTER_U_WITH_DOUBLE_ACUTE,
            Cyrillic::SmallLetterUWithDoubleAcute => SMALL_LETTER_U_WITH_DOUBLE_ACUTE,
            Cyrillic::CapitalLetterCheWithDiaeresis => CAPITAL_LETTER_CHE_WITH_DIAERESIS,
            Cyrillic::SmallLetterCheWithDiaeresis => SMALL_LETTER_CHE_WITH_DIAERESIS,
            Cyrillic::CapitalLetterGheWithDescender => CAPITAL_LETTER_GHE_WITH_DESCENDER,
            Cyrillic::SmallLetterGheWithDescender => SMALL_LETTER_GHE_WITH_DESCENDER,
            Cyrillic::CapitalLetterYeruWithDiaeresis => CAPITAL_LETTER_YERU_WITH_DIAERESIS,
            Cyrillic::SmallLetterYeruWithDiaeresis => SMALL_LETTER_YERU_WITH_DIAERESIS,
            Cyrillic::CapitalLetterGheWithStrokeAndHook => CAPITAL_LETTER_GHE_WITH_STROKE_AND_HOOK,
            Cyrillic::SmallLetterGheWithStrokeAndHook => SMALL_LETTER_GHE_WITH_STROKE_AND_HOOK,
            Cyrillic::CapitalLetterHaWithHook => CAPITAL_LETTER_HA_WITH_HOOK,
            Cyrillic::SmallLetterHaWithHook => SMALL_LETTER_HA_WITH_HOOK,
            Cyrillic::CapitalLetterHaWithStroke => CAPITAL_LETTER_HA_WITH_STROKE,
        }
    }
}

impl std::convert::TryFrom<char> for Cyrillic {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            CAPITAL_LETTER_IE_WITH_GRAVE => Ok(Cyrillic::CapitalLetterIeWithGrave),
            CAPITAL_LETTER_IO => Ok(Cyrillic::CapitalLetterIo),
            CAPITAL_LETTER_DJE => Ok(Cyrillic::CapitalLetterDje),
            CAPITAL_LETTER_GJE => Ok(Cyrillic::CapitalLetterGje),
            CAPITAL_LETTER_UKRAINIAN_IE => Ok(Cyrillic::CapitalLetterUkrainianIe),
            CAPITAL_LETTER_DZE => Ok(Cyrillic::CapitalLetterDze),
            CAPITAL_LETTER_BYELORUSSIAN_DASH_UKRAINIAN_I => Ok(Cyrillic::CapitalLetterByelorussianDashUkrainianI),
            CAPITAL_LETTER_YI => Ok(Cyrillic::CapitalLetterYi),
            CAPITAL_LETTER_JE => Ok(Cyrillic::CapitalLetterJe),
            CAPITAL_LETTER_LJE => Ok(Cyrillic::CapitalLetterLje),
            CAPITAL_LETTER_NJE => Ok(Cyrillic::CapitalLetterNje),
            CAPITAL_LETTER_TSHE => Ok(Cyrillic::CapitalLetterTshe),
            CAPITAL_LETTER_KJE => Ok(Cyrillic::CapitalLetterKje),
            CAPITAL_LETTER_I_WITH_GRAVE => Ok(Cyrillic::CapitalLetterIWithGrave),
            CAPITAL_LETTER_SHORT_U => Ok(Cyrillic::CapitalLetterShortU),
            CAPITAL_LETTER_DZHE => Ok(Cyrillic::CapitalLetterDzhe),
            CAPITAL_LETTER_A => Ok(Cyrillic::CapitalLetterA),
            CAPITAL_LETTER_BE => Ok(Cyrillic::CapitalLetterBe),
            CAPITAL_LETTER_VE => Ok(Cyrillic::CapitalLetterVe),
            CAPITAL_LETTER_GHE => Ok(Cyrillic::CapitalLetterGhe),
            CAPITAL_LETTER_DE => Ok(Cyrillic::CapitalLetterDe),
            CAPITAL_LETTER_IE => Ok(Cyrillic::CapitalLetterIe),
            CAPITAL_LETTER_ZHE => Ok(Cyrillic::CapitalLetterZhe),
            CAPITAL_LETTER_ZE => Ok(Cyrillic::CapitalLetterZe),
            CAPITAL_LETTER_I => Ok(Cyrillic::CapitalLetterI),
            CAPITAL_LETTER_SHORT_I => Ok(Cyrillic::CapitalLetterShortI),
            CAPITAL_LETTER_KA => Ok(Cyrillic::CapitalLetterKa),
            CAPITAL_LETTER_EL => Ok(Cyrillic::CapitalLetterEl),
            CAPITAL_LETTER_EM => Ok(Cyrillic::CapitalLetterEm),
            CAPITAL_LETTER_EN => Ok(Cyrillic::CapitalLetterEn),
            CAPITAL_LETTER_O => Ok(Cyrillic::CapitalLetterO),
            CAPITAL_LETTER_PE => Ok(Cyrillic::CapitalLetterPe),
            CAPITAL_LETTER_ER => Ok(Cyrillic::CapitalLetterEr),
            CAPITAL_LETTER_ES => Ok(Cyrillic::CapitalLetterEs),
            CAPITAL_LETTER_TE => Ok(Cyrillic::CapitalLetterTe),
            CAPITAL_LETTER_U => Ok(Cyrillic::CapitalLetterU),
            CAPITAL_LETTER_EF => Ok(Cyrillic::CapitalLetterEf),
            CAPITAL_LETTER_HA => Ok(Cyrillic::CapitalLetterHa),
            CAPITAL_LETTER_TSE => Ok(Cyrillic::CapitalLetterTse),
            CAPITAL_LETTER_CHE => Ok(Cyrillic::CapitalLetterChe),
            CAPITAL_LETTER_SHA => Ok(Cyrillic::CapitalLetterSha),
            CAPITAL_LETTER_SHCHA => Ok(Cyrillic::CapitalLetterShcha),
            CAPITAL_LETTER_HARD_SIGN => Ok(Cyrillic::CapitalLetterHardSign),
            CAPITAL_LETTER_YERU => Ok(Cyrillic::CapitalLetterYeru),
            CAPITAL_LETTER_SOFT_SIGN => Ok(Cyrillic::CapitalLetterSoftSign),
            CAPITAL_LETTER_E => Ok(Cyrillic::CapitalLetterE),
            CAPITAL_LETTER_YU => Ok(Cyrillic::CapitalLetterYu),
            CAPITAL_LETTER_YA => Ok(Cyrillic::CapitalLetterYa),
            SMALL_LETTER_A => Ok(Cyrillic::SmallLetterA),
            SMALL_LETTER_BE => Ok(Cyrillic::SmallLetterBe),
            SMALL_LETTER_VE => Ok(Cyrillic::SmallLetterVe),
            SMALL_LETTER_GHE => Ok(Cyrillic::SmallLetterGhe),
            SMALL_LETTER_DE => Ok(Cyrillic::SmallLetterDe),
            SMALL_LETTER_IE => Ok(Cyrillic::SmallLetterIe),
            SMALL_LETTER_ZHE => Ok(Cyrillic::SmallLetterZhe),
            SMALL_LETTER_ZE => Ok(Cyrillic::SmallLetterZe),
            SMALL_LETTER_I => Ok(Cyrillic::SmallLetterI),
            SMALL_LETTER_SHORT_I => Ok(Cyrillic::SmallLetterShortI),
            SMALL_LETTER_KA => Ok(Cyrillic::SmallLetterKa),
            SMALL_LETTER_EL => Ok(Cyrillic::SmallLetterEl),
            SMALL_LETTER_EM => Ok(Cyrillic::SmallLetterEm),
            SMALL_LETTER_EN => Ok(Cyrillic::SmallLetterEn),
            SMALL_LETTER_O => Ok(Cyrillic::SmallLetterO),
            SMALL_LETTER_PE => Ok(Cyrillic::SmallLetterPe),
            SMALL_LETTER_ER => Ok(Cyrillic::SmallLetterEr),
            SMALL_LETTER_ES => Ok(Cyrillic::SmallLetterEs),
            SMALL_LETTER_TE => Ok(Cyrillic::SmallLetterTe),
            SMALL_LETTER_U => Ok(Cyrillic::SmallLetterU),
            SMALL_LETTER_EF => Ok(Cyrillic::SmallLetterEf),
            SMALL_LETTER_HA => Ok(Cyrillic::SmallLetterHa),
            SMALL_LETTER_TSE => Ok(Cyrillic::SmallLetterTse),
            SMALL_LETTER_CHE => Ok(Cyrillic::SmallLetterChe),
            SMALL_LETTER_SHA => Ok(Cyrillic::SmallLetterSha),
            SMALL_LETTER_SHCHA => Ok(Cyrillic::SmallLetterShcha),
            SMALL_LETTER_HARD_SIGN => Ok(Cyrillic::SmallLetterHardSign),
            SMALL_LETTER_YERU => Ok(Cyrillic::SmallLetterYeru),
            SMALL_LETTER_SOFT_SIGN => Ok(Cyrillic::SmallLetterSoftSign),
            SMALL_LETTER_E => Ok(Cyrillic::SmallLetterE),
            SMALL_LETTER_YU => Ok(Cyrillic::SmallLetterYu),
            SMALL_LETTER_YA => Ok(Cyrillic::SmallLetterYa),
            SMALL_LETTER_IE_WITH_GRAVE => Ok(Cyrillic::SmallLetterIeWithGrave),
            SMALL_LETTER_IO => Ok(Cyrillic::SmallLetterIo),
            SMALL_LETTER_DJE => Ok(Cyrillic::SmallLetterDje),
            SMALL_LETTER_GJE => Ok(Cyrillic::SmallLetterGje),
            SMALL_LETTER_UKRAINIAN_IE => Ok(Cyrillic::SmallLetterUkrainianIe),
            SMALL_LETTER_DZE => Ok(Cyrillic::SmallLetterDze),
            SMALL_LETTER_BYELORUSSIAN_DASH_UKRAINIAN_I => Ok(Cyrillic::SmallLetterByelorussianDashUkrainianI),
            SMALL_LETTER_YI => Ok(Cyrillic::SmallLetterYi),
            SMALL_LETTER_JE => Ok(Cyrillic::SmallLetterJe),
            SMALL_LETTER_LJE => Ok(Cyrillic::SmallLetterLje),
            SMALL_LETTER_NJE => Ok(Cyrillic::SmallLetterNje),
            SMALL_LETTER_TSHE => Ok(Cyrillic::SmallLetterTshe),
            SMALL_LETTER_KJE => Ok(Cyrillic::SmallLetterKje),
            SMALL_LETTER_I_WITH_GRAVE => Ok(Cyrillic::SmallLetterIWithGrave),
            SMALL_LETTER_SHORT_U => Ok(Cyrillic::SmallLetterShortU),
            SMALL_LETTER_DZHE => Ok(Cyrillic::SmallLetterDzhe),
            CAPITAL_LETTER_OMEGA => Ok(Cyrillic::CapitalLetterOmega),
            SMALL_LETTER_OMEGA => Ok(Cyrillic::SmallLetterOmega),
            CAPITAL_LETTER_YAT => Ok(Cyrillic::CapitalLetterYat),
            SMALL_LETTER_YAT => Ok(Cyrillic::SmallLetterYat),
            CAPITAL_LETTER_IOTIFIED_E => Ok(Cyrillic::CapitalLetterIotifiedE),
            SMALL_LETTER_IOTIFIED_E => Ok(Cyrillic::SmallLetterIotifiedE),
            CAPITAL_LETTER_LITTLE_YUS => Ok(Cyrillic::CapitalLetterLittleYus),
            SMALL_LETTER_LITTLE_YUS => Ok(Cyrillic::SmallLetterLittleYus),
            CAPITAL_LETTER_IOTIFIED_LITTLE_YUS => Ok(Cyrillic::CapitalLetterIotifiedLittleYus),
            SMALL_LETTER_IOTIFIED_LITTLE_YUS => Ok(Cyrillic::SmallLetterIotifiedLittleYus),
            CAPITAL_LETTER_BIG_YUS => Ok(Cyrillic::CapitalLetterBigYus),
            SMALL_LETTER_BIG_YUS => Ok(Cyrillic::SmallLetterBigYus),
            CAPITAL_LETTER_IOTIFIED_BIG_YUS => Ok(Cyrillic::CapitalLetterIotifiedBigYus),
            SMALL_LETTER_IOTIFIED_BIG_YUS => Ok(Cyrillic::SmallLetterIotifiedBigYus),
            CAPITAL_LETTER_KSI => Ok(Cyrillic::CapitalLetterKsi),
            SMALL_LETTER_KSI => Ok(Cyrillic::SmallLetterKsi),
            CAPITAL_LETTER_PSI => Ok(Cyrillic::CapitalLetterPsi),
            SMALL_LETTER_PSI => Ok(Cyrillic::SmallLetterPsi),
            CAPITAL_LETTER_FITA => Ok(Cyrillic::CapitalLetterFita),
            SMALL_LETTER_FITA => Ok(Cyrillic::SmallLetterFita),
            CAPITAL_LETTER_IZHITSA => Ok(Cyrillic::CapitalLetterIzhitsa),
            SMALL_LETTER_IZHITSA => Ok(Cyrillic::SmallLetterIzhitsa),
            CAPITAL_LETTER_IZHITSA_WITH_DOUBLE_GRAVE_ACCENT => Ok(Cyrillic::CapitalLetterIzhitsaWithDoubleGraveAccent),
            SMALL_LETTER_IZHITSA_WITH_DOUBLE_GRAVE_ACCENT => Ok(Cyrillic::SmallLetterIzhitsaWithDoubleGraveAccent),
            CAPITAL_LETTER_UK => Ok(Cyrillic::CapitalLetterUk),
            SMALL_LETTER_UK => Ok(Cyrillic::SmallLetterUk),
            CAPITAL_LETTER_ROUND_OMEGA => Ok(Cyrillic::CapitalLetterRoundOmega),
            SMALL_LETTER_ROUND_OMEGA => Ok(Cyrillic::SmallLetterRoundOmega),
            CAPITAL_LETTER_OMEGA_WITH_TITLO => Ok(Cyrillic::CapitalLetterOmegaWithTitlo),
            SMALL_LETTER_OMEGA_WITH_TITLO => Ok(Cyrillic::SmallLetterOmegaWithTitlo),
            CAPITAL_LETTER_OT => Ok(Cyrillic::CapitalLetterOt),
            SMALL_LETTER_OT => Ok(Cyrillic::SmallLetterOt),
            CAPITAL_LETTER_KOPPA => Ok(Cyrillic::CapitalLetterKoppa),
            SMALL_LETTER_KOPPA => Ok(Cyrillic::SmallLetterKoppa),
            THOUSANDS_SIGN => Ok(Cyrillic::ThousandsSign),
            COMBINING_TITLO => Ok(Cyrillic::CombiningTitlo),
            COMBINING_PALATALIZATION => Ok(Cyrillic::CombiningPalatalization),
            COMBINING_DASIA_PNEUMATA => Ok(Cyrillic::CombiningDasiaPneumata),
            COMBINING_PSILI_PNEUMATA => Ok(Cyrillic::CombiningPsiliPneumata),
            COMBINING_POKRYTIE => Ok(Cyrillic::CombiningPokrytie),
            COMBINING_HUNDRED_THOUSANDS_SIGN => Ok(Cyrillic::CombiningHundredThousandsSign),
            COMBINING_MILLIONS_SIGN => Ok(Cyrillic::CombiningMillionsSign),
            CAPITAL_LETTER_SHORT_I_WITH_TAIL => Ok(Cyrillic::CapitalLetterShortIWithTail),
            SMALL_LETTER_SHORT_I_WITH_TAIL => Ok(Cyrillic::SmallLetterShortIWithTail),
            CAPITAL_LETTER_SEMISOFT_SIGN => Ok(Cyrillic::CapitalLetterSemisoftSign),
            SMALL_LETTER_SEMISOFT_SIGN => Ok(Cyrillic::SmallLetterSemisoftSign),
            CAPITAL_LETTER_ER_WITH_TICK => Ok(Cyrillic::CapitalLetterErWithTick),
            SMALL_LETTER_ER_WITH_TICK => Ok(Cyrillic::SmallLetterErWithTick),
            CAPITAL_LETTER_GHE_WITH_UPTURN => Ok(Cyrillic::CapitalLetterGheWithUpturn),
            SMALL_LETTER_GHE_WITH_UPTURN => Ok(Cyrillic::SmallLetterGheWithUpturn),
            CAPITAL_LETTER_GHE_WITH_STROKE => Ok(Cyrillic::CapitalLetterGheWithStroke),
            SMALL_LETTER_GHE_WITH_STROKE => Ok(Cyrillic::SmallLetterGheWithStroke),
            CAPITAL_LETTER_GHE_WITH_MIDDLE_HOOK => Ok(Cyrillic::CapitalLetterGheWithMiddleHook),
            SMALL_LETTER_GHE_WITH_MIDDLE_HOOK => Ok(Cyrillic::SmallLetterGheWithMiddleHook),
            CAPITAL_LETTER_ZHE_WITH_DESCENDER => Ok(Cyrillic::CapitalLetterZheWithDescender),
            SMALL_LETTER_ZHE_WITH_DESCENDER => Ok(Cyrillic::SmallLetterZheWithDescender),
            CAPITAL_LETTER_ZE_WITH_DESCENDER => Ok(Cyrillic::CapitalLetterZeWithDescender),
            SMALL_LETTER_ZE_WITH_DESCENDER => Ok(Cyrillic::SmallLetterZeWithDescender),
            CAPITAL_LETTER_KA_WITH_DESCENDER => Ok(Cyrillic::CapitalLetterKaWithDescender),
            SMALL_LETTER_KA_WITH_DESCENDER => Ok(Cyrillic::SmallLetterKaWithDescender),
            CAPITAL_LETTER_KA_WITH_VERTICAL_STROKE => Ok(Cyrillic::CapitalLetterKaWithVerticalStroke),
            SMALL_LETTER_KA_WITH_VERTICAL_STROKE => Ok(Cyrillic::SmallLetterKaWithVerticalStroke),
            CAPITAL_LETTER_KA_WITH_STROKE => Ok(Cyrillic::CapitalLetterKaWithStroke),
            SMALL_LETTER_KA_WITH_STROKE => Ok(Cyrillic::SmallLetterKaWithStroke),
            CAPITAL_LETTER_BASHKIR_KA => Ok(Cyrillic::CapitalLetterBashkirKa),
            SMALL_LETTER_BASHKIR_KA => Ok(Cyrillic::SmallLetterBashkirKa),
            CAPITAL_LETTER_EN_WITH_DESCENDER => Ok(Cyrillic::CapitalLetterEnWithDescender),
            SMALL_LETTER_EN_WITH_DESCENDER => Ok(Cyrillic::SmallLetterEnWithDescender),
            CAPITAL_LIGATURE_EN_GHE => Ok(Cyrillic::CapitalLigatureEnGhe),
            SMALL_LIGATURE_EN_GHE => Ok(Cyrillic::SmallLigatureEnGhe),
            CAPITAL_LETTER_PE_WITH_MIDDLE_HOOK => Ok(Cyrillic::CapitalLetterPeWithMiddleHook),
            SMALL_LETTER_PE_WITH_MIDDLE_HOOK => Ok(Cyrillic::SmallLetterPeWithMiddleHook),
            CAPITAL_LETTER_ABKHASIAN_HA => Ok(Cyrillic::CapitalLetterAbkhasianHa),
            SMALL_LETTER_ABKHASIAN_HA => Ok(Cyrillic::SmallLetterAbkhasianHa),
            CAPITAL_LETTER_ES_WITH_DESCENDER => Ok(Cyrillic::CapitalLetterEsWithDescender),
            SMALL_LETTER_ES_WITH_DESCENDER => Ok(Cyrillic::SmallLetterEsWithDescender),
            CAPITAL_LETTER_TE_WITH_DESCENDER => Ok(Cyrillic::CapitalLetterTeWithDescender),
            SMALL_LETTER_TE_WITH_DESCENDER => Ok(Cyrillic::SmallLetterTeWithDescender),
            CAPITAL_LETTER_STRAIGHT_U => Ok(Cyrillic::CapitalLetterStraightU),
            SMALL_LETTER_STRAIGHT_U => Ok(Cyrillic::SmallLetterStraightU),
            CAPITAL_LETTER_STRAIGHT_U_WITH_STROKE => Ok(Cyrillic::CapitalLetterStraightUWithStroke),
            SMALL_LETTER_STRAIGHT_U_WITH_STROKE => Ok(Cyrillic::SmallLetterStraightUWithStroke),
            CAPITAL_LETTER_HA_WITH_DESCENDER => Ok(Cyrillic::CapitalLetterHaWithDescender),
            SMALL_LETTER_HA_WITH_DESCENDER => Ok(Cyrillic::SmallLetterHaWithDescender),
            CAPITAL_LIGATURE_TE_TSE => Ok(Cyrillic::CapitalLigatureTeTse),
            SMALL_LIGATURE_TE_TSE => Ok(Cyrillic::SmallLigatureTeTse),
            CAPITAL_LETTER_CHE_WITH_DESCENDER => Ok(Cyrillic::CapitalLetterCheWithDescender),
            SMALL_LETTER_CHE_WITH_DESCENDER => Ok(Cyrillic::SmallLetterCheWithDescender),
            CAPITAL_LETTER_CHE_WITH_VERTICAL_STROKE => Ok(Cyrillic::CapitalLetterCheWithVerticalStroke),
            SMALL_LETTER_CHE_WITH_VERTICAL_STROKE => Ok(Cyrillic::SmallLetterCheWithVerticalStroke),
            CAPITAL_LETTER_SHHA => Ok(Cyrillic::CapitalLetterShha),
            SMALL_LETTER_SHHA => Ok(Cyrillic::SmallLetterShha),
            CAPITAL_LETTER_ABKHASIAN_CHE => Ok(Cyrillic::CapitalLetterAbkhasianChe),
            SMALL_LETTER_ABKHASIAN_CHE => Ok(Cyrillic::SmallLetterAbkhasianChe),
            CAPITAL_LETTER_ABKHASIAN_CHE_WITH_DESCENDER => Ok(Cyrillic::CapitalLetterAbkhasianCheWithDescender),
            SMALL_LETTER_ABKHASIAN_CHE_WITH_DESCENDER => Ok(Cyrillic::SmallLetterAbkhasianCheWithDescender),
            LETTER_PALOCHKA => Ok(Cyrillic::LetterPalochka),
            CAPITAL_LETTER_ZHE_WITH_BREVE => Ok(Cyrillic::CapitalLetterZheWithBreve),
            SMALL_LETTER_ZHE_WITH_BREVE => Ok(Cyrillic::SmallLetterZheWithBreve),
            CAPITAL_LETTER_KA_WITH_HOOK => Ok(Cyrillic::CapitalLetterKaWithHook),
            SMALL_LETTER_KA_WITH_HOOK => Ok(Cyrillic::SmallLetterKaWithHook),
            CAPITAL_LETTER_EL_WITH_TAIL => Ok(Cyrillic::CapitalLetterElWithTail),
            SMALL_LETTER_EL_WITH_TAIL => Ok(Cyrillic::SmallLetterElWithTail),
            CAPITAL_LETTER_EN_WITH_HOOK => Ok(Cyrillic::CapitalLetterEnWithHook),
            SMALL_LETTER_EN_WITH_HOOK => Ok(Cyrillic::SmallLetterEnWithHook),
            CAPITAL_LETTER_EN_WITH_TAIL => Ok(Cyrillic::CapitalLetterEnWithTail),
            SMALL_LETTER_EN_WITH_TAIL => Ok(Cyrillic::SmallLetterEnWithTail),
            CAPITAL_LETTER_KHAKASSIAN_CHE => Ok(Cyrillic::CapitalLetterKhakassianChe),
            SMALL_LETTER_KHAKASSIAN_CHE => Ok(Cyrillic::SmallLetterKhakassianChe),
            CAPITAL_LETTER_EM_WITH_TAIL => Ok(Cyrillic::CapitalLetterEmWithTail),
            SMALL_LETTER_EM_WITH_TAIL => Ok(Cyrillic::SmallLetterEmWithTail),
            SMALL_LETTER_PALOCHKA => Ok(Cyrillic::SmallLetterPalochka),
            CAPITAL_LETTER_A_WITH_BREVE => Ok(Cyrillic::CapitalLetterAWithBreve),
            SMALL_LETTER_A_WITH_BREVE => Ok(Cyrillic::SmallLetterAWithBreve),
            CAPITAL_LETTER_A_WITH_DIAERESIS => Ok(Cyrillic::CapitalLetterAWithDiaeresis),
            SMALL_LETTER_A_WITH_DIAERESIS => Ok(Cyrillic::SmallLetterAWithDiaeresis),
            CAPITAL_LIGATURE_A_IE => Ok(Cyrillic::CapitalLigatureAIe),
            SMALL_LIGATURE_A_IE => Ok(Cyrillic::SmallLigatureAIe),
            CAPITAL_LETTER_IE_WITH_BREVE => Ok(Cyrillic::CapitalLetterIeWithBreve),
            SMALL_LETTER_IE_WITH_BREVE => Ok(Cyrillic::SmallLetterIeWithBreve),
            CAPITAL_LETTER_SCHWA => Ok(Cyrillic::CapitalLetterSchwa),
            SMALL_LETTER_SCHWA => Ok(Cyrillic::SmallLetterSchwa),
            CAPITAL_LETTER_SCHWA_WITH_DIAERESIS => Ok(Cyrillic::CapitalLetterSchwaWithDiaeresis),
            SMALL_LETTER_SCHWA_WITH_DIAERESIS => Ok(Cyrillic::SmallLetterSchwaWithDiaeresis),
            CAPITAL_LETTER_ZHE_WITH_DIAERESIS => Ok(Cyrillic::CapitalLetterZheWithDiaeresis),
            SMALL_LETTER_ZHE_WITH_DIAERESIS => Ok(Cyrillic::SmallLetterZheWithDiaeresis),
            CAPITAL_LETTER_ZE_WITH_DIAERESIS => Ok(Cyrillic::CapitalLetterZeWithDiaeresis),
            SMALL_LETTER_ZE_WITH_DIAERESIS => Ok(Cyrillic::SmallLetterZeWithDiaeresis),
            CAPITAL_LETTER_ABKHASIAN_DZE => Ok(Cyrillic::CapitalLetterAbkhasianDze),
            SMALL_LETTER_ABKHASIAN_DZE => Ok(Cyrillic::SmallLetterAbkhasianDze),
            CAPITAL_LETTER_I_WITH_MACRON => Ok(Cyrillic::CapitalLetterIWithMacron),
            SMALL_LETTER_I_WITH_MACRON => Ok(Cyrillic::SmallLetterIWithMacron),
            CAPITAL_LETTER_I_WITH_DIAERESIS => Ok(Cyrillic::CapitalLetterIWithDiaeresis),
            SMALL_LETTER_I_WITH_DIAERESIS => Ok(Cyrillic::SmallLetterIWithDiaeresis),
            CAPITAL_LETTER_O_WITH_DIAERESIS => Ok(Cyrillic::CapitalLetterOWithDiaeresis),
            SMALL_LETTER_O_WITH_DIAERESIS => Ok(Cyrillic::SmallLetterOWithDiaeresis),
            CAPITAL_LETTER_BARRED_O => Ok(Cyrillic::CapitalLetterBarredO),
            SMALL_LETTER_BARRED_O => Ok(Cyrillic::SmallLetterBarredO),
            CAPITAL_LETTER_BARRED_O_WITH_DIAERESIS => Ok(Cyrillic::CapitalLetterBarredOWithDiaeresis),
            SMALL_LETTER_BARRED_O_WITH_DIAERESIS => Ok(Cyrillic::SmallLetterBarredOWithDiaeresis),
            CAPITAL_LETTER_E_WITH_DIAERESIS => Ok(Cyrillic::CapitalLetterEWithDiaeresis),
            SMALL_LETTER_E_WITH_DIAERESIS => Ok(Cyrillic::SmallLetterEWithDiaeresis),
            CAPITAL_LETTER_U_WITH_MACRON => Ok(Cyrillic::CapitalLetterUWithMacron),
            SMALL_LETTER_U_WITH_MACRON => Ok(Cyrillic::SmallLetterUWithMacron),
            CAPITAL_LETTER_U_WITH_DIAERESIS => Ok(Cyrillic::CapitalLetterUWithDiaeresis),
            SMALL_LETTER_U_WITH_DIAERESIS => Ok(Cyrillic::SmallLetterUWithDiaeresis),
            CAPITAL_LETTER_U_WITH_DOUBLE_ACUTE => Ok(Cyrillic::CapitalLetterUWithDoubleAcute),
            SMALL_LETTER_U_WITH_DOUBLE_ACUTE => Ok(Cyrillic::SmallLetterUWithDoubleAcute),
            CAPITAL_LETTER_CHE_WITH_DIAERESIS => Ok(Cyrillic::CapitalLetterCheWithDiaeresis),
            SMALL_LETTER_CHE_WITH_DIAERESIS => Ok(Cyrillic::SmallLetterCheWithDiaeresis),
            CAPITAL_LETTER_GHE_WITH_DESCENDER => Ok(Cyrillic::CapitalLetterGheWithDescender),
            SMALL_LETTER_GHE_WITH_DESCENDER => Ok(Cyrillic::SmallLetterGheWithDescender),
            CAPITAL_LETTER_YERU_WITH_DIAERESIS => Ok(Cyrillic::CapitalLetterYeruWithDiaeresis),
            SMALL_LETTER_YERU_WITH_DIAERESIS => Ok(Cyrillic::SmallLetterYeruWithDiaeresis),
            CAPITAL_LETTER_GHE_WITH_STROKE_AND_HOOK => Ok(Cyrillic::CapitalLetterGheWithStrokeAndHook),
            SMALL_LETTER_GHE_WITH_STROKE_AND_HOOK => Ok(Cyrillic::SmallLetterGheWithStrokeAndHook),
            CAPITAL_LETTER_HA_WITH_HOOK => Ok(Cyrillic::CapitalLetterHaWithHook),
            SMALL_LETTER_HA_WITH_HOOK => Ok(Cyrillic::SmallLetterHaWithHook),
            CAPITAL_LETTER_HA_WITH_STROKE => Ok(Cyrillic::CapitalLetterHaWithStroke),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Cyrillic {
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

impl std::convert::TryFrom<u32> for Cyrillic {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Cyrillic {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Cyrillic {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Cyrillic::CapitalLetterIeWithGrave
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Cyrillic::CapitalLetterIeWithGrave => "cyrillic capital letter ie with grave",
            Cyrillic::CapitalLetterIo => "cyrillic capital letter io",
            Cyrillic::CapitalLetterDje => "cyrillic capital letter dje",
            Cyrillic::CapitalLetterGje => "cyrillic capital letter gje",
            Cyrillic::CapitalLetterUkrainianIe => "cyrillic capital letter ukrainian ie",
            Cyrillic::CapitalLetterDze => "cyrillic capital letter dze",
            Cyrillic::CapitalLetterByelorussianDashUkrainianI => "cyrillic capital letter byelorussian-ukrainian i",
            Cyrillic::CapitalLetterYi => "cyrillic capital letter yi",
            Cyrillic::CapitalLetterJe => "cyrillic capital letter je",
            Cyrillic::CapitalLetterLje => "cyrillic capital letter lje",
            Cyrillic::CapitalLetterNje => "cyrillic capital letter nje",
            Cyrillic::CapitalLetterTshe => "cyrillic capital letter tshe",
            Cyrillic::CapitalLetterKje => "cyrillic capital letter kje",
            Cyrillic::CapitalLetterIWithGrave => "cyrillic capital letter i with grave",
            Cyrillic::CapitalLetterShortU => "cyrillic capital letter short u",
            Cyrillic::CapitalLetterDzhe => "cyrillic capital letter dzhe",
            Cyrillic::CapitalLetterA => "cyrillic capital letter a",
            Cyrillic::CapitalLetterBe => "cyrillic capital letter be",
            Cyrillic::CapitalLetterVe => "cyrillic capital letter ve",
            Cyrillic::CapitalLetterGhe => "cyrillic capital letter ghe",
            Cyrillic::CapitalLetterDe => "cyrillic capital letter de",
            Cyrillic::CapitalLetterIe => "cyrillic capital letter ie",
            Cyrillic::CapitalLetterZhe => "cyrillic capital letter zhe",
            Cyrillic::CapitalLetterZe => "cyrillic capital letter ze",
            Cyrillic::CapitalLetterI => "cyrillic capital letter i",
            Cyrillic::CapitalLetterShortI => "cyrillic capital letter short i",
            Cyrillic::CapitalLetterKa => "cyrillic capital letter ka",
            Cyrillic::CapitalLetterEl => "cyrillic capital letter el",
            Cyrillic::CapitalLetterEm => "cyrillic capital letter em",
            Cyrillic::CapitalLetterEn => "cyrillic capital letter en",
            Cyrillic::CapitalLetterO => "cyrillic capital letter o",
            Cyrillic::CapitalLetterPe => "cyrillic capital letter pe",
            Cyrillic::CapitalLetterEr => "cyrillic capital letter er",
            Cyrillic::CapitalLetterEs => "cyrillic capital letter es",
            Cyrillic::CapitalLetterTe => "cyrillic capital letter te",
            Cyrillic::CapitalLetterU => "cyrillic capital letter u",
            Cyrillic::CapitalLetterEf => "cyrillic capital letter ef",
            Cyrillic::CapitalLetterHa => "cyrillic capital letter ha",
            Cyrillic::CapitalLetterTse => "cyrillic capital letter tse",
            Cyrillic::CapitalLetterChe => "cyrillic capital letter che",
            Cyrillic::CapitalLetterSha => "cyrillic capital letter sha",
            Cyrillic::CapitalLetterShcha => "cyrillic capital letter shcha",
            Cyrillic::CapitalLetterHardSign => "cyrillic capital letter hard sign",
            Cyrillic::CapitalLetterYeru => "cyrillic capital letter yeru",
            Cyrillic::CapitalLetterSoftSign => "cyrillic capital letter soft sign",
            Cyrillic::CapitalLetterE => "cyrillic capital letter e",
            Cyrillic::CapitalLetterYu => "cyrillic capital letter yu",
            Cyrillic::CapitalLetterYa => "cyrillic capital letter ya",
            Cyrillic::SmallLetterA => "cyrillic small letter a",
            Cyrillic::SmallLetterBe => "cyrillic small letter be",
            Cyrillic::SmallLetterVe => "cyrillic small letter ve",
            Cyrillic::SmallLetterGhe => "cyrillic small letter ghe",
            Cyrillic::SmallLetterDe => "cyrillic small letter de",
            Cyrillic::SmallLetterIe => "cyrillic small letter ie",
            Cyrillic::SmallLetterZhe => "cyrillic small letter zhe",
            Cyrillic::SmallLetterZe => "cyrillic small letter ze",
            Cyrillic::SmallLetterI => "cyrillic small letter i",
            Cyrillic::SmallLetterShortI => "cyrillic small letter short i",
            Cyrillic::SmallLetterKa => "cyrillic small letter ka",
            Cyrillic::SmallLetterEl => "cyrillic small letter el",
            Cyrillic::SmallLetterEm => "cyrillic small letter em",
            Cyrillic::SmallLetterEn => "cyrillic small letter en",
            Cyrillic::SmallLetterO => "cyrillic small letter o",
            Cyrillic::SmallLetterPe => "cyrillic small letter pe",
            Cyrillic::SmallLetterEr => "cyrillic small letter er",
            Cyrillic::SmallLetterEs => "cyrillic small letter es",
            Cyrillic::SmallLetterTe => "cyrillic small letter te",
            Cyrillic::SmallLetterU => "cyrillic small letter u",
            Cyrillic::SmallLetterEf => "cyrillic small letter ef",
            Cyrillic::SmallLetterHa => "cyrillic small letter ha",
            Cyrillic::SmallLetterTse => "cyrillic small letter tse",
            Cyrillic::SmallLetterChe => "cyrillic small letter che",
            Cyrillic::SmallLetterSha => "cyrillic small letter sha",
            Cyrillic::SmallLetterShcha => "cyrillic small letter shcha",
            Cyrillic::SmallLetterHardSign => "cyrillic small letter hard sign",
            Cyrillic::SmallLetterYeru => "cyrillic small letter yeru",
            Cyrillic::SmallLetterSoftSign => "cyrillic small letter soft sign",
            Cyrillic::SmallLetterE => "cyrillic small letter e",
            Cyrillic::SmallLetterYu => "cyrillic small letter yu",
            Cyrillic::SmallLetterYa => "cyrillic small letter ya",
            Cyrillic::SmallLetterIeWithGrave => "cyrillic small letter ie with grave",
            Cyrillic::SmallLetterIo => "cyrillic small letter io",
            Cyrillic::SmallLetterDje => "cyrillic small letter dje",
            Cyrillic::SmallLetterGje => "cyrillic small letter gje",
            Cyrillic::SmallLetterUkrainianIe => "cyrillic small letter ukrainian ie",
            Cyrillic::SmallLetterDze => "cyrillic small letter dze",
            Cyrillic::SmallLetterByelorussianDashUkrainianI => "cyrillic small letter byelorussian-ukrainian i",
            Cyrillic::SmallLetterYi => "cyrillic small letter yi",
            Cyrillic::SmallLetterJe => "cyrillic small letter je",
            Cyrillic::SmallLetterLje => "cyrillic small letter lje",
            Cyrillic::SmallLetterNje => "cyrillic small letter nje",
            Cyrillic::SmallLetterTshe => "cyrillic small letter tshe",
            Cyrillic::SmallLetterKje => "cyrillic small letter kje",
            Cyrillic::SmallLetterIWithGrave => "cyrillic small letter i with grave",
            Cyrillic::SmallLetterShortU => "cyrillic small letter short u",
            Cyrillic::SmallLetterDzhe => "cyrillic small letter dzhe",
            Cyrillic::CapitalLetterOmega => "cyrillic capital letter omega",
            Cyrillic::SmallLetterOmega => "cyrillic small letter omega",
            Cyrillic::CapitalLetterYat => "cyrillic capital letter yat",
            Cyrillic::SmallLetterYat => "cyrillic small letter yat",
            Cyrillic::CapitalLetterIotifiedE => "cyrillic capital letter iotified e",
            Cyrillic::SmallLetterIotifiedE => "cyrillic small letter iotified e",
            Cyrillic::CapitalLetterLittleYus => "cyrillic capital letter little yus",
            Cyrillic::SmallLetterLittleYus => "cyrillic small letter little yus",
            Cyrillic::CapitalLetterIotifiedLittleYus => "cyrillic capital letter iotified little yus",
            Cyrillic::SmallLetterIotifiedLittleYus => "cyrillic small letter iotified little yus",
            Cyrillic::CapitalLetterBigYus => "cyrillic capital letter big yus",
            Cyrillic::SmallLetterBigYus => "cyrillic small letter big yus",
            Cyrillic::CapitalLetterIotifiedBigYus => "cyrillic capital letter iotified big yus",
            Cyrillic::SmallLetterIotifiedBigYus => "cyrillic small letter iotified big yus",
            Cyrillic::CapitalLetterKsi => "cyrillic capital letter ksi",
            Cyrillic::SmallLetterKsi => "cyrillic small letter ksi",
            Cyrillic::CapitalLetterPsi => "cyrillic capital letter psi",
            Cyrillic::SmallLetterPsi => "cyrillic small letter psi",
            Cyrillic::CapitalLetterFita => "cyrillic capital letter fita",
            Cyrillic::SmallLetterFita => "cyrillic small letter fita",
            Cyrillic::CapitalLetterIzhitsa => "cyrillic capital letter izhitsa",
            Cyrillic::SmallLetterIzhitsa => "cyrillic small letter izhitsa",
            Cyrillic::CapitalLetterIzhitsaWithDoubleGraveAccent => "cyrillic capital letter izhitsa with double grave accent",
            Cyrillic::SmallLetterIzhitsaWithDoubleGraveAccent => "cyrillic small letter izhitsa with double grave accent",
            Cyrillic::CapitalLetterUk => "cyrillic capital letter uk",
            Cyrillic::SmallLetterUk => "cyrillic small letter uk",
            Cyrillic::CapitalLetterRoundOmega => "cyrillic capital letter round omega",
            Cyrillic::SmallLetterRoundOmega => "cyrillic small letter round omega",
            Cyrillic::CapitalLetterOmegaWithTitlo => "cyrillic capital letter omega with titlo",
            Cyrillic::SmallLetterOmegaWithTitlo => "cyrillic small letter omega with titlo",
            Cyrillic::CapitalLetterOt => "cyrillic capital letter ot",
            Cyrillic::SmallLetterOt => "cyrillic small letter ot",
            Cyrillic::CapitalLetterKoppa => "cyrillic capital letter koppa",
            Cyrillic::SmallLetterKoppa => "cyrillic small letter koppa",
            Cyrillic::ThousandsSign => "cyrillic thousands sign",
            Cyrillic::CombiningTitlo => "combining cyrillic titlo",
            Cyrillic::CombiningPalatalization => "combining cyrillic palatalization",
            Cyrillic::CombiningDasiaPneumata => "combining cyrillic dasia pneumata",
            Cyrillic::CombiningPsiliPneumata => "combining cyrillic psili pneumata",
            Cyrillic::CombiningPokrytie => "combining cyrillic pokrytie",
            Cyrillic::CombiningHundredThousandsSign => "combining cyrillic hundred thousands sign",
            Cyrillic::CombiningMillionsSign => "combining cyrillic millions sign",
            Cyrillic::CapitalLetterShortIWithTail => "cyrillic capital letter short i with tail",
            Cyrillic::SmallLetterShortIWithTail => "cyrillic small letter short i with tail",
            Cyrillic::CapitalLetterSemisoftSign => "cyrillic capital letter semisoft sign",
            Cyrillic::SmallLetterSemisoftSign => "cyrillic small letter semisoft sign",
            Cyrillic::CapitalLetterErWithTick => "cyrillic capital letter er with tick",
            Cyrillic::SmallLetterErWithTick => "cyrillic small letter er with tick",
            Cyrillic::CapitalLetterGheWithUpturn => "cyrillic capital letter ghe with upturn",
            Cyrillic::SmallLetterGheWithUpturn => "cyrillic small letter ghe with upturn",
            Cyrillic::CapitalLetterGheWithStroke => "cyrillic capital letter ghe with stroke",
            Cyrillic::SmallLetterGheWithStroke => "cyrillic small letter ghe with stroke",
            Cyrillic::CapitalLetterGheWithMiddleHook => "cyrillic capital letter ghe with middle hook",
            Cyrillic::SmallLetterGheWithMiddleHook => "cyrillic small letter ghe with middle hook",
            Cyrillic::CapitalLetterZheWithDescender => "cyrillic capital letter zhe with descender",
            Cyrillic::SmallLetterZheWithDescender => "cyrillic small letter zhe with descender",
            Cyrillic::CapitalLetterZeWithDescender => "cyrillic capital letter ze with descender",
            Cyrillic::SmallLetterZeWithDescender => "cyrillic small letter ze with descender",
            Cyrillic::CapitalLetterKaWithDescender => "cyrillic capital letter ka with descender",
            Cyrillic::SmallLetterKaWithDescender => "cyrillic small letter ka with descender",
            Cyrillic::CapitalLetterKaWithVerticalStroke => "cyrillic capital letter ka with vertical stroke",
            Cyrillic::SmallLetterKaWithVerticalStroke => "cyrillic small letter ka with vertical stroke",
            Cyrillic::CapitalLetterKaWithStroke => "cyrillic capital letter ka with stroke",
            Cyrillic::SmallLetterKaWithStroke => "cyrillic small letter ka with stroke",
            Cyrillic::CapitalLetterBashkirKa => "cyrillic capital letter bashkir ka",
            Cyrillic::SmallLetterBashkirKa => "cyrillic small letter bashkir ka",
            Cyrillic::CapitalLetterEnWithDescender => "cyrillic capital letter en with descender",
            Cyrillic::SmallLetterEnWithDescender => "cyrillic small letter en with descender",
            Cyrillic::CapitalLigatureEnGhe => "cyrillic capital ligature en ghe",
            Cyrillic::SmallLigatureEnGhe => "cyrillic small ligature en ghe",
            Cyrillic::CapitalLetterPeWithMiddleHook => "cyrillic capital letter pe with middle hook",
            Cyrillic::SmallLetterPeWithMiddleHook => "cyrillic small letter pe with middle hook",
            Cyrillic::CapitalLetterAbkhasianHa => "cyrillic capital letter abkhasian ha",
            Cyrillic::SmallLetterAbkhasianHa => "cyrillic small letter abkhasian ha",
            Cyrillic::CapitalLetterEsWithDescender => "cyrillic capital letter es with descender",
            Cyrillic::SmallLetterEsWithDescender => "cyrillic small letter es with descender",
            Cyrillic::CapitalLetterTeWithDescender => "cyrillic capital letter te with descender",
            Cyrillic::SmallLetterTeWithDescender => "cyrillic small letter te with descender",
            Cyrillic::CapitalLetterStraightU => "cyrillic capital letter straight u",
            Cyrillic::SmallLetterStraightU => "cyrillic small letter straight u",
            Cyrillic::CapitalLetterStraightUWithStroke => "cyrillic capital letter straight u with stroke",
            Cyrillic::SmallLetterStraightUWithStroke => "cyrillic small letter straight u with stroke",
            Cyrillic::CapitalLetterHaWithDescender => "cyrillic capital letter ha with descender",
            Cyrillic::SmallLetterHaWithDescender => "cyrillic small letter ha with descender",
            Cyrillic::CapitalLigatureTeTse => "cyrillic capital ligature te tse",
            Cyrillic::SmallLigatureTeTse => "cyrillic small ligature te tse",
            Cyrillic::CapitalLetterCheWithDescender => "cyrillic capital letter che with descender",
            Cyrillic::SmallLetterCheWithDescender => "cyrillic small letter che with descender",
            Cyrillic::CapitalLetterCheWithVerticalStroke => "cyrillic capital letter che with vertical stroke",
            Cyrillic::SmallLetterCheWithVerticalStroke => "cyrillic small letter che with vertical stroke",
            Cyrillic::CapitalLetterShha => "cyrillic capital letter shha",
            Cyrillic::SmallLetterShha => "cyrillic small letter shha",
            Cyrillic::CapitalLetterAbkhasianChe => "cyrillic capital letter abkhasian che",
            Cyrillic::SmallLetterAbkhasianChe => "cyrillic small letter abkhasian che",
            Cyrillic::CapitalLetterAbkhasianCheWithDescender => "cyrillic capital letter abkhasian che with descender",
            Cyrillic::SmallLetterAbkhasianCheWithDescender => "cyrillic small letter abkhasian che with descender",
            Cyrillic::LetterPalochka => "cyrillic letter palochka",
            Cyrillic::CapitalLetterZheWithBreve => "cyrillic capital letter zhe with breve",
            Cyrillic::SmallLetterZheWithBreve => "cyrillic small letter zhe with breve",
            Cyrillic::CapitalLetterKaWithHook => "cyrillic capital letter ka with hook",
            Cyrillic::SmallLetterKaWithHook => "cyrillic small letter ka with hook",
            Cyrillic::CapitalLetterElWithTail => "cyrillic capital letter el with tail",
            Cyrillic::SmallLetterElWithTail => "cyrillic small letter el with tail",
            Cyrillic::CapitalLetterEnWithHook => "cyrillic capital letter en with hook",
            Cyrillic::SmallLetterEnWithHook => "cyrillic small letter en with hook",
            Cyrillic::CapitalLetterEnWithTail => "cyrillic capital letter en with tail",
            Cyrillic::SmallLetterEnWithTail => "cyrillic small letter en with tail",
            Cyrillic::CapitalLetterKhakassianChe => "cyrillic capital letter khakassian che",
            Cyrillic::SmallLetterKhakassianChe => "cyrillic small letter khakassian che",
            Cyrillic::CapitalLetterEmWithTail => "cyrillic capital letter em with tail",
            Cyrillic::SmallLetterEmWithTail => "cyrillic small letter em with tail",
            Cyrillic::SmallLetterPalochka => "cyrillic small letter palochka",
            Cyrillic::CapitalLetterAWithBreve => "cyrillic capital letter a with breve",
            Cyrillic::SmallLetterAWithBreve => "cyrillic small letter a with breve",
            Cyrillic::CapitalLetterAWithDiaeresis => "cyrillic capital letter a with diaeresis",
            Cyrillic::SmallLetterAWithDiaeresis => "cyrillic small letter a with diaeresis",
            Cyrillic::CapitalLigatureAIe => "cyrillic capital ligature a ie",
            Cyrillic::SmallLigatureAIe => "cyrillic small ligature a ie",
            Cyrillic::CapitalLetterIeWithBreve => "cyrillic capital letter ie with breve",
            Cyrillic::SmallLetterIeWithBreve => "cyrillic small letter ie with breve",
            Cyrillic::CapitalLetterSchwa => "cyrillic capital letter schwa",
            Cyrillic::SmallLetterSchwa => "cyrillic small letter schwa",
            Cyrillic::CapitalLetterSchwaWithDiaeresis => "cyrillic capital letter schwa with diaeresis",
            Cyrillic::SmallLetterSchwaWithDiaeresis => "cyrillic small letter schwa with diaeresis",
            Cyrillic::CapitalLetterZheWithDiaeresis => "cyrillic capital letter zhe with diaeresis",
            Cyrillic::SmallLetterZheWithDiaeresis => "cyrillic small letter zhe with diaeresis",
            Cyrillic::CapitalLetterZeWithDiaeresis => "cyrillic capital letter ze with diaeresis",
            Cyrillic::SmallLetterZeWithDiaeresis => "cyrillic small letter ze with diaeresis",
            Cyrillic::CapitalLetterAbkhasianDze => "cyrillic capital letter abkhasian dze",
            Cyrillic::SmallLetterAbkhasianDze => "cyrillic small letter abkhasian dze",
            Cyrillic::CapitalLetterIWithMacron => "cyrillic capital letter i with macron",
            Cyrillic::SmallLetterIWithMacron => "cyrillic small letter i with macron",
            Cyrillic::CapitalLetterIWithDiaeresis => "cyrillic capital letter i with diaeresis",
            Cyrillic::SmallLetterIWithDiaeresis => "cyrillic small letter i with diaeresis",
            Cyrillic::CapitalLetterOWithDiaeresis => "cyrillic capital letter o with diaeresis",
            Cyrillic::SmallLetterOWithDiaeresis => "cyrillic small letter o with diaeresis",
            Cyrillic::CapitalLetterBarredO => "cyrillic capital letter barred o",
            Cyrillic::SmallLetterBarredO => "cyrillic small letter barred o",
            Cyrillic::CapitalLetterBarredOWithDiaeresis => "cyrillic capital letter barred o with diaeresis",
            Cyrillic::SmallLetterBarredOWithDiaeresis => "cyrillic small letter barred o with diaeresis",
            Cyrillic::CapitalLetterEWithDiaeresis => "cyrillic capital letter e with diaeresis",
            Cyrillic::SmallLetterEWithDiaeresis => "cyrillic small letter e with diaeresis",
            Cyrillic::CapitalLetterUWithMacron => "cyrillic capital letter u with macron",
            Cyrillic::SmallLetterUWithMacron => "cyrillic small letter u with macron",
            Cyrillic::CapitalLetterUWithDiaeresis => "cyrillic capital letter u with diaeresis",
            Cyrillic::SmallLetterUWithDiaeresis => "cyrillic small letter u with diaeresis",
            Cyrillic::CapitalLetterUWithDoubleAcute => "cyrillic capital letter u with double acute",
            Cyrillic::SmallLetterUWithDoubleAcute => "cyrillic small letter u with double acute",
            Cyrillic::CapitalLetterCheWithDiaeresis => "cyrillic capital letter che with diaeresis",
            Cyrillic::SmallLetterCheWithDiaeresis => "cyrillic small letter che with diaeresis",
            Cyrillic::CapitalLetterGheWithDescender => "cyrillic capital letter ghe with descender",
            Cyrillic::SmallLetterGheWithDescender => "cyrillic small letter ghe with descender",
            Cyrillic::CapitalLetterYeruWithDiaeresis => "cyrillic capital letter yeru with diaeresis",
            Cyrillic::SmallLetterYeruWithDiaeresis => "cyrillic small letter yeru with diaeresis",
            Cyrillic::CapitalLetterGheWithStrokeAndHook => "cyrillic capital letter ghe with stroke and hook",
            Cyrillic::SmallLetterGheWithStrokeAndHook => "cyrillic small letter ghe with stroke and hook",
            Cyrillic::CapitalLetterHaWithHook => "cyrillic capital letter ha with hook",
            Cyrillic::SmallLetterHaWithHook => "cyrillic small letter ha with hook",
            Cyrillic::CapitalLetterHaWithStroke => "cyrillic capital letter ha with stroke",
        }
    }
}
