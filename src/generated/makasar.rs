
/// An enum to represent all characters in the Makasar block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Makasar {
    /// \u{11ee0}: '𑻠'
    LetterKa,
    /// \u{11ee1}: '𑻡'
    LetterGa,
    /// \u{11ee2}: '𑻢'
    LetterNga,
    /// \u{11ee3}: '𑻣'
    LetterPa,
    /// \u{11ee4}: '𑻤'
    LetterBa,
    /// \u{11ee5}: '𑻥'
    LetterMa,
    /// \u{11ee6}: '𑻦'
    LetterTa,
    /// \u{11ee7}: '𑻧'
    LetterDa,
    /// \u{11ee8}: '𑻨'
    LetterNa,
    /// \u{11ee9}: '𑻩'
    LetterCa,
    /// \u{11eea}: '𑻪'
    LetterJa,
    /// \u{11eeb}: '𑻫'
    LetterNya,
    /// \u{11eec}: '𑻬'
    LetterYa,
    /// \u{11eed}: '𑻭'
    LetterRa,
    /// \u{11eee}: '𑻮'
    LetterLa,
    /// \u{11eef}: '𑻯'
    LetterVa,
    /// \u{11ef0}: '𑻰'
    LetterSa,
    /// \u{11ef1}: '𑻱'
    LetterA,
    /// \u{11ef2}: '𑻲'
    Angka,
    /// \u{11ef3}: '𑻳'
    VowelSignI,
    /// \u{11ef4}: '𑻴'
    VowelSignU,
    /// \u{11ef5}: '𑻵'
    VowelSignE,
    /// \u{11ef6}: '𑻶'
    VowelSignO,
    /// \u{11ef7}: '𑻷'
    Passimbang,
    /// \u{11ef8}: '𑻸'
    EndOfSection,
}

impl Into<char> for Makasar {
    fn into(self) -> char {
        match self {
            Makasar::LetterKa => '𑻠',
            Makasar::LetterGa => '𑻡',
            Makasar::LetterNga => '𑻢',
            Makasar::LetterPa => '𑻣',
            Makasar::LetterBa => '𑻤',
            Makasar::LetterMa => '𑻥',
            Makasar::LetterTa => '𑻦',
            Makasar::LetterDa => '𑻧',
            Makasar::LetterNa => '𑻨',
            Makasar::LetterCa => '𑻩',
            Makasar::LetterJa => '𑻪',
            Makasar::LetterNya => '𑻫',
            Makasar::LetterYa => '𑻬',
            Makasar::LetterRa => '𑻭',
            Makasar::LetterLa => '𑻮',
            Makasar::LetterVa => '𑻯',
            Makasar::LetterSa => '𑻰',
            Makasar::LetterA => '𑻱',
            Makasar::Angka => '𑻲',
            Makasar::VowelSignI => '𑻳',
            Makasar::VowelSignU => '𑻴',
            Makasar::VowelSignE => '𑻵',
            Makasar::VowelSignO => '𑻶',
            Makasar::Passimbang => '𑻷',
            Makasar::EndOfSection => '𑻸',
        }
    }
}

impl std::convert::TryFrom<char> for Makasar {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𑻠' => Ok(Makasar::LetterKa),
            '𑻡' => Ok(Makasar::LetterGa),
            '𑻢' => Ok(Makasar::LetterNga),
            '𑻣' => Ok(Makasar::LetterPa),
            '𑻤' => Ok(Makasar::LetterBa),
            '𑻥' => Ok(Makasar::LetterMa),
            '𑻦' => Ok(Makasar::LetterTa),
            '𑻧' => Ok(Makasar::LetterDa),
            '𑻨' => Ok(Makasar::LetterNa),
            '𑻩' => Ok(Makasar::LetterCa),
            '𑻪' => Ok(Makasar::LetterJa),
            '𑻫' => Ok(Makasar::LetterNya),
            '𑻬' => Ok(Makasar::LetterYa),
            '𑻭' => Ok(Makasar::LetterRa),
            '𑻮' => Ok(Makasar::LetterLa),
            '𑻯' => Ok(Makasar::LetterVa),
            '𑻰' => Ok(Makasar::LetterSa),
            '𑻱' => Ok(Makasar::LetterA),
            '𑻲' => Ok(Makasar::Angka),
            '𑻳' => Ok(Makasar::VowelSignI),
            '𑻴' => Ok(Makasar::VowelSignU),
            '𑻵' => Ok(Makasar::VowelSignE),
            '𑻶' => Ok(Makasar::VowelSignO),
            '𑻷' => Ok(Makasar::Passimbang),
            '𑻸' => Ok(Makasar::EndOfSection),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Makasar {
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

impl std::convert::TryFrom<u32> for Makasar {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Makasar {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Makasar {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Makasar::LetterKa
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Makasar::LetterKa => "makasar letter ka",
            Makasar::LetterGa => "makasar letter ga",
            Makasar::LetterNga => "makasar letter nga",
            Makasar::LetterPa => "makasar letter pa",
            Makasar::LetterBa => "makasar letter ba",
            Makasar::LetterMa => "makasar letter ma",
            Makasar::LetterTa => "makasar letter ta",
            Makasar::LetterDa => "makasar letter da",
            Makasar::LetterNa => "makasar letter na",
            Makasar::LetterCa => "makasar letter ca",
            Makasar::LetterJa => "makasar letter ja",
            Makasar::LetterNya => "makasar letter nya",
            Makasar::LetterYa => "makasar letter ya",
            Makasar::LetterRa => "makasar letter ra",
            Makasar::LetterLa => "makasar letter la",
            Makasar::LetterVa => "makasar letter va",
            Makasar::LetterSa => "makasar letter sa",
            Makasar::LetterA => "makasar letter a",
            Makasar::Angka => "makasar angka",
            Makasar::VowelSignI => "makasar vowel sign i",
            Makasar::VowelSignU => "makasar vowel sign u",
            Makasar::VowelSignE => "makasar vowel sign e",
            Makasar::VowelSignO => "makasar vowel sign o",
            Makasar::Passimbang => "makasar passimbang",
            Makasar::EndOfSection => "makasar end of section",
        }
    }
}
