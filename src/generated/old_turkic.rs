
/// An enum to represent all characters in the OldTurkic block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum OldTurkic {
    /// \u{10c00}: '𐰀'
    LetterOrkhonA,
    /// \u{10c01}: '𐰁'
    LetterYeniseiA,
    /// \u{10c02}: '𐰂'
    LetterYeniseiAe,
    /// \u{10c03}: '𐰃'
    LetterOrkhonI,
    /// \u{10c04}: '𐰄'
    LetterYeniseiI,
    /// \u{10c05}: '𐰅'
    LetterYeniseiE,
    /// \u{10c06}: '𐰆'
    LetterOrkhonO,
    /// \u{10c07}: '𐰇'
    LetterOrkhonOe,
    /// \u{10c08}: '𐰈'
    LetterYeniseiOe,
    /// \u{10c09}: '𐰉'
    LetterOrkhonAb,
    /// \u{10c0a}: '𐰊'
    LetterYeniseiAb,
    /// \u{10c0b}: '𐰋'
    LetterOrkhonAeb,
    /// \u{10c0c}: '𐰌'
    LetterYeniseiAeb,
    /// \u{10c0d}: '𐰍'
    LetterOrkhonAg,
    /// \u{10c0e}: '𐰎'
    LetterYeniseiAg,
    /// \u{10c0f}: '𐰏'
    LetterOrkhonAeg,
    /// \u{10c10}: '𐰐'
    LetterYeniseiAeg,
    /// \u{10c11}: '𐰑'
    LetterOrkhonAd,
    /// \u{10c12}: '𐰒'
    LetterYeniseiAd,
    /// \u{10c13}: '𐰓'
    LetterOrkhonAed,
    /// \u{10c14}: '𐰔'
    LetterOrkhonEz,
    /// \u{10c15}: '𐰕'
    LetterYeniseiEz,
    /// \u{10c16}: '𐰖'
    LetterOrkhonAy,
    /// \u{10c17}: '𐰗'
    LetterYeniseiAy,
    /// \u{10c18}: '𐰘'
    LetterOrkhonAey,
    /// \u{10c19}: '𐰙'
    LetterYeniseiAey,
    /// \u{10c1a}: '𐰚'
    LetterOrkhonAek,
    /// \u{10c1b}: '𐰛'
    LetterYeniseiAek,
    /// \u{10c1c}: '𐰜'
    LetterOrkhonOek,
    /// \u{10c1d}: '𐰝'
    LetterYeniseiOek,
    /// \u{10c1e}: '𐰞'
    LetterOrkhonAl,
    /// \u{10c1f}: '𐰟'
    LetterYeniseiAl,
    /// \u{10c20}: '𐰠'
    LetterOrkhonAel,
    /// \u{10c21}: '𐰡'
    LetterOrkhonElt,
    /// \u{10c22}: '𐰢'
    LetterOrkhonEm,
    /// \u{10c23}: '𐰣'
    LetterOrkhonAn,
    /// \u{10c24}: '𐰤'
    LetterOrkhonAen,
    /// \u{10c25}: '𐰥'
    LetterYeniseiAen,
    /// \u{10c26}: '𐰦'
    LetterOrkhonEnt,
    /// \u{10c27}: '𐰧'
    LetterYeniseiEnt,
    /// \u{10c28}: '𐰨'
    LetterOrkhonEnc,
    /// \u{10c29}: '𐰩'
    LetterYeniseiEnc,
    /// \u{10c2a}: '𐰪'
    LetterOrkhonEny,
    /// \u{10c2b}: '𐰫'
    LetterYeniseiEny,
    /// \u{10c2c}: '𐰬'
    LetterYeniseiAng,
    /// \u{10c2d}: '𐰭'
    LetterOrkhonEng,
    /// \u{10c2e}: '𐰮'
    LetterYeniseiAeng,
    /// \u{10c2f}: '𐰯'
    LetterOrkhonEp,
    /// \u{10c30}: '𐰰'
    LetterOrkhonOp,
    /// \u{10c31}: '𐰱'
    LetterOrkhonIc,
    /// \u{10c32}: '𐰲'
    LetterOrkhonEc,
    /// \u{10c33}: '𐰳'
    LetterYeniseiEc,
    /// \u{10c34}: '𐰴'
    LetterOrkhonAq,
    /// \u{10c35}: '𐰵'
    LetterYeniseiAq,
    /// \u{10c36}: '𐰶'
    LetterOrkhonIq,
    /// \u{10c37}: '𐰷'
    LetterYeniseiIq,
    /// \u{10c38}: '𐰸'
    LetterOrkhonOq,
    /// \u{10c39}: '𐰹'
    LetterYeniseiOq,
    /// \u{10c3a}: '𐰺'
    LetterOrkhonAr,
    /// \u{10c3b}: '𐰻'
    LetterYeniseiAr,
    /// \u{10c3c}: '𐰼'
    LetterOrkhonAer,
    /// \u{10c3d}: '𐰽'
    LetterOrkhonAs,
    /// \u{10c3e}: '𐰾'
    LetterOrkhonAes,
    /// \u{10c3f}: '𐰿'
    LetterOrkhonAsh,
    /// \u{10c40}: '𐱀'
    LetterYeniseiAsh,
    /// \u{10c41}: '𐱁'
    LetterOrkhonEsh,
    /// \u{10c42}: '𐱂'
    LetterYeniseiEsh,
    /// \u{10c43}: '𐱃'
    LetterOrkhonAt,
    /// \u{10c44}: '𐱄'
    LetterYeniseiAt,
    /// \u{10c45}: '𐱅'
    LetterOrkhonAet,
    /// \u{10c46}: '𐱆'
    LetterYeniseiAet,
    /// \u{10c47}: '𐱇'
    LetterOrkhonOt,
    /// \u{10c48}: '𐱈'
    LetterOrkhonBash,
}

impl Into<char> for OldTurkic {
    fn into(self) -> char {
        match self {
            OldTurkic::LetterOrkhonA => '𐰀',
            OldTurkic::LetterYeniseiA => '𐰁',
            OldTurkic::LetterYeniseiAe => '𐰂',
            OldTurkic::LetterOrkhonI => '𐰃',
            OldTurkic::LetterYeniseiI => '𐰄',
            OldTurkic::LetterYeniseiE => '𐰅',
            OldTurkic::LetterOrkhonO => '𐰆',
            OldTurkic::LetterOrkhonOe => '𐰇',
            OldTurkic::LetterYeniseiOe => '𐰈',
            OldTurkic::LetterOrkhonAb => '𐰉',
            OldTurkic::LetterYeniseiAb => '𐰊',
            OldTurkic::LetterOrkhonAeb => '𐰋',
            OldTurkic::LetterYeniseiAeb => '𐰌',
            OldTurkic::LetterOrkhonAg => '𐰍',
            OldTurkic::LetterYeniseiAg => '𐰎',
            OldTurkic::LetterOrkhonAeg => '𐰏',
            OldTurkic::LetterYeniseiAeg => '𐰐',
            OldTurkic::LetterOrkhonAd => '𐰑',
            OldTurkic::LetterYeniseiAd => '𐰒',
            OldTurkic::LetterOrkhonAed => '𐰓',
            OldTurkic::LetterOrkhonEz => '𐰔',
            OldTurkic::LetterYeniseiEz => '𐰕',
            OldTurkic::LetterOrkhonAy => '𐰖',
            OldTurkic::LetterYeniseiAy => '𐰗',
            OldTurkic::LetterOrkhonAey => '𐰘',
            OldTurkic::LetterYeniseiAey => '𐰙',
            OldTurkic::LetterOrkhonAek => '𐰚',
            OldTurkic::LetterYeniseiAek => '𐰛',
            OldTurkic::LetterOrkhonOek => '𐰜',
            OldTurkic::LetterYeniseiOek => '𐰝',
            OldTurkic::LetterOrkhonAl => '𐰞',
            OldTurkic::LetterYeniseiAl => '𐰟',
            OldTurkic::LetterOrkhonAel => '𐰠',
            OldTurkic::LetterOrkhonElt => '𐰡',
            OldTurkic::LetterOrkhonEm => '𐰢',
            OldTurkic::LetterOrkhonAn => '𐰣',
            OldTurkic::LetterOrkhonAen => '𐰤',
            OldTurkic::LetterYeniseiAen => '𐰥',
            OldTurkic::LetterOrkhonEnt => '𐰦',
            OldTurkic::LetterYeniseiEnt => '𐰧',
            OldTurkic::LetterOrkhonEnc => '𐰨',
            OldTurkic::LetterYeniseiEnc => '𐰩',
            OldTurkic::LetterOrkhonEny => '𐰪',
            OldTurkic::LetterYeniseiEny => '𐰫',
            OldTurkic::LetterYeniseiAng => '𐰬',
            OldTurkic::LetterOrkhonEng => '𐰭',
            OldTurkic::LetterYeniseiAeng => '𐰮',
            OldTurkic::LetterOrkhonEp => '𐰯',
            OldTurkic::LetterOrkhonOp => '𐰰',
            OldTurkic::LetterOrkhonIc => '𐰱',
            OldTurkic::LetterOrkhonEc => '𐰲',
            OldTurkic::LetterYeniseiEc => '𐰳',
            OldTurkic::LetterOrkhonAq => '𐰴',
            OldTurkic::LetterYeniseiAq => '𐰵',
            OldTurkic::LetterOrkhonIq => '𐰶',
            OldTurkic::LetterYeniseiIq => '𐰷',
            OldTurkic::LetterOrkhonOq => '𐰸',
            OldTurkic::LetterYeniseiOq => '𐰹',
            OldTurkic::LetterOrkhonAr => '𐰺',
            OldTurkic::LetterYeniseiAr => '𐰻',
            OldTurkic::LetterOrkhonAer => '𐰼',
            OldTurkic::LetterOrkhonAs => '𐰽',
            OldTurkic::LetterOrkhonAes => '𐰾',
            OldTurkic::LetterOrkhonAsh => '𐰿',
            OldTurkic::LetterYeniseiAsh => '𐱀',
            OldTurkic::LetterOrkhonEsh => '𐱁',
            OldTurkic::LetterYeniseiEsh => '𐱂',
            OldTurkic::LetterOrkhonAt => '𐱃',
            OldTurkic::LetterYeniseiAt => '𐱄',
            OldTurkic::LetterOrkhonAet => '𐱅',
            OldTurkic::LetterYeniseiAet => '𐱆',
            OldTurkic::LetterOrkhonOt => '𐱇',
            OldTurkic::LetterOrkhonBash => '𐱈',
        }
    }
}

impl std::convert::TryFrom<char> for OldTurkic {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𐰀' => Ok(OldTurkic::LetterOrkhonA),
            '𐰁' => Ok(OldTurkic::LetterYeniseiA),
            '𐰂' => Ok(OldTurkic::LetterYeniseiAe),
            '𐰃' => Ok(OldTurkic::LetterOrkhonI),
            '𐰄' => Ok(OldTurkic::LetterYeniseiI),
            '𐰅' => Ok(OldTurkic::LetterYeniseiE),
            '𐰆' => Ok(OldTurkic::LetterOrkhonO),
            '𐰇' => Ok(OldTurkic::LetterOrkhonOe),
            '𐰈' => Ok(OldTurkic::LetterYeniseiOe),
            '𐰉' => Ok(OldTurkic::LetterOrkhonAb),
            '𐰊' => Ok(OldTurkic::LetterYeniseiAb),
            '𐰋' => Ok(OldTurkic::LetterOrkhonAeb),
            '𐰌' => Ok(OldTurkic::LetterYeniseiAeb),
            '𐰍' => Ok(OldTurkic::LetterOrkhonAg),
            '𐰎' => Ok(OldTurkic::LetterYeniseiAg),
            '𐰏' => Ok(OldTurkic::LetterOrkhonAeg),
            '𐰐' => Ok(OldTurkic::LetterYeniseiAeg),
            '𐰑' => Ok(OldTurkic::LetterOrkhonAd),
            '𐰒' => Ok(OldTurkic::LetterYeniseiAd),
            '𐰓' => Ok(OldTurkic::LetterOrkhonAed),
            '𐰔' => Ok(OldTurkic::LetterOrkhonEz),
            '𐰕' => Ok(OldTurkic::LetterYeniseiEz),
            '𐰖' => Ok(OldTurkic::LetterOrkhonAy),
            '𐰗' => Ok(OldTurkic::LetterYeniseiAy),
            '𐰘' => Ok(OldTurkic::LetterOrkhonAey),
            '𐰙' => Ok(OldTurkic::LetterYeniseiAey),
            '𐰚' => Ok(OldTurkic::LetterOrkhonAek),
            '𐰛' => Ok(OldTurkic::LetterYeniseiAek),
            '𐰜' => Ok(OldTurkic::LetterOrkhonOek),
            '𐰝' => Ok(OldTurkic::LetterYeniseiOek),
            '𐰞' => Ok(OldTurkic::LetterOrkhonAl),
            '𐰟' => Ok(OldTurkic::LetterYeniseiAl),
            '𐰠' => Ok(OldTurkic::LetterOrkhonAel),
            '𐰡' => Ok(OldTurkic::LetterOrkhonElt),
            '𐰢' => Ok(OldTurkic::LetterOrkhonEm),
            '𐰣' => Ok(OldTurkic::LetterOrkhonAn),
            '𐰤' => Ok(OldTurkic::LetterOrkhonAen),
            '𐰥' => Ok(OldTurkic::LetterYeniseiAen),
            '𐰦' => Ok(OldTurkic::LetterOrkhonEnt),
            '𐰧' => Ok(OldTurkic::LetterYeniseiEnt),
            '𐰨' => Ok(OldTurkic::LetterOrkhonEnc),
            '𐰩' => Ok(OldTurkic::LetterYeniseiEnc),
            '𐰪' => Ok(OldTurkic::LetterOrkhonEny),
            '𐰫' => Ok(OldTurkic::LetterYeniseiEny),
            '𐰬' => Ok(OldTurkic::LetterYeniseiAng),
            '𐰭' => Ok(OldTurkic::LetterOrkhonEng),
            '𐰮' => Ok(OldTurkic::LetterYeniseiAeng),
            '𐰯' => Ok(OldTurkic::LetterOrkhonEp),
            '𐰰' => Ok(OldTurkic::LetterOrkhonOp),
            '𐰱' => Ok(OldTurkic::LetterOrkhonIc),
            '𐰲' => Ok(OldTurkic::LetterOrkhonEc),
            '𐰳' => Ok(OldTurkic::LetterYeniseiEc),
            '𐰴' => Ok(OldTurkic::LetterOrkhonAq),
            '𐰵' => Ok(OldTurkic::LetterYeniseiAq),
            '𐰶' => Ok(OldTurkic::LetterOrkhonIq),
            '𐰷' => Ok(OldTurkic::LetterYeniseiIq),
            '𐰸' => Ok(OldTurkic::LetterOrkhonOq),
            '𐰹' => Ok(OldTurkic::LetterYeniseiOq),
            '𐰺' => Ok(OldTurkic::LetterOrkhonAr),
            '𐰻' => Ok(OldTurkic::LetterYeniseiAr),
            '𐰼' => Ok(OldTurkic::LetterOrkhonAer),
            '𐰽' => Ok(OldTurkic::LetterOrkhonAs),
            '𐰾' => Ok(OldTurkic::LetterOrkhonAes),
            '𐰿' => Ok(OldTurkic::LetterOrkhonAsh),
            '𐱀' => Ok(OldTurkic::LetterYeniseiAsh),
            '𐱁' => Ok(OldTurkic::LetterOrkhonEsh),
            '𐱂' => Ok(OldTurkic::LetterYeniseiEsh),
            '𐱃' => Ok(OldTurkic::LetterOrkhonAt),
            '𐱄' => Ok(OldTurkic::LetterYeniseiAt),
            '𐱅' => Ok(OldTurkic::LetterOrkhonAet),
            '𐱆' => Ok(OldTurkic::LetterYeniseiAet),
            '𐱇' => Ok(OldTurkic::LetterOrkhonOt),
            '𐱈' => Ok(OldTurkic::LetterOrkhonBash),
            _ => Err(()),
        }
    }
}

impl Into<u32> for OldTurkic {
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

impl std::convert::TryFrom<u32> for OldTurkic {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for OldTurkic {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl OldTurkic {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        OldTurkic::LetterOrkhonA
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            OldTurkic::LetterOrkhonA => "old turkic letter orkhon a",
            OldTurkic::LetterYeniseiA => "old turkic letter yenisei a",
            OldTurkic::LetterYeniseiAe => "old turkic letter yenisei ae",
            OldTurkic::LetterOrkhonI => "old turkic letter orkhon i",
            OldTurkic::LetterYeniseiI => "old turkic letter yenisei i",
            OldTurkic::LetterYeniseiE => "old turkic letter yenisei e",
            OldTurkic::LetterOrkhonO => "old turkic letter orkhon o",
            OldTurkic::LetterOrkhonOe => "old turkic letter orkhon oe",
            OldTurkic::LetterYeniseiOe => "old turkic letter yenisei oe",
            OldTurkic::LetterOrkhonAb => "old turkic letter orkhon ab",
            OldTurkic::LetterYeniseiAb => "old turkic letter yenisei ab",
            OldTurkic::LetterOrkhonAeb => "old turkic letter orkhon aeb",
            OldTurkic::LetterYeniseiAeb => "old turkic letter yenisei aeb",
            OldTurkic::LetterOrkhonAg => "old turkic letter orkhon ag",
            OldTurkic::LetterYeniseiAg => "old turkic letter yenisei ag",
            OldTurkic::LetterOrkhonAeg => "old turkic letter orkhon aeg",
            OldTurkic::LetterYeniseiAeg => "old turkic letter yenisei aeg",
            OldTurkic::LetterOrkhonAd => "old turkic letter orkhon ad",
            OldTurkic::LetterYeniseiAd => "old turkic letter yenisei ad",
            OldTurkic::LetterOrkhonAed => "old turkic letter orkhon aed",
            OldTurkic::LetterOrkhonEz => "old turkic letter orkhon ez",
            OldTurkic::LetterYeniseiEz => "old turkic letter yenisei ez",
            OldTurkic::LetterOrkhonAy => "old turkic letter orkhon ay",
            OldTurkic::LetterYeniseiAy => "old turkic letter yenisei ay",
            OldTurkic::LetterOrkhonAey => "old turkic letter orkhon aey",
            OldTurkic::LetterYeniseiAey => "old turkic letter yenisei aey",
            OldTurkic::LetterOrkhonAek => "old turkic letter orkhon aek",
            OldTurkic::LetterYeniseiAek => "old turkic letter yenisei aek",
            OldTurkic::LetterOrkhonOek => "old turkic letter orkhon oek",
            OldTurkic::LetterYeniseiOek => "old turkic letter yenisei oek",
            OldTurkic::LetterOrkhonAl => "old turkic letter orkhon al",
            OldTurkic::LetterYeniseiAl => "old turkic letter yenisei al",
            OldTurkic::LetterOrkhonAel => "old turkic letter orkhon ael",
            OldTurkic::LetterOrkhonElt => "old turkic letter orkhon elt",
            OldTurkic::LetterOrkhonEm => "old turkic letter orkhon em",
            OldTurkic::LetterOrkhonAn => "old turkic letter orkhon an",
            OldTurkic::LetterOrkhonAen => "old turkic letter orkhon aen",
            OldTurkic::LetterYeniseiAen => "old turkic letter yenisei aen",
            OldTurkic::LetterOrkhonEnt => "old turkic letter orkhon ent",
            OldTurkic::LetterYeniseiEnt => "old turkic letter yenisei ent",
            OldTurkic::LetterOrkhonEnc => "old turkic letter orkhon enc",
            OldTurkic::LetterYeniseiEnc => "old turkic letter yenisei enc",
            OldTurkic::LetterOrkhonEny => "old turkic letter orkhon eny",
            OldTurkic::LetterYeniseiEny => "old turkic letter yenisei eny",
            OldTurkic::LetterYeniseiAng => "old turkic letter yenisei ang",
            OldTurkic::LetterOrkhonEng => "old turkic letter orkhon eng",
            OldTurkic::LetterYeniseiAeng => "old turkic letter yenisei aeng",
            OldTurkic::LetterOrkhonEp => "old turkic letter orkhon ep",
            OldTurkic::LetterOrkhonOp => "old turkic letter orkhon op",
            OldTurkic::LetterOrkhonIc => "old turkic letter orkhon ic",
            OldTurkic::LetterOrkhonEc => "old turkic letter orkhon ec",
            OldTurkic::LetterYeniseiEc => "old turkic letter yenisei ec",
            OldTurkic::LetterOrkhonAq => "old turkic letter orkhon aq",
            OldTurkic::LetterYeniseiAq => "old turkic letter yenisei aq",
            OldTurkic::LetterOrkhonIq => "old turkic letter orkhon iq",
            OldTurkic::LetterYeniseiIq => "old turkic letter yenisei iq",
            OldTurkic::LetterOrkhonOq => "old turkic letter orkhon oq",
            OldTurkic::LetterYeniseiOq => "old turkic letter yenisei oq",
            OldTurkic::LetterOrkhonAr => "old turkic letter orkhon ar",
            OldTurkic::LetterYeniseiAr => "old turkic letter yenisei ar",
            OldTurkic::LetterOrkhonAer => "old turkic letter orkhon aer",
            OldTurkic::LetterOrkhonAs => "old turkic letter orkhon as",
            OldTurkic::LetterOrkhonAes => "old turkic letter orkhon aes",
            OldTurkic::LetterOrkhonAsh => "old turkic letter orkhon ash",
            OldTurkic::LetterYeniseiAsh => "old turkic letter yenisei ash",
            OldTurkic::LetterOrkhonEsh => "old turkic letter orkhon esh",
            OldTurkic::LetterYeniseiEsh => "old turkic letter yenisei esh",
            OldTurkic::LetterOrkhonAt => "old turkic letter orkhon at",
            OldTurkic::LetterYeniseiAt => "old turkic letter yenisei at",
            OldTurkic::LetterOrkhonAet => "old turkic letter orkhon aet",
            OldTurkic::LetterYeniseiAet => "old turkic letter yenisei aet",
            OldTurkic::LetterOrkhonOt => "old turkic letter orkhon ot",
            OldTurkic::LetterOrkhonBash => "old turkic letter orkhon bash",
        }
    }
}
