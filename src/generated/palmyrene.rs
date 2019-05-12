/// \u{10860} → \u{1087f}\
///\
/// 𐡠 𐡡 𐡢 𐡣 𐡤 𐡥 𐡦 𐡧 𐡨 𐡩 𐡪 𐡫 𐡬 𐡭 𐡮 𐡯\
/// 𐡰 𐡱 𐡲 𐡳 𐡴 𐡵 𐡶 𐡷 𐡸 𐡹 𐡺 𐡻 𐡼 𐡽 𐡾\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{10860}: '𐡠'
    pub const LETTER_ALEPH: char = '𐡠';
    /// \u{10861}: '𐡡'
    pub const LETTER_BETH: char = '𐡡';
    /// \u{10862}: '𐡢'
    pub const LETTER_GIMEL: char = '𐡢';
    /// \u{10863}: '𐡣'
    pub const LETTER_DALETH: char = '𐡣';
    /// \u{10864}: '𐡤'
    pub const LETTER_HE: char = '𐡤';
    /// \u{10865}: '𐡥'
    pub const LETTER_WAW: char = '𐡥';
    /// \u{10866}: '𐡦'
    pub const LETTER_ZAYIN: char = '𐡦';
    /// \u{10867}: '𐡧'
    pub const LETTER_HETH: char = '𐡧';
    /// \u{10868}: '𐡨'
    pub const LETTER_TETH: char = '𐡨';
    /// \u{10869}: '𐡩'
    pub const LETTER_YODH: char = '𐡩';
    /// \u{1086a}: '𐡪'
    pub const LETTER_KAPH: char = '𐡪';
    /// \u{1086b}: '𐡫'
    pub const LETTER_LAMEDH: char = '𐡫';
    /// \u{1086c}: '𐡬'
    pub const LETTER_MEM: char = '𐡬';
    /// \u{1086d}: '𐡭'
    pub const LETTER_FINAL_NUN: char = '𐡭';
    /// \u{1086e}: '𐡮'
    pub const LETTER_NUN: char = '𐡮';
    /// \u{1086f}: '𐡯'
    pub const LETTER_SAMEKH: char = '𐡯';
    /// \u{10870}: '𐡰'
    pub const LETTER_AYIN: char = '𐡰';
    /// \u{10871}: '𐡱'
    pub const LETTER_PE: char = '𐡱';
    /// \u{10872}: '𐡲'
    pub const LETTER_SADHE: char = '𐡲';
    /// \u{10873}: '𐡳'
    pub const LETTER_QOPH: char = '𐡳';
    /// \u{10874}: '𐡴'
    pub const LETTER_RESH: char = '𐡴';
    /// \u{10875}: '𐡵'
    pub const LETTER_SHIN: char = '𐡵';
    /// \u{10876}: '𐡶'
    pub const LETTER_TAW: char = '𐡶';
    /// \u{10877}: '𐡷'
    pub const LEFT_DASH_POINTING_FLEURON: char = '𐡷';
    /// \u{10878}: '𐡸'
    pub const RIGHT_DASH_POINTING_FLEURON: char = '𐡸';
    /// \u{10879}: '𐡹'
    pub const NUMBER_ONE: char = '𐡹';
    /// \u{1087a}: '𐡺'
    pub const NUMBER_TWO: char = '𐡺';
    /// \u{1087b}: '𐡻'
    pub const NUMBER_THREE: char = '𐡻';
    /// \u{1087c}: '𐡼'
    pub const NUMBER_FOUR: char = '𐡼';
    /// \u{1087d}: '𐡽'
    pub const NUMBER_FIVE: char = '𐡽';
    /// \u{1087e}: '𐡾'
    pub const NUMBER_TEN: char = '𐡾';
}

/// An enum to represent all characters in the Palmyrene block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Palmyrene {
    /// \u{10860}: '𐡠'
    LetterAleph,
    /// \u{10861}: '𐡡'
    LetterBeth,
    /// \u{10862}: '𐡢'
    LetterGimel,
    /// \u{10863}: '𐡣'
    LetterDaleth,
    /// \u{10864}: '𐡤'
    LetterHe,
    /// \u{10865}: '𐡥'
    LetterWaw,
    /// \u{10866}: '𐡦'
    LetterZayin,
    /// \u{10867}: '𐡧'
    LetterHeth,
    /// \u{10868}: '𐡨'
    LetterTeth,
    /// \u{10869}: '𐡩'
    LetterYodh,
    /// \u{1086a}: '𐡪'
    LetterKaph,
    /// \u{1086b}: '𐡫'
    LetterLamedh,
    /// \u{1086c}: '𐡬'
    LetterMem,
    /// \u{1086d}: '𐡭'
    LetterFinalNun,
    /// \u{1086e}: '𐡮'
    LetterNun,
    /// \u{1086f}: '𐡯'
    LetterSamekh,
    /// \u{10870}: '𐡰'
    LetterAyin,
    /// \u{10871}: '𐡱'
    LetterPe,
    /// \u{10872}: '𐡲'
    LetterSadhe,
    /// \u{10873}: '𐡳'
    LetterQoph,
    /// \u{10874}: '𐡴'
    LetterResh,
    /// \u{10875}: '𐡵'
    LetterShin,
    /// \u{10876}: '𐡶'
    LetterTaw,
    /// \u{10877}: '𐡷'
    LeftDashPointingFleuron,
    /// \u{10878}: '𐡸'
    RightDashPointingFleuron,
    /// \u{10879}: '𐡹'
    NumberOne,
    /// \u{1087a}: '𐡺'
    NumberTwo,
    /// \u{1087b}: '𐡻'
    NumberThree,
    /// \u{1087c}: '𐡼'
    NumberFour,
    /// \u{1087d}: '𐡽'
    NumberFive,
    /// \u{1087e}: '𐡾'
    NumberTen,
}

impl Into<char> for Palmyrene {
    fn into(self) -> char {
        use constants::*;
        match self {
            Palmyrene::LetterAleph => LETTER_ALEPH,
            Palmyrene::LetterBeth => LETTER_BETH,
            Palmyrene::LetterGimel => LETTER_GIMEL,
            Palmyrene::LetterDaleth => LETTER_DALETH,
            Palmyrene::LetterHe => LETTER_HE,
            Palmyrene::LetterWaw => LETTER_WAW,
            Palmyrene::LetterZayin => LETTER_ZAYIN,
            Palmyrene::LetterHeth => LETTER_HETH,
            Palmyrene::LetterTeth => LETTER_TETH,
            Palmyrene::LetterYodh => LETTER_YODH,
            Palmyrene::LetterKaph => LETTER_KAPH,
            Palmyrene::LetterLamedh => LETTER_LAMEDH,
            Palmyrene::LetterMem => LETTER_MEM,
            Palmyrene::LetterFinalNun => LETTER_FINAL_NUN,
            Palmyrene::LetterNun => LETTER_NUN,
            Palmyrene::LetterSamekh => LETTER_SAMEKH,
            Palmyrene::LetterAyin => LETTER_AYIN,
            Palmyrene::LetterPe => LETTER_PE,
            Palmyrene::LetterSadhe => LETTER_SADHE,
            Palmyrene::LetterQoph => LETTER_QOPH,
            Palmyrene::LetterResh => LETTER_RESH,
            Palmyrene::LetterShin => LETTER_SHIN,
            Palmyrene::LetterTaw => LETTER_TAW,
            Palmyrene::LeftDashPointingFleuron => LEFT_DASH_POINTING_FLEURON,
            Palmyrene::RightDashPointingFleuron => RIGHT_DASH_POINTING_FLEURON,
            Palmyrene::NumberOne => NUMBER_ONE,
            Palmyrene::NumberTwo => NUMBER_TWO,
            Palmyrene::NumberThree => NUMBER_THREE,
            Palmyrene::NumberFour => NUMBER_FOUR,
            Palmyrene::NumberFive => NUMBER_FIVE,
            Palmyrene::NumberTen => NUMBER_TEN,
        }
    }
}

impl std::convert::TryFrom<char> for Palmyrene {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_ALEPH => Ok(Palmyrene::LetterAleph),
            LETTER_BETH => Ok(Palmyrene::LetterBeth),
            LETTER_GIMEL => Ok(Palmyrene::LetterGimel),
            LETTER_DALETH => Ok(Palmyrene::LetterDaleth),
            LETTER_HE => Ok(Palmyrene::LetterHe),
            LETTER_WAW => Ok(Palmyrene::LetterWaw),
            LETTER_ZAYIN => Ok(Palmyrene::LetterZayin),
            LETTER_HETH => Ok(Palmyrene::LetterHeth),
            LETTER_TETH => Ok(Palmyrene::LetterTeth),
            LETTER_YODH => Ok(Palmyrene::LetterYodh),
            LETTER_KAPH => Ok(Palmyrene::LetterKaph),
            LETTER_LAMEDH => Ok(Palmyrene::LetterLamedh),
            LETTER_MEM => Ok(Palmyrene::LetterMem),
            LETTER_FINAL_NUN => Ok(Palmyrene::LetterFinalNun),
            LETTER_NUN => Ok(Palmyrene::LetterNun),
            LETTER_SAMEKH => Ok(Palmyrene::LetterSamekh),
            LETTER_AYIN => Ok(Palmyrene::LetterAyin),
            LETTER_PE => Ok(Palmyrene::LetterPe),
            LETTER_SADHE => Ok(Palmyrene::LetterSadhe),
            LETTER_QOPH => Ok(Palmyrene::LetterQoph),
            LETTER_RESH => Ok(Palmyrene::LetterResh),
            LETTER_SHIN => Ok(Palmyrene::LetterShin),
            LETTER_TAW => Ok(Palmyrene::LetterTaw),
            LEFT_DASH_POINTING_FLEURON => Ok(Palmyrene::LeftDashPointingFleuron),
            RIGHT_DASH_POINTING_FLEURON => Ok(Palmyrene::RightDashPointingFleuron),
            NUMBER_ONE => Ok(Palmyrene::NumberOne),
            NUMBER_TWO => Ok(Palmyrene::NumberTwo),
            NUMBER_THREE => Ok(Palmyrene::NumberThree),
            NUMBER_FOUR => Ok(Palmyrene::NumberFour),
            NUMBER_FIVE => Ok(Palmyrene::NumberFive),
            NUMBER_TEN => Ok(Palmyrene::NumberTen),
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
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Palmyrene::LetterAleph
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Palmyrene::LetterAleph => "palmyrene letter aleph",
            Palmyrene::LetterBeth => "palmyrene letter beth",
            Palmyrene::LetterGimel => "palmyrene letter gimel",
            Palmyrene::LetterDaleth => "palmyrene letter daleth",
            Palmyrene::LetterHe => "palmyrene letter he",
            Palmyrene::LetterWaw => "palmyrene letter waw",
            Palmyrene::LetterZayin => "palmyrene letter zayin",
            Palmyrene::LetterHeth => "palmyrene letter heth",
            Palmyrene::LetterTeth => "palmyrene letter teth",
            Palmyrene::LetterYodh => "palmyrene letter yodh",
            Palmyrene::LetterKaph => "palmyrene letter kaph",
            Palmyrene::LetterLamedh => "palmyrene letter lamedh",
            Palmyrene::LetterMem => "palmyrene letter mem",
            Palmyrene::LetterFinalNun => "palmyrene letter final nun",
            Palmyrene::LetterNun => "palmyrene letter nun",
            Palmyrene::LetterSamekh => "palmyrene letter samekh",
            Palmyrene::LetterAyin => "palmyrene letter ayin",
            Palmyrene::LetterPe => "palmyrene letter pe",
            Palmyrene::LetterSadhe => "palmyrene letter sadhe",
            Palmyrene::LetterQoph => "palmyrene letter qoph",
            Palmyrene::LetterResh => "palmyrene letter resh",
            Palmyrene::LetterShin => "palmyrene letter shin",
            Palmyrene::LetterTaw => "palmyrene letter taw",
            Palmyrene::LeftDashPointingFleuron => "palmyrene left-pointing fleuron",
            Palmyrene::RightDashPointingFleuron => "palmyrene right-pointing fleuron",
            Palmyrene::NumberOne => "palmyrene number one",
            Palmyrene::NumberTwo => "palmyrene number two",
            Palmyrene::NumberThree => "palmyrene number three",
            Palmyrene::NumberFour => "palmyrene number four",
            Palmyrene::NumberFive => "palmyrene number five",
            Palmyrene::NumberTen => "palmyrene number ten",
        }
    }
}
