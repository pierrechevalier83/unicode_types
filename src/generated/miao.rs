
/// An enum to represent all characters in the Miao block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Miao {
    /// \u{16f00}: '𖼀'
    LetterPa,
    /// \u{16f01}: '𖼁'
    LetterBa,
    /// \u{16f02}: '𖼂'
    LetterYiPa,
    /// \u{16f03}: '𖼃'
    LetterPla,
    /// \u{16f04}: '𖼄'
    LetterMa,
    /// \u{16f05}: '𖼅'
    LetterMha,
    /// \u{16f06}: '𖼆'
    LetterArchaicMa,
    /// \u{16f07}: '𖼇'
    LetterFa,
    /// \u{16f08}: '𖼈'
    LetterVa,
    /// \u{16f09}: '𖼉'
    LetterVfa,
    /// \u{16f0a}: '𖼊'
    LetterTa,
    /// \u{16f0b}: '𖼋'
    LetterDa,
    /// \u{16f0c}: '𖼌'
    LetterYiTta,
    /// \u{16f0d}: '𖼍'
    LetterYiTa,
    /// \u{16f0e}: '𖼎'
    LetterTta,
    /// \u{16f0f}: '𖼏'
    LetterDda,
    /// \u{16f10}: '𖼐'
    LetterNa,
    /// \u{16f11}: '𖼑'
    LetterNha,
    /// \u{16f12}: '𖼒'
    LetterYiNna,
    /// \u{16f13}: '𖼓'
    LetterArchaicNa,
    /// \u{16f14}: '𖼔'
    LetterNna,
    /// \u{16f15}: '𖼕'
    LetterNnha,
    /// \u{16f16}: '𖼖'
    LetterLa,
    /// \u{16f17}: '𖼗'
    LetterLya,
    /// \u{16f18}: '𖼘'
    LetterLha,
    /// \u{16f19}: '𖼙'
    LetterLhya,
    /// \u{16f1a}: '𖼚'
    LetterTlha,
    /// \u{16f1b}: '𖼛'
    LetterDlha,
    /// \u{16f1c}: '𖼜'
    LetterTlhya,
    /// \u{16f1d}: '𖼝'
    LetterDlhya,
    /// \u{16f1e}: '𖼞'
    LetterKa,
    /// \u{16f1f}: '𖼟'
    LetterGa,
    /// \u{16f20}: '𖼠'
    LetterYiKa,
    /// \u{16f21}: '𖼡'
    LetterQa,
    /// \u{16f22}: '𖼢'
    LetterQga,
    /// \u{16f23}: '𖼣'
    LetterNga,
    /// \u{16f24}: '𖼤'
    LetterNgha,
    /// \u{16f25}: '𖼥'
    LetterArchaicNga,
    /// \u{16f26}: '𖼦'
    LetterHa,
    /// \u{16f27}: '𖼧'
    LetterXa,
    /// \u{16f28}: '𖼨'
    LetterGha,
    /// \u{16f29}: '𖼩'
    LetterGhha,
    /// \u{16f2a}: '𖼪'
    LetterTssa,
    /// \u{16f2b}: '𖼫'
    LetterDzza,
    /// \u{16f2c}: '𖼬'
    LetterNya,
    /// \u{16f2d}: '𖼭'
    LetterNyha,
    /// \u{16f2e}: '𖼮'
    LetterTsha,
    /// \u{16f2f}: '𖼯'
    LetterDzha,
    /// \u{16f30}: '𖼰'
    LetterYiTsha,
    /// \u{16f31}: '𖼱'
    LetterYiDzha,
    /// \u{16f32}: '𖼲'
    LetterReformedTsha,
    /// \u{16f33}: '𖼳'
    LetterSha,
    /// \u{16f34}: '𖼴'
    LetterSsa,
    /// \u{16f35}: '𖼵'
    LetterZha,
    /// \u{16f36}: '𖼶'
    LetterZsha,
    /// \u{16f37}: '𖼷'
    LetterTsa,
    /// \u{16f38}: '𖼸'
    LetterDza,
    /// \u{16f39}: '𖼹'
    LetterYiTsa,
    /// \u{16f3a}: '𖼺'
    LetterSa,
    /// \u{16f3b}: '𖼻'
    LetterZa,
    /// \u{16f3c}: '𖼼'
    LetterZsa,
    /// \u{16f3d}: '𖼽'
    LetterZza,
    /// \u{16f3e}: '𖼾'
    LetterZzsa,
    /// \u{16f3f}: '𖼿'
    LetterArchaicZza,
    /// \u{16f40}: '𖽀'
    LetterZzya,
    /// \u{16f41}: '𖽁'
    LetterZzsya,
    /// \u{16f42}: '𖽂'
    LetterWa,
    /// \u{16f43}: '𖽃'
    LetterAh,
    /// \u{16f44}: '𖽄'
    LetterHha,
    /// \u{16f45}: '𖽅'
    LetterBri,
    /// \u{16f46}: '𖽆'
    LetterSyi,
    /// \u{16f47}: '𖽇'
    LetterDzyi,
    /// \u{16f48}: '𖽈'
    LetterTe,
    /// \u{16f49}: '𖽉'
    LetterTse,
    /// \u{16f4a}: '𖽊'
    LetterRte,
    /// \u{16f4f}: '𖽏'
    SignConsonantModifierBar,
    /// \u{16f50}: '𖽐'
    LetterNasalization,
    /// \u{16f51}: '𖽑'
    SignAspiration,
    /// \u{16f52}: '𖽒'
    SignReformedVoicing,
    /// \u{16f53}: '𖽓'
    SignReformedAspiration,
    /// \u{16f54}: '𖽔'
    VowelSignA,
    /// \u{16f55}: '𖽕'
    VowelSignAa,
    /// \u{16f56}: '𖽖'
    VowelSignAhh,
    /// \u{16f57}: '𖽗'
    VowelSignAn,
    /// \u{16f58}: '𖽘'
    VowelSignAng,
    /// \u{16f59}: '𖽙'
    VowelSignO,
    /// \u{16f5a}: '𖽚'
    VowelSignOo,
    /// \u{16f5b}: '𖽛'
    VowelSignWo,
    /// \u{16f5c}: '𖽜'
    VowelSignW,
    /// \u{16f5d}: '𖽝'
    VowelSignE,
    /// \u{16f5e}: '𖽞'
    VowelSignEn,
    /// \u{16f5f}: '𖽟'
    VowelSignEng,
    /// \u{16f60}: '𖽠'
    VowelSignOey,
    /// \u{16f61}: '𖽡'
    VowelSignI,
    /// \u{16f62}: '𖽢'
    VowelSignIa,
    /// \u{16f63}: '𖽣'
    VowelSignIan,
    /// \u{16f64}: '𖽤'
    VowelSignIang,
    /// \u{16f65}: '𖽥'
    VowelSignIo,
    /// \u{16f66}: '𖽦'
    VowelSignIe,
    /// \u{16f67}: '𖽧'
    VowelSignIi,
    /// \u{16f68}: '𖽨'
    VowelSignIu,
    /// \u{16f69}: '𖽩'
    VowelSignIng,
    /// \u{16f6a}: '𖽪'
    VowelSignU,
    /// \u{16f6b}: '𖽫'
    VowelSignUa,
    /// \u{16f6c}: '𖽬'
    VowelSignUan,
    /// \u{16f6d}: '𖽭'
    VowelSignUang,
    /// \u{16f6e}: '𖽮'
    VowelSignUu,
    /// \u{16f6f}: '𖽯'
    VowelSignUei,
    /// \u{16f70}: '𖽰'
    VowelSignUng,
    /// \u{16f71}: '𖽱'
    VowelSignY,
    /// \u{16f72}: '𖽲'
    VowelSignYi,
    /// \u{16f73}: '𖽳'
    VowelSignAe,
    /// \u{16f74}: '𖽴'
    VowelSignAee,
    /// \u{16f75}: '𖽵'
    VowelSignErr,
    /// \u{16f76}: '𖽶'
    VowelSignRoundedErr,
    /// \u{16f77}: '𖽷'
    VowelSignEr,
    /// \u{16f78}: '𖽸'
    VowelSignRoundedEr,
    /// \u{16f79}: '𖽹'
    VowelSignAi,
    /// \u{16f7a}: '𖽺'
    VowelSignEi,
    /// \u{16f7b}: '𖽻'
    VowelSignAu,
    /// \u{16f7c}: '𖽼'
    VowelSignOu,
    /// \u{16f7d}: '𖽽'
    VowelSignN,
    /// \u{16f7e}: '𖽾'
    VowelSignNg,
    /// \u{16f7f}: '𖽿'
    VowelSignUog,
    /// \u{16f80}: '𖾀'
    VowelSignYui,
    /// \u{16f81}: '𖾁'
    VowelSignOg,
    /// \u{16f82}: '𖾂'
    VowelSignOer,
    /// \u{16f83}: '𖾃'
    VowelSignVw,
    /// \u{16f84}: '𖾄'
    VowelSignIg,
    /// \u{16f85}: '𖾅'
    VowelSignEa,
    /// \u{16f86}: '𖾆'
    VowelSignIong,
    /// \u{16f87}: '𖾇'
    VowelSignUi,
    /// \u{16f8f}: '𖾏'
    ToneRight,
    /// \u{16f90}: '𖾐'
    ToneTopRight,
    /// \u{16f91}: '𖾑'
    ToneAbove,
    /// \u{16f92}: '𖾒'
    ToneBelow,
    /// \u{16f93}: '𖾓'
    LetterToneDash2,
    /// \u{16f94}: '𖾔'
    LetterToneDash3,
    /// \u{16f95}: '𖾕'
    LetterToneDash4,
    /// \u{16f96}: '𖾖'
    LetterToneDash5,
    /// \u{16f97}: '𖾗'
    LetterToneDash6,
    /// \u{16f98}: '𖾘'
    LetterToneDash7,
    /// \u{16f99}: '𖾙'
    LetterToneDash8,
    /// \u{16f9a}: '𖾚'
    LetterReformedToneDash1,
    /// \u{16f9b}: '𖾛'
    LetterReformedToneDash2,
    /// \u{16f9c}: '𖾜'
    LetterReformedToneDash4,
    /// \u{16f9d}: '𖾝'
    LetterReformedToneDash5,
    /// \u{16f9e}: '𖾞'
    LetterReformedToneDash6,
}

impl Into<char> for Miao {
    fn into(self) -> char {
        match self {
            Miao::LetterPa => '𖼀',
            Miao::LetterBa => '𖼁',
            Miao::LetterYiPa => '𖼂',
            Miao::LetterPla => '𖼃',
            Miao::LetterMa => '𖼄',
            Miao::LetterMha => '𖼅',
            Miao::LetterArchaicMa => '𖼆',
            Miao::LetterFa => '𖼇',
            Miao::LetterVa => '𖼈',
            Miao::LetterVfa => '𖼉',
            Miao::LetterTa => '𖼊',
            Miao::LetterDa => '𖼋',
            Miao::LetterYiTta => '𖼌',
            Miao::LetterYiTa => '𖼍',
            Miao::LetterTta => '𖼎',
            Miao::LetterDda => '𖼏',
            Miao::LetterNa => '𖼐',
            Miao::LetterNha => '𖼑',
            Miao::LetterYiNna => '𖼒',
            Miao::LetterArchaicNa => '𖼓',
            Miao::LetterNna => '𖼔',
            Miao::LetterNnha => '𖼕',
            Miao::LetterLa => '𖼖',
            Miao::LetterLya => '𖼗',
            Miao::LetterLha => '𖼘',
            Miao::LetterLhya => '𖼙',
            Miao::LetterTlha => '𖼚',
            Miao::LetterDlha => '𖼛',
            Miao::LetterTlhya => '𖼜',
            Miao::LetterDlhya => '𖼝',
            Miao::LetterKa => '𖼞',
            Miao::LetterGa => '𖼟',
            Miao::LetterYiKa => '𖼠',
            Miao::LetterQa => '𖼡',
            Miao::LetterQga => '𖼢',
            Miao::LetterNga => '𖼣',
            Miao::LetterNgha => '𖼤',
            Miao::LetterArchaicNga => '𖼥',
            Miao::LetterHa => '𖼦',
            Miao::LetterXa => '𖼧',
            Miao::LetterGha => '𖼨',
            Miao::LetterGhha => '𖼩',
            Miao::LetterTssa => '𖼪',
            Miao::LetterDzza => '𖼫',
            Miao::LetterNya => '𖼬',
            Miao::LetterNyha => '𖼭',
            Miao::LetterTsha => '𖼮',
            Miao::LetterDzha => '𖼯',
            Miao::LetterYiTsha => '𖼰',
            Miao::LetterYiDzha => '𖼱',
            Miao::LetterReformedTsha => '𖼲',
            Miao::LetterSha => '𖼳',
            Miao::LetterSsa => '𖼴',
            Miao::LetterZha => '𖼵',
            Miao::LetterZsha => '𖼶',
            Miao::LetterTsa => '𖼷',
            Miao::LetterDza => '𖼸',
            Miao::LetterYiTsa => '𖼹',
            Miao::LetterSa => '𖼺',
            Miao::LetterZa => '𖼻',
            Miao::LetterZsa => '𖼼',
            Miao::LetterZza => '𖼽',
            Miao::LetterZzsa => '𖼾',
            Miao::LetterArchaicZza => '𖼿',
            Miao::LetterZzya => '𖽀',
            Miao::LetterZzsya => '𖽁',
            Miao::LetterWa => '𖽂',
            Miao::LetterAh => '𖽃',
            Miao::LetterHha => '𖽄',
            Miao::LetterBri => '𖽅',
            Miao::LetterSyi => '𖽆',
            Miao::LetterDzyi => '𖽇',
            Miao::LetterTe => '𖽈',
            Miao::LetterTse => '𖽉',
            Miao::LetterRte => '𖽊',
            Miao::SignConsonantModifierBar => '𖽏',
            Miao::LetterNasalization => '𖽐',
            Miao::SignAspiration => '𖽑',
            Miao::SignReformedVoicing => '𖽒',
            Miao::SignReformedAspiration => '𖽓',
            Miao::VowelSignA => '𖽔',
            Miao::VowelSignAa => '𖽕',
            Miao::VowelSignAhh => '𖽖',
            Miao::VowelSignAn => '𖽗',
            Miao::VowelSignAng => '𖽘',
            Miao::VowelSignO => '𖽙',
            Miao::VowelSignOo => '𖽚',
            Miao::VowelSignWo => '𖽛',
            Miao::VowelSignW => '𖽜',
            Miao::VowelSignE => '𖽝',
            Miao::VowelSignEn => '𖽞',
            Miao::VowelSignEng => '𖽟',
            Miao::VowelSignOey => '𖽠',
            Miao::VowelSignI => '𖽡',
            Miao::VowelSignIa => '𖽢',
            Miao::VowelSignIan => '𖽣',
            Miao::VowelSignIang => '𖽤',
            Miao::VowelSignIo => '𖽥',
            Miao::VowelSignIe => '𖽦',
            Miao::VowelSignIi => '𖽧',
            Miao::VowelSignIu => '𖽨',
            Miao::VowelSignIng => '𖽩',
            Miao::VowelSignU => '𖽪',
            Miao::VowelSignUa => '𖽫',
            Miao::VowelSignUan => '𖽬',
            Miao::VowelSignUang => '𖽭',
            Miao::VowelSignUu => '𖽮',
            Miao::VowelSignUei => '𖽯',
            Miao::VowelSignUng => '𖽰',
            Miao::VowelSignY => '𖽱',
            Miao::VowelSignYi => '𖽲',
            Miao::VowelSignAe => '𖽳',
            Miao::VowelSignAee => '𖽴',
            Miao::VowelSignErr => '𖽵',
            Miao::VowelSignRoundedErr => '𖽶',
            Miao::VowelSignEr => '𖽷',
            Miao::VowelSignRoundedEr => '𖽸',
            Miao::VowelSignAi => '𖽹',
            Miao::VowelSignEi => '𖽺',
            Miao::VowelSignAu => '𖽻',
            Miao::VowelSignOu => '𖽼',
            Miao::VowelSignN => '𖽽',
            Miao::VowelSignNg => '𖽾',
            Miao::VowelSignUog => '𖽿',
            Miao::VowelSignYui => '𖾀',
            Miao::VowelSignOg => '𖾁',
            Miao::VowelSignOer => '𖾂',
            Miao::VowelSignVw => '𖾃',
            Miao::VowelSignIg => '𖾄',
            Miao::VowelSignEa => '𖾅',
            Miao::VowelSignIong => '𖾆',
            Miao::VowelSignUi => '𖾇',
            Miao::ToneRight => '𖾏',
            Miao::ToneTopRight => '𖾐',
            Miao::ToneAbove => '𖾑',
            Miao::ToneBelow => '𖾒',
            Miao::LetterToneDash2 => '𖾓',
            Miao::LetterToneDash3 => '𖾔',
            Miao::LetterToneDash4 => '𖾕',
            Miao::LetterToneDash5 => '𖾖',
            Miao::LetterToneDash6 => '𖾗',
            Miao::LetterToneDash7 => '𖾘',
            Miao::LetterToneDash8 => '𖾙',
            Miao::LetterReformedToneDash1 => '𖾚',
            Miao::LetterReformedToneDash2 => '𖾛',
            Miao::LetterReformedToneDash4 => '𖾜',
            Miao::LetterReformedToneDash5 => '𖾝',
            Miao::LetterReformedToneDash6 => '𖾞',
        }
    }
}

impl std::convert::TryFrom<char> for Miao {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𖼀' => Ok(Miao::LetterPa),
            '𖼁' => Ok(Miao::LetterBa),
            '𖼂' => Ok(Miao::LetterYiPa),
            '𖼃' => Ok(Miao::LetterPla),
            '𖼄' => Ok(Miao::LetterMa),
            '𖼅' => Ok(Miao::LetterMha),
            '𖼆' => Ok(Miao::LetterArchaicMa),
            '𖼇' => Ok(Miao::LetterFa),
            '𖼈' => Ok(Miao::LetterVa),
            '𖼉' => Ok(Miao::LetterVfa),
            '𖼊' => Ok(Miao::LetterTa),
            '𖼋' => Ok(Miao::LetterDa),
            '𖼌' => Ok(Miao::LetterYiTta),
            '𖼍' => Ok(Miao::LetterYiTa),
            '𖼎' => Ok(Miao::LetterTta),
            '𖼏' => Ok(Miao::LetterDda),
            '𖼐' => Ok(Miao::LetterNa),
            '𖼑' => Ok(Miao::LetterNha),
            '𖼒' => Ok(Miao::LetterYiNna),
            '𖼓' => Ok(Miao::LetterArchaicNa),
            '𖼔' => Ok(Miao::LetterNna),
            '𖼕' => Ok(Miao::LetterNnha),
            '𖼖' => Ok(Miao::LetterLa),
            '𖼗' => Ok(Miao::LetterLya),
            '𖼘' => Ok(Miao::LetterLha),
            '𖼙' => Ok(Miao::LetterLhya),
            '𖼚' => Ok(Miao::LetterTlha),
            '𖼛' => Ok(Miao::LetterDlha),
            '𖼜' => Ok(Miao::LetterTlhya),
            '𖼝' => Ok(Miao::LetterDlhya),
            '𖼞' => Ok(Miao::LetterKa),
            '𖼟' => Ok(Miao::LetterGa),
            '𖼠' => Ok(Miao::LetterYiKa),
            '𖼡' => Ok(Miao::LetterQa),
            '𖼢' => Ok(Miao::LetterQga),
            '𖼣' => Ok(Miao::LetterNga),
            '𖼤' => Ok(Miao::LetterNgha),
            '𖼥' => Ok(Miao::LetterArchaicNga),
            '𖼦' => Ok(Miao::LetterHa),
            '𖼧' => Ok(Miao::LetterXa),
            '𖼨' => Ok(Miao::LetterGha),
            '𖼩' => Ok(Miao::LetterGhha),
            '𖼪' => Ok(Miao::LetterTssa),
            '𖼫' => Ok(Miao::LetterDzza),
            '𖼬' => Ok(Miao::LetterNya),
            '𖼭' => Ok(Miao::LetterNyha),
            '𖼮' => Ok(Miao::LetterTsha),
            '𖼯' => Ok(Miao::LetterDzha),
            '𖼰' => Ok(Miao::LetterYiTsha),
            '𖼱' => Ok(Miao::LetterYiDzha),
            '𖼲' => Ok(Miao::LetterReformedTsha),
            '𖼳' => Ok(Miao::LetterSha),
            '𖼴' => Ok(Miao::LetterSsa),
            '𖼵' => Ok(Miao::LetterZha),
            '𖼶' => Ok(Miao::LetterZsha),
            '𖼷' => Ok(Miao::LetterTsa),
            '𖼸' => Ok(Miao::LetterDza),
            '𖼹' => Ok(Miao::LetterYiTsa),
            '𖼺' => Ok(Miao::LetterSa),
            '𖼻' => Ok(Miao::LetterZa),
            '𖼼' => Ok(Miao::LetterZsa),
            '𖼽' => Ok(Miao::LetterZza),
            '𖼾' => Ok(Miao::LetterZzsa),
            '𖼿' => Ok(Miao::LetterArchaicZza),
            '𖽀' => Ok(Miao::LetterZzya),
            '𖽁' => Ok(Miao::LetterZzsya),
            '𖽂' => Ok(Miao::LetterWa),
            '𖽃' => Ok(Miao::LetterAh),
            '𖽄' => Ok(Miao::LetterHha),
            '𖽅' => Ok(Miao::LetterBri),
            '𖽆' => Ok(Miao::LetterSyi),
            '𖽇' => Ok(Miao::LetterDzyi),
            '𖽈' => Ok(Miao::LetterTe),
            '𖽉' => Ok(Miao::LetterTse),
            '𖽊' => Ok(Miao::LetterRte),
            '𖽏' => Ok(Miao::SignConsonantModifierBar),
            '𖽐' => Ok(Miao::LetterNasalization),
            '𖽑' => Ok(Miao::SignAspiration),
            '𖽒' => Ok(Miao::SignReformedVoicing),
            '𖽓' => Ok(Miao::SignReformedAspiration),
            '𖽔' => Ok(Miao::VowelSignA),
            '𖽕' => Ok(Miao::VowelSignAa),
            '𖽖' => Ok(Miao::VowelSignAhh),
            '𖽗' => Ok(Miao::VowelSignAn),
            '𖽘' => Ok(Miao::VowelSignAng),
            '𖽙' => Ok(Miao::VowelSignO),
            '𖽚' => Ok(Miao::VowelSignOo),
            '𖽛' => Ok(Miao::VowelSignWo),
            '𖽜' => Ok(Miao::VowelSignW),
            '𖽝' => Ok(Miao::VowelSignE),
            '𖽞' => Ok(Miao::VowelSignEn),
            '𖽟' => Ok(Miao::VowelSignEng),
            '𖽠' => Ok(Miao::VowelSignOey),
            '𖽡' => Ok(Miao::VowelSignI),
            '𖽢' => Ok(Miao::VowelSignIa),
            '𖽣' => Ok(Miao::VowelSignIan),
            '𖽤' => Ok(Miao::VowelSignIang),
            '𖽥' => Ok(Miao::VowelSignIo),
            '𖽦' => Ok(Miao::VowelSignIe),
            '𖽧' => Ok(Miao::VowelSignIi),
            '𖽨' => Ok(Miao::VowelSignIu),
            '𖽩' => Ok(Miao::VowelSignIng),
            '𖽪' => Ok(Miao::VowelSignU),
            '𖽫' => Ok(Miao::VowelSignUa),
            '𖽬' => Ok(Miao::VowelSignUan),
            '𖽭' => Ok(Miao::VowelSignUang),
            '𖽮' => Ok(Miao::VowelSignUu),
            '𖽯' => Ok(Miao::VowelSignUei),
            '𖽰' => Ok(Miao::VowelSignUng),
            '𖽱' => Ok(Miao::VowelSignY),
            '𖽲' => Ok(Miao::VowelSignYi),
            '𖽳' => Ok(Miao::VowelSignAe),
            '𖽴' => Ok(Miao::VowelSignAee),
            '𖽵' => Ok(Miao::VowelSignErr),
            '𖽶' => Ok(Miao::VowelSignRoundedErr),
            '𖽷' => Ok(Miao::VowelSignEr),
            '𖽸' => Ok(Miao::VowelSignRoundedEr),
            '𖽹' => Ok(Miao::VowelSignAi),
            '𖽺' => Ok(Miao::VowelSignEi),
            '𖽻' => Ok(Miao::VowelSignAu),
            '𖽼' => Ok(Miao::VowelSignOu),
            '𖽽' => Ok(Miao::VowelSignN),
            '𖽾' => Ok(Miao::VowelSignNg),
            '𖽿' => Ok(Miao::VowelSignUog),
            '𖾀' => Ok(Miao::VowelSignYui),
            '𖾁' => Ok(Miao::VowelSignOg),
            '𖾂' => Ok(Miao::VowelSignOer),
            '𖾃' => Ok(Miao::VowelSignVw),
            '𖾄' => Ok(Miao::VowelSignIg),
            '𖾅' => Ok(Miao::VowelSignEa),
            '𖾆' => Ok(Miao::VowelSignIong),
            '𖾇' => Ok(Miao::VowelSignUi),
            '𖾏' => Ok(Miao::ToneRight),
            '𖾐' => Ok(Miao::ToneTopRight),
            '𖾑' => Ok(Miao::ToneAbove),
            '𖾒' => Ok(Miao::ToneBelow),
            '𖾓' => Ok(Miao::LetterToneDash2),
            '𖾔' => Ok(Miao::LetterToneDash3),
            '𖾕' => Ok(Miao::LetterToneDash4),
            '𖾖' => Ok(Miao::LetterToneDash5),
            '𖾗' => Ok(Miao::LetterToneDash6),
            '𖾘' => Ok(Miao::LetterToneDash7),
            '𖾙' => Ok(Miao::LetterToneDash8),
            '𖾚' => Ok(Miao::LetterReformedToneDash1),
            '𖾛' => Ok(Miao::LetterReformedToneDash2),
            '𖾜' => Ok(Miao::LetterReformedToneDash4),
            '𖾝' => Ok(Miao::LetterReformedToneDash5),
            '𖾞' => Ok(Miao::LetterReformedToneDash6),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Miao {
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

impl std::convert::TryFrom<u32> for Miao {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Miao {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Miao {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Miao::LetterPa
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Miao{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
