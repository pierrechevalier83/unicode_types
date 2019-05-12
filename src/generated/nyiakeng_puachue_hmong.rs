
/// An enum to represent all characters in the NyiakengPuachueHmong block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum NyiakengPuachueHmong {
    /// \u{1e100}: '𞄀'
    LetterMa,
    /// \u{1e101}: '𞄁'
    LetterTsa,
    /// \u{1e102}: '𞄂'
    LetterNta,
    /// \u{1e103}: '𞄃'
    LetterTa,
    /// \u{1e104}: '𞄄'
    LetterHa,
    /// \u{1e105}: '𞄅'
    LetterNa,
    /// \u{1e106}: '𞄆'
    LetterXa,
    /// \u{1e107}: '𞄇'
    LetterNka,
    /// \u{1e108}: '𞄈'
    LetterCa,
    /// \u{1e109}: '𞄉'
    LetterLa,
    /// \u{1e10a}: '𞄊'
    LetterSa,
    /// \u{1e10b}: '𞄋'
    LetterZa,
    /// \u{1e10c}: '𞄌'
    LetterNca,
    /// \u{1e10d}: '𞄍'
    LetterNtsa,
    /// \u{1e10e}: '𞄎'
    LetterKa,
    /// \u{1e10f}: '𞄏'
    LetterDa,
    /// \u{1e110}: '𞄐'
    LetterNya,
    /// \u{1e111}: '𞄑'
    LetterNra,
    /// \u{1e112}: '𞄒'
    LetterVa,
    /// \u{1e113}: '𞄓'
    LetterNtxa,
    /// \u{1e114}: '𞄔'
    LetterTxa,
    /// \u{1e115}: '𞄕'
    LetterFa,
    /// \u{1e116}: '𞄖'
    LetterRa,
    /// \u{1e117}: '𞄗'
    LetterQa,
    /// \u{1e118}: '𞄘'
    LetterYa,
    /// \u{1e119}: '𞄙'
    LetterNqa,
    /// \u{1e11a}: '𞄚'
    LetterPa,
    /// \u{1e11b}: '𞄛'
    LetterXya,
    /// \u{1e11c}: '𞄜'
    LetterNpa,
    /// \u{1e11d}: '𞄝'
    LetterDla,
    /// \u{1e11e}: '𞄞'
    LetterNpla,
    /// \u{1e11f}: '𞄟'
    LetterHah,
    /// \u{1e120}: '𞄠'
    LetterMla,
    /// \u{1e121}: '𞄡'
    LetterPla,
    /// \u{1e122}: '𞄢'
    LetterGa,
    /// \u{1e123}: '𞄣'
    LetterRra,
    /// \u{1e124}: '𞄤'
    LetterA,
    /// \u{1e125}: '𞄥'
    LetterAa,
    /// \u{1e126}: '𞄦'
    LetterI,
    /// \u{1e127}: '𞄧'
    LetterU,
    /// \u{1e128}: '𞄨'
    LetterO,
    /// \u{1e129}: '𞄩'
    LetterOo,
    /// \u{1e12a}: '𞄪'
    LetterE,
    /// \u{1e12b}: '𞄫'
    LetterEe,
    /// \u{1e12c}: '𞄬'
    LetterW,
    /// \u{1e130}: '𞄰'
    ToneDashB,
    /// \u{1e131}: '𞄱'
    ToneDashM,
    /// \u{1e132}: '𞄲'
    ToneDashJ,
    /// \u{1e133}: '𞄳'
    ToneDashV,
    /// \u{1e134}: '𞄴'
    ToneDashS,
    /// \u{1e135}: '𞄵'
    ToneDashG,
    /// \u{1e136}: '𞄶'
    ToneDashD,
    /// \u{1e137}: '𞄷'
    SignForPerson,
    /// \u{1e138}: '𞄸'
    SignForThing,
    /// \u{1e139}: '𞄹'
    SignForLocation,
    /// \u{1e13a}: '𞄺'
    SignForAnimal,
    /// \u{1e13b}: '𞄻'
    SignForInvertebrate,
    /// \u{1e13c}: '𞄼'
    SignXwXw,
    /// \u{1e13d}: '𞄽'
    SyllableLengthener,
    /// \u{1e140}: '𞅀'
    DigitZero,
    /// \u{1e141}: '𞅁'
    DigitOne,
    /// \u{1e142}: '𞅂'
    DigitTwo,
    /// \u{1e143}: '𞅃'
    DigitThree,
    /// \u{1e144}: '𞅄'
    DigitFour,
    /// \u{1e145}: '𞅅'
    DigitFive,
    /// \u{1e146}: '𞅆'
    DigitSix,
    /// \u{1e147}: '𞅇'
    DigitSeven,
    /// \u{1e148}: '𞅈'
    DigitEight,
    /// \u{1e149}: '𞅉'
    DigitNine,
    /// \u{1e14e}: '𞅎'
    LogogramNyaj,
}

impl Into<char> for NyiakengPuachueHmong {
    fn into(self) -> char {
        match self {
            NyiakengPuachueHmong::LetterMa => '𞄀',
            NyiakengPuachueHmong::LetterTsa => '𞄁',
            NyiakengPuachueHmong::LetterNta => '𞄂',
            NyiakengPuachueHmong::LetterTa => '𞄃',
            NyiakengPuachueHmong::LetterHa => '𞄄',
            NyiakengPuachueHmong::LetterNa => '𞄅',
            NyiakengPuachueHmong::LetterXa => '𞄆',
            NyiakengPuachueHmong::LetterNka => '𞄇',
            NyiakengPuachueHmong::LetterCa => '𞄈',
            NyiakengPuachueHmong::LetterLa => '𞄉',
            NyiakengPuachueHmong::LetterSa => '𞄊',
            NyiakengPuachueHmong::LetterZa => '𞄋',
            NyiakengPuachueHmong::LetterNca => '𞄌',
            NyiakengPuachueHmong::LetterNtsa => '𞄍',
            NyiakengPuachueHmong::LetterKa => '𞄎',
            NyiakengPuachueHmong::LetterDa => '𞄏',
            NyiakengPuachueHmong::LetterNya => '𞄐',
            NyiakengPuachueHmong::LetterNra => '𞄑',
            NyiakengPuachueHmong::LetterVa => '𞄒',
            NyiakengPuachueHmong::LetterNtxa => '𞄓',
            NyiakengPuachueHmong::LetterTxa => '𞄔',
            NyiakengPuachueHmong::LetterFa => '𞄕',
            NyiakengPuachueHmong::LetterRa => '𞄖',
            NyiakengPuachueHmong::LetterQa => '𞄗',
            NyiakengPuachueHmong::LetterYa => '𞄘',
            NyiakengPuachueHmong::LetterNqa => '𞄙',
            NyiakengPuachueHmong::LetterPa => '𞄚',
            NyiakengPuachueHmong::LetterXya => '𞄛',
            NyiakengPuachueHmong::LetterNpa => '𞄜',
            NyiakengPuachueHmong::LetterDla => '𞄝',
            NyiakengPuachueHmong::LetterNpla => '𞄞',
            NyiakengPuachueHmong::LetterHah => '𞄟',
            NyiakengPuachueHmong::LetterMla => '𞄠',
            NyiakengPuachueHmong::LetterPla => '𞄡',
            NyiakengPuachueHmong::LetterGa => '𞄢',
            NyiakengPuachueHmong::LetterRra => '𞄣',
            NyiakengPuachueHmong::LetterA => '𞄤',
            NyiakengPuachueHmong::LetterAa => '𞄥',
            NyiakengPuachueHmong::LetterI => '𞄦',
            NyiakengPuachueHmong::LetterU => '𞄧',
            NyiakengPuachueHmong::LetterO => '𞄨',
            NyiakengPuachueHmong::LetterOo => '𞄩',
            NyiakengPuachueHmong::LetterE => '𞄪',
            NyiakengPuachueHmong::LetterEe => '𞄫',
            NyiakengPuachueHmong::LetterW => '𞄬',
            NyiakengPuachueHmong::ToneDashB => '𞄰',
            NyiakengPuachueHmong::ToneDashM => '𞄱',
            NyiakengPuachueHmong::ToneDashJ => '𞄲',
            NyiakengPuachueHmong::ToneDashV => '𞄳',
            NyiakengPuachueHmong::ToneDashS => '𞄴',
            NyiakengPuachueHmong::ToneDashG => '𞄵',
            NyiakengPuachueHmong::ToneDashD => '𞄶',
            NyiakengPuachueHmong::SignForPerson => '𞄷',
            NyiakengPuachueHmong::SignForThing => '𞄸',
            NyiakengPuachueHmong::SignForLocation => '𞄹',
            NyiakengPuachueHmong::SignForAnimal => '𞄺',
            NyiakengPuachueHmong::SignForInvertebrate => '𞄻',
            NyiakengPuachueHmong::SignXwXw => '𞄼',
            NyiakengPuachueHmong::SyllableLengthener => '𞄽',
            NyiakengPuachueHmong::DigitZero => '𞅀',
            NyiakengPuachueHmong::DigitOne => '𞅁',
            NyiakengPuachueHmong::DigitTwo => '𞅂',
            NyiakengPuachueHmong::DigitThree => '𞅃',
            NyiakengPuachueHmong::DigitFour => '𞅄',
            NyiakengPuachueHmong::DigitFive => '𞅅',
            NyiakengPuachueHmong::DigitSix => '𞅆',
            NyiakengPuachueHmong::DigitSeven => '𞅇',
            NyiakengPuachueHmong::DigitEight => '𞅈',
            NyiakengPuachueHmong::DigitNine => '𞅉',
            NyiakengPuachueHmong::LogogramNyaj => '𞅎',
        }
    }
}

impl std::convert::TryFrom<char> for NyiakengPuachueHmong {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𞄀' => Ok(NyiakengPuachueHmong::LetterMa),
            '𞄁' => Ok(NyiakengPuachueHmong::LetterTsa),
            '𞄂' => Ok(NyiakengPuachueHmong::LetterNta),
            '𞄃' => Ok(NyiakengPuachueHmong::LetterTa),
            '𞄄' => Ok(NyiakengPuachueHmong::LetterHa),
            '𞄅' => Ok(NyiakengPuachueHmong::LetterNa),
            '𞄆' => Ok(NyiakengPuachueHmong::LetterXa),
            '𞄇' => Ok(NyiakengPuachueHmong::LetterNka),
            '𞄈' => Ok(NyiakengPuachueHmong::LetterCa),
            '𞄉' => Ok(NyiakengPuachueHmong::LetterLa),
            '𞄊' => Ok(NyiakengPuachueHmong::LetterSa),
            '𞄋' => Ok(NyiakengPuachueHmong::LetterZa),
            '𞄌' => Ok(NyiakengPuachueHmong::LetterNca),
            '𞄍' => Ok(NyiakengPuachueHmong::LetterNtsa),
            '𞄎' => Ok(NyiakengPuachueHmong::LetterKa),
            '𞄏' => Ok(NyiakengPuachueHmong::LetterDa),
            '𞄐' => Ok(NyiakengPuachueHmong::LetterNya),
            '𞄑' => Ok(NyiakengPuachueHmong::LetterNra),
            '𞄒' => Ok(NyiakengPuachueHmong::LetterVa),
            '𞄓' => Ok(NyiakengPuachueHmong::LetterNtxa),
            '𞄔' => Ok(NyiakengPuachueHmong::LetterTxa),
            '𞄕' => Ok(NyiakengPuachueHmong::LetterFa),
            '𞄖' => Ok(NyiakengPuachueHmong::LetterRa),
            '𞄗' => Ok(NyiakengPuachueHmong::LetterQa),
            '𞄘' => Ok(NyiakengPuachueHmong::LetterYa),
            '𞄙' => Ok(NyiakengPuachueHmong::LetterNqa),
            '𞄚' => Ok(NyiakengPuachueHmong::LetterPa),
            '𞄛' => Ok(NyiakengPuachueHmong::LetterXya),
            '𞄜' => Ok(NyiakengPuachueHmong::LetterNpa),
            '𞄝' => Ok(NyiakengPuachueHmong::LetterDla),
            '𞄞' => Ok(NyiakengPuachueHmong::LetterNpla),
            '𞄟' => Ok(NyiakengPuachueHmong::LetterHah),
            '𞄠' => Ok(NyiakengPuachueHmong::LetterMla),
            '𞄡' => Ok(NyiakengPuachueHmong::LetterPla),
            '𞄢' => Ok(NyiakengPuachueHmong::LetterGa),
            '𞄣' => Ok(NyiakengPuachueHmong::LetterRra),
            '𞄤' => Ok(NyiakengPuachueHmong::LetterA),
            '𞄥' => Ok(NyiakengPuachueHmong::LetterAa),
            '𞄦' => Ok(NyiakengPuachueHmong::LetterI),
            '𞄧' => Ok(NyiakengPuachueHmong::LetterU),
            '𞄨' => Ok(NyiakengPuachueHmong::LetterO),
            '𞄩' => Ok(NyiakengPuachueHmong::LetterOo),
            '𞄪' => Ok(NyiakengPuachueHmong::LetterE),
            '𞄫' => Ok(NyiakengPuachueHmong::LetterEe),
            '𞄬' => Ok(NyiakengPuachueHmong::LetterW),
            '𞄰' => Ok(NyiakengPuachueHmong::ToneDashB),
            '𞄱' => Ok(NyiakengPuachueHmong::ToneDashM),
            '𞄲' => Ok(NyiakengPuachueHmong::ToneDashJ),
            '𞄳' => Ok(NyiakengPuachueHmong::ToneDashV),
            '𞄴' => Ok(NyiakengPuachueHmong::ToneDashS),
            '𞄵' => Ok(NyiakengPuachueHmong::ToneDashG),
            '𞄶' => Ok(NyiakengPuachueHmong::ToneDashD),
            '𞄷' => Ok(NyiakengPuachueHmong::SignForPerson),
            '𞄸' => Ok(NyiakengPuachueHmong::SignForThing),
            '𞄹' => Ok(NyiakengPuachueHmong::SignForLocation),
            '𞄺' => Ok(NyiakengPuachueHmong::SignForAnimal),
            '𞄻' => Ok(NyiakengPuachueHmong::SignForInvertebrate),
            '𞄼' => Ok(NyiakengPuachueHmong::SignXwXw),
            '𞄽' => Ok(NyiakengPuachueHmong::SyllableLengthener),
            '𞅀' => Ok(NyiakengPuachueHmong::DigitZero),
            '𞅁' => Ok(NyiakengPuachueHmong::DigitOne),
            '𞅂' => Ok(NyiakengPuachueHmong::DigitTwo),
            '𞅃' => Ok(NyiakengPuachueHmong::DigitThree),
            '𞅄' => Ok(NyiakengPuachueHmong::DigitFour),
            '𞅅' => Ok(NyiakengPuachueHmong::DigitFive),
            '𞅆' => Ok(NyiakengPuachueHmong::DigitSix),
            '𞅇' => Ok(NyiakengPuachueHmong::DigitSeven),
            '𞅈' => Ok(NyiakengPuachueHmong::DigitEight),
            '𞅉' => Ok(NyiakengPuachueHmong::DigitNine),
            '𞅎' => Ok(NyiakengPuachueHmong::LogogramNyaj),
            _ => Err(()),
        }
    }
}

impl Into<u32> for NyiakengPuachueHmong {
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

impl std::convert::TryFrom<u32> for NyiakengPuachueHmong {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for NyiakengPuachueHmong {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl NyiakengPuachueHmong {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        NyiakengPuachueHmong::LetterMa
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            NyiakengPuachueHmong::LetterMa => "nyiakeng puachue hmong letter ma",
            NyiakengPuachueHmong::LetterTsa => "nyiakeng puachue hmong letter tsa",
            NyiakengPuachueHmong::LetterNta => "nyiakeng puachue hmong letter nta",
            NyiakengPuachueHmong::LetterTa => "nyiakeng puachue hmong letter ta",
            NyiakengPuachueHmong::LetterHa => "nyiakeng puachue hmong letter ha",
            NyiakengPuachueHmong::LetterNa => "nyiakeng puachue hmong letter na",
            NyiakengPuachueHmong::LetterXa => "nyiakeng puachue hmong letter xa",
            NyiakengPuachueHmong::LetterNka => "nyiakeng puachue hmong letter nka",
            NyiakengPuachueHmong::LetterCa => "nyiakeng puachue hmong letter ca",
            NyiakengPuachueHmong::LetterLa => "nyiakeng puachue hmong letter la",
            NyiakengPuachueHmong::LetterSa => "nyiakeng puachue hmong letter sa",
            NyiakengPuachueHmong::LetterZa => "nyiakeng puachue hmong letter za",
            NyiakengPuachueHmong::LetterNca => "nyiakeng puachue hmong letter nca",
            NyiakengPuachueHmong::LetterNtsa => "nyiakeng puachue hmong letter ntsa",
            NyiakengPuachueHmong::LetterKa => "nyiakeng puachue hmong letter ka",
            NyiakengPuachueHmong::LetterDa => "nyiakeng puachue hmong letter da",
            NyiakengPuachueHmong::LetterNya => "nyiakeng puachue hmong letter nya",
            NyiakengPuachueHmong::LetterNra => "nyiakeng puachue hmong letter nra",
            NyiakengPuachueHmong::LetterVa => "nyiakeng puachue hmong letter va",
            NyiakengPuachueHmong::LetterNtxa => "nyiakeng puachue hmong letter ntxa",
            NyiakengPuachueHmong::LetterTxa => "nyiakeng puachue hmong letter txa",
            NyiakengPuachueHmong::LetterFa => "nyiakeng puachue hmong letter fa",
            NyiakengPuachueHmong::LetterRa => "nyiakeng puachue hmong letter ra",
            NyiakengPuachueHmong::LetterQa => "nyiakeng puachue hmong letter qa",
            NyiakengPuachueHmong::LetterYa => "nyiakeng puachue hmong letter ya",
            NyiakengPuachueHmong::LetterNqa => "nyiakeng puachue hmong letter nqa",
            NyiakengPuachueHmong::LetterPa => "nyiakeng puachue hmong letter pa",
            NyiakengPuachueHmong::LetterXya => "nyiakeng puachue hmong letter xya",
            NyiakengPuachueHmong::LetterNpa => "nyiakeng puachue hmong letter npa",
            NyiakengPuachueHmong::LetterDla => "nyiakeng puachue hmong letter dla",
            NyiakengPuachueHmong::LetterNpla => "nyiakeng puachue hmong letter npla",
            NyiakengPuachueHmong::LetterHah => "nyiakeng puachue hmong letter hah",
            NyiakengPuachueHmong::LetterMla => "nyiakeng puachue hmong letter mla",
            NyiakengPuachueHmong::LetterPla => "nyiakeng puachue hmong letter pla",
            NyiakengPuachueHmong::LetterGa => "nyiakeng puachue hmong letter ga",
            NyiakengPuachueHmong::LetterRra => "nyiakeng puachue hmong letter rra",
            NyiakengPuachueHmong::LetterA => "nyiakeng puachue hmong letter a",
            NyiakengPuachueHmong::LetterAa => "nyiakeng puachue hmong letter aa",
            NyiakengPuachueHmong::LetterI => "nyiakeng puachue hmong letter i",
            NyiakengPuachueHmong::LetterU => "nyiakeng puachue hmong letter u",
            NyiakengPuachueHmong::LetterO => "nyiakeng puachue hmong letter o",
            NyiakengPuachueHmong::LetterOo => "nyiakeng puachue hmong letter oo",
            NyiakengPuachueHmong::LetterE => "nyiakeng puachue hmong letter e",
            NyiakengPuachueHmong::LetterEe => "nyiakeng puachue hmong letter ee",
            NyiakengPuachueHmong::LetterW => "nyiakeng puachue hmong letter w",
            NyiakengPuachueHmong::ToneDashB => "nyiakeng puachue hmong tone-b",
            NyiakengPuachueHmong::ToneDashM => "nyiakeng puachue hmong tone-m",
            NyiakengPuachueHmong::ToneDashJ => "nyiakeng puachue hmong tone-j",
            NyiakengPuachueHmong::ToneDashV => "nyiakeng puachue hmong tone-v",
            NyiakengPuachueHmong::ToneDashS => "nyiakeng puachue hmong tone-s",
            NyiakengPuachueHmong::ToneDashG => "nyiakeng puachue hmong tone-g",
            NyiakengPuachueHmong::ToneDashD => "nyiakeng puachue hmong tone-d",
            NyiakengPuachueHmong::SignForPerson => "nyiakeng puachue hmong sign for person",
            NyiakengPuachueHmong::SignForThing => "nyiakeng puachue hmong sign for thing",
            NyiakengPuachueHmong::SignForLocation => "nyiakeng puachue hmong sign for location",
            NyiakengPuachueHmong::SignForAnimal => "nyiakeng puachue hmong sign for animal",
            NyiakengPuachueHmong::SignForInvertebrate => "nyiakeng puachue hmong sign for invertebrate",
            NyiakengPuachueHmong::SignXwXw => "nyiakeng puachue hmong sign xw xw",
            NyiakengPuachueHmong::SyllableLengthener => "nyiakeng puachue hmong syllable lengthener",
            NyiakengPuachueHmong::DigitZero => "nyiakeng puachue hmong digit zero",
            NyiakengPuachueHmong::DigitOne => "nyiakeng puachue hmong digit one",
            NyiakengPuachueHmong::DigitTwo => "nyiakeng puachue hmong digit two",
            NyiakengPuachueHmong::DigitThree => "nyiakeng puachue hmong digit three",
            NyiakengPuachueHmong::DigitFour => "nyiakeng puachue hmong digit four",
            NyiakengPuachueHmong::DigitFive => "nyiakeng puachue hmong digit five",
            NyiakengPuachueHmong::DigitSix => "nyiakeng puachue hmong digit six",
            NyiakengPuachueHmong::DigitSeven => "nyiakeng puachue hmong digit seven",
            NyiakengPuachueHmong::DigitEight => "nyiakeng puachue hmong digit eight",
            NyiakengPuachueHmong::DigitNine => "nyiakeng puachue hmong digit nine",
            NyiakengPuachueHmong::LogogramNyaj => "nyiakeng puachue hmong logogram nyaj",
        }
    }
}
