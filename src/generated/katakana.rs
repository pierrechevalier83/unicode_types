
/// An enum to represent all characters in the Katakana block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Katakana {
    /// \u{30a0}: '゠'
    DashHiraganaDoubleHyphen,
    /// \u{30a1}: 'ァ'
    LetterSmallA,
    /// \u{30a2}: 'ア'
    LetterA,
    /// \u{30a3}: 'ィ'
    LetterSmallI,
    /// \u{30a4}: 'イ'
    LetterI,
    /// \u{30a5}: 'ゥ'
    LetterSmallU,
    /// \u{30a6}: 'ウ'
    LetterU,
    /// \u{30a7}: 'ェ'
    LetterSmallE,
    /// \u{30a8}: 'エ'
    LetterE,
    /// \u{30a9}: 'ォ'
    LetterSmallO,
    /// \u{30aa}: 'オ'
    LetterO,
    /// \u{30ab}: 'カ'
    LetterKa,
    /// \u{30ac}: 'ガ'
    LetterGa,
    /// \u{30ad}: 'キ'
    LetterKi,
    /// \u{30ae}: 'ギ'
    LetterGi,
    /// \u{30af}: 'ク'
    LetterKu,
    /// \u{30b0}: 'グ'
    LetterGu,
    /// \u{30b1}: 'ケ'
    LetterKe,
    /// \u{30b2}: 'ゲ'
    LetterGe,
    /// \u{30b3}: 'コ'
    LetterKo,
    /// \u{30b4}: 'ゴ'
    LetterGo,
    /// \u{30b5}: 'サ'
    LetterSa,
    /// \u{30b6}: 'ザ'
    LetterZa,
    /// \u{30b7}: 'シ'
    LetterSi,
    /// \u{30b8}: 'ジ'
    LetterZi,
    /// \u{30b9}: 'ス'
    LetterSu,
    /// \u{30ba}: 'ズ'
    LetterZu,
    /// \u{30bb}: 'セ'
    LetterSe,
    /// \u{30bc}: 'ゼ'
    LetterZe,
    /// \u{30bd}: 'ソ'
    LetterSo,
    /// \u{30be}: 'ゾ'
    LetterZo,
    /// \u{30bf}: 'タ'
    LetterTa,
    /// \u{30c0}: 'ダ'
    LetterDa,
    /// \u{30c1}: 'チ'
    LetterTi,
    /// \u{30c2}: 'ヂ'
    LetterDi,
    /// \u{30c3}: 'ッ'
    LetterSmallTu,
    /// \u{30c4}: 'ツ'
    LetterTu,
    /// \u{30c5}: 'ヅ'
    LetterDu,
    /// \u{30c6}: 'テ'
    LetterTe,
    /// \u{30c7}: 'デ'
    LetterDe,
    /// \u{30c8}: 'ト'
    LetterTo,
    /// \u{30c9}: 'ド'
    LetterDo,
    /// \u{30ca}: 'ナ'
    LetterNa,
    /// \u{30cb}: 'ニ'
    LetterNi,
    /// \u{30cc}: 'ヌ'
    LetterNu,
    /// \u{30cd}: 'ネ'
    LetterNe,
    /// \u{30ce}: 'ノ'
    LetterNo,
    /// \u{30cf}: 'ハ'
    LetterHa,
    /// \u{30d0}: 'バ'
    LetterBa,
    /// \u{30d1}: 'パ'
    LetterPa,
    /// \u{30d2}: 'ヒ'
    LetterHi,
    /// \u{30d3}: 'ビ'
    LetterBi,
    /// \u{30d4}: 'ピ'
    LetterPi,
    /// \u{30d5}: 'フ'
    LetterHu,
    /// \u{30d6}: 'ブ'
    LetterBu,
    /// \u{30d7}: 'プ'
    LetterPu,
    /// \u{30d8}: 'ヘ'
    LetterHe,
    /// \u{30d9}: 'ベ'
    LetterBe,
    /// \u{30da}: 'ペ'
    LetterPe,
    /// \u{30db}: 'ホ'
    LetterHo,
    /// \u{30dc}: 'ボ'
    LetterBo,
    /// \u{30dd}: 'ポ'
    LetterPo,
    /// \u{30de}: 'マ'
    LetterMa,
    /// \u{30df}: 'ミ'
    LetterMi,
    /// \u{30e0}: 'ム'
    LetterMu,
    /// \u{30e1}: 'メ'
    LetterMe,
    /// \u{30e2}: 'モ'
    LetterMo,
    /// \u{30e3}: 'ャ'
    LetterSmallYa,
    /// \u{30e4}: 'ヤ'
    LetterYa,
    /// \u{30e5}: 'ュ'
    LetterSmallYu,
    /// \u{30e6}: 'ユ'
    LetterYu,
    /// \u{30e7}: 'ョ'
    LetterSmallYo,
    /// \u{30e8}: 'ヨ'
    LetterYo,
    /// \u{30e9}: 'ラ'
    LetterRa,
    /// \u{30ea}: 'リ'
    LetterRi,
    /// \u{30eb}: 'ル'
    LetterRu,
    /// \u{30ec}: 'レ'
    LetterRe,
    /// \u{30ed}: 'ロ'
    LetterRo,
    /// \u{30ee}: 'ヮ'
    LetterSmallWa,
    /// \u{30ef}: 'ワ'
    LetterWa,
    /// \u{30f0}: 'ヰ'
    LetterWi,
    /// \u{30f1}: 'ヱ'
    LetterWe,
    /// \u{30f2}: 'ヲ'
    LetterWo,
    /// \u{30f3}: 'ン'
    LetterN,
    /// \u{30f4}: 'ヴ'
    LetterVu,
    /// \u{30f5}: 'ヵ'
    LetterSmallKa,
    /// \u{30f6}: 'ヶ'
    LetterSmallKe,
    /// \u{30f7}: 'ヷ'
    LetterVa,
    /// \u{30f8}: 'ヸ'
    LetterVi,
    /// \u{30f9}: 'ヹ'
    LetterVe,
    /// \u{30fa}: 'ヺ'
    LetterVo,
    /// \u{30fb}: '・'
    MiddleDot,
    /// \u{30fc}: 'ー'
    DashHiraganaProlongedSoundMark,
    /// \u{30fd}: 'ヽ'
    IterationMark,
    /// \u{30fe}: 'ヾ'
    VoicedIterationMark,
}

impl Into<char> for Katakana {
    fn into(self) -> char {
        match self {
            Katakana::DashHiraganaDoubleHyphen => '゠',
            Katakana::LetterSmallA => 'ァ',
            Katakana::LetterA => 'ア',
            Katakana::LetterSmallI => 'ィ',
            Katakana::LetterI => 'イ',
            Katakana::LetterSmallU => 'ゥ',
            Katakana::LetterU => 'ウ',
            Katakana::LetterSmallE => 'ェ',
            Katakana::LetterE => 'エ',
            Katakana::LetterSmallO => 'ォ',
            Katakana::LetterO => 'オ',
            Katakana::LetterKa => 'カ',
            Katakana::LetterGa => 'ガ',
            Katakana::LetterKi => 'キ',
            Katakana::LetterGi => 'ギ',
            Katakana::LetterKu => 'ク',
            Katakana::LetterGu => 'グ',
            Katakana::LetterKe => 'ケ',
            Katakana::LetterGe => 'ゲ',
            Katakana::LetterKo => 'コ',
            Katakana::LetterGo => 'ゴ',
            Katakana::LetterSa => 'サ',
            Katakana::LetterZa => 'ザ',
            Katakana::LetterSi => 'シ',
            Katakana::LetterZi => 'ジ',
            Katakana::LetterSu => 'ス',
            Katakana::LetterZu => 'ズ',
            Katakana::LetterSe => 'セ',
            Katakana::LetterZe => 'ゼ',
            Katakana::LetterSo => 'ソ',
            Katakana::LetterZo => 'ゾ',
            Katakana::LetterTa => 'タ',
            Katakana::LetterDa => 'ダ',
            Katakana::LetterTi => 'チ',
            Katakana::LetterDi => 'ヂ',
            Katakana::LetterSmallTu => 'ッ',
            Katakana::LetterTu => 'ツ',
            Katakana::LetterDu => 'ヅ',
            Katakana::LetterTe => 'テ',
            Katakana::LetterDe => 'デ',
            Katakana::LetterTo => 'ト',
            Katakana::LetterDo => 'ド',
            Katakana::LetterNa => 'ナ',
            Katakana::LetterNi => 'ニ',
            Katakana::LetterNu => 'ヌ',
            Katakana::LetterNe => 'ネ',
            Katakana::LetterNo => 'ノ',
            Katakana::LetterHa => 'ハ',
            Katakana::LetterBa => 'バ',
            Katakana::LetterPa => 'パ',
            Katakana::LetterHi => 'ヒ',
            Katakana::LetterBi => 'ビ',
            Katakana::LetterPi => 'ピ',
            Katakana::LetterHu => 'フ',
            Katakana::LetterBu => 'ブ',
            Katakana::LetterPu => 'プ',
            Katakana::LetterHe => 'ヘ',
            Katakana::LetterBe => 'ベ',
            Katakana::LetterPe => 'ペ',
            Katakana::LetterHo => 'ホ',
            Katakana::LetterBo => 'ボ',
            Katakana::LetterPo => 'ポ',
            Katakana::LetterMa => 'マ',
            Katakana::LetterMi => 'ミ',
            Katakana::LetterMu => 'ム',
            Katakana::LetterMe => 'メ',
            Katakana::LetterMo => 'モ',
            Katakana::LetterSmallYa => 'ャ',
            Katakana::LetterYa => 'ヤ',
            Katakana::LetterSmallYu => 'ュ',
            Katakana::LetterYu => 'ユ',
            Katakana::LetterSmallYo => 'ョ',
            Katakana::LetterYo => 'ヨ',
            Katakana::LetterRa => 'ラ',
            Katakana::LetterRi => 'リ',
            Katakana::LetterRu => 'ル',
            Katakana::LetterRe => 'レ',
            Katakana::LetterRo => 'ロ',
            Katakana::LetterSmallWa => 'ヮ',
            Katakana::LetterWa => 'ワ',
            Katakana::LetterWi => 'ヰ',
            Katakana::LetterWe => 'ヱ',
            Katakana::LetterWo => 'ヲ',
            Katakana::LetterN => 'ン',
            Katakana::LetterVu => 'ヴ',
            Katakana::LetterSmallKa => 'ヵ',
            Katakana::LetterSmallKe => 'ヶ',
            Katakana::LetterVa => 'ヷ',
            Katakana::LetterVi => 'ヸ',
            Katakana::LetterVe => 'ヹ',
            Katakana::LetterVo => 'ヺ',
            Katakana::MiddleDot => '・',
            Katakana::DashHiraganaProlongedSoundMark => 'ー',
            Katakana::IterationMark => 'ヽ',
            Katakana::VoicedIterationMark => 'ヾ',
        }
    }
}

impl std::convert::TryFrom<char> for Katakana {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '゠' => Ok(Katakana::DashHiraganaDoubleHyphen),
            'ァ' => Ok(Katakana::LetterSmallA),
            'ア' => Ok(Katakana::LetterA),
            'ィ' => Ok(Katakana::LetterSmallI),
            'イ' => Ok(Katakana::LetterI),
            'ゥ' => Ok(Katakana::LetterSmallU),
            'ウ' => Ok(Katakana::LetterU),
            'ェ' => Ok(Katakana::LetterSmallE),
            'エ' => Ok(Katakana::LetterE),
            'ォ' => Ok(Katakana::LetterSmallO),
            'オ' => Ok(Katakana::LetterO),
            'カ' => Ok(Katakana::LetterKa),
            'ガ' => Ok(Katakana::LetterGa),
            'キ' => Ok(Katakana::LetterKi),
            'ギ' => Ok(Katakana::LetterGi),
            'ク' => Ok(Katakana::LetterKu),
            'グ' => Ok(Katakana::LetterGu),
            'ケ' => Ok(Katakana::LetterKe),
            'ゲ' => Ok(Katakana::LetterGe),
            'コ' => Ok(Katakana::LetterKo),
            'ゴ' => Ok(Katakana::LetterGo),
            'サ' => Ok(Katakana::LetterSa),
            'ザ' => Ok(Katakana::LetterZa),
            'シ' => Ok(Katakana::LetterSi),
            'ジ' => Ok(Katakana::LetterZi),
            'ス' => Ok(Katakana::LetterSu),
            'ズ' => Ok(Katakana::LetterZu),
            'セ' => Ok(Katakana::LetterSe),
            'ゼ' => Ok(Katakana::LetterZe),
            'ソ' => Ok(Katakana::LetterSo),
            'ゾ' => Ok(Katakana::LetterZo),
            'タ' => Ok(Katakana::LetterTa),
            'ダ' => Ok(Katakana::LetterDa),
            'チ' => Ok(Katakana::LetterTi),
            'ヂ' => Ok(Katakana::LetterDi),
            'ッ' => Ok(Katakana::LetterSmallTu),
            'ツ' => Ok(Katakana::LetterTu),
            'ヅ' => Ok(Katakana::LetterDu),
            'テ' => Ok(Katakana::LetterTe),
            'デ' => Ok(Katakana::LetterDe),
            'ト' => Ok(Katakana::LetterTo),
            'ド' => Ok(Katakana::LetterDo),
            'ナ' => Ok(Katakana::LetterNa),
            'ニ' => Ok(Katakana::LetterNi),
            'ヌ' => Ok(Katakana::LetterNu),
            'ネ' => Ok(Katakana::LetterNe),
            'ノ' => Ok(Katakana::LetterNo),
            'ハ' => Ok(Katakana::LetterHa),
            'バ' => Ok(Katakana::LetterBa),
            'パ' => Ok(Katakana::LetterPa),
            'ヒ' => Ok(Katakana::LetterHi),
            'ビ' => Ok(Katakana::LetterBi),
            'ピ' => Ok(Katakana::LetterPi),
            'フ' => Ok(Katakana::LetterHu),
            'ブ' => Ok(Katakana::LetterBu),
            'プ' => Ok(Katakana::LetterPu),
            'ヘ' => Ok(Katakana::LetterHe),
            'ベ' => Ok(Katakana::LetterBe),
            'ペ' => Ok(Katakana::LetterPe),
            'ホ' => Ok(Katakana::LetterHo),
            'ボ' => Ok(Katakana::LetterBo),
            'ポ' => Ok(Katakana::LetterPo),
            'マ' => Ok(Katakana::LetterMa),
            'ミ' => Ok(Katakana::LetterMi),
            'ム' => Ok(Katakana::LetterMu),
            'メ' => Ok(Katakana::LetterMe),
            'モ' => Ok(Katakana::LetterMo),
            'ャ' => Ok(Katakana::LetterSmallYa),
            'ヤ' => Ok(Katakana::LetterYa),
            'ュ' => Ok(Katakana::LetterSmallYu),
            'ユ' => Ok(Katakana::LetterYu),
            'ョ' => Ok(Katakana::LetterSmallYo),
            'ヨ' => Ok(Katakana::LetterYo),
            'ラ' => Ok(Katakana::LetterRa),
            'リ' => Ok(Katakana::LetterRi),
            'ル' => Ok(Katakana::LetterRu),
            'レ' => Ok(Katakana::LetterRe),
            'ロ' => Ok(Katakana::LetterRo),
            'ヮ' => Ok(Katakana::LetterSmallWa),
            'ワ' => Ok(Katakana::LetterWa),
            'ヰ' => Ok(Katakana::LetterWi),
            'ヱ' => Ok(Katakana::LetterWe),
            'ヲ' => Ok(Katakana::LetterWo),
            'ン' => Ok(Katakana::LetterN),
            'ヴ' => Ok(Katakana::LetterVu),
            'ヵ' => Ok(Katakana::LetterSmallKa),
            'ヶ' => Ok(Katakana::LetterSmallKe),
            'ヷ' => Ok(Katakana::LetterVa),
            'ヸ' => Ok(Katakana::LetterVi),
            'ヹ' => Ok(Katakana::LetterVe),
            'ヺ' => Ok(Katakana::LetterVo),
            '・' => Ok(Katakana::MiddleDot),
            'ー' => Ok(Katakana::DashHiraganaProlongedSoundMark),
            'ヽ' => Ok(Katakana::IterationMark),
            'ヾ' => Ok(Katakana::VoicedIterationMark),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Katakana {
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

impl std::convert::TryFrom<u32> for Katakana {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Katakana {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Katakana {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Katakana::DashHiraganaDoubleHyphen
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Katakana::DashHiraganaDoubleHyphen => "katakana-hiragana double hyphen",
            Katakana::LetterSmallA => "katakana letter small a",
            Katakana::LetterA => "katakana letter a",
            Katakana::LetterSmallI => "katakana letter small i",
            Katakana::LetterI => "katakana letter i",
            Katakana::LetterSmallU => "katakana letter small u",
            Katakana::LetterU => "katakana letter u",
            Katakana::LetterSmallE => "katakana letter small e",
            Katakana::LetterE => "katakana letter e",
            Katakana::LetterSmallO => "katakana letter small o",
            Katakana::LetterO => "katakana letter o",
            Katakana::LetterKa => "katakana letter ka",
            Katakana::LetterGa => "katakana letter ga",
            Katakana::LetterKi => "katakana letter ki",
            Katakana::LetterGi => "katakana letter gi",
            Katakana::LetterKu => "katakana letter ku",
            Katakana::LetterGu => "katakana letter gu",
            Katakana::LetterKe => "katakana letter ke",
            Katakana::LetterGe => "katakana letter ge",
            Katakana::LetterKo => "katakana letter ko",
            Katakana::LetterGo => "katakana letter go",
            Katakana::LetterSa => "katakana letter sa",
            Katakana::LetterZa => "katakana letter za",
            Katakana::LetterSi => "katakana letter si",
            Katakana::LetterZi => "katakana letter zi",
            Katakana::LetterSu => "katakana letter su",
            Katakana::LetterZu => "katakana letter zu",
            Katakana::LetterSe => "katakana letter se",
            Katakana::LetterZe => "katakana letter ze",
            Katakana::LetterSo => "katakana letter so",
            Katakana::LetterZo => "katakana letter zo",
            Katakana::LetterTa => "katakana letter ta",
            Katakana::LetterDa => "katakana letter da",
            Katakana::LetterTi => "katakana letter ti",
            Katakana::LetterDi => "katakana letter di",
            Katakana::LetterSmallTu => "katakana letter small tu",
            Katakana::LetterTu => "katakana letter tu",
            Katakana::LetterDu => "katakana letter du",
            Katakana::LetterTe => "katakana letter te",
            Katakana::LetterDe => "katakana letter de",
            Katakana::LetterTo => "katakana letter to",
            Katakana::LetterDo => "katakana letter do",
            Katakana::LetterNa => "katakana letter na",
            Katakana::LetterNi => "katakana letter ni",
            Katakana::LetterNu => "katakana letter nu",
            Katakana::LetterNe => "katakana letter ne",
            Katakana::LetterNo => "katakana letter no",
            Katakana::LetterHa => "katakana letter ha",
            Katakana::LetterBa => "katakana letter ba",
            Katakana::LetterPa => "katakana letter pa",
            Katakana::LetterHi => "katakana letter hi",
            Katakana::LetterBi => "katakana letter bi",
            Katakana::LetterPi => "katakana letter pi",
            Katakana::LetterHu => "katakana letter hu",
            Katakana::LetterBu => "katakana letter bu",
            Katakana::LetterPu => "katakana letter pu",
            Katakana::LetterHe => "katakana letter he",
            Katakana::LetterBe => "katakana letter be",
            Katakana::LetterPe => "katakana letter pe",
            Katakana::LetterHo => "katakana letter ho",
            Katakana::LetterBo => "katakana letter bo",
            Katakana::LetterPo => "katakana letter po",
            Katakana::LetterMa => "katakana letter ma",
            Katakana::LetterMi => "katakana letter mi",
            Katakana::LetterMu => "katakana letter mu",
            Katakana::LetterMe => "katakana letter me",
            Katakana::LetterMo => "katakana letter mo",
            Katakana::LetterSmallYa => "katakana letter small ya",
            Katakana::LetterYa => "katakana letter ya",
            Katakana::LetterSmallYu => "katakana letter small yu",
            Katakana::LetterYu => "katakana letter yu",
            Katakana::LetterSmallYo => "katakana letter small yo",
            Katakana::LetterYo => "katakana letter yo",
            Katakana::LetterRa => "katakana letter ra",
            Katakana::LetterRi => "katakana letter ri",
            Katakana::LetterRu => "katakana letter ru",
            Katakana::LetterRe => "katakana letter re",
            Katakana::LetterRo => "katakana letter ro",
            Katakana::LetterSmallWa => "katakana letter small wa",
            Katakana::LetterWa => "katakana letter wa",
            Katakana::LetterWi => "katakana letter wi",
            Katakana::LetterWe => "katakana letter we",
            Katakana::LetterWo => "katakana letter wo",
            Katakana::LetterN => "katakana letter n",
            Katakana::LetterVu => "katakana letter vu",
            Katakana::LetterSmallKa => "katakana letter small ka",
            Katakana::LetterSmallKe => "katakana letter small ke",
            Katakana::LetterVa => "katakana letter va",
            Katakana::LetterVi => "katakana letter vi",
            Katakana::LetterVe => "katakana letter ve",
            Katakana::LetterVo => "katakana letter vo",
            Katakana::MiddleDot => "katakana middle dot",
            Katakana::DashHiraganaProlongedSoundMark => "katakana-hiragana prolonged sound mark",
            Katakana::IterationMark => "katakana iteration mark",
            Katakana::VoicedIterationMark => "katakana voiced iteration mark",
        }
    }
}
