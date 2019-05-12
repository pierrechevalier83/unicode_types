/// \u{16ad0} → \u{16aff}\
///\
/// 𖫐 𖫑 𖫒 𖫓 𖫔 𖫕 𖫖 𖫗 𖫘 𖫙 𖫚 𖫛 𖫜 𖫝 𖫞 𖫟
/// 𖫠 𖫡 𖫢 𖫣 𖫤 𖫥 𖫦 𖫧 𖫨 𖫩 𖫪 𖫫 𖫬 𖫭 𖫰 𖫱
/// 𖫲 𖫳 𖫴 𖫵
pub mod constants {
    /// \u{16ad0}: '𖫐'
    pub const BASSA_VAH_LETTER_ENNI: char = '𖫐';
    /// \u{16ad1}: '𖫑'
    pub const BASSA_VAH_LETTER_KA: char = '𖫑';
    /// \u{16ad2}: '𖫒'
    pub const BASSA_VAH_LETTER_SE: char = '𖫒';
    /// \u{16ad3}: '𖫓'
    pub const BASSA_VAH_LETTER_FA: char = '𖫓';
    /// \u{16ad4}: '𖫔'
    pub const BASSA_VAH_LETTER_MBE: char = '𖫔';
    /// \u{16ad5}: '𖫕'
    pub const BASSA_VAH_LETTER_YIE: char = '𖫕';
    /// \u{16ad6}: '𖫖'
    pub const BASSA_VAH_LETTER_GAH: char = '𖫖';
    /// \u{16ad7}: '𖫗'
    pub const BASSA_VAH_LETTER_DHII: char = '𖫗';
    /// \u{16ad8}: '𖫘'
    pub const BASSA_VAH_LETTER_KPAH: char = '𖫘';
    /// \u{16ad9}: '𖫙'
    pub const BASSA_VAH_LETTER_JO: char = '𖫙';
    /// \u{16ada}: '𖫚'
    pub const BASSA_VAH_LETTER_HWAH: char = '𖫚';
    /// \u{16adb}: '𖫛'
    pub const BASSA_VAH_LETTER_WA: char = '𖫛';
    /// \u{16adc}: '𖫜'
    pub const BASSA_VAH_LETTER_ZO: char = '𖫜';
    /// \u{16add}: '𖫝'
    pub const BASSA_VAH_LETTER_GBU: char = '𖫝';
    /// \u{16ade}: '𖫞'
    pub const BASSA_VAH_LETTER_DO: char = '𖫞';
    /// \u{16adf}: '𖫟'
    pub const BASSA_VAH_LETTER_CE: char = '𖫟';
    /// \u{16ae0}: '𖫠'
    pub const BASSA_VAH_LETTER_UWU: char = '𖫠';
    /// \u{16ae1}: '𖫡'
    pub const BASSA_VAH_LETTER_TO: char = '𖫡';
    /// \u{16ae2}: '𖫢'
    pub const BASSA_VAH_LETTER_BA: char = '𖫢';
    /// \u{16ae3}: '𖫣'
    pub const BASSA_VAH_LETTER_VU: char = '𖫣';
    /// \u{16ae4}: '𖫤'
    pub const BASSA_VAH_LETTER_YEIN: char = '𖫤';
    /// \u{16ae5}: '𖫥'
    pub const BASSA_VAH_LETTER_PA: char = '𖫥';
    /// \u{16ae6}: '𖫦'
    pub const BASSA_VAH_LETTER_WADDA: char = '𖫦';
    /// \u{16ae7}: '𖫧'
    pub const BASSA_VAH_LETTER_A: char = '𖫧';
    /// \u{16ae8}: '𖫨'
    pub const BASSA_VAH_LETTER_O: char = '𖫨';
    /// \u{16ae9}: '𖫩'
    pub const BASSA_VAH_LETTER_OO: char = '𖫩';
    /// \u{16aea}: '𖫪'
    pub const BASSA_VAH_LETTER_U: char = '𖫪';
    /// \u{16aeb}: '𖫫'
    pub const BASSA_VAH_LETTER_EE: char = '𖫫';
    /// \u{16aec}: '𖫬'
    pub const BASSA_VAH_LETTER_E: char = '𖫬';
    /// \u{16aed}: '𖫭'
    pub const BASSA_VAH_LETTER_I: char = '𖫭';
    /// \u{16af0}: '𖫰'
    pub const BASSA_VAH_COMBINING_HIGH_TONE: char = '𖫰';
    /// \u{16af1}: '𖫱'
    pub const BASSA_VAH_COMBINING_LOW_TONE: char = '𖫱';
    /// \u{16af2}: '𖫲'
    pub const BASSA_VAH_COMBINING_MID_TONE: char = '𖫲';
    /// \u{16af3}: '𖫳'
    pub const BASSA_VAH_COMBINING_LOW_DASH_MID_TONE: char = '𖫳';
    /// \u{16af4}: '𖫴'
    pub const BASSA_VAH_COMBINING_HIGH_DASH_LOW_TONE: char = '𖫴';
    /// \u{16af5}: '𖫵'
    pub const BASSA_VAH_FULL_STOP: char = '𖫵';
}

/// \u{16ad0} → \u{16aff}\
///\
/// 𖫐 𖫑 𖫒 𖫓 𖫔 𖫕 𖫖 𖫗 𖫘 𖫙 𖫚 𖫛 𖫜 𖫝 𖫞 𖫟
/// 𖫠 𖫡 𖫢 𖫣 𖫤 𖫥 𖫦 𖫧 𖫨 𖫩 𖫪 𖫫 𖫬 𖫭 𖫰 𖫱
/// 𖫲 𖫳 𖫴 𖫵
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum BassaVah {
    /// \u{16ad0}: '𖫐'
    BassaVahLetterEnni,
    /// \u{16ad1}: '𖫑'
    BassaVahLetterKa,
    /// \u{16ad2}: '𖫒'
    BassaVahLetterSe,
    /// \u{16ad3}: '𖫓'
    BassaVahLetterFa,
    /// \u{16ad4}: '𖫔'
    BassaVahLetterMbe,
    /// \u{16ad5}: '𖫕'
    BassaVahLetterYie,
    /// \u{16ad6}: '𖫖'
    BassaVahLetterGah,
    /// \u{16ad7}: '𖫗'
    BassaVahLetterDhii,
    /// \u{16ad8}: '𖫘'
    BassaVahLetterKpah,
    /// \u{16ad9}: '𖫙'
    BassaVahLetterJo,
    /// \u{16ada}: '𖫚'
    BassaVahLetterHwah,
    /// \u{16adb}: '𖫛'
    BassaVahLetterWa,
    /// \u{16adc}: '𖫜'
    BassaVahLetterZo,
    /// \u{16add}: '𖫝'
    BassaVahLetterGbu,
    /// \u{16ade}: '𖫞'
    BassaVahLetterDo,
    /// \u{16adf}: '𖫟'
    BassaVahLetterCe,
    /// \u{16ae0}: '𖫠'
    BassaVahLetterUwu,
    /// \u{16ae1}: '𖫡'
    BassaVahLetterTo,
    /// \u{16ae2}: '𖫢'
    BassaVahLetterBa,
    /// \u{16ae3}: '𖫣'
    BassaVahLetterVu,
    /// \u{16ae4}: '𖫤'
    BassaVahLetterYein,
    /// \u{16ae5}: '𖫥'
    BassaVahLetterPa,
    /// \u{16ae6}: '𖫦'
    BassaVahLetterWadda,
    /// \u{16ae7}: '𖫧'
    BassaVahLetterA,
    /// \u{16ae8}: '𖫨'
    BassaVahLetterO,
    /// \u{16ae9}: '𖫩'
    BassaVahLetterOo,
    /// \u{16aea}: '𖫪'
    BassaVahLetterU,
    /// \u{16aeb}: '𖫫'
    BassaVahLetterEe,
    /// \u{16aec}: '𖫬'
    BassaVahLetterE,
    /// \u{16aed}: '𖫭'
    BassaVahLetterI,
    /// \u{16af0}: '𖫰'
    BassaVahCombiningHighTone,
    /// \u{16af1}: '𖫱'
    BassaVahCombiningLowTone,
    /// \u{16af2}: '𖫲'
    BassaVahCombiningMidTone,
    /// \u{16af3}: '𖫳'
    BassaVahCombiningLowDashMidTone,
    /// \u{16af4}: '𖫴'
    BassaVahCombiningHighDashLowTone,
    /// \u{16af5}: '𖫵'
    BassaVahFullStop,
}

impl Into<char> for BassaVah {
    fn into(self) -> char {
        use constants::*;
        match self {
            BassaVah::BassaVahLetterEnni => BASSA_VAH_LETTER_ENNI,
            BassaVah::BassaVahLetterKa => BASSA_VAH_LETTER_KA,
            BassaVah::BassaVahLetterSe => BASSA_VAH_LETTER_SE,
            BassaVah::BassaVahLetterFa => BASSA_VAH_LETTER_FA,
            BassaVah::BassaVahLetterMbe => BASSA_VAH_LETTER_MBE,
            BassaVah::BassaVahLetterYie => BASSA_VAH_LETTER_YIE,
            BassaVah::BassaVahLetterGah => BASSA_VAH_LETTER_GAH,
            BassaVah::BassaVahLetterDhii => BASSA_VAH_LETTER_DHII,
            BassaVah::BassaVahLetterKpah => BASSA_VAH_LETTER_KPAH,
            BassaVah::BassaVahLetterJo => BASSA_VAH_LETTER_JO,
            BassaVah::BassaVahLetterHwah => BASSA_VAH_LETTER_HWAH,
            BassaVah::BassaVahLetterWa => BASSA_VAH_LETTER_WA,
            BassaVah::BassaVahLetterZo => BASSA_VAH_LETTER_ZO,
            BassaVah::BassaVahLetterGbu => BASSA_VAH_LETTER_GBU,
            BassaVah::BassaVahLetterDo => BASSA_VAH_LETTER_DO,
            BassaVah::BassaVahLetterCe => BASSA_VAH_LETTER_CE,
            BassaVah::BassaVahLetterUwu => BASSA_VAH_LETTER_UWU,
            BassaVah::BassaVahLetterTo => BASSA_VAH_LETTER_TO,
            BassaVah::BassaVahLetterBa => BASSA_VAH_LETTER_BA,
            BassaVah::BassaVahLetterVu => BASSA_VAH_LETTER_VU,
            BassaVah::BassaVahLetterYein => BASSA_VAH_LETTER_YEIN,
            BassaVah::BassaVahLetterPa => BASSA_VAH_LETTER_PA,
            BassaVah::BassaVahLetterWadda => BASSA_VAH_LETTER_WADDA,
            BassaVah::BassaVahLetterA => BASSA_VAH_LETTER_A,
            BassaVah::BassaVahLetterO => BASSA_VAH_LETTER_O,
            BassaVah::BassaVahLetterOo => BASSA_VAH_LETTER_OO,
            BassaVah::BassaVahLetterU => BASSA_VAH_LETTER_U,
            BassaVah::BassaVahLetterEe => BASSA_VAH_LETTER_EE,
            BassaVah::BassaVahLetterE => BASSA_VAH_LETTER_E,
            BassaVah::BassaVahLetterI => BASSA_VAH_LETTER_I,
            BassaVah::BassaVahCombiningHighTone => BASSA_VAH_COMBINING_HIGH_TONE,
            BassaVah::BassaVahCombiningLowTone => BASSA_VAH_COMBINING_LOW_TONE,
            BassaVah::BassaVahCombiningMidTone => BASSA_VAH_COMBINING_MID_TONE,
            BassaVah::BassaVahCombiningLowDashMidTone => BASSA_VAH_COMBINING_LOW_DASH_MID_TONE,
            BassaVah::BassaVahCombiningHighDashLowTone => BASSA_VAH_COMBINING_HIGH_DASH_LOW_TONE,
            BassaVah::BassaVahFullStop => BASSA_VAH_FULL_STOP,
        }
    }
}

impl std::convert::TryFrom<char> for BassaVah {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            BASSA_VAH_LETTER_ENNI => Ok(BassaVah::BassaVahLetterEnni),
            BASSA_VAH_LETTER_KA => Ok(BassaVah::BassaVahLetterKa),
            BASSA_VAH_LETTER_SE => Ok(BassaVah::BassaVahLetterSe),
            BASSA_VAH_LETTER_FA => Ok(BassaVah::BassaVahLetterFa),
            BASSA_VAH_LETTER_MBE => Ok(BassaVah::BassaVahLetterMbe),
            BASSA_VAH_LETTER_YIE => Ok(BassaVah::BassaVahLetterYie),
            BASSA_VAH_LETTER_GAH => Ok(BassaVah::BassaVahLetterGah),
            BASSA_VAH_LETTER_DHII => Ok(BassaVah::BassaVahLetterDhii),
            BASSA_VAH_LETTER_KPAH => Ok(BassaVah::BassaVahLetterKpah),
            BASSA_VAH_LETTER_JO => Ok(BassaVah::BassaVahLetterJo),
            BASSA_VAH_LETTER_HWAH => Ok(BassaVah::BassaVahLetterHwah),
            BASSA_VAH_LETTER_WA => Ok(BassaVah::BassaVahLetterWa),
            BASSA_VAH_LETTER_ZO => Ok(BassaVah::BassaVahLetterZo),
            BASSA_VAH_LETTER_GBU => Ok(BassaVah::BassaVahLetterGbu),
            BASSA_VAH_LETTER_DO => Ok(BassaVah::BassaVahLetterDo),
            BASSA_VAH_LETTER_CE => Ok(BassaVah::BassaVahLetterCe),
            BASSA_VAH_LETTER_UWU => Ok(BassaVah::BassaVahLetterUwu),
            BASSA_VAH_LETTER_TO => Ok(BassaVah::BassaVahLetterTo),
            BASSA_VAH_LETTER_BA => Ok(BassaVah::BassaVahLetterBa),
            BASSA_VAH_LETTER_VU => Ok(BassaVah::BassaVahLetterVu),
            BASSA_VAH_LETTER_YEIN => Ok(BassaVah::BassaVahLetterYein),
            BASSA_VAH_LETTER_PA => Ok(BassaVah::BassaVahLetterPa),
            BASSA_VAH_LETTER_WADDA => Ok(BassaVah::BassaVahLetterWadda),
            BASSA_VAH_LETTER_A => Ok(BassaVah::BassaVahLetterA),
            BASSA_VAH_LETTER_O => Ok(BassaVah::BassaVahLetterO),
            BASSA_VAH_LETTER_OO => Ok(BassaVah::BassaVahLetterOo),
            BASSA_VAH_LETTER_U => Ok(BassaVah::BassaVahLetterU),
            BASSA_VAH_LETTER_EE => Ok(BassaVah::BassaVahLetterEe),
            BASSA_VAH_LETTER_E => Ok(BassaVah::BassaVahLetterE),
            BASSA_VAH_LETTER_I => Ok(BassaVah::BassaVahLetterI),
            BASSA_VAH_COMBINING_HIGH_TONE => Ok(BassaVah::BassaVahCombiningHighTone),
            BASSA_VAH_COMBINING_LOW_TONE => Ok(BassaVah::BassaVahCombiningLowTone),
            BASSA_VAH_COMBINING_MID_TONE => Ok(BassaVah::BassaVahCombiningMidTone),
            BASSA_VAH_COMBINING_LOW_DASH_MID_TONE => Ok(BassaVah::BassaVahCombiningLowDashMidTone),
            BASSA_VAH_COMBINING_HIGH_DASH_LOW_TONE => Ok(BassaVah::BassaVahCombiningHighDashLowTone),
            BASSA_VAH_FULL_STOP => Ok(BassaVah::BassaVahFullStop),
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
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        BassaVah::BassaVahLetterEnni
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            BassaVah::BassaVahLetterEnni => "bassa vah letter enni",
            BassaVah::BassaVahLetterKa => "bassa vah letter ka",
            BassaVah::BassaVahLetterSe => "bassa vah letter se",
            BassaVah::BassaVahLetterFa => "bassa vah letter fa",
            BassaVah::BassaVahLetterMbe => "bassa vah letter mbe",
            BassaVah::BassaVahLetterYie => "bassa vah letter yie",
            BassaVah::BassaVahLetterGah => "bassa vah letter gah",
            BassaVah::BassaVahLetterDhii => "bassa vah letter dhii",
            BassaVah::BassaVahLetterKpah => "bassa vah letter kpah",
            BassaVah::BassaVahLetterJo => "bassa vah letter jo",
            BassaVah::BassaVahLetterHwah => "bassa vah letter hwah",
            BassaVah::BassaVahLetterWa => "bassa vah letter wa",
            BassaVah::BassaVahLetterZo => "bassa vah letter zo",
            BassaVah::BassaVahLetterGbu => "bassa vah letter gbu",
            BassaVah::BassaVahLetterDo => "bassa vah letter do",
            BassaVah::BassaVahLetterCe => "bassa vah letter ce",
            BassaVah::BassaVahLetterUwu => "bassa vah letter uwu",
            BassaVah::BassaVahLetterTo => "bassa vah letter to",
            BassaVah::BassaVahLetterBa => "bassa vah letter ba",
            BassaVah::BassaVahLetterVu => "bassa vah letter vu",
            BassaVah::BassaVahLetterYein => "bassa vah letter yein",
            BassaVah::BassaVahLetterPa => "bassa vah letter pa",
            BassaVah::BassaVahLetterWadda => "bassa vah letter wadda",
            BassaVah::BassaVahLetterA => "bassa vah letter a",
            BassaVah::BassaVahLetterO => "bassa vah letter o",
            BassaVah::BassaVahLetterOo => "bassa vah letter oo",
            BassaVah::BassaVahLetterU => "bassa vah letter u",
            BassaVah::BassaVahLetterEe => "bassa vah letter ee",
            BassaVah::BassaVahLetterE => "bassa vah letter e",
            BassaVah::BassaVahLetterI => "bassa vah letter i",
            BassaVah::BassaVahCombiningHighTone => "bassa vah combining high tone",
            BassaVah::BassaVahCombiningLowTone => "bassa vah combining low tone",
            BassaVah::BassaVahCombiningMidTone => "bassa vah combining mid tone",
            BassaVah::BassaVahCombiningLowDashMidTone => "bassa vah combining low-mid tone",
            BassaVah::BassaVahCombiningHighDashLowTone => "bassa vah combining high-low tone",
            BassaVah::BassaVahFullStop => "bassa vah full stop",
        }
    }
}
