
/// An enum to represent all characters in the Duployan block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Duployan {
    /// \u{1bc00}: '𛰀'
    LetterH,
    /// \u{1bc01}: '𛰁'
    LetterX,
    /// \u{1bc02}: '𛰂'
    LetterP,
    /// \u{1bc03}: '𛰃'
    LetterT,
    /// \u{1bc04}: '𛰄'
    LetterF,
    /// \u{1bc05}: '𛰅'
    LetterK,
    /// \u{1bc06}: '𛰆'
    LetterL,
    /// \u{1bc07}: '𛰇'
    LetterB,
    /// \u{1bc08}: '𛰈'
    LetterD,
    /// \u{1bc09}: '𛰉'
    LetterV,
    /// \u{1bc0a}: '𛰊'
    LetterG,
    /// \u{1bc0b}: '𛰋'
    LetterR,
    /// \u{1bc0c}: '𛰌'
    LetterPN,
    /// \u{1bc0d}: '𛰍'
    LetterDS,
    /// \u{1bc0e}: '𛰎'
    LetterFN,
    /// \u{1bc0f}: '𛰏'
    LetterKM,
    /// \u{1bc10}: '𛰐'
    LetterRS,
    /// \u{1bc11}: '𛰑'
    LetterTh,
    /// \u{1bc12}: '𛰒'
    LetterSloanDh,
    /// \u{1bc13}: '𛰓'
    LetterDh,
    /// \u{1bc14}: '𛰔'
    LetterKk,
    /// \u{1bc15}: '𛰕'
    LetterSloanJ,
    /// \u{1bc16}: '𛰖'
    LetterHl,
    /// \u{1bc17}: '𛰗'
    LetterLh,
    /// \u{1bc18}: '𛰘'
    LetterRh,
    /// \u{1bc19}: '𛰙'
    LetterM,
    /// \u{1bc1a}: '𛰚'
    LetterN,
    /// \u{1bc1b}: '𛰛'
    LetterJ,
    /// \u{1bc1c}: '𛰜'
    LetterS,
    /// \u{1bc1d}: '𛰝'
    LetterMN,
    /// \u{1bc1e}: '𛰞'
    LetterNM,
    /// \u{1bc1f}: '𛰟'
    LetterJM,
    /// \u{1bc20}: '𛰠'
    LetterSJ,
    /// \u{1bc21}: '𛰡'
    LetterMWithDot,
    /// \u{1bc22}: '𛰢'
    LetterNWithDot,
    /// \u{1bc23}: '𛰣'
    LetterJWithDot,
    /// \u{1bc24}: '𛰤'
    LetterJWithDotsInsideAndAbove,
    /// \u{1bc25}: '𛰥'
    LetterSWithDot,
    /// \u{1bc26}: '𛰦'
    LetterSWithDotBelow,
    /// \u{1bc27}: '𛰧'
    LetterMS,
    /// \u{1bc28}: '𛰨'
    LetterNS,
    /// \u{1bc29}: '𛰩'
    LetterJS,
    /// \u{1bc2a}: '𛰪'
    LetterSS,
    /// \u{1bc2b}: '𛰫'
    LetterMNS,
    /// \u{1bc2c}: '𛰬'
    LetterNMS,
    /// \u{1bc2d}: '𛰭'
    LetterJMS,
    /// \u{1bc2e}: '𛰮'
    LetterSJS,
    /// \u{1bc2f}: '𛰯'
    LetterJSWithDot,
    /// \u{1bc30}: '𛰰'
    LetterJN,
    /// \u{1bc31}: '𛰱'
    LetterJNS,
    /// \u{1bc32}: '𛰲'
    LetterST,
    /// \u{1bc33}: '𛰳'
    LetterSTR,
    /// \u{1bc34}: '𛰴'
    LetterSP,
    /// \u{1bc35}: '𛰵'
    LetterSPR,
    /// \u{1bc36}: '𛰶'
    LetterTS,
    /// \u{1bc37}: '𛰷'
    LetterTRS,
    /// \u{1bc38}: '𛰸'
    LetterW,
    /// \u{1bc39}: '𛰹'
    LetterWh,
    /// \u{1bc3a}: '𛰺'
    LetterWR,
    /// \u{1bc3b}: '𛰻'
    LetterSN,
    /// \u{1bc3c}: '𛰼'
    LetterSM,
    /// \u{1bc3d}: '𛰽'
    LetterKRS,
    /// \u{1bc3e}: '𛰾'
    LetterGRS,
    /// \u{1bc3f}: '𛰿'
    LetterSK,
    /// \u{1bc40}: '𛱀'
    LetterSKR,
    /// \u{1bc41}: '𛱁'
    LetterA,
    /// \u{1bc42}: '𛱂'
    LetterSloanOw,
    /// \u{1bc43}: '𛱃'
    LetterOa,
    /// \u{1bc44}: '𛱄'
    LetterO,
    /// \u{1bc45}: '𛱅'
    LetterAou,
    /// \u{1bc46}: '𛱆'
    LetterI,
    /// \u{1bc47}: '𛱇'
    LetterE,
    /// \u{1bc48}: '𛱈'
    LetterIe,
    /// \u{1bc49}: '𛱉'
    LetterShortI,
    /// \u{1bc4a}: '𛱊'
    LetterUi,
    /// \u{1bc4b}: '𛱋'
    LetterEe,
    /// \u{1bc4c}: '𛱌'
    LetterSloanEh,
    /// \u{1bc4d}: '𛱍'
    LetterRomanianI,
    /// \u{1bc4e}: '𛱎'
    LetterSloanEe,
    /// \u{1bc4f}: '𛱏'
    LetterLongI,
    /// \u{1bc50}: '𛱐'
    LetterYe,
    /// \u{1bc51}: '𛱑'
    LetterU,
    /// \u{1bc52}: '𛱒'
    LetterEu,
    /// \u{1bc53}: '𛱓'
    LetterXw,
    /// \u{1bc54}: '𛱔'
    LetterUN,
    /// \u{1bc55}: '𛱕'
    LetterLongU,
    /// \u{1bc56}: '𛱖'
    LetterRomanianU,
    /// \u{1bc57}: '𛱗'
    LetterUh,
    /// \u{1bc58}: '𛱘'
    LetterSloanU,
    /// \u{1bc59}: '𛱙'
    LetterOoh,
    /// \u{1bc5a}: '𛱚'
    LetterOw,
    /// \u{1bc5b}: '𛱛'
    LetterOu,
    /// \u{1bc5c}: '𛱜'
    LetterWa,
    /// \u{1bc5d}: '𛱝'
    LetterWo,
    /// \u{1bc5e}: '𛱞'
    LetterWi,
    /// \u{1bc5f}: '𛱟'
    LetterWei,
    /// \u{1bc60}: '𛱠'
    LetterWow,
    /// \u{1bc61}: '𛱡'
    LetterNasalU,
    /// \u{1bc62}: '𛱢'
    LetterNasalO,
    /// \u{1bc63}: '𛱣'
    LetterNasalI,
    /// \u{1bc64}: '𛱤'
    LetterNasalA,
    /// \u{1bc65}: '𛱥'
    LetterPerninAn,
    /// \u{1bc66}: '𛱦'
    LetterPerninAm,
    /// \u{1bc67}: '𛱧'
    LetterSloanEn,
    /// \u{1bc68}: '𛱨'
    LetterSloanAn,
    /// \u{1bc69}: '𛱩'
    LetterSloanOn,
    /// \u{1bc6a}: '𛱪'
    LetterVocalicM,
    /// \u{1bc70}: '𛱰'
    AffixLeftHorizontalSecant,
    /// \u{1bc71}: '𛱱'
    AffixMidHorizontalSecant,
    /// \u{1bc72}: '𛱲'
    AffixRightHorizontalSecant,
    /// \u{1bc73}: '𛱳'
    AffixLowVerticalSecant,
    /// \u{1bc74}: '𛱴'
    AffixMidVerticalSecant,
    /// \u{1bc75}: '𛱵'
    AffixHighVerticalSecant,
    /// \u{1bc76}: '𛱶'
    AffixAttachedSecant,
    /// \u{1bc77}: '𛱷'
    AffixAttachedLeftDashToDashRightSecant,
    /// \u{1bc78}: '𛱸'
    AffixAttachedTangent,
    /// \u{1bc79}: '𛱹'
    AffixAttachedTail,
    /// \u{1bc7a}: '𛱺'
    AffixAttachedEHook,
    /// \u{1bc7b}: '𛱻'
    AffixAttachedIHook,
    /// \u{1bc7c}: '𛱼'
    AffixAttachedTangentHook,
    /// \u{1bc80}: '𛲀'
    AffixHighAcute,
    /// \u{1bc81}: '𛲁'
    AffixHighTightAcute,
    /// \u{1bc82}: '𛲂'
    AffixHighGrave,
    /// \u{1bc83}: '𛲃'
    AffixHighLongGrave,
    /// \u{1bc84}: '𛲄'
    AffixHighDot,
    /// \u{1bc85}: '𛲅'
    AffixHighCircle,
    /// \u{1bc86}: '𛲆'
    AffixHighLine,
    /// \u{1bc87}: '𛲇'
    AffixHighWave,
    /// \u{1bc88}: '𛲈'
    AffixHighVertical,
    /// \u{1bc90}: '𛲐'
    AffixLowAcute,
    /// \u{1bc91}: '𛲑'
    AffixLowTightAcute,
    /// \u{1bc92}: '𛲒'
    AffixLowGrave,
    /// \u{1bc93}: '𛲓'
    AffixLowLongGrave,
    /// \u{1bc94}: '𛲔'
    AffixLowDot,
    /// \u{1bc95}: '𛲕'
    AffixLowCircle,
    /// \u{1bc96}: '𛲖'
    AffixLowLine,
    /// \u{1bc97}: '𛲗'
    AffixLowWave,
    /// \u{1bc98}: '𛲘'
    AffixLowVertical,
    /// \u{1bc99}: '𛲙'
    AffixLowArrow,
    /// \u{1bc9c}: '𛲜'
    SignOWithCross,
    /// \u{1bc9d}: '𛲝'
    ThickLetterSelector,
    /// \u{1bc9e}: '𛲞'
    DoubleMark,
}

impl Into<char> for Duployan {
    fn into(self) -> char {
        match self {
            Duployan::LetterH => '𛰀',
            Duployan::LetterX => '𛰁',
            Duployan::LetterP => '𛰂',
            Duployan::LetterT => '𛰃',
            Duployan::LetterF => '𛰄',
            Duployan::LetterK => '𛰅',
            Duployan::LetterL => '𛰆',
            Duployan::LetterB => '𛰇',
            Duployan::LetterD => '𛰈',
            Duployan::LetterV => '𛰉',
            Duployan::LetterG => '𛰊',
            Duployan::LetterR => '𛰋',
            Duployan::LetterPN => '𛰌',
            Duployan::LetterDS => '𛰍',
            Duployan::LetterFN => '𛰎',
            Duployan::LetterKM => '𛰏',
            Duployan::LetterRS => '𛰐',
            Duployan::LetterTh => '𛰑',
            Duployan::LetterSloanDh => '𛰒',
            Duployan::LetterDh => '𛰓',
            Duployan::LetterKk => '𛰔',
            Duployan::LetterSloanJ => '𛰕',
            Duployan::LetterHl => '𛰖',
            Duployan::LetterLh => '𛰗',
            Duployan::LetterRh => '𛰘',
            Duployan::LetterM => '𛰙',
            Duployan::LetterN => '𛰚',
            Duployan::LetterJ => '𛰛',
            Duployan::LetterS => '𛰜',
            Duployan::LetterMN => '𛰝',
            Duployan::LetterNM => '𛰞',
            Duployan::LetterJM => '𛰟',
            Duployan::LetterSJ => '𛰠',
            Duployan::LetterMWithDot => '𛰡',
            Duployan::LetterNWithDot => '𛰢',
            Duployan::LetterJWithDot => '𛰣',
            Duployan::LetterJWithDotsInsideAndAbove => '𛰤',
            Duployan::LetterSWithDot => '𛰥',
            Duployan::LetterSWithDotBelow => '𛰦',
            Duployan::LetterMS => '𛰧',
            Duployan::LetterNS => '𛰨',
            Duployan::LetterJS => '𛰩',
            Duployan::LetterSS => '𛰪',
            Duployan::LetterMNS => '𛰫',
            Duployan::LetterNMS => '𛰬',
            Duployan::LetterJMS => '𛰭',
            Duployan::LetterSJS => '𛰮',
            Duployan::LetterJSWithDot => '𛰯',
            Duployan::LetterJN => '𛰰',
            Duployan::LetterJNS => '𛰱',
            Duployan::LetterST => '𛰲',
            Duployan::LetterSTR => '𛰳',
            Duployan::LetterSP => '𛰴',
            Duployan::LetterSPR => '𛰵',
            Duployan::LetterTS => '𛰶',
            Duployan::LetterTRS => '𛰷',
            Duployan::LetterW => '𛰸',
            Duployan::LetterWh => '𛰹',
            Duployan::LetterWR => '𛰺',
            Duployan::LetterSN => '𛰻',
            Duployan::LetterSM => '𛰼',
            Duployan::LetterKRS => '𛰽',
            Duployan::LetterGRS => '𛰾',
            Duployan::LetterSK => '𛰿',
            Duployan::LetterSKR => '𛱀',
            Duployan::LetterA => '𛱁',
            Duployan::LetterSloanOw => '𛱂',
            Duployan::LetterOa => '𛱃',
            Duployan::LetterO => '𛱄',
            Duployan::LetterAou => '𛱅',
            Duployan::LetterI => '𛱆',
            Duployan::LetterE => '𛱇',
            Duployan::LetterIe => '𛱈',
            Duployan::LetterShortI => '𛱉',
            Duployan::LetterUi => '𛱊',
            Duployan::LetterEe => '𛱋',
            Duployan::LetterSloanEh => '𛱌',
            Duployan::LetterRomanianI => '𛱍',
            Duployan::LetterSloanEe => '𛱎',
            Duployan::LetterLongI => '𛱏',
            Duployan::LetterYe => '𛱐',
            Duployan::LetterU => '𛱑',
            Duployan::LetterEu => '𛱒',
            Duployan::LetterXw => '𛱓',
            Duployan::LetterUN => '𛱔',
            Duployan::LetterLongU => '𛱕',
            Duployan::LetterRomanianU => '𛱖',
            Duployan::LetterUh => '𛱗',
            Duployan::LetterSloanU => '𛱘',
            Duployan::LetterOoh => '𛱙',
            Duployan::LetterOw => '𛱚',
            Duployan::LetterOu => '𛱛',
            Duployan::LetterWa => '𛱜',
            Duployan::LetterWo => '𛱝',
            Duployan::LetterWi => '𛱞',
            Duployan::LetterWei => '𛱟',
            Duployan::LetterWow => '𛱠',
            Duployan::LetterNasalU => '𛱡',
            Duployan::LetterNasalO => '𛱢',
            Duployan::LetterNasalI => '𛱣',
            Duployan::LetterNasalA => '𛱤',
            Duployan::LetterPerninAn => '𛱥',
            Duployan::LetterPerninAm => '𛱦',
            Duployan::LetterSloanEn => '𛱧',
            Duployan::LetterSloanAn => '𛱨',
            Duployan::LetterSloanOn => '𛱩',
            Duployan::LetterVocalicM => '𛱪',
            Duployan::AffixLeftHorizontalSecant => '𛱰',
            Duployan::AffixMidHorizontalSecant => '𛱱',
            Duployan::AffixRightHorizontalSecant => '𛱲',
            Duployan::AffixLowVerticalSecant => '𛱳',
            Duployan::AffixMidVerticalSecant => '𛱴',
            Duployan::AffixHighVerticalSecant => '𛱵',
            Duployan::AffixAttachedSecant => '𛱶',
            Duployan::AffixAttachedLeftDashToDashRightSecant => '𛱷',
            Duployan::AffixAttachedTangent => '𛱸',
            Duployan::AffixAttachedTail => '𛱹',
            Duployan::AffixAttachedEHook => '𛱺',
            Duployan::AffixAttachedIHook => '𛱻',
            Duployan::AffixAttachedTangentHook => '𛱼',
            Duployan::AffixHighAcute => '𛲀',
            Duployan::AffixHighTightAcute => '𛲁',
            Duployan::AffixHighGrave => '𛲂',
            Duployan::AffixHighLongGrave => '𛲃',
            Duployan::AffixHighDot => '𛲄',
            Duployan::AffixHighCircle => '𛲅',
            Duployan::AffixHighLine => '𛲆',
            Duployan::AffixHighWave => '𛲇',
            Duployan::AffixHighVertical => '𛲈',
            Duployan::AffixLowAcute => '𛲐',
            Duployan::AffixLowTightAcute => '𛲑',
            Duployan::AffixLowGrave => '𛲒',
            Duployan::AffixLowLongGrave => '𛲓',
            Duployan::AffixLowDot => '𛲔',
            Duployan::AffixLowCircle => '𛲕',
            Duployan::AffixLowLine => '𛲖',
            Duployan::AffixLowWave => '𛲗',
            Duployan::AffixLowVertical => '𛲘',
            Duployan::AffixLowArrow => '𛲙',
            Duployan::SignOWithCross => '𛲜',
            Duployan::ThickLetterSelector => '𛲝',
            Duployan::DoubleMark => '𛲞',
        }
    }
}

impl std::convert::TryFrom<char> for Duployan {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𛰀' => Ok(Duployan::LetterH),
            '𛰁' => Ok(Duployan::LetterX),
            '𛰂' => Ok(Duployan::LetterP),
            '𛰃' => Ok(Duployan::LetterT),
            '𛰄' => Ok(Duployan::LetterF),
            '𛰅' => Ok(Duployan::LetterK),
            '𛰆' => Ok(Duployan::LetterL),
            '𛰇' => Ok(Duployan::LetterB),
            '𛰈' => Ok(Duployan::LetterD),
            '𛰉' => Ok(Duployan::LetterV),
            '𛰊' => Ok(Duployan::LetterG),
            '𛰋' => Ok(Duployan::LetterR),
            '𛰌' => Ok(Duployan::LetterPN),
            '𛰍' => Ok(Duployan::LetterDS),
            '𛰎' => Ok(Duployan::LetterFN),
            '𛰏' => Ok(Duployan::LetterKM),
            '𛰐' => Ok(Duployan::LetterRS),
            '𛰑' => Ok(Duployan::LetterTh),
            '𛰒' => Ok(Duployan::LetterSloanDh),
            '𛰓' => Ok(Duployan::LetterDh),
            '𛰔' => Ok(Duployan::LetterKk),
            '𛰕' => Ok(Duployan::LetterSloanJ),
            '𛰖' => Ok(Duployan::LetterHl),
            '𛰗' => Ok(Duployan::LetterLh),
            '𛰘' => Ok(Duployan::LetterRh),
            '𛰙' => Ok(Duployan::LetterM),
            '𛰚' => Ok(Duployan::LetterN),
            '𛰛' => Ok(Duployan::LetterJ),
            '𛰜' => Ok(Duployan::LetterS),
            '𛰝' => Ok(Duployan::LetterMN),
            '𛰞' => Ok(Duployan::LetterNM),
            '𛰟' => Ok(Duployan::LetterJM),
            '𛰠' => Ok(Duployan::LetterSJ),
            '𛰡' => Ok(Duployan::LetterMWithDot),
            '𛰢' => Ok(Duployan::LetterNWithDot),
            '𛰣' => Ok(Duployan::LetterJWithDot),
            '𛰤' => Ok(Duployan::LetterJWithDotsInsideAndAbove),
            '𛰥' => Ok(Duployan::LetterSWithDot),
            '𛰦' => Ok(Duployan::LetterSWithDotBelow),
            '𛰧' => Ok(Duployan::LetterMS),
            '𛰨' => Ok(Duployan::LetterNS),
            '𛰩' => Ok(Duployan::LetterJS),
            '𛰪' => Ok(Duployan::LetterSS),
            '𛰫' => Ok(Duployan::LetterMNS),
            '𛰬' => Ok(Duployan::LetterNMS),
            '𛰭' => Ok(Duployan::LetterJMS),
            '𛰮' => Ok(Duployan::LetterSJS),
            '𛰯' => Ok(Duployan::LetterJSWithDot),
            '𛰰' => Ok(Duployan::LetterJN),
            '𛰱' => Ok(Duployan::LetterJNS),
            '𛰲' => Ok(Duployan::LetterST),
            '𛰳' => Ok(Duployan::LetterSTR),
            '𛰴' => Ok(Duployan::LetterSP),
            '𛰵' => Ok(Duployan::LetterSPR),
            '𛰶' => Ok(Duployan::LetterTS),
            '𛰷' => Ok(Duployan::LetterTRS),
            '𛰸' => Ok(Duployan::LetterW),
            '𛰹' => Ok(Duployan::LetterWh),
            '𛰺' => Ok(Duployan::LetterWR),
            '𛰻' => Ok(Duployan::LetterSN),
            '𛰼' => Ok(Duployan::LetterSM),
            '𛰽' => Ok(Duployan::LetterKRS),
            '𛰾' => Ok(Duployan::LetterGRS),
            '𛰿' => Ok(Duployan::LetterSK),
            '𛱀' => Ok(Duployan::LetterSKR),
            '𛱁' => Ok(Duployan::LetterA),
            '𛱂' => Ok(Duployan::LetterSloanOw),
            '𛱃' => Ok(Duployan::LetterOa),
            '𛱄' => Ok(Duployan::LetterO),
            '𛱅' => Ok(Duployan::LetterAou),
            '𛱆' => Ok(Duployan::LetterI),
            '𛱇' => Ok(Duployan::LetterE),
            '𛱈' => Ok(Duployan::LetterIe),
            '𛱉' => Ok(Duployan::LetterShortI),
            '𛱊' => Ok(Duployan::LetterUi),
            '𛱋' => Ok(Duployan::LetterEe),
            '𛱌' => Ok(Duployan::LetterSloanEh),
            '𛱍' => Ok(Duployan::LetterRomanianI),
            '𛱎' => Ok(Duployan::LetterSloanEe),
            '𛱏' => Ok(Duployan::LetterLongI),
            '𛱐' => Ok(Duployan::LetterYe),
            '𛱑' => Ok(Duployan::LetterU),
            '𛱒' => Ok(Duployan::LetterEu),
            '𛱓' => Ok(Duployan::LetterXw),
            '𛱔' => Ok(Duployan::LetterUN),
            '𛱕' => Ok(Duployan::LetterLongU),
            '𛱖' => Ok(Duployan::LetterRomanianU),
            '𛱗' => Ok(Duployan::LetterUh),
            '𛱘' => Ok(Duployan::LetterSloanU),
            '𛱙' => Ok(Duployan::LetterOoh),
            '𛱚' => Ok(Duployan::LetterOw),
            '𛱛' => Ok(Duployan::LetterOu),
            '𛱜' => Ok(Duployan::LetterWa),
            '𛱝' => Ok(Duployan::LetterWo),
            '𛱞' => Ok(Duployan::LetterWi),
            '𛱟' => Ok(Duployan::LetterWei),
            '𛱠' => Ok(Duployan::LetterWow),
            '𛱡' => Ok(Duployan::LetterNasalU),
            '𛱢' => Ok(Duployan::LetterNasalO),
            '𛱣' => Ok(Duployan::LetterNasalI),
            '𛱤' => Ok(Duployan::LetterNasalA),
            '𛱥' => Ok(Duployan::LetterPerninAn),
            '𛱦' => Ok(Duployan::LetterPerninAm),
            '𛱧' => Ok(Duployan::LetterSloanEn),
            '𛱨' => Ok(Duployan::LetterSloanAn),
            '𛱩' => Ok(Duployan::LetterSloanOn),
            '𛱪' => Ok(Duployan::LetterVocalicM),
            '𛱰' => Ok(Duployan::AffixLeftHorizontalSecant),
            '𛱱' => Ok(Duployan::AffixMidHorizontalSecant),
            '𛱲' => Ok(Duployan::AffixRightHorizontalSecant),
            '𛱳' => Ok(Duployan::AffixLowVerticalSecant),
            '𛱴' => Ok(Duployan::AffixMidVerticalSecant),
            '𛱵' => Ok(Duployan::AffixHighVerticalSecant),
            '𛱶' => Ok(Duployan::AffixAttachedSecant),
            '𛱷' => Ok(Duployan::AffixAttachedLeftDashToDashRightSecant),
            '𛱸' => Ok(Duployan::AffixAttachedTangent),
            '𛱹' => Ok(Duployan::AffixAttachedTail),
            '𛱺' => Ok(Duployan::AffixAttachedEHook),
            '𛱻' => Ok(Duployan::AffixAttachedIHook),
            '𛱼' => Ok(Duployan::AffixAttachedTangentHook),
            '𛲀' => Ok(Duployan::AffixHighAcute),
            '𛲁' => Ok(Duployan::AffixHighTightAcute),
            '𛲂' => Ok(Duployan::AffixHighGrave),
            '𛲃' => Ok(Duployan::AffixHighLongGrave),
            '𛲄' => Ok(Duployan::AffixHighDot),
            '𛲅' => Ok(Duployan::AffixHighCircle),
            '𛲆' => Ok(Duployan::AffixHighLine),
            '𛲇' => Ok(Duployan::AffixHighWave),
            '𛲈' => Ok(Duployan::AffixHighVertical),
            '𛲐' => Ok(Duployan::AffixLowAcute),
            '𛲑' => Ok(Duployan::AffixLowTightAcute),
            '𛲒' => Ok(Duployan::AffixLowGrave),
            '𛲓' => Ok(Duployan::AffixLowLongGrave),
            '𛲔' => Ok(Duployan::AffixLowDot),
            '𛲕' => Ok(Duployan::AffixLowCircle),
            '𛲖' => Ok(Duployan::AffixLowLine),
            '𛲗' => Ok(Duployan::AffixLowWave),
            '𛲘' => Ok(Duployan::AffixLowVertical),
            '𛲙' => Ok(Duployan::AffixLowArrow),
            '𛲜' => Ok(Duployan::SignOWithCross),
            '𛲝' => Ok(Duployan::ThickLetterSelector),
            '𛲞' => Ok(Duployan::DoubleMark),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Duployan {
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

impl std::convert::TryFrom<u32> for Duployan {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Duployan {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Duployan {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Duployan::LetterH
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Duployan{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
