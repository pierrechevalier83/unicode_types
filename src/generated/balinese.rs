
/// An enum to represent all characters in the Balinese block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Balinese {
    /// \u{1b00}: 'ᬀ'
    SignUluRicem,
    /// \u{1b01}: 'ᬁ'
    SignUluCandra,
    /// \u{1b02}: 'ᬂ'
    SignCecek,
    /// \u{1b03}: 'ᬃ'
    SignSurang,
    /// \u{1b04}: 'ᬄ'
    SignBisah,
    /// \u{1b05}: 'ᬅ'
    LetterAkara,
    /// \u{1b06}: 'ᬆ'
    LetterAkaraTedung,
    /// \u{1b07}: 'ᬇ'
    LetterIkara,
    /// \u{1b08}: 'ᬈ'
    LetterIkaraTedung,
    /// \u{1b09}: 'ᬉ'
    LetterUkara,
    /// \u{1b0a}: 'ᬊ'
    LetterUkaraTedung,
    /// \u{1b0b}: 'ᬋ'
    LetterRaRepa,
    /// \u{1b0c}: 'ᬌ'
    LetterRaRepaTedung,
    /// \u{1b0d}: 'ᬍ'
    LetterLaLenga,
    /// \u{1b0e}: 'ᬎ'
    LetterLaLengaTedung,
    /// \u{1b0f}: 'ᬏ'
    LetterEkara,
    /// \u{1b10}: 'ᬐ'
    LetterAikara,
    /// \u{1b11}: 'ᬑ'
    LetterOkara,
    /// \u{1b12}: 'ᬒ'
    LetterOkaraTedung,
    /// \u{1b13}: 'ᬓ'
    LetterKa,
    /// \u{1b14}: 'ᬔ'
    LetterKaMahaprana,
    /// \u{1b15}: 'ᬕ'
    LetterGa,
    /// \u{1b16}: 'ᬖ'
    LetterGaGora,
    /// \u{1b17}: 'ᬗ'
    LetterNga,
    /// \u{1b18}: 'ᬘ'
    LetterCa,
    /// \u{1b19}: 'ᬙ'
    LetterCaLaca,
    /// \u{1b1a}: 'ᬚ'
    LetterJa,
    /// \u{1b1b}: 'ᬛ'
    LetterJaJera,
    /// \u{1b1c}: 'ᬜ'
    LetterNya,
    /// \u{1b1d}: 'ᬝ'
    LetterTaLatik,
    /// \u{1b1e}: 'ᬞ'
    LetterTaMurdaMahaprana,
    /// \u{1b1f}: 'ᬟ'
    LetterDaMurdaAlpaprana,
    /// \u{1b20}: 'ᬠ'
    LetterDaMurdaMahaprana,
    /// \u{1b21}: 'ᬡ'
    LetterNaRambat,
    /// \u{1b22}: 'ᬢ'
    LetterTa,
    /// \u{1b23}: 'ᬣ'
    LetterTaTawa,
    /// \u{1b24}: 'ᬤ'
    LetterDa,
    /// \u{1b25}: 'ᬥ'
    LetterDaMadu,
    /// \u{1b26}: 'ᬦ'
    LetterNa,
    /// \u{1b27}: 'ᬧ'
    LetterPa,
    /// \u{1b28}: 'ᬨ'
    LetterPaKapal,
    /// \u{1b29}: 'ᬩ'
    LetterBa,
    /// \u{1b2a}: 'ᬪ'
    LetterBaKembang,
    /// \u{1b2b}: 'ᬫ'
    LetterMa,
    /// \u{1b2c}: 'ᬬ'
    LetterYa,
    /// \u{1b2d}: 'ᬭ'
    LetterRa,
    /// \u{1b2e}: 'ᬮ'
    LetterLa,
    /// \u{1b2f}: 'ᬯ'
    LetterWa,
    /// \u{1b30}: 'ᬰ'
    LetterSaSaga,
    /// \u{1b31}: 'ᬱ'
    LetterSaSapa,
    /// \u{1b32}: 'ᬲ'
    LetterSa,
    /// \u{1b33}: 'ᬳ'
    LetterHa,
    /// \u{1b34}: '᬴'
    SignRerekan,
    /// \u{1b35}: 'ᬵ'
    VowelSignTedung,
    /// \u{1b36}: 'ᬶ'
    VowelSignUlu,
    /// \u{1b37}: 'ᬷ'
    VowelSignUluSari,
    /// \u{1b38}: 'ᬸ'
    VowelSignSuku,
    /// \u{1b39}: 'ᬹ'
    VowelSignSukuIlut,
    /// \u{1b3a}: 'ᬺ'
    VowelSignRaRepa,
    /// \u{1b3b}: 'ᬻ'
    VowelSignRaRepaTedung,
    /// \u{1b3c}: 'ᬼ'
    VowelSignLaLenga,
    /// \u{1b3d}: 'ᬽ'
    VowelSignLaLengaTedung,
    /// \u{1b3e}: 'ᬾ'
    VowelSignTaling,
    /// \u{1b3f}: 'ᬿ'
    VowelSignTalingRepa,
    /// \u{1b40}: 'ᭀ'
    VowelSignTalingTedung,
    /// \u{1b41}: 'ᭁ'
    VowelSignTalingRepaTedung,
    /// \u{1b42}: 'ᭂ'
    VowelSignPepet,
    /// \u{1b43}: 'ᭃ'
    VowelSignPepetTedung,
    /// \u{1b44}: '᭄'
    AdegAdeg,
    /// \u{1b45}: 'ᭅ'
    LetterKafSasak,
    /// \u{1b46}: 'ᭆ'
    LetterKhotSasak,
    /// \u{1b47}: 'ᭇ'
    LetterTzirSasak,
    /// \u{1b48}: 'ᭈ'
    LetterEfSasak,
    /// \u{1b49}: 'ᭉ'
    LetterVeSasak,
    /// \u{1b4a}: 'ᭊ'
    LetterZalSasak,
    /// \u{1b4b}: 'ᭋ'
    LetterAsyuraSasak,
    /// \u{1b50}: '᭐'
    DigitZero,
    /// \u{1b51}: '᭑'
    DigitOne,
    /// \u{1b52}: '᭒'
    DigitTwo,
    /// \u{1b53}: '᭓'
    DigitThree,
    /// \u{1b54}: '᭔'
    DigitFour,
    /// \u{1b55}: '᭕'
    DigitFive,
    /// \u{1b56}: '᭖'
    DigitSix,
    /// \u{1b57}: '᭗'
    DigitSeven,
    /// \u{1b58}: '᭘'
    DigitEight,
    /// \u{1b59}: '᭙'
    DigitNine,
    /// \u{1b5a}: '᭚'
    Panti,
    /// \u{1b5b}: '᭛'
    Pamada,
    /// \u{1b5c}: '᭜'
    Windu,
    /// \u{1b5d}: '᭝'
    CarikPamungkah,
    /// \u{1b5e}: '᭞'
    CarikSiki,
    /// \u{1b5f}: '᭟'
    CarikPareren,
    /// \u{1b60}: '᭠'
    Pameneng,
    /// \u{1b61}: '᭡'
    MusicalSymbolDong,
    /// \u{1b62}: '᭢'
    MusicalSymbolDeng,
    /// \u{1b63}: '᭣'
    MusicalSymbolDung,
    /// \u{1b64}: '᭤'
    MusicalSymbolDang,
    /// \u{1b65}: '᭥'
    MusicalSymbolDangSurang,
    /// \u{1b66}: '᭦'
    MusicalSymbolDing,
    /// \u{1b67}: '᭧'
    MusicalSymbolDaeng,
    /// \u{1b68}: '᭨'
    MusicalSymbolDeung,
    /// \u{1b69}: '᭩'
    MusicalSymbolDaing,
    /// \u{1b6a}: '᭪'
    MusicalSymbolDangGede,
    /// \u{1b6b}: '᭫'
    MusicalSymbolCombiningTegeh,
    /// \u{1b6c}: '᭬'
    MusicalSymbolCombiningEndep,
    /// \u{1b6d}: '᭭'
    MusicalSymbolCombiningKempul,
    /// \u{1b6e}: '᭮'
    MusicalSymbolCombiningKempli,
    /// \u{1b6f}: '᭯'
    MusicalSymbolCombiningJegogan,
    /// \u{1b70}: '᭰'
    MusicalSymbolCombiningKempulWithJegogan,
    /// \u{1b71}: '᭱'
    MusicalSymbolCombiningKempliWithJegogan,
    /// \u{1b72}: '᭲'
    MusicalSymbolCombiningBende,
    /// \u{1b73}: '᭳'
    MusicalSymbolCombiningGong,
    /// \u{1b74}: '᭴'
    MusicalSymbolRightDashHandOpenDug,
    /// \u{1b75}: '᭵'
    MusicalSymbolRightDashHandOpenDag,
    /// \u{1b76}: '᭶'
    MusicalSymbolRightDashHandClosedTuk,
    /// \u{1b77}: '᭷'
    MusicalSymbolRightDashHandClosedTak,
    /// \u{1b78}: '᭸'
    MusicalSymbolLeftDashHandOpenPang,
    /// \u{1b79}: '᭹'
    MusicalSymbolLeftDashHandOpenPung,
    /// \u{1b7a}: '᭺'
    MusicalSymbolLeftDashHandClosedPlak,
    /// \u{1b7b}: '᭻'
    MusicalSymbolLeftDashHandClosedPluk,
    /// \u{1b7c}: '᭼'
    MusicalSymbolLeftDashHandOpenPing,
}

impl Into<char> for Balinese {
    fn into(self) -> char {
        match self {
            Balinese::SignUluRicem => 'ᬀ',
            Balinese::SignUluCandra => 'ᬁ',
            Balinese::SignCecek => 'ᬂ',
            Balinese::SignSurang => 'ᬃ',
            Balinese::SignBisah => 'ᬄ',
            Balinese::LetterAkara => 'ᬅ',
            Balinese::LetterAkaraTedung => 'ᬆ',
            Balinese::LetterIkara => 'ᬇ',
            Balinese::LetterIkaraTedung => 'ᬈ',
            Balinese::LetterUkara => 'ᬉ',
            Balinese::LetterUkaraTedung => 'ᬊ',
            Balinese::LetterRaRepa => 'ᬋ',
            Balinese::LetterRaRepaTedung => 'ᬌ',
            Balinese::LetterLaLenga => 'ᬍ',
            Balinese::LetterLaLengaTedung => 'ᬎ',
            Balinese::LetterEkara => 'ᬏ',
            Balinese::LetterAikara => 'ᬐ',
            Balinese::LetterOkara => 'ᬑ',
            Balinese::LetterOkaraTedung => 'ᬒ',
            Balinese::LetterKa => 'ᬓ',
            Balinese::LetterKaMahaprana => 'ᬔ',
            Balinese::LetterGa => 'ᬕ',
            Balinese::LetterGaGora => 'ᬖ',
            Balinese::LetterNga => 'ᬗ',
            Balinese::LetterCa => 'ᬘ',
            Balinese::LetterCaLaca => 'ᬙ',
            Balinese::LetterJa => 'ᬚ',
            Balinese::LetterJaJera => 'ᬛ',
            Balinese::LetterNya => 'ᬜ',
            Balinese::LetterTaLatik => 'ᬝ',
            Balinese::LetterTaMurdaMahaprana => 'ᬞ',
            Balinese::LetterDaMurdaAlpaprana => 'ᬟ',
            Balinese::LetterDaMurdaMahaprana => 'ᬠ',
            Balinese::LetterNaRambat => 'ᬡ',
            Balinese::LetterTa => 'ᬢ',
            Balinese::LetterTaTawa => 'ᬣ',
            Balinese::LetterDa => 'ᬤ',
            Balinese::LetterDaMadu => 'ᬥ',
            Balinese::LetterNa => 'ᬦ',
            Balinese::LetterPa => 'ᬧ',
            Balinese::LetterPaKapal => 'ᬨ',
            Balinese::LetterBa => 'ᬩ',
            Balinese::LetterBaKembang => 'ᬪ',
            Balinese::LetterMa => 'ᬫ',
            Balinese::LetterYa => 'ᬬ',
            Balinese::LetterRa => 'ᬭ',
            Balinese::LetterLa => 'ᬮ',
            Balinese::LetterWa => 'ᬯ',
            Balinese::LetterSaSaga => 'ᬰ',
            Balinese::LetterSaSapa => 'ᬱ',
            Balinese::LetterSa => 'ᬲ',
            Balinese::LetterHa => 'ᬳ',
            Balinese::SignRerekan => '᬴',
            Balinese::VowelSignTedung => 'ᬵ',
            Balinese::VowelSignUlu => 'ᬶ',
            Balinese::VowelSignUluSari => 'ᬷ',
            Balinese::VowelSignSuku => 'ᬸ',
            Balinese::VowelSignSukuIlut => 'ᬹ',
            Balinese::VowelSignRaRepa => 'ᬺ',
            Balinese::VowelSignRaRepaTedung => 'ᬻ',
            Balinese::VowelSignLaLenga => 'ᬼ',
            Balinese::VowelSignLaLengaTedung => 'ᬽ',
            Balinese::VowelSignTaling => 'ᬾ',
            Balinese::VowelSignTalingRepa => 'ᬿ',
            Balinese::VowelSignTalingTedung => 'ᭀ',
            Balinese::VowelSignTalingRepaTedung => 'ᭁ',
            Balinese::VowelSignPepet => 'ᭂ',
            Balinese::VowelSignPepetTedung => 'ᭃ',
            Balinese::AdegAdeg => '᭄',
            Balinese::LetterKafSasak => 'ᭅ',
            Balinese::LetterKhotSasak => 'ᭆ',
            Balinese::LetterTzirSasak => 'ᭇ',
            Balinese::LetterEfSasak => 'ᭈ',
            Balinese::LetterVeSasak => 'ᭉ',
            Balinese::LetterZalSasak => 'ᭊ',
            Balinese::LetterAsyuraSasak => 'ᭋ',
            Balinese::DigitZero => '᭐',
            Balinese::DigitOne => '᭑',
            Balinese::DigitTwo => '᭒',
            Balinese::DigitThree => '᭓',
            Balinese::DigitFour => '᭔',
            Balinese::DigitFive => '᭕',
            Balinese::DigitSix => '᭖',
            Balinese::DigitSeven => '᭗',
            Balinese::DigitEight => '᭘',
            Balinese::DigitNine => '᭙',
            Balinese::Panti => '᭚',
            Balinese::Pamada => '᭛',
            Balinese::Windu => '᭜',
            Balinese::CarikPamungkah => '᭝',
            Balinese::CarikSiki => '᭞',
            Balinese::CarikPareren => '᭟',
            Balinese::Pameneng => '᭠',
            Balinese::MusicalSymbolDong => '᭡',
            Balinese::MusicalSymbolDeng => '᭢',
            Balinese::MusicalSymbolDung => '᭣',
            Balinese::MusicalSymbolDang => '᭤',
            Balinese::MusicalSymbolDangSurang => '᭥',
            Balinese::MusicalSymbolDing => '᭦',
            Balinese::MusicalSymbolDaeng => '᭧',
            Balinese::MusicalSymbolDeung => '᭨',
            Balinese::MusicalSymbolDaing => '᭩',
            Balinese::MusicalSymbolDangGede => '᭪',
            Balinese::MusicalSymbolCombiningTegeh => '᭫',
            Balinese::MusicalSymbolCombiningEndep => '᭬',
            Balinese::MusicalSymbolCombiningKempul => '᭭',
            Balinese::MusicalSymbolCombiningKempli => '᭮',
            Balinese::MusicalSymbolCombiningJegogan => '᭯',
            Balinese::MusicalSymbolCombiningKempulWithJegogan => '᭰',
            Balinese::MusicalSymbolCombiningKempliWithJegogan => '᭱',
            Balinese::MusicalSymbolCombiningBende => '᭲',
            Balinese::MusicalSymbolCombiningGong => '᭳',
            Balinese::MusicalSymbolRightDashHandOpenDug => '᭴',
            Balinese::MusicalSymbolRightDashHandOpenDag => '᭵',
            Balinese::MusicalSymbolRightDashHandClosedTuk => '᭶',
            Balinese::MusicalSymbolRightDashHandClosedTak => '᭷',
            Balinese::MusicalSymbolLeftDashHandOpenPang => '᭸',
            Balinese::MusicalSymbolLeftDashHandOpenPung => '᭹',
            Balinese::MusicalSymbolLeftDashHandClosedPlak => '᭺',
            Balinese::MusicalSymbolLeftDashHandClosedPluk => '᭻',
            Balinese::MusicalSymbolLeftDashHandOpenPing => '᭼',
        }
    }
}

impl std::convert::TryFrom<char> for Balinese {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ᬀ' => Ok(Balinese::SignUluRicem),
            'ᬁ' => Ok(Balinese::SignUluCandra),
            'ᬂ' => Ok(Balinese::SignCecek),
            'ᬃ' => Ok(Balinese::SignSurang),
            'ᬄ' => Ok(Balinese::SignBisah),
            'ᬅ' => Ok(Balinese::LetterAkara),
            'ᬆ' => Ok(Balinese::LetterAkaraTedung),
            'ᬇ' => Ok(Balinese::LetterIkara),
            'ᬈ' => Ok(Balinese::LetterIkaraTedung),
            'ᬉ' => Ok(Balinese::LetterUkara),
            'ᬊ' => Ok(Balinese::LetterUkaraTedung),
            'ᬋ' => Ok(Balinese::LetterRaRepa),
            'ᬌ' => Ok(Balinese::LetterRaRepaTedung),
            'ᬍ' => Ok(Balinese::LetterLaLenga),
            'ᬎ' => Ok(Balinese::LetterLaLengaTedung),
            'ᬏ' => Ok(Balinese::LetterEkara),
            'ᬐ' => Ok(Balinese::LetterAikara),
            'ᬑ' => Ok(Balinese::LetterOkara),
            'ᬒ' => Ok(Balinese::LetterOkaraTedung),
            'ᬓ' => Ok(Balinese::LetterKa),
            'ᬔ' => Ok(Balinese::LetterKaMahaprana),
            'ᬕ' => Ok(Balinese::LetterGa),
            'ᬖ' => Ok(Balinese::LetterGaGora),
            'ᬗ' => Ok(Balinese::LetterNga),
            'ᬘ' => Ok(Balinese::LetterCa),
            'ᬙ' => Ok(Balinese::LetterCaLaca),
            'ᬚ' => Ok(Balinese::LetterJa),
            'ᬛ' => Ok(Balinese::LetterJaJera),
            'ᬜ' => Ok(Balinese::LetterNya),
            'ᬝ' => Ok(Balinese::LetterTaLatik),
            'ᬞ' => Ok(Balinese::LetterTaMurdaMahaprana),
            'ᬟ' => Ok(Balinese::LetterDaMurdaAlpaprana),
            'ᬠ' => Ok(Balinese::LetterDaMurdaMahaprana),
            'ᬡ' => Ok(Balinese::LetterNaRambat),
            'ᬢ' => Ok(Balinese::LetterTa),
            'ᬣ' => Ok(Balinese::LetterTaTawa),
            'ᬤ' => Ok(Balinese::LetterDa),
            'ᬥ' => Ok(Balinese::LetterDaMadu),
            'ᬦ' => Ok(Balinese::LetterNa),
            'ᬧ' => Ok(Balinese::LetterPa),
            'ᬨ' => Ok(Balinese::LetterPaKapal),
            'ᬩ' => Ok(Balinese::LetterBa),
            'ᬪ' => Ok(Balinese::LetterBaKembang),
            'ᬫ' => Ok(Balinese::LetterMa),
            'ᬬ' => Ok(Balinese::LetterYa),
            'ᬭ' => Ok(Balinese::LetterRa),
            'ᬮ' => Ok(Balinese::LetterLa),
            'ᬯ' => Ok(Balinese::LetterWa),
            'ᬰ' => Ok(Balinese::LetterSaSaga),
            'ᬱ' => Ok(Balinese::LetterSaSapa),
            'ᬲ' => Ok(Balinese::LetterSa),
            'ᬳ' => Ok(Balinese::LetterHa),
            '᬴' => Ok(Balinese::SignRerekan),
            'ᬵ' => Ok(Balinese::VowelSignTedung),
            'ᬶ' => Ok(Balinese::VowelSignUlu),
            'ᬷ' => Ok(Balinese::VowelSignUluSari),
            'ᬸ' => Ok(Balinese::VowelSignSuku),
            'ᬹ' => Ok(Balinese::VowelSignSukuIlut),
            'ᬺ' => Ok(Balinese::VowelSignRaRepa),
            'ᬻ' => Ok(Balinese::VowelSignRaRepaTedung),
            'ᬼ' => Ok(Balinese::VowelSignLaLenga),
            'ᬽ' => Ok(Balinese::VowelSignLaLengaTedung),
            'ᬾ' => Ok(Balinese::VowelSignTaling),
            'ᬿ' => Ok(Balinese::VowelSignTalingRepa),
            'ᭀ' => Ok(Balinese::VowelSignTalingTedung),
            'ᭁ' => Ok(Balinese::VowelSignTalingRepaTedung),
            'ᭂ' => Ok(Balinese::VowelSignPepet),
            'ᭃ' => Ok(Balinese::VowelSignPepetTedung),
            '᭄' => Ok(Balinese::AdegAdeg),
            'ᭅ' => Ok(Balinese::LetterKafSasak),
            'ᭆ' => Ok(Balinese::LetterKhotSasak),
            'ᭇ' => Ok(Balinese::LetterTzirSasak),
            'ᭈ' => Ok(Balinese::LetterEfSasak),
            'ᭉ' => Ok(Balinese::LetterVeSasak),
            'ᭊ' => Ok(Balinese::LetterZalSasak),
            'ᭋ' => Ok(Balinese::LetterAsyuraSasak),
            '᭐' => Ok(Balinese::DigitZero),
            '᭑' => Ok(Balinese::DigitOne),
            '᭒' => Ok(Balinese::DigitTwo),
            '᭓' => Ok(Balinese::DigitThree),
            '᭔' => Ok(Balinese::DigitFour),
            '᭕' => Ok(Balinese::DigitFive),
            '᭖' => Ok(Balinese::DigitSix),
            '᭗' => Ok(Balinese::DigitSeven),
            '᭘' => Ok(Balinese::DigitEight),
            '᭙' => Ok(Balinese::DigitNine),
            '᭚' => Ok(Balinese::Panti),
            '᭛' => Ok(Balinese::Pamada),
            '᭜' => Ok(Balinese::Windu),
            '᭝' => Ok(Balinese::CarikPamungkah),
            '᭞' => Ok(Balinese::CarikSiki),
            '᭟' => Ok(Balinese::CarikPareren),
            '᭠' => Ok(Balinese::Pameneng),
            '᭡' => Ok(Balinese::MusicalSymbolDong),
            '᭢' => Ok(Balinese::MusicalSymbolDeng),
            '᭣' => Ok(Balinese::MusicalSymbolDung),
            '᭤' => Ok(Balinese::MusicalSymbolDang),
            '᭥' => Ok(Balinese::MusicalSymbolDangSurang),
            '᭦' => Ok(Balinese::MusicalSymbolDing),
            '᭧' => Ok(Balinese::MusicalSymbolDaeng),
            '᭨' => Ok(Balinese::MusicalSymbolDeung),
            '᭩' => Ok(Balinese::MusicalSymbolDaing),
            '᭪' => Ok(Balinese::MusicalSymbolDangGede),
            '᭫' => Ok(Balinese::MusicalSymbolCombiningTegeh),
            '᭬' => Ok(Balinese::MusicalSymbolCombiningEndep),
            '᭭' => Ok(Balinese::MusicalSymbolCombiningKempul),
            '᭮' => Ok(Balinese::MusicalSymbolCombiningKempli),
            '᭯' => Ok(Balinese::MusicalSymbolCombiningJegogan),
            '᭰' => Ok(Balinese::MusicalSymbolCombiningKempulWithJegogan),
            '᭱' => Ok(Balinese::MusicalSymbolCombiningKempliWithJegogan),
            '᭲' => Ok(Balinese::MusicalSymbolCombiningBende),
            '᭳' => Ok(Balinese::MusicalSymbolCombiningGong),
            '᭴' => Ok(Balinese::MusicalSymbolRightDashHandOpenDug),
            '᭵' => Ok(Balinese::MusicalSymbolRightDashHandOpenDag),
            '᭶' => Ok(Balinese::MusicalSymbolRightDashHandClosedTuk),
            '᭷' => Ok(Balinese::MusicalSymbolRightDashHandClosedTak),
            '᭸' => Ok(Balinese::MusicalSymbolLeftDashHandOpenPang),
            '᭹' => Ok(Balinese::MusicalSymbolLeftDashHandOpenPung),
            '᭺' => Ok(Balinese::MusicalSymbolLeftDashHandClosedPlak),
            '᭻' => Ok(Balinese::MusicalSymbolLeftDashHandClosedPluk),
            '᭼' => Ok(Balinese::MusicalSymbolLeftDashHandOpenPing),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Balinese {
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

impl std::convert::TryFrom<u32> for Balinese {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Balinese {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Balinese {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Balinese::SignUluRicem
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Balinese{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
