/// \u{f00} → \u{fff}\
///\
/// ༀ ༁ ༂ ༃ ༄ ༅ ༆ ༇ ༈ ༉ ༊ ་ ༌ ། ༎ ༏
/// ༐ ༑ ༒ ༓ ༔ ༕ ༖ ༗ ༘ ༙ ༚ ༛ ༜ ༝ ༞ ༟
/// ༠ ༡ ༢ ༣ ༤ ༥ ༦ ༧ ༨ ༩ ༪ ༫ ༬ ༭ ༮ ༯
/// ༰ ༱ ༲ ༳ ༴ ༵ ༶ ༷ ༸ ༹ ༺ ༻ ༼ ༽ ༾ ༿
/// ཀ ཁ ག གྷ ང ཅ ཆ ཇ ཉ ཊ ཋ ཌ ཌྷ ཎ ཏ ཐ
/// ད དྷ ན པ ཕ བ བྷ མ ཙ ཚ ཛ ཛྷ ཝ ཞ ཟ འ
/// ཡ ར ལ ཤ ཥ ས ཧ ཨ ཀྵ ཪ ཫ ཬ ཱ ི ཱི ུ
/// ཱུ ྲྀ ཷ ླྀ ཹ ེ ཻ ོ ཽ ཾ ཿ ྀ ཱྀ ྂ ྃ ྄
/// ྅ ྆ ྇ ྈ ྉ ྊ ྋ ྌ ྍ ྎ ྏ ྐ ྑ ྒ ྒྷ ྔ
/// ྕ ྖ ྗ ྙ ྚ ྛ ྜ ྜྷ ྞ ྟ ྠ ྡ ྡྷ ྣ ྤ ྥ
/// ྦ ྦྷ ྨ ྩ ྪ ྫ ྫྷ ྭ ྮ ྯ ྰ ྱ ྲ ླ ྴ ྵ
/// ྶ ྷ ྸ ྐྵ ྺ ྻ ྼ ྾ ྿ ࿀ ࿁ ࿂ ࿃ ࿄ ࿅ ࿆
/// ࿇ ࿈ ࿉ ࿊ ࿋ ࿌ ࿎ ࿏ ࿐ ࿑ ࿒ ࿓ ࿔ ࿕ ࿖ ࿗
/// ࿘ ࿙ ࿚
pub mod constants {
    /// \u{f00}: 'ༀ'
    pub const TIBETAN_SYLLABLE_OM: char = 'ༀ';
    /// \u{f01}: '༁'
    pub const TIBETAN_MARK_GTER_YIG_MGO_TRUNCATED_A: char = '༁';
    /// \u{f02}: '༂'
    pub const TIBETAN_MARK_GTER_YIG_MGO__DASH_UM_RNAM_BCAD_MA: char = '༂';
    /// \u{f03}: '༃'
    pub const TIBETAN_MARK_GTER_YIG_MGO__DASH_UM_GTER_TSHEG_MA: char = '༃';
    /// \u{f04}: '༄'
    pub const TIBETAN_MARK_INITIAL_YIG_MGO_MDUN_MA: char = '༄';
    /// \u{f05}: '༅'
    pub const TIBETAN_MARK_CLOSING_YIG_MGO_SGAB_MA: char = '༅';
    /// \u{f06}: '༆'
    pub const TIBETAN_MARK_CARET_YIG_MGO_PHUR_SHAD_MA: char = '༆';
    /// \u{f07}: '༇'
    pub const TIBETAN_MARK_YIG_MGO_TSHEG_SHAD_MA: char = '༇';
    /// \u{f08}: '༈'
    pub const TIBETAN_MARK_SBRUL_SHAD: char = '༈';
    /// \u{f09}: '༉'
    pub const TIBETAN_MARK_BSKUR_YIG_MGO: char = '༉';
    /// \u{f0a}: '༊'
    pub const TIBETAN_MARK_BKA_DASH__SHOG_YIG_MGO: char = '༊';
    /// \u{f0b}: '་'
    pub const TIBETAN_MARK_INTERSYLLABIC_TSHEG: char = '་';
    /// \u{f0c}: '༌'
    pub const TIBETAN_MARK_DELIMITER_TSHEG_BSTAR: char = '༌';
    /// \u{f0d}: '།'
    pub const TIBETAN_MARK_SHAD: char = '།';
    /// \u{f0e}: '༎'
    pub const TIBETAN_MARK_NYIS_SHAD: char = '༎';
    /// \u{f0f}: '༏'
    pub const TIBETAN_MARK_TSHEG_SHAD: char = '༏';
    /// \u{f10}: '༐'
    pub const TIBETAN_MARK_NYIS_TSHEG_SHAD: char = '༐';
    /// \u{f11}: '༑'
    pub const TIBETAN_MARK_RIN_CHEN_SPUNGS_SHAD: char = '༑';
    /// \u{f12}: '༒'
    pub const TIBETAN_MARK_RGYA_GRAM_SHAD: char = '༒';
    /// \u{f13}: '༓'
    pub const TIBETAN_MARK_CARET__DASH_DZUD_RTAGS_ME_LONG_CAN: char = '༓';
    /// \u{f14}: '༔'
    pub const TIBETAN_MARK_GTER_TSHEG: char = '༔';
    /// \u{f15}: '༕'
    pub const TIBETAN_LOGOTYPE_SIGN_CHAD_RTAGS: char = '༕';
    /// \u{f16}: '༖'
    pub const TIBETAN_LOGOTYPE_SIGN_LHAG_RTAGS: char = '༖';
    /// \u{f17}: '༗'
    pub const TIBETAN_ASTROLOGICAL_SIGN_SGRA_GCAN__DASH_CHAR_RTAGS: char = '༗';
    /// \u{f18}: '༘'
    pub const TIBETAN_ASTROLOGICAL_SIGN__DASH_KHYUD_PA: char = '༘';
    /// \u{f19}: '༙'
    pub const TIBETAN_ASTROLOGICAL_SIGN_SDONG_TSHUGS: char = '༙';
    /// \u{f1a}: '༚'
    pub const TIBETAN_SIGN_RDEL_DKAR_GCIG: char = '༚';
    /// \u{f1b}: '༛'
    pub const TIBETAN_SIGN_RDEL_DKAR_GNYIS: char = '༛';
    /// \u{f1c}: '༜'
    pub const TIBETAN_SIGN_RDEL_DKAR_GSUM: char = '༜';
    /// \u{f1d}: '༝'
    pub const TIBETAN_SIGN_RDEL_NAG_GCIG: char = '༝';
    /// \u{f1e}: '༞'
    pub const TIBETAN_SIGN_RDEL_NAG_GNYIS: char = '༞';
    /// \u{f1f}: '༟'
    pub const TIBETAN_SIGN_RDEL_DKAR_RDEL_NAG: char = '༟';
    /// \u{f20}: '༠'
    pub const TIBETAN_DIGIT_ZERO: char = '༠';
    /// \u{f21}: '༡'
    pub const TIBETAN_DIGIT_ONE: char = '༡';
    /// \u{f22}: '༢'
    pub const TIBETAN_DIGIT_TWO: char = '༢';
    /// \u{f23}: '༣'
    pub const TIBETAN_DIGIT_THREE: char = '༣';
    /// \u{f24}: '༤'
    pub const TIBETAN_DIGIT_FOUR: char = '༤';
    /// \u{f25}: '༥'
    pub const TIBETAN_DIGIT_FIVE: char = '༥';
    /// \u{f26}: '༦'
    pub const TIBETAN_DIGIT_SIX: char = '༦';
    /// \u{f27}: '༧'
    pub const TIBETAN_DIGIT_SEVEN: char = '༧';
    /// \u{f28}: '༨'
    pub const TIBETAN_DIGIT_EIGHT: char = '༨';
    /// \u{f29}: '༩'
    pub const TIBETAN_DIGIT_NINE: char = '༩';
    /// \u{f2a}: '༪'
    pub const TIBETAN_DIGIT_HALF_ONE: char = '༪';
    /// \u{f2b}: '༫'
    pub const TIBETAN_DIGIT_HALF_TWO: char = '༫';
    /// \u{f2c}: '༬'
    pub const TIBETAN_DIGIT_HALF_THREE: char = '༬';
    /// \u{f2d}: '༭'
    pub const TIBETAN_DIGIT_HALF_FOUR: char = '༭';
    /// \u{f2e}: '༮'
    pub const TIBETAN_DIGIT_HALF_FIVE: char = '༮';
    /// \u{f2f}: '༯'
    pub const TIBETAN_DIGIT_HALF_SIX: char = '༯';
    /// \u{f30}: '༰'
    pub const TIBETAN_DIGIT_HALF_SEVEN: char = '༰';
    /// \u{f31}: '༱'
    pub const TIBETAN_DIGIT_HALF_EIGHT: char = '༱';
    /// \u{f32}: '༲'
    pub const TIBETAN_DIGIT_HALF_NINE: char = '༲';
    /// \u{f33}: '༳'
    pub const TIBETAN_DIGIT_HALF_ZERO: char = '༳';
    /// \u{f34}: '༴'
    pub const TIBETAN_MARK_BSDUS_RTAGS: char = '༴';
    /// \u{f35}: '༵'
    pub const TIBETAN_MARK_NGAS_BZUNG_NYI_ZLA: char = '༵';
    /// \u{f36}: '༶'
    pub const TIBETAN_MARK_CARET__DASH_DZUD_RTAGS_BZHI_MIG_CAN: char = '༶';
    /// \u{f37}: '༷'
    pub const TIBETAN_MARK_NGAS_BZUNG_SGOR_RTAGS: char = '༷';
    /// \u{f38}: '༸'
    pub const TIBETAN_MARK_CHE_MGO: char = '༸';
    /// \u{f39}: '༹'
    pub const TIBETAN_MARK_TSA__DASH_PHRU: char = '༹';
    /// \u{f3a}: '༺'
    pub const TIBETAN_MARK_GUG_RTAGS_GYON: char = '༺';
    /// \u{f3b}: '༻'
    pub const TIBETAN_MARK_GUG_RTAGS_GYAS: char = '༻';
    /// \u{f3c}: '༼'
    pub const TIBETAN_MARK_ANG_KHANG_GYON: char = '༼';
    /// \u{f3d}: '༽'
    pub const TIBETAN_MARK_ANG_KHANG_GYAS: char = '༽';
    /// \u{f3e}: '༾'
    pub const TIBETAN_SIGN_YAR_TSHES: char = '༾';
    /// \u{f3f}: '༿'
    pub const TIBETAN_SIGN_MAR_TSHES: char = '༿';
    /// \u{f40}: 'ཀ'
    pub const TIBETAN_LETTER_KA: char = 'ཀ';
    /// \u{f41}: 'ཁ'
    pub const TIBETAN_LETTER_KHA: char = 'ཁ';
    /// \u{f42}: 'ག'
    pub const TIBETAN_LETTER_GA: char = 'ག';
    /// \u{f43}: 'གྷ'
    pub const TIBETAN_LETTER_GHA: char = 'གྷ';
    /// \u{f44}: 'ང'
    pub const TIBETAN_LETTER_NGA: char = 'ང';
    /// \u{f45}: 'ཅ'
    pub const TIBETAN_LETTER_CA: char = 'ཅ';
    /// \u{f46}: 'ཆ'
    pub const TIBETAN_LETTER_CHA: char = 'ཆ';
    /// \u{f47}: 'ཇ'
    pub const TIBETAN_LETTER_JA: char = 'ཇ';
    /// \u{f49}: 'ཉ'
    pub const TIBETAN_LETTER_NYA: char = 'ཉ';
    /// \u{f4a}: 'ཊ'
    pub const TIBETAN_LETTER_TTA: char = 'ཊ';
    /// \u{f4b}: 'ཋ'
    pub const TIBETAN_LETTER_TTHA: char = 'ཋ';
    /// \u{f4c}: 'ཌ'
    pub const TIBETAN_LETTER_DDA: char = 'ཌ';
    /// \u{f4d}: 'ཌྷ'
    pub const TIBETAN_LETTER_DDHA: char = 'ཌྷ';
    /// \u{f4e}: 'ཎ'
    pub const TIBETAN_LETTER_NNA: char = 'ཎ';
    /// \u{f4f}: 'ཏ'
    pub const TIBETAN_LETTER_TA: char = 'ཏ';
    /// \u{f50}: 'ཐ'
    pub const TIBETAN_LETTER_THA: char = 'ཐ';
    /// \u{f51}: 'ད'
    pub const TIBETAN_LETTER_DA: char = 'ད';
    /// \u{f52}: 'དྷ'
    pub const TIBETAN_LETTER_DHA: char = 'དྷ';
    /// \u{f53}: 'ན'
    pub const TIBETAN_LETTER_NA: char = 'ན';
    /// \u{f54}: 'པ'
    pub const TIBETAN_LETTER_PA: char = 'པ';
    /// \u{f55}: 'ཕ'
    pub const TIBETAN_LETTER_PHA: char = 'ཕ';
    /// \u{f56}: 'བ'
    pub const TIBETAN_LETTER_BA: char = 'བ';
    /// \u{f57}: 'བྷ'
    pub const TIBETAN_LETTER_BHA: char = 'བྷ';
    /// \u{f58}: 'མ'
    pub const TIBETAN_LETTER_MA: char = 'མ';
    /// \u{f59}: 'ཙ'
    pub const TIBETAN_LETTER_TSA: char = 'ཙ';
    /// \u{f5a}: 'ཚ'
    pub const TIBETAN_LETTER_TSHA: char = 'ཚ';
    /// \u{f5b}: 'ཛ'
    pub const TIBETAN_LETTER_DZA: char = 'ཛ';
    /// \u{f5c}: 'ཛྷ'
    pub const TIBETAN_LETTER_DZHA: char = 'ཛྷ';
    /// \u{f5d}: 'ཝ'
    pub const TIBETAN_LETTER_WA: char = 'ཝ';
    /// \u{f5e}: 'ཞ'
    pub const TIBETAN_LETTER_ZHA: char = 'ཞ';
    /// \u{f5f}: 'ཟ'
    pub const TIBETAN_LETTER_ZA: char = 'ཟ';
    /// \u{f60}: 'འ'
    pub const TIBETAN_LETTER__DASH_A: char = 'འ';
    /// \u{f61}: 'ཡ'
    pub const TIBETAN_LETTER_YA: char = 'ཡ';
    /// \u{f62}: 'ར'
    pub const TIBETAN_LETTER_RA: char = 'ར';
    /// \u{f63}: 'ལ'
    pub const TIBETAN_LETTER_LA: char = 'ལ';
    /// \u{f64}: 'ཤ'
    pub const TIBETAN_LETTER_SHA: char = 'ཤ';
    /// \u{f65}: 'ཥ'
    pub const TIBETAN_LETTER_SSA: char = 'ཥ';
    /// \u{f66}: 'ས'
    pub const TIBETAN_LETTER_SA: char = 'ས';
    /// \u{f67}: 'ཧ'
    pub const TIBETAN_LETTER_HA: char = 'ཧ';
    /// \u{f68}: 'ཨ'
    pub const TIBETAN_LETTER_A: char = 'ཨ';
    /// \u{f69}: 'ཀྵ'
    pub const TIBETAN_LETTER_KSSA: char = 'ཀྵ';
    /// \u{f6a}: 'ཪ'
    pub const TIBETAN_LETTER_FIXED_DASH_FORM_RA: char = 'ཪ';
    /// \u{f6b}: 'ཫ'
    pub const TIBETAN_LETTER_KKA: char = 'ཫ';
    /// \u{f6c}: 'ཬ'
    pub const TIBETAN_LETTER_RRA: char = 'ཬ';
    /// \u{f71}: 'ཱ'
    pub const TIBETAN_VOWEL_SIGN_AA: char = 'ཱ';
    /// \u{f72}: 'ི'
    pub const TIBETAN_VOWEL_SIGN_I: char = 'ི';
    /// \u{f73}: 'ཱི'
    pub const TIBETAN_VOWEL_SIGN_II: char = 'ཱི';
    /// \u{f74}: 'ུ'
    pub const TIBETAN_VOWEL_SIGN_U: char = 'ུ';
    /// \u{f75}: 'ཱུ'
    pub const TIBETAN_VOWEL_SIGN_UU: char = 'ཱུ';
    /// \u{f76}: 'ྲྀ'
    pub const TIBETAN_VOWEL_SIGN_VOCALIC_R: char = 'ྲྀ';
    /// \u{f77}: 'ཷ'
    pub const TIBETAN_VOWEL_SIGN_VOCALIC_RR: char = 'ཷ';
    /// \u{f78}: 'ླྀ'
    pub const TIBETAN_VOWEL_SIGN_VOCALIC_L: char = 'ླྀ';
    /// \u{f79}: 'ཹ'
    pub const TIBETAN_VOWEL_SIGN_VOCALIC_LL: char = 'ཹ';
    /// \u{f7a}: 'ེ'
    pub const TIBETAN_VOWEL_SIGN_E: char = 'ེ';
    /// \u{f7b}: 'ཻ'
    pub const TIBETAN_VOWEL_SIGN_EE: char = 'ཻ';
    /// \u{f7c}: 'ོ'
    pub const TIBETAN_VOWEL_SIGN_O: char = 'ོ';
    /// \u{f7d}: 'ཽ'
    pub const TIBETAN_VOWEL_SIGN_OO: char = 'ཽ';
    /// \u{f7e}: 'ཾ'
    pub const TIBETAN_SIGN_RJES_SU_NGA_RO: char = 'ཾ';
    /// \u{f7f}: 'ཿ'
    pub const TIBETAN_SIGN_RNAM_BCAD: char = 'ཿ';
    /// \u{f80}: 'ྀ'
    pub const TIBETAN_VOWEL_SIGN_REVERSED_I: char = 'ྀ';
    /// \u{f81}: 'ཱྀ'
    pub const TIBETAN_VOWEL_SIGN_REVERSED_II: char = 'ཱྀ';
    /// \u{f82}: 'ྂ'
    pub const TIBETAN_SIGN_NYI_ZLA_NAA_DA: char = 'ྂ';
    /// \u{f83}: 'ྃ'
    pub const TIBETAN_SIGN_SNA_LDAN: char = 'ྃ';
    /// \u{f84}: '྄'
    pub const TIBETAN_MARK_HALANTA: char = '྄';
    /// \u{f85}: '྅'
    pub const TIBETAN_MARK_PALUTA: char = '྅';
    /// \u{f86}: '྆'
    pub const TIBETAN_SIGN_LCI_RTAGS: char = '྆';
    /// \u{f87}: '྇'
    pub const TIBETAN_SIGN_YANG_RTAGS: char = '྇';
    /// \u{f88}: 'ྈ'
    pub const TIBETAN_SIGN_LCE_TSA_CAN: char = 'ྈ';
    /// \u{f89}: 'ྉ'
    pub const TIBETAN_SIGN_MCHU_CAN: char = 'ྉ';
    /// \u{f8a}: 'ྊ'
    pub const TIBETAN_SIGN_GRU_CAN_RGYINGS: char = 'ྊ';
    /// \u{f8b}: 'ྋ'
    pub const TIBETAN_SIGN_GRU_MED_RGYINGS: char = 'ྋ';
    /// \u{f8c}: 'ྌ'
    pub const TIBETAN_SIGN_INVERTED_MCHU_CAN: char = 'ྌ';
    /// \u{f8d}: 'ྍ'
    pub const TIBETAN_SUBJOINED_SIGN_LCE_TSA_CAN: char = 'ྍ';
    /// \u{f8e}: 'ྎ'
    pub const TIBETAN_SUBJOINED_SIGN_MCHU_CAN: char = 'ྎ';
    /// \u{f8f}: 'ྏ'
    pub const TIBETAN_SUBJOINED_SIGN_INVERTED_MCHU_CAN: char = 'ྏ';
    /// \u{f90}: 'ྐ'
    pub const TIBETAN_SUBJOINED_LETTER_KA: char = 'ྐ';
    /// \u{f91}: 'ྑ'
    pub const TIBETAN_SUBJOINED_LETTER_KHA: char = 'ྑ';
    /// \u{f92}: 'ྒ'
    pub const TIBETAN_SUBJOINED_LETTER_GA: char = 'ྒ';
    /// \u{f93}: 'ྒྷ'
    pub const TIBETAN_SUBJOINED_LETTER_GHA: char = 'ྒྷ';
    /// \u{f94}: 'ྔ'
    pub const TIBETAN_SUBJOINED_LETTER_NGA: char = 'ྔ';
    /// \u{f95}: 'ྕ'
    pub const TIBETAN_SUBJOINED_LETTER_CA: char = 'ྕ';
    /// \u{f96}: 'ྖ'
    pub const TIBETAN_SUBJOINED_LETTER_CHA: char = 'ྖ';
    /// \u{f97}: 'ྗ'
    pub const TIBETAN_SUBJOINED_LETTER_JA: char = 'ྗ';
    /// \u{f99}: 'ྙ'
    pub const TIBETAN_SUBJOINED_LETTER_NYA: char = 'ྙ';
    /// \u{f9a}: 'ྚ'
    pub const TIBETAN_SUBJOINED_LETTER_TTA: char = 'ྚ';
    /// \u{f9b}: 'ྛ'
    pub const TIBETAN_SUBJOINED_LETTER_TTHA: char = 'ྛ';
    /// \u{f9c}: 'ྜ'
    pub const TIBETAN_SUBJOINED_LETTER_DDA: char = 'ྜ';
    /// \u{f9d}: 'ྜྷ'
    pub const TIBETAN_SUBJOINED_LETTER_DDHA: char = 'ྜྷ';
    /// \u{f9e}: 'ྞ'
    pub const TIBETAN_SUBJOINED_LETTER_NNA: char = 'ྞ';
    /// \u{f9f}: 'ྟ'
    pub const TIBETAN_SUBJOINED_LETTER_TA: char = 'ྟ';
    /// \u{fa0}: 'ྠ'
    pub const TIBETAN_SUBJOINED_LETTER_THA: char = 'ྠ';
    /// \u{fa1}: 'ྡ'
    pub const TIBETAN_SUBJOINED_LETTER_DA: char = 'ྡ';
    /// \u{fa2}: 'ྡྷ'
    pub const TIBETAN_SUBJOINED_LETTER_DHA: char = 'ྡྷ';
    /// \u{fa3}: 'ྣ'
    pub const TIBETAN_SUBJOINED_LETTER_NA: char = 'ྣ';
    /// \u{fa4}: 'ྤ'
    pub const TIBETAN_SUBJOINED_LETTER_PA: char = 'ྤ';
    /// \u{fa5}: 'ྥ'
    pub const TIBETAN_SUBJOINED_LETTER_PHA: char = 'ྥ';
    /// \u{fa6}: 'ྦ'
    pub const TIBETAN_SUBJOINED_LETTER_BA: char = 'ྦ';
    /// \u{fa7}: 'ྦྷ'
    pub const TIBETAN_SUBJOINED_LETTER_BHA: char = 'ྦྷ';
    /// \u{fa8}: 'ྨ'
    pub const TIBETAN_SUBJOINED_LETTER_MA: char = 'ྨ';
    /// \u{fa9}: 'ྩ'
    pub const TIBETAN_SUBJOINED_LETTER_TSA: char = 'ྩ';
    /// \u{faa}: 'ྪ'
    pub const TIBETAN_SUBJOINED_LETTER_TSHA: char = 'ྪ';
    /// \u{fab}: 'ྫ'
    pub const TIBETAN_SUBJOINED_LETTER_DZA: char = 'ྫ';
    /// \u{fac}: 'ྫྷ'
    pub const TIBETAN_SUBJOINED_LETTER_DZHA: char = 'ྫྷ';
    /// \u{fad}: 'ྭ'
    pub const TIBETAN_SUBJOINED_LETTER_WA: char = 'ྭ';
    /// \u{fae}: 'ྮ'
    pub const TIBETAN_SUBJOINED_LETTER_ZHA: char = 'ྮ';
    /// \u{faf}: 'ྯ'
    pub const TIBETAN_SUBJOINED_LETTER_ZA: char = 'ྯ';
    /// \u{fb0}: 'ྰ'
    pub const TIBETAN_SUBJOINED_LETTER__DASH_A: char = 'ྰ';
    /// \u{fb1}: 'ྱ'
    pub const TIBETAN_SUBJOINED_LETTER_YA: char = 'ྱ';
    /// \u{fb2}: 'ྲ'
    pub const TIBETAN_SUBJOINED_LETTER_RA: char = 'ྲ';
    /// \u{fb3}: 'ླ'
    pub const TIBETAN_SUBJOINED_LETTER_LA: char = 'ླ';
    /// \u{fb4}: 'ྴ'
    pub const TIBETAN_SUBJOINED_LETTER_SHA: char = 'ྴ';
    /// \u{fb5}: 'ྵ'
    pub const TIBETAN_SUBJOINED_LETTER_SSA: char = 'ྵ';
    /// \u{fb6}: 'ྶ'
    pub const TIBETAN_SUBJOINED_LETTER_SA: char = 'ྶ';
    /// \u{fb7}: 'ྷ'
    pub const TIBETAN_SUBJOINED_LETTER_HA: char = 'ྷ';
    /// \u{fb8}: 'ྸ'
    pub const TIBETAN_SUBJOINED_LETTER_A: char = 'ྸ';
    /// \u{fb9}: 'ྐྵ'
    pub const TIBETAN_SUBJOINED_LETTER_KSSA: char = 'ྐྵ';
    /// \u{fba}: 'ྺ'
    pub const TIBETAN_SUBJOINED_LETTER_FIXED_DASH_FORM_WA: char = 'ྺ';
    /// \u{fbb}: 'ྻ'
    pub const TIBETAN_SUBJOINED_LETTER_FIXED_DASH_FORM_YA: char = 'ྻ';
    /// \u{fbc}: 'ྼ'
    pub const TIBETAN_SUBJOINED_LETTER_FIXED_DASH_FORM_RA: char = 'ྼ';
    /// \u{fbe}: '྾'
    pub const TIBETAN_KU_RU_KHA: char = '྾';
    /// \u{fbf}: '྿'
    pub const TIBETAN_KU_RU_KHA_BZHI_MIG_CAN: char = '྿';
    /// \u{fc0}: '࿀'
    pub const TIBETAN_CANTILLATION_SIGN_HEAVY_BEAT: char = '࿀';
    /// \u{fc1}: '࿁'
    pub const TIBETAN_CANTILLATION_SIGN_LIGHT_BEAT: char = '࿁';
    /// \u{fc2}: '࿂'
    pub const TIBETAN_CANTILLATION_SIGN_CANG_TE_DASH_U: char = '࿂';
    /// \u{fc3}: '࿃'
    pub const TIBETAN_CANTILLATION_SIGN_SBUB__DASH_CHAL: char = '࿃';
    /// \u{fc4}: '࿄'
    pub const TIBETAN_SYMBOL_DRIL_BU: char = '࿄';
    /// \u{fc5}: '࿅'
    pub const TIBETAN_SYMBOL_RDO_RJE: char = '࿅';
    /// \u{fc6}: '࿆'
    pub const TIBETAN_SYMBOL_PADMA_GDAN: char = '࿆';
    /// \u{fc7}: '࿇'
    pub const TIBETAN_SYMBOL_RDO_RJE_RGYA_GRAM: char = '࿇';
    /// \u{fc8}: '࿈'
    pub const TIBETAN_SYMBOL_PHUR_PA: char = '࿈';
    /// \u{fc9}: '࿉'
    pub const TIBETAN_SYMBOL_NOR_BU: char = '࿉';
    /// \u{fca}: '࿊'
    pub const TIBETAN_SYMBOL_NOR_BU_NYIS__DASH_KHYIL: char = '࿊';
    /// \u{fcb}: '࿋'
    pub const TIBETAN_SYMBOL_NOR_BU_GSUM__DASH_KHYIL: char = '࿋';
    /// \u{fcc}: '࿌'
    pub const TIBETAN_SYMBOL_NOR_BU_BZHI__DASH_KHYIL: char = '࿌';
    /// \u{fce}: '࿎'
    pub const TIBETAN_SIGN_RDEL_NAG_RDEL_DKAR: char = '࿎';
    /// \u{fcf}: '࿏'
    pub const TIBETAN_SIGN_RDEL_NAG_GSUM: char = '࿏';
    /// \u{fd0}: '࿐'
    pub const TIBETAN_MARK_BSKA_DASH__SHOG_GI_MGO_RGYAN: char = '࿐';
    /// \u{fd1}: '࿑'
    pub const TIBETAN_MARK_MNYAM_YIG_GI_MGO_RGYAN: char = '࿑';
    /// \u{fd2}: '࿒'
    pub const TIBETAN_MARK_NYIS_TSHEG: char = '࿒';
    /// \u{fd3}: '࿓'
    pub const TIBETAN_MARK_INITIAL_BRDA_RNYING_YIG_MGO_MDUN_MA: char = '࿓';
    /// \u{fd4}: '࿔'
    pub const TIBETAN_MARK_CLOSING_BRDA_RNYING_YIG_MGO_SGAB_MA: char = '࿔';
    /// \u{fd5}: '࿕'
    pub const RIGHT_DASH_FACING_SVASTI_SIGN: char = '࿕';
    /// \u{fd6}: '࿖'
    pub const LEFT_DASH_FACING_SVASTI_SIGN: char = '࿖';
    /// \u{fd7}: '࿗'
    pub const RIGHT_DASH_FACING_SVASTI_SIGN_WITH_DOTS: char = '࿗';
    /// \u{fd8}: '࿘'
    pub const LEFT_DASH_FACING_SVASTI_SIGN_WITH_DOTS: char = '࿘';
    /// \u{fd9}: '࿙'
    pub const TIBETAN_MARK_LEADING_MCHAN_RTAGS: char = '࿙';
    /// \u{fda}: '࿚'
    pub const TIBETAN_MARK_TRAILING_MCHAN_RTAGS: char = '࿚';
}

/// \u{f00} → \u{fff}\
///\
/// ༀ ༁ ༂ ༃ ༄ ༅ ༆ ༇ ༈ ༉ ༊ ་ ༌ ། ༎ ༏
/// ༐ ༑ ༒ ༓ ༔ ༕ ༖ ༗ ༘ ༙ ༚ ༛ ༜ ༝ ༞ ༟
/// ༠ ༡ ༢ ༣ ༤ ༥ ༦ ༧ ༨ ༩ ༪ ༫ ༬ ༭ ༮ ༯
/// ༰ ༱ ༲ ༳ ༴ ༵ ༶ ༷ ༸ ༹ ༺ ༻ ༼ ༽ ༾ ༿
/// ཀ ཁ ག གྷ ང ཅ ཆ ཇ ཉ ཊ ཋ ཌ ཌྷ ཎ ཏ ཐ
/// ད དྷ ན པ ཕ བ བྷ མ ཙ ཚ ཛ ཛྷ ཝ ཞ ཟ འ
/// ཡ ར ལ ཤ ཥ ས ཧ ཨ ཀྵ ཪ ཫ ཬ ཱ ི ཱི ུ
/// ཱུ ྲྀ ཷ ླྀ ཹ ེ ཻ ོ ཽ ཾ ཿ ྀ ཱྀ ྂ ྃ ྄
/// ྅ ྆ ྇ ྈ ྉ ྊ ྋ ྌ ྍ ྎ ྏ ྐ ྑ ྒ ྒྷ ྔ
/// ྕ ྖ ྗ ྙ ྚ ྛ ྜ ྜྷ ྞ ྟ ྠ ྡ ྡྷ ྣ ྤ ྥ
/// ྦ ྦྷ ྨ ྩ ྪ ྫ ྫྷ ྭ ྮ ྯ ྰ ྱ ྲ ླ ྴ ྵ
/// ྶ ྷ ྸ ྐྵ ྺ ྻ ྼ ྾ ྿ ࿀ ࿁ ࿂ ࿃ ࿄ ࿅ ࿆
/// ࿇ ࿈ ࿉ ࿊ ࿋ ࿌ ࿎ ࿏ ࿐ ࿑ ࿒ ࿓ ࿔ ࿕ ࿖ ࿗
/// ࿘ ࿙ ࿚
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Tibetan {
    /// \u{f00}: 'ༀ'
    TibetanSyllableOm,
    /// \u{f01}: '༁'
    TibetanMarkGterYigMgoTruncatedA,
    /// \u{f02}: '༂'
    TibetanMarkGterYigMgoDashUmRnamBcadMa,
    /// \u{f03}: '༃'
    TibetanMarkGterYigMgoDashUmGterTshegMa,
    /// \u{f04}: '༄'
    TibetanMarkInitialYigMgoMdunMa,
    /// \u{f05}: '༅'
    TibetanMarkClosingYigMgoSgabMa,
    /// \u{f06}: '༆'
    TibetanMarkCaretYigMgoPhurShadMa,
    /// \u{f07}: '༇'
    TibetanMarkYigMgoTshegShadMa,
    /// \u{f08}: '༈'
    TibetanMarkSbrulShad,
    /// \u{f09}: '༉'
    TibetanMarkBskurYigMgo,
    /// \u{f0a}: '༊'
    TibetanMarkBkaDashShogYigMgo,
    /// \u{f0b}: '་'
    TibetanMarkIntersyllabicTsheg,
    /// \u{f0c}: '༌'
    TibetanMarkDelimiterTshegBstar,
    /// \u{f0d}: '།'
    TibetanMarkShad,
    /// \u{f0e}: '༎'
    TibetanMarkNyisShad,
    /// \u{f0f}: '༏'
    TibetanMarkTshegShad,
    /// \u{f10}: '༐'
    TibetanMarkNyisTshegShad,
    /// \u{f11}: '༑'
    TibetanMarkRinChenSpungsShad,
    /// \u{f12}: '༒'
    TibetanMarkRgyaGramShad,
    /// \u{f13}: '༓'
    TibetanMarkCaretDashDzudRtagsMeLongCan,
    /// \u{f14}: '༔'
    TibetanMarkGterTsheg,
    /// \u{f15}: '༕'
    TibetanLogotypeSignChadRtags,
    /// \u{f16}: '༖'
    TibetanLogotypeSignLhagRtags,
    /// \u{f17}: '༗'
    TibetanAstrologicalSignSgraGcanDashCharRtags,
    /// \u{f18}: '༘'
    TibetanAstrologicalSignDashKhyudPa,
    /// \u{f19}: '༙'
    TibetanAstrologicalSignSdongTshugs,
    /// \u{f1a}: '༚'
    TibetanSignRdelDkarGcig,
    /// \u{f1b}: '༛'
    TibetanSignRdelDkarGnyis,
    /// \u{f1c}: '༜'
    TibetanSignRdelDkarGsum,
    /// \u{f1d}: '༝'
    TibetanSignRdelNagGcig,
    /// \u{f1e}: '༞'
    TibetanSignRdelNagGnyis,
    /// \u{f1f}: '༟'
    TibetanSignRdelDkarRdelNag,
    /// \u{f20}: '༠'
    TibetanDigitZero,
    /// \u{f21}: '༡'
    TibetanDigitOne,
    /// \u{f22}: '༢'
    TibetanDigitTwo,
    /// \u{f23}: '༣'
    TibetanDigitThree,
    /// \u{f24}: '༤'
    TibetanDigitFour,
    /// \u{f25}: '༥'
    TibetanDigitFive,
    /// \u{f26}: '༦'
    TibetanDigitSix,
    /// \u{f27}: '༧'
    TibetanDigitSeven,
    /// \u{f28}: '༨'
    TibetanDigitEight,
    /// \u{f29}: '༩'
    TibetanDigitNine,
    /// \u{f2a}: '༪'
    TibetanDigitHalfOne,
    /// \u{f2b}: '༫'
    TibetanDigitHalfTwo,
    /// \u{f2c}: '༬'
    TibetanDigitHalfThree,
    /// \u{f2d}: '༭'
    TibetanDigitHalfFour,
    /// \u{f2e}: '༮'
    TibetanDigitHalfFive,
    /// \u{f2f}: '༯'
    TibetanDigitHalfSix,
    /// \u{f30}: '༰'
    TibetanDigitHalfSeven,
    /// \u{f31}: '༱'
    TibetanDigitHalfEight,
    /// \u{f32}: '༲'
    TibetanDigitHalfNine,
    /// \u{f33}: '༳'
    TibetanDigitHalfZero,
    /// \u{f34}: '༴'
    TibetanMarkBsdusRtags,
    /// \u{f35}: '༵'
    TibetanMarkNgasBzungNyiZla,
    /// \u{f36}: '༶'
    TibetanMarkCaretDashDzudRtagsBzhiMigCan,
    /// \u{f37}: '༷'
    TibetanMarkNgasBzungSgorRtags,
    /// \u{f38}: '༸'
    TibetanMarkCheMgo,
    /// \u{f39}: '༹'
    TibetanMarkTsaDashPhru,
    /// \u{f3a}: '༺'
    TibetanMarkGugRtagsGyon,
    /// \u{f3b}: '༻'
    TibetanMarkGugRtagsGyas,
    /// \u{f3c}: '༼'
    TibetanMarkAngKhangGyon,
    /// \u{f3d}: '༽'
    TibetanMarkAngKhangGyas,
    /// \u{f3e}: '༾'
    TibetanSignYarTshes,
    /// \u{f3f}: '༿'
    TibetanSignMarTshes,
    /// \u{f40}: 'ཀ'
    TibetanLetterKa,
    /// \u{f41}: 'ཁ'
    TibetanLetterKha,
    /// \u{f42}: 'ག'
    TibetanLetterGa,
    /// \u{f43}: 'གྷ'
    TibetanLetterGha,
    /// \u{f44}: 'ང'
    TibetanLetterNga,
    /// \u{f45}: 'ཅ'
    TibetanLetterCa,
    /// \u{f46}: 'ཆ'
    TibetanLetterCha,
    /// \u{f47}: 'ཇ'
    TibetanLetterJa,
    /// \u{f49}: 'ཉ'
    TibetanLetterNya,
    /// \u{f4a}: 'ཊ'
    TibetanLetterTta,
    /// \u{f4b}: 'ཋ'
    TibetanLetterTtha,
    /// \u{f4c}: 'ཌ'
    TibetanLetterDda,
    /// \u{f4d}: 'ཌྷ'
    TibetanLetterDdha,
    /// \u{f4e}: 'ཎ'
    TibetanLetterNna,
    /// \u{f4f}: 'ཏ'
    TibetanLetterTa,
    /// \u{f50}: 'ཐ'
    TibetanLetterTha,
    /// \u{f51}: 'ད'
    TibetanLetterDa,
    /// \u{f52}: 'དྷ'
    TibetanLetterDha,
    /// \u{f53}: 'ན'
    TibetanLetterNa,
    /// \u{f54}: 'པ'
    TibetanLetterPa,
    /// \u{f55}: 'ཕ'
    TibetanLetterPha,
    /// \u{f56}: 'བ'
    TibetanLetterBa,
    /// \u{f57}: 'བྷ'
    TibetanLetterBha,
    /// \u{f58}: 'མ'
    TibetanLetterMa,
    /// \u{f59}: 'ཙ'
    TibetanLetterTsa,
    /// \u{f5a}: 'ཚ'
    TibetanLetterTsha,
    /// \u{f5b}: 'ཛ'
    TibetanLetterDza,
    /// \u{f5c}: 'ཛྷ'
    TibetanLetterDzha,
    /// \u{f5d}: 'ཝ'
    TibetanLetterWa,
    /// \u{f5e}: 'ཞ'
    TibetanLetterZha,
    /// \u{f5f}: 'ཟ'
    TibetanLetterZa,
    /// \u{f60}: 'འ'
    TibetanLetterDashA,
    /// \u{f61}: 'ཡ'
    TibetanLetterYa,
    /// \u{f62}: 'ར'
    TibetanLetterRa,
    /// \u{f63}: 'ལ'
    TibetanLetterLa,
    /// \u{f64}: 'ཤ'
    TibetanLetterSha,
    /// \u{f65}: 'ཥ'
    TibetanLetterSsa,
    /// \u{f66}: 'ས'
    TibetanLetterSa,
    /// \u{f67}: 'ཧ'
    TibetanLetterHa,
    /// \u{f68}: 'ཨ'
    TibetanLetterA,
    /// \u{f69}: 'ཀྵ'
    TibetanLetterKssa,
    /// \u{f6a}: 'ཪ'
    TibetanLetterFixedDashFormRa,
    /// \u{f6b}: 'ཫ'
    TibetanLetterKka,
    /// \u{f6c}: 'ཬ'
    TibetanLetterRra,
    /// \u{f71}: 'ཱ'
    TibetanVowelSignAa,
    /// \u{f72}: 'ི'
    TibetanVowelSignI,
    /// \u{f73}: 'ཱི'
    TibetanVowelSignIi,
    /// \u{f74}: 'ུ'
    TibetanVowelSignU,
    /// \u{f75}: 'ཱུ'
    TibetanVowelSignUu,
    /// \u{f76}: 'ྲྀ'
    TibetanVowelSignVocalicR,
    /// \u{f77}: 'ཷ'
    TibetanVowelSignVocalicRr,
    /// \u{f78}: 'ླྀ'
    TibetanVowelSignVocalicL,
    /// \u{f79}: 'ཹ'
    TibetanVowelSignVocalicLl,
    /// \u{f7a}: 'ེ'
    TibetanVowelSignE,
    /// \u{f7b}: 'ཻ'
    TibetanVowelSignEe,
    /// \u{f7c}: 'ོ'
    TibetanVowelSignO,
    /// \u{f7d}: 'ཽ'
    TibetanVowelSignOo,
    /// \u{f7e}: 'ཾ'
    TibetanSignRjesSuNgaRo,
    /// \u{f7f}: 'ཿ'
    TibetanSignRnamBcad,
    /// \u{f80}: 'ྀ'
    TibetanVowelSignReversedI,
    /// \u{f81}: 'ཱྀ'
    TibetanVowelSignReversedIi,
    /// \u{f82}: 'ྂ'
    TibetanSignNyiZlaNaaDa,
    /// \u{f83}: 'ྃ'
    TibetanSignSnaLdan,
    /// \u{f84}: '྄'
    TibetanMarkHalanta,
    /// \u{f85}: '྅'
    TibetanMarkPaluta,
    /// \u{f86}: '྆'
    TibetanSignLciRtags,
    /// \u{f87}: '྇'
    TibetanSignYangRtags,
    /// \u{f88}: 'ྈ'
    TibetanSignLceTsaCan,
    /// \u{f89}: 'ྉ'
    TibetanSignMchuCan,
    /// \u{f8a}: 'ྊ'
    TibetanSignGruCanRgyings,
    /// \u{f8b}: 'ྋ'
    TibetanSignGruMedRgyings,
    /// \u{f8c}: 'ྌ'
    TibetanSignInvertedMchuCan,
    /// \u{f8d}: 'ྍ'
    TibetanSubjoinedSignLceTsaCan,
    /// \u{f8e}: 'ྎ'
    TibetanSubjoinedSignMchuCan,
    /// \u{f8f}: 'ྏ'
    TibetanSubjoinedSignInvertedMchuCan,
    /// \u{f90}: 'ྐ'
    TibetanSubjoinedLetterKa,
    /// \u{f91}: 'ྑ'
    TibetanSubjoinedLetterKha,
    /// \u{f92}: 'ྒ'
    TibetanSubjoinedLetterGa,
    /// \u{f93}: 'ྒྷ'
    TibetanSubjoinedLetterGha,
    /// \u{f94}: 'ྔ'
    TibetanSubjoinedLetterNga,
    /// \u{f95}: 'ྕ'
    TibetanSubjoinedLetterCa,
    /// \u{f96}: 'ྖ'
    TibetanSubjoinedLetterCha,
    /// \u{f97}: 'ྗ'
    TibetanSubjoinedLetterJa,
    /// \u{f99}: 'ྙ'
    TibetanSubjoinedLetterNya,
    /// \u{f9a}: 'ྚ'
    TibetanSubjoinedLetterTta,
    /// \u{f9b}: 'ྛ'
    TibetanSubjoinedLetterTtha,
    /// \u{f9c}: 'ྜ'
    TibetanSubjoinedLetterDda,
    /// \u{f9d}: 'ྜྷ'
    TibetanSubjoinedLetterDdha,
    /// \u{f9e}: 'ྞ'
    TibetanSubjoinedLetterNna,
    /// \u{f9f}: 'ྟ'
    TibetanSubjoinedLetterTa,
    /// \u{fa0}: 'ྠ'
    TibetanSubjoinedLetterTha,
    /// \u{fa1}: 'ྡ'
    TibetanSubjoinedLetterDa,
    /// \u{fa2}: 'ྡྷ'
    TibetanSubjoinedLetterDha,
    /// \u{fa3}: 'ྣ'
    TibetanSubjoinedLetterNa,
    /// \u{fa4}: 'ྤ'
    TibetanSubjoinedLetterPa,
    /// \u{fa5}: 'ྥ'
    TibetanSubjoinedLetterPha,
    /// \u{fa6}: 'ྦ'
    TibetanSubjoinedLetterBa,
    /// \u{fa7}: 'ྦྷ'
    TibetanSubjoinedLetterBha,
    /// \u{fa8}: 'ྨ'
    TibetanSubjoinedLetterMa,
    /// \u{fa9}: 'ྩ'
    TibetanSubjoinedLetterTsa,
    /// \u{faa}: 'ྪ'
    TibetanSubjoinedLetterTsha,
    /// \u{fab}: 'ྫ'
    TibetanSubjoinedLetterDza,
    /// \u{fac}: 'ྫྷ'
    TibetanSubjoinedLetterDzha,
    /// \u{fad}: 'ྭ'
    TibetanSubjoinedLetterWa,
    /// \u{fae}: 'ྮ'
    TibetanSubjoinedLetterZha,
    /// \u{faf}: 'ྯ'
    TibetanSubjoinedLetterZa,
    /// \u{fb0}: 'ྰ'
    TibetanSubjoinedLetterDashA,
    /// \u{fb1}: 'ྱ'
    TibetanSubjoinedLetterYa,
    /// \u{fb2}: 'ྲ'
    TibetanSubjoinedLetterRa,
    /// \u{fb3}: 'ླ'
    TibetanSubjoinedLetterLa,
    /// \u{fb4}: 'ྴ'
    TibetanSubjoinedLetterSha,
    /// \u{fb5}: 'ྵ'
    TibetanSubjoinedLetterSsa,
    /// \u{fb6}: 'ྶ'
    TibetanSubjoinedLetterSa,
    /// \u{fb7}: 'ྷ'
    TibetanSubjoinedLetterHa,
    /// \u{fb8}: 'ྸ'
    TibetanSubjoinedLetterA,
    /// \u{fb9}: 'ྐྵ'
    TibetanSubjoinedLetterKssa,
    /// \u{fba}: 'ྺ'
    TibetanSubjoinedLetterFixedDashFormWa,
    /// \u{fbb}: 'ྻ'
    TibetanSubjoinedLetterFixedDashFormYa,
    /// \u{fbc}: 'ྼ'
    TibetanSubjoinedLetterFixedDashFormRa,
    /// \u{fbe}: '྾'
    TibetanKuRuKha,
    /// \u{fbf}: '྿'
    TibetanKuRuKhaBzhiMigCan,
    /// \u{fc0}: '࿀'
    TibetanCantillationSignHeavyBeat,
    /// \u{fc1}: '࿁'
    TibetanCantillationSignLightBeat,
    /// \u{fc2}: '࿂'
    TibetanCantillationSignCangTeDashU,
    /// \u{fc3}: '࿃'
    TibetanCantillationSignSbubDashChal,
    /// \u{fc4}: '࿄'
    TibetanSymbolDrilBu,
    /// \u{fc5}: '࿅'
    TibetanSymbolRdoRje,
    /// \u{fc6}: '࿆'
    TibetanSymbolPadmaGdan,
    /// \u{fc7}: '࿇'
    TibetanSymbolRdoRjeRgyaGram,
    /// \u{fc8}: '࿈'
    TibetanSymbolPhurPa,
    /// \u{fc9}: '࿉'
    TibetanSymbolNorBu,
    /// \u{fca}: '࿊'
    TibetanSymbolNorBuNyisDashKhyil,
    /// \u{fcb}: '࿋'
    TibetanSymbolNorBuGsumDashKhyil,
    /// \u{fcc}: '࿌'
    TibetanSymbolNorBuBzhiDashKhyil,
    /// \u{fce}: '࿎'
    TibetanSignRdelNagRdelDkar,
    /// \u{fcf}: '࿏'
    TibetanSignRdelNagGsum,
    /// \u{fd0}: '࿐'
    TibetanMarkBskaDashShogGiMgoRgyan,
    /// \u{fd1}: '࿑'
    TibetanMarkMnyamYigGiMgoRgyan,
    /// \u{fd2}: '࿒'
    TibetanMarkNyisTsheg,
    /// \u{fd3}: '࿓'
    TibetanMarkInitialBrdaRnyingYigMgoMdunMa,
    /// \u{fd4}: '࿔'
    TibetanMarkClosingBrdaRnyingYigMgoSgabMa,
    /// \u{fd5}: '࿕'
    RightDashFacingSvastiSign,
    /// \u{fd6}: '࿖'
    LeftDashFacingSvastiSign,
    /// \u{fd7}: '࿗'
    RightDashFacingSvastiSignWithDots,
    /// \u{fd8}: '࿘'
    LeftDashFacingSvastiSignWithDots,
    /// \u{fd9}: '࿙'
    TibetanMarkLeadingMchanRtags,
    /// \u{fda}: '࿚'
    TibetanMarkTrailingMchanRtags,
}

impl Into<char> for Tibetan {
    fn into(self) -> char {
        use constants::*;
        match self {
            Tibetan::TibetanSyllableOm => TIBETAN_SYLLABLE_OM,
            Tibetan::TibetanMarkGterYigMgoTruncatedA => TIBETAN_MARK_GTER_YIG_MGO_TRUNCATED_A,
            Tibetan::TibetanMarkGterYigMgoDashUmRnamBcadMa => TIBETAN_MARK_GTER_YIG_MGO__DASH_UM_RNAM_BCAD_MA,
            Tibetan::TibetanMarkGterYigMgoDashUmGterTshegMa => TIBETAN_MARK_GTER_YIG_MGO__DASH_UM_GTER_TSHEG_MA,
            Tibetan::TibetanMarkInitialYigMgoMdunMa => TIBETAN_MARK_INITIAL_YIG_MGO_MDUN_MA,
            Tibetan::TibetanMarkClosingYigMgoSgabMa => TIBETAN_MARK_CLOSING_YIG_MGO_SGAB_MA,
            Tibetan::TibetanMarkCaretYigMgoPhurShadMa => TIBETAN_MARK_CARET_YIG_MGO_PHUR_SHAD_MA,
            Tibetan::TibetanMarkYigMgoTshegShadMa => TIBETAN_MARK_YIG_MGO_TSHEG_SHAD_MA,
            Tibetan::TibetanMarkSbrulShad => TIBETAN_MARK_SBRUL_SHAD,
            Tibetan::TibetanMarkBskurYigMgo => TIBETAN_MARK_BSKUR_YIG_MGO,
            Tibetan::TibetanMarkBkaDashShogYigMgo => TIBETAN_MARK_BKA_DASH__SHOG_YIG_MGO,
            Tibetan::TibetanMarkIntersyllabicTsheg => TIBETAN_MARK_INTERSYLLABIC_TSHEG,
            Tibetan::TibetanMarkDelimiterTshegBstar => TIBETAN_MARK_DELIMITER_TSHEG_BSTAR,
            Tibetan::TibetanMarkShad => TIBETAN_MARK_SHAD,
            Tibetan::TibetanMarkNyisShad => TIBETAN_MARK_NYIS_SHAD,
            Tibetan::TibetanMarkTshegShad => TIBETAN_MARK_TSHEG_SHAD,
            Tibetan::TibetanMarkNyisTshegShad => TIBETAN_MARK_NYIS_TSHEG_SHAD,
            Tibetan::TibetanMarkRinChenSpungsShad => TIBETAN_MARK_RIN_CHEN_SPUNGS_SHAD,
            Tibetan::TibetanMarkRgyaGramShad => TIBETAN_MARK_RGYA_GRAM_SHAD,
            Tibetan::TibetanMarkCaretDashDzudRtagsMeLongCan => TIBETAN_MARK_CARET__DASH_DZUD_RTAGS_ME_LONG_CAN,
            Tibetan::TibetanMarkGterTsheg => TIBETAN_MARK_GTER_TSHEG,
            Tibetan::TibetanLogotypeSignChadRtags => TIBETAN_LOGOTYPE_SIGN_CHAD_RTAGS,
            Tibetan::TibetanLogotypeSignLhagRtags => TIBETAN_LOGOTYPE_SIGN_LHAG_RTAGS,
            Tibetan::TibetanAstrologicalSignSgraGcanDashCharRtags => TIBETAN_ASTROLOGICAL_SIGN_SGRA_GCAN__DASH_CHAR_RTAGS,
            Tibetan::TibetanAstrologicalSignDashKhyudPa => TIBETAN_ASTROLOGICAL_SIGN__DASH_KHYUD_PA,
            Tibetan::TibetanAstrologicalSignSdongTshugs => TIBETAN_ASTROLOGICAL_SIGN_SDONG_TSHUGS,
            Tibetan::TibetanSignRdelDkarGcig => TIBETAN_SIGN_RDEL_DKAR_GCIG,
            Tibetan::TibetanSignRdelDkarGnyis => TIBETAN_SIGN_RDEL_DKAR_GNYIS,
            Tibetan::TibetanSignRdelDkarGsum => TIBETAN_SIGN_RDEL_DKAR_GSUM,
            Tibetan::TibetanSignRdelNagGcig => TIBETAN_SIGN_RDEL_NAG_GCIG,
            Tibetan::TibetanSignRdelNagGnyis => TIBETAN_SIGN_RDEL_NAG_GNYIS,
            Tibetan::TibetanSignRdelDkarRdelNag => TIBETAN_SIGN_RDEL_DKAR_RDEL_NAG,
            Tibetan::TibetanDigitZero => TIBETAN_DIGIT_ZERO,
            Tibetan::TibetanDigitOne => TIBETAN_DIGIT_ONE,
            Tibetan::TibetanDigitTwo => TIBETAN_DIGIT_TWO,
            Tibetan::TibetanDigitThree => TIBETAN_DIGIT_THREE,
            Tibetan::TibetanDigitFour => TIBETAN_DIGIT_FOUR,
            Tibetan::TibetanDigitFive => TIBETAN_DIGIT_FIVE,
            Tibetan::TibetanDigitSix => TIBETAN_DIGIT_SIX,
            Tibetan::TibetanDigitSeven => TIBETAN_DIGIT_SEVEN,
            Tibetan::TibetanDigitEight => TIBETAN_DIGIT_EIGHT,
            Tibetan::TibetanDigitNine => TIBETAN_DIGIT_NINE,
            Tibetan::TibetanDigitHalfOne => TIBETAN_DIGIT_HALF_ONE,
            Tibetan::TibetanDigitHalfTwo => TIBETAN_DIGIT_HALF_TWO,
            Tibetan::TibetanDigitHalfThree => TIBETAN_DIGIT_HALF_THREE,
            Tibetan::TibetanDigitHalfFour => TIBETAN_DIGIT_HALF_FOUR,
            Tibetan::TibetanDigitHalfFive => TIBETAN_DIGIT_HALF_FIVE,
            Tibetan::TibetanDigitHalfSix => TIBETAN_DIGIT_HALF_SIX,
            Tibetan::TibetanDigitHalfSeven => TIBETAN_DIGIT_HALF_SEVEN,
            Tibetan::TibetanDigitHalfEight => TIBETAN_DIGIT_HALF_EIGHT,
            Tibetan::TibetanDigitHalfNine => TIBETAN_DIGIT_HALF_NINE,
            Tibetan::TibetanDigitHalfZero => TIBETAN_DIGIT_HALF_ZERO,
            Tibetan::TibetanMarkBsdusRtags => TIBETAN_MARK_BSDUS_RTAGS,
            Tibetan::TibetanMarkNgasBzungNyiZla => TIBETAN_MARK_NGAS_BZUNG_NYI_ZLA,
            Tibetan::TibetanMarkCaretDashDzudRtagsBzhiMigCan => TIBETAN_MARK_CARET__DASH_DZUD_RTAGS_BZHI_MIG_CAN,
            Tibetan::TibetanMarkNgasBzungSgorRtags => TIBETAN_MARK_NGAS_BZUNG_SGOR_RTAGS,
            Tibetan::TibetanMarkCheMgo => TIBETAN_MARK_CHE_MGO,
            Tibetan::TibetanMarkTsaDashPhru => TIBETAN_MARK_TSA__DASH_PHRU,
            Tibetan::TibetanMarkGugRtagsGyon => TIBETAN_MARK_GUG_RTAGS_GYON,
            Tibetan::TibetanMarkGugRtagsGyas => TIBETAN_MARK_GUG_RTAGS_GYAS,
            Tibetan::TibetanMarkAngKhangGyon => TIBETAN_MARK_ANG_KHANG_GYON,
            Tibetan::TibetanMarkAngKhangGyas => TIBETAN_MARK_ANG_KHANG_GYAS,
            Tibetan::TibetanSignYarTshes => TIBETAN_SIGN_YAR_TSHES,
            Tibetan::TibetanSignMarTshes => TIBETAN_SIGN_MAR_TSHES,
            Tibetan::TibetanLetterKa => TIBETAN_LETTER_KA,
            Tibetan::TibetanLetterKha => TIBETAN_LETTER_KHA,
            Tibetan::TibetanLetterGa => TIBETAN_LETTER_GA,
            Tibetan::TibetanLetterGha => TIBETAN_LETTER_GHA,
            Tibetan::TibetanLetterNga => TIBETAN_LETTER_NGA,
            Tibetan::TibetanLetterCa => TIBETAN_LETTER_CA,
            Tibetan::TibetanLetterCha => TIBETAN_LETTER_CHA,
            Tibetan::TibetanLetterJa => TIBETAN_LETTER_JA,
            Tibetan::TibetanLetterNya => TIBETAN_LETTER_NYA,
            Tibetan::TibetanLetterTta => TIBETAN_LETTER_TTA,
            Tibetan::TibetanLetterTtha => TIBETAN_LETTER_TTHA,
            Tibetan::TibetanLetterDda => TIBETAN_LETTER_DDA,
            Tibetan::TibetanLetterDdha => TIBETAN_LETTER_DDHA,
            Tibetan::TibetanLetterNna => TIBETAN_LETTER_NNA,
            Tibetan::TibetanLetterTa => TIBETAN_LETTER_TA,
            Tibetan::TibetanLetterTha => TIBETAN_LETTER_THA,
            Tibetan::TibetanLetterDa => TIBETAN_LETTER_DA,
            Tibetan::TibetanLetterDha => TIBETAN_LETTER_DHA,
            Tibetan::TibetanLetterNa => TIBETAN_LETTER_NA,
            Tibetan::TibetanLetterPa => TIBETAN_LETTER_PA,
            Tibetan::TibetanLetterPha => TIBETAN_LETTER_PHA,
            Tibetan::TibetanLetterBa => TIBETAN_LETTER_BA,
            Tibetan::TibetanLetterBha => TIBETAN_LETTER_BHA,
            Tibetan::TibetanLetterMa => TIBETAN_LETTER_MA,
            Tibetan::TibetanLetterTsa => TIBETAN_LETTER_TSA,
            Tibetan::TibetanLetterTsha => TIBETAN_LETTER_TSHA,
            Tibetan::TibetanLetterDza => TIBETAN_LETTER_DZA,
            Tibetan::TibetanLetterDzha => TIBETAN_LETTER_DZHA,
            Tibetan::TibetanLetterWa => TIBETAN_LETTER_WA,
            Tibetan::TibetanLetterZha => TIBETAN_LETTER_ZHA,
            Tibetan::TibetanLetterZa => TIBETAN_LETTER_ZA,
            Tibetan::TibetanLetterDashA => TIBETAN_LETTER__DASH_A,
            Tibetan::TibetanLetterYa => TIBETAN_LETTER_YA,
            Tibetan::TibetanLetterRa => TIBETAN_LETTER_RA,
            Tibetan::TibetanLetterLa => TIBETAN_LETTER_LA,
            Tibetan::TibetanLetterSha => TIBETAN_LETTER_SHA,
            Tibetan::TibetanLetterSsa => TIBETAN_LETTER_SSA,
            Tibetan::TibetanLetterSa => TIBETAN_LETTER_SA,
            Tibetan::TibetanLetterHa => TIBETAN_LETTER_HA,
            Tibetan::TibetanLetterA => TIBETAN_LETTER_A,
            Tibetan::TibetanLetterKssa => TIBETAN_LETTER_KSSA,
            Tibetan::TibetanLetterFixedDashFormRa => TIBETAN_LETTER_FIXED_DASH_FORM_RA,
            Tibetan::TibetanLetterKka => TIBETAN_LETTER_KKA,
            Tibetan::TibetanLetterRra => TIBETAN_LETTER_RRA,
            Tibetan::TibetanVowelSignAa => TIBETAN_VOWEL_SIGN_AA,
            Tibetan::TibetanVowelSignI => TIBETAN_VOWEL_SIGN_I,
            Tibetan::TibetanVowelSignIi => TIBETAN_VOWEL_SIGN_II,
            Tibetan::TibetanVowelSignU => TIBETAN_VOWEL_SIGN_U,
            Tibetan::TibetanVowelSignUu => TIBETAN_VOWEL_SIGN_UU,
            Tibetan::TibetanVowelSignVocalicR => TIBETAN_VOWEL_SIGN_VOCALIC_R,
            Tibetan::TibetanVowelSignVocalicRr => TIBETAN_VOWEL_SIGN_VOCALIC_RR,
            Tibetan::TibetanVowelSignVocalicL => TIBETAN_VOWEL_SIGN_VOCALIC_L,
            Tibetan::TibetanVowelSignVocalicLl => TIBETAN_VOWEL_SIGN_VOCALIC_LL,
            Tibetan::TibetanVowelSignE => TIBETAN_VOWEL_SIGN_E,
            Tibetan::TibetanVowelSignEe => TIBETAN_VOWEL_SIGN_EE,
            Tibetan::TibetanVowelSignO => TIBETAN_VOWEL_SIGN_O,
            Tibetan::TibetanVowelSignOo => TIBETAN_VOWEL_SIGN_OO,
            Tibetan::TibetanSignRjesSuNgaRo => TIBETAN_SIGN_RJES_SU_NGA_RO,
            Tibetan::TibetanSignRnamBcad => TIBETAN_SIGN_RNAM_BCAD,
            Tibetan::TibetanVowelSignReversedI => TIBETAN_VOWEL_SIGN_REVERSED_I,
            Tibetan::TibetanVowelSignReversedIi => TIBETAN_VOWEL_SIGN_REVERSED_II,
            Tibetan::TibetanSignNyiZlaNaaDa => TIBETAN_SIGN_NYI_ZLA_NAA_DA,
            Tibetan::TibetanSignSnaLdan => TIBETAN_SIGN_SNA_LDAN,
            Tibetan::TibetanMarkHalanta => TIBETAN_MARK_HALANTA,
            Tibetan::TibetanMarkPaluta => TIBETAN_MARK_PALUTA,
            Tibetan::TibetanSignLciRtags => TIBETAN_SIGN_LCI_RTAGS,
            Tibetan::TibetanSignYangRtags => TIBETAN_SIGN_YANG_RTAGS,
            Tibetan::TibetanSignLceTsaCan => TIBETAN_SIGN_LCE_TSA_CAN,
            Tibetan::TibetanSignMchuCan => TIBETAN_SIGN_MCHU_CAN,
            Tibetan::TibetanSignGruCanRgyings => TIBETAN_SIGN_GRU_CAN_RGYINGS,
            Tibetan::TibetanSignGruMedRgyings => TIBETAN_SIGN_GRU_MED_RGYINGS,
            Tibetan::TibetanSignInvertedMchuCan => TIBETAN_SIGN_INVERTED_MCHU_CAN,
            Tibetan::TibetanSubjoinedSignLceTsaCan => TIBETAN_SUBJOINED_SIGN_LCE_TSA_CAN,
            Tibetan::TibetanSubjoinedSignMchuCan => TIBETAN_SUBJOINED_SIGN_MCHU_CAN,
            Tibetan::TibetanSubjoinedSignInvertedMchuCan => TIBETAN_SUBJOINED_SIGN_INVERTED_MCHU_CAN,
            Tibetan::TibetanSubjoinedLetterKa => TIBETAN_SUBJOINED_LETTER_KA,
            Tibetan::TibetanSubjoinedLetterKha => TIBETAN_SUBJOINED_LETTER_KHA,
            Tibetan::TibetanSubjoinedLetterGa => TIBETAN_SUBJOINED_LETTER_GA,
            Tibetan::TibetanSubjoinedLetterGha => TIBETAN_SUBJOINED_LETTER_GHA,
            Tibetan::TibetanSubjoinedLetterNga => TIBETAN_SUBJOINED_LETTER_NGA,
            Tibetan::TibetanSubjoinedLetterCa => TIBETAN_SUBJOINED_LETTER_CA,
            Tibetan::TibetanSubjoinedLetterCha => TIBETAN_SUBJOINED_LETTER_CHA,
            Tibetan::TibetanSubjoinedLetterJa => TIBETAN_SUBJOINED_LETTER_JA,
            Tibetan::TibetanSubjoinedLetterNya => TIBETAN_SUBJOINED_LETTER_NYA,
            Tibetan::TibetanSubjoinedLetterTta => TIBETAN_SUBJOINED_LETTER_TTA,
            Tibetan::TibetanSubjoinedLetterTtha => TIBETAN_SUBJOINED_LETTER_TTHA,
            Tibetan::TibetanSubjoinedLetterDda => TIBETAN_SUBJOINED_LETTER_DDA,
            Tibetan::TibetanSubjoinedLetterDdha => TIBETAN_SUBJOINED_LETTER_DDHA,
            Tibetan::TibetanSubjoinedLetterNna => TIBETAN_SUBJOINED_LETTER_NNA,
            Tibetan::TibetanSubjoinedLetterTa => TIBETAN_SUBJOINED_LETTER_TA,
            Tibetan::TibetanSubjoinedLetterTha => TIBETAN_SUBJOINED_LETTER_THA,
            Tibetan::TibetanSubjoinedLetterDa => TIBETAN_SUBJOINED_LETTER_DA,
            Tibetan::TibetanSubjoinedLetterDha => TIBETAN_SUBJOINED_LETTER_DHA,
            Tibetan::TibetanSubjoinedLetterNa => TIBETAN_SUBJOINED_LETTER_NA,
            Tibetan::TibetanSubjoinedLetterPa => TIBETAN_SUBJOINED_LETTER_PA,
            Tibetan::TibetanSubjoinedLetterPha => TIBETAN_SUBJOINED_LETTER_PHA,
            Tibetan::TibetanSubjoinedLetterBa => TIBETAN_SUBJOINED_LETTER_BA,
            Tibetan::TibetanSubjoinedLetterBha => TIBETAN_SUBJOINED_LETTER_BHA,
            Tibetan::TibetanSubjoinedLetterMa => TIBETAN_SUBJOINED_LETTER_MA,
            Tibetan::TibetanSubjoinedLetterTsa => TIBETAN_SUBJOINED_LETTER_TSA,
            Tibetan::TibetanSubjoinedLetterTsha => TIBETAN_SUBJOINED_LETTER_TSHA,
            Tibetan::TibetanSubjoinedLetterDza => TIBETAN_SUBJOINED_LETTER_DZA,
            Tibetan::TibetanSubjoinedLetterDzha => TIBETAN_SUBJOINED_LETTER_DZHA,
            Tibetan::TibetanSubjoinedLetterWa => TIBETAN_SUBJOINED_LETTER_WA,
            Tibetan::TibetanSubjoinedLetterZha => TIBETAN_SUBJOINED_LETTER_ZHA,
            Tibetan::TibetanSubjoinedLetterZa => TIBETAN_SUBJOINED_LETTER_ZA,
            Tibetan::TibetanSubjoinedLetterDashA => TIBETAN_SUBJOINED_LETTER__DASH_A,
            Tibetan::TibetanSubjoinedLetterYa => TIBETAN_SUBJOINED_LETTER_YA,
            Tibetan::TibetanSubjoinedLetterRa => TIBETAN_SUBJOINED_LETTER_RA,
            Tibetan::TibetanSubjoinedLetterLa => TIBETAN_SUBJOINED_LETTER_LA,
            Tibetan::TibetanSubjoinedLetterSha => TIBETAN_SUBJOINED_LETTER_SHA,
            Tibetan::TibetanSubjoinedLetterSsa => TIBETAN_SUBJOINED_LETTER_SSA,
            Tibetan::TibetanSubjoinedLetterSa => TIBETAN_SUBJOINED_LETTER_SA,
            Tibetan::TibetanSubjoinedLetterHa => TIBETAN_SUBJOINED_LETTER_HA,
            Tibetan::TibetanSubjoinedLetterA => TIBETAN_SUBJOINED_LETTER_A,
            Tibetan::TibetanSubjoinedLetterKssa => TIBETAN_SUBJOINED_LETTER_KSSA,
            Tibetan::TibetanSubjoinedLetterFixedDashFormWa => TIBETAN_SUBJOINED_LETTER_FIXED_DASH_FORM_WA,
            Tibetan::TibetanSubjoinedLetterFixedDashFormYa => TIBETAN_SUBJOINED_LETTER_FIXED_DASH_FORM_YA,
            Tibetan::TibetanSubjoinedLetterFixedDashFormRa => TIBETAN_SUBJOINED_LETTER_FIXED_DASH_FORM_RA,
            Tibetan::TibetanKuRuKha => TIBETAN_KU_RU_KHA,
            Tibetan::TibetanKuRuKhaBzhiMigCan => TIBETAN_KU_RU_KHA_BZHI_MIG_CAN,
            Tibetan::TibetanCantillationSignHeavyBeat => TIBETAN_CANTILLATION_SIGN_HEAVY_BEAT,
            Tibetan::TibetanCantillationSignLightBeat => TIBETAN_CANTILLATION_SIGN_LIGHT_BEAT,
            Tibetan::TibetanCantillationSignCangTeDashU => TIBETAN_CANTILLATION_SIGN_CANG_TE_DASH_U,
            Tibetan::TibetanCantillationSignSbubDashChal => TIBETAN_CANTILLATION_SIGN_SBUB__DASH_CHAL,
            Tibetan::TibetanSymbolDrilBu => TIBETAN_SYMBOL_DRIL_BU,
            Tibetan::TibetanSymbolRdoRje => TIBETAN_SYMBOL_RDO_RJE,
            Tibetan::TibetanSymbolPadmaGdan => TIBETAN_SYMBOL_PADMA_GDAN,
            Tibetan::TibetanSymbolRdoRjeRgyaGram => TIBETAN_SYMBOL_RDO_RJE_RGYA_GRAM,
            Tibetan::TibetanSymbolPhurPa => TIBETAN_SYMBOL_PHUR_PA,
            Tibetan::TibetanSymbolNorBu => TIBETAN_SYMBOL_NOR_BU,
            Tibetan::TibetanSymbolNorBuNyisDashKhyil => TIBETAN_SYMBOL_NOR_BU_NYIS__DASH_KHYIL,
            Tibetan::TibetanSymbolNorBuGsumDashKhyil => TIBETAN_SYMBOL_NOR_BU_GSUM__DASH_KHYIL,
            Tibetan::TibetanSymbolNorBuBzhiDashKhyil => TIBETAN_SYMBOL_NOR_BU_BZHI__DASH_KHYIL,
            Tibetan::TibetanSignRdelNagRdelDkar => TIBETAN_SIGN_RDEL_NAG_RDEL_DKAR,
            Tibetan::TibetanSignRdelNagGsum => TIBETAN_SIGN_RDEL_NAG_GSUM,
            Tibetan::TibetanMarkBskaDashShogGiMgoRgyan => TIBETAN_MARK_BSKA_DASH__SHOG_GI_MGO_RGYAN,
            Tibetan::TibetanMarkMnyamYigGiMgoRgyan => TIBETAN_MARK_MNYAM_YIG_GI_MGO_RGYAN,
            Tibetan::TibetanMarkNyisTsheg => TIBETAN_MARK_NYIS_TSHEG,
            Tibetan::TibetanMarkInitialBrdaRnyingYigMgoMdunMa => TIBETAN_MARK_INITIAL_BRDA_RNYING_YIG_MGO_MDUN_MA,
            Tibetan::TibetanMarkClosingBrdaRnyingYigMgoSgabMa => TIBETAN_MARK_CLOSING_BRDA_RNYING_YIG_MGO_SGAB_MA,
            Tibetan::RightDashFacingSvastiSign => RIGHT_DASH_FACING_SVASTI_SIGN,
            Tibetan::LeftDashFacingSvastiSign => LEFT_DASH_FACING_SVASTI_SIGN,
            Tibetan::RightDashFacingSvastiSignWithDots => RIGHT_DASH_FACING_SVASTI_SIGN_WITH_DOTS,
            Tibetan::LeftDashFacingSvastiSignWithDots => LEFT_DASH_FACING_SVASTI_SIGN_WITH_DOTS,
            Tibetan::TibetanMarkLeadingMchanRtags => TIBETAN_MARK_LEADING_MCHAN_RTAGS,
            Tibetan::TibetanMarkTrailingMchanRtags => TIBETAN_MARK_TRAILING_MCHAN_RTAGS,
        }
    }
}

impl std::convert::TryFrom<char> for Tibetan {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            TIBETAN_SYLLABLE_OM => Ok(Tibetan::TibetanSyllableOm),
            TIBETAN_MARK_GTER_YIG_MGO_TRUNCATED_A => Ok(Tibetan::TibetanMarkGterYigMgoTruncatedA),
            TIBETAN_MARK_GTER_YIG_MGO__DASH_UM_RNAM_BCAD_MA => Ok(Tibetan::TibetanMarkGterYigMgoDashUmRnamBcadMa),
            TIBETAN_MARK_GTER_YIG_MGO__DASH_UM_GTER_TSHEG_MA => Ok(Tibetan::TibetanMarkGterYigMgoDashUmGterTshegMa),
            TIBETAN_MARK_INITIAL_YIG_MGO_MDUN_MA => Ok(Tibetan::TibetanMarkInitialYigMgoMdunMa),
            TIBETAN_MARK_CLOSING_YIG_MGO_SGAB_MA => Ok(Tibetan::TibetanMarkClosingYigMgoSgabMa),
            TIBETAN_MARK_CARET_YIG_MGO_PHUR_SHAD_MA => Ok(Tibetan::TibetanMarkCaretYigMgoPhurShadMa),
            TIBETAN_MARK_YIG_MGO_TSHEG_SHAD_MA => Ok(Tibetan::TibetanMarkYigMgoTshegShadMa),
            TIBETAN_MARK_SBRUL_SHAD => Ok(Tibetan::TibetanMarkSbrulShad),
            TIBETAN_MARK_BSKUR_YIG_MGO => Ok(Tibetan::TibetanMarkBskurYigMgo),
            TIBETAN_MARK_BKA_DASH__SHOG_YIG_MGO => Ok(Tibetan::TibetanMarkBkaDashShogYigMgo),
            TIBETAN_MARK_INTERSYLLABIC_TSHEG => Ok(Tibetan::TibetanMarkIntersyllabicTsheg),
            TIBETAN_MARK_DELIMITER_TSHEG_BSTAR => Ok(Tibetan::TibetanMarkDelimiterTshegBstar),
            TIBETAN_MARK_SHAD => Ok(Tibetan::TibetanMarkShad),
            TIBETAN_MARK_NYIS_SHAD => Ok(Tibetan::TibetanMarkNyisShad),
            TIBETAN_MARK_TSHEG_SHAD => Ok(Tibetan::TibetanMarkTshegShad),
            TIBETAN_MARK_NYIS_TSHEG_SHAD => Ok(Tibetan::TibetanMarkNyisTshegShad),
            TIBETAN_MARK_RIN_CHEN_SPUNGS_SHAD => Ok(Tibetan::TibetanMarkRinChenSpungsShad),
            TIBETAN_MARK_RGYA_GRAM_SHAD => Ok(Tibetan::TibetanMarkRgyaGramShad),
            TIBETAN_MARK_CARET__DASH_DZUD_RTAGS_ME_LONG_CAN => Ok(Tibetan::TibetanMarkCaretDashDzudRtagsMeLongCan),
            TIBETAN_MARK_GTER_TSHEG => Ok(Tibetan::TibetanMarkGterTsheg),
            TIBETAN_LOGOTYPE_SIGN_CHAD_RTAGS => Ok(Tibetan::TibetanLogotypeSignChadRtags),
            TIBETAN_LOGOTYPE_SIGN_LHAG_RTAGS => Ok(Tibetan::TibetanLogotypeSignLhagRtags),
            TIBETAN_ASTROLOGICAL_SIGN_SGRA_GCAN__DASH_CHAR_RTAGS => Ok(Tibetan::TibetanAstrologicalSignSgraGcanDashCharRtags),
            TIBETAN_ASTROLOGICAL_SIGN__DASH_KHYUD_PA => Ok(Tibetan::TibetanAstrologicalSignDashKhyudPa),
            TIBETAN_ASTROLOGICAL_SIGN_SDONG_TSHUGS => Ok(Tibetan::TibetanAstrologicalSignSdongTshugs),
            TIBETAN_SIGN_RDEL_DKAR_GCIG => Ok(Tibetan::TibetanSignRdelDkarGcig),
            TIBETAN_SIGN_RDEL_DKAR_GNYIS => Ok(Tibetan::TibetanSignRdelDkarGnyis),
            TIBETAN_SIGN_RDEL_DKAR_GSUM => Ok(Tibetan::TibetanSignRdelDkarGsum),
            TIBETAN_SIGN_RDEL_NAG_GCIG => Ok(Tibetan::TibetanSignRdelNagGcig),
            TIBETAN_SIGN_RDEL_NAG_GNYIS => Ok(Tibetan::TibetanSignRdelNagGnyis),
            TIBETAN_SIGN_RDEL_DKAR_RDEL_NAG => Ok(Tibetan::TibetanSignRdelDkarRdelNag),
            TIBETAN_DIGIT_ZERO => Ok(Tibetan::TibetanDigitZero),
            TIBETAN_DIGIT_ONE => Ok(Tibetan::TibetanDigitOne),
            TIBETAN_DIGIT_TWO => Ok(Tibetan::TibetanDigitTwo),
            TIBETAN_DIGIT_THREE => Ok(Tibetan::TibetanDigitThree),
            TIBETAN_DIGIT_FOUR => Ok(Tibetan::TibetanDigitFour),
            TIBETAN_DIGIT_FIVE => Ok(Tibetan::TibetanDigitFive),
            TIBETAN_DIGIT_SIX => Ok(Tibetan::TibetanDigitSix),
            TIBETAN_DIGIT_SEVEN => Ok(Tibetan::TibetanDigitSeven),
            TIBETAN_DIGIT_EIGHT => Ok(Tibetan::TibetanDigitEight),
            TIBETAN_DIGIT_NINE => Ok(Tibetan::TibetanDigitNine),
            TIBETAN_DIGIT_HALF_ONE => Ok(Tibetan::TibetanDigitHalfOne),
            TIBETAN_DIGIT_HALF_TWO => Ok(Tibetan::TibetanDigitHalfTwo),
            TIBETAN_DIGIT_HALF_THREE => Ok(Tibetan::TibetanDigitHalfThree),
            TIBETAN_DIGIT_HALF_FOUR => Ok(Tibetan::TibetanDigitHalfFour),
            TIBETAN_DIGIT_HALF_FIVE => Ok(Tibetan::TibetanDigitHalfFive),
            TIBETAN_DIGIT_HALF_SIX => Ok(Tibetan::TibetanDigitHalfSix),
            TIBETAN_DIGIT_HALF_SEVEN => Ok(Tibetan::TibetanDigitHalfSeven),
            TIBETAN_DIGIT_HALF_EIGHT => Ok(Tibetan::TibetanDigitHalfEight),
            TIBETAN_DIGIT_HALF_NINE => Ok(Tibetan::TibetanDigitHalfNine),
            TIBETAN_DIGIT_HALF_ZERO => Ok(Tibetan::TibetanDigitHalfZero),
            TIBETAN_MARK_BSDUS_RTAGS => Ok(Tibetan::TibetanMarkBsdusRtags),
            TIBETAN_MARK_NGAS_BZUNG_NYI_ZLA => Ok(Tibetan::TibetanMarkNgasBzungNyiZla),
            TIBETAN_MARK_CARET__DASH_DZUD_RTAGS_BZHI_MIG_CAN => Ok(Tibetan::TibetanMarkCaretDashDzudRtagsBzhiMigCan),
            TIBETAN_MARK_NGAS_BZUNG_SGOR_RTAGS => Ok(Tibetan::TibetanMarkNgasBzungSgorRtags),
            TIBETAN_MARK_CHE_MGO => Ok(Tibetan::TibetanMarkCheMgo),
            TIBETAN_MARK_TSA__DASH_PHRU => Ok(Tibetan::TibetanMarkTsaDashPhru),
            TIBETAN_MARK_GUG_RTAGS_GYON => Ok(Tibetan::TibetanMarkGugRtagsGyon),
            TIBETAN_MARK_GUG_RTAGS_GYAS => Ok(Tibetan::TibetanMarkGugRtagsGyas),
            TIBETAN_MARK_ANG_KHANG_GYON => Ok(Tibetan::TibetanMarkAngKhangGyon),
            TIBETAN_MARK_ANG_KHANG_GYAS => Ok(Tibetan::TibetanMarkAngKhangGyas),
            TIBETAN_SIGN_YAR_TSHES => Ok(Tibetan::TibetanSignYarTshes),
            TIBETAN_SIGN_MAR_TSHES => Ok(Tibetan::TibetanSignMarTshes),
            TIBETAN_LETTER_KA => Ok(Tibetan::TibetanLetterKa),
            TIBETAN_LETTER_KHA => Ok(Tibetan::TibetanLetterKha),
            TIBETAN_LETTER_GA => Ok(Tibetan::TibetanLetterGa),
            TIBETAN_LETTER_GHA => Ok(Tibetan::TibetanLetterGha),
            TIBETAN_LETTER_NGA => Ok(Tibetan::TibetanLetterNga),
            TIBETAN_LETTER_CA => Ok(Tibetan::TibetanLetterCa),
            TIBETAN_LETTER_CHA => Ok(Tibetan::TibetanLetterCha),
            TIBETAN_LETTER_JA => Ok(Tibetan::TibetanLetterJa),
            TIBETAN_LETTER_NYA => Ok(Tibetan::TibetanLetterNya),
            TIBETAN_LETTER_TTA => Ok(Tibetan::TibetanLetterTta),
            TIBETAN_LETTER_TTHA => Ok(Tibetan::TibetanLetterTtha),
            TIBETAN_LETTER_DDA => Ok(Tibetan::TibetanLetterDda),
            TIBETAN_LETTER_DDHA => Ok(Tibetan::TibetanLetterDdha),
            TIBETAN_LETTER_NNA => Ok(Tibetan::TibetanLetterNna),
            TIBETAN_LETTER_TA => Ok(Tibetan::TibetanLetterTa),
            TIBETAN_LETTER_THA => Ok(Tibetan::TibetanLetterTha),
            TIBETAN_LETTER_DA => Ok(Tibetan::TibetanLetterDa),
            TIBETAN_LETTER_DHA => Ok(Tibetan::TibetanLetterDha),
            TIBETAN_LETTER_NA => Ok(Tibetan::TibetanLetterNa),
            TIBETAN_LETTER_PA => Ok(Tibetan::TibetanLetterPa),
            TIBETAN_LETTER_PHA => Ok(Tibetan::TibetanLetterPha),
            TIBETAN_LETTER_BA => Ok(Tibetan::TibetanLetterBa),
            TIBETAN_LETTER_BHA => Ok(Tibetan::TibetanLetterBha),
            TIBETAN_LETTER_MA => Ok(Tibetan::TibetanLetterMa),
            TIBETAN_LETTER_TSA => Ok(Tibetan::TibetanLetterTsa),
            TIBETAN_LETTER_TSHA => Ok(Tibetan::TibetanLetterTsha),
            TIBETAN_LETTER_DZA => Ok(Tibetan::TibetanLetterDza),
            TIBETAN_LETTER_DZHA => Ok(Tibetan::TibetanLetterDzha),
            TIBETAN_LETTER_WA => Ok(Tibetan::TibetanLetterWa),
            TIBETAN_LETTER_ZHA => Ok(Tibetan::TibetanLetterZha),
            TIBETAN_LETTER_ZA => Ok(Tibetan::TibetanLetterZa),
            TIBETAN_LETTER__DASH_A => Ok(Tibetan::TibetanLetterDashA),
            TIBETAN_LETTER_YA => Ok(Tibetan::TibetanLetterYa),
            TIBETAN_LETTER_RA => Ok(Tibetan::TibetanLetterRa),
            TIBETAN_LETTER_LA => Ok(Tibetan::TibetanLetterLa),
            TIBETAN_LETTER_SHA => Ok(Tibetan::TibetanLetterSha),
            TIBETAN_LETTER_SSA => Ok(Tibetan::TibetanLetterSsa),
            TIBETAN_LETTER_SA => Ok(Tibetan::TibetanLetterSa),
            TIBETAN_LETTER_HA => Ok(Tibetan::TibetanLetterHa),
            TIBETAN_LETTER_A => Ok(Tibetan::TibetanLetterA),
            TIBETAN_LETTER_KSSA => Ok(Tibetan::TibetanLetterKssa),
            TIBETAN_LETTER_FIXED_DASH_FORM_RA => Ok(Tibetan::TibetanLetterFixedDashFormRa),
            TIBETAN_LETTER_KKA => Ok(Tibetan::TibetanLetterKka),
            TIBETAN_LETTER_RRA => Ok(Tibetan::TibetanLetterRra),
            TIBETAN_VOWEL_SIGN_AA => Ok(Tibetan::TibetanVowelSignAa),
            TIBETAN_VOWEL_SIGN_I => Ok(Tibetan::TibetanVowelSignI),
            TIBETAN_VOWEL_SIGN_II => Ok(Tibetan::TibetanVowelSignIi),
            TIBETAN_VOWEL_SIGN_U => Ok(Tibetan::TibetanVowelSignU),
            TIBETAN_VOWEL_SIGN_UU => Ok(Tibetan::TibetanVowelSignUu),
            TIBETAN_VOWEL_SIGN_VOCALIC_R => Ok(Tibetan::TibetanVowelSignVocalicR),
            TIBETAN_VOWEL_SIGN_VOCALIC_RR => Ok(Tibetan::TibetanVowelSignVocalicRr),
            TIBETAN_VOWEL_SIGN_VOCALIC_L => Ok(Tibetan::TibetanVowelSignVocalicL),
            TIBETAN_VOWEL_SIGN_VOCALIC_LL => Ok(Tibetan::TibetanVowelSignVocalicLl),
            TIBETAN_VOWEL_SIGN_E => Ok(Tibetan::TibetanVowelSignE),
            TIBETAN_VOWEL_SIGN_EE => Ok(Tibetan::TibetanVowelSignEe),
            TIBETAN_VOWEL_SIGN_O => Ok(Tibetan::TibetanVowelSignO),
            TIBETAN_VOWEL_SIGN_OO => Ok(Tibetan::TibetanVowelSignOo),
            TIBETAN_SIGN_RJES_SU_NGA_RO => Ok(Tibetan::TibetanSignRjesSuNgaRo),
            TIBETAN_SIGN_RNAM_BCAD => Ok(Tibetan::TibetanSignRnamBcad),
            TIBETAN_VOWEL_SIGN_REVERSED_I => Ok(Tibetan::TibetanVowelSignReversedI),
            TIBETAN_VOWEL_SIGN_REVERSED_II => Ok(Tibetan::TibetanVowelSignReversedIi),
            TIBETAN_SIGN_NYI_ZLA_NAA_DA => Ok(Tibetan::TibetanSignNyiZlaNaaDa),
            TIBETAN_SIGN_SNA_LDAN => Ok(Tibetan::TibetanSignSnaLdan),
            TIBETAN_MARK_HALANTA => Ok(Tibetan::TibetanMarkHalanta),
            TIBETAN_MARK_PALUTA => Ok(Tibetan::TibetanMarkPaluta),
            TIBETAN_SIGN_LCI_RTAGS => Ok(Tibetan::TibetanSignLciRtags),
            TIBETAN_SIGN_YANG_RTAGS => Ok(Tibetan::TibetanSignYangRtags),
            TIBETAN_SIGN_LCE_TSA_CAN => Ok(Tibetan::TibetanSignLceTsaCan),
            TIBETAN_SIGN_MCHU_CAN => Ok(Tibetan::TibetanSignMchuCan),
            TIBETAN_SIGN_GRU_CAN_RGYINGS => Ok(Tibetan::TibetanSignGruCanRgyings),
            TIBETAN_SIGN_GRU_MED_RGYINGS => Ok(Tibetan::TibetanSignGruMedRgyings),
            TIBETAN_SIGN_INVERTED_MCHU_CAN => Ok(Tibetan::TibetanSignInvertedMchuCan),
            TIBETAN_SUBJOINED_SIGN_LCE_TSA_CAN => Ok(Tibetan::TibetanSubjoinedSignLceTsaCan),
            TIBETAN_SUBJOINED_SIGN_MCHU_CAN => Ok(Tibetan::TibetanSubjoinedSignMchuCan),
            TIBETAN_SUBJOINED_SIGN_INVERTED_MCHU_CAN => Ok(Tibetan::TibetanSubjoinedSignInvertedMchuCan),
            TIBETAN_SUBJOINED_LETTER_KA => Ok(Tibetan::TibetanSubjoinedLetterKa),
            TIBETAN_SUBJOINED_LETTER_KHA => Ok(Tibetan::TibetanSubjoinedLetterKha),
            TIBETAN_SUBJOINED_LETTER_GA => Ok(Tibetan::TibetanSubjoinedLetterGa),
            TIBETAN_SUBJOINED_LETTER_GHA => Ok(Tibetan::TibetanSubjoinedLetterGha),
            TIBETAN_SUBJOINED_LETTER_NGA => Ok(Tibetan::TibetanSubjoinedLetterNga),
            TIBETAN_SUBJOINED_LETTER_CA => Ok(Tibetan::TibetanSubjoinedLetterCa),
            TIBETAN_SUBJOINED_LETTER_CHA => Ok(Tibetan::TibetanSubjoinedLetterCha),
            TIBETAN_SUBJOINED_LETTER_JA => Ok(Tibetan::TibetanSubjoinedLetterJa),
            TIBETAN_SUBJOINED_LETTER_NYA => Ok(Tibetan::TibetanSubjoinedLetterNya),
            TIBETAN_SUBJOINED_LETTER_TTA => Ok(Tibetan::TibetanSubjoinedLetterTta),
            TIBETAN_SUBJOINED_LETTER_TTHA => Ok(Tibetan::TibetanSubjoinedLetterTtha),
            TIBETAN_SUBJOINED_LETTER_DDA => Ok(Tibetan::TibetanSubjoinedLetterDda),
            TIBETAN_SUBJOINED_LETTER_DDHA => Ok(Tibetan::TibetanSubjoinedLetterDdha),
            TIBETAN_SUBJOINED_LETTER_NNA => Ok(Tibetan::TibetanSubjoinedLetterNna),
            TIBETAN_SUBJOINED_LETTER_TA => Ok(Tibetan::TibetanSubjoinedLetterTa),
            TIBETAN_SUBJOINED_LETTER_THA => Ok(Tibetan::TibetanSubjoinedLetterTha),
            TIBETAN_SUBJOINED_LETTER_DA => Ok(Tibetan::TibetanSubjoinedLetterDa),
            TIBETAN_SUBJOINED_LETTER_DHA => Ok(Tibetan::TibetanSubjoinedLetterDha),
            TIBETAN_SUBJOINED_LETTER_NA => Ok(Tibetan::TibetanSubjoinedLetterNa),
            TIBETAN_SUBJOINED_LETTER_PA => Ok(Tibetan::TibetanSubjoinedLetterPa),
            TIBETAN_SUBJOINED_LETTER_PHA => Ok(Tibetan::TibetanSubjoinedLetterPha),
            TIBETAN_SUBJOINED_LETTER_BA => Ok(Tibetan::TibetanSubjoinedLetterBa),
            TIBETAN_SUBJOINED_LETTER_BHA => Ok(Tibetan::TibetanSubjoinedLetterBha),
            TIBETAN_SUBJOINED_LETTER_MA => Ok(Tibetan::TibetanSubjoinedLetterMa),
            TIBETAN_SUBJOINED_LETTER_TSA => Ok(Tibetan::TibetanSubjoinedLetterTsa),
            TIBETAN_SUBJOINED_LETTER_TSHA => Ok(Tibetan::TibetanSubjoinedLetterTsha),
            TIBETAN_SUBJOINED_LETTER_DZA => Ok(Tibetan::TibetanSubjoinedLetterDza),
            TIBETAN_SUBJOINED_LETTER_DZHA => Ok(Tibetan::TibetanSubjoinedLetterDzha),
            TIBETAN_SUBJOINED_LETTER_WA => Ok(Tibetan::TibetanSubjoinedLetterWa),
            TIBETAN_SUBJOINED_LETTER_ZHA => Ok(Tibetan::TibetanSubjoinedLetterZha),
            TIBETAN_SUBJOINED_LETTER_ZA => Ok(Tibetan::TibetanSubjoinedLetterZa),
            TIBETAN_SUBJOINED_LETTER__DASH_A => Ok(Tibetan::TibetanSubjoinedLetterDashA),
            TIBETAN_SUBJOINED_LETTER_YA => Ok(Tibetan::TibetanSubjoinedLetterYa),
            TIBETAN_SUBJOINED_LETTER_RA => Ok(Tibetan::TibetanSubjoinedLetterRa),
            TIBETAN_SUBJOINED_LETTER_LA => Ok(Tibetan::TibetanSubjoinedLetterLa),
            TIBETAN_SUBJOINED_LETTER_SHA => Ok(Tibetan::TibetanSubjoinedLetterSha),
            TIBETAN_SUBJOINED_LETTER_SSA => Ok(Tibetan::TibetanSubjoinedLetterSsa),
            TIBETAN_SUBJOINED_LETTER_SA => Ok(Tibetan::TibetanSubjoinedLetterSa),
            TIBETAN_SUBJOINED_LETTER_HA => Ok(Tibetan::TibetanSubjoinedLetterHa),
            TIBETAN_SUBJOINED_LETTER_A => Ok(Tibetan::TibetanSubjoinedLetterA),
            TIBETAN_SUBJOINED_LETTER_KSSA => Ok(Tibetan::TibetanSubjoinedLetterKssa),
            TIBETAN_SUBJOINED_LETTER_FIXED_DASH_FORM_WA => Ok(Tibetan::TibetanSubjoinedLetterFixedDashFormWa),
            TIBETAN_SUBJOINED_LETTER_FIXED_DASH_FORM_YA => Ok(Tibetan::TibetanSubjoinedLetterFixedDashFormYa),
            TIBETAN_SUBJOINED_LETTER_FIXED_DASH_FORM_RA => Ok(Tibetan::TibetanSubjoinedLetterFixedDashFormRa),
            TIBETAN_KU_RU_KHA => Ok(Tibetan::TibetanKuRuKha),
            TIBETAN_KU_RU_KHA_BZHI_MIG_CAN => Ok(Tibetan::TibetanKuRuKhaBzhiMigCan),
            TIBETAN_CANTILLATION_SIGN_HEAVY_BEAT => Ok(Tibetan::TibetanCantillationSignHeavyBeat),
            TIBETAN_CANTILLATION_SIGN_LIGHT_BEAT => Ok(Tibetan::TibetanCantillationSignLightBeat),
            TIBETAN_CANTILLATION_SIGN_CANG_TE_DASH_U => Ok(Tibetan::TibetanCantillationSignCangTeDashU),
            TIBETAN_CANTILLATION_SIGN_SBUB__DASH_CHAL => Ok(Tibetan::TibetanCantillationSignSbubDashChal),
            TIBETAN_SYMBOL_DRIL_BU => Ok(Tibetan::TibetanSymbolDrilBu),
            TIBETAN_SYMBOL_RDO_RJE => Ok(Tibetan::TibetanSymbolRdoRje),
            TIBETAN_SYMBOL_PADMA_GDAN => Ok(Tibetan::TibetanSymbolPadmaGdan),
            TIBETAN_SYMBOL_RDO_RJE_RGYA_GRAM => Ok(Tibetan::TibetanSymbolRdoRjeRgyaGram),
            TIBETAN_SYMBOL_PHUR_PA => Ok(Tibetan::TibetanSymbolPhurPa),
            TIBETAN_SYMBOL_NOR_BU => Ok(Tibetan::TibetanSymbolNorBu),
            TIBETAN_SYMBOL_NOR_BU_NYIS__DASH_KHYIL => Ok(Tibetan::TibetanSymbolNorBuNyisDashKhyil),
            TIBETAN_SYMBOL_NOR_BU_GSUM__DASH_KHYIL => Ok(Tibetan::TibetanSymbolNorBuGsumDashKhyil),
            TIBETAN_SYMBOL_NOR_BU_BZHI__DASH_KHYIL => Ok(Tibetan::TibetanSymbolNorBuBzhiDashKhyil),
            TIBETAN_SIGN_RDEL_NAG_RDEL_DKAR => Ok(Tibetan::TibetanSignRdelNagRdelDkar),
            TIBETAN_SIGN_RDEL_NAG_GSUM => Ok(Tibetan::TibetanSignRdelNagGsum),
            TIBETAN_MARK_BSKA_DASH__SHOG_GI_MGO_RGYAN => Ok(Tibetan::TibetanMarkBskaDashShogGiMgoRgyan),
            TIBETAN_MARK_MNYAM_YIG_GI_MGO_RGYAN => Ok(Tibetan::TibetanMarkMnyamYigGiMgoRgyan),
            TIBETAN_MARK_NYIS_TSHEG => Ok(Tibetan::TibetanMarkNyisTsheg),
            TIBETAN_MARK_INITIAL_BRDA_RNYING_YIG_MGO_MDUN_MA => Ok(Tibetan::TibetanMarkInitialBrdaRnyingYigMgoMdunMa),
            TIBETAN_MARK_CLOSING_BRDA_RNYING_YIG_MGO_SGAB_MA => Ok(Tibetan::TibetanMarkClosingBrdaRnyingYigMgoSgabMa),
            RIGHT_DASH_FACING_SVASTI_SIGN => Ok(Tibetan::RightDashFacingSvastiSign),
            LEFT_DASH_FACING_SVASTI_SIGN => Ok(Tibetan::LeftDashFacingSvastiSign),
            RIGHT_DASH_FACING_SVASTI_SIGN_WITH_DOTS => Ok(Tibetan::RightDashFacingSvastiSignWithDots),
            LEFT_DASH_FACING_SVASTI_SIGN_WITH_DOTS => Ok(Tibetan::LeftDashFacingSvastiSignWithDots),
            TIBETAN_MARK_LEADING_MCHAN_RTAGS => Ok(Tibetan::TibetanMarkLeadingMchanRtags),
            TIBETAN_MARK_TRAILING_MCHAN_RTAGS => Ok(Tibetan::TibetanMarkTrailingMchanRtags),
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
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        Tibetan::TibetanSyllableOm
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Tibetan::TibetanSyllableOm => "tibetan syllable om",
            Tibetan::TibetanMarkGterYigMgoTruncatedA => "tibetan mark gter yig mgo truncated a",
            Tibetan::TibetanMarkGterYigMgoDashUmRnamBcadMa => "tibetan mark gter yig mgo -um rnam bcad ma",
            Tibetan::TibetanMarkGterYigMgoDashUmGterTshegMa => "tibetan mark gter yig mgo -um gter tsheg ma",
            Tibetan::TibetanMarkInitialYigMgoMdunMa => "tibetan mark initial yig mgo mdun ma",
            Tibetan::TibetanMarkClosingYigMgoSgabMa => "tibetan mark closing yig mgo sgab ma",
            Tibetan::TibetanMarkCaretYigMgoPhurShadMa => "tibetan mark caret yig mgo phur shad ma",
            Tibetan::TibetanMarkYigMgoTshegShadMa => "tibetan mark yig mgo tsheg shad ma",
            Tibetan::TibetanMarkSbrulShad => "tibetan mark sbrul shad",
            Tibetan::TibetanMarkBskurYigMgo => "tibetan mark bskur yig mgo",
            Tibetan::TibetanMarkBkaDashShogYigMgo => "tibetan mark bka- shog yig mgo",
            Tibetan::TibetanMarkIntersyllabicTsheg => "tibetan mark intersyllabic tsheg",
            Tibetan::TibetanMarkDelimiterTshegBstar => "tibetan mark delimiter tsheg bstar",
            Tibetan::TibetanMarkShad => "tibetan mark shad",
            Tibetan::TibetanMarkNyisShad => "tibetan mark nyis shad",
            Tibetan::TibetanMarkTshegShad => "tibetan mark tsheg shad",
            Tibetan::TibetanMarkNyisTshegShad => "tibetan mark nyis tsheg shad",
            Tibetan::TibetanMarkRinChenSpungsShad => "tibetan mark rin chen spungs shad",
            Tibetan::TibetanMarkRgyaGramShad => "tibetan mark rgya gram shad",
            Tibetan::TibetanMarkCaretDashDzudRtagsMeLongCan => "tibetan mark caret -dzud rtags me long can",
            Tibetan::TibetanMarkGterTsheg => "tibetan mark gter tsheg",
            Tibetan::TibetanLogotypeSignChadRtags => "tibetan logotype sign chad rtags",
            Tibetan::TibetanLogotypeSignLhagRtags => "tibetan logotype sign lhag rtags",
            Tibetan::TibetanAstrologicalSignSgraGcanDashCharRtags => "tibetan astrological sign sgra gcan -char rtags",
            Tibetan::TibetanAstrologicalSignDashKhyudPa => "tibetan astrological sign -khyud pa",
            Tibetan::TibetanAstrologicalSignSdongTshugs => "tibetan astrological sign sdong tshugs",
            Tibetan::TibetanSignRdelDkarGcig => "tibetan sign rdel dkar gcig",
            Tibetan::TibetanSignRdelDkarGnyis => "tibetan sign rdel dkar gnyis",
            Tibetan::TibetanSignRdelDkarGsum => "tibetan sign rdel dkar gsum",
            Tibetan::TibetanSignRdelNagGcig => "tibetan sign rdel nag gcig",
            Tibetan::TibetanSignRdelNagGnyis => "tibetan sign rdel nag gnyis",
            Tibetan::TibetanSignRdelDkarRdelNag => "tibetan sign rdel dkar rdel nag",
            Tibetan::TibetanDigitZero => "tibetan digit zero",
            Tibetan::TibetanDigitOne => "tibetan digit one",
            Tibetan::TibetanDigitTwo => "tibetan digit two",
            Tibetan::TibetanDigitThree => "tibetan digit three",
            Tibetan::TibetanDigitFour => "tibetan digit four",
            Tibetan::TibetanDigitFive => "tibetan digit five",
            Tibetan::TibetanDigitSix => "tibetan digit six",
            Tibetan::TibetanDigitSeven => "tibetan digit seven",
            Tibetan::TibetanDigitEight => "tibetan digit eight",
            Tibetan::TibetanDigitNine => "tibetan digit nine",
            Tibetan::TibetanDigitHalfOne => "tibetan digit half one",
            Tibetan::TibetanDigitHalfTwo => "tibetan digit half two",
            Tibetan::TibetanDigitHalfThree => "tibetan digit half three",
            Tibetan::TibetanDigitHalfFour => "tibetan digit half four",
            Tibetan::TibetanDigitHalfFive => "tibetan digit half five",
            Tibetan::TibetanDigitHalfSix => "tibetan digit half six",
            Tibetan::TibetanDigitHalfSeven => "tibetan digit half seven",
            Tibetan::TibetanDigitHalfEight => "tibetan digit half eight",
            Tibetan::TibetanDigitHalfNine => "tibetan digit half nine",
            Tibetan::TibetanDigitHalfZero => "tibetan digit half zero",
            Tibetan::TibetanMarkBsdusRtags => "tibetan mark bsdus rtags",
            Tibetan::TibetanMarkNgasBzungNyiZla => "tibetan mark ngas bzung nyi zla",
            Tibetan::TibetanMarkCaretDashDzudRtagsBzhiMigCan => "tibetan mark caret -dzud rtags bzhi mig can",
            Tibetan::TibetanMarkNgasBzungSgorRtags => "tibetan mark ngas bzung sgor rtags",
            Tibetan::TibetanMarkCheMgo => "tibetan mark che mgo",
            Tibetan::TibetanMarkTsaDashPhru => "tibetan mark tsa -phru",
            Tibetan::TibetanMarkGugRtagsGyon => "tibetan mark gug rtags gyon",
            Tibetan::TibetanMarkGugRtagsGyas => "tibetan mark gug rtags gyas",
            Tibetan::TibetanMarkAngKhangGyon => "tibetan mark ang khang gyon",
            Tibetan::TibetanMarkAngKhangGyas => "tibetan mark ang khang gyas",
            Tibetan::TibetanSignYarTshes => "tibetan sign yar tshes",
            Tibetan::TibetanSignMarTshes => "tibetan sign mar tshes",
            Tibetan::TibetanLetterKa => "tibetan letter ka",
            Tibetan::TibetanLetterKha => "tibetan letter kha",
            Tibetan::TibetanLetterGa => "tibetan letter ga",
            Tibetan::TibetanLetterGha => "tibetan letter gha",
            Tibetan::TibetanLetterNga => "tibetan letter nga",
            Tibetan::TibetanLetterCa => "tibetan letter ca",
            Tibetan::TibetanLetterCha => "tibetan letter cha",
            Tibetan::TibetanLetterJa => "tibetan letter ja",
            Tibetan::TibetanLetterNya => "tibetan letter nya",
            Tibetan::TibetanLetterTta => "tibetan letter tta",
            Tibetan::TibetanLetterTtha => "tibetan letter ttha",
            Tibetan::TibetanLetterDda => "tibetan letter dda",
            Tibetan::TibetanLetterDdha => "tibetan letter ddha",
            Tibetan::TibetanLetterNna => "tibetan letter nna",
            Tibetan::TibetanLetterTa => "tibetan letter ta",
            Tibetan::TibetanLetterTha => "tibetan letter tha",
            Tibetan::TibetanLetterDa => "tibetan letter da",
            Tibetan::TibetanLetterDha => "tibetan letter dha",
            Tibetan::TibetanLetterNa => "tibetan letter na",
            Tibetan::TibetanLetterPa => "tibetan letter pa",
            Tibetan::TibetanLetterPha => "tibetan letter pha",
            Tibetan::TibetanLetterBa => "tibetan letter ba",
            Tibetan::TibetanLetterBha => "tibetan letter bha",
            Tibetan::TibetanLetterMa => "tibetan letter ma",
            Tibetan::TibetanLetterTsa => "tibetan letter tsa",
            Tibetan::TibetanLetterTsha => "tibetan letter tsha",
            Tibetan::TibetanLetterDza => "tibetan letter dza",
            Tibetan::TibetanLetterDzha => "tibetan letter dzha",
            Tibetan::TibetanLetterWa => "tibetan letter wa",
            Tibetan::TibetanLetterZha => "tibetan letter zha",
            Tibetan::TibetanLetterZa => "tibetan letter za",
            Tibetan::TibetanLetterDashA => "tibetan letter -a",
            Tibetan::TibetanLetterYa => "tibetan letter ya",
            Tibetan::TibetanLetterRa => "tibetan letter ra",
            Tibetan::TibetanLetterLa => "tibetan letter la",
            Tibetan::TibetanLetterSha => "tibetan letter sha",
            Tibetan::TibetanLetterSsa => "tibetan letter ssa",
            Tibetan::TibetanLetterSa => "tibetan letter sa",
            Tibetan::TibetanLetterHa => "tibetan letter ha",
            Tibetan::TibetanLetterA => "tibetan letter a",
            Tibetan::TibetanLetterKssa => "tibetan letter kssa",
            Tibetan::TibetanLetterFixedDashFormRa => "tibetan letter fixed-form ra",
            Tibetan::TibetanLetterKka => "tibetan letter kka",
            Tibetan::TibetanLetterRra => "tibetan letter rra",
            Tibetan::TibetanVowelSignAa => "tibetan vowel sign aa",
            Tibetan::TibetanVowelSignI => "tibetan vowel sign i",
            Tibetan::TibetanVowelSignIi => "tibetan vowel sign ii",
            Tibetan::TibetanVowelSignU => "tibetan vowel sign u",
            Tibetan::TibetanVowelSignUu => "tibetan vowel sign uu",
            Tibetan::TibetanVowelSignVocalicR => "tibetan vowel sign vocalic r",
            Tibetan::TibetanVowelSignVocalicRr => "tibetan vowel sign vocalic rr",
            Tibetan::TibetanVowelSignVocalicL => "tibetan vowel sign vocalic l",
            Tibetan::TibetanVowelSignVocalicLl => "tibetan vowel sign vocalic ll",
            Tibetan::TibetanVowelSignE => "tibetan vowel sign e",
            Tibetan::TibetanVowelSignEe => "tibetan vowel sign ee",
            Tibetan::TibetanVowelSignO => "tibetan vowel sign o",
            Tibetan::TibetanVowelSignOo => "tibetan vowel sign oo",
            Tibetan::TibetanSignRjesSuNgaRo => "tibetan sign rjes su nga ro",
            Tibetan::TibetanSignRnamBcad => "tibetan sign rnam bcad",
            Tibetan::TibetanVowelSignReversedI => "tibetan vowel sign reversed i",
            Tibetan::TibetanVowelSignReversedIi => "tibetan vowel sign reversed ii",
            Tibetan::TibetanSignNyiZlaNaaDa => "tibetan sign nyi zla naa da",
            Tibetan::TibetanSignSnaLdan => "tibetan sign sna ldan",
            Tibetan::TibetanMarkHalanta => "tibetan mark halanta",
            Tibetan::TibetanMarkPaluta => "tibetan mark paluta",
            Tibetan::TibetanSignLciRtags => "tibetan sign lci rtags",
            Tibetan::TibetanSignYangRtags => "tibetan sign yang rtags",
            Tibetan::TibetanSignLceTsaCan => "tibetan sign lce tsa can",
            Tibetan::TibetanSignMchuCan => "tibetan sign mchu can",
            Tibetan::TibetanSignGruCanRgyings => "tibetan sign gru can rgyings",
            Tibetan::TibetanSignGruMedRgyings => "tibetan sign gru med rgyings",
            Tibetan::TibetanSignInvertedMchuCan => "tibetan sign inverted mchu can",
            Tibetan::TibetanSubjoinedSignLceTsaCan => "tibetan subjoined sign lce tsa can",
            Tibetan::TibetanSubjoinedSignMchuCan => "tibetan subjoined sign mchu can",
            Tibetan::TibetanSubjoinedSignInvertedMchuCan => "tibetan subjoined sign inverted mchu can",
            Tibetan::TibetanSubjoinedLetterKa => "tibetan subjoined letter ka",
            Tibetan::TibetanSubjoinedLetterKha => "tibetan subjoined letter kha",
            Tibetan::TibetanSubjoinedLetterGa => "tibetan subjoined letter ga",
            Tibetan::TibetanSubjoinedLetterGha => "tibetan subjoined letter gha",
            Tibetan::TibetanSubjoinedLetterNga => "tibetan subjoined letter nga",
            Tibetan::TibetanSubjoinedLetterCa => "tibetan subjoined letter ca",
            Tibetan::TibetanSubjoinedLetterCha => "tibetan subjoined letter cha",
            Tibetan::TibetanSubjoinedLetterJa => "tibetan subjoined letter ja",
            Tibetan::TibetanSubjoinedLetterNya => "tibetan subjoined letter nya",
            Tibetan::TibetanSubjoinedLetterTta => "tibetan subjoined letter tta",
            Tibetan::TibetanSubjoinedLetterTtha => "tibetan subjoined letter ttha",
            Tibetan::TibetanSubjoinedLetterDda => "tibetan subjoined letter dda",
            Tibetan::TibetanSubjoinedLetterDdha => "tibetan subjoined letter ddha",
            Tibetan::TibetanSubjoinedLetterNna => "tibetan subjoined letter nna",
            Tibetan::TibetanSubjoinedLetterTa => "tibetan subjoined letter ta",
            Tibetan::TibetanSubjoinedLetterTha => "tibetan subjoined letter tha",
            Tibetan::TibetanSubjoinedLetterDa => "tibetan subjoined letter da",
            Tibetan::TibetanSubjoinedLetterDha => "tibetan subjoined letter dha",
            Tibetan::TibetanSubjoinedLetterNa => "tibetan subjoined letter na",
            Tibetan::TibetanSubjoinedLetterPa => "tibetan subjoined letter pa",
            Tibetan::TibetanSubjoinedLetterPha => "tibetan subjoined letter pha",
            Tibetan::TibetanSubjoinedLetterBa => "tibetan subjoined letter ba",
            Tibetan::TibetanSubjoinedLetterBha => "tibetan subjoined letter bha",
            Tibetan::TibetanSubjoinedLetterMa => "tibetan subjoined letter ma",
            Tibetan::TibetanSubjoinedLetterTsa => "tibetan subjoined letter tsa",
            Tibetan::TibetanSubjoinedLetterTsha => "tibetan subjoined letter tsha",
            Tibetan::TibetanSubjoinedLetterDza => "tibetan subjoined letter dza",
            Tibetan::TibetanSubjoinedLetterDzha => "tibetan subjoined letter dzha",
            Tibetan::TibetanSubjoinedLetterWa => "tibetan subjoined letter wa",
            Tibetan::TibetanSubjoinedLetterZha => "tibetan subjoined letter zha",
            Tibetan::TibetanSubjoinedLetterZa => "tibetan subjoined letter za",
            Tibetan::TibetanSubjoinedLetterDashA => "tibetan subjoined letter -a",
            Tibetan::TibetanSubjoinedLetterYa => "tibetan subjoined letter ya",
            Tibetan::TibetanSubjoinedLetterRa => "tibetan subjoined letter ra",
            Tibetan::TibetanSubjoinedLetterLa => "tibetan subjoined letter la",
            Tibetan::TibetanSubjoinedLetterSha => "tibetan subjoined letter sha",
            Tibetan::TibetanSubjoinedLetterSsa => "tibetan subjoined letter ssa",
            Tibetan::TibetanSubjoinedLetterSa => "tibetan subjoined letter sa",
            Tibetan::TibetanSubjoinedLetterHa => "tibetan subjoined letter ha",
            Tibetan::TibetanSubjoinedLetterA => "tibetan subjoined letter a",
            Tibetan::TibetanSubjoinedLetterKssa => "tibetan subjoined letter kssa",
            Tibetan::TibetanSubjoinedLetterFixedDashFormWa => "tibetan subjoined letter fixed-form wa",
            Tibetan::TibetanSubjoinedLetterFixedDashFormYa => "tibetan subjoined letter fixed-form ya",
            Tibetan::TibetanSubjoinedLetterFixedDashFormRa => "tibetan subjoined letter fixed-form ra",
            Tibetan::TibetanKuRuKha => "tibetan ku ru kha",
            Tibetan::TibetanKuRuKhaBzhiMigCan => "tibetan ku ru kha bzhi mig can",
            Tibetan::TibetanCantillationSignHeavyBeat => "tibetan cantillation sign heavy beat",
            Tibetan::TibetanCantillationSignLightBeat => "tibetan cantillation sign light beat",
            Tibetan::TibetanCantillationSignCangTeDashU => "tibetan cantillation sign cang te-u",
            Tibetan::TibetanCantillationSignSbubDashChal => "tibetan cantillation sign sbub -chal",
            Tibetan::TibetanSymbolDrilBu => "tibetan symbol dril bu",
            Tibetan::TibetanSymbolRdoRje => "tibetan symbol rdo rje",
            Tibetan::TibetanSymbolPadmaGdan => "tibetan symbol padma gdan",
            Tibetan::TibetanSymbolRdoRjeRgyaGram => "tibetan symbol rdo rje rgya gram",
            Tibetan::TibetanSymbolPhurPa => "tibetan symbol phur pa",
            Tibetan::TibetanSymbolNorBu => "tibetan symbol nor bu",
            Tibetan::TibetanSymbolNorBuNyisDashKhyil => "tibetan symbol nor bu nyis -khyil",
            Tibetan::TibetanSymbolNorBuGsumDashKhyil => "tibetan symbol nor bu gsum -khyil",
            Tibetan::TibetanSymbolNorBuBzhiDashKhyil => "tibetan symbol nor bu bzhi -khyil",
            Tibetan::TibetanSignRdelNagRdelDkar => "tibetan sign rdel nag rdel dkar",
            Tibetan::TibetanSignRdelNagGsum => "tibetan sign rdel nag gsum",
            Tibetan::TibetanMarkBskaDashShogGiMgoRgyan => "tibetan mark bska- shog gi mgo rgyan",
            Tibetan::TibetanMarkMnyamYigGiMgoRgyan => "tibetan mark mnyam yig gi mgo rgyan",
            Tibetan::TibetanMarkNyisTsheg => "tibetan mark nyis tsheg",
            Tibetan::TibetanMarkInitialBrdaRnyingYigMgoMdunMa => "tibetan mark initial brda rnying yig mgo mdun ma",
            Tibetan::TibetanMarkClosingBrdaRnyingYigMgoSgabMa => "tibetan mark closing brda rnying yig mgo sgab ma",
            Tibetan::RightDashFacingSvastiSign => "right-facing svasti sign",
            Tibetan::LeftDashFacingSvastiSign => "left-facing svasti sign",
            Tibetan::RightDashFacingSvastiSignWithDots => "right-facing svasti sign with dots",
            Tibetan::LeftDashFacingSvastiSignWithDots => "left-facing svasti sign with dots",
            Tibetan::TibetanMarkLeadingMchanRtags => "tibetan mark leading mchan rtags",
            Tibetan::TibetanMarkTrailingMchanRtags => "tibetan mark trailing mchan rtags",
        }
    }
}
