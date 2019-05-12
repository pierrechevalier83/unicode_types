/// \u{1e00} → \u{1eff}\
///\
/// Ḁ ḁ Ḃ ḃ Ḅ ḅ Ḇ ḇ Ḉ ḉ Ḋ ḋ Ḍ ḍ Ḏ ḏ\
/// Ḑ ḑ Ḓ ḓ Ḕ ḕ Ḗ ḗ Ḙ ḙ Ḛ ḛ Ḝ ḝ Ḟ ḟ\
/// Ḡ ḡ Ḣ ḣ Ḥ ḥ Ḧ ḧ Ḩ ḩ Ḫ ḫ Ḭ ḭ Ḯ ḯ\
/// Ḱ ḱ Ḳ ḳ Ḵ ḵ Ḷ ḷ Ḹ ḹ Ḻ ḻ Ḽ ḽ Ḿ ḿ\
/// Ṁ ṁ Ṃ ṃ Ṅ ṅ Ṇ ṇ Ṉ ṉ Ṋ ṋ Ṍ ṍ Ṏ ṏ\
/// Ṑ ṑ Ṓ ṓ Ṕ ṕ Ṗ ṗ Ṙ ṙ Ṛ ṛ Ṝ ṝ Ṟ ṟ\
/// Ṡ ṡ Ṣ ṣ Ṥ ṥ Ṧ ṧ Ṩ ṩ Ṫ ṫ Ṭ ṭ Ṯ ṯ\
/// Ṱ ṱ Ṳ ṳ Ṵ ṵ Ṷ ṷ Ṹ ṹ Ṻ ṻ Ṽ ṽ Ṿ ṿ\
/// Ẁ ẁ Ẃ ẃ Ẅ ẅ Ẇ ẇ Ẉ ẉ Ẋ ẋ Ẍ ẍ Ẏ ẏ\
/// Ẑ ẑ Ẓ ẓ Ẕ ẕ ẖ ẗ ẘ ẙ ẚ ẛ ẜ ẝ ẞ ẟ\
/// Ạ ạ Ả ả Ấ ấ Ầ ầ Ẩ ẩ Ẫ ẫ Ậ ậ Ắ ắ\
/// Ằ ằ Ẳ ẳ Ẵ ẵ Ặ ặ Ẹ ẹ Ẻ ẻ Ẽ ẽ Ế ế\
/// Ề ề Ể ể Ễ ễ Ệ ệ Ỉ ỉ Ị ị Ọ ọ Ỏ ỏ\
/// Ố ố Ồ ồ Ổ ổ Ỗ ỗ Ộ ộ Ớ ớ Ờ ờ Ở ở\
/// Ỡ ỡ Ợ ợ Ụ ụ Ủ ủ Ứ ứ Ừ ừ Ử ử Ữ ữ\
/// Ự ự Ỳ ỳ Ỵ ỵ Ỷ ỷ Ỹ ỹ Ỻ ỻ Ỽ ỽ Ỿ\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{1e00}: 'Ḁ'
    pub const LATIN_CAPITAL_LETTER_A_WITH_RING_BELOW: char = 'Ḁ';
    /// \u{1e01}: 'ḁ'
    pub const LATIN_SMALL_LETTER_A_WITH_RING_BELOW: char = 'ḁ';
    /// \u{1e02}: 'Ḃ'
    pub const LATIN_CAPITAL_LETTER_B_WITH_DOT_ABOVE: char = 'Ḃ';
    /// \u{1e03}: 'ḃ'
    pub const LATIN_SMALL_LETTER_B_WITH_DOT_ABOVE: char = 'ḃ';
    /// \u{1e04}: 'Ḅ'
    pub const LATIN_CAPITAL_LETTER_B_WITH_DOT_BELOW: char = 'Ḅ';
    /// \u{1e05}: 'ḅ'
    pub const LATIN_SMALL_LETTER_B_WITH_DOT_BELOW: char = 'ḅ';
    /// \u{1e06}: 'Ḇ'
    pub const LATIN_CAPITAL_LETTER_B_WITH_LINE_BELOW: char = 'Ḇ';
    /// \u{1e07}: 'ḇ'
    pub const LATIN_SMALL_LETTER_B_WITH_LINE_BELOW: char = 'ḇ';
    /// \u{1e08}: 'Ḉ'
    pub const LATIN_CAPITAL_LETTER_C_WITH_CEDILLA_AND_ACUTE: char = 'Ḉ';
    /// \u{1e09}: 'ḉ'
    pub const LATIN_SMALL_LETTER_C_WITH_CEDILLA_AND_ACUTE: char = 'ḉ';
    /// \u{1e0a}: 'Ḋ'
    pub const LATIN_CAPITAL_LETTER_D_WITH_DOT_ABOVE: char = 'Ḋ';
    /// \u{1e0b}: 'ḋ'
    pub const LATIN_SMALL_LETTER_D_WITH_DOT_ABOVE: char = 'ḋ';
    /// \u{1e0c}: 'Ḍ'
    pub const LATIN_CAPITAL_LETTER_D_WITH_DOT_BELOW: char = 'Ḍ';
    /// \u{1e0d}: 'ḍ'
    pub const LATIN_SMALL_LETTER_D_WITH_DOT_BELOW: char = 'ḍ';
    /// \u{1e0e}: 'Ḏ'
    pub const LATIN_CAPITAL_LETTER_D_WITH_LINE_BELOW: char = 'Ḏ';
    /// \u{1e0f}: 'ḏ'
    pub const LATIN_SMALL_LETTER_D_WITH_LINE_BELOW: char = 'ḏ';
    /// \u{1e10}: 'Ḑ'
    pub const LATIN_CAPITAL_LETTER_D_WITH_CEDILLA: char = 'Ḑ';
    /// \u{1e11}: 'ḑ'
    pub const LATIN_SMALL_LETTER_D_WITH_CEDILLA: char = 'ḑ';
    /// \u{1e12}: 'Ḓ'
    pub const LATIN_CAPITAL_LETTER_D_WITH_CIRCUMFLEX_BELOW: char = 'Ḓ';
    /// \u{1e13}: 'ḓ'
    pub const LATIN_SMALL_LETTER_D_WITH_CIRCUMFLEX_BELOW: char = 'ḓ';
    /// \u{1e14}: 'Ḕ'
    pub const LATIN_CAPITAL_LETTER_E_WITH_MACRON_AND_GRAVE: char = 'Ḕ';
    /// \u{1e15}: 'ḕ'
    pub const LATIN_SMALL_LETTER_E_WITH_MACRON_AND_GRAVE: char = 'ḕ';
    /// \u{1e16}: 'Ḗ'
    pub const LATIN_CAPITAL_LETTER_E_WITH_MACRON_AND_ACUTE: char = 'Ḗ';
    /// \u{1e17}: 'ḗ'
    pub const LATIN_SMALL_LETTER_E_WITH_MACRON_AND_ACUTE: char = 'ḗ';
    /// \u{1e18}: 'Ḙ'
    pub const LATIN_CAPITAL_LETTER_E_WITH_CIRCUMFLEX_BELOW: char = 'Ḙ';
    /// \u{1e19}: 'ḙ'
    pub const LATIN_SMALL_LETTER_E_WITH_CIRCUMFLEX_BELOW: char = 'ḙ';
    /// \u{1e1a}: 'Ḛ'
    pub const LATIN_CAPITAL_LETTER_E_WITH_TILDE_BELOW: char = 'Ḛ';
    /// \u{1e1b}: 'ḛ'
    pub const LATIN_SMALL_LETTER_E_WITH_TILDE_BELOW: char = 'ḛ';
    /// \u{1e1c}: 'Ḝ'
    pub const LATIN_CAPITAL_LETTER_E_WITH_CEDILLA_AND_BREVE: char = 'Ḝ';
    /// \u{1e1d}: 'ḝ'
    pub const LATIN_SMALL_LETTER_E_WITH_CEDILLA_AND_BREVE: char = 'ḝ';
    /// \u{1e1e}: 'Ḟ'
    pub const LATIN_CAPITAL_LETTER_F_WITH_DOT_ABOVE: char = 'Ḟ';
    /// \u{1e1f}: 'ḟ'
    pub const LATIN_SMALL_LETTER_F_WITH_DOT_ABOVE: char = 'ḟ';
    /// \u{1e20}: 'Ḡ'
    pub const LATIN_CAPITAL_LETTER_G_WITH_MACRON: char = 'Ḡ';
    /// \u{1e21}: 'ḡ'
    pub const LATIN_SMALL_LETTER_G_WITH_MACRON: char = 'ḡ';
    /// \u{1e22}: 'Ḣ'
    pub const LATIN_CAPITAL_LETTER_H_WITH_DOT_ABOVE: char = 'Ḣ';
    /// \u{1e23}: 'ḣ'
    pub const LATIN_SMALL_LETTER_H_WITH_DOT_ABOVE: char = 'ḣ';
    /// \u{1e24}: 'Ḥ'
    pub const LATIN_CAPITAL_LETTER_H_WITH_DOT_BELOW: char = 'Ḥ';
    /// \u{1e25}: 'ḥ'
    pub const LATIN_SMALL_LETTER_H_WITH_DOT_BELOW: char = 'ḥ';
    /// \u{1e26}: 'Ḧ'
    pub const LATIN_CAPITAL_LETTER_H_WITH_DIAERESIS: char = 'Ḧ';
    /// \u{1e27}: 'ḧ'
    pub const LATIN_SMALL_LETTER_H_WITH_DIAERESIS: char = 'ḧ';
    /// \u{1e28}: 'Ḩ'
    pub const LATIN_CAPITAL_LETTER_H_WITH_CEDILLA: char = 'Ḩ';
    /// \u{1e29}: 'ḩ'
    pub const LATIN_SMALL_LETTER_H_WITH_CEDILLA: char = 'ḩ';
    /// \u{1e2a}: 'Ḫ'
    pub const LATIN_CAPITAL_LETTER_H_WITH_BREVE_BELOW: char = 'Ḫ';
    /// \u{1e2b}: 'ḫ'
    pub const LATIN_SMALL_LETTER_H_WITH_BREVE_BELOW: char = 'ḫ';
    /// \u{1e2c}: 'Ḭ'
    pub const LATIN_CAPITAL_LETTER_I_WITH_TILDE_BELOW: char = 'Ḭ';
    /// \u{1e2d}: 'ḭ'
    pub const LATIN_SMALL_LETTER_I_WITH_TILDE_BELOW: char = 'ḭ';
    /// \u{1e2e}: 'Ḯ'
    pub const LATIN_CAPITAL_LETTER_I_WITH_DIAERESIS_AND_ACUTE: char = 'Ḯ';
    /// \u{1e2f}: 'ḯ'
    pub const LATIN_SMALL_LETTER_I_WITH_DIAERESIS_AND_ACUTE: char = 'ḯ';
    /// \u{1e30}: 'Ḱ'
    pub const LATIN_CAPITAL_LETTER_K_WITH_ACUTE: char = 'Ḱ';
    /// \u{1e31}: 'ḱ'
    pub const LATIN_SMALL_LETTER_K_WITH_ACUTE: char = 'ḱ';
    /// \u{1e32}: 'Ḳ'
    pub const LATIN_CAPITAL_LETTER_K_WITH_DOT_BELOW: char = 'Ḳ';
    /// \u{1e33}: 'ḳ'
    pub const LATIN_SMALL_LETTER_K_WITH_DOT_BELOW: char = 'ḳ';
    /// \u{1e34}: 'Ḵ'
    pub const LATIN_CAPITAL_LETTER_K_WITH_LINE_BELOW: char = 'Ḵ';
    /// \u{1e35}: 'ḵ'
    pub const LATIN_SMALL_LETTER_K_WITH_LINE_BELOW: char = 'ḵ';
    /// \u{1e36}: 'Ḷ'
    pub const LATIN_CAPITAL_LETTER_L_WITH_DOT_BELOW: char = 'Ḷ';
    /// \u{1e37}: 'ḷ'
    pub const LATIN_SMALL_LETTER_L_WITH_DOT_BELOW: char = 'ḷ';
    /// \u{1e38}: 'Ḹ'
    pub const LATIN_CAPITAL_LETTER_L_WITH_DOT_BELOW_AND_MACRON: char = 'Ḹ';
    /// \u{1e39}: 'ḹ'
    pub const LATIN_SMALL_LETTER_L_WITH_DOT_BELOW_AND_MACRON: char = 'ḹ';
    /// \u{1e3a}: 'Ḻ'
    pub const LATIN_CAPITAL_LETTER_L_WITH_LINE_BELOW: char = 'Ḻ';
    /// \u{1e3b}: 'ḻ'
    pub const LATIN_SMALL_LETTER_L_WITH_LINE_BELOW: char = 'ḻ';
    /// \u{1e3c}: 'Ḽ'
    pub const LATIN_CAPITAL_LETTER_L_WITH_CIRCUMFLEX_BELOW: char = 'Ḽ';
    /// \u{1e3d}: 'ḽ'
    pub const LATIN_SMALL_LETTER_L_WITH_CIRCUMFLEX_BELOW: char = 'ḽ';
    /// \u{1e3e}: 'Ḿ'
    pub const LATIN_CAPITAL_LETTER_M_WITH_ACUTE: char = 'Ḿ';
    /// \u{1e3f}: 'ḿ'
    pub const LATIN_SMALL_LETTER_M_WITH_ACUTE: char = 'ḿ';
    /// \u{1e40}: 'Ṁ'
    pub const LATIN_CAPITAL_LETTER_M_WITH_DOT_ABOVE: char = 'Ṁ';
    /// \u{1e41}: 'ṁ'
    pub const LATIN_SMALL_LETTER_M_WITH_DOT_ABOVE: char = 'ṁ';
    /// \u{1e42}: 'Ṃ'
    pub const LATIN_CAPITAL_LETTER_M_WITH_DOT_BELOW: char = 'Ṃ';
    /// \u{1e43}: 'ṃ'
    pub const LATIN_SMALL_LETTER_M_WITH_DOT_BELOW: char = 'ṃ';
    /// \u{1e44}: 'Ṅ'
    pub const LATIN_CAPITAL_LETTER_N_WITH_DOT_ABOVE: char = 'Ṅ';
    /// \u{1e45}: 'ṅ'
    pub const LATIN_SMALL_LETTER_N_WITH_DOT_ABOVE: char = 'ṅ';
    /// \u{1e46}: 'Ṇ'
    pub const LATIN_CAPITAL_LETTER_N_WITH_DOT_BELOW: char = 'Ṇ';
    /// \u{1e47}: 'ṇ'
    pub const LATIN_SMALL_LETTER_N_WITH_DOT_BELOW: char = 'ṇ';
    /// \u{1e48}: 'Ṉ'
    pub const LATIN_CAPITAL_LETTER_N_WITH_LINE_BELOW: char = 'Ṉ';
    /// \u{1e49}: 'ṉ'
    pub const LATIN_SMALL_LETTER_N_WITH_LINE_BELOW: char = 'ṉ';
    /// \u{1e4a}: 'Ṋ'
    pub const LATIN_CAPITAL_LETTER_N_WITH_CIRCUMFLEX_BELOW: char = 'Ṋ';
    /// \u{1e4b}: 'ṋ'
    pub const LATIN_SMALL_LETTER_N_WITH_CIRCUMFLEX_BELOW: char = 'ṋ';
    /// \u{1e4c}: 'Ṍ'
    pub const LATIN_CAPITAL_LETTER_O_WITH_TILDE_AND_ACUTE: char = 'Ṍ';
    /// \u{1e4d}: 'ṍ'
    pub const LATIN_SMALL_LETTER_O_WITH_TILDE_AND_ACUTE: char = 'ṍ';
    /// \u{1e4e}: 'Ṏ'
    pub const LATIN_CAPITAL_LETTER_O_WITH_TILDE_AND_DIAERESIS: char = 'Ṏ';
    /// \u{1e4f}: 'ṏ'
    pub const LATIN_SMALL_LETTER_O_WITH_TILDE_AND_DIAERESIS: char = 'ṏ';
    /// \u{1e50}: 'Ṑ'
    pub const LATIN_CAPITAL_LETTER_O_WITH_MACRON_AND_GRAVE: char = 'Ṑ';
    /// \u{1e51}: 'ṑ'
    pub const LATIN_SMALL_LETTER_O_WITH_MACRON_AND_GRAVE: char = 'ṑ';
    /// \u{1e52}: 'Ṓ'
    pub const LATIN_CAPITAL_LETTER_O_WITH_MACRON_AND_ACUTE: char = 'Ṓ';
    /// \u{1e53}: 'ṓ'
    pub const LATIN_SMALL_LETTER_O_WITH_MACRON_AND_ACUTE: char = 'ṓ';
    /// \u{1e54}: 'Ṕ'
    pub const LATIN_CAPITAL_LETTER_P_WITH_ACUTE: char = 'Ṕ';
    /// \u{1e55}: 'ṕ'
    pub const LATIN_SMALL_LETTER_P_WITH_ACUTE: char = 'ṕ';
    /// \u{1e56}: 'Ṗ'
    pub const LATIN_CAPITAL_LETTER_P_WITH_DOT_ABOVE: char = 'Ṗ';
    /// \u{1e57}: 'ṗ'
    pub const LATIN_SMALL_LETTER_P_WITH_DOT_ABOVE: char = 'ṗ';
    /// \u{1e58}: 'Ṙ'
    pub const LATIN_CAPITAL_LETTER_R_WITH_DOT_ABOVE: char = 'Ṙ';
    /// \u{1e59}: 'ṙ'
    pub const LATIN_SMALL_LETTER_R_WITH_DOT_ABOVE: char = 'ṙ';
    /// \u{1e5a}: 'Ṛ'
    pub const LATIN_CAPITAL_LETTER_R_WITH_DOT_BELOW: char = 'Ṛ';
    /// \u{1e5b}: 'ṛ'
    pub const LATIN_SMALL_LETTER_R_WITH_DOT_BELOW: char = 'ṛ';
    /// \u{1e5c}: 'Ṝ'
    pub const LATIN_CAPITAL_LETTER_R_WITH_DOT_BELOW_AND_MACRON: char = 'Ṝ';
    /// \u{1e5d}: 'ṝ'
    pub const LATIN_SMALL_LETTER_R_WITH_DOT_BELOW_AND_MACRON: char = 'ṝ';
    /// \u{1e5e}: 'Ṟ'
    pub const LATIN_CAPITAL_LETTER_R_WITH_LINE_BELOW: char = 'Ṟ';
    /// \u{1e5f}: 'ṟ'
    pub const LATIN_SMALL_LETTER_R_WITH_LINE_BELOW: char = 'ṟ';
    /// \u{1e60}: 'Ṡ'
    pub const LATIN_CAPITAL_LETTER_S_WITH_DOT_ABOVE: char = 'Ṡ';
    /// \u{1e61}: 'ṡ'
    pub const LATIN_SMALL_LETTER_S_WITH_DOT_ABOVE: char = 'ṡ';
    /// \u{1e62}: 'Ṣ'
    pub const LATIN_CAPITAL_LETTER_S_WITH_DOT_BELOW: char = 'Ṣ';
    /// \u{1e63}: 'ṣ'
    pub const LATIN_SMALL_LETTER_S_WITH_DOT_BELOW: char = 'ṣ';
    /// \u{1e64}: 'Ṥ'
    pub const LATIN_CAPITAL_LETTER_S_WITH_ACUTE_AND_DOT_ABOVE: char = 'Ṥ';
    /// \u{1e65}: 'ṥ'
    pub const LATIN_SMALL_LETTER_S_WITH_ACUTE_AND_DOT_ABOVE: char = 'ṥ';
    /// \u{1e66}: 'Ṧ'
    pub const LATIN_CAPITAL_LETTER_S_WITH_CARON_AND_DOT_ABOVE: char = 'Ṧ';
    /// \u{1e67}: 'ṧ'
    pub const LATIN_SMALL_LETTER_S_WITH_CARON_AND_DOT_ABOVE: char = 'ṧ';
    /// \u{1e68}: 'Ṩ'
    pub const LATIN_CAPITAL_LETTER_S_WITH_DOT_BELOW_AND_DOT_ABOVE: char = 'Ṩ';
    /// \u{1e69}: 'ṩ'
    pub const LATIN_SMALL_LETTER_S_WITH_DOT_BELOW_AND_DOT_ABOVE: char = 'ṩ';
    /// \u{1e6a}: 'Ṫ'
    pub const LATIN_CAPITAL_LETTER_T_WITH_DOT_ABOVE: char = 'Ṫ';
    /// \u{1e6b}: 'ṫ'
    pub const LATIN_SMALL_LETTER_T_WITH_DOT_ABOVE: char = 'ṫ';
    /// \u{1e6c}: 'Ṭ'
    pub const LATIN_CAPITAL_LETTER_T_WITH_DOT_BELOW: char = 'Ṭ';
    /// \u{1e6d}: 'ṭ'
    pub const LATIN_SMALL_LETTER_T_WITH_DOT_BELOW: char = 'ṭ';
    /// \u{1e6e}: 'Ṯ'
    pub const LATIN_CAPITAL_LETTER_T_WITH_LINE_BELOW: char = 'Ṯ';
    /// \u{1e6f}: 'ṯ'
    pub const LATIN_SMALL_LETTER_T_WITH_LINE_BELOW: char = 'ṯ';
    /// \u{1e70}: 'Ṱ'
    pub const LATIN_CAPITAL_LETTER_T_WITH_CIRCUMFLEX_BELOW: char = 'Ṱ';
    /// \u{1e71}: 'ṱ'
    pub const LATIN_SMALL_LETTER_T_WITH_CIRCUMFLEX_BELOW: char = 'ṱ';
    /// \u{1e72}: 'Ṳ'
    pub const LATIN_CAPITAL_LETTER_U_WITH_DIAERESIS_BELOW: char = 'Ṳ';
    /// \u{1e73}: 'ṳ'
    pub const LATIN_SMALL_LETTER_U_WITH_DIAERESIS_BELOW: char = 'ṳ';
    /// \u{1e74}: 'Ṵ'
    pub const LATIN_CAPITAL_LETTER_U_WITH_TILDE_BELOW: char = 'Ṵ';
    /// \u{1e75}: 'ṵ'
    pub const LATIN_SMALL_LETTER_U_WITH_TILDE_BELOW: char = 'ṵ';
    /// \u{1e76}: 'Ṷ'
    pub const LATIN_CAPITAL_LETTER_U_WITH_CIRCUMFLEX_BELOW: char = 'Ṷ';
    /// \u{1e77}: 'ṷ'
    pub const LATIN_SMALL_LETTER_U_WITH_CIRCUMFLEX_BELOW: char = 'ṷ';
    /// \u{1e78}: 'Ṹ'
    pub const LATIN_CAPITAL_LETTER_U_WITH_TILDE_AND_ACUTE: char = 'Ṹ';
    /// \u{1e79}: 'ṹ'
    pub const LATIN_SMALL_LETTER_U_WITH_TILDE_AND_ACUTE: char = 'ṹ';
    /// \u{1e7a}: 'Ṻ'
    pub const LATIN_CAPITAL_LETTER_U_WITH_MACRON_AND_DIAERESIS: char = 'Ṻ';
    /// \u{1e7b}: 'ṻ'
    pub const LATIN_SMALL_LETTER_U_WITH_MACRON_AND_DIAERESIS: char = 'ṻ';
    /// \u{1e7c}: 'Ṽ'
    pub const LATIN_CAPITAL_LETTER_V_WITH_TILDE: char = 'Ṽ';
    /// \u{1e7d}: 'ṽ'
    pub const LATIN_SMALL_LETTER_V_WITH_TILDE: char = 'ṽ';
    /// \u{1e7e}: 'Ṿ'
    pub const LATIN_CAPITAL_LETTER_V_WITH_DOT_BELOW: char = 'Ṿ';
    /// \u{1e7f}: 'ṿ'
    pub const LATIN_SMALL_LETTER_V_WITH_DOT_BELOW: char = 'ṿ';
    /// \u{1e80}: 'Ẁ'
    pub const LATIN_CAPITAL_LETTER_W_WITH_GRAVE: char = 'Ẁ';
    /// \u{1e81}: 'ẁ'
    pub const LATIN_SMALL_LETTER_W_WITH_GRAVE: char = 'ẁ';
    /// \u{1e82}: 'Ẃ'
    pub const LATIN_CAPITAL_LETTER_W_WITH_ACUTE: char = 'Ẃ';
    /// \u{1e83}: 'ẃ'
    pub const LATIN_SMALL_LETTER_W_WITH_ACUTE: char = 'ẃ';
    /// \u{1e84}: 'Ẅ'
    pub const LATIN_CAPITAL_LETTER_W_WITH_DIAERESIS: char = 'Ẅ';
    /// \u{1e85}: 'ẅ'
    pub const LATIN_SMALL_LETTER_W_WITH_DIAERESIS: char = 'ẅ';
    /// \u{1e86}: 'Ẇ'
    pub const LATIN_CAPITAL_LETTER_W_WITH_DOT_ABOVE: char = 'Ẇ';
    /// \u{1e87}: 'ẇ'
    pub const LATIN_SMALL_LETTER_W_WITH_DOT_ABOVE: char = 'ẇ';
    /// \u{1e88}: 'Ẉ'
    pub const LATIN_CAPITAL_LETTER_W_WITH_DOT_BELOW: char = 'Ẉ';
    /// \u{1e89}: 'ẉ'
    pub const LATIN_SMALL_LETTER_W_WITH_DOT_BELOW: char = 'ẉ';
    /// \u{1e8a}: 'Ẋ'
    pub const LATIN_CAPITAL_LETTER_X_WITH_DOT_ABOVE: char = 'Ẋ';
    /// \u{1e8b}: 'ẋ'
    pub const LATIN_SMALL_LETTER_X_WITH_DOT_ABOVE: char = 'ẋ';
    /// \u{1e8c}: 'Ẍ'
    pub const LATIN_CAPITAL_LETTER_X_WITH_DIAERESIS: char = 'Ẍ';
    /// \u{1e8d}: 'ẍ'
    pub const LATIN_SMALL_LETTER_X_WITH_DIAERESIS: char = 'ẍ';
    /// \u{1e8e}: 'Ẏ'
    pub const LATIN_CAPITAL_LETTER_Y_WITH_DOT_ABOVE: char = 'Ẏ';
    /// \u{1e8f}: 'ẏ'
    pub const LATIN_SMALL_LETTER_Y_WITH_DOT_ABOVE: char = 'ẏ';
    /// \u{1e90}: 'Ẑ'
    pub const LATIN_CAPITAL_LETTER_Z_WITH_CIRCUMFLEX: char = 'Ẑ';
    /// \u{1e91}: 'ẑ'
    pub const LATIN_SMALL_LETTER_Z_WITH_CIRCUMFLEX: char = 'ẑ';
    /// \u{1e92}: 'Ẓ'
    pub const LATIN_CAPITAL_LETTER_Z_WITH_DOT_BELOW: char = 'Ẓ';
    /// \u{1e93}: 'ẓ'
    pub const LATIN_SMALL_LETTER_Z_WITH_DOT_BELOW: char = 'ẓ';
    /// \u{1e94}: 'Ẕ'
    pub const LATIN_CAPITAL_LETTER_Z_WITH_LINE_BELOW: char = 'Ẕ';
    /// \u{1e95}: 'ẕ'
    pub const LATIN_SMALL_LETTER_Z_WITH_LINE_BELOW: char = 'ẕ';
    /// \u{1e96}: 'ẖ'
    pub const LATIN_SMALL_LETTER_H_WITH_LINE_BELOW: char = 'ẖ';
    /// \u{1e97}: 'ẗ'
    pub const LATIN_SMALL_LETTER_T_WITH_DIAERESIS: char = 'ẗ';
    /// \u{1e98}: 'ẘ'
    pub const LATIN_SMALL_LETTER_W_WITH_RING_ABOVE: char = 'ẘ';
    /// \u{1e99}: 'ẙ'
    pub const LATIN_SMALL_LETTER_Y_WITH_RING_ABOVE: char = 'ẙ';
    /// \u{1e9a}: 'ẚ'
    pub const LATIN_SMALL_LETTER_A_WITH_RIGHT_HALF_RING: char = 'ẚ';
    /// \u{1e9b}: 'ẛ'
    pub const LATIN_SMALL_LETTER_LONG_S_WITH_DOT_ABOVE: char = 'ẛ';
    /// \u{1e9c}: 'ẜ'
    pub const LATIN_SMALL_LETTER_LONG_S_WITH_DIAGONAL_STROKE: char = 'ẜ';
    /// \u{1e9d}: 'ẝ'
    pub const LATIN_SMALL_LETTER_LONG_S_WITH_HIGH_STROKE: char = 'ẝ';
    /// \u{1e9e}: 'ẞ'
    pub const LATIN_CAPITAL_LETTER_SHARP_S: char = 'ẞ';
    /// \u{1e9f}: 'ẟ'
    pub const LATIN_SMALL_LETTER_DELTA: char = 'ẟ';
    /// \u{1ea0}: 'Ạ'
    pub const LATIN_CAPITAL_LETTER_A_WITH_DOT_BELOW: char = 'Ạ';
    /// \u{1ea1}: 'ạ'
    pub const LATIN_SMALL_LETTER_A_WITH_DOT_BELOW: char = 'ạ';
    /// \u{1ea2}: 'Ả'
    pub const LATIN_CAPITAL_LETTER_A_WITH_HOOK_ABOVE: char = 'Ả';
    /// \u{1ea3}: 'ả'
    pub const LATIN_SMALL_LETTER_A_WITH_HOOK_ABOVE: char = 'ả';
    /// \u{1ea4}: 'Ấ'
    pub const LATIN_CAPITAL_LETTER_A_WITH_CIRCUMFLEX_AND_ACUTE: char = 'Ấ';
    /// \u{1ea5}: 'ấ'
    pub const LATIN_SMALL_LETTER_A_WITH_CIRCUMFLEX_AND_ACUTE: char = 'ấ';
    /// \u{1ea6}: 'Ầ'
    pub const LATIN_CAPITAL_LETTER_A_WITH_CIRCUMFLEX_AND_GRAVE: char = 'Ầ';
    /// \u{1ea7}: 'ầ'
    pub const LATIN_SMALL_LETTER_A_WITH_CIRCUMFLEX_AND_GRAVE: char = 'ầ';
    /// \u{1ea8}: 'Ẩ'
    pub const LATIN_CAPITAL_LETTER_A_WITH_CIRCUMFLEX_AND_HOOK_ABOVE: char = 'Ẩ';
    /// \u{1ea9}: 'ẩ'
    pub const LATIN_SMALL_LETTER_A_WITH_CIRCUMFLEX_AND_HOOK_ABOVE: char = 'ẩ';
    /// \u{1eaa}: 'Ẫ'
    pub const LATIN_CAPITAL_LETTER_A_WITH_CIRCUMFLEX_AND_TILDE: char = 'Ẫ';
    /// \u{1eab}: 'ẫ'
    pub const LATIN_SMALL_LETTER_A_WITH_CIRCUMFLEX_AND_TILDE: char = 'ẫ';
    /// \u{1eac}: 'Ậ'
    pub const LATIN_CAPITAL_LETTER_A_WITH_CIRCUMFLEX_AND_DOT_BELOW: char = 'Ậ';
    /// \u{1ead}: 'ậ'
    pub const LATIN_SMALL_LETTER_A_WITH_CIRCUMFLEX_AND_DOT_BELOW: char = 'ậ';
    /// \u{1eae}: 'Ắ'
    pub const LATIN_CAPITAL_LETTER_A_WITH_BREVE_AND_ACUTE: char = 'Ắ';
    /// \u{1eaf}: 'ắ'
    pub const LATIN_SMALL_LETTER_A_WITH_BREVE_AND_ACUTE: char = 'ắ';
    /// \u{1eb0}: 'Ằ'
    pub const LATIN_CAPITAL_LETTER_A_WITH_BREVE_AND_GRAVE: char = 'Ằ';
    /// \u{1eb1}: 'ằ'
    pub const LATIN_SMALL_LETTER_A_WITH_BREVE_AND_GRAVE: char = 'ằ';
    /// \u{1eb2}: 'Ẳ'
    pub const LATIN_CAPITAL_LETTER_A_WITH_BREVE_AND_HOOK_ABOVE: char = 'Ẳ';
    /// \u{1eb3}: 'ẳ'
    pub const LATIN_SMALL_LETTER_A_WITH_BREVE_AND_HOOK_ABOVE: char = 'ẳ';
    /// \u{1eb4}: 'Ẵ'
    pub const LATIN_CAPITAL_LETTER_A_WITH_BREVE_AND_TILDE: char = 'Ẵ';
    /// \u{1eb5}: 'ẵ'
    pub const LATIN_SMALL_LETTER_A_WITH_BREVE_AND_TILDE: char = 'ẵ';
    /// \u{1eb6}: 'Ặ'
    pub const LATIN_CAPITAL_LETTER_A_WITH_BREVE_AND_DOT_BELOW: char = 'Ặ';
    /// \u{1eb7}: 'ặ'
    pub const LATIN_SMALL_LETTER_A_WITH_BREVE_AND_DOT_BELOW: char = 'ặ';
    /// \u{1eb8}: 'Ẹ'
    pub const LATIN_CAPITAL_LETTER_E_WITH_DOT_BELOW: char = 'Ẹ';
    /// \u{1eb9}: 'ẹ'
    pub const LATIN_SMALL_LETTER_E_WITH_DOT_BELOW: char = 'ẹ';
    /// \u{1eba}: 'Ẻ'
    pub const LATIN_CAPITAL_LETTER_E_WITH_HOOK_ABOVE: char = 'Ẻ';
    /// \u{1ebb}: 'ẻ'
    pub const LATIN_SMALL_LETTER_E_WITH_HOOK_ABOVE: char = 'ẻ';
    /// \u{1ebc}: 'Ẽ'
    pub const LATIN_CAPITAL_LETTER_E_WITH_TILDE: char = 'Ẽ';
    /// \u{1ebd}: 'ẽ'
    pub const LATIN_SMALL_LETTER_E_WITH_TILDE: char = 'ẽ';
    /// \u{1ebe}: 'Ế'
    pub const LATIN_CAPITAL_LETTER_E_WITH_CIRCUMFLEX_AND_ACUTE: char = 'Ế';
    /// \u{1ebf}: 'ế'
    pub const LATIN_SMALL_LETTER_E_WITH_CIRCUMFLEX_AND_ACUTE: char = 'ế';
    /// \u{1ec0}: 'Ề'
    pub const LATIN_CAPITAL_LETTER_E_WITH_CIRCUMFLEX_AND_GRAVE: char = 'Ề';
    /// \u{1ec1}: 'ề'
    pub const LATIN_SMALL_LETTER_E_WITH_CIRCUMFLEX_AND_GRAVE: char = 'ề';
    /// \u{1ec2}: 'Ể'
    pub const LATIN_CAPITAL_LETTER_E_WITH_CIRCUMFLEX_AND_HOOK_ABOVE: char = 'Ể';
    /// \u{1ec3}: 'ể'
    pub const LATIN_SMALL_LETTER_E_WITH_CIRCUMFLEX_AND_HOOK_ABOVE: char = 'ể';
    /// \u{1ec4}: 'Ễ'
    pub const LATIN_CAPITAL_LETTER_E_WITH_CIRCUMFLEX_AND_TILDE: char = 'Ễ';
    /// \u{1ec5}: 'ễ'
    pub const LATIN_SMALL_LETTER_E_WITH_CIRCUMFLEX_AND_TILDE: char = 'ễ';
    /// \u{1ec6}: 'Ệ'
    pub const LATIN_CAPITAL_LETTER_E_WITH_CIRCUMFLEX_AND_DOT_BELOW: char = 'Ệ';
    /// \u{1ec7}: 'ệ'
    pub const LATIN_SMALL_LETTER_E_WITH_CIRCUMFLEX_AND_DOT_BELOW: char = 'ệ';
    /// \u{1ec8}: 'Ỉ'
    pub const LATIN_CAPITAL_LETTER_I_WITH_HOOK_ABOVE: char = 'Ỉ';
    /// \u{1ec9}: 'ỉ'
    pub const LATIN_SMALL_LETTER_I_WITH_HOOK_ABOVE: char = 'ỉ';
    /// \u{1eca}: 'Ị'
    pub const LATIN_CAPITAL_LETTER_I_WITH_DOT_BELOW: char = 'Ị';
    /// \u{1ecb}: 'ị'
    pub const LATIN_SMALL_LETTER_I_WITH_DOT_BELOW: char = 'ị';
    /// \u{1ecc}: 'Ọ'
    pub const LATIN_CAPITAL_LETTER_O_WITH_DOT_BELOW: char = 'Ọ';
    /// \u{1ecd}: 'ọ'
    pub const LATIN_SMALL_LETTER_O_WITH_DOT_BELOW: char = 'ọ';
    /// \u{1ece}: 'Ỏ'
    pub const LATIN_CAPITAL_LETTER_O_WITH_HOOK_ABOVE: char = 'Ỏ';
    /// \u{1ecf}: 'ỏ'
    pub const LATIN_SMALL_LETTER_O_WITH_HOOK_ABOVE: char = 'ỏ';
    /// \u{1ed0}: 'Ố'
    pub const LATIN_CAPITAL_LETTER_O_WITH_CIRCUMFLEX_AND_ACUTE: char = 'Ố';
    /// \u{1ed1}: 'ố'
    pub const LATIN_SMALL_LETTER_O_WITH_CIRCUMFLEX_AND_ACUTE: char = 'ố';
    /// \u{1ed2}: 'Ồ'
    pub const LATIN_CAPITAL_LETTER_O_WITH_CIRCUMFLEX_AND_GRAVE: char = 'Ồ';
    /// \u{1ed3}: 'ồ'
    pub const LATIN_SMALL_LETTER_O_WITH_CIRCUMFLEX_AND_GRAVE: char = 'ồ';
    /// \u{1ed4}: 'Ổ'
    pub const LATIN_CAPITAL_LETTER_O_WITH_CIRCUMFLEX_AND_HOOK_ABOVE: char = 'Ổ';
    /// \u{1ed5}: 'ổ'
    pub const LATIN_SMALL_LETTER_O_WITH_CIRCUMFLEX_AND_HOOK_ABOVE: char = 'ổ';
    /// \u{1ed6}: 'Ỗ'
    pub const LATIN_CAPITAL_LETTER_O_WITH_CIRCUMFLEX_AND_TILDE: char = 'Ỗ';
    /// \u{1ed7}: 'ỗ'
    pub const LATIN_SMALL_LETTER_O_WITH_CIRCUMFLEX_AND_TILDE: char = 'ỗ';
    /// \u{1ed8}: 'Ộ'
    pub const LATIN_CAPITAL_LETTER_O_WITH_CIRCUMFLEX_AND_DOT_BELOW: char = 'Ộ';
    /// \u{1ed9}: 'ộ'
    pub const LATIN_SMALL_LETTER_O_WITH_CIRCUMFLEX_AND_DOT_BELOW: char = 'ộ';
    /// \u{1eda}: 'Ớ'
    pub const LATIN_CAPITAL_LETTER_O_WITH_HORN_AND_ACUTE: char = 'Ớ';
    /// \u{1edb}: 'ớ'
    pub const LATIN_SMALL_LETTER_O_WITH_HORN_AND_ACUTE: char = 'ớ';
    /// \u{1edc}: 'Ờ'
    pub const LATIN_CAPITAL_LETTER_O_WITH_HORN_AND_GRAVE: char = 'Ờ';
    /// \u{1edd}: 'ờ'
    pub const LATIN_SMALL_LETTER_O_WITH_HORN_AND_GRAVE: char = 'ờ';
    /// \u{1ede}: 'Ở'
    pub const LATIN_CAPITAL_LETTER_O_WITH_HORN_AND_HOOK_ABOVE: char = 'Ở';
    /// \u{1edf}: 'ở'
    pub const LATIN_SMALL_LETTER_O_WITH_HORN_AND_HOOK_ABOVE: char = 'ở';
    /// \u{1ee0}: 'Ỡ'
    pub const LATIN_CAPITAL_LETTER_O_WITH_HORN_AND_TILDE: char = 'Ỡ';
    /// \u{1ee1}: 'ỡ'
    pub const LATIN_SMALL_LETTER_O_WITH_HORN_AND_TILDE: char = 'ỡ';
    /// \u{1ee2}: 'Ợ'
    pub const LATIN_CAPITAL_LETTER_O_WITH_HORN_AND_DOT_BELOW: char = 'Ợ';
    /// \u{1ee3}: 'ợ'
    pub const LATIN_SMALL_LETTER_O_WITH_HORN_AND_DOT_BELOW: char = 'ợ';
    /// \u{1ee4}: 'Ụ'
    pub const LATIN_CAPITAL_LETTER_U_WITH_DOT_BELOW: char = 'Ụ';
    /// \u{1ee5}: 'ụ'
    pub const LATIN_SMALL_LETTER_U_WITH_DOT_BELOW: char = 'ụ';
    /// \u{1ee6}: 'Ủ'
    pub const LATIN_CAPITAL_LETTER_U_WITH_HOOK_ABOVE: char = 'Ủ';
    /// \u{1ee7}: 'ủ'
    pub const LATIN_SMALL_LETTER_U_WITH_HOOK_ABOVE: char = 'ủ';
    /// \u{1ee8}: 'Ứ'
    pub const LATIN_CAPITAL_LETTER_U_WITH_HORN_AND_ACUTE: char = 'Ứ';
    /// \u{1ee9}: 'ứ'
    pub const LATIN_SMALL_LETTER_U_WITH_HORN_AND_ACUTE: char = 'ứ';
    /// \u{1eea}: 'Ừ'
    pub const LATIN_CAPITAL_LETTER_U_WITH_HORN_AND_GRAVE: char = 'Ừ';
    /// \u{1eeb}: 'ừ'
    pub const LATIN_SMALL_LETTER_U_WITH_HORN_AND_GRAVE: char = 'ừ';
    /// \u{1eec}: 'Ử'
    pub const LATIN_CAPITAL_LETTER_U_WITH_HORN_AND_HOOK_ABOVE: char = 'Ử';
    /// \u{1eed}: 'ử'
    pub const LATIN_SMALL_LETTER_U_WITH_HORN_AND_HOOK_ABOVE: char = 'ử';
    /// \u{1eee}: 'Ữ'
    pub const LATIN_CAPITAL_LETTER_U_WITH_HORN_AND_TILDE: char = 'Ữ';
    /// \u{1eef}: 'ữ'
    pub const LATIN_SMALL_LETTER_U_WITH_HORN_AND_TILDE: char = 'ữ';
    /// \u{1ef0}: 'Ự'
    pub const LATIN_CAPITAL_LETTER_U_WITH_HORN_AND_DOT_BELOW: char = 'Ự';
    /// \u{1ef1}: 'ự'
    pub const LATIN_SMALL_LETTER_U_WITH_HORN_AND_DOT_BELOW: char = 'ự';
    /// \u{1ef2}: 'Ỳ'
    pub const LATIN_CAPITAL_LETTER_Y_WITH_GRAVE: char = 'Ỳ';
    /// \u{1ef3}: 'ỳ'
    pub const LATIN_SMALL_LETTER_Y_WITH_GRAVE: char = 'ỳ';
    /// \u{1ef4}: 'Ỵ'
    pub const LATIN_CAPITAL_LETTER_Y_WITH_DOT_BELOW: char = 'Ỵ';
    /// \u{1ef5}: 'ỵ'
    pub const LATIN_SMALL_LETTER_Y_WITH_DOT_BELOW: char = 'ỵ';
    /// \u{1ef6}: 'Ỷ'
    pub const LATIN_CAPITAL_LETTER_Y_WITH_HOOK_ABOVE: char = 'Ỷ';
    /// \u{1ef7}: 'ỷ'
    pub const LATIN_SMALL_LETTER_Y_WITH_HOOK_ABOVE: char = 'ỷ';
    /// \u{1ef8}: 'Ỹ'
    pub const LATIN_CAPITAL_LETTER_Y_WITH_TILDE: char = 'Ỹ';
    /// \u{1ef9}: 'ỹ'
    pub const LATIN_SMALL_LETTER_Y_WITH_TILDE: char = 'ỹ';
    /// \u{1efa}: 'Ỻ'
    pub const LATIN_CAPITAL_LETTER_MIDDLE_DASH_WELSH_LL: char = 'Ỻ';
    /// \u{1efb}: 'ỻ'
    pub const LATIN_SMALL_LETTER_MIDDLE_DASH_WELSH_LL: char = 'ỻ';
    /// \u{1efc}: 'Ỽ'
    pub const LATIN_CAPITAL_LETTER_MIDDLE_DASH_WELSH_V: char = 'Ỽ';
    /// \u{1efd}: 'ỽ'
    pub const LATIN_SMALL_LETTER_MIDDLE_DASH_WELSH_V: char = 'ỽ';
    /// \u{1efe}: 'Ỿ'
    pub const LATIN_CAPITAL_LETTER_Y_WITH_LOOP: char = 'Ỿ';
}

/// An enum to represent all characters in the LatinExtendedAdditional block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum LatinExtendedAdditional {
    /// \u{1e00}: 'Ḁ'
    LatinCapitalLetterAWithRingBelow,
    /// \u{1e01}: 'ḁ'
    LatinSmallLetterAWithRingBelow,
    /// \u{1e02}: 'Ḃ'
    LatinCapitalLetterBWithDotAbove,
    /// \u{1e03}: 'ḃ'
    LatinSmallLetterBWithDotAbove,
    /// \u{1e04}: 'Ḅ'
    LatinCapitalLetterBWithDotBelow,
    /// \u{1e05}: 'ḅ'
    LatinSmallLetterBWithDotBelow,
    /// \u{1e06}: 'Ḇ'
    LatinCapitalLetterBWithLineBelow,
    /// \u{1e07}: 'ḇ'
    LatinSmallLetterBWithLineBelow,
    /// \u{1e08}: 'Ḉ'
    LatinCapitalLetterCWithCedillaAndAcute,
    /// \u{1e09}: 'ḉ'
    LatinSmallLetterCWithCedillaAndAcute,
    /// \u{1e0a}: 'Ḋ'
    LatinCapitalLetterDWithDotAbove,
    /// \u{1e0b}: 'ḋ'
    LatinSmallLetterDWithDotAbove,
    /// \u{1e0c}: 'Ḍ'
    LatinCapitalLetterDWithDotBelow,
    /// \u{1e0d}: 'ḍ'
    LatinSmallLetterDWithDotBelow,
    /// \u{1e0e}: 'Ḏ'
    LatinCapitalLetterDWithLineBelow,
    /// \u{1e0f}: 'ḏ'
    LatinSmallLetterDWithLineBelow,
    /// \u{1e10}: 'Ḑ'
    LatinCapitalLetterDWithCedilla,
    /// \u{1e11}: 'ḑ'
    LatinSmallLetterDWithCedilla,
    /// \u{1e12}: 'Ḓ'
    LatinCapitalLetterDWithCircumflexBelow,
    /// \u{1e13}: 'ḓ'
    LatinSmallLetterDWithCircumflexBelow,
    /// \u{1e14}: 'Ḕ'
    LatinCapitalLetterEWithMacronAndGrave,
    /// \u{1e15}: 'ḕ'
    LatinSmallLetterEWithMacronAndGrave,
    /// \u{1e16}: 'Ḗ'
    LatinCapitalLetterEWithMacronAndAcute,
    /// \u{1e17}: 'ḗ'
    LatinSmallLetterEWithMacronAndAcute,
    /// \u{1e18}: 'Ḙ'
    LatinCapitalLetterEWithCircumflexBelow,
    /// \u{1e19}: 'ḙ'
    LatinSmallLetterEWithCircumflexBelow,
    /// \u{1e1a}: 'Ḛ'
    LatinCapitalLetterEWithTildeBelow,
    /// \u{1e1b}: 'ḛ'
    LatinSmallLetterEWithTildeBelow,
    /// \u{1e1c}: 'Ḝ'
    LatinCapitalLetterEWithCedillaAndBreve,
    /// \u{1e1d}: 'ḝ'
    LatinSmallLetterEWithCedillaAndBreve,
    /// \u{1e1e}: 'Ḟ'
    LatinCapitalLetterFWithDotAbove,
    /// \u{1e1f}: 'ḟ'
    LatinSmallLetterFWithDotAbove,
    /// \u{1e20}: 'Ḡ'
    LatinCapitalLetterGWithMacron,
    /// \u{1e21}: 'ḡ'
    LatinSmallLetterGWithMacron,
    /// \u{1e22}: 'Ḣ'
    LatinCapitalLetterHWithDotAbove,
    /// \u{1e23}: 'ḣ'
    LatinSmallLetterHWithDotAbove,
    /// \u{1e24}: 'Ḥ'
    LatinCapitalLetterHWithDotBelow,
    /// \u{1e25}: 'ḥ'
    LatinSmallLetterHWithDotBelow,
    /// \u{1e26}: 'Ḧ'
    LatinCapitalLetterHWithDiaeresis,
    /// \u{1e27}: 'ḧ'
    LatinSmallLetterHWithDiaeresis,
    /// \u{1e28}: 'Ḩ'
    LatinCapitalLetterHWithCedilla,
    /// \u{1e29}: 'ḩ'
    LatinSmallLetterHWithCedilla,
    /// \u{1e2a}: 'Ḫ'
    LatinCapitalLetterHWithBreveBelow,
    /// \u{1e2b}: 'ḫ'
    LatinSmallLetterHWithBreveBelow,
    /// \u{1e2c}: 'Ḭ'
    LatinCapitalLetterIWithTildeBelow,
    /// \u{1e2d}: 'ḭ'
    LatinSmallLetterIWithTildeBelow,
    /// \u{1e2e}: 'Ḯ'
    LatinCapitalLetterIWithDiaeresisAndAcute,
    /// \u{1e2f}: 'ḯ'
    LatinSmallLetterIWithDiaeresisAndAcute,
    /// \u{1e30}: 'Ḱ'
    LatinCapitalLetterKWithAcute,
    /// \u{1e31}: 'ḱ'
    LatinSmallLetterKWithAcute,
    /// \u{1e32}: 'Ḳ'
    LatinCapitalLetterKWithDotBelow,
    /// \u{1e33}: 'ḳ'
    LatinSmallLetterKWithDotBelow,
    /// \u{1e34}: 'Ḵ'
    LatinCapitalLetterKWithLineBelow,
    /// \u{1e35}: 'ḵ'
    LatinSmallLetterKWithLineBelow,
    /// \u{1e36}: 'Ḷ'
    LatinCapitalLetterLWithDotBelow,
    /// \u{1e37}: 'ḷ'
    LatinSmallLetterLWithDotBelow,
    /// \u{1e38}: 'Ḹ'
    LatinCapitalLetterLWithDotBelowAndMacron,
    /// \u{1e39}: 'ḹ'
    LatinSmallLetterLWithDotBelowAndMacron,
    /// \u{1e3a}: 'Ḻ'
    LatinCapitalLetterLWithLineBelow,
    /// \u{1e3b}: 'ḻ'
    LatinSmallLetterLWithLineBelow,
    /// \u{1e3c}: 'Ḽ'
    LatinCapitalLetterLWithCircumflexBelow,
    /// \u{1e3d}: 'ḽ'
    LatinSmallLetterLWithCircumflexBelow,
    /// \u{1e3e}: 'Ḿ'
    LatinCapitalLetterMWithAcute,
    /// \u{1e3f}: 'ḿ'
    LatinSmallLetterMWithAcute,
    /// \u{1e40}: 'Ṁ'
    LatinCapitalLetterMWithDotAbove,
    /// \u{1e41}: 'ṁ'
    LatinSmallLetterMWithDotAbove,
    /// \u{1e42}: 'Ṃ'
    LatinCapitalLetterMWithDotBelow,
    /// \u{1e43}: 'ṃ'
    LatinSmallLetterMWithDotBelow,
    /// \u{1e44}: 'Ṅ'
    LatinCapitalLetterNWithDotAbove,
    /// \u{1e45}: 'ṅ'
    LatinSmallLetterNWithDotAbove,
    /// \u{1e46}: 'Ṇ'
    LatinCapitalLetterNWithDotBelow,
    /// \u{1e47}: 'ṇ'
    LatinSmallLetterNWithDotBelow,
    /// \u{1e48}: 'Ṉ'
    LatinCapitalLetterNWithLineBelow,
    /// \u{1e49}: 'ṉ'
    LatinSmallLetterNWithLineBelow,
    /// \u{1e4a}: 'Ṋ'
    LatinCapitalLetterNWithCircumflexBelow,
    /// \u{1e4b}: 'ṋ'
    LatinSmallLetterNWithCircumflexBelow,
    /// \u{1e4c}: 'Ṍ'
    LatinCapitalLetterOWithTildeAndAcute,
    /// \u{1e4d}: 'ṍ'
    LatinSmallLetterOWithTildeAndAcute,
    /// \u{1e4e}: 'Ṏ'
    LatinCapitalLetterOWithTildeAndDiaeresis,
    /// \u{1e4f}: 'ṏ'
    LatinSmallLetterOWithTildeAndDiaeresis,
    /// \u{1e50}: 'Ṑ'
    LatinCapitalLetterOWithMacronAndGrave,
    /// \u{1e51}: 'ṑ'
    LatinSmallLetterOWithMacronAndGrave,
    /// \u{1e52}: 'Ṓ'
    LatinCapitalLetterOWithMacronAndAcute,
    /// \u{1e53}: 'ṓ'
    LatinSmallLetterOWithMacronAndAcute,
    /// \u{1e54}: 'Ṕ'
    LatinCapitalLetterPWithAcute,
    /// \u{1e55}: 'ṕ'
    LatinSmallLetterPWithAcute,
    /// \u{1e56}: 'Ṗ'
    LatinCapitalLetterPWithDotAbove,
    /// \u{1e57}: 'ṗ'
    LatinSmallLetterPWithDotAbove,
    /// \u{1e58}: 'Ṙ'
    LatinCapitalLetterRWithDotAbove,
    /// \u{1e59}: 'ṙ'
    LatinSmallLetterRWithDotAbove,
    /// \u{1e5a}: 'Ṛ'
    LatinCapitalLetterRWithDotBelow,
    /// \u{1e5b}: 'ṛ'
    LatinSmallLetterRWithDotBelow,
    /// \u{1e5c}: 'Ṝ'
    LatinCapitalLetterRWithDotBelowAndMacron,
    /// \u{1e5d}: 'ṝ'
    LatinSmallLetterRWithDotBelowAndMacron,
    /// \u{1e5e}: 'Ṟ'
    LatinCapitalLetterRWithLineBelow,
    /// \u{1e5f}: 'ṟ'
    LatinSmallLetterRWithLineBelow,
    /// \u{1e60}: 'Ṡ'
    LatinCapitalLetterSWithDotAbove,
    /// \u{1e61}: 'ṡ'
    LatinSmallLetterSWithDotAbove,
    /// \u{1e62}: 'Ṣ'
    LatinCapitalLetterSWithDotBelow,
    /// \u{1e63}: 'ṣ'
    LatinSmallLetterSWithDotBelow,
    /// \u{1e64}: 'Ṥ'
    LatinCapitalLetterSWithAcuteAndDotAbove,
    /// \u{1e65}: 'ṥ'
    LatinSmallLetterSWithAcuteAndDotAbove,
    /// \u{1e66}: 'Ṧ'
    LatinCapitalLetterSWithCaronAndDotAbove,
    /// \u{1e67}: 'ṧ'
    LatinSmallLetterSWithCaronAndDotAbove,
    /// \u{1e68}: 'Ṩ'
    LatinCapitalLetterSWithDotBelowAndDotAbove,
    /// \u{1e69}: 'ṩ'
    LatinSmallLetterSWithDotBelowAndDotAbove,
    /// \u{1e6a}: 'Ṫ'
    LatinCapitalLetterTWithDotAbove,
    /// \u{1e6b}: 'ṫ'
    LatinSmallLetterTWithDotAbove,
    /// \u{1e6c}: 'Ṭ'
    LatinCapitalLetterTWithDotBelow,
    /// \u{1e6d}: 'ṭ'
    LatinSmallLetterTWithDotBelow,
    /// \u{1e6e}: 'Ṯ'
    LatinCapitalLetterTWithLineBelow,
    /// \u{1e6f}: 'ṯ'
    LatinSmallLetterTWithLineBelow,
    /// \u{1e70}: 'Ṱ'
    LatinCapitalLetterTWithCircumflexBelow,
    /// \u{1e71}: 'ṱ'
    LatinSmallLetterTWithCircumflexBelow,
    /// \u{1e72}: 'Ṳ'
    LatinCapitalLetterUWithDiaeresisBelow,
    /// \u{1e73}: 'ṳ'
    LatinSmallLetterUWithDiaeresisBelow,
    /// \u{1e74}: 'Ṵ'
    LatinCapitalLetterUWithTildeBelow,
    /// \u{1e75}: 'ṵ'
    LatinSmallLetterUWithTildeBelow,
    /// \u{1e76}: 'Ṷ'
    LatinCapitalLetterUWithCircumflexBelow,
    /// \u{1e77}: 'ṷ'
    LatinSmallLetterUWithCircumflexBelow,
    /// \u{1e78}: 'Ṹ'
    LatinCapitalLetterUWithTildeAndAcute,
    /// \u{1e79}: 'ṹ'
    LatinSmallLetterUWithTildeAndAcute,
    /// \u{1e7a}: 'Ṻ'
    LatinCapitalLetterUWithMacronAndDiaeresis,
    /// \u{1e7b}: 'ṻ'
    LatinSmallLetterUWithMacronAndDiaeresis,
    /// \u{1e7c}: 'Ṽ'
    LatinCapitalLetterVWithTilde,
    /// \u{1e7d}: 'ṽ'
    LatinSmallLetterVWithTilde,
    /// \u{1e7e}: 'Ṿ'
    LatinCapitalLetterVWithDotBelow,
    /// \u{1e7f}: 'ṿ'
    LatinSmallLetterVWithDotBelow,
    /// \u{1e80}: 'Ẁ'
    LatinCapitalLetterWWithGrave,
    /// \u{1e81}: 'ẁ'
    LatinSmallLetterWWithGrave,
    /// \u{1e82}: 'Ẃ'
    LatinCapitalLetterWWithAcute,
    /// \u{1e83}: 'ẃ'
    LatinSmallLetterWWithAcute,
    /// \u{1e84}: 'Ẅ'
    LatinCapitalLetterWWithDiaeresis,
    /// \u{1e85}: 'ẅ'
    LatinSmallLetterWWithDiaeresis,
    /// \u{1e86}: 'Ẇ'
    LatinCapitalLetterWWithDotAbove,
    /// \u{1e87}: 'ẇ'
    LatinSmallLetterWWithDotAbove,
    /// \u{1e88}: 'Ẉ'
    LatinCapitalLetterWWithDotBelow,
    /// \u{1e89}: 'ẉ'
    LatinSmallLetterWWithDotBelow,
    /// \u{1e8a}: 'Ẋ'
    LatinCapitalLetterXWithDotAbove,
    /// \u{1e8b}: 'ẋ'
    LatinSmallLetterXWithDotAbove,
    /// \u{1e8c}: 'Ẍ'
    LatinCapitalLetterXWithDiaeresis,
    /// \u{1e8d}: 'ẍ'
    LatinSmallLetterXWithDiaeresis,
    /// \u{1e8e}: 'Ẏ'
    LatinCapitalLetterYWithDotAbove,
    /// \u{1e8f}: 'ẏ'
    LatinSmallLetterYWithDotAbove,
    /// \u{1e90}: 'Ẑ'
    LatinCapitalLetterZWithCircumflex,
    /// \u{1e91}: 'ẑ'
    LatinSmallLetterZWithCircumflex,
    /// \u{1e92}: 'Ẓ'
    LatinCapitalLetterZWithDotBelow,
    /// \u{1e93}: 'ẓ'
    LatinSmallLetterZWithDotBelow,
    /// \u{1e94}: 'Ẕ'
    LatinCapitalLetterZWithLineBelow,
    /// \u{1e95}: 'ẕ'
    LatinSmallLetterZWithLineBelow,
    /// \u{1e96}: 'ẖ'
    LatinSmallLetterHWithLineBelow,
    /// \u{1e97}: 'ẗ'
    LatinSmallLetterTWithDiaeresis,
    /// \u{1e98}: 'ẘ'
    LatinSmallLetterWWithRingAbove,
    /// \u{1e99}: 'ẙ'
    LatinSmallLetterYWithRingAbove,
    /// \u{1e9a}: 'ẚ'
    LatinSmallLetterAWithRightHalfRing,
    /// \u{1e9b}: 'ẛ'
    LatinSmallLetterLongSWithDotAbove,
    /// \u{1e9c}: 'ẜ'
    LatinSmallLetterLongSWithDiagonalStroke,
    /// \u{1e9d}: 'ẝ'
    LatinSmallLetterLongSWithHighStroke,
    /// \u{1e9e}: 'ẞ'
    LatinCapitalLetterSharpS,
    /// \u{1e9f}: 'ẟ'
    LatinSmallLetterDelta,
    /// \u{1ea0}: 'Ạ'
    LatinCapitalLetterAWithDotBelow,
    /// \u{1ea1}: 'ạ'
    LatinSmallLetterAWithDotBelow,
    /// \u{1ea2}: 'Ả'
    LatinCapitalLetterAWithHookAbove,
    /// \u{1ea3}: 'ả'
    LatinSmallLetterAWithHookAbove,
    /// \u{1ea4}: 'Ấ'
    LatinCapitalLetterAWithCircumflexAndAcute,
    /// \u{1ea5}: 'ấ'
    LatinSmallLetterAWithCircumflexAndAcute,
    /// \u{1ea6}: 'Ầ'
    LatinCapitalLetterAWithCircumflexAndGrave,
    /// \u{1ea7}: 'ầ'
    LatinSmallLetterAWithCircumflexAndGrave,
    /// \u{1ea8}: 'Ẩ'
    LatinCapitalLetterAWithCircumflexAndHookAbove,
    /// \u{1ea9}: 'ẩ'
    LatinSmallLetterAWithCircumflexAndHookAbove,
    /// \u{1eaa}: 'Ẫ'
    LatinCapitalLetterAWithCircumflexAndTilde,
    /// \u{1eab}: 'ẫ'
    LatinSmallLetterAWithCircumflexAndTilde,
    /// \u{1eac}: 'Ậ'
    LatinCapitalLetterAWithCircumflexAndDotBelow,
    /// \u{1ead}: 'ậ'
    LatinSmallLetterAWithCircumflexAndDotBelow,
    /// \u{1eae}: 'Ắ'
    LatinCapitalLetterAWithBreveAndAcute,
    /// \u{1eaf}: 'ắ'
    LatinSmallLetterAWithBreveAndAcute,
    /// \u{1eb0}: 'Ằ'
    LatinCapitalLetterAWithBreveAndGrave,
    /// \u{1eb1}: 'ằ'
    LatinSmallLetterAWithBreveAndGrave,
    /// \u{1eb2}: 'Ẳ'
    LatinCapitalLetterAWithBreveAndHookAbove,
    /// \u{1eb3}: 'ẳ'
    LatinSmallLetterAWithBreveAndHookAbove,
    /// \u{1eb4}: 'Ẵ'
    LatinCapitalLetterAWithBreveAndTilde,
    /// \u{1eb5}: 'ẵ'
    LatinSmallLetterAWithBreveAndTilde,
    /// \u{1eb6}: 'Ặ'
    LatinCapitalLetterAWithBreveAndDotBelow,
    /// \u{1eb7}: 'ặ'
    LatinSmallLetterAWithBreveAndDotBelow,
    /// \u{1eb8}: 'Ẹ'
    LatinCapitalLetterEWithDotBelow,
    /// \u{1eb9}: 'ẹ'
    LatinSmallLetterEWithDotBelow,
    /// \u{1eba}: 'Ẻ'
    LatinCapitalLetterEWithHookAbove,
    /// \u{1ebb}: 'ẻ'
    LatinSmallLetterEWithHookAbove,
    /// \u{1ebc}: 'Ẽ'
    LatinCapitalLetterEWithTilde,
    /// \u{1ebd}: 'ẽ'
    LatinSmallLetterEWithTilde,
    /// \u{1ebe}: 'Ế'
    LatinCapitalLetterEWithCircumflexAndAcute,
    /// \u{1ebf}: 'ế'
    LatinSmallLetterEWithCircumflexAndAcute,
    /// \u{1ec0}: 'Ề'
    LatinCapitalLetterEWithCircumflexAndGrave,
    /// \u{1ec1}: 'ề'
    LatinSmallLetterEWithCircumflexAndGrave,
    /// \u{1ec2}: 'Ể'
    LatinCapitalLetterEWithCircumflexAndHookAbove,
    /// \u{1ec3}: 'ể'
    LatinSmallLetterEWithCircumflexAndHookAbove,
    /// \u{1ec4}: 'Ễ'
    LatinCapitalLetterEWithCircumflexAndTilde,
    /// \u{1ec5}: 'ễ'
    LatinSmallLetterEWithCircumflexAndTilde,
    /// \u{1ec6}: 'Ệ'
    LatinCapitalLetterEWithCircumflexAndDotBelow,
    /// \u{1ec7}: 'ệ'
    LatinSmallLetterEWithCircumflexAndDotBelow,
    /// \u{1ec8}: 'Ỉ'
    LatinCapitalLetterIWithHookAbove,
    /// \u{1ec9}: 'ỉ'
    LatinSmallLetterIWithHookAbove,
    /// \u{1eca}: 'Ị'
    LatinCapitalLetterIWithDotBelow,
    /// \u{1ecb}: 'ị'
    LatinSmallLetterIWithDotBelow,
    /// \u{1ecc}: 'Ọ'
    LatinCapitalLetterOWithDotBelow,
    /// \u{1ecd}: 'ọ'
    LatinSmallLetterOWithDotBelow,
    /// \u{1ece}: 'Ỏ'
    LatinCapitalLetterOWithHookAbove,
    /// \u{1ecf}: 'ỏ'
    LatinSmallLetterOWithHookAbove,
    /// \u{1ed0}: 'Ố'
    LatinCapitalLetterOWithCircumflexAndAcute,
    /// \u{1ed1}: 'ố'
    LatinSmallLetterOWithCircumflexAndAcute,
    /// \u{1ed2}: 'Ồ'
    LatinCapitalLetterOWithCircumflexAndGrave,
    /// \u{1ed3}: 'ồ'
    LatinSmallLetterOWithCircumflexAndGrave,
    /// \u{1ed4}: 'Ổ'
    LatinCapitalLetterOWithCircumflexAndHookAbove,
    /// \u{1ed5}: 'ổ'
    LatinSmallLetterOWithCircumflexAndHookAbove,
    /// \u{1ed6}: 'Ỗ'
    LatinCapitalLetterOWithCircumflexAndTilde,
    /// \u{1ed7}: 'ỗ'
    LatinSmallLetterOWithCircumflexAndTilde,
    /// \u{1ed8}: 'Ộ'
    LatinCapitalLetterOWithCircumflexAndDotBelow,
    /// \u{1ed9}: 'ộ'
    LatinSmallLetterOWithCircumflexAndDotBelow,
    /// \u{1eda}: 'Ớ'
    LatinCapitalLetterOWithHornAndAcute,
    /// \u{1edb}: 'ớ'
    LatinSmallLetterOWithHornAndAcute,
    /// \u{1edc}: 'Ờ'
    LatinCapitalLetterOWithHornAndGrave,
    /// \u{1edd}: 'ờ'
    LatinSmallLetterOWithHornAndGrave,
    /// \u{1ede}: 'Ở'
    LatinCapitalLetterOWithHornAndHookAbove,
    /// \u{1edf}: 'ở'
    LatinSmallLetterOWithHornAndHookAbove,
    /// \u{1ee0}: 'Ỡ'
    LatinCapitalLetterOWithHornAndTilde,
    /// \u{1ee1}: 'ỡ'
    LatinSmallLetterOWithHornAndTilde,
    /// \u{1ee2}: 'Ợ'
    LatinCapitalLetterOWithHornAndDotBelow,
    /// \u{1ee3}: 'ợ'
    LatinSmallLetterOWithHornAndDotBelow,
    /// \u{1ee4}: 'Ụ'
    LatinCapitalLetterUWithDotBelow,
    /// \u{1ee5}: 'ụ'
    LatinSmallLetterUWithDotBelow,
    /// \u{1ee6}: 'Ủ'
    LatinCapitalLetterUWithHookAbove,
    /// \u{1ee7}: 'ủ'
    LatinSmallLetterUWithHookAbove,
    /// \u{1ee8}: 'Ứ'
    LatinCapitalLetterUWithHornAndAcute,
    /// \u{1ee9}: 'ứ'
    LatinSmallLetterUWithHornAndAcute,
    /// \u{1eea}: 'Ừ'
    LatinCapitalLetterUWithHornAndGrave,
    /// \u{1eeb}: 'ừ'
    LatinSmallLetterUWithHornAndGrave,
    /// \u{1eec}: 'Ử'
    LatinCapitalLetterUWithHornAndHookAbove,
    /// \u{1eed}: 'ử'
    LatinSmallLetterUWithHornAndHookAbove,
    /// \u{1eee}: 'Ữ'
    LatinCapitalLetterUWithHornAndTilde,
    /// \u{1eef}: 'ữ'
    LatinSmallLetterUWithHornAndTilde,
    /// \u{1ef0}: 'Ự'
    LatinCapitalLetterUWithHornAndDotBelow,
    /// \u{1ef1}: 'ự'
    LatinSmallLetterUWithHornAndDotBelow,
    /// \u{1ef2}: 'Ỳ'
    LatinCapitalLetterYWithGrave,
    /// \u{1ef3}: 'ỳ'
    LatinSmallLetterYWithGrave,
    /// \u{1ef4}: 'Ỵ'
    LatinCapitalLetterYWithDotBelow,
    /// \u{1ef5}: 'ỵ'
    LatinSmallLetterYWithDotBelow,
    /// \u{1ef6}: 'Ỷ'
    LatinCapitalLetterYWithHookAbove,
    /// \u{1ef7}: 'ỷ'
    LatinSmallLetterYWithHookAbove,
    /// \u{1ef8}: 'Ỹ'
    LatinCapitalLetterYWithTilde,
    /// \u{1ef9}: 'ỹ'
    LatinSmallLetterYWithTilde,
    /// \u{1efa}: 'Ỻ'
    LatinCapitalLetterMiddleDashWelshLl,
    /// \u{1efb}: 'ỻ'
    LatinSmallLetterMiddleDashWelshLl,
    /// \u{1efc}: 'Ỽ'
    LatinCapitalLetterMiddleDashWelshV,
    /// \u{1efd}: 'ỽ'
    LatinSmallLetterMiddleDashWelshV,
    /// \u{1efe}: 'Ỿ'
    LatinCapitalLetterYWithLoop,
}

impl Into<char> for LatinExtendedAdditional {
    fn into(self) -> char {
        use constants::*;
        match self {
            LatinExtendedAdditional::LatinCapitalLetterAWithRingBelow => LATIN_CAPITAL_LETTER_A_WITH_RING_BELOW,
            LatinExtendedAdditional::LatinSmallLetterAWithRingBelow => LATIN_SMALL_LETTER_A_WITH_RING_BELOW,
            LatinExtendedAdditional::LatinCapitalLetterBWithDotAbove => LATIN_CAPITAL_LETTER_B_WITH_DOT_ABOVE,
            LatinExtendedAdditional::LatinSmallLetterBWithDotAbove => LATIN_SMALL_LETTER_B_WITH_DOT_ABOVE,
            LatinExtendedAdditional::LatinCapitalLetterBWithDotBelow => LATIN_CAPITAL_LETTER_B_WITH_DOT_BELOW,
            LatinExtendedAdditional::LatinSmallLetterBWithDotBelow => LATIN_SMALL_LETTER_B_WITH_DOT_BELOW,
            LatinExtendedAdditional::LatinCapitalLetterBWithLineBelow => LATIN_CAPITAL_LETTER_B_WITH_LINE_BELOW,
            LatinExtendedAdditional::LatinSmallLetterBWithLineBelow => LATIN_SMALL_LETTER_B_WITH_LINE_BELOW,
            LatinExtendedAdditional::LatinCapitalLetterCWithCedillaAndAcute => LATIN_CAPITAL_LETTER_C_WITH_CEDILLA_AND_ACUTE,
            LatinExtendedAdditional::LatinSmallLetterCWithCedillaAndAcute => LATIN_SMALL_LETTER_C_WITH_CEDILLA_AND_ACUTE,
            LatinExtendedAdditional::LatinCapitalLetterDWithDotAbove => LATIN_CAPITAL_LETTER_D_WITH_DOT_ABOVE,
            LatinExtendedAdditional::LatinSmallLetterDWithDotAbove => LATIN_SMALL_LETTER_D_WITH_DOT_ABOVE,
            LatinExtendedAdditional::LatinCapitalLetterDWithDotBelow => LATIN_CAPITAL_LETTER_D_WITH_DOT_BELOW,
            LatinExtendedAdditional::LatinSmallLetterDWithDotBelow => LATIN_SMALL_LETTER_D_WITH_DOT_BELOW,
            LatinExtendedAdditional::LatinCapitalLetterDWithLineBelow => LATIN_CAPITAL_LETTER_D_WITH_LINE_BELOW,
            LatinExtendedAdditional::LatinSmallLetterDWithLineBelow => LATIN_SMALL_LETTER_D_WITH_LINE_BELOW,
            LatinExtendedAdditional::LatinCapitalLetterDWithCedilla => LATIN_CAPITAL_LETTER_D_WITH_CEDILLA,
            LatinExtendedAdditional::LatinSmallLetterDWithCedilla => LATIN_SMALL_LETTER_D_WITH_CEDILLA,
            LatinExtendedAdditional::LatinCapitalLetterDWithCircumflexBelow => LATIN_CAPITAL_LETTER_D_WITH_CIRCUMFLEX_BELOW,
            LatinExtendedAdditional::LatinSmallLetterDWithCircumflexBelow => LATIN_SMALL_LETTER_D_WITH_CIRCUMFLEX_BELOW,
            LatinExtendedAdditional::LatinCapitalLetterEWithMacronAndGrave => LATIN_CAPITAL_LETTER_E_WITH_MACRON_AND_GRAVE,
            LatinExtendedAdditional::LatinSmallLetterEWithMacronAndGrave => LATIN_SMALL_LETTER_E_WITH_MACRON_AND_GRAVE,
            LatinExtendedAdditional::LatinCapitalLetterEWithMacronAndAcute => LATIN_CAPITAL_LETTER_E_WITH_MACRON_AND_ACUTE,
            LatinExtendedAdditional::LatinSmallLetterEWithMacronAndAcute => LATIN_SMALL_LETTER_E_WITH_MACRON_AND_ACUTE,
            LatinExtendedAdditional::LatinCapitalLetterEWithCircumflexBelow => LATIN_CAPITAL_LETTER_E_WITH_CIRCUMFLEX_BELOW,
            LatinExtendedAdditional::LatinSmallLetterEWithCircumflexBelow => LATIN_SMALL_LETTER_E_WITH_CIRCUMFLEX_BELOW,
            LatinExtendedAdditional::LatinCapitalLetterEWithTildeBelow => LATIN_CAPITAL_LETTER_E_WITH_TILDE_BELOW,
            LatinExtendedAdditional::LatinSmallLetterEWithTildeBelow => LATIN_SMALL_LETTER_E_WITH_TILDE_BELOW,
            LatinExtendedAdditional::LatinCapitalLetterEWithCedillaAndBreve => LATIN_CAPITAL_LETTER_E_WITH_CEDILLA_AND_BREVE,
            LatinExtendedAdditional::LatinSmallLetterEWithCedillaAndBreve => LATIN_SMALL_LETTER_E_WITH_CEDILLA_AND_BREVE,
            LatinExtendedAdditional::LatinCapitalLetterFWithDotAbove => LATIN_CAPITAL_LETTER_F_WITH_DOT_ABOVE,
            LatinExtendedAdditional::LatinSmallLetterFWithDotAbove => LATIN_SMALL_LETTER_F_WITH_DOT_ABOVE,
            LatinExtendedAdditional::LatinCapitalLetterGWithMacron => LATIN_CAPITAL_LETTER_G_WITH_MACRON,
            LatinExtendedAdditional::LatinSmallLetterGWithMacron => LATIN_SMALL_LETTER_G_WITH_MACRON,
            LatinExtendedAdditional::LatinCapitalLetterHWithDotAbove => LATIN_CAPITAL_LETTER_H_WITH_DOT_ABOVE,
            LatinExtendedAdditional::LatinSmallLetterHWithDotAbove => LATIN_SMALL_LETTER_H_WITH_DOT_ABOVE,
            LatinExtendedAdditional::LatinCapitalLetterHWithDotBelow => LATIN_CAPITAL_LETTER_H_WITH_DOT_BELOW,
            LatinExtendedAdditional::LatinSmallLetterHWithDotBelow => LATIN_SMALL_LETTER_H_WITH_DOT_BELOW,
            LatinExtendedAdditional::LatinCapitalLetterHWithDiaeresis => LATIN_CAPITAL_LETTER_H_WITH_DIAERESIS,
            LatinExtendedAdditional::LatinSmallLetterHWithDiaeresis => LATIN_SMALL_LETTER_H_WITH_DIAERESIS,
            LatinExtendedAdditional::LatinCapitalLetterHWithCedilla => LATIN_CAPITAL_LETTER_H_WITH_CEDILLA,
            LatinExtendedAdditional::LatinSmallLetterHWithCedilla => LATIN_SMALL_LETTER_H_WITH_CEDILLA,
            LatinExtendedAdditional::LatinCapitalLetterHWithBreveBelow => LATIN_CAPITAL_LETTER_H_WITH_BREVE_BELOW,
            LatinExtendedAdditional::LatinSmallLetterHWithBreveBelow => LATIN_SMALL_LETTER_H_WITH_BREVE_BELOW,
            LatinExtendedAdditional::LatinCapitalLetterIWithTildeBelow => LATIN_CAPITAL_LETTER_I_WITH_TILDE_BELOW,
            LatinExtendedAdditional::LatinSmallLetterIWithTildeBelow => LATIN_SMALL_LETTER_I_WITH_TILDE_BELOW,
            LatinExtendedAdditional::LatinCapitalLetterIWithDiaeresisAndAcute => LATIN_CAPITAL_LETTER_I_WITH_DIAERESIS_AND_ACUTE,
            LatinExtendedAdditional::LatinSmallLetterIWithDiaeresisAndAcute => LATIN_SMALL_LETTER_I_WITH_DIAERESIS_AND_ACUTE,
            LatinExtendedAdditional::LatinCapitalLetterKWithAcute => LATIN_CAPITAL_LETTER_K_WITH_ACUTE,
            LatinExtendedAdditional::LatinSmallLetterKWithAcute => LATIN_SMALL_LETTER_K_WITH_ACUTE,
            LatinExtendedAdditional::LatinCapitalLetterKWithDotBelow => LATIN_CAPITAL_LETTER_K_WITH_DOT_BELOW,
            LatinExtendedAdditional::LatinSmallLetterKWithDotBelow => LATIN_SMALL_LETTER_K_WITH_DOT_BELOW,
            LatinExtendedAdditional::LatinCapitalLetterKWithLineBelow => LATIN_CAPITAL_LETTER_K_WITH_LINE_BELOW,
            LatinExtendedAdditional::LatinSmallLetterKWithLineBelow => LATIN_SMALL_LETTER_K_WITH_LINE_BELOW,
            LatinExtendedAdditional::LatinCapitalLetterLWithDotBelow => LATIN_CAPITAL_LETTER_L_WITH_DOT_BELOW,
            LatinExtendedAdditional::LatinSmallLetterLWithDotBelow => LATIN_SMALL_LETTER_L_WITH_DOT_BELOW,
            LatinExtendedAdditional::LatinCapitalLetterLWithDotBelowAndMacron => LATIN_CAPITAL_LETTER_L_WITH_DOT_BELOW_AND_MACRON,
            LatinExtendedAdditional::LatinSmallLetterLWithDotBelowAndMacron => LATIN_SMALL_LETTER_L_WITH_DOT_BELOW_AND_MACRON,
            LatinExtendedAdditional::LatinCapitalLetterLWithLineBelow => LATIN_CAPITAL_LETTER_L_WITH_LINE_BELOW,
            LatinExtendedAdditional::LatinSmallLetterLWithLineBelow => LATIN_SMALL_LETTER_L_WITH_LINE_BELOW,
            LatinExtendedAdditional::LatinCapitalLetterLWithCircumflexBelow => LATIN_CAPITAL_LETTER_L_WITH_CIRCUMFLEX_BELOW,
            LatinExtendedAdditional::LatinSmallLetterLWithCircumflexBelow => LATIN_SMALL_LETTER_L_WITH_CIRCUMFLEX_BELOW,
            LatinExtendedAdditional::LatinCapitalLetterMWithAcute => LATIN_CAPITAL_LETTER_M_WITH_ACUTE,
            LatinExtendedAdditional::LatinSmallLetterMWithAcute => LATIN_SMALL_LETTER_M_WITH_ACUTE,
            LatinExtendedAdditional::LatinCapitalLetterMWithDotAbove => LATIN_CAPITAL_LETTER_M_WITH_DOT_ABOVE,
            LatinExtendedAdditional::LatinSmallLetterMWithDotAbove => LATIN_SMALL_LETTER_M_WITH_DOT_ABOVE,
            LatinExtendedAdditional::LatinCapitalLetterMWithDotBelow => LATIN_CAPITAL_LETTER_M_WITH_DOT_BELOW,
            LatinExtendedAdditional::LatinSmallLetterMWithDotBelow => LATIN_SMALL_LETTER_M_WITH_DOT_BELOW,
            LatinExtendedAdditional::LatinCapitalLetterNWithDotAbove => LATIN_CAPITAL_LETTER_N_WITH_DOT_ABOVE,
            LatinExtendedAdditional::LatinSmallLetterNWithDotAbove => LATIN_SMALL_LETTER_N_WITH_DOT_ABOVE,
            LatinExtendedAdditional::LatinCapitalLetterNWithDotBelow => LATIN_CAPITAL_LETTER_N_WITH_DOT_BELOW,
            LatinExtendedAdditional::LatinSmallLetterNWithDotBelow => LATIN_SMALL_LETTER_N_WITH_DOT_BELOW,
            LatinExtendedAdditional::LatinCapitalLetterNWithLineBelow => LATIN_CAPITAL_LETTER_N_WITH_LINE_BELOW,
            LatinExtendedAdditional::LatinSmallLetterNWithLineBelow => LATIN_SMALL_LETTER_N_WITH_LINE_BELOW,
            LatinExtendedAdditional::LatinCapitalLetterNWithCircumflexBelow => LATIN_CAPITAL_LETTER_N_WITH_CIRCUMFLEX_BELOW,
            LatinExtendedAdditional::LatinSmallLetterNWithCircumflexBelow => LATIN_SMALL_LETTER_N_WITH_CIRCUMFLEX_BELOW,
            LatinExtendedAdditional::LatinCapitalLetterOWithTildeAndAcute => LATIN_CAPITAL_LETTER_O_WITH_TILDE_AND_ACUTE,
            LatinExtendedAdditional::LatinSmallLetterOWithTildeAndAcute => LATIN_SMALL_LETTER_O_WITH_TILDE_AND_ACUTE,
            LatinExtendedAdditional::LatinCapitalLetterOWithTildeAndDiaeresis => LATIN_CAPITAL_LETTER_O_WITH_TILDE_AND_DIAERESIS,
            LatinExtendedAdditional::LatinSmallLetterOWithTildeAndDiaeresis => LATIN_SMALL_LETTER_O_WITH_TILDE_AND_DIAERESIS,
            LatinExtendedAdditional::LatinCapitalLetterOWithMacronAndGrave => LATIN_CAPITAL_LETTER_O_WITH_MACRON_AND_GRAVE,
            LatinExtendedAdditional::LatinSmallLetterOWithMacronAndGrave => LATIN_SMALL_LETTER_O_WITH_MACRON_AND_GRAVE,
            LatinExtendedAdditional::LatinCapitalLetterOWithMacronAndAcute => LATIN_CAPITAL_LETTER_O_WITH_MACRON_AND_ACUTE,
            LatinExtendedAdditional::LatinSmallLetterOWithMacronAndAcute => LATIN_SMALL_LETTER_O_WITH_MACRON_AND_ACUTE,
            LatinExtendedAdditional::LatinCapitalLetterPWithAcute => LATIN_CAPITAL_LETTER_P_WITH_ACUTE,
            LatinExtendedAdditional::LatinSmallLetterPWithAcute => LATIN_SMALL_LETTER_P_WITH_ACUTE,
            LatinExtendedAdditional::LatinCapitalLetterPWithDotAbove => LATIN_CAPITAL_LETTER_P_WITH_DOT_ABOVE,
            LatinExtendedAdditional::LatinSmallLetterPWithDotAbove => LATIN_SMALL_LETTER_P_WITH_DOT_ABOVE,
            LatinExtendedAdditional::LatinCapitalLetterRWithDotAbove => LATIN_CAPITAL_LETTER_R_WITH_DOT_ABOVE,
            LatinExtendedAdditional::LatinSmallLetterRWithDotAbove => LATIN_SMALL_LETTER_R_WITH_DOT_ABOVE,
            LatinExtendedAdditional::LatinCapitalLetterRWithDotBelow => LATIN_CAPITAL_LETTER_R_WITH_DOT_BELOW,
            LatinExtendedAdditional::LatinSmallLetterRWithDotBelow => LATIN_SMALL_LETTER_R_WITH_DOT_BELOW,
            LatinExtendedAdditional::LatinCapitalLetterRWithDotBelowAndMacron => LATIN_CAPITAL_LETTER_R_WITH_DOT_BELOW_AND_MACRON,
            LatinExtendedAdditional::LatinSmallLetterRWithDotBelowAndMacron => LATIN_SMALL_LETTER_R_WITH_DOT_BELOW_AND_MACRON,
            LatinExtendedAdditional::LatinCapitalLetterRWithLineBelow => LATIN_CAPITAL_LETTER_R_WITH_LINE_BELOW,
            LatinExtendedAdditional::LatinSmallLetterRWithLineBelow => LATIN_SMALL_LETTER_R_WITH_LINE_BELOW,
            LatinExtendedAdditional::LatinCapitalLetterSWithDotAbove => LATIN_CAPITAL_LETTER_S_WITH_DOT_ABOVE,
            LatinExtendedAdditional::LatinSmallLetterSWithDotAbove => LATIN_SMALL_LETTER_S_WITH_DOT_ABOVE,
            LatinExtendedAdditional::LatinCapitalLetterSWithDotBelow => LATIN_CAPITAL_LETTER_S_WITH_DOT_BELOW,
            LatinExtendedAdditional::LatinSmallLetterSWithDotBelow => LATIN_SMALL_LETTER_S_WITH_DOT_BELOW,
            LatinExtendedAdditional::LatinCapitalLetterSWithAcuteAndDotAbove => LATIN_CAPITAL_LETTER_S_WITH_ACUTE_AND_DOT_ABOVE,
            LatinExtendedAdditional::LatinSmallLetterSWithAcuteAndDotAbove => LATIN_SMALL_LETTER_S_WITH_ACUTE_AND_DOT_ABOVE,
            LatinExtendedAdditional::LatinCapitalLetterSWithCaronAndDotAbove => LATIN_CAPITAL_LETTER_S_WITH_CARON_AND_DOT_ABOVE,
            LatinExtendedAdditional::LatinSmallLetterSWithCaronAndDotAbove => LATIN_SMALL_LETTER_S_WITH_CARON_AND_DOT_ABOVE,
            LatinExtendedAdditional::LatinCapitalLetterSWithDotBelowAndDotAbove => LATIN_CAPITAL_LETTER_S_WITH_DOT_BELOW_AND_DOT_ABOVE,
            LatinExtendedAdditional::LatinSmallLetterSWithDotBelowAndDotAbove => LATIN_SMALL_LETTER_S_WITH_DOT_BELOW_AND_DOT_ABOVE,
            LatinExtendedAdditional::LatinCapitalLetterTWithDotAbove => LATIN_CAPITAL_LETTER_T_WITH_DOT_ABOVE,
            LatinExtendedAdditional::LatinSmallLetterTWithDotAbove => LATIN_SMALL_LETTER_T_WITH_DOT_ABOVE,
            LatinExtendedAdditional::LatinCapitalLetterTWithDotBelow => LATIN_CAPITAL_LETTER_T_WITH_DOT_BELOW,
            LatinExtendedAdditional::LatinSmallLetterTWithDotBelow => LATIN_SMALL_LETTER_T_WITH_DOT_BELOW,
            LatinExtendedAdditional::LatinCapitalLetterTWithLineBelow => LATIN_CAPITAL_LETTER_T_WITH_LINE_BELOW,
            LatinExtendedAdditional::LatinSmallLetterTWithLineBelow => LATIN_SMALL_LETTER_T_WITH_LINE_BELOW,
            LatinExtendedAdditional::LatinCapitalLetterTWithCircumflexBelow => LATIN_CAPITAL_LETTER_T_WITH_CIRCUMFLEX_BELOW,
            LatinExtendedAdditional::LatinSmallLetterTWithCircumflexBelow => LATIN_SMALL_LETTER_T_WITH_CIRCUMFLEX_BELOW,
            LatinExtendedAdditional::LatinCapitalLetterUWithDiaeresisBelow => LATIN_CAPITAL_LETTER_U_WITH_DIAERESIS_BELOW,
            LatinExtendedAdditional::LatinSmallLetterUWithDiaeresisBelow => LATIN_SMALL_LETTER_U_WITH_DIAERESIS_BELOW,
            LatinExtendedAdditional::LatinCapitalLetterUWithTildeBelow => LATIN_CAPITAL_LETTER_U_WITH_TILDE_BELOW,
            LatinExtendedAdditional::LatinSmallLetterUWithTildeBelow => LATIN_SMALL_LETTER_U_WITH_TILDE_BELOW,
            LatinExtendedAdditional::LatinCapitalLetterUWithCircumflexBelow => LATIN_CAPITAL_LETTER_U_WITH_CIRCUMFLEX_BELOW,
            LatinExtendedAdditional::LatinSmallLetterUWithCircumflexBelow => LATIN_SMALL_LETTER_U_WITH_CIRCUMFLEX_BELOW,
            LatinExtendedAdditional::LatinCapitalLetterUWithTildeAndAcute => LATIN_CAPITAL_LETTER_U_WITH_TILDE_AND_ACUTE,
            LatinExtendedAdditional::LatinSmallLetterUWithTildeAndAcute => LATIN_SMALL_LETTER_U_WITH_TILDE_AND_ACUTE,
            LatinExtendedAdditional::LatinCapitalLetterUWithMacronAndDiaeresis => LATIN_CAPITAL_LETTER_U_WITH_MACRON_AND_DIAERESIS,
            LatinExtendedAdditional::LatinSmallLetterUWithMacronAndDiaeresis => LATIN_SMALL_LETTER_U_WITH_MACRON_AND_DIAERESIS,
            LatinExtendedAdditional::LatinCapitalLetterVWithTilde => LATIN_CAPITAL_LETTER_V_WITH_TILDE,
            LatinExtendedAdditional::LatinSmallLetterVWithTilde => LATIN_SMALL_LETTER_V_WITH_TILDE,
            LatinExtendedAdditional::LatinCapitalLetterVWithDotBelow => LATIN_CAPITAL_LETTER_V_WITH_DOT_BELOW,
            LatinExtendedAdditional::LatinSmallLetterVWithDotBelow => LATIN_SMALL_LETTER_V_WITH_DOT_BELOW,
            LatinExtendedAdditional::LatinCapitalLetterWWithGrave => LATIN_CAPITAL_LETTER_W_WITH_GRAVE,
            LatinExtendedAdditional::LatinSmallLetterWWithGrave => LATIN_SMALL_LETTER_W_WITH_GRAVE,
            LatinExtendedAdditional::LatinCapitalLetterWWithAcute => LATIN_CAPITAL_LETTER_W_WITH_ACUTE,
            LatinExtendedAdditional::LatinSmallLetterWWithAcute => LATIN_SMALL_LETTER_W_WITH_ACUTE,
            LatinExtendedAdditional::LatinCapitalLetterWWithDiaeresis => LATIN_CAPITAL_LETTER_W_WITH_DIAERESIS,
            LatinExtendedAdditional::LatinSmallLetterWWithDiaeresis => LATIN_SMALL_LETTER_W_WITH_DIAERESIS,
            LatinExtendedAdditional::LatinCapitalLetterWWithDotAbove => LATIN_CAPITAL_LETTER_W_WITH_DOT_ABOVE,
            LatinExtendedAdditional::LatinSmallLetterWWithDotAbove => LATIN_SMALL_LETTER_W_WITH_DOT_ABOVE,
            LatinExtendedAdditional::LatinCapitalLetterWWithDotBelow => LATIN_CAPITAL_LETTER_W_WITH_DOT_BELOW,
            LatinExtendedAdditional::LatinSmallLetterWWithDotBelow => LATIN_SMALL_LETTER_W_WITH_DOT_BELOW,
            LatinExtendedAdditional::LatinCapitalLetterXWithDotAbove => LATIN_CAPITAL_LETTER_X_WITH_DOT_ABOVE,
            LatinExtendedAdditional::LatinSmallLetterXWithDotAbove => LATIN_SMALL_LETTER_X_WITH_DOT_ABOVE,
            LatinExtendedAdditional::LatinCapitalLetterXWithDiaeresis => LATIN_CAPITAL_LETTER_X_WITH_DIAERESIS,
            LatinExtendedAdditional::LatinSmallLetterXWithDiaeresis => LATIN_SMALL_LETTER_X_WITH_DIAERESIS,
            LatinExtendedAdditional::LatinCapitalLetterYWithDotAbove => LATIN_CAPITAL_LETTER_Y_WITH_DOT_ABOVE,
            LatinExtendedAdditional::LatinSmallLetterYWithDotAbove => LATIN_SMALL_LETTER_Y_WITH_DOT_ABOVE,
            LatinExtendedAdditional::LatinCapitalLetterZWithCircumflex => LATIN_CAPITAL_LETTER_Z_WITH_CIRCUMFLEX,
            LatinExtendedAdditional::LatinSmallLetterZWithCircumflex => LATIN_SMALL_LETTER_Z_WITH_CIRCUMFLEX,
            LatinExtendedAdditional::LatinCapitalLetterZWithDotBelow => LATIN_CAPITAL_LETTER_Z_WITH_DOT_BELOW,
            LatinExtendedAdditional::LatinSmallLetterZWithDotBelow => LATIN_SMALL_LETTER_Z_WITH_DOT_BELOW,
            LatinExtendedAdditional::LatinCapitalLetterZWithLineBelow => LATIN_CAPITAL_LETTER_Z_WITH_LINE_BELOW,
            LatinExtendedAdditional::LatinSmallLetterZWithLineBelow => LATIN_SMALL_LETTER_Z_WITH_LINE_BELOW,
            LatinExtendedAdditional::LatinSmallLetterHWithLineBelow => LATIN_SMALL_LETTER_H_WITH_LINE_BELOW,
            LatinExtendedAdditional::LatinSmallLetterTWithDiaeresis => LATIN_SMALL_LETTER_T_WITH_DIAERESIS,
            LatinExtendedAdditional::LatinSmallLetterWWithRingAbove => LATIN_SMALL_LETTER_W_WITH_RING_ABOVE,
            LatinExtendedAdditional::LatinSmallLetterYWithRingAbove => LATIN_SMALL_LETTER_Y_WITH_RING_ABOVE,
            LatinExtendedAdditional::LatinSmallLetterAWithRightHalfRing => LATIN_SMALL_LETTER_A_WITH_RIGHT_HALF_RING,
            LatinExtendedAdditional::LatinSmallLetterLongSWithDotAbove => LATIN_SMALL_LETTER_LONG_S_WITH_DOT_ABOVE,
            LatinExtendedAdditional::LatinSmallLetterLongSWithDiagonalStroke => LATIN_SMALL_LETTER_LONG_S_WITH_DIAGONAL_STROKE,
            LatinExtendedAdditional::LatinSmallLetterLongSWithHighStroke => LATIN_SMALL_LETTER_LONG_S_WITH_HIGH_STROKE,
            LatinExtendedAdditional::LatinCapitalLetterSharpS => LATIN_CAPITAL_LETTER_SHARP_S,
            LatinExtendedAdditional::LatinSmallLetterDelta => LATIN_SMALL_LETTER_DELTA,
            LatinExtendedAdditional::LatinCapitalLetterAWithDotBelow => LATIN_CAPITAL_LETTER_A_WITH_DOT_BELOW,
            LatinExtendedAdditional::LatinSmallLetterAWithDotBelow => LATIN_SMALL_LETTER_A_WITH_DOT_BELOW,
            LatinExtendedAdditional::LatinCapitalLetterAWithHookAbove => LATIN_CAPITAL_LETTER_A_WITH_HOOK_ABOVE,
            LatinExtendedAdditional::LatinSmallLetterAWithHookAbove => LATIN_SMALL_LETTER_A_WITH_HOOK_ABOVE,
            LatinExtendedAdditional::LatinCapitalLetterAWithCircumflexAndAcute => LATIN_CAPITAL_LETTER_A_WITH_CIRCUMFLEX_AND_ACUTE,
            LatinExtendedAdditional::LatinSmallLetterAWithCircumflexAndAcute => LATIN_SMALL_LETTER_A_WITH_CIRCUMFLEX_AND_ACUTE,
            LatinExtendedAdditional::LatinCapitalLetterAWithCircumflexAndGrave => LATIN_CAPITAL_LETTER_A_WITH_CIRCUMFLEX_AND_GRAVE,
            LatinExtendedAdditional::LatinSmallLetterAWithCircumflexAndGrave => LATIN_SMALL_LETTER_A_WITH_CIRCUMFLEX_AND_GRAVE,
            LatinExtendedAdditional::LatinCapitalLetterAWithCircumflexAndHookAbove => LATIN_CAPITAL_LETTER_A_WITH_CIRCUMFLEX_AND_HOOK_ABOVE,
            LatinExtendedAdditional::LatinSmallLetterAWithCircumflexAndHookAbove => LATIN_SMALL_LETTER_A_WITH_CIRCUMFLEX_AND_HOOK_ABOVE,
            LatinExtendedAdditional::LatinCapitalLetterAWithCircumflexAndTilde => LATIN_CAPITAL_LETTER_A_WITH_CIRCUMFLEX_AND_TILDE,
            LatinExtendedAdditional::LatinSmallLetterAWithCircumflexAndTilde => LATIN_SMALL_LETTER_A_WITH_CIRCUMFLEX_AND_TILDE,
            LatinExtendedAdditional::LatinCapitalLetterAWithCircumflexAndDotBelow => LATIN_CAPITAL_LETTER_A_WITH_CIRCUMFLEX_AND_DOT_BELOW,
            LatinExtendedAdditional::LatinSmallLetterAWithCircumflexAndDotBelow => LATIN_SMALL_LETTER_A_WITH_CIRCUMFLEX_AND_DOT_BELOW,
            LatinExtendedAdditional::LatinCapitalLetterAWithBreveAndAcute => LATIN_CAPITAL_LETTER_A_WITH_BREVE_AND_ACUTE,
            LatinExtendedAdditional::LatinSmallLetterAWithBreveAndAcute => LATIN_SMALL_LETTER_A_WITH_BREVE_AND_ACUTE,
            LatinExtendedAdditional::LatinCapitalLetterAWithBreveAndGrave => LATIN_CAPITAL_LETTER_A_WITH_BREVE_AND_GRAVE,
            LatinExtendedAdditional::LatinSmallLetterAWithBreveAndGrave => LATIN_SMALL_LETTER_A_WITH_BREVE_AND_GRAVE,
            LatinExtendedAdditional::LatinCapitalLetterAWithBreveAndHookAbove => LATIN_CAPITAL_LETTER_A_WITH_BREVE_AND_HOOK_ABOVE,
            LatinExtendedAdditional::LatinSmallLetterAWithBreveAndHookAbove => LATIN_SMALL_LETTER_A_WITH_BREVE_AND_HOOK_ABOVE,
            LatinExtendedAdditional::LatinCapitalLetterAWithBreveAndTilde => LATIN_CAPITAL_LETTER_A_WITH_BREVE_AND_TILDE,
            LatinExtendedAdditional::LatinSmallLetterAWithBreveAndTilde => LATIN_SMALL_LETTER_A_WITH_BREVE_AND_TILDE,
            LatinExtendedAdditional::LatinCapitalLetterAWithBreveAndDotBelow => LATIN_CAPITAL_LETTER_A_WITH_BREVE_AND_DOT_BELOW,
            LatinExtendedAdditional::LatinSmallLetterAWithBreveAndDotBelow => LATIN_SMALL_LETTER_A_WITH_BREVE_AND_DOT_BELOW,
            LatinExtendedAdditional::LatinCapitalLetterEWithDotBelow => LATIN_CAPITAL_LETTER_E_WITH_DOT_BELOW,
            LatinExtendedAdditional::LatinSmallLetterEWithDotBelow => LATIN_SMALL_LETTER_E_WITH_DOT_BELOW,
            LatinExtendedAdditional::LatinCapitalLetterEWithHookAbove => LATIN_CAPITAL_LETTER_E_WITH_HOOK_ABOVE,
            LatinExtendedAdditional::LatinSmallLetterEWithHookAbove => LATIN_SMALL_LETTER_E_WITH_HOOK_ABOVE,
            LatinExtendedAdditional::LatinCapitalLetterEWithTilde => LATIN_CAPITAL_LETTER_E_WITH_TILDE,
            LatinExtendedAdditional::LatinSmallLetterEWithTilde => LATIN_SMALL_LETTER_E_WITH_TILDE,
            LatinExtendedAdditional::LatinCapitalLetterEWithCircumflexAndAcute => LATIN_CAPITAL_LETTER_E_WITH_CIRCUMFLEX_AND_ACUTE,
            LatinExtendedAdditional::LatinSmallLetterEWithCircumflexAndAcute => LATIN_SMALL_LETTER_E_WITH_CIRCUMFLEX_AND_ACUTE,
            LatinExtendedAdditional::LatinCapitalLetterEWithCircumflexAndGrave => LATIN_CAPITAL_LETTER_E_WITH_CIRCUMFLEX_AND_GRAVE,
            LatinExtendedAdditional::LatinSmallLetterEWithCircumflexAndGrave => LATIN_SMALL_LETTER_E_WITH_CIRCUMFLEX_AND_GRAVE,
            LatinExtendedAdditional::LatinCapitalLetterEWithCircumflexAndHookAbove => LATIN_CAPITAL_LETTER_E_WITH_CIRCUMFLEX_AND_HOOK_ABOVE,
            LatinExtendedAdditional::LatinSmallLetterEWithCircumflexAndHookAbove => LATIN_SMALL_LETTER_E_WITH_CIRCUMFLEX_AND_HOOK_ABOVE,
            LatinExtendedAdditional::LatinCapitalLetterEWithCircumflexAndTilde => LATIN_CAPITAL_LETTER_E_WITH_CIRCUMFLEX_AND_TILDE,
            LatinExtendedAdditional::LatinSmallLetterEWithCircumflexAndTilde => LATIN_SMALL_LETTER_E_WITH_CIRCUMFLEX_AND_TILDE,
            LatinExtendedAdditional::LatinCapitalLetterEWithCircumflexAndDotBelow => LATIN_CAPITAL_LETTER_E_WITH_CIRCUMFLEX_AND_DOT_BELOW,
            LatinExtendedAdditional::LatinSmallLetterEWithCircumflexAndDotBelow => LATIN_SMALL_LETTER_E_WITH_CIRCUMFLEX_AND_DOT_BELOW,
            LatinExtendedAdditional::LatinCapitalLetterIWithHookAbove => LATIN_CAPITAL_LETTER_I_WITH_HOOK_ABOVE,
            LatinExtendedAdditional::LatinSmallLetterIWithHookAbove => LATIN_SMALL_LETTER_I_WITH_HOOK_ABOVE,
            LatinExtendedAdditional::LatinCapitalLetterIWithDotBelow => LATIN_CAPITAL_LETTER_I_WITH_DOT_BELOW,
            LatinExtendedAdditional::LatinSmallLetterIWithDotBelow => LATIN_SMALL_LETTER_I_WITH_DOT_BELOW,
            LatinExtendedAdditional::LatinCapitalLetterOWithDotBelow => LATIN_CAPITAL_LETTER_O_WITH_DOT_BELOW,
            LatinExtendedAdditional::LatinSmallLetterOWithDotBelow => LATIN_SMALL_LETTER_O_WITH_DOT_BELOW,
            LatinExtendedAdditional::LatinCapitalLetterOWithHookAbove => LATIN_CAPITAL_LETTER_O_WITH_HOOK_ABOVE,
            LatinExtendedAdditional::LatinSmallLetterOWithHookAbove => LATIN_SMALL_LETTER_O_WITH_HOOK_ABOVE,
            LatinExtendedAdditional::LatinCapitalLetterOWithCircumflexAndAcute => LATIN_CAPITAL_LETTER_O_WITH_CIRCUMFLEX_AND_ACUTE,
            LatinExtendedAdditional::LatinSmallLetterOWithCircumflexAndAcute => LATIN_SMALL_LETTER_O_WITH_CIRCUMFLEX_AND_ACUTE,
            LatinExtendedAdditional::LatinCapitalLetterOWithCircumflexAndGrave => LATIN_CAPITAL_LETTER_O_WITH_CIRCUMFLEX_AND_GRAVE,
            LatinExtendedAdditional::LatinSmallLetterOWithCircumflexAndGrave => LATIN_SMALL_LETTER_O_WITH_CIRCUMFLEX_AND_GRAVE,
            LatinExtendedAdditional::LatinCapitalLetterOWithCircumflexAndHookAbove => LATIN_CAPITAL_LETTER_O_WITH_CIRCUMFLEX_AND_HOOK_ABOVE,
            LatinExtendedAdditional::LatinSmallLetterOWithCircumflexAndHookAbove => LATIN_SMALL_LETTER_O_WITH_CIRCUMFLEX_AND_HOOK_ABOVE,
            LatinExtendedAdditional::LatinCapitalLetterOWithCircumflexAndTilde => LATIN_CAPITAL_LETTER_O_WITH_CIRCUMFLEX_AND_TILDE,
            LatinExtendedAdditional::LatinSmallLetterOWithCircumflexAndTilde => LATIN_SMALL_LETTER_O_WITH_CIRCUMFLEX_AND_TILDE,
            LatinExtendedAdditional::LatinCapitalLetterOWithCircumflexAndDotBelow => LATIN_CAPITAL_LETTER_O_WITH_CIRCUMFLEX_AND_DOT_BELOW,
            LatinExtendedAdditional::LatinSmallLetterOWithCircumflexAndDotBelow => LATIN_SMALL_LETTER_O_WITH_CIRCUMFLEX_AND_DOT_BELOW,
            LatinExtendedAdditional::LatinCapitalLetterOWithHornAndAcute => LATIN_CAPITAL_LETTER_O_WITH_HORN_AND_ACUTE,
            LatinExtendedAdditional::LatinSmallLetterOWithHornAndAcute => LATIN_SMALL_LETTER_O_WITH_HORN_AND_ACUTE,
            LatinExtendedAdditional::LatinCapitalLetterOWithHornAndGrave => LATIN_CAPITAL_LETTER_O_WITH_HORN_AND_GRAVE,
            LatinExtendedAdditional::LatinSmallLetterOWithHornAndGrave => LATIN_SMALL_LETTER_O_WITH_HORN_AND_GRAVE,
            LatinExtendedAdditional::LatinCapitalLetterOWithHornAndHookAbove => LATIN_CAPITAL_LETTER_O_WITH_HORN_AND_HOOK_ABOVE,
            LatinExtendedAdditional::LatinSmallLetterOWithHornAndHookAbove => LATIN_SMALL_LETTER_O_WITH_HORN_AND_HOOK_ABOVE,
            LatinExtendedAdditional::LatinCapitalLetterOWithHornAndTilde => LATIN_CAPITAL_LETTER_O_WITH_HORN_AND_TILDE,
            LatinExtendedAdditional::LatinSmallLetterOWithHornAndTilde => LATIN_SMALL_LETTER_O_WITH_HORN_AND_TILDE,
            LatinExtendedAdditional::LatinCapitalLetterOWithHornAndDotBelow => LATIN_CAPITAL_LETTER_O_WITH_HORN_AND_DOT_BELOW,
            LatinExtendedAdditional::LatinSmallLetterOWithHornAndDotBelow => LATIN_SMALL_LETTER_O_WITH_HORN_AND_DOT_BELOW,
            LatinExtendedAdditional::LatinCapitalLetterUWithDotBelow => LATIN_CAPITAL_LETTER_U_WITH_DOT_BELOW,
            LatinExtendedAdditional::LatinSmallLetterUWithDotBelow => LATIN_SMALL_LETTER_U_WITH_DOT_BELOW,
            LatinExtendedAdditional::LatinCapitalLetterUWithHookAbove => LATIN_CAPITAL_LETTER_U_WITH_HOOK_ABOVE,
            LatinExtendedAdditional::LatinSmallLetterUWithHookAbove => LATIN_SMALL_LETTER_U_WITH_HOOK_ABOVE,
            LatinExtendedAdditional::LatinCapitalLetterUWithHornAndAcute => LATIN_CAPITAL_LETTER_U_WITH_HORN_AND_ACUTE,
            LatinExtendedAdditional::LatinSmallLetterUWithHornAndAcute => LATIN_SMALL_LETTER_U_WITH_HORN_AND_ACUTE,
            LatinExtendedAdditional::LatinCapitalLetterUWithHornAndGrave => LATIN_CAPITAL_LETTER_U_WITH_HORN_AND_GRAVE,
            LatinExtendedAdditional::LatinSmallLetterUWithHornAndGrave => LATIN_SMALL_LETTER_U_WITH_HORN_AND_GRAVE,
            LatinExtendedAdditional::LatinCapitalLetterUWithHornAndHookAbove => LATIN_CAPITAL_LETTER_U_WITH_HORN_AND_HOOK_ABOVE,
            LatinExtendedAdditional::LatinSmallLetterUWithHornAndHookAbove => LATIN_SMALL_LETTER_U_WITH_HORN_AND_HOOK_ABOVE,
            LatinExtendedAdditional::LatinCapitalLetterUWithHornAndTilde => LATIN_CAPITAL_LETTER_U_WITH_HORN_AND_TILDE,
            LatinExtendedAdditional::LatinSmallLetterUWithHornAndTilde => LATIN_SMALL_LETTER_U_WITH_HORN_AND_TILDE,
            LatinExtendedAdditional::LatinCapitalLetterUWithHornAndDotBelow => LATIN_CAPITAL_LETTER_U_WITH_HORN_AND_DOT_BELOW,
            LatinExtendedAdditional::LatinSmallLetterUWithHornAndDotBelow => LATIN_SMALL_LETTER_U_WITH_HORN_AND_DOT_BELOW,
            LatinExtendedAdditional::LatinCapitalLetterYWithGrave => LATIN_CAPITAL_LETTER_Y_WITH_GRAVE,
            LatinExtendedAdditional::LatinSmallLetterYWithGrave => LATIN_SMALL_LETTER_Y_WITH_GRAVE,
            LatinExtendedAdditional::LatinCapitalLetterYWithDotBelow => LATIN_CAPITAL_LETTER_Y_WITH_DOT_BELOW,
            LatinExtendedAdditional::LatinSmallLetterYWithDotBelow => LATIN_SMALL_LETTER_Y_WITH_DOT_BELOW,
            LatinExtendedAdditional::LatinCapitalLetterYWithHookAbove => LATIN_CAPITAL_LETTER_Y_WITH_HOOK_ABOVE,
            LatinExtendedAdditional::LatinSmallLetterYWithHookAbove => LATIN_SMALL_LETTER_Y_WITH_HOOK_ABOVE,
            LatinExtendedAdditional::LatinCapitalLetterYWithTilde => LATIN_CAPITAL_LETTER_Y_WITH_TILDE,
            LatinExtendedAdditional::LatinSmallLetterYWithTilde => LATIN_SMALL_LETTER_Y_WITH_TILDE,
            LatinExtendedAdditional::LatinCapitalLetterMiddleDashWelshLl => LATIN_CAPITAL_LETTER_MIDDLE_DASH_WELSH_LL,
            LatinExtendedAdditional::LatinSmallLetterMiddleDashWelshLl => LATIN_SMALL_LETTER_MIDDLE_DASH_WELSH_LL,
            LatinExtendedAdditional::LatinCapitalLetterMiddleDashWelshV => LATIN_CAPITAL_LETTER_MIDDLE_DASH_WELSH_V,
            LatinExtendedAdditional::LatinSmallLetterMiddleDashWelshV => LATIN_SMALL_LETTER_MIDDLE_DASH_WELSH_V,
            LatinExtendedAdditional::LatinCapitalLetterYWithLoop => LATIN_CAPITAL_LETTER_Y_WITH_LOOP,
        }
    }
}

impl std::convert::TryFrom<char> for LatinExtendedAdditional {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LATIN_CAPITAL_LETTER_A_WITH_RING_BELOW => Ok(LatinExtendedAdditional::LatinCapitalLetterAWithRingBelow),
            LATIN_SMALL_LETTER_A_WITH_RING_BELOW => Ok(LatinExtendedAdditional::LatinSmallLetterAWithRingBelow),
            LATIN_CAPITAL_LETTER_B_WITH_DOT_ABOVE => Ok(LatinExtendedAdditional::LatinCapitalLetterBWithDotAbove),
            LATIN_SMALL_LETTER_B_WITH_DOT_ABOVE => Ok(LatinExtendedAdditional::LatinSmallLetterBWithDotAbove),
            LATIN_CAPITAL_LETTER_B_WITH_DOT_BELOW => Ok(LatinExtendedAdditional::LatinCapitalLetterBWithDotBelow),
            LATIN_SMALL_LETTER_B_WITH_DOT_BELOW => Ok(LatinExtendedAdditional::LatinSmallLetterBWithDotBelow),
            LATIN_CAPITAL_LETTER_B_WITH_LINE_BELOW => Ok(LatinExtendedAdditional::LatinCapitalLetterBWithLineBelow),
            LATIN_SMALL_LETTER_B_WITH_LINE_BELOW => Ok(LatinExtendedAdditional::LatinSmallLetterBWithLineBelow),
            LATIN_CAPITAL_LETTER_C_WITH_CEDILLA_AND_ACUTE => Ok(LatinExtendedAdditional::LatinCapitalLetterCWithCedillaAndAcute),
            LATIN_SMALL_LETTER_C_WITH_CEDILLA_AND_ACUTE => Ok(LatinExtendedAdditional::LatinSmallLetterCWithCedillaAndAcute),
            LATIN_CAPITAL_LETTER_D_WITH_DOT_ABOVE => Ok(LatinExtendedAdditional::LatinCapitalLetterDWithDotAbove),
            LATIN_SMALL_LETTER_D_WITH_DOT_ABOVE => Ok(LatinExtendedAdditional::LatinSmallLetterDWithDotAbove),
            LATIN_CAPITAL_LETTER_D_WITH_DOT_BELOW => Ok(LatinExtendedAdditional::LatinCapitalLetterDWithDotBelow),
            LATIN_SMALL_LETTER_D_WITH_DOT_BELOW => Ok(LatinExtendedAdditional::LatinSmallLetterDWithDotBelow),
            LATIN_CAPITAL_LETTER_D_WITH_LINE_BELOW => Ok(LatinExtendedAdditional::LatinCapitalLetterDWithLineBelow),
            LATIN_SMALL_LETTER_D_WITH_LINE_BELOW => Ok(LatinExtendedAdditional::LatinSmallLetterDWithLineBelow),
            LATIN_CAPITAL_LETTER_D_WITH_CEDILLA => Ok(LatinExtendedAdditional::LatinCapitalLetterDWithCedilla),
            LATIN_SMALL_LETTER_D_WITH_CEDILLA => Ok(LatinExtendedAdditional::LatinSmallLetterDWithCedilla),
            LATIN_CAPITAL_LETTER_D_WITH_CIRCUMFLEX_BELOW => Ok(LatinExtendedAdditional::LatinCapitalLetterDWithCircumflexBelow),
            LATIN_SMALL_LETTER_D_WITH_CIRCUMFLEX_BELOW => Ok(LatinExtendedAdditional::LatinSmallLetterDWithCircumflexBelow),
            LATIN_CAPITAL_LETTER_E_WITH_MACRON_AND_GRAVE => Ok(LatinExtendedAdditional::LatinCapitalLetterEWithMacronAndGrave),
            LATIN_SMALL_LETTER_E_WITH_MACRON_AND_GRAVE => Ok(LatinExtendedAdditional::LatinSmallLetterEWithMacronAndGrave),
            LATIN_CAPITAL_LETTER_E_WITH_MACRON_AND_ACUTE => Ok(LatinExtendedAdditional::LatinCapitalLetterEWithMacronAndAcute),
            LATIN_SMALL_LETTER_E_WITH_MACRON_AND_ACUTE => Ok(LatinExtendedAdditional::LatinSmallLetterEWithMacronAndAcute),
            LATIN_CAPITAL_LETTER_E_WITH_CIRCUMFLEX_BELOW => Ok(LatinExtendedAdditional::LatinCapitalLetterEWithCircumflexBelow),
            LATIN_SMALL_LETTER_E_WITH_CIRCUMFLEX_BELOW => Ok(LatinExtendedAdditional::LatinSmallLetterEWithCircumflexBelow),
            LATIN_CAPITAL_LETTER_E_WITH_TILDE_BELOW => Ok(LatinExtendedAdditional::LatinCapitalLetterEWithTildeBelow),
            LATIN_SMALL_LETTER_E_WITH_TILDE_BELOW => Ok(LatinExtendedAdditional::LatinSmallLetterEWithTildeBelow),
            LATIN_CAPITAL_LETTER_E_WITH_CEDILLA_AND_BREVE => Ok(LatinExtendedAdditional::LatinCapitalLetterEWithCedillaAndBreve),
            LATIN_SMALL_LETTER_E_WITH_CEDILLA_AND_BREVE => Ok(LatinExtendedAdditional::LatinSmallLetterEWithCedillaAndBreve),
            LATIN_CAPITAL_LETTER_F_WITH_DOT_ABOVE => Ok(LatinExtendedAdditional::LatinCapitalLetterFWithDotAbove),
            LATIN_SMALL_LETTER_F_WITH_DOT_ABOVE => Ok(LatinExtendedAdditional::LatinSmallLetterFWithDotAbove),
            LATIN_CAPITAL_LETTER_G_WITH_MACRON => Ok(LatinExtendedAdditional::LatinCapitalLetterGWithMacron),
            LATIN_SMALL_LETTER_G_WITH_MACRON => Ok(LatinExtendedAdditional::LatinSmallLetterGWithMacron),
            LATIN_CAPITAL_LETTER_H_WITH_DOT_ABOVE => Ok(LatinExtendedAdditional::LatinCapitalLetterHWithDotAbove),
            LATIN_SMALL_LETTER_H_WITH_DOT_ABOVE => Ok(LatinExtendedAdditional::LatinSmallLetterHWithDotAbove),
            LATIN_CAPITAL_LETTER_H_WITH_DOT_BELOW => Ok(LatinExtendedAdditional::LatinCapitalLetterHWithDotBelow),
            LATIN_SMALL_LETTER_H_WITH_DOT_BELOW => Ok(LatinExtendedAdditional::LatinSmallLetterHWithDotBelow),
            LATIN_CAPITAL_LETTER_H_WITH_DIAERESIS => Ok(LatinExtendedAdditional::LatinCapitalLetterHWithDiaeresis),
            LATIN_SMALL_LETTER_H_WITH_DIAERESIS => Ok(LatinExtendedAdditional::LatinSmallLetterHWithDiaeresis),
            LATIN_CAPITAL_LETTER_H_WITH_CEDILLA => Ok(LatinExtendedAdditional::LatinCapitalLetterHWithCedilla),
            LATIN_SMALL_LETTER_H_WITH_CEDILLA => Ok(LatinExtendedAdditional::LatinSmallLetterHWithCedilla),
            LATIN_CAPITAL_LETTER_H_WITH_BREVE_BELOW => Ok(LatinExtendedAdditional::LatinCapitalLetterHWithBreveBelow),
            LATIN_SMALL_LETTER_H_WITH_BREVE_BELOW => Ok(LatinExtendedAdditional::LatinSmallLetterHWithBreveBelow),
            LATIN_CAPITAL_LETTER_I_WITH_TILDE_BELOW => Ok(LatinExtendedAdditional::LatinCapitalLetterIWithTildeBelow),
            LATIN_SMALL_LETTER_I_WITH_TILDE_BELOW => Ok(LatinExtendedAdditional::LatinSmallLetterIWithTildeBelow),
            LATIN_CAPITAL_LETTER_I_WITH_DIAERESIS_AND_ACUTE => Ok(LatinExtendedAdditional::LatinCapitalLetterIWithDiaeresisAndAcute),
            LATIN_SMALL_LETTER_I_WITH_DIAERESIS_AND_ACUTE => Ok(LatinExtendedAdditional::LatinSmallLetterIWithDiaeresisAndAcute),
            LATIN_CAPITAL_LETTER_K_WITH_ACUTE => Ok(LatinExtendedAdditional::LatinCapitalLetterKWithAcute),
            LATIN_SMALL_LETTER_K_WITH_ACUTE => Ok(LatinExtendedAdditional::LatinSmallLetterKWithAcute),
            LATIN_CAPITAL_LETTER_K_WITH_DOT_BELOW => Ok(LatinExtendedAdditional::LatinCapitalLetterKWithDotBelow),
            LATIN_SMALL_LETTER_K_WITH_DOT_BELOW => Ok(LatinExtendedAdditional::LatinSmallLetterKWithDotBelow),
            LATIN_CAPITAL_LETTER_K_WITH_LINE_BELOW => Ok(LatinExtendedAdditional::LatinCapitalLetterKWithLineBelow),
            LATIN_SMALL_LETTER_K_WITH_LINE_BELOW => Ok(LatinExtendedAdditional::LatinSmallLetterKWithLineBelow),
            LATIN_CAPITAL_LETTER_L_WITH_DOT_BELOW => Ok(LatinExtendedAdditional::LatinCapitalLetterLWithDotBelow),
            LATIN_SMALL_LETTER_L_WITH_DOT_BELOW => Ok(LatinExtendedAdditional::LatinSmallLetterLWithDotBelow),
            LATIN_CAPITAL_LETTER_L_WITH_DOT_BELOW_AND_MACRON => Ok(LatinExtendedAdditional::LatinCapitalLetterLWithDotBelowAndMacron),
            LATIN_SMALL_LETTER_L_WITH_DOT_BELOW_AND_MACRON => Ok(LatinExtendedAdditional::LatinSmallLetterLWithDotBelowAndMacron),
            LATIN_CAPITAL_LETTER_L_WITH_LINE_BELOW => Ok(LatinExtendedAdditional::LatinCapitalLetterLWithLineBelow),
            LATIN_SMALL_LETTER_L_WITH_LINE_BELOW => Ok(LatinExtendedAdditional::LatinSmallLetterLWithLineBelow),
            LATIN_CAPITAL_LETTER_L_WITH_CIRCUMFLEX_BELOW => Ok(LatinExtendedAdditional::LatinCapitalLetterLWithCircumflexBelow),
            LATIN_SMALL_LETTER_L_WITH_CIRCUMFLEX_BELOW => Ok(LatinExtendedAdditional::LatinSmallLetterLWithCircumflexBelow),
            LATIN_CAPITAL_LETTER_M_WITH_ACUTE => Ok(LatinExtendedAdditional::LatinCapitalLetterMWithAcute),
            LATIN_SMALL_LETTER_M_WITH_ACUTE => Ok(LatinExtendedAdditional::LatinSmallLetterMWithAcute),
            LATIN_CAPITAL_LETTER_M_WITH_DOT_ABOVE => Ok(LatinExtendedAdditional::LatinCapitalLetterMWithDotAbove),
            LATIN_SMALL_LETTER_M_WITH_DOT_ABOVE => Ok(LatinExtendedAdditional::LatinSmallLetterMWithDotAbove),
            LATIN_CAPITAL_LETTER_M_WITH_DOT_BELOW => Ok(LatinExtendedAdditional::LatinCapitalLetterMWithDotBelow),
            LATIN_SMALL_LETTER_M_WITH_DOT_BELOW => Ok(LatinExtendedAdditional::LatinSmallLetterMWithDotBelow),
            LATIN_CAPITAL_LETTER_N_WITH_DOT_ABOVE => Ok(LatinExtendedAdditional::LatinCapitalLetterNWithDotAbove),
            LATIN_SMALL_LETTER_N_WITH_DOT_ABOVE => Ok(LatinExtendedAdditional::LatinSmallLetterNWithDotAbove),
            LATIN_CAPITAL_LETTER_N_WITH_DOT_BELOW => Ok(LatinExtendedAdditional::LatinCapitalLetterNWithDotBelow),
            LATIN_SMALL_LETTER_N_WITH_DOT_BELOW => Ok(LatinExtendedAdditional::LatinSmallLetterNWithDotBelow),
            LATIN_CAPITAL_LETTER_N_WITH_LINE_BELOW => Ok(LatinExtendedAdditional::LatinCapitalLetterNWithLineBelow),
            LATIN_SMALL_LETTER_N_WITH_LINE_BELOW => Ok(LatinExtendedAdditional::LatinSmallLetterNWithLineBelow),
            LATIN_CAPITAL_LETTER_N_WITH_CIRCUMFLEX_BELOW => Ok(LatinExtendedAdditional::LatinCapitalLetterNWithCircumflexBelow),
            LATIN_SMALL_LETTER_N_WITH_CIRCUMFLEX_BELOW => Ok(LatinExtendedAdditional::LatinSmallLetterNWithCircumflexBelow),
            LATIN_CAPITAL_LETTER_O_WITH_TILDE_AND_ACUTE => Ok(LatinExtendedAdditional::LatinCapitalLetterOWithTildeAndAcute),
            LATIN_SMALL_LETTER_O_WITH_TILDE_AND_ACUTE => Ok(LatinExtendedAdditional::LatinSmallLetterOWithTildeAndAcute),
            LATIN_CAPITAL_LETTER_O_WITH_TILDE_AND_DIAERESIS => Ok(LatinExtendedAdditional::LatinCapitalLetterOWithTildeAndDiaeresis),
            LATIN_SMALL_LETTER_O_WITH_TILDE_AND_DIAERESIS => Ok(LatinExtendedAdditional::LatinSmallLetterOWithTildeAndDiaeresis),
            LATIN_CAPITAL_LETTER_O_WITH_MACRON_AND_GRAVE => Ok(LatinExtendedAdditional::LatinCapitalLetterOWithMacronAndGrave),
            LATIN_SMALL_LETTER_O_WITH_MACRON_AND_GRAVE => Ok(LatinExtendedAdditional::LatinSmallLetterOWithMacronAndGrave),
            LATIN_CAPITAL_LETTER_O_WITH_MACRON_AND_ACUTE => Ok(LatinExtendedAdditional::LatinCapitalLetterOWithMacronAndAcute),
            LATIN_SMALL_LETTER_O_WITH_MACRON_AND_ACUTE => Ok(LatinExtendedAdditional::LatinSmallLetterOWithMacronAndAcute),
            LATIN_CAPITAL_LETTER_P_WITH_ACUTE => Ok(LatinExtendedAdditional::LatinCapitalLetterPWithAcute),
            LATIN_SMALL_LETTER_P_WITH_ACUTE => Ok(LatinExtendedAdditional::LatinSmallLetterPWithAcute),
            LATIN_CAPITAL_LETTER_P_WITH_DOT_ABOVE => Ok(LatinExtendedAdditional::LatinCapitalLetterPWithDotAbove),
            LATIN_SMALL_LETTER_P_WITH_DOT_ABOVE => Ok(LatinExtendedAdditional::LatinSmallLetterPWithDotAbove),
            LATIN_CAPITAL_LETTER_R_WITH_DOT_ABOVE => Ok(LatinExtendedAdditional::LatinCapitalLetterRWithDotAbove),
            LATIN_SMALL_LETTER_R_WITH_DOT_ABOVE => Ok(LatinExtendedAdditional::LatinSmallLetterRWithDotAbove),
            LATIN_CAPITAL_LETTER_R_WITH_DOT_BELOW => Ok(LatinExtendedAdditional::LatinCapitalLetterRWithDotBelow),
            LATIN_SMALL_LETTER_R_WITH_DOT_BELOW => Ok(LatinExtendedAdditional::LatinSmallLetterRWithDotBelow),
            LATIN_CAPITAL_LETTER_R_WITH_DOT_BELOW_AND_MACRON => Ok(LatinExtendedAdditional::LatinCapitalLetterRWithDotBelowAndMacron),
            LATIN_SMALL_LETTER_R_WITH_DOT_BELOW_AND_MACRON => Ok(LatinExtendedAdditional::LatinSmallLetterRWithDotBelowAndMacron),
            LATIN_CAPITAL_LETTER_R_WITH_LINE_BELOW => Ok(LatinExtendedAdditional::LatinCapitalLetterRWithLineBelow),
            LATIN_SMALL_LETTER_R_WITH_LINE_BELOW => Ok(LatinExtendedAdditional::LatinSmallLetterRWithLineBelow),
            LATIN_CAPITAL_LETTER_S_WITH_DOT_ABOVE => Ok(LatinExtendedAdditional::LatinCapitalLetterSWithDotAbove),
            LATIN_SMALL_LETTER_S_WITH_DOT_ABOVE => Ok(LatinExtendedAdditional::LatinSmallLetterSWithDotAbove),
            LATIN_CAPITAL_LETTER_S_WITH_DOT_BELOW => Ok(LatinExtendedAdditional::LatinCapitalLetterSWithDotBelow),
            LATIN_SMALL_LETTER_S_WITH_DOT_BELOW => Ok(LatinExtendedAdditional::LatinSmallLetterSWithDotBelow),
            LATIN_CAPITAL_LETTER_S_WITH_ACUTE_AND_DOT_ABOVE => Ok(LatinExtendedAdditional::LatinCapitalLetterSWithAcuteAndDotAbove),
            LATIN_SMALL_LETTER_S_WITH_ACUTE_AND_DOT_ABOVE => Ok(LatinExtendedAdditional::LatinSmallLetterSWithAcuteAndDotAbove),
            LATIN_CAPITAL_LETTER_S_WITH_CARON_AND_DOT_ABOVE => Ok(LatinExtendedAdditional::LatinCapitalLetterSWithCaronAndDotAbove),
            LATIN_SMALL_LETTER_S_WITH_CARON_AND_DOT_ABOVE => Ok(LatinExtendedAdditional::LatinSmallLetterSWithCaronAndDotAbove),
            LATIN_CAPITAL_LETTER_S_WITH_DOT_BELOW_AND_DOT_ABOVE => Ok(LatinExtendedAdditional::LatinCapitalLetterSWithDotBelowAndDotAbove),
            LATIN_SMALL_LETTER_S_WITH_DOT_BELOW_AND_DOT_ABOVE => Ok(LatinExtendedAdditional::LatinSmallLetterSWithDotBelowAndDotAbove),
            LATIN_CAPITAL_LETTER_T_WITH_DOT_ABOVE => Ok(LatinExtendedAdditional::LatinCapitalLetterTWithDotAbove),
            LATIN_SMALL_LETTER_T_WITH_DOT_ABOVE => Ok(LatinExtendedAdditional::LatinSmallLetterTWithDotAbove),
            LATIN_CAPITAL_LETTER_T_WITH_DOT_BELOW => Ok(LatinExtendedAdditional::LatinCapitalLetterTWithDotBelow),
            LATIN_SMALL_LETTER_T_WITH_DOT_BELOW => Ok(LatinExtendedAdditional::LatinSmallLetterTWithDotBelow),
            LATIN_CAPITAL_LETTER_T_WITH_LINE_BELOW => Ok(LatinExtendedAdditional::LatinCapitalLetterTWithLineBelow),
            LATIN_SMALL_LETTER_T_WITH_LINE_BELOW => Ok(LatinExtendedAdditional::LatinSmallLetterTWithLineBelow),
            LATIN_CAPITAL_LETTER_T_WITH_CIRCUMFLEX_BELOW => Ok(LatinExtendedAdditional::LatinCapitalLetterTWithCircumflexBelow),
            LATIN_SMALL_LETTER_T_WITH_CIRCUMFLEX_BELOW => Ok(LatinExtendedAdditional::LatinSmallLetterTWithCircumflexBelow),
            LATIN_CAPITAL_LETTER_U_WITH_DIAERESIS_BELOW => Ok(LatinExtendedAdditional::LatinCapitalLetterUWithDiaeresisBelow),
            LATIN_SMALL_LETTER_U_WITH_DIAERESIS_BELOW => Ok(LatinExtendedAdditional::LatinSmallLetterUWithDiaeresisBelow),
            LATIN_CAPITAL_LETTER_U_WITH_TILDE_BELOW => Ok(LatinExtendedAdditional::LatinCapitalLetterUWithTildeBelow),
            LATIN_SMALL_LETTER_U_WITH_TILDE_BELOW => Ok(LatinExtendedAdditional::LatinSmallLetterUWithTildeBelow),
            LATIN_CAPITAL_LETTER_U_WITH_CIRCUMFLEX_BELOW => Ok(LatinExtendedAdditional::LatinCapitalLetterUWithCircumflexBelow),
            LATIN_SMALL_LETTER_U_WITH_CIRCUMFLEX_BELOW => Ok(LatinExtendedAdditional::LatinSmallLetterUWithCircumflexBelow),
            LATIN_CAPITAL_LETTER_U_WITH_TILDE_AND_ACUTE => Ok(LatinExtendedAdditional::LatinCapitalLetterUWithTildeAndAcute),
            LATIN_SMALL_LETTER_U_WITH_TILDE_AND_ACUTE => Ok(LatinExtendedAdditional::LatinSmallLetterUWithTildeAndAcute),
            LATIN_CAPITAL_LETTER_U_WITH_MACRON_AND_DIAERESIS => Ok(LatinExtendedAdditional::LatinCapitalLetterUWithMacronAndDiaeresis),
            LATIN_SMALL_LETTER_U_WITH_MACRON_AND_DIAERESIS => Ok(LatinExtendedAdditional::LatinSmallLetterUWithMacronAndDiaeresis),
            LATIN_CAPITAL_LETTER_V_WITH_TILDE => Ok(LatinExtendedAdditional::LatinCapitalLetterVWithTilde),
            LATIN_SMALL_LETTER_V_WITH_TILDE => Ok(LatinExtendedAdditional::LatinSmallLetterVWithTilde),
            LATIN_CAPITAL_LETTER_V_WITH_DOT_BELOW => Ok(LatinExtendedAdditional::LatinCapitalLetterVWithDotBelow),
            LATIN_SMALL_LETTER_V_WITH_DOT_BELOW => Ok(LatinExtendedAdditional::LatinSmallLetterVWithDotBelow),
            LATIN_CAPITAL_LETTER_W_WITH_GRAVE => Ok(LatinExtendedAdditional::LatinCapitalLetterWWithGrave),
            LATIN_SMALL_LETTER_W_WITH_GRAVE => Ok(LatinExtendedAdditional::LatinSmallLetterWWithGrave),
            LATIN_CAPITAL_LETTER_W_WITH_ACUTE => Ok(LatinExtendedAdditional::LatinCapitalLetterWWithAcute),
            LATIN_SMALL_LETTER_W_WITH_ACUTE => Ok(LatinExtendedAdditional::LatinSmallLetterWWithAcute),
            LATIN_CAPITAL_LETTER_W_WITH_DIAERESIS => Ok(LatinExtendedAdditional::LatinCapitalLetterWWithDiaeresis),
            LATIN_SMALL_LETTER_W_WITH_DIAERESIS => Ok(LatinExtendedAdditional::LatinSmallLetterWWithDiaeresis),
            LATIN_CAPITAL_LETTER_W_WITH_DOT_ABOVE => Ok(LatinExtendedAdditional::LatinCapitalLetterWWithDotAbove),
            LATIN_SMALL_LETTER_W_WITH_DOT_ABOVE => Ok(LatinExtendedAdditional::LatinSmallLetterWWithDotAbove),
            LATIN_CAPITAL_LETTER_W_WITH_DOT_BELOW => Ok(LatinExtendedAdditional::LatinCapitalLetterWWithDotBelow),
            LATIN_SMALL_LETTER_W_WITH_DOT_BELOW => Ok(LatinExtendedAdditional::LatinSmallLetterWWithDotBelow),
            LATIN_CAPITAL_LETTER_X_WITH_DOT_ABOVE => Ok(LatinExtendedAdditional::LatinCapitalLetterXWithDotAbove),
            LATIN_SMALL_LETTER_X_WITH_DOT_ABOVE => Ok(LatinExtendedAdditional::LatinSmallLetterXWithDotAbove),
            LATIN_CAPITAL_LETTER_X_WITH_DIAERESIS => Ok(LatinExtendedAdditional::LatinCapitalLetterXWithDiaeresis),
            LATIN_SMALL_LETTER_X_WITH_DIAERESIS => Ok(LatinExtendedAdditional::LatinSmallLetterXWithDiaeresis),
            LATIN_CAPITAL_LETTER_Y_WITH_DOT_ABOVE => Ok(LatinExtendedAdditional::LatinCapitalLetterYWithDotAbove),
            LATIN_SMALL_LETTER_Y_WITH_DOT_ABOVE => Ok(LatinExtendedAdditional::LatinSmallLetterYWithDotAbove),
            LATIN_CAPITAL_LETTER_Z_WITH_CIRCUMFLEX => Ok(LatinExtendedAdditional::LatinCapitalLetterZWithCircumflex),
            LATIN_SMALL_LETTER_Z_WITH_CIRCUMFLEX => Ok(LatinExtendedAdditional::LatinSmallLetterZWithCircumflex),
            LATIN_CAPITAL_LETTER_Z_WITH_DOT_BELOW => Ok(LatinExtendedAdditional::LatinCapitalLetterZWithDotBelow),
            LATIN_SMALL_LETTER_Z_WITH_DOT_BELOW => Ok(LatinExtendedAdditional::LatinSmallLetterZWithDotBelow),
            LATIN_CAPITAL_LETTER_Z_WITH_LINE_BELOW => Ok(LatinExtendedAdditional::LatinCapitalLetterZWithLineBelow),
            LATIN_SMALL_LETTER_Z_WITH_LINE_BELOW => Ok(LatinExtendedAdditional::LatinSmallLetterZWithLineBelow),
            LATIN_SMALL_LETTER_H_WITH_LINE_BELOW => Ok(LatinExtendedAdditional::LatinSmallLetterHWithLineBelow),
            LATIN_SMALL_LETTER_T_WITH_DIAERESIS => Ok(LatinExtendedAdditional::LatinSmallLetterTWithDiaeresis),
            LATIN_SMALL_LETTER_W_WITH_RING_ABOVE => Ok(LatinExtendedAdditional::LatinSmallLetterWWithRingAbove),
            LATIN_SMALL_LETTER_Y_WITH_RING_ABOVE => Ok(LatinExtendedAdditional::LatinSmallLetterYWithRingAbove),
            LATIN_SMALL_LETTER_A_WITH_RIGHT_HALF_RING => Ok(LatinExtendedAdditional::LatinSmallLetterAWithRightHalfRing),
            LATIN_SMALL_LETTER_LONG_S_WITH_DOT_ABOVE => Ok(LatinExtendedAdditional::LatinSmallLetterLongSWithDotAbove),
            LATIN_SMALL_LETTER_LONG_S_WITH_DIAGONAL_STROKE => Ok(LatinExtendedAdditional::LatinSmallLetterLongSWithDiagonalStroke),
            LATIN_SMALL_LETTER_LONG_S_WITH_HIGH_STROKE => Ok(LatinExtendedAdditional::LatinSmallLetterLongSWithHighStroke),
            LATIN_CAPITAL_LETTER_SHARP_S => Ok(LatinExtendedAdditional::LatinCapitalLetterSharpS),
            LATIN_SMALL_LETTER_DELTA => Ok(LatinExtendedAdditional::LatinSmallLetterDelta),
            LATIN_CAPITAL_LETTER_A_WITH_DOT_BELOW => Ok(LatinExtendedAdditional::LatinCapitalLetterAWithDotBelow),
            LATIN_SMALL_LETTER_A_WITH_DOT_BELOW => Ok(LatinExtendedAdditional::LatinSmallLetterAWithDotBelow),
            LATIN_CAPITAL_LETTER_A_WITH_HOOK_ABOVE => Ok(LatinExtendedAdditional::LatinCapitalLetterAWithHookAbove),
            LATIN_SMALL_LETTER_A_WITH_HOOK_ABOVE => Ok(LatinExtendedAdditional::LatinSmallLetterAWithHookAbove),
            LATIN_CAPITAL_LETTER_A_WITH_CIRCUMFLEX_AND_ACUTE => Ok(LatinExtendedAdditional::LatinCapitalLetterAWithCircumflexAndAcute),
            LATIN_SMALL_LETTER_A_WITH_CIRCUMFLEX_AND_ACUTE => Ok(LatinExtendedAdditional::LatinSmallLetterAWithCircumflexAndAcute),
            LATIN_CAPITAL_LETTER_A_WITH_CIRCUMFLEX_AND_GRAVE => Ok(LatinExtendedAdditional::LatinCapitalLetterAWithCircumflexAndGrave),
            LATIN_SMALL_LETTER_A_WITH_CIRCUMFLEX_AND_GRAVE => Ok(LatinExtendedAdditional::LatinSmallLetterAWithCircumflexAndGrave),
            LATIN_CAPITAL_LETTER_A_WITH_CIRCUMFLEX_AND_HOOK_ABOVE => Ok(LatinExtendedAdditional::LatinCapitalLetterAWithCircumflexAndHookAbove),
            LATIN_SMALL_LETTER_A_WITH_CIRCUMFLEX_AND_HOOK_ABOVE => Ok(LatinExtendedAdditional::LatinSmallLetterAWithCircumflexAndHookAbove),
            LATIN_CAPITAL_LETTER_A_WITH_CIRCUMFLEX_AND_TILDE => Ok(LatinExtendedAdditional::LatinCapitalLetterAWithCircumflexAndTilde),
            LATIN_SMALL_LETTER_A_WITH_CIRCUMFLEX_AND_TILDE => Ok(LatinExtendedAdditional::LatinSmallLetterAWithCircumflexAndTilde),
            LATIN_CAPITAL_LETTER_A_WITH_CIRCUMFLEX_AND_DOT_BELOW => Ok(LatinExtendedAdditional::LatinCapitalLetterAWithCircumflexAndDotBelow),
            LATIN_SMALL_LETTER_A_WITH_CIRCUMFLEX_AND_DOT_BELOW => Ok(LatinExtendedAdditional::LatinSmallLetterAWithCircumflexAndDotBelow),
            LATIN_CAPITAL_LETTER_A_WITH_BREVE_AND_ACUTE => Ok(LatinExtendedAdditional::LatinCapitalLetterAWithBreveAndAcute),
            LATIN_SMALL_LETTER_A_WITH_BREVE_AND_ACUTE => Ok(LatinExtendedAdditional::LatinSmallLetterAWithBreveAndAcute),
            LATIN_CAPITAL_LETTER_A_WITH_BREVE_AND_GRAVE => Ok(LatinExtendedAdditional::LatinCapitalLetterAWithBreveAndGrave),
            LATIN_SMALL_LETTER_A_WITH_BREVE_AND_GRAVE => Ok(LatinExtendedAdditional::LatinSmallLetterAWithBreveAndGrave),
            LATIN_CAPITAL_LETTER_A_WITH_BREVE_AND_HOOK_ABOVE => Ok(LatinExtendedAdditional::LatinCapitalLetterAWithBreveAndHookAbove),
            LATIN_SMALL_LETTER_A_WITH_BREVE_AND_HOOK_ABOVE => Ok(LatinExtendedAdditional::LatinSmallLetterAWithBreveAndHookAbove),
            LATIN_CAPITAL_LETTER_A_WITH_BREVE_AND_TILDE => Ok(LatinExtendedAdditional::LatinCapitalLetterAWithBreveAndTilde),
            LATIN_SMALL_LETTER_A_WITH_BREVE_AND_TILDE => Ok(LatinExtendedAdditional::LatinSmallLetterAWithBreveAndTilde),
            LATIN_CAPITAL_LETTER_A_WITH_BREVE_AND_DOT_BELOW => Ok(LatinExtendedAdditional::LatinCapitalLetterAWithBreveAndDotBelow),
            LATIN_SMALL_LETTER_A_WITH_BREVE_AND_DOT_BELOW => Ok(LatinExtendedAdditional::LatinSmallLetterAWithBreveAndDotBelow),
            LATIN_CAPITAL_LETTER_E_WITH_DOT_BELOW => Ok(LatinExtendedAdditional::LatinCapitalLetterEWithDotBelow),
            LATIN_SMALL_LETTER_E_WITH_DOT_BELOW => Ok(LatinExtendedAdditional::LatinSmallLetterEWithDotBelow),
            LATIN_CAPITAL_LETTER_E_WITH_HOOK_ABOVE => Ok(LatinExtendedAdditional::LatinCapitalLetterEWithHookAbove),
            LATIN_SMALL_LETTER_E_WITH_HOOK_ABOVE => Ok(LatinExtendedAdditional::LatinSmallLetterEWithHookAbove),
            LATIN_CAPITAL_LETTER_E_WITH_TILDE => Ok(LatinExtendedAdditional::LatinCapitalLetterEWithTilde),
            LATIN_SMALL_LETTER_E_WITH_TILDE => Ok(LatinExtendedAdditional::LatinSmallLetterEWithTilde),
            LATIN_CAPITAL_LETTER_E_WITH_CIRCUMFLEX_AND_ACUTE => Ok(LatinExtendedAdditional::LatinCapitalLetterEWithCircumflexAndAcute),
            LATIN_SMALL_LETTER_E_WITH_CIRCUMFLEX_AND_ACUTE => Ok(LatinExtendedAdditional::LatinSmallLetterEWithCircumflexAndAcute),
            LATIN_CAPITAL_LETTER_E_WITH_CIRCUMFLEX_AND_GRAVE => Ok(LatinExtendedAdditional::LatinCapitalLetterEWithCircumflexAndGrave),
            LATIN_SMALL_LETTER_E_WITH_CIRCUMFLEX_AND_GRAVE => Ok(LatinExtendedAdditional::LatinSmallLetterEWithCircumflexAndGrave),
            LATIN_CAPITAL_LETTER_E_WITH_CIRCUMFLEX_AND_HOOK_ABOVE => Ok(LatinExtendedAdditional::LatinCapitalLetterEWithCircumflexAndHookAbove),
            LATIN_SMALL_LETTER_E_WITH_CIRCUMFLEX_AND_HOOK_ABOVE => Ok(LatinExtendedAdditional::LatinSmallLetterEWithCircumflexAndHookAbove),
            LATIN_CAPITAL_LETTER_E_WITH_CIRCUMFLEX_AND_TILDE => Ok(LatinExtendedAdditional::LatinCapitalLetterEWithCircumflexAndTilde),
            LATIN_SMALL_LETTER_E_WITH_CIRCUMFLEX_AND_TILDE => Ok(LatinExtendedAdditional::LatinSmallLetterEWithCircumflexAndTilde),
            LATIN_CAPITAL_LETTER_E_WITH_CIRCUMFLEX_AND_DOT_BELOW => Ok(LatinExtendedAdditional::LatinCapitalLetterEWithCircumflexAndDotBelow),
            LATIN_SMALL_LETTER_E_WITH_CIRCUMFLEX_AND_DOT_BELOW => Ok(LatinExtendedAdditional::LatinSmallLetterEWithCircumflexAndDotBelow),
            LATIN_CAPITAL_LETTER_I_WITH_HOOK_ABOVE => Ok(LatinExtendedAdditional::LatinCapitalLetterIWithHookAbove),
            LATIN_SMALL_LETTER_I_WITH_HOOK_ABOVE => Ok(LatinExtendedAdditional::LatinSmallLetterIWithHookAbove),
            LATIN_CAPITAL_LETTER_I_WITH_DOT_BELOW => Ok(LatinExtendedAdditional::LatinCapitalLetterIWithDotBelow),
            LATIN_SMALL_LETTER_I_WITH_DOT_BELOW => Ok(LatinExtendedAdditional::LatinSmallLetterIWithDotBelow),
            LATIN_CAPITAL_LETTER_O_WITH_DOT_BELOW => Ok(LatinExtendedAdditional::LatinCapitalLetterOWithDotBelow),
            LATIN_SMALL_LETTER_O_WITH_DOT_BELOW => Ok(LatinExtendedAdditional::LatinSmallLetterOWithDotBelow),
            LATIN_CAPITAL_LETTER_O_WITH_HOOK_ABOVE => Ok(LatinExtendedAdditional::LatinCapitalLetterOWithHookAbove),
            LATIN_SMALL_LETTER_O_WITH_HOOK_ABOVE => Ok(LatinExtendedAdditional::LatinSmallLetterOWithHookAbove),
            LATIN_CAPITAL_LETTER_O_WITH_CIRCUMFLEX_AND_ACUTE => Ok(LatinExtendedAdditional::LatinCapitalLetterOWithCircumflexAndAcute),
            LATIN_SMALL_LETTER_O_WITH_CIRCUMFLEX_AND_ACUTE => Ok(LatinExtendedAdditional::LatinSmallLetterOWithCircumflexAndAcute),
            LATIN_CAPITAL_LETTER_O_WITH_CIRCUMFLEX_AND_GRAVE => Ok(LatinExtendedAdditional::LatinCapitalLetterOWithCircumflexAndGrave),
            LATIN_SMALL_LETTER_O_WITH_CIRCUMFLEX_AND_GRAVE => Ok(LatinExtendedAdditional::LatinSmallLetterOWithCircumflexAndGrave),
            LATIN_CAPITAL_LETTER_O_WITH_CIRCUMFLEX_AND_HOOK_ABOVE => Ok(LatinExtendedAdditional::LatinCapitalLetterOWithCircumflexAndHookAbove),
            LATIN_SMALL_LETTER_O_WITH_CIRCUMFLEX_AND_HOOK_ABOVE => Ok(LatinExtendedAdditional::LatinSmallLetterOWithCircumflexAndHookAbove),
            LATIN_CAPITAL_LETTER_O_WITH_CIRCUMFLEX_AND_TILDE => Ok(LatinExtendedAdditional::LatinCapitalLetterOWithCircumflexAndTilde),
            LATIN_SMALL_LETTER_O_WITH_CIRCUMFLEX_AND_TILDE => Ok(LatinExtendedAdditional::LatinSmallLetterOWithCircumflexAndTilde),
            LATIN_CAPITAL_LETTER_O_WITH_CIRCUMFLEX_AND_DOT_BELOW => Ok(LatinExtendedAdditional::LatinCapitalLetterOWithCircumflexAndDotBelow),
            LATIN_SMALL_LETTER_O_WITH_CIRCUMFLEX_AND_DOT_BELOW => Ok(LatinExtendedAdditional::LatinSmallLetterOWithCircumflexAndDotBelow),
            LATIN_CAPITAL_LETTER_O_WITH_HORN_AND_ACUTE => Ok(LatinExtendedAdditional::LatinCapitalLetterOWithHornAndAcute),
            LATIN_SMALL_LETTER_O_WITH_HORN_AND_ACUTE => Ok(LatinExtendedAdditional::LatinSmallLetterOWithHornAndAcute),
            LATIN_CAPITAL_LETTER_O_WITH_HORN_AND_GRAVE => Ok(LatinExtendedAdditional::LatinCapitalLetterOWithHornAndGrave),
            LATIN_SMALL_LETTER_O_WITH_HORN_AND_GRAVE => Ok(LatinExtendedAdditional::LatinSmallLetterOWithHornAndGrave),
            LATIN_CAPITAL_LETTER_O_WITH_HORN_AND_HOOK_ABOVE => Ok(LatinExtendedAdditional::LatinCapitalLetterOWithHornAndHookAbove),
            LATIN_SMALL_LETTER_O_WITH_HORN_AND_HOOK_ABOVE => Ok(LatinExtendedAdditional::LatinSmallLetterOWithHornAndHookAbove),
            LATIN_CAPITAL_LETTER_O_WITH_HORN_AND_TILDE => Ok(LatinExtendedAdditional::LatinCapitalLetterOWithHornAndTilde),
            LATIN_SMALL_LETTER_O_WITH_HORN_AND_TILDE => Ok(LatinExtendedAdditional::LatinSmallLetterOWithHornAndTilde),
            LATIN_CAPITAL_LETTER_O_WITH_HORN_AND_DOT_BELOW => Ok(LatinExtendedAdditional::LatinCapitalLetterOWithHornAndDotBelow),
            LATIN_SMALL_LETTER_O_WITH_HORN_AND_DOT_BELOW => Ok(LatinExtendedAdditional::LatinSmallLetterOWithHornAndDotBelow),
            LATIN_CAPITAL_LETTER_U_WITH_DOT_BELOW => Ok(LatinExtendedAdditional::LatinCapitalLetterUWithDotBelow),
            LATIN_SMALL_LETTER_U_WITH_DOT_BELOW => Ok(LatinExtendedAdditional::LatinSmallLetterUWithDotBelow),
            LATIN_CAPITAL_LETTER_U_WITH_HOOK_ABOVE => Ok(LatinExtendedAdditional::LatinCapitalLetterUWithHookAbove),
            LATIN_SMALL_LETTER_U_WITH_HOOK_ABOVE => Ok(LatinExtendedAdditional::LatinSmallLetterUWithHookAbove),
            LATIN_CAPITAL_LETTER_U_WITH_HORN_AND_ACUTE => Ok(LatinExtendedAdditional::LatinCapitalLetterUWithHornAndAcute),
            LATIN_SMALL_LETTER_U_WITH_HORN_AND_ACUTE => Ok(LatinExtendedAdditional::LatinSmallLetterUWithHornAndAcute),
            LATIN_CAPITAL_LETTER_U_WITH_HORN_AND_GRAVE => Ok(LatinExtendedAdditional::LatinCapitalLetterUWithHornAndGrave),
            LATIN_SMALL_LETTER_U_WITH_HORN_AND_GRAVE => Ok(LatinExtendedAdditional::LatinSmallLetterUWithHornAndGrave),
            LATIN_CAPITAL_LETTER_U_WITH_HORN_AND_HOOK_ABOVE => Ok(LatinExtendedAdditional::LatinCapitalLetterUWithHornAndHookAbove),
            LATIN_SMALL_LETTER_U_WITH_HORN_AND_HOOK_ABOVE => Ok(LatinExtendedAdditional::LatinSmallLetterUWithHornAndHookAbove),
            LATIN_CAPITAL_LETTER_U_WITH_HORN_AND_TILDE => Ok(LatinExtendedAdditional::LatinCapitalLetterUWithHornAndTilde),
            LATIN_SMALL_LETTER_U_WITH_HORN_AND_TILDE => Ok(LatinExtendedAdditional::LatinSmallLetterUWithHornAndTilde),
            LATIN_CAPITAL_LETTER_U_WITH_HORN_AND_DOT_BELOW => Ok(LatinExtendedAdditional::LatinCapitalLetterUWithHornAndDotBelow),
            LATIN_SMALL_LETTER_U_WITH_HORN_AND_DOT_BELOW => Ok(LatinExtendedAdditional::LatinSmallLetterUWithHornAndDotBelow),
            LATIN_CAPITAL_LETTER_Y_WITH_GRAVE => Ok(LatinExtendedAdditional::LatinCapitalLetterYWithGrave),
            LATIN_SMALL_LETTER_Y_WITH_GRAVE => Ok(LatinExtendedAdditional::LatinSmallLetterYWithGrave),
            LATIN_CAPITAL_LETTER_Y_WITH_DOT_BELOW => Ok(LatinExtendedAdditional::LatinCapitalLetterYWithDotBelow),
            LATIN_SMALL_LETTER_Y_WITH_DOT_BELOW => Ok(LatinExtendedAdditional::LatinSmallLetterYWithDotBelow),
            LATIN_CAPITAL_LETTER_Y_WITH_HOOK_ABOVE => Ok(LatinExtendedAdditional::LatinCapitalLetterYWithHookAbove),
            LATIN_SMALL_LETTER_Y_WITH_HOOK_ABOVE => Ok(LatinExtendedAdditional::LatinSmallLetterYWithHookAbove),
            LATIN_CAPITAL_LETTER_Y_WITH_TILDE => Ok(LatinExtendedAdditional::LatinCapitalLetterYWithTilde),
            LATIN_SMALL_LETTER_Y_WITH_TILDE => Ok(LatinExtendedAdditional::LatinSmallLetterYWithTilde),
            LATIN_CAPITAL_LETTER_MIDDLE_DASH_WELSH_LL => Ok(LatinExtendedAdditional::LatinCapitalLetterMiddleDashWelshLl),
            LATIN_SMALL_LETTER_MIDDLE_DASH_WELSH_LL => Ok(LatinExtendedAdditional::LatinSmallLetterMiddleDashWelshLl),
            LATIN_CAPITAL_LETTER_MIDDLE_DASH_WELSH_V => Ok(LatinExtendedAdditional::LatinCapitalLetterMiddleDashWelshV),
            LATIN_SMALL_LETTER_MIDDLE_DASH_WELSH_V => Ok(LatinExtendedAdditional::LatinSmallLetterMiddleDashWelshV),
            LATIN_CAPITAL_LETTER_Y_WITH_LOOP => Ok(LatinExtendedAdditional::LatinCapitalLetterYWithLoop),
            _ => Err(()),
        }
    }
}

impl Into<u32> for LatinExtendedAdditional {
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

impl std::convert::TryFrom<u32> for LatinExtendedAdditional {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for LatinExtendedAdditional {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl LatinExtendedAdditional {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        LatinExtendedAdditional::LatinCapitalLetterAWithRingBelow
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            LatinExtendedAdditional::LatinCapitalLetterAWithRingBelow => "latin capital letter a with ring below",
            LatinExtendedAdditional::LatinSmallLetterAWithRingBelow => "latin small letter a with ring below",
            LatinExtendedAdditional::LatinCapitalLetterBWithDotAbove => "latin capital letter b with dot above",
            LatinExtendedAdditional::LatinSmallLetterBWithDotAbove => "latin small letter b with dot above",
            LatinExtendedAdditional::LatinCapitalLetterBWithDotBelow => "latin capital letter b with dot below",
            LatinExtendedAdditional::LatinSmallLetterBWithDotBelow => "latin small letter b with dot below",
            LatinExtendedAdditional::LatinCapitalLetterBWithLineBelow => "latin capital letter b with line below",
            LatinExtendedAdditional::LatinSmallLetterBWithLineBelow => "latin small letter b with line below",
            LatinExtendedAdditional::LatinCapitalLetterCWithCedillaAndAcute => "latin capital letter c with cedilla and acute",
            LatinExtendedAdditional::LatinSmallLetterCWithCedillaAndAcute => "latin small letter c with cedilla and acute",
            LatinExtendedAdditional::LatinCapitalLetterDWithDotAbove => "latin capital letter d with dot above",
            LatinExtendedAdditional::LatinSmallLetterDWithDotAbove => "latin small letter d with dot above",
            LatinExtendedAdditional::LatinCapitalLetterDWithDotBelow => "latin capital letter d with dot below",
            LatinExtendedAdditional::LatinSmallLetterDWithDotBelow => "latin small letter d with dot below",
            LatinExtendedAdditional::LatinCapitalLetterDWithLineBelow => "latin capital letter d with line below",
            LatinExtendedAdditional::LatinSmallLetterDWithLineBelow => "latin small letter d with line below",
            LatinExtendedAdditional::LatinCapitalLetterDWithCedilla => "latin capital letter d with cedilla",
            LatinExtendedAdditional::LatinSmallLetterDWithCedilla => "latin small letter d with cedilla",
            LatinExtendedAdditional::LatinCapitalLetterDWithCircumflexBelow => "latin capital letter d with circumflex below",
            LatinExtendedAdditional::LatinSmallLetterDWithCircumflexBelow => "latin small letter d with circumflex below",
            LatinExtendedAdditional::LatinCapitalLetterEWithMacronAndGrave => "latin capital letter e with macron and grave",
            LatinExtendedAdditional::LatinSmallLetterEWithMacronAndGrave => "latin small letter e with macron and grave",
            LatinExtendedAdditional::LatinCapitalLetterEWithMacronAndAcute => "latin capital letter e with macron and acute",
            LatinExtendedAdditional::LatinSmallLetterEWithMacronAndAcute => "latin small letter e with macron and acute",
            LatinExtendedAdditional::LatinCapitalLetterEWithCircumflexBelow => "latin capital letter e with circumflex below",
            LatinExtendedAdditional::LatinSmallLetterEWithCircumflexBelow => "latin small letter e with circumflex below",
            LatinExtendedAdditional::LatinCapitalLetterEWithTildeBelow => "latin capital letter e with tilde below",
            LatinExtendedAdditional::LatinSmallLetterEWithTildeBelow => "latin small letter e with tilde below",
            LatinExtendedAdditional::LatinCapitalLetterEWithCedillaAndBreve => "latin capital letter e with cedilla and breve",
            LatinExtendedAdditional::LatinSmallLetterEWithCedillaAndBreve => "latin small letter e with cedilla and breve",
            LatinExtendedAdditional::LatinCapitalLetterFWithDotAbove => "latin capital letter f with dot above",
            LatinExtendedAdditional::LatinSmallLetterFWithDotAbove => "latin small letter f with dot above",
            LatinExtendedAdditional::LatinCapitalLetterGWithMacron => "latin capital letter g with macron",
            LatinExtendedAdditional::LatinSmallLetterGWithMacron => "latin small letter g with macron",
            LatinExtendedAdditional::LatinCapitalLetterHWithDotAbove => "latin capital letter h with dot above",
            LatinExtendedAdditional::LatinSmallLetterHWithDotAbove => "latin small letter h with dot above",
            LatinExtendedAdditional::LatinCapitalLetterHWithDotBelow => "latin capital letter h with dot below",
            LatinExtendedAdditional::LatinSmallLetterHWithDotBelow => "latin small letter h with dot below",
            LatinExtendedAdditional::LatinCapitalLetterHWithDiaeresis => "latin capital letter h with diaeresis",
            LatinExtendedAdditional::LatinSmallLetterHWithDiaeresis => "latin small letter h with diaeresis",
            LatinExtendedAdditional::LatinCapitalLetterHWithCedilla => "latin capital letter h with cedilla",
            LatinExtendedAdditional::LatinSmallLetterHWithCedilla => "latin small letter h with cedilla",
            LatinExtendedAdditional::LatinCapitalLetterHWithBreveBelow => "latin capital letter h with breve below",
            LatinExtendedAdditional::LatinSmallLetterHWithBreveBelow => "latin small letter h with breve below",
            LatinExtendedAdditional::LatinCapitalLetterIWithTildeBelow => "latin capital letter i with tilde below",
            LatinExtendedAdditional::LatinSmallLetterIWithTildeBelow => "latin small letter i with tilde below",
            LatinExtendedAdditional::LatinCapitalLetterIWithDiaeresisAndAcute => "latin capital letter i with diaeresis and acute",
            LatinExtendedAdditional::LatinSmallLetterIWithDiaeresisAndAcute => "latin small letter i with diaeresis and acute",
            LatinExtendedAdditional::LatinCapitalLetterKWithAcute => "latin capital letter k with acute",
            LatinExtendedAdditional::LatinSmallLetterKWithAcute => "latin small letter k with acute",
            LatinExtendedAdditional::LatinCapitalLetterKWithDotBelow => "latin capital letter k with dot below",
            LatinExtendedAdditional::LatinSmallLetterKWithDotBelow => "latin small letter k with dot below",
            LatinExtendedAdditional::LatinCapitalLetterKWithLineBelow => "latin capital letter k with line below",
            LatinExtendedAdditional::LatinSmallLetterKWithLineBelow => "latin small letter k with line below",
            LatinExtendedAdditional::LatinCapitalLetterLWithDotBelow => "latin capital letter l with dot below",
            LatinExtendedAdditional::LatinSmallLetterLWithDotBelow => "latin small letter l with dot below",
            LatinExtendedAdditional::LatinCapitalLetterLWithDotBelowAndMacron => "latin capital letter l with dot below and macron",
            LatinExtendedAdditional::LatinSmallLetterLWithDotBelowAndMacron => "latin small letter l with dot below and macron",
            LatinExtendedAdditional::LatinCapitalLetterLWithLineBelow => "latin capital letter l with line below",
            LatinExtendedAdditional::LatinSmallLetterLWithLineBelow => "latin small letter l with line below",
            LatinExtendedAdditional::LatinCapitalLetterLWithCircumflexBelow => "latin capital letter l with circumflex below",
            LatinExtendedAdditional::LatinSmallLetterLWithCircumflexBelow => "latin small letter l with circumflex below",
            LatinExtendedAdditional::LatinCapitalLetterMWithAcute => "latin capital letter m with acute",
            LatinExtendedAdditional::LatinSmallLetterMWithAcute => "latin small letter m with acute",
            LatinExtendedAdditional::LatinCapitalLetterMWithDotAbove => "latin capital letter m with dot above",
            LatinExtendedAdditional::LatinSmallLetterMWithDotAbove => "latin small letter m with dot above",
            LatinExtendedAdditional::LatinCapitalLetterMWithDotBelow => "latin capital letter m with dot below",
            LatinExtendedAdditional::LatinSmallLetterMWithDotBelow => "latin small letter m with dot below",
            LatinExtendedAdditional::LatinCapitalLetterNWithDotAbove => "latin capital letter n with dot above",
            LatinExtendedAdditional::LatinSmallLetterNWithDotAbove => "latin small letter n with dot above",
            LatinExtendedAdditional::LatinCapitalLetterNWithDotBelow => "latin capital letter n with dot below",
            LatinExtendedAdditional::LatinSmallLetterNWithDotBelow => "latin small letter n with dot below",
            LatinExtendedAdditional::LatinCapitalLetterNWithLineBelow => "latin capital letter n with line below",
            LatinExtendedAdditional::LatinSmallLetterNWithLineBelow => "latin small letter n with line below",
            LatinExtendedAdditional::LatinCapitalLetterNWithCircumflexBelow => "latin capital letter n with circumflex below",
            LatinExtendedAdditional::LatinSmallLetterNWithCircumflexBelow => "latin small letter n with circumflex below",
            LatinExtendedAdditional::LatinCapitalLetterOWithTildeAndAcute => "latin capital letter o with tilde and acute",
            LatinExtendedAdditional::LatinSmallLetterOWithTildeAndAcute => "latin small letter o with tilde and acute",
            LatinExtendedAdditional::LatinCapitalLetterOWithTildeAndDiaeresis => "latin capital letter o with tilde and diaeresis",
            LatinExtendedAdditional::LatinSmallLetterOWithTildeAndDiaeresis => "latin small letter o with tilde and diaeresis",
            LatinExtendedAdditional::LatinCapitalLetterOWithMacronAndGrave => "latin capital letter o with macron and grave",
            LatinExtendedAdditional::LatinSmallLetterOWithMacronAndGrave => "latin small letter o with macron and grave",
            LatinExtendedAdditional::LatinCapitalLetterOWithMacronAndAcute => "latin capital letter o with macron and acute",
            LatinExtendedAdditional::LatinSmallLetterOWithMacronAndAcute => "latin small letter o with macron and acute",
            LatinExtendedAdditional::LatinCapitalLetterPWithAcute => "latin capital letter p with acute",
            LatinExtendedAdditional::LatinSmallLetterPWithAcute => "latin small letter p with acute",
            LatinExtendedAdditional::LatinCapitalLetterPWithDotAbove => "latin capital letter p with dot above",
            LatinExtendedAdditional::LatinSmallLetterPWithDotAbove => "latin small letter p with dot above",
            LatinExtendedAdditional::LatinCapitalLetterRWithDotAbove => "latin capital letter r with dot above",
            LatinExtendedAdditional::LatinSmallLetterRWithDotAbove => "latin small letter r with dot above",
            LatinExtendedAdditional::LatinCapitalLetterRWithDotBelow => "latin capital letter r with dot below",
            LatinExtendedAdditional::LatinSmallLetterRWithDotBelow => "latin small letter r with dot below",
            LatinExtendedAdditional::LatinCapitalLetterRWithDotBelowAndMacron => "latin capital letter r with dot below and macron",
            LatinExtendedAdditional::LatinSmallLetterRWithDotBelowAndMacron => "latin small letter r with dot below and macron",
            LatinExtendedAdditional::LatinCapitalLetterRWithLineBelow => "latin capital letter r with line below",
            LatinExtendedAdditional::LatinSmallLetterRWithLineBelow => "latin small letter r with line below",
            LatinExtendedAdditional::LatinCapitalLetterSWithDotAbove => "latin capital letter s with dot above",
            LatinExtendedAdditional::LatinSmallLetterSWithDotAbove => "latin small letter s with dot above",
            LatinExtendedAdditional::LatinCapitalLetterSWithDotBelow => "latin capital letter s with dot below",
            LatinExtendedAdditional::LatinSmallLetterSWithDotBelow => "latin small letter s with dot below",
            LatinExtendedAdditional::LatinCapitalLetterSWithAcuteAndDotAbove => "latin capital letter s with acute and dot above",
            LatinExtendedAdditional::LatinSmallLetterSWithAcuteAndDotAbove => "latin small letter s with acute and dot above",
            LatinExtendedAdditional::LatinCapitalLetterSWithCaronAndDotAbove => "latin capital letter s with caron and dot above",
            LatinExtendedAdditional::LatinSmallLetterSWithCaronAndDotAbove => "latin small letter s with caron and dot above",
            LatinExtendedAdditional::LatinCapitalLetterSWithDotBelowAndDotAbove => "latin capital letter s with dot below and dot above",
            LatinExtendedAdditional::LatinSmallLetterSWithDotBelowAndDotAbove => "latin small letter s with dot below and dot above",
            LatinExtendedAdditional::LatinCapitalLetterTWithDotAbove => "latin capital letter t with dot above",
            LatinExtendedAdditional::LatinSmallLetterTWithDotAbove => "latin small letter t with dot above",
            LatinExtendedAdditional::LatinCapitalLetterTWithDotBelow => "latin capital letter t with dot below",
            LatinExtendedAdditional::LatinSmallLetterTWithDotBelow => "latin small letter t with dot below",
            LatinExtendedAdditional::LatinCapitalLetterTWithLineBelow => "latin capital letter t with line below",
            LatinExtendedAdditional::LatinSmallLetterTWithLineBelow => "latin small letter t with line below",
            LatinExtendedAdditional::LatinCapitalLetterTWithCircumflexBelow => "latin capital letter t with circumflex below",
            LatinExtendedAdditional::LatinSmallLetterTWithCircumflexBelow => "latin small letter t with circumflex below",
            LatinExtendedAdditional::LatinCapitalLetterUWithDiaeresisBelow => "latin capital letter u with diaeresis below",
            LatinExtendedAdditional::LatinSmallLetterUWithDiaeresisBelow => "latin small letter u with diaeresis below",
            LatinExtendedAdditional::LatinCapitalLetterUWithTildeBelow => "latin capital letter u with tilde below",
            LatinExtendedAdditional::LatinSmallLetterUWithTildeBelow => "latin small letter u with tilde below",
            LatinExtendedAdditional::LatinCapitalLetterUWithCircumflexBelow => "latin capital letter u with circumflex below",
            LatinExtendedAdditional::LatinSmallLetterUWithCircumflexBelow => "latin small letter u with circumflex below",
            LatinExtendedAdditional::LatinCapitalLetterUWithTildeAndAcute => "latin capital letter u with tilde and acute",
            LatinExtendedAdditional::LatinSmallLetterUWithTildeAndAcute => "latin small letter u with tilde and acute",
            LatinExtendedAdditional::LatinCapitalLetterUWithMacronAndDiaeresis => "latin capital letter u with macron and diaeresis",
            LatinExtendedAdditional::LatinSmallLetterUWithMacronAndDiaeresis => "latin small letter u with macron and diaeresis",
            LatinExtendedAdditional::LatinCapitalLetterVWithTilde => "latin capital letter v with tilde",
            LatinExtendedAdditional::LatinSmallLetterVWithTilde => "latin small letter v with tilde",
            LatinExtendedAdditional::LatinCapitalLetterVWithDotBelow => "latin capital letter v with dot below",
            LatinExtendedAdditional::LatinSmallLetterVWithDotBelow => "latin small letter v with dot below",
            LatinExtendedAdditional::LatinCapitalLetterWWithGrave => "latin capital letter w with grave",
            LatinExtendedAdditional::LatinSmallLetterWWithGrave => "latin small letter w with grave",
            LatinExtendedAdditional::LatinCapitalLetterWWithAcute => "latin capital letter w with acute",
            LatinExtendedAdditional::LatinSmallLetterWWithAcute => "latin small letter w with acute",
            LatinExtendedAdditional::LatinCapitalLetterWWithDiaeresis => "latin capital letter w with diaeresis",
            LatinExtendedAdditional::LatinSmallLetterWWithDiaeresis => "latin small letter w with diaeresis",
            LatinExtendedAdditional::LatinCapitalLetterWWithDotAbove => "latin capital letter w with dot above",
            LatinExtendedAdditional::LatinSmallLetterWWithDotAbove => "latin small letter w with dot above",
            LatinExtendedAdditional::LatinCapitalLetterWWithDotBelow => "latin capital letter w with dot below",
            LatinExtendedAdditional::LatinSmallLetterWWithDotBelow => "latin small letter w with dot below",
            LatinExtendedAdditional::LatinCapitalLetterXWithDotAbove => "latin capital letter x with dot above",
            LatinExtendedAdditional::LatinSmallLetterXWithDotAbove => "latin small letter x with dot above",
            LatinExtendedAdditional::LatinCapitalLetterXWithDiaeresis => "latin capital letter x with diaeresis",
            LatinExtendedAdditional::LatinSmallLetterXWithDiaeresis => "latin small letter x with diaeresis",
            LatinExtendedAdditional::LatinCapitalLetterYWithDotAbove => "latin capital letter y with dot above",
            LatinExtendedAdditional::LatinSmallLetterYWithDotAbove => "latin small letter y with dot above",
            LatinExtendedAdditional::LatinCapitalLetterZWithCircumflex => "latin capital letter z with circumflex",
            LatinExtendedAdditional::LatinSmallLetterZWithCircumflex => "latin small letter z with circumflex",
            LatinExtendedAdditional::LatinCapitalLetterZWithDotBelow => "latin capital letter z with dot below",
            LatinExtendedAdditional::LatinSmallLetterZWithDotBelow => "latin small letter z with dot below",
            LatinExtendedAdditional::LatinCapitalLetterZWithLineBelow => "latin capital letter z with line below",
            LatinExtendedAdditional::LatinSmallLetterZWithLineBelow => "latin small letter z with line below",
            LatinExtendedAdditional::LatinSmallLetterHWithLineBelow => "latin small letter h with line below",
            LatinExtendedAdditional::LatinSmallLetterTWithDiaeresis => "latin small letter t with diaeresis",
            LatinExtendedAdditional::LatinSmallLetterWWithRingAbove => "latin small letter w with ring above",
            LatinExtendedAdditional::LatinSmallLetterYWithRingAbove => "latin small letter y with ring above",
            LatinExtendedAdditional::LatinSmallLetterAWithRightHalfRing => "latin small letter a with right half ring",
            LatinExtendedAdditional::LatinSmallLetterLongSWithDotAbove => "latin small letter long s with dot above",
            LatinExtendedAdditional::LatinSmallLetterLongSWithDiagonalStroke => "latin small letter long s with diagonal stroke",
            LatinExtendedAdditional::LatinSmallLetterLongSWithHighStroke => "latin small letter long s with high stroke",
            LatinExtendedAdditional::LatinCapitalLetterSharpS => "latin capital letter sharp s",
            LatinExtendedAdditional::LatinSmallLetterDelta => "latin small letter delta",
            LatinExtendedAdditional::LatinCapitalLetterAWithDotBelow => "latin capital letter a with dot below",
            LatinExtendedAdditional::LatinSmallLetterAWithDotBelow => "latin small letter a with dot below",
            LatinExtendedAdditional::LatinCapitalLetterAWithHookAbove => "latin capital letter a with hook above",
            LatinExtendedAdditional::LatinSmallLetterAWithHookAbove => "latin small letter a with hook above",
            LatinExtendedAdditional::LatinCapitalLetterAWithCircumflexAndAcute => "latin capital letter a with circumflex and acute",
            LatinExtendedAdditional::LatinSmallLetterAWithCircumflexAndAcute => "latin small letter a with circumflex and acute",
            LatinExtendedAdditional::LatinCapitalLetterAWithCircumflexAndGrave => "latin capital letter a with circumflex and grave",
            LatinExtendedAdditional::LatinSmallLetterAWithCircumflexAndGrave => "latin small letter a with circumflex and grave",
            LatinExtendedAdditional::LatinCapitalLetterAWithCircumflexAndHookAbove => "latin capital letter a with circumflex and hook above",
            LatinExtendedAdditional::LatinSmallLetterAWithCircumflexAndHookAbove => "latin small letter a with circumflex and hook above",
            LatinExtendedAdditional::LatinCapitalLetterAWithCircumflexAndTilde => "latin capital letter a with circumflex and tilde",
            LatinExtendedAdditional::LatinSmallLetterAWithCircumflexAndTilde => "latin small letter a with circumflex and tilde",
            LatinExtendedAdditional::LatinCapitalLetterAWithCircumflexAndDotBelow => "latin capital letter a with circumflex and dot below",
            LatinExtendedAdditional::LatinSmallLetterAWithCircumflexAndDotBelow => "latin small letter a with circumflex and dot below",
            LatinExtendedAdditional::LatinCapitalLetterAWithBreveAndAcute => "latin capital letter a with breve and acute",
            LatinExtendedAdditional::LatinSmallLetterAWithBreveAndAcute => "latin small letter a with breve and acute",
            LatinExtendedAdditional::LatinCapitalLetterAWithBreveAndGrave => "latin capital letter a with breve and grave",
            LatinExtendedAdditional::LatinSmallLetterAWithBreveAndGrave => "latin small letter a with breve and grave",
            LatinExtendedAdditional::LatinCapitalLetterAWithBreveAndHookAbove => "latin capital letter a with breve and hook above",
            LatinExtendedAdditional::LatinSmallLetterAWithBreveAndHookAbove => "latin small letter a with breve and hook above",
            LatinExtendedAdditional::LatinCapitalLetterAWithBreveAndTilde => "latin capital letter a with breve and tilde",
            LatinExtendedAdditional::LatinSmallLetterAWithBreveAndTilde => "latin small letter a with breve and tilde",
            LatinExtendedAdditional::LatinCapitalLetterAWithBreveAndDotBelow => "latin capital letter a with breve and dot below",
            LatinExtendedAdditional::LatinSmallLetterAWithBreveAndDotBelow => "latin small letter a with breve and dot below",
            LatinExtendedAdditional::LatinCapitalLetterEWithDotBelow => "latin capital letter e with dot below",
            LatinExtendedAdditional::LatinSmallLetterEWithDotBelow => "latin small letter e with dot below",
            LatinExtendedAdditional::LatinCapitalLetterEWithHookAbove => "latin capital letter e with hook above",
            LatinExtendedAdditional::LatinSmallLetterEWithHookAbove => "latin small letter e with hook above",
            LatinExtendedAdditional::LatinCapitalLetterEWithTilde => "latin capital letter e with tilde",
            LatinExtendedAdditional::LatinSmallLetterEWithTilde => "latin small letter e with tilde",
            LatinExtendedAdditional::LatinCapitalLetterEWithCircumflexAndAcute => "latin capital letter e with circumflex and acute",
            LatinExtendedAdditional::LatinSmallLetterEWithCircumflexAndAcute => "latin small letter e with circumflex and acute",
            LatinExtendedAdditional::LatinCapitalLetterEWithCircumflexAndGrave => "latin capital letter e with circumflex and grave",
            LatinExtendedAdditional::LatinSmallLetterEWithCircumflexAndGrave => "latin small letter e with circumflex and grave",
            LatinExtendedAdditional::LatinCapitalLetterEWithCircumflexAndHookAbove => "latin capital letter e with circumflex and hook above",
            LatinExtendedAdditional::LatinSmallLetterEWithCircumflexAndHookAbove => "latin small letter e with circumflex and hook above",
            LatinExtendedAdditional::LatinCapitalLetterEWithCircumflexAndTilde => "latin capital letter e with circumflex and tilde",
            LatinExtendedAdditional::LatinSmallLetterEWithCircumflexAndTilde => "latin small letter e with circumflex and tilde",
            LatinExtendedAdditional::LatinCapitalLetterEWithCircumflexAndDotBelow => "latin capital letter e with circumflex and dot below",
            LatinExtendedAdditional::LatinSmallLetterEWithCircumflexAndDotBelow => "latin small letter e with circumflex and dot below",
            LatinExtendedAdditional::LatinCapitalLetterIWithHookAbove => "latin capital letter i with hook above",
            LatinExtendedAdditional::LatinSmallLetterIWithHookAbove => "latin small letter i with hook above",
            LatinExtendedAdditional::LatinCapitalLetterIWithDotBelow => "latin capital letter i with dot below",
            LatinExtendedAdditional::LatinSmallLetterIWithDotBelow => "latin small letter i with dot below",
            LatinExtendedAdditional::LatinCapitalLetterOWithDotBelow => "latin capital letter o with dot below",
            LatinExtendedAdditional::LatinSmallLetterOWithDotBelow => "latin small letter o with dot below",
            LatinExtendedAdditional::LatinCapitalLetterOWithHookAbove => "latin capital letter o with hook above",
            LatinExtendedAdditional::LatinSmallLetterOWithHookAbove => "latin small letter o with hook above",
            LatinExtendedAdditional::LatinCapitalLetterOWithCircumflexAndAcute => "latin capital letter o with circumflex and acute",
            LatinExtendedAdditional::LatinSmallLetterOWithCircumflexAndAcute => "latin small letter o with circumflex and acute",
            LatinExtendedAdditional::LatinCapitalLetterOWithCircumflexAndGrave => "latin capital letter o with circumflex and grave",
            LatinExtendedAdditional::LatinSmallLetterOWithCircumflexAndGrave => "latin small letter o with circumflex and grave",
            LatinExtendedAdditional::LatinCapitalLetterOWithCircumflexAndHookAbove => "latin capital letter o with circumflex and hook above",
            LatinExtendedAdditional::LatinSmallLetterOWithCircumflexAndHookAbove => "latin small letter o with circumflex and hook above",
            LatinExtendedAdditional::LatinCapitalLetterOWithCircumflexAndTilde => "latin capital letter o with circumflex and tilde",
            LatinExtendedAdditional::LatinSmallLetterOWithCircumflexAndTilde => "latin small letter o with circumflex and tilde",
            LatinExtendedAdditional::LatinCapitalLetterOWithCircumflexAndDotBelow => "latin capital letter o with circumflex and dot below",
            LatinExtendedAdditional::LatinSmallLetterOWithCircumflexAndDotBelow => "latin small letter o with circumflex and dot below",
            LatinExtendedAdditional::LatinCapitalLetterOWithHornAndAcute => "latin capital letter o with horn and acute",
            LatinExtendedAdditional::LatinSmallLetterOWithHornAndAcute => "latin small letter o with horn and acute",
            LatinExtendedAdditional::LatinCapitalLetterOWithHornAndGrave => "latin capital letter o with horn and grave",
            LatinExtendedAdditional::LatinSmallLetterOWithHornAndGrave => "latin small letter o with horn and grave",
            LatinExtendedAdditional::LatinCapitalLetterOWithHornAndHookAbove => "latin capital letter o with horn and hook above",
            LatinExtendedAdditional::LatinSmallLetterOWithHornAndHookAbove => "latin small letter o with horn and hook above",
            LatinExtendedAdditional::LatinCapitalLetterOWithHornAndTilde => "latin capital letter o with horn and tilde",
            LatinExtendedAdditional::LatinSmallLetterOWithHornAndTilde => "latin small letter o with horn and tilde",
            LatinExtendedAdditional::LatinCapitalLetterOWithHornAndDotBelow => "latin capital letter o with horn and dot below",
            LatinExtendedAdditional::LatinSmallLetterOWithHornAndDotBelow => "latin small letter o with horn and dot below",
            LatinExtendedAdditional::LatinCapitalLetterUWithDotBelow => "latin capital letter u with dot below",
            LatinExtendedAdditional::LatinSmallLetterUWithDotBelow => "latin small letter u with dot below",
            LatinExtendedAdditional::LatinCapitalLetterUWithHookAbove => "latin capital letter u with hook above",
            LatinExtendedAdditional::LatinSmallLetterUWithHookAbove => "latin small letter u with hook above",
            LatinExtendedAdditional::LatinCapitalLetterUWithHornAndAcute => "latin capital letter u with horn and acute",
            LatinExtendedAdditional::LatinSmallLetterUWithHornAndAcute => "latin small letter u with horn and acute",
            LatinExtendedAdditional::LatinCapitalLetterUWithHornAndGrave => "latin capital letter u with horn and grave",
            LatinExtendedAdditional::LatinSmallLetterUWithHornAndGrave => "latin small letter u with horn and grave",
            LatinExtendedAdditional::LatinCapitalLetterUWithHornAndHookAbove => "latin capital letter u with horn and hook above",
            LatinExtendedAdditional::LatinSmallLetterUWithHornAndHookAbove => "latin small letter u with horn and hook above",
            LatinExtendedAdditional::LatinCapitalLetterUWithHornAndTilde => "latin capital letter u with horn and tilde",
            LatinExtendedAdditional::LatinSmallLetterUWithHornAndTilde => "latin small letter u with horn and tilde",
            LatinExtendedAdditional::LatinCapitalLetterUWithHornAndDotBelow => "latin capital letter u with horn and dot below",
            LatinExtendedAdditional::LatinSmallLetterUWithHornAndDotBelow => "latin small letter u with horn and dot below",
            LatinExtendedAdditional::LatinCapitalLetterYWithGrave => "latin capital letter y with grave",
            LatinExtendedAdditional::LatinSmallLetterYWithGrave => "latin small letter y with grave",
            LatinExtendedAdditional::LatinCapitalLetterYWithDotBelow => "latin capital letter y with dot below",
            LatinExtendedAdditional::LatinSmallLetterYWithDotBelow => "latin small letter y with dot below",
            LatinExtendedAdditional::LatinCapitalLetterYWithHookAbove => "latin capital letter y with hook above",
            LatinExtendedAdditional::LatinSmallLetterYWithHookAbove => "latin small letter y with hook above",
            LatinExtendedAdditional::LatinCapitalLetterYWithTilde => "latin capital letter y with tilde",
            LatinExtendedAdditional::LatinSmallLetterYWithTilde => "latin small letter y with tilde",
            LatinExtendedAdditional::LatinCapitalLetterMiddleDashWelshLl => "latin capital letter middle-welsh ll",
            LatinExtendedAdditional::LatinSmallLetterMiddleDashWelshLl => "latin small letter middle-welsh ll",
            LatinExtendedAdditional::LatinCapitalLetterMiddleDashWelshV => "latin capital letter middle-welsh v",
            LatinExtendedAdditional::LatinSmallLetterMiddleDashWelshV => "latin small letter middle-welsh v",
            LatinExtendedAdditional::LatinCapitalLetterYWithLoop => "latin capital letter y with loop",
        }
    }
}
