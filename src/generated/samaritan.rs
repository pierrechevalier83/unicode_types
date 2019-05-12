/// \u{800} → \u{83f}
///
/// ࠀ ࠁ ࠂ ࠃ ࠄ ࠅ ࠆ ࠇ ࠈ ࠉ ࠊ ࠋ ࠌ ࠍ ࠎ ࠏ\
/// ࠐ ࠑ ࠒ ࠓ ࠔ ࠕ ࠖ ࠗ ࠘ ࠙ ࠚ ࠛ ࠜ ࠝ ࠞ ࠟ\
/// ࠠ ࠡ ࠢ ࠣ ࠤ ࠥ ࠦ ࠧ ࠨ ࠩ ࠪ ࠫ ࠬ ࠭ ࠰ ࠱\
/// ࠲ ࠳ ࠴ ࠵ ࠶ ࠷ ࠸ ࠹ ࠺ ࠻ ࠼ ࠽ ࠾\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{800}: 'ࠀ'
    pub const LETTER_ALAF: char = 'ࠀ';
    /// \u{801}: 'ࠁ'
    pub const LETTER_BIT: char = 'ࠁ';
    /// \u{802}: 'ࠂ'
    pub const LETTER_GAMAN: char = 'ࠂ';
    /// \u{803}: 'ࠃ'
    pub const LETTER_DALAT: char = 'ࠃ';
    /// \u{804}: 'ࠄ'
    pub const LETTER_IY: char = 'ࠄ';
    /// \u{805}: 'ࠅ'
    pub const LETTER_BAA: char = 'ࠅ';
    /// \u{806}: 'ࠆ'
    pub const LETTER_ZEN: char = 'ࠆ';
    /// \u{807}: 'ࠇ'
    pub const LETTER_IT: char = 'ࠇ';
    /// \u{808}: 'ࠈ'
    pub const LETTER_TIT: char = 'ࠈ';
    /// \u{809}: 'ࠉ'
    pub const LETTER_YUT: char = 'ࠉ';
    /// \u{80a}: 'ࠊ'
    pub const LETTER_KAAF: char = 'ࠊ';
    /// \u{80b}: 'ࠋ'
    pub const LETTER_LABAT: char = 'ࠋ';
    /// \u{80c}: 'ࠌ'
    pub const LETTER_MIM: char = 'ࠌ';
    /// \u{80d}: 'ࠍ'
    pub const LETTER_NUN: char = 'ࠍ';
    /// \u{80e}: 'ࠎ'
    pub const LETTER_SINGAAT: char = 'ࠎ';
    /// \u{80f}: 'ࠏ'
    pub const LETTER_IN: char = 'ࠏ';
    /// \u{810}: 'ࠐ'
    pub const LETTER_FI: char = 'ࠐ';
    /// \u{811}: 'ࠑ'
    pub const LETTER_TSAADIY: char = 'ࠑ';
    /// \u{812}: 'ࠒ'
    pub const LETTER_QUF: char = 'ࠒ';
    /// \u{813}: 'ࠓ'
    pub const LETTER_RISH: char = 'ࠓ';
    /// \u{814}: 'ࠔ'
    pub const LETTER_SHAN: char = 'ࠔ';
    /// \u{815}: 'ࠕ'
    pub const LETTER_TAAF: char = 'ࠕ';
    /// \u{816}: 'ࠖ'
    pub const MARK_IN: char = 'ࠖ';
    /// \u{817}: 'ࠗ'
    pub const MARK_IN_DASH_ALAF: char = 'ࠗ';
    /// \u{818}: '࠘'
    pub const MARK_OCCLUSION: char = '࠘';
    /// \u{819}: '࠙'
    pub const MARK_DAGESH: char = '࠙';
    /// \u{81a}: 'ࠚ'
    pub const MODIFIER_LETTER_EPENTHETIC_YUT: char = 'ࠚ';
    /// \u{81b}: 'ࠛ'
    pub const MARK_EPENTHETIC_YUT: char = 'ࠛ';
    /// \u{81c}: 'ࠜ'
    pub const VOWEL_SIGN_LONG_E: char = 'ࠜ';
    /// \u{81d}: 'ࠝ'
    pub const VOWEL_SIGN_E: char = 'ࠝ';
    /// \u{81e}: 'ࠞ'
    pub const VOWEL_SIGN_OVERLONG_AA: char = 'ࠞ';
    /// \u{81f}: 'ࠟ'
    pub const VOWEL_SIGN_LONG_AA: char = 'ࠟ';
    /// \u{820}: 'ࠠ'
    pub const VOWEL_SIGN_AA: char = 'ࠠ';
    /// \u{821}: 'ࠡ'
    pub const VOWEL_SIGN_OVERLONG_A: char = 'ࠡ';
    /// \u{822}: 'ࠢ'
    pub const VOWEL_SIGN_LONG_A: char = 'ࠢ';
    /// \u{823}: 'ࠣ'
    pub const VOWEL_SIGN_A: char = 'ࠣ';
    /// \u{824}: 'ࠤ'
    pub const MODIFIER_LETTER_SHORT_A: char = 'ࠤ';
    /// \u{825}: 'ࠥ'
    pub const VOWEL_SIGN_SHORT_A: char = 'ࠥ';
    /// \u{826}: 'ࠦ'
    pub const VOWEL_SIGN_LONG_U: char = 'ࠦ';
    /// \u{827}: 'ࠧ'
    pub const VOWEL_SIGN_U: char = 'ࠧ';
    /// \u{828}: 'ࠨ'
    pub const MODIFIER_LETTER_I: char = 'ࠨ';
    /// \u{829}: 'ࠩ'
    pub const VOWEL_SIGN_LONG_I: char = 'ࠩ';
    /// \u{82a}: 'ࠪ'
    pub const VOWEL_SIGN_I: char = 'ࠪ';
    /// \u{82b}: 'ࠫ'
    pub const VOWEL_SIGN_O: char = 'ࠫ';
    /// \u{82c}: 'ࠬ'
    pub const VOWEL_SIGN_SUKUN: char = 'ࠬ';
    /// \u{82d}: '࠭'
    pub const MARK_NEQUDAA: char = '࠭';
    /// \u{830}: '࠰'
    pub const PUNCTUATION_NEQUDAA: char = '࠰';
    /// \u{831}: '࠱'
    pub const PUNCTUATION_AFSAAQ: char = '࠱';
    /// \u{832}: '࠲'
    pub const PUNCTUATION_ANGED: char = '࠲';
    /// \u{833}: '࠳'
    pub const PUNCTUATION_BAU: char = '࠳';
    /// \u{834}: '࠴'
    pub const PUNCTUATION_ATMAAU: char = '࠴';
    /// \u{835}: '࠵'
    pub const PUNCTUATION_SHIYYAALAA: char = '࠵';
    /// \u{836}: '࠶'
    pub const ABBREVIATION_MARK: char = '࠶';
    /// \u{837}: '࠷'
    pub const PUNCTUATION_MELODIC_QITSA: char = '࠷';
    /// \u{838}: '࠸'
    pub const PUNCTUATION_ZIQAA: char = '࠸';
    /// \u{839}: '࠹'
    pub const PUNCTUATION_QITSA: char = '࠹';
    /// \u{83a}: '࠺'
    pub const PUNCTUATION_ZAEF: char = '࠺';
    /// \u{83b}: '࠻'
    pub const PUNCTUATION_TURU: char = '࠻';
    /// \u{83c}: '࠼'
    pub const PUNCTUATION_ARKAANU: char = '࠼';
    /// \u{83d}: '࠽'
    pub const PUNCTUATION_SOF_MASHFAAT: char = '࠽';
    /// \u{83e}: '࠾'
    pub const PUNCTUATION_ANNAAU: char = '࠾';
}

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
        use constants::*;
        match self {
            Samaritan::LetterAlaf => LETTER_ALAF,
            Samaritan::LetterBit => LETTER_BIT,
            Samaritan::LetterGaman => LETTER_GAMAN,
            Samaritan::LetterDalat => LETTER_DALAT,
            Samaritan::LetterIy => LETTER_IY,
            Samaritan::LetterBaa => LETTER_BAA,
            Samaritan::LetterZen => LETTER_ZEN,
            Samaritan::LetterIt => LETTER_IT,
            Samaritan::LetterTit => LETTER_TIT,
            Samaritan::LetterYut => LETTER_YUT,
            Samaritan::LetterKaaf => LETTER_KAAF,
            Samaritan::LetterLabat => LETTER_LABAT,
            Samaritan::LetterMim => LETTER_MIM,
            Samaritan::LetterNun => LETTER_NUN,
            Samaritan::LetterSingaat => LETTER_SINGAAT,
            Samaritan::LetterIn => LETTER_IN,
            Samaritan::LetterFi => LETTER_FI,
            Samaritan::LetterTsaadiy => LETTER_TSAADIY,
            Samaritan::LetterQuf => LETTER_QUF,
            Samaritan::LetterRish => LETTER_RISH,
            Samaritan::LetterShan => LETTER_SHAN,
            Samaritan::LetterTaaf => LETTER_TAAF,
            Samaritan::MarkIn => MARK_IN,
            Samaritan::MarkInDashAlaf => MARK_IN_DASH_ALAF,
            Samaritan::MarkOcclusion => MARK_OCCLUSION,
            Samaritan::MarkDagesh => MARK_DAGESH,
            Samaritan::ModifierLetterEpentheticYut => MODIFIER_LETTER_EPENTHETIC_YUT,
            Samaritan::MarkEpentheticYut => MARK_EPENTHETIC_YUT,
            Samaritan::VowelSignLongE => VOWEL_SIGN_LONG_E,
            Samaritan::VowelSignE => VOWEL_SIGN_E,
            Samaritan::VowelSignOverlongAa => VOWEL_SIGN_OVERLONG_AA,
            Samaritan::VowelSignLongAa => VOWEL_SIGN_LONG_AA,
            Samaritan::VowelSignAa => VOWEL_SIGN_AA,
            Samaritan::VowelSignOverlongA => VOWEL_SIGN_OVERLONG_A,
            Samaritan::VowelSignLongA => VOWEL_SIGN_LONG_A,
            Samaritan::VowelSignA => VOWEL_SIGN_A,
            Samaritan::ModifierLetterShortA => MODIFIER_LETTER_SHORT_A,
            Samaritan::VowelSignShortA => VOWEL_SIGN_SHORT_A,
            Samaritan::VowelSignLongU => VOWEL_SIGN_LONG_U,
            Samaritan::VowelSignU => VOWEL_SIGN_U,
            Samaritan::ModifierLetterI => MODIFIER_LETTER_I,
            Samaritan::VowelSignLongI => VOWEL_SIGN_LONG_I,
            Samaritan::VowelSignI => VOWEL_SIGN_I,
            Samaritan::VowelSignO => VOWEL_SIGN_O,
            Samaritan::VowelSignSukun => VOWEL_SIGN_SUKUN,
            Samaritan::MarkNequdaa => MARK_NEQUDAA,
            Samaritan::PunctuationNequdaa => PUNCTUATION_NEQUDAA,
            Samaritan::PunctuationAfsaaq => PUNCTUATION_AFSAAQ,
            Samaritan::PunctuationAnged => PUNCTUATION_ANGED,
            Samaritan::PunctuationBau => PUNCTUATION_BAU,
            Samaritan::PunctuationAtmaau => PUNCTUATION_ATMAAU,
            Samaritan::PunctuationShiyyaalaa => PUNCTUATION_SHIYYAALAA,
            Samaritan::AbbreviationMark => ABBREVIATION_MARK,
            Samaritan::PunctuationMelodicQitsa => PUNCTUATION_MELODIC_QITSA,
            Samaritan::PunctuationZiqaa => PUNCTUATION_ZIQAA,
            Samaritan::PunctuationQitsa => PUNCTUATION_QITSA,
            Samaritan::PunctuationZaef => PUNCTUATION_ZAEF,
            Samaritan::PunctuationTuru => PUNCTUATION_TURU,
            Samaritan::PunctuationArkaanu => PUNCTUATION_ARKAANU,
            Samaritan::PunctuationSofMashfaat => PUNCTUATION_SOF_MASHFAAT,
            Samaritan::PunctuationAnnaau => PUNCTUATION_ANNAAU,
        }
    }
}

impl std::convert::TryFrom<char> for Samaritan {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_ALAF => Ok(Samaritan::LetterAlaf),
            LETTER_BIT => Ok(Samaritan::LetterBit),
            LETTER_GAMAN => Ok(Samaritan::LetterGaman),
            LETTER_DALAT => Ok(Samaritan::LetterDalat),
            LETTER_IY => Ok(Samaritan::LetterIy),
            LETTER_BAA => Ok(Samaritan::LetterBaa),
            LETTER_ZEN => Ok(Samaritan::LetterZen),
            LETTER_IT => Ok(Samaritan::LetterIt),
            LETTER_TIT => Ok(Samaritan::LetterTit),
            LETTER_YUT => Ok(Samaritan::LetterYut),
            LETTER_KAAF => Ok(Samaritan::LetterKaaf),
            LETTER_LABAT => Ok(Samaritan::LetterLabat),
            LETTER_MIM => Ok(Samaritan::LetterMim),
            LETTER_NUN => Ok(Samaritan::LetterNun),
            LETTER_SINGAAT => Ok(Samaritan::LetterSingaat),
            LETTER_IN => Ok(Samaritan::LetterIn),
            LETTER_FI => Ok(Samaritan::LetterFi),
            LETTER_TSAADIY => Ok(Samaritan::LetterTsaadiy),
            LETTER_QUF => Ok(Samaritan::LetterQuf),
            LETTER_RISH => Ok(Samaritan::LetterRish),
            LETTER_SHAN => Ok(Samaritan::LetterShan),
            LETTER_TAAF => Ok(Samaritan::LetterTaaf),
            MARK_IN => Ok(Samaritan::MarkIn),
            MARK_IN_DASH_ALAF => Ok(Samaritan::MarkInDashAlaf),
            MARK_OCCLUSION => Ok(Samaritan::MarkOcclusion),
            MARK_DAGESH => Ok(Samaritan::MarkDagesh),
            MODIFIER_LETTER_EPENTHETIC_YUT => Ok(Samaritan::ModifierLetterEpentheticYut),
            MARK_EPENTHETIC_YUT => Ok(Samaritan::MarkEpentheticYut),
            VOWEL_SIGN_LONG_E => Ok(Samaritan::VowelSignLongE),
            VOWEL_SIGN_E => Ok(Samaritan::VowelSignE),
            VOWEL_SIGN_OVERLONG_AA => Ok(Samaritan::VowelSignOverlongAa),
            VOWEL_SIGN_LONG_AA => Ok(Samaritan::VowelSignLongAa),
            VOWEL_SIGN_AA => Ok(Samaritan::VowelSignAa),
            VOWEL_SIGN_OVERLONG_A => Ok(Samaritan::VowelSignOverlongA),
            VOWEL_SIGN_LONG_A => Ok(Samaritan::VowelSignLongA),
            VOWEL_SIGN_A => Ok(Samaritan::VowelSignA),
            MODIFIER_LETTER_SHORT_A => Ok(Samaritan::ModifierLetterShortA),
            VOWEL_SIGN_SHORT_A => Ok(Samaritan::VowelSignShortA),
            VOWEL_SIGN_LONG_U => Ok(Samaritan::VowelSignLongU),
            VOWEL_SIGN_U => Ok(Samaritan::VowelSignU),
            MODIFIER_LETTER_I => Ok(Samaritan::ModifierLetterI),
            VOWEL_SIGN_LONG_I => Ok(Samaritan::VowelSignLongI),
            VOWEL_SIGN_I => Ok(Samaritan::VowelSignI),
            VOWEL_SIGN_O => Ok(Samaritan::VowelSignO),
            VOWEL_SIGN_SUKUN => Ok(Samaritan::VowelSignSukun),
            MARK_NEQUDAA => Ok(Samaritan::MarkNequdaa),
            PUNCTUATION_NEQUDAA => Ok(Samaritan::PunctuationNequdaa),
            PUNCTUATION_AFSAAQ => Ok(Samaritan::PunctuationAfsaaq),
            PUNCTUATION_ANGED => Ok(Samaritan::PunctuationAnged),
            PUNCTUATION_BAU => Ok(Samaritan::PunctuationBau),
            PUNCTUATION_ATMAAU => Ok(Samaritan::PunctuationAtmaau),
            PUNCTUATION_SHIYYAALAA => Ok(Samaritan::PunctuationShiyyaalaa),
            ABBREVIATION_MARK => Ok(Samaritan::AbbreviationMark),
            PUNCTUATION_MELODIC_QITSA => Ok(Samaritan::PunctuationMelodicQitsa),
            PUNCTUATION_ZIQAA => Ok(Samaritan::PunctuationZiqaa),
            PUNCTUATION_QITSA => Ok(Samaritan::PunctuationQitsa),
            PUNCTUATION_ZAEF => Ok(Samaritan::PunctuationZaef),
            PUNCTUATION_TURU => Ok(Samaritan::PunctuationTuru),
            PUNCTUATION_ARKAANU => Ok(Samaritan::PunctuationArkaanu),
            PUNCTUATION_SOF_MASHFAAT => Ok(Samaritan::PunctuationSofMashfaat),
            PUNCTUATION_ANNAAU => Ok(Samaritan::PunctuationAnnaau),
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
