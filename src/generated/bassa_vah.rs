
/// An enum to represent all characters in the BassaVah block.
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
        match self {
            BassaVah::LetterEnni => '𖫐',
            BassaVah::LetterKa => '𖫑',
            BassaVah::LetterSe => '𖫒',
            BassaVah::LetterFa => '𖫓',
            BassaVah::LetterMbe => '𖫔',
            BassaVah::LetterYie => '𖫕',
            BassaVah::LetterGah => '𖫖',
            BassaVah::LetterDhii => '𖫗',
            BassaVah::LetterKpah => '𖫘',
            BassaVah::LetterJo => '𖫙',
            BassaVah::LetterHwah => '𖫚',
            BassaVah::LetterWa => '𖫛',
            BassaVah::LetterZo => '𖫜',
            BassaVah::LetterGbu => '𖫝',
            BassaVah::LetterDo => '𖫞',
            BassaVah::LetterCe => '𖫟',
            BassaVah::LetterUwu => '𖫠',
            BassaVah::LetterTo => '𖫡',
            BassaVah::LetterBa => '𖫢',
            BassaVah::LetterVu => '𖫣',
            BassaVah::LetterYein => '𖫤',
            BassaVah::LetterPa => '𖫥',
            BassaVah::LetterWadda => '𖫦',
            BassaVah::LetterA => '𖫧',
            BassaVah::LetterO => '𖫨',
            BassaVah::LetterOo => '𖫩',
            BassaVah::LetterU => '𖫪',
            BassaVah::LetterEe => '𖫫',
            BassaVah::LetterE => '𖫬',
            BassaVah::LetterI => '𖫭',
            BassaVah::CombiningHighTone => '𖫰',
            BassaVah::CombiningLowTone => '𖫱',
            BassaVah::CombiningMidTone => '𖫲',
            BassaVah::CombiningLowDashMidTone => '𖫳',
            BassaVah::CombiningHighDashLowTone => '𖫴',
            BassaVah::FullStop => '𖫵',
        }
    }
}

impl std::convert::TryFrom<char> for BassaVah {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𖫐' => Ok(BassaVah::LetterEnni),
            '𖫑' => Ok(BassaVah::LetterKa),
            '𖫒' => Ok(BassaVah::LetterSe),
            '𖫓' => Ok(BassaVah::LetterFa),
            '𖫔' => Ok(BassaVah::LetterMbe),
            '𖫕' => Ok(BassaVah::LetterYie),
            '𖫖' => Ok(BassaVah::LetterGah),
            '𖫗' => Ok(BassaVah::LetterDhii),
            '𖫘' => Ok(BassaVah::LetterKpah),
            '𖫙' => Ok(BassaVah::LetterJo),
            '𖫚' => Ok(BassaVah::LetterHwah),
            '𖫛' => Ok(BassaVah::LetterWa),
            '𖫜' => Ok(BassaVah::LetterZo),
            '𖫝' => Ok(BassaVah::LetterGbu),
            '𖫞' => Ok(BassaVah::LetterDo),
            '𖫟' => Ok(BassaVah::LetterCe),
            '𖫠' => Ok(BassaVah::LetterUwu),
            '𖫡' => Ok(BassaVah::LetterTo),
            '𖫢' => Ok(BassaVah::LetterBa),
            '𖫣' => Ok(BassaVah::LetterVu),
            '𖫤' => Ok(BassaVah::LetterYein),
            '𖫥' => Ok(BassaVah::LetterPa),
            '𖫦' => Ok(BassaVah::LetterWadda),
            '𖫧' => Ok(BassaVah::LetterA),
            '𖫨' => Ok(BassaVah::LetterO),
            '𖫩' => Ok(BassaVah::LetterOo),
            '𖫪' => Ok(BassaVah::LetterU),
            '𖫫' => Ok(BassaVah::LetterEe),
            '𖫬' => Ok(BassaVah::LetterE),
            '𖫭' => Ok(BassaVah::LetterI),
            '𖫰' => Ok(BassaVah::CombiningHighTone),
            '𖫱' => Ok(BassaVah::CombiningLowTone),
            '𖫲' => Ok(BassaVah::CombiningMidTone),
            '𖫳' => Ok(BassaVah::CombiningLowDashMidTone),
            '𖫴' => Ok(BassaVah::CombiningHighDashLowTone),
            '𖫵' => Ok(BassaVah::FullStop),
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
