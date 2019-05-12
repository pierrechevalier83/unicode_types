
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
        match self {
            OldPermic::LetterAn => '𐍐',
            OldPermic::LetterBur => '𐍑',
            OldPermic::LetterGai => '𐍒',
            OldPermic::LetterDoi => '𐍓',
            OldPermic::LetterE => '𐍔',
            OldPermic::LetterZhoi => '𐍕',
            OldPermic::LetterDzhoi => '𐍖',
            OldPermic::LetterZata => '𐍗',
            OldPermic::LetterDzita => '𐍘',
            OldPermic::LetterI => '𐍙',
            OldPermic::LetterKoke => '𐍚',
            OldPermic::LetterLei => '𐍛',
            OldPermic::LetterMenoe => '𐍜',
            OldPermic::LetterNenoe => '𐍝',
            OldPermic::LetterVooi => '𐍞',
            OldPermic::LetterPeei => '𐍟',
            OldPermic::LetterRei => '𐍠',
            OldPermic::LetterSii => '𐍡',
            OldPermic::LetterTai => '𐍢',
            OldPermic::LetterU => '𐍣',
            OldPermic::LetterChery => '𐍤',
            OldPermic::LetterShooi => '𐍥',
            OldPermic::LetterShchooi => '𐍦',
            OldPermic::LetterYry => '𐍧',
            OldPermic::LetterYeru => '𐍨',
            OldPermic::LetterO => '𐍩',
            OldPermic::LetterOo => '𐍪',
            OldPermic::LetterEf => '𐍫',
            OldPermic::LetterHa => '𐍬',
            OldPermic::LetterTsiu => '𐍭',
            OldPermic::LetterVer => '𐍮',
            OldPermic::LetterYer => '𐍯',
            OldPermic::LetterYeri => '𐍰',
            OldPermic::LetterYat => '𐍱',
            OldPermic::LetterIe => '𐍲',
            OldPermic::LetterYu => '𐍳',
            OldPermic::LetterYa => '𐍴',
            OldPermic::LetterIa => '𐍵',
            OldPermic::CombiningLetterAn => '𐍶',
            OldPermic::CombiningLetterDoi => '𐍷',
            OldPermic::CombiningLetterZata => '𐍸',
            OldPermic::CombiningLetterNenoe => '𐍹',
            OldPermic::CombiningLetterSii => '𐍺',
        }
    }
}

impl std::convert::TryFrom<char> for OldPermic {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𐍐' => Ok(OldPermic::LetterAn),
            '𐍑' => Ok(OldPermic::LetterBur),
            '𐍒' => Ok(OldPermic::LetterGai),
            '𐍓' => Ok(OldPermic::LetterDoi),
            '𐍔' => Ok(OldPermic::LetterE),
            '𐍕' => Ok(OldPermic::LetterZhoi),
            '𐍖' => Ok(OldPermic::LetterDzhoi),
            '𐍗' => Ok(OldPermic::LetterZata),
            '𐍘' => Ok(OldPermic::LetterDzita),
            '𐍙' => Ok(OldPermic::LetterI),
            '𐍚' => Ok(OldPermic::LetterKoke),
            '𐍛' => Ok(OldPermic::LetterLei),
            '𐍜' => Ok(OldPermic::LetterMenoe),
            '𐍝' => Ok(OldPermic::LetterNenoe),
            '𐍞' => Ok(OldPermic::LetterVooi),
            '𐍟' => Ok(OldPermic::LetterPeei),
            '𐍠' => Ok(OldPermic::LetterRei),
            '𐍡' => Ok(OldPermic::LetterSii),
            '𐍢' => Ok(OldPermic::LetterTai),
            '𐍣' => Ok(OldPermic::LetterU),
            '𐍤' => Ok(OldPermic::LetterChery),
            '𐍥' => Ok(OldPermic::LetterShooi),
            '𐍦' => Ok(OldPermic::LetterShchooi),
            '𐍧' => Ok(OldPermic::LetterYry),
            '𐍨' => Ok(OldPermic::LetterYeru),
            '𐍩' => Ok(OldPermic::LetterO),
            '𐍪' => Ok(OldPermic::LetterOo),
            '𐍫' => Ok(OldPermic::LetterEf),
            '𐍬' => Ok(OldPermic::LetterHa),
            '𐍭' => Ok(OldPermic::LetterTsiu),
            '𐍮' => Ok(OldPermic::LetterVer),
            '𐍯' => Ok(OldPermic::LetterYer),
            '𐍰' => Ok(OldPermic::LetterYeri),
            '𐍱' => Ok(OldPermic::LetterYat),
            '𐍲' => Ok(OldPermic::LetterIe),
            '𐍳' => Ok(OldPermic::LetterYu),
            '𐍴' => Ok(OldPermic::LetterYa),
            '𐍵' => Ok(OldPermic::LetterIa),
            '𐍶' => Ok(OldPermic::CombiningLetterAn),
            '𐍷' => Ok(OldPermic::CombiningLetterDoi),
            '𐍸' => Ok(OldPermic::CombiningLetterZata),
            '𐍹' => Ok(OldPermic::CombiningLetterNenoe),
            '𐍺' => Ok(OldPermic::CombiningLetterSii),
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
