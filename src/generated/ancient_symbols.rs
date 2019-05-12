
/// An enum to represent all characters in the AncientSymbols block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum AncientSymbols {
    /// \u{10190}: '𐆐'
    RomanSextansSign,
    /// \u{10191}: '𐆑'
    RomanUnciaSign,
    /// \u{10192}: '𐆒'
    RomanSemunciaSign,
    /// \u{10193}: '𐆓'
    RomanSextulaSign,
    /// \u{10194}: '𐆔'
    RomanDimidiaSextulaSign,
    /// \u{10195}: '𐆕'
    RomanSiliquaSign,
    /// \u{10196}: '𐆖'
    RomanDenariusSign,
    /// \u{10197}: '𐆗'
    RomanQuinariusSign,
    /// \u{10198}: '𐆘'
    RomanSestertiusSign,
    /// \u{10199}: '𐆙'
    RomanDupondiusSign,
    /// \u{1019a}: '𐆚'
    RomanAsSign,
    /// \u{1019b}: '𐆛'
    RomanCenturialSign,
    /// \u{101a0}: '𐆠'
    GreekSymbolTauRho,
}

impl Into<char> for AncientSymbols {
    fn into(self) -> char {
        match self {
            AncientSymbols::RomanSextansSign => '𐆐',
            AncientSymbols::RomanUnciaSign => '𐆑',
            AncientSymbols::RomanSemunciaSign => '𐆒',
            AncientSymbols::RomanSextulaSign => '𐆓',
            AncientSymbols::RomanDimidiaSextulaSign => '𐆔',
            AncientSymbols::RomanSiliquaSign => '𐆕',
            AncientSymbols::RomanDenariusSign => '𐆖',
            AncientSymbols::RomanQuinariusSign => '𐆗',
            AncientSymbols::RomanSestertiusSign => '𐆘',
            AncientSymbols::RomanDupondiusSign => '𐆙',
            AncientSymbols::RomanAsSign => '𐆚',
            AncientSymbols::RomanCenturialSign => '𐆛',
            AncientSymbols::GreekSymbolTauRho => '𐆠',
        }
    }
}

impl std::convert::TryFrom<char> for AncientSymbols {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𐆐' => Ok(AncientSymbols::RomanSextansSign),
            '𐆑' => Ok(AncientSymbols::RomanUnciaSign),
            '𐆒' => Ok(AncientSymbols::RomanSemunciaSign),
            '𐆓' => Ok(AncientSymbols::RomanSextulaSign),
            '𐆔' => Ok(AncientSymbols::RomanDimidiaSextulaSign),
            '𐆕' => Ok(AncientSymbols::RomanSiliquaSign),
            '𐆖' => Ok(AncientSymbols::RomanDenariusSign),
            '𐆗' => Ok(AncientSymbols::RomanQuinariusSign),
            '𐆘' => Ok(AncientSymbols::RomanSestertiusSign),
            '𐆙' => Ok(AncientSymbols::RomanDupondiusSign),
            '𐆚' => Ok(AncientSymbols::RomanAsSign),
            '𐆛' => Ok(AncientSymbols::RomanCenturialSign),
            '𐆠' => Ok(AncientSymbols::GreekSymbolTauRho),
            _ => Err(()),
        }
    }
}

impl Into<u32> for AncientSymbols {
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

impl std::convert::TryFrom<u32> for AncientSymbols {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for AncientSymbols {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl AncientSymbols {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        AncientSymbols::RomanSextansSign
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("AncientSymbols{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
