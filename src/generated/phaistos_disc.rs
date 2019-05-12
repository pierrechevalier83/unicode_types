
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

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            PhaistosDisc::SignPedestrian => "phaistos disc sign pedestrian",
            PhaistosDisc::SignPlumedHead => "phaistos disc sign plumed head",
            PhaistosDisc::SignTattooedHead => "phaistos disc sign tattooed head",
            PhaistosDisc::SignCaptive => "phaistos disc sign captive",
            PhaistosDisc::SignChild => "phaistos disc sign child",
            PhaistosDisc::SignWoman => "phaistos disc sign woman",
            PhaistosDisc::SignHelmet => "phaistos disc sign helmet",
            PhaistosDisc::SignGauntlet => "phaistos disc sign gauntlet",
            PhaistosDisc::SignTiara => "phaistos disc sign tiara",
            PhaistosDisc::SignArrow => "phaistos disc sign arrow",
            PhaistosDisc::SignBow => "phaistos disc sign bow",
            PhaistosDisc::SignShield => "phaistos disc sign shield",
            PhaistosDisc::SignClub => "phaistos disc sign club",
            PhaistosDisc::SignManacles => "phaistos disc sign manacles",
            PhaistosDisc::SignMattock => "phaistos disc sign mattock",
            PhaistosDisc::SignSaw => "phaistos disc sign saw",
            PhaistosDisc::SignLid => "phaistos disc sign lid",
            PhaistosDisc::SignBoomerang => "phaistos disc sign boomerang",
            PhaistosDisc::SignCarpentryPlane => "phaistos disc sign carpentry plane",
            PhaistosDisc::SignDolium => "phaistos disc sign dolium",
            PhaistosDisc::SignComb => "phaistos disc sign comb",
            PhaistosDisc::SignSling => "phaistos disc sign sling",
            PhaistosDisc::SignColumn => "phaistos disc sign column",
            PhaistosDisc::SignBeehive => "phaistos disc sign beehive",
            PhaistosDisc::SignShip => "phaistos disc sign ship",
            PhaistosDisc::SignHorn => "phaistos disc sign horn",
            PhaistosDisc::SignHide => "phaistos disc sign hide",
            PhaistosDisc::SignBullsLeg => "phaistos disc sign bulls leg",
            PhaistosDisc::SignCat => "phaistos disc sign cat",
            PhaistosDisc::SignRam => "phaistos disc sign ram",
            PhaistosDisc::SignEagle => "phaistos disc sign eagle",
            PhaistosDisc::SignDove => "phaistos disc sign dove",
            PhaistosDisc::SignTunny => "phaistos disc sign tunny",
            PhaistosDisc::SignBee => "phaistos disc sign bee",
            PhaistosDisc::SignPlaneTree => "phaistos disc sign plane tree",
            PhaistosDisc::SignVine => "phaistos disc sign vine",
            PhaistosDisc::SignPapyrus => "phaistos disc sign papyrus",
            PhaistosDisc::SignRosette => "phaistos disc sign rosette",
            PhaistosDisc::SignLily => "phaistos disc sign lily",
            PhaistosDisc::SignOxBack => "phaistos disc sign ox back",
            PhaistosDisc::SignFlute => "phaistos disc sign flute",
            PhaistosDisc::SignGrater => "phaistos disc sign grater",
            PhaistosDisc::SignStrainer => "phaistos disc sign strainer",
            PhaistosDisc::SignSmallAxe => "phaistos disc sign small axe",
            PhaistosDisc::SignWavyBand => "phaistos disc sign wavy band",
            PhaistosDisc::SignCombiningObliqueStroke => "phaistos disc sign combining oblique stroke",
        }
    }
}
