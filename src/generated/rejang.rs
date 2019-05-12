
/// An enum to represent all characters in the Rejang block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Rejang {
    /// \u{a930}: 'ꤰ'
    LetterKa,
    /// \u{a931}: 'ꤱ'
    LetterGa,
    /// \u{a932}: 'ꤲ'
    LetterNga,
    /// \u{a933}: 'ꤳ'
    LetterTa,
    /// \u{a934}: 'ꤴ'
    LetterDa,
    /// \u{a935}: 'ꤵ'
    LetterNa,
    /// \u{a936}: 'ꤶ'
    LetterPa,
    /// \u{a937}: 'ꤷ'
    LetterBa,
    /// \u{a938}: 'ꤸ'
    LetterMa,
    /// \u{a939}: 'ꤹ'
    LetterCa,
    /// \u{a93a}: 'ꤺ'
    LetterJa,
    /// \u{a93b}: 'ꤻ'
    LetterNya,
    /// \u{a93c}: 'ꤼ'
    LetterSa,
    /// \u{a93d}: 'ꤽ'
    LetterRa,
    /// \u{a93e}: 'ꤾ'
    LetterLa,
    /// \u{a93f}: 'ꤿ'
    LetterYa,
    /// \u{a940}: 'ꥀ'
    LetterWa,
    /// \u{a941}: 'ꥁ'
    LetterHa,
    /// \u{a942}: 'ꥂ'
    LetterMba,
    /// \u{a943}: 'ꥃ'
    LetterNgga,
    /// \u{a944}: 'ꥄ'
    LetterNda,
    /// \u{a945}: 'ꥅ'
    LetterNyja,
    /// \u{a946}: 'ꥆ'
    LetterA,
    /// \u{a947}: 'ꥇ'
    VowelSignI,
    /// \u{a948}: 'ꥈ'
    VowelSignU,
    /// \u{a949}: 'ꥉ'
    VowelSignE,
    /// \u{a94a}: 'ꥊ'
    VowelSignAi,
    /// \u{a94b}: 'ꥋ'
    VowelSignO,
    /// \u{a94c}: 'ꥌ'
    VowelSignAu,
    /// \u{a94d}: 'ꥍ'
    VowelSignEu,
    /// \u{a94e}: 'ꥎ'
    VowelSignEa,
    /// \u{a94f}: 'ꥏ'
    ConsonantSignNg,
    /// \u{a950}: 'ꥐ'
    ConsonantSignN,
    /// \u{a951}: 'ꥑ'
    ConsonantSignR,
    /// \u{a952}: 'ꥒ'
    ConsonantSignH,
    /// \u{a953}: '꥓'
    Virama,
}

impl Into<char> for Rejang {
    fn into(self) -> char {
        match self {
            Rejang::LetterKa => 'ꤰ',
            Rejang::LetterGa => 'ꤱ',
            Rejang::LetterNga => 'ꤲ',
            Rejang::LetterTa => 'ꤳ',
            Rejang::LetterDa => 'ꤴ',
            Rejang::LetterNa => 'ꤵ',
            Rejang::LetterPa => 'ꤶ',
            Rejang::LetterBa => 'ꤷ',
            Rejang::LetterMa => 'ꤸ',
            Rejang::LetterCa => 'ꤹ',
            Rejang::LetterJa => 'ꤺ',
            Rejang::LetterNya => 'ꤻ',
            Rejang::LetterSa => 'ꤼ',
            Rejang::LetterRa => 'ꤽ',
            Rejang::LetterLa => 'ꤾ',
            Rejang::LetterYa => 'ꤿ',
            Rejang::LetterWa => 'ꥀ',
            Rejang::LetterHa => 'ꥁ',
            Rejang::LetterMba => 'ꥂ',
            Rejang::LetterNgga => 'ꥃ',
            Rejang::LetterNda => 'ꥄ',
            Rejang::LetterNyja => 'ꥅ',
            Rejang::LetterA => 'ꥆ',
            Rejang::VowelSignI => 'ꥇ',
            Rejang::VowelSignU => 'ꥈ',
            Rejang::VowelSignE => 'ꥉ',
            Rejang::VowelSignAi => 'ꥊ',
            Rejang::VowelSignO => 'ꥋ',
            Rejang::VowelSignAu => 'ꥌ',
            Rejang::VowelSignEu => 'ꥍ',
            Rejang::VowelSignEa => 'ꥎ',
            Rejang::ConsonantSignNg => 'ꥏ',
            Rejang::ConsonantSignN => 'ꥐ',
            Rejang::ConsonantSignR => 'ꥑ',
            Rejang::ConsonantSignH => 'ꥒ',
            Rejang::Virama => '꥓',
        }
    }
}

impl std::convert::TryFrom<char> for Rejang {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ꤰ' => Ok(Rejang::LetterKa),
            'ꤱ' => Ok(Rejang::LetterGa),
            'ꤲ' => Ok(Rejang::LetterNga),
            'ꤳ' => Ok(Rejang::LetterTa),
            'ꤴ' => Ok(Rejang::LetterDa),
            'ꤵ' => Ok(Rejang::LetterNa),
            'ꤶ' => Ok(Rejang::LetterPa),
            'ꤷ' => Ok(Rejang::LetterBa),
            'ꤸ' => Ok(Rejang::LetterMa),
            'ꤹ' => Ok(Rejang::LetterCa),
            'ꤺ' => Ok(Rejang::LetterJa),
            'ꤻ' => Ok(Rejang::LetterNya),
            'ꤼ' => Ok(Rejang::LetterSa),
            'ꤽ' => Ok(Rejang::LetterRa),
            'ꤾ' => Ok(Rejang::LetterLa),
            'ꤿ' => Ok(Rejang::LetterYa),
            'ꥀ' => Ok(Rejang::LetterWa),
            'ꥁ' => Ok(Rejang::LetterHa),
            'ꥂ' => Ok(Rejang::LetterMba),
            'ꥃ' => Ok(Rejang::LetterNgga),
            'ꥄ' => Ok(Rejang::LetterNda),
            'ꥅ' => Ok(Rejang::LetterNyja),
            'ꥆ' => Ok(Rejang::LetterA),
            'ꥇ' => Ok(Rejang::VowelSignI),
            'ꥈ' => Ok(Rejang::VowelSignU),
            'ꥉ' => Ok(Rejang::VowelSignE),
            'ꥊ' => Ok(Rejang::VowelSignAi),
            'ꥋ' => Ok(Rejang::VowelSignO),
            'ꥌ' => Ok(Rejang::VowelSignAu),
            'ꥍ' => Ok(Rejang::VowelSignEu),
            'ꥎ' => Ok(Rejang::VowelSignEa),
            'ꥏ' => Ok(Rejang::ConsonantSignNg),
            'ꥐ' => Ok(Rejang::ConsonantSignN),
            'ꥑ' => Ok(Rejang::ConsonantSignR),
            'ꥒ' => Ok(Rejang::ConsonantSignH),
            '꥓' => Ok(Rejang::Virama),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Rejang {
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

impl std::convert::TryFrom<u32> for Rejang {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Rejang {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Rejang {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Rejang::LetterKa
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Rejang{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
