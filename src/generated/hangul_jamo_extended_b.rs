/// \u{d7b0} → \u{d7ff}
///
/// ힰ ힱ ힲ ힳ ힴ ힵ ힶ ힷ ힸ ힹ ힺ ힻ ힼ ힽ ힾ ힿ\
/// ퟀ ퟁ ퟂ ퟃ ퟄ ퟅ ퟆ ퟋ ퟌ ퟍ ퟎ ퟏ ퟐ ퟑ ퟒ ퟓ\
/// ퟔ ퟕ ퟖ ퟗ ퟘ ퟙ ퟚ ퟛ ퟜ ퟝ ퟞ ퟟ ퟠ ퟡ ퟢ ퟣ\
/// ퟤ ퟥ ퟦ ퟧ ퟨ ퟩ ퟪ ퟫ ퟬ ퟭ ퟮ ퟯ ퟰ ퟱ ퟲ ퟳ\
/// ퟴ ퟵ ퟶ ퟷ ퟸ ퟹ ퟺ ퟻ\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{d7b0}: 'ힰ'
    pub const HANGUL_JUNGSEONG_O_DASH_YEO: char = 'ힰ';
    /// \u{d7b1}: 'ힱ'
    pub const HANGUL_JUNGSEONG_O_DASH_O_DASH_I: char = 'ힱ';
    /// \u{d7b2}: 'ힲ'
    pub const HANGUL_JUNGSEONG_YO_DASH_A: char = 'ힲ';
    /// \u{d7b3}: 'ힳ'
    pub const HANGUL_JUNGSEONG_YO_DASH_AE: char = 'ힳ';
    /// \u{d7b4}: 'ힴ'
    pub const HANGUL_JUNGSEONG_YO_DASH_EO: char = 'ힴ';
    /// \u{d7b5}: 'ힵ'
    pub const HANGUL_JUNGSEONG_U_DASH_YEO: char = 'ힵ';
    /// \u{d7b6}: 'ힶ'
    pub const HANGUL_JUNGSEONG_U_DASH_I_DASH_I: char = 'ힶ';
    /// \u{d7b7}: 'ힷ'
    pub const HANGUL_JUNGSEONG_YU_DASH_AE: char = 'ힷ';
    /// \u{d7b8}: 'ힸ'
    pub const HANGUL_JUNGSEONG_YU_DASH_O: char = 'ힸ';
    /// \u{d7b9}: 'ힹ'
    pub const HANGUL_JUNGSEONG_EU_DASH_A: char = 'ힹ';
    /// \u{d7ba}: 'ힺ'
    pub const HANGUL_JUNGSEONG_EU_DASH_EO: char = 'ힺ';
    /// \u{d7bb}: 'ힻ'
    pub const HANGUL_JUNGSEONG_EU_DASH_E: char = 'ힻ';
    /// \u{d7bc}: 'ힼ'
    pub const HANGUL_JUNGSEONG_EU_DASH_O: char = 'ힼ';
    /// \u{d7bd}: 'ힽ'
    pub const HANGUL_JUNGSEONG_I_DASH_YA_DASH_O: char = 'ힽ';
    /// \u{d7be}: 'ힾ'
    pub const HANGUL_JUNGSEONG_I_DASH_YAE: char = 'ힾ';
    /// \u{d7bf}: 'ힿ'
    pub const HANGUL_JUNGSEONG_I_DASH_YEO: char = 'ힿ';
    /// \u{d7c0}: 'ퟀ'
    pub const HANGUL_JUNGSEONG_I_DASH_YE: char = 'ퟀ';
    /// \u{d7c1}: 'ퟁ'
    pub const HANGUL_JUNGSEONG_I_DASH_O_DASH_I: char = 'ퟁ';
    /// \u{d7c2}: 'ퟂ'
    pub const HANGUL_JUNGSEONG_I_DASH_YO: char = 'ퟂ';
    /// \u{d7c3}: 'ퟃ'
    pub const HANGUL_JUNGSEONG_I_DASH_YU: char = 'ퟃ';
    /// \u{d7c4}: 'ퟄ'
    pub const HANGUL_JUNGSEONG_I_DASH_I: char = 'ퟄ';
    /// \u{d7c5}: 'ퟅ'
    pub const HANGUL_JUNGSEONG_ARAEA_DASH_A: char = 'ퟅ';
    /// \u{d7c6}: 'ퟆ'
    pub const HANGUL_JUNGSEONG_ARAEA_DASH_E: char = 'ퟆ';
    /// \u{d7cb}: 'ퟋ'
    pub const HANGUL_JONGSEONG_NIEUN_DASH_RIEUL: char = 'ퟋ';
    /// \u{d7cc}: 'ퟌ'
    pub const HANGUL_JONGSEONG_NIEUN_DASH_CHIEUCH: char = 'ퟌ';
    /// \u{d7cd}: 'ퟍ'
    pub const HANGUL_JONGSEONG_SSANGTIKEUT: char = 'ퟍ';
    /// \u{d7ce}: 'ퟎ'
    pub const HANGUL_JONGSEONG_SSANGTIKEUT_DASH_PIEUP: char = 'ퟎ';
    /// \u{d7cf}: 'ퟏ'
    pub const HANGUL_JONGSEONG_TIKEUT_DASH_PIEUP: char = 'ퟏ';
    /// \u{d7d0}: 'ퟐ'
    pub const HANGUL_JONGSEONG_TIKEUT_DASH_SIOS: char = 'ퟐ';
    /// \u{d7d1}: 'ퟑ'
    pub const HANGUL_JONGSEONG_TIKEUT_DASH_SIOS_DASH_KIYEOK: char = 'ퟑ';
    /// \u{d7d2}: 'ퟒ'
    pub const HANGUL_JONGSEONG_TIKEUT_DASH_CIEUC: char = 'ퟒ';
    /// \u{d7d3}: 'ퟓ'
    pub const HANGUL_JONGSEONG_TIKEUT_DASH_CHIEUCH: char = 'ퟓ';
    /// \u{d7d4}: 'ퟔ'
    pub const HANGUL_JONGSEONG_TIKEUT_DASH_THIEUTH: char = 'ퟔ';
    /// \u{d7d5}: 'ퟕ'
    pub const HANGUL_JONGSEONG_RIEUL_DASH_SSANGKIYEOK: char = 'ퟕ';
    /// \u{d7d6}: 'ퟖ'
    pub const HANGUL_JONGSEONG_RIEUL_DASH_KIYEOK_DASH_HIEUH: char = 'ퟖ';
    /// \u{d7d7}: 'ퟗ'
    pub const HANGUL_JONGSEONG_SSANGRIEUL_DASH_KHIEUKH: char = 'ퟗ';
    /// \u{d7d8}: 'ퟘ'
    pub const HANGUL_JONGSEONG_RIEUL_DASH_MIEUM_DASH_HIEUH: char = 'ퟘ';
    /// \u{d7d9}: 'ퟙ'
    pub const HANGUL_JONGSEONG_RIEUL_DASH_PIEUP_DASH_TIKEUT: char = 'ퟙ';
    /// \u{d7da}: 'ퟚ'
    pub const HANGUL_JONGSEONG_RIEUL_DASH_PIEUP_DASH_PHIEUPH: char = 'ퟚ';
    /// \u{d7db}: 'ퟛ'
    pub const HANGUL_JONGSEONG_RIEUL_DASH_YESIEUNG: char = 'ퟛ';
    /// \u{d7dc}: 'ퟜ'
    pub const HANGUL_JONGSEONG_RIEUL_DASH_YEORINHIEUH_DASH_HIEUH: char = 'ퟜ';
    /// \u{d7dd}: 'ퟝ'
    pub const HANGUL_JONGSEONG_KAPYEOUNRIEUL: char = 'ퟝ';
    /// \u{d7de}: 'ퟞ'
    pub const HANGUL_JONGSEONG_MIEUM_DASH_NIEUN: char = 'ퟞ';
    /// \u{d7df}: 'ퟟ'
    pub const HANGUL_JONGSEONG_MIEUM_DASH_SSANGNIEUN: char = 'ퟟ';
    /// \u{d7e0}: 'ퟠ'
    pub const HANGUL_JONGSEONG_SSANGMIEUM: char = 'ퟠ';
    /// \u{d7e1}: 'ퟡ'
    pub const HANGUL_JONGSEONG_MIEUM_DASH_PIEUP_DASH_SIOS: char = 'ퟡ';
    /// \u{d7e2}: 'ퟢ'
    pub const HANGUL_JONGSEONG_MIEUM_DASH_CIEUC: char = 'ퟢ';
    /// \u{d7e3}: 'ퟣ'
    pub const HANGUL_JONGSEONG_PIEUP_DASH_TIKEUT: char = 'ퟣ';
    /// \u{d7e4}: 'ퟤ'
    pub const HANGUL_JONGSEONG_PIEUP_DASH_RIEUL_DASH_PHIEUPH: char = 'ퟤ';
    /// \u{d7e5}: 'ퟥ'
    pub const HANGUL_JONGSEONG_PIEUP_DASH_MIEUM: char = 'ퟥ';
    /// \u{d7e6}: 'ퟦ'
    pub const HANGUL_JONGSEONG_SSANGPIEUP: char = 'ퟦ';
    /// \u{d7e7}: 'ퟧ'
    pub const HANGUL_JONGSEONG_PIEUP_DASH_SIOS_DASH_TIKEUT: char = 'ퟧ';
    /// \u{d7e8}: 'ퟨ'
    pub const HANGUL_JONGSEONG_PIEUP_DASH_CIEUC: char = 'ퟨ';
    /// \u{d7e9}: 'ퟩ'
    pub const HANGUL_JONGSEONG_PIEUP_DASH_CHIEUCH: char = 'ퟩ';
    /// \u{d7ea}: 'ퟪ'
    pub const HANGUL_JONGSEONG_SIOS_DASH_MIEUM: char = 'ퟪ';
    /// \u{d7eb}: 'ퟫ'
    pub const HANGUL_JONGSEONG_SIOS_DASH_KAPYEOUNPIEUP: char = 'ퟫ';
    /// \u{d7ec}: 'ퟬ'
    pub const HANGUL_JONGSEONG_SSANGSIOS_DASH_KIYEOK: char = 'ퟬ';
    /// \u{d7ed}: 'ퟭ'
    pub const HANGUL_JONGSEONG_SSANGSIOS_DASH_TIKEUT: char = 'ퟭ';
    /// \u{d7ee}: 'ퟮ'
    pub const HANGUL_JONGSEONG_SIOS_DASH_PANSIOS: char = 'ퟮ';
    /// \u{d7ef}: 'ퟯ'
    pub const HANGUL_JONGSEONG_SIOS_DASH_CIEUC: char = 'ퟯ';
    /// \u{d7f0}: 'ퟰ'
    pub const HANGUL_JONGSEONG_SIOS_DASH_CHIEUCH: char = 'ퟰ';
    /// \u{d7f1}: 'ퟱ'
    pub const HANGUL_JONGSEONG_SIOS_DASH_THIEUTH: char = 'ퟱ';
    /// \u{d7f2}: 'ퟲ'
    pub const HANGUL_JONGSEONG_SIOS_DASH_HIEUH: char = 'ퟲ';
    /// \u{d7f3}: 'ퟳ'
    pub const HANGUL_JONGSEONG_PANSIOS_DASH_PIEUP: char = 'ퟳ';
    /// \u{d7f4}: 'ퟴ'
    pub const HANGUL_JONGSEONG_PANSIOS_DASH_KAPYEOUNPIEUP: char = 'ퟴ';
    /// \u{d7f5}: 'ퟵ'
    pub const HANGUL_JONGSEONG_YESIEUNG_DASH_MIEUM: char = 'ퟵ';
    /// \u{d7f6}: 'ퟶ'
    pub const HANGUL_JONGSEONG_YESIEUNG_DASH_HIEUH: char = 'ퟶ';
    /// \u{d7f7}: 'ퟷ'
    pub const HANGUL_JONGSEONG_CIEUC_DASH_PIEUP: char = 'ퟷ';
    /// \u{d7f8}: 'ퟸ'
    pub const HANGUL_JONGSEONG_CIEUC_DASH_SSANGPIEUP: char = 'ퟸ';
    /// \u{d7f9}: 'ퟹ'
    pub const HANGUL_JONGSEONG_SSANGCIEUC: char = 'ퟹ';
    /// \u{d7fa}: 'ퟺ'
    pub const HANGUL_JONGSEONG_PHIEUPH_DASH_SIOS: char = 'ퟺ';
    /// \u{d7fb}: 'ퟻ'
    pub const HANGUL_JONGSEONG_PHIEUPH_DASH_THIEUTH: char = 'ퟻ';
}

/// An enum to represent all characters in the HangulJamoExtendedB block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum HangulJamoExtendedB {
    /// \u{d7b0}: 'ힰ'
    HangulJungseongODashYeo,
    /// \u{d7b1}: 'ힱ'
    HangulJungseongODashODashI,
    /// \u{d7b2}: 'ힲ'
    HangulJungseongYoDashA,
    /// \u{d7b3}: 'ힳ'
    HangulJungseongYoDashAe,
    /// \u{d7b4}: 'ힴ'
    HangulJungseongYoDashEo,
    /// \u{d7b5}: 'ힵ'
    HangulJungseongUDashYeo,
    /// \u{d7b6}: 'ힶ'
    HangulJungseongUDashIDashI,
    /// \u{d7b7}: 'ힷ'
    HangulJungseongYuDashAe,
    /// \u{d7b8}: 'ힸ'
    HangulJungseongYuDashO,
    /// \u{d7b9}: 'ힹ'
    HangulJungseongEuDashA,
    /// \u{d7ba}: 'ힺ'
    HangulJungseongEuDashEo,
    /// \u{d7bb}: 'ힻ'
    HangulJungseongEuDashE,
    /// \u{d7bc}: 'ힼ'
    HangulJungseongEuDashO,
    /// \u{d7bd}: 'ힽ'
    HangulJungseongIDashYaDashO,
    /// \u{d7be}: 'ힾ'
    HangulJungseongIDashYae,
    /// \u{d7bf}: 'ힿ'
    HangulJungseongIDashYeo,
    /// \u{d7c0}: 'ퟀ'
    HangulJungseongIDashYe,
    /// \u{d7c1}: 'ퟁ'
    HangulJungseongIDashODashI,
    /// \u{d7c2}: 'ퟂ'
    HangulJungseongIDashYo,
    /// \u{d7c3}: 'ퟃ'
    HangulJungseongIDashYu,
    /// \u{d7c4}: 'ퟄ'
    HangulJungseongIDashI,
    /// \u{d7c5}: 'ퟅ'
    HangulJungseongAraeaDashA,
    /// \u{d7c6}: 'ퟆ'
    HangulJungseongAraeaDashE,
    /// \u{d7cb}: 'ퟋ'
    HangulJongseongNieunDashRieul,
    /// \u{d7cc}: 'ퟌ'
    HangulJongseongNieunDashChieuch,
    /// \u{d7cd}: 'ퟍ'
    HangulJongseongSsangtikeut,
    /// \u{d7ce}: 'ퟎ'
    HangulJongseongSsangtikeutDashPieup,
    /// \u{d7cf}: 'ퟏ'
    HangulJongseongTikeutDashPieup,
    /// \u{d7d0}: 'ퟐ'
    HangulJongseongTikeutDashSios,
    /// \u{d7d1}: 'ퟑ'
    HangulJongseongTikeutDashSiosDashKiyeok,
    /// \u{d7d2}: 'ퟒ'
    HangulJongseongTikeutDashCieuc,
    /// \u{d7d3}: 'ퟓ'
    HangulJongseongTikeutDashChieuch,
    /// \u{d7d4}: 'ퟔ'
    HangulJongseongTikeutDashThieuth,
    /// \u{d7d5}: 'ퟕ'
    HangulJongseongRieulDashSsangkiyeok,
    /// \u{d7d6}: 'ퟖ'
    HangulJongseongRieulDashKiyeokDashHieuh,
    /// \u{d7d7}: 'ퟗ'
    HangulJongseongSsangrieulDashKhieukh,
    /// \u{d7d8}: 'ퟘ'
    HangulJongseongRieulDashMieumDashHieuh,
    /// \u{d7d9}: 'ퟙ'
    HangulJongseongRieulDashPieupDashTikeut,
    /// \u{d7da}: 'ퟚ'
    HangulJongseongRieulDashPieupDashPhieuph,
    /// \u{d7db}: 'ퟛ'
    HangulJongseongRieulDashYesieung,
    /// \u{d7dc}: 'ퟜ'
    HangulJongseongRieulDashYeorinhieuhDashHieuh,
    /// \u{d7dd}: 'ퟝ'
    HangulJongseongKapyeounrieul,
    /// \u{d7de}: 'ퟞ'
    HangulJongseongMieumDashNieun,
    /// \u{d7df}: 'ퟟ'
    HangulJongseongMieumDashSsangnieun,
    /// \u{d7e0}: 'ퟠ'
    HangulJongseongSsangmieum,
    /// \u{d7e1}: 'ퟡ'
    HangulJongseongMieumDashPieupDashSios,
    /// \u{d7e2}: 'ퟢ'
    HangulJongseongMieumDashCieuc,
    /// \u{d7e3}: 'ퟣ'
    HangulJongseongPieupDashTikeut,
    /// \u{d7e4}: 'ퟤ'
    HangulJongseongPieupDashRieulDashPhieuph,
    /// \u{d7e5}: 'ퟥ'
    HangulJongseongPieupDashMieum,
    /// \u{d7e6}: 'ퟦ'
    HangulJongseongSsangpieup,
    /// \u{d7e7}: 'ퟧ'
    HangulJongseongPieupDashSiosDashTikeut,
    /// \u{d7e8}: 'ퟨ'
    HangulJongseongPieupDashCieuc,
    /// \u{d7e9}: 'ퟩ'
    HangulJongseongPieupDashChieuch,
    /// \u{d7ea}: 'ퟪ'
    HangulJongseongSiosDashMieum,
    /// \u{d7eb}: 'ퟫ'
    HangulJongseongSiosDashKapyeounpieup,
    /// \u{d7ec}: 'ퟬ'
    HangulJongseongSsangsiosDashKiyeok,
    /// \u{d7ed}: 'ퟭ'
    HangulJongseongSsangsiosDashTikeut,
    /// \u{d7ee}: 'ퟮ'
    HangulJongseongSiosDashPansios,
    /// \u{d7ef}: 'ퟯ'
    HangulJongseongSiosDashCieuc,
    /// \u{d7f0}: 'ퟰ'
    HangulJongseongSiosDashChieuch,
    /// \u{d7f1}: 'ퟱ'
    HangulJongseongSiosDashThieuth,
    /// \u{d7f2}: 'ퟲ'
    HangulJongseongSiosDashHieuh,
    /// \u{d7f3}: 'ퟳ'
    HangulJongseongPansiosDashPieup,
    /// \u{d7f4}: 'ퟴ'
    HangulJongseongPansiosDashKapyeounpieup,
    /// \u{d7f5}: 'ퟵ'
    HangulJongseongYesieungDashMieum,
    /// \u{d7f6}: 'ퟶ'
    HangulJongseongYesieungDashHieuh,
    /// \u{d7f7}: 'ퟷ'
    HangulJongseongCieucDashPieup,
    /// \u{d7f8}: 'ퟸ'
    HangulJongseongCieucDashSsangpieup,
    /// \u{d7f9}: 'ퟹ'
    HangulJongseongSsangcieuc,
    /// \u{d7fa}: 'ퟺ'
    HangulJongseongPhieuphDashSios,
    /// \u{d7fb}: 'ퟻ'
    HangulJongseongPhieuphDashThieuth,
}

impl Into<char> for HangulJamoExtendedB {
    fn into(self) -> char {
        use constants::*;
        match self {
            HangulJamoExtendedB::HangulJungseongODashYeo => HANGUL_JUNGSEONG_O_DASH_YEO,
            HangulJamoExtendedB::HangulJungseongODashODashI => HANGUL_JUNGSEONG_O_DASH_O_DASH_I,
            HangulJamoExtendedB::HangulJungseongYoDashA => HANGUL_JUNGSEONG_YO_DASH_A,
            HangulJamoExtendedB::HangulJungseongYoDashAe => HANGUL_JUNGSEONG_YO_DASH_AE,
            HangulJamoExtendedB::HangulJungseongYoDashEo => HANGUL_JUNGSEONG_YO_DASH_EO,
            HangulJamoExtendedB::HangulJungseongUDashYeo => HANGUL_JUNGSEONG_U_DASH_YEO,
            HangulJamoExtendedB::HangulJungseongUDashIDashI => HANGUL_JUNGSEONG_U_DASH_I_DASH_I,
            HangulJamoExtendedB::HangulJungseongYuDashAe => HANGUL_JUNGSEONG_YU_DASH_AE,
            HangulJamoExtendedB::HangulJungseongYuDashO => HANGUL_JUNGSEONG_YU_DASH_O,
            HangulJamoExtendedB::HangulJungseongEuDashA => HANGUL_JUNGSEONG_EU_DASH_A,
            HangulJamoExtendedB::HangulJungseongEuDashEo => HANGUL_JUNGSEONG_EU_DASH_EO,
            HangulJamoExtendedB::HangulJungseongEuDashE => HANGUL_JUNGSEONG_EU_DASH_E,
            HangulJamoExtendedB::HangulJungseongEuDashO => HANGUL_JUNGSEONG_EU_DASH_O,
            HangulJamoExtendedB::HangulJungseongIDashYaDashO => HANGUL_JUNGSEONG_I_DASH_YA_DASH_O,
            HangulJamoExtendedB::HangulJungseongIDashYae => HANGUL_JUNGSEONG_I_DASH_YAE,
            HangulJamoExtendedB::HangulJungseongIDashYeo => HANGUL_JUNGSEONG_I_DASH_YEO,
            HangulJamoExtendedB::HangulJungseongIDashYe => HANGUL_JUNGSEONG_I_DASH_YE,
            HangulJamoExtendedB::HangulJungseongIDashODashI => HANGUL_JUNGSEONG_I_DASH_O_DASH_I,
            HangulJamoExtendedB::HangulJungseongIDashYo => HANGUL_JUNGSEONG_I_DASH_YO,
            HangulJamoExtendedB::HangulJungseongIDashYu => HANGUL_JUNGSEONG_I_DASH_YU,
            HangulJamoExtendedB::HangulJungseongIDashI => HANGUL_JUNGSEONG_I_DASH_I,
            HangulJamoExtendedB::HangulJungseongAraeaDashA => HANGUL_JUNGSEONG_ARAEA_DASH_A,
            HangulJamoExtendedB::HangulJungseongAraeaDashE => HANGUL_JUNGSEONG_ARAEA_DASH_E,
            HangulJamoExtendedB::HangulJongseongNieunDashRieul => HANGUL_JONGSEONG_NIEUN_DASH_RIEUL,
            HangulJamoExtendedB::HangulJongseongNieunDashChieuch => HANGUL_JONGSEONG_NIEUN_DASH_CHIEUCH,
            HangulJamoExtendedB::HangulJongseongSsangtikeut => HANGUL_JONGSEONG_SSANGTIKEUT,
            HangulJamoExtendedB::HangulJongseongSsangtikeutDashPieup => HANGUL_JONGSEONG_SSANGTIKEUT_DASH_PIEUP,
            HangulJamoExtendedB::HangulJongseongTikeutDashPieup => HANGUL_JONGSEONG_TIKEUT_DASH_PIEUP,
            HangulJamoExtendedB::HangulJongseongTikeutDashSios => HANGUL_JONGSEONG_TIKEUT_DASH_SIOS,
            HangulJamoExtendedB::HangulJongseongTikeutDashSiosDashKiyeok => HANGUL_JONGSEONG_TIKEUT_DASH_SIOS_DASH_KIYEOK,
            HangulJamoExtendedB::HangulJongseongTikeutDashCieuc => HANGUL_JONGSEONG_TIKEUT_DASH_CIEUC,
            HangulJamoExtendedB::HangulJongseongTikeutDashChieuch => HANGUL_JONGSEONG_TIKEUT_DASH_CHIEUCH,
            HangulJamoExtendedB::HangulJongseongTikeutDashThieuth => HANGUL_JONGSEONG_TIKEUT_DASH_THIEUTH,
            HangulJamoExtendedB::HangulJongseongRieulDashSsangkiyeok => HANGUL_JONGSEONG_RIEUL_DASH_SSANGKIYEOK,
            HangulJamoExtendedB::HangulJongseongRieulDashKiyeokDashHieuh => HANGUL_JONGSEONG_RIEUL_DASH_KIYEOK_DASH_HIEUH,
            HangulJamoExtendedB::HangulJongseongSsangrieulDashKhieukh => HANGUL_JONGSEONG_SSANGRIEUL_DASH_KHIEUKH,
            HangulJamoExtendedB::HangulJongseongRieulDashMieumDashHieuh => HANGUL_JONGSEONG_RIEUL_DASH_MIEUM_DASH_HIEUH,
            HangulJamoExtendedB::HangulJongseongRieulDashPieupDashTikeut => HANGUL_JONGSEONG_RIEUL_DASH_PIEUP_DASH_TIKEUT,
            HangulJamoExtendedB::HangulJongseongRieulDashPieupDashPhieuph => HANGUL_JONGSEONG_RIEUL_DASH_PIEUP_DASH_PHIEUPH,
            HangulJamoExtendedB::HangulJongseongRieulDashYesieung => HANGUL_JONGSEONG_RIEUL_DASH_YESIEUNG,
            HangulJamoExtendedB::HangulJongseongRieulDashYeorinhieuhDashHieuh => HANGUL_JONGSEONG_RIEUL_DASH_YEORINHIEUH_DASH_HIEUH,
            HangulJamoExtendedB::HangulJongseongKapyeounrieul => HANGUL_JONGSEONG_KAPYEOUNRIEUL,
            HangulJamoExtendedB::HangulJongseongMieumDashNieun => HANGUL_JONGSEONG_MIEUM_DASH_NIEUN,
            HangulJamoExtendedB::HangulJongseongMieumDashSsangnieun => HANGUL_JONGSEONG_MIEUM_DASH_SSANGNIEUN,
            HangulJamoExtendedB::HangulJongseongSsangmieum => HANGUL_JONGSEONG_SSANGMIEUM,
            HangulJamoExtendedB::HangulJongseongMieumDashPieupDashSios => HANGUL_JONGSEONG_MIEUM_DASH_PIEUP_DASH_SIOS,
            HangulJamoExtendedB::HangulJongseongMieumDashCieuc => HANGUL_JONGSEONG_MIEUM_DASH_CIEUC,
            HangulJamoExtendedB::HangulJongseongPieupDashTikeut => HANGUL_JONGSEONG_PIEUP_DASH_TIKEUT,
            HangulJamoExtendedB::HangulJongseongPieupDashRieulDashPhieuph => HANGUL_JONGSEONG_PIEUP_DASH_RIEUL_DASH_PHIEUPH,
            HangulJamoExtendedB::HangulJongseongPieupDashMieum => HANGUL_JONGSEONG_PIEUP_DASH_MIEUM,
            HangulJamoExtendedB::HangulJongseongSsangpieup => HANGUL_JONGSEONG_SSANGPIEUP,
            HangulJamoExtendedB::HangulJongseongPieupDashSiosDashTikeut => HANGUL_JONGSEONG_PIEUP_DASH_SIOS_DASH_TIKEUT,
            HangulJamoExtendedB::HangulJongseongPieupDashCieuc => HANGUL_JONGSEONG_PIEUP_DASH_CIEUC,
            HangulJamoExtendedB::HangulJongseongPieupDashChieuch => HANGUL_JONGSEONG_PIEUP_DASH_CHIEUCH,
            HangulJamoExtendedB::HangulJongseongSiosDashMieum => HANGUL_JONGSEONG_SIOS_DASH_MIEUM,
            HangulJamoExtendedB::HangulJongseongSiosDashKapyeounpieup => HANGUL_JONGSEONG_SIOS_DASH_KAPYEOUNPIEUP,
            HangulJamoExtendedB::HangulJongseongSsangsiosDashKiyeok => HANGUL_JONGSEONG_SSANGSIOS_DASH_KIYEOK,
            HangulJamoExtendedB::HangulJongseongSsangsiosDashTikeut => HANGUL_JONGSEONG_SSANGSIOS_DASH_TIKEUT,
            HangulJamoExtendedB::HangulJongseongSiosDashPansios => HANGUL_JONGSEONG_SIOS_DASH_PANSIOS,
            HangulJamoExtendedB::HangulJongseongSiosDashCieuc => HANGUL_JONGSEONG_SIOS_DASH_CIEUC,
            HangulJamoExtendedB::HangulJongseongSiosDashChieuch => HANGUL_JONGSEONG_SIOS_DASH_CHIEUCH,
            HangulJamoExtendedB::HangulJongseongSiosDashThieuth => HANGUL_JONGSEONG_SIOS_DASH_THIEUTH,
            HangulJamoExtendedB::HangulJongseongSiosDashHieuh => HANGUL_JONGSEONG_SIOS_DASH_HIEUH,
            HangulJamoExtendedB::HangulJongseongPansiosDashPieup => HANGUL_JONGSEONG_PANSIOS_DASH_PIEUP,
            HangulJamoExtendedB::HangulJongseongPansiosDashKapyeounpieup => HANGUL_JONGSEONG_PANSIOS_DASH_KAPYEOUNPIEUP,
            HangulJamoExtendedB::HangulJongseongYesieungDashMieum => HANGUL_JONGSEONG_YESIEUNG_DASH_MIEUM,
            HangulJamoExtendedB::HangulJongseongYesieungDashHieuh => HANGUL_JONGSEONG_YESIEUNG_DASH_HIEUH,
            HangulJamoExtendedB::HangulJongseongCieucDashPieup => HANGUL_JONGSEONG_CIEUC_DASH_PIEUP,
            HangulJamoExtendedB::HangulJongseongCieucDashSsangpieup => HANGUL_JONGSEONG_CIEUC_DASH_SSANGPIEUP,
            HangulJamoExtendedB::HangulJongseongSsangcieuc => HANGUL_JONGSEONG_SSANGCIEUC,
            HangulJamoExtendedB::HangulJongseongPhieuphDashSios => HANGUL_JONGSEONG_PHIEUPH_DASH_SIOS,
            HangulJamoExtendedB::HangulJongseongPhieuphDashThieuth => HANGUL_JONGSEONG_PHIEUPH_DASH_THIEUTH,
        }
    }
}

impl std::convert::TryFrom<char> for HangulJamoExtendedB {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            HANGUL_JUNGSEONG_O_DASH_YEO => Ok(HangulJamoExtendedB::HangulJungseongODashYeo),
            HANGUL_JUNGSEONG_O_DASH_O_DASH_I => Ok(HangulJamoExtendedB::HangulJungseongODashODashI),
            HANGUL_JUNGSEONG_YO_DASH_A => Ok(HangulJamoExtendedB::HangulJungseongYoDashA),
            HANGUL_JUNGSEONG_YO_DASH_AE => Ok(HangulJamoExtendedB::HangulJungseongYoDashAe),
            HANGUL_JUNGSEONG_YO_DASH_EO => Ok(HangulJamoExtendedB::HangulJungseongYoDashEo),
            HANGUL_JUNGSEONG_U_DASH_YEO => Ok(HangulJamoExtendedB::HangulJungseongUDashYeo),
            HANGUL_JUNGSEONG_U_DASH_I_DASH_I => Ok(HangulJamoExtendedB::HangulJungseongUDashIDashI),
            HANGUL_JUNGSEONG_YU_DASH_AE => Ok(HangulJamoExtendedB::HangulJungseongYuDashAe),
            HANGUL_JUNGSEONG_YU_DASH_O => Ok(HangulJamoExtendedB::HangulJungseongYuDashO),
            HANGUL_JUNGSEONG_EU_DASH_A => Ok(HangulJamoExtendedB::HangulJungseongEuDashA),
            HANGUL_JUNGSEONG_EU_DASH_EO => Ok(HangulJamoExtendedB::HangulJungseongEuDashEo),
            HANGUL_JUNGSEONG_EU_DASH_E => Ok(HangulJamoExtendedB::HangulJungseongEuDashE),
            HANGUL_JUNGSEONG_EU_DASH_O => Ok(HangulJamoExtendedB::HangulJungseongEuDashO),
            HANGUL_JUNGSEONG_I_DASH_YA_DASH_O => Ok(HangulJamoExtendedB::HangulJungseongIDashYaDashO),
            HANGUL_JUNGSEONG_I_DASH_YAE => Ok(HangulJamoExtendedB::HangulJungseongIDashYae),
            HANGUL_JUNGSEONG_I_DASH_YEO => Ok(HangulJamoExtendedB::HangulJungseongIDashYeo),
            HANGUL_JUNGSEONG_I_DASH_YE => Ok(HangulJamoExtendedB::HangulJungseongIDashYe),
            HANGUL_JUNGSEONG_I_DASH_O_DASH_I => Ok(HangulJamoExtendedB::HangulJungseongIDashODashI),
            HANGUL_JUNGSEONG_I_DASH_YO => Ok(HangulJamoExtendedB::HangulJungseongIDashYo),
            HANGUL_JUNGSEONG_I_DASH_YU => Ok(HangulJamoExtendedB::HangulJungseongIDashYu),
            HANGUL_JUNGSEONG_I_DASH_I => Ok(HangulJamoExtendedB::HangulJungseongIDashI),
            HANGUL_JUNGSEONG_ARAEA_DASH_A => Ok(HangulJamoExtendedB::HangulJungseongAraeaDashA),
            HANGUL_JUNGSEONG_ARAEA_DASH_E => Ok(HangulJamoExtendedB::HangulJungseongAraeaDashE),
            HANGUL_JONGSEONG_NIEUN_DASH_RIEUL => Ok(HangulJamoExtendedB::HangulJongseongNieunDashRieul),
            HANGUL_JONGSEONG_NIEUN_DASH_CHIEUCH => Ok(HangulJamoExtendedB::HangulJongseongNieunDashChieuch),
            HANGUL_JONGSEONG_SSANGTIKEUT => Ok(HangulJamoExtendedB::HangulJongseongSsangtikeut),
            HANGUL_JONGSEONG_SSANGTIKEUT_DASH_PIEUP => Ok(HangulJamoExtendedB::HangulJongseongSsangtikeutDashPieup),
            HANGUL_JONGSEONG_TIKEUT_DASH_PIEUP => Ok(HangulJamoExtendedB::HangulJongseongTikeutDashPieup),
            HANGUL_JONGSEONG_TIKEUT_DASH_SIOS => Ok(HangulJamoExtendedB::HangulJongseongTikeutDashSios),
            HANGUL_JONGSEONG_TIKEUT_DASH_SIOS_DASH_KIYEOK => Ok(HangulJamoExtendedB::HangulJongseongTikeutDashSiosDashKiyeok),
            HANGUL_JONGSEONG_TIKEUT_DASH_CIEUC => Ok(HangulJamoExtendedB::HangulJongseongTikeutDashCieuc),
            HANGUL_JONGSEONG_TIKEUT_DASH_CHIEUCH => Ok(HangulJamoExtendedB::HangulJongseongTikeutDashChieuch),
            HANGUL_JONGSEONG_TIKEUT_DASH_THIEUTH => Ok(HangulJamoExtendedB::HangulJongseongTikeutDashThieuth),
            HANGUL_JONGSEONG_RIEUL_DASH_SSANGKIYEOK => Ok(HangulJamoExtendedB::HangulJongseongRieulDashSsangkiyeok),
            HANGUL_JONGSEONG_RIEUL_DASH_KIYEOK_DASH_HIEUH => Ok(HangulJamoExtendedB::HangulJongseongRieulDashKiyeokDashHieuh),
            HANGUL_JONGSEONG_SSANGRIEUL_DASH_KHIEUKH => Ok(HangulJamoExtendedB::HangulJongseongSsangrieulDashKhieukh),
            HANGUL_JONGSEONG_RIEUL_DASH_MIEUM_DASH_HIEUH => Ok(HangulJamoExtendedB::HangulJongseongRieulDashMieumDashHieuh),
            HANGUL_JONGSEONG_RIEUL_DASH_PIEUP_DASH_TIKEUT => Ok(HangulJamoExtendedB::HangulJongseongRieulDashPieupDashTikeut),
            HANGUL_JONGSEONG_RIEUL_DASH_PIEUP_DASH_PHIEUPH => Ok(HangulJamoExtendedB::HangulJongseongRieulDashPieupDashPhieuph),
            HANGUL_JONGSEONG_RIEUL_DASH_YESIEUNG => Ok(HangulJamoExtendedB::HangulJongseongRieulDashYesieung),
            HANGUL_JONGSEONG_RIEUL_DASH_YEORINHIEUH_DASH_HIEUH => Ok(HangulJamoExtendedB::HangulJongseongRieulDashYeorinhieuhDashHieuh),
            HANGUL_JONGSEONG_KAPYEOUNRIEUL => Ok(HangulJamoExtendedB::HangulJongseongKapyeounrieul),
            HANGUL_JONGSEONG_MIEUM_DASH_NIEUN => Ok(HangulJamoExtendedB::HangulJongseongMieumDashNieun),
            HANGUL_JONGSEONG_MIEUM_DASH_SSANGNIEUN => Ok(HangulJamoExtendedB::HangulJongseongMieumDashSsangnieun),
            HANGUL_JONGSEONG_SSANGMIEUM => Ok(HangulJamoExtendedB::HangulJongseongSsangmieum),
            HANGUL_JONGSEONG_MIEUM_DASH_PIEUP_DASH_SIOS => Ok(HangulJamoExtendedB::HangulJongseongMieumDashPieupDashSios),
            HANGUL_JONGSEONG_MIEUM_DASH_CIEUC => Ok(HangulJamoExtendedB::HangulJongseongMieumDashCieuc),
            HANGUL_JONGSEONG_PIEUP_DASH_TIKEUT => Ok(HangulJamoExtendedB::HangulJongseongPieupDashTikeut),
            HANGUL_JONGSEONG_PIEUP_DASH_RIEUL_DASH_PHIEUPH => Ok(HangulJamoExtendedB::HangulJongseongPieupDashRieulDashPhieuph),
            HANGUL_JONGSEONG_PIEUP_DASH_MIEUM => Ok(HangulJamoExtendedB::HangulJongseongPieupDashMieum),
            HANGUL_JONGSEONG_SSANGPIEUP => Ok(HangulJamoExtendedB::HangulJongseongSsangpieup),
            HANGUL_JONGSEONG_PIEUP_DASH_SIOS_DASH_TIKEUT => Ok(HangulJamoExtendedB::HangulJongseongPieupDashSiosDashTikeut),
            HANGUL_JONGSEONG_PIEUP_DASH_CIEUC => Ok(HangulJamoExtendedB::HangulJongseongPieupDashCieuc),
            HANGUL_JONGSEONG_PIEUP_DASH_CHIEUCH => Ok(HangulJamoExtendedB::HangulJongseongPieupDashChieuch),
            HANGUL_JONGSEONG_SIOS_DASH_MIEUM => Ok(HangulJamoExtendedB::HangulJongseongSiosDashMieum),
            HANGUL_JONGSEONG_SIOS_DASH_KAPYEOUNPIEUP => Ok(HangulJamoExtendedB::HangulJongseongSiosDashKapyeounpieup),
            HANGUL_JONGSEONG_SSANGSIOS_DASH_KIYEOK => Ok(HangulJamoExtendedB::HangulJongseongSsangsiosDashKiyeok),
            HANGUL_JONGSEONG_SSANGSIOS_DASH_TIKEUT => Ok(HangulJamoExtendedB::HangulJongseongSsangsiosDashTikeut),
            HANGUL_JONGSEONG_SIOS_DASH_PANSIOS => Ok(HangulJamoExtendedB::HangulJongseongSiosDashPansios),
            HANGUL_JONGSEONG_SIOS_DASH_CIEUC => Ok(HangulJamoExtendedB::HangulJongseongSiosDashCieuc),
            HANGUL_JONGSEONG_SIOS_DASH_CHIEUCH => Ok(HangulJamoExtendedB::HangulJongseongSiosDashChieuch),
            HANGUL_JONGSEONG_SIOS_DASH_THIEUTH => Ok(HangulJamoExtendedB::HangulJongseongSiosDashThieuth),
            HANGUL_JONGSEONG_SIOS_DASH_HIEUH => Ok(HangulJamoExtendedB::HangulJongseongSiosDashHieuh),
            HANGUL_JONGSEONG_PANSIOS_DASH_PIEUP => Ok(HangulJamoExtendedB::HangulJongseongPansiosDashPieup),
            HANGUL_JONGSEONG_PANSIOS_DASH_KAPYEOUNPIEUP => Ok(HangulJamoExtendedB::HangulJongseongPansiosDashKapyeounpieup),
            HANGUL_JONGSEONG_YESIEUNG_DASH_MIEUM => Ok(HangulJamoExtendedB::HangulJongseongYesieungDashMieum),
            HANGUL_JONGSEONG_YESIEUNG_DASH_HIEUH => Ok(HangulJamoExtendedB::HangulJongseongYesieungDashHieuh),
            HANGUL_JONGSEONG_CIEUC_DASH_PIEUP => Ok(HangulJamoExtendedB::HangulJongseongCieucDashPieup),
            HANGUL_JONGSEONG_CIEUC_DASH_SSANGPIEUP => Ok(HangulJamoExtendedB::HangulJongseongCieucDashSsangpieup),
            HANGUL_JONGSEONG_SSANGCIEUC => Ok(HangulJamoExtendedB::HangulJongseongSsangcieuc),
            HANGUL_JONGSEONG_PHIEUPH_DASH_SIOS => Ok(HangulJamoExtendedB::HangulJongseongPhieuphDashSios),
            HANGUL_JONGSEONG_PHIEUPH_DASH_THIEUTH => Ok(HangulJamoExtendedB::HangulJongseongPhieuphDashThieuth),
            _ => Err(()),
        }
    }
}

impl Into<u32> for HangulJamoExtendedB {
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

impl std::convert::TryFrom<u32> for HangulJamoExtendedB {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for HangulJamoExtendedB {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl HangulJamoExtendedB {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        HangulJamoExtendedB::HangulJungseongODashYeo
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            HangulJamoExtendedB::HangulJungseongODashYeo => "hangul jungseong o-yeo",
            HangulJamoExtendedB::HangulJungseongODashODashI => "hangul jungseong o-o-i",
            HangulJamoExtendedB::HangulJungseongYoDashA => "hangul jungseong yo-a",
            HangulJamoExtendedB::HangulJungseongYoDashAe => "hangul jungseong yo-ae",
            HangulJamoExtendedB::HangulJungseongYoDashEo => "hangul jungseong yo-eo",
            HangulJamoExtendedB::HangulJungseongUDashYeo => "hangul jungseong u-yeo",
            HangulJamoExtendedB::HangulJungseongUDashIDashI => "hangul jungseong u-i-i",
            HangulJamoExtendedB::HangulJungseongYuDashAe => "hangul jungseong yu-ae",
            HangulJamoExtendedB::HangulJungseongYuDashO => "hangul jungseong yu-o",
            HangulJamoExtendedB::HangulJungseongEuDashA => "hangul jungseong eu-a",
            HangulJamoExtendedB::HangulJungseongEuDashEo => "hangul jungseong eu-eo",
            HangulJamoExtendedB::HangulJungseongEuDashE => "hangul jungseong eu-e",
            HangulJamoExtendedB::HangulJungseongEuDashO => "hangul jungseong eu-o",
            HangulJamoExtendedB::HangulJungseongIDashYaDashO => "hangul jungseong i-ya-o",
            HangulJamoExtendedB::HangulJungseongIDashYae => "hangul jungseong i-yae",
            HangulJamoExtendedB::HangulJungseongIDashYeo => "hangul jungseong i-yeo",
            HangulJamoExtendedB::HangulJungseongIDashYe => "hangul jungseong i-ye",
            HangulJamoExtendedB::HangulJungseongIDashODashI => "hangul jungseong i-o-i",
            HangulJamoExtendedB::HangulJungseongIDashYo => "hangul jungseong i-yo",
            HangulJamoExtendedB::HangulJungseongIDashYu => "hangul jungseong i-yu",
            HangulJamoExtendedB::HangulJungseongIDashI => "hangul jungseong i-i",
            HangulJamoExtendedB::HangulJungseongAraeaDashA => "hangul jungseong araea-a",
            HangulJamoExtendedB::HangulJungseongAraeaDashE => "hangul jungseong araea-e",
            HangulJamoExtendedB::HangulJongseongNieunDashRieul => "hangul jongseong nieun-rieul",
            HangulJamoExtendedB::HangulJongseongNieunDashChieuch => "hangul jongseong nieun-chieuch",
            HangulJamoExtendedB::HangulJongseongSsangtikeut => "hangul jongseong ssangtikeut",
            HangulJamoExtendedB::HangulJongseongSsangtikeutDashPieup => "hangul jongseong ssangtikeut-pieup",
            HangulJamoExtendedB::HangulJongseongTikeutDashPieup => "hangul jongseong tikeut-pieup",
            HangulJamoExtendedB::HangulJongseongTikeutDashSios => "hangul jongseong tikeut-sios",
            HangulJamoExtendedB::HangulJongseongTikeutDashSiosDashKiyeok => "hangul jongseong tikeut-sios-kiyeok",
            HangulJamoExtendedB::HangulJongseongTikeutDashCieuc => "hangul jongseong tikeut-cieuc",
            HangulJamoExtendedB::HangulJongseongTikeutDashChieuch => "hangul jongseong tikeut-chieuch",
            HangulJamoExtendedB::HangulJongseongTikeutDashThieuth => "hangul jongseong tikeut-thieuth",
            HangulJamoExtendedB::HangulJongseongRieulDashSsangkiyeok => "hangul jongseong rieul-ssangkiyeok",
            HangulJamoExtendedB::HangulJongseongRieulDashKiyeokDashHieuh => "hangul jongseong rieul-kiyeok-hieuh",
            HangulJamoExtendedB::HangulJongseongSsangrieulDashKhieukh => "hangul jongseong ssangrieul-khieukh",
            HangulJamoExtendedB::HangulJongseongRieulDashMieumDashHieuh => "hangul jongseong rieul-mieum-hieuh",
            HangulJamoExtendedB::HangulJongseongRieulDashPieupDashTikeut => "hangul jongseong rieul-pieup-tikeut",
            HangulJamoExtendedB::HangulJongseongRieulDashPieupDashPhieuph => "hangul jongseong rieul-pieup-phieuph",
            HangulJamoExtendedB::HangulJongseongRieulDashYesieung => "hangul jongseong rieul-yesieung",
            HangulJamoExtendedB::HangulJongseongRieulDashYeorinhieuhDashHieuh => "hangul jongseong rieul-yeorinhieuh-hieuh",
            HangulJamoExtendedB::HangulJongseongKapyeounrieul => "hangul jongseong kapyeounrieul",
            HangulJamoExtendedB::HangulJongseongMieumDashNieun => "hangul jongseong mieum-nieun",
            HangulJamoExtendedB::HangulJongseongMieumDashSsangnieun => "hangul jongseong mieum-ssangnieun",
            HangulJamoExtendedB::HangulJongseongSsangmieum => "hangul jongseong ssangmieum",
            HangulJamoExtendedB::HangulJongseongMieumDashPieupDashSios => "hangul jongseong mieum-pieup-sios",
            HangulJamoExtendedB::HangulJongseongMieumDashCieuc => "hangul jongseong mieum-cieuc",
            HangulJamoExtendedB::HangulJongseongPieupDashTikeut => "hangul jongseong pieup-tikeut",
            HangulJamoExtendedB::HangulJongseongPieupDashRieulDashPhieuph => "hangul jongseong pieup-rieul-phieuph",
            HangulJamoExtendedB::HangulJongseongPieupDashMieum => "hangul jongseong pieup-mieum",
            HangulJamoExtendedB::HangulJongseongSsangpieup => "hangul jongseong ssangpieup",
            HangulJamoExtendedB::HangulJongseongPieupDashSiosDashTikeut => "hangul jongseong pieup-sios-tikeut",
            HangulJamoExtendedB::HangulJongseongPieupDashCieuc => "hangul jongseong pieup-cieuc",
            HangulJamoExtendedB::HangulJongseongPieupDashChieuch => "hangul jongseong pieup-chieuch",
            HangulJamoExtendedB::HangulJongseongSiosDashMieum => "hangul jongseong sios-mieum",
            HangulJamoExtendedB::HangulJongseongSiosDashKapyeounpieup => "hangul jongseong sios-kapyeounpieup",
            HangulJamoExtendedB::HangulJongseongSsangsiosDashKiyeok => "hangul jongseong ssangsios-kiyeok",
            HangulJamoExtendedB::HangulJongseongSsangsiosDashTikeut => "hangul jongseong ssangsios-tikeut",
            HangulJamoExtendedB::HangulJongseongSiosDashPansios => "hangul jongseong sios-pansios",
            HangulJamoExtendedB::HangulJongseongSiosDashCieuc => "hangul jongseong sios-cieuc",
            HangulJamoExtendedB::HangulJongseongSiosDashChieuch => "hangul jongseong sios-chieuch",
            HangulJamoExtendedB::HangulJongseongSiosDashThieuth => "hangul jongseong sios-thieuth",
            HangulJamoExtendedB::HangulJongseongSiosDashHieuh => "hangul jongseong sios-hieuh",
            HangulJamoExtendedB::HangulJongseongPansiosDashPieup => "hangul jongseong pansios-pieup",
            HangulJamoExtendedB::HangulJongseongPansiosDashKapyeounpieup => "hangul jongseong pansios-kapyeounpieup",
            HangulJamoExtendedB::HangulJongseongYesieungDashMieum => "hangul jongseong yesieung-mieum",
            HangulJamoExtendedB::HangulJongseongYesieungDashHieuh => "hangul jongseong yesieung-hieuh",
            HangulJamoExtendedB::HangulJongseongCieucDashPieup => "hangul jongseong cieuc-pieup",
            HangulJamoExtendedB::HangulJongseongCieucDashSsangpieup => "hangul jongseong cieuc-ssangpieup",
            HangulJamoExtendedB::HangulJongseongSsangcieuc => "hangul jongseong ssangcieuc",
            HangulJamoExtendedB::HangulJongseongPhieuphDashSios => "hangul jongseong phieuph-sios",
            HangulJamoExtendedB::HangulJongseongPhieuphDashThieuth => "hangul jongseong phieuph-thieuth",
        }
    }
}
