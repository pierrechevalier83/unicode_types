
/// An enum to represent all characters in the BopomofoExtended block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum BopomofoExtended {
    /// \u{31a0}: 'ㆠ'
    BopomofoLetterBu,
    /// \u{31a1}: 'ㆡ'
    BopomofoLetterZi,
    /// \u{31a2}: 'ㆢ'
    BopomofoLetterJi,
    /// \u{31a3}: 'ㆣ'
    BopomofoLetterGu,
    /// \u{31a4}: 'ㆤ'
    BopomofoLetterEe,
    /// \u{31a5}: 'ㆥ'
    BopomofoLetterEnn,
    /// \u{31a6}: 'ㆦ'
    BopomofoLetterOo,
    /// \u{31a7}: 'ㆧ'
    BopomofoLetterOnn,
    /// \u{31a8}: 'ㆨ'
    BopomofoLetterIr,
    /// \u{31a9}: 'ㆩ'
    BopomofoLetterAnn,
    /// \u{31aa}: 'ㆪ'
    BopomofoLetterInn,
    /// \u{31ab}: 'ㆫ'
    BopomofoLetterUnn,
    /// \u{31ac}: 'ㆬ'
    BopomofoLetterIm,
    /// \u{31ad}: 'ㆭ'
    BopomofoLetterNgg,
    /// \u{31ae}: 'ㆮ'
    BopomofoLetterAinn,
    /// \u{31af}: 'ㆯ'
    BopomofoLetterAunn,
    /// \u{31b0}: 'ㆰ'
    BopomofoLetterAm,
    /// \u{31b1}: 'ㆱ'
    BopomofoLetterOm,
    /// \u{31b2}: 'ㆲ'
    BopomofoLetterOng,
    /// \u{31b3}: 'ㆳ'
    BopomofoLetterInnn,
    /// \u{31b4}: 'ㆴ'
    BopomofoFinalLetterP,
    /// \u{31b5}: 'ㆵ'
    BopomofoFinalLetterT,
    /// \u{31b6}: 'ㆶ'
    BopomofoFinalLetterK,
    /// \u{31b7}: 'ㆷ'
    BopomofoFinalLetterH,
    /// \u{31b8}: 'ㆸ'
    BopomofoLetterGh,
    /// \u{31b9}: 'ㆹ'
    BopomofoLetterLh,
    /// \u{31ba}: 'ㆺ'
    BopomofoLetterZy,
}

impl Into<char> for BopomofoExtended {
    fn into(self) -> char {
        match self {
            BopomofoExtended::BopomofoLetterBu => 'ㆠ',
            BopomofoExtended::BopomofoLetterZi => 'ㆡ',
            BopomofoExtended::BopomofoLetterJi => 'ㆢ',
            BopomofoExtended::BopomofoLetterGu => 'ㆣ',
            BopomofoExtended::BopomofoLetterEe => 'ㆤ',
            BopomofoExtended::BopomofoLetterEnn => 'ㆥ',
            BopomofoExtended::BopomofoLetterOo => 'ㆦ',
            BopomofoExtended::BopomofoLetterOnn => 'ㆧ',
            BopomofoExtended::BopomofoLetterIr => 'ㆨ',
            BopomofoExtended::BopomofoLetterAnn => 'ㆩ',
            BopomofoExtended::BopomofoLetterInn => 'ㆪ',
            BopomofoExtended::BopomofoLetterUnn => 'ㆫ',
            BopomofoExtended::BopomofoLetterIm => 'ㆬ',
            BopomofoExtended::BopomofoLetterNgg => 'ㆭ',
            BopomofoExtended::BopomofoLetterAinn => 'ㆮ',
            BopomofoExtended::BopomofoLetterAunn => 'ㆯ',
            BopomofoExtended::BopomofoLetterAm => 'ㆰ',
            BopomofoExtended::BopomofoLetterOm => 'ㆱ',
            BopomofoExtended::BopomofoLetterOng => 'ㆲ',
            BopomofoExtended::BopomofoLetterInnn => 'ㆳ',
            BopomofoExtended::BopomofoFinalLetterP => 'ㆴ',
            BopomofoExtended::BopomofoFinalLetterT => 'ㆵ',
            BopomofoExtended::BopomofoFinalLetterK => 'ㆶ',
            BopomofoExtended::BopomofoFinalLetterH => 'ㆷ',
            BopomofoExtended::BopomofoLetterGh => 'ㆸ',
            BopomofoExtended::BopomofoLetterLh => 'ㆹ',
            BopomofoExtended::BopomofoLetterZy => 'ㆺ',
        }
    }
}

impl std::convert::TryFrom<char> for BopomofoExtended {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ㆠ' => Ok(BopomofoExtended::BopomofoLetterBu),
            'ㆡ' => Ok(BopomofoExtended::BopomofoLetterZi),
            'ㆢ' => Ok(BopomofoExtended::BopomofoLetterJi),
            'ㆣ' => Ok(BopomofoExtended::BopomofoLetterGu),
            'ㆤ' => Ok(BopomofoExtended::BopomofoLetterEe),
            'ㆥ' => Ok(BopomofoExtended::BopomofoLetterEnn),
            'ㆦ' => Ok(BopomofoExtended::BopomofoLetterOo),
            'ㆧ' => Ok(BopomofoExtended::BopomofoLetterOnn),
            'ㆨ' => Ok(BopomofoExtended::BopomofoLetterIr),
            'ㆩ' => Ok(BopomofoExtended::BopomofoLetterAnn),
            'ㆪ' => Ok(BopomofoExtended::BopomofoLetterInn),
            'ㆫ' => Ok(BopomofoExtended::BopomofoLetterUnn),
            'ㆬ' => Ok(BopomofoExtended::BopomofoLetterIm),
            'ㆭ' => Ok(BopomofoExtended::BopomofoLetterNgg),
            'ㆮ' => Ok(BopomofoExtended::BopomofoLetterAinn),
            'ㆯ' => Ok(BopomofoExtended::BopomofoLetterAunn),
            'ㆰ' => Ok(BopomofoExtended::BopomofoLetterAm),
            'ㆱ' => Ok(BopomofoExtended::BopomofoLetterOm),
            'ㆲ' => Ok(BopomofoExtended::BopomofoLetterOng),
            'ㆳ' => Ok(BopomofoExtended::BopomofoLetterInnn),
            'ㆴ' => Ok(BopomofoExtended::BopomofoFinalLetterP),
            'ㆵ' => Ok(BopomofoExtended::BopomofoFinalLetterT),
            'ㆶ' => Ok(BopomofoExtended::BopomofoFinalLetterK),
            'ㆷ' => Ok(BopomofoExtended::BopomofoFinalLetterH),
            'ㆸ' => Ok(BopomofoExtended::BopomofoLetterGh),
            'ㆹ' => Ok(BopomofoExtended::BopomofoLetterLh),
            'ㆺ' => Ok(BopomofoExtended::BopomofoLetterZy),
            _ => Err(()),
        }
    }
}

impl Into<u32> for BopomofoExtended {
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

impl std::convert::TryFrom<u32> for BopomofoExtended {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for BopomofoExtended {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl BopomofoExtended {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        BopomofoExtended::BopomofoLetterBu
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            BopomofoExtended::BopomofoLetterBu => "bopomofo letter bu",
            BopomofoExtended::BopomofoLetterZi => "bopomofo letter zi",
            BopomofoExtended::BopomofoLetterJi => "bopomofo letter ji",
            BopomofoExtended::BopomofoLetterGu => "bopomofo letter gu",
            BopomofoExtended::BopomofoLetterEe => "bopomofo letter ee",
            BopomofoExtended::BopomofoLetterEnn => "bopomofo letter enn",
            BopomofoExtended::BopomofoLetterOo => "bopomofo letter oo",
            BopomofoExtended::BopomofoLetterOnn => "bopomofo letter onn",
            BopomofoExtended::BopomofoLetterIr => "bopomofo letter ir",
            BopomofoExtended::BopomofoLetterAnn => "bopomofo letter ann",
            BopomofoExtended::BopomofoLetterInn => "bopomofo letter inn",
            BopomofoExtended::BopomofoLetterUnn => "bopomofo letter unn",
            BopomofoExtended::BopomofoLetterIm => "bopomofo letter im",
            BopomofoExtended::BopomofoLetterNgg => "bopomofo letter ngg",
            BopomofoExtended::BopomofoLetterAinn => "bopomofo letter ainn",
            BopomofoExtended::BopomofoLetterAunn => "bopomofo letter aunn",
            BopomofoExtended::BopomofoLetterAm => "bopomofo letter am",
            BopomofoExtended::BopomofoLetterOm => "bopomofo letter om",
            BopomofoExtended::BopomofoLetterOng => "bopomofo letter ong",
            BopomofoExtended::BopomofoLetterInnn => "bopomofo letter innn",
            BopomofoExtended::BopomofoFinalLetterP => "bopomofo final letter p",
            BopomofoExtended::BopomofoFinalLetterT => "bopomofo final letter t",
            BopomofoExtended::BopomofoFinalLetterK => "bopomofo final letter k",
            BopomofoExtended::BopomofoFinalLetterH => "bopomofo final letter h",
            BopomofoExtended::BopomofoLetterGh => "bopomofo letter gh",
            BopomofoExtended::BopomofoLetterLh => "bopomofo letter lh",
            BopomofoExtended::BopomofoLetterZy => "bopomofo letter zy",
        }
    }
}
