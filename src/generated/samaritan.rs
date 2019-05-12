
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

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Samaritan::LetterAlaf => "samaritan letter alaf",
            Samaritan::LetterBit => "samaritan letter bit",
            Samaritan::LetterGaman => "samaritan letter gaman",
            Samaritan::LetterDalat => "samaritan letter dalat",
            Samaritan::LetterIy => "samaritan letter iy",
            Samaritan::LetterBaa => "samaritan letter baa",
            Samaritan::LetterZen => "samaritan letter zen",
            Samaritan::LetterIt => "samaritan letter it",
            Samaritan::LetterTit => "samaritan letter tit",
            Samaritan::LetterYut => "samaritan letter yut",
            Samaritan::LetterKaaf => "samaritan letter kaaf",
            Samaritan::LetterLabat => "samaritan letter labat",
            Samaritan::LetterMim => "samaritan letter mim",
            Samaritan::LetterNun => "samaritan letter nun",
            Samaritan::LetterSingaat => "samaritan letter singaat",
            Samaritan::LetterIn => "samaritan letter in",
            Samaritan::LetterFi => "samaritan letter fi",
            Samaritan::LetterTsaadiy => "samaritan letter tsaadiy",
            Samaritan::LetterQuf => "samaritan letter quf",
            Samaritan::LetterRish => "samaritan letter rish",
            Samaritan::LetterShan => "samaritan letter shan",
            Samaritan::LetterTaaf => "samaritan letter taaf",
            Samaritan::MarkIn => "samaritan mark in",
            Samaritan::MarkInDashAlaf => "samaritan mark in-alaf",
            Samaritan::MarkOcclusion => "samaritan mark occlusion",
            Samaritan::MarkDagesh => "samaritan mark dagesh",
            Samaritan::ModifierLetterEpentheticYut => "samaritan modifier letter epenthetic yut",
            Samaritan::MarkEpentheticYut => "samaritan mark epenthetic yut",
            Samaritan::VowelSignLongE => "samaritan vowel sign long e",
            Samaritan::VowelSignE => "samaritan vowel sign e",
            Samaritan::VowelSignOverlongAa => "samaritan vowel sign overlong aa",
            Samaritan::VowelSignLongAa => "samaritan vowel sign long aa",
            Samaritan::VowelSignAa => "samaritan vowel sign aa",
            Samaritan::VowelSignOverlongA => "samaritan vowel sign overlong a",
            Samaritan::VowelSignLongA => "samaritan vowel sign long a",
            Samaritan::VowelSignA => "samaritan vowel sign a",
            Samaritan::ModifierLetterShortA => "samaritan modifier letter short a",
            Samaritan::VowelSignShortA => "samaritan vowel sign short a",
            Samaritan::VowelSignLongU => "samaritan vowel sign long u",
            Samaritan::VowelSignU => "samaritan vowel sign u",
            Samaritan::ModifierLetterI => "samaritan modifier letter i",
            Samaritan::VowelSignLongI => "samaritan vowel sign long i",
            Samaritan::VowelSignI => "samaritan vowel sign i",
            Samaritan::VowelSignO => "samaritan vowel sign o",
            Samaritan::VowelSignSukun => "samaritan vowel sign sukun",
            Samaritan::MarkNequdaa => "samaritan mark nequdaa",
            Samaritan::PunctuationNequdaa => "samaritan punctuation nequdaa",
            Samaritan::PunctuationAfsaaq => "samaritan punctuation afsaaq",
            Samaritan::PunctuationAnged => "samaritan punctuation anged",
            Samaritan::PunctuationBau => "samaritan punctuation bau",
            Samaritan::PunctuationAtmaau => "samaritan punctuation atmaau",
            Samaritan::PunctuationShiyyaalaa => "samaritan punctuation shiyyaalaa",
            Samaritan::AbbreviationMark => "samaritan abbreviation mark",
            Samaritan::PunctuationMelodicQitsa => "samaritan punctuation melodic qitsa",
            Samaritan::PunctuationZiqaa => "samaritan punctuation ziqaa",
            Samaritan::PunctuationQitsa => "samaritan punctuation qitsa",
            Samaritan::PunctuationZaef => "samaritan punctuation zaef",
            Samaritan::PunctuationTuru => "samaritan punctuation turu",
            Samaritan::PunctuationArkaanu => "samaritan punctuation arkaanu",
            Samaritan::PunctuationSofMashfaat => "samaritan punctuation sof mashfaat",
            Samaritan::PunctuationAnnaau => "samaritan punctuation annaau",
        }
    }
}
