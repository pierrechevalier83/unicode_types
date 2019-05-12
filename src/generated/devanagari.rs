
/// An enum to represent all characters in the Devanagari block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Devanagari {
    /// \u{900}: 'ऀ'
    SignInvertedCandrabindu,
    /// \u{901}: 'ँ'
    SignCandrabindu,
    /// \u{902}: 'ं'
    SignAnusvara,
    /// \u{903}: 'ः'
    SignVisarga,
    /// \u{904}: 'ऄ'
    LetterShortA,
    /// \u{905}: 'अ'
    LetterA,
    /// \u{906}: 'आ'
    LetterAa,
    /// \u{907}: 'इ'
    LetterI,
    /// \u{908}: 'ई'
    LetterIi,
    /// \u{909}: 'उ'
    LetterU,
    /// \u{90a}: 'ऊ'
    LetterUu,
    /// \u{90b}: 'ऋ'
    LetterVocalicR,
    /// \u{90c}: 'ऌ'
    LetterVocalicL,
    /// \u{90d}: 'ऍ'
    LetterCandraE,
    /// \u{90e}: 'ऎ'
    LetterShortE,
    /// \u{90f}: 'ए'
    LetterE,
    /// \u{910}: 'ऐ'
    LetterAi,
    /// \u{911}: 'ऑ'
    LetterCandraO,
    /// \u{912}: 'ऒ'
    LetterShortO,
    /// \u{913}: 'ओ'
    LetterO,
    /// \u{914}: 'औ'
    LetterAu,
    /// \u{915}: 'क'
    LetterKa,
    /// \u{916}: 'ख'
    LetterKha,
    /// \u{917}: 'ग'
    LetterGa,
    /// \u{918}: 'घ'
    LetterGha,
    /// \u{919}: 'ङ'
    LetterNga,
    /// \u{91a}: 'च'
    LetterCa,
    /// \u{91b}: 'छ'
    LetterCha,
    /// \u{91c}: 'ज'
    LetterJa,
    /// \u{91d}: 'झ'
    LetterJha,
    /// \u{91e}: 'ञ'
    LetterNya,
    /// \u{91f}: 'ट'
    LetterTta,
    /// \u{920}: 'ठ'
    LetterTtha,
    /// \u{921}: 'ड'
    LetterDda,
    /// \u{922}: 'ढ'
    LetterDdha,
    /// \u{923}: 'ण'
    LetterNna,
    /// \u{924}: 'त'
    LetterTa,
    /// \u{925}: 'थ'
    LetterTha,
    /// \u{926}: 'द'
    LetterDa,
    /// \u{927}: 'ध'
    LetterDha,
    /// \u{928}: 'न'
    LetterNa,
    /// \u{929}: 'ऩ'
    LetterNnna,
    /// \u{92a}: 'प'
    LetterPa,
    /// \u{92b}: 'फ'
    LetterPha,
    /// \u{92c}: 'ब'
    LetterBa,
    /// \u{92d}: 'भ'
    LetterBha,
    /// \u{92e}: 'म'
    LetterMa,
    /// \u{92f}: 'य'
    LetterYa,
    /// \u{930}: 'र'
    LetterRa,
    /// \u{931}: 'ऱ'
    LetterRra,
    /// \u{932}: 'ल'
    LetterLa,
    /// \u{933}: 'ळ'
    LetterLla,
    /// \u{934}: 'ऴ'
    LetterLlla,
    /// \u{935}: 'व'
    LetterVa,
    /// \u{936}: 'श'
    LetterSha,
    /// \u{937}: 'ष'
    LetterSsa,
    /// \u{938}: 'स'
    LetterSa,
    /// \u{939}: 'ह'
    LetterHa,
    /// \u{93a}: 'ऺ'
    VowelSignOe,
    /// \u{93b}: 'ऻ'
    VowelSignOoe,
    /// \u{93c}: '़'
    SignNukta,
    /// \u{93d}: 'ऽ'
    SignAvagraha,
    /// \u{93e}: 'ा'
    VowelSignAa,
    /// \u{93f}: 'ि'
    VowelSignI,
    /// \u{940}: 'ी'
    VowelSignIi,
    /// \u{941}: 'ु'
    VowelSignU,
    /// \u{942}: 'ू'
    VowelSignUu,
    /// \u{943}: 'ृ'
    VowelSignVocalicR,
    /// \u{944}: 'ॄ'
    VowelSignVocalicRr,
    /// \u{945}: 'ॅ'
    VowelSignCandraE,
    /// \u{946}: 'ॆ'
    VowelSignShortE,
    /// \u{947}: 'े'
    VowelSignE,
    /// \u{948}: 'ै'
    VowelSignAi,
    /// \u{949}: 'ॉ'
    VowelSignCandraO,
    /// \u{94a}: 'ॊ'
    VowelSignShortO,
    /// \u{94b}: 'ो'
    VowelSignO,
    /// \u{94c}: 'ौ'
    VowelSignAu,
    /// \u{94d}: '्'
    SignVirama,
    /// \u{94e}: 'ॎ'
    VowelSignPrishthamatraE,
    /// \u{94f}: 'ॏ'
    VowelSignAw,
    /// \u{950}: 'ॐ'
    Om,
    /// \u{951}: '॑'
    StressSignUdatta,
    /// \u{952}: '॒'
    StressSignAnudatta,
    /// \u{953}: '॓'
    GraveAccent,
    /// \u{954}: '॔'
    AcuteAccent,
    /// \u{955}: 'ॕ'
    VowelSignCandraLongE,
    /// \u{956}: 'ॖ'
    VowelSignUe,
    /// \u{957}: 'ॗ'
    VowelSignUue,
    /// \u{958}: 'क़'
    LetterQa,
    /// \u{959}: 'ख़'
    LetterKhha,
    /// \u{95a}: 'ग़'
    LetterGhha,
    /// \u{95b}: 'ज़'
    LetterZa,
    /// \u{95c}: 'ड़'
    LetterDddha,
    /// \u{95d}: 'ढ़'
    LetterRha,
    /// \u{95e}: 'फ़'
    LetterFa,
    /// \u{95f}: 'य़'
    LetterYya,
    /// \u{960}: 'ॠ'
    LetterVocalicRr,
    /// \u{961}: 'ॡ'
    LetterVocalicLl,
    /// \u{962}: 'ॢ'
    VowelSignVocalicL,
    /// \u{963}: 'ॣ'
    VowelSignVocalicLl,
    /// \u{964}: '।'
    Danda,
    /// \u{965}: '॥'
    DoubleDanda,
    /// \u{966}: '०'
    DigitZero,
    /// \u{967}: '१'
    DigitOne,
    /// \u{968}: '२'
    DigitTwo,
    /// \u{969}: '३'
    DigitThree,
    /// \u{96a}: '४'
    DigitFour,
    /// \u{96b}: '५'
    DigitFive,
    /// \u{96c}: '६'
    DigitSix,
    /// \u{96d}: '७'
    DigitSeven,
    /// \u{96e}: '८'
    DigitEight,
    /// \u{96f}: '९'
    DigitNine,
    /// \u{970}: '॰'
    AbbreviationSign,
    /// \u{971}: 'ॱ'
    SignHighSpacingDot,
    /// \u{972}: 'ॲ'
    LetterCandraA,
    /// \u{973}: 'ॳ'
    LetterOe,
    /// \u{974}: 'ॴ'
    LetterOoe,
    /// \u{975}: 'ॵ'
    LetterAw,
    /// \u{976}: 'ॶ'
    LetterUe,
    /// \u{977}: 'ॷ'
    LetterUue,
    /// \u{978}: 'ॸ'
    LetterMarwariDda,
    /// \u{979}: 'ॹ'
    LetterZha,
    /// \u{97a}: 'ॺ'
    LetterHeavyYa,
    /// \u{97b}: 'ॻ'
    LetterGga,
    /// \u{97c}: 'ॼ'
    LetterJja,
    /// \u{97d}: 'ॽ'
    LetterGlottalStop,
    /// \u{97e}: 'ॾ'
    LetterDdda,
}

impl Into<char> for Devanagari {
    fn into(self) -> char {
        match self {
            Devanagari::SignInvertedCandrabindu => 'ऀ',
            Devanagari::SignCandrabindu => 'ँ',
            Devanagari::SignAnusvara => 'ं',
            Devanagari::SignVisarga => 'ः',
            Devanagari::LetterShortA => 'ऄ',
            Devanagari::LetterA => 'अ',
            Devanagari::LetterAa => 'आ',
            Devanagari::LetterI => 'इ',
            Devanagari::LetterIi => 'ई',
            Devanagari::LetterU => 'उ',
            Devanagari::LetterUu => 'ऊ',
            Devanagari::LetterVocalicR => 'ऋ',
            Devanagari::LetterVocalicL => 'ऌ',
            Devanagari::LetterCandraE => 'ऍ',
            Devanagari::LetterShortE => 'ऎ',
            Devanagari::LetterE => 'ए',
            Devanagari::LetterAi => 'ऐ',
            Devanagari::LetterCandraO => 'ऑ',
            Devanagari::LetterShortO => 'ऒ',
            Devanagari::LetterO => 'ओ',
            Devanagari::LetterAu => 'औ',
            Devanagari::LetterKa => 'क',
            Devanagari::LetterKha => 'ख',
            Devanagari::LetterGa => 'ग',
            Devanagari::LetterGha => 'घ',
            Devanagari::LetterNga => 'ङ',
            Devanagari::LetterCa => 'च',
            Devanagari::LetterCha => 'छ',
            Devanagari::LetterJa => 'ज',
            Devanagari::LetterJha => 'झ',
            Devanagari::LetterNya => 'ञ',
            Devanagari::LetterTta => 'ट',
            Devanagari::LetterTtha => 'ठ',
            Devanagari::LetterDda => 'ड',
            Devanagari::LetterDdha => 'ढ',
            Devanagari::LetterNna => 'ण',
            Devanagari::LetterTa => 'त',
            Devanagari::LetterTha => 'थ',
            Devanagari::LetterDa => 'द',
            Devanagari::LetterDha => 'ध',
            Devanagari::LetterNa => 'न',
            Devanagari::LetterNnna => 'ऩ',
            Devanagari::LetterPa => 'प',
            Devanagari::LetterPha => 'फ',
            Devanagari::LetterBa => 'ब',
            Devanagari::LetterBha => 'भ',
            Devanagari::LetterMa => 'म',
            Devanagari::LetterYa => 'य',
            Devanagari::LetterRa => 'र',
            Devanagari::LetterRra => 'ऱ',
            Devanagari::LetterLa => 'ल',
            Devanagari::LetterLla => 'ळ',
            Devanagari::LetterLlla => 'ऴ',
            Devanagari::LetterVa => 'व',
            Devanagari::LetterSha => 'श',
            Devanagari::LetterSsa => 'ष',
            Devanagari::LetterSa => 'स',
            Devanagari::LetterHa => 'ह',
            Devanagari::VowelSignOe => 'ऺ',
            Devanagari::VowelSignOoe => 'ऻ',
            Devanagari::SignNukta => '़',
            Devanagari::SignAvagraha => 'ऽ',
            Devanagari::VowelSignAa => 'ा',
            Devanagari::VowelSignI => 'ि',
            Devanagari::VowelSignIi => 'ी',
            Devanagari::VowelSignU => 'ु',
            Devanagari::VowelSignUu => 'ू',
            Devanagari::VowelSignVocalicR => 'ृ',
            Devanagari::VowelSignVocalicRr => 'ॄ',
            Devanagari::VowelSignCandraE => 'ॅ',
            Devanagari::VowelSignShortE => 'ॆ',
            Devanagari::VowelSignE => 'े',
            Devanagari::VowelSignAi => 'ै',
            Devanagari::VowelSignCandraO => 'ॉ',
            Devanagari::VowelSignShortO => 'ॊ',
            Devanagari::VowelSignO => 'ो',
            Devanagari::VowelSignAu => 'ौ',
            Devanagari::SignVirama => '्',
            Devanagari::VowelSignPrishthamatraE => 'ॎ',
            Devanagari::VowelSignAw => 'ॏ',
            Devanagari::Om => 'ॐ',
            Devanagari::StressSignUdatta => '॑',
            Devanagari::StressSignAnudatta => '॒',
            Devanagari::GraveAccent => '॓',
            Devanagari::AcuteAccent => '॔',
            Devanagari::VowelSignCandraLongE => 'ॕ',
            Devanagari::VowelSignUe => 'ॖ',
            Devanagari::VowelSignUue => 'ॗ',
            Devanagari::LetterQa => 'क़',
            Devanagari::LetterKhha => 'ख़',
            Devanagari::LetterGhha => 'ग़',
            Devanagari::LetterZa => 'ज़',
            Devanagari::LetterDddha => 'ड़',
            Devanagari::LetterRha => 'ढ़',
            Devanagari::LetterFa => 'फ़',
            Devanagari::LetterYya => 'य़',
            Devanagari::LetterVocalicRr => 'ॠ',
            Devanagari::LetterVocalicLl => 'ॡ',
            Devanagari::VowelSignVocalicL => 'ॢ',
            Devanagari::VowelSignVocalicLl => 'ॣ',
            Devanagari::Danda => '।',
            Devanagari::DoubleDanda => '॥',
            Devanagari::DigitZero => '०',
            Devanagari::DigitOne => '१',
            Devanagari::DigitTwo => '२',
            Devanagari::DigitThree => '३',
            Devanagari::DigitFour => '४',
            Devanagari::DigitFive => '५',
            Devanagari::DigitSix => '६',
            Devanagari::DigitSeven => '७',
            Devanagari::DigitEight => '८',
            Devanagari::DigitNine => '९',
            Devanagari::AbbreviationSign => '॰',
            Devanagari::SignHighSpacingDot => 'ॱ',
            Devanagari::LetterCandraA => 'ॲ',
            Devanagari::LetterOe => 'ॳ',
            Devanagari::LetterOoe => 'ॴ',
            Devanagari::LetterAw => 'ॵ',
            Devanagari::LetterUe => 'ॶ',
            Devanagari::LetterUue => 'ॷ',
            Devanagari::LetterMarwariDda => 'ॸ',
            Devanagari::LetterZha => 'ॹ',
            Devanagari::LetterHeavyYa => 'ॺ',
            Devanagari::LetterGga => 'ॻ',
            Devanagari::LetterJja => 'ॼ',
            Devanagari::LetterGlottalStop => 'ॽ',
            Devanagari::LetterDdda => 'ॾ',
        }
    }
}

impl std::convert::TryFrom<char> for Devanagari {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ऀ' => Ok(Devanagari::SignInvertedCandrabindu),
            'ँ' => Ok(Devanagari::SignCandrabindu),
            'ं' => Ok(Devanagari::SignAnusvara),
            'ः' => Ok(Devanagari::SignVisarga),
            'ऄ' => Ok(Devanagari::LetterShortA),
            'अ' => Ok(Devanagari::LetterA),
            'आ' => Ok(Devanagari::LetterAa),
            'इ' => Ok(Devanagari::LetterI),
            'ई' => Ok(Devanagari::LetterIi),
            'उ' => Ok(Devanagari::LetterU),
            'ऊ' => Ok(Devanagari::LetterUu),
            'ऋ' => Ok(Devanagari::LetterVocalicR),
            'ऌ' => Ok(Devanagari::LetterVocalicL),
            'ऍ' => Ok(Devanagari::LetterCandraE),
            'ऎ' => Ok(Devanagari::LetterShortE),
            'ए' => Ok(Devanagari::LetterE),
            'ऐ' => Ok(Devanagari::LetterAi),
            'ऑ' => Ok(Devanagari::LetterCandraO),
            'ऒ' => Ok(Devanagari::LetterShortO),
            'ओ' => Ok(Devanagari::LetterO),
            'औ' => Ok(Devanagari::LetterAu),
            'क' => Ok(Devanagari::LetterKa),
            'ख' => Ok(Devanagari::LetterKha),
            'ग' => Ok(Devanagari::LetterGa),
            'घ' => Ok(Devanagari::LetterGha),
            'ङ' => Ok(Devanagari::LetterNga),
            'च' => Ok(Devanagari::LetterCa),
            'छ' => Ok(Devanagari::LetterCha),
            'ज' => Ok(Devanagari::LetterJa),
            'झ' => Ok(Devanagari::LetterJha),
            'ञ' => Ok(Devanagari::LetterNya),
            'ट' => Ok(Devanagari::LetterTta),
            'ठ' => Ok(Devanagari::LetterTtha),
            'ड' => Ok(Devanagari::LetterDda),
            'ढ' => Ok(Devanagari::LetterDdha),
            'ण' => Ok(Devanagari::LetterNna),
            'त' => Ok(Devanagari::LetterTa),
            'थ' => Ok(Devanagari::LetterTha),
            'द' => Ok(Devanagari::LetterDa),
            'ध' => Ok(Devanagari::LetterDha),
            'न' => Ok(Devanagari::LetterNa),
            'ऩ' => Ok(Devanagari::LetterNnna),
            'प' => Ok(Devanagari::LetterPa),
            'फ' => Ok(Devanagari::LetterPha),
            'ब' => Ok(Devanagari::LetterBa),
            'भ' => Ok(Devanagari::LetterBha),
            'म' => Ok(Devanagari::LetterMa),
            'य' => Ok(Devanagari::LetterYa),
            'र' => Ok(Devanagari::LetterRa),
            'ऱ' => Ok(Devanagari::LetterRra),
            'ल' => Ok(Devanagari::LetterLa),
            'ळ' => Ok(Devanagari::LetterLla),
            'ऴ' => Ok(Devanagari::LetterLlla),
            'व' => Ok(Devanagari::LetterVa),
            'श' => Ok(Devanagari::LetterSha),
            'ष' => Ok(Devanagari::LetterSsa),
            'स' => Ok(Devanagari::LetterSa),
            'ह' => Ok(Devanagari::LetterHa),
            'ऺ' => Ok(Devanagari::VowelSignOe),
            'ऻ' => Ok(Devanagari::VowelSignOoe),
            '़' => Ok(Devanagari::SignNukta),
            'ऽ' => Ok(Devanagari::SignAvagraha),
            'ा' => Ok(Devanagari::VowelSignAa),
            'ि' => Ok(Devanagari::VowelSignI),
            'ी' => Ok(Devanagari::VowelSignIi),
            'ु' => Ok(Devanagari::VowelSignU),
            'ू' => Ok(Devanagari::VowelSignUu),
            'ृ' => Ok(Devanagari::VowelSignVocalicR),
            'ॄ' => Ok(Devanagari::VowelSignVocalicRr),
            'ॅ' => Ok(Devanagari::VowelSignCandraE),
            'ॆ' => Ok(Devanagari::VowelSignShortE),
            'े' => Ok(Devanagari::VowelSignE),
            'ै' => Ok(Devanagari::VowelSignAi),
            'ॉ' => Ok(Devanagari::VowelSignCandraO),
            'ॊ' => Ok(Devanagari::VowelSignShortO),
            'ो' => Ok(Devanagari::VowelSignO),
            'ौ' => Ok(Devanagari::VowelSignAu),
            '्' => Ok(Devanagari::SignVirama),
            'ॎ' => Ok(Devanagari::VowelSignPrishthamatraE),
            'ॏ' => Ok(Devanagari::VowelSignAw),
            'ॐ' => Ok(Devanagari::Om),
            '॑' => Ok(Devanagari::StressSignUdatta),
            '॒' => Ok(Devanagari::StressSignAnudatta),
            '॓' => Ok(Devanagari::GraveAccent),
            '॔' => Ok(Devanagari::AcuteAccent),
            'ॕ' => Ok(Devanagari::VowelSignCandraLongE),
            'ॖ' => Ok(Devanagari::VowelSignUe),
            'ॗ' => Ok(Devanagari::VowelSignUue),
            'क़' => Ok(Devanagari::LetterQa),
            'ख़' => Ok(Devanagari::LetterKhha),
            'ग़' => Ok(Devanagari::LetterGhha),
            'ज़' => Ok(Devanagari::LetterZa),
            'ड़' => Ok(Devanagari::LetterDddha),
            'ढ़' => Ok(Devanagari::LetterRha),
            'फ़' => Ok(Devanagari::LetterFa),
            'य़' => Ok(Devanagari::LetterYya),
            'ॠ' => Ok(Devanagari::LetterVocalicRr),
            'ॡ' => Ok(Devanagari::LetterVocalicLl),
            'ॢ' => Ok(Devanagari::VowelSignVocalicL),
            'ॣ' => Ok(Devanagari::VowelSignVocalicLl),
            '।' => Ok(Devanagari::Danda),
            '॥' => Ok(Devanagari::DoubleDanda),
            '०' => Ok(Devanagari::DigitZero),
            '१' => Ok(Devanagari::DigitOne),
            '२' => Ok(Devanagari::DigitTwo),
            '३' => Ok(Devanagari::DigitThree),
            '४' => Ok(Devanagari::DigitFour),
            '५' => Ok(Devanagari::DigitFive),
            '६' => Ok(Devanagari::DigitSix),
            '७' => Ok(Devanagari::DigitSeven),
            '८' => Ok(Devanagari::DigitEight),
            '९' => Ok(Devanagari::DigitNine),
            '॰' => Ok(Devanagari::AbbreviationSign),
            'ॱ' => Ok(Devanagari::SignHighSpacingDot),
            'ॲ' => Ok(Devanagari::LetterCandraA),
            'ॳ' => Ok(Devanagari::LetterOe),
            'ॴ' => Ok(Devanagari::LetterOoe),
            'ॵ' => Ok(Devanagari::LetterAw),
            'ॶ' => Ok(Devanagari::LetterUe),
            'ॷ' => Ok(Devanagari::LetterUue),
            'ॸ' => Ok(Devanagari::LetterMarwariDda),
            'ॹ' => Ok(Devanagari::LetterZha),
            'ॺ' => Ok(Devanagari::LetterHeavyYa),
            'ॻ' => Ok(Devanagari::LetterGga),
            'ॼ' => Ok(Devanagari::LetterJja),
            'ॽ' => Ok(Devanagari::LetterGlottalStop),
            'ॾ' => Ok(Devanagari::LetterDdda),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Devanagari {
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

impl std::convert::TryFrom<u32> for Devanagari {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Devanagari {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Devanagari {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Devanagari::SignInvertedCandrabindu
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Devanagari{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
