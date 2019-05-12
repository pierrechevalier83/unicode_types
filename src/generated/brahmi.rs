/// A number of constants to give a name to all characters in this block.
mod constants {
    /// \u{11000}: '𑀀'
    pub const SIGN_CANDRABINDU: char = '𑀀';
    /// \u{11001}: '𑀁'
    pub const SIGN_ANUSVARA: char = '𑀁';
    /// \u{11002}: '𑀂'
    pub const SIGN_VISARGA: char = '𑀂';
    /// \u{11003}: '𑀃'
    pub const SIGN_JIHVAMULIYA: char = '𑀃';
    /// \u{11004}: '𑀄'
    pub const SIGN_UPADHMANIYA: char = '𑀄';
    /// \u{11005}: '𑀅'
    pub const LETTER_A: char = '𑀅';
    /// \u{11006}: '𑀆'
    pub const LETTER_AA: char = '𑀆';
    /// \u{11007}: '𑀇'
    pub const LETTER_I: char = '𑀇';
    /// \u{11008}: '𑀈'
    pub const LETTER_II: char = '𑀈';
    /// \u{11009}: '𑀉'
    pub const LETTER_U: char = '𑀉';
    /// \u{1100a}: '𑀊'
    pub const LETTER_UU: char = '𑀊';
    /// \u{1100b}: '𑀋'
    pub const LETTER_VOCALIC_R: char = '𑀋';
    /// \u{1100c}: '𑀌'
    pub const LETTER_VOCALIC_RR: char = '𑀌';
    /// \u{1100d}: '𑀍'
    pub const LETTER_VOCALIC_L: char = '𑀍';
    /// \u{1100e}: '𑀎'
    pub const LETTER_VOCALIC_LL: char = '𑀎';
    /// \u{1100f}: '𑀏'
    pub const LETTER_E: char = '𑀏';
    /// \u{11010}: '𑀐'
    pub const LETTER_AI: char = '𑀐';
    /// \u{11011}: '𑀑'
    pub const LETTER_O: char = '𑀑';
    /// \u{11012}: '𑀒'
    pub const LETTER_AU: char = '𑀒';
    /// \u{11013}: '𑀓'
    pub const LETTER_KA: char = '𑀓';
    /// \u{11014}: '𑀔'
    pub const LETTER_KHA: char = '𑀔';
    /// \u{11015}: '𑀕'
    pub const LETTER_GA: char = '𑀕';
    /// \u{11016}: '𑀖'
    pub const LETTER_GHA: char = '𑀖';
    /// \u{11017}: '𑀗'
    pub const LETTER_NGA: char = '𑀗';
    /// \u{11018}: '𑀘'
    pub const LETTER_CA: char = '𑀘';
    /// \u{11019}: '𑀙'
    pub const LETTER_CHA: char = '𑀙';
    /// \u{1101a}: '𑀚'
    pub const LETTER_JA: char = '𑀚';
    /// \u{1101b}: '𑀛'
    pub const LETTER_JHA: char = '𑀛';
    /// \u{1101c}: '𑀜'
    pub const LETTER_NYA: char = '𑀜';
    /// \u{1101d}: '𑀝'
    pub const LETTER_TTA: char = '𑀝';
    /// \u{1101e}: '𑀞'
    pub const LETTER_TTHA: char = '𑀞';
    /// \u{1101f}: '𑀟'
    pub const LETTER_DDA: char = '𑀟';
    /// \u{11020}: '𑀠'
    pub const LETTER_DDHA: char = '𑀠';
    /// \u{11021}: '𑀡'
    pub const LETTER_NNA: char = '𑀡';
    /// \u{11022}: '𑀢'
    pub const LETTER_TA: char = '𑀢';
    /// \u{11023}: '𑀣'
    pub const LETTER_THA: char = '𑀣';
    /// \u{11024}: '𑀤'
    pub const LETTER_DA: char = '𑀤';
    /// \u{11025}: '𑀥'
    pub const LETTER_DHA: char = '𑀥';
    /// \u{11026}: '𑀦'
    pub const LETTER_NA: char = '𑀦';
    /// \u{11027}: '𑀧'
    pub const LETTER_PA: char = '𑀧';
    /// \u{11028}: '𑀨'
    pub const LETTER_PHA: char = '𑀨';
    /// \u{11029}: '𑀩'
    pub const LETTER_BA: char = '𑀩';
    /// \u{1102a}: '𑀪'
    pub const LETTER_BHA: char = '𑀪';
    /// \u{1102b}: '𑀫'
    pub const LETTER_MA: char = '𑀫';
    /// \u{1102c}: '𑀬'
    pub const LETTER_YA: char = '𑀬';
    /// \u{1102d}: '𑀭'
    pub const LETTER_RA: char = '𑀭';
    /// \u{1102e}: '𑀮'
    pub const LETTER_LA: char = '𑀮';
    /// \u{1102f}: '𑀯'
    pub const LETTER_VA: char = '𑀯';
    /// \u{11030}: '𑀰'
    pub const LETTER_SHA: char = '𑀰';
    /// \u{11031}: '𑀱'
    pub const LETTER_SSA: char = '𑀱';
    /// \u{11032}: '𑀲'
    pub const LETTER_SA: char = '𑀲';
    /// \u{11033}: '𑀳'
    pub const LETTER_HA: char = '𑀳';
    /// \u{11034}: '𑀴'
    pub const LETTER_LLA: char = '𑀴';
    /// \u{11035}: '𑀵'
    pub const LETTER_OLD_TAMIL_LLLA: char = '𑀵';
    /// \u{11036}: '𑀶'
    pub const LETTER_OLD_TAMIL_RRA: char = '𑀶';
    /// \u{11037}: '𑀷'
    pub const LETTER_OLD_TAMIL_NNNA: char = '𑀷';
    /// \u{11038}: '𑀸'
    pub const VOWEL_SIGN_AA: char = '𑀸';
    /// \u{11039}: '𑀹'
    pub const VOWEL_SIGN_BHATTIPROLU_AA: char = '𑀹';
    /// \u{1103a}: '𑀺'
    pub const VOWEL_SIGN_I: char = '𑀺';
    /// \u{1103b}: '𑀻'
    pub const VOWEL_SIGN_II: char = '𑀻';
    /// \u{1103c}: '𑀼'
    pub const VOWEL_SIGN_U: char = '𑀼';
    /// \u{1103d}: '𑀽'
    pub const VOWEL_SIGN_UU: char = '𑀽';
    /// \u{1103e}: '𑀾'
    pub const VOWEL_SIGN_VOCALIC_R: char = '𑀾';
    /// \u{1103f}: '𑀿'
    pub const VOWEL_SIGN_VOCALIC_RR: char = '𑀿';
    /// \u{11040}: '𑁀'
    pub const VOWEL_SIGN_VOCALIC_L: char = '𑁀';
    /// \u{11041}: '𑁁'
    pub const VOWEL_SIGN_VOCALIC_LL: char = '𑁁';
    /// \u{11042}: '𑁂'
    pub const VOWEL_SIGN_E: char = '𑁂';
    /// \u{11043}: '𑁃'
    pub const VOWEL_SIGN_AI: char = '𑁃';
    /// \u{11044}: '𑁄'
    pub const VOWEL_SIGN_O: char = '𑁄';
    /// \u{11045}: '𑁅'
    pub const VOWEL_SIGN_AU: char = '𑁅';
    /// \u{11046}: '𑁆'
    pub const VIRAMA: char = '𑁆';
    /// \u{11047}: '𑁇'
    pub const DANDA: char = '𑁇';
    /// \u{11048}: '𑁈'
    pub const DOUBLE_DANDA: char = '𑁈';
    /// \u{11049}: '𑁉'
    pub const PUNCTUATION_DOT: char = '𑁉';
    /// \u{1104a}: '𑁊'
    pub const PUNCTUATION_DOUBLE_DOT: char = '𑁊';
    /// \u{1104b}: '𑁋'
    pub const PUNCTUATION_LINE: char = '𑁋';
    /// \u{1104c}: '𑁌'
    pub const PUNCTUATION_CRESCENT_BAR: char = '𑁌';
    /// \u{1104d}: '𑁍'
    pub const PUNCTUATION_LOTUS: char = '𑁍';
    /// \u{11052}: '𑁒'
    pub const NUMBER_ONE: char = '𑁒';
    /// \u{11053}: '𑁓'
    pub const NUMBER_TWO: char = '𑁓';
    /// \u{11054}: '𑁔'
    pub const NUMBER_THREE: char = '𑁔';
    /// \u{11055}: '𑁕'
    pub const NUMBER_FOUR: char = '𑁕';
    /// \u{11056}: '𑁖'
    pub const NUMBER_FIVE: char = '𑁖';
    /// \u{11057}: '𑁗'
    pub const NUMBER_SIX: char = '𑁗';
    /// \u{11058}: '𑁘'
    pub const NUMBER_SEVEN: char = '𑁘';
    /// \u{11059}: '𑁙'
    pub const NUMBER_EIGHT: char = '𑁙';
    /// \u{1105a}: '𑁚'
    pub const NUMBER_NINE: char = '𑁚';
    /// \u{1105b}: '𑁛'
    pub const NUMBER_TEN: char = '𑁛';
    /// \u{1105c}: '𑁜'
    pub const NUMBER_TWENTY: char = '𑁜';
    /// \u{1105d}: '𑁝'
    pub const NUMBER_THIRTY: char = '𑁝';
    /// \u{1105e}: '𑁞'
    pub const NUMBER_FORTY: char = '𑁞';
    /// \u{1105f}: '𑁟'
    pub const NUMBER_FIFTY: char = '𑁟';
    /// \u{11060}: '𑁠'
    pub const NUMBER_SIXTY: char = '𑁠';
    /// \u{11061}: '𑁡'
    pub const NUMBER_SEVENTY: char = '𑁡';
    /// \u{11062}: '𑁢'
    pub const NUMBER_EIGHTY: char = '𑁢';
    /// \u{11063}: '𑁣'
    pub const NUMBER_NINETY: char = '𑁣';
    /// \u{11064}: '𑁤'
    pub const NUMBER_ONE_HUNDRED: char = '𑁤';
    /// \u{11065}: '𑁥'
    pub const NUMBER_ONE_THOUSAND: char = '𑁥';
    /// \u{11066}: '𑁦'
    pub const DIGIT_ZERO: char = '𑁦';
    /// \u{11067}: '𑁧'
    pub const DIGIT_ONE: char = '𑁧';
    /// \u{11068}: '𑁨'
    pub const DIGIT_TWO: char = '𑁨';
    /// \u{11069}: '𑁩'
    pub const DIGIT_THREE: char = '𑁩';
    /// \u{1106a}: '𑁪'
    pub const DIGIT_FOUR: char = '𑁪';
    /// \u{1106b}: '𑁫'
    pub const DIGIT_FIVE: char = '𑁫';
    /// \u{1106c}: '𑁬'
    pub const DIGIT_SIX: char = '𑁬';
    /// \u{1106d}: '𑁭'
    pub const DIGIT_SEVEN: char = '𑁭';
    /// \u{1106e}: '𑁮'
    pub const DIGIT_EIGHT: char = '𑁮';
    /// \u{1106f}: '𑁯'
    pub const DIGIT_NINE: char = '𑁯';
}

/// An enum to represent all characters in the Brahmi block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Brahmi {
    /// \u{11000}: '𑀀'
    SignCandrabindu,
    /// \u{11001}: '𑀁'
    SignAnusvara,
    /// \u{11002}: '𑀂'
    SignVisarga,
    /// \u{11003}: '𑀃'
    SignJihvamuliya,
    /// \u{11004}: '𑀄'
    SignUpadhmaniya,
    /// \u{11005}: '𑀅'
    LetterA,
    /// \u{11006}: '𑀆'
    LetterAa,
    /// \u{11007}: '𑀇'
    LetterI,
    /// \u{11008}: '𑀈'
    LetterIi,
    /// \u{11009}: '𑀉'
    LetterU,
    /// \u{1100a}: '𑀊'
    LetterUu,
    /// \u{1100b}: '𑀋'
    LetterVocalicR,
    /// \u{1100c}: '𑀌'
    LetterVocalicRr,
    /// \u{1100d}: '𑀍'
    LetterVocalicL,
    /// \u{1100e}: '𑀎'
    LetterVocalicLl,
    /// \u{1100f}: '𑀏'
    LetterE,
    /// \u{11010}: '𑀐'
    LetterAi,
    /// \u{11011}: '𑀑'
    LetterO,
    /// \u{11012}: '𑀒'
    LetterAu,
    /// \u{11013}: '𑀓'
    LetterKa,
    /// \u{11014}: '𑀔'
    LetterKha,
    /// \u{11015}: '𑀕'
    LetterGa,
    /// \u{11016}: '𑀖'
    LetterGha,
    /// \u{11017}: '𑀗'
    LetterNga,
    /// \u{11018}: '𑀘'
    LetterCa,
    /// \u{11019}: '𑀙'
    LetterCha,
    /// \u{1101a}: '𑀚'
    LetterJa,
    /// \u{1101b}: '𑀛'
    LetterJha,
    /// \u{1101c}: '𑀜'
    LetterNya,
    /// \u{1101d}: '𑀝'
    LetterTta,
    /// \u{1101e}: '𑀞'
    LetterTtha,
    /// \u{1101f}: '𑀟'
    LetterDda,
    /// \u{11020}: '𑀠'
    LetterDdha,
    /// \u{11021}: '𑀡'
    LetterNna,
    /// \u{11022}: '𑀢'
    LetterTa,
    /// \u{11023}: '𑀣'
    LetterTha,
    /// \u{11024}: '𑀤'
    LetterDa,
    /// \u{11025}: '𑀥'
    LetterDha,
    /// \u{11026}: '𑀦'
    LetterNa,
    /// \u{11027}: '𑀧'
    LetterPa,
    /// \u{11028}: '𑀨'
    LetterPha,
    /// \u{11029}: '𑀩'
    LetterBa,
    /// \u{1102a}: '𑀪'
    LetterBha,
    /// \u{1102b}: '𑀫'
    LetterMa,
    /// \u{1102c}: '𑀬'
    LetterYa,
    /// \u{1102d}: '𑀭'
    LetterRa,
    /// \u{1102e}: '𑀮'
    LetterLa,
    /// \u{1102f}: '𑀯'
    LetterVa,
    /// \u{11030}: '𑀰'
    LetterSha,
    /// \u{11031}: '𑀱'
    LetterSsa,
    /// \u{11032}: '𑀲'
    LetterSa,
    /// \u{11033}: '𑀳'
    LetterHa,
    /// \u{11034}: '𑀴'
    LetterLla,
    /// \u{11035}: '𑀵'
    LetterOldTamilLlla,
    /// \u{11036}: '𑀶'
    LetterOldTamilRra,
    /// \u{11037}: '𑀷'
    LetterOldTamilNnna,
    /// \u{11038}: '𑀸'
    VowelSignAa,
    /// \u{11039}: '𑀹'
    VowelSignBhattiproluAa,
    /// \u{1103a}: '𑀺'
    VowelSignI,
    /// \u{1103b}: '𑀻'
    VowelSignIi,
    /// \u{1103c}: '𑀼'
    VowelSignU,
    /// \u{1103d}: '𑀽'
    VowelSignUu,
    /// \u{1103e}: '𑀾'
    VowelSignVocalicR,
    /// \u{1103f}: '𑀿'
    VowelSignVocalicRr,
    /// \u{11040}: '𑁀'
    VowelSignVocalicL,
    /// \u{11041}: '𑁁'
    VowelSignVocalicLl,
    /// \u{11042}: '𑁂'
    VowelSignE,
    /// \u{11043}: '𑁃'
    VowelSignAi,
    /// \u{11044}: '𑁄'
    VowelSignO,
    /// \u{11045}: '𑁅'
    VowelSignAu,
    /// \u{11046}: '𑁆'
    Virama,
    /// \u{11047}: '𑁇'
    Danda,
    /// \u{11048}: '𑁈'
    DoubleDanda,
    /// \u{11049}: '𑁉'
    PunctuationDot,
    /// \u{1104a}: '𑁊'
    PunctuationDoubleDot,
    /// \u{1104b}: '𑁋'
    PunctuationLine,
    /// \u{1104c}: '𑁌'
    PunctuationCrescentBar,
    /// \u{1104d}: '𑁍'
    PunctuationLotus,
    /// \u{11052}: '𑁒'
    NumberOne,
    /// \u{11053}: '𑁓'
    NumberTwo,
    /// \u{11054}: '𑁔'
    NumberThree,
    /// \u{11055}: '𑁕'
    NumberFour,
    /// \u{11056}: '𑁖'
    NumberFive,
    /// \u{11057}: '𑁗'
    NumberSix,
    /// \u{11058}: '𑁘'
    NumberSeven,
    /// \u{11059}: '𑁙'
    NumberEight,
    /// \u{1105a}: '𑁚'
    NumberNine,
    /// \u{1105b}: '𑁛'
    NumberTen,
    /// \u{1105c}: '𑁜'
    NumberTwenty,
    /// \u{1105d}: '𑁝'
    NumberThirty,
    /// \u{1105e}: '𑁞'
    NumberForty,
    /// \u{1105f}: '𑁟'
    NumberFifty,
    /// \u{11060}: '𑁠'
    NumberSixty,
    /// \u{11061}: '𑁡'
    NumberSeventy,
    /// \u{11062}: '𑁢'
    NumberEighty,
    /// \u{11063}: '𑁣'
    NumberNinety,
    /// \u{11064}: '𑁤'
    NumberOneHundred,
    /// \u{11065}: '𑁥'
    NumberOneThousand,
    /// \u{11066}: '𑁦'
    DigitZero,
    /// \u{11067}: '𑁧'
    DigitOne,
    /// \u{11068}: '𑁨'
    DigitTwo,
    /// \u{11069}: '𑁩'
    DigitThree,
    /// \u{1106a}: '𑁪'
    DigitFour,
    /// \u{1106b}: '𑁫'
    DigitFive,
    /// \u{1106c}: '𑁬'
    DigitSix,
    /// \u{1106d}: '𑁭'
    DigitSeven,
    /// \u{1106e}: '𑁮'
    DigitEight,
    /// \u{1106f}: '𑁯'
    DigitNine,
}

impl Into<char> for Brahmi {
    fn into(self) -> char {
        use constants::*;
        match self {
            Brahmi::SignCandrabindu => SIGN_CANDRABINDU,
            Brahmi::SignAnusvara => SIGN_ANUSVARA,
            Brahmi::SignVisarga => SIGN_VISARGA,
            Brahmi::SignJihvamuliya => SIGN_JIHVAMULIYA,
            Brahmi::SignUpadhmaniya => SIGN_UPADHMANIYA,
            Brahmi::LetterA => LETTER_A,
            Brahmi::LetterAa => LETTER_AA,
            Brahmi::LetterI => LETTER_I,
            Brahmi::LetterIi => LETTER_II,
            Brahmi::LetterU => LETTER_U,
            Brahmi::LetterUu => LETTER_UU,
            Brahmi::LetterVocalicR => LETTER_VOCALIC_R,
            Brahmi::LetterVocalicRr => LETTER_VOCALIC_RR,
            Brahmi::LetterVocalicL => LETTER_VOCALIC_L,
            Brahmi::LetterVocalicLl => LETTER_VOCALIC_LL,
            Brahmi::LetterE => LETTER_E,
            Brahmi::LetterAi => LETTER_AI,
            Brahmi::LetterO => LETTER_O,
            Brahmi::LetterAu => LETTER_AU,
            Brahmi::LetterKa => LETTER_KA,
            Brahmi::LetterKha => LETTER_KHA,
            Brahmi::LetterGa => LETTER_GA,
            Brahmi::LetterGha => LETTER_GHA,
            Brahmi::LetterNga => LETTER_NGA,
            Brahmi::LetterCa => LETTER_CA,
            Brahmi::LetterCha => LETTER_CHA,
            Brahmi::LetterJa => LETTER_JA,
            Brahmi::LetterJha => LETTER_JHA,
            Brahmi::LetterNya => LETTER_NYA,
            Brahmi::LetterTta => LETTER_TTA,
            Brahmi::LetterTtha => LETTER_TTHA,
            Brahmi::LetterDda => LETTER_DDA,
            Brahmi::LetterDdha => LETTER_DDHA,
            Brahmi::LetterNna => LETTER_NNA,
            Brahmi::LetterTa => LETTER_TA,
            Brahmi::LetterTha => LETTER_THA,
            Brahmi::LetterDa => LETTER_DA,
            Brahmi::LetterDha => LETTER_DHA,
            Brahmi::LetterNa => LETTER_NA,
            Brahmi::LetterPa => LETTER_PA,
            Brahmi::LetterPha => LETTER_PHA,
            Brahmi::LetterBa => LETTER_BA,
            Brahmi::LetterBha => LETTER_BHA,
            Brahmi::LetterMa => LETTER_MA,
            Brahmi::LetterYa => LETTER_YA,
            Brahmi::LetterRa => LETTER_RA,
            Brahmi::LetterLa => LETTER_LA,
            Brahmi::LetterVa => LETTER_VA,
            Brahmi::LetterSha => LETTER_SHA,
            Brahmi::LetterSsa => LETTER_SSA,
            Brahmi::LetterSa => LETTER_SA,
            Brahmi::LetterHa => LETTER_HA,
            Brahmi::LetterLla => LETTER_LLA,
            Brahmi::LetterOldTamilLlla => LETTER_OLD_TAMIL_LLLA,
            Brahmi::LetterOldTamilRra => LETTER_OLD_TAMIL_RRA,
            Brahmi::LetterOldTamilNnna => LETTER_OLD_TAMIL_NNNA,
            Brahmi::VowelSignAa => VOWEL_SIGN_AA,
            Brahmi::VowelSignBhattiproluAa => VOWEL_SIGN_BHATTIPROLU_AA,
            Brahmi::VowelSignI => VOWEL_SIGN_I,
            Brahmi::VowelSignIi => VOWEL_SIGN_II,
            Brahmi::VowelSignU => VOWEL_SIGN_U,
            Brahmi::VowelSignUu => VOWEL_SIGN_UU,
            Brahmi::VowelSignVocalicR => VOWEL_SIGN_VOCALIC_R,
            Brahmi::VowelSignVocalicRr => VOWEL_SIGN_VOCALIC_RR,
            Brahmi::VowelSignVocalicL => VOWEL_SIGN_VOCALIC_L,
            Brahmi::VowelSignVocalicLl => VOWEL_SIGN_VOCALIC_LL,
            Brahmi::VowelSignE => VOWEL_SIGN_E,
            Brahmi::VowelSignAi => VOWEL_SIGN_AI,
            Brahmi::VowelSignO => VOWEL_SIGN_O,
            Brahmi::VowelSignAu => VOWEL_SIGN_AU,
            Brahmi::Virama => VIRAMA,
            Brahmi::Danda => DANDA,
            Brahmi::DoubleDanda => DOUBLE_DANDA,
            Brahmi::PunctuationDot => PUNCTUATION_DOT,
            Brahmi::PunctuationDoubleDot => PUNCTUATION_DOUBLE_DOT,
            Brahmi::PunctuationLine => PUNCTUATION_LINE,
            Brahmi::PunctuationCrescentBar => PUNCTUATION_CRESCENT_BAR,
            Brahmi::PunctuationLotus => PUNCTUATION_LOTUS,
            Brahmi::NumberOne => NUMBER_ONE,
            Brahmi::NumberTwo => NUMBER_TWO,
            Brahmi::NumberThree => NUMBER_THREE,
            Brahmi::NumberFour => NUMBER_FOUR,
            Brahmi::NumberFive => NUMBER_FIVE,
            Brahmi::NumberSix => NUMBER_SIX,
            Brahmi::NumberSeven => NUMBER_SEVEN,
            Brahmi::NumberEight => NUMBER_EIGHT,
            Brahmi::NumberNine => NUMBER_NINE,
            Brahmi::NumberTen => NUMBER_TEN,
            Brahmi::NumberTwenty => NUMBER_TWENTY,
            Brahmi::NumberThirty => NUMBER_THIRTY,
            Brahmi::NumberForty => NUMBER_FORTY,
            Brahmi::NumberFifty => NUMBER_FIFTY,
            Brahmi::NumberSixty => NUMBER_SIXTY,
            Brahmi::NumberSeventy => NUMBER_SEVENTY,
            Brahmi::NumberEighty => NUMBER_EIGHTY,
            Brahmi::NumberNinety => NUMBER_NINETY,
            Brahmi::NumberOneHundred => NUMBER_ONE_HUNDRED,
            Brahmi::NumberOneThousand => NUMBER_ONE_THOUSAND,
            Brahmi::DigitZero => DIGIT_ZERO,
            Brahmi::DigitOne => DIGIT_ONE,
            Brahmi::DigitTwo => DIGIT_TWO,
            Brahmi::DigitThree => DIGIT_THREE,
            Brahmi::DigitFour => DIGIT_FOUR,
            Brahmi::DigitFive => DIGIT_FIVE,
            Brahmi::DigitSix => DIGIT_SIX,
            Brahmi::DigitSeven => DIGIT_SEVEN,
            Brahmi::DigitEight => DIGIT_EIGHT,
            Brahmi::DigitNine => DIGIT_NINE,
        }
    }
}

impl std::convert::TryFrom<char> for Brahmi {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            SIGN_CANDRABINDU => Ok(Brahmi::SignCandrabindu),
            SIGN_ANUSVARA => Ok(Brahmi::SignAnusvara),
            SIGN_VISARGA => Ok(Brahmi::SignVisarga),
            SIGN_JIHVAMULIYA => Ok(Brahmi::SignJihvamuliya),
            SIGN_UPADHMANIYA => Ok(Brahmi::SignUpadhmaniya),
            LETTER_A => Ok(Brahmi::LetterA),
            LETTER_AA => Ok(Brahmi::LetterAa),
            LETTER_I => Ok(Brahmi::LetterI),
            LETTER_II => Ok(Brahmi::LetterIi),
            LETTER_U => Ok(Brahmi::LetterU),
            LETTER_UU => Ok(Brahmi::LetterUu),
            LETTER_VOCALIC_R => Ok(Brahmi::LetterVocalicR),
            LETTER_VOCALIC_RR => Ok(Brahmi::LetterVocalicRr),
            LETTER_VOCALIC_L => Ok(Brahmi::LetterVocalicL),
            LETTER_VOCALIC_LL => Ok(Brahmi::LetterVocalicLl),
            LETTER_E => Ok(Brahmi::LetterE),
            LETTER_AI => Ok(Brahmi::LetterAi),
            LETTER_O => Ok(Brahmi::LetterO),
            LETTER_AU => Ok(Brahmi::LetterAu),
            LETTER_KA => Ok(Brahmi::LetterKa),
            LETTER_KHA => Ok(Brahmi::LetterKha),
            LETTER_GA => Ok(Brahmi::LetterGa),
            LETTER_GHA => Ok(Brahmi::LetterGha),
            LETTER_NGA => Ok(Brahmi::LetterNga),
            LETTER_CA => Ok(Brahmi::LetterCa),
            LETTER_CHA => Ok(Brahmi::LetterCha),
            LETTER_JA => Ok(Brahmi::LetterJa),
            LETTER_JHA => Ok(Brahmi::LetterJha),
            LETTER_NYA => Ok(Brahmi::LetterNya),
            LETTER_TTA => Ok(Brahmi::LetterTta),
            LETTER_TTHA => Ok(Brahmi::LetterTtha),
            LETTER_DDA => Ok(Brahmi::LetterDda),
            LETTER_DDHA => Ok(Brahmi::LetterDdha),
            LETTER_NNA => Ok(Brahmi::LetterNna),
            LETTER_TA => Ok(Brahmi::LetterTa),
            LETTER_THA => Ok(Brahmi::LetterTha),
            LETTER_DA => Ok(Brahmi::LetterDa),
            LETTER_DHA => Ok(Brahmi::LetterDha),
            LETTER_NA => Ok(Brahmi::LetterNa),
            LETTER_PA => Ok(Brahmi::LetterPa),
            LETTER_PHA => Ok(Brahmi::LetterPha),
            LETTER_BA => Ok(Brahmi::LetterBa),
            LETTER_BHA => Ok(Brahmi::LetterBha),
            LETTER_MA => Ok(Brahmi::LetterMa),
            LETTER_YA => Ok(Brahmi::LetterYa),
            LETTER_RA => Ok(Brahmi::LetterRa),
            LETTER_LA => Ok(Brahmi::LetterLa),
            LETTER_VA => Ok(Brahmi::LetterVa),
            LETTER_SHA => Ok(Brahmi::LetterSha),
            LETTER_SSA => Ok(Brahmi::LetterSsa),
            LETTER_SA => Ok(Brahmi::LetterSa),
            LETTER_HA => Ok(Brahmi::LetterHa),
            LETTER_LLA => Ok(Brahmi::LetterLla),
            LETTER_OLD_TAMIL_LLLA => Ok(Brahmi::LetterOldTamilLlla),
            LETTER_OLD_TAMIL_RRA => Ok(Brahmi::LetterOldTamilRra),
            LETTER_OLD_TAMIL_NNNA => Ok(Brahmi::LetterOldTamilNnna),
            VOWEL_SIGN_AA => Ok(Brahmi::VowelSignAa),
            VOWEL_SIGN_BHATTIPROLU_AA => Ok(Brahmi::VowelSignBhattiproluAa),
            VOWEL_SIGN_I => Ok(Brahmi::VowelSignI),
            VOWEL_SIGN_II => Ok(Brahmi::VowelSignIi),
            VOWEL_SIGN_U => Ok(Brahmi::VowelSignU),
            VOWEL_SIGN_UU => Ok(Brahmi::VowelSignUu),
            VOWEL_SIGN_VOCALIC_R => Ok(Brahmi::VowelSignVocalicR),
            VOWEL_SIGN_VOCALIC_RR => Ok(Brahmi::VowelSignVocalicRr),
            VOWEL_SIGN_VOCALIC_L => Ok(Brahmi::VowelSignVocalicL),
            VOWEL_SIGN_VOCALIC_LL => Ok(Brahmi::VowelSignVocalicLl),
            VOWEL_SIGN_E => Ok(Brahmi::VowelSignE),
            VOWEL_SIGN_AI => Ok(Brahmi::VowelSignAi),
            VOWEL_SIGN_O => Ok(Brahmi::VowelSignO),
            VOWEL_SIGN_AU => Ok(Brahmi::VowelSignAu),
            VIRAMA => Ok(Brahmi::Virama),
            DANDA => Ok(Brahmi::Danda),
            DOUBLE_DANDA => Ok(Brahmi::DoubleDanda),
            PUNCTUATION_DOT => Ok(Brahmi::PunctuationDot),
            PUNCTUATION_DOUBLE_DOT => Ok(Brahmi::PunctuationDoubleDot),
            PUNCTUATION_LINE => Ok(Brahmi::PunctuationLine),
            PUNCTUATION_CRESCENT_BAR => Ok(Brahmi::PunctuationCrescentBar),
            PUNCTUATION_LOTUS => Ok(Brahmi::PunctuationLotus),
            NUMBER_ONE => Ok(Brahmi::NumberOne),
            NUMBER_TWO => Ok(Brahmi::NumberTwo),
            NUMBER_THREE => Ok(Brahmi::NumberThree),
            NUMBER_FOUR => Ok(Brahmi::NumberFour),
            NUMBER_FIVE => Ok(Brahmi::NumberFive),
            NUMBER_SIX => Ok(Brahmi::NumberSix),
            NUMBER_SEVEN => Ok(Brahmi::NumberSeven),
            NUMBER_EIGHT => Ok(Brahmi::NumberEight),
            NUMBER_NINE => Ok(Brahmi::NumberNine),
            NUMBER_TEN => Ok(Brahmi::NumberTen),
            NUMBER_TWENTY => Ok(Brahmi::NumberTwenty),
            NUMBER_THIRTY => Ok(Brahmi::NumberThirty),
            NUMBER_FORTY => Ok(Brahmi::NumberForty),
            NUMBER_FIFTY => Ok(Brahmi::NumberFifty),
            NUMBER_SIXTY => Ok(Brahmi::NumberSixty),
            NUMBER_SEVENTY => Ok(Brahmi::NumberSeventy),
            NUMBER_EIGHTY => Ok(Brahmi::NumberEighty),
            NUMBER_NINETY => Ok(Brahmi::NumberNinety),
            NUMBER_ONE_HUNDRED => Ok(Brahmi::NumberOneHundred),
            NUMBER_ONE_THOUSAND => Ok(Brahmi::NumberOneThousand),
            DIGIT_ZERO => Ok(Brahmi::DigitZero),
            DIGIT_ONE => Ok(Brahmi::DigitOne),
            DIGIT_TWO => Ok(Brahmi::DigitTwo),
            DIGIT_THREE => Ok(Brahmi::DigitThree),
            DIGIT_FOUR => Ok(Brahmi::DigitFour),
            DIGIT_FIVE => Ok(Brahmi::DigitFive),
            DIGIT_SIX => Ok(Brahmi::DigitSix),
            DIGIT_SEVEN => Ok(Brahmi::DigitSeven),
            DIGIT_EIGHT => Ok(Brahmi::DigitEight),
            DIGIT_NINE => Ok(Brahmi::DigitNine),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Brahmi {
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

impl std::convert::TryFrom<u32> for Brahmi {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Brahmi {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Brahmi {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Brahmi::SignCandrabindu
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Brahmi::SignCandrabindu => "brahmi sign candrabindu",
            Brahmi::SignAnusvara => "brahmi sign anusvara",
            Brahmi::SignVisarga => "brahmi sign visarga",
            Brahmi::SignJihvamuliya => "brahmi sign jihvamuliya",
            Brahmi::SignUpadhmaniya => "brahmi sign upadhmaniya",
            Brahmi::LetterA => "brahmi letter a",
            Brahmi::LetterAa => "brahmi letter aa",
            Brahmi::LetterI => "brahmi letter i",
            Brahmi::LetterIi => "brahmi letter ii",
            Brahmi::LetterU => "brahmi letter u",
            Brahmi::LetterUu => "brahmi letter uu",
            Brahmi::LetterVocalicR => "brahmi letter vocalic r",
            Brahmi::LetterVocalicRr => "brahmi letter vocalic rr",
            Brahmi::LetterVocalicL => "brahmi letter vocalic l",
            Brahmi::LetterVocalicLl => "brahmi letter vocalic ll",
            Brahmi::LetterE => "brahmi letter e",
            Brahmi::LetterAi => "brahmi letter ai",
            Brahmi::LetterO => "brahmi letter o",
            Brahmi::LetterAu => "brahmi letter au",
            Brahmi::LetterKa => "brahmi letter ka",
            Brahmi::LetterKha => "brahmi letter kha",
            Brahmi::LetterGa => "brahmi letter ga",
            Brahmi::LetterGha => "brahmi letter gha",
            Brahmi::LetterNga => "brahmi letter nga",
            Brahmi::LetterCa => "brahmi letter ca",
            Brahmi::LetterCha => "brahmi letter cha",
            Brahmi::LetterJa => "brahmi letter ja",
            Brahmi::LetterJha => "brahmi letter jha",
            Brahmi::LetterNya => "brahmi letter nya",
            Brahmi::LetterTta => "brahmi letter tta",
            Brahmi::LetterTtha => "brahmi letter ttha",
            Brahmi::LetterDda => "brahmi letter dda",
            Brahmi::LetterDdha => "brahmi letter ddha",
            Brahmi::LetterNna => "brahmi letter nna",
            Brahmi::LetterTa => "brahmi letter ta",
            Brahmi::LetterTha => "brahmi letter tha",
            Brahmi::LetterDa => "brahmi letter da",
            Brahmi::LetterDha => "brahmi letter dha",
            Brahmi::LetterNa => "brahmi letter na",
            Brahmi::LetterPa => "brahmi letter pa",
            Brahmi::LetterPha => "brahmi letter pha",
            Brahmi::LetterBa => "brahmi letter ba",
            Brahmi::LetterBha => "brahmi letter bha",
            Brahmi::LetterMa => "brahmi letter ma",
            Brahmi::LetterYa => "brahmi letter ya",
            Brahmi::LetterRa => "brahmi letter ra",
            Brahmi::LetterLa => "brahmi letter la",
            Brahmi::LetterVa => "brahmi letter va",
            Brahmi::LetterSha => "brahmi letter sha",
            Brahmi::LetterSsa => "brahmi letter ssa",
            Brahmi::LetterSa => "brahmi letter sa",
            Brahmi::LetterHa => "brahmi letter ha",
            Brahmi::LetterLla => "brahmi letter lla",
            Brahmi::LetterOldTamilLlla => "brahmi letter old tamil llla",
            Brahmi::LetterOldTamilRra => "brahmi letter old tamil rra",
            Brahmi::LetterOldTamilNnna => "brahmi letter old tamil nnna",
            Brahmi::VowelSignAa => "brahmi vowel sign aa",
            Brahmi::VowelSignBhattiproluAa => "brahmi vowel sign bhattiprolu aa",
            Brahmi::VowelSignI => "brahmi vowel sign i",
            Brahmi::VowelSignIi => "brahmi vowel sign ii",
            Brahmi::VowelSignU => "brahmi vowel sign u",
            Brahmi::VowelSignUu => "brahmi vowel sign uu",
            Brahmi::VowelSignVocalicR => "brahmi vowel sign vocalic r",
            Brahmi::VowelSignVocalicRr => "brahmi vowel sign vocalic rr",
            Brahmi::VowelSignVocalicL => "brahmi vowel sign vocalic l",
            Brahmi::VowelSignVocalicLl => "brahmi vowel sign vocalic ll",
            Brahmi::VowelSignE => "brahmi vowel sign e",
            Brahmi::VowelSignAi => "brahmi vowel sign ai",
            Brahmi::VowelSignO => "brahmi vowel sign o",
            Brahmi::VowelSignAu => "brahmi vowel sign au",
            Brahmi::Virama => "brahmi virama",
            Brahmi::Danda => "brahmi danda",
            Brahmi::DoubleDanda => "brahmi double danda",
            Brahmi::PunctuationDot => "brahmi punctuation dot",
            Brahmi::PunctuationDoubleDot => "brahmi punctuation double dot",
            Brahmi::PunctuationLine => "brahmi punctuation line",
            Brahmi::PunctuationCrescentBar => "brahmi punctuation crescent bar",
            Brahmi::PunctuationLotus => "brahmi punctuation lotus",
            Brahmi::NumberOne => "brahmi number one",
            Brahmi::NumberTwo => "brahmi number two",
            Brahmi::NumberThree => "brahmi number three",
            Brahmi::NumberFour => "brahmi number four",
            Brahmi::NumberFive => "brahmi number five",
            Brahmi::NumberSix => "brahmi number six",
            Brahmi::NumberSeven => "brahmi number seven",
            Brahmi::NumberEight => "brahmi number eight",
            Brahmi::NumberNine => "brahmi number nine",
            Brahmi::NumberTen => "brahmi number ten",
            Brahmi::NumberTwenty => "brahmi number twenty",
            Brahmi::NumberThirty => "brahmi number thirty",
            Brahmi::NumberForty => "brahmi number forty",
            Brahmi::NumberFifty => "brahmi number fifty",
            Brahmi::NumberSixty => "brahmi number sixty",
            Brahmi::NumberSeventy => "brahmi number seventy",
            Brahmi::NumberEighty => "brahmi number eighty",
            Brahmi::NumberNinety => "brahmi number ninety",
            Brahmi::NumberOneHundred => "brahmi number one hundred",
            Brahmi::NumberOneThousand => "brahmi number one thousand",
            Brahmi::DigitZero => "brahmi digit zero",
            Brahmi::DigitOne => "brahmi digit one",
            Brahmi::DigitTwo => "brahmi digit two",
            Brahmi::DigitThree => "brahmi digit three",
            Brahmi::DigitFour => "brahmi digit four",
            Brahmi::DigitFive => "brahmi digit five",
            Brahmi::DigitSix => "brahmi digit six",
            Brahmi::DigitSeven => "brahmi digit seven",
            Brahmi::DigitEight => "brahmi digit eight",
            Brahmi::DigitNine => "brahmi digit nine",
        }
    }
}
