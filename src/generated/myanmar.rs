/// A number of constants to give a name to all characters in this block.
mod constants {
    /// \u{1000}: 'က'
    pub const LETTER_KA: char = 'က';
    /// \u{1001}: 'ခ'
    pub const LETTER_KHA: char = 'ခ';
    /// \u{1002}: 'ဂ'
    pub const LETTER_GA: char = 'ဂ';
    /// \u{1003}: 'ဃ'
    pub const LETTER_GHA: char = 'ဃ';
    /// \u{1004}: 'င'
    pub const LETTER_NGA: char = 'င';
    /// \u{1005}: 'စ'
    pub const LETTER_CA: char = 'စ';
    /// \u{1006}: 'ဆ'
    pub const LETTER_CHA: char = 'ဆ';
    /// \u{1007}: 'ဇ'
    pub const LETTER_JA: char = 'ဇ';
    /// \u{1008}: 'ဈ'
    pub const LETTER_JHA: char = 'ဈ';
    /// \u{1009}: 'ဉ'
    pub const LETTER_NYA: char = 'ဉ';
    /// \u{100a}: 'ည'
    pub const LETTER_NNYA: char = 'ည';
    /// \u{100b}: 'ဋ'
    pub const LETTER_TTA: char = 'ဋ';
    /// \u{100c}: 'ဌ'
    pub const LETTER_TTHA: char = 'ဌ';
    /// \u{100d}: 'ဍ'
    pub const LETTER_DDA: char = 'ဍ';
    /// \u{100e}: 'ဎ'
    pub const LETTER_DDHA: char = 'ဎ';
    /// \u{100f}: 'ဏ'
    pub const LETTER_NNA: char = 'ဏ';
    /// \u{1010}: 'တ'
    pub const LETTER_TA: char = 'တ';
    /// \u{1011}: 'ထ'
    pub const LETTER_THA: char = 'ထ';
    /// \u{1012}: 'ဒ'
    pub const LETTER_DA: char = 'ဒ';
    /// \u{1013}: 'ဓ'
    pub const LETTER_DHA: char = 'ဓ';
    /// \u{1014}: 'န'
    pub const LETTER_NA: char = 'န';
    /// \u{1015}: 'ပ'
    pub const LETTER_PA: char = 'ပ';
    /// \u{1016}: 'ဖ'
    pub const LETTER_PHA: char = 'ဖ';
    /// \u{1017}: 'ဗ'
    pub const LETTER_BA: char = 'ဗ';
    /// \u{1018}: 'ဘ'
    pub const LETTER_BHA: char = 'ဘ';
    /// \u{1019}: 'မ'
    pub const LETTER_MA: char = 'မ';
    /// \u{101a}: 'ယ'
    pub const LETTER_YA: char = 'ယ';
    /// \u{101b}: 'ရ'
    pub const LETTER_RA: char = 'ရ';
    /// \u{101c}: 'လ'
    pub const LETTER_LA: char = 'လ';
    /// \u{101d}: 'ဝ'
    pub const LETTER_WA: char = 'ဝ';
    /// \u{101e}: 'သ'
    pub const LETTER_SA: char = 'သ';
    /// \u{101f}: 'ဟ'
    pub const LETTER_HA: char = 'ဟ';
    /// \u{1020}: 'ဠ'
    pub const LETTER_LLA: char = 'ဠ';
    /// \u{1021}: 'အ'
    pub const LETTER_A: char = 'အ';
    /// \u{1022}: 'ဢ'
    pub const LETTER_SHAN_A: char = 'ဢ';
    /// \u{1023}: 'ဣ'
    pub const LETTER_I: char = 'ဣ';
    /// \u{1024}: 'ဤ'
    pub const LETTER_II: char = 'ဤ';
    /// \u{1025}: 'ဥ'
    pub const LETTER_U: char = 'ဥ';
    /// \u{1026}: 'ဦ'
    pub const LETTER_UU: char = 'ဦ';
    /// \u{1027}: 'ဧ'
    pub const LETTER_E: char = 'ဧ';
    /// \u{1028}: 'ဨ'
    pub const LETTER_MON_E: char = 'ဨ';
    /// \u{1029}: 'ဩ'
    pub const LETTER_O: char = 'ဩ';
    /// \u{102a}: 'ဪ'
    pub const LETTER_AU: char = 'ဪ';
    /// \u{102b}: 'ါ'
    pub const VOWEL_SIGN_TALL_AA: char = 'ါ';
    /// \u{102c}: 'ာ'
    pub const VOWEL_SIGN_AA: char = 'ာ';
    /// \u{102d}: 'ိ'
    pub const VOWEL_SIGN_I: char = 'ိ';
    /// \u{102e}: 'ီ'
    pub const VOWEL_SIGN_II: char = 'ီ';
    /// \u{102f}: 'ု'
    pub const VOWEL_SIGN_U: char = 'ု';
    /// \u{1030}: 'ူ'
    pub const VOWEL_SIGN_UU: char = 'ူ';
    /// \u{1031}: 'ေ'
    pub const VOWEL_SIGN_E: char = 'ေ';
    /// \u{1032}: 'ဲ'
    pub const VOWEL_SIGN_AI: char = 'ဲ';
    /// \u{1033}: 'ဳ'
    pub const VOWEL_SIGN_MON_II: char = 'ဳ';
    /// \u{1034}: 'ဴ'
    pub const VOWEL_SIGN_MON_O: char = 'ဴ';
    /// \u{1035}: 'ဵ'
    pub const VOWEL_SIGN_E_ABOVE: char = 'ဵ';
    /// \u{1036}: 'ံ'
    pub const SIGN_ANUSVARA: char = 'ံ';
    /// \u{1037}: '့'
    pub const SIGN_DOT_BELOW: char = '့';
    /// \u{1038}: 'း'
    pub const SIGN_VISARGA: char = 'း';
    /// \u{1039}: '္'
    pub const SIGN_VIRAMA: char = '္';
    /// \u{103a}: '်'
    pub const SIGN_ASAT: char = '်';
    /// \u{103b}: 'ျ'
    pub const CONSONANT_SIGN_MEDIAL_YA: char = 'ျ';
    /// \u{103c}: 'ြ'
    pub const CONSONANT_SIGN_MEDIAL_RA: char = 'ြ';
    /// \u{103d}: 'ွ'
    pub const CONSONANT_SIGN_MEDIAL_WA: char = 'ွ';
    /// \u{103e}: 'ှ'
    pub const CONSONANT_SIGN_MEDIAL_HA: char = 'ှ';
    /// \u{103f}: 'ဿ'
    pub const LETTER_GREAT_SA: char = 'ဿ';
    /// \u{1040}: '၀'
    pub const DIGIT_ZERO: char = '၀';
    /// \u{1041}: '၁'
    pub const DIGIT_ONE: char = '၁';
    /// \u{1042}: '၂'
    pub const DIGIT_TWO: char = '၂';
    /// \u{1043}: '၃'
    pub const DIGIT_THREE: char = '၃';
    /// \u{1044}: '၄'
    pub const DIGIT_FOUR: char = '၄';
    /// \u{1045}: '၅'
    pub const DIGIT_FIVE: char = '၅';
    /// \u{1046}: '၆'
    pub const DIGIT_SIX: char = '၆';
    /// \u{1047}: '၇'
    pub const DIGIT_SEVEN: char = '၇';
    /// \u{1048}: '၈'
    pub const DIGIT_EIGHT: char = '၈';
    /// \u{1049}: '၉'
    pub const DIGIT_NINE: char = '၉';
    /// \u{104a}: '၊'
    pub const SIGN_LITTLE_SECTION: char = '၊';
    /// \u{104b}: '။'
    pub const SIGN_SECTION: char = '။';
    /// \u{104c}: '၌'
    pub const SYMBOL_LOCATIVE: char = '၌';
    /// \u{104d}: '၍'
    pub const SYMBOL_COMPLETED: char = '၍';
    /// \u{104e}: '၎'
    pub const SYMBOL_AFOREMENTIONED: char = '၎';
    /// \u{104f}: '၏'
    pub const SYMBOL_GENITIVE: char = '၏';
    /// \u{1050}: 'ၐ'
    pub const LETTER_SHA: char = 'ၐ';
    /// \u{1051}: 'ၑ'
    pub const LETTER_SSA: char = 'ၑ';
    /// \u{1052}: 'ၒ'
    pub const LETTER_VOCALIC_R: char = 'ၒ';
    /// \u{1053}: 'ၓ'
    pub const LETTER_VOCALIC_RR: char = 'ၓ';
    /// \u{1054}: 'ၔ'
    pub const LETTER_VOCALIC_L: char = 'ၔ';
    /// \u{1055}: 'ၕ'
    pub const LETTER_VOCALIC_LL: char = 'ၕ';
    /// \u{1056}: 'ၖ'
    pub const VOWEL_SIGN_VOCALIC_R: char = 'ၖ';
    /// \u{1057}: 'ၗ'
    pub const VOWEL_SIGN_VOCALIC_RR: char = 'ၗ';
    /// \u{1058}: 'ၘ'
    pub const VOWEL_SIGN_VOCALIC_L: char = 'ၘ';
    /// \u{1059}: 'ၙ'
    pub const VOWEL_SIGN_VOCALIC_LL: char = 'ၙ';
    /// \u{105a}: 'ၚ'
    pub const LETTER_MON_NGA: char = 'ၚ';
    /// \u{105b}: 'ၛ'
    pub const LETTER_MON_JHA: char = 'ၛ';
    /// \u{105c}: 'ၜ'
    pub const LETTER_MON_BBA: char = 'ၜ';
    /// \u{105d}: 'ၝ'
    pub const LETTER_MON_BBE: char = 'ၝ';
    /// \u{105e}: 'ၞ'
    pub const CONSONANT_SIGN_MON_MEDIAL_NA: char = 'ၞ';
    /// \u{105f}: 'ၟ'
    pub const CONSONANT_SIGN_MON_MEDIAL_MA: char = 'ၟ';
    /// \u{1060}: 'ၠ'
    pub const CONSONANT_SIGN_MON_MEDIAL_LA: char = 'ၠ';
    /// \u{1061}: 'ၡ'
    pub const LETTER_SGAW_KAREN_SHA: char = 'ၡ';
    /// \u{1062}: 'ၢ'
    pub const VOWEL_SIGN_SGAW_KAREN_EU: char = 'ၢ';
    /// \u{1063}: 'ၣ'
    pub const TONE_MARK_SGAW_KAREN_HATHI: char = 'ၣ';
    /// \u{1064}: 'ၤ'
    pub const TONE_MARK_SGAW_KAREN_KE_PHO: char = 'ၤ';
    /// \u{1065}: 'ၥ'
    pub const LETTER_WESTERN_PWO_KAREN_THA: char = 'ၥ';
    /// \u{1066}: 'ၦ'
    pub const LETTER_WESTERN_PWO_KAREN_PWA: char = 'ၦ';
    /// \u{1067}: 'ၧ'
    pub const VOWEL_SIGN_WESTERN_PWO_KAREN_EU: char = 'ၧ';
    /// \u{1068}: 'ၨ'
    pub const VOWEL_SIGN_WESTERN_PWO_KAREN_UE: char = 'ၨ';
    /// \u{1069}: 'ၩ'
    pub const SIGN_WESTERN_PWO_KAREN_TONE_DASH_1: char = 'ၩ';
    /// \u{106a}: 'ၪ'
    pub const SIGN_WESTERN_PWO_KAREN_TONE_DASH_2: char = 'ၪ';
    /// \u{106b}: 'ၫ'
    pub const SIGN_WESTERN_PWO_KAREN_TONE_DASH_3: char = 'ၫ';
    /// \u{106c}: 'ၬ'
    pub const SIGN_WESTERN_PWO_KAREN_TONE_DASH_4: char = 'ၬ';
    /// \u{106d}: 'ၭ'
    pub const SIGN_WESTERN_PWO_KAREN_TONE_DASH_5: char = 'ၭ';
    /// \u{106e}: 'ၮ'
    pub const LETTER_EASTERN_PWO_KAREN_NNA: char = 'ၮ';
    /// \u{106f}: 'ၯ'
    pub const LETTER_EASTERN_PWO_KAREN_YWA: char = 'ၯ';
    /// \u{1070}: 'ၰ'
    pub const LETTER_EASTERN_PWO_KAREN_GHWA: char = 'ၰ';
    /// \u{1071}: 'ၱ'
    pub const VOWEL_SIGN_GEBA_KAREN_I: char = 'ၱ';
    /// \u{1072}: 'ၲ'
    pub const VOWEL_SIGN_KAYAH_OE: char = 'ၲ';
    /// \u{1073}: 'ၳ'
    pub const VOWEL_SIGN_KAYAH_U: char = 'ၳ';
    /// \u{1074}: 'ၴ'
    pub const VOWEL_SIGN_KAYAH_EE: char = 'ၴ';
    /// \u{1075}: 'ၵ'
    pub const LETTER_SHAN_KA: char = 'ၵ';
    /// \u{1076}: 'ၶ'
    pub const LETTER_SHAN_KHA: char = 'ၶ';
    /// \u{1077}: 'ၷ'
    pub const LETTER_SHAN_GA: char = 'ၷ';
    /// \u{1078}: 'ၸ'
    pub const LETTER_SHAN_CA: char = 'ၸ';
    /// \u{1079}: 'ၹ'
    pub const LETTER_SHAN_ZA: char = 'ၹ';
    /// \u{107a}: 'ၺ'
    pub const LETTER_SHAN_NYA: char = 'ၺ';
    /// \u{107b}: 'ၻ'
    pub const LETTER_SHAN_DA: char = 'ၻ';
    /// \u{107c}: 'ၼ'
    pub const LETTER_SHAN_NA: char = 'ၼ';
    /// \u{107d}: 'ၽ'
    pub const LETTER_SHAN_PHA: char = 'ၽ';
    /// \u{107e}: 'ၾ'
    pub const LETTER_SHAN_FA: char = 'ၾ';
    /// \u{107f}: 'ၿ'
    pub const LETTER_SHAN_BA: char = 'ၿ';
    /// \u{1080}: 'ႀ'
    pub const LETTER_SHAN_THA: char = 'ႀ';
    /// \u{1081}: 'ႁ'
    pub const LETTER_SHAN_HA: char = 'ႁ';
    /// \u{1082}: 'ႂ'
    pub const CONSONANT_SIGN_SHAN_MEDIAL_WA: char = 'ႂ';
    /// \u{1083}: 'ႃ'
    pub const VOWEL_SIGN_SHAN_AA: char = 'ႃ';
    /// \u{1084}: 'ႄ'
    pub const VOWEL_SIGN_SHAN_E: char = 'ႄ';
    /// \u{1085}: 'ႅ'
    pub const VOWEL_SIGN_SHAN_E_ABOVE: char = 'ႅ';
    /// \u{1086}: 'ႆ'
    pub const VOWEL_SIGN_SHAN_FINAL_Y: char = 'ႆ';
    /// \u{1087}: 'ႇ'
    pub const SIGN_SHAN_TONE_DASH_2: char = 'ႇ';
    /// \u{1088}: 'ႈ'
    pub const SIGN_SHAN_TONE_DASH_3: char = 'ႈ';
    /// \u{1089}: 'ႉ'
    pub const SIGN_SHAN_TONE_DASH_5: char = 'ႉ';
    /// \u{108a}: 'ႊ'
    pub const SIGN_SHAN_TONE_DASH_6: char = 'ႊ';
    /// \u{108b}: 'ႋ'
    pub const SIGN_SHAN_COUNCIL_TONE_DASH_2: char = 'ႋ';
    /// \u{108c}: 'ႌ'
    pub const SIGN_SHAN_COUNCIL_TONE_DASH_3: char = 'ႌ';
    /// \u{108d}: 'ႍ'
    pub const SIGN_SHAN_COUNCIL_EMPHATIC_TONE: char = 'ႍ';
    /// \u{108e}: 'ႎ'
    pub const LETTER_RUMAI_PALAUNG_FA: char = 'ႎ';
    /// \u{108f}: 'ႏ'
    pub const SIGN_RUMAI_PALAUNG_TONE_DASH_5: char = 'ႏ';
    /// \u{1090}: '႐'
    pub const SHAN_DIGIT_ZERO: char = '႐';
    /// \u{1091}: '႑'
    pub const SHAN_DIGIT_ONE: char = '႑';
    /// \u{1092}: '႒'
    pub const SHAN_DIGIT_TWO: char = '႒';
    /// \u{1093}: '႓'
    pub const SHAN_DIGIT_THREE: char = '႓';
    /// \u{1094}: '႔'
    pub const SHAN_DIGIT_FOUR: char = '႔';
    /// \u{1095}: '႕'
    pub const SHAN_DIGIT_FIVE: char = '႕';
    /// \u{1096}: '႖'
    pub const SHAN_DIGIT_SIX: char = '႖';
    /// \u{1097}: '႗'
    pub const SHAN_DIGIT_SEVEN: char = '႗';
    /// \u{1098}: '႘'
    pub const SHAN_DIGIT_EIGHT: char = '႘';
    /// \u{1099}: '႙'
    pub const SHAN_DIGIT_NINE: char = '႙';
    /// \u{109a}: 'ႚ'
    pub const SIGN_KHAMTI_TONE_DASH_1: char = 'ႚ';
    /// \u{109b}: 'ႛ'
    pub const SIGN_KHAMTI_TONE_DASH_3: char = 'ႛ';
    /// \u{109c}: 'ႜ'
    pub const VOWEL_SIGN_AITON_A: char = 'ႜ';
    /// \u{109d}: 'ႝ'
    pub const VOWEL_SIGN_AITON_AI: char = 'ႝ';
    /// \u{109e}: '႞'
    pub const SYMBOL_SHAN_ONE: char = '႞';
}

/// An enum to represent all characters in the Myanmar block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Myanmar {
    /// \u{1000}: 'က'
    LetterKa,
    /// \u{1001}: 'ခ'
    LetterKha,
    /// \u{1002}: 'ဂ'
    LetterGa,
    /// \u{1003}: 'ဃ'
    LetterGha,
    /// \u{1004}: 'င'
    LetterNga,
    /// \u{1005}: 'စ'
    LetterCa,
    /// \u{1006}: 'ဆ'
    LetterCha,
    /// \u{1007}: 'ဇ'
    LetterJa,
    /// \u{1008}: 'ဈ'
    LetterJha,
    /// \u{1009}: 'ဉ'
    LetterNya,
    /// \u{100a}: 'ည'
    LetterNnya,
    /// \u{100b}: 'ဋ'
    LetterTta,
    /// \u{100c}: 'ဌ'
    LetterTtha,
    /// \u{100d}: 'ဍ'
    LetterDda,
    /// \u{100e}: 'ဎ'
    LetterDdha,
    /// \u{100f}: 'ဏ'
    LetterNna,
    /// \u{1010}: 'တ'
    LetterTa,
    /// \u{1011}: 'ထ'
    LetterTha,
    /// \u{1012}: 'ဒ'
    LetterDa,
    /// \u{1013}: 'ဓ'
    LetterDha,
    /// \u{1014}: 'န'
    LetterNa,
    /// \u{1015}: 'ပ'
    LetterPa,
    /// \u{1016}: 'ဖ'
    LetterPha,
    /// \u{1017}: 'ဗ'
    LetterBa,
    /// \u{1018}: 'ဘ'
    LetterBha,
    /// \u{1019}: 'မ'
    LetterMa,
    /// \u{101a}: 'ယ'
    LetterYa,
    /// \u{101b}: 'ရ'
    LetterRa,
    /// \u{101c}: 'လ'
    LetterLa,
    /// \u{101d}: 'ဝ'
    LetterWa,
    /// \u{101e}: 'သ'
    LetterSa,
    /// \u{101f}: 'ဟ'
    LetterHa,
    /// \u{1020}: 'ဠ'
    LetterLla,
    /// \u{1021}: 'အ'
    LetterA,
    /// \u{1022}: 'ဢ'
    LetterShanA,
    /// \u{1023}: 'ဣ'
    LetterI,
    /// \u{1024}: 'ဤ'
    LetterIi,
    /// \u{1025}: 'ဥ'
    LetterU,
    /// \u{1026}: 'ဦ'
    LetterUu,
    /// \u{1027}: 'ဧ'
    LetterE,
    /// \u{1028}: 'ဨ'
    LetterMonE,
    /// \u{1029}: 'ဩ'
    LetterO,
    /// \u{102a}: 'ဪ'
    LetterAu,
    /// \u{102b}: 'ါ'
    VowelSignTallAa,
    /// \u{102c}: 'ာ'
    VowelSignAa,
    /// \u{102d}: 'ိ'
    VowelSignI,
    /// \u{102e}: 'ီ'
    VowelSignIi,
    /// \u{102f}: 'ု'
    VowelSignU,
    /// \u{1030}: 'ူ'
    VowelSignUu,
    /// \u{1031}: 'ေ'
    VowelSignE,
    /// \u{1032}: 'ဲ'
    VowelSignAi,
    /// \u{1033}: 'ဳ'
    VowelSignMonIi,
    /// \u{1034}: 'ဴ'
    VowelSignMonO,
    /// \u{1035}: 'ဵ'
    VowelSignEAbove,
    /// \u{1036}: 'ံ'
    SignAnusvara,
    /// \u{1037}: '့'
    SignDotBelow,
    /// \u{1038}: 'း'
    SignVisarga,
    /// \u{1039}: '္'
    SignVirama,
    /// \u{103a}: '်'
    SignAsat,
    /// \u{103b}: 'ျ'
    ConsonantSignMedialYa,
    /// \u{103c}: 'ြ'
    ConsonantSignMedialRa,
    /// \u{103d}: 'ွ'
    ConsonantSignMedialWa,
    /// \u{103e}: 'ှ'
    ConsonantSignMedialHa,
    /// \u{103f}: 'ဿ'
    LetterGreatSa,
    /// \u{1040}: '၀'
    DigitZero,
    /// \u{1041}: '၁'
    DigitOne,
    /// \u{1042}: '၂'
    DigitTwo,
    /// \u{1043}: '၃'
    DigitThree,
    /// \u{1044}: '၄'
    DigitFour,
    /// \u{1045}: '၅'
    DigitFive,
    /// \u{1046}: '၆'
    DigitSix,
    /// \u{1047}: '၇'
    DigitSeven,
    /// \u{1048}: '၈'
    DigitEight,
    /// \u{1049}: '၉'
    DigitNine,
    /// \u{104a}: '၊'
    SignLittleSection,
    /// \u{104b}: '။'
    SignSection,
    /// \u{104c}: '၌'
    SymbolLocative,
    /// \u{104d}: '၍'
    SymbolCompleted,
    /// \u{104e}: '၎'
    SymbolAforementioned,
    /// \u{104f}: '၏'
    SymbolGenitive,
    /// \u{1050}: 'ၐ'
    LetterSha,
    /// \u{1051}: 'ၑ'
    LetterSsa,
    /// \u{1052}: 'ၒ'
    LetterVocalicR,
    /// \u{1053}: 'ၓ'
    LetterVocalicRr,
    /// \u{1054}: 'ၔ'
    LetterVocalicL,
    /// \u{1055}: 'ၕ'
    LetterVocalicLl,
    /// \u{1056}: 'ၖ'
    VowelSignVocalicR,
    /// \u{1057}: 'ၗ'
    VowelSignVocalicRr,
    /// \u{1058}: 'ၘ'
    VowelSignVocalicL,
    /// \u{1059}: 'ၙ'
    VowelSignVocalicLl,
    /// \u{105a}: 'ၚ'
    LetterMonNga,
    /// \u{105b}: 'ၛ'
    LetterMonJha,
    /// \u{105c}: 'ၜ'
    LetterMonBba,
    /// \u{105d}: 'ၝ'
    LetterMonBbe,
    /// \u{105e}: 'ၞ'
    ConsonantSignMonMedialNa,
    /// \u{105f}: 'ၟ'
    ConsonantSignMonMedialMa,
    /// \u{1060}: 'ၠ'
    ConsonantSignMonMedialLa,
    /// \u{1061}: 'ၡ'
    LetterSgawKarenSha,
    /// \u{1062}: 'ၢ'
    VowelSignSgawKarenEu,
    /// \u{1063}: 'ၣ'
    ToneMarkSgawKarenHathi,
    /// \u{1064}: 'ၤ'
    ToneMarkSgawKarenKePho,
    /// \u{1065}: 'ၥ'
    LetterWesternPwoKarenTha,
    /// \u{1066}: 'ၦ'
    LetterWesternPwoKarenPwa,
    /// \u{1067}: 'ၧ'
    VowelSignWesternPwoKarenEu,
    /// \u{1068}: 'ၨ'
    VowelSignWesternPwoKarenUe,
    /// \u{1069}: 'ၩ'
    SignWesternPwoKarenToneDash1,
    /// \u{106a}: 'ၪ'
    SignWesternPwoKarenToneDash2,
    /// \u{106b}: 'ၫ'
    SignWesternPwoKarenToneDash3,
    /// \u{106c}: 'ၬ'
    SignWesternPwoKarenToneDash4,
    /// \u{106d}: 'ၭ'
    SignWesternPwoKarenToneDash5,
    /// \u{106e}: 'ၮ'
    LetterEasternPwoKarenNna,
    /// \u{106f}: 'ၯ'
    LetterEasternPwoKarenYwa,
    /// \u{1070}: 'ၰ'
    LetterEasternPwoKarenGhwa,
    /// \u{1071}: 'ၱ'
    VowelSignGebaKarenI,
    /// \u{1072}: 'ၲ'
    VowelSignKayahOe,
    /// \u{1073}: 'ၳ'
    VowelSignKayahU,
    /// \u{1074}: 'ၴ'
    VowelSignKayahEe,
    /// \u{1075}: 'ၵ'
    LetterShanKa,
    /// \u{1076}: 'ၶ'
    LetterShanKha,
    /// \u{1077}: 'ၷ'
    LetterShanGa,
    /// \u{1078}: 'ၸ'
    LetterShanCa,
    /// \u{1079}: 'ၹ'
    LetterShanZa,
    /// \u{107a}: 'ၺ'
    LetterShanNya,
    /// \u{107b}: 'ၻ'
    LetterShanDa,
    /// \u{107c}: 'ၼ'
    LetterShanNa,
    /// \u{107d}: 'ၽ'
    LetterShanPha,
    /// \u{107e}: 'ၾ'
    LetterShanFa,
    /// \u{107f}: 'ၿ'
    LetterShanBa,
    /// \u{1080}: 'ႀ'
    LetterShanTha,
    /// \u{1081}: 'ႁ'
    LetterShanHa,
    /// \u{1082}: 'ႂ'
    ConsonantSignShanMedialWa,
    /// \u{1083}: 'ႃ'
    VowelSignShanAa,
    /// \u{1084}: 'ႄ'
    VowelSignShanE,
    /// \u{1085}: 'ႅ'
    VowelSignShanEAbove,
    /// \u{1086}: 'ႆ'
    VowelSignShanFinalY,
    /// \u{1087}: 'ႇ'
    SignShanToneDash2,
    /// \u{1088}: 'ႈ'
    SignShanToneDash3,
    /// \u{1089}: 'ႉ'
    SignShanToneDash5,
    /// \u{108a}: 'ႊ'
    SignShanToneDash6,
    /// \u{108b}: 'ႋ'
    SignShanCouncilToneDash2,
    /// \u{108c}: 'ႌ'
    SignShanCouncilToneDash3,
    /// \u{108d}: 'ႍ'
    SignShanCouncilEmphaticTone,
    /// \u{108e}: 'ႎ'
    LetterRumaiPalaungFa,
    /// \u{108f}: 'ႏ'
    SignRumaiPalaungToneDash5,
    /// \u{1090}: '႐'
    ShanDigitZero,
    /// \u{1091}: '႑'
    ShanDigitOne,
    /// \u{1092}: '႒'
    ShanDigitTwo,
    /// \u{1093}: '႓'
    ShanDigitThree,
    /// \u{1094}: '႔'
    ShanDigitFour,
    /// \u{1095}: '႕'
    ShanDigitFive,
    /// \u{1096}: '႖'
    ShanDigitSix,
    /// \u{1097}: '႗'
    ShanDigitSeven,
    /// \u{1098}: '႘'
    ShanDigitEight,
    /// \u{1099}: '႙'
    ShanDigitNine,
    /// \u{109a}: 'ႚ'
    SignKhamtiToneDash1,
    /// \u{109b}: 'ႛ'
    SignKhamtiToneDash3,
    /// \u{109c}: 'ႜ'
    VowelSignAitonA,
    /// \u{109d}: 'ႝ'
    VowelSignAitonAi,
    /// \u{109e}: '႞'
    SymbolShanOne,
}

impl Into<char> for Myanmar {
    fn into(self) -> char {
        use constants::*;
        match self {
            Myanmar::LetterKa => LETTER_KA,
            Myanmar::LetterKha => LETTER_KHA,
            Myanmar::LetterGa => LETTER_GA,
            Myanmar::LetterGha => LETTER_GHA,
            Myanmar::LetterNga => LETTER_NGA,
            Myanmar::LetterCa => LETTER_CA,
            Myanmar::LetterCha => LETTER_CHA,
            Myanmar::LetterJa => LETTER_JA,
            Myanmar::LetterJha => LETTER_JHA,
            Myanmar::LetterNya => LETTER_NYA,
            Myanmar::LetterNnya => LETTER_NNYA,
            Myanmar::LetterTta => LETTER_TTA,
            Myanmar::LetterTtha => LETTER_TTHA,
            Myanmar::LetterDda => LETTER_DDA,
            Myanmar::LetterDdha => LETTER_DDHA,
            Myanmar::LetterNna => LETTER_NNA,
            Myanmar::LetterTa => LETTER_TA,
            Myanmar::LetterTha => LETTER_THA,
            Myanmar::LetterDa => LETTER_DA,
            Myanmar::LetterDha => LETTER_DHA,
            Myanmar::LetterNa => LETTER_NA,
            Myanmar::LetterPa => LETTER_PA,
            Myanmar::LetterPha => LETTER_PHA,
            Myanmar::LetterBa => LETTER_BA,
            Myanmar::LetterBha => LETTER_BHA,
            Myanmar::LetterMa => LETTER_MA,
            Myanmar::LetterYa => LETTER_YA,
            Myanmar::LetterRa => LETTER_RA,
            Myanmar::LetterLa => LETTER_LA,
            Myanmar::LetterWa => LETTER_WA,
            Myanmar::LetterSa => LETTER_SA,
            Myanmar::LetterHa => LETTER_HA,
            Myanmar::LetterLla => LETTER_LLA,
            Myanmar::LetterA => LETTER_A,
            Myanmar::LetterShanA => LETTER_SHAN_A,
            Myanmar::LetterI => LETTER_I,
            Myanmar::LetterIi => LETTER_II,
            Myanmar::LetterU => LETTER_U,
            Myanmar::LetterUu => LETTER_UU,
            Myanmar::LetterE => LETTER_E,
            Myanmar::LetterMonE => LETTER_MON_E,
            Myanmar::LetterO => LETTER_O,
            Myanmar::LetterAu => LETTER_AU,
            Myanmar::VowelSignTallAa => VOWEL_SIGN_TALL_AA,
            Myanmar::VowelSignAa => VOWEL_SIGN_AA,
            Myanmar::VowelSignI => VOWEL_SIGN_I,
            Myanmar::VowelSignIi => VOWEL_SIGN_II,
            Myanmar::VowelSignU => VOWEL_SIGN_U,
            Myanmar::VowelSignUu => VOWEL_SIGN_UU,
            Myanmar::VowelSignE => VOWEL_SIGN_E,
            Myanmar::VowelSignAi => VOWEL_SIGN_AI,
            Myanmar::VowelSignMonIi => VOWEL_SIGN_MON_II,
            Myanmar::VowelSignMonO => VOWEL_SIGN_MON_O,
            Myanmar::VowelSignEAbove => VOWEL_SIGN_E_ABOVE,
            Myanmar::SignAnusvara => SIGN_ANUSVARA,
            Myanmar::SignDotBelow => SIGN_DOT_BELOW,
            Myanmar::SignVisarga => SIGN_VISARGA,
            Myanmar::SignVirama => SIGN_VIRAMA,
            Myanmar::SignAsat => SIGN_ASAT,
            Myanmar::ConsonantSignMedialYa => CONSONANT_SIGN_MEDIAL_YA,
            Myanmar::ConsonantSignMedialRa => CONSONANT_SIGN_MEDIAL_RA,
            Myanmar::ConsonantSignMedialWa => CONSONANT_SIGN_MEDIAL_WA,
            Myanmar::ConsonantSignMedialHa => CONSONANT_SIGN_MEDIAL_HA,
            Myanmar::LetterGreatSa => LETTER_GREAT_SA,
            Myanmar::DigitZero => DIGIT_ZERO,
            Myanmar::DigitOne => DIGIT_ONE,
            Myanmar::DigitTwo => DIGIT_TWO,
            Myanmar::DigitThree => DIGIT_THREE,
            Myanmar::DigitFour => DIGIT_FOUR,
            Myanmar::DigitFive => DIGIT_FIVE,
            Myanmar::DigitSix => DIGIT_SIX,
            Myanmar::DigitSeven => DIGIT_SEVEN,
            Myanmar::DigitEight => DIGIT_EIGHT,
            Myanmar::DigitNine => DIGIT_NINE,
            Myanmar::SignLittleSection => SIGN_LITTLE_SECTION,
            Myanmar::SignSection => SIGN_SECTION,
            Myanmar::SymbolLocative => SYMBOL_LOCATIVE,
            Myanmar::SymbolCompleted => SYMBOL_COMPLETED,
            Myanmar::SymbolAforementioned => SYMBOL_AFOREMENTIONED,
            Myanmar::SymbolGenitive => SYMBOL_GENITIVE,
            Myanmar::LetterSha => LETTER_SHA,
            Myanmar::LetterSsa => LETTER_SSA,
            Myanmar::LetterVocalicR => LETTER_VOCALIC_R,
            Myanmar::LetterVocalicRr => LETTER_VOCALIC_RR,
            Myanmar::LetterVocalicL => LETTER_VOCALIC_L,
            Myanmar::LetterVocalicLl => LETTER_VOCALIC_LL,
            Myanmar::VowelSignVocalicR => VOWEL_SIGN_VOCALIC_R,
            Myanmar::VowelSignVocalicRr => VOWEL_SIGN_VOCALIC_RR,
            Myanmar::VowelSignVocalicL => VOWEL_SIGN_VOCALIC_L,
            Myanmar::VowelSignVocalicLl => VOWEL_SIGN_VOCALIC_LL,
            Myanmar::LetterMonNga => LETTER_MON_NGA,
            Myanmar::LetterMonJha => LETTER_MON_JHA,
            Myanmar::LetterMonBba => LETTER_MON_BBA,
            Myanmar::LetterMonBbe => LETTER_MON_BBE,
            Myanmar::ConsonantSignMonMedialNa => CONSONANT_SIGN_MON_MEDIAL_NA,
            Myanmar::ConsonantSignMonMedialMa => CONSONANT_SIGN_MON_MEDIAL_MA,
            Myanmar::ConsonantSignMonMedialLa => CONSONANT_SIGN_MON_MEDIAL_LA,
            Myanmar::LetterSgawKarenSha => LETTER_SGAW_KAREN_SHA,
            Myanmar::VowelSignSgawKarenEu => VOWEL_SIGN_SGAW_KAREN_EU,
            Myanmar::ToneMarkSgawKarenHathi => TONE_MARK_SGAW_KAREN_HATHI,
            Myanmar::ToneMarkSgawKarenKePho => TONE_MARK_SGAW_KAREN_KE_PHO,
            Myanmar::LetterWesternPwoKarenTha => LETTER_WESTERN_PWO_KAREN_THA,
            Myanmar::LetterWesternPwoKarenPwa => LETTER_WESTERN_PWO_KAREN_PWA,
            Myanmar::VowelSignWesternPwoKarenEu => VOWEL_SIGN_WESTERN_PWO_KAREN_EU,
            Myanmar::VowelSignWesternPwoKarenUe => VOWEL_SIGN_WESTERN_PWO_KAREN_UE,
            Myanmar::SignWesternPwoKarenToneDash1 => SIGN_WESTERN_PWO_KAREN_TONE_DASH_1,
            Myanmar::SignWesternPwoKarenToneDash2 => SIGN_WESTERN_PWO_KAREN_TONE_DASH_2,
            Myanmar::SignWesternPwoKarenToneDash3 => SIGN_WESTERN_PWO_KAREN_TONE_DASH_3,
            Myanmar::SignWesternPwoKarenToneDash4 => SIGN_WESTERN_PWO_KAREN_TONE_DASH_4,
            Myanmar::SignWesternPwoKarenToneDash5 => SIGN_WESTERN_PWO_KAREN_TONE_DASH_5,
            Myanmar::LetterEasternPwoKarenNna => LETTER_EASTERN_PWO_KAREN_NNA,
            Myanmar::LetterEasternPwoKarenYwa => LETTER_EASTERN_PWO_KAREN_YWA,
            Myanmar::LetterEasternPwoKarenGhwa => LETTER_EASTERN_PWO_KAREN_GHWA,
            Myanmar::VowelSignGebaKarenI => VOWEL_SIGN_GEBA_KAREN_I,
            Myanmar::VowelSignKayahOe => VOWEL_SIGN_KAYAH_OE,
            Myanmar::VowelSignKayahU => VOWEL_SIGN_KAYAH_U,
            Myanmar::VowelSignKayahEe => VOWEL_SIGN_KAYAH_EE,
            Myanmar::LetterShanKa => LETTER_SHAN_KA,
            Myanmar::LetterShanKha => LETTER_SHAN_KHA,
            Myanmar::LetterShanGa => LETTER_SHAN_GA,
            Myanmar::LetterShanCa => LETTER_SHAN_CA,
            Myanmar::LetterShanZa => LETTER_SHAN_ZA,
            Myanmar::LetterShanNya => LETTER_SHAN_NYA,
            Myanmar::LetterShanDa => LETTER_SHAN_DA,
            Myanmar::LetterShanNa => LETTER_SHAN_NA,
            Myanmar::LetterShanPha => LETTER_SHAN_PHA,
            Myanmar::LetterShanFa => LETTER_SHAN_FA,
            Myanmar::LetterShanBa => LETTER_SHAN_BA,
            Myanmar::LetterShanTha => LETTER_SHAN_THA,
            Myanmar::LetterShanHa => LETTER_SHAN_HA,
            Myanmar::ConsonantSignShanMedialWa => CONSONANT_SIGN_SHAN_MEDIAL_WA,
            Myanmar::VowelSignShanAa => VOWEL_SIGN_SHAN_AA,
            Myanmar::VowelSignShanE => VOWEL_SIGN_SHAN_E,
            Myanmar::VowelSignShanEAbove => VOWEL_SIGN_SHAN_E_ABOVE,
            Myanmar::VowelSignShanFinalY => VOWEL_SIGN_SHAN_FINAL_Y,
            Myanmar::SignShanToneDash2 => SIGN_SHAN_TONE_DASH_2,
            Myanmar::SignShanToneDash3 => SIGN_SHAN_TONE_DASH_3,
            Myanmar::SignShanToneDash5 => SIGN_SHAN_TONE_DASH_5,
            Myanmar::SignShanToneDash6 => SIGN_SHAN_TONE_DASH_6,
            Myanmar::SignShanCouncilToneDash2 => SIGN_SHAN_COUNCIL_TONE_DASH_2,
            Myanmar::SignShanCouncilToneDash3 => SIGN_SHAN_COUNCIL_TONE_DASH_3,
            Myanmar::SignShanCouncilEmphaticTone => SIGN_SHAN_COUNCIL_EMPHATIC_TONE,
            Myanmar::LetterRumaiPalaungFa => LETTER_RUMAI_PALAUNG_FA,
            Myanmar::SignRumaiPalaungToneDash5 => SIGN_RUMAI_PALAUNG_TONE_DASH_5,
            Myanmar::ShanDigitZero => SHAN_DIGIT_ZERO,
            Myanmar::ShanDigitOne => SHAN_DIGIT_ONE,
            Myanmar::ShanDigitTwo => SHAN_DIGIT_TWO,
            Myanmar::ShanDigitThree => SHAN_DIGIT_THREE,
            Myanmar::ShanDigitFour => SHAN_DIGIT_FOUR,
            Myanmar::ShanDigitFive => SHAN_DIGIT_FIVE,
            Myanmar::ShanDigitSix => SHAN_DIGIT_SIX,
            Myanmar::ShanDigitSeven => SHAN_DIGIT_SEVEN,
            Myanmar::ShanDigitEight => SHAN_DIGIT_EIGHT,
            Myanmar::ShanDigitNine => SHAN_DIGIT_NINE,
            Myanmar::SignKhamtiToneDash1 => SIGN_KHAMTI_TONE_DASH_1,
            Myanmar::SignKhamtiToneDash3 => SIGN_KHAMTI_TONE_DASH_3,
            Myanmar::VowelSignAitonA => VOWEL_SIGN_AITON_A,
            Myanmar::VowelSignAitonAi => VOWEL_SIGN_AITON_AI,
            Myanmar::SymbolShanOne => SYMBOL_SHAN_ONE,
        }
    }
}

impl std::convert::TryFrom<char> for Myanmar {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_KA => Ok(Myanmar::LetterKa),
            LETTER_KHA => Ok(Myanmar::LetterKha),
            LETTER_GA => Ok(Myanmar::LetterGa),
            LETTER_GHA => Ok(Myanmar::LetterGha),
            LETTER_NGA => Ok(Myanmar::LetterNga),
            LETTER_CA => Ok(Myanmar::LetterCa),
            LETTER_CHA => Ok(Myanmar::LetterCha),
            LETTER_JA => Ok(Myanmar::LetterJa),
            LETTER_JHA => Ok(Myanmar::LetterJha),
            LETTER_NYA => Ok(Myanmar::LetterNya),
            LETTER_NNYA => Ok(Myanmar::LetterNnya),
            LETTER_TTA => Ok(Myanmar::LetterTta),
            LETTER_TTHA => Ok(Myanmar::LetterTtha),
            LETTER_DDA => Ok(Myanmar::LetterDda),
            LETTER_DDHA => Ok(Myanmar::LetterDdha),
            LETTER_NNA => Ok(Myanmar::LetterNna),
            LETTER_TA => Ok(Myanmar::LetterTa),
            LETTER_THA => Ok(Myanmar::LetterTha),
            LETTER_DA => Ok(Myanmar::LetterDa),
            LETTER_DHA => Ok(Myanmar::LetterDha),
            LETTER_NA => Ok(Myanmar::LetterNa),
            LETTER_PA => Ok(Myanmar::LetterPa),
            LETTER_PHA => Ok(Myanmar::LetterPha),
            LETTER_BA => Ok(Myanmar::LetterBa),
            LETTER_BHA => Ok(Myanmar::LetterBha),
            LETTER_MA => Ok(Myanmar::LetterMa),
            LETTER_YA => Ok(Myanmar::LetterYa),
            LETTER_RA => Ok(Myanmar::LetterRa),
            LETTER_LA => Ok(Myanmar::LetterLa),
            LETTER_WA => Ok(Myanmar::LetterWa),
            LETTER_SA => Ok(Myanmar::LetterSa),
            LETTER_HA => Ok(Myanmar::LetterHa),
            LETTER_LLA => Ok(Myanmar::LetterLla),
            LETTER_A => Ok(Myanmar::LetterA),
            LETTER_SHAN_A => Ok(Myanmar::LetterShanA),
            LETTER_I => Ok(Myanmar::LetterI),
            LETTER_II => Ok(Myanmar::LetterIi),
            LETTER_U => Ok(Myanmar::LetterU),
            LETTER_UU => Ok(Myanmar::LetterUu),
            LETTER_E => Ok(Myanmar::LetterE),
            LETTER_MON_E => Ok(Myanmar::LetterMonE),
            LETTER_O => Ok(Myanmar::LetterO),
            LETTER_AU => Ok(Myanmar::LetterAu),
            VOWEL_SIGN_TALL_AA => Ok(Myanmar::VowelSignTallAa),
            VOWEL_SIGN_AA => Ok(Myanmar::VowelSignAa),
            VOWEL_SIGN_I => Ok(Myanmar::VowelSignI),
            VOWEL_SIGN_II => Ok(Myanmar::VowelSignIi),
            VOWEL_SIGN_U => Ok(Myanmar::VowelSignU),
            VOWEL_SIGN_UU => Ok(Myanmar::VowelSignUu),
            VOWEL_SIGN_E => Ok(Myanmar::VowelSignE),
            VOWEL_SIGN_AI => Ok(Myanmar::VowelSignAi),
            VOWEL_SIGN_MON_II => Ok(Myanmar::VowelSignMonIi),
            VOWEL_SIGN_MON_O => Ok(Myanmar::VowelSignMonO),
            VOWEL_SIGN_E_ABOVE => Ok(Myanmar::VowelSignEAbove),
            SIGN_ANUSVARA => Ok(Myanmar::SignAnusvara),
            SIGN_DOT_BELOW => Ok(Myanmar::SignDotBelow),
            SIGN_VISARGA => Ok(Myanmar::SignVisarga),
            SIGN_VIRAMA => Ok(Myanmar::SignVirama),
            SIGN_ASAT => Ok(Myanmar::SignAsat),
            CONSONANT_SIGN_MEDIAL_YA => Ok(Myanmar::ConsonantSignMedialYa),
            CONSONANT_SIGN_MEDIAL_RA => Ok(Myanmar::ConsonantSignMedialRa),
            CONSONANT_SIGN_MEDIAL_WA => Ok(Myanmar::ConsonantSignMedialWa),
            CONSONANT_SIGN_MEDIAL_HA => Ok(Myanmar::ConsonantSignMedialHa),
            LETTER_GREAT_SA => Ok(Myanmar::LetterGreatSa),
            DIGIT_ZERO => Ok(Myanmar::DigitZero),
            DIGIT_ONE => Ok(Myanmar::DigitOne),
            DIGIT_TWO => Ok(Myanmar::DigitTwo),
            DIGIT_THREE => Ok(Myanmar::DigitThree),
            DIGIT_FOUR => Ok(Myanmar::DigitFour),
            DIGIT_FIVE => Ok(Myanmar::DigitFive),
            DIGIT_SIX => Ok(Myanmar::DigitSix),
            DIGIT_SEVEN => Ok(Myanmar::DigitSeven),
            DIGIT_EIGHT => Ok(Myanmar::DigitEight),
            DIGIT_NINE => Ok(Myanmar::DigitNine),
            SIGN_LITTLE_SECTION => Ok(Myanmar::SignLittleSection),
            SIGN_SECTION => Ok(Myanmar::SignSection),
            SYMBOL_LOCATIVE => Ok(Myanmar::SymbolLocative),
            SYMBOL_COMPLETED => Ok(Myanmar::SymbolCompleted),
            SYMBOL_AFOREMENTIONED => Ok(Myanmar::SymbolAforementioned),
            SYMBOL_GENITIVE => Ok(Myanmar::SymbolGenitive),
            LETTER_SHA => Ok(Myanmar::LetterSha),
            LETTER_SSA => Ok(Myanmar::LetterSsa),
            LETTER_VOCALIC_R => Ok(Myanmar::LetterVocalicR),
            LETTER_VOCALIC_RR => Ok(Myanmar::LetterVocalicRr),
            LETTER_VOCALIC_L => Ok(Myanmar::LetterVocalicL),
            LETTER_VOCALIC_LL => Ok(Myanmar::LetterVocalicLl),
            VOWEL_SIGN_VOCALIC_R => Ok(Myanmar::VowelSignVocalicR),
            VOWEL_SIGN_VOCALIC_RR => Ok(Myanmar::VowelSignVocalicRr),
            VOWEL_SIGN_VOCALIC_L => Ok(Myanmar::VowelSignVocalicL),
            VOWEL_SIGN_VOCALIC_LL => Ok(Myanmar::VowelSignVocalicLl),
            LETTER_MON_NGA => Ok(Myanmar::LetterMonNga),
            LETTER_MON_JHA => Ok(Myanmar::LetterMonJha),
            LETTER_MON_BBA => Ok(Myanmar::LetterMonBba),
            LETTER_MON_BBE => Ok(Myanmar::LetterMonBbe),
            CONSONANT_SIGN_MON_MEDIAL_NA => Ok(Myanmar::ConsonantSignMonMedialNa),
            CONSONANT_SIGN_MON_MEDIAL_MA => Ok(Myanmar::ConsonantSignMonMedialMa),
            CONSONANT_SIGN_MON_MEDIAL_LA => Ok(Myanmar::ConsonantSignMonMedialLa),
            LETTER_SGAW_KAREN_SHA => Ok(Myanmar::LetterSgawKarenSha),
            VOWEL_SIGN_SGAW_KAREN_EU => Ok(Myanmar::VowelSignSgawKarenEu),
            TONE_MARK_SGAW_KAREN_HATHI => Ok(Myanmar::ToneMarkSgawKarenHathi),
            TONE_MARK_SGAW_KAREN_KE_PHO => Ok(Myanmar::ToneMarkSgawKarenKePho),
            LETTER_WESTERN_PWO_KAREN_THA => Ok(Myanmar::LetterWesternPwoKarenTha),
            LETTER_WESTERN_PWO_KAREN_PWA => Ok(Myanmar::LetterWesternPwoKarenPwa),
            VOWEL_SIGN_WESTERN_PWO_KAREN_EU => Ok(Myanmar::VowelSignWesternPwoKarenEu),
            VOWEL_SIGN_WESTERN_PWO_KAREN_UE => Ok(Myanmar::VowelSignWesternPwoKarenUe),
            SIGN_WESTERN_PWO_KAREN_TONE_DASH_1 => Ok(Myanmar::SignWesternPwoKarenToneDash1),
            SIGN_WESTERN_PWO_KAREN_TONE_DASH_2 => Ok(Myanmar::SignWesternPwoKarenToneDash2),
            SIGN_WESTERN_PWO_KAREN_TONE_DASH_3 => Ok(Myanmar::SignWesternPwoKarenToneDash3),
            SIGN_WESTERN_PWO_KAREN_TONE_DASH_4 => Ok(Myanmar::SignWesternPwoKarenToneDash4),
            SIGN_WESTERN_PWO_KAREN_TONE_DASH_5 => Ok(Myanmar::SignWesternPwoKarenToneDash5),
            LETTER_EASTERN_PWO_KAREN_NNA => Ok(Myanmar::LetterEasternPwoKarenNna),
            LETTER_EASTERN_PWO_KAREN_YWA => Ok(Myanmar::LetterEasternPwoKarenYwa),
            LETTER_EASTERN_PWO_KAREN_GHWA => Ok(Myanmar::LetterEasternPwoKarenGhwa),
            VOWEL_SIGN_GEBA_KAREN_I => Ok(Myanmar::VowelSignGebaKarenI),
            VOWEL_SIGN_KAYAH_OE => Ok(Myanmar::VowelSignKayahOe),
            VOWEL_SIGN_KAYAH_U => Ok(Myanmar::VowelSignKayahU),
            VOWEL_SIGN_KAYAH_EE => Ok(Myanmar::VowelSignKayahEe),
            LETTER_SHAN_KA => Ok(Myanmar::LetterShanKa),
            LETTER_SHAN_KHA => Ok(Myanmar::LetterShanKha),
            LETTER_SHAN_GA => Ok(Myanmar::LetterShanGa),
            LETTER_SHAN_CA => Ok(Myanmar::LetterShanCa),
            LETTER_SHAN_ZA => Ok(Myanmar::LetterShanZa),
            LETTER_SHAN_NYA => Ok(Myanmar::LetterShanNya),
            LETTER_SHAN_DA => Ok(Myanmar::LetterShanDa),
            LETTER_SHAN_NA => Ok(Myanmar::LetterShanNa),
            LETTER_SHAN_PHA => Ok(Myanmar::LetterShanPha),
            LETTER_SHAN_FA => Ok(Myanmar::LetterShanFa),
            LETTER_SHAN_BA => Ok(Myanmar::LetterShanBa),
            LETTER_SHAN_THA => Ok(Myanmar::LetterShanTha),
            LETTER_SHAN_HA => Ok(Myanmar::LetterShanHa),
            CONSONANT_SIGN_SHAN_MEDIAL_WA => Ok(Myanmar::ConsonantSignShanMedialWa),
            VOWEL_SIGN_SHAN_AA => Ok(Myanmar::VowelSignShanAa),
            VOWEL_SIGN_SHAN_E => Ok(Myanmar::VowelSignShanE),
            VOWEL_SIGN_SHAN_E_ABOVE => Ok(Myanmar::VowelSignShanEAbove),
            VOWEL_SIGN_SHAN_FINAL_Y => Ok(Myanmar::VowelSignShanFinalY),
            SIGN_SHAN_TONE_DASH_2 => Ok(Myanmar::SignShanToneDash2),
            SIGN_SHAN_TONE_DASH_3 => Ok(Myanmar::SignShanToneDash3),
            SIGN_SHAN_TONE_DASH_5 => Ok(Myanmar::SignShanToneDash5),
            SIGN_SHAN_TONE_DASH_6 => Ok(Myanmar::SignShanToneDash6),
            SIGN_SHAN_COUNCIL_TONE_DASH_2 => Ok(Myanmar::SignShanCouncilToneDash2),
            SIGN_SHAN_COUNCIL_TONE_DASH_3 => Ok(Myanmar::SignShanCouncilToneDash3),
            SIGN_SHAN_COUNCIL_EMPHATIC_TONE => Ok(Myanmar::SignShanCouncilEmphaticTone),
            LETTER_RUMAI_PALAUNG_FA => Ok(Myanmar::LetterRumaiPalaungFa),
            SIGN_RUMAI_PALAUNG_TONE_DASH_5 => Ok(Myanmar::SignRumaiPalaungToneDash5),
            SHAN_DIGIT_ZERO => Ok(Myanmar::ShanDigitZero),
            SHAN_DIGIT_ONE => Ok(Myanmar::ShanDigitOne),
            SHAN_DIGIT_TWO => Ok(Myanmar::ShanDigitTwo),
            SHAN_DIGIT_THREE => Ok(Myanmar::ShanDigitThree),
            SHAN_DIGIT_FOUR => Ok(Myanmar::ShanDigitFour),
            SHAN_DIGIT_FIVE => Ok(Myanmar::ShanDigitFive),
            SHAN_DIGIT_SIX => Ok(Myanmar::ShanDigitSix),
            SHAN_DIGIT_SEVEN => Ok(Myanmar::ShanDigitSeven),
            SHAN_DIGIT_EIGHT => Ok(Myanmar::ShanDigitEight),
            SHAN_DIGIT_NINE => Ok(Myanmar::ShanDigitNine),
            SIGN_KHAMTI_TONE_DASH_1 => Ok(Myanmar::SignKhamtiToneDash1),
            SIGN_KHAMTI_TONE_DASH_3 => Ok(Myanmar::SignKhamtiToneDash3),
            VOWEL_SIGN_AITON_A => Ok(Myanmar::VowelSignAitonA),
            VOWEL_SIGN_AITON_AI => Ok(Myanmar::VowelSignAitonAi),
            SYMBOL_SHAN_ONE => Ok(Myanmar::SymbolShanOne),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Myanmar {
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

impl std::convert::TryFrom<u32> for Myanmar {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Myanmar {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Myanmar {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Myanmar::LetterKa
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Myanmar::LetterKa => "myanmar letter ka",
            Myanmar::LetterKha => "myanmar letter kha",
            Myanmar::LetterGa => "myanmar letter ga",
            Myanmar::LetterGha => "myanmar letter gha",
            Myanmar::LetterNga => "myanmar letter nga",
            Myanmar::LetterCa => "myanmar letter ca",
            Myanmar::LetterCha => "myanmar letter cha",
            Myanmar::LetterJa => "myanmar letter ja",
            Myanmar::LetterJha => "myanmar letter jha",
            Myanmar::LetterNya => "myanmar letter nya",
            Myanmar::LetterNnya => "myanmar letter nnya",
            Myanmar::LetterTta => "myanmar letter tta",
            Myanmar::LetterTtha => "myanmar letter ttha",
            Myanmar::LetterDda => "myanmar letter dda",
            Myanmar::LetterDdha => "myanmar letter ddha",
            Myanmar::LetterNna => "myanmar letter nna",
            Myanmar::LetterTa => "myanmar letter ta",
            Myanmar::LetterTha => "myanmar letter tha",
            Myanmar::LetterDa => "myanmar letter da",
            Myanmar::LetterDha => "myanmar letter dha",
            Myanmar::LetterNa => "myanmar letter na",
            Myanmar::LetterPa => "myanmar letter pa",
            Myanmar::LetterPha => "myanmar letter pha",
            Myanmar::LetterBa => "myanmar letter ba",
            Myanmar::LetterBha => "myanmar letter bha",
            Myanmar::LetterMa => "myanmar letter ma",
            Myanmar::LetterYa => "myanmar letter ya",
            Myanmar::LetterRa => "myanmar letter ra",
            Myanmar::LetterLa => "myanmar letter la",
            Myanmar::LetterWa => "myanmar letter wa",
            Myanmar::LetterSa => "myanmar letter sa",
            Myanmar::LetterHa => "myanmar letter ha",
            Myanmar::LetterLla => "myanmar letter lla",
            Myanmar::LetterA => "myanmar letter a",
            Myanmar::LetterShanA => "myanmar letter shan a",
            Myanmar::LetterI => "myanmar letter i",
            Myanmar::LetterIi => "myanmar letter ii",
            Myanmar::LetterU => "myanmar letter u",
            Myanmar::LetterUu => "myanmar letter uu",
            Myanmar::LetterE => "myanmar letter e",
            Myanmar::LetterMonE => "myanmar letter mon e",
            Myanmar::LetterO => "myanmar letter o",
            Myanmar::LetterAu => "myanmar letter au",
            Myanmar::VowelSignTallAa => "myanmar vowel sign tall aa",
            Myanmar::VowelSignAa => "myanmar vowel sign aa",
            Myanmar::VowelSignI => "myanmar vowel sign i",
            Myanmar::VowelSignIi => "myanmar vowel sign ii",
            Myanmar::VowelSignU => "myanmar vowel sign u",
            Myanmar::VowelSignUu => "myanmar vowel sign uu",
            Myanmar::VowelSignE => "myanmar vowel sign e",
            Myanmar::VowelSignAi => "myanmar vowel sign ai",
            Myanmar::VowelSignMonIi => "myanmar vowel sign mon ii",
            Myanmar::VowelSignMonO => "myanmar vowel sign mon o",
            Myanmar::VowelSignEAbove => "myanmar vowel sign e above",
            Myanmar::SignAnusvara => "myanmar sign anusvara",
            Myanmar::SignDotBelow => "myanmar sign dot below",
            Myanmar::SignVisarga => "myanmar sign visarga",
            Myanmar::SignVirama => "myanmar sign virama",
            Myanmar::SignAsat => "myanmar sign asat",
            Myanmar::ConsonantSignMedialYa => "myanmar consonant sign medial ya",
            Myanmar::ConsonantSignMedialRa => "myanmar consonant sign medial ra",
            Myanmar::ConsonantSignMedialWa => "myanmar consonant sign medial wa",
            Myanmar::ConsonantSignMedialHa => "myanmar consonant sign medial ha",
            Myanmar::LetterGreatSa => "myanmar letter great sa",
            Myanmar::DigitZero => "myanmar digit zero",
            Myanmar::DigitOne => "myanmar digit one",
            Myanmar::DigitTwo => "myanmar digit two",
            Myanmar::DigitThree => "myanmar digit three",
            Myanmar::DigitFour => "myanmar digit four",
            Myanmar::DigitFive => "myanmar digit five",
            Myanmar::DigitSix => "myanmar digit six",
            Myanmar::DigitSeven => "myanmar digit seven",
            Myanmar::DigitEight => "myanmar digit eight",
            Myanmar::DigitNine => "myanmar digit nine",
            Myanmar::SignLittleSection => "myanmar sign little section",
            Myanmar::SignSection => "myanmar sign section",
            Myanmar::SymbolLocative => "myanmar symbol locative",
            Myanmar::SymbolCompleted => "myanmar symbol completed",
            Myanmar::SymbolAforementioned => "myanmar symbol aforementioned",
            Myanmar::SymbolGenitive => "myanmar symbol genitive",
            Myanmar::LetterSha => "myanmar letter sha",
            Myanmar::LetterSsa => "myanmar letter ssa",
            Myanmar::LetterVocalicR => "myanmar letter vocalic r",
            Myanmar::LetterVocalicRr => "myanmar letter vocalic rr",
            Myanmar::LetterVocalicL => "myanmar letter vocalic l",
            Myanmar::LetterVocalicLl => "myanmar letter vocalic ll",
            Myanmar::VowelSignVocalicR => "myanmar vowel sign vocalic r",
            Myanmar::VowelSignVocalicRr => "myanmar vowel sign vocalic rr",
            Myanmar::VowelSignVocalicL => "myanmar vowel sign vocalic l",
            Myanmar::VowelSignVocalicLl => "myanmar vowel sign vocalic ll",
            Myanmar::LetterMonNga => "myanmar letter mon nga",
            Myanmar::LetterMonJha => "myanmar letter mon jha",
            Myanmar::LetterMonBba => "myanmar letter mon bba",
            Myanmar::LetterMonBbe => "myanmar letter mon bbe",
            Myanmar::ConsonantSignMonMedialNa => "myanmar consonant sign mon medial na",
            Myanmar::ConsonantSignMonMedialMa => "myanmar consonant sign mon medial ma",
            Myanmar::ConsonantSignMonMedialLa => "myanmar consonant sign mon medial la",
            Myanmar::LetterSgawKarenSha => "myanmar letter sgaw karen sha",
            Myanmar::VowelSignSgawKarenEu => "myanmar vowel sign sgaw karen eu",
            Myanmar::ToneMarkSgawKarenHathi => "myanmar tone mark sgaw karen hathi",
            Myanmar::ToneMarkSgawKarenKePho => "myanmar tone mark sgaw karen ke pho",
            Myanmar::LetterWesternPwoKarenTha => "myanmar letter western pwo karen tha",
            Myanmar::LetterWesternPwoKarenPwa => "myanmar letter western pwo karen pwa",
            Myanmar::VowelSignWesternPwoKarenEu => "myanmar vowel sign western pwo karen eu",
            Myanmar::VowelSignWesternPwoKarenUe => "myanmar vowel sign western pwo karen ue",
            Myanmar::SignWesternPwoKarenToneDash1 => "myanmar sign western pwo karen tone-1",
            Myanmar::SignWesternPwoKarenToneDash2 => "myanmar sign western pwo karen tone-2",
            Myanmar::SignWesternPwoKarenToneDash3 => "myanmar sign western pwo karen tone-3",
            Myanmar::SignWesternPwoKarenToneDash4 => "myanmar sign western pwo karen tone-4",
            Myanmar::SignWesternPwoKarenToneDash5 => "myanmar sign western pwo karen tone-5",
            Myanmar::LetterEasternPwoKarenNna => "myanmar letter eastern pwo karen nna",
            Myanmar::LetterEasternPwoKarenYwa => "myanmar letter eastern pwo karen ywa",
            Myanmar::LetterEasternPwoKarenGhwa => "myanmar letter eastern pwo karen ghwa",
            Myanmar::VowelSignGebaKarenI => "myanmar vowel sign geba karen i",
            Myanmar::VowelSignKayahOe => "myanmar vowel sign kayah oe",
            Myanmar::VowelSignKayahU => "myanmar vowel sign kayah u",
            Myanmar::VowelSignKayahEe => "myanmar vowel sign kayah ee",
            Myanmar::LetterShanKa => "myanmar letter shan ka",
            Myanmar::LetterShanKha => "myanmar letter shan kha",
            Myanmar::LetterShanGa => "myanmar letter shan ga",
            Myanmar::LetterShanCa => "myanmar letter shan ca",
            Myanmar::LetterShanZa => "myanmar letter shan za",
            Myanmar::LetterShanNya => "myanmar letter shan nya",
            Myanmar::LetterShanDa => "myanmar letter shan da",
            Myanmar::LetterShanNa => "myanmar letter shan na",
            Myanmar::LetterShanPha => "myanmar letter shan pha",
            Myanmar::LetterShanFa => "myanmar letter shan fa",
            Myanmar::LetterShanBa => "myanmar letter shan ba",
            Myanmar::LetterShanTha => "myanmar letter shan tha",
            Myanmar::LetterShanHa => "myanmar letter shan ha",
            Myanmar::ConsonantSignShanMedialWa => "myanmar consonant sign shan medial wa",
            Myanmar::VowelSignShanAa => "myanmar vowel sign shan aa",
            Myanmar::VowelSignShanE => "myanmar vowel sign shan e",
            Myanmar::VowelSignShanEAbove => "myanmar vowel sign shan e above",
            Myanmar::VowelSignShanFinalY => "myanmar vowel sign shan final y",
            Myanmar::SignShanToneDash2 => "myanmar sign shan tone-2",
            Myanmar::SignShanToneDash3 => "myanmar sign shan tone-3",
            Myanmar::SignShanToneDash5 => "myanmar sign shan tone-5",
            Myanmar::SignShanToneDash6 => "myanmar sign shan tone-6",
            Myanmar::SignShanCouncilToneDash2 => "myanmar sign shan council tone-2",
            Myanmar::SignShanCouncilToneDash3 => "myanmar sign shan council tone-3",
            Myanmar::SignShanCouncilEmphaticTone => "myanmar sign shan council emphatic tone",
            Myanmar::LetterRumaiPalaungFa => "myanmar letter rumai palaung fa",
            Myanmar::SignRumaiPalaungToneDash5 => "myanmar sign rumai palaung tone-5",
            Myanmar::ShanDigitZero => "myanmar shan digit zero",
            Myanmar::ShanDigitOne => "myanmar shan digit one",
            Myanmar::ShanDigitTwo => "myanmar shan digit two",
            Myanmar::ShanDigitThree => "myanmar shan digit three",
            Myanmar::ShanDigitFour => "myanmar shan digit four",
            Myanmar::ShanDigitFive => "myanmar shan digit five",
            Myanmar::ShanDigitSix => "myanmar shan digit six",
            Myanmar::ShanDigitSeven => "myanmar shan digit seven",
            Myanmar::ShanDigitEight => "myanmar shan digit eight",
            Myanmar::ShanDigitNine => "myanmar shan digit nine",
            Myanmar::SignKhamtiToneDash1 => "myanmar sign khamti tone-1",
            Myanmar::SignKhamtiToneDash3 => "myanmar sign khamti tone-3",
            Myanmar::VowelSignAitonA => "myanmar vowel sign aiton a",
            Myanmar::VowelSignAitonAi => "myanmar vowel sign aiton ai",
            Myanmar::SymbolShanOne => "myanmar symbol shan one",
        }
    }
}
