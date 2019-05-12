
/// An enum to represent all characters in the Lisu block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Lisu {
    /// \u{a4d0}: 'ꓐ'
    LetterBa,
    /// \u{a4d1}: 'ꓑ'
    LetterPa,
    /// \u{a4d2}: 'ꓒ'
    LetterPha,
    /// \u{a4d3}: 'ꓓ'
    LetterDa,
    /// \u{a4d4}: 'ꓔ'
    LetterTa,
    /// \u{a4d5}: 'ꓕ'
    LetterTha,
    /// \u{a4d6}: 'ꓖ'
    LetterGa,
    /// \u{a4d7}: 'ꓗ'
    LetterKa,
    /// \u{a4d8}: 'ꓘ'
    LetterKha,
    /// \u{a4d9}: 'ꓙ'
    LetterJa,
    /// \u{a4da}: 'ꓚ'
    LetterCa,
    /// \u{a4db}: 'ꓛ'
    LetterCha,
    /// \u{a4dc}: 'ꓜ'
    LetterDza,
    /// \u{a4dd}: 'ꓝ'
    LetterTsa,
    /// \u{a4de}: 'ꓞ'
    LetterTsha,
    /// \u{a4df}: 'ꓟ'
    LetterMa,
    /// \u{a4e0}: 'ꓠ'
    LetterNa,
    /// \u{a4e1}: 'ꓡ'
    LetterLa,
    /// \u{a4e2}: 'ꓢ'
    LetterSa,
    /// \u{a4e3}: 'ꓣ'
    LetterZha,
    /// \u{a4e4}: 'ꓤ'
    LetterZa,
    /// \u{a4e5}: 'ꓥ'
    LetterNga,
    /// \u{a4e6}: 'ꓦ'
    LetterHa,
    /// \u{a4e7}: 'ꓧ'
    LetterXa,
    /// \u{a4e8}: 'ꓨ'
    LetterHha,
    /// \u{a4e9}: 'ꓩ'
    LetterFa,
    /// \u{a4ea}: 'ꓪ'
    LetterWa,
    /// \u{a4eb}: 'ꓫ'
    LetterSha,
    /// \u{a4ec}: 'ꓬ'
    LetterYa,
    /// \u{a4ed}: 'ꓭ'
    LetterGha,
    /// \u{a4ee}: 'ꓮ'
    LetterA,
    /// \u{a4ef}: 'ꓯ'
    LetterAe,
    /// \u{a4f0}: 'ꓰ'
    LetterE,
    /// \u{a4f1}: 'ꓱ'
    LetterEu,
    /// \u{a4f2}: 'ꓲ'
    LetterI,
    /// \u{a4f3}: 'ꓳ'
    LetterO,
    /// \u{a4f4}: 'ꓴ'
    LetterU,
    /// \u{a4f5}: 'ꓵ'
    LetterUe,
    /// \u{a4f6}: 'ꓶ'
    LetterUh,
    /// \u{a4f7}: 'ꓷ'
    LetterOe,
    /// \u{a4f8}: 'ꓸ'
    LetterToneMyaTi,
    /// \u{a4f9}: 'ꓹ'
    LetterToneNaPo,
    /// \u{a4fa}: 'ꓺ'
    LetterToneMyaCya,
    /// \u{a4fb}: 'ꓻ'
    LetterToneMyaBo,
    /// \u{a4fc}: 'ꓼ'
    LetterToneMyaNa,
    /// \u{a4fd}: 'ꓽ'
    LetterToneMyaJeu,
    /// \u{a4fe}: '꓾'
    PunctuationComma,
}

impl Into<char> for Lisu {
    fn into(self) -> char {
        match self {
            Lisu::LetterBa => 'ꓐ',
            Lisu::LetterPa => 'ꓑ',
            Lisu::LetterPha => 'ꓒ',
            Lisu::LetterDa => 'ꓓ',
            Lisu::LetterTa => 'ꓔ',
            Lisu::LetterTha => 'ꓕ',
            Lisu::LetterGa => 'ꓖ',
            Lisu::LetterKa => 'ꓗ',
            Lisu::LetterKha => 'ꓘ',
            Lisu::LetterJa => 'ꓙ',
            Lisu::LetterCa => 'ꓚ',
            Lisu::LetterCha => 'ꓛ',
            Lisu::LetterDza => 'ꓜ',
            Lisu::LetterTsa => 'ꓝ',
            Lisu::LetterTsha => 'ꓞ',
            Lisu::LetterMa => 'ꓟ',
            Lisu::LetterNa => 'ꓠ',
            Lisu::LetterLa => 'ꓡ',
            Lisu::LetterSa => 'ꓢ',
            Lisu::LetterZha => 'ꓣ',
            Lisu::LetterZa => 'ꓤ',
            Lisu::LetterNga => 'ꓥ',
            Lisu::LetterHa => 'ꓦ',
            Lisu::LetterXa => 'ꓧ',
            Lisu::LetterHha => 'ꓨ',
            Lisu::LetterFa => 'ꓩ',
            Lisu::LetterWa => 'ꓪ',
            Lisu::LetterSha => 'ꓫ',
            Lisu::LetterYa => 'ꓬ',
            Lisu::LetterGha => 'ꓭ',
            Lisu::LetterA => 'ꓮ',
            Lisu::LetterAe => 'ꓯ',
            Lisu::LetterE => 'ꓰ',
            Lisu::LetterEu => 'ꓱ',
            Lisu::LetterI => 'ꓲ',
            Lisu::LetterO => 'ꓳ',
            Lisu::LetterU => 'ꓴ',
            Lisu::LetterUe => 'ꓵ',
            Lisu::LetterUh => 'ꓶ',
            Lisu::LetterOe => 'ꓷ',
            Lisu::LetterToneMyaTi => 'ꓸ',
            Lisu::LetterToneNaPo => 'ꓹ',
            Lisu::LetterToneMyaCya => 'ꓺ',
            Lisu::LetterToneMyaBo => 'ꓻ',
            Lisu::LetterToneMyaNa => 'ꓼ',
            Lisu::LetterToneMyaJeu => 'ꓽ',
            Lisu::PunctuationComma => '꓾',
        }
    }
}

impl std::convert::TryFrom<char> for Lisu {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ꓐ' => Ok(Lisu::LetterBa),
            'ꓑ' => Ok(Lisu::LetterPa),
            'ꓒ' => Ok(Lisu::LetterPha),
            'ꓓ' => Ok(Lisu::LetterDa),
            'ꓔ' => Ok(Lisu::LetterTa),
            'ꓕ' => Ok(Lisu::LetterTha),
            'ꓖ' => Ok(Lisu::LetterGa),
            'ꓗ' => Ok(Lisu::LetterKa),
            'ꓘ' => Ok(Lisu::LetterKha),
            'ꓙ' => Ok(Lisu::LetterJa),
            'ꓚ' => Ok(Lisu::LetterCa),
            'ꓛ' => Ok(Lisu::LetterCha),
            'ꓜ' => Ok(Lisu::LetterDza),
            'ꓝ' => Ok(Lisu::LetterTsa),
            'ꓞ' => Ok(Lisu::LetterTsha),
            'ꓟ' => Ok(Lisu::LetterMa),
            'ꓠ' => Ok(Lisu::LetterNa),
            'ꓡ' => Ok(Lisu::LetterLa),
            'ꓢ' => Ok(Lisu::LetterSa),
            'ꓣ' => Ok(Lisu::LetterZha),
            'ꓤ' => Ok(Lisu::LetterZa),
            'ꓥ' => Ok(Lisu::LetterNga),
            'ꓦ' => Ok(Lisu::LetterHa),
            'ꓧ' => Ok(Lisu::LetterXa),
            'ꓨ' => Ok(Lisu::LetterHha),
            'ꓩ' => Ok(Lisu::LetterFa),
            'ꓪ' => Ok(Lisu::LetterWa),
            'ꓫ' => Ok(Lisu::LetterSha),
            'ꓬ' => Ok(Lisu::LetterYa),
            'ꓭ' => Ok(Lisu::LetterGha),
            'ꓮ' => Ok(Lisu::LetterA),
            'ꓯ' => Ok(Lisu::LetterAe),
            'ꓰ' => Ok(Lisu::LetterE),
            'ꓱ' => Ok(Lisu::LetterEu),
            'ꓲ' => Ok(Lisu::LetterI),
            'ꓳ' => Ok(Lisu::LetterO),
            'ꓴ' => Ok(Lisu::LetterU),
            'ꓵ' => Ok(Lisu::LetterUe),
            'ꓶ' => Ok(Lisu::LetterUh),
            'ꓷ' => Ok(Lisu::LetterOe),
            'ꓸ' => Ok(Lisu::LetterToneMyaTi),
            'ꓹ' => Ok(Lisu::LetterToneNaPo),
            'ꓺ' => Ok(Lisu::LetterToneMyaCya),
            'ꓻ' => Ok(Lisu::LetterToneMyaBo),
            'ꓼ' => Ok(Lisu::LetterToneMyaNa),
            'ꓽ' => Ok(Lisu::LetterToneMyaJeu),
            '꓾' => Ok(Lisu::PunctuationComma),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Lisu {
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

impl std::convert::TryFrom<u32> for Lisu {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Lisu {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Lisu {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Lisu::LetterBa
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Lisu::LetterBa => "lisu letter ba",
            Lisu::LetterPa => "lisu letter pa",
            Lisu::LetterPha => "lisu letter pha",
            Lisu::LetterDa => "lisu letter da",
            Lisu::LetterTa => "lisu letter ta",
            Lisu::LetterTha => "lisu letter tha",
            Lisu::LetterGa => "lisu letter ga",
            Lisu::LetterKa => "lisu letter ka",
            Lisu::LetterKha => "lisu letter kha",
            Lisu::LetterJa => "lisu letter ja",
            Lisu::LetterCa => "lisu letter ca",
            Lisu::LetterCha => "lisu letter cha",
            Lisu::LetterDza => "lisu letter dza",
            Lisu::LetterTsa => "lisu letter tsa",
            Lisu::LetterTsha => "lisu letter tsha",
            Lisu::LetterMa => "lisu letter ma",
            Lisu::LetterNa => "lisu letter na",
            Lisu::LetterLa => "lisu letter la",
            Lisu::LetterSa => "lisu letter sa",
            Lisu::LetterZha => "lisu letter zha",
            Lisu::LetterZa => "lisu letter za",
            Lisu::LetterNga => "lisu letter nga",
            Lisu::LetterHa => "lisu letter ha",
            Lisu::LetterXa => "lisu letter xa",
            Lisu::LetterHha => "lisu letter hha",
            Lisu::LetterFa => "lisu letter fa",
            Lisu::LetterWa => "lisu letter wa",
            Lisu::LetterSha => "lisu letter sha",
            Lisu::LetterYa => "lisu letter ya",
            Lisu::LetterGha => "lisu letter gha",
            Lisu::LetterA => "lisu letter a",
            Lisu::LetterAe => "lisu letter ae",
            Lisu::LetterE => "lisu letter e",
            Lisu::LetterEu => "lisu letter eu",
            Lisu::LetterI => "lisu letter i",
            Lisu::LetterO => "lisu letter o",
            Lisu::LetterU => "lisu letter u",
            Lisu::LetterUe => "lisu letter ue",
            Lisu::LetterUh => "lisu letter uh",
            Lisu::LetterOe => "lisu letter oe",
            Lisu::LetterToneMyaTi => "lisu letter tone mya ti",
            Lisu::LetterToneNaPo => "lisu letter tone na po",
            Lisu::LetterToneMyaCya => "lisu letter tone mya cya",
            Lisu::LetterToneMyaBo => "lisu letter tone mya bo",
            Lisu::LetterToneMyaNa => "lisu letter tone mya na",
            Lisu::LetterToneMyaJeu => "lisu letter tone mya jeu",
            Lisu::PunctuationComma => "lisu punctuation comma",
        }
    }
}
