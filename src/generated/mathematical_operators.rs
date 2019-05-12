/// \u{2200} → \u{22ff}\
///\
/// ∀ ∁ ∂ ∃ ∄ ∅ ∆ ∇ ∈ ∉ ∊ ∋ ∌ ∍ ∎ ∏\
/// ∐ ∑ − ∓ ∔ ∕ ∖ ∗ ∘ ∙ √ ∛ ∜ ∝ ∞ ∟\
/// ∠ ∡ ∢ ∣ ∤ ∥ ∦ ∧ ∨ ∩ ∪ ∫ ∬ ∭ ∮ ∯\
/// ∰ ∱ ∲ ∳ ∴ ∵ ∶ ∷ ∸ ∹ ∺ ∻ ∼ ∽ ∾ ∿\
/// ≀ ≁ ≂ ≃ ≄ ≅ ≆ ≇ ≈ ≉ ≊ ≋ ≌ ≍ ≎ ≏\
/// ≐ ≑ ≒ ≓ ≔ ≕ ≖ ≗ ≘ ≙ ≚ ≛ ≜ ≝ ≞ ≟\
/// ≠ ≡ ≢ ≣ ≤ ≥ ≦ ≧ ≨ ≩ ≪ ≫ ≬ ≭ ≮ ≯\
/// ≰ ≱ ≲ ≳ ≴ ≵ ≶ ≷ ≸ ≹ ≺ ≻ ≼ ≽ ≾ ≿\
/// ⊀ ⊁ ⊂ ⊃ ⊄ ⊅ ⊆ ⊇ ⊈ ⊉ ⊊ ⊋ ⊌ ⊍ ⊎ ⊏\
/// ⊐ ⊑ ⊒ ⊓ ⊔ ⊕ ⊖ ⊗ ⊘ ⊙ ⊚ ⊛ ⊜ ⊝ ⊞ ⊟\
/// ⊠ ⊡ ⊢ ⊣ ⊤ ⊥ ⊦ ⊧ ⊨ ⊩ ⊪ ⊫ ⊬ ⊭ ⊮ ⊯\
/// ⊰ ⊱ ⊲ ⊳ ⊴ ⊵ ⊶ ⊷ ⊸ ⊹ ⊺ ⊻ ⊼ ⊽ ⊾ ⊿\
/// ⋀ ⋁ ⋂ ⋃ ⋄ ⋅ ⋆ ⋇ ⋈ ⋉ ⋊ ⋋ ⋌ ⋍ ⋎ ⋏\
/// ⋐ ⋑ ⋒ ⋓ ⋔ ⋕ ⋖ ⋗ ⋘ ⋙ ⋚ ⋛ ⋜ ⋝ ⋞ ⋟\
/// ⋠ ⋡ ⋢ ⋣ ⋤ ⋥ ⋦ ⋧ ⋨ ⋩ ⋪ ⋫ ⋬ ⋭ ⋮ ⋯\
/// ⋰ ⋱ ⋲ ⋳ ⋴ ⋵ ⋶ ⋷ ⋸ ⋹ ⋺ ⋻ ⋼ ⋽ ⋾\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{2200}: '∀'
    pub const FOR_ALL: char = '∀';
    /// \u{2201}: '∁'
    pub const COMPLEMENT: char = '∁';
    /// \u{2202}: '∂'
    pub const PARTIAL_DIFFERENTIAL: char = '∂';
    /// \u{2203}: '∃'
    pub const THERE_EXISTS: char = '∃';
    /// \u{2204}: '∄'
    pub const THERE_DOES_NOT_EXIST: char = '∄';
    /// \u{2205}: '∅'
    pub const EMPTY_SET: char = '∅';
    /// \u{2206}: '∆'
    pub const INCREMENT: char = '∆';
    /// \u{2207}: '∇'
    pub const NABLA: char = '∇';
    /// \u{2208}: '∈'
    pub const ELEMENT_OF: char = '∈';
    /// \u{2209}: '∉'
    pub const NOT_AN_ELEMENT_OF: char = '∉';
    /// \u{220a}: '∊'
    pub const SMALL_ELEMENT_OF: char = '∊';
    /// \u{220b}: '∋'
    pub const CONTAINS_AS_MEMBER: char = '∋';
    /// \u{220c}: '∌'
    pub const DOES_NOT_CONTAIN_AS_MEMBER: char = '∌';
    /// \u{220d}: '∍'
    pub const SMALL_CONTAINS_AS_MEMBER: char = '∍';
    /// \u{220e}: '∎'
    pub const END_OF_PROOF: char = '∎';
    /// \u{220f}: '∏'
    pub const N_DASH_ARY_PRODUCT: char = '∏';
    /// \u{2210}: '∐'
    pub const N_DASH_ARY_COPRODUCT: char = '∐';
    /// \u{2211}: '∑'
    pub const N_DASH_ARY_SUMMATION: char = '∑';
    /// \u{2212}: '−'
    pub const MINUS_SIGN: char = '−';
    /// \u{2213}: '∓'
    pub const MINUS_DASH_OR_DASH_PLUS_SIGN: char = '∓';
    /// \u{2214}: '∔'
    pub const DOT_PLUS: char = '∔';
    /// \u{2215}: '∕'
    pub const DIVISION_SLASH: char = '∕';
    /// \u{2216}: '∖'
    pub const SET_MINUS: char = '∖';
    /// \u{2217}: '∗'
    pub const ASTERISK_OPERATOR: char = '∗';
    /// \u{2218}: '∘'
    pub const RING_OPERATOR: char = '∘';
    /// \u{2219}: '∙'
    pub const BULLET_OPERATOR: char = '∙';
    /// \u{221a}: '√'
    pub const SQUARE_ROOT: char = '√';
    /// \u{221b}: '∛'
    pub const CUBE_ROOT: char = '∛';
    /// \u{221c}: '∜'
    pub const FOURTH_ROOT: char = '∜';
    /// \u{221d}: '∝'
    pub const PROPORTIONAL_TO: char = '∝';
    /// \u{221e}: '∞'
    pub const INFINITY: char = '∞';
    /// \u{221f}: '∟'
    pub const RIGHT_ANGLE: char = '∟';
    /// \u{2220}: '∠'
    pub const ANGLE: char = '∠';
    /// \u{2221}: '∡'
    pub const MEASURED_ANGLE: char = '∡';
    /// \u{2222}: '∢'
    pub const SPHERICAL_ANGLE: char = '∢';
    /// \u{2223}: '∣'
    pub const DIVIDES: char = '∣';
    /// \u{2224}: '∤'
    pub const DOES_NOT_DIVIDE: char = '∤';
    /// \u{2225}: '∥'
    pub const PARALLEL_TO: char = '∥';
    /// \u{2226}: '∦'
    pub const NOT_PARALLEL_TO: char = '∦';
    /// \u{2227}: '∧'
    pub const LOGICAL_AND: char = '∧';
    /// \u{2228}: '∨'
    pub const LOGICAL_OR: char = '∨';
    /// \u{2229}: '∩'
    pub const INTERSECTION: char = '∩';
    /// \u{222a}: '∪'
    pub const UNION: char = '∪';
    /// \u{222b}: '∫'
    pub const INTEGRAL: char = '∫';
    /// \u{222c}: '∬'
    pub const DOUBLE_INTEGRAL: char = '∬';
    /// \u{222d}: '∭'
    pub const TRIPLE_INTEGRAL: char = '∭';
    /// \u{222e}: '∮'
    pub const CONTOUR_INTEGRAL: char = '∮';
    /// \u{222f}: '∯'
    pub const SURFACE_INTEGRAL: char = '∯';
    /// \u{2230}: '∰'
    pub const VOLUME_INTEGRAL: char = '∰';
    /// \u{2231}: '∱'
    pub const CLOCKWISE_INTEGRAL: char = '∱';
    /// \u{2232}: '∲'
    pub const CLOCKWISE_CONTOUR_INTEGRAL: char = '∲';
    /// \u{2233}: '∳'
    pub const ANTICLOCKWISE_CONTOUR_INTEGRAL: char = '∳';
    /// \u{2234}: '∴'
    pub const THEREFORE: char = '∴';
    /// \u{2235}: '∵'
    pub const BECAUSE: char = '∵';
    /// \u{2236}: '∶'
    pub const RATIO: char = '∶';
    /// \u{2237}: '∷'
    pub const PROPORTION: char = '∷';
    /// \u{2238}: '∸'
    pub const DOT_MINUS: char = '∸';
    /// \u{2239}: '∹'
    pub const EXCESS: char = '∹';
    /// \u{223a}: '∺'
    pub const GEOMETRIC_PROPORTION: char = '∺';
    /// \u{223b}: '∻'
    pub const HOMOTHETIC: char = '∻';
    /// \u{223c}: '∼'
    pub const TILDE_OPERATOR: char = '∼';
    /// \u{223d}: '∽'
    pub const REVERSED_TILDE: char = '∽';
    /// \u{223e}: '∾'
    pub const INVERTED_LAZY_S: char = '∾';
    /// \u{223f}: '∿'
    pub const SINE_WAVE: char = '∿';
    /// \u{2240}: '≀'
    pub const WREATH_PRODUCT: char = '≀';
    /// \u{2241}: '≁'
    pub const NOT_TILDE: char = '≁';
    /// \u{2242}: '≂'
    pub const MINUS_TILDE: char = '≂';
    /// \u{2243}: '≃'
    pub const ASYMPTOTICALLY_EQUAL_TO: char = '≃';
    /// \u{2244}: '≄'
    pub const NOT_ASYMPTOTICALLY_EQUAL_TO: char = '≄';
    /// \u{2245}: '≅'
    pub const APPROXIMATELY_EQUAL_TO: char = '≅';
    /// \u{2246}: '≆'
    pub const APPROXIMATELY_BUT_NOT_ACTUALLY_EQUAL_TO: char = '≆';
    /// \u{2247}: '≇'
    pub const NEITHER_APPROXIMATELY_NOR_ACTUALLY_EQUAL_TO: char = '≇';
    /// \u{2248}: '≈'
    pub const ALMOST_EQUAL_TO: char = '≈';
    /// \u{2249}: '≉'
    pub const NOT_ALMOST_EQUAL_TO: char = '≉';
    /// \u{224a}: '≊'
    pub const ALMOST_EQUAL_OR_EQUAL_TO: char = '≊';
    /// \u{224b}: '≋'
    pub const TRIPLE_TILDE: char = '≋';
    /// \u{224c}: '≌'
    pub const ALL_EQUAL_TO: char = '≌';
    /// \u{224d}: '≍'
    pub const EQUIVALENT_TO: char = '≍';
    /// \u{224e}: '≎'
    pub const GEOMETRICALLY_EQUIVALENT_TO: char = '≎';
    /// \u{224f}: '≏'
    pub const DIFFERENCE_BETWEEN: char = '≏';
    /// \u{2250}: '≐'
    pub const APPROACHES_THE_LIMIT: char = '≐';
    /// \u{2251}: '≑'
    pub const GEOMETRICALLY_EQUAL_TO: char = '≑';
    /// \u{2252}: '≒'
    pub const APPROXIMATELY_EQUAL_TO_OR_THE_IMAGE_OF: char = '≒';
    /// \u{2253}: '≓'
    pub const IMAGE_OF_OR_APPROXIMATELY_EQUAL_TO: char = '≓';
    /// \u{2254}: '≔'
    pub const COLON_EQUALS: char = '≔';
    /// \u{2255}: '≕'
    pub const EQUALS_COLON: char = '≕';
    /// \u{2256}: '≖'
    pub const RING_IN_EQUAL_TO: char = '≖';
    /// \u{2257}: '≗'
    pub const RING_EQUAL_TO: char = '≗';
    /// \u{2258}: '≘'
    pub const CORRESPONDS_TO: char = '≘';
    /// \u{2259}: '≙'
    pub const ESTIMATES: char = '≙';
    /// \u{225a}: '≚'
    pub const EQUIANGULAR_TO: char = '≚';
    /// \u{225b}: '≛'
    pub const STAR_EQUALS: char = '≛';
    /// \u{225c}: '≜'
    pub const DELTA_EQUAL_TO: char = '≜';
    /// \u{225d}: '≝'
    pub const EQUAL_TO_BY_DEFINITION: char = '≝';
    /// \u{225e}: '≞'
    pub const MEASURED_BY: char = '≞';
    /// \u{225f}: '≟'
    pub const QUESTIONED_EQUAL_TO: char = '≟';
    /// \u{2260}: '≠'
    pub const NOT_EQUAL_TO: char = '≠';
    /// \u{2261}: '≡'
    pub const IDENTICAL_TO: char = '≡';
    /// \u{2262}: '≢'
    pub const NOT_IDENTICAL_TO: char = '≢';
    /// \u{2263}: '≣'
    pub const STRICTLY_EQUIVALENT_TO: char = '≣';
    /// \u{2264}: '≤'
    pub const LESS_DASH_THAN_OR_EQUAL_TO: char = '≤';
    /// \u{2265}: '≥'
    pub const GREATER_DASH_THAN_OR_EQUAL_TO: char = '≥';
    /// \u{2266}: '≦'
    pub const LESS_DASH_THAN_OVER_EQUAL_TO: char = '≦';
    /// \u{2267}: '≧'
    pub const GREATER_DASH_THAN_OVER_EQUAL_TO: char = '≧';
    /// \u{2268}: '≨'
    pub const LESS_DASH_THAN_BUT_NOT_EQUAL_TO: char = '≨';
    /// \u{2269}: '≩'
    pub const GREATER_DASH_THAN_BUT_NOT_EQUAL_TO: char = '≩';
    /// \u{226a}: '≪'
    pub const MUCH_LESS_DASH_THAN: char = '≪';
    /// \u{226b}: '≫'
    pub const MUCH_GREATER_DASH_THAN: char = '≫';
    /// \u{226c}: '≬'
    pub const BETWEEN: char = '≬';
    /// \u{226d}: '≭'
    pub const NOT_EQUIVALENT_TO: char = '≭';
    /// \u{226e}: '≮'
    pub const NOT_LESS_DASH_THAN: char = '≮';
    /// \u{226f}: '≯'
    pub const NOT_GREATER_DASH_THAN: char = '≯';
    /// \u{2270}: '≰'
    pub const NEITHER_LESS_DASH_THAN_NOR_EQUAL_TO: char = '≰';
    /// \u{2271}: '≱'
    pub const NEITHER_GREATER_DASH_THAN_NOR_EQUAL_TO: char = '≱';
    /// \u{2272}: '≲'
    pub const LESS_DASH_THAN_OR_EQUIVALENT_TO: char = '≲';
    /// \u{2273}: '≳'
    pub const GREATER_DASH_THAN_OR_EQUIVALENT_TO: char = '≳';
    /// \u{2274}: '≴'
    pub const NEITHER_LESS_DASH_THAN_NOR_EQUIVALENT_TO: char = '≴';
    /// \u{2275}: '≵'
    pub const NEITHER_GREATER_DASH_THAN_NOR_EQUIVALENT_TO: char = '≵';
    /// \u{2276}: '≶'
    pub const LESS_DASH_THAN_OR_GREATER_DASH_THAN: char = '≶';
    /// \u{2277}: '≷'
    pub const GREATER_DASH_THAN_OR_LESS_DASH_THAN: char = '≷';
    /// \u{2278}: '≸'
    pub const NEITHER_LESS_DASH_THAN_NOR_GREATER_DASH_THAN: char = '≸';
    /// \u{2279}: '≹'
    pub const NEITHER_GREATER_DASH_THAN_NOR_LESS_DASH_THAN: char = '≹';
    /// \u{227a}: '≺'
    pub const PRECEDES: char = '≺';
    /// \u{227b}: '≻'
    pub const SUCCEEDS: char = '≻';
    /// \u{227c}: '≼'
    pub const PRECEDES_OR_EQUAL_TO: char = '≼';
    /// \u{227d}: '≽'
    pub const SUCCEEDS_OR_EQUAL_TO: char = '≽';
    /// \u{227e}: '≾'
    pub const PRECEDES_OR_EQUIVALENT_TO: char = '≾';
    /// \u{227f}: '≿'
    pub const SUCCEEDS_OR_EQUIVALENT_TO: char = '≿';
    /// \u{2280}: '⊀'
    pub const DOES_NOT_PRECEDE: char = '⊀';
    /// \u{2281}: '⊁'
    pub const DOES_NOT_SUCCEED: char = '⊁';
    /// \u{2282}: '⊂'
    pub const SUBSET_OF: char = '⊂';
    /// \u{2283}: '⊃'
    pub const SUPERSET_OF: char = '⊃';
    /// \u{2284}: '⊄'
    pub const NOT_A_SUBSET_OF: char = '⊄';
    /// \u{2285}: '⊅'
    pub const NOT_A_SUPERSET_OF: char = '⊅';
    /// \u{2286}: '⊆'
    pub const SUBSET_OF_OR_EQUAL_TO: char = '⊆';
    /// \u{2287}: '⊇'
    pub const SUPERSET_OF_OR_EQUAL_TO: char = '⊇';
    /// \u{2288}: '⊈'
    pub const NEITHER_A_SUBSET_OF_NOR_EQUAL_TO: char = '⊈';
    /// \u{2289}: '⊉'
    pub const NEITHER_A_SUPERSET_OF_NOR_EQUAL_TO: char = '⊉';
    /// \u{228a}: '⊊'
    pub const SUBSET_OF_WITH_NOT_EQUAL_TO: char = '⊊';
    /// \u{228b}: '⊋'
    pub const SUPERSET_OF_WITH_NOT_EQUAL_TO: char = '⊋';
    /// \u{228c}: '⊌'
    pub const MULTISET: char = '⊌';
    /// \u{228d}: '⊍'
    pub const MULTISET_MULTIPLICATION: char = '⊍';
    /// \u{228e}: '⊎'
    pub const MULTISET_UNION: char = '⊎';
    /// \u{228f}: '⊏'
    pub const SQUARE_IMAGE_OF: char = '⊏';
    /// \u{2290}: '⊐'
    pub const SQUARE_ORIGINAL_OF: char = '⊐';
    /// \u{2291}: '⊑'
    pub const SQUARE_IMAGE_OF_OR_EQUAL_TO: char = '⊑';
    /// \u{2292}: '⊒'
    pub const SQUARE_ORIGINAL_OF_OR_EQUAL_TO: char = '⊒';
    /// \u{2293}: '⊓'
    pub const SQUARE_CAP: char = '⊓';
    /// \u{2294}: '⊔'
    pub const SQUARE_CUP: char = '⊔';
    /// \u{2295}: '⊕'
    pub const CIRCLED_PLUS: char = '⊕';
    /// \u{2296}: '⊖'
    pub const CIRCLED_MINUS: char = '⊖';
    /// \u{2297}: '⊗'
    pub const CIRCLED_TIMES: char = '⊗';
    /// \u{2298}: '⊘'
    pub const CIRCLED_DIVISION_SLASH: char = '⊘';
    /// \u{2299}: '⊙'
    pub const CIRCLED_DOT_OPERATOR: char = '⊙';
    /// \u{229a}: '⊚'
    pub const CIRCLED_RING_OPERATOR: char = '⊚';
    /// \u{229b}: '⊛'
    pub const CIRCLED_ASTERISK_OPERATOR: char = '⊛';
    /// \u{229c}: '⊜'
    pub const CIRCLED_EQUALS: char = '⊜';
    /// \u{229d}: '⊝'
    pub const CIRCLED_DASH: char = '⊝';
    /// \u{229e}: '⊞'
    pub const SQUARED_PLUS: char = '⊞';
    /// \u{229f}: '⊟'
    pub const SQUARED_MINUS: char = '⊟';
    /// \u{22a0}: '⊠'
    pub const SQUARED_TIMES: char = '⊠';
    /// \u{22a1}: '⊡'
    pub const SQUARED_DOT_OPERATOR: char = '⊡';
    /// \u{22a2}: '⊢'
    pub const RIGHT_TACK: char = '⊢';
    /// \u{22a3}: '⊣'
    pub const LEFT_TACK: char = '⊣';
    /// \u{22a4}: '⊤'
    pub const DOWN_TACK: char = '⊤';
    /// \u{22a5}: '⊥'
    pub const UP_TACK: char = '⊥';
    /// \u{22a6}: '⊦'
    pub const ASSERTION: char = '⊦';
    /// \u{22a7}: '⊧'
    pub const MODELS: char = '⊧';
    /// \u{22a8}: '⊨'
    pub const TRUE: char = '⊨';
    /// \u{22a9}: '⊩'
    pub const FORCES: char = '⊩';
    /// \u{22aa}: '⊪'
    pub const TRIPLE_VERTICAL_BAR_RIGHT_TURNSTILE: char = '⊪';
    /// \u{22ab}: '⊫'
    pub const DOUBLE_VERTICAL_BAR_DOUBLE_RIGHT_TURNSTILE: char = '⊫';
    /// \u{22ac}: '⊬'
    pub const DOES_NOT_PROVE: char = '⊬';
    /// \u{22ad}: '⊭'
    pub const NOT_TRUE: char = '⊭';
    /// \u{22ae}: '⊮'
    pub const DOES_NOT_FORCE: char = '⊮';
    /// \u{22af}: '⊯'
    pub const NEGATED_DOUBLE_VERTICAL_BAR_DOUBLE_RIGHT_TURNSTILE: char = '⊯';
    /// \u{22b0}: '⊰'
    pub const PRECEDES_UNDER_RELATION: char = '⊰';
    /// \u{22b1}: '⊱'
    pub const SUCCEEDS_UNDER_RELATION: char = '⊱';
    /// \u{22b2}: '⊲'
    pub const NORMAL_SUBGROUP_OF: char = '⊲';
    /// \u{22b3}: '⊳'
    pub const CONTAINS_AS_NORMAL_SUBGROUP: char = '⊳';
    /// \u{22b4}: '⊴'
    pub const NORMAL_SUBGROUP_OF_OR_EQUAL_TO: char = '⊴';
    /// \u{22b5}: '⊵'
    pub const CONTAINS_AS_NORMAL_SUBGROUP_OR_EQUAL_TO: char = '⊵';
    /// \u{22b6}: '⊶'
    pub const ORIGINAL_OF: char = '⊶';
    /// \u{22b7}: '⊷'
    pub const IMAGE_OF: char = '⊷';
    /// \u{22b8}: '⊸'
    pub const MULTIMAP: char = '⊸';
    /// \u{22b9}: '⊹'
    pub const HERMITIAN_CONJUGATE_MATRIX: char = '⊹';
    /// \u{22ba}: '⊺'
    pub const INTERCALATE: char = '⊺';
    /// \u{22bb}: '⊻'
    pub const XOR: char = '⊻';
    /// \u{22bc}: '⊼'
    pub const NAND: char = '⊼';
    /// \u{22bd}: '⊽'
    pub const NOR: char = '⊽';
    /// \u{22be}: '⊾'
    pub const RIGHT_ANGLE_WITH_ARC: char = '⊾';
    /// \u{22bf}: '⊿'
    pub const RIGHT_TRIANGLE: char = '⊿';
    /// \u{22c0}: '⋀'
    pub const N_DASH_ARY_LOGICAL_AND: char = '⋀';
    /// \u{22c1}: '⋁'
    pub const N_DASH_ARY_LOGICAL_OR: char = '⋁';
    /// \u{22c2}: '⋂'
    pub const N_DASH_ARY_INTERSECTION: char = '⋂';
    /// \u{22c3}: '⋃'
    pub const N_DASH_ARY_UNION: char = '⋃';
    /// \u{22c4}: '⋄'
    pub const DIAMOND_OPERATOR: char = '⋄';
    /// \u{22c5}: '⋅'
    pub const DOT_OPERATOR: char = '⋅';
    /// \u{22c6}: '⋆'
    pub const STAR_OPERATOR: char = '⋆';
    /// \u{22c7}: '⋇'
    pub const DIVISION_TIMES: char = '⋇';
    /// \u{22c8}: '⋈'
    pub const BOWTIE: char = '⋈';
    /// \u{22c9}: '⋉'
    pub const LEFT_NORMAL_FACTOR_SEMIDIRECT_PRODUCT: char = '⋉';
    /// \u{22ca}: '⋊'
    pub const RIGHT_NORMAL_FACTOR_SEMIDIRECT_PRODUCT: char = '⋊';
    /// \u{22cb}: '⋋'
    pub const LEFT_SEMIDIRECT_PRODUCT: char = '⋋';
    /// \u{22cc}: '⋌'
    pub const RIGHT_SEMIDIRECT_PRODUCT: char = '⋌';
    /// \u{22cd}: '⋍'
    pub const REVERSED_TILDE_EQUALS: char = '⋍';
    /// \u{22ce}: '⋎'
    pub const CURLY_LOGICAL_OR: char = '⋎';
    /// \u{22cf}: '⋏'
    pub const CURLY_LOGICAL_AND: char = '⋏';
    /// \u{22d0}: '⋐'
    pub const DOUBLE_SUBSET: char = '⋐';
    /// \u{22d1}: '⋑'
    pub const DOUBLE_SUPERSET: char = '⋑';
    /// \u{22d2}: '⋒'
    pub const DOUBLE_INTERSECTION: char = '⋒';
    /// \u{22d3}: '⋓'
    pub const DOUBLE_UNION: char = '⋓';
    /// \u{22d4}: '⋔'
    pub const PITCHFORK: char = '⋔';
    /// \u{22d5}: '⋕'
    pub const EQUAL_AND_PARALLEL_TO: char = '⋕';
    /// \u{22d6}: '⋖'
    pub const LESS_DASH_THAN_WITH_DOT: char = '⋖';
    /// \u{22d7}: '⋗'
    pub const GREATER_DASH_THAN_WITH_DOT: char = '⋗';
    /// \u{22d8}: '⋘'
    pub const VERY_MUCH_LESS_DASH_THAN: char = '⋘';
    /// \u{22d9}: '⋙'
    pub const VERY_MUCH_GREATER_DASH_THAN: char = '⋙';
    /// \u{22da}: '⋚'
    pub const LESS_DASH_THAN_EQUAL_TO_OR_GREATER_DASH_THAN: char = '⋚';
    /// \u{22db}: '⋛'
    pub const GREATER_DASH_THAN_EQUAL_TO_OR_LESS_DASH_THAN: char = '⋛';
    /// \u{22dc}: '⋜'
    pub const EQUAL_TO_OR_LESS_DASH_THAN: char = '⋜';
    /// \u{22dd}: '⋝'
    pub const EQUAL_TO_OR_GREATER_DASH_THAN: char = '⋝';
    /// \u{22de}: '⋞'
    pub const EQUAL_TO_OR_PRECEDES: char = '⋞';
    /// \u{22df}: '⋟'
    pub const EQUAL_TO_OR_SUCCEEDS: char = '⋟';
    /// \u{22e0}: '⋠'
    pub const DOES_NOT_PRECEDE_OR_EQUAL: char = '⋠';
    /// \u{22e1}: '⋡'
    pub const DOES_NOT_SUCCEED_OR_EQUAL: char = '⋡';
    /// \u{22e2}: '⋢'
    pub const NOT_SQUARE_IMAGE_OF_OR_EQUAL_TO: char = '⋢';
    /// \u{22e3}: '⋣'
    pub const NOT_SQUARE_ORIGINAL_OF_OR_EQUAL_TO: char = '⋣';
    /// \u{22e4}: '⋤'
    pub const SQUARE_IMAGE_OF_OR_NOT_EQUAL_TO: char = '⋤';
    /// \u{22e5}: '⋥'
    pub const SQUARE_ORIGINAL_OF_OR_NOT_EQUAL_TO: char = '⋥';
    /// \u{22e6}: '⋦'
    pub const LESS_DASH_THAN_BUT_NOT_EQUIVALENT_TO: char = '⋦';
    /// \u{22e7}: '⋧'
    pub const GREATER_DASH_THAN_BUT_NOT_EQUIVALENT_TO: char = '⋧';
    /// \u{22e8}: '⋨'
    pub const PRECEDES_BUT_NOT_EQUIVALENT_TO: char = '⋨';
    /// \u{22e9}: '⋩'
    pub const SUCCEEDS_BUT_NOT_EQUIVALENT_TO: char = '⋩';
    /// \u{22ea}: '⋪'
    pub const NOT_NORMAL_SUBGROUP_OF: char = '⋪';
    /// \u{22eb}: '⋫'
    pub const DOES_NOT_CONTAIN_AS_NORMAL_SUBGROUP: char = '⋫';
    /// \u{22ec}: '⋬'
    pub const NOT_NORMAL_SUBGROUP_OF_OR_EQUAL_TO: char = '⋬';
    /// \u{22ed}: '⋭'
    pub const DOES_NOT_CONTAIN_AS_NORMAL_SUBGROUP_OR_EQUAL: char = '⋭';
    /// \u{22ee}: '⋮'
    pub const VERTICAL_ELLIPSIS: char = '⋮';
    /// \u{22ef}: '⋯'
    pub const MIDLINE_HORIZONTAL_ELLIPSIS: char = '⋯';
    /// \u{22f0}: '⋰'
    pub const UP_RIGHT_DIAGONAL_ELLIPSIS: char = '⋰';
    /// \u{22f1}: '⋱'
    pub const DOWN_RIGHT_DIAGONAL_ELLIPSIS: char = '⋱';
    /// \u{22f2}: '⋲'
    pub const ELEMENT_OF_WITH_LONG_HORIZONTAL_STROKE: char = '⋲';
    /// \u{22f3}: '⋳'
    pub const ELEMENT_OF_WITH_VERTICAL_BAR_AT_END_OF_HORIZONTAL_STROKE: char = '⋳';
    /// \u{22f4}: '⋴'
    pub const SMALL_ELEMENT_OF_WITH_VERTICAL_BAR_AT_END_OF_HORIZONTAL_STROKE: char = '⋴';
    /// \u{22f5}: '⋵'
    pub const ELEMENT_OF_WITH_DOT_ABOVE: char = '⋵';
    /// \u{22f6}: '⋶'
    pub const ELEMENT_OF_WITH_OVERBAR: char = '⋶';
    /// \u{22f7}: '⋷'
    pub const SMALL_ELEMENT_OF_WITH_OVERBAR: char = '⋷';
    /// \u{22f8}: '⋸'
    pub const ELEMENT_OF_WITH_UNDERBAR: char = '⋸';
    /// \u{22f9}: '⋹'
    pub const ELEMENT_OF_WITH_TWO_HORIZONTAL_STROKES: char = '⋹';
    /// \u{22fa}: '⋺'
    pub const CONTAINS_WITH_LONG_HORIZONTAL_STROKE: char = '⋺';
    /// \u{22fb}: '⋻'
    pub const CONTAINS_WITH_VERTICAL_BAR_AT_END_OF_HORIZONTAL_STROKE: char = '⋻';
    /// \u{22fc}: '⋼'
    pub const SMALL_CONTAINS_WITH_VERTICAL_BAR_AT_END_OF_HORIZONTAL_STROKE: char = '⋼';
    /// \u{22fd}: '⋽'
    pub const CONTAINS_WITH_OVERBAR: char = '⋽';
    /// \u{22fe}: '⋾'
    pub const SMALL_CONTAINS_WITH_OVERBAR: char = '⋾';
}

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
        use constants::*;
        match self {
            MathematicalOperators::ForAll => FOR_ALL,
            MathematicalOperators::Complement => COMPLEMENT,
            MathematicalOperators::PartialDifferential => PARTIAL_DIFFERENTIAL,
            MathematicalOperators::ThereExists => THERE_EXISTS,
            MathematicalOperators::ThereDoesNotExist => THERE_DOES_NOT_EXIST,
            MathematicalOperators::EmptySet => EMPTY_SET,
            MathematicalOperators::Increment => INCREMENT,
            MathematicalOperators::Nabla => NABLA,
            MathematicalOperators::ElementOf => ELEMENT_OF,
            MathematicalOperators::NotAnElementOf => NOT_AN_ELEMENT_OF,
            MathematicalOperators::SmallElementOf => SMALL_ELEMENT_OF,
            MathematicalOperators::ContainsAsMember => CONTAINS_AS_MEMBER,
            MathematicalOperators::DoesNotContainAsMember => DOES_NOT_CONTAIN_AS_MEMBER,
            MathematicalOperators::SmallContainsAsMember => SMALL_CONTAINS_AS_MEMBER,
            MathematicalOperators::EndOfProof => END_OF_PROOF,
            MathematicalOperators::NDashAryProduct => N_DASH_ARY_PRODUCT,
            MathematicalOperators::NDashAryCoproduct => N_DASH_ARY_COPRODUCT,
            MathematicalOperators::NDashArySummation => N_DASH_ARY_SUMMATION,
            MathematicalOperators::MinusSign => MINUS_SIGN,
            MathematicalOperators::MinusDashOrDashPlusSign => MINUS_DASH_OR_DASH_PLUS_SIGN,
            MathematicalOperators::DotPlus => DOT_PLUS,
            MathematicalOperators::DivisionSlash => DIVISION_SLASH,
            MathematicalOperators::SetMinus => SET_MINUS,
            MathematicalOperators::AsteriskOperator => ASTERISK_OPERATOR,
            MathematicalOperators::RingOperator => RING_OPERATOR,
            MathematicalOperators::BulletOperator => BULLET_OPERATOR,
            MathematicalOperators::SquareRoot => SQUARE_ROOT,
            MathematicalOperators::CubeRoot => CUBE_ROOT,
            MathematicalOperators::FourthRoot => FOURTH_ROOT,
            MathematicalOperators::ProportionalTo => PROPORTIONAL_TO,
            MathematicalOperators::Infinity => INFINITY,
            MathematicalOperators::RightAngle => RIGHT_ANGLE,
            MathematicalOperators::Angle => ANGLE,
            MathematicalOperators::MeasuredAngle => MEASURED_ANGLE,
            MathematicalOperators::SphericalAngle => SPHERICAL_ANGLE,
            MathematicalOperators::Divides => DIVIDES,
            MathematicalOperators::DoesNotDivide => DOES_NOT_DIVIDE,
            MathematicalOperators::ParallelTo => PARALLEL_TO,
            MathematicalOperators::NotParallelTo => NOT_PARALLEL_TO,
            MathematicalOperators::LogicalAnd => LOGICAL_AND,
            MathematicalOperators::LogicalOr => LOGICAL_OR,
            MathematicalOperators::Intersection => INTERSECTION,
            MathematicalOperators::Union => UNION,
            MathematicalOperators::Integral => INTEGRAL,
            MathematicalOperators::DoubleIntegral => DOUBLE_INTEGRAL,
            MathematicalOperators::TripleIntegral => TRIPLE_INTEGRAL,
            MathematicalOperators::ContourIntegral => CONTOUR_INTEGRAL,
            MathematicalOperators::SurfaceIntegral => SURFACE_INTEGRAL,
            MathematicalOperators::VolumeIntegral => VOLUME_INTEGRAL,
            MathematicalOperators::ClockwiseIntegral => CLOCKWISE_INTEGRAL,
            MathematicalOperators::ClockwiseContourIntegral => CLOCKWISE_CONTOUR_INTEGRAL,
            MathematicalOperators::AnticlockwiseContourIntegral => ANTICLOCKWISE_CONTOUR_INTEGRAL,
            MathematicalOperators::Therefore => THEREFORE,
            MathematicalOperators::Because => BECAUSE,
            MathematicalOperators::Ratio => RATIO,
            MathematicalOperators::Proportion => PROPORTION,
            MathematicalOperators::DotMinus => DOT_MINUS,
            MathematicalOperators::Excess => EXCESS,
            MathematicalOperators::GeometricProportion => GEOMETRIC_PROPORTION,
            MathematicalOperators::Homothetic => HOMOTHETIC,
            MathematicalOperators::TildeOperator => TILDE_OPERATOR,
            MathematicalOperators::ReversedTilde => REVERSED_TILDE,
            MathematicalOperators::InvertedLazyS => INVERTED_LAZY_S,
            MathematicalOperators::SineWave => SINE_WAVE,
            MathematicalOperators::WreathProduct => WREATH_PRODUCT,
            MathematicalOperators::NotTilde => NOT_TILDE,
            MathematicalOperators::MinusTilde => MINUS_TILDE,
            MathematicalOperators::AsymptoticallyEqualTo => ASYMPTOTICALLY_EQUAL_TO,
            MathematicalOperators::NotAsymptoticallyEqualTo => NOT_ASYMPTOTICALLY_EQUAL_TO,
            MathematicalOperators::ApproximatelyEqualTo => APPROXIMATELY_EQUAL_TO,
            MathematicalOperators::ApproximatelyButNotActuallyEqualTo => APPROXIMATELY_BUT_NOT_ACTUALLY_EQUAL_TO,
            MathematicalOperators::NeitherApproximatelyNorActuallyEqualTo => NEITHER_APPROXIMATELY_NOR_ACTUALLY_EQUAL_TO,
            MathematicalOperators::AlmostEqualTo => ALMOST_EQUAL_TO,
            MathematicalOperators::NotAlmostEqualTo => NOT_ALMOST_EQUAL_TO,
            MathematicalOperators::AlmostEqualOrEqualTo => ALMOST_EQUAL_OR_EQUAL_TO,
            MathematicalOperators::TripleTilde => TRIPLE_TILDE,
            MathematicalOperators::AllEqualTo => ALL_EQUAL_TO,
            MathematicalOperators::EquivalentTo => EQUIVALENT_TO,
            MathematicalOperators::GeometricallyEquivalentTo => GEOMETRICALLY_EQUIVALENT_TO,
            MathematicalOperators::DifferenceBetween => DIFFERENCE_BETWEEN,
            MathematicalOperators::ApproachesTheLimit => APPROACHES_THE_LIMIT,
            MathematicalOperators::GeometricallyEqualTo => GEOMETRICALLY_EQUAL_TO,
            MathematicalOperators::ApproximatelyEqualToOrTheImageOf => APPROXIMATELY_EQUAL_TO_OR_THE_IMAGE_OF,
            MathematicalOperators::ImageOfOrApproximatelyEqualTo => IMAGE_OF_OR_APPROXIMATELY_EQUAL_TO,
            MathematicalOperators::ColonEquals => COLON_EQUALS,
            MathematicalOperators::EqualsColon => EQUALS_COLON,
            MathematicalOperators::RingInEqualTo => RING_IN_EQUAL_TO,
            MathematicalOperators::RingEqualTo => RING_EQUAL_TO,
            MathematicalOperators::CorrespondsTo => CORRESPONDS_TO,
            MathematicalOperators::Estimates => ESTIMATES,
            MathematicalOperators::EquiangularTo => EQUIANGULAR_TO,
            MathematicalOperators::StarEquals => STAR_EQUALS,
            MathematicalOperators::DeltaEqualTo => DELTA_EQUAL_TO,
            MathematicalOperators::EqualToByDefinition => EQUAL_TO_BY_DEFINITION,
            MathematicalOperators::MeasuredBy => MEASURED_BY,
            MathematicalOperators::QuestionedEqualTo => QUESTIONED_EQUAL_TO,
            MathematicalOperators::NotEqualTo => NOT_EQUAL_TO,
            MathematicalOperators::IdenticalTo => IDENTICAL_TO,
            MathematicalOperators::NotIdenticalTo => NOT_IDENTICAL_TO,
            MathematicalOperators::StrictlyEquivalentTo => STRICTLY_EQUIVALENT_TO,
            MathematicalOperators::LessDashThanOrEqualTo => LESS_DASH_THAN_OR_EQUAL_TO,
            MathematicalOperators::GreaterDashThanOrEqualTo => GREATER_DASH_THAN_OR_EQUAL_TO,
            MathematicalOperators::LessDashThanOverEqualTo => LESS_DASH_THAN_OVER_EQUAL_TO,
            MathematicalOperators::GreaterDashThanOverEqualTo => GREATER_DASH_THAN_OVER_EQUAL_TO,
            MathematicalOperators::LessDashThanButNotEqualTo => LESS_DASH_THAN_BUT_NOT_EQUAL_TO,
            MathematicalOperators::GreaterDashThanButNotEqualTo => GREATER_DASH_THAN_BUT_NOT_EQUAL_TO,
            MathematicalOperators::MuchLessDashThan => MUCH_LESS_DASH_THAN,
            MathematicalOperators::MuchGreaterDashThan => MUCH_GREATER_DASH_THAN,
            MathematicalOperators::Between => BETWEEN,
            MathematicalOperators::NotEquivalentTo => NOT_EQUIVALENT_TO,
            MathematicalOperators::NotLessDashThan => NOT_LESS_DASH_THAN,
            MathematicalOperators::NotGreaterDashThan => NOT_GREATER_DASH_THAN,
            MathematicalOperators::NeitherLessDashThanNorEqualTo => NEITHER_LESS_DASH_THAN_NOR_EQUAL_TO,
            MathematicalOperators::NeitherGreaterDashThanNorEqualTo => NEITHER_GREATER_DASH_THAN_NOR_EQUAL_TO,
            MathematicalOperators::LessDashThanOrEquivalentTo => LESS_DASH_THAN_OR_EQUIVALENT_TO,
            MathematicalOperators::GreaterDashThanOrEquivalentTo => GREATER_DASH_THAN_OR_EQUIVALENT_TO,
            MathematicalOperators::NeitherLessDashThanNorEquivalentTo => NEITHER_LESS_DASH_THAN_NOR_EQUIVALENT_TO,
            MathematicalOperators::NeitherGreaterDashThanNorEquivalentTo => NEITHER_GREATER_DASH_THAN_NOR_EQUIVALENT_TO,
            MathematicalOperators::LessDashThanOrGreaterDashThan => LESS_DASH_THAN_OR_GREATER_DASH_THAN,
            MathematicalOperators::GreaterDashThanOrLessDashThan => GREATER_DASH_THAN_OR_LESS_DASH_THAN,
            MathematicalOperators::NeitherLessDashThanNorGreaterDashThan => NEITHER_LESS_DASH_THAN_NOR_GREATER_DASH_THAN,
            MathematicalOperators::NeitherGreaterDashThanNorLessDashThan => NEITHER_GREATER_DASH_THAN_NOR_LESS_DASH_THAN,
            MathematicalOperators::Precedes => PRECEDES,
            MathematicalOperators::Succeeds => SUCCEEDS,
            MathematicalOperators::PrecedesOrEqualTo => PRECEDES_OR_EQUAL_TO,
            MathematicalOperators::SucceedsOrEqualTo => SUCCEEDS_OR_EQUAL_TO,
            MathematicalOperators::PrecedesOrEquivalentTo => PRECEDES_OR_EQUIVALENT_TO,
            MathematicalOperators::SucceedsOrEquivalentTo => SUCCEEDS_OR_EQUIVALENT_TO,
            MathematicalOperators::DoesNotPrecede => DOES_NOT_PRECEDE,
            MathematicalOperators::DoesNotSucceed => DOES_NOT_SUCCEED,
            MathematicalOperators::SubsetOf => SUBSET_OF,
            MathematicalOperators::SupersetOf => SUPERSET_OF,
            MathematicalOperators::NotASubsetOf => NOT_A_SUBSET_OF,
            MathematicalOperators::NotASupersetOf => NOT_A_SUPERSET_OF,
            MathematicalOperators::SubsetOfOrEqualTo => SUBSET_OF_OR_EQUAL_TO,
            MathematicalOperators::SupersetOfOrEqualTo => SUPERSET_OF_OR_EQUAL_TO,
            MathematicalOperators::NeitherASubsetOfNorEqualTo => NEITHER_A_SUBSET_OF_NOR_EQUAL_TO,
            MathematicalOperators::NeitherASupersetOfNorEqualTo => NEITHER_A_SUPERSET_OF_NOR_EQUAL_TO,
            MathematicalOperators::SubsetOfWithNotEqualTo => SUBSET_OF_WITH_NOT_EQUAL_TO,
            MathematicalOperators::SupersetOfWithNotEqualTo => SUPERSET_OF_WITH_NOT_EQUAL_TO,
            MathematicalOperators::Multiset => MULTISET,
            MathematicalOperators::MultisetMultiplication => MULTISET_MULTIPLICATION,
            MathematicalOperators::MultisetUnion => MULTISET_UNION,
            MathematicalOperators::SquareImageOf => SQUARE_IMAGE_OF,
            MathematicalOperators::SquareOriginalOf => SQUARE_ORIGINAL_OF,
            MathematicalOperators::SquareImageOfOrEqualTo => SQUARE_IMAGE_OF_OR_EQUAL_TO,
            MathematicalOperators::SquareOriginalOfOrEqualTo => SQUARE_ORIGINAL_OF_OR_EQUAL_TO,
            MathematicalOperators::SquareCap => SQUARE_CAP,
            MathematicalOperators::SquareCup => SQUARE_CUP,
            MathematicalOperators::CircledPlus => CIRCLED_PLUS,
            MathematicalOperators::CircledMinus => CIRCLED_MINUS,
            MathematicalOperators::CircledTimes => CIRCLED_TIMES,
            MathematicalOperators::CircledDivisionSlash => CIRCLED_DIVISION_SLASH,
            MathematicalOperators::CircledDotOperator => CIRCLED_DOT_OPERATOR,
            MathematicalOperators::CircledRingOperator => CIRCLED_RING_OPERATOR,
            MathematicalOperators::CircledAsteriskOperator => CIRCLED_ASTERISK_OPERATOR,
            MathematicalOperators::CircledEquals => CIRCLED_EQUALS,
            MathematicalOperators::CircledDash => CIRCLED_DASH,
            MathematicalOperators::SquaredPlus => SQUARED_PLUS,
            MathematicalOperators::SquaredMinus => SQUARED_MINUS,
            MathematicalOperators::SquaredTimes => SQUARED_TIMES,
            MathematicalOperators::SquaredDotOperator => SQUARED_DOT_OPERATOR,
            MathematicalOperators::RightTack => RIGHT_TACK,
            MathematicalOperators::LeftTack => LEFT_TACK,
            MathematicalOperators::DownTack => DOWN_TACK,
            MathematicalOperators::UpTack => UP_TACK,
            MathematicalOperators::Assertion => ASSERTION,
            MathematicalOperators::Models => MODELS,
            MathematicalOperators::True => TRUE,
            MathematicalOperators::Forces => FORCES,
            MathematicalOperators::TripleVerticalBarRightTurnstile => TRIPLE_VERTICAL_BAR_RIGHT_TURNSTILE,
            MathematicalOperators::DoubleVerticalBarDoubleRightTurnstile => DOUBLE_VERTICAL_BAR_DOUBLE_RIGHT_TURNSTILE,
            MathematicalOperators::DoesNotProve => DOES_NOT_PROVE,
            MathematicalOperators::NotTrue => NOT_TRUE,
            MathematicalOperators::DoesNotForce => DOES_NOT_FORCE,
            MathematicalOperators::NegatedDoubleVerticalBarDoubleRightTurnstile => NEGATED_DOUBLE_VERTICAL_BAR_DOUBLE_RIGHT_TURNSTILE,
            MathematicalOperators::PrecedesUnderRelation => PRECEDES_UNDER_RELATION,
            MathematicalOperators::SucceedsUnderRelation => SUCCEEDS_UNDER_RELATION,
            MathematicalOperators::NormalSubgroupOf => NORMAL_SUBGROUP_OF,
            MathematicalOperators::ContainsAsNormalSubgroup => CONTAINS_AS_NORMAL_SUBGROUP,
            MathematicalOperators::NormalSubgroupOfOrEqualTo => NORMAL_SUBGROUP_OF_OR_EQUAL_TO,
            MathematicalOperators::ContainsAsNormalSubgroupOrEqualTo => CONTAINS_AS_NORMAL_SUBGROUP_OR_EQUAL_TO,
            MathematicalOperators::OriginalOf => ORIGINAL_OF,
            MathematicalOperators::ImageOf => IMAGE_OF,
            MathematicalOperators::Multimap => MULTIMAP,
            MathematicalOperators::HermitianConjugateMatrix => HERMITIAN_CONJUGATE_MATRIX,
            MathematicalOperators::Intercalate => INTERCALATE,
            MathematicalOperators::Xor => XOR,
            MathematicalOperators::Nand => NAND,
            MathematicalOperators::Nor => NOR,
            MathematicalOperators::RightAngleWithArc => RIGHT_ANGLE_WITH_ARC,
            MathematicalOperators::RightTriangle => RIGHT_TRIANGLE,
            MathematicalOperators::NDashAryLogicalAnd => N_DASH_ARY_LOGICAL_AND,
            MathematicalOperators::NDashAryLogicalOr => N_DASH_ARY_LOGICAL_OR,
            MathematicalOperators::NDashAryIntersection => N_DASH_ARY_INTERSECTION,
            MathematicalOperators::NDashAryUnion => N_DASH_ARY_UNION,
            MathematicalOperators::DiamondOperator => DIAMOND_OPERATOR,
            MathematicalOperators::DotOperator => DOT_OPERATOR,
            MathematicalOperators::StarOperator => STAR_OPERATOR,
            MathematicalOperators::DivisionTimes => DIVISION_TIMES,
            MathematicalOperators::Bowtie => BOWTIE,
            MathematicalOperators::LeftNormalFactorSemidirectProduct => LEFT_NORMAL_FACTOR_SEMIDIRECT_PRODUCT,
            MathematicalOperators::RightNormalFactorSemidirectProduct => RIGHT_NORMAL_FACTOR_SEMIDIRECT_PRODUCT,
            MathematicalOperators::LeftSemidirectProduct => LEFT_SEMIDIRECT_PRODUCT,
            MathematicalOperators::RightSemidirectProduct => RIGHT_SEMIDIRECT_PRODUCT,
            MathematicalOperators::ReversedTildeEquals => REVERSED_TILDE_EQUALS,
            MathematicalOperators::CurlyLogicalOr => CURLY_LOGICAL_OR,
            MathematicalOperators::CurlyLogicalAnd => CURLY_LOGICAL_AND,
            MathematicalOperators::DoubleSubset => DOUBLE_SUBSET,
            MathematicalOperators::DoubleSuperset => DOUBLE_SUPERSET,
            MathematicalOperators::DoubleIntersection => DOUBLE_INTERSECTION,
            MathematicalOperators::DoubleUnion => DOUBLE_UNION,
            MathematicalOperators::Pitchfork => PITCHFORK,
            MathematicalOperators::EqualAndParallelTo => EQUAL_AND_PARALLEL_TO,
            MathematicalOperators::LessDashThanWithDot => LESS_DASH_THAN_WITH_DOT,
            MathematicalOperators::GreaterDashThanWithDot => GREATER_DASH_THAN_WITH_DOT,
            MathematicalOperators::VeryMuchLessDashThan => VERY_MUCH_LESS_DASH_THAN,
            MathematicalOperators::VeryMuchGreaterDashThan => VERY_MUCH_GREATER_DASH_THAN,
            MathematicalOperators::LessDashThanEqualToOrGreaterDashThan => LESS_DASH_THAN_EQUAL_TO_OR_GREATER_DASH_THAN,
            MathematicalOperators::GreaterDashThanEqualToOrLessDashThan => GREATER_DASH_THAN_EQUAL_TO_OR_LESS_DASH_THAN,
            MathematicalOperators::EqualToOrLessDashThan => EQUAL_TO_OR_LESS_DASH_THAN,
            MathematicalOperators::EqualToOrGreaterDashThan => EQUAL_TO_OR_GREATER_DASH_THAN,
            MathematicalOperators::EqualToOrPrecedes => EQUAL_TO_OR_PRECEDES,
            MathematicalOperators::EqualToOrSucceeds => EQUAL_TO_OR_SUCCEEDS,
            MathematicalOperators::DoesNotPrecedeOrEqual => DOES_NOT_PRECEDE_OR_EQUAL,
            MathematicalOperators::DoesNotSucceedOrEqual => DOES_NOT_SUCCEED_OR_EQUAL,
            MathematicalOperators::NotSquareImageOfOrEqualTo => NOT_SQUARE_IMAGE_OF_OR_EQUAL_TO,
            MathematicalOperators::NotSquareOriginalOfOrEqualTo => NOT_SQUARE_ORIGINAL_OF_OR_EQUAL_TO,
            MathematicalOperators::SquareImageOfOrNotEqualTo => SQUARE_IMAGE_OF_OR_NOT_EQUAL_TO,
            MathematicalOperators::SquareOriginalOfOrNotEqualTo => SQUARE_ORIGINAL_OF_OR_NOT_EQUAL_TO,
            MathematicalOperators::LessDashThanButNotEquivalentTo => LESS_DASH_THAN_BUT_NOT_EQUIVALENT_TO,
            MathematicalOperators::GreaterDashThanButNotEquivalentTo => GREATER_DASH_THAN_BUT_NOT_EQUIVALENT_TO,
            MathematicalOperators::PrecedesButNotEquivalentTo => PRECEDES_BUT_NOT_EQUIVALENT_TO,
            MathematicalOperators::SucceedsButNotEquivalentTo => SUCCEEDS_BUT_NOT_EQUIVALENT_TO,
            MathematicalOperators::NotNormalSubgroupOf => NOT_NORMAL_SUBGROUP_OF,
            MathematicalOperators::DoesNotContainAsNormalSubgroup => DOES_NOT_CONTAIN_AS_NORMAL_SUBGROUP,
            MathematicalOperators::NotNormalSubgroupOfOrEqualTo => NOT_NORMAL_SUBGROUP_OF_OR_EQUAL_TO,
            MathematicalOperators::DoesNotContainAsNormalSubgroupOrEqual => DOES_NOT_CONTAIN_AS_NORMAL_SUBGROUP_OR_EQUAL,
            MathematicalOperators::VerticalEllipsis => VERTICAL_ELLIPSIS,
            MathematicalOperators::MidlineHorizontalEllipsis => MIDLINE_HORIZONTAL_ELLIPSIS,
            MathematicalOperators::UpRightDiagonalEllipsis => UP_RIGHT_DIAGONAL_ELLIPSIS,
            MathematicalOperators::DownRightDiagonalEllipsis => DOWN_RIGHT_DIAGONAL_ELLIPSIS,
            MathematicalOperators::ElementOfWithLongHorizontalStroke => ELEMENT_OF_WITH_LONG_HORIZONTAL_STROKE,
            MathematicalOperators::ElementOfWithVerticalBarAtEndOfHorizontalStroke => ELEMENT_OF_WITH_VERTICAL_BAR_AT_END_OF_HORIZONTAL_STROKE,
            MathematicalOperators::SmallElementOfWithVerticalBarAtEndOfHorizontalStroke => SMALL_ELEMENT_OF_WITH_VERTICAL_BAR_AT_END_OF_HORIZONTAL_STROKE,
            MathematicalOperators::ElementOfWithDotAbove => ELEMENT_OF_WITH_DOT_ABOVE,
            MathematicalOperators::ElementOfWithOverbar => ELEMENT_OF_WITH_OVERBAR,
            MathematicalOperators::SmallElementOfWithOverbar => SMALL_ELEMENT_OF_WITH_OVERBAR,
            MathematicalOperators::ElementOfWithUnderbar => ELEMENT_OF_WITH_UNDERBAR,
            MathematicalOperators::ElementOfWithTwoHorizontalStrokes => ELEMENT_OF_WITH_TWO_HORIZONTAL_STROKES,
            MathematicalOperators::ContainsWithLongHorizontalStroke => CONTAINS_WITH_LONG_HORIZONTAL_STROKE,
            MathematicalOperators::ContainsWithVerticalBarAtEndOfHorizontalStroke => CONTAINS_WITH_VERTICAL_BAR_AT_END_OF_HORIZONTAL_STROKE,
            MathematicalOperators::SmallContainsWithVerticalBarAtEndOfHorizontalStroke => SMALL_CONTAINS_WITH_VERTICAL_BAR_AT_END_OF_HORIZONTAL_STROKE,
            MathematicalOperators::ContainsWithOverbar => CONTAINS_WITH_OVERBAR,
            MathematicalOperators::SmallContainsWithOverbar => SMALL_CONTAINS_WITH_OVERBAR,
        }
    }
}

impl std::convert::TryFrom<char> for MathematicalOperators {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            FOR_ALL => Ok(MathematicalOperators::ForAll),
            COMPLEMENT => Ok(MathematicalOperators::Complement),
            PARTIAL_DIFFERENTIAL => Ok(MathematicalOperators::PartialDifferential),
            THERE_EXISTS => Ok(MathematicalOperators::ThereExists),
            THERE_DOES_NOT_EXIST => Ok(MathematicalOperators::ThereDoesNotExist),
            EMPTY_SET => Ok(MathematicalOperators::EmptySet),
            INCREMENT => Ok(MathematicalOperators::Increment),
            NABLA => Ok(MathematicalOperators::Nabla),
            ELEMENT_OF => Ok(MathematicalOperators::ElementOf),
            NOT_AN_ELEMENT_OF => Ok(MathematicalOperators::NotAnElementOf),
            SMALL_ELEMENT_OF => Ok(MathematicalOperators::SmallElementOf),
            CONTAINS_AS_MEMBER => Ok(MathematicalOperators::ContainsAsMember),
            DOES_NOT_CONTAIN_AS_MEMBER => Ok(MathematicalOperators::DoesNotContainAsMember),
            SMALL_CONTAINS_AS_MEMBER => Ok(MathematicalOperators::SmallContainsAsMember),
            END_OF_PROOF => Ok(MathematicalOperators::EndOfProof),
            N_DASH_ARY_PRODUCT => Ok(MathematicalOperators::NDashAryProduct),
            N_DASH_ARY_COPRODUCT => Ok(MathematicalOperators::NDashAryCoproduct),
            N_DASH_ARY_SUMMATION => Ok(MathematicalOperators::NDashArySummation),
            MINUS_SIGN => Ok(MathematicalOperators::MinusSign),
            MINUS_DASH_OR_DASH_PLUS_SIGN => Ok(MathematicalOperators::MinusDashOrDashPlusSign),
            DOT_PLUS => Ok(MathematicalOperators::DotPlus),
            DIVISION_SLASH => Ok(MathematicalOperators::DivisionSlash),
            SET_MINUS => Ok(MathematicalOperators::SetMinus),
            ASTERISK_OPERATOR => Ok(MathematicalOperators::AsteriskOperator),
            RING_OPERATOR => Ok(MathematicalOperators::RingOperator),
            BULLET_OPERATOR => Ok(MathematicalOperators::BulletOperator),
            SQUARE_ROOT => Ok(MathematicalOperators::SquareRoot),
            CUBE_ROOT => Ok(MathematicalOperators::CubeRoot),
            FOURTH_ROOT => Ok(MathematicalOperators::FourthRoot),
            PROPORTIONAL_TO => Ok(MathematicalOperators::ProportionalTo),
            INFINITY => Ok(MathematicalOperators::Infinity),
            RIGHT_ANGLE => Ok(MathematicalOperators::RightAngle),
            ANGLE => Ok(MathematicalOperators::Angle),
            MEASURED_ANGLE => Ok(MathematicalOperators::MeasuredAngle),
            SPHERICAL_ANGLE => Ok(MathematicalOperators::SphericalAngle),
            DIVIDES => Ok(MathematicalOperators::Divides),
            DOES_NOT_DIVIDE => Ok(MathematicalOperators::DoesNotDivide),
            PARALLEL_TO => Ok(MathematicalOperators::ParallelTo),
            NOT_PARALLEL_TO => Ok(MathematicalOperators::NotParallelTo),
            LOGICAL_AND => Ok(MathematicalOperators::LogicalAnd),
            LOGICAL_OR => Ok(MathematicalOperators::LogicalOr),
            INTERSECTION => Ok(MathematicalOperators::Intersection),
            UNION => Ok(MathematicalOperators::Union),
            INTEGRAL => Ok(MathematicalOperators::Integral),
            DOUBLE_INTEGRAL => Ok(MathematicalOperators::DoubleIntegral),
            TRIPLE_INTEGRAL => Ok(MathematicalOperators::TripleIntegral),
            CONTOUR_INTEGRAL => Ok(MathematicalOperators::ContourIntegral),
            SURFACE_INTEGRAL => Ok(MathematicalOperators::SurfaceIntegral),
            VOLUME_INTEGRAL => Ok(MathematicalOperators::VolumeIntegral),
            CLOCKWISE_INTEGRAL => Ok(MathematicalOperators::ClockwiseIntegral),
            CLOCKWISE_CONTOUR_INTEGRAL => Ok(MathematicalOperators::ClockwiseContourIntegral),
            ANTICLOCKWISE_CONTOUR_INTEGRAL => Ok(MathematicalOperators::AnticlockwiseContourIntegral),
            THEREFORE => Ok(MathematicalOperators::Therefore),
            BECAUSE => Ok(MathematicalOperators::Because),
            RATIO => Ok(MathematicalOperators::Ratio),
            PROPORTION => Ok(MathematicalOperators::Proportion),
            DOT_MINUS => Ok(MathematicalOperators::DotMinus),
            EXCESS => Ok(MathematicalOperators::Excess),
            GEOMETRIC_PROPORTION => Ok(MathematicalOperators::GeometricProportion),
            HOMOTHETIC => Ok(MathematicalOperators::Homothetic),
            TILDE_OPERATOR => Ok(MathematicalOperators::TildeOperator),
            REVERSED_TILDE => Ok(MathematicalOperators::ReversedTilde),
            INVERTED_LAZY_S => Ok(MathematicalOperators::InvertedLazyS),
            SINE_WAVE => Ok(MathematicalOperators::SineWave),
            WREATH_PRODUCT => Ok(MathematicalOperators::WreathProduct),
            NOT_TILDE => Ok(MathematicalOperators::NotTilde),
            MINUS_TILDE => Ok(MathematicalOperators::MinusTilde),
            ASYMPTOTICALLY_EQUAL_TO => Ok(MathematicalOperators::AsymptoticallyEqualTo),
            NOT_ASYMPTOTICALLY_EQUAL_TO => Ok(MathematicalOperators::NotAsymptoticallyEqualTo),
            APPROXIMATELY_EQUAL_TO => Ok(MathematicalOperators::ApproximatelyEqualTo),
            APPROXIMATELY_BUT_NOT_ACTUALLY_EQUAL_TO => Ok(MathematicalOperators::ApproximatelyButNotActuallyEqualTo),
            NEITHER_APPROXIMATELY_NOR_ACTUALLY_EQUAL_TO => Ok(MathematicalOperators::NeitherApproximatelyNorActuallyEqualTo),
            ALMOST_EQUAL_TO => Ok(MathematicalOperators::AlmostEqualTo),
            NOT_ALMOST_EQUAL_TO => Ok(MathematicalOperators::NotAlmostEqualTo),
            ALMOST_EQUAL_OR_EQUAL_TO => Ok(MathematicalOperators::AlmostEqualOrEqualTo),
            TRIPLE_TILDE => Ok(MathematicalOperators::TripleTilde),
            ALL_EQUAL_TO => Ok(MathematicalOperators::AllEqualTo),
            EQUIVALENT_TO => Ok(MathematicalOperators::EquivalentTo),
            GEOMETRICALLY_EQUIVALENT_TO => Ok(MathematicalOperators::GeometricallyEquivalentTo),
            DIFFERENCE_BETWEEN => Ok(MathematicalOperators::DifferenceBetween),
            APPROACHES_THE_LIMIT => Ok(MathematicalOperators::ApproachesTheLimit),
            GEOMETRICALLY_EQUAL_TO => Ok(MathematicalOperators::GeometricallyEqualTo),
            APPROXIMATELY_EQUAL_TO_OR_THE_IMAGE_OF => Ok(MathematicalOperators::ApproximatelyEqualToOrTheImageOf),
            IMAGE_OF_OR_APPROXIMATELY_EQUAL_TO => Ok(MathematicalOperators::ImageOfOrApproximatelyEqualTo),
            COLON_EQUALS => Ok(MathematicalOperators::ColonEquals),
            EQUALS_COLON => Ok(MathematicalOperators::EqualsColon),
            RING_IN_EQUAL_TO => Ok(MathematicalOperators::RingInEqualTo),
            RING_EQUAL_TO => Ok(MathematicalOperators::RingEqualTo),
            CORRESPONDS_TO => Ok(MathematicalOperators::CorrespondsTo),
            ESTIMATES => Ok(MathematicalOperators::Estimates),
            EQUIANGULAR_TO => Ok(MathematicalOperators::EquiangularTo),
            STAR_EQUALS => Ok(MathematicalOperators::StarEquals),
            DELTA_EQUAL_TO => Ok(MathematicalOperators::DeltaEqualTo),
            EQUAL_TO_BY_DEFINITION => Ok(MathematicalOperators::EqualToByDefinition),
            MEASURED_BY => Ok(MathematicalOperators::MeasuredBy),
            QUESTIONED_EQUAL_TO => Ok(MathematicalOperators::QuestionedEqualTo),
            NOT_EQUAL_TO => Ok(MathematicalOperators::NotEqualTo),
            IDENTICAL_TO => Ok(MathematicalOperators::IdenticalTo),
            NOT_IDENTICAL_TO => Ok(MathematicalOperators::NotIdenticalTo),
            STRICTLY_EQUIVALENT_TO => Ok(MathematicalOperators::StrictlyEquivalentTo),
            LESS_DASH_THAN_OR_EQUAL_TO => Ok(MathematicalOperators::LessDashThanOrEqualTo),
            GREATER_DASH_THAN_OR_EQUAL_TO => Ok(MathematicalOperators::GreaterDashThanOrEqualTo),
            LESS_DASH_THAN_OVER_EQUAL_TO => Ok(MathematicalOperators::LessDashThanOverEqualTo),
            GREATER_DASH_THAN_OVER_EQUAL_TO => Ok(MathematicalOperators::GreaterDashThanOverEqualTo),
            LESS_DASH_THAN_BUT_NOT_EQUAL_TO => Ok(MathematicalOperators::LessDashThanButNotEqualTo),
            GREATER_DASH_THAN_BUT_NOT_EQUAL_TO => Ok(MathematicalOperators::GreaterDashThanButNotEqualTo),
            MUCH_LESS_DASH_THAN => Ok(MathematicalOperators::MuchLessDashThan),
            MUCH_GREATER_DASH_THAN => Ok(MathematicalOperators::MuchGreaterDashThan),
            BETWEEN => Ok(MathematicalOperators::Between),
            NOT_EQUIVALENT_TO => Ok(MathematicalOperators::NotEquivalentTo),
            NOT_LESS_DASH_THAN => Ok(MathematicalOperators::NotLessDashThan),
            NOT_GREATER_DASH_THAN => Ok(MathematicalOperators::NotGreaterDashThan),
            NEITHER_LESS_DASH_THAN_NOR_EQUAL_TO => Ok(MathematicalOperators::NeitherLessDashThanNorEqualTo),
            NEITHER_GREATER_DASH_THAN_NOR_EQUAL_TO => Ok(MathematicalOperators::NeitherGreaterDashThanNorEqualTo),
            LESS_DASH_THAN_OR_EQUIVALENT_TO => Ok(MathematicalOperators::LessDashThanOrEquivalentTo),
            GREATER_DASH_THAN_OR_EQUIVALENT_TO => Ok(MathematicalOperators::GreaterDashThanOrEquivalentTo),
            NEITHER_LESS_DASH_THAN_NOR_EQUIVALENT_TO => Ok(MathematicalOperators::NeitherLessDashThanNorEquivalentTo),
            NEITHER_GREATER_DASH_THAN_NOR_EQUIVALENT_TO => Ok(MathematicalOperators::NeitherGreaterDashThanNorEquivalentTo),
            LESS_DASH_THAN_OR_GREATER_DASH_THAN => Ok(MathematicalOperators::LessDashThanOrGreaterDashThan),
            GREATER_DASH_THAN_OR_LESS_DASH_THAN => Ok(MathematicalOperators::GreaterDashThanOrLessDashThan),
            NEITHER_LESS_DASH_THAN_NOR_GREATER_DASH_THAN => Ok(MathematicalOperators::NeitherLessDashThanNorGreaterDashThan),
            NEITHER_GREATER_DASH_THAN_NOR_LESS_DASH_THAN => Ok(MathematicalOperators::NeitherGreaterDashThanNorLessDashThan),
            PRECEDES => Ok(MathematicalOperators::Precedes),
            SUCCEEDS => Ok(MathematicalOperators::Succeeds),
            PRECEDES_OR_EQUAL_TO => Ok(MathematicalOperators::PrecedesOrEqualTo),
            SUCCEEDS_OR_EQUAL_TO => Ok(MathematicalOperators::SucceedsOrEqualTo),
            PRECEDES_OR_EQUIVALENT_TO => Ok(MathematicalOperators::PrecedesOrEquivalentTo),
            SUCCEEDS_OR_EQUIVALENT_TO => Ok(MathematicalOperators::SucceedsOrEquivalentTo),
            DOES_NOT_PRECEDE => Ok(MathematicalOperators::DoesNotPrecede),
            DOES_NOT_SUCCEED => Ok(MathematicalOperators::DoesNotSucceed),
            SUBSET_OF => Ok(MathematicalOperators::SubsetOf),
            SUPERSET_OF => Ok(MathematicalOperators::SupersetOf),
            NOT_A_SUBSET_OF => Ok(MathematicalOperators::NotASubsetOf),
            NOT_A_SUPERSET_OF => Ok(MathematicalOperators::NotASupersetOf),
            SUBSET_OF_OR_EQUAL_TO => Ok(MathematicalOperators::SubsetOfOrEqualTo),
            SUPERSET_OF_OR_EQUAL_TO => Ok(MathematicalOperators::SupersetOfOrEqualTo),
            NEITHER_A_SUBSET_OF_NOR_EQUAL_TO => Ok(MathematicalOperators::NeitherASubsetOfNorEqualTo),
            NEITHER_A_SUPERSET_OF_NOR_EQUAL_TO => Ok(MathematicalOperators::NeitherASupersetOfNorEqualTo),
            SUBSET_OF_WITH_NOT_EQUAL_TO => Ok(MathematicalOperators::SubsetOfWithNotEqualTo),
            SUPERSET_OF_WITH_NOT_EQUAL_TO => Ok(MathematicalOperators::SupersetOfWithNotEqualTo),
            MULTISET => Ok(MathematicalOperators::Multiset),
            MULTISET_MULTIPLICATION => Ok(MathematicalOperators::MultisetMultiplication),
            MULTISET_UNION => Ok(MathematicalOperators::MultisetUnion),
            SQUARE_IMAGE_OF => Ok(MathematicalOperators::SquareImageOf),
            SQUARE_ORIGINAL_OF => Ok(MathematicalOperators::SquareOriginalOf),
            SQUARE_IMAGE_OF_OR_EQUAL_TO => Ok(MathematicalOperators::SquareImageOfOrEqualTo),
            SQUARE_ORIGINAL_OF_OR_EQUAL_TO => Ok(MathematicalOperators::SquareOriginalOfOrEqualTo),
            SQUARE_CAP => Ok(MathematicalOperators::SquareCap),
            SQUARE_CUP => Ok(MathematicalOperators::SquareCup),
            CIRCLED_PLUS => Ok(MathematicalOperators::CircledPlus),
            CIRCLED_MINUS => Ok(MathematicalOperators::CircledMinus),
            CIRCLED_TIMES => Ok(MathematicalOperators::CircledTimes),
            CIRCLED_DIVISION_SLASH => Ok(MathematicalOperators::CircledDivisionSlash),
            CIRCLED_DOT_OPERATOR => Ok(MathematicalOperators::CircledDotOperator),
            CIRCLED_RING_OPERATOR => Ok(MathematicalOperators::CircledRingOperator),
            CIRCLED_ASTERISK_OPERATOR => Ok(MathematicalOperators::CircledAsteriskOperator),
            CIRCLED_EQUALS => Ok(MathematicalOperators::CircledEquals),
            CIRCLED_DASH => Ok(MathematicalOperators::CircledDash),
            SQUARED_PLUS => Ok(MathematicalOperators::SquaredPlus),
            SQUARED_MINUS => Ok(MathematicalOperators::SquaredMinus),
            SQUARED_TIMES => Ok(MathematicalOperators::SquaredTimes),
            SQUARED_DOT_OPERATOR => Ok(MathematicalOperators::SquaredDotOperator),
            RIGHT_TACK => Ok(MathematicalOperators::RightTack),
            LEFT_TACK => Ok(MathematicalOperators::LeftTack),
            DOWN_TACK => Ok(MathematicalOperators::DownTack),
            UP_TACK => Ok(MathematicalOperators::UpTack),
            ASSERTION => Ok(MathematicalOperators::Assertion),
            MODELS => Ok(MathematicalOperators::Models),
            TRUE => Ok(MathematicalOperators::True),
            FORCES => Ok(MathematicalOperators::Forces),
            TRIPLE_VERTICAL_BAR_RIGHT_TURNSTILE => Ok(MathematicalOperators::TripleVerticalBarRightTurnstile),
            DOUBLE_VERTICAL_BAR_DOUBLE_RIGHT_TURNSTILE => Ok(MathematicalOperators::DoubleVerticalBarDoubleRightTurnstile),
            DOES_NOT_PROVE => Ok(MathematicalOperators::DoesNotProve),
            NOT_TRUE => Ok(MathematicalOperators::NotTrue),
            DOES_NOT_FORCE => Ok(MathematicalOperators::DoesNotForce),
            NEGATED_DOUBLE_VERTICAL_BAR_DOUBLE_RIGHT_TURNSTILE => Ok(MathematicalOperators::NegatedDoubleVerticalBarDoubleRightTurnstile),
            PRECEDES_UNDER_RELATION => Ok(MathematicalOperators::PrecedesUnderRelation),
            SUCCEEDS_UNDER_RELATION => Ok(MathematicalOperators::SucceedsUnderRelation),
            NORMAL_SUBGROUP_OF => Ok(MathematicalOperators::NormalSubgroupOf),
            CONTAINS_AS_NORMAL_SUBGROUP => Ok(MathematicalOperators::ContainsAsNormalSubgroup),
            NORMAL_SUBGROUP_OF_OR_EQUAL_TO => Ok(MathematicalOperators::NormalSubgroupOfOrEqualTo),
            CONTAINS_AS_NORMAL_SUBGROUP_OR_EQUAL_TO => Ok(MathematicalOperators::ContainsAsNormalSubgroupOrEqualTo),
            ORIGINAL_OF => Ok(MathematicalOperators::OriginalOf),
            IMAGE_OF => Ok(MathematicalOperators::ImageOf),
            MULTIMAP => Ok(MathematicalOperators::Multimap),
            HERMITIAN_CONJUGATE_MATRIX => Ok(MathematicalOperators::HermitianConjugateMatrix),
            INTERCALATE => Ok(MathematicalOperators::Intercalate),
            XOR => Ok(MathematicalOperators::Xor),
            NAND => Ok(MathematicalOperators::Nand),
            NOR => Ok(MathematicalOperators::Nor),
            RIGHT_ANGLE_WITH_ARC => Ok(MathematicalOperators::RightAngleWithArc),
            RIGHT_TRIANGLE => Ok(MathematicalOperators::RightTriangle),
            N_DASH_ARY_LOGICAL_AND => Ok(MathematicalOperators::NDashAryLogicalAnd),
            N_DASH_ARY_LOGICAL_OR => Ok(MathematicalOperators::NDashAryLogicalOr),
            N_DASH_ARY_INTERSECTION => Ok(MathematicalOperators::NDashAryIntersection),
            N_DASH_ARY_UNION => Ok(MathematicalOperators::NDashAryUnion),
            DIAMOND_OPERATOR => Ok(MathematicalOperators::DiamondOperator),
            DOT_OPERATOR => Ok(MathematicalOperators::DotOperator),
            STAR_OPERATOR => Ok(MathematicalOperators::StarOperator),
            DIVISION_TIMES => Ok(MathematicalOperators::DivisionTimes),
            BOWTIE => Ok(MathematicalOperators::Bowtie),
            LEFT_NORMAL_FACTOR_SEMIDIRECT_PRODUCT => Ok(MathematicalOperators::LeftNormalFactorSemidirectProduct),
            RIGHT_NORMAL_FACTOR_SEMIDIRECT_PRODUCT => Ok(MathematicalOperators::RightNormalFactorSemidirectProduct),
            LEFT_SEMIDIRECT_PRODUCT => Ok(MathematicalOperators::LeftSemidirectProduct),
            RIGHT_SEMIDIRECT_PRODUCT => Ok(MathematicalOperators::RightSemidirectProduct),
            REVERSED_TILDE_EQUALS => Ok(MathematicalOperators::ReversedTildeEquals),
            CURLY_LOGICAL_OR => Ok(MathematicalOperators::CurlyLogicalOr),
            CURLY_LOGICAL_AND => Ok(MathematicalOperators::CurlyLogicalAnd),
            DOUBLE_SUBSET => Ok(MathematicalOperators::DoubleSubset),
            DOUBLE_SUPERSET => Ok(MathematicalOperators::DoubleSuperset),
            DOUBLE_INTERSECTION => Ok(MathematicalOperators::DoubleIntersection),
            DOUBLE_UNION => Ok(MathematicalOperators::DoubleUnion),
            PITCHFORK => Ok(MathematicalOperators::Pitchfork),
            EQUAL_AND_PARALLEL_TO => Ok(MathematicalOperators::EqualAndParallelTo),
            LESS_DASH_THAN_WITH_DOT => Ok(MathematicalOperators::LessDashThanWithDot),
            GREATER_DASH_THAN_WITH_DOT => Ok(MathematicalOperators::GreaterDashThanWithDot),
            VERY_MUCH_LESS_DASH_THAN => Ok(MathematicalOperators::VeryMuchLessDashThan),
            VERY_MUCH_GREATER_DASH_THAN => Ok(MathematicalOperators::VeryMuchGreaterDashThan),
            LESS_DASH_THAN_EQUAL_TO_OR_GREATER_DASH_THAN => Ok(MathematicalOperators::LessDashThanEqualToOrGreaterDashThan),
            GREATER_DASH_THAN_EQUAL_TO_OR_LESS_DASH_THAN => Ok(MathematicalOperators::GreaterDashThanEqualToOrLessDashThan),
            EQUAL_TO_OR_LESS_DASH_THAN => Ok(MathematicalOperators::EqualToOrLessDashThan),
            EQUAL_TO_OR_GREATER_DASH_THAN => Ok(MathematicalOperators::EqualToOrGreaterDashThan),
            EQUAL_TO_OR_PRECEDES => Ok(MathematicalOperators::EqualToOrPrecedes),
            EQUAL_TO_OR_SUCCEEDS => Ok(MathematicalOperators::EqualToOrSucceeds),
            DOES_NOT_PRECEDE_OR_EQUAL => Ok(MathematicalOperators::DoesNotPrecedeOrEqual),
            DOES_NOT_SUCCEED_OR_EQUAL => Ok(MathematicalOperators::DoesNotSucceedOrEqual),
            NOT_SQUARE_IMAGE_OF_OR_EQUAL_TO => Ok(MathematicalOperators::NotSquareImageOfOrEqualTo),
            NOT_SQUARE_ORIGINAL_OF_OR_EQUAL_TO => Ok(MathematicalOperators::NotSquareOriginalOfOrEqualTo),
            SQUARE_IMAGE_OF_OR_NOT_EQUAL_TO => Ok(MathematicalOperators::SquareImageOfOrNotEqualTo),
            SQUARE_ORIGINAL_OF_OR_NOT_EQUAL_TO => Ok(MathematicalOperators::SquareOriginalOfOrNotEqualTo),
            LESS_DASH_THAN_BUT_NOT_EQUIVALENT_TO => Ok(MathematicalOperators::LessDashThanButNotEquivalentTo),
            GREATER_DASH_THAN_BUT_NOT_EQUIVALENT_TO => Ok(MathematicalOperators::GreaterDashThanButNotEquivalentTo),
            PRECEDES_BUT_NOT_EQUIVALENT_TO => Ok(MathematicalOperators::PrecedesButNotEquivalentTo),
            SUCCEEDS_BUT_NOT_EQUIVALENT_TO => Ok(MathematicalOperators::SucceedsButNotEquivalentTo),
            NOT_NORMAL_SUBGROUP_OF => Ok(MathematicalOperators::NotNormalSubgroupOf),
            DOES_NOT_CONTAIN_AS_NORMAL_SUBGROUP => Ok(MathematicalOperators::DoesNotContainAsNormalSubgroup),
            NOT_NORMAL_SUBGROUP_OF_OR_EQUAL_TO => Ok(MathematicalOperators::NotNormalSubgroupOfOrEqualTo),
            DOES_NOT_CONTAIN_AS_NORMAL_SUBGROUP_OR_EQUAL => Ok(MathematicalOperators::DoesNotContainAsNormalSubgroupOrEqual),
            VERTICAL_ELLIPSIS => Ok(MathematicalOperators::VerticalEllipsis),
            MIDLINE_HORIZONTAL_ELLIPSIS => Ok(MathematicalOperators::MidlineHorizontalEllipsis),
            UP_RIGHT_DIAGONAL_ELLIPSIS => Ok(MathematicalOperators::UpRightDiagonalEllipsis),
            DOWN_RIGHT_DIAGONAL_ELLIPSIS => Ok(MathematicalOperators::DownRightDiagonalEllipsis),
            ELEMENT_OF_WITH_LONG_HORIZONTAL_STROKE => Ok(MathematicalOperators::ElementOfWithLongHorizontalStroke),
            ELEMENT_OF_WITH_VERTICAL_BAR_AT_END_OF_HORIZONTAL_STROKE => Ok(MathematicalOperators::ElementOfWithVerticalBarAtEndOfHorizontalStroke),
            SMALL_ELEMENT_OF_WITH_VERTICAL_BAR_AT_END_OF_HORIZONTAL_STROKE => Ok(MathematicalOperators::SmallElementOfWithVerticalBarAtEndOfHorizontalStroke),
            ELEMENT_OF_WITH_DOT_ABOVE => Ok(MathematicalOperators::ElementOfWithDotAbove),
            ELEMENT_OF_WITH_OVERBAR => Ok(MathematicalOperators::ElementOfWithOverbar),
            SMALL_ELEMENT_OF_WITH_OVERBAR => Ok(MathematicalOperators::SmallElementOfWithOverbar),
            ELEMENT_OF_WITH_UNDERBAR => Ok(MathematicalOperators::ElementOfWithUnderbar),
            ELEMENT_OF_WITH_TWO_HORIZONTAL_STROKES => Ok(MathematicalOperators::ElementOfWithTwoHorizontalStrokes),
            CONTAINS_WITH_LONG_HORIZONTAL_STROKE => Ok(MathematicalOperators::ContainsWithLongHorizontalStroke),
            CONTAINS_WITH_VERTICAL_BAR_AT_END_OF_HORIZONTAL_STROKE => Ok(MathematicalOperators::ContainsWithVerticalBarAtEndOfHorizontalStroke),
            SMALL_CONTAINS_WITH_VERTICAL_BAR_AT_END_OF_HORIZONTAL_STROKE => Ok(MathematicalOperators::SmallContainsWithVerticalBarAtEndOfHorizontalStroke),
            CONTAINS_WITH_OVERBAR => Ok(MathematicalOperators::ContainsWithOverbar),
            SMALL_CONTAINS_WITH_OVERBAR => Ok(MathematicalOperators::SmallContainsWithOverbar),
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
