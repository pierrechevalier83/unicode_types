
/// An enum to represent all characters in the Javanese block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Javanese {
    /// \u{a980}: 'ꦀ'
    SignPanyangga,
    /// \u{a981}: 'ꦁ'
    SignCecak,
    /// \u{a982}: 'ꦂ'
    SignLayar,
    /// \u{a983}: 'ꦃ'
    SignWignyan,
    /// \u{a984}: 'ꦄ'
    LetterA,
    /// \u{a985}: 'ꦅ'
    LetterIKawi,
    /// \u{a986}: 'ꦆ'
    LetterI,
    /// \u{a987}: 'ꦇ'
    LetterIi,
    /// \u{a988}: 'ꦈ'
    LetterU,
    /// \u{a989}: 'ꦉ'
    LetterPaCerek,
    /// \u{a98a}: 'ꦊ'
    LetterNgaLelet,
    /// \u{a98b}: 'ꦋ'
    LetterNgaLeletRaswadi,
    /// \u{a98c}: 'ꦌ'
    LetterE,
    /// \u{a98d}: 'ꦍ'
    LetterAi,
    /// \u{a98e}: 'ꦎ'
    LetterO,
    /// \u{a98f}: 'ꦏ'
    LetterKa,
    /// \u{a990}: 'ꦐ'
    LetterKaSasak,
    /// \u{a991}: 'ꦑ'
    LetterKaMurda,
    /// \u{a992}: 'ꦒ'
    LetterGa,
    /// \u{a993}: 'ꦓ'
    LetterGaMurda,
    /// \u{a994}: 'ꦔ'
    LetterNga,
    /// \u{a995}: 'ꦕ'
    LetterCa,
    /// \u{a996}: 'ꦖ'
    LetterCaMurda,
    /// \u{a997}: 'ꦗ'
    LetterJa,
    /// \u{a998}: 'ꦘ'
    LetterNyaMurda,
    /// \u{a999}: 'ꦙ'
    LetterJaMahaprana,
    /// \u{a99a}: 'ꦚ'
    LetterNya,
    /// \u{a99b}: 'ꦛ'
    LetterTta,
    /// \u{a99c}: 'ꦜ'
    LetterTtaMahaprana,
    /// \u{a99d}: 'ꦝ'
    LetterDda,
    /// \u{a99e}: 'ꦞ'
    LetterDdaMahaprana,
    /// \u{a99f}: 'ꦟ'
    LetterNaMurda,
    /// \u{a9a0}: 'ꦠ'
    LetterTa,
    /// \u{a9a1}: 'ꦡ'
    LetterTaMurda,
    /// \u{a9a2}: 'ꦢ'
    LetterDa,
    /// \u{a9a3}: 'ꦣ'
    LetterDaMahaprana,
    /// \u{a9a4}: 'ꦤ'
    LetterNa,
    /// \u{a9a5}: 'ꦥ'
    LetterPa,
    /// \u{a9a6}: 'ꦦ'
    LetterPaMurda,
    /// \u{a9a7}: 'ꦧ'
    LetterBa,
    /// \u{a9a8}: 'ꦨ'
    LetterBaMurda,
    /// \u{a9a9}: 'ꦩ'
    LetterMa,
    /// \u{a9aa}: 'ꦪ'
    LetterYa,
    /// \u{a9ab}: 'ꦫ'
    LetterRa,
    /// \u{a9ac}: 'ꦬ'
    LetterRaAgung,
    /// \u{a9ad}: 'ꦭ'
    LetterLa,
    /// \u{a9ae}: 'ꦮ'
    LetterWa,
    /// \u{a9af}: 'ꦯ'
    LetterSaMurda,
    /// \u{a9b0}: 'ꦰ'
    LetterSaMahaprana,
    /// \u{a9b1}: 'ꦱ'
    LetterSa,
    /// \u{a9b2}: 'ꦲ'
    LetterHa,
    /// \u{a9b3}: '꦳'
    SignCecakTelu,
    /// \u{a9b4}: 'ꦴ'
    VowelSignTarung,
    /// \u{a9b5}: 'ꦵ'
    VowelSignTolong,
    /// \u{a9b6}: 'ꦶ'
    VowelSignWulu,
    /// \u{a9b7}: 'ꦷ'
    VowelSignWuluMelik,
    /// \u{a9b8}: 'ꦸ'
    VowelSignSuku,
    /// \u{a9b9}: 'ꦹ'
    VowelSignSukuMendut,
    /// \u{a9ba}: 'ꦺ'
    VowelSignTaling,
    /// \u{a9bb}: 'ꦻ'
    VowelSignDirgaMure,
    /// \u{a9bc}: 'ꦼ'
    VowelSignPepet,
    /// \u{a9bd}: 'ꦽ'
    ConsonantSignKeret,
    /// \u{a9be}: 'ꦾ'
    ConsonantSignPengkal,
    /// \u{a9bf}: 'ꦿ'
    ConsonantSignCakra,
    /// \u{a9c0}: '꧀'
    Pangkon,
    /// \u{a9c1}: '꧁'
    LeftRerenggan,
    /// \u{a9c2}: '꧂'
    RightRerenggan,
    /// \u{a9c3}: '꧃'
    PadaAndap,
    /// \u{a9c4}: '꧄'
    PadaMadya,
    /// \u{a9c5}: '꧅'
    PadaLuhur,
    /// \u{a9c6}: '꧆'
    PadaWindu,
    /// \u{a9c7}: '꧇'
    PadaPangkat,
    /// \u{a9c8}: '꧈'
    PadaLingsa,
    /// \u{a9c9}: '꧉'
    PadaLungsi,
    /// \u{a9ca}: '꧊'
    PadaAdeg,
    /// \u{a9cb}: '꧋'
    PadaAdegAdeg,
    /// \u{a9cc}: '꧌'
    PadaPiseleh,
    /// \u{a9cd}: '꧍'
    TurnedPadaPiseleh,
    /// \u{a9cf}: 'ꧏ'
    Pangrangkep,
    /// \u{a9d0}: '꧐'
    DigitZero,
    /// \u{a9d1}: '꧑'
    DigitOne,
    /// \u{a9d2}: '꧒'
    DigitTwo,
    /// \u{a9d3}: '꧓'
    DigitThree,
    /// \u{a9d4}: '꧔'
    DigitFour,
    /// \u{a9d5}: '꧕'
    DigitFive,
    /// \u{a9d6}: '꧖'
    DigitSix,
    /// \u{a9d7}: '꧗'
    DigitSeven,
    /// \u{a9d8}: '꧘'
    DigitEight,
    /// \u{a9d9}: '꧙'
    DigitNine,
    /// \u{a9de}: '꧞'
    PadaTirtaTumetes,
}

impl Into<char> for Javanese {
    fn into(self) -> char {
        match self {
            Javanese::SignPanyangga => 'ꦀ',
            Javanese::SignCecak => 'ꦁ',
            Javanese::SignLayar => 'ꦂ',
            Javanese::SignWignyan => 'ꦃ',
            Javanese::LetterA => 'ꦄ',
            Javanese::LetterIKawi => 'ꦅ',
            Javanese::LetterI => 'ꦆ',
            Javanese::LetterIi => 'ꦇ',
            Javanese::LetterU => 'ꦈ',
            Javanese::LetterPaCerek => 'ꦉ',
            Javanese::LetterNgaLelet => 'ꦊ',
            Javanese::LetterNgaLeletRaswadi => 'ꦋ',
            Javanese::LetterE => 'ꦌ',
            Javanese::LetterAi => 'ꦍ',
            Javanese::LetterO => 'ꦎ',
            Javanese::LetterKa => 'ꦏ',
            Javanese::LetterKaSasak => 'ꦐ',
            Javanese::LetterKaMurda => 'ꦑ',
            Javanese::LetterGa => 'ꦒ',
            Javanese::LetterGaMurda => 'ꦓ',
            Javanese::LetterNga => 'ꦔ',
            Javanese::LetterCa => 'ꦕ',
            Javanese::LetterCaMurda => 'ꦖ',
            Javanese::LetterJa => 'ꦗ',
            Javanese::LetterNyaMurda => 'ꦘ',
            Javanese::LetterJaMahaprana => 'ꦙ',
            Javanese::LetterNya => 'ꦚ',
            Javanese::LetterTta => 'ꦛ',
            Javanese::LetterTtaMahaprana => 'ꦜ',
            Javanese::LetterDda => 'ꦝ',
            Javanese::LetterDdaMahaprana => 'ꦞ',
            Javanese::LetterNaMurda => 'ꦟ',
            Javanese::LetterTa => 'ꦠ',
            Javanese::LetterTaMurda => 'ꦡ',
            Javanese::LetterDa => 'ꦢ',
            Javanese::LetterDaMahaprana => 'ꦣ',
            Javanese::LetterNa => 'ꦤ',
            Javanese::LetterPa => 'ꦥ',
            Javanese::LetterPaMurda => 'ꦦ',
            Javanese::LetterBa => 'ꦧ',
            Javanese::LetterBaMurda => 'ꦨ',
            Javanese::LetterMa => 'ꦩ',
            Javanese::LetterYa => 'ꦪ',
            Javanese::LetterRa => 'ꦫ',
            Javanese::LetterRaAgung => 'ꦬ',
            Javanese::LetterLa => 'ꦭ',
            Javanese::LetterWa => 'ꦮ',
            Javanese::LetterSaMurda => 'ꦯ',
            Javanese::LetterSaMahaprana => 'ꦰ',
            Javanese::LetterSa => 'ꦱ',
            Javanese::LetterHa => 'ꦲ',
            Javanese::SignCecakTelu => '꦳',
            Javanese::VowelSignTarung => 'ꦴ',
            Javanese::VowelSignTolong => 'ꦵ',
            Javanese::VowelSignWulu => 'ꦶ',
            Javanese::VowelSignWuluMelik => 'ꦷ',
            Javanese::VowelSignSuku => 'ꦸ',
            Javanese::VowelSignSukuMendut => 'ꦹ',
            Javanese::VowelSignTaling => 'ꦺ',
            Javanese::VowelSignDirgaMure => 'ꦻ',
            Javanese::VowelSignPepet => 'ꦼ',
            Javanese::ConsonantSignKeret => 'ꦽ',
            Javanese::ConsonantSignPengkal => 'ꦾ',
            Javanese::ConsonantSignCakra => 'ꦿ',
            Javanese::Pangkon => '꧀',
            Javanese::LeftRerenggan => '꧁',
            Javanese::RightRerenggan => '꧂',
            Javanese::PadaAndap => '꧃',
            Javanese::PadaMadya => '꧄',
            Javanese::PadaLuhur => '꧅',
            Javanese::PadaWindu => '꧆',
            Javanese::PadaPangkat => '꧇',
            Javanese::PadaLingsa => '꧈',
            Javanese::PadaLungsi => '꧉',
            Javanese::PadaAdeg => '꧊',
            Javanese::PadaAdegAdeg => '꧋',
            Javanese::PadaPiseleh => '꧌',
            Javanese::TurnedPadaPiseleh => '꧍',
            Javanese::Pangrangkep => 'ꧏ',
            Javanese::DigitZero => '꧐',
            Javanese::DigitOne => '꧑',
            Javanese::DigitTwo => '꧒',
            Javanese::DigitThree => '꧓',
            Javanese::DigitFour => '꧔',
            Javanese::DigitFive => '꧕',
            Javanese::DigitSix => '꧖',
            Javanese::DigitSeven => '꧗',
            Javanese::DigitEight => '꧘',
            Javanese::DigitNine => '꧙',
            Javanese::PadaTirtaTumetes => '꧞',
        }
    }
}

impl std::convert::TryFrom<char> for Javanese {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ꦀ' => Ok(Javanese::SignPanyangga),
            'ꦁ' => Ok(Javanese::SignCecak),
            'ꦂ' => Ok(Javanese::SignLayar),
            'ꦃ' => Ok(Javanese::SignWignyan),
            'ꦄ' => Ok(Javanese::LetterA),
            'ꦅ' => Ok(Javanese::LetterIKawi),
            'ꦆ' => Ok(Javanese::LetterI),
            'ꦇ' => Ok(Javanese::LetterIi),
            'ꦈ' => Ok(Javanese::LetterU),
            'ꦉ' => Ok(Javanese::LetterPaCerek),
            'ꦊ' => Ok(Javanese::LetterNgaLelet),
            'ꦋ' => Ok(Javanese::LetterNgaLeletRaswadi),
            'ꦌ' => Ok(Javanese::LetterE),
            'ꦍ' => Ok(Javanese::LetterAi),
            'ꦎ' => Ok(Javanese::LetterO),
            'ꦏ' => Ok(Javanese::LetterKa),
            'ꦐ' => Ok(Javanese::LetterKaSasak),
            'ꦑ' => Ok(Javanese::LetterKaMurda),
            'ꦒ' => Ok(Javanese::LetterGa),
            'ꦓ' => Ok(Javanese::LetterGaMurda),
            'ꦔ' => Ok(Javanese::LetterNga),
            'ꦕ' => Ok(Javanese::LetterCa),
            'ꦖ' => Ok(Javanese::LetterCaMurda),
            'ꦗ' => Ok(Javanese::LetterJa),
            'ꦘ' => Ok(Javanese::LetterNyaMurda),
            'ꦙ' => Ok(Javanese::LetterJaMahaprana),
            'ꦚ' => Ok(Javanese::LetterNya),
            'ꦛ' => Ok(Javanese::LetterTta),
            'ꦜ' => Ok(Javanese::LetterTtaMahaprana),
            'ꦝ' => Ok(Javanese::LetterDda),
            'ꦞ' => Ok(Javanese::LetterDdaMahaprana),
            'ꦟ' => Ok(Javanese::LetterNaMurda),
            'ꦠ' => Ok(Javanese::LetterTa),
            'ꦡ' => Ok(Javanese::LetterTaMurda),
            'ꦢ' => Ok(Javanese::LetterDa),
            'ꦣ' => Ok(Javanese::LetterDaMahaprana),
            'ꦤ' => Ok(Javanese::LetterNa),
            'ꦥ' => Ok(Javanese::LetterPa),
            'ꦦ' => Ok(Javanese::LetterPaMurda),
            'ꦧ' => Ok(Javanese::LetterBa),
            'ꦨ' => Ok(Javanese::LetterBaMurda),
            'ꦩ' => Ok(Javanese::LetterMa),
            'ꦪ' => Ok(Javanese::LetterYa),
            'ꦫ' => Ok(Javanese::LetterRa),
            'ꦬ' => Ok(Javanese::LetterRaAgung),
            'ꦭ' => Ok(Javanese::LetterLa),
            'ꦮ' => Ok(Javanese::LetterWa),
            'ꦯ' => Ok(Javanese::LetterSaMurda),
            'ꦰ' => Ok(Javanese::LetterSaMahaprana),
            'ꦱ' => Ok(Javanese::LetterSa),
            'ꦲ' => Ok(Javanese::LetterHa),
            '꦳' => Ok(Javanese::SignCecakTelu),
            'ꦴ' => Ok(Javanese::VowelSignTarung),
            'ꦵ' => Ok(Javanese::VowelSignTolong),
            'ꦶ' => Ok(Javanese::VowelSignWulu),
            'ꦷ' => Ok(Javanese::VowelSignWuluMelik),
            'ꦸ' => Ok(Javanese::VowelSignSuku),
            'ꦹ' => Ok(Javanese::VowelSignSukuMendut),
            'ꦺ' => Ok(Javanese::VowelSignTaling),
            'ꦻ' => Ok(Javanese::VowelSignDirgaMure),
            'ꦼ' => Ok(Javanese::VowelSignPepet),
            'ꦽ' => Ok(Javanese::ConsonantSignKeret),
            'ꦾ' => Ok(Javanese::ConsonantSignPengkal),
            'ꦿ' => Ok(Javanese::ConsonantSignCakra),
            '꧀' => Ok(Javanese::Pangkon),
            '꧁' => Ok(Javanese::LeftRerenggan),
            '꧂' => Ok(Javanese::RightRerenggan),
            '꧃' => Ok(Javanese::PadaAndap),
            '꧄' => Ok(Javanese::PadaMadya),
            '꧅' => Ok(Javanese::PadaLuhur),
            '꧆' => Ok(Javanese::PadaWindu),
            '꧇' => Ok(Javanese::PadaPangkat),
            '꧈' => Ok(Javanese::PadaLingsa),
            '꧉' => Ok(Javanese::PadaLungsi),
            '꧊' => Ok(Javanese::PadaAdeg),
            '꧋' => Ok(Javanese::PadaAdegAdeg),
            '꧌' => Ok(Javanese::PadaPiseleh),
            '꧍' => Ok(Javanese::TurnedPadaPiseleh),
            'ꧏ' => Ok(Javanese::Pangrangkep),
            '꧐' => Ok(Javanese::DigitZero),
            '꧑' => Ok(Javanese::DigitOne),
            '꧒' => Ok(Javanese::DigitTwo),
            '꧓' => Ok(Javanese::DigitThree),
            '꧔' => Ok(Javanese::DigitFour),
            '꧕' => Ok(Javanese::DigitFive),
            '꧖' => Ok(Javanese::DigitSix),
            '꧗' => Ok(Javanese::DigitSeven),
            '꧘' => Ok(Javanese::DigitEight),
            '꧙' => Ok(Javanese::DigitNine),
            '꧞' => Ok(Javanese::PadaTirtaTumetes),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Javanese {
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

impl std::convert::TryFrom<u32> for Javanese {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Javanese {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Javanese {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Javanese::SignPanyangga
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Javanese{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
