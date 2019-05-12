
/// An enum to represent all characters in the SupplementalPunctuation block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SupplementalPunctuation {
    /// \u{2e00}: '⸀'
    RightAngleSubstitutionMarker,
    /// \u{2e01}: '⸁'
    RightAngleDottedSubstitutionMarker,
    /// \u{2e02}: '⸂'
    LeftSubstitutionBracket,
    /// \u{2e03}: '⸃'
    RightSubstitutionBracket,
    /// \u{2e04}: '⸄'
    LeftDottedSubstitutionBracket,
    /// \u{2e05}: '⸅'
    RightDottedSubstitutionBracket,
    /// \u{2e06}: '⸆'
    RaisedInterpolationMarker,
    /// \u{2e07}: '⸇'
    RaisedDottedInterpolationMarker,
    /// \u{2e08}: '⸈'
    DottedTranspositionMarker,
    /// \u{2e09}: '⸉'
    LeftTranspositionBracket,
    /// \u{2e0a}: '⸊'
    RightTranspositionBracket,
    /// \u{2e0b}: '⸋'
    RaisedSquare,
    /// \u{2e0c}: '⸌'
    LeftRaisedOmissionBracket,
    /// \u{2e0d}: '⸍'
    RightRaisedOmissionBracket,
    /// \u{2e0e}: '⸎'
    EditorialCoronis,
    /// \u{2e0f}: '⸏'
    Paragraphos,
    /// \u{2e10}: '⸐'
    ForkedParagraphos,
    /// \u{2e11}: '⸑'
    ReversedForkedParagraphos,
    /// \u{2e12}: '⸒'
    Hypodiastole,
    /// \u{2e13}: '⸓'
    DottedObelos,
    /// \u{2e14}: '⸔'
    DownwardsAncora,
    /// \u{2e15}: '⸕'
    UpwardsAncora,
    /// \u{2e16}: '⸖'
    DottedRightDashPointingAngle,
    /// \u{2e17}: '⸗'
    DoubleObliqueHyphen,
    /// \u{2e18}: '⸘'
    InvertedInterrobang,
    /// \u{2e19}: '⸙'
    PalmBranch,
    /// \u{2e1a}: '⸚'
    HyphenWithDiaeresis,
    /// \u{2e1b}: '⸛'
    TildeWithRingAbove,
    /// \u{2e1c}: '⸜'
    LeftLowParaphraseBracket,
    /// \u{2e1d}: '⸝'
    RightLowParaphraseBracket,
    /// \u{2e1e}: '⸞'
    TildeWithDotAbove,
    /// \u{2e1f}: '⸟'
    TildeWithDotBelow,
    /// \u{2e20}: '⸠'
    LeftVerticalBarWithQuill,
    /// \u{2e21}: '⸡'
    RightVerticalBarWithQuill,
    /// \u{2e22}: '⸢'
    TopLeftHalfBracket,
    /// \u{2e23}: '⸣'
    TopRightHalfBracket,
    /// \u{2e24}: '⸤'
    BottomLeftHalfBracket,
    /// \u{2e25}: '⸥'
    BottomRightHalfBracket,
    /// \u{2e26}: '⸦'
    LeftSidewaysUBracket,
    /// \u{2e27}: '⸧'
    RightSidewaysUBracket,
    /// \u{2e28}: '⸨'
    LeftDoubleParenthesis,
    /// \u{2e29}: '⸩'
    RightDoubleParenthesis,
    /// \u{2e2a}: '⸪'
    TwoDotsOverOneDotPunctuation,
    /// \u{2e2b}: '⸫'
    OneDotOverTwoDotsPunctuation,
    /// \u{2e2c}: '⸬'
    SquaredFourDotPunctuation,
    /// \u{2e2d}: '⸭'
    FiveDotMark,
    /// \u{2e2e}: '⸮'
    ReversedQuestionMark,
    /// \u{2e2f}: 'ⸯ'
    VerticalTilde,
    /// \u{2e30}: '⸰'
    RingPoint,
    /// \u{2e31}: '⸱'
    WordSeparatorMiddleDot,
    /// \u{2e32}: '⸲'
    TurnedComma,
    /// \u{2e33}: '⸳'
    RaisedDot,
    /// \u{2e34}: '⸴'
    RaisedComma,
    /// \u{2e35}: '⸵'
    TurnedSemicolon,
    /// \u{2e36}: '⸶'
    DaggerWithLeftGuard,
    /// \u{2e37}: '⸷'
    DaggerWithRightGuard,
    /// \u{2e38}: '⸸'
    TurnedDagger,
    /// \u{2e39}: '⸹'
    TopHalfSectionSign,
    /// \u{2e3a}: '⸺'
    TwoDashEmDash,
    /// \u{2e3b}: '⸻'
    ThreeDashEmDash,
    /// \u{2e3c}: '⸼'
    StenographicFullStop,
    /// \u{2e3d}: '⸽'
    VerticalSixDots,
    /// \u{2e3e}: '⸾'
    WigglyVerticalLine,
    /// \u{2e3f}: '⸿'
    Capitulum,
    /// \u{2e40}: '⹀'
    DoubleHyphen,
    /// \u{2e41}: '⹁'
    ReversedComma,
    /// \u{2e42}: '⹂'
    DoubleLowDashReversedDash9QuotationMark,
    /// \u{2e43}: '⹃'
    DashWithLeftUpturn,
    /// \u{2e44}: '⹄'
    DoubleSuspensionMark,
    /// \u{2e45}: '⹅'
    InvertedLowKavyka,
    /// \u{2e46}: '⹆'
    InvertedLowKavykaWithKavykaAbove,
    /// \u{2e47}: '⹇'
    LowKavyka,
    /// \u{2e48}: '⹈'
    LowKavykaWithDot,
    /// \u{2e49}: '⹉'
    DoubleStackedComma,
    /// \u{2e4a}: '⹊'
    DottedSolidus,
    /// \u{2e4b}: '⹋'
    TripleDagger,
    /// \u{2e4c}: '⹌'
    MedievalComma,
    /// \u{2e4d}: '⹍'
    ParagraphusMark,
    /// \u{2e4e}: '⹎'
    PunctusElevatusMark,
    /// \u{2e4f}: '⹏'
    CornishVerseDivider,
}

impl Into<char> for SupplementalPunctuation {
    fn into(self) -> char {
        match self {
            SupplementalPunctuation::RightAngleSubstitutionMarker => '⸀',
            SupplementalPunctuation::RightAngleDottedSubstitutionMarker => '⸁',
            SupplementalPunctuation::LeftSubstitutionBracket => '⸂',
            SupplementalPunctuation::RightSubstitutionBracket => '⸃',
            SupplementalPunctuation::LeftDottedSubstitutionBracket => '⸄',
            SupplementalPunctuation::RightDottedSubstitutionBracket => '⸅',
            SupplementalPunctuation::RaisedInterpolationMarker => '⸆',
            SupplementalPunctuation::RaisedDottedInterpolationMarker => '⸇',
            SupplementalPunctuation::DottedTranspositionMarker => '⸈',
            SupplementalPunctuation::LeftTranspositionBracket => '⸉',
            SupplementalPunctuation::RightTranspositionBracket => '⸊',
            SupplementalPunctuation::RaisedSquare => '⸋',
            SupplementalPunctuation::LeftRaisedOmissionBracket => '⸌',
            SupplementalPunctuation::RightRaisedOmissionBracket => '⸍',
            SupplementalPunctuation::EditorialCoronis => '⸎',
            SupplementalPunctuation::Paragraphos => '⸏',
            SupplementalPunctuation::ForkedParagraphos => '⸐',
            SupplementalPunctuation::ReversedForkedParagraphos => '⸑',
            SupplementalPunctuation::Hypodiastole => '⸒',
            SupplementalPunctuation::DottedObelos => '⸓',
            SupplementalPunctuation::DownwardsAncora => '⸔',
            SupplementalPunctuation::UpwardsAncora => '⸕',
            SupplementalPunctuation::DottedRightDashPointingAngle => '⸖',
            SupplementalPunctuation::DoubleObliqueHyphen => '⸗',
            SupplementalPunctuation::InvertedInterrobang => '⸘',
            SupplementalPunctuation::PalmBranch => '⸙',
            SupplementalPunctuation::HyphenWithDiaeresis => '⸚',
            SupplementalPunctuation::TildeWithRingAbove => '⸛',
            SupplementalPunctuation::LeftLowParaphraseBracket => '⸜',
            SupplementalPunctuation::RightLowParaphraseBracket => '⸝',
            SupplementalPunctuation::TildeWithDotAbove => '⸞',
            SupplementalPunctuation::TildeWithDotBelow => '⸟',
            SupplementalPunctuation::LeftVerticalBarWithQuill => '⸠',
            SupplementalPunctuation::RightVerticalBarWithQuill => '⸡',
            SupplementalPunctuation::TopLeftHalfBracket => '⸢',
            SupplementalPunctuation::TopRightHalfBracket => '⸣',
            SupplementalPunctuation::BottomLeftHalfBracket => '⸤',
            SupplementalPunctuation::BottomRightHalfBracket => '⸥',
            SupplementalPunctuation::LeftSidewaysUBracket => '⸦',
            SupplementalPunctuation::RightSidewaysUBracket => '⸧',
            SupplementalPunctuation::LeftDoubleParenthesis => '⸨',
            SupplementalPunctuation::RightDoubleParenthesis => '⸩',
            SupplementalPunctuation::TwoDotsOverOneDotPunctuation => '⸪',
            SupplementalPunctuation::OneDotOverTwoDotsPunctuation => '⸫',
            SupplementalPunctuation::SquaredFourDotPunctuation => '⸬',
            SupplementalPunctuation::FiveDotMark => '⸭',
            SupplementalPunctuation::ReversedQuestionMark => '⸮',
            SupplementalPunctuation::VerticalTilde => 'ⸯ',
            SupplementalPunctuation::RingPoint => '⸰',
            SupplementalPunctuation::WordSeparatorMiddleDot => '⸱',
            SupplementalPunctuation::TurnedComma => '⸲',
            SupplementalPunctuation::RaisedDot => '⸳',
            SupplementalPunctuation::RaisedComma => '⸴',
            SupplementalPunctuation::TurnedSemicolon => '⸵',
            SupplementalPunctuation::DaggerWithLeftGuard => '⸶',
            SupplementalPunctuation::DaggerWithRightGuard => '⸷',
            SupplementalPunctuation::TurnedDagger => '⸸',
            SupplementalPunctuation::TopHalfSectionSign => '⸹',
            SupplementalPunctuation::TwoDashEmDash => '⸺',
            SupplementalPunctuation::ThreeDashEmDash => '⸻',
            SupplementalPunctuation::StenographicFullStop => '⸼',
            SupplementalPunctuation::VerticalSixDots => '⸽',
            SupplementalPunctuation::WigglyVerticalLine => '⸾',
            SupplementalPunctuation::Capitulum => '⸿',
            SupplementalPunctuation::DoubleHyphen => '⹀',
            SupplementalPunctuation::ReversedComma => '⹁',
            SupplementalPunctuation::DoubleLowDashReversedDash9QuotationMark => '⹂',
            SupplementalPunctuation::DashWithLeftUpturn => '⹃',
            SupplementalPunctuation::DoubleSuspensionMark => '⹄',
            SupplementalPunctuation::InvertedLowKavyka => '⹅',
            SupplementalPunctuation::InvertedLowKavykaWithKavykaAbove => '⹆',
            SupplementalPunctuation::LowKavyka => '⹇',
            SupplementalPunctuation::LowKavykaWithDot => '⹈',
            SupplementalPunctuation::DoubleStackedComma => '⹉',
            SupplementalPunctuation::DottedSolidus => '⹊',
            SupplementalPunctuation::TripleDagger => '⹋',
            SupplementalPunctuation::MedievalComma => '⹌',
            SupplementalPunctuation::ParagraphusMark => '⹍',
            SupplementalPunctuation::PunctusElevatusMark => '⹎',
            SupplementalPunctuation::CornishVerseDivider => '⹏',
        }
    }
}

impl std::convert::TryFrom<char> for SupplementalPunctuation {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '⸀' => Ok(SupplementalPunctuation::RightAngleSubstitutionMarker),
            '⸁' => Ok(SupplementalPunctuation::RightAngleDottedSubstitutionMarker),
            '⸂' => Ok(SupplementalPunctuation::LeftSubstitutionBracket),
            '⸃' => Ok(SupplementalPunctuation::RightSubstitutionBracket),
            '⸄' => Ok(SupplementalPunctuation::LeftDottedSubstitutionBracket),
            '⸅' => Ok(SupplementalPunctuation::RightDottedSubstitutionBracket),
            '⸆' => Ok(SupplementalPunctuation::RaisedInterpolationMarker),
            '⸇' => Ok(SupplementalPunctuation::RaisedDottedInterpolationMarker),
            '⸈' => Ok(SupplementalPunctuation::DottedTranspositionMarker),
            '⸉' => Ok(SupplementalPunctuation::LeftTranspositionBracket),
            '⸊' => Ok(SupplementalPunctuation::RightTranspositionBracket),
            '⸋' => Ok(SupplementalPunctuation::RaisedSquare),
            '⸌' => Ok(SupplementalPunctuation::LeftRaisedOmissionBracket),
            '⸍' => Ok(SupplementalPunctuation::RightRaisedOmissionBracket),
            '⸎' => Ok(SupplementalPunctuation::EditorialCoronis),
            '⸏' => Ok(SupplementalPunctuation::Paragraphos),
            '⸐' => Ok(SupplementalPunctuation::ForkedParagraphos),
            '⸑' => Ok(SupplementalPunctuation::ReversedForkedParagraphos),
            '⸒' => Ok(SupplementalPunctuation::Hypodiastole),
            '⸓' => Ok(SupplementalPunctuation::DottedObelos),
            '⸔' => Ok(SupplementalPunctuation::DownwardsAncora),
            '⸕' => Ok(SupplementalPunctuation::UpwardsAncora),
            '⸖' => Ok(SupplementalPunctuation::DottedRightDashPointingAngle),
            '⸗' => Ok(SupplementalPunctuation::DoubleObliqueHyphen),
            '⸘' => Ok(SupplementalPunctuation::InvertedInterrobang),
            '⸙' => Ok(SupplementalPunctuation::PalmBranch),
            '⸚' => Ok(SupplementalPunctuation::HyphenWithDiaeresis),
            '⸛' => Ok(SupplementalPunctuation::TildeWithRingAbove),
            '⸜' => Ok(SupplementalPunctuation::LeftLowParaphraseBracket),
            '⸝' => Ok(SupplementalPunctuation::RightLowParaphraseBracket),
            '⸞' => Ok(SupplementalPunctuation::TildeWithDotAbove),
            '⸟' => Ok(SupplementalPunctuation::TildeWithDotBelow),
            '⸠' => Ok(SupplementalPunctuation::LeftVerticalBarWithQuill),
            '⸡' => Ok(SupplementalPunctuation::RightVerticalBarWithQuill),
            '⸢' => Ok(SupplementalPunctuation::TopLeftHalfBracket),
            '⸣' => Ok(SupplementalPunctuation::TopRightHalfBracket),
            '⸤' => Ok(SupplementalPunctuation::BottomLeftHalfBracket),
            '⸥' => Ok(SupplementalPunctuation::BottomRightHalfBracket),
            '⸦' => Ok(SupplementalPunctuation::LeftSidewaysUBracket),
            '⸧' => Ok(SupplementalPunctuation::RightSidewaysUBracket),
            '⸨' => Ok(SupplementalPunctuation::LeftDoubleParenthesis),
            '⸩' => Ok(SupplementalPunctuation::RightDoubleParenthesis),
            '⸪' => Ok(SupplementalPunctuation::TwoDotsOverOneDotPunctuation),
            '⸫' => Ok(SupplementalPunctuation::OneDotOverTwoDotsPunctuation),
            '⸬' => Ok(SupplementalPunctuation::SquaredFourDotPunctuation),
            '⸭' => Ok(SupplementalPunctuation::FiveDotMark),
            '⸮' => Ok(SupplementalPunctuation::ReversedQuestionMark),
            'ⸯ' => Ok(SupplementalPunctuation::VerticalTilde),
            '⸰' => Ok(SupplementalPunctuation::RingPoint),
            '⸱' => Ok(SupplementalPunctuation::WordSeparatorMiddleDot),
            '⸲' => Ok(SupplementalPunctuation::TurnedComma),
            '⸳' => Ok(SupplementalPunctuation::RaisedDot),
            '⸴' => Ok(SupplementalPunctuation::RaisedComma),
            '⸵' => Ok(SupplementalPunctuation::TurnedSemicolon),
            '⸶' => Ok(SupplementalPunctuation::DaggerWithLeftGuard),
            '⸷' => Ok(SupplementalPunctuation::DaggerWithRightGuard),
            '⸸' => Ok(SupplementalPunctuation::TurnedDagger),
            '⸹' => Ok(SupplementalPunctuation::TopHalfSectionSign),
            '⸺' => Ok(SupplementalPunctuation::TwoDashEmDash),
            '⸻' => Ok(SupplementalPunctuation::ThreeDashEmDash),
            '⸼' => Ok(SupplementalPunctuation::StenographicFullStop),
            '⸽' => Ok(SupplementalPunctuation::VerticalSixDots),
            '⸾' => Ok(SupplementalPunctuation::WigglyVerticalLine),
            '⸿' => Ok(SupplementalPunctuation::Capitulum),
            '⹀' => Ok(SupplementalPunctuation::DoubleHyphen),
            '⹁' => Ok(SupplementalPunctuation::ReversedComma),
            '⹂' => Ok(SupplementalPunctuation::DoubleLowDashReversedDash9QuotationMark),
            '⹃' => Ok(SupplementalPunctuation::DashWithLeftUpturn),
            '⹄' => Ok(SupplementalPunctuation::DoubleSuspensionMark),
            '⹅' => Ok(SupplementalPunctuation::InvertedLowKavyka),
            '⹆' => Ok(SupplementalPunctuation::InvertedLowKavykaWithKavykaAbove),
            '⹇' => Ok(SupplementalPunctuation::LowKavyka),
            '⹈' => Ok(SupplementalPunctuation::LowKavykaWithDot),
            '⹉' => Ok(SupplementalPunctuation::DoubleStackedComma),
            '⹊' => Ok(SupplementalPunctuation::DottedSolidus),
            '⹋' => Ok(SupplementalPunctuation::TripleDagger),
            '⹌' => Ok(SupplementalPunctuation::MedievalComma),
            '⹍' => Ok(SupplementalPunctuation::ParagraphusMark),
            '⹎' => Ok(SupplementalPunctuation::PunctusElevatusMark),
            '⹏' => Ok(SupplementalPunctuation::CornishVerseDivider),
            _ => Err(()),
        }
    }
}

impl Into<u32> for SupplementalPunctuation {
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

impl std::convert::TryFrom<u32> for SupplementalPunctuation {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for SupplementalPunctuation {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl SupplementalPunctuation {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        SupplementalPunctuation::RightAngleSubstitutionMarker
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("SupplementalPunctuation{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
