/// \u{11280} → \u{112af}\
///\
/// 𑊀 𑊁 𑊂 𑊃 𑊄 𑊅 𑊆 𑊈 𑊊 𑊋 𑊌 𑊍 𑊏 𑊐 𑊑 𑊒
/// 𑊓 𑊔 𑊕 𑊖 𑊗 𑊘 𑊙 𑊚 𑊛 𑊜 𑊝 𑊟 𑊠 𑊡 𑊢 𑊣
/// 𑊤 𑊥 𑊦 𑊧 𑊨 𑊩
pub mod constants {
    /// \u{11280}: '𑊀'
    pub const LETTER_A: char = '𑊀';
    /// \u{11281}: '𑊁'
    pub const LETTER_I: char = '𑊁';
    /// \u{11282}: '𑊂'
    pub const LETTER_U: char = '𑊂';
    /// \u{11283}: '𑊃'
    pub const LETTER_E: char = '𑊃';
    /// \u{11284}: '𑊄'
    pub const LETTER_KA: char = '𑊄';
    /// \u{11285}: '𑊅'
    pub const LETTER_KHA: char = '𑊅';
    /// \u{11286}: '𑊆'
    pub const LETTER_GA: char = '𑊆';
    /// \u{11288}: '𑊈'
    pub const LETTER_GHA: char = '𑊈';
    /// \u{1128a}: '𑊊'
    pub const LETTER_CA: char = '𑊊';
    /// \u{1128b}: '𑊋'
    pub const LETTER_CHA: char = '𑊋';
    /// \u{1128c}: '𑊌'
    pub const LETTER_JA: char = '𑊌';
    /// \u{1128d}: '𑊍'
    pub const LETTER_JJA: char = '𑊍';
    /// \u{1128f}: '𑊏'
    pub const LETTER_NYA: char = '𑊏';
    /// \u{11290}: '𑊐'
    pub const LETTER_TTA: char = '𑊐';
    /// \u{11291}: '𑊑'
    pub const LETTER_TTHA: char = '𑊑';
    /// \u{11292}: '𑊒'
    pub const LETTER_DDA: char = '𑊒';
    /// \u{11293}: '𑊓'
    pub const LETTER_DDDA: char = '𑊓';
    /// \u{11294}: '𑊔'
    pub const LETTER_DDHA: char = '𑊔';
    /// \u{11295}: '𑊕'
    pub const LETTER_NNA: char = '𑊕';
    /// \u{11296}: '𑊖'
    pub const LETTER_TA: char = '𑊖';
    /// \u{11297}: '𑊗'
    pub const LETTER_THA: char = '𑊗';
    /// \u{11298}: '𑊘'
    pub const LETTER_DA: char = '𑊘';
    /// \u{11299}: '𑊙'
    pub const LETTER_DHA: char = '𑊙';
    /// \u{1129a}: '𑊚'
    pub const LETTER_NA: char = '𑊚';
    /// \u{1129b}: '𑊛'
    pub const LETTER_PA: char = '𑊛';
    /// \u{1129c}: '𑊜'
    pub const LETTER_PHA: char = '𑊜';
    /// \u{1129d}: '𑊝'
    pub const LETTER_BA: char = '𑊝';
    /// \u{1129f}: '𑊟'
    pub const LETTER_BHA: char = '𑊟';
    /// \u{112a0}: '𑊠'
    pub const LETTER_MA: char = '𑊠';
    /// \u{112a1}: '𑊡'
    pub const LETTER_YA: char = '𑊡';
    /// \u{112a2}: '𑊢'
    pub const LETTER_RA: char = '𑊢';
    /// \u{112a3}: '𑊣'
    pub const LETTER_LA: char = '𑊣';
    /// \u{112a4}: '𑊤'
    pub const LETTER_VA: char = '𑊤';
    /// \u{112a5}: '𑊥'
    pub const LETTER_SA: char = '𑊥';
    /// \u{112a6}: '𑊦'
    pub const LETTER_HA: char = '𑊦';
    /// \u{112a7}: '𑊧'
    pub const LETTER_RRA: char = '𑊧';
    /// \u{112a8}: '𑊨'
    pub const LETTER_RHA: char = '𑊨';
    /// \u{112a9}: '𑊩'
    pub const SECTION_MARK: char = '𑊩';
}

/// \u{11280} → \u{112af}\
///\
/// 𑊀 𑊁 𑊂 𑊃 𑊄 𑊅 𑊆 𑊈 𑊊 𑊋 𑊌 𑊍 𑊏 𑊐 𑊑 𑊒
/// 𑊓 𑊔 𑊕 𑊖 𑊗 𑊘 𑊙 𑊚 𑊛 𑊜 𑊝 𑊟 𑊠 𑊡 𑊢 𑊣
/// 𑊤 𑊥 𑊦 𑊧 𑊨 𑊩
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Multani {
    /// \u{11280}: '𑊀'
    LetterA,
    /// \u{11281}: '𑊁'
    LetterI,
    /// \u{11282}: '𑊂'
    LetterU,
    /// \u{11283}: '𑊃'
    LetterE,
    /// \u{11284}: '𑊄'
    LetterKa,
    /// \u{11285}: '𑊅'
    LetterKha,
    /// \u{11286}: '𑊆'
    LetterGa,
    /// \u{11288}: '𑊈'
    LetterGha,
    /// \u{1128a}: '𑊊'
    LetterCa,
    /// \u{1128b}: '𑊋'
    LetterCha,
    /// \u{1128c}: '𑊌'
    LetterJa,
    /// \u{1128d}: '𑊍'
    LetterJja,
    /// \u{1128f}: '𑊏'
    LetterNya,
    /// \u{11290}: '𑊐'
    LetterTta,
    /// \u{11291}: '𑊑'
    LetterTtha,
    /// \u{11292}: '𑊒'
    LetterDda,
    /// \u{11293}: '𑊓'
    LetterDdda,
    /// \u{11294}: '𑊔'
    LetterDdha,
    /// \u{11295}: '𑊕'
    LetterNna,
    /// \u{11296}: '𑊖'
    LetterTa,
    /// \u{11297}: '𑊗'
    LetterTha,
    /// \u{11298}: '𑊘'
    LetterDa,
    /// \u{11299}: '𑊙'
    LetterDha,
    /// \u{1129a}: '𑊚'
    LetterNa,
    /// \u{1129b}: '𑊛'
    LetterPa,
    /// \u{1129c}: '𑊜'
    LetterPha,
    /// \u{1129d}: '𑊝'
    LetterBa,
    /// \u{1129f}: '𑊟'
    LetterBha,
    /// \u{112a0}: '𑊠'
    LetterMa,
    /// \u{112a1}: '𑊡'
    LetterYa,
    /// \u{112a2}: '𑊢'
    LetterRa,
    /// \u{112a3}: '𑊣'
    LetterLa,
    /// \u{112a4}: '𑊤'
    LetterVa,
    /// \u{112a5}: '𑊥'
    LetterSa,
    /// \u{112a6}: '𑊦'
    LetterHa,
    /// \u{112a7}: '𑊧'
    LetterRra,
    /// \u{112a8}: '𑊨'
    LetterRha,
    /// \u{112a9}: '𑊩'
    SectionMark,
}

impl Into<char> for Multani {
    fn into(self) -> char {
        use constants::*;
        match self {
            Multani::LetterA => LETTER_A,
            Multani::LetterI => LETTER_I,
            Multani::LetterU => LETTER_U,
            Multani::LetterE => LETTER_E,
            Multani::LetterKa => LETTER_KA,
            Multani::LetterKha => LETTER_KHA,
            Multani::LetterGa => LETTER_GA,
            Multani::LetterGha => LETTER_GHA,
            Multani::LetterCa => LETTER_CA,
            Multani::LetterCha => LETTER_CHA,
            Multani::LetterJa => LETTER_JA,
            Multani::LetterJja => LETTER_JJA,
            Multani::LetterNya => LETTER_NYA,
            Multani::LetterTta => LETTER_TTA,
            Multani::LetterTtha => LETTER_TTHA,
            Multani::LetterDda => LETTER_DDA,
            Multani::LetterDdda => LETTER_DDDA,
            Multani::LetterDdha => LETTER_DDHA,
            Multani::LetterNna => LETTER_NNA,
            Multani::LetterTa => LETTER_TA,
            Multani::LetterTha => LETTER_THA,
            Multani::LetterDa => LETTER_DA,
            Multani::LetterDha => LETTER_DHA,
            Multani::LetterNa => LETTER_NA,
            Multani::LetterPa => LETTER_PA,
            Multani::LetterPha => LETTER_PHA,
            Multani::LetterBa => LETTER_BA,
            Multani::LetterBha => LETTER_BHA,
            Multani::LetterMa => LETTER_MA,
            Multani::LetterYa => LETTER_YA,
            Multani::LetterRa => LETTER_RA,
            Multani::LetterLa => LETTER_LA,
            Multani::LetterVa => LETTER_VA,
            Multani::LetterSa => LETTER_SA,
            Multani::LetterHa => LETTER_HA,
            Multani::LetterRra => LETTER_RRA,
            Multani::LetterRha => LETTER_RHA,
            Multani::SectionMark => SECTION_MARK,
        }
    }
}

impl std::convert::TryFrom<char> for Multani {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_A => Ok(Multani::LetterA),
            LETTER_I => Ok(Multani::LetterI),
            LETTER_U => Ok(Multani::LetterU),
            LETTER_E => Ok(Multani::LetterE),
            LETTER_KA => Ok(Multani::LetterKa),
            LETTER_KHA => Ok(Multani::LetterKha),
            LETTER_GA => Ok(Multani::LetterGa),
            LETTER_GHA => Ok(Multani::LetterGha),
            LETTER_CA => Ok(Multani::LetterCa),
            LETTER_CHA => Ok(Multani::LetterCha),
            LETTER_JA => Ok(Multani::LetterJa),
            LETTER_JJA => Ok(Multani::LetterJja),
            LETTER_NYA => Ok(Multani::LetterNya),
            LETTER_TTA => Ok(Multani::LetterTta),
            LETTER_TTHA => Ok(Multani::LetterTtha),
            LETTER_DDA => Ok(Multani::LetterDda),
            LETTER_DDDA => Ok(Multani::LetterDdda),
            LETTER_DDHA => Ok(Multani::LetterDdha),
            LETTER_NNA => Ok(Multani::LetterNna),
            LETTER_TA => Ok(Multani::LetterTa),
            LETTER_THA => Ok(Multani::LetterTha),
            LETTER_DA => Ok(Multani::LetterDa),
            LETTER_DHA => Ok(Multani::LetterDha),
            LETTER_NA => Ok(Multani::LetterNa),
            LETTER_PA => Ok(Multani::LetterPa),
            LETTER_PHA => Ok(Multani::LetterPha),
            LETTER_BA => Ok(Multani::LetterBa),
            LETTER_BHA => Ok(Multani::LetterBha),
            LETTER_MA => Ok(Multani::LetterMa),
            LETTER_YA => Ok(Multani::LetterYa),
            LETTER_RA => Ok(Multani::LetterRa),
            LETTER_LA => Ok(Multani::LetterLa),
            LETTER_VA => Ok(Multani::LetterVa),
            LETTER_SA => Ok(Multani::LetterSa),
            LETTER_HA => Ok(Multani::LetterHa),
            LETTER_RRA => Ok(Multani::LetterRra),
            LETTER_RHA => Ok(Multani::LetterRha),
            SECTION_MARK => Ok(Multani::SectionMark),
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
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Multani::LetterA
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Multani::LetterA => "multani letter a",
            Multani::LetterI => "multani letter i",
            Multani::LetterU => "multani letter u",
            Multani::LetterE => "multani letter e",
            Multani::LetterKa => "multani letter ka",
            Multani::LetterKha => "multani letter kha",
            Multani::LetterGa => "multani letter ga",
            Multani::LetterGha => "multani letter gha",
            Multani::LetterCa => "multani letter ca",
            Multani::LetterCha => "multani letter cha",
            Multani::LetterJa => "multani letter ja",
            Multani::LetterJja => "multani letter jja",
            Multani::LetterNya => "multani letter nya",
            Multani::LetterTta => "multani letter tta",
            Multani::LetterTtha => "multani letter ttha",
            Multani::LetterDda => "multani letter dda",
            Multani::LetterDdda => "multani letter ddda",
            Multani::LetterDdha => "multani letter ddha",
            Multani::LetterNna => "multani letter nna",
            Multani::LetterTa => "multani letter ta",
            Multani::LetterTha => "multani letter tha",
            Multani::LetterDa => "multani letter da",
            Multani::LetterDha => "multani letter dha",
            Multani::LetterNa => "multani letter na",
            Multani::LetterPa => "multani letter pa",
            Multani::LetterPha => "multani letter pha",
            Multani::LetterBa => "multani letter ba",
            Multani::LetterBha => "multani letter bha",
            Multani::LetterMa => "multani letter ma",
            Multani::LetterYa => "multani letter ya",
            Multani::LetterRa => "multani letter ra",
            Multani::LetterLa => "multani letter la",
            Multani::LetterVa => "multani letter va",
            Multani::LetterSa => "multani letter sa",
            Multani::LetterHa => "multani letter ha",
            Multani::LetterRra => "multani letter rra",
            Multani::LetterRha => "multani letter rha",
            Multani::SectionMark => "multani section mark",
        }
    }
}
