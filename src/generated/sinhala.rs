
/// An enum to represent all characters in the Sinhala block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Sinhala {
    /// \u{d82}: 'ං'
    SignAnusvaraya,
    /// \u{d83}: 'ඃ'
    SignVisargaya,
    /// \u{d85}: 'අ'
    LetterAyanna,
    /// \u{d86}: 'ආ'
    LetterAayanna,
    /// \u{d87}: 'ඇ'
    LetterAeyanna,
    /// \u{d88}: 'ඈ'
    LetterAeeyanna,
    /// \u{d89}: 'ඉ'
    LetterIyanna,
    /// \u{d8a}: 'ඊ'
    LetterIiyanna,
    /// \u{d8b}: 'උ'
    LetterUyanna,
    /// \u{d8c}: 'ඌ'
    LetterUuyanna,
    /// \u{d8d}: 'ඍ'
    LetterIruyanna,
    /// \u{d8e}: 'ඎ'
    LetterIruuyanna,
    /// \u{d8f}: 'ඏ'
    LetterIluyanna,
    /// \u{d90}: 'ඐ'
    LetterIluuyanna,
    /// \u{d91}: 'එ'
    LetterEyanna,
    /// \u{d92}: 'ඒ'
    LetterEeyanna,
    /// \u{d93}: 'ඓ'
    LetterAiyanna,
    /// \u{d94}: 'ඔ'
    LetterOyanna,
    /// \u{d95}: 'ඕ'
    LetterOoyanna,
    /// \u{d96}: 'ඖ'
    LetterAuyanna,
    /// \u{d9a}: 'ක'
    LetterAlpapraanaKayanna,
    /// \u{d9b}: 'ඛ'
    LetterMahaapraanaKayanna,
    /// \u{d9c}: 'ග'
    LetterAlpapraanaGayanna,
    /// \u{d9d}: 'ඝ'
    LetterMahaapraanaGayanna,
    /// \u{d9e}: 'ඞ'
    LetterKantajaNaasikyaya,
    /// \u{d9f}: 'ඟ'
    LetterSanyakaGayanna,
    /// \u{da0}: 'ච'
    LetterAlpapraanaCayanna,
    /// \u{da1}: 'ඡ'
    LetterMahaapraanaCayanna,
    /// \u{da2}: 'ජ'
    LetterAlpapraanaJayanna,
    /// \u{da3}: 'ඣ'
    LetterMahaapraanaJayanna,
    /// \u{da4}: 'ඤ'
    LetterTaalujaNaasikyaya,
    /// \u{da5}: 'ඥ'
    LetterTaalujaSanyoogaNaaksikyaya,
    /// \u{da6}: 'ඦ'
    LetterSanyakaJayanna,
    /// \u{da7}: 'ට'
    LetterAlpapraanaTtayanna,
    /// \u{da8}: 'ඨ'
    LetterMahaapraanaTtayanna,
    /// \u{da9}: 'ඩ'
    LetterAlpapraanaDdayanna,
    /// \u{daa}: 'ඪ'
    LetterMahaapraanaDdayanna,
    /// \u{dab}: 'ණ'
    LetterMuurdhajaNayanna,
    /// \u{dac}: 'ඬ'
    LetterSanyakaDdayanna,
    /// \u{dad}: 'ත'
    LetterAlpapraanaTayanna,
    /// \u{dae}: 'ථ'
    LetterMahaapraanaTayanna,
    /// \u{daf}: 'ද'
    LetterAlpapraanaDayanna,
    /// \u{db0}: 'ධ'
    LetterMahaapraanaDayanna,
    /// \u{db1}: 'න'
    LetterDantajaNayanna,
    /// \u{db3}: 'ඳ'
    LetterSanyakaDayanna,
    /// \u{db4}: 'ප'
    LetterAlpapraanaPayanna,
    /// \u{db5}: 'ඵ'
    LetterMahaapraanaPayanna,
    /// \u{db6}: 'බ'
    LetterAlpapraanaBayanna,
    /// \u{db7}: 'භ'
    LetterMahaapraanaBayanna,
    /// \u{db8}: 'ම'
    LetterMayanna,
    /// \u{db9}: 'ඹ'
    LetterAmbaBayanna,
    /// \u{dba}: 'ය'
    LetterYayanna,
    /// \u{dbb}: 'ර'
    LetterRayanna,
    /// \u{dbd}: 'ල'
    LetterDantajaLayanna,
    /// \u{dc0}: 'ව'
    LetterVayanna,
    /// \u{dc1}: 'ශ'
    LetterTaalujaSayanna,
    /// \u{dc2}: 'ෂ'
    LetterMuurdhajaSayanna,
    /// \u{dc3}: 'ස'
    LetterDantajaSayanna,
    /// \u{dc4}: 'හ'
    LetterHayanna,
    /// \u{dc5}: 'ළ'
    LetterMuurdhajaLayanna,
    /// \u{dc6}: 'ෆ'
    LetterFayanna,
    /// \u{dca}: '්'
    SignAlDashLakuna,
    /// \u{dcf}: 'ා'
    VowelSignAelaDashPilla,
    /// \u{dd0}: 'ැ'
    VowelSignKettiAedaDashPilla,
    /// \u{dd1}: 'ෑ'
    VowelSignDigaAedaDashPilla,
    /// \u{dd2}: 'ි'
    VowelSignKettiIsDashPilla,
    /// \u{dd3}: 'ී'
    VowelSignDigaIsDashPilla,
    /// \u{dd4}: 'ු'
    VowelSignKettiPaaDashPilla,
    /// \u{dd6}: 'ූ'
    VowelSignDigaPaaDashPilla,
    /// \u{dd8}: 'ෘ'
    VowelSignGaettaDashPilla,
    /// \u{dd9}: 'ෙ'
    VowelSignKombuva,
    /// \u{dda}: 'ේ'
    VowelSignDigaKombuva,
    /// \u{ddb}: 'ෛ'
    VowelSignKombuDeka,
    /// \u{ddc}: 'ො'
    VowelSignKombuvaHaaAelaDashPilla,
    /// \u{ddd}: 'ෝ'
    VowelSignKombuvaHaaDigaAelaDashPilla,
    /// \u{dde}: 'ෞ'
    VowelSignKombuvaHaaGayanukitta,
    /// \u{ddf}: 'ෟ'
    VowelSignGayanukitta,
    /// \u{de6}: '෦'
    LithDigitZero,
    /// \u{de7}: '෧'
    LithDigitOne,
    /// \u{de8}: '෨'
    LithDigitTwo,
    /// \u{de9}: '෩'
    LithDigitThree,
    /// \u{dea}: '෪'
    LithDigitFour,
    /// \u{deb}: '෫'
    LithDigitFive,
    /// \u{dec}: '෬'
    LithDigitSix,
    /// \u{ded}: '෭'
    LithDigitSeven,
    /// \u{dee}: '෮'
    LithDigitEight,
    /// \u{def}: '෯'
    LithDigitNine,
    /// \u{df2}: 'ෲ'
    VowelSignDigaGaettaDashPilla,
    /// \u{df3}: 'ෳ'
    VowelSignDigaGayanukitta,
    /// \u{df4}: '෴'
    PunctuationKunddaliya,
}

impl Into<char> for Sinhala {
    fn into(self) -> char {
        match self {
            Sinhala::SignAnusvaraya => 'ං',
            Sinhala::SignVisargaya => 'ඃ',
            Sinhala::LetterAyanna => 'අ',
            Sinhala::LetterAayanna => 'ආ',
            Sinhala::LetterAeyanna => 'ඇ',
            Sinhala::LetterAeeyanna => 'ඈ',
            Sinhala::LetterIyanna => 'ඉ',
            Sinhala::LetterIiyanna => 'ඊ',
            Sinhala::LetterUyanna => 'උ',
            Sinhala::LetterUuyanna => 'ඌ',
            Sinhala::LetterIruyanna => 'ඍ',
            Sinhala::LetterIruuyanna => 'ඎ',
            Sinhala::LetterIluyanna => 'ඏ',
            Sinhala::LetterIluuyanna => 'ඐ',
            Sinhala::LetterEyanna => 'එ',
            Sinhala::LetterEeyanna => 'ඒ',
            Sinhala::LetterAiyanna => 'ඓ',
            Sinhala::LetterOyanna => 'ඔ',
            Sinhala::LetterOoyanna => 'ඕ',
            Sinhala::LetterAuyanna => 'ඖ',
            Sinhala::LetterAlpapraanaKayanna => 'ක',
            Sinhala::LetterMahaapraanaKayanna => 'ඛ',
            Sinhala::LetterAlpapraanaGayanna => 'ග',
            Sinhala::LetterMahaapraanaGayanna => 'ඝ',
            Sinhala::LetterKantajaNaasikyaya => 'ඞ',
            Sinhala::LetterSanyakaGayanna => 'ඟ',
            Sinhala::LetterAlpapraanaCayanna => 'ච',
            Sinhala::LetterMahaapraanaCayanna => 'ඡ',
            Sinhala::LetterAlpapraanaJayanna => 'ජ',
            Sinhala::LetterMahaapraanaJayanna => 'ඣ',
            Sinhala::LetterTaalujaNaasikyaya => 'ඤ',
            Sinhala::LetterTaalujaSanyoogaNaaksikyaya => 'ඥ',
            Sinhala::LetterSanyakaJayanna => 'ඦ',
            Sinhala::LetterAlpapraanaTtayanna => 'ට',
            Sinhala::LetterMahaapraanaTtayanna => 'ඨ',
            Sinhala::LetterAlpapraanaDdayanna => 'ඩ',
            Sinhala::LetterMahaapraanaDdayanna => 'ඪ',
            Sinhala::LetterMuurdhajaNayanna => 'ණ',
            Sinhala::LetterSanyakaDdayanna => 'ඬ',
            Sinhala::LetterAlpapraanaTayanna => 'ත',
            Sinhala::LetterMahaapraanaTayanna => 'ථ',
            Sinhala::LetterAlpapraanaDayanna => 'ද',
            Sinhala::LetterMahaapraanaDayanna => 'ධ',
            Sinhala::LetterDantajaNayanna => 'න',
            Sinhala::LetterSanyakaDayanna => 'ඳ',
            Sinhala::LetterAlpapraanaPayanna => 'ප',
            Sinhala::LetterMahaapraanaPayanna => 'ඵ',
            Sinhala::LetterAlpapraanaBayanna => 'බ',
            Sinhala::LetterMahaapraanaBayanna => 'භ',
            Sinhala::LetterMayanna => 'ම',
            Sinhala::LetterAmbaBayanna => 'ඹ',
            Sinhala::LetterYayanna => 'ය',
            Sinhala::LetterRayanna => 'ර',
            Sinhala::LetterDantajaLayanna => 'ල',
            Sinhala::LetterVayanna => 'ව',
            Sinhala::LetterTaalujaSayanna => 'ශ',
            Sinhala::LetterMuurdhajaSayanna => 'ෂ',
            Sinhala::LetterDantajaSayanna => 'ස',
            Sinhala::LetterHayanna => 'හ',
            Sinhala::LetterMuurdhajaLayanna => 'ළ',
            Sinhala::LetterFayanna => 'ෆ',
            Sinhala::SignAlDashLakuna => '්',
            Sinhala::VowelSignAelaDashPilla => 'ා',
            Sinhala::VowelSignKettiAedaDashPilla => 'ැ',
            Sinhala::VowelSignDigaAedaDashPilla => 'ෑ',
            Sinhala::VowelSignKettiIsDashPilla => 'ි',
            Sinhala::VowelSignDigaIsDashPilla => 'ී',
            Sinhala::VowelSignKettiPaaDashPilla => 'ු',
            Sinhala::VowelSignDigaPaaDashPilla => 'ූ',
            Sinhala::VowelSignGaettaDashPilla => 'ෘ',
            Sinhala::VowelSignKombuva => 'ෙ',
            Sinhala::VowelSignDigaKombuva => 'ේ',
            Sinhala::VowelSignKombuDeka => 'ෛ',
            Sinhala::VowelSignKombuvaHaaAelaDashPilla => 'ො',
            Sinhala::VowelSignKombuvaHaaDigaAelaDashPilla => 'ෝ',
            Sinhala::VowelSignKombuvaHaaGayanukitta => 'ෞ',
            Sinhala::VowelSignGayanukitta => 'ෟ',
            Sinhala::LithDigitZero => '෦',
            Sinhala::LithDigitOne => '෧',
            Sinhala::LithDigitTwo => '෨',
            Sinhala::LithDigitThree => '෩',
            Sinhala::LithDigitFour => '෪',
            Sinhala::LithDigitFive => '෫',
            Sinhala::LithDigitSix => '෬',
            Sinhala::LithDigitSeven => '෭',
            Sinhala::LithDigitEight => '෮',
            Sinhala::LithDigitNine => '෯',
            Sinhala::VowelSignDigaGaettaDashPilla => 'ෲ',
            Sinhala::VowelSignDigaGayanukitta => 'ෳ',
            Sinhala::PunctuationKunddaliya => '෴',
        }
    }
}

impl std::convert::TryFrom<char> for Sinhala {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ං' => Ok(Sinhala::SignAnusvaraya),
            'ඃ' => Ok(Sinhala::SignVisargaya),
            'අ' => Ok(Sinhala::LetterAyanna),
            'ආ' => Ok(Sinhala::LetterAayanna),
            'ඇ' => Ok(Sinhala::LetterAeyanna),
            'ඈ' => Ok(Sinhala::LetterAeeyanna),
            'ඉ' => Ok(Sinhala::LetterIyanna),
            'ඊ' => Ok(Sinhala::LetterIiyanna),
            'උ' => Ok(Sinhala::LetterUyanna),
            'ඌ' => Ok(Sinhala::LetterUuyanna),
            'ඍ' => Ok(Sinhala::LetterIruyanna),
            'ඎ' => Ok(Sinhala::LetterIruuyanna),
            'ඏ' => Ok(Sinhala::LetterIluyanna),
            'ඐ' => Ok(Sinhala::LetterIluuyanna),
            'එ' => Ok(Sinhala::LetterEyanna),
            'ඒ' => Ok(Sinhala::LetterEeyanna),
            'ඓ' => Ok(Sinhala::LetterAiyanna),
            'ඔ' => Ok(Sinhala::LetterOyanna),
            'ඕ' => Ok(Sinhala::LetterOoyanna),
            'ඖ' => Ok(Sinhala::LetterAuyanna),
            'ක' => Ok(Sinhala::LetterAlpapraanaKayanna),
            'ඛ' => Ok(Sinhala::LetterMahaapraanaKayanna),
            'ග' => Ok(Sinhala::LetterAlpapraanaGayanna),
            'ඝ' => Ok(Sinhala::LetterMahaapraanaGayanna),
            'ඞ' => Ok(Sinhala::LetterKantajaNaasikyaya),
            'ඟ' => Ok(Sinhala::LetterSanyakaGayanna),
            'ච' => Ok(Sinhala::LetterAlpapraanaCayanna),
            'ඡ' => Ok(Sinhala::LetterMahaapraanaCayanna),
            'ජ' => Ok(Sinhala::LetterAlpapraanaJayanna),
            'ඣ' => Ok(Sinhala::LetterMahaapraanaJayanna),
            'ඤ' => Ok(Sinhala::LetterTaalujaNaasikyaya),
            'ඥ' => Ok(Sinhala::LetterTaalujaSanyoogaNaaksikyaya),
            'ඦ' => Ok(Sinhala::LetterSanyakaJayanna),
            'ට' => Ok(Sinhala::LetterAlpapraanaTtayanna),
            'ඨ' => Ok(Sinhala::LetterMahaapraanaTtayanna),
            'ඩ' => Ok(Sinhala::LetterAlpapraanaDdayanna),
            'ඪ' => Ok(Sinhala::LetterMahaapraanaDdayanna),
            'ණ' => Ok(Sinhala::LetterMuurdhajaNayanna),
            'ඬ' => Ok(Sinhala::LetterSanyakaDdayanna),
            'ත' => Ok(Sinhala::LetterAlpapraanaTayanna),
            'ථ' => Ok(Sinhala::LetterMahaapraanaTayanna),
            'ද' => Ok(Sinhala::LetterAlpapraanaDayanna),
            'ධ' => Ok(Sinhala::LetterMahaapraanaDayanna),
            'න' => Ok(Sinhala::LetterDantajaNayanna),
            'ඳ' => Ok(Sinhala::LetterSanyakaDayanna),
            'ප' => Ok(Sinhala::LetterAlpapraanaPayanna),
            'ඵ' => Ok(Sinhala::LetterMahaapraanaPayanna),
            'බ' => Ok(Sinhala::LetterAlpapraanaBayanna),
            'භ' => Ok(Sinhala::LetterMahaapraanaBayanna),
            'ම' => Ok(Sinhala::LetterMayanna),
            'ඹ' => Ok(Sinhala::LetterAmbaBayanna),
            'ය' => Ok(Sinhala::LetterYayanna),
            'ර' => Ok(Sinhala::LetterRayanna),
            'ල' => Ok(Sinhala::LetterDantajaLayanna),
            'ව' => Ok(Sinhala::LetterVayanna),
            'ශ' => Ok(Sinhala::LetterTaalujaSayanna),
            'ෂ' => Ok(Sinhala::LetterMuurdhajaSayanna),
            'ස' => Ok(Sinhala::LetterDantajaSayanna),
            'හ' => Ok(Sinhala::LetterHayanna),
            'ළ' => Ok(Sinhala::LetterMuurdhajaLayanna),
            'ෆ' => Ok(Sinhala::LetterFayanna),
            '්' => Ok(Sinhala::SignAlDashLakuna),
            'ා' => Ok(Sinhala::VowelSignAelaDashPilla),
            'ැ' => Ok(Sinhala::VowelSignKettiAedaDashPilla),
            'ෑ' => Ok(Sinhala::VowelSignDigaAedaDashPilla),
            'ි' => Ok(Sinhala::VowelSignKettiIsDashPilla),
            'ී' => Ok(Sinhala::VowelSignDigaIsDashPilla),
            'ු' => Ok(Sinhala::VowelSignKettiPaaDashPilla),
            'ූ' => Ok(Sinhala::VowelSignDigaPaaDashPilla),
            'ෘ' => Ok(Sinhala::VowelSignGaettaDashPilla),
            'ෙ' => Ok(Sinhala::VowelSignKombuva),
            'ේ' => Ok(Sinhala::VowelSignDigaKombuva),
            'ෛ' => Ok(Sinhala::VowelSignKombuDeka),
            'ො' => Ok(Sinhala::VowelSignKombuvaHaaAelaDashPilla),
            'ෝ' => Ok(Sinhala::VowelSignKombuvaHaaDigaAelaDashPilla),
            'ෞ' => Ok(Sinhala::VowelSignKombuvaHaaGayanukitta),
            'ෟ' => Ok(Sinhala::VowelSignGayanukitta),
            '෦' => Ok(Sinhala::LithDigitZero),
            '෧' => Ok(Sinhala::LithDigitOne),
            '෨' => Ok(Sinhala::LithDigitTwo),
            '෩' => Ok(Sinhala::LithDigitThree),
            '෪' => Ok(Sinhala::LithDigitFour),
            '෫' => Ok(Sinhala::LithDigitFive),
            '෬' => Ok(Sinhala::LithDigitSix),
            '෭' => Ok(Sinhala::LithDigitSeven),
            '෮' => Ok(Sinhala::LithDigitEight),
            '෯' => Ok(Sinhala::LithDigitNine),
            'ෲ' => Ok(Sinhala::VowelSignDigaGaettaDashPilla),
            'ෳ' => Ok(Sinhala::VowelSignDigaGayanukitta),
            '෴' => Ok(Sinhala::PunctuationKunddaliya),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Sinhala {
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

impl std::convert::TryFrom<u32> for Sinhala {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Sinhala {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Sinhala {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Sinhala::SignAnusvaraya
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Sinhala{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
