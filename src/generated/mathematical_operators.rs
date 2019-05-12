
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

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("MathematicalOperators{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
