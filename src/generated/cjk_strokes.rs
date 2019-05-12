
/// An enum to represent all characters in the CJKStrokes block.
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
        match self {
            CJKStrokes::CjkStrokeT => '㇀',
            CJKStrokes::CjkStrokeWg => '㇁',
            CJKStrokes::CjkStrokeXg => '㇂',
            CJKStrokes::CjkStrokeBxg => '㇃',
            CJKStrokes::CjkStrokeSw => '㇄',
            CJKStrokes::CjkStrokeHzz => '㇅',
            CJKStrokes::CjkStrokeHzg => '㇆',
            CJKStrokes::CjkStrokeHp => '㇇',
            CJKStrokes::CjkStrokeHzwg => '㇈',
            CJKStrokes::CjkStrokeSzwg => '㇉',
            CJKStrokes::CjkStrokeHzt => '㇊',
            CJKStrokes::CjkStrokeHzzp => '㇋',
            CJKStrokes::CjkStrokeHpwg => '㇌',
            CJKStrokes::CjkStrokeHzw => '㇍',
            CJKStrokes::CjkStrokeHzzz => '㇎',
            CJKStrokes::CjkStrokeN => '㇏',
            CJKStrokes::CjkStrokeH => '㇐',
            CJKStrokes::CjkStrokeS => '㇑',
            CJKStrokes::CjkStrokeP => '㇒',
            CJKStrokes::CjkStrokeSp => '㇓',
            CJKStrokes::CjkStrokeD => '㇔',
            CJKStrokes::CjkStrokeHz => '㇕',
            CJKStrokes::CjkStrokeHg => '㇖',
            CJKStrokes::CjkStrokeSz => '㇗',
            CJKStrokes::CjkStrokeSwz => '㇘',
            CJKStrokes::CjkStrokeSt => '㇙',
            CJKStrokes::CjkStrokeSg => '㇚',
            CJKStrokes::CjkStrokePd => '㇛',
            CJKStrokes::CjkStrokePz => '㇜',
            CJKStrokes::CjkStrokeTn => '㇝',
            CJKStrokes::CjkStrokeSzz => '㇞',
            CJKStrokes::CjkStrokeSwg => '㇟',
            CJKStrokes::CjkStrokeHxwg => '㇠',
            CJKStrokes::CjkStrokeHzzzg => '㇡',
            CJKStrokes::CjkStrokePg => '㇢',
            CJKStrokes::CjkStrokeQ => '㇣',
        }
    }
}

impl std::convert::TryFrom<char> for CJKStrokes {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '㇀' => Ok(CJKStrokes::CjkStrokeT),
            '㇁' => Ok(CJKStrokes::CjkStrokeWg),
            '㇂' => Ok(CJKStrokes::CjkStrokeXg),
            '㇃' => Ok(CJKStrokes::CjkStrokeBxg),
            '㇄' => Ok(CJKStrokes::CjkStrokeSw),
            '㇅' => Ok(CJKStrokes::CjkStrokeHzz),
            '㇆' => Ok(CJKStrokes::CjkStrokeHzg),
            '㇇' => Ok(CJKStrokes::CjkStrokeHp),
            '㇈' => Ok(CJKStrokes::CjkStrokeHzwg),
            '㇉' => Ok(CJKStrokes::CjkStrokeSzwg),
            '㇊' => Ok(CJKStrokes::CjkStrokeHzt),
            '㇋' => Ok(CJKStrokes::CjkStrokeHzzp),
            '㇌' => Ok(CJKStrokes::CjkStrokeHpwg),
            '㇍' => Ok(CJKStrokes::CjkStrokeHzw),
            '㇎' => Ok(CJKStrokes::CjkStrokeHzzz),
            '㇏' => Ok(CJKStrokes::CjkStrokeN),
            '㇐' => Ok(CJKStrokes::CjkStrokeH),
            '㇑' => Ok(CJKStrokes::CjkStrokeS),
            '㇒' => Ok(CJKStrokes::CjkStrokeP),
            '㇓' => Ok(CJKStrokes::CjkStrokeSp),
            '㇔' => Ok(CJKStrokes::CjkStrokeD),
            '㇕' => Ok(CJKStrokes::CjkStrokeHz),
            '㇖' => Ok(CJKStrokes::CjkStrokeHg),
            '㇗' => Ok(CJKStrokes::CjkStrokeSz),
            '㇘' => Ok(CJKStrokes::CjkStrokeSwz),
            '㇙' => Ok(CJKStrokes::CjkStrokeSt),
            '㇚' => Ok(CJKStrokes::CjkStrokeSg),
            '㇛' => Ok(CJKStrokes::CjkStrokePd),
            '㇜' => Ok(CJKStrokes::CjkStrokePz),
            '㇝' => Ok(CJKStrokes::CjkStrokeTn),
            '㇞' => Ok(CJKStrokes::CjkStrokeSzz),
            '㇟' => Ok(CJKStrokes::CjkStrokeSwg),
            '㇠' => Ok(CJKStrokes::CjkStrokeHxwg),
            '㇡' => Ok(CJKStrokes::CjkStrokeHzzzg),
            '㇢' => Ok(CJKStrokes::CjkStrokePg),
            '㇣' => Ok(CJKStrokes::CjkStrokeQ),
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

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("CJKStrokes{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
