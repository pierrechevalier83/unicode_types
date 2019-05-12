
/// An enum to represent all characters in the GeneralPunctuation block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum GeneralPunctuation {
    /// \u{2000}: ' '
    EnQuad,
    /// \u{2001}: ' '
    EmQuad,
    /// \u{2002}: ' '
    EnSpace,
    /// \u{2003}: ' '
    EmSpace,
    /// \u{2004}: ' '
    ThreeDashPerDashEmSpace,
    /// \u{2005}: ' '
    FourDashPerDashEmSpace,
    /// \u{2006}: ' '
    SixDashPerDashEmSpace,
    /// \u{2007}: ' '
    FigureSpace,
    /// \u{2008}: ' '
    PunctuationSpace,
    /// \u{2009}: ' '
    ThinSpace,
    /// \u{200a}: ' '
    HairSpace,
    /// \u{200b}: '​'
    ZeroWidthSpace,
    /// \u{200c}: '‌'
    ZeroWidthNonDashJoiner,
    /// \u{200d}: '‍'
    ZeroWidthJoiner,
    /// \u{200e}: '‎'
    LeftDashToDashRightMark,
    /// \u{200f}: '‏'
    RightDashToDashLeftMark,
    /// \u{2010}: '‐'
    Hyphen,
    /// \u{2011}: '‑'
    NonDashBreakingHyphen,
    /// \u{2012}: '‒'
    FigureDash,
    /// \u{2013}: '–'
    EnDash,
    /// \u{2014}: '—'
    EmDash,
    /// \u{2015}: '―'
    HorizontalBar,
    /// \u{2016}: '‖'
    DoubleVerticalLine,
    /// \u{2017}: '‗'
    DoubleLowLine,
    /// \u{2018}: '‘'
    LeftSingleQuotationMark,
    /// \u{2019}: '’'
    RightSingleQuotationMark,
    /// \u{201a}: '‚'
    SingleLowDash9QuotationMark,
    /// \u{201b}: '‛'
    SingleHighDashReversedDash9QuotationMark,
    /// \u{201c}: '“'
    LeftDoubleQuotationMark,
    /// \u{201d}: '”'
    RightDoubleQuotationMark,
    /// \u{201e}: '„'
    DoubleLowDash9QuotationMark,
    /// \u{201f}: '‟'
    DoubleHighDashReversedDash9QuotationMark,
    /// \u{2020}: '†'
    Dagger,
    /// \u{2021}: '‡'
    DoubleDagger,
    /// \u{2022}: '•'
    Bullet,
    /// \u{2023}: '‣'
    TriangularBullet,
    /// \u{2024}: '․'
    OneDotLeader,
    /// \u{2025}: '‥'
    TwoDotLeader,
    /// \u{2026}: '…'
    HorizontalEllipsis,
    /// \u{2027}: '‧'
    HyphenationPoint,
    /// \u{2028}: ' '
    LineSeparator,
    /// \u{2029}: ' '
    ParagraphSeparator,
    /// \u{202a}: '‪'
    LeftDashToDashRightEmbedding,
    /// \u{202b}: '‫'
    RightDashToDashLeftEmbedding,
    /// \u{202c}: '‬'
    PopDirectionalFormatting,
    /// \u{202d}: '‭'
    LeftDashToDashRightOverride,
    /// \u{202e}: '‮'
    RightDashToDashLeftOverride,
    /// \u{202f}: ' '
    NarrowNoDashBreakSpace,
    /// \u{2030}: '‰'
    PerMilleSign,
    /// \u{2031}: '‱'
    PerTenThousandSign,
    /// \u{2032}: '′'
    Prime,
    /// \u{2033}: '″'
    DoublePrime,
    /// \u{2034}: '‴'
    TriplePrime,
    /// \u{2035}: '‵'
    ReversedPrime,
    /// \u{2036}: '‶'
    ReversedDoublePrime,
    /// \u{2037}: '‷'
    ReversedTriplePrime,
    /// \u{2038}: '‸'
    Caret,
    /// \u{2039}: '‹'
    SingleLeftDashPointingAngleQuotationMark,
    /// \u{203a}: '›'
    SingleRightDashPointingAngleQuotationMark,
    /// \u{203b}: '※'
    ReferenceMark,
    /// \u{203c}: '‼'
    DoubleExclamationMark,
    /// \u{203d}: '‽'
    Interrobang,
    /// \u{203e}: '‾'
    Overline,
    /// \u{203f}: '‿'
    Undertie,
    /// \u{2040}: '⁀'
    CharacterTie,
    /// \u{2041}: '⁁'
    CaretInsertionPoint,
    /// \u{2042}: '⁂'
    Asterism,
    /// \u{2043}: '⁃'
    HyphenBullet,
    /// \u{2044}: '⁄'
    FractionSlash,
    /// \u{2045}: '⁅'
    LeftSquareBracketWithQuill,
    /// \u{2046}: '⁆'
    RightSquareBracketWithQuill,
    /// \u{2047}: '⁇'
    DoubleQuestionMark,
    /// \u{2048}: '⁈'
    QuestionExclamationMark,
    /// \u{2049}: '⁉'
    ExclamationQuestionMark,
    /// \u{204a}: '⁊'
    TironianSignEt,
    /// \u{204b}: '⁋'
    ReversedPilcrowSign,
    /// \u{204c}: '⁌'
    BlackLeftwardsBullet,
    /// \u{204d}: '⁍'
    BlackRightwardsBullet,
    /// \u{204e}: '⁎'
    LowAsterisk,
    /// \u{204f}: '⁏'
    ReversedSemicolon,
    /// \u{2050}: '⁐'
    CloseUp,
    /// \u{2051}: '⁑'
    TwoAsterisksAlignedVertically,
    /// \u{2052}: '⁒'
    CommercialMinusSign,
    /// \u{2053}: '⁓'
    SwungDash,
    /// \u{2054}: '⁔'
    InvertedUndertie,
    /// \u{2055}: '⁕'
    FlowerPunctuationMark,
    /// \u{2056}: '⁖'
    ThreeDotPunctuation,
    /// \u{2057}: '⁗'
    QuadruplePrime,
    /// \u{2058}: '⁘'
    FourDotPunctuation,
    /// \u{2059}: '⁙'
    FiveDotPunctuation,
    /// \u{205a}: '⁚'
    TwoDotPunctuation,
    /// \u{205b}: '⁛'
    FourDotMark,
    /// \u{205c}: '⁜'
    DottedCross,
    /// \u{205d}: '⁝'
    Tricolon,
    /// \u{205e}: '⁞'
    VerticalFourDots,
    /// \u{205f}: ' '
    MediumMathematicalSpace,
    /// \u{2060}: '⁠'
    WordJoiner,
    /// \u{2061}: '⁡'
    FunctionApplication,
    /// \u{2062}: '⁢'
    InvisibleTimes,
    /// \u{2063}: '⁣'
    InvisibleSeparator,
    /// \u{2064}: '⁤'
    InvisiblePlus,
    /// \u{2066}: '⁦'
    LeftDashToDashRightIsolate,
    /// \u{2067}: '⁧'
    RightDashToDashLeftIsolate,
    /// \u{2068}: '⁨'
    FirstStrongIsolate,
    /// \u{2069}: '⁩'
    PopDirectionalIsolate,
    /// \u{206a}: '⁪'
    InhibitSymmetricSwapping,
    /// \u{206b}: '⁫'
    ActivateSymmetricSwapping,
    /// \u{206c}: '⁬'
    InhibitArabicFormShaping,
    /// \u{206d}: '⁭'
    ActivateArabicFormShaping,
    /// \u{206e}: '⁮'
    NationalDigitShapes,
}

impl Into<char> for GeneralPunctuation {
    fn into(self) -> char {
        match self {
            GeneralPunctuation::EnQuad => ' ',
            GeneralPunctuation::EmQuad => ' ',
            GeneralPunctuation::EnSpace => ' ',
            GeneralPunctuation::EmSpace => ' ',
            GeneralPunctuation::ThreeDashPerDashEmSpace => ' ',
            GeneralPunctuation::FourDashPerDashEmSpace => ' ',
            GeneralPunctuation::SixDashPerDashEmSpace => ' ',
            GeneralPunctuation::FigureSpace => ' ',
            GeneralPunctuation::PunctuationSpace => ' ',
            GeneralPunctuation::ThinSpace => ' ',
            GeneralPunctuation::HairSpace => ' ',
            GeneralPunctuation::ZeroWidthSpace => '​',
            GeneralPunctuation::ZeroWidthNonDashJoiner => '‌',
            GeneralPunctuation::ZeroWidthJoiner => '‍',
            GeneralPunctuation::LeftDashToDashRightMark => '‎',
            GeneralPunctuation::RightDashToDashLeftMark => '‏',
            GeneralPunctuation::Hyphen => '‐',
            GeneralPunctuation::NonDashBreakingHyphen => '‑',
            GeneralPunctuation::FigureDash => '‒',
            GeneralPunctuation::EnDash => '–',
            GeneralPunctuation::EmDash => '—',
            GeneralPunctuation::HorizontalBar => '―',
            GeneralPunctuation::DoubleVerticalLine => '‖',
            GeneralPunctuation::DoubleLowLine => '‗',
            GeneralPunctuation::LeftSingleQuotationMark => '‘',
            GeneralPunctuation::RightSingleQuotationMark => '’',
            GeneralPunctuation::SingleLowDash9QuotationMark => '‚',
            GeneralPunctuation::SingleHighDashReversedDash9QuotationMark => '‛',
            GeneralPunctuation::LeftDoubleQuotationMark => '“',
            GeneralPunctuation::RightDoubleQuotationMark => '”',
            GeneralPunctuation::DoubleLowDash9QuotationMark => '„',
            GeneralPunctuation::DoubleHighDashReversedDash9QuotationMark => '‟',
            GeneralPunctuation::Dagger => '†',
            GeneralPunctuation::DoubleDagger => '‡',
            GeneralPunctuation::Bullet => '•',
            GeneralPunctuation::TriangularBullet => '‣',
            GeneralPunctuation::OneDotLeader => '․',
            GeneralPunctuation::TwoDotLeader => '‥',
            GeneralPunctuation::HorizontalEllipsis => '…',
            GeneralPunctuation::HyphenationPoint => '‧',
            GeneralPunctuation::LineSeparator => ' ',
            GeneralPunctuation::ParagraphSeparator => ' ',
            GeneralPunctuation::LeftDashToDashRightEmbedding => '‪',
            GeneralPunctuation::RightDashToDashLeftEmbedding => '‫',
            GeneralPunctuation::PopDirectionalFormatting => '‬',
            GeneralPunctuation::LeftDashToDashRightOverride => '‭',
            GeneralPunctuation::RightDashToDashLeftOverride => '‮',
            GeneralPunctuation::NarrowNoDashBreakSpace => ' ',
            GeneralPunctuation::PerMilleSign => '‰',
            GeneralPunctuation::PerTenThousandSign => '‱',
            GeneralPunctuation::Prime => '′',
            GeneralPunctuation::DoublePrime => '″',
            GeneralPunctuation::TriplePrime => '‴',
            GeneralPunctuation::ReversedPrime => '‵',
            GeneralPunctuation::ReversedDoublePrime => '‶',
            GeneralPunctuation::ReversedTriplePrime => '‷',
            GeneralPunctuation::Caret => '‸',
            GeneralPunctuation::SingleLeftDashPointingAngleQuotationMark => '‹',
            GeneralPunctuation::SingleRightDashPointingAngleQuotationMark => '›',
            GeneralPunctuation::ReferenceMark => '※',
            GeneralPunctuation::DoubleExclamationMark => '‼',
            GeneralPunctuation::Interrobang => '‽',
            GeneralPunctuation::Overline => '‾',
            GeneralPunctuation::Undertie => '‿',
            GeneralPunctuation::CharacterTie => '⁀',
            GeneralPunctuation::CaretInsertionPoint => '⁁',
            GeneralPunctuation::Asterism => '⁂',
            GeneralPunctuation::HyphenBullet => '⁃',
            GeneralPunctuation::FractionSlash => '⁄',
            GeneralPunctuation::LeftSquareBracketWithQuill => '⁅',
            GeneralPunctuation::RightSquareBracketWithQuill => '⁆',
            GeneralPunctuation::DoubleQuestionMark => '⁇',
            GeneralPunctuation::QuestionExclamationMark => '⁈',
            GeneralPunctuation::ExclamationQuestionMark => '⁉',
            GeneralPunctuation::TironianSignEt => '⁊',
            GeneralPunctuation::ReversedPilcrowSign => '⁋',
            GeneralPunctuation::BlackLeftwardsBullet => '⁌',
            GeneralPunctuation::BlackRightwardsBullet => '⁍',
            GeneralPunctuation::LowAsterisk => '⁎',
            GeneralPunctuation::ReversedSemicolon => '⁏',
            GeneralPunctuation::CloseUp => '⁐',
            GeneralPunctuation::TwoAsterisksAlignedVertically => '⁑',
            GeneralPunctuation::CommercialMinusSign => '⁒',
            GeneralPunctuation::SwungDash => '⁓',
            GeneralPunctuation::InvertedUndertie => '⁔',
            GeneralPunctuation::FlowerPunctuationMark => '⁕',
            GeneralPunctuation::ThreeDotPunctuation => '⁖',
            GeneralPunctuation::QuadruplePrime => '⁗',
            GeneralPunctuation::FourDotPunctuation => '⁘',
            GeneralPunctuation::FiveDotPunctuation => '⁙',
            GeneralPunctuation::TwoDotPunctuation => '⁚',
            GeneralPunctuation::FourDotMark => '⁛',
            GeneralPunctuation::DottedCross => '⁜',
            GeneralPunctuation::Tricolon => '⁝',
            GeneralPunctuation::VerticalFourDots => '⁞',
            GeneralPunctuation::MediumMathematicalSpace => ' ',
            GeneralPunctuation::WordJoiner => '⁠',
            GeneralPunctuation::FunctionApplication => '⁡',
            GeneralPunctuation::InvisibleTimes => '⁢',
            GeneralPunctuation::InvisibleSeparator => '⁣',
            GeneralPunctuation::InvisiblePlus => '⁤',
            GeneralPunctuation::LeftDashToDashRightIsolate => '⁦',
            GeneralPunctuation::RightDashToDashLeftIsolate => '⁧',
            GeneralPunctuation::FirstStrongIsolate => '⁨',
            GeneralPunctuation::PopDirectionalIsolate => '⁩',
            GeneralPunctuation::InhibitSymmetricSwapping => '⁪',
            GeneralPunctuation::ActivateSymmetricSwapping => '⁫',
            GeneralPunctuation::InhibitArabicFormShaping => '⁬',
            GeneralPunctuation::ActivateArabicFormShaping => '⁭',
            GeneralPunctuation::NationalDigitShapes => '⁮',
        }
    }
}

impl std::convert::TryFrom<char> for GeneralPunctuation {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            ' ' => Ok(GeneralPunctuation::EnQuad),
            ' ' => Ok(GeneralPunctuation::EmQuad),
            ' ' => Ok(GeneralPunctuation::EnSpace),
            ' ' => Ok(GeneralPunctuation::EmSpace),
            ' ' => Ok(GeneralPunctuation::ThreeDashPerDashEmSpace),
            ' ' => Ok(GeneralPunctuation::FourDashPerDashEmSpace),
            ' ' => Ok(GeneralPunctuation::SixDashPerDashEmSpace),
            ' ' => Ok(GeneralPunctuation::FigureSpace),
            ' ' => Ok(GeneralPunctuation::PunctuationSpace),
            ' ' => Ok(GeneralPunctuation::ThinSpace),
            ' ' => Ok(GeneralPunctuation::HairSpace),
            '​' => Ok(GeneralPunctuation::ZeroWidthSpace),
            '‌' => Ok(GeneralPunctuation::ZeroWidthNonDashJoiner),
            '‍' => Ok(GeneralPunctuation::ZeroWidthJoiner),
            '‎' => Ok(GeneralPunctuation::LeftDashToDashRightMark),
            '‏' => Ok(GeneralPunctuation::RightDashToDashLeftMark),
            '‐' => Ok(GeneralPunctuation::Hyphen),
            '‑' => Ok(GeneralPunctuation::NonDashBreakingHyphen),
            '‒' => Ok(GeneralPunctuation::FigureDash),
            '–' => Ok(GeneralPunctuation::EnDash),
            '—' => Ok(GeneralPunctuation::EmDash),
            '―' => Ok(GeneralPunctuation::HorizontalBar),
            '‖' => Ok(GeneralPunctuation::DoubleVerticalLine),
            '‗' => Ok(GeneralPunctuation::DoubleLowLine),
            '‘' => Ok(GeneralPunctuation::LeftSingleQuotationMark),
            '’' => Ok(GeneralPunctuation::RightSingleQuotationMark),
            '‚' => Ok(GeneralPunctuation::SingleLowDash9QuotationMark),
            '‛' => Ok(GeneralPunctuation::SingleHighDashReversedDash9QuotationMark),
            '“' => Ok(GeneralPunctuation::LeftDoubleQuotationMark),
            '”' => Ok(GeneralPunctuation::RightDoubleQuotationMark),
            '„' => Ok(GeneralPunctuation::DoubleLowDash9QuotationMark),
            '‟' => Ok(GeneralPunctuation::DoubleHighDashReversedDash9QuotationMark),
            '†' => Ok(GeneralPunctuation::Dagger),
            '‡' => Ok(GeneralPunctuation::DoubleDagger),
            '•' => Ok(GeneralPunctuation::Bullet),
            '‣' => Ok(GeneralPunctuation::TriangularBullet),
            '․' => Ok(GeneralPunctuation::OneDotLeader),
            '‥' => Ok(GeneralPunctuation::TwoDotLeader),
            '…' => Ok(GeneralPunctuation::HorizontalEllipsis),
            '‧' => Ok(GeneralPunctuation::HyphenationPoint),
            ' ' => Ok(GeneralPunctuation::LineSeparator),
            ' ' => Ok(GeneralPunctuation::ParagraphSeparator),
            '‪' => Ok(GeneralPunctuation::LeftDashToDashRightEmbedding),
            '‫' => Ok(GeneralPunctuation::RightDashToDashLeftEmbedding),
            '‬' => Ok(GeneralPunctuation::PopDirectionalFormatting),
            '‭' => Ok(GeneralPunctuation::LeftDashToDashRightOverride),
            '‮' => Ok(GeneralPunctuation::RightDashToDashLeftOverride),
            ' ' => Ok(GeneralPunctuation::NarrowNoDashBreakSpace),
            '‰' => Ok(GeneralPunctuation::PerMilleSign),
            '‱' => Ok(GeneralPunctuation::PerTenThousandSign),
            '′' => Ok(GeneralPunctuation::Prime),
            '″' => Ok(GeneralPunctuation::DoublePrime),
            '‴' => Ok(GeneralPunctuation::TriplePrime),
            '‵' => Ok(GeneralPunctuation::ReversedPrime),
            '‶' => Ok(GeneralPunctuation::ReversedDoublePrime),
            '‷' => Ok(GeneralPunctuation::ReversedTriplePrime),
            '‸' => Ok(GeneralPunctuation::Caret),
            '‹' => Ok(GeneralPunctuation::SingleLeftDashPointingAngleQuotationMark),
            '›' => Ok(GeneralPunctuation::SingleRightDashPointingAngleQuotationMark),
            '※' => Ok(GeneralPunctuation::ReferenceMark),
            '‼' => Ok(GeneralPunctuation::DoubleExclamationMark),
            '‽' => Ok(GeneralPunctuation::Interrobang),
            '‾' => Ok(GeneralPunctuation::Overline),
            '‿' => Ok(GeneralPunctuation::Undertie),
            '⁀' => Ok(GeneralPunctuation::CharacterTie),
            '⁁' => Ok(GeneralPunctuation::CaretInsertionPoint),
            '⁂' => Ok(GeneralPunctuation::Asterism),
            '⁃' => Ok(GeneralPunctuation::HyphenBullet),
            '⁄' => Ok(GeneralPunctuation::FractionSlash),
            '⁅' => Ok(GeneralPunctuation::LeftSquareBracketWithQuill),
            '⁆' => Ok(GeneralPunctuation::RightSquareBracketWithQuill),
            '⁇' => Ok(GeneralPunctuation::DoubleQuestionMark),
            '⁈' => Ok(GeneralPunctuation::QuestionExclamationMark),
            '⁉' => Ok(GeneralPunctuation::ExclamationQuestionMark),
            '⁊' => Ok(GeneralPunctuation::TironianSignEt),
            '⁋' => Ok(GeneralPunctuation::ReversedPilcrowSign),
            '⁌' => Ok(GeneralPunctuation::BlackLeftwardsBullet),
            '⁍' => Ok(GeneralPunctuation::BlackRightwardsBullet),
            '⁎' => Ok(GeneralPunctuation::LowAsterisk),
            '⁏' => Ok(GeneralPunctuation::ReversedSemicolon),
            '⁐' => Ok(GeneralPunctuation::CloseUp),
            '⁑' => Ok(GeneralPunctuation::TwoAsterisksAlignedVertically),
            '⁒' => Ok(GeneralPunctuation::CommercialMinusSign),
            '⁓' => Ok(GeneralPunctuation::SwungDash),
            '⁔' => Ok(GeneralPunctuation::InvertedUndertie),
            '⁕' => Ok(GeneralPunctuation::FlowerPunctuationMark),
            '⁖' => Ok(GeneralPunctuation::ThreeDotPunctuation),
            '⁗' => Ok(GeneralPunctuation::QuadruplePrime),
            '⁘' => Ok(GeneralPunctuation::FourDotPunctuation),
            '⁙' => Ok(GeneralPunctuation::FiveDotPunctuation),
            '⁚' => Ok(GeneralPunctuation::TwoDotPunctuation),
            '⁛' => Ok(GeneralPunctuation::FourDotMark),
            '⁜' => Ok(GeneralPunctuation::DottedCross),
            '⁝' => Ok(GeneralPunctuation::Tricolon),
            '⁞' => Ok(GeneralPunctuation::VerticalFourDots),
            ' ' => Ok(GeneralPunctuation::MediumMathematicalSpace),
            '⁠' => Ok(GeneralPunctuation::WordJoiner),
            '⁡' => Ok(GeneralPunctuation::FunctionApplication),
            '⁢' => Ok(GeneralPunctuation::InvisibleTimes),
            '⁣' => Ok(GeneralPunctuation::InvisibleSeparator),
            '⁤' => Ok(GeneralPunctuation::InvisiblePlus),
            '⁦' => Ok(GeneralPunctuation::LeftDashToDashRightIsolate),
            '⁧' => Ok(GeneralPunctuation::RightDashToDashLeftIsolate),
            '⁨' => Ok(GeneralPunctuation::FirstStrongIsolate),
            '⁩' => Ok(GeneralPunctuation::PopDirectionalIsolate),
            '⁪' => Ok(GeneralPunctuation::InhibitSymmetricSwapping),
            '⁫' => Ok(GeneralPunctuation::ActivateSymmetricSwapping),
            '⁬' => Ok(GeneralPunctuation::InhibitArabicFormShaping),
            '⁭' => Ok(GeneralPunctuation::ActivateArabicFormShaping),
            '⁮' => Ok(GeneralPunctuation::NationalDigitShapes),
            _ => Err(()),
        }
    }
}

impl Into<u32> for GeneralPunctuation {
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

impl std::convert::TryFrom<u32> for GeneralPunctuation {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for GeneralPunctuation {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl GeneralPunctuation {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        GeneralPunctuation::EnQuad
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("GeneralPunctuation{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
