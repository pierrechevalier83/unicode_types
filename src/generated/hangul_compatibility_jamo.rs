
/// An enum to represent all characters in the HangulCompatibilityJamo block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum HangulCompatibilityJamo {
    /// \u{3131}: 'ㄱ'
    HangulLetterKiyeok,
    /// \u{3132}: 'ㄲ'
    HangulLetterSsangkiyeok,
    /// \u{3133}: 'ㄳ'
    HangulLetterKiyeokDashSios,
    /// \u{3134}: 'ㄴ'
    HangulLetterNieun,
    /// \u{3135}: 'ㄵ'
    HangulLetterNieunDashCieuc,
    /// \u{3136}: 'ㄶ'
    HangulLetterNieunDashHieuh,
    /// \u{3137}: 'ㄷ'
    HangulLetterTikeut,
    /// \u{3138}: 'ㄸ'
    HangulLetterSsangtikeut,
    /// \u{3139}: 'ㄹ'
    HangulLetterRieul,
    /// \u{313a}: 'ㄺ'
    HangulLetterRieulDashKiyeok,
    /// \u{313b}: 'ㄻ'
    HangulLetterRieulDashMieum,
    /// \u{313c}: 'ㄼ'
    HangulLetterRieulDashPieup,
    /// \u{313d}: 'ㄽ'
    HangulLetterRieulDashSios,
    /// \u{313e}: 'ㄾ'
    HangulLetterRieulDashThieuth,
    /// \u{313f}: 'ㄿ'
    HangulLetterRieulDashPhieuph,
    /// \u{3140}: 'ㅀ'
    HangulLetterRieulDashHieuh,
    /// \u{3141}: 'ㅁ'
    HangulLetterMieum,
    /// \u{3142}: 'ㅂ'
    HangulLetterPieup,
    /// \u{3143}: 'ㅃ'
    HangulLetterSsangpieup,
    /// \u{3144}: 'ㅄ'
    HangulLetterPieupDashSios,
    /// \u{3145}: 'ㅅ'
    HangulLetterSios,
    /// \u{3146}: 'ㅆ'
    HangulLetterSsangsios,
    /// \u{3147}: 'ㅇ'
    HangulLetterIeung,
    /// \u{3148}: 'ㅈ'
    HangulLetterCieuc,
    /// \u{3149}: 'ㅉ'
    HangulLetterSsangcieuc,
    /// \u{314a}: 'ㅊ'
    HangulLetterChieuch,
    /// \u{314b}: 'ㅋ'
    HangulLetterKhieukh,
    /// \u{314c}: 'ㅌ'
    HangulLetterThieuth,
    /// \u{314d}: 'ㅍ'
    HangulLetterPhieuph,
    /// \u{314e}: 'ㅎ'
    HangulLetterHieuh,
    /// \u{314f}: 'ㅏ'
    HangulLetterA,
    /// \u{3150}: 'ㅐ'
    HangulLetterAe,
    /// \u{3151}: 'ㅑ'
    HangulLetterYa,
    /// \u{3152}: 'ㅒ'
    HangulLetterYae,
    /// \u{3153}: 'ㅓ'
    HangulLetterEo,
    /// \u{3154}: 'ㅔ'
    HangulLetterE,
    /// \u{3155}: 'ㅕ'
    HangulLetterYeo,
    /// \u{3156}: 'ㅖ'
    HangulLetterYe,
    /// \u{3157}: 'ㅗ'
    HangulLetterO,
    /// \u{3158}: 'ㅘ'
    HangulLetterWa,
    /// \u{3159}: 'ㅙ'
    HangulLetterWae,
    /// \u{315a}: 'ㅚ'
    HangulLetterOe,
    /// \u{315b}: 'ㅛ'
    HangulLetterYo,
    /// \u{315c}: 'ㅜ'
    HangulLetterU,
    /// \u{315d}: 'ㅝ'
    HangulLetterWeo,
    /// \u{315e}: 'ㅞ'
    HangulLetterWe,
    /// \u{315f}: 'ㅟ'
    HangulLetterWi,
    /// \u{3160}: 'ㅠ'
    HangulLetterYu,
    /// \u{3161}: 'ㅡ'
    HangulLetterEu,
    /// \u{3162}: 'ㅢ'
    HangulLetterYi,
    /// \u{3163}: 'ㅣ'
    HangulLetterI,
    /// \u{3164}: 'ㅤ'
    HangulFiller,
    /// \u{3165}: 'ㅥ'
    HangulLetterSsangnieun,
    /// \u{3166}: 'ㅦ'
    HangulLetterNieunDashTikeut,
    /// \u{3167}: 'ㅧ'
    HangulLetterNieunDashSios,
    /// \u{3168}: 'ㅨ'
    HangulLetterNieunDashPansios,
    /// \u{3169}: 'ㅩ'
    HangulLetterRieulDashKiyeokDashSios,
    /// \u{316a}: 'ㅪ'
    HangulLetterRieulDashTikeut,
    /// \u{316b}: 'ㅫ'
    HangulLetterRieulDashPieupDashSios,
    /// \u{316c}: 'ㅬ'
    HangulLetterRieulDashPansios,
    /// \u{316d}: 'ㅭ'
    HangulLetterRieulDashYeorinhieuh,
    /// \u{316e}: 'ㅮ'
    HangulLetterMieumDashPieup,
    /// \u{316f}: 'ㅯ'
    HangulLetterMieumDashSios,
    /// \u{3170}: 'ㅰ'
    HangulLetterMieumDashPansios,
    /// \u{3171}: 'ㅱ'
    HangulLetterKapyeounmieum,
    /// \u{3172}: 'ㅲ'
    HangulLetterPieupDashKiyeok,
    /// \u{3173}: 'ㅳ'
    HangulLetterPieupDashTikeut,
    /// \u{3174}: 'ㅴ'
    HangulLetterPieupDashSiosDashKiyeok,
    /// \u{3175}: 'ㅵ'
    HangulLetterPieupDashSiosDashTikeut,
    /// \u{3176}: 'ㅶ'
    HangulLetterPieupDashCieuc,
    /// \u{3177}: 'ㅷ'
    HangulLetterPieupDashThieuth,
    /// \u{3178}: 'ㅸ'
    HangulLetterKapyeounpieup,
    /// \u{3179}: 'ㅹ'
    HangulLetterKapyeounssangpieup,
    /// \u{317a}: 'ㅺ'
    HangulLetterSiosDashKiyeok,
    /// \u{317b}: 'ㅻ'
    HangulLetterSiosDashNieun,
    /// \u{317c}: 'ㅼ'
    HangulLetterSiosDashTikeut,
    /// \u{317d}: 'ㅽ'
    HangulLetterSiosDashPieup,
    /// \u{317e}: 'ㅾ'
    HangulLetterSiosDashCieuc,
    /// \u{317f}: 'ㅿ'
    HangulLetterPansios,
    /// \u{3180}: 'ㆀ'
    HangulLetterSsangieung,
    /// \u{3181}: 'ㆁ'
    HangulLetterYesieung,
    /// \u{3182}: 'ㆂ'
    HangulLetterYesieungDashSios,
    /// \u{3183}: 'ㆃ'
    HangulLetterYesieungDashPansios,
    /// \u{3184}: 'ㆄ'
    HangulLetterKapyeounphieuph,
    /// \u{3185}: 'ㆅ'
    HangulLetterSsanghieuh,
    /// \u{3186}: 'ㆆ'
    HangulLetterYeorinhieuh,
    /// \u{3187}: 'ㆇ'
    HangulLetterYoDashYa,
    /// \u{3188}: 'ㆈ'
    HangulLetterYoDashYae,
    /// \u{3189}: 'ㆉ'
    HangulLetterYoDashI,
    /// \u{318a}: 'ㆊ'
    HangulLetterYuDashYeo,
    /// \u{318b}: 'ㆋ'
    HangulLetterYuDashYe,
    /// \u{318c}: 'ㆌ'
    HangulLetterYuDashI,
    /// \u{318d}: 'ㆍ'
    HangulLetterAraea,
    /// \u{318e}: 'ㆎ'
    HangulLetterAraeae,
}

impl Into<char> for HangulCompatibilityJamo {
    fn into(self) -> char {
        match self {
            HangulCompatibilityJamo::HangulLetterKiyeok => 'ㄱ',
            HangulCompatibilityJamo::HangulLetterSsangkiyeok => 'ㄲ',
            HangulCompatibilityJamo::HangulLetterKiyeokDashSios => 'ㄳ',
            HangulCompatibilityJamo::HangulLetterNieun => 'ㄴ',
            HangulCompatibilityJamo::HangulLetterNieunDashCieuc => 'ㄵ',
            HangulCompatibilityJamo::HangulLetterNieunDashHieuh => 'ㄶ',
            HangulCompatibilityJamo::HangulLetterTikeut => 'ㄷ',
            HangulCompatibilityJamo::HangulLetterSsangtikeut => 'ㄸ',
            HangulCompatibilityJamo::HangulLetterRieul => 'ㄹ',
            HangulCompatibilityJamo::HangulLetterRieulDashKiyeok => 'ㄺ',
            HangulCompatibilityJamo::HangulLetterRieulDashMieum => 'ㄻ',
            HangulCompatibilityJamo::HangulLetterRieulDashPieup => 'ㄼ',
            HangulCompatibilityJamo::HangulLetterRieulDashSios => 'ㄽ',
            HangulCompatibilityJamo::HangulLetterRieulDashThieuth => 'ㄾ',
            HangulCompatibilityJamo::HangulLetterRieulDashPhieuph => 'ㄿ',
            HangulCompatibilityJamo::HangulLetterRieulDashHieuh => 'ㅀ',
            HangulCompatibilityJamo::HangulLetterMieum => 'ㅁ',
            HangulCompatibilityJamo::HangulLetterPieup => 'ㅂ',
            HangulCompatibilityJamo::HangulLetterSsangpieup => 'ㅃ',
            HangulCompatibilityJamo::HangulLetterPieupDashSios => 'ㅄ',
            HangulCompatibilityJamo::HangulLetterSios => 'ㅅ',
            HangulCompatibilityJamo::HangulLetterSsangsios => 'ㅆ',
            HangulCompatibilityJamo::HangulLetterIeung => 'ㅇ',
            HangulCompatibilityJamo::HangulLetterCieuc => 'ㅈ',
            HangulCompatibilityJamo::HangulLetterSsangcieuc => 'ㅉ',
            HangulCompatibilityJamo::HangulLetterChieuch => 'ㅊ',
            HangulCompatibilityJamo::HangulLetterKhieukh => 'ㅋ',
            HangulCompatibilityJamo::HangulLetterThieuth => 'ㅌ',
            HangulCompatibilityJamo::HangulLetterPhieuph => 'ㅍ',
            HangulCompatibilityJamo::HangulLetterHieuh => 'ㅎ',
            HangulCompatibilityJamo::HangulLetterA => 'ㅏ',
            HangulCompatibilityJamo::HangulLetterAe => 'ㅐ',
            HangulCompatibilityJamo::HangulLetterYa => 'ㅑ',
            HangulCompatibilityJamo::HangulLetterYae => 'ㅒ',
            HangulCompatibilityJamo::HangulLetterEo => 'ㅓ',
            HangulCompatibilityJamo::HangulLetterE => 'ㅔ',
            HangulCompatibilityJamo::HangulLetterYeo => 'ㅕ',
            HangulCompatibilityJamo::HangulLetterYe => 'ㅖ',
            HangulCompatibilityJamo::HangulLetterO => 'ㅗ',
            HangulCompatibilityJamo::HangulLetterWa => 'ㅘ',
            HangulCompatibilityJamo::HangulLetterWae => 'ㅙ',
            HangulCompatibilityJamo::HangulLetterOe => 'ㅚ',
            HangulCompatibilityJamo::HangulLetterYo => 'ㅛ',
            HangulCompatibilityJamo::HangulLetterU => 'ㅜ',
            HangulCompatibilityJamo::HangulLetterWeo => 'ㅝ',
            HangulCompatibilityJamo::HangulLetterWe => 'ㅞ',
            HangulCompatibilityJamo::HangulLetterWi => 'ㅟ',
            HangulCompatibilityJamo::HangulLetterYu => 'ㅠ',
            HangulCompatibilityJamo::HangulLetterEu => 'ㅡ',
            HangulCompatibilityJamo::HangulLetterYi => 'ㅢ',
            HangulCompatibilityJamo::HangulLetterI => 'ㅣ',
            HangulCompatibilityJamo::HangulFiller => 'ㅤ',
            HangulCompatibilityJamo::HangulLetterSsangnieun => 'ㅥ',
            HangulCompatibilityJamo::HangulLetterNieunDashTikeut => 'ㅦ',
            HangulCompatibilityJamo::HangulLetterNieunDashSios => 'ㅧ',
            HangulCompatibilityJamo::HangulLetterNieunDashPansios => 'ㅨ',
            HangulCompatibilityJamo::HangulLetterRieulDashKiyeokDashSios => 'ㅩ',
            HangulCompatibilityJamo::HangulLetterRieulDashTikeut => 'ㅪ',
            HangulCompatibilityJamo::HangulLetterRieulDashPieupDashSios => 'ㅫ',
            HangulCompatibilityJamo::HangulLetterRieulDashPansios => 'ㅬ',
            HangulCompatibilityJamo::HangulLetterRieulDashYeorinhieuh => 'ㅭ',
            HangulCompatibilityJamo::HangulLetterMieumDashPieup => 'ㅮ',
            HangulCompatibilityJamo::HangulLetterMieumDashSios => 'ㅯ',
            HangulCompatibilityJamo::HangulLetterMieumDashPansios => 'ㅰ',
            HangulCompatibilityJamo::HangulLetterKapyeounmieum => 'ㅱ',
            HangulCompatibilityJamo::HangulLetterPieupDashKiyeok => 'ㅲ',
            HangulCompatibilityJamo::HangulLetterPieupDashTikeut => 'ㅳ',
            HangulCompatibilityJamo::HangulLetterPieupDashSiosDashKiyeok => 'ㅴ',
            HangulCompatibilityJamo::HangulLetterPieupDashSiosDashTikeut => 'ㅵ',
            HangulCompatibilityJamo::HangulLetterPieupDashCieuc => 'ㅶ',
            HangulCompatibilityJamo::HangulLetterPieupDashThieuth => 'ㅷ',
            HangulCompatibilityJamo::HangulLetterKapyeounpieup => 'ㅸ',
            HangulCompatibilityJamo::HangulLetterKapyeounssangpieup => 'ㅹ',
            HangulCompatibilityJamo::HangulLetterSiosDashKiyeok => 'ㅺ',
            HangulCompatibilityJamo::HangulLetterSiosDashNieun => 'ㅻ',
            HangulCompatibilityJamo::HangulLetterSiosDashTikeut => 'ㅼ',
            HangulCompatibilityJamo::HangulLetterSiosDashPieup => 'ㅽ',
            HangulCompatibilityJamo::HangulLetterSiosDashCieuc => 'ㅾ',
            HangulCompatibilityJamo::HangulLetterPansios => 'ㅿ',
            HangulCompatibilityJamo::HangulLetterSsangieung => 'ㆀ',
            HangulCompatibilityJamo::HangulLetterYesieung => 'ㆁ',
            HangulCompatibilityJamo::HangulLetterYesieungDashSios => 'ㆂ',
            HangulCompatibilityJamo::HangulLetterYesieungDashPansios => 'ㆃ',
            HangulCompatibilityJamo::HangulLetterKapyeounphieuph => 'ㆄ',
            HangulCompatibilityJamo::HangulLetterSsanghieuh => 'ㆅ',
            HangulCompatibilityJamo::HangulLetterYeorinhieuh => 'ㆆ',
            HangulCompatibilityJamo::HangulLetterYoDashYa => 'ㆇ',
            HangulCompatibilityJamo::HangulLetterYoDashYae => 'ㆈ',
            HangulCompatibilityJamo::HangulLetterYoDashI => 'ㆉ',
            HangulCompatibilityJamo::HangulLetterYuDashYeo => 'ㆊ',
            HangulCompatibilityJamo::HangulLetterYuDashYe => 'ㆋ',
            HangulCompatibilityJamo::HangulLetterYuDashI => 'ㆌ',
            HangulCompatibilityJamo::HangulLetterAraea => 'ㆍ',
            HangulCompatibilityJamo::HangulLetterAraeae => 'ㆎ',
        }
    }
}

impl std::convert::TryFrom<char> for HangulCompatibilityJamo {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ㄱ' => Ok(HangulCompatibilityJamo::HangulLetterKiyeok),
            'ㄲ' => Ok(HangulCompatibilityJamo::HangulLetterSsangkiyeok),
            'ㄳ' => Ok(HangulCompatibilityJamo::HangulLetterKiyeokDashSios),
            'ㄴ' => Ok(HangulCompatibilityJamo::HangulLetterNieun),
            'ㄵ' => Ok(HangulCompatibilityJamo::HangulLetterNieunDashCieuc),
            'ㄶ' => Ok(HangulCompatibilityJamo::HangulLetterNieunDashHieuh),
            'ㄷ' => Ok(HangulCompatibilityJamo::HangulLetterTikeut),
            'ㄸ' => Ok(HangulCompatibilityJamo::HangulLetterSsangtikeut),
            'ㄹ' => Ok(HangulCompatibilityJamo::HangulLetterRieul),
            'ㄺ' => Ok(HangulCompatibilityJamo::HangulLetterRieulDashKiyeok),
            'ㄻ' => Ok(HangulCompatibilityJamo::HangulLetterRieulDashMieum),
            'ㄼ' => Ok(HangulCompatibilityJamo::HangulLetterRieulDashPieup),
            'ㄽ' => Ok(HangulCompatibilityJamo::HangulLetterRieulDashSios),
            'ㄾ' => Ok(HangulCompatibilityJamo::HangulLetterRieulDashThieuth),
            'ㄿ' => Ok(HangulCompatibilityJamo::HangulLetterRieulDashPhieuph),
            'ㅀ' => Ok(HangulCompatibilityJamo::HangulLetterRieulDashHieuh),
            'ㅁ' => Ok(HangulCompatibilityJamo::HangulLetterMieum),
            'ㅂ' => Ok(HangulCompatibilityJamo::HangulLetterPieup),
            'ㅃ' => Ok(HangulCompatibilityJamo::HangulLetterSsangpieup),
            'ㅄ' => Ok(HangulCompatibilityJamo::HangulLetterPieupDashSios),
            'ㅅ' => Ok(HangulCompatibilityJamo::HangulLetterSios),
            'ㅆ' => Ok(HangulCompatibilityJamo::HangulLetterSsangsios),
            'ㅇ' => Ok(HangulCompatibilityJamo::HangulLetterIeung),
            'ㅈ' => Ok(HangulCompatibilityJamo::HangulLetterCieuc),
            'ㅉ' => Ok(HangulCompatibilityJamo::HangulLetterSsangcieuc),
            'ㅊ' => Ok(HangulCompatibilityJamo::HangulLetterChieuch),
            'ㅋ' => Ok(HangulCompatibilityJamo::HangulLetterKhieukh),
            'ㅌ' => Ok(HangulCompatibilityJamo::HangulLetterThieuth),
            'ㅍ' => Ok(HangulCompatibilityJamo::HangulLetterPhieuph),
            'ㅎ' => Ok(HangulCompatibilityJamo::HangulLetterHieuh),
            'ㅏ' => Ok(HangulCompatibilityJamo::HangulLetterA),
            'ㅐ' => Ok(HangulCompatibilityJamo::HangulLetterAe),
            'ㅑ' => Ok(HangulCompatibilityJamo::HangulLetterYa),
            'ㅒ' => Ok(HangulCompatibilityJamo::HangulLetterYae),
            'ㅓ' => Ok(HangulCompatibilityJamo::HangulLetterEo),
            'ㅔ' => Ok(HangulCompatibilityJamo::HangulLetterE),
            'ㅕ' => Ok(HangulCompatibilityJamo::HangulLetterYeo),
            'ㅖ' => Ok(HangulCompatibilityJamo::HangulLetterYe),
            'ㅗ' => Ok(HangulCompatibilityJamo::HangulLetterO),
            'ㅘ' => Ok(HangulCompatibilityJamo::HangulLetterWa),
            'ㅙ' => Ok(HangulCompatibilityJamo::HangulLetterWae),
            'ㅚ' => Ok(HangulCompatibilityJamo::HangulLetterOe),
            'ㅛ' => Ok(HangulCompatibilityJamo::HangulLetterYo),
            'ㅜ' => Ok(HangulCompatibilityJamo::HangulLetterU),
            'ㅝ' => Ok(HangulCompatibilityJamo::HangulLetterWeo),
            'ㅞ' => Ok(HangulCompatibilityJamo::HangulLetterWe),
            'ㅟ' => Ok(HangulCompatibilityJamo::HangulLetterWi),
            'ㅠ' => Ok(HangulCompatibilityJamo::HangulLetterYu),
            'ㅡ' => Ok(HangulCompatibilityJamo::HangulLetterEu),
            'ㅢ' => Ok(HangulCompatibilityJamo::HangulLetterYi),
            'ㅣ' => Ok(HangulCompatibilityJamo::HangulLetterI),
            'ㅤ' => Ok(HangulCompatibilityJamo::HangulFiller),
            'ㅥ' => Ok(HangulCompatibilityJamo::HangulLetterSsangnieun),
            'ㅦ' => Ok(HangulCompatibilityJamo::HangulLetterNieunDashTikeut),
            'ㅧ' => Ok(HangulCompatibilityJamo::HangulLetterNieunDashSios),
            'ㅨ' => Ok(HangulCompatibilityJamo::HangulLetterNieunDashPansios),
            'ㅩ' => Ok(HangulCompatibilityJamo::HangulLetterRieulDashKiyeokDashSios),
            'ㅪ' => Ok(HangulCompatibilityJamo::HangulLetterRieulDashTikeut),
            'ㅫ' => Ok(HangulCompatibilityJamo::HangulLetterRieulDashPieupDashSios),
            'ㅬ' => Ok(HangulCompatibilityJamo::HangulLetterRieulDashPansios),
            'ㅭ' => Ok(HangulCompatibilityJamo::HangulLetterRieulDashYeorinhieuh),
            'ㅮ' => Ok(HangulCompatibilityJamo::HangulLetterMieumDashPieup),
            'ㅯ' => Ok(HangulCompatibilityJamo::HangulLetterMieumDashSios),
            'ㅰ' => Ok(HangulCompatibilityJamo::HangulLetterMieumDashPansios),
            'ㅱ' => Ok(HangulCompatibilityJamo::HangulLetterKapyeounmieum),
            'ㅲ' => Ok(HangulCompatibilityJamo::HangulLetterPieupDashKiyeok),
            'ㅳ' => Ok(HangulCompatibilityJamo::HangulLetterPieupDashTikeut),
            'ㅴ' => Ok(HangulCompatibilityJamo::HangulLetterPieupDashSiosDashKiyeok),
            'ㅵ' => Ok(HangulCompatibilityJamo::HangulLetterPieupDashSiosDashTikeut),
            'ㅶ' => Ok(HangulCompatibilityJamo::HangulLetterPieupDashCieuc),
            'ㅷ' => Ok(HangulCompatibilityJamo::HangulLetterPieupDashThieuth),
            'ㅸ' => Ok(HangulCompatibilityJamo::HangulLetterKapyeounpieup),
            'ㅹ' => Ok(HangulCompatibilityJamo::HangulLetterKapyeounssangpieup),
            'ㅺ' => Ok(HangulCompatibilityJamo::HangulLetterSiosDashKiyeok),
            'ㅻ' => Ok(HangulCompatibilityJamo::HangulLetterSiosDashNieun),
            'ㅼ' => Ok(HangulCompatibilityJamo::HangulLetterSiosDashTikeut),
            'ㅽ' => Ok(HangulCompatibilityJamo::HangulLetterSiosDashPieup),
            'ㅾ' => Ok(HangulCompatibilityJamo::HangulLetterSiosDashCieuc),
            'ㅿ' => Ok(HangulCompatibilityJamo::HangulLetterPansios),
            'ㆀ' => Ok(HangulCompatibilityJamo::HangulLetterSsangieung),
            'ㆁ' => Ok(HangulCompatibilityJamo::HangulLetterYesieung),
            'ㆂ' => Ok(HangulCompatibilityJamo::HangulLetterYesieungDashSios),
            'ㆃ' => Ok(HangulCompatibilityJamo::HangulLetterYesieungDashPansios),
            'ㆄ' => Ok(HangulCompatibilityJamo::HangulLetterKapyeounphieuph),
            'ㆅ' => Ok(HangulCompatibilityJamo::HangulLetterSsanghieuh),
            'ㆆ' => Ok(HangulCompatibilityJamo::HangulLetterYeorinhieuh),
            'ㆇ' => Ok(HangulCompatibilityJamo::HangulLetterYoDashYa),
            'ㆈ' => Ok(HangulCompatibilityJamo::HangulLetterYoDashYae),
            'ㆉ' => Ok(HangulCompatibilityJamo::HangulLetterYoDashI),
            'ㆊ' => Ok(HangulCompatibilityJamo::HangulLetterYuDashYeo),
            'ㆋ' => Ok(HangulCompatibilityJamo::HangulLetterYuDashYe),
            'ㆌ' => Ok(HangulCompatibilityJamo::HangulLetterYuDashI),
            'ㆍ' => Ok(HangulCompatibilityJamo::HangulLetterAraea),
            'ㆎ' => Ok(HangulCompatibilityJamo::HangulLetterAraeae),
            _ => Err(()),
        }
    }
}

impl Into<u32> for HangulCompatibilityJamo {
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

impl std::convert::TryFrom<u32> for HangulCompatibilityJamo {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for HangulCompatibilityJamo {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl HangulCompatibilityJamo {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        HangulCompatibilityJamo::HangulLetterKiyeok
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("HangulCompatibilityJamo{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
