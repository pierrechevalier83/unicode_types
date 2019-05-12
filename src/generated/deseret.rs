
/// An enum to represent all characters in the Deseret block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Deseret {
    /// \u{10400}: '𐐀'
    CapitalLetterLongI,
    /// \u{10401}: '𐐁'
    CapitalLetterLongE,
    /// \u{10402}: '𐐂'
    CapitalLetterLongA,
    /// \u{10403}: '𐐃'
    CapitalLetterLongAh,
    /// \u{10404}: '𐐄'
    CapitalLetterLongO,
    /// \u{10405}: '𐐅'
    CapitalLetterLongOo,
    /// \u{10406}: '𐐆'
    CapitalLetterShortI,
    /// \u{10407}: '𐐇'
    CapitalLetterShortE,
    /// \u{10408}: '𐐈'
    CapitalLetterShortA,
    /// \u{10409}: '𐐉'
    CapitalLetterShortAh,
    /// \u{1040a}: '𐐊'
    CapitalLetterShortO,
    /// \u{1040b}: '𐐋'
    CapitalLetterShortOo,
    /// \u{1040c}: '𐐌'
    CapitalLetterAy,
    /// \u{1040d}: '𐐍'
    CapitalLetterOw,
    /// \u{1040e}: '𐐎'
    CapitalLetterWu,
    /// \u{1040f}: '𐐏'
    CapitalLetterYee,
    /// \u{10410}: '𐐐'
    CapitalLetterH,
    /// \u{10411}: '𐐑'
    CapitalLetterPee,
    /// \u{10412}: '𐐒'
    CapitalLetterBee,
    /// \u{10413}: '𐐓'
    CapitalLetterTee,
    /// \u{10414}: '𐐔'
    CapitalLetterDee,
    /// \u{10415}: '𐐕'
    CapitalLetterChee,
    /// \u{10416}: '𐐖'
    CapitalLetterJee,
    /// \u{10417}: '𐐗'
    CapitalLetterKay,
    /// \u{10418}: '𐐘'
    CapitalLetterGay,
    /// \u{10419}: '𐐙'
    CapitalLetterEf,
    /// \u{1041a}: '𐐚'
    CapitalLetterVee,
    /// \u{1041b}: '𐐛'
    CapitalLetterEth,
    /// \u{1041c}: '𐐜'
    CapitalLetterThee,
    /// \u{1041d}: '𐐝'
    CapitalLetterEs,
    /// \u{1041e}: '𐐞'
    CapitalLetterZee,
    /// \u{1041f}: '𐐟'
    CapitalLetterEsh,
    /// \u{10420}: '𐐠'
    CapitalLetterZhee,
    /// \u{10421}: '𐐡'
    CapitalLetterEr,
    /// \u{10422}: '𐐢'
    CapitalLetterEl,
    /// \u{10423}: '𐐣'
    CapitalLetterEm,
    /// \u{10424}: '𐐤'
    CapitalLetterEn,
    /// \u{10425}: '𐐥'
    CapitalLetterEng,
    /// \u{10426}: '𐐦'
    CapitalLetterOi,
    /// \u{10427}: '𐐧'
    CapitalLetterEw,
    /// \u{10428}: '𐐨'
    SmallLetterLongI,
    /// \u{10429}: '𐐩'
    SmallLetterLongE,
    /// \u{1042a}: '𐐪'
    SmallLetterLongA,
    /// \u{1042b}: '𐐫'
    SmallLetterLongAh,
    /// \u{1042c}: '𐐬'
    SmallLetterLongO,
    /// \u{1042d}: '𐐭'
    SmallLetterLongOo,
    /// \u{1042e}: '𐐮'
    SmallLetterShortI,
    /// \u{1042f}: '𐐯'
    SmallLetterShortE,
    /// \u{10430}: '𐐰'
    SmallLetterShortA,
    /// \u{10431}: '𐐱'
    SmallLetterShortAh,
    /// \u{10432}: '𐐲'
    SmallLetterShortO,
    /// \u{10433}: '𐐳'
    SmallLetterShortOo,
    /// \u{10434}: '𐐴'
    SmallLetterAy,
    /// \u{10435}: '𐐵'
    SmallLetterOw,
    /// \u{10436}: '𐐶'
    SmallLetterWu,
    /// \u{10437}: '𐐷'
    SmallLetterYee,
    /// \u{10438}: '𐐸'
    SmallLetterH,
    /// \u{10439}: '𐐹'
    SmallLetterPee,
    /// \u{1043a}: '𐐺'
    SmallLetterBee,
    /// \u{1043b}: '𐐻'
    SmallLetterTee,
    /// \u{1043c}: '𐐼'
    SmallLetterDee,
    /// \u{1043d}: '𐐽'
    SmallLetterChee,
    /// \u{1043e}: '𐐾'
    SmallLetterJee,
    /// \u{1043f}: '𐐿'
    SmallLetterKay,
    /// \u{10440}: '𐑀'
    SmallLetterGay,
    /// \u{10441}: '𐑁'
    SmallLetterEf,
    /// \u{10442}: '𐑂'
    SmallLetterVee,
    /// \u{10443}: '𐑃'
    SmallLetterEth,
    /// \u{10444}: '𐑄'
    SmallLetterThee,
    /// \u{10445}: '𐑅'
    SmallLetterEs,
    /// \u{10446}: '𐑆'
    SmallLetterZee,
    /// \u{10447}: '𐑇'
    SmallLetterEsh,
    /// \u{10448}: '𐑈'
    SmallLetterZhee,
    /// \u{10449}: '𐑉'
    SmallLetterEr,
    /// \u{1044a}: '𐑊'
    SmallLetterEl,
    /// \u{1044b}: '𐑋'
    SmallLetterEm,
    /// \u{1044c}: '𐑌'
    SmallLetterEn,
    /// \u{1044d}: '𐑍'
    SmallLetterEng,
    /// \u{1044e}: '𐑎'
    SmallLetterOi,
}

impl Into<char> for Deseret {
    fn into(self) -> char {
        match self {
            Deseret::CapitalLetterLongI => '𐐀',
            Deseret::CapitalLetterLongE => '𐐁',
            Deseret::CapitalLetterLongA => '𐐂',
            Deseret::CapitalLetterLongAh => '𐐃',
            Deseret::CapitalLetterLongO => '𐐄',
            Deseret::CapitalLetterLongOo => '𐐅',
            Deseret::CapitalLetterShortI => '𐐆',
            Deseret::CapitalLetterShortE => '𐐇',
            Deseret::CapitalLetterShortA => '𐐈',
            Deseret::CapitalLetterShortAh => '𐐉',
            Deseret::CapitalLetterShortO => '𐐊',
            Deseret::CapitalLetterShortOo => '𐐋',
            Deseret::CapitalLetterAy => '𐐌',
            Deseret::CapitalLetterOw => '𐐍',
            Deseret::CapitalLetterWu => '𐐎',
            Deseret::CapitalLetterYee => '𐐏',
            Deseret::CapitalLetterH => '𐐐',
            Deseret::CapitalLetterPee => '𐐑',
            Deseret::CapitalLetterBee => '𐐒',
            Deseret::CapitalLetterTee => '𐐓',
            Deseret::CapitalLetterDee => '𐐔',
            Deseret::CapitalLetterChee => '𐐕',
            Deseret::CapitalLetterJee => '𐐖',
            Deseret::CapitalLetterKay => '𐐗',
            Deseret::CapitalLetterGay => '𐐘',
            Deseret::CapitalLetterEf => '𐐙',
            Deseret::CapitalLetterVee => '𐐚',
            Deseret::CapitalLetterEth => '𐐛',
            Deseret::CapitalLetterThee => '𐐜',
            Deseret::CapitalLetterEs => '𐐝',
            Deseret::CapitalLetterZee => '𐐞',
            Deseret::CapitalLetterEsh => '𐐟',
            Deseret::CapitalLetterZhee => '𐐠',
            Deseret::CapitalLetterEr => '𐐡',
            Deseret::CapitalLetterEl => '𐐢',
            Deseret::CapitalLetterEm => '𐐣',
            Deseret::CapitalLetterEn => '𐐤',
            Deseret::CapitalLetterEng => '𐐥',
            Deseret::CapitalLetterOi => '𐐦',
            Deseret::CapitalLetterEw => '𐐧',
            Deseret::SmallLetterLongI => '𐐨',
            Deseret::SmallLetterLongE => '𐐩',
            Deseret::SmallLetterLongA => '𐐪',
            Deseret::SmallLetterLongAh => '𐐫',
            Deseret::SmallLetterLongO => '𐐬',
            Deseret::SmallLetterLongOo => '𐐭',
            Deseret::SmallLetterShortI => '𐐮',
            Deseret::SmallLetterShortE => '𐐯',
            Deseret::SmallLetterShortA => '𐐰',
            Deseret::SmallLetterShortAh => '𐐱',
            Deseret::SmallLetterShortO => '𐐲',
            Deseret::SmallLetterShortOo => '𐐳',
            Deseret::SmallLetterAy => '𐐴',
            Deseret::SmallLetterOw => '𐐵',
            Deseret::SmallLetterWu => '𐐶',
            Deseret::SmallLetterYee => '𐐷',
            Deseret::SmallLetterH => '𐐸',
            Deseret::SmallLetterPee => '𐐹',
            Deseret::SmallLetterBee => '𐐺',
            Deseret::SmallLetterTee => '𐐻',
            Deseret::SmallLetterDee => '𐐼',
            Deseret::SmallLetterChee => '𐐽',
            Deseret::SmallLetterJee => '𐐾',
            Deseret::SmallLetterKay => '𐐿',
            Deseret::SmallLetterGay => '𐑀',
            Deseret::SmallLetterEf => '𐑁',
            Deseret::SmallLetterVee => '𐑂',
            Deseret::SmallLetterEth => '𐑃',
            Deseret::SmallLetterThee => '𐑄',
            Deseret::SmallLetterEs => '𐑅',
            Deseret::SmallLetterZee => '𐑆',
            Deseret::SmallLetterEsh => '𐑇',
            Deseret::SmallLetterZhee => '𐑈',
            Deseret::SmallLetterEr => '𐑉',
            Deseret::SmallLetterEl => '𐑊',
            Deseret::SmallLetterEm => '𐑋',
            Deseret::SmallLetterEn => '𐑌',
            Deseret::SmallLetterEng => '𐑍',
            Deseret::SmallLetterOi => '𐑎',
        }
    }
}

impl std::convert::TryFrom<char> for Deseret {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𐐀' => Ok(Deseret::CapitalLetterLongI),
            '𐐁' => Ok(Deseret::CapitalLetterLongE),
            '𐐂' => Ok(Deseret::CapitalLetterLongA),
            '𐐃' => Ok(Deseret::CapitalLetterLongAh),
            '𐐄' => Ok(Deseret::CapitalLetterLongO),
            '𐐅' => Ok(Deseret::CapitalLetterLongOo),
            '𐐆' => Ok(Deseret::CapitalLetterShortI),
            '𐐇' => Ok(Deseret::CapitalLetterShortE),
            '𐐈' => Ok(Deseret::CapitalLetterShortA),
            '𐐉' => Ok(Deseret::CapitalLetterShortAh),
            '𐐊' => Ok(Deseret::CapitalLetterShortO),
            '𐐋' => Ok(Deseret::CapitalLetterShortOo),
            '𐐌' => Ok(Deseret::CapitalLetterAy),
            '𐐍' => Ok(Deseret::CapitalLetterOw),
            '𐐎' => Ok(Deseret::CapitalLetterWu),
            '𐐏' => Ok(Deseret::CapitalLetterYee),
            '𐐐' => Ok(Deseret::CapitalLetterH),
            '𐐑' => Ok(Deseret::CapitalLetterPee),
            '𐐒' => Ok(Deseret::CapitalLetterBee),
            '𐐓' => Ok(Deseret::CapitalLetterTee),
            '𐐔' => Ok(Deseret::CapitalLetterDee),
            '𐐕' => Ok(Deseret::CapitalLetterChee),
            '𐐖' => Ok(Deseret::CapitalLetterJee),
            '𐐗' => Ok(Deseret::CapitalLetterKay),
            '𐐘' => Ok(Deseret::CapitalLetterGay),
            '𐐙' => Ok(Deseret::CapitalLetterEf),
            '𐐚' => Ok(Deseret::CapitalLetterVee),
            '𐐛' => Ok(Deseret::CapitalLetterEth),
            '𐐜' => Ok(Deseret::CapitalLetterThee),
            '𐐝' => Ok(Deseret::CapitalLetterEs),
            '𐐞' => Ok(Deseret::CapitalLetterZee),
            '𐐟' => Ok(Deseret::CapitalLetterEsh),
            '𐐠' => Ok(Deseret::CapitalLetterZhee),
            '𐐡' => Ok(Deseret::CapitalLetterEr),
            '𐐢' => Ok(Deseret::CapitalLetterEl),
            '𐐣' => Ok(Deseret::CapitalLetterEm),
            '𐐤' => Ok(Deseret::CapitalLetterEn),
            '𐐥' => Ok(Deseret::CapitalLetterEng),
            '𐐦' => Ok(Deseret::CapitalLetterOi),
            '𐐧' => Ok(Deseret::CapitalLetterEw),
            '𐐨' => Ok(Deseret::SmallLetterLongI),
            '𐐩' => Ok(Deseret::SmallLetterLongE),
            '𐐪' => Ok(Deseret::SmallLetterLongA),
            '𐐫' => Ok(Deseret::SmallLetterLongAh),
            '𐐬' => Ok(Deseret::SmallLetterLongO),
            '𐐭' => Ok(Deseret::SmallLetterLongOo),
            '𐐮' => Ok(Deseret::SmallLetterShortI),
            '𐐯' => Ok(Deseret::SmallLetterShortE),
            '𐐰' => Ok(Deseret::SmallLetterShortA),
            '𐐱' => Ok(Deseret::SmallLetterShortAh),
            '𐐲' => Ok(Deseret::SmallLetterShortO),
            '𐐳' => Ok(Deseret::SmallLetterShortOo),
            '𐐴' => Ok(Deseret::SmallLetterAy),
            '𐐵' => Ok(Deseret::SmallLetterOw),
            '𐐶' => Ok(Deseret::SmallLetterWu),
            '𐐷' => Ok(Deseret::SmallLetterYee),
            '𐐸' => Ok(Deseret::SmallLetterH),
            '𐐹' => Ok(Deseret::SmallLetterPee),
            '𐐺' => Ok(Deseret::SmallLetterBee),
            '𐐻' => Ok(Deseret::SmallLetterTee),
            '𐐼' => Ok(Deseret::SmallLetterDee),
            '𐐽' => Ok(Deseret::SmallLetterChee),
            '𐐾' => Ok(Deseret::SmallLetterJee),
            '𐐿' => Ok(Deseret::SmallLetterKay),
            '𐑀' => Ok(Deseret::SmallLetterGay),
            '𐑁' => Ok(Deseret::SmallLetterEf),
            '𐑂' => Ok(Deseret::SmallLetterVee),
            '𐑃' => Ok(Deseret::SmallLetterEth),
            '𐑄' => Ok(Deseret::SmallLetterThee),
            '𐑅' => Ok(Deseret::SmallLetterEs),
            '𐑆' => Ok(Deseret::SmallLetterZee),
            '𐑇' => Ok(Deseret::SmallLetterEsh),
            '𐑈' => Ok(Deseret::SmallLetterZhee),
            '𐑉' => Ok(Deseret::SmallLetterEr),
            '𐑊' => Ok(Deseret::SmallLetterEl),
            '𐑋' => Ok(Deseret::SmallLetterEm),
            '𐑌' => Ok(Deseret::SmallLetterEn),
            '𐑍' => Ok(Deseret::SmallLetterEng),
            '𐑎' => Ok(Deseret::SmallLetterOi),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Deseret {
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

impl std::convert::TryFrom<u32> for Deseret {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Deseret {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Deseret {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Deseret::CapitalLetterLongI
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Deseret::CapitalLetterLongI => "deseret capital letter long i",
            Deseret::CapitalLetterLongE => "deseret capital letter long e",
            Deseret::CapitalLetterLongA => "deseret capital letter long a",
            Deseret::CapitalLetterLongAh => "deseret capital letter long ah",
            Deseret::CapitalLetterLongO => "deseret capital letter long o",
            Deseret::CapitalLetterLongOo => "deseret capital letter long oo",
            Deseret::CapitalLetterShortI => "deseret capital letter short i",
            Deseret::CapitalLetterShortE => "deseret capital letter short e",
            Deseret::CapitalLetterShortA => "deseret capital letter short a",
            Deseret::CapitalLetterShortAh => "deseret capital letter short ah",
            Deseret::CapitalLetterShortO => "deseret capital letter short o",
            Deseret::CapitalLetterShortOo => "deseret capital letter short oo",
            Deseret::CapitalLetterAy => "deseret capital letter ay",
            Deseret::CapitalLetterOw => "deseret capital letter ow",
            Deseret::CapitalLetterWu => "deseret capital letter wu",
            Deseret::CapitalLetterYee => "deseret capital letter yee",
            Deseret::CapitalLetterH => "deseret capital letter h",
            Deseret::CapitalLetterPee => "deseret capital letter pee",
            Deseret::CapitalLetterBee => "deseret capital letter bee",
            Deseret::CapitalLetterTee => "deseret capital letter tee",
            Deseret::CapitalLetterDee => "deseret capital letter dee",
            Deseret::CapitalLetterChee => "deseret capital letter chee",
            Deseret::CapitalLetterJee => "deseret capital letter jee",
            Deseret::CapitalLetterKay => "deseret capital letter kay",
            Deseret::CapitalLetterGay => "deseret capital letter gay",
            Deseret::CapitalLetterEf => "deseret capital letter ef",
            Deseret::CapitalLetterVee => "deseret capital letter vee",
            Deseret::CapitalLetterEth => "deseret capital letter eth",
            Deseret::CapitalLetterThee => "deseret capital letter thee",
            Deseret::CapitalLetterEs => "deseret capital letter es",
            Deseret::CapitalLetterZee => "deseret capital letter zee",
            Deseret::CapitalLetterEsh => "deseret capital letter esh",
            Deseret::CapitalLetterZhee => "deseret capital letter zhee",
            Deseret::CapitalLetterEr => "deseret capital letter er",
            Deseret::CapitalLetterEl => "deseret capital letter el",
            Deseret::CapitalLetterEm => "deseret capital letter em",
            Deseret::CapitalLetterEn => "deseret capital letter en",
            Deseret::CapitalLetterEng => "deseret capital letter eng",
            Deseret::CapitalLetterOi => "deseret capital letter oi",
            Deseret::CapitalLetterEw => "deseret capital letter ew",
            Deseret::SmallLetterLongI => "deseret small letter long i",
            Deseret::SmallLetterLongE => "deseret small letter long e",
            Deseret::SmallLetterLongA => "deseret small letter long a",
            Deseret::SmallLetterLongAh => "deseret small letter long ah",
            Deseret::SmallLetterLongO => "deseret small letter long o",
            Deseret::SmallLetterLongOo => "deseret small letter long oo",
            Deseret::SmallLetterShortI => "deseret small letter short i",
            Deseret::SmallLetterShortE => "deseret small letter short e",
            Deseret::SmallLetterShortA => "deseret small letter short a",
            Deseret::SmallLetterShortAh => "deseret small letter short ah",
            Deseret::SmallLetterShortO => "deseret small letter short o",
            Deseret::SmallLetterShortOo => "deseret small letter short oo",
            Deseret::SmallLetterAy => "deseret small letter ay",
            Deseret::SmallLetterOw => "deseret small letter ow",
            Deseret::SmallLetterWu => "deseret small letter wu",
            Deseret::SmallLetterYee => "deseret small letter yee",
            Deseret::SmallLetterH => "deseret small letter h",
            Deseret::SmallLetterPee => "deseret small letter pee",
            Deseret::SmallLetterBee => "deseret small letter bee",
            Deseret::SmallLetterTee => "deseret small letter tee",
            Deseret::SmallLetterDee => "deseret small letter dee",
            Deseret::SmallLetterChee => "deseret small letter chee",
            Deseret::SmallLetterJee => "deseret small letter jee",
            Deseret::SmallLetterKay => "deseret small letter kay",
            Deseret::SmallLetterGay => "deseret small letter gay",
            Deseret::SmallLetterEf => "deseret small letter ef",
            Deseret::SmallLetterVee => "deseret small letter vee",
            Deseret::SmallLetterEth => "deseret small letter eth",
            Deseret::SmallLetterThee => "deseret small letter thee",
            Deseret::SmallLetterEs => "deseret small letter es",
            Deseret::SmallLetterZee => "deseret small letter zee",
            Deseret::SmallLetterEsh => "deseret small letter esh",
            Deseret::SmallLetterZhee => "deseret small letter zhee",
            Deseret::SmallLetterEr => "deseret small letter er",
            Deseret::SmallLetterEl => "deseret small letter el",
            Deseret::SmallLetterEm => "deseret small letter em",
            Deseret::SmallLetterEn => "deseret small letter en",
            Deseret::SmallLetterEng => "deseret small letter eng",
            Deseret::SmallLetterOi => "deseret small letter oi",
        }
    }
}
