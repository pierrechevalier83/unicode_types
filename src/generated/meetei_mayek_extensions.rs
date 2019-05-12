/// A number of constants to give a name to all characters in this block.
mod constants {
    /// \u{aae0}: 'ꫠ'
    pub const MEETEI_MAYEK_LETTER_E: char = 'ꫠ';
    /// \u{aae1}: 'ꫡ'
    pub const MEETEI_MAYEK_LETTER_O: char = 'ꫡ';
    /// \u{aae2}: 'ꫢ'
    pub const MEETEI_MAYEK_LETTER_CHA: char = 'ꫢ';
    /// \u{aae3}: 'ꫣ'
    pub const MEETEI_MAYEK_LETTER_NYA: char = 'ꫣ';
    /// \u{aae4}: 'ꫤ'
    pub const MEETEI_MAYEK_LETTER_TTA: char = 'ꫤ';
    /// \u{aae5}: 'ꫥ'
    pub const MEETEI_MAYEK_LETTER_TTHA: char = 'ꫥ';
    /// \u{aae6}: 'ꫦ'
    pub const MEETEI_MAYEK_LETTER_DDA: char = 'ꫦ';
    /// \u{aae7}: 'ꫧ'
    pub const MEETEI_MAYEK_LETTER_DDHA: char = 'ꫧ';
    /// \u{aae8}: 'ꫨ'
    pub const MEETEI_MAYEK_LETTER_NNA: char = 'ꫨ';
    /// \u{aae9}: 'ꫩ'
    pub const MEETEI_MAYEK_LETTER_SHA: char = 'ꫩ';
    /// \u{aaea}: 'ꫪ'
    pub const MEETEI_MAYEK_LETTER_SSA: char = 'ꫪ';
    /// \u{aaeb}: 'ꫫ'
    pub const MEETEI_MAYEK_VOWEL_SIGN_II: char = 'ꫫ';
    /// \u{aaec}: 'ꫬ'
    pub const MEETEI_MAYEK_VOWEL_SIGN_UU: char = 'ꫬ';
    /// \u{aaed}: 'ꫭ'
    pub const MEETEI_MAYEK_VOWEL_SIGN_AAI: char = 'ꫭ';
    /// \u{aaee}: 'ꫮ'
    pub const MEETEI_MAYEK_VOWEL_SIGN_AU: char = 'ꫮ';
    /// \u{aaef}: 'ꫯ'
    pub const MEETEI_MAYEK_VOWEL_SIGN_AAU: char = 'ꫯ';
    /// \u{aaf0}: '꫰'
    pub const MEETEI_MAYEK_CHEIKHAN: char = '꫰';
    /// \u{aaf1}: '꫱'
    pub const MEETEI_MAYEK_AHANG_KHUDAM: char = '꫱';
    /// \u{aaf2}: 'ꫲ'
    pub const MEETEI_MAYEK_ANJI: char = 'ꫲ';
    /// \u{aaf3}: 'ꫳ'
    pub const MEETEI_MAYEK_SYLLABLE_REPETITION_MARK: char = 'ꫳ';
    /// \u{aaf4}: 'ꫴ'
    pub const MEETEI_MAYEK_WORD_REPETITION_MARK: char = 'ꫴ';
    /// \u{aaf5}: 'ꫵ'
    pub const MEETEI_MAYEK_VOWEL_SIGN_VISARGA: char = 'ꫵ';
    /// \u{aaf6}: '꫶'
    pub const MEETEI_MAYEK_VIRAMA: char = '꫶';
}

/// An enum to represent all characters in the MeeteiMayekExtensions block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum MeeteiMayekExtensions {
    /// \u{aae0}: 'ꫠ'
    MeeteiMayekLetterE,
    /// \u{aae1}: 'ꫡ'
    MeeteiMayekLetterO,
    /// \u{aae2}: 'ꫢ'
    MeeteiMayekLetterCha,
    /// \u{aae3}: 'ꫣ'
    MeeteiMayekLetterNya,
    /// \u{aae4}: 'ꫤ'
    MeeteiMayekLetterTta,
    /// \u{aae5}: 'ꫥ'
    MeeteiMayekLetterTtha,
    /// \u{aae6}: 'ꫦ'
    MeeteiMayekLetterDda,
    /// \u{aae7}: 'ꫧ'
    MeeteiMayekLetterDdha,
    /// \u{aae8}: 'ꫨ'
    MeeteiMayekLetterNna,
    /// \u{aae9}: 'ꫩ'
    MeeteiMayekLetterSha,
    /// \u{aaea}: 'ꫪ'
    MeeteiMayekLetterSsa,
    /// \u{aaeb}: 'ꫫ'
    MeeteiMayekVowelSignIi,
    /// \u{aaec}: 'ꫬ'
    MeeteiMayekVowelSignUu,
    /// \u{aaed}: 'ꫭ'
    MeeteiMayekVowelSignAai,
    /// \u{aaee}: 'ꫮ'
    MeeteiMayekVowelSignAu,
    /// \u{aaef}: 'ꫯ'
    MeeteiMayekVowelSignAau,
    /// \u{aaf0}: '꫰'
    MeeteiMayekCheikhan,
    /// \u{aaf1}: '꫱'
    MeeteiMayekAhangKhudam,
    /// \u{aaf2}: 'ꫲ'
    MeeteiMayekAnji,
    /// \u{aaf3}: 'ꫳ'
    MeeteiMayekSyllableRepetitionMark,
    /// \u{aaf4}: 'ꫴ'
    MeeteiMayekWordRepetitionMark,
    /// \u{aaf5}: 'ꫵ'
    MeeteiMayekVowelSignVisarga,
    /// \u{aaf6}: '꫶'
    MeeteiMayekVirama,
}

impl Into<char> for MeeteiMayekExtensions {
    fn into(self) -> char {
        use constants::*;
        match self {
            MeeteiMayekExtensions::MeeteiMayekLetterE => MEETEI_MAYEK_LETTER_E,
            MeeteiMayekExtensions::MeeteiMayekLetterO => MEETEI_MAYEK_LETTER_O,
            MeeteiMayekExtensions::MeeteiMayekLetterCha => MEETEI_MAYEK_LETTER_CHA,
            MeeteiMayekExtensions::MeeteiMayekLetterNya => MEETEI_MAYEK_LETTER_NYA,
            MeeteiMayekExtensions::MeeteiMayekLetterTta => MEETEI_MAYEK_LETTER_TTA,
            MeeteiMayekExtensions::MeeteiMayekLetterTtha => MEETEI_MAYEK_LETTER_TTHA,
            MeeteiMayekExtensions::MeeteiMayekLetterDda => MEETEI_MAYEK_LETTER_DDA,
            MeeteiMayekExtensions::MeeteiMayekLetterDdha => MEETEI_MAYEK_LETTER_DDHA,
            MeeteiMayekExtensions::MeeteiMayekLetterNna => MEETEI_MAYEK_LETTER_NNA,
            MeeteiMayekExtensions::MeeteiMayekLetterSha => MEETEI_MAYEK_LETTER_SHA,
            MeeteiMayekExtensions::MeeteiMayekLetterSsa => MEETEI_MAYEK_LETTER_SSA,
            MeeteiMayekExtensions::MeeteiMayekVowelSignIi => MEETEI_MAYEK_VOWEL_SIGN_II,
            MeeteiMayekExtensions::MeeteiMayekVowelSignUu => MEETEI_MAYEK_VOWEL_SIGN_UU,
            MeeteiMayekExtensions::MeeteiMayekVowelSignAai => MEETEI_MAYEK_VOWEL_SIGN_AAI,
            MeeteiMayekExtensions::MeeteiMayekVowelSignAu => MEETEI_MAYEK_VOWEL_SIGN_AU,
            MeeteiMayekExtensions::MeeteiMayekVowelSignAau => MEETEI_MAYEK_VOWEL_SIGN_AAU,
            MeeteiMayekExtensions::MeeteiMayekCheikhan => MEETEI_MAYEK_CHEIKHAN,
            MeeteiMayekExtensions::MeeteiMayekAhangKhudam => MEETEI_MAYEK_AHANG_KHUDAM,
            MeeteiMayekExtensions::MeeteiMayekAnji => MEETEI_MAYEK_ANJI,
            MeeteiMayekExtensions::MeeteiMayekSyllableRepetitionMark => MEETEI_MAYEK_SYLLABLE_REPETITION_MARK,
            MeeteiMayekExtensions::MeeteiMayekWordRepetitionMark => MEETEI_MAYEK_WORD_REPETITION_MARK,
            MeeteiMayekExtensions::MeeteiMayekVowelSignVisarga => MEETEI_MAYEK_VOWEL_SIGN_VISARGA,
            MeeteiMayekExtensions::MeeteiMayekVirama => MEETEI_MAYEK_VIRAMA,
        }
    }
}

impl std::convert::TryFrom<char> for MeeteiMayekExtensions {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            MEETEI_MAYEK_LETTER_E => Ok(MeeteiMayekExtensions::MeeteiMayekLetterE),
            MEETEI_MAYEK_LETTER_O => Ok(MeeteiMayekExtensions::MeeteiMayekLetterO),
            MEETEI_MAYEK_LETTER_CHA => Ok(MeeteiMayekExtensions::MeeteiMayekLetterCha),
            MEETEI_MAYEK_LETTER_NYA => Ok(MeeteiMayekExtensions::MeeteiMayekLetterNya),
            MEETEI_MAYEK_LETTER_TTA => Ok(MeeteiMayekExtensions::MeeteiMayekLetterTta),
            MEETEI_MAYEK_LETTER_TTHA => Ok(MeeteiMayekExtensions::MeeteiMayekLetterTtha),
            MEETEI_MAYEK_LETTER_DDA => Ok(MeeteiMayekExtensions::MeeteiMayekLetterDda),
            MEETEI_MAYEK_LETTER_DDHA => Ok(MeeteiMayekExtensions::MeeteiMayekLetterDdha),
            MEETEI_MAYEK_LETTER_NNA => Ok(MeeteiMayekExtensions::MeeteiMayekLetterNna),
            MEETEI_MAYEK_LETTER_SHA => Ok(MeeteiMayekExtensions::MeeteiMayekLetterSha),
            MEETEI_MAYEK_LETTER_SSA => Ok(MeeteiMayekExtensions::MeeteiMayekLetterSsa),
            MEETEI_MAYEK_VOWEL_SIGN_II => Ok(MeeteiMayekExtensions::MeeteiMayekVowelSignIi),
            MEETEI_MAYEK_VOWEL_SIGN_UU => Ok(MeeteiMayekExtensions::MeeteiMayekVowelSignUu),
            MEETEI_MAYEK_VOWEL_SIGN_AAI => Ok(MeeteiMayekExtensions::MeeteiMayekVowelSignAai),
            MEETEI_MAYEK_VOWEL_SIGN_AU => Ok(MeeteiMayekExtensions::MeeteiMayekVowelSignAu),
            MEETEI_MAYEK_VOWEL_SIGN_AAU => Ok(MeeteiMayekExtensions::MeeteiMayekVowelSignAau),
            MEETEI_MAYEK_CHEIKHAN => Ok(MeeteiMayekExtensions::MeeteiMayekCheikhan),
            MEETEI_MAYEK_AHANG_KHUDAM => Ok(MeeteiMayekExtensions::MeeteiMayekAhangKhudam),
            MEETEI_MAYEK_ANJI => Ok(MeeteiMayekExtensions::MeeteiMayekAnji),
            MEETEI_MAYEK_SYLLABLE_REPETITION_MARK => Ok(MeeteiMayekExtensions::MeeteiMayekSyllableRepetitionMark),
            MEETEI_MAYEK_WORD_REPETITION_MARK => Ok(MeeteiMayekExtensions::MeeteiMayekWordRepetitionMark),
            MEETEI_MAYEK_VOWEL_SIGN_VISARGA => Ok(MeeteiMayekExtensions::MeeteiMayekVowelSignVisarga),
            MEETEI_MAYEK_VIRAMA => Ok(MeeteiMayekExtensions::MeeteiMayekVirama),
            _ => Err(()),
        }
    }
}

impl Into<u32> for MeeteiMayekExtensions {
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

impl std::convert::TryFrom<u32> for MeeteiMayekExtensions {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for MeeteiMayekExtensions {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl MeeteiMayekExtensions {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        MeeteiMayekExtensions::MeeteiMayekLetterE
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            MeeteiMayekExtensions::MeeteiMayekLetterE => "meetei mayek letter e",
            MeeteiMayekExtensions::MeeteiMayekLetterO => "meetei mayek letter o",
            MeeteiMayekExtensions::MeeteiMayekLetterCha => "meetei mayek letter cha",
            MeeteiMayekExtensions::MeeteiMayekLetterNya => "meetei mayek letter nya",
            MeeteiMayekExtensions::MeeteiMayekLetterTta => "meetei mayek letter tta",
            MeeteiMayekExtensions::MeeteiMayekLetterTtha => "meetei mayek letter ttha",
            MeeteiMayekExtensions::MeeteiMayekLetterDda => "meetei mayek letter dda",
            MeeteiMayekExtensions::MeeteiMayekLetterDdha => "meetei mayek letter ddha",
            MeeteiMayekExtensions::MeeteiMayekLetterNna => "meetei mayek letter nna",
            MeeteiMayekExtensions::MeeteiMayekLetterSha => "meetei mayek letter sha",
            MeeteiMayekExtensions::MeeteiMayekLetterSsa => "meetei mayek letter ssa",
            MeeteiMayekExtensions::MeeteiMayekVowelSignIi => "meetei mayek vowel sign ii",
            MeeteiMayekExtensions::MeeteiMayekVowelSignUu => "meetei mayek vowel sign uu",
            MeeteiMayekExtensions::MeeteiMayekVowelSignAai => "meetei mayek vowel sign aai",
            MeeteiMayekExtensions::MeeteiMayekVowelSignAu => "meetei mayek vowel sign au",
            MeeteiMayekExtensions::MeeteiMayekVowelSignAau => "meetei mayek vowel sign aau",
            MeeteiMayekExtensions::MeeteiMayekCheikhan => "meetei mayek cheikhan",
            MeeteiMayekExtensions::MeeteiMayekAhangKhudam => "meetei mayek ahang khudam",
            MeeteiMayekExtensions::MeeteiMayekAnji => "meetei mayek anji",
            MeeteiMayekExtensions::MeeteiMayekSyllableRepetitionMark => "meetei mayek syllable repetition mark",
            MeeteiMayekExtensions::MeeteiMayekWordRepetitionMark => "meetei mayek word repetition mark",
            MeeteiMayekExtensions::MeeteiMayekVowelSignVisarga => "meetei mayek vowel sign visarga",
            MeeteiMayekExtensions::MeeteiMayekVirama => "meetei mayek virama",
        }
    }
}
