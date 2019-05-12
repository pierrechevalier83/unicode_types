
/// An enum to represent all characters in the PauCinHau block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum PauCinHau {
    /// \u{11ac0}: '𑫀'
    LetterPa,
    /// \u{11ac1}: '𑫁'
    LetterKa,
    /// \u{11ac2}: '𑫂'
    LetterLa,
    /// \u{11ac3}: '𑫃'
    LetterMa,
    /// \u{11ac4}: '𑫄'
    LetterDa,
    /// \u{11ac5}: '𑫅'
    LetterZa,
    /// \u{11ac6}: '𑫆'
    LetterVa,
    /// \u{11ac7}: '𑫇'
    LetterNga,
    /// \u{11ac8}: '𑫈'
    LetterHa,
    /// \u{11ac9}: '𑫉'
    LetterGa,
    /// \u{11aca}: '𑫊'
    LetterKha,
    /// \u{11acb}: '𑫋'
    LetterSa,
    /// \u{11acc}: '𑫌'
    LetterBa,
    /// \u{11acd}: '𑫍'
    LetterCa,
    /// \u{11ace}: '𑫎'
    LetterTa,
    /// \u{11acf}: '𑫏'
    LetterTha,
    /// \u{11ad0}: '𑫐'
    LetterNa,
    /// \u{11ad1}: '𑫑'
    LetterPha,
    /// \u{11ad2}: '𑫒'
    LetterRa,
    /// \u{11ad3}: '𑫓'
    LetterFa,
    /// \u{11ad4}: '𑫔'
    LetterCha,
    /// \u{11ad5}: '𑫕'
    LetterA,
    /// \u{11ad6}: '𑫖'
    LetterE,
    /// \u{11ad7}: '𑫗'
    LetterI,
    /// \u{11ad8}: '𑫘'
    LetterO,
    /// \u{11ad9}: '𑫙'
    LetterU,
    /// \u{11ada}: '𑫚'
    LetterUa,
    /// \u{11adb}: '𑫛'
    LetterIa,
    /// \u{11adc}: '𑫜'
    LetterFinalP,
    /// \u{11add}: '𑫝'
    LetterFinalK,
    /// \u{11ade}: '𑫞'
    LetterFinalT,
    /// \u{11adf}: '𑫟'
    LetterFinalM,
    /// \u{11ae0}: '𑫠'
    LetterFinalN,
    /// \u{11ae1}: '𑫡'
    LetterFinalL,
    /// \u{11ae2}: '𑫢'
    LetterFinalW,
    /// \u{11ae3}: '𑫣'
    LetterFinalNg,
    /// \u{11ae4}: '𑫤'
    LetterFinalY,
    /// \u{11ae5}: '𑫥'
    RisingToneLong,
    /// \u{11ae6}: '𑫦'
    RisingTone,
    /// \u{11ae7}: '𑫧'
    SandhiGlottalStop,
    /// \u{11ae8}: '𑫨'
    RisingToneLongFinal,
    /// \u{11ae9}: '𑫩'
    RisingToneFinal,
    /// \u{11aea}: '𑫪'
    SandhiGlottalStopFinal,
    /// \u{11aeb}: '𑫫'
    SandhiToneLong,
    /// \u{11aec}: '𑫬'
    SandhiTone,
    /// \u{11aed}: '𑫭'
    SandhiToneLongFinal,
    /// \u{11aee}: '𑫮'
    SandhiToneFinal,
    /// \u{11aef}: '𑫯'
    MidDashLevelTone,
    /// \u{11af0}: '𑫰'
    GlottalStopVariant,
    /// \u{11af1}: '𑫱'
    MidDashLevelToneLongFinal,
    /// \u{11af2}: '𑫲'
    MidDashLevelToneFinal,
    /// \u{11af3}: '𑫳'
    LowDashFallingToneLong,
    /// \u{11af4}: '𑫴'
    LowDashFallingTone,
    /// \u{11af5}: '𑫵'
    GlottalStop,
    /// \u{11af6}: '𑫶'
    LowDashFallingToneLongFinal,
    /// \u{11af7}: '𑫷'
    LowDashFallingToneFinal,
    /// \u{11af8}: '𑫸'
    GlottalStopFinal,
}

impl Into<char> for PauCinHau {
    fn into(self) -> char {
        match self {
            PauCinHau::LetterPa => '𑫀',
            PauCinHau::LetterKa => '𑫁',
            PauCinHau::LetterLa => '𑫂',
            PauCinHau::LetterMa => '𑫃',
            PauCinHau::LetterDa => '𑫄',
            PauCinHau::LetterZa => '𑫅',
            PauCinHau::LetterVa => '𑫆',
            PauCinHau::LetterNga => '𑫇',
            PauCinHau::LetterHa => '𑫈',
            PauCinHau::LetterGa => '𑫉',
            PauCinHau::LetterKha => '𑫊',
            PauCinHau::LetterSa => '𑫋',
            PauCinHau::LetterBa => '𑫌',
            PauCinHau::LetterCa => '𑫍',
            PauCinHau::LetterTa => '𑫎',
            PauCinHau::LetterTha => '𑫏',
            PauCinHau::LetterNa => '𑫐',
            PauCinHau::LetterPha => '𑫑',
            PauCinHau::LetterRa => '𑫒',
            PauCinHau::LetterFa => '𑫓',
            PauCinHau::LetterCha => '𑫔',
            PauCinHau::LetterA => '𑫕',
            PauCinHau::LetterE => '𑫖',
            PauCinHau::LetterI => '𑫗',
            PauCinHau::LetterO => '𑫘',
            PauCinHau::LetterU => '𑫙',
            PauCinHau::LetterUa => '𑫚',
            PauCinHau::LetterIa => '𑫛',
            PauCinHau::LetterFinalP => '𑫜',
            PauCinHau::LetterFinalK => '𑫝',
            PauCinHau::LetterFinalT => '𑫞',
            PauCinHau::LetterFinalM => '𑫟',
            PauCinHau::LetterFinalN => '𑫠',
            PauCinHau::LetterFinalL => '𑫡',
            PauCinHau::LetterFinalW => '𑫢',
            PauCinHau::LetterFinalNg => '𑫣',
            PauCinHau::LetterFinalY => '𑫤',
            PauCinHau::RisingToneLong => '𑫥',
            PauCinHau::RisingTone => '𑫦',
            PauCinHau::SandhiGlottalStop => '𑫧',
            PauCinHau::RisingToneLongFinal => '𑫨',
            PauCinHau::RisingToneFinal => '𑫩',
            PauCinHau::SandhiGlottalStopFinal => '𑫪',
            PauCinHau::SandhiToneLong => '𑫫',
            PauCinHau::SandhiTone => '𑫬',
            PauCinHau::SandhiToneLongFinal => '𑫭',
            PauCinHau::SandhiToneFinal => '𑫮',
            PauCinHau::MidDashLevelTone => '𑫯',
            PauCinHau::GlottalStopVariant => '𑫰',
            PauCinHau::MidDashLevelToneLongFinal => '𑫱',
            PauCinHau::MidDashLevelToneFinal => '𑫲',
            PauCinHau::LowDashFallingToneLong => '𑫳',
            PauCinHau::LowDashFallingTone => '𑫴',
            PauCinHau::GlottalStop => '𑫵',
            PauCinHau::LowDashFallingToneLongFinal => '𑫶',
            PauCinHau::LowDashFallingToneFinal => '𑫷',
            PauCinHau::GlottalStopFinal => '𑫸',
        }
    }
}

impl std::convert::TryFrom<char> for PauCinHau {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𑫀' => Ok(PauCinHau::LetterPa),
            '𑫁' => Ok(PauCinHau::LetterKa),
            '𑫂' => Ok(PauCinHau::LetterLa),
            '𑫃' => Ok(PauCinHau::LetterMa),
            '𑫄' => Ok(PauCinHau::LetterDa),
            '𑫅' => Ok(PauCinHau::LetterZa),
            '𑫆' => Ok(PauCinHau::LetterVa),
            '𑫇' => Ok(PauCinHau::LetterNga),
            '𑫈' => Ok(PauCinHau::LetterHa),
            '𑫉' => Ok(PauCinHau::LetterGa),
            '𑫊' => Ok(PauCinHau::LetterKha),
            '𑫋' => Ok(PauCinHau::LetterSa),
            '𑫌' => Ok(PauCinHau::LetterBa),
            '𑫍' => Ok(PauCinHau::LetterCa),
            '𑫎' => Ok(PauCinHau::LetterTa),
            '𑫏' => Ok(PauCinHau::LetterTha),
            '𑫐' => Ok(PauCinHau::LetterNa),
            '𑫑' => Ok(PauCinHau::LetterPha),
            '𑫒' => Ok(PauCinHau::LetterRa),
            '𑫓' => Ok(PauCinHau::LetterFa),
            '𑫔' => Ok(PauCinHau::LetterCha),
            '𑫕' => Ok(PauCinHau::LetterA),
            '𑫖' => Ok(PauCinHau::LetterE),
            '𑫗' => Ok(PauCinHau::LetterI),
            '𑫘' => Ok(PauCinHau::LetterO),
            '𑫙' => Ok(PauCinHau::LetterU),
            '𑫚' => Ok(PauCinHau::LetterUa),
            '𑫛' => Ok(PauCinHau::LetterIa),
            '𑫜' => Ok(PauCinHau::LetterFinalP),
            '𑫝' => Ok(PauCinHau::LetterFinalK),
            '𑫞' => Ok(PauCinHau::LetterFinalT),
            '𑫟' => Ok(PauCinHau::LetterFinalM),
            '𑫠' => Ok(PauCinHau::LetterFinalN),
            '𑫡' => Ok(PauCinHau::LetterFinalL),
            '𑫢' => Ok(PauCinHau::LetterFinalW),
            '𑫣' => Ok(PauCinHau::LetterFinalNg),
            '𑫤' => Ok(PauCinHau::LetterFinalY),
            '𑫥' => Ok(PauCinHau::RisingToneLong),
            '𑫦' => Ok(PauCinHau::RisingTone),
            '𑫧' => Ok(PauCinHau::SandhiGlottalStop),
            '𑫨' => Ok(PauCinHau::RisingToneLongFinal),
            '𑫩' => Ok(PauCinHau::RisingToneFinal),
            '𑫪' => Ok(PauCinHau::SandhiGlottalStopFinal),
            '𑫫' => Ok(PauCinHau::SandhiToneLong),
            '𑫬' => Ok(PauCinHau::SandhiTone),
            '𑫭' => Ok(PauCinHau::SandhiToneLongFinal),
            '𑫮' => Ok(PauCinHau::SandhiToneFinal),
            '𑫯' => Ok(PauCinHau::MidDashLevelTone),
            '𑫰' => Ok(PauCinHau::GlottalStopVariant),
            '𑫱' => Ok(PauCinHau::MidDashLevelToneLongFinal),
            '𑫲' => Ok(PauCinHau::MidDashLevelToneFinal),
            '𑫳' => Ok(PauCinHau::LowDashFallingToneLong),
            '𑫴' => Ok(PauCinHau::LowDashFallingTone),
            '𑫵' => Ok(PauCinHau::GlottalStop),
            '𑫶' => Ok(PauCinHau::LowDashFallingToneLongFinal),
            '𑫷' => Ok(PauCinHau::LowDashFallingToneFinal),
            '𑫸' => Ok(PauCinHau::GlottalStopFinal),
            _ => Err(()),
        }
    }
}

impl Into<u32> for PauCinHau {
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

impl std::convert::TryFrom<u32> for PauCinHau {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for PauCinHau {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl PauCinHau {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        PauCinHau::LetterPa
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("PauCinHau{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
