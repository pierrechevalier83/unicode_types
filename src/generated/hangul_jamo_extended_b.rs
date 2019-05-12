
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
        match self {
            HangulJamoExtendedB::HangulJungseongODashYeo => 'ힰ',
            HangulJamoExtendedB::HangulJungseongODashODashI => 'ힱ',
            HangulJamoExtendedB::HangulJungseongYoDashA => 'ힲ',
            HangulJamoExtendedB::HangulJungseongYoDashAe => 'ힳ',
            HangulJamoExtendedB::HangulJungseongYoDashEo => 'ힴ',
            HangulJamoExtendedB::HangulJungseongUDashYeo => 'ힵ',
            HangulJamoExtendedB::HangulJungseongUDashIDashI => 'ힶ',
            HangulJamoExtendedB::HangulJungseongYuDashAe => 'ힷ',
            HangulJamoExtendedB::HangulJungseongYuDashO => 'ힸ',
            HangulJamoExtendedB::HangulJungseongEuDashA => 'ힹ',
            HangulJamoExtendedB::HangulJungseongEuDashEo => 'ힺ',
            HangulJamoExtendedB::HangulJungseongEuDashE => 'ힻ',
            HangulJamoExtendedB::HangulJungseongEuDashO => 'ힼ',
            HangulJamoExtendedB::HangulJungseongIDashYaDashO => 'ힽ',
            HangulJamoExtendedB::HangulJungseongIDashYae => 'ힾ',
            HangulJamoExtendedB::HangulJungseongIDashYeo => 'ힿ',
            HangulJamoExtendedB::HangulJungseongIDashYe => 'ퟀ',
            HangulJamoExtendedB::HangulJungseongIDashODashI => 'ퟁ',
            HangulJamoExtendedB::HangulJungseongIDashYo => 'ퟂ',
            HangulJamoExtendedB::HangulJungseongIDashYu => 'ퟃ',
            HangulJamoExtendedB::HangulJungseongIDashI => 'ퟄ',
            HangulJamoExtendedB::HangulJungseongAraeaDashA => 'ퟅ',
            HangulJamoExtendedB::HangulJungseongAraeaDashE => 'ퟆ',
            HangulJamoExtendedB::HangulJongseongNieunDashRieul => 'ퟋ',
            HangulJamoExtendedB::HangulJongseongNieunDashChieuch => 'ퟌ',
            HangulJamoExtendedB::HangulJongseongSsangtikeut => 'ퟍ',
            HangulJamoExtendedB::HangulJongseongSsangtikeutDashPieup => 'ퟎ',
            HangulJamoExtendedB::HangulJongseongTikeutDashPieup => 'ퟏ',
            HangulJamoExtendedB::HangulJongseongTikeutDashSios => 'ퟐ',
            HangulJamoExtendedB::HangulJongseongTikeutDashSiosDashKiyeok => 'ퟑ',
            HangulJamoExtendedB::HangulJongseongTikeutDashCieuc => 'ퟒ',
            HangulJamoExtendedB::HangulJongseongTikeutDashChieuch => 'ퟓ',
            HangulJamoExtendedB::HangulJongseongTikeutDashThieuth => 'ퟔ',
            HangulJamoExtendedB::HangulJongseongRieulDashSsangkiyeok => 'ퟕ',
            HangulJamoExtendedB::HangulJongseongRieulDashKiyeokDashHieuh => 'ퟖ',
            HangulJamoExtendedB::HangulJongseongSsangrieulDashKhieukh => 'ퟗ',
            HangulJamoExtendedB::HangulJongseongRieulDashMieumDashHieuh => 'ퟘ',
            HangulJamoExtendedB::HangulJongseongRieulDashPieupDashTikeut => 'ퟙ',
            HangulJamoExtendedB::HangulJongseongRieulDashPieupDashPhieuph => 'ퟚ',
            HangulJamoExtendedB::HangulJongseongRieulDashYesieung => 'ퟛ',
            HangulJamoExtendedB::HangulJongseongRieulDashYeorinhieuhDashHieuh => 'ퟜ',
            HangulJamoExtendedB::HangulJongseongKapyeounrieul => 'ퟝ',
            HangulJamoExtendedB::HangulJongseongMieumDashNieun => 'ퟞ',
            HangulJamoExtendedB::HangulJongseongMieumDashSsangnieun => 'ퟟ',
            HangulJamoExtendedB::HangulJongseongSsangmieum => 'ퟠ',
            HangulJamoExtendedB::HangulJongseongMieumDashPieupDashSios => 'ퟡ',
            HangulJamoExtendedB::HangulJongseongMieumDashCieuc => 'ퟢ',
            HangulJamoExtendedB::HangulJongseongPieupDashTikeut => 'ퟣ',
            HangulJamoExtendedB::HangulJongseongPieupDashRieulDashPhieuph => 'ퟤ',
            HangulJamoExtendedB::HangulJongseongPieupDashMieum => 'ퟥ',
            HangulJamoExtendedB::HangulJongseongSsangpieup => 'ퟦ',
            HangulJamoExtendedB::HangulJongseongPieupDashSiosDashTikeut => 'ퟧ',
            HangulJamoExtendedB::HangulJongseongPieupDashCieuc => 'ퟨ',
            HangulJamoExtendedB::HangulJongseongPieupDashChieuch => 'ퟩ',
            HangulJamoExtendedB::HangulJongseongSiosDashMieum => 'ퟪ',
            HangulJamoExtendedB::HangulJongseongSiosDashKapyeounpieup => 'ퟫ',
            HangulJamoExtendedB::HangulJongseongSsangsiosDashKiyeok => 'ퟬ',
            HangulJamoExtendedB::HangulJongseongSsangsiosDashTikeut => 'ퟭ',
            HangulJamoExtendedB::HangulJongseongSiosDashPansios => 'ퟮ',
            HangulJamoExtendedB::HangulJongseongSiosDashCieuc => 'ퟯ',
            HangulJamoExtendedB::HangulJongseongSiosDashChieuch => 'ퟰ',
            HangulJamoExtendedB::HangulJongseongSiosDashThieuth => 'ퟱ',
            HangulJamoExtendedB::HangulJongseongSiosDashHieuh => 'ퟲ',
            HangulJamoExtendedB::HangulJongseongPansiosDashPieup => 'ퟳ',
            HangulJamoExtendedB::HangulJongseongPansiosDashKapyeounpieup => 'ퟴ',
            HangulJamoExtendedB::HangulJongseongYesieungDashMieum => 'ퟵ',
            HangulJamoExtendedB::HangulJongseongYesieungDashHieuh => 'ퟶ',
            HangulJamoExtendedB::HangulJongseongCieucDashPieup => 'ퟷ',
            HangulJamoExtendedB::HangulJongseongCieucDashSsangpieup => 'ퟸ',
            HangulJamoExtendedB::HangulJongseongSsangcieuc => 'ퟹ',
            HangulJamoExtendedB::HangulJongseongPhieuphDashSios => 'ퟺ',
            HangulJamoExtendedB::HangulJongseongPhieuphDashThieuth => 'ퟻ',
        }
    }
}

impl std::convert::TryFrom<char> for HangulJamoExtendedB {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ힰ' => Ok(HangulJamoExtendedB::HangulJungseongODashYeo),
            'ힱ' => Ok(HangulJamoExtendedB::HangulJungseongODashODashI),
            'ힲ' => Ok(HangulJamoExtendedB::HangulJungseongYoDashA),
            'ힳ' => Ok(HangulJamoExtendedB::HangulJungseongYoDashAe),
            'ힴ' => Ok(HangulJamoExtendedB::HangulJungseongYoDashEo),
            'ힵ' => Ok(HangulJamoExtendedB::HangulJungseongUDashYeo),
            'ힶ' => Ok(HangulJamoExtendedB::HangulJungseongUDashIDashI),
            'ힷ' => Ok(HangulJamoExtendedB::HangulJungseongYuDashAe),
            'ힸ' => Ok(HangulJamoExtendedB::HangulJungseongYuDashO),
            'ힹ' => Ok(HangulJamoExtendedB::HangulJungseongEuDashA),
            'ힺ' => Ok(HangulJamoExtendedB::HangulJungseongEuDashEo),
            'ힻ' => Ok(HangulJamoExtendedB::HangulJungseongEuDashE),
            'ힼ' => Ok(HangulJamoExtendedB::HangulJungseongEuDashO),
            'ힽ' => Ok(HangulJamoExtendedB::HangulJungseongIDashYaDashO),
            'ힾ' => Ok(HangulJamoExtendedB::HangulJungseongIDashYae),
            'ힿ' => Ok(HangulJamoExtendedB::HangulJungseongIDashYeo),
            'ퟀ' => Ok(HangulJamoExtendedB::HangulJungseongIDashYe),
            'ퟁ' => Ok(HangulJamoExtendedB::HangulJungseongIDashODashI),
            'ퟂ' => Ok(HangulJamoExtendedB::HangulJungseongIDashYo),
            'ퟃ' => Ok(HangulJamoExtendedB::HangulJungseongIDashYu),
            'ퟄ' => Ok(HangulJamoExtendedB::HangulJungseongIDashI),
            'ퟅ' => Ok(HangulJamoExtendedB::HangulJungseongAraeaDashA),
            'ퟆ' => Ok(HangulJamoExtendedB::HangulJungseongAraeaDashE),
            'ퟋ' => Ok(HangulJamoExtendedB::HangulJongseongNieunDashRieul),
            'ퟌ' => Ok(HangulJamoExtendedB::HangulJongseongNieunDashChieuch),
            'ퟍ' => Ok(HangulJamoExtendedB::HangulJongseongSsangtikeut),
            'ퟎ' => Ok(HangulJamoExtendedB::HangulJongseongSsangtikeutDashPieup),
            'ퟏ' => Ok(HangulJamoExtendedB::HangulJongseongTikeutDashPieup),
            'ퟐ' => Ok(HangulJamoExtendedB::HangulJongseongTikeutDashSios),
            'ퟑ' => Ok(HangulJamoExtendedB::HangulJongseongTikeutDashSiosDashKiyeok),
            'ퟒ' => Ok(HangulJamoExtendedB::HangulJongseongTikeutDashCieuc),
            'ퟓ' => Ok(HangulJamoExtendedB::HangulJongseongTikeutDashChieuch),
            'ퟔ' => Ok(HangulJamoExtendedB::HangulJongseongTikeutDashThieuth),
            'ퟕ' => Ok(HangulJamoExtendedB::HangulJongseongRieulDashSsangkiyeok),
            'ퟖ' => Ok(HangulJamoExtendedB::HangulJongseongRieulDashKiyeokDashHieuh),
            'ퟗ' => Ok(HangulJamoExtendedB::HangulJongseongSsangrieulDashKhieukh),
            'ퟘ' => Ok(HangulJamoExtendedB::HangulJongseongRieulDashMieumDashHieuh),
            'ퟙ' => Ok(HangulJamoExtendedB::HangulJongseongRieulDashPieupDashTikeut),
            'ퟚ' => Ok(HangulJamoExtendedB::HangulJongseongRieulDashPieupDashPhieuph),
            'ퟛ' => Ok(HangulJamoExtendedB::HangulJongseongRieulDashYesieung),
            'ퟜ' => Ok(HangulJamoExtendedB::HangulJongseongRieulDashYeorinhieuhDashHieuh),
            'ퟝ' => Ok(HangulJamoExtendedB::HangulJongseongKapyeounrieul),
            'ퟞ' => Ok(HangulJamoExtendedB::HangulJongseongMieumDashNieun),
            'ퟟ' => Ok(HangulJamoExtendedB::HangulJongseongMieumDashSsangnieun),
            'ퟠ' => Ok(HangulJamoExtendedB::HangulJongseongSsangmieum),
            'ퟡ' => Ok(HangulJamoExtendedB::HangulJongseongMieumDashPieupDashSios),
            'ퟢ' => Ok(HangulJamoExtendedB::HangulJongseongMieumDashCieuc),
            'ퟣ' => Ok(HangulJamoExtendedB::HangulJongseongPieupDashTikeut),
            'ퟤ' => Ok(HangulJamoExtendedB::HangulJongseongPieupDashRieulDashPhieuph),
            'ퟥ' => Ok(HangulJamoExtendedB::HangulJongseongPieupDashMieum),
            'ퟦ' => Ok(HangulJamoExtendedB::HangulJongseongSsangpieup),
            'ퟧ' => Ok(HangulJamoExtendedB::HangulJongseongPieupDashSiosDashTikeut),
            'ퟨ' => Ok(HangulJamoExtendedB::HangulJongseongPieupDashCieuc),
            'ퟩ' => Ok(HangulJamoExtendedB::HangulJongseongPieupDashChieuch),
            'ퟪ' => Ok(HangulJamoExtendedB::HangulJongseongSiosDashMieum),
            'ퟫ' => Ok(HangulJamoExtendedB::HangulJongseongSiosDashKapyeounpieup),
            'ퟬ' => Ok(HangulJamoExtendedB::HangulJongseongSsangsiosDashKiyeok),
            'ퟭ' => Ok(HangulJamoExtendedB::HangulJongseongSsangsiosDashTikeut),
            'ퟮ' => Ok(HangulJamoExtendedB::HangulJongseongSiosDashPansios),
            'ퟯ' => Ok(HangulJamoExtendedB::HangulJongseongSiosDashCieuc),
            'ퟰ' => Ok(HangulJamoExtendedB::HangulJongseongSiosDashChieuch),
            'ퟱ' => Ok(HangulJamoExtendedB::HangulJongseongSiosDashThieuth),
            'ퟲ' => Ok(HangulJamoExtendedB::HangulJongseongSiosDashHieuh),
            'ퟳ' => Ok(HangulJamoExtendedB::HangulJongseongPansiosDashPieup),
            'ퟴ' => Ok(HangulJamoExtendedB::HangulJongseongPansiosDashKapyeounpieup),
            'ퟵ' => Ok(HangulJamoExtendedB::HangulJongseongYesieungDashMieum),
            'ퟶ' => Ok(HangulJamoExtendedB::HangulJongseongYesieungDashHieuh),
            'ퟷ' => Ok(HangulJamoExtendedB::HangulJongseongCieucDashPieup),
            'ퟸ' => Ok(HangulJamoExtendedB::HangulJongseongCieucDashSsangpieup),
            'ퟹ' => Ok(HangulJamoExtendedB::HangulJongseongSsangcieuc),
            'ퟺ' => Ok(HangulJamoExtendedB::HangulJongseongPhieuphDashSios),
            'ퟻ' => Ok(HangulJamoExtendedB::HangulJongseongPhieuphDashThieuth),
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
