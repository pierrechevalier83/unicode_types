
/// An enum to represent all characters in the PhaistosDisc block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum PhaistosDisc {
    /// \u{101d0}: '𐇐'
    SignPedestrian,
    /// \u{101d1}: '𐇑'
    SignPlumedHead,
    /// \u{101d2}: '𐇒'
    SignTattooedHead,
    /// \u{101d3}: '𐇓'
    SignCaptive,
    /// \u{101d4}: '𐇔'
    SignChild,
    /// \u{101d5}: '𐇕'
    SignWoman,
    /// \u{101d6}: '𐇖'
    SignHelmet,
    /// \u{101d7}: '𐇗'
    SignGauntlet,
    /// \u{101d8}: '𐇘'
    SignTiara,
    /// \u{101d9}: '𐇙'
    SignArrow,
    /// \u{101da}: '𐇚'
    SignBow,
    /// \u{101db}: '𐇛'
    SignShield,
    /// \u{101dc}: '𐇜'
    SignClub,
    /// \u{101dd}: '𐇝'
    SignManacles,
    /// \u{101de}: '𐇞'
    SignMattock,
    /// \u{101df}: '𐇟'
    SignSaw,
    /// \u{101e0}: '𐇠'
    SignLid,
    /// \u{101e1}: '𐇡'
    SignBoomerang,
    /// \u{101e2}: '𐇢'
    SignCarpentryPlane,
    /// \u{101e3}: '𐇣'
    SignDolium,
    /// \u{101e4}: '𐇤'
    SignComb,
    /// \u{101e5}: '𐇥'
    SignSling,
    /// \u{101e6}: '𐇦'
    SignColumn,
    /// \u{101e7}: '𐇧'
    SignBeehive,
    /// \u{101e8}: '𐇨'
    SignShip,
    /// \u{101e9}: '𐇩'
    SignHorn,
    /// \u{101ea}: '𐇪'
    SignHide,
    /// \u{101eb}: '𐇫'
    SignBullsLeg,
    /// \u{101ec}: '𐇬'
    SignCat,
    /// \u{101ed}: '𐇭'
    SignRam,
    /// \u{101ee}: '𐇮'
    SignEagle,
    /// \u{101ef}: '𐇯'
    SignDove,
    /// \u{101f0}: '𐇰'
    SignTunny,
    /// \u{101f1}: '𐇱'
    SignBee,
    /// \u{101f2}: '𐇲'
    SignPlaneTree,
    /// \u{101f3}: '𐇳'
    SignVine,
    /// \u{101f4}: '𐇴'
    SignPapyrus,
    /// \u{101f5}: '𐇵'
    SignRosette,
    /// \u{101f6}: '𐇶'
    SignLily,
    /// \u{101f7}: '𐇷'
    SignOxBack,
    /// \u{101f8}: '𐇸'
    SignFlute,
    /// \u{101f9}: '𐇹'
    SignGrater,
    /// \u{101fa}: '𐇺'
    SignStrainer,
    /// \u{101fb}: '𐇻'
    SignSmallAxe,
    /// \u{101fc}: '𐇼'
    SignWavyBand,
    /// \u{101fd}: '𐇽'
    SignCombiningObliqueStroke,
}

impl Into<char> for PhaistosDisc {
    fn into(self) -> char {
        match self {
            PhaistosDisc::SignPedestrian => '𐇐',
            PhaistosDisc::SignPlumedHead => '𐇑',
            PhaistosDisc::SignTattooedHead => '𐇒',
            PhaistosDisc::SignCaptive => '𐇓',
            PhaistosDisc::SignChild => '𐇔',
            PhaistosDisc::SignWoman => '𐇕',
            PhaistosDisc::SignHelmet => '𐇖',
            PhaistosDisc::SignGauntlet => '𐇗',
            PhaistosDisc::SignTiara => '𐇘',
            PhaistosDisc::SignArrow => '𐇙',
            PhaistosDisc::SignBow => '𐇚',
            PhaistosDisc::SignShield => '𐇛',
            PhaistosDisc::SignClub => '𐇜',
            PhaistosDisc::SignManacles => '𐇝',
            PhaistosDisc::SignMattock => '𐇞',
            PhaistosDisc::SignSaw => '𐇟',
            PhaistosDisc::SignLid => '𐇠',
            PhaistosDisc::SignBoomerang => '𐇡',
            PhaistosDisc::SignCarpentryPlane => '𐇢',
            PhaistosDisc::SignDolium => '𐇣',
            PhaistosDisc::SignComb => '𐇤',
            PhaistosDisc::SignSling => '𐇥',
            PhaistosDisc::SignColumn => '𐇦',
            PhaistosDisc::SignBeehive => '𐇧',
            PhaistosDisc::SignShip => '𐇨',
            PhaistosDisc::SignHorn => '𐇩',
            PhaistosDisc::SignHide => '𐇪',
            PhaistosDisc::SignBullsLeg => '𐇫',
            PhaistosDisc::SignCat => '𐇬',
            PhaistosDisc::SignRam => '𐇭',
            PhaistosDisc::SignEagle => '𐇮',
            PhaistosDisc::SignDove => '𐇯',
            PhaistosDisc::SignTunny => '𐇰',
            PhaistosDisc::SignBee => '𐇱',
            PhaistosDisc::SignPlaneTree => '𐇲',
            PhaistosDisc::SignVine => '𐇳',
            PhaistosDisc::SignPapyrus => '𐇴',
            PhaistosDisc::SignRosette => '𐇵',
            PhaistosDisc::SignLily => '𐇶',
            PhaistosDisc::SignOxBack => '𐇷',
            PhaistosDisc::SignFlute => '𐇸',
            PhaistosDisc::SignGrater => '𐇹',
            PhaistosDisc::SignStrainer => '𐇺',
            PhaistosDisc::SignSmallAxe => '𐇻',
            PhaistosDisc::SignWavyBand => '𐇼',
            PhaistosDisc::SignCombiningObliqueStroke => '𐇽',
        }
    }
}

impl std::convert::TryFrom<char> for PhaistosDisc {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𐇐' => Ok(PhaistosDisc::SignPedestrian),
            '𐇑' => Ok(PhaistosDisc::SignPlumedHead),
            '𐇒' => Ok(PhaistosDisc::SignTattooedHead),
            '𐇓' => Ok(PhaistosDisc::SignCaptive),
            '𐇔' => Ok(PhaistosDisc::SignChild),
            '𐇕' => Ok(PhaistosDisc::SignWoman),
            '𐇖' => Ok(PhaistosDisc::SignHelmet),
            '𐇗' => Ok(PhaistosDisc::SignGauntlet),
            '𐇘' => Ok(PhaistosDisc::SignTiara),
            '𐇙' => Ok(PhaistosDisc::SignArrow),
            '𐇚' => Ok(PhaistosDisc::SignBow),
            '𐇛' => Ok(PhaistosDisc::SignShield),
            '𐇜' => Ok(PhaistosDisc::SignClub),
            '𐇝' => Ok(PhaistosDisc::SignManacles),
            '𐇞' => Ok(PhaistosDisc::SignMattock),
            '𐇟' => Ok(PhaistosDisc::SignSaw),
            '𐇠' => Ok(PhaistosDisc::SignLid),
            '𐇡' => Ok(PhaistosDisc::SignBoomerang),
            '𐇢' => Ok(PhaistosDisc::SignCarpentryPlane),
            '𐇣' => Ok(PhaistosDisc::SignDolium),
            '𐇤' => Ok(PhaistosDisc::SignComb),
            '𐇥' => Ok(PhaistosDisc::SignSling),
            '𐇦' => Ok(PhaistosDisc::SignColumn),
            '𐇧' => Ok(PhaistosDisc::SignBeehive),
            '𐇨' => Ok(PhaistosDisc::SignShip),
            '𐇩' => Ok(PhaistosDisc::SignHorn),
            '𐇪' => Ok(PhaistosDisc::SignHide),
            '𐇫' => Ok(PhaistosDisc::SignBullsLeg),
            '𐇬' => Ok(PhaistosDisc::SignCat),
            '𐇭' => Ok(PhaistosDisc::SignRam),
            '𐇮' => Ok(PhaistosDisc::SignEagle),
            '𐇯' => Ok(PhaistosDisc::SignDove),
            '𐇰' => Ok(PhaistosDisc::SignTunny),
            '𐇱' => Ok(PhaistosDisc::SignBee),
            '𐇲' => Ok(PhaistosDisc::SignPlaneTree),
            '𐇳' => Ok(PhaistosDisc::SignVine),
            '𐇴' => Ok(PhaistosDisc::SignPapyrus),
            '𐇵' => Ok(PhaistosDisc::SignRosette),
            '𐇶' => Ok(PhaistosDisc::SignLily),
            '𐇷' => Ok(PhaistosDisc::SignOxBack),
            '𐇸' => Ok(PhaistosDisc::SignFlute),
            '𐇹' => Ok(PhaistosDisc::SignGrater),
            '𐇺' => Ok(PhaistosDisc::SignStrainer),
            '𐇻' => Ok(PhaistosDisc::SignSmallAxe),
            '𐇼' => Ok(PhaistosDisc::SignWavyBand),
            '𐇽' => Ok(PhaistosDisc::SignCombiningObliqueStroke),
            _ => Err(()),
        }
    }
}

impl Into<u32> for PhaistosDisc {
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

impl std::convert::TryFrom<u32> for PhaistosDisc {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for PhaistosDisc {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl PhaistosDisc {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        PhaistosDisc::SignPedestrian
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("PhaistosDisc{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
