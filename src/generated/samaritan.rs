
/// An enum to represent all characters in the Samaritan block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Samaritan {
    /// \u{800}: 'ࠀ'
    LetterAlaf,
    /// \u{801}: 'ࠁ'
    LetterBit,
    /// \u{802}: 'ࠂ'
    LetterGaman,
    /// \u{803}: 'ࠃ'
    LetterDalat,
    /// \u{804}: 'ࠄ'
    LetterIy,
    /// \u{805}: 'ࠅ'
    LetterBaa,
    /// \u{806}: 'ࠆ'
    LetterZen,
    /// \u{807}: 'ࠇ'
    LetterIt,
    /// \u{808}: 'ࠈ'
    LetterTit,
    /// \u{809}: 'ࠉ'
    LetterYut,
    /// \u{80a}: 'ࠊ'
    LetterKaaf,
    /// \u{80b}: 'ࠋ'
    LetterLabat,
    /// \u{80c}: 'ࠌ'
    LetterMim,
    /// \u{80d}: 'ࠍ'
    LetterNun,
    /// \u{80e}: 'ࠎ'
    LetterSingaat,
    /// \u{80f}: 'ࠏ'
    LetterIn,
    /// \u{810}: 'ࠐ'
    LetterFi,
    /// \u{811}: 'ࠑ'
    LetterTsaadiy,
    /// \u{812}: 'ࠒ'
    LetterQuf,
    /// \u{813}: 'ࠓ'
    LetterRish,
    /// \u{814}: 'ࠔ'
    LetterShan,
    /// \u{815}: 'ࠕ'
    LetterTaaf,
    /// \u{816}: 'ࠖ'
    MarkIn,
    /// \u{817}: 'ࠗ'
    MarkInDashAlaf,
    /// \u{818}: '࠘'
    MarkOcclusion,
    /// \u{819}: '࠙'
    MarkDagesh,
    /// \u{81a}: 'ࠚ'
    ModifierLetterEpentheticYut,
    /// \u{81b}: 'ࠛ'
    MarkEpentheticYut,
    /// \u{81c}: 'ࠜ'
    VowelSignLongE,
    /// \u{81d}: 'ࠝ'
    VowelSignE,
    /// \u{81e}: 'ࠞ'
    VowelSignOverlongAa,
    /// \u{81f}: 'ࠟ'
    VowelSignLongAa,
    /// \u{820}: 'ࠠ'
    VowelSignAa,
    /// \u{821}: 'ࠡ'
    VowelSignOverlongA,
    /// \u{822}: 'ࠢ'
    VowelSignLongA,
    /// \u{823}: 'ࠣ'
    VowelSignA,
    /// \u{824}: 'ࠤ'
    ModifierLetterShortA,
    /// \u{825}: 'ࠥ'
    VowelSignShortA,
    /// \u{826}: 'ࠦ'
    VowelSignLongU,
    /// \u{827}: 'ࠧ'
    VowelSignU,
    /// \u{828}: 'ࠨ'
    ModifierLetterI,
    /// \u{829}: 'ࠩ'
    VowelSignLongI,
    /// \u{82a}: 'ࠪ'
    VowelSignI,
    /// \u{82b}: 'ࠫ'
    VowelSignO,
    /// \u{82c}: 'ࠬ'
    VowelSignSukun,
    /// \u{82d}: '࠭'
    MarkNequdaa,
    /// \u{830}: '࠰'
    PunctuationNequdaa,
    /// \u{831}: '࠱'
    PunctuationAfsaaq,
    /// \u{832}: '࠲'
    PunctuationAnged,
    /// \u{833}: '࠳'
    PunctuationBau,
    /// \u{834}: '࠴'
    PunctuationAtmaau,
    /// \u{835}: '࠵'
    PunctuationShiyyaalaa,
    /// \u{836}: '࠶'
    AbbreviationMark,
    /// \u{837}: '࠷'
    PunctuationMelodicQitsa,
    /// \u{838}: '࠸'
    PunctuationZiqaa,
    /// \u{839}: '࠹'
    PunctuationQitsa,
    /// \u{83a}: '࠺'
    PunctuationZaef,
    /// \u{83b}: '࠻'
    PunctuationTuru,
    /// \u{83c}: '࠼'
    PunctuationArkaanu,
    /// \u{83d}: '࠽'
    PunctuationSofMashfaat,
    /// \u{83e}: '࠾'
    PunctuationAnnaau,
}

impl Into<char> for Samaritan {
    fn into(self) -> char {
        match self {
            Samaritan::LetterAlaf => 'ࠀ',
            Samaritan::LetterBit => 'ࠁ',
            Samaritan::LetterGaman => 'ࠂ',
            Samaritan::LetterDalat => 'ࠃ',
            Samaritan::LetterIy => 'ࠄ',
            Samaritan::LetterBaa => 'ࠅ',
            Samaritan::LetterZen => 'ࠆ',
            Samaritan::LetterIt => 'ࠇ',
            Samaritan::LetterTit => 'ࠈ',
            Samaritan::LetterYut => 'ࠉ',
            Samaritan::LetterKaaf => 'ࠊ',
            Samaritan::LetterLabat => 'ࠋ',
            Samaritan::LetterMim => 'ࠌ',
            Samaritan::LetterNun => 'ࠍ',
            Samaritan::LetterSingaat => 'ࠎ',
            Samaritan::LetterIn => 'ࠏ',
            Samaritan::LetterFi => 'ࠐ',
            Samaritan::LetterTsaadiy => 'ࠑ',
            Samaritan::LetterQuf => 'ࠒ',
            Samaritan::LetterRish => 'ࠓ',
            Samaritan::LetterShan => 'ࠔ',
            Samaritan::LetterTaaf => 'ࠕ',
            Samaritan::MarkIn => 'ࠖ',
            Samaritan::MarkInDashAlaf => 'ࠗ',
            Samaritan::MarkOcclusion => '࠘',
            Samaritan::MarkDagesh => '࠙',
            Samaritan::ModifierLetterEpentheticYut => 'ࠚ',
            Samaritan::MarkEpentheticYut => 'ࠛ',
            Samaritan::VowelSignLongE => 'ࠜ',
            Samaritan::VowelSignE => 'ࠝ',
            Samaritan::VowelSignOverlongAa => 'ࠞ',
            Samaritan::VowelSignLongAa => 'ࠟ',
            Samaritan::VowelSignAa => 'ࠠ',
            Samaritan::VowelSignOverlongA => 'ࠡ',
            Samaritan::VowelSignLongA => 'ࠢ',
            Samaritan::VowelSignA => 'ࠣ',
            Samaritan::ModifierLetterShortA => 'ࠤ',
            Samaritan::VowelSignShortA => 'ࠥ',
            Samaritan::VowelSignLongU => 'ࠦ',
            Samaritan::VowelSignU => 'ࠧ',
            Samaritan::ModifierLetterI => 'ࠨ',
            Samaritan::VowelSignLongI => 'ࠩ',
            Samaritan::VowelSignI => 'ࠪ',
            Samaritan::VowelSignO => 'ࠫ',
            Samaritan::VowelSignSukun => 'ࠬ',
            Samaritan::MarkNequdaa => '࠭',
            Samaritan::PunctuationNequdaa => '࠰',
            Samaritan::PunctuationAfsaaq => '࠱',
            Samaritan::PunctuationAnged => '࠲',
            Samaritan::PunctuationBau => '࠳',
            Samaritan::PunctuationAtmaau => '࠴',
            Samaritan::PunctuationShiyyaalaa => '࠵',
            Samaritan::AbbreviationMark => '࠶',
            Samaritan::PunctuationMelodicQitsa => '࠷',
            Samaritan::PunctuationZiqaa => '࠸',
            Samaritan::PunctuationQitsa => '࠹',
            Samaritan::PunctuationZaef => '࠺',
            Samaritan::PunctuationTuru => '࠻',
            Samaritan::PunctuationArkaanu => '࠼',
            Samaritan::PunctuationSofMashfaat => '࠽',
            Samaritan::PunctuationAnnaau => '࠾',
        }
    }
}

impl std::convert::TryFrom<char> for Samaritan {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ࠀ' => Ok(Samaritan::LetterAlaf),
            'ࠁ' => Ok(Samaritan::LetterBit),
            'ࠂ' => Ok(Samaritan::LetterGaman),
            'ࠃ' => Ok(Samaritan::LetterDalat),
            'ࠄ' => Ok(Samaritan::LetterIy),
            'ࠅ' => Ok(Samaritan::LetterBaa),
            'ࠆ' => Ok(Samaritan::LetterZen),
            'ࠇ' => Ok(Samaritan::LetterIt),
            'ࠈ' => Ok(Samaritan::LetterTit),
            'ࠉ' => Ok(Samaritan::LetterYut),
            'ࠊ' => Ok(Samaritan::LetterKaaf),
            'ࠋ' => Ok(Samaritan::LetterLabat),
            'ࠌ' => Ok(Samaritan::LetterMim),
            'ࠍ' => Ok(Samaritan::LetterNun),
            'ࠎ' => Ok(Samaritan::LetterSingaat),
            'ࠏ' => Ok(Samaritan::LetterIn),
            'ࠐ' => Ok(Samaritan::LetterFi),
            'ࠑ' => Ok(Samaritan::LetterTsaadiy),
            'ࠒ' => Ok(Samaritan::LetterQuf),
            'ࠓ' => Ok(Samaritan::LetterRish),
            'ࠔ' => Ok(Samaritan::LetterShan),
            'ࠕ' => Ok(Samaritan::LetterTaaf),
            'ࠖ' => Ok(Samaritan::MarkIn),
            'ࠗ' => Ok(Samaritan::MarkInDashAlaf),
            '࠘' => Ok(Samaritan::MarkOcclusion),
            '࠙' => Ok(Samaritan::MarkDagesh),
            'ࠚ' => Ok(Samaritan::ModifierLetterEpentheticYut),
            'ࠛ' => Ok(Samaritan::MarkEpentheticYut),
            'ࠜ' => Ok(Samaritan::VowelSignLongE),
            'ࠝ' => Ok(Samaritan::VowelSignE),
            'ࠞ' => Ok(Samaritan::VowelSignOverlongAa),
            'ࠟ' => Ok(Samaritan::VowelSignLongAa),
            'ࠠ' => Ok(Samaritan::VowelSignAa),
            'ࠡ' => Ok(Samaritan::VowelSignOverlongA),
            'ࠢ' => Ok(Samaritan::VowelSignLongA),
            'ࠣ' => Ok(Samaritan::VowelSignA),
            'ࠤ' => Ok(Samaritan::ModifierLetterShortA),
            'ࠥ' => Ok(Samaritan::VowelSignShortA),
            'ࠦ' => Ok(Samaritan::VowelSignLongU),
            'ࠧ' => Ok(Samaritan::VowelSignU),
            'ࠨ' => Ok(Samaritan::ModifierLetterI),
            'ࠩ' => Ok(Samaritan::VowelSignLongI),
            'ࠪ' => Ok(Samaritan::VowelSignI),
            'ࠫ' => Ok(Samaritan::VowelSignO),
            'ࠬ' => Ok(Samaritan::VowelSignSukun),
            '࠭' => Ok(Samaritan::MarkNequdaa),
            '࠰' => Ok(Samaritan::PunctuationNequdaa),
            '࠱' => Ok(Samaritan::PunctuationAfsaaq),
            '࠲' => Ok(Samaritan::PunctuationAnged),
            '࠳' => Ok(Samaritan::PunctuationBau),
            '࠴' => Ok(Samaritan::PunctuationAtmaau),
            '࠵' => Ok(Samaritan::PunctuationShiyyaalaa),
            '࠶' => Ok(Samaritan::AbbreviationMark),
            '࠷' => Ok(Samaritan::PunctuationMelodicQitsa),
            '࠸' => Ok(Samaritan::PunctuationZiqaa),
            '࠹' => Ok(Samaritan::PunctuationQitsa),
            '࠺' => Ok(Samaritan::PunctuationZaef),
            '࠻' => Ok(Samaritan::PunctuationTuru),
            '࠼' => Ok(Samaritan::PunctuationArkaanu),
            '࠽' => Ok(Samaritan::PunctuationSofMashfaat),
            '࠾' => Ok(Samaritan::PunctuationAnnaau),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Samaritan {
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

impl std::convert::TryFrom<u32> for Samaritan {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Samaritan {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Samaritan {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Samaritan::LetterAlaf
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Samaritan{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
