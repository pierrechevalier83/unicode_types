
/// An enum to represent all characters in the MathematicalOperators block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum MathematicalOperators {
    /// \u{2200}: '∀'
    ForAll,
    /// \u{2201}: '∁'
    Complement,
    /// \u{2202}: '∂'
    PartialDifferential,
    /// \u{2203}: '∃'
    ThereExists,
    /// \u{2204}: '∄'
    ThereDoesNotExist,
    /// \u{2205}: '∅'
    EmptySet,
    /// \u{2206}: '∆'
    Increment,
    /// \u{2207}: '∇'
    Nabla,
    /// \u{2208}: '∈'
    ElementOf,
    /// \u{2209}: '∉'
    NotAnElementOf,
    /// \u{220a}: '∊'
    SmallElementOf,
    /// \u{220b}: '∋'
    ContainsAsMember,
    /// \u{220c}: '∌'
    DoesNotContainAsMember,
    /// \u{220d}: '∍'
    SmallContainsAsMember,
    /// \u{220e}: '∎'
    EndOfProof,
    /// \u{220f}: '∏'
    NDashAryProduct,
    /// \u{2210}: '∐'
    NDashAryCoproduct,
    /// \u{2211}: '∑'
    NDashArySummation,
    /// \u{2212}: '−'
    MinusSign,
    /// \u{2213}: '∓'
    MinusDashOrDashPlusSign,
    /// \u{2214}: '∔'
    DotPlus,
    /// \u{2215}: '∕'
    DivisionSlash,
    /// \u{2216}: '∖'
    SetMinus,
    /// \u{2217}: '∗'
    AsteriskOperator,
    /// \u{2218}: '∘'
    RingOperator,
    /// \u{2219}: '∙'
    BulletOperator,
    /// \u{221a}: '√'
    SquareRoot,
    /// \u{221b}: '∛'
    CubeRoot,
    /// \u{221c}: '∜'
    FourthRoot,
    /// \u{221d}: '∝'
    ProportionalTo,
    /// \u{221e}: '∞'
    Infinity,
    /// \u{221f}: '∟'
    RightAngle,
    /// \u{2220}: '∠'
    Angle,
    /// \u{2221}: '∡'
    MeasuredAngle,
    /// \u{2222}: '∢'
    SphericalAngle,
    /// \u{2223}: '∣'
    Divides,
    /// \u{2224}: '∤'
    DoesNotDivide,
    /// \u{2225}: '∥'
    ParallelTo,
    /// \u{2226}: '∦'
    NotParallelTo,
    /// \u{2227}: '∧'
    LogicalAnd,
    /// \u{2228}: '∨'
    LogicalOr,
    /// \u{2229}: '∩'
    Intersection,
    /// \u{222a}: '∪'
    Union,
    /// \u{222b}: '∫'
    Integral,
    /// \u{222c}: '∬'
    DoubleIntegral,
    /// \u{222d}: '∭'
    TripleIntegral,
    /// \u{222e}: '∮'
    ContourIntegral,
    /// \u{222f}: '∯'
    SurfaceIntegral,
    /// \u{2230}: '∰'
    VolumeIntegral,
    /// \u{2231}: '∱'
    ClockwiseIntegral,
    /// \u{2232}: '∲'
    ClockwiseContourIntegral,
    /// \u{2233}: '∳'
    AnticlockwiseContourIntegral,
    /// \u{2234}: '∴'
    Therefore,
    /// \u{2235}: '∵'
    Because,
    /// \u{2236}: '∶'
    Ratio,
    /// \u{2237}: '∷'
    Proportion,
    /// \u{2238}: '∸'
    DotMinus,
    /// \u{2239}: '∹'
    Excess,
    /// \u{223a}: '∺'
    GeometricProportion,
    /// \u{223b}: '∻'
    Homothetic,
    /// \u{223c}: '∼'
    TildeOperator,
    /// \u{223d}: '∽'
    ReversedTilde,
    /// \u{223e}: '∾'
    InvertedLazyS,
    /// \u{223f}: '∿'
    SineWave,
    /// \u{2240}: '≀'
    WreathProduct,
    /// \u{2241}: '≁'
    NotTilde,
    /// \u{2242}: '≂'
    MinusTilde,
    /// \u{2243}: '≃'
    AsymptoticallyEqualTo,
    /// \u{2244}: '≄'
    NotAsymptoticallyEqualTo,
    /// \u{2245}: '≅'
    ApproximatelyEqualTo,
    /// \u{2246}: '≆'
    ApproximatelyButNotActuallyEqualTo,
    /// \u{2247}: '≇'
    NeitherApproximatelyNorActuallyEqualTo,
    /// \u{2248}: '≈'
    AlmostEqualTo,
    /// \u{2249}: '≉'
    NotAlmostEqualTo,
    /// \u{224a}: '≊'
    AlmostEqualOrEqualTo,
    /// \u{224b}: '≋'
    TripleTilde,
    /// \u{224c}: '≌'
    AllEqualTo,
    /// \u{224d}: '≍'
    EquivalentTo,
    /// \u{224e}: '≎'
    GeometricallyEquivalentTo,
    /// \u{224f}: '≏'
    DifferenceBetween,
    /// \u{2250}: '≐'
    ApproachesTheLimit,
    /// \u{2251}: '≑'
    GeometricallyEqualTo,
    /// \u{2252}: '≒'
    ApproximatelyEqualToOrTheImageOf,
    /// \u{2253}: '≓'
    ImageOfOrApproximatelyEqualTo,
    /// \u{2254}: '≔'
    ColonEquals,
    /// \u{2255}: '≕'
    EqualsColon,
    /// \u{2256}: '≖'
    RingInEqualTo,
    /// \u{2257}: '≗'
    RingEqualTo,
    /// \u{2258}: '≘'
    CorrespondsTo,
    /// \u{2259}: '≙'
    Estimates,
    /// \u{225a}: '≚'
    EquiangularTo,
    /// \u{225b}: '≛'
    StarEquals,
    /// \u{225c}: '≜'
    DeltaEqualTo,
    /// \u{225d}: '≝'
    EqualToByDefinition,
    /// \u{225e}: '≞'
    MeasuredBy,
    /// \u{225f}: '≟'
    QuestionedEqualTo,
    /// \u{2260}: '≠'
    NotEqualTo,
    /// \u{2261}: '≡'
    IdenticalTo,
    /// \u{2262}: '≢'
    NotIdenticalTo,
    /// \u{2263}: '≣'
    StrictlyEquivalentTo,
    /// \u{2264}: '≤'
    LessDashThanOrEqualTo,
    /// \u{2265}: '≥'
    GreaterDashThanOrEqualTo,
    /// \u{2266}: '≦'
    LessDashThanOverEqualTo,
    /// \u{2267}: '≧'
    GreaterDashThanOverEqualTo,
    /// \u{2268}: '≨'
    LessDashThanButNotEqualTo,
    /// \u{2269}: '≩'
    GreaterDashThanButNotEqualTo,
    /// \u{226a}: '≪'
    MuchLessDashThan,
    /// \u{226b}: '≫'
    MuchGreaterDashThan,
    /// \u{226c}: '≬'
    Between,
    /// \u{226d}: '≭'
    NotEquivalentTo,
    /// \u{226e}: '≮'
    NotLessDashThan,
    /// \u{226f}: '≯'
    NotGreaterDashThan,
    /// \u{2270}: '≰'
    NeitherLessDashThanNorEqualTo,
    /// \u{2271}: '≱'
    NeitherGreaterDashThanNorEqualTo,
    /// \u{2272}: '≲'
    LessDashThanOrEquivalentTo,
    /// \u{2273}: '≳'
    GreaterDashThanOrEquivalentTo,
    /// \u{2274}: '≴'
    NeitherLessDashThanNorEquivalentTo,
    /// \u{2275}: '≵'
    NeitherGreaterDashThanNorEquivalentTo,
    /// \u{2276}: '≶'
    LessDashThanOrGreaterDashThan,
    /// \u{2277}: '≷'
    GreaterDashThanOrLessDashThan,
    /// \u{2278}: '≸'
    NeitherLessDashThanNorGreaterDashThan,
    /// \u{2279}: '≹'
    NeitherGreaterDashThanNorLessDashThan,
    /// \u{227a}: '≺'
    Precedes,
    /// \u{227b}: '≻'
    Succeeds,
    /// \u{227c}: '≼'
    PrecedesOrEqualTo,
    /// \u{227d}: '≽'
    SucceedsOrEqualTo,
    /// \u{227e}: '≾'
    PrecedesOrEquivalentTo,
    /// \u{227f}: '≿'
    SucceedsOrEquivalentTo,
    /// \u{2280}: '⊀'
    DoesNotPrecede,
    /// \u{2281}: '⊁'
    DoesNotSucceed,
    /// \u{2282}: '⊂'
    SubsetOf,
    /// \u{2283}: '⊃'
    SupersetOf,
    /// \u{2284}: '⊄'
    NotASubsetOf,
    /// \u{2285}: '⊅'
    NotASupersetOf,
    /// \u{2286}: '⊆'
    SubsetOfOrEqualTo,
    /// \u{2287}: '⊇'
    SupersetOfOrEqualTo,
    /// \u{2288}: '⊈'
    NeitherASubsetOfNorEqualTo,
    /// \u{2289}: '⊉'
    NeitherASupersetOfNorEqualTo,
    /// \u{228a}: '⊊'
    SubsetOfWithNotEqualTo,
    /// \u{228b}: '⊋'
    SupersetOfWithNotEqualTo,
    /// \u{228c}: '⊌'
    Multiset,
    /// \u{228d}: '⊍'
    MultisetMultiplication,
    /// \u{228e}: '⊎'
    MultisetUnion,
    /// \u{228f}: '⊏'
    SquareImageOf,
    /// \u{2290}: '⊐'
    SquareOriginalOf,
    /// \u{2291}: '⊑'
    SquareImageOfOrEqualTo,
    /// \u{2292}: '⊒'
    SquareOriginalOfOrEqualTo,
    /// \u{2293}: '⊓'
    SquareCap,
    /// \u{2294}: '⊔'
    SquareCup,
    /// \u{2295}: '⊕'
    CircledPlus,
    /// \u{2296}: '⊖'
    CircledMinus,
    /// \u{2297}: '⊗'
    CircledTimes,
    /// \u{2298}: '⊘'
    CircledDivisionSlash,
    /// \u{2299}: '⊙'
    CircledDotOperator,
    /// \u{229a}: '⊚'
    CircledRingOperator,
    /// \u{229b}: '⊛'
    CircledAsteriskOperator,
    /// \u{229c}: '⊜'
    CircledEquals,
    /// \u{229d}: '⊝'
    CircledDash,
    /// \u{229e}: '⊞'
    SquaredPlus,
    /// \u{229f}: '⊟'
    SquaredMinus,
    /// \u{22a0}: '⊠'
    SquaredTimes,
    /// \u{22a1}: '⊡'
    SquaredDotOperator,
    /// \u{22a2}: '⊢'
    RightTack,
    /// \u{22a3}: '⊣'
    LeftTack,
    /// \u{22a4}: '⊤'
    DownTack,
    /// \u{22a5}: '⊥'
    UpTack,
    /// \u{22a6}: '⊦'
    Assertion,
    /// \u{22a7}: '⊧'
    Models,
    /// \u{22a8}: '⊨'
    True,
    /// \u{22a9}: '⊩'
    Forces,
    /// \u{22aa}: '⊪'
    TripleVerticalBarRightTurnstile,
    /// \u{22ab}: '⊫'
    DoubleVerticalBarDoubleRightTurnstile,
    /// \u{22ac}: '⊬'
    DoesNotProve,
    /// \u{22ad}: '⊭'
    NotTrue,
    /// \u{22ae}: '⊮'
    DoesNotForce,
    /// \u{22af}: '⊯'
    NegatedDoubleVerticalBarDoubleRightTurnstile,
    /// \u{22b0}: '⊰'
    PrecedesUnderRelation,
    /// \u{22b1}: '⊱'
    SucceedsUnderRelation,
    /// \u{22b2}: '⊲'
    NormalSubgroupOf,
    /// \u{22b3}: '⊳'
    ContainsAsNormalSubgroup,
    /// \u{22b4}: '⊴'
    NormalSubgroupOfOrEqualTo,
    /// \u{22b5}: '⊵'
    ContainsAsNormalSubgroupOrEqualTo,
    /// \u{22b6}: '⊶'
    OriginalOf,
    /// \u{22b7}: '⊷'
    ImageOf,
    /// \u{22b8}: '⊸'
    Multimap,
    /// \u{22b9}: '⊹'
    HermitianConjugateMatrix,
    /// \u{22ba}: '⊺'
    Intercalate,
    /// \u{22bb}: '⊻'
    Xor,
    /// \u{22bc}: '⊼'
    Nand,
    /// \u{22bd}: '⊽'
    Nor,
    /// \u{22be}: '⊾'
    RightAngleWithArc,
    /// \u{22bf}: '⊿'
    RightTriangle,
    /// \u{22c0}: '⋀'
    NDashAryLogicalAnd,
    /// \u{22c1}: '⋁'
    NDashAryLogicalOr,
    /// \u{22c2}: '⋂'
    NDashAryIntersection,
    /// \u{22c3}: '⋃'
    NDashAryUnion,
    /// \u{22c4}: '⋄'
    DiamondOperator,
    /// \u{22c5}: '⋅'
    DotOperator,
    /// \u{22c6}: '⋆'
    StarOperator,
    /// \u{22c7}: '⋇'
    DivisionTimes,
    /// \u{22c8}: '⋈'
    Bowtie,
    /// \u{22c9}: '⋉'
    LeftNormalFactorSemidirectProduct,
    /// \u{22ca}: '⋊'
    RightNormalFactorSemidirectProduct,
    /// \u{22cb}: '⋋'
    LeftSemidirectProduct,
    /// \u{22cc}: '⋌'
    RightSemidirectProduct,
    /// \u{22cd}: '⋍'
    ReversedTildeEquals,
    /// \u{22ce}: '⋎'
    CurlyLogicalOr,
    /// \u{22cf}: '⋏'
    CurlyLogicalAnd,
    /// \u{22d0}: '⋐'
    DoubleSubset,
    /// \u{22d1}: '⋑'
    DoubleSuperset,
    /// \u{22d2}: '⋒'
    DoubleIntersection,
    /// \u{22d3}: '⋓'
    DoubleUnion,
    /// \u{22d4}: '⋔'
    Pitchfork,
    /// \u{22d5}: '⋕'
    EqualAndParallelTo,
    /// \u{22d6}: '⋖'
    LessDashThanWithDot,
    /// \u{22d7}: '⋗'
    GreaterDashThanWithDot,
    /// \u{22d8}: '⋘'
    VeryMuchLessDashThan,
    /// \u{22d9}: '⋙'
    VeryMuchGreaterDashThan,
    /// \u{22da}: '⋚'
    LessDashThanEqualToOrGreaterDashThan,
    /// \u{22db}: '⋛'
    GreaterDashThanEqualToOrLessDashThan,
    /// \u{22dc}: '⋜'
    EqualToOrLessDashThan,
    /// \u{22dd}: '⋝'
    EqualToOrGreaterDashThan,
    /// \u{22de}: '⋞'
    EqualToOrPrecedes,
    /// \u{22df}: '⋟'
    EqualToOrSucceeds,
    /// \u{22e0}: '⋠'
    DoesNotPrecedeOrEqual,
    /// \u{22e1}: '⋡'
    DoesNotSucceedOrEqual,
    /// \u{22e2}: '⋢'
    NotSquareImageOfOrEqualTo,
    /// \u{22e3}: '⋣'
    NotSquareOriginalOfOrEqualTo,
    /// \u{22e4}: '⋤'
    SquareImageOfOrNotEqualTo,
    /// \u{22e5}: '⋥'
    SquareOriginalOfOrNotEqualTo,
    /// \u{22e6}: '⋦'
    LessDashThanButNotEquivalentTo,
    /// \u{22e7}: '⋧'
    GreaterDashThanButNotEquivalentTo,
    /// \u{22e8}: '⋨'
    PrecedesButNotEquivalentTo,
    /// \u{22e9}: '⋩'
    SucceedsButNotEquivalentTo,
    /// \u{22ea}: '⋪'
    NotNormalSubgroupOf,
    /// \u{22eb}: '⋫'
    DoesNotContainAsNormalSubgroup,
    /// \u{22ec}: '⋬'
    NotNormalSubgroupOfOrEqualTo,
    /// \u{22ed}: '⋭'
    DoesNotContainAsNormalSubgroupOrEqual,
    /// \u{22ee}: '⋮'
    VerticalEllipsis,
    /// \u{22ef}: '⋯'
    MidlineHorizontalEllipsis,
    /// \u{22f0}: '⋰'
    UpRightDiagonalEllipsis,
    /// \u{22f1}: '⋱'
    DownRightDiagonalEllipsis,
    /// \u{22f2}: '⋲'
    ElementOfWithLongHorizontalStroke,
    /// \u{22f3}: '⋳'
    ElementOfWithVerticalBarAtEndOfHorizontalStroke,
    /// \u{22f4}: '⋴'
    SmallElementOfWithVerticalBarAtEndOfHorizontalStroke,
    /// \u{22f5}: '⋵'
    ElementOfWithDotAbove,
    /// \u{22f6}: '⋶'
    ElementOfWithOverbar,
    /// \u{22f7}: '⋷'
    SmallElementOfWithOverbar,
    /// \u{22f8}: '⋸'
    ElementOfWithUnderbar,
    /// \u{22f9}: '⋹'
    ElementOfWithTwoHorizontalStrokes,
    /// \u{22fa}: '⋺'
    ContainsWithLongHorizontalStroke,
    /// \u{22fb}: '⋻'
    ContainsWithVerticalBarAtEndOfHorizontalStroke,
    /// \u{22fc}: '⋼'
    SmallContainsWithVerticalBarAtEndOfHorizontalStroke,
    /// \u{22fd}: '⋽'
    ContainsWithOverbar,
    /// \u{22fe}: '⋾'
    SmallContainsWithOverbar,
}

impl Into<char> for MathematicalOperators {
    fn into(self) -> char {
        match self {
            MathematicalOperators::ForAll => '∀',
            MathematicalOperators::Complement => '∁',
            MathematicalOperators::PartialDifferential => '∂',
            MathematicalOperators::ThereExists => '∃',
            MathematicalOperators::ThereDoesNotExist => '∄',
            MathematicalOperators::EmptySet => '∅',
            MathematicalOperators::Increment => '∆',
            MathematicalOperators::Nabla => '∇',
            MathematicalOperators::ElementOf => '∈',
            MathematicalOperators::NotAnElementOf => '∉',
            MathematicalOperators::SmallElementOf => '∊',
            MathematicalOperators::ContainsAsMember => '∋',
            MathematicalOperators::DoesNotContainAsMember => '∌',
            MathematicalOperators::SmallContainsAsMember => '∍',
            MathematicalOperators::EndOfProof => '∎',
            MathematicalOperators::NDashAryProduct => '∏',
            MathematicalOperators::NDashAryCoproduct => '∐',
            MathematicalOperators::NDashArySummation => '∑',
            MathematicalOperators::MinusSign => '−',
            MathematicalOperators::MinusDashOrDashPlusSign => '∓',
            MathematicalOperators::DotPlus => '∔',
            MathematicalOperators::DivisionSlash => '∕',
            MathematicalOperators::SetMinus => '∖',
            MathematicalOperators::AsteriskOperator => '∗',
            MathematicalOperators::RingOperator => '∘',
            MathematicalOperators::BulletOperator => '∙',
            MathematicalOperators::SquareRoot => '√',
            MathematicalOperators::CubeRoot => '∛',
            MathematicalOperators::FourthRoot => '∜',
            MathematicalOperators::ProportionalTo => '∝',
            MathematicalOperators::Infinity => '∞',
            MathematicalOperators::RightAngle => '∟',
            MathematicalOperators::Angle => '∠',
            MathematicalOperators::MeasuredAngle => '∡',
            MathematicalOperators::SphericalAngle => '∢',
            MathematicalOperators::Divides => '∣',
            MathematicalOperators::DoesNotDivide => '∤',
            MathematicalOperators::ParallelTo => '∥',
            MathematicalOperators::NotParallelTo => '∦',
            MathematicalOperators::LogicalAnd => '∧',
            MathematicalOperators::LogicalOr => '∨',
            MathematicalOperators::Intersection => '∩',
            MathematicalOperators::Union => '∪',
            MathematicalOperators::Integral => '∫',
            MathematicalOperators::DoubleIntegral => '∬',
            MathematicalOperators::TripleIntegral => '∭',
            MathematicalOperators::ContourIntegral => '∮',
            MathematicalOperators::SurfaceIntegral => '∯',
            MathematicalOperators::VolumeIntegral => '∰',
            MathematicalOperators::ClockwiseIntegral => '∱',
            MathematicalOperators::ClockwiseContourIntegral => '∲',
            MathematicalOperators::AnticlockwiseContourIntegral => '∳',
            MathematicalOperators::Therefore => '∴',
            MathematicalOperators::Because => '∵',
            MathematicalOperators::Ratio => '∶',
            MathematicalOperators::Proportion => '∷',
            MathematicalOperators::DotMinus => '∸',
            MathematicalOperators::Excess => '∹',
            MathematicalOperators::GeometricProportion => '∺',
            MathematicalOperators::Homothetic => '∻',
            MathematicalOperators::TildeOperator => '∼',
            MathematicalOperators::ReversedTilde => '∽',
            MathematicalOperators::InvertedLazyS => '∾',
            MathematicalOperators::SineWave => '∿',
            MathematicalOperators::WreathProduct => '≀',
            MathematicalOperators::NotTilde => '≁',
            MathematicalOperators::MinusTilde => '≂',
            MathematicalOperators::AsymptoticallyEqualTo => '≃',
            MathematicalOperators::NotAsymptoticallyEqualTo => '≄',
            MathematicalOperators::ApproximatelyEqualTo => '≅',
            MathematicalOperators::ApproximatelyButNotActuallyEqualTo => '≆',
            MathematicalOperators::NeitherApproximatelyNorActuallyEqualTo => '≇',
            MathematicalOperators::AlmostEqualTo => '≈',
            MathematicalOperators::NotAlmostEqualTo => '≉',
            MathematicalOperators::AlmostEqualOrEqualTo => '≊',
            MathematicalOperators::TripleTilde => '≋',
            MathematicalOperators::AllEqualTo => '≌',
            MathematicalOperators::EquivalentTo => '≍',
            MathematicalOperators::GeometricallyEquivalentTo => '≎',
            MathematicalOperators::DifferenceBetween => '≏',
            MathematicalOperators::ApproachesTheLimit => '≐',
            MathematicalOperators::GeometricallyEqualTo => '≑',
            MathematicalOperators::ApproximatelyEqualToOrTheImageOf => '≒',
            MathematicalOperators::ImageOfOrApproximatelyEqualTo => '≓',
            MathematicalOperators::ColonEquals => '≔',
            MathematicalOperators::EqualsColon => '≕',
            MathematicalOperators::RingInEqualTo => '≖',
            MathematicalOperators::RingEqualTo => '≗',
            MathematicalOperators::CorrespondsTo => '≘',
            MathematicalOperators::Estimates => '≙',
            MathematicalOperators::EquiangularTo => '≚',
            MathematicalOperators::StarEquals => '≛',
            MathematicalOperators::DeltaEqualTo => '≜',
            MathematicalOperators::EqualToByDefinition => '≝',
            MathematicalOperators::MeasuredBy => '≞',
            MathematicalOperators::QuestionedEqualTo => '≟',
            MathematicalOperators::NotEqualTo => '≠',
            MathematicalOperators::IdenticalTo => '≡',
            MathematicalOperators::NotIdenticalTo => '≢',
            MathematicalOperators::StrictlyEquivalentTo => '≣',
            MathematicalOperators::LessDashThanOrEqualTo => '≤',
            MathematicalOperators::GreaterDashThanOrEqualTo => '≥',
            MathematicalOperators::LessDashThanOverEqualTo => '≦',
            MathematicalOperators::GreaterDashThanOverEqualTo => '≧',
            MathematicalOperators::LessDashThanButNotEqualTo => '≨',
            MathematicalOperators::GreaterDashThanButNotEqualTo => '≩',
            MathematicalOperators::MuchLessDashThan => '≪',
            MathematicalOperators::MuchGreaterDashThan => '≫',
            MathematicalOperators::Between => '≬',
            MathematicalOperators::NotEquivalentTo => '≭',
            MathematicalOperators::NotLessDashThan => '≮',
            MathematicalOperators::NotGreaterDashThan => '≯',
            MathematicalOperators::NeitherLessDashThanNorEqualTo => '≰',
            MathematicalOperators::NeitherGreaterDashThanNorEqualTo => '≱',
            MathematicalOperators::LessDashThanOrEquivalentTo => '≲',
            MathematicalOperators::GreaterDashThanOrEquivalentTo => '≳',
            MathematicalOperators::NeitherLessDashThanNorEquivalentTo => '≴',
            MathematicalOperators::NeitherGreaterDashThanNorEquivalentTo => '≵',
            MathematicalOperators::LessDashThanOrGreaterDashThan => '≶',
            MathematicalOperators::GreaterDashThanOrLessDashThan => '≷',
            MathematicalOperators::NeitherLessDashThanNorGreaterDashThan => '≸',
            MathematicalOperators::NeitherGreaterDashThanNorLessDashThan => '≹',
            MathematicalOperators::Precedes => '≺',
            MathematicalOperators::Succeeds => '≻',
            MathematicalOperators::PrecedesOrEqualTo => '≼',
            MathematicalOperators::SucceedsOrEqualTo => '≽',
            MathematicalOperators::PrecedesOrEquivalentTo => '≾',
            MathematicalOperators::SucceedsOrEquivalentTo => '≿',
            MathematicalOperators::DoesNotPrecede => '⊀',
            MathematicalOperators::DoesNotSucceed => '⊁',
            MathematicalOperators::SubsetOf => '⊂',
            MathematicalOperators::SupersetOf => '⊃',
            MathematicalOperators::NotASubsetOf => '⊄',
            MathematicalOperators::NotASupersetOf => '⊅',
            MathematicalOperators::SubsetOfOrEqualTo => '⊆',
            MathematicalOperators::SupersetOfOrEqualTo => '⊇',
            MathematicalOperators::NeitherASubsetOfNorEqualTo => '⊈',
            MathematicalOperators::NeitherASupersetOfNorEqualTo => '⊉',
            MathematicalOperators::SubsetOfWithNotEqualTo => '⊊',
            MathematicalOperators::SupersetOfWithNotEqualTo => '⊋',
            MathematicalOperators::Multiset => '⊌',
            MathematicalOperators::MultisetMultiplication => '⊍',
            MathematicalOperators::MultisetUnion => '⊎',
            MathematicalOperators::SquareImageOf => '⊏',
            MathematicalOperators::SquareOriginalOf => '⊐',
            MathematicalOperators::SquareImageOfOrEqualTo => '⊑',
            MathematicalOperators::SquareOriginalOfOrEqualTo => '⊒',
            MathematicalOperators::SquareCap => '⊓',
            MathematicalOperators::SquareCup => '⊔',
            MathematicalOperators::CircledPlus => '⊕',
            MathematicalOperators::CircledMinus => '⊖',
            MathematicalOperators::CircledTimes => '⊗',
            MathematicalOperators::CircledDivisionSlash => '⊘',
            MathematicalOperators::CircledDotOperator => '⊙',
            MathematicalOperators::CircledRingOperator => '⊚',
            MathematicalOperators::CircledAsteriskOperator => '⊛',
            MathematicalOperators::CircledEquals => '⊜',
            MathematicalOperators::CircledDash => '⊝',
            MathematicalOperators::SquaredPlus => '⊞',
            MathematicalOperators::SquaredMinus => '⊟',
            MathematicalOperators::SquaredTimes => '⊠',
            MathematicalOperators::SquaredDotOperator => '⊡',
            MathematicalOperators::RightTack => '⊢',
            MathematicalOperators::LeftTack => '⊣',
            MathematicalOperators::DownTack => '⊤',
            MathematicalOperators::UpTack => '⊥',
            MathematicalOperators::Assertion => '⊦',
            MathematicalOperators::Models => '⊧',
            MathematicalOperators::True => '⊨',
            MathematicalOperators::Forces => '⊩',
            MathematicalOperators::TripleVerticalBarRightTurnstile => '⊪',
            MathematicalOperators::DoubleVerticalBarDoubleRightTurnstile => '⊫',
            MathematicalOperators::DoesNotProve => '⊬',
            MathematicalOperators::NotTrue => '⊭',
            MathematicalOperators::DoesNotForce => '⊮',
            MathematicalOperators::NegatedDoubleVerticalBarDoubleRightTurnstile => '⊯',
            MathematicalOperators::PrecedesUnderRelation => '⊰',
            MathematicalOperators::SucceedsUnderRelation => '⊱',
            MathematicalOperators::NormalSubgroupOf => '⊲',
            MathematicalOperators::ContainsAsNormalSubgroup => '⊳',
            MathematicalOperators::NormalSubgroupOfOrEqualTo => '⊴',
            MathematicalOperators::ContainsAsNormalSubgroupOrEqualTo => '⊵',
            MathematicalOperators::OriginalOf => '⊶',
            MathematicalOperators::ImageOf => '⊷',
            MathematicalOperators::Multimap => '⊸',
            MathematicalOperators::HermitianConjugateMatrix => '⊹',
            MathematicalOperators::Intercalate => '⊺',
            MathematicalOperators::Xor => '⊻',
            MathematicalOperators::Nand => '⊼',
            MathematicalOperators::Nor => '⊽',
            MathematicalOperators::RightAngleWithArc => '⊾',
            MathematicalOperators::RightTriangle => '⊿',
            MathematicalOperators::NDashAryLogicalAnd => '⋀',
            MathematicalOperators::NDashAryLogicalOr => '⋁',
            MathematicalOperators::NDashAryIntersection => '⋂',
            MathematicalOperators::NDashAryUnion => '⋃',
            MathematicalOperators::DiamondOperator => '⋄',
            MathematicalOperators::DotOperator => '⋅',
            MathematicalOperators::StarOperator => '⋆',
            MathematicalOperators::DivisionTimes => '⋇',
            MathematicalOperators::Bowtie => '⋈',
            MathematicalOperators::LeftNormalFactorSemidirectProduct => '⋉',
            MathematicalOperators::RightNormalFactorSemidirectProduct => '⋊',
            MathematicalOperators::LeftSemidirectProduct => '⋋',
            MathematicalOperators::RightSemidirectProduct => '⋌',
            MathematicalOperators::ReversedTildeEquals => '⋍',
            MathematicalOperators::CurlyLogicalOr => '⋎',
            MathematicalOperators::CurlyLogicalAnd => '⋏',
            MathematicalOperators::DoubleSubset => '⋐',
            MathematicalOperators::DoubleSuperset => '⋑',
            MathematicalOperators::DoubleIntersection => '⋒',
            MathematicalOperators::DoubleUnion => '⋓',
            MathematicalOperators::Pitchfork => '⋔',
            MathematicalOperators::EqualAndParallelTo => '⋕',
            MathematicalOperators::LessDashThanWithDot => '⋖',
            MathematicalOperators::GreaterDashThanWithDot => '⋗',
            MathematicalOperators::VeryMuchLessDashThan => '⋘',
            MathematicalOperators::VeryMuchGreaterDashThan => '⋙',
            MathematicalOperators::LessDashThanEqualToOrGreaterDashThan => '⋚',
            MathematicalOperators::GreaterDashThanEqualToOrLessDashThan => '⋛',
            MathematicalOperators::EqualToOrLessDashThan => '⋜',
            MathematicalOperators::EqualToOrGreaterDashThan => '⋝',
            MathematicalOperators::EqualToOrPrecedes => '⋞',
            MathematicalOperators::EqualToOrSucceeds => '⋟',
            MathematicalOperators::DoesNotPrecedeOrEqual => '⋠',
            MathematicalOperators::DoesNotSucceedOrEqual => '⋡',
            MathematicalOperators::NotSquareImageOfOrEqualTo => '⋢',
            MathematicalOperators::NotSquareOriginalOfOrEqualTo => '⋣',
            MathematicalOperators::SquareImageOfOrNotEqualTo => '⋤',
            MathematicalOperators::SquareOriginalOfOrNotEqualTo => '⋥',
            MathematicalOperators::LessDashThanButNotEquivalentTo => '⋦',
            MathematicalOperators::GreaterDashThanButNotEquivalentTo => '⋧',
            MathematicalOperators::PrecedesButNotEquivalentTo => '⋨',
            MathematicalOperators::SucceedsButNotEquivalentTo => '⋩',
            MathematicalOperators::NotNormalSubgroupOf => '⋪',
            MathematicalOperators::DoesNotContainAsNormalSubgroup => '⋫',
            MathematicalOperators::NotNormalSubgroupOfOrEqualTo => '⋬',
            MathematicalOperators::DoesNotContainAsNormalSubgroupOrEqual => '⋭',
            MathematicalOperators::VerticalEllipsis => '⋮',
            MathematicalOperators::MidlineHorizontalEllipsis => '⋯',
            MathematicalOperators::UpRightDiagonalEllipsis => '⋰',
            MathematicalOperators::DownRightDiagonalEllipsis => '⋱',
            MathematicalOperators::ElementOfWithLongHorizontalStroke => '⋲',
            MathematicalOperators::ElementOfWithVerticalBarAtEndOfHorizontalStroke => '⋳',
            MathematicalOperators::SmallElementOfWithVerticalBarAtEndOfHorizontalStroke => '⋴',
            MathematicalOperators::ElementOfWithDotAbove => '⋵',
            MathematicalOperators::ElementOfWithOverbar => '⋶',
            MathematicalOperators::SmallElementOfWithOverbar => '⋷',
            MathematicalOperators::ElementOfWithUnderbar => '⋸',
            MathematicalOperators::ElementOfWithTwoHorizontalStrokes => '⋹',
            MathematicalOperators::ContainsWithLongHorizontalStroke => '⋺',
            MathematicalOperators::ContainsWithVerticalBarAtEndOfHorizontalStroke => '⋻',
            MathematicalOperators::SmallContainsWithVerticalBarAtEndOfHorizontalStroke => '⋼',
            MathematicalOperators::ContainsWithOverbar => '⋽',
            MathematicalOperators::SmallContainsWithOverbar => '⋾',
        }
    }
}

impl std::convert::TryFrom<char> for MathematicalOperators {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '∀' => Ok(MathematicalOperators::ForAll),
            '∁' => Ok(MathematicalOperators::Complement),
            '∂' => Ok(MathematicalOperators::PartialDifferential),
            '∃' => Ok(MathematicalOperators::ThereExists),
            '∄' => Ok(MathematicalOperators::ThereDoesNotExist),
            '∅' => Ok(MathematicalOperators::EmptySet),
            '∆' => Ok(MathematicalOperators::Increment),
            '∇' => Ok(MathematicalOperators::Nabla),
            '∈' => Ok(MathematicalOperators::ElementOf),
            '∉' => Ok(MathematicalOperators::NotAnElementOf),
            '∊' => Ok(MathematicalOperators::SmallElementOf),
            '∋' => Ok(MathematicalOperators::ContainsAsMember),
            '∌' => Ok(MathematicalOperators::DoesNotContainAsMember),
            '∍' => Ok(MathematicalOperators::SmallContainsAsMember),
            '∎' => Ok(MathematicalOperators::EndOfProof),
            '∏' => Ok(MathematicalOperators::NDashAryProduct),
            '∐' => Ok(MathematicalOperators::NDashAryCoproduct),
            '∑' => Ok(MathematicalOperators::NDashArySummation),
            '−' => Ok(MathematicalOperators::MinusSign),
            '∓' => Ok(MathematicalOperators::MinusDashOrDashPlusSign),
            '∔' => Ok(MathematicalOperators::DotPlus),
            '∕' => Ok(MathematicalOperators::DivisionSlash),
            '∖' => Ok(MathematicalOperators::SetMinus),
            '∗' => Ok(MathematicalOperators::AsteriskOperator),
            '∘' => Ok(MathematicalOperators::RingOperator),
            '∙' => Ok(MathematicalOperators::BulletOperator),
            '√' => Ok(MathematicalOperators::SquareRoot),
            '∛' => Ok(MathematicalOperators::CubeRoot),
            '∜' => Ok(MathematicalOperators::FourthRoot),
            '∝' => Ok(MathematicalOperators::ProportionalTo),
            '∞' => Ok(MathematicalOperators::Infinity),
            '∟' => Ok(MathematicalOperators::RightAngle),
            '∠' => Ok(MathematicalOperators::Angle),
            '∡' => Ok(MathematicalOperators::MeasuredAngle),
            '∢' => Ok(MathematicalOperators::SphericalAngle),
            '∣' => Ok(MathematicalOperators::Divides),
            '∤' => Ok(MathematicalOperators::DoesNotDivide),
            '∥' => Ok(MathematicalOperators::ParallelTo),
            '∦' => Ok(MathematicalOperators::NotParallelTo),
            '∧' => Ok(MathematicalOperators::LogicalAnd),
            '∨' => Ok(MathematicalOperators::LogicalOr),
            '∩' => Ok(MathematicalOperators::Intersection),
            '∪' => Ok(MathematicalOperators::Union),
            '∫' => Ok(MathematicalOperators::Integral),
            '∬' => Ok(MathematicalOperators::DoubleIntegral),
            '∭' => Ok(MathematicalOperators::TripleIntegral),
            '∮' => Ok(MathematicalOperators::ContourIntegral),
            '∯' => Ok(MathematicalOperators::SurfaceIntegral),
            '∰' => Ok(MathematicalOperators::VolumeIntegral),
            '∱' => Ok(MathematicalOperators::ClockwiseIntegral),
            '∲' => Ok(MathematicalOperators::ClockwiseContourIntegral),
            '∳' => Ok(MathematicalOperators::AnticlockwiseContourIntegral),
            '∴' => Ok(MathematicalOperators::Therefore),
            '∵' => Ok(MathematicalOperators::Because),
            '∶' => Ok(MathematicalOperators::Ratio),
            '∷' => Ok(MathematicalOperators::Proportion),
            '∸' => Ok(MathematicalOperators::DotMinus),
            '∹' => Ok(MathematicalOperators::Excess),
            '∺' => Ok(MathematicalOperators::GeometricProportion),
            '∻' => Ok(MathematicalOperators::Homothetic),
            '∼' => Ok(MathematicalOperators::TildeOperator),
            '∽' => Ok(MathematicalOperators::ReversedTilde),
            '∾' => Ok(MathematicalOperators::InvertedLazyS),
            '∿' => Ok(MathematicalOperators::SineWave),
            '≀' => Ok(MathematicalOperators::WreathProduct),
            '≁' => Ok(MathematicalOperators::NotTilde),
            '≂' => Ok(MathematicalOperators::MinusTilde),
            '≃' => Ok(MathematicalOperators::AsymptoticallyEqualTo),
            '≄' => Ok(MathematicalOperators::NotAsymptoticallyEqualTo),
            '≅' => Ok(MathematicalOperators::ApproximatelyEqualTo),
            '≆' => Ok(MathematicalOperators::ApproximatelyButNotActuallyEqualTo),
            '≇' => Ok(MathematicalOperators::NeitherApproximatelyNorActuallyEqualTo),
            '≈' => Ok(MathematicalOperators::AlmostEqualTo),
            '≉' => Ok(MathematicalOperators::NotAlmostEqualTo),
            '≊' => Ok(MathematicalOperators::AlmostEqualOrEqualTo),
            '≋' => Ok(MathematicalOperators::TripleTilde),
            '≌' => Ok(MathematicalOperators::AllEqualTo),
            '≍' => Ok(MathematicalOperators::EquivalentTo),
            '≎' => Ok(MathematicalOperators::GeometricallyEquivalentTo),
            '≏' => Ok(MathematicalOperators::DifferenceBetween),
            '≐' => Ok(MathematicalOperators::ApproachesTheLimit),
            '≑' => Ok(MathematicalOperators::GeometricallyEqualTo),
            '≒' => Ok(MathematicalOperators::ApproximatelyEqualToOrTheImageOf),
            '≓' => Ok(MathematicalOperators::ImageOfOrApproximatelyEqualTo),
            '≔' => Ok(MathematicalOperators::ColonEquals),
            '≕' => Ok(MathematicalOperators::EqualsColon),
            '≖' => Ok(MathematicalOperators::RingInEqualTo),
            '≗' => Ok(MathematicalOperators::RingEqualTo),
            '≘' => Ok(MathematicalOperators::CorrespondsTo),
            '≙' => Ok(MathematicalOperators::Estimates),
            '≚' => Ok(MathematicalOperators::EquiangularTo),
            '≛' => Ok(MathematicalOperators::StarEquals),
            '≜' => Ok(MathematicalOperators::DeltaEqualTo),
            '≝' => Ok(MathematicalOperators::EqualToByDefinition),
            '≞' => Ok(MathematicalOperators::MeasuredBy),
            '≟' => Ok(MathematicalOperators::QuestionedEqualTo),
            '≠' => Ok(MathematicalOperators::NotEqualTo),
            '≡' => Ok(MathematicalOperators::IdenticalTo),
            '≢' => Ok(MathematicalOperators::NotIdenticalTo),
            '≣' => Ok(MathematicalOperators::StrictlyEquivalentTo),
            '≤' => Ok(MathematicalOperators::LessDashThanOrEqualTo),
            '≥' => Ok(MathematicalOperators::GreaterDashThanOrEqualTo),
            '≦' => Ok(MathematicalOperators::LessDashThanOverEqualTo),
            '≧' => Ok(MathematicalOperators::GreaterDashThanOverEqualTo),
            '≨' => Ok(MathematicalOperators::LessDashThanButNotEqualTo),
            '≩' => Ok(MathematicalOperators::GreaterDashThanButNotEqualTo),
            '≪' => Ok(MathematicalOperators::MuchLessDashThan),
            '≫' => Ok(MathematicalOperators::MuchGreaterDashThan),
            '≬' => Ok(MathematicalOperators::Between),
            '≭' => Ok(MathematicalOperators::NotEquivalentTo),
            '≮' => Ok(MathematicalOperators::NotLessDashThan),
            '≯' => Ok(MathematicalOperators::NotGreaterDashThan),
            '≰' => Ok(MathematicalOperators::NeitherLessDashThanNorEqualTo),
            '≱' => Ok(MathematicalOperators::NeitherGreaterDashThanNorEqualTo),
            '≲' => Ok(MathematicalOperators::LessDashThanOrEquivalentTo),
            '≳' => Ok(MathematicalOperators::GreaterDashThanOrEquivalentTo),
            '≴' => Ok(MathematicalOperators::NeitherLessDashThanNorEquivalentTo),
            '≵' => Ok(MathematicalOperators::NeitherGreaterDashThanNorEquivalentTo),
            '≶' => Ok(MathematicalOperators::LessDashThanOrGreaterDashThan),
            '≷' => Ok(MathematicalOperators::GreaterDashThanOrLessDashThan),
            '≸' => Ok(MathematicalOperators::NeitherLessDashThanNorGreaterDashThan),
            '≹' => Ok(MathematicalOperators::NeitherGreaterDashThanNorLessDashThan),
            '≺' => Ok(MathematicalOperators::Precedes),
            '≻' => Ok(MathematicalOperators::Succeeds),
            '≼' => Ok(MathematicalOperators::PrecedesOrEqualTo),
            '≽' => Ok(MathematicalOperators::SucceedsOrEqualTo),
            '≾' => Ok(MathematicalOperators::PrecedesOrEquivalentTo),
            '≿' => Ok(MathematicalOperators::SucceedsOrEquivalentTo),
            '⊀' => Ok(MathematicalOperators::DoesNotPrecede),
            '⊁' => Ok(MathematicalOperators::DoesNotSucceed),
            '⊂' => Ok(MathematicalOperators::SubsetOf),
            '⊃' => Ok(MathematicalOperators::SupersetOf),
            '⊄' => Ok(MathematicalOperators::NotASubsetOf),
            '⊅' => Ok(MathematicalOperators::NotASupersetOf),
            '⊆' => Ok(MathematicalOperators::SubsetOfOrEqualTo),
            '⊇' => Ok(MathematicalOperators::SupersetOfOrEqualTo),
            '⊈' => Ok(MathematicalOperators::NeitherASubsetOfNorEqualTo),
            '⊉' => Ok(MathematicalOperators::NeitherASupersetOfNorEqualTo),
            '⊊' => Ok(MathematicalOperators::SubsetOfWithNotEqualTo),
            '⊋' => Ok(MathematicalOperators::SupersetOfWithNotEqualTo),
            '⊌' => Ok(MathematicalOperators::Multiset),
            '⊍' => Ok(MathematicalOperators::MultisetMultiplication),
            '⊎' => Ok(MathematicalOperators::MultisetUnion),
            '⊏' => Ok(MathematicalOperators::SquareImageOf),
            '⊐' => Ok(MathematicalOperators::SquareOriginalOf),
            '⊑' => Ok(MathematicalOperators::SquareImageOfOrEqualTo),
            '⊒' => Ok(MathematicalOperators::SquareOriginalOfOrEqualTo),
            '⊓' => Ok(MathematicalOperators::SquareCap),
            '⊔' => Ok(MathematicalOperators::SquareCup),
            '⊕' => Ok(MathematicalOperators::CircledPlus),
            '⊖' => Ok(MathematicalOperators::CircledMinus),
            '⊗' => Ok(MathematicalOperators::CircledTimes),
            '⊘' => Ok(MathematicalOperators::CircledDivisionSlash),
            '⊙' => Ok(MathematicalOperators::CircledDotOperator),
            '⊚' => Ok(MathematicalOperators::CircledRingOperator),
            '⊛' => Ok(MathematicalOperators::CircledAsteriskOperator),
            '⊜' => Ok(MathematicalOperators::CircledEquals),
            '⊝' => Ok(MathematicalOperators::CircledDash),
            '⊞' => Ok(MathematicalOperators::SquaredPlus),
            '⊟' => Ok(MathematicalOperators::SquaredMinus),
            '⊠' => Ok(MathematicalOperators::SquaredTimes),
            '⊡' => Ok(MathematicalOperators::SquaredDotOperator),
            '⊢' => Ok(MathematicalOperators::RightTack),
            '⊣' => Ok(MathematicalOperators::LeftTack),
            '⊤' => Ok(MathematicalOperators::DownTack),
            '⊥' => Ok(MathematicalOperators::UpTack),
            '⊦' => Ok(MathematicalOperators::Assertion),
            '⊧' => Ok(MathematicalOperators::Models),
            '⊨' => Ok(MathematicalOperators::True),
            '⊩' => Ok(MathematicalOperators::Forces),
            '⊪' => Ok(MathematicalOperators::TripleVerticalBarRightTurnstile),
            '⊫' => Ok(MathematicalOperators::DoubleVerticalBarDoubleRightTurnstile),
            '⊬' => Ok(MathematicalOperators::DoesNotProve),
            '⊭' => Ok(MathematicalOperators::NotTrue),
            '⊮' => Ok(MathematicalOperators::DoesNotForce),
            '⊯' => Ok(MathematicalOperators::NegatedDoubleVerticalBarDoubleRightTurnstile),
            '⊰' => Ok(MathematicalOperators::PrecedesUnderRelation),
            '⊱' => Ok(MathematicalOperators::SucceedsUnderRelation),
            '⊲' => Ok(MathematicalOperators::NormalSubgroupOf),
            '⊳' => Ok(MathematicalOperators::ContainsAsNormalSubgroup),
            '⊴' => Ok(MathematicalOperators::NormalSubgroupOfOrEqualTo),
            '⊵' => Ok(MathematicalOperators::ContainsAsNormalSubgroupOrEqualTo),
            '⊶' => Ok(MathematicalOperators::OriginalOf),
            '⊷' => Ok(MathematicalOperators::ImageOf),
            '⊸' => Ok(MathematicalOperators::Multimap),
            '⊹' => Ok(MathematicalOperators::HermitianConjugateMatrix),
            '⊺' => Ok(MathematicalOperators::Intercalate),
            '⊻' => Ok(MathematicalOperators::Xor),
            '⊼' => Ok(MathematicalOperators::Nand),
            '⊽' => Ok(MathematicalOperators::Nor),
            '⊾' => Ok(MathematicalOperators::RightAngleWithArc),
            '⊿' => Ok(MathematicalOperators::RightTriangle),
            '⋀' => Ok(MathematicalOperators::NDashAryLogicalAnd),
            '⋁' => Ok(MathematicalOperators::NDashAryLogicalOr),
            '⋂' => Ok(MathematicalOperators::NDashAryIntersection),
            '⋃' => Ok(MathematicalOperators::NDashAryUnion),
            '⋄' => Ok(MathematicalOperators::DiamondOperator),
            '⋅' => Ok(MathematicalOperators::DotOperator),
            '⋆' => Ok(MathematicalOperators::StarOperator),
            '⋇' => Ok(MathematicalOperators::DivisionTimes),
            '⋈' => Ok(MathematicalOperators::Bowtie),
            '⋉' => Ok(MathematicalOperators::LeftNormalFactorSemidirectProduct),
            '⋊' => Ok(MathematicalOperators::RightNormalFactorSemidirectProduct),
            '⋋' => Ok(MathematicalOperators::LeftSemidirectProduct),
            '⋌' => Ok(MathematicalOperators::RightSemidirectProduct),
            '⋍' => Ok(MathematicalOperators::ReversedTildeEquals),
            '⋎' => Ok(MathematicalOperators::CurlyLogicalOr),
            '⋏' => Ok(MathematicalOperators::CurlyLogicalAnd),
            '⋐' => Ok(MathematicalOperators::DoubleSubset),
            '⋑' => Ok(MathematicalOperators::DoubleSuperset),
            '⋒' => Ok(MathematicalOperators::DoubleIntersection),
            '⋓' => Ok(MathematicalOperators::DoubleUnion),
            '⋔' => Ok(MathematicalOperators::Pitchfork),
            '⋕' => Ok(MathematicalOperators::EqualAndParallelTo),
            '⋖' => Ok(MathematicalOperators::LessDashThanWithDot),
            '⋗' => Ok(MathematicalOperators::GreaterDashThanWithDot),
            '⋘' => Ok(MathematicalOperators::VeryMuchLessDashThan),
            '⋙' => Ok(MathematicalOperators::VeryMuchGreaterDashThan),
            '⋚' => Ok(MathematicalOperators::LessDashThanEqualToOrGreaterDashThan),
            '⋛' => Ok(MathematicalOperators::GreaterDashThanEqualToOrLessDashThan),
            '⋜' => Ok(MathematicalOperators::EqualToOrLessDashThan),
            '⋝' => Ok(MathematicalOperators::EqualToOrGreaterDashThan),
            '⋞' => Ok(MathematicalOperators::EqualToOrPrecedes),
            '⋟' => Ok(MathematicalOperators::EqualToOrSucceeds),
            '⋠' => Ok(MathematicalOperators::DoesNotPrecedeOrEqual),
            '⋡' => Ok(MathematicalOperators::DoesNotSucceedOrEqual),
            '⋢' => Ok(MathematicalOperators::NotSquareImageOfOrEqualTo),
            '⋣' => Ok(MathematicalOperators::NotSquareOriginalOfOrEqualTo),
            '⋤' => Ok(MathematicalOperators::SquareImageOfOrNotEqualTo),
            '⋥' => Ok(MathematicalOperators::SquareOriginalOfOrNotEqualTo),
            '⋦' => Ok(MathematicalOperators::LessDashThanButNotEquivalentTo),
            '⋧' => Ok(MathematicalOperators::GreaterDashThanButNotEquivalentTo),
            '⋨' => Ok(MathematicalOperators::PrecedesButNotEquivalentTo),
            '⋩' => Ok(MathematicalOperators::SucceedsButNotEquivalentTo),
            '⋪' => Ok(MathematicalOperators::NotNormalSubgroupOf),
            '⋫' => Ok(MathematicalOperators::DoesNotContainAsNormalSubgroup),
            '⋬' => Ok(MathematicalOperators::NotNormalSubgroupOfOrEqualTo),
            '⋭' => Ok(MathematicalOperators::DoesNotContainAsNormalSubgroupOrEqual),
            '⋮' => Ok(MathematicalOperators::VerticalEllipsis),
            '⋯' => Ok(MathematicalOperators::MidlineHorizontalEllipsis),
            '⋰' => Ok(MathematicalOperators::UpRightDiagonalEllipsis),
            '⋱' => Ok(MathematicalOperators::DownRightDiagonalEllipsis),
            '⋲' => Ok(MathematicalOperators::ElementOfWithLongHorizontalStroke),
            '⋳' => Ok(MathematicalOperators::ElementOfWithVerticalBarAtEndOfHorizontalStroke),
            '⋴' => Ok(MathematicalOperators::SmallElementOfWithVerticalBarAtEndOfHorizontalStroke),
            '⋵' => Ok(MathematicalOperators::ElementOfWithDotAbove),
            '⋶' => Ok(MathematicalOperators::ElementOfWithOverbar),
            '⋷' => Ok(MathematicalOperators::SmallElementOfWithOverbar),
            '⋸' => Ok(MathematicalOperators::ElementOfWithUnderbar),
            '⋹' => Ok(MathematicalOperators::ElementOfWithTwoHorizontalStrokes),
            '⋺' => Ok(MathematicalOperators::ContainsWithLongHorizontalStroke),
            '⋻' => Ok(MathematicalOperators::ContainsWithVerticalBarAtEndOfHorizontalStroke),
            '⋼' => Ok(MathematicalOperators::SmallContainsWithVerticalBarAtEndOfHorizontalStroke),
            '⋽' => Ok(MathematicalOperators::ContainsWithOverbar),
            '⋾' => Ok(MathematicalOperators::SmallContainsWithOverbar),
            _ => Err(()),
        }
    }
}

impl Into<u32> for MathematicalOperators {
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

impl std::convert::TryFrom<u32> for MathematicalOperators {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for MathematicalOperators {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl MathematicalOperators {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        MathematicalOperators::ForAll
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            MathematicalOperators::ForAll => "for all",
            MathematicalOperators::Complement => "complement",
            MathematicalOperators::PartialDifferential => "partial differential",
            MathematicalOperators::ThereExists => "there exists",
            MathematicalOperators::ThereDoesNotExist => "there does not exist",
            MathematicalOperators::EmptySet => "empty set",
            MathematicalOperators::Increment => "increment",
            MathematicalOperators::Nabla => "nabla",
            MathematicalOperators::ElementOf => "element of",
            MathematicalOperators::NotAnElementOf => "not an element of",
            MathematicalOperators::SmallElementOf => "small element of",
            MathematicalOperators::ContainsAsMember => "contains as member",
            MathematicalOperators::DoesNotContainAsMember => "does not contain as member",
            MathematicalOperators::SmallContainsAsMember => "small contains as member",
            MathematicalOperators::EndOfProof => "end of proof",
            MathematicalOperators::NDashAryProduct => "n-ary product",
            MathematicalOperators::NDashAryCoproduct => "n-ary coproduct",
            MathematicalOperators::NDashArySummation => "n-ary summation",
            MathematicalOperators::MinusSign => "minus sign",
            MathematicalOperators::MinusDashOrDashPlusSign => "minus-or-plus sign",
            MathematicalOperators::DotPlus => "dot plus",
            MathematicalOperators::DivisionSlash => "division slash",
            MathematicalOperators::SetMinus => "set minus",
            MathematicalOperators::AsteriskOperator => "asterisk operator",
            MathematicalOperators::RingOperator => "ring operator",
            MathematicalOperators::BulletOperator => "bullet operator",
            MathematicalOperators::SquareRoot => "square root",
            MathematicalOperators::CubeRoot => "cube root",
            MathematicalOperators::FourthRoot => "fourth root",
            MathematicalOperators::ProportionalTo => "proportional to",
            MathematicalOperators::Infinity => "infinity",
            MathematicalOperators::RightAngle => "right angle",
            MathematicalOperators::Angle => "angle",
            MathematicalOperators::MeasuredAngle => "measured angle",
            MathematicalOperators::SphericalAngle => "spherical angle",
            MathematicalOperators::Divides => "divides",
            MathematicalOperators::DoesNotDivide => "does not divide",
            MathematicalOperators::ParallelTo => "parallel to",
            MathematicalOperators::NotParallelTo => "not parallel to",
            MathematicalOperators::LogicalAnd => "logical and",
            MathematicalOperators::LogicalOr => "logical or",
            MathematicalOperators::Intersection => "intersection",
            MathematicalOperators::Union => "union",
            MathematicalOperators::Integral => "integral",
            MathematicalOperators::DoubleIntegral => "double integral",
            MathematicalOperators::TripleIntegral => "triple integral",
            MathematicalOperators::ContourIntegral => "contour integral",
            MathematicalOperators::SurfaceIntegral => "surface integral",
            MathematicalOperators::VolumeIntegral => "volume integral",
            MathematicalOperators::ClockwiseIntegral => "clockwise integral",
            MathematicalOperators::ClockwiseContourIntegral => "clockwise contour integral",
            MathematicalOperators::AnticlockwiseContourIntegral => "anticlockwise contour integral",
            MathematicalOperators::Therefore => "therefore",
            MathematicalOperators::Because => "because",
            MathematicalOperators::Ratio => "ratio",
            MathematicalOperators::Proportion => "proportion",
            MathematicalOperators::DotMinus => "dot minus",
            MathematicalOperators::Excess => "excess",
            MathematicalOperators::GeometricProportion => "geometric proportion",
            MathematicalOperators::Homothetic => "homothetic",
            MathematicalOperators::TildeOperator => "tilde operator",
            MathematicalOperators::ReversedTilde => "reversed tilde",
            MathematicalOperators::InvertedLazyS => "inverted lazy s",
            MathematicalOperators::SineWave => "sine wave",
            MathematicalOperators::WreathProduct => "wreath product",
            MathematicalOperators::NotTilde => "not tilde",
            MathematicalOperators::MinusTilde => "minus tilde",
            MathematicalOperators::AsymptoticallyEqualTo => "asymptotically equal to",
            MathematicalOperators::NotAsymptoticallyEqualTo => "not asymptotically equal to",
            MathematicalOperators::ApproximatelyEqualTo => "approximately equal to",
            MathematicalOperators::ApproximatelyButNotActuallyEqualTo => "approximately but not actually equal to",
            MathematicalOperators::NeitherApproximatelyNorActuallyEqualTo => "neither approximately nor actually equal to",
            MathematicalOperators::AlmostEqualTo => "almost equal to",
            MathematicalOperators::NotAlmostEqualTo => "not almost equal to",
            MathematicalOperators::AlmostEqualOrEqualTo => "almost equal or equal to",
            MathematicalOperators::TripleTilde => "triple tilde",
            MathematicalOperators::AllEqualTo => "all equal to",
            MathematicalOperators::EquivalentTo => "equivalent to",
            MathematicalOperators::GeometricallyEquivalentTo => "geometrically equivalent to",
            MathematicalOperators::DifferenceBetween => "difference between",
            MathematicalOperators::ApproachesTheLimit => "approaches the limit",
            MathematicalOperators::GeometricallyEqualTo => "geometrically equal to",
            MathematicalOperators::ApproximatelyEqualToOrTheImageOf => "approximately equal to or the image of",
            MathematicalOperators::ImageOfOrApproximatelyEqualTo => "image of or approximately equal to",
            MathematicalOperators::ColonEquals => "colon equals",
            MathematicalOperators::EqualsColon => "equals colon",
            MathematicalOperators::RingInEqualTo => "ring in equal to",
            MathematicalOperators::RingEqualTo => "ring equal to",
            MathematicalOperators::CorrespondsTo => "corresponds to",
            MathematicalOperators::Estimates => "estimates",
            MathematicalOperators::EquiangularTo => "equiangular to",
            MathematicalOperators::StarEquals => "star equals",
            MathematicalOperators::DeltaEqualTo => "delta equal to",
            MathematicalOperators::EqualToByDefinition => "equal to by definition",
            MathematicalOperators::MeasuredBy => "measured by",
            MathematicalOperators::QuestionedEqualTo => "questioned equal to",
            MathematicalOperators::NotEqualTo => "not equal to",
            MathematicalOperators::IdenticalTo => "identical to",
            MathematicalOperators::NotIdenticalTo => "not identical to",
            MathematicalOperators::StrictlyEquivalentTo => "strictly equivalent to",
            MathematicalOperators::LessDashThanOrEqualTo => "less-than or equal to",
            MathematicalOperators::GreaterDashThanOrEqualTo => "greater-than or equal to",
            MathematicalOperators::LessDashThanOverEqualTo => "less-than over equal to",
            MathematicalOperators::GreaterDashThanOverEqualTo => "greater-than over equal to",
            MathematicalOperators::LessDashThanButNotEqualTo => "less-than but not equal to",
            MathematicalOperators::GreaterDashThanButNotEqualTo => "greater-than but not equal to",
            MathematicalOperators::MuchLessDashThan => "much less-than",
            MathematicalOperators::MuchGreaterDashThan => "much greater-than",
            MathematicalOperators::Between => "between",
            MathematicalOperators::NotEquivalentTo => "not equivalent to",
            MathematicalOperators::NotLessDashThan => "not less-than",
            MathematicalOperators::NotGreaterDashThan => "not greater-than",
            MathematicalOperators::NeitherLessDashThanNorEqualTo => "neither less-than nor equal to",
            MathematicalOperators::NeitherGreaterDashThanNorEqualTo => "neither greater-than nor equal to",
            MathematicalOperators::LessDashThanOrEquivalentTo => "less-than or equivalent to",
            MathematicalOperators::GreaterDashThanOrEquivalentTo => "greater-than or equivalent to",
            MathematicalOperators::NeitherLessDashThanNorEquivalentTo => "neither less-than nor equivalent to",
            MathematicalOperators::NeitherGreaterDashThanNorEquivalentTo => "neither greater-than nor equivalent to",
            MathematicalOperators::LessDashThanOrGreaterDashThan => "less-than or greater-than",
            MathematicalOperators::GreaterDashThanOrLessDashThan => "greater-than or less-than",
            MathematicalOperators::NeitherLessDashThanNorGreaterDashThan => "neither less-than nor greater-than",
            MathematicalOperators::NeitherGreaterDashThanNorLessDashThan => "neither greater-than nor less-than",
            MathematicalOperators::Precedes => "precedes",
            MathematicalOperators::Succeeds => "succeeds",
            MathematicalOperators::PrecedesOrEqualTo => "precedes or equal to",
            MathematicalOperators::SucceedsOrEqualTo => "succeeds or equal to",
            MathematicalOperators::PrecedesOrEquivalentTo => "precedes or equivalent to",
            MathematicalOperators::SucceedsOrEquivalentTo => "succeeds or equivalent to",
            MathematicalOperators::DoesNotPrecede => "does not precede",
            MathematicalOperators::DoesNotSucceed => "does not succeed",
            MathematicalOperators::SubsetOf => "subset of",
            MathematicalOperators::SupersetOf => "superset of",
            MathematicalOperators::NotASubsetOf => "not a subset of",
            MathematicalOperators::NotASupersetOf => "not a superset of",
            MathematicalOperators::SubsetOfOrEqualTo => "subset of or equal to",
            MathematicalOperators::SupersetOfOrEqualTo => "superset of or equal to",
            MathematicalOperators::NeitherASubsetOfNorEqualTo => "neither a subset of nor equal to",
            MathematicalOperators::NeitherASupersetOfNorEqualTo => "neither a superset of nor equal to",
            MathematicalOperators::SubsetOfWithNotEqualTo => "subset of with not equal to",
            MathematicalOperators::SupersetOfWithNotEqualTo => "superset of with not equal to",
            MathematicalOperators::Multiset => "multiset",
            MathematicalOperators::MultisetMultiplication => "multiset multiplication",
            MathematicalOperators::MultisetUnion => "multiset union",
            MathematicalOperators::SquareImageOf => "square image of",
            MathematicalOperators::SquareOriginalOf => "square original of",
            MathematicalOperators::SquareImageOfOrEqualTo => "square image of or equal to",
            MathematicalOperators::SquareOriginalOfOrEqualTo => "square original of or equal to",
            MathematicalOperators::SquareCap => "square cap",
            MathematicalOperators::SquareCup => "square cup",
            MathematicalOperators::CircledPlus => "circled plus",
            MathematicalOperators::CircledMinus => "circled minus",
            MathematicalOperators::CircledTimes => "circled times",
            MathematicalOperators::CircledDivisionSlash => "circled division slash",
            MathematicalOperators::CircledDotOperator => "circled dot operator",
            MathematicalOperators::CircledRingOperator => "circled ring operator",
            MathematicalOperators::CircledAsteriskOperator => "circled asterisk operator",
            MathematicalOperators::CircledEquals => "circled equals",
            MathematicalOperators::CircledDash => "circled dash",
            MathematicalOperators::SquaredPlus => "squared plus",
            MathematicalOperators::SquaredMinus => "squared minus",
            MathematicalOperators::SquaredTimes => "squared times",
            MathematicalOperators::SquaredDotOperator => "squared dot operator",
            MathematicalOperators::RightTack => "right tack",
            MathematicalOperators::LeftTack => "left tack",
            MathematicalOperators::DownTack => "down tack",
            MathematicalOperators::UpTack => "up tack",
            MathematicalOperators::Assertion => "assertion",
            MathematicalOperators::Models => "models",
            MathematicalOperators::True => "true",
            MathematicalOperators::Forces => "forces",
            MathematicalOperators::TripleVerticalBarRightTurnstile => "triple vertical bar right turnstile",
            MathematicalOperators::DoubleVerticalBarDoubleRightTurnstile => "double vertical bar double right turnstile",
            MathematicalOperators::DoesNotProve => "does not prove",
            MathematicalOperators::NotTrue => "not true",
            MathematicalOperators::DoesNotForce => "does not force",
            MathematicalOperators::NegatedDoubleVerticalBarDoubleRightTurnstile => "negated double vertical bar double right turnstile",
            MathematicalOperators::PrecedesUnderRelation => "precedes under relation",
            MathematicalOperators::SucceedsUnderRelation => "succeeds under relation",
            MathematicalOperators::NormalSubgroupOf => "normal subgroup of",
            MathematicalOperators::ContainsAsNormalSubgroup => "contains as normal subgroup",
            MathematicalOperators::NormalSubgroupOfOrEqualTo => "normal subgroup of or equal to",
            MathematicalOperators::ContainsAsNormalSubgroupOrEqualTo => "contains as normal subgroup or equal to",
            MathematicalOperators::OriginalOf => "original of",
            MathematicalOperators::ImageOf => "image of",
            MathematicalOperators::Multimap => "multimap",
            MathematicalOperators::HermitianConjugateMatrix => "hermitian conjugate matrix",
            MathematicalOperators::Intercalate => "intercalate",
            MathematicalOperators::Xor => "xor",
            MathematicalOperators::Nand => "nand",
            MathematicalOperators::Nor => "nor",
            MathematicalOperators::RightAngleWithArc => "right angle with arc",
            MathematicalOperators::RightTriangle => "right triangle",
            MathematicalOperators::NDashAryLogicalAnd => "n-ary logical and",
            MathematicalOperators::NDashAryLogicalOr => "n-ary logical or",
            MathematicalOperators::NDashAryIntersection => "n-ary intersection",
            MathematicalOperators::NDashAryUnion => "n-ary union",
            MathematicalOperators::DiamondOperator => "diamond operator",
            MathematicalOperators::DotOperator => "dot operator",
            MathematicalOperators::StarOperator => "star operator",
            MathematicalOperators::DivisionTimes => "division times",
            MathematicalOperators::Bowtie => "bowtie",
            MathematicalOperators::LeftNormalFactorSemidirectProduct => "left normal factor semidirect product",
            MathematicalOperators::RightNormalFactorSemidirectProduct => "right normal factor semidirect product",
            MathematicalOperators::LeftSemidirectProduct => "left semidirect product",
            MathematicalOperators::RightSemidirectProduct => "right semidirect product",
            MathematicalOperators::ReversedTildeEquals => "reversed tilde equals",
            MathematicalOperators::CurlyLogicalOr => "curly logical or",
            MathematicalOperators::CurlyLogicalAnd => "curly logical and",
            MathematicalOperators::DoubleSubset => "double subset",
            MathematicalOperators::DoubleSuperset => "double superset",
            MathematicalOperators::DoubleIntersection => "double intersection",
            MathematicalOperators::DoubleUnion => "double union",
            MathematicalOperators::Pitchfork => "pitchfork",
            MathematicalOperators::EqualAndParallelTo => "equal and parallel to",
            MathematicalOperators::LessDashThanWithDot => "less-than with dot",
            MathematicalOperators::GreaterDashThanWithDot => "greater-than with dot",
            MathematicalOperators::VeryMuchLessDashThan => "very much less-than",
            MathematicalOperators::VeryMuchGreaterDashThan => "very much greater-than",
            MathematicalOperators::LessDashThanEqualToOrGreaterDashThan => "less-than equal to or greater-than",
            MathematicalOperators::GreaterDashThanEqualToOrLessDashThan => "greater-than equal to or less-than",
            MathematicalOperators::EqualToOrLessDashThan => "equal to or less-than",
            MathematicalOperators::EqualToOrGreaterDashThan => "equal to or greater-than",
            MathematicalOperators::EqualToOrPrecedes => "equal to or precedes",
            MathematicalOperators::EqualToOrSucceeds => "equal to or succeeds",
            MathematicalOperators::DoesNotPrecedeOrEqual => "does not precede or equal",
            MathematicalOperators::DoesNotSucceedOrEqual => "does not succeed or equal",
            MathematicalOperators::NotSquareImageOfOrEqualTo => "not square image of or equal to",
            MathematicalOperators::NotSquareOriginalOfOrEqualTo => "not square original of or equal to",
            MathematicalOperators::SquareImageOfOrNotEqualTo => "square image of or not equal to",
            MathematicalOperators::SquareOriginalOfOrNotEqualTo => "square original of or not equal to",
            MathematicalOperators::LessDashThanButNotEquivalentTo => "less-than but not equivalent to",
            MathematicalOperators::GreaterDashThanButNotEquivalentTo => "greater-than but not equivalent to",
            MathematicalOperators::PrecedesButNotEquivalentTo => "precedes but not equivalent to",
            MathematicalOperators::SucceedsButNotEquivalentTo => "succeeds but not equivalent to",
            MathematicalOperators::NotNormalSubgroupOf => "not normal subgroup of",
            MathematicalOperators::DoesNotContainAsNormalSubgroup => "does not contain as normal subgroup",
            MathematicalOperators::NotNormalSubgroupOfOrEqualTo => "not normal subgroup of or equal to",
            MathematicalOperators::DoesNotContainAsNormalSubgroupOrEqual => "does not contain as normal subgroup or equal",
            MathematicalOperators::VerticalEllipsis => "vertical ellipsis",
            MathematicalOperators::MidlineHorizontalEllipsis => "midline horizontal ellipsis",
            MathematicalOperators::UpRightDiagonalEllipsis => "up right diagonal ellipsis",
            MathematicalOperators::DownRightDiagonalEllipsis => "down right diagonal ellipsis",
            MathematicalOperators::ElementOfWithLongHorizontalStroke => "element of with long horizontal stroke",
            MathematicalOperators::ElementOfWithVerticalBarAtEndOfHorizontalStroke => "element of with vertical bar at end of horizontal stroke",
            MathematicalOperators::SmallElementOfWithVerticalBarAtEndOfHorizontalStroke => "small element of with vertical bar at end of horizontal stroke",
            MathematicalOperators::ElementOfWithDotAbove => "element of with dot above",
            MathematicalOperators::ElementOfWithOverbar => "element of with overbar",
            MathematicalOperators::SmallElementOfWithOverbar => "small element of with overbar",
            MathematicalOperators::ElementOfWithUnderbar => "element of with underbar",
            MathematicalOperators::ElementOfWithTwoHorizontalStrokes => "element of with two horizontal strokes",
            MathematicalOperators::ContainsWithLongHorizontalStroke => "contains with long horizontal stroke",
            MathematicalOperators::ContainsWithVerticalBarAtEndOfHorizontalStroke => "contains with vertical bar at end of horizontal stroke",
            MathematicalOperators::SmallContainsWithVerticalBarAtEndOfHorizontalStroke => "small contains with vertical bar at end of horizontal stroke",
            MathematicalOperators::ContainsWithOverbar => "contains with overbar",
            MathematicalOperators::SmallContainsWithOverbar => "small contains with overbar",
        }
    }
}
