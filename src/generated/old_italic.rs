/// \u{10300} → \u{1032f}\
///\
/// 𐌀 𐌁 𐌂 𐌃 𐌄 𐌅 𐌆 𐌇 𐌈 𐌉 𐌊 𐌋 𐌌 𐌍 𐌎 𐌏\
/// 𐌐 𐌑 𐌒 𐌓 𐌔 𐌕 𐌖 𐌗 𐌘 𐌙 𐌚 𐌛 𐌜 𐌝 𐌞 𐌟\
/// 𐌠 𐌡 𐌢 𐌣 𐌭 𐌮\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{10300}: '𐌀'
    pub const LETTER_A: char = '𐌀';
    /// \u{10301}: '𐌁'
    pub const LETTER_BE: char = '𐌁';
    /// \u{10302}: '𐌂'
    pub const LETTER_KE: char = '𐌂';
    /// \u{10303}: '𐌃'
    pub const LETTER_DE: char = '𐌃';
    /// \u{10304}: '𐌄'
    pub const LETTER_E: char = '𐌄';
    /// \u{10305}: '𐌅'
    pub const LETTER_VE: char = '𐌅';
    /// \u{10306}: '𐌆'
    pub const LETTER_ZE: char = '𐌆';
    /// \u{10307}: '𐌇'
    pub const LETTER_HE: char = '𐌇';
    /// \u{10308}: '𐌈'
    pub const LETTER_THE: char = '𐌈';
    /// \u{10309}: '𐌉'
    pub const LETTER_I: char = '𐌉';
    /// \u{1030a}: '𐌊'
    pub const LETTER_KA: char = '𐌊';
    /// \u{1030b}: '𐌋'
    pub const LETTER_EL: char = '𐌋';
    /// \u{1030c}: '𐌌'
    pub const LETTER_EM: char = '𐌌';
    /// \u{1030d}: '𐌍'
    pub const LETTER_EN: char = '𐌍';
    /// \u{1030e}: '𐌎'
    pub const LETTER_ESH: char = '𐌎';
    /// \u{1030f}: '𐌏'
    pub const LETTER_O: char = '𐌏';
    /// \u{10310}: '𐌐'
    pub const LETTER_PE: char = '𐌐';
    /// \u{10311}: '𐌑'
    pub const LETTER_SHE: char = '𐌑';
    /// \u{10312}: '𐌒'
    pub const LETTER_KU: char = '𐌒';
    /// \u{10313}: '𐌓'
    pub const LETTER_ER: char = '𐌓';
    /// \u{10314}: '𐌔'
    pub const LETTER_ES: char = '𐌔';
    /// \u{10315}: '𐌕'
    pub const LETTER_TE: char = '𐌕';
    /// \u{10316}: '𐌖'
    pub const LETTER_U: char = '𐌖';
    /// \u{10317}: '𐌗'
    pub const LETTER_EKS: char = '𐌗';
    /// \u{10318}: '𐌘'
    pub const LETTER_PHE: char = '𐌘';
    /// \u{10319}: '𐌙'
    pub const LETTER_KHE: char = '𐌙';
    /// \u{1031a}: '𐌚'
    pub const LETTER_EF: char = '𐌚';
    /// \u{1031b}: '𐌛'
    pub const LETTER_ERS: char = '𐌛';
    /// \u{1031c}: '𐌜'
    pub const LETTER_CHE: char = '𐌜';
    /// \u{1031d}: '𐌝'
    pub const LETTER_II: char = '𐌝';
    /// \u{1031e}: '𐌞'
    pub const LETTER_UU: char = '𐌞';
    /// \u{1031f}: '𐌟'
    pub const LETTER_ESS: char = '𐌟';
    /// \u{10320}: '𐌠'
    pub const NUMERAL_ONE: char = '𐌠';
    /// \u{10321}: '𐌡'
    pub const NUMERAL_FIVE: char = '𐌡';
    /// \u{10322}: '𐌢'
    pub const NUMERAL_TEN: char = '𐌢';
    /// \u{10323}: '𐌣'
    pub const NUMERAL_FIFTY: char = '𐌣';
    /// \u{1032d}: '𐌭'
    pub const LETTER_YE: char = '𐌭';
    /// \u{1032e}: '𐌮'
    pub const LETTER_NORTHERN_TSE: char = '𐌮';
}

/// An enum to represent all characters in the OldItalic block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum OldItalic {
    /// \u{10300}: '𐌀'
    LetterA,
    /// \u{10301}: '𐌁'
    LetterBe,
    /// \u{10302}: '𐌂'
    LetterKe,
    /// \u{10303}: '𐌃'
    LetterDe,
    /// \u{10304}: '𐌄'
    LetterE,
    /// \u{10305}: '𐌅'
    LetterVe,
    /// \u{10306}: '𐌆'
    LetterZe,
    /// \u{10307}: '𐌇'
    LetterHe,
    /// \u{10308}: '𐌈'
    LetterThe,
    /// \u{10309}: '𐌉'
    LetterI,
    /// \u{1030a}: '𐌊'
    LetterKa,
    /// \u{1030b}: '𐌋'
    LetterEl,
    /// \u{1030c}: '𐌌'
    LetterEm,
    /// \u{1030d}: '𐌍'
    LetterEn,
    /// \u{1030e}: '𐌎'
    LetterEsh,
    /// \u{1030f}: '𐌏'
    LetterO,
    /// \u{10310}: '𐌐'
    LetterPe,
    /// \u{10311}: '𐌑'
    LetterShe,
    /// \u{10312}: '𐌒'
    LetterKu,
    /// \u{10313}: '𐌓'
    LetterEr,
    /// \u{10314}: '𐌔'
    LetterEs,
    /// \u{10315}: '𐌕'
    LetterTe,
    /// \u{10316}: '𐌖'
    LetterU,
    /// \u{10317}: '𐌗'
    LetterEks,
    /// \u{10318}: '𐌘'
    LetterPhe,
    /// \u{10319}: '𐌙'
    LetterKhe,
    /// \u{1031a}: '𐌚'
    LetterEf,
    /// \u{1031b}: '𐌛'
    LetterErs,
    /// \u{1031c}: '𐌜'
    LetterChe,
    /// \u{1031d}: '𐌝'
    LetterIi,
    /// \u{1031e}: '𐌞'
    LetterUu,
    /// \u{1031f}: '𐌟'
    LetterEss,
    /// \u{10320}: '𐌠'
    NumeralOne,
    /// \u{10321}: '𐌡'
    NumeralFive,
    /// \u{10322}: '𐌢'
    NumeralTen,
    /// \u{10323}: '𐌣'
    NumeralFifty,
    /// \u{1032d}: '𐌭'
    LetterYe,
    /// \u{1032e}: '𐌮'
    LetterNorthernTse,
}

impl Into<char> for OldItalic {
    fn into(self) -> char {
        use constants::*;
        match self {
            OldItalic::LetterA => LETTER_A,
            OldItalic::LetterBe => LETTER_BE,
            OldItalic::LetterKe => LETTER_KE,
            OldItalic::LetterDe => LETTER_DE,
            OldItalic::LetterE => LETTER_E,
            OldItalic::LetterVe => LETTER_VE,
            OldItalic::LetterZe => LETTER_ZE,
            OldItalic::LetterHe => LETTER_HE,
            OldItalic::LetterThe => LETTER_THE,
            OldItalic::LetterI => LETTER_I,
            OldItalic::LetterKa => LETTER_KA,
            OldItalic::LetterEl => LETTER_EL,
            OldItalic::LetterEm => LETTER_EM,
            OldItalic::LetterEn => LETTER_EN,
            OldItalic::LetterEsh => LETTER_ESH,
            OldItalic::LetterO => LETTER_O,
            OldItalic::LetterPe => LETTER_PE,
            OldItalic::LetterShe => LETTER_SHE,
            OldItalic::LetterKu => LETTER_KU,
            OldItalic::LetterEr => LETTER_ER,
            OldItalic::LetterEs => LETTER_ES,
            OldItalic::LetterTe => LETTER_TE,
            OldItalic::LetterU => LETTER_U,
            OldItalic::LetterEks => LETTER_EKS,
            OldItalic::LetterPhe => LETTER_PHE,
            OldItalic::LetterKhe => LETTER_KHE,
            OldItalic::LetterEf => LETTER_EF,
            OldItalic::LetterErs => LETTER_ERS,
            OldItalic::LetterChe => LETTER_CHE,
            OldItalic::LetterIi => LETTER_II,
            OldItalic::LetterUu => LETTER_UU,
            OldItalic::LetterEss => LETTER_ESS,
            OldItalic::NumeralOne => NUMERAL_ONE,
            OldItalic::NumeralFive => NUMERAL_FIVE,
            OldItalic::NumeralTen => NUMERAL_TEN,
            OldItalic::NumeralFifty => NUMERAL_FIFTY,
            OldItalic::LetterYe => LETTER_YE,
            OldItalic::LetterNorthernTse => LETTER_NORTHERN_TSE,
        }
    }
}

impl std::convert::TryFrom<char> for OldItalic {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_A => Ok(OldItalic::LetterA),
            LETTER_BE => Ok(OldItalic::LetterBe),
            LETTER_KE => Ok(OldItalic::LetterKe),
            LETTER_DE => Ok(OldItalic::LetterDe),
            LETTER_E => Ok(OldItalic::LetterE),
            LETTER_VE => Ok(OldItalic::LetterVe),
            LETTER_ZE => Ok(OldItalic::LetterZe),
            LETTER_HE => Ok(OldItalic::LetterHe),
            LETTER_THE => Ok(OldItalic::LetterThe),
            LETTER_I => Ok(OldItalic::LetterI),
            LETTER_KA => Ok(OldItalic::LetterKa),
            LETTER_EL => Ok(OldItalic::LetterEl),
            LETTER_EM => Ok(OldItalic::LetterEm),
            LETTER_EN => Ok(OldItalic::LetterEn),
            LETTER_ESH => Ok(OldItalic::LetterEsh),
            LETTER_O => Ok(OldItalic::LetterO),
            LETTER_PE => Ok(OldItalic::LetterPe),
            LETTER_SHE => Ok(OldItalic::LetterShe),
            LETTER_KU => Ok(OldItalic::LetterKu),
            LETTER_ER => Ok(OldItalic::LetterEr),
            LETTER_ES => Ok(OldItalic::LetterEs),
            LETTER_TE => Ok(OldItalic::LetterTe),
            LETTER_U => Ok(OldItalic::LetterU),
            LETTER_EKS => Ok(OldItalic::LetterEks),
            LETTER_PHE => Ok(OldItalic::LetterPhe),
            LETTER_KHE => Ok(OldItalic::LetterKhe),
            LETTER_EF => Ok(OldItalic::LetterEf),
            LETTER_ERS => Ok(OldItalic::LetterErs),
            LETTER_CHE => Ok(OldItalic::LetterChe),
            LETTER_II => Ok(OldItalic::LetterIi),
            LETTER_UU => Ok(OldItalic::LetterUu),
            LETTER_ESS => Ok(OldItalic::LetterEss),
            NUMERAL_ONE => Ok(OldItalic::NumeralOne),
            NUMERAL_FIVE => Ok(OldItalic::NumeralFive),
            NUMERAL_TEN => Ok(OldItalic::NumeralTen),
            NUMERAL_FIFTY => Ok(OldItalic::NumeralFifty),
            LETTER_YE => Ok(OldItalic::LetterYe),
            LETTER_NORTHERN_TSE => Ok(OldItalic::LetterNorthernTse),
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
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        OldItalic::LetterA
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            OldItalic::LetterA => "old italic letter a",
            OldItalic::LetterBe => "old italic letter be",
            OldItalic::LetterKe => "old italic letter ke",
            OldItalic::LetterDe => "old italic letter de",
            OldItalic::LetterE => "old italic letter e",
            OldItalic::LetterVe => "old italic letter ve",
            OldItalic::LetterZe => "old italic letter ze",
            OldItalic::LetterHe => "old italic letter he",
            OldItalic::LetterThe => "old italic letter the",
            OldItalic::LetterI => "old italic letter i",
            OldItalic::LetterKa => "old italic letter ka",
            OldItalic::LetterEl => "old italic letter el",
            OldItalic::LetterEm => "old italic letter em",
            OldItalic::LetterEn => "old italic letter en",
            OldItalic::LetterEsh => "old italic letter esh",
            OldItalic::LetterO => "old italic letter o",
            OldItalic::LetterPe => "old italic letter pe",
            OldItalic::LetterShe => "old italic letter she",
            OldItalic::LetterKu => "old italic letter ku",
            OldItalic::LetterEr => "old italic letter er",
            OldItalic::LetterEs => "old italic letter es",
            OldItalic::LetterTe => "old italic letter te",
            OldItalic::LetterU => "old italic letter u",
            OldItalic::LetterEks => "old italic letter eks",
            OldItalic::LetterPhe => "old italic letter phe",
            OldItalic::LetterKhe => "old italic letter khe",
            OldItalic::LetterEf => "old italic letter ef",
            OldItalic::LetterErs => "old italic letter ers",
            OldItalic::LetterChe => "old italic letter che",
            OldItalic::LetterIi => "old italic letter ii",
            OldItalic::LetterUu => "old italic letter uu",
            OldItalic::LetterEss => "old italic letter ess",
            OldItalic::NumeralOne => "old italic numeral one",
            OldItalic::NumeralFive => "old italic numeral five",
            OldItalic::NumeralTen => "old italic numeral ten",
            OldItalic::NumeralFifty => "old italic numeral fifty",
            OldItalic::LetterYe => "old italic letter ye",
            OldItalic::LetterNorthernTse => "old italic letter northern tse",
        }
    }
}
