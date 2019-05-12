/// \u{d80} → \u{dff}
///
/// ං ඃ අ ආ ඇ ඈ ඉ ඊ උ ඌ ඍ ඎ ඏ ඐ එ ඒ\
/// ඓ ඔ ඕ ඖ ක ඛ ග ඝ ඞ ඟ ච ඡ ජ ඣ ඤ ඥ\
/// ඦ ට ඨ ඩ ඪ ණ ඬ ත ථ ද ධ න ඳ ප ඵ බ\
/// භ ම ඹ ය ර ල ව ශ ෂ ස හ ළ ෆ ් ා ැ\
/// ෑ ි ී ු ූ ෘ ෙ ේ ෛ ො ෝ ෞ ෟ ෦ ෧ ෨\
/// ෩ ෪ ෫ ෬ ෭ ෮ ෯ ෲ ෳ ෴\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{d82}: 'ං'
    pub const SIGN_ANUSVARAYA: char = 'ං';
    /// \u{d83}: 'ඃ'
    pub const SIGN_VISARGAYA: char = 'ඃ';
    /// \u{d85}: 'අ'
    pub const LETTER_AYANNA: char = 'අ';
    /// \u{d86}: 'ආ'
    pub const LETTER_AAYANNA: char = 'ආ';
    /// \u{d87}: 'ඇ'
    pub const LETTER_AEYANNA: char = 'ඇ';
    /// \u{d88}: 'ඈ'
    pub const LETTER_AEEYANNA: char = 'ඈ';
    /// \u{d89}: 'ඉ'
    pub const LETTER_IYANNA: char = 'ඉ';
    /// \u{d8a}: 'ඊ'
    pub const LETTER_IIYANNA: char = 'ඊ';
    /// \u{d8b}: 'උ'
    pub const LETTER_UYANNA: char = 'උ';
    /// \u{d8c}: 'ඌ'
    pub const LETTER_UUYANNA: char = 'ඌ';
    /// \u{d8d}: 'ඍ'
    pub const LETTER_IRUYANNA: char = 'ඍ';
    /// \u{d8e}: 'ඎ'
    pub const LETTER_IRUUYANNA: char = 'ඎ';
    /// \u{d8f}: 'ඏ'
    pub const LETTER_ILUYANNA: char = 'ඏ';
    /// \u{d90}: 'ඐ'
    pub const LETTER_ILUUYANNA: char = 'ඐ';
    /// \u{d91}: 'එ'
    pub const LETTER_EYANNA: char = 'එ';
    /// \u{d92}: 'ඒ'
    pub const LETTER_EEYANNA: char = 'ඒ';
    /// \u{d93}: 'ඓ'
    pub const LETTER_AIYANNA: char = 'ඓ';
    /// \u{d94}: 'ඔ'
    pub const LETTER_OYANNA: char = 'ඔ';
    /// \u{d95}: 'ඕ'
    pub const LETTER_OOYANNA: char = 'ඕ';
    /// \u{d96}: 'ඖ'
    pub const LETTER_AUYANNA: char = 'ඖ';
    /// \u{d9a}: 'ක'
    pub const LETTER_ALPAPRAANA_KAYANNA: char = 'ක';
    /// \u{d9b}: 'ඛ'
    pub const LETTER_MAHAAPRAANA_KAYANNA: char = 'ඛ';
    /// \u{d9c}: 'ග'
    pub const LETTER_ALPAPRAANA_GAYANNA: char = 'ග';
    /// \u{d9d}: 'ඝ'
    pub const LETTER_MAHAAPRAANA_GAYANNA: char = 'ඝ';
    /// \u{d9e}: 'ඞ'
    pub const LETTER_KANTAJA_NAASIKYAYA: char = 'ඞ';
    /// \u{d9f}: 'ඟ'
    pub const LETTER_SANYAKA_GAYANNA: char = 'ඟ';
    /// \u{da0}: 'ච'
    pub const LETTER_ALPAPRAANA_CAYANNA: char = 'ච';
    /// \u{da1}: 'ඡ'
    pub const LETTER_MAHAAPRAANA_CAYANNA: char = 'ඡ';
    /// \u{da2}: 'ජ'
    pub const LETTER_ALPAPRAANA_JAYANNA: char = 'ජ';
    /// \u{da3}: 'ඣ'
    pub const LETTER_MAHAAPRAANA_JAYANNA: char = 'ඣ';
    /// \u{da4}: 'ඤ'
    pub const LETTER_TAALUJA_NAASIKYAYA: char = 'ඤ';
    /// \u{da5}: 'ඥ'
    pub const LETTER_TAALUJA_SANYOOGA_NAAKSIKYAYA: char = 'ඥ';
    /// \u{da6}: 'ඦ'
    pub const LETTER_SANYAKA_JAYANNA: char = 'ඦ';
    /// \u{da7}: 'ට'
    pub const LETTER_ALPAPRAANA_TTAYANNA: char = 'ට';
    /// \u{da8}: 'ඨ'
    pub const LETTER_MAHAAPRAANA_TTAYANNA: char = 'ඨ';
    /// \u{da9}: 'ඩ'
    pub const LETTER_ALPAPRAANA_DDAYANNA: char = 'ඩ';
    /// \u{daa}: 'ඪ'
    pub const LETTER_MAHAAPRAANA_DDAYANNA: char = 'ඪ';
    /// \u{dab}: 'ණ'
    pub const LETTER_MUURDHAJA_NAYANNA: char = 'ණ';
    /// \u{dac}: 'ඬ'
    pub const LETTER_SANYAKA_DDAYANNA: char = 'ඬ';
    /// \u{dad}: 'ත'
    pub const LETTER_ALPAPRAANA_TAYANNA: char = 'ත';
    /// \u{dae}: 'ථ'
    pub const LETTER_MAHAAPRAANA_TAYANNA: char = 'ථ';
    /// \u{daf}: 'ද'
    pub const LETTER_ALPAPRAANA_DAYANNA: char = 'ද';
    /// \u{db0}: 'ධ'
    pub const LETTER_MAHAAPRAANA_DAYANNA: char = 'ධ';
    /// \u{db1}: 'න'
    pub const LETTER_DANTAJA_NAYANNA: char = 'න';
    /// \u{db3}: 'ඳ'
    pub const LETTER_SANYAKA_DAYANNA: char = 'ඳ';
    /// \u{db4}: 'ප'
    pub const LETTER_ALPAPRAANA_PAYANNA: char = 'ප';
    /// \u{db5}: 'ඵ'
    pub const LETTER_MAHAAPRAANA_PAYANNA: char = 'ඵ';
    /// \u{db6}: 'බ'
    pub const LETTER_ALPAPRAANA_BAYANNA: char = 'බ';
    /// \u{db7}: 'භ'
    pub const LETTER_MAHAAPRAANA_BAYANNA: char = 'භ';
    /// \u{db8}: 'ම'
    pub const LETTER_MAYANNA: char = 'ම';
    /// \u{db9}: 'ඹ'
    pub const LETTER_AMBA_BAYANNA: char = 'ඹ';
    /// \u{dba}: 'ය'
    pub const LETTER_YAYANNA: char = 'ය';
    /// \u{dbb}: 'ර'
    pub const LETTER_RAYANNA: char = 'ර';
    /// \u{dbd}: 'ල'
    pub const LETTER_DANTAJA_LAYANNA: char = 'ල';
    /// \u{dc0}: 'ව'
    pub const LETTER_VAYANNA: char = 'ව';
    /// \u{dc1}: 'ශ'
    pub const LETTER_TAALUJA_SAYANNA: char = 'ශ';
    /// \u{dc2}: 'ෂ'
    pub const LETTER_MUURDHAJA_SAYANNA: char = 'ෂ';
    /// \u{dc3}: 'ස'
    pub const LETTER_DANTAJA_SAYANNA: char = 'ස';
    /// \u{dc4}: 'හ'
    pub const LETTER_HAYANNA: char = 'හ';
    /// \u{dc5}: 'ළ'
    pub const LETTER_MUURDHAJA_LAYANNA: char = 'ළ';
    /// \u{dc6}: 'ෆ'
    pub const LETTER_FAYANNA: char = 'ෆ';
    /// \u{dca}: '්'
    pub const SIGN_AL_DASH_LAKUNA: char = '්';
    /// \u{dcf}: 'ා'
    pub const VOWEL_SIGN_AELA_DASH_PILLA: char = 'ා';
    /// \u{dd0}: 'ැ'
    pub const VOWEL_SIGN_KETTI_AEDA_DASH_PILLA: char = 'ැ';
    /// \u{dd1}: 'ෑ'
    pub const VOWEL_SIGN_DIGA_AEDA_DASH_PILLA: char = 'ෑ';
    /// \u{dd2}: 'ි'
    pub const VOWEL_SIGN_KETTI_IS_DASH_PILLA: char = 'ි';
    /// \u{dd3}: 'ී'
    pub const VOWEL_SIGN_DIGA_IS_DASH_PILLA: char = 'ී';
    /// \u{dd4}: 'ු'
    pub const VOWEL_SIGN_KETTI_PAA_DASH_PILLA: char = 'ු';
    /// \u{dd6}: 'ූ'
    pub const VOWEL_SIGN_DIGA_PAA_DASH_PILLA: char = 'ූ';
    /// \u{dd8}: 'ෘ'
    pub const VOWEL_SIGN_GAETTA_DASH_PILLA: char = 'ෘ';
    /// \u{dd9}: 'ෙ'
    pub const VOWEL_SIGN_KOMBUVA: char = 'ෙ';
    /// \u{dda}: 'ේ'
    pub const VOWEL_SIGN_DIGA_KOMBUVA: char = 'ේ';
    /// \u{ddb}: 'ෛ'
    pub const VOWEL_SIGN_KOMBU_DEKA: char = 'ෛ';
    /// \u{ddc}: 'ො'
    pub const VOWEL_SIGN_KOMBUVA_HAA_AELA_DASH_PILLA: char = 'ො';
    /// \u{ddd}: 'ෝ'
    pub const VOWEL_SIGN_KOMBUVA_HAA_DIGA_AELA_DASH_PILLA: char = 'ෝ';
    /// \u{dde}: 'ෞ'
    pub const VOWEL_SIGN_KOMBUVA_HAA_GAYANUKITTA: char = 'ෞ';
    /// \u{ddf}: 'ෟ'
    pub const VOWEL_SIGN_GAYANUKITTA: char = 'ෟ';
    /// \u{de6}: '෦'
    pub const LITH_DIGIT_ZERO: char = '෦';
    /// \u{de7}: '෧'
    pub const LITH_DIGIT_ONE: char = '෧';
    /// \u{de8}: '෨'
    pub const LITH_DIGIT_TWO: char = '෨';
    /// \u{de9}: '෩'
    pub const LITH_DIGIT_THREE: char = '෩';
    /// \u{dea}: '෪'
    pub const LITH_DIGIT_FOUR: char = '෪';
    /// \u{deb}: '෫'
    pub const LITH_DIGIT_FIVE: char = '෫';
    /// \u{dec}: '෬'
    pub const LITH_DIGIT_SIX: char = '෬';
    /// \u{ded}: '෭'
    pub const LITH_DIGIT_SEVEN: char = '෭';
    /// \u{dee}: '෮'
    pub const LITH_DIGIT_EIGHT: char = '෮';
    /// \u{def}: '෯'
    pub const LITH_DIGIT_NINE: char = '෯';
    /// \u{df2}: 'ෲ'
    pub const VOWEL_SIGN_DIGA_GAETTA_DASH_PILLA: char = 'ෲ';
    /// \u{df3}: 'ෳ'
    pub const VOWEL_SIGN_DIGA_GAYANUKITTA: char = 'ෳ';
    /// \u{df4}: '෴'
    pub const PUNCTUATION_KUNDDALIYA: char = '෴';
}

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
        use constants::*;
        match self {
            Sinhala::SignAnusvaraya => SIGN_ANUSVARAYA,
            Sinhala::SignVisargaya => SIGN_VISARGAYA,
            Sinhala::LetterAyanna => LETTER_AYANNA,
            Sinhala::LetterAayanna => LETTER_AAYANNA,
            Sinhala::LetterAeyanna => LETTER_AEYANNA,
            Sinhala::LetterAeeyanna => LETTER_AEEYANNA,
            Sinhala::LetterIyanna => LETTER_IYANNA,
            Sinhala::LetterIiyanna => LETTER_IIYANNA,
            Sinhala::LetterUyanna => LETTER_UYANNA,
            Sinhala::LetterUuyanna => LETTER_UUYANNA,
            Sinhala::LetterIruyanna => LETTER_IRUYANNA,
            Sinhala::LetterIruuyanna => LETTER_IRUUYANNA,
            Sinhala::LetterIluyanna => LETTER_ILUYANNA,
            Sinhala::LetterIluuyanna => LETTER_ILUUYANNA,
            Sinhala::LetterEyanna => LETTER_EYANNA,
            Sinhala::LetterEeyanna => LETTER_EEYANNA,
            Sinhala::LetterAiyanna => LETTER_AIYANNA,
            Sinhala::LetterOyanna => LETTER_OYANNA,
            Sinhala::LetterOoyanna => LETTER_OOYANNA,
            Sinhala::LetterAuyanna => LETTER_AUYANNA,
            Sinhala::LetterAlpapraanaKayanna => LETTER_ALPAPRAANA_KAYANNA,
            Sinhala::LetterMahaapraanaKayanna => LETTER_MAHAAPRAANA_KAYANNA,
            Sinhala::LetterAlpapraanaGayanna => LETTER_ALPAPRAANA_GAYANNA,
            Sinhala::LetterMahaapraanaGayanna => LETTER_MAHAAPRAANA_GAYANNA,
            Sinhala::LetterKantajaNaasikyaya => LETTER_KANTAJA_NAASIKYAYA,
            Sinhala::LetterSanyakaGayanna => LETTER_SANYAKA_GAYANNA,
            Sinhala::LetterAlpapraanaCayanna => LETTER_ALPAPRAANA_CAYANNA,
            Sinhala::LetterMahaapraanaCayanna => LETTER_MAHAAPRAANA_CAYANNA,
            Sinhala::LetterAlpapraanaJayanna => LETTER_ALPAPRAANA_JAYANNA,
            Sinhala::LetterMahaapraanaJayanna => LETTER_MAHAAPRAANA_JAYANNA,
            Sinhala::LetterTaalujaNaasikyaya => LETTER_TAALUJA_NAASIKYAYA,
            Sinhala::LetterTaalujaSanyoogaNaaksikyaya => LETTER_TAALUJA_SANYOOGA_NAAKSIKYAYA,
            Sinhala::LetterSanyakaJayanna => LETTER_SANYAKA_JAYANNA,
            Sinhala::LetterAlpapraanaTtayanna => LETTER_ALPAPRAANA_TTAYANNA,
            Sinhala::LetterMahaapraanaTtayanna => LETTER_MAHAAPRAANA_TTAYANNA,
            Sinhala::LetterAlpapraanaDdayanna => LETTER_ALPAPRAANA_DDAYANNA,
            Sinhala::LetterMahaapraanaDdayanna => LETTER_MAHAAPRAANA_DDAYANNA,
            Sinhala::LetterMuurdhajaNayanna => LETTER_MUURDHAJA_NAYANNA,
            Sinhala::LetterSanyakaDdayanna => LETTER_SANYAKA_DDAYANNA,
            Sinhala::LetterAlpapraanaTayanna => LETTER_ALPAPRAANA_TAYANNA,
            Sinhala::LetterMahaapraanaTayanna => LETTER_MAHAAPRAANA_TAYANNA,
            Sinhala::LetterAlpapraanaDayanna => LETTER_ALPAPRAANA_DAYANNA,
            Sinhala::LetterMahaapraanaDayanna => LETTER_MAHAAPRAANA_DAYANNA,
            Sinhala::LetterDantajaNayanna => LETTER_DANTAJA_NAYANNA,
            Sinhala::LetterSanyakaDayanna => LETTER_SANYAKA_DAYANNA,
            Sinhala::LetterAlpapraanaPayanna => LETTER_ALPAPRAANA_PAYANNA,
            Sinhala::LetterMahaapraanaPayanna => LETTER_MAHAAPRAANA_PAYANNA,
            Sinhala::LetterAlpapraanaBayanna => LETTER_ALPAPRAANA_BAYANNA,
            Sinhala::LetterMahaapraanaBayanna => LETTER_MAHAAPRAANA_BAYANNA,
            Sinhala::LetterMayanna => LETTER_MAYANNA,
            Sinhala::LetterAmbaBayanna => LETTER_AMBA_BAYANNA,
            Sinhala::LetterYayanna => LETTER_YAYANNA,
            Sinhala::LetterRayanna => LETTER_RAYANNA,
            Sinhala::LetterDantajaLayanna => LETTER_DANTAJA_LAYANNA,
            Sinhala::LetterVayanna => LETTER_VAYANNA,
            Sinhala::LetterTaalujaSayanna => LETTER_TAALUJA_SAYANNA,
            Sinhala::LetterMuurdhajaSayanna => LETTER_MUURDHAJA_SAYANNA,
            Sinhala::LetterDantajaSayanna => LETTER_DANTAJA_SAYANNA,
            Sinhala::LetterHayanna => LETTER_HAYANNA,
            Sinhala::LetterMuurdhajaLayanna => LETTER_MUURDHAJA_LAYANNA,
            Sinhala::LetterFayanna => LETTER_FAYANNA,
            Sinhala::SignAlDashLakuna => SIGN_AL_DASH_LAKUNA,
            Sinhala::VowelSignAelaDashPilla => VOWEL_SIGN_AELA_DASH_PILLA,
            Sinhala::VowelSignKettiAedaDashPilla => VOWEL_SIGN_KETTI_AEDA_DASH_PILLA,
            Sinhala::VowelSignDigaAedaDashPilla => VOWEL_SIGN_DIGA_AEDA_DASH_PILLA,
            Sinhala::VowelSignKettiIsDashPilla => VOWEL_SIGN_KETTI_IS_DASH_PILLA,
            Sinhala::VowelSignDigaIsDashPilla => VOWEL_SIGN_DIGA_IS_DASH_PILLA,
            Sinhala::VowelSignKettiPaaDashPilla => VOWEL_SIGN_KETTI_PAA_DASH_PILLA,
            Sinhala::VowelSignDigaPaaDashPilla => VOWEL_SIGN_DIGA_PAA_DASH_PILLA,
            Sinhala::VowelSignGaettaDashPilla => VOWEL_SIGN_GAETTA_DASH_PILLA,
            Sinhala::VowelSignKombuva => VOWEL_SIGN_KOMBUVA,
            Sinhala::VowelSignDigaKombuva => VOWEL_SIGN_DIGA_KOMBUVA,
            Sinhala::VowelSignKombuDeka => VOWEL_SIGN_KOMBU_DEKA,
            Sinhala::VowelSignKombuvaHaaAelaDashPilla => VOWEL_SIGN_KOMBUVA_HAA_AELA_DASH_PILLA,
            Sinhala::VowelSignKombuvaHaaDigaAelaDashPilla => VOWEL_SIGN_KOMBUVA_HAA_DIGA_AELA_DASH_PILLA,
            Sinhala::VowelSignKombuvaHaaGayanukitta => VOWEL_SIGN_KOMBUVA_HAA_GAYANUKITTA,
            Sinhala::VowelSignGayanukitta => VOWEL_SIGN_GAYANUKITTA,
            Sinhala::LithDigitZero => LITH_DIGIT_ZERO,
            Sinhala::LithDigitOne => LITH_DIGIT_ONE,
            Sinhala::LithDigitTwo => LITH_DIGIT_TWO,
            Sinhala::LithDigitThree => LITH_DIGIT_THREE,
            Sinhala::LithDigitFour => LITH_DIGIT_FOUR,
            Sinhala::LithDigitFive => LITH_DIGIT_FIVE,
            Sinhala::LithDigitSix => LITH_DIGIT_SIX,
            Sinhala::LithDigitSeven => LITH_DIGIT_SEVEN,
            Sinhala::LithDigitEight => LITH_DIGIT_EIGHT,
            Sinhala::LithDigitNine => LITH_DIGIT_NINE,
            Sinhala::VowelSignDigaGaettaDashPilla => VOWEL_SIGN_DIGA_GAETTA_DASH_PILLA,
            Sinhala::VowelSignDigaGayanukitta => VOWEL_SIGN_DIGA_GAYANUKITTA,
            Sinhala::PunctuationKunddaliya => PUNCTUATION_KUNDDALIYA,
        }
    }
}

impl std::convert::TryFrom<char> for Sinhala {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            SIGN_ANUSVARAYA => Ok(Sinhala::SignAnusvaraya),
            SIGN_VISARGAYA => Ok(Sinhala::SignVisargaya),
            LETTER_AYANNA => Ok(Sinhala::LetterAyanna),
            LETTER_AAYANNA => Ok(Sinhala::LetterAayanna),
            LETTER_AEYANNA => Ok(Sinhala::LetterAeyanna),
            LETTER_AEEYANNA => Ok(Sinhala::LetterAeeyanna),
            LETTER_IYANNA => Ok(Sinhala::LetterIyanna),
            LETTER_IIYANNA => Ok(Sinhala::LetterIiyanna),
            LETTER_UYANNA => Ok(Sinhala::LetterUyanna),
            LETTER_UUYANNA => Ok(Sinhala::LetterUuyanna),
            LETTER_IRUYANNA => Ok(Sinhala::LetterIruyanna),
            LETTER_IRUUYANNA => Ok(Sinhala::LetterIruuyanna),
            LETTER_ILUYANNA => Ok(Sinhala::LetterIluyanna),
            LETTER_ILUUYANNA => Ok(Sinhala::LetterIluuyanna),
            LETTER_EYANNA => Ok(Sinhala::LetterEyanna),
            LETTER_EEYANNA => Ok(Sinhala::LetterEeyanna),
            LETTER_AIYANNA => Ok(Sinhala::LetterAiyanna),
            LETTER_OYANNA => Ok(Sinhala::LetterOyanna),
            LETTER_OOYANNA => Ok(Sinhala::LetterOoyanna),
            LETTER_AUYANNA => Ok(Sinhala::LetterAuyanna),
            LETTER_ALPAPRAANA_KAYANNA => Ok(Sinhala::LetterAlpapraanaKayanna),
            LETTER_MAHAAPRAANA_KAYANNA => Ok(Sinhala::LetterMahaapraanaKayanna),
            LETTER_ALPAPRAANA_GAYANNA => Ok(Sinhala::LetterAlpapraanaGayanna),
            LETTER_MAHAAPRAANA_GAYANNA => Ok(Sinhala::LetterMahaapraanaGayanna),
            LETTER_KANTAJA_NAASIKYAYA => Ok(Sinhala::LetterKantajaNaasikyaya),
            LETTER_SANYAKA_GAYANNA => Ok(Sinhala::LetterSanyakaGayanna),
            LETTER_ALPAPRAANA_CAYANNA => Ok(Sinhala::LetterAlpapraanaCayanna),
            LETTER_MAHAAPRAANA_CAYANNA => Ok(Sinhala::LetterMahaapraanaCayanna),
            LETTER_ALPAPRAANA_JAYANNA => Ok(Sinhala::LetterAlpapraanaJayanna),
            LETTER_MAHAAPRAANA_JAYANNA => Ok(Sinhala::LetterMahaapraanaJayanna),
            LETTER_TAALUJA_NAASIKYAYA => Ok(Sinhala::LetterTaalujaNaasikyaya),
            LETTER_TAALUJA_SANYOOGA_NAAKSIKYAYA => Ok(Sinhala::LetterTaalujaSanyoogaNaaksikyaya),
            LETTER_SANYAKA_JAYANNA => Ok(Sinhala::LetterSanyakaJayanna),
            LETTER_ALPAPRAANA_TTAYANNA => Ok(Sinhala::LetterAlpapraanaTtayanna),
            LETTER_MAHAAPRAANA_TTAYANNA => Ok(Sinhala::LetterMahaapraanaTtayanna),
            LETTER_ALPAPRAANA_DDAYANNA => Ok(Sinhala::LetterAlpapraanaDdayanna),
            LETTER_MAHAAPRAANA_DDAYANNA => Ok(Sinhala::LetterMahaapraanaDdayanna),
            LETTER_MUURDHAJA_NAYANNA => Ok(Sinhala::LetterMuurdhajaNayanna),
            LETTER_SANYAKA_DDAYANNA => Ok(Sinhala::LetterSanyakaDdayanna),
            LETTER_ALPAPRAANA_TAYANNA => Ok(Sinhala::LetterAlpapraanaTayanna),
            LETTER_MAHAAPRAANA_TAYANNA => Ok(Sinhala::LetterMahaapraanaTayanna),
            LETTER_ALPAPRAANA_DAYANNA => Ok(Sinhala::LetterAlpapraanaDayanna),
            LETTER_MAHAAPRAANA_DAYANNA => Ok(Sinhala::LetterMahaapraanaDayanna),
            LETTER_DANTAJA_NAYANNA => Ok(Sinhala::LetterDantajaNayanna),
            LETTER_SANYAKA_DAYANNA => Ok(Sinhala::LetterSanyakaDayanna),
            LETTER_ALPAPRAANA_PAYANNA => Ok(Sinhala::LetterAlpapraanaPayanna),
            LETTER_MAHAAPRAANA_PAYANNA => Ok(Sinhala::LetterMahaapraanaPayanna),
            LETTER_ALPAPRAANA_BAYANNA => Ok(Sinhala::LetterAlpapraanaBayanna),
            LETTER_MAHAAPRAANA_BAYANNA => Ok(Sinhala::LetterMahaapraanaBayanna),
            LETTER_MAYANNA => Ok(Sinhala::LetterMayanna),
            LETTER_AMBA_BAYANNA => Ok(Sinhala::LetterAmbaBayanna),
            LETTER_YAYANNA => Ok(Sinhala::LetterYayanna),
            LETTER_RAYANNA => Ok(Sinhala::LetterRayanna),
            LETTER_DANTAJA_LAYANNA => Ok(Sinhala::LetterDantajaLayanna),
            LETTER_VAYANNA => Ok(Sinhala::LetterVayanna),
            LETTER_TAALUJA_SAYANNA => Ok(Sinhala::LetterTaalujaSayanna),
            LETTER_MUURDHAJA_SAYANNA => Ok(Sinhala::LetterMuurdhajaSayanna),
            LETTER_DANTAJA_SAYANNA => Ok(Sinhala::LetterDantajaSayanna),
            LETTER_HAYANNA => Ok(Sinhala::LetterHayanna),
            LETTER_MUURDHAJA_LAYANNA => Ok(Sinhala::LetterMuurdhajaLayanna),
            LETTER_FAYANNA => Ok(Sinhala::LetterFayanna),
            SIGN_AL_DASH_LAKUNA => Ok(Sinhala::SignAlDashLakuna),
            VOWEL_SIGN_AELA_DASH_PILLA => Ok(Sinhala::VowelSignAelaDashPilla),
            VOWEL_SIGN_KETTI_AEDA_DASH_PILLA => Ok(Sinhala::VowelSignKettiAedaDashPilla),
            VOWEL_SIGN_DIGA_AEDA_DASH_PILLA => Ok(Sinhala::VowelSignDigaAedaDashPilla),
            VOWEL_SIGN_KETTI_IS_DASH_PILLA => Ok(Sinhala::VowelSignKettiIsDashPilla),
            VOWEL_SIGN_DIGA_IS_DASH_PILLA => Ok(Sinhala::VowelSignDigaIsDashPilla),
            VOWEL_SIGN_KETTI_PAA_DASH_PILLA => Ok(Sinhala::VowelSignKettiPaaDashPilla),
            VOWEL_SIGN_DIGA_PAA_DASH_PILLA => Ok(Sinhala::VowelSignDigaPaaDashPilla),
            VOWEL_SIGN_GAETTA_DASH_PILLA => Ok(Sinhala::VowelSignGaettaDashPilla),
            VOWEL_SIGN_KOMBUVA => Ok(Sinhala::VowelSignKombuva),
            VOWEL_SIGN_DIGA_KOMBUVA => Ok(Sinhala::VowelSignDigaKombuva),
            VOWEL_SIGN_KOMBU_DEKA => Ok(Sinhala::VowelSignKombuDeka),
            VOWEL_SIGN_KOMBUVA_HAA_AELA_DASH_PILLA => Ok(Sinhala::VowelSignKombuvaHaaAelaDashPilla),
            VOWEL_SIGN_KOMBUVA_HAA_DIGA_AELA_DASH_PILLA => Ok(Sinhala::VowelSignKombuvaHaaDigaAelaDashPilla),
            VOWEL_SIGN_KOMBUVA_HAA_GAYANUKITTA => Ok(Sinhala::VowelSignKombuvaHaaGayanukitta),
            VOWEL_SIGN_GAYANUKITTA => Ok(Sinhala::VowelSignGayanukitta),
            LITH_DIGIT_ZERO => Ok(Sinhala::LithDigitZero),
            LITH_DIGIT_ONE => Ok(Sinhala::LithDigitOne),
            LITH_DIGIT_TWO => Ok(Sinhala::LithDigitTwo),
            LITH_DIGIT_THREE => Ok(Sinhala::LithDigitThree),
            LITH_DIGIT_FOUR => Ok(Sinhala::LithDigitFour),
            LITH_DIGIT_FIVE => Ok(Sinhala::LithDigitFive),
            LITH_DIGIT_SIX => Ok(Sinhala::LithDigitSix),
            LITH_DIGIT_SEVEN => Ok(Sinhala::LithDigitSeven),
            LITH_DIGIT_EIGHT => Ok(Sinhala::LithDigitEight),
            LITH_DIGIT_NINE => Ok(Sinhala::LithDigitNine),
            VOWEL_SIGN_DIGA_GAETTA_DASH_PILLA => Ok(Sinhala::VowelSignDigaGaettaDashPilla),
            VOWEL_SIGN_DIGA_GAYANUKITTA => Ok(Sinhala::VowelSignDigaGayanukitta),
            PUNCTUATION_KUNDDALIYA => Ok(Sinhala::PunctuationKunddaliya),
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

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Sinhala::SignAnusvaraya => "sinhala sign anusvaraya",
            Sinhala::SignVisargaya => "sinhala sign visargaya",
            Sinhala::LetterAyanna => "sinhala letter ayanna",
            Sinhala::LetterAayanna => "sinhala letter aayanna",
            Sinhala::LetterAeyanna => "sinhala letter aeyanna",
            Sinhala::LetterAeeyanna => "sinhala letter aeeyanna",
            Sinhala::LetterIyanna => "sinhala letter iyanna",
            Sinhala::LetterIiyanna => "sinhala letter iiyanna",
            Sinhala::LetterUyanna => "sinhala letter uyanna",
            Sinhala::LetterUuyanna => "sinhala letter uuyanna",
            Sinhala::LetterIruyanna => "sinhala letter iruyanna",
            Sinhala::LetterIruuyanna => "sinhala letter iruuyanna",
            Sinhala::LetterIluyanna => "sinhala letter iluyanna",
            Sinhala::LetterIluuyanna => "sinhala letter iluuyanna",
            Sinhala::LetterEyanna => "sinhala letter eyanna",
            Sinhala::LetterEeyanna => "sinhala letter eeyanna",
            Sinhala::LetterAiyanna => "sinhala letter aiyanna",
            Sinhala::LetterOyanna => "sinhala letter oyanna",
            Sinhala::LetterOoyanna => "sinhala letter ooyanna",
            Sinhala::LetterAuyanna => "sinhala letter auyanna",
            Sinhala::LetterAlpapraanaKayanna => "sinhala letter alpapraana kayanna",
            Sinhala::LetterMahaapraanaKayanna => "sinhala letter mahaapraana kayanna",
            Sinhala::LetterAlpapraanaGayanna => "sinhala letter alpapraana gayanna",
            Sinhala::LetterMahaapraanaGayanna => "sinhala letter mahaapraana gayanna",
            Sinhala::LetterKantajaNaasikyaya => "sinhala letter kantaja naasikyaya",
            Sinhala::LetterSanyakaGayanna => "sinhala letter sanyaka gayanna",
            Sinhala::LetterAlpapraanaCayanna => "sinhala letter alpapraana cayanna",
            Sinhala::LetterMahaapraanaCayanna => "sinhala letter mahaapraana cayanna",
            Sinhala::LetterAlpapraanaJayanna => "sinhala letter alpapraana jayanna",
            Sinhala::LetterMahaapraanaJayanna => "sinhala letter mahaapraana jayanna",
            Sinhala::LetterTaalujaNaasikyaya => "sinhala letter taaluja naasikyaya",
            Sinhala::LetterTaalujaSanyoogaNaaksikyaya => "sinhala letter taaluja sanyooga naaksikyaya",
            Sinhala::LetterSanyakaJayanna => "sinhala letter sanyaka jayanna",
            Sinhala::LetterAlpapraanaTtayanna => "sinhala letter alpapraana ttayanna",
            Sinhala::LetterMahaapraanaTtayanna => "sinhala letter mahaapraana ttayanna",
            Sinhala::LetterAlpapraanaDdayanna => "sinhala letter alpapraana ddayanna",
            Sinhala::LetterMahaapraanaDdayanna => "sinhala letter mahaapraana ddayanna",
            Sinhala::LetterMuurdhajaNayanna => "sinhala letter muurdhaja nayanna",
            Sinhala::LetterSanyakaDdayanna => "sinhala letter sanyaka ddayanna",
            Sinhala::LetterAlpapraanaTayanna => "sinhala letter alpapraana tayanna",
            Sinhala::LetterMahaapraanaTayanna => "sinhala letter mahaapraana tayanna",
            Sinhala::LetterAlpapraanaDayanna => "sinhala letter alpapraana dayanna",
            Sinhala::LetterMahaapraanaDayanna => "sinhala letter mahaapraana dayanna",
            Sinhala::LetterDantajaNayanna => "sinhala letter dantaja nayanna",
            Sinhala::LetterSanyakaDayanna => "sinhala letter sanyaka dayanna",
            Sinhala::LetterAlpapraanaPayanna => "sinhala letter alpapraana payanna",
            Sinhala::LetterMahaapraanaPayanna => "sinhala letter mahaapraana payanna",
            Sinhala::LetterAlpapraanaBayanna => "sinhala letter alpapraana bayanna",
            Sinhala::LetterMahaapraanaBayanna => "sinhala letter mahaapraana bayanna",
            Sinhala::LetterMayanna => "sinhala letter mayanna",
            Sinhala::LetterAmbaBayanna => "sinhala letter amba bayanna",
            Sinhala::LetterYayanna => "sinhala letter yayanna",
            Sinhala::LetterRayanna => "sinhala letter rayanna",
            Sinhala::LetterDantajaLayanna => "sinhala letter dantaja layanna",
            Sinhala::LetterVayanna => "sinhala letter vayanna",
            Sinhala::LetterTaalujaSayanna => "sinhala letter taaluja sayanna",
            Sinhala::LetterMuurdhajaSayanna => "sinhala letter muurdhaja sayanna",
            Sinhala::LetterDantajaSayanna => "sinhala letter dantaja sayanna",
            Sinhala::LetterHayanna => "sinhala letter hayanna",
            Sinhala::LetterMuurdhajaLayanna => "sinhala letter muurdhaja layanna",
            Sinhala::LetterFayanna => "sinhala letter fayanna",
            Sinhala::SignAlDashLakuna => "sinhala sign al-lakuna",
            Sinhala::VowelSignAelaDashPilla => "sinhala vowel sign aela-pilla",
            Sinhala::VowelSignKettiAedaDashPilla => "sinhala vowel sign ketti aeda-pilla",
            Sinhala::VowelSignDigaAedaDashPilla => "sinhala vowel sign diga aeda-pilla",
            Sinhala::VowelSignKettiIsDashPilla => "sinhala vowel sign ketti is-pilla",
            Sinhala::VowelSignDigaIsDashPilla => "sinhala vowel sign diga is-pilla",
            Sinhala::VowelSignKettiPaaDashPilla => "sinhala vowel sign ketti paa-pilla",
            Sinhala::VowelSignDigaPaaDashPilla => "sinhala vowel sign diga paa-pilla",
            Sinhala::VowelSignGaettaDashPilla => "sinhala vowel sign gaetta-pilla",
            Sinhala::VowelSignKombuva => "sinhala vowel sign kombuva",
            Sinhala::VowelSignDigaKombuva => "sinhala vowel sign diga kombuva",
            Sinhala::VowelSignKombuDeka => "sinhala vowel sign kombu deka",
            Sinhala::VowelSignKombuvaHaaAelaDashPilla => "sinhala vowel sign kombuva haa aela-pilla",
            Sinhala::VowelSignKombuvaHaaDigaAelaDashPilla => "sinhala vowel sign kombuva haa diga aela-pilla",
            Sinhala::VowelSignKombuvaHaaGayanukitta => "sinhala vowel sign kombuva haa gayanukitta",
            Sinhala::VowelSignGayanukitta => "sinhala vowel sign gayanukitta",
            Sinhala::LithDigitZero => "sinhala lith digit zero",
            Sinhala::LithDigitOne => "sinhala lith digit one",
            Sinhala::LithDigitTwo => "sinhala lith digit two",
            Sinhala::LithDigitThree => "sinhala lith digit three",
            Sinhala::LithDigitFour => "sinhala lith digit four",
            Sinhala::LithDigitFive => "sinhala lith digit five",
            Sinhala::LithDigitSix => "sinhala lith digit six",
            Sinhala::LithDigitSeven => "sinhala lith digit seven",
            Sinhala::LithDigitEight => "sinhala lith digit eight",
            Sinhala::LithDigitNine => "sinhala lith digit nine",
            Sinhala::VowelSignDigaGaettaDashPilla => "sinhala vowel sign diga gaetta-pilla",
            Sinhala::VowelSignDigaGayanukitta => "sinhala vowel sign diga gayanukitta",
            Sinhala::PunctuationKunddaliya => "sinhala punctuation kunddaliya",
        }
    }
}
