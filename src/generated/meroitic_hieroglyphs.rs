
/// An enum to represent all characters in the MeroiticHieroglyphs block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum MeroiticHieroglyphs {
    /// \u{10980}: '𐦀'
    MeroiticHieroglyphicLetterA,
    /// \u{10981}: '𐦁'
    MeroiticHieroglyphicLetterE,
    /// \u{10982}: '𐦂'
    MeroiticHieroglyphicLetterI,
    /// \u{10983}: '𐦃'
    MeroiticHieroglyphicLetterO,
    /// \u{10984}: '𐦄'
    MeroiticHieroglyphicLetterYa,
    /// \u{10985}: '𐦅'
    MeroiticHieroglyphicLetterWa,
    /// \u{10986}: '𐦆'
    MeroiticHieroglyphicLetterBa,
    /// \u{10987}: '𐦇'
    MeroiticHieroglyphicLetterBaDash2,
    /// \u{10988}: '𐦈'
    MeroiticHieroglyphicLetterPa,
    /// \u{10989}: '𐦉'
    MeroiticHieroglyphicLetterMa,
    /// \u{1098a}: '𐦊'
    MeroiticHieroglyphicLetterNa,
    /// \u{1098b}: '𐦋'
    MeroiticHieroglyphicLetterNaDash2,
    /// \u{1098c}: '𐦌'
    MeroiticHieroglyphicLetterNe,
    /// \u{1098d}: '𐦍'
    MeroiticHieroglyphicLetterNeDash2,
    /// \u{1098e}: '𐦎'
    MeroiticHieroglyphicLetterRa,
    /// \u{1098f}: '𐦏'
    MeroiticHieroglyphicLetterRaDash2,
    /// \u{10990}: '𐦐'
    MeroiticHieroglyphicLetterLa,
    /// \u{10991}: '𐦑'
    MeroiticHieroglyphicLetterKha,
    /// \u{10992}: '𐦒'
    MeroiticHieroglyphicLetterHha,
    /// \u{10993}: '𐦓'
    MeroiticHieroglyphicLetterSa,
    /// \u{10994}: '𐦔'
    MeroiticHieroglyphicLetterSaDash2,
    /// \u{10995}: '𐦕'
    MeroiticHieroglyphicLetterSe,
    /// \u{10996}: '𐦖'
    MeroiticHieroglyphicLetterKa,
    /// \u{10997}: '𐦗'
    MeroiticHieroglyphicLetterQa,
    /// \u{10998}: '𐦘'
    MeroiticHieroglyphicLetterTa,
    /// \u{10999}: '𐦙'
    MeroiticHieroglyphicLetterTaDash2,
    /// \u{1099a}: '𐦚'
    MeroiticHieroglyphicLetterTe,
    /// \u{1099b}: '𐦛'
    MeroiticHieroglyphicLetterTeDash2,
    /// \u{1099c}: '𐦜'
    MeroiticHieroglyphicLetterTo,
    /// \u{1099d}: '𐦝'
    MeroiticHieroglyphicLetterDa,
    /// \u{1099e}: '𐦞'
    MeroiticHieroglyphicSymbolVidj,
}

impl Into<char> for MeroiticHieroglyphs {
    fn into(self) -> char {
        match self {
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterA => '𐦀',
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterE => '𐦁',
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterI => '𐦂',
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterO => '𐦃',
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterYa => '𐦄',
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterWa => '𐦅',
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterBa => '𐦆',
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterBaDash2 => '𐦇',
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterPa => '𐦈',
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterMa => '𐦉',
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterNa => '𐦊',
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterNaDash2 => '𐦋',
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterNe => '𐦌',
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterNeDash2 => '𐦍',
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterRa => '𐦎',
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterRaDash2 => '𐦏',
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterLa => '𐦐',
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterKha => '𐦑',
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterHha => '𐦒',
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterSa => '𐦓',
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterSaDash2 => '𐦔',
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterSe => '𐦕',
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterKa => '𐦖',
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterQa => '𐦗',
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterTa => '𐦘',
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterTaDash2 => '𐦙',
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterTe => '𐦚',
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterTeDash2 => '𐦛',
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterTo => '𐦜',
            MeroiticHieroglyphs::MeroiticHieroglyphicLetterDa => '𐦝',
            MeroiticHieroglyphs::MeroiticHieroglyphicSymbolVidj => '𐦞',
        }
    }
}

impl std::convert::TryFrom<char> for MeroiticHieroglyphs {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𐦀' => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterA),
            '𐦁' => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterE),
            '𐦂' => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterI),
            '𐦃' => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterO),
            '𐦄' => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterYa),
            '𐦅' => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterWa),
            '𐦆' => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterBa),
            '𐦇' => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterBaDash2),
            '𐦈' => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterPa),
            '𐦉' => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterMa),
            '𐦊' => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterNa),
            '𐦋' => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterNaDash2),
            '𐦌' => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterNe),
            '𐦍' => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterNeDash2),
            '𐦎' => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterRa),
            '𐦏' => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterRaDash2),
            '𐦐' => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterLa),
            '𐦑' => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterKha),
            '𐦒' => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterHha),
            '𐦓' => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterSa),
            '𐦔' => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterSaDash2),
            '𐦕' => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterSe),
            '𐦖' => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterKa),
            '𐦗' => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterQa),
            '𐦘' => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterTa),
            '𐦙' => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterTaDash2),
            '𐦚' => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterTe),
            '𐦛' => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterTeDash2),
            '𐦜' => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterTo),
            '𐦝' => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicLetterDa),
            '𐦞' => Ok(MeroiticHieroglyphs::MeroiticHieroglyphicSymbolVidj),
            _ => Err(()),
        }
    }
}

impl Into<u32> for MeroiticHieroglyphs {
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

impl std::convert::TryFrom<u32> for MeroiticHieroglyphs {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for MeroiticHieroglyphs {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl MeroiticHieroglyphs {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        MeroiticHieroglyphs::MeroiticHieroglyphicLetterA
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("MeroiticHieroglyphs{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
