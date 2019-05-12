
/// An enum to represent all characters in the Tibetan block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Tibetan {
    /// \u{f00}: 'ༀ'
    SyllableOm,
    /// \u{f01}: '༁'
    MarkGterYigMgoTruncatedA,
    /// \u{f02}: '༂'
    MarkGterYigMgoDashUmRnamBcadMa,
    /// \u{f03}: '༃'
    MarkGterYigMgoDashUmGterTshegMa,
    /// \u{f04}: '༄'
    MarkInitialYigMgoMdunMa,
    /// \u{f05}: '༅'
    MarkClosingYigMgoSgabMa,
    /// \u{f06}: '༆'
    MarkCaretYigMgoPhurShadMa,
    /// \u{f07}: '༇'
    MarkYigMgoTshegShadMa,
    /// \u{f08}: '༈'
    MarkSbrulShad,
    /// \u{f09}: '༉'
    MarkBskurYigMgo,
    /// \u{f0a}: '༊'
    MarkBkaDashShogYigMgo,
    /// \u{f0b}: '་'
    MarkIntersyllabicTsheg,
    /// \u{f0c}: '༌'
    MarkDelimiterTshegBstar,
    /// \u{f0d}: '།'
    MarkShad,
    /// \u{f0e}: '༎'
    MarkNyisShad,
    /// \u{f0f}: '༏'
    MarkTshegShad,
    /// \u{f10}: '༐'
    MarkNyisTshegShad,
    /// \u{f11}: '༑'
    MarkRinChenSpungsShad,
    /// \u{f12}: '༒'
    MarkRgyaGramShad,
    /// \u{f13}: '༓'
    MarkCaretDashDzudRtagsMeLongCan,
    /// \u{f14}: '༔'
    MarkGterTsheg,
    /// \u{f15}: '༕'
    LogotypeSignChadRtags,
    /// \u{f16}: '༖'
    LogotypeSignLhagRtags,
    /// \u{f17}: '༗'
    AstrologicalSignSgraGcanDashCharRtags,
    /// \u{f18}: '༘'
    AstrologicalSignDashKhyudPa,
    /// \u{f19}: '༙'
    AstrologicalSignSdongTshugs,
    /// \u{f1a}: '༚'
    SignRdelDkarGcig,
    /// \u{f1b}: '༛'
    SignRdelDkarGnyis,
    /// \u{f1c}: '༜'
    SignRdelDkarGsum,
    /// \u{f1d}: '༝'
    SignRdelNagGcig,
    /// \u{f1e}: '༞'
    SignRdelNagGnyis,
    /// \u{f1f}: '༟'
    SignRdelDkarRdelNag,
    /// \u{f20}: '༠'
    DigitZero,
    /// \u{f21}: '༡'
    DigitOne,
    /// \u{f22}: '༢'
    DigitTwo,
    /// \u{f23}: '༣'
    DigitThree,
    /// \u{f24}: '༤'
    DigitFour,
    /// \u{f25}: '༥'
    DigitFive,
    /// \u{f26}: '༦'
    DigitSix,
    /// \u{f27}: '༧'
    DigitSeven,
    /// \u{f28}: '༨'
    DigitEight,
    /// \u{f29}: '༩'
    DigitNine,
    /// \u{f2a}: '༪'
    DigitHalfOne,
    /// \u{f2b}: '༫'
    DigitHalfTwo,
    /// \u{f2c}: '༬'
    DigitHalfThree,
    /// \u{f2d}: '༭'
    DigitHalfFour,
    /// \u{f2e}: '༮'
    DigitHalfFive,
    /// \u{f2f}: '༯'
    DigitHalfSix,
    /// \u{f30}: '༰'
    DigitHalfSeven,
    /// \u{f31}: '༱'
    DigitHalfEight,
    /// \u{f32}: '༲'
    DigitHalfNine,
    /// \u{f33}: '༳'
    DigitHalfZero,
    /// \u{f34}: '༴'
    MarkBsdusRtags,
    /// \u{f35}: '༵'
    MarkNgasBzungNyiZla,
    /// \u{f36}: '༶'
    MarkCaretDashDzudRtagsBzhiMigCan,
    /// \u{f37}: '༷'
    MarkNgasBzungSgorRtags,
    /// \u{f38}: '༸'
    MarkCheMgo,
    /// \u{f39}: '༹'
    MarkTsaDashPhru,
    /// \u{f3a}: '༺'
    MarkGugRtagsGyon,
    /// \u{f3b}: '༻'
    MarkGugRtagsGyas,
    /// \u{f3c}: '༼'
    MarkAngKhangGyon,
    /// \u{f3d}: '༽'
    MarkAngKhangGyas,
    /// \u{f3e}: '༾'
    SignYarTshes,
    /// \u{f3f}: '༿'
    SignMarTshes,
    /// \u{f40}: 'ཀ'
    LetterKa,
    /// \u{f41}: 'ཁ'
    LetterKha,
    /// \u{f42}: 'ག'
    LetterGa,
    /// \u{f43}: 'གྷ'
    LetterGha,
    /// \u{f44}: 'ང'
    LetterNga,
    /// \u{f45}: 'ཅ'
    LetterCa,
    /// \u{f46}: 'ཆ'
    LetterCha,
    /// \u{f47}: 'ཇ'
    LetterJa,
    /// \u{f49}: 'ཉ'
    LetterNya,
    /// \u{f4a}: 'ཊ'
    LetterTta,
    /// \u{f4b}: 'ཋ'
    LetterTtha,
    /// \u{f4c}: 'ཌ'
    LetterDda,
    /// \u{f4d}: 'ཌྷ'
    LetterDdha,
    /// \u{f4e}: 'ཎ'
    LetterNna,
    /// \u{f4f}: 'ཏ'
    LetterTa,
    /// \u{f50}: 'ཐ'
    LetterTha,
    /// \u{f51}: 'ད'
    LetterDa,
    /// \u{f52}: 'དྷ'
    LetterDha,
    /// \u{f53}: 'ན'
    LetterNa,
    /// \u{f54}: 'པ'
    LetterPa,
    /// \u{f55}: 'ཕ'
    LetterPha,
    /// \u{f56}: 'བ'
    LetterBa,
    /// \u{f57}: 'བྷ'
    LetterBha,
    /// \u{f58}: 'མ'
    LetterMa,
    /// \u{f59}: 'ཙ'
    LetterTsa,
    /// \u{f5a}: 'ཚ'
    LetterTsha,
    /// \u{f5b}: 'ཛ'
    LetterDza,
    /// \u{f5c}: 'ཛྷ'
    LetterDzha,
    /// \u{f5d}: 'ཝ'
    LetterWa,
    /// \u{f5e}: 'ཞ'
    LetterZha,
    /// \u{f5f}: 'ཟ'
    LetterZa,
    /// \u{f60}: 'འ'
    LetterDashA,
    /// \u{f61}: 'ཡ'
    LetterYa,
    /// \u{f62}: 'ར'
    LetterRa,
    /// \u{f63}: 'ལ'
    LetterLa,
    /// \u{f64}: 'ཤ'
    LetterSha,
    /// \u{f65}: 'ཥ'
    LetterSsa,
    /// \u{f66}: 'ས'
    LetterSa,
    /// \u{f67}: 'ཧ'
    LetterHa,
    /// \u{f68}: 'ཨ'
    LetterA,
    /// \u{f69}: 'ཀྵ'
    LetterKssa,
    /// \u{f6a}: 'ཪ'
    LetterFixedDashFormRa,
    /// \u{f6b}: 'ཫ'
    LetterKka,
    /// \u{f6c}: 'ཬ'
    LetterRra,
    /// \u{f71}: 'ཱ'
    VowelSignAa,
    /// \u{f72}: 'ི'
    VowelSignI,
    /// \u{f73}: 'ཱི'
    VowelSignIi,
    /// \u{f74}: 'ུ'
    VowelSignU,
    /// \u{f75}: 'ཱུ'
    VowelSignUu,
    /// \u{f76}: 'ྲྀ'
    VowelSignVocalicR,
    /// \u{f77}: 'ཷ'
    VowelSignVocalicRr,
    /// \u{f78}: 'ླྀ'
    VowelSignVocalicL,
    /// \u{f79}: 'ཹ'
    VowelSignVocalicLl,
    /// \u{f7a}: 'ེ'
    VowelSignE,
    /// \u{f7b}: 'ཻ'
    VowelSignEe,
    /// \u{f7c}: 'ོ'
    VowelSignO,
    /// \u{f7d}: 'ཽ'
    VowelSignOo,
    /// \u{f7e}: 'ཾ'
    SignRjesSuNgaRo,
    /// \u{f7f}: 'ཿ'
    SignRnamBcad,
    /// \u{f80}: 'ྀ'
    VowelSignReversedI,
    /// \u{f81}: 'ཱྀ'
    VowelSignReversedIi,
    /// \u{f82}: 'ྂ'
    SignNyiZlaNaaDa,
    /// \u{f83}: 'ྃ'
    SignSnaLdan,
    /// \u{f84}: '྄'
    MarkHalanta,
    /// \u{f85}: '྅'
    MarkPaluta,
    /// \u{f86}: '྆'
    SignLciRtags,
    /// \u{f87}: '྇'
    SignYangRtags,
    /// \u{f88}: 'ྈ'
    SignLceTsaCan,
    /// \u{f89}: 'ྉ'
    SignMchuCan,
    /// \u{f8a}: 'ྊ'
    SignGruCanRgyings,
    /// \u{f8b}: 'ྋ'
    SignGruMedRgyings,
    /// \u{f8c}: 'ྌ'
    SignInvertedMchuCan,
    /// \u{f8d}: 'ྍ'
    SubjoinedSignLceTsaCan,
    /// \u{f8e}: 'ྎ'
    SubjoinedSignMchuCan,
    /// \u{f8f}: 'ྏ'
    SubjoinedSignInvertedMchuCan,
    /// \u{f90}: 'ྐ'
    SubjoinedLetterKa,
    /// \u{f91}: 'ྑ'
    SubjoinedLetterKha,
    /// \u{f92}: 'ྒ'
    SubjoinedLetterGa,
    /// \u{f93}: 'ྒྷ'
    SubjoinedLetterGha,
    /// \u{f94}: 'ྔ'
    SubjoinedLetterNga,
    /// \u{f95}: 'ྕ'
    SubjoinedLetterCa,
    /// \u{f96}: 'ྖ'
    SubjoinedLetterCha,
    /// \u{f97}: 'ྗ'
    SubjoinedLetterJa,
    /// \u{f99}: 'ྙ'
    SubjoinedLetterNya,
    /// \u{f9a}: 'ྚ'
    SubjoinedLetterTta,
    /// \u{f9b}: 'ྛ'
    SubjoinedLetterTtha,
    /// \u{f9c}: 'ྜ'
    SubjoinedLetterDda,
    /// \u{f9d}: 'ྜྷ'
    SubjoinedLetterDdha,
    /// \u{f9e}: 'ྞ'
    SubjoinedLetterNna,
    /// \u{f9f}: 'ྟ'
    SubjoinedLetterTa,
    /// \u{fa0}: 'ྠ'
    SubjoinedLetterTha,
    /// \u{fa1}: 'ྡ'
    SubjoinedLetterDa,
    /// \u{fa2}: 'ྡྷ'
    SubjoinedLetterDha,
    /// \u{fa3}: 'ྣ'
    SubjoinedLetterNa,
    /// \u{fa4}: 'ྤ'
    SubjoinedLetterPa,
    /// \u{fa5}: 'ྥ'
    SubjoinedLetterPha,
    /// \u{fa6}: 'ྦ'
    SubjoinedLetterBa,
    /// \u{fa7}: 'ྦྷ'
    SubjoinedLetterBha,
    /// \u{fa8}: 'ྨ'
    SubjoinedLetterMa,
    /// \u{fa9}: 'ྩ'
    SubjoinedLetterTsa,
    /// \u{faa}: 'ྪ'
    SubjoinedLetterTsha,
    /// \u{fab}: 'ྫ'
    SubjoinedLetterDza,
    /// \u{fac}: 'ྫྷ'
    SubjoinedLetterDzha,
    /// \u{fad}: 'ྭ'
    SubjoinedLetterWa,
    /// \u{fae}: 'ྮ'
    SubjoinedLetterZha,
    /// \u{faf}: 'ྯ'
    SubjoinedLetterZa,
    /// \u{fb0}: 'ྰ'
    SubjoinedLetterDashA,
    /// \u{fb1}: 'ྱ'
    SubjoinedLetterYa,
    /// \u{fb2}: 'ྲ'
    SubjoinedLetterRa,
    /// \u{fb3}: 'ླ'
    SubjoinedLetterLa,
    /// \u{fb4}: 'ྴ'
    SubjoinedLetterSha,
    /// \u{fb5}: 'ྵ'
    SubjoinedLetterSsa,
    /// \u{fb6}: 'ྶ'
    SubjoinedLetterSa,
    /// \u{fb7}: 'ྷ'
    SubjoinedLetterHa,
    /// \u{fb8}: 'ྸ'
    SubjoinedLetterA,
    /// \u{fb9}: 'ྐྵ'
    SubjoinedLetterKssa,
    /// \u{fba}: 'ྺ'
    SubjoinedLetterFixedDashFormWa,
    /// \u{fbb}: 'ྻ'
    SubjoinedLetterFixedDashFormYa,
    /// \u{fbc}: 'ྼ'
    SubjoinedLetterFixedDashFormRa,
    /// \u{fbe}: '྾'
    KuRuKha,
    /// \u{fbf}: '྿'
    KuRuKhaBzhiMigCan,
    /// \u{fc0}: '࿀'
    CantillationSignHeavyBeat,
    /// \u{fc1}: '࿁'
    CantillationSignLightBeat,
    /// \u{fc2}: '࿂'
    CantillationSignCangTeDashU,
    /// \u{fc3}: '࿃'
    CantillationSignSbubDashChal,
    /// \u{fc4}: '࿄'
    SymbolDrilBu,
    /// \u{fc5}: '࿅'
    SymbolRdoRje,
    /// \u{fc6}: '࿆'
    SymbolPadmaGdan,
    /// \u{fc7}: '࿇'
    SymbolRdoRjeRgyaGram,
    /// \u{fc8}: '࿈'
    SymbolPhurPa,
    /// \u{fc9}: '࿉'
    SymbolNorBu,
    /// \u{fca}: '࿊'
    SymbolNorBuNyisDashKhyil,
    /// \u{fcb}: '࿋'
    SymbolNorBuGsumDashKhyil,
    /// \u{fcc}: '࿌'
    SymbolNorBuBzhiDashKhyil,
    /// \u{fce}: '࿎'
    SignRdelNagRdelDkar,
    /// \u{fcf}: '࿏'
    SignRdelNagGsum,
    /// \u{fd0}: '࿐'
    MarkBskaDashShogGiMgoRgyan,
    /// \u{fd1}: '࿑'
    MarkMnyamYigGiMgoRgyan,
    /// \u{fd2}: '࿒'
    MarkNyisTsheg,
    /// \u{fd3}: '࿓'
    MarkInitialBrdaRnyingYigMgoMdunMa,
    /// \u{fd4}: '࿔'
    MarkClosingBrdaRnyingYigMgoSgabMa,
    /// \u{fd5}: '࿕'
    RightDashFacingSvastiSign,
    /// \u{fd6}: '࿖'
    LeftDashFacingSvastiSign,
    /// \u{fd7}: '࿗'
    RightDashFacingSvastiSignWithDots,
    /// \u{fd8}: '࿘'
    LeftDashFacingSvastiSignWithDots,
    /// \u{fd9}: '࿙'
    MarkLeadingMchanRtags,
    /// \u{fda}: '࿚'
    MarkTrailingMchanRtags,
}

impl Into<char> for Tibetan {
    fn into(self) -> char {
        match self {
            Tibetan::SyllableOm => 'ༀ',
            Tibetan::MarkGterYigMgoTruncatedA => '༁',
            Tibetan::MarkGterYigMgoDashUmRnamBcadMa => '༂',
            Tibetan::MarkGterYigMgoDashUmGterTshegMa => '༃',
            Tibetan::MarkInitialYigMgoMdunMa => '༄',
            Tibetan::MarkClosingYigMgoSgabMa => '༅',
            Tibetan::MarkCaretYigMgoPhurShadMa => '༆',
            Tibetan::MarkYigMgoTshegShadMa => '༇',
            Tibetan::MarkSbrulShad => '༈',
            Tibetan::MarkBskurYigMgo => '༉',
            Tibetan::MarkBkaDashShogYigMgo => '༊',
            Tibetan::MarkIntersyllabicTsheg => '་',
            Tibetan::MarkDelimiterTshegBstar => '༌',
            Tibetan::MarkShad => '།',
            Tibetan::MarkNyisShad => '༎',
            Tibetan::MarkTshegShad => '༏',
            Tibetan::MarkNyisTshegShad => '༐',
            Tibetan::MarkRinChenSpungsShad => '༑',
            Tibetan::MarkRgyaGramShad => '༒',
            Tibetan::MarkCaretDashDzudRtagsMeLongCan => '༓',
            Tibetan::MarkGterTsheg => '༔',
            Tibetan::LogotypeSignChadRtags => '༕',
            Tibetan::LogotypeSignLhagRtags => '༖',
            Tibetan::AstrologicalSignSgraGcanDashCharRtags => '༗',
            Tibetan::AstrologicalSignDashKhyudPa => '༘',
            Tibetan::AstrologicalSignSdongTshugs => '༙',
            Tibetan::SignRdelDkarGcig => '༚',
            Tibetan::SignRdelDkarGnyis => '༛',
            Tibetan::SignRdelDkarGsum => '༜',
            Tibetan::SignRdelNagGcig => '༝',
            Tibetan::SignRdelNagGnyis => '༞',
            Tibetan::SignRdelDkarRdelNag => '༟',
            Tibetan::DigitZero => '༠',
            Tibetan::DigitOne => '༡',
            Tibetan::DigitTwo => '༢',
            Tibetan::DigitThree => '༣',
            Tibetan::DigitFour => '༤',
            Tibetan::DigitFive => '༥',
            Tibetan::DigitSix => '༦',
            Tibetan::DigitSeven => '༧',
            Tibetan::DigitEight => '༨',
            Tibetan::DigitNine => '༩',
            Tibetan::DigitHalfOne => '༪',
            Tibetan::DigitHalfTwo => '༫',
            Tibetan::DigitHalfThree => '༬',
            Tibetan::DigitHalfFour => '༭',
            Tibetan::DigitHalfFive => '༮',
            Tibetan::DigitHalfSix => '༯',
            Tibetan::DigitHalfSeven => '༰',
            Tibetan::DigitHalfEight => '༱',
            Tibetan::DigitHalfNine => '༲',
            Tibetan::DigitHalfZero => '༳',
            Tibetan::MarkBsdusRtags => '༴',
            Tibetan::MarkNgasBzungNyiZla => '༵',
            Tibetan::MarkCaretDashDzudRtagsBzhiMigCan => '༶',
            Tibetan::MarkNgasBzungSgorRtags => '༷',
            Tibetan::MarkCheMgo => '༸',
            Tibetan::MarkTsaDashPhru => '༹',
            Tibetan::MarkGugRtagsGyon => '༺',
            Tibetan::MarkGugRtagsGyas => '༻',
            Tibetan::MarkAngKhangGyon => '༼',
            Tibetan::MarkAngKhangGyas => '༽',
            Tibetan::SignYarTshes => '༾',
            Tibetan::SignMarTshes => '༿',
            Tibetan::LetterKa => 'ཀ',
            Tibetan::LetterKha => 'ཁ',
            Tibetan::LetterGa => 'ག',
            Tibetan::LetterGha => 'གྷ',
            Tibetan::LetterNga => 'ང',
            Tibetan::LetterCa => 'ཅ',
            Tibetan::LetterCha => 'ཆ',
            Tibetan::LetterJa => 'ཇ',
            Tibetan::LetterNya => 'ཉ',
            Tibetan::LetterTta => 'ཊ',
            Tibetan::LetterTtha => 'ཋ',
            Tibetan::LetterDda => 'ཌ',
            Tibetan::LetterDdha => 'ཌྷ',
            Tibetan::LetterNna => 'ཎ',
            Tibetan::LetterTa => 'ཏ',
            Tibetan::LetterTha => 'ཐ',
            Tibetan::LetterDa => 'ད',
            Tibetan::LetterDha => 'དྷ',
            Tibetan::LetterNa => 'ན',
            Tibetan::LetterPa => 'པ',
            Tibetan::LetterPha => 'ཕ',
            Tibetan::LetterBa => 'བ',
            Tibetan::LetterBha => 'བྷ',
            Tibetan::LetterMa => 'མ',
            Tibetan::LetterTsa => 'ཙ',
            Tibetan::LetterTsha => 'ཚ',
            Tibetan::LetterDza => 'ཛ',
            Tibetan::LetterDzha => 'ཛྷ',
            Tibetan::LetterWa => 'ཝ',
            Tibetan::LetterZha => 'ཞ',
            Tibetan::LetterZa => 'ཟ',
            Tibetan::LetterDashA => 'འ',
            Tibetan::LetterYa => 'ཡ',
            Tibetan::LetterRa => 'ར',
            Tibetan::LetterLa => 'ལ',
            Tibetan::LetterSha => 'ཤ',
            Tibetan::LetterSsa => 'ཥ',
            Tibetan::LetterSa => 'ས',
            Tibetan::LetterHa => 'ཧ',
            Tibetan::LetterA => 'ཨ',
            Tibetan::LetterKssa => 'ཀྵ',
            Tibetan::LetterFixedDashFormRa => 'ཪ',
            Tibetan::LetterKka => 'ཫ',
            Tibetan::LetterRra => 'ཬ',
            Tibetan::VowelSignAa => 'ཱ',
            Tibetan::VowelSignI => 'ི',
            Tibetan::VowelSignIi => 'ཱི',
            Tibetan::VowelSignU => 'ུ',
            Tibetan::VowelSignUu => 'ཱུ',
            Tibetan::VowelSignVocalicR => 'ྲྀ',
            Tibetan::VowelSignVocalicRr => 'ཷ',
            Tibetan::VowelSignVocalicL => 'ླྀ',
            Tibetan::VowelSignVocalicLl => 'ཹ',
            Tibetan::VowelSignE => 'ེ',
            Tibetan::VowelSignEe => 'ཻ',
            Tibetan::VowelSignO => 'ོ',
            Tibetan::VowelSignOo => 'ཽ',
            Tibetan::SignRjesSuNgaRo => 'ཾ',
            Tibetan::SignRnamBcad => 'ཿ',
            Tibetan::VowelSignReversedI => 'ྀ',
            Tibetan::VowelSignReversedIi => 'ཱྀ',
            Tibetan::SignNyiZlaNaaDa => 'ྂ',
            Tibetan::SignSnaLdan => 'ྃ',
            Tibetan::MarkHalanta => '྄',
            Tibetan::MarkPaluta => '྅',
            Tibetan::SignLciRtags => '྆',
            Tibetan::SignYangRtags => '྇',
            Tibetan::SignLceTsaCan => 'ྈ',
            Tibetan::SignMchuCan => 'ྉ',
            Tibetan::SignGruCanRgyings => 'ྊ',
            Tibetan::SignGruMedRgyings => 'ྋ',
            Tibetan::SignInvertedMchuCan => 'ྌ',
            Tibetan::SubjoinedSignLceTsaCan => 'ྍ',
            Tibetan::SubjoinedSignMchuCan => 'ྎ',
            Tibetan::SubjoinedSignInvertedMchuCan => 'ྏ',
            Tibetan::SubjoinedLetterKa => 'ྐ',
            Tibetan::SubjoinedLetterKha => 'ྑ',
            Tibetan::SubjoinedLetterGa => 'ྒ',
            Tibetan::SubjoinedLetterGha => 'ྒྷ',
            Tibetan::SubjoinedLetterNga => 'ྔ',
            Tibetan::SubjoinedLetterCa => 'ྕ',
            Tibetan::SubjoinedLetterCha => 'ྖ',
            Tibetan::SubjoinedLetterJa => 'ྗ',
            Tibetan::SubjoinedLetterNya => 'ྙ',
            Tibetan::SubjoinedLetterTta => 'ྚ',
            Tibetan::SubjoinedLetterTtha => 'ྛ',
            Tibetan::SubjoinedLetterDda => 'ྜ',
            Tibetan::SubjoinedLetterDdha => 'ྜྷ',
            Tibetan::SubjoinedLetterNna => 'ྞ',
            Tibetan::SubjoinedLetterTa => 'ྟ',
            Tibetan::SubjoinedLetterTha => 'ྠ',
            Tibetan::SubjoinedLetterDa => 'ྡ',
            Tibetan::SubjoinedLetterDha => 'ྡྷ',
            Tibetan::SubjoinedLetterNa => 'ྣ',
            Tibetan::SubjoinedLetterPa => 'ྤ',
            Tibetan::SubjoinedLetterPha => 'ྥ',
            Tibetan::SubjoinedLetterBa => 'ྦ',
            Tibetan::SubjoinedLetterBha => 'ྦྷ',
            Tibetan::SubjoinedLetterMa => 'ྨ',
            Tibetan::SubjoinedLetterTsa => 'ྩ',
            Tibetan::SubjoinedLetterTsha => 'ྪ',
            Tibetan::SubjoinedLetterDza => 'ྫ',
            Tibetan::SubjoinedLetterDzha => 'ྫྷ',
            Tibetan::SubjoinedLetterWa => 'ྭ',
            Tibetan::SubjoinedLetterZha => 'ྮ',
            Tibetan::SubjoinedLetterZa => 'ྯ',
            Tibetan::SubjoinedLetterDashA => 'ྰ',
            Tibetan::SubjoinedLetterYa => 'ྱ',
            Tibetan::SubjoinedLetterRa => 'ྲ',
            Tibetan::SubjoinedLetterLa => 'ླ',
            Tibetan::SubjoinedLetterSha => 'ྴ',
            Tibetan::SubjoinedLetterSsa => 'ྵ',
            Tibetan::SubjoinedLetterSa => 'ྶ',
            Tibetan::SubjoinedLetterHa => 'ྷ',
            Tibetan::SubjoinedLetterA => 'ྸ',
            Tibetan::SubjoinedLetterKssa => 'ྐྵ',
            Tibetan::SubjoinedLetterFixedDashFormWa => 'ྺ',
            Tibetan::SubjoinedLetterFixedDashFormYa => 'ྻ',
            Tibetan::SubjoinedLetterFixedDashFormRa => 'ྼ',
            Tibetan::KuRuKha => '྾',
            Tibetan::KuRuKhaBzhiMigCan => '྿',
            Tibetan::CantillationSignHeavyBeat => '࿀',
            Tibetan::CantillationSignLightBeat => '࿁',
            Tibetan::CantillationSignCangTeDashU => '࿂',
            Tibetan::CantillationSignSbubDashChal => '࿃',
            Tibetan::SymbolDrilBu => '࿄',
            Tibetan::SymbolRdoRje => '࿅',
            Tibetan::SymbolPadmaGdan => '࿆',
            Tibetan::SymbolRdoRjeRgyaGram => '࿇',
            Tibetan::SymbolPhurPa => '࿈',
            Tibetan::SymbolNorBu => '࿉',
            Tibetan::SymbolNorBuNyisDashKhyil => '࿊',
            Tibetan::SymbolNorBuGsumDashKhyil => '࿋',
            Tibetan::SymbolNorBuBzhiDashKhyil => '࿌',
            Tibetan::SignRdelNagRdelDkar => '࿎',
            Tibetan::SignRdelNagGsum => '࿏',
            Tibetan::MarkBskaDashShogGiMgoRgyan => '࿐',
            Tibetan::MarkMnyamYigGiMgoRgyan => '࿑',
            Tibetan::MarkNyisTsheg => '࿒',
            Tibetan::MarkInitialBrdaRnyingYigMgoMdunMa => '࿓',
            Tibetan::MarkClosingBrdaRnyingYigMgoSgabMa => '࿔',
            Tibetan::RightDashFacingSvastiSign => '࿕',
            Tibetan::LeftDashFacingSvastiSign => '࿖',
            Tibetan::RightDashFacingSvastiSignWithDots => '࿗',
            Tibetan::LeftDashFacingSvastiSignWithDots => '࿘',
            Tibetan::MarkLeadingMchanRtags => '࿙',
            Tibetan::MarkTrailingMchanRtags => '࿚',
        }
    }
}

impl std::convert::TryFrom<char> for Tibetan {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ༀ' => Ok(Tibetan::SyllableOm),
            '༁' => Ok(Tibetan::MarkGterYigMgoTruncatedA),
            '༂' => Ok(Tibetan::MarkGterYigMgoDashUmRnamBcadMa),
            '༃' => Ok(Tibetan::MarkGterYigMgoDashUmGterTshegMa),
            '༄' => Ok(Tibetan::MarkInitialYigMgoMdunMa),
            '༅' => Ok(Tibetan::MarkClosingYigMgoSgabMa),
            '༆' => Ok(Tibetan::MarkCaretYigMgoPhurShadMa),
            '༇' => Ok(Tibetan::MarkYigMgoTshegShadMa),
            '༈' => Ok(Tibetan::MarkSbrulShad),
            '༉' => Ok(Tibetan::MarkBskurYigMgo),
            '༊' => Ok(Tibetan::MarkBkaDashShogYigMgo),
            '་' => Ok(Tibetan::MarkIntersyllabicTsheg),
            '༌' => Ok(Tibetan::MarkDelimiterTshegBstar),
            '།' => Ok(Tibetan::MarkShad),
            '༎' => Ok(Tibetan::MarkNyisShad),
            '༏' => Ok(Tibetan::MarkTshegShad),
            '༐' => Ok(Tibetan::MarkNyisTshegShad),
            '༑' => Ok(Tibetan::MarkRinChenSpungsShad),
            '༒' => Ok(Tibetan::MarkRgyaGramShad),
            '༓' => Ok(Tibetan::MarkCaretDashDzudRtagsMeLongCan),
            '༔' => Ok(Tibetan::MarkGterTsheg),
            '༕' => Ok(Tibetan::LogotypeSignChadRtags),
            '༖' => Ok(Tibetan::LogotypeSignLhagRtags),
            '༗' => Ok(Tibetan::AstrologicalSignSgraGcanDashCharRtags),
            '༘' => Ok(Tibetan::AstrologicalSignDashKhyudPa),
            '༙' => Ok(Tibetan::AstrologicalSignSdongTshugs),
            '༚' => Ok(Tibetan::SignRdelDkarGcig),
            '༛' => Ok(Tibetan::SignRdelDkarGnyis),
            '༜' => Ok(Tibetan::SignRdelDkarGsum),
            '༝' => Ok(Tibetan::SignRdelNagGcig),
            '༞' => Ok(Tibetan::SignRdelNagGnyis),
            '༟' => Ok(Tibetan::SignRdelDkarRdelNag),
            '༠' => Ok(Tibetan::DigitZero),
            '༡' => Ok(Tibetan::DigitOne),
            '༢' => Ok(Tibetan::DigitTwo),
            '༣' => Ok(Tibetan::DigitThree),
            '༤' => Ok(Tibetan::DigitFour),
            '༥' => Ok(Tibetan::DigitFive),
            '༦' => Ok(Tibetan::DigitSix),
            '༧' => Ok(Tibetan::DigitSeven),
            '༨' => Ok(Tibetan::DigitEight),
            '༩' => Ok(Tibetan::DigitNine),
            '༪' => Ok(Tibetan::DigitHalfOne),
            '༫' => Ok(Tibetan::DigitHalfTwo),
            '༬' => Ok(Tibetan::DigitHalfThree),
            '༭' => Ok(Tibetan::DigitHalfFour),
            '༮' => Ok(Tibetan::DigitHalfFive),
            '༯' => Ok(Tibetan::DigitHalfSix),
            '༰' => Ok(Tibetan::DigitHalfSeven),
            '༱' => Ok(Tibetan::DigitHalfEight),
            '༲' => Ok(Tibetan::DigitHalfNine),
            '༳' => Ok(Tibetan::DigitHalfZero),
            '༴' => Ok(Tibetan::MarkBsdusRtags),
            '༵' => Ok(Tibetan::MarkNgasBzungNyiZla),
            '༶' => Ok(Tibetan::MarkCaretDashDzudRtagsBzhiMigCan),
            '༷' => Ok(Tibetan::MarkNgasBzungSgorRtags),
            '༸' => Ok(Tibetan::MarkCheMgo),
            '༹' => Ok(Tibetan::MarkTsaDashPhru),
            '༺' => Ok(Tibetan::MarkGugRtagsGyon),
            '༻' => Ok(Tibetan::MarkGugRtagsGyas),
            '༼' => Ok(Tibetan::MarkAngKhangGyon),
            '༽' => Ok(Tibetan::MarkAngKhangGyas),
            '༾' => Ok(Tibetan::SignYarTshes),
            '༿' => Ok(Tibetan::SignMarTshes),
            'ཀ' => Ok(Tibetan::LetterKa),
            'ཁ' => Ok(Tibetan::LetterKha),
            'ག' => Ok(Tibetan::LetterGa),
            'གྷ' => Ok(Tibetan::LetterGha),
            'ང' => Ok(Tibetan::LetterNga),
            'ཅ' => Ok(Tibetan::LetterCa),
            'ཆ' => Ok(Tibetan::LetterCha),
            'ཇ' => Ok(Tibetan::LetterJa),
            'ཉ' => Ok(Tibetan::LetterNya),
            'ཊ' => Ok(Tibetan::LetterTta),
            'ཋ' => Ok(Tibetan::LetterTtha),
            'ཌ' => Ok(Tibetan::LetterDda),
            'ཌྷ' => Ok(Tibetan::LetterDdha),
            'ཎ' => Ok(Tibetan::LetterNna),
            'ཏ' => Ok(Tibetan::LetterTa),
            'ཐ' => Ok(Tibetan::LetterTha),
            'ད' => Ok(Tibetan::LetterDa),
            'དྷ' => Ok(Tibetan::LetterDha),
            'ན' => Ok(Tibetan::LetterNa),
            'པ' => Ok(Tibetan::LetterPa),
            'ཕ' => Ok(Tibetan::LetterPha),
            'བ' => Ok(Tibetan::LetterBa),
            'བྷ' => Ok(Tibetan::LetterBha),
            'མ' => Ok(Tibetan::LetterMa),
            'ཙ' => Ok(Tibetan::LetterTsa),
            'ཚ' => Ok(Tibetan::LetterTsha),
            'ཛ' => Ok(Tibetan::LetterDza),
            'ཛྷ' => Ok(Tibetan::LetterDzha),
            'ཝ' => Ok(Tibetan::LetterWa),
            'ཞ' => Ok(Tibetan::LetterZha),
            'ཟ' => Ok(Tibetan::LetterZa),
            'འ' => Ok(Tibetan::LetterDashA),
            'ཡ' => Ok(Tibetan::LetterYa),
            'ར' => Ok(Tibetan::LetterRa),
            'ལ' => Ok(Tibetan::LetterLa),
            'ཤ' => Ok(Tibetan::LetterSha),
            'ཥ' => Ok(Tibetan::LetterSsa),
            'ས' => Ok(Tibetan::LetterSa),
            'ཧ' => Ok(Tibetan::LetterHa),
            'ཨ' => Ok(Tibetan::LetterA),
            'ཀྵ' => Ok(Tibetan::LetterKssa),
            'ཪ' => Ok(Tibetan::LetterFixedDashFormRa),
            'ཫ' => Ok(Tibetan::LetterKka),
            'ཬ' => Ok(Tibetan::LetterRra),
            'ཱ' => Ok(Tibetan::VowelSignAa),
            'ི' => Ok(Tibetan::VowelSignI),
            'ཱི' => Ok(Tibetan::VowelSignIi),
            'ུ' => Ok(Tibetan::VowelSignU),
            'ཱུ' => Ok(Tibetan::VowelSignUu),
            'ྲྀ' => Ok(Tibetan::VowelSignVocalicR),
            'ཷ' => Ok(Tibetan::VowelSignVocalicRr),
            'ླྀ' => Ok(Tibetan::VowelSignVocalicL),
            'ཹ' => Ok(Tibetan::VowelSignVocalicLl),
            'ེ' => Ok(Tibetan::VowelSignE),
            'ཻ' => Ok(Tibetan::VowelSignEe),
            'ོ' => Ok(Tibetan::VowelSignO),
            'ཽ' => Ok(Tibetan::VowelSignOo),
            'ཾ' => Ok(Tibetan::SignRjesSuNgaRo),
            'ཿ' => Ok(Tibetan::SignRnamBcad),
            'ྀ' => Ok(Tibetan::VowelSignReversedI),
            'ཱྀ' => Ok(Tibetan::VowelSignReversedIi),
            'ྂ' => Ok(Tibetan::SignNyiZlaNaaDa),
            'ྃ' => Ok(Tibetan::SignSnaLdan),
            '྄' => Ok(Tibetan::MarkHalanta),
            '྅' => Ok(Tibetan::MarkPaluta),
            '྆' => Ok(Tibetan::SignLciRtags),
            '྇' => Ok(Tibetan::SignYangRtags),
            'ྈ' => Ok(Tibetan::SignLceTsaCan),
            'ྉ' => Ok(Tibetan::SignMchuCan),
            'ྊ' => Ok(Tibetan::SignGruCanRgyings),
            'ྋ' => Ok(Tibetan::SignGruMedRgyings),
            'ྌ' => Ok(Tibetan::SignInvertedMchuCan),
            'ྍ' => Ok(Tibetan::SubjoinedSignLceTsaCan),
            'ྎ' => Ok(Tibetan::SubjoinedSignMchuCan),
            'ྏ' => Ok(Tibetan::SubjoinedSignInvertedMchuCan),
            'ྐ' => Ok(Tibetan::SubjoinedLetterKa),
            'ྑ' => Ok(Tibetan::SubjoinedLetterKha),
            'ྒ' => Ok(Tibetan::SubjoinedLetterGa),
            'ྒྷ' => Ok(Tibetan::SubjoinedLetterGha),
            'ྔ' => Ok(Tibetan::SubjoinedLetterNga),
            'ྕ' => Ok(Tibetan::SubjoinedLetterCa),
            'ྖ' => Ok(Tibetan::SubjoinedLetterCha),
            'ྗ' => Ok(Tibetan::SubjoinedLetterJa),
            'ྙ' => Ok(Tibetan::SubjoinedLetterNya),
            'ྚ' => Ok(Tibetan::SubjoinedLetterTta),
            'ྛ' => Ok(Tibetan::SubjoinedLetterTtha),
            'ྜ' => Ok(Tibetan::SubjoinedLetterDda),
            'ྜྷ' => Ok(Tibetan::SubjoinedLetterDdha),
            'ྞ' => Ok(Tibetan::SubjoinedLetterNna),
            'ྟ' => Ok(Tibetan::SubjoinedLetterTa),
            'ྠ' => Ok(Tibetan::SubjoinedLetterTha),
            'ྡ' => Ok(Tibetan::SubjoinedLetterDa),
            'ྡྷ' => Ok(Tibetan::SubjoinedLetterDha),
            'ྣ' => Ok(Tibetan::SubjoinedLetterNa),
            'ྤ' => Ok(Tibetan::SubjoinedLetterPa),
            'ྥ' => Ok(Tibetan::SubjoinedLetterPha),
            'ྦ' => Ok(Tibetan::SubjoinedLetterBa),
            'ྦྷ' => Ok(Tibetan::SubjoinedLetterBha),
            'ྨ' => Ok(Tibetan::SubjoinedLetterMa),
            'ྩ' => Ok(Tibetan::SubjoinedLetterTsa),
            'ྪ' => Ok(Tibetan::SubjoinedLetterTsha),
            'ྫ' => Ok(Tibetan::SubjoinedLetterDza),
            'ྫྷ' => Ok(Tibetan::SubjoinedLetterDzha),
            'ྭ' => Ok(Tibetan::SubjoinedLetterWa),
            'ྮ' => Ok(Tibetan::SubjoinedLetterZha),
            'ྯ' => Ok(Tibetan::SubjoinedLetterZa),
            'ྰ' => Ok(Tibetan::SubjoinedLetterDashA),
            'ྱ' => Ok(Tibetan::SubjoinedLetterYa),
            'ྲ' => Ok(Tibetan::SubjoinedLetterRa),
            'ླ' => Ok(Tibetan::SubjoinedLetterLa),
            'ྴ' => Ok(Tibetan::SubjoinedLetterSha),
            'ྵ' => Ok(Tibetan::SubjoinedLetterSsa),
            'ྶ' => Ok(Tibetan::SubjoinedLetterSa),
            'ྷ' => Ok(Tibetan::SubjoinedLetterHa),
            'ྸ' => Ok(Tibetan::SubjoinedLetterA),
            'ྐྵ' => Ok(Tibetan::SubjoinedLetterKssa),
            'ྺ' => Ok(Tibetan::SubjoinedLetterFixedDashFormWa),
            'ྻ' => Ok(Tibetan::SubjoinedLetterFixedDashFormYa),
            'ྼ' => Ok(Tibetan::SubjoinedLetterFixedDashFormRa),
            '྾' => Ok(Tibetan::KuRuKha),
            '྿' => Ok(Tibetan::KuRuKhaBzhiMigCan),
            '࿀' => Ok(Tibetan::CantillationSignHeavyBeat),
            '࿁' => Ok(Tibetan::CantillationSignLightBeat),
            '࿂' => Ok(Tibetan::CantillationSignCangTeDashU),
            '࿃' => Ok(Tibetan::CantillationSignSbubDashChal),
            '࿄' => Ok(Tibetan::SymbolDrilBu),
            '࿅' => Ok(Tibetan::SymbolRdoRje),
            '࿆' => Ok(Tibetan::SymbolPadmaGdan),
            '࿇' => Ok(Tibetan::SymbolRdoRjeRgyaGram),
            '࿈' => Ok(Tibetan::SymbolPhurPa),
            '࿉' => Ok(Tibetan::SymbolNorBu),
            '࿊' => Ok(Tibetan::SymbolNorBuNyisDashKhyil),
            '࿋' => Ok(Tibetan::SymbolNorBuGsumDashKhyil),
            '࿌' => Ok(Tibetan::SymbolNorBuBzhiDashKhyil),
            '࿎' => Ok(Tibetan::SignRdelNagRdelDkar),
            '࿏' => Ok(Tibetan::SignRdelNagGsum),
            '࿐' => Ok(Tibetan::MarkBskaDashShogGiMgoRgyan),
            '࿑' => Ok(Tibetan::MarkMnyamYigGiMgoRgyan),
            '࿒' => Ok(Tibetan::MarkNyisTsheg),
            '࿓' => Ok(Tibetan::MarkInitialBrdaRnyingYigMgoMdunMa),
            '࿔' => Ok(Tibetan::MarkClosingBrdaRnyingYigMgoSgabMa),
            '࿕' => Ok(Tibetan::RightDashFacingSvastiSign),
            '࿖' => Ok(Tibetan::LeftDashFacingSvastiSign),
            '࿗' => Ok(Tibetan::RightDashFacingSvastiSignWithDots),
            '࿘' => Ok(Tibetan::LeftDashFacingSvastiSignWithDots),
            '࿙' => Ok(Tibetan::MarkLeadingMchanRtags),
            '࿚' => Ok(Tibetan::MarkTrailingMchanRtags),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Tibetan {
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

impl std::convert::TryFrom<u32> for Tibetan {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Tibetan {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Tibetan {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Tibetan::SyllableOm
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Tibetan{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
