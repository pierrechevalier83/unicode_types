/// \u{3000} → \u{303f}\
///\
/// 　 、 。 〃 〄 々 〆 〇 〈 〉 《 》 「 」 『 』\
/// 【 】 〒 〓 〔 〕 〖 〗 〘 〙 〚 〛 〜 〝 〞 〟\
/// 〠 〡 〢 〣 〤 〥 〦 〧 〨 〩 〪 〫 〬 〭 〮 〯\
/// 〰 〱 〲 〳 〴 〵 〶 〷 〸 〹 〺 〻 〼 〽 〾\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{3000}: '　'
    pub const IDEOGRAPHIC_SPACE: char = '　';
    /// \u{3001}: '、'
    pub const IDEOGRAPHIC_COMMA: char = '、';
    /// \u{3002}: '。'
    pub const IDEOGRAPHIC_FULL_STOP: char = '。';
    /// \u{3003}: '〃'
    pub const DITTO_MARK: char = '〃';
    /// \u{3004}: '〄'
    pub const JAPANESE_INDUSTRIAL_STANDARD_SYMBOL: char = '〄';
    /// \u{3005}: '々'
    pub const IDEOGRAPHIC_ITERATION_MARK: char = '々';
    /// \u{3006}: '〆'
    pub const IDEOGRAPHIC_CLOSING_MARK: char = '〆';
    /// \u{3007}: '〇'
    pub const IDEOGRAPHIC_NUMBER_ZERO: char = '〇';
    /// \u{3008}: '〈'
    pub const LEFT_ANGLE_BRACKET: char = '〈';
    /// \u{3009}: '〉'
    pub const RIGHT_ANGLE_BRACKET: char = '〉';
    /// \u{300a}: '《'
    pub const LEFT_DOUBLE_ANGLE_BRACKET: char = '《';
    /// \u{300b}: '》'
    pub const RIGHT_DOUBLE_ANGLE_BRACKET: char = '》';
    /// \u{300c}: '「'
    pub const LEFT_CORNER_BRACKET: char = '「';
    /// \u{300d}: '」'
    pub const RIGHT_CORNER_BRACKET: char = '」';
    /// \u{300e}: '『'
    pub const LEFT_WHITE_CORNER_BRACKET: char = '『';
    /// \u{300f}: '』'
    pub const RIGHT_WHITE_CORNER_BRACKET: char = '』';
    /// \u{3010}: '【'
    pub const LEFT_BLACK_LENTICULAR_BRACKET: char = '【';
    /// \u{3011}: '】'
    pub const RIGHT_BLACK_LENTICULAR_BRACKET: char = '】';
    /// \u{3012}: '〒'
    pub const POSTAL_MARK: char = '〒';
    /// \u{3013}: '〓'
    pub const GETA_MARK: char = '〓';
    /// \u{3014}: '〔'
    pub const LEFT_TORTOISE_SHELL_BRACKET: char = '〔';
    /// \u{3015}: '〕'
    pub const RIGHT_TORTOISE_SHELL_BRACKET: char = '〕';
    /// \u{3016}: '〖'
    pub const LEFT_WHITE_LENTICULAR_BRACKET: char = '〖';
    /// \u{3017}: '〗'
    pub const RIGHT_WHITE_LENTICULAR_BRACKET: char = '〗';
    /// \u{3018}: '〘'
    pub const LEFT_WHITE_TORTOISE_SHELL_BRACKET: char = '〘';
    /// \u{3019}: '〙'
    pub const RIGHT_WHITE_TORTOISE_SHELL_BRACKET: char = '〙';
    /// \u{301a}: '〚'
    pub const LEFT_WHITE_SQUARE_BRACKET: char = '〚';
    /// \u{301b}: '〛'
    pub const RIGHT_WHITE_SQUARE_BRACKET: char = '〛';
    /// \u{301c}: '〜'
    pub const WAVE_DASH: char = '〜';
    /// \u{301d}: '〝'
    pub const REVERSED_DOUBLE_PRIME_QUOTATION_MARK: char = '〝';
    /// \u{301e}: '〞'
    pub const DOUBLE_PRIME_QUOTATION_MARK: char = '〞';
    /// \u{301f}: '〟'
    pub const LOW_DOUBLE_PRIME_QUOTATION_MARK: char = '〟';
    /// \u{3020}: '〠'
    pub const POSTAL_MARK_FACE: char = '〠';
    /// \u{3021}: '〡'
    pub const HANGZHOU_NUMERAL_ONE: char = '〡';
    /// \u{3022}: '〢'
    pub const HANGZHOU_NUMERAL_TWO: char = '〢';
    /// \u{3023}: '〣'
    pub const HANGZHOU_NUMERAL_THREE: char = '〣';
    /// \u{3024}: '〤'
    pub const HANGZHOU_NUMERAL_FOUR: char = '〤';
    /// \u{3025}: '〥'
    pub const HANGZHOU_NUMERAL_FIVE: char = '〥';
    /// \u{3026}: '〦'
    pub const HANGZHOU_NUMERAL_SIX: char = '〦';
    /// \u{3027}: '〧'
    pub const HANGZHOU_NUMERAL_SEVEN: char = '〧';
    /// \u{3028}: '〨'
    pub const HANGZHOU_NUMERAL_EIGHT: char = '〨';
    /// \u{3029}: '〩'
    pub const HANGZHOU_NUMERAL_NINE: char = '〩';
    /// \u{302a}: '〪'
    pub const IDEOGRAPHIC_LEVEL_TONE_MARK: char = '〪';
    /// \u{302b}: '〫'
    pub const IDEOGRAPHIC_RISING_TONE_MARK: char = '〫';
    /// \u{302c}: '〬'
    pub const IDEOGRAPHIC_DEPARTING_TONE_MARK: char = '〬';
    /// \u{302d}: '〭'
    pub const IDEOGRAPHIC_ENTERING_TONE_MARK: char = '〭';
    /// \u{302e}: '〮'
    pub const HANGUL_SINGLE_DOT_TONE_MARK: char = '〮';
    /// \u{302f}: '〯'
    pub const HANGUL_DOUBLE_DOT_TONE_MARK: char = '〯';
    /// \u{3030}: '〰'
    pub const WAVY_DASH: char = '〰';
    /// \u{3031}: '〱'
    pub const VERTICAL_KANA_REPEAT_MARK: char = '〱';
    /// \u{3032}: '〲'
    pub const VERTICAL_KANA_REPEAT_WITH_VOICED_SOUND_MARK: char = '〲';
    /// \u{3033}: '〳'
    pub const VERTICAL_KANA_REPEAT_MARK_UPPER_HALF: char = '〳';
    /// \u{3034}: '〴'
    pub const VERTICAL_KANA_REPEAT_WITH_VOICED_SOUND_MARK_UPPER_HALF: char = '〴';
    /// \u{3035}: '〵'
    pub const VERTICAL_KANA_REPEAT_MARK_LOWER_HALF: char = '〵';
    /// \u{3036}: '〶'
    pub const CIRCLED_POSTAL_MARK: char = '〶';
    /// \u{3037}: '〷'
    pub const IDEOGRAPHIC_TELEGRAPH_LINE_FEED_SEPARATOR_SYMBOL: char = '〷';
    /// \u{3038}: '〸'
    pub const HANGZHOU_NUMERAL_TEN: char = '〸';
    /// \u{3039}: '〹'
    pub const HANGZHOU_NUMERAL_TWENTY: char = '〹';
    /// \u{303a}: '〺'
    pub const HANGZHOU_NUMERAL_THIRTY: char = '〺';
    /// \u{303b}: '〻'
    pub const VERTICAL_IDEOGRAPHIC_ITERATION_MARK: char = '〻';
    /// \u{303c}: '〼'
    pub const MASU_MARK: char = '〼';
    /// \u{303d}: '〽'
    pub const PART_ALTERNATION_MARK: char = '〽';
    /// \u{303e}: '〾'
    pub const IDEOGRAPHIC_VARIATION_INDICATOR: char = '〾';
}

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
        use constants::*;
        match self {
            CJKSymbolsandPunctuation::IdeographicSpace => IDEOGRAPHIC_SPACE,
            CJKSymbolsandPunctuation::IdeographicComma => IDEOGRAPHIC_COMMA,
            CJKSymbolsandPunctuation::IdeographicFullStop => IDEOGRAPHIC_FULL_STOP,
            CJKSymbolsandPunctuation::DittoMark => DITTO_MARK,
            CJKSymbolsandPunctuation::JapaneseIndustrialStandardSymbol => JAPANESE_INDUSTRIAL_STANDARD_SYMBOL,
            CJKSymbolsandPunctuation::IdeographicIterationMark => IDEOGRAPHIC_ITERATION_MARK,
            CJKSymbolsandPunctuation::IdeographicClosingMark => IDEOGRAPHIC_CLOSING_MARK,
            CJKSymbolsandPunctuation::IdeographicNumberZero => IDEOGRAPHIC_NUMBER_ZERO,
            CJKSymbolsandPunctuation::LeftAngleBracket => LEFT_ANGLE_BRACKET,
            CJKSymbolsandPunctuation::RightAngleBracket => RIGHT_ANGLE_BRACKET,
            CJKSymbolsandPunctuation::LeftDoubleAngleBracket => LEFT_DOUBLE_ANGLE_BRACKET,
            CJKSymbolsandPunctuation::RightDoubleAngleBracket => RIGHT_DOUBLE_ANGLE_BRACKET,
            CJKSymbolsandPunctuation::LeftCornerBracket => LEFT_CORNER_BRACKET,
            CJKSymbolsandPunctuation::RightCornerBracket => RIGHT_CORNER_BRACKET,
            CJKSymbolsandPunctuation::LeftWhiteCornerBracket => LEFT_WHITE_CORNER_BRACKET,
            CJKSymbolsandPunctuation::RightWhiteCornerBracket => RIGHT_WHITE_CORNER_BRACKET,
            CJKSymbolsandPunctuation::LeftBlackLenticularBracket => LEFT_BLACK_LENTICULAR_BRACKET,
            CJKSymbolsandPunctuation::RightBlackLenticularBracket => RIGHT_BLACK_LENTICULAR_BRACKET,
            CJKSymbolsandPunctuation::PostalMark => POSTAL_MARK,
            CJKSymbolsandPunctuation::GetaMark => GETA_MARK,
            CJKSymbolsandPunctuation::LeftTortoiseShellBracket => LEFT_TORTOISE_SHELL_BRACKET,
            CJKSymbolsandPunctuation::RightTortoiseShellBracket => RIGHT_TORTOISE_SHELL_BRACKET,
            CJKSymbolsandPunctuation::LeftWhiteLenticularBracket => LEFT_WHITE_LENTICULAR_BRACKET,
            CJKSymbolsandPunctuation::RightWhiteLenticularBracket => RIGHT_WHITE_LENTICULAR_BRACKET,
            CJKSymbolsandPunctuation::LeftWhiteTortoiseShellBracket => LEFT_WHITE_TORTOISE_SHELL_BRACKET,
            CJKSymbolsandPunctuation::RightWhiteTortoiseShellBracket => RIGHT_WHITE_TORTOISE_SHELL_BRACKET,
            CJKSymbolsandPunctuation::LeftWhiteSquareBracket => LEFT_WHITE_SQUARE_BRACKET,
            CJKSymbolsandPunctuation::RightWhiteSquareBracket => RIGHT_WHITE_SQUARE_BRACKET,
            CJKSymbolsandPunctuation::WaveDash => WAVE_DASH,
            CJKSymbolsandPunctuation::ReversedDoublePrimeQuotationMark => REVERSED_DOUBLE_PRIME_QUOTATION_MARK,
            CJKSymbolsandPunctuation::DoublePrimeQuotationMark => DOUBLE_PRIME_QUOTATION_MARK,
            CJKSymbolsandPunctuation::LowDoublePrimeQuotationMark => LOW_DOUBLE_PRIME_QUOTATION_MARK,
            CJKSymbolsandPunctuation::PostalMarkFace => POSTAL_MARK_FACE,
            CJKSymbolsandPunctuation::HangzhouNumeralOne => HANGZHOU_NUMERAL_ONE,
            CJKSymbolsandPunctuation::HangzhouNumeralTwo => HANGZHOU_NUMERAL_TWO,
            CJKSymbolsandPunctuation::HangzhouNumeralThree => HANGZHOU_NUMERAL_THREE,
            CJKSymbolsandPunctuation::HangzhouNumeralFour => HANGZHOU_NUMERAL_FOUR,
            CJKSymbolsandPunctuation::HangzhouNumeralFive => HANGZHOU_NUMERAL_FIVE,
            CJKSymbolsandPunctuation::HangzhouNumeralSix => HANGZHOU_NUMERAL_SIX,
            CJKSymbolsandPunctuation::HangzhouNumeralSeven => HANGZHOU_NUMERAL_SEVEN,
            CJKSymbolsandPunctuation::HangzhouNumeralEight => HANGZHOU_NUMERAL_EIGHT,
            CJKSymbolsandPunctuation::HangzhouNumeralNine => HANGZHOU_NUMERAL_NINE,
            CJKSymbolsandPunctuation::IdeographicLevelToneMark => IDEOGRAPHIC_LEVEL_TONE_MARK,
            CJKSymbolsandPunctuation::IdeographicRisingToneMark => IDEOGRAPHIC_RISING_TONE_MARK,
            CJKSymbolsandPunctuation::IdeographicDepartingToneMark => IDEOGRAPHIC_DEPARTING_TONE_MARK,
            CJKSymbolsandPunctuation::IdeographicEnteringToneMark => IDEOGRAPHIC_ENTERING_TONE_MARK,
            CJKSymbolsandPunctuation::HangulSingleDotToneMark => HANGUL_SINGLE_DOT_TONE_MARK,
            CJKSymbolsandPunctuation::HangulDoubleDotToneMark => HANGUL_DOUBLE_DOT_TONE_MARK,
            CJKSymbolsandPunctuation::WavyDash => WAVY_DASH,
            CJKSymbolsandPunctuation::VerticalKanaRepeatMark => VERTICAL_KANA_REPEAT_MARK,
            CJKSymbolsandPunctuation::VerticalKanaRepeatWithVoicedSoundMark => VERTICAL_KANA_REPEAT_WITH_VOICED_SOUND_MARK,
            CJKSymbolsandPunctuation::VerticalKanaRepeatMarkUpperHalf => VERTICAL_KANA_REPEAT_MARK_UPPER_HALF,
            CJKSymbolsandPunctuation::VerticalKanaRepeatWithVoicedSoundMarkUpperHalf => VERTICAL_KANA_REPEAT_WITH_VOICED_SOUND_MARK_UPPER_HALF,
            CJKSymbolsandPunctuation::VerticalKanaRepeatMarkLowerHalf => VERTICAL_KANA_REPEAT_MARK_LOWER_HALF,
            CJKSymbolsandPunctuation::CircledPostalMark => CIRCLED_POSTAL_MARK,
            CJKSymbolsandPunctuation::IdeographicTelegraphLineFeedSeparatorSymbol => IDEOGRAPHIC_TELEGRAPH_LINE_FEED_SEPARATOR_SYMBOL,
            CJKSymbolsandPunctuation::HangzhouNumeralTen => HANGZHOU_NUMERAL_TEN,
            CJKSymbolsandPunctuation::HangzhouNumeralTwenty => HANGZHOU_NUMERAL_TWENTY,
            CJKSymbolsandPunctuation::HangzhouNumeralThirty => HANGZHOU_NUMERAL_THIRTY,
            CJKSymbolsandPunctuation::VerticalIdeographicIterationMark => VERTICAL_IDEOGRAPHIC_ITERATION_MARK,
            CJKSymbolsandPunctuation::MasuMark => MASU_MARK,
            CJKSymbolsandPunctuation::PartAlternationMark => PART_ALTERNATION_MARK,
            CJKSymbolsandPunctuation::IdeographicVariationIndicator => IDEOGRAPHIC_VARIATION_INDICATOR,
        }
    }
}

impl std::convert::TryFrom<char> for CJKSymbolsandPunctuation {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            IDEOGRAPHIC_SPACE => Ok(CJKSymbolsandPunctuation::IdeographicSpace),
            IDEOGRAPHIC_COMMA => Ok(CJKSymbolsandPunctuation::IdeographicComma),
            IDEOGRAPHIC_FULL_STOP => Ok(CJKSymbolsandPunctuation::IdeographicFullStop),
            DITTO_MARK => Ok(CJKSymbolsandPunctuation::DittoMark),
            JAPANESE_INDUSTRIAL_STANDARD_SYMBOL => Ok(CJKSymbolsandPunctuation::JapaneseIndustrialStandardSymbol),
            IDEOGRAPHIC_ITERATION_MARK => Ok(CJKSymbolsandPunctuation::IdeographicIterationMark),
            IDEOGRAPHIC_CLOSING_MARK => Ok(CJKSymbolsandPunctuation::IdeographicClosingMark),
            IDEOGRAPHIC_NUMBER_ZERO => Ok(CJKSymbolsandPunctuation::IdeographicNumberZero),
            LEFT_ANGLE_BRACKET => Ok(CJKSymbolsandPunctuation::LeftAngleBracket),
            RIGHT_ANGLE_BRACKET => Ok(CJKSymbolsandPunctuation::RightAngleBracket),
            LEFT_DOUBLE_ANGLE_BRACKET => Ok(CJKSymbolsandPunctuation::LeftDoubleAngleBracket),
            RIGHT_DOUBLE_ANGLE_BRACKET => Ok(CJKSymbolsandPunctuation::RightDoubleAngleBracket),
            LEFT_CORNER_BRACKET => Ok(CJKSymbolsandPunctuation::LeftCornerBracket),
            RIGHT_CORNER_BRACKET => Ok(CJKSymbolsandPunctuation::RightCornerBracket),
            LEFT_WHITE_CORNER_BRACKET => Ok(CJKSymbolsandPunctuation::LeftWhiteCornerBracket),
            RIGHT_WHITE_CORNER_BRACKET => Ok(CJKSymbolsandPunctuation::RightWhiteCornerBracket),
            LEFT_BLACK_LENTICULAR_BRACKET => Ok(CJKSymbolsandPunctuation::LeftBlackLenticularBracket),
            RIGHT_BLACK_LENTICULAR_BRACKET => Ok(CJKSymbolsandPunctuation::RightBlackLenticularBracket),
            POSTAL_MARK => Ok(CJKSymbolsandPunctuation::PostalMark),
            GETA_MARK => Ok(CJKSymbolsandPunctuation::GetaMark),
            LEFT_TORTOISE_SHELL_BRACKET => Ok(CJKSymbolsandPunctuation::LeftTortoiseShellBracket),
            RIGHT_TORTOISE_SHELL_BRACKET => Ok(CJKSymbolsandPunctuation::RightTortoiseShellBracket),
            LEFT_WHITE_LENTICULAR_BRACKET => Ok(CJKSymbolsandPunctuation::LeftWhiteLenticularBracket),
            RIGHT_WHITE_LENTICULAR_BRACKET => Ok(CJKSymbolsandPunctuation::RightWhiteLenticularBracket),
            LEFT_WHITE_TORTOISE_SHELL_BRACKET => Ok(CJKSymbolsandPunctuation::LeftWhiteTortoiseShellBracket),
            RIGHT_WHITE_TORTOISE_SHELL_BRACKET => Ok(CJKSymbolsandPunctuation::RightWhiteTortoiseShellBracket),
            LEFT_WHITE_SQUARE_BRACKET => Ok(CJKSymbolsandPunctuation::LeftWhiteSquareBracket),
            RIGHT_WHITE_SQUARE_BRACKET => Ok(CJKSymbolsandPunctuation::RightWhiteSquareBracket),
            WAVE_DASH => Ok(CJKSymbolsandPunctuation::WaveDash),
            REVERSED_DOUBLE_PRIME_QUOTATION_MARK => Ok(CJKSymbolsandPunctuation::ReversedDoublePrimeQuotationMark),
            DOUBLE_PRIME_QUOTATION_MARK => Ok(CJKSymbolsandPunctuation::DoublePrimeQuotationMark),
            LOW_DOUBLE_PRIME_QUOTATION_MARK => Ok(CJKSymbolsandPunctuation::LowDoublePrimeQuotationMark),
            POSTAL_MARK_FACE => Ok(CJKSymbolsandPunctuation::PostalMarkFace),
            HANGZHOU_NUMERAL_ONE => Ok(CJKSymbolsandPunctuation::HangzhouNumeralOne),
            HANGZHOU_NUMERAL_TWO => Ok(CJKSymbolsandPunctuation::HangzhouNumeralTwo),
            HANGZHOU_NUMERAL_THREE => Ok(CJKSymbolsandPunctuation::HangzhouNumeralThree),
            HANGZHOU_NUMERAL_FOUR => Ok(CJKSymbolsandPunctuation::HangzhouNumeralFour),
            HANGZHOU_NUMERAL_FIVE => Ok(CJKSymbolsandPunctuation::HangzhouNumeralFive),
            HANGZHOU_NUMERAL_SIX => Ok(CJKSymbolsandPunctuation::HangzhouNumeralSix),
            HANGZHOU_NUMERAL_SEVEN => Ok(CJKSymbolsandPunctuation::HangzhouNumeralSeven),
            HANGZHOU_NUMERAL_EIGHT => Ok(CJKSymbolsandPunctuation::HangzhouNumeralEight),
            HANGZHOU_NUMERAL_NINE => Ok(CJKSymbolsandPunctuation::HangzhouNumeralNine),
            IDEOGRAPHIC_LEVEL_TONE_MARK => Ok(CJKSymbolsandPunctuation::IdeographicLevelToneMark),
            IDEOGRAPHIC_RISING_TONE_MARK => Ok(CJKSymbolsandPunctuation::IdeographicRisingToneMark),
            IDEOGRAPHIC_DEPARTING_TONE_MARK => Ok(CJKSymbolsandPunctuation::IdeographicDepartingToneMark),
            IDEOGRAPHIC_ENTERING_TONE_MARK => Ok(CJKSymbolsandPunctuation::IdeographicEnteringToneMark),
            HANGUL_SINGLE_DOT_TONE_MARK => Ok(CJKSymbolsandPunctuation::HangulSingleDotToneMark),
            HANGUL_DOUBLE_DOT_TONE_MARK => Ok(CJKSymbolsandPunctuation::HangulDoubleDotToneMark),
            WAVY_DASH => Ok(CJKSymbolsandPunctuation::WavyDash),
            VERTICAL_KANA_REPEAT_MARK => Ok(CJKSymbolsandPunctuation::VerticalKanaRepeatMark),
            VERTICAL_KANA_REPEAT_WITH_VOICED_SOUND_MARK => Ok(CJKSymbolsandPunctuation::VerticalKanaRepeatWithVoicedSoundMark),
            VERTICAL_KANA_REPEAT_MARK_UPPER_HALF => Ok(CJKSymbolsandPunctuation::VerticalKanaRepeatMarkUpperHalf),
            VERTICAL_KANA_REPEAT_WITH_VOICED_SOUND_MARK_UPPER_HALF => Ok(CJKSymbolsandPunctuation::VerticalKanaRepeatWithVoicedSoundMarkUpperHalf),
            VERTICAL_KANA_REPEAT_MARK_LOWER_HALF => Ok(CJKSymbolsandPunctuation::VerticalKanaRepeatMarkLowerHalf),
            CIRCLED_POSTAL_MARK => Ok(CJKSymbolsandPunctuation::CircledPostalMark),
            IDEOGRAPHIC_TELEGRAPH_LINE_FEED_SEPARATOR_SYMBOL => Ok(CJKSymbolsandPunctuation::IdeographicTelegraphLineFeedSeparatorSymbol),
            HANGZHOU_NUMERAL_TEN => Ok(CJKSymbolsandPunctuation::HangzhouNumeralTen),
            HANGZHOU_NUMERAL_TWENTY => Ok(CJKSymbolsandPunctuation::HangzhouNumeralTwenty),
            HANGZHOU_NUMERAL_THIRTY => Ok(CJKSymbolsandPunctuation::HangzhouNumeralThirty),
            VERTICAL_IDEOGRAPHIC_ITERATION_MARK => Ok(CJKSymbolsandPunctuation::VerticalIdeographicIterationMark),
            MASU_MARK => Ok(CJKSymbolsandPunctuation::MasuMark),
            PART_ALTERNATION_MARK => Ok(CJKSymbolsandPunctuation::PartAlternationMark),
            IDEOGRAPHIC_VARIATION_INDICATOR => Ok(CJKSymbolsandPunctuation::IdeographicVariationIndicator),
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

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            CJKSymbolsandPunctuation::IdeographicSpace => "ideographic space",
            CJKSymbolsandPunctuation::IdeographicComma => "ideographic comma",
            CJKSymbolsandPunctuation::IdeographicFullStop => "ideographic full stop",
            CJKSymbolsandPunctuation::DittoMark => "ditto mark",
            CJKSymbolsandPunctuation::JapaneseIndustrialStandardSymbol => "japanese industrial standard symbol",
            CJKSymbolsandPunctuation::IdeographicIterationMark => "ideographic iteration mark",
            CJKSymbolsandPunctuation::IdeographicClosingMark => "ideographic closing mark",
            CJKSymbolsandPunctuation::IdeographicNumberZero => "ideographic number zero",
            CJKSymbolsandPunctuation::LeftAngleBracket => "left angle bracket",
            CJKSymbolsandPunctuation::RightAngleBracket => "right angle bracket",
            CJKSymbolsandPunctuation::LeftDoubleAngleBracket => "left double angle bracket",
            CJKSymbolsandPunctuation::RightDoubleAngleBracket => "right double angle bracket",
            CJKSymbolsandPunctuation::LeftCornerBracket => "left corner bracket",
            CJKSymbolsandPunctuation::RightCornerBracket => "right corner bracket",
            CJKSymbolsandPunctuation::LeftWhiteCornerBracket => "left white corner bracket",
            CJKSymbolsandPunctuation::RightWhiteCornerBracket => "right white corner bracket",
            CJKSymbolsandPunctuation::LeftBlackLenticularBracket => "left black lenticular bracket",
            CJKSymbolsandPunctuation::RightBlackLenticularBracket => "right black lenticular bracket",
            CJKSymbolsandPunctuation::PostalMark => "postal mark",
            CJKSymbolsandPunctuation::GetaMark => "geta mark",
            CJKSymbolsandPunctuation::LeftTortoiseShellBracket => "left tortoise shell bracket",
            CJKSymbolsandPunctuation::RightTortoiseShellBracket => "right tortoise shell bracket",
            CJKSymbolsandPunctuation::LeftWhiteLenticularBracket => "left white lenticular bracket",
            CJKSymbolsandPunctuation::RightWhiteLenticularBracket => "right white lenticular bracket",
            CJKSymbolsandPunctuation::LeftWhiteTortoiseShellBracket => "left white tortoise shell bracket",
            CJKSymbolsandPunctuation::RightWhiteTortoiseShellBracket => "right white tortoise shell bracket",
            CJKSymbolsandPunctuation::LeftWhiteSquareBracket => "left white square bracket",
            CJKSymbolsandPunctuation::RightWhiteSquareBracket => "right white square bracket",
            CJKSymbolsandPunctuation::WaveDash => "wave dash",
            CJKSymbolsandPunctuation::ReversedDoublePrimeQuotationMark => "reversed double prime quotation mark",
            CJKSymbolsandPunctuation::DoublePrimeQuotationMark => "double prime quotation mark",
            CJKSymbolsandPunctuation::LowDoublePrimeQuotationMark => "low double prime quotation mark",
            CJKSymbolsandPunctuation::PostalMarkFace => "postal mark face",
            CJKSymbolsandPunctuation::HangzhouNumeralOne => "hangzhou numeral one",
            CJKSymbolsandPunctuation::HangzhouNumeralTwo => "hangzhou numeral two",
            CJKSymbolsandPunctuation::HangzhouNumeralThree => "hangzhou numeral three",
            CJKSymbolsandPunctuation::HangzhouNumeralFour => "hangzhou numeral four",
            CJKSymbolsandPunctuation::HangzhouNumeralFive => "hangzhou numeral five",
            CJKSymbolsandPunctuation::HangzhouNumeralSix => "hangzhou numeral six",
            CJKSymbolsandPunctuation::HangzhouNumeralSeven => "hangzhou numeral seven",
            CJKSymbolsandPunctuation::HangzhouNumeralEight => "hangzhou numeral eight",
            CJKSymbolsandPunctuation::HangzhouNumeralNine => "hangzhou numeral nine",
            CJKSymbolsandPunctuation::IdeographicLevelToneMark => "ideographic level tone mark",
            CJKSymbolsandPunctuation::IdeographicRisingToneMark => "ideographic rising tone mark",
            CJKSymbolsandPunctuation::IdeographicDepartingToneMark => "ideographic departing tone mark",
            CJKSymbolsandPunctuation::IdeographicEnteringToneMark => "ideographic entering tone mark",
            CJKSymbolsandPunctuation::HangulSingleDotToneMark => "hangul single dot tone mark",
            CJKSymbolsandPunctuation::HangulDoubleDotToneMark => "hangul double dot tone mark",
            CJKSymbolsandPunctuation::WavyDash => "wavy dash",
            CJKSymbolsandPunctuation::VerticalKanaRepeatMark => "vertical kana repeat mark",
            CJKSymbolsandPunctuation::VerticalKanaRepeatWithVoicedSoundMark => "vertical kana repeat with voiced sound mark",
            CJKSymbolsandPunctuation::VerticalKanaRepeatMarkUpperHalf => "vertical kana repeat mark upper half",
            CJKSymbolsandPunctuation::VerticalKanaRepeatWithVoicedSoundMarkUpperHalf => "vertical kana repeat with voiced sound mark upper half",
            CJKSymbolsandPunctuation::VerticalKanaRepeatMarkLowerHalf => "vertical kana repeat mark lower half",
            CJKSymbolsandPunctuation::CircledPostalMark => "circled postal mark",
            CJKSymbolsandPunctuation::IdeographicTelegraphLineFeedSeparatorSymbol => "ideographic telegraph line feed separator symbol",
            CJKSymbolsandPunctuation::HangzhouNumeralTen => "hangzhou numeral ten",
            CJKSymbolsandPunctuation::HangzhouNumeralTwenty => "hangzhou numeral twenty",
            CJKSymbolsandPunctuation::HangzhouNumeralThirty => "hangzhou numeral thirty",
            CJKSymbolsandPunctuation::VerticalIdeographicIterationMark => "vertical ideographic iteration mark",
            CJKSymbolsandPunctuation::MasuMark => "masu mark",
            CJKSymbolsandPunctuation::PartAlternationMark => "part alternation mark",
            CJKSymbolsandPunctuation::IdeographicVariationIndicator => "ideographic variation indicator",
        }
    }
}
