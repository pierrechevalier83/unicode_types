/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{1f000}: '🀀'
    pub const MAHJONG_TILE_EAST_WIND: char = '🀀';
    /// \u{1f001}: '🀁'
    pub const MAHJONG_TILE_SOUTH_WIND: char = '🀁';
    /// \u{1f002}: '🀂'
    pub const MAHJONG_TILE_WEST_WIND: char = '🀂';
    /// \u{1f003}: '🀃'
    pub const MAHJONG_TILE_NORTH_WIND: char = '🀃';
    /// \u{1f004}: '🀄'
    pub const MAHJONG_TILE_RED_DRAGON: char = '🀄';
    /// \u{1f005}: '🀅'
    pub const MAHJONG_TILE_GREEN_DRAGON: char = '🀅';
    /// \u{1f006}: '🀆'
    pub const MAHJONG_TILE_WHITE_DRAGON: char = '🀆';
    /// \u{1f007}: '🀇'
    pub const MAHJONG_TILE_ONE_OF_CHARACTERS: char = '🀇';
    /// \u{1f008}: '🀈'
    pub const MAHJONG_TILE_TWO_OF_CHARACTERS: char = '🀈';
    /// \u{1f009}: '🀉'
    pub const MAHJONG_TILE_THREE_OF_CHARACTERS: char = '🀉';
    /// \u{1f00a}: '🀊'
    pub const MAHJONG_TILE_FOUR_OF_CHARACTERS: char = '🀊';
    /// \u{1f00b}: '🀋'
    pub const MAHJONG_TILE_FIVE_OF_CHARACTERS: char = '🀋';
    /// \u{1f00c}: '🀌'
    pub const MAHJONG_TILE_SIX_OF_CHARACTERS: char = '🀌';
    /// \u{1f00d}: '🀍'
    pub const MAHJONG_TILE_SEVEN_OF_CHARACTERS: char = '🀍';
    /// \u{1f00e}: '🀎'
    pub const MAHJONG_TILE_EIGHT_OF_CHARACTERS: char = '🀎';
    /// \u{1f00f}: '🀏'
    pub const MAHJONG_TILE_NINE_OF_CHARACTERS: char = '🀏';
    /// \u{1f010}: '🀐'
    pub const MAHJONG_TILE_ONE_OF_BAMBOOS: char = '🀐';
    /// \u{1f011}: '🀑'
    pub const MAHJONG_TILE_TWO_OF_BAMBOOS: char = '🀑';
    /// \u{1f012}: '🀒'
    pub const MAHJONG_TILE_THREE_OF_BAMBOOS: char = '🀒';
    /// \u{1f013}: '🀓'
    pub const MAHJONG_TILE_FOUR_OF_BAMBOOS: char = '🀓';
    /// \u{1f014}: '🀔'
    pub const MAHJONG_TILE_FIVE_OF_BAMBOOS: char = '🀔';
    /// \u{1f015}: '🀕'
    pub const MAHJONG_TILE_SIX_OF_BAMBOOS: char = '🀕';
    /// \u{1f016}: '🀖'
    pub const MAHJONG_TILE_SEVEN_OF_BAMBOOS: char = '🀖';
    /// \u{1f017}: '🀗'
    pub const MAHJONG_TILE_EIGHT_OF_BAMBOOS: char = '🀗';
    /// \u{1f018}: '🀘'
    pub const MAHJONG_TILE_NINE_OF_BAMBOOS: char = '🀘';
    /// \u{1f019}: '🀙'
    pub const MAHJONG_TILE_ONE_OF_CIRCLES: char = '🀙';
    /// \u{1f01a}: '🀚'
    pub const MAHJONG_TILE_TWO_OF_CIRCLES: char = '🀚';
    /// \u{1f01b}: '🀛'
    pub const MAHJONG_TILE_THREE_OF_CIRCLES: char = '🀛';
    /// \u{1f01c}: '🀜'
    pub const MAHJONG_TILE_FOUR_OF_CIRCLES: char = '🀜';
    /// \u{1f01d}: '🀝'
    pub const MAHJONG_TILE_FIVE_OF_CIRCLES: char = '🀝';
    /// \u{1f01e}: '🀞'
    pub const MAHJONG_TILE_SIX_OF_CIRCLES: char = '🀞';
    /// \u{1f01f}: '🀟'
    pub const MAHJONG_TILE_SEVEN_OF_CIRCLES: char = '🀟';
    /// \u{1f020}: '🀠'
    pub const MAHJONG_TILE_EIGHT_OF_CIRCLES: char = '🀠';
    /// \u{1f021}: '🀡'
    pub const MAHJONG_TILE_NINE_OF_CIRCLES: char = '🀡';
    /// \u{1f022}: '🀢'
    pub const MAHJONG_TILE_PLUM: char = '🀢';
    /// \u{1f023}: '🀣'
    pub const MAHJONG_TILE_ORCHID: char = '🀣';
    /// \u{1f024}: '🀤'
    pub const MAHJONG_TILE_BAMBOO: char = '🀤';
    /// \u{1f025}: '🀥'
    pub const MAHJONG_TILE_CHRYSANTHEMUM: char = '🀥';
    /// \u{1f026}: '🀦'
    pub const MAHJONG_TILE_SPRING: char = '🀦';
    /// \u{1f027}: '🀧'
    pub const MAHJONG_TILE_SUMMER: char = '🀧';
    /// \u{1f028}: '🀨'
    pub const MAHJONG_TILE_AUTUMN: char = '🀨';
    /// \u{1f029}: '🀩'
    pub const MAHJONG_TILE_WINTER: char = '🀩';
    /// \u{1f02a}: '🀪'
    pub const MAHJONG_TILE_JOKER: char = '🀪';
    /// \u{1f02b}: '🀫'
    pub const MAHJONG_TILE_BACK: char = '🀫';
}

/// An enum to represent all characters in the MahjongTiles block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum MahjongTiles {
    /// \u{1f000}: '🀀'
    MahjongTileEastWind,
    /// \u{1f001}: '🀁'
    MahjongTileSouthWind,
    /// \u{1f002}: '🀂'
    MahjongTileWestWind,
    /// \u{1f003}: '🀃'
    MahjongTileNorthWind,
    /// \u{1f004}: '🀄'
    MahjongTileRedDragon,
    /// \u{1f005}: '🀅'
    MahjongTileGreenDragon,
    /// \u{1f006}: '🀆'
    MahjongTileWhiteDragon,
    /// \u{1f007}: '🀇'
    MahjongTileOneOfCharacters,
    /// \u{1f008}: '🀈'
    MahjongTileTwoOfCharacters,
    /// \u{1f009}: '🀉'
    MahjongTileThreeOfCharacters,
    /// \u{1f00a}: '🀊'
    MahjongTileFourOfCharacters,
    /// \u{1f00b}: '🀋'
    MahjongTileFiveOfCharacters,
    /// \u{1f00c}: '🀌'
    MahjongTileSixOfCharacters,
    /// \u{1f00d}: '🀍'
    MahjongTileSevenOfCharacters,
    /// \u{1f00e}: '🀎'
    MahjongTileEightOfCharacters,
    /// \u{1f00f}: '🀏'
    MahjongTileNineOfCharacters,
    /// \u{1f010}: '🀐'
    MahjongTileOneOfBamboos,
    /// \u{1f011}: '🀑'
    MahjongTileTwoOfBamboos,
    /// \u{1f012}: '🀒'
    MahjongTileThreeOfBamboos,
    /// \u{1f013}: '🀓'
    MahjongTileFourOfBamboos,
    /// \u{1f014}: '🀔'
    MahjongTileFiveOfBamboos,
    /// \u{1f015}: '🀕'
    MahjongTileSixOfBamboos,
    /// \u{1f016}: '🀖'
    MahjongTileSevenOfBamboos,
    /// \u{1f017}: '🀗'
    MahjongTileEightOfBamboos,
    /// \u{1f018}: '🀘'
    MahjongTileNineOfBamboos,
    /// \u{1f019}: '🀙'
    MahjongTileOneOfCircles,
    /// \u{1f01a}: '🀚'
    MahjongTileTwoOfCircles,
    /// \u{1f01b}: '🀛'
    MahjongTileThreeOfCircles,
    /// \u{1f01c}: '🀜'
    MahjongTileFourOfCircles,
    /// \u{1f01d}: '🀝'
    MahjongTileFiveOfCircles,
    /// \u{1f01e}: '🀞'
    MahjongTileSixOfCircles,
    /// \u{1f01f}: '🀟'
    MahjongTileSevenOfCircles,
    /// \u{1f020}: '🀠'
    MahjongTileEightOfCircles,
    /// \u{1f021}: '🀡'
    MahjongTileNineOfCircles,
    /// \u{1f022}: '🀢'
    MahjongTilePlum,
    /// \u{1f023}: '🀣'
    MahjongTileOrchid,
    /// \u{1f024}: '🀤'
    MahjongTileBamboo,
    /// \u{1f025}: '🀥'
    MahjongTileChrysanthemum,
    /// \u{1f026}: '🀦'
    MahjongTileSpring,
    /// \u{1f027}: '🀧'
    MahjongTileSummer,
    /// \u{1f028}: '🀨'
    MahjongTileAutumn,
    /// \u{1f029}: '🀩'
    MahjongTileWinter,
    /// \u{1f02a}: '🀪'
    MahjongTileJoker,
    /// \u{1f02b}: '🀫'
    MahjongTileBack,
}

impl Into<char> for MahjongTiles {
    fn into(self) -> char {
        use constants::*;
        match self {
            MahjongTiles::MahjongTileEastWind => MAHJONG_TILE_EAST_WIND,
            MahjongTiles::MahjongTileSouthWind => MAHJONG_TILE_SOUTH_WIND,
            MahjongTiles::MahjongTileWestWind => MAHJONG_TILE_WEST_WIND,
            MahjongTiles::MahjongTileNorthWind => MAHJONG_TILE_NORTH_WIND,
            MahjongTiles::MahjongTileRedDragon => MAHJONG_TILE_RED_DRAGON,
            MahjongTiles::MahjongTileGreenDragon => MAHJONG_TILE_GREEN_DRAGON,
            MahjongTiles::MahjongTileWhiteDragon => MAHJONG_TILE_WHITE_DRAGON,
            MahjongTiles::MahjongTileOneOfCharacters => MAHJONG_TILE_ONE_OF_CHARACTERS,
            MahjongTiles::MahjongTileTwoOfCharacters => MAHJONG_TILE_TWO_OF_CHARACTERS,
            MahjongTiles::MahjongTileThreeOfCharacters => MAHJONG_TILE_THREE_OF_CHARACTERS,
            MahjongTiles::MahjongTileFourOfCharacters => MAHJONG_TILE_FOUR_OF_CHARACTERS,
            MahjongTiles::MahjongTileFiveOfCharacters => MAHJONG_TILE_FIVE_OF_CHARACTERS,
            MahjongTiles::MahjongTileSixOfCharacters => MAHJONG_TILE_SIX_OF_CHARACTERS,
            MahjongTiles::MahjongTileSevenOfCharacters => MAHJONG_TILE_SEVEN_OF_CHARACTERS,
            MahjongTiles::MahjongTileEightOfCharacters => MAHJONG_TILE_EIGHT_OF_CHARACTERS,
            MahjongTiles::MahjongTileNineOfCharacters => MAHJONG_TILE_NINE_OF_CHARACTERS,
            MahjongTiles::MahjongTileOneOfBamboos => MAHJONG_TILE_ONE_OF_BAMBOOS,
            MahjongTiles::MahjongTileTwoOfBamboos => MAHJONG_TILE_TWO_OF_BAMBOOS,
            MahjongTiles::MahjongTileThreeOfBamboos => MAHJONG_TILE_THREE_OF_BAMBOOS,
            MahjongTiles::MahjongTileFourOfBamboos => MAHJONG_TILE_FOUR_OF_BAMBOOS,
            MahjongTiles::MahjongTileFiveOfBamboos => MAHJONG_TILE_FIVE_OF_BAMBOOS,
            MahjongTiles::MahjongTileSixOfBamboos => MAHJONG_TILE_SIX_OF_BAMBOOS,
            MahjongTiles::MahjongTileSevenOfBamboos => MAHJONG_TILE_SEVEN_OF_BAMBOOS,
            MahjongTiles::MahjongTileEightOfBamboos => MAHJONG_TILE_EIGHT_OF_BAMBOOS,
            MahjongTiles::MahjongTileNineOfBamboos => MAHJONG_TILE_NINE_OF_BAMBOOS,
            MahjongTiles::MahjongTileOneOfCircles => MAHJONG_TILE_ONE_OF_CIRCLES,
            MahjongTiles::MahjongTileTwoOfCircles => MAHJONG_TILE_TWO_OF_CIRCLES,
            MahjongTiles::MahjongTileThreeOfCircles => MAHJONG_TILE_THREE_OF_CIRCLES,
            MahjongTiles::MahjongTileFourOfCircles => MAHJONG_TILE_FOUR_OF_CIRCLES,
            MahjongTiles::MahjongTileFiveOfCircles => MAHJONG_TILE_FIVE_OF_CIRCLES,
            MahjongTiles::MahjongTileSixOfCircles => MAHJONG_TILE_SIX_OF_CIRCLES,
            MahjongTiles::MahjongTileSevenOfCircles => MAHJONG_TILE_SEVEN_OF_CIRCLES,
            MahjongTiles::MahjongTileEightOfCircles => MAHJONG_TILE_EIGHT_OF_CIRCLES,
            MahjongTiles::MahjongTileNineOfCircles => MAHJONG_TILE_NINE_OF_CIRCLES,
            MahjongTiles::MahjongTilePlum => MAHJONG_TILE_PLUM,
            MahjongTiles::MahjongTileOrchid => MAHJONG_TILE_ORCHID,
            MahjongTiles::MahjongTileBamboo => MAHJONG_TILE_BAMBOO,
            MahjongTiles::MahjongTileChrysanthemum => MAHJONG_TILE_CHRYSANTHEMUM,
            MahjongTiles::MahjongTileSpring => MAHJONG_TILE_SPRING,
            MahjongTiles::MahjongTileSummer => MAHJONG_TILE_SUMMER,
            MahjongTiles::MahjongTileAutumn => MAHJONG_TILE_AUTUMN,
            MahjongTiles::MahjongTileWinter => MAHJONG_TILE_WINTER,
            MahjongTiles::MahjongTileJoker => MAHJONG_TILE_JOKER,
            MahjongTiles::MahjongTileBack => MAHJONG_TILE_BACK,
        }
    }
}

impl std::convert::TryFrom<char> for MahjongTiles {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            MAHJONG_TILE_EAST_WIND => Ok(MahjongTiles::MahjongTileEastWind),
            MAHJONG_TILE_SOUTH_WIND => Ok(MahjongTiles::MahjongTileSouthWind),
            MAHJONG_TILE_WEST_WIND => Ok(MahjongTiles::MahjongTileWestWind),
            MAHJONG_TILE_NORTH_WIND => Ok(MahjongTiles::MahjongTileNorthWind),
            MAHJONG_TILE_RED_DRAGON => Ok(MahjongTiles::MahjongTileRedDragon),
            MAHJONG_TILE_GREEN_DRAGON => Ok(MahjongTiles::MahjongTileGreenDragon),
            MAHJONG_TILE_WHITE_DRAGON => Ok(MahjongTiles::MahjongTileWhiteDragon),
            MAHJONG_TILE_ONE_OF_CHARACTERS => Ok(MahjongTiles::MahjongTileOneOfCharacters),
            MAHJONG_TILE_TWO_OF_CHARACTERS => Ok(MahjongTiles::MahjongTileTwoOfCharacters),
            MAHJONG_TILE_THREE_OF_CHARACTERS => Ok(MahjongTiles::MahjongTileThreeOfCharacters),
            MAHJONG_TILE_FOUR_OF_CHARACTERS => Ok(MahjongTiles::MahjongTileFourOfCharacters),
            MAHJONG_TILE_FIVE_OF_CHARACTERS => Ok(MahjongTiles::MahjongTileFiveOfCharacters),
            MAHJONG_TILE_SIX_OF_CHARACTERS => Ok(MahjongTiles::MahjongTileSixOfCharacters),
            MAHJONG_TILE_SEVEN_OF_CHARACTERS => Ok(MahjongTiles::MahjongTileSevenOfCharacters),
            MAHJONG_TILE_EIGHT_OF_CHARACTERS => Ok(MahjongTiles::MahjongTileEightOfCharacters),
            MAHJONG_TILE_NINE_OF_CHARACTERS => Ok(MahjongTiles::MahjongTileNineOfCharacters),
            MAHJONG_TILE_ONE_OF_BAMBOOS => Ok(MahjongTiles::MahjongTileOneOfBamboos),
            MAHJONG_TILE_TWO_OF_BAMBOOS => Ok(MahjongTiles::MahjongTileTwoOfBamboos),
            MAHJONG_TILE_THREE_OF_BAMBOOS => Ok(MahjongTiles::MahjongTileThreeOfBamboos),
            MAHJONG_TILE_FOUR_OF_BAMBOOS => Ok(MahjongTiles::MahjongTileFourOfBamboos),
            MAHJONG_TILE_FIVE_OF_BAMBOOS => Ok(MahjongTiles::MahjongTileFiveOfBamboos),
            MAHJONG_TILE_SIX_OF_BAMBOOS => Ok(MahjongTiles::MahjongTileSixOfBamboos),
            MAHJONG_TILE_SEVEN_OF_BAMBOOS => Ok(MahjongTiles::MahjongTileSevenOfBamboos),
            MAHJONG_TILE_EIGHT_OF_BAMBOOS => Ok(MahjongTiles::MahjongTileEightOfBamboos),
            MAHJONG_TILE_NINE_OF_BAMBOOS => Ok(MahjongTiles::MahjongTileNineOfBamboos),
            MAHJONG_TILE_ONE_OF_CIRCLES => Ok(MahjongTiles::MahjongTileOneOfCircles),
            MAHJONG_TILE_TWO_OF_CIRCLES => Ok(MahjongTiles::MahjongTileTwoOfCircles),
            MAHJONG_TILE_THREE_OF_CIRCLES => Ok(MahjongTiles::MahjongTileThreeOfCircles),
            MAHJONG_TILE_FOUR_OF_CIRCLES => Ok(MahjongTiles::MahjongTileFourOfCircles),
            MAHJONG_TILE_FIVE_OF_CIRCLES => Ok(MahjongTiles::MahjongTileFiveOfCircles),
            MAHJONG_TILE_SIX_OF_CIRCLES => Ok(MahjongTiles::MahjongTileSixOfCircles),
            MAHJONG_TILE_SEVEN_OF_CIRCLES => Ok(MahjongTiles::MahjongTileSevenOfCircles),
            MAHJONG_TILE_EIGHT_OF_CIRCLES => Ok(MahjongTiles::MahjongTileEightOfCircles),
            MAHJONG_TILE_NINE_OF_CIRCLES => Ok(MahjongTiles::MahjongTileNineOfCircles),
            MAHJONG_TILE_PLUM => Ok(MahjongTiles::MahjongTilePlum),
            MAHJONG_TILE_ORCHID => Ok(MahjongTiles::MahjongTileOrchid),
            MAHJONG_TILE_BAMBOO => Ok(MahjongTiles::MahjongTileBamboo),
            MAHJONG_TILE_CHRYSANTHEMUM => Ok(MahjongTiles::MahjongTileChrysanthemum),
            MAHJONG_TILE_SPRING => Ok(MahjongTiles::MahjongTileSpring),
            MAHJONG_TILE_SUMMER => Ok(MahjongTiles::MahjongTileSummer),
            MAHJONG_TILE_AUTUMN => Ok(MahjongTiles::MahjongTileAutumn),
            MAHJONG_TILE_WINTER => Ok(MahjongTiles::MahjongTileWinter),
            MAHJONG_TILE_JOKER => Ok(MahjongTiles::MahjongTileJoker),
            MAHJONG_TILE_BACK => Ok(MahjongTiles::MahjongTileBack),
            _ => Err(()),
        }
    }
}

impl Into<u32> for MahjongTiles {
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

impl std::convert::TryFrom<u32> for MahjongTiles {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for MahjongTiles {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl MahjongTiles {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        MahjongTiles::MahjongTileEastWind
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            MahjongTiles::MahjongTileEastWind => "mahjong tile east wind",
            MahjongTiles::MahjongTileSouthWind => "mahjong tile south wind",
            MahjongTiles::MahjongTileWestWind => "mahjong tile west wind",
            MahjongTiles::MahjongTileNorthWind => "mahjong tile north wind",
            MahjongTiles::MahjongTileRedDragon => "mahjong tile red dragon",
            MahjongTiles::MahjongTileGreenDragon => "mahjong tile green dragon",
            MahjongTiles::MahjongTileWhiteDragon => "mahjong tile white dragon",
            MahjongTiles::MahjongTileOneOfCharacters => "mahjong tile one of characters",
            MahjongTiles::MahjongTileTwoOfCharacters => "mahjong tile two of characters",
            MahjongTiles::MahjongTileThreeOfCharacters => "mahjong tile three of characters",
            MahjongTiles::MahjongTileFourOfCharacters => "mahjong tile four of characters",
            MahjongTiles::MahjongTileFiveOfCharacters => "mahjong tile five of characters",
            MahjongTiles::MahjongTileSixOfCharacters => "mahjong tile six of characters",
            MahjongTiles::MahjongTileSevenOfCharacters => "mahjong tile seven of characters",
            MahjongTiles::MahjongTileEightOfCharacters => "mahjong tile eight of characters",
            MahjongTiles::MahjongTileNineOfCharacters => "mahjong tile nine of characters",
            MahjongTiles::MahjongTileOneOfBamboos => "mahjong tile one of bamboos",
            MahjongTiles::MahjongTileTwoOfBamboos => "mahjong tile two of bamboos",
            MahjongTiles::MahjongTileThreeOfBamboos => "mahjong tile three of bamboos",
            MahjongTiles::MahjongTileFourOfBamboos => "mahjong tile four of bamboos",
            MahjongTiles::MahjongTileFiveOfBamboos => "mahjong tile five of bamboos",
            MahjongTiles::MahjongTileSixOfBamboos => "mahjong tile six of bamboos",
            MahjongTiles::MahjongTileSevenOfBamboos => "mahjong tile seven of bamboos",
            MahjongTiles::MahjongTileEightOfBamboos => "mahjong tile eight of bamboos",
            MahjongTiles::MahjongTileNineOfBamboos => "mahjong tile nine of bamboos",
            MahjongTiles::MahjongTileOneOfCircles => "mahjong tile one of circles",
            MahjongTiles::MahjongTileTwoOfCircles => "mahjong tile two of circles",
            MahjongTiles::MahjongTileThreeOfCircles => "mahjong tile three of circles",
            MahjongTiles::MahjongTileFourOfCircles => "mahjong tile four of circles",
            MahjongTiles::MahjongTileFiveOfCircles => "mahjong tile five of circles",
            MahjongTiles::MahjongTileSixOfCircles => "mahjong tile six of circles",
            MahjongTiles::MahjongTileSevenOfCircles => "mahjong tile seven of circles",
            MahjongTiles::MahjongTileEightOfCircles => "mahjong tile eight of circles",
            MahjongTiles::MahjongTileNineOfCircles => "mahjong tile nine of circles",
            MahjongTiles::MahjongTilePlum => "mahjong tile plum",
            MahjongTiles::MahjongTileOrchid => "mahjong tile orchid",
            MahjongTiles::MahjongTileBamboo => "mahjong tile bamboo",
            MahjongTiles::MahjongTileChrysanthemum => "mahjong tile chrysanthemum",
            MahjongTiles::MahjongTileSpring => "mahjong tile spring",
            MahjongTiles::MahjongTileSummer => "mahjong tile summer",
            MahjongTiles::MahjongTileAutumn => "mahjong tile autumn",
            MahjongTiles::MahjongTileWinter => "mahjong tile winter",
            MahjongTiles::MahjongTileJoker => "mahjong tile joker",
            MahjongTiles::MahjongTileBack => "mahjong tile back",
        }
    }
}
