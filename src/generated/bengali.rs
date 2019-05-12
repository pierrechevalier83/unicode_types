
/// An enum to represent all characters in the Bengali block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Bengali {
    /// \u{980}: 'ঀ'
    Anji,
    /// \u{981}: 'ঁ'
    SignCandrabindu,
    /// \u{982}: 'ং'
    SignAnusvara,
    /// \u{983}: 'ঃ'
    SignVisarga,
    /// \u{985}: 'অ'
    LetterA,
    /// \u{986}: 'আ'
    LetterAa,
    /// \u{987}: 'ই'
    LetterI,
    /// \u{988}: 'ঈ'
    LetterIi,
    /// \u{989}: 'উ'
    LetterU,
    /// \u{98a}: 'ঊ'
    LetterUu,
    /// \u{98b}: 'ঋ'
    LetterVocalicR,
    /// \u{98c}: 'ঌ'
    LetterVocalicL,
    /// \u{98f}: 'এ'
    LetterE,
    /// \u{990}: 'ঐ'
    LetterAi,
    /// \u{993}: 'ও'
    LetterO,
    /// \u{994}: 'ঔ'
    LetterAu,
    /// \u{995}: 'ক'
    LetterKa,
    /// \u{996}: 'খ'
    LetterKha,
    /// \u{997}: 'গ'
    LetterGa,
    /// \u{998}: 'ঘ'
    LetterGha,
    /// \u{999}: 'ঙ'
    LetterNga,
    /// \u{99a}: 'চ'
    LetterCa,
    /// \u{99b}: 'ছ'
    LetterCha,
    /// \u{99c}: 'জ'
    LetterJa,
    /// \u{99d}: 'ঝ'
    LetterJha,
    /// \u{99e}: 'ঞ'
    LetterNya,
    /// \u{99f}: 'ট'
    LetterTta,
    /// \u{9a0}: 'ঠ'
    LetterTtha,
    /// \u{9a1}: 'ড'
    LetterDda,
    /// \u{9a2}: 'ঢ'
    LetterDdha,
    /// \u{9a3}: 'ণ'
    LetterNna,
    /// \u{9a4}: 'ত'
    LetterTa,
    /// \u{9a5}: 'থ'
    LetterTha,
    /// \u{9a6}: 'দ'
    LetterDa,
    /// \u{9a7}: 'ধ'
    LetterDha,
    /// \u{9a8}: 'ন'
    LetterNa,
    /// \u{9aa}: 'প'
    LetterPa,
    /// \u{9ab}: 'ফ'
    LetterPha,
    /// \u{9ac}: 'ব'
    LetterBa,
    /// \u{9ad}: 'ভ'
    LetterBha,
    /// \u{9ae}: 'ম'
    LetterMa,
    /// \u{9af}: 'য'
    LetterYa,
    /// \u{9b0}: 'র'
    LetterRa,
    /// \u{9b2}: 'ল'
    LetterLa,
    /// \u{9b6}: 'শ'
    LetterSha,
    /// \u{9b7}: 'ষ'
    LetterSsa,
    /// \u{9b8}: 'স'
    LetterSa,
    /// \u{9b9}: 'হ'
    LetterHa,
    /// \u{9bc}: '়'
    SignNukta,
    /// \u{9bd}: 'ঽ'
    SignAvagraha,
    /// \u{9be}: 'া'
    VowelSignAa,
    /// \u{9bf}: 'ি'
    VowelSignI,
    /// \u{9c0}: 'ী'
    VowelSignIi,
    /// \u{9c1}: 'ু'
    VowelSignU,
    /// \u{9c2}: 'ূ'
    VowelSignUu,
    /// \u{9c3}: 'ৃ'
    VowelSignVocalicR,
    /// \u{9c4}: 'ৄ'
    VowelSignVocalicRr,
    /// \u{9c7}: 'ে'
    VowelSignE,
    /// \u{9c8}: 'ৈ'
    VowelSignAi,
    /// \u{9cb}: 'ো'
    VowelSignO,
    /// \u{9cc}: 'ৌ'
    VowelSignAu,
    /// \u{9cd}: '্'
    SignVirama,
    /// \u{9ce}: 'ৎ'
    LetterKhandaTa,
    /// \u{9d7}: 'ৗ'
    AuLengthMark,
    /// \u{9dc}: 'ড়'
    LetterRra,
    /// \u{9dd}: 'ঢ়'
    LetterRha,
    /// \u{9df}: 'য়'
    LetterYya,
    /// \u{9e0}: 'ৠ'
    LetterVocalicRr,
    /// \u{9e1}: 'ৡ'
    LetterVocalicLl,
    /// \u{9e2}: 'ৢ'
    VowelSignVocalicL,
    /// \u{9e3}: 'ৣ'
    VowelSignVocalicLl,
    /// \u{9e6}: '০'
    DigitZero,
    /// \u{9e7}: '১'
    DigitOne,
    /// \u{9e8}: '২'
    DigitTwo,
    /// \u{9e9}: '৩'
    DigitThree,
    /// \u{9ea}: '৪'
    DigitFour,
    /// \u{9eb}: '৫'
    DigitFive,
    /// \u{9ec}: '৬'
    DigitSix,
    /// \u{9ed}: '৭'
    DigitSeven,
    /// \u{9ee}: '৮'
    DigitEight,
    /// \u{9ef}: '৯'
    DigitNine,
    /// \u{9f0}: 'ৰ'
    LetterRaWithMiddleDiagonal,
    /// \u{9f1}: 'ৱ'
    LetterRaWithLowerDiagonal,
    /// \u{9f2}: '৲'
    RupeeMark,
    /// \u{9f3}: '৳'
    RupeeSign,
    /// \u{9f4}: '৴'
    CurrencyNumeratorOne,
    /// \u{9f5}: '৵'
    CurrencyNumeratorTwo,
    /// \u{9f6}: '৶'
    CurrencyNumeratorThree,
    /// \u{9f7}: '৷'
    CurrencyNumeratorFour,
    /// \u{9f8}: '৸'
    CurrencyNumeratorOneLessThanTheDenominator,
    /// \u{9f9}: '৹'
    CurrencyDenominatorSixteen,
    /// \u{9fa}: '৺'
    Isshar,
    /// \u{9fb}: '৻'
    GandaMark,
    /// \u{9fc}: 'ৼ'
    LetterVedicAnusvara,
    /// \u{9fd}: '৽'
    AbbreviationSign,
    /// \u{9fe}: '৾'
    SandhiMark,
}

impl Into<char> for Bengali {
    fn into(self) -> char {
        match self {
            Bengali::Anji => 'ঀ',
            Bengali::SignCandrabindu => 'ঁ',
            Bengali::SignAnusvara => 'ং',
            Bengali::SignVisarga => 'ঃ',
            Bengali::LetterA => 'অ',
            Bengali::LetterAa => 'আ',
            Bengali::LetterI => 'ই',
            Bengali::LetterIi => 'ঈ',
            Bengali::LetterU => 'উ',
            Bengali::LetterUu => 'ঊ',
            Bengali::LetterVocalicR => 'ঋ',
            Bengali::LetterVocalicL => 'ঌ',
            Bengali::LetterE => 'এ',
            Bengali::LetterAi => 'ঐ',
            Bengali::LetterO => 'ও',
            Bengali::LetterAu => 'ঔ',
            Bengali::LetterKa => 'ক',
            Bengali::LetterKha => 'খ',
            Bengali::LetterGa => 'গ',
            Bengali::LetterGha => 'ঘ',
            Bengali::LetterNga => 'ঙ',
            Bengali::LetterCa => 'চ',
            Bengali::LetterCha => 'ছ',
            Bengali::LetterJa => 'জ',
            Bengali::LetterJha => 'ঝ',
            Bengali::LetterNya => 'ঞ',
            Bengali::LetterTta => 'ট',
            Bengali::LetterTtha => 'ঠ',
            Bengali::LetterDda => 'ড',
            Bengali::LetterDdha => 'ঢ',
            Bengali::LetterNna => 'ণ',
            Bengali::LetterTa => 'ত',
            Bengali::LetterTha => 'থ',
            Bengali::LetterDa => 'দ',
            Bengali::LetterDha => 'ধ',
            Bengali::LetterNa => 'ন',
            Bengali::LetterPa => 'প',
            Bengali::LetterPha => 'ফ',
            Bengali::LetterBa => 'ব',
            Bengali::LetterBha => 'ভ',
            Bengali::LetterMa => 'ম',
            Bengali::LetterYa => 'য',
            Bengali::LetterRa => 'র',
            Bengali::LetterLa => 'ল',
            Bengali::LetterSha => 'শ',
            Bengali::LetterSsa => 'ষ',
            Bengali::LetterSa => 'স',
            Bengali::LetterHa => 'হ',
            Bengali::SignNukta => '়',
            Bengali::SignAvagraha => 'ঽ',
            Bengali::VowelSignAa => 'া',
            Bengali::VowelSignI => 'ি',
            Bengali::VowelSignIi => 'ী',
            Bengali::VowelSignU => 'ু',
            Bengali::VowelSignUu => 'ূ',
            Bengali::VowelSignVocalicR => 'ৃ',
            Bengali::VowelSignVocalicRr => 'ৄ',
            Bengali::VowelSignE => 'ে',
            Bengali::VowelSignAi => 'ৈ',
            Bengali::VowelSignO => 'ো',
            Bengali::VowelSignAu => 'ৌ',
            Bengali::SignVirama => '্',
            Bengali::LetterKhandaTa => 'ৎ',
            Bengali::AuLengthMark => 'ৗ',
            Bengali::LetterRra => 'ড়',
            Bengali::LetterRha => 'ঢ়',
            Bengali::LetterYya => 'য়',
            Bengali::LetterVocalicRr => 'ৠ',
            Bengali::LetterVocalicLl => 'ৡ',
            Bengali::VowelSignVocalicL => 'ৢ',
            Bengali::VowelSignVocalicLl => 'ৣ',
            Bengali::DigitZero => '০',
            Bengali::DigitOne => '১',
            Bengali::DigitTwo => '২',
            Bengali::DigitThree => '৩',
            Bengali::DigitFour => '৪',
            Bengali::DigitFive => '৫',
            Bengali::DigitSix => '৬',
            Bengali::DigitSeven => '৭',
            Bengali::DigitEight => '৮',
            Bengali::DigitNine => '৯',
            Bengali::LetterRaWithMiddleDiagonal => 'ৰ',
            Bengali::LetterRaWithLowerDiagonal => 'ৱ',
            Bengali::RupeeMark => '৲',
            Bengali::RupeeSign => '৳',
            Bengali::CurrencyNumeratorOne => '৴',
            Bengali::CurrencyNumeratorTwo => '৵',
            Bengali::CurrencyNumeratorThree => '৶',
            Bengali::CurrencyNumeratorFour => '৷',
            Bengali::CurrencyNumeratorOneLessThanTheDenominator => '৸',
            Bengali::CurrencyDenominatorSixteen => '৹',
            Bengali::Isshar => '৺',
            Bengali::GandaMark => '৻',
            Bengali::LetterVedicAnusvara => 'ৼ',
            Bengali::AbbreviationSign => '৽',
            Bengali::SandhiMark => '৾',
        }
    }
}

impl std::convert::TryFrom<char> for Bengali {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ঀ' => Ok(Bengali::Anji),
            'ঁ' => Ok(Bengali::SignCandrabindu),
            'ং' => Ok(Bengali::SignAnusvara),
            'ঃ' => Ok(Bengali::SignVisarga),
            'অ' => Ok(Bengali::LetterA),
            'আ' => Ok(Bengali::LetterAa),
            'ই' => Ok(Bengali::LetterI),
            'ঈ' => Ok(Bengali::LetterIi),
            'উ' => Ok(Bengali::LetterU),
            'ঊ' => Ok(Bengali::LetterUu),
            'ঋ' => Ok(Bengali::LetterVocalicR),
            'ঌ' => Ok(Bengali::LetterVocalicL),
            'এ' => Ok(Bengali::LetterE),
            'ঐ' => Ok(Bengali::LetterAi),
            'ও' => Ok(Bengali::LetterO),
            'ঔ' => Ok(Bengali::LetterAu),
            'ক' => Ok(Bengali::LetterKa),
            'খ' => Ok(Bengali::LetterKha),
            'গ' => Ok(Bengali::LetterGa),
            'ঘ' => Ok(Bengali::LetterGha),
            'ঙ' => Ok(Bengali::LetterNga),
            'চ' => Ok(Bengali::LetterCa),
            'ছ' => Ok(Bengali::LetterCha),
            'জ' => Ok(Bengali::LetterJa),
            'ঝ' => Ok(Bengali::LetterJha),
            'ঞ' => Ok(Bengali::LetterNya),
            'ট' => Ok(Bengali::LetterTta),
            'ঠ' => Ok(Bengali::LetterTtha),
            'ড' => Ok(Bengali::LetterDda),
            'ঢ' => Ok(Bengali::LetterDdha),
            'ণ' => Ok(Bengali::LetterNna),
            'ত' => Ok(Bengali::LetterTa),
            'থ' => Ok(Bengali::LetterTha),
            'দ' => Ok(Bengali::LetterDa),
            'ধ' => Ok(Bengali::LetterDha),
            'ন' => Ok(Bengali::LetterNa),
            'প' => Ok(Bengali::LetterPa),
            'ফ' => Ok(Bengali::LetterPha),
            'ব' => Ok(Bengali::LetterBa),
            'ভ' => Ok(Bengali::LetterBha),
            'ম' => Ok(Bengali::LetterMa),
            'য' => Ok(Bengali::LetterYa),
            'র' => Ok(Bengali::LetterRa),
            'ল' => Ok(Bengali::LetterLa),
            'শ' => Ok(Bengali::LetterSha),
            'ষ' => Ok(Bengali::LetterSsa),
            'স' => Ok(Bengali::LetterSa),
            'হ' => Ok(Bengali::LetterHa),
            '়' => Ok(Bengali::SignNukta),
            'ঽ' => Ok(Bengali::SignAvagraha),
            'া' => Ok(Bengali::VowelSignAa),
            'ি' => Ok(Bengali::VowelSignI),
            'ী' => Ok(Bengali::VowelSignIi),
            'ু' => Ok(Bengali::VowelSignU),
            'ূ' => Ok(Bengali::VowelSignUu),
            'ৃ' => Ok(Bengali::VowelSignVocalicR),
            'ৄ' => Ok(Bengali::VowelSignVocalicRr),
            'ে' => Ok(Bengali::VowelSignE),
            'ৈ' => Ok(Bengali::VowelSignAi),
            'ো' => Ok(Bengali::VowelSignO),
            'ৌ' => Ok(Bengali::VowelSignAu),
            '্' => Ok(Bengali::SignVirama),
            'ৎ' => Ok(Bengali::LetterKhandaTa),
            'ৗ' => Ok(Bengali::AuLengthMark),
            'ড়' => Ok(Bengali::LetterRra),
            'ঢ়' => Ok(Bengali::LetterRha),
            'য়' => Ok(Bengali::LetterYya),
            'ৠ' => Ok(Bengali::LetterVocalicRr),
            'ৡ' => Ok(Bengali::LetterVocalicLl),
            'ৢ' => Ok(Bengali::VowelSignVocalicL),
            'ৣ' => Ok(Bengali::VowelSignVocalicLl),
            '০' => Ok(Bengali::DigitZero),
            '১' => Ok(Bengali::DigitOne),
            '২' => Ok(Bengali::DigitTwo),
            '৩' => Ok(Bengali::DigitThree),
            '৪' => Ok(Bengali::DigitFour),
            '৫' => Ok(Bengali::DigitFive),
            '৬' => Ok(Bengali::DigitSix),
            '৭' => Ok(Bengali::DigitSeven),
            '৮' => Ok(Bengali::DigitEight),
            '৯' => Ok(Bengali::DigitNine),
            'ৰ' => Ok(Bengali::LetterRaWithMiddleDiagonal),
            'ৱ' => Ok(Bengali::LetterRaWithLowerDiagonal),
            '৲' => Ok(Bengali::RupeeMark),
            '৳' => Ok(Bengali::RupeeSign),
            '৴' => Ok(Bengali::CurrencyNumeratorOne),
            '৵' => Ok(Bengali::CurrencyNumeratorTwo),
            '৶' => Ok(Bengali::CurrencyNumeratorThree),
            '৷' => Ok(Bengali::CurrencyNumeratorFour),
            '৸' => Ok(Bengali::CurrencyNumeratorOneLessThanTheDenominator),
            '৹' => Ok(Bengali::CurrencyDenominatorSixteen),
            '৺' => Ok(Bengali::Isshar),
            '৻' => Ok(Bengali::GandaMark),
            'ৼ' => Ok(Bengali::LetterVedicAnusvara),
            '৽' => Ok(Bengali::AbbreviationSign),
            '৾' => Ok(Bengali::SandhiMark),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Bengali {
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

impl std::convert::TryFrom<u32> for Bengali {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Bengali {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Bengali {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Bengali::Anji
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Bengali::Anji => "bengali anji",
            Bengali::SignCandrabindu => "bengali sign candrabindu",
            Bengali::SignAnusvara => "bengali sign anusvara",
            Bengali::SignVisarga => "bengali sign visarga",
            Bengali::LetterA => "bengali letter a",
            Bengali::LetterAa => "bengali letter aa",
            Bengali::LetterI => "bengali letter i",
            Bengali::LetterIi => "bengali letter ii",
            Bengali::LetterU => "bengali letter u",
            Bengali::LetterUu => "bengali letter uu",
            Bengali::LetterVocalicR => "bengali letter vocalic r",
            Bengali::LetterVocalicL => "bengali letter vocalic l",
            Bengali::LetterE => "bengali letter e",
            Bengali::LetterAi => "bengali letter ai",
            Bengali::LetterO => "bengali letter o",
            Bengali::LetterAu => "bengali letter au",
            Bengali::LetterKa => "bengali letter ka",
            Bengali::LetterKha => "bengali letter kha",
            Bengali::LetterGa => "bengali letter ga",
            Bengali::LetterGha => "bengali letter gha",
            Bengali::LetterNga => "bengali letter nga",
            Bengali::LetterCa => "bengali letter ca",
            Bengali::LetterCha => "bengali letter cha",
            Bengali::LetterJa => "bengali letter ja",
            Bengali::LetterJha => "bengali letter jha",
            Bengali::LetterNya => "bengali letter nya",
            Bengali::LetterTta => "bengali letter tta",
            Bengali::LetterTtha => "bengali letter ttha",
            Bengali::LetterDda => "bengali letter dda",
            Bengali::LetterDdha => "bengali letter ddha",
            Bengali::LetterNna => "bengali letter nna",
            Bengali::LetterTa => "bengali letter ta",
            Bengali::LetterTha => "bengali letter tha",
            Bengali::LetterDa => "bengali letter da",
            Bengali::LetterDha => "bengali letter dha",
            Bengali::LetterNa => "bengali letter na",
            Bengali::LetterPa => "bengali letter pa",
            Bengali::LetterPha => "bengali letter pha",
            Bengali::LetterBa => "bengali letter ba",
            Bengali::LetterBha => "bengali letter bha",
            Bengali::LetterMa => "bengali letter ma",
            Bengali::LetterYa => "bengali letter ya",
            Bengali::LetterRa => "bengali letter ra",
            Bengali::LetterLa => "bengali letter la",
            Bengali::LetterSha => "bengali letter sha",
            Bengali::LetterSsa => "bengali letter ssa",
            Bengali::LetterSa => "bengali letter sa",
            Bengali::LetterHa => "bengali letter ha",
            Bengali::SignNukta => "bengali sign nukta",
            Bengali::SignAvagraha => "bengali sign avagraha",
            Bengali::VowelSignAa => "bengali vowel sign aa",
            Bengali::VowelSignI => "bengali vowel sign i",
            Bengali::VowelSignIi => "bengali vowel sign ii",
            Bengali::VowelSignU => "bengali vowel sign u",
            Bengali::VowelSignUu => "bengali vowel sign uu",
            Bengali::VowelSignVocalicR => "bengali vowel sign vocalic r",
            Bengali::VowelSignVocalicRr => "bengali vowel sign vocalic rr",
            Bengali::VowelSignE => "bengali vowel sign e",
            Bengali::VowelSignAi => "bengali vowel sign ai",
            Bengali::VowelSignO => "bengali vowel sign o",
            Bengali::VowelSignAu => "bengali vowel sign au",
            Bengali::SignVirama => "bengali sign virama",
            Bengali::LetterKhandaTa => "bengali letter khanda ta",
            Bengali::AuLengthMark => "bengali au length mark",
            Bengali::LetterRra => "bengali letter rra",
            Bengali::LetterRha => "bengali letter rha",
            Bengali::LetterYya => "bengali letter yya",
            Bengali::LetterVocalicRr => "bengali letter vocalic rr",
            Bengali::LetterVocalicLl => "bengali letter vocalic ll",
            Bengali::VowelSignVocalicL => "bengali vowel sign vocalic l",
            Bengali::VowelSignVocalicLl => "bengali vowel sign vocalic ll",
            Bengali::DigitZero => "bengali digit zero",
            Bengali::DigitOne => "bengali digit one",
            Bengali::DigitTwo => "bengali digit two",
            Bengali::DigitThree => "bengali digit three",
            Bengali::DigitFour => "bengali digit four",
            Bengali::DigitFive => "bengali digit five",
            Bengali::DigitSix => "bengali digit six",
            Bengali::DigitSeven => "bengali digit seven",
            Bengali::DigitEight => "bengali digit eight",
            Bengali::DigitNine => "bengali digit nine",
            Bengali::LetterRaWithMiddleDiagonal => "bengali letter ra with middle diagonal",
            Bengali::LetterRaWithLowerDiagonal => "bengali letter ra with lower diagonal",
            Bengali::RupeeMark => "bengali rupee mark",
            Bengali::RupeeSign => "bengali rupee sign",
            Bengali::CurrencyNumeratorOne => "bengali currency numerator one",
            Bengali::CurrencyNumeratorTwo => "bengali currency numerator two",
            Bengali::CurrencyNumeratorThree => "bengali currency numerator three",
            Bengali::CurrencyNumeratorFour => "bengali currency numerator four",
            Bengali::CurrencyNumeratorOneLessThanTheDenominator => "bengali currency numerator one less than the denominator",
            Bengali::CurrencyDenominatorSixteen => "bengali currency denominator sixteen",
            Bengali::Isshar => "bengali isshar",
            Bengali::GandaMark => "bengali ganda mark",
            Bengali::LetterVedicAnusvara => "bengali letter vedic anusvara",
            Bengali::AbbreviationSign => "bengali abbreviation sign",
            Bengali::SandhiMark => "bengali sandhi mark",
        }
    }
}
