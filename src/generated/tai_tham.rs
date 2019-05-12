
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
        match self {
            TaiTham::LetterHighKa => 'ᨠ',
            TaiTham::LetterHighKha => 'ᨡ',
            TaiTham::LetterHighKxa => 'ᨢ',
            TaiTham::LetterLowKa => 'ᨣ',
            TaiTham::LetterLowKxa => 'ᨤ',
            TaiTham::LetterLowKha => 'ᨥ',
            TaiTham::LetterNga => 'ᨦ',
            TaiTham::LetterHighCa => 'ᨧ',
            TaiTham::LetterHighCha => 'ᨨ',
            TaiTham::LetterLowCa => 'ᨩ',
            TaiTham::LetterLowSa => 'ᨪ',
            TaiTham::LetterLowCha => 'ᨫ',
            TaiTham::LetterNya => 'ᨬ',
            TaiTham::LetterRata => 'ᨭ',
            TaiTham::LetterHighRatha => 'ᨮ',
            TaiTham::LetterDa => 'ᨯ',
            TaiTham::LetterLowRatha => 'ᨰ',
            TaiTham::LetterRana => 'ᨱ',
            TaiTham::LetterHighTa => 'ᨲ',
            TaiTham::LetterHighTha => 'ᨳ',
            TaiTham::LetterLowTa => 'ᨴ',
            TaiTham::LetterLowTha => 'ᨵ',
            TaiTham::LetterNa => 'ᨶ',
            TaiTham::LetterBa => 'ᨷ',
            TaiTham::LetterHighPa => 'ᨸ',
            TaiTham::LetterHighPha => 'ᨹ',
            TaiTham::LetterHighFa => 'ᨺ',
            TaiTham::LetterLowPa => 'ᨻ',
            TaiTham::LetterLowFa => 'ᨼ',
            TaiTham::LetterLowPha => 'ᨽ',
            TaiTham::LetterMa => 'ᨾ',
            TaiTham::LetterLowYa => 'ᨿ',
            TaiTham::LetterHighYa => 'ᩀ',
            TaiTham::LetterRa => 'ᩁ',
            TaiTham::LetterRue => 'ᩂ',
            TaiTham::LetterLa => 'ᩃ',
            TaiTham::LetterLue => 'ᩄ',
            TaiTham::LetterWa => 'ᩅ',
            TaiTham::LetterHighSha => 'ᩆ',
            TaiTham::LetterHighSsa => 'ᩇ',
            TaiTham::LetterHighSa => 'ᩈ',
            TaiTham::LetterHighHa => 'ᩉ',
            TaiTham::LetterLla => 'ᩊ',
            TaiTham::LetterA => 'ᩋ',
            TaiTham::LetterLowHa => 'ᩌ',
            TaiTham::LetterI => 'ᩍ',
            TaiTham::LetterIi => 'ᩎ',
            TaiTham::LetterU => 'ᩏ',
            TaiTham::LetterUu => 'ᩐ',
            TaiTham::LetterEe => 'ᩑ',
            TaiTham::LetterOo => 'ᩒ',
            TaiTham::LetterLae => 'ᩓ',
            TaiTham::LetterGreatSa => 'ᩔ',
            TaiTham::ConsonantSignMedialRa => 'ᩕ',
            TaiTham::ConsonantSignMedialLa => 'ᩖ',
            TaiTham::ConsonantSignLaTangLai => 'ᩗ',
            TaiTham::SignMaiKangLai => 'ᩘ',
            TaiTham::ConsonantSignFinalNga => 'ᩙ',
            TaiTham::ConsonantSignLowPa => 'ᩚ',
            TaiTham::ConsonantSignHighRathaOrLowPa => 'ᩛ',
            TaiTham::ConsonantSignMa => 'ᩜ',
            TaiTham::ConsonantSignBa => 'ᩝ',
            TaiTham::ConsonantSignSa => 'ᩞ',
            TaiTham::SignSakot => '᩠',
            TaiTham::VowelSignA => 'ᩡ',
            TaiTham::VowelSignMaiSat => 'ᩢ',
            TaiTham::VowelSignAa => 'ᩣ',
            TaiTham::VowelSignTallAa => 'ᩤ',
            TaiTham::VowelSignI => 'ᩥ',
            TaiTham::VowelSignIi => 'ᩦ',
            TaiTham::VowelSignUe => 'ᩧ',
            TaiTham::VowelSignUue => 'ᩨ',
            TaiTham::VowelSignU => 'ᩩ',
            TaiTham::VowelSignUu => 'ᩪ',
            TaiTham::VowelSignO => 'ᩫ',
            TaiTham::VowelSignOaBelow => 'ᩬ',
            TaiTham::VowelSignOy => 'ᩭ',
            TaiTham::VowelSignE => 'ᩮ',
            TaiTham::VowelSignAe => 'ᩯ',
            TaiTham::VowelSignOo => 'ᩰ',
            TaiTham::VowelSignAi => 'ᩱ',
            TaiTham::VowelSignThamAi => 'ᩲ',
            TaiTham::VowelSignOaAbove => 'ᩳ',
            TaiTham::SignMaiKang => 'ᩴ',
            TaiTham::SignToneDash1 => '᩵',
            TaiTham::SignToneDash2 => '᩶',
            TaiTham::SignKhuenToneDash3 => '᩷',
            TaiTham::SignKhuenToneDash4 => '᩸',
            TaiTham::SignKhuenToneDash5 => '᩹',
            TaiTham::SignRaHaam => '᩺',
            TaiTham::SignMaiSam => '᩻',
            TaiTham::SignKhuenDashLueKaran => '᩼',
            TaiTham::CombiningCryptogrammicDot => '᩿',
            TaiTham::HoraDigitZero => '᪀',
            TaiTham::HoraDigitOne => '᪁',
            TaiTham::HoraDigitTwo => '᪂',
            TaiTham::HoraDigitThree => '᪃',
            TaiTham::HoraDigitFour => '᪄',
            TaiTham::HoraDigitFive => '᪅',
            TaiTham::HoraDigitSix => '᪆',
            TaiTham::HoraDigitSeven => '᪇',
            TaiTham::HoraDigitEight => '᪈',
            TaiTham::HoraDigitNine => '᪉',
            TaiTham::ThamDigitZero => '᪐',
            TaiTham::ThamDigitOne => '᪑',
            TaiTham::ThamDigitTwo => '᪒',
            TaiTham::ThamDigitThree => '᪓',
            TaiTham::ThamDigitFour => '᪔',
            TaiTham::ThamDigitFive => '᪕',
            TaiTham::ThamDigitSix => '᪖',
            TaiTham::ThamDigitSeven => '᪗',
            TaiTham::ThamDigitEight => '᪘',
            TaiTham::ThamDigitNine => '᪙',
            TaiTham::SignWiang => '᪠',
            TaiTham::SignWiangwaak => '᪡',
            TaiTham::SignSawan => '᪢',
            TaiTham::SignKeow => '᪣',
            TaiTham::SignHoy => '᪤',
            TaiTham::SignDokmai => '᪥',
            TaiTham::SignReversedRotatedRana => '᪦',
            TaiTham::SignMaiYamok => 'ᪧ',
            TaiTham::SignKaan => '᪨',
            TaiTham::SignKaankuu => '᪩',
            TaiTham::SignSatkaan => '᪪',
            TaiTham::SignSatkaankuu => '᪫',
            TaiTham::SignHang => '᪬',
            TaiTham::SignCaang => '᪭',
        }
    }
}

impl std::convert::TryFrom<char> for TaiTham {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ᨠ' => Ok(TaiTham::LetterHighKa),
            'ᨡ' => Ok(TaiTham::LetterHighKha),
            'ᨢ' => Ok(TaiTham::LetterHighKxa),
            'ᨣ' => Ok(TaiTham::LetterLowKa),
            'ᨤ' => Ok(TaiTham::LetterLowKxa),
            'ᨥ' => Ok(TaiTham::LetterLowKha),
            'ᨦ' => Ok(TaiTham::LetterNga),
            'ᨧ' => Ok(TaiTham::LetterHighCa),
            'ᨨ' => Ok(TaiTham::LetterHighCha),
            'ᨩ' => Ok(TaiTham::LetterLowCa),
            'ᨪ' => Ok(TaiTham::LetterLowSa),
            'ᨫ' => Ok(TaiTham::LetterLowCha),
            'ᨬ' => Ok(TaiTham::LetterNya),
            'ᨭ' => Ok(TaiTham::LetterRata),
            'ᨮ' => Ok(TaiTham::LetterHighRatha),
            'ᨯ' => Ok(TaiTham::LetterDa),
            'ᨰ' => Ok(TaiTham::LetterLowRatha),
            'ᨱ' => Ok(TaiTham::LetterRana),
            'ᨲ' => Ok(TaiTham::LetterHighTa),
            'ᨳ' => Ok(TaiTham::LetterHighTha),
            'ᨴ' => Ok(TaiTham::LetterLowTa),
            'ᨵ' => Ok(TaiTham::LetterLowTha),
            'ᨶ' => Ok(TaiTham::LetterNa),
            'ᨷ' => Ok(TaiTham::LetterBa),
            'ᨸ' => Ok(TaiTham::LetterHighPa),
            'ᨹ' => Ok(TaiTham::LetterHighPha),
            'ᨺ' => Ok(TaiTham::LetterHighFa),
            'ᨻ' => Ok(TaiTham::LetterLowPa),
            'ᨼ' => Ok(TaiTham::LetterLowFa),
            'ᨽ' => Ok(TaiTham::LetterLowPha),
            'ᨾ' => Ok(TaiTham::LetterMa),
            'ᨿ' => Ok(TaiTham::LetterLowYa),
            'ᩀ' => Ok(TaiTham::LetterHighYa),
            'ᩁ' => Ok(TaiTham::LetterRa),
            'ᩂ' => Ok(TaiTham::LetterRue),
            'ᩃ' => Ok(TaiTham::LetterLa),
            'ᩄ' => Ok(TaiTham::LetterLue),
            'ᩅ' => Ok(TaiTham::LetterWa),
            'ᩆ' => Ok(TaiTham::LetterHighSha),
            'ᩇ' => Ok(TaiTham::LetterHighSsa),
            'ᩈ' => Ok(TaiTham::LetterHighSa),
            'ᩉ' => Ok(TaiTham::LetterHighHa),
            'ᩊ' => Ok(TaiTham::LetterLla),
            'ᩋ' => Ok(TaiTham::LetterA),
            'ᩌ' => Ok(TaiTham::LetterLowHa),
            'ᩍ' => Ok(TaiTham::LetterI),
            'ᩎ' => Ok(TaiTham::LetterIi),
            'ᩏ' => Ok(TaiTham::LetterU),
            'ᩐ' => Ok(TaiTham::LetterUu),
            'ᩑ' => Ok(TaiTham::LetterEe),
            'ᩒ' => Ok(TaiTham::LetterOo),
            'ᩓ' => Ok(TaiTham::LetterLae),
            'ᩔ' => Ok(TaiTham::LetterGreatSa),
            'ᩕ' => Ok(TaiTham::ConsonantSignMedialRa),
            'ᩖ' => Ok(TaiTham::ConsonantSignMedialLa),
            'ᩗ' => Ok(TaiTham::ConsonantSignLaTangLai),
            'ᩘ' => Ok(TaiTham::SignMaiKangLai),
            'ᩙ' => Ok(TaiTham::ConsonantSignFinalNga),
            'ᩚ' => Ok(TaiTham::ConsonantSignLowPa),
            'ᩛ' => Ok(TaiTham::ConsonantSignHighRathaOrLowPa),
            'ᩜ' => Ok(TaiTham::ConsonantSignMa),
            'ᩝ' => Ok(TaiTham::ConsonantSignBa),
            'ᩞ' => Ok(TaiTham::ConsonantSignSa),
            '᩠' => Ok(TaiTham::SignSakot),
            'ᩡ' => Ok(TaiTham::VowelSignA),
            'ᩢ' => Ok(TaiTham::VowelSignMaiSat),
            'ᩣ' => Ok(TaiTham::VowelSignAa),
            'ᩤ' => Ok(TaiTham::VowelSignTallAa),
            'ᩥ' => Ok(TaiTham::VowelSignI),
            'ᩦ' => Ok(TaiTham::VowelSignIi),
            'ᩧ' => Ok(TaiTham::VowelSignUe),
            'ᩨ' => Ok(TaiTham::VowelSignUue),
            'ᩩ' => Ok(TaiTham::VowelSignU),
            'ᩪ' => Ok(TaiTham::VowelSignUu),
            'ᩫ' => Ok(TaiTham::VowelSignO),
            'ᩬ' => Ok(TaiTham::VowelSignOaBelow),
            'ᩭ' => Ok(TaiTham::VowelSignOy),
            'ᩮ' => Ok(TaiTham::VowelSignE),
            'ᩯ' => Ok(TaiTham::VowelSignAe),
            'ᩰ' => Ok(TaiTham::VowelSignOo),
            'ᩱ' => Ok(TaiTham::VowelSignAi),
            'ᩲ' => Ok(TaiTham::VowelSignThamAi),
            'ᩳ' => Ok(TaiTham::VowelSignOaAbove),
            'ᩴ' => Ok(TaiTham::SignMaiKang),
            '᩵' => Ok(TaiTham::SignToneDash1),
            '᩶' => Ok(TaiTham::SignToneDash2),
            '᩷' => Ok(TaiTham::SignKhuenToneDash3),
            '᩸' => Ok(TaiTham::SignKhuenToneDash4),
            '᩹' => Ok(TaiTham::SignKhuenToneDash5),
            '᩺' => Ok(TaiTham::SignRaHaam),
            '᩻' => Ok(TaiTham::SignMaiSam),
            '᩼' => Ok(TaiTham::SignKhuenDashLueKaran),
            '᩿' => Ok(TaiTham::CombiningCryptogrammicDot),
            '᪀' => Ok(TaiTham::HoraDigitZero),
            '᪁' => Ok(TaiTham::HoraDigitOne),
            '᪂' => Ok(TaiTham::HoraDigitTwo),
            '᪃' => Ok(TaiTham::HoraDigitThree),
            '᪄' => Ok(TaiTham::HoraDigitFour),
            '᪅' => Ok(TaiTham::HoraDigitFive),
            '᪆' => Ok(TaiTham::HoraDigitSix),
            '᪇' => Ok(TaiTham::HoraDigitSeven),
            '᪈' => Ok(TaiTham::HoraDigitEight),
            '᪉' => Ok(TaiTham::HoraDigitNine),
            '᪐' => Ok(TaiTham::ThamDigitZero),
            '᪑' => Ok(TaiTham::ThamDigitOne),
            '᪒' => Ok(TaiTham::ThamDigitTwo),
            '᪓' => Ok(TaiTham::ThamDigitThree),
            '᪔' => Ok(TaiTham::ThamDigitFour),
            '᪕' => Ok(TaiTham::ThamDigitFive),
            '᪖' => Ok(TaiTham::ThamDigitSix),
            '᪗' => Ok(TaiTham::ThamDigitSeven),
            '᪘' => Ok(TaiTham::ThamDigitEight),
            '᪙' => Ok(TaiTham::ThamDigitNine),
            '᪠' => Ok(TaiTham::SignWiang),
            '᪡' => Ok(TaiTham::SignWiangwaak),
            '᪢' => Ok(TaiTham::SignSawan),
            '᪣' => Ok(TaiTham::SignKeow),
            '᪤' => Ok(TaiTham::SignHoy),
            '᪥' => Ok(TaiTham::SignDokmai),
            '᪦' => Ok(TaiTham::SignReversedRotatedRana),
            'ᪧ' => Ok(TaiTham::SignMaiYamok),
            '᪨' => Ok(TaiTham::SignKaan),
            '᪩' => Ok(TaiTham::SignKaankuu),
            '᪪' => Ok(TaiTham::SignSatkaan),
            '᪫' => Ok(TaiTham::SignSatkaankuu),
            '᪬' => Ok(TaiTham::SignHang),
            '᪭' => Ok(TaiTham::SignCaang),
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

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("TaiTham{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
