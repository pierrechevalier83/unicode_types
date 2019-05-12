
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
        match self {
            Khmer::LetterKa => 'ក',
            Khmer::LetterKha => 'ខ',
            Khmer::LetterKo => 'គ',
            Khmer::LetterKho => 'ឃ',
            Khmer::LetterNgo => 'ង',
            Khmer::LetterCa => 'ច',
            Khmer::LetterCha => 'ឆ',
            Khmer::LetterCo => 'ជ',
            Khmer::LetterCho => 'ឈ',
            Khmer::LetterNyo => 'ញ',
            Khmer::LetterDa => 'ដ',
            Khmer::LetterTtha => 'ឋ',
            Khmer::LetterDo => 'ឌ',
            Khmer::LetterTtho => 'ឍ',
            Khmer::LetterNno => 'ណ',
            Khmer::LetterTa => 'ត',
            Khmer::LetterTha => 'ថ',
            Khmer::LetterTo => 'ទ',
            Khmer::LetterTho => 'ធ',
            Khmer::LetterNo => 'ន',
            Khmer::LetterBa => 'ប',
            Khmer::LetterPha => 'ផ',
            Khmer::LetterPo => 'ព',
            Khmer::LetterPho => 'ភ',
            Khmer::LetterMo => 'ម',
            Khmer::LetterYo => 'យ',
            Khmer::LetterRo => 'រ',
            Khmer::LetterLo => 'ល',
            Khmer::LetterVo => 'វ',
            Khmer::LetterSha => 'ឝ',
            Khmer::LetterSso => 'ឞ',
            Khmer::LetterSa => 'ស',
            Khmer::LetterHa => 'ហ',
            Khmer::LetterLa => 'ឡ',
            Khmer::LetterQa => 'អ',
            Khmer::IndependentVowelQaq => 'ឣ',
            Khmer::IndependentVowelQaa => 'ឤ',
            Khmer::IndependentVowelQi => 'ឥ',
            Khmer::IndependentVowelQii => 'ឦ',
            Khmer::IndependentVowelQu => 'ឧ',
            Khmer::IndependentVowelQuk => 'ឨ',
            Khmer::IndependentVowelQuu => 'ឩ',
            Khmer::IndependentVowelQuuv => 'ឪ',
            Khmer::IndependentVowelRy => 'ឫ',
            Khmer::IndependentVowelRyy => 'ឬ',
            Khmer::IndependentVowelLy => 'ឭ',
            Khmer::IndependentVowelLyy => 'ឮ',
            Khmer::IndependentVowelQe => 'ឯ',
            Khmer::IndependentVowelQai => 'ឰ',
            Khmer::IndependentVowelQooTypeOne => 'ឱ',
            Khmer::IndependentVowelQooTypeTwo => 'ឲ',
            Khmer::IndependentVowelQau => 'ឳ',
            Khmer::VowelInherentAq => '឴',
            Khmer::VowelInherentAa => '឵',
            Khmer::VowelSignAa => 'ា',
            Khmer::VowelSignI => 'ិ',
            Khmer::VowelSignIi => 'ី',
            Khmer::VowelSignY => 'ឹ',
            Khmer::VowelSignYy => 'ឺ',
            Khmer::VowelSignU => 'ុ',
            Khmer::VowelSignUu => 'ូ',
            Khmer::VowelSignUa => 'ួ',
            Khmer::VowelSignOe => 'ើ',
            Khmer::VowelSignYa => 'ឿ',
            Khmer::VowelSignIe => 'ៀ',
            Khmer::VowelSignE => 'េ',
            Khmer::VowelSignAe => 'ែ',
            Khmer::VowelSignAi => 'ៃ',
            Khmer::VowelSignOo => 'ោ',
            Khmer::VowelSignAu => 'ៅ',
            Khmer::SignNikahit => 'ំ',
            Khmer::SignReahmuk => 'ះ',
            Khmer::SignYuukaleapintu => 'ៈ',
            Khmer::SignMuusikatoan => '៉',
            Khmer::SignTriisap => '៊',
            Khmer::SignBantoc => '់',
            Khmer::SignRobat => '៌',
            Khmer::SignToandakhiat => '៍',
            Khmer::SignKakabat => '៎',
            Khmer::SignAhsda => '៏',
            Khmer::SignSamyokSannya => '័',
            Khmer::SignViriam => '៑',
            Khmer::SignCoeng => '្',
            Khmer::SignBathamasat => '៓',
            Khmer::SignKhan => '។',
            Khmer::SignBariyoosan => '៕',
            Khmer::SignCamnucPiiKuuh => '៖',
            Khmer::SignLekToo => 'ៗ',
            Khmer::SignBeyyal => '៘',
            Khmer::SignPhnaekMuan => '៙',
            Khmer::SignKoomuut => '៚',
            Khmer::CurrencySymbolRiel => '៛',
            Khmer::SignAvakrahasanya => 'ៜ',
            Khmer::SignAtthacan => '៝',
            Khmer::DigitZero => '០',
            Khmer::DigitOne => '១',
            Khmer::DigitTwo => '២',
            Khmer::DigitThree => '៣',
            Khmer::DigitFour => '៤',
            Khmer::DigitFive => '៥',
            Khmer::DigitSix => '៦',
            Khmer::DigitSeven => '៧',
            Khmer::DigitEight => '៨',
            Khmer::DigitNine => '៩',
            Khmer::SymbolLekAttakSon => '៰',
            Khmer::SymbolLekAttakMuoy => '៱',
            Khmer::SymbolLekAttakPii => '៲',
            Khmer::SymbolLekAttakBei => '៳',
            Khmer::SymbolLekAttakBuon => '៴',
            Khmer::SymbolLekAttakPram => '៵',
            Khmer::SymbolLekAttakPramDashMuoy => '៶',
            Khmer::SymbolLekAttakPramDashPii => '៷',
            Khmer::SymbolLekAttakPramDashBei => '៸',
            Khmer::SymbolLekAttakPramDashBuon => '៹',
        }
    }
}

impl std::convert::TryFrom<char> for Khmer {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ក' => Ok(Khmer::LetterKa),
            'ខ' => Ok(Khmer::LetterKha),
            'គ' => Ok(Khmer::LetterKo),
            'ឃ' => Ok(Khmer::LetterKho),
            'ង' => Ok(Khmer::LetterNgo),
            'ច' => Ok(Khmer::LetterCa),
            'ឆ' => Ok(Khmer::LetterCha),
            'ជ' => Ok(Khmer::LetterCo),
            'ឈ' => Ok(Khmer::LetterCho),
            'ញ' => Ok(Khmer::LetterNyo),
            'ដ' => Ok(Khmer::LetterDa),
            'ឋ' => Ok(Khmer::LetterTtha),
            'ឌ' => Ok(Khmer::LetterDo),
            'ឍ' => Ok(Khmer::LetterTtho),
            'ណ' => Ok(Khmer::LetterNno),
            'ត' => Ok(Khmer::LetterTa),
            'ថ' => Ok(Khmer::LetterTha),
            'ទ' => Ok(Khmer::LetterTo),
            'ធ' => Ok(Khmer::LetterTho),
            'ន' => Ok(Khmer::LetterNo),
            'ប' => Ok(Khmer::LetterBa),
            'ផ' => Ok(Khmer::LetterPha),
            'ព' => Ok(Khmer::LetterPo),
            'ភ' => Ok(Khmer::LetterPho),
            'ម' => Ok(Khmer::LetterMo),
            'យ' => Ok(Khmer::LetterYo),
            'រ' => Ok(Khmer::LetterRo),
            'ល' => Ok(Khmer::LetterLo),
            'វ' => Ok(Khmer::LetterVo),
            'ឝ' => Ok(Khmer::LetterSha),
            'ឞ' => Ok(Khmer::LetterSso),
            'ស' => Ok(Khmer::LetterSa),
            'ហ' => Ok(Khmer::LetterHa),
            'ឡ' => Ok(Khmer::LetterLa),
            'អ' => Ok(Khmer::LetterQa),
            'ឣ' => Ok(Khmer::IndependentVowelQaq),
            'ឤ' => Ok(Khmer::IndependentVowelQaa),
            'ឥ' => Ok(Khmer::IndependentVowelQi),
            'ឦ' => Ok(Khmer::IndependentVowelQii),
            'ឧ' => Ok(Khmer::IndependentVowelQu),
            'ឨ' => Ok(Khmer::IndependentVowelQuk),
            'ឩ' => Ok(Khmer::IndependentVowelQuu),
            'ឪ' => Ok(Khmer::IndependentVowelQuuv),
            'ឫ' => Ok(Khmer::IndependentVowelRy),
            'ឬ' => Ok(Khmer::IndependentVowelRyy),
            'ឭ' => Ok(Khmer::IndependentVowelLy),
            'ឮ' => Ok(Khmer::IndependentVowelLyy),
            'ឯ' => Ok(Khmer::IndependentVowelQe),
            'ឰ' => Ok(Khmer::IndependentVowelQai),
            'ឱ' => Ok(Khmer::IndependentVowelQooTypeOne),
            'ឲ' => Ok(Khmer::IndependentVowelQooTypeTwo),
            'ឳ' => Ok(Khmer::IndependentVowelQau),
            '឴' => Ok(Khmer::VowelInherentAq),
            '឵' => Ok(Khmer::VowelInherentAa),
            'ា' => Ok(Khmer::VowelSignAa),
            'ិ' => Ok(Khmer::VowelSignI),
            'ី' => Ok(Khmer::VowelSignIi),
            'ឹ' => Ok(Khmer::VowelSignY),
            'ឺ' => Ok(Khmer::VowelSignYy),
            'ុ' => Ok(Khmer::VowelSignU),
            'ូ' => Ok(Khmer::VowelSignUu),
            'ួ' => Ok(Khmer::VowelSignUa),
            'ើ' => Ok(Khmer::VowelSignOe),
            'ឿ' => Ok(Khmer::VowelSignYa),
            'ៀ' => Ok(Khmer::VowelSignIe),
            'េ' => Ok(Khmer::VowelSignE),
            'ែ' => Ok(Khmer::VowelSignAe),
            'ៃ' => Ok(Khmer::VowelSignAi),
            'ោ' => Ok(Khmer::VowelSignOo),
            'ៅ' => Ok(Khmer::VowelSignAu),
            'ំ' => Ok(Khmer::SignNikahit),
            'ះ' => Ok(Khmer::SignReahmuk),
            'ៈ' => Ok(Khmer::SignYuukaleapintu),
            '៉' => Ok(Khmer::SignMuusikatoan),
            '៊' => Ok(Khmer::SignTriisap),
            '់' => Ok(Khmer::SignBantoc),
            '៌' => Ok(Khmer::SignRobat),
            '៍' => Ok(Khmer::SignToandakhiat),
            '៎' => Ok(Khmer::SignKakabat),
            '៏' => Ok(Khmer::SignAhsda),
            '័' => Ok(Khmer::SignSamyokSannya),
            '៑' => Ok(Khmer::SignViriam),
            '្' => Ok(Khmer::SignCoeng),
            '៓' => Ok(Khmer::SignBathamasat),
            '។' => Ok(Khmer::SignKhan),
            '៕' => Ok(Khmer::SignBariyoosan),
            '៖' => Ok(Khmer::SignCamnucPiiKuuh),
            'ៗ' => Ok(Khmer::SignLekToo),
            '៘' => Ok(Khmer::SignBeyyal),
            '៙' => Ok(Khmer::SignPhnaekMuan),
            '៚' => Ok(Khmer::SignKoomuut),
            '៛' => Ok(Khmer::CurrencySymbolRiel),
            'ៜ' => Ok(Khmer::SignAvakrahasanya),
            '៝' => Ok(Khmer::SignAtthacan),
            '០' => Ok(Khmer::DigitZero),
            '១' => Ok(Khmer::DigitOne),
            '២' => Ok(Khmer::DigitTwo),
            '៣' => Ok(Khmer::DigitThree),
            '៤' => Ok(Khmer::DigitFour),
            '៥' => Ok(Khmer::DigitFive),
            '៦' => Ok(Khmer::DigitSix),
            '៧' => Ok(Khmer::DigitSeven),
            '៨' => Ok(Khmer::DigitEight),
            '៩' => Ok(Khmer::DigitNine),
            '៰' => Ok(Khmer::SymbolLekAttakSon),
            '៱' => Ok(Khmer::SymbolLekAttakMuoy),
            '៲' => Ok(Khmer::SymbolLekAttakPii),
            '៳' => Ok(Khmer::SymbolLekAttakBei),
            '៴' => Ok(Khmer::SymbolLekAttakBuon),
            '៵' => Ok(Khmer::SymbolLekAttakPram),
            '៶' => Ok(Khmer::SymbolLekAttakPramDashMuoy),
            '៷' => Ok(Khmer::SymbolLekAttakPramDashPii),
            '៸' => Ok(Khmer::SymbolLekAttakPramDashBei),
            '៹' => Ok(Khmer::SymbolLekAttakPramDashBuon),
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

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Khmer{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
