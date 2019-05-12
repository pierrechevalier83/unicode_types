/// A number of constants to give a name to all characters in this block.
mod constants {
    /// \u{780}: 'ހ'
    pub const LETTER_HAA: char = 'ހ';
    /// \u{781}: 'ށ'
    pub const LETTER_SHAVIYANI: char = 'ށ';
    /// \u{782}: 'ނ'
    pub const LETTER_NOONU: char = 'ނ';
    /// \u{783}: 'ރ'
    pub const LETTER_RAA: char = 'ރ';
    /// \u{784}: 'ބ'
    pub const LETTER_BAA: char = 'ބ';
    /// \u{785}: 'ޅ'
    pub const LETTER_LHAVIYANI: char = 'ޅ';
    /// \u{786}: 'ކ'
    pub const LETTER_KAAFU: char = 'ކ';
    /// \u{787}: 'އ'
    pub const LETTER_ALIFU: char = 'އ';
    /// \u{788}: 'ވ'
    pub const LETTER_VAAVU: char = 'ވ';
    /// \u{789}: 'މ'
    pub const LETTER_MEEMU: char = 'މ';
    /// \u{78a}: 'ފ'
    pub const LETTER_FAAFU: char = 'ފ';
    /// \u{78b}: 'ދ'
    pub const LETTER_DHAALU: char = 'ދ';
    /// \u{78c}: 'ތ'
    pub const LETTER_THAA: char = 'ތ';
    /// \u{78d}: 'ލ'
    pub const LETTER_LAAMU: char = 'ލ';
    /// \u{78e}: 'ގ'
    pub const LETTER_GAAFU: char = 'ގ';
    /// \u{78f}: 'ޏ'
    pub const LETTER_GNAVIYANI: char = 'ޏ';
    /// \u{790}: 'ސ'
    pub const LETTER_SEENU: char = 'ސ';
    /// \u{791}: 'ޑ'
    pub const LETTER_DAVIYANI: char = 'ޑ';
    /// \u{792}: 'ޒ'
    pub const LETTER_ZAVIYANI: char = 'ޒ';
    /// \u{793}: 'ޓ'
    pub const LETTER_TAVIYANI: char = 'ޓ';
    /// \u{794}: 'ޔ'
    pub const LETTER_YAA: char = 'ޔ';
    /// \u{795}: 'ޕ'
    pub const LETTER_PAVIYANI: char = 'ޕ';
    /// \u{796}: 'ޖ'
    pub const LETTER_JAVIYANI: char = 'ޖ';
    /// \u{797}: 'ޗ'
    pub const LETTER_CHAVIYANI: char = 'ޗ';
    /// \u{798}: 'ޘ'
    pub const LETTER_TTAA: char = 'ޘ';
    /// \u{799}: 'ޙ'
    pub const LETTER_HHAA: char = 'ޙ';
    /// \u{79a}: 'ޚ'
    pub const LETTER_KHAA: char = 'ޚ';
    /// \u{79b}: 'ޛ'
    pub const LETTER_THAALU: char = 'ޛ';
    /// \u{79c}: 'ޜ'
    pub const LETTER_ZAA: char = 'ޜ';
    /// \u{79d}: 'ޝ'
    pub const LETTER_SHEENU: char = 'ޝ';
    /// \u{79e}: 'ޞ'
    pub const LETTER_SAADHU: char = 'ޞ';
    /// \u{79f}: 'ޟ'
    pub const LETTER_DAADHU: char = 'ޟ';
    /// \u{7a0}: 'ޠ'
    pub const LETTER_TO: char = 'ޠ';
    /// \u{7a1}: 'ޡ'
    pub const LETTER_ZO: char = 'ޡ';
    /// \u{7a2}: 'ޢ'
    pub const LETTER_AINU: char = 'ޢ';
    /// \u{7a3}: 'ޣ'
    pub const LETTER_GHAINU: char = 'ޣ';
    /// \u{7a4}: 'ޤ'
    pub const LETTER_QAAFU: char = 'ޤ';
    /// \u{7a5}: 'ޥ'
    pub const LETTER_WAAVU: char = 'ޥ';
    /// \u{7a6}: 'ަ'
    pub const ABAFILI: char = 'ަ';
    /// \u{7a7}: 'ާ'
    pub const AABAAFILI: char = 'ާ';
    /// \u{7a8}: 'ި'
    pub const IBIFILI: char = 'ި';
    /// \u{7a9}: 'ީ'
    pub const EEBEEFILI: char = 'ީ';
    /// \u{7aa}: 'ު'
    pub const UBUFILI: char = 'ު';
    /// \u{7ab}: 'ޫ'
    pub const OOBOOFILI: char = 'ޫ';
    /// \u{7ac}: 'ެ'
    pub const EBEFILI: char = 'ެ';
    /// \u{7ad}: 'ޭ'
    pub const EYBEYFILI: char = 'ޭ';
    /// \u{7ae}: 'ޮ'
    pub const OBOFILI: char = 'ޮ';
    /// \u{7af}: 'ޯ'
    pub const OABOAFILI: char = 'ޯ';
    /// \u{7b0}: 'ް'
    pub const SUKUN: char = 'ް';
    /// \u{7b1}: 'ޱ'
    pub const LETTER_NAA: char = 'ޱ';
}

/// An enum to represent all characters in the Thaana block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Thaana {
    /// \u{780}: 'ހ'
    LetterHaa,
    /// \u{781}: 'ށ'
    LetterShaviyani,
    /// \u{782}: 'ނ'
    LetterNoonu,
    /// \u{783}: 'ރ'
    LetterRaa,
    /// \u{784}: 'ބ'
    LetterBaa,
    /// \u{785}: 'ޅ'
    LetterLhaviyani,
    /// \u{786}: 'ކ'
    LetterKaafu,
    /// \u{787}: 'އ'
    LetterAlifu,
    /// \u{788}: 'ވ'
    LetterVaavu,
    /// \u{789}: 'މ'
    LetterMeemu,
    /// \u{78a}: 'ފ'
    LetterFaafu,
    /// \u{78b}: 'ދ'
    LetterDhaalu,
    /// \u{78c}: 'ތ'
    LetterThaa,
    /// \u{78d}: 'ލ'
    LetterLaamu,
    /// \u{78e}: 'ގ'
    LetterGaafu,
    /// \u{78f}: 'ޏ'
    LetterGnaviyani,
    /// \u{790}: 'ސ'
    LetterSeenu,
    /// \u{791}: 'ޑ'
    LetterDaviyani,
    /// \u{792}: 'ޒ'
    LetterZaviyani,
    /// \u{793}: 'ޓ'
    LetterTaviyani,
    /// \u{794}: 'ޔ'
    LetterYaa,
    /// \u{795}: 'ޕ'
    LetterPaviyani,
    /// \u{796}: 'ޖ'
    LetterJaviyani,
    /// \u{797}: 'ޗ'
    LetterChaviyani,
    /// \u{798}: 'ޘ'
    LetterTtaa,
    /// \u{799}: 'ޙ'
    LetterHhaa,
    /// \u{79a}: 'ޚ'
    LetterKhaa,
    /// \u{79b}: 'ޛ'
    LetterThaalu,
    /// \u{79c}: 'ޜ'
    LetterZaa,
    /// \u{79d}: 'ޝ'
    LetterSheenu,
    /// \u{79e}: 'ޞ'
    LetterSaadhu,
    /// \u{79f}: 'ޟ'
    LetterDaadhu,
    /// \u{7a0}: 'ޠ'
    LetterTo,
    /// \u{7a1}: 'ޡ'
    LetterZo,
    /// \u{7a2}: 'ޢ'
    LetterAinu,
    /// \u{7a3}: 'ޣ'
    LetterGhainu,
    /// \u{7a4}: 'ޤ'
    LetterQaafu,
    /// \u{7a5}: 'ޥ'
    LetterWaavu,
    /// \u{7a6}: 'ަ'
    Abafili,
    /// \u{7a7}: 'ާ'
    Aabaafili,
    /// \u{7a8}: 'ި'
    Ibifili,
    /// \u{7a9}: 'ީ'
    Eebeefili,
    /// \u{7aa}: 'ު'
    Ubufili,
    /// \u{7ab}: 'ޫ'
    Ooboofili,
    /// \u{7ac}: 'ެ'
    Ebefili,
    /// \u{7ad}: 'ޭ'
    Eybeyfili,
    /// \u{7ae}: 'ޮ'
    Obofili,
    /// \u{7af}: 'ޯ'
    Oaboafili,
    /// \u{7b0}: 'ް'
    Sukun,
    /// \u{7b1}: 'ޱ'
    LetterNaa,
}

impl Into<char> for Thaana {
    fn into(self) -> char {
        use constants::*;
        match self {
            Thaana::LetterHaa => LETTER_HAA,
            Thaana::LetterShaviyani => LETTER_SHAVIYANI,
            Thaana::LetterNoonu => LETTER_NOONU,
            Thaana::LetterRaa => LETTER_RAA,
            Thaana::LetterBaa => LETTER_BAA,
            Thaana::LetterLhaviyani => LETTER_LHAVIYANI,
            Thaana::LetterKaafu => LETTER_KAAFU,
            Thaana::LetterAlifu => LETTER_ALIFU,
            Thaana::LetterVaavu => LETTER_VAAVU,
            Thaana::LetterMeemu => LETTER_MEEMU,
            Thaana::LetterFaafu => LETTER_FAAFU,
            Thaana::LetterDhaalu => LETTER_DHAALU,
            Thaana::LetterThaa => LETTER_THAA,
            Thaana::LetterLaamu => LETTER_LAAMU,
            Thaana::LetterGaafu => LETTER_GAAFU,
            Thaana::LetterGnaviyani => LETTER_GNAVIYANI,
            Thaana::LetterSeenu => LETTER_SEENU,
            Thaana::LetterDaviyani => LETTER_DAVIYANI,
            Thaana::LetterZaviyani => LETTER_ZAVIYANI,
            Thaana::LetterTaviyani => LETTER_TAVIYANI,
            Thaana::LetterYaa => LETTER_YAA,
            Thaana::LetterPaviyani => LETTER_PAVIYANI,
            Thaana::LetterJaviyani => LETTER_JAVIYANI,
            Thaana::LetterChaviyani => LETTER_CHAVIYANI,
            Thaana::LetterTtaa => LETTER_TTAA,
            Thaana::LetterHhaa => LETTER_HHAA,
            Thaana::LetterKhaa => LETTER_KHAA,
            Thaana::LetterThaalu => LETTER_THAALU,
            Thaana::LetterZaa => LETTER_ZAA,
            Thaana::LetterSheenu => LETTER_SHEENU,
            Thaana::LetterSaadhu => LETTER_SAADHU,
            Thaana::LetterDaadhu => LETTER_DAADHU,
            Thaana::LetterTo => LETTER_TO,
            Thaana::LetterZo => LETTER_ZO,
            Thaana::LetterAinu => LETTER_AINU,
            Thaana::LetterGhainu => LETTER_GHAINU,
            Thaana::LetterQaafu => LETTER_QAAFU,
            Thaana::LetterWaavu => LETTER_WAAVU,
            Thaana::Abafili => ABAFILI,
            Thaana::Aabaafili => AABAAFILI,
            Thaana::Ibifili => IBIFILI,
            Thaana::Eebeefili => EEBEEFILI,
            Thaana::Ubufili => UBUFILI,
            Thaana::Ooboofili => OOBOOFILI,
            Thaana::Ebefili => EBEFILI,
            Thaana::Eybeyfili => EYBEYFILI,
            Thaana::Obofili => OBOFILI,
            Thaana::Oaboafili => OABOAFILI,
            Thaana::Sukun => SUKUN,
            Thaana::LetterNaa => LETTER_NAA,
        }
    }
}

impl std::convert::TryFrom<char> for Thaana {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        use constants::*;
        match c {
            LETTER_HAA => Ok(Thaana::LetterHaa),
            LETTER_SHAVIYANI => Ok(Thaana::LetterShaviyani),
            LETTER_NOONU => Ok(Thaana::LetterNoonu),
            LETTER_RAA => Ok(Thaana::LetterRaa),
            LETTER_BAA => Ok(Thaana::LetterBaa),
            LETTER_LHAVIYANI => Ok(Thaana::LetterLhaviyani),
            LETTER_KAAFU => Ok(Thaana::LetterKaafu),
            LETTER_ALIFU => Ok(Thaana::LetterAlifu),
            LETTER_VAAVU => Ok(Thaana::LetterVaavu),
            LETTER_MEEMU => Ok(Thaana::LetterMeemu),
            LETTER_FAAFU => Ok(Thaana::LetterFaafu),
            LETTER_DHAALU => Ok(Thaana::LetterDhaalu),
            LETTER_THAA => Ok(Thaana::LetterThaa),
            LETTER_LAAMU => Ok(Thaana::LetterLaamu),
            LETTER_GAAFU => Ok(Thaana::LetterGaafu),
            LETTER_GNAVIYANI => Ok(Thaana::LetterGnaviyani),
            LETTER_SEENU => Ok(Thaana::LetterSeenu),
            LETTER_DAVIYANI => Ok(Thaana::LetterDaviyani),
            LETTER_ZAVIYANI => Ok(Thaana::LetterZaviyani),
            LETTER_TAVIYANI => Ok(Thaana::LetterTaviyani),
            LETTER_YAA => Ok(Thaana::LetterYaa),
            LETTER_PAVIYANI => Ok(Thaana::LetterPaviyani),
            LETTER_JAVIYANI => Ok(Thaana::LetterJaviyani),
            LETTER_CHAVIYANI => Ok(Thaana::LetterChaviyani),
            LETTER_TTAA => Ok(Thaana::LetterTtaa),
            LETTER_HHAA => Ok(Thaana::LetterHhaa),
            LETTER_KHAA => Ok(Thaana::LetterKhaa),
            LETTER_THAALU => Ok(Thaana::LetterThaalu),
            LETTER_ZAA => Ok(Thaana::LetterZaa),
            LETTER_SHEENU => Ok(Thaana::LetterSheenu),
            LETTER_SAADHU => Ok(Thaana::LetterSaadhu),
            LETTER_DAADHU => Ok(Thaana::LetterDaadhu),
            LETTER_TO => Ok(Thaana::LetterTo),
            LETTER_ZO => Ok(Thaana::LetterZo),
            LETTER_AINU => Ok(Thaana::LetterAinu),
            LETTER_GHAINU => Ok(Thaana::LetterGhainu),
            LETTER_QAAFU => Ok(Thaana::LetterQaafu),
            LETTER_WAAVU => Ok(Thaana::LetterWaavu),
            ABAFILI => Ok(Thaana::Abafili),
            AABAAFILI => Ok(Thaana::Aabaafili),
            IBIFILI => Ok(Thaana::Ibifili),
            EEBEEFILI => Ok(Thaana::Eebeefili),
            UBUFILI => Ok(Thaana::Ubufili),
            OOBOOFILI => Ok(Thaana::Ooboofili),
            EBEFILI => Ok(Thaana::Ebefili),
            EYBEYFILI => Ok(Thaana::Eybeyfili),
            OBOFILI => Ok(Thaana::Obofili),
            OABOAFILI => Ok(Thaana::Oaboafili),
            SUKUN => Ok(Thaana::Sukun),
            LETTER_NAA => Ok(Thaana::LetterNaa),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Thaana {
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

impl std::convert::TryFrom<u32> for Thaana {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Thaana {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Thaana {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Thaana::LetterHaa
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Thaana::LetterHaa => "thaana letter haa",
            Thaana::LetterShaviyani => "thaana letter shaviyani",
            Thaana::LetterNoonu => "thaana letter noonu",
            Thaana::LetterRaa => "thaana letter raa",
            Thaana::LetterBaa => "thaana letter baa",
            Thaana::LetterLhaviyani => "thaana letter lhaviyani",
            Thaana::LetterKaafu => "thaana letter kaafu",
            Thaana::LetterAlifu => "thaana letter alifu",
            Thaana::LetterVaavu => "thaana letter vaavu",
            Thaana::LetterMeemu => "thaana letter meemu",
            Thaana::LetterFaafu => "thaana letter faafu",
            Thaana::LetterDhaalu => "thaana letter dhaalu",
            Thaana::LetterThaa => "thaana letter thaa",
            Thaana::LetterLaamu => "thaana letter laamu",
            Thaana::LetterGaafu => "thaana letter gaafu",
            Thaana::LetterGnaviyani => "thaana letter gnaviyani",
            Thaana::LetterSeenu => "thaana letter seenu",
            Thaana::LetterDaviyani => "thaana letter daviyani",
            Thaana::LetterZaviyani => "thaana letter zaviyani",
            Thaana::LetterTaviyani => "thaana letter taviyani",
            Thaana::LetterYaa => "thaana letter yaa",
            Thaana::LetterPaviyani => "thaana letter paviyani",
            Thaana::LetterJaviyani => "thaana letter javiyani",
            Thaana::LetterChaviyani => "thaana letter chaviyani",
            Thaana::LetterTtaa => "thaana letter ttaa",
            Thaana::LetterHhaa => "thaana letter hhaa",
            Thaana::LetterKhaa => "thaana letter khaa",
            Thaana::LetterThaalu => "thaana letter thaalu",
            Thaana::LetterZaa => "thaana letter zaa",
            Thaana::LetterSheenu => "thaana letter sheenu",
            Thaana::LetterSaadhu => "thaana letter saadhu",
            Thaana::LetterDaadhu => "thaana letter daadhu",
            Thaana::LetterTo => "thaana letter to",
            Thaana::LetterZo => "thaana letter zo",
            Thaana::LetterAinu => "thaana letter ainu",
            Thaana::LetterGhainu => "thaana letter ghainu",
            Thaana::LetterQaafu => "thaana letter qaafu",
            Thaana::LetterWaavu => "thaana letter waavu",
            Thaana::Abafili => "thaana abafili",
            Thaana::Aabaafili => "thaana aabaafili",
            Thaana::Ibifili => "thaana ibifili",
            Thaana::Eebeefili => "thaana eebeefili",
            Thaana::Ubufili => "thaana ubufili",
            Thaana::Ooboofili => "thaana ooboofili",
            Thaana::Ebefili => "thaana ebefili",
            Thaana::Eybeyfili => "thaana eybeyfili",
            Thaana::Obofili => "thaana obofili",
            Thaana::Oaboafili => "thaana oaboafili",
            Thaana::Sukun => "thaana sukun",
            Thaana::LetterNaa => "thaana letter naa",
        }
    }
}
