
/// An enum to represent all characters in the CherokeeSupplement block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum CherokeeSupplement {
    /// \u{ab70}: 'ꭰ'
    CherokeeSmallLetterA,
    /// \u{ab71}: 'ꭱ'
    CherokeeSmallLetterE,
    /// \u{ab72}: 'ꭲ'
    CherokeeSmallLetterI,
    /// \u{ab73}: 'ꭳ'
    CherokeeSmallLetterO,
    /// \u{ab74}: 'ꭴ'
    CherokeeSmallLetterU,
    /// \u{ab75}: 'ꭵ'
    CherokeeSmallLetterV,
    /// \u{ab76}: 'ꭶ'
    CherokeeSmallLetterGa,
    /// \u{ab77}: 'ꭷ'
    CherokeeSmallLetterKa,
    /// \u{ab78}: 'ꭸ'
    CherokeeSmallLetterGe,
    /// \u{ab79}: 'ꭹ'
    CherokeeSmallLetterGi,
    /// \u{ab7a}: 'ꭺ'
    CherokeeSmallLetterGo,
    /// \u{ab7b}: 'ꭻ'
    CherokeeSmallLetterGu,
    /// \u{ab7c}: 'ꭼ'
    CherokeeSmallLetterGv,
    /// \u{ab7d}: 'ꭽ'
    CherokeeSmallLetterHa,
    /// \u{ab7e}: 'ꭾ'
    CherokeeSmallLetterHe,
    /// \u{ab7f}: 'ꭿ'
    CherokeeSmallLetterHi,
    /// \u{ab80}: 'ꮀ'
    CherokeeSmallLetterHo,
    /// \u{ab81}: 'ꮁ'
    CherokeeSmallLetterHu,
    /// \u{ab82}: 'ꮂ'
    CherokeeSmallLetterHv,
    /// \u{ab83}: 'ꮃ'
    CherokeeSmallLetterLa,
    /// \u{ab84}: 'ꮄ'
    CherokeeSmallLetterLe,
    /// \u{ab85}: 'ꮅ'
    CherokeeSmallLetterLi,
    /// \u{ab86}: 'ꮆ'
    CherokeeSmallLetterLo,
    /// \u{ab87}: 'ꮇ'
    CherokeeSmallLetterLu,
    /// \u{ab88}: 'ꮈ'
    CherokeeSmallLetterLv,
    /// \u{ab89}: 'ꮉ'
    CherokeeSmallLetterMa,
    /// \u{ab8a}: 'ꮊ'
    CherokeeSmallLetterMe,
    /// \u{ab8b}: 'ꮋ'
    CherokeeSmallLetterMi,
    /// \u{ab8c}: 'ꮌ'
    CherokeeSmallLetterMo,
    /// \u{ab8d}: 'ꮍ'
    CherokeeSmallLetterMu,
    /// \u{ab8e}: 'ꮎ'
    CherokeeSmallLetterNa,
    /// \u{ab8f}: 'ꮏ'
    CherokeeSmallLetterHna,
    /// \u{ab90}: 'ꮐ'
    CherokeeSmallLetterNah,
    /// \u{ab91}: 'ꮑ'
    CherokeeSmallLetterNe,
    /// \u{ab92}: 'ꮒ'
    CherokeeSmallLetterNi,
    /// \u{ab93}: 'ꮓ'
    CherokeeSmallLetterNo,
    /// \u{ab94}: 'ꮔ'
    CherokeeSmallLetterNu,
    /// \u{ab95}: 'ꮕ'
    CherokeeSmallLetterNv,
    /// \u{ab96}: 'ꮖ'
    CherokeeSmallLetterQua,
    /// \u{ab97}: 'ꮗ'
    CherokeeSmallLetterQue,
    /// \u{ab98}: 'ꮘ'
    CherokeeSmallLetterQui,
    /// \u{ab99}: 'ꮙ'
    CherokeeSmallLetterQuo,
    /// \u{ab9a}: 'ꮚ'
    CherokeeSmallLetterQuu,
    /// \u{ab9b}: 'ꮛ'
    CherokeeSmallLetterQuv,
    /// \u{ab9c}: 'ꮜ'
    CherokeeSmallLetterSa,
    /// \u{ab9d}: 'ꮝ'
    CherokeeSmallLetterS,
    /// \u{ab9e}: 'ꮞ'
    CherokeeSmallLetterSe,
    /// \u{ab9f}: 'ꮟ'
    CherokeeSmallLetterSi,
    /// \u{aba0}: 'ꮠ'
    CherokeeSmallLetterSo,
    /// \u{aba1}: 'ꮡ'
    CherokeeSmallLetterSu,
    /// \u{aba2}: 'ꮢ'
    CherokeeSmallLetterSv,
    /// \u{aba3}: 'ꮣ'
    CherokeeSmallLetterDa,
    /// \u{aba4}: 'ꮤ'
    CherokeeSmallLetterTa,
    /// \u{aba5}: 'ꮥ'
    CherokeeSmallLetterDe,
    /// \u{aba6}: 'ꮦ'
    CherokeeSmallLetterTe,
    /// \u{aba7}: 'ꮧ'
    CherokeeSmallLetterDi,
    /// \u{aba8}: 'ꮨ'
    CherokeeSmallLetterTi,
    /// \u{aba9}: 'ꮩ'
    CherokeeSmallLetterDo,
    /// \u{abaa}: 'ꮪ'
    CherokeeSmallLetterDu,
    /// \u{abab}: 'ꮫ'
    CherokeeSmallLetterDv,
    /// \u{abac}: 'ꮬ'
    CherokeeSmallLetterDla,
    /// \u{abad}: 'ꮭ'
    CherokeeSmallLetterTla,
    /// \u{abae}: 'ꮮ'
    CherokeeSmallLetterTle,
    /// \u{abaf}: 'ꮯ'
    CherokeeSmallLetterTli,
    /// \u{abb0}: 'ꮰ'
    CherokeeSmallLetterTlo,
    /// \u{abb1}: 'ꮱ'
    CherokeeSmallLetterTlu,
    /// \u{abb2}: 'ꮲ'
    CherokeeSmallLetterTlv,
    /// \u{abb3}: 'ꮳ'
    CherokeeSmallLetterTsa,
    /// \u{abb4}: 'ꮴ'
    CherokeeSmallLetterTse,
    /// \u{abb5}: 'ꮵ'
    CherokeeSmallLetterTsi,
    /// \u{abb6}: 'ꮶ'
    CherokeeSmallLetterTso,
    /// \u{abb7}: 'ꮷ'
    CherokeeSmallLetterTsu,
    /// \u{abb8}: 'ꮸ'
    CherokeeSmallLetterTsv,
    /// \u{abb9}: 'ꮹ'
    CherokeeSmallLetterWa,
    /// \u{abba}: 'ꮺ'
    CherokeeSmallLetterWe,
    /// \u{abbb}: 'ꮻ'
    CherokeeSmallLetterWi,
    /// \u{abbc}: 'ꮼ'
    CherokeeSmallLetterWo,
    /// \u{abbd}: 'ꮽ'
    CherokeeSmallLetterWu,
    /// \u{abbe}: 'ꮾ'
    CherokeeSmallLetterWv,
}

impl Into<char> for CherokeeSupplement {
    fn into(self) -> char {
        match self {
            CherokeeSupplement::CherokeeSmallLetterA => 'ꭰ',
            CherokeeSupplement::CherokeeSmallLetterE => 'ꭱ',
            CherokeeSupplement::CherokeeSmallLetterI => 'ꭲ',
            CherokeeSupplement::CherokeeSmallLetterO => 'ꭳ',
            CherokeeSupplement::CherokeeSmallLetterU => 'ꭴ',
            CherokeeSupplement::CherokeeSmallLetterV => 'ꭵ',
            CherokeeSupplement::CherokeeSmallLetterGa => 'ꭶ',
            CherokeeSupplement::CherokeeSmallLetterKa => 'ꭷ',
            CherokeeSupplement::CherokeeSmallLetterGe => 'ꭸ',
            CherokeeSupplement::CherokeeSmallLetterGi => 'ꭹ',
            CherokeeSupplement::CherokeeSmallLetterGo => 'ꭺ',
            CherokeeSupplement::CherokeeSmallLetterGu => 'ꭻ',
            CherokeeSupplement::CherokeeSmallLetterGv => 'ꭼ',
            CherokeeSupplement::CherokeeSmallLetterHa => 'ꭽ',
            CherokeeSupplement::CherokeeSmallLetterHe => 'ꭾ',
            CherokeeSupplement::CherokeeSmallLetterHi => 'ꭿ',
            CherokeeSupplement::CherokeeSmallLetterHo => 'ꮀ',
            CherokeeSupplement::CherokeeSmallLetterHu => 'ꮁ',
            CherokeeSupplement::CherokeeSmallLetterHv => 'ꮂ',
            CherokeeSupplement::CherokeeSmallLetterLa => 'ꮃ',
            CherokeeSupplement::CherokeeSmallLetterLe => 'ꮄ',
            CherokeeSupplement::CherokeeSmallLetterLi => 'ꮅ',
            CherokeeSupplement::CherokeeSmallLetterLo => 'ꮆ',
            CherokeeSupplement::CherokeeSmallLetterLu => 'ꮇ',
            CherokeeSupplement::CherokeeSmallLetterLv => 'ꮈ',
            CherokeeSupplement::CherokeeSmallLetterMa => 'ꮉ',
            CherokeeSupplement::CherokeeSmallLetterMe => 'ꮊ',
            CherokeeSupplement::CherokeeSmallLetterMi => 'ꮋ',
            CherokeeSupplement::CherokeeSmallLetterMo => 'ꮌ',
            CherokeeSupplement::CherokeeSmallLetterMu => 'ꮍ',
            CherokeeSupplement::CherokeeSmallLetterNa => 'ꮎ',
            CherokeeSupplement::CherokeeSmallLetterHna => 'ꮏ',
            CherokeeSupplement::CherokeeSmallLetterNah => 'ꮐ',
            CherokeeSupplement::CherokeeSmallLetterNe => 'ꮑ',
            CherokeeSupplement::CherokeeSmallLetterNi => 'ꮒ',
            CherokeeSupplement::CherokeeSmallLetterNo => 'ꮓ',
            CherokeeSupplement::CherokeeSmallLetterNu => 'ꮔ',
            CherokeeSupplement::CherokeeSmallLetterNv => 'ꮕ',
            CherokeeSupplement::CherokeeSmallLetterQua => 'ꮖ',
            CherokeeSupplement::CherokeeSmallLetterQue => 'ꮗ',
            CherokeeSupplement::CherokeeSmallLetterQui => 'ꮘ',
            CherokeeSupplement::CherokeeSmallLetterQuo => 'ꮙ',
            CherokeeSupplement::CherokeeSmallLetterQuu => 'ꮚ',
            CherokeeSupplement::CherokeeSmallLetterQuv => 'ꮛ',
            CherokeeSupplement::CherokeeSmallLetterSa => 'ꮜ',
            CherokeeSupplement::CherokeeSmallLetterS => 'ꮝ',
            CherokeeSupplement::CherokeeSmallLetterSe => 'ꮞ',
            CherokeeSupplement::CherokeeSmallLetterSi => 'ꮟ',
            CherokeeSupplement::CherokeeSmallLetterSo => 'ꮠ',
            CherokeeSupplement::CherokeeSmallLetterSu => 'ꮡ',
            CherokeeSupplement::CherokeeSmallLetterSv => 'ꮢ',
            CherokeeSupplement::CherokeeSmallLetterDa => 'ꮣ',
            CherokeeSupplement::CherokeeSmallLetterTa => 'ꮤ',
            CherokeeSupplement::CherokeeSmallLetterDe => 'ꮥ',
            CherokeeSupplement::CherokeeSmallLetterTe => 'ꮦ',
            CherokeeSupplement::CherokeeSmallLetterDi => 'ꮧ',
            CherokeeSupplement::CherokeeSmallLetterTi => 'ꮨ',
            CherokeeSupplement::CherokeeSmallLetterDo => 'ꮩ',
            CherokeeSupplement::CherokeeSmallLetterDu => 'ꮪ',
            CherokeeSupplement::CherokeeSmallLetterDv => 'ꮫ',
            CherokeeSupplement::CherokeeSmallLetterDla => 'ꮬ',
            CherokeeSupplement::CherokeeSmallLetterTla => 'ꮭ',
            CherokeeSupplement::CherokeeSmallLetterTle => 'ꮮ',
            CherokeeSupplement::CherokeeSmallLetterTli => 'ꮯ',
            CherokeeSupplement::CherokeeSmallLetterTlo => 'ꮰ',
            CherokeeSupplement::CherokeeSmallLetterTlu => 'ꮱ',
            CherokeeSupplement::CherokeeSmallLetterTlv => 'ꮲ',
            CherokeeSupplement::CherokeeSmallLetterTsa => 'ꮳ',
            CherokeeSupplement::CherokeeSmallLetterTse => 'ꮴ',
            CherokeeSupplement::CherokeeSmallLetterTsi => 'ꮵ',
            CherokeeSupplement::CherokeeSmallLetterTso => 'ꮶ',
            CherokeeSupplement::CherokeeSmallLetterTsu => 'ꮷ',
            CherokeeSupplement::CherokeeSmallLetterTsv => 'ꮸ',
            CherokeeSupplement::CherokeeSmallLetterWa => 'ꮹ',
            CherokeeSupplement::CherokeeSmallLetterWe => 'ꮺ',
            CherokeeSupplement::CherokeeSmallLetterWi => 'ꮻ',
            CherokeeSupplement::CherokeeSmallLetterWo => 'ꮼ',
            CherokeeSupplement::CherokeeSmallLetterWu => 'ꮽ',
            CherokeeSupplement::CherokeeSmallLetterWv => 'ꮾ',
        }
    }
}

impl std::convert::TryFrom<char> for CherokeeSupplement {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ꭰ' => Ok(CherokeeSupplement::CherokeeSmallLetterA),
            'ꭱ' => Ok(CherokeeSupplement::CherokeeSmallLetterE),
            'ꭲ' => Ok(CherokeeSupplement::CherokeeSmallLetterI),
            'ꭳ' => Ok(CherokeeSupplement::CherokeeSmallLetterO),
            'ꭴ' => Ok(CherokeeSupplement::CherokeeSmallLetterU),
            'ꭵ' => Ok(CherokeeSupplement::CherokeeSmallLetterV),
            'ꭶ' => Ok(CherokeeSupplement::CherokeeSmallLetterGa),
            'ꭷ' => Ok(CherokeeSupplement::CherokeeSmallLetterKa),
            'ꭸ' => Ok(CherokeeSupplement::CherokeeSmallLetterGe),
            'ꭹ' => Ok(CherokeeSupplement::CherokeeSmallLetterGi),
            'ꭺ' => Ok(CherokeeSupplement::CherokeeSmallLetterGo),
            'ꭻ' => Ok(CherokeeSupplement::CherokeeSmallLetterGu),
            'ꭼ' => Ok(CherokeeSupplement::CherokeeSmallLetterGv),
            'ꭽ' => Ok(CherokeeSupplement::CherokeeSmallLetterHa),
            'ꭾ' => Ok(CherokeeSupplement::CherokeeSmallLetterHe),
            'ꭿ' => Ok(CherokeeSupplement::CherokeeSmallLetterHi),
            'ꮀ' => Ok(CherokeeSupplement::CherokeeSmallLetterHo),
            'ꮁ' => Ok(CherokeeSupplement::CherokeeSmallLetterHu),
            'ꮂ' => Ok(CherokeeSupplement::CherokeeSmallLetterHv),
            'ꮃ' => Ok(CherokeeSupplement::CherokeeSmallLetterLa),
            'ꮄ' => Ok(CherokeeSupplement::CherokeeSmallLetterLe),
            'ꮅ' => Ok(CherokeeSupplement::CherokeeSmallLetterLi),
            'ꮆ' => Ok(CherokeeSupplement::CherokeeSmallLetterLo),
            'ꮇ' => Ok(CherokeeSupplement::CherokeeSmallLetterLu),
            'ꮈ' => Ok(CherokeeSupplement::CherokeeSmallLetterLv),
            'ꮉ' => Ok(CherokeeSupplement::CherokeeSmallLetterMa),
            'ꮊ' => Ok(CherokeeSupplement::CherokeeSmallLetterMe),
            'ꮋ' => Ok(CherokeeSupplement::CherokeeSmallLetterMi),
            'ꮌ' => Ok(CherokeeSupplement::CherokeeSmallLetterMo),
            'ꮍ' => Ok(CherokeeSupplement::CherokeeSmallLetterMu),
            'ꮎ' => Ok(CherokeeSupplement::CherokeeSmallLetterNa),
            'ꮏ' => Ok(CherokeeSupplement::CherokeeSmallLetterHna),
            'ꮐ' => Ok(CherokeeSupplement::CherokeeSmallLetterNah),
            'ꮑ' => Ok(CherokeeSupplement::CherokeeSmallLetterNe),
            'ꮒ' => Ok(CherokeeSupplement::CherokeeSmallLetterNi),
            'ꮓ' => Ok(CherokeeSupplement::CherokeeSmallLetterNo),
            'ꮔ' => Ok(CherokeeSupplement::CherokeeSmallLetterNu),
            'ꮕ' => Ok(CherokeeSupplement::CherokeeSmallLetterNv),
            'ꮖ' => Ok(CherokeeSupplement::CherokeeSmallLetterQua),
            'ꮗ' => Ok(CherokeeSupplement::CherokeeSmallLetterQue),
            'ꮘ' => Ok(CherokeeSupplement::CherokeeSmallLetterQui),
            'ꮙ' => Ok(CherokeeSupplement::CherokeeSmallLetterQuo),
            'ꮚ' => Ok(CherokeeSupplement::CherokeeSmallLetterQuu),
            'ꮛ' => Ok(CherokeeSupplement::CherokeeSmallLetterQuv),
            'ꮜ' => Ok(CherokeeSupplement::CherokeeSmallLetterSa),
            'ꮝ' => Ok(CherokeeSupplement::CherokeeSmallLetterS),
            'ꮞ' => Ok(CherokeeSupplement::CherokeeSmallLetterSe),
            'ꮟ' => Ok(CherokeeSupplement::CherokeeSmallLetterSi),
            'ꮠ' => Ok(CherokeeSupplement::CherokeeSmallLetterSo),
            'ꮡ' => Ok(CherokeeSupplement::CherokeeSmallLetterSu),
            'ꮢ' => Ok(CherokeeSupplement::CherokeeSmallLetterSv),
            'ꮣ' => Ok(CherokeeSupplement::CherokeeSmallLetterDa),
            'ꮤ' => Ok(CherokeeSupplement::CherokeeSmallLetterTa),
            'ꮥ' => Ok(CherokeeSupplement::CherokeeSmallLetterDe),
            'ꮦ' => Ok(CherokeeSupplement::CherokeeSmallLetterTe),
            'ꮧ' => Ok(CherokeeSupplement::CherokeeSmallLetterDi),
            'ꮨ' => Ok(CherokeeSupplement::CherokeeSmallLetterTi),
            'ꮩ' => Ok(CherokeeSupplement::CherokeeSmallLetterDo),
            'ꮪ' => Ok(CherokeeSupplement::CherokeeSmallLetterDu),
            'ꮫ' => Ok(CherokeeSupplement::CherokeeSmallLetterDv),
            'ꮬ' => Ok(CherokeeSupplement::CherokeeSmallLetterDla),
            'ꮭ' => Ok(CherokeeSupplement::CherokeeSmallLetterTla),
            'ꮮ' => Ok(CherokeeSupplement::CherokeeSmallLetterTle),
            'ꮯ' => Ok(CherokeeSupplement::CherokeeSmallLetterTli),
            'ꮰ' => Ok(CherokeeSupplement::CherokeeSmallLetterTlo),
            'ꮱ' => Ok(CherokeeSupplement::CherokeeSmallLetterTlu),
            'ꮲ' => Ok(CherokeeSupplement::CherokeeSmallLetterTlv),
            'ꮳ' => Ok(CherokeeSupplement::CherokeeSmallLetterTsa),
            'ꮴ' => Ok(CherokeeSupplement::CherokeeSmallLetterTse),
            'ꮵ' => Ok(CherokeeSupplement::CherokeeSmallLetterTsi),
            'ꮶ' => Ok(CherokeeSupplement::CherokeeSmallLetterTso),
            'ꮷ' => Ok(CherokeeSupplement::CherokeeSmallLetterTsu),
            'ꮸ' => Ok(CherokeeSupplement::CherokeeSmallLetterTsv),
            'ꮹ' => Ok(CherokeeSupplement::CherokeeSmallLetterWa),
            'ꮺ' => Ok(CherokeeSupplement::CherokeeSmallLetterWe),
            'ꮻ' => Ok(CherokeeSupplement::CherokeeSmallLetterWi),
            'ꮼ' => Ok(CherokeeSupplement::CherokeeSmallLetterWo),
            'ꮽ' => Ok(CherokeeSupplement::CherokeeSmallLetterWu),
            'ꮾ' => Ok(CherokeeSupplement::CherokeeSmallLetterWv),
            _ => Err(()),
        }
    }
}

impl Into<u32> for CherokeeSupplement {
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

impl std::convert::TryFrom<u32> for CherokeeSupplement {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for CherokeeSupplement {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl CherokeeSupplement {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        CherokeeSupplement::CherokeeSmallLetterA
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("CherokeeSupplement{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
