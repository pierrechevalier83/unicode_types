
/// An enum to represent all characters in the OldPersian block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum OldPersian {
    /// \u{103a0}: '𐎠'
    SignA,
    /// \u{103a1}: '𐎡'
    SignI,
    /// \u{103a2}: '𐎢'
    SignU,
    /// \u{103a3}: '𐎣'
    SignKa,
    /// \u{103a4}: '𐎤'
    SignKu,
    /// \u{103a5}: '𐎥'
    SignGa,
    /// \u{103a6}: '𐎦'
    SignGu,
    /// \u{103a7}: '𐎧'
    SignXa,
    /// \u{103a8}: '𐎨'
    SignCa,
    /// \u{103a9}: '𐎩'
    SignJa,
    /// \u{103aa}: '𐎪'
    SignJi,
    /// \u{103ab}: '𐎫'
    SignTa,
    /// \u{103ac}: '𐎬'
    SignTu,
    /// \u{103ad}: '𐎭'
    SignDa,
    /// \u{103ae}: '𐎮'
    SignDi,
    /// \u{103af}: '𐎯'
    SignDu,
    /// \u{103b0}: '𐎰'
    SignTha,
    /// \u{103b1}: '𐎱'
    SignPa,
    /// \u{103b2}: '𐎲'
    SignBa,
    /// \u{103b3}: '𐎳'
    SignFa,
    /// \u{103b4}: '𐎴'
    SignNa,
    /// \u{103b5}: '𐎵'
    SignNu,
    /// \u{103b6}: '𐎶'
    SignMa,
    /// \u{103b7}: '𐎷'
    SignMi,
    /// \u{103b8}: '𐎸'
    SignMu,
    /// \u{103b9}: '𐎹'
    SignYa,
    /// \u{103ba}: '𐎺'
    SignVa,
    /// \u{103bb}: '𐎻'
    SignVi,
    /// \u{103bc}: '𐎼'
    SignRa,
    /// \u{103bd}: '𐎽'
    SignRu,
    /// \u{103be}: '𐎾'
    SignLa,
    /// \u{103bf}: '𐎿'
    SignSa,
    /// \u{103c0}: '𐏀'
    SignZa,
    /// \u{103c1}: '𐏁'
    SignSha,
    /// \u{103c2}: '𐏂'
    SignSsa,
    /// \u{103c3}: '𐏃'
    SignHa,
    /// \u{103c8}: '𐏈'
    SignAuramazdaa,
    /// \u{103c9}: '𐏉'
    SignAuramazdaaDash2,
    /// \u{103ca}: '𐏊'
    SignAuramazdaaha,
    /// \u{103cb}: '𐏋'
    SignXshaayathiya,
    /// \u{103cc}: '𐏌'
    SignDahyaaush,
    /// \u{103cd}: '𐏍'
    SignDahyaaushDash2,
    /// \u{103ce}: '𐏎'
    SignBaga,
    /// \u{103cf}: '𐏏'
    SignBuumish,
    /// \u{103d0}: '𐏐'
    WordDivider,
    /// \u{103d1}: '𐏑'
    NumberOne,
    /// \u{103d2}: '𐏒'
    NumberTwo,
    /// \u{103d3}: '𐏓'
    NumberTen,
    /// \u{103d4}: '𐏔'
    NumberTwenty,
    /// \u{103d5}: '𐏕'
    NumberHundred,
}

impl Into<char> for OldPersian {
    fn into(self) -> char {
        match self {
            OldPersian::SignA => '𐎠',
            OldPersian::SignI => '𐎡',
            OldPersian::SignU => '𐎢',
            OldPersian::SignKa => '𐎣',
            OldPersian::SignKu => '𐎤',
            OldPersian::SignGa => '𐎥',
            OldPersian::SignGu => '𐎦',
            OldPersian::SignXa => '𐎧',
            OldPersian::SignCa => '𐎨',
            OldPersian::SignJa => '𐎩',
            OldPersian::SignJi => '𐎪',
            OldPersian::SignTa => '𐎫',
            OldPersian::SignTu => '𐎬',
            OldPersian::SignDa => '𐎭',
            OldPersian::SignDi => '𐎮',
            OldPersian::SignDu => '𐎯',
            OldPersian::SignTha => '𐎰',
            OldPersian::SignPa => '𐎱',
            OldPersian::SignBa => '𐎲',
            OldPersian::SignFa => '𐎳',
            OldPersian::SignNa => '𐎴',
            OldPersian::SignNu => '𐎵',
            OldPersian::SignMa => '𐎶',
            OldPersian::SignMi => '𐎷',
            OldPersian::SignMu => '𐎸',
            OldPersian::SignYa => '𐎹',
            OldPersian::SignVa => '𐎺',
            OldPersian::SignVi => '𐎻',
            OldPersian::SignRa => '𐎼',
            OldPersian::SignRu => '𐎽',
            OldPersian::SignLa => '𐎾',
            OldPersian::SignSa => '𐎿',
            OldPersian::SignZa => '𐏀',
            OldPersian::SignSha => '𐏁',
            OldPersian::SignSsa => '𐏂',
            OldPersian::SignHa => '𐏃',
            OldPersian::SignAuramazdaa => '𐏈',
            OldPersian::SignAuramazdaaDash2 => '𐏉',
            OldPersian::SignAuramazdaaha => '𐏊',
            OldPersian::SignXshaayathiya => '𐏋',
            OldPersian::SignDahyaaush => '𐏌',
            OldPersian::SignDahyaaushDash2 => '𐏍',
            OldPersian::SignBaga => '𐏎',
            OldPersian::SignBuumish => '𐏏',
            OldPersian::WordDivider => '𐏐',
            OldPersian::NumberOne => '𐏑',
            OldPersian::NumberTwo => '𐏒',
            OldPersian::NumberTen => '𐏓',
            OldPersian::NumberTwenty => '𐏔',
            OldPersian::NumberHundred => '𐏕',
        }
    }
}

impl std::convert::TryFrom<char> for OldPersian {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𐎠' => Ok(OldPersian::SignA),
            '𐎡' => Ok(OldPersian::SignI),
            '𐎢' => Ok(OldPersian::SignU),
            '𐎣' => Ok(OldPersian::SignKa),
            '𐎤' => Ok(OldPersian::SignKu),
            '𐎥' => Ok(OldPersian::SignGa),
            '𐎦' => Ok(OldPersian::SignGu),
            '𐎧' => Ok(OldPersian::SignXa),
            '𐎨' => Ok(OldPersian::SignCa),
            '𐎩' => Ok(OldPersian::SignJa),
            '𐎪' => Ok(OldPersian::SignJi),
            '𐎫' => Ok(OldPersian::SignTa),
            '𐎬' => Ok(OldPersian::SignTu),
            '𐎭' => Ok(OldPersian::SignDa),
            '𐎮' => Ok(OldPersian::SignDi),
            '𐎯' => Ok(OldPersian::SignDu),
            '𐎰' => Ok(OldPersian::SignTha),
            '𐎱' => Ok(OldPersian::SignPa),
            '𐎲' => Ok(OldPersian::SignBa),
            '𐎳' => Ok(OldPersian::SignFa),
            '𐎴' => Ok(OldPersian::SignNa),
            '𐎵' => Ok(OldPersian::SignNu),
            '𐎶' => Ok(OldPersian::SignMa),
            '𐎷' => Ok(OldPersian::SignMi),
            '𐎸' => Ok(OldPersian::SignMu),
            '𐎹' => Ok(OldPersian::SignYa),
            '𐎺' => Ok(OldPersian::SignVa),
            '𐎻' => Ok(OldPersian::SignVi),
            '𐎼' => Ok(OldPersian::SignRa),
            '𐎽' => Ok(OldPersian::SignRu),
            '𐎾' => Ok(OldPersian::SignLa),
            '𐎿' => Ok(OldPersian::SignSa),
            '𐏀' => Ok(OldPersian::SignZa),
            '𐏁' => Ok(OldPersian::SignSha),
            '𐏂' => Ok(OldPersian::SignSsa),
            '𐏃' => Ok(OldPersian::SignHa),
            '𐏈' => Ok(OldPersian::SignAuramazdaa),
            '𐏉' => Ok(OldPersian::SignAuramazdaaDash2),
            '𐏊' => Ok(OldPersian::SignAuramazdaaha),
            '𐏋' => Ok(OldPersian::SignXshaayathiya),
            '𐏌' => Ok(OldPersian::SignDahyaaush),
            '𐏍' => Ok(OldPersian::SignDahyaaushDash2),
            '𐏎' => Ok(OldPersian::SignBaga),
            '𐏏' => Ok(OldPersian::SignBuumish),
            '𐏐' => Ok(OldPersian::WordDivider),
            '𐏑' => Ok(OldPersian::NumberOne),
            '𐏒' => Ok(OldPersian::NumberTwo),
            '𐏓' => Ok(OldPersian::NumberTen),
            '𐏔' => Ok(OldPersian::NumberTwenty),
            '𐏕' => Ok(OldPersian::NumberHundred),
            _ => Err(()),
        }
    }
}

impl Into<u32> for OldPersian {
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

impl std::convert::TryFrom<u32> for OldPersian {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for OldPersian {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl OldPersian {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        OldPersian::SignA
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("OldPersian{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
