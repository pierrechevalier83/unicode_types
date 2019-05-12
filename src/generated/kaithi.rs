
/// An enum to represent all characters in the Kaithi block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Kaithi {
    /// \u{11080}: '𑂀'
    SignCandrabindu,
    /// \u{11081}: '𑂁'
    SignAnusvara,
    /// \u{11082}: '𑂂'
    SignVisarga,
    /// \u{11083}: '𑂃'
    LetterA,
    /// \u{11084}: '𑂄'
    LetterAa,
    /// \u{11085}: '𑂅'
    LetterI,
    /// \u{11086}: '𑂆'
    LetterIi,
    /// \u{11087}: '𑂇'
    LetterU,
    /// \u{11088}: '𑂈'
    LetterUu,
    /// \u{11089}: '𑂉'
    LetterE,
    /// \u{1108a}: '𑂊'
    LetterAi,
    /// \u{1108b}: '𑂋'
    LetterO,
    /// \u{1108c}: '𑂌'
    LetterAu,
    /// \u{1108d}: '𑂍'
    LetterKa,
    /// \u{1108e}: '𑂎'
    LetterKha,
    /// \u{1108f}: '𑂏'
    LetterGa,
    /// \u{11090}: '𑂐'
    LetterGha,
    /// \u{11091}: '𑂑'
    LetterNga,
    /// \u{11092}: '𑂒'
    LetterCa,
    /// \u{11093}: '𑂓'
    LetterCha,
    /// \u{11094}: '𑂔'
    LetterJa,
    /// \u{11095}: '𑂕'
    LetterJha,
    /// \u{11096}: '𑂖'
    LetterNya,
    /// \u{11097}: '𑂗'
    LetterTta,
    /// \u{11098}: '𑂘'
    LetterTtha,
    /// \u{11099}: '𑂙'
    LetterDda,
    /// \u{1109a}: '𑂚'
    LetterDddha,
    /// \u{1109b}: '𑂛'
    LetterDdha,
    /// \u{1109c}: '𑂜'
    LetterRha,
    /// \u{1109d}: '𑂝'
    LetterNna,
    /// \u{1109e}: '𑂞'
    LetterTa,
    /// \u{1109f}: '𑂟'
    LetterTha,
    /// \u{110a0}: '𑂠'
    LetterDa,
    /// \u{110a1}: '𑂡'
    LetterDha,
    /// \u{110a2}: '𑂢'
    LetterNa,
    /// \u{110a3}: '𑂣'
    LetterPa,
    /// \u{110a4}: '𑂤'
    LetterPha,
    /// \u{110a5}: '𑂥'
    LetterBa,
    /// \u{110a6}: '𑂦'
    LetterBha,
    /// \u{110a7}: '𑂧'
    LetterMa,
    /// \u{110a8}: '𑂨'
    LetterYa,
    /// \u{110a9}: '𑂩'
    LetterRa,
    /// \u{110aa}: '𑂪'
    LetterLa,
    /// \u{110ab}: '𑂫'
    LetterVa,
    /// \u{110ac}: '𑂬'
    LetterSha,
    /// \u{110ad}: '𑂭'
    LetterSsa,
    /// \u{110ae}: '𑂮'
    LetterSa,
    /// \u{110af}: '𑂯'
    LetterHa,
    /// \u{110b0}: '𑂰'
    VowelSignAa,
    /// \u{110b1}: '𑂱'
    VowelSignI,
    /// \u{110b2}: '𑂲'
    VowelSignIi,
    /// \u{110b3}: '𑂳'
    VowelSignU,
    /// \u{110b4}: '𑂴'
    VowelSignUu,
    /// \u{110b5}: '𑂵'
    VowelSignE,
    /// \u{110b6}: '𑂶'
    VowelSignAi,
    /// \u{110b7}: '𑂷'
    VowelSignO,
    /// \u{110b8}: '𑂸'
    VowelSignAu,
    /// \u{110b9}: '𑂹'
    SignVirama,
    /// \u{110ba}: '𑂺'
    SignNukta,
    /// \u{110bb}: '𑂻'
    AbbreviationSign,
    /// \u{110bc}: '𑂼'
    EnumerationSign,
    /// \u{110bd}: '𑂽'
    NumberSign,
    /// \u{110be}: '𑂾'
    SectionMark,
    /// \u{110bf}: '𑂿'
    DoubleSectionMark,
    /// \u{110c0}: '𑃀'
    Danda,
    /// \u{110c1}: '𑃁'
    DoubleDanda,
    /// \u{110cd}: '𑃍'
    NumberSignAbove,
}

impl Into<char> for Kaithi {
    fn into(self) -> char {
        match self {
            Kaithi::SignCandrabindu => '𑂀',
            Kaithi::SignAnusvara => '𑂁',
            Kaithi::SignVisarga => '𑂂',
            Kaithi::LetterA => '𑂃',
            Kaithi::LetterAa => '𑂄',
            Kaithi::LetterI => '𑂅',
            Kaithi::LetterIi => '𑂆',
            Kaithi::LetterU => '𑂇',
            Kaithi::LetterUu => '𑂈',
            Kaithi::LetterE => '𑂉',
            Kaithi::LetterAi => '𑂊',
            Kaithi::LetterO => '𑂋',
            Kaithi::LetterAu => '𑂌',
            Kaithi::LetterKa => '𑂍',
            Kaithi::LetterKha => '𑂎',
            Kaithi::LetterGa => '𑂏',
            Kaithi::LetterGha => '𑂐',
            Kaithi::LetterNga => '𑂑',
            Kaithi::LetterCa => '𑂒',
            Kaithi::LetterCha => '𑂓',
            Kaithi::LetterJa => '𑂔',
            Kaithi::LetterJha => '𑂕',
            Kaithi::LetterNya => '𑂖',
            Kaithi::LetterTta => '𑂗',
            Kaithi::LetterTtha => '𑂘',
            Kaithi::LetterDda => '𑂙',
            Kaithi::LetterDddha => '𑂚',
            Kaithi::LetterDdha => '𑂛',
            Kaithi::LetterRha => '𑂜',
            Kaithi::LetterNna => '𑂝',
            Kaithi::LetterTa => '𑂞',
            Kaithi::LetterTha => '𑂟',
            Kaithi::LetterDa => '𑂠',
            Kaithi::LetterDha => '𑂡',
            Kaithi::LetterNa => '𑂢',
            Kaithi::LetterPa => '𑂣',
            Kaithi::LetterPha => '𑂤',
            Kaithi::LetterBa => '𑂥',
            Kaithi::LetterBha => '𑂦',
            Kaithi::LetterMa => '𑂧',
            Kaithi::LetterYa => '𑂨',
            Kaithi::LetterRa => '𑂩',
            Kaithi::LetterLa => '𑂪',
            Kaithi::LetterVa => '𑂫',
            Kaithi::LetterSha => '𑂬',
            Kaithi::LetterSsa => '𑂭',
            Kaithi::LetterSa => '𑂮',
            Kaithi::LetterHa => '𑂯',
            Kaithi::VowelSignAa => '𑂰',
            Kaithi::VowelSignI => '𑂱',
            Kaithi::VowelSignIi => '𑂲',
            Kaithi::VowelSignU => '𑂳',
            Kaithi::VowelSignUu => '𑂴',
            Kaithi::VowelSignE => '𑂵',
            Kaithi::VowelSignAi => '𑂶',
            Kaithi::VowelSignO => '𑂷',
            Kaithi::VowelSignAu => '𑂸',
            Kaithi::SignVirama => '𑂹',
            Kaithi::SignNukta => '𑂺',
            Kaithi::AbbreviationSign => '𑂻',
            Kaithi::EnumerationSign => '𑂼',
            Kaithi::NumberSign => '𑂽',
            Kaithi::SectionMark => '𑂾',
            Kaithi::DoubleSectionMark => '𑂿',
            Kaithi::Danda => '𑃀',
            Kaithi::DoubleDanda => '𑃁',
            Kaithi::NumberSignAbove => '𑃍',
        }
    }
}

impl std::convert::TryFrom<char> for Kaithi {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𑂀' => Ok(Kaithi::SignCandrabindu),
            '𑂁' => Ok(Kaithi::SignAnusvara),
            '𑂂' => Ok(Kaithi::SignVisarga),
            '𑂃' => Ok(Kaithi::LetterA),
            '𑂄' => Ok(Kaithi::LetterAa),
            '𑂅' => Ok(Kaithi::LetterI),
            '𑂆' => Ok(Kaithi::LetterIi),
            '𑂇' => Ok(Kaithi::LetterU),
            '𑂈' => Ok(Kaithi::LetterUu),
            '𑂉' => Ok(Kaithi::LetterE),
            '𑂊' => Ok(Kaithi::LetterAi),
            '𑂋' => Ok(Kaithi::LetterO),
            '𑂌' => Ok(Kaithi::LetterAu),
            '𑂍' => Ok(Kaithi::LetterKa),
            '𑂎' => Ok(Kaithi::LetterKha),
            '𑂏' => Ok(Kaithi::LetterGa),
            '𑂐' => Ok(Kaithi::LetterGha),
            '𑂑' => Ok(Kaithi::LetterNga),
            '𑂒' => Ok(Kaithi::LetterCa),
            '𑂓' => Ok(Kaithi::LetterCha),
            '𑂔' => Ok(Kaithi::LetterJa),
            '𑂕' => Ok(Kaithi::LetterJha),
            '𑂖' => Ok(Kaithi::LetterNya),
            '𑂗' => Ok(Kaithi::LetterTta),
            '𑂘' => Ok(Kaithi::LetterTtha),
            '𑂙' => Ok(Kaithi::LetterDda),
            '𑂚' => Ok(Kaithi::LetterDddha),
            '𑂛' => Ok(Kaithi::LetterDdha),
            '𑂜' => Ok(Kaithi::LetterRha),
            '𑂝' => Ok(Kaithi::LetterNna),
            '𑂞' => Ok(Kaithi::LetterTa),
            '𑂟' => Ok(Kaithi::LetterTha),
            '𑂠' => Ok(Kaithi::LetterDa),
            '𑂡' => Ok(Kaithi::LetterDha),
            '𑂢' => Ok(Kaithi::LetterNa),
            '𑂣' => Ok(Kaithi::LetterPa),
            '𑂤' => Ok(Kaithi::LetterPha),
            '𑂥' => Ok(Kaithi::LetterBa),
            '𑂦' => Ok(Kaithi::LetterBha),
            '𑂧' => Ok(Kaithi::LetterMa),
            '𑂨' => Ok(Kaithi::LetterYa),
            '𑂩' => Ok(Kaithi::LetterRa),
            '𑂪' => Ok(Kaithi::LetterLa),
            '𑂫' => Ok(Kaithi::LetterVa),
            '𑂬' => Ok(Kaithi::LetterSha),
            '𑂭' => Ok(Kaithi::LetterSsa),
            '𑂮' => Ok(Kaithi::LetterSa),
            '𑂯' => Ok(Kaithi::LetterHa),
            '𑂰' => Ok(Kaithi::VowelSignAa),
            '𑂱' => Ok(Kaithi::VowelSignI),
            '𑂲' => Ok(Kaithi::VowelSignIi),
            '𑂳' => Ok(Kaithi::VowelSignU),
            '𑂴' => Ok(Kaithi::VowelSignUu),
            '𑂵' => Ok(Kaithi::VowelSignE),
            '𑂶' => Ok(Kaithi::VowelSignAi),
            '𑂷' => Ok(Kaithi::VowelSignO),
            '𑂸' => Ok(Kaithi::VowelSignAu),
            '𑂹' => Ok(Kaithi::SignVirama),
            '𑂺' => Ok(Kaithi::SignNukta),
            '𑂻' => Ok(Kaithi::AbbreviationSign),
            '𑂼' => Ok(Kaithi::EnumerationSign),
            '𑂽' => Ok(Kaithi::NumberSign),
            '𑂾' => Ok(Kaithi::SectionMark),
            '𑂿' => Ok(Kaithi::DoubleSectionMark),
            '𑃀' => Ok(Kaithi::Danda),
            '𑃁' => Ok(Kaithi::DoubleDanda),
            '𑃍' => Ok(Kaithi::NumberSignAbove),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Kaithi {
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

impl std::convert::TryFrom<u32> for Kaithi {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Kaithi {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Kaithi {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Kaithi::SignCandrabindu
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Kaithi{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
