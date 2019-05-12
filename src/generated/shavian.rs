/// A number of constants to give a name to all characters in this block.
mod constants {
    /// \u{10450}: '𐑐'
    pub const LETTER_PEEP: char = '𐑐';
    /// \u{10451}: '𐑑'
    pub const LETTER_TOT: char = '𐑑';
    /// \u{10452}: '𐑒'
    pub const LETTER_KICK: char = '𐑒';
    /// \u{10453}: '𐑓'
    pub const LETTER_FEE: char = '𐑓';
    /// \u{10454}: '𐑔'
    pub const LETTER_THIGH: char = '𐑔';
    /// \u{10455}: '𐑕'
    pub const LETTER_SO: char = '𐑕';
    /// \u{10456}: '𐑖'
    pub const LETTER_SURE: char = '𐑖';
    /// \u{10457}: '𐑗'
    pub const LETTER_CHURCH: char = '𐑗';
    /// \u{10458}: '𐑘'
    pub const LETTER_YEA: char = '𐑘';
    /// \u{10459}: '𐑙'
    pub const LETTER_HUNG: char = '𐑙';
    /// \u{1045a}: '𐑚'
    pub const LETTER_BIB: char = '𐑚';
    /// \u{1045b}: '𐑛'
    pub const LETTER_DEAD: char = '𐑛';
    /// \u{1045c}: '𐑜'
    pub const LETTER_GAG: char = '𐑜';
    /// \u{1045d}: '𐑝'
    pub const LETTER_VOW: char = '𐑝';
    /// \u{1045e}: '𐑞'
    pub const LETTER_THEY: char = '𐑞';
    /// \u{1045f}: '𐑟'
    pub const LETTER_ZOO: char = '𐑟';
    /// \u{10460}: '𐑠'
    pub const LETTER_MEASURE: char = '𐑠';
    /// \u{10461}: '𐑡'
    pub const LETTER_JUDGE: char = '𐑡';
    /// \u{10462}: '𐑢'
    pub const LETTER_WOE: char = '𐑢';
    /// \u{10463}: '𐑣'
    pub const LETTER_HA_DASH_HA: char = '𐑣';
    /// \u{10464}: '𐑤'
    pub const LETTER_LOLL: char = '𐑤';
    /// \u{10465}: '𐑥'
    pub const LETTER_MIME: char = '𐑥';
    /// \u{10466}: '𐑦'
    pub const LETTER_IF: char = '𐑦';
    /// \u{10467}: '𐑧'
    pub const LETTER_EGG: char = '𐑧';
    /// \u{10468}: '𐑨'
    pub const LETTER_ASH: char = '𐑨';
    /// \u{10469}: '𐑩'
    pub const LETTER_ADO: char = '𐑩';
    /// \u{1046a}: '𐑪'
    pub const LETTER_ON: char = '𐑪';
    /// \u{1046b}: '𐑫'
    pub const LETTER_WOOL: char = '𐑫';
    /// \u{1046c}: '𐑬'
    pub const LETTER_OUT: char = '𐑬';
    /// \u{1046d}: '𐑭'
    pub const LETTER_AH: char = '𐑭';
    /// \u{1046e}: '𐑮'
    pub const LETTER_ROAR: char = '𐑮';
    /// \u{1046f}: '𐑯'
    pub const LETTER_NUN: char = '𐑯';
    /// \u{10470}: '𐑰'
    pub const LETTER_EAT: char = '𐑰';
    /// \u{10471}: '𐑱'
    pub const LETTER_AGE: char = '𐑱';
    /// \u{10472}: '𐑲'
    pub const LETTER_ICE: char = '𐑲';
    /// \u{10473}: '𐑳'
    pub const LETTER_UP: char = '𐑳';
    /// \u{10474}: '𐑴'
    pub const LETTER_OAK: char = '𐑴';
    /// \u{10475}: '𐑵'
    pub const LETTER_OOZE: char = '𐑵';
    /// \u{10476}: '𐑶'
    pub const LETTER_OIL: char = '𐑶';
    /// \u{10477}: '𐑷'
    pub const LETTER_AWE: char = '𐑷';
    /// \u{10478}: '𐑸'
    pub const LETTER_ARE: char = '𐑸';
    /// \u{10479}: '𐑹'
    pub const LETTER_OR: char = '𐑹';
    /// \u{1047a}: '𐑺'
    pub const LETTER_AIR: char = '𐑺';
    /// \u{1047b}: '𐑻'
    pub const LETTER_ERR: char = '𐑻';
    /// \u{1047c}: '𐑼'
    pub const LETTER_ARRAY: char = '𐑼';
    /// \u{1047d}: '𐑽'
    pub const LETTER_EAR: char = '𐑽';
    /// \u{1047e}: '𐑾'
    pub const LETTER_IAN: char = '𐑾';
}

/// An enum to represent all characters in the Shavian block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Shavian {
    /// \u{10450}: '𐑐'
    LetterPeep,
    /// \u{10451}: '𐑑'
    LetterTot,
    /// \u{10452}: '𐑒'
    LetterKick,
    /// \u{10453}: '𐑓'
    LetterFee,
    /// \u{10454}: '𐑔'
    LetterThigh,
    /// \u{10455}: '𐑕'
    LetterSo,
    /// \u{10456}: '𐑖'
    LetterSure,
    /// \u{10457}: '𐑗'
    LetterChurch,
    /// \u{10458}: '𐑘'
    LetterYea,
    /// \u{10459}: '𐑙'
    LetterHung,
    /// \u{1045a}: '𐑚'
    LetterBib,
    /// \u{1045b}: '𐑛'
    LetterDead,
    /// \u{1045c}: '𐑜'
    LetterGag,
    /// \u{1045d}: '𐑝'
    LetterVow,
    /// \u{1045e}: '𐑞'
    LetterThey,
    /// \u{1045f}: '𐑟'
    LetterZoo,
    /// \u{10460}: '𐑠'
    LetterMeasure,
    /// \u{10461}: '𐑡'
    LetterJudge,
    /// \u{10462}: '𐑢'
    LetterWoe,
    /// \u{10463}: '𐑣'
    LetterHaDashHa,
    /// \u{10464}: '𐑤'
    LetterLoll,
    /// \u{10465}: '𐑥'
    LetterMime,
    /// \u{10466}: '𐑦'
    LetterIf,
    /// \u{10467}: '𐑧'
    LetterEgg,
    /// \u{10468}: '𐑨'
    LetterAsh,
    /// \u{10469}: '𐑩'
    LetterAdo,
    /// \u{1046a}: '𐑪'
    LetterOn,
    /// \u{1046b}: '𐑫'
    LetterWool,
    /// \u{1046c}: '𐑬'
    LetterOut,
    /// \u{1046d}: '𐑭'
    LetterAh,
    /// \u{1046e}: '𐑮'
    LetterRoar,
    /// \u{1046f}: '𐑯'
    LetterNun,
    /// \u{10470}: '𐑰'
    LetterEat,
    /// \u{10471}: '𐑱'
    LetterAge,
    /// \u{10472}: '𐑲'
    LetterIce,
    /// \u{10473}: '𐑳'
    LetterUp,
    /// \u{10474}: '𐑴'
    LetterOak,
    /// \u{10475}: '𐑵'
    LetterOoze,
    /// \u{10476}: '𐑶'
    LetterOil,
    /// \u{10477}: '𐑷'
    LetterAwe,
    /// \u{10478}: '𐑸'
    LetterAre,
    /// \u{10479}: '𐑹'
    LetterOr,
    /// \u{1047a}: '𐑺'
    LetterAir,
    /// \u{1047b}: '𐑻'
    LetterErr,
    /// \u{1047c}: '𐑼'
    LetterArray,
    /// \u{1047d}: '𐑽'
    LetterEar,
    /// \u{1047e}: '𐑾'
    LetterIan,
}

impl Into<char> for Shavian {
    fn into(self) -> char {
        use constants::*;
        match self {
            Shavian::LetterPeep => LETTER_PEEP,
            Shavian::LetterTot => LETTER_TOT,
            Shavian::LetterKick => LETTER_KICK,
            Shavian::LetterFee => LETTER_FEE,
            Shavian::LetterThigh => LETTER_THIGH,
            Shavian::LetterSo => LETTER_SO,
            Shavian::LetterSure => LETTER_SURE,
            Shavian::LetterChurch => LETTER_CHURCH,
            Shavian::LetterYea => LETTER_YEA,
            Shavian::LetterHung => LETTER_HUNG,
            Shavian::LetterBib => LETTER_BIB,
            Shavian::LetterDead => LETTER_DEAD,
            Shavian::LetterGag => LETTER_GAG,
            Shavian::LetterVow => LETTER_VOW,
            Shavian::LetterThey => LETTER_THEY,
            Shavian::LetterZoo => LETTER_ZOO,
            Shavian::LetterMeasure => LETTER_MEASURE,
            Shavian::LetterJudge => LETTER_JUDGE,
            Shavian::LetterWoe => LETTER_WOE,
            Shavian::LetterHaDashHa => LETTER_HA_DASH_HA,
            Shavian::LetterLoll => LETTER_LOLL,
            Shavian::LetterMime => LETTER_MIME,
            Shavian::LetterIf => LETTER_IF,
            Shavian::LetterEgg => LETTER_EGG,
            Shavian::LetterAsh => LETTER_ASH,
            Shavian::LetterAdo => LETTER_ADO,
            Shavian::LetterOn => LETTER_ON,
            Shavian::LetterWool => LETTER_WOOL,
            Shavian::LetterOut => LETTER_OUT,
            Shavian::LetterAh => LETTER_AH,
            Shavian::LetterRoar => LETTER_ROAR,
            Shavian::LetterNun => LETTER_NUN,
            Shavian::LetterEat => LETTER_EAT,
            Shavian::LetterAge => LETTER_AGE,
            Shavian::LetterIce => LETTER_ICE,
            Shavian::LetterUp => LETTER_UP,
            Shavian::LetterOak => LETTER_OAK,
            Shavian::LetterOoze => LETTER_OOZE,
            Shavian::LetterOil => LETTER_OIL,
            Shavian::LetterAwe => LETTER_AWE,
            Shavian::LetterAre => LETTER_ARE,
            Shavian::LetterOr => LETTER_OR,
            Shavian::LetterAir => LETTER_AIR,
            Shavian::LetterErr => LETTER_ERR,
            Shavian::LetterArray => LETTER_ARRAY,
            Shavian::LetterEar => LETTER_EAR,
            Shavian::LetterIan => LETTER_IAN,
        }
    }
}

impl std::convert::TryFrom<char> for Shavian {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_PEEP => Ok(Shavian::LetterPeep),
            LETTER_TOT => Ok(Shavian::LetterTot),
            LETTER_KICK => Ok(Shavian::LetterKick),
            LETTER_FEE => Ok(Shavian::LetterFee),
            LETTER_THIGH => Ok(Shavian::LetterThigh),
            LETTER_SO => Ok(Shavian::LetterSo),
            LETTER_SURE => Ok(Shavian::LetterSure),
            LETTER_CHURCH => Ok(Shavian::LetterChurch),
            LETTER_YEA => Ok(Shavian::LetterYea),
            LETTER_HUNG => Ok(Shavian::LetterHung),
            LETTER_BIB => Ok(Shavian::LetterBib),
            LETTER_DEAD => Ok(Shavian::LetterDead),
            LETTER_GAG => Ok(Shavian::LetterGag),
            LETTER_VOW => Ok(Shavian::LetterVow),
            LETTER_THEY => Ok(Shavian::LetterThey),
            LETTER_ZOO => Ok(Shavian::LetterZoo),
            LETTER_MEASURE => Ok(Shavian::LetterMeasure),
            LETTER_JUDGE => Ok(Shavian::LetterJudge),
            LETTER_WOE => Ok(Shavian::LetterWoe),
            LETTER_HA_DASH_HA => Ok(Shavian::LetterHaDashHa),
            LETTER_LOLL => Ok(Shavian::LetterLoll),
            LETTER_MIME => Ok(Shavian::LetterMime),
            LETTER_IF => Ok(Shavian::LetterIf),
            LETTER_EGG => Ok(Shavian::LetterEgg),
            LETTER_ASH => Ok(Shavian::LetterAsh),
            LETTER_ADO => Ok(Shavian::LetterAdo),
            LETTER_ON => Ok(Shavian::LetterOn),
            LETTER_WOOL => Ok(Shavian::LetterWool),
            LETTER_OUT => Ok(Shavian::LetterOut),
            LETTER_AH => Ok(Shavian::LetterAh),
            LETTER_ROAR => Ok(Shavian::LetterRoar),
            LETTER_NUN => Ok(Shavian::LetterNun),
            LETTER_EAT => Ok(Shavian::LetterEat),
            LETTER_AGE => Ok(Shavian::LetterAge),
            LETTER_ICE => Ok(Shavian::LetterIce),
            LETTER_UP => Ok(Shavian::LetterUp),
            LETTER_OAK => Ok(Shavian::LetterOak),
            LETTER_OOZE => Ok(Shavian::LetterOoze),
            LETTER_OIL => Ok(Shavian::LetterOil),
            LETTER_AWE => Ok(Shavian::LetterAwe),
            LETTER_ARE => Ok(Shavian::LetterAre),
            LETTER_OR => Ok(Shavian::LetterOr),
            LETTER_AIR => Ok(Shavian::LetterAir),
            LETTER_ERR => Ok(Shavian::LetterErr),
            LETTER_ARRAY => Ok(Shavian::LetterArray),
            LETTER_EAR => Ok(Shavian::LetterEar),
            LETTER_IAN => Ok(Shavian::LetterIan),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Shavian {
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

impl std::convert::TryFrom<u32> for Shavian {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Shavian {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Shavian {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Shavian::LetterPeep
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Shavian::LetterPeep => "shavian letter peep",
            Shavian::LetterTot => "shavian letter tot",
            Shavian::LetterKick => "shavian letter kick",
            Shavian::LetterFee => "shavian letter fee",
            Shavian::LetterThigh => "shavian letter thigh",
            Shavian::LetterSo => "shavian letter so",
            Shavian::LetterSure => "shavian letter sure",
            Shavian::LetterChurch => "shavian letter church",
            Shavian::LetterYea => "shavian letter yea",
            Shavian::LetterHung => "shavian letter hung",
            Shavian::LetterBib => "shavian letter bib",
            Shavian::LetterDead => "shavian letter dead",
            Shavian::LetterGag => "shavian letter gag",
            Shavian::LetterVow => "shavian letter vow",
            Shavian::LetterThey => "shavian letter they",
            Shavian::LetterZoo => "shavian letter zoo",
            Shavian::LetterMeasure => "shavian letter measure",
            Shavian::LetterJudge => "shavian letter judge",
            Shavian::LetterWoe => "shavian letter woe",
            Shavian::LetterHaDashHa => "shavian letter ha-ha",
            Shavian::LetterLoll => "shavian letter loll",
            Shavian::LetterMime => "shavian letter mime",
            Shavian::LetterIf => "shavian letter if",
            Shavian::LetterEgg => "shavian letter egg",
            Shavian::LetterAsh => "shavian letter ash",
            Shavian::LetterAdo => "shavian letter ado",
            Shavian::LetterOn => "shavian letter on",
            Shavian::LetterWool => "shavian letter wool",
            Shavian::LetterOut => "shavian letter out",
            Shavian::LetterAh => "shavian letter ah",
            Shavian::LetterRoar => "shavian letter roar",
            Shavian::LetterNun => "shavian letter nun",
            Shavian::LetterEat => "shavian letter eat",
            Shavian::LetterAge => "shavian letter age",
            Shavian::LetterIce => "shavian letter ice",
            Shavian::LetterUp => "shavian letter up",
            Shavian::LetterOak => "shavian letter oak",
            Shavian::LetterOoze => "shavian letter ooze",
            Shavian::LetterOil => "shavian letter oil",
            Shavian::LetterAwe => "shavian letter awe",
            Shavian::LetterAre => "shavian letter are",
            Shavian::LetterOr => "shavian letter or",
            Shavian::LetterAir => "shavian letter air",
            Shavian::LetterErr => "shavian letter err",
            Shavian::LetterArray => "shavian letter array",
            Shavian::LetterEar => "shavian letter ear",
            Shavian::LetterIan => "shavian letter ian",
        }
    }
}
