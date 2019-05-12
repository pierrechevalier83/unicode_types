
/// An enum to represent all characters in the CaucasianAlbanian block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum CaucasianAlbanian {
    /// \u{10530}: '𐔰'
    LetterAlt,
    /// \u{10531}: '𐔱'
    LetterBet,
    /// \u{10532}: '𐔲'
    LetterGim,
    /// \u{10533}: '𐔳'
    LetterDat,
    /// \u{10534}: '𐔴'
    LetterEb,
    /// \u{10535}: '𐔵'
    LetterZarl,
    /// \u{10536}: '𐔶'
    LetterEyn,
    /// \u{10537}: '𐔷'
    LetterZhil,
    /// \u{10538}: '𐔸'
    LetterTas,
    /// \u{10539}: '𐔹'
    LetterCha,
    /// \u{1053a}: '𐔺'
    LetterYowd,
    /// \u{1053b}: '𐔻'
    LetterZha,
    /// \u{1053c}: '𐔼'
    LetterIrb,
    /// \u{1053d}: '𐔽'
    LetterSha,
    /// \u{1053e}: '𐔾'
    LetterLan,
    /// \u{1053f}: '𐔿'
    LetterInya,
    /// \u{10540}: '𐕀'
    LetterXeyn,
    /// \u{10541}: '𐕁'
    LetterDyan,
    /// \u{10542}: '𐕂'
    LetterCar,
    /// \u{10543}: '𐕃'
    LetterJhox,
    /// \u{10544}: '𐕄'
    LetterKar,
    /// \u{10545}: '𐕅'
    LetterLyit,
    /// \u{10546}: '𐕆'
    LetterHeyt,
    /// \u{10547}: '𐕇'
    LetterQay,
    /// \u{10548}: '𐕈'
    LetterAor,
    /// \u{10549}: '𐕉'
    LetterChoy,
    /// \u{1054a}: '𐕊'
    LetterChi,
    /// \u{1054b}: '𐕋'
    LetterCyay,
    /// \u{1054c}: '𐕌'
    LetterMaq,
    /// \u{1054d}: '𐕍'
    LetterQar,
    /// \u{1054e}: '𐕎'
    LetterNowc,
    /// \u{1054f}: '𐕏'
    LetterDzyay,
    /// \u{10550}: '𐕐'
    LetterShak,
    /// \u{10551}: '𐕑'
    LetterJayn,
    /// \u{10552}: '𐕒'
    LetterOn,
    /// \u{10553}: '𐕓'
    LetterTyay,
    /// \u{10554}: '𐕔'
    LetterFam,
    /// \u{10555}: '𐕕'
    LetterDzay,
    /// \u{10556}: '𐕖'
    LetterChat,
    /// \u{10557}: '𐕗'
    LetterPen,
    /// \u{10558}: '𐕘'
    LetterGheys,
    /// \u{10559}: '𐕙'
    LetterRat,
    /// \u{1055a}: '𐕚'
    LetterSeyk,
    /// \u{1055b}: '𐕛'
    LetterVeyz,
    /// \u{1055c}: '𐕜'
    LetterTiwr,
    /// \u{1055d}: '𐕝'
    LetterShoy,
    /// \u{1055e}: '𐕞'
    LetterIwn,
    /// \u{1055f}: '𐕟'
    LetterCyaw,
    /// \u{10560}: '𐕠'
    LetterCayn,
    /// \u{10561}: '𐕡'
    LetterYayd,
    /// \u{10562}: '𐕢'
    LetterPiwr,
    /// \u{10563}: '𐕣'
    LetterKiw,
}

impl Into<char> for CaucasianAlbanian {
    fn into(self) -> char {
        match self {
            CaucasianAlbanian::LetterAlt => '𐔰',
            CaucasianAlbanian::LetterBet => '𐔱',
            CaucasianAlbanian::LetterGim => '𐔲',
            CaucasianAlbanian::LetterDat => '𐔳',
            CaucasianAlbanian::LetterEb => '𐔴',
            CaucasianAlbanian::LetterZarl => '𐔵',
            CaucasianAlbanian::LetterEyn => '𐔶',
            CaucasianAlbanian::LetterZhil => '𐔷',
            CaucasianAlbanian::LetterTas => '𐔸',
            CaucasianAlbanian::LetterCha => '𐔹',
            CaucasianAlbanian::LetterYowd => '𐔺',
            CaucasianAlbanian::LetterZha => '𐔻',
            CaucasianAlbanian::LetterIrb => '𐔼',
            CaucasianAlbanian::LetterSha => '𐔽',
            CaucasianAlbanian::LetterLan => '𐔾',
            CaucasianAlbanian::LetterInya => '𐔿',
            CaucasianAlbanian::LetterXeyn => '𐕀',
            CaucasianAlbanian::LetterDyan => '𐕁',
            CaucasianAlbanian::LetterCar => '𐕂',
            CaucasianAlbanian::LetterJhox => '𐕃',
            CaucasianAlbanian::LetterKar => '𐕄',
            CaucasianAlbanian::LetterLyit => '𐕅',
            CaucasianAlbanian::LetterHeyt => '𐕆',
            CaucasianAlbanian::LetterQay => '𐕇',
            CaucasianAlbanian::LetterAor => '𐕈',
            CaucasianAlbanian::LetterChoy => '𐕉',
            CaucasianAlbanian::LetterChi => '𐕊',
            CaucasianAlbanian::LetterCyay => '𐕋',
            CaucasianAlbanian::LetterMaq => '𐕌',
            CaucasianAlbanian::LetterQar => '𐕍',
            CaucasianAlbanian::LetterNowc => '𐕎',
            CaucasianAlbanian::LetterDzyay => '𐕏',
            CaucasianAlbanian::LetterShak => '𐕐',
            CaucasianAlbanian::LetterJayn => '𐕑',
            CaucasianAlbanian::LetterOn => '𐕒',
            CaucasianAlbanian::LetterTyay => '𐕓',
            CaucasianAlbanian::LetterFam => '𐕔',
            CaucasianAlbanian::LetterDzay => '𐕕',
            CaucasianAlbanian::LetterChat => '𐕖',
            CaucasianAlbanian::LetterPen => '𐕗',
            CaucasianAlbanian::LetterGheys => '𐕘',
            CaucasianAlbanian::LetterRat => '𐕙',
            CaucasianAlbanian::LetterSeyk => '𐕚',
            CaucasianAlbanian::LetterVeyz => '𐕛',
            CaucasianAlbanian::LetterTiwr => '𐕜',
            CaucasianAlbanian::LetterShoy => '𐕝',
            CaucasianAlbanian::LetterIwn => '𐕞',
            CaucasianAlbanian::LetterCyaw => '𐕟',
            CaucasianAlbanian::LetterCayn => '𐕠',
            CaucasianAlbanian::LetterYayd => '𐕡',
            CaucasianAlbanian::LetterPiwr => '𐕢',
            CaucasianAlbanian::LetterKiw => '𐕣',
        }
    }
}

impl std::convert::TryFrom<char> for CaucasianAlbanian {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𐔰' => Ok(CaucasianAlbanian::LetterAlt),
            '𐔱' => Ok(CaucasianAlbanian::LetterBet),
            '𐔲' => Ok(CaucasianAlbanian::LetterGim),
            '𐔳' => Ok(CaucasianAlbanian::LetterDat),
            '𐔴' => Ok(CaucasianAlbanian::LetterEb),
            '𐔵' => Ok(CaucasianAlbanian::LetterZarl),
            '𐔶' => Ok(CaucasianAlbanian::LetterEyn),
            '𐔷' => Ok(CaucasianAlbanian::LetterZhil),
            '𐔸' => Ok(CaucasianAlbanian::LetterTas),
            '𐔹' => Ok(CaucasianAlbanian::LetterCha),
            '𐔺' => Ok(CaucasianAlbanian::LetterYowd),
            '𐔻' => Ok(CaucasianAlbanian::LetterZha),
            '𐔼' => Ok(CaucasianAlbanian::LetterIrb),
            '𐔽' => Ok(CaucasianAlbanian::LetterSha),
            '𐔾' => Ok(CaucasianAlbanian::LetterLan),
            '𐔿' => Ok(CaucasianAlbanian::LetterInya),
            '𐕀' => Ok(CaucasianAlbanian::LetterXeyn),
            '𐕁' => Ok(CaucasianAlbanian::LetterDyan),
            '𐕂' => Ok(CaucasianAlbanian::LetterCar),
            '𐕃' => Ok(CaucasianAlbanian::LetterJhox),
            '𐕄' => Ok(CaucasianAlbanian::LetterKar),
            '𐕅' => Ok(CaucasianAlbanian::LetterLyit),
            '𐕆' => Ok(CaucasianAlbanian::LetterHeyt),
            '𐕇' => Ok(CaucasianAlbanian::LetterQay),
            '𐕈' => Ok(CaucasianAlbanian::LetterAor),
            '𐕉' => Ok(CaucasianAlbanian::LetterChoy),
            '𐕊' => Ok(CaucasianAlbanian::LetterChi),
            '𐕋' => Ok(CaucasianAlbanian::LetterCyay),
            '𐕌' => Ok(CaucasianAlbanian::LetterMaq),
            '𐕍' => Ok(CaucasianAlbanian::LetterQar),
            '𐕎' => Ok(CaucasianAlbanian::LetterNowc),
            '𐕏' => Ok(CaucasianAlbanian::LetterDzyay),
            '𐕐' => Ok(CaucasianAlbanian::LetterShak),
            '𐕑' => Ok(CaucasianAlbanian::LetterJayn),
            '𐕒' => Ok(CaucasianAlbanian::LetterOn),
            '𐕓' => Ok(CaucasianAlbanian::LetterTyay),
            '𐕔' => Ok(CaucasianAlbanian::LetterFam),
            '𐕕' => Ok(CaucasianAlbanian::LetterDzay),
            '𐕖' => Ok(CaucasianAlbanian::LetterChat),
            '𐕗' => Ok(CaucasianAlbanian::LetterPen),
            '𐕘' => Ok(CaucasianAlbanian::LetterGheys),
            '𐕙' => Ok(CaucasianAlbanian::LetterRat),
            '𐕚' => Ok(CaucasianAlbanian::LetterSeyk),
            '𐕛' => Ok(CaucasianAlbanian::LetterVeyz),
            '𐕜' => Ok(CaucasianAlbanian::LetterTiwr),
            '𐕝' => Ok(CaucasianAlbanian::LetterShoy),
            '𐕞' => Ok(CaucasianAlbanian::LetterIwn),
            '𐕟' => Ok(CaucasianAlbanian::LetterCyaw),
            '𐕠' => Ok(CaucasianAlbanian::LetterCayn),
            '𐕡' => Ok(CaucasianAlbanian::LetterYayd),
            '𐕢' => Ok(CaucasianAlbanian::LetterPiwr),
            '𐕣' => Ok(CaucasianAlbanian::LetterKiw),
            _ => Err(()),
        }
    }
}

impl Into<u32> for CaucasianAlbanian {
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

impl std::convert::TryFrom<u32> for CaucasianAlbanian {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for CaucasianAlbanian {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl CaucasianAlbanian {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        CaucasianAlbanian::LetterAlt
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("CaucasianAlbanian{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
