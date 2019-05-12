/// A number of constants to give a name to all characters in this block.
mod constants {
    /// \u{11200}: '𑈀'
    pub const LETTER_A: char = '𑈀';
    /// \u{11201}: '𑈁'
    pub const LETTER_AA: char = '𑈁';
    /// \u{11202}: '𑈂'
    pub const LETTER_I: char = '𑈂';
    /// \u{11203}: '𑈃'
    pub const LETTER_U: char = '𑈃';
    /// \u{11204}: '𑈄'
    pub const LETTER_E: char = '𑈄';
    /// \u{11205}: '𑈅'
    pub const LETTER_AI: char = '𑈅';
    /// \u{11206}: '𑈆'
    pub const LETTER_O: char = '𑈆';
    /// \u{11207}: '𑈇'
    pub const LETTER_AU: char = '𑈇';
    /// \u{11208}: '𑈈'
    pub const LETTER_KA: char = '𑈈';
    /// \u{11209}: '𑈉'
    pub const LETTER_KHA: char = '𑈉';
    /// \u{1120a}: '𑈊'
    pub const LETTER_GA: char = '𑈊';
    /// \u{1120b}: '𑈋'
    pub const LETTER_GGA: char = '𑈋';
    /// \u{1120c}: '𑈌'
    pub const LETTER_GHA: char = '𑈌';
    /// \u{1120d}: '𑈍'
    pub const LETTER_NGA: char = '𑈍';
    /// \u{1120e}: '𑈎'
    pub const LETTER_CA: char = '𑈎';
    /// \u{1120f}: '𑈏'
    pub const LETTER_CHA: char = '𑈏';
    /// \u{11210}: '𑈐'
    pub const LETTER_JA: char = '𑈐';
    /// \u{11211}: '𑈑'
    pub const LETTER_JJA: char = '𑈑';
    /// \u{11213}: '𑈓'
    pub const LETTER_NYA: char = '𑈓';
    /// \u{11214}: '𑈔'
    pub const LETTER_TTA: char = '𑈔';
    /// \u{11215}: '𑈕'
    pub const LETTER_TTHA: char = '𑈕';
    /// \u{11216}: '𑈖'
    pub const LETTER_DDA: char = '𑈖';
    /// \u{11217}: '𑈗'
    pub const LETTER_DDHA: char = '𑈗';
    /// \u{11218}: '𑈘'
    pub const LETTER_NNA: char = '𑈘';
    /// \u{11219}: '𑈙'
    pub const LETTER_TA: char = '𑈙';
    /// \u{1121a}: '𑈚'
    pub const LETTER_THA: char = '𑈚';
    /// \u{1121b}: '𑈛'
    pub const LETTER_DA: char = '𑈛';
    /// \u{1121c}: '𑈜'
    pub const LETTER_DDDA: char = '𑈜';
    /// \u{1121d}: '𑈝'
    pub const LETTER_DHA: char = '𑈝';
    /// \u{1121e}: '𑈞'
    pub const LETTER_NA: char = '𑈞';
    /// \u{1121f}: '𑈟'
    pub const LETTER_PA: char = '𑈟';
    /// \u{11220}: '𑈠'
    pub const LETTER_PHA: char = '𑈠';
    /// \u{11221}: '𑈡'
    pub const LETTER_BA: char = '𑈡';
    /// \u{11222}: '𑈢'
    pub const LETTER_BBA: char = '𑈢';
    /// \u{11223}: '𑈣'
    pub const LETTER_BHA: char = '𑈣';
    /// \u{11224}: '𑈤'
    pub const LETTER_MA: char = '𑈤';
    /// \u{11225}: '𑈥'
    pub const LETTER_YA: char = '𑈥';
    /// \u{11226}: '𑈦'
    pub const LETTER_RA: char = '𑈦';
    /// \u{11227}: '𑈧'
    pub const LETTER_LA: char = '𑈧';
    /// \u{11228}: '𑈨'
    pub const LETTER_VA: char = '𑈨';
    /// \u{11229}: '𑈩'
    pub const LETTER_SA: char = '𑈩';
    /// \u{1122a}: '𑈪'
    pub const LETTER_HA: char = '𑈪';
    /// \u{1122b}: '𑈫'
    pub const LETTER_LLA: char = '𑈫';
    /// \u{1122c}: '𑈬'
    pub const VOWEL_SIGN_AA: char = '𑈬';
    /// \u{1122d}: '𑈭'
    pub const VOWEL_SIGN_I: char = '𑈭';
    /// \u{1122e}: '𑈮'
    pub const VOWEL_SIGN_II: char = '𑈮';
    /// \u{1122f}: '𑈯'
    pub const VOWEL_SIGN_U: char = '𑈯';
    /// \u{11230}: '𑈰'
    pub const VOWEL_SIGN_E: char = '𑈰';
    /// \u{11231}: '𑈱'
    pub const VOWEL_SIGN_AI: char = '𑈱';
    /// \u{11232}: '𑈲'
    pub const VOWEL_SIGN_O: char = '𑈲';
    /// \u{11233}: '𑈳'
    pub const VOWEL_SIGN_AU: char = '𑈳';
    /// \u{11234}: '𑈴'
    pub const SIGN_ANUSVARA: char = '𑈴';
    /// \u{11235}: '𑈵'
    pub const SIGN_VIRAMA: char = '𑈵';
    /// \u{11236}: '𑈶'
    pub const SIGN_NUKTA: char = '𑈶';
    /// \u{11237}: '𑈷'
    pub const SIGN_SHADDA: char = '𑈷';
    /// \u{11238}: '𑈸'
    pub const DANDA: char = '𑈸';
    /// \u{11239}: '𑈹'
    pub const DOUBLE_DANDA: char = '𑈹';
    /// \u{1123a}: '𑈺'
    pub const WORD_SEPARATOR: char = '𑈺';
    /// \u{1123b}: '𑈻'
    pub const SECTION_MARK: char = '𑈻';
    /// \u{1123c}: '𑈼'
    pub const DOUBLE_SECTION_MARK: char = '𑈼';
    /// \u{1123d}: '𑈽'
    pub const ABBREVIATION_SIGN: char = '𑈽';
    /// \u{1123e}: '𑈾'
    pub const SIGN_SUKUN: char = '𑈾';
}

/// An enum to represent all characters in the Khojki block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Khojki {
    /// \u{11200}: '𑈀'
    LetterA,
    /// \u{11201}: '𑈁'
    LetterAa,
    /// \u{11202}: '𑈂'
    LetterI,
    /// \u{11203}: '𑈃'
    LetterU,
    /// \u{11204}: '𑈄'
    LetterE,
    /// \u{11205}: '𑈅'
    LetterAi,
    /// \u{11206}: '𑈆'
    LetterO,
    /// \u{11207}: '𑈇'
    LetterAu,
    /// \u{11208}: '𑈈'
    LetterKa,
    /// \u{11209}: '𑈉'
    LetterKha,
    /// \u{1120a}: '𑈊'
    LetterGa,
    /// \u{1120b}: '𑈋'
    LetterGga,
    /// \u{1120c}: '𑈌'
    LetterGha,
    /// \u{1120d}: '𑈍'
    LetterNga,
    /// \u{1120e}: '𑈎'
    LetterCa,
    /// \u{1120f}: '𑈏'
    LetterCha,
    /// \u{11210}: '𑈐'
    LetterJa,
    /// \u{11211}: '𑈑'
    LetterJja,
    /// \u{11213}: '𑈓'
    LetterNya,
    /// \u{11214}: '𑈔'
    LetterTta,
    /// \u{11215}: '𑈕'
    LetterTtha,
    /// \u{11216}: '𑈖'
    LetterDda,
    /// \u{11217}: '𑈗'
    LetterDdha,
    /// \u{11218}: '𑈘'
    LetterNna,
    /// \u{11219}: '𑈙'
    LetterTa,
    /// \u{1121a}: '𑈚'
    LetterTha,
    /// \u{1121b}: '𑈛'
    LetterDa,
    /// \u{1121c}: '𑈜'
    LetterDdda,
    /// \u{1121d}: '𑈝'
    LetterDha,
    /// \u{1121e}: '𑈞'
    LetterNa,
    /// \u{1121f}: '𑈟'
    LetterPa,
    /// \u{11220}: '𑈠'
    LetterPha,
    /// \u{11221}: '𑈡'
    LetterBa,
    /// \u{11222}: '𑈢'
    LetterBba,
    /// \u{11223}: '𑈣'
    LetterBha,
    /// \u{11224}: '𑈤'
    LetterMa,
    /// \u{11225}: '𑈥'
    LetterYa,
    /// \u{11226}: '𑈦'
    LetterRa,
    /// \u{11227}: '𑈧'
    LetterLa,
    /// \u{11228}: '𑈨'
    LetterVa,
    /// \u{11229}: '𑈩'
    LetterSa,
    /// \u{1122a}: '𑈪'
    LetterHa,
    /// \u{1122b}: '𑈫'
    LetterLla,
    /// \u{1122c}: '𑈬'
    VowelSignAa,
    /// \u{1122d}: '𑈭'
    VowelSignI,
    /// \u{1122e}: '𑈮'
    VowelSignIi,
    /// \u{1122f}: '𑈯'
    VowelSignU,
    /// \u{11230}: '𑈰'
    VowelSignE,
    /// \u{11231}: '𑈱'
    VowelSignAi,
    /// \u{11232}: '𑈲'
    VowelSignO,
    /// \u{11233}: '𑈳'
    VowelSignAu,
    /// \u{11234}: '𑈴'
    SignAnusvara,
    /// \u{11235}: '𑈵'
    SignVirama,
    /// \u{11236}: '𑈶'
    SignNukta,
    /// \u{11237}: '𑈷'
    SignShadda,
    /// \u{11238}: '𑈸'
    Danda,
    /// \u{11239}: '𑈹'
    DoubleDanda,
    /// \u{1123a}: '𑈺'
    WordSeparator,
    /// \u{1123b}: '𑈻'
    SectionMark,
    /// \u{1123c}: '𑈼'
    DoubleSectionMark,
    /// \u{1123d}: '𑈽'
    AbbreviationSign,
    /// \u{1123e}: '𑈾'
    SignSukun,
}

impl Into<char> for Khojki {
    fn into(self) -> char {
        use constants::*;
        match self {
            Khojki::LetterA => LETTER_A,
            Khojki::LetterAa => LETTER_AA,
            Khojki::LetterI => LETTER_I,
            Khojki::LetterU => LETTER_U,
            Khojki::LetterE => LETTER_E,
            Khojki::LetterAi => LETTER_AI,
            Khojki::LetterO => LETTER_O,
            Khojki::LetterAu => LETTER_AU,
            Khojki::LetterKa => LETTER_KA,
            Khojki::LetterKha => LETTER_KHA,
            Khojki::LetterGa => LETTER_GA,
            Khojki::LetterGga => LETTER_GGA,
            Khojki::LetterGha => LETTER_GHA,
            Khojki::LetterNga => LETTER_NGA,
            Khojki::LetterCa => LETTER_CA,
            Khojki::LetterCha => LETTER_CHA,
            Khojki::LetterJa => LETTER_JA,
            Khojki::LetterJja => LETTER_JJA,
            Khojki::LetterNya => LETTER_NYA,
            Khojki::LetterTta => LETTER_TTA,
            Khojki::LetterTtha => LETTER_TTHA,
            Khojki::LetterDda => LETTER_DDA,
            Khojki::LetterDdha => LETTER_DDHA,
            Khojki::LetterNna => LETTER_NNA,
            Khojki::LetterTa => LETTER_TA,
            Khojki::LetterTha => LETTER_THA,
            Khojki::LetterDa => LETTER_DA,
            Khojki::LetterDdda => LETTER_DDDA,
            Khojki::LetterDha => LETTER_DHA,
            Khojki::LetterNa => LETTER_NA,
            Khojki::LetterPa => LETTER_PA,
            Khojki::LetterPha => LETTER_PHA,
            Khojki::LetterBa => LETTER_BA,
            Khojki::LetterBba => LETTER_BBA,
            Khojki::LetterBha => LETTER_BHA,
            Khojki::LetterMa => LETTER_MA,
            Khojki::LetterYa => LETTER_YA,
            Khojki::LetterRa => LETTER_RA,
            Khojki::LetterLa => LETTER_LA,
            Khojki::LetterVa => LETTER_VA,
            Khojki::LetterSa => LETTER_SA,
            Khojki::LetterHa => LETTER_HA,
            Khojki::LetterLla => LETTER_LLA,
            Khojki::VowelSignAa => VOWEL_SIGN_AA,
            Khojki::VowelSignI => VOWEL_SIGN_I,
            Khojki::VowelSignIi => VOWEL_SIGN_II,
            Khojki::VowelSignU => VOWEL_SIGN_U,
            Khojki::VowelSignE => VOWEL_SIGN_E,
            Khojki::VowelSignAi => VOWEL_SIGN_AI,
            Khojki::VowelSignO => VOWEL_SIGN_O,
            Khojki::VowelSignAu => VOWEL_SIGN_AU,
            Khojki::SignAnusvara => SIGN_ANUSVARA,
            Khojki::SignVirama => SIGN_VIRAMA,
            Khojki::SignNukta => SIGN_NUKTA,
            Khojki::SignShadda => SIGN_SHADDA,
            Khojki::Danda => DANDA,
            Khojki::DoubleDanda => DOUBLE_DANDA,
            Khojki::WordSeparator => WORD_SEPARATOR,
            Khojki::SectionMark => SECTION_MARK,
            Khojki::DoubleSectionMark => DOUBLE_SECTION_MARK,
            Khojki::AbbreviationSign => ABBREVIATION_SIGN,
            Khojki::SignSukun => SIGN_SUKUN,
        }
    }
}

impl std::convert::TryFrom<char> for Khojki {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_A => Ok(Khojki::LetterA),
            LETTER_AA => Ok(Khojki::LetterAa),
            LETTER_I => Ok(Khojki::LetterI),
            LETTER_U => Ok(Khojki::LetterU),
            LETTER_E => Ok(Khojki::LetterE),
            LETTER_AI => Ok(Khojki::LetterAi),
            LETTER_O => Ok(Khojki::LetterO),
            LETTER_AU => Ok(Khojki::LetterAu),
            LETTER_KA => Ok(Khojki::LetterKa),
            LETTER_KHA => Ok(Khojki::LetterKha),
            LETTER_GA => Ok(Khojki::LetterGa),
            LETTER_GGA => Ok(Khojki::LetterGga),
            LETTER_GHA => Ok(Khojki::LetterGha),
            LETTER_NGA => Ok(Khojki::LetterNga),
            LETTER_CA => Ok(Khojki::LetterCa),
            LETTER_CHA => Ok(Khojki::LetterCha),
            LETTER_JA => Ok(Khojki::LetterJa),
            LETTER_JJA => Ok(Khojki::LetterJja),
            LETTER_NYA => Ok(Khojki::LetterNya),
            LETTER_TTA => Ok(Khojki::LetterTta),
            LETTER_TTHA => Ok(Khojki::LetterTtha),
            LETTER_DDA => Ok(Khojki::LetterDda),
            LETTER_DDHA => Ok(Khojki::LetterDdha),
            LETTER_NNA => Ok(Khojki::LetterNna),
            LETTER_TA => Ok(Khojki::LetterTa),
            LETTER_THA => Ok(Khojki::LetterTha),
            LETTER_DA => Ok(Khojki::LetterDa),
            LETTER_DDDA => Ok(Khojki::LetterDdda),
            LETTER_DHA => Ok(Khojki::LetterDha),
            LETTER_NA => Ok(Khojki::LetterNa),
            LETTER_PA => Ok(Khojki::LetterPa),
            LETTER_PHA => Ok(Khojki::LetterPha),
            LETTER_BA => Ok(Khojki::LetterBa),
            LETTER_BBA => Ok(Khojki::LetterBba),
            LETTER_BHA => Ok(Khojki::LetterBha),
            LETTER_MA => Ok(Khojki::LetterMa),
            LETTER_YA => Ok(Khojki::LetterYa),
            LETTER_RA => Ok(Khojki::LetterRa),
            LETTER_LA => Ok(Khojki::LetterLa),
            LETTER_VA => Ok(Khojki::LetterVa),
            LETTER_SA => Ok(Khojki::LetterSa),
            LETTER_HA => Ok(Khojki::LetterHa),
            LETTER_LLA => Ok(Khojki::LetterLla),
            VOWEL_SIGN_AA => Ok(Khojki::VowelSignAa),
            VOWEL_SIGN_I => Ok(Khojki::VowelSignI),
            VOWEL_SIGN_II => Ok(Khojki::VowelSignIi),
            VOWEL_SIGN_U => Ok(Khojki::VowelSignU),
            VOWEL_SIGN_E => Ok(Khojki::VowelSignE),
            VOWEL_SIGN_AI => Ok(Khojki::VowelSignAi),
            VOWEL_SIGN_O => Ok(Khojki::VowelSignO),
            VOWEL_SIGN_AU => Ok(Khojki::VowelSignAu),
            SIGN_ANUSVARA => Ok(Khojki::SignAnusvara),
            SIGN_VIRAMA => Ok(Khojki::SignVirama),
            SIGN_NUKTA => Ok(Khojki::SignNukta),
            SIGN_SHADDA => Ok(Khojki::SignShadda),
            DANDA => Ok(Khojki::Danda),
            DOUBLE_DANDA => Ok(Khojki::DoubleDanda),
            WORD_SEPARATOR => Ok(Khojki::WordSeparator),
            SECTION_MARK => Ok(Khojki::SectionMark),
            DOUBLE_SECTION_MARK => Ok(Khojki::DoubleSectionMark),
            ABBREVIATION_SIGN => Ok(Khojki::AbbreviationSign),
            SIGN_SUKUN => Ok(Khojki::SignSukun),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Khojki {
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

impl std::convert::TryFrom<u32> for Khojki {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Khojki {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Khojki {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Khojki::LetterA
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Khojki::LetterA => "khojki letter a",
            Khojki::LetterAa => "khojki letter aa",
            Khojki::LetterI => "khojki letter i",
            Khojki::LetterU => "khojki letter u",
            Khojki::LetterE => "khojki letter e",
            Khojki::LetterAi => "khojki letter ai",
            Khojki::LetterO => "khojki letter o",
            Khojki::LetterAu => "khojki letter au",
            Khojki::LetterKa => "khojki letter ka",
            Khojki::LetterKha => "khojki letter kha",
            Khojki::LetterGa => "khojki letter ga",
            Khojki::LetterGga => "khojki letter gga",
            Khojki::LetterGha => "khojki letter gha",
            Khojki::LetterNga => "khojki letter nga",
            Khojki::LetterCa => "khojki letter ca",
            Khojki::LetterCha => "khojki letter cha",
            Khojki::LetterJa => "khojki letter ja",
            Khojki::LetterJja => "khojki letter jja",
            Khojki::LetterNya => "khojki letter nya",
            Khojki::LetterTta => "khojki letter tta",
            Khojki::LetterTtha => "khojki letter ttha",
            Khojki::LetterDda => "khojki letter dda",
            Khojki::LetterDdha => "khojki letter ddha",
            Khojki::LetterNna => "khojki letter nna",
            Khojki::LetterTa => "khojki letter ta",
            Khojki::LetterTha => "khojki letter tha",
            Khojki::LetterDa => "khojki letter da",
            Khojki::LetterDdda => "khojki letter ddda",
            Khojki::LetterDha => "khojki letter dha",
            Khojki::LetterNa => "khojki letter na",
            Khojki::LetterPa => "khojki letter pa",
            Khojki::LetterPha => "khojki letter pha",
            Khojki::LetterBa => "khojki letter ba",
            Khojki::LetterBba => "khojki letter bba",
            Khojki::LetterBha => "khojki letter bha",
            Khojki::LetterMa => "khojki letter ma",
            Khojki::LetterYa => "khojki letter ya",
            Khojki::LetterRa => "khojki letter ra",
            Khojki::LetterLa => "khojki letter la",
            Khojki::LetterVa => "khojki letter va",
            Khojki::LetterSa => "khojki letter sa",
            Khojki::LetterHa => "khojki letter ha",
            Khojki::LetterLla => "khojki letter lla",
            Khojki::VowelSignAa => "khojki vowel sign aa",
            Khojki::VowelSignI => "khojki vowel sign i",
            Khojki::VowelSignIi => "khojki vowel sign ii",
            Khojki::VowelSignU => "khojki vowel sign u",
            Khojki::VowelSignE => "khojki vowel sign e",
            Khojki::VowelSignAi => "khojki vowel sign ai",
            Khojki::VowelSignO => "khojki vowel sign o",
            Khojki::VowelSignAu => "khojki vowel sign au",
            Khojki::SignAnusvara => "khojki sign anusvara",
            Khojki::SignVirama => "khojki sign virama",
            Khojki::SignNukta => "khojki sign nukta",
            Khojki::SignShadda => "khojki sign shadda",
            Khojki::Danda => "khojki danda",
            Khojki::DoubleDanda => "khojki double danda",
            Khojki::WordSeparator => "khojki word separator",
            Khojki::SectionMark => "khojki section mark",
            Khojki::DoubleSectionMark => "khojki double section mark",
            Khojki::AbbreviationSign => "khojki abbreviation sign",
            Khojki::SignSukun => "khojki sign sukun",
        }
    }
}
