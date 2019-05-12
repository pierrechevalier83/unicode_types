
/// An enum to represent all characters in the Tags block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Tags {
    /// \u{e0001}: '󠀁'
    LanguageTag,
    /// \u{e0020}: '󠀠'
    TagSpace,
    /// \u{e0021}: '󠀡'
    TagExclamationMark,
    /// \u{e0022}: '󠀢'
    TagQuotationMark,
    /// \u{e0023}: '󠀣'
    TagNumberSign,
    /// \u{e0024}: '󠀤'
    TagDollarSign,
    /// \u{e0025}: '󠀥'
    TagPercentSign,
    /// \u{e0026}: '󠀦'
    TagAmpersand,
    /// \u{e0027}: '󠀧'
    TagApostrophe,
    /// \u{e0028}: '󠀨'
    TagLeftParenthesis,
    /// \u{e0029}: '󠀩'
    TagRightParenthesis,
    /// \u{e002a}: '󠀪'
    TagAsterisk,
    /// \u{e002b}: '󠀫'
    TagPlusSign,
    /// \u{e002c}: '󠀬'
    TagComma,
    /// \u{e002d}: '󠀭'
    TagHyphenDashMinus,
    /// \u{e002e}: '󠀮'
    TagFullStop,
    /// \u{e002f}: '󠀯'
    TagSolidus,
    /// \u{e0030}: '󠀰'
    TagDigitZero,
    /// \u{e0031}: '󠀱'
    TagDigitOne,
    /// \u{e0032}: '󠀲'
    TagDigitTwo,
    /// \u{e0033}: '󠀳'
    TagDigitThree,
    /// \u{e0034}: '󠀴'
    TagDigitFour,
    /// \u{e0035}: '󠀵'
    TagDigitFive,
    /// \u{e0036}: '󠀶'
    TagDigitSix,
    /// \u{e0037}: '󠀷'
    TagDigitSeven,
    /// \u{e0038}: '󠀸'
    TagDigitEight,
    /// \u{e0039}: '󠀹'
    TagDigitNine,
    /// \u{e003a}: '󠀺'
    TagColon,
    /// \u{e003b}: '󠀻'
    TagSemicolon,
    /// \u{e003c}: '󠀼'
    TagLessDashThanSign,
    /// \u{e003d}: '󠀽'
    TagEqualsSign,
    /// \u{e003e}: '󠀾'
    TagGreaterDashThanSign,
    /// \u{e003f}: '󠀿'
    TagQuestionMark,
    /// \u{e0040}: '󠁀'
    TagCommercialAt,
    /// \u{e0041}: '󠁁'
    TagLatinCapitalLetterA,
    /// \u{e0042}: '󠁂'
    TagLatinCapitalLetterB,
    /// \u{e0043}: '󠁃'
    TagLatinCapitalLetterC,
    /// \u{e0044}: '󠁄'
    TagLatinCapitalLetterD,
    /// \u{e0045}: '󠁅'
    TagLatinCapitalLetterE,
    /// \u{e0046}: '󠁆'
    TagLatinCapitalLetterF,
    /// \u{e0047}: '󠁇'
    TagLatinCapitalLetterG,
    /// \u{e0048}: '󠁈'
    TagLatinCapitalLetterH,
    /// \u{e0049}: '󠁉'
    TagLatinCapitalLetterI,
    /// \u{e004a}: '󠁊'
    TagLatinCapitalLetterJ,
    /// \u{e004b}: '󠁋'
    TagLatinCapitalLetterK,
    /// \u{e004c}: '󠁌'
    TagLatinCapitalLetterL,
    /// \u{e004d}: '󠁍'
    TagLatinCapitalLetterM,
    /// \u{e004e}: '󠁎'
    TagLatinCapitalLetterN,
    /// \u{e004f}: '󠁏'
    TagLatinCapitalLetterO,
    /// \u{e0050}: '󠁐'
    TagLatinCapitalLetterP,
    /// \u{e0051}: '󠁑'
    TagLatinCapitalLetterQ,
    /// \u{e0052}: '󠁒'
    TagLatinCapitalLetterR,
    /// \u{e0053}: '󠁓'
    TagLatinCapitalLetterS,
    /// \u{e0054}: '󠁔'
    TagLatinCapitalLetterT,
    /// \u{e0055}: '󠁕'
    TagLatinCapitalLetterU,
    /// \u{e0056}: '󠁖'
    TagLatinCapitalLetterV,
    /// \u{e0057}: '󠁗'
    TagLatinCapitalLetterW,
    /// \u{e0058}: '󠁘'
    TagLatinCapitalLetterX,
    /// \u{e0059}: '󠁙'
    TagLatinCapitalLetterY,
    /// \u{e005a}: '󠁚'
    TagLatinCapitalLetterZ,
    /// \u{e005b}: '󠁛'
    TagLeftSquareBracket,
    /// \u{e005c}: '󠁜'
    TagReverseSolidus,
    /// \u{e005d}: '󠁝'
    TagRightSquareBracket,
    /// \u{e005e}: '󠁞'
    TagCircumflexAccent,
    /// \u{e005f}: '󠁟'
    TagLowLine,
    /// \u{e0060}: '󠁠'
    TagGraveAccent,
    /// \u{e0061}: '󠁡'
    TagLatinSmallLetterA,
    /// \u{e0062}: '󠁢'
    TagLatinSmallLetterB,
    /// \u{e0063}: '󠁣'
    TagLatinSmallLetterC,
    /// \u{e0064}: '󠁤'
    TagLatinSmallLetterD,
    /// \u{e0065}: '󠁥'
    TagLatinSmallLetterE,
    /// \u{e0066}: '󠁦'
    TagLatinSmallLetterF,
    /// \u{e0067}: '󠁧'
    TagLatinSmallLetterG,
    /// \u{e0068}: '󠁨'
    TagLatinSmallLetterH,
    /// \u{e0069}: '󠁩'
    TagLatinSmallLetterI,
    /// \u{e006a}: '󠁪'
    TagLatinSmallLetterJ,
    /// \u{e006b}: '󠁫'
    TagLatinSmallLetterK,
    /// \u{e006c}: '󠁬'
    TagLatinSmallLetterL,
    /// \u{e006d}: '󠁭'
    TagLatinSmallLetterM,
    /// \u{e006e}: '󠁮'
    TagLatinSmallLetterN,
    /// \u{e006f}: '󠁯'
    TagLatinSmallLetterO,
    /// \u{e0070}: '󠁰'
    TagLatinSmallLetterP,
    /// \u{e0071}: '󠁱'
    TagLatinSmallLetterQ,
    /// \u{e0072}: '󠁲'
    TagLatinSmallLetterR,
    /// \u{e0073}: '󠁳'
    TagLatinSmallLetterS,
    /// \u{e0074}: '󠁴'
    TagLatinSmallLetterT,
    /// \u{e0075}: '󠁵'
    TagLatinSmallLetterU,
    /// \u{e0076}: '󠁶'
    TagLatinSmallLetterV,
    /// \u{e0077}: '󠁷'
    TagLatinSmallLetterW,
    /// \u{e0078}: '󠁸'
    TagLatinSmallLetterX,
    /// \u{e0079}: '󠁹'
    TagLatinSmallLetterY,
    /// \u{e007a}: '󠁺'
    TagLatinSmallLetterZ,
    /// \u{e007b}: '󠁻'
    TagLeftCurlyBracket,
    /// \u{e007c}: '󠁼'
    TagVerticalLine,
    /// \u{e007d}: '󠁽'
    TagRightCurlyBracket,
    /// \u{e007e}: '󠁾'
    TagTilde,
}

impl Into<char> for Tags {
    fn into(self) -> char {
        match self {
            Tags::LanguageTag => '󠀁',
            Tags::TagSpace => '󠀠',
            Tags::TagExclamationMark => '󠀡',
            Tags::TagQuotationMark => '󠀢',
            Tags::TagNumberSign => '󠀣',
            Tags::TagDollarSign => '󠀤',
            Tags::TagPercentSign => '󠀥',
            Tags::TagAmpersand => '󠀦',
            Tags::TagApostrophe => '󠀧',
            Tags::TagLeftParenthesis => '󠀨',
            Tags::TagRightParenthesis => '󠀩',
            Tags::TagAsterisk => '󠀪',
            Tags::TagPlusSign => '󠀫',
            Tags::TagComma => '󠀬',
            Tags::TagHyphenDashMinus => '󠀭',
            Tags::TagFullStop => '󠀮',
            Tags::TagSolidus => '󠀯',
            Tags::TagDigitZero => '󠀰',
            Tags::TagDigitOne => '󠀱',
            Tags::TagDigitTwo => '󠀲',
            Tags::TagDigitThree => '󠀳',
            Tags::TagDigitFour => '󠀴',
            Tags::TagDigitFive => '󠀵',
            Tags::TagDigitSix => '󠀶',
            Tags::TagDigitSeven => '󠀷',
            Tags::TagDigitEight => '󠀸',
            Tags::TagDigitNine => '󠀹',
            Tags::TagColon => '󠀺',
            Tags::TagSemicolon => '󠀻',
            Tags::TagLessDashThanSign => '󠀼',
            Tags::TagEqualsSign => '󠀽',
            Tags::TagGreaterDashThanSign => '󠀾',
            Tags::TagQuestionMark => '󠀿',
            Tags::TagCommercialAt => '󠁀',
            Tags::TagLatinCapitalLetterA => '󠁁',
            Tags::TagLatinCapitalLetterB => '󠁂',
            Tags::TagLatinCapitalLetterC => '󠁃',
            Tags::TagLatinCapitalLetterD => '󠁄',
            Tags::TagLatinCapitalLetterE => '󠁅',
            Tags::TagLatinCapitalLetterF => '󠁆',
            Tags::TagLatinCapitalLetterG => '󠁇',
            Tags::TagLatinCapitalLetterH => '󠁈',
            Tags::TagLatinCapitalLetterI => '󠁉',
            Tags::TagLatinCapitalLetterJ => '󠁊',
            Tags::TagLatinCapitalLetterK => '󠁋',
            Tags::TagLatinCapitalLetterL => '󠁌',
            Tags::TagLatinCapitalLetterM => '󠁍',
            Tags::TagLatinCapitalLetterN => '󠁎',
            Tags::TagLatinCapitalLetterO => '󠁏',
            Tags::TagLatinCapitalLetterP => '󠁐',
            Tags::TagLatinCapitalLetterQ => '󠁑',
            Tags::TagLatinCapitalLetterR => '󠁒',
            Tags::TagLatinCapitalLetterS => '󠁓',
            Tags::TagLatinCapitalLetterT => '󠁔',
            Tags::TagLatinCapitalLetterU => '󠁕',
            Tags::TagLatinCapitalLetterV => '󠁖',
            Tags::TagLatinCapitalLetterW => '󠁗',
            Tags::TagLatinCapitalLetterX => '󠁘',
            Tags::TagLatinCapitalLetterY => '󠁙',
            Tags::TagLatinCapitalLetterZ => '󠁚',
            Tags::TagLeftSquareBracket => '󠁛',
            Tags::TagReverseSolidus => '󠁜',
            Tags::TagRightSquareBracket => '󠁝',
            Tags::TagCircumflexAccent => '󠁞',
            Tags::TagLowLine => '󠁟',
            Tags::TagGraveAccent => '󠁠',
            Tags::TagLatinSmallLetterA => '󠁡',
            Tags::TagLatinSmallLetterB => '󠁢',
            Tags::TagLatinSmallLetterC => '󠁣',
            Tags::TagLatinSmallLetterD => '󠁤',
            Tags::TagLatinSmallLetterE => '󠁥',
            Tags::TagLatinSmallLetterF => '󠁦',
            Tags::TagLatinSmallLetterG => '󠁧',
            Tags::TagLatinSmallLetterH => '󠁨',
            Tags::TagLatinSmallLetterI => '󠁩',
            Tags::TagLatinSmallLetterJ => '󠁪',
            Tags::TagLatinSmallLetterK => '󠁫',
            Tags::TagLatinSmallLetterL => '󠁬',
            Tags::TagLatinSmallLetterM => '󠁭',
            Tags::TagLatinSmallLetterN => '󠁮',
            Tags::TagLatinSmallLetterO => '󠁯',
            Tags::TagLatinSmallLetterP => '󠁰',
            Tags::TagLatinSmallLetterQ => '󠁱',
            Tags::TagLatinSmallLetterR => '󠁲',
            Tags::TagLatinSmallLetterS => '󠁳',
            Tags::TagLatinSmallLetterT => '󠁴',
            Tags::TagLatinSmallLetterU => '󠁵',
            Tags::TagLatinSmallLetterV => '󠁶',
            Tags::TagLatinSmallLetterW => '󠁷',
            Tags::TagLatinSmallLetterX => '󠁸',
            Tags::TagLatinSmallLetterY => '󠁹',
            Tags::TagLatinSmallLetterZ => '󠁺',
            Tags::TagLeftCurlyBracket => '󠁻',
            Tags::TagVerticalLine => '󠁼',
            Tags::TagRightCurlyBracket => '󠁽',
            Tags::TagTilde => '󠁾',
        }
    }
}

impl std::convert::TryFrom<char> for Tags {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '󠀁' => Ok(Tags::LanguageTag),
            '󠀠' => Ok(Tags::TagSpace),
            '󠀡' => Ok(Tags::TagExclamationMark),
            '󠀢' => Ok(Tags::TagQuotationMark),
            '󠀣' => Ok(Tags::TagNumberSign),
            '󠀤' => Ok(Tags::TagDollarSign),
            '󠀥' => Ok(Tags::TagPercentSign),
            '󠀦' => Ok(Tags::TagAmpersand),
            '󠀧' => Ok(Tags::TagApostrophe),
            '󠀨' => Ok(Tags::TagLeftParenthesis),
            '󠀩' => Ok(Tags::TagRightParenthesis),
            '󠀪' => Ok(Tags::TagAsterisk),
            '󠀫' => Ok(Tags::TagPlusSign),
            '󠀬' => Ok(Tags::TagComma),
            '󠀭' => Ok(Tags::TagHyphenDashMinus),
            '󠀮' => Ok(Tags::TagFullStop),
            '󠀯' => Ok(Tags::TagSolidus),
            '󠀰' => Ok(Tags::TagDigitZero),
            '󠀱' => Ok(Tags::TagDigitOne),
            '󠀲' => Ok(Tags::TagDigitTwo),
            '󠀳' => Ok(Tags::TagDigitThree),
            '󠀴' => Ok(Tags::TagDigitFour),
            '󠀵' => Ok(Tags::TagDigitFive),
            '󠀶' => Ok(Tags::TagDigitSix),
            '󠀷' => Ok(Tags::TagDigitSeven),
            '󠀸' => Ok(Tags::TagDigitEight),
            '󠀹' => Ok(Tags::TagDigitNine),
            '󠀺' => Ok(Tags::TagColon),
            '󠀻' => Ok(Tags::TagSemicolon),
            '󠀼' => Ok(Tags::TagLessDashThanSign),
            '󠀽' => Ok(Tags::TagEqualsSign),
            '󠀾' => Ok(Tags::TagGreaterDashThanSign),
            '󠀿' => Ok(Tags::TagQuestionMark),
            '󠁀' => Ok(Tags::TagCommercialAt),
            '󠁁' => Ok(Tags::TagLatinCapitalLetterA),
            '󠁂' => Ok(Tags::TagLatinCapitalLetterB),
            '󠁃' => Ok(Tags::TagLatinCapitalLetterC),
            '󠁄' => Ok(Tags::TagLatinCapitalLetterD),
            '󠁅' => Ok(Tags::TagLatinCapitalLetterE),
            '󠁆' => Ok(Tags::TagLatinCapitalLetterF),
            '󠁇' => Ok(Tags::TagLatinCapitalLetterG),
            '󠁈' => Ok(Tags::TagLatinCapitalLetterH),
            '󠁉' => Ok(Tags::TagLatinCapitalLetterI),
            '󠁊' => Ok(Tags::TagLatinCapitalLetterJ),
            '󠁋' => Ok(Tags::TagLatinCapitalLetterK),
            '󠁌' => Ok(Tags::TagLatinCapitalLetterL),
            '󠁍' => Ok(Tags::TagLatinCapitalLetterM),
            '󠁎' => Ok(Tags::TagLatinCapitalLetterN),
            '󠁏' => Ok(Tags::TagLatinCapitalLetterO),
            '󠁐' => Ok(Tags::TagLatinCapitalLetterP),
            '󠁑' => Ok(Tags::TagLatinCapitalLetterQ),
            '󠁒' => Ok(Tags::TagLatinCapitalLetterR),
            '󠁓' => Ok(Tags::TagLatinCapitalLetterS),
            '󠁔' => Ok(Tags::TagLatinCapitalLetterT),
            '󠁕' => Ok(Tags::TagLatinCapitalLetterU),
            '󠁖' => Ok(Tags::TagLatinCapitalLetterV),
            '󠁗' => Ok(Tags::TagLatinCapitalLetterW),
            '󠁘' => Ok(Tags::TagLatinCapitalLetterX),
            '󠁙' => Ok(Tags::TagLatinCapitalLetterY),
            '󠁚' => Ok(Tags::TagLatinCapitalLetterZ),
            '󠁛' => Ok(Tags::TagLeftSquareBracket),
            '󠁜' => Ok(Tags::TagReverseSolidus),
            '󠁝' => Ok(Tags::TagRightSquareBracket),
            '󠁞' => Ok(Tags::TagCircumflexAccent),
            '󠁟' => Ok(Tags::TagLowLine),
            '󠁠' => Ok(Tags::TagGraveAccent),
            '󠁡' => Ok(Tags::TagLatinSmallLetterA),
            '󠁢' => Ok(Tags::TagLatinSmallLetterB),
            '󠁣' => Ok(Tags::TagLatinSmallLetterC),
            '󠁤' => Ok(Tags::TagLatinSmallLetterD),
            '󠁥' => Ok(Tags::TagLatinSmallLetterE),
            '󠁦' => Ok(Tags::TagLatinSmallLetterF),
            '󠁧' => Ok(Tags::TagLatinSmallLetterG),
            '󠁨' => Ok(Tags::TagLatinSmallLetterH),
            '󠁩' => Ok(Tags::TagLatinSmallLetterI),
            '󠁪' => Ok(Tags::TagLatinSmallLetterJ),
            '󠁫' => Ok(Tags::TagLatinSmallLetterK),
            '󠁬' => Ok(Tags::TagLatinSmallLetterL),
            '󠁭' => Ok(Tags::TagLatinSmallLetterM),
            '󠁮' => Ok(Tags::TagLatinSmallLetterN),
            '󠁯' => Ok(Tags::TagLatinSmallLetterO),
            '󠁰' => Ok(Tags::TagLatinSmallLetterP),
            '󠁱' => Ok(Tags::TagLatinSmallLetterQ),
            '󠁲' => Ok(Tags::TagLatinSmallLetterR),
            '󠁳' => Ok(Tags::TagLatinSmallLetterS),
            '󠁴' => Ok(Tags::TagLatinSmallLetterT),
            '󠁵' => Ok(Tags::TagLatinSmallLetterU),
            '󠁶' => Ok(Tags::TagLatinSmallLetterV),
            '󠁷' => Ok(Tags::TagLatinSmallLetterW),
            '󠁸' => Ok(Tags::TagLatinSmallLetterX),
            '󠁹' => Ok(Tags::TagLatinSmallLetterY),
            '󠁺' => Ok(Tags::TagLatinSmallLetterZ),
            '󠁻' => Ok(Tags::TagLeftCurlyBracket),
            '󠁼' => Ok(Tags::TagVerticalLine),
            '󠁽' => Ok(Tags::TagRightCurlyBracket),
            '󠁾' => Ok(Tags::TagTilde),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Tags {
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

impl std::convert::TryFrom<u32> for Tags {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Tags {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Tags {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Tags::LanguageTag
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Tags::LanguageTag => "language tag",
            Tags::TagSpace => "tag space",
            Tags::TagExclamationMark => "tag exclamation mark",
            Tags::TagQuotationMark => "tag quotation mark",
            Tags::TagNumberSign => "tag number sign",
            Tags::TagDollarSign => "tag dollar sign",
            Tags::TagPercentSign => "tag percent sign",
            Tags::TagAmpersand => "tag ampersand",
            Tags::TagApostrophe => "tag apostrophe",
            Tags::TagLeftParenthesis => "tag left parenthesis",
            Tags::TagRightParenthesis => "tag right parenthesis",
            Tags::TagAsterisk => "tag asterisk",
            Tags::TagPlusSign => "tag plus sign",
            Tags::TagComma => "tag comma",
            Tags::TagHyphenDashMinus => "tag hyphen-minus",
            Tags::TagFullStop => "tag full stop",
            Tags::TagSolidus => "tag solidus",
            Tags::TagDigitZero => "tag digit zero",
            Tags::TagDigitOne => "tag digit one",
            Tags::TagDigitTwo => "tag digit two",
            Tags::TagDigitThree => "tag digit three",
            Tags::TagDigitFour => "tag digit four",
            Tags::TagDigitFive => "tag digit five",
            Tags::TagDigitSix => "tag digit six",
            Tags::TagDigitSeven => "tag digit seven",
            Tags::TagDigitEight => "tag digit eight",
            Tags::TagDigitNine => "tag digit nine",
            Tags::TagColon => "tag colon",
            Tags::TagSemicolon => "tag semicolon",
            Tags::TagLessDashThanSign => "tag less-than sign",
            Tags::TagEqualsSign => "tag equals sign",
            Tags::TagGreaterDashThanSign => "tag greater-than sign",
            Tags::TagQuestionMark => "tag question mark",
            Tags::TagCommercialAt => "tag commercial at",
            Tags::TagLatinCapitalLetterA => "tag latin capital letter a",
            Tags::TagLatinCapitalLetterB => "tag latin capital letter b",
            Tags::TagLatinCapitalLetterC => "tag latin capital letter c",
            Tags::TagLatinCapitalLetterD => "tag latin capital letter d",
            Tags::TagLatinCapitalLetterE => "tag latin capital letter e",
            Tags::TagLatinCapitalLetterF => "tag latin capital letter f",
            Tags::TagLatinCapitalLetterG => "tag latin capital letter g",
            Tags::TagLatinCapitalLetterH => "tag latin capital letter h",
            Tags::TagLatinCapitalLetterI => "tag latin capital letter i",
            Tags::TagLatinCapitalLetterJ => "tag latin capital letter j",
            Tags::TagLatinCapitalLetterK => "tag latin capital letter k",
            Tags::TagLatinCapitalLetterL => "tag latin capital letter l",
            Tags::TagLatinCapitalLetterM => "tag latin capital letter m",
            Tags::TagLatinCapitalLetterN => "tag latin capital letter n",
            Tags::TagLatinCapitalLetterO => "tag latin capital letter o",
            Tags::TagLatinCapitalLetterP => "tag latin capital letter p",
            Tags::TagLatinCapitalLetterQ => "tag latin capital letter q",
            Tags::TagLatinCapitalLetterR => "tag latin capital letter r",
            Tags::TagLatinCapitalLetterS => "tag latin capital letter s",
            Tags::TagLatinCapitalLetterT => "tag latin capital letter t",
            Tags::TagLatinCapitalLetterU => "tag latin capital letter u",
            Tags::TagLatinCapitalLetterV => "tag latin capital letter v",
            Tags::TagLatinCapitalLetterW => "tag latin capital letter w",
            Tags::TagLatinCapitalLetterX => "tag latin capital letter x",
            Tags::TagLatinCapitalLetterY => "tag latin capital letter y",
            Tags::TagLatinCapitalLetterZ => "tag latin capital letter z",
            Tags::TagLeftSquareBracket => "tag left square bracket",
            Tags::TagReverseSolidus => "tag reverse solidus",
            Tags::TagRightSquareBracket => "tag right square bracket",
            Tags::TagCircumflexAccent => "tag circumflex accent",
            Tags::TagLowLine => "tag low line",
            Tags::TagGraveAccent => "tag grave accent",
            Tags::TagLatinSmallLetterA => "tag latin small letter a",
            Tags::TagLatinSmallLetterB => "tag latin small letter b",
            Tags::TagLatinSmallLetterC => "tag latin small letter c",
            Tags::TagLatinSmallLetterD => "tag latin small letter d",
            Tags::TagLatinSmallLetterE => "tag latin small letter e",
            Tags::TagLatinSmallLetterF => "tag latin small letter f",
            Tags::TagLatinSmallLetterG => "tag latin small letter g",
            Tags::TagLatinSmallLetterH => "tag latin small letter h",
            Tags::TagLatinSmallLetterI => "tag latin small letter i",
            Tags::TagLatinSmallLetterJ => "tag latin small letter j",
            Tags::TagLatinSmallLetterK => "tag latin small letter k",
            Tags::TagLatinSmallLetterL => "tag latin small letter l",
            Tags::TagLatinSmallLetterM => "tag latin small letter m",
            Tags::TagLatinSmallLetterN => "tag latin small letter n",
            Tags::TagLatinSmallLetterO => "tag latin small letter o",
            Tags::TagLatinSmallLetterP => "tag latin small letter p",
            Tags::TagLatinSmallLetterQ => "tag latin small letter q",
            Tags::TagLatinSmallLetterR => "tag latin small letter r",
            Tags::TagLatinSmallLetterS => "tag latin small letter s",
            Tags::TagLatinSmallLetterT => "tag latin small letter t",
            Tags::TagLatinSmallLetterU => "tag latin small letter u",
            Tags::TagLatinSmallLetterV => "tag latin small letter v",
            Tags::TagLatinSmallLetterW => "tag latin small letter w",
            Tags::TagLatinSmallLetterX => "tag latin small letter x",
            Tags::TagLatinSmallLetterY => "tag latin small letter y",
            Tags::TagLatinSmallLetterZ => "tag latin small letter z",
            Tags::TagLeftCurlyBracket => "tag left curly bracket",
            Tags::TagVerticalLine => "tag vertical line",
            Tags::TagRightCurlyBracket => "tag right curly bracket",
            Tags::TagTilde => "tag tilde",
        }
    }
}
