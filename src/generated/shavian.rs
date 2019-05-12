
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
        match self {
            Shavian::LetterPeep => '𐑐',
            Shavian::LetterTot => '𐑑',
            Shavian::LetterKick => '𐑒',
            Shavian::LetterFee => '𐑓',
            Shavian::LetterThigh => '𐑔',
            Shavian::LetterSo => '𐑕',
            Shavian::LetterSure => '𐑖',
            Shavian::LetterChurch => '𐑗',
            Shavian::LetterYea => '𐑘',
            Shavian::LetterHung => '𐑙',
            Shavian::LetterBib => '𐑚',
            Shavian::LetterDead => '𐑛',
            Shavian::LetterGag => '𐑜',
            Shavian::LetterVow => '𐑝',
            Shavian::LetterThey => '𐑞',
            Shavian::LetterZoo => '𐑟',
            Shavian::LetterMeasure => '𐑠',
            Shavian::LetterJudge => '𐑡',
            Shavian::LetterWoe => '𐑢',
            Shavian::LetterHaDashHa => '𐑣',
            Shavian::LetterLoll => '𐑤',
            Shavian::LetterMime => '𐑥',
            Shavian::LetterIf => '𐑦',
            Shavian::LetterEgg => '𐑧',
            Shavian::LetterAsh => '𐑨',
            Shavian::LetterAdo => '𐑩',
            Shavian::LetterOn => '𐑪',
            Shavian::LetterWool => '𐑫',
            Shavian::LetterOut => '𐑬',
            Shavian::LetterAh => '𐑭',
            Shavian::LetterRoar => '𐑮',
            Shavian::LetterNun => '𐑯',
            Shavian::LetterEat => '𐑰',
            Shavian::LetterAge => '𐑱',
            Shavian::LetterIce => '𐑲',
            Shavian::LetterUp => '𐑳',
            Shavian::LetterOak => '𐑴',
            Shavian::LetterOoze => '𐑵',
            Shavian::LetterOil => '𐑶',
            Shavian::LetterAwe => '𐑷',
            Shavian::LetterAre => '𐑸',
            Shavian::LetterOr => '𐑹',
            Shavian::LetterAir => '𐑺',
            Shavian::LetterErr => '𐑻',
            Shavian::LetterArray => '𐑼',
            Shavian::LetterEar => '𐑽',
            Shavian::LetterIan => '𐑾',
        }
    }
}

impl std::convert::TryFrom<char> for Shavian {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𐑐' => Ok(Shavian::LetterPeep),
            '𐑑' => Ok(Shavian::LetterTot),
            '𐑒' => Ok(Shavian::LetterKick),
            '𐑓' => Ok(Shavian::LetterFee),
            '𐑔' => Ok(Shavian::LetterThigh),
            '𐑕' => Ok(Shavian::LetterSo),
            '𐑖' => Ok(Shavian::LetterSure),
            '𐑗' => Ok(Shavian::LetterChurch),
            '𐑘' => Ok(Shavian::LetterYea),
            '𐑙' => Ok(Shavian::LetterHung),
            '𐑚' => Ok(Shavian::LetterBib),
            '𐑛' => Ok(Shavian::LetterDead),
            '𐑜' => Ok(Shavian::LetterGag),
            '𐑝' => Ok(Shavian::LetterVow),
            '𐑞' => Ok(Shavian::LetterThey),
            '𐑟' => Ok(Shavian::LetterZoo),
            '𐑠' => Ok(Shavian::LetterMeasure),
            '𐑡' => Ok(Shavian::LetterJudge),
            '𐑢' => Ok(Shavian::LetterWoe),
            '𐑣' => Ok(Shavian::LetterHaDashHa),
            '𐑤' => Ok(Shavian::LetterLoll),
            '𐑥' => Ok(Shavian::LetterMime),
            '𐑦' => Ok(Shavian::LetterIf),
            '𐑧' => Ok(Shavian::LetterEgg),
            '𐑨' => Ok(Shavian::LetterAsh),
            '𐑩' => Ok(Shavian::LetterAdo),
            '𐑪' => Ok(Shavian::LetterOn),
            '𐑫' => Ok(Shavian::LetterWool),
            '𐑬' => Ok(Shavian::LetterOut),
            '𐑭' => Ok(Shavian::LetterAh),
            '𐑮' => Ok(Shavian::LetterRoar),
            '𐑯' => Ok(Shavian::LetterNun),
            '𐑰' => Ok(Shavian::LetterEat),
            '𐑱' => Ok(Shavian::LetterAge),
            '𐑲' => Ok(Shavian::LetterIce),
            '𐑳' => Ok(Shavian::LetterUp),
            '𐑴' => Ok(Shavian::LetterOak),
            '𐑵' => Ok(Shavian::LetterOoze),
            '𐑶' => Ok(Shavian::LetterOil),
            '𐑷' => Ok(Shavian::LetterAwe),
            '𐑸' => Ok(Shavian::LetterAre),
            '𐑹' => Ok(Shavian::LetterOr),
            '𐑺' => Ok(Shavian::LetterAir),
            '𐑻' => Ok(Shavian::LetterErr),
            '𐑼' => Ok(Shavian::LetterArray),
            '𐑽' => Ok(Shavian::LetterEar),
            '𐑾' => Ok(Shavian::LetterIan),
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
