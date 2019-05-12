/// \u{16a0} → \u{16ff}
///
/// ᚠ ᚡ ᚢ ᚣ ᚤ ᚥ ᚦ ᚧ ᚨ ᚩ ᚪ ᚫ ᚬ ᚭ ᚮ ᚯ\
/// ᚰ ᚱ ᚲ ᚳ ᚴ ᚵ ᚶ ᚷ ᚸ ᚹ ᚺ ᚻ ᚼ ᚽ ᚾ ᚿ\
/// ᛀ ᛁ ᛂ ᛃ ᛄ ᛅ ᛆ ᛇ ᛈ ᛉ ᛊ ᛋ ᛌ ᛍ ᛎ ᛏ\
/// ᛐ ᛑ ᛒ ᛓ ᛔ ᛕ ᛖ ᛗ ᛘ ᛙ ᛚ ᛛ ᛜ ᛝ ᛞ ᛟ\
/// ᛠ ᛡ ᛢ ᛣ ᛤ ᛥ ᛦ ᛧ ᛨ ᛩ ᛪ ᛫ ᛬ ᛭ ᛮ ᛯ\
/// ᛰ ᛱ ᛲ ᛳ ᛴ ᛵ ᛶ ᛷ ᛸ\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{16a0}: 'ᚠ'
    pub const LETTER_FEHU_FEOH_FE_F: char = 'ᚠ';
    /// \u{16a1}: 'ᚡ'
    pub const LETTER_V: char = 'ᚡ';
    /// \u{16a2}: 'ᚢ'
    pub const LETTER_URUZ_UR_U: char = 'ᚢ';
    /// \u{16a3}: 'ᚣ'
    pub const LETTER_YR: char = 'ᚣ';
    /// \u{16a4}: 'ᚤ'
    pub const LETTER_Y: char = 'ᚤ';
    /// \u{16a5}: 'ᚥ'
    pub const LETTER_W: char = 'ᚥ';
    /// \u{16a6}: 'ᚦ'
    pub const LETTER_THURISAZ_THURS_THORN: char = 'ᚦ';
    /// \u{16a7}: 'ᚧ'
    pub const LETTER_ETH: char = 'ᚧ';
    /// \u{16a8}: 'ᚨ'
    pub const LETTER_ANSUZ_A: char = 'ᚨ';
    /// \u{16a9}: 'ᚩ'
    pub const LETTER_OS_O: char = 'ᚩ';
    /// \u{16aa}: 'ᚪ'
    pub const LETTER_AC_A: char = 'ᚪ';
    /// \u{16ab}: 'ᚫ'
    pub const LETTER_AESC: char = 'ᚫ';
    /// \u{16ac}: 'ᚬ'
    pub const LETTER_LONG_DASH_BRANCH_DASH_OSS_O: char = 'ᚬ';
    /// \u{16ad}: 'ᚭ'
    pub const LETTER_SHORT_DASH_TWIG_DASH_OSS_O: char = 'ᚭ';
    /// \u{16ae}: 'ᚮ'
    pub const LETTER_O: char = 'ᚮ';
    /// \u{16af}: 'ᚯ'
    pub const LETTER_OE: char = 'ᚯ';
    /// \u{16b0}: 'ᚰ'
    pub const LETTER_ON: char = 'ᚰ';
    /// \u{16b1}: 'ᚱ'
    pub const LETTER_RAIDO_RAD_REID_R: char = 'ᚱ';
    /// \u{16b2}: 'ᚲ'
    pub const LETTER_KAUNA: char = 'ᚲ';
    /// \u{16b3}: 'ᚳ'
    pub const LETTER_CEN: char = 'ᚳ';
    /// \u{16b4}: 'ᚴ'
    pub const LETTER_KAUN_K: char = 'ᚴ';
    /// \u{16b5}: 'ᚵ'
    pub const LETTER_G: char = 'ᚵ';
    /// \u{16b6}: 'ᚶ'
    pub const LETTER_ENG: char = 'ᚶ';
    /// \u{16b7}: 'ᚷ'
    pub const LETTER_GEBO_GYFU_G: char = 'ᚷ';
    /// \u{16b8}: 'ᚸ'
    pub const LETTER_GAR: char = 'ᚸ';
    /// \u{16b9}: 'ᚹ'
    pub const LETTER_WUNJO_WYNN_W: char = 'ᚹ';
    /// \u{16ba}: 'ᚺ'
    pub const LETTER_HAGLAZ_H: char = 'ᚺ';
    /// \u{16bb}: 'ᚻ'
    pub const LETTER_HAEGL_H: char = 'ᚻ';
    /// \u{16bc}: 'ᚼ'
    pub const LETTER_LONG_DASH_BRANCH_DASH_HAGALL_H: char = 'ᚼ';
    /// \u{16bd}: 'ᚽ'
    pub const LETTER_SHORT_DASH_TWIG_DASH_HAGALL_H: char = 'ᚽ';
    /// \u{16be}: 'ᚾ'
    pub const LETTER_NAUDIZ_NYD_NAUD_N: char = 'ᚾ';
    /// \u{16bf}: 'ᚿ'
    pub const LETTER_SHORT_DASH_TWIG_DASH_NAUD_N: char = 'ᚿ';
    /// \u{16c0}: 'ᛀ'
    pub const LETTER_DOTTED_DASH_N: char = 'ᛀ';
    /// \u{16c1}: 'ᛁ'
    pub const LETTER_ISAZ_IS_ISS_I: char = 'ᛁ';
    /// \u{16c2}: 'ᛂ'
    pub const LETTER_E: char = 'ᛂ';
    /// \u{16c3}: 'ᛃ'
    pub const LETTER_JERAN_J: char = 'ᛃ';
    /// \u{16c4}: 'ᛄ'
    pub const LETTER_GER: char = 'ᛄ';
    /// \u{16c5}: 'ᛅ'
    pub const LETTER_LONG_DASH_BRANCH_DASH_AR_AE: char = 'ᛅ';
    /// \u{16c6}: 'ᛆ'
    pub const LETTER_SHORT_DASH_TWIG_DASH_AR_A: char = 'ᛆ';
    /// \u{16c7}: 'ᛇ'
    pub const LETTER_IWAZ_EOH: char = 'ᛇ';
    /// \u{16c8}: 'ᛈ'
    pub const LETTER_PERTHO_PEORTH_P: char = 'ᛈ';
    /// \u{16c9}: 'ᛉ'
    pub const LETTER_ALGIZ_EOLHX: char = 'ᛉ';
    /// \u{16ca}: 'ᛊ'
    pub const LETTER_SOWILO_S: char = 'ᛊ';
    /// \u{16cb}: 'ᛋ'
    pub const LETTER_SIGEL_LONG_DASH_BRANCH_DASH_SOL_S: char = 'ᛋ';
    /// \u{16cc}: 'ᛌ'
    pub const LETTER_SHORT_DASH_TWIG_DASH_SOL_S: char = 'ᛌ';
    /// \u{16cd}: 'ᛍ'
    pub const LETTER_C: char = 'ᛍ';
    /// \u{16ce}: 'ᛎ'
    pub const LETTER_Z: char = 'ᛎ';
    /// \u{16cf}: 'ᛏ'
    pub const LETTER_TIWAZ_TIR_TYR_T: char = 'ᛏ';
    /// \u{16d0}: 'ᛐ'
    pub const LETTER_SHORT_DASH_TWIG_DASH_TYR_T: char = 'ᛐ';
    /// \u{16d1}: 'ᛑ'
    pub const LETTER_D: char = 'ᛑ';
    /// \u{16d2}: 'ᛒ'
    pub const LETTER_BERKANAN_BEORC_BJARKAN_B: char = 'ᛒ';
    /// \u{16d3}: 'ᛓ'
    pub const LETTER_SHORT_DASH_TWIG_DASH_BJARKAN_B: char = 'ᛓ';
    /// \u{16d4}: 'ᛔ'
    pub const LETTER_DOTTED_DASH_P: char = 'ᛔ';
    /// \u{16d5}: 'ᛕ'
    pub const LETTER_OPEN_DASH_P: char = 'ᛕ';
    /// \u{16d6}: 'ᛖ'
    pub const LETTER_EHWAZ_EH_E: char = 'ᛖ';
    /// \u{16d7}: 'ᛗ'
    pub const LETTER_MANNAZ_MAN_M: char = 'ᛗ';
    /// \u{16d8}: 'ᛘ'
    pub const LETTER_LONG_DASH_BRANCH_DASH_MADR_M: char = 'ᛘ';
    /// \u{16d9}: 'ᛙ'
    pub const LETTER_SHORT_DASH_TWIG_DASH_MADR_M: char = 'ᛙ';
    /// \u{16da}: 'ᛚ'
    pub const LETTER_LAUKAZ_LAGU_LOGR_L: char = 'ᛚ';
    /// \u{16db}: 'ᛛ'
    pub const LETTER_DOTTED_DASH_L: char = 'ᛛ';
    /// \u{16dc}: 'ᛜ'
    pub const LETTER_INGWAZ: char = 'ᛜ';
    /// \u{16dd}: 'ᛝ'
    pub const LETTER_ING: char = 'ᛝ';
    /// \u{16de}: 'ᛞ'
    pub const LETTER_DAGAZ_DAEG_D: char = 'ᛞ';
    /// \u{16df}: 'ᛟ'
    pub const LETTER_OTHALAN_ETHEL_O: char = 'ᛟ';
    /// \u{16e0}: 'ᛠ'
    pub const LETTER_EAR: char = 'ᛠ';
    /// \u{16e1}: 'ᛡ'
    pub const LETTER_IOR: char = 'ᛡ';
    /// \u{16e2}: 'ᛢ'
    pub const LETTER_CWEORTH: char = 'ᛢ';
    /// \u{16e3}: 'ᛣ'
    pub const LETTER_CALC: char = 'ᛣ';
    /// \u{16e4}: 'ᛤ'
    pub const LETTER_CEALC: char = 'ᛤ';
    /// \u{16e5}: 'ᛥ'
    pub const LETTER_STAN: char = 'ᛥ';
    /// \u{16e6}: 'ᛦ'
    pub const LETTER_LONG_DASH_BRANCH_DASH_YR: char = 'ᛦ';
    /// \u{16e7}: 'ᛧ'
    pub const LETTER_SHORT_DASH_TWIG_DASH_YR: char = 'ᛧ';
    /// \u{16e8}: 'ᛨ'
    pub const LETTER_ICELANDIC_DASH_YR: char = 'ᛨ';
    /// \u{16e9}: 'ᛩ'
    pub const LETTER_Q: char = 'ᛩ';
    /// \u{16ea}: 'ᛪ'
    pub const LETTER_X: char = 'ᛪ';
    /// \u{16eb}: '᛫'
    pub const SINGLE_PUNCTUATION: char = '᛫';
    /// \u{16ec}: '᛬'
    pub const MULTIPLE_PUNCTUATION: char = '᛬';
    /// \u{16ed}: '᛭'
    pub const CROSS_PUNCTUATION: char = '᛭';
    /// \u{16ee}: 'ᛮ'
    pub const ARLAUG_SYMBOL: char = 'ᛮ';
    /// \u{16ef}: 'ᛯ'
    pub const TVIMADUR_SYMBOL: char = 'ᛯ';
    /// \u{16f0}: 'ᛰ'
    pub const BELGTHOR_SYMBOL: char = 'ᛰ';
    /// \u{16f1}: 'ᛱ'
    pub const LETTER_K: char = 'ᛱ';
    /// \u{16f2}: 'ᛲ'
    pub const LETTER_SH: char = 'ᛲ';
    /// \u{16f3}: 'ᛳ'
    pub const LETTER_OO: char = 'ᛳ';
    /// \u{16f4}: 'ᛴ'
    pub const LETTER_FRANKS_CASKET_OS: char = 'ᛴ';
    /// \u{16f5}: 'ᛵ'
    pub const LETTER_FRANKS_CASKET_IS: char = 'ᛵ';
    /// \u{16f6}: 'ᛶ'
    pub const LETTER_FRANKS_CASKET_EH: char = 'ᛶ';
    /// \u{16f7}: 'ᛷ'
    pub const LETTER_FRANKS_CASKET_AC: char = 'ᛷ';
    /// \u{16f8}: 'ᛸ'
    pub const LETTER_FRANKS_CASKET_AESC: char = 'ᛸ';
}

/// An enum to represent all characters in the Runic block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Runic {
    /// \u{16a0}: 'ᚠ'
    LetterFehuFeohFeF,
    /// \u{16a1}: 'ᚡ'
    LetterV,
    /// \u{16a2}: 'ᚢ'
    LetterUruzUrU,
    /// \u{16a3}: 'ᚣ'
    LetterYr,
    /// \u{16a4}: 'ᚤ'
    LetterY,
    /// \u{16a5}: 'ᚥ'
    LetterW,
    /// \u{16a6}: 'ᚦ'
    LetterThurisazThursThorn,
    /// \u{16a7}: 'ᚧ'
    LetterEth,
    /// \u{16a8}: 'ᚨ'
    LetterAnsuzA,
    /// \u{16a9}: 'ᚩ'
    LetterOsO,
    /// \u{16aa}: 'ᚪ'
    LetterAcA,
    /// \u{16ab}: 'ᚫ'
    LetterAesc,
    /// \u{16ac}: 'ᚬ'
    LetterLongDashBranchDashOssO,
    /// \u{16ad}: 'ᚭ'
    LetterShortDashTwigDashOssO,
    /// \u{16ae}: 'ᚮ'
    LetterO,
    /// \u{16af}: 'ᚯ'
    LetterOe,
    /// \u{16b0}: 'ᚰ'
    LetterOn,
    /// \u{16b1}: 'ᚱ'
    LetterRaidoRadReidR,
    /// \u{16b2}: 'ᚲ'
    LetterKauna,
    /// \u{16b3}: 'ᚳ'
    LetterCen,
    /// \u{16b4}: 'ᚴ'
    LetterKaunK,
    /// \u{16b5}: 'ᚵ'
    LetterG,
    /// \u{16b6}: 'ᚶ'
    LetterEng,
    /// \u{16b7}: 'ᚷ'
    LetterGeboGyfuG,
    /// \u{16b8}: 'ᚸ'
    LetterGar,
    /// \u{16b9}: 'ᚹ'
    LetterWunjoWynnW,
    /// \u{16ba}: 'ᚺ'
    LetterHaglazH,
    /// \u{16bb}: 'ᚻ'
    LetterHaeglH,
    /// \u{16bc}: 'ᚼ'
    LetterLongDashBranchDashHagallH,
    /// \u{16bd}: 'ᚽ'
    LetterShortDashTwigDashHagallH,
    /// \u{16be}: 'ᚾ'
    LetterNaudizNydNaudN,
    /// \u{16bf}: 'ᚿ'
    LetterShortDashTwigDashNaudN,
    /// \u{16c0}: 'ᛀ'
    LetterDottedDashN,
    /// \u{16c1}: 'ᛁ'
    LetterIsazIsIssI,
    /// \u{16c2}: 'ᛂ'
    LetterE,
    /// \u{16c3}: 'ᛃ'
    LetterJeranJ,
    /// \u{16c4}: 'ᛄ'
    LetterGer,
    /// \u{16c5}: 'ᛅ'
    LetterLongDashBranchDashArAe,
    /// \u{16c6}: 'ᛆ'
    LetterShortDashTwigDashArA,
    /// \u{16c7}: 'ᛇ'
    LetterIwazEoh,
    /// \u{16c8}: 'ᛈ'
    LetterPerthoPeorthP,
    /// \u{16c9}: 'ᛉ'
    LetterAlgizEolhx,
    /// \u{16ca}: 'ᛊ'
    LetterSowiloS,
    /// \u{16cb}: 'ᛋ'
    LetterSigelLongDashBranchDashSolS,
    /// \u{16cc}: 'ᛌ'
    LetterShortDashTwigDashSolS,
    /// \u{16cd}: 'ᛍ'
    LetterC,
    /// \u{16ce}: 'ᛎ'
    LetterZ,
    /// \u{16cf}: 'ᛏ'
    LetterTiwazTirTyrT,
    /// \u{16d0}: 'ᛐ'
    LetterShortDashTwigDashTyrT,
    /// \u{16d1}: 'ᛑ'
    LetterD,
    /// \u{16d2}: 'ᛒ'
    LetterBerkananBeorcBjarkanB,
    /// \u{16d3}: 'ᛓ'
    LetterShortDashTwigDashBjarkanB,
    /// \u{16d4}: 'ᛔ'
    LetterDottedDashP,
    /// \u{16d5}: 'ᛕ'
    LetterOpenDashP,
    /// \u{16d6}: 'ᛖ'
    LetterEhwazEhE,
    /// \u{16d7}: 'ᛗ'
    LetterMannazManM,
    /// \u{16d8}: 'ᛘ'
    LetterLongDashBranchDashMadrM,
    /// \u{16d9}: 'ᛙ'
    LetterShortDashTwigDashMadrM,
    /// \u{16da}: 'ᛚ'
    LetterLaukazLaguLogrL,
    /// \u{16db}: 'ᛛ'
    LetterDottedDashL,
    /// \u{16dc}: 'ᛜ'
    LetterIngwaz,
    /// \u{16dd}: 'ᛝ'
    LetterIng,
    /// \u{16de}: 'ᛞ'
    LetterDagazDaegD,
    /// \u{16df}: 'ᛟ'
    LetterOthalanEthelO,
    /// \u{16e0}: 'ᛠ'
    LetterEar,
    /// \u{16e1}: 'ᛡ'
    LetterIor,
    /// \u{16e2}: 'ᛢ'
    LetterCweorth,
    /// \u{16e3}: 'ᛣ'
    LetterCalc,
    /// \u{16e4}: 'ᛤ'
    LetterCealc,
    /// \u{16e5}: 'ᛥ'
    LetterStan,
    /// \u{16e6}: 'ᛦ'
    LetterLongDashBranchDashYr,
    /// \u{16e7}: 'ᛧ'
    LetterShortDashTwigDashYr,
    /// \u{16e8}: 'ᛨ'
    LetterIcelandicDashYr,
    /// \u{16e9}: 'ᛩ'
    LetterQ,
    /// \u{16ea}: 'ᛪ'
    LetterX,
    /// \u{16eb}: '᛫'
    SinglePunctuation,
    /// \u{16ec}: '᛬'
    MultiplePunctuation,
    /// \u{16ed}: '᛭'
    CrossPunctuation,
    /// \u{16ee}: 'ᛮ'
    ArlaugSymbol,
    /// \u{16ef}: 'ᛯ'
    TvimadurSymbol,
    /// \u{16f0}: 'ᛰ'
    BelgthorSymbol,
    /// \u{16f1}: 'ᛱ'
    LetterK,
    /// \u{16f2}: 'ᛲ'
    LetterSh,
    /// \u{16f3}: 'ᛳ'
    LetterOo,
    /// \u{16f4}: 'ᛴ'
    LetterFranksCasketOs,
    /// \u{16f5}: 'ᛵ'
    LetterFranksCasketIs,
    /// \u{16f6}: 'ᛶ'
    LetterFranksCasketEh,
    /// \u{16f7}: 'ᛷ'
    LetterFranksCasketAc,
    /// \u{16f8}: 'ᛸ'
    LetterFranksCasketAesc,
}

impl Into<char> for Runic {
    fn into(self) -> char {
        use constants::*;
        match self {
            Runic::LetterFehuFeohFeF => LETTER_FEHU_FEOH_FE_F,
            Runic::LetterV => LETTER_V,
            Runic::LetterUruzUrU => LETTER_URUZ_UR_U,
            Runic::LetterYr => LETTER_YR,
            Runic::LetterY => LETTER_Y,
            Runic::LetterW => LETTER_W,
            Runic::LetterThurisazThursThorn => LETTER_THURISAZ_THURS_THORN,
            Runic::LetterEth => LETTER_ETH,
            Runic::LetterAnsuzA => LETTER_ANSUZ_A,
            Runic::LetterOsO => LETTER_OS_O,
            Runic::LetterAcA => LETTER_AC_A,
            Runic::LetterAesc => LETTER_AESC,
            Runic::LetterLongDashBranchDashOssO => LETTER_LONG_DASH_BRANCH_DASH_OSS_O,
            Runic::LetterShortDashTwigDashOssO => LETTER_SHORT_DASH_TWIG_DASH_OSS_O,
            Runic::LetterO => LETTER_O,
            Runic::LetterOe => LETTER_OE,
            Runic::LetterOn => LETTER_ON,
            Runic::LetterRaidoRadReidR => LETTER_RAIDO_RAD_REID_R,
            Runic::LetterKauna => LETTER_KAUNA,
            Runic::LetterCen => LETTER_CEN,
            Runic::LetterKaunK => LETTER_KAUN_K,
            Runic::LetterG => LETTER_G,
            Runic::LetterEng => LETTER_ENG,
            Runic::LetterGeboGyfuG => LETTER_GEBO_GYFU_G,
            Runic::LetterGar => LETTER_GAR,
            Runic::LetterWunjoWynnW => LETTER_WUNJO_WYNN_W,
            Runic::LetterHaglazH => LETTER_HAGLAZ_H,
            Runic::LetterHaeglH => LETTER_HAEGL_H,
            Runic::LetterLongDashBranchDashHagallH => LETTER_LONG_DASH_BRANCH_DASH_HAGALL_H,
            Runic::LetterShortDashTwigDashHagallH => LETTER_SHORT_DASH_TWIG_DASH_HAGALL_H,
            Runic::LetterNaudizNydNaudN => LETTER_NAUDIZ_NYD_NAUD_N,
            Runic::LetterShortDashTwigDashNaudN => LETTER_SHORT_DASH_TWIG_DASH_NAUD_N,
            Runic::LetterDottedDashN => LETTER_DOTTED_DASH_N,
            Runic::LetterIsazIsIssI => LETTER_ISAZ_IS_ISS_I,
            Runic::LetterE => LETTER_E,
            Runic::LetterJeranJ => LETTER_JERAN_J,
            Runic::LetterGer => LETTER_GER,
            Runic::LetterLongDashBranchDashArAe => LETTER_LONG_DASH_BRANCH_DASH_AR_AE,
            Runic::LetterShortDashTwigDashArA => LETTER_SHORT_DASH_TWIG_DASH_AR_A,
            Runic::LetterIwazEoh => LETTER_IWAZ_EOH,
            Runic::LetterPerthoPeorthP => LETTER_PERTHO_PEORTH_P,
            Runic::LetterAlgizEolhx => LETTER_ALGIZ_EOLHX,
            Runic::LetterSowiloS => LETTER_SOWILO_S,
            Runic::LetterSigelLongDashBranchDashSolS => LETTER_SIGEL_LONG_DASH_BRANCH_DASH_SOL_S,
            Runic::LetterShortDashTwigDashSolS => LETTER_SHORT_DASH_TWIG_DASH_SOL_S,
            Runic::LetterC => LETTER_C,
            Runic::LetterZ => LETTER_Z,
            Runic::LetterTiwazTirTyrT => LETTER_TIWAZ_TIR_TYR_T,
            Runic::LetterShortDashTwigDashTyrT => LETTER_SHORT_DASH_TWIG_DASH_TYR_T,
            Runic::LetterD => LETTER_D,
            Runic::LetterBerkananBeorcBjarkanB => LETTER_BERKANAN_BEORC_BJARKAN_B,
            Runic::LetterShortDashTwigDashBjarkanB => LETTER_SHORT_DASH_TWIG_DASH_BJARKAN_B,
            Runic::LetterDottedDashP => LETTER_DOTTED_DASH_P,
            Runic::LetterOpenDashP => LETTER_OPEN_DASH_P,
            Runic::LetterEhwazEhE => LETTER_EHWAZ_EH_E,
            Runic::LetterMannazManM => LETTER_MANNAZ_MAN_M,
            Runic::LetterLongDashBranchDashMadrM => LETTER_LONG_DASH_BRANCH_DASH_MADR_M,
            Runic::LetterShortDashTwigDashMadrM => LETTER_SHORT_DASH_TWIG_DASH_MADR_M,
            Runic::LetterLaukazLaguLogrL => LETTER_LAUKAZ_LAGU_LOGR_L,
            Runic::LetterDottedDashL => LETTER_DOTTED_DASH_L,
            Runic::LetterIngwaz => LETTER_INGWAZ,
            Runic::LetterIng => LETTER_ING,
            Runic::LetterDagazDaegD => LETTER_DAGAZ_DAEG_D,
            Runic::LetterOthalanEthelO => LETTER_OTHALAN_ETHEL_O,
            Runic::LetterEar => LETTER_EAR,
            Runic::LetterIor => LETTER_IOR,
            Runic::LetterCweorth => LETTER_CWEORTH,
            Runic::LetterCalc => LETTER_CALC,
            Runic::LetterCealc => LETTER_CEALC,
            Runic::LetterStan => LETTER_STAN,
            Runic::LetterLongDashBranchDashYr => LETTER_LONG_DASH_BRANCH_DASH_YR,
            Runic::LetterShortDashTwigDashYr => LETTER_SHORT_DASH_TWIG_DASH_YR,
            Runic::LetterIcelandicDashYr => LETTER_ICELANDIC_DASH_YR,
            Runic::LetterQ => LETTER_Q,
            Runic::LetterX => LETTER_X,
            Runic::SinglePunctuation => SINGLE_PUNCTUATION,
            Runic::MultiplePunctuation => MULTIPLE_PUNCTUATION,
            Runic::CrossPunctuation => CROSS_PUNCTUATION,
            Runic::ArlaugSymbol => ARLAUG_SYMBOL,
            Runic::TvimadurSymbol => TVIMADUR_SYMBOL,
            Runic::BelgthorSymbol => BELGTHOR_SYMBOL,
            Runic::LetterK => LETTER_K,
            Runic::LetterSh => LETTER_SH,
            Runic::LetterOo => LETTER_OO,
            Runic::LetterFranksCasketOs => LETTER_FRANKS_CASKET_OS,
            Runic::LetterFranksCasketIs => LETTER_FRANKS_CASKET_IS,
            Runic::LetterFranksCasketEh => LETTER_FRANKS_CASKET_EH,
            Runic::LetterFranksCasketAc => LETTER_FRANKS_CASKET_AC,
            Runic::LetterFranksCasketAesc => LETTER_FRANKS_CASKET_AESC,
        }
    }
}

impl std::convert::TryFrom<char> for Runic {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_FEHU_FEOH_FE_F => Ok(Runic::LetterFehuFeohFeF),
            LETTER_V => Ok(Runic::LetterV),
            LETTER_URUZ_UR_U => Ok(Runic::LetterUruzUrU),
            LETTER_YR => Ok(Runic::LetterYr),
            LETTER_Y => Ok(Runic::LetterY),
            LETTER_W => Ok(Runic::LetterW),
            LETTER_THURISAZ_THURS_THORN => Ok(Runic::LetterThurisazThursThorn),
            LETTER_ETH => Ok(Runic::LetterEth),
            LETTER_ANSUZ_A => Ok(Runic::LetterAnsuzA),
            LETTER_OS_O => Ok(Runic::LetterOsO),
            LETTER_AC_A => Ok(Runic::LetterAcA),
            LETTER_AESC => Ok(Runic::LetterAesc),
            LETTER_LONG_DASH_BRANCH_DASH_OSS_O => Ok(Runic::LetterLongDashBranchDashOssO),
            LETTER_SHORT_DASH_TWIG_DASH_OSS_O => Ok(Runic::LetterShortDashTwigDashOssO),
            LETTER_O => Ok(Runic::LetterO),
            LETTER_OE => Ok(Runic::LetterOe),
            LETTER_ON => Ok(Runic::LetterOn),
            LETTER_RAIDO_RAD_REID_R => Ok(Runic::LetterRaidoRadReidR),
            LETTER_KAUNA => Ok(Runic::LetterKauna),
            LETTER_CEN => Ok(Runic::LetterCen),
            LETTER_KAUN_K => Ok(Runic::LetterKaunK),
            LETTER_G => Ok(Runic::LetterG),
            LETTER_ENG => Ok(Runic::LetterEng),
            LETTER_GEBO_GYFU_G => Ok(Runic::LetterGeboGyfuG),
            LETTER_GAR => Ok(Runic::LetterGar),
            LETTER_WUNJO_WYNN_W => Ok(Runic::LetterWunjoWynnW),
            LETTER_HAGLAZ_H => Ok(Runic::LetterHaglazH),
            LETTER_HAEGL_H => Ok(Runic::LetterHaeglH),
            LETTER_LONG_DASH_BRANCH_DASH_HAGALL_H => Ok(Runic::LetterLongDashBranchDashHagallH),
            LETTER_SHORT_DASH_TWIG_DASH_HAGALL_H => Ok(Runic::LetterShortDashTwigDashHagallH),
            LETTER_NAUDIZ_NYD_NAUD_N => Ok(Runic::LetterNaudizNydNaudN),
            LETTER_SHORT_DASH_TWIG_DASH_NAUD_N => Ok(Runic::LetterShortDashTwigDashNaudN),
            LETTER_DOTTED_DASH_N => Ok(Runic::LetterDottedDashN),
            LETTER_ISAZ_IS_ISS_I => Ok(Runic::LetterIsazIsIssI),
            LETTER_E => Ok(Runic::LetterE),
            LETTER_JERAN_J => Ok(Runic::LetterJeranJ),
            LETTER_GER => Ok(Runic::LetterGer),
            LETTER_LONG_DASH_BRANCH_DASH_AR_AE => Ok(Runic::LetterLongDashBranchDashArAe),
            LETTER_SHORT_DASH_TWIG_DASH_AR_A => Ok(Runic::LetterShortDashTwigDashArA),
            LETTER_IWAZ_EOH => Ok(Runic::LetterIwazEoh),
            LETTER_PERTHO_PEORTH_P => Ok(Runic::LetterPerthoPeorthP),
            LETTER_ALGIZ_EOLHX => Ok(Runic::LetterAlgizEolhx),
            LETTER_SOWILO_S => Ok(Runic::LetterSowiloS),
            LETTER_SIGEL_LONG_DASH_BRANCH_DASH_SOL_S => Ok(Runic::LetterSigelLongDashBranchDashSolS),
            LETTER_SHORT_DASH_TWIG_DASH_SOL_S => Ok(Runic::LetterShortDashTwigDashSolS),
            LETTER_C => Ok(Runic::LetterC),
            LETTER_Z => Ok(Runic::LetterZ),
            LETTER_TIWAZ_TIR_TYR_T => Ok(Runic::LetterTiwazTirTyrT),
            LETTER_SHORT_DASH_TWIG_DASH_TYR_T => Ok(Runic::LetterShortDashTwigDashTyrT),
            LETTER_D => Ok(Runic::LetterD),
            LETTER_BERKANAN_BEORC_BJARKAN_B => Ok(Runic::LetterBerkananBeorcBjarkanB),
            LETTER_SHORT_DASH_TWIG_DASH_BJARKAN_B => Ok(Runic::LetterShortDashTwigDashBjarkanB),
            LETTER_DOTTED_DASH_P => Ok(Runic::LetterDottedDashP),
            LETTER_OPEN_DASH_P => Ok(Runic::LetterOpenDashP),
            LETTER_EHWAZ_EH_E => Ok(Runic::LetterEhwazEhE),
            LETTER_MANNAZ_MAN_M => Ok(Runic::LetterMannazManM),
            LETTER_LONG_DASH_BRANCH_DASH_MADR_M => Ok(Runic::LetterLongDashBranchDashMadrM),
            LETTER_SHORT_DASH_TWIG_DASH_MADR_M => Ok(Runic::LetterShortDashTwigDashMadrM),
            LETTER_LAUKAZ_LAGU_LOGR_L => Ok(Runic::LetterLaukazLaguLogrL),
            LETTER_DOTTED_DASH_L => Ok(Runic::LetterDottedDashL),
            LETTER_INGWAZ => Ok(Runic::LetterIngwaz),
            LETTER_ING => Ok(Runic::LetterIng),
            LETTER_DAGAZ_DAEG_D => Ok(Runic::LetterDagazDaegD),
            LETTER_OTHALAN_ETHEL_O => Ok(Runic::LetterOthalanEthelO),
            LETTER_EAR => Ok(Runic::LetterEar),
            LETTER_IOR => Ok(Runic::LetterIor),
            LETTER_CWEORTH => Ok(Runic::LetterCweorth),
            LETTER_CALC => Ok(Runic::LetterCalc),
            LETTER_CEALC => Ok(Runic::LetterCealc),
            LETTER_STAN => Ok(Runic::LetterStan),
            LETTER_LONG_DASH_BRANCH_DASH_YR => Ok(Runic::LetterLongDashBranchDashYr),
            LETTER_SHORT_DASH_TWIG_DASH_YR => Ok(Runic::LetterShortDashTwigDashYr),
            LETTER_ICELANDIC_DASH_YR => Ok(Runic::LetterIcelandicDashYr),
            LETTER_Q => Ok(Runic::LetterQ),
            LETTER_X => Ok(Runic::LetterX),
            SINGLE_PUNCTUATION => Ok(Runic::SinglePunctuation),
            MULTIPLE_PUNCTUATION => Ok(Runic::MultiplePunctuation),
            CROSS_PUNCTUATION => Ok(Runic::CrossPunctuation),
            ARLAUG_SYMBOL => Ok(Runic::ArlaugSymbol),
            TVIMADUR_SYMBOL => Ok(Runic::TvimadurSymbol),
            BELGTHOR_SYMBOL => Ok(Runic::BelgthorSymbol),
            LETTER_K => Ok(Runic::LetterK),
            LETTER_SH => Ok(Runic::LetterSh),
            LETTER_OO => Ok(Runic::LetterOo),
            LETTER_FRANKS_CASKET_OS => Ok(Runic::LetterFranksCasketOs),
            LETTER_FRANKS_CASKET_IS => Ok(Runic::LetterFranksCasketIs),
            LETTER_FRANKS_CASKET_EH => Ok(Runic::LetterFranksCasketEh),
            LETTER_FRANKS_CASKET_AC => Ok(Runic::LetterFranksCasketAc),
            LETTER_FRANKS_CASKET_AESC => Ok(Runic::LetterFranksCasketAesc),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Runic {
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

impl std::convert::TryFrom<u32> for Runic {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Runic {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Runic {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Runic::LetterFehuFeohFeF
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Runic::LetterFehuFeohFeF => "runic letter fehu feoh fe f",
            Runic::LetterV => "runic letter v",
            Runic::LetterUruzUrU => "runic letter uruz ur u",
            Runic::LetterYr => "runic letter yr",
            Runic::LetterY => "runic letter y",
            Runic::LetterW => "runic letter w",
            Runic::LetterThurisazThursThorn => "runic letter thurisaz thurs thorn",
            Runic::LetterEth => "runic letter eth",
            Runic::LetterAnsuzA => "runic letter ansuz a",
            Runic::LetterOsO => "runic letter os o",
            Runic::LetterAcA => "runic letter ac a",
            Runic::LetterAesc => "runic letter aesc",
            Runic::LetterLongDashBranchDashOssO => "runic letter long-branch-oss o",
            Runic::LetterShortDashTwigDashOssO => "runic letter short-twig-oss o",
            Runic::LetterO => "runic letter o",
            Runic::LetterOe => "runic letter oe",
            Runic::LetterOn => "runic letter on",
            Runic::LetterRaidoRadReidR => "runic letter raido rad reid r",
            Runic::LetterKauna => "runic letter kauna",
            Runic::LetterCen => "runic letter cen",
            Runic::LetterKaunK => "runic letter kaun k",
            Runic::LetterG => "runic letter g",
            Runic::LetterEng => "runic letter eng",
            Runic::LetterGeboGyfuG => "runic letter gebo gyfu g",
            Runic::LetterGar => "runic letter gar",
            Runic::LetterWunjoWynnW => "runic letter wunjo wynn w",
            Runic::LetterHaglazH => "runic letter haglaz h",
            Runic::LetterHaeglH => "runic letter haegl h",
            Runic::LetterLongDashBranchDashHagallH => "runic letter long-branch-hagall h",
            Runic::LetterShortDashTwigDashHagallH => "runic letter short-twig-hagall h",
            Runic::LetterNaudizNydNaudN => "runic letter naudiz nyd naud n",
            Runic::LetterShortDashTwigDashNaudN => "runic letter short-twig-naud n",
            Runic::LetterDottedDashN => "runic letter dotted-n",
            Runic::LetterIsazIsIssI => "runic letter isaz is iss i",
            Runic::LetterE => "runic letter e",
            Runic::LetterJeranJ => "runic letter jeran j",
            Runic::LetterGer => "runic letter ger",
            Runic::LetterLongDashBranchDashArAe => "runic letter long-branch-ar ae",
            Runic::LetterShortDashTwigDashArA => "runic letter short-twig-ar a",
            Runic::LetterIwazEoh => "runic letter iwaz eoh",
            Runic::LetterPerthoPeorthP => "runic letter pertho peorth p",
            Runic::LetterAlgizEolhx => "runic letter algiz eolhx",
            Runic::LetterSowiloS => "runic letter sowilo s",
            Runic::LetterSigelLongDashBranchDashSolS => "runic letter sigel long-branch-sol s",
            Runic::LetterShortDashTwigDashSolS => "runic letter short-twig-sol s",
            Runic::LetterC => "runic letter c",
            Runic::LetterZ => "runic letter z",
            Runic::LetterTiwazTirTyrT => "runic letter tiwaz tir tyr t",
            Runic::LetterShortDashTwigDashTyrT => "runic letter short-twig-tyr t",
            Runic::LetterD => "runic letter d",
            Runic::LetterBerkananBeorcBjarkanB => "runic letter berkanan beorc bjarkan b",
            Runic::LetterShortDashTwigDashBjarkanB => "runic letter short-twig-bjarkan b",
            Runic::LetterDottedDashP => "runic letter dotted-p",
            Runic::LetterOpenDashP => "runic letter open-p",
            Runic::LetterEhwazEhE => "runic letter ehwaz eh e",
            Runic::LetterMannazManM => "runic letter mannaz man m",
            Runic::LetterLongDashBranchDashMadrM => "runic letter long-branch-madr m",
            Runic::LetterShortDashTwigDashMadrM => "runic letter short-twig-madr m",
            Runic::LetterLaukazLaguLogrL => "runic letter laukaz lagu logr l",
            Runic::LetterDottedDashL => "runic letter dotted-l",
            Runic::LetterIngwaz => "runic letter ingwaz",
            Runic::LetterIng => "runic letter ing",
            Runic::LetterDagazDaegD => "runic letter dagaz daeg d",
            Runic::LetterOthalanEthelO => "runic letter othalan ethel o",
            Runic::LetterEar => "runic letter ear",
            Runic::LetterIor => "runic letter ior",
            Runic::LetterCweorth => "runic letter cweorth",
            Runic::LetterCalc => "runic letter calc",
            Runic::LetterCealc => "runic letter cealc",
            Runic::LetterStan => "runic letter stan",
            Runic::LetterLongDashBranchDashYr => "runic letter long-branch-yr",
            Runic::LetterShortDashTwigDashYr => "runic letter short-twig-yr",
            Runic::LetterIcelandicDashYr => "runic letter icelandic-yr",
            Runic::LetterQ => "runic letter q",
            Runic::LetterX => "runic letter x",
            Runic::SinglePunctuation => "runic single punctuation",
            Runic::MultiplePunctuation => "runic multiple punctuation",
            Runic::CrossPunctuation => "runic cross punctuation",
            Runic::ArlaugSymbol => "runic arlaug symbol",
            Runic::TvimadurSymbol => "runic tvimadur symbol",
            Runic::BelgthorSymbol => "runic belgthor symbol",
            Runic::LetterK => "runic letter k",
            Runic::LetterSh => "runic letter sh",
            Runic::LetterOo => "runic letter oo",
            Runic::LetterFranksCasketOs => "runic letter franks casket os",
            Runic::LetterFranksCasketIs => "runic letter franks casket is",
            Runic::LetterFranksCasketEh => "runic letter franks casket eh",
            Runic::LetterFranksCasketAc => "runic letter franks casket ac",
            Runic::LetterFranksCasketAesc => "runic letter franks casket aesc",
        }
    }
}
