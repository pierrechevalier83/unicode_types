
/// An enum to represent all characters in the MathematicalAlphanumericSymbols block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum MathematicalAlphanumericSymbols {
    /// \u{1d400}: '𝐀'
    MathematicalBoldCapitalA,
    /// \u{1d401}: '𝐁'
    MathematicalBoldCapitalB,
    /// \u{1d402}: '𝐂'
    MathematicalBoldCapitalC,
    /// \u{1d403}: '𝐃'
    MathematicalBoldCapitalD,
    /// \u{1d404}: '𝐄'
    MathematicalBoldCapitalE,
    /// \u{1d405}: '𝐅'
    MathematicalBoldCapitalF,
    /// \u{1d406}: '𝐆'
    MathematicalBoldCapitalG,
    /// \u{1d407}: '𝐇'
    MathematicalBoldCapitalH,
    /// \u{1d408}: '𝐈'
    MathematicalBoldCapitalI,
    /// \u{1d409}: '𝐉'
    MathematicalBoldCapitalJ,
    /// \u{1d40a}: '𝐊'
    MathematicalBoldCapitalK,
    /// \u{1d40b}: '𝐋'
    MathematicalBoldCapitalL,
    /// \u{1d40c}: '𝐌'
    MathematicalBoldCapitalM,
    /// \u{1d40d}: '𝐍'
    MathematicalBoldCapitalN,
    /// \u{1d40e}: '𝐎'
    MathematicalBoldCapitalO,
    /// \u{1d40f}: '𝐏'
    MathematicalBoldCapitalP,
    /// \u{1d410}: '𝐐'
    MathematicalBoldCapitalQ,
    /// \u{1d411}: '𝐑'
    MathematicalBoldCapitalR,
    /// \u{1d412}: '𝐒'
    MathematicalBoldCapitalS,
    /// \u{1d413}: '𝐓'
    MathematicalBoldCapitalT,
    /// \u{1d414}: '𝐔'
    MathematicalBoldCapitalU,
    /// \u{1d415}: '𝐕'
    MathematicalBoldCapitalV,
    /// \u{1d416}: '𝐖'
    MathematicalBoldCapitalW,
    /// \u{1d417}: '𝐗'
    MathematicalBoldCapitalX,
    /// \u{1d418}: '𝐘'
    MathematicalBoldCapitalY,
    /// \u{1d419}: '𝐙'
    MathematicalBoldCapitalZ,
    /// \u{1d41a}: '𝐚'
    MathematicalBoldSmallA,
    /// \u{1d41b}: '𝐛'
    MathematicalBoldSmallB,
    /// \u{1d41c}: '𝐜'
    MathematicalBoldSmallC,
    /// \u{1d41d}: '𝐝'
    MathematicalBoldSmallD,
    /// \u{1d41e}: '𝐞'
    MathematicalBoldSmallE,
    /// \u{1d41f}: '𝐟'
    MathematicalBoldSmallF,
    /// \u{1d420}: '𝐠'
    MathematicalBoldSmallG,
    /// \u{1d421}: '𝐡'
    MathematicalBoldSmallH,
    /// \u{1d422}: '𝐢'
    MathematicalBoldSmallI,
    /// \u{1d423}: '𝐣'
    MathematicalBoldSmallJ,
    /// \u{1d424}: '𝐤'
    MathematicalBoldSmallK,
    /// \u{1d425}: '𝐥'
    MathematicalBoldSmallL,
    /// \u{1d426}: '𝐦'
    MathematicalBoldSmallM,
    /// \u{1d427}: '𝐧'
    MathematicalBoldSmallN,
    /// \u{1d428}: '𝐨'
    MathematicalBoldSmallO,
    /// \u{1d429}: '𝐩'
    MathematicalBoldSmallP,
    /// \u{1d42a}: '𝐪'
    MathematicalBoldSmallQ,
    /// \u{1d42b}: '𝐫'
    MathematicalBoldSmallR,
    /// \u{1d42c}: '𝐬'
    MathematicalBoldSmallS,
    /// \u{1d42d}: '𝐭'
    MathematicalBoldSmallT,
    /// \u{1d42e}: '𝐮'
    MathematicalBoldSmallU,
    /// \u{1d42f}: '𝐯'
    MathematicalBoldSmallV,
    /// \u{1d430}: '𝐰'
    MathematicalBoldSmallW,
    /// \u{1d431}: '𝐱'
    MathematicalBoldSmallX,
    /// \u{1d432}: '𝐲'
    MathematicalBoldSmallY,
    /// \u{1d433}: '𝐳'
    MathematicalBoldSmallZ,
    /// \u{1d434}: '𝐴'
    MathematicalItalicCapitalA,
    /// \u{1d435}: '𝐵'
    MathematicalItalicCapitalB,
    /// \u{1d436}: '𝐶'
    MathematicalItalicCapitalC,
    /// \u{1d437}: '𝐷'
    MathematicalItalicCapitalD,
    /// \u{1d438}: '𝐸'
    MathematicalItalicCapitalE,
    /// \u{1d439}: '𝐹'
    MathematicalItalicCapitalF,
    /// \u{1d43a}: '𝐺'
    MathematicalItalicCapitalG,
    /// \u{1d43b}: '𝐻'
    MathematicalItalicCapitalH,
    /// \u{1d43c}: '𝐼'
    MathematicalItalicCapitalI,
    /// \u{1d43d}: '𝐽'
    MathematicalItalicCapitalJ,
    /// \u{1d43e}: '𝐾'
    MathematicalItalicCapitalK,
    /// \u{1d43f}: '𝐿'
    MathematicalItalicCapitalL,
    /// \u{1d440}: '𝑀'
    MathematicalItalicCapitalM,
    /// \u{1d441}: '𝑁'
    MathematicalItalicCapitalN,
    /// \u{1d442}: '𝑂'
    MathematicalItalicCapitalO,
    /// \u{1d443}: '𝑃'
    MathematicalItalicCapitalP,
    /// \u{1d444}: '𝑄'
    MathematicalItalicCapitalQ,
    /// \u{1d445}: '𝑅'
    MathematicalItalicCapitalR,
    /// \u{1d446}: '𝑆'
    MathematicalItalicCapitalS,
    /// \u{1d447}: '𝑇'
    MathematicalItalicCapitalT,
    /// \u{1d448}: '𝑈'
    MathematicalItalicCapitalU,
    /// \u{1d449}: '𝑉'
    MathematicalItalicCapitalV,
    /// \u{1d44a}: '𝑊'
    MathematicalItalicCapitalW,
    /// \u{1d44b}: '𝑋'
    MathematicalItalicCapitalX,
    /// \u{1d44c}: '𝑌'
    MathematicalItalicCapitalY,
    /// \u{1d44d}: '𝑍'
    MathematicalItalicCapitalZ,
    /// \u{1d44e}: '𝑎'
    MathematicalItalicSmallA,
    /// \u{1d44f}: '𝑏'
    MathematicalItalicSmallB,
    /// \u{1d450}: '𝑐'
    MathematicalItalicSmallC,
    /// \u{1d451}: '𝑑'
    MathematicalItalicSmallD,
    /// \u{1d452}: '𝑒'
    MathematicalItalicSmallE,
    /// \u{1d453}: '𝑓'
    MathematicalItalicSmallF,
    /// \u{1d454}: '𝑔'
    MathematicalItalicSmallG,
    /// \u{1d456}: '𝑖'
    MathematicalItalicSmallI,
    /// \u{1d457}: '𝑗'
    MathematicalItalicSmallJ,
    /// \u{1d458}: '𝑘'
    MathematicalItalicSmallK,
    /// \u{1d459}: '𝑙'
    MathematicalItalicSmallL,
    /// \u{1d45a}: '𝑚'
    MathematicalItalicSmallM,
    /// \u{1d45b}: '𝑛'
    MathematicalItalicSmallN,
    /// \u{1d45c}: '𝑜'
    MathematicalItalicSmallO,
    /// \u{1d45d}: '𝑝'
    MathematicalItalicSmallP,
    /// \u{1d45e}: '𝑞'
    MathematicalItalicSmallQ,
    /// \u{1d45f}: '𝑟'
    MathematicalItalicSmallR,
    /// \u{1d460}: '𝑠'
    MathematicalItalicSmallS,
    /// \u{1d461}: '𝑡'
    MathematicalItalicSmallT,
    /// \u{1d462}: '𝑢'
    MathematicalItalicSmallU,
    /// \u{1d463}: '𝑣'
    MathematicalItalicSmallV,
    /// \u{1d464}: '𝑤'
    MathematicalItalicSmallW,
    /// \u{1d465}: '𝑥'
    MathematicalItalicSmallX,
    /// \u{1d466}: '𝑦'
    MathematicalItalicSmallY,
    /// \u{1d467}: '𝑧'
    MathematicalItalicSmallZ,
    /// \u{1d468}: '𝑨'
    MathematicalBoldItalicCapitalA,
    /// \u{1d469}: '𝑩'
    MathematicalBoldItalicCapitalB,
    /// \u{1d46a}: '𝑪'
    MathematicalBoldItalicCapitalC,
    /// \u{1d46b}: '𝑫'
    MathematicalBoldItalicCapitalD,
    /// \u{1d46c}: '𝑬'
    MathematicalBoldItalicCapitalE,
    /// \u{1d46d}: '𝑭'
    MathematicalBoldItalicCapitalF,
    /// \u{1d46e}: '𝑮'
    MathematicalBoldItalicCapitalG,
    /// \u{1d46f}: '𝑯'
    MathematicalBoldItalicCapitalH,
    /// \u{1d470}: '𝑰'
    MathematicalBoldItalicCapitalI,
    /// \u{1d471}: '𝑱'
    MathematicalBoldItalicCapitalJ,
    /// \u{1d472}: '𝑲'
    MathematicalBoldItalicCapitalK,
    /// \u{1d473}: '𝑳'
    MathematicalBoldItalicCapitalL,
    /// \u{1d474}: '𝑴'
    MathematicalBoldItalicCapitalM,
    /// \u{1d475}: '𝑵'
    MathematicalBoldItalicCapitalN,
    /// \u{1d476}: '𝑶'
    MathematicalBoldItalicCapitalO,
    /// \u{1d477}: '𝑷'
    MathematicalBoldItalicCapitalP,
    /// \u{1d478}: '𝑸'
    MathematicalBoldItalicCapitalQ,
    /// \u{1d479}: '𝑹'
    MathematicalBoldItalicCapitalR,
    /// \u{1d47a}: '𝑺'
    MathematicalBoldItalicCapitalS,
    /// \u{1d47b}: '𝑻'
    MathematicalBoldItalicCapitalT,
    /// \u{1d47c}: '𝑼'
    MathematicalBoldItalicCapitalU,
    /// \u{1d47d}: '𝑽'
    MathematicalBoldItalicCapitalV,
    /// \u{1d47e}: '𝑾'
    MathematicalBoldItalicCapitalW,
    /// \u{1d47f}: '𝑿'
    MathematicalBoldItalicCapitalX,
    /// \u{1d480}: '𝒀'
    MathematicalBoldItalicCapitalY,
    /// \u{1d481}: '𝒁'
    MathematicalBoldItalicCapitalZ,
    /// \u{1d482}: '𝒂'
    MathematicalBoldItalicSmallA,
    /// \u{1d483}: '𝒃'
    MathematicalBoldItalicSmallB,
    /// \u{1d484}: '𝒄'
    MathematicalBoldItalicSmallC,
    /// \u{1d485}: '𝒅'
    MathematicalBoldItalicSmallD,
    /// \u{1d486}: '𝒆'
    MathematicalBoldItalicSmallE,
    /// \u{1d487}: '𝒇'
    MathematicalBoldItalicSmallF,
    /// \u{1d488}: '𝒈'
    MathematicalBoldItalicSmallG,
    /// \u{1d489}: '𝒉'
    MathematicalBoldItalicSmallH,
    /// \u{1d48a}: '𝒊'
    MathematicalBoldItalicSmallI,
    /// \u{1d48b}: '𝒋'
    MathematicalBoldItalicSmallJ,
    /// \u{1d48c}: '𝒌'
    MathematicalBoldItalicSmallK,
    /// \u{1d48d}: '𝒍'
    MathematicalBoldItalicSmallL,
    /// \u{1d48e}: '𝒎'
    MathematicalBoldItalicSmallM,
    /// \u{1d48f}: '𝒏'
    MathematicalBoldItalicSmallN,
    /// \u{1d490}: '𝒐'
    MathematicalBoldItalicSmallO,
    /// \u{1d491}: '𝒑'
    MathematicalBoldItalicSmallP,
    /// \u{1d492}: '𝒒'
    MathematicalBoldItalicSmallQ,
    /// \u{1d493}: '𝒓'
    MathematicalBoldItalicSmallR,
    /// \u{1d494}: '𝒔'
    MathematicalBoldItalicSmallS,
    /// \u{1d495}: '𝒕'
    MathematicalBoldItalicSmallT,
    /// \u{1d496}: '𝒖'
    MathematicalBoldItalicSmallU,
    /// \u{1d497}: '𝒗'
    MathematicalBoldItalicSmallV,
    /// \u{1d498}: '𝒘'
    MathematicalBoldItalicSmallW,
    /// \u{1d499}: '𝒙'
    MathematicalBoldItalicSmallX,
    /// \u{1d49a}: '𝒚'
    MathematicalBoldItalicSmallY,
    /// \u{1d49b}: '𝒛'
    MathematicalBoldItalicSmallZ,
    /// \u{1d49c}: '𝒜'
    MathematicalScriptCapitalA,
    /// \u{1d49e}: '𝒞'
    MathematicalScriptCapitalC,
    /// \u{1d49f}: '𝒟'
    MathematicalScriptCapitalD,
    /// \u{1d4a2}: '𝒢'
    MathematicalScriptCapitalG,
    /// \u{1d4a5}: '𝒥'
    MathematicalScriptCapitalJ,
    /// \u{1d4a6}: '𝒦'
    MathematicalScriptCapitalK,
    /// \u{1d4a9}: '𝒩'
    MathematicalScriptCapitalN,
    /// \u{1d4aa}: '𝒪'
    MathematicalScriptCapitalO,
    /// \u{1d4ab}: '𝒫'
    MathematicalScriptCapitalP,
    /// \u{1d4ac}: '𝒬'
    MathematicalScriptCapitalQ,
    /// \u{1d4ae}: '𝒮'
    MathematicalScriptCapitalS,
    /// \u{1d4af}: '𝒯'
    MathematicalScriptCapitalT,
    /// \u{1d4b0}: '𝒰'
    MathematicalScriptCapitalU,
    /// \u{1d4b1}: '𝒱'
    MathematicalScriptCapitalV,
    /// \u{1d4b2}: '𝒲'
    MathematicalScriptCapitalW,
    /// \u{1d4b3}: '𝒳'
    MathematicalScriptCapitalX,
    /// \u{1d4b4}: '𝒴'
    MathematicalScriptCapitalY,
    /// \u{1d4b5}: '𝒵'
    MathematicalScriptCapitalZ,
    /// \u{1d4b6}: '𝒶'
    MathematicalScriptSmallA,
    /// \u{1d4b7}: '𝒷'
    MathematicalScriptSmallB,
    /// \u{1d4b8}: '𝒸'
    MathematicalScriptSmallC,
    /// \u{1d4b9}: '𝒹'
    MathematicalScriptSmallD,
    /// \u{1d4bb}: '𝒻'
    MathematicalScriptSmallF,
    /// \u{1d4bd}: '𝒽'
    MathematicalScriptSmallH,
    /// \u{1d4be}: '𝒾'
    MathematicalScriptSmallI,
    /// \u{1d4bf}: '𝒿'
    MathematicalScriptSmallJ,
    /// \u{1d4c0}: '𝓀'
    MathematicalScriptSmallK,
    /// \u{1d4c1}: '𝓁'
    MathematicalScriptSmallL,
    /// \u{1d4c2}: '𝓂'
    MathematicalScriptSmallM,
    /// \u{1d4c3}: '𝓃'
    MathematicalScriptSmallN,
    /// \u{1d4c5}: '𝓅'
    MathematicalScriptSmallP,
    /// \u{1d4c6}: '𝓆'
    MathematicalScriptSmallQ,
    /// \u{1d4c7}: '𝓇'
    MathematicalScriptSmallR,
    /// \u{1d4c8}: '𝓈'
    MathematicalScriptSmallS,
    /// \u{1d4c9}: '𝓉'
    MathematicalScriptSmallT,
    /// \u{1d4ca}: '𝓊'
    MathematicalScriptSmallU,
    /// \u{1d4cb}: '𝓋'
    MathematicalScriptSmallV,
    /// \u{1d4cc}: '𝓌'
    MathematicalScriptSmallW,
    /// \u{1d4cd}: '𝓍'
    MathematicalScriptSmallX,
    /// \u{1d4ce}: '𝓎'
    MathematicalScriptSmallY,
    /// \u{1d4cf}: '𝓏'
    MathematicalScriptSmallZ,
    /// \u{1d4d0}: '𝓐'
    MathematicalBoldScriptCapitalA,
    /// \u{1d4d1}: '𝓑'
    MathematicalBoldScriptCapitalB,
    /// \u{1d4d2}: '𝓒'
    MathematicalBoldScriptCapitalC,
    /// \u{1d4d3}: '𝓓'
    MathematicalBoldScriptCapitalD,
    /// \u{1d4d4}: '𝓔'
    MathematicalBoldScriptCapitalE,
    /// \u{1d4d5}: '𝓕'
    MathematicalBoldScriptCapitalF,
    /// \u{1d4d6}: '𝓖'
    MathematicalBoldScriptCapitalG,
    /// \u{1d4d7}: '𝓗'
    MathematicalBoldScriptCapitalH,
    /// \u{1d4d8}: '𝓘'
    MathematicalBoldScriptCapitalI,
    /// \u{1d4d9}: '𝓙'
    MathematicalBoldScriptCapitalJ,
    /// \u{1d4da}: '𝓚'
    MathematicalBoldScriptCapitalK,
    /// \u{1d4db}: '𝓛'
    MathematicalBoldScriptCapitalL,
    /// \u{1d4dc}: '𝓜'
    MathematicalBoldScriptCapitalM,
    /// \u{1d4dd}: '𝓝'
    MathematicalBoldScriptCapitalN,
    /// \u{1d4de}: '𝓞'
    MathematicalBoldScriptCapitalO,
    /// \u{1d4df}: '𝓟'
    MathematicalBoldScriptCapitalP,
    /// \u{1d4e0}: '𝓠'
    MathematicalBoldScriptCapitalQ,
    /// \u{1d4e1}: '𝓡'
    MathematicalBoldScriptCapitalR,
    /// \u{1d4e2}: '𝓢'
    MathematicalBoldScriptCapitalS,
    /// \u{1d4e3}: '𝓣'
    MathematicalBoldScriptCapitalT,
    /// \u{1d4e4}: '𝓤'
    MathematicalBoldScriptCapitalU,
    /// \u{1d4e5}: '𝓥'
    MathematicalBoldScriptCapitalV,
    /// \u{1d4e6}: '𝓦'
    MathematicalBoldScriptCapitalW,
    /// \u{1d4e7}: '𝓧'
    MathematicalBoldScriptCapitalX,
    /// \u{1d4e8}: '𝓨'
    MathematicalBoldScriptCapitalY,
    /// \u{1d4e9}: '𝓩'
    MathematicalBoldScriptCapitalZ,
    /// \u{1d4ea}: '𝓪'
    MathematicalBoldScriptSmallA,
    /// \u{1d4eb}: '𝓫'
    MathematicalBoldScriptSmallB,
    /// \u{1d4ec}: '𝓬'
    MathematicalBoldScriptSmallC,
    /// \u{1d4ed}: '𝓭'
    MathematicalBoldScriptSmallD,
    /// \u{1d4ee}: '𝓮'
    MathematicalBoldScriptSmallE,
    /// \u{1d4ef}: '𝓯'
    MathematicalBoldScriptSmallF,
    /// \u{1d4f0}: '𝓰'
    MathematicalBoldScriptSmallG,
    /// \u{1d4f1}: '𝓱'
    MathematicalBoldScriptSmallH,
    /// \u{1d4f2}: '𝓲'
    MathematicalBoldScriptSmallI,
    /// \u{1d4f3}: '𝓳'
    MathematicalBoldScriptSmallJ,
    /// \u{1d4f4}: '𝓴'
    MathematicalBoldScriptSmallK,
    /// \u{1d4f5}: '𝓵'
    MathematicalBoldScriptSmallL,
    /// \u{1d4f6}: '𝓶'
    MathematicalBoldScriptSmallM,
    /// \u{1d4f7}: '𝓷'
    MathematicalBoldScriptSmallN,
    /// \u{1d4f8}: '𝓸'
    MathematicalBoldScriptSmallO,
    /// \u{1d4f9}: '𝓹'
    MathematicalBoldScriptSmallP,
    /// \u{1d4fa}: '𝓺'
    MathematicalBoldScriptSmallQ,
    /// \u{1d4fb}: '𝓻'
    MathematicalBoldScriptSmallR,
    /// \u{1d4fc}: '𝓼'
    MathematicalBoldScriptSmallS,
    /// \u{1d4fd}: '𝓽'
    MathematicalBoldScriptSmallT,
    /// \u{1d4fe}: '𝓾'
    MathematicalBoldScriptSmallU,
    /// \u{1d4ff}: '𝓿'
    MathematicalBoldScriptSmallV,
    /// \u{1d500}: '𝔀'
    MathematicalBoldScriptSmallW,
    /// \u{1d501}: '𝔁'
    MathematicalBoldScriptSmallX,
    /// \u{1d502}: '𝔂'
    MathematicalBoldScriptSmallY,
    /// \u{1d503}: '𝔃'
    MathematicalBoldScriptSmallZ,
    /// \u{1d504}: '𝔄'
    MathematicalFrakturCapitalA,
    /// \u{1d505}: '𝔅'
    MathematicalFrakturCapitalB,
    /// \u{1d507}: '𝔇'
    MathematicalFrakturCapitalD,
    /// \u{1d508}: '𝔈'
    MathematicalFrakturCapitalE,
    /// \u{1d509}: '𝔉'
    MathematicalFrakturCapitalF,
    /// \u{1d50a}: '𝔊'
    MathematicalFrakturCapitalG,
    /// \u{1d50d}: '𝔍'
    MathematicalFrakturCapitalJ,
    /// \u{1d50e}: '𝔎'
    MathematicalFrakturCapitalK,
    /// \u{1d50f}: '𝔏'
    MathematicalFrakturCapitalL,
    /// \u{1d510}: '𝔐'
    MathematicalFrakturCapitalM,
    /// \u{1d511}: '𝔑'
    MathematicalFrakturCapitalN,
    /// \u{1d512}: '𝔒'
    MathematicalFrakturCapitalO,
    /// \u{1d513}: '𝔓'
    MathematicalFrakturCapitalP,
    /// \u{1d514}: '𝔔'
    MathematicalFrakturCapitalQ,
    /// \u{1d516}: '𝔖'
    MathematicalFrakturCapitalS,
    /// \u{1d517}: '𝔗'
    MathematicalFrakturCapitalT,
    /// \u{1d518}: '𝔘'
    MathematicalFrakturCapitalU,
    /// \u{1d519}: '𝔙'
    MathematicalFrakturCapitalV,
    /// \u{1d51a}: '𝔚'
    MathematicalFrakturCapitalW,
    /// \u{1d51b}: '𝔛'
    MathematicalFrakturCapitalX,
    /// \u{1d51c}: '𝔜'
    MathematicalFrakturCapitalY,
    /// \u{1d51e}: '𝔞'
    MathematicalFrakturSmallA,
    /// \u{1d51f}: '𝔟'
    MathematicalFrakturSmallB,
    /// \u{1d520}: '𝔠'
    MathematicalFrakturSmallC,
    /// \u{1d521}: '𝔡'
    MathematicalFrakturSmallD,
    /// \u{1d522}: '𝔢'
    MathematicalFrakturSmallE,
    /// \u{1d523}: '𝔣'
    MathematicalFrakturSmallF,
    /// \u{1d524}: '𝔤'
    MathematicalFrakturSmallG,
    /// \u{1d525}: '𝔥'
    MathematicalFrakturSmallH,
    /// \u{1d526}: '𝔦'
    MathematicalFrakturSmallI,
    /// \u{1d527}: '𝔧'
    MathematicalFrakturSmallJ,
    /// \u{1d528}: '𝔨'
    MathematicalFrakturSmallK,
    /// \u{1d529}: '𝔩'
    MathematicalFrakturSmallL,
    /// \u{1d52a}: '𝔪'
    MathematicalFrakturSmallM,
    /// \u{1d52b}: '𝔫'
    MathematicalFrakturSmallN,
    /// \u{1d52c}: '𝔬'
    MathematicalFrakturSmallO,
    /// \u{1d52d}: '𝔭'
    MathematicalFrakturSmallP,
    /// \u{1d52e}: '𝔮'
    MathematicalFrakturSmallQ,
    /// \u{1d52f}: '𝔯'
    MathematicalFrakturSmallR,
    /// \u{1d530}: '𝔰'
    MathematicalFrakturSmallS,
    /// \u{1d531}: '𝔱'
    MathematicalFrakturSmallT,
    /// \u{1d532}: '𝔲'
    MathematicalFrakturSmallU,
    /// \u{1d533}: '𝔳'
    MathematicalFrakturSmallV,
    /// \u{1d534}: '𝔴'
    MathematicalFrakturSmallW,
    /// \u{1d535}: '𝔵'
    MathematicalFrakturSmallX,
    /// \u{1d536}: '𝔶'
    MathematicalFrakturSmallY,
    /// \u{1d537}: '𝔷'
    MathematicalFrakturSmallZ,
    /// \u{1d538}: '𝔸'
    MathematicalDoubleDashStruckCapitalA,
    /// \u{1d539}: '𝔹'
    MathematicalDoubleDashStruckCapitalB,
    /// \u{1d53b}: '𝔻'
    MathematicalDoubleDashStruckCapitalD,
    /// \u{1d53c}: '𝔼'
    MathematicalDoubleDashStruckCapitalE,
    /// \u{1d53d}: '𝔽'
    MathematicalDoubleDashStruckCapitalF,
    /// \u{1d53e}: '𝔾'
    MathematicalDoubleDashStruckCapitalG,
    /// \u{1d540}: '𝕀'
    MathematicalDoubleDashStruckCapitalI,
    /// \u{1d541}: '𝕁'
    MathematicalDoubleDashStruckCapitalJ,
    /// \u{1d542}: '𝕂'
    MathematicalDoubleDashStruckCapitalK,
    /// \u{1d543}: '𝕃'
    MathematicalDoubleDashStruckCapitalL,
    /// \u{1d544}: '𝕄'
    MathematicalDoubleDashStruckCapitalM,
    /// \u{1d546}: '𝕆'
    MathematicalDoubleDashStruckCapitalO,
    /// \u{1d54a}: '𝕊'
    MathematicalDoubleDashStruckCapitalS,
    /// \u{1d54b}: '𝕋'
    MathematicalDoubleDashStruckCapitalT,
    /// \u{1d54c}: '𝕌'
    MathematicalDoubleDashStruckCapitalU,
    /// \u{1d54d}: '𝕍'
    MathematicalDoubleDashStruckCapitalV,
    /// \u{1d54e}: '𝕎'
    MathematicalDoubleDashStruckCapitalW,
    /// \u{1d54f}: '𝕏'
    MathematicalDoubleDashStruckCapitalX,
    /// \u{1d550}: '𝕐'
    MathematicalDoubleDashStruckCapitalY,
    /// \u{1d552}: '𝕒'
    MathematicalDoubleDashStruckSmallA,
    /// \u{1d553}: '𝕓'
    MathematicalDoubleDashStruckSmallB,
    /// \u{1d554}: '𝕔'
    MathematicalDoubleDashStruckSmallC,
    /// \u{1d555}: '𝕕'
    MathematicalDoubleDashStruckSmallD,
    /// \u{1d556}: '𝕖'
    MathematicalDoubleDashStruckSmallE,
    /// \u{1d557}: '𝕗'
    MathematicalDoubleDashStruckSmallF,
    /// \u{1d558}: '𝕘'
    MathematicalDoubleDashStruckSmallG,
    /// \u{1d559}: '𝕙'
    MathematicalDoubleDashStruckSmallH,
    /// \u{1d55a}: '𝕚'
    MathematicalDoubleDashStruckSmallI,
    /// \u{1d55b}: '𝕛'
    MathematicalDoubleDashStruckSmallJ,
    /// \u{1d55c}: '𝕜'
    MathematicalDoubleDashStruckSmallK,
    /// \u{1d55d}: '𝕝'
    MathematicalDoubleDashStruckSmallL,
    /// \u{1d55e}: '𝕞'
    MathematicalDoubleDashStruckSmallM,
    /// \u{1d55f}: '𝕟'
    MathematicalDoubleDashStruckSmallN,
    /// \u{1d560}: '𝕠'
    MathematicalDoubleDashStruckSmallO,
    /// \u{1d561}: '𝕡'
    MathematicalDoubleDashStruckSmallP,
    /// \u{1d562}: '𝕢'
    MathematicalDoubleDashStruckSmallQ,
    /// \u{1d563}: '𝕣'
    MathematicalDoubleDashStruckSmallR,
    /// \u{1d564}: '𝕤'
    MathematicalDoubleDashStruckSmallS,
    /// \u{1d565}: '𝕥'
    MathematicalDoubleDashStruckSmallT,
    /// \u{1d566}: '𝕦'
    MathematicalDoubleDashStruckSmallU,
    /// \u{1d567}: '𝕧'
    MathematicalDoubleDashStruckSmallV,
    /// \u{1d568}: '𝕨'
    MathematicalDoubleDashStruckSmallW,
    /// \u{1d569}: '𝕩'
    MathematicalDoubleDashStruckSmallX,
    /// \u{1d56a}: '𝕪'
    MathematicalDoubleDashStruckSmallY,
    /// \u{1d56b}: '𝕫'
    MathematicalDoubleDashStruckSmallZ,
    /// \u{1d56c}: '𝕬'
    MathematicalBoldFrakturCapitalA,
    /// \u{1d56d}: '𝕭'
    MathematicalBoldFrakturCapitalB,
    /// \u{1d56e}: '𝕮'
    MathematicalBoldFrakturCapitalC,
    /// \u{1d56f}: '𝕯'
    MathematicalBoldFrakturCapitalD,
    /// \u{1d570}: '𝕰'
    MathematicalBoldFrakturCapitalE,
    /// \u{1d571}: '𝕱'
    MathematicalBoldFrakturCapitalF,
    /// \u{1d572}: '𝕲'
    MathematicalBoldFrakturCapitalG,
    /// \u{1d573}: '𝕳'
    MathematicalBoldFrakturCapitalH,
    /// \u{1d574}: '𝕴'
    MathematicalBoldFrakturCapitalI,
    /// \u{1d575}: '𝕵'
    MathematicalBoldFrakturCapitalJ,
    /// \u{1d576}: '𝕶'
    MathematicalBoldFrakturCapitalK,
    /// \u{1d577}: '𝕷'
    MathematicalBoldFrakturCapitalL,
    /// \u{1d578}: '𝕸'
    MathematicalBoldFrakturCapitalM,
    /// \u{1d579}: '𝕹'
    MathematicalBoldFrakturCapitalN,
    /// \u{1d57a}: '𝕺'
    MathematicalBoldFrakturCapitalO,
    /// \u{1d57b}: '𝕻'
    MathematicalBoldFrakturCapitalP,
    /// \u{1d57c}: '𝕼'
    MathematicalBoldFrakturCapitalQ,
    /// \u{1d57d}: '𝕽'
    MathematicalBoldFrakturCapitalR,
    /// \u{1d57e}: '𝕾'
    MathematicalBoldFrakturCapitalS,
    /// \u{1d57f}: '𝕿'
    MathematicalBoldFrakturCapitalT,
    /// \u{1d580}: '𝖀'
    MathematicalBoldFrakturCapitalU,
    /// \u{1d581}: '𝖁'
    MathematicalBoldFrakturCapitalV,
    /// \u{1d582}: '𝖂'
    MathematicalBoldFrakturCapitalW,
    /// \u{1d583}: '𝖃'
    MathematicalBoldFrakturCapitalX,
    /// \u{1d584}: '𝖄'
    MathematicalBoldFrakturCapitalY,
    /// \u{1d585}: '𝖅'
    MathematicalBoldFrakturCapitalZ,
    /// \u{1d586}: '𝖆'
    MathematicalBoldFrakturSmallA,
    /// \u{1d587}: '𝖇'
    MathematicalBoldFrakturSmallB,
    /// \u{1d588}: '𝖈'
    MathematicalBoldFrakturSmallC,
    /// \u{1d589}: '𝖉'
    MathematicalBoldFrakturSmallD,
    /// \u{1d58a}: '𝖊'
    MathematicalBoldFrakturSmallE,
    /// \u{1d58b}: '𝖋'
    MathematicalBoldFrakturSmallF,
    /// \u{1d58c}: '𝖌'
    MathematicalBoldFrakturSmallG,
    /// \u{1d58d}: '𝖍'
    MathematicalBoldFrakturSmallH,
    /// \u{1d58e}: '𝖎'
    MathematicalBoldFrakturSmallI,
    /// \u{1d58f}: '𝖏'
    MathematicalBoldFrakturSmallJ,
    /// \u{1d590}: '𝖐'
    MathematicalBoldFrakturSmallK,
    /// \u{1d591}: '𝖑'
    MathematicalBoldFrakturSmallL,
    /// \u{1d592}: '𝖒'
    MathematicalBoldFrakturSmallM,
    /// \u{1d593}: '𝖓'
    MathematicalBoldFrakturSmallN,
    /// \u{1d594}: '𝖔'
    MathematicalBoldFrakturSmallO,
    /// \u{1d595}: '𝖕'
    MathematicalBoldFrakturSmallP,
    /// \u{1d596}: '𝖖'
    MathematicalBoldFrakturSmallQ,
    /// \u{1d597}: '𝖗'
    MathematicalBoldFrakturSmallR,
    /// \u{1d598}: '𝖘'
    MathematicalBoldFrakturSmallS,
    /// \u{1d599}: '𝖙'
    MathematicalBoldFrakturSmallT,
    /// \u{1d59a}: '𝖚'
    MathematicalBoldFrakturSmallU,
    /// \u{1d59b}: '𝖛'
    MathematicalBoldFrakturSmallV,
    /// \u{1d59c}: '𝖜'
    MathematicalBoldFrakturSmallW,
    /// \u{1d59d}: '𝖝'
    MathematicalBoldFrakturSmallX,
    /// \u{1d59e}: '𝖞'
    MathematicalBoldFrakturSmallY,
    /// \u{1d59f}: '𝖟'
    MathematicalBoldFrakturSmallZ,
    /// \u{1d5a0}: '𝖠'
    MathematicalSansDashSerifCapitalA,
    /// \u{1d5a1}: '𝖡'
    MathematicalSansDashSerifCapitalB,
    /// \u{1d5a2}: '𝖢'
    MathematicalSansDashSerifCapitalC,
    /// \u{1d5a3}: '𝖣'
    MathematicalSansDashSerifCapitalD,
    /// \u{1d5a4}: '𝖤'
    MathematicalSansDashSerifCapitalE,
    /// \u{1d5a5}: '𝖥'
    MathematicalSansDashSerifCapitalF,
    /// \u{1d5a6}: '𝖦'
    MathematicalSansDashSerifCapitalG,
    /// \u{1d5a7}: '𝖧'
    MathematicalSansDashSerifCapitalH,
    /// \u{1d5a8}: '𝖨'
    MathematicalSansDashSerifCapitalI,
    /// \u{1d5a9}: '𝖩'
    MathematicalSansDashSerifCapitalJ,
    /// \u{1d5aa}: '𝖪'
    MathematicalSansDashSerifCapitalK,
    /// \u{1d5ab}: '𝖫'
    MathematicalSansDashSerifCapitalL,
    /// \u{1d5ac}: '𝖬'
    MathematicalSansDashSerifCapitalM,
    /// \u{1d5ad}: '𝖭'
    MathematicalSansDashSerifCapitalN,
    /// \u{1d5ae}: '𝖮'
    MathematicalSansDashSerifCapitalO,
    /// \u{1d5af}: '𝖯'
    MathematicalSansDashSerifCapitalP,
    /// \u{1d5b0}: '𝖰'
    MathematicalSansDashSerifCapitalQ,
    /// \u{1d5b1}: '𝖱'
    MathematicalSansDashSerifCapitalR,
    /// \u{1d5b2}: '𝖲'
    MathematicalSansDashSerifCapitalS,
    /// \u{1d5b3}: '𝖳'
    MathematicalSansDashSerifCapitalT,
    /// \u{1d5b4}: '𝖴'
    MathematicalSansDashSerifCapitalU,
    /// \u{1d5b5}: '𝖵'
    MathematicalSansDashSerifCapitalV,
    /// \u{1d5b6}: '𝖶'
    MathematicalSansDashSerifCapitalW,
    /// \u{1d5b7}: '𝖷'
    MathematicalSansDashSerifCapitalX,
    /// \u{1d5b8}: '𝖸'
    MathematicalSansDashSerifCapitalY,
    /// \u{1d5b9}: '𝖹'
    MathematicalSansDashSerifCapitalZ,
    /// \u{1d5ba}: '𝖺'
    MathematicalSansDashSerifSmallA,
    /// \u{1d5bb}: '𝖻'
    MathematicalSansDashSerifSmallB,
    /// \u{1d5bc}: '𝖼'
    MathematicalSansDashSerifSmallC,
    /// \u{1d5bd}: '𝖽'
    MathematicalSansDashSerifSmallD,
    /// \u{1d5be}: '𝖾'
    MathematicalSansDashSerifSmallE,
    /// \u{1d5bf}: '𝖿'
    MathematicalSansDashSerifSmallF,
    /// \u{1d5c0}: '𝗀'
    MathematicalSansDashSerifSmallG,
    /// \u{1d5c1}: '𝗁'
    MathematicalSansDashSerifSmallH,
    /// \u{1d5c2}: '𝗂'
    MathematicalSansDashSerifSmallI,
    /// \u{1d5c3}: '𝗃'
    MathematicalSansDashSerifSmallJ,
    /// \u{1d5c4}: '𝗄'
    MathematicalSansDashSerifSmallK,
    /// \u{1d5c5}: '𝗅'
    MathematicalSansDashSerifSmallL,
    /// \u{1d5c6}: '𝗆'
    MathematicalSansDashSerifSmallM,
    /// \u{1d5c7}: '𝗇'
    MathematicalSansDashSerifSmallN,
    /// \u{1d5c8}: '𝗈'
    MathematicalSansDashSerifSmallO,
    /// \u{1d5c9}: '𝗉'
    MathematicalSansDashSerifSmallP,
    /// \u{1d5ca}: '𝗊'
    MathematicalSansDashSerifSmallQ,
    /// \u{1d5cb}: '𝗋'
    MathematicalSansDashSerifSmallR,
    /// \u{1d5cc}: '𝗌'
    MathematicalSansDashSerifSmallS,
    /// \u{1d5cd}: '𝗍'
    MathematicalSansDashSerifSmallT,
    /// \u{1d5ce}: '𝗎'
    MathematicalSansDashSerifSmallU,
    /// \u{1d5cf}: '𝗏'
    MathematicalSansDashSerifSmallV,
    /// \u{1d5d0}: '𝗐'
    MathematicalSansDashSerifSmallW,
    /// \u{1d5d1}: '𝗑'
    MathematicalSansDashSerifSmallX,
    /// \u{1d5d2}: '𝗒'
    MathematicalSansDashSerifSmallY,
    /// \u{1d5d3}: '𝗓'
    MathematicalSansDashSerifSmallZ,
    /// \u{1d5d4}: '𝗔'
    MathematicalSansDashSerifBoldCapitalA,
    /// \u{1d5d5}: '𝗕'
    MathematicalSansDashSerifBoldCapitalB,
    /// \u{1d5d6}: '𝗖'
    MathematicalSansDashSerifBoldCapitalC,
    /// \u{1d5d7}: '𝗗'
    MathematicalSansDashSerifBoldCapitalD,
    /// \u{1d5d8}: '𝗘'
    MathematicalSansDashSerifBoldCapitalE,
    /// \u{1d5d9}: '𝗙'
    MathematicalSansDashSerifBoldCapitalF,
    /// \u{1d5da}: '𝗚'
    MathematicalSansDashSerifBoldCapitalG,
    /// \u{1d5db}: '𝗛'
    MathematicalSansDashSerifBoldCapitalH,
    /// \u{1d5dc}: '𝗜'
    MathematicalSansDashSerifBoldCapitalI,
    /// \u{1d5dd}: '𝗝'
    MathematicalSansDashSerifBoldCapitalJ,
    /// \u{1d5de}: '𝗞'
    MathematicalSansDashSerifBoldCapitalK,
    /// \u{1d5df}: '𝗟'
    MathematicalSansDashSerifBoldCapitalL,
    /// \u{1d5e0}: '𝗠'
    MathematicalSansDashSerifBoldCapitalM,
    /// \u{1d5e1}: '𝗡'
    MathematicalSansDashSerifBoldCapitalN,
    /// \u{1d5e2}: '𝗢'
    MathematicalSansDashSerifBoldCapitalO,
    /// \u{1d5e3}: '𝗣'
    MathematicalSansDashSerifBoldCapitalP,
    /// \u{1d5e4}: '𝗤'
    MathematicalSansDashSerifBoldCapitalQ,
    /// \u{1d5e5}: '𝗥'
    MathematicalSansDashSerifBoldCapitalR,
    /// \u{1d5e6}: '𝗦'
    MathematicalSansDashSerifBoldCapitalS,
    /// \u{1d5e7}: '𝗧'
    MathematicalSansDashSerifBoldCapitalT,
    /// \u{1d5e8}: '𝗨'
    MathematicalSansDashSerifBoldCapitalU,
    /// \u{1d5e9}: '𝗩'
    MathematicalSansDashSerifBoldCapitalV,
    /// \u{1d5ea}: '𝗪'
    MathematicalSansDashSerifBoldCapitalW,
    /// \u{1d5eb}: '𝗫'
    MathematicalSansDashSerifBoldCapitalX,
    /// \u{1d5ec}: '𝗬'
    MathematicalSansDashSerifBoldCapitalY,
    /// \u{1d5ed}: '𝗭'
    MathematicalSansDashSerifBoldCapitalZ,
    /// \u{1d5ee}: '𝗮'
    MathematicalSansDashSerifBoldSmallA,
    /// \u{1d5ef}: '𝗯'
    MathematicalSansDashSerifBoldSmallB,
    /// \u{1d5f0}: '𝗰'
    MathematicalSansDashSerifBoldSmallC,
    /// \u{1d5f1}: '𝗱'
    MathematicalSansDashSerifBoldSmallD,
    /// \u{1d5f2}: '𝗲'
    MathematicalSansDashSerifBoldSmallE,
    /// \u{1d5f3}: '𝗳'
    MathematicalSansDashSerifBoldSmallF,
    /// \u{1d5f4}: '𝗴'
    MathematicalSansDashSerifBoldSmallG,
    /// \u{1d5f5}: '𝗵'
    MathematicalSansDashSerifBoldSmallH,
    /// \u{1d5f6}: '𝗶'
    MathematicalSansDashSerifBoldSmallI,
    /// \u{1d5f7}: '𝗷'
    MathematicalSansDashSerifBoldSmallJ,
    /// \u{1d5f8}: '𝗸'
    MathematicalSansDashSerifBoldSmallK,
    /// \u{1d5f9}: '𝗹'
    MathematicalSansDashSerifBoldSmallL,
    /// \u{1d5fa}: '𝗺'
    MathematicalSansDashSerifBoldSmallM,
    /// \u{1d5fb}: '𝗻'
    MathematicalSansDashSerifBoldSmallN,
    /// \u{1d5fc}: '𝗼'
    MathematicalSansDashSerifBoldSmallO,
    /// \u{1d5fd}: '𝗽'
    MathematicalSansDashSerifBoldSmallP,
    /// \u{1d5fe}: '𝗾'
    MathematicalSansDashSerifBoldSmallQ,
    /// \u{1d5ff}: '𝗿'
    MathematicalSansDashSerifBoldSmallR,
    /// \u{1d600}: '𝘀'
    MathematicalSansDashSerifBoldSmallS,
    /// \u{1d601}: '𝘁'
    MathematicalSansDashSerifBoldSmallT,
    /// \u{1d602}: '𝘂'
    MathematicalSansDashSerifBoldSmallU,
    /// \u{1d603}: '𝘃'
    MathematicalSansDashSerifBoldSmallV,
    /// \u{1d604}: '𝘄'
    MathematicalSansDashSerifBoldSmallW,
    /// \u{1d605}: '𝘅'
    MathematicalSansDashSerifBoldSmallX,
    /// \u{1d606}: '𝘆'
    MathematicalSansDashSerifBoldSmallY,
    /// \u{1d607}: '𝘇'
    MathematicalSansDashSerifBoldSmallZ,
    /// \u{1d608}: '𝘈'
    MathematicalSansDashSerifItalicCapitalA,
    /// \u{1d609}: '𝘉'
    MathematicalSansDashSerifItalicCapitalB,
    /// \u{1d60a}: '𝘊'
    MathematicalSansDashSerifItalicCapitalC,
    /// \u{1d60b}: '𝘋'
    MathematicalSansDashSerifItalicCapitalD,
    /// \u{1d60c}: '𝘌'
    MathematicalSansDashSerifItalicCapitalE,
    /// \u{1d60d}: '𝘍'
    MathematicalSansDashSerifItalicCapitalF,
    /// \u{1d60e}: '𝘎'
    MathematicalSansDashSerifItalicCapitalG,
    /// \u{1d60f}: '𝘏'
    MathematicalSansDashSerifItalicCapitalH,
    /// \u{1d610}: '𝘐'
    MathematicalSansDashSerifItalicCapitalI,
    /// \u{1d611}: '𝘑'
    MathematicalSansDashSerifItalicCapitalJ,
    /// \u{1d612}: '𝘒'
    MathematicalSansDashSerifItalicCapitalK,
    /// \u{1d613}: '𝘓'
    MathematicalSansDashSerifItalicCapitalL,
    /// \u{1d614}: '𝘔'
    MathematicalSansDashSerifItalicCapitalM,
    /// \u{1d615}: '𝘕'
    MathematicalSansDashSerifItalicCapitalN,
    /// \u{1d616}: '𝘖'
    MathematicalSansDashSerifItalicCapitalO,
    /// \u{1d617}: '𝘗'
    MathematicalSansDashSerifItalicCapitalP,
    /// \u{1d618}: '𝘘'
    MathematicalSansDashSerifItalicCapitalQ,
    /// \u{1d619}: '𝘙'
    MathematicalSansDashSerifItalicCapitalR,
    /// \u{1d61a}: '𝘚'
    MathematicalSansDashSerifItalicCapitalS,
    /// \u{1d61b}: '𝘛'
    MathematicalSansDashSerifItalicCapitalT,
    /// \u{1d61c}: '𝘜'
    MathematicalSansDashSerifItalicCapitalU,
    /// \u{1d61d}: '𝘝'
    MathematicalSansDashSerifItalicCapitalV,
    /// \u{1d61e}: '𝘞'
    MathematicalSansDashSerifItalicCapitalW,
    /// \u{1d61f}: '𝘟'
    MathematicalSansDashSerifItalicCapitalX,
    /// \u{1d620}: '𝘠'
    MathematicalSansDashSerifItalicCapitalY,
    /// \u{1d621}: '𝘡'
    MathematicalSansDashSerifItalicCapitalZ,
    /// \u{1d622}: '𝘢'
    MathematicalSansDashSerifItalicSmallA,
    /// \u{1d623}: '𝘣'
    MathematicalSansDashSerifItalicSmallB,
    /// \u{1d624}: '𝘤'
    MathematicalSansDashSerifItalicSmallC,
    /// \u{1d625}: '𝘥'
    MathematicalSansDashSerifItalicSmallD,
    /// \u{1d626}: '𝘦'
    MathematicalSansDashSerifItalicSmallE,
    /// \u{1d627}: '𝘧'
    MathematicalSansDashSerifItalicSmallF,
    /// \u{1d628}: '𝘨'
    MathematicalSansDashSerifItalicSmallG,
    /// \u{1d629}: '𝘩'
    MathematicalSansDashSerifItalicSmallH,
    /// \u{1d62a}: '𝘪'
    MathematicalSansDashSerifItalicSmallI,
    /// \u{1d62b}: '𝘫'
    MathematicalSansDashSerifItalicSmallJ,
    /// \u{1d62c}: '𝘬'
    MathematicalSansDashSerifItalicSmallK,
    /// \u{1d62d}: '𝘭'
    MathematicalSansDashSerifItalicSmallL,
    /// \u{1d62e}: '𝘮'
    MathematicalSansDashSerifItalicSmallM,
    /// \u{1d62f}: '𝘯'
    MathematicalSansDashSerifItalicSmallN,
    /// \u{1d630}: '𝘰'
    MathematicalSansDashSerifItalicSmallO,
    /// \u{1d631}: '𝘱'
    MathematicalSansDashSerifItalicSmallP,
    /// \u{1d632}: '𝘲'
    MathematicalSansDashSerifItalicSmallQ,
    /// \u{1d633}: '𝘳'
    MathematicalSansDashSerifItalicSmallR,
    /// \u{1d634}: '𝘴'
    MathematicalSansDashSerifItalicSmallS,
    /// \u{1d635}: '𝘵'
    MathematicalSansDashSerifItalicSmallT,
    /// \u{1d636}: '𝘶'
    MathematicalSansDashSerifItalicSmallU,
    /// \u{1d637}: '𝘷'
    MathematicalSansDashSerifItalicSmallV,
    /// \u{1d638}: '𝘸'
    MathematicalSansDashSerifItalicSmallW,
    /// \u{1d639}: '𝘹'
    MathematicalSansDashSerifItalicSmallX,
    /// \u{1d63a}: '𝘺'
    MathematicalSansDashSerifItalicSmallY,
    /// \u{1d63b}: '𝘻'
    MathematicalSansDashSerifItalicSmallZ,
    /// \u{1d63c}: '𝘼'
    MathematicalSansDashSerifBoldItalicCapitalA,
    /// \u{1d63d}: '𝘽'
    MathematicalSansDashSerifBoldItalicCapitalB,
    /// \u{1d63e}: '𝘾'
    MathematicalSansDashSerifBoldItalicCapitalC,
    /// \u{1d63f}: '𝘿'
    MathematicalSansDashSerifBoldItalicCapitalD,
    /// \u{1d640}: '𝙀'
    MathematicalSansDashSerifBoldItalicCapitalE,
    /// \u{1d641}: '𝙁'
    MathematicalSansDashSerifBoldItalicCapitalF,
    /// \u{1d642}: '𝙂'
    MathematicalSansDashSerifBoldItalicCapitalG,
    /// \u{1d643}: '𝙃'
    MathematicalSansDashSerifBoldItalicCapitalH,
    /// \u{1d644}: '𝙄'
    MathematicalSansDashSerifBoldItalicCapitalI,
    /// \u{1d645}: '𝙅'
    MathematicalSansDashSerifBoldItalicCapitalJ,
    /// \u{1d646}: '𝙆'
    MathematicalSansDashSerifBoldItalicCapitalK,
    /// \u{1d647}: '𝙇'
    MathematicalSansDashSerifBoldItalicCapitalL,
    /// \u{1d648}: '𝙈'
    MathematicalSansDashSerifBoldItalicCapitalM,
    /// \u{1d649}: '𝙉'
    MathematicalSansDashSerifBoldItalicCapitalN,
    /// \u{1d64a}: '𝙊'
    MathematicalSansDashSerifBoldItalicCapitalO,
    /// \u{1d64b}: '𝙋'
    MathematicalSansDashSerifBoldItalicCapitalP,
    /// \u{1d64c}: '𝙌'
    MathematicalSansDashSerifBoldItalicCapitalQ,
    /// \u{1d64d}: '𝙍'
    MathematicalSansDashSerifBoldItalicCapitalR,
    /// \u{1d64e}: '𝙎'
    MathematicalSansDashSerifBoldItalicCapitalS,
    /// \u{1d64f}: '𝙏'
    MathematicalSansDashSerifBoldItalicCapitalT,
    /// \u{1d650}: '𝙐'
    MathematicalSansDashSerifBoldItalicCapitalU,
    /// \u{1d651}: '𝙑'
    MathematicalSansDashSerifBoldItalicCapitalV,
    /// \u{1d652}: '𝙒'
    MathematicalSansDashSerifBoldItalicCapitalW,
    /// \u{1d653}: '𝙓'
    MathematicalSansDashSerifBoldItalicCapitalX,
    /// \u{1d654}: '𝙔'
    MathematicalSansDashSerifBoldItalicCapitalY,
    /// \u{1d655}: '𝙕'
    MathematicalSansDashSerifBoldItalicCapitalZ,
    /// \u{1d656}: '𝙖'
    MathematicalSansDashSerifBoldItalicSmallA,
    /// \u{1d657}: '𝙗'
    MathematicalSansDashSerifBoldItalicSmallB,
    /// \u{1d658}: '𝙘'
    MathematicalSansDashSerifBoldItalicSmallC,
    /// \u{1d659}: '𝙙'
    MathematicalSansDashSerifBoldItalicSmallD,
    /// \u{1d65a}: '𝙚'
    MathematicalSansDashSerifBoldItalicSmallE,
    /// \u{1d65b}: '𝙛'
    MathematicalSansDashSerifBoldItalicSmallF,
    /// \u{1d65c}: '𝙜'
    MathematicalSansDashSerifBoldItalicSmallG,
    /// \u{1d65d}: '𝙝'
    MathematicalSansDashSerifBoldItalicSmallH,
    /// \u{1d65e}: '𝙞'
    MathematicalSansDashSerifBoldItalicSmallI,
    /// \u{1d65f}: '𝙟'
    MathematicalSansDashSerifBoldItalicSmallJ,
    /// \u{1d660}: '𝙠'
    MathematicalSansDashSerifBoldItalicSmallK,
    /// \u{1d661}: '𝙡'
    MathematicalSansDashSerifBoldItalicSmallL,
    /// \u{1d662}: '𝙢'
    MathematicalSansDashSerifBoldItalicSmallM,
    /// \u{1d663}: '𝙣'
    MathematicalSansDashSerifBoldItalicSmallN,
    /// \u{1d664}: '𝙤'
    MathematicalSansDashSerifBoldItalicSmallO,
    /// \u{1d665}: '𝙥'
    MathematicalSansDashSerifBoldItalicSmallP,
    /// \u{1d666}: '𝙦'
    MathematicalSansDashSerifBoldItalicSmallQ,
    /// \u{1d667}: '𝙧'
    MathematicalSansDashSerifBoldItalicSmallR,
    /// \u{1d668}: '𝙨'
    MathematicalSansDashSerifBoldItalicSmallS,
    /// \u{1d669}: '𝙩'
    MathematicalSansDashSerifBoldItalicSmallT,
    /// \u{1d66a}: '𝙪'
    MathematicalSansDashSerifBoldItalicSmallU,
    /// \u{1d66b}: '𝙫'
    MathematicalSansDashSerifBoldItalicSmallV,
    /// \u{1d66c}: '𝙬'
    MathematicalSansDashSerifBoldItalicSmallW,
    /// \u{1d66d}: '𝙭'
    MathematicalSansDashSerifBoldItalicSmallX,
    /// \u{1d66e}: '𝙮'
    MathematicalSansDashSerifBoldItalicSmallY,
    /// \u{1d66f}: '𝙯'
    MathematicalSansDashSerifBoldItalicSmallZ,
    /// \u{1d670}: '𝙰'
    MathematicalMonospaceCapitalA,
    /// \u{1d671}: '𝙱'
    MathematicalMonospaceCapitalB,
    /// \u{1d672}: '𝙲'
    MathematicalMonospaceCapitalC,
    /// \u{1d673}: '𝙳'
    MathematicalMonospaceCapitalD,
    /// \u{1d674}: '𝙴'
    MathematicalMonospaceCapitalE,
    /// \u{1d675}: '𝙵'
    MathematicalMonospaceCapitalF,
    /// \u{1d676}: '𝙶'
    MathematicalMonospaceCapitalG,
    /// \u{1d677}: '𝙷'
    MathematicalMonospaceCapitalH,
    /// \u{1d678}: '𝙸'
    MathematicalMonospaceCapitalI,
    /// \u{1d679}: '𝙹'
    MathematicalMonospaceCapitalJ,
    /// \u{1d67a}: '𝙺'
    MathematicalMonospaceCapitalK,
    /// \u{1d67b}: '𝙻'
    MathematicalMonospaceCapitalL,
    /// \u{1d67c}: '𝙼'
    MathematicalMonospaceCapitalM,
    /// \u{1d67d}: '𝙽'
    MathematicalMonospaceCapitalN,
    /// \u{1d67e}: '𝙾'
    MathematicalMonospaceCapitalO,
    /// \u{1d67f}: '𝙿'
    MathematicalMonospaceCapitalP,
    /// \u{1d680}: '𝚀'
    MathematicalMonospaceCapitalQ,
    /// \u{1d681}: '𝚁'
    MathematicalMonospaceCapitalR,
    /// \u{1d682}: '𝚂'
    MathematicalMonospaceCapitalS,
    /// \u{1d683}: '𝚃'
    MathematicalMonospaceCapitalT,
    /// \u{1d684}: '𝚄'
    MathematicalMonospaceCapitalU,
    /// \u{1d685}: '𝚅'
    MathematicalMonospaceCapitalV,
    /// \u{1d686}: '𝚆'
    MathematicalMonospaceCapitalW,
    /// \u{1d687}: '𝚇'
    MathematicalMonospaceCapitalX,
    /// \u{1d688}: '𝚈'
    MathematicalMonospaceCapitalY,
    /// \u{1d689}: '𝚉'
    MathematicalMonospaceCapitalZ,
    /// \u{1d68a}: '𝚊'
    MathematicalMonospaceSmallA,
    /// \u{1d68b}: '𝚋'
    MathematicalMonospaceSmallB,
    /// \u{1d68c}: '𝚌'
    MathematicalMonospaceSmallC,
    /// \u{1d68d}: '𝚍'
    MathematicalMonospaceSmallD,
    /// \u{1d68e}: '𝚎'
    MathematicalMonospaceSmallE,
    /// \u{1d68f}: '𝚏'
    MathematicalMonospaceSmallF,
    /// \u{1d690}: '𝚐'
    MathematicalMonospaceSmallG,
    /// \u{1d691}: '𝚑'
    MathematicalMonospaceSmallH,
    /// \u{1d692}: '𝚒'
    MathematicalMonospaceSmallI,
    /// \u{1d693}: '𝚓'
    MathematicalMonospaceSmallJ,
    /// \u{1d694}: '𝚔'
    MathematicalMonospaceSmallK,
    /// \u{1d695}: '𝚕'
    MathematicalMonospaceSmallL,
    /// \u{1d696}: '𝚖'
    MathematicalMonospaceSmallM,
    /// \u{1d697}: '𝚗'
    MathematicalMonospaceSmallN,
    /// \u{1d698}: '𝚘'
    MathematicalMonospaceSmallO,
    /// \u{1d699}: '𝚙'
    MathematicalMonospaceSmallP,
    /// \u{1d69a}: '𝚚'
    MathematicalMonospaceSmallQ,
    /// \u{1d69b}: '𝚛'
    MathematicalMonospaceSmallR,
    /// \u{1d69c}: '𝚜'
    MathematicalMonospaceSmallS,
    /// \u{1d69d}: '𝚝'
    MathematicalMonospaceSmallT,
    /// \u{1d69e}: '𝚞'
    MathematicalMonospaceSmallU,
    /// \u{1d69f}: '𝚟'
    MathematicalMonospaceSmallV,
    /// \u{1d6a0}: '𝚠'
    MathematicalMonospaceSmallW,
    /// \u{1d6a1}: '𝚡'
    MathematicalMonospaceSmallX,
    /// \u{1d6a2}: '𝚢'
    MathematicalMonospaceSmallY,
    /// \u{1d6a3}: '𝚣'
    MathematicalMonospaceSmallZ,
    /// \u{1d6a4}: '𝚤'
    MathematicalItalicSmallDotlessI,
    /// \u{1d6a5}: '𝚥'
    MathematicalItalicSmallDotlessJ,
    /// \u{1d6a8}: '𝚨'
    MathematicalBoldCapitalAlpha,
    /// \u{1d6a9}: '𝚩'
    MathematicalBoldCapitalBeta,
    /// \u{1d6aa}: '𝚪'
    MathematicalBoldCapitalGamma,
    /// \u{1d6ab}: '𝚫'
    MathematicalBoldCapitalDelta,
    /// \u{1d6ac}: '𝚬'
    MathematicalBoldCapitalEpsilon,
    /// \u{1d6ad}: '𝚭'
    MathematicalBoldCapitalZeta,
    /// \u{1d6ae}: '𝚮'
    MathematicalBoldCapitalEta,
    /// \u{1d6af}: '𝚯'
    MathematicalBoldCapitalTheta,
    /// \u{1d6b0}: '𝚰'
    MathematicalBoldCapitalIota,
    /// \u{1d6b1}: '𝚱'
    MathematicalBoldCapitalKappa,
    /// \u{1d6b2}: '𝚲'
    MathematicalBoldCapitalLamda,
    /// \u{1d6b3}: '𝚳'
    MathematicalBoldCapitalMu,
    /// \u{1d6b4}: '𝚴'
    MathematicalBoldCapitalNu,
    /// \u{1d6b5}: '𝚵'
    MathematicalBoldCapitalXi,
    /// \u{1d6b6}: '𝚶'
    MathematicalBoldCapitalOmicron,
    /// \u{1d6b7}: '𝚷'
    MathematicalBoldCapitalPi,
    /// \u{1d6b8}: '𝚸'
    MathematicalBoldCapitalRho,
    /// \u{1d6b9}: '𝚹'
    MathematicalBoldCapitalThetaSymbol,
    /// \u{1d6ba}: '𝚺'
    MathematicalBoldCapitalSigma,
    /// \u{1d6bb}: '𝚻'
    MathematicalBoldCapitalTau,
    /// \u{1d6bc}: '𝚼'
    MathematicalBoldCapitalUpsilon,
    /// \u{1d6bd}: '𝚽'
    MathematicalBoldCapitalPhi,
    /// \u{1d6be}: '𝚾'
    MathematicalBoldCapitalChi,
    /// \u{1d6bf}: '𝚿'
    MathematicalBoldCapitalPsi,
    /// \u{1d6c0}: '𝛀'
    MathematicalBoldCapitalOmega,
    /// \u{1d6c1}: '𝛁'
    MathematicalBoldNabla,
    /// \u{1d6c2}: '𝛂'
    MathematicalBoldSmallAlpha,
    /// \u{1d6c3}: '𝛃'
    MathematicalBoldSmallBeta,
    /// \u{1d6c4}: '𝛄'
    MathematicalBoldSmallGamma,
    /// \u{1d6c5}: '𝛅'
    MathematicalBoldSmallDelta,
    /// \u{1d6c6}: '𝛆'
    MathematicalBoldSmallEpsilon,
    /// \u{1d6c7}: '𝛇'
    MathematicalBoldSmallZeta,
    /// \u{1d6c8}: '𝛈'
    MathematicalBoldSmallEta,
    /// \u{1d6c9}: '𝛉'
    MathematicalBoldSmallTheta,
    /// \u{1d6ca}: '𝛊'
    MathematicalBoldSmallIota,
    /// \u{1d6cb}: '𝛋'
    MathematicalBoldSmallKappa,
    /// \u{1d6cc}: '𝛌'
    MathematicalBoldSmallLamda,
    /// \u{1d6cd}: '𝛍'
    MathematicalBoldSmallMu,
    /// \u{1d6ce}: '𝛎'
    MathematicalBoldSmallNu,
    /// \u{1d6cf}: '𝛏'
    MathematicalBoldSmallXi,
    /// \u{1d6d0}: '𝛐'
    MathematicalBoldSmallOmicron,
    /// \u{1d6d1}: '𝛑'
    MathematicalBoldSmallPi,
    /// \u{1d6d2}: '𝛒'
    MathematicalBoldSmallRho,
    /// \u{1d6d3}: '𝛓'
    MathematicalBoldSmallFinalSigma,
    /// \u{1d6d4}: '𝛔'
    MathematicalBoldSmallSigma,
    /// \u{1d6d5}: '𝛕'
    MathematicalBoldSmallTau,
    /// \u{1d6d6}: '𝛖'
    MathematicalBoldSmallUpsilon,
    /// \u{1d6d7}: '𝛗'
    MathematicalBoldSmallPhi,
    /// \u{1d6d8}: '𝛘'
    MathematicalBoldSmallChi,
    /// \u{1d6d9}: '𝛙'
    MathematicalBoldSmallPsi,
    /// \u{1d6da}: '𝛚'
    MathematicalBoldSmallOmega,
    /// \u{1d6db}: '𝛛'
    MathematicalBoldPartialDifferential,
    /// \u{1d6dc}: '𝛜'
    MathematicalBoldEpsilonSymbol,
    /// \u{1d6dd}: '𝛝'
    MathematicalBoldThetaSymbol,
    /// \u{1d6de}: '𝛞'
    MathematicalBoldKappaSymbol,
    /// \u{1d6df}: '𝛟'
    MathematicalBoldPhiSymbol,
    /// \u{1d6e0}: '𝛠'
    MathematicalBoldRhoSymbol,
    /// \u{1d6e1}: '𝛡'
    MathematicalBoldPiSymbol,
    /// \u{1d6e2}: '𝛢'
    MathematicalItalicCapitalAlpha,
    /// \u{1d6e3}: '𝛣'
    MathematicalItalicCapitalBeta,
    /// \u{1d6e4}: '𝛤'
    MathematicalItalicCapitalGamma,
    /// \u{1d6e5}: '𝛥'
    MathematicalItalicCapitalDelta,
    /// \u{1d6e6}: '𝛦'
    MathematicalItalicCapitalEpsilon,
    /// \u{1d6e7}: '𝛧'
    MathematicalItalicCapitalZeta,
    /// \u{1d6e8}: '𝛨'
    MathematicalItalicCapitalEta,
    /// \u{1d6e9}: '𝛩'
    MathematicalItalicCapitalTheta,
    /// \u{1d6ea}: '𝛪'
    MathematicalItalicCapitalIota,
    /// \u{1d6eb}: '𝛫'
    MathematicalItalicCapitalKappa,
    /// \u{1d6ec}: '𝛬'
    MathematicalItalicCapitalLamda,
    /// \u{1d6ed}: '𝛭'
    MathematicalItalicCapitalMu,
    /// \u{1d6ee}: '𝛮'
    MathematicalItalicCapitalNu,
    /// \u{1d6ef}: '𝛯'
    MathematicalItalicCapitalXi,
    /// \u{1d6f0}: '𝛰'
    MathematicalItalicCapitalOmicron,
    /// \u{1d6f1}: '𝛱'
    MathematicalItalicCapitalPi,
    /// \u{1d6f2}: '𝛲'
    MathematicalItalicCapitalRho,
    /// \u{1d6f3}: '𝛳'
    MathematicalItalicCapitalThetaSymbol,
    /// \u{1d6f4}: '𝛴'
    MathematicalItalicCapitalSigma,
    /// \u{1d6f5}: '𝛵'
    MathematicalItalicCapitalTau,
    /// \u{1d6f6}: '𝛶'
    MathematicalItalicCapitalUpsilon,
    /// \u{1d6f7}: '𝛷'
    MathematicalItalicCapitalPhi,
    /// \u{1d6f8}: '𝛸'
    MathematicalItalicCapitalChi,
    /// \u{1d6f9}: '𝛹'
    MathematicalItalicCapitalPsi,
    /// \u{1d6fa}: '𝛺'
    MathematicalItalicCapitalOmega,
    /// \u{1d6fb}: '𝛻'
    MathematicalItalicNabla,
    /// \u{1d6fc}: '𝛼'
    MathematicalItalicSmallAlpha,
    /// \u{1d6fd}: '𝛽'
    MathematicalItalicSmallBeta,
    /// \u{1d6fe}: '𝛾'
    MathematicalItalicSmallGamma,
    /// \u{1d6ff}: '𝛿'
    MathematicalItalicSmallDelta,
    /// \u{1d700}: '𝜀'
    MathematicalItalicSmallEpsilon,
    /// \u{1d701}: '𝜁'
    MathematicalItalicSmallZeta,
    /// \u{1d702}: '𝜂'
    MathematicalItalicSmallEta,
    /// \u{1d703}: '𝜃'
    MathematicalItalicSmallTheta,
    /// \u{1d704}: '𝜄'
    MathematicalItalicSmallIota,
    /// \u{1d705}: '𝜅'
    MathematicalItalicSmallKappa,
    /// \u{1d706}: '𝜆'
    MathematicalItalicSmallLamda,
    /// \u{1d707}: '𝜇'
    MathematicalItalicSmallMu,
    /// \u{1d708}: '𝜈'
    MathematicalItalicSmallNu,
    /// \u{1d709}: '𝜉'
    MathematicalItalicSmallXi,
    /// \u{1d70a}: '𝜊'
    MathematicalItalicSmallOmicron,
    /// \u{1d70b}: '𝜋'
    MathematicalItalicSmallPi,
    /// \u{1d70c}: '𝜌'
    MathematicalItalicSmallRho,
    /// \u{1d70d}: '𝜍'
    MathematicalItalicSmallFinalSigma,
    /// \u{1d70e}: '𝜎'
    MathematicalItalicSmallSigma,
    /// \u{1d70f}: '𝜏'
    MathematicalItalicSmallTau,
    /// \u{1d710}: '𝜐'
    MathematicalItalicSmallUpsilon,
    /// \u{1d711}: '𝜑'
    MathematicalItalicSmallPhi,
    /// \u{1d712}: '𝜒'
    MathematicalItalicSmallChi,
    /// \u{1d713}: '𝜓'
    MathematicalItalicSmallPsi,
    /// \u{1d714}: '𝜔'
    MathematicalItalicSmallOmega,
    /// \u{1d715}: '𝜕'
    MathematicalItalicPartialDifferential,
    /// \u{1d716}: '𝜖'
    MathematicalItalicEpsilonSymbol,
    /// \u{1d717}: '𝜗'
    MathematicalItalicThetaSymbol,
    /// \u{1d718}: '𝜘'
    MathematicalItalicKappaSymbol,
    /// \u{1d719}: '𝜙'
    MathematicalItalicPhiSymbol,
    /// \u{1d71a}: '𝜚'
    MathematicalItalicRhoSymbol,
    /// \u{1d71b}: '𝜛'
    MathematicalItalicPiSymbol,
    /// \u{1d71c}: '𝜜'
    MathematicalBoldItalicCapitalAlpha,
    /// \u{1d71d}: '𝜝'
    MathematicalBoldItalicCapitalBeta,
    /// \u{1d71e}: '𝜞'
    MathematicalBoldItalicCapitalGamma,
    /// \u{1d71f}: '𝜟'
    MathematicalBoldItalicCapitalDelta,
    /// \u{1d720}: '𝜠'
    MathematicalBoldItalicCapitalEpsilon,
    /// \u{1d721}: '𝜡'
    MathematicalBoldItalicCapitalZeta,
    /// \u{1d722}: '𝜢'
    MathematicalBoldItalicCapitalEta,
    /// \u{1d723}: '𝜣'
    MathematicalBoldItalicCapitalTheta,
    /// \u{1d724}: '𝜤'
    MathematicalBoldItalicCapitalIota,
    /// \u{1d725}: '𝜥'
    MathematicalBoldItalicCapitalKappa,
    /// \u{1d726}: '𝜦'
    MathematicalBoldItalicCapitalLamda,
    /// \u{1d727}: '𝜧'
    MathematicalBoldItalicCapitalMu,
    /// \u{1d728}: '𝜨'
    MathematicalBoldItalicCapitalNu,
    /// \u{1d729}: '𝜩'
    MathematicalBoldItalicCapitalXi,
    /// \u{1d72a}: '𝜪'
    MathematicalBoldItalicCapitalOmicron,
    /// \u{1d72b}: '𝜫'
    MathematicalBoldItalicCapitalPi,
    /// \u{1d72c}: '𝜬'
    MathematicalBoldItalicCapitalRho,
    /// \u{1d72d}: '𝜭'
    MathematicalBoldItalicCapitalThetaSymbol,
    /// \u{1d72e}: '𝜮'
    MathematicalBoldItalicCapitalSigma,
    /// \u{1d72f}: '𝜯'
    MathematicalBoldItalicCapitalTau,
    /// \u{1d730}: '𝜰'
    MathematicalBoldItalicCapitalUpsilon,
    /// \u{1d731}: '𝜱'
    MathematicalBoldItalicCapitalPhi,
    /// \u{1d732}: '𝜲'
    MathematicalBoldItalicCapitalChi,
    /// \u{1d733}: '𝜳'
    MathematicalBoldItalicCapitalPsi,
    /// \u{1d734}: '𝜴'
    MathematicalBoldItalicCapitalOmega,
    /// \u{1d735}: '𝜵'
    MathematicalBoldItalicNabla,
    /// \u{1d736}: '𝜶'
    MathematicalBoldItalicSmallAlpha,
    /// \u{1d737}: '𝜷'
    MathematicalBoldItalicSmallBeta,
    /// \u{1d738}: '𝜸'
    MathematicalBoldItalicSmallGamma,
    /// \u{1d739}: '𝜹'
    MathematicalBoldItalicSmallDelta,
    /// \u{1d73a}: '𝜺'
    MathematicalBoldItalicSmallEpsilon,
    /// \u{1d73b}: '𝜻'
    MathematicalBoldItalicSmallZeta,
    /// \u{1d73c}: '𝜼'
    MathematicalBoldItalicSmallEta,
    /// \u{1d73d}: '𝜽'
    MathematicalBoldItalicSmallTheta,
    /// \u{1d73e}: '𝜾'
    MathematicalBoldItalicSmallIota,
    /// \u{1d73f}: '𝜿'
    MathematicalBoldItalicSmallKappa,
    /// \u{1d740}: '𝝀'
    MathematicalBoldItalicSmallLamda,
    /// \u{1d741}: '𝝁'
    MathematicalBoldItalicSmallMu,
    /// \u{1d742}: '𝝂'
    MathematicalBoldItalicSmallNu,
    /// \u{1d743}: '𝝃'
    MathematicalBoldItalicSmallXi,
    /// \u{1d744}: '𝝄'
    MathematicalBoldItalicSmallOmicron,
    /// \u{1d745}: '𝝅'
    MathematicalBoldItalicSmallPi,
    /// \u{1d746}: '𝝆'
    MathematicalBoldItalicSmallRho,
    /// \u{1d747}: '𝝇'
    MathematicalBoldItalicSmallFinalSigma,
    /// \u{1d748}: '𝝈'
    MathematicalBoldItalicSmallSigma,
    /// \u{1d749}: '𝝉'
    MathematicalBoldItalicSmallTau,
    /// \u{1d74a}: '𝝊'
    MathematicalBoldItalicSmallUpsilon,
    /// \u{1d74b}: '𝝋'
    MathematicalBoldItalicSmallPhi,
    /// \u{1d74c}: '𝝌'
    MathematicalBoldItalicSmallChi,
    /// \u{1d74d}: '𝝍'
    MathematicalBoldItalicSmallPsi,
    /// \u{1d74e}: '𝝎'
    MathematicalBoldItalicSmallOmega,
    /// \u{1d74f}: '𝝏'
    MathematicalBoldItalicPartialDifferential,
    /// \u{1d750}: '𝝐'
    MathematicalBoldItalicEpsilonSymbol,
    /// \u{1d751}: '𝝑'
    MathematicalBoldItalicThetaSymbol,
    /// \u{1d752}: '𝝒'
    MathematicalBoldItalicKappaSymbol,
    /// \u{1d753}: '𝝓'
    MathematicalBoldItalicPhiSymbol,
    /// \u{1d754}: '𝝔'
    MathematicalBoldItalicRhoSymbol,
    /// \u{1d755}: '𝝕'
    MathematicalBoldItalicPiSymbol,
    /// \u{1d756}: '𝝖'
    MathematicalSansDashSerifBoldCapitalAlpha,
    /// \u{1d757}: '𝝗'
    MathematicalSansDashSerifBoldCapitalBeta,
    /// \u{1d758}: '𝝘'
    MathematicalSansDashSerifBoldCapitalGamma,
    /// \u{1d759}: '𝝙'
    MathematicalSansDashSerifBoldCapitalDelta,
    /// \u{1d75a}: '𝝚'
    MathematicalSansDashSerifBoldCapitalEpsilon,
    /// \u{1d75b}: '𝝛'
    MathematicalSansDashSerifBoldCapitalZeta,
    /// \u{1d75c}: '𝝜'
    MathematicalSansDashSerifBoldCapitalEta,
    /// \u{1d75d}: '𝝝'
    MathematicalSansDashSerifBoldCapitalTheta,
    /// \u{1d75e}: '𝝞'
    MathematicalSansDashSerifBoldCapitalIota,
    /// \u{1d75f}: '𝝟'
    MathematicalSansDashSerifBoldCapitalKappa,
    /// \u{1d760}: '𝝠'
    MathematicalSansDashSerifBoldCapitalLamda,
    /// \u{1d761}: '𝝡'
    MathematicalSansDashSerifBoldCapitalMu,
    /// \u{1d762}: '𝝢'
    MathematicalSansDashSerifBoldCapitalNu,
    /// \u{1d763}: '𝝣'
    MathematicalSansDashSerifBoldCapitalXi,
    /// \u{1d764}: '𝝤'
    MathematicalSansDashSerifBoldCapitalOmicron,
    /// \u{1d765}: '𝝥'
    MathematicalSansDashSerifBoldCapitalPi,
    /// \u{1d766}: '𝝦'
    MathematicalSansDashSerifBoldCapitalRho,
    /// \u{1d767}: '𝝧'
    MathematicalSansDashSerifBoldCapitalThetaSymbol,
    /// \u{1d768}: '𝝨'
    MathematicalSansDashSerifBoldCapitalSigma,
    /// \u{1d769}: '𝝩'
    MathematicalSansDashSerifBoldCapitalTau,
    /// \u{1d76a}: '𝝪'
    MathematicalSansDashSerifBoldCapitalUpsilon,
    /// \u{1d76b}: '𝝫'
    MathematicalSansDashSerifBoldCapitalPhi,
    /// \u{1d76c}: '𝝬'
    MathematicalSansDashSerifBoldCapitalChi,
    /// \u{1d76d}: '𝝭'
    MathematicalSansDashSerifBoldCapitalPsi,
    /// \u{1d76e}: '𝝮'
    MathematicalSansDashSerifBoldCapitalOmega,
    /// \u{1d76f}: '𝝯'
    MathematicalSansDashSerifBoldNabla,
    /// \u{1d770}: '𝝰'
    MathematicalSansDashSerifBoldSmallAlpha,
    /// \u{1d771}: '𝝱'
    MathematicalSansDashSerifBoldSmallBeta,
    /// \u{1d772}: '𝝲'
    MathematicalSansDashSerifBoldSmallGamma,
    /// \u{1d773}: '𝝳'
    MathematicalSansDashSerifBoldSmallDelta,
    /// \u{1d774}: '𝝴'
    MathematicalSansDashSerifBoldSmallEpsilon,
    /// \u{1d775}: '𝝵'
    MathematicalSansDashSerifBoldSmallZeta,
    /// \u{1d776}: '𝝶'
    MathematicalSansDashSerifBoldSmallEta,
    /// \u{1d777}: '𝝷'
    MathematicalSansDashSerifBoldSmallTheta,
    /// \u{1d778}: '𝝸'
    MathematicalSansDashSerifBoldSmallIota,
    /// \u{1d779}: '𝝹'
    MathematicalSansDashSerifBoldSmallKappa,
    /// \u{1d77a}: '𝝺'
    MathematicalSansDashSerifBoldSmallLamda,
    /// \u{1d77b}: '𝝻'
    MathematicalSansDashSerifBoldSmallMu,
    /// \u{1d77c}: '𝝼'
    MathematicalSansDashSerifBoldSmallNu,
    /// \u{1d77d}: '𝝽'
    MathematicalSansDashSerifBoldSmallXi,
    /// \u{1d77e}: '𝝾'
    MathematicalSansDashSerifBoldSmallOmicron,
    /// \u{1d77f}: '𝝿'
    MathematicalSansDashSerifBoldSmallPi,
    /// \u{1d780}: '𝞀'
    MathematicalSansDashSerifBoldSmallRho,
    /// \u{1d781}: '𝞁'
    MathematicalSansDashSerifBoldSmallFinalSigma,
    /// \u{1d782}: '𝞂'
    MathematicalSansDashSerifBoldSmallSigma,
    /// \u{1d783}: '𝞃'
    MathematicalSansDashSerifBoldSmallTau,
    /// \u{1d784}: '𝞄'
    MathematicalSansDashSerifBoldSmallUpsilon,
    /// \u{1d785}: '𝞅'
    MathematicalSansDashSerifBoldSmallPhi,
    /// \u{1d786}: '𝞆'
    MathematicalSansDashSerifBoldSmallChi,
    /// \u{1d787}: '𝞇'
    MathematicalSansDashSerifBoldSmallPsi,
    /// \u{1d788}: '𝞈'
    MathematicalSansDashSerifBoldSmallOmega,
    /// \u{1d789}: '𝞉'
    MathematicalSansDashSerifBoldPartialDifferential,
    /// \u{1d78a}: '𝞊'
    MathematicalSansDashSerifBoldEpsilonSymbol,
    /// \u{1d78b}: '𝞋'
    MathematicalSansDashSerifBoldThetaSymbol,
    /// \u{1d78c}: '𝞌'
    MathematicalSansDashSerifBoldKappaSymbol,
    /// \u{1d78d}: '𝞍'
    MathematicalSansDashSerifBoldPhiSymbol,
    /// \u{1d78e}: '𝞎'
    MathematicalSansDashSerifBoldRhoSymbol,
    /// \u{1d78f}: '𝞏'
    MathematicalSansDashSerifBoldPiSymbol,
    /// \u{1d790}: '𝞐'
    MathematicalSansDashSerifBoldItalicCapitalAlpha,
    /// \u{1d791}: '𝞑'
    MathematicalSansDashSerifBoldItalicCapitalBeta,
    /// \u{1d792}: '𝞒'
    MathematicalSansDashSerifBoldItalicCapitalGamma,
    /// \u{1d793}: '𝞓'
    MathematicalSansDashSerifBoldItalicCapitalDelta,
    /// \u{1d794}: '𝞔'
    MathematicalSansDashSerifBoldItalicCapitalEpsilon,
    /// \u{1d795}: '𝞕'
    MathematicalSansDashSerifBoldItalicCapitalZeta,
    /// \u{1d796}: '𝞖'
    MathematicalSansDashSerifBoldItalicCapitalEta,
    /// \u{1d797}: '𝞗'
    MathematicalSansDashSerifBoldItalicCapitalTheta,
    /// \u{1d798}: '𝞘'
    MathematicalSansDashSerifBoldItalicCapitalIota,
    /// \u{1d799}: '𝞙'
    MathematicalSansDashSerifBoldItalicCapitalKappa,
    /// \u{1d79a}: '𝞚'
    MathematicalSansDashSerifBoldItalicCapitalLamda,
    /// \u{1d79b}: '𝞛'
    MathematicalSansDashSerifBoldItalicCapitalMu,
    /// \u{1d79c}: '𝞜'
    MathematicalSansDashSerifBoldItalicCapitalNu,
    /// \u{1d79d}: '𝞝'
    MathematicalSansDashSerifBoldItalicCapitalXi,
    /// \u{1d79e}: '𝞞'
    MathematicalSansDashSerifBoldItalicCapitalOmicron,
    /// \u{1d79f}: '𝞟'
    MathematicalSansDashSerifBoldItalicCapitalPi,
    /// \u{1d7a0}: '𝞠'
    MathematicalSansDashSerifBoldItalicCapitalRho,
    /// \u{1d7a1}: '𝞡'
    MathematicalSansDashSerifBoldItalicCapitalThetaSymbol,
    /// \u{1d7a2}: '𝞢'
    MathematicalSansDashSerifBoldItalicCapitalSigma,
    /// \u{1d7a3}: '𝞣'
    MathematicalSansDashSerifBoldItalicCapitalTau,
    /// \u{1d7a4}: '𝞤'
    MathematicalSansDashSerifBoldItalicCapitalUpsilon,
    /// \u{1d7a5}: '𝞥'
    MathematicalSansDashSerifBoldItalicCapitalPhi,
    /// \u{1d7a6}: '𝞦'
    MathematicalSansDashSerifBoldItalicCapitalChi,
    /// \u{1d7a7}: '𝞧'
    MathematicalSansDashSerifBoldItalicCapitalPsi,
    /// \u{1d7a8}: '𝞨'
    MathematicalSansDashSerifBoldItalicCapitalOmega,
    /// \u{1d7a9}: '𝞩'
    MathematicalSansDashSerifBoldItalicNabla,
    /// \u{1d7aa}: '𝞪'
    MathematicalSansDashSerifBoldItalicSmallAlpha,
    /// \u{1d7ab}: '𝞫'
    MathematicalSansDashSerifBoldItalicSmallBeta,
    /// \u{1d7ac}: '𝞬'
    MathematicalSansDashSerifBoldItalicSmallGamma,
    /// \u{1d7ad}: '𝞭'
    MathematicalSansDashSerifBoldItalicSmallDelta,
    /// \u{1d7ae}: '𝞮'
    MathematicalSansDashSerifBoldItalicSmallEpsilon,
    /// \u{1d7af}: '𝞯'
    MathematicalSansDashSerifBoldItalicSmallZeta,
    /// \u{1d7b0}: '𝞰'
    MathematicalSansDashSerifBoldItalicSmallEta,
    /// \u{1d7b1}: '𝞱'
    MathematicalSansDashSerifBoldItalicSmallTheta,
    /// \u{1d7b2}: '𝞲'
    MathematicalSansDashSerifBoldItalicSmallIota,
    /// \u{1d7b3}: '𝞳'
    MathematicalSansDashSerifBoldItalicSmallKappa,
    /// \u{1d7b4}: '𝞴'
    MathematicalSansDashSerifBoldItalicSmallLamda,
    /// \u{1d7b5}: '𝞵'
    MathematicalSansDashSerifBoldItalicSmallMu,
    /// \u{1d7b6}: '𝞶'
    MathematicalSansDashSerifBoldItalicSmallNu,
    /// \u{1d7b7}: '𝞷'
    MathematicalSansDashSerifBoldItalicSmallXi,
    /// \u{1d7b8}: '𝞸'
    MathematicalSansDashSerifBoldItalicSmallOmicron,
    /// \u{1d7b9}: '𝞹'
    MathematicalSansDashSerifBoldItalicSmallPi,
    /// \u{1d7ba}: '𝞺'
    MathematicalSansDashSerifBoldItalicSmallRho,
    /// \u{1d7bb}: '𝞻'
    MathematicalSansDashSerifBoldItalicSmallFinalSigma,
    /// \u{1d7bc}: '𝞼'
    MathematicalSansDashSerifBoldItalicSmallSigma,
    /// \u{1d7bd}: '𝞽'
    MathematicalSansDashSerifBoldItalicSmallTau,
    /// \u{1d7be}: '𝞾'
    MathematicalSansDashSerifBoldItalicSmallUpsilon,
    /// \u{1d7bf}: '𝞿'
    MathematicalSansDashSerifBoldItalicSmallPhi,
    /// \u{1d7c0}: '𝟀'
    MathematicalSansDashSerifBoldItalicSmallChi,
    /// \u{1d7c1}: '𝟁'
    MathematicalSansDashSerifBoldItalicSmallPsi,
    /// \u{1d7c2}: '𝟂'
    MathematicalSansDashSerifBoldItalicSmallOmega,
    /// \u{1d7c3}: '𝟃'
    MathematicalSansDashSerifBoldItalicPartialDifferential,
    /// \u{1d7c4}: '𝟄'
    MathematicalSansDashSerifBoldItalicEpsilonSymbol,
    /// \u{1d7c5}: '𝟅'
    MathematicalSansDashSerifBoldItalicThetaSymbol,
    /// \u{1d7c6}: '𝟆'
    MathematicalSansDashSerifBoldItalicKappaSymbol,
    /// \u{1d7c7}: '𝟇'
    MathematicalSansDashSerifBoldItalicPhiSymbol,
    /// \u{1d7c8}: '𝟈'
    MathematicalSansDashSerifBoldItalicRhoSymbol,
    /// \u{1d7c9}: '𝟉'
    MathematicalSansDashSerifBoldItalicPiSymbol,
    /// \u{1d7ca}: '𝟊'
    MathematicalBoldCapitalDigamma,
    /// \u{1d7cb}: '𝟋'
    MathematicalBoldSmallDigamma,
    /// \u{1d7ce}: '𝟎'
    MathematicalBoldDigitZero,
    /// \u{1d7cf}: '𝟏'
    MathematicalBoldDigitOne,
    /// \u{1d7d0}: '𝟐'
    MathematicalBoldDigitTwo,
    /// \u{1d7d1}: '𝟑'
    MathematicalBoldDigitThree,
    /// \u{1d7d2}: '𝟒'
    MathematicalBoldDigitFour,
    /// \u{1d7d3}: '𝟓'
    MathematicalBoldDigitFive,
    /// \u{1d7d4}: '𝟔'
    MathematicalBoldDigitSix,
    /// \u{1d7d5}: '𝟕'
    MathematicalBoldDigitSeven,
    /// \u{1d7d6}: '𝟖'
    MathematicalBoldDigitEight,
    /// \u{1d7d7}: '𝟗'
    MathematicalBoldDigitNine,
    /// \u{1d7d8}: '𝟘'
    MathematicalDoubleDashStruckDigitZero,
    /// \u{1d7d9}: '𝟙'
    MathematicalDoubleDashStruckDigitOne,
    /// \u{1d7da}: '𝟚'
    MathematicalDoubleDashStruckDigitTwo,
    /// \u{1d7db}: '𝟛'
    MathematicalDoubleDashStruckDigitThree,
    /// \u{1d7dc}: '𝟜'
    MathematicalDoubleDashStruckDigitFour,
    /// \u{1d7dd}: '𝟝'
    MathematicalDoubleDashStruckDigitFive,
    /// \u{1d7de}: '𝟞'
    MathematicalDoubleDashStruckDigitSix,
    /// \u{1d7df}: '𝟟'
    MathematicalDoubleDashStruckDigitSeven,
    /// \u{1d7e0}: '𝟠'
    MathematicalDoubleDashStruckDigitEight,
    /// \u{1d7e1}: '𝟡'
    MathematicalDoubleDashStruckDigitNine,
    /// \u{1d7e2}: '𝟢'
    MathematicalSansDashSerifDigitZero,
    /// \u{1d7e3}: '𝟣'
    MathematicalSansDashSerifDigitOne,
    /// \u{1d7e4}: '𝟤'
    MathematicalSansDashSerifDigitTwo,
    /// \u{1d7e5}: '𝟥'
    MathematicalSansDashSerifDigitThree,
    /// \u{1d7e6}: '𝟦'
    MathematicalSansDashSerifDigitFour,
    /// \u{1d7e7}: '𝟧'
    MathematicalSansDashSerifDigitFive,
    /// \u{1d7e8}: '𝟨'
    MathematicalSansDashSerifDigitSix,
    /// \u{1d7e9}: '𝟩'
    MathematicalSansDashSerifDigitSeven,
    /// \u{1d7ea}: '𝟪'
    MathematicalSansDashSerifDigitEight,
    /// \u{1d7eb}: '𝟫'
    MathematicalSansDashSerifDigitNine,
    /// \u{1d7ec}: '𝟬'
    MathematicalSansDashSerifBoldDigitZero,
    /// \u{1d7ed}: '𝟭'
    MathematicalSansDashSerifBoldDigitOne,
    /// \u{1d7ee}: '𝟮'
    MathematicalSansDashSerifBoldDigitTwo,
    /// \u{1d7ef}: '𝟯'
    MathematicalSansDashSerifBoldDigitThree,
    /// \u{1d7f0}: '𝟰'
    MathematicalSansDashSerifBoldDigitFour,
    /// \u{1d7f1}: '𝟱'
    MathematicalSansDashSerifBoldDigitFive,
    /// \u{1d7f2}: '𝟲'
    MathematicalSansDashSerifBoldDigitSix,
    /// \u{1d7f3}: '𝟳'
    MathematicalSansDashSerifBoldDigitSeven,
    /// \u{1d7f4}: '𝟴'
    MathematicalSansDashSerifBoldDigitEight,
    /// \u{1d7f5}: '𝟵'
    MathematicalSansDashSerifBoldDigitNine,
    /// \u{1d7f6}: '𝟶'
    MathematicalMonospaceDigitZero,
    /// \u{1d7f7}: '𝟷'
    MathematicalMonospaceDigitOne,
    /// \u{1d7f8}: '𝟸'
    MathematicalMonospaceDigitTwo,
    /// \u{1d7f9}: '𝟹'
    MathematicalMonospaceDigitThree,
    /// \u{1d7fa}: '𝟺'
    MathematicalMonospaceDigitFour,
    /// \u{1d7fb}: '𝟻'
    MathematicalMonospaceDigitFive,
    /// \u{1d7fc}: '𝟼'
    MathematicalMonospaceDigitSix,
    /// \u{1d7fd}: '𝟽'
    MathematicalMonospaceDigitSeven,
    /// \u{1d7fe}: '𝟾'
    MathematicalMonospaceDigitEight,
}

impl Into<char> for MathematicalAlphanumericSymbols {
    fn into(self) -> char {
        match self {
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalA => '𝐀',
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalB => '𝐁',
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalC => '𝐂',
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalD => '𝐃',
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalE => '𝐄',
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalF => '𝐅',
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalG => '𝐆',
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalH => '𝐇',
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalI => '𝐈',
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalJ => '𝐉',
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalK => '𝐊',
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalL => '𝐋',
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalM => '𝐌',
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalN => '𝐍',
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalO => '𝐎',
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalP => '𝐏',
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalQ => '𝐐',
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalR => '𝐑',
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalS => '𝐒',
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalT => '𝐓',
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalU => '𝐔',
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalV => '𝐕',
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalW => '𝐖',
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalX => '𝐗',
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalY => '𝐘',
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalZ => '𝐙',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallA => '𝐚',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallB => '𝐛',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallC => '𝐜',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallD => '𝐝',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallE => '𝐞',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallF => '𝐟',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallG => '𝐠',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallH => '𝐡',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallI => '𝐢',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallJ => '𝐣',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallK => '𝐤',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallL => '𝐥',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallM => '𝐦',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallN => '𝐧',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallO => '𝐨',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallP => '𝐩',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallQ => '𝐪',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallR => '𝐫',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallS => '𝐬',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallT => '𝐭',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallU => '𝐮',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallV => '𝐯',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallW => '𝐰',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallX => '𝐱',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallY => '𝐲',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallZ => '𝐳',
            MathematicalAlphanumericSymbols::MathematicalItalicCapitalA => '𝐴',
            MathematicalAlphanumericSymbols::MathematicalItalicCapitalB => '𝐵',
            MathematicalAlphanumericSymbols::MathematicalItalicCapitalC => '𝐶',
            MathematicalAlphanumericSymbols::MathematicalItalicCapitalD => '𝐷',
            MathematicalAlphanumericSymbols::MathematicalItalicCapitalE => '𝐸',
            MathematicalAlphanumericSymbols::MathematicalItalicCapitalF => '𝐹',
            MathematicalAlphanumericSymbols::MathematicalItalicCapitalG => '𝐺',
            MathematicalAlphanumericSymbols::MathematicalItalicCapitalH => '𝐻',
            MathematicalAlphanumericSymbols::MathematicalItalicCapitalI => '𝐼',
            MathematicalAlphanumericSymbols::MathematicalItalicCapitalJ => '𝐽',
            MathematicalAlphanumericSymbols::MathematicalItalicCapitalK => '𝐾',
            MathematicalAlphanumericSymbols::MathematicalItalicCapitalL => '𝐿',
            MathematicalAlphanumericSymbols::MathematicalItalicCapitalM => '𝑀',
            MathematicalAlphanumericSymbols::MathematicalItalicCapitalN => '𝑁',
            MathematicalAlphanumericSymbols::MathematicalItalicCapitalO => '𝑂',
            MathematicalAlphanumericSymbols::MathematicalItalicCapitalP => '𝑃',
            MathematicalAlphanumericSymbols::MathematicalItalicCapitalQ => '𝑄',
            MathematicalAlphanumericSymbols::MathematicalItalicCapitalR => '𝑅',
            MathematicalAlphanumericSymbols::MathematicalItalicCapitalS => '𝑆',
            MathematicalAlphanumericSymbols::MathematicalItalicCapitalT => '𝑇',
            MathematicalAlphanumericSymbols::MathematicalItalicCapitalU => '𝑈',
            MathematicalAlphanumericSymbols::MathematicalItalicCapitalV => '𝑉',
            MathematicalAlphanumericSymbols::MathematicalItalicCapitalW => '𝑊',
            MathematicalAlphanumericSymbols::MathematicalItalicCapitalX => '𝑋',
            MathematicalAlphanumericSymbols::MathematicalItalicCapitalY => '𝑌',
            MathematicalAlphanumericSymbols::MathematicalItalicCapitalZ => '𝑍',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallA => '𝑎',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallB => '𝑏',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallC => '𝑐',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallD => '𝑑',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallE => '𝑒',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallF => '𝑓',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallG => '𝑔',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallI => '𝑖',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallJ => '𝑗',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallK => '𝑘',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallL => '𝑙',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallM => '𝑚',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallN => '𝑛',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallO => '𝑜',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallP => '𝑝',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallQ => '𝑞',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallR => '𝑟',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallS => '𝑠',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallT => '𝑡',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallU => '𝑢',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallV => '𝑣',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallW => '𝑤',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallX => '𝑥',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallY => '𝑦',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallZ => '𝑧',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalA => '𝑨',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalB => '𝑩',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalC => '𝑪',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalD => '𝑫',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalE => '𝑬',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalF => '𝑭',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalG => '𝑮',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalH => '𝑯',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalI => '𝑰',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalJ => '𝑱',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalK => '𝑲',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalL => '𝑳',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalM => '𝑴',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalN => '𝑵',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalO => '𝑶',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalP => '𝑷',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalQ => '𝑸',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalR => '𝑹',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalS => '𝑺',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalT => '𝑻',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalU => '𝑼',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalV => '𝑽',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalW => '𝑾',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalX => '𝑿',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalY => '𝒀',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalZ => '𝒁',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallA => '𝒂',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallB => '𝒃',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallC => '𝒄',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallD => '𝒅',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallE => '𝒆',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallF => '𝒇',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallG => '𝒈',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallH => '𝒉',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallI => '𝒊',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallJ => '𝒋',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallK => '𝒌',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallL => '𝒍',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallM => '𝒎',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallN => '𝒏',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallO => '𝒐',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallP => '𝒑',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallQ => '𝒒',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallR => '𝒓',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallS => '𝒔',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallT => '𝒕',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallU => '𝒖',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallV => '𝒗',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallW => '𝒘',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallX => '𝒙',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallY => '𝒚',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallZ => '𝒛',
            MathematicalAlphanumericSymbols::MathematicalScriptCapitalA => '𝒜',
            MathematicalAlphanumericSymbols::MathematicalScriptCapitalC => '𝒞',
            MathematicalAlphanumericSymbols::MathematicalScriptCapitalD => '𝒟',
            MathematicalAlphanumericSymbols::MathematicalScriptCapitalG => '𝒢',
            MathematicalAlphanumericSymbols::MathematicalScriptCapitalJ => '𝒥',
            MathematicalAlphanumericSymbols::MathematicalScriptCapitalK => '𝒦',
            MathematicalAlphanumericSymbols::MathematicalScriptCapitalN => '𝒩',
            MathematicalAlphanumericSymbols::MathematicalScriptCapitalO => '𝒪',
            MathematicalAlphanumericSymbols::MathematicalScriptCapitalP => '𝒫',
            MathematicalAlphanumericSymbols::MathematicalScriptCapitalQ => '𝒬',
            MathematicalAlphanumericSymbols::MathematicalScriptCapitalS => '𝒮',
            MathematicalAlphanumericSymbols::MathematicalScriptCapitalT => '𝒯',
            MathematicalAlphanumericSymbols::MathematicalScriptCapitalU => '𝒰',
            MathematicalAlphanumericSymbols::MathematicalScriptCapitalV => '𝒱',
            MathematicalAlphanumericSymbols::MathematicalScriptCapitalW => '𝒲',
            MathematicalAlphanumericSymbols::MathematicalScriptCapitalX => '𝒳',
            MathematicalAlphanumericSymbols::MathematicalScriptCapitalY => '𝒴',
            MathematicalAlphanumericSymbols::MathematicalScriptCapitalZ => '𝒵',
            MathematicalAlphanumericSymbols::MathematicalScriptSmallA => '𝒶',
            MathematicalAlphanumericSymbols::MathematicalScriptSmallB => '𝒷',
            MathematicalAlphanumericSymbols::MathematicalScriptSmallC => '𝒸',
            MathematicalAlphanumericSymbols::MathematicalScriptSmallD => '𝒹',
            MathematicalAlphanumericSymbols::MathematicalScriptSmallF => '𝒻',
            MathematicalAlphanumericSymbols::MathematicalScriptSmallH => '𝒽',
            MathematicalAlphanumericSymbols::MathematicalScriptSmallI => '𝒾',
            MathematicalAlphanumericSymbols::MathematicalScriptSmallJ => '𝒿',
            MathematicalAlphanumericSymbols::MathematicalScriptSmallK => '𝓀',
            MathematicalAlphanumericSymbols::MathematicalScriptSmallL => '𝓁',
            MathematicalAlphanumericSymbols::MathematicalScriptSmallM => '𝓂',
            MathematicalAlphanumericSymbols::MathematicalScriptSmallN => '𝓃',
            MathematicalAlphanumericSymbols::MathematicalScriptSmallP => '𝓅',
            MathematicalAlphanumericSymbols::MathematicalScriptSmallQ => '𝓆',
            MathematicalAlphanumericSymbols::MathematicalScriptSmallR => '𝓇',
            MathematicalAlphanumericSymbols::MathematicalScriptSmallS => '𝓈',
            MathematicalAlphanumericSymbols::MathematicalScriptSmallT => '𝓉',
            MathematicalAlphanumericSymbols::MathematicalScriptSmallU => '𝓊',
            MathematicalAlphanumericSymbols::MathematicalScriptSmallV => '𝓋',
            MathematicalAlphanumericSymbols::MathematicalScriptSmallW => '𝓌',
            MathematicalAlphanumericSymbols::MathematicalScriptSmallX => '𝓍',
            MathematicalAlphanumericSymbols::MathematicalScriptSmallY => '𝓎',
            MathematicalAlphanumericSymbols::MathematicalScriptSmallZ => '𝓏',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalA => '𝓐',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalB => '𝓑',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalC => '𝓒',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalD => '𝓓',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalE => '𝓔',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalF => '𝓕',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalG => '𝓖',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalH => '𝓗',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalI => '𝓘',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalJ => '𝓙',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalK => '𝓚',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalL => '𝓛',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalM => '𝓜',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalN => '𝓝',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalO => '𝓞',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalP => '𝓟',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalQ => '𝓠',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalR => '𝓡',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalS => '𝓢',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalT => '𝓣',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalU => '𝓤',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalV => '𝓥',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalW => '𝓦',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalX => '𝓧',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalY => '𝓨',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalZ => '𝓩',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallA => '𝓪',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallB => '𝓫',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallC => '𝓬',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallD => '𝓭',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallE => '𝓮',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallF => '𝓯',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallG => '𝓰',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallH => '𝓱',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallI => '𝓲',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallJ => '𝓳',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallK => '𝓴',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallL => '𝓵',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallM => '𝓶',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallN => '𝓷',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallO => '𝓸',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallP => '𝓹',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallQ => '𝓺',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallR => '𝓻',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallS => '𝓼',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallT => '𝓽',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallU => '𝓾',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallV => '𝓿',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallW => '𝔀',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallX => '𝔁',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallY => '𝔂',
            MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallZ => '𝔃',
            MathematicalAlphanumericSymbols::MathematicalFrakturCapitalA => '𝔄',
            MathematicalAlphanumericSymbols::MathematicalFrakturCapitalB => '𝔅',
            MathematicalAlphanumericSymbols::MathematicalFrakturCapitalD => '𝔇',
            MathematicalAlphanumericSymbols::MathematicalFrakturCapitalE => '𝔈',
            MathematicalAlphanumericSymbols::MathematicalFrakturCapitalF => '𝔉',
            MathematicalAlphanumericSymbols::MathematicalFrakturCapitalG => '𝔊',
            MathematicalAlphanumericSymbols::MathematicalFrakturCapitalJ => '𝔍',
            MathematicalAlphanumericSymbols::MathematicalFrakturCapitalK => '𝔎',
            MathematicalAlphanumericSymbols::MathematicalFrakturCapitalL => '𝔏',
            MathematicalAlphanumericSymbols::MathematicalFrakturCapitalM => '𝔐',
            MathematicalAlphanumericSymbols::MathematicalFrakturCapitalN => '𝔑',
            MathematicalAlphanumericSymbols::MathematicalFrakturCapitalO => '𝔒',
            MathematicalAlphanumericSymbols::MathematicalFrakturCapitalP => '𝔓',
            MathematicalAlphanumericSymbols::MathematicalFrakturCapitalQ => '𝔔',
            MathematicalAlphanumericSymbols::MathematicalFrakturCapitalS => '𝔖',
            MathematicalAlphanumericSymbols::MathematicalFrakturCapitalT => '𝔗',
            MathematicalAlphanumericSymbols::MathematicalFrakturCapitalU => '𝔘',
            MathematicalAlphanumericSymbols::MathematicalFrakturCapitalV => '𝔙',
            MathematicalAlphanumericSymbols::MathematicalFrakturCapitalW => '𝔚',
            MathematicalAlphanumericSymbols::MathematicalFrakturCapitalX => '𝔛',
            MathematicalAlphanumericSymbols::MathematicalFrakturCapitalY => '𝔜',
            MathematicalAlphanumericSymbols::MathematicalFrakturSmallA => '𝔞',
            MathematicalAlphanumericSymbols::MathematicalFrakturSmallB => '𝔟',
            MathematicalAlphanumericSymbols::MathematicalFrakturSmallC => '𝔠',
            MathematicalAlphanumericSymbols::MathematicalFrakturSmallD => '𝔡',
            MathematicalAlphanumericSymbols::MathematicalFrakturSmallE => '𝔢',
            MathematicalAlphanumericSymbols::MathematicalFrakturSmallF => '𝔣',
            MathematicalAlphanumericSymbols::MathematicalFrakturSmallG => '𝔤',
            MathematicalAlphanumericSymbols::MathematicalFrakturSmallH => '𝔥',
            MathematicalAlphanumericSymbols::MathematicalFrakturSmallI => '𝔦',
            MathematicalAlphanumericSymbols::MathematicalFrakturSmallJ => '𝔧',
            MathematicalAlphanumericSymbols::MathematicalFrakturSmallK => '𝔨',
            MathematicalAlphanumericSymbols::MathematicalFrakturSmallL => '𝔩',
            MathematicalAlphanumericSymbols::MathematicalFrakturSmallM => '𝔪',
            MathematicalAlphanumericSymbols::MathematicalFrakturSmallN => '𝔫',
            MathematicalAlphanumericSymbols::MathematicalFrakturSmallO => '𝔬',
            MathematicalAlphanumericSymbols::MathematicalFrakturSmallP => '𝔭',
            MathematicalAlphanumericSymbols::MathematicalFrakturSmallQ => '𝔮',
            MathematicalAlphanumericSymbols::MathematicalFrakturSmallR => '𝔯',
            MathematicalAlphanumericSymbols::MathematicalFrakturSmallS => '𝔰',
            MathematicalAlphanumericSymbols::MathematicalFrakturSmallT => '𝔱',
            MathematicalAlphanumericSymbols::MathematicalFrakturSmallU => '𝔲',
            MathematicalAlphanumericSymbols::MathematicalFrakturSmallV => '𝔳',
            MathematicalAlphanumericSymbols::MathematicalFrakturSmallW => '𝔴',
            MathematicalAlphanumericSymbols::MathematicalFrakturSmallX => '𝔵',
            MathematicalAlphanumericSymbols::MathematicalFrakturSmallY => '𝔶',
            MathematicalAlphanumericSymbols::MathematicalFrakturSmallZ => '𝔷',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckCapitalA => '𝔸',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckCapitalB => '𝔹',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckCapitalD => '𝔻',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckCapitalE => '𝔼',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckCapitalF => '𝔽',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckCapitalG => '𝔾',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckCapitalI => '𝕀',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckCapitalJ => '𝕁',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckCapitalK => '𝕂',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckCapitalL => '𝕃',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckCapitalM => '𝕄',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckCapitalO => '𝕆',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckCapitalS => '𝕊',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckCapitalT => '𝕋',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckCapitalU => '𝕌',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckCapitalV => '𝕍',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckCapitalW => '𝕎',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckCapitalX => '𝕏',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckCapitalY => '𝕐',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallA => '𝕒',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallB => '𝕓',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallC => '𝕔',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallD => '𝕕',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallE => '𝕖',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallF => '𝕗',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallG => '𝕘',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallH => '𝕙',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallI => '𝕚',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallJ => '𝕛',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallK => '𝕜',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallL => '𝕝',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallM => '𝕞',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallN => '𝕟',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallO => '𝕠',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallP => '𝕡',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallQ => '𝕢',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallR => '𝕣',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallS => '𝕤',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallT => '𝕥',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallU => '𝕦',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallV => '𝕧',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallW => '𝕨',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallX => '𝕩',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallY => '𝕪',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallZ => '𝕫',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalA => '𝕬',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalB => '𝕭',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalC => '𝕮',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalD => '𝕯',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalE => '𝕰',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalF => '𝕱',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalG => '𝕲',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalH => '𝕳',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalI => '𝕴',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalJ => '𝕵',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalK => '𝕶',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalL => '𝕷',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalM => '𝕸',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalN => '𝕹',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalO => '𝕺',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalP => '𝕻',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalQ => '𝕼',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalR => '𝕽',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalS => '𝕾',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalT => '𝕿',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalU => '𝖀',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalV => '𝖁',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalW => '𝖂',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalX => '𝖃',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalY => '𝖄',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalZ => '𝖅',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallA => '𝖆',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallB => '𝖇',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallC => '𝖈',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallD => '𝖉',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallE => '𝖊',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallF => '𝖋',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallG => '𝖌',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallH => '𝖍',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallI => '𝖎',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallJ => '𝖏',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallK => '𝖐',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallL => '𝖑',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallM => '𝖒',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallN => '𝖓',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallO => '𝖔',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallP => '𝖕',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallQ => '𝖖',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallR => '𝖗',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallS => '𝖘',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallT => '𝖙',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallU => '𝖚',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallV => '𝖛',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallW => '𝖜',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallX => '𝖝',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallY => '𝖞',
            MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallZ => '𝖟',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalA => '𝖠',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalB => '𝖡',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalC => '𝖢',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalD => '𝖣',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalE => '𝖤',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalF => '𝖥',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalG => '𝖦',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalH => '𝖧',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalI => '𝖨',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalJ => '𝖩',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalK => '𝖪',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalL => '𝖫',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalM => '𝖬',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalN => '𝖭',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalO => '𝖮',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalP => '𝖯',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalQ => '𝖰',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalR => '𝖱',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalS => '𝖲',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalT => '𝖳',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalU => '𝖴',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalV => '𝖵',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalW => '𝖶',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalX => '𝖷',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalY => '𝖸',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalZ => '𝖹',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallA => '𝖺',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallB => '𝖻',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallC => '𝖼',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallD => '𝖽',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallE => '𝖾',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallF => '𝖿',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallG => '𝗀',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallH => '𝗁',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallI => '𝗂',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallJ => '𝗃',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallK => '𝗄',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallL => '𝗅',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallM => '𝗆',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallN => '𝗇',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallO => '𝗈',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallP => '𝗉',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallQ => '𝗊',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallR => '𝗋',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallS => '𝗌',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallT => '𝗍',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallU => '𝗎',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallV => '𝗏',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallW => '𝗐',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallX => '𝗑',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallY => '𝗒',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallZ => '𝗓',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalA => '𝗔',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalB => '𝗕',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalC => '𝗖',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalD => '𝗗',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalE => '𝗘',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalF => '𝗙',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalG => '𝗚',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalH => '𝗛',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalI => '𝗜',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalJ => '𝗝',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalK => '𝗞',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalL => '𝗟',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalM => '𝗠',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalN => '𝗡',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalO => '𝗢',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalP => '𝗣',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalQ => '𝗤',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalR => '𝗥',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalS => '𝗦',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalT => '𝗧',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalU => '𝗨',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalV => '𝗩',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalW => '𝗪',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalX => '𝗫',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalY => '𝗬',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalZ => '𝗭',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallA => '𝗮',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallB => '𝗯',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallC => '𝗰',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallD => '𝗱',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallE => '𝗲',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallF => '𝗳',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallG => '𝗴',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallH => '𝗵',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallI => '𝗶',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallJ => '𝗷',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallK => '𝗸',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallL => '𝗹',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallM => '𝗺',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallN => '𝗻',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallO => '𝗼',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallP => '𝗽',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallQ => '𝗾',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallR => '𝗿',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallS => '𝘀',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallT => '𝘁',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallU => '𝘂',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallV => '𝘃',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallW => '𝘄',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallX => '𝘅',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallY => '𝘆',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallZ => '𝘇',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalA => '𝘈',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalB => '𝘉',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalC => '𝘊',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalD => '𝘋',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalE => '𝘌',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalF => '𝘍',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalG => '𝘎',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalH => '𝘏',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalI => '𝘐',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalJ => '𝘑',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalK => '𝘒',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalL => '𝘓',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalM => '𝘔',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalN => '𝘕',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalO => '𝘖',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalP => '𝘗',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalQ => '𝘘',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalR => '𝘙',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalS => '𝘚',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalT => '𝘛',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalU => '𝘜',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalV => '𝘝',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalW => '𝘞',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalX => '𝘟',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalY => '𝘠',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalZ => '𝘡',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallA => '𝘢',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallB => '𝘣',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallC => '𝘤',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallD => '𝘥',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallE => '𝘦',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallF => '𝘧',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallG => '𝘨',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallH => '𝘩',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallI => '𝘪',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallJ => '𝘫',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallK => '𝘬',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallL => '𝘭',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallM => '𝘮',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallN => '𝘯',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallO => '𝘰',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallP => '𝘱',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallQ => '𝘲',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallR => '𝘳',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallS => '𝘴',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallT => '𝘵',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallU => '𝘶',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallV => '𝘷',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallW => '𝘸',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallX => '𝘹',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallY => '𝘺',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallZ => '𝘻',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalA => '𝘼',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalB => '𝘽',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalC => '𝘾',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalD => '𝘿',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalE => '𝙀',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalF => '𝙁',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalG => '𝙂',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalH => '𝙃',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalI => '𝙄',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalJ => '𝙅',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalK => '𝙆',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalL => '𝙇',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalM => '𝙈',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalN => '𝙉',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalO => '𝙊',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalP => '𝙋',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalQ => '𝙌',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalR => '𝙍',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalS => '𝙎',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalT => '𝙏',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalU => '𝙐',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalV => '𝙑',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalW => '𝙒',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalX => '𝙓',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalY => '𝙔',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalZ => '𝙕',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallA => '𝙖',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallB => '𝙗',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallC => '𝙘',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallD => '𝙙',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallE => '𝙚',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallF => '𝙛',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallG => '𝙜',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallH => '𝙝',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallI => '𝙞',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallJ => '𝙟',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallK => '𝙠',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallL => '𝙡',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallM => '𝙢',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallN => '𝙣',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallO => '𝙤',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallP => '𝙥',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallQ => '𝙦',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallR => '𝙧',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallS => '𝙨',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallT => '𝙩',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallU => '𝙪',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallV => '𝙫',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallW => '𝙬',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallX => '𝙭',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallY => '𝙮',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallZ => '𝙯',
            MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalA => '𝙰',
            MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalB => '𝙱',
            MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalC => '𝙲',
            MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalD => '𝙳',
            MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalE => '𝙴',
            MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalF => '𝙵',
            MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalG => '𝙶',
            MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalH => '𝙷',
            MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalI => '𝙸',
            MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalJ => '𝙹',
            MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalK => '𝙺',
            MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalL => '𝙻',
            MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalM => '𝙼',
            MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalN => '𝙽',
            MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalO => '𝙾',
            MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalP => '𝙿',
            MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalQ => '𝚀',
            MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalR => '𝚁',
            MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalS => '𝚂',
            MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalT => '𝚃',
            MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalU => '𝚄',
            MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalV => '𝚅',
            MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalW => '𝚆',
            MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalX => '𝚇',
            MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalY => '𝚈',
            MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalZ => '𝚉',
            MathematicalAlphanumericSymbols::MathematicalMonospaceSmallA => '𝚊',
            MathematicalAlphanumericSymbols::MathematicalMonospaceSmallB => '𝚋',
            MathematicalAlphanumericSymbols::MathematicalMonospaceSmallC => '𝚌',
            MathematicalAlphanumericSymbols::MathematicalMonospaceSmallD => '𝚍',
            MathematicalAlphanumericSymbols::MathematicalMonospaceSmallE => '𝚎',
            MathematicalAlphanumericSymbols::MathematicalMonospaceSmallF => '𝚏',
            MathematicalAlphanumericSymbols::MathematicalMonospaceSmallG => '𝚐',
            MathematicalAlphanumericSymbols::MathematicalMonospaceSmallH => '𝚑',
            MathematicalAlphanumericSymbols::MathematicalMonospaceSmallI => '𝚒',
            MathematicalAlphanumericSymbols::MathematicalMonospaceSmallJ => '𝚓',
            MathematicalAlphanumericSymbols::MathematicalMonospaceSmallK => '𝚔',
            MathematicalAlphanumericSymbols::MathematicalMonospaceSmallL => '𝚕',
            MathematicalAlphanumericSymbols::MathematicalMonospaceSmallM => '𝚖',
            MathematicalAlphanumericSymbols::MathematicalMonospaceSmallN => '𝚗',
            MathematicalAlphanumericSymbols::MathematicalMonospaceSmallO => '𝚘',
            MathematicalAlphanumericSymbols::MathematicalMonospaceSmallP => '𝚙',
            MathematicalAlphanumericSymbols::MathematicalMonospaceSmallQ => '𝚚',
            MathematicalAlphanumericSymbols::MathematicalMonospaceSmallR => '𝚛',
            MathematicalAlphanumericSymbols::MathematicalMonospaceSmallS => '𝚜',
            MathematicalAlphanumericSymbols::MathematicalMonospaceSmallT => '𝚝',
            MathematicalAlphanumericSymbols::MathematicalMonospaceSmallU => '𝚞',
            MathematicalAlphanumericSymbols::MathematicalMonospaceSmallV => '𝚟',
            MathematicalAlphanumericSymbols::MathematicalMonospaceSmallW => '𝚠',
            MathematicalAlphanumericSymbols::MathematicalMonospaceSmallX => '𝚡',
            MathematicalAlphanumericSymbols::MathematicalMonospaceSmallY => '𝚢',
            MathematicalAlphanumericSymbols::MathematicalMonospaceSmallZ => '𝚣',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallDotlessI => '𝚤',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallDotlessJ => '𝚥',
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalAlpha => '𝚨',
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalBeta => '𝚩',
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalGamma => '𝚪',
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalDelta => '𝚫',
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalEpsilon => '𝚬',
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalZeta => '𝚭',
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalEta => '𝚮',
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalTheta => '𝚯',
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalIota => '𝚰',
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalKappa => '𝚱',
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalLamda => '𝚲',
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalMu => '𝚳',
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalNu => '𝚴',
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalXi => '𝚵',
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalOmicron => '𝚶',
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalPi => '𝚷',
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalRho => '𝚸',
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalThetaSymbol => '𝚹',
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalSigma => '𝚺',
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalTau => '𝚻',
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalUpsilon => '𝚼',
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalPhi => '𝚽',
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalChi => '𝚾',
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalPsi => '𝚿',
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalOmega => '𝛀',
            MathematicalAlphanumericSymbols::MathematicalBoldNabla => '𝛁',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallAlpha => '𝛂',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallBeta => '𝛃',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallGamma => '𝛄',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallDelta => '𝛅',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallEpsilon => '𝛆',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallZeta => '𝛇',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallEta => '𝛈',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallTheta => '𝛉',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallIota => '𝛊',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallKappa => '𝛋',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallLamda => '𝛌',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallMu => '𝛍',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallNu => '𝛎',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallXi => '𝛏',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallOmicron => '𝛐',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallPi => '𝛑',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallRho => '𝛒',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallFinalSigma => '𝛓',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallSigma => '𝛔',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallTau => '𝛕',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallUpsilon => '𝛖',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallPhi => '𝛗',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallChi => '𝛘',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallPsi => '𝛙',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallOmega => '𝛚',
            MathematicalAlphanumericSymbols::MathematicalBoldPartialDifferential => '𝛛',
            MathematicalAlphanumericSymbols::MathematicalBoldEpsilonSymbol => '𝛜',
            MathematicalAlphanumericSymbols::MathematicalBoldThetaSymbol => '𝛝',
            MathematicalAlphanumericSymbols::MathematicalBoldKappaSymbol => '𝛞',
            MathematicalAlphanumericSymbols::MathematicalBoldPhiSymbol => '𝛟',
            MathematicalAlphanumericSymbols::MathematicalBoldRhoSymbol => '𝛠',
            MathematicalAlphanumericSymbols::MathematicalBoldPiSymbol => '𝛡',
            MathematicalAlphanumericSymbols::MathematicalItalicCapitalAlpha => '𝛢',
            MathematicalAlphanumericSymbols::MathematicalItalicCapitalBeta => '𝛣',
            MathematicalAlphanumericSymbols::MathematicalItalicCapitalGamma => '𝛤',
            MathematicalAlphanumericSymbols::MathematicalItalicCapitalDelta => '𝛥',
            MathematicalAlphanumericSymbols::MathematicalItalicCapitalEpsilon => '𝛦',
            MathematicalAlphanumericSymbols::MathematicalItalicCapitalZeta => '𝛧',
            MathematicalAlphanumericSymbols::MathematicalItalicCapitalEta => '𝛨',
            MathematicalAlphanumericSymbols::MathematicalItalicCapitalTheta => '𝛩',
            MathematicalAlphanumericSymbols::MathematicalItalicCapitalIota => '𝛪',
            MathematicalAlphanumericSymbols::MathematicalItalicCapitalKappa => '𝛫',
            MathematicalAlphanumericSymbols::MathematicalItalicCapitalLamda => '𝛬',
            MathematicalAlphanumericSymbols::MathematicalItalicCapitalMu => '𝛭',
            MathematicalAlphanumericSymbols::MathematicalItalicCapitalNu => '𝛮',
            MathematicalAlphanumericSymbols::MathematicalItalicCapitalXi => '𝛯',
            MathematicalAlphanumericSymbols::MathematicalItalicCapitalOmicron => '𝛰',
            MathematicalAlphanumericSymbols::MathematicalItalicCapitalPi => '𝛱',
            MathematicalAlphanumericSymbols::MathematicalItalicCapitalRho => '𝛲',
            MathematicalAlphanumericSymbols::MathematicalItalicCapitalThetaSymbol => '𝛳',
            MathematicalAlphanumericSymbols::MathematicalItalicCapitalSigma => '𝛴',
            MathematicalAlphanumericSymbols::MathematicalItalicCapitalTau => '𝛵',
            MathematicalAlphanumericSymbols::MathematicalItalicCapitalUpsilon => '𝛶',
            MathematicalAlphanumericSymbols::MathematicalItalicCapitalPhi => '𝛷',
            MathematicalAlphanumericSymbols::MathematicalItalicCapitalChi => '𝛸',
            MathematicalAlphanumericSymbols::MathematicalItalicCapitalPsi => '𝛹',
            MathematicalAlphanumericSymbols::MathematicalItalicCapitalOmega => '𝛺',
            MathematicalAlphanumericSymbols::MathematicalItalicNabla => '𝛻',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallAlpha => '𝛼',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallBeta => '𝛽',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallGamma => '𝛾',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallDelta => '𝛿',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallEpsilon => '𝜀',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallZeta => '𝜁',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallEta => '𝜂',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallTheta => '𝜃',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallIota => '𝜄',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallKappa => '𝜅',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallLamda => '𝜆',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallMu => '𝜇',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallNu => '𝜈',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallXi => '𝜉',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallOmicron => '𝜊',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallPi => '𝜋',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallRho => '𝜌',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallFinalSigma => '𝜍',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallSigma => '𝜎',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallTau => '𝜏',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallUpsilon => '𝜐',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallPhi => '𝜑',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallChi => '𝜒',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallPsi => '𝜓',
            MathematicalAlphanumericSymbols::MathematicalItalicSmallOmega => '𝜔',
            MathematicalAlphanumericSymbols::MathematicalItalicPartialDifferential => '𝜕',
            MathematicalAlphanumericSymbols::MathematicalItalicEpsilonSymbol => '𝜖',
            MathematicalAlphanumericSymbols::MathematicalItalicThetaSymbol => '𝜗',
            MathematicalAlphanumericSymbols::MathematicalItalicKappaSymbol => '𝜘',
            MathematicalAlphanumericSymbols::MathematicalItalicPhiSymbol => '𝜙',
            MathematicalAlphanumericSymbols::MathematicalItalicRhoSymbol => '𝜚',
            MathematicalAlphanumericSymbols::MathematicalItalicPiSymbol => '𝜛',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalAlpha => '𝜜',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalBeta => '𝜝',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalGamma => '𝜞',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalDelta => '𝜟',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalEpsilon => '𝜠',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalZeta => '𝜡',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalEta => '𝜢',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalTheta => '𝜣',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalIota => '𝜤',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalKappa => '𝜥',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalLamda => '𝜦',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalMu => '𝜧',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalNu => '𝜨',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalXi => '𝜩',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalOmicron => '𝜪',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalPi => '𝜫',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalRho => '𝜬',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalThetaSymbol => '𝜭',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalSigma => '𝜮',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalTau => '𝜯',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalUpsilon => '𝜰',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalPhi => '𝜱',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalChi => '𝜲',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalPsi => '𝜳',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalOmega => '𝜴',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicNabla => '𝜵',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallAlpha => '𝜶',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallBeta => '𝜷',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallGamma => '𝜸',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallDelta => '𝜹',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallEpsilon => '𝜺',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallZeta => '𝜻',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallEta => '𝜼',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallTheta => '𝜽',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallIota => '𝜾',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallKappa => '𝜿',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallLamda => '𝝀',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallMu => '𝝁',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallNu => '𝝂',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallXi => '𝝃',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallOmicron => '𝝄',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallPi => '𝝅',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallRho => '𝝆',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallFinalSigma => '𝝇',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallSigma => '𝝈',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallTau => '𝝉',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallUpsilon => '𝝊',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallPhi => '𝝋',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallChi => '𝝌',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallPsi => '𝝍',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallOmega => '𝝎',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicPartialDifferential => '𝝏',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicEpsilonSymbol => '𝝐',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicThetaSymbol => '𝝑',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicKappaSymbol => '𝝒',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicPhiSymbol => '𝝓',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicRhoSymbol => '𝝔',
            MathematicalAlphanumericSymbols::MathematicalBoldItalicPiSymbol => '𝝕',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalAlpha => '𝝖',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalBeta => '𝝗',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalGamma => '𝝘',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalDelta => '𝝙',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalEpsilon => '𝝚',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalZeta => '𝝛',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalEta => '𝝜',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalTheta => '𝝝',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalIota => '𝝞',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalKappa => '𝝟',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalLamda => '𝝠',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalMu => '𝝡',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalNu => '𝝢',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalXi => '𝝣',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalOmicron => '𝝤',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalPi => '𝝥',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalRho => '𝝦',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalThetaSymbol => '𝝧',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalSigma => '𝝨',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalTau => '𝝩',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalUpsilon => '𝝪',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalPhi => '𝝫',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalChi => '𝝬',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalPsi => '𝝭',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalOmega => '𝝮',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldNabla => '𝝯',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallAlpha => '𝝰',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallBeta => '𝝱',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallGamma => '𝝲',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallDelta => '𝝳',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallEpsilon => '𝝴',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallZeta => '𝝵',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallEta => '𝝶',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallTheta => '𝝷',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallIota => '𝝸',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallKappa => '𝝹',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallLamda => '𝝺',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallMu => '𝝻',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallNu => '𝝼',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallXi => '𝝽',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallOmicron => '𝝾',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallPi => '𝝿',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallRho => '𝞀',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallFinalSigma => '𝞁',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallSigma => '𝞂',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallTau => '𝞃',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallUpsilon => '𝞄',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallPhi => '𝞅',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallChi => '𝞆',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallPsi => '𝞇',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallOmega => '𝞈',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldPartialDifferential => '𝞉',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldEpsilonSymbol => '𝞊',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldThetaSymbol => '𝞋',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldKappaSymbol => '𝞌',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldPhiSymbol => '𝞍',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldRhoSymbol => '𝞎',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldPiSymbol => '𝞏',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalAlpha => '𝞐',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalBeta => '𝞑',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalGamma => '𝞒',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalDelta => '𝞓',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalEpsilon => '𝞔',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalZeta => '𝞕',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalEta => '𝞖',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalTheta => '𝞗',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalIota => '𝞘',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalKappa => '𝞙',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalLamda => '𝞚',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalMu => '𝞛',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalNu => '𝞜',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalXi => '𝞝',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalOmicron => '𝞞',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalPi => '𝞟',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalRho => '𝞠',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalThetaSymbol => '𝞡',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalSigma => '𝞢',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalTau => '𝞣',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalUpsilon => '𝞤',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalPhi => '𝞥',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalChi => '𝞦',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalPsi => '𝞧',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalOmega => '𝞨',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicNabla => '𝞩',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallAlpha => '𝞪',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallBeta => '𝞫',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallGamma => '𝞬',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallDelta => '𝞭',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallEpsilon => '𝞮',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallZeta => '𝞯',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallEta => '𝞰',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallTheta => '𝞱',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallIota => '𝞲',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallKappa => '𝞳',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallLamda => '𝞴',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallMu => '𝞵',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallNu => '𝞶',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallXi => '𝞷',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallOmicron => '𝞸',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallPi => '𝞹',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallRho => '𝞺',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallFinalSigma => '𝞻',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallSigma => '𝞼',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallTau => '𝞽',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallUpsilon => '𝞾',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallPhi => '𝞿',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallChi => '𝟀',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallPsi => '𝟁',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallOmega => '𝟂',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicPartialDifferential => '𝟃',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicEpsilonSymbol => '𝟄',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicThetaSymbol => '𝟅',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicKappaSymbol => '𝟆',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicPhiSymbol => '𝟇',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicRhoSymbol => '𝟈',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicPiSymbol => '𝟉',
            MathematicalAlphanumericSymbols::MathematicalBoldCapitalDigamma => '𝟊',
            MathematicalAlphanumericSymbols::MathematicalBoldSmallDigamma => '𝟋',
            MathematicalAlphanumericSymbols::MathematicalBoldDigitZero => '𝟎',
            MathematicalAlphanumericSymbols::MathematicalBoldDigitOne => '𝟏',
            MathematicalAlphanumericSymbols::MathematicalBoldDigitTwo => '𝟐',
            MathematicalAlphanumericSymbols::MathematicalBoldDigitThree => '𝟑',
            MathematicalAlphanumericSymbols::MathematicalBoldDigitFour => '𝟒',
            MathematicalAlphanumericSymbols::MathematicalBoldDigitFive => '𝟓',
            MathematicalAlphanumericSymbols::MathematicalBoldDigitSix => '𝟔',
            MathematicalAlphanumericSymbols::MathematicalBoldDigitSeven => '𝟕',
            MathematicalAlphanumericSymbols::MathematicalBoldDigitEight => '𝟖',
            MathematicalAlphanumericSymbols::MathematicalBoldDigitNine => '𝟗',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckDigitZero => '𝟘',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckDigitOne => '𝟙',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckDigitTwo => '𝟚',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckDigitThree => '𝟛',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckDigitFour => '𝟜',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckDigitFive => '𝟝',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckDigitSix => '𝟞',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckDigitSeven => '𝟟',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckDigitEight => '𝟠',
            MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckDigitNine => '𝟡',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifDigitZero => '𝟢',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifDigitOne => '𝟣',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifDigitTwo => '𝟤',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifDigitThree => '𝟥',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifDigitFour => '𝟦',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifDigitFive => '𝟧',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifDigitSix => '𝟨',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifDigitSeven => '𝟩',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifDigitEight => '𝟪',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifDigitNine => '𝟫',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldDigitZero => '𝟬',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldDigitOne => '𝟭',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldDigitTwo => '𝟮',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldDigitThree => '𝟯',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldDigitFour => '𝟰',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldDigitFive => '𝟱',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldDigitSix => '𝟲',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldDigitSeven => '𝟳',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldDigitEight => '𝟴',
            MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldDigitNine => '𝟵',
            MathematicalAlphanumericSymbols::MathematicalMonospaceDigitZero => '𝟶',
            MathematicalAlphanumericSymbols::MathematicalMonospaceDigitOne => '𝟷',
            MathematicalAlphanumericSymbols::MathematicalMonospaceDigitTwo => '𝟸',
            MathematicalAlphanumericSymbols::MathematicalMonospaceDigitThree => '𝟹',
            MathematicalAlphanumericSymbols::MathematicalMonospaceDigitFour => '𝟺',
            MathematicalAlphanumericSymbols::MathematicalMonospaceDigitFive => '𝟻',
            MathematicalAlphanumericSymbols::MathematicalMonospaceDigitSix => '𝟼',
            MathematicalAlphanumericSymbols::MathematicalMonospaceDigitSeven => '𝟽',
            MathematicalAlphanumericSymbols::MathematicalMonospaceDigitEight => '𝟾',
        }
    }
}

impl std::convert::TryFrom<char> for MathematicalAlphanumericSymbols {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𝐀' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalA),
            '𝐁' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalB),
            '𝐂' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalC),
            '𝐃' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalD),
            '𝐄' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalE),
            '𝐅' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalF),
            '𝐆' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalG),
            '𝐇' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalH),
            '𝐈' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalI),
            '𝐉' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalJ),
            '𝐊' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalK),
            '𝐋' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalL),
            '𝐌' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalM),
            '𝐍' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalN),
            '𝐎' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalO),
            '𝐏' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalP),
            '𝐐' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalQ),
            '𝐑' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalR),
            '𝐒' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalS),
            '𝐓' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalT),
            '𝐔' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalU),
            '𝐕' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalV),
            '𝐖' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalW),
            '𝐗' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalX),
            '𝐘' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalY),
            '𝐙' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalZ),
            '𝐚' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallA),
            '𝐛' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallB),
            '𝐜' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallC),
            '𝐝' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallD),
            '𝐞' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallE),
            '𝐟' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallF),
            '𝐠' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallG),
            '𝐡' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallH),
            '𝐢' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallI),
            '𝐣' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallJ),
            '𝐤' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallK),
            '𝐥' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallL),
            '𝐦' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallM),
            '𝐧' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallN),
            '𝐨' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallO),
            '𝐩' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallP),
            '𝐪' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallQ),
            '𝐫' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallR),
            '𝐬' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallS),
            '𝐭' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallT),
            '𝐮' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallU),
            '𝐯' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallV),
            '𝐰' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallW),
            '𝐱' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallX),
            '𝐲' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallY),
            '𝐳' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallZ),
            '𝐴' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicCapitalA),
            '𝐵' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicCapitalB),
            '𝐶' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicCapitalC),
            '𝐷' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicCapitalD),
            '𝐸' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicCapitalE),
            '𝐹' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicCapitalF),
            '𝐺' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicCapitalG),
            '𝐻' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicCapitalH),
            '𝐼' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicCapitalI),
            '𝐽' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicCapitalJ),
            '𝐾' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicCapitalK),
            '𝐿' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicCapitalL),
            '𝑀' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicCapitalM),
            '𝑁' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicCapitalN),
            '𝑂' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicCapitalO),
            '𝑃' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicCapitalP),
            '𝑄' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicCapitalQ),
            '𝑅' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicCapitalR),
            '𝑆' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicCapitalS),
            '𝑇' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicCapitalT),
            '𝑈' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicCapitalU),
            '𝑉' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicCapitalV),
            '𝑊' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicCapitalW),
            '𝑋' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicCapitalX),
            '𝑌' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicCapitalY),
            '𝑍' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicCapitalZ),
            '𝑎' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallA),
            '𝑏' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallB),
            '𝑐' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallC),
            '𝑑' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallD),
            '𝑒' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallE),
            '𝑓' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallF),
            '𝑔' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallG),
            '𝑖' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallI),
            '𝑗' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallJ),
            '𝑘' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallK),
            '𝑙' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallL),
            '𝑚' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallM),
            '𝑛' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallN),
            '𝑜' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallO),
            '𝑝' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallP),
            '𝑞' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallQ),
            '𝑟' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallR),
            '𝑠' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallS),
            '𝑡' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallT),
            '𝑢' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallU),
            '𝑣' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallV),
            '𝑤' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallW),
            '𝑥' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallX),
            '𝑦' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallY),
            '𝑧' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallZ),
            '𝑨' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalA),
            '𝑩' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalB),
            '𝑪' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalC),
            '𝑫' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalD),
            '𝑬' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalE),
            '𝑭' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalF),
            '𝑮' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalG),
            '𝑯' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalH),
            '𝑰' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalI),
            '𝑱' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalJ),
            '𝑲' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalK),
            '𝑳' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalL),
            '𝑴' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalM),
            '𝑵' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalN),
            '𝑶' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalO),
            '𝑷' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalP),
            '𝑸' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalQ),
            '𝑹' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalR),
            '𝑺' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalS),
            '𝑻' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalT),
            '𝑼' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalU),
            '𝑽' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalV),
            '𝑾' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalW),
            '𝑿' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalX),
            '𝒀' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalY),
            '𝒁' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalZ),
            '𝒂' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallA),
            '𝒃' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallB),
            '𝒄' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallC),
            '𝒅' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallD),
            '𝒆' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallE),
            '𝒇' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallF),
            '𝒈' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallG),
            '𝒉' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallH),
            '𝒊' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallI),
            '𝒋' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallJ),
            '𝒌' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallK),
            '𝒍' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallL),
            '𝒎' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallM),
            '𝒏' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallN),
            '𝒐' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallO),
            '𝒑' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallP),
            '𝒒' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallQ),
            '𝒓' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallR),
            '𝒔' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallS),
            '𝒕' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallT),
            '𝒖' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallU),
            '𝒗' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallV),
            '𝒘' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallW),
            '𝒙' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallX),
            '𝒚' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallY),
            '𝒛' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallZ),
            '𝒜' => Ok(MathematicalAlphanumericSymbols::MathematicalScriptCapitalA),
            '𝒞' => Ok(MathematicalAlphanumericSymbols::MathematicalScriptCapitalC),
            '𝒟' => Ok(MathematicalAlphanumericSymbols::MathematicalScriptCapitalD),
            '𝒢' => Ok(MathematicalAlphanumericSymbols::MathematicalScriptCapitalG),
            '𝒥' => Ok(MathematicalAlphanumericSymbols::MathematicalScriptCapitalJ),
            '𝒦' => Ok(MathematicalAlphanumericSymbols::MathematicalScriptCapitalK),
            '𝒩' => Ok(MathematicalAlphanumericSymbols::MathematicalScriptCapitalN),
            '𝒪' => Ok(MathematicalAlphanumericSymbols::MathematicalScriptCapitalO),
            '𝒫' => Ok(MathematicalAlphanumericSymbols::MathematicalScriptCapitalP),
            '𝒬' => Ok(MathematicalAlphanumericSymbols::MathematicalScriptCapitalQ),
            '𝒮' => Ok(MathematicalAlphanumericSymbols::MathematicalScriptCapitalS),
            '𝒯' => Ok(MathematicalAlphanumericSymbols::MathematicalScriptCapitalT),
            '𝒰' => Ok(MathematicalAlphanumericSymbols::MathematicalScriptCapitalU),
            '𝒱' => Ok(MathematicalAlphanumericSymbols::MathematicalScriptCapitalV),
            '𝒲' => Ok(MathematicalAlphanumericSymbols::MathematicalScriptCapitalW),
            '𝒳' => Ok(MathematicalAlphanumericSymbols::MathematicalScriptCapitalX),
            '𝒴' => Ok(MathematicalAlphanumericSymbols::MathematicalScriptCapitalY),
            '𝒵' => Ok(MathematicalAlphanumericSymbols::MathematicalScriptCapitalZ),
            '𝒶' => Ok(MathematicalAlphanumericSymbols::MathematicalScriptSmallA),
            '𝒷' => Ok(MathematicalAlphanumericSymbols::MathematicalScriptSmallB),
            '𝒸' => Ok(MathematicalAlphanumericSymbols::MathematicalScriptSmallC),
            '𝒹' => Ok(MathematicalAlphanumericSymbols::MathematicalScriptSmallD),
            '𝒻' => Ok(MathematicalAlphanumericSymbols::MathematicalScriptSmallF),
            '𝒽' => Ok(MathematicalAlphanumericSymbols::MathematicalScriptSmallH),
            '𝒾' => Ok(MathematicalAlphanumericSymbols::MathematicalScriptSmallI),
            '𝒿' => Ok(MathematicalAlphanumericSymbols::MathematicalScriptSmallJ),
            '𝓀' => Ok(MathematicalAlphanumericSymbols::MathematicalScriptSmallK),
            '𝓁' => Ok(MathematicalAlphanumericSymbols::MathematicalScriptSmallL),
            '𝓂' => Ok(MathematicalAlphanumericSymbols::MathematicalScriptSmallM),
            '𝓃' => Ok(MathematicalAlphanumericSymbols::MathematicalScriptSmallN),
            '𝓅' => Ok(MathematicalAlphanumericSymbols::MathematicalScriptSmallP),
            '𝓆' => Ok(MathematicalAlphanumericSymbols::MathematicalScriptSmallQ),
            '𝓇' => Ok(MathematicalAlphanumericSymbols::MathematicalScriptSmallR),
            '𝓈' => Ok(MathematicalAlphanumericSymbols::MathematicalScriptSmallS),
            '𝓉' => Ok(MathematicalAlphanumericSymbols::MathematicalScriptSmallT),
            '𝓊' => Ok(MathematicalAlphanumericSymbols::MathematicalScriptSmallU),
            '𝓋' => Ok(MathematicalAlphanumericSymbols::MathematicalScriptSmallV),
            '𝓌' => Ok(MathematicalAlphanumericSymbols::MathematicalScriptSmallW),
            '𝓍' => Ok(MathematicalAlphanumericSymbols::MathematicalScriptSmallX),
            '𝓎' => Ok(MathematicalAlphanumericSymbols::MathematicalScriptSmallY),
            '𝓏' => Ok(MathematicalAlphanumericSymbols::MathematicalScriptSmallZ),
            '𝓐' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalA),
            '𝓑' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalB),
            '𝓒' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalC),
            '𝓓' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalD),
            '𝓔' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalE),
            '𝓕' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalF),
            '𝓖' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalG),
            '𝓗' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalH),
            '𝓘' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalI),
            '𝓙' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalJ),
            '𝓚' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalK),
            '𝓛' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalL),
            '𝓜' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalM),
            '𝓝' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalN),
            '𝓞' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalO),
            '𝓟' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalP),
            '𝓠' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalQ),
            '𝓡' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalR),
            '𝓢' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalS),
            '𝓣' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalT),
            '𝓤' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalU),
            '𝓥' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalV),
            '𝓦' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalW),
            '𝓧' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalX),
            '𝓨' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalY),
            '𝓩' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptCapitalZ),
            '𝓪' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallA),
            '𝓫' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallB),
            '𝓬' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallC),
            '𝓭' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallD),
            '𝓮' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallE),
            '𝓯' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallF),
            '𝓰' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallG),
            '𝓱' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallH),
            '𝓲' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallI),
            '𝓳' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallJ),
            '𝓴' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallK),
            '𝓵' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallL),
            '𝓶' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallM),
            '𝓷' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallN),
            '𝓸' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallO),
            '𝓹' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallP),
            '𝓺' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallQ),
            '𝓻' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallR),
            '𝓼' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallS),
            '𝓽' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallT),
            '𝓾' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallU),
            '𝓿' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallV),
            '𝔀' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallW),
            '𝔁' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallX),
            '𝔂' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallY),
            '𝔃' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldScriptSmallZ),
            '𝔄' => Ok(MathematicalAlphanumericSymbols::MathematicalFrakturCapitalA),
            '𝔅' => Ok(MathematicalAlphanumericSymbols::MathematicalFrakturCapitalB),
            '𝔇' => Ok(MathematicalAlphanumericSymbols::MathematicalFrakturCapitalD),
            '𝔈' => Ok(MathematicalAlphanumericSymbols::MathematicalFrakturCapitalE),
            '𝔉' => Ok(MathematicalAlphanumericSymbols::MathematicalFrakturCapitalF),
            '𝔊' => Ok(MathematicalAlphanumericSymbols::MathematicalFrakturCapitalG),
            '𝔍' => Ok(MathematicalAlphanumericSymbols::MathematicalFrakturCapitalJ),
            '𝔎' => Ok(MathematicalAlphanumericSymbols::MathematicalFrakturCapitalK),
            '𝔏' => Ok(MathematicalAlphanumericSymbols::MathematicalFrakturCapitalL),
            '𝔐' => Ok(MathematicalAlphanumericSymbols::MathematicalFrakturCapitalM),
            '𝔑' => Ok(MathematicalAlphanumericSymbols::MathematicalFrakturCapitalN),
            '𝔒' => Ok(MathematicalAlphanumericSymbols::MathematicalFrakturCapitalO),
            '𝔓' => Ok(MathematicalAlphanumericSymbols::MathematicalFrakturCapitalP),
            '𝔔' => Ok(MathematicalAlphanumericSymbols::MathematicalFrakturCapitalQ),
            '𝔖' => Ok(MathematicalAlphanumericSymbols::MathematicalFrakturCapitalS),
            '𝔗' => Ok(MathematicalAlphanumericSymbols::MathematicalFrakturCapitalT),
            '𝔘' => Ok(MathematicalAlphanumericSymbols::MathematicalFrakturCapitalU),
            '𝔙' => Ok(MathematicalAlphanumericSymbols::MathematicalFrakturCapitalV),
            '𝔚' => Ok(MathematicalAlphanumericSymbols::MathematicalFrakturCapitalW),
            '𝔛' => Ok(MathematicalAlphanumericSymbols::MathematicalFrakturCapitalX),
            '𝔜' => Ok(MathematicalAlphanumericSymbols::MathematicalFrakturCapitalY),
            '𝔞' => Ok(MathematicalAlphanumericSymbols::MathematicalFrakturSmallA),
            '𝔟' => Ok(MathematicalAlphanumericSymbols::MathematicalFrakturSmallB),
            '𝔠' => Ok(MathematicalAlphanumericSymbols::MathematicalFrakturSmallC),
            '𝔡' => Ok(MathematicalAlphanumericSymbols::MathematicalFrakturSmallD),
            '𝔢' => Ok(MathematicalAlphanumericSymbols::MathematicalFrakturSmallE),
            '𝔣' => Ok(MathematicalAlphanumericSymbols::MathematicalFrakturSmallF),
            '𝔤' => Ok(MathematicalAlphanumericSymbols::MathematicalFrakturSmallG),
            '𝔥' => Ok(MathematicalAlphanumericSymbols::MathematicalFrakturSmallH),
            '𝔦' => Ok(MathematicalAlphanumericSymbols::MathematicalFrakturSmallI),
            '𝔧' => Ok(MathematicalAlphanumericSymbols::MathematicalFrakturSmallJ),
            '𝔨' => Ok(MathematicalAlphanumericSymbols::MathematicalFrakturSmallK),
            '𝔩' => Ok(MathematicalAlphanumericSymbols::MathematicalFrakturSmallL),
            '𝔪' => Ok(MathematicalAlphanumericSymbols::MathematicalFrakturSmallM),
            '𝔫' => Ok(MathematicalAlphanumericSymbols::MathematicalFrakturSmallN),
            '𝔬' => Ok(MathematicalAlphanumericSymbols::MathematicalFrakturSmallO),
            '𝔭' => Ok(MathematicalAlphanumericSymbols::MathematicalFrakturSmallP),
            '𝔮' => Ok(MathematicalAlphanumericSymbols::MathematicalFrakturSmallQ),
            '𝔯' => Ok(MathematicalAlphanumericSymbols::MathematicalFrakturSmallR),
            '𝔰' => Ok(MathematicalAlphanumericSymbols::MathematicalFrakturSmallS),
            '𝔱' => Ok(MathematicalAlphanumericSymbols::MathematicalFrakturSmallT),
            '𝔲' => Ok(MathematicalAlphanumericSymbols::MathematicalFrakturSmallU),
            '𝔳' => Ok(MathematicalAlphanumericSymbols::MathematicalFrakturSmallV),
            '𝔴' => Ok(MathematicalAlphanumericSymbols::MathematicalFrakturSmallW),
            '𝔵' => Ok(MathematicalAlphanumericSymbols::MathematicalFrakturSmallX),
            '𝔶' => Ok(MathematicalAlphanumericSymbols::MathematicalFrakturSmallY),
            '𝔷' => Ok(MathematicalAlphanumericSymbols::MathematicalFrakturSmallZ),
            '𝔸' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckCapitalA),
            '𝔹' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckCapitalB),
            '𝔻' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckCapitalD),
            '𝔼' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckCapitalE),
            '𝔽' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckCapitalF),
            '𝔾' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckCapitalG),
            '𝕀' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckCapitalI),
            '𝕁' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckCapitalJ),
            '𝕂' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckCapitalK),
            '𝕃' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckCapitalL),
            '𝕄' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckCapitalM),
            '𝕆' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckCapitalO),
            '𝕊' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckCapitalS),
            '𝕋' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckCapitalT),
            '𝕌' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckCapitalU),
            '𝕍' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckCapitalV),
            '𝕎' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckCapitalW),
            '𝕏' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckCapitalX),
            '𝕐' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckCapitalY),
            '𝕒' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallA),
            '𝕓' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallB),
            '𝕔' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallC),
            '𝕕' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallD),
            '𝕖' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallE),
            '𝕗' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallF),
            '𝕘' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallG),
            '𝕙' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallH),
            '𝕚' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallI),
            '𝕛' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallJ),
            '𝕜' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallK),
            '𝕝' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallL),
            '𝕞' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallM),
            '𝕟' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallN),
            '𝕠' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallO),
            '𝕡' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallP),
            '𝕢' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallQ),
            '𝕣' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallR),
            '𝕤' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallS),
            '𝕥' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallT),
            '𝕦' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallU),
            '𝕧' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallV),
            '𝕨' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallW),
            '𝕩' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallX),
            '𝕪' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallY),
            '𝕫' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckSmallZ),
            '𝕬' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalA),
            '𝕭' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalB),
            '𝕮' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalC),
            '𝕯' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalD),
            '𝕰' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalE),
            '𝕱' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalF),
            '𝕲' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalG),
            '𝕳' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalH),
            '𝕴' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalI),
            '𝕵' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalJ),
            '𝕶' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalK),
            '𝕷' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalL),
            '𝕸' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalM),
            '𝕹' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalN),
            '𝕺' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalO),
            '𝕻' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalP),
            '𝕼' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalQ),
            '𝕽' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalR),
            '𝕾' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalS),
            '𝕿' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalT),
            '𝖀' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalU),
            '𝖁' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalV),
            '𝖂' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalW),
            '𝖃' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalX),
            '𝖄' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalY),
            '𝖅' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturCapitalZ),
            '𝖆' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallA),
            '𝖇' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallB),
            '𝖈' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallC),
            '𝖉' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallD),
            '𝖊' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallE),
            '𝖋' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallF),
            '𝖌' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallG),
            '𝖍' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallH),
            '𝖎' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallI),
            '𝖏' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallJ),
            '𝖐' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallK),
            '𝖑' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallL),
            '𝖒' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallM),
            '𝖓' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallN),
            '𝖔' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallO),
            '𝖕' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallP),
            '𝖖' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallQ),
            '𝖗' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallR),
            '𝖘' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallS),
            '𝖙' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallT),
            '𝖚' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallU),
            '𝖛' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallV),
            '𝖜' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallW),
            '𝖝' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallX),
            '𝖞' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallY),
            '𝖟' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldFrakturSmallZ),
            '𝖠' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalA),
            '𝖡' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalB),
            '𝖢' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalC),
            '𝖣' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalD),
            '𝖤' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalE),
            '𝖥' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalF),
            '𝖦' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalG),
            '𝖧' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalH),
            '𝖨' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalI),
            '𝖩' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalJ),
            '𝖪' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalK),
            '𝖫' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalL),
            '𝖬' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalM),
            '𝖭' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalN),
            '𝖮' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalO),
            '𝖯' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalP),
            '𝖰' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalQ),
            '𝖱' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalR),
            '𝖲' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalS),
            '𝖳' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalT),
            '𝖴' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalU),
            '𝖵' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalV),
            '𝖶' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalW),
            '𝖷' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalX),
            '𝖸' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalY),
            '𝖹' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifCapitalZ),
            '𝖺' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallA),
            '𝖻' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallB),
            '𝖼' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallC),
            '𝖽' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallD),
            '𝖾' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallE),
            '𝖿' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallF),
            '𝗀' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallG),
            '𝗁' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallH),
            '𝗂' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallI),
            '𝗃' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallJ),
            '𝗄' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallK),
            '𝗅' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallL),
            '𝗆' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallM),
            '𝗇' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallN),
            '𝗈' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallO),
            '𝗉' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallP),
            '𝗊' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallQ),
            '𝗋' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallR),
            '𝗌' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallS),
            '𝗍' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallT),
            '𝗎' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallU),
            '𝗏' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallV),
            '𝗐' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallW),
            '𝗑' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallX),
            '𝗒' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallY),
            '𝗓' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifSmallZ),
            '𝗔' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalA),
            '𝗕' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalB),
            '𝗖' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalC),
            '𝗗' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalD),
            '𝗘' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalE),
            '𝗙' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalF),
            '𝗚' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalG),
            '𝗛' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalH),
            '𝗜' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalI),
            '𝗝' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalJ),
            '𝗞' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalK),
            '𝗟' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalL),
            '𝗠' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalM),
            '𝗡' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalN),
            '𝗢' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalO),
            '𝗣' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalP),
            '𝗤' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalQ),
            '𝗥' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalR),
            '𝗦' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalS),
            '𝗧' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalT),
            '𝗨' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalU),
            '𝗩' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalV),
            '𝗪' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalW),
            '𝗫' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalX),
            '𝗬' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalY),
            '𝗭' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalZ),
            '𝗮' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallA),
            '𝗯' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallB),
            '𝗰' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallC),
            '𝗱' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallD),
            '𝗲' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallE),
            '𝗳' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallF),
            '𝗴' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallG),
            '𝗵' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallH),
            '𝗶' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallI),
            '𝗷' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallJ),
            '𝗸' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallK),
            '𝗹' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallL),
            '𝗺' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallM),
            '𝗻' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallN),
            '𝗼' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallO),
            '𝗽' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallP),
            '𝗾' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallQ),
            '𝗿' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallR),
            '𝘀' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallS),
            '𝘁' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallT),
            '𝘂' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallU),
            '𝘃' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallV),
            '𝘄' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallW),
            '𝘅' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallX),
            '𝘆' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallY),
            '𝘇' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallZ),
            '𝘈' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalA),
            '𝘉' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalB),
            '𝘊' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalC),
            '𝘋' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalD),
            '𝘌' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalE),
            '𝘍' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalF),
            '𝘎' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalG),
            '𝘏' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalH),
            '𝘐' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalI),
            '𝘑' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalJ),
            '𝘒' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalK),
            '𝘓' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalL),
            '𝘔' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalM),
            '𝘕' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalN),
            '𝘖' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalO),
            '𝘗' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalP),
            '𝘘' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalQ),
            '𝘙' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalR),
            '𝘚' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalS),
            '𝘛' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalT),
            '𝘜' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalU),
            '𝘝' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalV),
            '𝘞' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalW),
            '𝘟' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalX),
            '𝘠' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalY),
            '𝘡' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicCapitalZ),
            '𝘢' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallA),
            '𝘣' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallB),
            '𝘤' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallC),
            '𝘥' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallD),
            '𝘦' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallE),
            '𝘧' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallF),
            '𝘨' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallG),
            '𝘩' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallH),
            '𝘪' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallI),
            '𝘫' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallJ),
            '𝘬' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallK),
            '𝘭' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallL),
            '𝘮' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallM),
            '𝘯' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallN),
            '𝘰' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallO),
            '𝘱' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallP),
            '𝘲' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallQ),
            '𝘳' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallR),
            '𝘴' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallS),
            '𝘵' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallT),
            '𝘶' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallU),
            '𝘷' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallV),
            '𝘸' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallW),
            '𝘹' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallX),
            '𝘺' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallY),
            '𝘻' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifItalicSmallZ),
            '𝘼' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalA),
            '𝘽' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalB),
            '𝘾' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalC),
            '𝘿' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalD),
            '𝙀' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalE),
            '𝙁' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalF),
            '𝙂' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalG),
            '𝙃' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalH),
            '𝙄' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalI),
            '𝙅' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalJ),
            '𝙆' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalK),
            '𝙇' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalL),
            '𝙈' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalM),
            '𝙉' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalN),
            '𝙊' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalO),
            '𝙋' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalP),
            '𝙌' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalQ),
            '𝙍' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalR),
            '𝙎' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalS),
            '𝙏' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalT),
            '𝙐' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalU),
            '𝙑' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalV),
            '𝙒' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalW),
            '𝙓' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalX),
            '𝙔' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalY),
            '𝙕' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalZ),
            '𝙖' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallA),
            '𝙗' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallB),
            '𝙘' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallC),
            '𝙙' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallD),
            '𝙚' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallE),
            '𝙛' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallF),
            '𝙜' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallG),
            '𝙝' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallH),
            '𝙞' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallI),
            '𝙟' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallJ),
            '𝙠' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallK),
            '𝙡' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallL),
            '𝙢' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallM),
            '𝙣' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallN),
            '𝙤' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallO),
            '𝙥' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallP),
            '𝙦' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallQ),
            '𝙧' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallR),
            '𝙨' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallS),
            '𝙩' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallT),
            '𝙪' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallU),
            '𝙫' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallV),
            '𝙬' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallW),
            '𝙭' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallX),
            '𝙮' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallY),
            '𝙯' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallZ),
            '𝙰' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalA),
            '𝙱' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalB),
            '𝙲' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalC),
            '𝙳' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalD),
            '𝙴' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalE),
            '𝙵' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalF),
            '𝙶' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalG),
            '𝙷' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalH),
            '𝙸' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalI),
            '𝙹' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalJ),
            '𝙺' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalK),
            '𝙻' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalL),
            '𝙼' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalM),
            '𝙽' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalN),
            '𝙾' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalO),
            '𝙿' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalP),
            '𝚀' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalQ),
            '𝚁' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalR),
            '𝚂' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalS),
            '𝚃' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalT),
            '𝚄' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalU),
            '𝚅' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalV),
            '𝚆' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalW),
            '𝚇' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalX),
            '𝚈' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalY),
            '𝚉' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceCapitalZ),
            '𝚊' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceSmallA),
            '𝚋' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceSmallB),
            '𝚌' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceSmallC),
            '𝚍' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceSmallD),
            '𝚎' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceSmallE),
            '𝚏' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceSmallF),
            '𝚐' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceSmallG),
            '𝚑' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceSmallH),
            '𝚒' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceSmallI),
            '𝚓' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceSmallJ),
            '𝚔' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceSmallK),
            '𝚕' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceSmallL),
            '𝚖' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceSmallM),
            '𝚗' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceSmallN),
            '𝚘' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceSmallO),
            '𝚙' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceSmallP),
            '𝚚' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceSmallQ),
            '𝚛' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceSmallR),
            '𝚜' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceSmallS),
            '𝚝' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceSmallT),
            '𝚞' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceSmallU),
            '𝚟' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceSmallV),
            '𝚠' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceSmallW),
            '𝚡' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceSmallX),
            '𝚢' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceSmallY),
            '𝚣' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceSmallZ),
            '𝚤' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallDotlessI),
            '𝚥' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallDotlessJ),
            '𝚨' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalAlpha),
            '𝚩' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalBeta),
            '𝚪' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalGamma),
            '𝚫' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalDelta),
            '𝚬' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalEpsilon),
            '𝚭' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalZeta),
            '𝚮' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalEta),
            '𝚯' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalTheta),
            '𝚰' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalIota),
            '𝚱' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalKappa),
            '𝚲' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalLamda),
            '𝚳' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalMu),
            '𝚴' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalNu),
            '𝚵' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalXi),
            '𝚶' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalOmicron),
            '𝚷' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalPi),
            '𝚸' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalRho),
            '𝚹' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalThetaSymbol),
            '𝚺' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalSigma),
            '𝚻' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalTau),
            '𝚼' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalUpsilon),
            '𝚽' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalPhi),
            '𝚾' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalChi),
            '𝚿' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalPsi),
            '𝛀' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalOmega),
            '𝛁' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldNabla),
            '𝛂' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallAlpha),
            '𝛃' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallBeta),
            '𝛄' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallGamma),
            '𝛅' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallDelta),
            '𝛆' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallEpsilon),
            '𝛇' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallZeta),
            '𝛈' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallEta),
            '𝛉' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallTheta),
            '𝛊' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallIota),
            '𝛋' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallKappa),
            '𝛌' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallLamda),
            '𝛍' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallMu),
            '𝛎' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallNu),
            '𝛏' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallXi),
            '𝛐' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallOmicron),
            '𝛑' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallPi),
            '𝛒' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallRho),
            '𝛓' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallFinalSigma),
            '𝛔' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallSigma),
            '𝛕' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallTau),
            '𝛖' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallUpsilon),
            '𝛗' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallPhi),
            '𝛘' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallChi),
            '𝛙' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallPsi),
            '𝛚' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallOmega),
            '𝛛' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldPartialDifferential),
            '𝛜' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldEpsilonSymbol),
            '𝛝' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldThetaSymbol),
            '𝛞' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldKappaSymbol),
            '𝛟' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldPhiSymbol),
            '𝛠' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldRhoSymbol),
            '𝛡' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldPiSymbol),
            '𝛢' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicCapitalAlpha),
            '𝛣' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicCapitalBeta),
            '𝛤' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicCapitalGamma),
            '𝛥' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicCapitalDelta),
            '𝛦' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicCapitalEpsilon),
            '𝛧' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicCapitalZeta),
            '𝛨' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicCapitalEta),
            '𝛩' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicCapitalTheta),
            '𝛪' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicCapitalIota),
            '𝛫' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicCapitalKappa),
            '𝛬' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicCapitalLamda),
            '𝛭' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicCapitalMu),
            '𝛮' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicCapitalNu),
            '𝛯' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicCapitalXi),
            '𝛰' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicCapitalOmicron),
            '𝛱' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicCapitalPi),
            '𝛲' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicCapitalRho),
            '𝛳' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicCapitalThetaSymbol),
            '𝛴' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicCapitalSigma),
            '𝛵' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicCapitalTau),
            '𝛶' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicCapitalUpsilon),
            '𝛷' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicCapitalPhi),
            '𝛸' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicCapitalChi),
            '𝛹' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicCapitalPsi),
            '𝛺' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicCapitalOmega),
            '𝛻' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicNabla),
            '𝛼' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallAlpha),
            '𝛽' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallBeta),
            '𝛾' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallGamma),
            '𝛿' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallDelta),
            '𝜀' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallEpsilon),
            '𝜁' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallZeta),
            '𝜂' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallEta),
            '𝜃' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallTheta),
            '𝜄' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallIota),
            '𝜅' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallKappa),
            '𝜆' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallLamda),
            '𝜇' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallMu),
            '𝜈' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallNu),
            '𝜉' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallXi),
            '𝜊' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallOmicron),
            '𝜋' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallPi),
            '𝜌' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallRho),
            '𝜍' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallFinalSigma),
            '𝜎' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallSigma),
            '𝜏' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallTau),
            '𝜐' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallUpsilon),
            '𝜑' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallPhi),
            '𝜒' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallChi),
            '𝜓' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallPsi),
            '𝜔' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicSmallOmega),
            '𝜕' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicPartialDifferential),
            '𝜖' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicEpsilonSymbol),
            '𝜗' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicThetaSymbol),
            '𝜘' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicKappaSymbol),
            '𝜙' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicPhiSymbol),
            '𝜚' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicRhoSymbol),
            '𝜛' => Ok(MathematicalAlphanumericSymbols::MathematicalItalicPiSymbol),
            '𝜜' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalAlpha),
            '𝜝' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalBeta),
            '𝜞' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalGamma),
            '𝜟' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalDelta),
            '𝜠' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalEpsilon),
            '𝜡' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalZeta),
            '𝜢' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalEta),
            '𝜣' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalTheta),
            '𝜤' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalIota),
            '𝜥' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalKappa),
            '𝜦' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalLamda),
            '𝜧' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalMu),
            '𝜨' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalNu),
            '𝜩' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalXi),
            '𝜪' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalOmicron),
            '𝜫' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalPi),
            '𝜬' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalRho),
            '𝜭' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalThetaSymbol),
            '𝜮' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalSigma),
            '𝜯' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalTau),
            '𝜰' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalUpsilon),
            '𝜱' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalPhi),
            '𝜲' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalChi),
            '𝜳' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalPsi),
            '𝜴' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicCapitalOmega),
            '𝜵' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicNabla),
            '𝜶' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallAlpha),
            '𝜷' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallBeta),
            '𝜸' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallGamma),
            '𝜹' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallDelta),
            '𝜺' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallEpsilon),
            '𝜻' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallZeta),
            '𝜼' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallEta),
            '𝜽' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallTheta),
            '𝜾' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallIota),
            '𝜿' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallKappa),
            '𝝀' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallLamda),
            '𝝁' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallMu),
            '𝝂' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallNu),
            '𝝃' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallXi),
            '𝝄' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallOmicron),
            '𝝅' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallPi),
            '𝝆' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallRho),
            '𝝇' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallFinalSigma),
            '𝝈' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallSigma),
            '𝝉' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallTau),
            '𝝊' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallUpsilon),
            '𝝋' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallPhi),
            '𝝌' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallChi),
            '𝝍' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallPsi),
            '𝝎' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicSmallOmega),
            '𝝏' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicPartialDifferential),
            '𝝐' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicEpsilonSymbol),
            '𝝑' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicThetaSymbol),
            '𝝒' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicKappaSymbol),
            '𝝓' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicPhiSymbol),
            '𝝔' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicRhoSymbol),
            '𝝕' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldItalicPiSymbol),
            '𝝖' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalAlpha),
            '𝝗' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalBeta),
            '𝝘' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalGamma),
            '𝝙' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalDelta),
            '𝝚' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalEpsilon),
            '𝝛' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalZeta),
            '𝝜' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalEta),
            '𝝝' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalTheta),
            '𝝞' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalIota),
            '𝝟' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalKappa),
            '𝝠' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalLamda),
            '𝝡' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalMu),
            '𝝢' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalNu),
            '𝝣' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalXi),
            '𝝤' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalOmicron),
            '𝝥' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalPi),
            '𝝦' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalRho),
            '𝝧' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalThetaSymbol),
            '𝝨' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalSigma),
            '𝝩' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalTau),
            '𝝪' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalUpsilon),
            '𝝫' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalPhi),
            '𝝬' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalChi),
            '𝝭' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalPsi),
            '𝝮' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldCapitalOmega),
            '𝝯' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldNabla),
            '𝝰' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallAlpha),
            '𝝱' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallBeta),
            '𝝲' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallGamma),
            '𝝳' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallDelta),
            '𝝴' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallEpsilon),
            '𝝵' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallZeta),
            '𝝶' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallEta),
            '𝝷' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallTheta),
            '𝝸' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallIota),
            '𝝹' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallKappa),
            '𝝺' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallLamda),
            '𝝻' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallMu),
            '𝝼' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallNu),
            '𝝽' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallXi),
            '𝝾' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallOmicron),
            '𝝿' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallPi),
            '𝞀' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallRho),
            '𝞁' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallFinalSigma),
            '𝞂' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallSigma),
            '𝞃' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallTau),
            '𝞄' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallUpsilon),
            '𝞅' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallPhi),
            '𝞆' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallChi),
            '𝞇' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallPsi),
            '𝞈' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldSmallOmega),
            '𝞉' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldPartialDifferential),
            '𝞊' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldEpsilonSymbol),
            '𝞋' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldThetaSymbol),
            '𝞌' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldKappaSymbol),
            '𝞍' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldPhiSymbol),
            '𝞎' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldRhoSymbol),
            '𝞏' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldPiSymbol),
            '𝞐' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalAlpha),
            '𝞑' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalBeta),
            '𝞒' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalGamma),
            '𝞓' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalDelta),
            '𝞔' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalEpsilon),
            '𝞕' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalZeta),
            '𝞖' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalEta),
            '𝞗' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalTheta),
            '𝞘' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalIota),
            '𝞙' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalKappa),
            '𝞚' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalLamda),
            '𝞛' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalMu),
            '𝞜' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalNu),
            '𝞝' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalXi),
            '𝞞' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalOmicron),
            '𝞟' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalPi),
            '𝞠' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalRho),
            '𝞡' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalThetaSymbol),
            '𝞢' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalSigma),
            '𝞣' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalTau),
            '𝞤' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalUpsilon),
            '𝞥' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalPhi),
            '𝞦' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalChi),
            '𝞧' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalPsi),
            '𝞨' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicCapitalOmega),
            '𝞩' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicNabla),
            '𝞪' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallAlpha),
            '𝞫' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallBeta),
            '𝞬' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallGamma),
            '𝞭' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallDelta),
            '𝞮' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallEpsilon),
            '𝞯' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallZeta),
            '𝞰' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallEta),
            '𝞱' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallTheta),
            '𝞲' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallIota),
            '𝞳' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallKappa),
            '𝞴' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallLamda),
            '𝞵' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallMu),
            '𝞶' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallNu),
            '𝞷' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallXi),
            '𝞸' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallOmicron),
            '𝞹' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallPi),
            '𝞺' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallRho),
            '𝞻' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallFinalSigma),
            '𝞼' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallSigma),
            '𝞽' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallTau),
            '𝞾' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallUpsilon),
            '𝞿' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallPhi),
            '𝟀' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallChi),
            '𝟁' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallPsi),
            '𝟂' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicSmallOmega),
            '𝟃' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicPartialDifferential),
            '𝟄' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicEpsilonSymbol),
            '𝟅' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicThetaSymbol),
            '𝟆' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicKappaSymbol),
            '𝟇' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicPhiSymbol),
            '𝟈' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicRhoSymbol),
            '𝟉' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldItalicPiSymbol),
            '𝟊' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldCapitalDigamma),
            '𝟋' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldSmallDigamma),
            '𝟎' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldDigitZero),
            '𝟏' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldDigitOne),
            '𝟐' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldDigitTwo),
            '𝟑' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldDigitThree),
            '𝟒' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldDigitFour),
            '𝟓' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldDigitFive),
            '𝟔' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldDigitSix),
            '𝟕' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldDigitSeven),
            '𝟖' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldDigitEight),
            '𝟗' => Ok(MathematicalAlphanumericSymbols::MathematicalBoldDigitNine),
            '𝟘' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckDigitZero),
            '𝟙' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckDigitOne),
            '𝟚' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckDigitTwo),
            '𝟛' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckDigitThree),
            '𝟜' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckDigitFour),
            '𝟝' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckDigitFive),
            '𝟞' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckDigitSix),
            '𝟟' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckDigitSeven),
            '𝟠' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckDigitEight),
            '𝟡' => Ok(MathematicalAlphanumericSymbols::MathematicalDoubleDashStruckDigitNine),
            '𝟢' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifDigitZero),
            '𝟣' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifDigitOne),
            '𝟤' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifDigitTwo),
            '𝟥' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifDigitThree),
            '𝟦' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifDigitFour),
            '𝟧' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifDigitFive),
            '𝟨' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifDigitSix),
            '𝟩' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifDigitSeven),
            '𝟪' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifDigitEight),
            '𝟫' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifDigitNine),
            '𝟬' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldDigitZero),
            '𝟭' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldDigitOne),
            '𝟮' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldDigitTwo),
            '𝟯' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldDigitThree),
            '𝟰' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldDigitFour),
            '𝟱' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldDigitFive),
            '𝟲' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldDigitSix),
            '𝟳' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldDigitSeven),
            '𝟴' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldDigitEight),
            '𝟵' => Ok(MathematicalAlphanumericSymbols::MathematicalSansDashSerifBoldDigitNine),
            '𝟶' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceDigitZero),
            '𝟷' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceDigitOne),
            '𝟸' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceDigitTwo),
            '𝟹' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceDigitThree),
            '𝟺' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceDigitFour),
            '𝟻' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceDigitFive),
            '𝟼' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceDigitSix),
            '𝟽' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceDigitSeven),
            '𝟾' => Ok(MathematicalAlphanumericSymbols::MathematicalMonospaceDigitEight),
            _ => Err(()),
        }
    }
}

impl Into<u32> for MathematicalAlphanumericSymbols {
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

impl std::convert::TryFrom<u32> for MathematicalAlphanumericSymbols {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for MathematicalAlphanumericSymbols {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl MathematicalAlphanumericSymbols {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        MathematicalAlphanumericSymbols::MathematicalBoldCapitalA
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("MathematicalAlphanumericSymbols{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
