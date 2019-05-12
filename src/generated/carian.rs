
/// An enum to represent all characters in the Carian block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Carian {
    /// \u{102a0}: '𐊠'
    LetterA,
    /// \u{102a1}: '𐊡'
    LetterP2,
    /// \u{102a2}: '𐊢'
    LetterD,
    /// \u{102a3}: '𐊣'
    LetterL,
    /// \u{102a4}: '𐊤'
    LetterUuu,
    /// \u{102a5}: '𐊥'
    LetterR,
    /// \u{102a6}: '𐊦'
    LetterLd,
    /// \u{102a7}: '𐊧'
    LetterA2,
    /// \u{102a8}: '𐊨'
    LetterQ,
    /// \u{102a9}: '𐊩'
    LetterB,
    /// \u{102aa}: '𐊪'
    LetterM,
    /// \u{102ab}: '𐊫'
    LetterO,
    /// \u{102ac}: '𐊬'
    LetterD2,
    /// \u{102ad}: '𐊭'
    LetterT,
    /// \u{102ae}: '𐊮'
    LetterSh,
    /// \u{102af}: '𐊯'
    LetterSh2,
    /// \u{102b0}: '𐊰'
    LetterS,
    /// \u{102b1}: '𐊱'
    LetterCDash18,
    /// \u{102b2}: '𐊲'
    LetterU,
    /// \u{102b3}: '𐊳'
    LetterNn,
    /// \u{102b4}: '𐊴'
    LetterX,
    /// \u{102b5}: '𐊵'
    LetterN,
    /// \u{102b6}: '𐊶'
    LetterTt2,
    /// \u{102b7}: '𐊷'
    LetterP,
    /// \u{102b8}: '𐊸'
    LetterSs,
    /// \u{102b9}: '𐊹'
    LetterI,
    /// \u{102ba}: '𐊺'
    LetterE,
    /// \u{102bb}: '𐊻'
    LetterUuuu,
    /// \u{102bc}: '𐊼'
    LetterK,
    /// \u{102bd}: '𐊽'
    LetterK2,
    /// \u{102be}: '𐊾'
    LetterNd,
    /// \u{102bf}: '𐊿'
    LetterUu,
    /// \u{102c0}: '𐋀'
    LetterG,
    /// \u{102c1}: '𐋁'
    LetterG2,
    /// \u{102c2}: '𐋂'
    LetterSt,
    /// \u{102c3}: '𐋃'
    LetterSt2,
    /// \u{102c4}: '𐋄'
    LetterNg,
    /// \u{102c5}: '𐋅'
    LetterIi,
    /// \u{102c6}: '𐋆'
    LetterCDash39,
    /// \u{102c7}: '𐋇'
    LetterTt,
    /// \u{102c8}: '𐋈'
    LetterUuu2,
    /// \u{102c9}: '𐋉'
    LetterRr,
    /// \u{102ca}: '𐋊'
    LetterMb,
    /// \u{102cb}: '𐋋'
    LetterMb2,
    /// \u{102cc}: '𐋌'
    LetterMb3,
    /// \u{102cd}: '𐋍'
    LetterMb4,
    /// \u{102ce}: '𐋎'
    LetterLd2,
    /// \u{102cf}: '𐋏'
    LetterE2,
    /// \u{102d0}: '𐋐'
    LetterUuu3,
}

impl Into<char> for Carian {
    fn into(self) -> char {
        match self {
            Carian::LetterA => '𐊠',
            Carian::LetterP2 => '𐊡',
            Carian::LetterD => '𐊢',
            Carian::LetterL => '𐊣',
            Carian::LetterUuu => '𐊤',
            Carian::LetterR => '𐊥',
            Carian::LetterLd => '𐊦',
            Carian::LetterA2 => '𐊧',
            Carian::LetterQ => '𐊨',
            Carian::LetterB => '𐊩',
            Carian::LetterM => '𐊪',
            Carian::LetterO => '𐊫',
            Carian::LetterD2 => '𐊬',
            Carian::LetterT => '𐊭',
            Carian::LetterSh => '𐊮',
            Carian::LetterSh2 => '𐊯',
            Carian::LetterS => '𐊰',
            Carian::LetterCDash18 => '𐊱',
            Carian::LetterU => '𐊲',
            Carian::LetterNn => '𐊳',
            Carian::LetterX => '𐊴',
            Carian::LetterN => '𐊵',
            Carian::LetterTt2 => '𐊶',
            Carian::LetterP => '𐊷',
            Carian::LetterSs => '𐊸',
            Carian::LetterI => '𐊹',
            Carian::LetterE => '𐊺',
            Carian::LetterUuuu => '𐊻',
            Carian::LetterK => '𐊼',
            Carian::LetterK2 => '𐊽',
            Carian::LetterNd => '𐊾',
            Carian::LetterUu => '𐊿',
            Carian::LetterG => '𐋀',
            Carian::LetterG2 => '𐋁',
            Carian::LetterSt => '𐋂',
            Carian::LetterSt2 => '𐋃',
            Carian::LetterNg => '𐋄',
            Carian::LetterIi => '𐋅',
            Carian::LetterCDash39 => '𐋆',
            Carian::LetterTt => '𐋇',
            Carian::LetterUuu2 => '𐋈',
            Carian::LetterRr => '𐋉',
            Carian::LetterMb => '𐋊',
            Carian::LetterMb2 => '𐋋',
            Carian::LetterMb3 => '𐋌',
            Carian::LetterMb4 => '𐋍',
            Carian::LetterLd2 => '𐋎',
            Carian::LetterE2 => '𐋏',
            Carian::LetterUuu3 => '𐋐',
        }
    }
}

impl std::convert::TryFrom<char> for Carian {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𐊠' => Ok(Carian::LetterA),
            '𐊡' => Ok(Carian::LetterP2),
            '𐊢' => Ok(Carian::LetterD),
            '𐊣' => Ok(Carian::LetterL),
            '𐊤' => Ok(Carian::LetterUuu),
            '𐊥' => Ok(Carian::LetterR),
            '𐊦' => Ok(Carian::LetterLd),
            '𐊧' => Ok(Carian::LetterA2),
            '𐊨' => Ok(Carian::LetterQ),
            '𐊩' => Ok(Carian::LetterB),
            '𐊪' => Ok(Carian::LetterM),
            '𐊫' => Ok(Carian::LetterO),
            '𐊬' => Ok(Carian::LetterD2),
            '𐊭' => Ok(Carian::LetterT),
            '𐊮' => Ok(Carian::LetterSh),
            '𐊯' => Ok(Carian::LetterSh2),
            '𐊰' => Ok(Carian::LetterS),
            '𐊱' => Ok(Carian::LetterCDash18),
            '𐊲' => Ok(Carian::LetterU),
            '𐊳' => Ok(Carian::LetterNn),
            '𐊴' => Ok(Carian::LetterX),
            '𐊵' => Ok(Carian::LetterN),
            '𐊶' => Ok(Carian::LetterTt2),
            '𐊷' => Ok(Carian::LetterP),
            '𐊸' => Ok(Carian::LetterSs),
            '𐊹' => Ok(Carian::LetterI),
            '𐊺' => Ok(Carian::LetterE),
            '𐊻' => Ok(Carian::LetterUuuu),
            '𐊼' => Ok(Carian::LetterK),
            '𐊽' => Ok(Carian::LetterK2),
            '𐊾' => Ok(Carian::LetterNd),
            '𐊿' => Ok(Carian::LetterUu),
            '𐋀' => Ok(Carian::LetterG),
            '𐋁' => Ok(Carian::LetterG2),
            '𐋂' => Ok(Carian::LetterSt),
            '𐋃' => Ok(Carian::LetterSt2),
            '𐋄' => Ok(Carian::LetterNg),
            '𐋅' => Ok(Carian::LetterIi),
            '𐋆' => Ok(Carian::LetterCDash39),
            '𐋇' => Ok(Carian::LetterTt),
            '𐋈' => Ok(Carian::LetterUuu2),
            '𐋉' => Ok(Carian::LetterRr),
            '𐋊' => Ok(Carian::LetterMb),
            '𐋋' => Ok(Carian::LetterMb2),
            '𐋌' => Ok(Carian::LetterMb3),
            '𐋍' => Ok(Carian::LetterMb4),
            '𐋎' => Ok(Carian::LetterLd2),
            '𐋏' => Ok(Carian::LetterE2),
            '𐋐' => Ok(Carian::LetterUuu3),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Carian {
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

impl std::convert::TryFrom<u32> for Carian {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Carian {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Carian {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Carian::LetterA
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Carian::LetterA => "carian letter a",
            Carian::LetterP2 => "carian letter p2",
            Carian::LetterD => "carian letter d",
            Carian::LetterL => "carian letter l",
            Carian::LetterUuu => "carian letter uuu",
            Carian::LetterR => "carian letter r",
            Carian::LetterLd => "carian letter ld",
            Carian::LetterA2 => "carian letter a2",
            Carian::LetterQ => "carian letter q",
            Carian::LetterB => "carian letter b",
            Carian::LetterM => "carian letter m",
            Carian::LetterO => "carian letter o",
            Carian::LetterD2 => "carian letter d2",
            Carian::LetterT => "carian letter t",
            Carian::LetterSh => "carian letter sh",
            Carian::LetterSh2 => "carian letter sh2",
            Carian::LetterS => "carian letter s",
            Carian::LetterCDash18 => "carian letter c-18",
            Carian::LetterU => "carian letter u",
            Carian::LetterNn => "carian letter nn",
            Carian::LetterX => "carian letter x",
            Carian::LetterN => "carian letter n",
            Carian::LetterTt2 => "carian letter tt2",
            Carian::LetterP => "carian letter p",
            Carian::LetterSs => "carian letter ss",
            Carian::LetterI => "carian letter i",
            Carian::LetterE => "carian letter e",
            Carian::LetterUuuu => "carian letter uuuu",
            Carian::LetterK => "carian letter k",
            Carian::LetterK2 => "carian letter k2",
            Carian::LetterNd => "carian letter nd",
            Carian::LetterUu => "carian letter uu",
            Carian::LetterG => "carian letter g",
            Carian::LetterG2 => "carian letter g2",
            Carian::LetterSt => "carian letter st",
            Carian::LetterSt2 => "carian letter st2",
            Carian::LetterNg => "carian letter ng",
            Carian::LetterIi => "carian letter ii",
            Carian::LetterCDash39 => "carian letter c-39",
            Carian::LetterTt => "carian letter tt",
            Carian::LetterUuu2 => "carian letter uuu2",
            Carian::LetterRr => "carian letter rr",
            Carian::LetterMb => "carian letter mb",
            Carian::LetterMb2 => "carian letter mb2",
            Carian::LetterMb3 => "carian letter mb3",
            Carian::LetterMb4 => "carian letter mb4",
            Carian::LetterLd2 => "carian letter ld2",
            Carian::LetterE2 => "carian letter e2",
            Carian::LetterUuu3 => "carian letter uuu3",
        }
    }
}
