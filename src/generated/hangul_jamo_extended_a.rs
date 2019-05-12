/// \u{a960} → \u{a97f}\
///\
/// ꥠ ꥡ ꥢ ꥣ ꥤ ꥥ ꥦ ꥧ ꥨ ꥩ ꥪ ꥫ ꥬ ꥭ ꥮ ꥯ\
/// ꥰ ꥱ ꥲ ꥳ ꥴ ꥵ ꥶ ꥷ ꥸ ꥹ ꥺ ꥻ ꥼ\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{a960}: 'ꥠ'
    pub const HANGUL_CHOSEONG_TIKEUT_DASH_MIEUM: char = 'ꥠ';
    /// \u{a961}: 'ꥡ'
    pub const HANGUL_CHOSEONG_TIKEUT_DASH_PIEUP: char = 'ꥡ';
    /// \u{a962}: 'ꥢ'
    pub const HANGUL_CHOSEONG_TIKEUT_DASH_SIOS: char = 'ꥢ';
    /// \u{a963}: 'ꥣ'
    pub const HANGUL_CHOSEONG_TIKEUT_DASH_CIEUC: char = 'ꥣ';
    /// \u{a964}: 'ꥤ'
    pub const HANGUL_CHOSEONG_RIEUL_DASH_KIYEOK: char = 'ꥤ';
    /// \u{a965}: 'ꥥ'
    pub const HANGUL_CHOSEONG_RIEUL_DASH_SSANGKIYEOK: char = 'ꥥ';
    /// \u{a966}: 'ꥦ'
    pub const HANGUL_CHOSEONG_RIEUL_DASH_TIKEUT: char = 'ꥦ';
    /// \u{a967}: 'ꥧ'
    pub const HANGUL_CHOSEONG_RIEUL_DASH_SSANGTIKEUT: char = 'ꥧ';
    /// \u{a968}: 'ꥨ'
    pub const HANGUL_CHOSEONG_RIEUL_DASH_MIEUM: char = 'ꥨ';
    /// \u{a969}: 'ꥩ'
    pub const HANGUL_CHOSEONG_RIEUL_DASH_PIEUP: char = 'ꥩ';
    /// \u{a96a}: 'ꥪ'
    pub const HANGUL_CHOSEONG_RIEUL_DASH_SSANGPIEUP: char = 'ꥪ';
    /// \u{a96b}: 'ꥫ'
    pub const HANGUL_CHOSEONG_RIEUL_DASH_KAPYEOUNPIEUP: char = 'ꥫ';
    /// \u{a96c}: 'ꥬ'
    pub const HANGUL_CHOSEONG_RIEUL_DASH_SIOS: char = 'ꥬ';
    /// \u{a96d}: 'ꥭ'
    pub const HANGUL_CHOSEONG_RIEUL_DASH_CIEUC: char = 'ꥭ';
    /// \u{a96e}: 'ꥮ'
    pub const HANGUL_CHOSEONG_RIEUL_DASH_KHIEUKH: char = 'ꥮ';
    /// \u{a96f}: 'ꥯ'
    pub const HANGUL_CHOSEONG_MIEUM_DASH_KIYEOK: char = 'ꥯ';
    /// \u{a970}: 'ꥰ'
    pub const HANGUL_CHOSEONG_MIEUM_DASH_TIKEUT: char = 'ꥰ';
    /// \u{a971}: 'ꥱ'
    pub const HANGUL_CHOSEONG_MIEUM_DASH_SIOS: char = 'ꥱ';
    /// \u{a972}: 'ꥲ'
    pub const HANGUL_CHOSEONG_PIEUP_DASH_SIOS_DASH_THIEUTH: char = 'ꥲ';
    /// \u{a973}: 'ꥳ'
    pub const HANGUL_CHOSEONG_PIEUP_DASH_KHIEUKH: char = 'ꥳ';
    /// \u{a974}: 'ꥴ'
    pub const HANGUL_CHOSEONG_PIEUP_DASH_HIEUH: char = 'ꥴ';
    /// \u{a975}: 'ꥵ'
    pub const HANGUL_CHOSEONG_SSANGSIOS_DASH_PIEUP: char = 'ꥵ';
    /// \u{a976}: 'ꥶ'
    pub const HANGUL_CHOSEONG_IEUNG_DASH_RIEUL: char = 'ꥶ';
    /// \u{a977}: 'ꥷ'
    pub const HANGUL_CHOSEONG_IEUNG_DASH_HIEUH: char = 'ꥷ';
    /// \u{a978}: 'ꥸ'
    pub const HANGUL_CHOSEONG_SSANGCIEUC_DASH_HIEUH: char = 'ꥸ';
    /// \u{a979}: 'ꥹ'
    pub const HANGUL_CHOSEONG_SSANGTHIEUTH: char = 'ꥹ';
    /// \u{a97a}: 'ꥺ'
    pub const HANGUL_CHOSEONG_PHIEUPH_DASH_HIEUH: char = 'ꥺ';
    /// \u{a97b}: 'ꥻ'
    pub const HANGUL_CHOSEONG_HIEUH_DASH_SIOS: char = 'ꥻ';
    /// \u{a97c}: 'ꥼ'
    pub const HANGUL_CHOSEONG_SSANGYEORINHIEUH: char = 'ꥼ';
}

/// An enum to represent all characters in the HangulJamoExtendedA block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum HangulJamoExtendedA {
    /// \u{a960}: 'ꥠ'
    HangulChoseongTikeutDashMieum,
    /// \u{a961}: 'ꥡ'
    HangulChoseongTikeutDashPieup,
    /// \u{a962}: 'ꥢ'
    HangulChoseongTikeutDashSios,
    /// \u{a963}: 'ꥣ'
    HangulChoseongTikeutDashCieuc,
    /// \u{a964}: 'ꥤ'
    HangulChoseongRieulDashKiyeok,
    /// \u{a965}: 'ꥥ'
    HangulChoseongRieulDashSsangkiyeok,
    /// \u{a966}: 'ꥦ'
    HangulChoseongRieulDashTikeut,
    /// \u{a967}: 'ꥧ'
    HangulChoseongRieulDashSsangtikeut,
    /// \u{a968}: 'ꥨ'
    HangulChoseongRieulDashMieum,
    /// \u{a969}: 'ꥩ'
    HangulChoseongRieulDashPieup,
    /// \u{a96a}: 'ꥪ'
    HangulChoseongRieulDashSsangpieup,
    /// \u{a96b}: 'ꥫ'
    HangulChoseongRieulDashKapyeounpieup,
    /// \u{a96c}: 'ꥬ'
    HangulChoseongRieulDashSios,
    /// \u{a96d}: 'ꥭ'
    HangulChoseongRieulDashCieuc,
    /// \u{a96e}: 'ꥮ'
    HangulChoseongRieulDashKhieukh,
    /// \u{a96f}: 'ꥯ'
    HangulChoseongMieumDashKiyeok,
    /// \u{a970}: 'ꥰ'
    HangulChoseongMieumDashTikeut,
    /// \u{a971}: 'ꥱ'
    HangulChoseongMieumDashSios,
    /// \u{a972}: 'ꥲ'
    HangulChoseongPieupDashSiosDashThieuth,
    /// \u{a973}: 'ꥳ'
    HangulChoseongPieupDashKhieukh,
    /// \u{a974}: 'ꥴ'
    HangulChoseongPieupDashHieuh,
    /// \u{a975}: 'ꥵ'
    HangulChoseongSsangsiosDashPieup,
    /// \u{a976}: 'ꥶ'
    HangulChoseongIeungDashRieul,
    /// \u{a977}: 'ꥷ'
    HangulChoseongIeungDashHieuh,
    /// \u{a978}: 'ꥸ'
    HangulChoseongSsangcieucDashHieuh,
    /// \u{a979}: 'ꥹ'
    HangulChoseongSsangthieuth,
    /// \u{a97a}: 'ꥺ'
    HangulChoseongPhieuphDashHieuh,
    /// \u{a97b}: 'ꥻ'
    HangulChoseongHieuhDashSios,
    /// \u{a97c}: 'ꥼ'
    HangulChoseongSsangyeorinhieuh,
}

impl Into<char> for HangulJamoExtendedA {
    fn into(self) -> char {
        use constants::*;
        match self {
            HangulJamoExtendedA::HangulChoseongTikeutDashMieum => HANGUL_CHOSEONG_TIKEUT_DASH_MIEUM,
            HangulJamoExtendedA::HangulChoseongTikeutDashPieup => HANGUL_CHOSEONG_TIKEUT_DASH_PIEUP,
            HangulJamoExtendedA::HangulChoseongTikeutDashSios => HANGUL_CHOSEONG_TIKEUT_DASH_SIOS,
            HangulJamoExtendedA::HangulChoseongTikeutDashCieuc => HANGUL_CHOSEONG_TIKEUT_DASH_CIEUC,
            HangulJamoExtendedA::HangulChoseongRieulDashKiyeok => HANGUL_CHOSEONG_RIEUL_DASH_KIYEOK,
            HangulJamoExtendedA::HangulChoseongRieulDashSsangkiyeok => HANGUL_CHOSEONG_RIEUL_DASH_SSANGKIYEOK,
            HangulJamoExtendedA::HangulChoseongRieulDashTikeut => HANGUL_CHOSEONG_RIEUL_DASH_TIKEUT,
            HangulJamoExtendedA::HangulChoseongRieulDashSsangtikeut => HANGUL_CHOSEONG_RIEUL_DASH_SSANGTIKEUT,
            HangulJamoExtendedA::HangulChoseongRieulDashMieum => HANGUL_CHOSEONG_RIEUL_DASH_MIEUM,
            HangulJamoExtendedA::HangulChoseongRieulDashPieup => HANGUL_CHOSEONG_RIEUL_DASH_PIEUP,
            HangulJamoExtendedA::HangulChoseongRieulDashSsangpieup => HANGUL_CHOSEONG_RIEUL_DASH_SSANGPIEUP,
            HangulJamoExtendedA::HangulChoseongRieulDashKapyeounpieup => HANGUL_CHOSEONG_RIEUL_DASH_KAPYEOUNPIEUP,
            HangulJamoExtendedA::HangulChoseongRieulDashSios => HANGUL_CHOSEONG_RIEUL_DASH_SIOS,
            HangulJamoExtendedA::HangulChoseongRieulDashCieuc => HANGUL_CHOSEONG_RIEUL_DASH_CIEUC,
            HangulJamoExtendedA::HangulChoseongRieulDashKhieukh => HANGUL_CHOSEONG_RIEUL_DASH_KHIEUKH,
            HangulJamoExtendedA::HangulChoseongMieumDashKiyeok => HANGUL_CHOSEONG_MIEUM_DASH_KIYEOK,
            HangulJamoExtendedA::HangulChoseongMieumDashTikeut => HANGUL_CHOSEONG_MIEUM_DASH_TIKEUT,
            HangulJamoExtendedA::HangulChoseongMieumDashSios => HANGUL_CHOSEONG_MIEUM_DASH_SIOS,
            HangulJamoExtendedA::HangulChoseongPieupDashSiosDashThieuth => HANGUL_CHOSEONG_PIEUP_DASH_SIOS_DASH_THIEUTH,
            HangulJamoExtendedA::HangulChoseongPieupDashKhieukh => HANGUL_CHOSEONG_PIEUP_DASH_KHIEUKH,
            HangulJamoExtendedA::HangulChoseongPieupDashHieuh => HANGUL_CHOSEONG_PIEUP_DASH_HIEUH,
            HangulJamoExtendedA::HangulChoseongSsangsiosDashPieup => HANGUL_CHOSEONG_SSANGSIOS_DASH_PIEUP,
            HangulJamoExtendedA::HangulChoseongIeungDashRieul => HANGUL_CHOSEONG_IEUNG_DASH_RIEUL,
            HangulJamoExtendedA::HangulChoseongIeungDashHieuh => HANGUL_CHOSEONG_IEUNG_DASH_HIEUH,
            HangulJamoExtendedA::HangulChoseongSsangcieucDashHieuh => HANGUL_CHOSEONG_SSANGCIEUC_DASH_HIEUH,
            HangulJamoExtendedA::HangulChoseongSsangthieuth => HANGUL_CHOSEONG_SSANGTHIEUTH,
            HangulJamoExtendedA::HangulChoseongPhieuphDashHieuh => HANGUL_CHOSEONG_PHIEUPH_DASH_HIEUH,
            HangulJamoExtendedA::HangulChoseongHieuhDashSios => HANGUL_CHOSEONG_HIEUH_DASH_SIOS,
            HangulJamoExtendedA::HangulChoseongSsangyeorinhieuh => HANGUL_CHOSEONG_SSANGYEORINHIEUH,
        }
    }
}

impl std::convert::TryFrom<char> for HangulJamoExtendedA {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            HANGUL_CHOSEONG_TIKEUT_DASH_MIEUM => Ok(HangulJamoExtendedA::HangulChoseongTikeutDashMieum),
            HANGUL_CHOSEONG_TIKEUT_DASH_PIEUP => Ok(HangulJamoExtendedA::HangulChoseongTikeutDashPieup),
            HANGUL_CHOSEONG_TIKEUT_DASH_SIOS => Ok(HangulJamoExtendedA::HangulChoseongTikeutDashSios),
            HANGUL_CHOSEONG_TIKEUT_DASH_CIEUC => Ok(HangulJamoExtendedA::HangulChoseongTikeutDashCieuc),
            HANGUL_CHOSEONG_RIEUL_DASH_KIYEOK => Ok(HangulJamoExtendedA::HangulChoseongRieulDashKiyeok),
            HANGUL_CHOSEONG_RIEUL_DASH_SSANGKIYEOK => Ok(HangulJamoExtendedA::HangulChoseongRieulDashSsangkiyeok),
            HANGUL_CHOSEONG_RIEUL_DASH_TIKEUT => Ok(HangulJamoExtendedA::HangulChoseongRieulDashTikeut),
            HANGUL_CHOSEONG_RIEUL_DASH_SSANGTIKEUT => Ok(HangulJamoExtendedA::HangulChoseongRieulDashSsangtikeut),
            HANGUL_CHOSEONG_RIEUL_DASH_MIEUM => Ok(HangulJamoExtendedA::HangulChoseongRieulDashMieum),
            HANGUL_CHOSEONG_RIEUL_DASH_PIEUP => Ok(HangulJamoExtendedA::HangulChoseongRieulDashPieup),
            HANGUL_CHOSEONG_RIEUL_DASH_SSANGPIEUP => Ok(HangulJamoExtendedA::HangulChoseongRieulDashSsangpieup),
            HANGUL_CHOSEONG_RIEUL_DASH_KAPYEOUNPIEUP => Ok(HangulJamoExtendedA::HangulChoseongRieulDashKapyeounpieup),
            HANGUL_CHOSEONG_RIEUL_DASH_SIOS => Ok(HangulJamoExtendedA::HangulChoseongRieulDashSios),
            HANGUL_CHOSEONG_RIEUL_DASH_CIEUC => Ok(HangulJamoExtendedA::HangulChoseongRieulDashCieuc),
            HANGUL_CHOSEONG_RIEUL_DASH_KHIEUKH => Ok(HangulJamoExtendedA::HangulChoseongRieulDashKhieukh),
            HANGUL_CHOSEONG_MIEUM_DASH_KIYEOK => Ok(HangulJamoExtendedA::HangulChoseongMieumDashKiyeok),
            HANGUL_CHOSEONG_MIEUM_DASH_TIKEUT => Ok(HangulJamoExtendedA::HangulChoseongMieumDashTikeut),
            HANGUL_CHOSEONG_MIEUM_DASH_SIOS => Ok(HangulJamoExtendedA::HangulChoseongMieumDashSios),
            HANGUL_CHOSEONG_PIEUP_DASH_SIOS_DASH_THIEUTH => Ok(HangulJamoExtendedA::HangulChoseongPieupDashSiosDashThieuth),
            HANGUL_CHOSEONG_PIEUP_DASH_KHIEUKH => Ok(HangulJamoExtendedA::HangulChoseongPieupDashKhieukh),
            HANGUL_CHOSEONG_PIEUP_DASH_HIEUH => Ok(HangulJamoExtendedA::HangulChoseongPieupDashHieuh),
            HANGUL_CHOSEONG_SSANGSIOS_DASH_PIEUP => Ok(HangulJamoExtendedA::HangulChoseongSsangsiosDashPieup),
            HANGUL_CHOSEONG_IEUNG_DASH_RIEUL => Ok(HangulJamoExtendedA::HangulChoseongIeungDashRieul),
            HANGUL_CHOSEONG_IEUNG_DASH_HIEUH => Ok(HangulJamoExtendedA::HangulChoseongIeungDashHieuh),
            HANGUL_CHOSEONG_SSANGCIEUC_DASH_HIEUH => Ok(HangulJamoExtendedA::HangulChoseongSsangcieucDashHieuh),
            HANGUL_CHOSEONG_SSANGTHIEUTH => Ok(HangulJamoExtendedA::HangulChoseongSsangthieuth),
            HANGUL_CHOSEONG_PHIEUPH_DASH_HIEUH => Ok(HangulJamoExtendedA::HangulChoseongPhieuphDashHieuh),
            HANGUL_CHOSEONG_HIEUH_DASH_SIOS => Ok(HangulJamoExtendedA::HangulChoseongHieuhDashSios),
            HANGUL_CHOSEONG_SSANGYEORINHIEUH => Ok(HangulJamoExtendedA::HangulChoseongSsangyeorinhieuh),
            _ => Err(()),
        }
    }
}

impl Into<u32> for HangulJamoExtendedA {
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

impl std::convert::TryFrom<u32> for HangulJamoExtendedA {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for HangulJamoExtendedA {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl HangulJamoExtendedA {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        HangulJamoExtendedA::HangulChoseongTikeutDashMieum
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            HangulJamoExtendedA::HangulChoseongTikeutDashMieum => "hangul choseong tikeut-mieum",
            HangulJamoExtendedA::HangulChoseongTikeutDashPieup => "hangul choseong tikeut-pieup",
            HangulJamoExtendedA::HangulChoseongTikeutDashSios => "hangul choseong tikeut-sios",
            HangulJamoExtendedA::HangulChoseongTikeutDashCieuc => "hangul choseong tikeut-cieuc",
            HangulJamoExtendedA::HangulChoseongRieulDashKiyeok => "hangul choseong rieul-kiyeok",
            HangulJamoExtendedA::HangulChoseongRieulDashSsangkiyeok => "hangul choseong rieul-ssangkiyeok",
            HangulJamoExtendedA::HangulChoseongRieulDashTikeut => "hangul choseong rieul-tikeut",
            HangulJamoExtendedA::HangulChoseongRieulDashSsangtikeut => "hangul choseong rieul-ssangtikeut",
            HangulJamoExtendedA::HangulChoseongRieulDashMieum => "hangul choseong rieul-mieum",
            HangulJamoExtendedA::HangulChoseongRieulDashPieup => "hangul choseong rieul-pieup",
            HangulJamoExtendedA::HangulChoseongRieulDashSsangpieup => "hangul choseong rieul-ssangpieup",
            HangulJamoExtendedA::HangulChoseongRieulDashKapyeounpieup => "hangul choseong rieul-kapyeounpieup",
            HangulJamoExtendedA::HangulChoseongRieulDashSios => "hangul choseong rieul-sios",
            HangulJamoExtendedA::HangulChoseongRieulDashCieuc => "hangul choseong rieul-cieuc",
            HangulJamoExtendedA::HangulChoseongRieulDashKhieukh => "hangul choseong rieul-khieukh",
            HangulJamoExtendedA::HangulChoseongMieumDashKiyeok => "hangul choseong mieum-kiyeok",
            HangulJamoExtendedA::HangulChoseongMieumDashTikeut => "hangul choseong mieum-tikeut",
            HangulJamoExtendedA::HangulChoseongMieumDashSios => "hangul choseong mieum-sios",
            HangulJamoExtendedA::HangulChoseongPieupDashSiosDashThieuth => "hangul choseong pieup-sios-thieuth",
            HangulJamoExtendedA::HangulChoseongPieupDashKhieukh => "hangul choseong pieup-khieukh",
            HangulJamoExtendedA::HangulChoseongPieupDashHieuh => "hangul choseong pieup-hieuh",
            HangulJamoExtendedA::HangulChoseongSsangsiosDashPieup => "hangul choseong ssangsios-pieup",
            HangulJamoExtendedA::HangulChoseongIeungDashRieul => "hangul choseong ieung-rieul",
            HangulJamoExtendedA::HangulChoseongIeungDashHieuh => "hangul choseong ieung-hieuh",
            HangulJamoExtendedA::HangulChoseongSsangcieucDashHieuh => "hangul choseong ssangcieuc-hieuh",
            HangulJamoExtendedA::HangulChoseongSsangthieuth => "hangul choseong ssangthieuth",
            HangulJamoExtendedA::HangulChoseongPhieuphDashHieuh => "hangul choseong phieuph-hieuh",
            HangulJamoExtendedA::HangulChoseongHieuhDashSios => "hangul choseong hieuh-sios",
            HangulJamoExtendedA::HangulChoseongSsangyeorinhieuh => "hangul choseong ssangyeorinhieuh",
        }
    }
}
