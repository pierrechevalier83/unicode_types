
/// An enum to represent all characters in the Oriya block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Oriya {
    /// \u{b01}: 'ଁ'
    SignCandrabindu,
    /// \u{b02}: 'ଂ'
    SignAnusvara,
    /// \u{b03}: 'ଃ'
    SignVisarga,
    /// \u{b05}: 'ଅ'
    LetterA,
    /// \u{b06}: 'ଆ'
    LetterAa,
    /// \u{b07}: 'ଇ'
    LetterI,
    /// \u{b08}: 'ଈ'
    LetterIi,
    /// \u{b09}: 'ଉ'
    LetterU,
    /// \u{b0a}: 'ଊ'
    LetterUu,
    /// \u{b0b}: 'ଋ'
    LetterVocalicR,
    /// \u{b0c}: 'ଌ'
    LetterVocalicL,
    /// \u{b0f}: 'ଏ'
    LetterE,
    /// \u{b10}: 'ଐ'
    LetterAi,
    /// \u{b13}: 'ଓ'
    LetterO,
    /// \u{b14}: 'ଔ'
    LetterAu,
    /// \u{b15}: 'କ'
    LetterKa,
    /// \u{b16}: 'ଖ'
    LetterKha,
    /// \u{b17}: 'ଗ'
    LetterGa,
    /// \u{b18}: 'ଘ'
    LetterGha,
    /// \u{b19}: 'ଙ'
    LetterNga,
    /// \u{b1a}: 'ଚ'
    LetterCa,
    /// \u{b1b}: 'ଛ'
    LetterCha,
    /// \u{b1c}: 'ଜ'
    LetterJa,
    /// \u{b1d}: 'ଝ'
    LetterJha,
    /// \u{b1e}: 'ଞ'
    LetterNya,
    /// \u{b1f}: 'ଟ'
    LetterTta,
    /// \u{b20}: 'ଠ'
    LetterTtha,
    /// \u{b21}: 'ଡ'
    LetterDda,
    /// \u{b22}: 'ଢ'
    LetterDdha,
    /// \u{b23}: 'ଣ'
    LetterNna,
    /// \u{b24}: 'ତ'
    LetterTa,
    /// \u{b25}: 'ଥ'
    LetterTha,
    /// \u{b26}: 'ଦ'
    LetterDa,
    /// \u{b27}: 'ଧ'
    LetterDha,
    /// \u{b28}: 'ନ'
    LetterNa,
    /// \u{b2a}: 'ପ'
    LetterPa,
    /// \u{b2b}: 'ଫ'
    LetterPha,
    /// \u{b2c}: 'ବ'
    LetterBa,
    /// \u{b2d}: 'ଭ'
    LetterBha,
    /// \u{b2e}: 'ମ'
    LetterMa,
    /// \u{b2f}: 'ଯ'
    LetterYa,
    /// \u{b30}: 'ର'
    LetterRa,
    /// \u{b32}: 'ଲ'
    LetterLa,
    /// \u{b33}: 'ଳ'
    LetterLla,
    /// \u{b35}: 'ଵ'
    LetterVa,
    /// \u{b36}: 'ଶ'
    LetterSha,
    /// \u{b37}: 'ଷ'
    LetterSsa,
    /// \u{b38}: 'ସ'
    LetterSa,
    /// \u{b39}: 'ହ'
    LetterHa,
    /// \u{b3c}: '଼'
    SignNukta,
    /// \u{b3d}: 'ଽ'
    SignAvagraha,
    /// \u{b3e}: 'ା'
    VowelSignAa,
    /// \u{b3f}: 'ି'
    VowelSignI,
    /// \u{b40}: 'ୀ'
    VowelSignIi,
    /// \u{b41}: 'ୁ'
    VowelSignU,
    /// \u{b42}: 'ୂ'
    VowelSignUu,
    /// \u{b43}: 'ୃ'
    VowelSignVocalicR,
    /// \u{b44}: 'ୄ'
    VowelSignVocalicRr,
    /// \u{b47}: 'େ'
    VowelSignE,
    /// \u{b48}: 'ୈ'
    VowelSignAi,
    /// \u{b4b}: 'ୋ'
    VowelSignO,
    /// \u{b4c}: 'ୌ'
    VowelSignAu,
    /// \u{b4d}: '୍'
    SignVirama,
    /// \u{b56}: 'ୖ'
    AiLengthMark,
    /// \u{b57}: 'ୗ'
    AuLengthMark,
    /// \u{b5c}: 'ଡ଼'
    LetterRra,
    /// \u{b5d}: 'ଢ଼'
    LetterRha,
    /// \u{b5f}: 'ୟ'
    LetterYya,
    /// \u{b60}: 'ୠ'
    LetterVocalicRr,
    /// \u{b61}: 'ୡ'
    LetterVocalicLl,
    /// \u{b62}: 'ୢ'
    VowelSignVocalicL,
    /// \u{b63}: 'ୣ'
    VowelSignVocalicLl,
    /// \u{b66}: '୦'
    DigitZero,
    /// \u{b67}: '୧'
    DigitOne,
    /// \u{b68}: '୨'
    DigitTwo,
    /// \u{b69}: '୩'
    DigitThree,
    /// \u{b6a}: '୪'
    DigitFour,
    /// \u{b6b}: '୫'
    DigitFive,
    /// \u{b6c}: '୬'
    DigitSix,
    /// \u{b6d}: '୭'
    DigitSeven,
    /// \u{b6e}: '୮'
    DigitEight,
    /// \u{b6f}: '୯'
    DigitNine,
    /// \u{b70}: '୰'
    Isshar,
    /// \u{b71}: 'ୱ'
    LetterWa,
    /// \u{b72}: '୲'
    FractionOneQuarter,
    /// \u{b73}: '୳'
    FractionOneHalf,
    /// \u{b74}: '୴'
    FractionThreeQuarters,
    /// \u{b75}: '୵'
    FractionOneSixteenth,
    /// \u{b76}: '୶'
    FractionOneEighth,
    /// \u{b77}: '୷'
    FractionThreeSixteenths,
}

impl Into<char> for Oriya {
    fn into(self) -> char {
        match self {
            Oriya::SignCandrabindu => 'ଁ',
            Oriya::SignAnusvara => 'ଂ',
            Oriya::SignVisarga => 'ଃ',
            Oriya::LetterA => 'ଅ',
            Oriya::LetterAa => 'ଆ',
            Oriya::LetterI => 'ଇ',
            Oriya::LetterIi => 'ଈ',
            Oriya::LetterU => 'ଉ',
            Oriya::LetterUu => 'ଊ',
            Oriya::LetterVocalicR => 'ଋ',
            Oriya::LetterVocalicL => 'ଌ',
            Oriya::LetterE => 'ଏ',
            Oriya::LetterAi => 'ଐ',
            Oriya::LetterO => 'ଓ',
            Oriya::LetterAu => 'ଔ',
            Oriya::LetterKa => 'କ',
            Oriya::LetterKha => 'ଖ',
            Oriya::LetterGa => 'ଗ',
            Oriya::LetterGha => 'ଘ',
            Oriya::LetterNga => 'ଙ',
            Oriya::LetterCa => 'ଚ',
            Oriya::LetterCha => 'ଛ',
            Oriya::LetterJa => 'ଜ',
            Oriya::LetterJha => 'ଝ',
            Oriya::LetterNya => 'ଞ',
            Oriya::LetterTta => 'ଟ',
            Oriya::LetterTtha => 'ଠ',
            Oriya::LetterDda => 'ଡ',
            Oriya::LetterDdha => 'ଢ',
            Oriya::LetterNna => 'ଣ',
            Oriya::LetterTa => 'ତ',
            Oriya::LetterTha => 'ଥ',
            Oriya::LetterDa => 'ଦ',
            Oriya::LetterDha => 'ଧ',
            Oriya::LetterNa => 'ନ',
            Oriya::LetterPa => 'ପ',
            Oriya::LetterPha => 'ଫ',
            Oriya::LetterBa => 'ବ',
            Oriya::LetterBha => 'ଭ',
            Oriya::LetterMa => 'ମ',
            Oriya::LetterYa => 'ଯ',
            Oriya::LetterRa => 'ର',
            Oriya::LetterLa => 'ଲ',
            Oriya::LetterLla => 'ଳ',
            Oriya::LetterVa => 'ଵ',
            Oriya::LetterSha => 'ଶ',
            Oriya::LetterSsa => 'ଷ',
            Oriya::LetterSa => 'ସ',
            Oriya::LetterHa => 'ହ',
            Oriya::SignNukta => '଼',
            Oriya::SignAvagraha => 'ଽ',
            Oriya::VowelSignAa => 'ା',
            Oriya::VowelSignI => 'ି',
            Oriya::VowelSignIi => 'ୀ',
            Oriya::VowelSignU => 'ୁ',
            Oriya::VowelSignUu => 'ୂ',
            Oriya::VowelSignVocalicR => 'ୃ',
            Oriya::VowelSignVocalicRr => 'ୄ',
            Oriya::VowelSignE => 'େ',
            Oriya::VowelSignAi => 'ୈ',
            Oriya::VowelSignO => 'ୋ',
            Oriya::VowelSignAu => 'ୌ',
            Oriya::SignVirama => '୍',
            Oriya::AiLengthMark => 'ୖ',
            Oriya::AuLengthMark => 'ୗ',
            Oriya::LetterRra => 'ଡ଼',
            Oriya::LetterRha => 'ଢ଼',
            Oriya::LetterYya => 'ୟ',
            Oriya::LetterVocalicRr => 'ୠ',
            Oriya::LetterVocalicLl => 'ୡ',
            Oriya::VowelSignVocalicL => 'ୢ',
            Oriya::VowelSignVocalicLl => 'ୣ',
            Oriya::DigitZero => '୦',
            Oriya::DigitOne => '୧',
            Oriya::DigitTwo => '୨',
            Oriya::DigitThree => '୩',
            Oriya::DigitFour => '୪',
            Oriya::DigitFive => '୫',
            Oriya::DigitSix => '୬',
            Oriya::DigitSeven => '୭',
            Oriya::DigitEight => '୮',
            Oriya::DigitNine => '୯',
            Oriya::Isshar => '୰',
            Oriya::LetterWa => 'ୱ',
            Oriya::FractionOneQuarter => '୲',
            Oriya::FractionOneHalf => '୳',
            Oriya::FractionThreeQuarters => '୴',
            Oriya::FractionOneSixteenth => '୵',
            Oriya::FractionOneEighth => '୶',
            Oriya::FractionThreeSixteenths => '୷',
        }
    }
}

impl std::convert::TryFrom<char> for Oriya {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ଁ' => Ok(Oriya::SignCandrabindu),
            'ଂ' => Ok(Oriya::SignAnusvara),
            'ଃ' => Ok(Oriya::SignVisarga),
            'ଅ' => Ok(Oriya::LetterA),
            'ଆ' => Ok(Oriya::LetterAa),
            'ଇ' => Ok(Oriya::LetterI),
            'ଈ' => Ok(Oriya::LetterIi),
            'ଉ' => Ok(Oriya::LetterU),
            'ଊ' => Ok(Oriya::LetterUu),
            'ଋ' => Ok(Oriya::LetterVocalicR),
            'ଌ' => Ok(Oriya::LetterVocalicL),
            'ଏ' => Ok(Oriya::LetterE),
            'ଐ' => Ok(Oriya::LetterAi),
            'ଓ' => Ok(Oriya::LetterO),
            'ଔ' => Ok(Oriya::LetterAu),
            'କ' => Ok(Oriya::LetterKa),
            'ଖ' => Ok(Oriya::LetterKha),
            'ଗ' => Ok(Oriya::LetterGa),
            'ଘ' => Ok(Oriya::LetterGha),
            'ଙ' => Ok(Oriya::LetterNga),
            'ଚ' => Ok(Oriya::LetterCa),
            'ଛ' => Ok(Oriya::LetterCha),
            'ଜ' => Ok(Oriya::LetterJa),
            'ଝ' => Ok(Oriya::LetterJha),
            'ଞ' => Ok(Oriya::LetterNya),
            'ଟ' => Ok(Oriya::LetterTta),
            'ଠ' => Ok(Oriya::LetterTtha),
            'ଡ' => Ok(Oriya::LetterDda),
            'ଢ' => Ok(Oriya::LetterDdha),
            'ଣ' => Ok(Oriya::LetterNna),
            'ତ' => Ok(Oriya::LetterTa),
            'ଥ' => Ok(Oriya::LetterTha),
            'ଦ' => Ok(Oriya::LetterDa),
            'ଧ' => Ok(Oriya::LetterDha),
            'ନ' => Ok(Oriya::LetterNa),
            'ପ' => Ok(Oriya::LetterPa),
            'ଫ' => Ok(Oriya::LetterPha),
            'ବ' => Ok(Oriya::LetterBa),
            'ଭ' => Ok(Oriya::LetterBha),
            'ମ' => Ok(Oriya::LetterMa),
            'ଯ' => Ok(Oriya::LetterYa),
            'ର' => Ok(Oriya::LetterRa),
            'ଲ' => Ok(Oriya::LetterLa),
            'ଳ' => Ok(Oriya::LetterLla),
            'ଵ' => Ok(Oriya::LetterVa),
            'ଶ' => Ok(Oriya::LetterSha),
            'ଷ' => Ok(Oriya::LetterSsa),
            'ସ' => Ok(Oriya::LetterSa),
            'ହ' => Ok(Oriya::LetterHa),
            '଼' => Ok(Oriya::SignNukta),
            'ଽ' => Ok(Oriya::SignAvagraha),
            'ା' => Ok(Oriya::VowelSignAa),
            'ି' => Ok(Oriya::VowelSignI),
            'ୀ' => Ok(Oriya::VowelSignIi),
            'ୁ' => Ok(Oriya::VowelSignU),
            'ୂ' => Ok(Oriya::VowelSignUu),
            'ୃ' => Ok(Oriya::VowelSignVocalicR),
            'ୄ' => Ok(Oriya::VowelSignVocalicRr),
            'େ' => Ok(Oriya::VowelSignE),
            'ୈ' => Ok(Oriya::VowelSignAi),
            'ୋ' => Ok(Oriya::VowelSignO),
            'ୌ' => Ok(Oriya::VowelSignAu),
            '୍' => Ok(Oriya::SignVirama),
            'ୖ' => Ok(Oriya::AiLengthMark),
            'ୗ' => Ok(Oriya::AuLengthMark),
            'ଡ଼' => Ok(Oriya::LetterRra),
            'ଢ଼' => Ok(Oriya::LetterRha),
            'ୟ' => Ok(Oriya::LetterYya),
            'ୠ' => Ok(Oriya::LetterVocalicRr),
            'ୡ' => Ok(Oriya::LetterVocalicLl),
            'ୢ' => Ok(Oriya::VowelSignVocalicL),
            'ୣ' => Ok(Oriya::VowelSignVocalicLl),
            '୦' => Ok(Oriya::DigitZero),
            '୧' => Ok(Oriya::DigitOne),
            '୨' => Ok(Oriya::DigitTwo),
            '୩' => Ok(Oriya::DigitThree),
            '୪' => Ok(Oriya::DigitFour),
            '୫' => Ok(Oriya::DigitFive),
            '୬' => Ok(Oriya::DigitSix),
            '୭' => Ok(Oriya::DigitSeven),
            '୮' => Ok(Oriya::DigitEight),
            '୯' => Ok(Oriya::DigitNine),
            '୰' => Ok(Oriya::Isshar),
            'ୱ' => Ok(Oriya::LetterWa),
            '୲' => Ok(Oriya::FractionOneQuarter),
            '୳' => Ok(Oriya::FractionOneHalf),
            '୴' => Ok(Oriya::FractionThreeQuarters),
            '୵' => Ok(Oriya::FractionOneSixteenth),
            '୶' => Ok(Oriya::FractionOneEighth),
            '୷' => Ok(Oriya::FractionThreeSixteenths),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Oriya {
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

impl std::convert::TryFrom<u32> for Oriya {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Oriya {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Oriya {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Oriya::SignCandrabindu
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Oriya::SignCandrabindu => "oriya sign candrabindu",
            Oriya::SignAnusvara => "oriya sign anusvara",
            Oriya::SignVisarga => "oriya sign visarga",
            Oriya::LetterA => "oriya letter a",
            Oriya::LetterAa => "oriya letter aa",
            Oriya::LetterI => "oriya letter i",
            Oriya::LetterIi => "oriya letter ii",
            Oriya::LetterU => "oriya letter u",
            Oriya::LetterUu => "oriya letter uu",
            Oriya::LetterVocalicR => "oriya letter vocalic r",
            Oriya::LetterVocalicL => "oriya letter vocalic l",
            Oriya::LetterE => "oriya letter e",
            Oriya::LetterAi => "oriya letter ai",
            Oriya::LetterO => "oriya letter o",
            Oriya::LetterAu => "oriya letter au",
            Oriya::LetterKa => "oriya letter ka",
            Oriya::LetterKha => "oriya letter kha",
            Oriya::LetterGa => "oriya letter ga",
            Oriya::LetterGha => "oriya letter gha",
            Oriya::LetterNga => "oriya letter nga",
            Oriya::LetterCa => "oriya letter ca",
            Oriya::LetterCha => "oriya letter cha",
            Oriya::LetterJa => "oriya letter ja",
            Oriya::LetterJha => "oriya letter jha",
            Oriya::LetterNya => "oriya letter nya",
            Oriya::LetterTta => "oriya letter tta",
            Oriya::LetterTtha => "oriya letter ttha",
            Oriya::LetterDda => "oriya letter dda",
            Oriya::LetterDdha => "oriya letter ddha",
            Oriya::LetterNna => "oriya letter nna",
            Oriya::LetterTa => "oriya letter ta",
            Oriya::LetterTha => "oriya letter tha",
            Oriya::LetterDa => "oriya letter da",
            Oriya::LetterDha => "oriya letter dha",
            Oriya::LetterNa => "oriya letter na",
            Oriya::LetterPa => "oriya letter pa",
            Oriya::LetterPha => "oriya letter pha",
            Oriya::LetterBa => "oriya letter ba",
            Oriya::LetterBha => "oriya letter bha",
            Oriya::LetterMa => "oriya letter ma",
            Oriya::LetterYa => "oriya letter ya",
            Oriya::LetterRa => "oriya letter ra",
            Oriya::LetterLa => "oriya letter la",
            Oriya::LetterLla => "oriya letter lla",
            Oriya::LetterVa => "oriya letter va",
            Oriya::LetterSha => "oriya letter sha",
            Oriya::LetterSsa => "oriya letter ssa",
            Oriya::LetterSa => "oriya letter sa",
            Oriya::LetterHa => "oriya letter ha",
            Oriya::SignNukta => "oriya sign nukta",
            Oriya::SignAvagraha => "oriya sign avagraha",
            Oriya::VowelSignAa => "oriya vowel sign aa",
            Oriya::VowelSignI => "oriya vowel sign i",
            Oriya::VowelSignIi => "oriya vowel sign ii",
            Oriya::VowelSignU => "oriya vowel sign u",
            Oriya::VowelSignUu => "oriya vowel sign uu",
            Oriya::VowelSignVocalicR => "oriya vowel sign vocalic r",
            Oriya::VowelSignVocalicRr => "oriya vowel sign vocalic rr",
            Oriya::VowelSignE => "oriya vowel sign e",
            Oriya::VowelSignAi => "oriya vowel sign ai",
            Oriya::VowelSignO => "oriya vowel sign o",
            Oriya::VowelSignAu => "oriya vowel sign au",
            Oriya::SignVirama => "oriya sign virama",
            Oriya::AiLengthMark => "oriya ai length mark",
            Oriya::AuLengthMark => "oriya au length mark",
            Oriya::LetterRra => "oriya letter rra",
            Oriya::LetterRha => "oriya letter rha",
            Oriya::LetterYya => "oriya letter yya",
            Oriya::LetterVocalicRr => "oriya letter vocalic rr",
            Oriya::LetterVocalicLl => "oriya letter vocalic ll",
            Oriya::VowelSignVocalicL => "oriya vowel sign vocalic l",
            Oriya::VowelSignVocalicLl => "oriya vowel sign vocalic ll",
            Oriya::DigitZero => "oriya digit zero",
            Oriya::DigitOne => "oriya digit one",
            Oriya::DigitTwo => "oriya digit two",
            Oriya::DigitThree => "oriya digit three",
            Oriya::DigitFour => "oriya digit four",
            Oriya::DigitFive => "oriya digit five",
            Oriya::DigitSix => "oriya digit six",
            Oriya::DigitSeven => "oriya digit seven",
            Oriya::DigitEight => "oriya digit eight",
            Oriya::DigitNine => "oriya digit nine",
            Oriya::Isshar => "oriya isshar",
            Oriya::LetterWa => "oriya letter wa",
            Oriya::FractionOneQuarter => "oriya fraction one quarter",
            Oriya::FractionOneHalf => "oriya fraction one half",
            Oriya::FractionThreeQuarters => "oriya fraction three quarters",
            Oriya::FractionOneSixteenth => "oriya fraction one sixteenth",
            Oriya::FractionOneEighth => "oriya fraction one eighth",
            Oriya::FractionThreeSixteenths => "oriya fraction three sixteenths",
        }
    }
}
