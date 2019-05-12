
/// An enum to represent all characters in the ZanabazarSquare block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ZanabazarSquare {
    /// \u{11a00}: '𑨀'
    LetterA,
    /// \u{11a01}: '𑨁'
    VowelSignI,
    /// \u{11a02}: '𑨂'
    VowelSignUe,
    /// \u{11a03}: '𑨃'
    VowelSignU,
    /// \u{11a04}: '𑨄'
    VowelSignE,
    /// \u{11a05}: '𑨅'
    VowelSignOe,
    /// \u{11a06}: '𑨆'
    VowelSignO,
    /// \u{11a07}: '𑨇'
    VowelSignAi,
    /// \u{11a08}: '𑨈'
    VowelSignAu,
    /// \u{11a09}: '𑨉'
    VowelSignReversedI,
    /// \u{11a0a}: '𑨊'
    VowelLengthMark,
    /// \u{11a0b}: '𑨋'
    LetterKa,
    /// \u{11a0c}: '𑨌'
    LetterKha,
    /// \u{11a0d}: '𑨍'
    LetterGa,
    /// \u{11a0e}: '𑨎'
    LetterGha,
    /// \u{11a0f}: '𑨏'
    LetterNga,
    /// \u{11a10}: '𑨐'
    LetterCa,
    /// \u{11a11}: '𑨑'
    LetterCha,
    /// \u{11a12}: '𑨒'
    LetterJa,
    /// \u{11a13}: '𑨓'
    LetterNya,
    /// \u{11a14}: '𑨔'
    LetterTta,
    /// \u{11a15}: '𑨕'
    LetterTtha,
    /// \u{11a16}: '𑨖'
    LetterDda,
    /// \u{11a17}: '𑨗'
    LetterDdha,
    /// \u{11a18}: '𑨘'
    LetterNna,
    /// \u{11a19}: '𑨙'
    LetterTa,
    /// \u{11a1a}: '𑨚'
    LetterTha,
    /// \u{11a1b}: '𑨛'
    LetterDa,
    /// \u{11a1c}: '𑨜'
    LetterDha,
    /// \u{11a1d}: '𑨝'
    LetterNa,
    /// \u{11a1e}: '𑨞'
    LetterPa,
    /// \u{11a1f}: '𑨟'
    LetterPha,
    /// \u{11a20}: '𑨠'
    LetterBa,
    /// \u{11a21}: '𑨡'
    LetterBha,
    /// \u{11a22}: '𑨢'
    LetterMa,
    /// \u{11a23}: '𑨣'
    LetterTsa,
    /// \u{11a24}: '𑨤'
    LetterTsha,
    /// \u{11a25}: '𑨥'
    LetterDza,
    /// \u{11a26}: '𑨦'
    LetterDzha,
    /// \u{11a27}: '𑨧'
    LetterZha,
    /// \u{11a28}: '𑨨'
    LetterZa,
    /// \u{11a29}: '𑨩'
    LetterDashA,
    /// \u{11a2a}: '𑨪'
    LetterYa,
    /// \u{11a2b}: '𑨫'
    LetterRa,
    /// \u{11a2c}: '𑨬'
    LetterLa,
    /// \u{11a2d}: '𑨭'
    LetterVa,
    /// \u{11a2e}: '𑨮'
    LetterSha,
    /// \u{11a2f}: '𑨯'
    LetterSsa,
    /// \u{11a30}: '𑨰'
    LetterSa,
    /// \u{11a31}: '𑨱'
    LetterHa,
    /// \u{11a32}: '𑨲'
    LetterKssa,
    /// \u{11a33}: '𑨳'
    FinalConsonantMark,
    /// \u{11a34}: '𑨴'
    SignVirama,
    /// \u{11a35}: '𑨵'
    SignCandrabindu,
    /// \u{11a36}: '𑨶'
    SignCandrabinduWithOrnament,
    /// \u{11a37}: '𑨷'
    SignCandraWithOrnament,
    /// \u{11a38}: '𑨸'
    SignAnusvara,
    /// \u{11a39}: '𑨹'
    SignVisarga,
    /// \u{11a3a}: '𑨺'
    ClusterDashInitialLetterRa,
    /// \u{11a3b}: '𑨻'
    ClusterDashFinalLetterYa,
    /// \u{11a3c}: '𑨼'
    ClusterDashFinalLetterRa,
    /// \u{11a3d}: '𑨽'
    ClusterDashFinalLetterLa,
    /// \u{11a3e}: '𑨾'
    ClusterDashFinalLetterVa,
    /// \u{11a3f}: '𑨿'
    InitialHeadMark,
    /// \u{11a40}: '𑩀'
    ClosingHeadMark,
    /// \u{11a41}: '𑩁'
    MarkTsheg,
    /// \u{11a42}: '𑩂'
    MarkShad,
    /// \u{11a43}: '𑩃'
    MarkDoubleShad,
    /// \u{11a44}: '𑩄'
    MarkLongTsheg,
    /// \u{11a45}: '𑩅'
    InitialDoubleDashLinedHeadMark,
    /// \u{11a46}: '𑩆'
    ClosingDoubleDashLinedHeadMark,
    /// \u{11a47}: '𑩇'
    Subjoiner,
}

impl Into<char> for ZanabazarSquare {
    fn into(self) -> char {
        match self {
            ZanabazarSquare::LetterA => '𑨀',
            ZanabazarSquare::VowelSignI => '𑨁',
            ZanabazarSquare::VowelSignUe => '𑨂',
            ZanabazarSquare::VowelSignU => '𑨃',
            ZanabazarSquare::VowelSignE => '𑨄',
            ZanabazarSquare::VowelSignOe => '𑨅',
            ZanabazarSquare::VowelSignO => '𑨆',
            ZanabazarSquare::VowelSignAi => '𑨇',
            ZanabazarSquare::VowelSignAu => '𑨈',
            ZanabazarSquare::VowelSignReversedI => '𑨉',
            ZanabazarSquare::VowelLengthMark => '𑨊',
            ZanabazarSquare::LetterKa => '𑨋',
            ZanabazarSquare::LetterKha => '𑨌',
            ZanabazarSquare::LetterGa => '𑨍',
            ZanabazarSquare::LetterGha => '𑨎',
            ZanabazarSquare::LetterNga => '𑨏',
            ZanabazarSquare::LetterCa => '𑨐',
            ZanabazarSquare::LetterCha => '𑨑',
            ZanabazarSquare::LetterJa => '𑨒',
            ZanabazarSquare::LetterNya => '𑨓',
            ZanabazarSquare::LetterTta => '𑨔',
            ZanabazarSquare::LetterTtha => '𑨕',
            ZanabazarSquare::LetterDda => '𑨖',
            ZanabazarSquare::LetterDdha => '𑨗',
            ZanabazarSquare::LetterNna => '𑨘',
            ZanabazarSquare::LetterTa => '𑨙',
            ZanabazarSquare::LetterTha => '𑨚',
            ZanabazarSquare::LetterDa => '𑨛',
            ZanabazarSquare::LetterDha => '𑨜',
            ZanabazarSquare::LetterNa => '𑨝',
            ZanabazarSquare::LetterPa => '𑨞',
            ZanabazarSquare::LetterPha => '𑨟',
            ZanabazarSquare::LetterBa => '𑨠',
            ZanabazarSquare::LetterBha => '𑨡',
            ZanabazarSquare::LetterMa => '𑨢',
            ZanabazarSquare::LetterTsa => '𑨣',
            ZanabazarSquare::LetterTsha => '𑨤',
            ZanabazarSquare::LetterDza => '𑨥',
            ZanabazarSquare::LetterDzha => '𑨦',
            ZanabazarSquare::LetterZha => '𑨧',
            ZanabazarSquare::LetterZa => '𑨨',
            ZanabazarSquare::LetterDashA => '𑨩',
            ZanabazarSquare::LetterYa => '𑨪',
            ZanabazarSquare::LetterRa => '𑨫',
            ZanabazarSquare::LetterLa => '𑨬',
            ZanabazarSquare::LetterVa => '𑨭',
            ZanabazarSquare::LetterSha => '𑨮',
            ZanabazarSquare::LetterSsa => '𑨯',
            ZanabazarSquare::LetterSa => '𑨰',
            ZanabazarSquare::LetterHa => '𑨱',
            ZanabazarSquare::LetterKssa => '𑨲',
            ZanabazarSquare::FinalConsonantMark => '𑨳',
            ZanabazarSquare::SignVirama => '𑨴',
            ZanabazarSquare::SignCandrabindu => '𑨵',
            ZanabazarSquare::SignCandrabinduWithOrnament => '𑨶',
            ZanabazarSquare::SignCandraWithOrnament => '𑨷',
            ZanabazarSquare::SignAnusvara => '𑨸',
            ZanabazarSquare::SignVisarga => '𑨹',
            ZanabazarSquare::ClusterDashInitialLetterRa => '𑨺',
            ZanabazarSquare::ClusterDashFinalLetterYa => '𑨻',
            ZanabazarSquare::ClusterDashFinalLetterRa => '𑨼',
            ZanabazarSquare::ClusterDashFinalLetterLa => '𑨽',
            ZanabazarSquare::ClusterDashFinalLetterVa => '𑨾',
            ZanabazarSquare::InitialHeadMark => '𑨿',
            ZanabazarSquare::ClosingHeadMark => '𑩀',
            ZanabazarSquare::MarkTsheg => '𑩁',
            ZanabazarSquare::MarkShad => '𑩂',
            ZanabazarSquare::MarkDoubleShad => '𑩃',
            ZanabazarSquare::MarkLongTsheg => '𑩄',
            ZanabazarSquare::InitialDoubleDashLinedHeadMark => '𑩅',
            ZanabazarSquare::ClosingDoubleDashLinedHeadMark => '𑩆',
            ZanabazarSquare::Subjoiner => '𑩇',
        }
    }
}

impl std::convert::TryFrom<char> for ZanabazarSquare {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𑨀' => Ok(ZanabazarSquare::LetterA),
            '𑨁' => Ok(ZanabazarSquare::VowelSignI),
            '𑨂' => Ok(ZanabazarSquare::VowelSignUe),
            '𑨃' => Ok(ZanabazarSquare::VowelSignU),
            '𑨄' => Ok(ZanabazarSquare::VowelSignE),
            '𑨅' => Ok(ZanabazarSquare::VowelSignOe),
            '𑨆' => Ok(ZanabazarSquare::VowelSignO),
            '𑨇' => Ok(ZanabazarSquare::VowelSignAi),
            '𑨈' => Ok(ZanabazarSquare::VowelSignAu),
            '𑨉' => Ok(ZanabazarSquare::VowelSignReversedI),
            '𑨊' => Ok(ZanabazarSquare::VowelLengthMark),
            '𑨋' => Ok(ZanabazarSquare::LetterKa),
            '𑨌' => Ok(ZanabazarSquare::LetterKha),
            '𑨍' => Ok(ZanabazarSquare::LetterGa),
            '𑨎' => Ok(ZanabazarSquare::LetterGha),
            '𑨏' => Ok(ZanabazarSquare::LetterNga),
            '𑨐' => Ok(ZanabazarSquare::LetterCa),
            '𑨑' => Ok(ZanabazarSquare::LetterCha),
            '𑨒' => Ok(ZanabazarSquare::LetterJa),
            '𑨓' => Ok(ZanabazarSquare::LetterNya),
            '𑨔' => Ok(ZanabazarSquare::LetterTta),
            '𑨕' => Ok(ZanabazarSquare::LetterTtha),
            '𑨖' => Ok(ZanabazarSquare::LetterDda),
            '𑨗' => Ok(ZanabazarSquare::LetterDdha),
            '𑨘' => Ok(ZanabazarSquare::LetterNna),
            '𑨙' => Ok(ZanabazarSquare::LetterTa),
            '𑨚' => Ok(ZanabazarSquare::LetterTha),
            '𑨛' => Ok(ZanabazarSquare::LetterDa),
            '𑨜' => Ok(ZanabazarSquare::LetterDha),
            '𑨝' => Ok(ZanabazarSquare::LetterNa),
            '𑨞' => Ok(ZanabazarSquare::LetterPa),
            '𑨟' => Ok(ZanabazarSquare::LetterPha),
            '𑨠' => Ok(ZanabazarSquare::LetterBa),
            '𑨡' => Ok(ZanabazarSquare::LetterBha),
            '𑨢' => Ok(ZanabazarSquare::LetterMa),
            '𑨣' => Ok(ZanabazarSquare::LetterTsa),
            '𑨤' => Ok(ZanabazarSquare::LetterTsha),
            '𑨥' => Ok(ZanabazarSquare::LetterDza),
            '𑨦' => Ok(ZanabazarSquare::LetterDzha),
            '𑨧' => Ok(ZanabazarSquare::LetterZha),
            '𑨨' => Ok(ZanabazarSquare::LetterZa),
            '𑨩' => Ok(ZanabazarSquare::LetterDashA),
            '𑨪' => Ok(ZanabazarSquare::LetterYa),
            '𑨫' => Ok(ZanabazarSquare::LetterRa),
            '𑨬' => Ok(ZanabazarSquare::LetterLa),
            '𑨭' => Ok(ZanabazarSquare::LetterVa),
            '𑨮' => Ok(ZanabazarSquare::LetterSha),
            '𑨯' => Ok(ZanabazarSquare::LetterSsa),
            '𑨰' => Ok(ZanabazarSquare::LetterSa),
            '𑨱' => Ok(ZanabazarSquare::LetterHa),
            '𑨲' => Ok(ZanabazarSquare::LetterKssa),
            '𑨳' => Ok(ZanabazarSquare::FinalConsonantMark),
            '𑨴' => Ok(ZanabazarSquare::SignVirama),
            '𑨵' => Ok(ZanabazarSquare::SignCandrabindu),
            '𑨶' => Ok(ZanabazarSquare::SignCandrabinduWithOrnament),
            '𑨷' => Ok(ZanabazarSquare::SignCandraWithOrnament),
            '𑨸' => Ok(ZanabazarSquare::SignAnusvara),
            '𑨹' => Ok(ZanabazarSquare::SignVisarga),
            '𑨺' => Ok(ZanabazarSquare::ClusterDashInitialLetterRa),
            '𑨻' => Ok(ZanabazarSquare::ClusterDashFinalLetterYa),
            '𑨼' => Ok(ZanabazarSquare::ClusterDashFinalLetterRa),
            '𑨽' => Ok(ZanabazarSquare::ClusterDashFinalLetterLa),
            '𑨾' => Ok(ZanabazarSquare::ClusterDashFinalLetterVa),
            '𑨿' => Ok(ZanabazarSquare::InitialHeadMark),
            '𑩀' => Ok(ZanabazarSquare::ClosingHeadMark),
            '𑩁' => Ok(ZanabazarSquare::MarkTsheg),
            '𑩂' => Ok(ZanabazarSquare::MarkShad),
            '𑩃' => Ok(ZanabazarSquare::MarkDoubleShad),
            '𑩄' => Ok(ZanabazarSquare::MarkLongTsheg),
            '𑩅' => Ok(ZanabazarSquare::InitialDoubleDashLinedHeadMark),
            '𑩆' => Ok(ZanabazarSquare::ClosingDoubleDashLinedHeadMark),
            '𑩇' => Ok(ZanabazarSquare::Subjoiner),
            _ => Err(()),
        }
    }
}

impl Into<u32> for ZanabazarSquare {
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

impl std::convert::TryFrom<u32> for ZanabazarSquare {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for ZanabazarSquare {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl ZanabazarSquare {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        ZanabazarSquare::LetterA
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("ZanabazarSquare{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
