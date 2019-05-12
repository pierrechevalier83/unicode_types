/// \u{20a0} → \u{20cf}
///
/// ₠ ₡ ₢ ₣ ₤ ₥ ₦ ₧ ₨ ₩ ₪ ₫ € ₭ ₮ ₯\
/// ₰ ₱ ₲ ₳ ₴ ₵ ₶ ₷ ₸ ₹ ₺ ₻ ₼ ₽ ₾ ₿\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{20a0}: '₠'
    pub const EURO_DASH_CURRENCY_SIGN: char = '₠';
    /// \u{20a1}: '₡'
    pub const COLON_SIGN: char = '₡';
    /// \u{20a2}: '₢'
    pub const CRUZEIRO_SIGN: char = '₢';
    /// \u{20a3}: '₣'
    pub const FRENCH_FRANC_SIGN: char = '₣';
    /// \u{20a4}: '₤'
    pub const LIRA_SIGN: char = '₤';
    /// \u{20a5}: '₥'
    pub const MILL_SIGN: char = '₥';
    /// \u{20a6}: '₦'
    pub const NAIRA_SIGN: char = '₦';
    /// \u{20a7}: '₧'
    pub const PESETA_SIGN: char = '₧';
    /// \u{20a8}: '₨'
    pub const RUPEE_SIGN: char = '₨';
    /// \u{20a9}: '₩'
    pub const WON_SIGN: char = '₩';
    /// \u{20aa}: '₪'
    pub const NEW_SHEQEL_SIGN: char = '₪';
    /// \u{20ab}: '₫'
    pub const DONG_SIGN: char = '₫';
    /// \u{20ac}: '€'
    pub const EURO_SIGN: char = '€';
    /// \u{20ad}: '₭'
    pub const KIP_SIGN: char = '₭';
    /// \u{20ae}: '₮'
    pub const TUGRIK_SIGN: char = '₮';
    /// \u{20af}: '₯'
    pub const DRACHMA_SIGN: char = '₯';
    /// \u{20b0}: '₰'
    pub const GERMAN_PENNY_SIGN: char = '₰';
    /// \u{20b1}: '₱'
    pub const PESO_SIGN: char = '₱';
    /// \u{20b2}: '₲'
    pub const GUARANI_SIGN: char = '₲';
    /// \u{20b3}: '₳'
    pub const AUSTRAL_SIGN: char = '₳';
    /// \u{20b4}: '₴'
    pub const HRYVNIA_SIGN: char = '₴';
    /// \u{20b5}: '₵'
    pub const CEDI_SIGN: char = '₵';
    /// \u{20b6}: '₶'
    pub const LIVRE_TOURNOIS_SIGN: char = '₶';
    /// \u{20b7}: '₷'
    pub const SPESMILO_SIGN: char = '₷';
    /// \u{20b8}: '₸'
    pub const TENGE_SIGN: char = '₸';
    /// \u{20b9}: '₹'
    pub const INDIAN_RUPEE_SIGN: char = '₹';
    /// \u{20ba}: '₺'
    pub const TURKISH_LIRA_SIGN: char = '₺';
    /// \u{20bb}: '₻'
    pub const NORDIC_MARK_SIGN: char = '₻';
    /// \u{20bc}: '₼'
    pub const MANAT_SIGN: char = '₼';
    /// \u{20bd}: '₽'
    pub const RUBLE_SIGN: char = '₽';
    /// \u{20be}: '₾'
    pub const LARI_SIGN: char = '₾';
    /// \u{20bf}: '₿'
    pub const BITCOIN_SIGN: char = '₿';
}

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
        use constants::*;
        match self {
            CurrencySymbols::EuroDashCurrencySign => EURO_DASH_CURRENCY_SIGN,
            CurrencySymbols::ColonSign => COLON_SIGN,
            CurrencySymbols::CruzeiroSign => CRUZEIRO_SIGN,
            CurrencySymbols::FrenchFrancSign => FRENCH_FRANC_SIGN,
            CurrencySymbols::LiraSign => LIRA_SIGN,
            CurrencySymbols::MillSign => MILL_SIGN,
            CurrencySymbols::NairaSign => NAIRA_SIGN,
            CurrencySymbols::PesetaSign => PESETA_SIGN,
            CurrencySymbols::RupeeSign => RUPEE_SIGN,
            CurrencySymbols::WonSign => WON_SIGN,
            CurrencySymbols::NewSheqelSign => NEW_SHEQEL_SIGN,
            CurrencySymbols::DongSign => DONG_SIGN,
            CurrencySymbols::EuroSign => EURO_SIGN,
            CurrencySymbols::KipSign => KIP_SIGN,
            CurrencySymbols::TugrikSign => TUGRIK_SIGN,
            CurrencySymbols::DrachmaSign => DRACHMA_SIGN,
            CurrencySymbols::GermanPennySign => GERMAN_PENNY_SIGN,
            CurrencySymbols::PesoSign => PESO_SIGN,
            CurrencySymbols::GuaraniSign => GUARANI_SIGN,
            CurrencySymbols::AustralSign => AUSTRAL_SIGN,
            CurrencySymbols::HryvniaSign => HRYVNIA_SIGN,
            CurrencySymbols::CediSign => CEDI_SIGN,
            CurrencySymbols::LivreTournoisSign => LIVRE_TOURNOIS_SIGN,
            CurrencySymbols::SpesmiloSign => SPESMILO_SIGN,
            CurrencySymbols::TengeSign => TENGE_SIGN,
            CurrencySymbols::IndianRupeeSign => INDIAN_RUPEE_SIGN,
            CurrencySymbols::TurkishLiraSign => TURKISH_LIRA_SIGN,
            CurrencySymbols::NordicMarkSign => NORDIC_MARK_SIGN,
            CurrencySymbols::ManatSign => MANAT_SIGN,
            CurrencySymbols::RubleSign => RUBLE_SIGN,
            CurrencySymbols::LariSign => LARI_SIGN,
            CurrencySymbols::BitcoinSign => BITCOIN_SIGN,
        }
    }
}

impl std::convert::TryFrom<char> for CurrencySymbols {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            EURO_DASH_CURRENCY_SIGN => Ok(CurrencySymbols::EuroDashCurrencySign),
            COLON_SIGN => Ok(CurrencySymbols::ColonSign),
            CRUZEIRO_SIGN => Ok(CurrencySymbols::CruzeiroSign),
            FRENCH_FRANC_SIGN => Ok(CurrencySymbols::FrenchFrancSign),
            LIRA_SIGN => Ok(CurrencySymbols::LiraSign),
            MILL_SIGN => Ok(CurrencySymbols::MillSign),
            NAIRA_SIGN => Ok(CurrencySymbols::NairaSign),
            PESETA_SIGN => Ok(CurrencySymbols::PesetaSign),
            RUPEE_SIGN => Ok(CurrencySymbols::RupeeSign),
            WON_SIGN => Ok(CurrencySymbols::WonSign),
            NEW_SHEQEL_SIGN => Ok(CurrencySymbols::NewSheqelSign),
            DONG_SIGN => Ok(CurrencySymbols::DongSign),
            EURO_SIGN => Ok(CurrencySymbols::EuroSign),
            KIP_SIGN => Ok(CurrencySymbols::KipSign),
            TUGRIK_SIGN => Ok(CurrencySymbols::TugrikSign),
            DRACHMA_SIGN => Ok(CurrencySymbols::DrachmaSign),
            GERMAN_PENNY_SIGN => Ok(CurrencySymbols::GermanPennySign),
            PESO_SIGN => Ok(CurrencySymbols::PesoSign),
            GUARANI_SIGN => Ok(CurrencySymbols::GuaraniSign),
            AUSTRAL_SIGN => Ok(CurrencySymbols::AustralSign),
            HRYVNIA_SIGN => Ok(CurrencySymbols::HryvniaSign),
            CEDI_SIGN => Ok(CurrencySymbols::CediSign),
            LIVRE_TOURNOIS_SIGN => Ok(CurrencySymbols::LivreTournoisSign),
            SPESMILO_SIGN => Ok(CurrencySymbols::SpesmiloSign),
            TENGE_SIGN => Ok(CurrencySymbols::TengeSign),
            INDIAN_RUPEE_SIGN => Ok(CurrencySymbols::IndianRupeeSign),
            TURKISH_LIRA_SIGN => Ok(CurrencySymbols::TurkishLiraSign),
            NORDIC_MARK_SIGN => Ok(CurrencySymbols::NordicMarkSign),
            MANAT_SIGN => Ok(CurrencySymbols::ManatSign),
            RUBLE_SIGN => Ok(CurrencySymbols::RubleSign),
            LARI_SIGN => Ok(CurrencySymbols::LariSign),
            BITCOIN_SIGN => Ok(CurrencySymbols::BitcoinSign),
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
