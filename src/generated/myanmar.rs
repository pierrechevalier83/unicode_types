
/// An enum to represent all characters in the Myanmar block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Myanmar {
    /// \u{1000}: 'က'
    LetterKa,
    /// \u{1001}: 'ခ'
    LetterKha,
    /// \u{1002}: 'ဂ'
    LetterGa,
    /// \u{1003}: 'ဃ'
    LetterGha,
    /// \u{1004}: 'င'
    LetterNga,
    /// \u{1005}: 'စ'
    LetterCa,
    /// \u{1006}: 'ဆ'
    LetterCha,
    /// \u{1007}: 'ဇ'
    LetterJa,
    /// \u{1008}: 'ဈ'
    LetterJha,
    /// \u{1009}: 'ဉ'
    LetterNya,
    /// \u{100a}: 'ည'
    LetterNnya,
    /// \u{100b}: 'ဋ'
    LetterTta,
    /// \u{100c}: 'ဌ'
    LetterTtha,
    /// \u{100d}: 'ဍ'
    LetterDda,
    /// \u{100e}: 'ဎ'
    LetterDdha,
    /// \u{100f}: 'ဏ'
    LetterNna,
    /// \u{1010}: 'တ'
    LetterTa,
    /// \u{1011}: 'ထ'
    LetterTha,
    /// \u{1012}: 'ဒ'
    LetterDa,
    /// \u{1013}: 'ဓ'
    LetterDha,
    /// \u{1014}: 'န'
    LetterNa,
    /// \u{1015}: 'ပ'
    LetterPa,
    /// \u{1016}: 'ဖ'
    LetterPha,
    /// \u{1017}: 'ဗ'
    LetterBa,
    /// \u{1018}: 'ဘ'
    LetterBha,
    /// \u{1019}: 'မ'
    LetterMa,
    /// \u{101a}: 'ယ'
    LetterYa,
    /// \u{101b}: 'ရ'
    LetterRa,
    /// \u{101c}: 'လ'
    LetterLa,
    /// \u{101d}: 'ဝ'
    LetterWa,
    /// \u{101e}: 'သ'
    LetterSa,
    /// \u{101f}: 'ဟ'
    LetterHa,
    /// \u{1020}: 'ဠ'
    LetterLla,
    /// \u{1021}: 'အ'
    LetterA,
    /// \u{1022}: 'ဢ'
    LetterShanA,
    /// \u{1023}: 'ဣ'
    LetterI,
    /// \u{1024}: 'ဤ'
    LetterIi,
    /// \u{1025}: 'ဥ'
    LetterU,
    /// \u{1026}: 'ဦ'
    LetterUu,
    /// \u{1027}: 'ဧ'
    LetterE,
    /// \u{1028}: 'ဨ'
    LetterMonE,
    /// \u{1029}: 'ဩ'
    LetterO,
    /// \u{102a}: 'ဪ'
    LetterAu,
    /// \u{102b}: 'ါ'
    VowelSignTallAa,
    /// \u{102c}: 'ာ'
    VowelSignAa,
    /// \u{102d}: 'ိ'
    VowelSignI,
    /// \u{102e}: 'ီ'
    VowelSignIi,
    /// \u{102f}: 'ု'
    VowelSignU,
    /// \u{1030}: 'ူ'
    VowelSignUu,
    /// \u{1031}: 'ေ'
    VowelSignE,
    /// \u{1032}: 'ဲ'
    VowelSignAi,
    /// \u{1033}: 'ဳ'
    VowelSignMonIi,
    /// \u{1034}: 'ဴ'
    VowelSignMonO,
    /// \u{1035}: 'ဵ'
    VowelSignEAbove,
    /// \u{1036}: 'ံ'
    SignAnusvara,
    /// \u{1037}: '့'
    SignDotBelow,
    /// \u{1038}: 'း'
    SignVisarga,
    /// \u{1039}: '္'
    SignVirama,
    /// \u{103a}: '်'
    SignAsat,
    /// \u{103b}: 'ျ'
    ConsonantSignMedialYa,
    /// \u{103c}: 'ြ'
    ConsonantSignMedialRa,
    /// \u{103d}: 'ွ'
    ConsonantSignMedialWa,
    /// \u{103e}: 'ှ'
    ConsonantSignMedialHa,
    /// \u{103f}: 'ဿ'
    LetterGreatSa,
    /// \u{1040}: '၀'
    DigitZero,
    /// \u{1041}: '၁'
    DigitOne,
    /// \u{1042}: '၂'
    DigitTwo,
    /// \u{1043}: '၃'
    DigitThree,
    /// \u{1044}: '၄'
    DigitFour,
    /// \u{1045}: '၅'
    DigitFive,
    /// \u{1046}: '၆'
    DigitSix,
    /// \u{1047}: '၇'
    DigitSeven,
    /// \u{1048}: '၈'
    DigitEight,
    /// \u{1049}: '၉'
    DigitNine,
    /// \u{104a}: '၊'
    SignLittleSection,
    /// \u{104b}: '။'
    SignSection,
    /// \u{104c}: '၌'
    SymbolLocative,
    /// \u{104d}: '၍'
    SymbolCompleted,
    /// \u{104e}: '၎'
    SymbolAforementioned,
    /// \u{104f}: '၏'
    SymbolGenitive,
    /// \u{1050}: 'ၐ'
    LetterSha,
    /// \u{1051}: 'ၑ'
    LetterSsa,
    /// \u{1052}: 'ၒ'
    LetterVocalicR,
    /// \u{1053}: 'ၓ'
    LetterVocalicRr,
    /// \u{1054}: 'ၔ'
    LetterVocalicL,
    /// \u{1055}: 'ၕ'
    LetterVocalicLl,
    /// \u{1056}: 'ၖ'
    VowelSignVocalicR,
    /// \u{1057}: 'ၗ'
    VowelSignVocalicRr,
    /// \u{1058}: 'ၘ'
    VowelSignVocalicL,
    /// \u{1059}: 'ၙ'
    VowelSignVocalicLl,
    /// \u{105a}: 'ၚ'
    LetterMonNga,
    /// \u{105b}: 'ၛ'
    LetterMonJha,
    /// \u{105c}: 'ၜ'
    LetterMonBba,
    /// \u{105d}: 'ၝ'
    LetterMonBbe,
    /// \u{105e}: 'ၞ'
    ConsonantSignMonMedialNa,
    /// \u{105f}: 'ၟ'
    ConsonantSignMonMedialMa,
    /// \u{1060}: 'ၠ'
    ConsonantSignMonMedialLa,
    /// \u{1061}: 'ၡ'
    LetterSgawKarenSha,
    /// \u{1062}: 'ၢ'
    VowelSignSgawKarenEu,
    /// \u{1063}: 'ၣ'
    ToneMarkSgawKarenHathi,
    /// \u{1064}: 'ၤ'
    ToneMarkSgawKarenKePho,
    /// \u{1065}: 'ၥ'
    LetterWesternPwoKarenTha,
    /// \u{1066}: 'ၦ'
    LetterWesternPwoKarenPwa,
    /// \u{1067}: 'ၧ'
    VowelSignWesternPwoKarenEu,
    /// \u{1068}: 'ၨ'
    VowelSignWesternPwoKarenUe,
    /// \u{1069}: 'ၩ'
    SignWesternPwoKarenToneDash1,
    /// \u{106a}: 'ၪ'
    SignWesternPwoKarenToneDash2,
    /// \u{106b}: 'ၫ'
    SignWesternPwoKarenToneDash3,
    /// \u{106c}: 'ၬ'
    SignWesternPwoKarenToneDash4,
    /// \u{106d}: 'ၭ'
    SignWesternPwoKarenToneDash5,
    /// \u{106e}: 'ၮ'
    LetterEasternPwoKarenNna,
    /// \u{106f}: 'ၯ'
    LetterEasternPwoKarenYwa,
    /// \u{1070}: 'ၰ'
    LetterEasternPwoKarenGhwa,
    /// \u{1071}: 'ၱ'
    VowelSignGebaKarenI,
    /// \u{1072}: 'ၲ'
    VowelSignKayahOe,
    /// \u{1073}: 'ၳ'
    VowelSignKayahU,
    /// \u{1074}: 'ၴ'
    VowelSignKayahEe,
    /// \u{1075}: 'ၵ'
    LetterShanKa,
    /// \u{1076}: 'ၶ'
    LetterShanKha,
    /// \u{1077}: 'ၷ'
    LetterShanGa,
    /// \u{1078}: 'ၸ'
    LetterShanCa,
    /// \u{1079}: 'ၹ'
    LetterShanZa,
    /// \u{107a}: 'ၺ'
    LetterShanNya,
    /// \u{107b}: 'ၻ'
    LetterShanDa,
    /// \u{107c}: 'ၼ'
    LetterShanNa,
    /// \u{107d}: 'ၽ'
    LetterShanPha,
    /// \u{107e}: 'ၾ'
    LetterShanFa,
    /// \u{107f}: 'ၿ'
    LetterShanBa,
    /// \u{1080}: 'ႀ'
    LetterShanTha,
    /// \u{1081}: 'ႁ'
    LetterShanHa,
    /// \u{1082}: 'ႂ'
    ConsonantSignShanMedialWa,
    /// \u{1083}: 'ႃ'
    VowelSignShanAa,
    /// \u{1084}: 'ႄ'
    VowelSignShanE,
    /// \u{1085}: 'ႅ'
    VowelSignShanEAbove,
    /// \u{1086}: 'ႆ'
    VowelSignShanFinalY,
    /// \u{1087}: 'ႇ'
    SignShanToneDash2,
    /// \u{1088}: 'ႈ'
    SignShanToneDash3,
    /// \u{1089}: 'ႉ'
    SignShanToneDash5,
    /// \u{108a}: 'ႊ'
    SignShanToneDash6,
    /// \u{108b}: 'ႋ'
    SignShanCouncilToneDash2,
    /// \u{108c}: 'ႌ'
    SignShanCouncilToneDash3,
    /// \u{108d}: 'ႍ'
    SignShanCouncilEmphaticTone,
    /// \u{108e}: 'ႎ'
    LetterRumaiPalaungFa,
    /// \u{108f}: 'ႏ'
    SignRumaiPalaungToneDash5,
    /// \u{1090}: '႐'
    ShanDigitZero,
    /// \u{1091}: '႑'
    ShanDigitOne,
    /// \u{1092}: '႒'
    ShanDigitTwo,
    /// \u{1093}: '႓'
    ShanDigitThree,
    /// \u{1094}: '႔'
    ShanDigitFour,
    /// \u{1095}: '႕'
    ShanDigitFive,
    /// \u{1096}: '႖'
    ShanDigitSix,
    /// \u{1097}: '႗'
    ShanDigitSeven,
    /// \u{1098}: '႘'
    ShanDigitEight,
    /// \u{1099}: '႙'
    ShanDigitNine,
    /// \u{109a}: 'ႚ'
    SignKhamtiToneDash1,
    /// \u{109b}: 'ႛ'
    SignKhamtiToneDash3,
    /// \u{109c}: 'ႜ'
    VowelSignAitonA,
    /// \u{109d}: 'ႝ'
    VowelSignAitonAi,
    /// \u{109e}: '႞'
    SymbolShanOne,
}

impl Into<char> for Myanmar {
    fn into(self) -> char {
        match self {
            Myanmar::LetterKa => 'က',
            Myanmar::LetterKha => 'ခ',
            Myanmar::LetterGa => 'ဂ',
            Myanmar::LetterGha => 'ဃ',
            Myanmar::LetterNga => 'င',
            Myanmar::LetterCa => 'စ',
            Myanmar::LetterCha => 'ဆ',
            Myanmar::LetterJa => 'ဇ',
            Myanmar::LetterJha => 'ဈ',
            Myanmar::LetterNya => 'ဉ',
            Myanmar::LetterNnya => 'ည',
            Myanmar::LetterTta => 'ဋ',
            Myanmar::LetterTtha => 'ဌ',
            Myanmar::LetterDda => 'ဍ',
            Myanmar::LetterDdha => 'ဎ',
            Myanmar::LetterNna => 'ဏ',
            Myanmar::LetterTa => 'တ',
            Myanmar::LetterTha => 'ထ',
            Myanmar::LetterDa => 'ဒ',
            Myanmar::LetterDha => 'ဓ',
            Myanmar::LetterNa => 'န',
            Myanmar::LetterPa => 'ပ',
            Myanmar::LetterPha => 'ဖ',
            Myanmar::LetterBa => 'ဗ',
            Myanmar::LetterBha => 'ဘ',
            Myanmar::LetterMa => 'မ',
            Myanmar::LetterYa => 'ယ',
            Myanmar::LetterRa => 'ရ',
            Myanmar::LetterLa => 'လ',
            Myanmar::LetterWa => 'ဝ',
            Myanmar::LetterSa => 'သ',
            Myanmar::LetterHa => 'ဟ',
            Myanmar::LetterLla => 'ဠ',
            Myanmar::LetterA => 'အ',
            Myanmar::LetterShanA => 'ဢ',
            Myanmar::LetterI => 'ဣ',
            Myanmar::LetterIi => 'ဤ',
            Myanmar::LetterU => 'ဥ',
            Myanmar::LetterUu => 'ဦ',
            Myanmar::LetterE => 'ဧ',
            Myanmar::LetterMonE => 'ဨ',
            Myanmar::LetterO => 'ဩ',
            Myanmar::LetterAu => 'ဪ',
            Myanmar::VowelSignTallAa => 'ါ',
            Myanmar::VowelSignAa => 'ာ',
            Myanmar::VowelSignI => 'ိ',
            Myanmar::VowelSignIi => 'ီ',
            Myanmar::VowelSignU => 'ု',
            Myanmar::VowelSignUu => 'ူ',
            Myanmar::VowelSignE => 'ေ',
            Myanmar::VowelSignAi => 'ဲ',
            Myanmar::VowelSignMonIi => 'ဳ',
            Myanmar::VowelSignMonO => 'ဴ',
            Myanmar::VowelSignEAbove => 'ဵ',
            Myanmar::SignAnusvara => 'ံ',
            Myanmar::SignDotBelow => '့',
            Myanmar::SignVisarga => 'း',
            Myanmar::SignVirama => '္',
            Myanmar::SignAsat => '်',
            Myanmar::ConsonantSignMedialYa => 'ျ',
            Myanmar::ConsonantSignMedialRa => 'ြ',
            Myanmar::ConsonantSignMedialWa => 'ွ',
            Myanmar::ConsonantSignMedialHa => 'ှ',
            Myanmar::LetterGreatSa => 'ဿ',
            Myanmar::DigitZero => '၀',
            Myanmar::DigitOne => '၁',
            Myanmar::DigitTwo => '၂',
            Myanmar::DigitThree => '၃',
            Myanmar::DigitFour => '၄',
            Myanmar::DigitFive => '၅',
            Myanmar::DigitSix => '၆',
            Myanmar::DigitSeven => '၇',
            Myanmar::DigitEight => '၈',
            Myanmar::DigitNine => '၉',
            Myanmar::SignLittleSection => '၊',
            Myanmar::SignSection => '။',
            Myanmar::SymbolLocative => '၌',
            Myanmar::SymbolCompleted => '၍',
            Myanmar::SymbolAforementioned => '၎',
            Myanmar::SymbolGenitive => '၏',
            Myanmar::LetterSha => 'ၐ',
            Myanmar::LetterSsa => 'ၑ',
            Myanmar::LetterVocalicR => 'ၒ',
            Myanmar::LetterVocalicRr => 'ၓ',
            Myanmar::LetterVocalicL => 'ၔ',
            Myanmar::LetterVocalicLl => 'ၕ',
            Myanmar::VowelSignVocalicR => 'ၖ',
            Myanmar::VowelSignVocalicRr => 'ၗ',
            Myanmar::VowelSignVocalicL => 'ၘ',
            Myanmar::VowelSignVocalicLl => 'ၙ',
            Myanmar::LetterMonNga => 'ၚ',
            Myanmar::LetterMonJha => 'ၛ',
            Myanmar::LetterMonBba => 'ၜ',
            Myanmar::LetterMonBbe => 'ၝ',
            Myanmar::ConsonantSignMonMedialNa => 'ၞ',
            Myanmar::ConsonantSignMonMedialMa => 'ၟ',
            Myanmar::ConsonantSignMonMedialLa => 'ၠ',
            Myanmar::LetterSgawKarenSha => 'ၡ',
            Myanmar::VowelSignSgawKarenEu => 'ၢ',
            Myanmar::ToneMarkSgawKarenHathi => 'ၣ',
            Myanmar::ToneMarkSgawKarenKePho => 'ၤ',
            Myanmar::LetterWesternPwoKarenTha => 'ၥ',
            Myanmar::LetterWesternPwoKarenPwa => 'ၦ',
            Myanmar::VowelSignWesternPwoKarenEu => 'ၧ',
            Myanmar::VowelSignWesternPwoKarenUe => 'ၨ',
            Myanmar::SignWesternPwoKarenToneDash1 => 'ၩ',
            Myanmar::SignWesternPwoKarenToneDash2 => 'ၪ',
            Myanmar::SignWesternPwoKarenToneDash3 => 'ၫ',
            Myanmar::SignWesternPwoKarenToneDash4 => 'ၬ',
            Myanmar::SignWesternPwoKarenToneDash5 => 'ၭ',
            Myanmar::LetterEasternPwoKarenNna => 'ၮ',
            Myanmar::LetterEasternPwoKarenYwa => 'ၯ',
            Myanmar::LetterEasternPwoKarenGhwa => 'ၰ',
            Myanmar::VowelSignGebaKarenI => 'ၱ',
            Myanmar::VowelSignKayahOe => 'ၲ',
            Myanmar::VowelSignKayahU => 'ၳ',
            Myanmar::VowelSignKayahEe => 'ၴ',
            Myanmar::LetterShanKa => 'ၵ',
            Myanmar::LetterShanKha => 'ၶ',
            Myanmar::LetterShanGa => 'ၷ',
            Myanmar::LetterShanCa => 'ၸ',
            Myanmar::LetterShanZa => 'ၹ',
            Myanmar::LetterShanNya => 'ၺ',
            Myanmar::LetterShanDa => 'ၻ',
            Myanmar::LetterShanNa => 'ၼ',
            Myanmar::LetterShanPha => 'ၽ',
            Myanmar::LetterShanFa => 'ၾ',
            Myanmar::LetterShanBa => 'ၿ',
            Myanmar::LetterShanTha => 'ႀ',
            Myanmar::LetterShanHa => 'ႁ',
            Myanmar::ConsonantSignShanMedialWa => 'ႂ',
            Myanmar::VowelSignShanAa => 'ႃ',
            Myanmar::VowelSignShanE => 'ႄ',
            Myanmar::VowelSignShanEAbove => 'ႅ',
            Myanmar::VowelSignShanFinalY => 'ႆ',
            Myanmar::SignShanToneDash2 => 'ႇ',
            Myanmar::SignShanToneDash3 => 'ႈ',
            Myanmar::SignShanToneDash5 => 'ႉ',
            Myanmar::SignShanToneDash6 => 'ႊ',
            Myanmar::SignShanCouncilToneDash2 => 'ႋ',
            Myanmar::SignShanCouncilToneDash3 => 'ႌ',
            Myanmar::SignShanCouncilEmphaticTone => 'ႍ',
            Myanmar::LetterRumaiPalaungFa => 'ႎ',
            Myanmar::SignRumaiPalaungToneDash5 => 'ႏ',
            Myanmar::ShanDigitZero => '႐',
            Myanmar::ShanDigitOne => '႑',
            Myanmar::ShanDigitTwo => '႒',
            Myanmar::ShanDigitThree => '႓',
            Myanmar::ShanDigitFour => '႔',
            Myanmar::ShanDigitFive => '႕',
            Myanmar::ShanDigitSix => '႖',
            Myanmar::ShanDigitSeven => '႗',
            Myanmar::ShanDigitEight => '႘',
            Myanmar::ShanDigitNine => '႙',
            Myanmar::SignKhamtiToneDash1 => 'ႚ',
            Myanmar::SignKhamtiToneDash3 => 'ႛ',
            Myanmar::VowelSignAitonA => 'ႜ',
            Myanmar::VowelSignAitonAi => 'ႝ',
            Myanmar::SymbolShanOne => '႞',
        }
    }
}

impl std::convert::TryFrom<char> for Myanmar {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'က' => Ok(Myanmar::LetterKa),
            'ခ' => Ok(Myanmar::LetterKha),
            'ဂ' => Ok(Myanmar::LetterGa),
            'ဃ' => Ok(Myanmar::LetterGha),
            'င' => Ok(Myanmar::LetterNga),
            'စ' => Ok(Myanmar::LetterCa),
            'ဆ' => Ok(Myanmar::LetterCha),
            'ဇ' => Ok(Myanmar::LetterJa),
            'ဈ' => Ok(Myanmar::LetterJha),
            'ဉ' => Ok(Myanmar::LetterNya),
            'ည' => Ok(Myanmar::LetterNnya),
            'ဋ' => Ok(Myanmar::LetterTta),
            'ဌ' => Ok(Myanmar::LetterTtha),
            'ဍ' => Ok(Myanmar::LetterDda),
            'ဎ' => Ok(Myanmar::LetterDdha),
            'ဏ' => Ok(Myanmar::LetterNna),
            'တ' => Ok(Myanmar::LetterTa),
            'ထ' => Ok(Myanmar::LetterTha),
            'ဒ' => Ok(Myanmar::LetterDa),
            'ဓ' => Ok(Myanmar::LetterDha),
            'န' => Ok(Myanmar::LetterNa),
            'ပ' => Ok(Myanmar::LetterPa),
            'ဖ' => Ok(Myanmar::LetterPha),
            'ဗ' => Ok(Myanmar::LetterBa),
            'ဘ' => Ok(Myanmar::LetterBha),
            'မ' => Ok(Myanmar::LetterMa),
            'ယ' => Ok(Myanmar::LetterYa),
            'ရ' => Ok(Myanmar::LetterRa),
            'လ' => Ok(Myanmar::LetterLa),
            'ဝ' => Ok(Myanmar::LetterWa),
            'သ' => Ok(Myanmar::LetterSa),
            'ဟ' => Ok(Myanmar::LetterHa),
            'ဠ' => Ok(Myanmar::LetterLla),
            'အ' => Ok(Myanmar::LetterA),
            'ဢ' => Ok(Myanmar::LetterShanA),
            'ဣ' => Ok(Myanmar::LetterI),
            'ဤ' => Ok(Myanmar::LetterIi),
            'ဥ' => Ok(Myanmar::LetterU),
            'ဦ' => Ok(Myanmar::LetterUu),
            'ဧ' => Ok(Myanmar::LetterE),
            'ဨ' => Ok(Myanmar::LetterMonE),
            'ဩ' => Ok(Myanmar::LetterO),
            'ဪ' => Ok(Myanmar::LetterAu),
            'ါ' => Ok(Myanmar::VowelSignTallAa),
            'ာ' => Ok(Myanmar::VowelSignAa),
            'ိ' => Ok(Myanmar::VowelSignI),
            'ီ' => Ok(Myanmar::VowelSignIi),
            'ု' => Ok(Myanmar::VowelSignU),
            'ူ' => Ok(Myanmar::VowelSignUu),
            'ေ' => Ok(Myanmar::VowelSignE),
            'ဲ' => Ok(Myanmar::VowelSignAi),
            'ဳ' => Ok(Myanmar::VowelSignMonIi),
            'ဴ' => Ok(Myanmar::VowelSignMonO),
            'ဵ' => Ok(Myanmar::VowelSignEAbove),
            'ံ' => Ok(Myanmar::SignAnusvara),
            '့' => Ok(Myanmar::SignDotBelow),
            'း' => Ok(Myanmar::SignVisarga),
            '္' => Ok(Myanmar::SignVirama),
            '်' => Ok(Myanmar::SignAsat),
            'ျ' => Ok(Myanmar::ConsonantSignMedialYa),
            'ြ' => Ok(Myanmar::ConsonantSignMedialRa),
            'ွ' => Ok(Myanmar::ConsonantSignMedialWa),
            'ှ' => Ok(Myanmar::ConsonantSignMedialHa),
            'ဿ' => Ok(Myanmar::LetterGreatSa),
            '၀' => Ok(Myanmar::DigitZero),
            '၁' => Ok(Myanmar::DigitOne),
            '၂' => Ok(Myanmar::DigitTwo),
            '၃' => Ok(Myanmar::DigitThree),
            '၄' => Ok(Myanmar::DigitFour),
            '၅' => Ok(Myanmar::DigitFive),
            '၆' => Ok(Myanmar::DigitSix),
            '၇' => Ok(Myanmar::DigitSeven),
            '၈' => Ok(Myanmar::DigitEight),
            '၉' => Ok(Myanmar::DigitNine),
            '၊' => Ok(Myanmar::SignLittleSection),
            '။' => Ok(Myanmar::SignSection),
            '၌' => Ok(Myanmar::SymbolLocative),
            '၍' => Ok(Myanmar::SymbolCompleted),
            '၎' => Ok(Myanmar::SymbolAforementioned),
            '၏' => Ok(Myanmar::SymbolGenitive),
            'ၐ' => Ok(Myanmar::LetterSha),
            'ၑ' => Ok(Myanmar::LetterSsa),
            'ၒ' => Ok(Myanmar::LetterVocalicR),
            'ၓ' => Ok(Myanmar::LetterVocalicRr),
            'ၔ' => Ok(Myanmar::LetterVocalicL),
            'ၕ' => Ok(Myanmar::LetterVocalicLl),
            'ၖ' => Ok(Myanmar::VowelSignVocalicR),
            'ၗ' => Ok(Myanmar::VowelSignVocalicRr),
            'ၘ' => Ok(Myanmar::VowelSignVocalicL),
            'ၙ' => Ok(Myanmar::VowelSignVocalicLl),
            'ၚ' => Ok(Myanmar::LetterMonNga),
            'ၛ' => Ok(Myanmar::LetterMonJha),
            'ၜ' => Ok(Myanmar::LetterMonBba),
            'ၝ' => Ok(Myanmar::LetterMonBbe),
            'ၞ' => Ok(Myanmar::ConsonantSignMonMedialNa),
            'ၟ' => Ok(Myanmar::ConsonantSignMonMedialMa),
            'ၠ' => Ok(Myanmar::ConsonantSignMonMedialLa),
            'ၡ' => Ok(Myanmar::LetterSgawKarenSha),
            'ၢ' => Ok(Myanmar::VowelSignSgawKarenEu),
            'ၣ' => Ok(Myanmar::ToneMarkSgawKarenHathi),
            'ၤ' => Ok(Myanmar::ToneMarkSgawKarenKePho),
            'ၥ' => Ok(Myanmar::LetterWesternPwoKarenTha),
            'ၦ' => Ok(Myanmar::LetterWesternPwoKarenPwa),
            'ၧ' => Ok(Myanmar::VowelSignWesternPwoKarenEu),
            'ၨ' => Ok(Myanmar::VowelSignWesternPwoKarenUe),
            'ၩ' => Ok(Myanmar::SignWesternPwoKarenToneDash1),
            'ၪ' => Ok(Myanmar::SignWesternPwoKarenToneDash2),
            'ၫ' => Ok(Myanmar::SignWesternPwoKarenToneDash3),
            'ၬ' => Ok(Myanmar::SignWesternPwoKarenToneDash4),
            'ၭ' => Ok(Myanmar::SignWesternPwoKarenToneDash5),
            'ၮ' => Ok(Myanmar::LetterEasternPwoKarenNna),
            'ၯ' => Ok(Myanmar::LetterEasternPwoKarenYwa),
            'ၰ' => Ok(Myanmar::LetterEasternPwoKarenGhwa),
            'ၱ' => Ok(Myanmar::VowelSignGebaKarenI),
            'ၲ' => Ok(Myanmar::VowelSignKayahOe),
            'ၳ' => Ok(Myanmar::VowelSignKayahU),
            'ၴ' => Ok(Myanmar::VowelSignKayahEe),
            'ၵ' => Ok(Myanmar::LetterShanKa),
            'ၶ' => Ok(Myanmar::LetterShanKha),
            'ၷ' => Ok(Myanmar::LetterShanGa),
            'ၸ' => Ok(Myanmar::LetterShanCa),
            'ၹ' => Ok(Myanmar::LetterShanZa),
            'ၺ' => Ok(Myanmar::LetterShanNya),
            'ၻ' => Ok(Myanmar::LetterShanDa),
            'ၼ' => Ok(Myanmar::LetterShanNa),
            'ၽ' => Ok(Myanmar::LetterShanPha),
            'ၾ' => Ok(Myanmar::LetterShanFa),
            'ၿ' => Ok(Myanmar::LetterShanBa),
            'ႀ' => Ok(Myanmar::LetterShanTha),
            'ႁ' => Ok(Myanmar::LetterShanHa),
            'ႂ' => Ok(Myanmar::ConsonantSignShanMedialWa),
            'ႃ' => Ok(Myanmar::VowelSignShanAa),
            'ႄ' => Ok(Myanmar::VowelSignShanE),
            'ႅ' => Ok(Myanmar::VowelSignShanEAbove),
            'ႆ' => Ok(Myanmar::VowelSignShanFinalY),
            'ႇ' => Ok(Myanmar::SignShanToneDash2),
            'ႈ' => Ok(Myanmar::SignShanToneDash3),
            'ႉ' => Ok(Myanmar::SignShanToneDash5),
            'ႊ' => Ok(Myanmar::SignShanToneDash6),
            'ႋ' => Ok(Myanmar::SignShanCouncilToneDash2),
            'ႌ' => Ok(Myanmar::SignShanCouncilToneDash3),
            'ႍ' => Ok(Myanmar::SignShanCouncilEmphaticTone),
            'ႎ' => Ok(Myanmar::LetterRumaiPalaungFa),
            'ႏ' => Ok(Myanmar::SignRumaiPalaungToneDash5),
            '႐' => Ok(Myanmar::ShanDigitZero),
            '႑' => Ok(Myanmar::ShanDigitOne),
            '႒' => Ok(Myanmar::ShanDigitTwo),
            '႓' => Ok(Myanmar::ShanDigitThree),
            '႔' => Ok(Myanmar::ShanDigitFour),
            '႕' => Ok(Myanmar::ShanDigitFive),
            '႖' => Ok(Myanmar::ShanDigitSix),
            '႗' => Ok(Myanmar::ShanDigitSeven),
            '႘' => Ok(Myanmar::ShanDigitEight),
            '႙' => Ok(Myanmar::ShanDigitNine),
            'ႚ' => Ok(Myanmar::SignKhamtiToneDash1),
            'ႛ' => Ok(Myanmar::SignKhamtiToneDash3),
            'ႜ' => Ok(Myanmar::VowelSignAitonA),
            'ႝ' => Ok(Myanmar::VowelSignAitonAi),
            '႞' => Ok(Myanmar::SymbolShanOne),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Myanmar {
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

impl std::convert::TryFrom<u32> for Myanmar {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Myanmar {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Myanmar {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Myanmar::LetterKa
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Myanmar::LetterKa => "myanmar letter ka",
            Myanmar::LetterKha => "myanmar letter kha",
            Myanmar::LetterGa => "myanmar letter ga",
            Myanmar::LetterGha => "myanmar letter gha",
            Myanmar::LetterNga => "myanmar letter nga",
            Myanmar::LetterCa => "myanmar letter ca",
            Myanmar::LetterCha => "myanmar letter cha",
            Myanmar::LetterJa => "myanmar letter ja",
            Myanmar::LetterJha => "myanmar letter jha",
            Myanmar::LetterNya => "myanmar letter nya",
            Myanmar::LetterNnya => "myanmar letter nnya",
            Myanmar::LetterTta => "myanmar letter tta",
            Myanmar::LetterTtha => "myanmar letter ttha",
            Myanmar::LetterDda => "myanmar letter dda",
            Myanmar::LetterDdha => "myanmar letter ddha",
            Myanmar::LetterNna => "myanmar letter nna",
            Myanmar::LetterTa => "myanmar letter ta",
            Myanmar::LetterTha => "myanmar letter tha",
            Myanmar::LetterDa => "myanmar letter da",
            Myanmar::LetterDha => "myanmar letter dha",
            Myanmar::LetterNa => "myanmar letter na",
            Myanmar::LetterPa => "myanmar letter pa",
            Myanmar::LetterPha => "myanmar letter pha",
            Myanmar::LetterBa => "myanmar letter ba",
            Myanmar::LetterBha => "myanmar letter bha",
            Myanmar::LetterMa => "myanmar letter ma",
            Myanmar::LetterYa => "myanmar letter ya",
            Myanmar::LetterRa => "myanmar letter ra",
            Myanmar::LetterLa => "myanmar letter la",
            Myanmar::LetterWa => "myanmar letter wa",
            Myanmar::LetterSa => "myanmar letter sa",
            Myanmar::LetterHa => "myanmar letter ha",
            Myanmar::LetterLla => "myanmar letter lla",
            Myanmar::LetterA => "myanmar letter a",
            Myanmar::LetterShanA => "myanmar letter shan a",
            Myanmar::LetterI => "myanmar letter i",
            Myanmar::LetterIi => "myanmar letter ii",
            Myanmar::LetterU => "myanmar letter u",
            Myanmar::LetterUu => "myanmar letter uu",
            Myanmar::LetterE => "myanmar letter e",
            Myanmar::LetterMonE => "myanmar letter mon e",
            Myanmar::LetterO => "myanmar letter o",
            Myanmar::LetterAu => "myanmar letter au",
            Myanmar::VowelSignTallAa => "myanmar vowel sign tall aa",
            Myanmar::VowelSignAa => "myanmar vowel sign aa",
            Myanmar::VowelSignI => "myanmar vowel sign i",
            Myanmar::VowelSignIi => "myanmar vowel sign ii",
            Myanmar::VowelSignU => "myanmar vowel sign u",
            Myanmar::VowelSignUu => "myanmar vowel sign uu",
            Myanmar::VowelSignE => "myanmar vowel sign e",
            Myanmar::VowelSignAi => "myanmar vowel sign ai",
            Myanmar::VowelSignMonIi => "myanmar vowel sign mon ii",
            Myanmar::VowelSignMonO => "myanmar vowel sign mon o",
            Myanmar::VowelSignEAbove => "myanmar vowel sign e above",
            Myanmar::SignAnusvara => "myanmar sign anusvara",
            Myanmar::SignDotBelow => "myanmar sign dot below",
            Myanmar::SignVisarga => "myanmar sign visarga",
            Myanmar::SignVirama => "myanmar sign virama",
            Myanmar::SignAsat => "myanmar sign asat",
            Myanmar::ConsonantSignMedialYa => "myanmar consonant sign medial ya",
            Myanmar::ConsonantSignMedialRa => "myanmar consonant sign medial ra",
            Myanmar::ConsonantSignMedialWa => "myanmar consonant sign medial wa",
            Myanmar::ConsonantSignMedialHa => "myanmar consonant sign medial ha",
            Myanmar::LetterGreatSa => "myanmar letter great sa",
            Myanmar::DigitZero => "myanmar digit zero",
            Myanmar::DigitOne => "myanmar digit one",
            Myanmar::DigitTwo => "myanmar digit two",
            Myanmar::DigitThree => "myanmar digit three",
            Myanmar::DigitFour => "myanmar digit four",
            Myanmar::DigitFive => "myanmar digit five",
            Myanmar::DigitSix => "myanmar digit six",
            Myanmar::DigitSeven => "myanmar digit seven",
            Myanmar::DigitEight => "myanmar digit eight",
            Myanmar::DigitNine => "myanmar digit nine",
            Myanmar::SignLittleSection => "myanmar sign little section",
            Myanmar::SignSection => "myanmar sign section",
            Myanmar::SymbolLocative => "myanmar symbol locative",
            Myanmar::SymbolCompleted => "myanmar symbol completed",
            Myanmar::SymbolAforementioned => "myanmar symbol aforementioned",
            Myanmar::SymbolGenitive => "myanmar symbol genitive",
            Myanmar::LetterSha => "myanmar letter sha",
            Myanmar::LetterSsa => "myanmar letter ssa",
            Myanmar::LetterVocalicR => "myanmar letter vocalic r",
            Myanmar::LetterVocalicRr => "myanmar letter vocalic rr",
            Myanmar::LetterVocalicL => "myanmar letter vocalic l",
            Myanmar::LetterVocalicLl => "myanmar letter vocalic ll",
            Myanmar::VowelSignVocalicR => "myanmar vowel sign vocalic r",
            Myanmar::VowelSignVocalicRr => "myanmar vowel sign vocalic rr",
            Myanmar::VowelSignVocalicL => "myanmar vowel sign vocalic l",
            Myanmar::VowelSignVocalicLl => "myanmar vowel sign vocalic ll",
            Myanmar::LetterMonNga => "myanmar letter mon nga",
            Myanmar::LetterMonJha => "myanmar letter mon jha",
            Myanmar::LetterMonBba => "myanmar letter mon bba",
            Myanmar::LetterMonBbe => "myanmar letter mon bbe",
            Myanmar::ConsonantSignMonMedialNa => "myanmar consonant sign mon medial na",
            Myanmar::ConsonantSignMonMedialMa => "myanmar consonant sign mon medial ma",
            Myanmar::ConsonantSignMonMedialLa => "myanmar consonant sign mon medial la",
            Myanmar::LetterSgawKarenSha => "myanmar letter sgaw karen sha",
            Myanmar::VowelSignSgawKarenEu => "myanmar vowel sign sgaw karen eu",
            Myanmar::ToneMarkSgawKarenHathi => "myanmar tone mark sgaw karen hathi",
            Myanmar::ToneMarkSgawKarenKePho => "myanmar tone mark sgaw karen ke pho",
            Myanmar::LetterWesternPwoKarenTha => "myanmar letter western pwo karen tha",
            Myanmar::LetterWesternPwoKarenPwa => "myanmar letter western pwo karen pwa",
            Myanmar::VowelSignWesternPwoKarenEu => "myanmar vowel sign western pwo karen eu",
            Myanmar::VowelSignWesternPwoKarenUe => "myanmar vowel sign western pwo karen ue",
            Myanmar::SignWesternPwoKarenToneDash1 => "myanmar sign western pwo karen tone-1",
            Myanmar::SignWesternPwoKarenToneDash2 => "myanmar sign western pwo karen tone-2",
            Myanmar::SignWesternPwoKarenToneDash3 => "myanmar sign western pwo karen tone-3",
            Myanmar::SignWesternPwoKarenToneDash4 => "myanmar sign western pwo karen tone-4",
            Myanmar::SignWesternPwoKarenToneDash5 => "myanmar sign western pwo karen tone-5",
            Myanmar::LetterEasternPwoKarenNna => "myanmar letter eastern pwo karen nna",
            Myanmar::LetterEasternPwoKarenYwa => "myanmar letter eastern pwo karen ywa",
            Myanmar::LetterEasternPwoKarenGhwa => "myanmar letter eastern pwo karen ghwa",
            Myanmar::VowelSignGebaKarenI => "myanmar vowel sign geba karen i",
            Myanmar::VowelSignKayahOe => "myanmar vowel sign kayah oe",
            Myanmar::VowelSignKayahU => "myanmar vowel sign kayah u",
            Myanmar::VowelSignKayahEe => "myanmar vowel sign kayah ee",
            Myanmar::LetterShanKa => "myanmar letter shan ka",
            Myanmar::LetterShanKha => "myanmar letter shan kha",
            Myanmar::LetterShanGa => "myanmar letter shan ga",
            Myanmar::LetterShanCa => "myanmar letter shan ca",
            Myanmar::LetterShanZa => "myanmar letter shan za",
            Myanmar::LetterShanNya => "myanmar letter shan nya",
            Myanmar::LetterShanDa => "myanmar letter shan da",
            Myanmar::LetterShanNa => "myanmar letter shan na",
            Myanmar::LetterShanPha => "myanmar letter shan pha",
            Myanmar::LetterShanFa => "myanmar letter shan fa",
            Myanmar::LetterShanBa => "myanmar letter shan ba",
            Myanmar::LetterShanTha => "myanmar letter shan tha",
            Myanmar::LetterShanHa => "myanmar letter shan ha",
            Myanmar::ConsonantSignShanMedialWa => "myanmar consonant sign shan medial wa",
            Myanmar::VowelSignShanAa => "myanmar vowel sign shan aa",
            Myanmar::VowelSignShanE => "myanmar vowel sign shan e",
            Myanmar::VowelSignShanEAbove => "myanmar vowel sign shan e above",
            Myanmar::VowelSignShanFinalY => "myanmar vowel sign shan final y",
            Myanmar::SignShanToneDash2 => "myanmar sign shan tone-2",
            Myanmar::SignShanToneDash3 => "myanmar sign shan tone-3",
            Myanmar::SignShanToneDash5 => "myanmar sign shan tone-5",
            Myanmar::SignShanToneDash6 => "myanmar sign shan tone-6",
            Myanmar::SignShanCouncilToneDash2 => "myanmar sign shan council tone-2",
            Myanmar::SignShanCouncilToneDash3 => "myanmar sign shan council tone-3",
            Myanmar::SignShanCouncilEmphaticTone => "myanmar sign shan council emphatic tone",
            Myanmar::LetterRumaiPalaungFa => "myanmar letter rumai palaung fa",
            Myanmar::SignRumaiPalaungToneDash5 => "myanmar sign rumai palaung tone-5",
            Myanmar::ShanDigitZero => "myanmar shan digit zero",
            Myanmar::ShanDigitOne => "myanmar shan digit one",
            Myanmar::ShanDigitTwo => "myanmar shan digit two",
            Myanmar::ShanDigitThree => "myanmar shan digit three",
            Myanmar::ShanDigitFour => "myanmar shan digit four",
            Myanmar::ShanDigitFive => "myanmar shan digit five",
            Myanmar::ShanDigitSix => "myanmar shan digit six",
            Myanmar::ShanDigitSeven => "myanmar shan digit seven",
            Myanmar::ShanDigitEight => "myanmar shan digit eight",
            Myanmar::ShanDigitNine => "myanmar shan digit nine",
            Myanmar::SignKhamtiToneDash1 => "myanmar sign khamti tone-1",
            Myanmar::SignKhamtiToneDash3 => "myanmar sign khamti tone-3",
            Myanmar::VowelSignAitonA => "myanmar vowel sign aiton a",
            Myanmar::VowelSignAitonAi => "myanmar vowel sign aiton ai",
            Myanmar::SymbolShanOne => "myanmar symbol shan one",
        }
    }
}
