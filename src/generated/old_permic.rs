/// \u{10350} → \u{1037f}\
///\
/// 𐍐 𐍑 𐍒 𐍓 𐍔 𐍕 𐍖 𐍗 𐍘 𐍙 𐍚 𐍛 𐍜 𐍝 𐍞 𐍟\
/// 𐍠 𐍡 𐍢 𐍣 𐍤 𐍥 𐍦 𐍧 𐍨 𐍩 𐍪 𐍫 𐍬 𐍭 𐍮 𐍯\
/// 𐍰 𐍱 𐍲 𐍳 𐍴 𐍵 𐍶 𐍷 𐍸 𐍹 𐍺\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{10350}: '𐍐'
    pub const LETTER_AN: char = '𐍐';
    /// \u{10351}: '𐍑'
    pub const LETTER_BUR: char = '𐍑';
    /// \u{10352}: '𐍒'
    pub const LETTER_GAI: char = '𐍒';
    /// \u{10353}: '𐍓'
    pub const LETTER_DOI: char = '𐍓';
    /// \u{10354}: '𐍔'
    pub const LETTER_E: char = '𐍔';
    /// \u{10355}: '𐍕'
    pub const LETTER_ZHOI: char = '𐍕';
    /// \u{10356}: '𐍖'
    pub const LETTER_DZHOI: char = '𐍖';
    /// \u{10357}: '𐍗'
    pub const LETTER_ZATA: char = '𐍗';
    /// \u{10358}: '𐍘'
    pub const LETTER_DZITA: char = '𐍘';
    /// \u{10359}: '𐍙'
    pub const LETTER_I: char = '𐍙';
    /// \u{1035a}: '𐍚'
    pub const LETTER_KOKE: char = '𐍚';
    /// \u{1035b}: '𐍛'
    pub const LETTER_LEI: char = '𐍛';
    /// \u{1035c}: '𐍜'
    pub const LETTER_MENOE: char = '𐍜';
    /// \u{1035d}: '𐍝'
    pub const LETTER_NENOE: char = '𐍝';
    /// \u{1035e}: '𐍞'
    pub const LETTER_VOOI: char = '𐍞';
    /// \u{1035f}: '𐍟'
    pub const LETTER_PEEI: char = '𐍟';
    /// \u{10360}: '𐍠'
    pub const LETTER_REI: char = '𐍠';
    /// \u{10361}: '𐍡'
    pub const LETTER_SII: char = '𐍡';
    /// \u{10362}: '𐍢'
    pub const LETTER_TAI: char = '𐍢';
    /// \u{10363}: '𐍣'
    pub const LETTER_U: char = '𐍣';
    /// \u{10364}: '𐍤'
    pub const LETTER_CHERY: char = '𐍤';
    /// \u{10365}: '𐍥'
    pub const LETTER_SHOOI: char = '𐍥';
    /// \u{10366}: '𐍦'
    pub const LETTER_SHCHOOI: char = '𐍦';
    /// \u{10367}: '𐍧'
    pub const LETTER_YRY: char = '𐍧';
    /// \u{10368}: '𐍨'
    pub const LETTER_YERU: char = '𐍨';
    /// \u{10369}: '𐍩'
    pub const LETTER_O: char = '𐍩';
    /// \u{1036a}: '𐍪'
    pub const LETTER_OO: char = '𐍪';
    /// \u{1036b}: '𐍫'
    pub const LETTER_EF: char = '𐍫';
    /// \u{1036c}: '𐍬'
    pub const LETTER_HA: char = '𐍬';
    /// \u{1036d}: '𐍭'
    pub const LETTER_TSIU: char = '𐍭';
    /// \u{1036e}: '𐍮'
    pub const LETTER_VER: char = '𐍮';
    /// \u{1036f}: '𐍯'
    pub const LETTER_YER: char = '𐍯';
    /// \u{10370}: '𐍰'
    pub const LETTER_YERI: char = '𐍰';
    /// \u{10371}: '𐍱'
    pub const LETTER_YAT: char = '𐍱';
    /// \u{10372}: '𐍲'
    pub const LETTER_IE: char = '𐍲';
    /// \u{10373}: '𐍳'
    pub const LETTER_YU: char = '𐍳';
    /// \u{10374}: '𐍴'
    pub const LETTER_YA: char = '𐍴';
    /// \u{10375}: '𐍵'
    pub const LETTER_IA: char = '𐍵';
    /// \u{10376}: '𐍶'
    pub const COMBINING_LETTER_AN: char = '𐍶';
    /// \u{10377}: '𐍷'
    pub const COMBINING_LETTER_DOI: char = '𐍷';
    /// \u{10378}: '𐍸'
    pub const COMBINING_LETTER_ZATA: char = '𐍸';
    /// \u{10379}: '𐍹'
    pub const COMBINING_LETTER_NENOE: char = '𐍹';
    /// \u{1037a}: '𐍺'
    pub const COMBINING_LETTER_SII: char = '𐍺';
}

/// An enum to represent all characters in the OldPermic block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum OldPermic {
    /// \u{10350}: '𐍐'
    LetterAn,
    /// \u{10351}: '𐍑'
    LetterBur,
    /// \u{10352}: '𐍒'
    LetterGai,
    /// \u{10353}: '𐍓'
    LetterDoi,
    /// \u{10354}: '𐍔'
    LetterE,
    /// \u{10355}: '𐍕'
    LetterZhoi,
    /// \u{10356}: '𐍖'
    LetterDzhoi,
    /// \u{10357}: '𐍗'
    LetterZata,
    /// \u{10358}: '𐍘'
    LetterDzita,
    /// \u{10359}: '𐍙'
    LetterI,
    /// \u{1035a}: '𐍚'
    LetterKoke,
    /// \u{1035b}: '𐍛'
    LetterLei,
    /// \u{1035c}: '𐍜'
    LetterMenoe,
    /// \u{1035d}: '𐍝'
    LetterNenoe,
    /// \u{1035e}: '𐍞'
    LetterVooi,
    /// \u{1035f}: '𐍟'
    LetterPeei,
    /// \u{10360}: '𐍠'
    LetterRei,
    /// \u{10361}: '𐍡'
    LetterSii,
    /// \u{10362}: '𐍢'
    LetterTai,
    /// \u{10363}: '𐍣'
    LetterU,
    /// \u{10364}: '𐍤'
    LetterChery,
    /// \u{10365}: '𐍥'
    LetterShooi,
    /// \u{10366}: '𐍦'
    LetterShchooi,
    /// \u{10367}: '𐍧'
    LetterYry,
    /// \u{10368}: '𐍨'
    LetterYeru,
    /// \u{10369}: '𐍩'
    LetterO,
    /// \u{1036a}: '𐍪'
    LetterOo,
    /// \u{1036b}: '𐍫'
    LetterEf,
    /// \u{1036c}: '𐍬'
    LetterHa,
    /// \u{1036d}: '𐍭'
    LetterTsiu,
    /// \u{1036e}: '𐍮'
    LetterVer,
    /// \u{1036f}: '𐍯'
    LetterYer,
    /// \u{10370}: '𐍰'
    LetterYeri,
    /// \u{10371}: '𐍱'
    LetterYat,
    /// \u{10372}: '𐍲'
    LetterIe,
    /// \u{10373}: '𐍳'
    LetterYu,
    /// \u{10374}: '𐍴'
    LetterYa,
    /// \u{10375}: '𐍵'
    LetterIa,
    /// \u{10376}: '𐍶'
    CombiningLetterAn,
    /// \u{10377}: '𐍷'
    CombiningLetterDoi,
    /// \u{10378}: '𐍸'
    CombiningLetterZata,
    /// \u{10379}: '𐍹'
    CombiningLetterNenoe,
    /// \u{1037a}: '𐍺'
    CombiningLetterSii,
}

impl Into<char> for OldPermic {
    fn into(self) -> char {
        use constants::*;
        match self {
            OldPermic::LetterAn => LETTER_AN,
            OldPermic::LetterBur => LETTER_BUR,
            OldPermic::LetterGai => LETTER_GAI,
            OldPermic::LetterDoi => LETTER_DOI,
            OldPermic::LetterE => LETTER_E,
            OldPermic::LetterZhoi => LETTER_ZHOI,
            OldPermic::LetterDzhoi => LETTER_DZHOI,
            OldPermic::LetterZata => LETTER_ZATA,
            OldPermic::LetterDzita => LETTER_DZITA,
            OldPermic::LetterI => LETTER_I,
            OldPermic::LetterKoke => LETTER_KOKE,
            OldPermic::LetterLei => LETTER_LEI,
            OldPermic::LetterMenoe => LETTER_MENOE,
            OldPermic::LetterNenoe => LETTER_NENOE,
            OldPermic::LetterVooi => LETTER_VOOI,
            OldPermic::LetterPeei => LETTER_PEEI,
            OldPermic::LetterRei => LETTER_REI,
            OldPermic::LetterSii => LETTER_SII,
            OldPermic::LetterTai => LETTER_TAI,
            OldPermic::LetterU => LETTER_U,
            OldPermic::LetterChery => LETTER_CHERY,
            OldPermic::LetterShooi => LETTER_SHOOI,
            OldPermic::LetterShchooi => LETTER_SHCHOOI,
            OldPermic::LetterYry => LETTER_YRY,
            OldPermic::LetterYeru => LETTER_YERU,
            OldPermic::LetterO => LETTER_O,
            OldPermic::LetterOo => LETTER_OO,
            OldPermic::LetterEf => LETTER_EF,
            OldPermic::LetterHa => LETTER_HA,
            OldPermic::LetterTsiu => LETTER_TSIU,
            OldPermic::LetterVer => LETTER_VER,
            OldPermic::LetterYer => LETTER_YER,
            OldPermic::LetterYeri => LETTER_YERI,
            OldPermic::LetterYat => LETTER_YAT,
            OldPermic::LetterIe => LETTER_IE,
            OldPermic::LetterYu => LETTER_YU,
            OldPermic::LetterYa => LETTER_YA,
            OldPermic::LetterIa => LETTER_IA,
            OldPermic::CombiningLetterAn => COMBINING_LETTER_AN,
            OldPermic::CombiningLetterDoi => COMBINING_LETTER_DOI,
            OldPermic::CombiningLetterZata => COMBINING_LETTER_ZATA,
            OldPermic::CombiningLetterNenoe => COMBINING_LETTER_NENOE,
            OldPermic::CombiningLetterSii => COMBINING_LETTER_SII,
        }
    }
}

impl std::convert::TryFrom<char> for OldPermic {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_AN => Ok(OldPermic::LetterAn),
            LETTER_BUR => Ok(OldPermic::LetterBur),
            LETTER_GAI => Ok(OldPermic::LetterGai),
            LETTER_DOI => Ok(OldPermic::LetterDoi),
            LETTER_E => Ok(OldPermic::LetterE),
            LETTER_ZHOI => Ok(OldPermic::LetterZhoi),
            LETTER_DZHOI => Ok(OldPermic::LetterDzhoi),
            LETTER_ZATA => Ok(OldPermic::LetterZata),
            LETTER_DZITA => Ok(OldPermic::LetterDzita),
            LETTER_I => Ok(OldPermic::LetterI),
            LETTER_KOKE => Ok(OldPermic::LetterKoke),
            LETTER_LEI => Ok(OldPermic::LetterLei),
            LETTER_MENOE => Ok(OldPermic::LetterMenoe),
            LETTER_NENOE => Ok(OldPermic::LetterNenoe),
            LETTER_VOOI => Ok(OldPermic::LetterVooi),
            LETTER_PEEI => Ok(OldPermic::LetterPeei),
            LETTER_REI => Ok(OldPermic::LetterRei),
            LETTER_SII => Ok(OldPermic::LetterSii),
            LETTER_TAI => Ok(OldPermic::LetterTai),
            LETTER_U => Ok(OldPermic::LetterU),
            LETTER_CHERY => Ok(OldPermic::LetterChery),
            LETTER_SHOOI => Ok(OldPermic::LetterShooi),
            LETTER_SHCHOOI => Ok(OldPermic::LetterShchooi),
            LETTER_YRY => Ok(OldPermic::LetterYry),
            LETTER_YERU => Ok(OldPermic::LetterYeru),
            LETTER_O => Ok(OldPermic::LetterO),
            LETTER_OO => Ok(OldPermic::LetterOo),
            LETTER_EF => Ok(OldPermic::LetterEf),
            LETTER_HA => Ok(OldPermic::LetterHa),
            LETTER_TSIU => Ok(OldPermic::LetterTsiu),
            LETTER_VER => Ok(OldPermic::LetterVer),
            LETTER_YER => Ok(OldPermic::LetterYer),
            LETTER_YERI => Ok(OldPermic::LetterYeri),
            LETTER_YAT => Ok(OldPermic::LetterYat),
            LETTER_IE => Ok(OldPermic::LetterIe),
            LETTER_YU => Ok(OldPermic::LetterYu),
            LETTER_YA => Ok(OldPermic::LetterYa),
            LETTER_IA => Ok(OldPermic::LetterIa),
            COMBINING_LETTER_AN => Ok(OldPermic::CombiningLetterAn),
            COMBINING_LETTER_DOI => Ok(OldPermic::CombiningLetterDoi),
            COMBINING_LETTER_ZATA => Ok(OldPermic::CombiningLetterZata),
            COMBINING_LETTER_NENOE => Ok(OldPermic::CombiningLetterNenoe),
            COMBINING_LETTER_SII => Ok(OldPermic::CombiningLetterSii),
            _ => Err(()),
        }
    }
}

impl Into<u32> for OldPermic {
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

impl std::convert::TryFrom<u32> for OldPermic {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for OldPermic {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl OldPermic {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        OldPermic::LetterAn
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            OldPermic::LetterAn => "old permic letter an",
            OldPermic::LetterBur => "old permic letter bur",
            OldPermic::LetterGai => "old permic letter gai",
            OldPermic::LetterDoi => "old permic letter doi",
            OldPermic::LetterE => "old permic letter e",
            OldPermic::LetterZhoi => "old permic letter zhoi",
            OldPermic::LetterDzhoi => "old permic letter dzhoi",
            OldPermic::LetterZata => "old permic letter zata",
            OldPermic::LetterDzita => "old permic letter dzita",
            OldPermic::LetterI => "old permic letter i",
            OldPermic::LetterKoke => "old permic letter koke",
            OldPermic::LetterLei => "old permic letter lei",
            OldPermic::LetterMenoe => "old permic letter menoe",
            OldPermic::LetterNenoe => "old permic letter nenoe",
            OldPermic::LetterVooi => "old permic letter vooi",
            OldPermic::LetterPeei => "old permic letter peei",
            OldPermic::LetterRei => "old permic letter rei",
            OldPermic::LetterSii => "old permic letter sii",
            OldPermic::LetterTai => "old permic letter tai",
            OldPermic::LetterU => "old permic letter u",
            OldPermic::LetterChery => "old permic letter chery",
            OldPermic::LetterShooi => "old permic letter shooi",
            OldPermic::LetterShchooi => "old permic letter shchooi",
            OldPermic::LetterYry => "old permic letter yry",
            OldPermic::LetterYeru => "old permic letter yeru",
            OldPermic::LetterO => "old permic letter o",
            OldPermic::LetterOo => "old permic letter oo",
            OldPermic::LetterEf => "old permic letter ef",
            OldPermic::LetterHa => "old permic letter ha",
            OldPermic::LetterTsiu => "old permic letter tsiu",
            OldPermic::LetterVer => "old permic letter ver",
            OldPermic::LetterYer => "old permic letter yer",
            OldPermic::LetterYeri => "old permic letter yeri",
            OldPermic::LetterYat => "old permic letter yat",
            OldPermic::LetterIe => "old permic letter ie",
            OldPermic::LetterYu => "old permic letter yu",
            OldPermic::LetterYa => "old permic letter ya",
            OldPermic::LetterIa => "old permic letter ia",
            OldPermic::CombiningLetterAn => "combining old permic letter an",
            OldPermic::CombiningLetterDoi => "combining old permic letter doi",
            OldPermic::CombiningLetterZata => "combining old permic letter zata",
            OldPermic::CombiningLetterNenoe => "combining old permic letter nenoe",
            OldPermic::CombiningLetterSii => "combining old permic letter sii",
        }
    }
}
