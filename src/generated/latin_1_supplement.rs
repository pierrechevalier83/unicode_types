
/// An enum to represent all characters in the Latin1Supplement block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Latin1Supplement {
    /// \u{80}: ''
    Control0080,
    /// \u{81}: ''
    Control0081,
    /// \u{82}: ''
    Control0082,
    /// \u{83}: ''
    Control0083,
    /// \u{84}: ''
    Control0084,
    /// \u{85}: ''
    Control0085,
    /// \u{86}: ''
    Control0086,
    /// \u{87}: ''
    Control0087,
    /// \u{88}: ''
    Control0088,
    /// \u{89}: ''
    Control0089,
    /// \u{8a}: ''
    Control008a,
    /// \u{8b}: ''
    Control008b,
    /// \u{8c}: ''
    Control008c,
    /// \u{8d}: ''
    Control008d,
    /// \u{8e}: ''
    Control008e,
    /// \u{8f}: ''
    Control008f,
    /// \u{90}: ''
    Control0090,
    /// \u{91}: ''
    Control0091,
    /// \u{92}: ''
    Control0092,
    /// \u{93}: ''
    Control0093,
    /// \u{94}: ''
    Control0094,
    /// \u{95}: ''
    Control0095,
    /// \u{96}: ''
    Control0096,
    /// \u{97}: ''
    Control0097,
    /// \u{98}: ''
    Control0098,
    /// \u{99}: ''
    Control0099,
    /// \u{9a}: ''
    Control009a,
    /// \u{9b}: ''
    Control009b,
    /// \u{9c}: ''
    Control009c,
    /// \u{9d}: ''
    Control009d,
    /// \u{9e}: ''
    Control009e,
    /// \u{9f}: ''
    Control009f,
    /// \u{a0}: ' '
    NoDashBreakSpace,
    /// \u{a1}: '¡'
    InvertedExclamationMark,
    /// \u{a2}: '¢'
    CentSign,
    /// \u{a3}: '£'
    PoundSign,
    /// \u{a4}: '¤'
    CurrencySign,
    /// \u{a5}: '¥'
    YenSign,
    /// \u{a6}: '¦'
    BrokenBar,
    /// \u{a7}: '§'
    SectionSign,
    /// \u{a8}: '¨'
    Diaeresis,
    /// \u{a9}: '©'
    CopyrightSign,
    /// \u{aa}: 'ª'
    FeminineOrdinalIndicator,
    /// \u{ab}: '«'
    LeftDashPointingDoubleAngleQuotationMark,
    /// \u{ac}: '¬'
    NotSign,
    /// \u{ad}: '­'
    SoftHyphen,
    /// \u{ae}: '®'
    RegisteredSign,
    /// \u{af}: '¯'
    Macron,
    /// \u{b0}: '°'
    DegreeSign,
    /// \u{b1}: '±'
    PlusDashMinusSign,
    /// \u{b2}: '²'
    SuperscriptTwo,
    /// \u{b3}: '³'
    SuperscriptThree,
    /// \u{b4}: '´'
    AcuteAccent,
    /// \u{b5}: 'µ'
    MicroSign,
    /// \u{b6}: '¶'
    PilcrowSign,
    /// \u{b7}: '·'
    MiddleDot,
    /// \u{b8}: '¸'
    Cedilla,
    /// \u{b9}: '¹'
    SuperscriptOne,
    /// \u{ba}: 'º'
    MasculineOrdinalIndicator,
    /// \u{bb}: '»'
    RightDashPointingDoubleAngleQuotationMark,
    /// \u{bc}: '¼'
    VulgarFractionOneQuarter,
    /// \u{bd}: '½'
    VulgarFractionOneHalf,
    /// \u{be}: '¾'
    VulgarFractionThreeQuarters,
    /// \u{bf}: '¿'
    InvertedQuestionMark,
    /// \u{c0}: 'À'
    LatinCapitalLetterAWithGrave,
    /// \u{c1}: 'Á'
    LatinCapitalLetterAWithAcute,
    /// \u{c2}: 'Â'
    LatinCapitalLetterAWithCircumflex,
    /// \u{c3}: 'Ã'
    LatinCapitalLetterAWithTilde,
    /// \u{c4}: 'Ä'
    LatinCapitalLetterAWithDiaeresis,
    /// \u{c5}: 'Å'
    LatinCapitalLetterAWithRingAbove,
    /// \u{c6}: 'Æ'
    LatinCapitalLetterAe,
    /// \u{c7}: 'Ç'
    LatinCapitalLetterCWithCedilla,
    /// \u{c8}: 'È'
    LatinCapitalLetterEWithGrave,
    /// \u{c9}: 'É'
    LatinCapitalLetterEWithAcute,
    /// \u{ca}: 'Ê'
    LatinCapitalLetterEWithCircumflex,
    /// \u{cb}: 'Ë'
    LatinCapitalLetterEWithDiaeresis,
    /// \u{cc}: 'Ì'
    LatinCapitalLetterIWithGrave,
    /// \u{cd}: 'Í'
    LatinCapitalLetterIWithAcute,
    /// \u{ce}: 'Î'
    LatinCapitalLetterIWithCircumflex,
    /// \u{cf}: 'Ï'
    LatinCapitalLetterIWithDiaeresis,
    /// \u{d0}: 'Ð'
    LatinCapitalLetterEth,
    /// \u{d1}: 'Ñ'
    LatinCapitalLetterNWithTilde,
    /// \u{d2}: 'Ò'
    LatinCapitalLetterOWithGrave,
    /// \u{d3}: 'Ó'
    LatinCapitalLetterOWithAcute,
    /// \u{d4}: 'Ô'
    LatinCapitalLetterOWithCircumflex,
    /// \u{d5}: 'Õ'
    LatinCapitalLetterOWithTilde,
    /// \u{d6}: 'Ö'
    LatinCapitalLetterOWithDiaeresis,
    /// \u{d7}: '×'
    MultiplicationSign,
    /// \u{d8}: 'Ø'
    LatinCapitalLetterOWithStroke,
    /// \u{d9}: 'Ù'
    LatinCapitalLetterUWithGrave,
    /// \u{da}: 'Ú'
    LatinCapitalLetterUWithAcute,
    /// \u{db}: 'Û'
    LatinCapitalLetterUWithCircumflex,
    /// \u{dc}: 'Ü'
    LatinCapitalLetterUWithDiaeresis,
    /// \u{dd}: 'Ý'
    LatinCapitalLetterYWithAcute,
    /// \u{de}: 'Þ'
    LatinCapitalLetterThorn,
    /// \u{df}: 'ß'
    LatinSmallLetterSharpS,
    /// \u{e0}: 'à'
    LatinSmallLetterAWithGrave,
    /// \u{e1}: 'á'
    LatinSmallLetterAWithAcute,
    /// \u{e2}: 'â'
    LatinSmallLetterAWithCircumflex,
    /// \u{e3}: 'ã'
    LatinSmallLetterAWithTilde,
    /// \u{e4}: 'ä'
    LatinSmallLetterAWithDiaeresis,
    /// \u{e5}: 'å'
    LatinSmallLetterAWithRingAbove,
    /// \u{e6}: 'æ'
    LatinSmallLetterAe,
    /// \u{e7}: 'ç'
    LatinSmallLetterCWithCedilla,
    /// \u{e8}: 'è'
    LatinSmallLetterEWithGrave,
    /// \u{e9}: 'é'
    LatinSmallLetterEWithAcute,
    /// \u{ea}: 'ê'
    LatinSmallLetterEWithCircumflex,
    /// \u{eb}: 'ë'
    LatinSmallLetterEWithDiaeresis,
    /// \u{ec}: 'ì'
    LatinSmallLetterIWithGrave,
    /// \u{ed}: 'í'
    LatinSmallLetterIWithAcute,
    /// \u{ee}: 'î'
    LatinSmallLetterIWithCircumflex,
    /// \u{ef}: 'ï'
    LatinSmallLetterIWithDiaeresis,
    /// \u{f0}: 'ð'
    LatinSmallLetterEth,
    /// \u{f1}: 'ñ'
    LatinSmallLetterNWithTilde,
    /// \u{f2}: 'ò'
    LatinSmallLetterOWithGrave,
    /// \u{f3}: 'ó'
    LatinSmallLetterOWithAcute,
    /// \u{f4}: 'ô'
    LatinSmallLetterOWithCircumflex,
    /// \u{f5}: 'õ'
    LatinSmallLetterOWithTilde,
    /// \u{f6}: 'ö'
    LatinSmallLetterOWithDiaeresis,
    /// \u{f7}: '÷'
    DivisionSign,
    /// \u{f8}: 'ø'
    LatinSmallLetterOWithStroke,
    /// \u{f9}: 'ù'
    LatinSmallLetterUWithGrave,
    /// \u{fa}: 'ú'
    LatinSmallLetterUWithAcute,
    /// \u{fb}: 'û'
    LatinSmallLetterUWithCircumflex,
    /// \u{fc}: 'ü'
    LatinSmallLetterUWithDiaeresis,
    /// \u{fd}: 'ý'
    LatinSmallLetterYWithAcute,
    /// \u{fe}: 'þ'
    LatinSmallLetterThorn,
}

impl Into<char> for Latin1Supplement {
    fn into(self) -> char {
        match self {
            Latin1Supplement::Control0080 => '',
            Latin1Supplement::Control0081 => '',
            Latin1Supplement::Control0082 => '',
            Latin1Supplement::Control0083 => '',
            Latin1Supplement::Control0084 => '',
            Latin1Supplement::Control0085 => '',
            Latin1Supplement::Control0086 => '',
            Latin1Supplement::Control0087 => '',
            Latin1Supplement::Control0088 => '',
            Latin1Supplement::Control0089 => '',
            Latin1Supplement::Control008a => '',
            Latin1Supplement::Control008b => '',
            Latin1Supplement::Control008c => '',
            Latin1Supplement::Control008d => '',
            Latin1Supplement::Control008e => '',
            Latin1Supplement::Control008f => '',
            Latin1Supplement::Control0090 => '',
            Latin1Supplement::Control0091 => '',
            Latin1Supplement::Control0092 => '',
            Latin1Supplement::Control0093 => '',
            Latin1Supplement::Control0094 => '',
            Latin1Supplement::Control0095 => '',
            Latin1Supplement::Control0096 => '',
            Latin1Supplement::Control0097 => '',
            Latin1Supplement::Control0098 => '',
            Latin1Supplement::Control0099 => '',
            Latin1Supplement::Control009a => '',
            Latin1Supplement::Control009b => '',
            Latin1Supplement::Control009c => '',
            Latin1Supplement::Control009d => '',
            Latin1Supplement::Control009e => '',
            Latin1Supplement::Control009f => '',
            Latin1Supplement::NoDashBreakSpace => ' ',
            Latin1Supplement::InvertedExclamationMark => '¡',
            Latin1Supplement::CentSign => '¢',
            Latin1Supplement::PoundSign => '£',
            Latin1Supplement::CurrencySign => '¤',
            Latin1Supplement::YenSign => '¥',
            Latin1Supplement::BrokenBar => '¦',
            Latin1Supplement::SectionSign => '§',
            Latin1Supplement::Diaeresis => '¨',
            Latin1Supplement::CopyrightSign => '©',
            Latin1Supplement::FeminineOrdinalIndicator => 'ª',
            Latin1Supplement::LeftDashPointingDoubleAngleQuotationMark => '«',
            Latin1Supplement::NotSign => '¬',
            Latin1Supplement::SoftHyphen => '­',
            Latin1Supplement::RegisteredSign => '®',
            Latin1Supplement::Macron => '¯',
            Latin1Supplement::DegreeSign => '°',
            Latin1Supplement::PlusDashMinusSign => '±',
            Latin1Supplement::SuperscriptTwo => '²',
            Latin1Supplement::SuperscriptThree => '³',
            Latin1Supplement::AcuteAccent => '´',
            Latin1Supplement::MicroSign => 'µ',
            Latin1Supplement::PilcrowSign => '¶',
            Latin1Supplement::MiddleDot => '·',
            Latin1Supplement::Cedilla => '¸',
            Latin1Supplement::SuperscriptOne => '¹',
            Latin1Supplement::MasculineOrdinalIndicator => 'º',
            Latin1Supplement::RightDashPointingDoubleAngleQuotationMark => '»',
            Latin1Supplement::VulgarFractionOneQuarter => '¼',
            Latin1Supplement::VulgarFractionOneHalf => '½',
            Latin1Supplement::VulgarFractionThreeQuarters => '¾',
            Latin1Supplement::InvertedQuestionMark => '¿',
            Latin1Supplement::LatinCapitalLetterAWithGrave => 'À',
            Latin1Supplement::LatinCapitalLetterAWithAcute => 'Á',
            Latin1Supplement::LatinCapitalLetterAWithCircumflex => 'Â',
            Latin1Supplement::LatinCapitalLetterAWithTilde => 'Ã',
            Latin1Supplement::LatinCapitalLetterAWithDiaeresis => 'Ä',
            Latin1Supplement::LatinCapitalLetterAWithRingAbove => 'Å',
            Latin1Supplement::LatinCapitalLetterAe => 'Æ',
            Latin1Supplement::LatinCapitalLetterCWithCedilla => 'Ç',
            Latin1Supplement::LatinCapitalLetterEWithGrave => 'È',
            Latin1Supplement::LatinCapitalLetterEWithAcute => 'É',
            Latin1Supplement::LatinCapitalLetterEWithCircumflex => 'Ê',
            Latin1Supplement::LatinCapitalLetterEWithDiaeresis => 'Ë',
            Latin1Supplement::LatinCapitalLetterIWithGrave => 'Ì',
            Latin1Supplement::LatinCapitalLetterIWithAcute => 'Í',
            Latin1Supplement::LatinCapitalLetterIWithCircumflex => 'Î',
            Latin1Supplement::LatinCapitalLetterIWithDiaeresis => 'Ï',
            Latin1Supplement::LatinCapitalLetterEth => 'Ð',
            Latin1Supplement::LatinCapitalLetterNWithTilde => 'Ñ',
            Latin1Supplement::LatinCapitalLetterOWithGrave => 'Ò',
            Latin1Supplement::LatinCapitalLetterOWithAcute => 'Ó',
            Latin1Supplement::LatinCapitalLetterOWithCircumflex => 'Ô',
            Latin1Supplement::LatinCapitalLetterOWithTilde => 'Õ',
            Latin1Supplement::LatinCapitalLetterOWithDiaeresis => 'Ö',
            Latin1Supplement::MultiplicationSign => '×',
            Latin1Supplement::LatinCapitalLetterOWithStroke => 'Ø',
            Latin1Supplement::LatinCapitalLetterUWithGrave => 'Ù',
            Latin1Supplement::LatinCapitalLetterUWithAcute => 'Ú',
            Latin1Supplement::LatinCapitalLetterUWithCircumflex => 'Û',
            Latin1Supplement::LatinCapitalLetterUWithDiaeresis => 'Ü',
            Latin1Supplement::LatinCapitalLetterYWithAcute => 'Ý',
            Latin1Supplement::LatinCapitalLetterThorn => 'Þ',
            Latin1Supplement::LatinSmallLetterSharpS => 'ß',
            Latin1Supplement::LatinSmallLetterAWithGrave => 'à',
            Latin1Supplement::LatinSmallLetterAWithAcute => 'á',
            Latin1Supplement::LatinSmallLetterAWithCircumflex => 'â',
            Latin1Supplement::LatinSmallLetterAWithTilde => 'ã',
            Latin1Supplement::LatinSmallLetterAWithDiaeresis => 'ä',
            Latin1Supplement::LatinSmallLetterAWithRingAbove => 'å',
            Latin1Supplement::LatinSmallLetterAe => 'æ',
            Latin1Supplement::LatinSmallLetterCWithCedilla => 'ç',
            Latin1Supplement::LatinSmallLetterEWithGrave => 'è',
            Latin1Supplement::LatinSmallLetterEWithAcute => 'é',
            Latin1Supplement::LatinSmallLetterEWithCircumflex => 'ê',
            Latin1Supplement::LatinSmallLetterEWithDiaeresis => 'ë',
            Latin1Supplement::LatinSmallLetterIWithGrave => 'ì',
            Latin1Supplement::LatinSmallLetterIWithAcute => 'í',
            Latin1Supplement::LatinSmallLetterIWithCircumflex => 'î',
            Latin1Supplement::LatinSmallLetterIWithDiaeresis => 'ï',
            Latin1Supplement::LatinSmallLetterEth => 'ð',
            Latin1Supplement::LatinSmallLetterNWithTilde => 'ñ',
            Latin1Supplement::LatinSmallLetterOWithGrave => 'ò',
            Latin1Supplement::LatinSmallLetterOWithAcute => 'ó',
            Latin1Supplement::LatinSmallLetterOWithCircumflex => 'ô',
            Latin1Supplement::LatinSmallLetterOWithTilde => 'õ',
            Latin1Supplement::LatinSmallLetterOWithDiaeresis => 'ö',
            Latin1Supplement::DivisionSign => '÷',
            Latin1Supplement::LatinSmallLetterOWithStroke => 'ø',
            Latin1Supplement::LatinSmallLetterUWithGrave => 'ù',
            Latin1Supplement::LatinSmallLetterUWithAcute => 'ú',
            Latin1Supplement::LatinSmallLetterUWithCircumflex => 'û',
            Latin1Supplement::LatinSmallLetterUWithDiaeresis => 'ü',
            Latin1Supplement::LatinSmallLetterYWithAcute => 'ý',
            Latin1Supplement::LatinSmallLetterThorn => 'þ',
        }
    }
}

impl std::convert::TryFrom<char> for Latin1Supplement {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '' => Ok(Latin1Supplement::Control0080),
            '' => Ok(Latin1Supplement::Control0081),
            '' => Ok(Latin1Supplement::Control0082),
            '' => Ok(Latin1Supplement::Control0083),
            '' => Ok(Latin1Supplement::Control0084),
            '' => Ok(Latin1Supplement::Control0085),
            '' => Ok(Latin1Supplement::Control0086),
            '' => Ok(Latin1Supplement::Control0087),
            '' => Ok(Latin1Supplement::Control0088),
            '' => Ok(Latin1Supplement::Control0089),
            '' => Ok(Latin1Supplement::Control008a),
            '' => Ok(Latin1Supplement::Control008b),
            '' => Ok(Latin1Supplement::Control008c),
            '' => Ok(Latin1Supplement::Control008d),
            '' => Ok(Latin1Supplement::Control008e),
            '' => Ok(Latin1Supplement::Control008f),
            '' => Ok(Latin1Supplement::Control0090),
            '' => Ok(Latin1Supplement::Control0091),
            '' => Ok(Latin1Supplement::Control0092),
            '' => Ok(Latin1Supplement::Control0093),
            '' => Ok(Latin1Supplement::Control0094),
            '' => Ok(Latin1Supplement::Control0095),
            '' => Ok(Latin1Supplement::Control0096),
            '' => Ok(Latin1Supplement::Control0097),
            '' => Ok(Latin1Supplement::Control0098),
            '' => Ok(Latin1Supplement::Control0099),
            '' => Ok(Latin1Supplement::Control009a),
            '' => Ok(Latin1Supplement::Control009b),
            '' => Ok(Latin1Supplement::Control009c),
            '' => Ok(Latin1Supplement::Control009d),
            '' => Ok(Latin1Supplement::Control009e),
            '' => Ok(Latin1Supplement::Control009f),
            ' ' => Ok(Latin1Supplement::NoDashBreakSpace),
            '¡' => Ok(Latin1Supplement::InvertedExclamationMark),
            '¢' => Ok(Latin1Supplement::CentSign),
            '£' => Ok(Latin1Supplement::PoundSign),
            '¤' => Ok(Latin1Supplement::CurrencySign),
            '¥' => Ok(Latin1Supplement::YenSign),
            '¦' => Ok(Latin1Supplement::BrokenBar),
            '§' => Ok(Latin1Supplement::SectionSign),
            '¨' => Ok(Latin1Supplement::Diaeresis),
            '©' => Ok(Latin1Supplement::CopyrightSign),
            'ª' => Ok(Latin1Supplement::FeminineOrdinalIndicator),
            '«' => Ok(Latin1Supplement::LeftDashPointingDoubleAngleQuotationMark),
            '¬' => Ok(Latin1Supplement::NotSign),
            '­' => Ok(Latin1Supplement::SoftHyphen),
            '®' => Ok(Latin1Supplement::RegisteredSign),
            '¯' => Ok(Latin1Supplement::Macron),
            '°' => Ok(Latin1Supplement::DegreeSign),
            '±' => Ok(Latin1Supplement::PlusDashMinusSign),
            '²' => Ok(Latin1Supplement::SuperscriptTwo),
            '³' => Ok(Latin1Supplement::SuperscriptThree),
            '´' => Ok(Latin1Supplement::AcuteAccent),
            'µ' => Ok(Latin1Supplement::MicroSign),
            '¶' => Ok(Latin1Supplement::PilcrowSign),
            '·' => Ok(Latin1Supplement::MiddleDot),
            '¸' => Ok(Latin1Supplement::Cedilla),
            '¹' => Ok(Latin1Supplement::SuperscriptOne),
            'º' => Ok(Latin1Supplement::MasculineOrdinalIndicator),
            '»' => Ok(Latin1Supplement::RightDashPointingDoubleAngleQuotationMark),
            '¼' => Ok(Latin1Supplement::VulgarFractionOneQuarter),
            '½' => Ok(Latin1Supplement::VulgarFractionOneHalf),
            '¾' => Ok(Latin1Supplement::VulgarFractionThreeQuarters),
            '¿' => Ok(Latin1Supplement::InvertedQuestionMark),
            'À' => Ok(Latin1Supplement::LatinCapitalLetterAWithGrave),
            'Á' => Ok(Latin1Supplement::LatinCapitalLetterAWithAcute),
            'Â' => Ok(Latin1Supplement::LatinCapitalLetterAWithCircumflex),
            'Ã' => Ok(Latin1Supplement::LatinCapitalLetterAWithTilde),
            'Ä' => Ok(Latin1Supplement::LatinCapitalLetterAWithDiaeresis),
            'Å' => Ok(Latin1Supplement::LatinCapitalLetterAWithRingAbove),
            'Æ' => Ok(Latin1Supplement::LatinCapitalLetterAe),
            'Ç' => Ok(Latin1Supplement::LatinCapitalLetterCWithCedilla),
            'È' => Ok(Latin1Supplement::LatinCapitalLetterEWithGrave),
            'É' => Ok(Latin1Supplement::LatinCapitalLetterEWithAcute),
            'Ê' => Ok(Latin1Supplement::LatinCapitalLetterEWithCircumflex),
            'Ë' => Ok(Latin1Supplement::LatinCapitalLetterEWithDiaeresis),
            'Ì' => Ok(Latin1Supplement::LatinCapitalLetterIWithGrave),
            'Í' => Ok(Latin1Supplement::LatinCapitalLetterIWithAcute),
            'Î' => Ok(Latin1Supplement::LatinCapitalLetterIWithCircumflex),
            'Ï' => Ok(Latin1Supplement::LatinCapitalLetterIWithDiaeresis),
            'Ð' => Ok(Latin1Supplement::LatinCapitalLetterEth),
            'Ñ' => Ok(Latin1Supplement::LatinCapitalLetterNWithTilde),
            'Ò' => Ok(Latin1Supplement::LatinCapitalLetterOWithGrave),
            'Ó' => Ok(Latin1Supplement::LatinCapitalLetterOWithAcute),
            'Ô' => Ok(Latin1Supplement::LatinCapitalLetterOWithCircumflex),
            'Õ' => Ok(Latin1Supplement::LatinCapitalLetterOWithTilde),
            'Ö' => Ok(Latin1Supplement::LatinCapitalLetterOWithDiaeresis),
            '×' => Ok(Latin1Supplement::MultiplicationSign),
            'Ø' => Ok(Latin1Supplement::LatinCapitalLetterOWithStroke),
            'Ù' => Ok(Latin1Supplement::LatinCapitalLetterUWithGrave),
            'Ú' => Ok(Latin1Supplement::LatinCapitalLetterUWithAcute),
            'Û' => Ok(Latin1Supplement::LatinCapitalLetterUWithCircumflex),
            'Ü' => Ok(Latin1Supplement::LatinCapitalLetterUWithDiaeresis),
            'Ý' => Ok(Latin1Supplement::LatinCapitalLetterYWithAcute),
            'Þ' => Ok(Latin1Supplement::LatinCapitalLetterThorn),
            'ß' => Ok(Latin1Supplement::LatinSmallLetterSharpS),
            'à' => Ok(Latin1Supplement::LatinSmallLetterAWithGrave),
            'á' => Ok(Latin1Supplement::LatinSmallLetterAWithAcute),
            'â' => Ok(Latin1Supplement::LatinSmallLetterAWithCircumflex),
            'ã' => Ok(Latin1Supplement::LatinSmallLetterAWithTilde),
            'ä' => Ok(Latin1Supplement::LatinSmallLetterAWithDiaeresis),
            'å' => Ok(Latin1Supplement::LatinSmallLetterAWithRingAbove),
            'æ' => Ok(Latin1Supplement::LatinSmallLetterAe),
            'ç' => Ok(Latin1Supplement::LatinSmallLetterCWithCedilla),
            'è' => Ok(Latin1Supplement::LatinSmallLetterEWithGrave),
            'é' => Ok(Latin1Supplement::LatinSmallLetterEWithAcute),
            'ê' => Ok(Latin1Supplement::LatinSmallLetterEWithCircumflex),
            'ë' => Ok(Latin1Supplement::LatinSmallLetterEWithDiaeresis),
            'ì' => Ok(Latin1Supplement::LatinSmallLetterIWithGrave),
            'í' => Ok(Latin1Supplement::LatinSmallLetterIWithAcute),
            'î' => Ok(Latin1Supplement::LatinSmallLetterIWithCircumflex),
            'ï' => Ok(Latin1Supplement::LatinSmallLetterIWithDiaeresis),
            'ð' => Ok(Latin1Supplement::LatinSmallLetterEth),
            'ñ' => Ok(Latin1Supplement::LatinSmallLetterNWithTilde),
            'ò' => Ok(Latin1Supplement::LatinSmallLetterOWithGrave),
            'ó' => Ok(Latin1Supplement::LatinSmallLetterOWithAcute),
            'ô' => Ok(Latin1Supplement::LatinSmallLetterOWithCircumflex),
            'õ' => Ok(Latin1Supplement::LatinSmallLetterOWithTilde),
            'ö' => Ok(Latin1Supplement::LatinSmallLetterOWithDiaeresis),
            '÷' => Ok(Latin1Supplement::DivisionSign),
            'ø' => Ok(Latin1Supplement::LatinSmallLetterOWithStroke),
            'ù' => Ok(Latin1Supplement::LatinSmallLetterUWithGrave),
            'ú' => Ok(Latin1Supplement::LatinSmallLetterUWithAcute),
            'û' => Ok(Latin1Supplement::LatinSmallLetterUWithCircumflex),
            'ü' => Ok(Latin1Supplement::LatinSmallLetterUWithDiaeresis),
            'ý' => Ok(Latin1Supplement::LatinSmallLetterYWithAcute),
            'þ' => Ok(Latin1Supplement::LatinSmallLetterThorn),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Latin1Supplement {
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

impl std::convert::TryFrom<u32> for Latin1Supplement {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Latin1Supplement {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Latin1Supplement {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Latin1Supplement::Control0080
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Latin1Supplement{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
