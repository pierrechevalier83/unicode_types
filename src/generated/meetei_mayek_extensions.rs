
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
        match self {
            MeeteiMayekExtensions::MeeteiMayekLetterE => 'ꫠ',
            MeeteiMayekExtensions::MeeteiMayekLetterO => 'ꫡ',
            MeeteiMayekExtensions::MeeteiMayekLetterCha => 'ꫢ',
            MeeteiMayekExtensions::MeeteiMayekLetterNya => 'ꫣ',
            MeeteiMayekExtensions::MeeteiMayekLetterTta => 'ꫤ',
            MeeteiMayekExtensions::MeeteiMayekLetterTtha => 'ꫥ',
            MeeteiMayekExtensions::MeeteiMayekLetterDda => 'ꫦ',
            MeeteiMayekExtensions::MeeteiMayekLetterDdha => 'ꫧ',
            MeeteiMayekExtensions::MeeteiMayekLetterNna => 'ꫨ',
            MeeteiMayekExtensions::MeeteiMayekLetterSha => 'ꫩ',
            MeeteiMayekExtensions::MeeteiMayekLetterSsa => 'ꫪ',
            MeeteiMayekExtensions::MeeteiMayekVowelSignIi => 'ꫫ',
            MeeteiMayekExtensions::MeeteiMayekVowelSignUu => 'ꫬ',
            MeeteiMayekExtensions::MeeteiMayekVowelSignAai => 'ꫭ',
            MeeteiMayekExtensions::MeeteiMayekVowelSignAu => 'ꫮ',
            MeeteiMayekExtensions::MeeteiMayekVowelSignAau => 'ꫯ',
            MeeteiMayekExtensions::MeeteiMayekCheikhan => '꫰',
            MeeteiMayekExtensions::MeeteiMayekAhangKhudam => '꫱',
            MeeteiMayekExtensions::MeeteiMayekAnji => 'ꫲ',
            MeeteiMayekExtensions::MeeteiMayekSyllableRepetitionMark => 'ꫳ',
            MeeteiMayekExtensions::MeeteiMayekWordRepetitionMark => 'ꫴ',
            MeeteiMayekExtensions::MeeteiMayekVowelSignVisarga => 'ꫵ',
            MeeteiMayekExtensions::MeeteiMayekVirama => '꫶',
        }
    }
}

impl std::convert::TryFrom<char> for MeeteiMayekExtensions {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ꫠ' => Ok(MeeteiMayekExtensions::MeeteiMayekLetterE),
            'ꫡ' => Ok(MeeteiMayekExtensions::MeeteiMayekLetterO),
            'ꫢ' => Ok(MeeteiMayekExtensions::MeeteiMayekLetterCha),
            'ꫣ' => Ok(MeeteiMayekExtensions::MeeteiMayekLetterNya),
            'ꫤ' => Ok(MeeteiMayekExtensions::MeeteiMayekLetterTta),
            'ꫥ' => Ok(MeeteiMayekExtensions::MeeteiMayekLetterTtha),
            'ꫦ' => Ok(MeeteiMayekExtensions::MeeteiMayekLetterDda),
            'ꫧ' => Ok(MeeteiMayekExtensions::MeeteiMayekLetterDdha),
            'ꫨ' => Ok(MeeteiMayekExtensions::MeeteiMayekLetterNna),
            'ꫩ' => Ok(MeeteiMayekExtensions::MeeteiMayekLetterSha),
            'ꫪ' => Ok(MeeteiMayekExtensions::MeeteiMayekLetterSsa),
            'ꫫ' => Ok(MeeteiMayekExtensions::MeeteiMayekVowelSignIi),
            'ꫬ' => Ok(MeeteiMayekExtensions::MeeteiMayekVowelSignUu),
            'ꫭ' => Ok(MeeteiMayekExtensions::MeeteiMayekVowelSignAai),
            'ꫮ' => Ok(MeeteiMayekExtensions::MeeteiMayekVowelSignAu),
            'ꫯ' => Ok(MeeteiMayekExtensions::MeeteiMayekVowelSignAau),
            '꫰' => Ok(MeeteiMayekExtensions::MeeteiMayekCheikhan),
            '꫱' => Ok(MeeteiMayekExtensions::MeeteiMayekAhangKhudam),
            'ꫲ' => Ok(MeeteiMayekExtensions::MeeteiMayekAnji),
            'ꫳ' => Ok(MeeteiMayekExtensions::MeeteiMayekSyllableRepetitionMark),
            'ꫴ' => Ok(MeeteiMayekExtensions::MeeteiMayekWordRepetitionMark),
            'ꫵ' => Ok(MeeteiMayekExtensions::MeeteiMayekVowelSignVisarga),
            '꫶' => Ok(MeeteiMayekExtensions::MeeteiMayekVirama),
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
