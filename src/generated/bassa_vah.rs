/// \u{16ad0} → \u{16aff}\
///\
/// 𖫐 𖫑 𖫒 𖫓 𖫔 𖫕 𖫖 𖫗 𖫘 𖫙 𖫚 𖫛 𖫜 𖫝 𖫞 𖫟
/// 𖫠 𖫡 𖫢 𖫣 𖫤 𖫥 𖫦 𖫧 𖫨 𖫩 𖫪 𖫫 𖫬 𖫭 𖫰 𖫱
/// 𖫲 𖫳 𖫴 𖫵
pub mod constants {
    /// \u{16ad0}: '𖫐'
    pub const LETTER_ENNI: char = '𖫐';
    /// \u{16ad1}: '𖫑'
    pub const LETTER_KA: char = '𖫑';
    /// \u{16ad2}: '𖫒'
    pub const LETTER_SE: char = '𖫒';
    /// \u{16ad3}: '𖫓'
    pub const LETTER_FA: char = '𖫓';
    /// \u{16ad4}: '𖫔'
    pub const LETTER_MBE: char = '𖫔';
    /// \u{16ad5}: '𖫕'
    pub const LETTER_YIE: char = '𖫕';
    /// \u{16ad6}: '𖫖'
    pub const LETTER_GAH: char = '𖫖';
    /// \u{16ad7}: '𖫗'
    pub const LETTER_DHII: char = '𖫗';
    /// \u{16ad8}: '𖫘'
    pub const LETTER_KPAH: char = '𖫘';
    /// \u{16ad9}: '𖫙'
    pub const LETTER_JO: char = '𖫙';
    /// \u{16ada}: '𖫚'
    pub const LETTER_HWAH: char = '𖫚';
    /// \u{16adb}: '𖫛'
    pub const LETTER_WA: char = '𖫛';
    /// \u{16adc}: '𖫜'
    pub const LETTER_ZO: char = '𖫜';
    /// \u{16add}: '𖫝'
    pub const LETTER_GBU: char = '𖫝';
    /// \u{16ade}: '𖫞'
    pub const LETTER_DO: char = '𖫞';
    /// \u{16adf}: '𖫟'
    pub const LETTER_CE: char = '𖫟';
    /// \u{16ae0}: '𖫠'
    pub const LETTER_UWU: char = '𖫠';
    /// \u{16ae1}: '𖫡'
    pub const LETTER_TO: char = '𖫡';
    /// \u{16ae2}: '𖫢'
    pub const LETTER_BA: char = '𖫢';
    /// \u{16ae3}: '𖫣'
    pub const LETTER_VU: char = '𖫣';
    /// \u{16ae4}: '𖫤'
    pub const LETTER_YEIN: char = '𖫤';
    /// \u{16ae5}: '𖫥'
    pub const LETTER_PA: char = '𖫥';
    /// \u{16ae6}: '𖫦'
    pub const LETTER_WADDA: char = '𖫦';
    /// \u{16ae7}: '𖫧'
    pub const LETTER_A: char = '𖫧';
    /// \u{16ae8}: '𖫨'
    pub const LETTER_O: char = '𖫨';
    /// \u{16ae9}: '𖫩'
    pub const LETTER_OO: char = '𖫩';
    /// \u{16aea}: '𖫪'
    pub const LETTER_U: char = '𖫪';
    /// \u{16aeb}: '𖫫'
    pub const LETTER_EE: char = '𖫫';
    /// \u{16aec}: '𖫬'
    pub const LETTER_E: char = '𖫬';
    /// \u{16aed}: '𖫭'
    pub const LETTER_I: char = '𖫭';
    /// \u{16af0}: '𖫰'
    pub const COMBINING_HIGH_TONE: char = '𖫰';
    /// \u{16af1}: '𖫱'
    pub const COMBINING_LOW_TONE: char = '𖫱';
    /// \u{16af2}: '𖫲'
    pub const COMBINING_MID_TONE: char = '𖫲';
    /// \u{16af3}: '𖫳'
    pub const COMBINING_LOW_DASH_MID_TONE: char = '𖫳';
    /// \u{16af4}: '𖫴'
    pub const COMBINING_HIGH_DASH_LOW_TONE: char = '𖫴';
    /// \u{16af5}: '𖫵'
    pub const FULL_STOP: char = '𖫵';
}

/// \u{16ad0} → \u{16aff}\
///\
/// 𖫐 𖫑 𖫒 𖫓 𖫔 𖫕 𖫖 𖫗 𖫘 𖫙 𖫚 𖫛 𖫜 𖫝 𖫞 𖫟
/// 𖫠 𖫡 𖫢 𖫣 𖫤 𖫥 𖫦 𖫧 𖫨 𖫩 𖫪 𖫫 𖫬 𖫭 𖫰 𖫱
/// 𖫲 𖫳 𖫴 𖫵
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum BassaVah {
    /// \u{16ad0}: '𖫐'
    LetterEnni,
    /// \u{16ad1}: '𖫑'
    LetterKa,
    /// \u{16ad2}: '𖫒'
    LetterSe,
    /// \u{16ad3}: '𖫓'
    LetterFa,
    /// \u{16ad4}: '𖫔'
    LetterMbe,
    /// \u{16ad5}: '𖫕'
    LetterYie,
    /// \u{16ad6}: '𖫖'
    LetterGah,
    /// \u{16ad7}: '𖫗'
    LetterDhii,
    /// \u{16ad8}: '𖫘'
    LetterKpah,
    /// \u{16ad9}: '𖫙'
    LetterJo,
    /// \u{16ada}: '𖫚'
    LetterHwah,
    /// \u{16adb}: '𖫛'
    LetterWa,
    /// \u{16adc}: '𖫜'
    LetterZo,
    /// \u{16add}: '𖫝'
    LetterGbu,
    /// \u{16ade}: '𖫞'
    LetterDo,
    /// \u{16adf}: '𖫟'
    LetterCe,
    /// \u{16ae0}: '𖫠'
    LetterUwu,
    /// \u{16ae1}: '𖫡'
    LetterTo,
    /// \u{16ae2}: '𖫢'
    LetterBa,
    /// \u{16ae3}: '𖫣'
    LetterVu,
    /// \u{16ae4}: '𖫤'
    LetterYein,
    /// \u{16ae5}: '𖫥'
    LetterPa,
    /// \u{16ae6}: '𖫦'
    LetterWadda,
    /// \u{16ae7}: '𖫧'
    LetterA,
    /// \u{16ae8}: '𖫨'
    LetterO,
    /// \u{16ae9}: '𖫩'
    LetterOo,
    /// \u{16aea}: '𖫪'
    LetterU,
    /// \u{16aeb}: '𖫫'
    LetterEe,
    /// \u{16aec}: '𖫬'
    LetterE,
    /// \u{16aed}: '𖫭'
    LetterI,
    /// \u{16af0}: '𖫰'
    CombiningHighTone,
    /// \u{16af1}: '𖫱'
    CombiningLowTone,
    /// \u{16af2}: '𖫲'
    CombiningMidTone,
    /// \u{16af3}: '𖫳'
    CombiningLowDashMidTone,
    /// \u{16af4}: '𖫴'
    CombiningHighDashLowTone,
    /// \u{16af5}: '𖫵'
    FullStop,
}

impl Into<char> for BassaVah {
    fn into(self) -> char {
        use constants::*;
        match self {
            BassaVah::LetterEnni => LETTER_ENNI,
            BassaVah::LetterKa => LETTER_KA,
            BassaVah::LetterSe => LETTER_SE,
            BassaVah::LetterFa => LETTER_FA,
            BassaVah::LetterMbe => LETTER_MBE,
            BassaVah::LetterYie => LETTER_YIE,
            BassaVah::LetterGah => LETTER_GAH,
            BassaVah::LetterDhii => LETTER_DHII,
            BassaVah::LetterKpah => LETTER_KPAH,
            BassaVah::LetterJo => LETTER_JO,
            BassaVah::LetterHwah => LETTER_HWAH,
            BassaVah::LetterWa => LETTER_WA,
            BassaVah::LetterZo => LETTER_ZO,
            BassaVah::LetterGbu => LETTER_GBU,
            BassaVah::LetterDo => LETTER_DO,
            BassaVah::LetterCe => LETTER_CE,
            BassaVah::LetterUwu => LETTER_UWU,
            BassaVah::LetterTo => LETTER_TO,
            BassaVah::LetterBa => LETTER_BA,
            BassaVah::LetterVu => LETTER_VU,
            BassaVah::LetterYein => LETTER_YEIN,
            BassaVah::LetterPa => LETTER_PA,
            BassaVah::LetterWadda => LETTER_WADDA,
            BassaVah::LetterA => LETTER_A,
            BassaVah::LetterO => LETTER_O,
            BassaVah::LetterOo => LETTER_OO,
            BassaVah::LetterU => LETTER_U,
            BassaVah::LetterEe => LETTER_EE,
            BassaVah::LetterE => LETTER_E,
            BassaVah::LetterI => LETTER_I,
            BassaVah::CombiningHighTone => COMBINING_HIGH_TONE,
            BassaVah::CombiningLowTone => COMBINING_LOW_TONE,
            BassaVah::CombiningMidTone => COMBINING_MID_TONE,
            BassaVah::CombiningLowDashMidTone => COMBINING_LOW_DASH_MID_TONE,
            BassaVah::CombiningHighDashLowTone => COMBINING_HIGH_DASH_LOW_TONE,
            BassaVah::FullStop => FULL_STOP,
        }
    }
}

impl std::convert::TryFrom<char> for BassaVah {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_ENNI => Ok(BassaVah::LetterEnni),
            LETTER_KA => Ok(BassaVah::LetterKa),
            LETTER_SE => Ok(BassaVah::LetterSe),
            LETTER_FA => Ok(BassaVah::LetterFa),
            LETTER_MBE => Ok(BassaVah::LetterMbe),
            LETTER_YIE => Ok(BassaVah::LetterYie),
            LETTER_GAH => Ok(BassaVah::LetterGah),
            LETTER_DHII => Ok(BassaVah::LetterDhii),
            LETTER_KPAH => Ok(BassaVah::LetterKpah),
            LETTER_JO => Ok(BassaVah::LetterJo),
            LETTER_HWAH => Ok(BassaVah::LetterHwah),
            LETTER_WA => Ok(BassaVah::LetterWa),
            LETTER_ZO => Ok(BassaVah::LetterZo),
            LETTER_GBU => Ok(BassaVah::LetterGbu),
            LETTER_DO => Ok(BassaVah::LetterDo),
            LETTER_CE => Ok(BassaVah::LetterCe),
            LETTER_UWU => Ok(BassaVah::LetterUwu),
            LETTER_TO => Ok(BassaVah::LetterTo),
            LETTER_BA => Ok(BassaVah::LetterBa),
            LETTER_VU => Ok(BassaVah::LetterVu),
            LETTER_YEIN => Ok(BassaVah::LetterYein),
            LETTER_PA => Ok(BassaVah::LetterPa),
            LETTER_WADDA => Ok(BassaVah::LetterWadda),
            LETTER_A => Ok(BassaVah::LetterA),
            LETTER_O => Ok(BassaVah::LetterO),
            LETTER_OO => Ok(BassaVah::LetterOo),
            LETTER_U => Ok(BassaVah::LetterU),
            LETTER_EE => Ok(BassaVah::LetterEe),
            LETTER_E => Ok(BassaVah::LetterE),
            LETTER_I => Ok(BassaVah::LetterI),
            COMBINING_HIGH_TONE => Ok(BassaVah::CombiningHighTone),
            COMBINING_LOW_TONE => Ok(BassaVah::CombiningLowTone),
            COMBINING_MID_TONE => Ok(BassaVah::CombiningMidTone),
            COMBINING_LOW_DASH_MID_TONE => Ok(BassaVah::CombiningLowDashMidTone),
            COMBINING_HIGH_DASH_LOW_TONE => Ok(BassaVah::CombiningHighDashLowTone),
            FULL_STOP => Ok(BassaVah::FullStop),
            _ => Err(()),
        }
    }
}

impl Into<u32> for BassaVah {
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

impl std::convert::TryFrom<u32> for BassaVah {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for BassaVah {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl BassaVah {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        BassaVah::LetterEnni
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            BassaVah::LetterEnni => "bassa vah letter enni",
            BassaVah::LetterKa => "bassa vah letter ka",
            BassaVah::LetterSe => "bassa vah letter se",
            BassaVah::LetterFa => "bassa vah letter fa",
            BassaVah::LetterMbe => "bassa vah letter mbe",
            BassaVah::LetterYie => "bassa vah letter yie",
            BassaVah::LetterGah => "bassa vah letter gah",
            BassaVah::LetterDhii => "bassa vah letter dhii",
            BassaVah::LetterKpah => "bassa vah letter kpah",
            BassaVah::LetterJo => "bassa vah letter jo",
            BassaVah::LetterHwah => "bassa vah letter hwah",
            BassaVah::LetterWa => "bassa vah letter wa",
            BassaVah::LetterZo => "bassa vah letter zo",
            BassaVah::LetterGbu => "bassa vah letter gbu",
            BassaVah::LetterDo => "bassa vah letter do",
            BassaVah::LetterCe => "bassa vah letter ce",
            BassaVah::LetterUwu => "bassa vah letter uwu",
            BassaVah::LetterTo => "bassa vah letter to",
            BassaVah::LetterBa => "bassa vah letter ba",
            BassaVah::LetterVu => "bassa vah letter vu",
            BassaVah::LetterYein => "bassa vah letter yein",
            BassaVah::LetterPa => "bassa vah letter pa",
            BassaVah::LetterWadda => "bassa vah letter wadda",
            BassaVah::LetterA => "bassa vah letter a",
            BassaVah::LetterO => "bassa vah letter o",
            BassaVah::LetterOo => "bassa vah letter oo",
            BassaVah::LetterU => "bassa vah letter u",
            BassaVah::LetterEe => "bassa vah letter ee",
            BassaVah::LetterE => "bassa vah letter e",
            BassaVah::LetterI => "bassa vah letter i",
            BassaVah::CombiningHighTone => "bassa vah combining high tone",
            BassaVah::CombiningLowTone => "bassa vah combining low tone",
            BassaVah::CombiningMidTone => "bassa vah combining mid tone",
            BassaVah::CombiningLowDashMidTone => "bassa vah combining low-mid tone",
            BassaVah::CombiningHighDashLowTone => "bassa vah combining high-low tone",
            BassaVah::FullStop => "bassa vah full stop",
        }
    }
}
