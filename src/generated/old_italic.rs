
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
        match self {
            OldItalic::LetterA => '𐌀',
            OldItalic::LetterBe => '𐌁',
            OldItalic::LetterKe => '𐌂',
            OldItalic::LetterDe => '𐌃',
            OldItalic::LetterE => '𐌄',
            OldItalic::LetterVe => '𐌅',
            OldItalic::LetterZe => '𐌆',
            OldItalic::LetterHe => '𐌇',
            OldItalic::LetterThe => '𐌈',
            OldItalic::LetterI => '𐌉',
            OldItalic::LetterKa => '𐌊',
            OldItalic::LetterEl => '𐌋',
            OldItalic::LetterEm => '𐌌',
            OldItalic::LetterEn => '𐌍',
            OldItalic::LetterEsh => '𐌎',
            OldItalic::LetterO => '𐌏',
            OldItalic::LetterPe => '𐌐',
            OldItalic::LetterShe => '𐌑',
            OldItalic::LetterKu => '𐌒',
            OldItalic::LetterEr => '𐌓',
            OldItalic::LetterEs => '𐌔',
            OldItalic::LetterTe => '𐌕',
            OldItalic::LetterU => '𐌖',
            OldItalic::LetterEks => '𐌗',
            OldItalic::LetterPhe => '𐌘',
            OldItalic::LetterKhe => '𐌙',
            OldItalic::LetterEf => '𐌚',
            OldItalic::LetterErs => '𐌛',
            OldItalic::LetterChe => '𐌜',
            OldItalic::LetterIi => '𐌝',
            OldItalic::LetterUu => '𐌞',
            OldItalic::LetterEss => '𐌟',
            OldItalic::NumeralOne => '𐌠',
            OldItalic::NumeralFive => '𐌡',
            OldItalic::NumeralTen => '𐌢',
            OldItalic::NumeralFifty => '𐌣',
            OldItalic::LetterYe => '𐌭',
            OldItalic::LetterNorthernTse => '𐌮',
        }
    }
}

impl std::convert::TryFrom<char> for OldItalic {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𐌀' => Ok(OldItalic::LetterA),
            '𐌁' => Ok(OldItalic::LetterBe),
            '𐌂' => Ok(OldItalic::LetterKe),
            '𐌃' => Ok(OldItalic::LetterDe),
            '𐌄' => Ok(OldItalic::LetterE),
            '𐌅' => Ok(OldItalic::LetterVe),
            '𐌆' => Ok(OldItalic::LetterZe),
            '𐌇' => Ok(OldItalic::LetterHe),
            '𐌈' => Ok(OldItalic::LetterThe),
            '𐌉' => Ok(OldItalic::LetterI),
            '𐌊' => Ok(OldItalic::LetterKa),
            '𐌋' => Ok(OldItalic::LetterEl),
            '𐌌' => Ok(OldItalic::LetterEm),
            '𐌍' => Ok(OldItalic::LetterEn),
            '𐌎' => Ok(OldItalic::LetterEsh),
            '𐌏' => Ok(OldItalic::LetterO),
            '𐌐' => Ok(OldItalic::LetterPe),
            '𐌑' => Ok(OldItalic::LetterShe),
            '𐌒' => Ok(OldItalic::LetterKu),
            '𐌓' => Ok(OldItalic::LetterEr),
            '𐌔' => Ok(OldItalic::LetterEs),
            '𐌕' => Ok(OldItalic::LetterTe),
            '𐌖' => Ok(OldItalic::LetterU),
            '𐌗' => Ok(OldItalic::LetterEks),
            '𐌘' => Ok(OldItalic::LetterPhe),
            '𐌙' => Ok(OldItalic::LetterKhe),
            '𐌚' => Ok(OldItalic::LetterEf),
            '𐌛' => Ok(OldItalic::LetterErs),
            '𐌜' => Ok(OldItalic::LetterChe),
            '𐌝' => Ok(OldItalic::LetterIi),
            '𐌞' => Ok(OldItalic::LetterUu),
            '𐌟' => Ok(OldItalic::LetterEss),
            '𐌠' => Ok(OldItalic::NumeralOne),
            '𐌡' => Ok(OldItalic::NumeralFive),
            '𐌢' => Ok(OldItalic::NumeralTen),
            '𐌣' => Ok(OldItalic::NumeralFifty),
            '𐌭' => Ok(OldItalic::LetterYe),
            '𐌮' => Ok(OldItalic::LetterNorthernTse),
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
