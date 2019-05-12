
/// An enum to represent all characters in the OldHungarian block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum OldHungarian {
    /// \u{10c80}: '𐲀'
    CapitalLetterA,
    /// \u{10c81}: '𐲁'
    CapitalLetterAa,
    /// \u{10c82}: '𐲂'
    CapitalLetterEb,
    /// \u{10c83}: '𐲃'
    CapitalLetterAmb,
    /// \u{10c84}: '𐲄'
    CapitalLetterEc,
    /// \u{10c85}: '𐲅'
    CapitalLetterEnc,
    /// \u{10c86}: '𐲆'
    CapitalLetterEcs,
    /// \u{10c87}: '𐲇'
    CapitalLetterEd,
    /// \u{10c88}: '𐲈'
    CapitalLetterAnd,
    /// \u{10c89}: '𐲉'
    CapitalLetterE,
    /// \u{10c8a}: '𐲊'
    CapitalLetterCloseE,
    /// \u{10c8b}: '𐲋'
    CapitalLetterEe,
    /// \u{10c8c}: '𐲌'
    CapitalLetterEf,
    /// \u{10c8d}: '𐲍'
    CapitalLetterEg,
    /// \u{10c8e}: '𐲎'
    CapitalLetterEgy,
    /// \u{10c8f}: '𐲏'
    CapitalLetterEh,
    /// \u{10c90}: '𐲐'
    CapitalLetterI,
    /// \u{10c91}: '𐲑'
    CapitalLetterIi,
    /// \u{10c92}: '𐲒'
    CapitalLetterEj,
    /// \u{10c93}: '𐲓'
    CapitalLetterEk,
    /// \u{10c94}: '𐲔'
    CapitalLetterAk,
    /// \u{10c95}: '𐲕'
    CapitalLetterUnk,
    /// \u{10c96}: '𐲖'
    CapitalLetterEl,
    /// \u{10c97}: '𐲗'
    CapitalLetterEly,
    /// \u{10c98}: '𐲘'
    CapitalLetterEm,
    /// \u{10c99}: '𐲙'
    CapitalLetterEn,
    /// \u{10c9a}: '𐲚'
    CapitalLetterEny,
    /// \u{10c9b}: '𐲛'
    CapitalLetterO,
    /// \u{10c9c}: '𐲜'
    CapitalLetterOo,
    /// \u{10c9d}: '𐲝'
    CapitalLetterNikolsburgOe,
    /// \u{10c9e}: '𐲞'
    CapitalLetterRudimentaOe,
    /// \u{10c9f}: '𐲟'
    CapitalLetterOee,
    /// \u{10ca0}: '𐲠'
    CapitalLetterEp,
    /// \u{10ca1}: '𐲡'
    CapitalLetterEmp,
    /// \u{10ca2}: '𐲢'
    CapitalLetterEr,
    /// \u{10ca3}: '𐲣'
    CapitalLetterShortEr,
    /// \u{10ca4}: '𐲤'
    CapitalLetterEs,
    /// \u{10ca5}: '𐲥'
    CapitalLetterEsz,
    /// \u{10ca6}: '𐲦'
    CapitalLetterEt,
    /// \u{10ca7}: '𐲧'
    CapitalLetterEnt,
    /// \u{10ca8}: '𐲨'
    CapitalLetterEty,
    /// \u{10ca9}: '𐲩'
    CapitalLetterEch,
    /// \u{10caa}: '𐲪'
    CapitalLetterU,
    /// \u{10cab}: '𐲫'
    CapitalLetterUu,
    /// \u{10cac}: '𐲬'
    CapitalLetterNikolsburgUe,
    /// \u{10cad}: '𐲭'
    CapitalLetterRudimentaUe,
    /// \u{10cae}: '𐲮'
    CapitalLetterEv,
    /// \u{10caf}: '𐲯'
    CapitalLetterEz,
    /// \u{10cb0}: '𐲰'
    CapitalLetterEzs,
    /// \u{10cb1}: '𐲱'
    CapitalLetterEntDashShapedSign,
    /// \u{10cb2}: '𐲲'
    CapitalLetterUs,
    /// \u{10cc0}: '𐳀'
    SmallLetterA,
    /// \u{10cc1}: '𐳁'
    SmallLetterAa,
    /// \u{10cc2}: '𐳂'
    SmallLetterEb,
    /// \u{10cc3}: '𐳃'
    SmallLetterAmb,
    /// \u{10cc4}: '𐳄'
    SmallLetterEc,
    /// \u{10cc5}: '𐳅'
    SmallLetterEnc,
    /// \u{10cc6}: '𐳆'
    SmallLetterEcs,
    /// \u{10cc7}: '𐳇'
    SmallLetterEd,
    /// \u{10cc8}: '𐳈'
    SmallLetterAnd,
    /// \u{10cc9}: '𐳉'
    SmallLetterE,
    /// \u{10cca}: '𐳊'
    SmallLetterCloseE,
    /// \u{10ccb}: '𐳋'
    SmallLetterEe,
    /// \u{10ccc}: '𐳌'
    SmallLetterEf,
    /// \u{10ccd}: '𐳍'
    SmallLetterEg,
    /// \u{10cce}: '𐳎'
    SmallLetterEgy,
    /// \u{10ccf}: '𐳏'
    SmallLetterEh,
    /// \u{10cd0}: '𐳐'
    SmallLetterI,
    /// \u{10cd1}: '𐳑'
    SmallLetterIi,
    /// \u{10cd2}: '𐳒'
    SmallLetterEj,
    /// \u{10cd3}: '𐳓'
    SmallLetterEk,
    /// \u{10cd4}: '𐳔'
    SmallLetterAk,
    /// \u{10cd5}: '𐳕'
    SmallLetterUnk,
    /// \u{10cd6}: '𐳖'
    SmallLetterEl,
    /// \u{10cd7}: '𐳗'
    SmallLetterEly,
    /// \u{10cd8}: '𐳘'
    SmallLetterEm,
    /// \u{10cd9}: '𐳙'
    SmallLetterEn,
    /// \u{10cda}: '𐳚'
    SmallLetterEny,
    /// \u{10cdb}: '𐳛'
    SmallLetterO,
    /// \u{10cdc}: '𐳜'
    SmallLetterOo,
    /// \u{10cdd}: '𐳝'
    SmallLetterNikolsburgOe,
    /// \u{10cde}: '𐳞'
    SmallLetterRudimentaOe,
    /// \u{10cdf}: '𐳟'
    SmallLetterOee,
    /// \u{10ce0}: '𐳠'
    SmallLetterEp,
    /// \u{10ce1}: '𐳡'
    SmallLetterEmp,
    /// \u{10ce2}: '𐳢'
    SmallLetterEr,
    /// \u{10ce3}: '𐳣'
    SmallLetterShortEr,
    /// \u{10ce4}: '𐳤'
    SmallLetterEs,
    /// \u{10ce5}: '𐳥'
    SmallLetterEsz,
    /// \u{10ce6}: '𐳦'
    SmallLetterEt,
    /// \u{10ce7}: '𐳧'
    SmallLetterEnt,
    /// \u{10ce8}: '𐳨'
    SmallLetterEty,
    /// \u{10ce9}: '𐳩'
    SmallLetterEch,
    /// \u{10cea}: '𐳪'
    SmallLetterU,
    /// \u{10ceb}: '𐳫'
    SmallLetterUu,
    /// \u{10cec}: '𐳬'
    SmallLetterNikolsburgUe,
    /// \u{10ced}: '𐳭'
    SmallLetterRudimentaUe,
    /// \u{10cee}: '𐳮'
    SmallLetterEv,
    /// \u{10cef}: '𐳯'
    SmallLetterEz,
    /// \u{10cf0}: '𐳰'
    SmallLetterEzs,
    /// \u{10cf1}: '𐳱'
    SmallLetterEntDashShapedSign,
    /// \u{10cf2}: '𐳲'
    SmallLetterUs,
    /// \u{10cfa}: '𐳺'
    NumberOne,
    /// \u{10cfb}: '𐳻'
    NumberFive,
    /// \u{10cfc}: '𐳼'
    NumberTen,
    /// \u{10cfd}: '𐳽'
    NumberFifty,
    /// \u{10cfe}: '𐳾'
    NumberOneHundred,
}

impl Into<char> for OldHungarian {
    fn into(self) -> char {
        match self {
            OldHungarian::CapitalLetterA => '𐲀',
            OldHungarian::CapitalLetterAa => '𐲁',
            OldHungarian::CapitalLetterEb => '𐲂',
            OldHungarian::CapitalLetterAmb => '𐲃',
            OldHungarian::CapitalLetterEc => '𐲄',
            OldHungarian::CapitalLetterEnc => '𐲅',
            OldHungarian::CapitalLetterEcs => '𐲆',
            OldHungarian::CapitalLetterEd => '𐲇',
            OldHungarian::CapitalLetterAnd => '𐲈',
            OldHungarian::CapitalLetterE => '𐲉',
            OldHungarian::CapitalLetterCloseE => '𐲊',
            OldHungarian::CapitalLetterEe => '𐲋',
            OldHungarian::CapitalLetterEf => '𐲌',
            OldHungarian::CapitalLetterEg => '𐲍',
            OldHungarian::CapitalLetterEgy => '𐲎',
            OldHungarian::CapitalLetterEh => '𐲏',
            OldHungarian::CapitalLetterI => '𐲐',
            OldHungarian::CapitalLetterIi => '𐲑',
            OldHungarian::CapitalLetterEj => '𐲒',
            OldHungarian::CapitalLetterEk => '𐲓',
            OldHungarian::CapitalLetterAk => '𐲔',
            OldHungarian::CapitalLetterUnk => '𐲕',
            OldHungarian::CapitalLetterEl => '𐲖',
            OldHungarian::CapitalLetterEly => '𐲗',
            OldHungarian::CapitalLetterEm => '𐲘',
            OldHungarian::CapitalLetterEn => '𐲙',
            OldHungarian::CapitalLetterEny => '𐲚',
            OldHungarian::CapitalLetterO => '𐲛',
            OldHungarian::CapitalLetterOo => '𐲜',
            OldHungarian::CapitalLetterNikolsburgOe => '𐲝',
            OldHungarian::CapitalLetterRudimentaOe => '𐲞',
            OldHungarian::CapitalLetterOee => '𐲟',
            OldHungarian::CapitalLetterEp => '𐲠',
            OldHungarian::CapitalLetterEmp => '𐲡',
            OldHungarian::CapitalLetterEr => '𐲢',
            OldHungarian::CapitalLetterShortEr => '𐲣',
            OldHungarian::CapitalLetterEs => '𐲤',
            OldHungarian::CapitalLetterEsz => '𐲥',
            OldHungarian::CapitalLetterEt => '𐲦',
            OldHungarian::CapitalLetterEnt => '𐲧',
            OldHungarian::CapitalLetterEty => '𐲨',
            OldHungarian::CapitalLetterEch => '𐲩',
            OldHungarian::CapitalLetterU => '𐲪',
            OldHungarian::CapitalLetterUu => '𐲫',
            OldHungarian::CapitalLetterNikolsburgUe => '𐲬',
            OldHungarian::CapitalLetterRudimentaUe => '𐲭',
            OldHungarian::CapitalLetterEv => '𐲮',
            OldHungarian::CapitalLetterEz => '𐲯',
            OldHungarian::CapitalLetterEzs => '𐲰',
            OldHungarian::CapitalLetterEntDashShapedSign => '𐲱',
            OldHungarian::CapitalLetterUs => '𐲲',
            OldHungarian::SmallLetterA => '𐳀',
            OldHungarian::SmallLetterAa => '𐳁',
            OldHungarian::SmallLetterEb => '𐳂',
            OldHungarian::SmallLetterAmb => '𐳃',
            OldHungarian::SmallLetterEc => '𐳄',
            OldHungarian::SmallLetterEnc => '𐳅',
            OldHungarian::SmallLetterEcs => '𐳆',
            OldHungarian::SmallLetterEd => '𐳇',
            OldHungarian::SmallLetterAnd => '𐳈',
            OldHungarian::SmallLetterE => '𐳉',
            OldHungarian::SmallLetterCloseE => '𐳊',
            OldHungarian::SmallLetterEe => '𐳋',
            OldHungarian::SmallLetterEf => '𐳌',
            OldHungarian::SmallLetterEg => '𐳍',
            OldHungarian::SmallLetterEgy => '𐳎',
            OldHungarian::SmallLetterEh => '𐳏',
            OldHungarian::SmallLetterI => '𐳐',
            OldHungarian::SmallLetterIi => '𐳑',
            OldHungarian::SmallLetterEj => '𐳒',
            OldHungarian::SmallLetterEk => '𐳓',
            OldHungarian::SmallLetterAk => '𐳔',
            OldHungarian::SmallLetterUnk => '𐳕',
            OldHungarian::SmallLetterEl => '𐳖',
            OldHungarian::SmallLetterEly => '𐳗',
            OldHungarian::SmallLetterEm => '𐳘',
            OldHungarian::SmallLetterEn => '𐳙',
            OldHungarian::SmallLetterEny => '𐳚',
            OldHungarian::SmallLetterO => '𐳛',
            OldHungarian::SmallLetterOo => '𐳜',
            OldHungarian::SmallLetterNikolsburgOe => '𐳝',
            OldHungarian::SmallLetterRudimentaOe => '𐳞',
            OldHungarian::SmallLetterOee => '𐳟',
            OldHungarian::SmallLetterEp => '𐳠',
            OldHungarian::SmallLetterEmp => '𐳡',
            OldHungarian::SmallLetterEr => '𐳢',
            OldHungarian::SmallLetterShortEr => '𐳣',
            OldHungarian::SmallLetterEs => '𐳤',
            OldHungarian::SmallLetterEsz => '𐳥',
            OldHungarian::SmallLetterEt => '𐳦',
            OldHungarian::SmallLetterEnt => '𐳧',
            OldHungarian::SmallLetterEty => '𐳨',
            OldHungarian::SmallLetterEch => '𐳩',
            OldHungarian::SmallLetterU => '𐳪',
            OldHungarian::SmallLetterUu => '𐳫',
            OldHungarian::SmallLetterNikolsburgUe => '𐳬',
            OldHungarian::SmallLetterRudimentaUe => '𐳭',
            OldHungarian::SmallLetterEv => '𐳮',
            OldHungarian::SmallLetterEz => '𐳯',
            OldHungarian::SmallLetterEzs => '𐳰',
            OldHungarian::SmallLetterEntDashShapedSign => '𐳱',
            OldHungarian::SmallLetterUs => '𐳲',
            OldHungarian::NumberOne => '𐳺',
            OldHungarian::NumberFive => '𐳻',
            OldHungarian::NumberTen => '𐳼',
            OldHungarian::NumberFifty => '𐳽',
            OldHungarian::NumberOneHundred => '𐳾',
        }
    }
}

impl std::convert::TryFrom<char> for OldHungarian {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𐲀' => Ok(OldHungarian::CapitalLetterA),
            '𐲁' => Ok(OldHungarian::CapitalLetterAa),
            '𐲂' => Ok(OldHungarian::CapitalLetterEb),
            '𐲃' => Ok(OldHungarian::CapitalLetterAmb),
            '𐲄' => Ok(OldHungarian::CapitalLetterEc),
            '𐲅' => Ok(OldHungarian::CapitalLetterEnc),
            '𐲆' => Ok(OldHungarian::CapitalLetterEcs),
            '𐲇' => Ok(OldHungarian::CapitalLetterEd),
            '𐲈' => Ok(OldHungarian::CapitalLetterAnd),
            '𐲉' => Ok(OldHungarian::CapitalLetterE),
            '𐲊' => Ok(OldHungarian::CapitalLetterCloseE),
            '𐲋' => Ok(OldHungarian::CapitalLetterEe),
            '𐲌' => Ok(OldHungarian::CapitalLetterEf),
            '𐲍' => Ok(OldHungarian::CapitalLetterEg),
            '𐲎' => Ok(OldHungarian::CapitalLetterEgy),
            '𐲏' => Ok(OldHungarian::CapitalLetterEh),
            '𐲐' => Ok(OldHungarian::CapitalLetterI),
            '𐲑' => Ok(OldHungarian::CapitalLetterIi),
            '𐲒' => Ok(OldHungarian::CapitalLetterEj),
            '𐲓' => Ok(OldHungarian::CapitalLetterEk),
            '𐲔' => Ok(OldHungarian::CapitalLetterAk),
            '𐲕' => Ok(OldHungarian::CapitalLetterUnk),
            '𐲖' => Ok(OldHungarian::CapitalLetterEl),
            '𐲗' => Ok(OldHungarian::CapitalLetterEly),
            '𐲘' => Ok(OldHungarian::CapitalLetterEm),
            '𐲙' => Ok(OldHungarian::CapitalLetterEn),
            '𐲚' => Ok(OldHungarian::CapitalLetterEny),
            '𐲛' => Ok(OldHungarian::CapitalLetterO),
            '𐲜' => Ok(OldHungarian::CapitalLetterOo),
            '𐲝' => Ok(OldHungarian::CapitalLetterNikolsburgOe),
            '𐲞' => Ok(OldHungarian::CapitalLetterRudimentaOe),
            '𐲟' => Ok(OldHungarian::CapitalLetterOee),
            '𐲠' => Ok(OldHungarian::CapitalLetterEp),
            '𐲡' => Ok(OldHungarian::CapitalLetterEmp),
            '𐲢' => Ok(OldHungarian::CapitalLetterEr),
            '𐲣' => Ok(OldHungarian::CapitalLetterShortEr),
            '𐲤' => Ok(OldHungarian::CapitalLetterEs),
            '𐲥' => Ok(OldHungarian::CapitalLetterEsz),
            '𐲦' => Ok(OldHungarian::CapitalLetterEt),
            '𐲧' => Ok(OldHungarian::CapitalLetterEnt),
            '𐲨' => Ok(OldHungarian::CapitalLetterEty),
            '𐲩' => Ok(OldHungarian::CapitalLetterEch),
            '𐲪' => Ok(OldHungarian::CapitalLetterU),
            '𐲫' => Ok(OldHungarian::CapitalLetterUu),
            '𐲬' => Ok(OldHungarian::CapitalLetterNikolsburgUe),
            '𐲭' => Ok(OldHungarian::CapitalLetterRudimentaUe),
            '𐲮' => Ok(OldHungarian::CapitalLetterEv),
            '𐲯' => Ok(OldHungarian::CapitalLetterEz),
            '𐲰' => Ok(OldHungarian::CapitalLetterEzs),
            '𐲱' => Ok(OldHungarian::CapitalLetterEntDashShapedSign),
            '𐲲' => Ok(OldHungarian::CapitalLetterUs),
            '𐳀' => Ok(OldHungarian::SmallLetterA),
            '𐳁' => Ok(OldHungarian::SmallLetterAa),
            '𐳂' => Ok(OldHungarian::SmallLetterEb),
            '𐳃' => Ok(OldHungarian::SmallLetterAmb),
            '𐳄' => Ok(OldHungarian::SmallLetterEc),
            '𐳅' => Ok(OldHungarian::SmallLetterEnc),
            '𐳆' => Ok(OldHungarian::SmallLetterEcs),
            '𐳇' => Ok(OldHungarian::SmallLetterEd),
            '𐳈' => Ok(OldHungarian::SmallLetterAnd),
            '𐳉' => Ok(OldHungarian::SmallLetterE),
            '𐳊' => Ok(OldHungarian::SmallLetterCloseE),
            '𐳋' => Ok(OldHungarian::SmallLetterEe),
            '𐳌' => Ok(OldHungarian::SmallLetterEf),
            '𐳍' => Ok(OldHungarian::SmallLetterEg),
            '𐳎' => Ok(OldHungarian::SmallLetterEgy),
            '𐳏' => Ok(OldHungarian::SmallLetterEh),
            '𐳐' => Ok(OldHungarian::SmallLetterI),
            '𐳑' => Ok(OldHungarian::SmallLetterIi),
            '𐳒' => Ok(OldHungarian::SmallLetterEj),
            '𐳓' => Ok(OldHungarian::SmallLetterEk),
            '𐳔' => Ok(OldHungarian::SmallLetterAk),
            '𐳕' => Ok(OldHungarian::SmallLetterUnk),
            '𐳖' => Ok(OldHungarian::SmallLetterEl),
            '𐳗' => Ok(OldHungarian::SmallLetterEly),
            '𐳘' => Ok(OldHungarian::SmallLetterEm),
            '𐳙' => Ok(OldHungarian::SmallLetterEn),
            '𐳚' => Ok(OldHungarian::SmallLetterEny),
            '𐳛' => Ok(OldHungarian::SmallLetterO),
            '𐳜' => Ok(OldHungarian::SmallLetterOo),
            '𐳝' => Ok(OldHungarian::SmallLetterNikolsburgOe),
            '𐳞' => Ok(OldHungarian::SmallLetterRudimentaOe),
            '𐳟' => Ok(OldHungarian::SmallLetterOee),
            '𐳠' => Ok(OldHungarian::SmallLetterEp),
            '𐳡' => Ok(OldHungarian::SmallLetterEmp),
            '𐳢' => Ok(OldHungarian::SmallLetterEr),
            '𐳣' => Ok(OldHungarian::SmallLetterShortEr),
            '𐳤' => Ok(OldHungarian::SmallLetterEs),
            '𐳥' => Ok(OldHungarian::SmallLetterEsz),
            '𐳦' => Ok(OldHungarian::SmallLetterEt),
            '𐳧' => Ok(OldHungarian::SmallLetterEnt),
            '𐳨' => Ok(OldHungarian::SmallLetterEty),
            '𐳩' => Ok(OldHungarian::SmallLetterEch),
            '𐳪' => Ok(OldHungarian::SmallLetterU),
            '𐳫' => Ok(OldHungarian::SmallLetterUu),
            '𐳬' => Ok(OldHungarian::SmallLetterNikolsburgUe),
            '𐳭' => Ok(OldHungarian::SmallLetterRudimentaUe),
            '𐳮' => Ok(OldHungarian::SmallLetterEv),
            '𐳯' => Ok(OldHungarian::SmallLetterEz),
            '𐳰' => Ok(OldHungarian::SmallLetterEzs),
            '𐳱' => Ok(OldHungarian::SmallLetterEntDashShapedSign),
            '𐳲' => Ok(OldHungarian::SmallLetterUs),
            '𐳺' => Ok(OldHungarian::NumberOne),
            '𐳻' => Ok(OldHungarian::NumberFive),
            '𐳼' => Ok(OldHungarian::NumberTen),
            '𐳽' => Ok(OldHungarian::NumberFifty),
            '𐳾' => Ok(OldHungarian::NumberOneHundred),
            _ => Err(()),
        }
    }
}

impl Into<u32> for OldHungarian {
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

impl std::convert::TryFrom<u32> for OldHungarian {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for OldHungarian {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl OldHungarian {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        OldHungarian::CapitalLetterA
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            OldHungarian::CapitalLetterA => "old hungarian capital letter a",
            OldHungarian::CapitalLetterAa => "old hungarian capital letter aa",
            OldHungarian::CapitalLetterEb => "old hungarian capital letter eb",
            OldHungarian::CapitalLetterAmb => "old hungarian capital letter amb",
            OldHungarian::CapitalLetterEc => "old hungarian capital letter ec",
            OldHungarian::CapitalLetterEnc => "old hungarian capital letter enc",
            OldHungarian::CapitalLetterEcs => "old hungarian capital letter ecs",
            OldHungarian::CapitalLetterEd => "old hungarian capital letter ed",
            OldHungarian::CapitalLetterAnd => "old hungarian capital letter and",
            OldHungarian::CapitalLetterE => "old hungarian capital letter e",
            OldHungarian::CapitalLetterCloseE => "old hungarian capital letter close e",
            OldHungarian::CapitalLetterEe => "old hungarian capital letter ee",
            OldHungarian::CapitalLetterEf => "old hungarian capital letter ef",
            OldHungarian::CapitalLetterEg => "old hungarian capital letter eg",
            OldHungarian::CapitalLetterEgy => "old hungarian capital letter egy",
            OldHungarian::CapitalLetterEh => "old hungarian capital letter eh",
            OldHungarian::CapitalLetterI => "old hungarian capital letter i",
            OldHungarian::CapitalLetterIi => "old hungarian capital letter ii",
            OldHungarian::CapitalLetterEj => "old hungarian capital letter ej",
            OldHungarian::CapitalLetterEk => "old hungarian capital letter ek",
            OldHungarian::CapitalLetterAk => "old hungarian capital letter ak",
            OldHungarian::CapitalLetterUnk => "old hungarian capital letter unk",
            OldHungarian::CapitalLetterEl => "old hungarian capital letter el",
            OldHungarian::CapitalLetterEly => "old hungarian capital letter ely",
            OldHungarian::CapitalLetterEm => "old hungarian capital letter em",
            OldHungarian::CapitalLetterEn => "old hungarian capital letter en",
            OldHungarian::CapitalLetterEny => "old hungarian capital letter eny",
            OldHungarian::CapitalLetterO => "old hungarian capital letter o",
            OldHungarian::CapitalLetterOo => "old hungarian capital letter oo",
            OldHungarian::CapitalLetterNikolsburgOe => "old hungarian capital letter nikolsburg oe",
            OldHungarian::CapitalLetterRudimentaOe => "old hungarian capital letter rudimenta oe",
            OldHungarian::CapitalLetterOee => "old hungarian capital letter oee",
            OldHungarian::CapitalLetterEp => "old hungarian capital letter ep",
            OldHungarian::CapitalLetterEmp => "old hungarian capital letter emp",
            OldHungarian::CapitalLetterEr => "old hungarian capital letter er",
            OldHungarian::CapitalLetterShortEr => "old hungarian capital letter short er",
            OldHungarian::CapitalLetterEs => "old hungarian capital letter es",
            OldHungarian::CapitalLetterEsz => "old hungarian capital letter esz",
            OldHungarian::CapitalLetterEt => "old hungarian capital letter et",
            OldHungarian::CapitalLetterEnt => "old hungarian capital letter ent",
            OldHungarian::CapitalLetterEty => "old hungarian capital letter ety",
            OldHungarian::CapitalLetterEch => "old hungarian capital letter ech",
            OldHungarian::CapitalLetterU => "old hungarian capital letter u",
            OldHungarian::CapitalLetterUu => "old hungarian capital letter uu",
            OldHungarian::CapitalLetterNikolsburgUe => "old hungarian capital letter nikolsburg ue",
            OldHungarian::CapitalLetterRudimentaUe => "old hungarian capital letter rudimenta ue",
            OldHungarian::CapitalLetterEv => "old hungarian capital letter ev",
            OldHungarian::CapitalLetterEz => "old hungarian capital letter ez",
            OldHungarian::CapitalLetterEzs => "old hungarian capital letter ezs",
            OldHungarian::CapitalLetterEntDashShapedSign => "old hungarian capital letter ent-shaped sign",
            OldHungarian::CapitalLetterUs => "old hungarian capital letter us",
            OldHungarian::SmallLetterA => "old hungarian small letter a",
            OldHungarian::SmallLetterAa => "old hungarian small letter aa",
            OldHungarian::SmallLetterEb => "old hungarian small letter eb",
            OldHungarian::SmallLetterAmb => "old hungarian small letter amb",
            OldHungarian::SmallLetterEc => "old hungarian small letter ec",
            OldHungarian::SmallLetterEnc => "old hungarian small letter enc",
            OldHungarian::SmallLetterEcs => "old hungarian small letter ecs",
            OldHungarian::SmallLetterEd => "old hungarian small letter ed",
            OldHungarian::SmallLetterAnd => "old hungarian small letter and",
            OldHungarian::SmallLetterE => "old hungarian small letter e",
            OldHungarian::SmallLetterCloseE => "old hungarian small letter close e",
            OldHungarian::SmallLetterEe => "old hungarian small letter ee",
            OldHungarian::SmallLetterEf => "old hungarian small letter ef",
            OldHungarian::SmallLetterEg => "old hungarian small letter eg",
            OldHungarian::SmallLetterEgy => "old hungarian small letter egy",
            OldHungarian::SmallLetterEh => "old hungarian small letter eh",
            OldHungarian::SmallLetterI => "old hungarian small letter i",
            OldHungarian::SmallLetterIi => "old hungarian small letter ii",
            OldHungarian::SmallLetterEj => "old hungarian small letter ej",
            OldHungarian::SmallLetterEk => "old hungarian small letter ek",
            OldHungarian::SmallLetterAk => "old hungarian small letter ak",
            OldHungarian::SmallLetterUnk => "old hungarian small letter unk",
            OldHungarian::SmallLetterEl => "old hungarian small letter el",
            OldHungarian::SmallLetterEly => "old hungarian small letter ely",
            OldHungarian::SmallLetterEm => "old hungarian small letter em",
            OldHungarian::SmallLetterEn => "old hungarian small letter en",
            OldHungarian::SmallLetterEny => "old hungarian small letter eny",
            OldHungarian::SmallLetterO => "old hungarian small letter o",
            OldHungarian::SmallLetterOo => "old hungarian small letter oo",
            OldHungarian::SmallLetterNikolsburgOe => "old hungarian small letter nikolsburg oe",
            OldHungarian::SmallLetterRudimentaOe => "old hungarian small letter rudimenta oe",
            OldHungarian::SmallLetterOee => "old hungarian small letter oee",
            OldHungarian::SmallLetterEp => "old hungarian small letter ep",
            OldHungarian::SmallLetterEmp => "old hungarian small letter emp",
            OldHungarian::SmallLetterEr => "old hungarian small letter er",
            OldHungarian::SmallLetterShortEr => "old hungarian small letter short er",
            OldHungarian::SmallLetterEs => "old hungarian small letter es",
            OldHungarian::SmallLetterEsz => "old hungarian small letter esz",
            OldHungarian::SmallLetterEt => "old hungarian small letter et",
            OldHungarian::SmallLetterEnt => "old hungarian small letter ent",
            OldHungarian::SmallLetterEty => "old hungarian small letter ety",
            OldHungarian::SmallLetterEch => "old hungarian small letter ech",
            OldHungarian::SmallLetterU => "old hungarian small letter u",
            OldHungarian::SmallLetterUu => "old hungarian small letter uu",
            OldHungarian::SmallLetterNikolsburgUe => "old hungarian small letter nikolsburg ue",
            OldHungarian::SmallLetterRudimentaUe => "old hungarian small letter rudimenta ue",
            OldHungarian::SmallLetterEv => "old hungarian small letter ev",
            OldHungarian::SmallLetterEz => "old hungarian small letter ez",
            OldHungarian::SmallLetterEzs => "old hungarian small letter ezs",
            OldHungarian::SmallLetterEntDashShapedSign => "old hungarian small letter ent-shaped sign",
            OldHungarian::SmallLetterUs => "old hungarian small letter us",
            OldHungarian::NumberOne => "old hungarian number one",
            OldHungarian::NumberFive => "old hungarian number five",
            OldHungarian::NumberTen => "old hungarian number ten",
            OldHungarian::NumberFifty => "old hungarian number fifty",
            OldHungarian::NumberOneHundred => "old hungarian number one hundred",
        }
    }
}
