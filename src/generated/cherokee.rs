
/// An enum to represent all characters in the Cherokee block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Cherokee {
    /// \u{13a0}: 'Ꭰ'
    LetterA,
    /// \u{13a1}: 'Ꭱ'
    LetterE,
    /// \u{13a2}: 'Ꭲ'
    LetterI,
    /// \u{13a3}: 'Ꭳ'
    LetterO,
    /// \u{13a4}: 'Ꭴ'
    LetterU,
    /// \u{13a5}: 'Ꭵ'
    LetterV,
    /// \u{13a6}: 'Ꭶ'
    LetterGa,
    /// \u{13a7}: 'Ꭷ'
    LetterKa,
    /// \u{13a8}: 'Ꭸ'
    LetterGe,
    /// \u{13a9}: 'Ꭹ'
    LetterGi,
    /// \u{13aa}: 'Ꭺ'
    LetterGo,
    /// \u{13ab}: 'Ꭻ'
    LetterGu,
    /// \u{13ac}: 'Ꭼ'
    LetterGv,
    /// \u{13ad}: 'Ꭽ'
    LetterHa,
    /// \u{13ae}: 'Ꭾ'
    LetterHe,
    /// \u{13af}: 'Ꭿ'
    LetterHi,
    /// \u{13b0}: 'Ꮀ'
    LetterHo,
    /// \u{13b1}: 'Ꮁ'
    LetterHu,
    /// \u{13b2}: 'Ꮂ'
    LetterHv,
    /// \u{13b3}: 'Ꮃ'
    LetterLa,
    /// \u{13b4}: 'Ꮄ'
    LetterLe,
    /// \u{13b5}: 'Ꮅ'
    LetterLi,
    /// \u{13b6}: 'Ꮆ'
    LetterLo,
    /// \u{13b7}: 'Ꮇ'
    LetterLu,
    /// \u{13b8}: 'Ꮈ'
    LetterLv,
    /// \u{13b9}: 'Ꮉ'
    LetterMa,
    /// \u{13ba}: 'Ꮊ'
    LetterMe,
    /// \u{13bb}: 'Ꮋ'
    LetterMi,
    /// \u{13bc}: 'Ꮌ'
    LetterMo,
    /// \u{13bd}: 'Ꮍ'
    LetterMu,
    /// \u{13be}: 'Ꮎ'
    LetterNa,
    /// \u{13bf}: 'Ꮏ'
    LetterHna,
    /// \u{13c0}: 'Ꮐ'
    LetterNah,
    /// \u{13c1}: 'Ꮑ'
    LetterNe,
    /// \u{13c2}: 'Ꮒ'
    LetterNi,
    /// \u{13c3}: 'Ꮓ'
    LetterNo,
    /// \u{13c4}: 'Ꮔ'
    LetterNu,
    /// \u{13c5}: 'Ꮕ'
    LetterNv,
    /// \u{13c6}: 'Ꮖ'
    LetterQua,
    /// \u{13c7}: 'Ꮗ'
    LetterQue,
    /// \u{13c8}: 'Ꮘ'
    LetterQui,
    /// \u{13c9}: 'Ꮙ'
    LetterQuo,
    /// \u{13ca}: 'Ꮚ'
    LetterQuu,
    /// \u{13cb}: 'Ꮛ'
    LetterQuv,
    /// \u{13cc}: 'Ꮜ'
    LetterSa,
    /// \u{13cd}: 'Ꮝ'
    LetterS,
    /// \u{13ce}: 'Ꮞ'
    LetterSe,
    /// \u{13cf}: 'Ꮟ'
    LetterSi,
    /// \u{13d0}: 'Ꮠ'
    LetterSo,
    /// \u{13d1}: 'Ꮡ'
    LetterSu,
    /// \u{13d2}: 'Ꮢ'
    LetterSv,
    /// \u{13d3}: 'Ꮣ'
    LetterDa,
    /// \u{13d4}: 'Ꮤ'
    LetterTa,
    /// \u{13d5}: 'Ꮥ'
    LetterDe,
    /// \u{13d6}: 'Ꮦ'
    LetterTe,
    /// \u{13d7}: 'Ꮧ'
    LetterDi,
    /// \u{13d8}: 'Ꮨ'
    LetterTi,
    /// \u{13d9}: 'Ꮩ'
    LetterDo,
    /// \u{13da}: 'Ꮪ'
    LetterDu,
    /// \u{13db}: 'Ꮫ'
    LetterDv,
    /// \u{13dc}: 'Ꮬ'
    LetterDla,
    /// \u{13dd}: 'Ꮭ'
    LetterTla,
    /// \u{13de}: 'Ꮮ'
    LetterTle,
    /// \u{13df}: 'Ꮯ'
    LetterTli,
    /// \u{13e0}: 'Ꮰ'
    LetterTlo,
    /// \u{13e1}: 'Ꮱ'
    LetterTlu,
    /// \u{13e2}: 'Ꮲ'
    LetterTlv,
    /// \u{13e3}: 'Ꮳ'
    LetterTsa,
    /// \u{13e4}: 'Ꮴ'
    LetterTse,
    /// \u{13e5}: 'Ꮵ'
    LetterTsi,
    /// \u{13e6}: 'Ꮶ'
    LetterTso,
    /// \u{13e7}: 'Ꮷ'
    LetterTsu,
    /// \u{13e8}: 'Ꮸ'
    LetterTsv,
    /// \u{13e9}: 'Ꮹ'
    LetterWa,
    /// \u{13ea}: 'Ꮺ'
    LetterWe,
    /// \u{13eb}: 'Ꮻ'
    LetterWi,
    /// \u{13ec}: 'Ꮼ'
    LetterWo,
    /// \u{13ed}: 'Ꮽ'
    LetterWu,
    /// \u{13ee}: 'Ꮾ'
    LetterWv,
    /// \u{13ef}: 'Ꮿ'
    LetterYa,
    /// \u{13f0}: 'Ᏸ'
    LetterYe,
    /// \u{13f1}: 'Ᏹ'
    LetterYi,
    /// \u{13f2}: 'Ᏺ'
    LetterYo,
    /// \u{13f3}: 'Ᏻ'
    LetterYu,
    /// \u{13f4}: 'Ᏼ'
    LetterYv,
    /// \u{13f5}: 'Ᏽ'
    LetterMv,
    /// \u{13f8}: 'ᏸ'
    SmallLetterYe,
    /// \u{13f9}: 'ᏹ'
    SmallLetterYi,
    /// \u{13fa}: 'ᏺ'
    SmallLetterYo,
    /// \u{13fb}: 'ᏻ'
    SmallLetterYu,
    /// \u{13fc}: 'ᏼ'
    SmallLetterYv,
    /// \u{13fd}: 'ᏽ'
    SmallLetterMv,
}

impl Into<char> for Cherokee {
    fn into(self) -> char {
        match self {
            Cherokee::LetterA => 'Ꭰ',
            Cherokee::LetterE => 'Ꭱ',
            Cherokee::LetterI => 'Ꭲ',
            Cherokee::LetterO => 'Ꭳ',
            Cherokee::LetterU => 'Ꭴ',
            Cherokee::LetterV => 'Ꭵ',
            Cherokee::LetterGa => 'Ꭶ',
            Cherokee::LetterKa => 'Ꭷ',
            Cherokee::LetterGe => 'Ꭸ',
            Cherokee::LetterGi => 'Ꭹ',
            Cherokee::LetterGo => 'Ꭺ',
            Cherokee::LetterGu => 'Ꭻ',
            Cherokee::LetterGv => 'Ꭼ',
            Cherokee::LetterHa => 'Ꭽ',
            Cherokee::LetterHe => 'Ꭾ',
            Cherokee::LetterHi => 'Ꭿ',
            Cherokee::LetterHo => 'Ꮀ',
            Cherokee::LetterHu => 'Ꮁ',
            Cherokee::LetterHv => 'Ꮂ',
            Cherokee::LetterLa => 'Ꮃ',
            Cherokee::LetterLe => 'Ꮄ',
            Cherokee::LetterLi => 'Ꮅ',
            Cherokee::LetterLo => 'Ꮆ',
            Cherokee::LetterLu => 'Ꮇ',
            Cherokee::LetterLv => 'Ꮈ',
            Cherokee::LetterMa => 'Ꮉ',
            Cherokee::LetterMe => 'Ꮊ',
            Cherokee::LetterMi => 'Ꮋ',
            Cherokee::LetterMo => 'Ꮌ',
            Cherokee::LetterMu => 'Ꮍ',
            Cherokee::LetterNa => 'Ꮎ',
            Cherokee::LetterHna => 'Ꮏ',
            Cherokee::LetterNah => 'Ꮐ',
            Cherokee::LetterNe => 'Ꮑ',
            Cherokee::LetterNi => 'Ꮒ',
            Cherokee::LetterNo => 'Ꮓ',
            Cherokee::LetterNu => 'Ꮔ',
            Cherokee::LetterNv => 'Ꮕ',
            Cherokee::LetterQua => 'Ꮖ',
            Cherokee::LetterQue => 'Ꮗ',
            Cherokee::LetterQui => 'Ꮘ',
            Cherokee::LetterQuo => 'Ꮙ',
            Cherokee::LetterQuu => 'Ꮚ',
            Cherokee::LetterQuv => 'Ꮛ',
            Cherokee::LetterSa => 'Ꮜ',
            Cherokee::LetterS => 'Ꮝ',
            Cherokee::LetterSe => 'Ꮞ',
            Cherokee::LetterSi => 'Ꮟ',
            Cherokee::LetterSo => 'Ꮠ',
            Cherokee::LetterSu => 'Ꮡ',
            Cherokee::LetterSv => 'Ꮢ',
            Cherokee::LetterDa => 'Ꮣ',
            Cherokee::LetterTa => 'Ꮤ',
            Cherokee::LetterDe => 'Ꮥ',
            Cherokee::LetterTe => 'Ꮦ',
            Cherokee::LetterDi => 'Ꮧ',
            Cherokee::LetterTi => 'Ꮨ',
            Cherokee::LetterDo => 'Ꮩ',
            Cherokee::LetterDu => 'Ꮪ',
            Cherokee::LetterDv => 'Ꮫ',
            Cherokee::LetterDla => 'Ꮬ',
            Cherokee::LetterTla => 'Ꮭ',
            Cherokee::LetterTle => 'Ꮮ',
            Cherokee::LetterTli => 'Ꮯ',
            Cherokee::LetterTlo => 'Ꮰ',
            Cherokee::LetterTlu => 'Ꮱ',
            Cherokee::LetterTlv => 'Ꮲ',
            Cherokee::LetterTsa => 'Ꮳ',
            Cherokee::LetterTse => 'Ꮴ',
            Cherokee::LetterTsi => 'Ꮵ',
            Cherokee::LetterTso => 'Ꮶ',
            Cherokee::LetterTsu => 'Ꮷ',
            Cherokee::LetterTsv => 'Ꮸ',
            Cherokee::LetterWa => 'Ꮹ',
            Cherokee::LetterWe => 'Ꮺ',
            Cherokee::LetterWi => 'Ꮻ',
            Cherokee::LetterWo => 'Ꮼ',
            Cherokee::LetterWu => 'Ꮽ',
            Cherokee::LetterWv => 'Ꮾ',
            Cherokee::LetterYa => 'Ꮿ',
            Cherokee::LetterYe => 'Ᏸ',
            Cherokee::LetterYi => 'Ᏹ',
            Cherokee::LetterYo => 'Ᏺ',
            Cherokee::LetterYu => 'Ᏻ',
            Cherokee::LetterYv => 'Ᏼ',
            Cherokee::LetterMv => 'Ᏽ',
            Cherokee::SmallLetterYe => 'ᏸ',
            Cherokee::SmallLetterYi => 'ᏹ',
            Cherokee::SmallLetterYo => 'ᏺ',
            Cherokee::SmallLetterYu => 'ᏻ',
            Cherokee::SmallLetterYv => 'ᏼ',
            Cherokee::SmallLetterMv => 'ᏽ',
        }
    }
}

impl std::convert::TryFrom<char> for Cherokee {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'Ꭰ' => Ok(Cherokee::LetterA),
            'Ꭱ' => Ok(Cherokee::LetterE),
            'Ꭲ' => Ok(Cherokee::LetterI),
            'Ꭳ' => Ok(Cherokee::LetterO),
            'Ꭴ' => Ok(Cherokee::LetterU),
            'Ꭵ' => Ok(Cherokee::LetterV),
            'Ꭶ' => Ok(Cherokee::LetterGa),
            'Ꭷ' => Ok(Cherokee::LetterKa),
            'Ꭸ' => Ok(Cherokee::LetterGe),
            'Ꭹ' => Ok(Cherokee::LetterGi),
            'Ꭺ' => Ok(Cherokee::LetterGo),
            'Ꭻ' => Ok(Cherokee::LetterGu),
            'Ꭼ' => Ok(Cherokee::LetterGv),
            'Ꭽ' => Ok(Cherokee::LetterHa),
            'Ꭾ' => Ok(Cherokee::LetterHe),
            'Ꭿ' => Ok(Cherokee::LetterHi),
            'Ꮀ' => Ok(Cherokee::LetterHo),
            'Ꮁ' => Ok(Cherokee::LetterHu),
            'Ꮂ' => Ok(Cherokee::LetterHv),
            'Ꮃ' => Ok(Cherokee::LetterLa),
            'Ꮄ' => Ok(Cherokee::LetterLe),
            'Ꮅ' => Ok(Cherokee::LetterLi),
            'Ꮆ' => Ok(Cherokee::LetterLo),
            'Ꮇ' => Ok(Cherokee::LetterLu),
            'Ꮈ' => Ok(Cherokee::LetterLv),
            'Ꮉ' => Ok(Cherokee::LetterMa),
            'Ꮊ' => Ok(Cherokee::LetterMe),
            'Ꮋ' => Ok(Cherokee::LetterMi),
            'Ꮌ' => Ok(Cherokee::LetterMo),
            'Ꮍ' => Ok(Cherokee::LetterMu),
            'Ꮎ' => Ok(Cherokee::LetterNa),
            'Ꮏ' => Ok(Cherokee::LetterHna),
            'Ꮐ' => Ok(Cherokee::LetterNah),
            'Ꮑ' => Ok(Cherokee::LetterNe),
            'Ꮒ' => Ok(Cherokee::LetterNi),
            'Ꮓ' => Ok(Cherokee::LetterNo),
            'Ꮔ' => Ok(Cherokee::LetterNu),
            'Ꮕ' => Ok(Cherokee::LetterNv),
            'Ꮖ' => Ok(Cherokee::LetterQua),
            'Ꮗ' => Ok(Cherokee::LetterQue),
            'Ꮘ' => Ok(Cherokee::LetterQui),
            'Ꮙ' => Ok(Cherokee::LetterQuo),
            'Ꮚ' => Ok(Cherokee::LetterQuu),
            'Ꮛ' => Ok(Cherokee::LetterQuv),
            'Ꮜ' => Ok(Cherokee::LetterSa),
            'Ꮝ' => Ok(Cherokee::LetterS),
            'Ꮞ' => Ok(Cherokee::LetterSe),
            'Ꮟ' => Ok(Cherokee::LetterSi),
            'Ꮠ' => Ok(Cherokee::LetterSo),
            'Ꮡ' => Ok(Cherokee::LetterSu),
            'Ꮢ' => Ok(Cherokee::LetterSv),
            'Ꮣ' => Ok(Cherokee::LetterDa),
            'Ꮤ' => Ok(Cherokee::LetterTa),
            'Ꮥ' => Ok(Cherokee::LetterDe),
            'Ꮦ' => Ok(Cherokee::LetterTe),
            'Ꮧ' => Ok(Cherokee::LetterDi),
            'Ꮨ' => Ok(Cherokee::LetterTi),
            'Ꮩ' => Ok(Cherokee::LetterDo),
            'Ꮪ' => Ok(Cherokee::LetterDu),
            'Ꮫ' => Ok(Cherokee::LetterDv),
            'Ꮬ' => Ok(Cherokee::LetterDla),
            'Ꮭ' => Ok(Cherokee::LetterTla),
            'Ꮮ' => Ok(Cherokee::LetterTle),
            'Ꮯ' => Ok(Cherokee::LetterTli),
            'Ꮰ' => Ok(Cherokee::LetterTlo),
            'Ꮱ' => Ok(Cherokee::LetterTlu),
            'Ꮲ' => Ok(Cherokee::LetterTlv),
            'Ꮳ' => Ok(Cherokee::LetterTsa),
            'Ꮴ' => Ok(Cherokee::LetterTse),
            'Ꮵ' => Ok(Cherokee::LetterTsi),
            'Ꮶ' => Ok(Cherokee::LetterTso),
            'Ꮷ' => Ok(Cherokee::LetterTsu),
            'Ꮸ' => Ok(Cherokee::LetterTsv),
            'Ꮹ' => Ok(Cherokee::LetterWa),
            'Ꮺ' => Ok(Cherokee::LetterWe),
            'Ꮻ' => Ok(Cherokee::LetterWi),
            'Ꮼ' => Ok(Cherokee::LetterWo),
            'Ꮽ' => Ok(Cherokee::LetterWu),
            'Ꮾ' => Ok(Cherokee::LetterWv),
            'Ꮿ' => Ok(Cherokee::LetterYa),
            'Ᏸ' => Ok(Cherokee::LetterYe),
            'Ᏹ' => Ok(Cherokee::LetterYi),
            'Ᏺ' => Ok(Cherokee::LetterYo),
            'Ᏻ' => Ok(Cherokee::LetterYu),
            'Ᏼ' => Ok(Cherokee::LetterYv),
            'Ᏽ' => Ok(Cherokee::LetterMv),
            'ᏸ' => Ok(Cherokee::SmallLetterYe),
            'ᏹ' => Ok(Cherokee::SmallLetterYi),
            'ᏺ' => Ok(Cherokee::SmallLetterYo),
            'ᏻ' => Ok(Cherokee::SmallLetterYu),
            'ᏼ' => Ok(Cherokee::SmallLetterYv),
            'ᏽ' => Ok(Cherokee::SmallLetterMv),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Cherokee {
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

impl std::convert::TryFrom<u32> for Cherokee {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Cherokee {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Cherokee {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Cherokee::LetterA
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Cherokee{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
