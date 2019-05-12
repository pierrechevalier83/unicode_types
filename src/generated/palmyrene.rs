/// \u{10860} → \u{1087f}\
///\
/// 𐡠 𐡡 𐡢 𐡣 𐡤 𐡥 𐡦 𐡧 𐡨 𐡩 𐡪 𐡫 𐡬 𐡭 𐡮 𐡯
/// 𐡰 𐡱 𐡲 𐡳 𐡴 𐡵 𐡶 𐡷 𐡸 𐡹 𐡺 𐡻 𐡼 𐡽 𐡾
pub mod constants {
    /// \u{10860}: '𐡠'
    pub const PALMYRENE_LETTER_ALEPH: char = '𐡠';
    /// \u{10861}: '𐡡'
    pub const PALMYRENE_LETTER_BETH: char = '𐡡';
    /// \u{10862}: '𐡢'
    pub const PALMYRENE_LETTER_GIMEL: char = '𐡢';
    /// \u{10863}: '𐡣'
    pub const PALMYRENE_LETTER_DALETH: char = '𐡣';
    /// \u{10864}: '𐡤'
    pub const PALMYRENE_LETTER_HE: char = '𐡤';
    /// \u{10865}: '𐡥'
    pub const PALMYRENE_LETTER_WAW: char = '𐡥';
    /// \u{10866}: '𐡦'
    pub const PALMYRENE_LETTER_ZAYIN: char = '𐡦';
    /// \u{10867}: '𐡧'
    pub const PALMYRENE_LETTER_HETH: char = '𐡧';
    /// \u{10868}: '𐡨'
    pub const PALMYRENE_LETTER_TETH: char = '𐡨';
    /// \u{10869}: '𐡩'
    pub const PALMYRENE_LETTER_YODH: char = '𐡩';
    /// \u{1086a}: '𐡪'
    pub const PALMYRENE_LETTER_KAPH: char = '𐡪';
    /// \u{1086b}: '𐡫'
    pub const PALMYRENE_LETTER_LAMEDH: char = '𐡫';
    /// \u{1086c}: '𐡬'
    pub const PALMYRENE_LETTER_MEM: char = '𐡬';
    /// \u{1086d}: '𐡭'
    pub const PALMYRENE_LETTER_FINAL_NUN: char = '𐡭';
    /// \u{1086e}: '𐡮'
    pub const PALMYRENE_LETTER_NUN: char = '𐡮';
    /// \u{1086f}: '𐡯'
    pub const PALMYRENE_LETTER_SAMEKH: char = '𐡯';
    /// \u{10870}: '𐡰'
    pub const PALMYRENE_LETTER_AYIN: char = '𐡰';
    /// \u{10871}: '𐡱'
    pub const PALMYRENE_LETTER_PE: char = '𐡱';
    /// \u{10872}: '𐡲'
    pub const PALMYRENE_LETTER_SADHE: char = '𐡲';
    /// \u{10873}: '𐡳'
    pub const PALMYRENE_LETTER_QOPH: char = '𐡳';
    /// \u{10874}: '𐡴'
    pub const PALMYRENE_LETTER_RESH: char = '𐡴';
    /// \u{10875}: '𐡵'
    pub const PALMYRENE_LETTER_SHIN: char = '𐡵';
    /// \u{10876}: '𐡶'
    pub const PALMYRENE_LETTER_TAW: char = '𐡶';
    /// \u{10877}: '𐡷'
    pub const PALMYRENE_LEFT_DASH_POINTING_FLEURON: char = '𐡷';
    /// \u{10878}: '𐡸'
    pub const PALMYRENE_RIGHT_DASH_POINTING_FLEURON: char = '𐡸';
    /// \u{10879}: '𐡹'
    pub const PALMYRENE_NUMBER_ONE: char = '𐡹';
    /// \u{1087a}: '𐡺'
    pub const PALMYRENE_NUMBER_TWO: char = '𐡺';
    /// \u{1087b}: '𐡻'
    pub const PALMYRENE_NUMBER_THREE: char = '𐡻';
    /// \u{1087c}: '𐡼'
    pub const PALMYRENE_NUMBER_FOUR: char = '𐡼';
    /// \u{1087d}: '𐡽'
    pub const PALMYRENE_NUMBER_FIVE: char = '𐡽';
    /// \u{1087e}: '𐡾'
    pub const PALMYRENE_NUMBER_TEN: char = '𐡾';
}

/// \u{10860} → \u{1087f}\
///\
/// 𐡠 𐡡 𐡢 𐡣 𐡤 𐡥 𐡦 𐡧 𐡨 𐡩 𐡪 𐡫 𐡬 𐡭 𐡮 𐡯
/// 𐡰 𐡱 𐡲 𐡳 𐡴 𐡵 𐡶 𐡷 𐡸 𐡹 𐡺 𐡻 𐡼 𐡽 𐡾
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Palmyrene {
    /// \u{10860}: '𐡠'
    PalmyreneLetterAleph,
    /// \u{10861}: '𐡡'
    PalmyreneLetterBeth,
    /// \u{10862}: '𐡢'
    PalmyreneLetterGimel,
    /// \u{10863}: '𐡣'
    PalmyreneLetterDaleth,
    /// \u{10864}: '𐡤'
    PalmyreneLetterHe,
    /// \u{10865}: '𐡥'
    PalmyreneLetterWaw,
    /// \u{10866}: '𐡦'
    PalmyreneLetterZayin,
    /// \u{10867}: '𐡧'
    PalmyreneLetterHeth,
    /// \u{10868}: '𐡨'
    PalmyreneLetterTeth,
    /// \u{10869}: '𐡩'
    PalmyreneLetterYodh,
    /// \u{1086a}: '𐡪'
    PalmyreneLetterKaph,
    /// \u{1086b}: '𐡫'
    PalmyreneLetterLamedh,
    /// \u{1086c}: '𐡬'
    PalmyreneLetterMem,
    /// \u{1086d}: '𐡭'
    PalmyreneLetterFinalNun,
    /// \u{1086e}: '𐡮'
    PalmyreneLetterNun,
    /// \u{1086f}: '𐡯'
    PalmyreneLetterSamekh,
    /// \u{10870}: '𐡰'
    PalmyreneLetterAyin,
    /// \u{10871}: '𐡱'
    PalmyreneLetterPe,
    /// \u{10872}: '𐡲'
    PalmyreneLetterSadhe,
    /// \u{10873}: '𐡳'
    PalmyreneLetterQoph,
    /// \u{10874}: '𐡴'
    PalmyreneLetterResh,
    /// \u{10875}: '𐡵'
    PalmyreneLetterShin,
    /// \u{10876}: '𐡶'
    PalmyreneLetterTaw,
    /// \u{10877}: '𐡷'
    PalmyreneLeftDashPointingFleuron,
    /// \u{10878}: '𐡸'
    PalmyreneRightDashPointingFleuron,
    /// \u{10879}: '𐡹'
    PalmyreneNumberOne,
    /// \u{1087a}: '𐡺'
    PalmyreneNumberTwo,
    /// \u{1087b}: '𐡻'
    PalmyreneNumberThree,
    /// \u{1087c}: '𐡼'
    PalmyreneNumberFour,
    /// \u{1087d}: '𐡽'
    PalmyreneNumberFive,
    /// \u{1087e}: '𐡾'
    PalmyreneNumberTen,
}

impl Into<char> for Palmyrene {
    fn into(self) -> char {
        use constants::*;
        match self {
            Palmyrene::PalmyreneLetterAleph => PALMYRENE_LETTER_ALEPH,
            Palmyrene::PalmyreneLetterBeth => PALMYRENE_LETTER_BETH,
            Palmyrene::PalmyreneLetterGimel => PALMYRENE_LETTER_GIMEL,
            Palmyrene::PalmyreneLetterDaleth => PALMYRENE_LETTER_DALETH,
            Palmyrene::PalmyreneLetterHe => PALMYRENE_LETTER_HE,
            Palmyrene::PalmyreneLetterWaw => PALMYRENE_LETTER_WAW,
            Palmyrene::PalmyreneLetterZayin => PALMYRENE_LETTER_ZAYIN,
            Palmyrene::PalmyreneLetterHeth => PALMYRENE_LETTER_HETH,
            Palmyrene::PalmyreneLetterTeth => PALMYRENE_LETTER_TETH,
            Palmyrene::PalmyreneLetterYodh => PALMYRENE_LETTER_YODH,
            Palmyrene::PalmyreneLetterKaph => PALMYRENE_LETTER_KAPH,
            Palmyrene::PalmyreneLetterLamedh => PALMYRENE_LETTER_LAMEDH,
            Palmyrene::PalmyreneLetterMem => PALMYRENE_LETTER_MEM,
            Palmyrene::PalmyreneLetterFinalNun => PALMYRENE_LETTER_FINAL_NUN,
            Palmyrene::PalmyreneLetterNun => PALMYRENE_LETTER_NUN,
            Palmyrene::PalmyreneLetterSamekh => PALMYRENE_LETTER_SAMEKH,
            Palmyrene::PalmyreneLetterAyin => PALMYRENE_LETTER_AYIN,
            Palmyrene::PalmyreneLetterPe => PALMYRENE_LETTER_PE,
            Palmyrene::PalmyreneLetterSadhe => PALMYRENE_LETTER_SADHE,
            Palmyrene::PalmyreneLetterQoph => PALMYRENE_LETTER_QOPH,
            Palmyrene::PalmyreneLetterResh => PALMYRENE_LETTER_RESH,
            Palmyrene::PalmyreneLetterShin => PALMYRENE_LETTER_SHIN,
            Palmyrene::PalmyreneLetterTaw => PALMYRENE_LETTER_TAW,
            Palmyrene::PalmyreneLeftDashPointingFleuron => PALMYRENE_LEFT_DASH_POINTING_FLEURON,
            Palmyrene::PalmyreneRightDashPointingFleuron => PALMYRENE_RIGHT_DASH_POINTING_FLEURON,
            Palmyrene::PalmyreneNumberOne => PALMYRENE_NUMBER_ONE,
            Palmyrene::PalmyreneNumberTwo => PALMYRENE_NUMBER_TWO,
            Palmyrene::PalmyreneNumberThree => PALMYRENE_NUMBER_THREE,
            Palmyrene::PalmyreneNumberFour => PALMYRENE_NUMBER_FOUR,
            Palmyrene::PalmyreneNumberFive => PALMYRENE_NUMBER_FIVE,
            Palmyrene::PalmyreneNumberTen => PALMYRENE_NUMBER_TEN,
        }
    }
}

impl std::convert::TryFrom<char> for Palmyrene {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            PALMYRENE_LETTER_ALEPH => Ok(Palmyrene::PalmyreneLetterAleph),
            PALMYRENE_LETTER_BETH => Ok(Palmyrene::PalmyreneLetterBeth),
            PALMYRENE_LETTER_GIMEL => Ok(Palmyrene::PalmyreneLetterGimel),
            PALMYRENE_LETTER_DALETH => Ok(Palmyrene::PalmyreneLetterDaleth),
            PALMYRENE_LETTER_HE => Ok(Palmyrene::PalmyreneLetterHe),
            PALMYRENE_LETTER_WAW => Ok(Palmyrene::PalmyreneLetterWaw),
            PALMYRENE_LETTER_ZAYIN => Ok(Palmyrene::PalmyreneLetterZayin),
            PALMYRENE_LETTER_HETH => Ok(Palmyrene::PalmyreneLetterHeth),
            PALMYRENE_LETTER_TETH => Ok(Palmyrene::PalmyreneLetterTeth),
            PALMYRENE_LETTER_YODH => Ok(Palmyrene::PalmyreneLetterYodh),
            PALMYRENE_LETTER_KAPH => Ok(Palmyrene::PalmyreneLetterKaph),
            PALMYRENE_LETTER_LAMEDH => Ok(Palmyrene::PalmyreneLetterLamedh),
            PALMYRENE_LETTER_MEM => Ok(Palmyrene::PalmyreneLetterMem),
            PALMYRENE_LETTER_FINAL_NUN => Ok(Palmyrene::PalmyreneLetterFinalNun),
            PALMYRENE_LETTER_NUN => Ok(Palmyrene::PalmyreneLetterNun),
            PALMYRENE_LETTER_SAMEKH => Ok(Palmyrene::PalmyreneLetterSamekh),
            PALMYRENE_LETTER_AYIN => Ok(Palmyrene::PalmyreneLetterAyin),
            PALMYRENE_LETTER_PE => Ok(Palmyrene::PalmyreneLetterPe),
            PALMYRENE_LETTER_SADHE => Ok(Palmyrene::PalmyreneLetterSadhe),
            PALMYRENE_LETTER_QOPH => Ok(Palmyrene::PalmyreneLetterQoph),
            PALMYRENE_LETTER_RESH => Ok(Palmyrene::PalmyreneLetterResh),
            PALMYRENE_LETTER_SHIN => Ok(Palmyrene::PalmyreneLetterShin),
            PALMYRENE_LETTER_TAW => Ok(Palmyrene::PalmyreneLetterTaw),
            PALMYRENE_LEFT_DASH_POINTING_FLEURON => Ok(Palmyrene::PalmyreneLeftDashPointingFleuron),
            PALMYRENE_RIGHT_DASH_POINTING_FLEURON => Ok(Palmyrene::PalmyreneRightDashPointingFleuron),
            PALMYRENE_NUMBER_ONE => Ok(Palmyrene::PalmyreneNumberOne),
            PALMYRENE_NUMBER_TWO => Ok(Palmyrene::PalmyreneNumberTwo),
            PALMYRENE_NUMBER_THREE => Ok(Palmyrene::PalmyreneNumberThree),
            PALMYRENE_NUMBER_FOUR => Ok(Palmyrene::PalmyreneNumberFour),
            PALMYRENE_NUMBER_FIVE => Ok(Palmyrene::PalmyreneNumberFive),
            PALMYRENE_NUMBER_TEN => Ok(Palmyrene::PalmyreneNumberTen),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Palmyrene {
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

impl std::convert::TryFrom<u32> for Palmyrene {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Palmyrene {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Palmyrene {
    /// The character with the lowest index this unicode block
    pub fn new() -> Self {
        Palmyrene::PalmyreneLetterAleph
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Palmyrene::PalmyreneLetterAleph => "palmyrene letter aleph",
            Palmyrene::PalmyreneLetterBeth => "palmyrene letter beth",
            Palmyrene::PalmyreneLetterGimel => "palmyrene letter gimel",
            Palmyrene::PalmyreneLetterDaleth => "palmyrene letter daleth",
            Palmyrene::PalmyreneLetterHe => "palmyrene letter he",
            Palmyrene::PalmyreneLetterWaw => "palmyrene letter waw",
            Palmyrene::PalmyreneLetterZayin => "palmyrene letter zayin",
            Palmyrene::PalmyreneLetterHeth => "palmyrene letter heth",
            Palmyrene::PalmyreneLetterTeth => "palmyrene letter teth",
            Palmyrene::PalmyreneLetterYodh => "palmyrene letter yodh",
            Palmyrene::PalmyreneLetterKaph => "palmyrene letter kaph",
            Palmyrene::PalmyreneLetterLamedh => "palmyrene letter lamedh",
            Palmyrene::PalmyreneLetterMem => "palmyrene letter mem",
            Palmyrene::PalmyreneLetterFinalNun => "palmyrene letter final nun",
            Palmyrene::PalmyreneLetterNun => "palmyrene letter nun",
            Palmyrene::PalmyreneLetterSamekh => "palmyrene letter samekh",
            Palmyrene::PalmyreneLetterAyin => "palmyrene letter ayin",
            Palmyrene::PalmyreneLetterPe => "palmyrene letter pe",
            Palmyrene::PalmyreneLetterSadhe => "palmyrene letter sadhe",
            Palmyrene::PalmyreneLetterQoph => "palmyrene letter qoph",
            Palmyrene::PalmyreneLetterResh => "palmyrene letter resh",
            Palmyrene::PalmyreneLetterShin => "palmyrene letter shin",
            Palmyrene::PalmyreneLetterTaw => "palmyrene letter taw",
            Palmyrene::PalmyreneLeftDashPointingFleuron => "palmyrene left-pointing fleuron",
            Palmyrene::PalmyreneRightDashPointingFleuron => "palmyrene right-pointing fleuron",
            Palmyrene::PalmyreneNumberOne => "palmyrene number one",
            Palmyrene::PalmyreneNumberTwo => "palmyrene number two",
            Palmyrene::PalmyreneNumberThree => "palmyrene number three",
            Palmyrene::PalmyreneNumberFour => "palmyrene number four",
            Palmyrene::PalmyreneNumberFive => "palmyrene number five",
            Palmyrene::PalmyreneNumberTen => "palmyrene number ten",
        }
    }
}
