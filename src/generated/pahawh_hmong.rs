
/// An enum to represent all characters in the PahawhHmong block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum PahawhHmong {
    /// \u{16b00}: '𖬀'
    VowelKeeb,
    /// \u{16b01}: '𖬁'
    VowelKeev,
    /// \u{16b02}: '𖬂'
    VowelKib,
    /// \u{16b03}: '𖬃'
    VowelKiv,
    /// \u{16b04}: '𖬄'
    VowelKaub,
    /// \u{16b05}: '𖬅'
    VowelKauv,
    /// \u{16b06}: '𖬆'
    VowelKub,
    /// \u{16b07}: '𖬇'
    VowelKuv,
    /// \u{16b08}: '𖬈'
    VowelKeb,
    /// \u{16b09}: '𖬉'
    VowelKev,
    /// \u{16b0a}: '𖬊'
    VowelKaib,
    /// \u{16b0b}: '𖬋'
    VowelKaiv,
    /// \u{16b0c}: '𖬌'
    VowelKoob,
    /// \u{16b0d}: '𖬍'
    VowelKoov,
    /// \u{16b0e}: '𖬎'
    VowelKawb,
    /// \u{16b0f}: '𖬏'
    VowelKawv,
    /// \u{16b10}: '𖬐'
    VowelKuab,
    /// \u{16b11}: '𖬑'
    VowelKuav,
    /// \u{16b12}: '𖬒'
    VowelKob,
    /// \u{16b13}: '𖬓'
    VowelKov,
    /// \u{16b14}: '𖬔'
    VowelKiab,
    /// \u{16b15}: '𖬕'
    VowelKiav,
    /// \u{16b16}: '𖬖'
    VowelKab,
    /// \u{16b17}: '𖬗'
    VowelKav,
    /// \u{16b18}: '𖬘'
    VowelKwb,
    /// \u{16b19}: '𖬙'
    VowelKwv,
    /// \u{16b1a}: '𖬚'
    VowelKaab,
    /// \u{16b1b}: '𖬛'
    VowelKaav,
    /// \u{16b1c}: '𖬜'
    ConsonantVau,
    /// \u{16b1d}: '𖬝'
    ConsonantNtsau,
    /// \u{16b1e}: '𖬞'
    ConsonantLau,
    /// \u{16b1f}: '𖬟'
    ConsonantHau,
    /// \u{16b20}: '𖬠'
    ConsonantNlau,
    /// \u{16b21}: '𖬡'
    ConsonantRau,
    /// \u{16b22}: '𖬢'
    ConsonantNkau,
    /// \u{16b23}: '𖬣'
    ConsonantQhau,
    /// \u{16b24}: '𖬤'
    ConsonantYau,
    /// \u{16b25}: '𖬥'
    ConsonantHlau,
    /// \u{16b26}: '𖬦'
    ConsonantMau,
    /// \u{16b27}: '𖬧'
    ConsonantChau,
    /// \u{16b28}: '𖬨'
    ConsonantNchau,
    /// \u{16b29}: '𖬩'
    ConsonantHnau,
    /// \u{16b2a}: '𖬪'
    ConsonantPlhau,
    /// \u{16b2b}: '𖬫'
    ConsonantNthau,
    /// \u{16b2c}: '𖬬'
    ConsonantNau,
    /// \u{16b2d}: '𖬭'
    ConsonantAu,
    /// \u{16b2e}: '𖬮'
    ConsonantXau,
    /// \u{16b2f}: '𖬯'
    ConsonantCau,
    /// \u{16b30}: '𖬰'
    MarkCimTub,
    /// \u{16b31}: '𖬱'
    MarkCimSo,
    /// \u{16b32}: '𖬲'
    MarkCimKes,
    /// \u{16b33}: '𖬳'
    MarkCimKhav,
    /// \u{16b34}: '𖬴'
    MarkCimSuam,
    /// \u{16b35}: '𖬵'
    MarkCimHom,
    /// \u{16b36}: '𖬶'
    MarkCimTaum,
    /// \u{16b37}: '𖬷'
    SignVosThom,
    /// \u{16b38}: '𖬸'
    SignVosTshabCeeb,
    /// \u{16b39}: '𖬹'
    SignCimCheem,
    /// \u{16b3a}: '𖬺'
    SignVosThiab,
    /// \u{16b3b}: '𖬻'
    SignVosFeem,
    /// \u{16b3c}: '𖬼'
    SignXyeemNtxiv,
    /// \u{16b3d}: '𖬽'
    SignXyeemRho,
    /// \u{16b3e}: '𖬾'
    SignXyeemTov,
    /// \u{16b3f}: '𖬿'
    SignXyeemFaib,
    /// \u{16b40}: '𖭀'
    SignVosSeev,
    /// \u{16b41}: '𖭁'
    SignMeejSuab,
    /// \u{16b42}: '𖭂'
    SignVosNrua,
    /// \u{16b43}: '𖭃'
    SignIbYam,
    /// \u{16b44}: '𖭄'
    SignXaus,
    /// \u{16b45}: '𖭅'
    SignCimTsovRog,
    /// \u{16b50}: '𖭐'
    DigitZero,
    /// \u{16b51}: '𖭑'
    DigitOne,
    /// \u{16b52}: '𖭒'
    DigitTwo,
    /// \u{16b53}: '𖭓'
    DigitThree,
    /// \u{16b54}: '𖭔'
    DigitFour,
    /// \u{16b55}: '𖭕'
    DigitFive,
    /// \u{16b56}: '𖭖'
    DigitSix,
    /// \u{16b57}: '𖭗'
    DigitSeven,
    /// \u{16b58}: '𖭘'
    DigitEight,
    /// \u{16b59}: '𖭙'
    DigitNine,
    /// \u{16b5b}: '𖭛'
    NumberTens,
    /// \u{16b5c}: '𖭜'
    NumberHundreds,
    /// \u{16b5d}: '𖭝'
    NumberTenThousands,
    /// \u{16b5e}: '𖭞'
    NumberMillions,
    /// \u{16b5f}: '𖭟'
    NumberHundredMillions,
    /// \u{16b60}: '𖭠'
    NumberTenBillions,
    /// \u{16b61}: '𖭡'
    NumberTrillions,
    /// \u{16b63}: '𖭣'
    SignVosLub,
    /// \u{16b64}: '𖭤'
    SignXyoo,
    /// \u{16b65}: '𖭥'
    SignHli,
    /// \u{16b66}: '𖭦'
    SignThirdDashStageHli,
    /// \u{16b67}: '𖭧'
    SignZwjThaj,
    /// \u{16b68}: '𖭨'
    SignHnub,
    /// \u{16b69}: '𖭩'
    SignNqig,
    /// \u{16b6a}: '𖭪'
    SignXiab,
    /// \u{16b6b}: '𖭫'
    SignNtuj,
    /// \u{16b6c}: '𖭬'
    SignAv,
    /// \u{16b6d}: '𖭭'
    SignTxheejCeev,
    /// \u{16b6e}: '𖭮'
    SignMeejTseeb,
    /// \u{16b6f}: '𖭯'
    SignTau,
    /// \u{16b70}: '𖭰'
    SignLos,
    /// \u{16b71}: '𖭱'
    SignMus,
    /// \u{16b72}: '𖭲'
    SignCimHaisLusNtogNtog,
    /// \u{16b73}: '𖭳'
    SignCimCuamTshooj,
    /// \u{16b74}: '𖭴'
    SignCimTxwv,
    /// \u{16b75}: '𖭵'
    SignCimTxwvChwv,
    /// \u{16b76}: '𖭶'
    SignCimPubDawb,
    /// \u{16b77}: '𖭷'
    SignCimNresTos,
    /// \u{16b7d}: '𖭽'
    ClanSignTsheej,
    /// \u{16b7e}: '𖭾'
    ClanSignYeeg,
    /// \u{16b7f}: '𖭿'
    ClanSignLis,
    /// \u{16b80}: '𖮀'
    ClanSignLauj,
    /// \u{16b81}: '𖮁'
    ClanSignXyooj,
    /// \u{16b82}: '𖮂'
    ClanSignKoo,
    /// \u{16b83}: '𖮃'
    ClanSignHawj,
    /// \u{16b84}: '𖮄'
    ClanSignMuas,
    /// \u{16b85}: '𖮅'
    ClanSignThoj,
    /// \u{16b86}: '𖮆'
    ClanSignTsab,
    /// \u{16b87}: '𖮇'
    ClanSignPhab,
    /// \u{16b88}: '𖮈'
    ClanSignKhab,
    /// \u{16b89}: '𖮉'
    ClanSignHam,
    /// \u{16b8a}: '𖮊'
    ClanSignVaj,
    /// \u{16b8b}: '𖮋'
    ClanSignFaj,
    /// \u{16b8c}: '𖮌'
    ClanSignYaj,
    /// \u{16b8d}: '𖮍'
    ClanSignTswb,
    /// \u{16b8e}: '𖮎'
    ClanSignKwm,
}

impl Into<char> for PahawhHmong {
    fn into(self) -> char {
        match self {
            PahawhHmong::VowelKeeb => '𖬀',
            PahawhHmong::VowelKeev => '𖬁',
            PahawhHmong::VowelKib => '𖬂',
            PahawhHmong::VowelKiv => '𖬃',
            PahawhHmong::VowelKaub => '𖬄',
            PahawhHmong::VowelKauv => '𖬅',
            PahawhHmong::VowelKub => '𖬆',
            PahawhHmong::VowelKuv => '𖬇',
            PahawhHmong::VowelKeb => '𖬈',
            PahawhHmong::VowelKev => '𖬉',
            PahawhHmong::VowelKaib => '𖬊',
            PahawhHmong::VowelKaiv => '𖬋',
            PahawhHmong::VowelKoob => '𖬌',
            PahawhHmong::VowelKoov => '𖬍',
            PahawhHmong::VowelKawb => '𖬎',
            PahawhHmong::VowelKawv => '𖬏',
            PahawhHmong::VowelKuab => '𖬐',
            PahawhHmong::VowelKuav => '𖬑',
            PahawhHmong::VowelKob => '𖬒',
            PahawhHmong::VowelKov => '𖬓',
            PahawhHmong::VowelKiab => '𖬔',
            PahawhHmong::VowelKiav => '𖬕',
            PahawhHmong::VowelKab => '𖬖',
            PahawhHmong::VowelKav => '𖬗',
            PahawhHmong::VowelKwb => '𖬘',
            PahawhHmong::VowelKwv => '𖬙',
            PahawhHmong::VowelKaab => '𖬚',
            PahawhHmong::VowelKaav => '𖬛',
            PahawhHmong::ConsonantVau => '𖬜',
            PahawhHmong::ConsonantNtsau => '𖬝',
            PahawhHmong::ConsonantLau => '𖬞',
            PahawhHmong::ConsonantHau => '𖬟',
            PahawhHmong::ConsonantNlau => '𖬠',
            PahawhHmong::ConsonantRau => '𖬡',
            PahawhHmong::ConsonantNkau => '𖬢',
            PahawhHmong::ConsonantQhau => '𖬣',
            PahawhHmong::ConsonantYau => '𖬤',
            PahawhHmong::ConsonantHlau => '𖬥',
            PahawhHmong::ConsonantMau => '𖬦',
            PahawhHmong::ConsonantChau => '𖬧',
            PahawhHmong::ConsonantNchau => '𖬨',
            PahawhHmong::ConsonantHnau => '𖬩',
            PahawhHmong::ConsonantPlhau => '𖬪',
            PahawhHmong::ConsonantNthau => '𖬫',
            PahawhHmong::ConsonantNau => '𖬬',
            PahawhHmong::ConsonantAu => '𖬭',
            PahawhHmong::ConsonantXau => '𖬮',
            PahawhHmong::ConsonantCau => '𖬯',
            PahawhHmong::MarkCimTub => '𖬰',
            PahawhHmong::MarkCimSo => '𖬱',
            PahawhHmong::MarkCimKes => '𖬲',
            PahawhHmong::MarkCimKhav => '𖬳',
            PahawhHmong::MarkCimSuam => '𖬴',
            PahawhHmong::MarkCimHom => '𖬵',
            PahawhHmong::MarkCimTaum => '𖬶',
            PahawhHmong::SignVosThom => '𖬷',
            PahawhHmong::SignVosTshabCeeb => '𖬸',
            PahawhHmong::SignCimCheem => '𖬹',
            PahawhHmong::SignVosThiab => '𖬺',
            PahawhHmong::SignVosFeem => '𖬻',
            PahawhHmong::SignXyeemNtxiv => '𖬼',
            PahawhHmong::SignXyeemRho => '𖬽',
            PahawhHmong::SignXyeemTov => '𖬾',
            PahawhHmong::SignXyeemFaib => '𖬿',
            PahawhHmong::SignVosSeev => '𖭀',
            PahawhHmong::SignMeejSuab => '𖭁',
            PahawhHmong::SignVosNrua => '𖭂',
            PahawhHmong::SignIbYam => '𖭃',
            PahawhHmong::SignXaus => '𖭄',
            PahawhHmong::SignCimTsovRog => '𖭅',
            PahawhHmong::DigitZero => '𖭐',
            PahawhHmong::DigitOne => '𖭑',
            PahawhHmong::DigitTwo => '𖭒',
            PahawhHmong::DigitThree => '𖭓',
            PahawhHmong::DigitFour => '𖭔',
            PahawhHmong::DigitFive => '𖭕',
            PahawhHmong::DigitSix => '𖭖',
            PahawhHmong::DigitSeven => '𖭗',
            PahawhHmong::DigitEight => '𖭘',
            PahawhHmong::DigitNine => '𖭙',
            PahawhHmong::NumberTens => '𖭛',
            PahawhHmong::NumberHundreds => '𖭜',
            PahawhHmong::NumberTenThousands => '𖭝',
            PahawhHmong::NumberMillions => '𖭞',
            PahawhHmong::NumberHundredMillions => '𖭟',
            PahawhHmong::NumberTenBillions => '𖭠',
            PahawhHmong::NumberTrillions => '𖭡',
            PahawhHmong::SignVosLub => '𖭣',
            PahawhHmong::SignXyoo => '𖭤',
            PahawhHmong::SignHli => '𖭥',
            PahawhHmong::SignThirdDashStageHli => '𖭦',
            PahawhHmong::SignZwjThaj => '𖭧',
            PahawhHmong::SignHnub => '𖭨',
            PahawhHmong::SignNqig => '𖭩',
            PahawhHmong::SignXiab => '𖭪',
            PahawhHmong::SignNtuj => '𖭫',
            PahawhHmong::SignAv => '𖭬',
            PahawhHmong::SignTxheejCeev => '𖭭',
            PahawhHmong::SignMeejTseeb => '𖭮',
            PahawhHmong::SignTau => '𖭯',
            PahawhHmong::SignLos => '𖭰',
            PahawhHmong::SignMus => '𖭱',
            PahawhHmong::SignCimHaisLusNtogNtog => '𖭲',
            PahawhHmong::SignCimCuamTshooj => '𖭳',
            PahawhHmong::SignCimTxwv => '𖭴',
            PahawhHmong::SignCimTxwvChwv => '𖭵',
            PahawhHmong::SignCimPubDawb => '𖭶',
            PahawhHmong::SignCimNresTos => '𖭷',
            PahawhHmong::ClanSignTsheej => '𖭽',
            PahawhHmong::ClanSignYeeg => '𖭾',
            PahawhHmong::ClanSignLis => '𖭿',
            PahawhHmong::ClanSignLauj => '𖮀',
            PahawhHmong::ClanSignXyooj => '𖮁',
            PahawhHmong::ClanSignKoo => '𖮂',
            PahawhHmong::ClanSignHawj => '𖮃',
            PahawhHmong::ClanSignMuas => '𖮄',
            PahawhHmong::ClanSignThoj => '𖮅',
            PahawhHmong::ClanSignTsab => '𖮆',
            PahawhHmong::ClanSignPhab => '𖮇',
            PahawhHmong::ClanSignKhab => '𖮈',
            PahawhHmong::ClanSignHam => '𖮉',
            PahawhHmong::ClanSignVaj => '𖮊',
            PahawhHmong::ClanSignFaj => '𖮋',
            PahawhHmong::ClanSignYaj => '𖮌',
            PahawhHmong::ClanSignTswb => '𖮍',
            PahawhHmong::ClanSignKwm => '𖮎',
        }
    }
}

impl std::convert::TryFrom<char> for PahawhHmong {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𖬀' => Ok(PahawhHmong::VowelKeeb),
            '𖬁' => Ok(PahawhHmong::VowelKeev),
            '𖬂' => Ok(PahawhHmong::VowelKib),
            '𖬃' => Ok(PahawhHmong::VowelKiv),
            '𖬄' => Ok(PahawhHmong::VowelKaub),
            '𖬅' => Ok(PahawhHmong::VowelKauv),
            '𖬆' => Ok(PahawhHmong::VowelKub),
            '𖬇' => Ok(PahawhHmong::VowelKuv),
            '𖬈' => Ok(PahawhHmong::VowelKeb),
            '𖬉' => Ok(PahawhHmong::VowelKev),
            '𖬊' => Ok(PahawhHmong::VowelKaib),
            '𖬋' => Ok(PahawhHmong::VowelKaiv),
            '𖬌' => Ok(PahawhHmong::VowelKoob),
            '𖬍' => Ok(PahawhHmong::VowelKoov),
            '𖬎' => Ok(PahawhHmong::VowelKawb),
            '𖬏' => Ok(PahawhHmong::VowelKawv),
            '𖬐' => Ok(PahawhHmong::VowelKuab),
            '𖬑' => Ok(PahawhHmong::VowelKuav),
            '𖬒' => Ok(PahawhHmong::VowelKob),
            '𖬓' => Ok(PahawhHmong::VowelKov),
            '𖬔' => Ok(PahawhHmong::VowelKiab),
            '𖬕' => Ok(PahawhHmong::VowelKiav),
            '𖬖' => Ok(PahawhHmong::VowelKab),
            '𖬗' => Ok(PahawhHmong::VowelKav),
            '𖬘' => Ok(PahawhHmong::VowelKwb),
            '𖬙' => Ok(PahawhHmong::VowelKwv),
            '𖬚' => Ok(PahawhHmong::VowelKaab),
            '𖬛' => Ok(PahawhHmong::VowelKaav),
            '𖬜' => Ok(PahawhHmong::ConsonantVau),
            '𖬝' => Ok(PahawhHmong::ConsonantNtsau),
            '𖬞' => Ok(PahawhHmong::ConsonantLau),
            '𖬟' => Ok(PahawhHmong::ConsonantHau),
            '𖬠' => Ok(PahawhHmong::ConsonantNlau),
            '𖬡' => Ok(PahawhHmong::ConsonantRau),
            '𖬢' => Ok(PahawhHmong::ConsonantNkau),
            '𖬣' => Ok(PahawhHmong::ConsonantQhau),
            '𖬤' => Ok(PahawhHmong::ConsonantYau),
            '𖬥' => Ok(PahawhHmong::ConsonantHlau),
            '𖬦' => Ok(PahawhHmong::ConsonantMau),
            '𖬧' => Ok(PahawhHmong::ConsonantChau),
            '𖬨' => Ok(PahawhHmong::ConsonantNchau),
            '𖬩' => Ok(PahawhHmong::ConsonantHnau),
            '𖬪' => Ok(PahawhHmong::ConsonantPlhau),
            '𖬫' => Ok(PahawhHmong::ConsonantNthau),
            '𖬬' => Ok(PahawhHmong::ConsonantNau),
            '𖬭' => Ok(PahawhHmong::ConsonantAu),
            '𖬮' => Ok(PahawhHmong::ConsonantXau),
            '𖬯' => Ok(PahawhHmong::ConsonantCau),
            '𖬰' => Ok(PahawhHmong::MarkCimTub),
            '𖬱' => Ok(PahawhHmong::MarkCimSo),
            '𖬲' => Ok(PahawhHmong::MarkCimKes),
            '𖬳' => Ok(PahawhHmong::MarkCimKhav),
            '𖬴' => Ok(PahawhHmong::MarkCimSuam),
            '𖬵' => Ok(PahawhHmong::MarkCimHom),
            '𖬶' => Ok(PahawhHmong::MarkCimTaum),
            '𖬷' => Ok(PahawhHmong::SignVosThom),
            '𖬸' => Ok(PahawhHmong::SignVosTshabCeeb),
            '𖬹' => Ok(PahawhHmong::SignCimCheem),
            '𖬺' => Ok(PahawhHmong::SignVosThiab),
            '𖬻' => Ok(PahawhHmong::SignVosFeem),
            '𖬼' => Ok(PahawhHmong::SignXyeemNtxiv),
            '𖬽' => Ok(PahawhHmong::SignXyeemRho),
            '𖬾' => Ok(PahawhHmong::SignXyeemTov),
            '𖬿' => Ok(PahawhHmong::SignXyeemFaib),
            '𖭀' => Ok(PahawhHmong::SignVosSeev),
            '𖭁' => Ok(PahawhHmong::SignMeejSuab),
            '𖭂' => Ok(PahawhHmong::SignVosNrua),
            '𖭃' => Ok(PahawhHmong::SignIbYam),
            '𖭄' => Ok(PahawhHmong::SignXaus),
            '𖭅' => Ok(PahawhHmong::SignCimTsovRog),
            '𖭐' => Ok(PahawhHmong::DigitZero),
            '𖭑' => Ok(PahawhHmong::DigitOne),
            '𖭒' => Ok(PahawhHmong::DigitTwo),
            '𖭓' => Ok(PahawhHmong::DigitThree),
            '𖭔' => Ok(PahawhHmong::DigitFour),
            '𖭕' => Ok(PahawhHmong::DigitFive),
            '𖭖' => Ok(PahawhHmong::DigitSix),
            '𖭗' => Ok(PahawhHmong::DigitSeven),
            '𖭘' => Ok(PahawhHmong::DigitEight),
            '𖭙' => Ok(PahawhHmong::DigitNine),
            '𖭛' => Ok(PahawhHmong::NumberTens),
            '𖭜' => Ok(PahawhHmong::NumberHundreds),
            '𖭝' => Ok(PahawhHmong::NumberTenThousands),
            '𖭞' => Ok(PahawhHmong::NumberMillions),
            '𖭟' => Ok(PahawhHmong::NumberHundredMillions),
            '𖭠' => Ok(PahawhHmong::NumberTenBillions),
            '𖭡' => Ok(PahawhHmong::NumberTrillions),
            '𖭣' => Ok(PahawhHmong::SignVosLub),
            '𖭤' => Ok(PahawhHmong::SignXyoo),
            '𖭥' => Ok(PahawhHmong::SignHli),
            '𖭦' => Ok(PahawhHmong::SignThirdDashStageHli),
            '𖭧' => Ok(PahawhHmong::SignZwjThaj),
            '𖭨' => Ok(PahawhHmong::SignHnub),
            '𖭩' => Ok(PahawhHmong::SignNqig),
            '𖭪' => Ok(PahawhHmong::SignXiab),
            '𖭫' => Ok(PahawhHmong::SignNtuj),
            '𖭬' => Ok(PahawhHmong::SignAv),
            '𖭭' => Ok(PahawhHmong::SignTxheejCeev),
            '𖭮' => Ok(PahawhHmong::SignMeejTseeb),
            '𖭯' => Ok(PahawhHmong::SignTau),
            '𖭰' => Ok(PahawhHmong::SignLos),
            '𖭱' => Ok(PahawhHmong::SignMus),
            '𖭲' => Ok(PahawhHmong::SignCimHaisLusNtogNtog),
            '𖭳' => Ok(PahawhHmong::SignCimCuamTshooj),
            '𖭴' => Ok(PahawhHmong::SignCimTxwv),
            '𖭵' => Ok(PahawhHmong::SignCimTxwvChwv),
            '𖭶' => Ok(PahawhHmong::SignCimPubDawb),
            '𖭷' => Ok(PahawhHmong::SignCimNresTos),
            '𖭽' => Ok(PahawhHmong::ClanSignTsheej),
            '𖭾' => Ok(PahawhHmong::ClanSignYeeg),
            '𖭿' => Ok(PahawhHmong::ClanSignLis),
            '𖮀' => Ok(PahawhHmong::ClanSignLauj),
            '𖮁' => Ok(PahawhHmong::ClanSignXyooj),
            '𖮂' => Ok(PahawhHmong::ClanSignKoo),
            '𖮃' => Ok(PahawhHmong::ClanSignHawj),
            '𖮄' => Ok(PahawhHmong::ClanSignMuas),
            '𖮅' => Ok(PahawhHmong::ClanSignThoj),
            '𖮆' => Ok(PahawhHmong::ClanSignTsab),
            '𖮇' => Ok(PahawhHmong::ClanSignPhab),
            '𖮈' => Ok(PahawhHmong::ClanSignKhab),
            '𖮉' => Ok(PahawhHmong::ClanSignHam),
            '𖮊' => Ok(PahawhHmong::ClanSignVaj),
            '𖮋' => Ok(PahawhHmong::ClanSignFaj),
            '𖮌' => Ok(PahawhHmong::ClanSignYaj),
            '𖮍' => Ok(PahawhHmong::ClanSignTswb),
            '𖮎' => Ok(PahawhHmong::ClanSignKwm),
            _ => Err(()),
        }
    }
}

impl Into<u32> for PahawhHmong {
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

impl std::convert::TryFrom<u32> for PahawhHmong {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for PahawhHmong {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl PahawhHmong {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        PahawhHmong::VowelKeeb
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            PahawhHmong::VowelKeeb => "pahawh hmong vowel keeb",
            PahawhHmong::VowelKeev => "pahawh hmong vowel keev",
            PahawhHmong::VowelKib => "pahawh hmong vowel kib",
            PahawhHmong::VowelKiv => "pahawh hmong vowel kiv",
            PahawhHmong::VowelKaub => "pahawh hmong vowel kaub",
            PahawhHmong::VowelKauv => "pahawh hmong vowel kauv",
            PahawhHmong::VowelKub => "pahawh hmong vowel kub",
            PahawhHmong::VowelKuv => "pahawh hmong vowel kuv",
            PahawhHmong::VowelKeb => "pahawh hmong vowel keb",
            PahawhHmong::VowelKev => "pahawh hmong vowel kev",
            PahawhHmong::VowelKaib => "pahawh hmong vowel kaib",
            PahawhHmong::VowelKaiv => "pahawh hmong vowel kaiv",
            PahawhHmong::VowelKoob => "pahawh hmong vowel koob",
            PahawhHmong::VowelKoov => "pahawh hmong vowel koov",
            PahawhHmong::VowelKawb => "pahawh hmong vowel kawb",
            PahawhHmong::VowelKawv => "pahawh hmong vowel kawv",
            PahawhHmong::VowelKuab => "pahawh hmong vowel kuab",
            PahawhHmong::VowelKuav => "pahawh hmong vowel kuav",
            PahawhHmong::VowelKob => "pahawh hmong vowel kob",
            PahawhHmong::VowelKov => "pahawh hmong vowel kov",
            PahawhHmong::VowelKiab => "pahawh hmong vowel kiab",
            PahawhHmong::VowelKiav => "pahawh hmong vowel kiav",
            PahawhHmong::VowelKab => "pahawh hmong vowel kab",
            PahawhHmong::VowelKav => "pahawh hmong vowel kav",
            PahawhHmong::VowelKwb => "pahawh hmong vowel kwb",
            PahawhHmong::VowelKwv => "pahawh hmong vowel kwv",
            PahawhHmong::VowelKaab => "pahawh hmong vowel kaab",
            PahawhHmong::VowelKaav => "pahawh hmong vowel kaav",
            PahawhHmong::ConsonantVau => "pahawh hmong consonant vau",
            PahawhHmong::ConsonantNtsau => "pahawh hmong consonant ntsau",
            PahawhHmong::ConsonantLau => "pahawh hmong consonant lau",
            PahawhHmong::ConsonantHau => "pahawh hmong consonant hau",
            PahawhHmong::ConsonantNlau => "pahawh hmong consonant nlau",
            PahawhHmong::ConsonantRau => "pahawh hmong consonant rau",
            PahawhHmong::ConsonantNkau => "pahawh hmong consonant nkau",
            PahawhHmong::ConsonantQhau => "pahawh hmong consonant qhau",
            PahawhHmong::ConsonantYau => "pahawh hmong consonant yau",
            PahawhHmong::ConsonantHlau => "pahawh hmong consonant hlau",
            PahawhHmong::ConsonantMau => "pahawh hmong consonant mau",
            PahawhHmong::ConsonantChau => "pahawh hmong consonant chau",
            PahawhHmong::ConsonantNchau => "pahawh hmong consonant nchau",
            PahawhHmong::ConsonantHnau => "pahawh hmong consonant hnau",
            PahawhHmong::ConsonantPlhau => "pahawh hmong consonant plhau",
            PahawhHmong::ConsonantNthau => "pahawh hmong consonant nthau",
            PahawhHmong::ConsonantNau => "pahawh hmong consonant nau",
            PahawhHmong::ConsonantAu => "pahawh hmong consonant au",
            PahawhHmong::ConsonantXau => "pahawh hmong consonant xau",
            PahawhHmong::ConsonantCau => "pahawh hmong consonant cau",
            PahawhHmong::MarkCimTub => "pahawh hmong mark cim tub",
            PahawhHmong::MarkCimSo => "pahawh hmong mark cim so",
            PahawhHmong::MarkCimKes => "pahawh hmong mark cim kes",
            PahawhHmong::MarkCimKhav => "pahawh hmong mark cim khav",
            PahawhHmong::MarkCimSuam => "pahawh hmong mark cim suam",
            PahawhHmong::MarkCimHom => "pahawh hmong mark cim hom",
            PahawhHmong::MarkCimTaum => "pahawh hmong mark cim taum",
            PahawhHmong::SignVosThom => "pahawh hmong sign vos thom",
            PahawhHmong::SignVosTshabCeeb => "pahawh hmong sign vos tshab ceeb",
            PahawhHmong::SignCimCheem => "pahawh hmong sign cim cheem",
            PahawhHmong::SignVosThiab => "pahawh hmong sign vos thiab",
            PahawhHmong::SignVosFeem => "pahawh hmong sign vos feem",
            PahawhHmong::SignXyeemNtxiv => "pahawh hmong sign xyeem ntxiv",
            PahawhHmong::SignXyeemRho => "pahawh hmong sign xyeem rho",
            PahawhHmong::SignXyeemTov => "pahawh hmong sign xyeem tov",
            PahawhHmong::SignXyeemFaib => "pahawh hmong sign xyeem faib",
            PahawhHmong::SignVosSeev => "pahawh hmong sign vos seev",
            PahawhHmong::SignMeejSuab => "pahawh hmong sign meej suab",
            PahawhHmong::SignVosNrua => "pahawh hmong sign vos nrua",
            PahawhHmong::SignIbYam => "pahawh hmong sign ib yam",
            PahawhHmong::SignXaus => "pahawh hmong sign xaus",
            PahawhHmong::SignCimTsovRog => "pahawh hmong sign cim tsov rog",
            PahawhHmong::DigitZero => "pahawh hmong digit zero",
            PahawhHmong::DigitOne => "pahawh hmong digit one",
            PahawhHmong::DigitTwo => "pahawh hmong digit two",
            PahawhHmong::DigitThree => "pahawh hmong digit three",
            PahawhHmong::DigitFour => "pahawh hmong digit four",
            PahawhHmong::DigitFive => "pahawh hmong digit five",
            PahawhHmong::DigitSix => "pahawh hmong digit six",
            PahawhHmong::DigitSeven => "pahawh hmong digit seven",
            PahawhHmong::DigitEight => "pahawh hmong digit eight",
            PahawhHmong::DigitNine => "pahawh hmong digit nine",
            PahawhHmong::NumberTens => "pahawh hmong number tens",
            PahawhHmong::NumberHundreds => "pahawh hmong number hundreds",
            PahawhHmong::NumberTenThousands => "pahawh hmong number ten thousands",
            PahawhHmong::NumberMillions => "pahawh hmong number millions",
            PahawhHmong::NumberHundredMillions => "pahawh hmong number hundred millions",
            PahawhHmong::NumberTenBillions => "pahawh hmong number ten billions",
            PahawhHmong::NumberTrillions => "pahawh hmong number trillions",
            PahawhHmong::SignVosLub => "pahawh hmong sign vos lub",
            PahawhHmong::SignXyoo => "pahawh hmong sign xyoo",
            PahawhHmong::SignHli => "pahawh hmong sign hli",
            PahawhHmong::SignThirdDashStageHli => "pahawh hmong sign third-stage hli",
            PahawhHmong::SignZwjThaj => "pahawh hmong sign zwj thaj",
            PahawhHmong::SignHnub => "pahawh hmong sign hnub",
            PahawhHmong::SignNqig => "pahawh hmong sign nqig",
            PahawhHmong::SignXiab => "pahawh hmong sign xiab",
            PahawhHmong::SignNtuj => "pahawh hmong sign ntuj",
            PahawhHmong::SignAv => "pahawh hmong sign av",
            PahawhHmong::SignTxheejCeev => "pahawh hmong sign txheej ceev",
            PahawhHmong::SignMeejTseeb => "pahawh hmong sign meej tseeb",
            PahawhHmong::SignTau => "pahawh hmong sign tau",
            PahawhHmong::SignLos => "pahawh hmong sign los",
            PahawhHmong::SignMus => "pahawh hmong sign mus",
            PahawhHmong::SignCimHaisLusNtogNtog => "pahawh hmong sign cim hais lus ntog ntog",
            PahawhHmong::SignCimCuamTshooj => "pahawh hmong sign cim cuam tshooj",
            PahawhHmong::SignCimTxwv => "pahawh hmong sign cim txwv",
            PahawhHmong::SignCimTxwvChwv => "pahawh hmong sign cim txwv chwv",
            PahawhHmong::SignCimPubDawb => "pahawh hmong sign cim pub dawb",
            PahawhHmong::SignCimNresTos => "pahawh hmong sign cim nres tos",
            PahawhHmong::ClanSignTsheej => "pahawh hmong clan sign tsheej",
            PahawhHmong::ClanSignYeeg => "pahawh hmong clan sign yeeg",
            PahawhHmong::ClanSignLis => "pahawh hmong clan sign lis",
            PahawhHmong::ClanSignLauj => "pahawh hmong clan sign lauj",
            PahawhHmong::ClanSignXyooj => "pahawh hmong clan sign xyooj",
            PahawhHmong::ClanSignKoo => "pahawh hmong clan sign koo",
            PahawhHmong::ClanSignHawj => "pahawh hmong clan sign hawj",
            PahawhHmong::ClanSignMuas => "pahawh hmong clan sign muas",
            PahawhHmong::ClanSignThoj => "pahawh hmong clan sign thoj",
            PahawhHmong::ClanSignTsab => "pahawh hmong clan sign tsab",
            PahawhHmong::ClanSignPhab => "pahawh hmong clan sign phab",
            PahawhHmong::ClanSignKhab => "pahawh hmong clan sign khab",
            PahawhHmong::ClanSignHam => "pahawh hmong clan sign ham",
            PahawhHmong::ClanSignVaj => "pahawh hmong clan sign vaj",
            PahawhHmong::ClanSignFaj => "pahawh hmong clan sign faj",
            PahawhHmong::ClanSignYaj => "pahawh hmong clan sign yaj",
            PahawhHmong::ClanSignTswb => "pahawh hmong clan sign tswb",
            PahawhHmong::ClanSignKwm => "pahawh hmong clan sign kwm",
        }
    }
}
