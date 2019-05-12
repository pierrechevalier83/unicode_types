/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{a490}: '꒐'
    pub const YI_RADICAL_QOT: char = '꒐';
    /// \u{a491}: '꒑'
    pub const YI_RADICAL_LI: char = '꒑';
    /// \u{a492}: '꒒'
    pub const YI_RADICAL_KIT: char = '꒒';
    /// \u{a493}: '꒓'
    pub const YI_RADICAL_NYIP: char = '꒓';
    /// \u{a494}: '꒔'
    pub const YI_RADICAL_CYP: char = '꒔';
    /// \u{a495}: '꒕'
    pub const YI_RADICAL_SSI: char = '꒕';
    /// \u{a496}: '꒖'
    pub const YI_RADICAL_GGOP: char = '꒖';
    /// \u{a497}: '꒗'
    pub const YI_RADICAL_GEP: char = '꒗';
    /// \u{a498}: '꒘'
    pub const YI_RADICAL_MI: char = '꒘';
    /// \u{a499}: '꒙'
    pub const YI_RADICAL_HXIT: char = '꒙';
    /// \u{a49a}: '꒚'
    pub const YI_RADICAL_LYR: char = '꒚';
    /// \u{a49b}: '꒛'
    pub const YI_RADICAL_BBUT: char = '꒛';
    /// \u{a49c}: '꒜'
    pub const YI_RADICAL_MOP: char = '꒜';
    /// \u{a49d}: '꒝'
    pub const YI_RADICAL_YO: char = '꒝';
    /// \u{a49e}: '꒞'
    pub const YI_RADICAL_PUT: char = '꒞';
    /// \u{a49f}: '꒟'
    pub const YI_RADICAL_HXUO: char = '꒟';
    /// \u{a4a0}: '꒠'
    pub const YI_RADICAL_TAT: char = '꒠';
    /// \u{a4a1}: '꒡'
    pub const YI_RADICAL_GA: char = '꒡';
    /// \u{a4a2}: '꒢'
    pub const YI_RADICAL_ZUP: char = '꒢';
    /// \u{a4a3}: '꒣'
    pub const YI_RADICAL_CYT: char = '꒣';
    /// \u{a4a4}: '꒤'
    pub const YI_RADICAL_DDUR: char = '꒤';
    /// \u{a4a5}: '꒥'
    pub const YI_RADICAL_BUR: char = '꒥';
    /// \u{a4a6}: '꒦'
    pub const YI_RADICAL_GGUO: char = '꒦';
    /// \u{a4a7}: '꒧'
    pub const YI_RADICAL_NYOP: char = '꒧';
    /// \u{a4a8}: '꒨'
    pub const YI_RADICAL_TU: char = '꒨';
    /// \u{a4a9}: '꒩'
    pub const YI_RADICAL_OP: char = '꒩';
    /// \u{a4aa}: '꒪'
    pub const YI_RADICAL_JJUT: char = '꒪';
    /// \u{a4ab}: '꒫'
    pub const YI_RADICAL_ZOT: char = '꒫';
    /// \u{a4ac}: '꒬'
    pub const YI_RADICAL_PYT: char = '꒬';
    /// \u{a4ad}: '꒭'
    pub const YI_RADICAL_HMO: char = '꒭';
    /// \u{a4ae}: '꒮'
    pub const YI_RADICAL_YIT: char = '꒮';
    /// \u{a4af}: '꒯'
    pub const YI_RADICAL_VUR: char = '꒯';
    /// \u{a4b0}: '꒰'
    pub const YI_RADICAL_SHY: char = '꒰';
    /// \u{a4b1}: '꒱'
    pub const YI_RADICAL_VEP: char = '꒱';
    /// \u{a4b2}: '꒲'
    pub const YI_RADICAL_ZA: char = '꒲';
    /// \u{a4b3}: '꒳'
    pub const YI_RADICAL_JO: char = '꒳';
    /// \u{a4b4}: '꒴'
    pub const YI_RADICAL_NZUP: char = '꒴';
    /// \u{a4b5}: '꒵'
    pub const YI_RADICAL_JJY: char = '꒵';
    /// \u{a4b6}: '꒶'
    pub const YI_RADICAL_GOT: char = '꒶';
    /// \u{a4b7}: '꒷'
    pub const YI_RADICAL_JJIE: char = '꒷';
    /// \u{a4b8}: '꒸'
    pub const YI_RADICAL_WO: char = '꒸';
    /// \u{a4b9}: '꒹'
    pub const YI_RADICAL_DU: char = '꒹';
    /// \u{a4ba}: '꒺'
    pub const YI_RADICAL_SHUR: char = '꒺';
    /// \u{a4bb}: '꒻'
    pub const YI_RADICAL_LIE: char = '꒻';
    /// \u{a4bc}: '꒼'
    pub const YI_RADICAL_CY: char = '꒼';
    /// \u{a4bd}: '꒽'
    pub const YI_RADICAL_CUOP: char = '꒽';
    /// \u{a4be}: '꒾'
    pub const YI_RADICAL_CIP: char = '꒾';
    /// \u{a4bf}: '꒿'
    pub const YI_RADICAL_HXOP: char = '꒿';
    /// \u{a4c0}: '꓀'
    pub const YI_RADICAL_SHAT: char = '꓀';
    /// \u{a4c1}: '꓁'
    pub const YI_RADICAL_ZUR: char = '꓁';
    /// \u{a4c2}: '꓂'
    pub const YI_RADICAL_SHOP: char = '꓂';
    /// \u{a4c3}: '꓃'
    pub const YI_RADICAL_CHE: char = '꓃';
    /// \u{a4c4}: '꓄'
    pub const YI_RADICAL_ZZIET: char = '꓄';
    /// \u{a4c5}: '꓅'
    pub const YI_RADICAL_NBIE: char = '꓅';
    /// \u{a4c6}: '꓆'
    pub const YI_RADICAL_KE: char = '꓆';
}

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
        use constants::*;
        match self {
            YiRadicals::YiRadicalQot => YI_RADICAL_QOT,
            YiRadicals::YiRadicalLi => YI_RADICAL_LI,
            YiRadicals::YiRadicalKit => YI_RADICAL_KIT,
            YiRadicals::YiRadicalNyip => YI_RADICAL_NYIP,
            YiRadicals::YiRadicalCyp => YI_RADICAL_CYP,
            YiRadicals::YiRadicalSsi => YI_RADICAL_SSI,
            YiRadicals::YiRadicalGgop => YI_RADICAL_GGOP,
            YiRadicals::YiRadicalGep => YI_RADICAL_GEP,
            YiRadicals::YiRadicalMi => YI_RADICAL_MI,
            YiRadicals::YiRadicalHxit => YI_RADICAL_HXIT,
            YiRadicals::YiRadicalLyr => YI_RADICAL_LYR,
            YiRadicals::YiRadicalBbut => YI_RADICAL_BBUT,
            YiRadicals::YiRadicalMop => YI_RADICAL_MOP,
            YiRadicals::YiRadicalYo => YI_RADICAL_YO,
            YiRadicals::YiRadicalPut => YI_RADICAL_PUT,
            YiRadicals::YiRadicalHxuo => YI_RADICAL_HXUO,
            YiRadicals::YiRadicalTat => YI_RADICAL_TAT,
            YiRadicals::YiRadicalGa => YI_RADICAL_GA,
            YiRadicals::YiRadicalZup => YI_RADICAL_ZUP,
            YiRadicals::YiRadicalCyt => YI_RADICAL_CYT,
            YiRadicals::YiRadicalDdur => YI_RADICAL_DDUR,
            YiRadicals::YiRadicalBur => YI_RADICAL_BUR,
            YiRadicals::YiRadicalGguo => YI_RADICAL_GGUO,
            YiRadicals::YiRadicalNyop => YI_RADICAL_NYOP,
            YiRadicals::YiRadicalTu => YI_RADICAL_TU,
            YiRadicals::YiRadicalOp => YI_RADICAL_OP,
            YiRadicals::YiRadicalJjut => YI_RADICAL_JJUT,
            YiRadicals::YiRadicalZot => YI_RADICAL_ZOT,
            YiRadicals::YiRadicalPyt => YI_RADICAL_PYT,
            YiRadicals::YiRadicalHmo => YI_RADICAL_HMO,
            YiRadicals::YiRadicalYit => YI_RADICAL_YIT,
            YiRadicals::YiRadicalVur => YI_RADICAL_VUR,
            YiRadicals::YiRadicalShy => YI_RADICAL_SHY,
            YiRadicals::YiRadicalVep => YI_RADICAL_VEP,
            YiRadicals::YiRadicalZa => YI_RADICAL_ZA,
            YiRadicals::YiRadicalJo => YI_RADICAL_JO,
            YiRadicals::YiRadicalNzup => YI_RADICAL_NZUP,
            YiRadicals::YiRadicalJjy => YI_RADICAL_JJY,
            YiRadicals::YiRadicalGot => YI_RADICAL_GOT,
            YiRadicals::YiRadicalJjie => YI_RADICAL_JJIE,
            YiRadicals::YiRadicalWo => YI_RADICAL_WO,
            YiRadicals::YiRadicalDu => YI_RADICAL_DU,
            YiRadicals::YiRadicalShur => YI_RADICAL_SHUR,
            YiRadicals::YiRadicalLie => YI_RADICAL_LIE,
            YiRadicals::YiRadicalCy => YI_RADICAL_CY,
            YiRadicals::YiRadicalCuop => YI_RADICAL_CUOP,
            YiRadicals::YiRadicalCip => YI_RADICAL_CIP,
            YiRadicals::YiRadicalHxop => YI_RADICAL_HXOP,
            YiRadicals::YiRadicalShat => YI_RADICAL_SHAT,
            YiRadicals::YiRadicalZur => YI_RADICAL_ZUR,
            YiRadicals::YiRadicalShop => YI_RADICAL_SHOP,
            YiRadicals::YiRadicalChe => YI_RADICAL_CHE,
            YiRadicals::YiRadicalZziet => YI_RADICAL_ZZIET,
            YiRadicals::YiRadicalNbie => YI_RADICAL_NBIE,
            YiRadicals::YiRadicalKe => YI_RADICAL_KE,
        }
    }
}

impl std::convert::TryFrom<char> for YiRadicals {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            YI_RADICAL_QOT => Ok(YiRadicals::YiRadicalQot),
            YI_RADICAL_LI => Ok(YiRadicals::YiRadicalLi),
            YI_RADICAL_KIT => Ok(YiRadicals::YiRadicalKit),
            YI_RADICAL_NYIP => Ok(YiRadicals::YiRadicalNyip),
            YI_RADICAL_CYP => Ok(YiRadicals::YiRadicalCyp),
            YI_RADICAL_SSI => Ok(YiRadicals::YiRadicalSsi),
            YI_RADICAL_GGOP => Ok(YiRadicals::YiRadicalGgop),
            YI_RADICAL_GEP => Ok(YiRadicals::YiRadicalGep),
            YI_RADICAL_MI => Ok(YiRadicals::YiRadicalMi),
            YI_RADICAL_HXIT => Ok(YiRadicals::YiRadicalHxit),
            YI_RADICAL_LYR => Ok(YiRadicals::YiRadicalLyr),
            YI_RADICAL_BBUT => Ok(YiRadicals::YiRadicalBbut),
            YI_RADICAL_MOP => Ok(YiRadicals::YiRadicalMop),
            YI_RADICAL_YO => Ok(YiRadicals::YiRadicalYo),
            YI_RADICAL_PUT => Ok(YiRadicals::YiRadicalPut),
            YI_RADICAL_HXUO => Ok(YiRadicals::YiRadicalHxuo),
            YI_RADICAL_TAT => Ok(YiRadicals::YiRadicalTat),
            YI_RADICAL_GA => Ok(YiRadicals::YiRadicalGa),
            YI_RADICAL_ZUP => Ok(YiRadicals::YiRadicalZup),
            YI_RADICAL_CYT => Ok(YiRadicals::YiRadicalCyt),
            YI_RADICAL_DDUR => Ok(YiRadicals::YiRadicalDdur),
            YI_RADICAL_BUR => Ok(YiRadicals::YiRadicalBur),
            YI_RADICAL_GGUO => Ok(YiRadicals::YiRadicalGguo),
            YI_RADICAL_NYOP => Ok(YiRadicals::YiRadicalNyop),
            YI_RADICAL_TU => Ok(YiRadicals::YiRadicalTu),
            YI_RADICAL_OP => Ok(YiRadicals::YiRadicalOp),
            YI_RADICAL_JJUT => Ok(YiRadicals::YiRadicalJjut),
            YI_RADICAL_ZOT => Ok(YiRadicals::YiRadicalZot),
            YI_RADICAL_PYT => Ok(YiRadicals::YiRadicalPyt),
            YI_RADICAL_HMO => Ok(YiRadicals::YiRadicalHmo),
            YI_RADICAL_YIT => Ok(YiRadicals::YiRadicalYit),
            YI_RADICAL_VUR => Ok(YiRadicals::YiRadicalVur),
            YI_RADICAL_SHY => Ok(YiRadicals::YiRadicalShy),
            YI_RADICAL_VEP => Ok(YiRadicals::YiRadicalVep),
            YI_RADICAL_ZA => Ok(YiRadicals::YiRadicalZa),
            YI_RADICAL_JO => Ok(YiRadicals::YiRadicalJo),
            YI_RADICAL_NZUP => Ok(YiRadicals::YiRadicalNzup),
            YI_RADICAL_JJY => Ok(YiRadicals::YiRadicalJjy),
            YI_RADICAL_GOT => Ok(YiRadicals::YiRadicalGot),
            YI_RADICAL_JJIE => Ok(YiRadicals::YiRadicalJjie),
            YI_RADICAL_WO => Ok(YiRadicals::YiRadicalWo),
            YI_RADICAL_DU => Ok(YiRadicals::YiRadicalDu),
            YI_RADICAL_SHUR => Ok(YiRadicals::YiRadicalShur),
            YI_RADICAL_LIE => Ok(YiRadicals::YiRadicalLie),
            YI_RADICAL_CY => Ok(YiRadicals::YiRadicalCy),
            YI_RADICAL_CUOP => Ok(YiRadicals::YiRadicalCuop),
            YI_RADICAL_CIP => Ok(YiRadicals::YiRadicalCip),
            YI_RADICAL_HXOP => Ok(YiRadicals::YiRadicalHxop),
            YI_RADICAL_SHAT => Ok(YiRadicals::YiRadicalShat),
            YI_RADICAL_ZUR => Ok(YiRadicals::YiRadicalZur),
            YI_RADICAL_SHOP => Ok(YiRadicals::YiRadicalShop),
            YI_RADICAL_CHE => Ok(YiRadicals::YiRadicalChe),
            YI_RADICAL_ZZIET => Ok(YiRadicals::YiRadicalZziet),
            YI_RADICAL_NBIE => Ok(YiRadicals::YiRadicalNbie),
            YI_RADICAL_KE => Ok(YiRadicals::YiRadicalKe),
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
