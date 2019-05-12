
/// An enum to represent all characters in the HalfwidthandFullwidthForms block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum HalfwidthandFullwidthForms {
    /// \u{ff01}: '！'
    FullwidthExclamationMark,
    /// \u{ff02}: '＂'
    FullwidthQuotationMark,
    /// \u{ff03}: '＃'
    FullwidthNumberSign,
    /// \u{ff04}: '＄'
    FullwidthDollarSign,
    /// \u{ff05}: '％'
    FullwidthPercentSign,
    /// \u{ff06}: '＆'
    FullwidthAmpersand,
    /// \u{ff07}: '＇'
    FullwidthApostrophe,
    /// \u{ff08}: '（'
    FullwidthLeftParenthesis,
    /// \u{ff09}: '）'
    FullwidthRightParenthesis,
    /// \u{ff0a}: '＊'
    FullwidthAsterisk,
    /// \u{ff0b}: '＋'
    FullwidthPlusSign,
    /// \u{ff0c}: '，'
    FullwidthComma,
    /// \u{ff0d}: '－'
    FullwidthHyphenDashMinus,
    /// \u{ff0e}: '．'
    FullwidthFullStop,
    /// \u{ff0f}: '／'
    FullwidthSolidus,
    /// \u{ff10}: '０'
    FullwidthDigitZero,
    /// \u{ff11}: '１'
    FullwidthDigitOne,
    /// \u{ff12}: '２'
    FullwidthDigitTwo,
    /// \u{ff13}: '３'
    FullwidthDigitThree,
    /// \u{ff14}: '４'
    FullwidthDigitFour,
    /// \u{ff15}: '５'
    FullwidthDigitFive,
    /// \u{ff16}: '６'
    FullwidthDigitSix,
    /// \u{ff17}: '７'
    FullwidthDigitSeven,
    /// \u{ff18}: '８'
    FullwidthDigitEight,
    /// \u{ff19}: '９'
    FullwidthDigitNine,
    /// \u{ff1a}: '：'
    FullwidthColon,
    /// \u{ff1b}: '；'
    FullwidthSemicolon,
    /// \u{ff1c}: '＜'
    FullwidthLessDashThanSign,
    /// \u{ff1d}: '＝'
    FullwidthEqualsSign,
    /// \u{ff1e}: '＞'
    FullwidthGreaterDashThanSign,
    /// \u{ff1f}: '？'
    FullwidthQuestionMark,
    /// \u{ff20}: '＠'
    FullwidthCommercialAt,
    /// \u{ff21}: 'Ａ'
    FullwidthLatinCapitalLetterA,
    /// \u{ff22}: 'Ｂ'
    FullwidthLatinCapitalLetterB,
    /// \u{ff23}: 'Ｃ'
    FullwidthLatinCapitalLetterC,
    /// \u{ff24}: 'Ｄ'
    FullwidthLatinCapitalLetterD,
    /// \u{ff25}: 'Ｅ'
    FullwidthLatinCapitalLetterE,
    /// \u{ff26}: 'Ｆ'
    FullwidthLatinCapitalLetterF,
    /// \u{ff27}: 'Ｇ'
    FullwidthLatinCapitalLetterG,
    /// \u{ff28}: 'Ｈ'
    FullwidthLatinCapitalLetterH,
    /// \u{ff29}: 'Ｉ'
    FullwidthLatinCapitalLetterI,
    /// \u{ff2a}: 'Ｊ'
    FullwidthLatinCapitalLetterJ,
    /// \u{ff2b}: 'Ｋ'
    FullwidthLatinCapitalLetterK,
    /// \u{ff2c}: 'Ｌ'
    FullwidthLatinCapitalLetterL,
    /// \u{ff2d}: 'Ｍ'
    FullwidthLatinCapitalLetterM,
    /// \u{ff2e}: 'Ｎ'
    FullwidthLatinCapitalLetterN,
    /// \u{ff2f}: 'Ｏ'
    FullwidthLatinCapitalLetterO,
    /// \u{ff30}: 'Ｐ'
    FullwidthLatinCapitalLetterP,
    /// \u{ff31}: 'Ｑ'
    FullwidthLatinCapitalLetterQ,
    /// \u{ff32}: 'Ｒ'
    FullwidthLatinCapitalLetterR,
    /// \u{ff33}: 'Ｓ'
    FullwidthLatinCapitalLetterS,
    /// \u{ff34}: 'Ｔ'
    FullwidthLatinCapitalLetterT,
    /// \u{ff35}: 'Ｕ'
    FullwidthLatinCapitalLetterU,
    /// \u{ff36}: 'Ｖ'
    FullwidthLatinCapitalLetterV,
    /// \u{ff37}: 'Ｗ'
    FullwidthLatinCapitalLetterW,
    /// \u{ff38}: 'Ｘ'
    FullwidthLatinCapitalLetterX,
    /// \u{ff39}: 'Ｙ'
    FullwidthLatinCapitalLetterY,
    /// \u{ff3a}: 'Ｚ'
    FullwidthLatinCapitalLetterZ,
    /// \u{ff3b}: '［'
    FullwidthLeftSquareBracket,
    /// \u{ff3c}: '＼'
    FullwidthReverseSolidus,
    /// \u{ff3d}: '］'
    FullwidthRightSquareBracket,
    /// \u{ff3e}: '＾'
    FullwidthCircumflexAccent,
    /// \u{ff3f}: '＿'
    FullwidthLowLine,
    /// \u{ff40}: '｀'
    FullwidthGraveAccent,
    /// \u{ff41}: 'ａ'
    FullwidthLatinSmallLetterA,
    /// \u{ff42}: 'ｂ'
    FullwidthLatinSmallLetterB,
    /// \u{ff43}: 'ｃ'
    FullwidthLatinSmallLetterC,
    /// \u{ff44}: 'ｄ'
    FullwidthLatinSmallLetterD,
    /// \u{ff45}: 'ｅ'
    FullwidthLatinSmallLetterE,
    /// \u{ff46}: 'ｆ'
    FullwidthLatinSmallLetterF,
    /// \u{ff47}: 'ｇ'
    FullwidthLatinSmallLetterG,
    /// \u{ff48}: 'ｈ'
    FullwidthLatinSmallLetterH,
    /// \u{ff49}: 'ｉ'
    FullwidthLatinSmallLetterI,
    /// \u{ff4a}: 'ｊ'
    FullwidthLatinSmallLetterJ,
    /// \u{ff4b}: 'ｋ'
    FullwidthLatinSmallLetterK,
    /// \u{ff4c}: 'ｌ'
    FullwidthLatinSmallLetterL,
    /// \u{ff4d}: 'ｍ'
    FullwidthLatinSmallLetterM,
    /// \u{ff4e}: 'ｎ'
    FullwidthLatinSmallLetterN,
    /// \u{ff4f}: 'ｏ'
    FullwidthLatinSmallLetterO,
    /// \u{ff50}: 'ｐ'
    FullwidthLatinSmallLetterP,
    /// \u{ff51}: 'ｑ'
    FullwidthLatinSmallLetterQ,
    /// \u{ff52}: 'ｒ'
    FullwidthLatinSmallLetterR,
    /// \u{ff53}: 'ｓ'
    FullwidthLatinSmallLetterS,
    /// \u{ff54}: 'ｔ'
    FullwidthLatinSmallLetterT,
    /// \u{ff55}: 'ｕ'
    FullwidthLatinSmallLetterU,
    /// \u{ff56}: 'ｖ'
    FullwidthLatinSmallLetterV,
    /// \u{ff57}: 'ｗ'
    FullwidthLatinSmallLetterW,
    /// \u{ff58}: 'ｘ'
    FullwidthLatinSmallLetterX,
    /// \u{ff59}: 'ｙ'
    FullwidthLatinSmallLetterY,
    /// \u{ff5a}: 'ｚ'
    FullwidthLatinSmallLetterZ,
    /// \u{ff5b}: '｛'
    FullwidthLeftCurlyBracket,
    /// \u{ff5c}: '｜'
    FullwidthVerticalLine,
    /// \u{ff5d}: '｝'
    FullwidthRightCurlyBracket,
    /// \u{ff5e}: '～'
    FullwidthTilde,
    /// \u{ff5f}: '｟'
    FullwidthLeftWhiteParenthesis,
    /// \u{ff60}: '｠'
    FullwidthRightWhiteParenthesis,
    /// \u{ff61}: '｡'
    HalfwidthIdeographicFullStop,
    /// \u{ff62}: '｢'
    HalfwidthLeftCornerBracket,
    /// \u{ff63}: '｣'
    HalfwidthRightCornerBracket,
    /// \u{ff64}: '､'
    HalfwidthIdeographicComma,
    /// \u{ff65}: '･'
    HalfwidthKatakanaMiddleDot,
    /// \u{ff66}: 'ｦ'
    HalfwidthKatakanaLetterWo,
    /// \u{ff67}: 'ｧ'
    HalfwidthKatakanaLetterSmallA,
    /// \u{ff68}: 'ｨ'
    HalfwidthKatakanaLetterSmallI,
    /// \u{ff69}: 'ｩ'
    HalfwidthKatakanaLetterSmallU,
    /// \u{ff6a}: 'ｪ'
    HalfwidthKatakanaLetterSmallE,
    /// \u{ff6b}: 'ｫ'
    HalfwidthKatakanaLetterSmallO,
    /// \u{ff6c}: 'ｬ'
    HalfwidthKatakanaLetterSmallYa,
    /// \u{ff6d}: 'ｭ'
    HalfwidthKatakanaLetterSmallYu,
    /// \u{ff6e}: 'ｮ'
    HalfwidthKatakanaLetterSmallYo,
    /// \u{ff6f}: 'ｯ'
    HalfwidthKatakanaLetterSmallTu,
    /// \u{ff70}: 'ｰ'
    HalfwidthKatakanaDashHiraganaProlongedSoundMark,
    /// \u{ff71}: 'ｱ'
    HalfwidthKatakanaLetterA,
    /// \u{ff72}: 'ｲ'
    HalfwidthKatakanaLetterI,
    /// \u{ff73}: 'ｳ'
    HalfwidthKatakanaLetterU,
    /// \u{ff74}: 'ｴ'
    HalfwidthKatakanaLetterE,
    /// \u{ff75}: 'ｵ'
    HalfwidthKatakanaLetterO,
    /// \u{ff76}: 'ｶ'
    HalfwidthKatakanaLetterKa,
    /// \u{ff77}: 'ｷ'
    HalfwidthKatakanaLetterKi,
    /// \u{ff78}: 'ｸ'
    HalfwidthKatakanaLetterKu,
    /// \u{ff79}: 'ｹ'
    HalfwidthKatakanaLetterKe,
    /// \u{ff7a}: 'ｺ'
    HalfwidthKatakanaLetterKo,
    /// \u{ff7b}: 'ｻ'
    HalfwidthKatakanaLetterSa,
    /// \u{ff7c}: 'ｼ'
    HalfwidthKatakanaLetterSi,
    /// \u{ff7d}: 'ｽ'
    HalfwidthKatakanaLetterSu,
    /// \u{ff7e}: 'ｾ'
    HalfwidthKatakanaLetterSe,
    /// \u{ff7f}: 'ｿ'
    HalfwidthKatakanaLetterSo,
    /// \u{ff80}: 'ﾀ'
    HalfwidthKatakanaLetterTa,
    /// \u{ff81}: 'ﾁ'
    HalfwidthKatakanaLetterTi,
    /// \u{ff82}: 'ﾂ'
    HalfwidthKatakanaLetterTu,
    /// \u{ff83}: 'ﾃ'
    HalfwidthKatakanaLetterTe,
    /// \u{ff84}: 'ﾄ'
    HalfwidthKatakanaLetterTo,
    /// \u{ff85}: 'ﾅ'
    HalfwidthKatakanaLetterNa,
    /// \u{ff86}: 'ﾆ'
    HalfwidthKatakanaLetterNi,
    /// \u{ff87}: 'ﾇ'
    HalfwidthKatakanaLetterNu,
    /// \u{ff88}: 'ﾈ'
    HalfwidthKatakanaLetterNe,
    /// \u{ff89}: 'ﾉ'
    HalfwidthKatakanaLetterNo,
    /// \u{ff8a}: 'ﾊ'
    HalfwidthKatakanaLetterHa,
    /// \u{ff8b}: 'ﾋ'
    HalfwidthKatakanaLetterHi,
    /// \u{ff8c}: 'ﾌ'
    HalfwidthKatakanaLetterHu,
    /// \u{ff8d}: 'ﾍ'
    HalfwidthKatakanaLetterHe,
    /// \u{ff8e}: 'ﾎ'
    HalfwidthKatakanaLetterHo,
    /// \u{ff8f}: 'ﾏ'
    HalfwidthKatakanaLetterMa,
    /// \u{ff90}: 'ﾐ'
    HalfwidthKatakanaLetterMi,
    /// \u{ff91}: 'ﾑ'
    HalfwidthKatakanaLetterMu,
    /// \u{ff92}: 'ﾒ'
    HalfwidthKatakanaLetterMe,
    /// \u{ff93}: 'ﾓ'
    HalfwidthKatakanaLetterMo,
    /// \u{ff94}: 'ﾔ'
    HalfwidthKatakanaLetterYa,
    /// \u{ff95}: 'ﾕ'
    HalfwidthKatakanaLetterYu,
    /// \u{ff96}: 'ﾖ'
    HalfwidthKatakanaLetterYo,
    /// \u{ff97}: 'ﾗ'
    HalfwidthKatakanaLetterRa,
    /// \u{ff98}: 'ﾘ'
    HalfwidthKatakanaLetterRi,
    /// \u{ff99}: 'ﾙ'
    HalfwidthKatakanaLetterRu,
    /// \u{ff9a}: 'ﾚ'
    HalfwidthKatakanaLetterRe,
    /// \u{ff9b}: 'ﾛ'
    HalfwidthKatakanaLetterRo,
    /// \u{ff9c}: 'ﾜ'
    HalfwidthKatakanaLetterWa,
    /// \u{ff9d}: 'ﾝ'
    HalfwidthKatakanaLetterN,
    /// \u{ff9e}: 'ﾞ'
    HalfwidthKatakanaVoicedSoundMark,
    /// \u{ff9f}: 'ﾟ'
    HalfwidthKatakanaSemiDashVoicedSoundMark,
    /// \u{ffa0}: 'ﾠ'
    HalfwidthHangulFiller,
    /// \u{ffa1}: 'ﾡ'
    HalfwidthHangulLetterKiyeok,
    /// \u{ffa2}: 'ﾢ'
    HalfwidthHangulLetterSsangkiyeok,
    /// \u{ffa3}: 'ﾣ'
    HalfwidthHangulLetterKiyeokDashSios,
    /// \u{ffa4}: 'ﾤ'
    HalfwidthHangulLetterNieun,
    /// \u{ffa5}: 'ﾥ'
    HalfwidthHangulLetterNieunDashCieuc,
    /// \u{ffa6}: 'ﾦ'
    HalfwidthHangulLetterNieunDashHieuh,
    /// \u{ffa7}: 'ﾧ'
    HalfwidthHangulLetterTikeut,
    /// \u{ffa8}: 'ﾨ'
    HalfwidthHangulLetterSsangtikeut,
    /// \u{ffa9}: 'ﾩ'
    HalfwidthHangulLetterRieul,
    /// \u{ffaa}: 'ﾪ'
    HalfwidthHangulLetterRieulDashKiyeok,
    /// \u{ffab}: 'ﾫ'
    HalfwidthHangulLetterRieulDashMieum,
    /// \u{ffac}: 'ﾬ'
    HalfwidthHangulLetterRieulDashPieup,
    /// \u{ffad}: 'ﾭ'
    HalfwidthHangulLetterRieulDashSios,
    /// \u{ffae}: 'ﾮ'
    HalfwidthHangulLetterRieulDashThieuth,
    /// \u{ffaf}: 'ﾯ'
    HalfwidthHangulLetterRieulDashPhieuph,
    /// \u{ffb0}: 'ﾰ'
    HalfwidthHangulLetterRieulDashHieuh,
    /// \u{ffb1}: 'ﾱ'
    HalfwidthHangulLetterMieum,
    /// \u{ffb2}: 'ﾲ'
    HalfwidthHangulLetterPieup,
    /// \u{ffb3}: 'ﾳ'
    HalfwidthHangulLetterSsangpieup,
    /// \u{ffb4}: 'ﾴ'
    HalfwidthHangulLetterPieupDashSios,
    /// \u{ffb5}: 'ﾵ'
    HalfwidthHangulLetterSios,
    /// \u{ffb6}: 'ﾶ'
    HalfwidthHangulLetterSsangsios,
    /// \u{ffb7}: 'ﾷ'
    HalfwidthHangulLetterIeung,
    /// \u{ffb8}: 'ﾸ'
    HalfwidthHangulLetterCieuc,
    /// \u{ffb9}: 'ﾹ'
    HalfwidthHangulLetterSsangcieuc,
    /// \u{ffba}: 'ﾺ'
    HalfwidthHangulLetterChieuch,
    /// \u{ffbb}: 'ﾻ'
    HalfwidthHangulLetterKhieukh,
    /// \u{ffbc}: 'ﾼ'
    HalfwidthHangulLetterThieuth,
    /// \u{ffbd}: 'ﾽ'
    HalfwidthHangulLetterPhieuph,
    /// \u{ffbe}: 'ﾾ'
    HalfwidthHangulLetterHieuh,
    /// \u{ffc2}: 'ￂ'
    HalfwidthHangulLetterA,
    /// \u{ffc3}: 'ￃ'
    HalfwidthHangulLetterAe,
    /// \u{ffc4}: 'ￄ'
    HalfwidthHangulLetterYa,
    /// \u{ffc5}: 'ￅ'
    HalfwidthHangulLetterYae,
    /// \u{ffc6}: 'ￆ'
    HalfwidthHangulLetterEo,
    /// \u{ffc7}: 'ￇ'
    HalfwidthHangulLetterE,
    /// \u{ffca}: 'ￊ'
    HalfwidthHangulLetterYeo,
    /// \u{ffcb}: 'ￋ'
    HalfwidthHangulLetterYe,
    /// \u{ffcc}: 'ￌ'
    HalfwidthHangulLetterO,
    /// \u{ffcd}: 'ￍ'
    HalfwidthHangulLetterWa,
    /// \u{ffce}: 'ￎ'
    HalfwidthHangulLetterWae,
    /// \u{ffcf}: 'ￏ'
    HalfwidthHangulLetterOe,
    /// \u{ffd2}: 'ￒ'
    HalfwidthHangulLetterYo,
    /// \u{ffd3}: 'ￓ'
    HalfwidthHangulLetterU,
    /// \u{ffd4}: 'ￔ'
    HalfwidthHangulLetterWeo,
    /// \u{ffd5}: 'ￕ'
    HalfwidthHangulLetterWe,
    /// \u{ffd6}: 'ￖ'
    HalfwidthHangulLetterWi,
    /// \u{ffd7}: 'ￗ'
    HalfwidthHangulLetterYu,
    /// \u{ffda}: 'ￚ'
    HalfwidthHangulLetterEu,
    /// \u{ffdb}: 'ￛ'
    HalfwidthHangulLetterYi,
    /// \u{ffdc}: 'ￜ'
    HalfwidthHangulLetterI,
    /// \u{ffe0}: '￠'
    FullwidthCentSign,
    /// \u{ffe1}: '￡'
    FullwidthPoundSign,
    /// \u{ffe2}: '￢'
    FullwidthNotSign,
    /// \u{ffe3}: '￣'
    FullwidthMacron,
    /// \u{ffe4}: '￤'
    FullwidthBrokenBar,
    /// \u{ffe5}: '￥'
    FullwidthYenSign,
    /// \u{ffe6}: '￦'
    FullwidthWonSign,
    /// \u{ffe8}: '￨'
    HalfwidthFormsLightVertical,
    /// \u{ffe9}: '￩'
    HalfwidthLeftwardsArrow,
    /// \u{ffea}: '￪'
    HalfwidthUpwardsArrow,
    /// \u{ffeb}: '￫'
    HalfwidthRightwardsArrow,
    /// \u{ffec}: '￬'
    HalfwidthDownwardsArrow,
    /// \u{ffed}: '￭'
    HalfwidthBlackSquare,
    /// \u{ffee}: '￮'
    HalfwidthWhiteCircle,
}

impl Into<char> for HalfwidthandFullwidthForms {
    fn into(self) -> char {
        match self {
            HalfwidthandFullwidthForms::FullwidthExclamationMark => '！',
            HalfwidthandFullwidthForms::FullwidthQuotationMark => '＂',
            HalfwidthandFullwidthForms::FullwidthNumberSign => '＃',
            HalfwidthandFullwidthForms::FullwidthDollarSign => '＄',
            HalfwidthandFullwidthForms::FullwidthPercentSign => '％',
            HalfwidthandFullwidthForms::FullwidthAmpersand => '＆',
            HalfwidthandFullwidthForms::FullwidthApostrophe => '＇',
            HalfwidthandFullwidthForms::FullwidthLeftParenthesis => '（',
            HalfwidthandFullwidthForms::FullwidthRightParenthesis => '）',
            HalfwidthandFullwidthForms::FullwidthAsterisk => '＊',
            HalfwidthandFullwidthForms::FullwidthPlusSign => '＋',
            HalfwidthandFullwidthForms::FullwidthComma => '，',
            HalfwidthandFullwidthForms::FullwidthHyphenDashMinus => '－',
            HalfwidthandFullwidthForms::FullwidthFullStop => '．',
            HalfwidthandFullwidthForms::FullwidthSolidus => '／',
            HalfwidthandFullwidthForms::FullwidthDigitZero => '０',
            HalfwidthandFullwidthForms::FullwidthDigitOne => '１',
            HalfwidthandFullwidthForms::FullwidthDigitTwo => '２',
            HalfwidthandFullwidthForms::FullwidthDigitThree => '３',
            HalfwidthandFullwidthForms::FullwidthDigitFour => '４',
            HalfwidthandFullwidthForms::FullwidthDigitFive => '５',
            HalfwidthandFullwidthForms::FullwidthDigitSix => '６',
            HalfwidthandFullwidthForms::FullwidthDigitSeven => '７',
            HalfwidthandFullwidthForms::FullwidthDigitEight => '８',
            HalfwidthandFullwidthForms::FullwidthDigitNine => '９',
            HalfwidthandFullwidthForms::FullwidthColon => '：',
            HalfwidthandFullwidthForms::FullwidthSemicolon => '；',
            HalfwidthandFullwidthForms::FullwidthLessDashThanSign => '＜',
            HalfwidthandFullwidthForms::FullwidthEqualsSign => '＝',
            HalfwidthandFullwidthForms::FullwidthGreaterDashThanSign => '＞',
            HalfwidthandFullwidthForms::FullwidthQuestionMark => '？',
            HalfwidthandFullwidthForms::FullwidthCommercialAt => '＠',
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterA => 'Ａ',
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterB => 'Ｂ',
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterC => 'Ｃ',
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterD => 'Ｄ',
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterE => 'Ｅ',
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterF => 'Ｆ',
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterG => 'Ｇ',
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterH => 'Ｈ',
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterI => 'Ｉ',
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterJ => 'Ｊ',
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterK => 'Ｋ',
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterL => 'Ｌ',
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterM => 'Ｍ',
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterN => 'Ｎ',
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterO => 'Ｏ',
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterP => 'Ｐ',
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterQ => 'Ｑ',
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterR => 'Ｒ',
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterS => 'Ｓ',
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterT => 'Ｔ',
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterU => 'Ｕ',
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterV => 'Ｖ',
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterW => 'Ｗ',
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterX => 'Ｘ',
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterY => 'Ｙ',
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterZ => 'Ｚ',
            HalfwidthandFullwidthForms::FullwidthLeftSquareBracket => '［',
            HalfwidthandFullwidthForms::FullwidthReverseSolidus => '＼',
            HalfwidthandFullwidthForms::FullwidthRightSquareBracket => '］',
            HalfwidthandFullwidthForms::FullwidthCircumflexAccent => '＾',
            HalfwidthandFullwidthForms::FullwidthLowLine => '＿',
            HalfwidthandFullwidthForms::FullwidthGraveAccent => '｀',
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterA => 'ａ',
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterB => 'ｂ',
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterC => 'ｃ',
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterD => 'ｄ',
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterE => 'ｅ',
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterF => 'ｆ',
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterG => 'ｇ',
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterH => 'ｈ',
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterI => 'ｉ',
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterJ => 'ｊ',
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterK => 'ｋ',
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterL => 'ｌ',
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterM => 'ｍ',
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterN => 'ｎ',
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterO => 'ｏ',
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterP => 'ｐ',
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterQ => 'ｑ',
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterR => 'ｒ',
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterS => 'ｓ',
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterT => 'ｔ',
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterU => 'ｕ',
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterV => 'ｖ',
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterW => 'ｗ',
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterX => 'ｘ',
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterY => 'ｙ',
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterZ => 'ｚ',
            HalfwidthandFullwidthForms::FullwidthLeftCurlyBracket => '｛',
            HalfwidthandFullwidthForms::FullwidthVerticalLine => '｜',
            HalfwidthandFullwidthForms::FullwidthRightCurlyBracket => '｝',
            HalfwidthandFullwidthForms::FullwidthTilde => '～',
            HalfwidthandFullwidthForms::FullwidthLeftWhiteParenthesis => '｟',
            HalfwidthandFullwidthForms::FullwidthRightWhiteParenthesis => '｠',
            HalfwidthandFullwidthForms::HalfwidthIdeographicFullStop => '｡',
            HalfwidthandFullwidthForms::HalfwidthLeftCornerBracket => '｢',
            HalfwidthandFullwidthForms::HalfwidthRightCornerBracket => '｣',
            HalfwidthandFullwidthForms::HalfwidthIdeographicComma => '､',
            HalfwidthandFullwidthForms::HalfwidthKatakanaMiddleDot => '･',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterWo => 'ｦ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallA => 'ｧ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallI => 'ｨ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallU => 'ｩ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallE => 'ｪ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallO => 'ｫ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallYa => 'ｬ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallYu => 'ｭ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallYo => 'ｮ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallTu => 'ｯ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaDashHiraganaProlongedSoundMark => 'ｰ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterA => 'ｱ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterI => 'ｲ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterU => 'ｳ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterE => 'ｴ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterO => 'ｵ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterKa => 'ｶ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterKi => 'ｷ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterKu => 'ｸ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterKe => 'ｹ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterKo => 'ｺ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSa => 'ｻ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSi => 'ｼ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSu => 'ｽ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSe => 'ｾ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSo => 'ｿ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterTa => 'ﾀ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterTi => 'ﾁ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterTu => 'ﾂ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterTe => 'ﾃ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterTo => 'ﾄ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterNa => 'ﾅ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterNi => 'ﾆ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterNu => 'ﾇ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterNe => 'ﾈ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterNo => 'ﾉ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterHa => 'ﾊ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterHi => 'ﾋ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterHu => 'ﾌ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterHe => 'ﾍ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterHo => 'ﾎ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterMa => 'ﾏ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterMi => 'ﾐ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterMu => 'ﾑ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterMe => 'ﾒ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterMo => 'ﾓ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterYa => 'ﾔ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterYu => 'ﾕ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterYo => 'ﾖ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterRa => 'ﾗ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterRi => 'ﾘ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterRu => 'ﾙ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterRe => 'ﾚ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterRo => 'ﾛ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterWa => 'ﾜ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterN => 'ﾝ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaVoicedSoundMark => 'ﾞ',
            HalfwidthandFullwidthForms::HalfwidthKatakanaSemiDashVoicedSoundMark => 'ﾟ',
            HalfwidthandFullwidthForms::HalfwidthHangulFiller => 'ﾠ',
            HalfwidthandFullwidthForms::HalfwidthHangulLetterKiyeok => 'ﾡ',
            HalfwidthandFullwidthForms::HalfwidthHangulLetterSsangkiyeok => 'ﾢ',
            HalfwidthandFullwidthForms::HalfwidthHangulLetterKiyeokDashSios => 'ﾣ',
            HalfwidthandFullwidthForms::HalfwidthHangulLetterNieun => 'ﾤ',
            HalfwidthandFullwidthForms::HalfwidthHangulLetterNieunDashCieuc => 'ﾥ',
            HalfwidthandFullwidthForms::HalfwidthHangulLetterNieunDashHieuh => 'ﾦ',
            HalfwidthandFullwidthForms::HalfwidthHangulLetterTikeut => 'ﾧ',
            HalfwidthandFullwidthForms::HalfwidthHangulLetterSsangtikeut => 'ﾨ',
            HalfwidthandFullwidthForms::HalfwidthHangulLetterRieul => 'ﾩ',
            HalfwidthandFullwidthForms::HalfwidthHangulLetterRieulDashKiyeok => 'ﾪ',
            HalfwidthandFullwidthForms::HalfwidthHangulLetterRieulDashMieum => 'ﾫ',
            HalfwidthandFullwidthForms::HalfwidthHangulLetterRieulDashPieup => 'ﾬ',
            HalfwidthandFullwidthForms::HalfwidthHangulLetterRieulDashSios => 'ﾭ',
            HalfwidthandFullwidthForms::HalfwidthHangulLetterRieulDashThieuth => 'ﾮ',
            HalfwidthandFullwidthForms::HalfwidthHangulLetterRieulDashPhieuph => 'ﾯ',
            HalfwidthandFullwidthForms::HalfwidthHangulLetterRieulDashHieuh => 'ﾰ',
            HalfwidthandFullwidthForms::HalfwidthHangulLetterMieum => 'ﾱ',
            HalfwidthandFullwidthForms::HalfwidthHangulLetterPieup => 'ﾲ',
            HalfwidthandFullwidthForms::HalfwidthHangulLetterSsangpieup => 'ﾳ',
            HalfwidthandFullwidthForms::HalfwidthHangulLetterPieupDashSios => 'ﾴ',
            HalfwidthandFullwidthForms::HalfwidthHangulLetterSios => 'ﾵ',
            HalfwidthandFullwidthForms::HalfwidthHangulLetterSsangsios => 'ﾶ',
            HalfwidthandFullwidthForms::HalfwidthHangulLetterIeung => 'ﾷ',
            HalfwidthandFullwidthForms::HalfwidthHangulLetterCieuc => 'ﾸ',
            HalfwidthandFullwidthForms::HalfwidthHangulLetterSsangcieuc => 'ﾹ',
            HalfwidthandFullwidthForms::HalfwidthHangulLetterChieuch => 'ﾺ',
            HalfwidthandFullwidthForms::HalfwidthHangulLetterKhieukh => 'ﾻ',
            HalfwidthandFullwidthForms::HalfwidthHangulLetterThieuth => 'ﾼ',
            HalfwidthandFullwidthForms::HalfwidthHangulLetterPhieuph => 'ﾽ',
            HalfwidthandFullwidthForms::HalfwidthHangulLetterHieuh => 'ﾾ',
            HalfwidthandFullwidthForms::HalfwidthHangulLetterA => 'ￂ',
            HalfwidthandFullwidthForms::HalfwidthHangulLetterAe => 'ￃ',
            HalfwidthandFullwidthForms::HalfwidthHangulLetterYa => 'ￄ',
            HalfwidthandFullwidthForms::HalfwidthHangulLetterYae => 'ￅ',
            HalfwidthandFullwidthForms::HalfwidthHangulLetterEo => 'ￆ',
            HalfwidthandFullwidthForms::HalfwidthHangulLetterE => 'ￇ',
            HalfwidthandFullwidthForms::HalfwidthHangulLetterYeo => 'ￊ',
            HalfwidthandFullwidthForms::HalfwidthHangulLetterYe => 'ￋ',
            HalfwidthandFullwidthForms::HalfwidthHangulLetterO => 'ￌ',
            HalfwidthandFullwidthForms::HalfwidthHangulLetterWa => 'ￍ',
            HalfwidthandFullwidthForms::HalfwidthHangulLetterWae => 'ￎ',
            HalfwidthandFullwidthForms::HalfwidthHangulLetterOe => 'ￏ',
            HalfwidthandFullwidthForms::HalfwidthHangulLetterYo => 'ￒ',
            HalfwidthandFullwidthForms::HalfwidthHangulLetterU => 'ￓ',
            HalfwidthandFullwidthForms::HalfwidthHangulLetterWeo => 'ￔ',
            HalfwidthandFullwidthForms::HalfwidthHangulLetterWe => 'ￕ',
            HalfwidthandFullwidthForms::HalfwidthHangulLetterWi => 'ￖ',
            HalfwidthandFullwidthForms::HalfwidthHangulLetterYu => 'ￗ',
            HalfwidthandFullwidthForms::HalfwidthHangulLetterEu => 'ￚ',
            HalfwidthandFullwidthForms::HalfwidthHangulLetterYi => 'ￛ',
            HalfwidthandFullwidthForms::HalfwidthHangulLetterI => 'ￜ',
            HalfwidthandFullwidthForms::FullwidthCentSign => '￠',
            HalfwidthandFullwidthForms::FullwidthPoundSign => '￡',
            HalfwidthandFullwidthForms::FullwidthNotSign => '￢',
            HalfwidthandFullwidthForms::FullwidthMacron => '￣',
            HalfwidthandFullwidthForms::FullwidthBrokenBar => '￤',
            HalfwidthandFullwidthForms::FullwidthYenSign => '￥',
            HalfwidthandFullwidthForms::FullwidthWonSign => '￦',
            HalfwidthandFullwidthForms::HalfwidthFormsLightVertical => '￨',
            HalfwidthandFullwidthForms::HalfwidthLeftwardsArrow => '￩',
            HalfwidthandFullwidthForms::HalfwidthUpwardsArrow => '￪',
            HalfwidthandFullwidthForms::HalfwidthRightwardsArrow => '￫',
            HalfwidthandFullwidthForms::HalfwidthDownwardsArrow => '￬',
            HalfwidthandFullwidthForms::HalfwidthBlackSquare => '￭',
            HalfwidthandFullwidthForms::HalfwidthWhiteCircle => '￮',
        }
    }
}

impl std::convert::TryFrom<char> for HalfwidthandFullwidthForms {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '！' => Ok(HalfwidthandFullwidthForms::FullwidthExclamationMark),
            '＂' => Ok(HalfwidthandFullwidthForms::FullwidthQuotationMark),
            '＃' => Ok(HalfwidthandFullwidthForms::FullwidthNumberSign),
            '＄' => Ok(HalfwidthandFullwidthForms::FullwidthDollarSign),
            '％' => Ok(HalfwidthandFullwidthForms::FullwidthPercentSign),
            '＆' => Ok(HalfwidthandFullwidthForms::FullwidthAmpersand),
            '＇' => Ok(HalfwidthandFullwidthForms::FullwidthApostrophe),
            '（' => Ok(HalfwidthandFullwidthForms::FullwidthLeftParenthesis),
            '）' => Ok(HalfwidthandFullwidthForms::FullwidthRightParenthesis),
            '＊' => Ok(HalfwidthandFullwidthForms::FullwidthAsterisk),
            '＋' => Ok(HalfwidthandFullwidthForms::FullwidthPlusSign),
            '，' => Ok(HalfwidthandFullwidthForms::FullwidthComma),
            '－' => Ok(HalfwidthandFullwidthForms::FullwidthHyphenDashMinus),
            '．' => Ok(HalfwidthandFullwidthForms::FullwidthFullStop),
            '／' => Ok(HalfwidthandFullwidthForms::FullwidthSolidus),
            '０' => Ok(HalfwidthandFullwidthForms::FullwidthDigitZero),
            '１' => Ok(HalfwidthandFullwidthForms::FullwidthDigitOne),
            '２' => Ok(HalfwidthandFullwidthForms::FullwidthDigitTwo),
            '３' => Ok(HalfwidthandFullwidthForms::FullwidthDigitThree),
            '４' => Ok(HalfwidthandFullwidthForms::FullwidthDigitFour),
            '５' => Ok(HalfwidthandFullwidthForms::FullwidthDigitFive),
            '６' => Ok(HalfwidthandFullwidthForms::FullwidthDigitSix),
            '７' => Ok(HalfwidthandFullwidthForms::FullwidthDigitSeven),
            '８' => Ok(HalfwidthandFullwidthForms::FullwidthDigitEight),
            '９' => Ok(HalfwidthandFullwidthForms::FullwidthDigitNine),
            '：' => Ok(HalfwidthandFullwidthForms::FullwidthColon),
            '；' => Ok(HalfwidthandFullwidthForms::FullwidthSemicolon),
            '＜' => Ok(HalfwidthandFullwidthForms::FullwidthLessDashThanSign),
            '＝' => Ok(HalfwidthandFullwidthForms::FullwidthEqualsSign),
            '＞' => Ok(HalfwidthandFullwidthForms::FullwidthGreaterDashThanSign),
            '？' => Ok(HalfwidthandFullwidthForms::FullwidthQuestionMark),
            '＠' => Ok(HalfwidthandFullwidthForms::FullwidthCommercialAt),
            'Ａ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterA),
            'Ｂ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterB),
            'Ｃ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterC),
            'Ｄ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterD),
            'Ｅ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterE),
            'Ｆ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterF),
            'Ｇ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterG),
            'Ｈ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterH),
            'Ｉ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterI),
            'Ｊ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterJ),
            'Ｋ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterK),
            'Ｌ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterL),
            'Ｍ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterM),
            'Ｎ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterN),
            'Ｏ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterO),
            'Ｐ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterP),
            'Ｑ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterQ),
            'Ｒ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterR),
            'Ｓ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterS),
            'Ｔ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterT),
            'Ｕ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterU),
            'Ｖ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterV),
            'Ｗ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterW),
            'Ｘ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterX),
            'Ｙ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterY),
            'Ｚ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterZ),
            '［' => Ok(HalfwidthandFullwidthForms::FullwidthLeftSquareBracket),
            '＼' => Ok(HalfwidthandFullwidthForms::FullwidthReverseSolidus),
            '］' => Ok(HalfwidthandFullwidthForms::FullwidthRightSquareBracket),
            '＾' => Ok(HalfwidthandFullwidthForms::FullwidthCircumflexAccent),
            '＿' => Ok(HalfwidthandFullwidthForms::FullwidthLowLine),
            '｀' => Ok(HalfwidthandFullwidthForms::FullwidthGraveAccent),
            'ａ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterA),
            'ｂ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterB),
            'ｃ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterC),
            'ｄ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterD),
            'ｅ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterE),
            'ｆ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterF),
            'ｇ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterG),
            'ｈ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterH),
            'ｉ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterI),
            'ｊ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterJ),
            'ｋ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterK),
            'ｌ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterL),
            'ｍ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterM),
            'ｎ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterN),
            'ｏ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterO),
            'ｐ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterP),
            'ｑ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterQ),
            'ｒ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterR),
            'ｓ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterS),
            'ｔ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterT),
            'ｕ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterU),
            'ｖ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterV),
            'ｗ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterW),
            'ｘ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterX),
            'ｙ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterY),
            'ｚ' => Ok(HalfwidthandFullwidthForms::FullwidthLatinSmallLetterZ),
            '｛' => Ok(HalfwidthandFullwidthForms::FullwidthLeftCurlyBracket),
            '｜' => Ok(HalfwidthandFullwidthForms::FullwidthVerticalLine),
            '｝' => Ok(HalfwidthandFullwidthForms::FullwidthRightCurlyBracket),
            '～' => Ok(HalfwidthandFullwidthForms::FullwidthTilde),
            '｟' => Ok(HalfwidthandFullwidthForms::FullwidthLeftWhiteParenthesis),
            '｠' => Ok(HalfwidthandFullwidthForms::FullwidthRightWhiteParenthesis),
            '｡' => Ok(HalfwidthandFullwidthForms::HalfwidthIdeographicFullStop),
            '｢' => Ok(HalfwidthandFullwidthForms::HalfwidthLeftCornerBracket),
            '｣' => Ok(HalfwidthandFullwidthForms::HalfwidthRightCornerBracket),
            '､' => Ok(HalfwidthandFullwidthForms::HalfwidthIdeographicComma),
            '･' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaMiddleDot),
            'ｦ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterWo),
            'ｧ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallA),
            'ｨ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallI),
            'ｩ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallU),
            'ｪ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallE),
            'ｫ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallO),
            'ｬ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallYa),
            'ｭ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallYu),
            'ｮ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallYo),
            'ｯ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallTu),
            'ｰ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaDashHiraganaProlongedSoundMark),
            'ｱ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterA),
            'ｲ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterI),
            'ｳ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterU),
            'ｴ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterE),
            'ｵ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterO),
            'ｶ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterKa),
            'ｷ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterKi),
            'ｸ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterKu),
            'ｹ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterKe),
            'ｺ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterKo),
            'ｻ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSa),
            'ｼ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSi),
            'ｽ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSu),
            'ｾ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSe),
            'ｿ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSo),
            'ﾀ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterTa),
            'ﾁ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterTi),
            'ﾂ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterTu),
            'ﾃ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterTe),
            'ﾄ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterTo),
            'ﾅ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterNa),
            'ﾆ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterNi),
            'ﾇ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterNu),
            'ﾈ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterNe),
            'ﾉ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterNo),
            'ﾊ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterHa),
            'ﾋ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterHi),
            'ﾌ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterHu),
            'ﾍ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterHe),
            'ﾎ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterHo),
            'ﾏ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterMa),
            'ﾐ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterMi),
            'ﾑ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterMu),
            'ﾒ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterMe),
            'ﾓ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterMo),
            'ﾔ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterYa),
            'ﾕ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterYu),
            'ﾖ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterYo),
            'ﾗ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterRa),
            'ﾘ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterRi),
            'ﾙ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterRu),
            'ﾚ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterRe),
            'ﾛ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterRo),
            'ﾜ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterWa),
            'ﾝ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaLetterN),
            'ﾞ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaVoicedSoundMark),
            'ﾟ' => Ok(HalfwidthandFullwidthForms::HalfwidthKatakanaSemiDashVoicedSoundMark),
            'ﾠ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulFiller),
            'ﾡ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterKiyeok),
            'ﾢ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterSsangkiyeok),
            'ﾣ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterKiyeokDashSios),
            'ﾤ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterNieun),
            'ﾥ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterNieunDashCieuc),
            'ﾦ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterNieunDashHieuh),
            'ﾧ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterTikeut),
            'ﾨ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterSsangtikeut),
            'ﾩ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterRieul),
            'ﾪ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterRieulDashKiyeok),
            'ﾫ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterRieulDashMieum),
            'ﾬ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterRieulDashPieup),
            'ﾭ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterRieulDashSios),
            'ﾮ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterRieulDashThieuth),
            'ﾯ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterRieulDashPhieuph),
            'ﾰ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterRieulDashHieuh),
            'ﾱ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterMieum),
            'ﾲ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterPieup),
            'ﾳ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterSsangpieup),
            'ﾴ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterPieupDashSios),
            'ﾵ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterSios),
            'ﾶ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterSsangsios),
            'ﾷ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterIeung),
            'ﾸ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterCieuc),
            'ﾹ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterSsangcieuc),
            'ﾺ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterChieuch),
            'ﾻ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterKhieukh),
            'ﾼ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterThieuth),
            'ﾽ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterPhieuph),
            'ﾾ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterHieuh),
            'ￂ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterA),
            'ￃ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterAe),
            'ￄ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterYa),
            'ￅ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterYae),
            'ￆ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterEo),
            'ￇ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterE),
            'ￊ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterYeo),
            'ￋ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterYe),
            'ￌ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterO),
            'ￍ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterWa),
            'ￎ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterWae),
            'ￏ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterOe),
            'ￒ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterYo),
            'ￓ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterU),
            'ￔ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterWeo),
            'ￕ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterWe),
            'ￖ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterWi),
            'ￗ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterYu),
            'ￚ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterEu),
            'ￛ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterYi),
            'ￜ' => Ok(HalfwidthandFullwidthForms::HalfwidthHangulLetterI),
            '￠' => Ok(HalfwidthandFullwidthForms::FullwidthCentSign),
            '￡' => Ok(HalfwidthandFullwidthForms::FullwidthPoundSign),
            '￢' => Ok(HalfwidthandFullwidthForms::FullwidthNotSign),
            '￣' => Ok(HalfwidthandFullwidthForms::FullwidthMacron),
            '￤' => Ok(HalfwidthandFullwidthForms::FullwidthBrokenBar),
            '￥' => Ok(HalfwidthandFullwidthForms::FullwidthYenSign),
            '￦' => Ok(HalfwidthandFullwidthForms::FullwidthWonSign),
            '￨' => Ok(HalfwidthandFullwidthForms::HalfwidthFormsLightVertical),
            '￩' => Ok(HalfwidthandFullwidthForms::HalfwidthLeftwardsArrow),
            '￪' => Ok(HalfwidthandFullwidthForms::HalfwidthUpwardsArrow),
            '￫' => Ok(HalfwidthandFullwidthForms::HalfwidthRightwardsArrow),
            '￬' => Ok(HalfwidthandFullwidthForms::HalfwidthDownwardsArrow),
            '￭' => Ok(HalfwidthandFullwidthForms::HalfwidthBlackSquare),
            '￮' => Ok(HalfwidthandFullwidthForms::HalfwidthWhiteCircle),
            _ => Err(()),
        }
    }
}

impl Into<u32> for HalfwidthandFullwidthForms {
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

impl std::convert::TryFrom<u32> for HalfwidthandFullwidthForms {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for HalfwidthandFullwidthForms {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl HalfwidthandFullwidthForms {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        HalfwidthandFullwidthForms::FullwidthExclamationMark
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            HalfwidthandFullwidthForms::FullwidthExclamationMark => "fullwidth exclamation mark",
            HalfwidthandFullwidthForms::FullwidthQuotationMark => "fullwidth quotation mark",
            HalfwidthandFullwidthForms::FullwidthNumberSign => "fullwidth number sign",
            HalfwidthandFullwidthForms::FullwidthDollarSign => "fullwidth dollar sign",
            HalfwidthandFullwidthForms::FullwidthPercentSign => "fullwidth percent sign",
            HalfwidthandFullwidthForms::FullwidthAmpersand => "fullwidth ampersand",
            HalfwidthandFullwidthForms::FullwidthApostrophe => "fullwidth apostrophe",
            HalfwidthandFullwidthForms::FullwidthLeftParenthesis => "fullwidth left parenthesis",
            HalfwidthandFullwidthForms::FullwidthRightParenthesis => "fullwidth right parenthesis",
            HalfwidthandFullwidthForms::FullwidthAsterisk => "fullwidth asterisk",
            HalfwidthandFullwidthForms::FullwidthPlusSign => "fullwidth plus sign",
            HalfwidthandFullwidthForms::FullwidthComma => "fullwidth comma",
            HalfwidthandFullwidthForms::FullwidthHyphenDashMinus => "fullwidth hyphen-minus",
            HalfwidthandFullwidthForms::FullwidthFullStop => "fullwidth full stop",
            HalfwidthandFullwidthForms::FullwidthSolidus => "fullwidth solidus",
            HalfwidthandFullwidthForms::FullwidthDigitZero => "fullwidth digit zero",
            HalfwidthandFullwidthForms::FullwidthDigitOne => "fullwidth digit one",
            HalfwidthandFullwidthForms::FullwidthDigitTwo => "fullwidth digit two",
            HalfwidthandFullwidthForms::FullwidthDigitThree => "fullwidth digit three",
            HalfwidthandFullwidthForms::FullwidthDigitFour => "fullwidth digit four",
            HalfwidthandFullwidthForms::FullwidthDigitFive => "fullwidth digit five",
            HalfwidthandFullwidthForms::FullwidthDigitSix => "fullwidth digit six",
            HalfwidthandFullwidthForms::FullwidthDigitSeven => "fullwidth digit seven",
            HalfwidthandFullwidthForms::FullwidthDigitEight => "fullwidth digit eight",
            HalfwidthandFullwidthForms::FullwidthDigitNine => "fullwidth digit nine",
            HalfwidthandFullwidthForms::FullwidthColon => "fullwidth colon",
            HalfwidthandFullwidthForms::FullwidthSemicolon => "fullwidth semicolon",
            HalfwidthandFullwidthForms::FullwidthLessDashThanSign => "fullwidth less-than sign",
            HalfwidthandFullwidthForms::FullwidthEqualsSign => "fullwidth equals sign",
            HalfwidthandFullwidthForms::FullwidthGreaterDashThanSign => "fullwidth greater-than sign",
            HalfwidthandFullwidthForms::FullwidthQuestionMark => "fullwidth question mark",
            HalfwidthandFullwidthForms::FullwidthCommercialAt => "fullwidth commercial at",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterA => "fullwidth latin capital letter a",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterB => "fullwidth latin capital letter b",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterC => "fullwidth latin capital letter c",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterD => "fullwidth latin capital letter d",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterE => "fullwidth latin capital letter e",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterF => "fullwidth latin capital letter f",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterG => "fullwidth latin capital letter g",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterH => "fullwidth latin capital letter h",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterI => "fullwidth latin capital letter i",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterJ => "fullwidth latin capital letter j",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterK => "fullwidth latin capital letter k",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterL => "fullwidth latin capital letter l",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterM => "fullwidth latin capital letter m",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterN => "fullwidth latin capital letter n",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterO => "fullwidth latin capital letter o",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterP => "fullwidth latin capital letter p",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterQ => "fullwidth latin capital letter q",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterR => "fullwidth latin capital letter r",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterS => "fullwidth latin capital letter s",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterT => "fullwidth latin capital letter t",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterU => "fullwidth latin capital letter u",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterV => "fullwidth latin capital letter v",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterW => "fullwidth latin capital letter w",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterX => "fullwidth latin capital letter x",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterY => "fullwidth latin capital letter y",
            HalfwidthandFullwidthForms::FullwidthLatinCapitalLetterZ => "fullwidth latin capital letter z",
            HalfwidthandFullwidthForms::FullwidthLeftSquareBracket => "fullwidth left square bracket",
            HalfwidthandFullwidthForms::FullwidthReverseSolidus => "fullwidth reverse solidus",
            HalfwidthandFullwidthForms::FullwidthRightSquareBracket => "fullwidth right square bracket",
            HalfwidthandFullwidthForms::FullwidthCircumflexAccent => "fullwidth circumflex accent",
            HalfwidthandFullwidthForms::FullwidthLowLine => "fullwidth low line",
            HalfwidthandFullwidthForms::FullwidthGraveAccent => "fullwidth grave accent",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterA => "fullwidth latin small letter a",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterB => "fullwidth latin small letter b",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterC => "fullwidth latin small letter c",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterD => "fullwidth latin small letter d",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterE => "fullwidth latin small letter e",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterF => "fullwidth latin small letter f",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterG => "fullwidth latin small letter g",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterH => "fullwidth latin small letter h",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterI => "fullwidth latin small letter i",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterJ => "fullwidth latin small letter j",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterK => "fullwidth latin small letter k",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterL => "fullwidth latin small letter l",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterM => "fullwidth latin small letter m",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterN => "fullwidth latin small letter n",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterO => "fullwidth latin small letter o",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterP => "fullwidth latin small letter p",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterQ => "fullwidth latin small letter q",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterR => "fullwidth latin small letter r",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterS => "fullwidth latin small letter s",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterT => "fullwidth latin small letter t",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterU => "fullwidth latin small letter u",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterV => "fullwidth latin small letter v",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterW => "fullwidth latin small letter w",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterX => "fullwidth latin small letter x",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterY => "fullwidth latin small letter y",
            HalfwidthandFullwidthForms::FullwidthLatinSmallLetterZ => "fullwidth latin small letter z",
            HalfwidthandFullwidthForms::FullwidthLeftCurlyBracket => "fullwidth left curly bracket",
            HalfwidthandFullwidthForms::FullwidthVerticalLine => "fullwidth vertical line",
            HalfwidthandFullwidthForms::FullwidthRightCurlyBracket => "fullwidth right curly bracket",
            HalfwidthandFullwidthForms::FullwidthTilde => "fullwidth tilde",
            HalfwidthandFullwidthForms::FullwidthLeftWhiteParenthesis => "fullwidth left white parenthesis",
            HalfwidthandFullwidthForms::FullwidthRightWhiteParenthesis => "fullwidth right white parenthesis",
            HalfwidthandFullwidthForms::HalfwidthIdeographicFullStop => "halfwidth ideographic full stop",
            HalfwidthandFullwidthForms::HalfwidthLeftCornerBracket => "halfwidth left corner bracket",
            HalfwidthandFullwidthForms::HalfwidthRightCornerBracket => "halfwidth right corner bracket",
            HalfwidthandFullwidthForms::HalfwidthIdeographicComma => "halfwidth ideographic comma",
            HalfwidthandFullwidthForms::HalfwidthKatakanaMiddleDot => "halfwidth katakana middle dot",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterWo => "halfwidth katakana letter wo",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallA => "halfwidth katakana letter small a",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallI => "halfwidth katakana letter small i",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallU => "halfwidth katakana letter small u",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallE => "halfwidth katakana letter small e",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallO => "halfwidth katakana letter small o",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallYa => "halfwidth katakana letter small ya",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallYu => "halfwidth katakana letter small yu",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallYo => "halfwidth katakana letter small yo",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSmallTu => "halfwidth katakana letter small tu",
            HalfwidthandFullwidthForms::HalfwidthKatakanaDashHiraganaProlongedSoundMark => "halfwidth katakana-hiragana prolonged sound mark",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterA => "halfwidth katakana letter a",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterI => "halfwidth katakana letter i",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterU => "halfwidth katakana letter u",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterE => "halfwidth katakana letter e",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterO => "halfwidth katakana letter o",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterKa => "halfwidth katakana letter ka",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterKi => "halfwidth katakana letter ki",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterKu => "halfwidth katakana letter ku",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterKe => "halfwidth katakana letter ke",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterKo => "halfwidth katakana letter ko",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSa => "halfwidth katakana letter sa",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSi => "halfwidth katakana letter si",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSu => "halfwidth katakana letter su",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSe => "halfwidth katakana letter se",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterSo => "halfwidth katakana letter so",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterTa => "halfwidth katakana letter ta",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterTi => "halfwidth katakana letter ti",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterTu => "halfwidth katakana letter tu",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterTe => "halfwidth katakana letter te",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterTo => "halfwidth katakana letter to",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterNa => "halfwidth katakana letter na",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterNi => "halfwidth katakana letter ni",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterNu => "halfwidth katakana letter nu",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterNe => "halfwidth katakana letter ne",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterNo => "halfwidth katakana letter no",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterHa => "halfwidth katakana letter ha",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterHi => "halfwidth katakana letter hi",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterHu => "halfwidth katakana letter hu",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterHe => "halfwidth katakana letter he",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterHo => "halfwidth katakana letter ho",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterMa => "halfwidth katakana letter ma",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterMi => "halfwidth katakana letter mi",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterMu => "halfwidth katakana letter mu",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterMe => "halfwidth katakana letter me",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterMo => "halfwidth katakana letter mo",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterYa => "halfwidth katakana letter ya",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterYu => "halfwidth katakana letter yu",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterYo => "halfwidth katakana letter yo",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterRa => "halfwidth katakana letter ra",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterRi => "halfwidth katakana letter ri",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterRu => "halfwidth katakana letter ru",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterRe => "halfwidth katakana letter re",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterRo => "halfwidth katakana letter ro",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterWa => "halfwidth katakana letter wa",
            HalfwidthandFullwidthForms::HalfwidthKatakanaLetterN => "halfwidth katakana letter n",
            HalfwidthandFullwidthForms::HalfwidthKatakanaVoicedSoundMark => "halfwidth katakana voiced sound mark",
            HalfwidthandFullwidthForms::HalfwidthKatakanaSemiDashVoicedSoundMark => "halfwidth katakana semi-voiced sound mark",
            HalfwidthandFullwidthForms::HalfwidthHangulFiller => "halfwidth hangul filler",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterKiyeok => "halfwidth hangul letter kiyeok",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterSsangkiyeok => "halfwidth hangul letter ssangkiyeok",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterKiyeokDashSios => "halfwidth hangul letter kiyeok-sios",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterNieun => "halfwidth hangul letter nieun",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterNieunDashCieuc => "halfwidth hangul letter nieun-cieuc",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterNieunDashHieuh => "halfwidth hangul letter nieun-hieuh",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterTikeut => "halfwidth hangul letter tikeut",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterSsangtikeut => "halfwidth hangul letter ssangtikeut",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterRieul => "halfwidth hangul letter rieul",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterRieulDashKiyeok => "halfwidth hangul letter rieul-kiyeok",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterRieulDashMieum => "halfwidth hangul letter rieul-mieum",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterRieulDashPieup => "halfwidth hangul letter rieul-pieup",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterRieulDashSios => "halfwidth hangul letter rieul-sios",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterRieulDashThieuth => "halfwidth hangul letter rieul-thieuth",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterRieulDashPhieuph => "halfwidth hangul letter rieul-phieuph",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterRieulDashHieuh => "halfwidth hangul letter rieul-hieuh",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterMieum => "halfwidth hangul letter mieum",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterPieup => "halfwidth hangul letter pieup",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterSsangpieup => "halfwidth hangul letter ssangpieup",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterPieupDashSios => "halfwidth hangul letter pieup-sios",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterSios => "halfwidth hangul letter sios",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterSsangsios => "halfwidth hangul letter ssangsios",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterIeung => "halfwidth hangul letter ieung",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterCieuc => "halfwidth hangul letter cieuc",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterSsangcieuc => "halfwidth hangul letter ssangcieuc",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterChieuch => "halfwidth hangul letter chieuch",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterKhieukh => "halfwidth hangul letter khieukh",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterThieuth => "halfwidth hangul letter thieuth",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterPhieuph => "halfwidth hangul letter phieuph",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterHieuh => "halfwidth hangul letter hieuh",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterA => "halfwidth hangul letter a",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterAe => "halfwidth hangul letter ae",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterYa => "halfwidth hangul letter ya",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterYae => "halfwidth hangul letter yae",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterEo => "halfwidth hangul letter eo",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterE => "halfwidth hangul letter e",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterYeo => "halfwidth hangul letter yeo",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterYe => "halfwidth hangul letter ye",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterO => "halfwidth hangul letter o",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterWa => "halfwidth hangul letter wa",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterWae => "halfwidth hangul letter wae",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterOe => "halfwidth hangul letter oe",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterYo => "halfwidth hangul letter yo",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterU => "halfwidth hangul letter u",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterWeo => "halfwidth hangul letter weo",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterWe => "halfwidth hangul letter we",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterWi => "halfwidth hangul letter wi",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterYu => "halfwidth hangul letter yu",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterEu => "halfwidth hangul letter eu",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterYi => "halfwidth hangul letter yi",
            HalfwidthandFullwidthForms::HalfwidthHangulLetterI => "halfwidth hangul letter i",
            HalfwidthandFullwidthForms::FullwidthCentSign => "fullwidth cent sign",
            HalfwidthandFullwidthForms::FullwidthPoundSign => "fullwidth pound sign",
            HalfwidthandFullwidthForms::FullwidthNotSign => "fullwidth not sign",
            HalfwidthandFullwidthForms::FullwidthMacron => "fullwidth macron",
            HalfwidthandFullwidthForms::FullwidthBrokenBar => "fullwidth broken bar",
            HalfwidthandFullwidthForms::FullwidthYenSign => "fullwidth yen sign",
            HalfwidthandFullwidthForms::FullwidthWonSign => "fullwidth won sign",
            HalfwidthandFullwidthForms::HalfwidthFormsLightVertical => "halfwidth forms light vertical",
            HalfwidthandFullwidthForms::HalfwidthLeftwardsArrow => "halfwidth leftwards arrow",
            HalfwidthandFullwidthForms::HalfwidthUpwardsArrow => "halfwidth upwards arrow",
            HalfwidthandFullwidthForms::HalfwidthRightwardsArrow => "halfwidth rightwards arrow",
            HalfwidthandFullwidthForms::HalfwidthDownwardsArrow => "halfwidth downwards arrow",
            HalfwidthandFullwidthForms::HalfwidthBlackSquare => "halfwidth black square",
            HalfwidthandFullwidthForms::HalfwidthWhiteCircle => "halfwidth white circle",
        }
    }
}
