
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
        match self {
            Mongolian::Birga => '᠀',
            Mongolian::Ellipsis => '᠁',
            Mongolian::Comma => '᠂',
            Mongolian::FullStop => '᠃',
            Mongolian::Colon => '᠄',
            Mongolian::FourDots => '᠅',
            Mongolian::TodoSoftHyphen => '᠆',
            Mongolian::SibeSyllableBoundaryMarker => '᠇',
            Mongolian::ManchuComma => '᠈',
            Mongolian::ManchuFullStop => '᠉',
            Mongolian::Nirugu => '᠊',
            Mongolian::FreeVariationSelectorOne => '᠋',
            Mongolian::FreeVariationSelectorTwo => '᠌',
            Mongolian::FreeVariationSelectorThree => '᠍',
            Mongolian::VowelSeparator => '᠎',
            Mongolian::DigitZero => '᠐',
            Mongolian::DigitOne => '᠑',
            Mongolian::DigitTwo => '᠒',
            Mongolian::DigitThree => '᠓',
            Mongolian::DigitFour => '᠔',
            Mongolian::DigitFive => '᠕',
            Mongolian::DigitSix => '᠖',
            Mongolian::DigitSeven => '᠗',
            Mongolian::DigitEight => '᠘',
            Mongolian::DigitNine => '᠙',
            Mongolian::LetterA => 'ᠠ',
            Mongolian::LetterE => 'ᠡ',
            Mongolian::LetterI => 'ᠢ',
            Mongolian::LetterO => 'ᠣ',
            Mongolian::LetterU => 'ᠤ',
            Mongolian::LetterOe => 'ᠥ',
            Mongolian::LetterUe => 'ᠦ',
            Mongolian::LetterEe => 'ᠧ',
            Mongolian::LetterNa => 'ᠨ',
            Mongolian::LetterAng => 'ᠩ',
            Mongolian::LetterBa => 'ᠪ',
            Mongolian::LetterPa => 'ᠫ',
            Mongolian::LetterQa => 'ᠬ',
            Mongolian::LetterGa => 'ᠭ',
            Mongolian::LetterMa => 'ᠮ',
            Mongolian::LetterLa => 'ᠯ',
            Mongolian::LetterSa => 'ᠰ',
            Mongolian::LetterSha => 'ᠱ',
            Mongolian::LetterTa => 'ᠲ',
            Mongolian::LetterDa => 'ᠳ',
            Mongolian::LetterCha => 'ᠴ',
            Mongolian::LetterJa => 'ᠵ',
            Mongolian::LetterYa => 'ᠶ',
            Mongolian::LetterRa => 'ᠷ',
            Mongolian::LetterWa => 'ᠸ',
            Mongolian::LetterFa => 'ᠹ',
            Mongolian::LetterKa => 'ᠺ',
            Mongolian::LetterKha => 'ᠻ',
            Mongolian::LetterTsa => 'ᠼ',
            Mongolian::LetterZa => 'ᠽ',
            Mongolian::LetterHaa => 'ᠾ',
            Mongolian::LetterZra => 'ᠿ',
            Mongolian::LetterLha => 'ᡀ',
            Mongolian::LetterZhi => 'ᡁ',
            Mongolian::LetterChi => 'ᡂ',
            Mongolian::LetterTodoLongVowelSign => 'ᡃ',
            Mongolian::LetterTodoE => 'ᡄ',
            Mongolian::LetterTodoI => 'ᡅ',
            Mongolian::LetterTodoO => 'ᡆ',
            Mongolian::LetterTodoU => 'ᡇ',
            Mongolian::LetterTodoOe => 'ᡈ',
            Mongolian::LetterTodoUe => 'ᡉ',
            Mongolian::LetterTodoAng => 'ᡊ',
            Mongolian::LetterTodoBa => 'ᡋ',
            Mongolian::LetterTodoPa => 'ᡌ',
            Mongolian::LetterTodoQa => 'ᡍ',
            Mongolian::LetterTodoGa => 'ᡎ',
            Mongolian::LetterTodoMa => 'ᡏ',
            Mongolian::LetterTodoTa => 'ᡐ',
            Mongolian::LetterTodoDa => 'ᡑ',
            Mongolian::LetterTodoCha => 'ᡒ',
            Mongolian::LetterTodoJa => 'ᡓ',
            Mongolian::LetterTodoTsa => 'ᡔ',
            Mongolian::LetterTodoYa => 'ᡕ',
            Mongolian::LetterTodoWa => 'ᡖ',
            Mongolian::LetterTodoKa => 'ᡗ',
            Mongolian::LetterTodoGaa => 'ᡘ',
            Mongolian::LetterTodoHaa => 'ᡙ',
            Mongolian::LetterTodoJia => 'ᡚ',
            Mongolian::LetterTodoNia => 'ᡛ',
            Mongolian::LetterTodoDza => 'ᡜ',
            Mongolian::LetterSibeE => 'ᡝ',
            Mongolian::LetterSibeI => 'ᡞ',
            Mongolian::LetterSibeIy => 'ᡟ',
            Mongolian::LetterSibeUe => 'ᡠ',
            Mongolian::LetterSibeU => 'ᡡ',
            Mongolian::LetterSibeAng => 'ᡢ',
            Mongolian::LetterSibeKa => 'ᡣ',
            Mongolian::LetterSibeGa => 'ᡤ',
            Mongolian::LetterSibeHa => 'ᡥ',
            Mongolian::LetterSibePa => 'ᡦ',
            Mongolian::LetterSibeSha => 'ᡧ',
            Mongolian::LetterSibeTa => 'ᡨ',
            Mongolian::LetterSibeDa => 'ᡩ',
            Mongolian::LetterSibeJa => 'ᡪ',
            Mongolian::LetterSibeFa => 'ᡫ',
            Mongolian::LetterSibeGaa => 'ᡬ',
            Mongolian::LetterSibeHaa => 'ᡭ',
            Mongolian::LetterSibeTsa => 'ᡮ',
            Mongolian::LetterSibeZa => 'ᡯ',
            Mongolian::LetterSibeRaa => 'ᡰ',
            Mongolian::LetterSibeCha => 'ᡱ',
            Mongolian::LetterSibeZha => 'ᡲ',
            Mongolian::LetterManchuI => 'ᡳ',
            Mongolian::LetterManchuKa => 'ᡴ',
            Mongolian::LetterManchuRa => 'ᡵ',
            Mongolian::LetterManchuFa => 'ᡶ',
            Mongolian::LetterManchuZha => 'ᡷ',
            Mongolian::LetterChaWithTwoDots => 'ᡸ',
            Mongolian::LetterAliGaliAnusvaraOne => 'ᢀ',
            Mongolian::LetterAliGaliVisargaOne => 'ᢁ',
            Mongolian::LetterAliGaliDamaru => 'ᢂ',
            Mongolian::LetterAliGaliUbadama => 'ᢃ',
            Mongolian::LetterAliGaliInvertedUbadama => 'ᢄ',
            Mongolian::LetterAliGaliBaluda => 'ᢅ',
            Mongolian::LetterAliGaliThreeBaluda => 'ᢆ',
            Mongolian::LetterAliGaliA => 'ᢇ',
            Mongolian::LetterAliGaliI => 'ᢈ',
            Mongolian::LetterAliGaliKa => 'ᢉ',
            Mongolian::LetterAliGaliNga => 'ᢊ',
            Mongolian::LetterAliGaliCa => 'ᢋ',
            Mongolian::LetterAliGaliTta => 'ᢌ',
            Mongolian::LetterAliGaliTtha => 'ᢍ',
            Mongolian::LetterAliGaliDda => 'ᢎ',
            Mongolian::LetterAliGaliNna => 'ᢏ',
            Mongolian::LetterAliGaliTa => 'ᢐ',
            Mongolian::LetterAliGaliDa => 'ᢑ',
            Mongolian::LetterAliGaliPa => 'ᢒ',
            Mongolian::LetterAliGaliPha => 'ᢓ',
            Mongolian::LetterAliGaliSsa => 'ᢔ',
            Mongolian::LetterAliGaliZha => 'ᢕ',
            Mongolian::LetterAliGaliZa => 'ᢖ',
            Mongolian::LetterAliGaliAh => 'ᢗ',
            Mongolian::LetterTodoAliGaliTa => 'ᢘ',
            Mongolian::LetterTodoAliGaliZha => 'ᢙ',
            Mongolian::LetterManchuAliGaliGha => 'ᢚ',
            Mongolian::LetterManchuAliGaliNga => 'ᢛ',
            Mongolian::LetterManchuAliGaliCa => 'ᢜ',
            Mongolian::LetterManchuAliGaliJha => 'ᢝ',
            Mongolian::LetterManchuAliGaliTta => 'ᢞ',
            Mongolian::LetterManchuAliGaliDdha => 'ᢟ',
            Mongolian::LetterManchuAliGaliTa => 'ᢠ',
            Mongolian::LetterManchuAliGaliDha => 'ᢡ',
            Mongolian::LetterManchuAliGaliSsa => 'ᢢ',
            Mongolian::LetterManchuAliGaliCya => 'ᢣ',
            Mongolian::LetterManchuAliGaliZha => 'ᢤ',
            Mongolian::LetterManchuAliGaliZa => 'ᢥ',
            Mongolian::LetterAliGaliHalfU => 'ᢦ',
            Mongolian::LetterAliGaliHalfYa => 'ᢧ',
            Mongolian::LetterManchuAliGaliBha => 'ᢨ',
            Mongolian::LetterAliGaliDagalga => 'ᢩ',
            Mongolian::LetterManchuAliGaliLha => 'ᢪ',
        }
    }
}

impl std::convert::TryFrom<char> for Mongolian {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '᠀' => Ok(Mongolian::Birga),
            '᠁' => Ok(Mongolian::Ellipsis),
            '᠂' => Ok(Mongolian::Comma),
            '᠃' => Ok(Mongolian::FullStop),
            '᠄' => Ok(Mongolian::Colon),
            '᠅' => Ok(Mongolian::FourDots),
            '᠆' => Ok(Mongolian::TodoSoftHyphen),
            '᠇' => Ok(Mongolian::SibeSyllableBoundaryMarker),
            '᠈' => Ok(Mongolian::ManchuComma),
            '᠉' => Ok(Mongolian::ManchuFullStop),
            '᠊' => Ok(Mongolian::Nirugu),
            '᠋' => Ok(Mongolian::FreeVariationSelectorOne),
            '᠌' => Ok(Mongolian::FreeVariationSelectorTwo),
            '᠍' => Ok(Mongolian::FreeVariationSelectorThree),
            '᠎' => Ok(Mongolian::VowelSeparator),
            '᠐' => Ok(Mongolian::DigitZero),
            '᠑' => Ok(Mongolian::DigitOne),
            '᠒' => Ok(Mongolian::DigitTwo),
            '᠓' => Ok(Mongolian::DigitThree),
            '᠔' => Ok(Mongolian::DigitFour),
            '᠕' => Ok(Mongolian::DigitFive),
            '᠖' => Ok(Mongolian::DigitSix),
            '᠗' => Ok(Mongolian::DigitSeven),
            '᠘' => Ok(Mongolian::DigitEight),
            '᠙' => Ok(Mongolian::DigitNine),
            'ᠠ' => Ok(Mongolian::LetterA),
            'ᠡ' => Ok(Mongolian::LetterE),
            'ᠢ' => Ok(Mongolian::LetterI),
            'ᠣ' => Ok(Mongolian::LetterO),
            'ᠤ' => Ok(Mongolian::LetterU),
            'ᠥ' => Ok(Mongolian::LetterOe),
            'ᠦ' => Ok(Mongolian::LetterUe),
            'ᠧ' => Ok(Mongolian::LetterEe),
            'ᠨ' => Ok(Mongolian::LetterNa),
            'ᠩ' => Ok(Mongolian::LetterAng),
            'ᠪ' => Ok(Mongolian::LetterBa),
            'ᠫ' => Ok(Mongolian::LetterPa),
            'ᠬ' => Ok(Mongolian::LetterQa),
            'ᠭ' => Ok(Mongolian::LetterGa),
            'ᠮ' => Ok(Mongolian::LetterMa),
            'ᠯ' => Ok(Mongolian::LetterLa),
            'ᠰ' => Ok(Mongolian::LetterSa),
            'ᠱ' => Ok(Mongolian::LetterSha),
            'ᠲ' => Ok(Mongolian::LetterTa),
            'ᠳ' => Ok(Mongolian::LetterDa),
            'ᠴ' => Ok(Mongolian::LetterCha),
            'ᠵ' => Ok(Mongolian::LetterJa),
            'ᠶ' => Ok(Mongolian::LetterYa),
            'ᠷ' => Ok(Mongolian::LetterRa),
            'ᠸ' => Ok(Mongolian::LetterWa),
            'ᠹ' => Ok(Mongolian::LetterFa),
            'ᠺ' => Ok(Mongolian::LetterKa),
            'ᠻ' => Ok(Mongolian::LetterKha),
            'ᠼ' => Ok(Mongolian::LetterTsa),
            'ᠽ' => Ok(Mongolian::LetterZa),
            'ᠾ' => Ok(Mongolian::LetterHaa),
            'ᠿ' => Ok(Mongolian::LetterZra),
            'ᡀ' => Ok(Mongolian::LetterLha),
            'ᡁ' => Ok(Mongolian::LetterZhi),
            'ᡂ' => Ok(Mongolian::LetterChi),
            'ᡃ' => Ok(Mongolian::LetterTodoLongVowelSign),
            'ᡄ' => Ok(Mongolian::LetterTodoE),
            'ᡅ' => Ok(Mongolian::LetterTodoI),
            'ᡆ' => Ok(Mongolian::LetterTodoO),
            'ᡇ' => Ok(Mongolian::LetterTodoU),
            'ᡈ' => Ok(Mongolian::LetterTodoOe),
            'ᡉ' => Ok(Mongolian::LetterTodoUe),
            'ᡊ' => Ok(Mongolian::LetterTodoAng),
            'ᡋ' => Ok(Mongolian::LetterTodoBa),
            'ᡌ' => Ok(Mongolian::LetterTodoPa),
            'ᡍ' => Ok(Mongolian::LetterTodoQa),
            'ᡎ' => Ok(Mongolian::LetterTodoGa),
            'ᡏ' => Ok(Mongolian::LetterTodoMa),
            'ᡐ' => Ok(Mongolian::LetterTodoTa),
            'ᡑ' => Ok(Mongolian::LetterTodoDa),
            'ᡒ' => Ok(Mongolian::LetterTodoCha),
            'ᡓ' => Ok(Mongolian::LetterTodoJa),
            'ᡔ' => Ok(Mongolian::LetterTodoTsa),
            'ᡕ' => Ok(Mongolian::LetterTodoYa),
            'ᡖ' => Ok(Mongolian::LetterTodoWa),
            'ᡗ' => Ok(Mongolian::LetterTodoKa),
            'ᡘ' => Ok(Mongolian::LetterTodoGaa),
            'ᡙ' => Ok(Mongolian::LetterTodoHaa),
            'ᡚ' => Ok(Mongolian::LetterTodoJia),
            'ᡛ' => Ok(Mongolian::LetterTodoNia),
            'ᡜ' => Ok(Mongolian::LetterTodoDza),
            'ᡝ' => Ok(Mongolian::LetterSibeE),
            'ᡞ' => Ok(Mongolian::LetterSibeI),
            'ᡟ' => Ok(Mongolian::LetterSibeIy),
            'ᡠ' => Ok(Mongolian::LetterSibeUe),
            'ᡡ' => Ok(Mongolian::LetterSibeU),
            'ᡢ' => Ok(Mongolian::LetterSibeAng),
            'ᡣ' => Ok(Mongolian::LetterSibeKa),
            'ᡤ' => Ok(Mongolian::LetterSibeGa),
            'ᡥ' => Ok(Mongolian::LetterSibeHa),
            'ᡦ' => Ok(Mongolian::LetterSibePa),
            'ᡧ' => Ok(Mongolian::LetterSibeSha),
            'ᡨ' => Ok(Mongolian::LetterSibeTa),
            'ᡩ' => Ok(Mongolian::LetterSibeDa),
            'ᡪ' => Ok(Mongolian::LetterSibeJa),
            'ᡫ' => Ok(Mongolian::LetterSibeFa),
            'ᡬ' => Ok(Mongolian::LetterSibeGaa),
            'ᡭ' => Ok(Mongolian::LetterSibeHaa),
            'ᡮ' => Ok(Mongolian::LetterSibeTsa),
            'ᡯ' => Ok(Mongolian::LetterSibeZa),
            'ᡰ' => Ok(Mongolian::LetterSibeRaa),
            'ᡱ' => Ok(Mongolian::LetterSibeCha),
            'ᡲ' => Ok(Mongolian::LetterSibeZha),
            'ᡳ' => Ok(Mongolian::LetterManchuI),
            'ᡴ' => Ok(Mongolian::LetterManchuKa),
            'ᡵ' => Ok(Mongolian::LetterManchuRa),
            'ᡶ' => Ok(Mongolian::LetterManchuFa),
            'ᡷ' => Ok(Mongolian::LetterManchuZha),
            'ᡸ' => Ok(Mongolian::LetterChaWithTwoDots),
            'ᢀ' => Ok(Mongolian::LetterAliGaliAnusvaraOne),
            'ᢁ' => Ok(Mongolian::LetterAliGaliVisargaOne),
            'ᢂ' => Ok(Mongolian::LetterAliGaliDamaru),
            'ᢃ' => Ok(Mongolian::LetterAliGaliUbadama),
            'ᢄ' => Ok(Mongolian::LetterAliGaliInvertedUbadama),
            'ᢅ' => Ok(Mongolian::LetterAliGaliBaluda),
            'ᢆ' => Ok(Mongolian::LetterAliGaliThreeBaluda),
            'ᢇ' => Ok(Mongolian::LetterAliGaliA),
            'ᢈ' => Ok(Mongolian::LetterAliGaliI),
            'ᢉ' => Ok(Mongolian::LetterAliGaliKa),
            'ᢊ' => Ok(Mongolian::LetterAliGaliNga),
            'ᢋ' => Ok(Mongolian::LetterAliGaliCa),
            'ᢌ' => Ok(Mongolian::LetterAliGaliTta),
            'ᢍ' => Ok(Mongolian::LetterAliGaliTtha),
            'ᢎ' => Ok(Mongolian::LetterAliGaliDda),
            'ᢏ' => Ok(Mongolian::LetterAliGaliNna),
            'ᢐ' => Ok(Mongolian::LetterAliGaliTa),
            'ᢑ' => Ok(Mongolian::LetterAliGaliDa),
            'ᢒ' => Ok(Mongolian::LetterAliGaliPa),
            'ᢓ' => Ok(Mongolian::LetterAliGaliPha),
            'ᢔ' => Ok(Mongolian::LetterAliGaliSsa),
            'ᢕ' => Ok(Mongolian::LetterAliGaliZha),
            'ᢖ' => Ok(Mongolian::LetterAliGaliZa),
            'ᢗ' => Ok(Mongolian::LetterAliGaliAh),
            'ᢘ' => Ok(Mongolian::LetterTodoAliGaliTa),
            'ᢙ' => Ok(Mongolian::LetterTodoAliGaliZha),
            'ᢚ' => Ok(Mongolian::LetterManchuAliGaliGha),
            'ᢛ' => Ok(Mongolian::LetterManchuAliGaliNga),
            'ᢜ' => Ok(Mongolian::LetterManchuAliGaliCa),
            'ᢝ' => Ok(Mongolian::LetterManchuAliGaliJha),
            'ᢞ' => Ok(Mongolian::LetterManchuAliGaliTta),
            'ᢟ' => Ok(Mongolian::LetterManchuAliGaliDdha),
            'ᢠ' => Ok(Mongolian::LetterManchuAliGaliTa),
            'ᢡ' => Ok(Mongolian::LetterManchuAliGaliDha),
            'ᢢ' => Ok(Mongolian::LetterManchuAliGaliSsa),
            'ᢣ' => Ok(Mongolian::LetterManchuAliGaliCya),
            'ᢤ' => Ok(Mongolian::LetterManchuAliGaliZha),
            'ᢥ' => Ok(Mongolian::LetterManchuAliGaliZa),
            'ᢦ' => Ok(Mongolian::LetterAliGaliHalfU),
            'ᢧ' => Ok(Mongolian::LetterAliGaliHalfYa),
            'ᢨ' => Ok(Mongolian::LetterManchuAliGaliBha),
            'ᢩ' => Ok(Mongolian::LetterAliGaliDagalga),
            'ᢪ' => Ok(Mongolian::LetterManchuAliGaliLha),
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

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Mongolian{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
