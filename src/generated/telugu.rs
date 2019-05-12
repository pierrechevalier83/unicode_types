
/// An enum to represent all characters in the Telugu block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Telugu {
    /// \u{c00}: 'ఀ'
    SignCombiningCandrabinduAbove,
    /// \u{c01}: 'ఁ'
    SignCandrabindu,
    /// \u{c02}: 'ం'
    SignAnusvara,
    /// \u{c03}: 'ః'
    SignVisarga,
    /// \u{c04}: 'ఄ'
    SignCombiningAnusvaraAbove,
    /// \u{c05}: 'అ'
    LetterA,
    /// \u{c06}: 'ఆ'
    LetterAa,
    /// \u{c07}: 'ఇ'
    LetterI,
    /// \u{c08}: 'ఈ'
    LetterIi,
    /// \u{c09}: 'ఉ'
    LetterU,
    /// \u{c0a}: 'ఊ'
    LetterUu,
    /// \u{c0b}: 'ఋ'
    LetterVocalicR,
    /// \u{c0c}: 'ఌ'
    LetterVocalicL,
    /// \u{c0e}: 'ఎ'
    LetterE,
    /// \u{c0f}: 'ఏ'
    LetterEe,
    /// \u{c10}: 'ఐ'
    LetterAi,
    /// \u{c12}: 'ఒ'
    LetterO,
    /// \u{c13}: 'ఓ'
    LetterOo,
    /// \u{c14}: 'ఔ'
    LetterAu,
    /// \u{c15}: 'క'
    LetterKa,
    /// \u{c16}: 'ఖ'
    LetterKha,
    /// \u{c17}: 'గ'
    LetterGa,
    /// \u{c18}: 'ఘ'
    LetterGha,
    /// \u{c19}: 'ఙ'
    LetterNga,
    /// \u{c1a}: 'చ'
    LetterCa,
    /// \u{c1b}: 'ఛ'
    LetterCha,
    /// \u{c1c}: 'జ'
    LetterJa,
    /// \u{c1d}: 'ఝ'
    LetterJha,
    /// \u{c1e}: 'ఞ'
    LetterNya,
    /// \u{c1f}: 'ట'
    LetterTta,
    /// \u{c20}: 'ఠ'
    LetterTtha,
    /// \u{c21}: 'డ'
    LetterDda,
    /// \u{c22}: 'ఢ'
    LetterDdha,
    /// \u{c23}: 'ణ'
    LetterNna,
    /// \u{c24}: 'త'
    LetterTa,
    /// \u{c25}: 'థ'
    LetterTha,
    /// \u{c26}: 'ద'
    LetterDa,
    /// \u{c27}: 'ధ'
    LetterDha,
    /// \u{c28}: 'న'
    LetterNa,
    /// \u{c2a}: 'ప'
    LetterPa,
    /// \u{c2b}: 'ఫ'
    LetterPha,
    /// \u{c2c}: 'బ'
    LetterBa,
    /// \u{c2d}: 'భ'
    LetterBha,
    /// \u{c2e}: 'మ'
    LetterMa,
    /// \u{c2f}: 'య'
    LetterYa,
    /// \u{c30}: 'ర'
    LetterRa,
    /// \u{c31}: 'ఱ'
    LetterRra,
    /// \u{c32}: 'ల'
    LetterLa,
    /// \u{c33}: 'ళ'
    LetterLla,
    /// \u{c34}: 'ఴ'
    LetterLlla,
    /// \u{c35}: 'వ'
    LetterVa,
    /// \u{c36}: 'శ'
    LetterSha,
    /// \u{c37}: 'ష'
    LetterSsa,
    /// \u{c38}: 'స'
    LetterSa,
    /// \u{c39}: 'హ'
    LetterHa,
    /// \u{c3d}: 'ఽ'
    SignAvagraha,
    /// \u{c3e}: 'ా'
    VowelSignAa,
    /// \u{c3f}: 'ి'
    VowelSignI,
    /// \u{c40}: 'ీ'
    VowelSignIi,
    /// \u{c41}: 'ు'
    VowelSignU,
    /// \u{c42}: 'ూ'
    VowelSignUu,
    /// \u{c43}: 'ృ'
    VowelSignVocalicR,
    /// \u{c44}: 'ౄ'
    VowelSignVocalicRr,
    /// \u{c46}: 'ె'
    VowelSignE,
    /// \u{c47}: 'ే'
    VowelSignEe,
    /// \u{c48}: 'ై'
    VowelSignAi,
    /// \u{c4a}: 'ొ'
    VowelSignO,
    /// \u{c4b}: 'ో'
    VowelSignOo,
    /// \u{c4c}: 'ౌ'
    VowelSignAu,
    /// \u{c4d}: '్'
    SignVirama,
    /// \u{c55}: 'ౕ'
    LengthMark,
    /// \u{c56}: 'ౖ'
    AiLengthMark,
    /// \u{c58}: 'ౘ'
    LetterTsa,
    /// \u{c59}: 'ౙ'
    LetterDza,
    /// \u{c5a}: 'ౚ'
    LetterRrra,
    /// \u{c60}: 'ౠ'
    LetterVocalicRr,
    /// \u{c61}: 'ౡ'
    LetterVocalicLl,
    /// \u{c62}: 'ౢ'
    VowelSignVocalicL,
    /// \u{c63}: 'ౣ'
    VowelSignVocalicLl,
    /// \u{c66}: '౦'
    DigitZero,
    /// \u{c67}: '౧'
    DigitOne,
    /// \u{c68}: '౨'
    DigitTwo,
    /// \u{c69}: '౩'
    DigitThree,
    /// \u{c6a}: '౪'
    DigitFour,
    /// \u{c6b}: '౫'
    DigitFive,
    /// \u{c6c}: '౬'
    DigitSix,
    /// \u{c6d}: '౭'
    DigitSeven,
    /// \u{c6e}: '౮'
    DigitEight,
    /// \u{c6f}: '౯'
    DigitNine,
    /// \u{c77}: '౷'
    SignSiddham,
    /// \u{c78}: '౸'
    FractionDigitZeroForOddPowersOfFour,
    /// \u{c79}: '౹'
    FractionDigitOneForOddPowersOfFour,
    /// \u{c7a}: '౺'
    FractionDigitTwoForOddPowersOfFour,
    /// \u{c7b}: '౻'
    FractionDigitThreeForOddPowersOfFour,
    /// \u{c7c}: '౼'
    FractionDigitOneForEvenPowersOfFour,
    /// \u{c7d}: '౽'
    FractionDigitTwoForEvenPowersOfFour,
    /// \u{c7e}: '౾'
    FractionDigitThreeForEvenPowersOfFour,
}

impl Into<char> for Telugu {
    fn into(self) -> char {
        match self {
            Telugu::SignCombiningCandrabinduAbove => 'ఀ',
            Telugu::SignCandrabindu => 'ఁ',
            Telugu::SignAnusvara => 'ం',
            Telugu::SignVisarga => 'ః',
            Telugu::SignCombiningAnusvaraAbove => 'ఄ',
            Telugu::LetterA => 'అ',
            Telugu::LetterAa => 'ఆ',
            Telugu::LetterI => 'ఇ',
            Telugu::LetterIi => 'ఈ',
            Telugu::LetterU => 'ఉ',
            Telugu::LetterUu => 'ఊ',
            Telugu::LetterVocalicR => 'ఋ',
            Telugu::LetterVocalicL => 'ఌ',
            Telugu::LetterE => 'ఎ',
            Telugu::LetterEe => 'ఏ',
            Telugu::LetterAi => 'ఐ',
            Telugu::LetterO => 'ఒ',
            Telugu::LetterOo => 'ఓ',
            Telugu::LetterAu => 'ఔ',
            Telugu::LetterKa => 'క',
            Telugu::LetterKha => 'ఖ',
            Telugu::LetterGa => 'గ',
            Telugu::LetterGha => 'ఘ',
            Telugu::LetterNga => 'ఙ',
            Telugu::LetterCa => 'చ',
            Telugu::LetterCha => 'ఛ',
            Telugu::LetterJa => 'జ',
            Telugu::LetterJha => 'ఝ',
            Telugu::LetterNya => 'ఞ',
            Telugu::LetterTta => 'ట',
            Telugu::LetterTtha => 'ఠ',
            Telugu::LetterDda => 'డ',
            Telugu::LetterDdha => 'ఢ',
            Telugu::LetterNna => 'ణ',
            Telugu::LetterTa => 'త',
            Telugu::LetterTha => 'థ',
            Telugu::LetterDa => 'ద',
            Telugu::LetterDha => 'ధ',
            Telugu::LetterNa => 'న',
            Telugu::LetterPa => 'ప',
            Telugu::LetterPha => 'ఫ',
            Telugu::LetterBa => 'బ',
            Telugu::LetterBha => 'భ',
            Telugu::LetterMa => 'మ',
            Telugu::LetterYa => 'య',
            Telugu::LetterRa => 'ర',
            Telugu::LetterRra => 'ఱ',
            Telugu::LetterLa => 'ల',
            Telugu::LetterLla => 'ళ',
            Telugu::LetterLlla => 'ఴ',
            Telugu::LetterVa => 'వ',
            Telugu::LetterSha => 'శ',
            Telugu::LetterSsa => 'ష',
            Telugu::LetterSa => 'స',
            Telugu::LetterHa => 'హ',
            Telugu::SignAvagraha => 'ఽ',
            Telugu::VowelSignAa => 'ా',
            Telugu::VowelSignI => 'ి',
            Telugu::VowelSignIi => 'ీ',
            Telugu::VowelSignU => 'ు',
            Telugu::VowelSignUu => 'ూ',
            Telugu::VowelSignVocalicR => 'ృ',
            Telugu::VowelSignVocalicRr => 'ౄ',
            Telugu::VowelSignE => 'ె',
            Telugu::VowelSignEe => 'ే',
            Telugu::VowelSignAi => 'ై',
            Telugu::VowelSignO => 'ొ',
            Telugu::VowelSignOo => 'ో',
            Telugu::VowelSignAu => 'ౌ',
            Telugu::SignVirama => '్',
            Telugu::LengthMark => 'ౕ',
            Telugu::AiLengthMark => 'ౖ',
            Telugu::LetterTsa => 'ౘ',
            Telugu::LetterDza => 'ౙ',
            Telugu::LetterRrra => 'ౚ',
            Telugu::LetterVocalicRr => 'ౠ',
            Telugu::LetterVocalicLl => 'ౡ',
            Telugu::VowelSignVocalicL => 'ౢ',
            Telugu::VowelSignVocalicLl => 'ౣ',
            Telugu::DigitZero => '౦',
            Telugu::DigitOne => '౧',
            Telugu::DigitTwo => '౨',
            Telugu::DigitThree => '౩',
            Telugu::DigitFour => '౪',
            Telugu::DigitFive => '౫',
            Telugu::DigitSix => '౬',
            Telugu::DigitSeven => '౭',
            Telugu::DigitEight => '౮',
            Telugu::DigitNine => '౯',
            Telugu::SignSiddham => '౷',
            Telugu::FractionDigitZeroForOddPowersOfFour => '౸',
            Telugu::FractionDigitOneForOddPowersOfFour => '౹',
            Telugu::FractionDigitTwoForOddPowersOfFour => '౺',
            Telugu::FractionDigitThreeForOddPowersOfFour => '౻',
            Telugu::FractionDigitOneForEvenPowersOfFour => '౼',
            Telugu::FractionDigitTwoForEvenPowersOfFour => '౽',
            Telugu::FractionDigitThreeForEvenPowersOfFour => '౾',
        }
    }
}

impl std::convert::TryFrom<char> for Telugu {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ఀ' => Ok(Telugu::SignCombiningCandrabinduAbove),
            'ఁ' => Ok(Telugu::SignCandrabindu),
            'ం' => Ok(Telugu::SignAnusvara),
            'ః' => Ok(Telugu::SignVisarga),
            'ఄ' => Ok(Telugu::SignCombiningAnusvaraAbove),
            'అ' => Ok(Telugu::LetterA),
            'ఆ' => Ok(Telugu::LetterAa),
            'ఇ' => Ok(Telugu::LetterI),
            'ఈ' => Ok(Telugu::LetterIi),
            'ఉ' => Ok(Telugu::LetterU),
            'ఊ' => Ok(Telugu::LetterUu),
            'ఋ' => Ok(Telugu::LetterVocalicR),
            'ఌ' => Ok(Telugu::LetterVocalicL),
            'ఎ' => Ok(Telugu::LetterE),
            'ఏ' => Ok(Telugu::LetterEe),
            'ఐ' => Ok(Telugu::LetterAi),
            'ఒ' => Ok(Telugu::LetterO),
            'ఓ' => Ok(Telugu::LetterOo),
            'ఔ' => Ok(Telugu::LetterAu),
            'క' => Ok(Telugu::LetterKa),
            'ఖ' => Ok(Telugu::LetterKha),
            'గ' => Ok(Telugu::LetterGa),
            'ఘ' => Ok(Telugu::LetterGha),
            'ఙ' => Ok(Telugu::LetterNga),
            'చ' => Ok(Telugu::LetterCa),
            'ఛ' => Ok(Telugu::LetterCha),
            'జ' => Ok(Telugu::LetterJa),
            'ఝ' => Ok(Telugu::LetterJha),
            'ఞ' => Ok(Telugu::LetterNya),
            'ట' => Ok(Telugu::LetterTta),
            'ఠ' => Ok(Telugu::LetterTtha),
            'డ' => Ok(Telugu::LetterDda),
            'ఢ' => Ok(Telugu::LetterDdha),
            'ణ' => Ok(Telugu::LetterNna),
            'త' => Ok(Telugu::LetterTa),
            'థ' => Ok(Telugu::LetterTha),
            'ద' => Ok(Telugu::LetterDa),
            'ధ' => Ok(Telugu::LetterDha),
            'న' => Ok(Telugu::LetterNa),
            'ప' => Ok(Telugu::LetterPa),
            'ఫ' => Ok(Telugu::LetterPha),
            'బ' => Ok(Telugu::LetterBa),
            'భ' => Ok(Telugu::LetterBha),
            'మ' => Ok(Telugu::LetterMa),
            'య' => Ok(Telugu::LetterYa),
            'ర' => Ok(Telugu::LetterRa),
            'ఱ' => Ok(Telugu::LetterRra),
            'ల' => Ok(Telugu::LetterLa),
            'ళ' => Ok(Telugu::LetterLla),
            'ఴ' => Ok(Telugu::LetterLlla),
            'వ' => Ok(Telugu::LetterVa),
            'శ' => Ok(Telugu::LetterSha),
            'ష' => Ok(Telugu::LetterSsa),
            'స' => Ok(Telugu::LetterSa),
            'హ' => Ok(Telugu::LetterHa),
            'ఽ' => Ok(Telugu::SignAvagraha),
            'ా' => Ok(Telugu::VowelSignAa),
            'ి' => Ok(Telugu::VowelSignI),
            'ీ' => Ok(Telugu::VowelSignIi),
            'ు' => Ok(Telugu::VowelSignU),
            'ూ' => Ok(Telugu::VowelSignUu),
            'ృ' => Ok(Telugu::VowelSignVocalicR),
            'ౄ' => Ok(Telugu::VowelSignVocalicRr),
            'ె' => Ok(Telugu::VowelSignE),
            'ే' => Ok(Telugu::VowelSignEe),
            'ై' => Ok(Telugu::VowelSignAi),
            'ొ' => Ok(Telugu::VowelSignO),
            'ో' => Ok(Telugu::VowelSignOo),
            'ౌ' => Ok(Telugu::VowelSignAu),
            '్' => Ok(Telugu::SignVirama),
            'ౕ' => Ok(Telugu::LengthMark),
            'ౖ' => Ok(Telugu::AiLengthMark),
            'ౘ' => Ok(Telugu::LetterTsa),
            'ౙ' => Ok(Telugu::LetterDza),
            'ౚ' => Ok(Telugu::LetterRrra),
            'ౠ' => Ok(Telugu::LetterVocalicRr),
            'ౡ' => Ok(Telugu::LetterVocalicLl),
            'ౢ' => Ok(Telugu::VowelSignVocalicL),
            'ౣ' => Ok(Telugu::VowelSignVocalicLl),
            '౦' => Ok(Telugu::DigitZero),
            '౧' => Ok(Telugu::DigitOne),
            '౨' => Ok(Telugu::DigitTwo),
            '౩' => Ok(Telugu::DigitThree),
            '౪' => Ok(Telugu::DigitFour),
            '౫' => Ok(Telugu::DigitFive),
            '౬' => Ok(Telugu::DigitSix),
            '౭' => Ok(Telugu::DigitSeven),
            '౮' => Ok(Telugu::DigitEight),
            '౯' => Ok(Telugu::DigitNine),
            '౷' => Ok(Telugu::SignSiddham),
            '౸' => Ok(Telugu::FractionDigitZeroForOddPowersOfFour),
            '౹' => Ok(Telugu::FractionDigitOneForOddPowersOfFour),
            '౺' => Ok(Telugu::FractionDigitTwoForOddPowersOfFour),
            '౻' => Ok(Telugu::FractionDigitThreeForOddPowersOfFour),
            '౼' => Ok(Telugu::FractionDigitOneForEvenPowersOfFour),
            '౽' => Ok(Telugu::FractionDigitTwoForEvenPowersOfFour),
            '౾' => Ok(Telugu::FractionDigitThreeForEvenPowersOfFour),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Telugu {
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

impl std::convert::TryFrom<u32> for Telugu {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Telugu {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Telugu {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Telugu::SignCombiningCandrabinduAbove
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Telugu{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
