/// \u{1a20} → \u{1aaf}\
///\
/// ᨠ ᨡ ᨢ ᨣ ᨤ ᨥ ᨦ ᨧ ᨨ ᨩ ᨪ ᨫ ᨬ ᨭ ᨮ ᨯ\
/// ᨰ ᨱ ᨲ ᨳ ᨴ ᨵ ᨶ ᨷ ᨸ ᨹ ᨺ ᨻ ᨼ ᨽ ᨾ ᨿ\
/// ᩀ ᩁ ᩂ ᩃ ᩄ ᩅ ᩆ ᩇ ᩈ ᩉ ᩊ ᩋ ᩌ ᩍ ᩎ ᩏ\
/// ᩐ ᩑ ᩒ ᩓ ᩔ ᩕ ᩖ ᩗ ᩘ ᩙ ᩚ ᩛ ᩜ ᩝ ᩞ ᩠\
/// ᩡ ᩢ ᩣ ᩤ ᩥ ᩦ ᩧ ᩨ ᩩ ᩪ ᩫ ᩬ ᩭ ᩮ ᩯ ᩰ\
/// ᩱ ᩲ ᩳ ᩴ ᩵ ᩶ ᩷ ᩸ ᩹ ᩺ ᩻ ᩼ ᩿ ᪀ ᪁ ᪂\
/// ᪃ ᪄ ᪅ ᪆ ᪇ ᪈ ᪉ ᪐ ᪑ ᪒ ᪓ ᪔ ᪕ ᪖ ᪗ ᪘\
/// ᪙ ᪠ ᪡ ᪢ ᪣ ᪤ ᪥ ᪦ ᪧ ᪨ ᪩ ᪪ ᪫ ᪬ ᪭\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{1a20}: 'ᨠ'
    pub const LETTER_HIGH_KA: char = 'ᨠ';
    /// \u{1a21}: 'ᨡ'
    pub const LETTER_HIGH_KHA: char = 'ᨡ';
    /// \u{1a22}: 'ᨢ'
    pub const LETTER_HIGH_KXA: char = 'ᨢ';
    /// \u{1a23}: 'ᨣ'
    pub const LETTER_LOW_KA: char = 'ᨣ';
    /// \u{1a24}: 'ᨤ'
    pub const LETTER_LOW_KXA: char = 'ᨤ';
    /// \u{1a25}: 'ᨥ'
    pub const LETTER_LOW_KHA: char = 'ᨥ';
    /// \u{1a26}: 'ᨦ'
    pub const LETTER_NGA: char = 'ᨦ';
    /// \u{1a27}: 'ᨧ'
    pub const LETTER_HIGH_CA: char = 'ᨧ';
    /// \u{1a28}: 'ᨨ'
    pub const LETTER_HIGH_CHA: char = 'ᨨ';
    /// \u{1a29}: 'ᨩ'
    pub const LETTER_LOW_CA: char = 'ᨩ';
    /// \u{1a2a}: 'ᨪ'
    pub const LETTER_LOW_SA: char = 'ᨪ';
    /// \u{1a2b}: 'ᨫ'
    pub const LETTER_LOW_CHA: char = 'ᨫ';
    /// \u{1a2c}: 'ᨬ'
    pub const LETTER_NYA: char = 'ᨬ';
    /// \u{1a2d}: 'ᨭ'
    pub const LETTER_RATA: char = 'ᨭ';
    /// \u{1a2e}: 'ᨮ'
    pub const LETTER_HIGH_RATHA: char = 'ᨮ';
    /// \u{1a2f}: 'ᨯ'
    pub const LETTER_DA: char = 'ᨯ';
    /// \u{1a30}: 'ᨰ'
    pub const LETTER_LOW_RATHA: char = 'ᨰ';
    /// \u{1a31}: 'ᨱ'
    pub const LETTER_RANA: char = 'ᨱ';
    /// \u{1a32}: 'ᨲ'
    pub const LETTER_HIGH_TA: char = 'ᨲ';
    /// \u{1a33}: 'ᨳ'
    pub const LETTER_HIGH_THA: char = 'ᨳ';
    /// \u{1a34}: 'ᨴ'
    pub const LETTER_LOW_TA: char = 'ᨴ';
    /// \u{1a35}: 'ᨵ'
    pub const LETTER_LOW_THA: char = 'ᨵ';
    /// \u{1a36}: 'ᨶ'
    pub const LETTER_NA: char = 'ᨶ';
    /// \u{1a37}: 'ᨷ'
    pub const LETTER_BA: char = 'ᨷ';
    /// \u{1a38}: 'ᨸ'
    pub const LETTER_HIGH_PA: char = 'ᨸ';
    /// \u{1a39}: 'ᨹ'
    pub const LETTER_HIGH_PHA: char = 'ᨹ';
    /// \u{1a3a}: 'ᨺ'
    pub const LETTER_HIGH_FA: char = 'ᨺ';
    /// \u{1a3b}: 'ᨻ'
    pub const LETTER_LOW_PA: char = 'ᨻ';
    /// \u{1a3c}: 'ᨼ'
    pub const LETTER_LOW_FA: char = 'ᨼ';
    /// \u{1a3d}: 'ᨽ'
    pub const LETTER_LOW_PHA: char = 'ᨽ';
    /// \u{1a3e}: 'ᨾ'
    pub const LETTER_MA: char = 'ᨾ';
    /// \u{1a3f}: 'ᨿ'
    pub const LETTER_LOW_YA: char = 'ᨿ';
    /// \u{1a40}: 'ᩀ'
    pub const LETTER_HIGH_YA: char = 'ᩀ';
    /// \u{1a41}: 'ᩁ'
    pub const LETTER_RA: char = 'ᩁ';
    /// \u{1a42}: 'ᩂ'
    pub const LETTER_RUE: char = 'ᩂ';
    /// \u{1a43}: 'ᩃ'
    pub const LETTER_LA: char = 'ᩃ';
    /// \u{1a44}: 'ᩄ'
    pub const LETTER_LUE: char = 'ᩄ';
    /// \u{1a45}: 'ᩅ'
    pub const LETTER_WA: char = 'ᩅ';
    /// \u{1a46}: 'ᩆ'
    pub const LETTER_HIGH_SHA: char = 'ᩆ';
    /// \u{1a47}: 'ᩇ'
    pub const LETTER_HIGH_SSA: char = 'ᩇ';
    /// \u{1a48}: 'ᩈ'
    pub const LETTER_HIGH_SA: char = 'ᩈ';
    /// \u{1a49}: 'ᩉ'
    pub const LETTER_HIGH_HA: char = 'ᩉ';
    /// \u{1a4a}: 'ᩊ'
    pub const LETTER_LLA: char = 'ᩊ';
    /// \u{1a4b}: 'ᩋ'
    pub const LETTER_A: char = 'ᩋ';
    /// \u{1a4c}: 'ᩌ'
    pub const LETTER_LOW_HA: char = 'ᩌ';
    /// \u{1a4d}: 'ᩍ'
    pub const LETTER_I: char = 'ᩍ';
    /// \u{1a4e}: 'ᩎ'
    pub const LETTER_II: char = 'ᩎ';
    /// \u{1a4f}: 'ᩏ'
    pub const LETTER_U: char = 'ᩏ';
    /// \u{1a50}: 'ᩐ'
    pub const LETTER_UU: char = 'ᩐ';
    /// \u{1a51}: 'ᩑ'
    pub const LETTER_EE: char = 'ᩑ';
    /// \u{1a52}: 'ᩒ'
    pub const LETTER_OO: char = 'ᩒ';
    /// \u{1a53}: 'ᩓ'
    pub const LETTER_LAE: char = 'ᩓ';
    /// \u{1a54}: 'ᩔ'
    pub const LETTER_GREAT_SA: char = 'ᩔ';
    /// \u{1a55}: 'ᩕ'
    pub const CONSONANT_SIGN_MEDIAL_RA: char = 'ᩕ';
    /// \u{1a56}: 'ᩖ'
    pub const CONSONANT_SIGN_MEDIAL_LA: char = 'ᩖ';
    /// \u{1a57}: 'ᩗ'
    pub const CONSONANT_SIGN_LA_TANG_LAI: char = 'ᩗ';
    /// \u{1a58}: 'ᩘ'
    pub const SIGN_MAI_KANG_LAI: char = 'ᩘ';
    /// \u{1a59}: 'ᩙ'
    pub const CONSONANT_SIGN_FINAL_NGA: char = 'ᩙ';
    /// \u{1a5a}: 'ᩚ'
    pub const CONSONANT_SIGN_LOW_PA: char = 'ᩚ';
    /// \u{1a5b}: 'ᩛ'
    pub const CONSONANT_SIGN_HIGH_RATHA_OR_LOW_PA: char = 'ᩛ';
    /// \u{1a5c}: 'ᩜ'
    pub const CONSONANT_SIGN_MA: char = 'ᩜ';
    /// \u{1a5d}: 'ᩝ'
    pub const CONSONANT_SIGN_BA: char = 'ᩝ';
    /// \u{1a5e}: 'ᩞ'
    pub const CONSONANT_SIGN_SA: char = 'ᩞ';
    /// \u{1a60}: '᩠'
    pub const SIGN_SAKOT: char = '᩠';
    /// \u{1a61}: 'ᩡ'
    pub const VOWEL_SIGN_A: char = 'ᩡ';
    /// \u{1a62}: 'ᩢ'
    pub const VOWEL_SIGN_MAI_SAT: char = 'ᩢ';
    /// \u{1a63}: 'ᩣ'
    pub const VOWEL_SIGN_AA: char = 'ᩣ';
    /// \u{1a64}: 'ᩤ'
    pub const VOWEL_SIGN_TALL_AA: char = 'ᩤ';
    /// \u{1a65}: 'ᩥ'
    pub const VOWEL_SIGN_I: char = 'ᩥ';
    /// \u{1a66}: 'ᩦ'
    pub const VOWEL_SIGN_II: char = 'ᩦ';
    /// \u{1a67}: 'ᩧ'
    pub const VOWEL_SIGN_UE: char = 'ᩧ';
    /// \u{1a68}: 'ᩨ'
    pub const VOWEL_SIGN_UUE: char = 'ᩨ';
    /// \u{1a69}: 'ᩩ'
    pub const VOWEL_SIGN_U: char = 'ᩩ';
    /// \u{1a6a}: 'ᩪ'
    pub const VOWEL_SIGN_UU: char = 'ᩪ';
    /// \u{1a6b}: 'ᩫ'
    pub const VOWEL_SIGN_O: char = 'ᩫ';
    /// \u{1a6c}: 'ᩬ'
    pub const VOWEL_SIGN_OA_BELOW: char = 'ᩬ';
    /// \u{1a6d}: 'ᩭ'
    pub const VOWEL_SIGN_OY: char = 'ᩭ';
    /// \u{1a6e}: 'ᩮ'
    pub const VOWEL_SIGN_E: char = 'ᩮ';
    /// \u{1a6f}: 'ᩯ'
    pub const VOWEL_SIGN_AE: char = 'ᩯ';
    /// \u{1a70}: 'ᩰ'
    pub const VOWEL_SIGN_OO: char = 'ᩰ';
    /// \u{1a71}: 'ᩱ'
    pub const VOWEL_SIGN_AI: char = 'ᩱ';
    /// \u{1a72}: 'ᩲ'
    pub const VOWEL_SIGN_THAM_AI: char = 'ᩲ';
    /// \u{1a73}: 'ᩳ'
    pub const VOWEL_SIGN_OA_ABOVE: char = 'ᩳ';
    /// \u{1a74}: 'ᩴ'
    pub const SIGN_MAI_KANG: char = 'ᩴ';
    /// \u{1a75}: '᩵'
    pub const SIGN_TONE_DASH_1: char = '᩵';
    /// \u{1a76}: '᩶'
    pub const SIGN_TONE_DASH_2: char = '᩶';
    /// \u{1a77}: '᩷'
    pub const SIGN_KHUEN_TONE_DASH_3: char = '᩷';
    /// \u{1a78}: '᩸'
    pub const SIGN_KHUEN_TONE_DASH_4: char = '᩸';
    /// \u{1a79}: '᩹'
    pub const SIGN_KHUEN_TONE_DASH_5: char = '᩹';
    /// \u{1a7a}: '᩺'
    pub const SIGN_RA_HAAM: char = '᩺';
    /// \u{1a7b}: '᩻'
    pub const SIGN_MAI_SAM: char = '᩻';
    /// \u{1a7c}: '᩼'
    pub const SIGN_KHUEN_DASH_LUE_KARAN: char = '᩼';
    /// \u{1a7f}: '᩿'
    pub const COMBINING_CRYPTOGRAMMIC_DOT: char = '᩿';
    /// \u{1a80}: '᪀'
    pub const HORA_DIGIT_ZERO: char = '᪀';
    /// \u{1a81}: '᪁'
    pub const HORA_DIGIT_ONE: char = '᪁';
    /// \u{1a82}: '᪂'
    pub const HORA_DIGIT_TWO: char = '᪂';
    /// \u{1a83}: '᪃'
    pub const HORA_DIGIT_THREE: char = '᪃';
    /// \u{1a84}: '᪄'
    pub const HORA_DIGIT_FOUR: char = '᪄';
    /// \u{1a85}: '᪅'
    pub const HORA_DIGIT_FIVE: char = '᪅';
    /// \u{1a86}: '᪆'
    pub const HORA_DIGIT_SIX: char = '᪆';
    /// \u{1a87}: '᪇'
    pub const HORA_DIGIT_SEVEN: char = '᪇';
    /// \u{1a88}: '᪈'
    pub const HORA_DIGIT_EIGHT: char = '᪈';
    /// \u{1a89}: '᪉'
    pub const HORA_DIGIT_NINE: char = '᪉';
    /// \u{1a90}: '᪐'
    pub const THAM_DIGIT_ZERO: char = '᪐';
    /// \u{1a91}: '᪑'
    pub const THAM_DIGIT_ONE: char = '᪑';
    /// \u{1a92}: '᪒'
    pub const THAM_DIGIT_TWO: char = '᪒';
    /// \u{1a93}: '᪓'
    pub const THAM_DIGIT_THREE: char = '᪓';
    /// \u{1a94}: '᪔'
    pub const THAM_DIGIT_FOUR: char = '᪔';
    /// \u{1a95}: '᪕'
    pub const THAM_DIGIT_FIVE: char = '᪕';
    /// \u{1a96}: '᪖'
    pub const THAM_DIGIT_SIX: char = '᪖';
    /// \u{1a97}: '᪗'
    pub const THAM_DIGIT_SEVEN: char = '᪗';
    /// \u{1a98}: '᪘'
    pub const THAM_DIGIT_EIGHT: char = '᪘';
    /// \u{1a99}: '᪙'
    pub const THAM_DIGIT_NINE: char = '᪙';
    /// \u{1aa0}: '᪠'
    pub const SIGN_WIANG: char = '᪠';
    /// \u{1aa1}: '᪡'
    pub const SIGN_WIANGWAAK: char = '᪡';
    /// \u{1aa2}: '᪢'
    pub const SIGN_SAWAN: char = '᪢';
    /// \u{1aa3}: '᪣'
    pub const SIGN_KEOW: char = '᪣';
    /// \u{1aa4}: '᪤'
    pub const SIGN_HOY: char = '᪤';
    /// \u{1aa5}: '᪥'
    pub const SIGN_DOKMAI: char = '᪥';
    /// \u{1aa6}: '᪦'
    pub const SIGN_REVERSED_ROTATED_RANA: char = '᪦';
    /// \u{1aa7}: 'ᪧ'
    pub const SIGN_MAI_YAMOK: char = 'ᪧ';
    /// \u{1aa8}: '᪨'
    pub const SIGN_KAAN: char = '᪨';
    /// \u{1aa9}: '᪩'
    pub const SIGN_KAANKUU: char = '᪩';
    /// \u{1aaa}: '᪪'
    pub const SIGN_SATKAAN: char = '᪪';
    /// \u{1aab}: '᪫'
    pub const SIGN_SATKAANKUU: char = '᪫';
    /// \u{1aac}: '᪬'
    pub const SIGN_HANG: char = '᪬';
    /// \u{1aad}: '᪭'
    pub const SIGN_CAANG: char = '᪭';
}

/// An enum to represent all characters in the TaiTham block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum TaiTham {
    /// \u{1a20}: 'ᨠ'
    LetterHighKa,
    /// \u{1a21}: 'ᨡ'
    LetterHighKha,
    /// \u{1a22}: 'ᨢ'
    LetterHighKxa,
    /// \u{1a23}: 'ᨣ'
    LetterLowKa,
    /// \u{1a24}: 'ᨤ'
    LetterLowKxa,
    /// \u{1a25}: 'ᨥ'
    LetterLowKha,
    /// \u{1a26}: 'ᨦ'
    LetterNga,
    /// \u{1a27}: 'ᨧ'
    LetterHighCa,
    /// \u{1a28}: 'ᨨ'
    LetterHighCha,
    /// \u{1a29}: 'ᨩ'
    LetterLowCa,
    /// \u{1a2a}: 'ᨪ'
    LetterLowSa,
    /// \u{1a2b}: 'ᨫ'
    LetterLowCha,
    /// \u{1a2c}: 'ᨬ'
    LetterNya,
    /// \u{1a2d}: 'ᨭ'
    LetterRata,
    /// \u{1a2e}: 'ᨮ'
    LetterHighRatha,
    /// \u{1a2f}: 'ᨯ'
    LetterDa,
    /// \u{1a30}: 'ᨰ'
    LetterLowRatha,
    /// \u{1a31}: 'ᨱ'
    LetterRana,
    /// \u{1a32}: 'ᨲ'
    LetterHighTa,
    /// \u{1a33}: 'ᨳ'
    LetterHighTha,
    /// \u{1a34}: 'ᨴ'
    LetterLowTa,
    /// \u{1a35}: 'ᨵ'
    LetterLowTha,
    /// \u{1a36}: 'ᨶ'
    LetterNa,
    /// \u{1a37}: 'ᨷ'
    LetterBa,
    /// \u{1a38}: 'ᨸ'
    LetterHighPa,
    /// \u{1a39}: 'ᨹ'
    LetterHighPha,
    /// \u{1a3a}: 'ᨺ'
    LetterHighFa,
    /// \u{1a3b}: 'ᨻ'
    LetterLowPa,
    /// \u{1a3c}: 'ᨼ'
    LetterLowFa,
    /// \u{1a3d}: 'ᨽ'
    LetterLowPha,
    /// \u{1a3e}: 'ᨾ'
    LetterMa,
    /// \u{1a3f}: 'ᨿ'
    LetterLowYa,
    /// \u{1a40}: 'ᩀ'
    LetterHighYa,
    /// \u{1a41}: 'ᩁ'
    LetterRa,
    /// \u{1a42}: 'ᩂ'
    LetterRue,
    /// \u{1a43}: 'ᩃ'
    LetterLa,
    /// \u{1a44}: 'ᩄ'
    LetterLue,
    /// \u{1a45}: 'ᩅ'
    LetterWa,
    /// \u{1a46}: 'ᩆ'
    LetterHighSha,
    /// \u{1a47}: 'ᩇ'
    LetterHighSsa,
    /// \u{1a48}: 'ᩈ'
    LetterHighSa,
    /// \u{1a49}: 'ᩉ'
    LetterHighHa,
    /// \u{1a4a}: 'ᩊ'
    LetterLla,
    /// \u{1a4b}: 'ᩋ'
    LetterA,
    /// \u{1a4c}: 'ᩌ'
    LetterLowHa,
    /// \u{1a4d}: 'ᩍ'
    LetterI,
    /// \u{1a4e}: 'ᩎ'
    LetterIi,
    /// \u{1a4f}: 'ᩏ'
    LetterU,
    /// \u{1a50}: 'ᩐ'
    LetterUu,
    /// \u{1a51}: 'ᩑ'
    LetterEe,
    /// \u{1a52}: 'ᩒ'
    LetterOo,
    /// \u{1a53}: 'ᩓ'
    LetterLae,
    /// \u{1a54}: 'ᩔ'
    LetterGreatSa,
    /// \u{1a55}: 'ᩕ'
    ConsonantSignMedialRa,
    /// \u{1a56}: 'ᩖ'
    ConsonantSignMedialLa,
    /// \u{1a57}: 'ᩗ'
    ConsonantSignLaTangLai,
    /// \u{1a58}: 'ᩘ'
    SignMaiKangLai,
    /// \u{1a59}: 'ᩙ'
    ConsonantSignFinalNga,
    /// \u{1a5a}: 'ᩚ'
    ConsonantSignLowPa,
    /// \u{1a5b}: 'ᩛ'
    ConsonantSignHighRathaOrLowPa,
    /// \u{1a5c}: 'ᩜ'
    ConsonantSignMa,
    /// \u{1a5d}: 'ᩝ'
    ConsonantSignBa,
    /// \u{1a5e}: 'ᩞ'
    ConsonantSignSa,
    /// \u{1a60}: '᩠'
    SignSakot,
    /// \u{1a61}: 'ᩡ'
    VowelSignA,
    /// \u{1a62}: 'ᩢ'
    VowelSignMaiSat,
    /// \u{1a63}: 'ᩣ'
    VowelSignAa,
    /// \u{1a64}: 'ᩤ'
    VowelSignTallAa,
    /// \u{1a65}: 'ᩥ'
    VowelSignI,
    /// \u{1a66}: 'ᩦ'
    VowelSignIi,
    /// \u{1a67}: 'ᩧ'
    VowelSignUe,
    /// \u{1a68}: 'ᩨ'
    VowelSignUue,
    /// \u{1a69}: 'ᩩ'
    VowelSignU,
    /// \u{1a6a}: 'ᩪ'
    VowelSignUu,
    /// \u{1a6b}: 'ᩫ'
    VowelSignO,
    /// \u{1a6c}: 'ᩬ'
    VowelSignOaBelow,
    /// \u{1a6d}: 'ᩭ'
    VowelSignOy,
    /// \u{1a6e}: 'ᩮ'
    VowelSignE,
    /// \u{1a6f}: 'ᩯ'
    VowelSignAe,
    /// \u{1a70}: 'ᩰ'
    VowelSignOo,
    /// \u{1a71}: 'ᩱ'
    VowelSignAi,
    /// \u{1a72}: 'ᩲ'
    VowelSignThamAi,
    /// \u{1a73}: 'ᩳ'
    VowelSignOaAbove,
    /// \u{1a74}: 'ᩴ'
    SignMaiKang,
    /// \u{1a75}: '᩵'
    SignToneDash1,
    /// \u{1a76}: '᩶'
    SignToneDash2,
    /// \u{1a77}: '᩷'
    SignKhuenToneDash3,
    /// \u{1a78}: '᩸'
    SignKhuenToneDash4,
    /// \u{1a79}: '᩹'
    SignKhuenToneDash5,
    /// \u{1a7a}: '᩺'
    SignRaHaam,
    /// \u{1a7b}: '᩻'
    SignMaiSam,
    /// \u{1a7c}: '᩼'
    SignKhuenDashLueKaran,
    /// \u{1a7f}: '᩿'
    CombiningCryptogrammicDot,
    /// \u{1a80}: '᪀'
    HoraDigitZero,
    /// \u{1a81}: '᪁'
    HoraDigitOne,
    /// \u{1a82}: '᪂'
    HoraDigitTwo,
    /// \u{1a83}: '᪃'
    HoraDigitThree,
    /// \u{1a84}: '᪄'
    HoraDigitFour,
    /// \u{1a85}: '᪅'
    HoraDigitFive,
    /// \u{1a86}: '᪆'
    HoraDigitSix,
    /// \u{1a87}: '᪇'
    HoraDigitSeven,
    /// \u{1a88}: '᪈'
    HoraDigitEight,
    /// \u{1a89}: '᪉'
    HoraDigitNine,
    /// \u{1a90}: '᪐'
    ThamDigitZero,
    /// \u{1a91}: '᪑'
    ThamDigitOne,
    /// \u{1a92}: '᪒'
    ThamDigitTwo,
    /// \u{1a93}: '᪓'
    ThamDigitThree,
    /// \u{1a94}: '᪔'
    ThamDigitFour,
    /// \u{1a95}: '᪕'
    ThamDigitFive,
    /// \u{1a96}: '᪖'
    ThamDigitSix,
    /// \u{1a97}: '᪗'
    ThamDigitSeven,
    /// \u{1a98}: '᪘'
    ThamDigitEight,
    /// \u{1a99}: '᪙'
    ThamDigitNine,
    /// \u{1aa0}: '᪠'
    SignWiang,
    /// \u{1aa1}: '᪡'
    SignWiangwaak,
    /// \u{1aa2}: '᪢'
    SignSawan,
    /// \u{1aa3}: '᪣'
    SignKeow,
    /// \u{1aa4}: '᪤'
    SignHoy,
    /// \u{1aa5}: '᪥'
    SignDokmai,
    /// \u{1aa6}: '᪦'
    SignReversedRotatedRana,
    /// \u{1aa7}: 'ᪧ'
    SignMaiYamok,
    /// \u{1aa8}: '᪨'
    SignKaan,
    /// \u{1aa9}: '᪩'
    SignKaankuu,
    /// \u{1aaa}: '᪪'
    SignSatkaan,
    /// \u{1aab}: '᪫'
    SignSatkaankuu,
    /// \u{1aac}: '᪬'
    SignHang,
    /// \u{1aad}: '᪭'
    SignCaang,
}

impl Into<char> for TaiTham {
    fn into(self) -> char {
        use constants::*;
        match self {
            TaiTham::LetterHighKa => LETTER_HIGH_KA,
            TaiTham::LetterHighKha => LETTER_HIGH_KHA,
            TaiTham::LetterHighKxa => LETTER_HIGH_KXA,
            TaiTham::LetterLowKa => LETTER_LOW_KA,
            TaiTham::LetterLowKxa => LETTER_LOW_KXA,
            TaiTham::LetterLowKha => LETTER_LOW_KHA,
            TaiTham::LetterNga => LETTER_NGA,
            TaiTham::LetterHighCa => LETTER_HIGH_CA,
            TaiTham::LetterHighCha => LETTER_HIGH_CHA,
            TaiTham::LetterLowCa => LETTER_LOW_CA,
            TaiTham::LetterLowSa => LETTER_LOW_SA,
            TaiTham::LetterLowCha => LETTER_LOW_CHA,
            TaiTham::LetterNya => LETTER_NYA,
            TaiTham::LetterRata => LETTER_RATA,
            TaiTham::LetterHighRatha => LETTER_HIGH_RATHA,
            TaiTham::LetterDa => LETTER_DA,
            TaiTham::LetterLowRatha => LETTER_LOW_RATHA,
            TaiTham::LetterRana => LETTER_RANA,
            TaiTham::LetterHighTa => LETTER_HIGH_TA,
            TaiTham::LetterHighTha => LETTER_HIGH_THA,
            TaiTham::LetterLowTa => LETTER_LOW_TA,
            TaiTham::LetterLowTha => LETTER_LOW_THA,
            TaiTham::LetterNa => LETTER_NA,
            TaiTham::LetterBa => LETTER_BA,
            TaiTham::LetterHighPa => LETTER_HIGH_PA,
            TaiTham::LetterHighPha => LETTER_HIGH_PHA,
            TaiTham::LetterHighFa => LETTER_HIGH_FA,
            TaiTham::LetterLowPa => LETTER_LOW_PA,
            TaiTham::LetterLowFa => LETTER_LOW_FA,
            TaiTham::LetterLowPha => LETTER_LOW_PHA,
            TaiTham::LetterMa => LETTER_MA,
            TaiTham::LetterLowYa => LETTER_LOW_YA,
            TaiTham::LetterHighYa => LETTER_HIGH_YA,
            TaiTham::LetterRa => LETTER_RA,
            TaiTham::LetterRue => LETTER_RUE,
            TaiTham::LetterLa => LETTER_LA,
            TaiTham::LetterLue => LETTER_LUE,
            TaiTham::LetterWa => LETTER_WA,
            TaiTham::LetterHighSha => LETTER_HIGH_SHA,
            TaiTham::LetterHighSsa => LETTER_HIGH_SSA,
            TaiTham::LetterHighSa => LETTER_HIGH_SA,
            TaiTham::LetterHighHa => LETTER_HIGH_HA,
            TaiTham::LetterLla => LETTER_LLA,
            TaiTham::LetterA => LETTER_A,
            TaiTham::LetterLowHa => LETTER_LOW_HA,
            TaiTham::LetterI => LETTER_I,
            TaiTham::LetterIi => LETTER_II,
            TaiTham::LetterU => LETTER_U,
            TaiTham::LetterUu => LETTER_UU,
            TaiTham::LetterEe => LETTER_EE,
            TaiTham::LetterOo => LETTER_OO,
            TaiTham::LetterLae => LETTER_LAE,
            TaiTham::LetterGreatSa => LETTER_GREAT_SA,
            TaiTham::ConsonantSignMedialRa => CONSONANT_SIGN_MEDIAL_RA,
            TaiTham::ConsonantSignMedialLa => CONSONANT_SIGN_MEDIAL_LA,
            TaiTham::ConsonantSignLaTangLai => CONSONANT_SIGN_LA_TANG_LAI,
            TaiTham::SignMaiKangLai => SIGN_MAI_KANG_LAI,
            TaiTham::ConsonantSignFinalNga => CONSONANT_SIGN_FINAL_NGA,
            TaiTham::ConsonantSignLowPa => CONSONANT_SIGN_LOW_PA,
            TaiTham::ConsonantSignHighRathaOrLowPa => CONSONANT_SIGN_HIGH_RATHA_OR_LOW_PA,
            TaiTham::ConsonantSignMa => CONSONANT_SIGN_MA,
            TaiTham::ConsonantSignBa => CONSONANT_SIGN_BA,
            TaiTham::ConsonantSignSa => CONSONANT_SIGN_SA,
            TaiTham::SignSakot => SIGN_SAKOT,
            TaiTham::VowelSignA => VOWEL_SIGN_A,
            TaiTham::VowelSignMaiSat => VOWEL_SIGN_MAI_SAT,
            TaiTham::VowelSignAa => VOWEL_SIGN_AA,
            TaiTham::VowelSignTallAa => VOWEL_SIGN_TALL_AA,
            TaiTham::VowelSignI => VOWEL_SIGN_I,
            TaiTham::VowelSignIi => VOWEL_SIGN_II,
            TaiTham::VowelSignUe => VOWEL_SIGN_UE,
            TaiTham::VowelSignUue => VOWEL_SIGN_UUE,
            TaiTham::VowelSignU => VOWEL_SIGN_U,
            TaiTham::VowelSignUu => VOWEL_SIGN_UU,
            TaiTham::VowelSignO => VOWEL_SIGN_O,
            TaiTham::VowelSignOaBelow => VOWEL_SIGN_OA_BELOW,
            TaiTham::VowelSignOy => VOWEL_SIGN_OY,
            TaiTham::VowelSignE => VOWEL_SIGN_E,
            TaiTham::VowelSignAe => VOWEL_SIGN_AE,
            TaiTham::VowelSignOo => VOWEL_SIGN_OO,
            TaiTham::VowelSignAi => VOWEL_SIGN_AI,
            TaiTham::VowelSignThamAi => VOWEL_SIGN_THAM_AI,
            TaiTham::VowelSignOaAbove => VOWEL_SIGN_OA_ABOVE,
            TaiTham::SignMaiKang => SIGN_MAI_KANG,
            TaiTham::SignToneDash1 => SIGN_TONE_DASH_1,
            TaiTham::SignToneDash2 => SIGN_TONE_DASH_2,
            TaiTham::SignKhuenToneDash3 => SIGN_KHUEN_TONE_DASH_3,
            TaiTham::SignKhuenToneDash4 => SIGN_KHUEN_TONE_DASH_4,
            TaiTham::SignKhuenToneDash5 => SIGN_KHUEN_TONE_DASH_5,
            TaiTham::SignRaHaam => SIGN_RA_HAAM,
            TaiTham::SignMaiSam => SIGN_MAI_SAM,
            TaiTham::SignKhuenDashLueKaran => SIGN_KHUEN_DASH_LUE_KARAN,
            TaiTham::CombiningCryptogrammicDot => COMBINING_CRYPTOGRAMMIC_DOT,
            TaiTham::HoraDigitZero => HORA_DIGIT_ZERO,
            TaiTham::HoraDigitOne => HORA_DIGIT_ONE,
            TaiTham::HoraDigitTwo => HORA_DIGIT_TWO,
            TaiTham::HoraDigitThree => HORA_DIGIT_THREE,
            TaiTham::HoraDigitFour => HORA_DIGIT_FOUR,
            TaiTham::HoraDigitFive => HORA_DIGIT_FIVE,
            TaiTham::HoraDigitSix => HORA_DIGIT_SIX,
            TaiTham::HoraDigitSeven => HORA_DIGIT_SEVEN,
            TaiTham::HoraDigitEight => HORA_DIGIT_EIGHT,
            TaiTham::HoraDigitNine => HORA_DIGIT_NINE,
            TaiTham::ThamDigitZero => THAM_DIGIT_ZERO,
            TaiTham::ThamDigitOne => THAM_DIGIT_ONE,
            TaiTham::ThamDigitTwo => THAM_DIGIT_TWO,
            TaiTham::ThamDigitThree => THAM_DIGIT_THREE,
            TaiTham::ThamDigitFour => THAM_DIGIT_FOUR,
            TaiTham::ThamDigitFive => THAM_DIGIT_FIVE,
            TaiTham::ThamDigitSix => THAM_DIGIT_SIX,
            TaiTham::ThamDigitSeven => THAM_DIGIT_SEVEN,
            TaiTham::ThamDigitEight => THAM_DIGIT_EIGHT,
            TaiTham::ThamDigitNine => THAM_DIGIT_NINE,
            TaiTham::SignWiang => SIGN_WIANG,
            TaiTham::SignWiangwaak => SIGN_WIANGWAAK,
            TaiTham::SignSawan => SIGN_SAWAN,
            TaiTham::SignKeow => SIGN_KEOW,
            TaiTham::SignHoy => SIGN_HOY,
            TaiTham::SignDokmai => SIGN_DOKMAI,
            TaiTham::SignReversedRotatedRana => SIGN_REVERSED_ROTATED_RANA,
            TaiTham::SignMaiYamok => SIGN_MAI_YAMOK,
            TaiTham::SignKaan => SIGN_KAAN,
            TaiTham::SignKaankuu => SIGN_KAANKUU,
            TaiTham::SignSatkaan => SIGN_SATKAAN,
            TaiTham::SignSatkaankuu => SIGN_SATKAANKUU,
            TaiTham::SignHang => SIGN_HANG,
            TaiTham::SignCaang => SIGN_CAANG,
        }
    }
}

impl std::convert::TryFrom<char> for TaiTham {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_HIGH_KA => Ok(TaiTham::LetterHighKa),
            LETTER_HIGH_KHA => Ok(TaiTham::LetterHighKha),
            LETTER_HIGH_KXA => Ok(TaiTham::LetterHighKxa),
            LETTER_LOW_KA => Ok(TaiTham::LetterLowKa),
            LETTER_LOW_KXA => Ok(TaiTham::LetterLowKxa),
            LETTER_LOW_KHA => Ok(TaiTham::LetterLowKha),
            LETTER_NGA => Ok(TaiTham::LetterNga),
            LETTER_HIGH_CA => Ok(TaiTham::LetterHighCa),
            LETTER_HIGH_CHA => Ok(TaiTham::LetterHighCha),
            LETTER_LOW_CA => Ok(TaiTham::LetterLowCa),
            LETTER_LOW_SA => Ok(TaiTham::LetterLowSa),
            LETTER_LOW_CHA => Ok(TaiTham::LetterLowCha),
            LETTER_NYA => Ok(TaiTham::LetterNya),
            LETTER_RATA => Ok(TaiTham::LetterRata),
            LETTER_HIGH_RATHA => Ok(TaiTham::LetterHighRatha),
            LETTER_DA => Ok(TaiTham::LetterDa),
            LETTER_LOW_RATHA => Ok(TaiTham::LetterLowRatha),
            LETTER_RANA => Ok(TaiTham::LetterRana),
            LETTER_HIGH_TA => Ok(TaiTham::LetterHighTa),
            LETTER_HIGH_THA => Ok(TaiTham::LetterHighTha),
            LETTER_LOW_TA => Ok(TaiTham::LetterLowTa),
            LETTER_LOW_THA => Ok(TaiTham::LetterLowTha),
            LETTER_NA => Ok(TaiTham::LetterNa),
            LETTER_BA => Ok(TaiTham::LetterBa),
            LETTER_HIGH_PA => Ok(TaiTham::LetterHighPa),
            LETTER_HIGH_PHA => Ok(TaiTham::LetterHighPha),
            LETTER_HIGH_FA => Ok(TaiTham::LetterHighFa),
            LETTER_LOW_PA => Ok(TaiTham::LetterLowPa),
            LETTER_LOW_FA => Ok(TaiTham::LetterLowFa),
            LETTER_LOW_PHA => Ok(TaiTham::LetterLowPha),
            LETTER_MA => Ok(TaiTham::LetterMa),
            LETTER_LOW_YA => Ok(TaiTham::LetterLowYa),
            LETTER_HIGH_YA => Ok(TaiTham::LetterHighYa),
            LETTER_RA => Ok(TaiTham::LetterRa),
            LETTER_RUE => Ok(TaiTham::LetterRue),
            LETTER_LA => Ok(TaiTham::LetterLa),
            LETTER_LUE => Ok(TaiTham::LetterLue),
            LETTER_WA => Ok(TaiTham::LetterWa),
            LETTER_HIGH_SHA => Ok(TaiTham::LetterHighSha),
            LETTER_HIGH_SSA => Ok(TaiTham::LetterHighSsa),
            LETTER_HIGH_SA => Ok(TaiTham::LetterHighSa),
            LETTER_HIGH_HA => Ok(TaiTham::LetterHighHa),
            LETTER_LLA => Ok(TaiTham::LetterLla),
            LETTER_A => Ok(TaiTham::LetterA),
            LETTER_LOW_HA => Ok(TaiTham::LetterLowHa),
            LETTER_I => Ok(TaiTham::LetterI),
            LETTER_II => Ok(TaiTham::LetterIi),
            LETTER_U => Ok(TaiTham::LetterU),
            LETTER_UU => Ok(TaiTham::LetterUu),
            LETTER_EE => Ok(TaiTham::LetterEe),
            LETTER_OO => Ok(TaiTham::LetterOo),
            LETTER_LAE => Ok(TaiTham::LetterLae),
            LETTER_GREAT_SA => Ok(TaiTham::LetterGreatSa),
            CONSONANT_SIGN_MEDIAL_RA => Ok(TaiTham::ConsonantSignMedialRa),
            CONSONANT_SIGN_MEDIAL_LA => Ok(TaiTham::ConsonantSignMedialLa),
            CONSONANT_SIGN_LA_TANG_LAI => Ok(TaiTham::ConsonantSignLaTangLai),
            SIGN_MAI_KANG_LAI => Ok(TaiTham::SignMaiKangLai),
            CONSONANT_SIGN_FINAL_NGA => Ok(TaiTham::ConsonantSignFinalNga),
            CONSONANT_SIGN_LOW_PA => Ok(TaiTham::ConsonantSignLowPa),
            CONSONANT_SIGN_HIGH_RATHA_OR_LOW_PA => Ok(TaiTham::ConsonantSignHighRathaOrLowPa),
            CONSONANT_SIGN_MA => Ok(TaiTham::ConsonantSignMa),
            CONSONANT_SIGN_BA => Ok(TaiTham::ConsonantSignBa),
            CONSONANT_SIGN_SA => Ok(TaiTham::ConsonantSignSa),
            SIGN_SAKOT => Ok(TaiTham::SignSakot),
            VOWEL_SIGN_A => Ok(TaiTham::VowelSignA),
            VOWEL_SIGN_MAI_SAT => Ok(TaiTham::VowelSignMaiSat),
            VOWEL_SIGN_AA => Ok(TaiTham::VowelSignAa),
            VOWEL_SIGN_TALL_AA => Ok(TaiTham::VowelSignTallAa),
            VOWEL_SIGN_I => Ok(TaiTham::VowelSignI),
            VOWEL_SIGN_II => Ok(TaiTham::VowelSignIi),
            VOWEL_SIGN_UE => Ok(TaiTham::VowelSignUe),
            VOWEL_SIGN_UUE => Ok(TaiTham::VowelSignUue),
            VOWEL_SIGN_U => Ok(TaiTham::VowelSignU),
            VOWEL_SIGN_UU => Ok(TaiTham::VowelSignUu),
            VOWEL_SIGN_O => Ok(TaiTham::VowelSignO),
            VOWEL_SIGN_OA_BELOW => Ok(TaiTham::VowelSignOaBelow),
            VOWEL_SIGN_OY => Ok(TaiTham::VowelSignOy),
            VOWEL_SIGN_E => Ok(TaiTham::VowelSignE),
            VOWEL_SIGN_AE => Ok(TaiTham::VowelSignAe),
            VOWEL_SIGN_OO => Ok(TaiTham::VowelSignOo),
            VOWEL_SIGN_AI => Ok(TaiTham::VowelSignAi),
            VOWEL_SIGN_THAM_AI => Ok(TaiTham::VowelSignThamAi),
            VOWEL_SIGN_OA_ABOVE => Ok(TaiTham::VowelSignOaAbove),
            SIGN_MAI_KANG => Ok(TaiTham::SignMaiKang),
            SIGN_TONE_DASH_1 => Ok(TaiTham::SignToneDash1),
            SIGN_TONE_DASH_2 => Ok(TaiTham::SignToneDash2),
            SIGN_KHUEN_TONE_DASH_3 => Ok(TaiTham::SignKhuenToneDash3),
            SIGN_KHUEN_TONE_DASH_4 => Ok(TaiTham::SignKhuenToneDash4),
            SIGN_KHUEN_TONE_DASH_5 => Ok(TaiTham::SignKhuenToneDash5),
            SIGN_RA_HAAM => Ok(TaiTham::SignRaHaam),
            SIGN_MAI_SAM => Ok(TaiTham::SignMaiSam),
            SIGN_KHUEN_DASH_LUE_KARAN => Ok(TaiTham::SignKhuenDashLueKaran),
            COMBINING_CRYPTOGRAMMIC_DOT => Ok(TaiTham::CombiningCryptogrammicDot),
            HORA_DIGIT_ZERO => Ok(TaiTham::HoraDigitZero),
            HORA_DIGIT_ONE => Ok(TaiTham::HoraDigitOne),
            HORA_DIGIT_TWO => Ok(TaiTham::HoraDigitTwo),
            HORA_DIGIT_THREE => Ok(TaiTham::HoraDigitThree),
            HORA_DIGIT_FOUR => Ok(TaiTham::HoraDigitFour),
            HORA_DIGIT_FIVE => Ok(TaiTham::HoraDigitFive),
            HORA_DIGIT_SIX => Ok(TaiTham::HoraDigitSix),
            HORA_DIGIT_SEVEN => Ok(TaiTham::HoraDigitSeven),
            HORA_DIGIT_EIGHT => Ok(TaiTham::HoraDigitEight),
            HORA_DIGIT_NINE => Ok(TaiTham::HoraDigitNine),
            THAM_DIGIT_ZERO => Ok(TaiTham::ThamDigitZero),
            THAM_DIGIT_ONE => Ok(TaiTham::ThamDigitOne),
            THAM_DIGIT_TWO => Ok(TaiTham::ThamDigitTwo),
            THAM_DIGIT_THREE => Ok(TaiTham::ThamDigitThree),
            THAM_DIGIT_FOUR => Ok(TaiTham::ThamDigitFour),
            THAM_DIGIT_FIVE => Ok(TaiTham::ThamDigitFive),
            THAM_DIGIT_SIX => Ok(TaiTham::ThamDigitSix),
            THAM_DIGIT_SEVEN => Ok(TaiTham::ThamDigitSeven),
            THAM_DIGIT_EIGHT => Ok(TaiTham::ThamDigitEight),
            THAM_DIGIT_NINE => Ok(TaiTham::ThamDigitNine),
            SIGN_WIANG => Ok(TaiTham::SignWiang),
            SIGN_WIANGWAAK => Ok(TaiTham::SignWiangwaak),
            SIGN_SAWAN => Ok(TaiTham::SignSawan),
            SIGN_KEOW => Ok(TaiTham::SignKeow),
            SIGN_HOY => Ok(TaiTham::SignHoy),
            SIGN_DOKMAI => Ok(TaiTham::SignDokmai),
            SIGN_REVERSED_ROTATED_RANA => Ok(TaiTham::SignReversedRotatedRana),
            SIGN_MAI_YAMOK => Ok(TaiTham::SignMaiYamok),
            SIGN_KAAN => Ok(TaiTham::SignKaan),
            SIGN_KAANKUU => Ok(TaiTham::SignKaankuu),
            SIGN_SATKAAN => Ok(TaiTham::SignSatkaan),
            SIGN_SATKAANKUU => Ok(TaiTham::SignSatkaankuu),
            SIGN_HANG => Ok(TaiTham::SignHang),
            SIGN_CAANG => Ok(TaiTham::SignCaang),
            _ => Err(()),
        }
    }
}

impl Into<u32> for TaiTham {
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

impl std::convert::TryFrom<u32> for TaiTham {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for TaiTham {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl TaiTham {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        TaiTham::LetterHighKa
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            TaiTham::LetterHighKa => "tai tham letter high ka",
            TaiTham::LetterHighKha => "tai tham letter high kha",
            TaiTham::LetterHighKxa => "tai tham letter high kxa",
            TaiTham::LetterLowKa => "tai tham letter low ka",
            TaiTham::LetterLowKxa => "tai tham letter low kxa",
            TaiTham::LetterLowKha => "tai tham letter low kha",
            TaiTham::LetterNga => "tai tham letter nga",
            TaiTham::LetterHighCa => "tai tham letter high ca",
            TaiTham::LetterHighCha => "tai tham letter high cha",
            TaiTham::LetterLowCa => "tai tham letter low ca",
            TaiTham::LetterLowSa => "tai tham letter low sa",
            TaiTham::LetterLowCha => "tai tham letter low cha",
            TaiTham::LetterNya => "tai tham letter nya",
            TaiTham::LetterRata => "tai tham letter rata",
            TaiTham::LetterHighRatha => "tai tham letter high ratha",
            TaiTham::LetterDa => "tai tham letter da",
            TaiTham::LetterLowRatha => "tai tham letter low ratha",
            TaiTham::LetterRana => "tai tham letter rana",
            TaiTham::LetterHighTa => "tai tham letter high ta",
            TaiTham::LetterHighTha => "tai tham letter high tha",
            TaiTham::LetterLowTa => "tai tham letter low ta",
            TaiTham::LetterLowTha => "tai tham letter low tha",
            TaiTham::LetterNa => "tai tham letter na",
            TaiTham::LetterBa => "tai tham letter ba",
            TaiTham::LetterHighPa => "tai tham letter high pa",
            TaiTham::LetterHighPha => "tai tham letter high pha",
            TaiTham::LetterHighFa => "tai tham letter high fa",
            TaiTham::LetterLowPa => "tai tham letter low pa",
            TaiTham::LetterLowFa => "tai tham letter low fa",
            TaiTham::LetterLowPha => "tai tham letter low pha",
            TaiTham::LetterMa => "tai tham letter ma",
            TaiTham::LetterLowYa => "tai tham letter low ya",
            TaiTham::LetterHighYa => "tai tham letter high ya",
            TaiTham::LetterRa => "tai tham letter ra",
            TaiTham::LetterRue => "tai tham letter rue",
            TaiTham::LetterLa => "tai tham letter la",
            TaiTham::LetterLue => "tai tham letter lue",
            TaiTham::LetterWa => "tai tham letter wa",
            TaiTham::LetterHighSha => "tai tham letter high sha",
            TaiTham::LetterHighSsa => "tai tham letter high ssa",
            TaiTham::LetterHighSa => "tai tham letter high sa",
            TaiTham::LetterHighHa => "tai tham letter high ha",
            TaiTham::LetterLla => "tai tham letter lla",
            TaiTham::LetterA => "tai tham letter a",
            TaiTham::LetterLowHa => "tai tham letter low ha",
            TaiTham::LetterI => "tai tham letter i",
            TaiTham::LetterIi => "tai tham letter ii",
            TaiTham::LetterU => "tai tham letter u",
            TaiTham::LetterUu => "tai tham letter uu",
            TaiTham::LetterEe => "tai tham letter ee",
            TaiTham::LetterOo => "tai tham letter oo",
            TaiTham::LetterLae => "tai tham letter lae",
            TaiTham::LetterGreatSa => "tai tham letter great sa",
            TaiTham::ConsonantSignMedialRa => "tai tham consonant sign medial ra",
            TaiTham::ConsonantSignMedialLa => "tai tham consonant sign medial la",
            TaiTham::ConsonantSignLaTangLai => "tai tham consonant sign la tang lai",
            TaiTham::SignMaiKangLai => "tai tham sign mai kang lai",
            TaiTham::ConsonantSignFinalNga => "tai tham consonant sign final nga",
            TaiTham::ConsonantSignLowPa => "tai tham consonant sign low pa",
            TaiTham::ConsonantSignHighRathaOrLowPa => "tai tham consonant sign high ratha or low pa",
            TaiTham::ConsonantSignMa => "tai tham consonant sign ma",
            TaiTham::ConsonantSignBa => "tai tham consonant sign ba",
            TaiTham::ConsonantSignSa => "tai tham consonant sign sa",
            TaiTham::SignSakot => "tai tham sign sakot",
            TaiTham::VowelSignA => "tai tham vowel sign a",
            TaiTham::VowelSignMaiSat => "tai tham vowel sign mai sat",
            TaiTham::VowelSignAa => "tai tham vowel sign aa",
            TaiTham::VowelSignTallAa => "tai tham vowel sign tall aa",
            TaiTham::VowelSignI => "tai tham vowel sign i",
            TaiTham::VowelSignIi => "tai tham vowel sign ii",
            TaiTham::VowelSignUe => "tai tham vowel sign ue",
            TaiTham::VowelSignUue => "tai tham vowel sign uue",
            TaiTham::VowelSignU => "tai tham vowel sign u",
            TaiTham::VowelSignUu => "tai tham vowel sign uu",
            TaiTham::VowelSignO => "tai tham vowel sign o",
            TaiTham::VowelSignOaBelow => "tai tham vowel sign oa below",
            TaiTham::VowelSignOy => "tai tham vowel sign oy",
            TaiTham::VowelSignE => "tai tham vowel sign e",
            TaiTham::VowelSignAe => "tai tham vowel sign ae",
            TaiTham::VowelSignOo => "tai tham vowel sign oo",
            TaiTham::VowelSignAi => "tai tham vowel sign ai",
            TaiTham::VowelSignThamAi => "tai tham vowel sign tham ai",
            TaiTham::VowelSignOaAbove => "tai tham vowel sign oa above",
            TaiTham::SignMaiKang => "tai tham sign mai kang",
            TaiTham::SignToneDash1 => "tai tham sign tone-1",
            TaiTham::SignToneDash2 => "tai tham sign tone-2",
            TaiTham::SignKhuenToneDash3 => "tai tham sign khuen tone-3",
            TaiTham::SignKhuenToneDash4 => "tai tham sign khuen tone-4",
            TaiTham::SignKhuenToneDash5 => "tai tham sign khuen tone-5",
            TaiTham::SignRaHaam => "tai tham sign ra haam",
            TaiTham::SignMaiSam => "tai tham sign mai sam",
            TaiTham::SignKhuenDashLueKaran => "tai tham sign khuen-lue karan",
            TaiTham::CombiningCryptogrammicDot => "tai tham combining cryptogrammic dot",
            TaiTham::HoraDigitZero => "tai tham hora digit zero",
            TaiTham::HoraDigitOne => "tai tham hora digit one",
            TaiTham::HoraDigitTwo => "tai tham hora digit two",
            TaiTham::HoraDigitThree => "tai tham hora digit three",
            TaiTham::HoraDigitFour => "tai tham hora digit four",
            TaiTham::HoraDigitFive => "tai tham hora digit five",
            TaiTham::HoraDigitSix => "tai tham hora digit six",
            TaiTham::HoraDigitSeven => "tai tham hora digit seven",
            TaiTham::HoraDigitEight => "tai tham hora digit eight",
            TaiTham::HoraDigitNine => "tai tham hora digit nine",
            TaiTham::ThamDigitZero => "tai tham tham digit zero",
            TaiTham::ThamDigitOne => "tai tham tham digit one",
            TaiTham::ThamDigitTwo => "tai tham tham digit two",
            TaiTham::ThamDigitThree => "tai tham tham digit three",
            TaiTham::ThamDigitFour => "tai tham tham digit four",
            TaiTham::ThamDigitFive => "tai tham tham digit five",
            TaiTham::ThamDigitSix => "tai tham tham digit six",
            TaiTham::ThamDigitSeven => "tai tham tham digit seven",
            TaiTham::ThamDigitEight => "tai tham tham digit eight",
            TaiTham::ThamDigitNine => "tai tham tham digit nine",
            TaiTham::SignWiang => "tai tham sign wiang",
            TaiTham::SignWiangwaak => "tai tham sign wiangwaak",
            TaiTham::SignSawan => "tai tham sign sawan",
            TaiTham::SignKeow => "tai tham sign keow",
            TaiTham::SignHoy => "tai tham sign hoy",
            TaiTham::SignDokmai => "tai tham sign dokmai",
            TaiTham::SignReversedRotatedRana => "tai tham sign reversed rotated rana",
            TaiTham::SignMaiYamok => "tai tham sign mai yamok",
            TaiTham::SignKaan => "tai tham sign kaan",
            TaiTham::SignKaankuu => "tai tham sign kaankuu",
            TaiTham::SignSatkaan => "tai tham sign satkaan",
            TaiTham::SignSatkaankuu => "tai tham sign satkaankuu",
            TaiTham::SignHang => "tai tham sign hang",
            TaiTham::SignCaang => "tai tham sign caang",
        }
    }
}
