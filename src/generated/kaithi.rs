/// \u{11080} → \u{110cf}\
///\
/// 𑂀 𑂁 𑂂 𑂃 𑂄 𑂅 𑂆 𑂇 𑂈 𑂉 𑂊 𑂋 𑂌 𑂍 𑂎 𑂏\
/// 𑂐 𑂑 𑂒 𑂓 𑂔 𑂕 𑂖 𑂗 𑂘 𑂙 𑂚 𑂛 𑂜 𑂝 𑂞 𑂟\
/// 𑂠 𑂡 𑂢 𑂣 𑂤 𑂥 𑂦 𑂧 𑂨 𑂩 𑂪 𑂫 𑂬 𑂭 𑂮 𑂯\
/// 𑂰 𑂱 𑂲 𑂳 𑂴 𑂵 𑂶 𑂷 𑂸 𑂹 𑂺 𑂻 𑂼 𑂽 𑂾 𑂿\
/// 𑃀 𑃁 𑃍\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{11080}: '𑂀'
    pub const SIGN_CANDRABINDU: char = '𑂀';
    /// \u{11081}: '𑂁'
    pub const SIGN_ANUSVARA: char = '𑂁';
    /// \u{11082}: '𑂂'
    pub const SIGN_VISARGA: char = '𑂂';
    /// \u{11083}: '𑂃'
    pub const LETTER_A: char = '𑂃';
    /// \u{11084}: '𑂄'
    pub const LETTER_AA: char = '𑂄';
    /// \u{11085}: '𑂅'
    pub const LETTER_I: char = '𑂅';
    /// \u{11086}: '𑂆'
    pub const LETTER_II: char = '𑂆';
    /// \u{11087}: '𑂇'
    pub const LETTER_U: char = '𑂇';
    /// \u{11088}: '𑂈'
    pub const LETTER_UU: char = '𑂈';
    /// \u{11089}: '𑂉'
    pub const LETTER_E: char = '𑂉';
    /// \u{1108a}: '𑂊'
    pub const LETTER_AI: char = '𑂊';
    /// \u{1108b}: '𑂋'
    pub const LETTER_O: char = '𑂋';
    /// \u{1108c}: '𑂌'
    pub const LETTER_AU: char = '𑂌';
    /// \u{1108d}: '𑂍'
    pub const LETTER_KA: char = '𑂍';
    /// \u{1108e}: '𑂎'
    pub const LETTER_KHA: char = '𑂎';
    /// \u{1108f}: '𑂏'
    pub const LETTER_GA: char = '𑂏';
    /// \u{11090}: '𑂐'
    pub const LETTER_GHA: char = '𑂐';
    /// \u{11091}: '𑂑'
    pub const LETTER_NGA: char = '𑂑';
    /// \u{11092}: '𑂒'
    pub const LETTER_CA: char = '𑂒';
    /// \u{11093}: '𑂓'
    pub const LETTER_CHA: char = '𑂓';
    /// \u{11094}: '𑂔'
    pub const LETTER_JA: char = '𑂔';
    /// \u{11095}: '𑂕'
    pub const LETTER_JHA: char = '𑂕';
    /// \u{11096}: '𑂖'
    pub const LETTER_NYA: char = '𑂖';
    /// \u{11097}: '𑂗'
    pub const LETTER_TTA: char = '𑂗';
    /// \u{11098}: '𑂘'
    pub const LETTER_TTHA: char = '𑂘';
    /// \u{11099}: '𑂙'
    pub const LETTER_DDA: char = '𑂙';
    /// \u{1109a}: '𑂚'
    pub const LETTER_DDDHA: char = '𑂚';
    /// \u{1109b}: '𑂛'
    pub const LETTER_DDHA: char = '𑂛';
    /// \u{1109c}: '𑂜'
    pub const LETTER_RHA: char = '𑂜';
    /// \u{1109d}: '𑂝'
    pub const LETTER_NNA: char = '𑂝';
    /// \u{1109e}: '𑂞'
    pub const LETTER_TA: char = '𑂞';
    /// \u{1109f}: '𑂟'
    pub const LETTER_THA: char = '𑂟';
    /// \u{110a0}: '𑂠'
    pub const LETTER_DA: char = '𑂠';
    /// \u{110a1}: '𑂡'
    pub const LETTER_DHA: char = '𑂡';
    /// \u{110a2}: '𑂢'
    pub const LETTER_NA: char = '𑂢';
    /// \u{110a3}: '𑂣'
    pub const LETTER_PA: char = '𑂣';
    /// \u{110a4}: '𑂤'
    pub const LETTER_PHA: char = '𑂤';
    /// \u{110a5}: '𑂥'
    pub const LETTER_BA: char = '𑂥';
    /// \u{110a6}: '𑂦'
    pub const LETTER_BHA: char = '𑂦';
    /// \u{110a7}: '𑂧'
    pub const LETTER_MA: char = '𑂧';
    /// \u{110a8}: '𑂨'
    pub const LETTER_YA: char = '𑂨';
    /// \u{110a9}: '𑂩'
    pub const LETTER_RA: char = '𑂩';
    /// \u{110aa}: '𑂪'
    pub const LETTER_LA: char = '𑂪';
    /// \u{110ab}: '𑂫'
    pub const LETTER_VA: char = '𑂫';
    /// \u{110ac}: '𑂬'
    pub const LETTER_SHA: char = '𑂬';
    /// \u{110ad}: '𑂭'
    pub const LETTER_SSA: char = '𑂭';
    /// \u{110ae}: '𑂮'
    pub const LETTER_SA: char = '𑂮';
    /// \u{110af}: '𑂯'
    pub const LETTER_HA: char = '𑂯';
    /// \u{110b0}: '𑂰'
    pub const VOWEL_SIGN_AA: char = '𑂰';
    /// \u{110b1}: '𑂱'
    pub const VOWEL_SIGN_I: char = '𑂱';
    /// \u{110b2}: '𑂲'
    pub const VOWEL_SIGN_II: char = '𑂲';
    /// \u{110b3}: '𑂳'
    pub const VOWEL_SIGN_U: char = '𑂳';
    /// \u{110b4}: '𑂴'
    pub const VOWEL_SIGN_UU: char = '𑂴';
    /// \u{110b5}: '𑂵'
    pub const VOWEL_SIGN_E: char = '𑂵';
    /// \u{110b6}: '𑂶'
    pub const VOWEL_SIGN_AI: char = '𑂶';
    /// \u{110b7}: '𑂷'
    pub const VOWEL_SIGN_O: char = '𑂷';
    /// \u{110b8}: '𑂸'
    pub const VOWEL_SIGN_AU: char = '𑂸';
    /// \u{110b9}: '𑂹'
    pub const SIGN_VIRAMA: char = '𑂹';
    /// \u{110ba}: '𑂺'
    pub const SIGN_NUKTA: char = '𑂺';
    /// \u{110bb}: '𑂻'
    pub const ABBREVIATION_SIGN: char = '𑂻';
    /// \u{110bc}: '𑂼'
    pub const ENUMERATION_SIGN: char = '𑂼';
    /// \u{110bd}: '𑂽'
    pub const NUMBER_SIGN: char = '𑂽';
    /// \u{110be}: '𑂾'
    pub const SECTION_MARK: char = '𑂾';
    /// \u{110bf}: '𑂿'
    pub const DOUBLE_SECTION_MARK: char = '𑂿';
    /// \u{110c0}: '𑃀'
    pub const DANDA: char = '𑃀';
    /// \u{110c1}: '𑃁'
    pub const DOUBLE_DANDA: char = '𑃁';
    /// \u{110cd}: '𑃍'
    pub const NUMBER_SIGN_ABOVE: char = '𑃍';
}

/// An enum to represent all characters in the Kaithi block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Kaithi {
    /// \u{11080}: '𑂀'
    SignCandrabindu,
    /// \u{11081}: '𑂁'
    SignAnusvara,
    /// \u{11082}: '𑂂'
    SignVisarga,
    /// \u{11083}: '𑂃'
    LetterA,
    /// \u{11084}: '𑂄'
    LetterAa,
    /// \u{11085}: '𑂅'
    LetterI,
    /// \u{11086}: '𑂆'
    LetterIi,
    /// \u{11087}: '𑂇'
    LetterU,
    /// \u{11088}: '𑂈'
    LetterUu,
    /// \u{11089}: '𑂉'
    LetterE,
    /// \u{1108a}: '𑂊'
    LetterAi,
    /// \u{1108b}: '𑂋'
    LetterO,
    /// \u{1108c}: '𑂌'
    LetterAu,
    /// \u{1108d}: '𑂍'
    LetterKa,
    /// \u{1108e}: '𑂎'
    LetterKha,
    /// \u{1108f}: '𑂏'
    LetterGa,
    /// \u{11090}: '𑂐'
    LetterGha,
    /// \u{11091}: '𑂑'
    LetterNga,
    /// \u{11092}: '𑂒'
    LetterCa,
    /// \u{11093}: '𑂓'
    LetterCha,
    /// \u{11094}: '𑂔'
    LetterJa,
    /// \u{11095}: '𑂕'
    LetterJha,
    /// \u{11096}: '𑂖'
    LetterNya,
    /// \u{11097}: '𑂗'
    LetterTta,
    /// \u{11098}: '𑂘'
    LetterTtha,
    /// \u{11099}: '𑂙'
    LetterDda,
    /// \u{1109a}: '𑂚'
    LetterDddha,
    /// \u{1109b}: '𑂛'
    LetterDdha,
    /// \u{1109c}: '𑂜'
    LetterRha,
    /// \u{1109d}: '𑂝'
    LetterNna,
    /// \u{1109e}: '𑂞'
    LetterTa,
    /// \u{1109f}: '𑂟'
    LetterTha,
    /// \u{110a0}: '𑂠'
    LetterDa,
    /// \u{110a1}: '𑂡'
    LetterDha,
    /// \u{110a2}: '𑂢'
    LetterNa,
    /// \u{110a3}: '𑂣'
    LetterPa,
    /// \u{110a4}: '𑂤'
    LetterPha,
    /// \u{110a5}: '𑂥'
    LetterBa,
    /// \u{110a6}: '𑂦'
    LetterBha,
    /// \u{110a7}: '𑂧'
    LetterMa,
    /// \u{110a8}: '𑂨'
    LetterYa,
    /// \u{110a9}: '𑂩'
    LetterRa,
    /// \u{110aa}: '𑂪'
    LetterLa,
    /// \u{110ab}: '𑂫'
    LetterVa,
    /// \u{110ac}: '𑂬'
    LetterSha,
    /// \u{110ad}: '𑂭'
    LetterSsa,
    /// \u{110ae}: '𑂮'
    LetterSa,
    /// \u{110af}: '𑂯'
    LetterHa,
    /// \u{110b0}: '𑂰'
    VowelSignAa,
    /// \u{110b1}: '𑂱'
    VowelSignI,
    /// \u{110b2}: '𑂲'
    VowelSignIi,
    /// \u{110b3}: '𑂳'
    VowelSignU,
    /// \u{110b4}: '𑂴'
    VowelSignUu,
    /// \u{110b5}: '𑂵'
    VowelSignE,
    /// \u{110b6}: '𑂶'
    VowelSignAi,
    /// \u{110b7}: '𑂷'
    VowelSignO,
    /// \u{110b8}: '𑂸'
    VowelSignAu,
    /// \u{110b9}: '𑂹'
    SignVirama,
    /// \u{110ba}: '𑂺'
    SignNukta,
    /// \u{110bb}: '𑂻'
    AbbreviationSign,
    /// \u{110bc}: '𑂼'
    EnumerationSign,
    /// \u{110bd}: '𑂽'
    NumberSign,
    /// \u{110be}: '𑂾'
    SectionMark,
    /// \u{110bf}: '𑂿'
    DoubleSectionMark,
    /// \u{110c0}: '𑃀'
    Danda,
    /// \u{110c1}: '𑃁'
    DoubleDanda,
    /// \u{110cd}: '𑃍'
    NumberSignAbove,
}

impl Into<char> for Kaithi {
    fn into(self) -> char {
        use constants::*;
        match self {
            Kaithi::SignCandrabindu => SIGN_CANDRABINDU,
            Kaithi::SignAnusvara => SIGN_ANUSVARA,
            Kaithi::SignVisarga => SIGN_VISARGA,
            Kaithi::LetterA => LETTER_A,
            Kaithi::LetterAa => LETTER_AA,
            Kaithi::LetterI => LETTER_I,
            Kaithi::LetterIi => LETTER_II,
            Kaithi::LetterU => LETTER_U,
            Kaithi::LetterUu => LETTER_UU,
            Kaithi::LetterE => LETTER_E,
            Kaithi::LetterAi => LETTER_AI,
            Kaithi::LetterO => LETTER_O,
            Kaithi::LetterAu => LETTER_AU,
            Kaithi::LetterKa => LETTER_KA,
            Kaithi::LetterKha => LETTER_KHA,
            Kaithi::LetterGa => LETTER_GA,
            Kaithi::LetterGha => LETTER_GHA,
            Kaithi::LetterNga => LETTER_NGA,
            Kaithi::LetterCa => LETTER_CA,
            Kaithi::LetterCha => LETTER_CHA,
            Kaithi::LetterJa => LETTER_JA,
            Kaithi::LetterJha => LETTER_JHA,
            Kaithi::LetterNya => LETTER_NYA,
            Kaithi::LetterTta => LETTER_TTA,
            Kaithi::LetterTtha => LETTER_TTHA,
            Kaithi::LetterDda => LETTER_DDA,
            Kaithi::LetterDddha => LETTER_DDDHA,
            Kaithi::LetterDdha => LETTER_DDHA,
            Kaithi::LetterRha => LETTER_RHA,
            Kaithi::LetterNna => LETTER_NNA,
            Kaithi::LetterTa => LETTER_TA,
            Kaithi::LetterTha => LETTER_THA,
            Kaithi::LetterDa => LETTER_DA,
            Kaithi::LetterDha => LETTER_DHA,
            Kaithi::LetterNa => LETTER_NA,
            Kaithi::LetterPa => LETTER_PA,
            Kaithi::LetterPha => LETTER_PHA,
            Kaithi::LetterBa => LETTER_BA,
            Kaithi::LetterBha => LETTER_BHA,
            Kaithi::LetterMa => LETTER_MA,
            Kaithi::LetterYa => LETTER_YA,
            Kaithi::LetterRa => LETTER_RA,
            Kaithi::LetterLa => LETTER_LA,
            Kaithi::LetterVa => LETTER_VA,
            Kaithi::LetterSha => LETTER_SHA,
            Kaithi::LetterSsa => LETTER_SSA,
            Kaithi::LetterSa => LETTER_SA,
            Kaithi::LetterHa => LETTER_HA,
            Kaithi::VowelSignAa => VOWEL_SIGN_AA,
            Kaithi::VowelSignI => VOWEL_SIGN_I,
            Kaithi::VowelSignIi => VOWEL_SIGN_II,
            Kaithi::VowelSignU => VOWEL_SIGN_U,
            Kaithi::VowelSignUu => VOWEL_SIGN_UU,
            Kaithi::VowelSignE => VOWEL_SIGN_E,
            Kaithi::VowelSignAi => VOWEL_SIGN_AI,
            Kaithi::VowelSignO => VOWEL_SIGN_O,
            Kaithi::VowelSignAu => VOWEL_SIGN_AU,
            Kaithi::SignVirama => SIGN_VIRAMA,
            Kaithi::SignNukta => SIGN_NUKTA,
            Kaithi::AbbreviationSign => ABBREVIATION_SIGN,
            Kaithi::EnumerationSign => ENUMERATION_SIGN,
            Kaithi::NumberSign => NUMBER_SIGN,
            Kaithi::SectionMark => SECTION_MARK,
            Kaithi::DoubleSectionMark => DOUBLE_SECTION_MARK,
            Kaithi::Danda => DANDA,
            Kaithi::DoubleDanda => DOUBLE_DANDA,
            Kaithi::NumberSignAbove => NUMBER_SIGN_ABOVE,
        }
    }
}

impl std::convert::TryFrom<char> for Kaithi {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            SIGN_CANDRABINDU => Ok(Kaithi::SignCandrabindu),
            SIGN_ANUSVARA => Ok(Kaithi::SignAnusvara),
            SIGN_VISARGA => Ok(Kaithi::SignVisarga),
            LETTER_A => Ok(Kaithi::LetterA),
            LETTER_AA => Ok(Kaithi::LetterAa),
            LETTER_I => Ok(Kaithi::LetterI),
            LETTER_II => Ok(Kaithi::LetterIi),
            LETTER_U => Ok(Kaithi::LetterU),
            LETTER_UU => Ok(Kaithi::LetterUu),
            LETTER_E => Ok(Kaithi::LetterE),
            LETTER_AI => Ok(Kaithi::LetterAi),
            LETTER_O => Ok(Kaithi::LetterO),
            LETTER_AU => Ok(Kaithi::LetterAu),
            LETTER_KA => Ok(Kaithi::LetterKa),
            LETTER_KHA => Ok(Kaithi::LetterKha),
            LETTER_GA => Ok(Kaithi::LetterGa),
            LETTER_GHA => Ok(Kaithi::LetterGha),
            LETTER_NGA => Ok(Kaithi::LetterNga),
            LETTER_CA => Ok(Kaithi::LetterCa),
            LETTER_CHA => Ok(Kaithi::LetterCha),
            LETTER_JA => Ok(Kaithi::LetterJa),
            LETTER_JHA => Ok(Kaithi::LetterJha),
            LETTER_NYA => Ok(Kaithi::LetterNya),
            LETTER_TTA => Ok(Kaithi::LetterTta),
            LETTER_TTHA => Ok(Kaithi::LetterTtha),
            LETTER_DDA => Ok(Kaithi::LetterDda),
            LETTER_DDDHA => Ok(Kaithi::LetterDddha),
            LETTER_DDHA => Ok(Kaithi::LetterDdha),
            LETTER_RHA => Ok(Kaithi::LetterRha),
            LETTER_NNA => Ok(Kaithi::LetterNna),
            LETTER_TA => Ok(Kaithi::LetterTa),
            LETTER_THA => Ok(Kaithi::LetterTha),
            LETTER_DA => Ok(Kaithi::LetterDa),
            LETTER_DHA => Ok(Kaithi::LetterDha),
            LETTER_NA => Ok(Kaithi::LetterNa),
            LETTER_PA => Ok(Kaithi::LetterPa),
            LETTER_PHA => Ok(Kaithi::LetterPha),
            LETTER_BA => Ok(Kaithi::LetterBa),
            LETTER_BHA => Ok(Kaithi::LetterBha),
            LETTER_MA => Ok(Kaithi::LetterMa),
            LETTER_YA => Ok(Kaithi::LetterYa),
            LETTER_RA => Ok(Kaithi::LetterRa),
            LETTER_LA => Ok(Kaithi::LetterLa),
            LETTER_VA => Ok(Kaithi::LetterVa),
            LETTER_SHA => Ok(Kaithi::LetterSha),
            LETTER_SSA => Ok(Kaithi::LetterSsa),
            LETTER_SA => Ok(Kaithi::LetterSa),
            LETTER_HA => Ok(Kaithi::LetterHa),
            VOWEL_SIGN_AA => Ok(Kaithi::VowelSignAa),
            VOWEL_SIGN_I => Ok(Kaithi::VowelSignI),
            VOWEL_SIGN_II => Ok(Kaithi::VowelSignIi),
            VOWEL_SIGN_U => Ok(Kaithi::VowelSignU),
            VOWEL_SIGN_UU => Ok(Kaithi::VowelSignUu),
            VOWEL_SIGN_E => Ok(Kaithi::VowelSignE),
            VOWEL_SIGN_AI => Ok(Kaithi::VowelSignAi),
            VOWEL_SIGN_O => Ok(Kaithi::VowelSignO),
            VOWEL_SIGN_AU => Ok(Kaithi::VowelSignAu),
            SIGN_VIRAMA => Ok(Kaithi::SignVirama),
            SIGN_NUKTA => Ok(Kaithi::SignNukta),
            ABBREVIATION_SIGN => Ok(Kaithi::AbbreviationSign),
            ENUMERATION_SIGN => Ok(Kaithi::EnumerationSign),
            NUMBER_SIGN => Ok(Kaithi::NumberSign),
            SECTION_MARK => Ok(Kaithi::SectionMark),
            DOUBLE_SECTION_MARK => Ok(Kaithi::DoubleSectionMark),
            DANDA => Ok(Kaithi::Danda),
            DOUBLE_DANDA => Ok(Kaithi::DoubleDanda),
            NUMBER_SIGN_ABOVE => Ok(Kaithi::NumberSignAbove),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Kaithi {
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

impl std::convert::TryFrom<u32> for Kaithi {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Kaithi {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Kaithi {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Kaithi::SignCandrabindu
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Kaithi::SignCandrabindu => "kaithi sign candrabindu",
            Kaithi::SignAnusvara => "kaithi sign anusvara",
            Kaithi::SignVisarga => "kaithi sign visarga",
            Kaithi::LetterA => "kaithi letter a",
            Kaithi::LetterAa => "kaithi letter aa",
            Kaithi::LetterI => "kaithi letter i",
            Kaithi::LetterIi => "kaithi letter ii",
            Kaithi::LetterU => "kaithi letter u",
            Kaithi::LetterUu => "kaithi letter uu",
            Kaithi::LetterE => "kaithi letter e",
            Kaithi::LetterAi => "kaithi letter ai",
            Kaithi::LetterO => "kaithi letter o",
            Kaithi::LetterAu => "kaithi letter au",
            Kaithi::LetterKa => "kaithi letter ka",
            Kaithi::LetterKha => "kaithi letter kha",
            Kaithi::LetterGa => "kaithi letter ga",
            Kaithi::LetterGha => "kaithi letter gha",
            Kaithi::LetterNga => "kaithi letter nga",
            Kaithi::LetterCa => "kaithi letter ca",
            Kaithi::LetterCha => "kaithi letter cha",
            Kaithi::LetterJa => "kaithi letter ja",
            Kaithi::LetterJha => "kaithi letter jha",
            Kaithi::LetterNya => "kaithi letter nya",
            Kaithi::LetterTta => "kaithi letter tta",
            Kaithi::LetterTtha => "kaithi letter ttha",
            Kaithi::LetterDda => "kaithi letter dda",
            Kaithi::LetterDddha => "kaithi letter dddha",
            Kaithi::LetterDdha => "kaithi letter ddha",
            Kaithi::LetterRha => "kaithi letter rha",
            Kaithi::LetterNna => "kaithi letter nna",
            Kaithi::LetterTa => "kaithi letter ta",
            Kaithi::LetterTha => "kaithi letter tha",
            Kaithi::LetterDa => "kaithi letter da",
            Kaithi::LetterDha => "kaithi letter dha",
            Kaithi::LetterNa => "kaithi letter na",
            Kaithi::LetterPa => "kaithi letter pa",
            Kaithi::LetterPha => "kaithi letter pha",
            Kaithi::LetterBa => "kaithi letter ba",
            Kaithi::LetterBha => "kaithi letter bha",
            Kaithi::LetterMa => "kaithi letter ma",
            Kaithi::LetterYa => "kaithi letter ya",
            Kaithi::LetterRa => "kaithi letter ra",
            Kaithi::LetterLa => "kaithi letter la",
            Kaithi::LetterVa => "kaithi letter va",
            Kaithi::LetterSha => "kaithi letter sha",
            Kaithi::LetterSsa => "kaithi letter ssa",
            Kaithi::LetterSa => "kaithi letter sa",
            Kaithi::LetterHa => "kaithi letter ha",
            Kaithi::VowelSignAa => "kaithi vowel sign aa",
            Kaithi::VowelSignI => "kaithi vowel sign i",
            Kaithi::VowelSignIi => "kaithi vowel sign ii",
            Kaithi::VowelSignU => "kaithi vowel sign u",
            Kaithi::VowelSignUu => "kaithi vowel sign uu",
            Kaithi::VowelSignE => "kaithi vowel sign e",
            Kaithi::VowelSignAi => "kaithi vowel sign ai",
            Kaithi::VowelSignO => "kaithi vowel sign o",
            Kaithi::VowelSignAu => "kaithi vowel sign au",
            Kaithi::SignVirama => "kaithi sign virama",
            Kaithi::SignNukta => "kaithi sign nukta",
            Kaithi::AbbreviationSign => "kaithi abbreviation sign",
            Kaithi::EnumerationSign => "kaithi enumeration sign",
            Kaithi::NumberSign => "kaithi number sign",
            Kaithi::SectionMark => "kaithi section mark",
            Kaithi::DoubleSectionMark => "kaithi double section mark",
            Kaithi::Danda => "kaithi danda",
            Kaithi::DoubleDanda => "kaithi double danda",
            Kaithi::NumberSignAbove => "kaithi number sign above",
        }
    }
}
