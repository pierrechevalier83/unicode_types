
/// An enum to represent all characters in the Glagolitic block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Glagolitic {
    /// \u{2c00}: 'Ⰰ'
    CapitalLetterAzu,
    /// \u{2c01}: 'Ⰱ'
    CapitalLetterBuky,
    /// \u{2c02}: 'Ⰲ'
    CapitalLetterVede,
    /// \u{2c03}: 'Ⰳ'
    CapitalLetterGlagoli,
    /// \u{2c04}: 'Ⰴ'
    CapitalLetterDobro,
    /// \u{2c05}: 'Ⰵ'
    CapitalLetterYestu,
    /// \u{2c06}: 'Ⰶ'
    CapitalLetterZhivete,
    /// \u{2c07}: 'Ⰷ'
    CapitalLetterDzelo,
    /// \u{2c08}: 'Ⰸ'
    CapitalLetterZemlja,
    /// \u{2c09}: 'Ⰹ'
    CapitalLetterIzhe,
    /// \u{2c0a}: 'Ⰺ'
    CapitalLetterInitialIzhe,
    /// \u{2c0b}: 'Ⰻ'
    CapitalLetterI,
    /// \u{2c0c}: 'Ⰼ'
    CapitalLetterDjervi,
    /// \u{2c0d}: 'Ⰽ'
    CapitalLetterKako,
    /// \u{2c0e}: 'Ⰾ'
    CapitalLetterLjudije,
    /// \u{2c0f}: 'Ⰿ'
    CapitalLetterMyslite,
    /// \u{2c10}: 'Ⱀ'
    CapitalLetterNashi,
    /// \u{2c11}: 'Ⱁ'
    CapitalLetterOnu,
    /// \u{2c12}: 'Ⱂ'
    CapitalLetterPokoji,
    /// \u{2c13}: 'Ⱃ'
    CapitalLetterRitsi,
    /// \u{2c14}: 'Ⱄ'
    CapitalLetterSlovo,
    /// \u{2c15}: 'Ⱅ'
    CapitalLetterTvrido,
    /// \u{2c16}: 'Ⱆ'
    CapitalLetterUku,
    /// \u{2c17}: 'Ⱇ'
    CapitalLetterFritu,
    /// \u{2c18}: 'Ⱈ'
    CapitalLetterHeru,
    /// \u{2c19}: 'Ⱉ'
    CapitalLetterOtu,
    /// \u{2c1a}: 'Ⱊ'
    CapitalLetterPe,
    /// \u{2c1b}: 'Ⱋ'
    CapitalLetterShta,
    /// \u{2c1c}: 'Ⱌ'
    CapitalLetterTsi,
    /// \u{2c1d}: 'Ⱍ'
    CapitalLetterChrivi,
    /// \u{2c1e}: 'Ⱎ'
    CapitalLetterSha,
    /// \u{2c1f}: 'Ⱏ'
    CapitalLetterYeru,
    /// \u{2c20}: 'Ⱐ'
    CapitalLetterYeri,
    /// \u{2c21}: 'Ⱑ'
    CapitalLetterYati,
    /// \u{2c22}: 'Ⱒ'
    CapitalLetterSpideryHa,
    /// \u{2c23}: 'Ⱓ'
    CapitalLetterYu,
    /// \u{2c24}: 'Ⱔ'
    CapitalLetterSmallYus,
    /// \u{2c25}: 'Ⱕ'
    CapitalLetterSmallYusWithTail,
    /// \u{2c26}: 'Ⱖ'
    CapitalLetterYo,
    /// \u{2c27}: 'Ⱗ'
    CapitalLetterIotatedSmallYus,
    /// \u{2c28}: 'Ⱘ'
    CapitalLetterBigYus,
    /// \u{2c29}: 'Ⱙ'
    CapitalLetterIotatedBigYus,
    /// \u{2c2a}: 'Ⱚ'
    CapitalLetterFita,
    /// \u{2c2b}: 'Ⱛ'
    CapitalLetterIzhitsa,
    /// \u{2c2c}: 'Ⱜ'
    CapitalLetterShtapic,
    /// \u{2c2d}: 'Ⱝ'
    CapitalLetterTrokutastiA,
    /// \u{2c2e}: 'Ⱞ'
    CapitalLetterLatinateMyslite,
    /// \u{2c30}: 'ⰰ'
    SmallLetterAzu,
    /// \u{2c31}: 'ⰱ'
    SmallLetterBuky,
    /// \u{2c32}: 'ⰲ'
    SmallLetterVede,
    /// \u{2c33}: 'ⰳ'
    SmallLetterGlagoli,
    /// \u{2c34}: 'ⰴ'
    SmallLetterDobro,
    /// \u{2c35}: 'ⰵ'
    SmallLetterYestu,
    /// \u{2c36}: 'ⰶ'
    SmallLetterZhivete,
    /// \u{2c37}: 'ⰷ'
    SmallLetterDzelo,
    /// \u{2c38}: 'ⰸ'
    SmallLetterZemlja,
    /// \u{2c39}: 'ⰹ'
    SmallLetterIzhe,
    /// \u{2c3a}: 'ⰺ'
    SmallLetterInitialIzhe,
    /// \u{2c3b}: 'ⰻ'
    SmallLetterI,
    /// \u{2c3c}: 'ⰼ'
    SmallLetterDjervi,
    /// \u{2c3d}: 'ⰽ'
    SmallLetterKako,
    /// \u{2c3e}: 'ⰾ'
    SmallLetterLjudije,
    /// \u{2c3f}: 'ⰿ'
    SmallLetterMyslite,
    /// \u{2c40}: 'ⱀ'
    SmallLetterNashi,
    /// \u{2c41}: 'ⱁ'
    SmallLetterOnu,
    /// \u{2c42}: 'ⱂ'
    SmallLetterPokoji,
    /// \u{2c43}: 'ⱃ'
    SmallLetterRitsi,
    /// \u{2c44}: 'ⱄ'
    SmallLetterSlovo,
    /// \u{2c45}: 'ⱅ'
    SmallLetterTvrido,
    /// \u{2c46}: 'ⱆ'
    SmallLetterUku,
    /// \u{2c47}: 'ⱇ'
    SmallLetterFritu,
    /// \u{2c48}: 'ⱈ'
    SmallLetterHeru,
    /// \u{2c49}: 'ⱉ'
    SmallLetterOtu,
    /// \u{2c4a}: 'ⱊ'
    SmallLetterPe,
    /// \u{2c4b}: 'ⱋ'
    SmallLetterShta,
    /// \u{2c4c}: 'ⱌ'
    SmallLetterTsi,
    /// \u{2c4d}: 'ⱍ'
    SmallLetterChrivi,
    /// \u{2c4e}: 'ⱎ'
    SmallLetterSha,
    /// \u{2c4f}: 'ⱏ'
    SmallLetterYeru,
    /// \u{2c50}: 'ⱐ'
    SmallLetterYeri,
    /// \u{2c51}: 'ⱑ'
    SmallLetterYati,
    /// \u{2c52}: 'ⱒ'
    SmallLetterSpideryHa,
    /// \u{2c53}: 'ⱓ'
    SmallLetterYu,
    /// \u{2c54}: 'ⱔ'
    SmallLetterSmallYus,
    /// \u{2c55}: 'ⱕ'
    SmallLetterSmallYusWithTail,
    /// \u{2c56}: 'ⱖ'
    SmallLetterYo,
    /// \u{2c57}: 'ⱗ'
    SmallLetterIotatedSmallYus,
    /// \u{2c58}: 'ⱘ'
    SmallLetterBigYus,
    /// \u{2c59}: 'ⱙ'
    SmallLetterIotatedBigYus,
    /// \u{2c5a}: 'ⱚ'
    SmallLetterFita,
    /// \u{2c5b}: 'ⱛ'
    SmallLetterIzhitsa,
    /// \u{2c5c}: 'ⱜ'
    SmallLetterShtapic,
    /// \u{2c5d}: 'ⱝ'
    SmallLetterTrokutastiA,
    /// \u{2c5e}: 'ⱞ'
    SmallLetterLatinateMyslite,
}

impl Into<char> for Glagolitic {
    fn into(self) -> char {
        match self {
            Glagolitic::CapitalLetterAzu => 'Ⰰ',
            Glagolitic::CapitalLetterBuky => 'Ⰱ',
            Glagolitic::CapitalLetterVede => 'Ⰲ',
            Glagolitic::CapitalLetterGlagoli => 'Ⰳ',
            Glagolitic::CapitalLetterDobro => 'Ⰴ',
            Glagolitic::CapitalLetterYestu => 'Ⰵ',
            Glagolitic::CapitalLetterZhivete => 'Ⰶ',
            Glagolitic::CapitalLetterDzelo => 'Ⰷ',
            Glagolitic::CapitalLetterZemlja => 'Ⰸ',
            Glagolitic::CapitalLetterIzhe => 'Ⰹ',
            Glagolitic::CapitalLetterInitialIzhe => 'Ⰺ',
            Glagolitic::CapitalLetterI => 'Ⰻ',
            Glagolitic::CapitalLetterDjervi => 'Ⰼ',
            Glagolitic::CapitalLetterKako => 'Ⰽ',
            Glagolitic::CapitalLetterLjudije => 'Ⰾ',
            Glagolitic::CapitalLetterMyslite => 'Ⰿ',
            Glagolitic::CapitalLetterNashi => 'Ⱀ',
            Glagolitic::CapitalLetterOnu => 'Ⱁ',
            Glagolitic::CapitalLetterPokoji => 'Ⱂ',
            Glagolitic::CapitalLetterRitsi => 'Ⱃ',
            Glagolitic::CapitalLetterSlovo => 'Ⱄ',
            Glagolitic::CapitalLetterTvrido => 'Ⱅ',
            Glagolitic::CapitalLetterUku => 'Ⱆ',
            Glagolitic::CapitalLetterFritu => 'Ⱇ',
            Glagolitic::CapitalLetterHeru => 'Ⱈ',
            Glagolitic::CapitalLetterOtu => 'Ⱉ',
            Glagolitic::CapitalLetterPe => 'Ⱊ',
            Glagolitic::CapitalLetterShta => 'Ⱋ',
            Glagolitic::CapitalLetterTsi => 'Ⱌ',
            Glagolitic::CapitalLetterChrivi => 'Ⱍ',
            Glagolitic::CapitalLetterSha => 'Ⱎ',
            Glagolitic::CapitalLetterYeru => 'Ⱏ',
            Glagolitic::CapitalLetterYeri => 'Ⱐ',
            Glagolitic::CapitalLetterYati => 'Ⱑ',
            Glagolitic::CapitalLetterSpideryHa => 'Ⱒ',
            Glagolitic::CapitalLetterYu => 'Ⱓ',
            Glagolitic::CapitalLetterSmallYus => 'Ⱔ',
            Glagolitic::CapitalLetterSmallYusWithTail => 'Ⱕ',
            Glagolitic::CapitalLetterYo => 'Ⱖ',
            Glagolitic::CapitalLetterIotatedSmallYus => 'Ⱗ',
            Glagolitic::CapitalLetterBigYus => 'Ⱘ',
            Glagolitic::CapitalLetterIotatedBigYus => 'Ⱙ',
            Glagolitic::CapitalLetterFita => 'Ⱚ',
            Glagolitic::CapitalLetterIzhitsa => 'Ⱛ',
            Glagolitic::CapitalLetterShtapic => 'Ⱜ',
            Glagolitic::CapitalLetterTrokutastiA => 'Ⱝ',
            Glagolitic::CapitalLetterLatinateMyslite => 'Ⱞ',
            Glagolitic::SmallLetterAzu => 'ⰰ',
            Glagolitic::SmallLetterBuky => 'ⰱ',
            Glagolitic::SmallLetterVede => 'ⰲ',
            Glagolitic::SmallLetterGlagoli => 'ⰳ',
            Glagolitic::SmallLetterDobro => 'ⰴ',
            Glagolitic::SmallLetterYestu => 'ⰵ',
            Glagolitic::SmallLetterZhivete => 'ⰶ',
            Glagolitic::SmallLetterDzelo => 'ⰷ',
            Glagolitic::SmallLetterZemlja => 'ⰸ',
            Glagolitic::SmallLetterIzhe => 'ⰹ',
            Glagolitic::SmallLetterInitialIzhe => 'ⰺ',
            Glagolitic::SmallLetterI => 'ⰻ',
            Glagolitic::SmallLetterDjervi => 'ⰼ',
            Glagolitic::SmallLetterKako => 'ⰽ',
            Glagolitic::SmallLetterLjudije => 'ⰾ',
            Glagolitic::SmallLetterMyslite => 'ⰿ',
            Glagolitic::SmallLetterNashi => 'ⱀ',
            Glagolitic::SmallLetterOnu => 'ⱁ',
            Glagolitic::SmallLetterPokoji => 'ⱂ',
            Glagolitic::SmallLetterRitsi => 'ⱃ',
            Glagolitic::SmallLetterSlovo => 'ⱄ',
            Glagolitic::SmallLetterTvrido => 'ⱅ',
            Glagolitic::SmallLetterUku => 'ⱆ',
            Glagolitic::SmallLetterFritu => 'ⱇ',
            Glagolitic::SmallLetterHeru => 'ⱈ',
            Glagolitic::SmallLetterOtu => 'ⱉ',
            Glagolitic::SmallLetterPe => 'ⱊ',
            Glagolitic::SmallLetterShta => 'ⱋ',
            Glagolitic::SmallLetterTsi => 'ⱌ',
            Glagolitic::SmallLetterChrivi => 'ⱍ',
            Glagolitic::SmallLetterSha => 'ⱎ',
            Glagolitic::SmallLetterYeru => 'ⱏ',
            Glagolitic::SmallLetterYeri => 'ⱐ',
            Glagolitic::SmallLetterYati => 'ⱑ',
            Glagolitic::SmallLetterSpideryHa => 'ⱒ',
            Glagolitic::SmallLetterYu => 'ⱓ',
            Glagolitic::SmallLetterSmallYus => 'ⱔ',
            Glagolitic::SmallLetterSmallYusWithTail => 'ⱕ',
            Glagolitic::SmallLetterYo => 'ⱖ',
            Glagolitic::SmallLetterIotatedSmallYus => 'ⱗ',
            Glagolitic::SmallLetterBigYus => 'ⱘ',
            Glagolitic::SmallLetterIotatedBigYus => 'ⱙ',
            Glagolitic::SmallLetterFita => 'ⱚ',
            Glagolitic::SmallLetterIzhitsa => 'ⱛ',
            Glagolitic::SmallLetterShtapic => 'ⱜ',
            Glagolitic::SmallLetterTrokutastiA => 'ⱝ',
            Glagolitic::SmallLetterLatinateMyslite => 'ⱞ',
        }
    }
}

impl std::convert::TryFrom<char> for Glagolitic {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'Ⰰ' => Ok(Glagolitic::CapitalLetterAzu),
            'Ⰱ' => Ok(Glagolitic::CapitalLetterBuky),
            'Ⰲ' => Ok(Glagolitic::CapitalLetterVede),
            'Ⰳ' => Ok(Glagolitic::CapitalLetterGlagoli),
            'Ⰴ' => Ok(Glagolitic::CapitalLetterDobro),
            'Ⰵ' => Ok(Glagolitic::CapitalLetterYestu),
            'Ⰶ' => Ok(Glagolitic::CapitalLetterZhivete),
            'Ⰷ' => Ok(Glagolitic::CapitalLetterDzelo),
            'Ⰸ' => Ok(Glagolitic::CapitalLetterZemlja),
            'Ⰹ' => Ok(Glagolitic::CapitalLetterIzhe),
            'Ⰺ' => Ok(Glagolitic::CapitalLetterInitialIzhe),
            'Ⰻ' => Ok(Glagolitic::CapitalLetterI),
            'Ⰼ' => Ok(Glagolitic::CapitalLetterDjervi),
            'Ⰽ' => Ok(Glagolitic::CapitalLetterKako),
            'Ⰾ' => Ok(Glagolitic::CapitalLetterLjudije),
            'Ⰿ' => Ok(Glagolitic::CapitalLetterMyslite),
            'Ⱀ' => Ok(Glagolitic::CapitalLetterNashi),
            'Ⱁ' => Ok(Glagolitic::CapitalLetterOnu),
            'Ⱂ' => Ok(Glagolitic::CapitalLetterPokoji),
            'Ⱃ' => Ok(Glagolitic::CapitalLetterRitsi),
            'Ⱄ' => Ok(Glagolitic::CapitalLetterSlovo),
            'Ⱅ' => Ok(Glagolitic::CapitalLetterTvrido),
            'Ⱆ' => Ok(Glagolitic::CapitalLetterUku),
            'Ⱇ' => Ok(Glagolitic::CapitalLetterFritu),
            'Ⱈ' => Ok(Glagolitic::CapitalLetterHeru),
            'Ⱉ' => Ok(Glagolitic::CapitalLetterOtu),
            'Ⱊ' => Ok(Glagolitic::CapitalLetterPe),
            'Ⱋ' => Ok(Glagolitic::CapitalLetterShta),
            'Ⱌ' => Ok(Glagolitic::CapitalLetterTsi),
            'Ⱍ' => Ok(Glagolitic::CapitalLetterChrivi),
            'Ⱎ' => Ok(Glagolitic::CapitalLetterSha),
            'Ⱏ' => Ok(Glagolitic::CapitalLetterYeru),
            'Ⱐ' => Ok(Glagolitic::CapitalLetterYeri),
            'Ⱑ' => Ok(Glagolitic::CapitalLetterYati),
            'Ⱒ' => Ok(Glagolitic::CapitalLetterSpideryHa),
            'Ⱓ' => Ok(Glagolitic::CapitalLetterYu),
            'Ⱔ' => Ok(Glagolitic::CapitalLetterSmallYus),
            'Ⱕ' => Ok(Glagolitic::CapitalLetterSmallYusWithTail),
            'Ⱖ' => Ok(Glagolitic::CapitalLetterYo),
            'Ⱗ' => Ok(Glagolitic::CapitalLetterIotatedSmallYus),
            'Ⱘ' => Ok(Glagolitic::CapitalLetterBigYus),
            'Ⱙ' => Ok(Glagolitic::CapitalLetterIotatedBigYus),
            'Ⱚ' => Ok(Glagolitic::CapitalLetterFita),
            'Ⱛ' => Ok(Glagolitic::CapitalLetterIzhitsa),
            'Ⱜ' => Ok(Glagolitic::CapitalLetterShtapic),
            'Ⱝ' => Ok(Glagolitic::CapitalLetterTrokutastiA),
            'Ⱞ' => Ok(Glagolitic::CapitalLetterLatinateMyslite),
            'ⰰ' => Ok(Glagolitic::SmallLetterAzu),
            'ⰱ' => Ok(Glagolitic::SmallLetterBuky),
            'ⰲ' => Ok(Glagolitic::SmallLetterVede),
            'ⰳ' => Ok(Glagolitic::SmallLetterGlagoli),
            'ⰴ' => Ok(Glagolitic::SmallLetterDobro),
            'ⰵ' => Ok(Glagolitic::SmallLetterYestu),
            'ⰶ' => Ok(Glagolitic::SmallLetterZhivete),
            'ⰷ' => Ok(Glagolitic::SmallLetterDzelo),
            'ⰸ' => Ok(Glagolitic::SmallLetterZemlja),
            'ⰹ' => Ok(Glagolitic::SmallLetterIzhe),
            'ⰺ' => Ok(Glagolitic::SmallLetterInitialIzhe),
            'ⰻ' => Ok(Glagolitic::SmallLetterI),
            'ⰼ' => Ok(Glagolitic::SmallLetterDjervi),
            'ⰽ' => Ok(Glagolitic::SmallLetterKako),
            'ⰾ' => Ok(Glagolitic::SmallLetterLjudije),
            'ⰿ' => Ok(Glagolitic::SmallLetterMyslite),
            'ⱀ' => Ok(Glagolitic::SmallLetterNashi),
            'ⱁ' => Ok(Glagolitic::SmallLetterOnu),
            'ⱂ' => Ok(Glagolitic::SmallLetterPokoji),
            'ⱃ' => Ok(Glagolitic::SmallLetterRitsi),
            'ⱄ' => Ok(Glagolitic::SmallLetterSlovo),
            'ⱅ' => Ok(Glagolitic::SmallLetterTvrido),
            'ⱆ' => Ok(Glagolitic::SmallLetterUku),
            'ⱇ' => Ok(Glagolitic::SmallLetterFritu),
            'ⱈ' => Ok(Glagolitic::SmallLetterHeru),
            'ⱉ' => Ok(Glagolitic::SmallLetterOtu),
            'ⱊ' => Ok(Glagolitic::SmallLetterPe),
            'ⱋ' => Ok(Glagolitic::SmallLetterShta),
            'ⱌ' => Ok(Glagolitic::SmallLetterTsi),
            'ⱍ' => Ok(Glagolitic::SmallLetterChrivi),
            'ⱎ' => Ok(Glagolitic::SmallLetterSha),
            'ⱏ' => Ok(Glagolitic::SmallLetterYeru),
            'ⱐ' => Ok(Glagolitic::SmallLetterYeri),
            'ⱑ' => Ok(Glagolitic::SmallLetterYati),
            'ⱒ' => Ok(Glagolitic::SmallLetterSpideryHa),
            'ⱓ' => Ok(Glagolitic::SmallLetterYu),
            'ⱔ' => Ok(Glagolitic::SmallLetterSmallYus),
            'ⱕ' => Ok(Glagolitic::SmallLetterSmallYusWithTail),
            'ⱖ' => Ok(Glagolitic::SmallLetterYo),
            'ⱗ' => Ok(Glagolitic::SmallLetterIotatedSmallYus),
            'ⱘ' => Ok(Glagolitic::SmallLetterBigYus),
            'ⱙ' => Ok(Glagolitic::SmallLetterIotatedBigYus),
            'ⱚ' => Ok(Glagolitic::SmallLetterFita),
            'ⱛ' => Ok(Glagolitic::SmallLetterIzhitsa),
            'ⱜ' => Ok(Glagolitic::SmallLetterShtapic),
            'ⱝ' => Ok(Glagolitic::SmallLetterTrokutastiA),
            'ⱞ' => Ok(Glagolitic::SmallLetterLatinateMyslite),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Glagolitic {
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

impl std::convert::TryFrom<u32> for Glagolitic {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Glagolitic {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Glagolitic {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Glagolitic::CapitalLetterAzu
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Glagolitic{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
