
/// An enum to represent all characters in the GlagoliticSupplement block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum GlagoliticSupplement {
    /// \u{1e000}: '𞀀'
    CombiningGlagoliticLetterAzu,
    /// \u{1e001}: '𞀁'
    CombiningGlagoliticLetterBuky,
    /// \u{1e002}: '𞀂'
    CombiningGlagoliticLetterVede,
    /// \u{1e003}: '𞀃'
    CombiningGlagoliticLetterGlagoli,
    /// \u{1e004}: '𞀄'
    CombiningGlagoliticLetterDobro,
    /// \u{1e005}: '𞀅'
    CombiningGlagoliticLetterYestu,
    /// \u{1e006}: '𞀆'
    CombiningGlagoliticLetterZhivete,
    /// \u{1e008}: '𞀈'
    CombiningGlagoliticLetterZemlja,
    /// \u{1e009}: '𞀉'
    CombiningGlagoliticLetterIzhe,
    /// \u{1e00a}: '𞀊'
    CombiningGlagoliticLetterInitialIzhe,
    /// \u{1e00b}: '𞀋'
    CombiningGlagoliticLetterI,
    /// \u{1e00c}: '𞀌'
    CombiningGlagoliticLetterDjervi,
    /// \u{1e00d}: '𞀍'
    CombiningGlagoliticLetterKako,
    /// \u{1e00e}: '𞀎'
    CombiningGlagoliticLetterLjudije,
    /// \u{1e00f}: '𞀏'
    CombiningGlagoliticLetterMyslite,
    /// \u{1e010}: '𞀐'
    CombiningGlagoliticLetterNashi,
    /// \u{1e011}: '𞀑'
    CombiningGlagoliticLetterOnu,
    /// \u{1e012}: '𞀒'
    CombiningGlagoliticLetterPokoji,
    /// \u{1e013}: '𞀓'
    CombiningGlagoliticLetterRitsi,
    /// \u{1e014}: '𞀔'
    CombiningGlagoliticLetterSlovo,
    /// \u{1e015}: '𞀕'
    CombiningGlagoliticLetterTvrido,
    /// \u{1e016}: '𞀖'
    CombiningGlagoliticLetterUku,
    /// \u{1e017}: '𞀗'
    CombiningGlagoliticLetterFritu,
    /// \u{1e018}: '𞀘'
    CombiningGlagoliticLetterHeru,
    /// \u{1e01b}: '𞀛'
    CombiningGlagoliticLetterShta,
    /// \u{1e01c}: '𞀜'
    CombiningGlagoliticLetterTsi,
    /// \u{1e01d}: '𞀝'
    CombiningGlagoliticLetterChrivi,
    /// \u{1e01e}: '𞀞'
    CombiningGlagoliticLetterSha,
    /// \u{1e01f}: '𞀟'
    CombiningGlagoliticLetterYeru,
    /// \u{1e020}: '𞀠'
    CombiningGlagoliticLetterYeri,
    /// \u{1e021}: '𞀡'
    CombiningGlagoliticLetterYati,
    /// \u{1e023}: '𞀣'
    CombiningGlagoliticLetterYu,
    /// \u{1e024}: '𞀤'
    CombiningGlagoliticLetterSmallYus,
    /// \u{1e026}: '𞀦'
    CombiningGlagoliticLetterYo,
    /// \u{1e027}: '𞀧'
    CombiningGlagoliticLetterIotatedSmallYus,
    /// \u{1e028}: '𞀨'
    CombiningGlagoliticLetterBigYus,
    /// \u{1e029}: '𞀩'
    CombiningGlagoliticLetterIotatedBigYus,
    /// \u{1e02a}: '𞀪'
    CombiningGlagoliticLetterFita,
}

impl Into<char> for GlagoliticSupplement {
    fn into(self) -> char {
        match self {
            GlagoliticSupplement::CombiningGlagoliticLetterAzu => '𞀀',
            GlagoliticSupplement::CombiningGlagoliticLetterBuky => '𞀁',
            GlagoliticSupplement::CombiningGlagoliticLetterVede => '𞀂',
            GlagoliticSupplement::CombiningGlagoliticLetterGlagoli => '𞀃',
            GlagoliticSupplement::CombiningGlagoliticLetterDobro => '𞀄',
            GlagoliticSupplement::CombiningGlagoliticLetterYestu => '𞀅',
            GlagoliticSupplement::CombiningGlagoliticLetterZhivete => '𞀆',
            GlagoliticSupplement::CombiningGlagoliticLetterZemlja => '𞀈',
            GlagoliticSupplement::CombiningGlagoliticLetterIzhe => '𞀉',
            GlagoliticSupplement::CombiningGlagoliticLetterInitialIzhe => '𞀊',
            GlagoliticSupplement::CombiningGlagoliticLetterI => '𞀋',
            GlagoliticSupplement::CombiningGlagoliticLetterDjervi => '𞀌',
            GlagoliticSupplement::CombiningGlagoliticLetterKako => '𞀍',
            GlagoliticSupplement::CombiningGlagoliticLetterLjudije => '𞀎',
            GlagoliticSupplement::CombiningGlagoliticLetterMyslite => '𞀏',
            GlagoliticSupplement::CombiningGlagoliticLetterNashi => '𞀐',
            GlagoliticSupplement::CombiningGlagoliticLetterOnu => '𞀑',
            GlagoliticSupplement::CombiningGlagoliticLetterPokoji => '𞀒',
            GlagoliticSupplement::CombiningGlagoliticLetterRitsi => '𞀓',
            GlagoliticSupplement::CombiningGlagoliticLetterSlovo => '𞀔',
            GlagoliticSupplement::CombiningGlagoliticLetterTvrido => '𞀕',
            GlagoliticSupplement::CombiningGlagoliticLetterUku => '𞀖',
            GlagoliticSupplement::CombiningGlagoliticLetterFritu => '𞀗',
            GlagoliticSupplement::CombiningGlagoliticLetterHeru => '𞀘',
            GlagoliticSupplement::CombiningGlagoliticLetterShta => '𞀛',
            GlagoliticSupplement::CombiningGlagoliticLetterTsi => '𞀜',
            GlagoliticSupplement::CombiningGlagoliticLetterChrivi => '𞀝',
            GlagoliticSupplement::CombiningGlagoliticLetterSha => '𞀞',
            GlagoliticSupplement::CombiningGlagoliticLetterYeru => '𞀟',
            GlagoliticSupplement::CombiningGlagoliticLetterYeri => '𞀠',
            GlagoliticSupplement::CombiningGlagoliticLetterYati => '𞀡',
            GlagoliticSupplement::CombiningGlagoliticLetterYu => '𞀣',
            GlagoliticSupplement::CombiningGlagoliticLetterSmallYus => '𞀤',
            GlagoliticSupplement::CombiningGlagoliticLetterYo => '𞀦',
            GlagoliticSupplement::CombiningGlagoliticLetterIotatedSmallYus => '𞀧',
            GlagoliticSupplement::CombiningGlagoliticLetterBigYus => '𞀨',
            GlagoliticSupplement::CombiningGlagoliticLetterIotatedBigYus => '𞀩',
            GlagoliticSupplement::CombiningGlagoliticLetterFita => '𞀪',
        }
    }
}

impl std::convert::TryFrom<char> for GlagoliticSupplement {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𞀀' => Ok(GlagoliticSupplement::CombiningGlagoliticLetterAzu),
            '𞀁' => Ok(GlagoliticSupplement::CombiningGlagoliticLetterBuky),
            '𞀂' => Ok(GlagoliticSupplement::CombiningGlagoliticLetterVede),
            '𞀃' => Ok(GlagoliticSupplement::CombiningGlagoliticLetterGlagoli),
            '𞀄' => Ok(GlagoliticSupplement::CombiningGlagoliticLetterDobro),
            '𞀅' => Ok(GlagoliticSupplement::CombiningGlagoliticLetterYestu),
            '𞀆' => Ok(GlagoliticSupplement::CombiningGlagoliticLetterZhivete),
            '𞀈' => Ok(GlagoliticSupplement::CombiningGlagoliticLetterZemlja),
            '𞀉' => Ok(GlagoliticSupplement::CombiningGlagoliticLetterIzhe),
            '𞀊' => Ok(GlagoliticSupplement::CombiningGlagoliticLetterInitialIzhe),
            '𞀋' => Ok(GlagoliticSupplement::CombiningGlagoliticLetterI),
            '𞀌' => Ok(GlagoliticSupplement::CombiningGlagoliticLetterDjervi),
            '𞀍' => Ok(GlagoliticSupplement::CombiningGlagoliticLetterKako),
            '𞀎' => Ok(GlagoliticSupplement::CombiningGlagoliticLetterLjudije),
            '𞀏' => Ok(GlagoliticSupplement::CombiningGlagoliticLetterMyslite),
            '𞀐' => Ok(GlagoliticSupplement::CombiningGlagoliticLetterNashi),
            '𞀑' => Ok(GlagoliticSupplement::CombiningGlagoliticLetterOnu),
            '𞀒' => Ok(GlagoliticSupplement::CombiningGlagoliticLetterPokoji),
            '𞀓' => Ok(GlagoliticSupplement::CombiningGlagoliticLetterRitsi),
            '𞀔' => Ok(GlagoliticSupplement::CombiningGlagoliticLetterSlovo),
            '𞀕' => Ok(GlagoliticSupplement::CombiningGlagoliticLetterTvrido),
            '𞀖' => Ok(GlagoliticSupplement::CombiningGlagoliticLetterUku),
            '𞀗' => Ok(GlagoliticSupplement::CombiningGlagoliticLetterFritu),
            '𞀘' => Ok(GlagoliticSupplement::CombiningGlagoliticLetterHeru),
            '𞀛' => Ok(GlagoliticSupplement::CombiningGlagoliticLetterShta),
            '𞀜' => Ok(GlagoliticSupplement::CombiningGlagoliticLetterTsi),
            '𞀝' => Ok(GlagoliticSupplement::CombiningGlagoliticLetterChrivi),
            '𞀞' => Ok(GlagoliticSupplement::CombiningGlagoliticLetterSha),
            '𞀟' => Ok(GlagoliticSupplement::CombiningGlagoliticLetterYeru),
            '𞀠' => Ok(GlagoliticSupplement::CombiningGlagoliticLetterYeri),
            '𞀡' => Ok(GlagoliticSupplement::CombiningGlagoliticLetterYati),
            '𞀣' => Ok(GlagoliticSupplement::CombiningGlagoliticLetterYu),
            '𞀤' => Ok(GlagoliticSupplement::CombiningGlagoliticLetterSmallYus),
            '𞀦' => Ok(GlagoliticSupplement::CombiningGlagoliticLetterYo),
            '𞀧' => Ok(GlagoliticSupplement::CombiningGlagoliticLetterIotatedSmallYus),
            '𞀨' => Ok(GlagoliticSupplement::CombiningGlagoliticLetterBigYus),
            '𞀩' => Ok(GlagoliticSupplement::CombiningGlagoliticLetterIotatedBigYus),
            '𞀪' => Ok(GlagoliticSupplement::CombiningGlagoliticLetterFita),
            _ => Err(()),
        }
    }
}

impl Into<u32> for GlagoliticSupplement {
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

impl std::convert::TryFrom<u32> for GlagoliticSupplement {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for GlagoliticSupplement {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl GlagoliticSupplement {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        GlagoliticSupplement::CombiningGlagoliticLetterAzu
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            GlagoliticSupplement::CombiningGlagoliticLetterAzu => "combining glagolitic letter azu",
            GlagoliticSupplement::CombiningGlagoliticLetterBuky => "combining glagolitic letter buky",
            GlagoliticSupplement::CombiningGlagoliticLetterVede => "combining glagolitic letter vede",
            GlagoliticSupplement::CombiningGlagoliticLetterGlagoli => "combining glagolitic letter glagoli",
            GlagoliticSupplement::CombiningGlagoliticLetterDobro => "combining glagolitic letter dobro",
            GlagoliticSupplement::CombiningGlagoliticLetterYestu => "combining glagolitic letter yestu",
            GlagoliticSupplement::CombiningGlagoliticLetterZhivete => "combining glagolitic letter zhivete",
            GlagoliticSupplement::CombiningGlagoliticLetterZemlja => "combining glagolitic letter zemlja",
            GlagoliticSupplement::CombiningGlagoliticLetterIzhe => "combining glagolitic letter izhe",
            GlagoliticSupplement::CombiningGlagoliticLetterInitialIzhe => "combining glagolitic letter initial izhe",
            GlagoliticSupplement::CombiningGlagoliticLetterI => "combining glagolitic letter i",
            GlagoliticSupplement::CombiningGlagoliticLetterDjervi => "combining glagolitic letter djervi",
            GlagoliticSupplement::CombiningGlagoliticLetterKako => "combining glagolitic letter kako",
            GlagoliticSupplement::CombiningGlagoliticLetterLjudije => "combining glagolitic letter ljudije",
            GlagoliticSupplement::CombiningGlagoliticLetterMyslite => "combining glagolitic letter myslite",
            GlagoliticSupplement::CombiningGlagoliticLetterNashi => "combining glagolitic letter nashi",
            GlagoliticSupplement::CombiningGlagoliticLetterOnu => "combining glagolitic letter onu",
            GlagoliticSupplement::CombiningGlagoliticLetterPokoji => "combining glagolitic letter pokoji",
            GlagoliticSupplement::CombiningGlagoliticLetterRitsi => "combining glagolitic letter ritsi",
            GlagoliticSupplement::CombiningGlagoliticLetterSlovo => "combining glagolitic letter slovo",
            GlagoliticSupplement::CombiningGlagoliticLetterTvrido => "combining glagolitic letter tvrido",
            GlagoliticSupplement::CombiningGlagoliticLetterUku => "combining glagolitic letter uku",
            GlagoliticSupplement::CombiningGlagoliticLetterFritu => "combining glagolitic letter fritu",
            GlagoliticSupplement::CombiningGlagoliticLetterHeru => "combining glagolitic letter heru",
            GlagoliticSupplement::CombiningGlagoliticLetterShta => "combining glagolitic letter shta",
            GlagoliticSupplement::CombiningGlagoliticLetterTsi => "combining glagolitic letter tsi",
            GlagoliticSupplement::CombiningGlagoliticLetterChrivi => "combining glagolitic letter chrivi",
            GlagoliticSupplement::CombiningGlagoliticLetterSha => "combining glagolitic letter sha",
            GlagoliticSupplement::CombiningGlagoliticLetterYeru => "combining glagolitic letter yeru",
            GlagoliticSupplement::CombiningGlagoliticLetterYeri => "combining glagolitic letter yeri",
            GlagoliticSupplement::CombiningGlagoliticLetterYati => "combining glagolitic letter yati",
            GlagoliticSupplement::CombiningGlagoliticLetterYu => "combining glagolitic letter yu",
            GlagoliticSupplement::CombiningGlagoliticLetterSmallYus => "combining glagolitic letter small yus",
            GlagoliticSupplement::CombiningGlagoliticLetterYo => "combining glagolitic letter yo",
            GlagoliticSupplement::CombiningGlagoliticLetterIotatedSmallYus => "combining glagolitic letter iotated small yus",
            GlagoliticSupplement::CombiningGlagoliticLetterBigYus => "combining glagolitic letter big yus",
            GlagoliticSupplement::CombiningGlagoliticLetterIotatedBigYus => "combining glagolitic letter iotated big yus",
            GlagoliticSupplement::CombiningGlagoliticLetterFita => "combining glagolitic letter fita",
        }
    }
}
