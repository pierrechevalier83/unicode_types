/// A number of constants to give a name to all characters in this block.
mod constants {
    /// \u{1b00}: 'ᬀ'
    pub const SIGN_ULU_RICEM: char = 'ᬀ';
    /// \u{1b01}: 'ᬁ'
    pub const SIGN_ULU_CANDRA: char = 'ᬁ';
    /// \u{1b02}: 'ᬂ'
    pub const SIGN_CECEK: char = 'ᬂ';
    /// \u{1b03}: 'ᬃ'
    pub const SIGN_SURANG: char = 'ᬃ';
    /// \u{1b04}: 'ᬄ'
    pub const SIGN_BISAH: char = 'ᬄ';
    /// \u{1b05}: 'ᬅ'
    pub const LETTER_AKARA: char = 'ᬅ';
    /// \u{1b06}: 'ᬆ'
    pub const LETTER_AKARA_TEDUNG: char = 'ᬆ';
    /// \u{1b07}: 'ᬇ'
    pub const LETTER_IKARA: char = 'ᬇ';
    /// \u{1b08}: 'ᬈ'
    pub const LETTER_IKARA_TEDUNG: char = 'ᬈ';
    /// \u{1b09}: 'ᬉ'
    pub const LETTER_UKARA: char = 'ᬉ';
    /// \u{1b0a}: 'ᬊ'
    pub const LETTER_UKARA_TEDUNG: char = 'ᬊ';
    /// \u{1b0b}: 'ᬋ'
    pub const LETTER_RA_REPA: char = 'ᬋ';
    /// \u{1b0c}: 'ᬌ'
    pub const LETTER_RA_REPA_TEDUNG: char = 'ᬌ';
    /// \u{1b0d}: 'ᬍ'
    pub const LETTER_LA_LENGA: char = 'ᬍ';
    /// \u{1b0e}: 'ᬎ'
    pub const LETTER_LA_LENGA_TEDUNG: char = 'ᬎ';
    /// \u{1b0f}: 'ᬏ'
    pub const LETTER_EKARA: char = 'ᬏ';
    /// \u{1b10}: 'ᬐ'
    pub const LETTER_AIKARA: char = 'ᬐ';
    /// \u{1b11}: 'ᬑ'
    pub const LETTER_OKARA: char = 'ᬑ';
    /// \u{1b12}: 'ᬒ'
    pub const LETTER_OKARA_TEDUNG: char = 'ᬒ';
    /// \u{1b13}: 'ᬓ'
    pub const LETTER_KA: char = 'ᬓ';
    /// \u{1b14}: 'ᬔ'
    pub const LETTER_KA_MAHAPRANA: char = 'ᬔ';
    /// \u{1b15}: 'ᬕ'
    pub const LETTER_GA: char = 'ᬕ';
    /// \u{1b16}: 'ᬖ'
    pub const LETTER_GA_GORA: char = 'ᬖ';
    /// \u{1b17}: 'ᬗ'
    pub const LETTER_NGA: char = 'ᬗ';
    /// \u{1b18}: 'ᬘ'
    pub const LETTER_CA: char = 'ᬘ';
    /// \u{1b19}: 'ᬙ'
    pub const LETTER_CA_LACA: char = 'ᬙ';
    /// \u{1b1a}: 'ᬚ'
    pub const LETTER_JA: char = 'ᬚ';
    /// \u{1b1b}: 'ᬛ'
    pub const LETTER_JA_JERA: char = 'ᬛ';
    /// \u{1b1c}: 'ᬜ'
    pub const LETTER_NYA: char = 'ᬜ';
    /// \u{1b1d}: 'ᬝ'
    pub const LETTER_TA_LATIK: char = 'ᬝ';
    /// \u{1b1e}: 'ᬞ'
    pub const LETTER_TA_MURDA_MAHAPRANA: char = 'ᬞ';
    /// \u{1b1f}: 'ᬟ'
    pub const LETTER_DA_MURDA_ALPAPRANA: char = 'ᬟ';
    /// \u{1b20}: 'ᬠ'
    pub const LETTER_DA_MURDA_MAHAPRANA: char = 'ᬠ';
    /// \u{1b21}: 'ᬡ'
    pub const LETTER_NA_RAMBAT: char = 'ᬡ';
    /// \u{1b22}: 'ᬢ'
    pub const LETTER_TA: char = 'ᬢ';
    /// \u{1b23}: 'ᬣ'
    pub const LETTER_TA_TAWA: char = 'ᬣ';
    /// \u{1b24}: 'ᬤ'
    pub const LETTER_DA: char = 'ᬤ';
    /// \u{1b25}: 'ᬥ'
    pub const LETTER_DA_MADU: char = 'ᬥ';
    /// \u{1b26}: 'ᬦ'
    pub const LETTER_NA: char = 'ᬦ';
    /// \u{1b27}: 'ᬧ'
    pub const LETTER_PA: char = 'ᬧ';
    /// \u{1b28}: 'ᬨ'
    pub const LETTER_PA_KAPAL: char = 'ᬨ';
    /// \u{1b29}: 'ᬩ'
    pub const LETTER_BA: char = 'ᬩ';
    /// \u{1b2a}: 'ᬪ'
    pub const LETTER_BA_KEMBANG: char = 'ᬪ';
    /// \u{1b2b}: 'ᬫ'
    pub const LETTER_MA: char = 'ᬫ';
    /// \u{1b2c}: 'ᬬ'
    pub const LETTER_YA: char = 'ᬬ';
    /// \u{1b2d}: 'ᬭ'
    pub const LETTER_RA: char = 'ᬭ';
    /// \u{1b2e}: 'ᬮ'
    pub const LETTER_LA: char = 'ᬮ';
    /// \u{1b2f}: 'ᬯ'
    pub const LETTER_WA: char = 'ᬯ';
    /// \u{1b30}: 'ᬰ'
    pub const LETTER_SA_SAGA: char = 'ᬰ';
    /// \u{1b31}: 'ᬱ'
    pub const LETTER_SA_SAPA: char = 'ᬱ';
    /// \u{1b32}: 'ᬲ'
    pub const LETTER_SA: char = 'ᬲ';
    /// \u{1b33}: 'ᬳ'
    pub const LETTER_HA: char = 'ᬳ';
    /// \u{1b34}: '᬴'
    pub const SIGN_REREKAN: char = '᬴';
    /// \u{1b35}: 'ᬵ'
    pub const VOWEL_SIGN_TEDUNG: char = 'ᬵ';
    /// \u{1b36}: 'ᬶ'
    pub const VOWEL_SIGN_ULU: char = 'ᬶ';
    /// \u{1b37}: 'ᬷ'
    pub const VOWEL_SIGN_ULU_SARI: char = 'ᬷ';
    /// \u{1b38}: 'ᬸ'
    pub const VOWEL_SIGN_SUKU: char = 'ᬸ';
    /// \u{1b39}: 'ᬹ'
    pub const VOWEL_SIGN_SUKU_ILUT: char = 'ᬹ';
    /// \u{1b3a}: 'ᬺ'
    pub const VOWEL_SIGN_RA_REPA: char = 'ᬺ';
    /// \u{1b3b}: 'ᬻ'
    pub const VOWEL_SIGN_RA_REPA_TEDUNG: char = 'ᬻ';
    /// \u{1b3c}: 'ᬼ'
    pub const VOWEL_SIGN_LA_LENGA: char = 'ᬼ';
    /// \u{1b3d}: 'ᬽ'
    pub const VOWEL_SIGN_LA_LENGA_TEDUNG: char = 'ᬽ';
    /// \u{1b3e}: 'ᬾ'
    pub const VOWEL_SIGN_TALING: char = 'ᬾ';
    /// \u{1b3f}: 'ᬿ'
    pub const VOWEL_SIGN_TALING_REPA: char = 'ᬿ';
    /// \u{1b40}: 'ᭀ'
    pub const VOWEL_SIGN_TALING_TEDUNG: char = 'ᭀ';
    /// \u{1b41}: 'ᭁ'
    pub const VOWEL_SIGN_TALING_REPA_TEDUNG: char = 'ᭁ';
    /// \u{1b42}: 'ᭂ'
    pub const VOWEL_SIGN_PEPET: char = 'ᭂ';
    /// \u{1b43}: 'ᭃ'
    pub const VOWEL_SIGN_PEPET_TEDUNG: char = 'ᭃ';
    /// \u{1b44}: '᭄'
    pub const ADEG_ADEG: char = '᭄';
    /// \u{1b45}: 'ᭅ'
    pub const LETTER_KAF_SASAK: char = 'ᭅ';
    /// \u{1b46}: 'ᭆ'
    pub const LETTER_KHOT_SASAK: char = 'ᭆ';
    /// \u{1b47}: 'ᭇ'
    pub const LETTER_TZIR_SASAK: char = 'ᭇ';
    /// \u{1b48}: 'ᭈ'
    pub const LETTER_EF_SASAK: char = 'ᭈ';
    /// \u{1b49}: 'ᭉ'
    pub const LETTER_VE_SASAK: char = 'ᭉ';
    /// \u{1b4a}: 'ᭊ'
    pub const LETTER_ZAL_SASAK: char = 'ᭊ';
    /// \u{1b4b}: 'ᭋ'
    pub const LETTER_ASYURA_SASAK: char = 'ᭋ';
    /// \u{1b50}: '᭐'
    pub const DIGIT_ZERO: char = '᭐';
    /// \u{1b51}: '᭑'
    pub const DIGIT_ONE: char = '᭑';
    /// \u{1b52}: '᭒'
    pub const DIGIT_TWO: char = '᭒';
    /// \u{1b53}: '᭓'
    pub const DIGIT_THREE: char = '᭓';
    /// \u{1b54}: '᭔'
    pub const DIGIT_FOUR: char = '᭔';
    /// \u{1b55}: '᭕'
    pub const DIGIT_FIVE: char = '᭕';
    /// \u{1b56}: '᭖'
    pub const DIGIT_SIX: char = '᭖';
    /// \u{1b57}: '᭗'
    pub const DIGIT_SEVEN: char = '᭗';
    /// \u{1b58}: '᭘'
    pub const DIGIT_EIGHT: char = '᭘';
    /// \u{1b59}: '᭙'
    pub const DIGIT_NINE: char = '᭙';
    /// \u{1b5a}: '᭚'
    pub const PANTI: char = '᭚';
    /// \u{1b5b}: '᭛'
    pub const PAMADA: char = '᭛';
    /// \u{1b5c}: '᭜'
    pub const WINDU: char = '᭜';
    /// \u{1b5d}: '᭝'
    pub const CARIK_PAMUNGKAH: char = '᭝';
    /// \u{1b5e}: '᭞'
    pub const CARIK_SIKI: char = '᭞';
    /// \u{1b5f}: '᭟'
    pub const CARIK_PAREREN: char = '᭟';
    /// \u{1b60}: '᭠'
    pub const PAMENENG: char = '᭠';
    /// \u{1b61}: '᭡'
    pub const MUSICAL_SYMBOL_DONG: char = '᭡';
    /// \u{1b62}: '᭢'
    pub const MUSICAL_SYMBOL_DENG: char = '᭢';
    /// \u{1b63}: '᭣'
    pub const MUSICAL_SYMBOL_DUNG: char = '᭣';
    /// \u{1b64}: '᭤'
    pub const MUSICAL_SYMBOL_DANG: char = '᭤';
    /// \u{1b65}: '᭥'
    pub const MUSICAL_SYMBOL_DANG_SURANG: char = '᭥';
    /// \u{1b66}: '᭦'
    pub const MUSICAL_SYMBOL_DING: char = '᭦';
    /// \u{1b67}: '᭧'
    pub const MUSICAL_SYMBOL_DAENG: char = '᭧';
    /// \u{1b68}: '᭨'
    pub const MUSICAL_SYMBOL_DEUNG: char = '᭨';
    /// \u{1b69}: '᭩'
    pub const MUSICAL_SYMBOL_DAING: char = '᭩';
    /// \u{1b6a}: '᭪'
    pub const MUSICAL_SYMBOL_DANG_GEDE: char = '᭪';
    /// \u{1b6b}: '᭫'
    pub const MUSICAL_SYMBOL_COMBINING_TEGEH: char = '᭫';
    /// \u{1b6c}: '᭬'
    pub const MUSICAL_SYMBOL_COMBINING_ENDEP: char = '᭬';
    /// \u{1b6d}: '᭭'
    pub const MUSICAL_SYMBOL_COMBINING_KEMPUL: char = '᭭';
    /// \u{1b6e}: '᭮'
    pub const MUSICAL_SYMBOL_COMBINING_KEMPLI: char = '᭮';
    /// \u{1b6f}: '᭯'
    pub const MUSICAL_SYMBOL_COMBINING_JEGOGAN: char = '᭯';
    /// \u{1b70}: '᭰'
    pub const MUSICAL_SYMBOL_COMBINING_KEMPUL_WITH_JEGOGAN: char = '᭰';
    /// \u{1b71}: '᭱'
    pub const MUSICAL_SYMBOL_COMBINING_KEMPLI_WITH_JEGOGAN: char = '᭱';
    /// \u{1b72}: '᭲'
    pub const MUSICAL_SYMBOL_COMBINING_BENDE: char = '᭲';
    /// \u{1b73}: '᭳'
    pub const MUSICAL_SYMBOL_COMBINING_GONG: char = '᭳';
    /// \u{1b74}: '᭴'
    pub const MUSICAL_SYMBOL_RIGHT_DASH_HAND_OPEN_DUG: char = '᭴';
    /// \u{1b75}: '᭵'
    pub const MUSICAL_SYMBOL_RIGHT_DASH_HAND_OPEN_DAG: char = '᭵';
    /// \u{1b76}: '᭶'
    pub const MUSICAL_SYMBOL_RIGHT_DASH_HAND_CLOSED_TUK: char = '᭶';
    /// \u{1b77}: '᭷'
    pub const MUSICAL_SYMBOL_RIGHT_DASH_HAND_CLOSED_TAK: char = '᭷';
    /// \u{1b78}: '᭸'
    pub const MUSICAL_SYMBOL_LEFT_DASH_HAND_OPEN_PANG: char = '᭸';
    /// \u{1b79}: '᭹'
    pub const MUSICAL_SYMBOL_LEFT_DASH_HAND_OPEN_PUNG: char = '᭹';
    /// \u{1b7a}: '᭺'
    pub const MUSICAL_SYMBOL_LEFT_DASH_HAND_CLOSED_PLAK: char = '᭺';
    /// \u{1b7b}: '᭻'
    pub const MUSICAL_SYMBOL_LEFT_DASH_HAND_CLOSED_PLUK: char = '᭻';
    /// \u{1b7c}: '᭼'
    pub const MUSICAL_SYMBOL_LEFT_DASH_HAND_OPEN_PING: char = '᭼';
}

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
        use constants::*;
        match self {
            Balinese::SignUluRicem => SIGN_ULU_RICEM,
            Balinese::SignUluCandra => SIGN_ULU_CANDRA,
            Balinese::SignCecek => SIGN_CECEK,
            Balinese::SignSurang => SIGN_SURANG,
            Balinese::SignBisah => SIGN_BISAH,
            Balinese::LetterAkara => LETTER_AKARA,
            Balinese::LetterAkaraTedung => LETTER_AKARA_TEDUNG,
            Balinese::LetterIkara => LETTER_IKARA,
            Balinese::LetterIkaraTedung => LETTER_IKARA_TEDUNG,
            Balinese::LetterUkara => LETTER_UKARA,
            Balinese::LetterUkaraTedung => LETTER_UKARA_TEDUNG,
            Balinese::LetterRaRepa => LETTER_RA_REPA,
            Balinese::LetterRaRepaTedung => LETTER_RA_REPA_TEDUNG,
            Balinese::LetterLaLenga => LETTER_LA_LENGA,
            Balinese::LetterLaLengaTedung => LETTER_LA_LENGA_TEDUNG,
            Balinese::LetterEkara => LETTER_EKARA,
            Balinese::LetterAikara => LETTER_AIKARA,
            Balinese::LetterOkara => LETTER_OKARA,
            Balinese::LetterOkaraTedung => LETTER_OKARA_TEDUNG,
            Balinese::LetterKa => LETTER_KA,
            Balinese::LetterKaMahaprana => LETTER_KA_MAHAPRANA,
            Balinese::LetterGa => LETTER_GA,
            Balinese::LetterGaGora => LETTER_GA_GORA,
            Balinese::LetterNga => LETTER_NGA,
            Balinese::LetterCa => LETTER_CA,
            Balinese::LetterCaLaca => LETTER_CA_LACA,
            Balinese::LetterJa => LETTER_JA,
            Balinese::LetterJaJera => LETTER_JA_JERA,
            Balinese::LetterNya => LETTER_NYA,
            Balinese::LetterTaLatik => LETTER_TA_LATIK,
            Balinese::LetterTaMurdaMahaprana => LETTER_TA_MURDA_MAHAPRANA,
            Balinese::LetterDaMurdaAlpaprana => LETTER_DA_MURDA_ALPAPRANA,
            Balinese::LetterDaMurdaMahaprana => LETTER_DA_MURDA_MAHAPRANA,
            Balinese::LetterNaRambat => LETTER_NA_RAMBAT,
            Balinese::LetterTa => LETTER_TA,
            Balinese::LetterTaTawa => LETTER_TA_TAWA,
            Balinese::LetterDa => LETTER_DA,
            Balinese::LetterDaMadu => LETTER_DA_MADU,
            Balinese::LetterNa => LETTER_NA,
            Balinese::LetterPa => LETTER_PA,
            Balinese::LetterPaKapal => LETTER_PA_KAPAL,
            Balinese::LetterBa => LETTER_BA,
            Balinese::LetterBaKembang => LETTER_BA_KEMBANG,
            Balinese::LetterMa => LETTER_MA,
            Balinese::LetterYa => LETTER_YA,
            Balinese::LetterRa => LETTER_RA,
            Balinese::LetterLa => LETTER_LA,
            Balinese::LetterWa => LETTER_WA,
            Balinese::LetterSaSaga => LETTER_SA_SAGA,
            Balinese::LetterSaSapa => LETTER_SA_SAPA,
            Balinese::LetterSa => LETTER_SA,
            Balinese::LetterHa => LETTER_HA,
            Balinese::SignRerekan => SIGN_REREKAN,
            Balinese::VowelSignTedung => VOWEL_SIGN_TEDUNG,
            Balinese::VowelSignUlu => VOWEL_SIGN_ULU,
            Balinese::VowelSignUluSari => VOWEL_SIGN_ULU_SARI,
            Balinese::VowelSignSuku => VOWEL_SIGN_SUKU,
            Balinese::VowelSignSukuIlut => VOWEL_SIGN_SUKU_ILUT,
            Balinese::VowelSignRaRepa => VOWEL_SIGN_RA_REPA,
            Balinese::VowelSignRaRepaTedung => VOWEL_SIGN_RA_REPA_TEDUNG,
            Balinese::VowelSignLaLenga => VOWEL_SIGN_LA_LENGA,
            Balinese::VowelSignLaLengaTedung => VOWEL_SIGN_LA_LENGA_TEDUNG,
            Balinese::VowelSignTaling => VOWEL_SIGN_TALING,
            Balinese::VowelSignTalingRepa => VOWEL_SIGN_TALING_REPA,
            Balinese::VowelSignTalingTedung => VOWEL_SIGN_TALING_TEDUNG,
            Balinese::VowelSignTalingRepaTedung => VOWEL_SIGN_TALING_REPA_TEDUNG,
            Balinese::VowelSignPepet => VOWEL_SIGN_PEPET,
            Balinese::VowelSignPepetTedung => VOWEL_SIGN_PEPET_TEDUNG,
            Balinese::AdegAdeg => ADEG_ADEG,
            Balinese::LetterKafSasak => LETTER_KAF_SASAK,
            Balinese::LetterKhotSasak => LETTER_KHOT_SASAK,
            Balinese::LetterTzirSasak => LETTER_TZIR_SASAK,
            Balinese::LetterEfSasak => LETTER_EF_SASAK,
            Balinese::LetterVeSasak => LETTER_VE_SASAK,
            Balinese::LetterZalSasak => LETTER_ZAL_SASAK,
            Balinese::LetterAsyuraSasak => LETTER_ASYURA_SASAK,
            Balinese::DigitZero => DIGIT_ZERO,
            Balinese::DigitOne => DIGIT_ONE,
            Balinese::DigitTwo => DIGIT_TWO,
            Balinese::DigitThree => DIGIT_THREE,
            Balinese::DigitFour => DIGIT_FOUR,
            Balinese::DigitFive => DIGIT_FIVE,
            Balinese::DigitSix => DIGIT_SIX,
            Balinese::DigitSeven => DIGIT_SEVEN,
            Balinese::DigitEight => DIGIT_EIGHT,
            Balinese::DigitNine => DIGIT_NINE,
            Balinese::Panti => PANTI,
            Balinese::Pamada => PAMADA,
            Balinese::Windu => WINDU,
            Balinese::CarikPamungkah => CARIK_PAMUNGKAH,
            Balinese::CarikSiki => CARIK_SIKI,
            Balinese::CarikPareren => CARIK_PAREREN,
            Balinese::Pameneng => PAMENENG,
            Balinese::MusicalSymbolDong => MUSICAL_SYMBOL_DONG,
            Balinese::MusicalSymbolDeng => MUSICAL_SYMBOL_DENG,
            Balinese::MusicalSymbolDung => MUSICAL_SYMBOL_DUNG,
            Balinese::MusicalSymbolDang => MUSICAL_SYMBOL_DANG,
            Balinese::MusicalSymbolDangSurang => MUSICAL_SYMBOL_DANG_SURANG,
            Balinese::MusicalSymbolDing => MUSICAL_SYMBOL_DING,
            Balinese::MusicalSymbolDaeng => MUSICAL_SYMBOL_DAENG,
            Balinese::MusicalSymbolDeung => MUSICAL_SYMBOL_DEUNG,
            Balinese::MusicalSymbolDaing => MUSICAL_SYMBOL_DAING,
            Balinese::MusicalSymbolDangGede => MUSICAL_SYMBOL_DANG_GEDE,
            Balinese::MusicalSymbolCombiningTegeh => MUSICAL_SYMBOL_COMBINING_TEGEH,
            Balinese::MusicalSymbolCombiningEndep => MUSICAL_SYMBOL_COMBINING_ENDEP,
            Balinese::MusicalSymbolCombiningKempul => MUSICAL_SYMBOL_COMBINING_KEMPUL,
            Balinese::MusicalSymbolCombiningKempli => MUSICAL_SYMBOL_COMBINING_KEMPLI,
            Balinese::MusicalSymbolCombiningJegogan => MUSICAL_SYMBOL_COMBINING_JEGOGAN,
            Balinese::MusicalSymbolCombiningKempulWithJegogan => MUSICAL_SYMBOL_COMBINING_KEMPUL_WITH_JEGOGAN,
            Balinese::MusicalSymbolCombiningKempliWithJegogan => MUSICAL_SYMBOL_COMBINING_KEMPLI_WITH_JEGOGAN,
            Balinese::MusicalSymbolCombiningBende => MUSICAL_SYMBOL_COMBINING_BENDE,
            Balinese::MusicalSymbolCombiningGong => MUSICAL_SYMBOL_COMBINING_GONG,
            Balinese::MusicalSymbolRightDashHandOpenDug => MUSICAL_SYMBOL_RIGHT_DASH_HAND_OPEN_DUG,
            Balinese::MusicalSymbolRightDashHandOpenDag => MUSICAL_SYMBOL_RIGHT_DASH_HAND_OPEN_DAG,
            Balinese::MusicalSymbolRightDashHandClosedTuk => MUSICAL_SYMBOL_RIGHT_DASH_HAND_CLOSED_TUK,
            Balinese::MusicalSymbolRightDashHandClosedTak => MUSICAL_SYMBOL_RIGHT_DASH_HAND_CLOSED_TAK,
            Balinese::MusicalSymbolLeftDashHandOpenPang => MUSICAL_SYMBOL_LEFT_DASH_HAND_OPEN_PANG,
            Balinese::MusicalSymbolLeftDashHandOpenPung => MUSICAL_SYMBOL_LEFT_DASH_HAND_OPEN_PUNG,
            Balinese::MusicalSymbolLeftDashHandClosedPlak => MUSICAL_SYMBOL_LEFT_DASH_HAND_CLOSED_PLAK,
            Balinese::MusicalSymbolLeftDashHandClosedPluk => MUSICAL_SYMBOL_LEFT_DASH_HAND_CLOSED_PLUK,
            Balinese::MusicalSymbolLeftDashHandOpenPing => MUSICAL_SYMBOL_LEFT_DASH_HAND_OPEN_PING,
        }
    }
}

impl std::convert::TryFrom<char> for Balinese {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            SIGN_ULU_RICEM => Ok(Balinese::SignUluRicem),
            SIGN_ULU_CANDRA => Ok(Balinese::SignUluCandra),
            SIGN_CECEK => Ok(Balinese::SignCecek),
            SIGN_SURANG => Ok(Balinese::SignSurang),
            SIGN_BISAH => Ok(Balinese::SignBisah),
            LETTER_AKARA => Ok(Balinese::LetterAkara),
            LETTER_AKARA_TEDUNG => Ok(Balinese::LetterAkaraTedung),
            LETTER_IKARA => Ok(Balinese::LetterIkara),
            LETTER_IKARA_TEDUNG => Ok(Balinese::LetterIkaraTedung),
            LETTER_UKARA => Ok(Balinese::LetterUkara),
            LETTER_UKARA_TEDUNG => Ok(Balinese::LetterUkaraTedung),
            LETTER_RA_REPA => Ok(Balinese::LetterRaRepa),
            LETTER_RA_REPA_TEDUNG => Ok(Balinese::LetterRaRepaTedung),
            LETTER_LA_LENGA => Ok(Balinese::LetterLaLenga),
            LETTER_LA_LENGA_TEDUNG => Ok(Balinese::LetterLaLengaTedung),
            LETTER_EKARA => Ok(Balinese::LetterEkara),
            LETTER_AIKARA => Ok(Balinese::LetterAikara),
            LETTER_OKARA => Ok(Balinese::LetterOkara),
            LETTER_OKARA_TEDUNG => Ok(Balinese::LetterOkaraTedung),
            LETTER_KA => Ok(Balinese::LetterKa),
            LETTER_KA_MAHAPRANA => Ok(Balinese::LetterKaMahaprana),
            LETTER_GA => Ok(Balinese::LetterGa),
            LETTER_GA_GORA => Ok(Balinese::LetterGaGora),
            LETTER_NGA => Ok(Balinese::LetterNga),
            LETTER_CA => Ok(Balinese::LetterCa),
            LETTER_CA_LACA => Ok(Balinese::LetterCaLaca),
            LETTER_JA => Ok(Balinese::LetterJa),
            LETTER_JA_JERA => Ok(Balinese::LetterJaJera),
            LETTER_NYA => Ok(Balinese::LetterNya),
            LETTER_TA_LATIK => Ok(Balinese::LetterTaLatik),
            LETTER_TA_MURDA_MAHAPRANA => Ok(Balinese::LetterTaMurdaMahaprana),
            LETTER_DA_MURDA_ALPAPRANA => Ok(Balinese::LetterDaMurdaAlpaprana),
            LETTER_DA_MURDA_MAHAPRANA => Ok(Balinese::LetterDaMurdaMahaprana),
            LETTER_NA_RAMBAT => Ok(Balinese::LetterNaRambat),
            LETTER_TA => Ok(Balinese::LetterTa),
            LETTER_TA_TAWA => Ok(Balinese::LetterTaTawa),
            LETTER_DA => Ok(Balinese::LetterDa),
            LETTER_DA_MADU => Ok(Balinese::LetterDaMadu),
            LETTER_NA => Ok(Balinese::LetterNa),
            LETTER_PA => Ok(Balinese::LetterPa),
            LETTER_PA_KAPAL => Ok(Balinese::LetterPaKapal),
            LETTER_BA => Ok(Balinese::LetterBa),
            LETTER_BA_KEMBANG => Ok(Balinese::LetterBaKembang),
            LETTER_MA => Ok(Balinese::LetterMa),
            LETTER_YA => Ok(Balinese::LetterYa),
            LETTER_RA => Ok(Balinese::LetterRa),
            LETTER_LA => Ok(Balinese::LetterLa),
            LETTER_WA => Ok(Balinese::LetterWa),
            LETTER_SA_SAGA => Ok(Balinese::LetterSaSaga),
            LETTER_SA_SAPA => Ok(Balinese::LetterSaSapa),
            LETTER_SA => Ok(Balinese::LetterSa),
            LETTER_HA => Ok(Balinese::LetterHa),
            SIGN_REREKAN => Ok(Balinese::SignRerekan),
            VOWEL_SIGN_TEDUNG => Ok(Balinese::VowelSignTedung),
            VOWEL_SIGN_ULU => Ok(Balinese::VowelSignUlu),
            VOWEL_SIGN_ULU_SARI => Ok(Balinese::VowelSignUluSari),
            VOWEL_SIGN_SUKU => Ok(Balinese::VowelSignSuku),
            VOWEL_SIGN_SUKU_ILUT => Ok(Balinese::VowelSignSukuIlut),
            VOWEL_SIGN_RA_REPA => Ok(Balinese::VowelSignRaRepa),
            VOWEL_SIGN_RA_REPA_TEDUNG => Ok(Balinese::VowelSignRaRepaTedung),
            VOWEL_SIGN_LA_LENGA => Ok(Balinese::VowelSignLaLenga),
            VOWEL_SIGN_LA_LENGA_TEDUNG => Ok(Balinese::VowelSignLaLengaTedung),
            VOWEL_SIGN_TALING => Ok(Balinese::VowelSignTaling),
            VOWEL_SIGN_TALING_REPA => Ok(Balinese::VowelSignTalingRepa),
            VOWEL_SIGN_TALING_TEDUNG => Ok(Balinese::VowelSignTalingTedung),
            VOWEL_SIGN_TALING_REPA_TEDUNG => Ok(Balinese::VowelSignTalingRepaTedung),
            VOWEL_SIGN_PEPET => Ok(Balinese::VowelSignPepet),
            VOWEL_SIGN_PEPET_TEDUNG => Ok(Balinese::VowelSignPepetTedung),
            ADEG_ADEG => Ok(Balinese::AdegAdeg),
            LETTER_KAF_SASAK => Ok(Balinese::LetterKafSasak),
            LETTER_KHOT_SASAK => Ok(Balinese::LetterKhotSasak),
            LETTER_TZIR_SASAK => Ok(Balinese::LetterTzirSasak),
            LETTER_EF_SASAK => Ok(Balinese::LetterEfSasak),
            LETTER_VE_SASAK => Ok(Balinese::LetterVeSasak),
            LETTER_ZAL_SASAK => Ok(Balinese::LetterZalSasak),
            LETTER_ASYURA_SASAK => Ok(Balinese::LetterAsyuraSasak),
            DIGIT_ZERO => Ok(Balinese::DigitZero),
            DIGIT_ONE => Ok(Balinese::DigitOne),
            DIGIT_TWO => Ok(Balinese::DigitTwo),
            DIGIT_THREE => Ok(Balinese::DigitThree),
            DIGIT_FOUR => Ok(Balinese::DigitFour),
            DIGIT_FIVE => Ok(Balinese::DigitFive),
            DIGIT_SIX => Ok(Balinese::DigitSix),
            DIGIT_SEVEN => Ok(Balinese::DigitSeven),
            DIGIT_EIGHT => Ok(Balinese::DigitEight),
            DIGIT_NINE => Ok(Balinese::DigitNine),
            PANTI => Ok(Balinese::Panti),
            PAMADA => Ok(Balinese::Pamada),
            WINDU => Ok(Balinese::Windu),
            CARIK_PAMUNGKAH => Ok(Balinese::CarikPamungkah),
            CARIK_SIKI => Ok(Balinese::CarikSiki),
            CARIK_PAREREN => Ok(Balinese::CarikPareren),
            PAMENENG => Ok(Balinese::Pameneng),
            MUSICAL_SYMBOL_DONG => Ok(Balinese::MusicalSymbolDong),
            MUSICAL_SYMBOL_DENG => Ok(Balinese::MusicalSymbolDeng),
            MUSICAL_SYMBOL_DUNG => Ok(Balinese::MusicalSymbolDung),
            MUSICAL_SYMBOL_DANG => Ok(Balinese::MusicalSymbolDang),
            MUSICAL_SYMBOL_DANG_SURANG => Ok(Balinese::MusicalSymbolDangSurang),
            MUSICAL_SYMBOL_DING => Ok(Balinese::MusicalSymbolDing),
            MUSICAL_SYMBOL_DAENG => Ok(Balinese::MusicalSymbolDaeng),
            MUSICAL_SYMBOL_DEUNG => Ok(Balinese::MusicalSymbolDeung),
            MUSICAL_SYMBOL_DAING => Ok(Balinese::MusicalSymbolDaing),
            MUSICAL_SYMBOL_DANG_GEDE => Ok(Balinese::MusicalSymbolDangGede),
            MUSICAL_SYMBOL_COMBINING_TEGEH => Ok(Balinese::MusicalSymbolCombiningTegeh),
            MUSICAL_SYMBOL_COMBINING_ENDEP => Ok(Balinese::MusicalSymbolCombiningEndep),
            MUSICAL_SYMBOL_COMBINING_KEMPUL => Ok(Balinese::MusicalSymbolCombiningKempul),
            MUSICAL_SYMBOL_COMBINING_KEMPLI => Ok(Balinese::MusicalSymbolCombiningKempli),
            MUSICAL_SYMBOL_COMBINING_JEGOGAN => Ok(Balinese::MusicalSymbolCombiningJegogan),
            MUSICAL_SYMBOL_COMBINING_KEMPUL_WITH_JEGOGAN => Ok(Balinese::MusicalSymbolCombiningKempulWithJegogan),
            MUSICAL_SYMBOL_COMBINING_KEMPLI_WITH_JEGOGAN => Ok(Balinese::MusicalSymbolCombiningKempliWithJegogan),
            MUSICAL_SYMBOL_COMBINING_BENDE => Ok(Balinese::MusicalSymbolCombiningBende),
            MUSICAL_SYMBOL_COMBINING_GONG => Ok(Balinese::MusicalSymbolCombiningGong),
            MUSICAL_SYMBOL_RIGHT_DASH_HAND_OPEN_DUG => Ok(Balinese::MusicalSymbolRightDashHandOpenDug),
            MUSICAL_SYMBOL_RIGHT_DASH_HAND_OPEN_DAG => Ok(Balinese::MusicalSymbolRightDashHandOpenDag),
            MUSICAL_SYMBOL_RIGHT_DASH_HAND_CLOSED_TUK => Ok(Balinese::MusicalSymbolRightDashHandClosedTuk),
            MUSICAL_SYMBOL_RIGHT_DASH_HAND_CLOSED_TAK => Ok(Balinese::MusicalSymbolRightDashHandClosedTak),
            MUSICAL_SYMBOL_LEFT_DASH_HAND_OPEN_PANG => Ok(Balinese::MusicalSymbolLeftDashHandOpenPang),
            MUSICAL_SYMBOL_LEFT_DASH_HAND_OPEN_PUNG => Ok(Balinese::MusicalSymbolLeftDashHandOpenPung),
            MUSICAL_SYMBOL_LEFT_DASH_HAND_CLOSED_PLAK => Ok(Balinese::MusicalSymbolLeftDashHandClosedPlak),
            MUSICAL_SYMBOL_LEFT_DASH_HAND_CLOSED_PLUK => Ok(Balinese::MusicalSymbolLeftDashHandClosedPluk),
            MUSICAL_SYMBOL_LEFT_DASH_HAND_OPEN_PING => Ok(Balinese::MusicalSymbolLeftDashHandOpenPing),
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

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Balinese::SignUluRicem => "balinese sign ulu ricem",
            Balinese::SignUluCandra => "balinese sign ulu candra",
            Balinese::SignCecek => "balinese sign cecek",
            Balinese::SignSurang => "balinese sign surang",
            Balinese::SignBisah => "balinese sign bisah",
            Balinese::LetterAkara => "balinese letter akara",
            Balinese::LetterAkaraTedung => "balinese letter akara tedung",
            Balinese::LetterIkara => "balinese letter ikara",
            Balinese::LetterIkaraTedung => "balinese letter ikara tedung",
            Balinese::LetterUkara => "balinese letter ukara",
            Balinese::LetterUkaraTedung => "balinese letter ukara tedung",
            Balinese::LetterRaRepa => "balinese letter ra repa",
            Balinese::LetterRaRepaTedung => "balinese letter ra repa tedung",
            Balinese::LetterLaLenga => "balinese letter la lenga",
            Balinese::LetterLaLengaTedung => "balinese letter la lenga tedung",
            Balinese::LetterEkara => "balinese letter ekara",
            Balinese::LetterAikara => "balinese letter aikara",
            Balinese::LetterOkara => "balinese letter okara",
            Balinese::LetterOkaraTedung => "balinese letter okara tedung",
            Balinese::LetterKa => "balinese letter ka",
            Balinese::LetterKaMahaprana => "balinese letter ka mahaprana",
            Balinese::LetterGa => "balinese letter ga",
            Balinese::LetterGaGora => "balinese letter ga gora",
            Balinese::LetterNga => "balinese letter nga",
            Balinese::LetterCa => "balinese letter ca",
            Balinese::LetterCaLaca => "balinese letter ca laca",
            Balinese::LetterJa => "balinese letter ja",
            Balinese::LetterJaJera => "balinese letter ja jera",
            Balinese::LetterNya => "balinese letter nya",
            Balinese::LetterTaLatik => "balinese letter ta latik",
            Balinese::LetterTaMurdaMahaprana => "balinese letter ta murda mahaprana",
            Balinese::LetterDaMurdaAlpaprana => "balinese letter da murda alpaprana",
            Balinese::LetterDaMurdaMahaprana => "balinese letter da murda mahaprana",
            Balinese::LetterNaRambat => "balinese letter na rambat",
            Balinese::LetterTa => "balinese letter ta",
            Balinese::LetterTaTawa => "balinese letter ta tawa",
            Balinese::LetterDa => "balinese letter da",
            Balinese::LetterDaMadu => "balinese letter da madu",
            Balinese::LetterNa => "balinese letter na",
            Balinese::LetterPa => "balinese letter pa",
            Balinese::LetterPaKapal => "balinese letter pa kapal",
            Balinese::LetterBa => "balinese letter ba",
            Balinese::LetterBaKembang => "balinese letter ba kembang",
            Balinese::LetterMa => "balinese letter ma",
            Balinese::LetterYa => "balinese letter ya",
            Balinese::LetterRa => "balinese letter ra",
            Balinese::LetterLa => "balinese letter la",
            Balinese::LetterWa => "balinese letter wa",
            Balinese::LetterSaSaga => "balinese letter sa saga",
            Balinese::LetterSaSapa => "balinese letter sa sapa",
            Balinese::LetterSa => "balinese letter sa",
            Balinese::LetterHa => "balinese letter ha",
            Balinese::SignRerekan => "balinese sign rerekan",
            Balinese::VowelSignTedung => "balinese vowel sign tedung",
            Balinese::VowelSignUlu => "balinese vowel sign ulu",
            Balinese::VowelSignUluSari => "balinese vowel sign ulu sari",
            Balinese::VowelSignSuku => "balinese vowel sign suku",
            Balinese::VowelSignSukuIlut => "balinese vowel sign suku ilut",
            Balinese::VowelSignRaRepa => "balinese vowel sign ra repa",
            Balinese::VowelSignRaRepaTedung => "balinese vowel sign ra repa tedung",
            Balinese::VowelSignLaLenga => "balinese vowel sign la lenga",
            Balinese::VowelSignLaLengaTedung => "balinese vowel sign la lenga tedung",
            Balinese::VowelSignTaling => "balinese vowel sign taling",
            Balinese::VowelSignTalingRepa => "balinese vowel sign taling repa",
            Balinese::VowelSignTalingTedung => "balinese vowel sign taling tedung",
            Balinese::VowelSignTalingRepaTedung => "balinese vowel sign taling repa tedung",
            Balinese::VowelSignPepet => "balinese vowel sign pepet",
            Balinese::VowelSignPepetTedung => "balinese vowel sign pepet tedung",
            Balinese::AdegAdeg => "balinese adeg adeg",
            Balinese::LetterKafSasak => "balinese letter kaf sasak",
            Balinese::LetterKhotSasak => "balinese letter khot sasak",
            Balinese::LetterTzirSasak => "balinese letter tzir sasak",
            Balinese::LetterEfSasak => "balinese letter ef sasak",
            Balinese::LetterVeSasak => "balinese letter ve sasak",
            Balinese::LetterZalSasak => "balinese letter zal sasak",
            Balinese::LetterAsyuraSasak => "balinese letter asyura sasak",
            Balinese::DigitZero => "balinese digit zero",
            Balinese::DigitOne => "balinese digit one",
            Balinese::DigitTwo => "balinese digit two",
            Balinese::DigitThree => "balinese digit three",
            Balinese::DigitFour => "balinese digit four",
            Balinese::DigitFive => "balinese digit five",
            Balinese::DigitSix => "balinese digit six",
            Balinese::DigitSeven => "balinese digit seven",
            Balinese::DigitEight => "balinese digit eight",
            Balinese::DigitNine => "balinese digit nine",
            Balinese::Panti => "balinese panti",
            Balinese::Pamada => "balinese pamada",
            Balinese::Windu => "balinese windu",
            Balinese::CarikPamungkah => "balinese carik pamungkah",
            Balinese::CarikSiki => "balinese carik siki",
            Balinese::CarikPareren => "balinese carik pareren",
            Balinese::Pameneng => "balinese pameneng",
            Balinese::MusicalSymbolDong => "balinese musical symbol dong",
            Balinese::MusicalSymbolDeng => "balinese musical symbol deng",
            Balinese::MusicalSymbolDung => "balinese musical symbol dung",
            Balinese::MusicalSymbolDang => "balinese musical symbol dang",
            Balinese::MusicalSymbolDangSurang => "balinese musical symbol dang surang",
            Balinese::MusicalSymbolDing => "balinese musical symbol ding",
            Balinese::MusicalSymbolDaeng => "balinese musical symbol daeng",
            Balinese::MusicalSymbolDeung => "balinese musical symbol deung",
            Balinese::MusicalSymbolDaing => "balinese musical symbol daing",
            Balinese::MusicalSymbolDangGede => "balinese musical symbol dang gede",
            Balinese::MusicalSymbolCombiningTegeh => "balinese musical symbol combining tegeh",
            Balinese::MusicalSymbolCombiningEndep => "balinese musical symbol combining endep",
            Balinese::MusicalSymbolCombiningKempul => "balinese musical symbol combining kempul",
            Balinese::MusicalSymbolCombiningKempli => "balinese musical symbol combining kempli",
            Balinese::MusicalSymbolCombiningJegogan => "balinese musical symbol combining jegogan",
            Balinese::MusicalSymbolCombiningKempulWithJegogan => "balinese musical symbol combining kempul with jegogan",
            Balinese::MusicalSymbolCombiningKempliWithJegogan => "balinese musical symbol combining kempli with jegogan",
            Balinese::MusicalSymbolCombiningBende => "balinese musical symbol combining bende",
            Balinese::MusicalSymbolCombiningGong => "balinese musical symbol combining gong",
            Balinese::MusicalSymbolRightDashHandOpenDug => "balinese musical symbol right-hand open dug",
            Balinese::MusicalSymbolRightDashHandOpenDag => "balinese musical symbol right-hand open dag",
            Balinese::MusicalSymbolRightDashHandClosedTuk => "balinese musical symbol right-hand closed tuk",
            Balinese::MusicalSymbolRightDashHandClosedTak => "balinese musical symbol right-hand closed tak",
            Balinese::MusicalSymbolLeftDashHandOpenPang => "balinese musical symbol left-hand open pang",
            Balinese::MusicalSymbolLeftDashHandOpenPung => "balinese musical symbol left-hand open pung",
            Balinese::MusicalSymbolLeftDashHandClosedPlak => "balinese musical symbol left-hand closed plak",
            Balinese::MusicalSymbolLeftDashHandClosedPluk => "balinese musical symbol left-hand closed pluk",
            Balinese::MusicalSymbolLeftDashHandOpenPing => "balinese musical symbol left-hand open ping",
        }
    }
}
