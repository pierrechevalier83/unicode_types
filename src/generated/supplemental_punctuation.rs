/// \u{2e00} → \u{2e7f}
///
/// ⸀ ⸁ ⸂ ⸃ ⸄ ⸅ ⸆ ⸇ ⸈ ⸉ ⸊ ⸋ ⸌ ⸍ ⸎ ⸏\
/// ⸐ ⸑ ⸒ ⸓ ⸔ ⸕ ⸖ ⸗ ⸘ ⸙ ⸚ ⸛ ⸜ ⸝ ⸞ ⸟\
/// ⸠ ⸡ ⸢ ⸣ ⸤ ⸥ ⸦ ⸧ ⸨ ⸩ ⸪ ⸫ ⸬ ⸭ ⸮ ⸯ\
/// ⸰ ⸱ ⸲ ⸳ ⸴ ⸵ ⸶ ⸷ ⸸ ⸹ ⸺ ⸻ ⸼ ⸽ ⸾ ⸿\
/// ⹀ ⹁ ⹂ ⹃ ⹄ ⹅ ⹆ ⹇ ⹈ ⹉ ⹊ ⹋ ⹌ ⹍ ⹎ ⹏\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{2e00}: '⸀'
    pub const RIGHT_ANGLE_SUBSTITUTION_MARKER: char = '⸀';
    /// \u{2e01}: '⸁'
    pub const RIGHT_ANGLE_DOTTED_SUBSTITUTION_MARKER: char = '⸁';
    /// \u{2e02}: '⸂'
    pub const LEFT_SUBSTITUTION_BRACKET: char = '⸂';
    /// \u{2e03}: '⸃'
    pub const RIGHT_SUBSTITUTION_BRACKET: char = '⸃';
    /// \u{2e04}: '⸄'
    pub const LEFT_DOTTED_SUBSTITUTION_BRACKET: char = '⸄';
    /// \u{2e05}: '⸅'
    pub const RIGHT_DOTTED_SUBSTITUTION_BRACKET: char = '⸅';
    /// \u{2e06}: '⸆'
    pub const RAISED_INTERPOLATION_MARKER: char = '⸆';
    /// \u{2e07}: '⸇'
    pub const RAISED_DOTTED_INTERPOLATION_MARKER: char = '⸇';
    /// \u{2e08}: '⸈'
    pub const DOTTED_TRANSPOSITION_MARKER: char = '⸈';
    /// \u{2e09}: '⸉'
    pub const LEFT_TRANSPOSITION_BRACKET: char = '⸉';
    /// \u{2e0a}: '⸊'
    pub const RIGHT_TRANSPOSITION_BRACKET: char = '⸊';
    /// \u{2e0b}: '⸋'
    pub const RAISED_SQUARE: char = '⸋';
    /// \u{2e0c}: '⸌'
    pub const LEFT_RAISED_OMISSION_BRACKET: char = '⸌';
    /// \u{2e0d}: '⸍'
    pub const RIGHT_RAISED_OMISSION_BRACKET: char = '⸍';
    /// \u{2e0e}: '⸎'
    pub const EDITORIAL_CORONIS: char = '⸎';
    /// \u{2e0f}: '⸏'
    pub const PARAGRAPHOS: char = '⸏';
    /// \u{2e10}: '⸐'
    pub const FORKED_PARAGRAPHOS: char = '⸐';
    /// \u{2e11}: '⸑'
    pub const REVERSED_FORKED_PARAGRAPHOS: char = '⸑';
    /// \u{2e12}: '⸒'
    pub const HYPODIASTOLE: char = '⸒';
    /// \u{2e13}: '⸓'
    pub const DOTTED_OBELOS: char = '⸓';
    /// \u{2e14}: '⸔'
    pub const DOWNWARDS_ANCORA: char = '⸔';
    /// \u{2e15}: '⸕'
    pub const UPWARDS_ANCORA: char = '⸕';
    /// \u{2e16}: '⸖'
    pub const DOTTED_RIGHT_DASH_POINTING_ANGLE: char = '⸖';
    /// \u{2e17}: '⸗'
    pub const DOUBLE_OBLIQUE_HYPHEN: char = '⸗';
    /// \u{2e18}: '⸘'
    pub const INVERTED_INTERROBANG: char = '⸘';
    /// \u{2e19}: '⸙'
    pub const PALM_BRANCH: char = '⸙';
    /// \u{2e1a}: '⸚'
    pub const HYPHEN_WITH_DIAERESIS: char = '⸚';
    /// \u{2e1b}: '⸛'
    pub const TILDE_WITH_RING_ABOVE: char = '⸛';
    /// \u{2e1c}: '⸜'
    pub const LEFT_LOW_PARAPHRASE_BRACKET: char = '⸜';
    /// \u{2e1d}: '⸝'
    pub const RIGHT_LOW_PARAPHRASE_BRACKET: char = '⸝';
    /// \u{2e1e}: '⸞'
    pub const TILDE_WITH_DOT_ABOVE: char = '⸞';
    /// \u{2e1f}: '⸟'
    pub const TILDE_WITH_DOT_BELOW: char = '⸟';
    /// \u{2e20}: '⸠'
    pub const LEFT_VERTICAL_BAR_WITH_QUILL: char = '⸠';
    /// \u{2e21}: '⸡'
    pub const RIGHT_VERTICAL_BAR_WITH_QUILL: char = '⸡';
    /// \u{2e22}: '⸢'
    pub const TOP_LEFT_HALF_BRACKET: char = '⸢';
    /// \u{2e23}: '⸣'
    pub const TOP_RIGHT_HALF_BRACKET: char = '⸣';
    /// \u{2e24}: '⸤'
    pub const BOTTOM_LEFT_HALF_BRACKET: char = '⸤';
    /// \u{2e25}: '⸥'
    pub const BOTTOM_RIGHT_HALF_BRACKET: char = '⸥';
    /// \u{2e26}: '⸦'
    pub const LEFT_SIDEWAYS_U_BRACKET: char = '⸦';
    /// \u{2e27}: '⸧'
    pub const RIGHT_SIDEWAYS_U_BRACKET: char = '⸧';
    /// \u{2e28}: '⸨'
    pub const LEFT_DOUBLE_PARENTHESIS: char = '⸨';
    /// \u{2e29}: '⸩'
    pub const RIGHT_DOUBLE_PARENTHESIS: char = '⸩';
    /// \u{2e2a}: '⸪'
    pub const TWO_DOTS_OVER_ONE_DOT_PUNCTUATION: char = '⸪';
    /// \u{2e2b}: '⸫'
    pub const ONE_DOT_OVER_TWO_DOTS_PUNCTUATION: char = '⸫';
    /// \u{2e2c}: '⸬'
    pub const SQUARED_FOUR_DOT_PUNCTUATION: char = '⸬';
    /// \u{2e2d}: '⸭'
    pub const FIVE_DOT_MARK: char = '⸭';
    /// \u{2e2e}: '⸮'
    pub const REVERSED_QUESTION_MARK: char = '⸮';
    /// \u{2e2f}: 'ⸯ'
    pub const VERTICAL_TILDE: char = 'ⸯ';
    /// \u{2e30}: '⸰'
    pub const RING_POINT: char = '⸰';
    /// \u{2e31}: '⸱'
    pub const WORD_SEPARATOR_MIDDLE_DOT: char = '⸱';
    /// \u{2e32}: '⸲'
    pub const TURNED_COMMA: char = '⸲';
    /// \u{2e33}: '⸳'
    pub const RAISED_DOT: char = '⸳';
    /// \u{2e34}: '⸴'
    pub const RAISED_COMMA: char = '⸴';
    /// \u{2e35}: '⸵'
    pub const TURNED_SEMICOLON: char = '⸵';
    /// \u{2e36}: '⸶'
    pub const DAGGER_WITH_LEFT_GUARD: char = '⸶';
    /// \u{2e37}: '⸷'
    pub const DAGGER_WITH_RIGHT_GUARD: char = '⸷';
    /// \u{2e38}: '⸸'
    pub const TURNED_DAGGER: char = '⸸';
    /// \u{2e39}: '⸹'
    pub const TOP_HALF_SECTION_SIGN: char = '⸹';
    /// \u{2e3a}: '⸺'
    pub const TWO_DASH_EM_DASH: char = '⸺';
    /// \u{2e3b}: '⸻'
    pub const THREE_DASH_EM_DASH: char = '⸻';
    /// \u{2e3c}: '⸼'
    pub const STENOGRAPHIC_FULL_STOP: char = '⸼';
    /// \u{2e3d}: '⸽'
    pub const VERTICAL_SIX_DOTS: char = '⸽';
    /// \u{2e3e}: '⸾'
    pub const WIGGLY_VERTICAL_LINE: char = '⸾';
    /// \u{2e3f}: '⸿'
    pub const CAPITULUM: char = '⸿';
    /// \u{2e40}: '⹀'
    pub const DOUBLE_HYPHEN: char = '⹀';
    /// \u{2e41}: '⹁'
    pub const REVERSED_COMMA: char = '⹁';
    /// \u{2e42}: '⹂'
    pub const DOUBLE_LOW_DASH_REVERSED_DASH_9_QUOTATION_MARK: char = '⹂';
    /// \u{2e43}: '⹃'
    pub const DASH_WITH_LEFT_UPTURN: char = '⹃';
    /// \u{2e44}: '⹄'
    pub const DOUBLE_SUSPENSION_MARK: char = '⹄';
    /// \u{2e45}: '⹅'
    pub const INVERTED_LOW_KAVYKA: char = '⹅';
    /// \u{2e46}: '⹆'
    pub const INVERTED_LOW_KAVYKA_WITH_KAVYKA_ABOVE: char = '⹆';
    /// \u{2e47}: '⹇'
    pub const LOW_KAVYKA: char = '⹇';
    /// \u{2e48}: '⹈'
    pub const LOW_KAVYKA_WITH_DOT: char = '⹈';
    /// \u{2e49}: '⹉'
    pub const DOUBLE_STACKED_COMMA: char = '⹉';
    /// \u{2e4a}: '⹊'
    pub const DOTTED_SOLIDUS: char = '⹊';
    /// \u{2e4b}: '⹋'
    pub const TRIPLE_DAGGER: char = '⹋';
    /// \u{2e4c}: '⹌'
    pub const MEDIEVAL_COMMA: char = '⹌';
    /// \u{2e4d}: '⹍'
    pub const PARAGRAPHUS_MARK: char = '⹍';
    /// \u{2e4e}: '⹎'
    pub const PUNCTUS_ELEVATUS_MARK: char = '⹎';
    /// \u{2e4f}: '⹏'
    pub const CORNISH_VERSE_DIVIDER: char = '⹏';
}

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
        use constants::*;
        match self {
            SupplementalPunctuation::RightAngleSubstitutionMarker => RIGHT_ANGLE_SUBSTITUTION_MARKER,
            SupplementalPunctuation::RightAngleDottedSubstitutionMarker => RIGHT_ANGLE_DOTTED_SUBSTITUTION_MARKER,
            SupplementalPunctuation::LeftSubstitutionBracket => LEFT_SUBSTITUTION_BRACKET,
            SupplementalPunctuation::RightSubstitutionBracket => RIGHT_SUBSTITUTION_BRACKET,
            SupplementalPunctuation::LeftDottedSubstitutionBracket => LEFT_DOTTED_SUBSTITUTION_BRACKET,
            SupplementalPunctuation::RightDottedSubstitutionBracket => RIGHT_DOTTED_SUBSTITUTION_BRACKET,
            SupplementalPunctuation::RaisedInterpolationMarker => RAISED_INTERPOLATION_MARKER,
            SupplementalPunctuation::RaisedDottedInterpolationMarker => RAISED_DOTTED_INTERPOLATION_MARKER,
            SupplementalPunctuation::DottedTranspositionMarker => DOTTED_TRANSPOSITION_MARKER,
            SupplementalPunctuation::LeftTranspositionBracket => LEFT_TRANSPOSITION_BRACKET,
            SupplementalPunctuation::RightTranspositionBracket => RIGHT_TRANSPOSITION_BRACKET,
            SupplementalPunctuation::RaisedSquare => RAISED_SQUARE,
            SupplementalPunctuation::LeftRaisedOmissionBracket => LEFT_RAISED_OMISSION_BRACKET,
            SupplementalPunctuation::RightRaisedOmissionBracket => RIGHT_RAISED_OMISSION_BRACKET,
            SupplementalPunctuation::EditorialCoronis => EDITORIAL_CORONIS,
            SupplementalPunctuation::Paragraphos => PARAGRAPHOS,
            SupplementalPunctuation::ForkedParagraphos => FORKED_PARAGRAPHOS,
            SupplementalPunctuation::ReversedForkedParagraphos => REVERSED_FORKED_PARAGRAPHOS,
            SupplementalPunctuation::Hypodiastole => HYPODIASTOLE,
            SupplementalPunctuation::DottedObelos => DOTTED_OBELOS,
            SupplementalPunctuation::DownwardsAncora => DOWNWARDS_ANCORA,
            SupplementalPunctuation::UpwardsAncora => UPWARDS_ANCORA,
            SupplementalPunctuation::DottedRightDashPointingAngle => DOTTED_RIGHT_DASH_POINTING_ANGLE,
            SupplementalPunctuation::DoubleObliqueHyphen => DOUBLE_OBLIQUE_HYPHEN,
            SupplementalPunctuation::InvertedInterrobang => INVERTED_INTERROBANG,
            SupplementalPunctuation::PalmBranch => PALM_BRANCH,
            SupplementalPunctuation::HyphenWithDiaeresis => HYPHEN_WITH_DIAERESIS,
            SupplementalPunctuation::TildeWithRingAbove => TILDE_WITH_RING_ABOVE,
            SupplementalPunctuation::LeftLowParaphraseBracket => LEFT_LOW_PARAPHRASE_BRACKET,
            SupplementalPunctuation::RightLowParaphraseBracket => RIGHT_LOW_PARAPHRASE_BRACKET,
            SupplementalPunctuation::TildeWithDotAbove => TILDE_WITH_DOT_ABOVE,
            SupplementalPunctuation::TildeWithDotBelow => TILDE_WITH_DOT_BELOW,
            SupplementalPunctuation::LeftVerticalBarWithQuill => LEFT_VERTICAL_BAR_WITH_QUILL,
            SupplementalPunctuation::RightVerticalBarWithQuill => RIGHT_VERTICAL_BAR_WITH_QUILL,
            SupplementalPunctuation::TopLeftHalfBracket => TOP_LEFT_HALF_BRACKET,
            SupplementalPunctuation::TopRightHalfBracket => TOP_RIGHT_HALF_BRACKET,
            SupplementalPunctuation::BottomLeftHalfBracket => BOTTOM_LEFT_HALF_BRACKET,
            SupplementalPunctuation::BottomRightHalfBracket => BOTTOM_RIGHT_HALF_BRACKET,
            SupplementalPunctuation::LeftSidewaysUBracket => LEFT_SIDEWAYS_U_BRACKET,
            SupplementalPunctuation::RightSidewaysUBracket => RIGHT_SIDEWAYS_U_BRACKET,
            SupplementalPunctuation::LeftDoubleParenthesis => LEFT_DOUBLE_PARENTHESIS,
            SupplementalPunctuation::RightDoubleParenthesis => RIGHT_DOUBLE_PARENTHESIS,
            SupplementalPunctuation::TwoDotsOverOneDotPunctuation => TWO_DOTS_OVER_ONE_DOT_PUNCTUATION,
            SupplementalPunctuation::OneDotOverTwoDotsPunctuation => ONE_DOT_OVER_TWO_DOTS_PUNCTUATION,
            SupplementalPunctuation::SquaredFourDotPunctuation => SQUARED_FOUR_DOT_PUNCTUATION,
            SupplementalPunctuation::FiveDotMark => FIVE_DOT_MARK,
            SupplementalPunctuation::ReversedQuestionMark => REVERSED_QUESTION_MARK,
            SupplementalPunctuation::VerticalTilde => VERTICAL_TILDE,
            SupplementalPunctuation::RingPoint => RING_POINT,
            SupplementalPunctuation::WordSeparatorMiddleDot => WORD_SEPARATOR_MIDDLE_DOT,
            SupplementalPunctuation::TurnedComma => TURNED_COMMA,
            SupplementalPunctuation::RaisedDot => RAISED_DOT,
            SupplementalPunctuation::RaisedComma => RAISED_COMMA,
            SupplementalPunctuation::TurnedSemicolon => TURNED_SEMICOLON,
            SupplementalPunctuation::DaggerWithLeftGuard => DAGGER_WITH_LEFT_GUARD,
            SupplementalPunctuation::DaggerWithRightGuard => DAGGER_WITH_RIGHT_GUARD,
            SupplementalPunctuation::TurnedDagger => TURNED_DAGGER,
            SupplementalPunctuation::TopHalfSectionSign => TOP_HALF_SECTION_SIGN,
            SupplementalPunctuation::TwoDashEmDash => TWO_DASH_EM_DASH,
            SupplementalPunctuation::ThreeDashEmDash => THREE_DASH_EM_DASH,
            SupplementalPunctuation::StenographicFullStop => STENOGRAPHIC_FULL_STOP,
            SupplementalPunctuation::VerticalSixDots => VERTICAL_SIX_DOTS,
            SupplementalPunctuation::WigglyVerticalLine => WIGGLY_VERTICAL_LINE,
            SupplementalPunctuation::Capitulum => CAPITULUM,
            SupplementalPunctuation::DoubleHyphen => DOUBLE_HYPHEN,
            SupplementalPunctuation::ReversedComma => REVERSED_COMMA,
            SupplementalPunctuation::DoubleLowDashReversedDash9QuotationMark => DOUBLE_LOW_DASH_REVERSED_DASH_9_QUOTATION_MARK,
            SupplementalPunctuation::DashWithLeftUpturn => DASH_WITH_LEFT_UPTURN,
            SupplementalPunctuation::DoubleSuspensionMark => DOUBLE_SUSPENSION_MARK,
            SupplementalPunctuation::InvertedLowKavyka => INVERTED_LOW_KAVYKA,
            SupplementalPunctuation::InvertedLowKavykaWithKavykaAbove => INVERTED_LOW_KAVYKA_WITH_KAVYKA_ABOVE,
            SupplementalPunctuation::LowKavyka => LOW_KAVYKA,
            SupplementalPunctuation::LowKavykaWithDot => LOW_KAVYKA_WITH_DOT,
            SupplementalPunctuation::DoubleStackedComma => DOUBLE_STACKED_COMMA,
            SupplementalPunctuation::DottedSolidus => DOTTED_SOLIDUS,
            SupplementalPunctuation::TripleDagger => TRIPLE_DAGGER,
            SupplementalPunctuation::MedievalComma => MEDIEVAL_COMMA,
            SupplementalPunctuation::ParagraphusMark => PARAGRAPHUS_MARK,
            SupplementalPunctuation::PunctusElevatusMark => PUNCTUS_ELEVATUS_MARK,
            SupplementalPunctuation::CornishVerseDivider => CORNISH_VERSE_DIVIDER,
        }
    }
}

impl std::convert::TryFrom<char> for SupplementalPunctuation {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            RIGHT_ANGLE_SUBSTITUTION_MARKER => Ok(SupplementalPunctuation::RightAngleSubstitutionMarker),
            RIGHT_ANGLE_DOTTED_SUBSTITUTION_MARKER => Ok(SupplementalPunctuation::RightAngleDottedSubstitutionMarker),
            LEFT_SUBSTITUTION_BRACKET => Ok(SupplementalPunctuation::LeftSubstitutionBracket),
            RIGHT_SUBSTITUTION_BRACKET => Ok(SupplementalPunctuation::RightSubstitutionBracket),
            LEFT_DOTTED_SUBSTITUTION_BRACKET => Ok(SupplementalPunctuation::LeftDottedSubstitutionBracket),
            RIGHT_DOTTED_SUBSTITUTION_BRACKET => Ok(SupplementalPunctuation::RightDottedSubstitutionBracket),
            RAISED_INTERPOLATION_MARKER => Ok(SupplementalPunctuation::RaisedInterpolationMarker),
            RAISED_DOTTED_INTERPOLATION_MARKER => Ok(SupplementalPunctuation::RaisedDottedInterpolationMarker),
            DOTTED_TRANSPOSITION_MARKER => Ok(SupplementalPunctuation::DottedTranspositionMarker),
            LEFT_TRANSPOSITION_BRACKET => Ok(SupplementalPunctuation::LeftTranspositionBracket),
            RIGHT_TRANSPOSITION_BRACKET => Ok(SupplementalPunctuation::RightTranspositionBracket),
            RAISED_SQUARE => Ok(SupplementalPunctuation::RaisedSquare),
            LEFT_RAISED_OMISSION_BRACKET => Ok(SupplementalPunctuation::LeftRaisedOmissionBracket),
            RIGHT_RAISED_OMISSION_BRACKET => Ok(SupplementalPunctuation::RightRaisedOmissionBracket),
            EDITORIAL_CORONIS => Ok(SupplementalPunctuation::EditorialCoronis),
            PARAGRAPHOS => Ok(SupplementalPunctuation::Paragraphos),
            FORKED_PARAGRAPHOS => Ok(SupplementalPunctuation::ForkedParagraphos),
            REVERSED_FORKED_PARAGRAPHOS => Ok(SupplementalPunctuation::ReversedForkedParagraphos),
            HYPODIASTOLE => Ok(SupplementalPunctuation::Hypodiastole),
            DOTTED_OBELOS => Ok(SupplementalPunctuation::DottedObelos),
            DOWNWARDS_ANCORA => Ok(SupplementalPunctuation::DownwardsAncora),
            UPWARDS_ANCORA => Ok(SupplementalPunctuation::UpwardsAncora),
            DOTTED_RIGHT_DASH_POINTING_ANGLE => Ok(SupplementalPunctuation::DottedRightDashPointingAngle),
            DOUBLE_OBLIQUE_HYPHEN => Ok(SupplementalPunctuation::DoubleObliqueHyphen),
            INVERTED_INTERROBANG => Ok(SupplementalPunctuation::InvertedInterrobang),
            PALM_BRANCH => Ok(SupplementalPunctuation::PalmBranch),
            HYPHEN_WITH_DIAERESIS => Ok(SupplementalPunctuation::HyphenWithDiaeresis),
            TILDE_WITH_RING_ABOVE => Ok(SupplementalPunctuation::TildeWithRingAbove),
            LEFT_LOW_PARAPHRASE_BRACKET => Ok(SupplementalPunctuation::LeftLowParaphraseBracket),
            RIGHT_LOW_PARAPHRASE_BRACKET => Ok(SupplementalPunctuation::RightLowParaphraseBracket),
            TILDE_WITH_DOT_ABOVE => Ok(SupplementalPunctuation::TildeWithDotAbove),
            TILDE_WITH_DOT_BELOW => Ok(SupplementalPunctuation::TildeWithDotBelow),
            LEFT_VERTICAL_BAR_WITH_QUILL => Ok(SupplementalPunctuation::LeftVerticalBarWithQuill),
            RIGHT_VERTICAL_BAR_WITH_QUILL => Ok(SupplementalPunctuation::RightVerticalBarWithQuill),
            TOP_LEFT_HALF_BRACKET => Ok(SupplementalPunctuation::TopLeftHalfBracket),
            TOP_RIGHT_HALF_BRACKET => Ok(SupplementalPunctuation::TopRightHalfBracket),
            BOTTOM_LEFT_HALF_BRACKET => Ok(SupplementalPunctuation::BottomLeftHalfBracket),
            BOTTOM_RIGHT_HALF_BRACKET => Ok(SupplementalPunctuation::BottomRightHalfBracket),
            LEFT_SIDEWAYS_U_BRACKET => Ok(SupplementalPunctuation::LeftSidewaysUBracket),
            RIGHT_SIDEWAYS_U_BRACKET => Ok(SupplementalPunctuation::RightSidewaysUBracket),
            LEFT_DOUBLE_PARENTHESIS => Ok(SupplementalPunctuation::LeftDoubleParenthesis),
            RIGHT_DOUBLE_PARENTHESIS => Ok(SupplementalPunctuation::RightDoubleParenthesis),
            TWO_DOTS_OVER_ONE_DOT_PUNCTUATION => Ok(SupplementalPunctuation::TwoDotsOverOneDotPunctuation),
            ONE_DOT_OVER_TWO_DOTS_PUNCTUATION => Ok(SupplementalPunctuation::OneDotOverTwoDotsPunctuation),
            SQUARED_FOUR_DOT_PUNCTUATION => Ok(SupplementalPunctuation::SquaredFourDotPunctuation),
            FIVE_DOT_MARK => Ok(SupplementalPunctuation::FiveDotMark),
            REVERSED_QUESTION_MARK => Ok(SupplementalPunctuation::ReversedQuestionMark),
            VERTICAL_TILDE => Ok(SupplementalPunctuation::VerticalTilde),
            RING_POINT => Ok(SupplementalPunctuation::RingPoint),
            WORD_SEPARATOR_MIDDLE_DOT => Ok(SupplementalPunctuation::WordSeparatorMiddleDot),
            TURNED_COMMA => Ok(SupplementalPunctuation::TurnedComma),
            RAISED_DOT => Ok(SupplementalPunctuation::RaisedDot),
            RAISED_COMMA => Ok(SupplementalPunctuation::RaisedComma),
            TURNED_SEMICOLON => Ok(SupplementalPunctuation::TurnedSemicolon),
            DAGGER_WITH_LEFT_GUARD => Ok(SupplementalPunctuation::DaggerWithLeftGuard),
            DAGGER_WITH_RIGHT_GUARD => Ok(SupplementalPunctuation::DaggerWithRightGuard),
            TURNED_DAGGER => Ok(SupplementalPunctuation::TurnedDagger),
            TOP_HALF_SECTION_SIGN => Ok(SupplementalPunctuation::TopHalfSectionSign),
            TWO_DASH_EM_DASH => Ok(SupplementalPunctuation::TwoDashEmDash),
            THREE_DASH_EM_DASH => Ok(SupplementalPunctuation::ThreeDashEmDash),
            STENOGRAPHIC_FULL_STOP => Ok(SupplementalPunctuation::StenographicFullStop),
            VERTICAL_SIX_DOTS => Ok(SupplementalPunctuation::VerticalSixDots),
            WIGGLY_VERTICAL_LINE => Ok(SupplementalPunctuation::WigglyVerticalLine),
            CAPITULUM => Ok(SupplementalPunctuation::Capitulum),
            DOUBLE_HYPHEN => Ok(SupplementalPunctuation::DoubleHyphen),
            REVERSED_COMMA => Ok(SupplementalPunctuation::ReversedComma),
            DOUBLE_LOW_DASH_REVERSED_DASH_9_QUOTATION_MARK => Ok(SupplementalPunctuation::DoubleLowDashReversedDash9QuotationMark),
            DASH_WITH_LEFT_UPTURN => Ok(SupplementalPunctuation::DashWithLeftUpturn),
            DOUBLE_SUSPENSION_MARK => Ok(SupplementalPunctuation::DoubleSuspensionMark),
            INVERTED_LOW_KAVYKA => Ok(SupplementalPunctuation::InvertedLowKavyka),
            INVERTED_LOW_KAVYKA_WITH_KAVYKA_ABOVE => Ok(SupplementalPunctuation::InvertedLowKavykaWithKavykaAbove),
            LOW_KAVYKA => Ok(SupplementalPunctuation::LowKavyka),
            LOW_KAVYKA_WITH_DOT => Ok(SupplementalPunctuation::LowKavykaWithDot),
            DOUBLE_STACKED_COMMA => Ok(SupplementalPunctuation::DoubleStackedComma),
            DOTTED_SOLIDUS => Ok(SupplementalPunctuation::DottedSolidus),
            TRIPLE_DAGGER => Ok(SupplementalPunctuation::TripleDagger),
            MEDIEVAL_COMMA => Ok(SupplementalPunctuation::MedievalComma),
            PARAGRAPHUS_MARK => Ok(SupplementalPunctuation::ParagraphusMark),
            PUNCTUS_ELEVATUS_MARK => Ok(SupplementalPunctuation::PunctusElevatusMark),
            CORNISH_VERSE_DIVIDER => Ok(SupplementalPunctuation::CornishVerseDivider),
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

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            SupplementalPunctuation::RightAngleSubstitutionMarker => "right angle substitution marker",
            SupplementalPunctuation::RightAngleDottedSubstitutionMarker => "right angle dotted substitution marker",
            SupplementalPunctuation::LeftSubstitutionBracket => "left substitution bracket",
            SupplementalPunctuation::RightSubstitutionBracket => "right substitution bracket",
            SupplementalPunctuation::LeftDottedSubstitutionBracket => "left dotted substitution bracket",
            SupplementalPunctuation::RightDottedSubstitutionBracket => "right dotted substitution bracket",
            SupplementalPunctuation::RaisedInterpolationMarker => "raised interpolation marker",
            SupplementalPunctuation::RaisedDottedInterpolationMarker => "raised dotted interpolation marker",
            SupplementalPunctuation::DottedTranspositionMarker => "dotted transposition marker",
            SupplementalPunctuation::LeftTranspositionBracket => "left transposition bracket",
            SupplementalPunctuation::RightTranspositionBracket => "right transposition bracket",
            SupplementalPunctuation::RaisedSquare => "raised square",
            SupplementalPunctuation::LeftRaisedOmissionBracket => "left raised omission bracket",
            SupplementalPunctuation::RightRaisedOmissionBracket => "right raised omission bracket",
            SupplementalPunctuation::EditorialCoronis => "editorial coronis",
            SupplementalPunctuation::Paragraphos => "paragraphos",
            SupplementalPunctuation::ForkedParagraphos => "forked paragraphos",
            SupplementalPunctuation::ReversedForkedParagraphos => "reversed forked paragraphos",
            SupplementalPunctuation::Hypodiastole => "hypodiastole",
            SupplementalPunctuation::DottedObelos => "dotted obelos",
            SupplementalPunctuation::DownwardsAncora => "downwards ancora",
            SupplementalPunctuation::UpwardsAncora => "upwards ancora",
            SupplementalPunctuation::DottedRightDashPointingAngle => "dotted right-pointing angle",
            SupplementalPunctuation::DoubleObliqueHyphen => "double oblique hyphen",
            SupplementalPunctuation::InvertedInterrobang => "inverted interrobang",
            SupplementalPunctuation::PalmBranch => "palm branch",
            SupplementalPunctuation::HyphenWithDiaeresis => "hyphen with diaeresis",
            SupplementalPunctuation::TildeWithRingAbove => "tilde with ring above",
            SupplementalPunctuation::LeftLowParaphraseBracket => "left low paraphrase bracket",
            SupplementalPunctuation::RightLowParaphraseBracket => "right low paraphrase bracket",
            SupplementalPunctuation::TildeWithDotAbove => "tilde with dot above",
            SupplementalPunctuation::TildeWithDotBelow => "tilde with dot below",
            SupplementalPunctuation::LeftVerticalBarWithQuill => "left vertical bar with quill",
            SupplementalPunctuation::RightVerticalBarWithQuill => "right vertical bar with quill",
            SupplementalPunctuation::TopLeftHalfBracket => "top left half bracket",
            SupplementalPunctuation::TopRightHalfBracket => "top right half bracket",
            SupplementalPunctuation::BottomLeftHalfBracket => "bottom left half bracket",
            SupplementalPunctuation::BottomRightHalfBracket => "bottom right half bracket",
            SupplementalPunctuation::LeftSidewaysUBracket => "left sideways u bracket",
            SupplementalPunctuation::RightSidewaysUBracket => "right sideways u bracket",
            SupplementalPunctuation::LeftDoubleParenthesis => "left double parenthesis",
            SupplementalPunctuation::RightDoubleParenthesis => "right double parenthesis",
            SupplementalPunctuation::TwoDotsOverOneDotPunctuation => "two dots over one dot punctuation",
            SupplementalPunctuation::OneDotOverTwoDotsPunctuation => "one dot over two dots punctuation",
            SupplementalPunctuation::SquaredFourDotPunctuation => "squared four dot punctuation",
            SupplementalPunctuation::FiveDotMark => "five dot mark",
            SupplementalPunctuation::ReversedQuestionMark => "reversed question mark",
            SupplementalPunctuation::VerticalTilde => "vertical tilde",
            SupplementalPunctuation::RingPoint => "ring point",
            SupplementalPunctuation::WordSeparatorMiddleDot => "word separator middle dot",
            SupplementalPunctuation::TurnedComma => "turned comma",
            SupplementalPunctuation::RaisedDot => "raised dot",
            SupplementalPunctuation::RaisedComma => "raised comma",
            SupplementalPunctuation::TurnedSemicolon => "turned semicolon",
            SupplementalPunctuation::DaggerWithLeftGuard => "dagger with left guard",
            SupplementalPunctuation::DaggerWithRightGuard => "dagger with right guard",
            SupplementalPunctuation::TurnedDagger => "turned dagger",
            SupplementalPunctuation::TopHalfSectionSign => "top half section sign",
            SupplementalPunctuation::TwoDashEmDash => "two-em dash",
            SupplementalPunctuation::ThreeDashEmDash => "three-em dash",
            SupplementalPunctuation::StenographicFullStop => "stenographic full stop",
            SupplementalPunctuation::VerticalSixDots => "vertical six dots",
            SupplementalPunctuation::WigglyVerticalLine => "wiggly vertical line",
            SupplementalPunctuation::Capitulum => "capitulum",
            SupplementalPunctuation::DoubleHyphen => "double hyphen",
            SupplementalPunctuation::ReversedComma => "reversed comma",
            SupplementalPunctuation::DoubleLowDashReversedDash9QuotationMark => "double low-reversed-9 quotation mark",
            SupplementalPunctuation::DashWithLeftUpturn => "dash with left upturn",
            SupplementalPunctuation::DoubleSuspensionMark => "double suspension mark",
            SupplementalPunctuation::InvertedLowKavyka => "inverted low kavyka",
            SupplementalPunctuation::InvertedLowKavykaWithKavykaAbove => "inverted low kavyka with kavyka above",
            SupplementalPunctuation::LowKavyka => "low kavyka",
            SupplementalPunctuation::LowKavykaWithDot => "low kavyka with dot",
            SupplementalPunctuation::DoubleStackedComma => "double stacked comma",
            SupplementalPunctuation::DottedSolidus => "dotted solidus",
            SupplementalPunctuation::TripleDagger => "triple dagger",
            SupplementalPunctuation::MedievalComma => "medieval comma",
            SupplementalPunctuation::ParagraphusMark => "paragraphus mark",
            SupplementalPunctuation::PunctusElevatusMark => "punctus elevatus mark",
            SupplementalPunctuation::CornishVerseDivider => "cornish verse divider",
        }
    }
}
