
/// An enum to represent all characters in the SylotiNagri block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SylotiNagri {
    /// \u{a800}: 'ꠀ'
    LetterA,
    /// \u{a801}: 'ꠁ'
    LetterI,
    /// \u{a802}: 'ꠂ'
    SignDvisvara,
    /// \u{a803}: 'ꠃ'
    LetterU,
    /// \u{a804}: 'ꠄ'
    LetterE,
    /// \u{a805}: 'ꠅ'
    LetterO,
    /// \u{a806}: '꠆'
    SignHasanta,
    /// \u{a807}: 'ꠇ'
    LetterKo,
    /// \u{a808}: 'ꠈ'
    LetterKho,
    /// \u{a809}: 'ꠉ'
    LetterGo,
    /// \u{a80a}: 'ꠊ'
    LetterGho,
    /// \u{a80b}: 'ꠋ'
    SignAnusvara,
    /// \u{a80c}: 'ꠌ'
    LetterCo,
    /// \u{a80d}: 'ꠍ'
    LetterCho,
    /// \u{a80e}: 'ꠎ'
    LetterJo,
    /// \u{a80f}: 'ꠏ'
    LetterJho,
    /// \u{a810}: 'ꠐ'
    LetterTto,
    /// \u{a811}: 'ꠑ'
    LetterTtho,
    /// \u{a812}: 'ꠒ'
    LetterDdo,
    /// \u{a813}: 'ꠓ'
    LetterDdho,
    /// \u{a814}: 'ꠔ'
    LetterTo,
    /// \u{a815}: 'ꠕ'
    LetterTho,
    /// \u{a816}: 'ꠖ'
    LetterDo,
    /// \u{a817}: 'ꠗ'
    LetterDho,
    /// \u{a818}: 'ꠘ'
    LetterNo,
    /// \u{a819}: 'ꠙ'
    LetterPo,
    /// \u{a81a}: 'ꠚ'
    LetterPho,
    /// \u{a81b}: 'ꠛ'
    LetterBo,
    /// \u{a81c}: 'ꠜ'
    LetterBho,
    /// \u{a81d}: 'ꠝ'
    LetterMo,
    /// \u{a81e}: 'ꠞ'
    LetterRo,
    /// \u{a81f}: 'ꠟ'
    LetterLo,
    /// \u{a820}: 'ꠠ'
    LetterRro,
    /// \u{a821}: 'ꠡ'
    LetterSo,
    /// \u{a822}: 'ꠢ'
    LetterHo,
    /// \u{a823}: 'ꠣ'
    VowelSignA,
    /// \u{a824}: 'ꠤ'
    VowelSignI,
    /// \u{a825}: 'ꠥ'
    VowelSignU,
    /// \u{a826}: 'ꠦ'
    VowelSignE,
    /// \u{a827}: 'ꠧ'
    VowelSignOo,
    /// \u{a828}: '꠨'
    PoetryMarkDash1,
    /// \u{a829}: '꠩'
    PoetryMarkDash2,
    /// \u{a82a}: '꠪'
    PoetryMarkDash3,
    /// \u{a82b}: '꠫'
    PoetryMarkDash4,
}

impl Into<char> for SylotiNagri {
    fn into(self) -> char {
        match self {
            SylotiNagri::LetterA => 'ꠀ',
            SylotiNagri::LetterI => 'ꠁ',
            SylotiNagri::SignDvisvara => 'ꠂ',
            SylotiNagri::LetterU => 'ꠃ',
            SylotiNagri::LetterE => 'ꠄ',
            SylotiNagri::LetterO => 'ꠅ',
            SylotiNagri::SignHasanta => '꠆',
            SylotiNagri::LetterKo => 'ꠇ',
            SylotiNagri::LetterKho => 'ꠈ',
            SylotiNagri::LetterGo => 'ꠉ',
            SylotiNagri::LetterGho => 'ꠊ',
            SylotiNagri::SignAnusvara => 'ꠋ',
            SylotiNagri::LetterCo => 'ꠌ',
            SylotiNagri::LetterCho => 'ꠍ',
            SylotiNagri::LetterJo => 'ꠎ',
            SylotiNagri::LetterJho => 'ꠏ',
            SylotiNagri::LetterTto => 'ꠐ',
            SylotiNagri::LetterTtho => 'ꠑ',
            SylotiNagri::LetterDdo => 'ꠒ',
            SylotiNagri::LetterDdho => 'ꠓ',
            SylotiNagri::LetterTo => 'ꠔ',
            SylotiNagri::LetterTho => 'ꠕ',
            SylotiNagri::LetterDo => 'ꠖ',
            SylotiNagri::LetterDho => 'ꠗ',
            SylotiNagri::LetterNo => 'ꠘ',
            SylotiNagri::LetterPo => 'ꠙ',
            SylotiNagri::LetterPho => 'ꠚ',
            SylotiNagri::LetterBo => 'ꠛ',
            SylotiNagri::LetterBho => 'ꠜ',
            SylotiNagri::LetterMo => 'ꠝ',
            SylotiNagri::LetterRo => 'ꠞ',
            SylotiNagri::LetterLo => 'ꠟ',
            SylotiNagri::LetterRro => 'ꠠ',
            SylotiNagri::LetterSo => 'ꠡ',
            SylotiNagri::LetterHo => 'ꠢ',
            SylotiNagri::VowelSignA => 'ꠣ',
            SylotiNagri::VowelSignI => 'ꠤ',
            SylotiNagri::VowelSignU => 'ꠥ',
            SylotiNagri::VowelSignE => 'ꠦ',
            SylotiNagri::VowelSignOo => 'ꠧ',
            SylotiNagri::PoetryMarkDash1 => '꠨',
            SylotiNagri::PoetryMarkDash2 => '꠩',
            SylotiNagri::PoetryMarkDash3 => '꠪',
            SylotiNagri::PoetryMarkDash4 => '꠫',
        }
    }
}

impl std::convert::TryFrom<char> for SylotiNagri {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ꠀ' => Ok(SylotiNagri::LetterA),
            'ꠁ' => Ok(SylotiNagri::LetterI),
            'ꠂ' => Ok(SylotiNagri::SignDvisvara),
            'ꠃ' => Ok(SylotiNagri::LetterU),
            'ꠄ' => Ok(SylotiNagri::LetterE),
            'ꠅ' => Ok(SylotiNagri::LetterO),
            '꠆' => Ok(SylotiNagri::SignHasanta),
            'ꠇ' => Ok(SylotiNagri::LetterKo),
            'ꠈ' => Ok(SylotiNagri::LetterKho),
            'ꠉ' => Ok(SylotiNagri::LetterGo),
            'ꠊ' => Ok(SylotiNagri::LetterGho),
            'ꠋ' => Ok(SylotiNagri::SignAnusvara),
            'ꠌ' => Ok(SylotiNagri::LetterCo),
            'ꠍ' => Ok(SylotiNagri::LetterCho),
            'ꠎ' => Ok(SylotiNagri::LetterJo),
            'ꠏ' => Ok(SylotiNagri::LetterJho),
            'ꠐ' => Ok(SylotiNagri::LetterTto),
            'ꠑ' => Ok(SylotiNagri::LetterTtho),
            'ꠒ' => Ok(SylotiNagri::LetterDdo),
            'ꠓ' => Ok(SylotiNagri::LetterDdho),
            'ꠔ' => Ok(SylotiNagri::LetterTo),
            'ꠕ' => Ok(SylotiNagri::LetterTho),
            'ꠖ' => Ok(SylotiNagri::LetterDo),
            'ꠗ' => Ok(SylotiNagri::LetterDho),
            'ꠘ' => Ok(SylotiNagri::LetterNo),
            'ꠙ' => Ok(SylotiNagri::LetterPo),
            'ꠚ' => Ok(SylotiNagri::LetterPho),
            'ꠛ' => Ok(SylotiNagri::LetterBo),
            'ꠜ' => Ok(SylotiNagri::LetterBho),
            'ꠝ' => Ok(SylotiNagri::LetterMo),
            'ꠞ' => Ok(SylotiNagri::LetterRo),
            'ꠟ' => Ok(SylotiNagri::LetterLo),
            'ꠠ' => Ok(SylotiNagri::LetterRro),
            'ꠡ' => Ok(SylotiNagri::LetterSo),
            'ꠢ' => Ok(SylotiNagri::LetterHo),
            'ꠣ' => Ok(SylotiNagri::VowelSignA),
            'ꠤ' => Ok(SylotiNagri::VowelSignI),
            'ꠥ' => Ok(SylotiNagri::VowelSignU),
            'ꠦ' => Ok(SylotiNagri::VowelSignE),
            'ꠧ' => Ok(SylotiNagri::VowelSignOo),
            '꠨' => Ok(SylotiNagri::PoetryMarkDash1),
            '꠩' => Ok(SylotiNagri::PoetryMarkDash2),
            '꠪' => Ok(SylotiNagri::PoetryMarkDash3),
            '꠫' => Ok(SylotiNagri::PoetryMarkDash4),
            _ => Err(()),
        }
    }
}

impl Into<u32> for SylotiNagri {
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

impl std::convert::TryFrom<u32> for SylotiNagri {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for SylotiNagri {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl SylotiNagri {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        SylotiNagri::LetterA
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            SylotiNagri::LetterA => "syloti nagri letter a",
            SylotiNagri::LetterI => "syloti nagri letter i",
            SylotiNagri::SignDvisvara => "syloti nagri sign dvisvara",
            SylotiNagri::LetterU => "syloti nagri letter u",
            SylotiNagri::LetterE => "syloti nagri letter e",
            SylotiNagri::LetterO => "syloti nagri letter o",
            SylotiNagri::SignHasanta => "syloti nagri sign hasanta",
            SylotiNagri::LetterKo => "syloti nagri letter ko",
            SylotiNagri::LetterKho => "syloti nagri letter kho",
            SylotiNagri::LetterGo => "syloti nagri letter go",
            SylotiNagri::LetterGho => "syloti nagri letter gho",
            SylotiNagri::SignAnusvara => "syloti nagri sign anusvara",
            SylotiNagri::LetterCo => "syloti nagri letter co",
            SylotiNagri::LetterCho => "syloti nagri letter cho",
            SylotiNagri::LetterJo => "syloti nagri letter jo",
            SylotiNagri::LetterJho => "syloti nagri letter jho",
            SylotiNagri::LetterTto => "syloti nagri letter tto",
            SylotiNagri::LetterTtho => "syloti nagri letter ttho",
            SylotiNagri::LetterDdo => "syloti nagri letter ddo",
            SylotiNagri::LetterDdho => "syloti nagri letter ddho",
            SylotiNagri::LetterTo => "syloti nagri letter to",
            SylotiNagri::LetterTho => "syloti nagri letter tho",
            SylotiNagri::LetterDo => "syloti nagri letter do",
            SylotiNagri::LetterDho => "syloti nagri letter dho",
            SylotiNagri::LetterNo => "syloti nagri letter no",
            SylotiNagri::LetterPo => "syloti nagri letter po",
            SylotiNagri::LetterPho => "syloti nagri letter pho",
            SylotiNagri::LetterBo => "syloti nagri letter bo",
            SylotiNagri::LetterBho => "syloti nagri letter bho",
            SylotiNagri::LetterMo => "syloti nagri letter mo",
            SylotiNagri::LetterRo => "syloti nagri letter ro",
            SylotiNagri::LetterLo => "syloti nagri letter lo",
            SylotiNagri::LetterRro => "syloti nagri letter rro",
            SylotiNagri::LetterSo => "syloti nagri letter so",
            SylotiNagri::LetterHo => "syloti nagri letter ho",
            SylotiNagri::VowelSignA => "syloti nagri vowel sign a",
            SylotiNagri::VowelSignI => "syloti nagri vowel sign i",
            SylotiNagri::VowelSignU => "syloti nagri vowel sign u",
            SylotiNagri::VowelSignE => "syloti nagri vowel sign e",
            SylotiNagri::VowelSignOo => "syloti nagri vowel sign oo",
            SylotiNagri::PoetryMarkDash1 => "syloti nagri poetry mark-1",
            SylotiNagri::PoetryMarkDash2 => "syloti nagri poetry mark-2",
            SylotiNagri::PoetryMarkDash3 => "syloti nagri poetry mark-3",
            SylotiNagri::PoetryMarkDash4 => "syloti nagri poetry mark-4",
        }
    }
}
