
/// An enum to represent all characters in the Batak block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Batak {
    /// \u{1bc0}: 'ᯀ'
    LetterA,
    /// \u{1bc1}: 'ᯁ'
    LetterSimalungunA,
    /// \u{1bc2}: 'ᯂ'
    LetterHa,
    /// \u{1bc3}: 'ᯃ'
    LetterSimalungunHa,
    /// \u{1bc4}: 'ᯄ'
    LetterMandailingHa,
    /// \u{1bc5}: 'ᯅ'
    LetterBa,
    /// \u{1bc6}: 'ᯆ'
    LetterKaroBa,
    /// \u{1bc7}: 'ᯇ'
    LetterPa,
    /// \u{1bc8}: 'ᯈ'
    LetterSimalungunPa,
    /// \u{1bc9}: 'ᯉ'
    LetterNa,
    /// \u{1bca}: 'ᯊ'
    LetterMandailingNa,
    /// \u{1bcb}: 'ᯋ'
    LetterWa,
    /// \u{1bcc}: 'ᯌ'
    LetterSimalungunWa,
    /// \u{1bcd}: 'ᯍ'
    LetterPakpakWa,
    /// \u{1bce}: 'ᯎ'
    LetterGa,
    /// \u{1bcf}: 'ᯏ'
    LetterSimalungunGa,
    /// \u{1bd0}: 'ᯐ'
    LetterJa,
    /// \u{1bd1}: 'ᯑ'
    LetterDa,
    /// \u{1bd2}: 'ᯒ'
    LetterRa,
    /// \u{1bd3}: 'ᯓ'
    LetterSimalungunRa,
    /// \u{1bd4}: 'ᯔ'
    LetterMa,
    /// \u{1bd5}: 'ᯕ'
    LetterSimalungunMa,
    /// \u{1bd6}: 'ᯖ'
    LetterSouthernTa,
    /// \u{1bd7}: 'ᯗ'
    LetterNorthernTa,
    /// \u{1bd8}: 'ᯘ'
    LetterSa,
    /// \u{1bd9}: 'ᯙ'
    LetterSimalungunSa,
    /// \u{1bda}: 'ᯚ'
    LetterMandailingSa,
    /// \u{1bdb}: 'ᯛ'
    LetterYa,
    /// \u{1bdc}: 'ᯜ'
    LetterSimalungunYa,
    /// \u{1bdd}: 'ᯝ'
    LetterNga,
    /// \u{1bde}: 'ᯞ'
    LetterLa,
    /// \u{1bdf}: 'ᯟ'
    LetterSimalungunLa,
    /// \u{1be0}: 'ᯠ'
    LetterNya,
    /// \u{1be1}: 'ᯡ'
    LetterCa,
    /// \u{1be2}: 'ᯢ'
    LetterNda,
    /// \u{1be3}: 'ᯣ'
    LetterMba,
    /// \u{1be4}: 'ᯤ'
    LetterI,
    /// \u{1be5}: 'ᯥ'
    LetterU,
    /// \u{1be6}: '᯦'
    SignTompi,
    /// \u{1be7}: 'ᯧ'
    VowelSignE,
    /// \u{1be8}: 'ᯨ'
    VowelSignPakpakE,
    /// \u{1be9}: 'ᯩ'
    VowelSignEe,
    /// \u{1bea}: 'ᯪ'
    VowelSignI,
    /// \u{1beb}: 'ᯫ'
    VowelSignKaroI,
    /// \u{1bec}: 'ᯬ'
    VowelSignO,
    /// \u{1bed}: 'ᯭ'
    VowelSignKaroO,
    /// \u{1bee}: 'ᯮ'
    VowelSignU,
    /// \u{1bef}: 'ᯯ'
    VowelSignUForSimalungunSa,
    /// \u{1bf0}: 'ᯰ'
    ConsonantSignNg,
    /// \u{1bf1}: 'ᯱ'
    ConsonantSignH,
    /// \u{1bf2}: '᯲'
    Pangolat,
    /// \u{1bf3}: '᯳'
    Panongonan,
    /// \u{1bfc}: '᯼'
    SymbolBinduNaMetek,
    /// \u{1bfd}: '᯽'
    SymbolBinduPinarboras,
    /// \u{1bfe}: '᯾'
    SymbolBinduJudul,
}

impl Into<char> for Batak {
    fn into(self) -> char {
        match self {
            Batak::LetterA => 'ᯀ',
            Batak::LetterSimalungunA => 'ᯁ',
            Batak::LetterHa => 'ᯂ',
            Batak::LetterSimalungunHa => 'ᯃ',
            Batak::LetterMandailingHa => 'ᯄ',
            Batak::LetterBa => 'ᯅ',
            Batak::LetterKaroBa => 'ᯆ',
            Batak::LetterPa => 'ᯇ',
            Batak::LetterSimalungunPa => 'ᯈ',
            Batak::LetterNa => 'ᯉ',
            Batak::LetterMandailingNa => 'ᯊ',
            Batak::LetterWa => 'ᯋ',
            Batak::LetterSimalungunWa => 'ᯌ',
            Batak::LetterPakpakWa => 'ᯍ',
            Batak::LetterGa => 'ᯎ',
            Batak::LetterSimalungunGa => 'ᯏ',
            Batak::LetterJa => 'ᯐ',
            Batak::LetterDa => 'ᯑ',
            Batak::LetterRa => 'ᯒ',
            Batak::LetterSimalungunRa => 'ᯓ',
            Batak::LetterMa => 'ᯔ',
            Batak::LetterSimalungunMa => 'ᯕ',
            Batak::LetterSouthernTa => 'ᯖ',
            Batak::LetterNorthernTa => 'ᯗ',
            Batak::LetterSa => 'ᯘ',
            Batak::LetterSimalungunSa => 'ᯙ',
            Batak::LetterMandailingSa => 'ᯚ',
            Batak::LetterYa => 'ᯛ',
            Batak::LetterSimalungunYa => 'ᯜ',
            Batak::LetterNga => 'ᯝ',
            Batak::LetterLa => 'ᯞ',
            Batak::LetterSimalungunLa => 'ᯟ',
            Batak::LetterNya => 'ᯠ',
            Batak::LetterCa => 'ᯡ',
            Batak::LetterNda => 'ᯢ',
            Batak::LetterMba => 'ᯣ',
            Batak::LetterI => 'ᯤ',
            Batak::LetterU => 'ᯥ',
            Batak::SignTompi => '᯦',
            Batak::VowelSignE => 'ᯧ',
            Batak::VowelSignPakpakE => 'ᯨ',
            Batak::VowelSignEe => 'ᯩ',
            Batak::VowelSignI => 'ᯪ',
            Batak::VowelSignKaroI => 'ᯫ',
            Batak::VowelSignO => 'ᯬ',
            Batak::VowelSignKaroO => 'ᯭ',
            Batak::VowelSignU => 'ᯮ',
            Batak::VowelSignUForSimalungunSa => 'ᯯ',
            Batak::ConsonantSignNg => 'ᯰ',
            Batak::ConsonantSignH => 'ᯱ',
            Batak::Pangolat => '᯲',
            Batak::Panongonan => '᯳',
            Batak::SymbolBinduNaMetek => '᯼',
            Batak::SymbolBinduPinarboras => '᯽',
            Batak::SymbolBinduJudul => '᯾',
        }
    }
}

impl std::convert::TryFrom<char> for Batak {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ᯀ' => Ok(Batak::LetterA),
            'ᯁ' => Ok(Batak::LetterSimalungunA),
            'ᯂ' => Ok(Batak::LetterHa),
            'ᯃ' => Ok(Batak::LetterSimalungunHa),
            'ᯄ' => Ok(Batak::LetterMandailingHa),
            'ᯅ' => Ok(Batak::LetterBa),
            'ᯆ' => Ok(Batak::LetterKaroBa),
            'ᯇ' => Ok(Batak::LetterPa),
            'ᯈ' => Ok(Batak::LetterSimalungunPa),
            'ᯉ' => Ok(Batak::LetterNa),
            'ᯊ' => Ok(Batak::LetterMandailingNa),
            'ᯋ' => Ok(Batak::LetterWa),
            'ᯌ' => Ok(Batak::LetterSimalungunWa),
            'ᯍ' => Ok(Batak::LetterPakpakWa),
            'ᯎ' => Ok(Batak::LetterGa),
            'ᯏ' => Ok(Batak::LetterSimalungunGa),
            'ᯐ' => Ok(Batak::LetterJa),
            'ᯑ' => Ok(Batak::LetterDa),
            'ᯒ' => Ok(Batak::LetterRa),
            'ᯓ' => Ok(Batak::LetterSimalungunRa),
            'ᯔ' => Ok(Batak::LetterMa),
            'ᯕ' => Ok(Batak::LetterSimalungunMa),
            'ᯖ' => Ok(Batak::LetterSouthernTa),
            'ᯗ' => Ok(Batak::LetterNorthernTa),
            'ᯘ' => Ok(Batak::LetterSa),
            'ᯙ' => Ok(Batak::LetterSimalungunSa),
            'ᯚ' => Ok(Batak::LetterMandailingSa),
            'ᯛ' => Ok(Batak::LetterYa),
            'ᯜ' => Ok(Batak::LetterSimalungunYa),
            'ᯝ' => Ok(Batak::LetterNga),
            'ᯞ' => Ok(Batak::LetterLa),
            'ᯟ' => Ok(Batak::LetterSimalungunLa),
            'ᯠ' => Ok(Batak::LetterNya),
            'ᯡ' => Ok(Batak::LetterCa),
            'ᯢ' => Ok(Batak::LetterNda),
            'ᯣ' => Ok(Batak::LetterMba),
            'ᯤ' => Ok(Batak::LetterI),
            'ᯥ' => Ok(Batak::LetterU),
            '᯦' => Ok(Batak::SignTompi),
            'ᯧ' => Ok(Batak::VowelSignE),
            'ᯨ' => Ok(Batak::VowelSignPakpakE),
            'ᯩ' => Ok(Batak::VowelSignEe),
            'ᯪ' => Ok(Batak::VowelSignI),
            'ᯫ' => Ok(Batak::VowelSignKaroI),
            'ᯬ' => Ok(Batak::VowelSignO),
            'ᯭ' => Ok(Batak::VowelSignKaroO),
            'ᯮ' => Ok(Batak::VowelSignU),
            'ᯯ' => Ok(Batak::VowelSignUForSimalungunSa),
            'ᯰ' => Ok(Batak::ConsonantSignNg),
            'ᯱ' => Ok(Batak::ConsonantSignH),
            '᯲' => Ok(Batak::Pangolat),
            '᯳' => Ok(Batak::Panongonan),
            '᯼' => Ok(Batak::SymbolBinduNaMetek),
            '᯽' => Ok(Batak::SymbolBinduPinarboras),
            '᯾' => Ok(Batak::SymbolBinduJudul),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Batak {
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

impl std::convert::TryFrom<u32> for Batak {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Batak {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Batak {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Batak::LetterA
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Batak{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
