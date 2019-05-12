/// \u{ff00} → \u{ffef}
///
/// ！ ＂ ＃ ＄ ％ ＆ ＇ （ ） ＊ ＋ ， － ． ／ ０\
/// １ ２ ３ ４ ５ ６ ７ ８ ９ ： ； ＜ ＝ ＞ ？ ＠\
/// Ａ Ｂ Ｃ Ｄ Ｅ Ｆ Ｇ Ｈ Ｉ Ｊ Ｋ Ｌ Ｍ Ｎ Ｏ Ｐ\
/// Ｑ Ｒ Ｓ Ｔ Ｕ Ｖ Ｗ Ｘ Ｙ Ｚ ［ ＼ ］ ＾ ＿ ｀\
/// ａ ｂ ｃ ｄ ｅ ｆ ｇ ｈ ｉ ｊ ｋ ｌ ｍ ｎ ｏ ｐ\
/// ｑ ｒ ｓ ｔ ｕ ｖ ｗ ｘ ｙ ｚ ｛ ｜ ｝ ～ ｟ ｠\
/// ｡ ｢ ｣ ､ ･ ｦ ｧ ｨ ｩ ｪ ｫ ｬ ｭ ｮ ｯ ｰ\
/// ｱ ｲ ｳ ｴ ｵ ｶ ｷ ｸ ｹ ｺ ｻ ｼ ｽ ｾ ｿ ﾀ\
/// ﾁ ﾂ ﾃ ﾄ ﾅ ﾆ ﾇ ﾈ ﾉ ﾊ ﾋ ﾌ ﾍ ﾎ ﾏ ﾐ\
/// ﾑ ﾒ ﾓ ﾔ ﾕ ﾖ ﾗ ﾘ ﾙ ﾚ ﾛ ﾜ ﾝ ﾞ ﾟ ﾠ\
/// ﾡ ﾢ ﾣ ﾤ ﾥ ﾦ ﾧ ﾨ ﾩ ﾪ ﾫ ﾬ ﾭ ﾮ ﾯ ﾰ\
/// ﾱ ﾲ ﾳ ﾴ ﾵ ﾶ ﾷ ﾸ ﾹ ﾺ ﾻ ﾼ ﾽ ﾾ ￂ ￃ\
/// ￄ ￅ ￆ ￇ ￊ ￋ ￌ ￍ ￎ ￏ ￒ ￓ ￔ ￕ ￖ ￗ\
/// ￚ ￛ ￜ ￠ ￡ ￢ ￣ ￤ ￥ ￦ ￨ ￩ ￪ ￫ ￬ ￭\
/// ￮\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{ff01}: '！'
    pub const FULLWIDTH_EXCLAMATION_MARK: char = '！';
    /// \u{ff02}: '＂'
    pub const FULLWIDTH_QUOTATION_MARK: char = '＂';
    /// \u{ff03}: '＃'
    pub const FULLWIDTH_NUMBER_SIGN: char = '＃';
    /// \u{ff04}: '＄'
    pub const FULLWIDTH_DOLLAR_SIGN: char = '＄';
    /// \u{ff05}: '％'
    pub const FULLWIDTH_PERCENT_SIGN: char = '％';
    /// \u{ff06}: '＆'
    pub const FULLWIDTH_AMPERSAND: char = '＆';
    /// \u{ff07}: '＇'
    pub const FULLWIDTH_APOSTROPHE: char = '＇';
    /// \u{ff08}: '（'
    pub const FULLWIDTH_LEFT_PARENTHESIS: char = '（';
    /// \u{ff09}: '）'
    pub const FULLWIDTH_RIGHT_PARENTHESIS: char = '）';
    /// \u{ff0a}: '＊'
    pub const FULLWIDTH_ASTERISK: char = '＊';
    /// \u{ff0b}: '＋'
    pub const FULLWIDTH_PLUS_SIGN: char = '＋';
    /// \u{ff0c}: '，'
    pub const FULLWIDTH_COMMA: char = '，';
    /// \u{ff0d}: '－'
    pub const FULLWIDTH_HYPHEN_DASH_MINUS: char = '－';
    /// \u{ff0e}: '．'
    pub const FULLWIDTH_FULL_STOP: char = '．';
    /// \u{ff0f}: '／'
    pub const FULLWIDTH_SOLIDUS: char = '／';
    /// \u{ff10}: '０'
    pub const FULLWIDTH_DIGIT_ZERO: char = '０';
    /// \u{ff11}: '１'
    pub const FULLWIDTH_DIGIT_ONE: char = '１';
    /// \u{ff12}: '２'
    pub const FULLWIDTH_DIGIT_TWO: char = '２';
    /// \u{ff13}: '３'
    pub const FULLWIDTH_DIGIT_THREE: char = '３';
    /// \u{ff14}: '４'
    pub const FULLWIDTH_DIGIT_FOUR: char = '４';
    /// \u{ff15}: '５'
    pub const FULLWIDTH_DIGIT_FIVE: char = '５';
    /// \u{ff16}: '６'
    pub const FULLWIDTH_DIGIT_SIX: char = '６';
    /// \u{ff17}: '７'
    pub const FULLWIDTH_DIGIT_SEVEN: char = '７';
    /// \u{ff18}: '８'
    pub const FULLWIDTH_DIGIT_EIGHT: char = '８';
    /// \u{ff19}: '９'
    pub const FULLWIDTH_DIGIT_NINE: char = '９';
    /// \u{ff1a}: '：'
    pub const FULLWIDTH_COLON: char = '：';
    /// \u{ff1b}: '；'
    pub const FULLWIDTH_SEMICOLON: char = '；';
    /// \u{ff1c}: '＜'
    pub const FULLWIDTH_LESS_DASH_THAN_SIGN: char = '＜';
    /// \u{ff1d}: '＝'
    pub const FULLWIDTH_EQUALS_SIGN: char = '＝';
    /// \u{ff1e}: '＞'
    pub const FULLWIDTH_GREATER_DASH_THAN_SIGN: char = '＞';
    /// \u{ff1f}: '？'
    pub const FULLWIDTH_QUESTION_MARK: char = '？';
    /// \u{ff20}: '＠'
    pub const FULLWIDTH_COMMERCIAL_AT: char = '＠';
    /// \u{ff21}: 'Ａ'
    pub const FULLWIDTH_LATIN_CAPITAL_LETTER_A: char = 'Ａ';
    /// \u{ff22}: 'Ｂ'
    pub const FULLWIDTH_LATIN_CAPITAL_LETTER_B: char = 'Ｂ';
    /// \u{ff23}: 'Ｃ'
    pub const FULLWIDTH_LATIN_CAPITAL_LETTER_C: char = 'Ｃ';
    /// \u{ff24}: 'Ｄ'
    pub const FULLWIDTH_LATIN_CAPITAL_LETTER_D: char = 'Ｄ';
    /// \u{ff25}: 'Ｅ'
    pub const FULLWIDTH_LATIN_CAPITAL_LETTER_E: char = 'Ｅ';
    /// \u{ff26}: 'Ｆ'
    pub const FULLWIDTH_LATIN_CAPITAL_LETTER_F: char = 'Ｆ';
    /// \u{ff27}: 'Ｇ'
    pub const FULLWIDTH_LATIN_CAPITAL_LETTER_G: char = 'Ｇ';
    /// \u{ff28}: 'Ｈ'
    pub const FULLWIDTH_LATIN_CAPITAL_LETTER_H: char = 'Ｈ';
    /// \u{ff29}: 'Ｉ'
    pub const FULLWIDTH_LATIN_CAPITAL_LETTER_I: char = 'Ｉ';
    /// \u{ff2a}: 'Ｊ'
    pub const FULLWIDTH_LATIN_CAPITAL_LETTER_J: char = 'Ｊ';
    /// \u{ff2b}: 'Ｋ'
    pub const FULLWIDTH_LATIN_CAPITAL_LETTER_K: char = 'Ｋ';
    /// \u{ff2c}: 'Ｌ'
    pub const FULLWIDTH_LATIN_CAPITAL_LETTER_L: char = 'Ｌ';
    /// \u{ff2d}: 'Ｍ'
    pub const FULLWIDTH_LATIN_CAPITAL_LETTER_M: char = 'Ｍ';
    /// \u{ff2e}: 'Ｎ'
    pub const FULLWIDTH_LATIN_CAPITAL_LETTER_N: char = 'Ｎ';
    /// \u{ff2f}: 'Ｏ'
    pub const FULLWIDTH_LATIN_CAPITAL_LETTER_O: char = 'Ｏ';
    /// \u{ff30}: 'Ｐ'
    pub const FULLWIDTH_LATIN_CAPITAL_LETTER_P: char = 'Ｐ';
    /// \u{ff31}: 'Ｑ'
    pub const FULLWIDTH_LATIN_CAPITAL_LETTER_Q: char = 'Ｑ';
    /// \u{ff32}: 'Ｒ'
    pub const FULLWIDTH_LATIN_CAPITAL_LETTER_R: char = 'Ｒ';
    /// \u{ff33}: 'Ｓ'
    pub const FULLWIDTH_LATIN_CAPITAL_LETTER_S: char = 'Ｓ';
    /// \u{ff34}: 'Ｔ'
    pub const FULLWIDTH_LATIN_CAPITAL_LETTER_T: char = 'Ｔ';
    /// \u{ff35}: 'Ｕ'
    pub const FULLWIDTH_LATIN_CAPITAL_LETTER_U: char = 'Ｕ';
    /// \u{ff36}: 'Ｖ'
    pub const FULLWIDTH_LATIN_CAPITAL_LETTER_V: char = 'Ｖ';
    /// \u{ff37}: 'Ｗ'
    pub const FULLWIDTH_LATIN_CAPITAL_LETTER_W: char = 'Ｗ';
    /// \u{ff38}: 'Ｘ'
    pub const FULLWIDTH_LATIN_CAPITAL_LETTER_X: char = 'Ｘ';
    /// \u{ff39}: 'Ｙ'
    pub const FULLWIDTH_LATIN_CAPITAL_LETTER_Y: char = 'Ｙ';
    /// \u{ff3a}: 'Ｚ'
    pub const FULLWIDTH_LATIN_CAPITAL_LETTER_Z: char = 'Ｚ';
    /// \u{ff3b}: '［'
    pub const FULLWIDTH_LEFT_SQUARE_BRACKET: char = '［';
    /// \u{ff3c}: '＼'
    pub const FULLWIDTH_REVERSE_SOLIDUS: char = '＼';
    /// \u{ff3d}: '］'
    pub const FULLWIDTH_RIGHT_SQUARE_BRACKET: char = '］';
    /// \u{ff3e}: '＾'
    pub const FULLWIDTH_CIRCUMFLEX_ACCENT: char = '＾';
    /// \u{ff3f}: '＿'
    pub const FULLWIDTH_LOW_LINE: char = '＿';
    /// \u{ff40}: '｀'
    pub const FULLWIDTH_GRAVE_ACCENT: char = '｀';
    /// \u{ff41}: 'ａ'
    pub const FULLWIDTH_LATIN_SMALL_LETTER_A: char = 'ａ';
    /// \u{ff42}: 'ｂ'
    pub const FULLWIDTH_LATIN_SMALL_LETTER_B: char = 'ｂ';
    /// \u{ff43}: 'ｃ'
    pub const FULLWIDTH_LATIN_SMALL_LETTER_C: char = 'ｃ';
    /// \u{ff44}: 'ｄ'
    pub const FULLWIDTH_LATIN_SMALL_LETTER_D: char = 'ｄ';
    /// \u{ff45}: 'ｅ'
    pub const FULLWIDTH_LATIN_SMALL_LETTER_E: char = 'ｅ';
    /// \u{ff46}: 'ｆ'
    pub const FULLWIDTH_LATIN_SMALL_LETTER_F: char = 'ｆ';
    /// \u{ff47}: 'ｇ'
    pub const FULLWIDTH_LATIN_SMALL_LETTER_G: char = 'ｇ';
    /// \u{ff48}: 'ｈ'
    pub const FULLWIDTH_LATIN_SMALL_LETTER_H: char = 'ｈ';
    /// \u{ff49}: 'ｉ'
    pub const FULLWIDTH_LATIN_SMALL_LETTER_I: char = 'ｉ';
    /// \u{ff4a}: 'ｊ'
    pub const FULLWIDTH_LATIN_SMALL_LETTER_J: char = 'ｊ';
    /// \u{ff4b}: 'ｋ'
    pub const FULLWIDTH_LATIN_SMALL_LETTER_K: char = 'ｋ';
    /// \u{ff4c}: 'ｌ'
    pub const FULLWIDTH_LATIN_SMALL_LETTER_L: char = 'ｌ';
    /// \u{ff4d}: 'ｍ'
    pub const FULLWIDTH_LATIN_SMALL_LETTER_M: char = 'ｍ';
    /// \u{ff4e}: 'ｎ'
    pub const FULLWIDTH_LATIN_SMALL_LETTER_N: char = 'ｎ';
    /// \u{ff4f}: 'ｏ'
    pub const FULLWIDTH_LATIN_SMALL_LETTER_O: char = 'ｏ';
    /// \u{ff50}: 'ｐ'
    pub const FULLWIDTH_LATIN_SMALL_LETTER_P: char = 'ｐ';
    /// \u{ff51}: 'ｑ'
    pub const FULLWIDTH_LATIN_SMALL_LETTER_Q: char = 'ｑ';
    /// \u{ff52}: 'ｒ'
    pub const FULLWIDTH_LATIN_SMALL_LETTER_R: char = 'ｒ';
    /// \u{ff53}: 'ｓ'
    pub const FULLWIDTH_LATIN_SMALL_LETTER_S: char = 'ｓ';
    /// \u{ff54}: 'ｔ'
    pub const FULLWIDTH_LATIN_SMALL_LETTER_T: char = 'ｔ';
    /// \u{ff55}: 'ｕ'
    pub const FULLWIDTH_LATIN_SMALL_LETTER_U: char = 'ｕ';
    /// \u{ff56}: 'ｖ'
    pub const FULLWIDTH_LATIN_SMALL_LETTER_V: char = 'ｖ';
    /// \u{ff57}: 'ｗ'
    pub const FULLWIDTH_LATIN_SMALL_LETTER_W: char = 'ｗ';
    /// \u{ff58}: 'ｘ'
    pub const FULLWIDTH_LATIN_SMALL_LETTER_X: char = 'ｘ';
    /// \u{ff59}: 'ｙ'
    pub const FULLWIDTH_LATIN_SMALL_LETTER_Y: char = 'ｙ';
    /// \u{ff5a}: 'ｚ'
    pub const FULLWIDTH_LATIN_SMALL_LETTER_Z: char = 'ｚ';
    /// \u{ff5b}: '｛'
    pub const FULLWIDTH_LEFT_CURLY_BRACKET: char = '｛';
    /// \u{ff5c}: '｜'
    pub const FULLWIDTH_VERTICAL_LINE: char = '｜';
    /// \u{ff5d}: '｝'
    pub const FULLWIDTH_RIGHT_CURLY_BRACKET: char = '｝';
    /// \u{ff5e}: '～'
    pub const FULLWIDTH_TILDE: char = '～';
    /// \u{ff5f}: '｟'
    pub const FULLWIDTH_LEFT_WHITE_PARENTHESIS: char = '｟';
    /// \u{ff60}: '｠'
    pub const FULLWIDTH_RIGHT_WHITE_PARENTHESIS: char = '｠';
    /// \u{ff61}: '｡'
    pub const HALFWIDTH_IDEOGRAPHIC_FULL_STOP: char = '｡';
    /// \u{ff62}: '｢'
    pub const HALFWIDTH_LEFT_CORNER_BRACKET: char = '｢';
    /// \u{ff63}: '｣'
    pub const HALFWIDTH_RIGHT_CORNER_BRACKET: char = '｣';
    /// \u{ff64}: '､'
    pub const HALFWIDTH_IDEOGRAPHIC_COMMA: char = '､';
    /// \u{ff65}: '･'
    pub const HALFWIDTH_KATAKANA_MIDDLE_DOT: char = '･';
    /// \u{ff66}: 'ｦ'
    pub const HALFWIDTH_KATAKANA_LETTER_WO: char = 'ｦ';
    /// \u{ff67}: 'ｧ'
    pub const HALFWIDTH_KATAKANA_LETTER_SMALL_A: char = 'ｧ';
    /// \u{ff68}: 'ｨ'
    pub const HALFWIDTH_KATAKANA_LETTER_SMALL_I: char = 'ｨ';
    /// \u{ff69}: 'ｩ'
    pub const HALFWIDTH_KATAKANA_LETTER_SMALL_U: char = 'ｩ';
    /// \u{ff6a}: 'ｪ'
    pub const HALFWIDTH_KATAKANA_LETTER_SMALL_E: char = 'ｪ';
    /// \u{ff6b}: 'ｫ'
    pub const HALFWIDTH_KATAKANA_LETTER_SMALL_O: char = 'ｫ';
    /// \u{ff6c}: 'ｬ'
    pub const HALFWIDTH_KATAKANA_LETTER_SMALL_YA: char = 'ｬ';
    /// \u{ff6d}: 'ｭ'
    pub const HALFWIDTH_KATAKANA_LETTER_SMALL_YU: char = 'ｭ';
    /// \u{ff6e}: 'ｮ'
    pub const HALFWIDTH_KATAKANA_LETTER_SMALL_YO: char = 'ｮ';
    /// \u{ff6f}: 'ｯ'
    pub const HALFWIDTH_KATAKANA_LETTER_SMALL_TU: char = 'ｯ';
    /// \u{ff70}: 'ｰ'
    pub const HALFWIDTH_KATAKANA_DASH_HIRAGANA_PROLONGED_SOUND_MARK: char = 'ｰ';
    /// \u{ff71}: 'ｱ'
    pub const HALFWIDTH_KATAKANA_LETTER_A: char = 'ｱ';
    /// \u{ff72}: 'ｲ'
    pub const HALFWIDTH_KATAKANA_LETTER_I: char = 'ｲ';
    /// \u{ff73}: 'ｳ'
    pub const HALFWIDTH_KATAKANA_LETTER_U: char = 'ｳ';
    /// \u{ff74}: 'ｴ'
    pub const HALFWIDTH_KATAKANA_LETTER_E: char = 'ｴ';
    /// \u{ff75}: 'ｵ'
    pub const HALFWIDTH_KATAKANA_LETTER_O: char = 'ｵ';
    /// \u{ff76}: 'ｶ'
    pub const HALFWIDTH_KATAKANA_LETTER_KA: char = 'ｶ';
    /// \u{ff77}: 'ｷ'
    pub const HALFWIDTH_KATAKANA_LETTER_KI: char = 'ｷ';
    /// \u{ff78}: 'ｸ'
    pub const HALFWIDTH_KATAKANA_LETTER_KU: char = 'ｸ';
    /// \u{ff79}: 'ｹ'
    pub const HALFWIDTH_KATAKANA_LETTER_KE: char = 'ｹ';
    /// \u{ff7a}: 'ｺ'
    pub const HALFWIDTH_KATAKANA_LETTER_KO: char = 'ｺ';
    /// \u{ff7b}: 'ｻ'
    pub const HALFWIDTH_KATAKANA_LETTER_SA: char = 'ｻ';
    /// \u{ff7c}: 'ｼ'
    pub const HALFWIDTH_KATAKANA_LETTER_SI: char = 'ｼ';
    /// \u{ff7d}: 'ｽ'
    pub const HALFWIDTH_KATAKANA_LETTER_SU: char = 'ｽ';
    /// \u{ff7e}: 'ｾ'
    pub const HALFWIDTH_KATAKANA_LETTER_SE: char = 'ｾ';
    /// \u{ff7f}: 'ｿ'
    pub const HALFWIDTH_KATAKANA_LETTER_SO: char = 'ｿ';
    /// \u{ff80}: 'ﾀ'
    pub const HALFWIDTH_KATAKANA_LETTER_TA: char = 'ﾀ';
    /// \u{ff81}: 'ﾁ'
    pub const HALFWIDTH_KATAKANA_LETTER_TI: char = 'ﾁ';
    /// \u{ff82}: 'ﾂ'
    pub const HALFWIDTH_KATAKANA_LETTER_TU: char = 'ﾂ';
    /// \u{ff83}: 'ﾃ'
    pub const HALFWIDTH_KATAKANA_LETTER_TE: char = 'ﾃ';
    /// \u{ff84}: 'ﾄ'
    pub const HALFWIDTH_KATAKANA_LETTER_TO: char = 'ﾄ';
    /// \u{ff85}: 'ﾅ'
    pub const HALFWIDTH_KATAKANA_LETTER_NA: char = 'ﾅ';
    /// \u{ff86}: 'ﾆ'
    pub const HALFWIDTH_KATAKANA_LETTER_NI: char = 'ﾆ';
    /// \u{ff87}: 'ﾇ'
    pub const HALFWIDTH_KATAKANA_LETTER_NU: char = 'ﾇ';
    /// \u{ff88}: 'ﾈ'
    pub const HALFWIDTH_KATAKANA_LETTER_NE: char = 'ﾈ';
    /// \u{ff89}: 'ﾉ'
    pub const HALFWIDTH_KATAKANA_LETTER_NO: char = 'ﾉ';
    /// \u{ff8a}: 'ﾊ'
    pub const HALFWIDTH_KATAKANA_LETTER_HA: char = 'ﾊ';
    /// \u{ff8b}: 'ﾋ'
    pub const HALFWIDTH_KATAKANA_LETTER_HI: char = 'ﾋ';
    /// \u{ff8c}: 'ﾌ'
    pub const HALFWIDTH_KATAKANA_LETTER_HU: char = 'ﾌ';
    /// \u{ff8d}: 'ﾍ'
    pub const HALFWIDTH_KATAKANA_LETTER_HE: char = 'ﾍ';
    /// \u{ff8e}: 'ﾎ'
    pub const HALFWIDTH_KATAKANA_LETTER_HO: char = 'ﾎ';
    /// \u{ff8f}: 'ﾏ'
    pub const HALFWIDTH_KATAKANA_LETTER_MA: char = 'ﾏ';
    /// \u{ff90}: 'ﾐ'
    pub const HALFWIDTH_KATAKANA_LETTER_MI: char = 'ﾐ';
    /// \u{ff91}: 'ﾑ'
    pub const HALFWIDTH_KATAKANA_LETTER_MU: char = 'ﾑ';
    /// \u{ff92}: 'ﾒ'
    pub const HALFWIDTH_KATAKANA_LETTER_ME: char = 'ﾒ';
    /// \u{ff93}: 'ﾓ'
    pub const HALFWIDTH_KATAKANA_LETTER_MO: char = 'ﾓ';
    /// \u{ff94}: 'ﾔ'
    pub const HALFWIDTH_KATAKANA_LETTER_YA: char = 'ﾔ';
    /// \u{ff95}: 'ﾕ'
    pub const HALFWIDTH_KATAKANA_LETTER_YU: char = 'ﾕ';
    /// \u{ff96}: 'ﾖ'
    pub const HALFWIDTH_KATAKANA_LETTER_YO: char = 'ﾖ';
    /// \u{ff97}: 'ﾗ'
    pub const HALFWIDTH_KATAKANA_LETTER_RA: char = 'ﾗ';
    /// \u{ff98}: 'ﾘ'
    pub const HALFWIDTH_KATAKANA_LETTER_RI: char = 'ﾘ';
    /// \u{ff99}: 'ﾙ'
    pub const HALFWIDTH_KATAKANA_LETTER_RU: char = 'ﾙ';
    /// \u{ff9a}: 'ﾚ'
    pub const HALFWIDTH_KATAKANA_LETTER_RE: char = 'ﾚ';
    /// \u{ff9b}: 'ﾛ'
    pub const HALFWIDTH_KATAKANA_LETTER_RO: char = 'ﾛ';
    /// \u{ff9c}: 'ﾜ'
    pub const HALFWIDTH_KATAKANA_LETTER_WA: char = 'ﾜ';
    /// \u{ff9d}: 'ﾝ'
    pub const HALFWIDTH_KATAKANA_LETTER_N: char = 'ﾝ';
    /// \u{ff9e}: 'ﾞ'
    pub const HALFWIDTH_KATAKANA_VOICED_SOUND_MARK: char = 'ﾞ';
    /// \u{ff9f}: 'ﾟ'
    pub const HALFWIDTH_KATAKANA_SEMI_DASH_VOICED_SOUND_MARK: char = 'ﾟ';
    /// \u{ffa0}: 'ﾠ'
    pub const HALFWIDTH_HANGUL_FILLER: char = 'ﾠ';
    /// \u{ffa1}: 'ﾡ'
    pub const HALFWIDTH_HANGUL_LETTER_KIYEOK: char = 'ﾡ';
    /// \u{ffa2}: 'ﾢ'
    pub const HALFWIDTH_HANGUL_LETTER_SSANGKIYEOK: char = 'ﾢ';
    /// \u{ffa3}: 'ﾣ'
    pub const HALFWIDTH_HANGUL_LETTER_KIYEOK_DASH_SIOS: char = 'ﾣ';
    /// \u{ffa4}: 'ﾤ'
    pub const HALFWIDTH_HANGUL_LETTER_NIEUN: char = 'ﾤ';
    /// \u{ffa5}: 'ﾥ'
    pub const HALFWIDTH_HANGUL_LETTER_NIEUN_DASH_CIEUC: char = 'ﾥ';
    /// \u{ffa6}: 'ﾦ'
    pub const HALFWIDTH_HANGUL_LETTER_NIEUN_DASH_HIEUH: char = 'ﾦ';
    /// \u{ffa7}: 'ﾧ'
    pub const HALFWIDTH_HANGUL_LETTER_TIKEUT: char = 'ﾧ';
    /// \u{ffa8}: 'ﾨ'
    pub const HALFWIDTH_HANGUL_LETTER_SSANGTIKEUT: char = 'ﾨ';
    /// \u{ffa9}: 'ﾩ'
    pub const HALFWIDTH_HANGUL_LETTER_RIEUL: char = 'ﾩ';
    /// \u{ffaa}: 'ﾪ'
    pub const HALFWIDTH_HANGUL_LETTER_RIEUL_DASH_KIYEOK: char = 'ﾪ';
    /// \u{ffab}: 'ﾫ'
    pub const HALFWIDTH_HANGUL_LETTER_RIEUL_DASH_MIEUM: char = 'ﾫ';
    /// \u{ffac}: 'ﾬ'
    pub const HALFWIDTH_HANGUL_LETTER_RIEUL_DASH_PIEUP: char = 'ﾬ';
    /// \u{ffad}: 'ﾭ'
    pub const HALFWIDTH_HANGUL_LETTER_RIEUL_DASH_SIOS: char = 'ﾭ';
    /// \u{ffae}: 'ﾮ'
    pub const HALFWIDTH_HANGUL_LETTER_RIEUL_DASH_THIEUTH: char = 'ﾮ';
    /// \u{ffaf}: 'ﾯ'
    pub const HALFWIDTH_HANGUL_LETTER_RIEUL_DASH_PHIEUPH: char = 'ﾯ';
    /// \u{ffb0}: 'ﾰ'
    pub const HALFWIDTH_HANGUL_LETTER_RIEUL_DASH_HIEUH: char = 'ﾰ';
    /// \u{ffb1}: 'ﾱ'
    pub const HALFWIDTH_HANGUL_LETTER_MIEUM: char = 'ﾱ';
    /// \u{ffb2}: 'ﾲ'
    pub const HALFWIDTH_HANGUL_LETTER_PIEUP: char = 'ﾲ';
    /// \u{ffb3}: 'ﾳ'
    pub const HALFWIDTH_HANGUL_LETTER_SSANGPIEUP: char = 'ﾳ';
    /// \u{ffb4}: 'ﾴ'
    pub const HALFWIDTH_HANGUL_LETTER_PIEUP_DASH_SIOS: char = 'ﾴ';
    /// \u{ffb5}: 'ﾵ'
    pub const HALFWIDTH_HANGUL_LETTER_SIOS: char = 'ﾵ';
    /// \u{ffb6}: 'ﾶ'
    pub const HALFWIDTH_HANGUL_LETTER_SSANGSIOS: char = 'ﾶ';
    /// \u{ffb7}: 'ﾷ'
    pub const HALFWIDTH_HANGUL_LETTER_IEUNG: char = 'ﾷ';
    /// \u{ffb8}: 'ﾸ'
    pub const HALFWIDTH_HANGUL_LETTER_CIEUC: char = 'ﾸ';
    /// \u{ffb9}: 'ﾹ'
    pub const HALFWIDTH_HANGUL_LETTER_SSANGCIEUC: char = 'ﾹ';
    /// \u{ffba}: 'ﾺ'
    pub const HALFWIDTH_HANGUL_LETTER_CHIEUCH: char = 'ﾺ';
    /// \u{ffbb}: 'ﾻ'
    pub const HALFWIDTH_HANGUL_LETTER_KHIEUKH: char = 'ﾻ';
    /// \u{ffbc}: 'ﾼ'
    pub const HALFWIDTH_HANGUL_LETTER_THIEUTH: char = 'ﾼ';
    /// \u{ffbd}: 'ﾽ'
    pub const HALFWIDTH_HANGUL_LETTER_PHIEUPH: char = 'ﾽ';
    /// \u{ffbe}: 'ﾾ'
    pub const HALFWIDTH_HANGUL_LETTER_HIEUH: char = 'ﾾ';
    /// \u{ffc2}: 'ￂ'
    pub const HALFWIDTH_HANGUL_LETTER_A: char = 'ￂ';
    /// \u{ffc3}: 'ￃ'
    pub const HALFWIDTH_HANGUL_LETTER_AE: char = 'ￃ';
    /// \u{ffc4}: 'ￄ'
    pub const HALFWIDTH_HANGUL_LETTER_YA: char = 'ￄ';
    /// \u{ffc5}: 'ￅ'
    pub const HALFWIDTH_HANGUL_LETTER_YAE: char = 'ￅ';
    /// \u{ffc6}: 'ￆ'
    pub const HALFWIDTH_HANGUL_LETTER_EO: char = 'ￆ';
    /// \u{ffc7}: 'ￇ'
    pub const HALFWIDTH_HANGUL_LETTER_E: char = 'ￇ';
    /// \u{ffca}: 'ￊ'
    pub const HALFWIDTH_HANGUL_LETTER_YEO: char = 'ￊ';
    /// \u{ffcb}: 'ￋ'
    pub const HALFWIDTH_HANGUL_LETTER_YE: char = 'ￋ';
    /// \u{ffcc}: 'ￌ'
    pub const HALFWIDTH_HANGUL_LETTER_O: char = 'ￌ';
    /// \u{ffcd}: 'ￍ'
    pub const HALFWIDTH_HANGUL_LETTER_WA: char = 'ￍ';
    /// \u{ffce}: 'ￎ'
    pub const HALFWIDTH_HANGUL_LETTER_WAE: char = 'ￎ';
    /// \u{ffcf}: 'ￏ'
    pub const HALFWIDTH_HANGUL_LETTER_OE: char = 'ￏ';
    /// \u{ffd2}: 'ￒ'
    pub const HALFWIDTH_HANGUL_LETTER_YO: char = 'ￒ';
    /// \u{ffd3}: 'ￓ'
    pub const HALFWIDTH_HANGUL_LETTER_U: char = 'ￓ';
    /// \u{ffd4}: 'ￔ'
    pub const HALFWIDTH_HANGUL_LETTER_WEO: char = 'ￔ';
    /// \u{ffd5}: 'ￕ'
    pub const HALFWIDTH_HANGUL_LETTER_WE: char = 'ￕ';
    /// \u{ffd6}: 'ￖ'
    pub const HALFWIDTH_HANGUL_LETTER_WI: char = 'ￖ';
    /// \u{ffd7}: 'ￗ'
    pub const HALFWIDTH_HANGUL_LETTER_YU: char = 'ￗ';
    /// \u{ffda}: 'ￚ'
    pub const HALFWIDTH_HANGUL_LETTER_EU: char = 'ￚ';
    /// \u{ffdb}: 'ￛ'
    pub const HALFWIDTH_HANGUL_LETTER_YI: char = 'ￛ';
    /// \u{ffdc}: 'ￜ'
    pub const HALFWIDTH_HANGUL_LETTER_I: char = 'ￜ';
    /// \u{ffe0}: '￠'
    pub const FULLWIDTH_CENT_SIGN: char = '￠';
    /// \u{ffe1}: '￡'
    pub const FULLWIDTH_POUND_SIGN: char = '￡';
    /// \u{ffe2}: '￢'
    pub const FULLWIDTH_NOT_SIGN: char = '￢';
    /// \u{ffe3}: '￣'
    pub const FULLWIDTH_MACRON: char = '￣';
    /// \u{ffe4}: '￤'
    pub const FULLWIDTH_BROKEN_BAR: char = '￤';
    /// \u{ffe5}: '￥'
    pub const FULLWIDTH_YEN_SIGN: char = '￥';
    /// \u{ffe6}: '￦'
    pub const FULLWIDTH_WON_SIGN: char = '￦';
    /// \u{ffe8}: '￨'
    pub const HALFWIDTH_FORMS_LIGHT_VERTICAL: char = '￨';
    /// \u{ffe9}: '￩'
    pub const HALFWIDTH_LEFTWARDS_ARROW: char = '￩';
    /// \u{ffea}: '￪'
    pub const HALFWIDTH_UPWARDS_ARROW: char = '￪';
    /// \u{ffeb}: '￫'
    pub const HALFWIDTH_RIGHTWARDS_ARROW: char = '￫';
    /// \u{ffec}: '￬'
    pub const HALFWIDTH_DOWNWARDS_ARROW: char = '￬';
    /// \u{ffed}: '￭'
    pub const HALFWIDTH_BLACK_SQUARE: char = '￭';
    /// \u{ffee}: '￮'
    pub const HALFWIDTH_WHITE_CIRCLE: char = '￮';
}

/// An enum to represent all characters in the HalfwidthandFullwidthForms block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum HalfwidthandFullwidthForms {
    /// \u{ff01}: '！'
    FullwidthExclamationMark,
    /// \u{ff02}: '＂'
    FullwidthQuotationMark,
    /// \u{ff03}: '＃'
    FullwidthNumberSign,
    /// \u{ff04}: '＄'
    FullwidthDollarSign,
    /// \u{ff05}: '％'
    FullwidthPercentSign,
    /// \u{ff06}: '＆'
    FullwidthAmpersand,
    /// \u{ff07}: '＇'
    FullwidthApostrophe,
    /// \u{ff08}: '（'
    FullwidthLeftParenthesis,
    /// \u{ff09}: '）'
    FullwidthRightParenthesis,
    /// \u{ff0a}: '＊'
    FullwidthAsterisk,
    /// \u{ff0b}: '＋'
    FullwidthPlusSign,
    /// \u{ff0c}: '，'
    FullwidthComma,
    /// \u{ff0d}: '－'
    FullwidthHyphenDashMinus,
    /// \u{ff0e}: '．'
    FullwidthFullStop,
    /// \u{ff0f}: '／'
    FullwidthSolidus,
    /// \u{ff10}: '０'
    FullwidthDigitZero,
    /// \u{ff11}: '１'
    FullwidthDigitOne,
    /// \u{ff12}: '２'
    FullwidthDigitTwo,
    /// \u{ff13}: '３'
    FullwidthDigitThree,
    /// \u{ff14}: '４'
    FullwidthDigitFour,
    /// \u{ff15}: '５'
    FullwidthDigitFive,
    /// \u{ff16}: '６'
    FullwidthDigitSix,
    /// \u{ff17}: '７'
    FullwidthDigitSeven,
    /// \u{ff18}: '８'
    FullwidthDigitEight,
    /// \u{ff19}: '９'
    FullwidthDigitNine,
    /// \u{ff1a}: '：'
    FullwidthColon,
    /// \u{ff1b}: '；'
    FullwidthSemicolon,
    /// \u{ff1c}: '＜'
    FullwidthLessDashThanSign,
    /// \u{ff1d}: '＝'
    FullwidthEqualsSign,
    /// \u{ff1e}: '＞'
    FullwidthGreaterDashThanSign,
    /// \u{ff1f}: '？'
    FullwidthQuestionMark,
    /// \u{ff20}: '＠'
    FullwidthCommercialAt,
    /// \u{ff21}: 'Ａ'
    FullwidthLatinCapitalLetterA,
    /// \u{ff22}: 'Ｂ'
    FullwidthLatinCapitalLetterB,
    /// \u{ff23}: 'Ｃ'
    FullwidthLatinCapitalLetterC,
    /// \u{ff24}: 'Ｄ'
    FullwidthLatinCapitalLetterD,
    /// \u{ff25}: 'Ｅ'
    FullwidthLatinCapitalLetterE,
    /// \u{ff26}: 'Ｆ'
    FullwidthLatinCapitalLetterF,
    /// \u{ff27}: 'Ｇ'
    FullwidthLatinCapitalLetterG,
    /// \u{ff28}: 'Ｈ'
    FullwidthLatinCapitalLetterH,
    /// \u{ff29}: 'Ｉ'
    FullwidthLatinCapitalLetterI,
    /// \u{ff2a}: 'Ｊ'
    FullwidthLatinCapitalLetterJ,
    /// \u{ff2b}: 'Ｋ'
    FullwidthLatinCapitalLetterK,
    /// \u{ff2c}: 'Ｌ'
    FullwidthLatinCapitalLetterL,
    /// \u{ff2d}: 'Ｍ'
    FullwidthLatinCapitalLetterM,
    /// \u{ff2e}: 'Ｎ'
    FullwidthLatinCapitalLetterN,
    /// \u{ff2f}: 'Ｏ'
    FullwidthLatinCapitalLetterO,
    /// \u{ff30}: 'Ｐ'
    FullwidthLatinCapitalLetterP,
    /// \u{ff31}: 'Ｑ'
    FullwidthLatinCapitalLetterQ,
    /// \u{ff32}: 'Ｒ'
    FullwidthLatinCapitalLetterR,
    /// \u{ff33}: 'Ｓ'
    FullwidthLatinCapitalLetterS,
    /// \u{ff34}: 'Ｔ'
    FullwidthLatinCapitalLetterT,
    /// \u{ff35}: 'Ｕ'
    FullwidthLatinCapitalLetterU,
    /// \u{ff36}: 'Ｖ'
    FullwidthLatinCapitalLetterV,
    /// \u{ff37}: 'Ｗ'
    FullwidthLatinCapitalLetterW,
    /// \u{ff38}: 'Ｘ'
    FullwidthLatinCapitalLetterX,
    /// \u{ff39}: 'Ｙ'
    FullwidthLatinCapitalLetterY,
    /// \u{ff3a}: 'Ｚ'
    FullwidthLatinCapitalLetterZ,
    /// \u{ff3b}: '［'
    FullwidthLeftSquareBracket,
    /// \u{ff3c}: '＼'
    FullwidthReverseSolidus,
    /// \u{ff3d}: '］'
    FullwidthRightSquareBracket,
    /// \u{ff3e}: '＾'
    FullwidthCircumflexAccent,
    /// \u{ff3f}: '＿'
    FullwidthLowLine,
    /// \u{ff40}: '｀'
    FullwidthGraveAccent,
    /// \u{ff41}: 'ａ'
    FullwidthLatinSmallLetterA,
    /// \u{ff42}: 'ｂ'
    FullwidthLatinSmallLetterB,
    /// \u{ff43}: 'ｃ'
    FullwidthLatinSmallLetterC,
    /// \u{ff44}: 'ｄ'
    FullwidthLatinSmallLetterD,
    /// \u{ff45}: 'ｅ'
    FullwidthLatinSmallLetterE,
    /// \u{ff46}: 'ｆ'
    FullwidthLatinSmallLetterF,
    /// \u{ff47}: 'ｇ'
    FullwidthLatinSmallLetterG,
    /// \u{ff48}: 'ｈ'
    FullwidthLatinSmallLetterH,
    /// \u{ff49}: 'ｉ'
    FullwidthLatinSmallLetterI,
    /// \u{ff4a}: 'ｊ'
    FullwidthLatinSmallLetterJ,
    /// \u{ff4b}: 'ｋ'
    FullwidthLatinSmallLetterK,
    /// \u{ff4c}: 'ｌ'
    FullwidthLatinSmallLetterL,
    /// \u{ff4d}: 'ｍ'
    FullwidthLatinSmallLetterM,
    /// \u{ff4e}: 'ｎ'
    FullwidthLatinSmallLetterN,
    /// \u{ff4f}: 'ｏ'
    FullwidthLatinSmallLetterO,
    /// \u{ff50}: 'ｐ'
    FullwidthLatinSmallLetterP,
    /// \u{ff51}: 'ｑ'
    FullwidthLatinSmallLetterQ,
    /// \u{ff52}: 'ｒ'
    FullwidthLatinSmallLetterR,
    /// \u{ff53}: 'ｓ'
    FullwidthLatinSmallLetterS,
    /// \u{ff54}: 'ｔ'
    FullwidthLatinSmallLetterT,
    /// \u{ff55}: 'ｕ'
    FullwidthLatinSmallLetterU,
    /// \u{ff56}: 'ｖ'
    FullwidthLatinSmallLetterV,
    /// \u{ff57}: 'ｗ'
    FullwidthLatinSmallLetterW,
    /// \u{ff58}: 'ｘ'
    FullwidthLatinSmallLetterX,
    /// \u{ff59}: 'ｙ'
    FullwidthLatinSmallLetterY,
    /// \u{ff5a}: 'ｚ'
    FullwidthLatinSmallLetterZ,
    /// \u{ff5b}: '｛'
    FullwidthLeftCurlyBracket,
    /// \u{ff5c}: '｜'
    FullwidthVerticalLine,
    /// \u{ff5d}: '｝'
    FullwidthRightCurlyBracket,
    /// \u{ff5e}: '～'
    FullwidthTilde,
    /// \u{ff5f}: '｟'
    FullwidthLeftWhiteParenthesis,
    /// \u{ff60}: '｠'
    FullwidthRightWhiteParenthesis,
    /// \u{ff61}: '｡'
    HalfwidthIdeographicFullStop,
    /// \u{ff62}: '｢'
    HalfwidthLeftCornerBracket,
    /// \u{ff63}: '｣'
    HalfwidthRightCornerBracket,
    /// \u{ff64}: '､'
    HalfwidthIdeographicComma,
    /// \u{ff65}: '･'
    HalfwidthKatakanaMiddleDot,
    /// \u{ff66}: 'ｦ'
    HalfwidthKatakanaLetterWo,
    /// \u{ff67}: 'ｧ'
    HalfwidthKatakanaLetterSmallA,
    /// \u{ff68}: 'ｨ'
    HalfwidthKatakanaLetterSmallI,
    /// \u{ff69}: 'ｩ'
    HalfwidthKatakanaLetterSmallU,
    /// \u{ff6a}: 'ｪ'
    HalfwidthKatakanaLetterSmallE,
    /// \u{ff6b}: 'ｫ'
    HalfwidthKatakanaLetterSmallO,
    /// \u{ff6c}: 'ｬ'
    HalfwidthKatakanaLetterSmallYa,
    /// \u{ff6d}: 'ｭ'
    HalfwidthKatakanaLetterSmallYu,
    /// \u{ff6e}: 'ｮ'
    HalfwidthKatakanaLetterSmallYo,
    /// \u{ff6f}: 'ｯ'
    HalfwidthKatakanaLetterSmallTu,
    /// \u{ff70}: 'ｰ'
    HalfwidthKatakanaDashHiraganaProlongedSoundMark,
    /// \u{ff71}: 'ｱ'
    HalfwidthKatakanaLetterA,
    /// \u{ff72}: 'ｲ'
    HalfwidthKatakanaLetterI,
    /// \u{ff73}: 'ｳ'
    HalfwidthKatakanaLetterU,
    /// \u{ff74}: 'ｴ'
    HalfwidthKatakanaLetterE,
    /// \u{ff75}: 'ｵ'
    HalfwidthKatakanaLetterO,
    /// \u{ff76}: 'ｶ'
    HalfwidthKatakanaLetterKa,
    /// \u{ff77}: 'ｷ'
    HalfwidthKatakanaLetterKi,
    /// \u{ff78}: 'ｸ'
    HalfwidthKatakanaLetterKu,
    /// \u{ff79}: 'ｹ'
    HalfwidthKatakanaLetterKe,
    /// \u{ff7a}: 'ｺ'
    HalfwidthKatakanaLetterKo,
    /// \u{ff7b}: 'ｻ'
    HalfwidthKatakanaLetterSa,
    /// \u{ff7c}: 'ｼ'
    HalfwidthKatakanaLetterSi,
    /// \u{ff7d}: 'ｽ'
    HalfwidthKatakanaLetterSu,
    /// \u{ff7e}: 'ｾ'
    HalfwidthKatakanaLetterSe,
    /// \u{ff7f}: 'ｿ'
    HalfwidthKatakanaLetterSo,
    /// \u{ff80}: 'ﾀ'
    HalfwidthKatakanaLetterTa,
    /// \u{ff81}: 'ﾁ'
    HalfwidthKatakanaLetterTi,
    /// \u{ff82}: 'ﾂ'
    HalfwidthKatakanaLetterTu,
    /// \u{ff83}: 'ﾃ'
    HalfwidthKatakanaLetterTe,
    /// \u{ff84}: 'ﾄ'
    HalfwidthKatakanaLetterTo,
    /// \u{ff85}: 'ﾅ'
    HalfwidthKatakanaLetterNa,
    /// \u{ff86}: 'ﾆ'
    HalfwidthKatakanaLetterNi,
    /// \u{ff87}: 'ﾇ'
    HalfwidthKatakanaLetterNu,
    /// \u{ff88}: 'ﾈ'
    HalfwidthKatakanaLetterNe,
    /// \u{ff89}: 'ﾉ'
    HalfwidthKatakanaLetterNo,
    /// \u{ff8a}: 'ﾊ'
    HalfwidthKatakanaLetterHa,
    /// \u{ff8b}: 'ﾋ'
    HalfwidthKatakanaLetterHi,
    /// \u{ff8c}: 'ﾌ'
    HalfwidthKatakanaLetterHu,
    /// \u{ff8d}: 'ﾍ'
    HalfwidthKatakanaLetterHe,
    /// \u{ff8e}: 'ﾎ'
    HalfwidthKatakanaLetterHo,
    /// \u{ff8f}: 'ﾏ'
    HalfwidthKatakanaLetterMa,
    /// \u{ff90}: 'ﾐ'
    HalfwidthKatakanaLetterMi,
    /// \u{ff91}: 'ﾑ'
    HalfwidthKatakanaLetterMu,
    /// \u{ff92}: 'ﾒ'
    HalfwidthKatakanaLetterMe,
    /// \u{ff93}: 'ﾓ'
    HalfwidthKatakanaLetterMo,
    /// \u{ff94}: 'ﾔ'
    HalfwidthKatakanaLetterYa,
    /// \u{ff95}: 'ﾕ'
    HalfwidthKatakanaLetterYu,
    /// \u{ff96}: 'ﾖ'
    HalfwidthKatakanaLetterYo,
    /// \u{ff97}: 'ﾗ'
    HalfwidthKatakanaLetterRa,
    /// \u{ff98}: 'ﾘ'
    HalfwidthKatakanaLetterRi,
    /// \u{ff99}: 'ﾙ'
    HalfwidthKatakanaLetterRu,
    /// \u{ff9a}: 'ﾚ'
    HalfwidthKatakanaLetterRe,
    /// \u{ff9b}: 'ﾛ'
    HalfwidthKatakanaLetterRo,
    /// \u{ff9c}: 'ﾜ'
    HalfwidthKatakanaLetterWa,
    /// \u{ff9d}: 'ﾝ'
    HalfwidthKatakanaLetterN,
    /// \u{ff9e}: 'ﾞ'
    HalfwidthKatakanaVoicedSoundMark,
    /// \u{ff9f}: 'ﾟ'
    HalfwidthKatakanaSemiDashVoicedSoundMark,
    /// \u{ffa0}: 'ﾠ'
    HalfwidthHangulFiller,
    /// \u{ffa1}: 'ﾡ'
    HalfwidthHangulLetterKiyeok,
    /// \u{ffa2}: 'ﾢ'
    HalfwidthHangulLetterSsangkiyeok,
    /// \u{ffa3}: 'ﾣ'
    HalfwidthHangulLetterKiyeokDashSios,
    /// \u{ffa4}: 'ﾤ'
    HalfwidthHangulLetterNieun,
    /// \u{ffa5}: 'ﾥ'
    HalfwidthHangulLetterNieunDashCieuc,
    /// \u{ffa6}: 'ﾦ'
    HalfwidthHangulLetterNieunDashHieuh,
    /// \u{ffa7}: 'ﾧ'
    HalfwidthHangulLetterTikeut,
    /// \u{ffa8}: 'ﾨ'
    HalfwidthHangulLetterSsangtikeut,
    /// \u{ffa9}: 'ﾩ'
    HalfwidthHangulLetterRieul,
    /// \u{ffaa}: 'ﾪ'
    HalfwidthHangulLetterRieulDashKiyeok,
    /// \u{ffab}: 'ﾫ'
    HalfwidthHangulLetterRieulDashMieum,
    /// \u{ffac}: 'ﾬ'
    HalfwidthHangulLetterRieulDashPieup,
    /// \u{ffad}: 'ﾭ'
    HalfwidthHangulLetterRieulDashSios,
    /// \u{ffae}: 'ﾮ'
    HalfwidthHangulLetterRieulDashThieuth,
    /// \u{ffaf}: 'ﾯ'
    HalfwidthHangulLetterRieulDashPhieuph,
    /// \u{ffb0}: 'ﾰ'
    HalfwidthHangulLetterRieulDashHieuh,
    /// \u{ffb1}: 'ﾱ'
    HalfwidthHangulLetterMieum,
    /// \u{ffb2}: 'ﾲ'
    HalfwidthHangulLetterPieup,
    /// \u{ffb3}: 'ﾳ'
    HalfwidthHangulLetterSsangpieup,
    /// \u{ffb4}: 'ﾴ'
    HalfwidthHangulLetterPieupDashSios,
    /// \u{ffb5}: 'ﾵ'
    HalfwidthHangulLetterSios,
    /// \u{ffb6}: 'ﾶ'
    HalfwidthHangulLetterSsangsios,
    /// \u{ffb7}: 'ﾷ'
    HalfwidthHangulLetterIeung,
    /// \u{ffb8}: 'ﾸ'
    HalfwidthHangulLetterCieuc,
    /// \u{ffb9}: 'ﾹ'
    HalfwidthHangulLetterSsangcieuc,
    /// \u{ffba}: 'ﾺ'
    HalfwidthHangulLetterChieuch,
    /// \u{ffbb}: 'ﾻ'
    HalfwidthHangulLetterKhieukh,
    /// \u{ffbc}: 'ﾼ'
    HalfwidthHangulLetterThieuth,
    /// \u{ffbd}: 'ﾽ'
    HalfwidthHangulLetterPhieuph,
    /// \u{ffbe}: 'ﾾ'
    HalfwidthHangulLetterHieuh,
    /// \u{ffc2}: 'ￂ'
    HalfwidthHangulLetterA,
    /// \u{ffc3}: 'ￃ'
    HalfwidthHangulLetterAe,
    /// \u{ffc4}: 'ￄ'
    HalfwidthHangulLetterYa,
    /// \u{ffc5}: 'ￅ'
    HalfwidthHangulLetterYae,
    /// \u{ffc6}: 'ￆ'
    HalfwidthHangulLetterEo,
    /// \u{ffc7}: 'ￇ'
    HalfwidthHangulLetterE,
    /// \u{ffca}: 'ￊ'
    HalfwidthHangulLetterYeo,
    /// \u{ffcb}: 'ￋ'
    HalfwidthHangulLetterYe,
    /// \u{ffcc}: 'ￌ'
    HalfwidthHangulLetterO,
    /// \u{ffcd}: 'ￍ'
    HalfwidthHangulLetterWa,
    /// \u{ffce}: 'ￎ'
    HalfwidthHangulLetterWae,
    /// \u{ffcf}: 'ￏ'
    HalfwidthHangulLetterOe,
    /// \u{ffd2}: 'ￒ'
    HalfwidthHangulLetterYo,
    /// \u{ffd3}: 'ￓ'
    HalfwidthHangulLetterU,
    /// \u{ffd4}: 'ￔ'
    HalfwidthHangulLetterWeo,
    /// \u{ffd5}: 'ￕ'
    HalfwidthHangulLetterWe,
    /// \u{ffd6}: 'ￖ'
    HalfwidthHangulLetterWi,
    /// \u{ffd7}: 'ￗ'
    HalfwidthHangulLetterYu,
    /// \u{ffda}: 'ￚ'
    HalfwidthHangulLetterEu,
    /// \u{ffdb}: 'ￛ'
    HalfwidthHangulLetterYi,
    /// \u{ffdc}: 'ￜ'
    HalfwidthHangulLetterI,
    /// \u{ffe0}: '￠'
    FullwidthCentSign,
    /// \u{ffe1}: '￡'
    FullwidthPoundSign,
    /// \u{ffe2}: '￢'
    FullwidthNotSign,
    /// \u{ffe3}: '￣'
    FullwidthMacron,
    /// \u{ffe4}: '￤'
    FullwidthBrokenBar,
    /// \u{ffe5}: '￥'
    FullwidthYenSign,
    /// \u{ffe6}: '￦'
    FullwidthWonSign,
    /// \u{ffe8}: '￨'
    HalfwidthFormsLightVertical,
    /// \u{ffe9}: '￩'
    HalfwidthLeftwardsArrow,
    /// \u{ffea}: '￪'
    HalfwidthUpwardsArrow,
    /// \u{ffeb}: '￫'
    HalfwidthRightwardsArrow,
    /// \u{ffec}: '￬'
    HalfwidthDownwardsArrow,
    /// \u{ffed}: '￭'
    HalfwidthBlackSquare,
    /// \u{ffee}: '￮'
    HalfwidthWhiteCircle,
}

impl Into<char> for HalfwidthandFullwidthForms {
    fn into(self) -> char {
        use constants::*;
        match self {
            HalfwidthandFullwidthForms::FullwidthExclamationMark => FULLWIDTH_EXCLAMATION_MARK,
            HalfwidthandFullwidthForms::FullwidthQuotationMark => FULLWIDTH_QUOTATION_MARK,
            HalfwidthandFullwidthForms::FullwidthNumberSign => FULLWIDTH_NUMBER_SIGN,
            HalfwidthandFullwidthForms::FullwidthDollarSign => FULLWIDTH_DOLLAR_SIGN,
            HalfwidthandFullwidthForms::FullwidthPercentSign => FULLWIDTH_PERCENT_SIGN,
            HalfwidthandFullwidthForms::FullwidthAmpersand => FULLWIDTH_AMPERSAND,
            HalfwidthandFullwidthForms::FullwidthApostrophe => FULLWIDTH_APOSTROPHE,
            HalfwidthandFullwidthForms::FullwidthLeftParenthesis => FULLWIDTH_LEFT_PARENTHESIS,
            HalfwidthandFullwidthForms::FullwidthRightParenthesis => FULLWIDTH_RIGHT_PARENTHESIS,
            HalfwidthandFullwidthForms::FullwidthAsterisk => FULLWIDTH_ASTERISK,
            HalfwidthandFullwidthForms::FullwidthPlusSign => FULLWIDTH_PLUS_SIGN,
            HalfwidthandFullwidthForms::FullwidthComma => FULLWIDTH_COMMA,
            HalfwidthandFullwidthForms::FullwidthHyphenDashMinus => FULLWIDTH_HYPHEN_DASH_MINUS,
            HalfwidthandFullwidthForms::FullwidthFullStop => FULLWIDTH_FULL_STOP,
            HalfwidthandFullwidthForms::FullwidthSolidus => FULLWIDTH_SOLIDUS,
            HalfwidthandFullwidthForms::FullwidthDigitZero => FULLWIDTH_DIGIT_ZERO,
            HalfwidthandFullwidthForms::FullwidthDigitOne => FULLWIDTH_DIGIT_ONE,
            HalfwidthandFullwidthForms::FullwidthDigitTwo => FULLWIDTH_DIGIT_TWO,
            HalfwidthandFullwidthForms::FullwidthDigitThree => FULLWIDTH_DIGIT_THREE,
            HalfwidthandFullwidthForms::FullwidthDigitFour => FULLWIDTH_DIGIT_FOUR,
            HalfwidthandFullwidthForms::FullwidthDigitFive => FULLWIDTH_DIGIT_FIVE,
            HalfwidthandFullwidthForms::FullwidthDigitSix => FULLWIDTH_DIGIT_SIX,
            HalfwidthandFullwidthForms::FullwidthDigitSeven => FULLWIDTH_DIGIT_SEVEN,
            HalfwidthandFullwidthForms::FullwidthDigitEight => FULLWIDTH_DIGIT_EIGHT,
            HalfwidthandFullwidthForms::FullwidthDigitNine => FULLWIDTH_DIGIT_NINE,
            HalfwidthandFullwidthForms::FullwidthColon => FULLWIDTH_COLON,
            HalfwidthandFullwidthForms::FullwidthSemicolon => FULLWIDTH_SEMICOLON,
            HalfwidthandFullwidthForms::FullwidthLessDashThanSign => FULLWIDTH_LESS_DASH_THAN_SIGN,
            HalfwidthandFullwidthForms::FullwidthEqualsSign => FULLWIDTH_EQUALS_SIGN,
            HalfwidthandFullwidthForms::FullwidthGreaterDashThanSign => FULLWIDTH_GREATER_DASH_THAN_SIGN,
            HalfwidthandFullwidthForms::FullwidthQuestionMark => FULLWIDTH_QUESTION_MARK,
            HalfwidthandFullwidthForms::FullwidthCommercialAt => FULLWIDTH_COMMERCIAL_AT,
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterA => FULLWIDTH_LATIN_CAPITAL_LETTER_A,
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterB => FULLWIDTH_LATIN_CAPITAL_LETTER_B,
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterC => FULLWIDTH_LATIN_CAPITAL_LETTER_C,
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterD => FULLWIDTH_LATIN_CAPITAL_LETTER_D,
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterE => FULLWIDTH_LATIN_CAPITAL_LETTER_E,
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterF => FULLWIDTH_LATIN_CAPITAL_LETTER_F,
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterG => FULLWIDTH_LATIN_CAPITAL_LETTER_G,
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterH => FULLWIDTH_LATIN_CAPITAL_LETTER_H,
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterI => FULLWIDTH_LATIN_CAPITAL_LETTER_I,
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterJ => FULLWIDTH_LATIN_CAPITAL_LETTER_J,
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterK => FULLWIDTH_LATIN_CAPITAL_LETTER_K,
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterL => FULLWIDTH_LATIN_CAPITAL_LETTER_L,
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterM => FULLWIDTH_LATIN_CAPITAL_LETTER_M,
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterN => FULLWIDTH_LATIN_CAPITAL_LETTER_N,
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterO => FULLWIDTH_LATIN_CAPITAL_LETTER_O,
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterP => FULLWIDTH_LATIN_CAPITAL_LETTER_P,
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterQ => FULLWIDTH_LATIN_CAPITAL_LETTER_Q,
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterR => FULLWIDTH_LATIN_CAPITAL_LETTER_R,
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterS => FULLWIDTH_LATIN_CAPITAL_LETTER_S,
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterT => FULLWIDTH_LATIN_CAPITAL_LETTER_T,
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterU => FULLWIDTH_LATIN_CAPITAL_LETTER_U,
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterV => FULLWIDTH_LATIN_CAPITAL_LETTER_V,
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterW => FULLWIDTH_LATIN_CAPITAL_LETTER_W,
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterX => FULLWIDTH_LATIN_CAPITAL_LETTER_X,
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterY => FULLWIDTH_LATIN_CAPITAL_LETTER_Y,
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterZ => FULLWIDTH_LATIN_CAPITAL_LETTER_Z,
            HalfwidthandFullwidthForms::FullwidthLeftSquareBracket => FULLWIDTH_LEFT_SQUARE_BRACKET,
            HalfwidthandFullwidthForms::FullwidthReverseSolidus => FULLWIDTH_REVERSE_SOLIDUS,
            HalfwidthandFullwidthForms::FullwidthRightSquareBracket => FULLWIDTH_RIGHT_SQUARE_BRACKET,
            HalfwidthandFullwidthForms::FullwidthCircumflexAccent => FULLWIDTH_CIRCUMFLEX_ACCENT,
            HalfwidthandFullwidthForms::FullwidthLowLine => FULLWIDTH_LOW_LINE,
            HalfwidthandFullwidthForms::FullwidthGraveAccent => FULLWIDTH_GRAVE_ACCENT,
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterA => FULLWIDTH_LATIN_SMALL_LETTER_A,
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterB => FULLWIDTH_LATIN_SMALL_LETTER_B,
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterC => FULLWIDTH_LATIN_SMALL_LETTER_C,
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterD => FULLWIDTH_LATIN_SMALL_LETTER_D,
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterE => FULLWIDTH_LATIN_SMALL_LETTER_E,
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterF => FULLWIDTH_LATIN_SMALL_LETTER_F,
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterG => FULLWIDTH_LATIN_SMALL_LETTER_G,
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterH => FULLWIDTH_LATIN_SMALL_LETTER_H,
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterI => FULLWIDTH_LATIN_SMALL_LETTER_I,
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterJ => FULLWIDTH_LATIN_SMALL_LETTER_J,
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterK => FULLWIDTH_LATIN_SMALL_LETTER_K,
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterL => FULLWIDTH_LATIN_SMALL_LETTER_L,
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterM => FULLWIDTH_LATIN_SMALL_LETTER_M,
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterN => FULLWIDTH_LATIN_SMALL_LETTER_N,
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterO => FULLWIDTH_LATIN_SMALL_LETTER_O,
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterP => FULLWIDTH_LATIN_SMALL_LETTER_P,
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterQ => FULLWIDTH_LATIN_SMALL_LETTER_Q,
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterR => FULLWIDTH_LATIN_SMALL_LETTER_R,
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterS => FULLWIDTH_LATIN_SMALL_LETTER_S,
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterT => FULLWIDTH_LATIN_SMALL_LETTER_T,
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterU => FULLWIDTH_LATIN_SMALL_LETTER_U,
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterV => FULLWIDTH_LATIN_SMALL_LETTER_V,
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterW => FULLWIDTH_LATIN_SMALL_LETTER_W,
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterX => FULLWIDTH_LATIN_SMALL_LETTER_X,
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterY => FULLWIDTH_LATIN_SMALL_LETTER_Y,
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterZ => FULLWIDTH_LATIN_SMALL_LETTER_Z,
            HalfwidthandFullwidthForms::FullwidthLeftCurlyBracket => FULLWIDTH_LEFT_CURLY_BRACKET,
            HalfwidthandFullwidthForms::FullwidthVerticalLine => FULLWIDTH_VERTICAL_LINE,
            HalfwidthandFullwidthForms::FullwidthRightCurlyBracket => FULLWIDTH_RIGHT_CURLY_BRACKET,
            HalfwidthandFullwidthForms::FullwidthTilde => FULLWIDTH_TILDE,
            HalfwidthandFullwidthForms::FullwidthLeftWhiteParenthesis => FULLWIDTH_LEFT_WHITE_PARENTHESIS,
            HalfwidthandFullwidthForms::FullwidthRightWhiteParenthesis => FULLWIDTH_RIGHT_WHITE_PARENTHESIS,
            HalfwidthandFullwidthForms::HalfwidthIdeographicFullStop => HALFWIDTH_IDEOGRAPHIC_FULL_STOP,
            HalfwidthandFullwidthForms::HalfwidthLeftCornerBracket => HALFWIDTH_LEFT_CORNER_BRACKET,
            HalfwidthandFullwidthForms::HalfwidthRightCornerBracket => HALFWIDTH_RIGHT_CORNER_BRACKET,
            HalfwidthandFullwidthForms::HalfwidthIdeographicComma => HALFWIDTH_IDEOGRAPHIC_COMMA,
            HalfwidthandFullwidthForms::HalfwidthKatakanaMiddleDot => HALFWIDTH_KATAKANA_MIDDLE_DOT,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterWo => HALFWIDTH_KATAKANA_LETTER_WO,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallA => HALFWIDTH_KATAKANA_LETTER_SMALL_A,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallI => HALFWIDTH_KATAKANA_LETTER_SMALL_I,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallU => HALFWIDTH_KATAKANA_LETTER_SMALL_U,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallE => HALFWIDTH_KATAKANA_LETTER_SMALL_E,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallO => HALFWIDTH_KATAKANA_LETTER_SMALL_O,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallYa => HALFWIDTH_KATAKANA_LETTER_SMALL_YA,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallYu => HALFWIDTH_KATAKANA_LETTER_SMALL_YU,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallYo => HALFWIDTH_KATAKANA_LETTER_SMALL_YO,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallTu => HALFWIDTH_KATAKANA_LETTER_SMALL_TU,
            HalfwidthandFullwidthForms::HalfwidthKatakanaDashHiraganaProlongedSoundMark => HALFWIDTH_KATAKANA_DASH_HIRAGANA_PROLONGED_SOUND_MARK,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterA => HALFWIDTH_KATAKANA_LETTER_A,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterI => HALFWIDTH_KATAKANA_LETTER_I,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterU => HALFWIDTH_KATAKANA_LETTER_U,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterE => HALFWIDTH_KATAKANA_LETTER_E,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterO => HALFWIDTH_KATAKANA_LETTER_O,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterKa => HALFWIDTH_KATAKANA_LETTER_KA,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterKi => HALFWIDTH_KATAKANA_LETTER_KI,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterKu => HALFWIDTH_KATAKANA_LETTER_KU,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterKe => HALFWIDTH_KATAKANA_LETTER_KE,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterKo => HALFWIDTH_KATAKANA_LETTER_KO,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSa => HALFWIDTH_KATAKANA_LETTER_SA,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSi => HALFWIDTH_KATAKANA_LETTER_SI,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSu => HALFWIDTH_KATAKANA_LETTER_SU,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSe => HALFWIDTH_KATAKANA_LETTER_SE,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSo => HALFWIDTH_KATAKANA_LETTER_SO,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterTa => HALFWIDTH_KATAKANA_LETTER_TA,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterTi => HALFWIDTH_KATAKANA_LETTER_TI,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterTu => HALFWIDTH_KATAKANA_LETTER_TU,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterTe => HALFWIDTH_KATAKANA_LETTER_TE,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterTo => HALFWIDTH_KATAKANA_LETTER_TO,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterNa => HALFWIDTH_KATAKANA_LETTER_NA,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterNi => HALFWIDTH_KATAKANA_LETTER_NI,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterNu => HALFWIDTH_KATAKANA_LETTER_NU,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterNe => HALFWIDTH_KATAKANA_LETTER_NE,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterNo => HALFWIDTH_KATAKANA_LETTER_NO,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterHa => HALFWIDTH_KATAKANA_LETTER_HA,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterHi => HALFWIDTH_KATAKANA_LETTER_HI,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterHu => HALFWIDTH_KATAKANA_LETTER_HU,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterHe => HALFWIDTH_KATAKANA_LETTER_HE,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterHo => HALFWIDTH_KATAKANA_LETTER_HO,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterMa => HALFWIDTH_KATAKANA_LETTER_MA,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterMi => HALFWIDTH_KATAKANA_LETTER_MI,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterMu => HALFWIDTH_KATAKANA_LETTER_MU,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterMe => HALFWIDTH_KATAKANA_LETTER_ME,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterMo => HALFWIDTH_KATAKANA_LETTER_MO,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterYa => HALFWIDTH_KATAKANA_LETTER_YA,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterYu => HALFWIDTH_KATAKANA_LETTER_YU,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterYo => HALFWIDTH_KATAKANA_LETTER_YO,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterRa => HALFWIDTH_KATAKANA_LETTER_RA,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterRi => HALFWIDTH_KATAKANA_LETTER_RI,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterRu => HALFWIDTH_KATAKANA_LETTER_RU,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterRe => HALFWIDTH_KATAKANA_LETTER_RE,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterRo => HALFWIDTH_KATAKANA_LETTER_RO,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterWa => HALFWIDTH_KATAKANA_LETTER_WA,
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterN => HALFWIDTH_KATAKANA_LETTER_N,
            HalfwidthandFullwidthForms::HalfwidthKatakanaVoicedSoundMark => HALFWIDTH_KATAKANA_VOICED_SOUND_MARK,
            HalfwidthandFullwidthForms::HalfwidthKatakanaSemiDashVoicedSoundMark => HALFWIDTH_KATAKANA_SEMI_DASH_VOICED_SOUND_MARK,
            HalfwidthandFullwidthForms::HalfwidthHangulFiller => HALFWIDTH_HANGUL_FILLER,
            HalfwidthandFullwidthForms::HalfwidthHangulLetterKiyeok => HALFWIDTH_HANGUL_LETTER_KIYEOK,
            HalfwidthandFullwidthForms::HalfwidthHangulLetterSsangkiyeok => HALFWIDTH_HANGUL_LETTER_SSANGKIYEOK,
            HalfwidthandFullwidthForms::HalfwidthHangulLetterKiyeokDashSios => HALFWIDTH_HANGUL_LETTER_KIYEOK_DASH_SIOS,
            HalfwidthandFullwidthForms::HalfwidthHangulLetterNieun => HALFWIDTH_HANGUL_LETTER_NIEUN,
            HalfwidthandFullwidthForms::HalfwidthHangulLetterNieunDashCieuc => HALFWIDTH_HANGUL_LETTER_NIEUN_DASH_CIEUC,
            HalfwidthandFullwidthForms::HalfwidthHangulLetterNieunDashHieuh => HALFWIDTH_HANGUL_LETTER_NIEUN_DASH_HIEUH,
            HalfwidthandFullwidthForms::HalfwidthHangulLetterTikeut => HALFWIDTH_HANGUL_LETTER_TIKEUT,
            HalfwidthandFullwidthForms::HalfwidthHangulLetterSsangtikeut => HALFWIDTH_HANGUL_LETTER_SSANGTIKEUT,
            HalfwidthandFullwidthForms::HalfwidthHangulLetterRieul => HALFWIDTH_HANGUL_LETTER_RIEUL,
            HalfwidthandFullwidthForms::HalfwidthHangulLetterRieulDashKiyeok => HALFWIDTH_HANGUL_LETTER_RIEUL_DASH_KIYEOK,
            HalfwidthandFullwidthForms::HalfwidthHangulLetterRieulDashMieum => HALFWIDTH_HANGUL_LETTER_RIEUL_DASH_MIEUM,
            HalfwidthandFullwidthForms::HalfwidthHangulLetterRieulDashPieup => HALFWIDTH_HANGUL_LETTER_RIEUL_DASH_PIEUP,
            HalfwidthandFullwidthForms::HalfwidthHangulLetterRieulDashSios => HALFWIDTH_HANGUL_LETTER_RIEUL_DASH_SIOS,
            HalfwidthandFullwidthForms::HalfwidthHangulLetterRieulDashThieuth => HALFWIDTH_HANGUL_LETTER_RIEUL_DASH_THIEUTH,
            HalfwidthandFullwidthForms::HalfwidthHangulLetterRieulDashPhieuph => HALFWIDTH_HANGUL_LETTER_RIEUL_DASH_PHIEUPH,
            HalfwidthandFullwidthForms::HalfwidthHangulLetterRieulDashHieuh => HALFWIDTH_HANGUL_LETTER_RIEUL_DASH_HIEUH,
            HalfwidthandFullwidthForms::HalfwidthHangulLetterMieum => HALFWIDTH_HANGUL_LETTER_MIEUM,
            HalfwidthandFullwidthForms::HalfwidthHangulLetterPieup => HALFWIDTH_HANGUL_LETTER_PIEUP,
            HalfwidthandFullwidthForms::HalfwidthHangulLetterSsangpieup => HALFWIDTH_HANGUL_LETTER_SSANGPIEUP,
            HalfwidthandFullwidthForms::HalfwidthHangulLetterPieupDashSios => HALFWIDTH_HANGUL_LETTER_PIEUP_DASH_SIOS,
            HalfwidthandFullwidthForms::HalfwidthHangulLetterSios => HALFWIDTH_HANGUL_LETTER_SIOS,
            HalfwidthandFullwidthForms::HalfwidthHangulLetterSsangsios => HALFWIDTH_HANGUL_LETTER_SSANGSIOS,
            HalfwidthandFullwidthForms::HalfwidthHangulLetterIeung => HALFWIDTH_HANGUL_LETTER_IEUNG,
            HalfwidthandFullwidthForms::HalfwidthHangulLetterCieuc => HALFWIDTH_HANGUL_LETTER_CIEUC,
            HalfwidthandFullwidthForms::HalfwidthHangulLetterSsangcieuc => HALFWIDTH_HANGUL_LETTER_SSANGCIEUC,
            HalfwidthandFullwidthForms::HalfwidthHangulLetterChieuch => HALFWIDTH_HANGUL_LETTER_CHIEUCH,
            HalfwidthandFullwidthForms::HalfwidthHangulLetterKhieukh => HALFWIDTH_HANGUL_LETTER_KHIEUKH,
            HalfwidthandFullwidthForms::HalfwidthHangulLetterThieuth => HALFWIDTH_HANGUL_LETTER_THIEUTH,
            HalfwidthandFullwidthForms::HalfwidthHangulLetterPhieuph => HALFWIDTH_HANGUL_LETTER_PHIEUPH,
            HalfwidthandFullwidthForms::HalfwidthHangulLetterHieuh => HALFWIDTH_HANGUL_LETTER_HIEUH,
            HalfwidthandFullwidthForms::HalfwidthHangulLetterA => HALFWIDTH_HANGUL_LETTER_A,
            HalfwidthandFullwidthForms::HalfwidthHangulLetterAe => HALFWIDTH_HANGUL_LETTER_AE,
            HalfwidthandFullwidthForms::HalfwidthHangulLetterYa => HALFWIDTH_HANGUL_LETTER_YA,
            HalfwidthandFullwidthForms::HalfwidthHangulLetterYae => HALFWIDTH_HANGUL_LETTER_YAE,
            HalfwidthandFullwidthForms::HalfwidthHangulLetterEo => HALFWIDTH_HANGUL_LETTER_EO,
            HalfwidthandFullwidthForms::HalfwidthHangulLetterE => HALFWIDTH_HANGUL_LETTER_E,
            HalfwidthandFullwidthForms::HalfwidthHangulLetterYeo => HALFWIDTH_HANGUL_LETTER_YEO,
            HalfwidthandFullwidthForms::HalfwidthHangulLetterYe => HALFWIDTH_HANGUL_LETTER_YE,
            HalfwidthandFullwidthForms::HalfwidthHangulLetterO => HALFWIDTH_HANGUL_LETTER_O,
            HalfwidthandFullwidthForms::HalfwidthHangulLetterWa => HALFWIDTH_HANGUL_LETTER_WA,
            HalfwidthandFullwidthForms::HalfwidthHangulLetterWae => HALFWIDTH_HANGUL_LETTER_WAE,
            HalfwidthandFullwidthForms::HalfwidthHangulLetterOe => HALFWIDTH_HANGUL_LETTER_OE,
            HalfwidthandFullwidthForms::HalfwidthHangulLetterYo => HALFWIDTH_HANGUL_LETTER_YO,
            HalfwidthandFullwidthForms::HalfwidthHangulLetterU => HALFWIDTH_HANGUL_LETTER_U,
            HalfwidthandFullwidthForms::HalfwidthHangulLetterWeo => HALFWIDTH_HANGUL_LETTER_WEO,
            HalfwidthandFullwidthForms::HalfwidthHangulLetterWe => HALFWIDTH_HANGUL_LETTER_WE,
            HalfwidthandFullwidthForms::HalfwidthHangulLetterWi => HALFWIDTH_HANGUL_LETTER_WI,
            HalfwidthandFullwidthForms::HalfwidthHangulLetterYu => HALFWIDTH_HANGUL_LETTER_YU,
            HalfwidthandFullwidthForms::HalfwidthHangulLetterEu => HALFWIDTH_HANGUL_LETTER_EU,
            HalfwidthandFullwidthForms::HalfwidthHangulLetterYi => HALFWIDTH_HANGUL_LETTER_YI,
            HalfwidthandFullwidthForms::HalfwidthHangulLetterI => HALFWIDTH_HANGUL_LETTER_I,
            HalfwidthandFullwidthForms::FullwidthCentSign => FULLWIDTH_CENT_SIGN,
            HalfwidthandFullwidthForms::FullwidthPoundSign => FULLWIDTH_POUND_SIGN,
            HalfwidthandFullwidthForms::FullwidthNotSign => FULLWIDTH_NOT_SIGN,
            HalfwidthandFullwidthForms::FullwidthMacron => FULLWIDTH_MACRON,
            HalfwidthandFullwidthForms::FullwidthBrokenBar => FULLWIDTH_BROKEN_BAR,
            HalfwidthandFullwidthForms::FullwidthYenSign => FULLWIDTH_YEN_SIGN,
            HalfwidthandFullwidthForms::FullwidthWonSign => FULLWIDTH_WON_SIGN,
            HalfwidthandFullwidthForms::HalfwidthFormsLightVertical => HALFWIDTH_FORMS_LIGHT_VERTICAL,
            HalfwidthandFullwidthForms::HalfwidthLeftwardsArrow => HALFWIDTH_LEFTWARDS_ARROW,
            HalfwidthandFullwidthForms::HalfwidthUpwardsArrow => HALFWIDTH_UPWARDS_ARROW,
            HalfwidthandFullwidthForms::HalfwidthRightwardsArrow => HALFWIDTH_RIGHTWARDS_ARROW,
            HalfwidthandFullwidthForms::HalfwidthDownwardsArrow => HALFWIDTH_DOWNWARDS_ARROW,
            HalfwidthandFullwidthForms::HalfwidthBlackSquare => HALFWIDTH_BLACK_SQUARE,
            HalfwidthandFullwidthForms::HalfwidthWhiteCircle => HALFWIDTH_WHITE_CIRCLE,
        }
    }
}

impl std::convert::TryFrom<char> for HalfwidthandFullwidthForms {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            FULLWIDTH_EXCLAMATION_MARK => Ok(HalfwidthandFullwidthForms::FullwidthExclamationMark),
            FULLWIDTH_QUOTATION_MARK => Ok(HalfwidthandFullwidthForms::FullwidthQuotationMark),
            FULLWIDTH_NUMBER_SIGN => Ok(HalfwidthandFullwidthForms::FullwidthNumberSign),
            FULLWIDTH_DOLLAR_SIGN => Ok(HalfwidthandFullwidthForms::FullwidthDollarSign),
            FULLWIDTH_PERCENT_SIGN => Ok(HalfwidthandFullwidthForms::FullwidthPercentSign),
            FULLWIDTH_AMPERSAND => Ok(HalfwidthandFullwidthForms::FullwidthAmpersand),
            FULLWIDTH_APOSTROPHE => Ok(HalfwidthandFullwidthForms::FullwidthApostrophe),
            FULLWIDTH_LEFT_PARENTHESIS => Ok(HalfwidthandFullwidthForms::FullwidthLeftParenthesis),
            FULLWIDTH_RIGHT_PARENTHESIS => Ok(HalfwidthandFullwidthForms::FullwidthRightParenthesis),
            FULLWIDTH_ASTERISK => Ok(HalfwidthandFullwidthForms::FullwidthAsterisk),
            FULLWIDTH_PLUS_SIGN => Ok(HalfwidthandFullwidthForms::FullwidthPlusSign),
            FULLWIDTH_COMMA => Ok(HalfwidthandFullwidthForms::FullwidthComma),
            FULLWIDTH_HYPHEN_DASH_MINUS => Ok(HalfwidthandFullwidthForms::FullwidthHyphenDashMinus),
            FULLWIDTH_FULL_STOP => Ok(HalfwidthandFullwidthForms::FullwidthFullStop),
            FULLWIDTH_SOLIDUS => Ok(HalfwidthandFullwidthForms::FullwidthSolidus),
            FULLWIDTH_DIGIT_ZERO => Ok(HalfwidthandFullwidthForms::FullwidthDigitZero),
            FULLWIDTH_DIGIT_ONE => Ok(HalfwidthandFullwidthForms::FullwidthDigitOne),
            FULLWIDTH_DIGIT_TWO => Ok(HalfwidthandFullwidthForms::FullwidthDigitTwo),
            FULLWIDTH_DIGIT_THREE => Ok(HalfwidthandFullwidthForms::FullwidthDigitThree),
            FULLWIDTH_DIGIT_FOUR => Ok(HalfwidthandFullwidthForms::FullwidthDigitFour),
            FULLWIDTH_DIGIT_FIVE => Ok(HalfwidthandFullwidthForms::FullwidthDigitFive),
            FULLWIDTH_DIGIT_SIX => Ok(HalfwidthandFullwidthForms::FullwidthDigitSix),
            FULLWIDTH_DIGIT_SEVEN => Ok(HalfwidthandFullwidthForms::FullwidthDigitSeven),
            FULLWIDTH_DIGIT_EIGHT => Ok(HalfwidthandFullwidthForms::FullwidthDigitEight),
            FULLWIDTH_DIGIT_NINE => Ok(HalfwidthandFullwidthForms::FullwidthDigitNine),
            FULLWIDTH_COLON => Ok(HalfwidthandFullwidthForms::FullwidthColon),
            FULLWIDTH_SEMICOLON => Ok(HalfwidthandFullwidthForms::FullwidthSemicolon),
            FULLWIDTH_LESS_DASH_THAN_SIGN => Ok(HalfwidthandFullwidthForms::FullwidthLessDashThanSign),
            FULLWIDTH_EQUALS_SIGN => Ok(HalfwidthandFullwidthForms::FullwidthEqualsSign),
            FULLWIDTH_GREATER_DASH_THAN_SIGN => Ok(HalfwidthandFullwidthForms::FullwidthGreaterDashThanSign),
            FULLWIDTH_QUESTION_MARK => Ok(HalfwidthandFullwidthForms::FullwidthQuestionMark),
            FULLWIDTH_COMMERCIAL_AT => Ok(HalfwidthandFullwidthForms::FullwidthCommercialAt),
            FULLWIDTH_LATIN_CAPITAL_LETTER_A => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterA),
            FULLWIDTH_LATIN_CAPITAL_LETTER_B => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterB),
            FULLWIDTH_LATIN_CAPITAL_LETTER_C => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterC),
            FULLWIDTH_LATIN_CAPITAL_LETTER_D => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterD),
            FULLWIDTH_LATIN_CAPITAL_LETTER_E => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterE),
            FULLWIDTH_LATIN_CAPITAL_LETTER_F => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterF),
            FULLWIDTH_LATIN_CAPITAL_LETTER_G => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterG),
            FULLWIDTH_LATIN_CAPITAL_LETTER_H => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterH),
            FULLWIDTH_LATIN_CAPITAL_LETTER_I => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterI),
            FULLWIDTH_LATIN_CAPITAL_LETTER_J => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterJ),
            FULLWIDTH_LATIN_CAPITAL_LETTER_K => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterK),
            FULLWIDTH_LATIN_CAPITAL_LETTER_L => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterL),
            FULLWIDTH_LATIN_CAPITAL_LETTER_M => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterM),
            FULLWIDTH_LATIN_CAPITAL_LETTER_N => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterN),
            FULLWIDTH_LATIN_CAPITAL_LETTER_O => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterO),
            FULLWIDTH_LATIN_CAPITAL_LETTER_P => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterP),
            FULLWIDTH_LATIN_CAPITAL_LETTER_Q => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterQ),
            FULLWIDTH_LATIN_CAPITAL_LETTER_R => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterR),
            FULLWIDTH_LATIN_CAPITAL_LETTER_S => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterS),
            FULLWIDTH_LATIN_CAPITAL_LETTER_T => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterT),
            FULLWIDTH_LATIN_CAPITAL_LETTER_U => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterU),
            FULLWIDTH_LATIN_CAPITAL_LETTER_V => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterV),
            FULLWIDTH_LATIN_CAPITAL_LETTER_W => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterW),
            FULLWIDTH_LATIN_CAPITAL_LETTER_X => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterX),
            FULLWIDTH_LATIN_CAPITAL_LETTER_Y => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterY),
            FULLWIDTH_LATIN_CAPITAL_LETTER_Z => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterZ),
            FULLWIDTH_LEFT_SQUARE_BRACKET => Ok(HalfwidthandFullwidthForms::FullwidthLeftSquareBracket),
            FULLWIDTH_REVERSE_SOLIDUS => Ok(HalfwidthandFullwidthForms::FullwidthReverseSolidus),
            FULLWIDTH_RIGHT_SQUARE_BRACKET => Ok(HalfwidthandFullwidthForms::FullwidthRightSquareBracket),
            FULLWIDTH_CIRCUMFLEX_ACCENT => Ok(HalfwidthandFullwidthForms::FullwidthCircumflexAccent),
            FULLWIDTH_LOW_LINE => Ok(HalfwidthandFullwidthForms::FullwidthLowLine),
            FULLWIDTH_GRAVE_ACCENT => Ok(HalfwidthandFullwidthForms::FullwidthGraveAccent),
            FULLWIDTH_LATIN_SMALL_LETTER_A => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterA),
            FULLWIDTH_LATIN_SMALL_LETTER_B => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterB),
            FULLWIDTH_LATIN_SMALL_LETTER_C => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterC),
            FULLWIDTH_LATIN_SMALL_LETTER_D => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterD),
            FULLWIDTH_LATIN_SMALL_LETTER_E => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterE),
            FULLWIDTH_LATIN_SMALL_LETTER_F => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterF),
            FULLWIDTH_LATIN_SMALL_LETTER_G => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterG),
            FULLWIDTH_LATIN_SMALL_LETTER_H => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterH),
            FULLWIDTH_LATIN_SMALL_LETTER_I => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterI),
            FULLWIDTH_LATIN_SMALL_LETTER_J => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterJ),
            FULLWIDTH_LATIN_SMALL_LETTER_K => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterK),
            FULLWIDTH_LATIN_SMALL_LETTER_L => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterL),
            FULLWIDTH_LATIN_SMALL_LETTER_M => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterM),
            FULLWIDTH_LATIN_SMALL_LETTER_N => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterN),
            FULLWIDTH_LATIN_SMALL_LETTER_O => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterO),
            FULLWIDTH_LATIN_SMALL_LETTER_P => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterP),
            FULLWIDTH_LATIN_SMALL_LETTER_Q => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterQ),
            FULLWIDTH_LATIN_SMALL_LETTER_R => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterR),
            FULLWIDTH_LATIN_SMALL_LETTER_S => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterS),
            FULLWIDTH_LATIN_SMALL_LETTER_T => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterT),
            FULLWIDTH_LATIN_SMALL_LETTER_U => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterU),
            FULLWIDTH_LATIN_SMALL_LETTER_V => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterV),
            FULLWIDTH_LATIN_SMALL_LETTER_W => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterW),
            FULLWIDTH_LATIN_SMALL_LETTER_X => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterX),
            FULLWIDTH_LATIN_SMALL_LETTER_Y => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterY),
            FULLWIDTH_LATIN_SMALL_LETTER_Z => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterZ),
            FULLWIDTH_LEFT_CURLY_BRACKET => Ok(HalfwidthandFullwidthForms::FullwidthLeftCurlyBracket),
            FULLWIDTH_VERTICAL_LINE => Ok(HalfwidthandFullwidthForms::FullwidthVerticalLine),
            FULLWIDTH_RIGHT_CURLY_BRACKET => Ok(HalfwidthandFullwidthForms::FullwidthRightCurlyBracket),
            FULLWIDTH_TILDE => Ok(HalfwidthandFullwidthForms::FullwidthTilde),
            FULLWIDTH_LEFT_WHITE_PARENTHESIS => Ok(HalfwidthandFullwidthForms::FullwidthLeftWhiteParenthesis),
            FULLWIDTH_RIGHT_WHITE_PARENTHESIS => Ok(HalfwidthandFullwidthForms::FullwidthRightWhiteParenthesis),
            HALFWIDTH_IDEOGRAPHIC_FULL_STOP => Ok(HalfwidthandFullwidthForms::HalfwidthIdeographicFullStop),
            HALFWIDTH_LEFT_CORNER_BRACKET => Ok(HalfwidthandFullwidthForms::HalfwidthLeftCornerBracket),
            HALFWIDTH_RIGHT_CORNER_BRACKET => Ok(HalfwidthandFullwidthForms::HalfwidthRightCornerBracket),
            HALFWIDTH_IDEOGRAPHIC_COMMA => Ok(HalfwidthandFullwidthForms::HalfwidthIdeographicComma),
            HALFWIDTH_KATAKANA_MIDDLE_DOT => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaMiddleDot),
            HALFWIDTH_KATAKANA_LETTER_WO => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterWo),
            HALFWIDTH_KATAKANA_LETTER_SMALL_A => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallA),
            HALFWIDTH_KATAKANA_LETTER_SMALL_I => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallI),
            HALFWIDTH_KATAKANA_LETTER_SMALL_U => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallU),
            HALFWIDTH_KATAKANA_LETTER_SMALL_E => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallE),
            HALFWIDTH_KATAKANA_LETTER_SMALL_O => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallO),
            HALFWIDTH_KATAKANA_LETTER_SMALL_YA => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallYa),
            HALFWIDTH_KATAKANA_LETTER_SMALL_YU => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallYu),
            HALFWIDTH_KATAKANA_LETTER_SMALL_YO => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallYo),
            HALFWIDTH_KATAKANA_LETTER_SMALL_TU => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallTu),
            HALFWIDTH_KATAKANA_DASH_HIRAGANA_PROLONGED_SOUND_MARK => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaDashHiraganaProlongedSoundMark),
            HALFWIDTH_KATAKANA_LETTER_A => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterA),
            HALFWIDTH_KATAKANA_LETTER_I => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterI),
            HALFWIDTH_KATAKANA_LETTER_U => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterU),
            HALFWIDTH_KATAKANA_LETTER_E => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterE),
            HALFWIDTH_KATAKANA_LETTER_O => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterO),
            HALFWIDTH_KATAKANA_LETTER_KA => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterKa),
            HALFWIDTH_KATAKANA_LETTER_KI => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterKi),
            HALFWIDTH_KATAKANA_LETTER_KU => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterKu),
            HALFWIDTH_KATAKANA_LETTER_KE => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterKe),
            HALFWIDTH_KATAKANA_LETTER_KO => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterKo),
            HALFWIDTH_KATAKANA_LETTER_SA => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSa),
            HALFWIDTH_KATAKANA_LETTER_SI => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSi),
            HALFWIDTH_KATAKANA_LETTER_SU => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSu),
            HALFWIDTH_KATAKANA_LETTER_SE => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSe),
            HALFWIDTH_KATAKANA_LETTER_SO => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSo),
            HALFWIDTH_KATAKANA_LETTER_TA => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterTa),
            HALFWIDTH_KATAKANA_LETTER_TI => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterTi),
            HALFWIDTH_KATAKANA_LETTER_TU => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterTu),
            HALFWIDTH_KATAKANA_LETTER_TE => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterTe),
            HALFWIDTH_KATAKANA_LETTER_TO => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterTo),
            HALFWIDTH_KATAKANA_LETTER_NA => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterNa),
            HALFWIDTH_KATAKANA_LETTER_NI => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterNi),
            HALFWIDTH_KATAKANA_LETTER_NU => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterNu),
            HALFWIDTH_KATAKANA_LETTER_NE => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterNe),
            HALFWIDTH_KATAKANA_LETTER_NO => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterNo),
            HALFWIDTH_KATAKANA_LETTER_HA => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterHa),
            HALFWIDTH_KATAKANA_LETTER_HI => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterHi),
            HALFWIDTH_KATAKANA_LETTER_HU => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterHu),
            HALFWIDTH_KATAKANA_LETTER_HE => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterHe),
            HALFWIDTH_KATAKANA_LETTER_HO => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterHo),
            HALFWIDTH_KATAKANA_LETTER_MA => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterMa),
            HALFWIDTH_KATAKANA_LETTER_MI => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterMi),
            HALFWIDTH_KATAKANA_LETTER_MU => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterMu),
            HALFWIDTH_KATAKANA_LETTER_ME => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterMe),
            HALFWIDTH_KATAKANA_LETTER_MO => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterMo),
            HALFWIDTH_KATAKANA_LETTER_YA => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterYa),
            HALFWIDTH_KATAKANA_LETTER_YU => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterYu),
            HALFWIDTH_KATAKANA_LETTER_YO => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterYo),
            HALFWIDTH_KATAKANA_LETTER_RA => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterRa),
            HALFWIDTH_KATAKANA_LETTER_RI => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterRi),
            HALFWIDTH_KATAKANA_LETTER_RU => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterRu),
            HALFWIDTH_KATAKANA_LETTER_RE => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterRe),
            HALFWIDTH_KATAKANA_LETTER_RO => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterRo),
            HALFWIDTH_KATAKANA_LETTER_WA => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterWa),
            HALFWIDTH_KATAKANA_LETTER_N => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterN),
            HALFWIDTH_KATAKANA_VOICED_SOUND_MARK => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaVoicedSoundMark),
            HALFWIDTH_KATAKANA_SEMI_DASH_VOICED_SOUND_MARK => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaSemiDashVoicedSoundMark),
            HALFWIDTH_HANGUL_FILLER => Ok(HalfwidthandFullwidthForms::HalfwidthHangulFiller),
            HALFWIDTH_HANGUL_LETTER_KIYEOK => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterKiyeok),
            HALFWIDTH_HANGUL_LETTER_SSANGKIYEOK => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterSsangkiyeok),
            HALFWIDTH_HANGUL_LETTER_KIYEOK_DASH_SIOS => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterKiyeokDashSios),
            HALFWIDTH_HANGUL_LETTER_NIEUN => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterNieun),
            HALFWIDTH_HANGUL_LETTER_NIEUN_DASH_CIEUC => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterNieunDashCieuc),
            HALFWIDTH_HANGUL_LETTER_NIEUN_DASH_HIEUH => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterNieunDashHieuh),
            HALFWIDTH_HANGUL_LETTER_TIKEUT => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterTikeut),
            HALFWIDTH_HANGUL_LETTER_SSANGTIKEUT => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterSsangtikeut),
            HALFWIDTH_HANGUL_LETTER_RIEUL => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterRieul),
            HALFWIDTH_HANGUL_LETTER_RIEUL_DASH_KIYEOK => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterRieulDashKiyeok),
            HALFWIDTH_HANGUL_LETTER_RIEUL_DASH_MIEUM => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterRieulDashMieum),
            HALFWIDTH_HANGUL_LETTER_RIEUL_DASH_PIEUP => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterRieulDashPieup),
            HALFWIDTH_HANGUL_LETTER_RIEUL_DASH_SIOS => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterRieulDashSios),
            HALFWIDTH_HANGUL_LETTER_RIEUL_DASH_THIEUTH => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterRieulDashThieuth),
            HALFWIDTH_HANGUL_LETTER_RIEUL_DASH_PHIEUPH => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterRieulDashPhieuph),
            HALFWIDTH_HANGUL_LETTER_RIEUL_DASH_HIEUH => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterRieulDashHieuh),
            HALFWIDTH_HANGUL_LETTER_MIEUM => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterMieum),
            HALFWIDTH_HANGUL_LETTER_PIEUP => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterPieup),
            HALFWIDTH_HANGUL_LETTER_SSANGPIEUP => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterSsangpieup),
            HALFWIDTH_HANGUL_LETTER_PIEUP_DASH_SIOS => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterPieupDashSios),
            HALFWIDTH_HANGUL_LETTER_SIOS => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterSios),
            HALFWIDTH_HANGUL_LETTER_SSANGSIOS => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterSsangsios),
            HALFWIDTH_HANGUL_LETTER_IEUNG => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterIeung),
            HALFWIDTH_HANGUL_LETTER_CIEUC => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterCieuc),
            HALFWIDTH_HANGUL_LETTER_SSANGCIEUC => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterSsangcieuc),
            HALFWIDTH_HANGUL_LETTER_CHIEUCH => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterChieuch),
            HALFWIDTH_HANGUL_LETTER_KHIEUKH => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterKhieukh),
            HALFWIDTH_HANGUL_LETTER_THIEUTH => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterThieuth),
            HALFWIDTH_HANGUL_LETTER_PHIEUPH => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterPhieuph),
            HALFWIDTH_HANGUL_LETTER_HIEUH => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterHieuh),
            HALFWIDTH_HANGUL_LETTER_A => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterA),
            HALFWIDTH_HANGUL_LETTER_AE => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterAe),
            HALFWIDTH_HANGUL_LETTER_YA => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterYa),
            HALFWIDTH_HANGUL_LETTER_YAE => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterYae),
            HALFWIDTH_HANGUL_LETTER_EO => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterEo),
            HALFWIDTH_HANGUL_LETTER_E => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterE),
            HALFWIDTH_HANGUL_LETTER_YEO => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterYeo),
            HALFWIDTH_HANGUL_LETTER_YE => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterYe),
            HALFWIDTH_HANGUL_LETTER_O => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterO),
            HALFWIDTH_HANGUL_LETTER_WA => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterWa),
            HALFWIDTH_HANGUL_LETTER_WAE => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterWae),
            HALFWIDTH_HANGUL_LETTER_OE => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterOe),
            HALFWIDTH_HANGUL_LETTER_YO => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterYo),
            HALFWIDTH_HANGUL_LETTER_U => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterU),
            HALFWIDTH_HANGUL_LETTER_WEO => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterWeo),
            HALFWIDTH_HANGUL_LETTER_WE => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterWe),
            HALFWIDTH_HANGUL_LETTER_WI => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterWi),
            HALFWIDTH_HANGUL_LETTER_YU => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterYu),
            HALFWIDTH_HANGUL_LETTER_EU => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterEu),
            HALFWIDTH_HANGUL_LETTER_YI => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterYi),
            HALFWIDTH_HANGUL_LETTER_I => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterI),
            FULLWIDTH_CENT_SIGN => Ok(HalfwidthandFullwidthForms::FullwidthCentSign),
            FULLWIDTH_POUND_SIGN => Ok(HalfwidthandFullwidthForms::FullwidthPoundSign),
            FULLWIDTH_NOT_SIGN => Ok(HalfwidthandFullwidthForms::FullwidthNotSign),
            FULLWIDTH_MACRON => Ok(HalfwidthandFullwidthForms::FullwidthMacron),
            FULLWIDTH_BROKEN_BAR => Ok(HalfwidthandFullwidthForms::FullwidthBrokenBar),
            FULLWIDTH_YEN_SIGN => Ok(HalfwidthandFullwidthForms::FullwidthYenSign),
            FULLWIDTH_WON_SIGN => Ok(HalfwidthandFullwidthForms::FullwidthWonSign),
            HALFWIDTH_FORMS_LIGHT_VERTICAL => Ok(HalfwidthandFullwidthForms::HalfwidthFormsLightVertical),
            HALFWIDTH_LEFTWARDS_ARROW => Ok(HalfwidthandFullwidthForms::HalfwidthLeftwardsArrow),
            HALFWIDTH_UPWARDS_ARROW => Ok(HalfwidthandFullwidthForms::HalfwidthUpwardsArrow),
            HALFWIDTH_RIGHTWARDS_ARROW => Ok(HalfwidthandFullwidthForms::HalfwidthRightwardsArrow),
            HALFWIDTH_DOWNWARDS_ARROW => Ok(HalfwidthandFullwidthForms::HalfwidthDownwardsArrow),
            HALFWIDTH_BLACK_SQUARE => Ok(HalfwidthandFullwidthForms::HalfwidthBlackSquare),
            HALFWIDTH_WHITE_CIRCLE => Ok(HalfwidthandFullwidthForms::HalfwidthWhiteCircle),
            _ => Err(()),
        }
    }
}

impl Into<u32> for HalfwidthandFullwidthForms {
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

impl std::convert::TryFrom<u32> for HalfwidthandFullwidthForms {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for HalfwidthandFullwidthForms {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl HalfwidthandFullwidthForms {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        HalfwidthandFullwidthForms::FullwidthExclamationMark
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            HalfwidthandFullwidthForms::FullwidthExclamationMark => "fullwidth exclamation mark",
            HalfwidthandFullwidthForms::FullwidthQuotationMark => "fullwidth quotation mark",
            HalfwidthandFullwidthForms::FullwidthNumberSign => "fullwidth number sign",
            HalfwidthandFullwidthForms::FullwidthDollarSign => "fullwidth dollar sign",
            HalfwidthandFullwidthForms::FullwidthPercentSign => "fullwidth percent sign",
            HalfwidthandFullwidthForms::FullwidthAmpersand => "fullwidth ampersand",
            HalfwidthandFullwidthForms::FullwidthApostrophe => "fullwidth apostrophe",
            HalfwidthandFullwidthForms::FullwidthLeftParenthesis => "fullwidth left parenthesis",
            HalfwidthandFullwidthForms::FullwidthRightParenthesis => "fullwidth right parenthesis",
            HalfwidthandFullwidthForms::FullwidthAsterisk => "fullwidth asterisk",
            HalfwidthandFullwidthForms::FullwidthPlusSign => "fullwidth plus sign",
            HalfwidthandFullwidthForms::FullwidthComma => "fullwidth comma",
            HalfwidthandFullwidthForms::FullwidthHyphenDashMinus => "fullwidth hyphen-minus",
            HalfwidthandFullwidthForms::FullwidthFullStop => "fullwidth full stop",
            HalfwidthandFullwidthForms::FullwidthSolidus => "fullwidth solidus",
            HalfwidthandFullwidthForms::FullwidthDigitZero => "fullwidth digit zero",
            HalfwidthandFullwidthForms::FullwidthDigitOne => "fullwidth digit one",
            HalfwidthandFullwidthForms::FullwidthDigitTwo => "fullwidth digit two",
            HalfwidthandFullwidthForms::FullwidthDigitThree => "fullwidth digit three",
            HalfwidthandFullwidthForms::FullwidthDigitFour => "fullwidth digit four",
            HalfwidthandFullwidthForms::FullwidthDigitFive => "fullwidth digit five",
            HalfwidthandFullwidthForms::FullwidthDigitSix => "fullwidth digit six",
            HalfwidthandFullwidthForms::FullwidthDigitSeven => "fullwidth digit seven",
            HalfwidthandFullwidthForms::FullwidthDigitEight => "fullwidth digit eight",
            HalfwidthandFullwidthForms::FullwidthDigitNine => "fullwidth digit nine",
            HalfwidthandFullwidthForms::FullwidthColon => "fullwidth colon",
            HalfwidthandFullwidthForms::FullwidthSemicolon => "fullwidth semicolon",
            HalfwidthandFullwidthForms::FullwidthLessDashThanSign => "fullwidth less-than sign",
            HalfwidthandFullwidthForms::FullwidthEqualsSign => "fullwidth equals sign",
            HalfwidthandFullwidthForms::FullwidthGreaterDashThanSign => "fullwidth greater-than sign",
            HalfwidthandFullwidthForms::FullwidthQuestionMark => "fullwidth question mark",
            HalfwidthandFullwidthForms::FullwidthCommercialAt => "fullwidth commercial at",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterA => "fullwidth latin capital letter a",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterB => "fullwidth latin capital letter b",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterC => "fullwidth latin capital letter c",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterD => "fullwidth latin capital letter d",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterE => "fullwidth latin capital letter e",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterF => "fullwidth latin capital letter f",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterG => "fullwidth latin capital letter g",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterH => "fullwidth latin capital letter h",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterI => "fullwidth latin capital letter i",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterJ => "fullwidth latin capital letter j",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterK => "fullwidth latin capital letter k",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterL => "fullwidth latin capital letter l",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterM => "fullwidth latin capital letter m",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterN => "fullwidth latin capital letter n",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterO => "fullwidth latin capital letter o",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterP => "fullwidth latin capital letter p",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterQ => "fullwidth latin capital letter q",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterR => "fullwidth latin capital letter r",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterS => "fullwidth latin capital letter s",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterT => "fullwidth latin capital letter t",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterU => "fullwidth latin capital letter u",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterV => "fullwidth latin capital letter v",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterW => "fullwidth latin capital letter w",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterX => "fullwidth latin capital letter x",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterY => "fullwidth latin capital letter y",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterZ => "fullwidth latin capital letter z",
            HalfwidthandFullwidthForms::FullwidthLeftSquareBracket => "fullwidth left square bracket",
            HalfwidthandFullwidthForms::FullwidthReverseSolidus => "fullwidth reverse solidus",
            HalfwidthandFullwidthForms::FullwidthRightSquareBracket => "fullwidth right square bracket",
            HalfwidthandFullwidthForms::FullwidthCircumflexAccent => "fullwidth circumflex accent",
            HalfwidthandFullwidthForms::FullwidthLowLine => "fullwidth low line",
            HalfwidthandFullwidthForms::FullwidthGraveAccent => "fullwidth grave accent",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterA => "fullwidth latin small letter a",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterB => "fullwidth latin small letter b",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterC => "fullwidth latin small letter c",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterD => "fullwidth latin small letter d",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterE => "fullwidth latin small letter e",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterF => "fullwidth latin small letter f",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterG => "fullwidth latin small letter g",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterH => "fullwidth latin small letter h",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterI => "fullwidth latin small letter i",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterJ => "fullwidth latin small letter j",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterK => "fullwidth latin small letter k",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterL => "fullwidth latin small letter l",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterM => "fullwidth latin small letter m",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterN => "fullwidth latin small letter n",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterO => "fullwidth latin small letter o",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterP => "fullwidth latin small letter p",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterQ => "fullwidth latin small letter q",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterR => "fullwidth latin small letter r",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterS => "fullwidth latin small letter s",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterT => "fullwidth latin small letter t",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterU => "fullwidth latin small letter u",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterV => "fullwidth latin small letter v",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterW => "fullwidth latin small letter w",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterX => "fullwidth latin small letter x",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterY => "fullwidth latin small letter y",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterZ => "fullwidth latin small letter z",
            HalfwidthandFullwidthForms::FullwidthLeftCurlyBracket => "fullwidth left curly bracket",
            HalfwidthandFullwidthForms::FullwidthVerticalLine => "fullwidth vertical line",
            HalfwidthandFullwidthForms::FullwidthRightCurlyBracket => "fullwidth right curly bracket",
            HalfwidthandFullwidthForms::FullwidthTilde => "fullwidth tilde",
            HalfwidthandFullwidthForms::FullwidthLeftWhiteParenthesis => "fullwidth left white parenthesis",
            HalfwidthandFullwidthForms::FullwidthRightWhiteParenthesis => "fullwidth right white parenthesis",
            HalfwidthandFullwidthForms::HalfwidthIdeographicFullStop => "halfwidth ideographic full stop",
            HalfwidthandFullwidthForms::HalfwidthLeftCornerBracket => "halfwidth left corner bracket",
            HalfwidthandFullwidthForms::HalfwidthRightCornerBracket => "halfwidth right corner bracket",
            HalfwidthandFullwidthForms::HalfwidthIdeographicComma => "halfwidth ideographic comma",
            HalfwidthandFullwidthForms::HalfwidthKatakanaMiddleDot => "halfwidth katakana middle dot",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterWo => "halfwidth katakana letter wo",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallA => "halfwidth katakana letter small a",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallI => "halfwidth katakana letter small i",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallU => "halfwidth katakana letter small u",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallE => "halfwidth katakana letter small e",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallO => "halfwidth katakana letter small o",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallYa => "halfwidth katakana letter small ya",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallYu => "halfwidth katakana letter small yu",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallYo => "halfwidth katakana letter small yo",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallTu => "halfwidth katakana letter small tu",
            HalfwidthandFullwidthForms::HalfwidthKatakanaDashHiraganaProlongedSoundMark => "halfwidth katakana-hiragana prolonged sound mark",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterA => "halfwidth katakana letter a",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterI => "halfwidth katakana letter i",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterU => "halfwidth katakana letter u",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterE => "halfwidth katakana letter e",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterO => "halfwidth katakana letter o",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterKa => "halfwidth katakana letter ka",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterKi => "halfwidth katakana letter ki",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterKu => "halfwidth katakana letter ku",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterKe => "halfwidth katakana letter ke",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterKo => "halfwidth katakana letter ko",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSa => "halfwidth katakana letter sa",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSi => "halfwidth katakana letter si",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSu => "halfwidth katakana letter su",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSe => "halfwidth katakana letter se",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSo => "halfwidth katakana letter so",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterTa => "halfwidth katakana letter ta",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterTi => "halfwidth katakana letter ti",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterTu => "halfwidth katakana letter tu",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterTe => "halfwidth katakana letter te",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterTo => "halfwidth katakana letter to",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterNa => "halfwidth katakana letter na",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterNi => "halfwidth katakana letter ni",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterNu => "halfwidth katakana letter nu",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterNe => "halfwidth katakana letter ne",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterNo => "halfwidth katakana letter no",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterHa => "halfwidth katakana letter ha",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterHi => "halfwidth katakana letter hi",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterHu => "halfwidth katakana letter hu",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterHe => "halfwidth katakana letter he",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterHo => "halfwidth katakana letter ho",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterMa => "halfwidth katakana letter ma",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterMi => "halfwidth katakana letter mi",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterMu => "halfwidth katakana letter mu",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterMe => "halfwidth katakana letter me",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterMo => "halfwidth katakana letter mo",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterYa => "halfwidth katakana letter ya",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterYu => "halfwidth katakana letter yu",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterYo => "halfwidth katakana letter yo",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterRa => "halfwidth katakana letter ra",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterRi => "halfwidth katakana letter ri",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterRu => "halfwidth katakana letter ru",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterRe => "halfwidth katakana letter re",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterRo => "halfwidth katakana letter ro",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterWa => "halfwidth katakana letter wa",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterN => "halfwidth katakana letter n",
            HalfwidthandFullwidthForms::HalfwidthKatakanaVoicedSoundMark => "halfwidth katakana voiced sound mark",
            HalfwidthandFullwidthForms::HalfwidthKatakanaSemiDashVoicedSoundMark => "halfwidth katakana semi-voiced sound mark",
            HalfwidthandFullwidthForms::HalfwidthHangulFiller => "halfwidth hangul filler",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterKiyeok => "halfwidth hangul letter kiyeok",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterSsangkiyeok => "halfwidth hangul letter ssangkiyeok",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterKiyeokDashSios => "halfwidth hangul letter kiyeok-sios",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterNieun => "halfwidth hangul letter nieun",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterNieunDashCieuc => "halfwidth hangul letter nieun-cieuc",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterNieunDashHieuh => "halfwidth hangul letter nieun-hieuh",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterTikeut => "halfwidth hangul letter tikeut",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterSsangtikeut => "halfwidth hangul letter ssangtikeut",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterRieul => "halfwidth hangul letter rieul",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterRieulDashKiyeok => "halfwidth hangul letter rieul-kiyeok",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterRieulDashMieum => "halfwidth hangul letter rieul-mieum",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterRieulDashPieup => "halfwidth hangul letter rieul-pieup",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterRieulDashSios => "halfwidth hangul letter rieul-sios",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterRieulDashThieuth => "halfwidth hangul letter rieul-thieuth",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterRieulDashPhieuph => "halfwidth hangul letter rieul-phieuph",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterRieulDashHieuh => "halfwidth hangul letter rieul-hieuh",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterMieum => "halfwidth hangul letter mieum",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterPieup => "halfwidth hangul letter pieup",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterSsangpieup => "halfwidth hangul letter ssangpieup",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterPieupDashSios => "halfwidth hangul letter pieup-sios",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterSios => "halfwidth hangul letter sios",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterSsangsios => "halfwidth hangul letter ssangsios",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterIeung => "halfwidth hangul letter ieung",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterCieuc => "halfwidth hangul letter cieuc",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterSsangcieuc => "halfwidth hangul letter ssangcieuc",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterChieuch => "halfwidth hangul letter chieuch",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterKhieukh => "halfwidth hangul letter khieukh",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterThieuth => "halfwidth hangul letter thieuth",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterPhieuph => "halfwidth hangul letter phieuph",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterHieuh => "halfwidth hangul letter hieuh",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterA => "halfwidth hangul letter a",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterAe => "halfwidth hangul letter ae",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterYa => "halfwidth hangul letter ya",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterYae => "halfwidth hangul letter yae",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterEo => "halfwidth hangul letter eo",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterE => "halfwidth hangul letter e",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterYeo => "halfwidth hangul letter yeo",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterYe => "halfwidth hangul letter ye",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterO => "halfwidth hangul letter o",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterWa => "halfwidth hangul letter wa",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterWae => "halfwidth hangul letter wae",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterOe => "halfwidth hangul letter oe",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterYo => "halfwidth hangul letter yo",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterU => "halfwidth hangul letter u",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterWeo => "halfwidth hangul letter weo",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterWe => "halfwidth hangul letter we",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterWi => "halfwidth hangul letter wi",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterYu => "halfwidth hangul letter yu",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterEu => "halfwidth hangul letter eu",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterYi => "halfwidth hangul letter yi",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterI => "halfwidth hangul letter i",
            HalfwidthandFullwidthForms::FullwidthCentSign => "fullwidth cent sign",
            HalfwidthandFullwidthForms::FullwidthPoundSign => "fullwidth pound sign",
            HalfwidthandFullwidthForms::FullwidthNotSign => "fullwidth not sign",
            HalfwidthandFullwidthForms::FullwidthMacron => "fullwidth macron",
            HalfwidthandFullwidthForms::FullwidthBrokenBar => "fullwidth broken bar",
            HalfwidthandFullwidthForms::FullwidthYenSign => "fullwidth yen sign",
            HalfwidthandFullwidthForms::FullwidthWonSign => "fullwidth won sign",
            HalfwidthandFullwidthForms::HalfwidthFormsLightVertical => "halfwidth forms light vertical",
            HalfwidthandFullwidthForms::HalfwidthLeftwardsArrow => "halfwidth leftwards arrow",
            HalfwidthandFullwidthForms::HalfwidthUpwardsArrow => "halfwidth upwards arrow",
            HalfwidthandFullwidthForms::HalfwidthRightwardsArrow => "halfwidth rightwards arrow",
            HalfwidthandFullwidthForms::HalfwidthDownwardsArrow => "halfwidth downwards arrow",
            HalfwidthandFullwidthForms::HalfwidthBlackSquare => "halfwidth black square",
            HalfwidthandFullwidthForms::HalfwidthWhiteCircle => "halfwidth white circle",
        }
    }
}
