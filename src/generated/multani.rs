/// \u{11280} → \u{112af}\
///\
/// 𑊀 𑊁 𑊂 𑊃 𑊄 𑊅 𑊆 𑊈 𑊊 𑊋 𑊌 𑊍 𑊏 𑊐 𑊑 𑊒
/// 𑊓 𑊔 𑊕 𑊖 𑊗 𑊘 𑊙 𑊚 𑊛 𑊜 𑊝 𑊟 𑊠 𑊡 𑊢 𑊣
/// 𑊤 𑊥 𑊦 𑊧 𑊨 𑊩
pub mod constants {
    /// \u{11280}: '𑊀'
    pub const MULTANI_LETTER_A: char = '𑊀';
    /// \u{11281}: '𑊁'
    pub const MULTANI_LETTER_I: char = '𑊁';
    /// \u{11282}: '𑊂'
    pub const MULTANI_LETTER_U: char = '𑊂';
    /// \u{11283}: '𑊃'
    pub const MULTANI_LETTER_E: char = '𑊃';
    /// \u{11284}: '𑊄'
    pub const MULTANI_LETTER_KA: char = '𑊄';
    /// \u{11285}: '𑊅'
    pub const MULTANI_LETTER_KHA: char = '𑊅';
    /// \u{11286}: '𑊆'
    pub const MULTANI_LETTER_GA: char = '𑊆';
    /// \u{11288}: '𑊈'
    pub const MULTANI_LETTER_GHA: char = '𑊈';
    /// \u{1128a}: '𑊊'
    pub const MULTANI_LETTER_CA: char = '𑊊';
    /// \u{1128b}: '𑊋'
    pub const MULTANI_LETTER_CHA: char = '𑊋';
    /// \u{1128c}: '𑊌'
    pub const MULTANI_LETTER_JA: char = '𑊌';
    /// \u{1128d}: '𑊍'
    pub const MULTANI_LETTER_JJA: char = '𑊍';
    /// \u{1128f}: '𑊏'
    pub const MULTANI_LETTER_NYA: char = '𑊏';
    /// \u{11290}: '𑊐'
    pub const MULTANI_LETTER_TTA: char = '𑊐';
    /// \u{11291}: '𑊑'
    pub const MULTANI_LETTER_TTHA: char = '𑊑';
    /// \u{11292}: '𑊒'
    pub const MULTANI_LETTER_DDA: char = '𑊒';
    /// \u{11293}: '𑊓'
    pub const MULTANI_LETTER_DDDA: char = '𑊓';
    /// \u{11294}: '𑊔'
    pub const MULTANI_LETTER_DDHA: char = '𑊔';
    /// \u{11295}: '𑊕'
    pub const MULTANI_LETTER_NNA: char = '𑊕';
    /// \u{11296}: '𑊖'
    pub const MULTANI_LETTER_TA: char = '𑊖';
    /// \u{11297}: '𑊗'
    pub const MULTANI_LETTER_THA: char = '𑊗';
    /// \u{11298}: '𑊘'
    pub const MULTANI_LETTER_DA: char = '𑊘';
    /// \u{11299}: '𑊙'
    pub const MULTANI_LETTER_DHA: char = '𑊙';
    /// \u{1129a}: '𑊚'
    pub const MULTANI_LETTER_NA: char = '𑊚';
    /// \u{1129b}: '𑊛'
    pub const MULTANI_LETTER_PA: char = '𑊛';
    /// \u{1129c}: '𑊜'
    pub const MULTANI_LETTER_PHA: char = '𑊜';
    /// \u{1129d}: '𑊝'
    pub const MULTANI_LETTER_BA: char = '𑊝';
    /// \u{1129f}: '𑊟'
    pub const MULTANI_LETTER_BHA: char = '𑊟';
    /// \u{112a0}: '𑊠'
    pub const MULTANI_LETTER_MA: char = '𑊠';
    /// \u{112a1}: '𑊡'
    pub const MULTANI_LETTER_YA: char = '𑊡';
    /// \u{112a2}: '𑊢'
    pub const MULTANI_LETTER_RA: char = '𑊢';
    /// \u{112a3}: '𑊣'
    pub const MULTANI_LETTER_LA: char = '𑊣';
    /// \u{112a4}: '𑊤'
    pub const MULTANI_LETTER_VA: char = '𑊤';
    /// \u{112a5}: '𑊥'
    pub const MULTANI_LETTER_SA: char = '𑊥';
    /// \u{112a6}: '𑊦'
    pub const MULTANI_LETTER_HA: char = '𑊦';
    /// \u{112a7}: '𑊧'
    pub const MULTANI_LETTER_RRA: char = '𑊧';
    /// \u{112a8}: '𑊨'
    pub const MULTANI_LETTER_RHA: char = '𑊨';
    /// \u{112a9}: '𑊩'
    pub const MULTANI_SECTION_MARK: char = '𑊩';
}

/// \u{11280} → \u{112af}\
///\
/// 𑊀 𑊁 𑊂 𑊃 𑊄 𑊅 𑊆 𑊈 𑊊 𑊋 𑊌 𑊍 𑊏 𑊐 𑊑 𑊒
/// 𑊓 𑊔 𑊕 𑊖 𑊗 𑊘 𑊙 𑊚 𑊛 𑊜 𑊝 𑊟 𑊠 𑊡 𑊢 𑊣
/// 𑊤 𑊥 𑊦 𑊧 𑊨 𑊩
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Multani {
    /// \u{11280}: '𑊀'
    MultaniLetterA,
    /// \u{11281}: '𑊁'
    MultaniLetterI,
    /// \u{11282}: '𑊂'
    MultaniLetterU,
    /// \u{11283}: '𑊃'
    MultaniLetterE,
    /// \u{11284}: '𑊄'
    MultaniLetterKa,
    /// \u{11285}: '𑊅'
    MultaniLetterKha,
    /// \u{11286}: '𑊆'
    MultaniLetterGa,
    /// \u{11288}: '𑊈'
    MultaniLetterGha,
    /// \u{1128a}: '𑊊'
    MultaniLetterCa,
    /// \u{1128b}: '𑊋'
    MultaniLetterCha,
    /// \u{1128c}: '𑊌'
    MultaniLetterJa,
    /// \u{1128d}: '𑊍'
    MultaniLetterJja,
    /// \u{1128f}: '𑊏'
    MultaniLetterNya,
    /// \u{11290}: '𑊐'
    MultaniLetterTta,
    /// \u{11291}: '𑊑'
    MultaniLetterTtha,
    /// \u{11292}: '𑊒'
    MultaniLetterDda,
    /// \u{11293}: '𑊓'
    MultaniLetterDdda,
    /// \u{11294}: '𑊔'
    MultaniLetterDdha,
    /// \u{11295}: '𑊕'
    MultaniLetterNna,
    /// \u{11296}: '𑊖'
    MultaniLetterTa,
    /// \u{11297}: '𑊗'
    MultaniLetterTha,
    /// \u{11298}: '𑊘'
    MultaniLetterDa,
    /// \u{11299}: '𑊙'
    MultaniLetterDha,
    /// \u{1129a}: '𑊚'
    MultaniLetterNa,
    /// \u{1129b}: '𑊛'
    MultaniLetterPa,
    /// \u{1129c}: '𑊜'
    MultaniLetterPha,
    /// \u{1129d}: '𑊝'
    MultaniLetterBa,
    /// \u{1129f}: '𑊟'
    MultaniLetterBha,
    /// \u{112a0}: '𑊠'
    MultaniLetterMa,
    /// \u{112a1}: '𑊡'
    MultaniLetterYa,
    /// \u{112a2}: '𑊢'
    MultaniLetterRa,
    /// \u{112a3}: '𑊣'
    MultaniLetterLa,
    /// \u{112a4}: '𑊤'
    MultaniLetterVa,
    /// \u{112a5}: '𑊥'
    MultaniLetterSa,
    /// \u{112a6}: '𑊦'
    MultaniLetterHa,
    /// \u{112a7}: '𑊧'
    MultaniLetterRra,
    /// \u{112a8}: '𑊨'
    MultaniLetterRha,
    /// \u{112a9}: '𑊩'
    MultaniSectionMark,
}

impl Into<char> for Multani {
    fn into(self) -> char {
        use constants::*;
        match self {
            Multani::MultaniLetterA => MULTANI_LETTER_A,
            Multani::MultaniLetterI => MULTANI_LETTER_I,
            Multani::MultaniLetterU => MULTANI_LETTER_U,
            Multani::MultaniLetterE => MULTANI_LETTER_E,
            Multani::MultaniLetterKa => MULTANI_LETTER_KA,
            Multani::MultaniLetterKha => MULTANI_LETTER_KHA,
            Multani::MultaniLetterGa => MULTANI_LETTER_GA,
            Multani::MultaniLetterGha => MULTANI_LETTER_GHA,
            Multani::MultaniLetterCa => MULTANI_LETTER_CA,
            Multani::MultaniLetterCha => MULTANI_LETTER_CHA,
            Multani::MultaniLetterJa => MULTANI_LETTER_JA,
            Multani::MultaniLetterJja => MULTANI_LETTER_JJA,
            Multani::MultaniLetterNya => MULTANI_LETTER_NYA,
            Multani::MultaniLetterTta => MULTANI_LETTER_TTA,
            Multani::MultaniLetterTtha => MULTANI_LETTER_TTHA,
            Multani::MultaniLetterDda => MULTANI_LETTER_DDA,
            Multani::MultaniLetterDdda => MULTANI_LETTER_DDDA,
            Multani::MultaniLetterDdha => MULTANI_LETTER_DDHA,
            Multani::MultaniLetterNna => MULTANI_LETTER_NNA,
            Multani::MultaniLetterTa => MULTANI_LETTER_TA,
            Multani::MultaniLetterTha => MULTANI_LETTER_THA,
            Multani::MultaniLetterDa => MULTANI_LETTER_DA,
            Multani::MultaniLetterDha => MULTANI_LETTER_DHA,
            Multani::MultaniLetterNa => MULTANI_LETTER_NA,
            Multani::MultaniLetterPa => MULTANI_LETTER_PA,
            Multani::MultaniLetterPha => MULTANI_LETTER_PHA,
            Multani::MultaniLetterBa => MULTANI_LETTER_BA,
            Multani::MultaniLetterBha => MULTANI_LETTER_BHA,
            Multani::MultaniLetterMa => MULTANI_LETTER_MA,
            Multani::MultaniLetterYa => MULTANI_LETTER_YA,
            Multani::MultaniLetterRa => MULTANI_LETTER_RA,
            Multani::MultaniLetterLa => MULTANI_LETTER_LA,
            Multani::MultaniLetterVa => MULTANI_LETTER_VA,
            Multani::MultaniLetterSa => MULTANI_LETTER_SA,
            Multani::MultaniLetterHa => MULTANI_LETTER_HA,
            Multani::MultaniLetterRra => MULTANI_LETTER_RRA,
            Multani::MultaniLetterRha => MULTANI_LETTER_RHA,
            Multani::MultaniSectionMark => MULTANI_SECTION_MARK,
        }
    }
}

impl std::convert::TryFrom<char> for Multani {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            MULTANI_LETTER_A => Ok(Multani::MultaniLetterA),
            MULTANI_LETTER_I => Ok(Multani::MultaniLetterI),
            MULTANI_LETTER_U => Ok(Multani::MultaniLetterU),
            MULTANI_LETTER_E => Ok(Multani::MultaniLetterE),
            MULTANI_LETTER_KA => Ok(Multani::MultaniLetterKa),
            MULTANI_LETTER_KHA => Ok(Multani::MultaniLetterKha),
            MULTANI_LETTER_GA => Ok(Multani::MultaniLetterGa),
            MULTANI_LETTER_GHA => Ok(Multani::MultaniLetterGha),
            MULTANI_LETTER_CA => Ok(Multani::MultaniLetterCa),
            MULTANI_LETTER_CHA => Ok(Multani::MultaniLetterCha),
            MULTANI_LETTER_JA => Ok(Multani::MultaniLetterJa),
            MULTANI_LETTER_JJA => Ok(Multani::MultaniLetterJja),
            MULTANI_LETTER_NYA => Ok(Multani::MultaniLetterNya),
            MULTANI_LETTER_TTA => Ok(Multani::MultaniLetterTta),
            MULTANI_LETTER_TTHA => Ok(Multani::MultaniLetterTtha),
            MULTANI_LETTER_DDA => Ok(Multani::MultaniLetterDda),
            MULTANI_LETTER_DDDA => Ok(Multani::MultaniLetterDdda),
            MULTANI_LETTER_DDHA => Ok(Multani::MultaniLetterDdha),
            MULTANI_LETTER_NNA => Ok(Multani::MultaniLetterNna),
            MULTANI_LETTER_TA => Ok(Multani::MultaniLetterTa),
            MULTANI_LETTER_THA => Ok(Multani::MultaniLetterTha),
            MULTANI_LETTER_DA => Ok(Multani::MultaniLetterDa),
            MULTANI_LETTER_DHA => Ok(Multani::MultaniLetterDha),
            MULTANI_LETTER_NA => Ok(Multani::MultaniLetterNa),
            MULTANI_LETTER_PA => Ok(Multani::MultaniLetterPa),
            MULTANI_LETTER_PHA => Ok(Multani::MultaniLetterPha),
            MULTANI_LETTER_BA => Ok(Multani::MultaniLetterBa),
            MULTANI_LETTER_BHA => Ok(Multani::MultaniLetterBha),
            MULTANI_LETTER_MA => Ok(Multani::MultaniLetterMa),
            MULTANI_LETTER_YA => Ok(Multani::MultaniLetterYa),
            MULTANI_LETTER_RA => Ok(Multani::MultaniLetterRa),
            MULTANI_LETTER_LA => Ok(Multani::MultaniLetterLa),
            MULTANI_LETTER_VA => Ok(Multani::MultaniLetterVa),
            MULTANI_LETTER_SA => Ok(Multani::MultaniLetterSa),
            MULTANI_LETTER_HA => Ok(Multani::MultaniLetterHa),
            MULTANI_LETTER_RRA => Ok(Multani::MultaniLetterRra),
            MULTANI_LETTER_RHA => Ok(Multani::MultaniLetterRha),
            MULTANI_SECTION_MARK => Ok(Multani::MultaniSectionMark),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Multani {
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

impl std::convert::TryFrom<u32> for Multani {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Multani {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Multani {
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        Multani::MultaniLetterA
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Multani::MultaniLetterA => "multani letter a",
            Multani::MultaniLetterI => "multani letter i",
            Multani::MultaniLetterU => "multani letter u",
            Multani::MultaniLetterE => "multani letter e",
            Multani::MultaniLetterKa => "multani letter ka",
            Multani::MultaniLetterKha => "multani letter kha",
            Multani::MultaniLetterGa => "multani letter ga",
            Multani::MultaniLetterGha => "multani letter gha",
            Multani::MultaniLetterCa => "multani letter ca",
            Multani::MultaniLetterCha => "multani letter cha",
            Multani::MultaniLetterJa => "multani letter ja",
            Multani::MultaniLetterJja => "multani letter jja",
            Multani::MultaniLetterNya => "multani letter nya",
            Multani::MultaniLetterTta => "multani letter tta",
            Multani::MultaniLetterTtha => "multani letter ttha",
            Multani::MultaniLetterDda => "multani letter dda",
            Multani::MultaniLetterDdda => "multani letter ddda",
            Multani::MultaniLetterDdha => "multani letter ddha",
            Multani::MultaniLetterNna => "multani letter nna",
            Multani::MultaniLetterTa => "multani letter ta",
            Multani::MultaniLetterTha => "multani letter tha",
            Multani::MultaniLetterDa => "multani letter da",
            Multani::MultaniLetterDha => "multani letter dha",
            Multani::MultaniLetterNa => "multani letter na",
            Multani::MultaniLetterPa => "multani letter pa",
            Multani::MultaniLetterPha => "multani letter pha",
            Multani::MultaniLetterBa => "multani letter ba",
            Multani::MultaniLetterBha => "multani letter bha",
            Multani::MultaniLetterMa => "multani letter ma",
            Multani::MultaniLetterYa => "multani letter ya",
            Multani::MultaniLetterRa => "multani letter ra",
            Multani::MultaniLetterLa => "multani letter la",
            Multani::MultaniLetterVa => "multani letter va",
            Multani::MultaniLetterSa => "multani letter sa",
            Multani::MultaniLetterHa => "multani letter ha",
            Multani::MultaniLetterRra => "multani letter rra",
            Multani::MultaniLetterRha => "multani letter rha",
            Multani::MultaniSectionMark => "multani section mark",
        }
    }
}
