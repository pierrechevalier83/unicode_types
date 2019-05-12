
/// An enum to represent all characters in the Lao block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Lao {
    /// \u{e81}: 'ກ'
    LetterKo,
    /// \u{e82}: 'ຂ'
    LetterKhoSung,
    /// \u{e84}: 'ຄ'
    LetterKhoTam,
    /// \u{e86}: 'ຆ'
    LetterPaliGha,
    /// \u{e87}: 'ງ'
    LetterNgo,
    /// \u{e88}: 'ຈ'
    LetterCo,
    /// \u{e89}: 'ຉ'
    LetterPaliCha,
    /// \u{e8a}: 'ຊ'
    LetterSoTam,
    /// \u{e8c}: 'ຌ'
    LetterPaliJha,
    /// \u{e8d}: 'ຍ'
    LetterNyo,
    /// \u{e8e}: 'ຎ'
    LetterPaliNya,
    /// \u{e8f}: 'ຏ'
    LetterPaliTta,
    /// \u{e90}: 'ຐ'
    LetterPaliTtha,
    /// \u{e91}: 'ຑ'
    LetterPaliDda,
    /// \u{e92}: 'ຒ'
    LetterPaliDdha,
    /// \u{e93}: 'ຓ'
    LetterPaliNna,
    /// \u{e94}: 'ດ'
    LetterDo,
    /// \u{e95}: 'ຕ'
    LetterTo,
    /// \u{e96}: 'ຖ'
    LetterThoSung,
    /// \u{e97}: 'ທ'
    LetterThoTam,
    /// \u{e98}: 'ຘ'
    LetterPaliDha,
    /// \u{e99}: 'ນ'
    LetterNo,
    /// \u{e9a}: 'ບ'
    LetterBo,
    /// \u{e9b}: 'ປ'
    LetterPo,
    /// \u{e9c}: 'ຜ'
    LetterPhoSung,
    /// \u{e9d}: 'ຝ'
    LetterFoTam,
    /// \u{e9e}: 'ພ'
    LetterPhoTam,
    /// \u{e9f}: 'ຟ'
    LetterFoSung,
    /// \u{ea0}: 'ຠ'
    LetterPaliBha,
    /// \u{ea1}: 'ມ'
    LetterMo,
    /// \u{ea2}: 'ຢ'
    LetterYo,
    /// \u{ea3}: 'ຣ'
    LetterLoLing,
    /// \u{ea5}: 'ລ'
    LetterLoLoot,
    /// \u{ea7}: 'ວ'
    LetterWo,
    /// \u{ea8}: 'ຨ'
    LetterSanskritSha,
    /// \u{ea9}: 'ຩ'
    LetterSanskritSsa,
    /// \u{eaa}: 'ສ'
    LetterSoSung,
    /// \u{eab}: 'ຫ'
    LetterHoSung,
    /// \u{eac}: 'ຬ'
    LetterPaliLla,
    /// \u{ead}: 'ອ'
    LetterO,
    /// \u{eae}: 'ຮ'
    LetterHoTam,
    /// \u{eaf}: 'ຯ'
    Ellipsis,
    /// \u{eb0}: 'ະ'
    VowelSignA,
    /// \u{eb1}: 'ັ'
    VowelSignMaiKan,
    /// \u{eb2}: 'າ'
    VowelSignAa,
    /// \u{eb3}: 'ຳ'
    VowelSignAm,
    /// \u{eb4}: 'ິ'
    VowelSignI,
    /// \u{eb5}: 'ີ'
    VowelSignIi,
    /// \u{eb6}: 'ຶ'
    VowelSignY,
    /// \u{eb7}: 'ື'
    VowelSignYy,
    /// \u{eb8}: 'ຸ'
    VowelSignU,
    /// \u{eb9}: 'ູ'
    VowelSignUu,
    /// \u{eba}: '຺'
    SignPaliVirama,
    /// \u{ebb}: 'ົ'
    VowelSignMaiKon,
    /// \u{ebc}: 'ຼ'
    SemivowelSignLo,
    /// \u{ebd}: 'ຽ'
    SemivowelSignNyo,
    /// \u{ec0}: 'ເ'
    VowelSignE,
    /// \u{ec1}: 'ແ'
    VowelSignEi,
    /// \u{ec2}: 'ໂ'
    VowelSignO,
    /// \u{ec3}: 'ໃ'
    VowelSignAy,
    /// \u{ec4}: 'ໄ'
    VowelSignAi,
    /// \u{ec6}: 'ໆ'
    KoLa,
    /// \u{ec8}: '່'
    ToneMaiEk,
    /// \u{ec9}: '້'
    ToneMaiTho,
    /// \u{eca}: '໊'
    ToneMaiTi,
    /// \u{ecb}: '໋'
    ToneMaiCatawa,
    /// \u{ecc}: '໌'
    CancellationMark,
    /// \u{ecd}: 'ໍ'
    Niggahita,
    /// \u{ed0}: '໐'
    DigitZero,
    /// \u{ed1}: '໑'
    DigitOne,
    /// \u{ed2}: '໒'
    DigitTwo,
    /// \u{ed3}: '໓'
    DigitThree,
    /// \u{ed4}: '໔'
    DigitFour,
    /// \u{ed5}: '໕'
    DigitFive,
    /// \u{ed6}: '໖'
    DigitSix,
    /// \u{ed7}: '໗'
    DigitSeven,
    /// \u{ed8}: '໘'
    DigitEight,
    /// \u{ed9}: '໙'
    DigitNine,
    /// \u{edc}: 'ໜ'
    HoNo,
    /// \u{edd}: 'ໝ'
    HoMo,
    /// \u{ede}: 'ໞ'
    LetterKhmuGo,
    /// \u{edf}: 'ໟ'
    LetterKhmuNyo,
}

impl Into<char> for Lao {
    fn into(self) -> char {
        match self {
            Lao::LetterKo => 'ກ',
            Lao::LetterKhoSung => 'ຂ',
            Lao::LetterKhoTam => 'ຄ',
            Lao::LetterPaliGha => 'ຆ',
            Lao::LetterNgo => 'ງ',
            Lao::LetterCo => 'ຈ',
            Lao::LetterPaliCha => 'ຉ',
            Lao::LetterSoTam => 'ຊ',
            Lao::LetterPaliJha => 'ຌ',
            Lao::LetterNyo => 'ຍ',
            Lao::LetterPaliNya => 'ຎ',
            Lao::LetterPaliTta => 'ຏ',
            Lao::LetterPaliTtha => 'ຐ',
            Lao::LetterPaliDda => 'ຑ',
            Lao::LetterPaliDdha => 'ຒ',
            Lao::LetterPaliNna => 'ຓ',
            Lao::LetterDo => 'ດ',
            Lao::LetterTo => 'ຕ',
            Lao::LetterThoSung => 'ຖ',
            Lao::LetterThoTam => 'ທ',
            Lao::LetterPaliDha => 'ຘ',
            Lao::LetterNo => 'ນ',
            Lao::LetterBo => 'ບ',
            Lao::LetterPo => 'ປ',
            Lao::LetterPhoSung => 'ຜ',
            Lao::LetterFoTam => 'ຝ',
            Lao::LetterPhoTam => 'ພ',
            Lao::LetterFoSung => 'ຟ',
            Lao::LetterPaliBha => 'ຠ',
            Lao::LetterMo => 'ມ',
            Lao::LetterYo => 'ຢ',
            Lao::LetterLoLing => 'ຣ',
            Lao::LetterLoLoot => 'ລ',
            Lao::LetterWo => 'ວ',
            Lao::LetterSanskritSha => 'ຨ',
            Lao::LetterSanskritSsa => 'ຩ',
            Lao::LetterSoSung => 'ສ',
            Lao::LetterHoSung => 'ຫ',
            Lao::LetterPaliLla => 'ຬ',
            Lao::LetterO => 'ອ',
            Lao::LetterHoTam => 'ຮ',
            Lao::Ellipsis => 'ຯ',
            Lao::VowelSignA => 'ະ',
            Lao::VowelSignMaiKan => 'ັ',
            Lao::VowelSignAa => 'າ',
            Lao::VowelSignAm => 'ຳ',
            Lao::VowelSignI => 'ິ',
            Lao::VowelSignIi => 'ີ',
            Lao::VowelSignY => 'ຶ',
            Lao::VowelSignYy => 'ື',
            Lao::VowelSignU => 'ຸ',
            Lao::VowelSignUu => 'ູ',
            Lao::SignPaliVirama => '຺',
            Lao::VowelSignMaiKon => 'ົ',
            Lao::SemivowelSignLo => 'ຼ',
            Lao::SemivowelSignNyo => 'ຽ',
            Lao::VowelSignE => 'ເ',
            Lao::VowelSignEi => 'ແ',
            Lao::VowelSignO => 'ໂ',
            Lao::VowelSignAy => 'ໃ',
            Lao::VowelSignAi => 'ໄ',
            Lao::KoLa => 'ໆ',
            Lao::ToneMaiEk => '່',
            Lao::ToneMaiTho => '້',
            Lao::ToneMaiTi => '໊',
            Lao::ToneMaiCatawa => '໋',
            Lao::CancellationMark => '໌',
            Lao::Niggahita => 'ໍ',
            Lao::DigitZero => '໐',
            Lao::DigitOne => '໑',
            Lao::DigitTwo => '໒',
            Lao::DigitThree => '໓',
            Lao::DigitFour => '໔',
            Lao::DigitFive => '໕',
            Lao::DigitSix => '໖',
            Lao::DigitSeven => '໗',
            Lao::DigitEight => '໘',
            Lao::DigitNine => '໙',
            Lao::HoNo => 'ໜ',
            Lao::HoMo => 'ໝ',
            Lao::LetterKhmuGo => 'ໞ',
            Lao::LetterKhmuNyo => 'ໟ',
        }
    }
}

impl std::convert::TryFrom<char> for Lao {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ກ' => Ok(Lao::LetterKo),
            'ຂ' => Ok(Lao::LetterKhoSung),
            'ຄ' => Ok(Lao::LetterKhoTam),
            'ຆ' => Ok(Lao::LetterPaliGha),
            'ງ' => Ok(Lao::LetterNgo),
            'ຈ' => Ok(Lao::LetterCo),
            'ຉ' => Ok(Lao::LetterPaliCha),
            'ຊ' => Ok(Lao::LetterSoTam),
            'ຌ' => Ok(Lao::LetterPaliJha),
            'ຍ' => Ok(Lao::LetterNyo),
            'ຎ' => Ok(Lao::LetterPaliNya),
            'ຏ' => Ok(Lao::LetterPaliTta),
            'ຐ' => Ok(Lao::LetterPaliTtha),
            'ຑ' => Ok(Lao::LetterPaliDda),
            'ຒ' => Ok(Lao::LetterPaliDdha),
            'ຓ' => Ok(Lao::LetterPaliNna),
            'ດ' => Ok(Lao::LetterDo),
            'ຕ' => Ok(Lao::LetterTo),
            'ຖ' => Ok(Lao::LetterThoSung),
            'ທ' => Ok(Lao::LetterThoTam),
            'ຘ' => Ok(Lao::LetterPaliDha),
            'ນ' => Ok(Lao::LetterNo),
            'ບ' => Ok(Lao::LetterBo),
            'ປ' => Ok(Lao::LetterPo),
            'ຜ' => Ok(Lao::LetterPhoSung),
            'ຝ' => Ok(Lao::LetterFoTam),
            'ພ' => Ok(Lao::LetterPhoTam),
            'ຟ' => Ok(Lao::LetterFoSung),
            'ຠ' => Ok(Lao::LetterPaliBha),
            'ມ' => Ok(Lao::LetterMo),
            'ຢ' => Ok(Lao::LetterYo),
            'ຣ' => Ok(Lao::LetterLoLing),
            'ລ' => Ok(Lao::LetterLoLoot),
            'ວ' => Ok(Lao::LetterWo),
            'ຨ' => Ok(Lao::LetterSanskritSha),
            'ຩ' => Ok(Lao::LetterSanskritSsa),
            'ສ' => Ok(Lao::LetterSoSung),
            'ຫ' => Ok(Lao::LetterHoSung),
            'ຬ' => Ok(Lao::LetterPaliLla),
            'ອ' => Ok(Lao::LetterO),
            'ຮ' => Ok(Lao::LetterHoTam),
            'ຯ' => Ok(Lao::Ellipsis),
            'ະ' => Ok(Lao::VowelSignA),
            'ັ' => Ok(Lao::VowelSignMaiKan),
            'າ' => Ok(Lao::VowelSignAa),
            'ຳ' => Ok(Lao::VowelSignAm),
            'ິ' => Ok(Lao::VowelSignI),
            'ີ' => Ok(Lao::VowelSignIi),
            'ຶ' => Ok(Lao::VowelSignY),
            'ື' => Ok(Lao::VowelSignYy),
            'ຸ' => Ok(Lao::VowelSignU),
            'ູ' => Ok(Lao::VowelSignUu),
            '຺' => Ok(Lao::SignPaliVirama),
            'ົ' => Ok(Lao::VowelSignMaiKon),
            'ຼ' => Ok(Lao::SemivowelSignLo),
            'ຽ' => Ok(Lao::SemivowelSignNyo),
            'ເ' => Ok(Lao::VowelSignE),
            'ແ' => Ok(Lao::VowelSignEi),
            'ໂ' => Ok(Lao::VowelSignO),
            'ໃ' => Ok(Lao::VowelSignAy),
            'ໄ' => Ok(Lao::VowelSignAi),
            'ໆ' => Ok(Lao::KoLa),
            '່' => Ok(Lao::ToneMaiEk),
            '້' => Ok(Lao::ToneMaiTho),
            '໊' => Ok(Lao::ToneMaiTi),
            '໋' => Ok(Lao::ToneMaiCatawa),
            '໌' => Ok(Lao::CancellationMark),
            'ໍ' => Ok(Lao::Niggahita),
            '໐' => Ok(Lao::DigitZero),
            '໑' => Ok(Lao::DigitOne),
            '໒' => Ok(Lao::DigitTwo),
            '໓' => Ok(Lao::DigitThree),
            '໔' => Ok(Lao::DigitFour),
            '໕' => Ok(Lao::DigitFive),
            '໖' => Ok(Lao::DigitSix),
            '໗' => Ok(Lao::DigitSeven),
            '໘' => Ok(Lao::DigitEight),
            '໙' => Ok(Lao::DigitNine),
            'ໜ' => Ok(Lao::HoNo),
            'ໝ' => Ok(Lao::HoMo),
            'ໞ' => Ok(Lao::LetterKhmuGo),
            'ໟ' => Ok(Lao::LetterKhmuNyo),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Lao {
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

impl std::convert::TryFrom<u32> for Lao {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Lao {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Lao {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Lao::LetterKo
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Lao{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
