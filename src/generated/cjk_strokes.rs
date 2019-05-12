/// \u{31c0} → \u{31ef}\
///\
/// ㇀ ㇁ ㇂ ㇃ ㇄ ㇅ ㇆ ㇇ ㇈ ㇉ ㇊ ㇋ ㇌ ㇍ ㇎ ㇏
/// ㇐ ㇑ ㇒ ㇓ ㇔ ㇕ ㇖ ㇗ ㇘ ㇙ ㇚ ㇛ ㇜ ㇝ ㇞ ㇟
/// ㇠ ㇡ ㇢ ㇣
pub mod constants {
    /// \u{31c0}: '㇀'
    pub const CJK_STROKE_T: char = '㇀';
    /// \u{31c1}: '㇁'
    pub const CJK_STROKE_WG: char = '㇁';
    /// \u{31c2}: '㇂'
    pub const CJK_STROKE_XG: char = '㇂';
    /// \u{31c3}: '㇃'
    pub const CJK_STROKE_BXG: char = '㇃';
    /// \u{31c4}: '㇄'
    pub const CJK_STROKE_SW: char = '㇄';
    /// \u{31c5}: '㇅'
    pub const CJK_STROKE_HZZ: char = '㇅';
    /// \u{31c6}: '㇆'
    pub const CJK_STROKE_HZG: char = '㇆';
    /// \u{31c7}: '㇇'
    pub const CJK_STROKE_HP: char = '㇇';
    /// \u{31c8}: '㇈'
    pub const CJK_STROKE_HZWG: char = '㇈';
    /// \u{31c9}: '㇉'
    pub const CJK_STROKE_SZWG: char = '㇉';
    /// \u{31ca}: '㇊'
    pub const CJK_STROKE_HZT: char = '㇊';
    /// \u{31cb}: '㇋'
    pub const CJK_STROKE_HZZP: char = '㇋';
    /// \u{31cc}: '㇌'
    pub const CJK_STROKE_HPWG: char = '㇌';
    /// \u{31cd}: '㇍'
    pub const CJK_STROKE_HZW: char = '㇍';
    /// \u{31ce}: '㇎'
    pub const CJK_STROKE_HZZZ: char = '㇎';
    /// \u{31cf}: '㇏'
    pub const CJK_STROKE_N: char = '㇏';
    /// \u{31d0}: '㇐'
    pub const CJK_STROKE_H: char = '㇐';
    /// \u{31d1}: '㇑'
    pub const CJK_STROKE_S: char = '㇑';
    /// \u{31d2}: '㇒'
    pub const CJK_STROKE_P: char = '㇒';
    /// \u{31d3}: '㇓'
    pub const CJK_STROKE_SP: char = '㇓';
    /// \u{31d4}: '㇔'
    pub const CJK_STROKE_D: char = '㇔';
    /// \u{31d5}: '㇕'
    pub const CJK_STROKE_HZ: char = '㇕';
    /// \u{31d6}: '㇖'
    pub const CJK_STROKE_HG: char = '㇖';
    /// \u{31d7}: '㇗'
    pub const CJK_STROKE_SZ: char = '㇗';
    /// \u{31d8}: '㇘'
    pub const CJK_STROKE_SWZ: char = '㇘';
    /// \u{31d9}: '㇙'
    pub const CJK_STROKE_ST: char = '㇙';
    /// \u{31da}: '㇚'
    pub const CJK_STROKE_SG: char = '㇚';
    /// \u{31db}: '㇛'
    pub const CJK_STROKE_PD: char = '㇛';
    /// \u{31dc}: '㇜'
    pub const CJK_STROKE_PZ: char = '㇜';
    /// \u{31dd}: '㇝'
    pub const CJK_STROKE_TN: char = '㇝';
    /// \u{31de}: '㇞'
    pub const CJK_STROKE_SZZ: char = '㇞';
    /// \u{31df}: '㇟'
    pub const CJK_STROKE_SWG: char = '㇟';
    /// \u{31e0}: '㇠'
    pub const CJK_STROKE_HXWG: char = '㇠';
    /// \u{31e1}: '㇡'
    pub const CJK_STROKE_HZZZG: char = '㇡';
    /// \u{31e2}: '㇢'
    pub const CJK_STROKE_PG: char = '㇢';
    /// \u{31e3}: '㇣'
    pub const CJK_STROKE_Q: char = '㇣';
}

/// \u{31c0} → \u{31ef}\
///\
/// ㇀ ㇁ ㇂ ㇃ ㇄ ㇅ ㇆ ㇇ ㇈ ㇉ ㇊ ㇋ ㇌ ㇍ ㇎ ㇏
/// ㇐ ㇑ ㇒ ㇓ ㇔ ㇕ ㇖ ㇗ ㇘ ㇙ ㇚ ㇛ ㇜ ㇝ ㇞ ㇟
/// ㇠ ㇡ ㇢ ㇣
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum CJKStrokes {
    /// \u{31c0}: '㇀'
    CjkStrokeT,
    /// \u{31c1}: '㇁'
    CjkStrokeWg,
    /// \u{31c2}: '㇂'
    CjkStrokeXg,
    /// \u{31c3}: '㇃'
    CjkStrokeBxg,
    /// \u{31c4}: '㇄'
    CjkStrokeSw,
    /// \u{31c5}: '㇅'
    CjkStrokeHzz,
    /// \u{31c6}: '㇆'
    CjkStrokeHzg,
    /// \u{31c7}: '㇇'
    CjkStrokeHp,
    /// \u{31c8}: '㇈'
    CjkStrokeHzwg,
    /// \u{31c9}: '㇉'
    CjkStrokeSzwg,
    /// \u{31ca}: '㇊'
    CjkStrokeHzt,
    /// \u{31cb}: '㇋'
    CjkStrokeHzzp,
    /// \u{31cc}: '㇌'
    CjkStrokeHpwg,
    /// \u{31cd}: '㇍'
    CjkStrokeHzw,
    /// \u{31ce}: '㇎'
    CjkStrokeHzzz,
    /// \u{31cf}: '㇏'
    CjkStrokeN,
    /// \u{31d0}: '㇐'
    CjkStrokeH,
    /// \u{31d1}: '㇑'
    CjkStrokeS,
    /// \u{31d2}: '㇒'
    CjkStrokeP,
    /// \u{31d3}: '㇓'
    CjkStrokeSp,
    /// \u{31d4}: '㇔'
    CjkStrokeD,
    /// \u{31d5}: '㇕'
    CjkStrokeHz,
    /// \u{31d6}: '㇖'
    CjkStrokeHg,
    /// \u{31d7}: '㇗'
    CjkStrokeSz,
    /// \u{31d8}: '㇘'
    CjkStrokeSwz,
    /// \u{31d9}: '㇙'
    CjkStrokeSt,
    /// \u{31da}: '㇚'
    CjkStrokeSg,
    /// \u{31db}: '㇛'
    CjkStrokePd,
    /// \u{31dc}: '㇜'
    CjkStrokePz,
    /// \u{31dd}: '㇝'
    CjkStrokeTn,
    /// \u{31de}: '㇞'
    CjkStrokeSzz,
    /// \u{31df}: '㇟'
    CjkStrokeSwg,
    /// \u{31e0}: '㇠'
    CjkStrokeHxwg,
    /// \u{31e1}: '㇡'
    CjkStrokeHzzzg,
    /// \u{31e2}: '㇢'
    CjkStrokePg,
    /// \u{31e3}: '㇣'
    CjkStrokeQ,
}

impl Into<char> for CJKStrokes {
    fn into(self) -> char {
        use constants::*;
        match self {
            CJKStrokes::CjkStrokeT => CJK_STROKE_T,
            CJKStrokes::CjkStrokeWg => CJK_STROKE_WG,
            CJKStrokes::CjkStrokeXg => CJK_STROKE_XG,
            CJKStrokes::CjkStrokeBxg => CJK_STROKE_BXG,
            CJKStrokes::CjkStrokeSw => CJK_STROKE_SW,
            CJKStrokes::CjkStrokeHzz => CJK_STROKE_HZZ,
            CJKStrokes::CjkStrokeHzg => CJK_STROKE_HZG,
            CJKStrokes::CjkStrokeHp => CJK_STROKE_HP,
            CJKStrokes::CjkStrokeHzwg => CJK_STROKE_HZWG,
            CJKStrokes::CjkStrokeSzwg => CJK_STROKE_SZWG,
            CJKStrokes::CjkStrokeHzt => CJK_STROKE_HZT,
            CJKStrokes::CjkStrokeHzzp => CJK_STROKE_HZZP,
            CJKStrokes::CjkStrokeHpwg => CJK_STROKE_HPWG,
            CJKStrokes::CjkStrokeHzw => CJK_STROKE_HZW,
            CJKStrokes::CjkStrokeHzzz => CJK_STROKE_HZZZ,
            CJKStrokes::CjkStrokeN => CJK_STROKE_N,
            CJKStrokes::CjkStrokeH => CJK_STROKE_H,
            CJKStrokes::CjkStrokeS => CJK_STROKE_S,
            CJKStrokes::CjkStrokeP => CJK_STROKE_P,
            CJKStrokes::CjkStrokeSp => CJK_STROKE_SP,
            CJKStrokes::CjkStrokeD => CJK_STROKE_D,
            CJKStrokes::CjkStrokeHz => CJK_STROKE_HZ,
            CJKStrokes::CjkStrokeHg => CJK_STROKE_HG,
            CJKStrokes::CjkStrokeSz => CJK_STROKE_SZ,
            CJKStrokes::CjkStrokeSwz => CJK_STROKE_SWZ,
            CJKStrokes::CjkStrokeSt => CJK_STROKE_ST,
            CJKStrokes::CjkStrokeSg => CJK_STROKE_SG,
            CJKStrokes::CjkStrokePd => CJK_STROKE_PD,
            CJKStrokes::CjkStrokePz => CJK_STROKE_PZ,
            CJKStrokes::CjkStrokeTn => CJK_STROKE_TN,
            CJKStrokes::CjkStrokeSzz => CJK_STROKE_SZZ,
            CJKStrokes::CjkStrokeSwg => CJK_STROKE_SWG,
            CJKStrokes::CjkStrokeHxwg => CJK_STROKE_HXWG,
            CJKStrokes::CjkStrokeHzzzg => CJK_STROKE_HZZZG,
            CJKStrokes::CjkStrokePg => CJK_STROKE_PG,
            CJKStrokes::CjkStrokeQ => CJK_STROKE_Q,
        }
    }
}

impl std::convert::TryFrom<char> for CJKStrokes {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            CJK_STROKE_T => Ok(CJKStrokes::CjkStrokeT),
            CJK_STROKE_WG => Ok(CJKStrokes::CjkStrokeWg),
            CJK_STROKE_XG => Ok(CJKStrokes::CjkStrokeXg),
            CJK_STROKE_BXG => Ok(CJKStrokes::CjkStrokeBxg),
            CJK_STROKE_SW => Ok(CJKStrokes::CjkStrokeSw),
            CJK_STROKE_HZZ => Ok(CJKStrokes::CjkStrokeHzz),
            CJK_STROKE_HZG => Ok(CJKStrokes::CjkStrokeHzg),
            CJK_STROKE_HP => Ok(CJKStrokes::CjkStrokeHp),
            CJK_STROKE_HZWG => Ok(CJKStrokes::CjkStrokeHzwg),
            CJK_STROKE_SZWG => Ok(CJKStrokes::CjkStrokeSzwg),
            CJK_STROKE_HZT => Ok(CJKStrokes::CjkStrokeHzt),
            CJK_STROKE_HZZP => Ok(CJKStrokes::CjkStrokeHzzp),
            CJK_STROKE_HPWG => Ok(CJKStrokes::CjkStrokeHpwg),
            CJK_STROKE_HZW => Ok(CJKStrokes::CjkStrokeHzw),
            CJK_STROKE_HZZZ => Ok(CJKStrokes::CjkStrokeHzzz),
            CJK_STROKE_N => Ok(CJKStrokes::CjkStrokeN),
            CJK_STROKE_H => Ok(CJKStrokes::CjkStrokeH),
            CJK_STROKE_S => Ok(CJKStrokes::CjkStrokeS),
            CJK_STROKE_P => Ok(CJKStrokes::CjkStrokeP),
            CJK_STROKE_SP => Ok(CJKStrokes::CjkStrokeSp),
            CJK_STROKE_D => Ok(CJKStrokes::CjkStrokeD),
            CJK_STROKE_HZ => Ok(CJKStrokes::CjkStrokeHz),
            CJK_STROKE_HG => Ok(CJKStrokes::CjkStrokeHg),
            CJK_STROKE_SZ => Ok(CJKStrokes::CjkStrokeSz),
            CJK_STROKE_SWZ => Ok(CJKStrokes::CjkStrokeSwz),
            CJK_STROKE_ST => Ok(CJKStrokes::CjkStrokeSt),
            CJK_STROKE_SG => Ok(CJKStrokes::CjkStrokeSg),
            CJK_STROKE_PD => Ok(CJKStrokes::CjkStrokePd),
            CJK_STROKE_PZ => Ok(CJKStrokes::CjkStrokePz),
            CJK_STROKE_TN => Ok(CJKStrokes::CjkStrokeTn),
            CJK_STROKE_SZZ => Ok(CJKStrokes::CjkStrokeSzz),
            CJK_STROKE_SWG => Ok(CJKStrokes::CjkStrokeSwg),
            CJK_STROKE_HXWG => Ok(CJKStrokes::CjkStrokeHxwg),
            CJK_STROKE_HZZZG => Ok(CJKStrokes::CjkStrokeHzzzg),
            CJK_STROKE_PG => Ok(CJKStrokes::CjkStrokePg),
            CJK_STROKE_Q => Ok(CJKStrokes::CjkStrokeQ),
            _ => Err(()),
        }
    }
}

impl Into<u32> for CJKStrokes {
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

impl std::convert::TryFrom<u32> for CJKStrokes {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for CJKStrokes {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl CJKStrokes {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        CJKStrokes::CjkStrokeT
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            CJKStrokes::CjkStrokeT => "cjk stroke t",
            CJKStrokes::CjkStrokeWg => "cjk stroke wg",
            CJKStrokes::CjkStrokeXg => "cjk stroke xg",
            CJKStrokes::CjkStrokeBxg => "cjk stroke bxg",
            CJKStrokes::CjkStrokeSw => "cjk stroke sw",
            CJKStrokes::CjkStrokeHzz => "cjk stroke hzz",
            CJKStrokes::CjkStrokeHzg => "cjk stroke hzg",
            CJKStrokes::CjkStrokeHp => "cjk stroke hp",
            CJKStrokes::CjkStrokeHzwg => "cjk stroke hzwg",
            CJKStrokes::CjkStrokeSzwg => "cjk stroke szwg",
            CJKStrokes::CjkStrokeHzt => "cjk stroke hzt",
            CJKStrokes::CjkStrokeHzzp => "cjk stroke hzzp",
            CJKStrokes::CjkStrokeHpwg => "cjk stroke hpwg",
            CJKStrokes::CjkStrokeHzw => "cjk stroke hzw",
            CJKStrokes::CjkStrokeHzzz => "cjk stroke hzzz",
            CJKStrokes::CjkStrokeN => "cjk stroke n",
            CJKStrokes::CjkStrokeH => "cjk stroke h",
            CJKStrokes::CjkStrokeS => "cjk stroke s",
            CJKStrokes::CjkStrokeP => "cjk stroke p",
            CJKStrokes::CjkStrokeSp => "cjk stroke sp",
            CJKStrokes::CjkStrokeD => "cjk stroke d",
            CJKStrokes::CjkStrokeHz => "cjk stroke hz",
            CJKStrokes::CjkStrokeHg => "cjk stroke hg",
            CJKStrokes::CjkStrokeSz => "cjk stroke sz",
            CJKStrokes::CjkStrokeSwz => "cjk stroke swz",
            CJKStrokes::CjkStrokeSt => "cjk stroke st",
            CJKStrokes::CjkStrokeSg => "cjk stroke sg",
            CJKStrokes::CjkStrokePd => "cjk stroke pd",
            CJKStrokes::CjkStrokePz => "cjk stroke pz",
            CJKStrokes::CjkStrokeTn => "cjk stroke tn",
            CJKStrokes::CjkStrokeSzz => "cjk stroke szz",
            CJKStrokes::CjkStrokeSwg => "cjk stroke swg",
            CJKStrokes::CjkStrokeHxwg => "cjk stroke hxwg",
            CJKStrokes::CjkStrokeHzzzg => "cjk stroke hzzzg",
            CJKStrokes::CjkStrokePg => "cjk stroke pg",
            CJKStrokes::CjkStrokeQ => "cjk stroke q",
        }
    }
}
