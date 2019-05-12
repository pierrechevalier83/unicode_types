
/// An enum to represent all characters in the CurrencySymbols block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum CurrencySymbols {
    /// \u{20a0}: '₠'
    EuroDashCurrencySign,
    /// \u{20a1}: '₡'
    ColonSign,
    /// \u{20a2}: '₢'
    CruzeiroSign,
    /// \u{20a3}: '₣'
    FrenchFrancSign,
    /// \u{20a4}: '₤'
    LiraSign,
    /// \u{20a5}: '₥'
    MillSign,
    /// \u{20a6}: '₦'
    NairaSign,
    /// \u{20a7}: '₧'
    PesetaSign,
    /// \u{20a8}: '₨'
    RupeeSign,
    /// \u{20a9}: '₩'
    WonSign,
    /// \u{20aa}: '₪'
    NewSheqelSign,
    /// \u{20ab}: '₫'
    DongSign,
    /// \u{20ac}: '€'
    EuroSign,
    /// \u{20ad}: '₭'
    KipSign,
    /// \u{20ae}: '₮'
    TugrikSign,
    /// \u{20af}: '₯'
    DrachmaSign,
    /// \u{20b0}: '₰'
    GermanPennySign,
    /// \u{20b1}: '₱'
    PesoSign,
    /// \u{20b2}: '₲'
    GuaraniSign,
    /// \u{20b3}: '₳'
    AustralSign,
    /// \u{20b4}: '₴'
    HryvniaSign,
    /// \u{20b5}: '₵'
    CediSign,
    /// \u{20b6}: '₶'
    LivreTournoisSign,
    /// \u{20b7}: '₷'
    SpesmiloSign,
    /// \u{20b8}: '₸'
    TengeSign,
    /// \u{20b9}: '₹'
    IndianRupeeSign,
    /// \u{20ba}: '₺'
    TurkishLiraSign,
    /// \u{20bb}: '₻'
    NordicMarkSign,
    /// \u{20bc}: '₼'
    ManatSign,
    /// \u{20bd}: '₽'
    RubleSign,
    /// \u{20be}: '₾'
    LariSign,
    /// \u{20bf}: '₿'
    BitcoinSign,
}

impl Into<char> for CurrencySymbols {
    fn into(self) -> char {
        match self {
            CurrencySymbols::EuroDashCurrencySign => '₠',
            CurrencySymbols::ColonSign => '₡',
            CurrencySymbols::CruzeiroSign => '₢',
            CurrencySymbols::FrenchFrancSign => '₣',
            CurrencySymbols::LiraSign => '₤',
            CurrencySymbols::MillSign => '₥',
            CurrencySymbols::NairaSign => '₦',
            CurrencySymbols::PesetaSign => '₧',
            CurrencySymbols::RupeeSign => '₨',
            CurrencySymbols::WonSign => '₩',
            CurrencySymbols::NewSheqelSign => '₪',
            CurrencySymbols::DongSign => '₫',
            CurrencySymbols::EuroSign => '€',
            CurrencySymbols::KipSign => '₭',
            CurrencySymbols::TugrikSign => '₮',
            CurrencySymbols::DrachmaSign => '₯',
            CurrencySymbols::GermanPennySign => '₰',
            CurrencySymbols::PesoSign => '₱',
            CurrencySymbols::GuaraniSign => '₲',
            CurrencySymbols::AustralSign => '₳',
            CurrencySymbols::HryvniaSign => '₴',
            CurrencySymbols::CediSign => '₵',
            CurrencySymbols::LivreTournoisSign => '₶',
            CurrencySymbols::SpesmiloSign => '₷',
            CurrencySymbols::TengeSign => '₸',
            CurrencySymbols::IndianRupeeSign => '₹',
            CurrencySymbols::TurkishLiraSign => '₺',
            CurrencySymbols::NordicMarkSign => '₻',
            CurrencySymbols::ManatSign => '₼',
            CurrencySymbols::RubleSign => '₽',
            CurrencySymbols::LariSign => '₾',
            CurrencySymbols::BitcoinSign => '₿',
        }
    }
}

impl std::convert::TryFrom<char> for CurrencySymbols {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '₠' => Ok(CurrencySymbols::EuroDashCurrencySign),
            '₡' => Ok(CurrencySymbols::ColonSign),
            '₢' => Ok(CurrencySymbols::CruzeiroSign),
            '₣' => Ok(CurrencySymbols::FrenchFrancSign),
            '₤' => Ok(CurrencySymbols::LiraSign),
            '₥' => Ok(CurrencySymbols::MillSign),
            '₦' => Ok(CurrencySymbols::NairaSign),
            '₧' => Ok(CurrencySymbols::PesetaSign),
            '₨' => Ok(CurrencySymbols::RupeeSign),
            '₩' => Ok(CurrencySymbols::WonSign),
            '₪' => Ok(CurrencySymbols::NewSheqelSign),
            '₫' => Ok(CurrencySymbols::DongSign),
            '€' => Ok(CurrencySymbols::EuroSign),
            '₭' => Ok(CurrencySymbols::KipSign),
            '₮' => Ok(CurrencySymbols::TugrikSign),
            '₯' => Ok(CurrencySymbols::DrachmaSign),
            '₰' => Ok(CurrencySymbols::GermanPennySign),
            '₱' => Ok(CurrencySymbols::PesoSign),
            '₲' => Ok(CurrencySymbols::GuaraniSign),
            '₳' => Ok(CurrencySymbols::AustralSign),
            '₴' => Ok(CurrencySymbols::HryvniaSign),
            '₵' => Ok(CurrencySymbols::CediSign),
            '₶' => Ok(CurrencySymbols::LivreTournoisSign),
            '₷' => Ok(CurrencySymbols::SpesmiloSign),
            '₸' => Ok(CurrencySymbols::TengeSign),
            '₹' => Ok(CurrencySymbols::IndianRupeeSign),
            '₺' => Ok(CurrencySymbols::TurkishLiraSign),
            '₻' => Ok(CurrencySymbols::NordicMarkSign),
            '₼' => Ok(CurrencySymbols::ManatSign),
            '₽' => Ok(CurrencySymbols::RubleSign),
            '₾' => Ok(CurrencySymbols::LariSign),
            '₿' => Ok(CurrencySymbols::BitcoinSign),
            _ => Err(()),
        }
    }
}

impl Into<u32> for CurrencySymbols {
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

impl std::convert::TryFrom<u32> for CurrencySymbols {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for CurrencySymbols {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl CurrencySymbols {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        CurrencySymbols::EuroDashCurrencySign
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            CurrencySymbols::EuroDashCurrencySign => "euro-currency sign",
            CurrencySymbols::ColonSign => "colon sign",
            CurrencySymbols::CruzeiroSign => "cruzeiro sign",
            CurrencySymbols::FrenchFrancSign => "french franc sign",
            CurrencySymbols::LiraSign => "lira sign",
            CurrencySymbols::MillSign => "mill sign",
            CurrencySymbols::NairaSign => "naira sign",
            CurrencySymbols::PesetaSign => "peseta sign",
            CurrencySymbols::RupeeSign => "rupee sign",
            CurrencySymbols::WonSign => "won sign",
            CurrencySymbols::NewSheqelSign => "new sheqel sign",
            CurrencySymbols::DongSign => "dong sign",
            CurrencySymbols::EuroSign => "euro sign",
            CurrencySymbols::KipSign => "kip sign",
            CurrencySymbols::TugrikSign => "tugrik sign",
            CurrencySymbols::DrachmaSign => "drachma sign",
            CurrencySymbols::GermanPennySign => "german penny sign",
            CurrencySymbols::PesoSign => "peso sign",
            CurrencySymbols::GuaraniSign => "guarani sign",
            CurrencySymbols::AustralSign => "austral sign",
            CurrencySymbols::HryvniaSign => "hryvnia sign",
            CurrencySymbols::CediSign => "cedi sign",
            CurrencySymbols::LivreTournoisSign => "livre tournois sign",
            CurrencySymbols::SpesmiloSign => "spesmilo sign",
            CurrencySymbols::TengeSign => "tenge sign",
            CurrencySymbols::IndianRupeeSign => "indian rupee sign",
            CurrencySymbols::TurkishLiraSign => "turkish lira sign",
            CurrencySymbols::NordicMarkSign => "nordic mark sign",
            CurrencySymbols::ManatSign => "manat sign",
            CurrencySymbols::RubleSign => "ruble sign",
            CurrencySymbols::LariSign => "lari sign",
            CurrencySymbols::BitcoinSign => "bitcoin sign",
        }
    }
}
