
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
        match self {
            Hiragana::LetterSmallA => 'ぁ',
            Hiragana::LetterA => 'あ',
            Hiragana::LetterSmallI => 'ぃ',
            Hiragana::LetterI => 'い',
            Hiragana::LetterSmallU => 'ぅ',
            Hiragana::LetterU => 'う',
            Hiragana::LetterSmallE => 'ぇ',
            Hiragana::LetterE => 'え',
            Hiragana::LetterSmallO => 'ぉ',
            Hiragana::LetterO => 'お',
            Hiragana::LetterKa => 'か',
            Hiragana::LetterGa => 'が',
            Hiragana::LetterKi => 'き',
            Hiragana::LetterGi => 'ぎ',
            Hiragana::LetterKu => 'く',
            Hiragana::LetterGu => 'ぐ',
            Hiragana::LetterKe => 'け',
            Hiragana::LetterGe => 'げ',
            Hiragana::LetterKo => 'こ',
            Hiragana::LetterGo => 'ご',
            Hiragana::LetterSa => 'さ',
            Hiragana::LetterZa => 'ざ',
            Hiragana::LetterSi => 'し',
            Hiragana::LetterZi => 'じ',
            Hiragana::LetterSu => 'す',
            Hiragana::LetterZu => 'ず',
            Hiragana::LetterSe => 'せ',
            Hiragana::LetterZe => 'ぜ',
            Hiragana::LetterSo => 'そ',
            Hiragana::LetterZo => 'ぞ',
            Hiragana::LetterTa => 'た',
            Hiragana::LetterDa => 'だ',
            Hiragana::LetterTi => 'ち',
            Hiragana::LetterDi => 'ぢ',
            Hiragana::LetterSmallTu => 'っ',
            Hiragana::LetterTu => 'つ',
            Hiragana::LetterDu => 'づ',
            Hiragana::LetterTe => 'て',
            Hiragana::LetterDe => 'で',
            Hiragana::LetterTo => 'と',
            Hiragana::LetterDo => 'ど',
            Hiragana::LetterNa => 'な',
            Hiragana::LetterNi => 'に',
            Hiragana::LetterNu => 'ぬ',
            Hiragana::LetterNe => 'ね',
            Hiragana::LetterNo => 'の',
            Hiragana::LetterHa => 'は',
            Hiragana::LetterBa => 'ば',
            Hiragana::LetterPa => 'ぱ',
            Hiragana::LetterHi => 'ひ',
            Hiragana::LetterBi => 'び',
            Hiragana::LetterPi => 'ぴ',
            Hiragana::LetterHu => 'ふ',
            Hiragana::LetterBu => 'ぶ',
            Hiragana::LetterPu => 'ぷ',
            Hiragana::LetterHe => 'へ',
            Hiragana::LetterBe => 'べ',
            Hiragana::LetterPe => 'ぺ',
            Hiragana::LetterHo => 'ほ',
            Hiragana::LetterBo => 'ぼ',
            Hiragana::LetterPo => 'ぽ',
            Hiragana::LetterMa => 'ま',
            Hiragana::LetterMi => 'み',
            Hiragana::LetterMu => 'む',
            Hiragana::LetterMe => 'め',
            Hiragana::LetterMo => 'も',
            Hiragana::LetterSmallYa => 'ゃ',
            Hiragana::LetterYa => 'や',
            Hiragana::LetterSmallYu => 'ゅ',
            Hiragana::LetterYu => 'ゆ',
            Hiragana::LetterSmallYo => 'ょ',
            Hiragana::LetterYo => 'よ',
            Hiragana::LetterRa => 'ら',
            Hiragana::LetterRi => 'り',
            Hiragana::LetterRu => 'る',
            Hiragana::LetterRe => 'れ',
            Hiragana::LetterRo => 'ろ',
            Hiragana::LetterSmallWa => 'ゎ',
            Hiragana::LetterWa => 'わ',
            Hiragana::LetterWi => 'ゐ',
            Hiragana::LetterWe => 'ゑ',
            Hiragana::LetterWo => 'を',
            Hiragana::LetterN => 'ん',
            Hiragana::LetterVu => 'ゔ',
            Hiragana::LetterSmallKa => 'ゕ',
            Hiragana::LetterSmallKe => 'ゖ',
            Hiragana::CombiningKatakanaDashVoicedSoundMark => '゙',
            Hiragana::CombiningKatakanaDashSemiDashVoicedSoundMark => '゚',
            Hiragana::KatakanaDashVoicedSoundMark => '゛',
            Hiragana::KatakanaDashSemiDashVoicedSoundMark => '゜',
            Hiragana::IterationMark => 'ゝ',
            Hiragana::VoicedIterationMark => 'ゞ',
        }
    }
}

impl std::convert::TryFrom<char> for Hiragana {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ぁ' => Ok(Hiragana::LetterSmallA),
            'あ' => Ok(Hiragana::LetterA),
            'ぃ' => Ok(Hiragana::LetterSmallI),
            'い' => Ok(Hiragana::LetterI),
            'ぅ' => Ok(Hiragana::LetterSmallU),
            'う' => Ok(Hiragana::LetterU),
            'ぇ' => Ok(Hiragana::LetterSmallE),
            'え' => Ok(Hiragana::LetterE),
            'ぉ' => Ok(Hiragana::LetterSmallO),
            'お' => Ok(Hiragana::LetterO),
            'か' => Ok(Hiragana::LetterKa),
            'が' => Ok(Hiragana::LetterGa),
            'き' => Ok(Hiragana::LetterKi),
            'ぎ' => Ok(Hiragana::LetterGi),
            'く' => Ok(Hiragana::LetterKu),
            'ぐ' => Ok(Hiragana::LetterGu),
            'け' => Ok(Hiragana::LetterKe),
            'げ' => Ok(Hiragana::LetterGe),
            'こ' => Ok(Hiragana::LetterKo),
            'ご' => Ok(Hiragana::LetterGo),
            'さ' => Ok(Hiragana::LetterSa),
            'ざ' => Ok(Hiragana::LetterZa),
            'し' => Ok(Hiragana::LetterSi),
            'じ' => Ok(Hiragana::LetterZi),
            'す' => Ok(Hiragana::LetterSu),
            'ず' => Ok(Hiragana::LetterZu),
            'せ' => Ok(Hiragana::LetterSe),
            'ぜ' => Ok(Hiragana::LetterZe),
            'そ' => Ok(Hiragana::LetterSo),
            'ぞ' => Ok(Hiragana::LetterZo),
            'た' => Ok(Hiragana::LetterTa),
            'だ' => Ok(Hiragana::LetterDa),
            'ち' => Ok(Hiragana::LetterTi),
            'ぢ' => Ok(Hiragana::LetterDi),
            'っ' => Ok(Hiragana::LetterSmallTu),
            'つ' => Ok(Hiragana::LetterTu),
            'づ' => Ok(Hiragana::LetterDu),
            'て' => Ok(Hiragana::LetterTe),
            'で' => Ok(Hiragana::LetterDe),
            'と' => Ok(Hiragana::LetterTo),
            'ど' => Ok(Hiragana::LetterDo),
            'な' => Ok(Hiragana::LetterNa),
            'に' => Ok(Hiragana::LetterNi),
            'ぬ' => Ok(Hiragana::LetterNu),
            'ね' => Ok(Hiragana::LetterNe),
            'の' => Ok(Hiragana::LetterNo),
            'は' => Ok(Hiragana::LetterHa),
            'ば' => Ok(Hiragana::LetterBa),
            'ぱ' => Ok(Hiragana::LetterPa),
            'ひ' => Ok(Hiragana::LetterHi),
            'び' => Ok(Hiragana::LetterBi),
            'ぴ' => Ok(Hiragana::LetterPi),
            'ふ' => Ok(Hiragana::LetterHu),
            'ぶ' => Ok(Hiragana::LetterBu),
            'ぷ' => Ok(Hiragana::LetterPu),
            'へ' => Ok(Hiragana::LetterHe),
            'べ' => Ok(Hiragana::LetterBe),
            'ぺ' => Ok(Hiragana::LetterPe),
            'ほ' => Ok(Hiragana::LetterHo),
            'ぼ' => Ok(Hiragana::LetterBo),
            'ぽ' => Ok(Hiragana::LetterPo),
            'ま' => Ok(Hiragana::LetterMa),
            'み' => Ok(Hiragana::LetterMi),
            'む' => Ok(Hiragana::LetterMu),
            'め' => Ok(Hiragana::LetterMe),
            'も' => Ok(Hiragana::LetterMo),
            'ゃ' => Ok(Hiragana::LetterSmallYa),
            'や' => Ok(Hiragana::LetterYa),
            'ゅ' => Ok(Hiragana::LetterSmallYu),
            'ゆ' => Ok(Hiragana::LetterYu),
            'ょ' => Ok(Hiragana::LetterSmallYo),
            'よ' => Ok(Hiragana::LetterYo),
            'ら' => Ok(Hiragana::LetterRa),
            'り' => Ok(Hiragana::LetterRi),
            'る' => Ok(Hiragana::LetterRu),
            'れ' => Ok(Hiragana::LetterRe),
            'ろ' => Ok(Hiragana::LetterRo),
            'ゎ' => Ok(Hiragana::LetterSmallWa),
            'わ' => Ok(Hiragana::LetterWa),
            'ゐ' => Ok(Hiragana::LetterWi),
            'ゑ' => Ok(Hiragana::LetterWe),
            'を' => Ok(Hiragana::LetterWo),
            'ん' => Ok(Hiragana::LetterN),
            'ゔ' => Ok(Hiragana::LetterVu),
            'ゕ' => Ok(Hiragana::LetterSmallKa),
            'ゖ' => Ok(Hiragana::LetterSmallKe),
            '゙' => Ok(Hiragana::CombiningKatakanaDashVoicedSoundMark),
            '゚' => Ok(Hiragana::CombiningKatakanaDashSemiDashVoicedSoundMark),
            '゛' => Ok(Hiragana::KatakanaDashVoicedSoundMark),
            '゜' => Ok(Hiragana::KatakanaDashSemiDashVoicedSoundMark),
            'ゝ' => Ok(Hiragana::IterationMark),
            'ゞ' => Ok(Hiragana::VoicedIterationMark),
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

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Hiragana{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
