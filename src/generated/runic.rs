
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
        match self {
            Runic::LetterFehuFeohFeF => 'ᚠ',
            Runic::LetterV => 'ᚡ',
            Runic::LetterUruzUrU => 'ᚢ',
            Runic::LetterYr => 'ᚣ',
            Runic::LetterY => 'ᚤ',
            Runic::LetterW => 'ᚥ',
            Runic::LetterThurisazThursThorn => 'ᚦ',
            Runic::LetterEth => 'ᚧ',
            Runic::LetterAnsuzA => 'ᚨ',
            Runic::LetterOsO => 'ᚩ',
            Runic::LetterAcA => 'ᚪ',
            Runic::LetterAesc => 'ᚫ',
            Runic::LetterLongDashBranchDashOssO => 'ᚬ',
            Runic::LetterShortDashTwigDashOssO => 'ᚭ',
            Runic::LetterO => 'ᚮ',
            Runic::LetterOe => 'ᚯ',
            Runic::LetterOn => 'ᚰ',
            Runic::LetterRaidoRadReidR => 'ᚱ',
            Runic::LetterKauna => 'ᚲ',
            Runic::LetterCen => 'ᚳ',
            Runic::LetterKaunK => 'ᚴ',
            Runic::LetterG => 'ᚵ',
            Runic::LetterEng => 'ᚶ',
            Runic::LetterGeboGyfuG => 'ᚷ',
            Runic::LetterGar => 'ᚸ',
            Runic::LetterWunjoWynnW => 'ᚹ',
            Runic::LetterHaglazH => 'ᚺ',
            Runic::LetterHaeglH => 'ᚻ',
            Runic::LetterLongDashBranchDashHagallH => 'ᚼ',
            Runic::LetterShortDashTwigDashHagallH => 'ᚽ',
            Runic::LetterNaudizNydNaudN => 'ᚾ',
            Runic::LetterShortDashTwigDashNaudN => 'ᚿ',
            Runic::LetterDottedDashN => 'ᛀ',
            Runic::LetterIsazIsIssI => 'ᛁ',
            Runic::LetterE => 'ᛂ',
            Runic::LetterJeranJ => 'ᛃ',
            Runic::LetterGer => 'ᛄ',
            Runic::LetterLongDashBranchDashArAe => 'ᛅ',
            Runic::LetterShortDashTwigDashArA => 'ᛆ',
            Runic::LetterIwazEoh => 'ᛇ',
            Runic::LetterPerthoPeorthP => 'ᛈ',
            Runic::LetterAlgizEolhx => 'ᛉ',
            Runic::LetterSowiloS => 'ᛊ',
            Runic::LetterSigelLongDashBranchDashSolS => 'ᛋ',
            Runic::LetterShortDashTwigDashSolS => 'ᛌ',
            Runic::LetterC => 'ᛍ',
            Runic::LetterZ => 'ᛎ',
            Runic::LetterTiwazTirTyrT => 'ᛏ',
            Runic::LetterShortDashTwigDashTyrT => 'ᛐ',
            Runic::LetterD => 'ᛑ',
            Runic::LetterBerkananBeorcBjarkanB => 'ᛒ',
            Runic::LetterShortDashTwigDashBjarkanB => 'ᛓ',
            Runic::LetterDottedDashP => 'ᛔ',
            Runic::LetterOpenDashP => 'ᛕ',
            Runic::LetterEhwazEhE => 'ᛖ',
            Runic::LetterMannazManM => 'ᛗ',
            Runic::LetterLongDashBranchDashMadrM => 'ᛘ',
            Runic::LetterShortDashTwigDashMadrM => 'ᛙ',
            Runic::LetterLaukazLaguLogrL => 'ᛚ',
            Runic::LetterDottedDashL => 'ᛛ',
            Runic::LetterIngwaz => 'ᛜ',
            Runic::LetterIng => 'ᛝ',
            Runic::LetterDagazDaegD => 'ᛞ',
            Runic::LetterOthalanEthelO => 'ᛟ',
            Runic::LetterEar => 'ᛠ',
            Runic::LetterIor => 'ᛡ',
            Runic::LetterCweorth => 'ᛢ',
            Runic::LetterCalc => 'ᛣ',
            Runic::LetterCealc => 'ᛤ',
            Runic::LetterStan => 'ᛥ',
            Runic::LetterLongDashBranchDashYr => 'ᛦ',
            Runic::LetterShortDashTwigDashYr => 'ᛧ',
            Runic::LetterIcelandicDashYr => 'ᛨ',
            Runic::LetterQ => 'ᛩ',
            Runic::LetterX => 'ᛪ',
            Runic::SinglePunctuation => '᛫',
            Runic::MultiplePunctuation => '᛬',
            Runic::CrossPunctuation => '᛭',
            Runic::ArlaugSymbol => 'ᛮ',
            Runic::TvimadurSymbol => 'ᛯ',
            Runic::BelgthorSymbol => 'ᛰ',
            Runic::LetterK => 'ᛱ',
            Runic::LetterSh => 'ᛲ',
            Runic::LetterOo => 'ᛳ',
            Runic::LetterFranksCasketOs => 'ᛴ',
            Runic::LetterFranksCasketIs => 'ᛵ',
            Runic::LetterFranksCasketEh => 'ᛶ',
            Runic::LetterFranksCasketAc => 'ᛷ',
            Runic::LetterFranksCasketAesc => 'ᛸ',
        }
    }
}

impl std::convert::TryFrom<char> for Runic {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ᚠ' => Ok(Runic::LetterFehuFeohFeF),
            'ᚡ' => Ok(Runic::LetterV),
            'ᚢ' => Ok(Runic::LetterUruzUrU),
            'ᚣ' => Ok(Runic::LetterYr),
            'ᚤ' => Ok(Runic::LetterY),
            'ᚥ' => Ok(Runic::LetterW),
            'ᚦ' => Ok(Runic::LetterThurisazThursThorn),
            'ᚧ' => Ok(Runic::LetterEth),
            'ᚨ' => Ok(Runic::LetterAnsuzA),
            'ᚩ' => Ok(Runic::LetterOsO),
            'ᚪ' => Ok(Runic::LetterAcA),
            'ᚫ' => Ok(Runic::LetterAesc),
            'ᚬ' => Ok(Runic::LetterLongDashBranchDashOssO),
            'ᚭ' => Ok(Runic::LetterShortDashTwigDashOssO),
            'ᚮ' => Ok(Runic::LetterO),
            'ᚯ' => Ok(Runic::LetterOe),
            'ᚰ' => Ok(Runic::LetterOn),
            'ᚱ' => Ok(Runic::LetterRaidoRadReidR),
            'ᚲ' => Ok(Runic::LetterKauna),
            'ᚳ' => Ok(Runic::LetterCen),
            'ᚴ' => Ok(Runic::LetterKaunK),
            'ᚵ' => Ok(Runic::LetterG),
            'ᚶ' => Ok(Runic::LetterEng),
            'ᚷ' => Ok(Runic::LetterGeboGyfuG),
            'ᚸ' => Ok(Runic::LetterGar),
            'ᚹ' => Ok(Runic::LetterWunjoWynnW),
            'ᚺ' => Ok(Runic::LetterHaglazH),
            'ᚻ' => Ok(Runic::LetterHaeglH),
            'ᚼ' => Ok(Runic::LetterLongDashBranchDashHagallH),
            'ᚽ' => Ok(Runic::LetterShortDashTwigDashHagallH),
            'ᚾ' => Ok(Runic::LetterNaudizNydNaudN),
            'ᚿ' => Ok(Runic::LetterShortDashTwigDashNaudN),
            'ᛀ' => Ok(Runic::LetterDottedDashN),
            'ᛁ' => Ok(Runic::LetterIsazIsIssI),
            'ᛂ' => Ok(Runic::LetterE),
            'ᛃ' => Ok(Runic::LetterJeranJ),
            'ᛄ' => Ok(Runic::LetterGer),
            'ᛅ' => Ok(Runic::LetterLongDashBranchDashArAe),
            'ᛆ' => Ok(Runic::LetterShortDashTwigDashArA),
            'ᛇ' => Ok(Runic::LetterIwazEoh),
            'ᛈ' => Ok(Runic::LetterPerthoPeorthP),
            'ᛉ' => Ok(Runic::LetterAlgizEolhx),
            'ᛊ' => Ok(Runic::LetterSowiloS),
            'ᛋ' => Ok(Runic::LetterSigelLongDashBranchDashSolS),
            'ᛌ' => Ok(Runic::LetterShortDashTwigDashSolS),
            'ᛍ' => Ok(Runic::LetterC),
            'ᛎ' => Ok(Runic::LetterZ),
            'ᛏ' => Ok(Runic::LetterTiwazTirTyrT),
            'ᛐ' => Ok(Runic::LetterShortDashTwigDashTyrT),
            'ᛑ' => Ok(Runic::LetterD),
            'ᛒ' => Ok(Runic::LetterBerkananBeorcBjarkanB),
            'ᛓ' => Ok(Runic::LetterShortDashTwigDashBjarkanB),
            'ᛔ' => Ok(Runic::LetterDottedDashP),
            'ᛕ' => Ok(Runic::LetterOpenDashP),
            'ᛖ' => Ok(Runic::LetterEhwazEhE),
            'ᛗ' => Ok(Runic::LetterMannazManM),
            'ᛘ' => Ok(Runic::LetterLongDashBranchDashMadrM),
            'ᛙ' => Ok(Runic::LetterShortDashTwigDashMadrM),
            'ᛚ' => Ok(Runic::LetterLaukazLaguLogrL),
            'ᛛ' => Ok(Runic::LetterDottedDashL),
            'ᛜ' => Ok(Runic::LetterIngwaz),
            'ᛝ' => Ok(Runic::LetterIng),
            'ᛞ' => Ok(Runic::LetterDagazDaegD),
            'ᛟ' => Ok(Runic::LetterOthalanEthelO),
            'ᛠ' => Ok(Runic::LetterEar),
            'ᛡ' => Ok(Runic::LetterIor),
            'ᛢ' => Ok(Runic::LetterCweorth),
            'ᛣ' => Ok(Runic::LetterCalc),
            'ᛤ' => Ok(Runic::LetterCealc),
            'ᛥ' => Ok(Runic::LetterStan),
            'ᛦ' => Ok(Runic::LetterLongDashBranchDashYr),
            'ᛧ' => Ok(Runic::LetterShortDashTwigDashYr),
            'ᛨ' => Ok(Runic::LetterIcelandicDashYr),
            'ᛩ' => Ok(Runic::LetterQ),
            'ᛪ' => Ok(Runic::LetterX),
            '᛫' => Ok(Runic::SinglePunctuation),
            '᛬' => Ok(Runic::MultiplePunctuation),
            '᛭' => Ok(Runic::CrossPunctuation),
            'ᛮ' => Ok(Runic::ArlaugSymbol),
            'ᛯ' => Ok(Runic::TvimadurSymbol),
            'ᛰ' => Ok(Runic::BelgthorSymbol),
            'ᛱ' => Ok(Runic::LetterK),
            'ᛲ' => Ok(Runic::LetterSh),
            'ᛳ' => Ok(Runic::LetterOo),
            'ᛴ' => Ok(Runic::LetterFranksCasketOs),
            'ᛵ' => Ok(Runic::LetterFranksCasketIs),
            'ᛶ' => Ok(Runic::LetterFranksCasketEh),
            'ᛷ' => Ok(Runic::LetterFranksCasketAc),
            'ᛸ' => Ok(Runic::LetterFranksCasketAesc),
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
