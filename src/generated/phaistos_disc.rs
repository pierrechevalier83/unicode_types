/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{101d0}: '𐇐'
    pub const SIGN_PEDESTRIAN: char = '𐇐';
    /// \u{101d1}: '𐇑'
    pub const SIGN_PLUMED_HEAD: char = '𐇑';
    /// \u{101d2}: '𐇒'
    pub const SIGN_TATTOOED_HEAD: char = '𐇒';
    /// \u{101d3}: '𐇓'
    pub const SIGN_CAPTIVE: char = '𐇓';
    /// \u{101d4}: '𐇔'
    pub const SIGN_CHILD: char = '𐇔';
    /// \u{101d5}: '𐇕'
    pub const SIGN_WOMAN: char = '𐇕';
    /// \u{101d6}: '𐇖'
    pub const SIGN_HELMET: char = '𐇖';
    /// \u{101d7}: '𐇗'
    pub const SIGN_GAUNTLET: char = '𐇗';
    /// \u{101d8}: '𐇘'
    pub const SIGN_TIARA: char = '𐇘';
    /// \u{101d9}: '𐇙'
    pub const SIGN_ARROW: char = '𐇙';
    /// \u{101da}: '𐇚'
    pub const SIGN_BOW: char = '𐇚';
    /// \u{101db}: '𐇛'
    pub const SIGN_SHIELD: char = '𐇛';
    /// \u{101dc}: '𐇜'
    pub const SIGN_CLUB: char = '𐇜';
    /// \u{101dd}: '𐇝'
    pub const SIGN_MANACLES: char = '𐇝';
    /// \u{101de}: '𐇞'
    pub const SIGN_MATTOCK: char = '𐇞';
    /// \u{101df}: '𐇟'
    pub const SIGN_SAW: char = '𐇟';
    /// \u{101e0}: '𐇠'
    pub const SIGN_LID: char = '𐇠';
    /// \u{101e1}: '𐇡'
    pub const SIGN_BOOMERANG: char = '𐇡';
    /// \u{101e2}: '𐇢'
    pub const SIGN_CARPENTRY_PLANE: char = '𐇢';
    /// \u{101e3}: '𐇣'
    pub const SIGN_DOLIUM: char = '𐇣';
    /// \u{101e4}: '𐇤'
    pub const SIGN_COMB: char = '𐇤';
    /// \u{101e5}: '𐇥'
    pub const SIGN_SLING: char = '𐇥';
    /// \u{101e6}: '𐇦'
    pub const SIGN_COLUMN: char = '𐇦';
    /// \u{101e7}: '𐇧'
    pub const SIGN_BEEHIVE: char = '𐇧';
    /// \u{101e8}: '𐇨'
    pub const SIGN_SHIP: char = '𐇨';
    /// \u{101e9}: '𐇩'
    pub const SIGN_HORN: char = '𐇩';
    /// \u{101ea}: '𐇪'
    pub const SIGN_HIDE: char = '𐇪';
    /// \u{101eb}: '𐇫'
    pub const SIGN_BULLS_LEG: char = '𐇫';
    /// \u{101ec}: '𐇬'
    pub const SIGN_CAT: char = '𐇬';
    /// \u{101ed}: '𐇭'
    pub const SIGN_RAM: char = '𐇭';
    /// \u{101ee}: '𐇮'
    pub const SIGN_EAGLE: char = '𐇮';
    /// \u{101ef}: '𐇯'
    pub const SIGN_DOVE: char = '𐇯';
    /// \u{101f0}: '𐇰'
    pub const SIGN_TUNNY: char = '𐇰';
    /// \u{101f1}: '𐇱'
    pub const SIGN_BEE: char = '𐇱';
    /// \u{101f2}: '𐇲'
    pub const SIGN_PLANE_TREE: char = '𐇲';
    /// \u{101f3}: '𐇳'
    pub const SIGN_VINE: char = '𐇳';
    /// \u{101f4}: '𐇴'
    pub const SIGN_PAPYRUS: char = '𐇴';
    /// \u{101f5}: '𐇵'
    pub const SIGN_ROSETTE: char = '𐇵';
    /// \u{101f6}: '𐇶'
    pub const SIGN_LILY: char = '𐇶';
    /// \u{101f7}: '𐇷'
    pub const SIGN_OX_BACK: char = '𐇷';
    /// \u{101f8}: '𐇸'
    pub const SIGN_FLUTE: char = '𐇸';
    /// \u{101f9}: '𐇹'
    pub const SIGN_GRATER: char = '𐇹';
    /// \u{101fa}: '𐇺'
    pub const SIGN_STRAINER: char = '𐇺';
    /// \u{101fb}: '𐇻'
    pub const SIGN_SMALL_AXE: char = '𐇻';
    /// \u{101fc}: '𐇼'
    pub const SIGN_WAVY_BAND: char = '𐇼';
    /// \u{101fd}: '𐇽'
    pub const SIGN_COMBINING_OBLIQUE_STROKE: char = '𐇽';
}

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
        use constants::*;
        match self {
            PhaistosDisc::SignPedestrian => SIGN_PEDESTRIAN,
            PhaistosDisc::SignPlumedHead => SIGN_PLUMED_HEAD,
            PhaistosDisc::SignTattooedHead => SIGN_TATTOOED_HEAD,
            PhaistosDisc::SignCaptive => SIGN_CAPTIVE,
            PhaistosDisc::SignChild => SIGN_CHILD,
            PhaistosDisc::SignWoman => SIGN_WOMAN,
            PhaistosDisc::SignHelmet => SIGN_HELMET,
            PhaistosDisc::SignGauntlet => SIGN_GAUNTLET,
            PhaistosDisc::SignTiara => SIGN_TIARA,
            PhaistosDisc::SignArrow => SIGN_ARROW,
            PhaistosDisc::SignBow => SIGN_BOW,
            PhaistosDisc::SignShield => SIGN_SHIELD,
            PhaistosDisc::SignClub => SIGN_CLUB,
            PhaistosDisc::SignManacles => SIGN_MANACLES,
            PhaistosDisc::SignMattock => SIGN_MATTOCK,
            PhaistosDisc::SignSaw => SIGN_SAW,
            PhaistosDisc::SignLid => SIGN_LID,
            PhaistosDisc::SignBoomerang => SIGN_BOOMERANG,
            PhaistosDisc::SignCarpentryPlane => SIGN_CARPENTRY_PLANE,
            PhaistosDisc::SignDolium => SIGN_DOLIUM,
            PhaistosDisc::SignComb => SIGN_COMB,
            PhaistosDisc::SignSling => SIGN_SLING,
            PhaistosDisc::SignColumn => SIGN_COLUMN,
            PhaistosDisc::SignBeehive => SIGN_BEEHIVE,
            PhaistosDisc::SignShip => SIGN_SHIP,
            PhaistosDisc::SignHorn => SIGN_HORN,
            PhaistosDisc::SignHide => SIGN_HIDE,
            PhaistosDisc::SignBullsLeg => SIGN_BULLS_LEG,
            PhaistosDisc::SignCat => SIGN_CAT,
            PhaistosDisc::SignRam => SIGN_RAM,
            PhaistosDisc::SignEagle => SIGN_EAGLE,
            PhaistosDisc::SignDove => SIGN_DOVE,
            PhaistosDisc::SignTunny => SIGN_TUNNY,
            PhaistosDisc::SignBee => SIGN_BEE,
            PhaistosDisc::SignPlaneTree => SIGN_PLANE_TREE,
            PhaistosDisc::SignVine => SIGN_VINE,
            PhaistosDisc::SignPapyrus => SIGN_PAPYRUS,
            PhaistosDisc::SignRosette => SIGN_ROSETTE,
            PhaistosDisc::SignLily => SIGN_LILY,
            PhaistosDisc::SignOxBack => SIGN_OX_BACK,
            PhaistosDisc::SignFlute => SIGN_FLUTE,
            PhaistosDisc::SignGrater => SIGN_GRATER,
            PhaistosDisc::SignStrainer => SIGN_STRAINER,
            PhaistosDisc::SignSmallAxe => SIGN_SMALL_AXE,
            PhaistosDisc::SignWavyBand => SIGN_WAVY_BAND,
            PhaistosDisc::SignCombiningObliqueStroke => SIGN_COMBINING_OBLIQUE_STROKE,
        }
    }
}

impl std::convert::TryFrom<char> for PhaistosDisc {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            SIGN_PEDESTRIAN => Ok(PhaistosDisc::SignPedestrian),
            SIGN_PLUMED_HEAD => Ok(PhaistosDisc::SignPlumedHead),
            SIGN_TATTOOED_HEAD => Ok(PhaistosDisc::SignTattooedHead),
            SIGN_CAPTIVE => Ok(PhaistosDisc::SignCaptive),
            SIGN_CHILD => Ok(PhaistosDisc::SignChild),
            SIGN_WOMAN => Ok(PhaistosDisc::SignWoman),
            SIGN_HELMET => Ok(PhaistosDisc::SignHelmet),
            SIGN_GAUNTLET => Ok(PhaistosDisc::SignGauntlet),
            SIGN_TIARA => Ok(PhaistosDisc::SignTiara),
            SIGN_ARROW => Ok(PhaistosDisc::SignArrow),
            SIGN_BOW => Ok(PhaistosDisc::SignBow),
            SIGN_SHIELD => Ok(PhaistosDisc::SignShield),
            SIGN_CLUB => Ok(PhaistosDisc::SignClub),
            SIGN_MANACLES => Ok(PhaistosDisc::SignManacles),
            SIGN_MATTOCK => Ok(PhaistosDisc::SignMattock),
            SIGN_SAW => Ok(PhaistosDisc::SignSaw),
            SIGN_LID => Ok(PhaistosDisc::SignLid),
            SIGN_BOOMERANG => Ok(PhaistosDisc::SignBoomerang),
            SIGN_CARPENTRY_PLANE => Ok(PhaistosDisc::SignCarpentryPlane),
            SIGN_DOLIUM => Ok(PhaistosDisc::SignDolium),
            SIGN_COMB => Ok(PhaistosDisc::SignComb),
            SIGN_SLING => Ok(PhaistosDisc::SignSling),
            SIGN_COLUMN => Ok(PhaistosDisc::SignColumn),
            SIGN_BEEHIVE => Ok(PhaistosDisc::SignBeehive),
            SIGN_SHIP => Ok(PhaistosDisc::SignShip),
            SIGN_HORN => Ok(PhaistosDisc::SignHorn),
            SIGN_HIDE => Ok(PhaistosDisc::SignHide),
            SIGN_BULLS_LEG => Ok(PhaistosDisc::SignBullsLeg),
            SIGN_CAT => Ok(PhaistosDisc::SignCat),
            SIGN_RAM => Ok(PhaistosDisc::SignRam),
            SIGN_EAGLE => Ok(PhaistosDisc::SignEagle),
            SIGN_DOVE => Ok(PhaistosDisc::SignDove),
            SIGN_TUNNY => Ok(PhaistosDisc::SignTunny),
            SIGN_BEE => Ok(PhaistosDisc::SignBee),
            SIGN_PLANE_TREE => Ok(PhaistosDisc::SignPlaneTree),
            SIGN_VINE => Ok(PhaistosDisc::SignVine),
            SIGN_PAPYRUS => Ok(PhaistosDisc::SignPapyrus),
            SIGN_ROSETTE => Ok(PhaistosDisc::SignRosette),
            SIGN_LILY => Ok(PhaistosDisc::SignLily),
            SIGN_OX_BACK => Ok(PhaistosDisc::SignOxBack),
            SIGN_FLUTE => Ok(PhaistosDisc::SignFlute),
            SIGN_GRATER => Ok(PhaistosDisc::SignGrater),
            SIGN_STRAINER => Ok(PhaistosDisc::SignStrainer),
            SIGN_SMALL_AXE => Ok(PhaistosDisc::SignSmallAxe),
            SIGN_WAVY_BAND => Ok(PhaistosDisc::SignWavyBand),
            SIGN_COMBINING_OBLIQUE_STROKE => Ok(PhaistosDisc::SignCombiningObliqueStroke),
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
