/// A number of constants to give a name to all characters in this block.
mod constants {
    /// \u{a800}: 'ꠀ'
    pub const LETTER_A: char = 'ꠀ';
    /// \u{a801}: 'ꠁ'
    pub const LETTER_I: char = 'ꠁ';
    /// \u{a802}: 'ꠂ'
    pub const SIGN_DVISVARA: char = 'ꠂ';
    /// \u{a803}: 'ꠃ'
    pub const LETTER_U: char = 'ꠃ';
    /// \u{a804}: 'ꠄ'
    pub const LETTER_E: char = 'ꠄ';
    /// \u{a805}: 'ꠅ'
    pub const LETTER_O: char = 'ꠅ';
    /// \u{a806}: '꠆'
    pub const SIGN_HASANTA: char = '꠆';
    /// \u{a807}: 'ꠇ'
    pub const LETTER_KO: char = 'ꠇ';
    /// \u{a808}: 'ꠈ'
    pub const LETTER_KHO: char = 'ꠈ';
    /// \u{a809}: 'ꠉ'
    pub const LETTER_GO: char = 'ꠉ';
    /// \u{a80a}: 'ꠊ'
    pub const LETTER_GHO: char = 'ꠊ';
    /// \u{a80b}: 'ꠋ'
    pub const SIGN_ANUSVARA: char = 'ꠋ';
    /// \u{a80c}: 'ꠌ'
    pub const LETTER_CO: char = 'ꠌ';
    /// \u{a80d}: 'ꠍ'
    pub const LETTER_CHO: char = 'ꠍ';
    /// \u{a80e}: 'ꠎ'
    pub const LETTER_JO: char = 'ꠎ';
    /// \u{a80f}: 'ꠏ'
    pub const LETTER_JHO: char = 'ꠏ';
    /// \u{a810}: 'ꠐ'
    pub const LETTER_TTO: char = 'ꠐ';
    /// \u{a811}: 'ꠑ'
    pub const LETTER_TTHO: char = 'ꠑ';
    /// \u{a812}: 'ꠒ'
    pub const LETTER_DDO: char = 'ꠒ';
    /// \u{a813}: 'ꠓ'
    pub const LETTER_DDHO: char = 'ꠓ';
    /// \u{a814}: 'ꠔ'
    pub const LETTER_TO: char = 'ꠔ';
    /// \u{a815}: 'ꠕ'
    pub const LETTER_THO: char = 'ꠕ';
    /// \u{a816}: 'ꠖ'
    pub const LETTER_DO: char = 'ꠖ';
    /// \u{a817}: 'ꠗ'
    pub const LETTER_DHO: char = 'ꠗ';
    /// \u{a818}: 'ꠘ'
    pub const LETTER_NO: char = 'ꠘ';
    /// \u{a819}: 'ꠙ'
    pub const LETTER_PO: char = 'ꠙ';
    /// \u{a81a}: 'ꠚ'
    pub const LETTER_PHO: char = 'ꠚ';
    /// \u{a81b}: 'ꠛ'
    pub const LETTER_BO: char = 'ꠛ';
    /// \u{a81c}: 'ꠜ'
    pub const LETTER_BHO: char = 'ꠜ';
    /// \u{a81d}: 'ꠝ'
    pub const LETTER_MO: char = 'ꠝ';
    /// \u{a81e}: 'ꠞ'
    pub const LETTER_RO: char = 'ꠞ';
    /// \u{a81f}: 'ꠟ'
    pub const LETTER_LO: char = 'ꠟ';
    /// \u{a820}: 'ꠠ'
    pub const LETTER_RRO: char = 'ꠠ';
    /// \u{a821}: 'ꠡ'
    pub const LETTER_SO: char = 'ꠡ';
    /// \u{a822}: 'ꠢ'
    pub const LETTER_HO: char = 'ꠢ';
    /// \u{a823}: 'ꠣ'
    pub const VOWEL_SIGN_A: char = 'ꠣ';
    /// \u{a824}: 'ꠤ'
    pub const VOWEL_SIGN_I: char = 'ꠤ';
    /// \u{a825}: 'ꠥ'
    pub const VOWEL_SIGN_U: char = 'ꠥ';
    /// \u{a826}: 'ꠦ'
    pub const VOWEL_SIGN_E: char = 'ꠦ';
    /// \u{a827}: 'ꠧ'
    pub const VOWEL_SIGN_OO: char = 'ꠧ';
    /// \u{a828}: '꠨'
    pub const POETRY_MARK_DASH_1: char = '꠨';
    /// \u{a829}: '꠩'
    pub const POETRY_MARK_DASH_2: char = '꠩';
    /// \u{a82a}: '꠪'
    pub const POETRY_MARK_DASH_3: char = '꠪';
    /// \u{a82b}: '꠫'
    pub const POETRY_MARK_DASH_4: char = '꠫';
}

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
        use constants::*;
        match self {
            SylotiNagri::LetterA => LETTER_A,
            SylotiNagri::LetterI => LETTER_I,
            SylotiNagri::SignDvisvara => SIGN_DVISVARA,
            SylotiNagri::LetterU => LETTER_U,
            SylotiNagri::LetterE => LETTER_E,
            SylotiNagri::LetterO => LETTER_O,
            SylotiNagri::SignHasanta => SIGN_HASANTA,
            SylotiNagri::LetterKo => LETTER_KO,
            SylotiNagri::LetterKho => LETTER_KHO,
            SylotiNagri::LetterGo => LETTER_GO,
            SylotiNagri::LetterGho => LETTER_GHO,
            SylotiNagri::SignAnusvara => SIGN_ANUSVARA,
            SylotiNagri::LetterCo => LETTER_CO,
            SylotiNagri::LetterCho => LETTER_CHO,
            SylotiNagri::LetterJo => LETTER_JO,
            SylotiNagri::LetterJho => LETTER_JHO,
            SylotiNagri::LetterTto => LETTER_TTO,
            SylotiNagri::LetterTtho => LETTER_TTHO,
            SylotiNagri::LetterDdo => LETTER_DDO,
            SylotiNagri::LetterDdho => LETTER_DDHO,
            SylotiNagri::LetterTo => LETTER_TO,
            SylotiNagri::LetterTho => LETTER_THO,
            SylotiNagri::LetterDo => LETTER_DO,
            SylotiNagri::LetterDho => LETTER_DHO,
            SylotiNagri::LetterNo => LETTER_NO,
            SylotiNagri::LetterPo => LETTER_PO,
            SylotiNagri::LetterPho => LETTER_PHO,
            SylotiNagri::LetterBo => LETTER_BO,
            SylotiNagri::LetterBho => LETTER_BHO,
            SylotiNagri::LetterMo => LETTER_MO,
            SylotiNagri::LetterRo => LETTER_RO,
            SylotiNagri::LetterLo => LETTER_LO,
            SylotiNagri::LetterRro => LETTER_RRO,
            SylotiNagri::LetterSo => LETTER_SO,
            SylotiNagri::LetterHo => LETTER_HO,
            SylotiNagri::VowelSignA => VOWEL_SIGN_A,
            SylotiNagri::VowelSignI => VOWEL_SIGN_I,
            SylotiNagri::VowelSignU => VOWEL_SIGN_U,
            SylotiNagri::VowelSignE => VOWEL_SIGN_E,
            SylotiNagri::VowelSignOo => VOWEL_SIGN_OO,
            SylotiNagri::PoetryMarkDash1 => POETRY_MARK_DASH_1,
            SylotiNagri::PoetryMarkDash2 => POETRY_MARK_DASH_2,
            SylotiNagri::PoetryMarkDash3 => POETRY_MARK_DASH_3,
            SylotiNagri::PoetryMarkDash4 => POETRY_MARK_DASH_4,
        }
    }
}

impl std::convert::TryFrom<char> for SylotiNagri {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_A => Ok(SylotiNagri::LetterA),
            LETTER_I => Ok(SylotiNagri::LetterI),
            SIGN_DVISVARA => Ok(SylotiNagri::SignDvisvara),
            LETTER_U => Ok(SylotiNagri::LetterU),
            LETTER_E => Ok(SylotiNagri::LetterE),
            LETTER_O => Ok(SylotiNagri::LetterO),
            SIGN_HASANTA => Ok(SylotiNagri::SignHasanta),
            LETTER_KO => Ok(SylotiNagri::LetterKo),
            LETTER_KHO => Ok(SylotiNagri::LetterKho),
            LETTER_GO => Ok(SylotiNagri::LetterGo),
            LETTER_GHO => Ok(SylotiNagri::LetterGho),
            SIGN_ANUSVARA => Ok(SylotiNagri::SignAnusvara),
            LETTER_CO => Ok(SylotiNagri::LetterCo),
            LETTER_CHO => Ok(SylotiNagri::LetterCho),
            LETTER_JO => Ok(SylotiNagri::LetterJo),
            LETTER_JHO => Ok(SylotiNagri::LetterJho),
            LETTER_TTO => Ok(SylotiNagri::LetterTto),
            LETTER_TTHO => Ok(SylotiNagri::LetterTtho),
            LETTER_DDO => Ok(SylotiNagri::LetterDdo),
            LETTER_DDHO => Ok(SylotiNagri::LetterDdho),
            LETTER_TO => Ok(SylotiNagri::LetterTo),
            LETTER_THO => Ok(SylotiNagri::LetterTho),
            LETTER_DO => Ok(SylotiNagri::LetterDo),
            LETTER_DHO => Ok(SylotiNagri::LetterDho),
            LETTER_NO => Ok(SylotiNagri::LetterNo),
            LETTER_PO => Ok(SylotiNagri::LetterPo),
            LETTER_PHO => Ok(SylotiNagri::LetterPho),
            LETTER_BO => Ok(SylotiNagri::LetterBo),
            LETTER_BHO => Ok(SylotiNagri::LetterBho),
            LETTER_MO => Ok(SylotiNagri::LetterMo),
            LETTER_RO => Ok(SylotiNagri::LetterRo),
            LETTER_LO => Ok(SylotiNagri::LetterLo),
            LETTER_RRO => Ok(SylotiNagri::LetterRro),
            LETTER_SO => Ok(SylotiNagri::LetterSo),
            LETTER_HO => Ok(SylotiNagri::LetterHo),
            VOWEL_SIGN_A => Ok(SylotiNagri::VowelSignA),
            VOWEL_SIGN_I => Ok(SylotiNagri::VowelSignI),
            VOWEL_SIGN_U => Ok(SylotiNagri::VowelSignU),
            VOWEL_SIGN_E => Ok(SylotiNagri::VowelSignE),
            VOWEL_SIGN_OO => Ok(SylotiNagri::VowelSignOo),
            POETRY_MARK_DASH_1 => Ok(SylotiNagri::PoetryMarkDash1),
            POETRY_MARK_DASH_2 => Ok(SylotiNagri::PoetryMarkDash2),
            POETRY_MARK_DASH_3 => Ok(SylotiNagri::PoetryMarkDash3),
            POETRY_MARK_DASH_4 => Ok(SylotiNagri::PoetryMarkDash4),
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
