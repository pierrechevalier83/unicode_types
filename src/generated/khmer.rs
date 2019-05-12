/// \u{1780} → \u{17ff}\
///\
/// ក ខ គ ឃ ង ច ឆ ជ ឈ ញ ដ ឋ ឌ ឍ ណ ត\
/// ថ ទ ធ ន ប ផ ព ភ ម យ រ ល វ ឝ ឞ ស\
/// ហ ឡ អ ឣ ឤ ឥ ឦ ឧ ឨ ឩ ឪ ឫ ឬ ឭ ឮ ឯ\
/// ឰ ឱ ឲ ឳ ឴ ឵ ា ិ ី ឹ ឺ ុ ូ ួ ើ ឿ\
/// ៀ េ ែ ៃ ោ ៅ ំ ះ ៈ ៉ ៊ ់ ៌ ៍ ៎ ៏\
/// ័ ៑ ្ ៓ ។ ៕ ៖ ៗ ៘ ៙ ៚ ៛ ៜ ៝ ០ ១\
/// ២ ៣ ៤ ៥ ៦ ៧ ៨ ៩ ៰ ៱ ៲ ៳ ៴ ៵ ៶ ៷\
/// ៸ ៹\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{1780}: 'ក'
    pub const LETTER_KA: char = 'ក';
    /// \u{1781}: 'ខ'
    pub const LETTER_KHA: char = 'ខ';
    /// \u{1782}: 'គ'
    pub const LETTER_KO: char = 'គ';
    /// \u{1783}: 'ឃ'
    pub const LETTER_KHO: char = 'ឃ';
    /// \u{1784}: 'ង'
    pub const LETTER_NGO: char = 'ង';
    /// \u{1785}: 'ច'
    pub const LETTER_CA: char = 'ច';
    /// \u{1786}: 'ឆ'
    pub const LETTER_CHA: char = 'ឆ';
    /// \u{1787}: 'ជ'
    pub const LETTER_CO: char = 'ជ';
    /// \u{1788}: 'ឈ'
    pub const LETTER_CHO: char = 'ឈ';
    /// \u{1789}: 'ញ'
    pub const LETTER_NYO: char = 'ញ';
    /// \u{178a}: 'ដ'
    pub const LETTER_DA: char = 'ដ';
    /// \u{178b}: 'ឋ'
    pub const LETTER_TTHA: char = 'ឋ';
    /// \u{178c}: 'ឌ'
    pub const LETTER_DO: char = 'ឌ';
    /// \u{178d}: 'ឍ'
    pub const LETTER_TTHO: char = 'ឍ';
    /// \u{178e}: 'ណ'
    pub const LETTER_NNO: char = 'ណ';
    /// \u{178f}: 'ត'
    pub const LETTER_TA: char = 'ត';
    /// \u{1790}: 'ថ'
    pub const LETTER_THA: char = 'ថ';
    /// \u{1791}: 'ទ'
    pub const LETTER_TO: char = 'ទ';
    /// \u{1792}: 'ធ'
    pub const LETTER_THO: char = 'ធ';
    /// \u{1793}: 'ន'
    pub const LETTER_NO: char = 'ន';
    /// \u{1794}: 'ប'
    pub const LETTER_BA: char = 'ប';
    /// \u{1795}: 'ផ'
    pub const LETTER_PHA: char = 'ផ';
    /// \u{1796}: 'ព'
    pub const LETTER_PO: char = 'ព';
    /// \u{1797}: 'ភ'
    pub const LETTER_PHO: char = 'ភ';
    /// \u{1798}: 'ម'
    pub const LETTER_MO: char = 'ម';
    /// \u{1799}: 'យ'
    pub const LETTER_YO: char = 'យ';
    /// \u{179a}: 'រ'
    pub const LETTER_RO: char = 'រ';
    /// \u{179b}: 'ល'
    pub const LETTER_LO: char = 'ល';
    /// \u{179c}: 'វ'
    pub const LETTER_VO: char = 'វ';
    /// \u{179d}: 'ឝ'
    pub const LETTER_SHA: char = 'ឝ';
    /// \u{179e}: 'ឞ'
    pub const LETTER_SSO: char = 'ឞ';
    /// \u{179f}: 'ស'
    pub const LETTER_SA: char = 'ស';
    /// \u{17a0}: 'ហ'
    pub const LETTER_HA: char = 'ហ';
    /// \u{17a1}: 'ឡ'
    pub const LETTER_LA: char = 'ឡ';
    /// \u{17a2}: 'អ'
    pub const LETTER_QA: char = 'អ';
    /// \u{17a3}: 'ឣ'
    pub const INDEPENDENT_VOWEL_QAQ: char = 'ឣ';
    /// \u{17a4}: 'ឤ'
    pub const INDEPENDENT_VOWEL_QAA: char = 'ឤ';
    /// \u{17a5}: 'ឥ'
    pub const INDEPENDENT_VOWEL_QI: char = 'ឥ';
    /// \u{17a6}: 'ឦ'
    pub const INDEPENDENT_VOWEL_QII: char = 'ឦ';
    /// \u{17a7}: 'ឧ'
    pub const INDEPENDENT_VOWEL_QU: char = 'ឧ';
    /// \u{17a8}: 'ឨ'
    pub const INDEPENDENT_VOWEL_QUK: char = 'ឨ';
    /// \u{17a9}: 'ឩ'
    pub const INDEPENDENT_VOWEL_QUU: char = 'ឩ';
    /// \u{17aa}: 'ឪ'
    pub const INDEPENDENT_VOWEL_QUUV: char = 'ឪ';
    /// \u{17ab}: 'ឫ'
    pub const INDEPENDENT_VOWEL_RY: char = 'ឫ';
    /// \u{17ac}: 'ឬ'
    pub const INDEPENDENT_VOWEL_RYY: char = 'ឬ';
    /// \u{17ad}: 'ឭ'
    pub const INDEPENDENT_VOWEL_LY: char = 'ឭ';
    /// \u{17ae}: 'ឮ'
    pub const INDEPENDENT_VOWEL_LYY: char = 'ឮ';
    /// \u{17af}: 'ឯ'
    pub const INDEPENDENT_VOWEL_QE: char = 'ឯ';
    /// \u{17b0}: 'ឰ'
    pub const INDEPENDENT_VOWEL_QAI: char = 'ឰ';
    /// \u{17b1}: 'ឱ'
    pub const INDEPENDENT_VOWEL_QOO_TYPE_ONE: char = 'ឱ';
    /// \u{17b2}: 'ឲ'
    pub const INDEPENDENT_VOWEL_QOO_TYPE_TWO: char = 'ឲ';
    /// \u{17b3}: 'ឳ'
    pub const INDEPENDENT_VOWEL_QAU: char = 'ឳ';
    /// \u{17b4}: '឴'
    pub const VOWEL_INHERENT_AQ: char = '឴';
    /// \u{17b5}: '឵'
    pub const VOWEL_INHERENT_AA: char = '឵';
    /// \u{17b6}: 'ា'
    pub const VOWEL_SIGN_AA: char = 'ា';
    /// \u{17b7}: 'ិ'
    pub const VOWEL_SIGN_I: char = 'ិ';
    /// \u{17b8}: 'ី'
    pub const VOWEL_SIGN_II: char = 'ី';
    /// \u{17b9}: 'ឹ'
    pub const VOWEL_SIGN_Y: char = 'ឹ';
    /// \u{17ba}: 'ឺ'
    pub const VOWEL_SIGN_YY: char = 'ឺ';
    /// \u{17bb}: 'ុ'
    pub const VOWEL_SIGN_U: char = 'ុ';
    /// \u{17bc}: 'ូ'
    pub const VOWEL_SIGN_UU: char = 'ូ';
    /// \u{17bd}: 'ួ'
    pub const VOWEL_SIGN_UA: char = 'ួ';
    /// \u{17be}: 'ើ'
    pub const VOWEL_SIGN_OE: char = 'ើ';
    /// \u{17bf}: 'ឿ'
    pub const VOWEL_SIGN_YA: char = 'ឿ';
    /// \u{17c0}: 'ៀ'
    pub const VOWEL_SIGN_IE: char = 'ៀ';
    /// \u{17c1}: 'េ'
    pub const VOWEL_SIGN_E: char = 'េ';
    /// \u{17c2}: 'ែ'
    pub const VOWEL_SIGN_AE: char = 'ែ';
    /// \u{17c3}: 'ៃ'
    pub const VOWEL_SIGN_AI: char = 'ៃ';
    /// \u{17c4}: 'ោ'
    pub const VOWEL_SIGN_OO: char = 'ោ';
    /// \u{17c5}: 'ៅ'
    pub const VOWEL_SIGN_AU: char = 'ៅ';
    /// \u{17c6}: 'ំ'
    pub const SIGN_NIKAHIT: char = 'ំ';
    /// \u{17c7}: 'ះ'
    pub const SIGN_REAHMUK: char = 'ះ';
    /// \u{17c8}: 'ៈ'
    pub const SIGN_YUUKALEAPINTU: char = 'ៈ';
    /// \u{17c9}: '៉'
    pub const SIGN_MUUSIKATOAN: char = '៉';
    /// \u{17ca}: '៊'
    pub const SIGN_TRIISAP: char = '៊';
    /// \u{17cb}: '់'
    pub const SIGN_BANTOC: char = '់';
    /// \u{17cc}: '៌'
    pub const SIGN_ROBAT: char = '៌';
    /// \u{17cd}: '៍'
    pub const SIGN_TOANDAKHIAT: char = '៍';
    /// \u{17ce}: '៎'
    pub const SIGN_KAKABAT: char = '៎';
    /// \u{17cf}: '៏'
    pub const SIGN_AHSDA: char = '៏';
    /// \u{17d0}: '័'
    pub const SIGN_SAMYOK_SANNYA: char = '័';
    /// \u{17d1}: '៑'
    pub const SIGN_VIRIAM: char = '៑';
    /// \u{17d2}: '្'
    pub const SIGN_COENG: char = '្';
    /// \u{17d3}: '៓'
    pub const SIGN_BATHAMASAT: char = '៓';
    /// \u{17d4}: '។'
    pub const SIGN_KHAN: char = '។';
    /// \u{17d5}: '៕'
    pub const SIGN_BARIYOOSAN: char = '៕';
    /// \u{17d6}: '៖'
    pub const SIGN_CAMNUC_PII_KUUH: char = '៖';
    /// \u{17d7}: 'ៗ'
    pub const SIGN_LEK_TOO: char = 'ៗ';
    /// \u{17d8}: '៘'
    pub const SIGN_BEYYAL: char = '៘';
    /// \u{17d9}: '៙'
    pub const SIGN_PHNAEK_MUAN: char = '៙';
    /// \u{17da}: '៚'
    pub const SIGN_KOOMUUT: char = '៚';
    /// \u{17db}: '៛'
    pub const CURRENCY_SYMBOL_RIEL: char = '៛';
    /// \u{17dc}: 'ៜ'
    pub const SIGN_AVAKRAHASANYA: char = 'ៜ';
    /// \u{17dd}: '៝'
    pub const SIGN_ATTHACAN: char = '៝';
    /// \u{17e0}: '០'
    pub const DIGIT_ZERO: char = '០';
    /// \u{17e1}: '១'
    pub const DIGIT_ONE: char = '១';
    /// \u{17e2}: '២'
    pub const DIGIT_TWO: char = '២';
    /// \u{17e3}: '៣'
    pub const DIGIT_THREE: char = '៣';
    /// \u{17e4}: '៤'
    pub const DIGIT_FOUR: char = '៤';
    /// \u{17e5}: '៥'
    pub const DIGIT_FIVE: char = '៥';
    /// \u{17e6}: '៦'
    pub const DIGIT_SIX: char = '៦';
    /// \u{17e7}: '៧'
    pub const DIGIT_SEVEN: char = '៧';
    /// \u{17e8}: '៨'
    pub const DIGIT_EIGHT: char = '៨';
    /// \u{17e9}: '៩'
    pub const DIGIT_NINE: char = '៩';
    /// \u{17f0}: '៰'
    pub const SYMBOL_LEK_ATTAK_SON: char = '៰';
    /// \u{17f1}: '៱'
    pub const SYMBOL_LEK_ATTAK_MUOY: char = '៱';
    /// \u{17f2}: '៲'
    pub const SYMBOL_LEK_ATTAK_PII: char = '៲';
    /// \u{17f3}: '៳'
    pub const SYMBOL_LEK_ATTAK_BEI: char = '៳';
    /// \u{17f4}: '៴'
    pub const SYMBOL_LEK_ATTAK_BUON: char = '៴';
    /// \u{17f5}: '៵'
    pub const SYMBOL_LEK_ATTAK_PRAM: char = '៵';
    /// \u{17f6}: '៶'
    pub const SYMBOL_LEK_ATTAK_PRAM_DASH_MUOY: char = '៶';
    /// \u{17f7}: '៷'
    pub const SYMBOL_LEK_ATTAK_PRAM_DASH_PII: char = '៷';
    /// \u{17f8}: '៸'
    pub const SYMBOL_LEK_ATTAK_PRAM_DASH_BEI: char = '៸';
    /// \u{17f9}: '៹'
    pub const SYMBOL_LEK_ATTAK_PRAM_DASH_BUON: char = '៹';
}

/// An enum to represent all characters in the Khmer block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Khmer {
    /// \u{1780}: 'ក'
    LetterKa,
    /// \u{1781}: 'ខ'
    LetterKha,
    /// \u{1782}: 'គ'
    LetterKo,
    /// \u{1783}: 'ឃ'
    LetterKho,
    /// \u{1784}: 'ង'
    LetterNgo,
    /// \u{1785}: 'ច'
    LetterCa,
    /// \u{1786}: 'ឆ'
    LetterCha,
    /// \u{1787}: 'ជ'
    LetterCo,
    /// \u{1788}: 'ឈ'
    LetterCho,
    /// \u{1789}: 'ញ'
    LetterNyo,
    /// \u{178a}: 'ដ'
    LetterDa,
    /// \u{178b}: 'ឋ'
    LetterTtha,
    /// \u{178c}: 'ឌ'
    LetterDo,
    /// \u{178d}: 'ឍ'
    LetterTtho,
    /// \u{178e}: 'ណ'
    LetterNno,
    /// \u{178f}: 'ត'
    LetterTa,
    /// \u{1790}: 'ថ'
    LetterTha,
    /// \u{1791}: 'ទ'
    LetterTo,
    /// \u{1792}: 'ធ'
    LetterTho,
    /// \u{1793}: 'ន'
    LetterNo,
    /// \u{1794}: 'ប'
    LetterBa,
    /// \u{1795}: 'ផ'
    LetterPha,
    /// \u{1796}: 'ព'
    LetterPo,
    /// \u{1797}: 'ភ'
    LetterPho,
    /// \u{1798}: 'ម'
    LetterMo,
    /// \u{1799}: 'យ'
    LetterYo,
    /// \u{179a}: 'រ'
    LetterRo,
    /// \u{179b}: 'ល'
    LetterLo,
    /// \u{179c}: 'វ'
    LetterVo,
    /// \u{179d}: 'ឝ'
    LetterSha,
    /// \u{179e}: 'ឞ'
    LetterSso,
    /// \u{179f}: 'ស'
    LetterSa,
    /// \u{17a0}: 'ហ'
    LetterHa,
    /// \u{17a1}: 'ឡ'
    LetterLa,
    /// \u{17a2}: 'អ'
    LetterQa,
    /// \u{17a3}: 'ឣ'
    IndependentVowelQaq,
    /// \u{17a4}: 'ឤ'
    IndependentVowelQaa,
    /// \u{17a5}: 'ឥ'
    IndependentVowelQi,
    /// \u{17a6}: 'ឦ'
    IndependentVowelQii,
    /// \u{17a7}: 'ឧ'
    IndependentVowelQu,
    /// \u{17a8}: 'ឨ'
    IndependentVowelQuk,
    /// \u{17a9}: 'ឩ'
    IndependentVowelQuu,
    /// \u{17aa}: 'ឪ'
    IndependentVowelQuuv,
    /// \u{17ab}: 'ឫ'
    IndependentVowelRy,
    /// \u{17ac}: 'ឬ'
    IndependentVowelRyy,
    /// \u{17ad}: 'ឭ'
    IndependentVowelLy,
    /// \u{17ae}: 'ឮ'
    IndependentVowelLyy,
    /// \u{17af}: 'ឯ'
    IndependentVowelQe,
    /// \u{17b0}: 'ឰ'
    IndependentVowelQai,
    /// \u{17b1}: 'ឱ'
    IndependentVowelQooTypeOne,
    /// \u{17b2}: 'ឲ'
    IndependentVowelQooTypeTwo,
    /// \u{17b3}: 'ឳ'
    IndependentVowelQau,
    /// \u{17b4}: '឴'
    VowelInherentAq,
    /// \u{17b5}: '឵'
    VowelInherentAa,
    /// \u{17b6}: 'ា'
    VowelSignAa,
    /// \u{17b7}: 'ិ'
    VowelSignI,
    /// \u{17b8}: 'ី'
    VowelSignIi,
    /// \u{17b9}: 'ឹ'
    VowelSignY,
    /// \u{17ba}: 'ឺ'
    VowelSignYy,
    /// \u{17bb}: 'ុ'
    VowelSignU,
    /// \u{17bc}: 'ូ'
    VowelSignUu,
    /// \u{17bd}: 'ួ'
    VowelSignUa,
    /// \u{17be}: 'ើ'
    VowelSignOe,
    /// \u{17bf}: 'ឿ'
    VowelSignYa,
    /// \u{17c0}: 'ៀ'
    VowelSignIe,
    /// \u{17c1}: 'េ'
    VowelSignE,
    /// \u{17c2}: 'ែ'
    VowelSignAe,
    /// \u{17c3}: 'ៃ'
    VowelSignAi,
    /// \u{17c4}: 'ោ'
    VowelSignOo,
    /// \u{17c5}: 'ៅ'
    VowelSignAu,
    /// \u{17c6}: 'ំ'
    SignNikahit,
    /// \u{17c7}: 'ះ'
    SignReahmuk,
    /// \u{17c8}: 'ៈ'
    SignYuukaleapintu,
    /// \u{17c9}: '៉'
    SignMuusikatoan,
    /// \u{17ca}: '៊'
    SignTriisap,
    /// \u{17cb}: '់'
    SignBantoc,
    /// \u{17cc}: '៌'
    SignRobat,
    /// \u{17cd}: '៍'
    SignToandakhiat,
    /// \u{17ce}: '៎'
    SignKakabat,
    /// \u{17cf}: '៏'
    SignAhsda,
    /// \u{17d0}: '័'
    SignSamyokSannya,
    /// \u{17d1}: '៑'
    SignViriam,
    /// \u{17d2}: '្'
    SignCoeng,
    /// \u{17d3}: '៓'
    SignBathamasat,
    /// \u{17d4}: '។'
    SignKhan,
    /// \u{17d5}: '៕'
    SignBariyoosan,
    /// \u{17d6}: '៖'
    SignCamnucPiiKuuh,
    /// \u{17d7}: 'ៗ'
    SignLekToo,
    /// \u{17d8}: '៘'
    SignBeyyal,
    /// \u{17d9}: '៙'
    SignPhnaekMuan,
    /// \u{17da}: '៚'
    SignKoomuut,
    /// \u{17db}: '៛'
    CurrencySymbolRiel,
    /// \u{17dc}: 'ៜ'
    SignAvakrahasanya,
    /// \u{17dd}: '៝'
    SignAtthacan,
    /// \u{17e0}: '០'
    DigitZero,
    /// \u{17e1}: '១'
    DigitOne,
    /// \u{17e2}: '២'
    DigitTwo,
    /// \u{17e3}: '៣'
    DigitThree,
    /// \u{17e4}: '៤'
    DigitFour,
    /// \u{17e5}: '៥'
    DigitFive,
    /// \u{17e6}: '៦'
    DigitSix,
    /// \u{17e7}: '៧'
    DigitSeven,
    /// \u{17e8}: '៨'
    DigitEight,
    /// \u{17e9}: '៩'
    DigitNine,
    /// \u{17f0}: '៰'
    SymbolLekAttakSon,
    /// \u{17f1}: '៱'
    SymbolLekAttakMuoy,
    /// \u{17f2}: '៲'
    SymbolLekAttakPii,
    /// \u{17f3}: '៳'
    SymbolLekAttakBei,
    /// \u{17f4}: '៴'
    SymbolLekAttakBuon,
    /// \u{17f5}: '៵'
    SymbolLekAttakPram,
    /// \u{17f6}: '៶'
    SymbolLekAttakPramDashMuoy,
    /// \u{17f7}: '៷'
    SymbolLekAttakPramDashPii,
    /// \u{17f8}: '៸'
    SymbolLekAttakPramDashBei,
    /// \u{17f9}: '៹'
    SymbolLekAttakPramDashBuon,
}

impl Into<char> for Khmer {
    fn into(self) -> char {
        use constants::*;
        match self {
            Khmer::LetterKa => LETTER_KA,
            Khmer::LetterKha => LETTER_KHA,
            Khmer::LetterKo => LETTER_KO,
            Khmer::LetterKho => LETTER_KHO,
            Khmer::LetterNgo => LETTER_NGO,
            Khmer::LetterCa => LETTER_CA,
            Khmer::LetterCha => LETTER_CHA,
            Khmer::LetterCo => LETTER_CO,
            Khmer::LetterCho => LETTER_CHO,
            Khmer::LetterNyo => LETTER_NYO,
            Khmer::LetterDa => LETTER_DA,
            Khmer::LetterTtha => LETTER_TTHA,
            Khmer::LetterDo => LETTER_DO,
            Khmer::LetterTtho => LETTER_TTHO,
            Khmer::LetterNno => LETTER_NNO,
            Khmer::LetterTa => LETTER_TA,
            Khmer::LetterTha => LETTER_THA,
            Khmer::LetterTo => LETTER_TO,
            Khmer::LetterTho => LETTER_THO,
            Khmer::LetterNo => LETTER_NO,
            Khmer::LetterBa => LETTER_BA,
            Khmer::LetterPha => LETTER_PHA,
            Khmer::LetterPo => LETTER_PO,
            Khmer::LetterPho => LETTER_PHO,
            Khmer::LetterMo => LETTER_MO,
            Khmer::LetterYo => LETTER_YO,
            Khmer::LetterRo => LETTER_RO,
            Khmer::LetterLo => LETTER_LO,
            Khmer::LetterVo => LETTER_VO,
            Khmer::LetterSha => LETTER_SHA,
            Khmer::LetterSso => LETTER_SSO,
            Khmer::LetterSa => LETTER_SA,
            Khmer::LetterHa => LETTER_HA,
            Khmer::LetterLa => LETTER_LA,
            Khmer::LetterQa => LETTER_QA,
            Khmer::IndependentVowelQaq => INDEPENDENT_VOWEL_QAQ,
            Khmer::IndependentVowelQaa => INDEPENDENT_VOWEL_QAA,
            Khmer::IndependentVowelQi => INDEPENDENT_VOWEL_QI,
            Khmer::IndependentVowelQii => INDEPENDENT_VOWEL_QII,
            Khmer::IndependentVowelQu => INDEPENDENT_VOWEL_QU,
            Khmer::IndependentVowelQuk => INDEPENDENT_VOWEL_QUK,
            Khmer::IndependentVowelQuu => INDEPENDENT_VOWEL_QUU,
            Khmer::IndependentVowelQuuv => INDEPENDENT_VOWEL_QUUV,
            Khmer::IndependentVowelRy => INDEPENDENT_VOWEL_RY,
            Khmer::IndependentVowelRyy => INDEPENDENT_VOWEL_RYY,
            Khmer::IndependentVowelLy => INDEPENDENT_VOWEL_LY,
            Khmer::IndependentVowelLyy => INDEPENDENT_VOWEL_LYY,
            Khmer::IndependentVowelQe => INDEPENDENT_VOWEL_QE,
            Khmer::IndependentVowelQai => INDEPENDENT_VOWEL_QAI,
            Khmer::IndependentVowelQooTypeOne => INDEPENDENT_VOWEL_QOO_TYPE_ONE,
            Khmer::IndependentVowelQooTypeTwo => INDEPENDENT_VOWEL_QOO_TYPE_TWO,
            Khmer::IndependentVowelQau => INDEPENDENT_VOWEL_QAU,
            Khmer::VowelInherentAq => VOWEL_INHERENT_AQ,
            Khmer::VowelInherentAa => VOWEL_INHERENT_AA,
            Khmer::VowelSignAa => VOWEL_SIGN_AA,
            Khmer::VowelSignI => VOWEL_SIGN_I,
            Khmer::VowelSignIi => VOWEL_SIGN_II,
            Khmer::VowelSignY => VOWEL_SIGN_Y,
            Khmer::VowelSignYy => VOWEL_SIGN_YY,
            Khmer::VowelSignU => VOWEL_SIGN_U,
            Khmer::VowelSignUu => VOWEL_SIGN_UU,
            Khmer::VowelSignUa => VOWEL_SIGN_UA,
            Khmer::VowelSignOe => VOWEL_SIGN_OE,
            Khmer::VowelSignYa => VOWEL_SIGN_YA,
            Khmer::VowelSignIe => VOWEL_SIGN_IE,
            Khmer::VowelSignE => VOWEL_SIGN_E,
            Khmer::VowelSignAe => VOWEL_SIGN_AE,
            Khmer::VowelSignAi => VOWEL_SIGN_AI,
            Khmer::VowelSignOo => VOWEL_SIGN_OO,
            Khmer::VowelSignAu => VOWEL_SIGN_AU,
            Khmer::SignNikahit => SIGN_NIKAHIT,
            Khmer::SignReahmuk => SIGN_REAHMUK,
            Khmer::SignYuukaleapintu => SIGN_YUUKALEAPINTU,
            Khmer::SignMuusikatoan => SIGN_MUUSIKATOAN,
            Khmer::SignTriisap => SIGN_TRIISAP,
            Khmer::SignBantoc => SIGN_BANTOC,
            Khmer::SignRobat => SIGN_ROBAT,
            Khmer::SignToandakhiat => SIGN_TOANDAKHIAT,
            Khmer::SignKakabat => SIGN_KAKABAT,
            Khmer::SignAhsda => SIGN_AHSDA,
            Khmer::SignSamyokSannya => SIGN_SAMYOK_SANNYA,
            Khmer::SignViriam => SIGN_VIRIAM,
            Khmer::SignCoeng => SIGN_COENG,
            Khmer::SignBathamasat => SIGN_BATHAMASAT,
            Khmer::SignKhan => SIGN_KHAN,
            Khmer::SignBariyoosan => SIGN_BARIYOOSAN,
            Khmer::SignCamnucPiiKuuh => SIGN_CAMNUC_PII_KUUH,
            Khmer::SignLekToo => SIGN_LEK_TOO,
            Khmer::SignBeyyal => SIGN_BEYYAL,
            Khmer::SignPhnaekMuan => SIGN_PHNAEK_MUAN,
            Khmer::SignKoomuut => SIGN_KOOMUUT,
            Khmer::CurrencySymbolRiel => CURRENCY_SYMBOL_RIEL,
            Khmer::SignAvakrahasanya => SIGN_AVAKRAHASANYA,
            Khmer::SignAtthacan => SIGN_ATTHACAN,
            Khmer::DigitZero => DIGIT_ZERO,
            Khmer::DigitOne => DIGIT_ONE,
            Khmer::DigitTwo => DIGIT_TWO,
            Khmer::DigitThree => DIGIT_THREE,
            Khmer::DigitFour => DIGIT_FOUR,
            Khmer::DigitFive => DIGIT_FIVE,
            Khmer::DigitSix => DIGIT_SIX,
            Khmer::DigitSeven => DIGIT_SEVEN,
            Khmer::DigitEight => DIGIT_EIGHT,
            Khmer::DigitNine => DIGIT_NINE,
            Khmer::SymbolLekAttakSon => SYMBOL_LEK_ATTAK_SON,
            Khmer::SymbolLekAttakMuoy => SYMBOL_LEK_ATTAK_MUOY,
            Khmer::SymbolLekAttakPii => SYMBOL_LEK_ATTAK_PII,
            Khmer::SymbolLekAttakBei => SYMBOL_LEK_ATTAK_BEI,
            Khmer::SymbolLekAttakBuon => SYMBOL_LEK_ATTAK_BUON,
            Khmer::SymbolLekAttakPram => SYMBOL_LEK_ATTAK_PRAM,
            Khmer::SymbolLekAttakPramDashMuoy => SYMBOL_LEK_ATTAK_PRAM_DASH_MUOY,
            Khmer::SymbolLekAttakPramDashPii => SYMBOL_LEK_ATTAK_PRAM_DASH_PII,
            Khmer::SymbolLekAttakPramDashBei => SYMBOL_LEK_ATTAK_PRAM_DASH_BEI,
            Khmer::SymbolLekAttakPramDashBuon => SYMBOL_LEK_ATTAK_PRAM_DASH_BUON,
        }
    }
}

impl std::convert::TryFrom<char> for Khmer {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_KA => Ok(Khmer::LetterKa),
            LETTER_KHA => Ok(Khmer::LetterKha),
            LETTER_KO => Ok(Khmer::LetterKo),
            LETTER_KHO => Ok(Khmer::LetterKho),
            LETTER_NGO => Ok(Khmer::LetterNgo),
            LETTER_CA => Ok(Khmer::LetterCa),
            LETTER_CHA => Ok(Khmer::LetterCha),
            LETTER_CO => Ok(Khmer::LetterCo),
            LETTER_CHO => Ok(Khmer::LetterCho),
            LETTER_NYO => Ok(Khmer::LetterNyo),
            LETTER_DA => Ok(Khmer::LetterDa),
            LETTER_TTHA => Ok(Khmer::LetterTtha),
            LETTER_DO => Ok(Khmer::LetterDo),
            LETTER_TTHO => Ok(Khmer::LetterTtho),
            LETTER_NNO => Ok(Khmer::LetterNno),
            LETTER_TA => Ok(Khmer::LetterTa),
            LETTER_THA => Ok(Khmer::LetterTha),
            LETTER_TO => Ok(Khmer::LetterTo),
            LETTER_THO => Ok(Khmer::LetterTho),
            LETTER_NO => Ok(Khmer::LetterNo),
            LETTER_BA => Ok(Khmer::LetterBa),
            LETTER_PHA => Ok(Khmer::LetterPha),
            LETTER_PO => Ok(Khmer::LetterPo),
            LETTER_PHO => Ok(Khmer::LetterPho),
            LETTER_MO => Ok(Khmer::LetterMo),
            LETTER_YO => Ok(Khmer::LetterYo),
            LETTER_RO => Ok(Khmer::LetterRo),
            LETTER_LO => Ok(Khmer::LetterLo),
            LETTER_VO => Ok(Khmer::LetterVo),
            LETTER_SHA => Ok(Khmer::LetterSha),
            LETTER_SSO => Ok(Khmer::LetterSso),
            LETTER_SA => Ok(Khmer::LetterSa),
            LETTER_HA => Ok(Khmer::LetterHa),
            LETTER_LA => Ok(Khmer::LetterLa),
            LETTER_QA => Ok(Khmer::LetterQa),
            INDEPENDENT_VOWEL_QAQ => Ok(Khmer::IndependentVowelQaq),
            INDEPENDENT_VOWEL_QAA => Ok(Khmer::IndependentVowelQaa),
            INDEPENDENT_VOWEL_QI => Ok(Khmer::IndependentVowelQi),
            INDEPENDENT_VOWEL_QII => Ok(Khmer::IndependentVowelQii),
            INDEPENDENT_VOWEL_QU => Ok(Khmer::IndependentVowelQu),
            INDEPENDENT_VOWEL_QUK => Ok(Khmer::IndependentVowelQuk),
            INDEPENDENT_VOWEL_QUU => Ok(Khmer::IndependentVowelQuu),
            INDEPENDENT_VOWEL_QUUV => Ok(Khmer::IndependentVowelQuuv),
            INDEPENDENT_VOWEL_RY => Ok(Khmer::IndependentVowelRy),
            INDEPENDENT_VOWEL_RYY => Ok(Khmer::IndependentVowelRyy),
            INDEPENDENT_VOWEL_LY => Ok(Khmer::IndependentVowelLy),
            INDEPENDENT_VOWEL_LYY => Ok(Khmer::IndependentVowelLyy),
            INDEPENDENT_VOWEL_QE => Ok(Khmer::IndependentVowelQe),
            INDEPENDENT_VOWEL_QAI => Ok(Khmer::IndependentVowelQai),
            INDEPENDENT_VOWEL_QOO_TYPE_ONE => Ok(Khmer::IndependentVowelQooTypeOne),
            INDEPENDENT_VOWEL_QOO_TYPE_TWO => Ok(Khmer::IndependentVowelQooTypeTwo),
            INDEPENDENT_VOWEL_QAU => Ok(Khmer::IndependentVowelQau),
            VOWEL_INHERENT_AQ => Ok(Khmer::VowelInherentAq),
            VOWEL_INHERENT_AA => Ok(Khmer::VowelInherentAa),
            VOWEL_SIGN_AA => Ok(Khmer::VowelSignAa),
            VOWEL_SIGN_I => Ok(Khmer::VowelSignI),
            VOWEL_SIGN_II => Ok(Khmer::VowelSignIi),
            VOWEL_SIGN_Y => Ok(Khmer::VowelSignY),
            VOWEL_SIGN_YY => Ok(Khmer::VowelSignYy),
            VOWEL_SIGN_U => Ok(Khmer::VowelSignU),
            VOWEL_SIGN_UU => Ok(Khmer::VowelSignUu),
            VOWEL_SIGN_UA => Ok(Khmer::VowelSignUa),
            VOWEL_SIGN_OE => Ok(Khmer::VowelSignOe),
            VOWEL_SIGN_YA => Ok(Khmer::VowelSignYa),
            VOWEL_SIGN_IE => Ok(Khmer::VowelSignIe),
            VOWEL_SIGN_E => Ok(Khmer::VowelSignE),
            VOWEL_SIGN_AE => Ok(Khmer::VowelSignAe),
            VOWEL_SIGN_AI => Ok(Khmer::VowelSignAi),
            VOWEL_SIGN_OO => Ok(Khmer::VowelSignOo),
            VOWEL_SIGN_AU => Ok(Khmer::VowelSignAu),
            SIGN_NIKAHIT => Ok(Khmer::SignNikahit),
            SIGN_REAHMUK => Ok(Khmer::SignReahmuk),
            SIGN_YUUKALEAPINTU => Ok(Khmer::SignYuukaleapintu),
            SIGN_MUUSIKATOAN => Ok(Khmer::SignMuusikatoan),
            SIGN_TRIISAP => Ok(Khmer::SignTriisap),
            SIGN_BANTOC => Ok(Khmer::SignBantoc),
            SIGN_ROBAT => Ok(Khmer::SignRobat),
            SIGN_TOANDAKHIAT => Ok(Khmer::SignToandakhiat),
            SIGN_KAKABAT => Ok(Khmer::SignKakabat),
            SIGN_AHSDA => Ok(Khmer::SignAhsda),
            SIGN_SAMYOK_SANNYA => Ok(Khmer::SignSamyokSannya),
            SIGN_VIRIAM => Ok(Khmer::SignViriam),
            SIGN_COENG => Ok(Khmer::SignCoeng),
            SIGN_BATHAMASAT => Ok(Khmer::SignBathamasat),
            SIGN_KHAN => Ok(Khmer::SignKhan),
            SIGN_BARIYOOSAN => Ok(Khmer::SignBariyoosan),
            SIGN_CAMNUC_PII_KUUH => Ok(Khmer::SignCamnucPiiKuuh),
            SIGN_LEK_TOO => Ok(Khmer::SignLekToo),
            SIGN_BEYYAL => Ok(Khmer::SignBeyyal),
            SIGN_PHNAEK_MUAN => Ok(Khmer::SignPhnaekMuan),
            SIGN_KOOMUUT => Ok(Khmer::SignKoomuut),
            CURRENCY_SYMBOL_RIEL => Ok(Khmer::CurrencySymbolRiel),
            SIGN_AVAKRAHASANYA => Ok(Khmer::SignAvakrahasanya),
            SIGN_ATTHACAN => Ok(Khmer::SignAtthacan),
            DIGIT_ZERO => Ok(Khmer::DigitZero),
            DIGIT_ONE => Ok(Khmer::DigitOne),
            DIGIT_TWO => Ok(Khmer::DigitTwo),
            DIGIT_THREE => Ok(Khmer::DigitThree),
            DIGIT_FOUR => Ok(Khmer::DigitFour),
            DIGIT_FIVE => Ok(Khmer::DigitFive),
            DIGIT_SIX => Ok(Khmer::DigitSix),
            DIGIT_SEVEN => Ok(Khmer::DigitSeven),
            DIGIT_EIGHT => Ok(Khmer::DigitEight),
            DIGIT_NINE => Ok(Khmer::DigitNine),
            SYMBOL_LEK_ATTAK_SON => Ok(Khmer::SymbolLekAttakSon),
            SYMBOL_LEK_ATTAK_MUOY => Ok(Khmer::SymbolLekAttakMuoy),
            SYMBOL_LEK_ATTAK_PII => Ok(Khmer::SymbolLekAttakPii),
            SYMBOL_LEK_ATTAK_BEI => Ok(Khmer::SymbolLekAttakBei),
            SYMBOL_LEK_ATTAK_BUON => Ok(Khmer::SymbolLekAttakBuon),
            SYMBOL_LEK_ATTAK_PRAM => Ok(Khmer::SymbolLekAttakPram),
            SYMBOL_LEK_ATTAK_PRAM_DASH_MUOY => Ok(Khmer::SymbolLekAttakPramDashMuoy),
            SYMBOL_LEK_ATTAK_PRAM_DASH_PII => Ok(Khmer::SymbolLekAttakPramDashPii),
            SYMBOL_LEK_ATTAK_PRAM_DASH_BEI => Ok(Khmer::SymbolLekAttakPramDashBei),
            SYMBOL_LEK_ATTAK_PRAM_DASH_BUON => Ok(Khmer::SymbolLekAttakPramDashBuon),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Khmer {
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

impl std::convert::TryFrom<u32> for Khmer {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Khmer {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Khmer {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Khmer::LetterKa
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Khmer::LetterKa => "khmer letter ka",
            Khmer::LetterKha => "khmer letter kha",
            Khmer::LetterKo => "khmer letter ko",
            Khmer::LetterKho => "khmer letter kho",
            Khmer::LetterNgo => "khmer letter ngo",
            Khmer::LetterCa => "khmer letter ca",
            Khmer::LetterCha => "khmer letter cha",
            Khmer::LetterCo => "khmer letter co",
            Khmer::LetterCho => "khmer letter cho",
            Khmer::LetterNyo => "khmer letter nyo",
            Khmer::LetterDa => "khmer letter da",
            Khmer::LetterTtha => "khmer letter ttha",
            Khmer::LetterDo => "khmer letter do",
            Khmer::LetterTtho => "khmer letter ttho",
            Khmer::LetterNno => "khmer letter nno",
            Khmer::LetterTa => "khmer letter ta",
            Khmer::LetterTha => "khmer letter tha",
            Khmer::LetterTo => "khmer letter to",
            Khmer::LetterTho => "khmer letter tho",
            Khmer::LetterNo => "khmer letter no",
            Khmer::LetterBa => "khmer letter ba",
            Khmer::LetterPha => "khmer letter pha",
            Khmer::LetterPo => "khmer letter po",
            Khmer::LetterPho => "khmer letter pho",
            Khmer::LetterMo => "khmer letter mo",
            Khmer::LetterYo => "khmer letter yo",
            Khmer::LetterRo => "khmer letter ro",
            Khmer::LetterLo => "khmer letter lo",
            Khmer::LetterVo => "khmer letter vo",
            Khmer::LetterSha => "khmer letter sha",
            Khmer::LetterSso => "khmer letter sso",
            Khmer::LetterSa => "khmer letter sa",
            Khmer::LetterHa => "khmer letter ha",
            Khmer::LetterLa => "khmer letter la",
            Khmer::LetterQa => "khmer letter qa",
            Khmer::IndependentVowelQaq => "khmer independent vowel qaq",
            Khmer::IndependentVowelQaa => "khmer independent vowel qaa",
            Khmer::IndependentVowelQi => "khmer independent vowel qi",
            Khmer::IndependentVowelQii => "khmer independent vowel qii",
            Khmer::IndependentVowelQu => "khmer independent vowel qu",
            Khmer::IndependentVowelQuk => "khmer independent vowel quk",
            Khmer::IndependentVowelQuu => "khmer independent vowel quu",
            Khmer::IndependentVowelQuuv => "khmer independent vowel quuv",
            Khmer::IndependentVowelRy => "khmer independent vowel ry",
            Khmer::IndependentVowelRyy => "khmer independent vowel ryy",
            Khmer::IndependentVowelLy => "khmer independent vowel ly",
            Khmer::IndependentVowelLyy => "khmer independent vowel lyy",
            Khmer::IndependentVowelQe => "khmer independent vowel qe",
            Khmer::IndependentVowelQai => "khmer independent vowel qai",
            Khmer::IndependentVowelQooTypeOne => "khmer independent vowel qoo type one",
            Khmer::IndependentVowelQooTypeTwo => "khmer independent vowel qoo type two",
            Khmer::IndependentVowelQau => "khmer independent vowel qau",
            Khmer::VowelInherentAq => "khmer vowel inherent aq",
            Khmer::VowelInherentAa => "khmer vowel inherent aa",
            Khmer::VowelSignAa => "khmer vowel sign aa",
            Khmer::VowelSignI => "khmer vowel sign i",
            Khmer::VowelSignIi => "khmer vowel sign ii",
            Khmer::VowelSignY => "khmer vowel sign y",
            Khmer::VowelSignYy => "khmer vowel sign yy",
            Khmer::VowelSignU => "khmer vowel sign u",
            Khmer::VowelSignUu => "khmer vowel sign uu",
            Khmer::VowelSignUa => "khmer vowel sign ua",
            Khmer::VowelSignOe => "khmer vowel sign oe",
            Khmer::VowelSignYa => "khmer vowel sign ya",
            Khmer::VowelSignIe => "khmer vowel sign ie",
            Khmer::VowelSignE => "khmer vowel sign e",
            Khmer::VowelSignAe => "khmer vowel sign ae",
            Khmer::VowelSignAi => "khmer vowel sign ai",
            Khmer::VowelSignOo => "khmer vowel sign oo",
            Khmer::VowelSignAu => "khmer vowel sign au",
            Khmer::SignNikahit => "khmer sign nikahit",
            Khmer::SignReahmuk => "khmer sign reahmuk",
            Khmer::SignYuukaleapintu => "khmer sign yuukaleapintu",
            Khmer::SignMuusikatoan => "khmer sign muusikatoan",
            Khmer::SignTriisap => "khmer sign triisap",
            Khmer::SignBantoc => "khmer sign bantoc",
            Khmer::SignRobat => "khmer sign robat",
            Khmer::SignToandakhiat => "khmer sign toandakhiat",
            Khmer::SignKakabat => "khmer sign kakabat",
            Khmer::SignAhsda => "khmer sign ahsda",
            Khmer::SignSamyokSannya => "khmer sign samyok sannya",
            Khmer::SignViriam => "khmer sign viriam",
            Khmer::SignCoeng => "khmer sign coeng",
            Khmer::SignBathamasat => "khmer sign bathamasat",
            Khmer::SignKhan => "khmer sign khan",
            Khmer::SignBariyoosan => "khmer sign bariyoosan",
            Khmer::SignCamnucPiiKuuh => "khmer sign camnuc pii kuuh",
            Khmer::SignLekToo => "khmer sign lek too",
            Khmer::SignBeyyal => "khmer sign beyyal",
            Khmer::SignPhnaekMuan => "khmer sign phnaek muan",
            Khmer::SignKoomuut => "khmer sign koomuut",
            Khmer::CurrencySymbolRiel => "khmer currency symbol riel",
            Khmer::SignAvakrahasanya => "khmer sign avakrahasanya",
            Khmer::SignAtthacan => "khmer sign atthacan",
            Khmer::DigitZero => "khmer digit zero",
            Khmer::DigitOne => "khmer digit one",
            Khmer::DigitTwo => "khmer digit two",
            Khmer::DigitThree => "khmer digit three",
            Khmer::DigitFour => "khmer digit four",
            Khmer::DigitFive => "khmer digit five",
            Khmer::DigitSix => "khmer digit six",
            Khmer::DigitSeven => "khmer digit seven",
            Khmer::DigitEight => "khmer digit eight",
            Khmer::DigitNine => "khmer digit nine",
            Khmer::SymbolLekAttakSon => "khmer symbol lek attak son",
            Khmer::SymbolLekAttakMuoy => "khmer symbol lek attak muoy",
            Khmer::SymbolLekAttakPii => "khmer symbol lek attak pii",
            Khmer::SymbolLekAttakBei => "khmer symbol lek attak bei",
            Khmer::SymbolLekAttakBuon => "khmer symbol lek attak buon",
            Khmer::SymbolLekAttakPram => "khmer symbol lek attak pram",
            Khmer::SymbolLekAttakPramDashMuoy => "khmer symbol lek attak pram-muoy",
            Khmer::SymbolLekAttakPramDashPii => "khmer symbol lek attak pram-pii",
            Khmer::SymbolLekAttakPramDashBei => "khmer symbol lek attak pram-bei",
            Khmer::SymbolLekAttakPramDashBuon => "khmer symbol lek attak pram-buon",
        }
    }
}
