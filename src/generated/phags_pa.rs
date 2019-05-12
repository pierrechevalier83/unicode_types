
/// An enum to represent all characters in the Phagspa block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Phagspa {
    /// \u{a840}: 'ꡀ'
    PhagsDashPaLetterKa,
    /// \u{a841}: 'ꡁ'
    PhagsDashPaLetterKha,
    /// \u{a842}: 'ꡂ'
    PhagsDashPaLetterGa,
    /// \u{a843}: 'ꡃ'
    PhagsDashPaLetterNga,
    /// \u{a844}: 'ꡄ'
    PhagsDashPaLetterCa,
    /// \u{a845}: 'ꡅ'
    PhagsDashPaLetterCha,
    /// \u{a846}: 'ꡆ'
    PhagsDashPaLetterJa,
    /// \u{a847}: 'ꡇ'
    PhagsDashPaLetterNya,
    /// \u{a848}: 'ꡈ'
    PhagsDashPaLetterTa,
    /// \u{a849}: 'ꡉ'
    PhagsDashPaLetterTha,
    /// \u{a84a}: 'ꡊ'
    PhagsDashPaLetterDa,
    /// \u{a84b}: 'ꡋ'
    PhagsDashPaLetterNa,
    /// \u{a84c}: 'ꡌ'
    PhagsDashPaLetterPa,
    /// \u{a84d}: 'ꡍ'
    PhagsDashPaLetterPha,
    /// \u{a84e}: 'ꡎ'
    PhagsDashPaLetterBa,
    /// \u{a84f}: 'ꡏ'
    PhagsDashPaLetterMa,
    /// \u{a850}: 'ꡐ'
    PhagsDashPaLetterTsa,
    /// \u{a851}: 'ꡑ'
    PhagsDashPaLetterTsha,
    /// \u{a852}: 'ꡒ'
    PhagsDashPaLetterDza,
    /// \u{a853}: 'ꡓ'
    PhagsDashPaLetterWa,
    /// \u{a854}: 'ꡔ'
    PhagsDashPaLetterZha,
    /// \u{a855}: 'ꡕ'
    PhagsDashPaLetterZa,
    /// \u{a856}: 'ꡖ'
    PhagsDashPaLetterSmallA,
    /// \u{a857}: 'ꡗ'
    PhagsDashPaLetterYa,
    /// \u{a858}: 'ꡘ'
    PhagsDashPaLetterRa,
    /// \u{a859}: 'ꡙ'
    PhagsDashPaLetterLa,
    /// \u{a85a}: 'ꡚ'
    PhagsDashPaLetterSha,
    /// \u{a85b}: 'ꡛ'
    PhagsDashPaLetterSa,
    /// \u{a85c}: 'ꡜ'
    PhagsDashPaLetterHa,
    /// \u{a85d}: 'ꡝ'
    PhagsDashPaLetterA,
    /// \u{a85e}: 'ꡞ'
    PhagsDashPaLetterI,
    /// \u{a85f}: 'ꡟ'
    PhagsDashPaLetterU,
    /// \u{a860}: 'ꡠ'
    PhagsDashPaLetterE,
    /// \u{a861}: 'ꡡ'
    PhagsDashPaLetterO,
    /// \u{a862}: 'ꡢ'
    PhagsDashPaLetterQa,
    /// \u{a863}: 'ꡣ'
    PhagsDashPaLetterXa,
    /// \u{a864}: 'ꡤ'
    PhagsDashPaLetterFa,
    /// \u{a865}: 'ꡥ'
    PhagsDashPaLetterGga,
    /// \u{a866}: 'ꡦ'
    PhagsDashPaLetterEe,
    /// \u{a867}: 'ꡧ'
    PhagsDashPaSubjoinedLetterWa,
    /// \u{a868}: 'ꡨ'
    PhagsDashPaSubjoinedLetterYa,
    /// \u{a869}: 'ꡩ'
    PhagsDashPaLetterTta,
    /// \u{a86a}: 'ꡪ'
    PhagsDashPaLetterTtha,
    /// \u{a86b}: 'ꡫ'
    PhagsDashPaLetterDda,
    /// \u{a86c}: 'ꡬ'
    PhagsDashPaLetterNna,
    /// \u{a86d}: 'ꡭ'
    PhagsDashPaLetterAlternateYa,
    /// \u{a86e}: 'ꡮ'
    PhagsDashPaLetterVoicelessSha,
    /// \u{a86f}: 'ꡯ'
    PhagsDashPaLetterVoicedHa,
    /// \u{a870}: 'ꡰ'
    PhagsDashPaLetterAspiratedFa,
    /// \u{a871}: 'ꡱ'
    PhagsDashPaSubjoinedLetterRa,
    /// \u{a872}: 'ꡲ'
    PhagsDashPaSuperfixedLetterRa,
    /// \u{a873}: 'ꡳ'
    PhagsDashPaLetterCandrabindu,
    /// \u{a874}: '꡴'
    PhagsDashPaSingleHeadMark,
    /// \u{a875}: '꡵'
    PhagsDashPaDoubleHeadMark,
    /// \u{a876}: '꡶'
    PhagsDashPaMarkShad,
    /// \u{a877}: '꡷'
    PhagsDashPaMarkDoubleShad,
}

impl Into<char> for Phagspa {
    fn into(self) -> char {
        match self {
            Phagspa::PhagsDashPaLetterKa => 'ꡀ',
            Phagspa::PhagsDashPaLetterKha => 'ꡁ',
            Phagspa::PhagsDashPaLetterGa => 'ꡂ',
            Phagspa::PhagsDashPaLetterNga => 'ꡃ',
            Phagspa::PhagsDashPaLetterCa => 'ꡄ',
            Phagspa::PhagsDashPaLetterCha => 'ꡅ',
            Phagspa::PhagsDashPaLetterJa => 'ꡆ',
            Phagspa::PhagsDashPaLetterNya => 'ꡇ',
            Phagspa::PhagsDashPaLetterTa => 'ꡈ',
            Phagspa::PhagsDashPaLetterTha => 'ꡉ',
            Phagspa::PhagsDashPaLetterDa => 'ꡊ',
            Phagspa::PhagsDashPaLetterNa => 'ꡋ',
            Phagspa::PhagsDashPaLetterPa => 'ꡌ',
            Phagspa::PhagsDashPaLetterPha => 'ꡍ',
            Phagspa::PhagsDashPaLetterBa => 'ꡎ',
            Phagspa::PhagsDashPaLetterMa => 'ꡏ',
            Phagspa::PhagsDashPaLetterTsa => 'ꡐ',
            Phagspa::PhagsDashPaLetterTsha => 'ꡑ',
            Phagspa::PhagsDashPaLetterDza => 'ꡒ',
            Phagspa::PhagsDashPaLetterWa => 'ꡓ',
            Phagspa::PhagsDashPaLetterZha => 'ꡔ',
            Phagspa::PhagsDashPaLetterZa => 'ꡕ',
            Phagspa::PhagsDashPaLetterSmallA => 'ꡖ',
            Phagspa::PhagsDashPaLetterYa => 'ꡗ',
            Phagspa::PhagsDashPaLetterRa => 'ꡘ',
            Phagspa::PhagsDashPaLetterLa => 'ꡙ',
            Phagspa::PhagsDashPaLetterSha => 'ꡚ',
            Phagspa::PhagsDashPaLetterSa => 'ꡛ',
            Phagspa::PhagsDashPaLetterHa => 'ꡜ',
            Phagspa::PhagsDashPaLetterA => 'ꡝ',
            Phagspa::PhagsDashPaLetterI => 'ꡞ',
            Phagspa::PhagsDashPaLetterU => 'ꡟ',
            Phagspa::PhagsDashPaLetterE => 'ꡠ',
            Phagspa::PhagsDashPaLetterO => 'ꡡ',
            Phagspa::PhagsDashPaLetterQa => 'ꡢ',
            Phagspa::PhagsDashPaLetterXa => 'ꡣ',
            Phagspa::PhagsDashPaLetterFa => 'ꡤ',
            Phagspa::PhagsDashPaLetterGga => 'ꡥ',
            Phagspa::PhagsDashPaLetterEe => 'ꡦ',
            Phagspa::PhagsDashPaSubjoinedLetterWa => 'ꡧ',
            Phagspa::PhagsDashPaSubjoinedLetterYa => 'ꡨ',
            Phagspa::PhagsDashPaLetterTta => 'ꡩ',
            Phagspa::PhagsDashPaLetterTtha => 'ꡪ',
            Phagspa::PhagsDashPaLetterDda => 'ꡫ',
            Phagspa::PhagsDashPaLetterNna => 'ꡬ',
            Phagspa::PhagsDashPaLetterAlternateYa => 'ꡭ',
            Phagspa::PhagsDashPaLetterVoicelessSha => 'ꡮ',
            Phagspa::PhagsDashPaLetterVoicedHa => 'ꡯ',
            Phagspa::PhagsDashPaLetterAspiratedFa => 'ꡰ',
            Phagspa::PhagsDashPaSubjoinedLetterRa => 'ꡱ',
            Phagspa::PhagsDashPaSuperfixedLetterRa => 'ꡲ',
            Phagspa::PhagsDashPaLetterCandrabindu => 'ꡳ',
            Phagspa::PhagsDashPaSingleHeadMark => '꡴',
            Phagspa::PhagsDashPaDoubleHeadMark => '꡵',
            Phagspa::PhagsDashPaMarkShad => '꡶',
            Phagspa::PhagsDashPaMarkDoubleShad => '꡷',
        }
    }
}

impl std::convert::TryFrom<char> for Phagspa {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ꡀ' => Ok(Phagspa::PhagsDashPaLetterKa),
            'ꡁ' => Ok(Phagspa::PhagsDashPaLetterKha),
            'ꡂ' => Ok(Phagspa::PhagsDashPaLetterGa),
            'ꡃ' => Ok(Phagspa::PhagsDashPaLetterNga),
            'ꡄ' => Ok(Phagspa::PhagsDashPaLetterCa),
            'ꡅ' => Ok(Phagspa::PhagsDashPaLetterCha),
            'ꡆ' => Ok(Phagspa::PhagsDashPaLetterJa),
            'ꡇ' => Ok(Phagspa::PhagsDashPaLetterNya),
            'ꡈ' => Ok(Phagspa::PhagsDashPaLetterTa),
            'ꡉ' => Ok(Phagspa::PhagsDashPaLetterTha),
            'ꡊ' => Ok(Phagspa::PhagsDashPaLetterDa),
            'ꡋ' => Ok(Phagspa::PhagsDashPaLetterNa),
            'ꡌ' => Ok(Phagspa::PhagsDashPaLetterPa),
            'ꡍ' => Ok(Phagspa::PhagsDashPaLetterPha),
            'ꡎ' => Ok(Phagspa::PhagsDashPaLetterBa),
            'ꡏ' => Ok(Phagspa::PhagsDashPaLetterMa),
            'ꡐ' => Ok(Phagspa::PhagsDashPaLetterTsa),
            'ꡑ' => Ok(Phagspa::PhagsDashPaLetterTsha),
            'ꡒ' => Ok(Phagspa::PhagsDashPaLetterDza),
            'ꡓ' => Ok(Phagspa::PhagsDashPaLetterWa),
            'ꡔ' => Ok(Phagspa::PhagsDashPaLetterZha),
            'ꡕ' => Ok(Phagspa::PhagsDashPaLetterZa),
            'ꡖ' => Ok(Phagspa::PhagsDashPaLetterSmallA),
            'ꡗ' => Ok(Phagspa::PhagsDashPaLetterYa),
            'ꡘ' => Ok(Phagspa::PhagsDashPaLetterRa),
            'ꡙ' => Ok(Phagspa::PhagsDashPaLetterLa),
            'ꡚ' => Ok(Phagspa::PhagsDashPaLetterSha),
            'ꡛ' => Ok(Phagspa::PhagsDashPaLetterSa),
            'ꡜ' => Ok(Phagspa::PhagsDashPaLetterHa),
            'ꡝ' => Ok(Phagspa::PhagsDashPaLetterA),
            'ꡞ' => Ok(Phagspa::PhagsDashPaLetterI),
            'ꡟ' => Ok(Phagspa::PhagsDashPaLetterU),
            'ꡠ' => Ok(Phagspa::PhagsDashPaLetterE),
            'ꡡ' => Ok(Phagspa::PhagsDashPaLetterO),
            'ꡢ' => Ok(Phagspa::PhagsDashPaLetterQa),
            'ꡣ' => Ok(Phagspa::PhagsDashPaLetterXa),
            'ꡤ' => Ok(Phagspa::PhagsDashPaLetterFa),
            'ꡥ' => Ok(Phagspa::PhagsDashPaLetterGga),
            'ꡦ' => Ok(Phagspa::PhagsDashPaLetterEe),
            'ꡧ' => Ok(Phagspa::PhagsDashPaSubjoinedLetterWa),
            'ꡨ' => Ok(Phagspa::PhagsDashPaSubjoinedLetterYa),
            'ꡩ' => Ok(Phagspa::PhagsDashPaLetterTta),
            'ꡪ' => Ok(Phagspa::PhagsDashPaLetterTtha),
            'ꡫ' => Ok(Phagspa::PhagsDashPaLetterDda),
            'ꡬ' => Ok(Phagspa::PhagsDashPaLetterNna),
            'ꡭ' => Ok(Phagspa::PhagsDashPaLetterAlternateYa),
            'ꡮ' => Ok(Phagspa::PhagsDashPaLetterVoicelessSha),
            'ꡯ' => Ok(Phagspa::PhagsDashPaLetterVoicedHa),
            'ꡰ' => Ok(Phagspa::PhagsDashPaLetterAspiratedFa),
            'ꡱ' => Ok(Phagspa::PhagsDashPaSubjoinedLetterRa),
            'ꡲ' => Ok(Phagspa::PhagsDashPaSuperfixedLetterRa),
            'ꡳ' => Ok(Phagspa::PhagsDashPaLetterCandrabindu),
            '꡴' => Ok(Phagspa::PhagsDashPaSingleHeadMark),
            '꡵' => Ok(Phagspa::PhagsDashPaDoubleHeadMark),
            '꡶' => Ok(Phagspa::PhagsDashPaMarkShad),
            '꡷' => Ok(Phagspa::PhagsDashPaMarkDoubleShad),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Phagspa {
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

impl std::convert::TryFrom<u32> for Phagspa {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Phagspa {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Phagspa {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Phagspa::PhagsDashPaLetterKa
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Phagspa::PhagsDashPaLetterKa => "phags-pa letter ka",
            Phagspa::PhagsDashPaLetterKha => "phags-pa letter kha",
            Phagspa::PhagsDashPaLetterGa => "phags-pa letter ga",
            Phagspa::PhagsDashPaLetterNga => "phags-pa letter nga",
            Phagspa::PhagsDashPaLetterCa => "phags-pa letter ca",
            Phagspa::PhagsDashPaLetterCha => "phags-pa letter cha",
            Phagspa::PhagsDashPaLetterJa => "phags-pa letter ja",
            Phagspa::PhagsDashPaLetterNya => "phags-pa letter nya",
            Phagspa::PhagsDashPaLetterTa => "phags-pa letter ta",
            Phagspa::PhagsDashPaLetterTha => "phags-pa letter tha",
            Phagspa::PhagsDashPaLetterDa => "phags-pa letter da",
            Phagspa::PhagsDashPaLetterNa => "phags-pa letter na",
            Phagspa::PhagsDashPaLetterPa => "phags-pa letter pa",
            Phagspa::PhagsDashPaLetterPha => "phags-pa letter pha",
            Phagspa::PhagsDashPaLetterBa => "phags-pa letter ba",
            Phagspa::PhagsDashPaLetterMa => "phags-pa letter ma",
            Phagspa::PhagsDashPaLetterTsa => "phags-pa letter tsa",
            Phagspa::PhagsDashPaLetterTsha => "phags-pa letter tsha",
            Phagspa::PhagsDashPaLetterDza => "phags-pa letter dza",
            Phagspa::PhagsDashPaLetterWa => "phags-pa letter wa",
            Phagspa::PhagsDashPaLetterZha => "phags-pa letter zha",
            Phagspa::PhagsDashPaLetterZa => "phags-pa letter za",
            Phagspa::PhagsDashPaLetterSmallA => "phags-pa letter small a",
            Phagspa::PhagsDashPaLetterYa => "phags-pa letter ya",
            Phagspa::PhagsDashPaLetterRa => "phags-pa letter ra",
            Phagspa::PhagsDashPaLetterLa => "phags-pa letter la",
            Phagspa::PhagsDashPaLetterSha => "phags-pa letter sha",
            Phagspa::PhagsDashPaLetterSa => "phags-pa letter sa",
            Phagspa::PhagsDashPaLetterHa => "phags-pa letter ha",
            Phagspa::PhagsDashPaLetterA => "phags-pa letter a",
            Phagspa::PhagsDashPaLetterI => "phags-pa letter i",
            Phagspa::PhagsDashPaLetterU => "phags-pa letter u",
            Phagspa::PhagsDashPaLetterE => "phags-pa letter e",
            Phagspa::PhagsDashPaLetterO => "phags-pa letter o",
            Phagspa::PhagsDashPaLetterQa => "phags-pa letter qa",
            Phagspa::PhagsDashPaLetterXa => "phags-pa letter xa",
            Phagspa::PhagsDashPaLetterFa => "phags-pa letter fa",
            Phagspa::PhagsDashPaLetterGga => "phags-pa letter gga",
            Phagspa::PhagsDashPaLetterEe => "phags-pa letter ee",
            Phagspa::PhagsDashPaSubjoinedLetterWa => "phags-pa subjoined letter wa",
            Phagspa::PhagsDashPaSubjoinedLetterYa => "phags-pa subjoined letter ya",
            Phagspa::PhagsDashPaLetterTta => "phags-pa letter tta",
            Phagspa::PhagsDashPaLetterTtha => "phags-pa letter ttha",
            Phagspa::PhagsDashPaLetterDda => "phags-pa letter dda",
            Phagspa::PhagsDashPaLetterNna => "phags-pa letter nna",
            Phagspa::PhagsDashPaLetterAlternateYa => "phags-pa letter alternate ya",
            Phagspa::PhagsDashPaLetterVoicelessSha => "phags-pa letter voiceless sha",
            Phagspa::PhagsDashPaLetterVoicedHa => "phags-pa letter voiced ha",
            Phagspa::PhagsDashPaLetterAspiratedFa => "phags-pa letter aspirated fa",
            Phagspa::PhagsDashPaSubjoinedLetterRa => "phags-pa subjoined letter ra",
            Phagspa::PhagsDashPaSuperfixedLetterRa => "phags-pa superfixed letter ra",
            Phagspa::PhagsDashPaLetterCandrabindu => "phags-pa letter candrabindu",
            Phagspa::PhagsDashPaSingleHeadMark => "phags-pa single head mark",
            Phagspa::PhagsDashPaDoubleHeadMark => "phags-pa double head mark",
            Phagspa::PhagsDashPaMarkShad => "phags-pa mark shad",
            Phagspa::PhagsDashPaMarkDoubleShad => "phags-pa mark double shad",
        }
    }
}
