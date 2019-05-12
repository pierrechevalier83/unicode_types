/// \u{3040} → \u{309f}
///
/// ぁ あ ぃ い ぅ う ぇ え ぉ お か が き ぎ く ぐ\
/// け げ こ ご さ ざ し じ す ず せ ぜ そ ぞ た だ\
/// ち ぢ っ つ づ て で と ど な に ぬ ね の は ば\
/// ぱ ひ び ぴ ふ ぶ ぷ へ べ ぺ ほ ぼ ぽ ま み む\
/// め も ゃ や ゅ ゆ ょ よ ら り る れ ろ ゎ わ ゐ\
/// ゑ を ん ゔ ゕ ゖ ゙ ゚ ゛ ゜ ゝ ゞ\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{3041}: 'ぁ'
    pub const LETTER_SMALL_A: char = 'ぁ';
    /// \u{3042}: 'あ'
    pub const LETTER_A: char = 'あ';
    /// \u{3043}: 'ぃ'
    pub const LETTER_SMALL_I: char = 'ぃ';
    /// \u{3044}: 'い'
    pub const LETTER_I: char = 'い';
    /// \u{3045}: 'ぅ'
    pub const LETTER_SMALL_U: char = 'ぅ';
    /// \u{3046}: 'う'
    pub const LETTER_U: char = 'う';
    /// \u{3047}: 'ぇ'
    pub const LETTER_SMALL_E: char = 'ぇ';
    /// \u{3048}: 'え'
    pub const LETTER_E: char = 'え';
    /// \u{3049}: 'ぉ'
    pub const LETTER_SMALL_O: char = 'ぉ';
    /// \u{304a}: 'お'
    pub const LETTER_O: char = 'お';
    /// \u{304b}: 'か'
    pub const LETTER_KA: char = 'か';
    /// \u{304c}: 'が'
    pub const LETTER_GA: char = 'が';
    /// \u{304d}: 'き'
    pub const LETTER_KI: char = 'き';
    /// \u{304e}: 'ぎ'
    pub const LETTER_GI: char = 'ぎ';
    /// \u{304f}: 'く'
    pub const LETTER_KU: char = 'く';
    /// \u{3050}: 'ぐ'
    pub const LETTER_GU: char = 'ぐ';
    /// \u{3051}: 'け'
    pub const LETTER_KE: char = 'け';
    /// \u{3052}: 'げ'
    pub const LETTER_GE: char = 'げ';
    /// \u{3053}: 'こ'
    pub const LETTER_KO: char = 'こ';
    /// \u{3054}: 'ご'
    pub const LETTER_GO: char = 'ご';
    /// \u{3055}: 'さ'
    pub const LETTER_SA: char = 'さ';
    /// \u{3056}: 'ざ'
    pub const LETTER_ZA: char = 'ざ';
    /// \u{3057}: 'し'
    pub const LETTER_SI: char = 'し';
    /// \u{3058}: 'じ'
    pub const LETTER_ZI: char = 'じ';
    /// \u{3059}: 'す'
    pub const LETTER_SU: char = 'す';
    /// \u{305a}: 'ず'
    pub const LETTER_ZU: char = 'ず';
    /// \u{305b}: 'せ'
    pub const LETTER_SE: char = 'せ';
    /// \u{305c}: 'ぜ'
    pub const LETTER_ZE: char = 'ぜ';
    /// \u{305d}: 'そ'
    pub const LETTER_SO: char = 'そ';
    /// \u{305e}: 'ぞ'
    pub const LETTER_ZO: char = 'ぞ';
    /// \u{305f}: 'た'
    pub const LETTER_TA: char = 'た';
    /// \u{3060}: 'だ'
    pub const LETTER_DA: char = 'だ';
    /// \u{3061}: 'ち'
    pub const LETTER_TI: char = 'ち';
    /// \u{3062}: 'ぢ'
    pub const LETTER_DI: char = 'ぢ';
    /// \u{3063}: 'っ'
    pub const LETTER_SMALL_TU: char = 'っ';
    /// \u{3064}: 'つ'
    pub const LETTER_TU: char = 'つ';
    /// \u{3065}: 'づ'
    pub const LETTER_DU: char = 'づ';
    /// \u{3066}: 'て'
    pub const LETTER_TE: char = 'て';
    /// \u{3067}: 'で'
    pub const LETTER_DE: char = 'で';
    /// \u{3068}: 'と'
    pub const LETTER_TO: char = 'と';
    /// \u{3069}: 'ど'
    pub const LETTER_DO: char = 'ど';
    /// \u{306a}: 'な'
    pub const LETTER_NA: char = 'な';
    /// \u{306b}: 'に'
    pub const LETTER_NI: char = 'に';
    /// \u{306c}: 'ぬ'
    pub const LETTER_NU: char = 'ぬ';
    /// \u{306d}: 'ね'
    pub const LETTER_NE: char = 'ね';
    /// \u{306e}: 'の'
    pub const LETTER_NO: char = 'の';
    /// \u{306f}: 'は'
    pub const LETTER_HA: char = 'は';
    /// \u{3070}: 'ば'
    pub const LETTER_BA: char = 'ば';
    /// \u{3071}: 'ぱ'
    pub const LETTER_PA: char = 'ぱ';
    /// \u{3072}: 'ひ'
    pub const LETTER_HI: char = 'ひ';
    /// \u{3073}: 'び'
    pub const LETTER_BI: char = 'び';
    /// \u{3074}: 'ぴ'
    pub const LETTER_PI: char = 'ぴ';
    /// \u{3075}: 'ふ'
    pub const LETTER_HU: char = 'ふ';
    /// \u{3076}: 'ぶ'
    pub const LETTER_BU: char = 'ぶ';
    /// \u{3077}: 'ぷ'
    pub const LETTER_PU: char = 'ぷ';
    /// \u{3078}: 'へ'
    pub const LETTER_HE: char = 'へ';
    /// \u{3079}: 'べ'
    pub const LETTER_BE: char = 'べ';
    /// \u{307a}: 'ぺ'
    pub const LETTER_PE: char = 'ぺ';
    /// \u{307b}: 'ほ'
    pub const LETTER_HO: char = 'ほ';
    /// \u{307c}: 'ぼ'
    pub const LETTER_BO: char = 'ぼ';
    /// \u{307d}: 'ぽ'
    pub const LETTER_PO: char = 'ぽ';
    /// \u{307e}: 'ま'
    pub const LETTER_MA: char = 'ま';
    /// \u{307f}: 'み'
    pub const LETTER_MI: char = 'み';
    /// \u{3080}: 'む'
    pub const LETTER_MU: char = 'む';
    /// \u{3081}: 'め'
    pub const LETTER_ME: char = 'め';
    /// \u{3082}: 'も'
    pub const LETTER_MO: char = 'も';
    /// \u{3083}: 'ゃ'
    pub const LETTER_SMALL_YA: char = 'ゃ';
    /// \u{3084}: 'や'
    pub const LETTER_YA: char = 'や';
    /// \u{3085}: 'ゅ'
    pub const LETTER_SMALL_YU: char = 'ゅ';
    /// \u{3086}: 'ゆ'
    pub const LETTER_YU: char = 'ゆ';
    /// \u{3087}: 'ょ'
    pub const LETTER_SMALL_YO: char = 'ょ';
    /// \u{3088}: 'よ'
    pub const LETTER_YO: char = 'よ';
    /// \u{3089}: 'ら'
    pub const LETTER_RA: char = 'ら';
    /// \u{308a}: 'り'
    pub const LETTER_RI: char = 'り';
    /// \u{308b}: 'る'
    pub const LETTER_RU: char = 'る';
    /// \u{308c}: 'れ'
    pub const LETTER_RE: char = 'れ';
    /// \u{308d}: 'ろ'
    pub const LETTER_RO: char = 'ろ';
    /// \u{308e}: 'ゎ'
    pub const LETTER_SMALL_WA: char = 'ゎ';
    /// \u{308f}: 'わ'
    pub const LETTER_WA: char = 'わ';
    /// \u{3090}: 'ゐ'
    pub const LETTER_WI: char = 'ゐ';
    /// \u{3091}: 'ゑ'
    pub const LETTER_WE: char = 'ゑ';
    /// \u{3092}: 'を'
    pub const LETTER_WO: char = 'を';
    /// \u{3093}: 'ん'
    pub const LETTER_N: char = 'ん';
    /// \u{3094}: 'ゔ'
    pub const LETTER_VU: char = 'ゔ';
    /// \u{3095}: 'ゕ'
    pub const LETTER_SMALL_KA: char = 'ゕ';
    /// \u{3096}: 'ゖ'
    pub const LETTER_SMALL_KE: char = 'ゖ';
    /// \u{3099}: '゙'
    pub const COMBINING_KATAKANA_DASH_VOICED_SOUND_MARK: char = '゙';
    /// \u{309a}: '゚'
    pub const COMBINING_KATAKANA_DASH_SEMI_DASH_VOICED_SOUND_MARK: char = '゚';
    /// \u{309b}: '゛'
    pub const KATAKANA_DASH_VOICED_SOUND_MARK: char = '゛';
    /// \u{309c}: '゜'
    pub const KATAKANA_DASH_SEMI_DASH_VOICED_SOUND_MARK: char = '゜';
    /// \u{309d}: 'ゝ'
    pub const ITERATION_MARK: char = 'ゝ';
    /// \u{309e}: 'ゞ'
    pub const VOICED_ITERATION_MARK: char = 'ゞ';
}

/// An enum to represent all characters in the Hiragana block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Hiragana {
    /// \u{3041}: 'ぁ'
    LetterSmallA,
    /// \u{3042}: 'あ'
    LetterA,
    /// \u{3043}: 'ぃ'
    LetterSmallI,
    /// \u{3044}: 'い'
    LetterI,
    /// \u{3045}: 'ぅ'
    LetterSmallU,
    /// \u{3046}: 'う'
    LetterU,
    /// \u{3047}: 'ぇ'
    LetterSmallE,
    /// \u{3048}: 'え'
    LetterE,
    /// \u{3049}: 'ぉ'
    LetterSmallO,
    /// \u{304a}: 'お'
    LetterO,
    /// \u{304b}: 'か'
    LetterKa,
    /// \u{304c}: 'が'
    LetterGa,
    /// \u{304d}: 'き'
    LetterKi,
    /// \u{304e}: 'ぎ'
    LetterGi,
    /// \u{304f}: 'く'
    LetterKu,
    /// \u{3050}: 'ぐ'
    LetterGu,
    /// \u{3051}: 'け'
    LetterKe,
    /// \u{3052}: 'げ'
    LetterGe,
    /// \u{3053}: 'こ'
    LetterKo,
    /// \u{3054}: 'ご'
    LetterGo,
    /// \u{3055}: 'さ'
    LetterSa,
    /// \u{3056}: 'ざ'
    LetterZa,
    /// \u{3057}: 'し'
    LetterSi,
    /// \u{3058}: 'じ'
    LetterZi,
    /// \u{3059}: 'す'
    LetterSu,
    /// \u{305a}: 'ず'
    LetterZu,
    /// \u{305b}: 'せ'
    LetterSe,
    /// \u{305c}: 'ぜ'
    LetterZe,
    /// \u{305d}: 'そ'
    LetterSo,
    /// \u{305e}: 'ぞ'
    LetterZo,
    /// \u{305f}: 'た'
    LetterTa,
    /// \u{3060}: 'だ'
    LetterDa,
    /// \u{3061}: 'ち'
    LetterTi,
    /// \u{3062}: 'ぢ'
    LetterDi,
    /// \u{3063}: 'っ'
    LetterSmallTu,
    /// \u{3064}: 'つ'
    LetterTu,
    /// \u{3065}: 'づ'
    LetterDu,
    /// \u{3066}: 'て'
    LetterTe,
    /// \u{3067}: 'で'
    LetterDe,
    /// \u{3068}: 'と'
    LetterTo,
    /// \u{3069}: 'ど'
    LetterDo,
    /// \u{306a}: 'な'
    LetterNa,
    /// \u{306b}: 'に'
    LetterNi,
    /// \u{306c}: 'ぬ'
    LetterNu,
    /// \u{306d}: 'ね'
    LetterNe,
    /// \u{306e}: 'の'
    LetterNo,
    /// \u{306f}: 'は'
    LetterHa,
    /// \u{3070}: 'ば'
    LetterBa,
    /// \u{3071}: 'ぱ'
    LetterPa,
    /// \u{3072}: 'ひ'
    LetterHi,
    /// \u{3073}: 'び'
    LetterBi,
    /// \u{3074}: 'ぴ'
    LetterPi,
    /// \u{3075}: 'ふ'
    LetterHu,
    /// \u{3076}: 'ぶ'
    LetterBu,
    /// \u{3077}: 'ぷ'
    LetterPu,
    /// \u{3078}: 'へ'
    LetterHe,
    /// \u{3079}: 'べ'
    LetterBe,
    /// \u{307a}: 'ぺ'
    LetterPe,
    /// \u{307b}: 'ほ'
    LetterHo,
    /// \u{307c}: 'ぼ'
    LetterBo,
    /// \u{307d}: 'ぽ'
    LetterPo,
    /// \u{307e}: 'ま'
    LetterMa,
    /// \u{307f}: 'み'
    LetterMi,
    /// \u{3080}: 'む'
    LetterMu,
    /// \u{3081}: 'め'
    LetterMe,
    /// \u{3082}: 'も'
    LetterMo,
    /// \u{3083}: 'ゃ'
    LetterSmallYa,
    /// \u{3084}: 'や'
    LetterYa,
    /// \u{3085}: 'ゅ'
    LetterSmallYu,
    /// \u{3086}: 'ゆ'
    LetterYu,
    /// \u{3087}: 'ょ'
    LetterSmallYo,
    /// \u{3088}: 'よ'
    LetterYo,
    /// \u{3089}: 'ら'
    LetterRa,
    /// \u{308a}: 'り'
    LetterRi,
    /// \u{308b}: 'る'
    LetterRu,
    /// \u{308c}: 'れ'
    LetterRe,
    /// \u{308d}: 'ろ'
    LetterRo,
    /// \u{308e}: 'ゎ'
    LetterSmallWa,
    /// \u{308f}: 'わ'
    LetterWa,
    /// \u{3090}: 'ゐ'
    LetterWi,
    /// \u{3091}: 'ゑ'
    LetterWe,
    /// \u{3092}: 'を'
    LetterWo,
    /// \u{3093}: 'ん'
    LetterN,
    /// \u{3094}: 'ゔ'
    LetterVu,
    /// \u{3095}: 'ゕ'
    LetterSmallKa,
    /// \u{3096}: 'ゖ'
    LetterSmallKe,
    /// \u{3099}: '゙'
    CombiningKatakanaDashVoicedSoundMark,
    /// \u{309a}: '゚'
    CombiningKatakanaDashSemiDashVoicedSoundMark,
    /// \u{309b}: '゛'
    KatakanaDashVoicedSoundMark,
    /// \u{309c}: '゜'
    KatakanaDashSemiDashVoicedSoundMark,
    /// \u{309d}: 'ゝ'
    IterationMark,
    /// \u{309e}: 'ゞ'
    VoicedIterationMark,
}

impl Into<char> for Hiragana {
    fn into(self) -> char {
        use constants::*;
        match self {
            Hiragana::LetterSmallA => LETTER_SMALL_A,
            Hiragana::LetterA => LETTER_A,
            Hiragana::LetterSmallI => LETTER_SMALL_I,
            Hiragana::LetterI => LETTER_I,
            Hiragana::LetterSmallU => LETTER_SMALL_U,
            Hiragana::LetterU => LETTER_U,
            Hiragana::LetterSmallE => LETTER_SMALL_E,
            Hiragana::LetterE => LETTER_E,
            Hiragana::LetterSmallO => LETTER_SMALL_O,
            Hiragana::LetterO => LETTER_O,
            Hiragana::LetterKa => LETTER_KA,
            Hiragana::LetterGa => LETTER_GA,
            Hiragana::LetterKi => LETTER_KI,
            Hiragana::LetterGi => LETTER_GI,
            Hiragana::LetterKu => LETTER_KU,
            Hiragana::LetterGu => LETTER_GU,
            Hiragana::LetterKe => LETTER_KE,
            Hiragana::LetterGe => LETTER_GE,
            Hiragana::LetterKo => LETTER_KO,
            Hiragana::LetterGo => LETTER_GO,
            Hiragana::LetterSa => LETTER_SA,
            Hiragana::LetterZa => LETTER_ZA,
            Hiragana::LetterSi => LETTER_SI,
            Hiragana::LetterZi => LETTER_ZI,
            Hiragana::LetterSu => LETTER_SU,
            Hiragana::LetterZu => LETTER_ZU,
            Hiragana::LetterSe => LETTER_SE,
            Hiragana::LetterZe => LETTER_ZE,
            Hiragana::LetterSo => LETTER_SO,
            Hiragana::LetterZo => LETTER_ZO,
            Hiragana::LetterTa => LETTER_TA,
            Hiragana::LetterDa => LETTER_DA,
            Hiragana::LetterTi => LETTER_TI,
            Hiragana::LetterDi => LETTER_DI,
            Hiragana::LetterSmallTu => LETTER_SMALL_TU,
            Hiragana::LetterTu => LETTER_TU,
            Hiragana::LetterDu => LETTER_DU,
            Hiragana::LetterTe => LETTER_TE,
            Hiragana::LetterDe => LETTER_DE,
            Hiragana::LetterTo => LETTER_TO,
            Hiragana::LetterDo => LETTER_DO,
            Hiragana::LetterNa => LETTER_NA,
            Hiragana::LetterNi => LETTER_NI,
            Hiragana::LetterNu => LETTER_NU,
            Hiragana::LetterNe => LETTER_NE,
            Hiragana::LetterNo => LETTER_NO,
            Hiragana::LetterHa => LETTER_HA,
            Hiragana::LetterBa => LETTER_BA,
            Hiragana::LetterPa => LETTER_PA,
            Hiragana::LetterHi => LETTER_HI,
            Hiragana::LetterBi => LETTER_BI,
            Hiragana::LetterPi => LETTER_PI,
            Hiragana::LetterHu => LETTER_HU,
            Hiragana::LetterBu => LETTER_BU,
            Hiragana::LetterPu => LETTER_PU,
            Hiragana::LetterHe => LETTER_HE,
            Hiragana::LetterBe => LETTER_BE,
            Hiragana::LetterPe => LETTER_PE,
            Hiragana::LetterHo => LETTER_HO,
            Hiragana::LetterBo => LETTER_BO,
            Hiragana::LetterPo => LETTER_PO,
            Hiragana::LetterMa => LETTER_MA,
            Hiragana::LetterMi => LETTER_MI,
            Hiragana::LetterMu => LETTER_MU,
            Hiragana::LetterMe => LETTER_ME,
            Hiragana::LetterMo => LETTER_MO,
            Hiragana::LetterSmallYa => LETTER_SMALL_YA,
            Hiragana::LetterYa => LETTER_YA,
            Hiragana::LetterSmallYu => LETTER_SMALL_YU,
            Hiragana::LetterYu => LETTER_YU,
            Hiragana::LetterSmallYo => LETTER_SMALL_YO,
            Hiragana::LetterYo => LETTER_YO,
            Hiragana::LetterRa => LETTER_RA,
            Hiragana::LetterRi => LETTER_RI,
            Hiragana::LetterRu => LETTER_RU,
            Hiragana::LetterRe => LETTER_RE,
            Hiragana::LetterRo => LETTER_RO,
            Hiragana::LetterSmallWa => LETTER_SMALL_WA,
            Hiragana::LetterWa => LETTER_WA,
            Hiragana::LetterWi => LETTER_WI,
            Hiragana::LetterWe => LETTER_WE,
            Hiragana::LetterWo => LETTER_WO,
            Hiragana::LetterN => LETTER_N,
            Hiragana::LetterVu => LETTER_VU,
            Hiragana::LetterSmallKa => LETTER_SMALL_KA,
            Hiragana::LetterSmallKe => LETTER_SMALL_KE,
            Hiragana::CombiningKatakanaDashVoicedSoundMark => COMBINING_KATAKANA_DASH_VOICED_SOUND_MARK,
            Hiragana::CombiningKatakanaDashSemiDashVoicedSoundMark => COMBINING_KATAKANA_DASH_SEMI_DASH_VOICED_SOUND_MARK,
            Hiragana::KatakanaDashVoicedSoundMark => KATAKANA_DASH_VOICED_SOUND_MARK,
            Hiragana::KatakanaDashSemiDashVoicedSoundMark => KATAKANA_DASH_SEMI_DASH_VOICED_SOUND_MARK,
            Hiragana::IterationMark => ITERATION_MARK,
            Hiragana::VoicedIterationMark => VOICED_ITERATION_MARK,
        }
    }
}

impl std::convert::TryFrom<char> for Hiragana {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_SMALL_A => Ok(Hiragana::LetterSmallA),
            LETTER_A => Ok(Hiragana::LetterA),
            LETTER_SMALL_I => Ok(Hiragana::LetterSmallI),
            LETTER_I => Ok(Hiragana::LetterI),
            LETTER_SMALL_U => Ok(Hiragana::LetterSmallU),
            LETTER_U => Ok(Hiragana::LetterU),
            LETTER_SMALL_E => Ok(Hiragana::LetterSmallE),
            LETTER_E => Ok(Hiragana::LetterE),
            LETTER_SMALL_O => Ok(Hiragana::LetterSmallO),
            LETTER_O => Ok(Hiragana::LetterO),
            LETTER_KA => Ok(Hiragana::LetterKa),
            LETTER_GA => Ok(Hiragana::LetterGa),
            LETTER_KI => Ok(Hiragana::LetterKi),
            LETTER_GI => Ok(Hiragana::LetterGi),
            LETTER_KU => Ok(Hiragana::LetterKu),
            LETTER_GU => Ok(Hiragana::LetterGu),
            LETTER_KE => Ok(Hiragana::LetterKe),
            LETTER_GE => Ok(Hiragana::LetterGe),
            LETTER_KO => Ok(Hiragana::LetterKo),
            LETTER_GO => Ok(Hiragana::LetterGo),
            LETTER_SA => Ok(Hiragana::LetterSa),
            LETTER_ZA => Ok(Hiragana::LetterZa),
            LETTER_SI => Ok(Hiragana::LetterSi),
            LETTER_ZI => Ok(Hiragana::LetterZi),
            LETTER_SU => Ok(Hiragana::LetterSu),
            LETTER_ZU => Ok(Hiragana::LetterZu),
            LETTER_SE => Ok(Hiragana::LetterSe),
            LETTER_ZE => Ok(Hiragana::LetterZe),
            LETTER_SO => Ok(Hiragana::LetterSo),
            LETTER_ZO => Ok(Hiragana::LetterZo),
            LETTER_TA => Ok(Hiragana::LetterTa),
            LETTER_DA => Ok(Hiragana::LetterDa),
            LETTER_TI => Ok(Hiragana::LetterTi),
            LETTER_DI => Ok(Hiragana::LetterDi),
            LETTER_SMALL_TU => Ok(Hiragana::LetterSmallTu),
            LETTER_TU => Ok(Hiragana::LetterTu),
            LETTER_DU => Ok(Hiragana::LetterDu),
            LETTER_TE => Ok(Hiragana::LetterTe),
            LETTER_DE => Ok(Hiragana::LetterDe),
            LETTER_TO => Ok(Hiragana::LetterTo),
            LETTER_DO => Ok(Hiragana::LetterDo),
            LETTER_NA => Ok(Hiragana::LetterNa),
            LETTER_NI => Ok(Hiragana::LetterNi),
            LETTER_NU => Ok(Hiragana::LetterNu),
            LETTER_NE => Ok(Hiragana::LetterNe),
            LETTER_NO => Ok(Hiragana::LetterNo),
            LETTER_HA => Ok(Hiragana::LetterHa),
            LETTER_BA => Ok(Hiragana::LetterBa),
            LETTER_PA => Ok(Hiragana::LetterPa),
            LETTER_HI => Ok(Hiragana::LetterHi),
            LETTER_BI => Ok(Hiragana::LetterBi),
            LETTER_PI => Ok(Hiragana::LetterPi),
            LETTER_HU => Ok(Hiragana::LetterHu),
            LETTER_BU => Ok(Hiragana::LetterBu),
            LETTER_PU => Ok(Hiragana::LetterPu),
            LETTER_HE => Ok(Hiragana::LetterHe),
            LETTER_BE => Ok(Hiragana::LetterBe),
            LETTER_PE => Ok(Hiragana::LetterPe),
            LETTER_HO => Ok(Hiragana::LetterHo),
            LETTER_BO => Ok(Hiragana::LetterBo),
            LETTER_PO => Ok(Hiragana::LetterPo),
            LETTER_MA => Ok(Hiragana::LetterMa),
            LETTER_MI => Ok(Hiragana::LetterMi),
            LETTER_MU => Ok(Hiragana::LetterMu),
            LETTER_ME => Ok(Hiragana::LetterMe),
            LETTER_MO => Ok(Hiragana::LetterMo),
            LETTER_SMALL_YA => Ok(Hiragana::LetterSmallYa),
            LETTER_YA => Ok(Hiragana::LetterYa),
            LETTER_SMALL_YU => Ok(Hiragana::LetterSmallYu),
            LETTER_YU => Ok(Hiragana::LetterYu),
            LETTER_SMALL_YO => Ok(Hiragana::LetterSmallYo),
            LETTER_YO => Ok(Hiragana::LetterYo),
            LETTER_RA => Ok(Hiragana::LetterRa),
            LETTER_RI => Ok(Hiragana::LetterRi),
            LETTER_RU => Ok(Hiragana::LetterRu),
            LETTER_RE => Ok(Hiragana::LetterRe),
            LETTER_RO => Ok(Hiragana::LetterRo),
            LETTER_SMALL_WA => Ok(Hiragana::LetterSmallWa),
            LETTER_WA => Ok(Hiragana::LetterWa),
            LETTER_WI => Ok(Hiragana::LetterWi),
            LETTER_WE => Ok(Hiragana::LetterWe),
            LETTER_WO => Ok(Hiragana::LetterWo),
            LETTER_N => Ok(Hiragana::LetterN),
            LETTER_VU => Ok(Hiragana::LetterVu),
            LETTER_SMALL_KA => Ok(Hiragana::LetterSmallKa),
            LETTER_SMALL_KE => Ok(Hiragana::LetterSmallKe),
            COMBINING_KATAKANA_DASH_VOICED_SOUND_MARK => Ok(Hiragana::CombiningKatakanaDashVoicedSoundMark),
            COMBINING_KATAKANA_DASH_SEMI_DASH_VOICED_SOUND_MARK => Ok(Hiragana::CombiningKatakanaDashSemiDashVoicedSoundMark),
            KATAKANA_DASH_VOICED_SOUND_MARK => Ok(Hiragana::KatakanaDashVoicedSoundMark),
            KATAKANA_DASH_SEMI_DASH_VOICED_SOUND_MARK => Ok(Hiragana::KatakanaDashSemiDashVoicedSoundMark),
            ITERATION_MARK => Ok(Hiragana::IterationMark),
            VOICED_ITERATION_MARK => Ok(Hiragana::VoicedIterationMark),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Hiragana {
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

impl std::convert::TryFrom<u32> for Hiragana {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Hiragana {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Hiragana {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Hiragana::LetterSmallA
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Hiragana::LetterSmallA => "hiragana letter small a",
            Hiragana::LetterA => "hiragana letter a",
            Hiragana::LetterSmallI => "hiragana letter small i",
            Hiragana::LetterI => "hiragana letter i",
            Hiragana::LetterSmallU => "hiragana letter small u",
            Hiragana::LetterU => "hiragana letter u",
            Hiragana::LetterSmallE => "hiragana letter small e",
            Hiragana::LetterE => "hiragana letter e",
            Hiragana::LetterSmallO => "hiragana letter small o",
            Hiragana::LetterO => "hiragana letter o",
            Hiragana::LetterKa => "hiragana letter ka",
            Hiragana::LetterGa => "hiragana letter ga",
            Hiragana::LetterKi => "hiragana letter ki",
            Hiragana::LetterGi => "hiragana letter gi",
            Hiragana::LetterKu => "hiragana letter ku",
            Hiragana::LetterGu => "hiragana letter gu",
            Hiragana::LetterKe => "hiragana letter ke",
            Hiragana::LetterGe => "hiragana letter ge",
            Hiragana::LetterKo => "hiragana letter ko",
            Hiragana::LetterGo => "hiragana letter go",
            Hiragana::LetterSa => "hiragana letter sa",
            Hiragana::LetterZa => "hiragana letter za",
            Hiragana::LetterSi => "hiragana letter si",
            Hiragana::LetterZi => "hiragana letter zi",
            Hiragana::LetterSu => "hiragana letter su",
            Hiragana::LetterZu => "hiragana letter zu",
            Hiragana::LetterSe => "hiragana letter se",
            Hiragana::LetterZe => "hiragana letter ze",
            Hiragana::LetterSo => "hiragana letter so",
            Hiragana::LetterZo => "hiragana letter zo",
            Hiragana::LetterTa => "hiragana letter ta",
            Hiragana::LetterDa => "hiragana letter da",
            Hiragana::LetterTi => "hiragana letter ti",
            Hiragana::LetterDi => "hiragana letter di",
            Hiragana::LetterSmallTu => "hiragana letter small tu",
            Hiragana::LetterTu => "hiragana letter tu",
            Hiragana::LetterDu => "hiragana letter du",
            Hiragana::LetterTe => "hiragana letter te",
            Hiragana::LetterDe => "hiragana letter de",
            Hiragana::LetterTo => "hiragana letter to",
            Hiragana::LetterDo => "hiragana letter do",
            Hiragana::LetterNa => "hiragana letter na",
            Hiragana::LetterNi => "hiragana letter ni",
            Hiragana::LetterNu => "hiragana letter nu",
            Hiragana::LetterNe => "hiragana letter ne",
            Hiragana::LetterNo => "hiragana letter no",
            Hiragana::LetterHa => "hiragana letter ha",
            Hiragana::LetterBa => "hiragana letter ba",
            Hiragana::LetterPa => "hiragana letter pa",
            Hiragana::LetterHi => "hiragana letter hi",
            Hiragana::LetterBi => "hiragana letter bi",
            Hiragana::LetterPi => "hiragana letter pi",
            Hiragana::LetterHu => "hiragana letter hu",
            Hiragana::LetterBu => "hiragana letter bu",
            Hiragana::LetterPu => "hiragana letter pu",
            Hiragana::LetterHe => "hiragana letter he",
            Hiragana::LetterBe => "hiragana letter be",
            Hiragana::LetterPe => "hiragana letter pe",
            Hiragana::LetterHo => "hiragana letter ho",
            Hiragana::LetterBo => "hiragana letter bo",
            Hiragana::LetterPo => "hiragana letter po",
            Hiragana::LetterMa => "hiragana letter ma",
            Hiragana::LetterMi => "hiragana letter mi",
            Hiragana::LetterMu => "hiragana letter mu",
            Hiragana::LetterMe => "hiragana letter me",
            Hiragana::LetterMo => "hiragana letter mo",
            Hiragana::LetterSmallYa => "hiragana letter small ya",
            Hiragana::LetterYa => "hiragana letter ya",
            Hiragana::LetterSmallYu => "hiragana letter small yu",
            Hiragana::LetterYu => "hiragana letter yu",
            Hiragana::LetterSmallYo => "hiragana letter small yo",
            Hiragana::LetterYo => "hiragana letter yo",
            Hiragana::LetterRa => "hiragana letter ra",
            Hiragana::LetterRi => "hiragana letter ri",
            Hiragana::LetterRu => "hiragana letter ru",
            Hiragana::LetterRe => "hiragana letter re",
            Hiragana::LetterRo => "hiragana letter ro",
            Hiragana::LetterSmallWa => "hiragana letter small wa",
            Hiragana::LetterWa => "hiragana letter wa",
            Hiragana::LetterWi => "hiragana letter wi",
            Hiragana::LetterWe => "hiragana letter we",
            Hiragana::LetterWo => "hiragana letter wo",
            Hiragana::LetterN => "hiragana letter n",
            Hiragana::LetterVu => "hiragana letter vu",
            Hiragana::LetterSmallKa => "hiragana letter small ka",
            Hiragana::LetterSmallKe => "hiragana letter small ke",
            Hiragana::CombiningKatakanaDashVoicedSoundMark => "combining katakana-hiragana voiced sound mark",
            Hiragana::CombiningKatakanaDashSemiDashVoicedSoundMark => "combining katakana-hiragana semi-voiced sound mark",
            Hiragana::KatakanaDashVoicedSoundMark => "katakana-hiragana voiced sound mark",
            Hiragana::KatakanaDashSemiDashVoicedSoundMark => "katakana-hiragana semi-voiced sound mark",
            Hiragana::IterationMark => "hiragana iteration mark",
            Hiragana::VoicedIterationMark => "hiragana voiced iteration mark",
        }
    }
}
