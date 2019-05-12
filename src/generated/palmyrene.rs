
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
        match self {
            Palmyrene::LetterAleph => '𐡠',
            Palmyrene::LetterBeth => '𐡡',
            Palmyrene::LetterGimel => '𐡢',
            Palmyrene::LetterDaleth => '𐡣',
            Palmyrene::LetterHe => '𐡤',
            Palmyrene::LetterWaw => '𐡥',
            Palmyrene::LetterZayin => '𐡦',
            Palmyrene::LetterHeth => '𐡧',
            Palmyrene::LetterTeth => '𐡨',
            Palmyrene::LetterYodh => '𐡩',
            Palmyrene::LetterKaph => '𐡪',
            Palmyrene::LetterLamedh => '𐡫',
            Palmyrene::LetterMem => '𐡬',
            Palmyrene::LetterFinalNun => '𐡭',
            Palmyrene::LetterNun => '𐡮',
            Palmyrene::LetterSamekh => '𐡯',
            Palmyrene::LetterAyin => '𐡰',
            Palmyrene::LetterPe => '𐡱',
            Palmyrene::LetterSadhe => '𐡲',
            Palmyrene::LetterQoph => '𐡳',
            Palmyrene::LetterResh => '𐡴',
            Palmyrene::LetterShin => '𐡵',
            Palmyrene::LetterTaw => '𐡶',
            Palmyrene::LeftDashPointingFleuron => '𐡷',
            Palmyrene::RightDashPointingFleuron => '𐡸',
            Palmyrene::NumberOne => '𐡹',
            Palmyrene::NumberTwo => '𐡺',
            Palmyrene::NumberThree => '𐡻',
            Palmyrene::NumberFour => '𐡼',
            Palmyrene::NumberFive => '𐡽',
            Palmyrene::NumberTen => '𐡾',
        }
    }
}

impl std::convert::TryFrom<char> for Palmyrene {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𐡠' => Ok(Palmyrene::LetterAleph),
            '𐡡' => Ok(Palmyrene::LetterBeth),
            '𐡢' => Ok(Palmyrene::LetterGimel),
            '𐡣' => Ok(Palmyrene::LetterDaleth),
            '𐡤' => Ok(Palmyrene::LetterHe),
            '𐡥' => Ok(Palmyrene::LetterWaw),
            '𐡦' => Ok(Palmyrene::LetterZayin),
            '𐡧' => Ok(Palmyrene::LetterHeth),
            '𐡨' => Ok(Palmyrene::LetterTeth),
            '𐡩' => Ok(Palmyrene::LetterYodh),
            '𐡪' => Ok(Palmyrene::LetterKaph),
            '𐡫' => Ok(Palmyrene::LetterLamedh),
            '𐡬' => Ok(Palmyrene::LetterMem),
            '𐡭' => Ok(Palmyrene::LetterFinalNun),
            '𐡮' => Ok(Palmyrene::LetterNun),
            '𐡯' => Ok(Palmyrene::LetterSamekh),
            '𐡰' => Ok(Palmyrene::LetterAyin),
            '𐡱' => Ok(Palmyrene::LetterPe),
            '𐡲' => Ok(Palmyrene::LetterSadhe),
            '𐡳' => Ok(Palmyrene::LetterQoph),
            '𐡴' => Ok(Palmyrene::LetterResh),
            '𐡵' => Ok(Palmyrene::LetterShin),
            '𐡶' => Ok(Palmyrene::LetterTaw),
            '𐡷' => Ok(Palmyrene::LeftDashPointingFleuron),
            '𐡸' => Ok(Palmyrene::RightDashPointingFleuron),
            '𐡹' => Ok(Palmyrene::NumberOne),
            '𐡺' => Ok(Palmyrene::NumberTwo),
            '𐡻' => Ok(Palmyrene::NumberThree),
            '𐡼' => Ok(Palmyrene::NumberFour),
            '𐡽' => Ok(Palmyrene::NumberFive),
            '𐡾' => Ok(Palmyrene::NumberTen),
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
