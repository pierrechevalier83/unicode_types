
/// An enum to represent all characters in the Tifinagh block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Tifinagh {
    /// \u{2d30}: 'ⴰ'
    LetterYa,
    /// \u{2d31}: 'ⴱ'
    LetterYab,
    /// \u{2d32}: 'ⴲ'
    LetterYabh,
    /// \u{2d33}: 'ⴳ'
    LetterYag,
    /// \u{2d34}: 'ⴴ'
    LetterYaghh,
    /// \u{2d35}: 'ⴵ'
    LetterBerberAcademyYaj,
    /// \u{2d36}: 'ⴶ'
    LetterYaj,
    /// \u{2d37}: 'ⴷ'
    LetterYad,
    /// \u{2d38}: 'ⴸ'
    LetterYadh,
    /// \u{2d39}: 'ⴹ'
    LetterYadd,
    /// \u{2d3a}: 'ⴺ'
    LetterYaddh,
    /// \u{2d3b}: 'ⴻ'
    LetterYey,
    /// \u{2d3c}: 'ⴼ'
    LetterYaf,
    /// \u{2d3d}: 'ⴽ'
    LetterYak,
    /// \u{2d3e}: 'ⴾ'
    LetterTuaregYak,
    /// \u{2d3f}: 'ⴿ'
    LetterYakhh,
    /// \u{2d40}: 'ⵀ'
    LetterYah,
    /// \u{2d41}: 'ⵁ'
    LetterBerberAcademyYah,
    /// \u{2d42}: 'ⵂ'
    LetterTuaregYah,
    /// \u{2d43}: 'ⵃ'
    LetterYahh,
    /// \u{2d44}: 'ⵄ'
    LetterYaa,
    /// \u{2d45}: 'ⵅ'
    LetterYakh,
    /// \u{2d46}: 'ⵆ'
    LetterTuaregYakh,
    /// \u{2d47}: 'ⵇ'
    LetterYaq,
    /// \u{2d48}: 'ⵈ'
    LetterTuaregYaq,
    /// \u{2d49}: 'ⵉ'
    LetterYi,
    /// \u{2d4a}: 'ⵊ'
    LetterYazh,
    /// \u{2d4b}: 'ⵋ'
    LetterAhaggarYazh,
    /// \u{2d4c}: 'ⵌ'
    LetterTuaregYazh,
    /// \u{2d4d}: 'ⵍ'
    LetterYal,
    /// \u{2d4e}: 'ⵎ'
    LetterYam,
    /// \u{2d4f}: 'ⵏ'
    LetterYan,
    /// \u{2d50}: 'ⵐ'
    LetterTuaregYagn,
    /// \u{2d51}: 'ⵑ'
    LetterTuaregYang,
    /// \u{2d52}: 'ⵒ'
    LetterYap,
    /// \u{2d53}: 'ⵓ'
    LetterYu,
    /// \u{2d54}: 'ⵔ'
    LetterYar,
    /// \u{2d55}: 'ⵕ'
    LetterYarr,
    /// \u{2d56}: 'ⵖ'
    LetterYagh,
    /// \u{2d57}: 'ⵗ'
    LetterTuaregYagh,
    /// \u{2d58}: 'ⵘ'
    LetterAyerYagh,
    /// \u{2d59}: 'ⵙ'
    LetterYas,
    /// \u{2d5a}: 'ⵚ'
    LetterYass,
    /// \u{2d5b}: 'ⵛ'
    LetterYash,
    /// \u{2d5c}: 'ⵜ'
    LetterYat,
    /// \u{2d5d}: 'ⵝ'
    LetterYath,
    /// \u{2d5e}: 'ⵞ'
    LetterYach,
    /// \u{2d5f}: 'ⵟ'
    LetterYatt,
    /// \u{2d60}: 'ⵠ'
    LetterYav,
    /// \u{2d61}: 'ⵡ'
    LetterYaw,
    /// \u{2d62}: 'ⵢ'
    LetterYay,
    /// \u{2d63}: 'ⵣ'
    LetterYaz,
    /// \u{2d64}: 'ⵤ'
    LetterTawellemetYaz,
    /// \u{2d65}: 'ⵥ'
    LetterYazz,
    /// \u{2d66}: 'ⵦ'
    LetterYe,
    /// \u{2d67}: 'ⵧ'
    LetterYo,
    /// \u{2d6f}: 'ⵯ'
    ModifierLetterLabializationMark,
    /// \u{2d70}: '⵰'
    SeparatorMark,
}

impl Into<char> for Tifinagh {
    fn into(self) -> char {
        match self {
            Tifinagh::LetterYa => 'ⴰ',
            Tifinagh::LetterYab => 'ⴱ',
            Tifinagh::LetterYabh => 'ⴲ',
            Tifinagh::LetterYag => 'ⴳ',
            Tifinagh::LetterYaghh => 'ⴴ',
            Tifinagh::LetterBerberAcademyYaj => 'ⴵ',
            Tifinagh::LetterYaj => 'ⴶ',
            Tifinagh::LetterYad => 'ⴷ',
            Tifinagh::LetterYadh => 'ⴸ',
            Tifinagh::LetterYadd => 'ⴹ',
            Tifinagh::LetterYaddh => 'ⴺ',
            Tifinagh::LetterYey => 'ⴻ',
            Tifinagh::LetterYaf => 'ⴼ',
            Tifinagh::LetterYak => 'ⴽ',
            Tifinagh::LetterTuaregYak => 'ⴾ',
            Tifinagh::LetterYakhh => 'ⴿ',
            Tifinagh::LetterYah => 'ⵀ',
            Tifinagh::LetterBerberAcademyYah => 'ⵁ',
            Tifinagh::LetterTuaregYah => 'ⵂ',
            Tifinagh::LetterYahh => 'ⵃ',
            Tifinagh::LetterYaa => 'ⵄ',
            Tifinagh::LetterYakh => 'ⵅ',
            Tifinagh::LetterTuaregYakh => 'ⵆ',
            Tifinagh::LetterYaq => 'ⵇ',
            Tifinagh::LetterTuaregYaq => 'ⵈ',
            Tifinagh::LetterYi => 'ⵉ',
            Tifinagh::LetterYazh => 'ⵊ',
            Tifinagh::LetterAhaggarYazh => 'ⵋ',
            Tifinagh::LetterTuaregYazh => 'ⵌ',
            Tifinagh::LetterYal => 'ⵍ',
            Tifinagh::LetterYam => 'ⵎ',
            Tifinagh::LetterYan => 'ⵏ',
            Tifinagh::LetterTuaregYagn => 'ⵐ',
            Tifinagh::LetterTuaregYang => 'ⵑ',
            Tifinagh::LetterYap => 'ⵒ',
            Tifinagh::LetterYu => 'ⵓ',
            Tifinagh::LetterYar => 'ⵔ',
            Tifinagh::LetterYarr => 'ⵕ',
            Tifinagh::LetterYagh => 'ⵖ',
            Tifinagh::LetterTuaregYagh => 'ⵗ',
            Tifinagh::LetterAyerYagh => 'ⵘ',
            Tifinagh::LetterYas => 'ⵙ',
            Tifinagh::LetterYass => 'ⵚ',
            Tifinagh::LetterYash => 'ⵛ',
            Tifinagh::LetterYat => 'ⵜ',
            Tifinagh::LetterYath => 'ⵝ',
            Tifinagh::LetterYach => 'ⵞ',
            Tifinagh::LetterYatt => 'ⵟ',
            Tifinagh::LetterYav => 'ⵠ',
            Tifinagh::LetterYaw => 'ⵡ',
            Tifinagh::LetterYay => 'ⵢ',
            Tifinagh::LetterYaz => 'ⵣ',
            Tifinagh::LetterTawellemetYaz => 'ⵤ',
            Tifinagh::LetterYazz => 'ⵥ',
            Tifinagh::LetterYe => 'ⵦ',
            Tifinagh::LetterYo => 'ⵧ',
            Tifinagh::ModifierLetterLabializationMark => 'ⵯ',
            Tifinagh::SeparatorMark => '⵰',
        }
    }
}

impl std::convert::TryFrom<char> for Tifinagh {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ⴰ' => Ok(Tifinagh::LetterYa),
            'ⴱ' => Ok(Tifinagh::LetterYab),
            'ⴲ' => Ok(Tifinagh::LetterYabh),
            'ⴳ' => Ok(Tifinagh::LetterYag),
            'ⴴ' => Ok(Tifinagh::LetterYaghh),
            'ⴵ' => Ok(Tifinagh::LetterBerberAcademyYaj),
            'ⴶ' => Ok(Tifinagh::LetterYaj),
            'ⴷ' => Ok(Tifinagh::LetterYad),
            'ⴸ' => Ok(Tifinagh::LetterYadh),
            'ⴹ' => Ok(Tifinagh::LetterYadd),
            'ⴺ' => Ok(Tifinagh::LetterYaddh),
            'ⴻ' => Ok(Tifinagh::LetterYey),
            'ⴼ' => Ok(Tifinagh::LetterYaf),
            'ⴽ' => Ok(Tifinagh::LetterYak),
            'ⴾ' => Ok(Tifinagh::LetterTuaregYak),
            'ⴿ' => Ok(Tifinagh::LetterYakhh),
            'ⵀ' => Ok(Tifinagh::LetterYah),
            'ⵁ' => Ok(Tifinagh::LetterBerberAcademyYah),
            'ⵂ' => Ok(Tifinagh::LetterTuaregYah),
            'ⵃ' => Ok(Tifinagh::LetterYahh),
            'ⵄ' => Ok(Tifinagh::LetterYaa),
            'ⵅ' => Ok(Tifinagh::LetterYakh),
            'ⵆ' => Ok(Tifinagh::LetterTuaregYakh),
            'ⵇ' => Ok(Tifinagh::LetterYaq),
            'ⵈ' => Ok(Tifinagh::LetterTuaregYaq),
            'ⵉ' => Ok(Tifinagh::LetterYi),
            'ⵊ' => Ok(Tifinagh::LetterYazh),
            'ⵋ' => Ok(Tifinagh::LetterAhaggarYazh),
            'ⵌ' => Ok(Tifinagh::LetterTuaregYazh),
            'ⵍ' => Ok(Tifinagh::LetterYal),
            'ⵎ' => Ok(Tifinagh::LetterYam),
            'ⵏ' => Ok(Tifinagh::LetterYan),
            'ⵐ' => Ok(Tifinagh::LetterTuaregYagn),
            'ⵑ' => Ok(Tifinagh::LetterTuaregYang),
            'ⵒ' => Ok(Tifinagh::LetterYap),
            'ⵓ' => Ok(Tifinagh::LetterYu),
            'ⵔ' => Ok(Tifinagh::LetterYar),
            'ⵕ' => Ok(Tifinagh::LetterYarr),
            'ⵖ' => Ok(Tifinagh::LetterYagh),
            'ⵗ' => Ok(Tifinagh::LetterTuaregYagh),
            'ⵘ' => Ok(Tifinagh::LetterAyerYagh),
            'ⵙ' => Ok(Tifinagh::LetterYas),
            'ⵚ' => Ok(Tifinagh::LetterYass),
            'ⵛ' => Ok(Tifinagh::LetterYash),
            'ⵜ' => Ok(Tifinagh::LetterYat),
            'ⵝ' => Ok(Tifinagh::LetterYath),
            'ⵞ' => Ok(Tifinagh::LetterYach),
            'ⵟ' => Ok(Tifinagh::LetterYatt),
            'ⵠ' => Ok(Tifinagh::LetterYav),
            'ⵡ' => Ok(Tifinagh::LetterYaw),
            'ⵢ' => Ok(Tifinagh::LetterYay),
            'ⵣ' => Ok(Tifinagh::LetterYaz),
            'ⵤ' => Ok(Tifinagh::LetterTawellemetYaz),
            'ⵥ' => Ok(Tifinagh::LetterYazz),
            'ⵦ' => Ok(Tifinagh::LetterYe),
            'ⵧ' => Ok(Tifinagh::LetterYo),
            'ⵯ' => Ok(Tifinagh::ModifierLetterLabializationMark),
            '⵰' => Ok(Tifinagh::SeparatorMark),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Tifinagh {
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

impl std::convert::TryFrom<u32> for Tifinagh {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Tifinagh {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Tifinagh {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Tifinagh::LetterYa
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Tifinagh::LetterYa => "tifinagh letter ya",
            Tifinagh::LetterYab => "tifinagh letter yab",
            Tifinagh::LetterYabh => "tifinagh letter yabh",
            Tifinagh::LetterYag => "tifinagh letter yag",
            Tifinagh::LetterYaghh => "tifinagh letter yaghh",
            Tifinagh::LetterBerberAcademyYaj => "tifinagh letter berber academy yaj",
            Tifinagh::LetterYaj => "tifinagh letter yaj",
            Tifinagh::LetterYad => "tifinagh letter yad",
            Tifinagh::LetterYadh => "tifinagh letter yadh",
            Tifinagh::LetterYadd => "tifinagh letter yadd",
            Tifinagh::LetterYaddh => "tifinagh letter yaddh",
            Tifinagh::LetterYey => "tifinagh letter yey",
            Tifinagh::LetterYaf => "tifinagh letter yaf",
            Tifinagh::LetterYak => "tifinagh letter yak",
            Tifinagh::LetterTuaregYak => "tifinagh letter tuareg yak",
            Tifinagh::LetterYakhh => "tifinagh letter yakhh",
            Tifinagh::LetterYah => "tifinagh letter yah",
            Tifinagh::LetterBerberAcademyYah => "tifinagh letter berber academy yah",
            Tifinagh::LetterTuaregYah => "tifinagh letter tuareg yah",
            Tifinagh::LetterYahh => "tifinagh letter yahh",
            Tifinagh::LetterYaa => "tifinagh letter yaa",
            Tifinagh::LetterYakh => "tifinagh letter yakh",
            Tifinagh::LetterTuaregYakh => "tifinagh letter tuareg yakh",
            Tifinagh::LetterYaq => "tifinagh letter yaq",
            Tifinagh::LetterTuaregYaq => "tifinagh letter tuareg yaq",
            Tifinagh::LetterYi => "tifinagh letter yi",
            Tifinagh::LetterYazh => "tifinagh letter yazh",
            Tifinagh::LetterAhaggarYazh => "tifinagh letter ahaggar yazh",
            Tifinagh::LetterTuaregYazh => "tifinagh letter tuareg yazh",
            Tifinagh::LetterYal => "tifinagh letter yal",
            Tifinagh::LetterYam => "tifinagh letter yam",
            Tifinagh::LetterYan => "tifinagh letter yan",
            Tifinagh::LetterTuaregYagn => "tifinagh letter tuareg yagn",
            Tifinagh::LetterTuaregYang => "tifinagh letter tuareg yang",
            Tifinagh::LetterYap => "tifinagh letter yap",
            Tifinagh::LetterYu => "tifinagh letter yu",
            Tifinagh::LetterYar => "tifinagh letter yar",
            Tifinagh::LetterYarr => "tifinagh letter yarr",
            Tifinagh::LetterYagh => "tifinagh letter yagh",
            Tifinagh::LetterTuaregYagh => "tifinagh letter tuareg yagh",
            Tifinagh::LetterAyerYagh => "tifinagh letter ayer yagh",
            Tifinagh::LetterYas => "tifinagh letter yas",
            Tifinagh::LetterYass => "tifinagh letter yass",
            Tifinagh::LetterYash => "tifinagh letter yash",
            Tifinagh::LetterYat => "tifinagh letter yat",
            Tifinagh::LetterYath => "tifinagh letter yath",
            Tifinagh::LetterYach => "tifinagh letter yach",
            Tifinagh::LetterYatt => "tifinagh letter yatt",
            Tifinagh::LetterYav => "tifinagh letter yav",
            Tifinagh::LetterYaw => "tifinagh letter yaw",
            Tifinagh::LetterYay => "tifinagh letter yay",
            Tifinagh::LetterYaz => "tifinagh letter yaz",
            Tifinagh::LetterTawellemetYaz => "tifinagh letter tawellemet yaz",
            Tifinagh::LetterYazz => "tifinagh letter yazz",
            Tifinagh::LetterYe => "tifinagh letter ye",
            Tifinagh::LetterYo => "tifinagh letter yo",
            Tifinagh::ModifierLetterLabializationMark => "tifinagh modifier letter labialization mark",
            Tifinagh::SeparatorMark => "tifinagh separator mark",
        }
    }
}
