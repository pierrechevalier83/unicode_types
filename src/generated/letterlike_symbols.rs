
/// An enum to represent all characters in the LetterlikeSymbols block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum LetterlikeSymbols {
    /// \u{2100}: '℀'
    AccountOf,
    /// \u{2101}: '℁'
    AddressedToTheSubject,
    /// \u{2102}: 'ℂ'
    DoubleDashStruckCapitalC,
    /// \u{2103}: '℃'
    DegreeCelsius,
    /// \u{2104}: '℄'
    CentreLineSymbol,
    /// \u{2105}: '℅'
    CareOf,
    /// \u{2106}: '℆'
    CadaUna,
    /// \u{2107}: 'ℇ'
    EulerConstant,
    /// \u{2108}: '℈'
    Scruple,
    /// \u{2109}: '℉'
    DegreeFahrenheit,
    /// \u{210a}: 'ℊ'
    ScriptSmallG,
    /// \u{210b}: 'ℋ'
    ScriptCapitalH,
    /// \u{210c}: 'ℌ'
    BlackDashLetterCapitalH,
    /// \u{210d}: 'ℍ'
    DoubleDashStruckCapitalH,
    /// \u{210e}: 'ℎ'
    PlanckConstant,
    /// \u{210f}: 'ℏ'
    PlanckConstantOverTwoPi,
    /// \u{2110}: 'ℐ'
    ScriptCapitalI,
    /// \u{2111}: 'ℑ'
    BlackDashLetterCapitalI,
    /// \u{2112}: 'ℒ'
    ScriptCapitalL,
    /// \u{2113}: 'ℓ'
    ScriptSmallL,
    /// \u{2114}: '℔'
    LBBarSymbol,
    /// \u{2115}: 'ℕ'
    DoubleDashStruckCapitalN,
    /// \u{2116}: '№'
    NumeroSign,
    /// \u{2117}: '℗'
    SoundRecordingCopyright,
    /// \u{2118}: '℘'
    ScriptCapitalP,
    /// \u{2119}: 'ℙ'
    DoubleDashStruckCapitalP,
    /// \u{211a}: 'ℚ'
    DoubleDashStruckCapitalQ,
    /// \u{211b}: 'ℛ'
    ScriptCapitalR,
    /// \u{211c}: 'ℜ'
    BlackDashLetterCapitalR,
    /// \u{211d}: 'ℝ'
    DoubleDashStruckCapitalR,
    /// \u{211e}: '℞'
    PrescriptionTake,
    /// \u{211f}: '℟'
    Response,
    /// \u{2120}: '℠'
    ServiceMark,
    /// \u{2121}: '℡'
    TelephoneSign,
    /// \u{2122}: '™'
    TradeMarkSign,
    /// \u{2123}: '℣'
    Versicle,
    /// \u{2124}: 'ℤ'
    DoubleDashStruckCapitalZ,
    /// \u{2125}: '℥'
    OunceSign,
    /// \u{2126}: 'Ω'
    OhmSign,
    /// \u{2127}: '℧'
    InvertedOhmSign,
    /// \u{2128}: 'ℨ'
    BlackDashLetterCapitalZ,
    /// \u{2129}: '℩'
    TurnedGreekSmallLetterIota,
    /// \u{212a}: 'K'
    KelvinSign,
    /// \u{212b}: 'Å'
    AngstromSign,
    /// \u{212c}: 'ℬ'
    ScriptCapitalB,
    /// \u{212d}: 'ℭ'
    BlackDashLetterCapitalC,
    /// \u{212e}: '℮'
    EstimatedSymbol,
    /// \u{212f}: 'ℯ'
    ScriptSmallE,
    /// \u{2130}: 'ℰ'
    ScriptCapitalE,
    /// \u{2131}: 'ℱ'
    ScriptCapitalF,
    /// \u{2132}: 'Ⅎ'
    TurnedCapitalF,
    /// \u{2133}: 'ℳ'
    ScriptCapitalM,
    /// \u{2134}: 'ℴ'
    ScriptSmallO,
    /// \u{2135}: 'ℵ'
    AlefSymbol,
    /// \u{2136}: 'ℶ'
    BetSymbol,
    /// \u{2137}: 'ℷ'
    GimelSymbol,
    /// \u{2138}: 'ℸ'
    DaletSymbol,
    /// \u{2139}: 'ℹ'
    InformationSource,
    /// \u{213a}: '℺'
    RotatedCapitalQ,
    /// \u{213b}: '℻'
    FacsimileSign,
    /// \u{213c}: 'ℼ'
    DoubleDashStruckSmallPi,
    /// \u{213d}: 'ℽ'
    DoubleDashStruckSmallGamma,
    /// \u{213e}: 'ℾ'
    DoubleDashStruckCapitalGamma,
    /// \u{213f}: 'ℿ'
    DoubleDashStruckCapitalPi,
    /// \u{2140}: '⅀'
    DoubleDashStruckNDashArySummation,
    /// \u{2141}: '⅁'
    TurnedSansDashSerifCapitalG,
    /// \u{2142}: '⅂'
    TurnedSansDashSerifCapitalL,
    /// \u{2143}: '⅃'
    ReversedSansDashSerifCapitalL,
    /// \u{2144}: '⅄'
    TurnedSansDashSerifCapitalY,
    /// \u{2145}: 'ⅅ'
    DoubleDashStruckItalicCapitalD,
    /// \u{2146}: 'ⅆ'
    DoubleDashStruckItalicSmallD,
    /// \u{2147}: 'ⅇ'
    DoubleDashStruckItalicSmallE,
    /// \u{2148}: 'ⅈ'
    DoubleDashStruckItalicSmallI,
    /// \u{2149}: 'ⅉ'
    DoubleDashStruckItalicSmallJ,
    /// \u{214a}: '⅊'
    PropertyLine,
    /// \u{214b}: '⅋'
    TurnedAmpersand,
    /// \u{214c}: '⅌'
    PerSign,
    /// \u{214d}: '⅍'
    Aktieselskab,
    /// \u{214e}: 'ⅎ'
    TurnedSmallF,
}

impl Into<char> for LetterlikeSymbols {
    fn into(self) -> char {
        match self {
            LetterlikeSymbols::AccountOf => '℀',
            LetterlikeSymbols::AddressedToTheSubject => '℁',
            LetterlikeSymbols::DoubleDashStruckCapitalC => 'ℂ',
            LetterlikeSymbols::DegreeCelsius => '℃',
            LetterlikeSymbols::CentreLineSymbol => '℄',
            LetterlikeSymbols::CareOf => '℅',
            LetterlikeSymbols::CadaUna => '℆',
            LetterlikeSymbols::EulerConstant => 'ℇ',
            LetterlikeSymbols::Scruple => '℈',
            LetterlikeSymbols::DegreeFahrenheit => '℉',
            LetterlikeSymbols::ScriptSmallG => 'ℊ',
            LetterlikeSymbols::ScriptCapitalH => 'ℋ',
            LetterlikeSymbols::BlackDashLetterCapitalH => 'ℌ',
            LetterlikeSymbols::DoubleDashStruckCapitalH => 'ℍ',
            LetterlikeSymbols::PlanckConstant => 'ℎ',
            LetterlikeSymbols::PlanckConstantOverTwoPi => 'ℏ',
            LetterlikeSymbols::ScriptCapitalI => 'ℐ',
            LetterlikeSymbols::BlackDashLetterCapitalI => 'ℑ',
            LetterlikeSymbols::ScriptCapitalL => 'ℒ',
            LetterlikeSymbols::ScriptSmallL => 'ℓ',
            LetterlikeSymbols::LBBarSymbol => '℔',
            LetterlikeSymbols::DoubleDashStruckCapitalN => 'ℕ',
            LetterlikeSymbols::NumeroSign => '№',
            LetterlikeSymbols::SoundRecordingCopyright => '℗',
            LetterlikeSymbols::ScriptCapitalP => '℘',
            LetterlikeSymbols::DoubleDashStruckCapitalP => 'ℙ',
            LetterlikeSymbols::DoubleDashStruckCapitalQ => 'ℚ',
            LetterlikeSymbols::ScriptCapitalR => 'ℛ',
            LetterlikeSymbols::BlackDashLetterCapitalR => 'ℜ',
            LetterlikeSymbols::DoubleDashStruckCapitalR => 'ℝ',
            LetterlikeSymbols::PrescriptionTake => '℞',
            LetterlikeSymbols::Response => '℟',
            LetterlikeSymbols::ServiceMark => '℠',
            LetterlikeSymbols::TelephoneSign => '℡',
            LetterlikeSymbols::TradeMarkSign => '™',
            LetterlikeSymbols::Versicle => '℣',
            LetterlikeSymbols::DoubleDashStruckCapitalZ => 'ℤ',
            LetterlikeSymbols::OunceSign => '℥',
            LetterlikeSymbols::OhmSign => 'Ω',
            LetterlikeSymbols::InvertedOhmSign => '℧',
            LetterlikeSymbols::BlackDashLetterCapitalZ => 'ℨ',
            LetterlikeSymbols::TurnedGreekSmallLetterIota => '℩',
            LetterlikeSymbols::KelvinSign => 'K',
            LetterlikeSymbols::AngstromSign => 'Å',
            LetterlikeSymbols::ScriptCapitalB => 'ℬ',
            LetterlikeSymbols::BlackDashLetterCapitalC => 'ℭ',
            LetterlikeSymbols::EstimatedSymbol => '℮',
            LetterlikeSymbols::ScriptSmallE => 'ℯ',
            LetterlikeSymbols::ScriptCapitalE => 'ℰ',
            LetterlikeSymbols::ScriptCapitalF => 'ℱ',
            LetterlikeSymbols::TurnedCapitalF => 'Ⅎ',
            LetterlikeSymbols::ScriptCapitalM => 'ℳ',
            LetterlikeSymbols::ScriptSmallO => 'ℴ',
            LetterlikeSymbols::AlefSymbol => 'ℵ',
            LetterlikeSymbols::BetSymbol => 'ℶ',
            LetterlikeSymbols::GimelSymbol => 'ℷ',
            LetterlikeSymbols::DaletSymbol => 'ℸ',
            LetterlikeSymbols::InformationSource => 'ℹ',
            LetterlikeSymbols::RotatedCapitalQ => '℺',
            LetterlikeSymbols::FacsimileSign => '℻',
            LetterlikeSymbols::DoubleDashStruckSmallPi => 'ℼ',
            LetterlikeSymbols::DoubleDashStruckSmallGamma => 'ℽ',
            LetterlikeSymbols::DoubleDashStruckCapitalGamma => 'ℾ',
            LetterlikeSymbols::DoubleDashStruckCapitalPi => 'ℿ',
            LetterlikeSymbols::DoubleDashStruckNDashArySummation => '⅀',
            LetterlikeSymbols::TurnedSansDashSerifCapitalG => '⅁',
            LetterlikeSymbols::TurnedSansDashSerifCapitalL => '⅂',
            LetterlikeSymbols::ReversedSansDashSerifCapitalL => '⅃',
            LetterlikeSymbols::TurnedSansDashSerifCapitalY => '⅄',
            LetterlikeSymbols::DoubleDashStruckItalicCapitalD => 'ⅅ',
            LetterlikeSymbols::DoubleDashStruckItalicSmallD => 'ⅆ',
            LetterlikeSymbols::DoubleDashStruckItalicSmallE => 'ⅇ',
            LetterlikeSymbols::DoubleDashStruckItalicSmallI => 'ⅈ',
            LetterlikeSymbols::DoubleDashStruckItalicSmallJ => 'ⅉ',
            LetterlikeSymbols::PropertyLine => '⅊',
            LetterlikeSymbols::TurnedAmpersand => '⅋',
            LetterlikeSymbols::PerSign => '⅌',
            LetterlikeSymbols::Aktieselskab => '⅍',
            LetterlikeSymbols::TurnedSmallF => 'ⅎ',
        }
    }
}

impl std::convert::TryFrom<char> for LetterlikeSymbols {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '℀' => Ok(LetterlikeSymbols::AccountOf),
            '℁' => Ok(LetterlikeSymbols::AddressedToTheSubject),
            'ℂ' => Ok(LetterlikeSymbols::DoubleDashStruckCapitalC),
            '℃' => Ok(LetterlikeSymbols::DegreeCelsius),
            '℄' => Ok(LetterlikeSymbols::CentreLineSymbol),
            '℅' => Ok(LetterlikeSymbols::CareOf),
            '℆' => Ok(LetterlikeSymbols::CadaUna),
            'ℇ' => Ok(LetterlikeSymbols::EulerConstant),
            '℈' => Ok(LetterlikeSymbols::Scruple),
            '℉' => Ok(LetterlikeSymbols::DegreeFahrenheit),
            'ℊ' => Ok(LetterlikeSymbols::ScriptSmallG),
            'ℋ' => Ok(LetterlikeSymbols::ScriptCapitalH),
            'ℌ' => Ok(LetterlikeSymbols::BlackDashLetterCapitalH),
            'ℍ' => Ok(LetterlikeSymbols::DoubleDashStruckCapitalH),
            'ℎ' => Ok(LetterlikeSymbols::PlanckConstant),
            'ℏ' => Ok(LetterlikeSymbols::PlanckConstantOverTwoPi),
            'ℐ' => Ok(LetterlikeSymbols::ScriptCapitalI),
            'ℑ' => Ok(LetterlikeSymbols::BlackDashLetterCapitalI),
            'ℒ' => Ok(LetterlikeSymbols::ScriptCapitalL),
            'ℓ' => Ok(LetterlikeSymbols::ScriptSmallL),
            '℔' => Ok(LetterlikeSymbols::LBBarSymbol),
            'ℕ' => Ok(LetterlikeSymbols::DoubleDashStruckCapitalN),
            '№' => Ok(LetterlikeSymbols::NumeroSign),
            '℗' => Ok(LetterlikeSymbols::SoundRecordingCopyright),
            '℘' => Ok(LetterlikeSymbols::ScriptCapitalP),
            'ℙ' => Ok(LetterlikeSymbols::DoubleDashStruckCapitalP),
            'ℚ' => Ok(LetterlikeSymbols::DoubleDashStruckCapitalQ),
            'ℛ' => Ok(LetterlikeSymbols::ScriptCapitalR),
            'ℜ' => Ok(LetterlikeSymbols::BlackDashLetterCapitalR),
            'ℝ' => Ok(LetterlikeSymbols::DoubleDashStruckCapitalR),
            '℞' => Ok(LetterlikeSymbols::PrescriptionTake),
            '℟' => Ok(LetterlikeSymbols::Response),
            '℠' => Ok(LetterlikeSymbols::ServiceMark),
            '℡' => Ok(LetterlikeSymbols::TelephoneSign),
            '™' => Ok(LetterlikeSymbols::TradeMarkSign),
            '℣' => Ok(LetterlikeSymbols::Versicle),
            'ℤ' => Ok(LetterlikeSymbols::DoubleDashStruckCapitalZ),
            '℥' => Ok(LetterlikeSymbols::OunceSign),
            'Ω' => Ok(LetterlikeSymbols::OhmSign),
            '℧' => Ok(LetterlikeSymbols::InvertedOhmSign),
            'ℨ' => Ok(LetterlikeSymbols::BlackDashLetterCapitalZ),
            '℩' => Ok(LetterlikeSymbols::TurnedGreekSmallLetterIota),
            'K' => Ok(LetterlikeSymbols::KelvinSign),
            'Å' => Ok(LetterlikeSymbols::AngstromSign),
            'ℬ' => Ok(LetterlikeSymbols::ScriptCapitalB),
            'ℭ' => Ok(LetterlikeSymbols::BlackDashLetterCapitalC),
            '℮' => Ok(LetterlikeSymbols::EstimatedSymbol),
            'ℯ' => Ok(LetterlikeSymbols::ScriptSmallE),
            'ℰ' => Ok(LetterlikeSymbols::ScriptCapitalE),
            'ℱ' => Ok(LetterlikeSymbols::ScriptCapitalF),
            'Ⅎ' => Ok(LetterlikeSymbols::TurnedCapitalF),
            'ℳ' => Ok(LetterlikeSymbols::ScriptCapitalM),
            'ℴ' => Ok(LetterlikeSymbols::ScriptSmallO),
            'ℵ' => Ok(LetterlikeSymbols::AlefSymbol),
            'ℶ' => Ok(LetterlikeSymbols::BetSymbol),
            'ℷ' => Ok(LetterlikeSymbols::GimelSymbol),
            'ℸ' => Ok(LetterlikeSymbols::DaletSymbol),
            'ℹ' => Ok(LetterlikeSymbols::InformationSource),
            '℺' => Ok(LetterlikeSymbols::RotatedCapitalQ),
            '℻' => Ok(LetterlikeSymbols::FacsimileSign),
            'ℼ' => Ok(LetterlikeSymbols::DoubleDashStruckSmallPi),
            'ℽ' => Ok(LetterlikeSymbols::DoubleDashStruckSmallGamma),
            'ℾ' => Ok(LetterlikeSymbols::DoubleDashStruckCapitalGamma),
            'ℿ' => Ok(LetterlikeSymbols::DoubleDashStruckCapitalPi),
            '⅀' => Ok(LetterlikeSymbols::DoubleDashStruckNDashArySummation),
            '⅁' => Ok(LetterlikeSymbols::TurnedSansDashSerifCapitalG),
            '⅂' => Ok(LetterlikeSymbols::TurnedSansDashSerifCapitalL),
            '⅃' => Ok(LetterlikeSymbols::ReversedSansDashSerifCapitalL),
            '⅄' => Ok(LetterlikeSymbols::TurnedSansDashSerifCapitalY),
            'ⅅ' => Ok(LetterlikeSymbols::DoubleDashStruckItalicCapitalD),
            'ⅆ' => Ok(LetterlikeSymbols::DoubleDashStruckItalicSmallD),
            'ⅇ' => Ok(LetterlikeSymbols::DoubleDashStruckItalicSmallE),
            'ⅈ' => Ok(LetterlikeSymbols::DoubleDashStruckItalicSmallI),
            'ⅉ' => Ok(LetterlikeSymbols::DoubleDashStruckItalicSmallJ),
            '⅊' => Ok(LetterlikeSymbols::PropertyLine),
            '⅋' => Ok(LetterlikeSymbols::TurnedAmpersand),
            '⅌' => Ok(LetterlikeSymbols::PerSign),
            '⅍' => Ok(LetterlikeSymbols::Aktieselskab),
            'ⅎ' => Ok(LetterlikeSymbols::TurnedSmallF),
            _ => Err(()),
        }
    }
}

impl Into<u32> for LetterlikeSymbols {
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

impl std::convert::TryFrom<u32> for LetterlikeSymbols {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for LetterlikeSymbols {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl LetterlikeSymbols {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        LetterlikeSymbols::AccountOf
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("LetterlikeSymbols{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
