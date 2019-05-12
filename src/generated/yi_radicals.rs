
/// An enum to represent all characters in the YiRadicals block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum YiRadicals {
    /// \u{a490}: '꒐'
    YiRadicalQot,
    /// \u{a491}: '꒑'
    YiRadicalLi,
    /// \u{a492}: '꒒'
    YiRadicalKit,
    /// \u{a493}: '꒓'
    YiRadicalNyip,
    /// \u{a494}: '꒔'
    YiRadicalCyp,
    /// \u{a495}: '꒕'
    YiRadicalSsi,
    /// \u{a496}: '꒖'
    YiRadicalGgop,
    /// \u{a497}: '꒗'
    YiRadicalGep,
    /// \u{a498}: '꒘'
    YiRadicalMi,
    /// \u{a499}: '꒙'
    YiRadicalHxit,
    /// \u{a49a}: '꒚'
    YiRadicalLyr,
    /// \u{a49b}: '꒛'
    YiRadicalBbut,
    /// \u{a49c}: '꒜'
    YiRadicalMop,
    /// \u{a49d}: '꒝'
    YiRadicalYo,
    /// \u{a49e}: '꒞'
    YiRadicalPut,
    /// \u{a49f}: '꒟'
    YiRadicalHxuo,
    /// \u{a4a0}: '꒠'
    YiRadicalTat,
    /// \u{a4a1}: '꒡'
    YiRadicalGa,
    /// \u{a4a2}: '꒢'
    YiRadicalZup,
    /// \u{a4a3}: '꒣'
    YiRadicalCyt,
    /// \u{a4a4}: '꒤'
    YiRadicalDdur,
    /// \u{a4a5}: '꒥'
    YiRadicalBur,
    /// \u{a4a6}: '꒦'
    YiRadicalGguo,
    /// \u{a4a7}: '꒧'
    YiRadicalNyop,
    /// \u{a4a8}: '꒨'
    YiRadicalTu,
    /// \u{a4a9}: '꒩'
    YiRadicalOp,
    /// \u{a4aa}: '꒪'
    YiRadicalJjut,
    /// \u{a4ab}: '꒫'
    YiRadicalZot,
    /// \u{a4ac}: '꒬'
    YiRadicalPyt,
    /// \u{a4ad}: '꒭'
    YiRadicalHmo,
    /// \u{a4ae}: '꒮'
    YiRadicalYit,
    /// \u{a4af}: '꒯'
    YiRadicalVur,
    /// \u{a4b0}: '꒰'
    YiRadicalShy,
    /// \u{a4b1}: '꒱'
    YiRadicalVep,
    /// \u{a4b2}: '꒲'
    YiRadicalZa,
    /// \u{a4b3}: '꒳'
    YiRadicalJo,
    /// \u{a4b4}: '꒴'
    YiRadicalNzup,
    /// \u{a4b5}: '꒵'
    YiRadicalJjy,
    /// \u{a4b6}: '꒶'
    YiRadicalGot,
    /// \u{a4b7}: '꒷'
    YiRadicalJjie,
    /// \u{a4b8}: '꒸'
    YiRadicalWo,
    /// \u{a4b9}: '꒹'
    YiRadicalDu,
    /// \u{a4ba}: '꒺'
    YiRadicalShur,
    /// \u{a4bb}: '꒻'
    YiRadicalLie,
    /// \u{a4bc}: '꒼'
    YiRadicalCy,
    /// \u{a4bd}: '꒽'
    YiRadicalCuop,
    /// \u{a4be}: '꒾'
    YiRadicalCip,
    /// \u{a4bf}: '꒿'
    YiRadicalHxop,
    /// \u{a4c0}: '꓀'
    YiRadicalShat,
    /// \u{a4c1}: '꓁'
    YiRadicalZur,
    /// \u{a4c2}: '꓂'
    YiRadicalShop,
    /// \u{a4c3}: '꓃'
    YiRadicalChe,
    /// \u{a4c4}: '꓄'
    YiRadicalZziet,
    /// \u{a4c5}: '꓅'
    YiRadicalNbie,
    /// \u{a4c6}: '꓆'
    YiRadicalKe,
}

impl Into<char> for YiRadicals {
    fn into(self) -> char {
        match self {
            YiRadicals::YiRadicalQot => '꒐',
            YiRadicals::YiRadicalLi => '꒑',
            YiRadicals::YiRadicalKit => '꒒',
            YiRadicals::YiRadicalNyip => '꒓',
            YiRadicals::YiRadicalCyp => '꒔',
            YiRadicals::YiRadicalSsi => '꒕',
            YiRadicals::YiRadicalGgop => '꒖',
            YiRadicals::YiRadicalGep => '꒗',
            YiRadicals::YiRadicalMi => '꒘',
            YiRadicals::YiRadicalHxit => '꒙',
            YiRadicals::YiRadicalLyr => '꒚',
            YiRadicals::YiRadicalBbut => '꒛',
            YiRadicals::YiRadicalMop => '꒜',
            YiRadicals::YiRadicalYo => '꒝',
            YiRadicals::YiRadicalPut => '꒞',
            YiRadicals::YiRadicalHxuo => '꒟',
            YiRadicals::YiRadicalTat => '꒠',
            YiRadicals::YiRadicalGa => '꒡',
            YiRadicals::YiRadicalZup => '꒢',
            YiRadicals::YiRadicalCyt => '꒣',
            YiRadicals::YiRadicalDdur => '꒤',
            YiRadicals::YiRadicalBur => '꒥',
            YiRadicals::YiRadicalGguo => '꒦',
            YiRadicals::YiRadicalNyop => '꒧',
            YiRadicals::YiRadicalTu => '꒨',
            YiRadicals::YiRadicalOp => '꒩',
            YiRadicals::YiRadicalJjut => '꒪',
            YiRadicals::YiRadicalZot => '꒫',
            YiRadicals::YiRadicalPyt => '꒬',
            YiRadicals::YiRadicalHmo => '꒭',
            YiRadicals::YiRadicalYit => '꒮',
            YiRadicals::YiRadicalVur => '꒯',
            YiRadicals::YiRadicalShy => '꒰',
            YiRadicals::YiRadicalVep => '꒱',
            YiRadicals::YiRadicalZa => '꒲',
            YiRadicals::YiRadicalJo => '꒳',
            YiRadicals::YiRadicalNzup => '꒴',
            YiRadicals::YiRadicalJjy => '꒵',
            YiRadicals::YiRadicalGot => '꒶',
            YiRadicals::YiRadicalJjie => '꒷',
            YiRadicals::YiRadicalWo => '꒸',
            YiRadicals::YiRadicalDu => '꒹',
            YiRadicals::YiRadicalShur => '꒺',
            YiRadicals::YiRadicalLie => '꒻',
            YiRadicals::YiRadicalCy => '꒼',
            YiRadicals::YiRadicalCuop => '꒽',
            YiRadicals::YiRadicalCip => '꒾',
            YiRadicals::YiRadicalHxop => '꒿',
            YiRadicals::YiRadicalShat => '꓀',
            YiRadicals::YiRadicalZur => '꓁',
            YiRadicals::YiRadicalShop => '꓂',
            YiRadicals::YiRadicalChe => '꓃',
            YiRadicals::YiRadicalZziet => '꓄',
            YiRadicals::YiRadicalNbie => '꓅',
            YiRadicals::YiRadicalKe => '꓆',
        }
    }
}

impl std::convert::TryFrom<char> for YiRadicals {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '꒐' => Ok(YiRadicals::YiRadicalQot),
            '꒑' => Ok(YiRadicals::YiRadicalLi),
            '꒒' => Ok(YiRadicals::YiRadicalKit),
            '꒓' => Ok(YiRadicals::YiRadicalNyip),
            '꒔' => Ok(YiRadicals::YiRadicalCyp),
            '꒕' => Ok(YiRadicals::YiRadicalSsi),
            '꒖' => Ok(YiRadicals::YiRadicalGgop),
            '꒗' => Ok(YiRadicals::YiRadicalGep),
            '꒘' => Ok(YiRadicals::YiRadicalMi),
            '꒙' => Ok(YiRadicals::YiRadicalHxit),
            '꒚' => Ok(YiRadicals::YiRadicalLyr),
            '꒛' => Ok(YiRadicals::YiRadicalBbut),
            '꒜' => Ok(YiRadicals::YiRadicalMop),
            '꒝' => Ok(YiRadicals::YiRadicalYo),
            '꒞' => Ok(YiRadicals::YiRadicalPut),
            '꒟' => Ok(YiRadicals::YiRadicalHxuo),
            '꒠' => Ok(YiRadicals::YiRadicalTat),
            '꒡' => Ok(YiRadicals::YiRadicalGa),
            '꒢' => Ok(YiRadicals::YiRadicalZup),
            '꒣' => Ok(YiRadicals::YiRadicalCyt),
            '꒤' => Ok(YiRadicals::YiRadicalDdur),
            '꒥' => Ok(YiRadicals::YiRadicalBur),
            '꒦' => Ok(YiRadicals::YiRadicalGguo),
            '꒧' => Ok(YiRadicals::YiRadicalNyop),
            '꒨' => Ok(YiRadicals::YiRadicalTu),
            '꒩' => Ok(YiRadicals::YiRadicalOp),
            '꒪' => Ok(YiRadicals::YiRadicalJjut),
            '꒫' => Ok(YiRadicals::YiRadicalZot),
            '꒬' => Ok(YiRadicals::YiRadicalPyt),
            '꒭' => Ok(YiRadicals::YiRadicalHmo),
            '꒮' => Ok(YiRadicals::YiRadicalYit),
            '꒯' => Ok(YiRadicals::YiRadicalVur),
            '꒰' => Ok(YiRadicals::YiRadicalShy),
            '꒱' => Ok(YiRadicals::YiRadicalVep),
            '꒲' => Ok(YiRadicals::YiRadicalZa),
            '꒳' => Ok(YiRadicals::YiRadicalJo),
            '꒴' => Ok(YiRadicals::YiRadicalNzup),
            '꒵' => Ok(YiRadicals::YiRadicalJjy),
            '꒶' => Ok(YiRadicals::YiRadicalGot),
            '꒷' => Ok(YiRadicals::YiRadicalJjie),
            '꒸' => Ok(YiRadicals::YiRadicalWo),
            '꒹' => Ok(YiRadicals::YiRadicalDu),
            '꒺' => Ok(YiRadicals::YiRadicalShur),
            '꒻' => Ok(YiRadicals::YiRadicalLie),
            '꒼' => Ok(YiRadicals::YiRadicalCy),
            '꒽' => Ok(YiRadicals::YiRadicalCuop),
            '꒾' => Ok(YiRadicals::YiRadicalCip),
            '꒿' => Ok(YiRadicals::YiRadicalHxop),
            '꓀' => Ok(YiRadicals::YiRadicalShat),
            '꓁' => Ok(YiRadicals::YiRadicalZur),
            '꓂' => Ok(YiRadicals::YiRadicalShop),
            '꓃' => Ok(YiRadicals::YiRadicalChe),
            '꓄' => Ok(YiRadicals::YiRadicalZziet),
            '꓅' => Ok(YiRadicals::YiRadicalNbie),
            '꓆' => Ok(YiRadicals::YiRadicalKe),
            _ => Err(()),
        }
    }
}

impl Into<u32> for YiRadicals {
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

impl std::convert::TryFrom<u32> for YiRadicals {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for YiRadicals {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl YiRadicals {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        YiRadicals::YiRadicalQot
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            YiRadicals::YiRadicalQot => "yi radical qot",
            YiRadicals::YiRadicalLi => "yi radical li",
            YiRadicals::YiRadicalKit => "yi radical kit",
            YiRadicals::YiRadicalNyip => "yi radical nyip",
            YiRadicals::YiRadicalCyp => "yi radical cyp",
            YiRadicals::YiRadicalSsi => "yi radical ssi",
            YiRadicals::YiRadicalGgop => "yi radical ggop",
            YiRadicals::YiRadicalGep => "yi radical gep",
            YiRadicals::YiRadicalMi => "yi radical mi",
            YiRadicals::YiRadicalHxit => "yi radical hxit",
            YiRadicals::YiRadicalLyr => "yi radical lyr",
            YiRadicals::YiRadicalBbut => "yi radical bbut",
            YiRadicals::YiRadicalMop => "yi radical mop",
            YiRadicals::YiRadicalYo => "yi radical yo",
            YiRadicals::YiRadicalPut => "yi radical put",
            YiRadicals::YiRadicalHxuo => "yi radical hxuo",
            YiRadicals::YiRadicalTat => "yi radical tat",
            YiRadicals::YiRadicalGa => "yi radical ga",
            YiRadicals::YiRadicalZup => "yi radical zup",
            YiRadicals::YiRadicalCyt => "yi radical cyt",
            YiRadicals::YiRadicalDdur => "yi radical ddur",
            YiRadicals::YiRadicalBur => "yi radical bur",
            YiRadicals::YiRadicalGguo => "yi radical gguo",
            YiRadicals::YiRadicalNyop => "yi radical nyop",
            YiRadicals::YiRadicalTu => "yi radical tu",
            YiRadicals::YiRadicalOp => "yi radical op",
            YiRadicals::YiRadicalJjut => "yi radical jjut",
            YiRadicals::YiRadicalZot => "yi radical zot",
            YiRadicals::YiRadicalPyt => "yi radical pyt",
            YiRadicals::YiRadicalHmo => "yi radical hmo",
            YiRadicals::YiRadicalYit => "yi radical yit",
            YiRadicals::YiRadicalVur => "yi radical vur",
            YiRadicals::YiRadicalShy => "yi radical shy",
            YiRadicals::YiRadicalVep => "yi radical vep",
            YiRadicals::YiRadicalZa => "yi radical za",
            YiRadicals::YiRadicalJo => "yi radical jo",
            YiRadicals::YiRadicalNzup => "yi radical nzup",
            YiRadicals::YiRadicalJjy => "yi radical jjy",
            YiRadicals::YiRadicalGot => "yi radical got",
            YiRadicals::YiRadicalJjie => "yi radical jjie",
            YiRadicals::YiRadicalWo => "yi radical wo",
            YiRadicals::YiRadicalDu => "yi radical du",
            YiRadicals::YiRadicalShur => "yi radical shur",
            YiRadicals::YiRadicalLie => "yi radical lie",
            YiRadicals::YiRadicalCy => "yi radical cy",
            YiRadicals::YiRadicalCuop => "yi radical cuop",
            YiRadicals::YiRadicalCip => "yi radical cip",
            YiRadicals::YiRadicalHxop => "yi radical hxop",
            YiRadicals::YiRadicalShat => "yi radical shat",
            YiRadicals::YiRadicalZur => "yi radical zur",
            YiRadicals::YiRadicalShop => "yi radical shop",
            YiRadicals::YiRadicalChe => "yi radical che",
            YiRadicals::YiRadicalZziet => "yi radical zziet",
            YiRadicals::YiRadicalNbie => "yi radical nbie",
            YiRadicals::YiRadicalKe => "yi radical ke",
        }
    }
}
