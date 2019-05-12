/// \u{1800} → \u{18af}
///
/// ᠀ ᠁ ᠂ ᠃ ᠄ ᠅ ᠆ ᠇ ᠈ ᠉ ᠊ ᠋ ᠌ ᠍ ᠎ ᠐\
/// ᠑ ᠒ ᠓ ᠔ ᠕ ᠖ ᠗ ᠘ ᠙ ᠠ ᠡ ᠢ ᠣ ᠤ ᠥ ᠦ\
/// ᠧ ᠨ ᠩ ᠪ ᠫ ᠬ ᠭ ᠮ ᠯ ᠰ ᠱ ᠲ ᠳ ᠴ ᠵ ᠶ\
/// ᠷ ᠸ ᠹ ᠺ ᠻ ᠼ ᠽ ᠾ ᠿ ᡀ ᡁ ᡂ ᡃ ᡄ ᡅ ᡆ\
/// ᡇ ᡈ ᡉ ᡊ ᡋ ᡌ ᡍ ᡎ ᡏ ᡐ ᡑ ᡒ ᡓ ᡔ ᡕ ᡖ\
/// ᡗ ᡘ ᡙ ᡚ ᡛ ᡜ ᡝ ᡞ ᡟ ᡠ ᡡ ᡢ ᡣ ᡤ ᡥ ᡦ\
/// ᡧ ᡨ ᡩ ᡪ ᡫ ᡬ ᡭ ᡮ ᡯ ᡰ ᡱ ᡲ ᡳ ᡴ ᡵ ᡶ\
/// ᡷ ᡸ ᢀ ᢁ ᢂ ᢃ ᢄ ᢅ ᢆ ᢇ ᢈ ᢉ ᢊ ᢋ ᢌ ᢍ\
/// ᢎ ᢏ ᢐ ᢑ ᢒ ᢓ ᢔ ᢕ ᢖ ᢗ ᢘ ᢙ ᢚ ᢛ ᢜ ᢝ\
/// ᢞ ᢟ ᢠ ᢡ ᢢ ᢣ ᢤ ᢥ ᢦ ᢧ ᢨ ᢩ ᢪ\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{1800}: '᠀'
    pub const BIRGA: char = '᠀';
    /// \u{1801}: '᠁'
    pub const ELLIPSIS: char = '᠁';
    /// \u{1802}: '᠂'
    pub const COMMA: char = '᠂';
    /// \u{1803}: '᠃'
    pub const FULL_STOP: char = '᠃';
    /// \u{1804}: '᠄'
    pub const COLON: char = '᠄';
    /// \u{1805}: '᠅'
    pub const FOUR_DOTS: char = '᠅';
    /// \u{1806}: '᠆'
    pub const TODO_SOFT_HYPHEN: char = '᠆';
    /// \u{1807}: '᠇'
    pub const SIBE_SYLLABLE_BOUNDARY_MARKER: char = '᠇';
    /// \u{1808}: '᠈'
    pub const MANCHU_COMMA: char = '᠈';
    /// \u{1809}: '᠉'
    pub const MANCHU_FULL_STOP: char = '᠉';
    /// \u{180a}: '᠊'
    pub const NIRUGU: char = '᠊';
    /// \u{180b}: '᠋'
    pub const FREE_VARIATION_SELECTOR_ONE: char = '᠋';
    /// \u{180c}: '᠌'
    pub const FREE_VARIATION_SELECTOR_TWO: char = '᠌';
    /// \u{180d}: '᠍'
    pub const FREE_VARIATION_SELECTOR_THREE: char = '᠍';
    /// \u{180e}: '᠎'
    pub const VOWEL_SEPARATOR: char = '᠎';
    /// \u{1810}: '᠐'
    pub const DIGIT_ZERO: char = '᠐';
    /// \u{1811}: '᠑'
    pub const DIGIT_ONE: char = '᠑';
    /// \u{1812}: '᠒'
    pub const DIGIT_TWO: char = '᠒';
    /// \u{1813}: '᠓'
    pub const DIGIT_THREE: char = '᠓';
    /// \u{1814}: '᠔'
    pub const DIGIT_FOUR: char = '᠔';
    /// \u{1815}: '᠕'
    pub const DIGIT_FIVE: char = '᠕';
    /// \u{1816}: '᠖'
    pub const DIGIT_SIX: char = '᠖';
    /// \u{1817}: '᠗'
    pub const DIGIT_SEVEN: char = '᠗';
    /// \u{1818}: '᠘'
    pub const DIGIT_EIGHT: char = '᠘';
    /// \u{1819}: '᠙'
    pub const DIGIT_NINE: char = '᠙';
    /// \u{1820}: 'ᠠ'
    pub const LETTER_A: char = 'ᠠ';
    /// \u{1821}: 'ᠡ'
    pub const LETTER_E: char = 'ᠡ';
    /// \u{1822}: 'ᠢ'
    pub const LETTER_I: char = 'ᠢ';
    /// \u{1823}: 'ᠣ'
    pub const LETTER_O: char = 'ᠣ';
    /// \u{1824}: 'ᠤ'
    pub const LETTER_U: char = 'ᠤ';
    /// \u{1825}: 'ᠥ'
    pub const LETTER_OE: char = 'ᠥ';
    /// \u{1826}: 'ᠦ'
    pub const LETTER_UE: char = 'ᠦ';
    /// \u{1827}: 'ᠧ'
    pub const LETTER_EE: char = 'ᠧ';
    /// \u{1828}: 'ᠨ'
    pub const LETTER_NA: char = 'ᠨ';
    /// \u{1829}: 'ᠩ'
    pub const LETTER_ANG: char = 'ᠩ';
    /// \u{182a}: 'ᠪ'
    pub const LETTER_BA: char = 'ᠪ';
    /// \u{182b}: 'ᠫ'
    pub const LETTER_PA: char = 'ᠫ';
    /// \u{182c}: 'ᠬ'
    pub const LETTER_QA: char = 'ᠬ';
    /// \u{182d}: 'ᠭ'
    pub const LETTER_GA: char = 'ᠭ';
    /// \u{182e}: 'ᠮ'
    pub const LETTER_MA: char = 'ᠮ';
    /// \u{182f}: 'ᠯ'
    pub const LETTER_LA: char = 'ᠯ';
    /// \u{1830}: 'ᠰ'
    pub const LETTER_SA: char = 'ᠰ';
    /// \u{1831}: 'ᠱ'
    pub const LETTER_SHA: char = 'ᠱ';
    /// \u{1832}: 'ᠲ'
    pub const LETTER_TA: char = 'ᠲ';
    /// \u{1833}: 'ᠳ'
    pub const LETTER_DA: char = 'ᠳ';
    /// \u{1834}: 'ᠴ'
    pub const LETTER_CHA: char = 'ᠴ';
    /// \u{1835}: 'ᠵ'
    pub const LETTER_JA: char = 'ᠵ';
    /// \u{1836}: 'ᠶ'
    pub const LETTER_YA: char = 'ᠶ';
    /// \u{1837}: 'ᠷ'
    pub const LETTER_RA: char = 'ᠷ';
    /// \u{1838}: 'ᠸ'
    pub const LETTER_WA: char = 'ᠸ';
    /// \u{1839}: 'ᠹ'
    pub const LETTER_FA: char = 'ᠹ';
    /// \u{183a}: 'ᠺ'
    pub const LETTER_KA: char = 'ᠺ';
    /// \u{183b}: 'ᠻ'
    pub const LETTER_KHA: char = 'ᠻ';
    /// \u{183c}: 'ᠼ'
    pub const LETTER_TSA: char = 'ᠼ';
    /// \u{183d}: 'ᠽ'
    pub const LETTER_ZA: char = 'ᠽ';
    /// \u{183e}: 'ᠾ'
    pub const LETTER_HAA: char = 'ᠾ';
    /// \u{183f}: 'ᠿ'
    pub const LETTER_ZRA: char = 'ᠿ';
    /// \u{1840}: 'ᡀ'
    pub const LETTER_LHA: char = 'ᡀ';
    /// \u{1841}: 'ᡁ'
    pub const LETTER_ZHI: char = 'ᡁ';
    /// \u{1842}: 'ᡂ'
    pub const LETTER_CHI: char = 'ᡂ';
    /// \u{1843}: 'ᡃ'
    pub const LETTER_TODO_LONG_VOWEL_SIGN: char = 'ᡃ';
    /// \u{1844}: 'ᡄ'
    pub const LETTER_TODO_E: char = 'ᡄ';
    /// \u{1845}: 'ᡅ'
    pub const LETTER_TODO_I: char = 'ᡅ';
    /// \u{1846}: 'ᡆ'
    pub const LETTER_TODO_O: char = 'ᡆ';
    /// \u{1847}: 'ᡇ'
    pub const LETTER_TODO_U: char = 'ᡇ';
    /// \u{1848}: 'ᡈ'
    pub const LETTER_TODO_OE: char = 'ᡈ';
    /// \u{1849}: 'ᡉ'
    pub const LETTER_TODO_UE: char = 'ᡉ';
    /// \u{184a}: 'ᡊ'
    pub const LETTER_TODO_ANG: char = 'ᡊ';
    /// \u{184b}: 'ᡋ'
    pub const LETTER_TODO_BA: char = 'ᡋ';
    /// \u{184c}: 'ᡌ'
    pub const LETTER_TODO_PA: char = 'ᡌ';
    /// \u{184d}: 'ᡍ'
    pub const LETTER_TODO_QA: char = 'ᡍ';
    /// \u{184e}: 'ᡎ'
    pub const LETTER_TODO_GA: char = 'ᡎ';
    /// \u{184f}: 'ᡏ'
    pub const LETTER_TODO_MA: char = 'ᡏ';
    /// \u{1850}: 'ᡐ'
    pub const LETTER_TODO_TA: char = 'ᡐ';
    /// \u{1851}: 'ᡑ'
    pub const LETTER_TODO_DA: char = 'ᡑ';
    /// \u{1852}: 'ᡒ'
    pub const LETTER_TODO_CHA: char = 'ᡒ';
    /// \u{1853}: 'ᡓ'
    pub const LETTER_TODO_JA: char = 'ᡓ';
    /// \u{1854}: 'ᡔ'
    pub const LETTER_TODO_TSA: char = 'ᡔ';
    /// \u{1855}: 'ᡕ'
    pub const LETTER_TODO_YA: char = 'ᡕ';
    /// \u{1856}: 'ᡖ'
    pub const LETTER_TODO_WA: char = 'ᡖ';
    /// \u{1857}: 'ᡗ'
    pub const LETTER_TODO_KA: char = 'ᡗ';
    /// \u{1858}: 'ᡘ'
    pub const LETTER_TODO_GAA: char = 'ᡘ';
    /// \u{1859}: 'ᡙ'
    pub const LETTER_TODO_HAA: char = 'ᡙ';
    /// \u{185a}: 'ᡚ'
    pub const LETTER_TODO_JIA: char = 'ᡚ';
    /// \u{185b}: 'ᡛ'
    pub const LETTER_TODO_NIA: char = 'ᡛ';
    /// \u{185c}: 'ᡜ'
    pub const LETTER_TODO_DZA: char = 'ᡜ';
    /// \u{185d}: 'ᡝ'
    pub const LETTER_SIBE_E: char = 'ᡝ';
    /// \u{185e}: 'ᡞ'
    pub const LETTER_SIBE_I: char = 'ᡞ';
    /// \u{185f}: 'ᡟ'
    pub const LETTER_SIBE_IY: char = 'ᡟ';
    /// \u{1860}: 'ᡠ'
    pub const LETTER_SIBE_UE: char = 'ᡠ';
    /// \u{1861}: 'ᡡ'
    pub const LETTER_SIBE_U: char = 'ᡡ';
    /// \u{1862}: 'ᡢ'
    pub const LETTER_SIBE_ANG: char = 'ᡢ';
    /// \u{1863}: 'ᡣ'
    pub const LETTER_SIBE_KA: char = 'ᡣ';
    /// \u{1864}: 'ᡤ'
    pub const LETTER_SIBE_GA: char = 'ᡤ';
    /// \u{1865}: 'ᡥ'
    pub const LETTER_SIBE_HA: char = 'ᡥ';
    /// \u{1866}: 'ᡦ'
    pub const LETTER_SIBE_PA: char = 'ᡦ';
    /// \u{1867}: 'ᡧ'
    pub const LETTER_SIBE_SHA: char = 'ᡧ';
    /// \u{1868}: 'ᡨ'
    pub const LETTER_SIBE_TA: char = 'ᡨ';
    /// \u{1869}: 'ᡩ'
    pub const LETTER_SIBE_DA: char = 'ᡩ';
    /// \u{186a}: 'ᡪ'
    pub const LETTER_SIBE_JA: char = 'ᡪ';
    /// \u{186b}: 'ᡫ'
    pub const LETTER_SIBE_FA: char = 'ᡫ';
    /// \u{186c}: 'ᡬ'
    pub const LETTER_SIBE_GAA: char = 'ᡬ';
    /// \u{186d}: 'ᡭ'
    pub const LETTER_SIBE_HAA: char = 'ᡭ';
    /// \u{186e}: 'ᡮ'
    pub const LETTER_SIBE_TSA: char = 'ᡮ';
    /// \u{186f}: 'ᡯ'
    pub const LETTER_SIBE_ZA: char = 'ᡯ';
    /// \u{1870}: 'ᡰ'
    pub const LETTER_SIBE_RAA: char = 'ᡰ';
    /// \u{1871}: 'ᡱ'
    pub const LETTER_SIBE_CHA: char = 'ᡱ';
    /// \u{1872}: 'ᡲ'
    pub const LETTER_SIBE_ZHA: char = 'ᡲ';
    /// \u{1873}: 'ᡳ'
    pub const LETTER_MANCHU_I: char = 'ᡳ';
    /// \u{1874}: 'ᡴ'
    pub const LETTER_MANCHU_KA: char = 'ᡴ';
    /// \u{1875}: 'ᡵ'
    pub const LETTER_MANCHU_RA: char = 'ᡵ';
    /// \u{1876}: 'ᡶ'
    pub const LETTER_MANCHU_FA: char = 'ᡶ';
    /// \u{1877}: 'ᡷ'
    pub const LETTER_MANCHU_ZHA: char = 'ᡷ';
    /// \u{1878}: 'ᡸ'
    pub const LETTER_CHA_WITH_TWO_DOTS: char = 'ᡸ';
    /// \u{1880}: 'ᢀ'
    pub const LETTER_ALI_GALI_ANUSVARA_ONE: char = 'ᢀ';
    /// \u{1881}: 'ᢁ'
    pub const LETTER_ALI_GALI_VISARGA_ONE: char = 'ᢁ';
    /// \u{1882}: 'ᢂ'
    pub const LETTER_ALI_GALI_DAMARU: char = 'ᢂ';
    /// \u{1883}: 'ᢃ'
    pub const LETTER_ALI_GALI_UBADAMA: char = 'ᢃ';
    /// \u{1884}: 'ᢄ'
    pub const LETTER_ALI_GALI_INVERTED_UBADAMA: char = 'ᢄ';
    /// \u{1885}: 'ᢅ'
    pub const LETTER_ALI_GALI_BALUDA: char = 'ᢅ';
    /// \u{1886}: 'ᢆ'
    pub const LETTER_ALI_GALI_THREE_BALUDA: char = 'ᢆ';
    /// \u{1887}: 'ᢇ'
    pub const LETTER_ALI_GALI_A: char = 'ᢇ';
    /// \u{1888}: 'ᢈ'
    pub const LETTER_ALI_GALI_I: char = 'ᢈ';
    /// \u{1889}: 'ᢉ'
    pub const LETTER_ALI_GALI_KA: char = 'ᢉ';
    /// \u{188a}: 'ᢊ'
    pub const LETTER_ALI_GALI_NGA: char = 'ᢊ';
    /// \u{188b}: 'ᢋ'
    pub const LETTER_ALI_GALI_CA: char = 'ᢋ';
    /// \u{188c}: 'ᢌ'
    pub const LETTER_ALI_GALI_TTA: char = 'ᢌ';
    /// \u{188d}: 'ᢍ'
    pub const LETTER_ALI_GALI_TTHA: char = 'ᢍ';
    /// \u{188e}: 'ᢎ'
    pub const LETTER_ALI_GALI_DDA: char = 'ᢎ';
    /// \u{188f}: 'ᢏ'
    pub const LETTER_ALI_GALI_NNA: char = 'ᢏ';
    /// \u{1890}: 'ᢐ'
    pub const LETTER_ALI_GALI_TA: char = 'ᢐ';
    /// \u{1891}: 'ᢑ'
    pub const LETTER_ALI_GALI_DA: char = 'ᢑ';
    /// \u{1892}: 'ᢒ'
    pub const LETTER_ALI_GALI_PA: char = 'ᢒ';
    /// \u{1893}: 'ᢓ'
    pub const LETTER_ALI_GALI_PHA: char = 'ᢓ';
    /// \u{1894}: 'ᢔ'
    pub const LETTER_ALI_GALI_SSA: char = 'ᢔ';
    /// \u{1895}: 'ᢕ'
    pub const LETTER_ALI_GALI_ZHA: char = 'ᢕ';
    /// \u{1896}: 'ᢖ'
    pub const LETTER_ALI_GALI_ZA: char = 'ᢖ';
    /// \u{1897}: 'ᢗ'
    pub const LETTER_ALI_GALI_AH: char = 'ᢗ';
    /// \u{1898}: 'ᢘ'
    pub const LETTER_TODO_ALI_GALI_TA: char = 'ᢘ';
    /// \u{1899}: 'ᢙ'
    pub const LETTER_TODO_ALI_GALI_ZHA: char = 'ᢙ';
    /// \u{189a}: 'ᢚ'
    pub const LETTER_MANCHU_ALI_GALI_GHA: char = 'ᢚ';
    /// \u{189b}: 'ᢛ'
    pub const LETTER_MANCHU_ALI_GALI_NGA: char = 'ᢛ';
    /// \u{189c}: 'ᢜ'
    pub const LETTER_MANCHU_ALI_GALI_CA: char = 'ᢜ';
    /// \u{189d}: 'ᢝ'
    pub const LETTER_MANCHU_ALI_GALI_JHA: char = 'ᢝ';
    /// \u{189e}: 'ᢞ'
    pub const LETTER_MANCHU_ALI_GALI_TTA: char = 'ᢞ';
    /// \u{189f}: 'ᢟ'
    pub const LETTER_MANCHU_ALI_GALI_DDHA: char = 'ᢟ';
    /// \u{18a0}: 'ᢠ'
    pub const LETTER_MANCHU_ALI_GALI_TA: char = 'ᢠ';
    /// \u{18a1}: 'ᢡ'
    pub const LETTER_MANCHU_ALI_GALI_DHA: char = 'ᢡ';
    /// \u{18a2}: 'ᢢ'
    pub const LETTER_MANCHU_ALI_GALI_SSA: char = 'ᢢ';
    /// \u{18a3}: 'ᢣ'
    pub const LETTER_MANCHU_ALI_GALI_CYA: char = 'ᢣ';
    /// \u{18a4}: 'ᢤ'
    pub const LETTER_MANCHU_ALI_GALI_ZHA: char = 'ᢤ';
    /// \u{18a5}: 'ᢥ'
    pub const LETTER_MANCHU_ALI_GALI_ZA: char = 'ᢥ';
    /// \u{18a6}: 'ᢦ'
    pub const LETTER_ALI_GALI_HALF_U: char = 'ᢦ';
    /// \u{18a7}: 'ᢧ'
    pub const LETTER_ALI_GALI_HALF_YA: char = 'ᢧ';
    /// \u{18a8}: 'ᢨ'
    pub const LETTER_MANCHU_ALI_GALI_BHA: char = 'ᢨ';
    /// \u{18a9}: 'ᢩ'
    pub const LETTER_ALI_GALI_DAGALGA: char = 'ᢩ';
    /// \u{18aa}: 'ᢪ'
    pub const LETTER_MANCHU_ALI_GALI_LHA: char = 'ᢪ';
}

/// An enum to represent all characters in the Mongolian block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Mongolian {
    /// \u{1800}: '᠀'
    Birga,
    /// \u{1801}: '᠁'
    Ellipsis,
    /// \u{1802}: '᠂'
    Comma,
    /// \u{1803}: '᠃'
    FullStop,
    /// \u{1804}: '᠄'
    Colon,
    /// \u{1805}: '᠅'
    FourDots,
    /// \u{1806}: '᠆'
    TodoSoftHyphen,
    /// \u{1807}: '᠇'
    SibeSyllableBoundaryMarker,
    /// \u{1808}: '᠈'
    ManchuComma,
    /// \u{1809}: '᠉'
    ManchuFullStop,
    /// \u{180a}: '᠊'
    Nirugu,
    /// \u{180b}: '᠋'
    FreeVariationSelectorOne,
    /// \u{180c}: '᠌'
    FreeVariationSelectorTwo,
    /// \u{180d}: '᠍'
    FreeVariationSelectorThree,
    /// \u{180e}: '᠎'
    VowelSeparator,
    /// \u{1810}: '᠐'
    DigitZero,
    /// \u{1811}: '᠑'
    DigitOne,
    /// \u{1812}: '᠒'
    DigitTwo,
    /// \u{1813}: '᠓'
    DigitThree,
    /// \u{1814}: '᠔'
    DigitFour,
    /// \u{1815}: '᠕'
    DigitFive,
    /// \u{1816}: '᠖'
    DigitSix,
    /// \u{1817}: '᠗'
    DigitSeven,
    /// \u{1818}: '᠘'
    DigitEight,
    /// \u{1819}: '᠙'
    DigitNine,
    /// \u{1820}: 'ᠠ'
    LetterA,
    /// \u{1821}: 'ᠡ'
    LetterE,
    /// \u{1822}: 'ᠢ'
    LetterI,
    /// \u{1823}: 'ᠣ'
    LetterO,
    /// \u{1824}: 'ᠤ'
    LetterU,
    /// \u{1825}: 'ᠥ'
    LetterOe,
    /// \u{1826}: 'ᠦ'
    LetterUe,
    /// \u{1827}: 'ᠧ'
    LetterEe,
    /// \u{1828}: 'ᠨ'
    LetterNa,
    /// \u{1829}: 'ᠩ'
    LetterAng,
    /// \u{182a}: 'ᠪ'
    LetterBa,
    /// \u{182b}: 'ᠫ'
    LetterPa,
    /// \u{182c}: 'ᠬ'
    LetterQa,
    /// \u{182d}: 'ᠭ'
    LetterGa,
    /// \u{182e}: 'ᠮ'
    LetterMa,
    /// \u{182f}: 'ᠯ'
    LetterLa,
    /// \u{1830}: 'ᠰ'
    LetterSa,
    /// \u{1831}: 'ᠱ'
    LetterSha,
    /// \u{1832}: 'ᠲ'
    LetterTa,
    /// \u{1833}: 'ᠳ'
    LetterDa,
    /// \u{1834}: 'ᠴ'
    LetterCha,
    /// \u{1835}: 'ᠵ'
    LetterJa,
    /// \u{1836}: 'ᠶ'
    LetterYa,
    /// \u{1837}: 'ᠷ'
    LetterRa,
    /// \u{1838}: 'ᠸ'
    LetterWa,
    /// \u{1839}: 'ᠹ'
    LetterFa,
    /// \u{183a}: 'ᠺ'
    LetterKa,
    /// \u{183b}: 'ᠻ'
    LetterKha,
    /// \u{183c}: 'ᠼ'
    LetterTsa,
    /// \u{183d}: 'ᠽ'
    LetterZa,
    /// \u{183e}: 'ᠾ'
    LetterHaa,
    /// \u{183f}: 'ᠿ'
    LetterZra,
    /// \u{1840}: 'ᡀ'
    LetterLha,
    /// \u{1841}: 'ᡁ'
    LetterZhi,
    /// \u{1842}: 'ᡂ'
    LetterChi,
    /// \u{1843}: 'ᡃ'
    LetterTodoLongVowelSign,
    /// \u{1844}: 'ᡄ'
    LetterTodoE,
    /// \u{1845}: 'ᡅ'
    LetterTodoI,
    /// \u{1846}: 'ᡆ'
    LetterTodoO,
    /// \u{1847}: 'ᡇ'
    LetterTodoU,
    /// \u{1848}: 'ᡈ'
    LetterTodoOe,
    /// \u{1849}: 'ᡉ'
    LetterTodoUe,
    /// \u{184a}: 'ᡊ'
    LetterTodoAng,
    /// \u{184b}: 'ᡋ'
    LetterTodoBa,
    /// \u{184c}: 'ᡌ'
    LetterTodoPa,
    /// \u{184d}: 'ᡍ'
    LetterTodoQa,
    /// \u{184e}: 'ᡎ'
    LetterTodoGa,
    /// \u{184f}: 'ᡏ'
    LetterTodoMa,
    /// \u{1850}: 'ᡐ'
    LetterTodoTa,
    /// \u{1851}: 'ᡑ'
    LetterTodoDa,
    /// \u{1852}: 'ᡒ'
    LetterTodoCha,
    /// \u{1853}: 'ᡓ'
    LetterTodoJa,
    /// \u{1854}: 'ᡔ'
    LetterTodoTsa,
    /// \u{1855}: 'ᡕ'
    LetterTodoYa,
    /// \u{1856}: 'ᡖ'
    LetterTodoWa,
    /// \u{1857}: 'ᡗ'
    LetterTodoKa,
    /// \u{1858}: 'ᡘ'
    LetterTodoGaa,
    /// \u{1859}: 'ᡙ'
    LetterTodoHaa,
    /// \u{185a}: 'ᡚ'
    LetterTodoJia,
    /// \u{185b}: 'ᡛ'
    LetterTodoNia,
    /// \u{185c}: 'ᡜ'
    LetterTodoDza,
    /// \u{185d}: 'ᡝ'
    LetterSibeE,
    /// \u{185e}: 'ᡞ'
    LetterSibeI,
    /// \u{185f}: 'ᡟ'
    LetterSibeIy,
    /// \u{1860}: 'ᡠ'
    LetterSibeUe,
    /// \u{1861}: 'ᡡ'
    LetterSibeU,
    /// \u{1862}: 'ᡢ'
    LetterSibeAng,
    /// \u{1863}: 'ᡣ'
    LetterSibeKa,
    /// \u{1864}: 'ᡤ'
    LetterSibeGa,
    /// \u{1865}: 'ᡥ'
    LetterSibeHa,
    /// \u{1866}: 'ᡦ'
    LetterSibePa,
    /// \u{1867}: 'ᡧ'
    LetterSibeSha,
    /// \u{1868}: 'ᡨ'
    LetterSibeTa,
    /// \u{1869}: 'ᡩ'
    LetterSibeDa,
    /// \u{186a}: 'ᡪ'
    LetterSibeJa,
    /// \u{186b}: 'ᡫ'
    LetterSibeFa,
    /// \u{186c}: 'ᡬ'
    LetterSibeGaa,
    /// \u{186d}: 'ᡭ'
    LetterSibeHaa,
    /// \u{186e}: 'ᡮ'
    LetterSibeTsa,
    /// \u{186f}: 'ᡯ'
    LetterSibeZa,
    /// \u{1870}: 'ᡰ'
    LetterSibeRaa,
    /// \u{1871}: 'ᡱ'
    LetterSibeCha,
    /// \u{1872}: 'ᡲ'
    LetterSibeZha,
    /// \u{1873}: 'ᡳ'
    LetterManchuI,
    /// \u{1874}: 'ᡴ'
    LetterManchuKa,
    /// \u{1875}: 'ᡵ'
    LetterManchuRa,
    /// \u{1876}: 'ᡶ'
    LetterManchuFa,
    /// \u{1877}: 'ᡷ'
    LetterManchuZha,
    /// \u{1878}: 'ᡸ'
    LetterChaWithTwoDots,
    /// \u{1880}: 'ᢀ'
    LetterAliGaliAnusvaraOne,
    /// \u{1881}: 'ᢁ'
    LetterAliGaliVisargaOne,
    /// \u{1882}: 'ᢂ'
    LetterAliGaliDamaru,
    /// \u{1883}: 'ᢃ'
    LetterAliGaliUbadama,
    /// \u{1884}: 'ᢄ'
    LetterAliGaliInvertedUbadama,
    /// \u{1885}: 'ᢅ'
    LetterAliGaliBaluda,
    /// \u{1886}: 'ᢆ'
    LetterAliGaliThreeBaluda,
    /// \u{1887}: 'ᢇ'
    LetterAliGaliA,
    /// \u{1888}: 'ᢈ'
    LetterAliGaliI,
    /// \u{1889}: 'ᢉ'
    LetterAliGaliKa,
    /// \u{188a}: 'ᢊ'
    LetterAliGaliNga,
    /// \u{188b}: 'ᢋ'
    LetterAliGaliCa,
    /// \u{188c}: 'ᢌ'
    LetterAliGaliTta,
    /// \u{188d}: 'ᢍ'
    LetterAliGaliTtha,
    /// \u{188e}: 'ᢎ'
    LetterAliGaliDda,
    /// \u{188f}: 'ᢏ'
    LetterAliGaliNna,
    /// \u{1890}: 'ᢐ'
    LetterAliGaliTa,
    /// \u{1891}: 'ᢑ'
    LetterAliGaliDa,
    /// \u{1892}: 'ᢒ'
    LetterAliGaliPa,
    /// \u{1893}: 'ᢓ'
    LetterAliGaliPha,
    /// \u{1894}: 'ᢔ'
    LetterAliGaliSsa,
    /// \u{1895}: 'ᢕ'
    LetterAliGaliZha,
    /// \u{1896}: 'ᢖ'
    LetterAliGaliZa,
    /// \u{1897}: 'ᢗ'
    LetterAliGaliAh,
    /// \u{1898}: 'ᢘ'
    LetterTodoAliGaliTa,
    /// \u{1899}: 'ᢙ'
    LetterTodoAliGaliZha,
    /// \u{189a}: 'ᢚ'
    LetterManchuAliGaliGha,
    /// \u{189b}: 'ᢛ'
    LetterManchuAliGaliNga,
    /// \u{189c}: 'ᢜ'
    LetterManchuAliGaliCa,
    /// \u{189d}: 'ᢝ'
    LetterManchuAliGaliJha,
    /// \u{189e}: 'ᢞ'
    LetterManchuAliGaliTta,
    /// \u{189f}: 'ᢟ'
    LetterManchuAliGaliDdha,
    /// \u{18a0}: 'ᢠ'
    LetterManchuAliGaliTa,
    /// \u{18a1}: 'ᢡ'
    LetterManchuAliGaliDha,
    /// \u{18a2}: 'ᢢ'
    LetterManchuAliGaliSsa,
    /// \u{18a3}: 'ᢣ'
    LetterManchuAliGaliCya,
    /// \u{18a4}: 'ᢤ'
    LetterManchuAliGaliZha,
    /// \u{18a5}: 'ᢥ'
    LetterManchuAliGaliZa,
    /// \u{18a6}: 'ᢦ'
    LetterAliGaliHalfU,
    /// \u{18a7}: 'ᢧ'
    LetterAliGaliHalfYa,
    /// \u{18a8}: 'ᢨ'
    LetterManchuAliGaliBha,
    /// \u{18a9}: 'ᢩ'
    LetterAliGaliDagalga,
    /// \u{18aa}: 'ᢪ'
    LetterManchuAliGaliLha,
}

impl Into<char> for Mongolian {
    fn into(self) -> char {
        use constants::*;
        match self {
            Mongolian::Birga => BIRGA,
            Mongolian::Ellipsis => ELLIPSIS,
            Mongolian::Comma => COMMA,
            Mongolian::FullStop => FULL_STOP,
            Mongolian::Colon => COLON,
            Mongolian::FourDots => FOUR_DOTS,
            Mongolian::TodoSoftHyphen => TODO_SOFT_HYPHEN,
            Mongolian::SibeSyllableBoundaryMarker => SIBE_SYLLABLE_BOUNDARY_MARKER,
            Mongolian::ManchuComma => MANCHU_COMMA,
            Mongolian::ManchuFullStop => MANCHU_FULL_STOP,
            Mongolian::Nirugu => NIRUGU,
            Mongolian::FreeVariationSelectorOne => FREE_VARIATION_SELECTOR_ONE,
            Mongolian::FreeVariationSelectorTwo => FREE_VARIATION_SELECTOR_TWO,
            Mongolian::FreeVariationSelectorThree => FREE_VARIATION_SELECTOR_THREE,
            Mongolian::VowelSeparator => VOWEL_SEPARATOR,
            Mongolian::DigitZero => DIGIT_ZERO,
            Mongolian::DigitOne => DIGIT_ONE,
            Mongolian::DigitTwo => DIGIT_TWO,
            Mongolian::DigitThree => DIGIT_THREE,
            Mongolian::DigitFour => DIGIT_FOUR,
            Mongolian::DigitFive => DIGIT_FIVE,
            Mongolian::DigitSix => DIGIT_SIX,
            Mongolian::DigitSeven => DIGIT_SEVEN,
            Mongolian::DigitEight => DIGIT_EIGHT,
            Mongolian::DigitNine => DIGIT_NINE,
            Mongolian::LetterA => LETTER_A,
            Mongolian::LetterE => LETTER_E,
            Mongolian::LetterI => LETTER_I,
            Mongolian::LetterO => LETTER_O,
            Mongolian::LetterU => LETTER_U,
            Mongolian::LetterOe => LETTER_OE,
            Mongolian::LetterUe => LETTER_UE,
            Mongolian::LetterEe => LETTER_EE,
            Mongolian::LetterNa => LETTER_NA,
            Mongolian::LetterAng => LETTER_ANG,
            Mongolian::LetterBa => LETTER_BA,
            Mongolian::LetterPa => LETTER_PA,
            Mongolian::LetterQa => LETTER_QA,
            Mongolian::LetterGa => LETTER_GA,
            Mongolian::LetterMa => LETTER_MA,
            Mongolian::LetterLa => LETTER_LA,
            Mongolian::LetterSa => LETTER_SA,
            Mongolian::LetterSha => LETTER_SHA,
            Mongolian::LetterTa => LETTER_TA,
            Mongolian::LetterDa => LETTER_DA,
            Mongolian::LetterCha => LETTER_CHA,
            Mongolian::LetterJa => LETTER_JA,
            Mongolian::LetterYa => LETTER_YA,
            Mongolian::LetterRa => LETTER_RA,
            Mongolian::LetterWa => LETTER_WA,
            Mongolian::LetterFa => LETTER_FA,
            Mongolian::LetterKa => LETTER_KA,
            Mongolian::LetterKha => LETTER_KHA,
            Mongolian::LetterTsa => LETTER_TSA,
            Mongolian::LetterZa => LETTER_ZA,
            Mongolian::LetterHaa => LETTER_HAA,
            Mongolian::LetterZra => LETTER_ZRA,
            Mongolian::LetterLha => LETTER_LHA,
            Mongolian::LetterZhi => LETTER_ZHI,
            Mongolian::LetterChi => LETTER_CHI,
            Mongolian::LetterTodoLongVowelSign => LETTER_TODO_LONG_VOWEL_SIGN,
            Mongolian::LetterTodoE => LETTER_TODO_E,
            Mongolian::LetterTodoI => LETTER_TODO_I,
            Mongolian::LetterTodoO => LETTER_TODO_O,
            Mongolian::LetterTodoU => LETTER_TODO_U,
            Mongolian::LetterTodoOe => LETTER_TODO_OE,
            Mongolian::LetterTodoUe => LETTER_TODO_UE,
            Mongolian::LetterTodoAng => LETTER_TODO_ANG,
            Mongolian::LetterTodoBa => LETTER_TODO_BA,
            Mongolian::LetterTodoPa => LETTER_TODO_PA,
            Mongolian::LetterTodoQa => LETTER_TODO_QA,
            Mongolian::LetterTodoGa => LETTER_TODO_GA,
            Mongolian::LetterTodoMa => LETTER_TODO_MA,
            Mongolian::LetterTodoTa => LETTER_TODO_TA,
            Mongolian::LetterTodoDa => LETTER_TODO_DA,
            Mongolian::LetterTodoCha => LETTER_TODO_CHA,
            Mongolian::LetterTodoJa => LETTER_TODO_JA,
            Mongolian::LetterTodoTsa => LETTER_TODO_TSA,
            Mongolian::LetterTodoYa => LETTER_TODO_YA,
            Mongolian::LetterTodoWa => LETTER_TODO_WA,
            Mongolian::LetterTodoKa => LETTER_TODO_KA,
            Mongolian::LetterTodoGaa => LETTER_TODO_GAA,
            Mongolian::LetterTodoHaa => LETTER_TODO_HAA,
            Mongolian::LetterTodoJia => LETTER_TODO_JIA,
            Mongolian::LetterTodoNia => LETTER_TODO_NIA,
            Mongolian::LetterTodoDza => LETTER_TODO_DZA,
            Mongolian::LetterSibeE => LETTER_SIBE_E,
            Mongolian::LetterSibeI => LETTER_SIBE_I,
            Mongolian::LetterSibeIy => LETTER_SIBE_IY,
            Mongolian::LetterSibeUe => LETTER_SIBE_UE,
            Mongolian::LetterSibeU => LETTER_SIBE_U,
            Mongolian::LetterSibeAng => LETTER_SIBE_ANG,
            Mongolian::LetterSibeKa => LETTER_SIBE_KA,
            Mongolian::LetterSibeGa => LETTER_SIBE_GA,
            Mongolian::LetterSibeHa => LETTER_SIBE_HA,
            Mongolian::LetterSibePa => LETTER_SIBE_PA,
            Mongolian::LetterSibeSha => LETTER_SIBE_SHA,
            Mongolian::LetterSibeTa => LETTER_SIBE_TA,
            Mongolian::LetterSibeDa => LETTER_SIBE_DA,
            Mongolian::LetterSibeJa => LETTER_SIBE_JA,
            Mongolian::LetterSibeFa => LETTER_SIBE_FA,
            Mongolian::LetterSibeGaa => LETTER_SIBE_GAA,
            Mongolian::LetterSibeHaa => LETTER_SIBE_HAA,
            Mongolian::LetterSibeTsa => LETTER_SIBE_TSA,
            Mongolian::LetterSibeZa => LETTER_SIBE_ZA,
            Mongolian::LetterSibeRaa => LETTER_SIBE_RAA,
            Mongolian::LetterSibeCha => LETTER_SIBE_CHA,
            Mongolian::LetterSibeZha => LETTER_SIBE_ZHA,
            Mongolian::LetterManchuI => LETTER_MANCHU_I,
            Mongolian::LetterManchuKa => LETTER_MANCHU_KA,
            Mongolian::LetterManchuRa => LETTER_MANCHU_RA,
            Mongolian::LetterManchuFa => LETTER_MANCHU_FA,
            Mongolian::LetterManchuZha => LETTER_MANCHU_ZHA,
            Mongolian::LetterChaWithTwoDots => LETTER_CHA_WITH_TWO_DOTS,
            Mongolian::LetterAliGaliAnusvaraOne => LETTER_ALI_GALI_ANUSVARA_ONE,
            Mongolian::LetterAliGaliVisargaOne => LETTER_ALI_GALI_VISARGA_ONE,
            Mongolian::LetterAliGaliDamaru => LETTER_ALI_GALI_DAMARU,
            Mongolian::LetterAliGaliUbadama => LETTER_ALI_GALI_UBADAMA,
            Mongolian::LetterAliGaliInvertedUbadama => LETTER_ALI_GALI_INVERTED_UBADAMA,
            Mongolian::LetterAliGaliBaluda => LETTER_ALI_GALI_BALUDA,
            Mongolian::LetterAliGaliThreeBaluda => LETTER_ALI_GALI_THREE_BALUDA,
            Mongolian::LetterAliGaliA => LETTER_ALI_GALI_A,
            Mongolian::LetterAliGaliI => LETTER_ALI_GALI_I,
            Mongolian::LetterAliGaliKa => LETTER_ALI_GALI_KA,
            Mongolian::LetterAliGaliNga => LETTER_ALI_GALI_NGA,
            Mongolian::LetterAliGaliCa => LETTER_ALI_GALI_CA,
            Mongolian::LetterAliGaliTta => LETTER_ALI_GALI_TTA,
            Mongolian::LetterAliGaliTtha => LETTER_ALI_GALI_TTHA,
            Mongolian::LetterAliGaliDda => LETTER_ALI_GALI_DDA,
            Mongolian::LetterAliGaliNna => LETTER_ALI_GALI_NNA,
            Mongolian::LetterAliGaliTa => LETTER_ALI_GALI_TA,
            Mongolian::LetterAliGaliDa => LETTER_ALI_GALI_DA,
            Mongolian::LetterAliGaliPa => LETTER_ALI_GALI_PA,
            Mongolian::LetterAliGaliPha => LETTER_ALI_GALI_PHA,
            Mongolian::LetterAliGaliSsa => LETTER_ALI_GALI_SSA,
            Mongolian::LetterAliGaliZha => LETTER_ALI_GALI_ZHA,
            Mongolian::LetterAliGaliZa => LETTER_ALI_GALI_ZA,
            Mongolian::LetterAliGaliAh => LETTER_ALI_GALI_AH,
            Mongolian::LetterTodoAliGaliTa => LETTER_TODO_ALI_GALI_TA,
            Mongolian::LetterTodoAliGaliZha => LETTER_TODO_ALI_GALI_ZHA,
            Mongolian::LetterManchuAliGaliGha => LETTER_MANCHU_ALI_GALI_GHA,
            Mongolian::LetterManchuAliGaliNga => LETTER_MANCHU_ALI_GALI_NGA,
            Mongolian::LetterManchuAliGaliCa => LETTER_MANCHU_ALI_GALI_CA,
            Mongolian::LetterManchuAliGaliJha => LETTER_MANCHU_ALI_GALI_JHA,
            Mongolian::LetterManchuAliGaliTta => LETTER_MANCHU_ALI_GALI_TTA,
            Mongolian::LetterManchuAliGaliDdha => LETTER_MANCHU_ALI_GALI_DDHA,
            Mongolian::LetterManchuAliGaliTa => LETTER_MANCHU_ALI_GALI_TA,
            Mongolian::LetterManchuAliGaliDha => LETTER_MANCHU_ALI_GALI_DHA,
            Mongolian::LetterManchuAliGaliSsa => LETTER_MANCHU_ALI_GALI_SSA,
            Mongolian::LetterManchuAliGaliCya => LETTER_MANCHU_ALI_GALI_CYA,
            Mongolian::LetterManchuAliGaliZha => LETTER_MANCHU_ALI_GALI_ZHA,
            Mongolian::LetterManchuAliGaliZa => LETTER_MANCHU_ALI_GALI_ZA,
            Mongolian::LetterAliGaliHalfU => LETTER_ALI_GALI_HALF_U,
            Mongolian::LetterAliGaliHalfYa => LETTER_ALI_GALI_HALF_YA,
            Mongolian::LetterManchuAliGaliBha => LETTER_MANCHU_ALI_GALI_BHA,
            Mongolian::LetterAliGaliDagalga => LETTER_ALI_GALI_DAGALGA,
            Mongolian::LetterManchuAliGaliLha => LETTER_MANCHU_ALI_GALI_LHA,
        }
    }
}

impl std::convert::TryFrom<char> for Mongolian {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            BIRGA => Ok(Mongolian::Birga),
            ELLIPSIS => Ok(Mongolian::Ellipsis),
            COMMA => Ok(Mongolian::Comma),
            FULL_STOP => Ok(Mongolian::FullStop),
            COLON => Ok(Mongolian::Colon),
            FOUR_DOTS => Ok(Mongolian::FourDots),
            TODO_SOFT_HYPHEN => Ok(Mongolian::TodoSoftHyphen),
            SIBE_SYLLABLE_BOUNDARY_MARKER => Ok(Mongolian::SibeSyllableBoundaryMarker),
            MANCHU_COMMA => Ok(Mongolian::ManchuComma),
            MANCHU_FULL_STOP => Ok(Mongolian::ManchuFullStop),
            NIRUGU => Ok(Mongolian::Nirugu),
            FREE_VARIATION_SELECTOR_ONE => Ok(Mongolian::FreeVariationSelectorOne),
            FREE_VARIATION_SELECTOR_TWO => Ok(Mongolian::FreeVariationSelectorTwo),
            FREE_VARIATION_SELECTOR_THREE => Ok(Mongolian::FreeVariationSelectorThree),
            VOWEL_SEPARATOR => Ok(Mongolian::VowelSeparator),
            DIGIT_ZERO => Ok(Mongolian::DigitZero),
            DIGIT_ONE => Ok(Mongolian::DigitOne),
            DIGIT_TWO => Ok(Mongolian::DigitTwo),
            DIGIT_THREE => Ok(Mongolian::DigitThree),
            DIGIT_FOUR => Ok(Mongolian::DigitFour),
            DIGIT_FIVE => Ok(Mongolian::DigitFive),
            DIGIT_SIX => Ok(Mongolian::DigitSix),
            DIGIT_SEVEN => Ok(Mongolian::DigitSeven),
            DIGIT_EIGHT => Ok(Mongolian::DigitEight),
            DIGIT_NINE => Ok(Mongolian::DigitNine),
            LETTER_A => Ok(Mongolian::LetterA),
            LETTER_E => Ok(Mongolian::LetterE),
            LETTER_I => Ok(Mongolian::LetterI),
            LETTER_O => Ok(Mongolian::LetterO),
            LETTER_U => Ok(Mongolian::LetterU),
            LETTER_OE => Ok(Mongolian::LetterOe),
            LETTER_UE => Ok(Mongolian::LetterUe),
            LETTER_EE => Ok(Mongolian::LetterEe),
            LETTER_NA => Ok(Mongolian::LetterNa),
            LETTER_ANG => Ok(Mongolian::LetterAng),
            LETTER_BA => Ok(Mongolian::LetterBa),
            LETTER_PA => Ok(Mongolian::LetterPa),
            LETTER_QA => Ok(Mongolian::LetterQa),
            LETTER_GA => Ok(Mongolian::LetterGa),
            LETTER_MA => Ok(Mongolian::LetterMa),
            LETTER_LA => Ok(Mongolian::LetterLa),
            LETTER_SA => Ok(Mongolian::LetterSa),
            LETTER_SHA => Ok(Mongolian::LetterSha),
            LETTER_TA => Ok(Mongolian::LetterTa),
            LETTER_DA => Ok(Mongolian::LetterDa),
            LETTER_CHA => Ok(Mongolian::LetterCha),
            LETTER_JA => Ok(Mongolian::LetterJa),
            LETTER_YA => Ok(Mongolian::LetterYa),
            LETTER_RA => Ok(Mongolian::LetterRa),
            LETTER_WA => Ok(Mongolian::LetterWa),
            LETTER_FA => Ok(Mongolian::LetterFa),
            LETTER_KA => Ok(Mongolian::LetterKa),
            LETTER_KHA => Ok(Mongolian::LetterKha),
            LETTER_TSA => Ok(Mongolian::LetterTsa),
            LETTER_ZA => Ok(Mongolian::LetterZa),
            LETTER_HAA => Ok(Mongolian::LetterHaa),
            LETTER_ZRA => Ok(Mongolian::LetterZra),
            LETTER_LHA => Ok(Mongolian::LetterLha),
            LETTER_ZHI => Ok(Mongolian::LetterZhi),
            LETTER_CHI => Ok(Mongolian::LetterChi),
            LETTER_TODO_LONG_VOWEL_SIGN => Ok(Mongolian::LetterTodoLongVowelSign),
            LETTER_TODO_E => Ok(Mongolian::LetterTodoE),
            LETTER_TODO_I => Ok(Mongolian::LetterTodoI),
            LETTER_TODO_O => Ok(Mongolian::LetterTodoO),
            LETTER_TODO_U => Ok(Mongolian::LetterTodoU),
            LETTER_TODO_OE => Ok(Mongolian::LetterTodoOe),
            LETTER_TODO_UE => Ok(Mongolian::LetterTodoUe),
            LETTER_TODO_ANG => Ok(Mongolian::LetterTodoAng),
            LETTER_TODO_BA => Ok(Mongolian::LetterTodoBa),
            LETTER_TODO_PA => Ok(Mongolian::LetterTodoPa),
            LETTER_TODO_QA => Ok(Mongolian::LetterTodoQa),
            LETTER_TODO_GA => Ok(Mongolian::LetterTodoGa),
            LETTER_TODO_MA => Ok(Mongolian::LetterTodoMa),
            LETTER_TODO_TA => Ok(Mongolian::LetterTodoTa),
            LETTER_TODO_DA => Ok(Mongolian::LetterTodoDa),
            LETTER_TODO_CHA => Ok(Mongolian::LetterTodoCha),
            LETTER_TODO_JA => Ok(Mongolian::LetterTodoJa),
            LETTER_TODO_TSA => Ok(Mongolian::LetterTodoTsa),
            LETTER_TODO_YA => Ok(Mongolian::LetterTodoYa),
            LETTER_TODO_WA => Ok(Mongolian::LetterTodoWa),
            LETTER_TODO_KA => Ok(Mongolian::LetterTodoKa),
            LETTER_TODO_GAA => Ok(Mongolian::LetterTodoGaa),
            LETTER_TODO_HAA => Ok(Mongolian::LetterTodoHaa),
            LETTER_TODO_JIA => Ok(Mongolian::LetterTodoJia),
            LETTER_TODO_NIA => Ok(Mongolian::LetterTodoNia),
            LETTER_TODO_DZA => Ok(Mongolian::LetterTodoDza),
            LETTER_SIBE_E => Ok(Mongolian::LetterSibeE),
            LETTER_SIBE_I => Ok(Mongolian::LetterSibeI),
            LETTER_SIBE_IY => Ok(Mongolian::LetterSibeIy),
            LETTER_SIBE_UE => Ok(Mongolian::LetterSibeUe),
            LETTER_SIBE_U => Ok(Mongolian::LetterSibeU),
            LETTER_SIBE_ANG => Ok(Mongolian::LetterSibeAng),
            LETTER_SIBE_KA => Ok(Mongolian::LetterSibeKa),
            LETTER_SIBE_GA => Ok(Mongolian::LetterSibeGa),
            LETTER_SIBE_HA => Ok(Mongolian::LetterSibeHa),
            LETTER_SIBE_PA => Ok(Mongolian::LetterSibePa),
            LETTER_SIBE_SHA => Ok(Mongolian::LetterSibeSha),
            LETTER_SIBE_TA => Ok(Mongolian::LetterSibeTa),
            LETTER_SIBE_DA => Ok(Mongolian::LetterSibeDa),
            LETTER_SIBE_JA => Ok(Mongolian::LetterSibeJa),
            LETTER_SIBE_FA => Ok(Mongolian::LetterSibeFa),
            LETTER_SIBE_GAA => Ok(Mongolian::LetterSibeGaa),
            LETTER_SIBE_HAA => Ok(Mongolian::LetterSibeHaa),
            LETTER_SIBE_TSA => Ok(Mongolian::LetterSibeTsa),
            LETTER_SIBE_ZA => Ok(Mongolian::LetterSibeZa),
            LETTER_SIBE_RAA => Ok(Mongolian::LetterSibeRaa),
            LETTER_SIBE_CHA => Ok(Mongolian::LetterSibeCha),
            LETTER_SIBE_ZHA => Ok(Mongolian::LetterSibeZha),
            LETTER_MANCHU_I => Ok(Mongolian::LetterManchuI),
            LETTER_MANCHU_KA => Ok(Mongolian::LetterManchuKa),
            LETTER_MANCHU_RA => Ok(Mongolian::LetterManchuRa),
            LETTER_MANCHU_FA => Ok(Mongolian::LetterManchuFa),
            LETTER_MANCHU_ZHA => Ok(Mongolian::LetterManchuZha),
            LETTER_CHA_WITH_TWO_DOTS => Ok(Mongolian::LetterChaWithTwoDots),
            LETTER_ALI_GALI_ANUSVARA_ONE => Ok(Mongolian::LetterAliGaliAnusvaraOne),
            LETTER_ALI_GALI_VISARGA_ONE => Ok(Mongolian::LetterAliGaliVisargaOne),
            LETTER_ALI_GALI_DAMARU => Ok(Mongolian::LetterAliGaliDamaru),
            LETTER_ALI_GALI_UBADAMA => Ok(Mongolian::LetterAliGaliUbadama),
            LETTER_ALI_GALI_INVERTED_UBADAMA => Ok(Mongolian::LetterAliGaliInvertedUbadama),
            LETTER_ALI_GALI_BALUDA => Ok(Mongolian::LetterAliGaliBaluda),
            LETTER_ALI_GALI_THREE_BALUDA => Ok(Mongolian::LetterAliGaliThreeBaluda),
            LETTER_ALI_GALI_A => Ok(Mongolian::LetterAliGaliA),
            LETTER_ALI_GALI_I => Ok(Mongolian::LetterAliGaliI),
            LETTER_ALI_GALI_KA => Ok(Mongolian::LetterAliGaliKa),
            LETTER_ALI_GALI_NGA => Ok(Mongolian::LetterAliGaliNga),
            LETTER_ALI_GALI_CA => Ok(Mongolian::LetterAliGaliCa),
            LETTER_ALI_GALI_TTA => Ok(Mongolian::LetterAliGaliTta),
            LETTER_ALI_GALI_TTHA => Ok(Mongolian::LetterAliGaliTtha),
            LETTER_ALI_GALI_DDA => Ok(Mongolian::LetterAliGaliDda),
            LETTER_ALI_GALI_NNA => Ok(Mongolian::LetterAliGaliNna),
            LETTER_ALI_GALI_TA => Ok(Mongolian::LetterAliGaliTa),
            LETTER_ALI_GALI_DA => Ok(Mongolian::LetterAliGaliDa),
            LETTER_ALI_GALI_PA => Ok(Mongolian::LetterAliGaliPa),
            LETTER_ALI_GALI_PHA => Ok(Mongolian::LetterAliGaliPha),
            LETTER_ALI_GALI_SSA => Ok(Mongolian::LetterAliGaliSsa),
            LETTER_ALI_GALI_ZHA => Ok(Mongolian::LetterAliGaliZha),
            LETTER_ALI_GALI_ZA => Ok(Mongolian::LetterAliGaliZa),
            LETTER_ALI_GALI_AH => Ok(Mongolian::LetterAliGaliAh),
            LETTER_TODO_ALI_GALI_TA => Ok(Mongolian::LetterTodoAliGaliTa),
            LETTER_TODO_ALI_GALI_ZHA => Ok(Mongolian::LetterTodoAliGaliZha),
            LETTER_MANCHU_ALI_GALI_GHA => Ok(Mongolian::LetterManchuAliGaliGha),
            LETTER_MANCHU_ALI_GALI_NGA => Ok(Mongolian::LetterManchuAliGaliNga),
            LETTER_MANCHU_ALI_GALI_CA => Ok(Mongolian::LetterManchuAliGaliCa),
            LETTER_MANCHU_ALI_GALI_JHA => Ok(Mongolian::LetterManchuAliGaliJha),
            LETTER_MANCHU_ALI_GALI_TTA => Ok(Mongolian::LetterManchuAliGaliTta),
            LETTER_MANCHU_ALI_GALI_DDHA => Ok(Mongolian::LetterManchuAliGaliDdha),
            LETTER_MANCHU_ALI_GALI_TA => Ok(Mongolian::LetterManchuAliGaliTa),
            LETTER_MANCHU_ALI_GALI_DHA => Ok(Mongolian::LetterManchuAliGaliDha),
            LETTER_MANCHU_ALI_GALI_SSA => Ok(Mongolian::LetterManchuAliGaliSsa),
            LETTER_MANCHU_ALI_GALI_CYA => Ok(Mongolian::LetterManchuAliGaliCya),
            LETTER_MANCHU_ALI_GALI_ZHA => Ok(Mongolian::LetterManchuAliGaliZha),
            LETTER_MANCHU_ALI_GALI_ZA => Ok(Mongolian::LetterManchuAliGaliZa),
            LETTER_ALI_GALI_HALF_U => Ok(Mongolian::LetterAliGaliHalfU),
            LETTER_ALI_GALI_HALF_YA => Ok(Mongolian::LetterAliGaliHalfYa),
            LETTER_MANCHU_ALI_GALI_BHA => Ok(Mongolian::LetterManchuAliGaliBha),
            LETTER_ALI_GALI_DAGALGA => Ok(Mongolian::LetterAliGaliDagalga),
            LETTER_MANCHU_ALI_GALI_LHA => Ok(Mongolian::LetterManchuAliGaliLha),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Mongolian {
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

impl std::convert::TryFrom<u32> for Mongolian {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Mongolian {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Mongolian {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Mongolian::Birga
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Mongolian::Birga => "mongolian birga",
            Mongolian::Ellipsis => "mongolian ellipsis",
            Mongolian::Comma => "mongolian comma",
            Mongolian::FullStop => "mongolian full stop",
            Mongolian::Colon => "mongolian colon",
            Mongolian::FourDots => "mongolian four dots",
            Mongolian::TodoSoftHyphen => "mongolian todo soft hyphen",
            Mongolian::SibeSyllableBoundaryMarker => "mongolian sibe syllable boundary marker",
            Mongolian::ManchuComma => "mongolian manchu comma",
            Mongolian::ManchuFullStop => "mongolian manchu full stop",
            Mongolian::Nirugu => "mongolian nirugu",
            Mongolian::FreeVariationSelectorOne => "mongolian free variation selector one",
            Mongolian::FreeVariationSelectorTwo => "mongolian free variation selector two",
            Mongolian::FreeVariationSelectorThree => "mongolian free variation selector three",
            Mongolian::VowelSeparator => "mongolian vowel separator",
            Mongolian::DigitZero => "mongolian digit zero",
            Mongolian::DigitOne => "mongolian digit one",
            Mongolian::DigitTwo => "mongolian digit two",
            Mongolian::DigitThree => "mongolian digit three",
            Mongolian::DigitFour => "mongolian digit four",
            Mongolian::DigitFive => "mongolian digit five",
            Mongolian::DigitSix => "mongolian digit six",
            Mongolian::DigitSeven => "mongolian digit seven",
            Mongolian::DigitEight => "mongolian digit eight",
            Mongolian::DigitNine => "mongolian digit nine",
            Mongolian::LetterA => "mongolian letter a",
            Mongolian::LetterE => "mongolian letter e",
            Mongolian::LetterI => "mongolian letter i",
            Mongolian::LetterO => "mongolian letter o",
            Mongolian::LetterU => "mongolian letter u",
            Mongolian::LetterOe => "mongolian letter oe",
            Mongolian::LetterUe => "mongolian letter ue",
            Mongolian::LetterEe => "mongolian letter ee",
            Mongolian::LetterNa => "mongolian letter na",
            Mongolian::LetterAng => "mongolian letter ang",
            Mongolian::LetterBa => "mongolian letter ba",
            Mongolian::LetterPa => "mongolian letter pa",
            Mongolian::LetterQa => "mongolian letter qa",
            Mongolian::LetterGa => "mongolian letter ga",
            Mongolian::LetterMa => "mongolian letter ma",
            Mongolian::LetterLa => "mongolian letter la",
            Mongolian::LetterSa => "mongolian letter sa",
            Mongolian::LetterSha => "mongolian letter sha",
            Mongolian::LetterTa => "mongolian letter ta",
            Mongolian::LetterDa => "mongolian letter da",
            Mongolian::LetterCha => "mongolian letter cha",
            Mongolian::LetterJa => "mongolian letter ja",
            Mongolian::LetterYa => "mongolian letter ya",
            Mongolian::LetterRa => "mongolian letter ra",
            Mongolian::LetterWa => "mongolian letter wa",
            Mongolian::LetterFa => "mongolian letter fa",
            Mongolian::LetterKa => "mongolian letter ka",
            Mongolian::LetterKha => "mongolian letter kha",
            Mongolian::LetterTsa => "mongolian letter tsa",
            Mongolian::LetterZa => "mongolian letter za",
            Mongolian::LetterHaa => "mongolian letter haa",
            Mongolian::LetterZra => "mongolian letter zra",
            Mongolian::LetterLha => "mongolian letter lha",
            Mongolian::LetterZhi => "mongolian letter zhi",
            Mongolian::LetterChi => "mongolian letter chi",
            Mongolian::LetterTodoLongVowelSign => "mongolian letter todo long vowel sign",
            Mongolian::LetterTodoE => "mongolian letter todo e",
            Mongolian::LetterTodoI => "mongolian letter todo i",
            Mongolian::LetterTodoO => "mongolian letter todo o",
            Mongolian::LetterTodoU => "mongolian letter todo u",
            Mongolian::LetterTodoOe => "mongolian letter todo oe",
            Mongolian::LetterTodoUe => "mongolian letter todo ue",
            Mongolian::LetterTodoAng => "mongolian letter todo ang",
            Mongolian::LetterTodoBa => "mongolian letter todo ba",
            Mongolian::LetterTodoPa => "mongolian letter todo pa",
            Mongolian::LetterTodoQa => "mongolian letter todo qa",
            Mongolian::LetterTodoGa => "mongolian letter todo ga",
            Mongolian::LetterTodoMa => "mongolian letter todo ma",
            Mongolian::LetterTodoTa => "mongolian letter todo ta",
            Mongolian::LetterTodoDa => "mongolian letter todo da",
            Mongolian::LetterTodoCha => "mongolian letter todo cha",
            Mongolian::LetterTodoJa => "mongolian letter todo ja",
            Mongolian::LetterTodoTsa => "mongolian letter todo tsa",
            Mongolian::LetterTodoYa => "mongolian letter todo ya",
            Mongolian::LetterTodoWa => "mongolian letter todo wa",
            Mongolian::LetterTodoKa => "mongolian letter todo ka",
            Mongolian::LetterTodoGaa => "mongolian letter todo gaa",
            Mongolian::LetterTodoHaa => "mongolian letter todo haa",
            Mongolian::LetterTodoJia => "mongolian letter todo jia",
            Mongolian::LetterTodoNia => "mongolian letter todo nia",
            Mongolian::LetterTodoDza => "mongolian letter todo dza",
            Mongolian::LetterSibeE => "mongolian letter sibe e",
            Mongolian::LetterSibeI => "mongolian letter sibe i",
            Mongolian::LetterSibeIy => "mongolian letter sibe iy",
            Mongolian::LetterSibeUe => "mongolian letter sibe ue",
            Mongolian::LetterSibeU => "mongolian letter sibe u",
            Mongolian::LetterSibeAng => "mongolian letter sibe ang",
            Mongolian::LetterSibeKa => "mongolian letter sibe ka",
            Mongolian::LetterSibeGa => "mongolian letter sibe ga",
            Mongolian::LetterSibeHa => "mongolian letter sibe ha",
            Mongolian::LetterSibePa => "mongolian letter sibe pa",
            Mongolian::LetterSibeSha => "mongolian letter sibe sha",
            Mongolian::LetterSibeTa => "mongolian letter sibe ta",
            Mongolian::LetterSibeDa => "mongolian letter sibe da",
            Mongolian::LetterSibeJa => "mongolian letter sibe ja",
            Mongolian::LetterSibeFa => "mongolian letter sibe fa",
            Mongolian::LetterSibeGaa => "mongolian letter sibe gaa",
            Mongolian::LetterSibeHaa => "mongolian letter sibe haa",
            Mongolian::LetterSibeTsa => "mongolian letter sibe tsa",
            Mongolian::LetterSibeZa => "mongolian letter sibe za",
            Mongolian::LetterSibeRaa => "mongolian letter sibe raa",
            Mongolian::LetterSibeCha => "mongolian letter sibe cha",
            Mongolian::LetterSibeZha => "mongolian letter sibe zha",
            Mongolian::LetterManchuI => "mongolian letter manchu i",
            Mongolian::LetterManchuKa => "mongolian letter manchu ka",
            Mongolian::LetterManchuRa => "mongolian letter manchu ra",
            Mongolian::LetterManchuFa => "mongolian letter manchu fa",
            Mongolian::LetterManchuZha => "mongolian letter manchu zha",
            Mongolian::LetterChaWithTwoDots => "mongolian letter cha with two dots",
            Mongolian::LetterAliGaliAnusvaraOne => "mongolian letter ali gali anusvara one",
            Mongolian::LetterAliGaliVisargaOne => "mongolian letter ali gali visarga one",
            Mongolian::LetterAliGaliDamaru => "mongolian letter ali gali damaru",
            Mongolian::LetterAliGaliUbadama => "mongolian letter ali gali ubadama",
            Mongolian::LetterAliGaliInvertedUbadama => "mongolian letter ali gali inverted ubadama",
            Mongolian::LetterAliGaliBaluda => "mongolian letter ali gali baluda",
            Mongolian::LetterAliGaliThreeBaluda => "mongolian letter ali gali three baluda",
            Mongolian::LetterAliGaliA => "mongolian letter ali gali a",
            Mongolian::LetterAliGaliI => "mongolian letter ali gali i",
            Mongolian::LetterAliGaliKa => "mongolian letter ali gali ka",
            Mongolian::LetterAliGaliNga => "mongolian letter ali gali nga",
            Mongolian::LetterAliGaliCa => "mongolian letter ali gali ca",
            Mongolian::LetterAliGaliTta => "mongolian letter ali gali tta",
            Mongolian::LetterAliGaliTtha => "mongolian letter ali gali ttha",
            Mongolian::LetterAliGaliDda => "mongolian letter ali gali dda",
            Mongolian::LetterAliGaliNna => "mongolian letter ali gali nna",
            Mongolian::LetterAliGaliTa => "mongolian letter ali gali ta",
            Mongolian::LetterAliGaliDa => "mongolian letter ali gali da",
            Mongolian::LetterAliGaliPa => "mongolian letter ali gali pa",
            Mongolian::LetterAliGaliPha => "mongolian letter ali gali pha",
            Mongolian::LetterAliGaliSsa => "mongolian letter ali gali ssa",
            Mongolian::LetterAliGaliZha => "mongolian letter ali gali zha",
            Mongolian::LetterAliGaliZa => "mongolian letter ali gali za",
            Mongolian::LetterAliGaliAh => "mongolian letter ali gali ah",
            Mongolian::LetterTodoAliGaliTa => "mongolian letter todo ali gali ta",
            Mongolian::LetterTodoAliGaliZha => "mongolian letter todo ali gali zha",
            Mongolian::LetterManchuAliGaliGha => "mongolian letter manchu ali gali gha",
            Mongolian::LetterManchuAliGaliNga => "mongolian letter manchu ali gali nga",
            Mongolian::LetterManchuAliGaliCa => "mongolian letter manchu ali gali ca",
            Mongolian::LetterManchuAliGaliJha => "mongolian letter manchu ali gali jha",
            Mongolian::LetterManchuAliGaliTta => "mongolian letter manchu ali gali tta",
            Mongolian::LetterManchuAliGaliDdha => "mongolian letter manchu ali gali ddha",
            Mongolian::LetterManchuAliGaliTa => "mongolian letter manchu ali gali ta",
            Mongolian::LetterManchuAliGaliDha => "mongolian letter manchu ali gali dha",
            Mongolian::LetterManchuAliGaliSsa => "mongolian letter manchu ali gali ssa",
            Mongolian::LetterManchuAliGaliCya => "mongolian letter manchu ali gali cya",
            Mongolian::LetterManchuAliGaliZha => "mongolian letter manchu ali gali zha",
            Mongolian::LetterManchuAliGaliZa => "mongolian letter manchu ali gali za",
            Mongolian::LetterAliGaliHalfU => "mongolian letter ali gali half u",
            Mongolian::LetterAliGaliHalfYa => "mongolian letter ali gali half ya",
            Mongolian::LetterManchuAliGaliBha => "mongolian letter manchu ali gali bha",
            Mongolian::LetterAliGaliDagalga => "mongolian letter ali gali dagalga",
            Mongolian::LetterManchuAliGaliLha => "mongolian letter manchu ali gali lha",
        }
    }
}
