/// A number of constants to give a name to all characters in this block.
mod constants {
    /// \u{1e000}: '𞀀'
    pub const COMBINING_GLAGOLITIC_LETTER_AZU: char = '𞀀';
    /// \u{1e001}: '𞀁'
    pub const COMBINING_GLAGOLITIC_LETTER_BUKY: char = '𞀁';
    /// \u{1e002}: '𞀂'
    pub const COMBINING_GLAGOLITIC_LETTER_VEDE: char = '𞀂';
    /// \u{1e003}: '𞀃'
    pub const COMBINING_GLAGOLITIC_LETTER_GLAGOLI: char = '𞀃';
    /// \u{1e004}: '𞀄'
    pub const COMBINING_GLAGOLITIC_LETTER_DOBRO: char = '𞀄';
    /// \u{1e005}: '𞀅'
    pub const COMBINING_GLAGOLITIC_LETTER_YESTU: char = '𞀅';
    /// \u{1e006}: '𞀆'
    pub const COMBINING_GLAGOLITIC_LETTER_ZHIVETE: char = '𞀆';
    /// \u{1e008}: '𞀈'
    pub const COMBINING_GLAGOLITIC_LETTER_ZEMLJA: char = '𞀈';
    /// \u{1e009}: '𞀉'
    pub const COMBINING_GLAGOLITIC_LETTER_IZHE: char = '𞀉';
    /// \u{1e00a}: '𞀊'
    pub const COMBINING_GLAGOLITIC_LETTER_INITIAL_IZHE: char = '𞀊';
    /// \u{1e00b}: '𞀋'
    pub const COMBINING_GLAGOLITIC_LETTER_I: char = '𞀋';
    /// \u{1e00c}: '𞀌'
    pub const COMBINING_GLAGOLITIC_LETTER_DJERVI: char = '𞀌';
    /// \u{1e00d}: '𞀍'
    pub const COMBINING_GLAGOLITIC_LETTER_KAKO: char = '𞀍';
    /// \u{1e00e}: '𞀎'
    pub const COMBINING_GLAGOLITIC_LETTER_LJUDIJE: char = '𞀎';
    /// \u{1e00f}: '𞀏'
    pub const COMBINING_GLAGOLITIC_LETTER_MYSLITE: char = '𞀏';
    /// \u{1e010}: '𞀐'
    pub const COMBINING_GLAGOLITIC_LETTER_NASHI: char = '𞀐';
    /// \u{1e011}: '𞀑'
    pub const COMBINING_GLAGOLITIC_LETTER_ONU: char = '𞀑';
    /// \u{1e012}: '𞀒'
    pub const COMBINING_GLAGOLITIC_LETTER_POKOJI: char = '𞀒';
    /// \u{1e013}: '𞀓'
    pub const COMBINING_GLAGOLITIC_LETTER_RITSI: char = '𞀓';
    /// \u{1e014}: '𞀔'
    pub const COMBINING_GLAGOLITIC_LETTER_SLOVO: char = '𞀔';
    /// \u{1e015}: '𞀕'
    pub const COMBINING_GLAGOLITIC_LETTER_TVRIDO: char = '𞀕';
    /// \u{1e016}: '𞀖'
    pub const COMBINING_GLAGOLITIC_LETTER_UKU: char = '𞀖';
    /// \u{1e017}: '𞀗'
    pub const COMBINING_GLAGOLITIC_LETTER_FRITU: char = '𞀗';
    /// \u{1e018}: '𞀘'
    pub const COMBINING_GLAGOLITIC_LETTER_HERU: char = '𞀘';
    /// \u{1e01b}: '𞀛'
    pub const COMBINING_GLAGOLITIC_LETTER_SHTA: char = '𞀛';
    /// \u{1e01c}: '𞀜'
    pub const COMBINING_GLAGOLITIC_LETTER_TSI: char = '𞀜';
    /// \u{1e01d}: '𞀝'
    pub const COMBINING_GLAGOLITIC_LETTER_CHRIVI: char = '𞀝';
    /// \u{1e01e}: '𞀞'
    pub const COMBINING_GLAGOLITIC_LETTER_SHA: char = '𞀞';
    /// \u{1e01f}: '𞀟'
    pub const COMBINING_GLAGOLITIC_LETTER_YERU: char = '𞀟';
    /// \u{1e020}: '𞀠'
    pub const COMBINING_GLAGOLITIC_LETTER_YERI: char = '𞀠';
    /// \u{1e021}: '𞀡'
    pub const COMBINING_GLAGOLITIC_LETTER_YATI: char = '𞀡';
    /// \u{1e023}: '𞀣'
    pub const COMBINING_GLAGOLITIC_LETTER_YU: char = '𞀣';
    /// \u{1e024}: '𞀤'
    pub const COMBINING_GLAGOLITIC_LETTER_SMALL_YUS: char = '𞀤';
    /// \u{1e026}: '𞀦'
    pub const COMBINING_GLAGOLITIC_LETTER_YO: char = '𞀦';
    /// \u{1e027}: '𞀧'
    pub const COMBINING_GLAGOLITIC_LETTER_IOTATED_SMALL_YUS: char = '𞀧';
    /// \u{1e028}: '𞀨'
    pub const COMBINING_GLAGOLITIC_LETTER_BIG_YUS: char = '𞀨';
    /// \u{1e029}: '𞀩'
    pub const COMBINING_GLAGOLITIC_LETTER_IOTATED_BIG_YUS: char = '𞀩';
    /// \u{1e02a}: '𞀪'
    pub const COMBINING_GLAGOLITIC_LETTER_FITA: char = '𞀪';
}

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
        use constants::*;
        match self {
            GlagoliticSupplement::CombiningGlagoliticLetterAzu => COMBINING_GLAGOLITIC_LETTER_AZU,
            GlagoliticSupplement::CombiningGlagoliticLetterBuky => COMBINING_GLAGOLITIC_LETTER_BUKY,
            GlagoliticSupplement::CombiningGlagoliticLetterVede => COMBINING_GLAGOLITIC_LETTER_VEDE,
            GlagoliticSupplement::CombiningGlagoliticLetterGlagoli => COMBINING_GLAGOLITIC_LETTER_GLAGOLI,
            GlagoliticSupplement::CombiningGlagoliticLetterDobro => COMBINING_GLAGOLITIC_LETTER_DOBRO,
            GlagoliticSupplement::CombiningGlagoliticLetterYestu => COMBINING_GLAGOLITIC_LETTER_YESTU,
            GlagoliticSupplement::CombiningGlagoliticLetterZhivete => COMBINING_GLAGOLITIC_LETTER_ZHIVETE,
            GlagoliticSupplement::CombiningGlagoliticLetterZemlja => COMBINING_GLAGOLITIC_LETTER_ZEMLJA,
            GlagoliticSupplement::CombiningGlagoliticLetterIzhe => COMBINING_GLAGOLITIC_LETTER_IZHE,
            GlagoliticSupplement::CombiningGlagoliticLetterInitialIzhe => COMBINING_GLAGOLITIC_LETTER_INITIAL_IZHE,
            GlagoliticSupplement::CombiningGlagoliticLetterI => COMBINING_GLAGOLITIC_LETTER_I,
            GlagoliticSupplement::CombiningGlagoliticLetterDjervi => COMBINING_GLAGOLITIC_LETTER_DJERVI,
            GlagoliticSupplement::CombiningGlagoliticLetterKako => COMBINING_GLAGOLITIC_LETTER_KAKO,
            GlagoliticSupplement::CombiningGlagoliticLetterLjudije => COMBINING_GLAGOLITIC_LETTER_LJUDIJE,
            GlagoliticSupplement::CombiningGlagoliticLetterMyslite => COMBINING_GLAGOLITIC_LETTER_MYSLITE,
            GlagoliticSupplement::CombiningGlagoliticLetterNashi => COMBINING_GLAGOLITIC_LETTER_NASHI,
            GlagoliticSupplement::CombiningGlagoliticLetterOnu => COMBINING_GLAGOLITIC_LETTER_ONU,
            GlagoliticSupplement::CombiningGlagoliticLetterPokoji => COMBINING_GLAGOLITIC_LETTER_POKOJI,
            GlagoliticSupplement::CombiningGlagoliticLetterRitsi => COMBINING_GLAGOLITIC_LETTER_RITSI,
            GlagoliticSupplement::CombiningGlagoliticLetterSlovo => COMBINING_GLAGOLITIC_LETTER_SLOVO,
            GlagoliticSupplement::CombiningGlagoliticLetterTvrido => COMBINING_GLAGOLITIC_LETTER_TVRIDO,
            GlagoliticSupplement::CombiningGlagoliticLetterUku => COMBINING_GLAGOLITIC_LETTER_UKU,
            GlagoliticSupplement::CombiningGlagoliticLetterFritu => COMBINING_GLAGOLITIC_LETTER_FRITU,
            GlagoliticSupplement::CombiningGlagoliticLetterHeru => COMBINING_GLAGOLITIC_LETTER_HERU,
            GlagoliticSupplement::CombiningGlagoliticLetterShta => COMBINING_GLAGOLITIC_LETTER_SHTA,
            GlagoliticSupplement::CombiningGlagoliticLetterTsi => COMBINING_GLAGOLITIC_LETTER_TSI,
            GlagoliticSupplement::CombiningGlagoliticLetterChrivi => COMBINING_GLAGOLITIC_LETTER_CHRIVI,
            GlagoliticSupplement::CombiningGlagoliticLetterSha => COMBINING_GLAGOLITIC_LETTER_SHA,
            GlagoliticSupplement::CombiningGlagoliticLetterYeru => COMBINING_GLAGOLITIC_LETTER_YERU,
            GlagoliticSupplement::CombiningGlagoliticLetterYeri => COMBINING_GLAGOLITIC_LETTER_YERI,
            GlagoliticSupplement::CombiningGlagoliticLetterYati => COMBINING_GLAGOLITIC_LETTER_YATI,
            GlagoliticSupplement::CombiningGlagoliticLetterYu => COMBINING_GLAGOLITIC_LETTER_YU,
            GlagoliticSupplement::CombiningGlagoliticLetterSmallYus => COMBINING_GLAGOLITIC_LETTER_SMALL_YUS,
            GlagoliticSupplement::CombiningGlagoliticLetterYo => COMBINING_GLAGOLITIC_LETTER_YO,
            GlagoliticSupplement::CombiningGlagoliticLetterIotatedSmallYus => COMBINING_GLAGOLITIC_LETTER_IOTATED_SMALL_YUS,
            GlagoliticSupplement::CombiningGlagoliticLetterBigYus => COMBINING_GLAGOLITIC_LETTER_BIG_YUS,
            GlagoliticSupplement::CombiningGlagoliticLetterIotatedBigYus => COMBINING_GLAGOLITIC_LETTER_IOTATED_BIG_YUS,
            GlagoliticSupplement::CombiningGlagoliticLetterFita => COMBINING_GLAGOLITIC_LETTER_FITA,
        }
    }
}

impl std::convert::TryFrom<char> for GlagoliticSupplement {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            COMBINING_GLAGOLITIC_LETTER_AZU => Ok(GlagoliticSupplement::CombiningGlagoliticLetterAzu),
            COMBINING_GLAGOLITIC_LETTER_BUKY => Ok(GlagoliticSupplement::CombiningGlagoliticLetterBuky),
            COMBINING_GLAGOLITIC_LETTER_VEDE => Ok(GlagoliticSupplement::CombiningGlagoliticLetterVede),
            COMBINING_GLAGOLITIC_LETTER_GLAGOLI => Ok(GlagoliticSupplement::CombiningGlagoliticLetterGlagoli),
            COMBINING_GLAGOLITIC_LETTER_DOBRO => Ok(GlagoliticSupplement::CombiningGlagoliticLetterDobro),
            COMBINING_GLAGOLITIC_LETTER_YESTU => Ok(GlagoliticSupplement::CombiningGlagoliticLetterYestu),
            COMBINING_GLAGOLITIC_LETTER_ZHIVETE => Ok(GlagoliticSupplement::CombiningGlagoliticLetterZhivete),
            COMBINING_GLAGOLITIC_LETTER_ZEMLJA => Ok(GlagoliticSupplement::CombiningGlagoliticLetterZemlja),
            COMBINING_GLAGOLITIC_LETTER_IZHE => Ok(GlagoliticSupplement::CombiningGlagoliticLetterIzhe),
            COMBINING_GLAGOLITIC_LETTER_INITIAL_IZHE => Ok(GlagoliticSupplement::CombiningGlagoliticLetterInitialIzhe),
            COMBINING_GLAGOLITIC_LETTER_I => Ok(GlagoliticSupplement::CombiningGlagoliticLetterI),
            COMBINING_GLAGOLITIC_LETTER_DJERVI => Ok(GlagoliticSupplement::CombiningGlagoliticLetterDjervi),
            COMBINING_GLAGOLITIC_LETTER_KAKO => Ok(GlagoliticSupplement::CombiningGlagoliticLetterKako),
            COMBINING_GLAGOLITIC_LETTER_LJUDIJE => Ok(GlagoliticSupplement::CombiningGlagoliticLetterLjudije),
            COMBINING_GLAGOLITIC_LETTER_MYSLITE => Ok(GlagoliticSupplement::CombiningGlagoliticLetterMyslite),
            COMBINING_GLAGOLITIC_LETTER_NASHI => Ok(GlagoliticSupplement::CombiningGlagoliticLetterNashi),
            COMBINING_GLAGOLITIC_LETTER_ONU => Ok(GlagoliticSupplement::CombiningGlagoliticLetterOnu),
            COMBINING_GLAGOLITIC_LETTER_POKOJI => Ok(GlagoliticSupplement::CombiningGlagoliticLetterPokoji),
            COMBINING_GLAGOLITIC_LETTER_RITSI => Ok(GlagoliticSupplement::CombiningGlagoliticLetterRitsi),
            COMBINING_GLAGOLITIC_LETTER_SLOVO => Ok(GlagoliticSupplement::CombiningGlagoliticLetterSlovo),
            COMBINING_GLAGOLITIC_LETTER_TVRIDO => Ok(GlagoliticSupplement::CombiningGlagoliticLetterTvrido),
            COMBINING_GLAGOLITIC_LETTER_UKU => Ok(GlagoliticSupplement::CombiningGlagoliticLetterUku),
            COMBINING_GLAGOLITIC_LETTER_FRITU => Ok(GlagoliticSupplement::CombiningGlagoliticLetterFritu),
            COMBINING_GLAGOLITIC_LETTER_HERU => Ok(GlagoliticSupplement::CombiningGlagoliticLetterHeru),
            COMBINING_GLAGOLITIC_LETTER_SHTA => Ok(GlagoliticSupplement::CombiningGlagoliticLetterShta),
            COMBINING_GLAGOLITIC_LETTER_TSI => Ok(GlagoliticSupplement::CombiningGlagoliticLetterTsi),
            COMBINING_GLAGOLITIC_LETTER_CHRIVI => Ok(GlagoliticSupplement::CombiningGlagoliticLetterChrivi),
            COMBINING_GLAGOLITIC_LETTER_SHA => Ok(GlagoliticSupplement::CombiningGlagoliticLetterSha),
            COMBINING_GLAGOLITIC_LETTER_YERU => Ok(GlagoliticSupplement::CombiningGlagoliticLetterYeru),
            COMBINING_GLAGOLITIC_LETTER_YERI => Ok(GlagoliticSupplement::CombiningGlagoliticLetterYeri),
            COMBINING_GLAGOLITIC_LETTER_YATI => Ok(GlagoliticSupplement::CombiningGlagoliticLetterYati),
            COMBINING_GLAGOLITIC_LETTER_YU => Ok(GlagoliticSupplement::CombiningGlagoliticLetterYu),
            COMBINING_GLAGOLITIC_LETTER_SMALL_YUS => Ok(GlagoliticSupplement::CombiningGlagoliticLetterSmallYus),
            COMBINING_GLAGOLITIC_LETTER_YO => Ok(GlagoliticSupplement::CombiningGlagoliticLetterYo),
            COMBINING_GLAGOLITIC_LETTER_IOTATED_SMALL_YUS => Ok(GlagoliticSupplement::CombiningGlagoliticLetterIotatedSmallYus),
            COMBINING_GLAGOLITIC_LETTER_BIG_YUS => Ok(GlagoliticSupplement::CombiningGlagoliticLetterBigYus),
            COMBINING_GLAGOLITIC_LETTER_IOTATED_BIG_YUS => Ok(GlagoliticSupplement::CombiningGlagoliticLetterIotatedBigYus),
            COMBINING_GLAGOLITIC_LETTER_FITA => Ok(GlagoliticSupplement::CombiningGlagoliticLetterFita),
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
