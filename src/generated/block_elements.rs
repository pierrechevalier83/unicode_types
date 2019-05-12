
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
        match self {
            BlockElements::UpperHalfBlock => '▀',
            BlockElements::LowerOneEighthBlock => '▁',
            BlockElements::LowerOneQuarterBlock => '▂',
            BlockElements::LowerThreeEighthsBlock => '▃',
            BlockElements::LowerHalfBlock => '▄',
            BlockElements::LowerFiveEighthsBlock => '▅',
            BlockElements::LowerThreeQuartersBlock => '▆',
            BlockElements::LowerSevenEighthsBlock => '▇',
            BlockElements::FullBlock => '█',
            BlockElements::LeftSevenEighthsBlock => '▉',
            BlockElements::LeftThreeQuartersBlock => '▊',
            BlockElements::LeftFiveEighthsBlock => '▋',
            BlockElements::LeftHalfBlock => '▌',
            BlockElements::LeftThreeEighthsBlock => '▍',
            BlockElements::LeftOneQuarterBlock => '▎',
            BlockElements::LeftOneEighthBlock => '▏',
            BlockElements::RightHalfBlock => '▐',
            BlockElements::LightShade => '░',
            BlockElements::MediumShade => '▒',
            BlockElements::DarkShade => '▓',
            BlockElements::UpperOneEighthBlock => '▔',
            BlockElements::RightOneEighthBlock => '▕',
            BlockElements::QuadrantLowerLeft => '▖',
            BlockElements::QuadrantLowerRight => '▗',
            BlockElements::QuadrantUpperLeft => '▘',
            BlockElements::QuadrantUpperLeftAndLowerLeftAndLowerRight => '▙',
            BlockElements::QuadrantUpperLeftAndLowerRight => '▚',
            BlockElements::QuadrantUpperLeftAndUpperRightAndLowerLeft => '▛',
            BlockElements::QuadrantUpperLeftAndUpperRightAndLowerRight => '▜',
            BlockElements::QuadrantUpperRight => '▝',
            BlockElements::QuadrantUpperRightAndLowerLeft => '▞',
        }
    }
}

impl std::convert::TryFrom<char> for BlockElements {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '▀' => Ok(BlockElements::UpperHalfBlock),
            '▁' => Ok(BlockElements::LowerOneEighthBlock),
            '▂' => Ok(BlockElements::LowerOneQuarterBlock),
            '▃' => Ok(BlockElements::LowerThreeEighthsBlock),
            '▄' => Ok(BlockElements::LowerHalfBlock),
            '▅' => Ok(BlockElements::LowerFiveEighthsBlock),
            '▆' => Ok(BlockElements::LowerThreeQuartersBlock),
            '▇' => Ok(BlockElements::LowerSevenEighthsBlock),
            '█' => Ok(BlockElements::FullBlock),
            '▉' => Ok(BlockElements::LeftSevenEighthsBlock),
            '▊' => Ok(BlockElements::LeftThreeQuartersBlock),
            '▋' => Ok(BlockElements::LeftFiveEighthsBlock),
            '▌' => Ok(BlockElements::LeftHalfBlock),
            '▍' => Ok(BlockElements::LeftThreeEighthsBlock),
            '▎' => Ok(BlockElements::LeftOneQuarterBlock),
            '▏' => Ok(BlockElements::LeftOneEighthBlock),
            '▐' => Ok(BlockElements::RightHalfBlock),
            '░' => Ok(BlockElements::LightShade),
            '▒' => Ok(BlockElements::MediumShade),
            '▓' => Ok(BlockElements::DarkShade),
            '▔' => Ok(BlockElements::UpperOneEighthBlock),
            '▕' => Ok(BlockElements::RightOneEighthBlock),
            '▖' => Ok(BlockElements::QuadrantLowerLeft),
            '▗' => Ok(BlockElements::QuadrantLowerRight),
            '▘' => Ok(BlockElements::QuadrantUpperLeft),
            '▙' => Ok(BlockElements::QuadrantUpperLeftAndLowerLeftAndLowerRight),
            '▚' => Ok(BlockElements::QuadrantUpperLeftAndLowerRight),
            '▛' => Ok(BlockElements::QuadrantUpperLeftAndUpperRightAndLowerLeft),
            '▜' => Ok(BlockElements::QuadrantUpperLeftAndUpperRightAndLowerRight),
            '▝' => Ok(BlockElements::QuadrantUpperRight),
            '▞' => Ok(BlockElements::QuadrantUpperRightAndLowerLeft),
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
