
/// An enum to represent all characters in the Bamum block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Bamum {
    /// \u{a6a0}: 'ꚠ'
    LetterA,
    /// \u{a6a1}: 'ꚡ'
    LetterKa,
    /// \u{a6a2}: 'ꚢ'
    LetterU,
    /// \u{a6a3}: 'ꚣ'
    LetterKu,
    /// \u{a6a4}: 'ꚤ'
    LetterEe,
    /// \u{a6a5}: 'ꚥ'
    LetterRee,
    /// \u{a6a6}: 'ꚦ'
    LetterTae,
    /// \u{a6a7}: 'ꚧ'
    LetterO,
    /// \u{a6a8}: 'ꚨ'
    LetterNyi,
    /// \u{a6a9}: 'ꚩ'
    LetterI,
    /// \u{a6aa}: 'ꚪ'
    LetterLa,
    /// \u{a6ab}: 'ꚫ'
    LetterPa,
    /// \u{a6ac}: 'ꚬ'
    LetterRii,
    /// \u{a6ad}: 'ꚭ'
    LetterRiee,
    /// \u{a6ae}: 'ꚮ'
    LetterLeeee,
    /// \u{a6af}: 'ꚯ'
    LetterMeeee,
    /// \u{a6b0}: 'ꚰ'
    LetterTaa,
    /// \u{a6b1}: 'ꚱ'
    LetterNdaa,
    /// \u{a6b2}: 'ꚲ'
    LetterNjaem,
    /// \u{a6b3}: 'ꚳ'
    LetterM,
    /// \u{a6b4}: 'ꚴ'
    LetterSuu,
    /// \u{a6b5}: 'ꚵ'
    LetterMu,
    /// \u{a6b6}: 'ꚶ'
    LetterShii,
    /// \u{a6b7}: 'ꚷ'
    LetterSi,
    /// \u{a6b8}: 'ꚸ'
    LetterSheux,
    /// \u{a6b9}: 'ꚹ'
    LetterSeux,
    /// \u{a6ba}: 'ꚺ'
    LetterKyee,
    /// \u{a6bb}: 'ꚻ'
    LetterKet,
    /// \u{a6bc}: 'ꚼ'
    LetterNuae,
    /// \u{a6bd}: 'ꚽ'
    LetterNu,
    /// \u{a6be}: 'ꚾ'
    LetterNjuae,
    /// \u{a6bf}: 'ꚿ'
    LetterYoq,
    /// \u{a6c0}: 'ꛀ'
    LetterShu,
    /// \u{a6c1}: 'ꛁ'
    LetterYuq,
    /// \u{a6c2}: 'ꛂ'
    LetterYa,
    /// \u{a6c3}: 'ꛃ'
    LetterNsha,
    /// \u{a6c4}: 'ꛄ'
    LetterKeux,
    /// \u{a6c5}: 'ꛅ'
    LetterPeux,
    /// \u{a6c6}: 'ꛆ'
    LetterNjee,
    /// \u{a6c7}: 'ꛇ'
    LetterNtee,
    /// \u{a6c8}: 'ꛈ'
    LetterPue,
    /// \u{a6c9}: 'ꛉ'
    LetterWue,
    /// \u{a6ca}: 'ꛊ'
    LetterPee,
    /// \u{a6cb}: 'ꛋ'
    LetterFee,
    /// \u{a6cc}: 'ꛌ'
    LetterRu,
    /// \u{a6cd}: 'ꛍ'
    LetterLu,
    /// \u{a6ce}: 'ꛎ'
    LetterMi,
    /// \u{a6cf}: 'ꛏ'
    LetterNi,
    /// \u{a6d0}: 'ꛐ'
    LetterReux,
    /// \u{a6d1}: 'ꛑ'
    LetterRae,
    /// \u{a6d2}: 'ꛒ'
    LetterKen,
    /// \u{a6d3}: 'ꛓ'
    LetterNgkwaen,
    /// \u{a6d4}: 'ꛔ'
    LetterNgga,
    /// \u{a6d5}: 'ꛕ'
    LetterNga,
    /// \u{a6d6}: 'ꛖ'
    LetterSho,
    /// \u{a6d7}: 'ꛗ'
    LetterPuae,
    /// \u{a6d8}: 'ꛘ'
    LetterFu,
    /// \u{a6d9}: 'ꛙ'
    LetterFom,
    /// \u{a6da}: 'ꛚ'
    LetterWa,
    /// \u{a6db}: 'ꛛ'
    LetterNa,
    /// \u{a6dc}: 'ꛜ'
    LetterLi,
    /// \u{a6dd}: 'ꛝ'
    LetterPi,
    /// \u{a6de}: 'ꛞ'
    LetterLoq,
    /// \u{a6df}: 'ꛟ'
    LetterKo,
    /// \u{a6e0}: 'ꛠ'
    LetterMben,
    /// \u{a6e1}: 'ꛡ'
    LetterRen,
    /// \u{a6e2}: 'ꛢ'
    LetterMen,
    /// \u{a6e3}: 'ꛣ'
    LetterMa,
    /// \u{a6e4}: 'ꛤ'
    LetterTi,
    /// \u{a6e5}: 'ꛥ'
    LetterKi,
    /// \u{a6e6}: 'ꛦ'
    LetterMo,
    /// \u{a6e7}: 'ꛧ'
    LetterMbaa,
    /// \u{a6e8}: 'ꛨ'
    LetterTet,
    /// \u{a6e9}: 'ꛩ'
    LetterKpa,
    /// \u{a6ea}: 'ꛪ'
    LetterTen,
    /// \u{a6eb}: 'ꛫ'
    LetterNtuu,
    /// \u{a6ec}: 'ꛬ'
    LetterSamba,
    /// \u{a6ed}: 'ꛭ'
    LetterFaamae,
    /// \u{a6ee}: 'ꛮ'
    LetterKovuu,
    /// \u{a6ef}: 'ꛯ'
    LetterKoghom,
    /// \u{a6f0}: '꛰'
    CombiningMarkKoqndon,
    /// \u{a6f1}: '꛱'
    CombiningMarkTukwentis,
    /// \u{a6f2}: '꛲'
    Njaemli,
    /// \u{a6f3}: '꛳'
    FullStop,
    /// \u{a6f4}: '꛴'
    Colon,
    /// \u{a6f5}: '꛵'
    Comma,
    /// \u{a6f6}: '꛶'
    Semicolon,
    /// \u{a6f7}: '꛷'
    QuestionMark,
}

impl Into<char> for Bamum {
    fn into(self) -> char {
        match self {
            Bamum::LetterA => 'ꚠ',
            Bamum::LetterKa => 'ꚡ',
            Bamum::LetterU => 'ꚢ',
            Bamum::LetterKu => 'ꚣ',
            Bamum::LetterEe => 'ꚤ',
            Bamum::LetterRee => 'ꚥ',
            Bamum::LetterTae => 'ꚦ',
            Bamum::LetterO => 'ꚧ',
            Bamum::LetterNyi => 'ꚨ',
            Bamum::LetterI => 'ꚩ',
            Bamum::LetterLa => 'ꚪ',
            Bamum::LetterPa => 'ꚫ',
            Bamum::LetterRii => 'ꚬ',
            Bamum::LetterRiee => 'ꚭ',
            Bamum::LetterLeeee => 'ꚮ',
            Bamum::LetterMeeee => 'ꚯ',
            Bamum::LetterTaa => 'ꚰ',
            Bamum::LetterNdaa => 'ꚱ',
            Bamum::LetterNjaem => 'ꚲ',
            Bamum::LetterM => 'ꚳ',
            Bamum::LetterSuu => 'ꚴ',
            Bamum::LetterMu => 'ꚵ',
            Bamum::LetterShii => 'ꚶ',
            Bamum::LetterSi => 'ꚷ',
            Bamum::LetterSheux => 'ꚸ',
            Bamum::LetterSeux => 'ꚹ',
            Bamum::LetterKyee => 'ꚺ',
            Bamum::LetterKet => 'ꚻ',
            Bamum::LetterNuae => 'ꚼ',
            Bamum::LetterNu => 'ꚽ',
            Bamum::LetterNjuae => 'ꚾ',
            Bamum::LetterYoq => 'ꚿ',
            Bamum::LetterShu => 'ꛀ',
            Bamum::LetterYuq => 'ꛁ',
            Bamum::LetterYa => 'ꛂ',
            Bamum::LetterNsha => 'ꛃ',
            Bamum::LetterKeux => 'ꛄ',
            Bamum::LetterPeux => 'ꛅ',
            Bamum::LetterNjee => 'ꛆ',
            Bamum::LetterNtee => 'ꛇ',
            Bamum::LetterPue => 'ꛈ',
            Bamum::LetterWue => 'ꛉ',
            Bamum::LetterPee => 'ꛊ',
            Bamum::LetterFee => 'ꛋ',
            Bamum::LetterRu => 'ꛌ',
            Bamum::LetterLu => 'ꛍ',
            Bamum::LetterMi => 'ꛎ',
            Bamum::LetterNi => 'ꛏ',
            Bamum::LetterReux => 'ꛐ',
            Bamum::LetterRae => 'ꛑ',
            Bamum::LetterKen => 'ꛒ',
            Bamum::LetterNgkwaen => 'ꛓ',
            Bamum::LetterNgga => 'ꛔ',
            Bamum::LetterNga => 'ꛕ',
            Bamum::LetterSho => 'ꛖ',
            Bamum::LetterPuae => 'ꛗ',
            Bamum::LetterFu => 'ꛘ',
            Bamum::LetterFom => 'ꛙ',
            Bamum::LetterWa => 'ꛚ',
            Bamum::LetterNa => 'ꛛ',
            Bamum::LetterLi => 'ꛜ',
            Bamum::LetterPi => 'ꛝ',
            Bamum::LetterLoq => 'ꛞ',
            Bamum::LetterKo => 'ꛟ',
            Bamum::LetterMben => 'ꛠ',
            Bamum::LetterRen => 'ꛡ',
            Bamum::LetterMen => 'ꛢ',
            Bamum::LetterMa => 'ꛣ',
            Bamum::LetterTi => 'ꛤ',
            Bamum::LetterKi => 'ꛥ',
            Bamum::LetterMo => 'ꛦ',
            Bamum::LetterMbaa => 'ꛧ',
            Bamum::LetterTet => 'ꛨ',
            Bamum::LetterKpa => 'ꛩ',
            Bamum::LetterTen => 'ꛪ',
            Bamum::LetterNtuu => 'ꛫ',
            Bamum::LetterSamba => 'ꛬ',
            Bamum::LetterFaamae => 'ꛭ',
            Bamum::LetterKovuu => 'ꛮ',
            Bamum::LetterKoghom => 'ꛯ',
            Bamum::CombiningMarkKoqndon => '꛰',
            Bamum::CombiningMarkTukwentis => '꛱',
            Bamum::Njaemli => '꛲',
            Bamum::FullStop => '꛳',
            Bamum::Colon => '꛴',
            Bamum::Comma => '꛵',
            Bamum::Semicolon => '꛶',
            Bamum::QuestionMark => '꛷',
        }
    }
}

impl std::convert::TryFrom<char> for Bamum {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ꚠ' => Ok(Bamum::LetterA),
            'ꚡ' => Ok(Bamum::LetterKa),
            'ꚢ' => Ok(Bamum::LetterU),
            'ꚣ' => Ok(Bamum::LetterKu),
            'ꚤ' => Ok(Bamum::LetterEe),
            'ꚥ' => Ok(Bamum::LetterRee),
            'ꚦ' => Ok(Bamum::LetterTae),
            'ꚧ' => Ok(Bamum::LetterO),
            'ꚨ' => Ok(Bamum::LetterNyi),
            'ꚩ' => Ok(Bamum::LetterI),
            'ꚪ' => Ok(Bamum::LetterLa),
            'ꚫ' => Ok(Bamum::LetterPa),
            'ꚬ' => Ok(Bamum::LetterRii),
            'ꚭ' => Ok(Bamum::LetterRiee),
            'ꚮ' => Ok(Bamum::LetterLeeee),
            'ꚯ' => Ok(Bamum::LetterMeeee),
            'ꚰ' => Ok(Bamum::LetterTaa),
            'ꚱ' => Ok(Bamum::LetterNdaa),
            'ꚲ' => Ok(Bamum::LetterNjaem),
            'ꚳ' => Ok(Bamum::LetterM),
            'ꚴ' => Ok(Bamum::LetterSuu),
            'ꚵ' => Ok(Bamum::LetterMu),
            'ꚶ' => Ok(Bamum::LetterShii),
            'ꚷ' => Ok(Bamum::LetterSi),
            'ꚸ' => Ok(Bamum::LetterSheux),
            'ꚹ' => Ok(Bamum::LetterSeux),
            'ꚺ' => Ok(Bamum::LetterKyee),
            'ꚻ' => Ok(Bamum::LetterKet),
            'ꚼ' => Ok(Bamum::LetterNuae),
            'ꚽ' => Ok(Bamum::LetterNu),
            'ꚾ' => Ok(Bamum::LetterNjuae),
            'ꚿ' => Ok(Bamum::LetterYoq),
            'ꛀ' => Ok(Bamum::LetterShu),
            'ꛁ' => Ok(Bamum::LetterYuq),
            'ꛂ' => Ok(Bamum::LetterYa),
            'ꛃ' => Ok(Bamum::LetterNsha),
            'ꛄ' => Ok(Bamum::LetterKeux),
            'ꛅ' => Ok(Bamum::LetterPeux),
            'ꛆ' => Ok(Bamum::LetterNjee),
            'ꛇ' => Ok(Bamum::LetterNtee),
            'ꛈ' => Ok(Bamum::LetterPue),
            'ꛉ' => Ok(Bamum::LetterWue),
            'ꛊ' => Ok(Bamum::LetterPee),
            'ꛋ' => Ok(Bamum::LetterFee),
            'ꛌ' => Ok(Bamum::LetterRu),
            'ꛍ' => Ok(Bamum::LetterLu),
            'ꛎ' => Ok(Bamum::LetterMi),
            'ꛏ' => Ok(Bamum::LetterNi),
            'ꛐ' => Ok(Bamum::LetterReux),
            'ꛑ' => Ok(Bamum::LetterRae),
            'ꛒ' => Ok(Bamum::LetterKen),
            'ꛓ' => Ok(Bamum::LetterNgkwaen),
            'ꛔ' => Ok(Bamum::LetterNgga),
            'ꛕ' => Ok(Bamum::LetterNga),
            'ꛖ' => Ok(Bamum::LetterSho),
            'ꛗ' => Ok(Bamum::LetterPuae),
            'ꛘ' => Ok(Bamum::LetterFu),
            'ꛙ' => Ok(Bamum::LetterFom),
            'ꛚ' => Ok(Bamum::LetterWa),
            'ꛛ' => Ok(Bamum::LetterNa),
            'ꛜ' => Ok(Bamum::LetterLi),
            'ꛝ' => Ok(Bamum::LetterPi),
            'ꛞ' => Ok(Bamum::LetterLoq),
            'ꛟ' => Ok(Bamum::LetterKo),
            'ꛠ' => Ok(Bamum::LetterMben),
            'ꛡ' => Ok(Bamum::LetterRen),
            'ꛢ' => Ok(Bamum::LetterMen),
            'ꛣ' => Ok(Bamum::LetterMa),
            'ꛤ' => Ok(Bamum::LetterTi),
            'ꛥ' => Ok(Bamum::LetterKi),
            'ꛦ' => Ok(Bamum::LetterMo),
            'ꛧ' => Ok(Bamum::LetterMbaa),
            'ꛨ' => Ok(Bamum::LetterTet),
            'ꛩ' => Ok(Bamum::LetterKpa),
            'ꛪ' => Ok(Bamum::LetterTen),
            'ꛫ' => Ok(Bamum::LetterNtuu),
            'ꛬ' => Ok(Bamum::LetterSamba),
            'ꛭ' => Ok(Bamum::LetterFaamae),
            'ꛮ' => Ok(Bamum::LetterKovuu),
            'ꛯ' => Ok(Bamum::LetterKoghom),
            '꛰' => Ok(Bamum::CombiningMarkKoqndon),
            '꛱' => Ok(Bamum::CombiningMarkTukwentis),
            '꛲' => Ok(Bamum::Njaemli),
            '꛳' => Ok(Bamum::FullStop),
            '꛴' => Ok(Bamum::Colon),
            '꛵' => Ok(Bamum::Comma),
            '꛶' => Ok(Bamum::Semicolon),
            '꛷' => Ok(Bamum::QuestionMark),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Bamum {
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

impl std::convert::TryFrom<u32> for Bamum {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Bamum {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Bamum {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Bamum::LetterA
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Bamum::LetterA => "bamum letter a",
            Bamum::LetterKa => "bamum letter ka",
            Bamum::LetterU => "bamum letter u",
            Bamum::LetterKu => "bamum letter ku",
            Bamum::LetterEe => "bamum letter ee",
            Bamum::LetterRee => "bamum letter ree",
            Bamum::LetterTae => "bamum letter tae",
            Bamum::LetterO => "bamum letter o",
            Bamum::LetterNyi => "bamum letter nyi",
            Bamum::LetterI => "bamum letter i",
            Bamum::LetterLa => "bamum letter la",
            Bamum::LetterPa => "bamum letter pa",
            Bamum::LetterRii => "bamum letter rii",
            Bamum::LetterRiee => "bamum letter riee",
            Bamum::LetterLeeee => "bamum letter leeee",
            Bamum::LetterMeeee => "bamum letter meeee",
            Bamum::LetterTaa => "bamum letter taa",
            Bamum::LetterNdaa => "bamum letter ndaa",
            Bamum::LetterNjaem => "bamum letter njaem",
            Bamum::LetterM => "bamum letter m",
            Bamum::LetterSuu => "bamum letter suu",
            Bamum::LetterMu => "bamum letter mu",
            Bamum::LetterShii => "bamum letter shii",
            Bamum::LetterSi => "bamum letter si",
            Bamum::LetterSheux => "bamum letter sheux",
            Bamum::LetterSeux => "bamum letter seux",
            Bamum::LetterKyee => "bamum letter kyee",
            Bamum::LetterKet => "bamum letter ket",
            Bamum::LetterNuae => "bamum letter nuae",
            Bamum::LetterNu => "bamum letter nu",
            Bamum::LetterNjuae => "bamum letter njuae",
            Bamum::LetterYoq => "bamum letter yoq",
            Bamum::LetterShu => "bamum letter shu",
            Bamum::LetterYuq => "bamum letter yuq",
            Bamum::LetterYa => "bamum letter ya",
            Bamum::LetterNsha => "bamum letter nsha",
            Bamum::LetterKeux => "bamum letter keux",
            Bamum::LetterPeux => "bamum letter peux",
            Bamum::LetterNjee => "bamum letter njee",
            Bamum::LetterNtee => "bamum letter ntee",
            Bamum::LetterPue => "bamum letter pue",
            Bamum::LetterWue => "bamum letter wue",
            Bamum::LetterPee => "bamum letter pee",
            Bamum::LetterFee => "bamum letter fee",
            Bamum::LetterRu => "bamum letter ru",
            Bamum::LetterLu => "bamum letter lu",
            Bamum::LetterMi => "bamum letter mi",
            Bamum::LetterNi => "bamum letter ni",
            Bamum::LetterReux => "bamum letter reux",
            Bamum::LetterRae => "bamum letter rae",
            Bamum::LetterKen => "bamum letter ken",
            Bamum::LetterNgkwaen => "bamum letter ngkwaen",
            Bamum::LetterNgga => "bamum letter ngga",
            Bamum::LetterNga => "bamum letter nga",
            Bamum::LetterSho => "bamum letter sho",
            Bamum::LetterPuae => "bamum letter puae",
            Bamum::LetterFu => "bamum letter fu",
            Bamum::LetterFom => "bamum letter fom",
            Bamum::LetterWa => "bamum letter wa",
            Bamum::LetterNa => "bamum letter na",
            Bamum::LetterLi => "bamum letter li",
            Bamum::LetterPi => "bamum letter pi",
            Bamum::LetterLoq => "bamum letter loq",
            Bamum::LetterKo => "bamum letter ko",
            Bamum::LetterMben => "bamum letter mben",
            Bamum::LetterRen => "bamum letter ren",
            Bamum::LetterMen => "bamum letter men",
            Bamum::LetterMa => "bamum letter ma",
            Bamum::LetterTi => "bamum letter ti",
            Bamum::LetterKi => "bamum letter ki",
            Bamum::LetterMo => "bamum letter mo",
            Bamum::LetterMbaa => "bamum letter mbaa",
            Bamum::LetterTet => "bamum letter tet",
            Bamum::LetterKpa => "bamum letter kpa",
            Bamum::LetterTen => "bamum letter ten",
            Bamum::LetterNtuu => "bamum letter ntuu",
            Bamum::LetterSamba => "bamum letter samba",
            Bamum::LetterFaamae => "bamum letter faamae",
            Bamum::LetterKovuu => "bamum letter kovuu",
            Bamum::LetterKoghom => "bamum letter koghom",
            Bamum::CombiningMarkKoqndon => "bamum combining mark koqndon",
            Bamum::CombiningMarkTukwentis => "bamum combining mark tukwentis",
            Bamum::Njaemli => "bamum njaemli",
            Bamum::FullStop => "bamum full stop",
            Bamum::Colon => "bamum colon",
            Bamum::Comma => "bamum comma",
            Bamum::Semicolon => "bamum semicolon",
            Bamum::QuestionMark => "bamum question mark",
        }
    }
}
