
/// An enum to represent all characters in the Saurashtra block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Saurashtra {
    /// \u{a880}: 'ꢀ'
    SignAnusvara,
    /// \u{a881}: 'ꢁ'
    SignVisarga,
    /// \u{a882}: 'ꢂ'
    LetterA,
    /// \u{a883}: 'ꢃ'
    LetterAa,
    /// \u{a884}: 'ꢄ'
    LetterI,
    /// \u{a885}: 'ꢅ'
    LetterIi,
    /// \u{a886}: 'ꢆ'
    LetterU,
    /// \u{a887}: 'ꢇ'
    LetterUu,
    /// \u{a888}: 'ꢈ'
    LetterVocalicR,
    /// \u{a889}: 'ꢉ'
    LetterVocalicRr,
    /// \u{a88a}: 'ꢊ'
    LetterVocalicL,
    /// \u{a88b}: 'ꢋ'
    LetterVocalicLl,
    /// \u{a88c}: 'ꢌ'
    LetterE,
    /// \u{a88d}: 'ꢍ'
    LetterEe,
    /// \u{a88e}: 'ꢎ'
    LetterAi,
    /// \u{a88f}: 'ꢏ'
    LetterO,
    /// \u{a890}: 'ꢐ'
    LetterOo,
    /// \u{a891}: 'ꢑ'
    LetterAu,
    /// \u{a892}: 'ꢒ'
    LetterKa,
    /// \u{a893}: 'ꢓ'
    LetterKha,
    /// \u{a894}: 'ꢔ'
    LetterGa,
    /// \u{a895}: 'ꢕ'
    LetterGha,
    /// \u{a896}: 'ꢖ'
    LetterNga,
    /// \u{a897}: 'ꢗ'
    LetterCa,
    /// \u{a898}: 'ꢘ'
    LetterCha,
    /// \u{a899}: 'ꢙ'
    LetterJa,
    /// \u{a89a}: 'ꢚ'
    LetterJha,
    /// \u{a89b}: 'ꢛ'
    LetterNya,
    /// \u{a89c}: 'ꢜ'
    LetterTta,
    /// \u{a89d}: 'ꢝ'
    LetterTtha,
    /// \u{a89e}: 'ꢞ'
    LetterDda,
    /// \u{a89f}: 'ꢟ'
    LetterDdha,
    /// \u{a8a0}: 'ꢠ'
    LetterNna,
    /// \u{a8a1}: 'ꢡ'
    LetterTa,
    /// \u{a8a2}: 'ꢢ'
    LetterTha,
    /// \u{a8a3}: 'ꢣ'
    LetterDa,
    /// \u{a8a4}: 'ꢤ'
    LetterDha,
    /// \u{a8a5}: 'ꢥ'
    LetterNa,
    /// \u{a8a6}: 'ꢦ'
    LetterPa,
    /// \u{a8a7}: 'ꢧ'
    LetterPha,
    /// \u{a8a8}: 'ꢨ'
    LetterBa,
    /// \u{a8a9}: 'ꢩ'
    LetterBha,
    /// \u{a8aa}: 'ꢪ'
    LetterMa,
    /// \u{a8ab}: 'ꢫ'
    LetterYa,
    /// \u{a8ac}: 'ꢬ'
    LetterRa,
    /// \u{a8ad}: 'ꢭ'
    LetterLa,
    /// \u{a8ae}: 'ꢮ'
    LetterVa,
    /// \u{a8af}: 'ꢯ'
    LetterSha,
    /// \u{a8b0}: 'ꢰ'
    LetterSsa,
    /// \u{a8b1}: 'ꢱ'
    LetterSa,
    /// \u{a8b2}: 'ꢲ'
    LetterHa,
    /// \u{a8b3}: 'ꢳ'
    LetterLla,
    /// \u{a8b4}: 'ꢴ'
    ConsonantSignHaaru,
    /// \u{a8b5}: 'ꢵ'
    VowelSignAa,
    /// \u{a8b6}: 'ꢶ'
    VowelSignI,
    /// \u{a8b7}: 'ꢷ'
    VowelSignIi,
    /// \u{a8b8}: 'ꢸ'
    VowelSignU,
    /// \u{a8b9}: 'ꢹ'
    VowelSignUu,
    /// \u{a8ba}: 'ꢺ'
    VowelSignVocalicR,
    /// \u{a8bb}: 'ꢻ'
    VowelSignVocalicRr,
    /// \u{a8bc}: 'ꢼ'
    VowelSignVocalicL,
    /// \u{a8bd}: 'ꢽ'
    VowelSignVocalicLl,
    /// \u{a8be}: 'ꢾ'
    VowelSignE,
    /// \u{a8bf}: 'ꢿ'
    VowelSignEe,
    /// \u{a8c0}: 'ꣀ'
    VowelSignAi,
    /// \u{a8c1}: 'ꣁ'
    VowelSignO,
    /// \u{a8c2}: 'ꣂ'
    VowelSignOo,
    /// \u{a8c3}: 'ꣃ'
    VowelSignAu,
    /// \u{a8c4}: '꣄'
    SignVirama,
    /// \u{a8c5}: 'ꣅ'
    SignCandrabindu,
    /// \u{a8ce}: '꣎'
    Danda,
    /// \u{a8cf}: '꣏'
    DoubleDanda,
    /// \u{a8d0}: '꣐'
    DigitZero,
    /// \u{a8d1}: '꣑'
    DigitOne,
    /// \u{a8d2}: '꣒'
    DigitTwo,
    /// \u{a8d3}: '꣓'
    DigitThree,
    /// \u{a8d4}: '꣔'
    DigitFour,
    /// \u{a8d5}: '꣕'
    DigitFive,
    /// \u{a8d6}: '꣖'
    DigitSix,
    /// \u{a8d7}: '꣗'
    DigitSeven,
    /// \u{a8d8}: '꣘'
    DigitEight,
    /// \u{a8d9}: '꣙'
    DigitNine,
}

impl Into<char> for Saurashtra {
    fn into(self) -> char {
        match self {
            Saurashtra::SignAnusvara => 'ꢀ',
            Saurashtra::SignVisarga => 'ꢁ',
            Saurashtra::LetterA => 'ꢂ',
            Saurashtra::LetterAa => 'ꢃ',
            Saurashtra::LetterI => 'ꢄ',
            Saurashtra::LetterIi => 'ꢅ',
            Saurashtra::LetterU => 'ꢆ',
            Saurashtra::LetterUu => 'ꢇ',
            Saurashtra::LetterVocalicR => 'ꢈ',
            Saurashtra::LetterVocalicRr => 'ꢉ',
            Saurashtra::LetterVocalicL => 'ꢊ',
            Saurashtra::LetterVocalicLl => 'ꢋ',
            Saurashtra::LetterE => 'ꢌ',
            Saurashtra::LetterEe => 'ꢍ',
            Saurashtra::LetterAi => 'ꢎ',
            Saurashtra::LetterO => 'ꢏ',
            Saurashtra::LetterOo => 'ꢐ',
            Saurashtra::LetterAu => 'ꢑ',
            Saurashtra::LetterKa => 'ꢒ',
            Saurashtra::LetterKha => 'ꢓ',
            Saurashtra::LetterGa => 'ꢔ',
            Saurashtra::LetterGha => 'ꢕ',
            Saurashtra::LetterNga => 'ꢖ',
            Saurashtra::LetterCa => 'ꢗ',
            Saurashtra::LetterCha => 'ꢘ',
            Saurashtra::LetterJa => 'ꢙ',
            Saurashtra::LetterJha => 'ꢚ',
            Saurashtra::LetterNya => 'ꢛ',
            Saurashtra::LetterTta => 'ꢜ',
            Saurashtra::LetterTtha => 'ꢝ',
            Saurashtra::LetterDda => 'ꢞ',
            Saurashtra::LetterDdha => 'ꢟ',
            Saurashtra::LetterNna => 'ꢠ',
            Saurashtra::LetterTa => 'ꢡ',
            Saurashtra::LetterTha => 'ꢢ',
            Saurashtra::LetterDa => 'ꢣ',
            Saurashtra::LetterDha => 'ꢤ',
            Saurashtra::LetterNa => 'ꢥ',
            Saurashtra::LetterPa => 'ꢦ',
            Saurashtra::LetterPha => 'ꢧ',
            Saurashtra::LetterBa => 'ꢨ',
            Saurashtra::LetterBha => 'ꢩ',
            Saurashtra::LetterMa => 'ꢪ',
            Saurashtra::LetterYa => 'ꢫ',
            Saurashtra::LetterRa => 'ꢬ',
            Saurashtra::LetterLa => 'ꢭ',
            Saurashtra::LetterVa => 'ꢮ',
            Saurashtra::LetterSha => 'ꢯ',
            Saurashtra::LetterSsa => 'ꢰ',
            Saurashtra::LetterSa => 'ꢱ',
            Saurashtra::LetterHa => 'ꢲ',
            Saurashtra::LetterLla => 'ꢳ',
            Saurashtra::ConsonantSignHaaru => 'ꢴ',
            Saurashtra::VowelSignAa => 'ꢵ',
            Saurashtra::VowelSignI => 'ꢶ',
            Saurashtra::VowelSignIi => 'ꢷ',
            Saurashtra::VowelSignU => 'ꢸ',
            Saurashtra::VowelSignUu => 'ꢹ',
            Saurashtra::VowelSignVocalicR => 'ꢺ',
            Saurashtra::VowelSignVocalicRr => 'ꢻ',
            Saurashtra::VowelSignVocalicL => 'ꢼ',
            Saurashtra::VowelSignVocalicLl => 'ꢽ',
            Saurashtra::VowelSignE => 'ꢾ',
            Saurashtra::VowelSignEe => 'ꢿ',
            Saurashtra::VowelSignAi => 'ꣀ',
            Saurashtra::VowelSignO => 'ꣁ',
            Saurashtra::VowelSignOo => 'ꣂ',
            Saurashtra::VowelSignAu => 'ꣃ',
            Saurashtra::SignVirama => '꣄',
            Saurashtra::SignCandrabindu => 'ꣅ',
            Saurashtra::Danda => '꣎',
            Saurashtra::DoubleDanda => '꣏',
            Saurashtra::DigitZero => '꣐',
            Saurashtra::DigitOne => '꣑',
            Saurashtra::DigitTwo => '꣒',
            Saurashtra::DigitThree => '꣓',
            Saurashtra::DigitFour => '꣔',
            Saurashtra::DigitFive => '꣕',
            Saurashtra::DigitSix => '꣖',
            Saurashtra::DigitSeven => '꣗',
            Saurashtra::DigitEight => '꣘',
            Saurashtra::DigitNine => '꣙',
        }
    }
}

impl std::convert::TryFrom<char> for Saurashtra {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ꢀ' => Ok(Saurashtra::SignAnusvara),
            'ꢁ' => Ok(Saurashtra::SignVisarga),
            'ꢂ' => Ok(Saurashtra::LetterA),
            'ꢃ' => Ok(Saurashtra::LetterAa),
            'ꢄ' => Ok(Saurashtra::LetterI),
            'ꢅ' => Ok(Saurashtra::LetterIi),
            'ꢆ' => Ok(Saurashtra::LetterU),
            'ꢇ' => Ok(Saurashtra::LetterUu),
            'ꢈ' => Ok(Saurashtra::LetterVocalicR),
            'ꢉ' => Ok(Saurashtra::LetterVocalicRr),
            'ꢊ' => Ok(Saurashtra::LetterVocalicL),
            'ꢋ' => Ok(Saurashtra::LetterVocalicLl),
            'ꢌ' => Ok(Saurashtra::LetterE),
            'ꢍ' => Ok(Saurashtra::LetterEe),
            'ꢎ' => Ok(Saurashtra::LetterAi),
            'ꢏ' => Ok(Saurashtra::LetterO),
            'ꢐ' => Ok(Saurashtra::LetterOo),
            'ꢑ' => Ok(Saurashtra::LetterAu),
            'ꢒ' => Ok(Saurashtra::LetterKa),
            'ꢓ' => Ok(Saurashtra::LetterKha),
            'ꢔ' => Ok(Saurashtra::LetterGa),
            'ꢕ' => Ok(Saurashtra::LetterGha),
            'ꢖ' => Ok(Saurashtra::LetterNga),
            'ꢗ' => Ok(Saurashtra::LetterCa),
            'ꢘ' => Ok(Saurashtra::LetterCha),
            'ꢙ' => Ok(Saurashtra::LetterJa),
            'ꢚ' => Ok(Saurashtra::LetterJha),
            'ꢛ' => Ok(Saurashtra::LetterNya),
            'ꢜ' => Ok(Saurashtra::LetterTta),
            'ꢝ' => Ok(Saurashtra::LetterTtha),
            'ꢞ' => Ok(Saurashtra::LetterDda),
            'ꢟ' => Ok(Saurashtra::LetterDdha),
            'ꢠ' => Ok(Saurashtra::LetterNna),
            'ꢡ' => Ok(Saurashtra::LetterTa),
            'ꢢ' => Ok(Saurashtra::LetterTha),
            'ꢣ' => Ok(Saurashtra::LetterDa),
            'ꢤ' => Ok(Saurashtra::LetterDha),
            'ꢥ' => Ok(Saurashtra::LetterNa),
            'ꢦ' => Ok(Saurashtra::LetterPa),
            'ꢧ' => Ok(Saurashtra::LetterPha),
            'ꢨ' => Ok(Saurashtra::LetterBa),
            'ꢩ' => Ok(Saurashtra::LetterBha),
            'ꢪ' => Ok(Saurashtra::LetterMa),
            'ꢫ' => Ok(Saurashtra::LetterYa),
            'ꢬ' => Ok(Saurashtra::LetterRa),
            'ꢭ' => Ok(Saurashtra::LetterLa),
            'ꢮ' => Ok(Saurashtra::LetterVa),
            'ꢯ' => Ok(Saurashtra::LetterSha),
            'ꢰ' => Ok(Saurashtra::LetterSsa),
            'ꢱ' => Ok(Saurashtra::LetterSa),
            'ꢲ' => Ok(Saurashtra::LetterHa),
            'ꢳ' => Ok(Saurashtra::LetterLla),
            'ꢴ' => Ok(Saurashtra::ConsonantSignHaaru),
            'ꢵ' => Ok(Saurashtra::VowelSignAa),
            'ꢶ' => Ok(Saurashtra::VowelSignI),
            'ꢷ' => Ok(Saurashtra::VowelSignIi),
            'ꢸ' => Ok(Saurashtra::VowelSignU),
            'ꢹ' => Ok(Saurashtra::VowelSignUu),
            'ꢺ' => Ok(Saurashtra::VowelSignVocalicR),
            'ꢻ' => Ok(Saurashtra::VowelSignVocalicRr),
            'ꢼ' => Ok(Saurashtra::VowelSignVocalicL),
            'ꢽ' => Ok(Saurashtra::VowelSignVocalicLl),
            'ꢾ' => Ok(Saurashtra::VowelSignE),
            'ꢿ' => Ok(Saurashtra::VowelSignEe),
            'ꣀ' => Ok(Saurashtra::VowelSignAi),
            'ꣁ' => Ok(Saurashtra::VowelSignO),
            'ꣂ' => Ok(Saurashtra::VowelSignOo),
            'ꣃ' => Ok(Saurashtra::VowelSignAu),
            '꣄' => Ok(Saurashtra::SignVirama),
            'ꣅ' => Ok(Saurashtra::SignCandrabindu),
            '꣎' => Ok(Saurashtra::Danda),
            '꣏' => Ok(Saurashtra::DoubleDanda),
            '꣐' => Ok(Saurashtra::DigitZero),
            '꣑' => Ok(Saurashtra::DigitOne),
            '꣒' => Ok(Saurashtra::DigitTwo),
            '꣓' => Ok(Saurashtra::DigitThree),
            '꣔' => Ok(Saurashtra::DigitFour),
            '꣕' => Ok(Saurashtra::DigitFive),
            '꣖' => Ok(Saurashtra::DigitSix),
            '꣗' => Ok(Saurashtra::DigitSeven),
            '꣘' => Ok(Saurashtra::DigitEight),
            '꣙' => Ok(Saurashtra::DigitNine),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Saurashtra {
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

impl std::convert::TryFrom<u32> for Saurashtra {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Saurashtra {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Saurashtra {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Saurashtra::SignAnusvara
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Saurashtra::SignAnusvara => "saurashtra sign anusvara",
            Saurashtra::SignVisarga => "saurashtra sign visarga",
            Saurashtra::LetterA => "saurashtra letter a",
            Saurashtra::LetterAa => "saurashtra letter aa",
            Saurashtra::LetterI => "saurashtra letter i",
            Saurashtra::LetterIi => "saurashtra letter ii",
            Saurashtra::LetterU => "saurashtra letter u",
            Saurashtra::LetterUu => "saurashtra letter uu",
            Saurashtra::LetterVocalicR => "saurashtra letter vocalic r",
            Saurashtra::LetterVocalicRr => "saurashtra letter vocalic rr",
            Saurashtra::LetterVocalicL => "saurashtra letter vocalic l",
            Saurashtra::LetterVocalicLl => "saurashtra letter vocalic ll",
            Saurashtra::LetterE => "saurashtra letter e",
            Saurashtra::LetterEe => "saurashtra letter ee",
            Saurashtra::LetterAi => "saurashtra letter ai",
            Saurashtra::LetterO => "saurashtra letter o",
            Saurashtra::LetterOo => "saurashtra letter oo",
            Saurashtra::LetterAu => "saurashtra letter au",
            Saurashtra::LetterKa => "saurashtra letter ka",
            Saurashtra::LetterKha => "saurashtra letter kha",
            Saurashtra::LetterGa => "saurashtra letter ga",
            Saurashtra::LetterGha => "saurashtra letter gha",
            Saurashtra::LetterNga => "saurashtra letter nga",
            Saurashtra::LetterCa => "saurashtra letter ca",
            Saurashtra::LetterCha => "saurashtra letter cha",
            Saurashtra::LetterJa => "saurashtra letter ja",
            Saurashtra::LetterJha => "saurashtra letter jha",
            Saurashtra::LetterNya => "saurashtra letter nya",
            Saurashtra::LetterTta => "saurashtra letter tta",
            Saurashtra::LetterTtha => "saurashtra letter ttha",
            Saurashtra::LetterDda => "saurashtra letter dda",
            Saurashtra::LetterDdha => "saurashtra letter ddha",
            Saurashtra::LetterNna => "saurashtra letter nna",
            Saurashtra::LetterTa => "saurashtra letter ta",
            Saurashtra::LetterTha => "saurashtra letter tha",
            Saurashtra::LetterDa => "saurashtra letter da",
            Saurashtra::LetterDha => "saurashtra letter dha",
            Saurashtra::LetterNa => "saurashtra letter na",
            Saurashtra::LetterPa => "saurashtra letter pa",
            Saurashtra::LetterPha => "saurashtra letter pha",
            Saurashtra::LetterBa => "saurashtra letter ba",
            Saurashtra::LetterBha => "saurashtra letter bha",
            Saurashtra::LetterMa => "saurashtra letter ma",
            Saurashtra::LetterYa => "saurashtra letter ya",
            Saurashtra::LetterRa => "saurashtra letter ra",
            Saurashtra::LetterLa => "saurashtra letter la",
            Saurashtra::LetterVa => "saurashtra letter va",
            Saurashtra::LetterSha => "saurashtra letter sha",
            Saurashtra::LetterSsa => "saurashtra letter ssa",
            Saurashtra::LetterSa => "saurashtra letter sa",
            Saurashtra::LetterHa => "saurashtra letter ha",
            Saurashtra::LetterLla => "saurashtra letter lla",
            Saurashtra::ConsonantSignHaaru => "saurashtra consonant sign haaru",
            Saurashtra::VowelSignAa => "saurashtra vowel sign aa",
            Saurashtra::VowelSignI => "saurashtra vowel sign i",
            Saurashtra::VowelSignIi => "saurashtra vowel sign ii",
            Saurashtra::VowelSignU => "saurashtra vowel sign u",
            Saurashtra::VowelSignUu => "saurashtra vowel sign uu",
            Saurashtra::VowelSignVocalicR => "saurashtra vowel sign vocalic r",
            Saurashtra::VowelSignVocalicRr => "saurashtra vowel sign vocalic rr",
            Saurashtra::VowelSignVocalicL => "saurashtra vowel sign vocalic l",
            Saurashtra::VowelSignVocalicLl => "saurashtra vowel sign vocalic ll",
            Saurashtra::VowelSignE => "saurashtra vowel sign e",
            Saurashtra::VowelSignEe => "saurashtra vowel sign ee",
            Saurashtra::VowelSignAi => "saurashtra vowel sign ai",
            Saurashtra::VowelSignO => "saurashtra vowel sign o",
            Saurashtra::VowelSignOo => "saurashtra vowel sign oo",
            Saurashtra::VowelSignAu => "saurashtra vowel sign au",
            Saurashtra::SignVirama => "saurashtra sign virama",
            Saurashtra::SignCandrabindu => "saurashtra sign candrabindu",
            Saurashtra::Danda => "saurashtra danda",
            Saurashtra::DoubleDanda => "saurashtra double danda",
            Saurashtra::DigitZero => "saurashtra digit zero",
            Saurashtra::DigitOne => "saurashtra digit one",
            Saurashtra::DigitTwo => "saurashtra digit two",
            Saurashtra::DigitThree => "saurashtra digit three",
            Saurashtra::DigitFour => "saurashtra digit four",
            Saurashtra::DigitFive => "saurashtra digit five",
            Saurashtra::DigitSix => "saurashtra digit six",
            Saurashtra::DigitSeven => "saurashtra digit seven",
            Saurashtra::DigitEight => "saurashtra digit eight",
            Saurashtra::DigitNine => "saurashtra digit nine",
        }
    }
}
