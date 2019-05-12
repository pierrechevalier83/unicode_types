/// \u{100} → \u{17f}\
///\
/// Ā ā Ă ă Ą ą Ć ć Ĉ ĉ Ċ ċ Č č Ď ď\
/// Đ đ Ē ē Ĕ ĕ Ė ė Ę ę Ě ě Ĝ ĝ Ğ ğ\
/// Ġ ġ Ģ ģ Ĥ ĥ Ħ ħ Ĩ ĩ Ī ī Ĭ ĭ Į į\
/// İ ı Ĳ ĳ Ĵ ĵ Ķ ķ ĸ Ĺ ĺ Ļ ļ Ľ ľ Ŀ\
/// ŀ Ł ł Ń ń Ņ ņ Ň ň ŉ Ŋ ŋ Ō ō Ŏ ŏ\
/// Ő ő Œ œ Ŕ ŕ Ŗ ŗ Ř ř Ś ś Ŝ ŝ Ş ş\
/// Š š Ţ ţ Ť ť Ŧ ŧ Ũ ũ Ū ū Ŭ ŭ Ů ů\
/// Ű ű Ų ų Ŵ ŵ Ŷ ŷ Ÿ Ź ź Ż ż Ž ž\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{100}: 'Ā'
    pub const LATIN_CAPITAL_LETTER_A_WITH_MACRON: char = 'Ā';
    /// \u{101}: 'ā'
    pub const LATIN_SMALL_LETTER_A_WITH_MACRON: char = 'ā';
    /// \u{102}: 'Ă'
    pub const LATIN_CAPITAL_LETTER_A_WITH_BREVE: char = 'Ă';
    /// \u{103}: 'ă'
    pub const LATIN_SMALL_LETTER_A_WITH_BREVE: char = 'ă';
    /// \u{104}: 'Ą'
    pub const LATIN_CAPITAL_LETTER_A_WITH_OGONEK: char = 'Ą';
    /// \u{105}: 'ą'
    pub const LATIN_SMALL_LETTER_A_WITH_OGONEK: char = 'ą';
    /// \u{106}: 'Ć'
    pub const LATIN_CAPITAL_LETTER_C_WITH_ACUTE: char = 'Ć';
    /// \u{107}: 'ć'
    pub const LATIN_SMALL_LETTER_C_WITH_ACUTE: char = 'ć';
    /// \u{108}: 'Ĉ'
    pub const LATIN_CAPITAL_LETTER_C_WITH_CIRCUMFLEX: char = 'Ĉ';
    /// \u{109}: 'ĉ'
    pub const LATIN_SMALL_LETTER_C_WITH_CIRCUMFLEX: char = 'ĉ';
    /// \u{10a}: 'Ċ'
    pub const LATIN_CAPITAL_LETTER_C_WITH_DOT_ABOVE: char = 'Ċ';
    /// \u{10b}: 'ċ'
    pub const LATIN_SMALL_LETTER_C_WITH_DOT_ABOVE: char = 'ċ';
    /// \u{10c}: 'Č'
    pub const LATIN_CAPITAL_LETTER_C_WITH_CARON: char = 'Č';
    /// \u{10d}: 'č'
    pub const LATIN_SMALL_LETTER_C_WITH_CARON: char = 'č';
    /// \u{10e}: 'Ď'
    pub const LATIN_CAPITAL_LETTER_D_WITH_CARON: char = 'Ď';
    /// \u{10f}: 'ď'
    pub const LATIN_SMALL_LETTER_D_WITH_CARON: char = 'ď';
    /// \u{110}: 'Đ'
    pub const LATIN_CAPITAL_LETTER_D_WITH_STROKE: char = 'Đ';
    /// \u{111}: 'đ'
    pub const LATIN_SMALL_LETTER_D_WITH_STROKE: char = 'đ';
    /// \u{112}: 'Ē'
    pub const LATIN_CAPITAL_LETTER_E_WITH_MACRON: char = 'Ē';
    /// \u{113}: 'ē'
    pub const LATIN_SMALL_LETTER_E_WITH_MACRON: char = 'ē';
    /// \u{114}: 'Ĕ'
    pub const LATIN_CAPITAL_LETTER_E_WITH_BREVE: char = 'Ĕ';
    /// \u{115}: 'ĕ'
    pub const LATIN_SMALL_LETTER_E_WITH_BREVE: char = 'ĕ';
    /// \u{116}: 'Ė'
    pub const LATIN_CAPITAL_LETTER_E_WITH_DOT_ABOVE: char = 'Ė';
    /// \u{117}: 'ė'
    pub const LATIN_SMALL_LETTER_E_WITH_DOT_ABOVE: char = 'ė';
    /// \u{118}: 'Ę'
    pub const LATIN_CAPITAL_LETTER_E_WITH_OGONEK: char = 'Ę';
    /// \u{119}: 'ę'
    pub const LATIN_SMALL_LETTER_E_WITH_OGONEK: char = 'ę';
    /// \u{11a}: 'Ě'
    pub const LATIN_CAPITAL_LETTER_E_WITH_CARON: char = 'Ě';
    /// \u{11b}: 'ě'
    pub const LATIN_SMALL_LETTER_E_WITH_CARON: char = 'ě';
    /// \u{11c}: 'Ĝ'
    pub const LATIN_CAPITAL_LETTER_G_WITH_CIRCUMFLEX: char = 'Ĝ';
    /// \u{11d}: 'ĝ'
    pub const LATIN_SMALL_LETTER_G_WITH_CIRCUMFLEX: char = 'ĝ';
    /// \u{11e}: 'Ğ'
    pub const LATIN_CAPITAL_LETTER_G_WITH_BREVE: char = 'Ğ';
    /// \u{11f}: 'ğ'
    pub const LATIN_SMALL_LETTER_G_WITH_BREVE: char = 'ğ';
    /// \u{120}: 'Ġ'
    pub const LATIN_CAPITAL_LETTER_G_WITH_DOT_ABOVE: char = 'Ġ';
    /// \u{121}: 'ġ'
    pub const LATIN_SMALL_LETTER_G_WITH_DOT_ABOVE: char = 'ġ';
    /// \u{122}: 'Ģ'
    pub const LATIN_CAPITAL_LETTER_G_WITH_CEDILLA: char = 'Ģ';
    /// \u{123}: 'ģ'
    pub const LATIN_SMALL_LETTER_G_WITH_CEDILLA: char = 'ģ';
    /// \u{124}: 'Ĥ'
    pub const LATIN_CAPITAL_LETTER_H_WITH_CIRCUMFLEX: char = 'Ĥ';
    /// \u{125}: 'ĥ'
    pub const LATIN_SMALL_LETTER_H_WITH_CIRCUMFLEX: char = 'ĥ';
    /// \u{126}: 'Ħ'
    pub const LATIN_CAPITAL_LETTER_H_WITH_STROKE: char = 'Ħ';
    /// \u{127}: 'ħ'
    pub const LATIN_SMALL_LETTER_H_WITH_STROKE: char = 'ħ';
    /// \u{128}: 'Ĩ'
    pub const LATIN_CAPITAL_LETTER_I_WITH_TILDE: char = 'Ĩ';
    /// \u{129}: 'ĩ'
    pub const LATIN_SMALL_LETTER_I_WITH_TILDE: char = 'ĩ';
    /// \u{12a}: 'Ī'
    pub const LATIN_CAPITAL_LETTER_I_WITH_MACRON: char = 'Ī';
    /// \u{12b}: 'ī'
    pub const LATIN_SMALL_LETTER_I_WITH_MACRON: char = 'ī';
    /// \u{12c}: 'Ĭ'
    pub const LATIN_CAPITAL_LETTER_I_WITH_BREVE: char = 'Ĭ';
    /// \u{12d}: 'ĭ'
    pub const LATIN_SMALL_LETTER_I_WITH_BREVE: char = 'ĭ';
    /// \u{12e}: 'Į'
    pub const LATIN_CAPITAL_LETTER_I_WITH_OGONEK: char = 'Į';
    /// \u{12f}: 'į'
    pub const LATIN_SMALL_LETTER_I_WITH_OGONEK: char = 'į';
    /// \u{130}: 'İ'
    pub const LATIN_CAPITAL_LETTER_I_WITH_DOT_ABOVE: char = 'İ';
    /// \u{131}: 'ı'
    pub const LATIN_SMALL_LETTER_DOTLESS_I: char = 'ı';
    /// \u{132}: 'Ĳ'
    pub const LATIN_CAPITAL_LIGATURE_IJ: char = 'Ĳ';
    /// \u{133}: 'ĳ'
    pub const LATIN_SMALL_LIGATURE_IJ: char = 'ĳ';
    /// \u{134}: 'Ĵ'
    pub const LATIN_CAPITAL_LETTER_J_WITH_CIRCUMFLEX: char = 'Ĵ';
    /// \u{135}: 'ĵ'
    pub const LATIN_SMALL_LETTER_J_WITH_CIRCUMFLEX: char = 'ĵ';
    /// \u{136}: 'Ķ'
    pub const LATIN_CAPITAL_LETTER_K_WITH_CEDILLA: char = 'Ķ';
    /// \u{137}: 'ķ'
    pub const LATIN_SMALL_LETTER_K_WITH_CEDILLA: char = 'ķ';
    /// \u{138}: 'ĸ'
    pub const LATIN_SMALL_LETTER_KRA: char = 'ĸ';
    /// \u{139}: 'Ĺ'
    pub const LATIN_CAPITAL_LETTER_L_WITH_ACUTE: char = 'Ĺ';
    /// \u{13a}: 'ĺ'
    pub const LATIN_SMALL_LETTER_L_WITH_ACUTE: char = 'ĺ';
    /// \u{13b}: 'Ļ'
    pub const LATIN_CAPITAL_LETTER_L_WITH_CEDILLA: char = 'Ļ';
    /// \u{13c}: 'ļ'
    pub const LATIN_SMALL_LETTER_L_WITH_CEDILLA: char = 'ļ';
    /// \u{13d}: 'Ľ'
    pub const LATIN_CAPITAL_LETTER_L_WITH_CARON: char = 'Ľ';
    /// \u{13e}: 'ľ'
    pub const LATIN_SMALL_LETTER_L_WITH_CARON: char = 'ľ';
    /// \u{13f}: 'Ŀ'
    pub const LATIN_CAPITAL_LETTER_L_WITH_MIDDLE_DOT: char = 'Ŀ';
    /// \u{140}: 'ŀ'
    pub const LATIN_SMALL_LETTER_L_WITH_MIDDLE_DOT: char = 'ŀ';
    /// \u{141}: 'Ł'
    pub const LATIN_CAPITAL_LETTER_L_WITH_STROKE: char = 'Ł';
    /// \u{142}: 'ł'
    pub const LATIN_SMALL_LETTER_L_WITH_STROKE: char = 'ł';
    /// \u{143}: 'Ń'
    pub const LATIN_CAPITAL_LETTER_N_WITH_ACUTE: char = 'Ń';
    /// \u{144}: 'ń'
    pub const LATIN_SMALL_LETTER_N_WITH_ACUTE: char = 'ń';
    /// \u{145}: 'Ņ'
    pub const LATIN_CAPITAL_LETTER_N_WITH_CEDILLA: char = 'Ņ';
    /// \u{146}: 'ņ'
    pub const LATIN_SMALL_LETTER_N_WITH_CEDILLA: char = 'ņ';
    /// \u{147}: 'Ň'
    pub const LATIN_CAPITAL_LETTER_N_WITH_CARON: char = 'Ň';
    /// \u{148}: 'ň'
    pub const LATIN_SMALL_LETTER_N_WITH_CARON: char = 'ň';
    /// \u{149}: 'ŉ'
    pub const LATIN_SMALL_LETTER_N_PRECEDED_BY_APOSTROPHE: char = 'ŉ';
    /// \u{14a}: 'Ŋ'
    pub const LATIN_CAPITAL_LETTER_ENG: char = 'Ŋ';
    /// \u{14b}: 'ŋ'
    pub const LATIN_SMALL_LETTER_ENG: char = 'ŋ';
    /// \u{14c}: 'Ō'
    pub const LATIN_CAPITAL_LETTER_O_WITH_MACRON: char = 'Ō';
    /// \u{14d}: 'ō'
    pub const LATIN_SMALL_LETTER_O_WITH_MACRON: char = 'ō';
    /// \u{14e}: 'Ŏ'
    pub const LATIN_CAPITAL_LETTER_O_WITH_BREVE: char = 'Ŏ';
    /// \u{14f}: 'ŏ'
    pub const LATIN_SMALL_LETTER_O_WITH_BREVE: char = 'ŏ';
    /// \u{150}: 'Ő'
    pub const LATIN_CAPITAL_LETTER_O_WITH_DOUBLE_ACUTE: char = 'Ő';
    /// \u{151}: 'ő'
    pub const LATIN_SMALL_LETTER_O_WITH_DOUBLE_ACUTE: char = 'ő';
    /// \u{152}: 'Œ'
    pub const LATIN_CAPITAL_LIGATURE_OE: char = 'Œ';
    /// \u{153}: 'œ'
    pub const LATIN_SMALL_LIGATURE_OE: char = 'œ';
    /// \u{154}: 'Ŕ'
    pub const LATIN_CAPITAL_LETTER_R_WITH_ACUTE: char = 'Ŕ';
    /// \u{155}: 'ŕ'
    pub const LATIN_SMALL_LETTER_R_WITH_ACUTE: char = 'ŕ';
    /// \u{156}: 'Ŗ'
    pub const LATIN_CAPITAL_LETTER_R_WITH_CEDILLA: char = 'Ŗ';
    /// \u{157}: 'ŗ'
    pub const LATIN_SMALL_LETTER_R_WITH_CEDILLA: char = 'ŗ';
    /// \u{158}: 'Ř'
    pub const LATIN_CAPITAL_LETTER_R_WITH_CARON: char = 'Ř';
    /// \u{159}: 'ř'
    pub const LATIN_SMALL_LETTER_R_WITH_CARON: char = 'ř';
    /// \u{15a}: 'Ś'
    pub const LATIN_CAPITAL_LETTER_S_WITH_ACUTE: char = 'Ś';
    /// \u{15b}: 'ś'
    pub const LATIN_SMALL_LETTER_S_WITH_ACUTE: char = 'ś';
    /// \u{15c}: 'Ŝ'
    pub const LATIN_CAPITAL_LETTER_S_WITH_CIRCUMFLEX: char = 'Ŝ';
    /// \u{15d}: 'ŝ'
    pub const LATIN_SMALL_LETTER_S_WITH_CIRCUMFLEX: char = 'ŝ';
    /// \u{15e}: 'Ş'
    pub const LATIN_CAPITAL_LETTER_S_WITH_CEDILLA: char = 'Ş';
    /// \u{15f}: 'ş'
    pub const LATIN_SMALL_LETTER_S_WITH_CEDILLA: char = 'ş';
    /// \u{160}: 'Š'
    pub const LATIN_CAPITAL_LETTER_S_WITH_CARON: char = 'Š';
    /// \u{161}: 'š'
    pub const LATIN_SMALL_LETTER_S_WITH_CARON: char = 'š';
    /// \u{162}: 'Ţ'
    pub const LATIN_CAPITAL_LETTER_T_WITH_CEDILLA: char = 'Ţ';
    /// \u{163}: 'ţ'
    pub const LATIN_SMALL_LETTER_T_WITH_CEDILLA: char = 'ţ';
    /// \u{164}: 'Ť'
    pub const LATIN_CAPITAL_LETTER_T_WITH_CARON: char = 'Ť';
    /// \u{165}: 'ť'
    pub const LATIN_SMALL_LETTER_T_WITH_CARON: char = 'ť';
    /// \u{166}: 'Ŧ'
    pub const LATIN_CAPITAL_LETTER_T_WITH_STROKE: char = 'Ŧ';
    /// \u{167}: 'ŧ'
    pub const LATIN_SMALL_LETTER_T_WITH_STROKE: char = 'ŧ';
    /// \u{168}: 'Ũ'
    pub const LATIN_CAPITAL_LETTER_U_WITH_TILDE: char = 'Ũ';
    /// \u{169}: 'ũ'
    pub const LATIN_SMALL_LETTER_U_WITH_TILDE: char = 'ũ';
    /// \u{16a}: 'Ū'
    pub const LATIN_CAPITAL_LETTER_U_WITH_MACRON: char = 'Ū';
    /// \u{16b}: 'ū'
    pub const LATIN_SMALL_LETTER_U_WITH_MACRON: char = 'ū';
    /// \u{16c}: 'Ŭ'
    pub const LATIN_CAPITAL_LETTER_U_WITH_BREVE: char = 'Ŭ';
    /// \u{16d}: 'ŭ'
    pub const LATIN_SMALL_LETTER_U_WITH_BREVE: char = 'ŭ';
    /// \u{16e}: 'Ů'
    pub const LATIN_CAPITAL_LETTER_U_WITH_RING_ABOVE: char = 'Ů';
    /// \u{16f}: 'ů'
    pub const LATIN_SMALL_LETTER_U_WITH_RING_ABOVE: char = 'ů';
    /// \u{170}: 'Ű'
    pub const LATIN_CAPITAL_LETTER_U_WITH_DOUBLE_ACUTE: char = 'Ű';
    /// \u{171}: 'ű'
    pub const LATIN_SMALL_LETTER_U_WITH_DOUBLE_ACUTE: char = 'ű';
    /// \u{172}: 'Ų'
    pub const LATIN_CAPITAL_LETTER_U_WITH_OGONEK: char = 'Ų';
    /// \u{173}: 'ų'
    pub const LATIN_SMALL_LETTER_U_WITH_OGONEK: char = 'ų';
    /// \u{174}: 'Ŵ'
    pub const LATIN_CAPITAL_LETTER_W_WITH_CIRCUMFLEX: char = 'Ŵ';
    /// \u{175}: 'ŵ'
    pub const LATIN_SMALL_LETTER_W_WITH_CIRCUMFLEX: char = 'ŵ';
    /// \u{176}: 'Ŷ'
    pub const LATIN_CAPITAL_LETTER_Y_WITH_CIRCUMFLEX: char = 'Ŷ';
    /// \u{177}: 'ŷ'
    pub const LATIN_SMALL_LETTER_Y_WITH_CIRCUMFLEX: char = 'ŷ';
    /// \u{178}: 'Ÿ'
    pub const LATIN_CAPITAL_LETTER_Y_WITH_DIAERESIS: char = 'Ÿ';
    /// \u{179}: 'Ź'
    pub const LATIN_CAPITAL_LETTER_Z_WITH_ACUTE: char = 'Ź';
    /// \u{17a}: 'ź'
    pub const LATIN_SMALL_LETTER_Z_WITH_ACUTE: char = 'ź';
    /// \u{17b}: 'Ż'
    pub const LATIN_CAPITAL_LETTER_Z_WITH_DOT_ABOVE: char = 'Ż';
    /// \u{17c}: 'ż'
    pub const LATIN_SMALL_LETTER_Z_WITH_DOT_ABOVE: char = 'ż';
    /// \u{17d}: 'Ž'
    pub const LATIN_CAPITAL_LETTER_Z_WITH_CARON: char = 'Ž';
    /// \u{17e}: 'ž'
    pub const LATIN_SMALL_LETTER_Z_WITH_CARON: char = 'ž';
}

/// An enum to represent all characters in the LatinExtendedA block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum LatinExtendedA {
    /// \u{100}: 'Ā'
    LatinCapitalLetterAWithMacron,
    /// \u{101}: 'ā'
    LatinSmallLetterAWithMacron,
    /// \u{102}: 'Ă'
    LatinCapitalLetterAWithBreve,
    /// \u{103}: 'ă'
    LatinSmallLetterAWithBreve,
    /// \u{104}: 'Ą'
    LatinCapitalLetterAWithOgonek,
    /// \u{105}: 'ą'
    LatinSmallLetterAWithOgonek,
    /// \u{106}: 'Ć'
    LatinCapitalLetterCWithAcute,
    /// \u{107}: 'ć'
    LatinSmallLetterCWithAcute,
    /// \u{108}: 'Ĉ'
    LatinCapitalLetterCWithCircumflex,
    /// \u{109}: 'ĉ'
    LatinSmallLetterCWithCircumflex,
    /// \u{10a}: 'Ċ'
    LatinCapitalLetterCWithDotAbove,
    /// \u{10b}: 'ċ'
    LatinSmallLetterCWithDotAbove,
    /// \u{10c}: 'Č'
    LatinCapitalLetterCWithCaron,
    /// \u{10d}: 'č'
    LatinSmallLetterCWithCaron,
    /// \u{10e}: 'Ď'
    LatinCapitalLetterDWithCaron,
    /// \u{10f}: 'ď'
    LatinSmallLetterDWithCaron,
    /// \u{110}: 'Đ'
    LatinCapitalLetterDWithStroke,
    /// \u{111}: 'đ'
    LatinSmallLetterDWithStroke,
    /// \u{112}: 'Ē'
    LatinCapitalLetterEWithMacron,
    /// \u{113}: 'ē'
    LatinSmallLetterEWithMacron,
    /// \u{114}: 'Ĕ'
    LatinCapitalLetterEWithBreve,
    /// \u{115}: 'ĕ'
    LatinSmallLetterEWithBreve,
    /// \u{116}: 'Ė'
    LatinCapitalLetterEWithDotAbove,
    /// \u{117}: 'ė'
    LatinSmallLetterEWithDotAbove,
    /// \u{118}: 'Ę'
    LatinCapitalLetterEWithOgonek,
    /// \u{119}: 'ę'
    LatinSmallLetterEWithOgonek,
    /// \u{11a}: 'Ě'
    LatinCapitalLetterEWithCaron,
    /// \u{11b}: 'ě'
    LatinSmallLetterEWithCaron,
    /// \u{11c}: 'Ĝ'
    LatinCapitalLetterGWithCircumflex,
    /// \u{11d}: 'ĝ'
    LatinSmallLetterGWithCircumflex,
    /// \u{11e}: 'Ğ'
    LatinCapitalLetterGWithBreve,
    /// \u{11f}: 'ğ'
    LatinSmallLetterGWithBreve,
    /// \u{120}: 'Ġ'
    LatinCapitalLetterGWithDotAbove,
    /// \u{121}: 'ġ'
    LatinSmallLetterGWithDotAbove,
    /// \u{122}: 'Ģ'
    LatinCapitalLetterGWithCedilla,
    /// \u{123}: 'ģ'
    LatinSmallLetterGWithCedilla,
    /// \u{124}: 'Ĥ'
    LatinCapitalLetterHWithCircumflex,
    /// \u{125}: 'ĥ'
    LatinSmallLetterHWithCircumflex,
    /// \u{126}: 'Ħ'
    LatinCapitalLetterHWithStroke,
    /// \u{127}: 'ħ'
    LatinSmallLetterHWithStroke,
    /// \u{128}: 'Ĩ'
    LatinCapitalLetterIWithTilde,
    /// \u{129}: 'ĩ'
    LatinSmallLetterIWithTilde,
    /// \u{12a}: 'Ī'
    LatinCapitalLetterIWithMacron,
    /// \u{12b}: 'ī'
    LatinSmallLetterIWithMacron,
    /// \u{12c}: 'Ĭ'
    LatinCapitalLetterIWithBreve,
    /// \u{12d}: 'ĭ'
    LatinSmallLetterIWithBreve,
    /// \u{12e}: 'Į'
    LatinCapitalLetterIWithOgonek,
    /// \u{12f}: 'į'
    LatinSmallLetterIWithOgonek,
    /// \u{130}: 'İ'
    LatinCapitalLetterIWithDotAbove,
    /// \u{131}: 'ı'
    LatinSmallLetterDotlessI,
    /// \u{132}: 'Ĳ'
    LatinCapitalLigatureIj,
    /// \u{133}: 'ĳ'
    LatinSmallLigatureIj,
    /// \u{134}: 'Ĵ'
    LatinCapitalLetterJWithCircumflex,
    /// \u{135}: 'ĵ'
    LatinSmallLetterJWithCircumflex,
    /// \u{136}: 'Ķ'
    LatinCapitalLetterKWithCedilla,
    /// \u{137}: 'ķ'
    LatinSmallLetterKWithCedilla,
    /// \u{138}: 'ĸ'
    LatinSmallLetterKra,
    /// \u{139}: 'Ĺ'
    LatinCapitalLetterLWithAcute,
    /// \u{13a}: 'ĺ'
    LatinSmallLetterLWithAcute,
    /// \u{13b}: 'Ļ'
    LatinCapitalLetterLWithCedilla,
    /// \u{13c}: 'ļ'
    LatinSmallLetterLWithCedilla,
    /// \u{13d}: 'Ľ'
    LatinCapitalLetterLWithCaron,
    /// \u{13e}: 'ľ'
    LatinSmallLetterLWithCaron,
    /// \u{13f}: 'Ŀ'
    LatinCapitalLetterLWithMiddleDot,
    /// \u{140}: 'ŀ'
    LatinSmallLetterLWithMiddleDot,
    /// \u{141}: 'Ł'
    LatinCapitalLetterLWithStroke,
    /// \u{142}: 'ł'
    LatinSmallLetterLWithStroke,
    /// \u{143}: 'Ń'
    LatinCapitalLetterNWithAcute,
    /// \u{144}: 'ń'
    LatinSmallLetterNWithAcute,
    /// \u{145}: 'Ņ'
    LatinCapitalLetterNWithCedilla,
    /// \u{146}: 'ņ'
    LatinSmallLetterNWithCedilla,
    /// \u{147}: 'Ň'
    LatinCapitalLetterNWithCaron,
    /// \u{148}: 'ň'
    LatinSmallLetterNWithCaron,
    /// \u{149}: 'ŉ'
    LatinSmallLetterNPrecededByApostrophe,
    /// \u{14a}: 'Ŋ'
    LatinCapitalLetterEng,
    /// \u{14b}: 'ŋ'
    LatinSmallLetterEng,
    /// \u{14c}: 'Ō'
    LatinCapitalLetterOWithMacron,
    /// \u{14d}: 'ō'
    LatinSmallLetterOWithMacron,
    /// \u{14e}: 'Ŏ'
    LatinCapitalLetterOWithBreve,
    /// \u{14f}: 'ŏ'
    LatinSmallLetterOWithBreve,
    /// \u{150}: 'Ő'
    LatinCapitalLetterOWithDoubleAcute,
    /// \u{151}: 'ő'
    LatinSmallLetterOWithDoubleAcute,
    /// \u{152}: 'Œ'
    LatinCapitalLigatureOe,
    /// \u{153}: 'œ'
    LatinSmallLigatureOe,
    /// \u{154}: 'Ŕ'
    LatinCapitalLetterRWithAcute,
    /// \u{155}: 'ŕ'
    LatinSmallLetterRWithAcute,
    /// \u{156}: 'Ŗ'
    LatinCapitalLetterRWithCedilla,
    /// \u{157}: 'ŗ'
    LatinSmallLetterRWithCedilla,
    /// \u{158}: 'Ř'
    LatinCapitalLetterRWithCaron,
    /// \u{159}: 'ř'
    LatinSmallLetterRWithCaron,
    /// \u{15a}: 'Ś'
    LatinCapitalLetterSWithAcute,
    /// \u{15b}: 'ś'
    LatinSmallLetterSWithAcute,
    /// \u{15c}: 'Ŝ'
    LatinCapitalLetterSWithCircumflex,
    /// \u{15d}: 'ŝ'
    LatinSmallLetterSWithCircumflex,
    /// \u{15e}: 'Ş'
    LatinCapitalLetterSWithCedilla,
    /// \u{15f}: 'ş'
    LatinSmallLetterSWithCedilla,
    /// \u{160}: 'Š'
    LatinCapitalLetterSWithCaron,
    /// \u{161}: 'š'
    LatinSmallLetterSWithCaron,
    /// \u{162}: 'Ţ'
    LatinCapitalLetterTWithCedilla,
    /// \u{163}: 'ţ'
    LatinSmallLetterTWithCedilla,
    /// \u{164}: 'Ť'
    LatinCapitalLetterTWithCaron,
    /// \u{165}: 'ť'
    LatinSmallLetterTWithCaron,
    /// \u{166}: 'Ŧ'
    LatinCapitalLetterTWithStroke,
    /// \u{167}: 'ŧ'
    LatinSmallLetterTWithStroke,
    /// \u{168}: 'Ũ'
    LatinCapitalLetterUWithTilde,
    /// \u{169}: 'ũ'
    LatinSmallLetterUWithTilde,
    /// \u{16a}: 'Ū'
    LatinCapitalLetterUWithMacron,
    /// \u{16b}: 'ū'
    LatinSmallLetterUWithMacron,
    /// \u{16c}: 'Ŭ'
    LatinCapitalLetterUWithBreve,
    /// \u{16d}: 'ŭ'
    LatinSmallLetterUWithBreve,
    /// \u{16e}: 'Ů'
    LatinCapitalLetterUWithRingAbove,
    /// \u{16f}: 'ů'
    LatinSmallLetterUWithRingAbove,
    /// \u{170}: 'Ű'
    LatinCapitalLetterUWithDoubleAcute,
    /// \u{171}: 'ű'
    LatinSmallLetterUWithDoubleAcute,
    /// \u{172}: 'Ų'
    LatinCapitalLetterUWithOgonek,
    /// \u{173}: 'ų'
    LatinSmallLetterUWithOgonek,
    /// \u{174}: 'Ŵ'
    LatinCapitalLetterWWithCircumflex,
    /// \u{175}: 'ŵ'
    LatinSmallLetterWWithCircumflex,
    /// \u{176}: 'Ŷ'
    LatinCapitalLetterYWithCircumflex,
    /// \u{177}: 'ŷ'
    LatinSmallLetterYWithCircumflex,
    /// \u{178}: 'Ÿ'
    LatinCapitalLetterYWithDiaeresis,
    /// \u{179}: 'Ź'
    LatinCapitalLetterZWithAcute,
    /// \u{17a}: 'ź'
    LatinSmallLetterZWithAcute,
    /// \u{17b}: 'Ż'
    LatinCapitalLetterZWithDotAbove,
    /// \u{17c}: 'ż'
    LatinSmallLetterZWithDotAbove,
    /// \u{17d}: 'Ž'
    LatinCapitalLetterZWithCaron,
    /// \u{17e}: 'ž'
    LatinSmallLetterZWithCaron,
}

impl Into<char> for LatinExtendedA {
    fn into(self) -> char {
        use constants::*;
        match self {
            LatinExtendedA::LatinCapitalLetterAWithMacron => LATIN_CAPITAL_LETTER_A_WITH_MACRON,
            LatinExtendedA::LatinSmallLetterAWithMacron => LATIN_SMALL_LETTER_A_WITH_MACRON,
            LatinExtendedA::LatinCapitalLetterAWithBreve => LATIN_CAPITAL_LETTER_A_WITH_BREVE,
            LatinExtendedA::LatinSmallLetterAWithBreve => LATIN_SMALL_LETTER_A_WITH_BREVE,
            LatinExtendedA::LatinCapitalLetterAWithOgonek => LATIN_CAPITAL_LETTER_A_WITH_OGONEK,
            LatinExtendedA::LatinSmallLetterAWithOgonek => LATIN_SMALL_LETTER_A_WITH_OGONEK,
            LatinExtendedA::LatinCapitalLetterCWithAcute => LATIN_CAPITAL_LETTER_C_WITH_ACUTE,
            LatinExtendedA::LatinSmallLetterCWithAcute => LATIN_SMALL_LETTER_C_WITH_ACUTE,
            LatinExtendedA::LatinCapitalLetterCWithCircumflex => LATIN_CAPITAL_LETTER_C_WITH_CIRCUMFLEX,
            LatinExtendedA::LatinSmallLetterCWithCircumflex => LATIN_SMALL_LETTER_C_WITH_CIRCUMFLEX,
            LatinExtendedA::LatinCapitalLetterCWithDotAbove => LATIN_CAPITAL_LETTER_C_WITH_DOT_ABOVE,
            LatinExtendedA::LatinSmallLetterCWithDotAbove => LATIN_SMALL_LETTER_C_WITH_DOT_ABOVE,
            LatinExtendedA::LatinCapitalLetterCWithCaron => LATIN_CAPITAL_LETTER_C_WITH_CARON,
            LatinExtendedA::LatinSmallLetterCWithCaron => LATIN_SMALL_LETTER_C_WITH_CARON,
            LatinExtendedA::LatinCapitalLetterDWithCaron => LATIN_CAPITAL_LETTER_D_WITH_CARON,
            LatinExtendedA::LatinSmallLetterDWithCaron => LATIN_SMALL_LETTER_D_WITH_CARON,
            LatinExtendedA::LatinCapitalLetterDWithStroke => LATIN_CAPITAL_LETTER_D_WITH_STROKE,
            LatinExtendedA::LatinSmallLetterDWithStroke => LATIN_SMALL_LETTER_D_WITH_STROKE,
            LatinExtendedA::LatinCapitalLetterEWithMacron => LATIN_CAPITAL_LETTER_E_WITH_MACRON,
            LatinExtendedA::LatinSmallLetterEWithMacron => LATIN_SMALL_LETTER_E_WITH_MACRON,
            LatinExtendedA::LatinCapitalLetterEWithBreve => LATIN_CAPITAL_LETTER_E_WITH_BREVE,
            LatinExtendedA::LatinSmallLetterEWithBreve => LATIN_SMALL_LETTER_E_WITH_BREVE,
            LatinExtendedA::LatinCapitalLetterEWithDotAbove => LATIN_CAPITAL_LETTER_E_WITH_DOT_ABOVE,
            LatinExtendedA::LatinSmallLetterEWithDotAbove => LATIN_SMALL_LETTER_E_WITH_DOT_ABOVE,
            LatinExtendedA::LatinCapitalLetterEWithOgonek => LATIN_CAPITAL_LETTER_E_WITH_OGONEK,
            LatinExtendedA::LatinSmallLetterEWithOgonek => LATIN_SMALL_LETTER_E_WITH_OGONEK,
            LatinExtendedA::LatinCapitalLetterEWithCaron => LATIN_CAPITAL_LETTER_E_WITH_CARON,
            LatinExtendedA::LatinSmallLetterEWithCaron => LATIN_SMALL_LETTER_E_WITH_CARON,
            LatinExtendedA::LatinCapitalLetterGWithCircumflex => LATIN_CAPITAL_LETTER_G_WITH_CIRCUMFLEX,
            LatinExtendedA::LatinSmallLetterGWithCircumflex => LATIN_SMALL_LETTER_G_WITH_CIRCUMFLEX,
            LatinExtendedA::LatinCapitalLetterGWithBreve => LATIN_CAPITAL_LETTER_G_WITH_BREVE,
            LatinExtendedA::LatinSmallLetterGWithBreve => LATIN_SMALL_LETTER_G_WITH_BREVE,
            LatinExtendedA::LatinCapitalLetterGWithDotAbove => LATIN_CAPITAL_LETTER_G_WITH_DOT_ABOVE,
            LatinExtendedA::LatinSmallLetterGWithDotAbove => LATIN_SMALL_LETTER_G_WITH_DOT_ABOVE,
            LatinExtendedA::LatinCapitalLetterGWithCedilla => LATIN_CAPITAL_LETTER_G_WITH_CEDILLA,
            LatinExtendedA::LatinSmallLetterGWithCedilla => LATIN_SMALL_LETTER_G_WITH_CEDILLA,
            LatinExtendedA::LatinCapitalLetterHWithCircumflex => LATIN_CAPITAL_LETTER_H_WITH_CIRCUMFLEX,
            LatinExtendedA::LatinSmallLetterHWithCircumflex => LATIN_SMALL_LETTER_H_WITH_CIRCUMFLEX,
            LatinExtendedA::LatinCapitalLetterHWithStroke => LATIN_CAPITAL_LETTER_H_WITH_STROKE,
            LatinExtendedA::LatinSmallLetterHWithStroke => LATIN_SMALL_LETTER_H_WITH_STROKE,
            LatinExtendedA::LatinCapitalLetterIWithTilde => LATIN_CAPITAL_LETTER_I_WITH_TILDE,
            LatinExtendedA::LatinSmallLetterIWithTilde => LATIN_SMALL_LETTER_I_WITH_TILDE,
            LatinExtendedA::LatinCapitalLetterIWithMacron => LATIN_CAPITAL_LETTER_I_WITH_MACRON,
            LatinExtendedA::LatinSmallLetterIWithMacron => LATIN_SMALL_LETTER_I_WITH_MACRON,
            LatinExtendedA::LatinCapitalLetterIWithBreve => LATIN_CAPITAL_LETTER_I_WITH_BREVE,
            LatinExtendedA::LatinSmallLetterIWithBreve => LATIN_SMALL_LETTER_I_WITH_BREVE,
            LatinExtendedA::LatinCapitalLetterIWithOgonek => LATIN_CAPITAL_LETTER_I_WITH_OGONEK,
            LatinExtendedA::LatinSmallLetterIWithOgonek => LATIN_SMALL_LETTER_I_WITH_OGONEK,
            LatinExtendedA::LatinCapitalLetterIWithDotAbove => LATIN_CAPITAL_LETTER_I_WITH_DOT_ABOVE,
            LatinExtendedA::LatinSmallLetterDotlessI => LATIN_SMALL_LETTER_DOTLESS_I,
            LatinExtendedA::LatinCapitalLigatureIj => LATIN_CAPITAL_LIGATURE_IJ,
            LatinExtendedA::LatinSmallLigatureIj => LATIN_SMALL_LIGATURE_IJ,
            LatinExtendedA::LatinCapitalLetterJWithCircumflex => LATIN_CAPITAL_LETTER_J_WITH_CIRCUMFLEX,
            LatinExtendedA::LatinSmallLetterJWithCircumflex => LATIN_SMALL_LETTER_J_WITH_CIRCUMFLEX,
            LatinExtendedA::LatinCapitalLetterKWithCedilla => LATIN_CAPITAL_LETTER_K_WITH_CEDILLA,
            LatinExtendedA::LatinSmallLetterKWithCedilla => LATIN_SMALL_LETTER_K_WITH_CEDILLA,
            LatinExtendedA::LatinSmallLetterKra => LATIN_SMALL_LETTER_KRA,
            LatinExtendedA::LatinCapitalLetterLWithAcute => LATIN_CAPITAL_LETTER_L_WITH_ACUTE,
            LatinExtendedA::LatinSmallLetterLWithAcute => LATIN_SMALL_LETTER_L_WITH_ACUTE,
            LatinExtendedA::LatinCapitalLetterLWithCedilla => LATIN_CAPITAL_LETTER_L_WITH_CEDILLA,
            LatinExtendedA::LatinSmallLetterLWithCedilla => LATIN_SMALL_LETTER_L_WITH_CEDILLA,
            LatinExtendedA::LatinCapitalLetterLWithCaron => LATIN_CAPITAL_LETTER_L_WITH_CARON,
            LatinExtendedA::LatinSmallLetterLWithCaron => LATIN_SMALL_LETTER_L_WITH_CARON,
            LatinExtendedA::LatinCapitalLetterLWithMiddleDot => LATIN_CAPITAL_LETTER_L_WITH_MIDDLE_DOT,
            LatinExtendedA::LatinSmallLetterLWithMiddleDot => LATIN_SMALL_LETTER_L_WITH_MIDDLE_DOT,
            LatinExtendedA::LatinCapitalLetterLWithStroke => LATIN_CAPITAL_LETTER_L_WITH_STROKE,
            LatinExtendedA::LatinSmallLetterLWithStroke => LATIN_SMALL_LETTER_L_WITH_STROKE,
            LatinExtendedA::LatinCapitalLetterNWithAcute => LATIN_CAPITAL_LETTER_N_WITH_ACUTE,
            LatinExtendedA::LatinSmallLetterNWithAcute => LATIN_SMALL_LETTER_N_WITH_ACUTE,
            LatinExtendedA::LatinCapitalLetterNWithCedilla => LATIN_CAPITAL_LETTER_N_WITH_CEDILLA,
            LatinExtendedA::LatinSmallLetterNWithCedilla => LATIN_SMALL_LETTER_N_WITH_CEDILLA,
            LatinExtendedA::LatinCapitalLetterNWithCaron => LATIN_CAPITAL_LETTER_N_WITH_CARON,
            LatinExtendedA::LatinSmallLetterNWithCaron => LATIN_SMALL_LETTER_N_WITH_CARON,
            LatinExtendedA::LatinSmallLetterNPrecededByApostrophe => LATIN_SMALL_LETTER_N_PRECEDED_BY_APOSTROPHE,
            LatinExtendedA::LatinCapitalLetterEng => LATIN_CAPITAL_LETTER_ENG,
            LatinExtendedA::LatinSmallLetterEng => LATIN_SMALL_LETTER_ENG,
            LatinExtendedA::LatinCapitalLetterOWithMacron => LATIN_CAPITAL_LETTER_O_WITH_MACRON,
            LatinExtendedA::LatinSmallLetterOWithMacron => LATIN_SMALL_LETTER_O_WITH_MACRON,
            LatinExtendedA::LatinCapitalLetterOWithBreve => LATIN_CAPITAL_LETTER_O_WITH_BREVE,
            LatinExtendedA::LatinSmallLetterOWithBreve => LATIN_SMALL_LETTER_O_WITH_BREVE,
            LatinExtendedA::LatinCapitalLetterOWithDoubleAcute => LATIN_CAPITAL_LETTER_O_WITH_DOUBLE_ACUTE,
            LatinExtendedA::LatinSmallLetterOWithDoubleAcute => LATIN_SMALL_LETTER_O_WITH_DOUBLE_ACUTE,
            LatinExtendedA::LatinCapitalLigatureOe => LATIN_CAPITAL_LIGATURE_OE,
            LatinExtendedA::LatinSmallLigatureOe => LATIN_SMALL_LIGATURE_OE,
            LatinExtendedA::LatinCapitalLetterRWithAcute => LATIN_CAPITAL_LETTER_R_WITH_ACUTE,
            LatinExtendedA::LatinSmallLetterRWithAcute => LATIN_SMALL_LETTER_R_WITH_ACUTE,
            LatinExtendedA::LatinCapitalLetterRWithCedilla => LATIN_CAPITAL_LETTER_R_WITH_CEDILLA,
            LatinExtendedA::LatinSmallLetterRWithCedilla => LATIN_SMALL_LETTER_R_WITH_CEDILLA,
            LatinExtendedA::LatinCapitalLetterRWithCaron => LATIN_CAPITAL_LETTER_R_WITH_CARON,
            LatinExtendedA::LatinSmallLetterRWithCaron => LATIN_SMALL_LETTER_R_WITH_CARON,
            LatinExtendedA::LatinCapitalLetterSWithAcute => LATIN_CAPITAL_LETTER_S_WITH_ACUTE,
            LatinExtendedA::LatinSmallLetterSWithAcute => LATIN_SMALL_LETTER_S_WITH_ACUTE,
            LatinExtendedA::LatinCapitalLetterSWithCircumflex => LATIN_CAPITAL_LETTER_S_WITH_CIRCUMFLEX,
            LatinExtendedA::LatinSmallLetterSWithCircumflex => LATIN_SMALL_LETTER_S_WITH_CIRCUMFLEX,
            LatinExtendedA::LatinCapitalLetterSWithCedilla => LATIN_CAPITAL_LETTER_S_WITH_CEDILLA,
            LatinExtendedA::LatinSmallLetterSWithCedilla => LATIN_SMALL_LETTER_S_WITH_CEDILLA,
            LatinExtendedA::LatinCapitalLetterSWithCaron => LATIN_CAPITAL_LETTER_S_WITH_CARON,
            LatinExtendedA::LatinSmallLetterSWithCaron => LATIN_SMALL_LETTER_S_WITH_CARON,
            LatinExtendedA::LatinCapitalLetterTWithCedilla => LATIN_CAPITAL_LETTER_T_WITH_CEDILLA,
            LatinExtendedA::LatinSmallLetterTWithCedilla => LATIN_SMALL_LETTER_T_WITH_CEDILLA,
            LatinExtendedA::LatinCapitalLetterTWithCaron => LATIN_CAPITAL_LETTER_T_WITH_CARON,
            LatinExtendedA::LatinSmallLetterTWithCaron => LATIN_SMALL_LETTER_T_WITH_CARON,
            LatinExtendedA::LatinCapitalLetterTWithStroke => LATIN_CAPITAL_LETTER_T_WITH_STROKE,
            LatinExtendedA::LatinSmallLetterTWithStroke => LATIN_SMALL_LETTER_T_WITH_STROKE,
            LatinExtendedA::LatinCapitalLetterUWithTilde => LATIN_CAPITAL_LETTER_U_WITH_TILDE,
            LatinExtendedA::LatinSmallLetterUWithTilde => LATIN_SMALL_LETTER_U_WITH_TILDE,
            LatinExtendedA::LatinCapitalLetterUWithMacron => LATIN_CAPITAL_LETTER_U_WITH_MACRON,
            LatinExtendedA::LatinSmallLetterUWithMacron => LATIN_SMALL_LETTER_U_WITH_MACRON,
            LatinExtendedA::LatinCapitalLetterUWithBreve => LATIN_CAPITAL_LETTER_U_WITH_BREVE,
            LatinExtendedA::LatinSmallLetterUWithBreve => LATIN_SMALL_LETTER_U_WITH_BREVE,
            LatinExtendedA::LatinCapitalLetterUWithRingAbove => LATIN_CAPITAL_LETTER_U_WITH_RING_ABOVE,
            LatinExtendedA::LatinSmallLetterUWithRingAbove => LATIN_SMALL_LETTER_U_WITH_RING_ABOVE,
            LatinExtendedA::LatinCapitalLetterUWithDoubleAcute => LATIN_CAPITAL_LETTER_U_WITH_DOUBLE_ACUTE,
            LatinExtendedA::LatinSmallLetterUWithDoubleAcute => LATIN_SMALL_LETTER_U_WITH_DOUBLE_ACUTE,
            LatinExtendedA::LatinCapitalLetterUWithOgonek => LATIN_CAPITAL_LETTER_U_WITH_OGONEK,
            LatinExtendedA::LatinSmallLetterUWithOgonek => LATIN_SMALL_LETTER_U_WITH_OGONEK,
            LatinExtendedA::LatinCapitalLetterWWithCircumflex => LATIN_CAPITAL_LETTER_W_WITH_CIRCUMFLEX,
            LatinExtendedA::LatinSmallLetterWWithCircumflex => LATIN_SMALL_LETTER_W_WITH_CIRCUMFLEX,
            LatinExtendedA::LatinCapitalLetterYWithCircumflex => LATIN_CAPITAL_LETTER_Y_WITH_CIRCUMFLEX,
            LatinExtendedA::LatinSmallLetterYWithCircumflex => LATIN_SMALL_LETTER_Y_WITH_CIRCUMFLEX,
            LatinExtendedA::LatinCapitalLetterYWithDiaeresis => LATIN_CAPITAL_LETTER_Y_WITH_DIAERESIS,
            LatinExtendedA::LatinCapitalLetterZWithAcute => LATIN_CAPITAL_LETTER_Z_WITH_ACUTE,
            LatinExtendedA::LatinSmallLetterZWithAcute => LATIN_SMALL_LETTER_Z_WITH_ACUTE,
            LatinExtendedA::LatinCapitalLetterZWithDotAbove => LATIN_CAPITAL_LETTER_Z_WITH_DOT_ABOVE,
            LatinExtendedA::LatinSmallLetterZWithDotAbove => LATIN_SMALL_LETTER_Z_WITH_DOT_ABOVE,
            LatinExtendedA::LatinCapitalLetterZWithCaron => LATIN_CAPITAL_LETTER_Z_WITH_CARON,
            LatinExtendedA::LatinSmallLetterZWithCaron => LATIN_SMALL_LETTER_Z_WITH_CARON,
        }
    }
}

impl std::convert::TryFrom<char> for LatinExtendedA {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LATIN_CAPITAL_LETTER_A_WITH_MACRON => Ok(LatinExtendedA::LatinCapitalLetterAWithMacron),
            LATIN_SMALL_LETTER_A_WITH_MACRON => Ok(LatinExtendedA::LatinSmallLetterAWithMacron),
            LATIN_CAPITAL_LETTER_A_WITH_BREVE => Ok(LatinExtendedA::LatinCapitalLetterAWithBreve),
            LATIN_SMALL_LETTER_A_WITH_BREVE => Ok(LatinExtendedA::LatinSmallLetterAWithBreve),
            LATIN_CAPITAL_LETTER_A_WITH_OGONEK => Ok(LatinExtendedA::LatinCapitalLetterAWithOgonek),
            LATIN_SMALL_LETTER_A_WITH_OGONEK => Ok(LatinExtendedA::LatinSmallLetterAWithOgonek),
            LATIN_CAPITAL_LETTER_C_WITH_ACUTE => Ok(LatinExtendedA::LatinCapitalLetterCWithAcute),
            LATIN_SMALL_LETTER_C_WITH_ACUTE => Ok(LatinExtendedA::LatinSmallLetterCWithAcute),
            LATIN_CAPITAL_LETTER_C_WITH_CIRCUMFLEX => Ok(LatinExtendedA::LatinCapitalLetterCWithCircumflex),
            LATIN_SMALL_LETTER_C_WITH_CIRCUMFLEX => Ok(LatinExtendedA::LatinSmallLetterCWithCircumflex),
            LATIN_CAPITAL_LETTER_C_WITH_DOT_ABOVE => Ok(LatinExtendedA::LatinCapitalLetterCWithDotAbove),
            LATIN_SMALL_LETTER_C_WITH_DOT_ABOVE => Ok(LatinExtendedA::LatinSmallLetterCWithDotAbove),
            LATIN_CAPITAL_LETTER_C_WITH_CARON => Ok(LatinExtendedA::LatinCapitalLetterCWithCaron),
            LATIN_SMALL_LETTER_C_WITH_CARON => Ok(LatinExtendedA::LatinSmallLetterCWithCaron),
            LATIN_CAPITAL_LETTER_D_WITH_CARON => Ok(LatinExtendedA::LatinCapitalLetterDWithCaron),
            LATIN_SMALL_LETTER_D_WITH_CARON => Ok(LatinExtendedA::LatinSmallLetterDWithCaron),
            LATIN_CAPITAL_LETTER_D_WITH_STROKE => Ok(LatinExtendedA::LatinCapitalLetterDWithStroke),
            LATIN_SMALL_LETTER_D_WITH_STROKE => Ok(LatinExtendedA::LatinSmallLetterDWithStroke),
            LATIN_CAPITAL_LETTER_E_WITH_MACRON => Ok(LatinExtendedA::LatinCapitalLetterEWithMacron),
            LATIN_SMALL_LETTER_E_WITH_MACRON => Ok(LatinExtendedA::LatinSmallLetterEWithMacron),
            LATIN_CAPITAL_LETTER_E_WITH_BREVE => Ok(LatinExtendedA::LatinCapitalLetterEWithBreve),
            LATIN_SMALL_LETTER_E_WITH_BREVE => Ok(LatinExtendedA::LatinSmallLetterEWithBreve),
            LATIN_CAPITAL_LETTER_E_WITH_DOT_ABOVE => Ok(LatinExtendedA::LatinCapitalLetterEWithDotAbove),
            LATIN_SMALL_LETTER_E_WITH_DOT_ABOVE => Ok(LatinExtendedA::LatinSmallLetterEWithDotAbove),
            LATIN_CAPITAL_LETTER_E_WITH_OGONEK => Ok(LatinExtendedA::LatinCapitalLetterEWithOgonek),
            LATIN_SMALL_LETTER_E_WITH_OGONEK => Ok(LatinExtendedA::LatinSmallLetterEWithOgonek),
            LATIN_CAPITAL_LETTER_E_WITH_CARON => Ok(LatinExtendedA::LatinCapitalLetterEWithCaron),
            LATIN_SMALL_LETTER_E_WITH_CARON => Ok(LatinExtendedA::LatinSmallLetterEWithCaron),
            LATIN_CAPITAL_LETTER_G_WITH_CIRCUMFLEX => Ok(LatinExtendedA::LatinCapitalLetterGWithCircumflex),
            LATIN_SMALL_LETTER_G_WITH_CIRCUMFLEX => Ok(LatinExtendedA::LatinSmallLetterGWithCircumflex),
            LATIN_CAPITAL_LETTER_G_WITH_BREVE => Ok(LatinExtendedA::LatinCapitalLetterGWithBreve),
            LATIN_SMALL_LETTER_G_WITH_BREVE => Ok(LatinExtendedA::LatinSmallLetterGWithBreve),
            LATIN_CAPITAL_LETTER_G_WITH_DOT_ABOVE => Ok(LatinExtendedA::LatinCapitalLetterGWithDotAbove),
            LATIN_SMALL_LETTER_G_WITH_DOT_ABOVE => Ok(LatinExtendedA::LatinSmallLetterGWithDotAbove),
            LATIN_CAPITAL_LETTER_G_WITH_CEDILLA => Ok(LatinExtendedA::LatinCapitalLetterGWithCedilla),
            LATIN_SMALL_LETTER_G_WITH_CEDILLA => Ok(LatinExtendedA::LatinSmallLetterGWithCedilla),
            LATIN_CAPITAL_LETTER_H_WITH_CIRCUMFLEX => Ok(LatinExtendedA::LatinCapitalLetterHWithCircumflex),
            LATIN_SMALL_LETTER_H_WITH_CIRCUMFLEX => Ok(LatinExtendedA::LatinSmallLetterHWithCircumflex),
            LATIN_CAPITAL_LETTER_H_WITH_STROKE => Ok(LatinExtendedA::LatinCapitalLetterHWithStroke),
            LATIN_SMALL_LETTER_H_WITH_STROKE => Ok(LatinExtendedA::LatinSmallLetterHWithStroke),
            LATIN_CAPITAL_LETTER_I_WITH_TILDE => Ok(LatinExtendedA::LatinCapitalLetterIWithTilde),
            LATIN_SMALL_LETTER_I_WITH_TILDE => Ok(LatinExtendedA::LatinSmallLetterIWithTilde),
            LATIN_CAPITAL_LETTER_I_WITH_MACRON => Ok(LatinExtendedA::LatinCapitalLetterIWithMacron),
            LATIN_SMALL_LETTER_I_WITH_MACRON => Ok(LatinExtendedA::LatinSmallLetterIWithMacron),
            LATIN_CAPITAL_LETTER_I_WITH_BREVE => Ok(LatinExtendedA::LatinCapitalLetterIWithBreve),
            LATIN_SMALL_LETTER_I_WITH_BREVE => Ok(LatinExtendedA::LatinSmallLetterIWithBreve),
            LATIN_CAPITAL_LETTER_I_WITH_OGONEK => Ok(LatinExtendedA::LatinCapitalLetterIWithOgonek),
            LATIN_SMALL_LETTER_I_WITH_OGONEK => Ok(LatinExtendedA::LatinSmallLetterIWithOgonek),
            LATIN_CAPITAL_LETTER_I_WITH_DOT_ABOVE => Ok(LatinExtendedA::LatinCapitalLetterIWithDotAbove),
            LATIN_SMALL_LETTER_DOTLESS_I => Ok(LatinExtendedA::LatinSmallLetterDotlessI),
            LATIN_CAPITAL_LIGATURE_IJ => Ok(LatinExtendedA::LatinCapitalLigatureIj),
            LATIN_SMALL_LIGATURE_IJ => Ok(LatinExtendedA::LatinSmallLigatureIj),
            LATIN_CAPITAL_LETTER_J_WITH_CIRCUMFLEX => Ok(LatinExtendedA::LatinCapitalLetterJWithCircumflex),
            LATIN_SMALL_LETTER_J_WITH_CIRCUMFLEX => Ok(LatinExtendedA::LatinSmallLetterJWithCircumflex),
            LATIN_CAPITAL_LETTER_K_WITH_CEDILLA => Ok(LatinExtendedA::LatinCapitalLetterKWithCedilla),
            LATIN_SMALL_LETTER_K_WITH_CEDILLA => Ok(LatinExtendedA::LatinSmallLetterKWithCedilla),
            LATIN_SMALL_LETTER_KRA => Ok(LatinExtendedA::LatinSmallLetterKra),
            LATIN_CAPITAL_LETTER_L_WITH_ACUTE => Ok(LatinExtendedA::LatinCapitalLetterLWithAcute),
            LATIN_SMALL_LETTER_L_WITH_ACUTE => Ok(LatinExtendedA::LatinSmallLetterLWithAcute),
            LATIN_CAPITAL_LETTER_L_WITH_CEDILLA => Ok(LatinExtendedA::LatinCapitalLetterLWithCedilla),
            LATIN_SMALL_LETTER_L_WITH_CEDILLA => Ok(LatinExtendedA::LatinSmallLetterLWithCedilla),
            LATIN_CAPITAL_LETTER_L_WITH_CARON => Ok(LatinExtendedA::LatinCapitalLetterLWithCaron),
            LATIN_SMALL_LETTER_L_WITH_CARON => Ok(LatinExtendedA::LatinSmallLetterLWithCaron),
            LATIN_CAPITAL_LETTER_L_WITH_MIDDLE_DOT => Ok(LatinExtendedA::LatinCapitalLetterLWithMiddleDot),
            LATIN_SMALL_LETTER_L_WITH_MIDDLE_DOT => Ok(LatinExtendedA::LatinSmallLetterLWithMiddleDot),
            LATIN_CAPITAL_LETTER_L_WITH_STROKE => Ok(LatinExtendedA::LatinCapitalLetterLWithStroke),
            LATIN_SMALL_LETTER_L_WITH_STROKE => Ok(LatinExtendedA::LatinSmallLetterLWithStroke),
            LATIN_CAPITAL_LETTER_N_WITH_ACUTE => Ok(LatinExtendedA::LatinCapitalLetterNWithAcute),
            LATIN_SMALL_LETTER_N_WITH_ACUTE => Ok(LatinExtendedA::LatinSmallLetterNWithAcute),
            LATIN_CAPITAL_LETTER_N_WITH_CEDILLA => Ok(LatinExtendedA::LatinCapitalLetterNWithCedilla),
            LATIN_SMALL_LETTER_N_WITH_CEDILLA => Ok(LatinExtendedA::LatinSmallLetterNWithCedilla),
            LATIN_CAPITAL_LETTER_N_WITH_CARON => Ok(LatinExtendedA::LatinCapitalLetterNWithCaron),
            LATIN_SMALL_LETTER_N_WITH_CARON => Ok(LatinExtendedA::LatinSmallLetterNWithCaron),
            LATIN_SMALL_LETTER_N_PRECEDED_BY_APOSTROPHE => Ok(LatinExtendedA::LatinSmallLetterNPrecededByApostrophe),
            LATIN_CAPITAL_LETTER_ENG => Ok(LatinExtendedA::LatinCapitalLetterEng),
            LATIN_SMALL_LETTER_ENG => Ok(LatinExtendedA::LatinSmallLetterEng),
            LATIN_CAPITAL_LETTER_O_WITH_MACRON => Ok(LatinExtendedA::LatinCapitalLetterOWithMacron),
            LATIN_SMALL_LETTER_O_WITH_MACRON => Ok(LatinExtendedA::LatinSmallLetterOWithMacron),
            LATIN_CAPITAL_LETTER_O_WITH_BREVE => Ok(LatinExtendedA::LatinCapitalLetterOWithBreve),
            LATIN_SMALL_LETTER_O_WITH_BREVE => Ok(LatinExtendedA::LatinSmallLetterOWithBreve),
            LATIN_CAPITAL_LETTER_O_WITH_DOUBLE_ACUTE => Ok(LatinExtendedA::LatinCapitalLetterOWithDoubleAcute),
            LATIN_SMALL_LETTER_O_WITH_DOUBLE_ACUTE => Ok(LatinExtendedA::LatinSmallLetterOWithDoubleAcute),
            LATIN_CAPITAL_LIGATURE_OE => Ok(LatinExtendedA::LatinCapitalLigatureOe),
            LATIN_SMALL_LIGATURE_OE => Ok(LatinExtendedA::LatinSmallLigatureOe),
            LATIN_CAPITAL_LETTER_R_WITH_ACUTE => Ok(LatinExtendedA::LatinCapitalLetterRWithAcute),
            LATIN_SMALL_LETTER_R_WITH_ACUTE => Ok(LatinExtendedA::LatinSmallLetterRWithAcute),
            LATIN_CAPITAL_LETTER_R_WITH_CEDILLA => Ok(LatinExtendedA::LatinCapitalLetterRWithCedilla),
            LATIN_SMALL_LETTER_R_WITH_CEDILLA => Ok(LatinExtendedA::LatinSmallLetterRWithCedilla),
            LATIN_CAPITAL_LETTER_R_WITH_CARON => Ok(LatinExtendedA::LatinCapitalLetterRWithCaron),
            LATIN_SMALL_LETTER_R_WITH_CARON => Ok(LatinExtendedA::LatinSmallLetterRWithCaron),
            LATIN_CAPITAL_LETTER_S_WITH_ACUTE => Ok(LatinExtendedA::LatinCapitalLetterSWithAcute),
            LATIN_SMALL_LETTER_S_WITH_ACUTE => Ok(LatinExtendedA::LatinSmallLetterSWithAcute),
            LATIN_CAPITAL_LETTER_S_WITH_CIRCUMFLEX => Ok(LatinExtendedA::LatinCapitalLetterSWithCircumflex),
            LATIN_SMALL_LETTER_S_WITH_CIRCUMFLEX => Ok(LatinExtendedA::LatinSmallLetterSWithCircumflex),
            LATIN_CAPITAL_LETTER_S_WITH_CEDILLA => Ok(LatinExtendedA::LatinCapitalLetterSWithCedilla),
            LATIN_SMALL_LETTER_S_WITH_CEDILLA => Ok(LatinExtendedA::LatinSmallLetterSWithCedilla),
            LATIN_CAPITAL_LETTER_S_WITH_CARON => Ok(LatinExtendedA::LatinCapitalLetterSWithCaron),
            LATIN_SMALL_LETTER_S_WITH_CARON => Ok(LatinExtendedA::LatinSmallLetterSWithCaron),
            LATIN_CAPITAL_LETTER_T_WITH_CEDILLA => Ok(LatinExtendedA::LatinCapitalLetterTWithCedilla),
            LATIN_SMALL_LETTER_T_WITH_CEDILLA => Ok(LatinExtendedA::LatinSmallLetterTWithCedilla),
            LATIN_CAPITAL_LETTER_T_WITH_CARON => Ok(LatinExtendedA::LatinCapitalLetterTWithCaron),
            LATIN_SMALL_LETTER_T_WITH_CARON => Ok(LatinExtendedA::LatinSmallLetterTWithCaron),
            LATIN_CAPITAL_LETTER_T_WITH_STROKE => Ok(LatinExtendedA::LatinCapitalLetterTWithStroke),
            LATIN_SMALL_LETTER_T_WITH_STROKE => Ok(LatinExtendedA::LatinSmallLetterTWithStroke),
            LATIN_CAPITAL_LETTER_U_WITH_TILDE => Ok(LatinExtendedA::LatinCapitalLetterUWithTilde),
            LATIN_SMALL_LETTER_U_WITH_TILDE => Ok(LatinExtendedA::LatinSmallLetterUWithTilde),
            LATIN_CAPITAL_LETTER_U_WITH_MACRON => Ok(LatinExtendedA::LatinCapitalLetterUWithMacron),
            LATIN_SMALL_LETTER_U_WITH_MACRON => Ok(LatinExtendedA::LatinSmallLetterUWithMacron),
            LATIN_CAPITAL_LETTER_U_WITH_BREVE => Ok(LatinExtendedA::LatinCapitalLetterUWithBreve),
            LATIN_SMALL_LETTER_U_WITH_BREVE => Ok(LatinExtendedA::LatinSmallLetterUWithBreve),
            LATIN_CAPITAL_LETTER_U_WITH_RING_ABOVE => Ok(LatinExtendedA::LatinCapitalLetterUWithRingAbove),
            LATIN_SMALL_LETTER_U_WITH_RING_ABOVE => Ok(LatinExtendedA::LatinSmallLetterUWithRingAbove),
            LATIN_CAPITAL_LETTER_U_WITH_DOUBLE_ACUTE => Ok(LatinExtendedA::LatinCapitalLetterUWithDoubleAcute),
            LATIN_SMALL_LETTER_U_WITH_DOUBLE_ACUTE => Ok(LatinExtendedA::LatinSmallLetterUWithDoubleAcute),
            LATIN_CAPITAL_LETTER_U_WITH_OGONEK => Ok(LatinExtendedA::LatinCapitalLetterUWithOgonek),
            LATIN_SMALL_LETTER_U_WITH_OGONEK => Ok(LatinExtendedA::LatinSmallLetterUWithOgonek),
            LATIN_CAPITAL_LETTER_W_WITH_CIRCUMFLEX => Ok(LatinExtendedA::LatinCapitalLetterWWithCircumflex),
            LATIN_SMALL_LETTER_W_WITH_CIRCUMFLEX => Ok(LatinExtendedA::LatinSmallLetterWWithCircumflex),
            LATIN_CAPITAL_LETTER_Y_WITH_CIRCUMFLEX => Ok(LatinExtendedA::LatinCapitalLetterYWithCircumflex),
            LATIN_SMALL_LETTER_Y_WITH_CIRCUMFLEX => Ok(LatinExtendedA::LatinSmallLetterYWithCircumflex),
            LATIN_CAPITAL_LETTER_Y_WITH_DIAERESIS => Ok(LatinExtendedA::LatinCapitalLetterYWithDiaeresis),
            LATIN_CAPITAL_LETTER_Z_WITH_ACUTE => Ok(LatinExtendedA::LatinCapitalLetterZWithAcute),
            LATIN_SMALL_LETTER_Z_WITH_ACUTE => Ok(LatinExtendedA::LatinSmallLetterZWithAcute),
            LATIN_CAPITAL_LETTER_Z_WITH_DOT_ABOVE => Ok(LatinExtendedA::LatinCapitalLetterZWithDotAbove),
            LATIN_SMALL_LETTER_Z_WITH_DOT_ABOVE => Ok(LatinExtendedA::LatinSmallLetterZWithDotAbove),
            LATIN_CAPITAL_LETTER_Z_WITH_CARON => Ok(LatinExtendedA::LatinCapitalLetterZWithCaron),
            LATIN_SMALL_LETTER_Z_WITH_CARON => Ok(LatinExtendedA::LatinSmallLetterZWithCaron),
            _ => Err(()),
        }
    }
}

impl Into<u32> for LatinExtendedA {
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

impl std::convert::TryFrom<u32> for LatinExtendedA {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for LatinExtendedA {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl LatinExtendedA {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        LatinExtendedA::LatinCapitalLetterAWithMacron
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            LatinExtendedA::LatinCapitalLetterAWithMacron => "latin capital letter a with macron",
            LatinExtendedA::LatinSmallLetterAWithMacron => "latin small letter a with macron",
            LatinExtendedA::LatinCapitalLetterAWithBreve => "latin capital letter a with breve",
            LatinExtendedA::LatinSmallLetterAWithBreve => "latin small letter a with breve",
            LatinExtendedA::LatinCapitalLetterAWithOgonek => "latin capital letter a with ogonek",
            LatinExtendedA::LatinSmallLetterAWithOgonek => "latin small letter a with ogonek",
            LatinExtendedA::LatinCapitalLetterCWithAcute => "latin capital letter c with acute",
            LatinExtendedA::LatinSmallLetterCWithAcute => "latin small letter c with acute",
            LatinExtendedA::LatinCapitalLetterCWithCircumflex => "latin capital letter c with circumflex",
            LatinExtendedA::LatinSmallLetterCWithCircumflex => "latin small letter c with circumflex",
            LatinExtendedA::LatinCapitalLetterCWithDotAbove => "latin capital letter c with dot above",
            LatinExtendedA::LatinSmallLetterCWithDotAbove => "latin small letter c with dot above",
            LatinExtendedA::LatinCapitalLetterCWithCaron => "latin capital letter c with caron",
            LatinExtendedA::LatinSmallLetterCWithCaron => "latin small letter c with caron",
            LatinExtendedA::LatinCapitalLetterDWithCaron => "latin capital letter d with caron",
            LatinExtendedA::LatinSmallLetterDWithCaron => "latin small letter d with caron",
            LatinExtendedA::LatinCapitalLetterDWithStroke => "latin capital letter d with stroke",
            LatinExtendedA::LatinSmallLetterDWithStroke => "latin small letter d with stroke",
            LatinExtendedA::LatinCapitalLetterEWithMacron => "latin capital letter e with macron",
            LatinExtendedA::LatinSmallLetterEWithMacron => "latin small letter e with macron",
            LatinExtendedA::LatinCapitalLetterEWithBreve => "latin capital letter e with breve",
            LatinExtendedA::LatinSmallLetterEWithBreve => "latin small letter e with breve",
            LatinExtendedA::LatinCapitalLetterEWithDotAbove => "latin capital letter e with dot above",
            LatinExtendedA::LatinSmallLetterEWithDotAbove => "latin small letter e with dot above",
            LatinExtendedA::LatinCapitalLetterEWithOgonek => "latin capital letter e with ogonek",
            LatinExtendedA::LatinSmallLetterEWithOgonek => "latin small letter e with ogonek",
            LatinExtendedA::LatinCapitalLetterEWithCaron => "latin capital letter e with caron",
            LatinExtendedA::LatinSmallLetterEWithCaron => "latin small letter e with caron",
            LatinExtendedA::LatinCapitalLetterGWithCircumflex => "latin capital letter g with circumflex",
            LatinExtendedA::LatinSmallLetterGWithCircumflex => "latin small letter g with circumflex",
            LatinExtendedA::LatinCapitalLetterGWithBreve => "latin capital letter g with breve",
            LatinExtendedA::LatinSmallLetterGWithBreve => "latin small letter g with breve",
            LatinExtendedA::LatinCapitalLetterGWithDotAbove => "latin capital letter g with dot above",
            LatinExtendedA::LatinSmallLetterGWithDotAbove => "latin small letter g with dot above",
            LatinExtendedA::LatinCapitalLetterGWithCedilla => "latin capital letter g with cedilla",
            LatinExtendedA::LatinSmallLetterGWithCedilla => "latin small letter g with cedilla",
            LatinExtendedA::LatinCapitalLetterHWithCircumflex => "latin capital letter h with circumflex",
            LatinExtendedA::LatinSmallLetterHWithCircumflex => "latin small letter h with circumflex",
            LatinExtendedA::LatinCapitalLetterHWithStroke => "latin capital letter h with stroke",
            LatinExtendedA::LatinSmallLetterHWithStroke => "latin small letter h with stroke",
            LatinExtendedA::LatinCapitalLetterIWithTilde => "latin capital letter i with tilde",
            LatinExtendedA::LatinSmallLetterIWithTilde => "latin small letter i with tilde",
            LatinExtendedA::LatinCapitalLetterIWithMacron => "latin capital letter i with macron",
            LatinExtendedA::LatinSmallLetterIWithMacron => "latin small letter i with macron",
            LatinExtendedA::LatinCapitalLetterIWithBreve => "latin capital letter i with breve",
            LatinExtendedA::LatinSmallLetterIWithBreve => "latin small letter i with breve",
            LatinExtendedA::LatinCapitalLetterIWithOgonek => "latin capital letter i with ogonek",
            LatinExtendedA::LatinSmallLetterIWithOgonek => "latin small letter i with ogonek",
            LatinExtendedA::LatinCapitalLetterIWithDotAbove => "latin capital letter i with dot above",
            LatinExtendedA::LatinSmallLetterDotlessI => "latin small letter dotless i",
            LatinExtendedA::LatinCapitalLigatureIj => "latin capital ligature ij",
            LatinExtendedA::LatinSmallLigatureIj => "latin small ligature ij",
            LatinExtendedA::LatinCapitalLetterJWithCircumflex => "latin capital letter j with circumflex",
            LatinExtendedA::LatinSmallLetterJWithCircumflex => "latin small letter j with circumflex",
            LatinExtendedA::LatinCapitalLetterKWithCedilla => "latin capital letter k with cedilla",
            LatinExtendedA::LatinSmallLetterKWithCedilla => "latin small letter k with cedilla",
            LatinExtendedA::LatinSmallLetterKra => "latin small letter kra",
            LatinExtendedA::LatinCapitalLetterLWithAcute => "latin capital letter l with acute",
            LatinExtendedA::LatinSmallLetterLWithAcute => "latin small letter l with acute",
            LatinExtendedA::LatinCapitalLetterLWithCedilla => "latin capital letter l with cedilla",
            LatinExtendedA::LatinSmallLetterLWithCedilla => "latin small letter l with cedilla",
            LatinExtendedA::LatinCapitalLetterLWithCaron => "latin capital letter l with caron",
            LatinExtendedA::LatinSmallLetterLWithCaron => "latin small letter l with caron",
            LatinExtendedA::LatinCapitalLetterLWithMiddleDot => "latin capital letter l with middle dot",
            LatinExtendedA::LatinSmallLetterLWithMiddleDot => "latin small letter l with middle dot",
            LatinExtendedA::LatinCapitalLetterLWithStroke => "latin capital letter l with stroke",
            LatinExtendedA::LatinSmallLetterLWithStroke => "latin small letter l with stroke",
            LatinExtendedA::LatinCapitalLetterNWithAcute => "latin capital letter n with acute",
            LatinExtendedA::LatinSmallLetterNWithAcute => "latin small letter n with acute",
            LatinExtendedA::LatinCapitalLetterNWithCedilla => "latin capital letter n with cedilla",
            LatinExtendedA::LatinSmallLetterNWithCedilla => "latin small letter n with cedilla",
            LatinExtendedA::LatinCapitalLetterNWithCaron => "latin capital letter n with caron",
            LatinExtendedA::LatinSmallLetterNWithCaron => "latin small letter n with caron",
            LatinExtendedA::LatinSmallLetterNPrecededByApostrophe => "latin small letter n preceded by apostrophe",
            LatinExtendedA::LatinCapitalLetterEng => "latin capital letter eng",
            LatinExtendedA::LatinSmallLetterEng => "latin small letter eng",
            LatinExtendedA::LatinCapitalLetterOWithMacron => "latin capital letter o with macron",
            LatinExtendedA::LatinSmallLetterOWithMacron => "latin small letter o with macron",
            LatinExtendedA::LatinCapitalLetterOWithBreve => "latin capital letter o with breve",
            LatinExtendedA::LatinSmallLetterOWithBreve => "latin small letter o with breve",
            LatinExtendedA::LatinCapitalLetterOWithDoubleAcute => "latin capital letter o with double acute",
            LatinExtendedA::LatinSmallLetterOWithDoubleAcute => "latin small letter o with double acute",
            LatinExtendedA::LatinCapitalLigatureOe => "latin capital ligature oe",
            LatinExtendedA::LatinSmallLigatureOe => "latin small ligature oe",
            LatinExtendedA::LatinCapitalLetterRWithAcute => "latin capital letter r with acute",
            LatinExtendedA::LatinSmallLetterRWithAcute => "latin small letter r with acute",
            LatinExtendedA::LatinCapitalLetterRWithCedilla => "latin capital letter r with cedilla",
            LatinExtendedA::LatinSmallLetterRWithCedilla => "latin small letter r with cedilla",
            LatinExtendedA::LatinCapitalLetterRWithCaron => "latin capital letter r with caron",
            LatinExtendedA::LatinSmallLetterRWithCaron => "latin small letter r with caron",
            LatinExtendedA::LatinCapitalLetterSWithAcute => "latin capital letter s with acute",
            LatinExtendedA::LatinSmallLetterSWithAcute => "latin small letter s with acute",
            LatinExtendedA::LatinCapitalLetterSWithCircumflex => "latin capital letter s with circumflex",
            LatinExtendedA::LatinSmallLetterSWithCircumflex => "latin small letter s with circumflex",
            LatinExtendedA::LatinCapitalLetterSWithCedilla => "latin capital letter s with cedilla",
            LatinExtendedA::LatinSmallLetterSWithCedilla => "latin small letter s with cedilla",
            LatinExtendedA::LatinCapitalLetterSWithCaron => "latin capital letter s with caron",
            LatinExtendedA::LatinSmallLetterSWithCaron => "latin small letter s with caron",
            LatinExtendedA::LatinCapitalLetterTWithCedilla => "latin capital letter t with cedilla",
            LatinExtendedA::LatinSmallLetterTWithCedilla => "latin small letter t with cedilla",
            LatinExtendedA::LatinCapitalLetterTWithCaron => "latin capital letter t with caron",
            LatinExtendedA::LatinSmallLetterTWithCaron => "latin small letter t with caron",
            LatinExtendedA::LatinCapitalLetterTWithStroke => "latin capital letter t with stroke",
            LatinExtendedA::LatinSmallLetterTWithStroke => "latin small letter t with stroke",
            LatinExtendedA::LatinCapitalLetterUWithTilde => "latin capital letter u with tilde",
            LatinExtendedA::LatinSmallLetterUWithTilde => "latin small letter u with tilde",
            LatinExtendedA::LatinCapitalLetterUWithMacron => "latin capital letter u with macron",
            LatinExtendedA::LatinSmallLetterUWithMacron => "latin small letter u with macron",
            LatinExtendedA::LatinCapitalLetterUWithBreve => "latin capital letter u with breve",
            LatinExtendedA::LatinSmallLetterUWithBreve => "latin small letter u with breve",
            LatinExtendedA::LatinCapitalLetterUWithRingAbove => "latin capital letter u with ring above",
            LatinExtendedA::LatinSmallLetterUWithRingAbove => "latin small letter u with ring above",
            LatinExtendedA::LatinCapitalLetterUWithDoubleAcute => "latin capital letter u with double acute",
            LatinExtendedA::LatinSmallLetterUWithDoubleAcute => "latin small letter u with double acute",
            LatinExtendedA::LatinCapitalLetterUWithOgonek => "latin capital letter u with ogonek",
            LatinExtendedA::LatinSmallLetterUWithOgonek => "latin small letter u with ogonek",
            LatinExtendedA::LatinCapitalLetterWWithCircumflex => "latin capital letter w with circumflex",
            LatinExtendedA::LatinSmallLetterWWithCircumflex => "latin small letter w with circumflex",
            LatinExtendedA::LatinCapitalLetterYWithCircumflex => "latin capital letter y with circumflex",
            LatinExtendedA::LatinSmallLetterYWithCircumflex => "latin small letter y with circumflex",
            LatinExtendedA::LatinCapitalLetterYWithDiaeresis => "latin capital letter y with diaeresis",
            LatinExtendedA::LatinCapitalLetterZWithAcute => "latin capital letter z with acute",
            LatinExtendedA::LatinSmallLetterZWithAcute => "latin small letter z with acute",
            LatinExtendedA::LatinCapitalLetterZWithDotAbove => "latin capital letter z with dot above",
            LatinExtendedA::LatinSmallLetterZWithDotAbove => "latin small letter z with dot above",
            LatinExtendedA::LatinCapitalLetterZWithCaron => "latin capital letter z with caron",
            LatinExtendedA::LatinSmallLetterZWithCaron => "latin small letter z with caron",
        }
    }
}
