
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
        match self {
            Thaana::LetterHaa => 'ހ',
            Thaana::LetterShaviyani => 'ށ',
            Thaana::LetterNoonu => 'ނ',
            Thaana::LetterRaa => 'ރ',
            Thaana::LetterBaa => 'ބ',
            Thaana::LetterLhaviyani => 'ޅ',
            Thaana::LetterKaafu => 'ކ',
            Thaana::LetterAlifu => 'އ',
            Thaana::LetterVaavu => 'ވ',
            Thaana::LetterMeemu => 'މ',
            Thaana::LetterFaafu => 'ފ',
            Thaana::LetterDhaalu => 'ދ',
            Thaana::LetterThaa => 'ތ',
            Thaana::LetterLaamu => 'ލ',
            Thaana::LetterGaafu => 'ގ',
            Thaana::LetterGnaviyani => 'ޏ',
            Thaana::LetterSeenu => 'ސ',
            Thaana::LetterDaviyani => 'ޑ',
            Thaana::LetterZaviyani => 'ޒ',
            Thaana::LetterTaviyani => 'ޓ',
            Thaana::LetterYaa => 'ޔ',
            Thaana::LetterPaviyani => 'ޕ',
            Thaana::LetterJaviyani => 'ޖ',
            Thaana::LetterChaviyani => 'ޗ',
            Thaana::LetterTtaa => 'ޘ',
            Thaana::LetterHhaa => 'ޙ',
            Thaana::LetterKhaa => 'ޚ',
            Thaana::LetterThaalu => 'ޛ',
            Thaana::LetterZaa => 'ޜ',
            Thaana::LetterSheenu => 'ޝ',
            Thaana::LetterSaadhu => 'ޞ',
            Thaana::LetterDaadhu => 'ޟ',
            Thaana::LetterTo => 'ޠ',
            Thaana::LetterZo => 'ޡ',
            Thaana::LetterAinu => 'ޢ',
            Thaana::LetterGhainu => 'ޣ',
            Thaana::LetterQaafu => 'ޤ',
            Thaana::LetterWaavu => 'ޥ',
            Thaana::Abafili => 'ަ',
            Thaana::Aabaafili => 'ާ',
            Thaana::Ibifili => 'ި',
            Thaana::Eebeefili => 'ީ',
            Thaana::Ubufili => 'ު',
            Thaana::Ooboofili => 'ޫ',
            Thaana::Ebefili => 'ެ',
            Thaana::Eybeyfili => 'ޭ',
            Thaana::Obofili => 'ޮ',
            Thaana::Oaboafili => 'ޯ',
            Thaana::Sukun => 'ް',
            Thaana::LetterNaa => 'ޱ',
        }
    }
}

impl std::convert::TryFrom<char> for Thaana {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ހ' => Ok(Thaana::LetterHaa),
            'ށ' => Ok(Thaana::LetterShaviyani),
            'ނ' => Ok(Thaana::LetterNoonu),
            'ރ' => Ok(Thaana::LetterRaa),
            'ބ' => Ok(Thaana::LetterBaa),
            'ޅ' => Ok(Thaana::LetterLhaviyani),
            'ކ' => Ok(Thaana::LetterKaafu),
            'އ' => Ok(Thaana::LetterAlifu),
            'ވ' => Ok(Thaana::LetterVaavu),
            'މ' => Ok(Thaana::LetterMeemu),
            'ފ' => Ok(Thaana::LetterFaafu),
            'ދ' => Ok(Thaana::LetterDhaalu),
            'ތ' => Ok(Thaana::LetterThaa),
            'ލ' => Ok(Thaana::LetterLaamu),
            'ގ' => Ok(Thaana::LetterGaafu),
            'ޏ' => Ok(Thaana::LetterGnaviyani),
            'ސ' => Ok(Thaana::LetterSeenu),
            'ޑ' => Ok(Thaana::LetterDaviyani),
            'ޒ' => Ok(Thaana::LetterZaviyani),
            'ޓ' => Ok(Thaana::LetterTaviyani),
            'ޔ' => Ok(Thaana::LetterYaa),
            'ޕ' => Ok(Thaana::LetterPaviyani),
            'ޖ' => Ok(Thaana::LetterJaviyani),
            'ޗ' => Ok(Thaana::LetterChaviyani),
            'ޘ' => Ok(Thaana::LetterTtaa),
            'ޙ' => Ok(Thaana::LetterHhaa),
            'ޚ' => Ok(Thaana::LetterKhaa),
            'ޛ' => Ok(Thaana::LetterThaalu),
            'ޜ' => Ok(Thaana::LetterZaa),
            'ޝ' => Ok(Thaana::LetterSheenu),
            'ޞ' => Ok(Thaana::LetterSaadhu),
            'ޟ' => Ok(Thaana::LetterDaadhu),
            'ޠ' => Ok(Thaana::LetterTo),
            'ޡ' => Ok(Thaana::LetterZo),
            'ޢ' => Ok(Thaana::LetterAinu),
            'ޣ' => Ok(Thaana::LetterGhainu),
            'ޤ' => Ok(Thaana::LetterQaafu),
            'ޥ' => Ok(Thaana::LetterWaavu),
            'ަ' => Ok(Thaana::Abafili),
            'ާ' => Ok(Thaana::Aabaafili),
            'ި' => Ok(Thaana::Ibifili),
            'ީ' => Ok(Thaana::Eebeefili),
            'ު' => Ok(Thaana::Ubufili),
            'ޫ' => Ok(Thaana::Ooboofili),
            'ެ' => Ok(Thaana::Ebefili),
            'ޭ' => Ok(Thaana::Eybeyfili),
            'ޮ' => Ok(Thaana::Obofili),
            'ޯ' => Ok(Thaana::Oaboafili),
            'ް' => Ok(Thaana::Sukun),
            'ޱ' => Ok(Thaana::LetterNaa),
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
