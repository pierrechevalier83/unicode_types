
/// An enum to represent all characters in the Siddham block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Siddham {
    /// \u{11580}: '𑖀'
    LetterA,
    /// \u{11581}: '𑖁'
    LetterAa,
    /// \u{11582}: '𑖂'
    LetterI,
    /// \u{11583}: '𑖃'
    LetterIi,
    /// \u{11584}: '𑖄'
    LetterU,
    /// \u{11585}: '𑖅'
    LetterUu,
    /// \u{11586}: '𑖆'
    LetterVocalicR,
    /// \u{11587}: '𑖇'
    LetterVocalicRr,
    /// \u{11588}: '𑖈'
    LetterVocalicL,
    /// \u{11589}: '𑖉'
    LetterVocalicLl,
    /// \u{1158a}: '𑖊'
    LetterE,
    /// \u{1158b}: '𑖋'
    LetterAi,
    /// \u{1158c}: '𑖌'
    LetterO,
    /// \u{1158d}: '𑖍'
    LetterAu,
    /// \u{1158e}: '𑖎'
    LetterKa,
    /// \u{1158f}: '𑖏'
    LetterKha,
    /// \u{11590}: '𑖐'
    LetterGa,
    /// \u{11591}: '𑖑'
    LetterGha,
    /// \u{11592}: '𑖒'
    LetterNga,
    /// \u{11593}: '𑖓'
    LetterCa,
    /// \u{11594}: '𑖔'
    LetterCha,
    /// \u{11595}: '𑖕'
    LetterJa,
    /// \u{11596}: '𑖖'
    LetterJha,
    /// \u{11597}: '𑖗'
    LetterNya,
    /// \u{11598}: '𑖘'
    LetterTta,
    /// \u{11599}: '𑖙'
    LetterTtha,
    /// \u{1159a}: '𑖚'
    LetterDda,
    /// \u{1159b}: '𑖛'
    LetterDdha,
    /// \u{1159c}: '𑖜'
    LetterNna,
    /// \u{1159d}: '𑖝'
    LetterTa,
    /// \u{1159e}: '𑖞'
    LetterTha,
    /// \u{1159f}: '𑖟'
    LetterDa,
    /// \u{115a0}: '𑖠'
    LetterDha,
    /// \u{115a1}: '𑖡'
    LetterNa,
    /// \u{115a2}: '𑖢'
    LetterPa,
    /// \u{115a3}: '𑖣'
    LetterPha,
    /// \u{115a4}: '𑖤'
    LetterBa,
    /// \u{115a5}: '𑖥'
    LetterBha,
    /// \u{115a6}: '𑖦'
    LetterMa,
    /// \u{115a7}: '𑖧'
    LetterYa,
    /// \u{115a8}: '𑖨'
    LetterRa,
    /// \u{115a9}: '𑖩'
    LetterLa,
    /// \u{115aa}: '𑖪'
    LetterVa,
    /// \u{115ab}: '𑖫'
    LetterSha,
    /// \u{115ac}: '𑖬'
    LetterSsa,
    /// \u{115ad}: '𑖭'
    LetterSa,
    /// \u{115ae}: '𑖮'
    LetterHa,
    /// \u{115af}: '𑖯'
    VowelSignAa,
    /// \u{115b0}: '𑖰'
    VowelSignI,
    /// \u{115b1}: '𑖱'
    VowelSignIi,
    /// \u{115b2}: '𑖲'
    VowelSignU,
    /// \u{115b3}: '𑖳'
    VowelSignUu,
    /// \u{115b4}: '𑖴'
    VowelSignVocalicR,
    /// \u{115b5}: '𑖵'
    VowelSignVocalicRr,
    /// \u{115b8}: '𑖸'
    VowelSignE,
    /// \u{115b9}: '𑖹'
    VowelSignAi,
    /// \u{115ba}: '𑖺'
    VowelSignO,
    /// \u{115bb}: '𑖻'
    VowelSignAu,
    /// \u{115bc}: '𑖼'
    SignCandrabindu,
    /// \u{115bd}: '𑖽'
    SignAnusvara,
    /// \u{115be}: '𑖾'
    SignVisarga,
    /// \u{115bf}: '𑖿'
    SignVirama,
    /// \u{115c0}: '𑗀'
    SignNukta,
    /// \u{115c1}: '𑗁'
    Sign,
    /// \u{115c2}: '𑗂'
    Danda,
    /// \u{115c3}: '𑗃'
    DoubleDanda,
    /// \u{115c4}: '𑗄'
    SeparatorDot,
    /// \u{115c5}: '𑗅'
    SeparatorBar,
    /// \u{115c6}: '𑗆'
    RepetitionMarkDash1,
    /// \u{115c7}: '𑗇'
    RepetitionMarkDash2,
    /// \u{115c8}: '𑗈'
    RepetitionMarkDash3,
    /// \u{115c9}: '𑗉'
    EndOfTextMark,
    /// \u{115ca}: '𑗊'
    SectionMarkWithTridentAndUDashShapedOrnaments,
    /// \u{115cb}: '𑗋'
    SectionMarkWithTridentAndDottedCrescents,
    /// \u{115cc}: '𑗌'
    SectionMarkWithRaysAndDottedCrescents,
    /// \u{115cd}: '𑗍'
    SectionMarkWithRaysAndDottedDoubleCrescents,
    /// \u{115ce}: '𑗎'
    SectionMarkWithRaysAndDottedTripleCrescents,
    /// \u{115cf}: '𑗏'
    SectionMarkDoubleRing,
    /// \u{115d0}: '𑗐'
    SectionMarkDoubleRingWithRays,
    /// \u{115d1}: '𑗑'
    SectionMarkWithDoubleCrescents,
    /// \u{115d2}: '𑗒'
    SectionMarkWithTripleCrescents,
    /// \u{115d3}: '𑗓'
    SectionMarkWithQuadrupleCrescents,
    /// \u{115d4}: '𑗔'
    SectionMarkWithSeptupleCrescents,
    /// \u{115d5}: '𑗕'
    SectionMarkWithCirclesAndRays,
    /// \u{115d6}: '𑗖'
    SectionMarkWithCirclesAndTwoEnclosures,
    /// \u{115d7}: '𑗗'
    SectionMarkWithCirclesAndFourEnclosures,
    /// \u{115d8}: '𑗘'
    LetterThreeDashCircleAlternateI,
    /// \u{115d9}: '𑗙'
    LetterTwoDashCircleAlternateI,
    /// \u{115da}: '𑗚'
    LetterTwoDashCircleAlternateIi,
    /// \u{115db}: '𑗛'
    LetterAlternateU,
    /// \u{115dc}: '𑗜'
    VowelSignAlternateU,
    /// \u{115dd}: '𑗝'
    VowelSignAlternateUu,
}

impl Into<char> for Siddham {
    fn into(self) -> char {
        match self {
            Siddham::LetterA => '𑖀',
            Siddham::LetterAa => '𑖁',
            Siddham::LetterI => '𑖂',
            Siddham::LetterIi => '𑖃',
            Siddham::LetterU => '𑖄',
            Siddham::LetterUu => '𑖅',
            Siddham::LetterVocalicR => '𑖆',
            Siddham::LetterVocalicRr => '𑖇',
            Siddham::LetterVocalicL => '𑖈',
            Siddham::LetterVocalicLl => '𑖉',
            Siddham::LetterE => '𑖊',
            Siddham::LetterAi => '𑖋',
            Siddham::LetterO => '𑖌',
            Siddham::LetterAu => '𑖍',
            Siddham::LetterKa => '𑖎',
            Siddham::LetterKha => '𑖏',
            Siddham::LetterGa => '𑖐',
            Siddham::LetterGha => '𑖑',
            Siddham::LetterNga => '𑖒',
            Siddham::LetterCa => '𑖓',
            Siddham::LetterCha => '𑖔',
            Siddham::LetterJa => '𑖕',
            Siddham::LetterJha => '𑖖',
            Siddham::LetterNya => '𑖗',
            Siddham::LetterTta => '𑖘',
            Siddham::LetterTtha => '𑖙',
            Siddham::LetterDda => '𑖚',
            Siddham::LetterDdha => '𑖛',
            Siddham::LetterNna => '𑖜',
            Siddham::LetterTa => '𑖝',
            Siddham::LetterTha => '𑖞',
            Siddham::LetterDa => '𑖟',
            Siddham::LetterDha => '𑖠',
            Siddham::LetterNa => '𑖡',
            Siddham::LetterPa => '𑖢',
            Siddham::LetterPha => '𑖣',
            Siddham::LetterBa => '𑖤',
            Siddham::LetterBha => '𑖥',
            Siddham::LetterMa => '𑖦',
            Siddham::LetterYa => '𑖧',
            Siddham::LetterRa => '𑖨',
            Siddham::LetterLa => '𑖩',
            Siddham::LetterVa => '𑖪',
            Siddham::LetterSha => '𑖫',
            Siddham::LetterSsa => '𑖬',
            Siddham::LetterSa => '𑖭',
            Siddham::LetterHa => '𑖮',
            Siddham::VowelSignAa => '𑖯',
            Siddham::VowelSignI => '𑖰',
            Siddham::VowelSignIi => '𑖱',
            Siddham::VowelSignU => '𑖲',
            Siddham::VowelSignUu => '𑖳',
            Siddham::VowelSignVocalicR => '𑖴',
            Siddham::VowelSignVocalicRr => '𑖵',
            Siddham::VowelSignE => '𑖸',
            Siddham::VowelSignAi => '𑖹',
            Siddham::VowelSignO => '𑖺',
            Siddham::VowelSignAu => '𑖻',
            Siddham::SignCandrabindu => '𑖼',
            Siddham::SignAnusvara => '𑖽',
            Siddham::SignVisarga => '𑖾',
            Siddham::SignVirama => '𑖿',
            Siddham::SignNukta => '𑗀',
            Siddham::Sign => '𑗁',
            Siddham::Danda => '𑗂',
            Siddham::DoubleDanda => '𑗃',
            Siddham::SeparatorDot => '𑗄',
            Siddham::SeparatorBar => '𑗅',
            Siddham::RepetitionMarkDash1 => '𑗆',
            Siddham::RepetitionMarkDash2 => '𑗇',
            Siddham::RepetitionMarkDash3 => '𑗈',
            Siddham::EndOfTextMark => '𑗉',
            Siddham::SectionMarkWithTridentAndUDashShapedOrnaments => '𑗊',
            Siddham::SectionMarkWithTridentAndDottedCrescents => '𑗋',
            Siddham::SectionMarkWithRaysAndDottedCrescents => '𑗌',
            Siddham::SectionMarkWithRaysAndDottedDoubleCrescents => '𑗍',
            Siddham::SectionMarkWithRaysAndDottedTripleCrescents => '𑗎',
            Siddham::SectionMarkDoubleRing => '𑗏',
            Siddham::SectionMarkDoubleRingWithRays => '𑗐',
            Siddham::SectionMarkWithDoubleCrescents => '𑗑',
            Siddham::SectionMarkWithTripleCrescents => '𑗒',
            Siddham::SectionMarkWithQuadrupleCrescents => '𑗓',
            Siddham::SectionMarkWithSeptupleCrescents => '𑗔',
            Siddham::SectionMarkWithCirclesAndRays => '𑗕',
            Siddham::SectionMarkWithCirclesAndTwoEnclosures => '𑗖',
            Siddham::SectionMarkWithCirclesAndFourEnclosures => '𑗗',
            Siddham::LetterThreeDashCircleAlternateI => '𑗘',
            Siddham::LetterTwoDashCircleAlternateI => '𑗙',
            Siddham::LetterTwoDashCircleAlternateIi => '𑗚',
            Siddham::LetterAlternateU => '𑗛',
            Siddham::VowelSignAlternateU => '𑗜',
            Siddham::VowelSignAlternateUu => '𑗝',
        }
    }
}

impl std::convert::TryFrom<char> for Siddham {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𑖀' => Ok(Siddham::LetterA),
            '𑖁' => Ok(Siddham::LetterAa),
            '𑖂' => Ok(Siddham::LetterI),
            '𑖃' => Ok(Siddham::LetterIi),
            '𑖄' => Ok(Siddham::LetterU),
            '𑖅' => Ok(Siddham::LetterUu),
            '𑖆' => Ok(Siddham::LetterVocalicR),
            '𑖇' => Ok(Siddham::LetterVocalicRr),
            '𑖈' => Ok(Siddham::LetterVocalicL),
            '𑖉' => Ok(Siddham::LetterVocalicLl),
            '𑖊' => Ok(Siddham::LetterE),
            '𑖋' => Ok(Siddham::LetterAi),
            '𑖌' => Ok(Siddham::LetterO),
            '𑖍' => Ok(Siddham::LetterAu),
            '𑖎' => Ok(Siddham::LetterKa),
            '𑖏' => Ok(Siddham::LetterKha),
            '𑖐' => Ok(Siddham::LetterGa),
            '𑖑' => Ok(Siddham::LetterGha),
            '𑖒' => Ok(Siddham::LetterNga),
            '𑖓' => Ok(Siddham::LetterCa),
            '𑖔' => Ok(Siddham::LetterCha),
            '𑖕' => Ok(Siddham::LetterJa),
            '𑖖' => Ok(Siddham::LetterJha),
            '𑖗' => Ok(Siddham::LetterNya),
            '𑖘' => Ok(Siddham::LetterTta),
            '𑖙' => Ok(Siddham::LetterTtha),
            '𑖚' => Ok(Siddham::LetterDda),
            '𑖛' => Ok(Siddham::LetterDdha),
            '𑖜' => Ok(Siddham::LetterNna),
            '𑖝' => Ok(Siddham::LetterTa),
            '𑖞' => Ok(Siddham::LetterTha),
            '𑖟' => Ok(Siddham::LetterDa),
            '𑖠' => Ok(Siddham::LetterDha),
            '𑖡' => Ok(Siddham::LetterNa),
            '𑖢' => Ok(Siddham::LetterPa),
            '𑖣' => Ok(Siddham::LetterPha),
            '𑖤' => Ok(Siddham::LetterBa),
            '𑖥' => Ok(Siddham::LetterBha),
            '𑖦' => Ok(Siddham::LetterMa),
            '𑖧' => Ok(Siddham::LetterYa),
            '𑖨' => Ok(Siddham::LetterRa),
            '𑖩' => Ok(Siddham::LetterLa),
            '𑖪' => Ok(Siddham::LetterVa),
            '𑖫' => Ok(Siddham::LetterSha),
            '𑖬' => Ok(Siddham::LetterSsa),
            '𑖭' => Ok(Siddham::LetterSa),
            '𑖮' => Ok(Siddham::LetterHa),
            '𑖯' => Ok(Siddham::VowelSignAa),
            '𑖰' => Ok(Siddham::VowelSignI),
            '𑖱' => Ok(Siddham::VowelSignIi),
            '𑖲' => Ok(Siddham::VowelSignU),
            '𑖳' => Ok(Siddham::VowelSignUu),
            '𑖴' => Ok(Siddham::VowelSignVocalicR),
            '𑖵' => Ok(Siddham::VowelSignVocalicRr),
            '𑖸' => Ok(Siddham::VowelSignE),
            '𑖹' => Ok(Siddham::VowelSignAi),
            '𑖺' => Ok(Siddham::VowelSignO),
            '𑖻' => Ok(Siddham::VowelSignAu),
            '𑖼' => Ok(Siddham::SignCandrabindu),
            '𑖽' => Ok(Siddham::SignAnusvara),
            '𑖾' => Ok(Siddham::SignVisarga),
            '𑖿' => Ok(Siddham::SignVirama),
            '𑗀' => Ok(Siddham::SignNukta),
            '𑗁' => Ok(Siddham::Sign),
            '𑗂' => Ok(Siddham::Danda),
            '𑗃' => Ok(Siddham::DoubleDanda),
            '𑗄' => Ok(Siddham::SeparatorDot),
            '𑗅' => Ok(Siddham::SeparatorBar),
            '𑗆' => Ok(Siddham::RepetitionMarkDash1),
            '𑗇' => Ok(Siddham::RepetitionMarkDash2),
            '𑗈' => Ok(Siddham::RepetitionMarkDash3),
            '𑗉' => Ok(Siddham::EndOfTextMark),
            '𑗊' => Ok(Siddham::SectionMarkWithTridentAndUDashShapedOrnaments),
            '𑗋' => Ok(Siddham::SectionMarkWithTridentAndDottedCrescents),
            '𑗌' => Ok(Siddham::SectionMarkWithRaysAndDottedCrescents),
            '𑗍' => Ok(Siddham::SectionMarkWithRaysAndDottedDoubleCrescents),
            '𑗎' => Ok(Siddham::SectionMarkWithRaysAndDottedTripleCrescents),
            '𑗏' => Ok(Siddham::SectionMarkDoubleRing),
            '𑗐' => Ok(Siddham::SectionMarkDoubleRingWithRays),
            '𑗑' => Ok(Siddham::SectionMarkWithDoubleCrescents),
            '𑗒' => Ok(Siddham::SectionMarkWithTripleCrescents),
            '𑗓' => Ok(Siddham::SectionMarkWithQuadrupleCrescents),
            '𑗔' => Ok(Siddham::SectionMarkWithSeptupleCrescents),
            '𑗕' => Ok(Siddham::SectionMarkWithCirclesAndRays),
            '𑗖' => Ok(Siddham::SectionMarkWithCirclesAndTwoEnclosures),
            '𑗗' => Ok(Siddham::SectionMarkWithCirclesAndFourEnclosures),
            '𑗘' => Ok(Siddham::LetterThreeDashCircleAlternateI),
            '𑗙' => Ok(Siddham::LetterTwoDashCircleAlternateI),
            '𑗚' => Ok(Siddham::LetterTwoDashCircleAlternateIi),
            '𑗛' => Ok(Siddham::LetterAlternateU),
            '𑗜' => Ok(Siddham::VowelSignAlternateU),
            '𑗝' => Ok(Siddham::VowelSignAlternateUu),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Siddham {
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

impl std::convert::TryFrom<u32> for Siddham {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Siddham {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Siddham {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Siddham::LetterA
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Siddham{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
