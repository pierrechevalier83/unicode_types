
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
        match self {
            KhmerSymbols::KhmerSymbolPathamasat => '᧠',
            KhmerSymbols::KhmerSymbolMuoyKoet => '᧡',
            KhmerSymbols::KhmerSymbolPiiKoet => '᧢',
            KhmerSymbols::KhmerSymbolBeiKoet => '᧣',
            KhmerSymbols::KhmerSymbolBuonKoet => '᧤',
            KhmerSymbols::KhmerSymbolPramKoet => '᧥',
            KhmerSymbols::KhmerSymbolPramDashMuoyKoet => '᧦',
            KhmerSymbols::KhmerSymbolPramDashPiiKoet => '᧧',
            KhmerSymbols::KhmerSymbolPramDashBeiKoet => '᧨',
            KhmerSymbols::KhmerSymbolPramDashBuonKoet => '᧩',
            KhmerSymbols::KhmerSymbolDapKoet => '᧪',
            KhmerSymbols::KhmerSymbolDapDashMuoyKoet => '᧫',
            KhmerSymbols::KhmerSymbolDapDashPiiKoet => '᧬',
            KhmerSymbols::KhmerSymbolDapDashBeiKoet => '᧭',
            KhmerSymbols::KhmerSymbolDapDashBuonKoet => '᧮',
            KhmerSymbols::KhmerSymbolDapDashPramKoet => '᧯',
            KhmerSymbols::KhmerSymbolTuteyasat => '᧰',
            KhmerSymbols::KhmerSymbolMuoyRoc => '᧱',
            KhmerSymbols::KhmerSymbolPiiRoc => '᧲',
            KhmerSymbols::KhmerSymbolBeiRoc => '᧳',
            KhmerSymbols::KhmerSymbolBuonRoc => '᧴',
            KhmerSymbols::KhmerSymbolPramRoc => '᧵',
            KhmerSymbols::KhmerSymbolPramDashMuoyRoc => '᧶',
            KhmerSymbols::KhmerSymbolPramDashPiiRoc => '᧷',
            KhmerSymbols::KhmerSymbolPramDashBeiRoc => '᧸',
            KhmerSymbols::KhmerSymbolPramDashBuonRoc => '᧹',
            KhmerSymbols::KhmerSymbolDapRoc => '᧺',
            KhmerSymbols::KhmerSymbolDapDashMuoyRoc => '᧻',
            KhmerSymbols::KhmerSymbolDapDashPiiRoc => '᧼',
            KhmerSymbols::KhmerSymbolDapDashBeiRoc => '᧽',
            KhmerSymbols::KhmerSymbolDapDashBuonRoc => '᧾',
        }
    }
}

impl std::convert::TryFrom<char> for KhmerSymbols {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '᧠' => Ok(KhmerSymbols::KhmerSymbolPathamasat),
            '᧡' => Ok(KhmerSymbols::KhmerSymbolMuoyKoet),
            '᧢' => Ok(KhmerSymbols::KhmerSymbolPiiKoet),
            '᧣' => Ok(KhmerSymbols::KhmerSymbolBeiKoet),
            '᧤' => Ok(KhmerSymbols::KhmerSymbolBuonKoet),
            '᧥' => Ok(KhmerSymbols::KhmerSymbolPramKoet),
            '᧦' => Ok(KhmerSymbols::KhmerSymbolPramDashMuoyKoet),
            '᧧' => Ok(KhmerSymbols::KhmerSymbolPramDashPiiKoet),
            '᧨' => Ok(KhmerSymbols::KhmerSymbolPramDashBeiKoet),
            '᧩' => Ok(KhmerSymbols::KhmerSymbolPramDashBuonKoet),
            '᧪' => Ok(KhmerSymbols::KhmerSymbolDapKoet),
            '᧫' => Ok(KhmerSymbols::KhmerSymbolDapDashMuoyKoet),
            '᧬' => Ok(KhmerSymbols::KhmerSymbolDapDashPiiKoet),
            '᧭' => Ok(KhmerSymbols::KhmerSymbolDapDashBeiKoet),
            '᧮' => Ok(KhmerSymbols::KhmerSymbolDapDashBuonKoet),
            '᧯' => Ok(KhmerSymbols::KhmerSymbolDapDashPramKoet),
            '᧰' => Ok(KhmerSymbols::KhmerSymbolTuteyasat),
            '᧱' => Ok(KhmerSymbols::KhmerSymbolMuoyRoc),
            '᧲' => Ok(KhmerSymbols::KhmerSymbolPiiRoc),
            '᧳' => Ok(KhmerSymbols::KhmerSymbolBeiRoc),
            '᧴' => Ok(KhmerSymbols::KhmerSymbolBuonRoc),
            '᧵' => Ok(KhmerSymbols::KhmerSymbolPramRoc),
            '᧶' => Ok(KhmerSymbols::KhmerSymbolPramDashMuoyRoc),
            '᧷' => Ok(KhmerSymbols::KhmerSymbolPramDashPiiRoc),
            '᧸' => Ok(KhmerSymbols::KhmerSymbolPramDashBeiRoc),
            '᧹' => Ok(KhmerSymbols::KhmerSymbolPramDashBuonRoc),
            '᧺' => Ok(KhmerSymbols::KhmerSymbolDapRoc),
            '᧻' => Ok(KhmerSymbols::KhmerSymbolDapDashMuoyRoc),
            '᧼' => Ok(KhmerSymbols::KhmerSymbolDapDashPiiRoc),
            '᧽' => Ok(KhmerSymbols::KhmerSymbolDapDashBeiRoc),
            '᧾' => Ok(KhmerSymbols::KhmerSymbolDapDashBuonRoc),
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
