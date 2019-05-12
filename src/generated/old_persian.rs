/// \u{103a0} → \u{103df}\
///\
/// 𐎠 𐎡 𐎢 𐎣 𐎤 𐎥 𐎦 𐎧 𐎨 𐎩 𐎪 𐎫 𐎬 𐎭 𐎮 𐎯\
/// 𐎰 𐎱 𐎲 𐎳 𐎴 𐎵 𐎶 𐎷 𐎸 𐎹 𐎺 𐎻 𐎼 𐎽 𐎾 𐎿\
/// 𐏀 𐏁 𐏂 𐏃 𐏈 𐏉 𐏊 𐏋 𐏌 𐏍 𐏎 𐏏 𐏐 𐏑 𐏒 𐏓\
/// 𐏔 𐏕\

/// A number of constants to give a name to all characters in this block.
pub mod constants {
    /// \u{103a0}: '𐎠'
    pub const SIGN_A: char = '𐎠';
    /// \u{103a1}: '𐎡'
    pub const SIGN_I: char = '𐎡';
    /// \u{103a2}: '𐎢'
    pub const SIGN_U: char = '𐎢';
    /// \u{103a3}: '𐎣'
    pub const SIGN_KA: char = '𐎣';
    /// \u{103a4}: '𐎤'
    pub const SIGN_KU: char = '𐎤';
    /// \u{103a5}: '𐎥'
    pub const SIGN_GA: char = '𐎥';
    /// \u{103a6}: '𐎦'
    pub const SIGN_GU: char = '𐎦';
    /// \u{103a7}: '𐎧'
    pub const SIGN_XA: char = '𐎧';
    /// \u{103a8}: '𐎨'
    pub const SIGN_CA: char = '𐎨';
    /// \u{103a9}: '𐎩'
    pub const SIGN_JA: char = '𐎩';
    /// \u{103aa}: '𐎪'
    pub const SIGN_JI: char = '𐎪';
    /// \u{103ab}: '𐎫'
    pub const SIGN_TA: char = '𐎫';
    /// \u{103ac}: '𐎬'
    pub const SIGN_TU: char = '𐎬';
    /// \u{103ad}: '𐎭'
    pub const SIGN_DA: char = '𐎭';
    /// \u{103ae}: '𐎮'
    pub const SIGN_DI: char = '𐎮';
    /// \u{103af}: '𐎯'
    pub const SIGN_DU: char = '𐎯';
    /// \u{103b0}: '𐎰'
    pub const SIGN_THA: char = '𐎰';
    /// \u{103b1}: '𐎱'
    pub const SIGN_PA: char = '𐎱';
    /// \u{103b2}: '𐎲'
    pub const SIGN_BA: char = '𐎲';
    /// \u{103b3}: '𐎳'
    pub const SIGN_FA: char = '𐎳';
    /// \u{103b4}: '𐎴'
    pub const SIGN_NA: char = '𐎴';
    /// \u{103b5}: '𐎵'
    pub const SIGN_NU: char = '𐎵';
    /// \u{103b6}: '𐎶'
    pub const SIGN_MA: char = '𐎶';
    /// \u{103b7}: '𐎷'
    pub const SIGN_MI: char = '𐎷';
    /// \u{103b8}: '𐎸'
    pub const SIGN_MU: char = '𐎸';
    /// \u{103b9}: '𐎹'
    pub const SIGN_YA: char = '𐎹';
    /// \u{103ba}: '𐎺'
    pub const SIGN_VA: char = '𐎺';
    /// \u{103bb}: '𐎻'
    pub const SIGN_VI: char = '𐎻';
    /// \u{103bc}: '𐎼'
    pub const SIGN_RA: char = '𐎼';
    /// \u{103bd}: '𐎽'
    pub const SIGN_RU: char = '𐎽';
    /// \u{103be}: '𐎾'
    pub const SIGN_LA: char = '𐎾';
    /// \u{103bf}: '𐎿'
    pub const SIGN_SA: char = '𐎿';
    /// \u{103c0}: '𐏀'
    pub const SIGN_ZA: char = '𐏀';
    /// \u{103c1}: '𐏁'
    pub const SIGN_SHA: char = '𐏁';
    /// \u{103c2}: '𐏂'
    pub const SIGN_SSA: char = '𐏂';
    /// \u{103c3}: '𐏃'
    pub const SIGN_HA: char = '𐏃';
    /// \u{103c8}: '𐏈'
    pub const SIGN_AURAMAZDAA: char = '𐏈';
    /// \u{103c9}: '𐏉'
    pub const SIGN_AURAMAZDAA_DASH_2: char = '𐏉';
    /// \u{103ca}: '𐏊'
    pub const SIGN_AURAMAZDAAHA: char = '𐏊';
    /// \u{103cb}: '𐏋'
    pub const SIGN_XSHAAYATHIYA: char = '𐏋';
    /// \u{103cc}: '𐏌'
    pub const SIGN_DAHYAAUSH: char = '𐏌';
    /// \u{103cd}: '𐏍'
    pub const SIGN_DAHYAAUSH_DASH_2: char = '𐏍';
    /// \u{103ce}: '𐏎'
    pub const SIGN_BAGA: char = '𐏎';
    /// \u{103cf}: '𐏏'
    pub const SIGN_BUUMISH: char = '𐏏';
    /// \u{103d0}: '𐏐'
    pub const WORD_DIVIDER: char = '𐏐';
    /// \u{103d1}: '𐏑'
    pub const NUMBER_ONE: char = '𐏑';
    /// \u{103d2}: '𐏒'
    pub const NUMBER_TWO: char = '𐏒';
    /// \u{103d3}: '𐏓'
    pub const NUMBER_TEN: char = '𐏓';
    /// \u{103d4}: '𐏔'
    pub const NUMBER_TWENTY: char = '𐏔';
    /// \u{103d5}: '𐏕'
    pub const NUMBER_HUNDRED: char = '𐏕';
}

/// An enum to represent all characters in the OldPersian block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum OldPersian {
    /// \u{103a0}: '𐎠'
    SignA,
    /// \u{103a1}: '𐎡'
    SignI,
    /// \u{103a2}: '𐎢'
    SignU,
    /// \u{103a3}: '𐎣'
    SignKa,
    /// \u{103a4}: '𐎤'
    SignKu,
    /// \u{103a5}: '𐎥'
    SignGa,
    /// \u{103a6}: '𐎦'
    SignGu,
    /// \u{103a7}: '𐎧'
    SignXa,
    /// \u{103a8}: '𐎨'
    SignCa,
    /// \u{103a9}: '𐎩'
    SignJa,
    /// \u{103aa}: '𐎪'
    SignJi,
    /// \u{103ab}: '𐎫'
    SignTa,
    /// \u{103ac}: '𐎬'
    SignTu,
    /// \u{103ad}: '𐎭'
    SignDa,
    /// \u{103ae}: '𐎮'
    SignDi,
    /// \u{103af}: '𐎯'
    SignDu,
    /// \u{103b0}: '𐎰'
    SignTha,
    /// \u{103b1}: '𐎱'
    SignPa,
    /// \u{103b2}: '𐎲'
    SignBa,
    /// \u{103b3}: '𐎳'
    SignFa,
    /// \u{103b4}: '𐎴'
    SignNa,
    /// \u{103b5}: '𐎵'
    SignNu,
    /// \u{103b6}: '𐎶'
    SignMa,
    /// \u{103b7}: '𐎷'
    SignMi,
    /// \u{103b8}: '𐎸'
    SignMu,
    /// \u{103b9}: '𐎹'
    SignYa,
    /// \u{103ba}: '𐎺'
    SignVa,
    /// \u{103bb}: '𐎻'
    SignVi,
    /// \u{103bc}: '𐎼'
    SignRa,
    /// \u{103bd}: '𐎽'
    SignRu,
    /// \u{103be}: '𐎾'
    SignLa,
    /// \u{103bf}: '𐎿'
    SignSa,
    /// \u{103c0}: '𐏀'
    SignZa,
    /// \u{103c1}: '𐏁'
    SignSha,
    /// \u{103c2}: '𐏂'
    SignSsa,
    /// \u{103c3}: '𐏃'
    SignHa,
    /// \u{103c8}: '𐏈'
    SignAuramazdaa,
    /// \u{103c9}: '𐏉'
    SignAuramazdaaDash2,
    /// \u{103ca}: '𐏊'
    SignAuramazdaaha,
    /// \u{103cb}: '𐏋'
    SignXshaayathiya,
    /// \u{103cc}: '𐏌'
    SignDahyaaush,
    /// \u{103cd}: '𐏍'
    SignDahyaaushDash2,
    /// \u{103ce}: '𐏎'
    SignBaga,
    /// \u{103cf}: '𐏏'
    SignBuumish,
    /// \u{103d0}: '𐏐'
    WordDivider,
    /// \u{103d1}: '𐏑'
    NumberOne,
    /// \u{103d2}: '𐏒'
    NumberTwo,
    /// \u{103d3}: '𐏓'
    NumberTen,
    /// \u{103d4}: '𐏔'
    NumberTwenty,
    /// \u{103d5}: '𐏕'
    NumberHundred,
}

impl Into<char> for OldPersian {
    fn into(self) -> char {
        use constants::*;
        match self {
            OldPersian::SignA => SIGN_A,
            OldPersian::SignI => SIGN_I,
            OldPersian::SignU => SIGN_U,
            OldPersian::SignKa => SIGN_KA,
            OldPersian::SignKu => SIGN_KU,
            OldPersian::SignGa => SIGN_GA,
            OldPersian::SignGu => SIGN_GU,
            OldPersian::SignXa => SIGN_XA,
            OldPersian::SignCa => SIGN_CA,
            OldPersian::SignJa => SIGN_JA,
            OldPersian::SignJi => SIGN_JI,
            OldPersian::SignTa => SIGN_TA,
            OldPersian::SignTu => SIGN_TU,
            OldPersian::SignDa => SIGN_DA,
            OldPersian::SignDi => SIGN_DI,
            OldPersian::SignDu => SIGN_DU,
            OldPersian::SignTha => SIGN_THA,
            OldPersian::SignPa => SIGN_PA,
            OldPersian::SignBa => SIGN_BA,
            OldPersian::SignFa => SIGN_FA,
            OldPersian::SignNa => SIGN_NA,
            OldPersian::SignNu => SIGN_NU,
            OldPersian::SignMa => SIGN_MA,
            OldPersian::SignMi => SIGN_MI,
            OldPersian::SignMu => SIGN_MU,
            OldPersian::SignYa => SIGN_YA,
            OldPersian::SignVa => SIGN_VA,
            OldPersian::SignVi => SIGN_VI,
            OldPersian::SignRa => SIGN_RA,
            OldPersian::SignRu => SIGN_RU,
            OldPersian::SignLa => SIGN_LA,
            OldPersian::SignSa => SIGN_SA,
            OldPersian::SignZa => SIGN_ZA,
            OldPersian::SignSha => SIGN_SHA,
            OldPersian::SignSsa => SIGN_SSA,
            OldPersian::SignHa => SIGN_HA,
            OldPersian::SignAuramazdaa => SIGN_AURAMAZDAA,
            OldPersian::SignAuramazdaaDash2 => SIGN_AURAMAZDAA_DASH_2,
            OldPersian::SignAuramazdaaha => SIGN_AURAMAZDAAHA,
            OldPersian::SignXshaayathiya => SIGN_XSHAAYATHIYA,
            OldPersian::SignDahyaaush => SIGN_DAHYAAUSH,
            OldPersian::SignDahyaaushDash2 => SIGN_DAHYAAUSH_DASH_2,
            OldPersian::SignBaga => SIGN_BAGA,
            OldPersian::SignBuumish => SIGN_BUUMISH,
            OldPersian::WordDivider => WORD_DIVIDER,
            OldPersian::NumberOne => NUMBER_ONE,
            OldPersian::NumberTwo => NUMBER_TWO,
            OldPersian::NumberTen => NUMBER_TEN,
            OldPersian::NumberTwenty => NUMBER_TWENTY,
            OldPersian::NumberHundred => NUMBER_HUNDRED,
        }
    }
}

impl std::convert::TryFrom<char> for OldPersian {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            SIGN_A => Ok(OldPersian::SignA),
            SIGN_I => Ok(OldPersian::SignI),
            SIGN_U => Ok(OldPersian::SignU),
            SIGN_KA => Ok(OldPersian::SignKa),
            SIGN_KU => Ok(OldPersian::SignKu),
            SIGN_GA => Ok(OldPersian::SignGa),
            SIGN_GU => Ok(OldPersian::SignGu),
            SIGN_XA => Ok(OldPersian::SignXa),
            SIGN_CA => Ok(OldPersian::SignCa),
            SIGN_JA => Ok(OldPersian::SignJa),
            SIGN_JI => Ok(OldPersian::SignJi),
            SIGN_TA => Ok(OldPersian::SignTa),
            SIGN_TU => Ok(OldPersian::SignTu),
            SIGN_DA => Ok(OldPersian::SignDa),
            SIGN_DI => Ok(OldPersian::SignDi),
            SIGN_DU => Ok(OldPersian::SignDu),
            SIGN_THA => Ok(OldPersian::SignTha),
            SIGN_PA => Ok(OldPersian::SignPa),
            SIGN_BA => Ok(OldPersian::SignBa),
            SIGN_FA => Ok(OldPersian::SignFa),
            SIGN_NA => Ok(OldPersian::SignNa),
            SIGN_NU => Ok(OldPersian::SignNu),
            SIGN_MA => Ok(OldPersian::SignMa),
            SIGN_MI => Ok(OldPersian::SignMi),
            SIGN_MU => Ok(OldPersian::SignMu),
            SIGN_YA => Ok(OldPersian::SignYa),
            SIGN_VA => Ok(OldPersian::SignVa),
            SIGN_VI => Ok(OldPersian::SignVi),
            SIGN_RA => Ok(OldPersian::SignRa),
            SIGN_RU => Ok(OldPersian::SignRu),
            SIGN_LA => Ok(OldPersian::SignLa),
            SIGN_SA => Ok(OldPersian::SignSa),
            SIGN_ZA => Ok(OldPersian::SignZa),
            SIGN_SHA => Ok(OldPersian::SignSha),
            SIGN_SSA => Ok(OldPersian::SignSsa),
            SIGN_HA => Ok(OldPersian::SignHa),
            SIGN_AURAMAZDAA => Ok(OldPersian::SignAuramazdaa),
            SIGN_AURAMAZDAA_DASH_2 => Ok(OldPersian::SignAuramazdaaDash2),
            SIGN_AURAMAZDAAHA => Ok(OldPersian::SignAuramazdaaha),
            SIGN_XSHAAYATHIYA => Ok(OldPersian::SignXshaayathiya),
            SIGN_DAHYAAUSH => Ok(OldPersian::SignDahyaaush),
            SIGN_DAHYAAUSH_DASH_2 => Ok(OldPersian::SignDahyaaushDash2),
            SIGN_BAGA => Ok(OldPersian::SignBaga),
            SIGN_BUUMISH => Ok(OldPersian::SignBuumish),
            WORD_DIVIDER => Ok(OldPersian::WordDivider),
            NUMBER_ONE => Ok(OldPersian::NumberOne),
            NUMBER_TWO => Ok(OldPersian::NumberTwo),
            NUMBER_TEN => Ok(OldPersian::NumberTen),
            NUMBER_TWENTY => Ok(OldPersian::NumberTwenty),
            NUMBER_HUNDRED => Ok(OldPersian::NumberHundred),
            _ => Err(()),
        }
    }
}

impl Into<u32> for OldPersian {
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

impl std::convert::TryFrom<u32> for OldPersian {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for OldPersian {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl OldPersian {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        OldPersian::SignA
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            OldPersian::SignA => "old persian sign a",
            OldPersian::SignI => "old persian sign i",
            OldPersian::SignU => "old persian sign u",
            OldPersian::SignKa => "old persian sign ka",
            OldPersian::SignKu => "old persian sign ku",
            OldPersian::SignGa => "old persian sign ga",
            OldPersian::SignGu => "old persian sign gu",
            OldPersian::SignXa => "old persian sign xa",
            OldPersian::SignCa => "old persian sign ca",
            OldPersian::SignJa => "old persian sign ja",
            OldPersian::SignJi => "old persian sign ji",
            OldPersian::SignTa => "old persian sign ta",
            OldPersian::SignTu => "old persian sign tu",
            OldPersian::SignDa => "old persian sign da",
            OldPersian::SignDi => "old persian sign di",
            OldPersian::SignDu => "old persian sign du",
            OldPersian::SignTha => "old persian sign tha",
            OldPersian::SignPa => "old persian sign pa",
            OldPersian::SignBa => "old persian sign ba",
            OldPersian::SignFa => "old persian sign fa",
            OldPersian::SignNa => "old persian sign na",
            OldPersian::SignNu => "old persian sign nu",
            OldPersian::SignMa => "old persian sign ma",
            OldPersian::SignMi => "old persian sign mi",
            OldPersian::SignMu => "old persian sign mu",
            OldPersian::SignYa => "old persian sign ya",
            OldPersian::SignVa => "old persian sign va",
            OldPersian::SignVi => "old persian sign vi",
            OldPersian::SignRa => "old persian sign ra",
            OldPersian::SignRu => "old persian sign ru",
            OldPersian::SignLa => "old persian sign la",
            OldPersian::SignSa => "old persian sign sa",
            OldPersian::SignZa => "old persian sign za",
            OldPersian::SignSha => "old persian sign sha",
            OldPersian::SignSsa => "old persian sign ssa",
            OldPersian::SignHa => "old persian sign ha",
            OldPersian::SignAuramazdaa => "old persian sign auramazdaa",
            OldPersian::SignAuramazdaaDash2 => "old persian sign auramazdaa-2",
            OldPersian::SignAuramazdaaha => "old persian sign auramazdaaha",
            OldPersian::SignXshaayathiya => "old persian sign xshaayathiya",
            OldPersian::SignDahyaaush => "old persian sign dahyaaush",
            OldPersian::SignDahyaaushDash2 => "old persian sign dahyaaush-2",
            OldPersian::SignBaga => "old persian sign baga",
            OldPersian::SignBuumish => "old persian sign buumish",
            OldPersian::WordDivider => "old persian word divider",
            OldPersian::NumberOne => "old persian number one",
            OldPersian::NumberTwo => "old persian number two",
            OldPersian::NumberTen => "old persian number ten",
            OldPersian::NumberTwenty => "old persian number twenty",
            OldPersian::NumberHundred => "old persian number hundred",
        }
    }
}
