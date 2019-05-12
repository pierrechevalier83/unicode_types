/// A number of constants to give a name to all characters in this block.
mod constants {
    /// \u{19e0}: '᧠'
    pub const KHMER_SYMBOL_PATHAMASAT: char = '᧠';
    /// \u{19e1}: '᧡'
    pub const KHMER_SYMBOL_MUOY_KOET: char = '᧡';
    /// \u{19e2}: '᧢'
    pub const KHMER_SYMBOL_PII_KOET: char = '᧢';
    /// \u{19e3}: '᧣'
    pub const KHMER_SYMBOL_BEI_KOET: char = '᧣';
    /// \u{19e4}: '᧤'
    pub const KHMER_SYMBOL_BUON_KOET: char = '᧤';
    /// \u{19e5}: '᧥'
    pub const KHMER_SYMBOL_PRAM_KOET: char = '᧥';
    /// \u{19e6}: '᧦'
    pub const KHMER_SYMBOL_PRAM_DASH_MUOY_KOET: char = '᧦';
    /// \u{19e7}: '᧧'
    pub const KHMER_SYMBOL_PRAM_DASH_PII_KOET: char = '᧧';
    /// \u{19e8}: '᧨'
    pub const KHMER_SYMBOL_PRAM_DASH_BEI_KOET: char = '᧨';
    /// \u{19e9}: '᧩'
    pub const KHMER_SYMBOL_PRAM_DASH_BUON_KOET: char = '᧩';
    /// \u{19ea}: '᧪'
    pub const KHMER_SYMBOL_DAP_KOET: char = '᧪';
    /// \u{19eb}: '᧫'
    pub const KHMER_SYMBOL_DAP_DASH_MUOY_KOET: char = '᧫';
    /// \u{19ec}: '᧬'
    pub const KHMER_SYMBOL_DAP_DASH_PII_KOET: char = '᧬';
    /// \u{19ed}: '᧭'
    pub const KHMER_SYMBOL_DAP_DASH_BEI_KOET: char = '᧭';
    /// \u{19ee}: '᧮'
    pub const KHMER_SYMBOL_DAP_DASH_BUON_KOET: char = '᧮';
    /// \u{19ef}: '᧯'
    pub const KHMER_SYMBOL_DAP_DASH_PRAM_KOET: char = '᧯';
    /// \u{19f0}: '᧰'
    pub const KHMER_SYMBOL_TUTEYASAT: char = '᧰';
    /// \u{19f1}: '᧱'
    pub const KHMER_SYMBOL_MUOY_ROC: char = '᧱';
    /// \u{19f2}: '᧲'
    pub const KHMER_SYMBOL_PII_ROC: char = '᧲';
    /// \u{19f3}: '᧳'
    pub const KHMER_SYMBOL_BEI_ROC: char = '᧳';
    /// \u{19f4}: '᧴'
    pub const KHMER_SYMBOL_BUON_ROC: char = '᧴';
    /// \u{19f5}: '᧵'
    pub const KHMER_SYMBOL_PRAM_ROC: char = '᧵';
    /// \u{19f6}: '᧶'
    pub const KHMER_SYMBOL_PRAM_DASH_MUOY_ROC: char = '᧶';
    /// \u{19f7}: '᧷'
    pub const KHMER_SYMBOL_PRAM_DASH_PII_ROC: char = '᧷';
    /// \u{19f8}: '᧸'
    pub const KHMER_SYMBOL_PRAM_DASH_BEI_ROC: char = '᧸';
    /// \u{19f9}: '᧹'
    pub const KHMER_SYMBOL_PRAM_DASH_BUON_ROC: char = '᧹';
    /// \u{19fa}: '᧺'
    pub const KHMER_SYMBOL_DAP_ROC: char = '᧺';
    /// \u{19fb}: '᧻'
    pub const KHMER_SYMBOL_DAP_DASH_MUOY_ROC: char = '᧻';
    /// \u{19fc}: '᧼'
    pub const KHMER_SYMBOL_DAP_DASH_PII_ROC: char = '᧼';
    /// \u{19fd}: '᧽'
    pub const KHMER_SYMBOL_DAP_DASH_BEI_ROC: char = '᧽';
    /// \u{19fe}: '᧾'
    pub const KHMER_SYMBOL_DAP_DASH_BUON_ROC: char = '᧾';
}

/// An enum to represent all characters in the KhmerSymbols block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum KhmerSymbols {
    /// \u{19e0}: '᧠'
    KhmerSymbolPathamasat,
    /// \u{19e1}: '᧡'
    KhmerSymbolMuoyKoet,
    /// \u{19e2}: '᧢'
    KhmerSymbolPiiKoet,
    /// \u{19e3}: '᧣'
    KhmerSymbolBeiKoet,
    /// \u{19e4}: '᧤'
    KhmerSymbolBuonKoet,
    /// \u{19e5}: '᧥'
    KhmerSymbolPramKoet,
    /// \u{19e6}: '᧦'
    KhmerSymbolPramDashMuoyKoet,
    /// \u{19e7}: '᧧'
    KhmerSymbolPramDashPiiKoet,
    /// \u{19e8}: '᧨'
    KhmerSymbolPramDashBeiKoet,
    /// \u{19e9}: '᧩'
    KhmerSymbolPramDashBuonKoet,
    /// \u{19ea}: '᧪'
    KhmerSymbolDapKoet,
    /// \u{19eb}: '᧫'
    KhmerSymbolDapDashMuoyKoet,
    /// \u{19ec}: '᧬'
    KhmerSymbolDapDashPiiKoet,
    /// \u{19ed}: '᧭'
    KhmerSymbolDapDashBeiKoet,
    /// \u{19ee}: '᧮'
    KhmerSymbolDapDashBuonKoet,
    /// \u{19ef}: '᧯'
    KhmerSymbolDapDashPramKoet,
    /// \u{19f0}: '᧰'
    KhmerSymbolTuteyasat,
    /// \u{19f1}: '᧱'
    KhmerSymbolMuoyRoc,
    /// \u{19f2}: '᧲'
    KhmerSymbolPiiRoc,
    /// \u{19f3}: '᧳'
    KhmerSymbolBeiRoc,
    /// \u{19f4}: '᧴'
    KhmerSymbolBuonRoc,
    /// \u{19f5}: '᧵'
    KhmerSymbolPramRoc,
    /// \u{19f6}: '᧶'
    KhmerSymbolPramDashMuoyRoc,
    /// \u{19f7}: '᧷'
    KhmerSymbolPramDashPiiRoc,
    /// \u{19f8}: '᧸'
    KhmerSymbolPramDashBeiRoc,
    /// \u{19f9}: '᧹'
    KhmerSymbolPramDashBuonRoc,
    /// \u{19fa}: '᧺'
    KhmerSymbolDapRoc,
    /// \u{19fb}: '᧻'
    KhmerSymbolDapDashMuoyRoc,
    /// \u{19fc}: '᧼'
    KhmerSymbolDapDashPiiRoc,
    /// \u{19fd}: '᧽'
    KhmerSymbolDapDashBeiRoc,
    /// \u{19fe}: '᧾'
    KhmerSymbolDapDashBuonRoc,
}

impl Into<char> for KhmerSymbols {
    fn into(self) -> char {
        use constants::*;
        match self {
            KhmerSymbols::KhmerSymbolPathamasat => KHMER_SYMBOL_PATHAMASAT,
            KhmerSymbols::KhmerSymbolMuoyKoet => KHMER_SYMBOL_MUOY_KOET,
            KhmerSymbols::KhmerSymbolPiiKoet => KHMER_SYMBOL_PII_KOET,
            KhmerSymbols::KhmerSymbolBeiKoet => KHMER_SYMBOL_BEI_KOET,
            KhmerSymbols::KhmerSymbolBuonKoet => KHMER_SYMBOL_BUON_KOET,
            KhmerSymbols::KhmerSymbolPramKoet => KHMER_SYMBOL_PRAM_KOET,
            KhmerSymbols::KhmerSymbolPramDashMuoyKoet => KHMER_SYMBOL_PRAM_DASH_MUOY_KOET,
            KhmerSymbols::KhmerSymbolPramDashPiiKoet => KHMER_SYMBOL_PRAM_DASH_PII_KOET,
            KhmerSymbols::KhmerSymbolPramDashBeiKoet => KHMER_SYMBOL_PRAM_DASH_BEI_KOET,
            KhmerSymbols::KhmerSymbolPramDashBuonKoet => KHMER_SYMBOL_PRAM_DASH_BUON_KOET,
            KhmerSymbols::KhmerSymbolDapKoet => KHMER_SYMBOL_DAP_KOET,
            KhmerSymbols::KhmerSymbolDapDashMuoyKoet => KHMER_SYMBOL_DAP_DASH_MUOY_KOET,
            KhmerSymbols::KhmerSymbolDapDashPiiKoet => KHMER_SYMBOL_DAP_DASH_PII_KOET,
            KhmerSymbols::KhmerSymbolDapDashBeiKoet => KHMER_SYMBOL_DAP_DASH_BEI_KOET,
            KhmerSymbols::KhmerSymbolDapDashBuonKoet => KHMER_SYMBOL_DAP_DASH_BUON_KOET,
            KhmerSymbols::KhmerSymbolDapDashPramKoet => KHMER_SYMBOL_DAP_DASH_PRAM_KOET,
            KhmerSymbols::KhmerSymbolTuteyasat => KHMER_SYMBOL_TUTEYASAT,
            KhmerSymbols::KhmerSymbolMuoyRoc => KHMER_SYMBOL_MUOY_ROC,
            KhmerSymbols::KhmerSymbolPiiRoc => KHMER_SYMBOL_PII_ROC,
            KhmerSymbols::KhmerSymbolBeiRoc => KHMER_SYMBOL_BEI_ROC,
            KhmerSymbols::KhmerSymbolBuonRoc => KHMER_SYMBOL_BUON_ROC,
            KhmerSymbols::KhmerSymbolPramRoc => KHMER_SYMBOL_PRAM_ROC,
            KhmerSymbols::KhmerSymbolPramDashMuoyRoc => KHMER_SYMBOL_PRAM_DASH_MUOY_ROC,
            KhmerSymbols::KhmerSymbolPramDashPiiRoc => KHMER_SYMBOL_PRAM_DASH_PII_ROC,
            KhmerSymbols::KhmerSymbolPramDashBeiRoc => KHMER_SYMBOL_PRAM_DASH_BEI_ROC,
            KhmerSymbols::KhmerSymbolPramDashBuonRoc => KHMER_SYMBOL_PRAM_DASH_BUON_ROC,
            KhmerSymbols::KhmerSymbolDapRoc => KHMER_SYMBOL_DAP_ROC,
            KhmerSymbols::KhmerSymbolDapDashMuoyRoc => KHMER_SYMBOL_DAP_DASH_MUOY_ROC,
            KhmerSymbols::KhmerSymbolDapDashPiiRoc => KHMER_SYMBOL_DAP_DASH_PII_ROC,
            KhmerSymbols::KhmerSymbolDapDashBeiRoc => KHMER_SYMBOL_DAP_DASH_BEI_ROC,
            KhmerSymbols::KhmerSymbolDapDashBuonRoc => KHMER_SYMBOL_DAP_DASH_BUON_ROC,
        }
    }
}

impl std::convert::TryFrom<char> for KhmerSymbols {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            KHMER_SYMBOL_PATHAMASAT => Ok(KhmerSymbols::KhmerSymbolPathamasat),
            KHMER_SYMBOL_MUOY_KOET => Ok(KhmerSymbols::KhmerSymbolMuoyKoet),
            KHMER_SYMBOL_PII_KOET => Ok(KhmerSymbols::KhmerSymbolPiiKoet),
            KHMER_SYMBOL_BEI_KOET => Ok(KhmerSymbols::KhmerSymbolBeiKoet),
            KHMER_SYMBOL_BUON_KOET => Ok(KhmerSymbols::KhmerSymbolBuonKoet),
            KHMER_SYMBOL_PRAM_KOET => Ok(KhmerSymbols::KhmerSymbolPramKoet),
            KHMER_SYMBOL_PRAM_DASH_MUOY_KOET => Ok(KhmerSymbols::KhmerSymbolPramDashMuoyKoet),
            KHMER_SYMBOL_PRAM_DASH_PII_KOET => Ok(KhmerSymbols::KhmerSymbolPramDashPiiKoet),
            KHMER_SYMBOL_PRAM_DASH_BEI_KOET => Ok(KhmerSymbols::KhmerSymbolPramDashBeiKoet),
            KHMER_SYMBOL_PRAM_DASH_BUON_KOET => Ok(KhmerSymbols::KhmerSymbolPramDashBuonKoet),
            KHMER_SYMBOL_DAP_KOET => Ok(KhmerSymbols::KhmerSymbolDapKoet),
            KHMER_SYMBOL_DAP_DASH_MUOY_KOET => Ok(KhmerSymbols::KhmerSymbolDapDashMuoyKoet),
            KHMER_SYMBOL_DAP_DASH_PII_KOET => Ok(KhmerSymbols::KhmerSymbolDapDashPiiKoet),
            KHMER_SYMBOL_DAP_DASH_BEI_KOET => Ok(KhmerSymbols::KhmerSymbolDapDashBeiKoet),
            KHMER_SYMBOL_DAP_DASH_BUON_KOET => Ok(KhmerSymbols::KhmerSymbolDapDashBuonKoet),
            KHMER_SYMBOL_DAP_DASH_PRAM_KOET => Ok(KhmerSymbols::KhmerSymbolDapDashPramKoet),
            KHMER_SYMBOL_TUTEYASAT => Ok(KhmerSymbols::KhmerSymbolTuteyasat),
            KHMER_SYMBOL_MUOY_ROC => Ok(KhmerSymbols::KhmerSymbolMuoyRoc),
            KHMER_SYMBOL_PII_ROC => Ok(KhmerSymbols::KhmerSymbolPiiRoc),
            KHMER_SYMBOL_BEI_ROC => Ok(KhmerSymbols::KhmerSymbolBeiRoc),
            KHMER_SYMBOL_BUON_ROC => Ok(KhmerSymbols::KhmerSymbolBuonRoc),
            KHMER_SYMBOL_PRAM_ROC => Ok(KhmerSymbols::KhmerSymbolPramRoc),
            KHMER_SYMBOL_PRAM_DASH_MUOY_ROC => Ok(KhmerSymbols::KhmerSymbolPramDashMuoyRoc),
            KHMER_SYMBOL_PRAM_DASH_PII_ROC => Ok(KhmerSymbols::KhmerSymbolPramDashPiiRoc),
            KHMER_SYMBOL_PRAM_DASH_BEI_ROC => Ok(KhmerSymbols::KhmerSymbolPramDashBeiRoc),
            KHMER_SYMBOL_PRAM_DASH_BUON_ROC => Ok(KhmerSymbols::KhmerSymbolPramDashBuonRoc),
            KHMER_SYMBOL_DAP_ROC => Ok(KhmerSymbols::KhmerSymbolDapRoc),
            KHMER_SYMBOL_DAP_DASH_MUOY_ROC => Ok(KhmerSymbols::KhmerSymbolDapDashMuoyRoc),
            KHMER_SYMBOL_DAP_DASH_PII_ROC => Ok(KhmerSymbols::KhmerSymbolDapDashPiiRoc),
            KHMER_SYMBOL_DAP_DASH_BEI_ROC => Ok(KhmerSymbols::KhmerSymbolDapDashBeiRoc),
            KHMER_SYMBOL_DAP_DASH_BUON_ROC => Ok(KhmerSymbols::KhmerSymbolDapDashBuonRoc),
            _ => Err(()),
        }
    }
}

impl Into<u32> for KhmerSymbols {
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

impl std::convert::TryFrom<u32> for KhmerSymbols {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for KhmerSymbols {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl KhmerSymbols {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        KhmerSymbols::KhmerSymbolPathamasat
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            KhmerSymbols::KhmerSymbolPathamasat => "khmer symbol pathamasat",
            KhmerSymbols::KhmerSymbolMuoyKoet => "khmer symbol muoy koet",
            KhmerSymbols::KhmerSymbolPiiKoet => "khmer symbol pii koet",
            KhmerSymbols::KhmerSymbolBeiKoet => "khmer symbol bei koet",
            KhmerSymbols::KhmerSymbolBuonKoet => "khmer symbol buon koet",
            KhmerSymbols::KhmerSymbolPramKoet => "khmer symbol pram koet",
            KhmerSymbols::KhmerSymbolPramDashMuoyKoet => "khmer symbol pram-muoy koet",
            KhmerSymbols::KhmerSymbolPramDashPiiKoet => "khmer symbol pram-pii koet",
            KhmerSymbols::KhmerSymbolPramDashBeiKoet => "khmer symbol pram-bei koet",
            KhmerSymbols::KhmerSymbolPramDashBuonKoet => "khmer symbol pram-buon koet",
            KhmerSymbols::KhmerSymbolDapKoet => "khmer symbol dap koet",
            KhmerSymbols::KhmerSymbolDapDashMuoyKoet => "khmer symbol dap-muoy koet",
            KhmerSymbols::KhmerSymbolDapDashPiiKoet => "khmer symbol dap-pii koet",
            KhmerSymbols::KhmerSymbolDapDashBeiKoet => "khmer symbol dap-bei koet",
            KhmerSymbols::KhmerSymbolDapDashBuonKoet => "khmer symbol dap-buon koet",
            KhmerSymbols::KhmerSymbolDapDashPramKoet => "khmer symbol dap-pram koet",
            KhmerSymbols::KhmerSymbolTuteyasat => "khmer symbol tuteyasat",
            KhmerSymbols::KhmerSymbolMuoyRoc => "khmer symbol muoy roc",
            KhmerSymbols::KhmerSymbolPiiRoc => "khmer symbol pii roc",
            KhmerSymbols::KhmerSymbolBeiRoc => "khmer symbol bei roc",
            KhmerSymbols::KhmerSymbolBuonRoc => "khmer symbol buon roc",
            KhmerSymbols::KhmerSymbolPramRoc => "khmer symbol pram roc",
            KhmerSymbols::KhmerSymbolPramDashMuoyRoc => "khmer symbol pram-muoy roc",
            KhmerSymbols::KhmerSymbolPramDashPiiRoc => "khmer symbol pram-pii roc",
            KhmerSymbols::KhmerSymbolPramDashBeiRoc => "khmer symbol pram-bei roc",
            KhmerSymbols::KhmerSymbolPramDashBuonRoc => "khmer symbol pram-buon roc",
            KhmerSymbols::KhmerSymbolDapRoc => "khmer symbol dap roc",
            KhmerSymbols::KhmerSymbolDapDashMuoyRoc => "khmer symbol dap-muoy roc",
            KhmerSymbols::KhmerSymbolDapDashPiiRoc => "khmer symbol dap-pii roc",
            KhmerSymbols::KhmerSymbolDapDashBeiRoc => "khmer symbol dap-bei roc",
            KhmerSymbols::KhmerSymbolDapDashBuonRoc => "khmer symbol dap-buon roc",
        }
    }
}
