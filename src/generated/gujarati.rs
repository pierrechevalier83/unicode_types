
/// An enum to represent all characters in the Gujarati block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Gujarati {
    /// \u{a81}: 'ઁ'
    SignCandrabindu,
    /// \u{a82}: 'ં'
    SignAnusvara,
    /// \u{a83}: 'ઃ'
    SignVisarga,
    /// \u{a85}: 'અ'
    LetterA,
    /// \u{a86}: 'આ'
    LetterAa,
    /// \u{a87}: 'ઇ'
    LetterI,
    /// \u{a88}: 'ઈ'
    LetterIi,
    /// \u{a89}: 'ઉ'
    LetterU,
    /// \u{a8a}: 'ઊ'
    LetterUu,
    /// \u{a8b}: 'ઋ'
    LetterVocalicR,
    /// \u{a8c}: 'ઌ'
    LetterVocalicL,
    /// \u{a8d}: 'ઍ'
    VowelCandraE,
    /// \u{a8f}: 'એ'
    LetterE,
    /// \u{a90}: 'ઐ'
    LetterAi,
    /// \u{a91}: 'ઑ'
    VowelCandraO,
    /// \u{a93}: 'ઓ'
    LetterO,
    /// \u{a94}: 'ઔ'
    LetterAu,
    /// \u{a95}: 'ક'
    LetterKa,
    /// \u{a96}: 'ખ'
    LetterKha,
    /// \u{a97}: 'ગ'
    LetterGa,
    /// \u{a98}: 'ઘ'
    LetterGha,
    /// \u{a99}: 'ઙ'
    LetterNga,
    /// \u{a9a}: 'ચ'
    LetterCa,
    /// \u{a9b}: 'છ'
    LetterCha,
    /// \u{a9c}: 'જ'
    LetterJa,
    /// \u{a9d}: 'ઝ'
    LetterJha,
    /// \u{a9e}: 'ઞ'
    LetterNya,
    /// \u{a9f}: 'ટ'
    LetterTta,
    /// \u{aa0}: 'ઠ'
    LetterTtha,
    /// \u{aa1}: 'ડ'
    LetterDda,
    /// \u{aa2}: 'ઢ'
    LetterDdha,
    /// \u{aa3}: 'ણ'
    LetterNna,
    /// \u{aa4}: 'ત'
    LetterTa,
    /// \u{aa5}: 'થ'
    LetterTha,
    /// \u{aa6}: 'દ'
    LetterDa,
    /// \u{aa7}: 'ધ'
    LetterDha,
    /// \u{aa8}: 'ન'
    LetterNa,
    /// \u{aaa}: 'પ'
    LetterPa,
    /// \u{aab}: 'ફ'
    LetterPha,
    /// \u{aac}: 'બ'
    LetterBa,
    /// \u{aad}: 'ભ'
    LetterBha,
    /// \u{aae}: 'મ'
    LetterMa,
    /// \u{aaf}: 'ય'
    LetterYa,
    /// \u{ab0}: 'ર'
    LetterRa,
    /// \u{ab2}: 'લ'
    LetterLa,
    /// \u{ab3}: 'ળ'
    LetterLla,
    /// \u{ab5}: 'વ'
    LetterVa,
    /// \u{ab6}: 'શ'
    LetterSha,
    /// \u{ab7}: 'ષ'
    LetterSsa,
    /// \u{ab8}: 'સ'
    LetterSa,
    /// \u{ab9}: 'હ'
    LetterHa,
    /// \u{abc}: '઼'
    SignNukta,
    /// \u{abd}: 'ઽ'
    SignAvagraha,
    /// \u{abe}: 'ા'
    VowelSignAa,
    /// \u{abf}: 'િ'
    VowelSignI,
    /// \u{ac0}: 'ી'
    VowelSignIi,
    /// \u{ac1}: 'ુ'
    VowelSignU,
    /// \u{ac2}: 'ૂ'
    VowelSignUu,
    /// \u{ac3}: 'ૃ'
    VowelSignVocalicR,
    /// \u{ac4}: 'ૄ'
    VowelSignVocalicRr,
    /// \u{ac5}: 'ૅ'
    VowelSignCandraE,
    /// \u{ac7}: 'ે'
    VowelSignE,
    /// \u{ac8}: 'ૈ'
    VowelSignAi,
    /// \u{ac9}: 'ૉ'
    VowelSignCandraO,
    /// \u{acb}: 'ો'
    VowelSignO,
    /// \u{acc}: 'ૌ'
    VowelSignAu,
    /// \u{acd}: '્'
    SignVirama,
    /// \u{ad0}: 'ૐ'
    Om,
    /// \u{ae0}: 'ૠ'
    LetterVocalicRr,
    /// \u{ae1}: 'ૡ'
    LetterVocalicLl,
    /// \u{ae2}: 'ૢ'
    VowelSignVocalicL,
    /// \u{ae3}: 'ૣ'
    VowelSignVocalicLl,
    /// \u{ae6}: '૦'
    DigitZero,
    /// \u{ae7}: '૧'
    DigitOne,
    /// \u{ae8}: '૨'
    DigitTwo,
    /// \u{ae9}: '૩'
    DigitThree,
    /// \u{aea}: '૪'
    DigitFour,
    /// \u{aeb}: '૫'
    DigitFive,
    /// \u{aec}: '૬'
    DigitSix,
    /// \u{aed}: '૭'
    DigitSeven,
    /// \u{aee}: '૮'
    DigitEight,
    /// \u{aef}: '૯'
    DigitNine,
    /// \u{af0}: '૰'
    AbbreviationSign,
    /// \u{af1}: '૱'
    RupeeSign,
    /// \u{af9}: 'ૹ'
    LetterZha,
    /// \u{afa}: 'ૺ'
    SignSukun,
    /// \u{afb}: 'ૻ'
    SignShadda,
    /// \u{afc}: 'ૼ'
    SignMaddah,
    /// \u{afd}: '૽'
    SignThreeDashDotNuktaAbove,
    /// \u{afe}: '૾'
    SignCircleNuktaAbove,
}

impl Into<char> for Gujarati {
    fn into(self) -> char {
        match self {
            Gujarati::SignCandrabindu => 'ઁ',
            Gujarati::SignAnusvara => 'ં',
            Gujarati::SignVisarga => 'ઃ',
            Gujarati::LetterA => 'અ',
            Gujarati::LetterAa => 'આ',
            Gujarati::LetterI => 'ઇ',
            Gujarati::LetterIi => 'ઈ',
            Gujarati::LetterU => 'ઉ',
            Gujarati::LetterUu => 'ઊ',
            Gujarati::LetterVocalicR => 'ઋ',
            Gujarati::LetterVocalicL => 'ઌ',
            Gujarati::VowelCandraE => 'ઍ',
            Gujarati::LetterE => 'એ',
            Gujarati::LetterAi => 'ઐ',
            Gujarati::VowelCandraO => 'ઑ',
            Gujarati::LetterO => 'ઓ',
            Gujarati::LetterAu => 'ઔ',
            Gujarati::LetterKa => 'ક',
            Gujarati::LetterKha => 'ખ',
            Gujarati::LetterGa => 'ગ',
            Gujarati::LetterGha => 'ઘ',
            Gujarati::LetterNga => 'ઙ',
            Gujarati::LetterCa => 'ચ',
            Gujarati::LetterCha => 'છ',
            Gujarati::LetterJa => 'જ',
            Gujarati::LetterJha => 'ઝ',
            Gujarati::LetterNya => 'ઞ',
            Gujarati::LetterTta => 'ટ',
            Gujarati::LetterTtha => 'ઠ',
            Gujarati::LetterDda => 'ડ',
            Gujarati::LetterDdha => 'ઢ',
            Gujarati::LetterNna => 'ણ',
            Gujarati::LetterTa => 'ત',
            Gujarati::LetterTha => 'થ',
            Gujarati::LetterDa => 'દ',
            Gujarati::LetterDha => 'ધ',
            Gujarati::LetterNa => 'ન',
            Gujarati::LetterPa => 'પ',
            Gujarati::LetterPha => 'ફ',
            Gujarati::LetterBa => 'બ',
            Gujarati::LetterBha => 'ભ',
            Gujarati::LetterMa => 'મ',
            Gujarati::LetterYa => 'ય',
            Gujarati::LetterRa => 'ર',
            Gujarati::LetterLa => 'લ',
            Gujarati::LetterLla => 'ળ',
            Gujarati::LetterVa => 'વ',
            Gujarati::LetterSha => 'શ',
            Gujarati::LetterSsa => 'ષ',
            Gujarati::LetterSa => 'સ',
            Gujarati::LetterHa => 'હ',
            Gujarati::SignNukta => '઼',
            Gujarati::SignAvagraha => 'ઽ',
            Gujarati::VowelSignAa => 'ા',
            Gujarati::VowelSignI => 'િ',
            Gujarati::VowelSignIi => 'ી',
            Gujarati::VowelSignU => 'ુ',
            Gujarati::VowelSignUu => 'ૂ',
            Gujarati::VowelSignVocalicR => 'ૃ',
            Gujarati::VowelSignVocalicRr => 'ૄ',
            Gujarati::VowelSignCandraE => 'ૅ',
            Gujarati::VowelSignE => 'ે',
            Gujarati::VowelSignAi => 'ૈ',
            Gujarati::VowelSignCandraO => 'ૉ',
            Gujarati::VowelSignO => 'ો',
            Gujarati::VowelSignAu => 'ૌ',
            Gujarati::SignVirama => '્',
            Gujarati::Om => 'ૐ',
            Gujarati::LetterVocalicRr => 'ૠ',
            Gujarati::LetterVocalicLl => 'ૡ',
            Gujarati::VowelSignVocalicL => 'ૢ',
            Gujarati::VowelSignVocalicLl => 'ૣ',
            Gujarati::DigitZero => '૦',
            Gujarati::DigitOne => '૧',
            Gujarati::DigitTwo => '૨',
            Gujarati::DigitThree => '૩',
            Gujarati::DigitFour => '૪',
            Gujarati::DigitFive => '૫',
            Gujarati::DigitSix => '૬',
            Gujarati::DigitSeven => '૭',
            Gujarati::DigitEight => '૮',
            Gujarati::DigitNine => '૯',
            Gujarati::AbbreviationSign => '૰',
            Gujarati::RupeeSign => '૱',
            Gujarati::LetterZha => 'ૹ',
            Gujarati::SignSukun => 'ૺ',
            Gujarati::SignShadda => 'ૻ',
            Gujarati::SignMaddah => 'ૼ',
            Gujarati::SignThreeDashDotNuktaAbove => '૽',
            Gujarati::SignCircleNuktaAbove => '૾',
        }
    }
}

impl std::convert::TryFrom<char> for Gujarati {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ઁ' => Ok(Gujarati::SignCandrabindu),
            'ં' => Ok(Gujarati::SignAnusvara),
            'ઃ' => Ok(Gujarati::SignVisarga),
            'અ' => Ok(Gujarati::LetterA),
            'આ' => Ok(Gujarati::LetterAa),
            'ઇ' => Ok(Gujarati::LetterI),
            'ઈ' => Ok(Gujarati::LetterIi),
            'ઉ' => Ok(Gujarati::LetterU),
            'ઊ' => Ok(Gujarati::LetterUu),
            'ઋ' => Ok(Gujarati::LetterVocalicR),
            'ઌ' => Ok(Gujarati::LetterVocalicL),
            'ઍ' => Ok(Gujarati::VowelCandraE),
            'એ' => Ok(Gujarati::LetterE),
            'ઐ' => Ok(Gujarati::LetterAi),
            'ઑ' => Ok(Gujarati::VowelCandraO),
            'ઓ' => Ok(Gujarati::LetterO),
            'ઔ' => Ok(Gujarati::LetterAu),
            'ક' => Ok(Gujarati::LetterKa),
            'ખ' => Ok(Gujarati::LetterKha),
            'ગ' => Ok(Gujarati::LetterGa),
            'ઘ' => Ok(Gujarati::LetterGha),
            'ઙ' => Ok(Gujarati::LetterNga),
            'ચ' => Ok(Gujarati::LetterCa),
            'છ' => Ok(Gujarati::LetterCha),
            'જ' => Ok(Gujarati::LetterJa),
            'ઝ' => Ok(Gujarati::LetterJha),
            'ઞ' => Ok(Gujarati::LetterNya),
            'ટ' => Ok(Gujarati::LetterTta),
            'ઠ' => Ok(Gujarati::LetterTtha),
            'ડ' => Ok(Gujarati::LetterDda),
            'ઢ' => Ok(Gujarati::LetterDdha),
            'ણ' => Ok(Gujarati::LetterNna),
            'ત' => Ok(Gujarati::LetterTa),
            'થ' => Ok(Gujarati::LetterTha),
            'દ' => Ok(Gujarati::LetterDa),
            'ધ' => Ok(Gujarati::LetterDha),
            'ન' => Ok(Gujarati::LetterNa),
            'પ' => Ok(Gujarati::LetterPa),
            'ફ' => Ok(Gujarati::LetterPha),
            'બ' => Ok(Gujarati::LetterBa),
            'ભ' => Ok(Gujarati::LetterBha),
            'મ' => Ok(Gujarati::LetterMa),
            'ય' => Ok(Gujarati::LetterYa),
            'ર' => Ok(Gujarati::LetterRa),
            'લ' => Ok(Gujarati::LetterLa),
            'ળ' => Ok(Gujarati::LetterLla),
            'વ' => Ok(Gujarati::LetterVa),
            'શ' => Ok(Gujarati::LetterSha),
            'ષ' => Ok(Gujarati::LetterSsa),
            'સ' => Ok(Gujarati::LetterSa),
            'હ' => Ok(Gujarati::LetterHa),
            '઼' => Ok(Gujarati::SignNukta),
            'ઽ' => Ok(Gujarati::SignAvagraha),
            'ા' => Ok(Gujarati::VowelSignAa),
            'િ' => Ok(Gujarati::VowelSignI),
            'ી' => Ok(Gujarati::VowelSignIi),
            'ુ' => Ok(Gujarati::VowelSignU),
            'ૂ' => Ok(Gujarati::VowelSignUu),
            'ૃ' => Ok(Gujarati::VowelSignVocalicR),
            'ૄ' => Ok(Gujarati::VowelSignVocalicRr),
            'ૅ' => Ok(Gujarati::VowelSignCandraE),
            'ે' => Ok(Gujarati::VowelSignE),
            'ૈ' => Ok(Gujarati::VowelSignAi),
            'ૉ' => Ok(Gujarati::VowelSignCandraO),
            'ો' => Ok(Gujarati::VowelSignO),
            'ૌ' => Ok(Gujarati::VowelSignAu),
            '્' => Ok(Gujarati::SignVirama),
            'ૐ' => Ok(Gujarati::Om),
            'ૠ' => Ok(Gujarati::LetterVocalicRr),
            'ૡ' => Ok(Gujarati::LetterVocalicLl),
            'ૢ' => Ok(Gujarati::VowelSignVocalicL),
            'ૣ' => Ok(Gujarati::VowelSignVocalicLl),
            '૦' => Ok(Gujarati::DigitZero),
            '૧' => Ok(Gujarati::DigitOne),
            '૨' => Ok(Gujarati::DigitTwo),
            '૩' => Ok(Gujarati::DigitThree),
            '૪' => Ok(Gujarati::DigitFour),
            '૫' => Ok(Gujarati::DigitFive),
            '૬' => Ok(Gujarati::DigitSix),
            '૭' => Ok(Gujarati::DigitSeven),
            '૮' => Ok(Gujarati::DigitEight),
            '૯' => Ok(Gujarati::DigitNine),
            '૰' => Ok(Gujarati::AbbreviationSign),
            '૱' => Ok(Gujarati::RupeeSign),
            'ૹ' => Ok(Gujarati::LetterZha),
            'ૺ' => Ok(Gujarati::SignSukun),
            'ૻ' => Ok(Gujarati::SignShadda),
            'ૼ' => Ok(Gujarati::SignMaddah),
            '૽' => Ok(Gujarati::SignThreeDashDotNuktaAbove),
            '૾' => Ok(Gujarati::SignCircleNuktaAbove),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Gujarati {
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

impl std::convert::TryFrom<u32> for Gujarati {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Gujarati {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Gujarati {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Gujarati::SignCandrabindu
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Gujarati{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
