
/// An enum to represent all characters in the Lepcha block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Lepcha {
    /// \u{1c00}: 'ᰀ'
    LetterKa,
    /// \u{1c01}: 'ᰁ'
    LetterKla,
    /// \u{1c02}: 'ᰂ'
    LetterKha,
    /// \u{1c03}: 'ᰃ'
    LetterGa,
    /// \u{1c04}: 'ᰄ'
    LetterGla,
    /// \u{1c05}: 'ᰅ'
    LetterNga,
    /// \u{1c06}: 'ᰆ'
    LetterCa,
    /// \u{1c07}: 'ᰇ'
    LetterCha,
    /// \u{1c08}: 'ᰈ'
    LetterJa,
    /// \u{1c09}: 'ᰉ'
    LetterNya,
    /// \u{1c0a}: 'ᰊ'
    LetterTa,
    /// \u{1c0b}: 'ᰋ'
    LetterTha,
    /// \u{1c0c}: 'ᰌ'
    LetterDa,
    /// \u{1c0d}: 'ᰍ'
    LetterNa,
    /// \u{1c0e}: 'ᰎ'
    LetterPa,
    /// \u{1c0f}: 'ᰏ'
    LetterPla,
    /// \u{1c10}: 'ᰐ'
    LetterPha,
    /// \u{1c11}: 'ᰑ'
    LetterFa,
    /// \u{1c12}: 'ᰒ'
    LetterFla,
    /// \u{1c13}: 'ᰓ'
    LetterBa,
    /// \u{1c14}: 'ᰔ'
    LetterBla,
    /// \u{1c15}: 'ᰕ'
    LetterMa,
    /// \u{1c16}: 'ᰖ'
    LetterMla,
    /// \u{1c17}: 'ᰗ'
    LetterTsa,
    /// \u{1c18}: 'ᰘ'
    LetterTsha,
    /// \u{1c19}: 'ᰙ'
    LetterDza,
    /// \u{1c1a}: 'ᰚ'
    LetterYa,
    /// \u{1c1b}: 'ᰛ'
    LetterRa,
    /// \u{1c1c}: 'ᰜ'
    LetterLa,
    /// \u{1c1d}: 'ᰝ'
    LetterHa,
    /// \u{1c1e}: 'ᰞ'
    LetterHla,
    /// \u{1c1f}: 'ᰟ'
    LetterVa,
    /// \u{1c20}: 'ᰠ'
    LetterSa,
    /// \u{1c21}: 'ᰡ'
    LetterSha,
    /// \u{1c22}: 'ᰢ'
    LetterWa,
    /// \u{1c23}: 'ᰣ'
    LetterA,
    /// \u{1c24}: 'ᰤ'
    SubjoinedLetterYa,
    /// \u{1c25}: 'ᰥ'
    SubjoinedLetterRa,
    /// \u{1c26}: 'ᰦ'
    VowelSignAa,
    /// \u{1c27}: 'ᰧ'
    VowelSignI,
    /// \u{1c28}: 'ᰨ'
    VowelSignO,
    /// \u{1c29}: 'ᰩ'
    VowelSignOo,
    /// \u{1c2a}: 'ᰪ'
    VowelSignU,
    /// \u{1c2b}: 'ᰫ'
    VowelSignUu,
    /// \u{1c2c}: 'ᰬ'
    VowelSignE,
    /// \u{1c2d}: 'ᰭ'
    ConsonantSignK,
    /// \u{1c2e}: 'ᰮ'
    ConsonantSignM,
    /// \u{1c2f}: 'ᰯ'
    ConsonantSignL,
    /// \u{1c30}: 'ᰰ'
    ConsonantSignN,
    /// \u{1c31}: 'ᰱ'
    ConsonantSignP,
    /// \u{1c32}: 'ᰲ'
    ConsonantSignR,
    /// \u{1c33}: 'ᰳ'
    ConsonantSignT,
    /// \u{1c34}: 'ᰴ'
    ConsonantSignNyinDashDo,
    /// \u{1c35}: 'ᰵ'
    ConsonantSignKang,
    /// \u{1c36}: 'ᰶ'
    SignRan,
    /// \u{1c37}: '᰷'
    SignNukta,
    /// \u{1c3b}: '᰻'
    PunctuationTaDashRol,
    /// \u{1c3c}: '᰼'
    PunctuationNyetThyoomTaDashRol,
    /// \u{1c3d}: '᰽'
    PunctuationCerDashWa,
    /// \u{1c3e}: '᰾'
    PunctuationTshookCerDashWa,
    /// \u{1c3f}: '᰿'
    PunctuationTshook,
    /// \u{1c40}: '᱀'
    DigitZero,
    /// \u{1c41}: '᱁'
    DigitOne,
    /// \u{1c42}: '᱂'
    DigitTwo,
    /// \u{1c43}: '᱃'
    DigitThree,
    /// \u{1c44}: '᱄'
    DigitFour,
    /// \u{1c45}: '᱅'
    DigitFive,
    /// \u{1c46}: '᱆'
    DigitSix,
    /// \u{1c47}: '᱇'
    DigitSeven,
    /// \u{1c48}: '᱈'
    DigitEight,
    /// \u{1c49}: '᱉'
    DigitNine,
    /// \u{1c4d}: 'ᱍ'
    LetterTta,
    /// \u{1c4e}: 'ᱎ'
    LetterTtha,
}

impl Into<char> for Lepcha {
    fn into(self) -> char {
        match self {
            Lepcha::LetterKa => 'ᰀ',
            Lepcha::LetterKla => 'ᰁ',
            Lepcha::LetterKha => 'ᰂ',
            Lepcha::LetterGa => 'ᰃ',
            Lepcha::LetterGla => 'ᰄ',
            Lepcha::LetterNga => 'ᰅ',
            Lepcha::LetterCa => 'ᰆ',
            Lepcha::LetterCha => 'ᰇ',
            Lepcha::LetterJa => 'ᰈ',
            Lepcha::LetterNya => 'ᰉ',
            Lepcha::LetterTa => 'ᰊ',
            Lepcha::LetterTha => 'ᰋ',
            Lepcha::LetterDa => 'ᰌ',
            Lepcha::LetterNa => 'ᰍ',
            Lepcha::LetterPa => 'ᰎ',
            Lepcha::LetterPla => 'ᰏ',
            Lepcha::LetterPha => 'ᰐ',
            Lepcha::LetterFa => 'ᰑ',
            Lepcha::LetterFla => 'ᰒ',
            Lepcha::LetterBa => 'ᰓ',
            Lepcha::LetterBla => 'ᰔ',
            Lepcha::LetterMa => 'ᰕ',
            Lepcha::LetterMla => 'ᰖ',
            Lepcha::LetterTsa => 'ᰗ',
            Lepcha::LetterTsha => 'ᰘ',
            Lepcha::LetterDza => 'ᰙ',
            Lepcha::LetterYa => 'ᰚ',
            Lepcha::LetterRa => 'ᰛ',
            Lepcha::LetterLa => 'ᰜ',
            Lepcha::LetterHa => 'ᰝ',
            Lepcha::LetterHla => 'ᰞ',
            Lepcha::LetterVa => 'ᰟ',
            Lepcha::LetterSa => 'ᰠ',
            Lepcha::LetterSha => 'ᰡ',
            Lepcha::LetterWa => 'ᰢ',
            Lepcha::LetterA => 'ᰣ',
            Lepcha::SubjoinedLetterYa => 'ᰤ',
            Lepcha::SubjoinedLetterRa => 'ᰥ',
            Lepcha::VowelSignAa => 'ᰦ',
            Lepcha::VowelSignI => 'ᰧ',
            Lepcha::VowelSignO => 'ᰨ',
            Lepcha::VowelSignOo => 'ᰩ',
            Lepcha::VowelSignU => 'ᰪ',
            Lepcha::VowelSignUu => 'ᰫ',
            Lepcha::VowelSignE => 'ᰬ',
            Lepcha::ConsonantSignK => 'ᰭ',
            Lepcha::ConsonantSignM => 'ᰮ',
            Lepcha::ConsonantSignL => 'ᰯ',
            Lepcha::ConsonantSignN => 'ᰰ',
            Lepcha::ConsonantSignP => 'ᰱ',
            Lepcha::ConsonantSignR => 'ᰲ',
            Lepcha::ConsonantSignT => 'ᰳ',
            Lepcha::ConsonantSignNyinDashDo => 'ᰴ',
            Lepcha::ConsonantSignKang => 'ᰵ',
            Lepcha::SignRan => 'ᰶ',
            Lepcha::SignNukta => '᰷',
            Lepcha::PunctuationTaDashRol => '᰻',
            Lepcha::PunctuationNyetThyoomTaDashRol => '᰼',
            Lepcha::PunctuationCerDashWa => '᰽',
            Lepcha::PunctuationTshookCerDashWa => '᰾',
            Lepcha::PunctuationTshook => '᰿',
            Lepcha::DigitZero => '᱀',
            Lepcha::DigitOne => '᱁',
            Lepcha::DigitTwo => '᱂',
            Lepcha::DigitThree => '᱃',
            Lepcha::DigitFour => '᱄',
            Lepcha::DigitFive => '᱅',
            Lepcha::DigitSix => '᱆',
            Lepcha::DigitSeven => '᱇',
            Lepcha::DigitEight => '᱈',
            Lepcha::DigitNine => '᱉',
            Lepcha::LetterTta => 'ᱍ',
            Lepcha::LetterTtha => 'ᱎ',
        }
    }
}

impl std::convert::TryFrom<char> for Lepcha {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ᰀ' => Ok(Lepcha::LetterKa),
            'ᰁ' => Ok(Lepcha::LetterKla),
            'ᰂ' => Ok(Lepcha::LetterKha),
            'ᰃ' => Ok(Lepcha::LetterGa),
            'ᰄ' => Ok(Lepcha::LetterGla),
            'ᰅ' => Ok(Lepcha::LetterNga),
            'ᰆ' => Ok(Lepcha::LetterCa),
            'ᰇ' => Ok(Lepcha::LetterCha),
            'ᰈ' => Ok(Lepcha::LetterJa),
            'ᰉ' => Ok(Lepcha::LetterNya),
            'ᰊ' => Ok(Lepcha::LetterTa),
            'ᰋ' => Ok(Lepcha::LetterTha),
            'ᰌ' => Ok(Lepcha::LetterDa),
            'ᰍ' => Ok(Lepcha::LetterNa),
            'ᰎ' => Ok(Lepcha::LetterPa),
            'ᰏ' => Ok(Lepcha::LetterPla),
            'ᰐ' => Ok(Lepcha::LetterPha),
            'ᰑ' => Ok(Lepcha::LetterFa),
            'ᰒ' => Ok(Lepcha::LetterFla),
            'ᰓ' => Ok(Lepcha::LetterBa),
            'ᰔ' => Ok(Lepcha::LetterBla),
            'ᰕ' => Ok(Lepcha::LetterMa),
            'ᰖ' => Ok(Lepcha::LetterMla),
            'ᰗ' => Ok(Lepcha::LetterTsa),
            'ᰘ' => Ok(Lepcha::LetterTsha),
            'ᰙ' => Ok(Lepcha::LetterDza),
            'ᰚ' => Ok(Lepcha::LetterYa),
            'ᰛ' => Ok(Lepcha::LetterRa),
            'ᰜ' => Ok(Lepcha::LetterLa),
            'ᰝ' => Ok(Lepcha::LetterHa),
            'ᰞ' => Ok(Lepcha::LetterHla),
            'ᰟ' => Ok(Lepcha::LetterVa),
            'ᰠ' => Ok(Lepcha::LetterSa),
            'ᰡ' => Ok(Lepcha::LetterSha),
            'ᰢ' => Ok(Lepcha::LetterWa),
            'ᰣ' => Ok(Lepcha::LetterA),
            'ᰤ' => Ok(Lepcha::SubjoinedLetterYa),
            'ᰥ' => Ok(Lepcha::SubjoinedLetterRa),
            'ᰦ' => Ok(Lepcha::VowelSignAa),
            'ᰧ' => Ok(Lepcha::VowelSignI),
            'ᰨ' => Ok(Lepcha::VowelSignO),
            'ᰩ' => Ok(Lepcha::VowelSignOo),
            'ᰪ' => Ok(Lepcha::VowelSignU),
            'ᰫ' => Ok(Lepcha::VowelSignUu),
            'ᰬ' => Ok(Lepcha::VowelSignE),
            'ᰭ' => Ok(Lepcha::ConsonantSignK),
            'ᰮ' => Ok(Lepcha::ConsonantSignM),
            'ᰯ' => Ok(Lepcha::ConsonantSignL),
            'ᰰ' => Ok(Lepcha::ConsonantSignN),
            'ᰱ' => Ok(Lepcha::ConsonantSignP),
            'ᰲ' => Ok(Lepcha::ConsonantSignR),
            'ᰳ' => Ok(Lepcha::ConsonantSignT),
            'ᰴ' => Ok(Lepcha::ConsonantSignNyinDashDo),
            'ᰵ' => Ok(Lepcha::ConsonantSignKang),
            'ᰶ' => Ok(Lepcha::SignRan),
            '᰷' => Ok(Lepcha::SignNukta),
            '᰻' => Ok(Lepcha::PunctuationTaDashRol),
            '᰼' => Ok(Lepcha::PunctuationNyetThyoomTaDashRol),
            '᰽' => Ok(Lepcha::PunctuationCerDashWa),
            '᰾' => Ok(Lepcha::PunctuationTshookCerDashWa),
            '᰿' => Ok(Lepcha::PunctuationTshook),
            '᱀' => Ok(Lepcha::DigitZero),
            '᱁' => Ok(Lepcha::DigitOne),
            '᱂' => Ok(Lepcha::DigitTwo),
            '᱃' => Ok(Lepcha::DigitThree),
            '᱄' => Ok(Lepcha::DigitFour),
            '᱅' => Ok(Lepcha::DigitFive),
            '᱆' => Ok(Lepcha::DigitSix),
            '᱇' => Ok(Lepcha::DigitSeven),
            '᱈' => Ok(Lepcha::DigitEight),
            '᱉' => Ok(Lepcha::DigitNine),
            'ᱍ' => Ok(Lepcha::LetterTta),
            'ᱎ' => Ok(Lepcha::LetterTtha),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Lepcha {
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

impl std::convert::TryFrom<u32> for Lepcha {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Lepcha {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Lepcha {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Lepcha::LetterKa
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Lepcha{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
