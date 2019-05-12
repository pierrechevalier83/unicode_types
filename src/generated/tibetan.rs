/// \u{f00} → \u{fff}
///
/// ༀ ༁ ༂ ༃ ༄ ༅ ༆ ༇ ༈ ༉ ༊ ་ ༌ ། ༎ ༏\
/// ༐ ༑ ༒ ༓ ༔ ༕ ༖ ༗ ༘ ༙ ༚ ༛ ༜ ༝ ༞ ༟\
/// ༠ ༡ ༢ ༣ ༤ ༥ ༦ ༧ ༨ ༩ ༪ ༫ ༬ ༭ ༮ ༯\
/// ༰ ༱ ༲ ༳ ༴ ༵ ༶ ༷ ༸ ༹ ༺ ༻ ༼ ༽ ༾ ༿\
/// ཀ ཁ ག གྷ ང ཅ ཆ ཇ ཉ ཊ ཋ ཌ ཌྷ ཎ ཏ ཐ\
/// ད དྷ ན པ ཕ བ བྷ མ ཙ ཚ ཛ ཛྷ ཝ ཞ ཟ འ\
/// ཡ ར ལ ཤ ཥ ས ཧ ཨ ཀྵ ཪ ཫ ཬ ཱ ི ཱི ུ\
/// ཱུ ྲྀ ཷ ླྀ ཹ ེ ཻ ོ ཽ ཾ ཿ ྀ ཱྀ ྂ ྃ ྄\
/// ྅ ྆ ྇ ྈ ྉ ྊ ྋ ྌ ྍ ྎ ྏ ྐ ྑ ྒ ྒྷ ྔ\
/// ྕ ྖ ྗ ྙ ྚ ྛ ྜ ྜྷ ྞ ྟ ྠ ྡ ྡྷ ྣ ྤ ྥ\
/// ྦ ྦྷ ྨ ྩ ྪ ྫ ྫྷ ྭ ྮ ྯ ྰ ྱ ྲ ླ ྴ ྵ\
/// ྶ ྷ ྸ ྐྵ ྺ ྻ ྼ ྾ ྿ ࿀ ࿁ ࿂ ࿃ ࿄ ࿅ ࿆\
/// ࿇ ࿈ ࿉ ࿊ ࿋ ࿌ ࿎ ࿏ ࿐ ࿑ ࿒ ࿓ ࿔ ࿕ ࿖ ࿗\
/// ࿘ ࿙ ࿚\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{f00}: 'ༀ'
    pub const SYLLABLE_OM: char = 'ༀ';
    /// \u{f01}: '༁'
    pub const MARK_GTER_YIG_MGO_TRUNCATED_A: char = '༁';
    /// \u{f02}: '༂'
    pub const MARK_GTER_YIG_MGO__DASH_UM_RNAM_BCAD_MA: char = '༂';
    /// \u{f03}: '༃'
    pub const MARK_GTER_YIG_MGO__DASH_UM_GTER_TSHEG_MA: char = '༃';
    /// \u{f04}: '༄'
    pub const MARK_INITIAL_YIG_MGO_MDUN_MA: char = '༄';
    /// \u{f05}: '༅'
    pub const MARK_CLOSING_YIG_MGO_SGAB_MA: char = '༅';
    /// \u{f06}: '༆'
    pub const MARK_CARET_YIG_MGO_PHUR_SHAD_MA: char = '༆';
    /// \u{f07}: '༇'
    pub const MARK_YIG_MGO_TSHEG_SHAD_MA: char = '༇';
    /// \u{f08}: '༈'
    pub const MARK_SBRUL_SHAD: char = '༈';
    /// \u{f09}: '༉'
    pub const MARK_BSKUR_YIG_MGO: char = '༉';
    /// \u{f0a}: '༊'
    pub const MARK_BKA_DASH__SHOG_YIG_MGO: char = '༊';
    /// \u{f0b}: '་'
    pub const MARK_INTERSYLLABIC_TSHEG: char = '་';
    /// \u{f0c}: '༌'
    pub const MARK_DELIMITER_TSHEG_BSTAR: char = '༌';
    /// \u{f0d}: '།'
    pub const MARK_SHAD: char = '།';
    /// \u{f0e}: '༎'
    pub const MARK_NYIS_SHAD: char = '༎';
    /// \u{f0f}: '༏'
    pub const MARK_TSHEG_SHAD: char = '༏';
    /// \u{f10}: '༐'
    pub const MARK_NYIS_TSHEG_SHAD: char = '༐';
    /// \u{f11}: '༑'
    pub const MARK_RIN_CHEN_SPUNGS_SHAD: char = '༑';
    /// \u{f12}: '༒'
    pub const MARK_RGYA_GRAM_SHAD: char = '༒';
    /// \u{f13}: '༓'
    pub const MARK_CARET__DASH_DZUD_RTAGS_ME_LONG_CAN: char = '༓';
    /// \u{f14}: '༔'
    pub const MARK_GTER_TSHEG: char = '༔';
    /// \u{f15}: '༕'
    pub const LOGOTYPE_SIGN_CHAD_RTAGS: char = '༕';
    /// \u{f16}: '༖'
    pub const LOGOTYPE_SIGN_LHAG_RTAGS: char = '༖';
    /// \u{f17}: '༗'
    pub const ASTROLOGICAL_SIGN_SGRA_GCAN__DASH_CHAR_RTAGS: char = '༗';
    /// \u{f18}: '༘'
    pub const ASTROLOGICAL_SIGN__DASH_KHYUD_PA: char = '༘';
    /// \u{f19}: '༙'
    pub const ASTROLOGICAL_SIGN_SDONG_TSHUGS: char = '༙';
    /// \u{f1a}: '༚'
    pub const SIGN_RDEL_DKAR_GCIG: char = '༚';
    /// \u{f1b}: '༛'
    pub const SIGN_RDEL_DKAR_GNYIS: char = '༛';
    /// \u{f1c}: '༜'
    pub const SIGN_RDEL_DKAR_GSUM: char = '༜';
    /// \u{f1d}: '༝'
    pub const SIGN_RDEL_NAG_GCIG: char = '༝';
    /// \u{f1e}: '༞'
    pub const SIGN_RDEL_NAG_GNYIS: char = '༞';
    /// \u{f1f}: '༟'
    pub const SIGN_RDEL_DKAR_RDEL_NAG: char = '༟';
    /// \u{f20}: '༠'
    pub const DIGIT_ZERO: char = '༠';
    /// \u{f21}: '༡'
    pub const DIGIT_ONE: char = '༡';
    /// \u{f22}: '༢'
    pub const DIGIT_TWO: char = '༢';
    /// \u{f23}: '༣'
    pub const DIGIT_THREE: char = '༣';
    /// \u{f24}: '༤'
    pub const DIGIT_FOUR: char = '༤';
    /// \u{f25}: '༥'
    pub const DIGIT_FIVE: char = '༥';
    /// \u{f26}: '༦'
    pub const DIGIT_SIX: char = '༦';
    /// \u{f27}: '༧'
    pub const DIGIT_SEVEN: char = '༧';
    /// \u{f28}: '༨'
    pub const DIGIT_EIGHT: char = '༨';
    /// \u{f29}: '༩'
    pub const DIGIT_NINE: char = '༩';
    /// \u{f2a}: '༪'
    pub const DIGIT_HALF_ONE: char = '༪';
    /// \u{f2b}: '༫'
    pub const DIGIT_HALF_TWO: char = '༫';
    /// \u{f2c}: '༬'
    pub const DIGIT_HALF_THREE: char = '༬';
    /// \u{f2d}: '༭'
    pub const DIGIT_HALF_FOUR: char = '༭';
    /// \u{f2e}: '༮'
    pub const DIGIT_HALF_FIVE: char = '༮';
    /// \u{f2f}: '༯'
    pub const DIGIT_HALF_SIX: char = '༯';
    /// \u{f30}: '༰'
    pub const DIGIT_HALF_SEVEN: char = '༰';
    /// \u{f31}: '༱'
    pub const DIGIT_HALF_EIGHT: char = '༱';
    /// \u{f32}: '༲'
    pub const DIGIT_HALF_NINE: char = '༲';
    /// \u{f33}: '༳'
    pub const DIGIT_HALF_ZERO: char = '༳';
    /// \u{f34}: '༴'
    pub const MARK_BSDUS_RTAGS: char = '༴';
    /// \u{f35}: '༵'
    pub const MARK_NGAS_BZUNG_NYI_ZLA: char = '༵';
    /// \u{f36}: '༶'
    pub const MARK_CARET__DASH_DZUD_RTAGS_BZHI_MIG_CAN: char = '༶';
    /// \u{f37}: '༷'
    pub const MARK_NGAS_BZUNG_SGOR_RTAGS: char = '༷';
    /// \u{f38}: '༸'
    pub const MARK_CHE_MGO: char = '༸';
    /// \u{f39}: '༹'
    pub const MARK_TSA__DASH_PHRU: char = '༹';
    /// \u{f3a}: '༺'
    pub const MARK_GUG_RTAGS_GYON: char = '༺';
    /// \u{f3b}: '༻'
    pub const MARK_GUG_RTAGS_GYAS: char = '༻';
    /// \u{f3c}: '༼'
    pub const MARK_ANG_KHANG_GYON: char = '༼';
    /// \u{f3d}: '༽'
    pub const MARK_ANG_KHANG_GYAS: char = '༽';
    /// \u{f3e}: '༾'
    pub const SIGN_YAR_TSHES: char = '༾';
    /// \u{f3f}: '༿'
    pub const SIGN_MAR_TSHES: char = '༿';
    /// \u{f40}: 'ཀ'
    pub const LETTER_KA: char = 'ཀ';
    /// \u{f41}: 'ཁ'
    pub const LETTER_KHA: char = 'ཁ';
    /// \u{f42}: 'ག'
    pub const LETTER_GA: char = 'ག';
    /// \u{f43}: 'གྷ'
    pub const LETTER_GHA: char = 'གྷ';
    /// \u{f44}: 'ང'
    pub const LETTER_NGA: char = 'ང';
    /// \u{f45}: 'ཅ'
    pub const LETTER_CA: char = 'ཅ';
    /// \u{f46}: 'ཆ'
    pub const LETTER_CHA: char = 'ཆ';
    /// \u{f47}: 'ཇ'
    pub const LETTER_JA: char = 'ཇ';
    /// \u{f49}: 'ཉ'
    pub const LETTER_NYA: char = 'ཉ';
    /// \u{f4a}: 'ཊ'
    pub const LETTER_TTA: char = 'ཊ';
    /// \u{f4b}: 'ཋ'
    pub const LETTER_TTHA: char = 'ཋ';
    /// \u{f4c}: 'ཌ'
    pub const LETTER_DDA: char = 'ཌ';
    /// \u{f4d}: 'ཌྷ'
    pub const LETTER_DDHA: char = 'ཌྷ';
    /// \u{f4e}: 'ཎ'
    pub const LETTER_NNA: char = 'ཎ';
    /// \u{f4f}: 'ཏ'
    pub const LETTER_TA: char = 'ཏ';
    /// \u{f50}: 'ཐ'
    pub const LETTER_THA: char = 'ཐ';
    /// \u{f51}: 'ད'
    pub const LETTER_DA: char = 'ད';
    /// \u{f52}: 'དྷ'
    pub const LETTER_DHA: char = 'དྷ';
    /// \u{f53}: 'ན'
    pub const LETTER_NA: char = 'ན';
    /// \u{f54}: 'པ'
    pub const LETTER_PA: char = 'པ';
    /// \u{f55}: 'ཕ'
    pub const LETTER_PHA: char = 'ཕ';
    /// \u{f56}: 'བ'
    pub const LETTER_BA: char = 'བ';
    /// \u{f57}: 'བྷ'
    pub const LETTER_BHA: char = 'བྷ';
    /// \u{f58}: 'མ'
    pub const LETTER_MA: char = 'མ';
    /// \u{f59}: 'ཙ'
    pub const LETTER_TSA: char = 'ཙ';
    /// \u{f5a}: 'ཚ'
    pub const LETTER_TSHA: char = 'ཚ';
    /// \u{f5b}: 'ཛ'
    pub const LETTER_DZA: char = 'ཛ';
    /// \u{f5c}: 'ཛྷ'
    pub const LETTER_DZHA: char = 'ཛྷ';
    /// \u{f5d}: 'ཝ'
    pub const LETTER_WA: char = 'ཝ';
    /// \u{f5e}: 'ཞ'
    pub const LETTER_ZHA: char = 'ཞ';
    /// \u{f5f}: 'ཟ'
    pub const LETTER_ZA: char = 'ཟ';
    /// \u{f60}: 'འ'
    pub const LETTER__DASH_A: char = 'འ';
    /// \u{f61}: 'ཡ'
    pub const LETTER_YA: char = 'ཡ';
    /// \u{f62}: 'ར'
    pub const LETTER_RA: char = 'ར';
    /// \u{f63}: 'ལ'
    pub const LETTER_LA: char = 'ལ';
    /// \u{f64}: 'ཤ'
    pub const LETTER_SHA: char = 'ཤ';
    /// \u{f65}: 'ཥ'
    pub const LETTER_SSA: char = 'ཥ';
    /// \u{f66}: 'ས'
    pub const LETTER_SA: char = 'ས';
    /// \u{f67}: 'ཧ'
    pub const LETTER_HA: char = 'ཧ';
    /// \u{f68}: 'ཨ'
    pub const LETTER_A: char = 'ཨ';
    /// \u{f69}: 'ཀྵ'
    pub const LETTER_KSSA: char = 'ཀྵ';
    /// \u{f6a}: 'ཪ'
    pub const LETTER_FIXED_DASH_FORM_RA: char = 'ཪ';
    /// \u{f6b}: 'ཫ'
    pub const LETTER_KKA: char = 'ཫ';
    /// \u{f6c}: 'ཬ'
    pub const LETTER_RRA: char = 'ཬ';
    /// \u{f71}: 'ཱ'
    pub const VOWEL_SIGN_AA: char = 'ཱ';
    /// \u{f72}: 'ི'
    pub const VOWEL_SIGN_I: char = 'ི';
    /// \u{f73}: 'ཱི'
    pub const VOWEL_SIGN_II: char = 'ཱི';
    /// \u{f74}: 'ུ'
    pub const VOWEL_SIGN_U: char = 'ུ';
    /// \u{f75}: 'ཱུ'
    pub const VOWEL_SIGN_UU: char = 'ཱུ';
    /// \u{f76}: 'ྲྀ'
    pub const VOWEL_SIGN_VOCALIC_R: char = 'ྲྀ';
    /// \u{f77}: 'ཷ'
    pub const VOWEL_SIGN_VOCALIC_RR: char = 'ཷ';
    /// \u{f78}: 'ླྀ'
    pub const VOWEL_SIGN_VOCALIC_L: char = 'ླྀ';
    /// \u{f79}: 'ཹ'
    pub const VOWEL_SIGN_VOCALIC_LL: char = 'ཹ';
    /// \u{f7a}: 'ེ'
    pub const VOWEL_SIGN_E: char = 'ེ';
    /// \u{f7b}: 'ཻ'
    pub const VOWEL_SIGN_EE: char = 'ཻ';
    /// \u{f7c}: 'ོ'
    pub const VOWEL_SIGN_O: char = 'ོ';
    /// \u{f7d}: 'ཽ'
    pub const VOWEL_SIGN_OO: char = 'ཽ';
    /// \u{f7e}: 'ཾ'
    pub const SIGN_RJES_SU_NGA_RO: char = 'ཾ';
    /// \u{f7f}: 'ཿ'
    pub const SIGN_RNAM_BCAD: char = 'ཿ';
    /// \u{f80}: 'ྀ'
    pub const VOWEL_SIGN_REVERSED_I: char = 'ྀ';
    /// \u{f81}: 'ཱྀ'
    pub const VOWEL_SIGN_REVERSED_II: char = 'ཱྀ';
    /// \u{f82}: 'ྂ'
    pub const SIGN_NYI_ZLA_NAA_DA: char = 'ྂ';
    /// \u{f83}: 'ྃ'
    pub const SIGN_SNA_LDAN: char = 'ྃ';
    /// \u{f84}: '྄'
    pub const MARK_HALANTA: char = '྄';
    /// \u{f85}: '྅'
    pub const MARK_PALUTA: char = '྅';
    /// \u{f86}: '྆'
    pub const SIGN_LCI_RTAGS: char = '྆';
    /// \u{f87}: '྇'
    pub const SIGN_YANG_RTAGS: char = '྇';
    /// \u{f88}: 'ྈ'
    pub const SIGN_LCE_TSA_CAN: char = 'ྈ';
    /// \u{f89}: 'ྉ'
    pub const SIGN_MCHU_CAN: char = 'ྉ';
    /// \u{f8a}: 'ྊ'
    pub const SIGN_GRU_CAN_RGYINGS: char = 'ྊ';
    /// \u{f8b}: 'ྋ'
    pub const SIGN_GRU_MED_RGYINGS: char = 'ྋ';
    /// \u{f8c}: 'ྌ'
    pub const SIGN_INVERTED_MCHU_CAN: char = 'ྌ';
    /// \u{f8d}: 'ྍ'
    pub const SUBJOINED_SIGN_LCE_TSA_CAN: char = 'ྍ';
    /// \u{f8e}: 'ྎ'
    pub const SUBJOINED_SIGN_MCHU_CAN: char = 'ྎ';
    /// \u{f8f}: 'ྏ'
    pub const SUBJOINED_SIGN_INVERTED_MCHU_CAN: char = 'ྏ';
    /// \u{f90}: 'ྐ'
    pub const SUBJOINED_LETTER_KA: char = 'ྐ';
    /// \u{f91}: 'ྑ'
    pub const SUBJOINED_LETTER_KHA: char = 'ྑ';
    /// \u{f92}: 'ྒ'
    pub const SUBJOINED_LETTER_GA: char = 'ྒ';
    /// \u{f93}: 'ྒྷ'
    pub const SUBJOINED_LETTER_GHA: char = 'ྒྷ';
    /// \u{f94}: 'ྔ'
    pub const SUBJOINED_LETTER_NGA: char = 'ྔ';
    /// \u{f95}: 'ྕ'
    pub const SUBJOINED_LETTER_CA: char = 'ྕ';
    /// \u{f96}: 'ྖ'
    pub const SUBJOINED_LETTER_CHA: char = 'ྖ';
    /// \u{f97}: 'ྗ'
    pub const SUBJOINED_LETTER_JA: char = 'ྗ';
    /// \u{f99}: 'ྙ'
    pub const SUBJOINED_LETTER_NYA: char = 'ྙ';
    /// \u{f9a}: 'ྚ'
    pub const SUBJOINED_LETTER_TTA: char = 'ྚ';
    /// \u{f9b}: 'ྛ'
    pub const SUBJOINED_LETTER_TTHA: char = 'ྛ';
    /// \u{f9c}: 'ྜ'
    pub const SUBJOINED_LETTER_DDA: char = 'ྜ';
    /// \u{f9d}: 'ྜྷ'
    pub const SUBJOINED_LETTER_DDHA: char = 'ྜྷ';
    /// \u{f9e}: 'ྞ'
    pub const SUBJOINED_LETTER_NNA: char = 'ྞ';
    /// \u{f9f}: 'ྟ'
    pub const SUBJOINED_LETTER_TA: char = 'ྟ';
    /// \u{fa0}: 'ྠ'
    pub const SUBJOINED_LETTER_THA: char = 'ྠ';
    /// \u{fa1}: 'ྡ'
    pub const SUBJOINED_LETTER_DA: char = 'ྡ';
    /// \u{fa2}: 'ྡྷ'
    pub const SUBJOINED_LETTER_DHA: char = 'ྡྷ';
    /// \u{fa3}: 'ྣ'
    pub const SUBJOINED_LETTER_NA: char = 'ྣ';
    /// \u{fa4}: 'ྤ'
    pub const SUBJOINED_LETTER_PA: char = 'ྤ';
    /// \u{fa5}: 'ྥ'
    pub const SUBJOINED_LETTER_PHA: char = 'ྥ';
    /// \u{fa6}: 'ྦ'
    pub const SUBJOINED_LETTER_BA: char = 'ྦ';
    /// \u{fa7}: 'ྦྷ'
    pub const SUBJOINED_LETTER_BHA: char = 'ྦྷ';
    /// \u{fa8}: 'ྨ'
    pub const SUBJOINED_LETTER_MA: char = 'ྨ';
    /// \u{fa9}: 'ྩ'
    pub const SUBJOINED_LETTER_TSA: char = 'ྩ';
    /// \u{faa}: 'ྪ'
    pub const SUBJOINED_LETTER_TSHA: char = 'ྪ';
    /// \u{fab}: 'ྫ'
    pub const SUBJOINED_LETTER_DZA: char = 'ྫ';
    /// \u{fac}: 'ྫྷ'
    pub const SUBJOINED_LETTER_DZHA: char = 'ྫྷ';
    /// \u{fad}: 'ྭ'
    pub const SUBJOINED_LETTER_WA: char = 'ྭ';
    /// \u{fae}: 'ྮ'
    pub const SUBJOINED_LETTER_ZHA: char = 'ྮ';
    /// \u{faf}: 'ྯ'
    pub const SUBJOINED_LETTER_ZA: char = 'ྯ';
    /// \u{fb0}: 'ྰ'
    pub const SUBJOINED_LETTER__DASH_A: char = 'ྰ';
    /// \u{fb1}: 'ྱ'
    pub const SUBJOINED_LETTER_YA: char = 'ྱ';
    /// \u{fb2}: 'ྲ'
    pub const SUBJOINED_LETTER_RA: char = 'ྲ';
    /// \u{fb3}: 'ླ'
    pub const SUBJOINED_LETTER_LA: char = 'ླ';
    /// \u{fb4}: 'ྴ'
    pub const SUBJOINED_LETTER_SHA: char = 'ྴ';
    /// \u{fb5}: 'ྵ'
    pub const SUBJOINED_LETTER_SSA: char = 'ྵ';
    /// \u{fb6}: 'ྶ'
    pub const SUBJOINED_LETTER_SA: char = 'ྶ';
    /// \u{fb7}: 'ྷ'
    pub const SUBJOINED_LETTER_HA: char = 'ྷ';
    /// \u{fb8}: 'ྸ'
    pub const SUBJOINED_LETTER_A: char = 'ྸ';
    /// \u{fb9}: 'ྐྵ'
    pub const SUBJOINED_LETTER_KSSA: char = 'ྐྵ';
    /// \u{fba}: 'ྺ'
    pub const SUBJOINED_LETTER_FIXED_DASH_FORM_WA: char = 'ྺ';
    /// \u{fbb}: 'ྻ'
    pub const SUBJOINED_LETTER_FIXED_DASH_FORM_YA: char = 'ྻ';
    /// \u{fbc}: 'ྼ'
    pub const SUBJOINED_LETTER_FIXED_DASH_FORM_RA: char = 'ྼ';
    /// \u{fbe}: '྾'
    pub const KU_RU_KHA: char = '྾';
    /// \u{fbf}: '྿'
    pub const KU_RU_KHA_BZHI_MIG_CAN: char = '྿';
    /// \u{fc0}: '࿀'
    pub const CANTILLATION_SIGN_HEAVY_BEAT: char = '࿀';
    /// \u{fc1}: '࿁'
    pub const CANTILLATION_SIGN_LIGHT_BEAT: char = '࿁';
    /// \u{fc2}: '࿂'
    pub const CANTILLATION_SIGN_CANG_TE_DASH_U: char = '࿂';
    /// \u{fc3}: '࿃'
    pub const CANTILLATION_SIGN_SBUB__DASH_CHAL: char = '࿃';
    /// \u{fc4}: '࿄'
    pub const SYMBOL_DRIL_BU: char = '࿄';
    /// \u{fc5}: '࿅'
    pub const SYMBOL_RDO_RJE: char = '࿅';
    /// \u{fc6}: '࿆'
    pub const SYMBOL_PADMA_GDAN: char = '࿆';
    /// \u{fc7}: '࿇'
    pub const SYMBOL_RDO_RJE_RGYA_GRAM: char = '࿇';
    /// \u{fc8}: '࿈'
    pub const SYMBOL_PHUR_PA: char = '࿈';
    /// \u{fc9}: '࿉'
    pub const SYMBOL_NOR_BU: char = '࿉';
    /// \u{fca}: '࿊'
    pub const SYMBOL_NOR_BU_NYIS__DASH_KHYIL: char = '࿊';
    /// \u{fcb}: '࿋'
    pub const SYMBOL_NOR_BU_GSUM__DASH_KHYIL: char = '࿋';
    /// \u{fcc}: '࿌'
    pub const SYMBOL_NOR_BU_BZHI__DASH_KHYIL: char = '࿌';
    /// \u{fce}: '࿎'
    pub const SIGN_RDEL_NAG_RDEL_DKAR: char = '࿎';
    /// \u{fcf}: '࿏'
    pub const SIGN_RDEL_NAG_GSUM: char = '࿏';
    /// \u{fd0}: '࿐'
    pub const MARK_BSKA_DASH__SHOG_GI_MGO_RGYAN: char = '࿐';
    /// \u{fd1}: '࿑'
    pub const MARK_MNYAM_YIG_GI_MGO_RGYAN: char = '࿑';
    /// \u{fd2}: '࿒'
    pub const MARK_NYIS_TSHEG: char = '࿒';
    /// \u{fd3}: '࿓'
    pub const MARK_INITIAL_BRDA_RNYING_YIG_MGO_MDUN_MA: char = '࿓';
    /// \u{fd4}: '࿔'
    pub const MARK_CLOSING_BRDA_RNYING_YIG_MGO_SGAB_MA: char = '࿔';
    /// \u{fd5}: '࿕'
    pub const RIGHT_DASH_FACING_SVASTI_SIGN: char = '࿕';
    /// \u{fd6}: '࿖'
    pub const LEFT_DASH_FACING_SVASTI_SIGN: char = '࿖';
    /// \u{fd7}: '࿗'
    pub const RIGHT_DASH_FACING_SVASTI_SIGN_WITH_DOTS: char = '࿗';
    /// \u{fd8}: '࿘'
    pub const LEFT_DASH_FACING_SVASTI_SIGN_WITH_DOTS: char = '࿘';
    /// \u{fd9}: '࿙'
    pub const MARK_LEADING_MCHAN_RTAGS: char = '࿙';
    /// \u{fda}: '࿚'
    pub const MARK_TRAILING_MCHAN_RTAGS: char = '࿚';
}

/// An enum to represent all characters in the Tibetan block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Tibetan {
    /// \u{f00}: 'ༀ'
    SyllableOm,
    /// \u{f01}: '༁'
    MarkGterYigMgoTruncatedA,
    /// \u{f02}: '༂'
    MarkGterYigMgoDashUmRnamBcadMa,
    /// \u{f03}: '༃'
    MarkGterYigMgoDashUmGterTshegMa,
    /// \u{f04}: '༄'
    MarkInitialYigMgoMdunMa,
    /// \u{f05}: '༅'
    MarkClosingYigMgoSgabMa,
    /// \u{f06}: '༆'
    MarkCaretYigMgoPhurShadMa,
    /// \u{f07}: '༇'
    MarkYigMgoTshegShadMa,
    /// \u{f08}: '༈'
    MarkSbrulShad,
    /// \u{f09}: '༉'
    MarkBskurYigMgo,
    /// \u{f0a}: '༊'
    MarkBkaDashShogYigMgo,
    /// \u{f0b}: '་'
    MarkIntersyllabicTsheg,
    /// \u{f0c}: '༌'
    MarkDelimiterTshegBstar,
    /// \u{f0d}: '།'
    MarkShad,
    /// \u{f0e}: '༎'
    MarkNyisShad,
    /// \u{f0f}: '༏'
    MarkTshegShad,
    /// \u{f10}: '༐'
    MarkNyisTshegShad,
    /// \u{f11}: '༑'
    MarkRinChenSpungsShad,
    /// \u{f12}: '༒'
    MarkRgyaGramShad,
    /// \u{f13}: '༓'
    MarkCaretDashDzudRtagsMeLongCan,
    /// \u{f14}: '༔'
    MarkGterTsheg,
    /// \u{f15}: '༕'
    LogotypeSignChadRtags,
    /// \u{f16}: '༖'
    LogotypeSignLhagRtags,
    /// \u{f17}: '༗'
    AstrologicalSignSgraGcanDashCharRtags,
    /// \u{f18}: '༘'
    AstrologicalSignDashKhyudPa,
    /// \u{f19}: '༙'
    AstrologicalSignSdongTshugs,
    /// \u{f1a}: '༚'
    SignRdelDkarGcig,
    /// \u{f1b}: '༛'
    SignRdelDkarGnyis,
    /// \u{f1c}: '༜'
    SignRdelDkarGsum,
    /// \u{f1d}: '༝'
    SignRdelNagGcig,
    /// \u{f1e}: '༞'
    SignRdelNagGnyis,
    /// \u{f1f}: '༟'
    SignRdelDkarRdelNag,
    /// \u{f20}: '༠'
    DigitZero,
    /// \u{f21}: '༡'
    DigitOne,
    /// \u{f22}: '༢'
    DigitTwo,
    /// \u{f23}: '༣'
    DigitThree,
    /// \u{f24}: '༤'
    DigitFour,
    /// \u{f25}: '༥'
    DigitFive,
    /// \u{f26}: '༦'
    DigitSix,
    /// \u{f27}: '༧'
    DigitSeven,
    /// \u{f28}: '༨'
    DigitEight,
    /// \u{f29}: '༩'
    DigitNine,
    /// \u{f2a}: '༪'
    DigitHalfOne,
    /// \u{f2b}: '༫'
    DigitHalfTwo,
    /// \u{f2c}: '༬'
    DigitHalfThree,
    /// \u{f2d}: '༭'
    DigitHalfFour,
    /// \u{f2e}: '༮'
    DigitHalfFive,
    /// \u{f2f}: '༯'
    DigitHalfSix,
    /// \u{f30}: '༰'
    DigitHalfSeven,
    /// \u{f31}: '༱'
    DigitHalfEight,
    /// \u{f32}: '༲'
    DigitHalfNine,
    /// \u{f33}: '༳'
    DigitHalfZero,
    /// \u{f34}: '༴'
    MarkBsdusRtags,
    /// \u{f35}: '༵'
    MarkNgasBzungNyiZla,
    /// \u{f36}: '༶'
    MarkCaretDashDzudRtagsBzhiMigCan,
    /// \u{f37}: '༷'
    MarkNgasBzungSgorRtags,
    /// \u{f38}: '༸'
    MarkCheMgo,
    /// \u{f39}: '༹'
    MarkTsaDashPhru,
    /// \u{f3a}: '༺'
    MarkGugRtagsGyon,
    /// \u{f3b}: '༻'
    MarkGugRtagsGyas,
    /// \u{f3c}: '༼'
    MarkAngKhangGyon,
    /// \u{f3d}: '༽'
    MarkAngKhangGyas,
    /// \u{f3e}: '༾'
    SignYarTshes,
    /// \u{f3f}: '༿'
    SignMarTshes,
    /// \u{f40}: 'ཀ'
    LetterKa,
    /// \u{f41}: 'ཁ'
    LetterKha,
    /// \u{f42}: 'ག'
    LetterGa,
    /// \u{f43}: 'གྷ'
    LetterGha,
    /// \u{f44}: 'ང'
    LetterNga,
    /// \u{f45}: 'ཅ'
    LetterCa,
    /// \u{f46}: 'ཆ'
    LetterCha,
    /// \u{f47}: 'ཇ'
    LetterJa,
    /// \u{f49}: 'ཉ'
    LetterNya,
    /// \u{f4a}: 'ཊ'
    LetterTta,
    /// \u{f4b}: 'ཋ'
    LetterTtha,
    /// \u{f4c}: 'ཌ'
    LetterDda,
    /// \u{f4d}: 'ཌྷ'
    LetterDdha,
    /// \u{f4e}: 'ཎ'
    LetterNna,
    /// \u{f4f}: 'ཏ'
    LetterTa,
    /// \u{f50}: 'ཐ'
    LetterTha,
    /// \u{f51}: 'ད'
    LetterDa,
    /// \u{f52}: 'དྷ'
    LetterDha,
    /// \u{f53}: 'ན'
    LetterNa,
    /// \u{f54}: 'པ'
    LetterPa,
    /// \u{f55}: 'ཕ'
    LetterPha,
    /// \u{f56}: 'བ'
    LetterBa,
    /// \u{f57}: 'བྷ'
    LetterBha,
    /// \u{f58}: 'མ'
    LetterMa,
    /// \u{f59}: 'ཙ'
    LetterTsa,
    /// \u{f5a}: 'ཚ'
    LetterTsha,
    /// \u{f5b}: 'ཛ'
    LetterDza,
    /// \u{f5c}: 'ཛྷ'
    LetterDzha,
    /// \u{f5d}: 'ཝ'
    LetterWa,
    /// \u{f5e}: 'ཞ'
    LetterZha,
    /// \u{f5f}: 'ཟ'
    LetterZa,
    /// \u{f60}: 'འ'
    LetterDashA,
    /// \u{f61}: 'ཡ'
    LetterYa,
    /// \u{f62}: 'ར'
    LetterRa,
    /// \u{f63}: 'ལ'
    LetterLa,
    /// \u{f64}: 'ཤ'
    LetterSha,
    /// \u{f65}: 'ཥ'
    LetterSsa,
    /// \u{f66}: 'ས'
    LetterSa,
    /// \u{f67}: 'ཧ'
    LetterHa,
    /// \u{f68}: 'ཨ'
    LetterA,
    /// \u{f69}: 'ཀྵ'
    LetterKssa,
    /// \u{f6a}: 'ཪ'
    LetterFixedDashFormRa,
    /// \u{f6b}: 'ཫ'
    LetterKka,
    /// \u{f6c}: 'ཬ'
    LetterRra,
    /// \u{f71}: 'ཱ'
    VowelSignAa,
    /// \u{f72}: 'ི'
    VowelSignI,
    /// \u{f73}: 'ཱི'
    VowelSignIi,
    /// \u{f74}: 'ུ'
    VowelSignU,
    /// \u{f75}: 'ཱུ'
    VowelSignUu,
    /// \u{f76}: 'ྲྀ'
    VowelSignVocalicR,
    /// \u{f77}: 'ཷ'
    VowelSignVocalicRr,
    /// \u{f78}: 'ླྀ'
    VowelSignVocalicL,
    /// \u{f79}: 'ཹ'
    VowelSignVocalicLl,
    /// \u{f7a}: 'ེ'
    VowelSignE,
    /// \u{f7b}: 'ཻ'
    VowelSignEe,
    /// \u{f7c}: 'ོ'
    VowelSignO,
    /// \u{f7d}: 'ཽ'
    VowelSignOo,
    /// \u{f7e}: 'ཾ'
    SignRjesSuNgaRo,
    /// \u{f7f}: 'ཿ'
    SignRnamBcad,
    /// \u{f80}: 'ྀ'
    VowelSignReversedI,
    /// \u{f81}: 'ཱྀ'
    VowelSignReversedIi,
    /// \u{f82}: 'ྂ'
    SignNyiZlaNaaDa,
    /// \u{f83}: 'ྃ'
    SignSnaLdan,
    /// \u{f84}: '྄'
    MarkHalanta,
    /// \u{f85}: '྅'
    MarkPaluta,
    /// \u{f86}: '྆'
    SignLciRtags,
    /// \u{f87}: '྇'
    SignYangRtags,
    /// \u{f88}: 'ྈ'
    SignLceTsaCan,
    /// \u{f89}: 'ྉ'
    SignMchuCan,
    /// \u{f8a}: 'ྊ'
    SignGruCanRgyings,
    /// \u{f8b}: 'ྋ'
    SignGruMedRgyings,
    /// \u{f8c}: 'ྌ'
    SignInvertedMchuCan,
    /// \u{f8d}: 'ྍ'
    SubjoinedSignLceTsaCan,
    /// \u{f8e}: 'ྎ'
    SubjoinedSignMchuCan,
    /// \u{f8f}: 'ྏ'
    SubjoinedSignInvertedMchuCan,
    /// \u{f90}: 'ྐ'
    SubjoinedLetterKa,
    /// \u{f91}: 'ྑ'
    SubjoinedLetterKha,
    /// \u{f92}: 'ྒ'
    SubjoinedLetterGa,
    /// \u{f93}: 'ྒྷ'
    SubjoinedLetterGha,
    /// \u{f94}: 'ྔ'
    SubjoinedLetterNga,
    /// \u{f95}: 'ྕ'
    SubjoinedLetterCa,
    /// \u{f96}: 'ྖ'
    SubjoinedLetterCha,
    /// \u{f97}: 'ྗ'
    SubjoinedLetterJa,
    /// \u{f99}: 'ྙ'
    SubjoinedLetterNya,
    /// \u{f9a}: 'ྚ'
    SubjoinedLetterTta,
    /// \u{f9b}: 'ྛ'
    SubjoinedLetterTtha,
    /// \u{f9c}: 'ྜ'
    SubjoinedLetterDda,
    /// \u{f9d}: 'ྜྷ'
    SubjoinedLetterDdha,
    /// \u{f9e}: 'ྞ'
    SubjoinedLetterNna,
    /// \u{f9f}: 'ྟ'
    SubjoinedLetterTa,
    /// \u{fa0}: 'ྠ'
    SubjoinedLetterTha,
    /// \u{fa1}: 'ྡ'
    SubjoinedLetterDa,
    /// \u{fa2}: 'ྡྷ'
    SubjoinedLetterDha,
    /// \u{fa3}: 'ྣ'
    SubjoinedLetterNa,
    /// \u{fa4}: 'ྤ'
    SubjoinedLetterPa,
    /// \u{fa5}: 'ྥ'
    SubjoinedLetterPha,
    /// \u{fa6}: 'ྦ'
    SubjoinedLetterBa,
    /// \u{fa7}: 'ྦྷ'
    SubjoinedLetterBha,
    /// \u{fa8}: 'ྨ'
    SubjoinedLetterMa,
    /// \u{fa9}: 'ྩ'
    SubjoinedLetterTsa,
    /// \u{faa}: 'ྪ'
    SubjoinedLetterTsha,
    /// \u{fab}: 'ྫ'
    SubjoinedLetterDza,
    /// \u{fac}: 'ྫྷ'
    SubjoinedLetterDzha,
    /// \u{fad}: 'ྭ'
    SubjoinedLetterWa,
    /// \u{fae}: 'ྮ'
    SubjoinedLetterZha,
    /// \u{faf}: 'ྯ'
    SubjoinedLetterZa,
    /// \u{fb0}: 'ྰ'
    SubjoinedLetterDashA,
    /// \u{fb1}: 'ྱ'
    SubjoinedLetterYa,
    /// \u{fb2}: 'ྲ'
    SubjoinedLetterRa,
    /// \u{fb3}: 'ླ'
    SubjoinedLetterLa,
    /// \u{fb4}: 'ྴ'
    SubjoinedLetterSha,
    /// \u{fb5}: 'ྵ'
    SubjoinedLetterSsa,
    /// \u{fb6}: 'ྶ'
    SubjoinedLetterSa,
    /// \u{fb7}: 'ྷ'
    SubjoinedLetterHa,
    /// \u{fb8}: 'ྸ'
    SubjoinedLetterA,
    /// \u{fb9}: 'ྐྵ'
    SubjoinedLetterKssa,
    /// \u{fba}: 'ྺ'
    SubjoinedLetterFixedDashFormWa,
    /// \u{fbb}: 'ྻ'
    SubjoinedLetterFixedDashFormYa,
    /// \u{fbc}: 'ྼ'
    SubjoinedLetterFixedDashFormRa,
    /// \u{fbe}: '྾'
    KuRuKha,
    /// \u{fbf}: '྿'
    KuRuKhaBzhiMigCan,
    /// \u{fc0}: '࿀'
    CantillationSignHeavyBeat,
    /// \u{fc1}: '࿁'
    CantillationSignLightBeat,
    /// \u{fc2}: '࿂'
    CantillationSignCangTeDashU,
    /// \u{fc3}: '࿃'
    CantillationSignSbubDashChal,
    /// \u{fc4}: '࿄'
    SymbolDrilBu,
    /// \u{fc5}: '࿅'
    SymbolRdoRje,
    /// \u{fc6}: '࿆'
    SymbolPadmaGdan,
    /// \u{fc7}: '࿇'
    SymbolRdoRjeRgyaGram,
    /// \u{fc8}: '࿈'
    SymbolPhurPa,
    /// \u{fc9}: '࿉'
    SymbolNorBu,
    /// \u{fca}: '࿊'
    SymbolNorBuNyisDashKhyil,
    /// \u{fcb}: '࿋'
    SymbolNorBuGsumDashKhyil,
    /// \u{fcc}: '࿌'
    SymbolNorBuBzhiDashKhyil,
    /// \u{fce}: '࿎'
    SignRdelNagRdelDkar,
    /// \u{fcf}: '࿏'
    SignRdelNagGsum,
    /// \u{fd0}: '࿐'
    MarkBskaDashShogGiMgoRgyan,
    /// \u{fd1}: '࿑'
    MarkMnyamYigGiMgoRgyan,
    /// \u{fd2}: '࿒'
    MarkNyisTsheg,
    /// \u{fd3}: '࿓'
    MarkInitialBrdaRnyingYigMgoMdunMa,
    /// \u{fd4}: '࿔'
    MarkClosingBrdaRnyingYigMgoSgabMa,
    /// \u{fd5}: '࿕'
    RightDashFacingSvastiSign,
    /// \u{fd6}: '࿖'
    LeftDashFacingSvastiSign,
    /// \u{fd7}: '࿗'
    RightDashFacingSvastiSignWithDots,
    /// \u{fd8}: '࿘'
    LeftDashFacingSvastiSignWithDots,
    /// \u{fd9}: '࿙'
    MarkLeadingMchanRtags,
    /// \u{fda}: '࿚'
    MarkTrailingMchanRtags,
}

impl Into<char> for Tibetan {
    fn into(self) -> char {
        use constants::*;
        match self {
            Tibetan::SyllableOm => SYLLABLE_OM,
            Tibetan::MarkGterYigMgoTruncatedA => MARK_GTER_YIG_MGO_TRUNCATED_A,
            Tibetan::MarkGterYigMgoDashUmRnamBcadMa => MARK_GTER_YIG_MGO__DASH_UM_RNAM_BCAD_MA,
            Tibetan::MarkGterYigMgoDashUmGterTshegMa => MARK_GTER_YIG_MGO__DASH_UM_GTER_TSHEG_MA,
            Tibetan::MarkInitialYigMgoMdunMa => MARK_INITIAL_YIG_MGO_MDUN_MA,
            Tibetan::MarkClosingYigMgoSgabMa => MARK_CLOSING_YIG_MGO_SGAB_MA,
            Tibetan::MarkCaretYigMgoPhurShadMa => MARK_CARET_YIG_MGO_PHUR_SHAD_MA,
            Tibetan::MarkYigMgoTshegShadMa => MARK_YIG_MGO_TSHEG_SHAD_MA,
            Tibetan::MarkSbrulShad => MARK_SBRUL_SHAD,
            Tibetan::MarkBskurYigMgo => MARK_BSKUR_YIG_MGO,
            Tibetan::MarkBkaDashShogYigMgo => MARK_BKA_DASH__SHOG_YIG_MGO,
            Tibetan::MarkIntersyllabicTsheg => MARK_INTERSYLLABIC_TSHEG,
            Tibetan::MarkDelimiterTshegBstar => MARK_DELIMITER_TSHEG_BSTAR,
            Tibetan::MarkShad => MARK_SHAD,
            Tibetan::MarkNyisShad => MARK_NYIS_SHAD,
            Tibetan::MarkTshegShad => MARK_TSHEG_SHAD,
            Tibetan::MarkNyisTshegShad => MARK_NYIS_TSHEG_SHAD,
            Tibetan::MarkRinChenSpungsShad => MARK_RIN_CHEN_SPUNGS_SHAD,
            Tibetan::MarkRgyaGramShad => MARK_RGYA_GRAM_SHAD,
            Tibetan::MarkCaretDashDzudRtagsMeLongCan => MARK_CARET__DASH_DZUD_RTAGS_ME_LONG_CAN,
            Tibetan::MarkGterTsheg => MARK_GTER_TSHEG,
            Tibetan::LogotypeSignChadRtags => LOGOTYPE_SIGN_CHAD_RTAGS,
            Tibetan::LogotypeSignLhagRtags => LOGOTYPE_SIGN_LHAG_RTAGS,
            Tibetan::AstrologicalSignSgraGcanDashCharRtags => ASTROLOGICAL_SIGN_SGRA_GCAN__DASH_CHAR_RTAGS,
            Tibetan::AstrologicalSignDashKhyudPa => ASTROLOGICAL_SIGN__DASH_KHYUD_PA,
            Tibetan::AstrologicalSignSdongTshugs => ASTROLOGICAL_SIGN_SDONG_TSHUGS,
            Tibetan::SignRdelDkarGcig => SIGN_RDEL_DKAR_GCIG,
            Tibetan::SignRdelDkarGnyis => SIGN_RDEL_DKAR_GNYIS,
            Tibetan::SignRdelDkarGsum => SIGN_RDEL_DKAR_GSUM,
            Tibetan::SignRdelNagGcig => SIGN_RDEL_NAG_GCIG,
            Tibetan::SignRdelNagGnyis => SIGN_RDEL_NAG_GNYIS,
            Tibetan::SignRdelDkarRdelNag => SIGN_RDEL_DKAR_RDEL_NAG,
            Tibetan::DigitZero => DIGIT_ZERO,
            Tibetan::DigitOne => DIGIT_ONE,
            Tibetan::DigitTwo => DIGIT_TWO,
            Tibetan::DigitThree => DIGIT_THREE,
            Tibetan::DigitFour => DIGIT_FOUR,
            Tibetan::DigitFive => DIGIT_FIVE,
            Tibetan::DigitSix => DIGIT_SIX,
            Tibetan::DigitSeven => DIGIT_SEVEN,
            Tibetan::DigitEight => DIGIT_EIGHT,
            Tibetan::DigitNine => DIGIT_NINE,
            Tibetan::DigitHalfOne => DIGIT_HALF_ONE,
            Tibetan::DigitHalfTwo => DIGIT_HALF_TWO,
            Tibetan::DigitHalfThree => DIGIT_HALF_THREE,
            Tibetan::DigitHalfFour => DIGIT_HALF_FOUR,
            Tibetan::DigitHalfFive => DIGIT_HALF_FIVE,
            Tibetan::DigitHalfSix => DIGIT_HALF_SIX,
            Tibetan::DigitHalfSeven => DIGIT_HALF_SEVEN,
            Tibetan::DigitHalfEight => DIGIT_HALF_EIGHT,
            Tibetan::DigitHalfNine => DIGIT_HALF_NINE,
            Tibetan::DigitHalfZero => DIGIT_HALF_ZERO,
            Tibetan::MarkBsdusRtags => MARK_BSDUS_RTAGS,
            Tibetan::MarkNgasBzungNyiZla => MARK_NGAS_BZUNG_NYI_ZLA,
            Tibetan::MarkCaretDashDzudRtagsBzhiMigCan => MARK_CARET__DASH_DZUD_RTAGS_BZHI_MIG_CAN,
            Tibetan::MarkNgasBzungSgorRtags => MARK_NGAS_BZUNG_SGOR_RTAGS,
            Tibetan::MarkCheMgo => MARK_CHE_MGO,
            Tibetan::MarkTsaDashPhru => MARK_TSA__DASH_PHRU,
            Tibetan::MarkGugRtagsGyon => MARK_GUG_RTAGS_GYON,
            Tibetan::MarkGugRtagsGyas => MARK_GUG_RTAGS_GYAS,
            Tibetan::MarkAngKhangGyon => MARK_ANG_KHANG_GYON,
            Tibetan::MarkAngKhangGyas => MARK_ANG_KHANG_GYAS,
            Tibetan::SignYarTshes => SIGN_YAR_TSHES,
            Tibetan::SignMarTshes => SIGN_MAR_TSHES,
            Tibetan::LetterKa => LETTER_KA,
            Tibetan::LetterKha => LETTER_KHA,
            Tibetan::LetterGa => LETTER_GA,
            Tibetan::LetterGha => LETTER_GHA,
            Tibetan::LetterNga => LETTER_NGA,
            Tibetan::LetterCa => LETTER_CA,
            Tibetan::LetterCha => LETTER_CHA,
            Tibetan::LetterJa => LETTER_JA,
            Tibetan::LetterNya => LETTER_NYA,
            Tibetan::LetterTta => LETTER_TTA,
            Tibetan::LetterTtha => LETTER_TTHA,
            Tibetan::LetterDda => LETTER_DDA,
            Tibetan::LetterDdha => LETTER_DDHA,
            Tibetan::LetterNna => LETTER_NNA,
            Tibetan::LetterTa => LETTER_TA,
            Tibetan::LetterTha => LETTER_THA,
            Tibetan::LetterDa => LETTER_DA,
            Tibetan::LetterDha => LETTER_DHA,
            Tibetan::LetterNa => LETTER_NA,
            Tibetan::LetterPa => LETTER_PA,
            Tibetan::LetterPha => LETTER_PHA,
            Tibetan::LetterBa => LETTER_BA,
            Tibetan::LetterBha => LETTER_BHA,
            Tibetan::LetterMa => LETTER_MA,
            Tibetan::LetterTsa => LETTER_TSA,
            Tibetan::LetterTsha => LETTER_TSHA,
            Tibetan::LetterDza => LETTER_DZA,
            Tibetan::LetterDzha => LETTER_DZHA,
            Tibetan::LetterWa => LETTER_WA,
            Tibetan::LetterZha => LETTER_ZHA,
            Tibetan::LetterZa => LETTER_ZA,
            Tibetan::LetterDashA => LETTER__DASH_A,
            Tibetan::LetterYa => LETTER_YA,
            Tibetan::LetterRa => LETTER_RA,
            Tibetan::LetterLa => LETTER_LA,
            Tibetan::LetterSha => LETTER_SHA,
            Tibetan::LetterSsa => LETTER_SSA,
            Tibetan::LetterSa => LETTER_SA,
            Tibetan::LetterHa => LETTER_HA,
            Tibetan::LetterA => LETTER_A,
            Tibetan::LetterKssa => LETTER_KSSA,
            Tibetan::LetterFixedDashFormRa => LETTER_FIXED_DASH_FORM_RA,
            Tibetan::LetterKka => LETTER_KKA,
            Tibetan::LetterRra => LETTER_RRA,
            Tibetan::VowelSignAa => VOWEL_SIGN_AA,
            Tibetan::VowelSignI => VOWEL_SIGN_I,
            Tibetan::VowelSignIi => VOWEL_SIGN_II,
            Tibetan::VowelSignU => VOWEL_SIGN_U,
            Tibetan::VowelSignUu => VOWEL_SIGN_UU,
            Tibetan::VowelSignVocalicR => VOWEL_SIGN_VOCALIC_R,
            Tibetan::VowelSignVocalicRr => VOWEL_SIGN_VOCALIC_RR,
            Tibetan::VowelSignVocalicL => VOWEL_SIGN_VOCALIC_L,
            Tibetan::VowelSignVocalicLl => VOWEL_SIGN_VOCALIC_LL,
            Tibetan::VowelSignE => VOWEL_SIGN_E,
            Tibetan::VowelSignEe => VOWEL_SIGN_EE,
            Tibetan::VowelSignO => VOWEL_SIGN_O,
            Tibetan::VowelSignOo => VOWEL_SIGN_OO,
            Tibetan::SignRjesSuNgaRo => SIGN_RJES_SU_NGA_RO,
            Tibetan::SignRnamBcad => SIGN_RNAM_BCAD,
            Tibetan::VowelSignReversedI => VOWEL_SIGN_REVERSED_I,
            Tibetan::VowelSignReversedIi => VOWEL_SIGN_REVERSED_II,
            Tibetan::SignNyiZlaNaaDa => SIGN_NYI_ZLA_NAA_DA,
            Tibetan::SignSnaLdan => SIGN_SNA_LDAN,
            Tibetan::MarkHalanta => MARK_HALANTA,
            Tibetan::MarkPaluta => MARK_PALUTA,
            Tibetan::SignLciRtags => SIGN_LCI_RTAGS,
            Tibetan::SignYangRtags => SIGN_YANG_RTAGS,
            Tibetan::SignLceTsaCan => SIGN_LCE_TSA_CAN,
            Tibetan::SignMchuCan => SIGN_MCHU_CAN,
            Tibetan::SignGruCanRgyings => SIGN_GRU_CAN_RGYINGS,
            Tibetan::SignGruMedRgyings => SIGN_GRU_MED_RGYINGS,
            Tibetan::SignInvertedMchuCan => SIGN_INVERTED_MCHU_CAN,
            Tibetan::SubjoinedSignLceTsaCan => SUBJOINED_SIGN_LCE_TSA_CAN,
            Tibetan::SubjoinedSignMchuCan => SUBJOINED_SIGN_MCHU_CAN,
            Tibetan::SubjoinedSignInvertedMchuCan => SUBJOINED_SIGN_INVERTED_MCHU_CAN,
            Tibetan::SubjoinedLetterKa => SUBJOINED_LETTER_KA,
            Tibetan::SubjoinedLetterKha => SUBJOINED_LETTER_KHA,
            Tibetan::SubjoinedLetterGa => SUBJOINED_LETTER_GA,
            Tibetan::SubjoinedLetterGha => SUBJOINED_LETTER_GHA,
            Tibetan::SubjoinedLetterNga => SUBJOINED_LETTER_NGA,
            Tibetan::SubjoinedLetterCa => SUBJOINED_LETTER_CA,
            Tibetan::SubjoinedLetterCha => SUBJOINED_LETTER_CHA,
            Tibetan::SubjoinedLetterJa => SUBJOINED_LETTER_JA,
            Tibetan::SubjoinedLetterNya => SUBJOINED_LETTER_NYA,
            Tibetan::SubjoinedLetterTta => SUBJOINED_LETTER_TTA,
            Tibetan::SubjoinedLetterTtha => SUBJOINED_LETTER_TTHA,
            Tibetan::SubjoinedLetterDda => SUBJOINED_LETTER_DDA,
            Tibetan::SubjoinedLetterDdha => SUBJOINED_LETTER_DDHA,
            Tibetan::SubjoinedLetterNna => SUBJOINED_LETTER_NNA,
            Tibetan::SubjoinedLetterTa => SUBJOINED_LETTER_TA,
            Tibetan::SubjoinedLetterTha => SUBJOINED_LETTER_THA,
            Tibetan::SubjoinedLetterDa => SUBJOINED_LETTER_DA,
            Tibetan::SubjoinedLetterDha => SUBJOINED_LETTER_DHA,
            Tibetan::SubjoinedLetterNa => SUBJOINED_LETTER_NA,
            Tibetan::SubjoinedLetterPa => SUBJOINED_LETTER_PA,
            Tibetan::SubjoinedLetterPha => SUBJOINED_LETTER_PHA,
            Tibetan::SubjoinedLetterBa => SUBJOINED_LETTER_BA,
            Tibetan::SubjoinedLetterBha => SUBJOINED_LETTER_BHA,
            Tibetan::SubjoinedLetterMa => SUBJOINED_LETTER_MA,
            Tibetan::SubjoinedLetterTsa => SUBJOINED_LETTER_TSA,
            Tibetan::SubjoinedLetterTsha => SUBJOINED_LETTER_TSHA,
            Tibetan::SubjoinedLetterDza => SUBJOINED_LETTER_DZA,
            Tibetan::SubjoinedLetterDzha => SUBJOINED_LETTER_DZHA,
            Tibetan::SubjoinedLetterWa => SUBJOINED_LETTER_WA,
            Tibetan::SubjoinedLetterZha => SUBJOINED_LETTER_ZHA,
            Tibetan::SubjoinedLetterZa => SUBJOINED_LETTER_ZA,
            Tibetan::SubjoinedLetterDashA => SUBJOINED_LETTER__DASH_A,
            Tibetan::SubjoinedLetterYa => SUBJOINED_LETTER_YA,
            Tibetan::SubjoinedLetterRa => SUBJOINED_LETTER_RA,
            Tibetan::SubjoinedLetterLa => SUBJOINED_LETTER_LA,
            Tibetan::SubjoinedLetterSha => SUBJOINED_LETTER_SHA,
            Tibetan::SubjoinedLetterSsa => SUBJOINED_LETTER_SSA,
            Tibetan::SubjoinedLetterSa => SUBJOINED_LETTER_SA,
            Tibetan::SubjoinedLetterHa => SUBJOINED_LETTER_HA,
            Tibetan::SubjoinedLetterA => SUBJOINED_LETTER_A,
            Tibetan::SubjoinedLetterKssa => SUBJOINED_LETTER_KSSA,
            Tibetan::SubjoinedLetterFixedDashFormWa => SUBJOINED_LETTER_FIXED_DASH_FORM_WA,
            Tibetan::SubjoinedLetterFixedDashFormYa => SUBJOINED_LETTER_FIXED_DASH_FORM_YA,
            Tibetan::SubjoinedLetterFixedDashFormRa => SUBJOINED_LETTER_FIXED_DASH_FORM_RA,
            Tibetan::KuRuKha => KU_RU_KHA,
            Tibetan::KuRuKhaBzhiMigCan => KU_RU_KHA_BZHI_MIG_CAN,
            Tibetan::CantillationSignHeavyBeat => CANTILLATION_SIGN_HEAVY_BEAT,
            Tibetan::CantillationSignLightBeat => CANTILLATION_SIGN_LIGHT_BEAT,
            Tibetan::CantillationSignCangTeDashU => CANTILLATION_SIGN_CANG_TE_DASH_U,
            Tibetan::CantillationSignSbubDashChal => CANTILLATION_SIGN_SBUB__DASH_CHAL,
            Tibetan::SymbolDrilBu => SYMBOL_DRIL_BU,
            Tibetan::SymbolRdoRje => SYMBOL_RDO_RJE,
            Tibetan::SymbolPadmaGdan => SYMBOL_PADMA_GDAN,
            Tibetan::SymbolRdoRjeRgyaGram => SYMBOL_RDO_RJE_RGYA_GRAM,
            Tibetan::SymbolPhurPa => SYMBOL_PHUR_PA,
            Tibetan::SymbolNorBu => SYMBOL_NOR_BU,
            Tibetan::SymbolNorBuNyisDashKhyil => SYMBOL_NOR_BU_NYIS__DASH_KHYIL,
            Tibetan::SymbolNorBuGsumDashKhyil => SYMBOL_NOR_BU_GSUM__DASH_KHYIL,
            Tibetan::SymbolNorBuBzhiDashKhyil => SYMBOL_NOR_BU_BZHI__DASH_KHYIL,
            Tibetan::SignRdelNagRdelDkar => SIGN_RDEL_NAG_RDEL_DKAR,
            Tibetan::SignRdelNagGsum => SIGN_RDEL_NAG_GSUM,
            Tibetan::MarkBskaDashShogGiMgoRgyan => MARK_BSKA_DASH__SHOG_GI_MGO_RGYAN,
            Tibetan::MarkMnyamYigGiMgoRgyan => MARK_MNYAM_YIG_GI_MGO_RGYAN,
            Tibetan::MarkNyisTsheg => MARK_NYIS_TSHEG,
            Tibetan::MarkInitialBrdaRnyingYigMgoMdunMa => MARK_INITIAL_BRDA_RNYING_YIG_MGO_MDUN_MA,
            Tibetan::MarkClosingBrdaRnyingYigMgoSgabMa => MARK_CLOSING_BRDA_RNYING_YIG_MGO_SGAB_MA,
            Tibetan::RightDashFacingSvastiSign => RIGHT_DASH_FACING_SVASTI_SIGN,
            Tibetan::LeftDashFacingSvastiSign => LEFT_DASH_FACING_SVASTI_SIGN,
            Tibetan::RightDashFacingSvastiSignWithDots => RIGHT_DASH_FACING_SVASTI_SIGN_WITH_DOTS,
            Tibetan::LeftDashFacingSvastiSignWithDots => LEFT_DASH_FACING_SVASTI_SIGN_WITH_DOTS,
            Tibetan::MarkLeadingMchanRtags => MARK_LEADING_MCHAN_RTAGS,
            Tibetan::MarkTrailingMchanRtags => MARK_TRAILING_MCHAN_RTAGS,
        }
    }
}

impl std::convert::TryFrom<char> for Tibetan {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            SYLLABLE_OM => Ok(Tibetan::SyllableOm),
            MARK_GTER_YIG_MGO_TRUNCATED_A => Ok(Tibetan::MarkGterYigMgoTruncatedA),
            MARK_GTER_YIG_MGO__DASH_UM_RNAM_BCAD_MA => Ok(Tibetan::MarkGterYigMgoDashUmRnamBcadMa),
            MARK_GTER_YIG_MGO__DASH_UM_GTER_TSHEG_MA => Ok(Tibetan::MarkGterYigMgoDashUmGterTshegMa),
            MARK_INITIAL_YIG_MGO_MDUN_MA => Ok(Tibetan::MarkInitialYigMgoMdunMa),
            MARK_CLOSING_YIG_MGO_SGAB_MA => Ok(Tibetan::MarkClosingYigMgoSgabMa),
            MARK_CARET_YIG_MGO_PHUR_SHAD_MA => Ok(Tibetan::MarkCaretYigMgoPhurShadMa),
            MARK_YIG_MGO_TSHEG_SHAD_MA => Ok(Tibetan::MarkYigMgoTshegShadMa),
            MARK_SBRUL_SHAD => Ok(Tibetan::MarkSbrulShad),
            MARK_BSKUR_YIG_MGO => Ok(Tibetan::MarkBskurYigMgo),
            MARK_BKA_DASH__SHOG_YIG_MGO => Ok(Tibetan::MarkBkaDashShogYigMgo),
            MARK_INTERSYLLABIC_TSHEG => Ok(Tibetan::MarkIntersyllabicTsheg),
            MARK_DELIMITER_TSHEG_BSTAR => Ok(Tibetan::MarkDelimiterTshegBstar),
            MARK_SHAD => Ok(Tibetan::MarkShad),
            MARK_NYIS_SHAD => Ok(Tibetan::MarkNyisShad),
            MARK_TSHEG_SHAD => Ok(Tibetan::MarkTshegShad),
            MARK_NYIS_TSHEG_SHAD => Ok(Tibetan::MarkNyisTshegShad),
            MARK_RIN_CHEN_SPUNGS_SHAD => Ok(Tibetan::MarkRinChenSpungsShad),
            MARK_RGYA_GRAM_SHAD => Ok(Tibetan::MarkRgyaGramShad),
            MARK_CARET__DASH_DZUD_RTAGS_ME_LONG_CAN => Ok(Tibetan::MarkCaretDashDzudRtagsMeLongCan),
            MARK_GTER_TSHEG => Ok(Tibetan::MarkGterTsheg),
            LOGOTYPE_SIGN_CHAD_RTAGS => Ok(Tibetan::LogotypeSignChadRtags),
            LOGOTYPE_SIGN_LHAG_RTAGS => Ok(Tibetan::LogotypeSignLhagRtags),
            ASTROLOGICAL_SIGN_SGRA_GCAN__DASH_CHAR_RTAGS => Ok(Tibetan::AstrologicalSignSgraGcanDashCharRtags),
            ASTROLOGICAL_SIGN__DASH_KHYUD_PA => Ok(Tibetan::AstrologicalSignDashKhyudPa),
            ASTROLOGICAL_SIGN_SDONG_TSHUGS => Ok(Tibetan::AstrologicalSignSdongTshugs),
            SIGN_RDEL_DKAR_GCIG => Ok(Tibetan::SignRdelDkarGcig),
            SIGN_RDEL_DKAR_GNYIS => Ok(Tibetan::SignRdelDkarGnyis),
            SIGN_RDEL_DKAR_GSUM => Ok(Tibetan::SignRdelDkarGsum),
            SIGN_RDEL_NAG_GCIG => Ok(Tibetan::SignRdelNagGcig),
            SIGN_RDEL_NAG_GNYIS => Ok(Tibetan::SignRdelNagGnyis),
            SIGN_RDEL_DKAR_RDEL_NAG => Ok(Tibetan::SignRdelDkarRdelNag),
            DIGIT_ZERO => Ok(Tibetan::DigitZero),
            DIGIT_ONE => Ok(Tibetan::DigitOne),
            DIGIT_TWO => Ok(Tibetan::DigitTwo),
            DIGIT_THREE => Ok(Tibetan::DigitThree),
            DIGIT_FOUR => Ok(Tibetan::DigitFour),
            DIGIT_FIVE => Ok(Tibetan::DigitFive),
            DIGIT_SIX => Ok(Tibetan::DigitSix),
            DIGIT_SEVEN => Ok(Tibetan::DigitSeven),
            DIGIT_EIGHT => Ok(Tibetan::DigitEight),
            DIGIT_NINE => Ok(Tibetan::DigitNine),
            DIGIT_HALF_ONE => Ok(Tibetan::DigitHalfOne),
            DIGIT_HALF_TWO => Ok(Tibetan::DigitHalfTwo),
            DIGIT_HALF_THREE => Ok(Tibetan::DigitHalfThree),
            DIGIT_HALF_FOUR => Ok(Tibetan::DigitHalfFour),
            DIGIT_HALF_FIVE => Ok(Tibetan::DigitHalfFive),
            DIGIT_HALF_SIX => Ok(Tibetan::DigitHalfSix),
            DIGIT_HALF_SEVEN => Ok(Tibetan::DigitHalfSeven),
            DIGIT_HALF_EIGHT => Ok(Tibetan::DigitHalfEight),
            DIGIT_HALF_NINE => Ok(Tibetan::DigitHalfNine),
            DIGIT_HALF_ZERO => Ok(Tibetan::DigitHalfZero),
            MARK_BSDUS_RTAGS => Ok(Tibetan::MarkBsdusRtags),
            MARK_NGAS_BZUNG_NYI_ZLA => Ok(Tibetan::MarkNgasBzungNyiZla),
            MARK_CARET__DASH_DZUD_RTAGS_BZHI_MIG_CAN => Ok(Tibetan::MarkCaretDashDzudRtagsBzhiMigCan),
            MARK_NGAS_BZUNG_SGOR_RTAGS => Ok(Tibetan::MarkNgasBzungSgorRtags),
            MARK_CHE_MGO => Ok(Tibetan::MarkCheMgo),
            MARK_TSA__DASH_PHRU => Ok(Tibetan::MarkTsaDashPhru),
            MARK_GUG_RTAGS_GYON => Ok(Tibetan::MarkGugRtagsGyon),
            MARK_GUG_RTAGS_GYAS => Ok(Tibetan::MarkGugRtagsGyas),
            MARK_ANG_KHANG_GYON => Ok(Tibetan::MarkAngKhangGyon),
            MARK_ANG_KHANG_GYAS => Ok(Tibetan::MarkAngKhangGyas),
            SIGN_YAR_TSHES => Ok(Tibetan::SignYarTshes),
            SIGN_MAR_TSHES => Ok(Tibetan::SignMarTshes),
            LETTER_KA => Ok(Tibetan::LetterKa),
            LETTER_KHA => Ok(Tibetan::LetterKha),
            LETTER_GA => Ok(Tibetan::LetterGa),
            LETTER_GHA => Ok(Tibetan::LetterGha),
            LETTER_NGA => Ok(Tibetan::LetterNga),
            LETTER_CA => Ok(Tibetan::LetterCa),
            LETTER_CHA => Ok(Tibetan::LetterCha),
            LETTER_JA => Ok(Tibetan::LetterJa),
            LETTER_NYA => Ok(Tibetan::LetterNya),
            LETTER_TTA => Ok(Tibetan::LetterTta),
            LETTER_TTHA => Ok(Tibetan::LetterTtha),
            LETTER_DDA => Ok(Tibetan::LetterDda),
            LETTER_DDHA => Ok(Tibetan::LetterDdha),
            LETTER_NNA => Ok(Tibetan::LetterNna),
            LETTER_TA => Ok(Tibetan::LetterTa),
            LETTER_THA => Ok(Tibetan::LetterTha),
            LETTER_DA => Ok(Tibetan::LetterDa),
            LETTER_DHA => Ok(Tibetan::LetterDha),
            LETTER_NA => Ok(Tibetan::LetterNa),
            LETTER_PA => Ok(Tibetan::LetterPa),
            LETTER_PHA => Ok(Tibetan::LetterPha),
            LETTER_BA => Ok(Tibetan::LetterBa),
            LETTER_BHA => Ok(Tibetan::LetterBha),
            LETTER_MA => Ok(Tibetan::LetterMa),
            LETTER_TSA => Ok(Tibetan::LetterTsa),
            LETTER_TSHA => Ok(Tibetan::LetterTsha),
            LETTER_DZA => Ok(Tibetan::LetterDza),
            LETTER_DZHA => Ok(Tibetan::LetterDzha),
            LETTER_WA => Ok(Tibetan::LetterWa),
            LETTER_ZHA => Ok(Tibetan::LetterZha),
            LETTER_ZA => Ok(Tibetan::LetterZa),
            LETTER__DASH_A => Ok(Tibetan::LetterDashA),
            LETTER_YA => Ok(Tibetan::LetterYa),
            LETTER_RA => Ok(Tibetan::LetterRa),
            LETTER_LA => Ok(Tibetan::LetterLa),
            LETTER_SHA => Ok(Tibetan::LetterSha),
            LETTER_SSA => Ok(Tibetan::LetterSsa),
            LETTER_SA => Ok(Tibetan::LetterSa),
            LETTER_HA => Ok(Tibetan::LetterHa),
            LETTER_A => Ok(Tibetan::LetterA),
            LETTER_KSSA => Ok(Tibetan::LetterKssa),
            LETTER_FIXED_DASH_FORM_RA => Ok(Tibetan::LetterFixedDashFormRa),
            LETTER_KKA => Ok(Tibetan::LetterKka),
            LETTER_RRA => Ok(Tibetan::LetterRra),
            VOWEL_SIGN_AA => Ok(Tibetan::VowelSignAa),
            VOWEL_SIGN_I => Ok(Tibetan::VowelSignI),
            VOWEL_SIGN_II => Ok(Tibetan::VowelSignIi),
            VOWEL_SIGN_U => Ok(Tibetan::VowelSignU),
            VOWEL_SIGN_UU => Ok(Tibetan::VowelSignUu),
            VOWEL_SIGN_VOCALIC_R => Ok(Tibetan::VowelSignVocalicR),
            VOWEL_SIGN_VOCALIC_RR => Ok(Tibetan::VowelSignVocalicRr),
            VOWEL_SIGN_VOCALIC_L => Ok(Tibetan::VowelSignVocalicL),
            VOWEL_SIGN_VOCALIC_LL => Ok(Tibetan::VowelSignVocalicLl),
            VOWEL_SIGN_E => Ok(Tibetan::VowelSignE),
            VOWEL_SIGN_EE => Ok(Tibetan::VowelSignEe),
            VOWEL_SIGN_O => Ok(Tibetan::VowelSignO),
            VOWEL_SIGN_OO => Ok(Tibetan::VowelSignOo),
            SIGN_RJES_SU_NGA_RO => Ok(Tibetan::SignRjesSuNgaRo),
            SIGN_RNAM_BCAD => Ok(Tibetan::SignRnamBcad),
            VOWEL_SIGN_REVERSED_I => Ok(Tibetan::VowelSignReversedI),
            VOWEL_SIGN_REVERSED_II => Ok(Tibetan::VowelSignReversedIi),
            SIGN_NYI_ZLA_NAA_DA => Ok(Tibetan::SignNyiZlaNaaDa),
            SIGN_SNA_LDAN => Ok(Tibetan::SignSnaLdan),
            MARK_HALANTA => Ok(Tibetan::MarkHalanta),
            MARK_PALUTA => Ok(Tibetan::MarkPaluta),
            SIGN_LCI_RTAGS => Ok(Tibetan::SignLciRtags),
            SIGN_YANG_RTAGS => Ok(Tibetan::SignYangRtags),
            SIGN_LCE_TSA_CAN => Ok(Tibetan::SignLceTsaCan),
            SIGN_MCHU_CAN => Ok(Tibetan::SignMchuCan),
            SIGN_GRU_CAN_RGYINGS => Ok(Tibetan::SignGruCanRgyings),
            SIGN_GRU_MED_RGYINGS => Ok(Tibetan::SignGruMedRgyings),
            SIGN_INVERTED_MCHU_CAN => Ok(Tibetan::SignInvertedMchuCan),
            SUBJOINED_SIGN_LCE_TSA_CAN => Ok(Tibetan::SubjoinedSignLceTsaCan),
            SUBJOINED_SIGN_MCHU_CAN => Ok(Tibetan::SubjoinedSignMchuCan),
            SUBJOINED_SIGN_INVERTED_MCHU_CAN => Ok(Tibetan::SubjoinedSignInvertedMchuCan),
            SUBJOINED_LETTER_KA => Ok(Tibetan::SubjoinedLetterKa),
            SUBJOINED_LETTER_KHA => Ok(Tibetan::SubjoinedLetterKha),
            SUBJOINED_LETTER_GA => Ok(Tibetan::SubjoinedLetterGa),
            SUBJOINED_LETTER_GHA => Ok(Tibetan::SubjoinedLetterGha),
            SUBJOINED_LETTER_NGA => Ok(Tibetan::SubjoinedLetterNga),
            SUBJOINED_LETTER_CA => Ok(Tibetan::SubjoinedLetterCa),
            SUBJOINED_LETTER_CHA => Ok(Tibetan::SubjoinedLetterCha),
            SUBJOINED_LETTER_JA => Ok(Tibetan::SubjoinedLetterJa),
            SUBJOINED_LETTER_NYA => Ok(Tibetan::SubjoinedLetterNya),
            SUBJOINED_LETTER_TTA => Ok(Tibetan::SubjoinedLetterTta),
            SUBJOINED_LETTER_TTHA => Ok(Tibetan::SubjoinedLetterTtha),
            SUBJOINED_LETTER_DDA => Ok(Tibetan::SubjoinedLetterDda),
            SUBJOINED_LETTER_DDHA => Ok(Tibetan::SubjoinedLetterDdha),
            SUBJOINED_LETTER_NNA => Ok(Tibetan::SubjoinedLetterNna),
            SUBJOINED_LETTER_TA => Ok(Tibetan::SubjoinedLetterTa),
            SUBJOINED_LETTER_THA => Ok(Tibetan::SubjoinedLetterTha),
            SUBJOINED_LETTER_DA => Ok(Tibetan::SubjoinedLetterDa),
            SUBJOINED_LETTER_DHA => Ok(Tibetan::SubjoinedLetterDha),
            SUBJOINED_LETTER_NA => Ok(Tibetan::SubjoinedLetterNa),
            SUBJOINED_LETTER_PA => Ok(Tibetan::SubjoinedLetterPa),
            SUBJOINED_LETTER_PHA => Ok(Tibetan::SubjoinedLetterPha),
            SUBJOINED_LETTER_BA => Ok(Tibetan::SubjoinedLetterBa),
            SUBJOINED_LETTER_BHA => Ok(Tibetan::SubjoinedLetterBha),
            SUBJOINED_LETTER_MA => Ok(Tibetan::SubjoinedLetterMa),
            SUBJOINED_LETTER_TSA => Ok(Tibetan::SubjoinedLetterTsa),
            SUBJOINED_LETTER_TSHA => Ok(Tibetan::SubjoinedLetterTsha),
            SUBJOINED_LETTER_DZA => Ok(Tibetan::SubjoinedLetterDza),
            SUBJOINED_LETTER_DZHA => Ok(Tibetan::SubjoinedLetterDzha),
            SUBJOINED_LETTER_WA => Ok(Tibetan::SubjoinedLetterWa),
            SUBJOINED_LETTER_ZHA => Ok(Tibetan::SubjoinedLetterZha),
            SUBJOINED_LETTER_ZA => Ok(Tibetan::SubjoinedLetterZa),
            SUBJOINED_LETTER__DASH_A => Ok(Tibetan::SubjoinedLetterDashA),
            SUBJOINED_LETTER_YA => Ok(Tibetan::SubjoinedLetterYa),
            SUBJOINED_LETTER_RA => Ok(Tibetan::SubjoinedLetterRa),
            SUBJOINED_LETTER_LA => Ok(Tibetan::SubjoinedLetterLa),
            SUBJOINED_LETTER_SHA => Ok(Tibetan::SubjoinedLetterSha),
            SUBJOINED_LETTER_SSA => Ok(Tibetan::SubjoinedLetterSsa),
            SUBJOINED_LETTER_SA => Ok(Tibetan::SubjoinedLetterSa),
            SUBJOINED_LETTER_HA => Ok(Tibetan::SubjoinedLetterHa),
            SUBJOINED_LETTER_A => Ok(Tibetan::SubjoinedLetterA),
            SUBJOINED_LETTER_KSSA => Ok(Tibetan::SubjoinedLetterKssa),
            SUBJOINED_LETTER_FIXED_DASH_FORM_WA => Ok(Tibetan::SubjoinedLetterFixedDashFormWa),
            SUBJOINED_LETTER_FIXED_DASH_FORM_YA => Ok(Tibetan::SubjoinedLetterFixedDashFormYa),
            SUBJOINED_LETTER_FIXED_DASH_FORM_RA => Ok(Tibetan::SubjoinedLetterFixedDashFormRa),
            KU_RU_KHA => Ok(Tibetan::KuRuKha),
            KU_RU_KHA_BZHI_MIG_CAN => Ok(Tibetan::KuRuKhaBzhiMigCan),
            CANTILLATION_SIGN_HEAVY_BEAT => Ok(Tibetan::CantillationSignHeavyBeat),
            CANTILLATION_SIGN_LIGHT_BEAT => Ok(Tibetan::CantillationSignLightBeat),
            CANTILLATION_SIGN_CANG_TE_DASH_U => Ok(Tibetan::CantillationSignCangTeDashU),
            CANTILLATION_SIGN_SBUB__DASH_CHAL => Ok(Tibetan::CantillationSignSbubDashChal),
            SYMBOL_DRIL_BU => Ok(Tibetan::SymbolDrilBu),
            SYMBOL_RDO_RJE => Ok(Tibetan::SymbolRdoRje),
            SYMBOL_PADMA_GDAN => Ok(Tibetan::SymbolPadmaGdan),
            SYMBOL_RDO_RJE_RGYA_GRAM => Ok(Tibetan::SymbolRdoRjeRgyaGram),
            SYMBOL_PHUR_PA => Ok(Tibetan::SymbolPhurPa),
            SYMBOL_NOR_BU => Ok(Tibetan::SymbolNorBu),
            SYMBOL_NOR_BU_NYIS__DASH_KHYIL => Ok(Tibetan::SymbolNorBuNyisDashKhyil),
            SYMBOL_NOR_BU_GSUM__DASH_KHYIL => Ok(Tibetan::SymbolNorBuGsumDashKhyil),
            SYMBOL_NOR_BU_BZHI__DASH_KHYIL => Ok(Tibetan::SymbolNorBuBzhiDashKhyil),
            SIGN_RDEL_NAG_RDEL_DKAR => Ok(Tibetan::SignRdelNagRdelDkar),
            SIGN_RDEL_NAG_GSUM => Ok(Tibetan::SignRdelNagGsum),
            MARK_BSKA_DASH__SHOG_GI_MGO_RGYAN => Ok(Tibetan::MarkBskaDashShogGiMgoRgyan),
            MARK_MNYAM_YIG_GI_MGO_RGYAN => Ok(Tibetan::MarkMnyamYigGiMgoRgyan),
            MARK_NYIS_TSHEG => Ok(Tibetan::MarkNyisTsheg),
            MARK_INITIAL_BRDA_RNYING_YIG_MGO_MDUN_MA => Ok(Tibetan::MarkInitialBrdaRnyingYigMgoMdunMa),
            MARK_CLOSING_BRDA_RNYING_YIG_MGO_SGAB_MA => Ok(Tibetan::MarkClosingBrdaRnyingYigMgoSgabMa),
            RIGHT_DASH_FACING_SVASTI_SIGN => Ok(Tibetan::RightDashFacingSvastiSign),
            LEFT_DASH_FACING_SVASTI_SIGN => Ok(Tibetan::LeftDashFacingSvastiSign),
            RIGHT_DASH_FACING_SVASTI_SIGN_WITH_DOTS => Ok(Tibetan::RightDashFacingSvastiSignWithDots),
            LEFT_DASH_FACING_SVASTI_SIGN_WITH_DOTS => Ok(Tibetan::LeftDashFacingSvastiSignWithDots),
            MARK_LEADING_MCHAN_RTAGS => Ok(Tibetan::MarkLeadingMchanRtags),
            MARK_TRAILING_MCHAN_RTAGS => Ok(Tibetan::MarkTrailingMchanRtags),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Tibetan {
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

impl std::convert::TryFrom<u32> for Tibetan {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Tibetan {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Tibetan {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Tibetan::SyllableOm
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Tibetan::SyllableOm => "tibetan syllable om",
            Tibetan::MarkGterYigMgoTruncatedA => "tibetan mark gter yig mgo truncated a",
            Tibetan::MarkGterYigMgoDashUmRnamBcadMa => "tibetan mark gter yig mgo -um rnam bcad ma",
            Tibetan::MarkGterYigMgoDashUmGterTshegMa => "tibetan mark gter yig mgo -um gter tsheg ma",
            Tibetan::MarkInitialYigMgoMdunMa => "tibetan mark initial yig mgo mdun ma",
            Tibetan::MarkClosingYigMgoSgabMa => "tibetan mark closing yig mgo sgab ma",
            Tibetan::MarkCaretYigMgoPhurShadMa => "tibetan mark caret yig mgo phur shad ma",
            Tibetan::MarkYigMgoTshegShadMa => "tibetan mark yig mgo tsheg shad ma",
            Tibetan::MarkSbrulShad => "tibetan mark sbrul shad",
            Tibetan::MarkBskurYigMgo => "tibetan mark bskur yig mgo",
            Tibetan::MarkBkaDashShogYigMgo => "tibetan mark bka- shog yig mgo",
            Tibetan::MarkIntersyllabicTsheg => "tibetan mark intersyllabic tsheg",
            Tibetan::MarkDelimiterTshegBstar => "tibetan mark delimiter tsheg bstar",
            Tibetan::MarkShad => "tibetan mark shad",
            Tibetan::MarkNyisShad => "tibetan mark nyis shad",
            Tibetan::MarkTshegShad => "tibetan mark tsheg shad",
            Tibetan::MarkNyisTshegShad => "tibetan mark nyis tsheg shad",
            Tibetan::MarkRinChenSpungsShad => "tibetan mark rin chen spungs shad",
            Tibetan::MarkRgyaGramShad => "tibetan mark rgya gram shad",
            Tibetan::MarkCaretDashDzudRtagsMeLongCan => "tibetan mark caret -dzud rtags me long can",
            Tibetan::MarkGterTsheg => "tibetan mark gter tsheg",
            Tibetan::LogotypeSignChadRtags => "tibetan logotype sign chad rtags",
            Tibetan::LogotypeSignLhagRtags => "tibetan logotype sign lhag rtags",
            Tibetan::AstrologicalSignSgraGcanDashCharRtags => "tibetan astrological sign sgra gcan -char rtags",
            Tibetan::AstrologicalSignDashKhyudPa => "tibetan astrological sign -khyud pa",
            Tibetan::AstrologicalSignSdongTshugs => "tibetan astrological sign sdong tshugs",
            Tibetan::SignRdelDkarGcig => "tibetan sign rdel dkar gcig",
            Tibetan::SignRdelDkarGnyis => "tibetan sign rdel dkar gnyis",
            Tibetan::SignRdelDkarGsum => "tibetan sign rdel dkar gsum",
            Tibetan::SignRdelNagGcig => "tibetan sign rdel nag gcig",
            Tibetan::SignRdelNagGnyis => "tibetan sign rdel nag gnyis",
            Tibetan::SignRdelDkarRdelNag => "tibetan sign rdel dkar rdel nag",
            Tibetan::DigitZero => "tibetan digit zero",
            Tibetan::DigitOne => "tibetan digit one",
            Tibetan::DigitTwo => "tibetan digit two",
            Tibetan::DigitThree => "tibetan digit three",
            Tibetan::DigitFour => "tibetan digit four",
            Tibetan::DigitFive => "tibetan digit five",
            Tibetan::DigitSix => "tibetan digit six",
            Tibetan::DigitSeven => "tibetan digit seven",
            Tibetan::DigitEight => "tibetan digit eight",
            Tibetan::DigitNine => "tibetan digit nine",
            Tibetan::DigitHalfOne => "tibetan digit half one",
            Tibetan::DigitHalfTwo => "tibetan digit half two",
            Tibetan::DigitHalfThree => "tibetan digit half three",
            Tibetan::DigitHalfFour => "tibetan digit half four",
            Tibetan::DigitHalfFive => "tibetan digit half five",
            Tibetan::DigitHalfSix => "tibetan digit half six",
            Tibetan::DigitHalfSeven => "tibetan digit half seven",
            Tibetan::DigitHalfEight => "tibetan digit half eight",
            Tibetan::DigitHalfNine => "tibetan digit half nine",
            Tibetan::DigitHalfZero => "tibetan digit half zero",
            Tibetan::MarkBsdusRtags => "tibetan mark bsdus rtags",
            Tibetan::MarkNgasBzungNyiZla => "tibetan mark ngas bzung nyi zla",
            Tibetan::MarkCaretDashDzudRtagsBzhiMigCan => "tibetan mark caret -dzud rtags bzhi mig can",
            Tibetan::MarkNgasBzungSgorRtags => "tibetan mark ngas bzung sgor rtags",
            Tibetan::MarkCheMgo => "tibetan mark che mgo",
            Tibetan::MarkTsaDashPhru => "tibetan mark tsa -phru",
            Tibetan::MarkGugRtagsGyon => "tibetan mark gug rtags gyon",
            Tibetan::MarkGugRtagsGyas => "tibetan mark gug rtags gyas",
            Tibetan::MarkAngKhangGyon => "tibetan mark ang khang gyon",
            Tibetan::MarkAngKhangGyas => "tibetan mark ang khang gyas",
            Tibetan::SignYarTshes => "tibetan sign yar tshes",
            Tibetan::SignMarTshes => "tibetan sign mar tshes",
            Tibetan::LetterKa => "tibetan letter ka",
            Tibetan::LetterKha => "tibetan letter kha",
            Tibetan::LetterGa => "tibetan letter ga",
            Tibetan::LetterGha => "tibetan letter gha",
            Tibetan::LetterNga => "tibetan letter nga",
            Tibetan::LetterCa => "tibetan letter ca",
            Tibetan::LetterCha => "tibetan letter cha",
            Tibetan::LetterJa => "tibetan letter ja",
            Tibetan::LetterNya => "tibetan letter nya",
            Tibetan::LetterTta => "tibetan letter tta",
            Tibetan::LetterTtha => "tibetan letter ttha",
            Tibetan::LetterDda => "tibetan letter dda",
            Tibetan::LetterDdha => "tibetan letter ddha",
            Tibetan::LetterNna => "tibetan letter nna",
            Tibetan::LetterTa => "tibetan letter ta",
            Tibetan::LetterTha => "tibetan letter tha",
            Tibetan::LetterDa => "tibetan letter da",
            Tibetan::LetterDha => "tibetan letter dha",
            Tibetan::LetterNa => "tibetan letter na",
            Tibetan::LetterPa => "tibetan letter pa",
            Tibetan::LetterPha => "tibetan letter pha",
            Tibetan::LetterBa => "tibetan letter ba",
            Tibetan::LetterBha => "tibetan letter bha",
            Tibetan::LetterMa => "tibetan letter ma",
            Tibetan::LetterTsa => "tibetan letter tsa",
            Tibetan::LetterTsha => "tibetan letter tsha",
            Tibetan::LetterDza => "tibetan letter dza",
            Tibetan::LetterDzha => "tibetan letter dzha",
            Tibetan::LetterWa => "tibetan letter wa",
            Tibetan::LetterZha => "tibetan letter zha",
            Tibetan::LetterZa => "tibetan letter za",
            Tibetan::LetterDashA => "tibetan letter -a",
            Tibetan::LetterYa => "tibetan letter ya",
            Tibetan::LetterRa => "tibetan letter ra",
            Tibetan::LetterLa => "tibetan letter la",
            Tibetan::LetterSha => "tibetan letter sha",
            Tibetan::LetterSsa => "tibetan letter ssa",
            Tibetan::LetterSa => "tibetan letter sa",
            Tibetan::LetterHa => "tibetan letter ha",
            Tibetan::LetterA => "tibetan letter a",
            Tibetan::LetterKssa => "tibetan letter kssa",
            Tibetan::LetterFixedDashFormRa => "tibetan letter fixed-form ra",
            Tibetan::LetterKka => "tibetan letter kka",
            Tibetan::LetterRra => "tibetan letter rra",
            Tibetan::VowelSignAa => "tibetan vowel sign aa",
            Tibetan::VowelSignI => "tibetan vowel sign i",
            Tibetan::VowelSignIi => "tibetan vowel sign ii",
            Tibetan::VowelSignU => "tibetan vowel sign u",
            Tibetan::VowelSignUu => "tibetan vowel sign uu",
            Tibetan::VowelSignVocalicR => "tibetan vowel sign vocalic r",
            Tibetan::VowelSignVocalicRr => "tibetan vowel sign vocalic rr",
            Tibetan::VowelSignVocalicL => "tibetan vowel sign vocalic l",
            Tibetan::VowelSignVocalicLl => "tibetan vowel sign vocalic ll",
            Tibetan::VowelSignE => "tibetan vowel sign e",
            Tibetan::VowelSignEe => "tibetan vowel sign ee",
            Tibetan::VowelSignO => "tibetan vowel sign o",
            Tibetan::VowelSignOo => "tibetan vowel sign oo",
            Tibetan::SignRjesSuNgaRo => "tibetan sign rjes su nga ro",
            Tibetan::SignRnamBcad => "tibetan sign rnam bcad",
            Tibetan::VowelSignReversedI => "tibetan vowel sign reversed i",
            Tibetan::VowelSignReversedIi => "tibetan vowel sign reversed ii",
            Tibetan::SignNyiZlaNaaDa => "tibetan sign nyi zla naa da",
            Tibetan::SignSnaLdan => "tibetan sign sna ldan",
            Tibetan::MarkHalanta => "tibetan mark halanta",
            Tibetan::MarkPaluta => "tibetan mark paluta",
            Tibetan::SignLciRtags => "tibetan sign lci rtags",
            Tibetan::SignYangRtags => "tibetan sign yang rtags",
            Tibetan::SignLceTsaCan => "tibetan sign lce tsa can",
            Tibetan::SignMchuCan => "tibetan sign mchu can",
            Tibetan::SignGruCanRgyings => "tibetan sign gru can rgyings",
            Tibetan::SignGruMedRgyings => "tibetan sign gru med rgyings",
            Tibetan::SignInvertedMchuCan => "tibetan sign inverted mchu can",
            Tibetan::SubjoinedSignLceTsaCan => "tibetan subjoined sign lce tsa can",
            Tibetan::SubjoinedSignMchuCan => "tibetan subjoined sign mchu can",
            Tibetan::SubjoinedSignInvertedMchuCan => "tibetan subjoined sign inverted mchu can",
            Tibetan::SubjoinedLetterKa => "tibetan subjoined letter ka",
            Tibetan::SubjoinedLetterKha => "tibetan subjoined letter kha",
            Tibetan::SubjoinedLetterGa => "tibetan subjoined letter ga",
            Tibetan::SubjoinedLetterGha => "tibetan subjoined letter gha",
            Tibetan::SubjoinedLetterNga => "tibetan subjoined letter nga",
            Tibetan::SubjoinedLetterCa => "tibetan subjoined letter ca",
            Tibetan::SubjoinedLetterCha => "tibetan subjoined letter cha",
            Tibetan::SubjoinedLetterJa => "tibetan subjoined letter ja",
            Tibetan::SubjoinedLetterNya => "tibetan subjoined letter nya",
            Tibetan::SubjoinedLetterTta => "tibetan subjoined letter tta",
            Tibetan::SubjoinedLetterTtha => "tibetan subjoined letter ttha",
            Tibetan::SubjoinedLetterDda => "tibetan subjoined letter dda",
            Tibetan::SubjoinedLetterDdha => "tibetan subjoined letter ddha",
            Tibetan::SubjoinedLetterNna => "tibetan subjoined letter nna",
            Tibetan::SubjoinedLetterTa => "tibetan subjoined letter ta",
            Tibetan::SubjoinedLetterTha => "tibetan subjoined letter tha",
            Tibetan::SubjoinedLetterDa => "tibetan subjoined letter da",
            Tibetan::SubjoinedLetterDha => "tibetan subjoined letter dha",
            Tibetan::SubjoinedLetterNa => "tibetan subjoined letter na",
            Tibetan::SubjoinedLetterPa => "tibetan subjoined letter pa",
            Tibetan::SubjoinedLetterPha => "tibetan subjoined letter pha",
            Tibetan::SubjoinedLetterBa => "tibetan subjoined letter ba",
            Tibetan::SubjoinedLetterBha => "tibetan subjoined letter bha",
            Tibetan::SubjoinedLetterMa => "tibetan subjoined letter ma",
            Tibetan::SubjoinedLetterTsa => "tibetan subjoined letter tsa",
            Tibetan::SubjoinedLetterTsha => "tibetan subjoined letter tsha",
            Tibetan::SubjoinedLetterDza => "tibetan subjoined letter dza",
            Tibetan::SubjoinedLetterDzha => "tibetan subjoined letter dzha",
            Tibetan::SubjoinedLetterWa => "tibetan subjoined letter wa",
            Tibetan::SubjoinedLetterZha => "tibetan subjoined letter zha",
            Tibetan::SubjoinedLetterZa => "tibetan subjoined letter za",
            Tibetan::SubjoinedLetterDashA => "tibetan subjoined letter -a",
            Tibetan::SubjoinedLetterYa => "tibetan subjoined letter ya",
            Tibetan::SubjoinedLetterRa => "tibetan subjoined letter ra",
            Tibetan::SubjoinedLetterLa => "tibetan subjoined letter la",
            Tibetan::SubjoinedLetterSha => "tibetan subjoined letter sha",
            Tibetan::SubjoinedLetterSsa => "tibetan subjoined letter ssa",
            Tibetan::SubjoinedLetterSa => "tibetan subjoined letter sa",
            Tibetan::SubjoinedLetterHa => "tibetan subjoined letter ha",
            Tibetan::SubjoinedLetterA => "tibetan subjoined letter a",
            Tibetan::SubjoinedLetterKssa => "tibetan subjoined letter kssa",
            Tibetan::SubjoinedLetterFixedDashFormWa => "tibetan subjoined letter fixed-form wa",
            Tibetan::SubjoinedLetterFixedDashFormYa => "tibetan subjoined letter fixed-form ya",
            Tibetan::SubjoinedLetterFixedDashFormRa => "tibetan subjoined letter fixed-form ra",
            Tibetan::KuRuKha => "tibetan ku ru kha",
            Tibetan::KuRuKhaBzhiMigCan => "tibetan ku ru kha bzhi mig can",
            Tibetan::CantillationSignHeavyBeat => "tibetan cantillation sign heavy beat",
            Tibetan::CantillationSignLightBeat => "tibetan cantillation sign light beat",
            Tibetan::CantillationSignCangTeDashU => "tibetan cantillation sign cang te-u",
            Tibetan::CantillationSignSbubDashChal => "tibetan cantillation sign sbub -chal",
            Tibetan::SymbolDrilBu => "tibetan symbol dril bu",
            Tibetan::SymbolRdoRje => "tibetan symbol rdo rje",
            Tibetan::SymbolPadmaGdan => "tibetan symbol padma gdan",
            Tibetan::SymbolRdoRjeRgyaGram => "tibetan symbol rdo rje rgya gram",
            Tibetan::SymbolPhurPa => "tibetan symbol phur pa",
            Tibetan::SymbolNorBu => "tibetan symbol nor bu",
            Tibetan::SymbolNorBuNyisDashKhyil => "tibetan symbol nor bu nyis -khyil",
            Tibetan::SymbolNorBuGsumDashKhyil => "tibetan symbol nor bu gsum -khyil",
            Tibetan::SymbolNorBuBzhiDashKhyil => "tibetan symbol nor bu bzhi -khyil",
            Tibetan::SignRdelNagRdelDkar => "tibetan sign rdel nag rdel dkar",
            Tibetan::SignRdelNagGsum => "tibetan sign rdel nag gsum",
            Tibetan::MarkBskaDashShogGiMgoRgyan => "tibetan mark bska- shog gi mgo rgyan",
            Tibetan::MarkMnyamYigGiMgoRgyan => "tibetan mark mnyam yig gi mgo rgyan",
            Tibetan::MarkNyisTsheg => "tibetan mark nyis tsheg",
            Tibetan::MarkInitialBrdaRnyingYigMgoMdunMa => "tibetan mark initial brda rnying yig mgo mdun ma",
            Tibetan::MarkClosingBrdaRnyingYigMgoSgabMa => "tibetan mark closing brda rnying yig mgo sgab ma",
            Tibetan::RightDashFacingSvastiSign => "right-facing svasti sign",
            Tibetan::LeftDashFacingSvastiSign => "left-facing svasti sign",
            Tibetan::RightDashFacingSvastiSignWithDots => "right-facing svasti sign with dots",
            Tibetan::LeftDashFacingSvastiSignWithDots => "left-facing svasti sign with dots",
            Tibetan::MarkLeadingMchanRtags => "tibetan mark leading mchan rtags",
            Tibetan::MarkTrailingMchanRtags => "tibetan mark trailing mchan rtags",
        }
    }
}
