
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

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Tibetan::SyllableOm => "tibetan syllable om",
            Tibetan::MarkGterYigMgoTruncatedA => "tibetan mark gter yig mgo truncated a",
            Tibetan::MarkGterYigMgoDashUmRnamBcadMa => "tibetan mark gter yig mgo -um rnam bcad ma",
            Tibetan::MarkGterYigMgoDashUmGterTshegMa => "tibetan mark gter yig mgo -um gter tsheg ma",
            Tibetan::MarkInitialYigMgoMdunMa => "tibetan mark initial yig mgo mdun ma",
            Tibetan::MarkClosingYigMgoSgabMa => "tibetan mark closing yig mgo sgab ma",
            Tibetan::MarkCaretYigMgoPhurShadMa => "tibetan mark caret yig mgo phur shad ma",
            Tibetan::MarkYigMgoTshegShadMa => "tibetan mark yig mgo tsheg shad ma",
            Tibetan::MarkSbrulShad => "tibetan mark sbrul shad",
            Tibetan::MarkBskurYigMgo => "tibetan mark bskur yig mgo",
            Tibetan::MarkBkaDashShogYigMgo => "tibetan mark bka- shog yig mgo",
            Tibetan::MarkIntersyllabicTsheg => "tibetan mark intersyllabic tsheg",
            Tibetan::MarkDelimiterTshegBstar => "tibetan mark delimiter tsheg bstar",
            Tibetan::MarkShad => "tibetan mark shad",
            Tibetan::MarkNyisShad => "tibetan mark nyis shad",
            Tibetan::MarkTshegShad => "tibetan mark tsheg shad",
            Tibetan::MarkNyisTshegShad => "tibetan mark nyis tsheg shad",
            Tibetan::MarkRinChenSpungsShad => "tibetan mark rin chen spungs shad",
            Tibetan::MarkRgyaGramShad => "tibetan mark rgya gram shad",
            Tibetan::MarkCaretDashDzudRtagsMeLongCan => "tibetan mark caret -dzud rtags me long can",
            Tibetan::MarkGterTsheg => "tibetan mark gter tsheg",
            Tibetan::LogotypeSignChadRtags => "tibetan logotype sign chad rtags",
            Tibetan::LogotypeSignLhagRtags => "tibetan logotype sign lhag rtags",
            Tibetan::AstrologicalSignSgraGcanDashCharRtags => "tibetan astrological sign sgra gcan -char rtags",
            Tibetan::AstrologicalSignDashKhyudPa => "tibetan astrological sign -khyud pa",
            Tibetan::AstrologicalSignSdongTshugs => "tibetan astrological sign sdong tshugs",
            Tibetan::SignRdelDkarGcig => "tibetan sign rdel dkar gcig",
            Tibetan::SignRdelDkarGnyis => "tibetan sign rdel dkar gnyis",
            Tibetan::SignRdelDkarGsum => "tibetan sign rdel dkar gsum",
            Tibetan::SignRdelNagGcig => "tibetan sign rdel nag gcig",
            Tibetan::SignRdelNagGnyis => "tibetan sign rdel nag gnyis",
            Tibetan::SignRdelDkarRdelNag => "tibetan sign rdel dkar rdel nag",
            Tibetan::DigitZero => "tibetan digit zero",
            Tibetan::DigitOne => "tibetan digit one",
            Tibetan::DigitTwo => "tibetan digit two",
            Tibetan::DigitThree => "tibetan digit three",
            Tibetan::DigitFour => "tibetan digit four",
            Tibetan::DigitFive => "tibetan digit five",
            Tibetan::DigitSix => "tibetan digit six",
            Tibetan::DigitSeven => "tibetan digit seven",
            Tibetan::DigitEight => "tibetan digit eight",
            Tibetan::DigitNine => "tibetan digit nine",
            Tibetan::DigitHalfOne => "tibetan digit half one",
            Tibetan::DigitHalfTwo => "tibetan digit half two",
            Tibetan::DigitHalfThree => "tibetan digit half three",
            Tibetan::DigitHalfFour => "tibetan digit half four",
            Tibetan::DigitHalfFive => "tibetan digit half five",
            Tibetan::DigitHalfSix => "tibetan digit half six",
            Tibetan::DigitHalfSeven => "tibetan digit half seven",
            Tibetan::DigitHalfEight => "tibetan digit half eight",
            Tibetan::DigitHalfNine => "tibetan digit half nine",
            Tibetan::DigitHalfZero => "tibetan digit half zero",
            Tibetan::MarkBsdusRtags => "tibetan mark bsdus rtags",
            Tibetan::MarkNgasBzungNyiZla => "tibetan mark ngas bzung nyi zla",
            Tibetan::MarkCaretDashDzudRtagsBzhiMigCan => "tibetan mark caret -dzud rtags bzhi mig can",
            Tibetan::MarkNgasBzungSgorRtags => "tibetan mark ngas bzung sgor rtags",
            Tibetan::MarkCheMgo => "tibetan mark che mgo",
            Tibetan::MarkTsaDashPhru => "tibetan mark tsa -phru",
            Tibetan::MarkGugRtagsGyon => "tibetan mark gug rtags gyon",
            Tibetan::MarkGugRtagsGyas => "tibetan mark gug rtags gyas",
            Tibetan::MarkAngKhangGyon => "tibetan mark ang khang gyon",
            Tibetan::MarkAngKhangGyas => "tibetan mark ang khang gyas",
            Tibetan::SignYarTshes => "tibetan sign yar tshes",
            Tibetan::SignMarTshes => "tibetan sign mar tshes",
            Tibetan::LetterKa => "tibetan letter ka",
            Tibetan::LetterKha => "tibetan letter kha",
            Tibetan::LetterGa => "tibetan letter ga",
            Tibetan::LetterGha => "tibetan letter gha",
            Tibetan::LetterNga => "tibetan letter nga",
            Tibetan::LetterCa => "tibetan letter ca",
            Tibetan::LetterCha => "tibetan letter cha",
            Tibetan::LetterJa => "tibetan letter ja",
            Tibetan::LetterNya => "tibetan letter nya",
            Tibetan::LetterTta => "tibetan letter tta",
            Tibetan::LetterTtha => "tibetan letter ttha",
            Tibetan::LetterDda => "tibetan letter dda",
            Tibetan::LetterDdha => "tibetan letter ddha",
            Tibetan::LetterNna => "tibetan letter nna",
            Tibetan::LetterTa => "tibetan letter ta",
            Tibetan::LetterTha => "tibetan letter tha",
            Tibetan::LetterDa => "tibetan letter da",
            Tibetan::LetterDha => "tibetan letter dha",
            Tibetan::LetterNa => "tibetan letter na",
            Tibetan::LetterPa => "tibetan letter pa",
            Tibetan::LetterPha => "tibetan letter pha",
            Tibetan::LetterBa => "tibetan letter ba",
            Tibetan::LetterBha => "tibetan letter bha",
            Tibetan::LetterMa => "tibetan letter ma",
            Tibetan::LetterTsa => "tibetan letter tsa",
            Tibetan::LetterTsha => "tibetan letter tsha",
            Tibetan::LetterDza => "tibetan letter dza",
            Tibetan::LetterDzha => "tibetan letter dzha",
            Tibetan::LetterWa => "tibetan letter wa",
            Tibetan::LetterZha => "tibetan letter zha",
            Tibetan::LetterZa => "tibetan letter za",
            Tibetan::LetterDashA => "tibetan letter -a",
            Tibetan::LetterYa => "tibetan letter ya",
            Tibetan::LetterRa => "tibetan letter ra",
            Tibetan::LetterLa => "tibetan letter la",
            Tibetan::LetterSha => "tibetan letter sha",
            Tibetan::LetterSsa => "tibetan letter ssa",
            Tibetan::LetterSa => "tibetan letter sa",
            Tibetan::LetterHa => "tibetan letter ha",
            Tibetan::LetterA => "tibetan letter a",
            Tibetan::LetterKssa => "tibetan letter kssa",
            Tibetan::LetterFixedDashFormRa => "tibetan letter fixed-form ra",
            Tibetan::LetterKka => "tibetan letter kka",
            Tibetan::LetterRra => "tibetan letter rra",
            Tibetan::VowelSignAa => "tibetan vowel sign aa",
            Tibetan::VowelSignI => "tibetan vowel sign i",
            Tibetan::VowelSignIi => "tibetan vowel sign ii",
            Tibetan::VowelSignU => "tibetan vowel sign u",
            Tibetan::VowelSignUu => "tibetan vowel sign uu",
            Tibetan::VowelSignVocalicR => "tibetan vowel sign vocalic r",
            Tibetan::VowelSignVocalicRr => "tibetan vowel sign vocalic rr",
            Tibetan::VowelSignVocalicL => "tibetan vowel sign vocalic l",
            Tibetan::VowelSignVocalicLl => "tibetan vowel sign vocalic ll",
            Tibetan::VowelSignE => "tibetan vowel sign e",
            Tibetan::VowelSignEe => "tibetan vowel sign ee",
            Tibetan::VowelSignO => "tibetan vowel sign o",
            Tibetan::VowelSignOo => "tibetan vowel sign oo",
            Tibetan::SignRjesSuNgaRo => "tibetan sign rjes su nga ro",
            Tibetan::SignRnamBcad => "tibetan sign rnam bcad",
            Tibetan::VowelSignReversedI => "tibetan vowel sign reversed i",
            Tibetan::VowelSignReversedIi => "tibetan vowel sign reversed ii",
            Tibetan::SignNyiZlaNaaDa => "tibetan sign nyi zla naa da",
            Tibetan::SignSnaLdan => "tibetan sign sna ldan",
            Tibetan::MarkHalanta => "tibetan mark halanta",
            Tibetan::MarkPaluta => "tibetan mark paluta",
            Tibetan::SignLciRtags => "tibetan sign lci rtags",
            Tibetan::SignYangRtags => "tibetan sign yang rtags",
            Tibetan::SignLceTsaCan => "tibetan sign lce tsa can",
            Tibetan::SignMchuCan => "tibetan sign mchu can",
            Tibetan::SignGruCanRgyings => "tibetan sign gru can rgyings",
            Tibetan::SignGruMedRgyings => "tibetan sign gru med rgyings",
            Tibetan::SignInvertedMchuCan => "tibetan sign inverted mchu can",
            Tibetan::SubjoinedSignLceTsaCan => "tibetan subjoined sign lce tsa can",
            Tibetan::SubjoinedSignMchuCan => "tibetan subjoined sign mchu can",
            Tibetan::SubjoinedSignInvertedMchuCan => "tibetan subjoined sign inverted mchu can",
            Tibetan::SubjoinedLetterKa => "tibetan subjoined letter ka",
            Tibetan::SubjoinedLetterKha => "tibetan subjoined letter kha",
            Tibetan::SubjoinedLetterGa => "tibetan subjoined letter ga",
            Tibetan::SubjoinedLetterGha => "tibetan subjoined letter gha",
            Tibetan::SubjoinedLetterNga => "tibetan subjoined letter nga",
            Tibetan::SubjoinedLetterCa => "tibetan subjoined letter ca",
            Tibetan::SubjoinedLetterCha => "tibetan subjoined letter cha",
            Tibetan::SubjoinedLetterJa => "tibetan subjoined letter ja",
            Tibetan::SubjoinedLetterNya => "tibetan subjoined letter nya",
            Tibetan::SubjoinedLetterTta => "tibetan subjoined letter tta",
            Tibetan::SubjoinedLetterTtha => "tibetan subjoined letter ttha",
            Tibetan::SubjoinedLetterDda => "tibetan subjoined letter dda",
            Tibetan::SubjoinedLetterDdha => "tibetan subjoined letter ddha",
            Tibetan::SubjoinedLetterNna => "tibetan subjoined letter nna",
            Tibetan::SubjoinedLetterTa => "tibetan subjoined letter ta",
            Tibetan::SubjoinedLetterTha => "tibetan subjoined letter tha",
            Tibetan::SubjoinedLetterDa => "tibetan subjoined letter da",
            Tibetan::SubjoinedLetterDha => "tibetan subjoined letter dha",
            Tibetan::SubjoinedLetterNa => "tibetan subjoined letter na",
            Tibetan::SubjoinedLetterPa => "tibetan subjoined letter pa",
            Tibetan::SubjoinedLetterPha => "tibetan subjoined letter pha",
            Tibetan::SubjoinedLetterBa => "tibetan subjoined letter ba",
            Tibetan::SubjoinedLetterBha => "tibetan subjoined letter bha",
            Tibetan::SubjoinedLetterMa => "tibetan subjoined letter ma",
            Tibetan::SubjoinedLetterTsa => "tibetan subjoined letter tsa",
            Tibetan::SubjoinedLetterTsha => "tibetan subjoined letter tsha",
            Tibetan::SubjoinedLetterDza => "tibetan subjoined letter dza",
            Tibetan::SubjoinedLetterDzha => "tibetan subjoined letter dzha",
            Tibetan::SubjoinedLetterWa => "tibetan subjoined letter wa",
            Tibetan::SubjoinedLetterZha => "tibetan subjoined letter zha",
            Tibetan::SubjoinedLetterZa => "tibetan subjoined letter za",
            Tibetan::SubjoinedLetterDashA => "tibetan subjoined letter -a",
            Tibetan::SubjoinedLetterYa => "tibetan subjoined letter ya",
            Tibetan::SubjoinedLetterRa => "tibetan subjoined letter ra",
            Tibetan::SubjoinedLetterLa => "tibetan subjoined letter la",
            Tibetan::SubjoinedLetterSha => "tibetan subjoined letter sha",
            Tibetan::SubjoinedLetterSsa => "tibetan subjoined letter ssa",
            Tibetan::SubjoinedLetterSa => "tibetan subjoined letter sa",
            Tibetan::SubjoinedLetterHa => "tibetan subjoined letter ha",
            Tibetan::SubjoinedLetterA => "tibetan subjoined letter a",
            Tibetan::SubjoinedLetterKssa => "tibetan subjoined letter kssa",
            Tibetan::SubjoinedLetterFixedDashFormWa => "tibetan subjoined letter fixed-form wa",
            Tibetan::SubjoinedLetterFixedDashFormYa => "tibetan subjoined letter fixed-form ya",
            Tibetan::SubjoinedLetterFixedDashFormRa => "tibetan subjoined letter fixed-form ra",
            Tibetan::KuRuKha => "tibetan ku ru kha",
            Tibetan::KuRuKhaBzhiMigCan => "tibetan ku ru kha bzhi mig can",
            Tibetan::CantillationSignHeavyBeat => "tibetan cantillation sign heavy beat",
            Tibetan::CantillationSignLightBeat => "tibetan cantillation sign light beat",
            Tibetan::CantillationSignCangTeDashU => "tibetan cantillation sign cang te-u",
            Tibetan::CantillationSignSbubDashChal => "tibetan cantillation sign sbub -chal",
            Tibetan::SymbolDrilBu => "tibetan symbol dril bu",
            Tibetan::SymbolRdoRje => "tibetan symbol rdo rje",
            Tibetan::SymbolPadmaGdan => "tibetan symbol padma gdan",
            Tibetan::SymbolRdoRjeRgyaGram => "tibetan symbol rdo rje rgya gram",
            Tibetan::SymbolPhurPa => "tibetan symbol phur pa",
            Tibetan::SymbolNorBu => "tibetan symbol nor bu",
            Tibetan::SymbolNorBuNyisDashKhyil => "tibetan symbol nor bu nyis -khyil",
            Tibetan::SymbolNorBuGsumDashKhyil => "tibetan symbol nor bu gsum -khyil",
            Tibetan::SymbolNorBuBzhiDashKhyil => "tibetan symbol nor bu bzhi -khyil",
            Tibetan::SignRdelNagRdelDkar => "tibetan sign rdel nag rdel dkar",
            Tibetan::SignRdelNagGsum => "tibetan sign rdel nag gsum",
            Tibetan::MarkBskaDashShogGiMgoRgyan => "tibetan mark bska- shog gi mgo rgyan",
            Tibetan::MarkMnyamYigGiMgoRgyan => "tibetan mark mnyam yig gi mgo rgyan",
            Tibetan::MarkNyisTsheg => "tibetan mark nyis tsheg",
            Tibetan::MarkInitialBrdaRnyingYigMgoMdunMa => "tibetan mark initial brda rnying yig mgo mdun ma",
            Tibetan::MarkClosingBrdaRnyingYigMgoSgabMa => "tibetan mark closing brda rnying yig mgo sgab ma",
            Tibetan::RightDashFacingSvastiSign => "right-facing svasti sign",
            Tibetan::LeftDashFacingSvastiSign => "left-facing svasti sign",
            Tibetan::RightDashFacingSvastiSignWithDots => "right-facing svasti sign with dots",
            Tibetan::LeftDashFacingSvastiSignWithDots => "left-facing svasti sign with dots",
            Tibetan::MarkLeadingMchanRtags => "tibetan mark leading mchan rtags",
            Tibetan::MarkTrailingMchanRtags => "tibetan mark trailing mchan rtags",
        }
    }
}
