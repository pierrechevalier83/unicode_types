
/// An enum to represent all characters in the CJKSymbolsandPunctuation block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum CJKSymbolsandPunctuation {
    /// \u{3000}: '　'
    IdeographicSpace,
    /// \u{3001}: '、'
    IdeographicComma,
    /// \u{3002}: '。'
    IdeographicFullStop,
    /// \u{3003}: '〃'
    DittoMark,
    /// \u{3004}: '〄'
    JapaneseIndustrialStandardSymbol,
    /// \u{3005}: '々'
    IdeographicIterationMark,
    /// \u{3006}: '〆'
    IdeographicClosingMark,
    /// \u{3007}: '〇'
    IdeographicNumberZero,
    /// \u{3008}: '〈'
    LeftAngleBracket,
    /// \u{3009}: '〉'
    RightAngleBracket,
    /// \u{300a}: '《'
    LeftDoubleAngleBracket,
    /// \u{300b}: '》'
    RightDoubleAngleBracket,
    /// \u{300c}: '「'
    LeftCornerBracket,
    /// \u{300d}: '」'
    RightCornerBracket,
    /// \u{300e}: '『'
    LeftWhiteCornerBracket,
    /// \u{300f}: '』'
    RightWhiteCornerBracket,
    /// \u{3010}: '【'
    LeftBlackLenticularBracket,
    /// \u{3011}: '】'
    RightBlackLenticularBracket,
    /// \u{3012}: '〒'
    PostalMark,
    /// \u{3013}: '〓'
    GetaMark,
    /// \u{3014}: '〔'
    LeftTortoiseShellBracket,
    /// \u{3015}: '〕'
    RightTortoiseShellBracket,
    /// \u{3016}: '〖'
    LeftWhiteLenticularBracket,
    /// \u{3017}: '〗'
    RightWhiteLenticularBracket,
    /// \u{3018}: '〘'
    LeftWhiteTortoiseShellBracket,
    /// \u{3019}: '〙'
    RightWhiteTortoiseShellBracket,
    /// \u{301a}: '〚'
    LeftWhiteSquareBracket,
    /// \u{301b}: '〛'
    RightWhiteSquareBracket,
    /// \u{301c}: '〜'
    WaveDash,
    /// \u{301d}: '〝'
    ReversedDoublePrimeQuotationMark,
    /// \u{301e}: '〞'
    DoublePrimeQuotationMark,
    /// \u{301f}: '〟'
    LowDoublePrimeQuotationMark,
    /// \u{3020}: '〠'
    PostalMarkFace,
    /// \u{3021}: '〡'
    HangzhouNumeralOne,
    /// \u{3022}: '〢'
    HangzhouNumeralTwo,
    /// \u{3023}: '〣'
    HangzhouNumeralThree,
    /// \u{3024}: '〤'
    HangzhouNumeralFour,
    /// \u{3025}: '〥'
    HangzhouNumeralFive,
    /// \u{3026}: '〦'
    HangzhouNumeralSix,
    /// \u{3027}: '〧'
    HangzhouNumeralSeven,
    /// \u{3028}: '〨'
    HangzhouNumeralEight,
    /// \u{3029}: '〩'
    HangzhouNumeralNine,
    /// \u{302a}: '〪'
    IdeographicLevelToneMark,
    /// \u{302b}: '〫'
    IdeographicRisingToneMark,
    /// \u{302c}: '〬'
    IdeographicDepartingToneMark,
    /// \u{302d}: '〭'
    IdeographicEnteringToneMark,
    /// \u{302e}: '〮'
    HangulSingleDotToneMark,
    /// \u{302f}: '〯'
    HangulDoubleDotToneMark,
    /// \u{3030}: '〰'
    WavyDash,
    /// \u{3031}: '〱'
    VerticalKanaRepeatMark,
    /// \u{3032}: '〲'
    VerticalKanaRepeatWithVoicedSoundMark,
    /// \u{3033}: '〳'
    VerticalKanaRepeatMarkUpperHalf,
    /// \u{3034}: '〴'
    VerticalKanaRepeatWithVoicedSoundMarkUpperHalf,
    /// \u{3035}: '〵'
    VerticalKanaRepeatMarkLowerHalf,
    /// \u{3036}: '〶'
    CircledPostalMark,
    /// \u{3037}: '〷'
    IdeographicTelegraphLineFeedSeparatorSymbol,
    /// \u{3038}: '〸'
    HangzhouNumeralTen,
    /// \u{3039}: '〹'
    HangzhouNumeralTwenty,
    /// \u{303a}: '〺'
    HangzhouNumeralThirty,
    /// \u{303b}: '〻'
    VerticalIdeographicIterationMark,
    /// \u{303c}: '〼'
    MasuMark,
    /// \u{303d}: '〽'
    PartAlternationMark,
    /// \u{303e}: '〾'
    IdeographicVariationIndicator,
}

impl Into<char> for CJKSymbolsandPunctuation {
    fn into(self) -> char {
        match self {
            CJKSymbolsandPunctuation::IdeographicSpace => '　',
            CJKSymbolsandPunctuation::IdeographicComma => '、',
            CJKSymbolsandPunctuation::IdeographicFullStop => '。',
            CJKSymbolsandPunctuation::DittoMark => '〃',
            CJKSymbolsandPunctuation::JapaneseIndustrialStandardSymbol => '〄',
            CJKSymbolsandPunctuation::IdeographicIterationMark => '々',
            CJKSymbolsandPunctuation::IdeographicClosingMark => '〆',
            CJKSymbolsandPunctuation::IdeographicNumberZero => '〇',
            CJKSymbolsandPunctuation::LeftAngleBracket => '〈',
            CJKSymbolsandPunctuation::RightAngleBracket => '〉',
            CJKSymbolsandPunctuation::LeftDoubleAngleBracket => '《',
            CJKSymbolsandPunctuation::RightDoubleAngleBracket => '》',
            CJKSymbolsandPunctuation::LeftCornerBracket => '「',
            CJKSymbolsandPunctuation::RightCornerBracket => '」',
            CJKSymbolsandPunctuation::LeftWhiteCornerBracket => '『',
            CJKSymbolsandPunctuation::RightWhiteCornerBracket => '』',
            CJKSymbolsandPunctuation::LeftBlackLenticularBracket => '【',
            CJKSymbolsandPunctuation::RightBlackLenticularBracket => '】',
            CJKSymbolsandPunctuation::PostalMark => '〒',
            CJKSymbolsandPunctuation::GetaMark => '〓',
            CJKSymbolsandPunctuation::LeftTortoiseShellBracket => '〔',
            CJKSymbolsandPunctuation::RightTortoiseShellBracket => '〕',
            CJKSymbolsandPunctuation::LeftWhiteLenticularBracket => '〖',
            CJKSymbolsandPunctuation::RightWhiteLenticularBracket => '〗',
            CJKSymbolsandPunctuation::LeftWhiteTortoiseShellBracket => '〘',
            CJKSymbolsandPunctuation::RightWhiteTortoiseShellBracket => '〙',
            CJKSymbolsandPunctuation::LeftWhiteSquareBracket => '〚',
            CJKSymbolsandPunctuation::RightWhiteSquareBracket => '〛',
            CJKSymbolsandPunctuation::WaveDash => '〜',
            CJKSymbolsandPunctuation::ReversedDoublePrimeQuotationMark => '〝',
            CJKSymbolsandPunctuation::DoublePrimeQuotationMark => '〞',
            CJKSymbolsandPunctuation::LowDoublePrimeQuotationMark => '〟',
            CJKSymbolsandPunctuation::PostalMarkFace => '〠',
            CJKSymbolsandPunctuation::HangzhouNumeralOne => '〡',
            CJKSymbolsandPunctuation::HangzhouNumeralTwo => '〢',
            CJKSymbolsandPunctuation::HangzhouNumeralThree => '〣',
            CJKSymbolsandPunctuation::HangzhouNumeralFour => '〤',
            CJKSymbolsandPunctuation::HangzhouNumeralFive => '〥',
            CJKSymbolsandPunctuation::HangzhouNumeralSix => '〦',
            CJKSymbolsandPunctuation::HangzhouNumeralSeven => '〧',
            CJKSymbolsandPunctuation::HangzhouNumeralEight => '〨',
            CJKSymbolsandPunctuation::HangzhouNumeralNine => '〩',
            CJKSymbolsandPunctuation::IdeographicLevelToneMark => '〪',
            CJKSymbolsandPunctuation::IdeographicRisingToneMark => '〫',
            CJKSymbolsandPunctuation::IdeographicDepartingToneMark => '〬',
            CJKSymbolsandPunctuation::IdeographicEnteringToneMark => '〭',
            CJKSymbolsandPunctuation::HangulSingleDotToneMark => '〮',
            CJKSymbolsandPunctuation::HangulDoubleDotToneMark => '〯',
            CJKSymbolsandPunctuation::WavyDash => '〰',
            CJKSymbolsandPunctuation::VerticalKanaRepeatMark => '〱',
            CJKSymbolsandPunctuation::VerticalKanaRepeatWithVoicedSoundMark => '〲',
            CJKSymbolsandPunctuation::VerticalKanaRepeatMarkUpperHalf => '〳',
            CJKSymbolsandPunctuation::VerticalKanaRepeatWithVoicedSoundMarkUpperHalf => '〴',
            CJKSymbolsandPunctuation::VerticalKanaRepeatMarkLowerHalf => '〵',
            CJKSymbolsandPunctuation::CircledPostalMark => '〶',
            CJKSymbolsandPunctuation::IdeographicTelegraphLineFeedSeparatorSymbol => '〷',
            CJKSymbolsandPunctuation::HangzhouNumeralTen => '〸',
            CJKSymbolsandPunctuation::HangzhouNumeralTwenty => '〹',
            CJKSymbolsandPunctuation::HangzhouNumeralThirty => '〺',
            CJKSymbolsandPunctuation::VerticalIdeographicIterationMark => '〻',
            CJKSymbolsandPunctuation::MasuMark => '〼',
            CJKSymbolsandPunctuation::PartAlternationMark => '〽',
            CJKSymbolsandPunctuation::IdeographicVariationIndicator => '〾',
        }
    }
}

impl std::convert::TryFrom<char> for CJKSymbolsandPunctuation {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '　' => Ok(CJKSymbolsandPunctuation::IdeographicSpace),
            '、' => Ok(CJKSymbolsandPunctuation::IdeographicComma),
            '。' => Ok(CJKSymbolsandPunctuation::IdeographicFullStop),
            '〃' => Ok(CJKSymbolsandPunctuation::DittoMark),
            '〄' => Ok(CJKSymbolsandPunctuation::JapaneseIndustrialStandardSymbol),
            '々' => Ok(CJKSymbolsandPunctuation::IdeographicIterationMark),
            '〆' => Ok(CJKSymbolsandPunctuation::IdeographicClosingMark),
            '〇' => Ok(CJKSymbolsandPunctuation::IdeographicNumberZero),
            '〈' => Ok(CJKSymbolsandPunctuation::LeftAngleBracket),
            '〉' => Ok(CJKSymbolsandPunctuation::RightAngleBracket),
            '《' => Ok(CJKSymbolsandPunctuation::LeftDoubleAngleBracket),
            '》' => Ok(CJKSymbolsandPunctuation::RightDoubleAngleBracket),
            '「' => Ok(CJKSymbolsandPunctuation::LeftCornerBracket),
            '」' => Ok(CJKSymbolsandPunctuation::RightCornerBracket),
            '『' => Ok(CJKSymbolsandPunctuation::LeftWhiteCornerBracket),
            '』' => Ok(CJKSymbolsandPunctuation::RightWhiteCornerBracket),
            '【' => Ok(CJKSymbolsandPunctuation::LeftBlackLenticularBracket),
            '】' => Ok(CJKSymbolsandPunctuation::RightBlackLenticularBracket),
            '〒' => Ok(CJKSymbolsandPunctuation::PostalMark),
            '〓' => Ok(CJKSymbolsandPunctuation::GetaMark),
            '〔' => Ok(CJKSymbolsandPunctuation::LeftTortoiseShellBracket),
            '〕' => Ok(CJKSymbolsandPunctuation::RightTortoiseShellBracket),
            '〖' => Ok(CJKSymbolsandPunctuation::LeftWhiteLenticularBracket),
            '〗' => Ok(CJKSymbolsandPunctuation::RightWhiteLenticularBracket),
            '〘' => Ok(CJKSymbolsandPunctuation::LeftWhiteTortoiseShellBracket),
            '〙' => Ok(CJKSymbolsandPunctuation::RightWhiteTortoiseShellBracket),
            '〚' => Ok(CJKSymbolsandPunctuation::LeftWhiteSquareBracket),
            '〛' => Ok(CJKSymbolsandPunctuation::RightWhiteSquareBracket),
            '〜' => Ok(CJKSymbolsandPunctuation::WaveDash),
            '〝' => Ok(CJKSymbolsandPunctuation::ReversedDoublePrimeQuotationMark),
            '〞' => Ok(CJKSymbolsandPunctuation::DoublePrimeQuotationMark),
            '〟' => Ok(CJKSymbolsandPunctuation::LowDoublePrimeQuotationMark),
            '〠' => Ok(CJKSymbolsandPunctuation::PostalMarkFace),
            '〡' => Ok(CJKSymbolsandPunctuation::HangzhouNumeralOne),
            '〢' => Ok(CJKSymbolsandPunctuation::HangzhouNumeralTwo),
            '〣' => Ok(CJKSymbolsandPunctuation::HangzhouNumeralThree),
            '〤' => Ok(CJKSymbolsandPunctuation::HangzhouNumeralFour),
            '〥' => Ok(CJKSymbolsandPunctuation::HangzhouNumeralFive),
            '〦' => Ok(CJKSymbolsandPunctuation::HangzhouNumeralSix),
            '〧' => Ok(CJKSymbolsandPunctuation::HangzhouNumeralSeven),
            '〨' => Ok(CJKSymbolsandPunctuation::HangzhouNumeralEight),
            '〩' => Ok(CJKSymbolsandPunctuation::HangzhouNumeralNine),
            '〪' => Ok(CJKSymbolsandPunctuation::IdeographicLevelToneMark),
            '〫' => Ok(CJKSymbolsandPunctuation::IdeographicRisingToneMark),
            '〬' => Ok(CJKSymbolsandPunctuation::IdeographicDepartingToneMark),
            '〭' => Ok(CJKSymbolsandPunctuation::IdeographicEnteringToneMark),
            '〮' => Ok(CJKSymbolsandPunctuation::HangulSingleDotToneMark),
            '〯' => Ok(CJKSymbolsandPunctuation::HangulDoubleDotToneMark),
            '〰' => Ok(CJKSymbolsandPunctuation::WavyDash),
            '〱' => Ok(CJKSymbolsandPunctuation::VerticalKanaRepeatMark),
            '〲' => Ok(CJKSymbolsandPunctuation::VerticalKanaRepeatWithVoicedSoundMark),
            '〳' => Ok(CJKSymbolsandPunctuation::VerticalKanaRepeatMarkUpperHalf),
            '〴' => Ok(CJKSymbolsandPunctuation::VerticalKanaRepeatWithVoicedSoundMarkUpperHalf),
            '〵' => Ok(CJKSymbolsandPunctuation::VerticalKanaRepeatMarkLowerHalf),
            '〶' => Ok(CJKSymbolsandPunctuation::CircledPostalMark),
            '〷' => Ok(CJKSymbolsandPunctuation::IdeographicTelegraphLineFeedSeparatorSymbol),
            '〸' => Ok(CJKSymbolsandPunctuation::HangzhouNumeralTen),
            '〹' => Ok(CJKSymbolsandPunctuation::HangzhouNumeralTwenty),
            '〺' => Ok(CJKSymbolsandPunctuation::HangzhouNumeralThirty),
            '〻' => Ok(CJKSymbolsandPunctuation::VerticalIdeographicIterationMark),
            '〼' => Ok(CJKSymbolsandPunctuation::MasuMark),
            '〽' => Ok(CJKSymbolsandPunctuation::PartAlternationMark),
            '〾' => Ok(CJKSymbolsandPunctuation::IdeographicVariationIndicator),
            _ => Err(()),
        }
    }
}

impl Into<u32> for CJKSymbolsandPunctuation {
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

impl std::convert::TryFrom<u32> for CJKSymbolsandPunctuation {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for CJKSymbolsandPunctuation {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl CJKSymbolsandPunctuation {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        CJKSymbolsandPunctuation::IdeographicSpace
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("CJKSymbolsandPunctuation{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
