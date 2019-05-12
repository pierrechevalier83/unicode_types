
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
        match self {
            HangulJamoExtendedA::HangulChoseongTikeutDashMieum => 'ꥠ',
            HangulJamoExtendedA::HangulChoseongTikeutDashPieup => 'ꥡ',
            HangulJamoExtendedA::HangulChoseongTikeutDashSios => 'ꥢ',
            HangulJamoExtendedA::HangulChoseongTikeutDashCieuc => 'ꥣ',
            HangulJamoExtendedA::HangulChoseongRieulDashKiyeok => 'ꥤ',
            HangulJamoExtendedA::HangulChoseongRieulDashSsangkiyeok => 'ꥥ',
            HangulJamoExtendedA::HangulChoseongRieulDashTikeut => 'ꥦ',
            HangulJamoExtendedA::HangulChoseongRieulDashSsangtikeut => 'ꥧ',
            HangulJamoExtendedA::HangulChoseongRieulDashMieum => 'ꥨ',
            HangulJamoExtendedA::HangulChoseongRieulDashPieup => 'ꥩ',
            HangulJamoExtendedA::HangulChoseongRieulDashSsangpieup => 'ꥪ',
            HangulJamoExtendedA::HangulChoseongRieulDashKapyeounpieup => 'ꥫ',
            HangulJamoExtendedA::HangulChoseongRieulDashSios => 'ꥬ',
            HangulJamoExtendedA::HangulChoseongRieulDashCieuc => 'ꥭ',
            HangulJamoExtendedA::HangulChoseongRieulDashKhieukh => 'ꥮ',
            HangulJamoExtendedA::HangulChoseongMieumDashKiyeok => 'ꥯ',
            HangulJamoExtendedA::HangulChoseongMieumDashTikeut => 'ꥰ',
            HangulJamoExtendedA::HangulChoseongMieumDashSios => 'ꥱ',
            HangulJamoExtendedA::HangulChoseongPieupDashSiosDashThieuth => 'ꥲ',
            HangulJamoExtendedA::HangulChoseongPieupDashKhieukh => 'ꥳ',
            HangulJamoExtendedA::HangulChoseongPieupDashHieuh => 'ꥴ',
            HangulJamoExtendedA::HangulChoseongSsangsiosDashPieup => 'ꥵ',
            HangulJamoExtendedA::HangulChoseongIeungDashRieul => 'ꥶ',
            HangulJamoExtendedA::HangulChoseongIeungDashHieuh => 'ꥷ',
            HangulJamoExtendedA::HangulChoseongSsangcieucDashHieuh => 'ꥸ',
            HangulJamoExtendedA::HangulChoseongSsangthieuth => 'ꥹ',
            HangulJamoExtendedA::HangulChoseongPhieuphDashHieuh => 'ꥺ',
            HangulJamoExtendedA::HangulChoseongHieuhDashSios => 'ꥻ',
            HangulJamoExtendedA::HangulChoseongSsangyeorinhieuh => 'ꥼ',
        }
    }
}

impl std::convert::TryFrom<char> for HangulJamoExtendedA {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ꥠ' => Ok(HangulJamoExtendedA::HangulChoseongTikeutDashMieum),
            'ꥡ' => Ok(HangulJamoExtendedA::HangulChoseongTikeutDashPieup),
            'ꥢ' => Ok(HangulJamoExtendedA::HangulChoseongTikeutDashSios),
            'ꥣ' => Ok(HangulJamoExtendedA::HangulChoseongTikeutDashCieuc),
            'ꥤ' => Ok(HangulJamoExtendedA::HangulChoseongRieulDashKiyeok),
            'ꥥ' => Ok(HangulJamoExtendedA::HangulChoseongRieulDashSsangkiyeok),
            'ꥦ' => Ok(HangulJamoExtendedA::HangulChoseongRieulDashTikeut),
            'ꥧ' => Ok(HangulJamoExtendedA::HangulChoseongRieulDashSsangtikeut),
            'ꥨ' => Ok(HangulJamoExtendedA::HangulChoseongRieulDashMieum),
            'ꥩ' => Ok(HangulJamoExtendedA::HangulChoseongRieulDashPieup),
            'ꥪ' => Ok(HangulJamoExtendedA::HangulChoseongRieulDashSsangpieup),
            'ꥫ' => Ok(HangulJamoExtendedA::HangulChoseongRieulDashKapyeounpieup),
            'ꥬ' => Ok(HangulJamoExtendedA::HangulChoseongRieulDashSios),
            'ꥭ' => Ok(HangulJamoExtendedA::HangulChoseongRieulDashCieuc),
            'ꥮ' => Ok(HangulJamoExtendedA::HangulChoseongRieulDashKhieukh),
            'ꥯ' => Ok(HangulJamoExtendedA::HangulChoseongMieumDashKiyeok),
            'ꥰ' => Ok(HangulJamoExtendedA::HangulChoseongMieumDashTikeut),
            'ꥱ' => Ok(HangulJamoExtendedA::HangulChoseongMieumDashSios),
            'ꥲ' => Ok(HangulJamoExtendedA::HangulChoseongPieupDashSiosDashThieuth),
            'ꥳ' => Ok(HangulJamoExtendedA::HangulChoseongPieupDashKhieukh),
            'ꥴ' => Ok(HangulJamoExtendedA::HangulChoseongPieupDashHieuh),
            'ꥵ' => Ok(HangulJamoExtendedA::HangulChoseongSsangsiosDashPieup),
            'ꥶ' => Ok(HangulJamoExtendedA::HangulChoseongIeungDashRieul),
            'ꥷ' => Ok(HangulJamoExtendedA::HangulChoseongIeungDashHieuh),
            'ꥸ' => Ok(HangulJamoExtendedA::HangulChoseongSsangcieucDashHieuh),
            'ꥹ' => Ok(HangulJamoExtendedA::HangulChoseongSsangthieuth),
            'ꥺ' => Ok(HangulJamoExtendedA::HangulChoseongPhieuphDashHieuh),
            'ꥻ' => Ok(HangulJamoExtendedA::HangulChoseongHieuhDashSios),
            'ꥼ' => Ok(HangulJamoExtendedA::HangulChoseongSsangyeorinhieuh),
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
