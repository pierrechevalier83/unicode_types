
/// An enum to represent all characters in the Malayalam block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Malayalam {
    /// \u{d00}: 'ഀ'
    SignCombiningAnusvaraAbove,
    /// \u{d01}: 'ഁ'
    SignCandrabindu,
    /// \u{d02}: 'ം'
    SignAnusvara,
    /// \u{d03}: 'ഃ'
    SignVisarga,
    /// \u{d05}: 'അ'
    LetterA,
    /// \u{d06}: 'ആ'
    LetterAa,
    /// \u{d07}: 'ഇ'
    LetterI,
    /// \u{d08}: 'ഈ'
    LetterIi,
    /// \u{d09}: 'ഉ'
    LetterU,
    /// \u{d0a}: 'ഊ'
    LetterUu,
    /// \u{d0b}: 'ഋ'
    LetterVocalicR,
    /// \u{d0c}: 'ഌ'
    LetterVocalicL,
    /// \u{d0e}: 'എ'
    LetterE,
    /// \u{d0f}: 'ഏ'
    LetterEe,
    /// \u{d10}: 'ഐ'
    LetterAi,
    /// \u{d12}: 'ഒ'
    LetterO,
    /// \u{d13}: 'ഓ'
    LetterOo,
    /// \u{d14}: 'ഔ'
    LetterAu,
    /// \u{d15}: 'ക'
    LetterKa,
    /// \u{d16}: 'ഖ'
    LetterKha,
    /// \u{d17}: 'ഗ'
    LetterGa,
    /// \u{d18}: 'ഘ'
    LetterGha,
    /// \u{d19}: 'ങ'
    LetterNga,
    /// \u{d1a}: 'ച'
    LetterCa,
    /// \u{d1b}: 'ഛ'
    LetterCha,
    /// \u{d1c}: 'ജ'
    LetterJa,
    /// \u{d1d}: 'ഝ'
    LetterJha,
    /// \u{d1e}: 'ഞ'
    LetterNya,
    /// \u{d1f}: 'ട'
    LetterTta,
    /// \u{d20}: 'ഠ'
    LetterTtha,
    /// \u{d21}: 'ഡ'
    LetterDda,
    /// \u{d22}: 'ഢ'
    LetterDdha,
    /// \u{d23}: 'ണ'
    LetterNna,
    /// \u{d24}: 'ത'
    LetterTa,
    /// \u{d25}: 'ഥ'
    LetterTha,
    /// \u{d26}: 'ദ'
    LetterDa,
    /// \u{d27}: 'ധ'
    LetterDha,
    /// \u{d28}: 'ന'
    LetterNa,
    /// \u{d29}: 'ഩ'
    LetterNnna,
    /// \u{d2a}: 'പ'
    LetterPa,
    /// \u{d2b}: 'ഫ'
    LetterPha,
    /// \u{d2c}: 'ബ'
    LetterBa,
    /// \u{d2d}: 'ഭ'
    LetterBha,
    /// \u{d2e}: 'മ'
    LetterMa,
    /// \u{d2f}: 'യ'
    LetterYa,
    /// \u{d30}: 'ര'
    LetterRa,
    /// \u{d31}: 'റ'
    LetterRra,
    /// \u{d32}: 'ല'
    LetterLa,
    /// \u{d33}: 'ള'
    LetterLla,
    /// \u{d34}: 'ഴ'
    LetterLlla,
    /// \u{d35}: 'വ'
    LetterVa,
    /// \u{d36}: 'ശ'
    LetterSha,
    /// \u{d37}: 'ഷ'
    LetterSsa,
    /// \u{d38}: 'സ'
    LetterSa,
    /// \u{d39}: 'ഹ'
    LetterHa,
    /// \u{d3a}: 'ഺ'
    LetterTtta,
    /// \u{d3b}: '഻'
    SignVerticalBarVirama,
    /// \u{d3c}: '഼'
    SignCircularVirama,
    /// \u{d3d}: 'ഽ'
    SignAvagraha,
    /// \u{d3e}: 'ാ'
    VowelSignAa,
    /// \u{d3f}: 'ി'
    VowelSignI,
    /// \u{d40}: 'ീ'
    VowelSignIi,
    /// \u{d41}: 'ു'
    VowelSignU,
    /// \u{d42}: 'ൂ'
    VowelSignUu,
    /// \u{d43}: 'ൃ'
    VowelSignVocalicR,
    /// \u{d44}: 'ൄ'
    VowelSignVocalicRr,
    /// \u{d46}: 'െ'
    VowelSignE,
    /// \u{d47}: 'േ'
    VowelSignEe,
    /// \u{d48}: 'ൈ'
    VowelSignAi,
    /// \u{d4a}: 'ൊ'
    VowelSignO,
    /// \u{d4b}: 'ോ'
    VowelSignOo,
    /// \u{d4c}: 'ൌ'
    VowelSignAu,
    /// \u{d4d}: '്'
    SignVirama,
    /// \u{d4e}: 'ൎ'
    LetterDotReph,
    /// \u{d4f}: '൏'
    SignPara,
    /// \u{d54}: 'ൔ'
    LetterChilluM,
    /// \u{d55}: 'ൕ'
    LetterChilluY,
    /// \u{d56}: 'ൖ'
    LetterChilluLll,
    /// \u{d57}: 'ൗ'
    AuLengthMark,
    /// \u{d58}: '൘'
    FractionOneOneDashHundredDashAndDashSixtieth,
    /// \u{d59}: '൙'
    FractionOneFortieth,
    /// \u{d5a}: '൚'
    FractionThreeEightieths,
    /// \u{d5b}: '൛'
    FractionOneTwentieth,
    /// \u{d5c}: '൜'
    FractionOneTenth,
    /// \u{d5d}: '൝'
    FractionThreeTwentieths,
    /// \u{d5e}: '൞'
    FractionOneFifth,
    /// \u{d5f}: 'ൟ'
    LetterArchaicIi,
    /// \u{d60}: 'ൠ'
    LetterVocalicRr,
    /// \u{d61}: 'ൡ'
    LetterVocalicLl,
    /// \u{d62}: 'ൢ'
    VowelSignVocalicL,
    /// \u{d63}: 'ൣ'
    VowelSignVocalicLl,
    /// \u{d66}: '൦'
    DigitZero,
    /// \u{d67}: '൧'
    DigitOne,
    /// \u{d68}: '൨'
    DigitTwo,
    /// \u{d69}: '൩'
    DigitThree,
    /// \u{d6a}: '൪'
    DigitFour,
    /// \u{d6b}: '൫'
    DigitFive,
    /// \u{d6c}: '൬'
    DigitSix,
    /// \u{d6d}: '൭'
    DigitSeven,
    /// \u{d6e}: '൮'
    DigitEight,
    /// \u{d6f}: '൯'
    DigitNine,
    /// \u{d70}: '൰'
    NumberTen,
    /// \u{d71}: '൱'
    NumberOneHundred,
    /// \u{d72}: '൲'
    NumberOneThousand,
    /// \u{d73}: '൳'
    FractionOneQuarter,
    /// \u{d74}: '൴'
    FractionOneHalf,
    /// \u{d75}: '൵'
    FractionThreeQuarters,
    /// \u{d76}: '൶'
    FractionOneSixteenth,
    /// \u{d77}: '൷'
    FractionOneEighth,
    /// \u{d78}: '൸'
    FractionThreeSixteenths,
    /// \u{d79}: '൹'
    DateMark,
    /// \u{d7a}: 'ൺ'
    LetterChilluNn,
    /// \u{d7b}: 'ൻ'
    LetterChilluN,
    /// \u{d7c}: 'ർ'
    LetterChilluRr,
    /// \u{d7d}: 'ൽ'
    LetterChilluL,
    /// \u{d7e}: 'ൾ'
    LetterChilluLl,
}

impl Into<char> for Malayalam {
    fn into(self) -> char {
        match self {
            Malayalam::SignCombiningAnusvaraAbove => 'ഀ',
            Malayalam::SignCandrabindu => 'ഁ',
            Malayalam::SignAnusvara => 'ം',
            Malayalam::SignVisarga => 'ഃ',
            Malayalam::LetterA => 'അ',
            Malayalam::LetterAa => 'ആ',
            Malayalam::LetterI => 'ഇ',
            Malayalam::LetterIi => 'ഈ',
            Malayalam::LetterU => 'ഉ',
            Malayalam::LetterUu => 'ഊ',
            Malayalam::LetterVocalicR => 'ഋ',
            Malayalam::LetterVocalicL => 'ഌ',
            Malayalam::LetterE => 'എ',
            Malayalam::LetterEe => 'ഏ',
            Malayalam::LetterAi => 'ഐ',
            Malayalam::LetterO => 'ഒ',
            Malayalam::LetterOo => 'ഓ',
            Malayalam::LetterAu => 'ഔ',
            Malayalam::LetterKa => 'ക',
            Malayalam::LetterKha => 'ഖ',
            Malayalam::LetterGa => 'ഗ',
            Malayalam::LetterGha => 'ഘ',
            Malayalam::LetterNga => 'ങ',
            Malayalam::LetterCa => 'ച',
            Malayalam::LetterCha => 'ഛ',
            Malayalam::LetterJa => 'ജ',
            Malayalam::LetterJha => 'ഝ',
            Malayalam::LetterNya => 'ഞ',
            Malayalam::LetterTta => 'ട',
            Malayalam::LetterTtha => 'ഠ',
            Malayalam::LetterDda => 'ഡ',
            Malayalam::LetterDdha => 'ഢ',
            Malayalam::LetterNna => 'ണ',
            Malayalam::LetterTa => 'ത',
            Malayalam::LetterTha => 'ഥ',
            Malayalam::LetterDa => 'ദ',
            Malayalam::LetterDha => 'ധ',
            Malayalam::LetterNa => 'ന',
            Malayalam::LetterNnna => 'ഩ',
            Malayalam::LetterPa => 'പ',
            Malayalam::LetterPha => 'ഫ',
            Malayalam::LetterBa => 'ബ',
            Malayalam::LetterBha => 'ഭ',
            Malayalam::LetterMa => 'മ',
            Malayalam::LetterYa => 'യ',
            Malayalam::LetterRa => 'ര',
            Malayalam::LetterRra => 'റ',
            Malayalam::LetterLa => 'ല',
            Malayalam::LetterLla => 'ള',
            Malayalam::LetterLlla => 'ഴ',
            Malayalam::LetterVa => 'വ',
            Malayalam::LetterSha => 'ശ',
            Malayalam::LetterSsa => 'ഷ',
            Malayalam::LetterSa => 'സ',
            Malayalam::LetterHa => 'ഹ',
            Malayalam::LetterTtta => 'ഺ',
            Malayalam::SignVerticalBarVirama => '഻',
            Malayalam::SignCircularVirama => '഼',
            Malayalam::SignAvagraha => 'ഽ',
            Malayalam::VowelSignAa => 'ാ',
            Malayalam::VowelSignI => 'ി',
            Malayalam::VowelSignIi => 'ീ',
            Malayalam::VowelSignU => 'ു',
            Malayalam::VowelSignUu => 'ൂ',
            Malayalam::VowelSignVocalicR => 'ൃ',
            Malayalam::VowelSignVocalicRr => 'ൄ',
            Malayalam::VowelSignE => 'െ',
            Malayalam::VowelSignEe => 'േ',
            Malayalam::VowelSignAi => 'ൈ',
            Malayalam::VowelSignO => 'ൊ',
            Malayalam::VowelSignOo => 'ോ',
            Malayalam::VowelSignAu => 'ൌ',
            Malayalam::SignVirama => '്',
            Malayalam::LetterDotReph => 'ൎ',
            Malayalam::SignPara => '൏',
            Malayalam::LetterChilluM => 'ൔ',
            Malayalam::LetterChilluY => 'ൕ',
            Malayalam::LetterChilluLll => 'ൖ',
            Malayalam::AuLengthMark => 'ൗ',
            Malayalam::FractionOneOneDashHundredDashAndDashSixtieth => '൘',
            Malayalam::FractionOneFortieth => '൙',
            Malayalam::FractionThreeEightieths => '൚',
            Malayalam::FractionOneTwentieth => '൛',
            Malayalam::FractionOneTenth => '൜',
            Malayalam::FractionThreeTwentieths => '൝',
            Malayalam::FractionOneFifth => '൞',
            Malayalam::LetterArchaicIi => 'ൟ',
            Malayalam::LetterVocalicRr => 'ൠ',
            Malayalam::LetterVocalicLl => 'ൡ',
            Malayalam::VowelSignVocalicL => 'ൢ',
            Malayalam::VowelSignVocalicLl => 'ൣ',
            Malayalam::DigitZero => '൦',
            Malayalam::DigitOne => '൧',
            Malayalam::DigitTwo => '൨',
            Malayalam::DigitThree => '൩',
            Malayalam::DigitFour => '൪',
            Malayalam::DigitFive => '൫',
            Malayalam::DigitSix => '൬',
            Malayalam::DigitSeven => '൭',
            Malayalam::DigitEight => '൮',
            Malayalam::DigitNine => '൯',
            Malayalam::NumberTen => '൰',
            Malayalam::NumberOneHundred => '൱',
            Malayalam::NumberOneThousand => '൲',
            Malayalam::FractionOneQuarter => '൳',
            Malayalam::FractionOneHalf => '൴',
            Malayalam::FractionThreeQuarters => '൵',
            Malayalam::FractionOneSixteenth => '൶',
            Malayalam::FractionOneEighth => '൷',
            Malayalam::FractionThreeSixteenths => '൸',
            Malayalam::DateMark => '൹',
            Malayalam::LetterChilluNn => 'ൺ',
            Malayalam::LetterChilluN => 'ൻ',
            Malayalam::LetterChilluRr => 'ർ',
            Malayalam::LetterChilluL => 'ൽ',
            Malayalam::LetterChilluLl => 'ൾ',
        }
    }
}

impl std::convert::TryFrom<char> for Malayalam {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ഀ' => Ok(Malayalam::SignCombiningAnusvaraAbove),
            'ഁ' => Ok(Malayalam::SignCandrabindu),
            'ം' => Ok(Malayalam::SignAnusvara),
            'ഃ' => Ok(Malayalam::SignVisarga),
            'അ' => Ok(Malayalam::LetterA),
            'ആ' => Ok(Malayalam::LetterAa),
            'ഇ' => Ok(Malayalam::LetterI),
            'ഈ' => Ok(Malayalam::LetterIi),
            'ഉ' => Ok(Malayalam::LetterU),
            'ഊ' => Ok(Malayalam::LetterUu),
            'ഋ' => Ok(Malayalam::LetterVocalicR),
            'ഌ' => Ok(Malayalam::LetterVocalicL),
            'എ' => Ok(Malayalam::LetterE),
            'ഏ' => Ok(Malayalam::LetterEe),
            'ഐ' => Ok(Malayalam::LetterAi),
            'ഒ' => Ok(Malayalam::LetterO),
            'ഓ' => Ok(Malayalam::LetterOo),
            'ഔ' => Ok(Malayalam::LetterAu),
            'ക' => Ok(Malayalam::LetterKa),
            'ഖ' => Ok(Malayalam::LetterKha),
            'ഗ' => Ok(Malayalam::LetterGa),
            'ഘ' => Ok(Malayalam::LetterGha),
            'ങ' => Ok(Malayalam::LetterNga),
            'ച' => Ok(Malayalam::LetterCa),
            'ഛ' => Ok(Malayalam::LetterCha),
            'ജ' => Ok(Malayalam::LetterJa),
            'ഝ' => Ok(Malayalam::LetterJha),
            'ഞ' => Ok(Malayalam::LetterNya),
            'ട' => Ok(Malayalam::LetterTta),
            'ഠ' => Ok(Malayalam::LetterTtha),
            'ഡ' => Ok(Malayalam::LetterDda),
            'ഢ' => Ok(Malayalam::LetterDdha),
            'ണ' => Ok(Malayalam::LetterNna),
            'ത' => Ok(Malayalam::LetterTa),
            'ഥ' => Ok(Malayalam::LetterTha),
            'ദ' => Ok(Malayalam::LetterDa),
            'ധ' => Ok(Malayalam::LetterDha),
            'ന' => Ok(Malayalam::LetterNa),
            'ഩ' => Ok(Malayalam::LetterNnna),
            'പ' => Ok(Malayalam::LetterPa),
            'ഫ' => Ok(Malayalam::LetterPha),
            'ബ' => Ok(Malayalam::LetterBa),
            'ഭ' => Ok(Malayalam::LetterBha),
            'മ' => Ok(Malayalam::LetterMa),
            'യ' => Ok(Malayalam::LetterYa),
            'ര' => Ok(Malayalam::LetterRa),
            'റ' => Ok(Malayalam::LetterRra),
            'ല' => Ok(Malayalam::LetterLa),
            'ള' => Ok(Malayalam::LetterLla),
            'ഴ' => Ok(Malayalam::LetterLlla),
            'വ' => Ok(Malayalam::LetterVa),
            'ശ' => Ok(Malayalam::LetterSha),
            'ഷ' => Ok(Malayalam::LetterSsa),
            'സ' => Ok(Malayalam::LetterSa),
            'ഹ' => Ok(Malayalam::LetterHa),
            'ഺ' => Ok(Malayalam::LetterTtta),
            '഻' => Ok(Malayalam::SignVerticalBarVirama),
            '഼' => Ok(Malayalam::SignCircularVirama),
            'ഽ' => Ok(Malayalam::SignAvagraha),
            'ാ' => Ok(Malayalam::VowelSignAa),
            'ി' => Ok(Malayalam::VowelSignI),
            'ീ' => Ok(Malayalam::VowelSignIi),
            'ു' => Ok(Malayalam::VowelSignU),
            'ൂ' => Ok(Malayalam::VowelSignUu),
            'ൃ' => Ok(Malayalam::VowelSignVocalicR),
            'ൄ' => Ok(Malayalam::VowelSignVocalicRr),
            'െ' => Ok(Malayalam::VowelSignE),
            'േ' => Ok(Malayalam::VowelSignEe),
            'ൈ' => Ok(Malayalam::VowelSignAi),
            'ൊ' => Ok(Malayalam::VowelSignO),
            'ോ' => Ok(Malayalam::VowelSignOo),
            'ൌ' => Ok(Malayalam::VowelSignAu),
            '്' => Ok(Malayalam::SignVirama),
            'ൎ' => Ok(Malayalam::LetterDotReph),
            '൏' => Ok(Malayalam::SignPara),
            'ൔ' => Ok(Malayalam::LetterChilluM),
            'ൕ' => Ok(Malayalam::LetterChilluY),
            'ൖ' => Ok(Malayalam::LetterChilluLll),
            'ൗ' => Ok(Malayalam::AuLengthMark),
            '൘' => Ok(Malayalam::FractionOneOneDashHundredDashAndDashSixtieth),
            '൙' => Ok(Malayalam::FractionOneFortieth),
            '൚' => Ok(Malayalam::FractionThreeEightieths),
            '൛' => Ok(Malayalam::FractionOneTwentieth),
            '൜' => Ok(Malayalam::FractionOneTenth),
            '൝' => Ok(Malayalam::FractionThreeTwentieths),
            '൞' => Ok(Malayalam::FractionOneFifth),
            'ൟ' => Ok(Malayalam::LetterArchaicIi),
            'ൠ' => Ok(Malayalam::LetterVocalicRr),
            'ൡ' => Ok(Malayalam::LetterVocalicLl),
            'ൢ' => Ok(Malayalam::VowelSignVocalicL),
            'ൣ' => Ok(Malayalam::VowelSignVocalicLl),
            '൦' => Ok(Malayalam::DigitZero),
            '൧' => Ok(Malayalam::DigitOne),
            '൨' => Ok(Malayalam::DigitTwo),
            '൩' => Ok(Malayalam::DigitThree),
            '൪' => Ok(Malayalam::DigitFour),
            '൫' => Ok(Malayalam::DigitFive),
            '൬' => Ok(Malayalam::DigitSix),
            '൭' => Ok(Malayalam::DigitSeven),
            '൮' => Ok(Malayalam::DigitEight),
            '൯' => Ok(Malayalam::DigitNine),
            '൰' => Ok(Malayalam::NumberTen),
            '൱' => Ok(Malayalam::NumberOneHundred),
            '൲' => Ok(Malayalam::NumberOneThousand),
            '൳' => Ok(Malayalam::FractionOneQuarter),
            '൴' => Ok(Malayalam::FractionOneHalf),
            '൵' => Ok(Malayalam::FractionThreeQuarters),
            '൶' => Ok(Malayalam::FractionOneSixteenth),
            '൷' => Ok(Malayalam::FractionOneEighth),
            '൸' => Ok(Malayalam::FractionThreeSixteenths),
            '൹' => Ok(Malayalam::DateMark),
            'ൺ' => Ok(Malayalam::LetterChilluNn),
            'ൻ' => Ok(Malayalam::LetterChilluN),
            'ർ' => Ok(Malayalam::LetterChilluRr),
            'ൽ' => Ok(Malayalam::LetterChilluL),
            'ൾ' => Ok(Malayalam::LetterChilluLl),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Malayalam {
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

impl std::convert::TryFrom<u32> for Malayalam {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Malayalam {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Malayalam {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Malayalam::SignCombiningAnusvaraAbove
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Malayalam{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
