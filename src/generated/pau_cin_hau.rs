/// A number of constants to give a name to all characters in this block.
mod constants {
    /// \u{11ac0}: '𑫀'
    pub const LETTER_PA: char = '𑫀';
    /// \u{11ac1}: '𑫁'
    pub const LETTER_KA: char = '𑫁';
    /// \u{11ac2}: '𑫂'
    pub const LETTER_LA: char = '𑫂';
    /// \u{11ac3}: '𑫃'
    pub const LETTER_MA: char = '𑫃';
    /// \u{11ac4}: '𑫄'
    pub const LETTER_DA: char = '𑫄';
    /// \u{11ac5}: '𑫅'
    pub const LETTER_ZA: char = '𑫅';
    /// \u{11ac6}: '𑫆'
    pub const LETTER_VA: char = '𑫆';
    /// \u{11ac7}: '𑫇'
    pub const LETTER_NGA: char = '𑫇';
    /// \u{11ac8}: '𑫈'
    pub const LETTER_HA: char = '𑫈';
    /// \u{11ac9}: '𑫉'
    pub const LETTER_GA: char = '𑫉';
    /// \u{11aca}: '𑫊'
    pub const LETTER_KHA: char = '𑫊';
    /// \u{11acb}: '𑫋'
    pub const LETTER_SA: char = '𑫋';
    /// \u{11acc}: '𑫌'
    pub const LETTER_BA: char = '𑫌';
    /// \u{11acd}: '𑫍'
    pub const LETTER_CA: char = '𑫍';
    /// \u{11ace}: '𑫎'
    pub const LETTER_TA: char = '𑫎';
    /// \u{11acf}: '𑫏'
    pub const LETTER_THA: char = '𑫏';
    /// \u{11ad0}: '𑫐'
    pub const LETTER_NA: char = '𑫐';
    /// \u{11ad1}: '𑫑'
    pub const LETTER_PHA: char = '𑫑';
    /// \u{11ad2}: '𑫒'
    pub const LETTER_RA: char = '𑫒';
    /// \u{11ad3}: '𑫓'
    pub const LETTER_FA: char = '𑫓';
    /// \u{11ad4}: '𑫔'
    pub const LETTER_CHA: char = '𑫔';
    /// \u{11ad5}: '𑫕'
    pub const LETTER_A: char = '𑫕';
    /// \u{11ad6}: '𑫖'
    pub const LETTER_E: char = '𑫖';
    /// \u{11ad7}: '𑫗'
    pub const LETTER_I: char = '𑫗';
    /// \u{11ad8}: '𑫘'
    pub const LETTER_O: char = '𑫘';
    /// \u{11ad9}: '𑫙'
    pub const LETTER_U: char = '𑫙';
    /// \u{11ada}: '𑫚'
    pub const LETTER_UA: char = '𑫚';
    /// \u{11adb}: '𑫛'
    pub const LETTER_IA: char = '𑫛';
    /// \u{11adc}: '𑫜'
    pub const LETTER_FINAL_P: char = '𑫜';
    /// \u{11add}: '𑫝'
    pub const LETTER_FINAL_K: char = '𑫝';
    /// \u{11ade}: '𑫞'
    pub const LETTER_FINAL_T: char = '𑫞';
    /// \u{11adf}: '𑫟'
    pub const LETTER_FINAL_M: char = '𑫟';
    /// \u{11ae0}: '𑫠'
    pub const LETTER_FINAL_N: char = '𑫠';
    /// \u{11ae1}: '𑫡'
    pub const LETTER_FINAL_L: char = '𑫡';
    /// \u{11ae2}: '𑫢'
    pub const LETTER_FINAL_W: char = '𑫢';
    /// \u{11ae3}: '𑫣'
    pub const LETTER_FINAL_NG: char = '𑫣';
    /// \u{11ae4}: '𑫤'
    pub const LETTER_FINAL_Y: char = '𑫤';
    /// \u{11ae5}: '𑫥'
    pub const RISING_TONE_LONG: char = '𑫥';
    /// \u{11ae6}: '𑫦'
    pub const RISING_TONE: char = '𑫦';
    /// \u{11ae7}: '𑫧'
    pub const SANDHI_GLOTTAL_STOP: char = '𑫧';
    /// \u{11ae8}: '𑫨'
    pub const RISING_TONE_LONG_FINAL: char = '𑫨';
    /// \u{11ae9}: '𑫩'
    pub const RISING_TONE_FINAL: char = '𑫩';
    /// \u{11aea}: '𑫪'
    pub const SANDHI_GLOTTAL_STOP_FINAL: char = '𑫪';
    /// \u{11aeb}: '𑫫'
    pub const SANDHI_TONE_LONG: char = '𑫫';
    /// \u{11aec}: '𑫬'
    pub const SANDHI_TONE: char = '𑫬';
    /// \u{11aed}: '𑫭'
    pub const SANDHI_TONE_LONG_FINAL: char = '𑫭';
    /// \u{11aee}: '𑫮'
    pub const SANDHI_TONE_FINAL: char = '𑫮';
    /// \u{11aef}: '𑫯'
    pub const MID_DASH_LEVEL_TONE: char = '𑫯';
    /// \u{11af0}: '𑫰'
    pub const GLOTTAL_STOP_VARIANT: char = '𑫰';
    /// \u{11af1}: '𑫱'
    pub const MID_DASH_LEVEL_TONE_LONG_FINAL: char = '𑫱';
    /// \u{11af2}: '𑫲'
    pub const MID_DASH_LEVEL_TONE_FINAL: char = '𑫲';
    /// \u{11af3}: '𑫳'
    pub const LOW_DASH_FALLING_TONE_LONG: char = '𑫳';
    /// \u{11af4}: '𑫴'
    pub const LOW_DASH_FALLING_TONE: char = '𑫴';
    /// \u{11af5}: '𑫵'
    pub const GLOTTAL_STOP: char = '𑫵';
    /// \u{11af6}: '𑫶'
    pub const LOW_DASH_FALLING_TONE_LONG_FINAL: char = '𑫶';
    /// \u{11af7}: '𑫷'
    pub const LOW_DASH_FALLING_TONE_FINAL: char = '𑫷';
    /// \u{11af8}: '𑫸'
    pub const GLOTTAL_STOP_FINAL: char = '𑫸';
}

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
        use constants::*;
        match self {
            PauCinHau::LetterPa => LETTER_PA,
            PauCinHau::LetterKa => LETTER_KA,
            PauCinHau::LetterLa => LETTER_LA,
            PauCinHau::LetterMa => LETTER_MA,
            PauCinHau::LetterDa => LETTER_DA,
            PauCinHau::LetterZa => LETTER_ZA,
            PauCinHau::LetterVa => LETTER_VA,
            PauCinHau::LetterNga => LETTER_NGA,
            PauCinHau::LetterHa => LETTER_HA,
            PauCinHau::LetterGa => LETTER_GA,
            PauCinHau::LetterKha => LETTER_KHA,
            PauCinHau::LetterSa => LETTER_SA,
            PauCinHau::LetterBa => LETTER_BA,
            PauCinHau::LetterCa => LETTER_CA,
            PauCinHau::LetterTa => LETTER_TA,
            PauCinHau::LetterTha => LETTER_THA,
            PauCinHau::LetterNa => LETTER_NA,
            PauCinHau::LetterPha => LETTER_PHA,
            PauCinHau::LetterRa => LETTER_RA,
            PauCinHau::LetterFa => LETTER_FA,
            PauCinHau::LetterCha => LETTER_CHA,
            PauCinHau::LetterA => LETTER_A,
            PauCinHau::LetterE => LETTER_E,
            PauCinHau::LetterI => LETTER_I,
            PauCinHau::LetterO => LETTER_O,
            PauCinHau::LetterU => LETTER_U,
            PauCinHau::LetterUa => LETTER_UA,
            PauCinHau::LetterIa => LETTER_IA,
            PauCinHau::LetterFinalP => LETTER_FINAL_P,
            PauCinHau::LetterFinalK => LETTER_FINAL_K,
            PauCinHau::LetterFinalT => LETTER_FINAL_T,
            PauCinHau::LetterFinalM => LETTER_FINAL_M,
            PauCinHau::LetterFinalN => LETTER_FINAL_N,
            PauCinHau::LetterFinalL => LETTER_FINAL_L,
            PauCinHau::LetterFinalW => LETTER_FINAL_W,
            PauCinHau::LetterFinalNg => LETTER_FINAL_NG,
            PauCinHau::LetterFinalY => LETTER_FINAL_Y,
            PauCinHau::RisingToneLong => RISING_TONE_LONG,
            PauCinHau::RisingTone => RISING_TONE,
            PauCinHau::SandhiGlottalStop => SANDHI_GLOTTAL_STOP,
            PauCinHau::RisingToneLongFinal => RISING_TONE_LONG_FINAL,
            PauCinHau::RisingToneFinal => RISING_TONE_FINAL,
            PauCinHau::SandhiGlottalStopFinal => SANDHI_GLOTTAL_STOP_FINAL,
            PauCinHau::SandhiToneLong => SANDHI_TONE_LONG,
            PauCinHau::SandhiTone => SANDHI_TONE,
            PauCinHau::SandhiToneLongFinal => SANDHI_TONE_LONG_FINAL,
            PauCinHau::SandhiToneFinal => SANDHI_TONE_FINAL,
            PauCinHau::MidDashLevelTone => MID_DASH_LEVEL_TONE,
            PauCinHau::GlottalStopVariant => GLOTTAL_STOP_VARIANT,
            PauCinHau::MidDashLevelToneLongFinal => MID_DASH_LEVEL_TONE_LONG_FINAL,
            PauCinHau::MidDashLevelToneFinal => MID_DASH_LEVEL_TONE_FINAL,
            PauCinHau::LowDashFallingToneLong => LOW_DASH_FALLING_TONE_LONG,
            PauCinHau::LowDashFallingTone => LOW_DASH_FALLING_TONE,
            PauCinHau::GlottalStop => GLOTTAL_STOP,
            PauCinHau::LowDashFallingToneLongFinal => LOW_DASH_FALLING_TONE_LONG_FINAL,
            PauCinHau::LowDashFallingToneFinal => LOW_DASH_FALLING_TONE_FINAL,
            PauCinHau::GlottalStopFinal => GLOTTAL_STOP_FINAL,
        }
    }
}

impl std::convert::TryFrom<char> for PauCinHau {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_PA => Ok(PauCinHau::LetterPa),
            LETTER_KA => Ok(PauCinHau::LetterKa),
            LETTER_LA => Ok(PauCinHau::LetterLa),
            LETTER_MA => Ok(PauCinHau::LetterMa),
            LETTER_DA => Ok(PauCinHau::LetterDa),
            LETTER_ZA => Ok(PauCinHau::LetterZa),
            LETTER_VA => Ok(PauCinHau::LetterVa),
            LETTER_NGA => Ok(PauCinHau::LetterNga),
            LETTER_HA => Ok(PauCinHau::LetterHa),
            LETTER_GA => Ok(PauCinHau::LetterGa),
            LETTER_KHA => Ok(PauCinHau::LetterKha),
            LETTER_SA => Ok(PauCinHau::LetterSa),
            LETTER_BA => Ok(PauCinHau::LetterBa),
            LETTER_CA => Ok(PauCinHau::LetterCa),
            LETTER_TA => Ok(PauCinHau::LetterTa),
            LETTER_THA => Ok(PauCinHau::LetterTha),
            LETTER_NA => Ok(PauCinHau::LetterNa),
            LETTER_PHA => Ok(PauCinHau::LetterPha),
            LETTER_RA => Ok(PauCinHau::LetterRa),
            LETTER_FA => Ok(PauCinHau::LetterFa),
            LETTER_CHA => Ok(PauCinHau::LetterCha),
            LETTER_A => Ok(PauCinHau::LetterA),
            LETTER_E => Ok(PauCinHau::LetterE),
            LETTER_I => Ok(PauCinHau::LetterI),
            LETTER_O => Ok(PauCinHau::LetterO),
            LETTER_U => Ok(PauCinHau::LetterU),
            LETTER_UA => Ok(PauCinHau::LetterUa),
            LETTER_IA => Ok(PauCinHau::LetterIa),
            LETTER_FINAL_P => Ok(PauCinHau::LetterFinalP),
            LETTER_FINAL_K => Ok(PauCinHau::LetterFinalK),
            LETTER_FINAL_T => Ok(PauCinHau::LetterFinalT),
            LETTER_FINAL_M => Ok(PauCinHau::LetterFinalM),
            LETTER_FINAL_N => Ok(PauCinHau::LetterFinalN),
            LETTER_FINAL_L => Ok(PauCinHau::LetterFinalL),
            LETTER_FINAL_W => Ok(PauCinHau::LetterFinalW),
            LETTER_FINAL_NG => Ok(PauCinHau::LetterFinalNg),
            LETTER_FINAL_Y => Ok(PauCinHau::LetterFinalY),
            RISING_TONE_LONG => Ok(PauCinHau::RisingToneLong),
            RISING_TONE => Ok(PauCinHau::RisingTone),
            SANDHI_GLOTTAL_STOP => Ok(PauCinHau::SandhiGlottalStop),
            RISING_TONE_LONG_FINAL => Ok(PauCinHau::RisingToneLongFinal),
            RISING_TONE_FINAL => Ok(PauCinHau::RisingToneFinal),
            SANDHI_GLOTTAL_STOP_FINAL => Ok(PauCinHau::SandhiGlottalStopFinal),
            SANDHI_TONE_LONG => Ok(PauCinHau::SandhiToneLong),
            SANDHI_TONE => Ok(PauCinHau::SandhiTone),
            SANDHI_TONE_LONG_FINAL => Ok(PauCinHau::SandhiToneLongFinal),
            SANDHI_TONE_FINAL => Ok(PauCinHau::SandhiToneFinal),
            MID_DASH_LEVEL_TONE => Ok(PauCinHau::MidDashLevelTone),
            GLOTTAL_STOP_VARIANT => Ok(PauCinHau::GlottalStopVariant),
            MID_DASH_LEVEL_TONE_LONG_FINAL => Ok(PauCinHau::MidDashLevelToneLongFinal),
            MID_DASH_LEVEL_TONE_FINAL => Ok(PauCinHau::MidDashLevelToneFinal),
            LOW_DASH_FALLING_TONE_LONG => Ok(PauCinHau::LowDashFallingToneLong),
            LOW_DASH_FALLING_TONE => Ok(PauCinHau::LowDashFallingTone),
            GLOTTAL_STOP => Ok(PauCinHau::GlottalStop),
            LOW_DASH_FALLING_TONE_LONG_FINAL => Ok(PauCinHau::LowDashFallingToneLongFinal),
            LOW_DASH_FALLING_TONE_FINAL => Ok(PauCinHau::LowDashFallingToneFinal),
            GLOTTAL_STOP_FINAL => Ok(PauCinHau::GlottalStopFinal),
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

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            PauCinHau::LetterPa => "pau cin hau letter pa",
            PauCinHau::LetterKa => "pau cin hau letter ka",
            PauCinHau::LetterLa => "pau cin hau letter la",
            PauCinHau::LetterMa => "pau cin hau letter ma",
            PauCinHau::LetterDa => "pau cin hau letter da",
            PauCinHau::LetterZa => "pau cin hau letter za",
            PauCinHau::LetterVa => "pau cin hau letter va",
            PauCinHau::LetterNga => "pau cin hau letter nga",
            PauCinHau::LetterHa => "pau cin hau letter ha",
            PauCinHau::LetterGa => "pau cin hau letter ga",
            PauCinHau::LetterKha => "pau cin hau letter kha",
            PauCinHau::LetterSa => "pau cin hau letter sa",
            PauCinHau::LetterBa => "pau cin hau letter ba",
            PauCinHau::LetterCa => "pau cin hau letter ca",
            PauCinHau::LetterTa => "pau cin hau letter ta",
            PauCinHau::LetterTha => "pau cin hau letter tha",
            PauCinHau::LetterNa => "pau cin hau letter na",
            PauCinHau::LetterPha => "pau cin hau letter pha",
            PauCinHau::LetterRa => "pau cin hau letter ra",
            PauCinHau::LetterFa => "pau cin hau letter fa",
            PauCinHau::LetterCha => "pau cin hau letter cha",
            PauCinHau::LetterA => "pau cin hau letter a",
            PauCinHau::LetterE => "pau cin hau letter e",
            PauCinHau::LetterI => "pau cin hau letter i",
            PauCinHau::LetterO => "pau cin hau letter o",
            PauCinHau::LetterU => "pau cin hau letter u",
            PauCinHau::LetterUa => "pau cin hau letter ua",
            PauCinHau::LetterIa => "pau cin hau letter ia",
            PauCinHau::LetterFinalP => "pau cin hau letter final p",
            PauCinHau::LetterFinalK => "pau cin hau letter final k",
            PauCinHau::LetterFinalT => "pau cin hau letter final t",
            PauCinHau::LetterFinalM => "pau cin hau letter final m",
            PauCinHau::LetterFinalN => "pau cin hau letter final n",
            PauCinHau::LetterFinalL => "pau cin hau letter final l",
            PauCinHau::LetterFinalW => "pau cin hau letter final w",
            PauCinHau::LetterFinalNg => "pau cin hau letter final ng",
            PauCinHau::LetterFinalY => "pau cin hau letter final y",
            PauCinHau::RisingToneLong => "pau cin hau rising tone long",
            PauCinHau::RisingTone => "pau cin hau rising tone",
            PauCinHau::SandhiGlottalStop => "pau cin hau sandhi glottal stop",
            PauCinHau::RisingToneLongFinal => "pau cin hau rising tone long final",
            PauCinHau::RisingToneFinal => "pau cin hau rising tone final",
            PauCinHau::SandhiGlottalStopFinal => "pau cin hau sandhi glottal stop final",
            PauCinHau::SandhiToneLong => "pau cin hau sandhi tone long",
            PauCinHau::SandhiTone => "pau cin hau sandhi tone",
            PauCinHau::SandhiToneLongFinal => "pau cin hau sandhi tone long final",
            PauCinHau::SandhiToneFinal => "pau cin hau sandhi tone final",
            PauCinHau::MidDashLevelTone => "pau cin hau mid-level tone",
            PauCinHau::GlottalStopVariant => "pau cin hau glottal stop variant",
            PauCinHau::MidDashLevelToneLongFinal => "pau cin hau mid-level tone long final",
            PauCinHau::MidDashLevelToneFinal => "pau cin hau mid-level tone final",
            PauCinHau::LowDashFallingToneLong => "pau cin hau low-falling tone long",
            PauCinHau::LowDashFallingTone => "pau cin hau low-falling tone",
            PauCinHau::GlottalStop => "pau cin hau glottal stop",
            PauCinHau::LowDashFallingToneLongFinal => "pau cin hau low-falling tone long final",
            PauCinHau::LowDashFallingToneFinal => "pau cin hau low-falling tone final",
            PauCinHau::GlottalStopFinal => "pau cin hau glottal stop final",
        }
    }
}
