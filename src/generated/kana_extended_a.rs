/// \u{1b100} → \u{1b12f}\
///\
/// 𛄀 𛄁 𛄂 𛄃 𛄄 𛄅 𛄆 𛄇 𛄈 𛄉 𛄊 𛄋 𛄌 𛄍 𛄎 𛄏
/// 𛄐 𛄑 𛄒 𛄓 𛄔 𛄕 𛄖 𛄗 𛄘 𛄙 𛄚 𛄛 𛄜 𛄝 𛄞
pub mod constants {
    /// \u{1b100}: '𛄀'
    pub const HENTAIGANA_LETTER_RE_DASH_3: char = '𛄀';
    /// \u{1b101}: '𛄁'
    pub const HENTAIGANA_LETTER_RE_DASH_4: char = '𛄁';
    /// \u{1b102}: '𛄂'
    pub const HENTAIGANA_LETTER_RO_DASH_1: char = '𛄂';
    /// \u{1b103}: '𛄃'
    pub const HENTAIGANA_LETTER_RO_DASH_2: char = '𛄃';
    /// \u{1b104}: '𛄄'
    pub const HENTAIGANA_LETTER_RO_DASH_3: char = '𛄄';
    /// \u{1b105}: '𛄅'
    pub const HENTAIGANA_LETTER_RO_DASH_4: char = '𛄅';
    /// \u{1b106}: '𛄆'
    pub const HENTAIGANA_LETTER_RO_DASH_5: char = '𛄆';
    /// \u{1b107}: '𛄇'
    pub const HENTAIGANA_LETTER_RO_DASH_6: char = '𛄇';
    /// \u{1b108}: '𛄈'
    pub const HENTAIGANA_LETTER_WA_DASH_1: char = '𛄈';
    /// \u{1b109}: '𛄉'
    pub const HENTAIGANA_LETTER_WA_DASH_2: char = '𛄉';
    /// \u{1b10a}: '𛄊'
    pub const HENTAIGANA_LETTER_WA_DASH_3: char = '𛄊';
    /// \u{1b10b}: '𛄋'
    pub const HENTAIGANA_LETTER_WA_DASH_4: char = '𛄋';
    /// \u{1b10c}: '𛄌'
    pub const HENTAIGANA_LETTER_WA_DASH_5: char = '𛄌';
    /// \u{1b10d}: '𛄍'
    pub const HENTAIGANA_LETTER_WI_DASH_1: char = '𛄍';
    /// \u{1b10e}: '𛄎'
    pub const HENTAIGANA_LETTER_WI_DASH_2: char = '𛄎';
    /// \u{1b10f}: '𛄏'
    pub const HENTAIGANA_LETTER_WI_DASH_3: char = '𛄏';
    /// \u{1b110}: '𛄐'
    pub const HENTAIGANA_LETTER_WI_DASH_4: char = '𛄐';
    /// \u{1b111}: '𛄑'
    pub const HENTAIGANA_LETTER_WI_DASH_5: char = '𛄑';
    /// \u{1b112}: '𛄒'
    pub const HENTAIGANA_LETTER_WE_DASH_1: char = '𛄒';
    /// \u{1b113}: '𛄓'
    pub const HENTAIGANA_LETTER_WE_DASH_2: char = '𛄓';
    /// \u{1b114}: '𛄔'
    pub const HENTAIGANA_LETTER_WE_DASH_3: char = '𛄔';
    /// \u{1b115}: '𛄕'
    pub const HENTAIGANA_LETTER_WE_DASH_4: char = '𛄕';
    /// \u{1b116}: '𛄖'
    pub const HENTAIGANA_LETTER_WO_DASH_1: char = '𛄖';
    /// \u{1b117}: '𛄗'
    pub const HENTAIGANA_LETTER_WO_DASH_2: char = '𛄗';
    /// \u{1b118}: '𛄘'
    pub const HENTAIGANA_LETTER_WO_DASH_3: char = '𛄘';
    /// \u{1b119}: '𛄙'
    pub const HENTAIGANA_LETTER_WO_DASH_4: char = '𛄙';
    /// \u{1b11a}: '𛄚'
    pub const HENTAIGANA_LETTER_WO_DASH_5: char = '𛄚';
    /// \u{1b11b}: '𛄛'
    pub const HENTAIGANA_LETTER_WO_DASH_6: char = '𛄛';
    /// \u{1b11c}: '𛄜'
    pub const HENTAIGANA_LETTER_WO_DASH_7: char = '𛄜';
    /// \u{1b11d}: '𛄝'
    pub const HENTAIGANA_LETTER_N_DASH_MU_DASH_MO_DASH_1: char = '𛄝';
    /// \u{1b11e}: '𛄞'
    pub const HENTAIGANA_LETTER_N_DASH_MU_DASH_MO_DASH_2: char = '𛄞';
}

/// \u{1b100} → \u{1b12f}\
///\
/// 𛄀 𛄁 𛄂 𛄃 𛄄 𛄅 𛄆 𛄇 𛄈 𛄉 𛄊 𛄋 𛄌 𛄍 𛄎 𛄏
/// 𛄐 𛄑 𛄒 𛄓 𛄔 𛄕 𛄖 𛄗 𛄘 𛄙 𛄚 𛄛 𛄜 𛄝 𛄞
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum KanaExtendedA {
    /// \u{1b100}: '𛄀'
    HentaiganaLetterReDash3,
    /// \u{1b101}: '𛄁'
    HentaiganaLetterReDash4,
    /// \u{1b102}: '𛄂'
    HentaiganaLetterRoDash1,
    /// \u{1b103}: '𛄃'
    HentaiganaLetterRoDash2,
    /// \u{1b104}: '𛄄'
    HentaiganaLetterRoDash3,
    /// \u{1b105}: '𛄅'
    HentaiganaLetterRoDash4,
    /// \u{1b106}: '𛄆'
    HentaiganaLetterRoDash5,
    /// \u{1b107}: '𛄇'
    HentaiganaLetterRoDash6,
    /// \u{1b108}: '𛄈'
    HentaiganaLetterWaDash1,
    /// \u{1b109}: '𛄉'
    HentaiganaLetterWaDash2,
    /// \u{1b10a}: '𛄊'
    HentaiganaLetterWaDash3,
    /// \u{1b10b}: '𛄋'
    HentaiganaLetterWaDash4,
    /// \u{1b10c}: '𛄌'
    HentaiganaLetterWaDash5,
    /// \u{1b10d}: '𛄍'
    HentaiganaLetterWiDash1,
    /// \u{1b10e}: '𛄎'
    HentaiganaLetterWiDash2,
    /// \u{1b10f}: '𛄏'
    HentaiganaLetterWiDash3,
    /// \u{1b110}: '𛄐'
    HentaiganaLetterWiDash4,
    /// \u{1b111}: '𛄑'
    HentaiganaLetterWiDash5,
    /// \u{1b112}: '𛄒'
    HentaiganaLetterWeDash1,
    /// \u{1b113}: '𛄓'
    HentaiganaLetterWeDash2,
    /// \u{1b114}: '𛄔'
    HentaiganaLetterWeDash3,
    /// \u{1b115}: '𛄕'
    HentaiganaLetterWeDash4,
    /// \u{1b116}: '𛄖'
    HentaiganaLetterWoDash1,
    /// \u{1b117}: '𛄗'
    HentaiganaLetterWoDash2,
    /// \u{1b118}: '𛄘'
    HentaiganaLetterWoDash3,
    /// \u{1b119}: '𛄙'
    HentaiganaLetterWoDash4,
    /// \u{1b11a}: '𛄚'
    HentaiganaLetterWoDash5,
    /// \u{1b11b}: '𛄛'
    HentaiganaLetterWoDash6,
    /// \u{1b11c}: '𛄜'
    HentaiganaLetterWoDash7,
    /// \u{1b11d}: '𛄝'
    HentaiganaLetterNDashMuDashMoDash1,
    /// \u{1b11e}: '𛄞'
    HentaiganaLetterNDashMuDashMoDash2,
}

impl Into<char> for KanaExtendedA {
    fn into(self) -> char {
        use constants::*;
        match self {
            KanaExtendedA::HentaiganaLetterReDash3 => HENTAIGANA_LETTER_RE_DASH_3,
            KanaExtendedA::HentaiganaLetterReDash4 => HENTAIGANA_LETTER_RE_DASH_4,
            KanaExtendedA::HentaiganaLetterRoDash1 => HENTAIGANA_LETTER_RO_DASH_1,
            KanaExtendedA::HentaiganaLetterRoDash2 => HENTAIGANA_LETTER_RO_DASH_2,
            KanaExtendedA::HentaiganaLetterRoDash3 => HENTAIGANA_LETTER_RO_DASH_3,
            KanaExtendedA::HentaiganaLetterRoDash4 => HENTAIGANA_LETTER_RO_DASH_4,
            KanaExtendedA::HentaiganaLetterRoDash5 => HENTAIGANA_LETTER_RO_DASH_5,
            KanaExtendedA::HentaiganaLetterRoDash6 => HENTAIGANA_LETTER_RO_DASH_6,
            KanaExtendedA::HentaiganaLetterWaDash1 => HENTAIGANA_LETTER_WA_DASH_1,
            KanaExtendedA::HentaiganaLetterWaDash2 => HENTAIGANA_LETTER_WA_DASH_2,
            KanaExtendedA::HentaiganaLetterWaDash3 => HENTAIGANA_LETTER_WA_DASH_3,
            KanaExtendedA::HentaiganaLetterWaDash4 => HENTAIGANA_LETTER_WA_DASH_4,
            KanaExtendedA::HentaiganaLetterWaDash5 => HENTAIGANA_LETTER_WA_DASH_5,
            KanaExtendedA::HentaiganaLetterWiDash1 => HENTAIGANA_LETTER_WI_DASH_1,
            KanaExtendedA::HentaiganaLetterWiDash2 => HENTAIGANA_LETTER_WI_DASH_2,
            KanaExtendedA::HentaiganaLetterWiDash3 => HENTAIGANA_LETTER_WI_DASH_3,
            KanaExtendedA::HentaiganaLetterWiDash4 => HENTAIGANA_LETTER_WI_DASH_4,
            KanaExtendedA::HentaiganaLetterWiDash5 => HENTAIGANA_LETTER_WI_DASH_5,
            KanaExtendedA::HentaiganaLetterWeDash1 => HENTAIGANA_LETTER_WE_DASH_1,
            KanaExtendedA::HentaiganaLetterWeDash2 => HENTAIGANA_LETTER_WE_DASH_2,
            KanaExtendedA::HentaiganaLetterWeDash3 => HENTAIGANA_LETTER_WE_DASH_3,
            KanaExtendedA::HentaiganaLetterWeDash4 => HENTAIGANA_LETTER_WE_DASH_4,
            KanaExtendedA::HentaiganaLetterWoDash1 => HENTAIGANA_LETTER_WO_DASH_1,
            KanaExtendedA::HentaiganaLetterWoDash2 => HENTAIGANA_LETTER_WO_DASH_2,
            KanaExtendedA::HentaiganaLetterWoDash3 => HENTAIGANA_LETTER_WO_DASH_3,
            KanaExtendedA::HentaiganaLetterWoDash4 => HENTAIGANA_LETTER_WO_DASH_4,
            KanaExtendedA::HentaiganaLetterWoDash5 => HENTAIGANA_LETTER_WO_DASH_5,
            KanaExtendedA::HentaiganaLetterWoDash6 => HENTAIGANA_LETTER_WO_DASH_6,
            KanaExtendedA::HentaiganaLetterWoDash7 => HENTAIGANA_LETTER_WO_DASH_7,
            KanaExtendedA::HentaiganaLetterNDashMuDashMoDash1 => HENTAIGANA_LETTER_N_DASH_MU_DASH_MO_DASH_1,
            KanaExtendedA::HentaiganaLetterNDashMuDashMoDash2 => HENTAIGANA_LETTER_N_DASH_MU_DASH_MO_DASH_2,
        }
    }
}

impl std::convert::TryFrom<char> for KanaExtendedA {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            HENTAIGANA_LETTER_RE_DASH_3 => Ok(KanaExtendedA::HentaiganaLetterReDash3),
            HENTAIGANA_LETTER_RE_DASH_4 => Ok(KanaExtendedA::HentaiganaLetterReDash4),
            HENTAIGANA_LETTER_RO_DASH_1 => Ok(KanaExtendedA::HentaiganaLetterRoDash1),
            HENTAIGANA_LETTER_RO_DASH_2 => Ok(KanaExtendedA::HentaiganaLetterRoDash2),
            HENTAIGANA_LETTER_RO_DASH_3 => Ok(KanaExtendedA::HentaiganaLetterRoDash3),
            HENTAIGANA_LETTER_RO_DASH_4 => Ok(KanaExtendedA::HentaiganaLetterRoDash4),
            HENTAIGANA_LETTER_RO_DASH_5 => Ok(KanaExtendedA::HentaiganaLetterRoDash5),
            HENTAIGANA_LETTER_RO_DASH_6 => Ok(KanaExtendedA::HentaiganaLetterRoDash6),
            HENTAIGANA_LETTER_WA_DASH_1 => Ok(KanaExtendedA::HentaiganaLetterWaDash1),
            HENTAIGANA_LETTER_WA_DASH_2 => Ok(KanaExtendedA::HentaiganaLetterWaDash2),
            HENTAIGANA_LETTER_WA_DASH_3 => Ok(KanaExtendedA::HentaiganaLetterWaDash3),
            HENTAIGANA_LETTER_WA_DASH_4 => Ok(KanaExtendedA::HentaiganaLetterWaDash4),
            HENTAIGANA_LETTER_WA_DASH_5 => Ok(KanaExtendedA::HentaiganaLetterWaDash5),
            HENTAIGANA_LETTER_WI_DASH_1 => Ok(KanaExtendedA::HentaiganaLetterWiDash1),
            HENTAIGANA_LETTER_WI_DASH_2 => Ok(KanaExtendedA::HentaiganaLetterWiDash2),
            HENTAIGANA_LETTER_WI_DASH_3 => Ok(KanaExtendedA::HentaiganaLetterWiDash3),
            HENTAIGANA_LETTER_WI_DASH_4 => Ok(KanaExtendedA::HentaiganaLetterWiDash4),
            HENTAIGANA_LETTER_WI_DASH_5 => Ok(KanaExtendedA::HentaiganaLetterWiDash5),
            HENTAIGANA_LETTER_WE_DASH_1 => Ok(KanaExtendedA::HentaiganaLetterWeDash1),
            HENTAIGANA_LETTER_WE_DASH_2 => Ok(KanaExtendedA::HentaiganaLetterWeDash2),
            HENTAIGANA_LETTER_WE_DASH_3 => Ok(KanaExtendedA::HentaiganaLetterWeDash3),
            HENTAIGANA_LETTER_WE_DASH_4 => Ok(KanaExtendedA::HentaiganaLetterWeDash4),
            HENTAIGANA_LETTER_WO_DASH_1 => Ok(KanaExtendedA::HentaiganaLetterWoDash1),
            HENTAIGANA_LETTER_WO_DASH_2 => Ok(KanaExtendedA::HentaiganaLetterWoDash2),
            HENTAIGANA_LETTER_WO_DASH_3 => Ok(KanaExtendedA::HentaiganaLetterWoDash3),
            HENTAIGANA_LETTER_WO_DASH_4 => Ok(KanaExtendedA::HentaiganaLetterWoDash4),
            HENTAIGANA_LETTER_WO_DASH_5 => Ok(KanaExtendedA::HentaiganaLetterWoDash5),
            HENTAIGANA_LETTER_WO_DASH_6 => Ok(KanaExtendedA::HentaiganaLetterWoDash6),
            HENTAIGANA_LETTER_WO_DASH_7 => Ok(KanaExtendedA::HentaiganaLetterWoDash7),
            HENTAIGANA_LETTER_N_DASH_MU_DASH_MO_DASH_1 => Ok(KanaExtendedA::HentaiganaLetterNDashMuDashMoDash1),
            HENTAIGANA_LETTER_N_DASH_MU_DASH_MO_DASH_2 => Ok(KanaExtendedA::HentaiganaLetterNDashMuDashMoDash2),
            _ => Err(()),
        }
    }
}

impl Into<u32> for KanaExtendedA {
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

impl std::convert::TryFrom<u32> for KanaExtendedA {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for KanaExtendedA {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl KanaExtendedA {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        KanaExtendedA::HentaiganaLetterReDash3
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            KanaExtendedA::HentaiganaLetterReDash3 => "hentaigana letter re-3",
            KanaExtendedA::HentaiganaLetterReDash4 => "hentaigana letter re-4",
            KanaExtendedA::HentaiganaLetterRoDash1 => "hentaigana letter ro-1",
            KanaExtendedA::HentaiganaLetterRoDash2 => "hentaigana letter ro-2",
            KanaExtendedA::HentaiganaLetterRoDash3 => "hentaigana letter ro-3",
            KanaExtendedA::HentaiganaLetterRoDash4 => "hentaigana letter ro-4",
            KanaExtendedA::HentaiganaLetterRoDash5 => "hentaigana letter ro-5",
            KanaExtendedA::HentaiganaLetterRoDash6 => "hentaigana letter ro-6",
            KanaExtendedA::HentaiganaLetterWaDash1 => "hentaigana letter wa-1",
            KanaExtendedA::HentaiganaLetterWaDash2 => "hentaigana letter wa-2",
            KanaExtendedA::HentaiganaLetterWaDash3 => "hentaigana letter wa-3",
            KanaExtendedA::HentaiganaLetterWaDash4 => "hentaigana letter wa-4",
            KanaExtendedA::HentaiganaLetterWaDash5 => "hentaigana letter wa-5",
            KanaExtendedA::HentaiganaLetterWiDash1 => "hentaigana letter wi-1",
            KanaExtendedA::HentaiganaLetterWiDash2 => "hentaigana letter wi-2",
            KanaExtendedA::HentaiganaLetterWiDash3 => "hentaigana letter wi-3",
            KanaExtendedA::HentaiganaLetterWiDash4 => "hentaigana letter wi-4",
            KanaExtendedA::HentaiganaLetterWiDash5 => "hentaigana letter wi-5",
            KanaExtendedA::HentaiganaLetterWeDash1 => "hentaigana letter we-1",
            KanaExtendedA::HentaiganaLetterWeDash2 => "hentaigana letter we-2",
            KanaExtendedA::HentaiganaLetterWeDash3 => "hentaigana letter we-3",
            KanaExtendedA::HentaiganaLetterWeDash4 => "hentaigana letter we-4",
            KanaExtendedA::HentaiganaLetterWoDash1 => "hentaigana letter wo-1",
            KanaExtendedA::HentaiganaLetterWoDash2 => "hentaigana letter wo-2",
            KanaExtendedA::HentaiganaLetterWoDash3 => "hentaigana letter wo-3",
            KanaExtendedA::HentaiganaLetterWoDash4 => "hentaigana letter wo-4",
            KanaExtendedA::HentaiganaLetterWoDash5 => "hentaigana letter wo-5",
            KanaExtendedA::HentaiganaLetterWoDash6 => "hentaigana letter wo-6",
            KanaExtendedA::HentaiganaLetterWoDash7 => "hentaigana letter wo-7",
            KanaExtendedA::HentaiganaLetterNDashMuDashMoDash1 => "hentaigana letter n-mu-mo-1",
            KanaExtendedA::HentaiganaLetterNDashMuDashMoDash2 => "hentaigana letter n-mu-mo-2",
        }
    }
}
