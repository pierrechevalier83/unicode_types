/// \u{10300} → \u{1032f}\
///\
/// 𐌀 𐌁 𐌂 𐌃 𐌄 𐌅 𐌆 𐌇 𐌈 𐌉 𐌊 𐌋 𐌌 𐌍 𐌎 𐌏
/// 𐌐 𐌑 𐌒 𐌓 𐌔 𐌕 𐌖 𐌗 𐌘 𐌙 𐌚 𐌛 𐌜 𐌝 𐌞 𐌟
/// 𐌠 𐌡 𐌢 𐌣 𐌭 𐌮
pub mod constants {
    /// \u{10300}: '𐌀'
    pub const OLD_ITALIC_LETTER_A: char = '𐌀';
    /// \u{10301}: '𐌁'
    pub const OLD_ITALIC_LETTER_BE: char = '𐌁';
    /// \u{10302}: '𐌂'
    pub const OLD_ITALIC_LETTER_KE: char = '𐌂';
    /// \u{10303}: '𐌃'
    pub const OLD_ITALIC_LETTER_DE: char = '𐌃';
    /// \u{10304}: '𐌄'
    pub const OLD_ITALIC_LETTER_E: char = '𐌄';
    /// \u{10305}: '𐌅'
    pub const OLD_ITALIC_LETTER_VE: char = '𐌅';
    /// \u{10306}: '𐌆'
    pub const OLD_ITALIC_LETTER_ZE: char = '𐌆';
    /// \u{10307}: '𐌇'
    pub const OLD_ITALIC_LETTER_HE: char = '𐌇';
    /// \u{10308}: '𐌈'
    pub const OLD_ITALIC_LETTER_THE: char = '𐌈';
    /// \u{10309}: '𐌉'
    pub const OLD_ITALIC_LETTER_I: char = '𐌉';
    /// \u{1030a}: '𐌊'
    pub const OLD_ITALIC_LETTER_KA: char = '𐌊';
    /// \u{1030b}: '𐌋'
    pub const OLD_ITALIC_LETTER_EL: char = '𐌋';
    /// \u{1030c}: '𐌌'
    pub const OLD_ITALIC_LETTER_EM: char = '𐌌';
    /// \u{1030d}: '𐌍'
    pub const OLD_ITALIC_LETTER_EN: char = '𐌍';
    /// \u{1030e}: '𐌎'
    pub const OLD_ITALIC_LETTER_ESH: char = '𐌎';
    /// \u{1030f}: '𐌏'
    pub const OLD_ITALIC_LETTER_O: char = '𐌏';
    /// \u{10310}: '𐌐'
    pub const OLD_ITALIC_LETTER_PE: char = '𐌐';
    /// \u{10311}: '𐌑'
    pub const OLD_ITALIC_LETTER_SHE: char = '𐌑';
    /// \u{10312}: '𐌒'
    pub const OLD_ITALIC_LETTER_KU: char = '𐌒';
    /// \u{10313}: '𐌓'
    pub const OLD_ITALIC_LETTER_ER: char = '𐌓';
    /// \u{10314}: '𐌔'
    pub const OLD_ITALIC_LETTER_ES: char = '𐌔';
    /// \u{10315}: '𐌕'
    pub const OLD_ITALIC_LETTER_TE: char = '𐌕';
    /// \u{10316}: '𐌖'
    pub const OLD_ITALIC_LETTER_U: char = '𐌖';
    /// \u{10317}: '𐌗'
    pub const OLD_ITALIC_LETTER_EKS: char = '𐌗';
    /// \u{10318}: '𐌘'
    pub const OLD_ITALIC_LETTER_PHE: char = '𐌘';
    /// \u{10319}: '𐌙'
    pub const OLD_ITALIC_LETTER_KHE: char = '𐌙';
    /// \u{1031a}: '𐌚'
    pub const OLD_ITALIC_LETTER_EF: char = '𐌚';
    /// \u{1031b}: '𐌛'
    pub const OLD_ITALIC_LETTER_ERS: char = '𐌛';
    /// \u{1031c}: '𐌜'
    pub const OLD_ITALIC_LETTER_CHE: char = '𐌜';
    /// \u{1031d}: '𐌝'
    pub const OLD_ITALIC_LETTER_II: char = '𐌝';
    /// \u{1031e}: '𐌞'
    pub const OLD_ITALIC_LETTER_UU: char = '𐌞';
    /// \u{1031f}: '𐌟'
    pub const OLD_ITALIC_LETTER_ESS: char = '𐌟';
    /// \u{10320}: '𐌠'
    pub const OLD_ITALIC_NUMERAL_ONE: char = '𐌠';
    /// \u{10321}: '𐌡'
    pub const OLD_ITALIC_NUMERAL_FIVE: char = '𐌡';
    /// \u{10322}: '𐌢'
    pub const OLD_ITALIC_NUMERAL_TEN: char = '𐌢';
    /// \u{10323}: '𐌣'
    pub const OLD_ITALIC_NUMERAL_FIFTY: char = '𐌣';
    /// \u{1032d}: '𐌭'
    pub const OLD_ITALIC_LETTER_YE: char = '𐌭';
    /// \u{1032e}: '𐌮'
    pub const OLD_ITALIC_LETTER_NORTHERN_TSE: char = '𐌮';
}

/// \u{10300} → \u{1032f}\
///\
/// 𐌀 𐌁 𐌂 𐌃 𐌄 𐌅 𐌆 𐌇 𐌈 𐌉 𐌊 𐌋 𐌌 𐌍 𐌎 𐌏
/// 𐌐 𐌑 𐌒 𐌓 𐌔 𐌕 𐌖 𐌗 𐌘 𐌙 𐌚 𐌛 𐌜 𐌝 𐌞 𐌟
/// 𐌠 𐌡 𐌢 𐌣 𐌭 𐌮
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum OldItalic {
    /// \u{10300}: '𐌀'
    OldItalicLetterA,
    /// \u{10301}: '𐌁'
    OldItalicLetterBe,
    /// \u{10302}: '𐌂'
    OldItalicLetterKe,
    /// \u{10303}: '𐌃'
    OldItalicLetterDe,
    /// \u{10304}: '𐌄'
    OldItalicLetterE,
    /// \u{10305}: '𐌅'
    OldItalicLetterVe,
    /// \u{10306}: '𐌆'
    OldItalicLetterZe,
    /// \u{10307}: '𐌇'
    OldItalicLetterHe,
    /// \u{10308}: '𐌈'
    OldItalicLetterThe,
    /// \u{10309}: '𐌉'
    OldItalicLetterI,
    /// \u{1030a}: '𐌊'
    OldItalicLetterKa,
    /// \u{1030b}: '𐌋'
    OldItalicLetterEl,
    /// \u{1030c}: '𐌌'
    OldItalicLetterEm,
    /// \u{1030d}: '𐌍'
    OldItalicLetterEn,
    /// \u{1030e}: '𐌎'
    OldItalicLetterEsh,
    /// \u{1030f}: '𐌏'
    OldItalicLetterO,
    /// \u{10310}: '𐌐'
    OldItalicLetterPe,
    /// \u{10311}: '𐌑'
    OldItalicLetterShe,
    /// \u{10312}: '𐌒'
    OldItalicLetterKu,
    /// \u{10313}: '𐌓'
    OldItalicLetterEr,
    /// \u{10314}: '𐌔'
    OldItalicLetterEs,
    /// \u{10315}: '𐌕'
    OldItalicLetterTe,
    /// \u{10316}: '𐌖'
    OldItalicLetterU,
    /// \u{10317}: '𐌗'
    OldItalicLetterEks,
    /// \u{10318}: '𐌘'
    OldItalicLetterPhe,
    /// \u{10319}: '𐌙'
    OldItalicLetterKhe,
    /// \u{1031a}: '𐌚'
    OldItalicLetterEf,
    /// \u{1031b}: '𐌛'
    OldItalicLetterErs,
    /// \u{1031c}: '𐌜'
    OldItalicLetterChe,
    /// \u{1031d}: '𐌝'
    OldItalicLetterIi,
    /// \u{1031e}: '𐌞'
    OldItalicLetterUu,
    /// \u{1031f}: '𐌟'
    OldItalicLetterEss,
    /// \u{10320}: '𐌠'
    OldItalicNumeralOne,
    /// \u{10321}: '𐌡'
    OldItalicNumeralFive,
    /// \u{10322}: '𐌢'
    OldItalicNumeralTen,
    /// \u{10323}: '𐌣'
    OldItalicNumeralFifty,
    /// \u{1032d}: '𐌭'
    OldItalicLetterYe,
    /// \u{1032e}: '𐌮'
    OldItalicLetterNorthernTse,
}

impl Into<char> for OldItalic {
    fn into(self) -> char {
        use constants::*;
        match self {
            OldItalic::OldItalicLetterA => OLD_ITALIC_LETTER_A,
            OldItalic::OldItalicLetterBe => OLD_ITALIC_LETTER_BE,
            OldItalic::OldItalicLetterKe => OLD_ITALIC_LETTER_KE,
            OldItalic::OldItalicLetterDe => OLD_ITALIC_LETTER_DE,
            OldItalic::OldItalicLetterE => OLD_ITALIC_LETTER_E,
            OldItalic::OldItalicLetterVe => OLD_ITALIC_LETTER_VE,
            OldItalic::OldItalicLetterZe => OLD_ITALIC_LETTER_ZE,
            OldItalic::OldItalicLetterHe => OLD_ITALIC_LETTER_HE,
            OldItalic::OldItalicLetterThe => OLD_ITALIC_LETTER_THE,
            OldItalic::OldItalicLetterI => OLD_ITALIC_LETTER_I,
            OldItalic::OldItalicLetterKa => OLD_ITALIC_LETTER_KA,
            OldItalic::OldItalicLetterEl => OLD_ITALIC_LETTER_EL,
            OldItalic::OldItalicLetterEm => OLD_ITALIC_LETTER_EM,
            OldItalic::OldItalicLetterEn => OLD_ITALIC_LETTER_EN,
            OldItalic::OldItalicLetterEsh => OLD_ITALIC_LETTER_ESH,
            OldItalic::OldItalicLetterO => OLD_ITALIC_LETTER_O,
            OldItalic::OldItalicLetterPe => OLD_ITALIC_LETTER_PE,
            OldItalic::OldItalicLetterShe => OLD_ITALIC_LETTER_SHE,
            OldItalic::OldItalicLetterKu => OLD_ITALIC_LETTER_KU,
            OldItalic::OldItalicLetterEr => OLD_ITALIC_LETTER_ER,
            OldItalic::OldItalicLetterEs => OLD_ITALIC_LETTER_ES,
            OldItalic::OldItalicLetterTe => OLD_ITALIC_LETTER_TE,
            OldItalic::OldItalicLetterU => OLD_ITALIC_LETTER_U,
            OldItalic::OldItalicLetterEks => OLD_ITALIC_LETTER_EKS,
            OldItalic::OldItalicLetterPhe => OLD_ITALIC_LETTER_PHE,
            OldItalic::OldItalicLetterKhe => OLD_ITALIC_LETTER_KHE,
            OldItalic::OldItalicLetterEf => OLD_ITALIC_LETTER_EF,
            OldItalic::OldItalicLetterErs => OLD_ITALIC_LETTER_ERS,
            OldItalic::OldItalicLetterChe => OLD_ITALIC_LETTER_CHE,
            OldItalic::OldItalicLetterIi => OLD_ITALIC_LETTER_II,
            OldItalic::OldItalicLetterUu => OLD_ITALIC_LETTER_UU,
            OldItalic::OldItalicLetterEss => OLD_ITALIC_LETTER_ESS,
            OldItalic::OldItalicNumeralOne => OLD_ITALIC_NUMERAL_ONE,
            OldItalic::OldItalicNumeralFive => OLD_ITALIC_NUMERAL_FIVE,
            OldItalic::OldItalicNumeralTen => OLD_ITALIC_NUMERAL_TEN,
            OldItalic::OldItalicNumeralFifty => OLD_ITALIC_NUMERAL_FIFTY,
            OldItalic::OldItalicLetterYe => OLD_ITALIC_LETTER_YE,
            OldItalic::OldItalicLetterNorthernTse => OLD_ITALIC_LETTER_NORTHERN_TSE,
        }
    }
}

impl std::convert::TryFrom<char> for OldItalic {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            OLD_ITALIC_LETTER_A => Ok(OldItalic::OldItalicLetterA),
            OLD_ITALIC_LETTER_BE => Ok(OldItalic::OldItalicLetterBe),
            OLD_ITALIC_LETTER_KE => Ok(OldItalic::OldItalicLetterKe),
            OLD_ITALIC_LETTER_DE => Ok(OldItalic::OldItalicLetterDe),
            OLD_ITALIC_LETTER_E => Ok(OldItalic::OldItalicLetterE),
            OLD_ITALIC_LETTER_VE => Ok(OldItalic::OldItalicLetterVe),
            OLD_ITALIC_LETTER_ZE => Ok(OldItalic::OldItalicLetterZe),
            OLD_ITALIC_LETTER_HE => Ok(OldItalic::OldItalicLetterHe),
            OLD_ITALIC_LETTER_THE => Ok(OldItalic::OldItalicLetterThe),
            OLD_ITALIC_LETTER_I => Ok(OldItalic::OldItalicLetterI),
            OLD_ITALIC_LETTER_KA => Ok(OldItalic::OldItalicLetterKa),
            OLD_ITALIC_LETTER_EL => Ok(OldItalic::OldItalicLetterEl),
            OLD_ITALIC_LETTER_EM => Ok(OldItalic::OldItalicLetterEm),
            OLD_ITALIC_LETTER_EN => Ok(OldItalic::OldItalicLetterEn),
            OLD_ITALIC_LETTER_ESH => Ok(OldItalic::OldItalicLetterEsh),
            OLD_ITALIC_LETTER_O => Ok(OldItalic::OldItalicLetterO),
            OLD_ITALIC_LETTER_PE => Ok(OldItalic::OldItalicLetterPe),
            OLD_ITALIC_LETTER_SHE => Ok(OldItalic::OldItalicLetterShe),
            OLD_ITALIC_LETTER_KU => Ok(OldItalic::OldItalicLetterKu),
            OLD_ITALIC_LETTER_ER => Ok(OldItalic::OldItalicLetterEr),
            OLD_ITALIC_LETTER_ES => Ok(OldItalic::OldItalicLetterEs),
            OLD_ITALIC_LETTER_TE => Ok(OldItalic::OldItalicLetterTe),
            OLD_ITALIC_LETTER_U => Ok(OldItalic::OldItalicLetterU),
            OLD_ITALIC_LETTER_EKS => Ok(OldItalic::OldItalicLetterEks),
            OLD_ITALIC_LETTER_PHE => Ok(OldItalic::OldItalicLetterPhe),
            OLD_ITALIC_LETTER_KHE => Ok(OldItalic::OldItalicLetterKhe),
            OLD_ITALIC_LETTER_EF => Ok(OldItalic::OldItalicLetterEf),
            OLD_ITALIC_LETTER_ERS => Ok(OldItalic::OldItalicLetterErs),
            OLD_ITALIC_LETTER_CHE => Ok(OldItalic::OldItalicLetterChe),
            OLD_ITALIC_LETTER_II => Ok(OldItalic::OldItalicLetterIi),
            OLD_ITALIC_LETTER_UU => Ok(OldItalic::OldItalicLetterUu),
            OLD_ITALIC_LETTER_ESS => Ok(OldItalic::OldItalicLetterEss),
            OLD_ITALIC_NUMERAL_ONE => Ok(OldItalic::OldItalicNumeralOne),
            OLD_ITALIC_NUMERAL_FIVE => Ok(OldItalic::OldItalicNumeralFive),
            OLD_ITALIC_NUMERAL_TEN => Ok(OldItalic::OldItalicNumeralTen),
            OLD_ITALIC_NUMERAL_FIFTY => Ok(OldItalic::OldItalicNumeralFifty),
            OLD_ITALIC_LETTER_YE => Ok(OldItalic::OldItalicLetterYe),
            OLD_ITALIC_LETTER_NORTHERN_TSE => Ok(OldItalic::OldItalicLetterNorthernTse),
            _ => Err(()),
        }
    }
}

impl Into<u32> for OldItalic {
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

impl std::convert::TryFrom<u32> for OldItalic {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for OldItalic {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl OldItalic {
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        OldItalic::OldItalicLetterA
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            OldItalic::OldItalicLetterA => "old italic letter a",
            OldItalic::OldItalicLetterBe => "old italic letter be",
            OldItalic::OldItalicLetterKe => "old italic letter ke",
            OldItalic::OldItalicLetterDe => "old italic letter de",
            OldItalic::OldItalicLetterE => "old italic letter e",
            OldItalic::OldItalicLetterVe => "old italic letter ve",
            OldItalic::OldItalicLetterZe => "old italic letter ze",
            OldItalic::OldItalicLetterHe => "old italic letter he",
            OldItalic::OldItalicLetterThe => "old italic letter the",
            OldItalic::OldItalicLetterI => "old italic letter i",
            OldItalic::OldItalicLetterKa => "old italic letter ka",
            OldItalic::OldItalicLetterEl => "old italic letter el",
            OldItalic::OldItalicLetterEm => "old italic letter em",
            OldItalic::OldItalicLetterEn => "old italic letter en",
            OldItalic::OldItalicLetterEsh => "old italic letter esh",
            OldItalic::OldItalicLetterO => "old italic letter o",
            OldItalic::OldItalicLetterPe => "old italic letter pe",
            OldItalic::OldItalicLetterShe => "old italic letter she",
            OldItalic::OldItalicLetterKu => "old italic letter ku",
            OldItalic::OldItalicLetterEr => "old italic letter er",
            OldItalic::OldItalicLetterEs => "old italic letter es",
            OldItalic::OldItalicLetterTe => "old italic letter te",
            OldItalic::OldItalicLetterU => "old italic letter u",
            OldItalic::OldItalicLetterEks => "old italic letter eks",
            OldItalic::OldItalicLetterPhe => "old italic letter phe",
            OldItalic::OldItalicLetterKhe => "old italic letter khe",
            OldItalic::OldItalicLetterEf => "old italic letter ef",
            OldItalic::OldItalicLetterErs => "old italic letter ers",
            OldItalic::OldItalicLetterChe => "old italic letter che",
            OldItalic::OldItalicLetterIi => "old italic letter ii",
            OldItalic::OldItalicLetterUu => "old italic letter uu",
            OldItalic::OldItalicLetterEss => "old italic letter ess",
            OldItalic::OldItalicNumeralOne => "old italic numeral one",
            OldItalic::OldItalicNumeralFive => "old italic numeral five",
            OldItalic::OldItalicNumeralTen => "old italic numeral ten",
            OldItalic::OldItalicNumeralFifty => "old italic numeral fifty",
            OldItalic::OldItalicLetterYe => "old italic letter ye",
            OldItalic::OldItalicLetterNorthernTse => "old italic letter northern tse",
        }
    }
}
