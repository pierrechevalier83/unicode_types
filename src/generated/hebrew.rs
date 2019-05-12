
/// An enum to represent all characters in the Hebrew block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Hebrew {
    /// \u{591}: '֑'
    AccentEtnahta,
    /// \u{592}: '֒'
    AccentSegol,
    /// \u{593}: '֓'
    AccentShalshelet,
    /// \u{594}: '֔'
    AccentZaqefQatan,
    /// \u{595}: '֕'
    AccentZaqefGadol,
    /// \u{596}: '֖'
    AccentTipeha,
    /// \u{597}: '֗'
    AccentRevia,
    /// \u{598}: '֘'
    AccentZarqa,
    /// \u{599}: '֙'
    AccentPashta,
    /// \u{59a}: '֚'
    AccentYetiv,
    /// \u{59b}: '֛'
    AccentTevir,
    /// \u{59c}: '֜'
    AccentGeresh,
    /// \u{59d}: '֝'
    AccentGereshMuqdam,
    /// \u{59e}: '֞'
    AccentGershayim,
    /// \u{59f}: '֟'
    AccentQarneyPara,
    /// \u{5a0}: '֠'
    AccentTelishaGedola,
    /// \u{5a1}: '֡'
    AccentPazer,
    /// \u{5a2}: '֢'
    AccentAtnahHafukh,
    /// \u{5a3}: '֣'
    AccentMunah,
    /// \u{5a4}: '֤'
    AccentMahapakh,
    /// \u{5a5}: '֥'
    AccentMerkha,
    /// \u{5a6}: '֦'
    AccentMerkhaKefula,
    /// \u{5a7}: '֧'
    AccentDarga,
    /// \u{5a8}: '֨'
    AccentQadma,
    /// \u{5a9}: '֩'
    AccentTelishaQetana,
    /// \u{5aa}: '֪'
    AccentYerahBenYomo,
    /// \u{5ab}: '֫'
    AccentOle,
    /// \u{5ac}: '֬'
    AccentIluy,
    /// \u{5ad}: '֭'
    AccentDehi,
    /// \u{5ae}: '֮'
    AccentZinor,
    /// \u{5af}: '֯'
    MarkMasoraCircle,
    /// \u{5b0}: 'ְ'
    PointSheva,
    /// \u{5b1}: 'ֱ'
    PointHatafSegol,
    /// \u{5b2}: 'ֲ'
    PointHatafPatah,
    /// \u{5b3}: 'ֳ'
    PointHatafQamats,
    /// \u{5b4}: 'ִ'
    PointHiriq,
    /// \u{5b5}: 'ֵ'
    PointTsere,
    /// \u{5b6}: 'ֶ'
    PointSegol,
    /// \u{5b7}: 'ַ'
    PointPatah,
    /// \u{5b8}: 'ָ'
    PointQamats,
    /// \u{5b9}: 'ֹ'
    PointHolam,
    /// \u{5ba}: 'ֺ'
    PointHolamHaserForVav,
    /// \u{5bb}: 'ֻ'
    PointQubuts,
    /// \u{5bc}: 'ּ'
    PointDageshOrMapiq,
    /// \u{5bd}: 'ֽ'
    PointMeteg,
    /// \u{5be}: '־'
    PunctuationMaqaf,
    /// \u{5bf}: 'ֿ'
    PointRafe,
    /// \u{5c0}: '׀'
    PunctuationPaseq,
    /// \u{5c1}: 'ׁ'
    PointShinDot,
    /// \u{5c2}: 'ׂ'
    PointSinDot,
    /// \u{5c3}: '׃'
    PunctuationSofPasuq,
    /// \u{5c4}: 'ׄ'
    MarkUpperDot,
    /// \u{5c5}: 'ׅ'
    MarkLowerDot,
    /// \u{5c6}: '׆'
    PunctuationNunHafukha,
    /// \u{5c7}: 'ׇ'
    PointQamatsQatan,
    /// \u{5d0}: 'א'
    LetterAlef,
    /// \u{5d1}: 'ב'
    LetterBet,
    /// \u{5d2}: 'ג'
    LetterGimel,
    /// \u{5d3}: 'ד'
    LetterDalet,
    /// \u{5d4}: 'ה'
    LetterHe,
    /// \u{5d5}: 'ו'
    LetterVav,
    /// \u{5d6}: 'ז'
    LetterZayin,
    /// \u{5d7}: 'ח'
    LetterHet,
    /// \u{5d8}: 'ט'
    LetterTet,
    /// \u{5d9}: 'י'
    LetterYod,
    /// \u{5da}: 'ך'
    LetterFinalKaf,
    /// \u{5db}: 'כ'
    LetterKaf,
    /// \u{5dc}: 'ל'
    LetterLamed,
    /// \u{5dd}: 'ם'
    LetterFinalMem,
    /// \u{5de}: 'מ'
    LetterMem,
    /// \u{5df}: 'ן'
    LetterFinalNun,
    /// \u{5e0}: 'נ'
    LetterNun,
    /// \u{5e1}: 'ס'
    LetterSamekh,
    /// \u{5e2}: 'ע'
    LetterAyin,
    /// \u{5e3}: 'ף'
    LetterFinalPe,
    /// \u{5e4}: 'פ'
    LetterPe,
    /// \u{5e5}: 'ץ'
    LetterFinalTsadi,
    /// \u{5e6}: 'צ'
    LetterTsadi,
    /// \u{5e7}: 'ק'
    LetterQof,
    /// \u{5e8}: 'ר'
    LetterResh,
    /// \u{5e9}: 'ש'
    LetterShin,
    /// \u{5ea}: 'ת'
    LetterTav,
    /// \u{5ef}: 'ׯ'
    YodTriangle,
    /// \u{5f0}: 'װ'
    LigatureYiddishDoubleVav,
    /// \u{5f1}: 'ױ'
    LigatureYiddishVavYod,
    /// \u{5f2}: 'ײ'
    LigatureYiddishDoubleYod,
    /// \u{5f3}: '׳'
    PunctuationGeresh,
    /// \u{5f4}: '״'
    PunctuationGershayim,
}

impl Into<char> for Hebrew {
    fn into(self) -> char {
        match self {
            Hebrew::AccentEtnahta => '֑',
            Hebrew::AccentSegol => '֒',
            Hebrew::AccentShalshelet => '֓',
            Hebrew::AccentZaqefQatan => '֔',
            Hebrew::AccentZaqefGadol => '֕',
            Hebrew::AccentTipeha => '֖',
            Hebrew::AccentRevia => '֗',
            Hebrew::AccentZarqa => '֘',
            Hebrew::AccentPashta => '֙',
            Hebrew::AccentYetiv => '֚',
            Hebrew::AccentTevir => '֛',
            Hebrew::AccentGeresh => '֜',
            Hebrew::AccentGereshMuqdam => '֝',
            Hebrew::AccentGershayim => '֞',
            Hebrew::AccentQarneyPara => '֟',
            Hebrew::AccentTelishaGedola => '֠',
            Hebrew::AccentPazer => '֡',
            Hebrew::AccentAtnahHafukh => '֢',
            Hebrew::AccentMunah => '֣',
            Hebrew::AccentMahapakh => '֤',
            Hebrew::AccentMerkha => '֥',
            Hebrew::AccentMerkhaKefula => '֦',
            Hebrew::AccentDarga => '֧',
            Hebrew::AccentQadma => '֨',
            Hebrew::AccentTelishaQetana => '֩',
            Hebrew::AccentYerahBenYomo => '֪',
            Hebrew::AccentOle => '֫',
            Hebrew::AccentIluy => '֬',
            Hebrew::AccentDehi => '֭',
            Hebrew::AccentZinor => '֮',
            Hebrew::MarkMasoraCircle => '֯',
            Hebrew::PointSheva => 'ְ',
            Hebrew::PointHatafSegol => 'ֱ',
            Hebrew::PointHatafPatah => 'ֲ',
            Hebrew::PointHatafQamats => 'ֳ',
            Hebrew::PointHiriq => 'ִ',
            Hebrew::PointTsere => 'ֵ',
            Hebrew::PointSegol => 'ֶ',
            Hebrew::PointPatah => 'ַ',
            Hebrew::PointQamats => 'ָ',
            Hebrew::PointHolam => 'ֹ',
            Hebrew::PointHolamHaserForVav => 'ֺ',
            Hebrew::PointQubuts => 'ֻ',
            Hebrew::PointDageshOrMapiq => 'ּ',
            Hebrew::PointMeteg => 'ֽ',
            Hebrew::PunctuationMaqaf => '־',
            Hebrew::PointRafe => 'ֿ',
            Hebrew::PunctuationPaseq => '׀',
            Hebrew::PointShinDot => 'ׁ',
            Hebrew::PointSinDot => 'ׂ',
            Hebrew::PunctuationSofPasuq => '׃',
            Hebrew::MarkUpperDot => 'ׄ',
            Hebrew::MarkLowerDot => 'ׅ',
            Hebrew::PunctuationNunHafukha => '׆',
            Hebrew::PointQamatsQatan => 'ׇ',
            Hebrew::LetterAlef => 'א',
            Hebrew::LetterBet => 'ב',
            Hebrew::LetterGimel => 'ג',
            Hebrew::LetterDalet => 'ד',
            Hebrew::LetterHe => 'ה',
            Hebrew::LetterVav => 'ו',
            Hebrew::LetterZayin => 'ז',
            Hebrew::LetterHet => 'ח',
            Hebrew::LetterTet => 'ט',
            Hebrew::LetterYod => 'י',
            Hebrew::LetterFinalKaf => 'ך',
            Hebrew::LetterKaf => 'כ',
            Hebrew::LetterLamed => 'ל',
            Hebrew::LetterFinalMem => 'ם',
            Hebrew::LetterMem => 'מ',
            Hebrew::LetterFinalNun => 'ן',
            Hebrew::LetterNun => 'נ',
            Hebrew::LetterSamekh => 'ס',
            Hebrew::LetterAyin => 'ע',
            Hebrew::LetterFinalPe => 'ף',
            Hebrew::LetterPe => 'פ',
            Hebrew::LetterFinalTsadi => 'ץ',
            Hebrew::LetterTsadi => 'צ',
            Hebrew::LetterQof => 'ק',
            Hebrew::LetterResh => 'ר',
            Hebrew::LetterShin => 'ש',
            Hebrew::LetterTav => 'ת',
            Hebrew::YodTriangle => 'ׯ',
            Hebrew::LigatureYiddishDoubleVav => 'װ',
            Hebrew::LigatureYiddishVavYod => 'ױ',
            Hebrew::LigatureYiddishDoubleYod => 'ײ',
            Hebrew::PunctuationGeresh => '׳',
            Hebrew::PunctuationGershayim => '״',
        }
    }
}

impl std::convert::TryFrom<char> for Hebrew {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '֑' => Ok(Hebrew::AccentEtnahta),
            '֒' => Ok(Hebrew::AccentSegol),
            '֓' => Ok(Hebrew::AccentShalshelet),
            '֔' => Ok(Hebrew::AccentZaqefQatan),
            '֕' => Ok(Hebrew::AccentZaqefGadol),
            '֖' => Ok(Hebrew::AccentTipeha),
            '֗' => Ok(Hebrew::AccentRevia),
            '֘' => Ok(Hebrew::AccentZarqa),
            '֙' => Ok(Hebrew::AccentPashta),
            '֚' => Ok(Hebrew::AccentYetiv),
            '֛' => Ok(Hebrew::AccentTevir),
            '֜' => Ok(Hebrew::AccentGeresh),
            '֝' => Ok(Hebrew::AccentGereshMuqdam),
            '֞' => Ok(Hebrew::AccentGershayim),
            '֟' => Ok(Hebrew::AccentQarneyPara),
            '֠' => Ok(Hebrew::AccentTelishaGedola),
            '֡' => Ok(Hebrew::AccentPazer),
            '֢' => Ok(Hebrew::AccentAtnahHafukh),
            '֣' => Ok(Hebrew::AccentMunah),
            '֤' => Ok(Hebrew::AccentMahapakh),
            '֥' => Ok(Hebrew::AccentMerkha),
            '֦' => Ok(Hebrew::AccentMerkhaKefula),
            '֧' => Ok(Hebrew::AccentDarga),
            '֨' => Ok(Hebrew::AccentQadma),
            '֩' => Ok(Hebrew::AccentTelishaQetana),
            '֪' => Ok(Hebrew::AccentYerahBenYomo),
            '֫' => Ok(Hebrew::AccentOle),
            '֬' => Ok(Hebrew::AccentIluy),
            '֭' => Ok(Hebrew::AccentDehi),
            '֮' => Ok(Hebrew::AccentZinor),
            '֯' => Ok(Hebrew::MarkMasoraCircle),
            'ְ' => Ok(Hebrew::PointSheva),
            'ֱ' => Ok(Hebrew::PointHatafSegol),
            'ֲ' => Ok(Hebrew::PointHatafPatah),
            'ֳ' => Ok(Hebrew::PointHatafQamats),
            'ִ' => Ok(Hebrew::PointHiriq),
            'ֵ' => Ok(Hebrew::PointTsere),
            'ֶ' => Ok(Hebrew::PointSegol),
            'ַ' => Ok(Hebrew::PointPatah),
            'ָ' => Ok(Hebrew::PointQamats),
            'ֹ' => Ok(Hebrew::PointHolam),
            'ֺ' => Ok(Hebrew::PointHolamHaserForVav),
            'ֻ' => Ok(Hebrew::PointQubuts),
            'ּ' => Ok(Hebrew::PointDageshOrMapiq),
            'ֽ' => Ok(Hebrew::PointMeteg),
            '־' => Ok(Hebrew::PunctuationMaqaf),
            'ֿ' => Ok(Hebrew::PointRafe),
            '׀' => Ok(Hebrew::PunctuationPaseq),
            'ׁ' => Ok(Hebrew::PointShinDot),
            'ׂ' => Ok(Hebrew::PointSinDot),
            '׃' => Ok(Hebrew::PunctuationSofPasuq),
            'ׄ' => Ok(Hebrew::MarkUpperDot),
            'ׅ' => Ok(Hebrew::MarkLowerDot),
            '׆' => Ok(Hebrew::PunctuationNunHafukha),
            'ׇ' => Ok(Hebrew::PointQamatsQatan),
            'א' => Ok(Hebrew::LetterAlef),
            'ב' => Ok(Hebrew::LetterBet),
            'ג' => Ok(Hebrew::LetterGimel),
            'ד' => Ok(Hebrew::LetterDalet),
            'ה' => Ok(Hebrew::LetterHe),
            'ו' => Ok(Hebrew::LetterVav),
            'ז' => Ok(Hebrew::LetterZayin),
            'ח' => Ok(Hebrew::LetterHet),
            'ט' => Ok(Hebrew::LetterTet),
            'י' => Ok(Hebrew::LetterYod),
            'ך' => Ok(Hebrew::LetterFinalKaf),
            'כ' => Ok(Hebrew::LetterKaf),
            'ל' => Ok(Hebrew::LetterLamed),
            'ם' => Ok(Hebrew::LetterFinalMem),
            'מ' => Ok(Hebrew::LetterMem),
            'ן' => Ok(Hebrew::LetterFinalNun),
            'נ' => Ok(Hebrew::LetterNun),
            'ס' => Ok(Hebrew::LetterSamekh),
            'ע' => Ok(Hebrew::LetterAyin),
            'ף' => Ok(Hebrew::LetterFinalPe),
            'פ' => Ok(Hebrew::LetterPe),
            'ץ' => Ok(Hebrew::LetterFinalTsadi),
            'צ' => Ok(Hebrew::LetterTsadi),
            'ק' => Ok(Hebrew::LetterQof),
            'ר' => Ok(Hebrew::LetterResh),
            'ש' => Ok(Hebrew::LetterShin),
            'ת' => Ok(Hebrew::LetterTav),
            'ׯ' => Ok(Hebrew::YodTriangle),
            'װ' => Ok(Hebrew::LigatureYiddishDoubleVav),
            'ױ' => Ok(Hebrew::LigatureYiddishVavYod),
            'ײ' => Ok(Hebrew::LigatureYiddishDoubleYod),
            '׳' => Ok(Hebrew::PunctuationGeresh),
            '״' => Ok(Hebrew::PunctuationGershayim),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Hebrew {
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

impl std::convert::TryFrom<u32> for Hebrew {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Hebrew {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Hebrew {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Hebrew::AccentEtnahta
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Hebrew{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
