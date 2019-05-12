/// \u{2580} → \u{259f}\
///\
/// ▀ ▁ ▂ ▃ ▄ ▅ ▆ ▇ █ ▉ ▊ ▋ ▌ ▍ ▎ ▏\
/// ▐ ░ ▒ ▓ ▔ ▕ ▖ ▗ ▘ ▙ ▚ ▛ ▜ ▝ ▞\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{2580}: '▀'
    pub const UPPER_HALF_BLOCK: char = '▀';
    /// \u{2581}: '▁'
    pub const LOWER_ONE_EIGHTH_BLOCK: char = '▁';
    /// \u{2582}: '▂'
    pub const LOWER_ONE_QUARTER_BLOCK: char = '▂';
    /// \u{2583}: '▃'
    pub const LOWER_THREE_EIGHTHS_BLOCK: char = '▃';
    /// \u{2584}: '▄'
    pub const LOWER_HALF_BLOCK: char = '▄';
    /// \u{2585}: '▅'
    pub const LOWER_FIVE_EIGHTHS_BLOCK: char = '▅';
    /// \u{2586}: '▆'
    pub const LOWER_THREE_QUARTERS_BLOCK: char = '▆';
    /// \u{2587}: '▇'
    pub const LOWER_SEVEN_EIGHTHS_BLOCK: char = '▇';
    /// \u{2588}: '█'
    pub const FULL_BLOCK: char = '█';
    /// \u{2589}: '▉'
    pub const LEFT_SEVEN_EIGHTHS_BLOCK: char = '▉';
    /// \u{258a}: '▊'
    pub const LEFT_THREE_QUARTERS_BLOCK: char = '▊';
    /// \u{258b}: '▋'
    pub const LEFT_FIVE_EIGHTHS_BLOCK: char = '▋';
    /// \u{258c}: '▌'
    pub const LEFT_HALF_BLOCK: char = '▌';
    /// \u{258d}: '▍'
    pub const LEFT_THREE_EIGHTHS_BLOCK: char = '▍';
    /// \u{258e}: '▎'
    pub const LEFT_ONE_QUARTER_BLOCK: char = '▎';
    /// \u{258f}: '▏'
    pub const LEFT_ONE_EIGHTH_BLOCK: char = '▏';
    /// \u{2590}: '▐'
    pub const RIGHT_HALF_BLOCK: char = '▐';
    /// \u{2591}: '░'
    pub const LIGHT_SHADE: char = '░';
    /// \u{2592}: '▒'
    pub const MEDIUM_SHADE: char = '▒';
    /// \u{2593}: '▓'
    pub const DARK_SHADE: char = '▓';
    /// \u{2594}: '▔'
    pub const UPPER_ONE_EIGHTH_BLOCK: char = '▔';
    /// \u{2595}: '▕'
    pub const RIGHT_ONE_EIGHTH_BLOCK: char = '▕';
    /// \u{2596}: '▖'
    pub const QUADRANT_LOWER_LEFT: char = '▖';
    /// \u{2597}: '▗'
    pub const QUADRANT_LOWER_RIGHT: char = '▗';
    /// \u{2598}: '▘'
    pub const QUADRANT_UPPER_LEFT: char = '▘';
    /// \u{2599}: '▙'
    pub const QUADRANT_UPPER_LEFT_AND_LOWER_LEFT_AND_LOWER_RIGHT: char = '▙';
    /// \u{259a}: '▚'
    pub const QUADRANT_UPPER_LEFT_AND_LOWER_RIGHT: char = '▚';
    /// \u{259b}: '▛'
    pub const QUADRANT_UPPER_LEFT_AND_UPPER_RIGHT_AND_LOWER_LEFT: char = '▛';
    /// \u{259c}: '▜'
    pub const QUADRANT_UPPER_LEFT_AND_UPPER_RIGHT_AND_LOWER_RIGHT: char = '▜';
    /// \u{259d}: '▝'
    pub const QUADRANT_UPPER_RIGHT: char = '▝';
    /// \u{259e}: '▞'
    pub const QUADRANT_UPPER_RIGHT_AND_LOWER_LEFT: char = '▞';
}

/// An enum to represent all characters in the BlockElements block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum BlockElements {
    /// \u{2580}: '▀'
    UpperHalfBlock,
    /// \u{2581}: '▁'
    LowerOneEighthBlock,
    /// \u{2582}: '▂'
    LowerOneQuarterBlock,
    /// \u{2583}: '▃'
    LowerThreeEighthsBlock,
    /// \u{2584}: '▄'
    LowerHalfBlock,
    /// \u{2585}: '▅'
    LowerFiveEighthsBlock,
    /// \u{2586}: '▆'
    LowerThreeQuartersBlock,
    /// \u{2587}: '▇'
    LowerSevenEighthsBlock,
    /// \u{2588}: '█'
    FullBlock,
    /// \u{2589}: '▉'
    LeftSevenEighthsBlock,
    /// \u{258a}: '▊'
    LeftThreeQuartersBlock,
    /// \u{258b}: '▋'
    LeftFiveEighthsBlock,
    /// \u{258c}: '▌'
    LeftHalfBlock,
    /// \u{258d}: '▍'
    LeftThreeEighthsBlock,
    /// \u{258e}: '▎'
    LeftOneQuarterBlock,
    /// \u{258f}: '▏'
    LeftOneEighthBlock,
    /// \u{2590}: '▐'
    RightHalfBlock,
    /// \u{2591}: '░'
    LightShade,
    /// \u{2592}: '▒'
    MediumShade,
    /// \u{2593}: '▓'
    DarkShade,
    /// \u{2594}: '▔'
    UpperOneEighthBlock,
    /// \u{2595}: '▕'
    RightOneEighthBlock,
    /// \u{2596}: '▖'
    QuadrantLowerLeft,
    /// \u{2597}: '▗'
    QuadrantLowerRight,
    /// \u{2598}: '▘'
    QuadrantUpperLeft,
    /// \u{2599}: '▙'
    QuadrantUpperLeftAndLowerLeftAndLowerRight,
    /// \u{259a}: '▚'
    QuadrantUpperLeftAndLowerRight,
    /// \u{259b}: '▛'
    QuadrantUpperLeftAndUpperRightAndLowerLeft,
    /// \u{259c}: '▜'
    QuadrantUpperLeftAndUpperRightAndLowerRight,
    /// \u{259d}: '▝'
    QuadrantUpperRight,
    /// \u{259e}: '▞'
    QuadrantUpperRightAndLowerLeft,
}

impl Into<char> for BlockElements {
    fn into(self) -> char {
        use constants::*;
        match self {
            BlockElements::UpperHalfBlock => UPPER_HALF_BLOCK,
            BlockElements::LowerOneEighthBlock => LOWER_ONE_EIGHTH_BLOCK,
            BlockElements::LowerOneQuarterBlock => LOWER_ONE_QUARTER_BLOCK,
            BlockElements::LowerThreeEighthsBlock => LOWER_THREE_EIGHTHS_BLOCK,
            BlockElements::LowerHalfBlock => LOWER_HALF_BLOCK,
            BlockElements::LowerFiveEighthsBlock => LOWER_FIVE_EIGHTHS_BLOCK,
            BlockElements::LowerThreeQuartersBlock => LOWER_THREE_QUARTERS_BLOCK,
            BlockElements::LowerSevenEighthsBlock => LOWER_SEVEN_EIGHTHS_BLOCK,
            BlockElements::FullBlock => FULL_BLOCK,
            BlockElements::LeftSevenEighthsBlock => LEFT_SEVEN_EIGHTHS_BLOCK,
            BlockElements::LeftThreeQuartersBlock => LEFT_THREE_QUARTERS_BLOCK,
            BlockElements::LeftFiveEighthsBlock => LEFT_FIVE_EIGHTHS_BLOCK,
            BlockElements::LeftHalfBlock => LEFT_HALF_BLOCK,
            BlockElements::LeftThreeEighthsBlock => LEFT_THREE_EIGHTHS_BLOCK,
            BlockElements::LeftOneQuarterBlock => LEFT_ONE_QUARTER_BLOCK,
            BlockElements::LeftOneEighthBlock => LEFT_ONE_EIGHTH_BLOCK,
            BlockElements::RightHalfBlock => RIGHT_HALF_BLOCK,
            BlockElements::LightShade => LIGHT_SHADE,
            BlockElements::MediumShade => MEDIUM_SHADE,
            BlockElements::DarkShade => DARK_SHADE,
            BlockElements::UpperOneEighthBlock => UPPER_ONE_EIGHTH_BLOCK,
            BlockElements::RightOneEighthBlock => RIGHT_ONE_EIGHTH_BLOCK,
            BlockElements::QuadrantLowerLeft => QUADRANT_LOWER_LEFT,
            BlockElements::QuadrantLowerRight => QUADRANT_LOWER_RIGHT,
            BlockElements::QuadrantUpperLeft => QUADRANT_UPPER_LEFT,
            BlockElements::QuadrantUpperLeftAndLowerLeftAndLowerRight => QUADRANT_UPPER_LEFT_AND_LOWER_LEFT_AND_LOWER_RIGHT,
            BlockElements::QuadrantUpperLeftAndLowerRight => QUADRANT_UPPER_LEFT_AND_LOWER_RIGHT,
            BlockElements::QuadrantUpperLeftAndUpperRightAndLowerLeft => QUADRANT_UPPER_LEFT_AND_UPPER_RIGHT_AND_LOWER_LEFT,
            BlockElements::QuadrantUpperLeftAndUpperRightAndLowerRight => QUADRANT_UPPER_LEFT_AND_UPPER_RIGHT_AND_LOWER_RIGHT,
            BlockElements::QuadrantUpperRight => QUADRANT_UPPER_RIGHT,
            BlockElements::QuadrantUpperRightAndLowerLeft => QUADRANT_UPPER_RIGHT_AND_LOWER_LEFT,
        }
    }
}

impl std::convert::TryFrom<char> for BlockElements {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            UPPER_HALF_BLOCK => Ok(BlockElements::UpperHalfBlock),
            LOWER_ONE_EIGHTH_BLOCK => Ok(BlockElements::LowerOneEighthBlock),
            LOWER_ONE_QUARTER_BLOCK => Ok(BlockElements::LowerOneQuarterBlock),
            LOWER_THREE_EIGHTHS_BLOCK => Ok(BlockElements::LowerThreeEighthsBlock),
            LOWER_HALF_BLOCK => Ok(BlockElements::LowerHalfBlock),
            LOWER_FIVE_EIGHTHS_BLOCK => Ok(BlockElements::LowerFiveEighthsBlock),
            LOWER_THREE_QUARTERS_BLOCK => Ok(BlockElements::LowerThreeQuartersBlock),
            LOWER_SEVEN_EIGHTHS_BLOCK => Ok(BlockElements::LowerSevenEighthsBlock),
            FULL_BLOCK => Ok(BlockElements::FullBlock),
            LEFT_SEVEN_EIGHTHS_BLOCK => Ok(BlockElements::LeftSevenEighthsBlock),
            LEFT_THREE_QUARTERS_BLOCK => Ok(BlockElements::LeftThreeQuartersBlock),
            LEFT_FIVE_EIGHTHS_BLOCK => Ok(BlockElements::LeftFiveEighthsBlock),
            LEFT_HALF_BLOCK => Ok(BlockElements::LeftHalfBlock),
            LEFT_THREE_EIGHTHS_BLOCK => Ok(BlockElements::LeftThreeEighthsBlock),
            LEFT_ONE_QUARTER_BLOCK => Ok(BlockElements::LeftOneQuarterBlock),
            LEFT_ONE_EIGHTH_BLOCK => Ok(BlockElements::LeftOneEighthBlock),
            RIGHT_HALF_BLOCK => Ok(BlockElements::RightHalfBlock),
            LIGHT_SHADE => Ok(BlockElements::LightShade),
            MEDIUM_SHADE => Ok(BlockElements::MediumShade),
            DARK_SHADE => Ok(BlockElements::DarkShade),
            UPPER_ONE_EIGHTH_BLOCK => Ok(BlockElements::UpperOneEighthBlock),
            RIGHT_ONE_EIGHTH_BLOCK => Ok(BlockElements::RightOneEighthBlock),
            QUADRANT_LOWER_LEFT => Ok(BlockElements::QuadrantLowerLeft),
            QUADRANT_LOWER_RIGHT => Ok(BlockElements::QuadrantLowerRight),
            QUADRANT_UPPER_LEFT => Ok(BlockElements::QuadrantUpperLeft),
            QUADRANT_UPPER_LEFT_AND_LOWER_LEFT_AND_LOWER_RIGHT => Ok(BlockElements::QuadrantUpperLeftAndLowerLeftAndLowerRight),
            QUADRANT_UPPER_LEFT_AND_LOWER_RIGHT => Ok(BlockElements::QuadrantUpperLeftAndLowerRight),
            QUADRANT_UPPER_LEFT_AND_UPPER_RIGHT_AND_LOWER_LEFT => Ok(BlockElements::QuadrantUpperLeftAndUpperRightAndLowerLeft),
            QUADRANT_UPPER_LEFT_AND_UPPER_RIGHT_AND_LOWER_RIGHT => Ok(BlockElements::QuadrantUpperLeftAndUpperRightAndLowerRight),
            QUADRANT_UPPER_RIGHT => Ok(BlockElements::QuadrantUpperRight),
            QUADRANT_UPPER_RIGHT_AND_LOWER_LEFT => Ok(BlockElements::QuadrantUpperRightAndLowerLeft),
            _ => Err(()),
        }
    }
}

impl Into<u32> for BlockElements {
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

impl std::convert::TryFrom<u32> for BlockElements {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for BlockElements {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl BlockElements {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        BlockElements::UpperHalfBlock
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            BlockElements::UpperHalfBlock => "upper half block",
            BlockElements::LowerOneEighthBlock => "lower one eighth block",
            BlockElements::LowerOneQuarterBlock => "lower one quarter block",
            BlockElements::LowerThreeEighthsBlock => "lower three eighths block",
            BlockElements::LowerHalfBlock => "lower half block",
            BlockElements::LowerFiveEighthsBlock => "lower five eighths block",
            BlockElements::LowerThreeQuartersBlock => "lower three quarters block",
            BlockElements::LowerSevenEighthsBlock => "lower seven eighths block",
            BlockElements::FullBlock => "full block",
            BlockElements::LeftSevenEighthsBlock => "left seven eighths block",
            BlockElements::LeftThreeQuartersBlock => "left three quarters block",
            BlockElements::LeftFiveEighthsBlock => "left five eighths block",
            BlockElements::LeftHalfBlock => "left half block",
            BlockElements::LeftThreeEighthsBlock => "left three eighths block",
            BlockElements::LeftOneQuarterBlock => "left one quarter block",
            BlockElements::LeftOneEighthBlock => "left one eighth block",
            BlockElements::RightHalfBlock => "right half block",
            BlockElements::LightShade => "light shade",
            BlockElements::MediumShade => "medium shade",
            BlockElements::DarkShade => "dark shade",
            BlockElements::UpperOneEighthBlock => "upper one eighth block",
            BlockElements::RightOneEighthBlock => "right one eighth block",
            BlockElements::QuadrantLowerLeft => "quadrant lower left",
            BlockElements::QuadrantLowerRight => "quadrant lower right",
            BlockElements::QuadrantUpperLeft => "quadrant upper left",
            BlockElements::QuadrantUpperLeftAndLowerLeftAndLowerRight => "quadrant upper left and lower left and lower right",
            BlockElements::QuadrantUpperLeftAndLowerRight => "quadrant upper left and lower right",
            BlockElements::QuadrantUpperLeftAndUpperRightAndLowerLeft => "quadrant upper left and upper right and lower left",
            BlockElements::QuadrantUpperLeftAndUpperRightAndLowerRight => "quadrant upper left and upper right and lower right",
            BlockElements::QuadrantUpperRight => "quadrant upper right",
            BlockElements::QuadrantUpperRightAndLowerLeft => "quadrant upper right and lower left",
        }
    }
}
