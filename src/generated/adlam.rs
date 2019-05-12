
/// An enum to represent all characters in the Adlam block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Adlam {
    /// \u{1e900}: '𞤀'
    CapitalLetterAlif,
    /// \u{1e901}: '𞤁'
    CapitalLetterDaali,
    /// \u{1e902}: '𞤂'
    CapitalLetterLaam,
    /// \u{1e903}: '𞤃'
    CapitalLetterMiim,
    /// \u{1e904}: '𞤄'
    CapitalLetterBa,
    /// \u{1e905}: '𞤅'
    CapitalLetterSinnyiiyhe,
    /// \u{1e906}: '𞤆'
    CapitalLetterPe,
    /// \u{1e907}: '𞤇'
    CapitalLetterBhe,
    /// \u{1e908}: '𞤈'
    CapitalLetterRa,
    /// \u{1e909}: '𞤉'
    CapitalLetterE,
    /// \u{1e90a}: '𞤊'
    CapitalLetterFa,
    /// \u{1e90b}: '𞤋'
    CapitalLetterI,
    /// \u{1e90c}: '𞤌'
    CapitalLetterO,
    /// \u{1e90d}: '𞤍'
    CapitalLetterDha,
    /// \u{1e90e}: '𞤎'
    CapitalLetterYhe,
    /// \u{1e90f}: '𞤏'
    CapitalLetterWaw,
    /// \u{1e910}: '𞤐'
    CapitalLetterNun,
    /// \u{1e911}: '𞤑'
    CapitalLetterKaf,
    /// \u{1e912}: '𞤒'
    CapitalLetterYa,
    /// \u{1e913}: '𞤓'
    CapitalLetterU,
    /// \u{1e914}: '𞤔'
    CapitalLetterJiim,
    /// \u{1e915}: '𞤕'
    CapitalLetterChi,
    /// \u{1e916}: '𞤖'
    CapitalLetterHa,
    /// \u{1e917}: '𞤗'
    CapitalLetterQaaf,
    /// \u{1e918}: '𞤘'
    CapitalLetterGa,
    /// \u{1e919}: '𞤙'
    CapitalLetterNya,
    /// \u{1e91a}: '𞤚'
    CapitalLetterTu,
    /// \u{1e91b}: '𞤛'
    CapitalLetterNha,
    /// \u{1e91c}: '𞤜'
    CapitalLetterVa,
    /// \u{1e91d}: '𞤝'
    CapitalLetterKha,
    /// \u{1e91e}: '𞤞'
    CapitalLetterGbe,
    /// \u{1e91f}: '𞤟'
    CapitalLetterZal,
    /// \u{1e920}: '𞤠'
    CapitalLetterKpo,
    /// \u{1e921}: '𞤡'
    CapitalLetterSha,
    /// \u{1e922}: '𞤢'
    SmallLetterAlif,
    /// \u{1e923}: '𞤣'
    SmallLetterDaali,
    /// \u{1e924}: '𞤤'
    SmallLetterLaam,
    /// \u{1e925}: '𞤥'
    SmallLetterMiim,
    /// \u{1e926}: '𞤦'
    SmallLetterBa,
    /// \u{1e927}: '𞤧'
    SmallLetterSinnyiiyhe,
    /// \u{1e928}: '𞤨'
    SmallLetterPe,
    /// \u{1e929}: '𞤩'
    SmallLetterBhe,
    /// \u{1e92a}: '𞤪'
    SmallLetterRa,
    /// \u{1e92b}: '𞤫'
    SmallLetterE,
    /// \u{1e92c}: '𞤬'
    SmallLetterFa,
    /// \u{1e92d}: '𞤭'
    SmallLetterI,
    /// \u{1e92e}: '𞤮'
    SmallLetterO,
    /// \u{1e92f}: '𞤯'
    SmallLetterDha,
    /// \u{1e930}: '𞤰'
    SmallLetterYhe,
    /// \u{1e931}: '𞤱'
    SmallLetterWaw,
    /// \u{1e932}: '𞤲'
    SmallLetterNun,
    /// \u{1e933}: '𞤳'
    SmallLetterKaf,
    /// \u{1e934}: '𞤴'
    SmallLetterYa,
    /// \u{1e935}: '𞤵'
    SmallLetterU,
    /// \u{1e936}: '𞤶'
    SmallLetterJiim,
    /// \u{1e937}: '𞤷'
    SmallLetterChi,
    /// \u{1e938}: '𞤸'
    SmallLetterHa,
    /// \u{1e939}: '𞤹'
    SmallLetterQaaf,
    /// \u{1e93a}: '𞤺'
    SmallLetterGa,
    /// \u{1e93b}: '𞤻'
    SmallLetterNya,
    /// \u{1e93c}: '𞤼'
    SmallLetterTu,
    /// \u{1e93d}: '𞤽'
    SmallLetterNha,
    /// \u{1e93e}: '𞤾'
    SmallLetterVa,
    /// \u{1e93f}: '𞤿'
    SmallLetterKha,
    /// \u{1e940}: '𞥀'
    SmallLetterGbe,
    /// \u{1e941}: '𞥁'
    SmallLetterZal,
    /// \u{1e942}: '𞥂'
    SmallLetterKpo,
    /// \u{1e943}: '𞥃'
    SmallLetterSha,
    /// \u{1e944}: '𞥄'
    AlifLengthener,
    /// \u{1e945}: '𞥅'
    VowelLengthener,
    /// \u{1e946}: '𞥆'
    GeminationMark,
    /// \u{1e947}: '𞥇'
    Hamza,
    /// \u{1e948}: '𞥈'
    ConsonantModifier,
    /// \u{1e949}: '𞥉'
    GeminateConsonantModifier,
    /// \u{1e94a}: '𞥊'
    Nukta,
    /// \u{1e94b}: '𞥋'
    NasalizationMark,
    /// \u{1e950}: '𞥐'
    DigitZero,
    /// \u{1e951}: '𞥑'
    DigitOne,
    /// \u{1e952}: '𞥒'
    DigitTwo,
    /// \u{1e953}: '𞥓'
    DigitThree,
    /// \u{1e954}: '𞥔'
    DigitFour,
    /// \u{1e955}: '𞥕'
    DigitFive,
    /// \u{1e956}: '𞥖'
    DigitSix,
    /// \u{1e957}: '𞥗'
    DigitSeven,
    /// \u{1e958}: '𞥘'
    DigitEight,
    /// \u{1e959}: '𞥙'
    DigitNine,
    /// \u{1e95e}: '𞥞'
    InitialExclamationMark,
}

impl Into<char> for Adlam {
    fn into(self) -> char {
        match self {
            Adlam::CapitalLetterAlif => '𞤀',
            Adlam::CapitalLetterDaali => '𞤁',
            Adlam::CapitalLetterLaam => '𞤂',
            Adlam::CapitalLetterMiim => '𞤃',
            Adlam::CapitalLetterBa => '𞤄',
            Adlam::CapitalLetterSinnyiiyhe => '𞤅',
            Adlam::CapitalLetterPe => '𞤆',
            Adlam::CapitalLetterBhe => '𞤇',
            Adlam::CapitalLetterRa => '𞤈',
            Adlam::CapitalLetterE => '𞤉',
            Adlam::CapitalLetterFa => '𞤊',
            Adlam::CapitalLetterI => '𞤋',
            Adlam::CapitalLetterO => '𞤌',
            Adlam::CapitalLetterDha => '𞤍',
            Adlam::CapitalLetterYhe => '𞤎',
            Adlam::CapitalLetterWaw => '𞤏',
            Adlam::CapitalLetterNun => '𞤐',
            Adlam::CapitalLetterKaf => '𞤑',
            Adlam::CapitalLetterYa => '𞤒',
            Adlam::CapitalLetterU => '𞤓',
            Adlam::CapitalLetterJiim => '𞤔',
            Adlam::CapitalLetterChi => '𞤕',
            Adlam::CapitalLetterHa => '𞤖',
            Adlam::CapitalLetterQaaf => '𞤗',
            Adlam::CapitalLetterGa => '𞤘',
            Adlam::CapitalLetterNya => '𞤙',
            Adlam::CapitalLetterTu => '𞤚',
            Adlam::CapitalLetterNha => '𞤛',
            Adlam::CapitalLetterVa => '𞤜',
            Adlam::CapitalLetterKha => '𞤝',
            Adlam::CapitalLetterGbe => '𞤞',
            Adlam::CapitalLetterZal => '𞤟',
            Adlam::CapitalLetterKpo => '𞤠',
            Adlam::CapitalLetterSha => '𞤡',
            Adlam::SmallLetterAlif => '𞤢',
            Adlam::SmallLetterDaali => '𞤣',
            Adlam::SmallLetterLaam => '𞤤',
            Adlam::SmallLetterMiim => '𞤥',
            Adlam::SmallLetterBa => '𞤦',
            Adlam::SmallLetterSinnyiiyhe => '𞤧',
            Adlam::SmallLetterPe => '𞤨',
            Adlam::SmallLetterBhe => '𞤩',
            Adlam::SmallLetterRa => '𞤪',
            Adlam::SmallLetterE => '𞤫',
            Adlam::SmallLetterFa => '𞤬',
            Adlam::SmallLetterI => '𞤭',
            Adlam::SmallLetterO => '𞤮',
            Adlam::SmallLetterDha => '𞤯',
            Adlam::SmallLetterYhe => '𞤰',
            Adlam::SmallLetterWaw => '𞤱',
            Adlam::SmallLetterNun => '𞤲',
            Adlam::SmallLetterKaf => '𞤳',
            Adlam::SmallLetterYa => '𞤴',
            Adlam::SmallLetterU => '𞤵',
            Adlam::SmallLetterJiim => '𞤶',
            Adlam::SmallLetterChi => '𞤷',
            Adlam::SmallLetterHa => '𞤸',
            Adlam::SmallLetterQaaf => '𞤹',
            Adlam::SmallLetterGa => '𞤺',
            Adlam::SmallLetterNya => '𞤻',
            Adlam::SmallLetterTu => '𞤼',
            Adlam::SmallLetterNha => '𞤽',
            Adlam::SmallLetterVa => '𞤾',
            Adlam::SmallLetterKha => '𞤿',
            Adlam::SmallLetterGbe => '𞥀',
            Adlam::SmallLetterZal => '𞥁',
            Adlam::SmallLetterKpo => '𞥂',
            Adlam::SmallLetterSha => '𞥃',
            Adlam::AlifLengthener => '𞥄',
            Adlam::VowelLengthener => '𞥅',
            Adlam::GeminationMark => '𞥆',
            Adlam::Hamza => '𞥇',
            Adlam::ConsonantModifier => '𞥈',
            Adlam::GeminateConsonantModifier => '𞥉',
            Adlam::Nukta => '𞥊',
            Adlam::NasalizationMark => '𞥋',
            Adlam::DigitZero => '𞥐',
            Adlam::DigitOne => '𞥑',
            Adlam::DigitTwo => '𞥒',
            Adlam::DigitThree => '𞥓',
            Adlam::DigitFour => '𞥔',
            Adlam::DigitFive => '𞥕',
            Adlam::DigitSix => '𞥖',
            Adlam::DigitSeven => '𞥗',
            Adlam::DigitEight => '𞥘',
            Adlam::DigitNine => '𞥙',
            Adlam::InitialExclamationMark => '𞥞',
        }
    }
}

impl std::convert::TryFrom<char> for Adlam {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𞤀' => Ok(Adlam::CapitalLetterAlif),
            '𞤁' => Ok(Adlam::CapitalLetterDaali),
            '𞤂' => Ok(Adlam::CapitalLetterLaam),
            '𞤃' => Ok(Adlam::CapitalLetterMiim),
            '𞤄' => Ok(Adlam::CapitalLetterBa),
            '𞤅' => Ok(Adlam::CapitalLetterSinnyiiyhe),
            '𞤆' => Ok(Adlam::CapitalLetterPe),
            '𞤇' => Ok(Adlam::CapitalLetterBhe),
            '𞤈' => Ok(Adlam::CapitalLetterRa),
            '𞤉' => Ok(Adlam::CapitalLetterE),
            '𞤊' => Ok(Adlam::CapitalLetterFa),
            '𞤋' => Ok(Adlam::CapitalLetterI),
            '𞤌' => Ok(Adlam::CapitalLetterO),
            '𞤍' => Ok(Adlam::CapitalLetterDha),
            '𞤎' => Ok(Adlam::CapitalLetterYhe),
            '𞤏' => Ok(Adlam::CapitalLetterWaw),
            '𞤐' => Ok(Adlam::CapitalLetterNun),
            '𞤑' => Ok(Adlam::CapitalLetterKaf),
            '𞤒' => Ok(Adlam::CapitalLetterYa),
            '𞤓' => Ok(Adlam::CapitalLetterU),
            '𞤔' => Ok(Adlam::CapitalLetterJiim),
            '𞤕' => Ok(Adlam::CapitalLetterChi),
            '𞤖' => Ok(Adlam::CapitalLetterHa),
            '𞤗' => Ok(Adlam::CapitalLetterQaaf),
            '𞤘' => Ok(Adlam::CapitalLetterGa),
            '𞤙' => Ok(Adlam::CapitalLetterNya),
            '𞤚' => Ok(Adlam::CapitalLetterTu),
            '𞤛' => Ok(Adlam::CapitalLetterNha),
            '𞤜' => Ok(Adlam::CapitalLetterVa),
            '𞤝' => Ok(Adlam::CapitalLetterKha),
            '𞤞' => Ok(Adlam::CapitalLetterGbe),
            '𞤟' => Ok(Adlam::CapitalLetterZal),
            '𞤠' => Ok(Adlam::CapitalLetterKpo),
            '𞤡' => Ok(Adlam::CapitalLetterSha),
            '𞤢' => Ok(Adlam::SmallLetterAlif),
            '𞤣' => Ok(Adlam::SmallLetterDaali),
            '𞤤' => Ok(Adlam::SmallLetterLaam),
            '𞤥' => Ok(Adlam::SmallLetterMiim),
            '𞤦' => Ok(Adlam::SmallLetterBa),
            '𞤧' => Ok(Adlam::SmallLetterSinnyiiyhe),
            '𞤨' => Ok(Adlam::SmallLetterPe),
            '𞤩' => Ok(Adlam::SmallLetterBhe),
            '𞤪' => Ok(Adlam::SmallLetterRa),
            '𞤫' => Ok(Adlam::SmallLetterE),
            '𞤬' => Ok(Adlam::SmallLetterFa),
            '𞤭' => Ok(Adlam::SmallLetterI),
            '𞤮' => Ok(Adlam::SmallLetterO),
            '𞤯' => Ok(Adlam::SmallLetterDha),
            '𞤰' => Ok(Adlam::SmallLetterYhe),
            '𞤱' => Ok(Adlam::SmallLetterWaw),
            '𞤲' => Ok(Adlam::SmallLetterNun),
            '𞤳' => Ok(Adlam::SmallLetterKaf),
            '𞤴' => Ok(Adlam::SmallLetterYa),
            '𞤵' => Ok(Adlam::SmallLetterU),
            '𞤶' => Ok(Adlam::SmallLetterJiim),
            '𞤷' => Ok(Adlam::SmallLetterChi),
            '𞤸' => Ok(Adlam::SmallLetterHa),
            '𞤹' => Ok(Adlam::SmallLetterQaaf),
            '𞤺' => Ok(Adlam::SmallLetterGa),
            '𞤻' => Ok(Adlam::SmallLetterNya),
            '𞤼' => Ok(Adlam::SmallLetterTu),
            '𞤽' => Ok(Adlam::SmallLetterNha),
            '𞤾' => Ok(Adlam::SmallLetterVa),
            '𞤿' => Ok(Adlam::SmallLetterKha),
            '𞥀' => Ok(Adlam::SmallLetterGbe),
            '𞥁' => Ok(Adlam::SmallLetterZal),
            '𞥂' => Ok(Adlam::SmallLetterKpo),
            '𞥃' => Ok(Adlam::SmallLetterSha),
            '𞥄' => Ok(Adlam::AlifLengthener),
            '𞥅' => Ok(Adlam::VowelLengthener),
            '𞥆' => Ok(Adlam::GeminationMark),
            '𞥇' => Ok(Adlam::Hamza),
            '𞥈' => Ok(Adlam::ConsonantModifier),
            '𞥉' => Ok(Adlam::GeminateConsonantModifier),
            '𞥊' => Ok(Adlam::Nukta),
            '𞥋' => Ok(Adlam::NasalizationMark),
            '𞥐' => Ok(Adlam::DigitZero),
            '𞥑' => Ok(Adlam::DigitOne),
            '𞥒' => Ok(Adlam::DigitTwo),
            '𞥓' => Ok(Adlam::DigitThree),
            '𞥔' => Ok(Adlam::DigitFour),
            '𞥕' => Ok(Adlam::DigitFive),
            '𞥖' => Ok(Adlam::DigitSix),
            '𞥗' => Ok(Adlam::DigitSeven),
            '𞥘' => Ok(Adlam::DigitEight),
            '𞥙' => Ok(Adlam::DigitNine),
            '𞥞' => Ok(Adlam::InitialExclamationMark),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Adlam {
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

impl std::convert::TryFrom<u32> for Adlam {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Adlam {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Adlam {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Adlam::CapitalLetterAlif
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Adlam::CapitalLetterAlif => "adlam capital letter alif",
            Adlam::CapitalLetterDaali => "adlam capital letter daali",
            Adlam::CapitalLetterLaam => "adlam capital letter laam",
            Adlam::CapitalLetterMiim => "adlam capital letter miim",
            Adlam::CapitalLetterBa => "adlam capital letter ba",
            Adlam::CapitalLetterSinnyiiyhe => "adlam capital letter sinnyiiyhe",
            Adlam::CapitalLetterPe => "adlam capital letter pe",
            Adlam::CapitalLetterBhe => "adlam capital letter bhe",
            Adlam::CapitalLetterRa => "adlam capital letter ra",
            Adlam::CapitalLetterE => "adlam capital letter e",
            Adlam::CapitalLetterFa => "adlam capital letter fa",
            Adlam::CapitalLetterI => "adlam capital letter i",
            Adlam::CapitalLetterO => "adlam capital letter o",
            Adlam::CapitalLetterDha => "adlam capital letter dha",
            Adlam::CapitalLetterYhe => "adlam capital letter yhe",
            Adlam::CapitalLetterWaw => "adlam capital letter waw",
            Adlam::CapitalLetterNun => "adlam capital letter nun",
            Adlam::CapitalLetterKaf => "adlam capital letter kaf",
            Adlam::CapitalLetterYa => "adlam capital letter ya",
            Adlam::CapitalLetterU => "adlam capital letter u",
            Adlam::CapitalLetterJiim => "adlam capital letter jiim",
            Adlam::CapitalLetterChi => "adlam capital letter chi",
            Adlam::CapitalLetterHa => "adlam capital letter ha",
            Adlam::CapitalLetterQaaf => "adlam capital letter qaaf",
            Adlam::CapitalLetterGa => "adlam capital letter ga",
            Adlam::CapitalLetterNya => "adlam capital letter nya",
            Adlam::CapitalLetterTu => "adlam capital letter tu",
            Adlam::CapitalLetterNha => "adlam capital letter nha",
            Adlam::CapitalLetterVa => "adlam capital letter va",
            Adlam::CapitalLetterKha => "adlam capital letter kha",
            Adlam::CapitalLetterGbe => "adlam capital letter gbe",
            Adlam::CapitalLetterZal => "adlam capital letter zal",
            Adlam::CapitalLetterKpo => "adlam capital letter kpo",
            Adlam::CapitalLetterSha => "adlam capital letter sha",
            Adlam::SmallLetterAlif => "adlam small letter alif",
            Adlam::SmallLetterDaali => "adlam small letter daali",
            Adlam::SmallLetterLaam => "adlam small letter laam",
            Adlam::SmallLetterMiim => "adlam small letter miim",
            Adlam::SmallLetterBa => "adlam small letter ba",
            Adlam::SmallLetterSinnyiiyhe => "adlam small letter sinnyiiyhe",
            Adlam::SmallLetterPe => "adlam small letter pe",
            Adlam::SmallLetterBhe => "adlam small letter bhe",
            Adlam::SmallLetterRa => "adlam small letter ra",
            Adlam::SmallLetterE => "adlam small letter e",
            Adlam::SmallLetterFa => "adlam small letter fa",
            Adlam::SmallLetterI => "adlam small letter i",
            Adlam::SmallLetterO => "adlam small letter o",
            Adlam::SmallLetterDha => "adlam small letter dha",
            Adlam::SmallLetterYhe => "adlam small letter yhe",
            Adlam::SmallLetterWaw => "adlam small letter waw",
            Adlam::SmallLetterNun => "adlam small letter nun",
            Adlam::SmallLetterKaf => "adlam small letter kaf",
            Adlam::SmallLetterYa => "adlam small letter ya",
            Adlam::SmallLetterU => "adlam small letter u",
            Adlam::SmallLetterJiim => "adlam small letter jiim",
            Adlam::SmallLetterChi => "adlam small letter chi",
            Adlam::SmallLetterHa => "adlam small letter ha",
            Adlam::SmallLetterQaaf => "adlam small letter qaaf",
            Adlam::SmallLetterGa => "adlam small letter ga",
            Adlam::SmallLetterNya => "adlam small letter nya",
            Adlam::SmallLetterTu => "adlam small letter tu",
            Adlam::SmallLetterNha => "adlam small letter nha",
            Adlam::SmallLetterVa => "adlam small letter va",
            Adlam::SmallLetterKha => "adlam small letter kha",
            Adlam::SmallLetterGbe => "adlam small letter gbe",
            Adlam::SmallLetterZal => "adlam small letter zal",
            Adlam::SmallLetterKpo => "adlam small letter kpo",
            Adlam::SmallLetterSha => "adlam small letter sha",
            Adlam::AlifLengthener => "adlam alif lengthener",
            Adlam::VowelLengthener => "adlam vowel lengthener",
            Adlam::GeminationMark => "adlam gemination mark",
            Adlam::Hamza => "adlam hamza",
            Adlam::ConsonantModifier => "adlam consonant modifier",
            Adlam::GeminateConsonantModifier => "adlam geminate consonant modifier",
            Adlam::Nukta => "adlam nukta",
            Adlam::NasalizationMark => "adlam nasalization mark",
            Adlam::DigitZero => "adlam digit zero",
            Adlam::DigitOne => "adlam digit one",
            Adlam::DigitTwo => "adlam digit two",
            Adlam::DigitThree => "adlam digit three",
            Adlam::DigitFour => "adlam digit four",
            Adlam::DigitFive => "adlam digit five",
            Adlam::DigitSix => "adlam digit six",
            Adlam::DigitSeven => "adlam digit seven",
            Adlam::DigitEight => "adlam digit eight",
            Adlam::DigitNine => "adlam digit nine",
            Adlam::InitialExclamationMark => "adlam initial exclamation mark",
        }
    }
}
