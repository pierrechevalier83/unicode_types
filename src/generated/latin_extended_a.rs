
/// An enum to represent all characters in the LatinExtendedA block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum LatinExtendedA {
    /// \u{100}: 'Ā'
    LatinCapitalLetterAWithMacron,
    /// \u{101}: 'ā'
    LatinSmallLetterAWithMacron,
    /// \u{102}: 'Ă'
    LatinCapitalLetterAWithBreve,
    /// \u{103}: 'ă'
    LatinSmallLetterAWithBreve,
    /// \u{104}: 'Ą'
    LatinCapitalLetterAWithOgonek,
    /// \u{105}: 'ą'
    LatinSmallLetterAWithOgonek,
    /// \u{106}: 'Ć'
    LatinCapitalLetterCWithAcute,
    /// \u{107}: 'ć'
    LatinSmallLetterCWithAcute,
    /// \u{108}: 'Ĉ'
    LatinCapitalLetterCWithCircumflex,
    /// \u{109}: 'ĉ'
    LatinSmallLetterCWithCircumflex,
    /// \u{10a}: 'Ċ'
    LatinCapitalLetterCWithDotAbove,
    /// \u{10b}: 'ċ'
    LatinSmallLetterCWithDotAbove,
    /// \u{10c}: 'Č'
    LatinCapitalLetterCWithCaron,
    /// \u{10d}: 'č'
    LatinSmallLetterCWithCaron,
    /// \u{10e}: 'Ď'
    LatinCapitalLetterDWithCaron,
    /// \u{10f}: 'ď'
    LatinSmallLetterDWithCaron,
    /// \u{110}: 'Đ'
    LatinCapitalLetterDWithStroke,
    /// \u{111}: 'đ'
    LatinSmallLetterDWithStroke,
    /// \u{112}: 'Ē'
    LatinCapitalLetterEWithMacron,
    /// \u{113}: 'ē'
    LatinSmallLetterEWithMacron,
    /// \u{114}: 'Ĕ'
    LatinCapitalLetterEWithBreve,
    /// \u{115}: 'ĕ'
    LatinSmallLetterEWithBreve,
    /// \u{116}: 'Ė'
    LatinCapitalLetterEWithDotAbove,
    /// \u{117}: 'ė'
    LatinSmallLetterEWithDotAbove,
    /// \u{118}: 'Ę'
    LatinCapitalLetterEWithOgonek,
    /// \u{119}: 'ę'
    LatinSmallLetterEWithOgonek,
    /// \u{11a}: 'Ě'
    LatinCapitalLetterEWithCaron,
    /// \u{11b}: 'ě'
    LatinSmallLetterEWithCaron,
    /// \u{11c}: 'Ĝ'
    LatinCapitalLetterGWithCircumflex,
    /// \u{11d}: 'ĝ'
    LatinSmallLetterGWithCircumflex,
    /// \u{11e}: 'Ğ'
    LatinCapitalLetterGWithBreve,
    /// \u{11f}: 'ğ'
    LatinSmallLetterGWithBreve,
    /// \u{120}: 'Ġ'
    LatinCapitalLetterGWithDotAbove,
    /// \u{121}: 'ġ'
    LatinSmallLetterGWithDotAbove,
    /// \u{122}: 'Ģ'
    LatinCapitalLetterGWithCedilla,
    /// \u{123}: 'ģ'
    LatinSmallLetterGWithCedilla,
    /// \u{124}: 'Ĥ'
    LatinCapitalLetterHWithCircumflex,
    /// \u{125}: 'ĥ'
    LatinSmallLetterHWithCircumflex,
    /// \u{126}: 'Ħ'
    LatinCapitalLetterHWithStroke,
    /// \u{127}: 'ħ'
    LatinSmallLetterHWithStroke,
    /// \u{128}: 'Ĩ'
    LatinCapitalLetterIWithTilde,
    /// \u{129}: 'ĩ'
    LatinSmallLetterIWithTilde,
    /// \u{12a}: 'Ī'
    LatinCapitalLetterIWithMacron,
    /// \u{12b}: 'ī'
    LatinSmallLetterIWithMacron,
    /// \u{12c}: 'Ĭ'
    LatinCapitalLetterIWithBreve,
    /// \u{12d}: 'ĭ'
    LatinSmallLetterIWithBreve,
    /// \u{12e}: 'Į'
    LatinCapitalLetterIWithOgonek,
    /// \u{12f}: 'į'
    LatinSmallLetterIWithOgonek,
    /// \u{130}: 'İ'
    LatinCapitalLetterIWithDotAbove,
    /// \u{131}: 'ı'
    LatinSmallLetterDotlessI,
    /// \u{132}: 'Ĳ'
    LatinCapitalLigatureIj,
    /// \u{133}: 'ĳ'
    LatinSmallLigatureIj,
    /// \u{134}: 'Ĵ'
    LatinCapitalLetterJWithCircumflex,
    /// \u{135}: 'ĵ'
    LatinSmallLetterJWithCircumflex,
    /// \u{136}: 'Ķ'
    LatinCapitalLetterKWithCedilla,
    /// \u{137}: 'ķ'
    LatinSmallLetterKWithCedilla,
    /// \u{138}: 'ĸ'
    LatinSmallLetterKra,
    /// \u{139}: 'Ĺ'
    LatinCapitalLetterLWithAcute,
    /// \u{13a}: 'ĺ'
    LatinSmallLetterLWithAcute,
    /// \u{13b}: 'Ļ'
    LatinCapitalLetterLWithCedilla,
    /// \u{13c}: 'ļ'
    LatinSmallLetterLWithCedilla,
    /// \u{13d}: 'Ľ'
    LatinCapitalLetterLWithCaron,
    /// \u{13e}: 'ľ'
    LatinSmallLetterLWithCaron,
    /// \u{13f}: 'Ŀ'
    LatinCapitalLetterLWithMiddleDot,
    /// \u{140}: 'ŀ'
    LatinSmallLetterLWithMiddleDot,
    /// \u{141}: 'Ł'
    LatinCapitalLetterLWithStroke,
    /// \u{142}: 'ł'
    LatinSmallLetterLWithStroke,
    /// \u{143}: 'Ń'
    LatinCapitalLetterNWithAcute,
    /// \u{144}: 'ń'
    LatinSmallLetterNWithAcute,
    /// \u{145}: 'Ņ'
    LatinCapitalLetterNWithCedilla,
    /// \u{146}: 'ņ'
    LatinSmallLetterNWithCedilla,
    /// \u{147}: 'Ň'
    LatinCapitalLetterNWithCaron,
    /// \u{148}: 'ň'
    LatinSmallLetterNWithCaron,
    /// \u{149}: 'ŉ'
    LatinSmallLetterNPrecededByApostrophe,
    /// \u{14a}: 'Ŋ'
    LatinCapitalLetterEng,
    /// \u{14b}: 'ŋ'
    LatinSmallLetterEng,
    /// \u{14c}: 'Ō'
    LatinCapitalLetterOWithMacron,
    /// \u{14d}: 'ō'
    LatinSmallLetterOWithMacron,
    /// \u{14e}: 'Ŏ'
    LatinCapitalLetterOWithBreve,
    /// \u{14f}: 'ŏ'
    LatinSmallLetterOWithBreve,
    /// \u{150}: 'Ő'
    LatinCapitalLetterOWithDoubleAcute,
    /// \u{151}: 'ő'
    LatinSmallLetterOWithDoubleAcute,
    /// \u{152}: 'Œ'
    LatinCapitalLigatureOe,
    /// \u{153}: 'œ'
    LatinSmallLigatureOe,
    /// \u{154}: 'Ŕ'
    LatinCapitalLetterRWithAcute,
    /// \u{155}: 'ŕ'
    LatinSmallLetterRWithAcute,
    /// \u{156}: 'Ŗ'
    LatinCapitalLetterRWithCedilla,
    /// \u{157}: 'ŗ'
    LatinSmallLetterRWithCedilla,
    /// \u{158}: 'Ř'
    LatinCapitalLetterRWithCaron,
    /// \u{159}: 'ř'
    LatinSmallLetterRWithCaron,
    /// \u{15a}: 'Ś'
    LatinCapitalLetterSWithAcute,
    /// \u{15b}: 'ś'
    LatinSmallLetterSWithAcute,
    /// \u{15c}: 'Ŝ'
    LatinCapitalLetterSWithCircumflex,
    /// \u{15d}: 'ŝ'
    LatinSmallLetterSWithCircumflex,
    /// \u{15e}: 'Ş'
    LatinCapitalLetterSWithCedilla,
    /// \u{15f}: 'ş'
    LatinSmallLetterSWithCedilla,
    /// \u{160}: 'Š'
    LatinCapitalLetterSWithCaron,
    /// \u{161}: 'š'
    LatinSmallLetterSWithCaron,
    /// \u{162}: 'Ţ'
    LatinCapitalLetterTWithCedilla,
    /// \u{163}: 'ţ'
    LatinSmallLetterTWithCedilla,
    /// \u{164}: 'Ť'
    LatinCapitalLetterTWithCaron,
    /// \u{165}: 'ť'
    LatinSmallLetterTWithCaron,
    /// \u{166}: 'Ŧ'
    LatinCapitalLetterTWithStroke,
    /// \u{167}: 'ŧ'
    LatinSmallLetterTWithStroke,
    /// \u{168}: 'Ũ'
    LatinCapitalLetterUWithTilde,
    /// \u{169}: 'ũ'
    LatinSmallLetterUWithTilde,
    /// \u{16a}: 'Ū'
    LatinCapitalLetterUWithMacron,
    /// \u{16b}: 'ū'
    LatinSmallLetterUWithMacron,
    /// \u{16c}: 'Ŭ'
    LatinCapitalLetterUWithBreve,
    /// \u{16d}: 'ŭ'
    LatinSmallLetterUWithBreve,
    /// \u{16e}: 'Ů'
    LatinCapitalLetterUWithRingAbove,
    /// \u{16f}: 'ů'
    LatinSmallLetterUWithRingAbove,
    /// \u{170}: 'Ű'
    LatinCapitalLetterUWithDoubleAcute,
    /// \u{171}: 'ű'
    LatinSmallLetterUWithDoubleAcute,
    /// \u{172}: 'Ų'
    LatinCapitalLetterUWithOgonek,
    /// \u{173}: 'ų'
    LatinSmallLetterUWithOgonek,
    /// \u{174}: 'Ŵ'
    LatinCapitalLetterWWithCircumflex,
    /// \u{175}: 'ŵ'
    LatinSmallLetterWWithCircumflex,
    /// \u{176}: 'Ŷ'
    LatinCapitalLetterYWithCircumflex,
    /// \u{177}: 'ŷ'
    LatinSmallLetterYWithCircumflex,
    /// \u{178}: 'Ÿ'
    LatinCapitalLetterYWithDiaeresis,
    /// \u{179}: 'Ź'
    LatinCapitalLetterZWithAcute,
    /// \u{17a}: 'ź'
    LatinSmallLetterZWithAcute,
    /// \u{17b}: 'Ż'
    LatinCapitalLetterZWithDotAbove,
    /// \u{17c}: 'ż'
    LatinSmallLetterZWithDotAbove,
    /// \u{17d}: 'Ž'
    LatinCapitalLetterZWithCaron,
    /// \u{17e}: 'ž'
    LatinSmallLetterZWithCaron,
}

impl Into<char> for LatinExtendedA {
    fn into(self) -> char {
        match self {
            LatinExtendedA::LatinCapitalLetterAWithMacron => 'Ā',
            LatinExtendedA::LatinSmallLetterAWithMacron => 'ā',
            LatinExtendedA::LatinCapitalLetterAWithBreve => 'Ă',
            LatinExtendedA::LatinSmallLetterAWithBreve => 'ă',
            LatinExtendedA::LatinCapitalLetterAWithOgonek => 'Ą',
            LatinExtendedA::LatinSmallLetterAWithOgonek => 'ą',
            LatinExtendedA::LatinCapitalLetterCWithAcute => 'Ć',
            LatinExtendedA::LatinSmallLetterCWithAcute => 'ć',
            LatinExtendedA::LatinCapitalLetterCWithCircumflex => 'Ĉ',
            LatinExtendedA::LatinSmallLetterCWithCircumflex => 'ĉ',
            LatinExtendedA::LatinCapitalLetterCWithDotAbove => 'Ċ',
            LatinExtendedA::LatinSmallLetterCWithDotAbove => 'ċ',
            LatinExtendedA::LatinCapitalLetterCWithCaron => 'Č',
            LatinExtendedA::LatinSmallLetterCWithCaron => 'č',
            LatinExtendedA::LatinCapitalLetterDWithCaron => 'Ď',
            LatinExtendedA::LatinSmallLetterDWithCaron => 'ď',
            LatinExtendedA::LatinCapitalLetterDWithStroke => 'Đ',
            LatinExtendedA::LatinSmallLetterDWithStroke => 'đ',
            LatinExtendedA::LatinCapitalLetterEWithMacron => 'Ē',
            LatinExtendedA::LatinSmallLetterEWithMacron => 'ē',
            LatinExtendedA::LatinCapitalLetterEWithBreve => 'Ĕ',
            LatinExtendedA::LatinSmallLetterEWithBreve => 'ĕ',
            LatinExtendedA::LatinCapitalLetterEWithDotAbove => 'Ė',
            LatinExtendedA::LatinSmallLetterEWithDotAbove => 'ė',
            LatinExtendedA::LatinCapitalLetterEWithOgonek => 'Ę',
            LatinExtendedA::LatinSmallLetterEWithOgonek => 'ę',
            LatinExtendedA::LatinCapitalLetterEWithCaron => 'Ě',
            LatinExtendedA::LatinSmallLetterEWithCaron => 'ě',
            LatinExtendedA::LatinCapitalLetterGWithCircumflex => 'Ĝ',
            LatinExtendedA::LatinSmallLetterGWithCircumflex => 'ĝ',
            LatinExtendedA::LatinCapitalLetterGWithBreve => 'Ğ',
            LatinExtendedA::LatinSmallLetterGWithBreve => 'ğ',
            LatinExtendedA::LatinCapitalLetterGWithDotAbove => 'Ġ',
            LatinExtendedA::LatinSmallLetterGWithDotAbove => 'ġ',
            LatinExtendedA::LatinCapitalLetterGWithCedilla => 'Ģ',
            LatinExtendedA::LatinSmallLetterGWithCedilla => 'ģ',
            LatinExtendedA::LatinCapitalLetterHWithCircumflex => 'Ĥ',
            LatinExtendedA::LatinSmallLetterHWithCircumflex => 'ĥ',
            LatinExtendedA::LatinCapitalLetterHWithStroke => 'Ħ',
            LatinExtendedA::LatinSmallLetterHWithStroke => 'ħ',
            LatinExtendedA::LatinCapitalLetterIWithTilde => 'Ĩ',
            LatinExtendedA::LatinSmallLetterIWithTilde => 'ĩ',
            LatinExtendedA::LatinCapitalLetterIWithMacron => 'Ī',
            LatinExtendedA::LatinSmallLetterIWithMacron => 'ī',
            LatinExtendedA::LatinCapitalLetterIWithBreve => 'Ĭ',
            LatinExtendedA::LatinSmallLetterIWithBreve => 'ĭ',
            LatinExtendedA::LatinCapitalLetterIWithOgonek => 'Į',
            LatinExtendedA::LatinSmallLetterIWithOgonek => 'į',
            LatinExtendedA::LatinCapitalLetterIWithDotAbove => 'İ',
            LatinExtendedA::LatinSmallLetterDotlessI => 'ı',
            LatinExtendedA::LatinCapitalLigatureIj => 'Ĳ',
            LatinExtendedA::LatinSmallLigatureIj => 'ĳ',
            LatinExtendedA::LatinCapitalLetterJWithCircumflex => 'Ĵ',
            LatinExtendedA::LatinSmallLetterJWithCircumflex => 'ĵ',
            LatinExtendedA::LatinCapitalLetterKWithCedilla => 'Ķ',
            LatinExtendedA::LatinSmallLetterKWithCedilla => 'ķ',
            LatinExtendedA::LatinSmallLetterKra => 'ĸ',
            LatinExtendedA::LatinCapitalLetterLWithAcute => 'Ĺ',
            LatinExtendedA::LatinSmallLetterLWithAcute => 'ĺ',
            LatinExtendedA::LatinCapitalLetterLWithCedilla => 'Ļ',
            LatinExtendedA::LatinSmallLetterLWithCedilla => 'ļ',
            LatinExtendedA::LatinCapitalLetterLWithCaron => 'Ľ',
            LatinExtendedA::LatinSmallLetterLWithCaron => 'ľ',
            LatinExtendedA::LatinCapitalLetterLWithMiddleDot => 'Ŀ',
            LatinExtendedA::LatinSmallLetterLWithMiddleDot => 'ŀ',
            LatinExtendedA::LatinCapitalLetterLWithStroke => 'Ł',
            LatinExtendedA::LatinSmallLetterLWithStroke => 'ł',
            LatinExtendedA::LatinCapitalLetterNWithAcute => 'Ń',
            LatinExtendedA::LatinSmallLetterNWithAcute => 'ń',
            LatinExtendedA::LatinCapitalLetterNWithCedilla => 'Ņ',
            LatinExtendedA::LatinSmallLetterNWithCedilla => 'ņ',
            LatinExtendedA::LatinCapitalLetterNWithCaron => 'Ň',
            LatinExtendedA::LatinSmallLetterNWithCaron => 'ň',
            LatinExtendedA::LatinSmallLetterNPrecededByApostrophe => 'ŉ',
            LatinExtendedA::LatinCapitalLetterEng => 'Ŋ',
            LatinExtendedA::LatinSmallLetterEng => 'ŋ',
            LatinExtendedA::LatinCapitalLetterOWithMacron => 'Ō',
            LatinExtendedA::LatinSmallLetterOWithMacron => 'ō',
            LatinExtendedA::LatinCapitalLetterOWithBreve => 'Ŏ',
            LatinExtendedA::LatinSmallLetterOWithBreve => 'ŏ',
            LatinExtendedA::LatinCapitalLetterOWithDoubleAcute => 'Ő',
            LatinExtendedA::LatinSmallLetterOWithDoubleAcute => 'ő',
            LatinExtendedA::LatinCapitalLigatureOe => 'Œ',
            LatinExtendedA::LatinSmallLigatureOe => 'œ',
            LatinExtendedA::LatinCapitalLetterRWithAcute => 'Ŕ',
            LatinExtendedA::LatinSmallLetterRWithAcute => 'ŕ',
            LatinExtendedA::LatinCapitalLetterRWithCedilla => 'Ŗ',
            LatinExtendedA::LatinSmallLetterRWithCedilla => 'ŗ',
            LatinExtendedA::LatinCapitalLetterRWithCaron => 'Ř',
            LatinExtendedA::LatinSmallLetterRWithCaron => 'ř',
            LatinExtendedA::LatinCapitalLetterSWithAcute => 'Ś',
            LatinExtendedA::LatinSmallLetterSWithAcute => 'ś',
            LatinExtendedA::LatinCapitalLetterSWithCircumflex => 'Ŝ',
            LatinExtendedA::LatinSmallLetterSWithCircumflex => 'ŝ',
            LatinExtendedA::LatinCapitalLetterSWithCedilla => 'Ş',
            LatinExtendedA::LatinSmallLetterSWithCedilla => 'ş',
            LatinExtendedA::LatinCapitalLetterSWithCaron => 'Š',
            LatinExtendedA::LatinSmallLetterSWithCaron => 'š',
            LatinExtendedA::LatinCapitalLetterTWithCedilla => 'Ţ',
            LatinExtendedA::LatinSmallLetterTWithCedilla => 'ţ',
            LatinExtendedA::LatinCapitalLetterTWithCaron => 'Ť',
            LatinExtendedA::LatinSmallLetterTWithCaron => 'ť',
            LatinExtendedA::LatinCapitalLetterTWithStroke => 'Ŧ',
            LatinExtendedA::LatinSmallLetterTWithStroke => 'ŧ',
            LatinExtendedA::LatinCapitalLetterUWithTilde => 'Ũ',
            LatinExtendedA::LatinSmallLetterUWithTilde => 'ũ',
            LatinExtendedA::LatinCapitalLetterUWithMacron => 'Ū',
            LatinExtendedA::LatinSmallLetterUWithMacron => 'ū',
            LatinExtendedA::LatinCapitalLetterUWithBreve => 'Ŭ',
            LatinExtendedA::LatinSmallLetterUWithBreve => 'ŭ',
            LatinExtendedA::LatinCapitalLetterUWithRingAbove => 'Ů',
            LatinExtendedA::LatinSmallLetterUWithRingAbove => 'ů',
            LatinExtendedA::LatinCapitalLetterUWithDoubleAcute => 'Ű',
            LatinExtendedA::LatinSmallLetterUWithDoubleAcute => 'ű',
            LatinExtendedA::LatinCapitalLetterUWithOgonek => 'Ų',
            LatinExtendedA::LatinSmallLetterUWithOgonek => 'ų',
            LatinExtendedA::LatinCapitalLetterWWithCircumflex => 'Ŵ',
            LatinExtendedA::LatinSmallLetterWWithCircumflex => 'ŵ',
            LatinExtendedA::LatinCapitalLetterYWithCircumflex => 'Ŷ',
            LatinExtendedA::LatinSmallLetterYWithCircumflex => 'ŷ',
            LatinExtendedA::LatinCapitalLetterYWithDiaeresis => 'Ÿ',
            LatinExtendedA::LatinCapitalLetterZWithAcute => 'Ź',
            LatinExtendedA::LatinSmallLetterZWithAcute => 'ź',
            LatinExtendedA::LatinCapitalLetterZWithDotAbove => 'Ż',
            LatinExtendedA::LatinSmallLetterZWithDotAbove => 'ż',
            LatinExtendedA::LatinCapitalLetterZWithCaron => 'Ž',
            LatinExtendedA::LatinSmallLetterZWithCaron => 'ž',
        }
    }
}

impl std::convert::TryFrom<char> for LatinExtendedA {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'Ā' => Ok(LatinExtendedA::LatinCapitalLetterAWithMacron),
            'ā' => Ok(LatinExtendedA::LatinSmallLetterAWithMacron),
            'Ă' => Ok(LatinExtendedA::LatinCapitalLetterAWithBreve),
            'ă' => Ok(LatinExtendedA::LatinSmallLetterAWithBreve),
            'Ą' => Ok(LatinExtendedA::LatinCapitalLetterAWithOgonek),
            'ą' => Ok(LatinExtendedA::LatinSmallLetterAWithOgonek),
            'Ć' => Ok(LatinExtendedA::LatinCapitalLetterCWithAcute),
            'ć' => Ok(LatinExtendedA::LatinSmallLetterCWithAcute),
            'Ĉ' => Ok(LatinExtendedA::LatinCapitalLetterCWithCircumflex),
            'ĉ' => Ok(LatinExtendedA::LatinSmallLetterCWithCircumflex),
            'Ċ' => Ok(LatinExtendedA::LatinCapitalLetterCWithDotAbove),
            'ċ' => Ok(LatinExtendedA::LatinSmallLetterCWithDotAbove),
            'Č' => Ok(LatinExtendedA::LatinCapitalLetterCWithCaron),
            'č' => Ok(LatinExtendedA::LatinSmallLetterCWithCaron),
            'Ď' => Ok(LatinExtendedA::LatinCapitalLetterDWithCaron),
            'ď' => Ok(LatinExtendedA::LatinSmallLetterDWithCaron),
            'Đ' => Ok(LatinExtendedA::LatinCapitalLetterDWithStroke),
            'đ' => Ok(LatinExtendedA::LatinSmallLetterDWithStroke),
            'Ē' => Ok(LatinExtendedA::LatinCapitalLetterEWithMacron),
            'ē' => Ok(LatinExtendedA::LatinSmallLetterEWithMacron),
            'Ĕ' => Ok(LatinExtendedA::LatinCapitalLetterEWithBreve),
            'ĕ' => Ok(LatinExtendedA::LatinSmallLetterEWithBreve),
            'Ė' => Ok(LatinExtendedA::LatinCapitalLetterEWithDotAbove),
            'ė' => Ok(LatinExtendedA::LatinSmallLetterEWithDotAbove),
            'Ę' => Ok(LatinExtendedA::LatinCapitalLetterEWithOgonek),
            'ę' => Ok(LatinExtendedA::LatinSmallLetterEWithOgonek),
            'Ě' => Ok(LatinExtendedA::LatinCapitalLetterEWithCaron),
            'ě' => Ok(LatinExtendedA::LatinSmallLetterEWithCaron),
            'Ĝ' => Ok(LatinExtendedA::LatinCapitalLetterGWithCircumflex),
            'ĝ' => Ok(LatinExtendedA::LatinSmallLetterGWithCircumflex),
            'Ğ' => Ok(LatinExtendedA::LatinCapitalLetterGWithBreve),
            'ğ' => Ok(LatinExtendedA::LatinSmallLetterGWithBreve),
            'Ġ' => Ok(LatinExtendedA::LatinCapitalLetterGWithDotAbove),
            'ġ' => Ok(LatinExtendedA::LatinSmallLetterGWithDotAbove),
            'Ģ' => Ok(LatinExtendedA::LatinCapitalLetterGWithCedilla),
            'ģ' => Ok(LatinExtendedA::LatinSmallLetterGWithCedilla),
            'Ĥ' => Ok(LatinExtendedA::LatinCapitalLetterHWithCircumflex),
            'ĥ' => Ok(LatinExtendedA::LatinSmallLetterHWithCircumflex),
            'Ħ' => Ok(LatinExtendedA::LatinCapitalLetterHWithStroke),
            'ħ' => Ok(LatinExtendedA::LatinSmallLetterHWithStroke),
            'Ĩ' => Ok(LatinExtendedA::LatinCapitalLetterIWithTilde),
            'ĩ' => Ok(LatinExtendedA::LatinSmallLetterIWithTilde),
            'Ī' => Ok(LatinExtendedA::LatinCapitalLetterIWithMacron),
            'ī' => Ok(LatinExtendedA::LatinSmallLetterIWithMacron),
            'Ĭ' => Ok(LatinExtendedA::LatinCapitalLetterIWithBreve),
            'ĭ' => Ok(LatinExtendedA::LatinSmallLetterIWithBreve),
            'Į' => Ok(LatinExtendedA::LatinCapitalLetterIWithOgonek),
            'į' => Ok(LatinExtendedA::LatinSmallLetterIWithOgonek),
            'İ' => Ok(LatinExtendedA::LatinCapitalLetterIWithDotAbove),
            'ı' => Ok(LatinExtendedA::LatinSmallLetterDotlessI),
            'Ĳ' => Ok(LatinExtendedA::LatinCapitalLigatureIj),
            'ĳ' => Ok(LatinExtendedA::LatinSmallLigatureIj),
            'Ĵ' => Ok(LatinExtendedA::LatinCapitalLetterJWithCircumflex),
            'ĵ' => Ok(LatinExtendedA::LatinSmallLetterJWithCircumflex),
            'Ķ' => Ok(LatinExtendedA::LatinCapitalLetterKWithCedilla),
            'ķ' => Ok(LatinExtendedA::LatinSmallLetterKWithCedilla),
            'ĸ' => Ok(LatinExtendedA::LatinSmallLetterKra),
            'Ĺ' => Ok(LatinExtendedA::LatinCapitalLetterLWithAcute),
            'ĺ' => Ok(LatinExtendedA::LatinSmallLetterLWithAcute),
            'Ļ' => Ok(LatinExtendedA::LatinCapitalLetterLWithCedilla),
            'ļ' => Ok(LatinExtendedA::LatinSmallLetterLWithCedilla),
            'Ľ' => Ok(LatinExtendedA::LatinCapitalLetterLWithCaron),
            'ľ' => Ok(LatinExtendedA::LatinSmallLetterLWithCaron),
            'Ŀ' => Ok(LatinExtendedA::LatinCapitalLetterLWithMiddleDot),
            'ŀ' => Ok(LatinExtendedA::LatinSmallLetterLWithMiddleDot),
            'Ł' => Ok(LatinExtendedA::LatinCapitalLetterLWithStroke),
            'ł' => Ok(LatinExtendedA::LatinSmallLetterLWithStroke),
            'Ń' => Ok(LatinExtendedA::LatinCapitalLetterNWithAcute),
            'ń' => Ok(LatinExtendedA::LatinSmallLetterNWithAcute),
            'Ņ' => Ok(LatinExtendedA::LatinCapitalLetterNWithCedilla),
            'ņ' => Ok(LatinExtendedA::LatinSmallLetterNWithCedilla),
            'Ň' => Ok(LatinExtendedA::LatinCapitalLetterNWithCaron),
            'ň' => Ok(LatinExtendedA::LatinSmallLetterNWithCaron),
            'ŉ' => Ok(LatinExtendedA::LatinSmallLetterNPrecededByApostrophe),
            'Ŋ' => Ok(LatinExtendedA::LatinCapitalLetterEng),
            'ŋ' => Ok(LatinExtendedA::LatinSmallLetterEng),
            'Ō' => Ok(LatinExtendedA::LatinCapitalLetterOWithMacron),
            'ō' => Ok(LatinExtendedA::LatinSmallLetterOWithMacron),
            'Ŏ' => Ok(LatinExtendedA::LatinCapitalLetterOWithBreve),
            'ŏ' => Ok(LatinExtendedA::LatinSmallLetterOWithBreve),
            'Ő' => Ok(LatinExtendedA::LatinCapitalLetterOWithDoubleAcute),
            'ő' => Ok(LatinExtendedA::LatinSmallLetterOWithDoubleAcute),
            'Œ' => Ok(LatinExtendedA::LatinCapitalLigatureOe),
            'œ' => Ok(LatinExtendedA::LatinSmallLigatureOe),
            'Ŕ' => Ok(LatinExtendedA::LatinCapitalLetterRWithAcute),
            'ŕ' => Ok(LatinExtendedA::LatinSmallLetterRWithAcute),
            'Ŗ' => Ok(LatinExtendedA::LatinCapitalLetterRWithCedilla),
            'ŗ' => Ok(LatinExtendedA::LatinSmallLetterRWithCedilla),
            'Ř' => Ok(LatinExtendedA::LatinCapitalLetterRWithCaron),
            'ř' => Ok(LatinExtendedA::LatinSmallLetterRWithCaron),
            'Ś' => Ok(LatinExtendedA::LatinCapitalLetterSWithAcute),
            'ś' => Ok(LatinExtendedA::LatinSmallLetterSWithAcute),
            'Ŝ' => Ok(LatinExtendedA::LatinCapitalLetterSWithCircumflex),
            'ŝ' => Ok(LatinExtendedA::LatinSmallLetterSWithCircumflex),
            'Ş' => Ok(LatinExtendedA::LatinCapitalLetterSWithCedilla),
            'ş' => Ok(LatinExtendedA::LatinSmallLetterSWithCedilla),
            'Š' => Ok(LatinExtendedA::LatinCapitalLetterSWithCaron),
            'š' => Ok(LatinExtendedA::LatinSmallLetterSWithCaron),
            'Ţ' => Ok(LatinExtendedA::LatinCapitalLetterTWithCedilla),
            'ţ' => Ok(LatinExtendedA::LatinSmallLetterTWithCedilla),
            'Ť' => Ok(LatinExtendedA::LatinCapitalLetterTWithCaron),
            'ť' => Ok(LatinExtendedA::LatinSmallLetterTWithCaron),
            'Ŧ' => Ok(LatinExtendedA::LatinCapitalLetterTWithStroke),
            'ŧ' => Ok(LatinExtendedA::LatinSmallLetterTWithStroke),
            'Ũ' => Ok(LatinExtendedA::LatinCapitalLetterUWithTilde),
            'ũ' => Ok(LatinExtendedA::LatinSmallLetterUWithTilde),
            'Ū' => Ok(LatinExtendedA::LatinCapitalLetterUWithMacron),
            'ū' => Ok(LatinExtendedA::LatinSmallLetterUWithMacron),
            'Ŭ' => Ok(LatinExtendedA::LatinCapitalLetterUWithBreve),
            'ŭ' => Ok(LatinExtendedA::LatinSmallLetterUWithBreve),
            'Ů' => Ok(LatinExtendedA::LatinCapitalLetterUWithRingAbove),
            'ů' => Ok(LatinExtendedA::LatinSmallLetterUWithRingAbove),
            'Ű' => Ok(LatinExtendedA::LatinCapitalLetterUWithDoubleAcute),
            'ű' => Ok(LatinExtendedA::LatinSmallLetterUWithDoubleAcute),
            'Ų' => Ok(LatinExtendedA::LatinCapitalLetterUWithOgonek),
            'ų' => Ok(LatinExtendedA::LatinSmallLetterUWithOgonek),
            'Ŵ' => Ok(LatinExtendedA::LatinCapitalLetterWWithCircumflex),
            'ŵ' => Ok(LatinExtendedA::LatinSmallLetterWWithCircumflex),
            'Ŷ' => Ok(LatinExtendedA::LatinCapitalLetterYWithCircumflex),
            'ŷ' => Ok(LatinExtendedA::LatinSmallLetterYWithCircumflex),
            'Ÿ' => Ok(LatinExtendedA::LatinCapitalLetterYWithDiaeresis),
            'Ź' => Ok(LatinExtendedA::LatinCapitalLetterZWithAcute),
            'ź' => Ok(LatinExtendedA::LatinSmallLetterZWithAcute),
            'Ż' => Ok(LatinExtendedA::LatinCapitalLetterZWithDotAbove),
            'ż' => Ok(LatinExtendedA::LatinSmallLetterZWithDotAbove),
            'Ž' => Ok(LatinExtendedA::LatinCapitalLetterZWithCaron),
            'ž' => Ok(LatinExtendedA::LatinSmallLetterZWithCaron),
            _ => Err(()),
        }
    }
}

impl Into<u32> for LatinExtendedA {
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

impl std::convert::TryFrom<u32> for LatinExtendedA {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for LatinExtendedA {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl LatinExtendedA {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        LatinExtendedA::LatinCapitalLetterAWithMacron
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            LatinExtendedA::LatinCapitalLetterAWithMacron => "latin capital letter a with macron",
            LatinExtendedA::LatinSmallLetterAWithMacron => "latin small letter a with macron",
            LatinExtendedA::LatinCapitalLetterAWithBreve => "latin capital letter a with breve",
            LatinExtendedA::LatinSmallLetterAWithBreve => "latin small letter a with breve",
            LatinExtendedA::LatinCapitalLetterAWithOgonek => "latin capital letter a with ogonek",
            LatinExtendedA::LatinSmallLetterAWithOgonek => "latin small letter a with ogonek",
            LatinExtendedA::LatinCapitalLetterCWithAcute => "latin capital letter c with acute",
            LatinExtendedA::LatinSmallLetterCWithAcute => "latin small letter c with acute",
            LatinExtendedA::LatinCapitalLetterCWithCircumflex => "latin capital letter c with circumflex",
            LatinExtendedA::LatinSmallLetterCWithCircumflex => "latin small letter c with circumflex",
            LatinExtendedA::LatinCapitalLetterCWithDotAbove => "latin capital letter c with dot above",
            LatinExtendedA::LatinSmallLetterCWithDotAbove => "latin small letter c with dot above",
            LatinExtendedA::LatinCapitalLetterCWithCaron => "latin capital letter c with caron",
            LatinExtendedA::LatinSmallLetterCWithCaron => "latin small letter c with caron",
            LatinExtendedA::LatinCapitalLetterDWithCaron => "latin capital letter d with caron",
            LatinExtendedA::LatinSmallLetterDWithCaron => "latin small letter d with caron",
            LatinExtendedA::LatinCapitalLetterDWithStroke => "latin capital letter d with stroke",
            LatinExtendedA::LatinSmallLetterDWithStroke => "latin small letter d with stroke",
            LatinExtendedA::LatinCapitalLetterEWithMacron => "latin capital letter e with macron",
            LatinExtendedA::LatinSmallLetterEWithMacron => "latin small letter e with macron",
            LatinExtendedA::LatinCapitalLetterEWithBreve => "latin capital letter e with breve",
            LatinExtendedA::LatinSmallLetterEWithBreve => "latin small letter e with breve",
            LatinExtendedA::LatinCapitalLetterEWithDotAbove => "latin capital letter e with dot above",
            LatinExtendedA::LatinSmallLetterEWithDotAbove => "latin small letter e with dot above",
            LatinExtendedA::LatinCapitalLetterEWithOgonek => "latin capital letter e with ogonek",
            LatinExtendedA::LatinSmallLetterEWithOgonek => "latin small letter e with ogonek",
            LatinExtendedA::LatinCapitalLetterEWithCaron => "latin capital letter e with caron",
            LatinExtendedA::LatinSmallLetterEWithCaron => "latin small letter e with caron",
            LatinExtendedA::LatinCapitalLetterGWithCircumflex => "latin capital letter g with circumflex",
            LatinExtendedA::LatinSmallLetterGWithCircumflex => "latin small letter g with circumflex",
            LatinExtendedA::LatinCapitalLetterGWithBreve => "latin capital letter g with breve",
            LatinExtendedA::LatinSmallLetterGWithBreve => "latin small letter g with breve",
            LatinExtendedA::LatinCapitalLetterGWithDotAbove => "latin capital letter g with dot above",
            LatinExtendedA::LatinSmallLetterGWithDotAbove => "latin small letter g with dot above",
            LatinExtendedA::LatinCapitalLetterGWithCedilla => "latin capital letter g with cedilla",
            LatinExtendedA::LatinSmallLetterGWithCedilla => "latin small letter g with cedilla",
            LatinExtendedA::LatinCapitalLetterHWithCircumflex => "latin capital letter h with circumflex",
            LatinExtendedA::LatinSmallLetterHWithCircumflex => "latin small letter h with circumflex",
            LatinExtendedA::LatinCapitalLetterHWithStroke => "latin capital letter h with stroke",
            LatinExtendedA::LatinSmallLetterHWithStroke => "latin small letter h with stroke",
            LatinExtendedA::LatinCapitalLetterIWithTilde => "latin capital letter i with tilde",
            LatinExtendedA::LatinSmallLetterIWithTilde => "latin small letter i with tilde",
            LatinExtendedA::LatinCapitalLetterIWithMacron => "latin capital letter i with macron",
            LatinExtendedA::LatinSmallLetterIWithMacron => "latin small letter i with macron",
            LatinExtendedA::LatinCapitalLetterIWithBreve => "latin capital letter i with breve",
            LatinExtendedA::LatinSmallLetterIWithBreve => "latin small letter i with breve",
            LatinExtendedA::LatinCapitalLetterIWithOgonek => "latin capital letter i with ogonek",
            LatinExtendedA::LatinSmallLetterIWithOgonek => "latin small letter i with ogonek",
            LatinExtendedA::LatinCapitalLetterIWithDotAbove => "latin capital letter i with dot above",
            LatinExtendedA::LatinSmallLetterDotlessI => "latin small letter dotless i",
            LatinExtendedA::LatinCapitalLigatureIj => "latin capital ligature ij",
            LatinExtendedA::LatinSmallLigatureIj => "latin small ligature ij",
            LatinExtendedA::LatinCapitalLetterJWithCircumflex => "latin capital letter j with circumflex",
            LatinExtendedA::LatinSmallLetterJWithCircumflex => "latin small letter j with circumflex",
            LatinExtendedA::LatinCapitalLetterKWithCedilla => "latin capital letter k with cedilla",
            LatinExtendedA::LatinSmallLetterKWithCedilla => "latin small letter k with cedilla",
            LatinExtendedA::LatinSmallLetterKra => "latin small letter kra",
            LatinExtendedA::LatinCapitalLetterLWithAcute => "latin capital letter l with acute",
            LatinExtendedA::LatinSmallLetterLWithAcute => "latin small letter l with acute",
            LatinExtendedA::LatinCapitalLetterLWithCedilla => "latin capital letter l with cedilla",
            LatinExtendedA::LatinSmallLetterLWithCedilla => "latin small letter l with cedilla",
            LatinExtendedA::LatinCapitalLetterLWithCaron => "latin capital letter l with caron",
            LatinExtendedA::LatinSmallLetterLWithCaron => "latin small letter l with caron",
            LatinExtendedA::LatinCapitalLetterLWithMiddleDot => "latin capital letter l with middle dot",
            LatinExtendedA::LatinSmallLetterLWithMiddleDot => "latin small letter l with middle dot",
            LatinExtendedA::LatinCapitalLetterLWithStroke => "latin capital letter l with stroke",
            LatinExtendedA::LatinSmallLetterLWithStroke => "latin small letter l with stroke",
            LatinExtendedA::LatinCapitalLetterNWithAcute => "latin capital letter n with acute",
            LatinExtendedA::LatinSmallLetterNWithAcute => "latin small letter n with acute",
            LatinExtendedA::LatinCapitalLetterNWithCedilla => "latin capital letter n with cedilla",
            LatinExtendedA::LatinSmallLetterNWithCedilla => "latin small letter n with cedilla",
            LatinExtendedA::LatinCapitalLetterNWithCaron => "latin capital letter n with caron",
            LatinExtendedA::LatinSmallLetterNWithCaron => "latin small letter n with caron",
            LatinExtendedA::LatinSmallLetterNPrecededByApostrophe => "latin small letter n preceded by apostrophe",
            LatinExtendedA::LatinCapitalLetterEng => "latin capital letter eng",
            LatinExtendedA::LatinSmallLetterEng => "latin small letter eng",
            LatinExtendedA::LatinCapitalLetterOWithMacron => "latin capital letter o with macron",
            LatinExtendedA::LatinSmallLetterOWithMacron => "latin small letter o with macron",
            LatinExtendedA::LatinCapitalLetterOWithBreve => "latin capital letter o with breve",
            LatinExtendedA::LatinSmallLetterOWithBreve => "latin small letter o with breve",
            LatinExtendedA::LatinCapitalLetterOWithDoubleAcute => "latin capital letter o with double acute",
            LatinExtendedA::LatinSmallLetterOWithDoubleAcute => "latin small letter o with double acute",
            LatinExtendedA::LatinCapitalLigatureOe => "latin capital ligature oe",
            LatinExtendedA::LatinSmallLigatureOe => "latin small ligature oe",
            LatinExtendedA::LatinCapitalLetterRWithAcute => "latin capital letter r with acute",
            LatinExtendedA::LatinSmallLetterRWithAcute => "latin small letter r with acute",
            LatinExtendedA::LatinCapitalLetterRWithCedilla => "latin capital letter r with cedilla",
            LatinExtendedA::LatinSmallLetterRWithCedilla => "latin small letter r with cedilla",
            LatinExtendedA::LatinCapitalLetterRWithCaron => "latin capital letter r with caron",
            LatinExtendedA::LatinSmallLetterRWithCaron => "latin small letter r with caron",
            LatinExtendedA::LatinCapitalLetterSWithAcute => "latin capital letter s with acute",
            LatinExtendedA::LatinSmallLetterSWithAcute => "latin small letter s with acute",
            LatinExtendedA::LatinCapitalLetterSWithCircumflex => "latin capital letter s with circumflex",
            LatinExtendedA::LatinSmallLetterSWithCircumflex => "latin small letter s with circumflex",
            LatinExtendedA::LatinCapitalLetterSWithCedilla => "latin capital letter s with cedilla",
            LatinExtendedA::LatinSmallLetterSWithCedilla => "latin small letter s with cedilla",
            LatinExtendedA::LatinCapitalLetterSWithCaron => "latin capital letter s with caron",
            LatinExtendedA::LatinSmallLetterSWithCaron => "latin small letter s with caron",
            LatinExtendedA::LatinCapitalLetterTWithCedilla => "latin capital letter t with cedilla",
            LatinExtendedA::LatinSmallLetterTWithCedilla => "latin small letter t with cedilla",
            LatinExtendedA::LatinCapitalLetterTWithCaron => "latin capital letter t with caron",
            LatinExtendedA::LatinSmallLetterTWithCaron => "latin small letter t with caron",
            LatinExtendedA::LatinCapitalLetterTWithStroke => "latin capital letter t with stroke",
            LatinExtendedA::LatinSmallLetterTWithStroke => "latin small letter t with stroke",
            LatinExtendedA::LatinCapitalLetterUWithTilde => "latin capital letter u with tilde",
            LatinExtendedA::LatinSmallLetterUWithTilde => "latin small letter u with tilde",
            LatinExtendedA::LatinCapitalLetterUWithMacron => "latin capital letter u with macron",
            LatinExtendedA::LatinSmallLetterUWithMacron => "latin small letter u with macron",
            LatinExtendedA::LatinCapitalLetterUWithBreve => "latin capital letter u with breve",
            LatinExtendedA::LatinSmallLetterUWithBreve => "latin small letter u with breve",
            LatinExtendedA::LatinCapitalLetterUWithRingAbove => "latin capital letter u with ring above",
            LatinExtendedA::LatinSmallLetterUWithRingAbove => "latin small letter u with ring above",
            LatinExtendedA::LatinCapitalLetterUWithDoubleAcute => "latin capital letter u with double acute",
            LatinExtendedA::LatinSmallLetterUWithDoubleAcute => "latin small letter u with double acute",
            LatinExtendedA::LatinCapitalLetterUWithOgonek => "latin capital letter u with ogonek",
            LatinExtendedA::LatinSmallLetterUWithOgonek => "latin small letter u with ogonek",
            LatinExtendedA::LatinCapitalLetterWWithCircumflex => "latin capital letter w with circumflex",
            LatinExtendedA::LatinSmallLetterWWithCircumflex => "latin small letter w with circumflex",
            LatinExtendedA::LatinCapitalLetterYWithCircumflex => "latin capital letter y with circumflex",
            LatinExtendedA::LatinSmallLetterYWithCircumflex => "latin small letter y with circumflex",
            LatinExtendedA::LatinCapitalLetterYWithDiaeresis => "latin capital letter y with diaeresis",
            LatinExtendedA::LatinCapitalLetterZWithAcute => "latin capital letter z with acute",
            LatinExtendedA::LatinSmallLetterZWithAcute => "latin small letter z with acute",
            LatinExtendedA::LatinCapitalLetterZWithDotAbove => "latin capital letter z with dot above",
            LatinExtendedA::LatinSmallLetterZWithDotAbove => "latin small letter z with dot above",
            LatinExtendedA::LatinCapitalLetterZWithCaron => "latin capital letter z with caron",
            LatinExtendedA::LatinSmallLetterZWithCaron => "latin small letter z with caron",
        }
    }
}
