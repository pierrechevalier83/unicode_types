
/// An enum to represent all characters in the Kannada block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Kannada {
    /// \u{c80}: 'ಀ'
    SignSpacingCandrabindu,
    /// \u{c81}: 'ಁ'
    SignCandrabindu,
    /// \u{c82}: 'ಂ'
    SignAnusvara,
    /// \u{c83}: 'ಃ'
    SignVisarga,
    /// \u{c84}: '಄'
    SignSiddham,
    /// \u{c85}: 'ಅ'
    LetterA,
    /// \u{c86}: 'ಆ'
    LetterAa,
    /// \u{c87}: 'ಇ'
    LetterI,
    /// \u{c88}: 'ಈ'
    LetterIi,
    /// \u{c89}: 'ಉ'
    LetterU,
    /// \u{c8a}: 'ಊ'
    LetterUu,
    /// \u{c8b}: 'ಋ'
    LetterVocalicR,
    /// \u{c8c}: 'ಌ'
    LetterVocalicL,
    /// \u{c8e}: 'ಎ'
    LetterE,
    /// \u{c8f}: 'ಏ'
    LetterEe,
    /// \u{c90}: 'ಐ'
    LetterAi,
    /// \u{c92}: 'ಒ'
    LetterO,
    /// \u{c93}: 'ಓ'
    LetterOo,
    /// \u{c94}: 'ಔ'
    LetterAu,
    /// \u{c95}: 'ಕ'
    LetterKa,
    /// \u{c96}: 'ಖ'
    LetterKha,
    /// \u{c97}: 'ಗ'
    LetterGa,
    /// \u{c98}: 'ಘ'
    LetterGha,
    /// \u{c99}: 'ಙ'
    LetterNga,
    /// \u{c9a}: 'ಚ'
    LetterCa,
    /// \u{c9b}: 'ಛ'
    LetterCha,
    /// \u{c9c}: 'ಜ'
    LetterJa,
    /// \u{c9d}: 'ಝ'
    LetterJha,
    /// \u{c9e}: 'ಞ'
    LetterNya,
    /// \u{c9f}: 'ಟ'
    LetterTta,
    /// \u{ca0}: 'ಠ'
    LetterTtha,
    /// \u{ca1}: 'ಡ'
    LetterDda,
    /// \u{ca2}: 'ಢ'
    LetterDdha,
    /// \u{ca3}: 'ಣ'
    LetterNna,
    /// \u{ca4}: 'ತ'
    LetterTa,
    /// \u{ca5}: 'ಥ'
    LetterTha,
    /// \u{ca6}: 'ದ'
    LetterDa,
    /// \u{ca7}: 'ಧ'
    LetterDha,
    /// \u{ca8}: 'ನ'
    LetterNa,
    /// \u{caa}: 'ಪ'
    LetterPa,
    /// \u{cab}: 'ಫ'
    LetterPha,
    /// \u{cac}: 'ಬ'
    LetterBa,
    /// \u{cad}: 'ಭ'
    LetterBha,
    /// \u{cae}: 'ಮ'
    LetterMa,
    /// \u{caf}: 'ಯ'
    LetterYa,
    /// \u{cb0}: 'ರ'
    LetterRa,
    /// \u{cb1}: 'ಱ'
    LetterRra,
    /// \u{cb2}: 'ಲ'
    LetterLa,
    /// \u{cb3}: 'ಳ'
    LetterLla,
    /// \u{cb5}: 'ವ'
    LetterVa,
    /// \u{cb6}: 'ಶ'
    LetterSha,
    /// \u{cb7}: 'ಷ'
    LetterSsa,
    /// \u{cb8}: 'ಸ'
    LetterSa,
    /// \u{cb9}: 'ಹ'
    LetterHa,
    /// \u{cbc}: '಼'
    SignNukta,
    /// \u{cbd}: 'ಽ'
    SignAvagraha,
    /// \u{cbe}: 'ಾ'
    VowelSignAa,
    /// \u{cbf}: 'ಿ'
    VowelSignI,
    /// \u{cc0}: 'ೀ'
    VowelSignIi,
    /// \u{cc1}: 'ು'
    VowelSignU,
    /// \u{cc2}: 'ೂ'
    VowelSignUu,
    /// \u{cc3}: 'ೃ'
    VowelSignVocalicR,
    /// \u{cc4}: 'ೄ'
    VowelSignVocalicRr,
    /// \u{cc6}: 'ೆ'
    VowelSignE,
    /// \u{cc7}: 'ೇ'
    VowelSignEe,
    /// \u{cc8}: 'ೈ'
    VowelSignAi,
    /// \u{cca}: 'ೊ'
    VowelSignO,
    /// \u{ccb}: 'ೋ'
    VowelSignOo,
    /// \u{ccc}: 'ೌ'
    VowelSignAu,
    /// \u{ccd}: '್'
    SignVirama,
    /// \u{cd5}: 'ೕ'
    LengthMark,
    /// \u{cd6}: 'ೖ'
    AiLengthMark,
    /// \u{cde}: 'ೞ'
    LetterFa,
    /// \u{ce0}: 'ೠ'
    LetterVocalicRr,
    /// \u{ce1}: 'ೡ'
    LetterVocalicLl,
    /// \u{ce2}: 'ೢ'
    VowelSignVocalicL,
    /// \u{ce3}: 'ೣ'
    VowelSignVocalicLl,
    /// \u{ce6}: '೦'
    DigitZero,
    /// \u{ce7}: '೧'
    DigitOne,
    /// \u{ce8}: '೨'
    DigitTwo,
    /// \u{ce9}: '೩'
    DigitThree,
    /// \u{cea}: '೪'
    DigitFour,
    /// \u{ceb}: '೫'
    DigitFive,
    /// \u{cec}: '೬'
    DigitSix,
    /// \u{ced}: '೭'
    DigitSeven,
    /// \u{cee}: '೮'
    DigitEight,
    /// \u{cef}: '೯'
    DigitNine,
    /// \u{cf1}: 'ೱ'
    SignJihvamuliya,
    /// \u{cf2}: 'ೲ'
    SignUpadhmaniya,
}

impl Into<char> for Kannada {
    fn into(self) -> char {
        match self {
            Kannada::SignSpacingCandrabindu => 'ಀ',
            Kannada::SignCandrabindu => 'ಁ',
            Kannada::SignAnusvara => 'ಂ',
            Kannada::SignVisarga => 'ಃ',
            Kannada::SignSiddham => '಄',
            Kannada::LetterA => 'ಅ',
            Kannada::LetterAa => 'ಆ',
            Kannada::LetterI => 'ಇ',
            Kannada::LetterIi => 'ಈ',
            Kannada::LetterU => 'ಉ',
            Kannada::LetterUu => 'ಊ',
            Kannada::LetterVocalicR => 'ಋ',
            Kannada::LetterVocalicL => 'ಌ',
            Kannada::LetterE => 'ಎ',
            Kannada::LetterEe => 'ಏ',
            Kannada::LetterAi => 'ಐ',
            Kannada::LetterO => 'ಒ',
            Kannada::LetterOo => 'ಓ',
            Kannada::LetterAu => 'ಔ',
            Kannada::LetterKa => 'ಕ',
            Kannada::LetterKha => 'ಖ',
            Kannada::LetterGa => 'ಗ',
            Kannada::LetterGha => 'ಘ',
            Kannada::LetterNga => 'ಙ',
            Kannada::LetterCa => 'ಚ',
            Kannada::LetterCha => 'ಛ',
            Kannada::LetterJa => 'ಜ',
            Kannada::LetterJha => 'ಝ',
            Kannada::LetterNya => 'ಞ',
            Kannada::LetterTta => 'ಟ',
            Kannada::LetterTtha => 'ಠ',
            Kannada::LetterDda => 'ಡ',
            Kannada::LetterDdha => 'ಢ',
            Kannada::LetterNna => 'ಣ',
            Kannada::LetterTa => 'ತ',
            Kannada::LetterTha => 'ಥ',
            Kannada::LetterDa => 'ದ',
            Kannada::LetterDha => 'ಧ',
            Kannada::LetterNa => 'ನ',
            Kannada::LetterPa => 'ಪ',
            Kannada::LetterPha => 'ಫ',
            Kannada::LetterBa => 'ಬ',
            Kannada::LetterBha => 'ಭ',
            Kannada::LetterMa => 'ಮ',
            Kannada::LetterYa => 'ಯ',
            Kannada::LetterRa => 'ರ',
            Kannada::LetterRra => 'ಱ',
            Kannada::LetterLa => 'ಲ',
            Kannada::LetterLla => 'ಳ',
            Kannada::LetterVa => 'ವ',
            Kannada::LetterSha => 'ಶ',
            Kannada::LetterSsa => 'ಷ',
            Kannada::LetterSa => 'ಸ',
            Kannada::LetterHa => 'ಹ',
            Kannada::SignNukta => '಼',
            Kannada::SignAvagraha => 'ಽ',
            Kannada::VowelSignAa => 'ಾ',
            Kannada::VowelSignI => 'ಿ',
            Kannada::VowelSignIi => 'ೀ',
            Kannada::VowelSignU => 'ು',
            Kannada::VowelSignUu => 'ೂ',
            Kannada::VowelSignVocalicR => 'ೃ',
            Kannada::VowelSignVocalicRr => 'ೄ',
            Kannada::VowelSignE => 'ೆ',
            Kannada::VowelSignEe => 'ೇ',
            Kannada::VowelSignAi => 'ೈ',
            Kannada::VowelSignO => 'ೊ',
            Kannada::VowelSignOo => 'ೋ',
            Kannada::VowelSignAu => 'ೌ',
            Kannada::SignVirama => '್',
            Kannada::LengthMark => 'ೕ',
            Kannada::AiLengthMark => 'ೖ',
            Kannada::LetterFa => 'ೞ',
            Kannada::LetterVocalicRr => 'ೠ',
            Kannada::LetterVocalicLl => 'ೡ',
            Kannada::VowelSignVocalicL => 'ೢ',
            Kannada::VowelSignVocalicLl => 'ೣ',
            Kannada::DigitZero => '೦',
            Kannada::DigitOne => '೧',
            Kannada::DigitTwo => '೨',
            Kannada::DigitThree => '೩',
            Kannada::DigitFour => '೪',
            Kannada::DigitFive => '೫',
            Kannada::DigitSix => '೬',
            Kannada::DigitSeven => '೭',
            Kannada::DigitEight => '೮',
            Kannada::DigitNine => '೯',
            Kannada::SignJihvamuliya => 'ೱ',
            Kannada::SignUpadhmaniya => 'ೲ',
        }
    }
}

impl std::convert::TryFrom<char> for Kannada {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ಀ' => Ok(Kannada::SignSpacingCandrabindu),
            'ಁ' => Ok(Kannada::SignCandrabindu),
            'ಂ' => Ok(Kannada::SignAnusvara),
            'ಃ' => Ok(Kannada::SignVisarga),
            '಄' => Ok(Kannada::SignSiddham),
            'ಅ' => Ok(Kannada::LetterA),
            'ಆ' => Ok(Kannada::LetterAa),
            'ಇ' => Ok(Kannada::LetterI),
            'ಈ' => Ok(Kannada::LetterIi),
            'ಉ' => Ok(Kannada::LetterU),
            'ಊ' => Ok(Kannada::LetterUu),
            'ಋ' => Ok(Kannada::LetterVocalicR),
            'ಌ' => Ok(Kannada::LetterVocalicL),
            'ಎ' => Ok(Kannada::LetterE),
            'ಏ' => Ok(Kannada::LetterEe),
            'ಐ' => Ok(Kannada::LetterAi),
            'ಒ' => Ok(Kannada::LetterO),
            'ಓ' => Ok(Kannada::LetterOo),
            'ಔ' => Ok(Kannada::LetterAu),
            'ಕ' => Ok(Kannada::LetterKa),
            'ಖ' => Ok(Kannada::LetterKha),
            'ಗ' => Ok(Kannada::LetterGa),
            'ಘ' => Ok(Kannada::LetterGha),
            'ಙ' => Ok(Kannada::LetterNga),
            'ಚ' => Ok(Kannada::LetterCa),
            'ಛ' => Ok(Kannada::LetterCha),
            'ಜ' => Ok(Kannada::LetterJa),
            'ಝ' => Ok(Kannada::LetterJha),
            'ಞ' => Ok(Kannada::LetterNya),
            'ಟ' => Ok(Kannada::LetterTta),
            'ಠ' => Ok(Kannada::LetterTtha),
            'ಡ' => Ok(Kannada::LetterDda),
            'ಢ' => Ok(Kannada::LetterDdha),
            'ಣ' => Ok(Kannada::LetterNna),
            'ತ' => Ok(Kannada::LetterTa),
            'ಥ' => Ok(Kannada::LetterTha),
            'ದ' => Ok(Kannada::LetterDa),
            'ಧ' => Ok(Kannada::LetterDha),
            'ನ' => Ok(Kannada::LetterNa),
            'ಪ' => Ok(Kannada::LetterPa),
            'ಫ' => Ok(Kannada::LetterPha),
            'ಬ' => Ok(Kannada::LetterBa),
            'ಭ' => Ok(Kannada::LetterBha),
            'ಮ' => Ok(Kannada::LetterMa),
            'ಯ' => Ok(Kannada::LetterYa),
            'ರ' => Ok(Kannada::LetterRa),
            'ಱ' => Ok(Kannada::LetterRra),
            'ಲ' => Ok(Kannada::LetterLa),
            'ಳ' => Ok(Kannada::LetterLla),
            'ವ' => Ok(Kannada::LetterVa),
            'ಶ' => Ok(Kannada::LetterSha),
            'ಷ' => Ok(Kannada::LetterSsa),
            'ಸ' => Ok(Kannada::LetterSa),
            'ಹ' => Ok(Kannada::LetterHa),
            '಼' => Ok(Kannada::SignNukta),
            'ಽ' => Ok(Kannada::SignAvagraha),
            'ಾ' => Ok(Kannada::VowelSignAa),
            'ಿ' => Ok(Kannada::VowelSignI),
            'ೀ' => Ok(Kannada::VowelSignIi),
            'ು' => Ok(Kannada::VowelSignU),
            'ೂ' => Ok(Kannada::VowelSignUu),
            'ೃ' => Ok(Kannada::VowelSignVocalicR),
            'ೄ' => Ok(Kannada::VowelSignVocalicRr),
            'ೆ' => Ok(Kannada::VowelSignE),
            'ೇ' => Ok(Kannada::VowelSignEe),
            'ೈ' => Ok(Kannada::VowelSignAi),
            'ೊ' => Ok(Kannada::VowelSignO),
            'ೋ' => Ok(Kannada::VowelSignOo),
            'ೌ' => Ok(Kannada::VowelSignAu),
            '್' => Ok(Kannada::SignVirama),
            'ೕ' => Ok(Kannada::LengthMark),
            'ೖ' => Ok(Kannada::AiLengthMark),
            'ೞ' => Ok(Kannada::LetterFa),
            'ೠ' => Ok(Kannada::LetterVocalicRr),
            'ೡ' => Ok(Kannada::LetterVocalicLl),
            'ೢ' => Ok(Kannada::VowelSignVocalicL),
            'ೣ' => Ok(Kannada::VowelSignVocalicLl),
            '೦' => Ok(Kannada::DigitZero),
            '೧' => Ok(Kannada::DigitOne),
            '೨' => Ok(Kannada::DigitTwo),
            '೩' => Ok(Kannada::DigitThree),
            '೪' => Ok(Kannada::DigitFour),
            '೫' => Ok(Kannada::DigitFive),
            '೬' => Ok(Kannada::DigitSix),
            '೭' => Ok(Kannada::DigitSeven),
            '೮' => Ok(Kannada::DigitEight),
            '೯' => Ok(Kannada::DigitNine),
            'ೱ' => Ok(Kannada::SignJihvamuliya),
            'ೲ' => Ok(Kannada::SignUpadhmaniya),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Kannada {
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

impl std::convert::TryFrom<u32> for Kannada {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Kannada {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Kannada {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Kannada::SignSpacingCandrabindu
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Kannada::SignSpacingCandrabindu => "kannada sign spacing candrabindu",
            Kannada::SignCandrabindu => "kannada sign candrabindu",
            Kannada::SignAnusvara => "kannada sign anusvara",
            Kannada::SignVisarga => "kannada sign visarga",
            Kannada::SignSiddham => "kannada sign siddham",
            Kannada::LetterA => "kannada letter a",
            Kannada::LetterAa => "kannada letter aa",
            Kannada::LetterI => "kannada letter i",
            Kannada::LetterIi => "kannada letter ii",
            Kannada::LetterU => "kannada letter u",
            Kannada::LetterUu => "kannada letter uu",
            Kannada::LetterVocalicR => "kannada letter vocalic r",
            Kannada::LetterVocalicL => "kannada letter vocalic l",
            Kannada::LetterE => "kannada letter e",
            Kannada::LetterEe => "kannada letter ee",
            Kannada::LetterAi => "kannada letter ai",
            Kannada::LetterO => "kannada letter o",
            Kannada::LetterOo => "kannada letter oo",
            Kannada::LetterAu => "kannada letter au",
            Kannada::LetterKa => "kannada letter ka",
            Kannada::LetterKha => "kannada letter kha",
            Kannada::LetterGa => "kannada letter ga",
            Kannada::LetterGha => "kannada letter gha",
            Kannada::LetterNga => "kannada letter nga",
            Kannada::LetterCa => "kannada letter ca",
            Kannada::LetterCha => "kannada letter cha",
            Kannada::LetterJa => "kannada letter ja",
            Kannada::LetterJha => "kannada letter jha",
            Kannada::LetterNya => "kannada letter nya",
            Kannada::LetterTta => "kannada letter tta",
            Kannada::LetterTtha => "kannada letter ttha",
            Kannada::LetterDda => "kannada letter dda",
            Kannada::LetterDdha => "kannada letter ddha",
            Kannada::LetterNna => "kannada letter nna",
            Kannada::LetterTa => "kannada letter ta",
            Kannada::LetterTha => "kannada letter tha",
            Kannada::LetterDa => "kannada letter da",
            Kannada::LetterDha => "kannada letter dha",
            Kannada::LetterNa => "kannada letter na",
            Kannada::LetterPa => "kannada letter pa",
            Kannada::LetterPha => "kannada letter pha",
            Kannada::LetterBa => "kannada letter ba",
            Kannada::LetterBha => "kannada letter bha",
            Kannada::LetterMa => "kannada letter ma",
            Kannada::LetterYa => "kannada letter ya",
            Kannada::LetterRa => "kannada letter ra",
            Kannada::LetterRra => "kannada letter rra",
            Kannada::LetterLa => "kannada letter la",
            Kannada::LetterLla => "kannada letter lla",
            Kannada::LetterVa => "kannada letter va",
            Kannada::LetterSha => "kannada letter sha",
            Kannada::LetterSsa => "kannada letter ssa",
            Kannada::LetterSa => "kannada letter sa",
            Kannada::LetterHa => "kannada letter ha",
            Kannada::SignNukta => "kannada sign nukta",
            Kannada::SignAvagraha => "kannada sign avagraha",
            Kannada::VowelSignAa => "kannada vowel sign aa",
            Kannada::VowelSignI => "kannada vowel sign i",
            Kannada::VowelSignIi => "kannada vowel sign ii",
            Kannada::VowelSignU => "kannada vowel sign u",
            Kannada::VowelSignUu => "kannada vowel sign uu",
            Kannada::VowelSignVocalicR => "kannada vowel sign vocalic r",
            Kannada::VowelSignVocalicRr => "kannada vowel sign vocalic rr",
            Kannada::VowelSignE => "kannada vowel sign e",
            Kannada::VowelSignEe => "kannada vowel sign ee",
            Kannada::VowelSignAi => "kannada vowel sign ai",
            Kannada::VowelSignO => "kannada vowel sign o",
            Kannada::VowelSignOo => "kannada vowel sign oo",
            Kannada::VowelSignAu => "kannada vowel sign au",
            Kannada::SignVirama => "kannada sign virama",
            Kannada::LengthMark => "kannada length mark",
            Kannada::AiLengthMark => "kannada ai length mark",
            Kannada::LetterFa => "kannada letter fa",
            Kannada::LetterVocalicRr => "kannada letter vocalic rr",
            Kannada::LetterVocalicLl => "kannada letter vocalic ll",
            Kannada::VowelSignVocalicL => "kannada vowel sign vocalic l",
            Kannada::VowelSignVocalicLl => "kannada vowel sign vocalic ll",
            Kannada::DigitZero => "kannada digit zero",
            Kannada::DigitOne => "kannada digit one",
            Kannada::DigitTwo => "kannada digit two",
            Kannada::DigitThree => "kannada digit three",
            Kannada::DigitFour => "kannada digit four",
            Kannada::DigitFive => "kannada digit five",
            Kannada::DigitSix => "kannada digit six",
            Kannada::DigitSeven => "kannada digit seven",
            Kannada::DigitEight => "kannada digit eight",
            Kannada::DigitNine => "kannada digit nine",
            Kannada::SignJihvamuliya => "kannada sign jihvamuliya",
            Kannada::SignUpadhmaniya => "kannada sign upadhmaniya",
        }
    }
}
