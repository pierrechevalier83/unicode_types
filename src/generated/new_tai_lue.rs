
/// An enum to represent all characters in the NewTaiLue block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum NewTaiLue {
    /// \u{1980}: 'ᦀ'
    LetterHighQa,
    /// \u{1981}: 'ᦁ'
    LetterLowQa,
    /// \u{1982}: 'ᦂ'
    LetterHighKa,
    /// \u{1983}: 'ᦃ'
    LetterHighXa,
    /// \u{1984}: 'ᦄ'
    LetterHighNga,
    /// \u{1985}: 'ᦅ'
    LetterLowKa,
    /// \u{1986}: 'ᦆ'
    LetterLowXa,
    /// \u{1987}: 'ᦇ'
    LetterLowNga,
    /// \u{1988}: 'ᦈ'
    LetterHighTsa,
    /// \u{1989}: 'ᦉ'
    LetterHighSa,
    /// \u{198a}: 'ᦊ'
    LetterHighYa,
    /// \u{198b}: 'ᦋ'
    LetterLowTsa,
    /// \u{198c}: 'ᦌ'
    LetterLowSa,
    /// \u{198d}: 'ᦍ'
    LetterLowYa,
    /// \u{198e}: 'ᦎ'
    LetterHighTa,
    /// \u{198f}: 'ᦏ'
    LetterHighTha,
    /// \u{1990}: 'ᦐ'
    LetterHighNa,
    /// \u{1991}: 'ᦑ'
    LetterLowTa,
    /// \u{1992}: 'ᦒ'
    LetterLowTha,
    /// \u{1993}: 'ᦓ'
    LetterLowNa,
    /// \u{1994}: 'ᦔ'
    LetterHighPa,
    /// \u{1995}: 'ᦕ'
    LetterHighPha,
    /// \u{1996}: 'ᦖ'
    LetterHighMa,
    /// \u{1997}: 'ᦗ'
    LetterLowPa,
    /// \u{1998}: 'ᦘ'
    LetterLowPha,
    /// \u{1999}: 'ᦙ'
    LetterLowMa,
    /// \u{199a}: 'ᦚ'
    LetterHighFa,
    /// \u{199b}: 'ᦛ'
    LetterHighVa,
    /// \u{199c}: 'ᦜ'
    LetterHighLa,
    /// \u{199d}: 'ᦝ'
    LetterLowFa,
    /// \u{199e}: 'ᦞ'
    LetterLowVa,
    /// \u{199f}: 'ᦟ'
    LetterLowLa,
    /// \u{19a0}: 'ᦠ'
    LetterHighHa,
    /// \u{19a1}: 'ᦡ'
    LetterHighDa,
    /// \u{19a2}: 'ᦢ'
    LetterHighBa,
    /// \u{19a3}: 'ᦣ'
    LetterLowHa,
    /// \u{19a4}: 'ᦤ'
    LetterLowDa,
    /// \u{19a5}: 'ᦥ'
    LetterLowBa,
    /// \u{19a6}: 'ᦦ'
    LetterHighKva,
    /// \u{19a7}: 'ᦧ'
    LetterHighXva,
    /// \u{19a8}: 'ᦨ'
    LetterLowKva,
    /// \u{19a9}: 'ᦩ'
    LetterLowXva,
    /// \u{19aa}: 'ᦪ'
    LetterHighSua,
    /// \u{19ab}: 'ᦫ'
    LetterLowSua,
    /// \u{19b0}: 'ᦰ'
    VowelSignVowelShortener,
    /// \u{19b1}: 'ᦱ'
    VowelSignAa,
    /// \u{19b2}: 'ᦲ'
    VowelSignIi,
    /// \u{19b3}: 'ᦳ'
    VowelSignU,
    /// \u{19b4}: 'ᦴ'
    VowelSignUu,
    /// \u{19b5}: 'ᦵ'
    VowelSignE,
    /// \u{19b6}: 'ᦶ'
    VowelSignAe,
    /// \u{19b7}: 'ᦷ'
    VowelSignO,
    /// \u{19b8}: 'ᦸ'
    VowelSignOa,
    /// \u{19b9}: 'ᦹ'
    VowelSignUe,
    /// \u{19ba}: 'ᦺ'
    VowelSignAy,
    /// \u{19bb}: 'ᦻ'
    VowelSignAay,
    /// \u{19bc}: 'ᦼ'
    VowelSignUy,
    /// \u{19bd}: 'ᦽ'
    VowelSignOy,
    /// \u{19be}: 'ᦾ'
    VowelSignOay,
    /// \u{19bf}: 'ᦿ'
    VowelSignUey,
    /// \u{19c0}: 'ᧀ'
    VowelSignIy,
    /// \u{19c1}: 'ᧁ'
    LetterFinalV,
    /// \u{19c2}: 'ᧂ'
    LetterFinalNg,
    /// \u{19c3}: 'ᧃ'
    LetterFinalN,
    /// \u{19c4}: 'ᧄ'
    LetterFinalM,
    /// \u{19c5}: 'ᧅ'
    LetterFinalK,
    /// \u{19c6}: 'ᧆ'
    LetterFinalD,
    /// \u{19c7}: 'ᧇ'
    LetterFinalB,
    /// \u{19c8}: 'ᧈ'
    ToneMarkDash1,
    /// \u{19c9}: 'ᧉ'
    ToneMarkDash2,
    /// \u{19d0}: '᧐'
    DigitZero,
    /// \u{19d1}: '᧑'
    DigitOne,
    /// \u{19d2}: '᧒'
    DigitTwo,
    /// \u{19d3}: '᧓'
    DigitThree,
    /// \u{19d4}: '᧔'
    DigitFour,
    /// \u{19d5}: '᧕'
    DigitFive,
    /// \u{19d6}: '᧖'
    DigitSix,
    /// \u{19d7}: '᧗'
    DigitSeven,
    /// \u{19d8}: '᧘'
    DigitEight,
    /// \u{19d9}: '᧙'
    DigitNine,
    /// \u{19da}: '᧚'
    ThamDigitOne,
    /// \u{19de}: '᧞'
    SignLae,
}

impl Into<char> for NewTaiLue {
    fn into(self) -> char {
        match self {
            NewTaiLue::LetterHighQa => 'ᦀ',
            NewTaiLue::LetterLowQa => 'ᦁ',
            NewTaiLue::LetterHighKa => 'ᦂ',
            NewTaiLue::LetterHighXa => 'ᦃ',
            NewTaiLue::LetterHighNga => 'ᦄ',
            NewTaiLue::LetterLowKa => 'ᦅ',
            NewTaiLue::LetterLowXa => 'ᦆ',
            NewTaiLue::LetterLowNga => 'ᦇ',
            NewTaiLue::LetterHighTsa => 'ᦈ',
            NewTaiLue::LetterHighSa => 'ᦉ',
            NewTaiLue::LetterHighYa => 'ᦊ',
            NewTaiLue::LetterLowTsa => 'ᦋ',
            NewTaiLue::LetterLowSa => 'ᦌ',
            NewTaiLue::LetterLowYa => 'ᦍ',
            NewTaiLue::LetterHighTa => 'ᦎ',
            NewTaiLue::LetterHighTha => 'ᦏ',
            NewTaiLue::LetterHighNa => 'ᦐ',
            NewTaiLue::LetterLowTa => 'ᦑ',
            NewTaiLue::LetterLowTha => 'ᦒ',
            NewTaiLue::LetterLowNa => 'ᦓ',
            NewTaiLue::LetterHighPa => 'ᦔ',
            NewTaiLue::LetterHighPha => 'ᦕ',
            NewTaiLue::LetterHighMa => 'ᦖ',
            NewTaiLue::LetterLowPa => 'ᦗ',
            NewTaiLue::LetterLowPha => 'ᦘ',
            NewTaiLue::LetterLowMa => 'ᦙ',
            NewTaiLue::LetterHighFa => 'ᦚ',
            NewTaiLue::LetterHighVa => 'ᦛ',
            NewTaiLue::LetterHighLa => 'ᦜ',
            NewTaiLue::LetterLowFa => 'ᦝ',
            NewTaiLue::LetterLowVa => 'ᦞ',
            NewTaiLue::LetterLowLa => 'ᦟ',
            NewTaiLue::LetterHighHa => 'ᦠ',
            NewTaiLue::LetterHighDa => 'ᦡ',
            NewTaiLue::LetterHighBa => 'ᦢ',
            NewTaiLue::LetterLowHa => 'ᦣ',
            NewTaiLue::LetterLowDa => 'ᦤ',
            NewTaiLue::LetterLowBa => 'ᦥ',
            NewTaiLue::LetterHighKva => 'ᦦ',
            NewTaiLue::LetterHighXva => 'ᦧ',
            NewTaiLue::LetterLowKva => 'ᦨ',
            NewTaiLue::LetterLowXva => 'ᦩ',
            NewTaiLue::LetterHighSua => 'ᦪ',
            NewTaiLue::LetterLowSua => 'ᦫ',
            NewTaiLue::VowelSignVowelShortener => 'ᦰ',
            NewTaiLue::VowelSignAa => 'ᦱ',
            NewTaiLue::VowelSignIi => 'ᦲ',
            NewTaiLue::VowelSignU => 'ᦳ',
            NewTaiLue::VowelSignUu => 'ᦴ',
            NewTaiLue::VowelSignE => 'ᦵ',
            NewTaiLue::VowelSignAe => 'ᦶ',
            NewTaiLue::VowelSignO => 'ᦷ',
            NewTaiLue::VowelSignOa => 'ᦸ',
            NewTaiLue::VowelSignUe => 'ᦹ',
            NewTaiLue::VowelSignAy => 'ᦺ',
            NewTaiLue::VowelSignAay => 'ᦻ',
            NewTaiLue::VowelSignUy => 'ᦼ',
            NewTaiLue::VowelSignOy => 'ᦽ',
            NewTaiLue::VowelSignOay => 'ᦾ',
            NewTaiLue::VowelSignUey => 'ᦿ',
            NewTaiLue::VowelSignIy => 'ᧀ',
            NewTaiLue::LetterFinalV => 'ᧁ',
            NewTaiLue::LetterFinalNg => 'ᧂ',
            NewTaiLue::LetterFinalN => 'ᧃ',
            NewTaiLue::LetterFinalM => 'ᧄ',
            NewTaiLue::LetterFinalK => 'ᧅ',
            NewTaiLue::LetterFinalD => 'ᧆ',
            NewTaiLue::LetterFinalB => 'ᧇ',
            NewTaiLue::ToneMarkDash1 => 'ᧈ',
            NewTaiLue::ToneMarkDash2 => 'ᧉ',
            NewTaiLue::DigitZero => '᧐',
            NewTaiLue::DigitOne => '᧑',
            NewTaiLue::DigitTwo => '᧒',
            NewTaiLue::DigitThree => '᧓',
            NewTaiLue::DigitFour => '᧔',
            NewTaiLue::DigitFive => '᧕',
            NewTaiLue::DigitSix => '᧖',
            NewTaiLue::DigitSeven => '᧗',
            NewTaiLue::DigitEight => '᧘',
            NewTaiLue::DigitNine => '᧙',
            NewTaiLue::ThamDigitOne => '᧚',
            NewTaiLue::SignLae => '᧞',
        }
    }
}

impl std::convert::TryFrom<char> for NewTaiLue {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ᦀ' => Ok(NewTaiLue::LetterHighQa),
            'ᦁ' => Ok(NewTaiLue::LetterLowQa),
            'ᦂ' => Ok(NewTaiLue::LetterHighKa),
            'ᦃ' => Ok(NewTaiLue::LetterHighXa),
            'ᦄ' => Ok(NewTaiLue::LetterHighNga),
            'ᦅ' => Ok(NewTaiLue::LetterLowKa),
            'ᦆ' => Ok(NewTaiLue::LetterLowXa),
            'ᦇ' => Ok(NewTaiLue::LetterLowNga),
            'ᦈ' => Ok(NewTaiLue::LetterHighTsa),
            'ᦉ' => Ok(NewTaiLue::LetterHighSa),
            'ᦊ' => Ok(NewTaiLue::LetterHighYa),
            'ᦋ' => Ok(NewTaiLue::LetterLowTsa),
            'ᦌ' => Ok(NewTaiLue::LetterLowSa),
            'ᦍ' => Ok(NewTaiLue::LetterLowYa),
            'ᦎ' => Ok(NewTaiLue::LetterHighTa),
            'ᦏ' => Ok(NewTaiLue::LetterHighTha),
            'ᦐ' => Ok(NewTaiLue::LetterHighNa),
            'ᦑ' => Ok(NewTaiLue::LetterLowTa),
            'ᦒ' => Ok(NewTaiLue::LetterLowTha),
            'ᦓ' => Ok(NewTaiLue::LetterLowNa),
            'ᦔ' => Ok(NewTaiLue::LetterHighPa),
            'ᦕ' => Ok(NewTaiLue::LetterHighPha),
            'ᦖ' => Ok(NewTaiLue::LetterHighMa),
            'ᦗ' => Ok(NewTaiLue::LetterLowPa),
            'ᦘ' => Ok(NewTaiLue::LetterLowPha),
            'ᦙ' => Ok(NewTaiLue::LetterLowMa),
            'ᦚ' => Ok(NewTaiLue::LetterHighFa),
            'ᦛ' => Ok(NewTaiLue::LetterHighVa),
            'ᦜ' => Ok(NewTaiLue::LetterHighLa),
            'ᦝ' => Ok(NewTaiLue::LetterLowFa),
            'ᦞ' => Ok(NewTaiLue::LetterLowVa),
            'ᦟ' => Ok(NewTaiLue::LetterLowLa),
            'ᦠ' => Ok(NewTaiLue::LetterHighHa),
            'ᦡ' => Ok(NewTaiLue::LetterHighDa),
            'ᦢ' => Ok(NewTaiLue::LetterHighBa),
            'ᦣ' => Ok(NewTaiLue::LetterLowHa),
            'ᦤ' => Ok(NewTaiLue::LetterLowDa),
            'ᦥ' => Ok(NewTaiLue::LetterLowBa),
            'ᦦ' => Ok(NewTaiLue::LetterHighKva),
            'ᦧ' => Ok(NewTaiLue::LetterHighXva),
            'ᦨ' => Ok(NewTaiLue::LetterLowKva),
            'ᦩ' => Ok(NewTaiLue::LetterLowXva),
            'ᦪ' => Ok(NewTaiLue::LetterHighSua),
            'ᦫ' => Ok(NewTaiLue::LetterLowSua),
            'ᦰ' => Ok(NewTaiLue::VowelSignVowelShortener),
            'ᦱ' => Ok(NewTaiLue::VowelSignAa),
            'ᦲ' => Ok(NewTaiLue::VowelSignIi),
            'ᦳ' => Ok(NewTaiLue::VowelSignU),
            'ᦴ' => Ok(NewTaiLue::VowelSignUu),
            'ᦵ' => Ok(NewTaiLue::VowelSignE),
            'ᦶ' => Ok(NewTaiLue::VowelSignAe),
            'ᦷ' => Ok(NewTaiLue::VowelSignO),
            'ᦸ' => Ok(NewTaiLue::VowelSignOa),
            'ᦹ' => Ok(NewTaiLue::VowelSignUe),
            'ᦺ' => Ok(NewTaiLue::VowelSignAy),
            'ᦻ' => Ok(NewTaiLue::VowelSignAay),
            'ᦼ' => Ok(NewTaiLue::VowelSignUy),
            'ᦽ' => Ok(NewTaiLue::VowelSignOy),
            'ᦾ' => Ok(NewTaiLue::VowelSignOay),
            'ᦿ' => Ok(NewTaiLue::VowelSignUey),
            'ᧀ' => Ok(NewTaiLue::VowelSignIy),
            'ᧁ' => Ok(NewTaiLue::LetterFinalV),
            'ᧂ' => Ok(NewTaiLue::LetterFinalNg),
            'ᧃ' => Ok(NewTaiLue::LetterFinalN),
            'ᧄ' => Ok(NewTaiLue::LetterFinalM),
            'ᧅ' => Ok(NewTaiLue::LetterFinalK),
            'ᧆ' => Ok(NewTaiLue::LetterFinalD),
            'ᧇ' => Ok(NewTaiLue::LetterFinalB),
            'ᧈ' => Ok(NewTaiLue::ToneMarkDash1),
            'ᧉ' => Ok(NewTaiLue::ToneMarkDash2),
            '᧐' => Ok(NewTaiLue::DigitZero),
            '᧑' => Ok(NewTaiLue::DigitOne),
            '᧒' => Ok(NewTaiLue::DigitTwo),
            '᧓' => Ok(NewTaiLue::DigitThree),
            '᧔' => Ok(NewTaiLue::DigitFour),
            '᧕' => Ok(NewTaiLue::DigitFive),
            '᧖' => Ok(NewTaiLue::DigitSix),
            '᧗' => Ok(NewTaiLue::DigitSeven),
            '᧘' => Ok(NewTaiLue::DigitEight),
            '᧙' => Ok(NewTaiLue::DigitNine),
            '᧚' => Ok(NewTaiLue::ThamDigitOne),
            '᧞' => Ok(NewTaiLue::SignLae),
            _ => Err(()),
        }
    }
}

impl Into<u32> for NewTaiLue {
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

impl std::convert::TryFrom<u32> for NewTaiLue {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for NewTaiLue {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl NewTaiLue {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        NewTaiLue::LetterHighQa
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("NewTaiLue{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
