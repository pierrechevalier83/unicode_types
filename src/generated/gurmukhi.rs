
/// An enum to represent all characters in the Gurmukhi block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Gurmukhi {
    /// \u{a01}: 'ਁ'
    SignAdakBindi,
    /// \u{a02}: 'ਂ'
    SignBindi,
    /// \u{a03}: 'ਃ'
    SignVisarga,
    /// \u{a05}: 'ਅ'
    LetterA,
    /// \u{a06}: 'ਆ'
    LetterAa,
    /// \u{a07}: 'ਇ'
    LetterI,
    /// \u{a08}: 'ਈ'
    LetterIi,
    /// \u{a09}: 'ਉ'
    LetterU,
    /// \u{a0a}: 'ਊ'
    LetterUu,
    /// \u{a0f}: 'ਏ'
    LetterEe,
    /// \u{a10}: 'ਐ'
    LetterAi,
    /// \u{a13}: 'ਓ'
    LetterOo,
    /// \u{a14}: 'ਔ'
    LetterAu,
    /// \u{a15}: 'ਕ'
    LetterKa,
    /// \u{a16}: 'ਖ'
    LetterKha,
    /// \u{a17}: 'ਗ'
    LetterGa,
    /// \u{a18}: 'ਘ'
    LetterGha,
    /// \u{a19}: 'ਙ'
    LetterNga,
    /// \u{a1a}: 'ਚ'
    LetterCa,
    /// \u{a1b}: 'ਛ'
    LetterCha,
    /// \u{a1c}: 'ਜ'
    LetterJa,
    /// \u{a1d}: 'ਝ'
    LetterJha,
    /// \u{a1e}: 'ਞ'
    LetterNya,
    /// \u{a1f}: 'ਟ'
    LetterTta,
    /// \u{a20}: 'ਠ'
    LetterTtha,
    /// \u{a21}: 'ਡ'
    LetterDda,
    /// \u{a22}: 'ਢ'
    LetterDdha,
    /// \u{a23}: 'ਣ'
    LetterNna,
    /// \u{a24}: 'ਤ'
    LetterTa,
    /// \u{a25}: 'ਥ'
    LetterTha,
    /// \u{a26}: 'ਦ'
    LetterDa,
    /// \u{a27}: 'ਧ'
    LetterDha,
    /// \u{a28}: 'ਨ'
    LetterNa,
    /// \u{a2a}: 'ਪ'
    LetterPa,
    /// \u{a2b}: 'ਫ'
    LetterPha,
    /// \u{a2c}: 'ਬ'
    LetterBa,
    /// \u{a2d}: 'ਭ'
    LetterBha,
    /// \u{a2e}: 'ਮ'
    LetterMa,
    /// \u{a2f}: 'ਯ'
    LetterYa,
    /// \u{a30}: 'ਰ'
    LetterRa,
    /// \u{a32}: 'ਲ'
    LetterLa,
    /// \u{a33}: 'ਲ਼'
    LetterLla,
    /// \u{a35}: 'ਵ'
    LetterVa,
    /// \u{a36}: 'ਸ਼'
    LetterSha,
    /// \u{a38}: 'ਸ'
    LetterSa,
    /// \u{a39}: 'ਹ'
    LetterHa,
    /// \u{a3c}: '਼'
    SignNukta,
    /// \u{a3e}: 'ਾ'
    VowelSignAa,
    /// \u{a3f}: 'ਿ'
    VowelSignI,
    /// \u{a40}: 'ੀ'
    VowelSignIi,
    /// \u{a41}: 'ੁ'
    VowelSignU,
    /// \u{a42}: 'ੂ'
    VowelSignUu,
    /// \u{a47}: 'ੇ'
    VowelSignEe,
    /// \u{a48}: 'ੈ'
    VowelSignAi,
    /// \u{a4b}: 'ੋ'
    VowelSignOo,
    /// \u{a4c}: 'ੌ'
    VowelSignAu,
    /// \u{a4d}: '੍'
    SignVirama,
    /// \u{a51}: 'ੑ'
    SignUdaat,
    /// \u{a59}: 'ਖ਼'
    LetterKhha,
    /// \u{a5a}: 'ਗ਼'
    LetterGhha,
    /// \u{a5b}: 'ਜ਼'
    LetterZa,
    /// \u{a5c}: 'ੜ'
    LetterRra,
    /// \u{a5e}: 'ਫ਼'
    LetterFa,
    /// \u{a66}: '੦'
    DigitZero,
    /// \u{a67}: '੧'
    DigitOne,
    /// \u{a68}: '੨'
    DigitTwo,
    /// \u{a69}: '੩'
    DigitThree,
    /// \u{a6a}: '੪'
    DigitFour,
    /// \u{a6b}: '੫'
    DigitFive,
    /// \u{a6c}: '੬'
    DigitSix,
    /// \u{a6d}: '੭'
    DigitSeven,
    /// \u{a6e}: '੮'
    DigitEight,
    /// \u{a6f}: '੯'
    DigitNine,
    /// \u{a70}: 'ੰ'
    Tippi,
    /// \u{a71}: 'ੱ'
    Addak,
    /// \u{a72}: 'ੲ'
    Iri,
    /// \u{a73}: 'ੳ'
    Ura,
    /// \u{a74}: 'ੴ'
    EkOnkar,
    /// \u{a75}: 'ੵ'
    SignYakash,
    /// \u{a76}: '੶'
    AbbreviationSign,
}

impl Into<char> for Gurmukhi {
    fn into(self) -> char {
        match self {
            Gurmukhi::SignAdakBindi => 'ਁ',
            Gurmukhi::SignBindi => 'ਂ',
            Gurmukhi::SignVisarga => 'ਃ',
            Gurmukhi::LetterA => 'ਅ',
            Gurmukhi::LetterAa => 'ਆ',
            Gurmukhi::LetterI => 'ਇ',
            Gurmukhi::LetterIi => 'ਈ',
            Gurmukhi::LetterU => 'ਉ',
            Gurmukhi::LetterUu => 'ਊ',
            Gurmukhi::LetterEe => 'ਏ',
            Gurmukhi::LetterAi => 'ਐ',
            Gurmukhi::LetterOo => 'ਓ',
            Gurmukhi::LetterAu => 'ਔ',
            Gurmukhi::LetterKa => 'ਕ',
            Gurmukhi::LetterKha => 'ਖ',
            Gurmukhi::LetterGa => 'ਗ',
            Gurmukhi::LetterGha => 'ਘ',
            Gurmukhi::LetterNga => 'ਙ',
            Gurmukhi::LetterCa => 'ਚ',
            Gurmukhi::LetterCha => 'ਛ',
            Gurmukhi::LetterJa => 'ਜ',
            Gurmukhi::LetterJha => 'ਝ',
            Gurmukhi::LetterNya => 'ਞ',
            Gurmukhi::LetterTta => 'ਟ',
            Gurmukhi::LetterTtha => 'ਠ',
            Gurmukhi::LetterDda => 'ਡ',
            Gurmukhi::LetterDdha => 'ਢ',
            Gurmukhi::LetterNna => 'ਣ',
            Gurmukhi::LetterTa => 'ਤ',
            Gurmukhi::LetterTha => 'ਥ',
            Gurmukhi::LetterDa => 'ਦ',
            Gurmukhi::LetterDha => 'ਧ',
            Gurmukhi::LetterNa => 'ਨ',
            Gurmukhi::LetterPa => 'ਪ',
            Gurmukhi::LetterPha => 'ਫ',
            Gurmukhi::LetterBa => 'ਬ',
            Gurmukhi::LetterBha => 'ਭ',
            Gurmukhi::LetterMa => 'ਮ',
            Gurmukhi::LetterYa => 'ਯ',
            Gurmukhi::LetterRa => 'ਰ',
            Gurmukhi::LetterLa => 'ਲ',
            Gurmukhi::LetterLla => 'ਲ਼',
            Gurmukhi::LetterVa => 'ਵ',
            Gurmukhi::LetterSha => 'ਸ਼',
            Gurmukhi::LetterSa => 'ਸ',
            Gurmukhi::LetterHa => 'ਹ',
            Gurmukhi::SignNukta => '਼',
            Gurmukhi::VowelSignAa => 'ਾ',
            Gurmukhi::VowelSignI => 'ਿ',
            Gurmukhi::VowelSignIi => 'ੀ',
            Gurmukhi::VowelSignU => 'ੁ',
            Gurmukhi::VowelSignUu => 'ੂ',
            Gurmukhi::VowelSignEe => 'ੇ',
            Gurmukhi::VowelSignAi => 'ੈ',
            Gurmukhi::VowelSignOo => 'ੋ',
            Gurmukhi::VowelSignAu => 'ੌ',
            Gurmukhi::SignVirama => '੍',
            Gurmukhi::SignUdaat => 'ੑ',
            Gurmukhi::LetterKhha => 'ਖ਼',
            Gurmukhi::LetterGhha => 'ਗ਼',
            Gurmukhi::LetterZa => 'ਜ਼',
            Gurmukhi::LetterRra => 'ੜ',
            Gurmukhi::LetterFa => 'ਫ਼',
            Gurmukhi::DigitZero => '੦',
            Gurmukhi::DigitOne => '੧',
            Gurmukhi::DigitTwo => '੨',
            Gurmukhi::DigitThree => '੩',
            Gurmukhi::DigitFour => '੪',
            Gurmukhi::DigitFive => '੫',
            Gurmukhi::DigitSix => '੬',
            Gurmukhi::DigitSeven => '੭',
            Gurmukhi::DigitEight => '੮',
            Gurmukhi::DigitNine => '੯',
            Gurmukhi::Tippi => 'ੰ',
            Gurmukhi::Addak => 'ੱ',
            Gurmukhi::Iri => 'ੲ',
            Gurmukhi::Ura => 'ੳ',
            Gurmukhi::EkOnkar => 'ੴ',
            Gurmukhi::SignYakash => 'ੵ',
            Gurmukhi::AbbreviationSign => '੶',
        }
    }
}

impl std::convert::TryFrom<char> for Gurmukhi {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ਁ' => Ok(Gurmukhi::SignAdakBindi),
            'ਂ' => Ok(Gurmukhi::SignBindi),
            'ਃ' => Ok(Gurmukhi::SignVisarga),
            'ਅ' => Ok(Gurmukhi::LetterA),
            'ਆ' => Ok(Gurmukhi::LetterAa),
            'ਇ' => Ok(Gurmukhi::LetterI),
            'ਈ' => Ok(Gurmukhi::LetterIi),
            'ਉ' => Ok(Gurmukhi::LetterU),
            'ਊ' => Ok(Gurmukhi::LetterUu),
            'ਏ' => Ok(Gurmukhi::LetterEe),
            'ਐ' => Ok(Gurmukhi::LetterAi),
            'ਓ' => Ok(Gurmukhi::LetterOo),
            'ਔ' => Ok(Gurmukhi::LetterAu),
            'ਕ' => Ok(Gurmukhi::LetterKa),
            'ਖ' => Ok(Gurmukhi::LetterKha),
            'ਗ' => Ok(Gurmukhi::LetterGa),
            'ਘ' => Ok(Gurmukhi::LetterGha),
            'ਙ' => Ok(Gurmukhi::LetterNga),
            'ਚ' => Ok(Gurmukhi::LetterCa),
            'ਛ' => Ok(Gurmukhi::LetterCha),
            'ਜ' => Ok(Gurmukhi::LetterJa),
            'ਝ' => Ok(Gurmukhi::LetterJha),
            'ਞ' => Ok(Gurmukhi::LetterNya),
            'ਟ' => Ok(Gurmukhi::LetterTta),
            'ਠ' => Ok(Gurmukhi::LetterTtha),
            'ਡ' => Ok(Gurmukhi::LetterDda),
            'ਢ' => Ok(Gurmukhi::LetterDdha),
            'ਣ' => Ok(Gurmukhi::LetterNna),
            'ਤ' => Ok(Gurmukhi::LetterTa),
            'ਥ' => Ok(Gurmukhi::LetterTha),
            'ਦ' => Ok(Gurmukhi::LetterDa),
            'ਧ' => Ok(Gurmukhi::LetterDha),
            'ਨ' => Ok(Gurmukhi::LetterNa),
            'ਪ' => Ok(Gurmukhi::LetterPa),
            'ਫ' => Ok(Gurmukhi::LetterPha),
            'ਬ' => Ok(Gurmukhi::LetterBa),
            'ਭ' => Ok(Gurmukhi::LetterBha),
            'ਮ' => Ok(Gurmukhi::LetterMa),
            'ਯ' => Ok(Gurmukhi::LetterYa),
            'ਰ' => Ok(Gurmukhi::LetterRa),
            'ਲ' => Ok(Gurmukhi::LetterLa),
            'ਲ਼' => Ok(Gurmukhi::LetterLla),
            'ਵ' => Ok(Gurmukhi::LetterVa),
            'ਸ਼' => Ok(Gurmukhi::LetterSha),
            'ਸ' => Ok(Gurmukhi::LetterSa),
            'ਹ' => Ok(Gurmukhi::LetterHa),
            '਼' => Ok(Gurmukhi::SignNukta),
            'ਾ' => Ok(Gurmukhi::VowelSignAa),
            'ਿ' => Ok(Gurmukhi::VowelSignI),
            'ੀ' => Ok(Gurmukhi::VowelSignIi),
            'ੁ' => Ok(Gurmukhi::VowelSignU),
            'ੂ' => Ok(Gurmukhi::VowelSignUu),
            'ੇ' => Ok(Gurmukhi::VowelSignEe),
            'ੈ' => Ok(Gurmukhi::VowelSignAi),
            'ੋ' => Ok(Gurmukhi::VowelSignOo),
            'ੌ' => Ok(Gurmukhi::VowelSignAu),
            '੍' => Ok(Gurmukhi::SignVirama),
            'ੑ' => Ok(Gurmukhi::SignUdaat),
            'ਖ਼' => Ok(Gurmukhi::LetterKhha),
            'ਗ਼' => Ok(Gurmukhi::LetterGhha),
            'ਜ਼' => Ok(Gurmukhi::LetterZa),
            'ੜ' => Ok(Gurmukhi::LetterRra),
            'ਫ਼' => Ok(Gurmukhi::LetterFa),
            '੦' => Ok(Gurmukhi::DigitZero),
            '੧' => Ok(Gurmukhi::DigitOne),
            '੨' => Ok(Gurmukhi::DigitTwo),
            '੩' => Ok(Gurmukhi::DigitThree),
            '੪' => Ok(Gurmukhi::DigitFour),
            '੫' => Ok(Gurmukhi::DigitFive),
            '੬' => Ok(Gurmukhi::DigitSix),
            '੭' => Ok(Gurmukhi::DigitSeven),
            '੮' => Ok(Gurmukhi::DigitEight),
            '੯' => Ok(Gurmukhi::DigitNine),
            'ੰ' => Ok(Gurmukhi::Tippi),
            'ੱ' => Ok(Gurmukhi::Addak),
            'ੲ' => Ok(Gurmukhi::Iri),
            'ੳ' => Ok(Gurmukhi::Ura),
            'ੴ' => Ok(Gurmukhi::EkOnkar),
            'ੵ' => Ok(Gurmukhi::SignYakash),
            '੶' => Ok(Gurmukhi::AbbreviationSign),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Gurmukhi {
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

impl std::convert::TryFrom<u32> for Gurmukhi {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Gurmukhi {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Gurmukhi {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Gurmukhi::SignAdakBindi
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Gurmukhi::SignAdakBindi => "gurmukhi sign adak bindi",
            Gurmukhi::SignBindi => "gurmukhi sign bindi",
            Gurmukhi::SignVisarga => "gurmukhi sign visarga",
            Gurmukhi::LetterA => "gurmukhi letter a",
            Gurmukhi::LetterAa => "gurmukhi letter aa",
            Gurmukhi::LetterI => "gurmukhi letter i",
            Gurmukhi::LetterIi => "gurmukhi letter ii",
            Gurmukhi::LetterU => "gurmukhi letter u",
            Gurmukhi::LetterUu => "gurmukhi letter uu",
            Gurmukhi::LetterEe => "gurmukhi letter ee",
            Gurmukhi::LetterAi => "gurmukhi letter ai",
            Gurmukhi::LetterOo => "gurmukhi letter oo",
            Gurmukhi::LetterAu => "gurmukhi letter au",
            Gurmukhi::LetterKa => "gurmukhi letter ka",
            Gurmukhi::LetterKha => "gurmukhi letter kha",
            Gurmukhi::LetterGa => "gurmukhi letter ga",
            Gurmukhi::LetterGha => "gurmukhi letter gha",
            Gurmukhi::LetterNga => "gurmukhi letter nga",
            Gurmukhi::LetterCa => "gurmukhi letter ca",
            Gurmukhi::LetterCha => "gurmukhi letter cha",
            Gurmukhi::LetterJa => "gurmukhi letter ja",
            Gurmukhi::LetterJha => "gurmukhi letter jha",
            Gurmukhi::LetterNya => "gurmukhi letter nya",
            Gurmukhi::LetterTta => "gurmukhi letter tta",
            Gurmukhi::LetterTtha => "gurmukhi letter ttha",
            Gurmukhi::LetterDda => "gurmukhi letter dda",
            Gurmukhi::LetterDdha => "gurmukhi letter ddha",
            Gurmukhi::LetterNna => "gurmukhi letter nna",
            Gurmukhi::LetterTa => "gurmukhi letter ta",
            Gurmukhi::LetterTha => "gurmukhi letter tha",
            Gurmukhi::LetterDa => "gurmukhi letter da",
            Gurmukhi::LetterDha => "gurmukhi letter dha",
            Gurmukhi::LetterNa => "gurmukhi letter na",
            Gurmukhi::LetterPa => "gurmukhi letter pa",
            Gurmukhi::LetterPha => "gurmukhi letter pha",
            Gurmukhi::LetterBa => "gurmukhi letter ba",
            Gurmukhi::LetterBha => "gurmukhi letter bha",
            Gurmukhi::LetterMa => "gurmukhi letter ma",
            Gurmukhi::LetterYa => "gurmukhi letter ya",
            Gurmukhi::LetterRa => "gurmukhi letter ra",
            Gurmukhi::LetterLa => "gurmukhi letter la",
            Gurmukhi::LetterLla => "gurmukhi letter lla",
            Gurmukhi::LetterVa => "gurmukhi letter va",
            Gurmukhi::LetterSha => "gurmukhi letter sha",
            Gurmukhi::LetterSa => "gurmukhi letter sa",
            Gurmukhi::LetterHa => "gurmukhi letter ha",
            Gurmukhi::SignNukta => "gurmukhi sign nukta",
            Gurmukhi::VowelSignAa => "gurmukhi vowel sign aa",
            Gurmukhi::VowelSignI => "gurmukhi vowel sign i",
            Gurmukhi::VowelSignIi => "gurmukhi vowel sign ii",
            Gurmukhi::VowelSignU => "gurmukhi vowel sign u",
            Gurmukhi::VowelSignUu => "gurmukhi vowel sign uu",
            Gurmukhi::VowelSignEe => "gurmukhi vowel sign ee",
            Gurmukhi::VowelSignAi => "gurmukhi vowel sign ai",
            Gurmukhi::VowelSignOo => "gurmukhi vowel sign oo",
            Gurmukhi::VowelSignAu => "gurmukhi vowel sign au",
            Gurmukhi::SignVirama => "gurmukhi sign virama",
            Gurmukhi::SignUdaat => "gurmukhi sign udaat",
            Gurmukhi::LetterKhha => "gurmukhi letter khha",
            Gurmukhi::LetterGhha => "gurmukhi letter ghha",
            Gurmukhi::LetterZa => "gurmukhi letter za",
            Gurmukhi::LetterRra => "gurmukhi letter rra",
            Gurmukhi::LetterFa => "gurmukhi letter fa",
            Gurmukhi::DigitZero => "gurmukhi digit zero",
            Gurmukhi::DigitOne => "gurmukhi digit one",
            Gurmukhi::DigitTwo => "gurmukhi digit two",
            Gurmukhi::DigitThree => "gurmukhi digit three",
            Gurmukhi::DigitFour => "gurmukhi digit four",
            Gurmukhi::DigitFive => "gurmukhi digit five",
            Gurmukhi::DigitSix => "gurmukhi digit six",
            Gurmukhi::DigitSeven => "gurmukhi digit seven",
            Gurmukhi::DigitEight => "gurmukhi digit eight",
            Gurmukhi::DigitNine => "gurmukhi digit nine",
            Gurmukhi::Tippi => "gurmukhi tippi",
            Gurmukhi::Addak => "gurmukhi addak",
            Gurmukhi::Iri => "gurmukhi iri",
            Gurmukhi::Ura => "gurmukhi ura",
            Gurmukhi::EkOnkar => "gurmukhi ek onkar",
            Gurmukhi::SignYakash => "gurmukhi sign yakash",
            Gurmukhi::AbbreviationSign => "gurmukhi abbreviation sign",
        }
    }
}
